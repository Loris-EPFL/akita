use super::kernels::{read_raw, source_partial, validate_provider};
use super::*;
use akita_error::AkitaError;
use akita_sumcheck::{EqFactoredSumcheckKernel, SumcheckKernel};
use jolt_field::{CanonicalBytes, CanonicalEncoding, Field};
use jolt_poly::{OmittedConstantPoly, UnivariatePoly};

fn quartic<F: Field>(child: usize, value: F) -> F {
    (4 * child..4 * child + 4).fold(F::one(), |p, t| {
        p * (value - F::from_u64((t * (t + 1)) as u64))
    })
}
/// Exact signed predicate; includes both -16 and +15, rejects -17 and +16.
pub fn signed_plane_residual<F: Field>(value: F) -> F {
    let u = value * (value + F::one());
    (0..4).fold(F::one(), |v, j| v * quartic(j, u))
}
/// Evaluate the powers-gamma batch of four exact quartic child polynomials.
pub fn quartic_batch<F: Field>(gamma: F, value: F) -> F {
    let mut power = F::one();
    let mut result = F::zero();
    for j in 0..4 {
        result += power * quartic(j, value);
        power *= gamma;
    }
    result
}
fn multiply_linear<F: Field>(coefficients: &mut [F; 5], degree: usize, a: F, b: F) {
    for d in (1..=degree + 1).rev() {
        coefficients[d] = coefficients[d] * a + coefficients[d - 1] * b;
    }
    coefficients[0] *= a;
}

/// Low-first Boolean equality factors, shared by every contraction in a round.
/// Each table covers at most eight bound coordinates, never original sources.
/// Multiplication only: zero/one challenges require neither inverses nor skips.
/// Scratch is at most 256 * ceil(bound.len()/8) field elements; construction
/// takes O(256 * bound.len()) work and each prefix uses ceil(bound.len()/8)
/// products. Tables preserve low-first indexing, including the short last chunk.
pub(super) struct BoundPrefixFactors<F> {
    bound_len: usize,
    prefix_count: usize,
    tables: Vec<F>,
}
impl<F: Field> BoundPrefixFactors<F> {
    const CHUNK_BITS: usize = 8;
    pub(super) fn new(bound: &[F], num_vars: usize) -> Result<Self, AkitaError> {
        if bound.len() > num_vars {
            return Err(invalid("range bound factor dimension"));
        }
        let prefix_count = cube_len(bound.len())?;
        let tail_bits = bound.len() % Self::CHUNK_BITS;
        let tail_cells = if tail_bits == 0 {
            0
        } else {
            cube_len(tail_bits)?
        };
        let cells = (bound.len() / Self::CHUNK_BITS)
            .checked_mul(256)
            .and_then(|n| n.checked_add(tail_cells))
            .ok_or_else(|| invalid("range bound factor size"))?;
        let mut tables = Vec::with_capacity(cells);
        for chunk in bound.chunks(Self::CHUNK_BITS) {
            // The final short chunk stores only its reachable Boolean indices.
            for prefix in 0..cube_len(chunk.len())? {
                tables.push(eq_index(chunk, prefix));
            }
        }
        Ok(Self {
            bound_len: bound.len(),
            prefix_count,
            tables,
        })
    }
    pub(super) fn weight(&self, mut prefix: usize) -> Result<F, AkitaError> {
        if prefix >= self.prefix_count {
            return Err(invalid("range bound factor index"));
        }
        let mut weight = F::one();
        for chunk in self.tables.chunks(256) {
            weight *= chunk
                .get(prefix & 255)
                .copied()
                .ok_or_else(|| invalid("range bound chunk index"))?;
            prefix >>= Self::CHUNK_BITS;
        }
        Ok(weight)
    }
    /// Factor ascending prefixes into at most eight low bits and a shared high
    /// weight. Consumers must visit every low cell, even when either factor is
    /// zero, and apply the high weight only after summing that block.
    pub(super) fn for_each_block(
        &self,
        mut consume: impl FnMut(usize, &[F], F) -> Result<(), AkitaError>,
    ) -> Result<(), AkitaError> {
        if self.bound_len == 0 {
            return consume(0, &[F::one()], F::one());
        }
        let low_len = cube_len(self.bound_len.min(Self::CHUNK_BITS))?;
        let (low, high) = self.tables.split_at(low_len);
        for start in (0..self.prefix_count).step_by(low_len) {
            let mut index = start >> Self::CHUNK_BITS;
            let mut weight = F::one();
            for chunk in high.chunks(256) {
                weight *= chunk
                    .get(index & 255)
                    .copied()
                    .ok_or_else(|| invalid("range bound chunk index"))?;
                index >>= Self::CHUNK_BITS;
            }
            consume(start, low, weight)?;
        }
        Ok(())
    }
}

/// Frozen RAM-only roster. `boundary_aliases` retains exact original boundary
/// ownership; indices are first-occurrence canonical, never value deduplicated.
#[derive(Clone, Debug)]
pub struct GlobalRamRange {
    sources: Vec<ApprovedSource>,
    ram_slots: Vec<usize>,
    boundary_aliases: Vec<usize>,
    num_vars: usize,
    cube_cells: usize,
}
/// Bounds for the streaming baseline, not cryptographic runtime admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeArithmeticBudget {
    /// Raw21 plus source-index coordinate count.
    pub num_vars: usize,
    /// Padded global Boolean cube size, including structural virtual cells.
    pub cube_cells: usize,
    /// Literal original physical signed-plane coefficient count.
    pub physical_cells: usize,
    /// Anchor, product, child-batch, leaf and link scalar request count.
    pub scalar_requests: usize,
    /// Compressed round coefficients, helpers and raw endpoint proof atoms.
    pub proof_atoms: usize,
    /// Uniform fixed-source statistical numerator 12n+3.
    pub normalized_degree_charge: usize,
    /// Conservative bound on original coefficient reads in all stages/endpoints.
    pub max_source_reads: u128,
    /// Conservative kernel field-operation bound, excluding backend/oracle work.
    pub conservative_field_operations: u128,
}
impl GlobalRamRange {
    /// Freeze all original RAM sources and checked canonical boundary aliases.
    pub fn new<F: Field + CanonicalEncoding>(
        ledger: &QueryLedger<F>,
        ram_slots: Vec<usize>,
        boundary_aliases: Vec<usize>,
        max_cube_cells: usize,
    ) -> Result<Self, AkitaError> {
        if ledger.is_sealed()
            || ram_slots.is_empty()
            || boundary_aliases.is_empty()
            || boundary_aliases.len() > ledger.limits.claim_horizon.saturating_add(1)
        {
            return Err(invalid(
                "range must freeze bounded nonempty roster before seal",
            ));
        }
        for (i, &slot) in ram_slots.iter().enumerate() {
            let source = ledger
                .sources()
                .get(slot)
                .ok_or_else(|| invalid("range source slot"))?;
            if source.view != SourceView::OriginalRamPlanesV1 || ram_slots[..i].contains(&slot) {
                return Err(invalid("range roster must contain each original RAM once"));
            }
        }
        let all_ram: Vec<usize> = ledger
            .sources()
            .iter()
            .enumerate()
            .filter_map(|(i, s)| (s.view == SourceView::OriginalRamPlanesV1).then_some(i))
            .collect();
        if all_ram != ram_slots {
            return Err(invalid("range missing or reordered original RAM source"));
        }
        let mut seen = vec![false; ram_slots.len()];
        let mut next = 0;
        for &alias in &boundary_aliases {
            let old = seen
                .get_mut(alias)
                .ok_or_else(|| invalid("boundary RAM alias"))?;
            if !*old {
                if alias != next {
                    return Err(invalid("RAM alias first occurrence order"));
                }
                *old = true;
                next += 1;
            }
        }
        if next != ram_slots.len() {
            return Err(invalid("unowned range RAM source"));
        }
        let buckets = ram_slots
            .len()
            .checked_next_power_of_two()
            .ok_or_else(|| invalid("RAM source bucket overflow"))?;
        let num_vars = 21usize
            .checked_add(buckets.trailing_zeros() as usize)
            .ok_or_else(|| invalid("range dimension overflow"))?;
        let cube_cells = cube_len(num_vars)?;
        if cube_cells > max_cube_cells {
            return Err(invalid("range trusted cube budget"));
        }
        Ok(Self {
            sources: ledger.sources().to_vec(),
            ram_slots,
            boundary_aliases,
            num_vars,
            cube_cells,
        })
    }
    /// Dimension of the global range cube, with raw axes first.
    pub fn num_vars(&self) -> usize {
        self.num_vars
    }
    /// Finite public-shape accounting, independent of observed source values.
    pub fn arithmetic_budget(&self) -> Result<RangeArithmeticBudget, AkitaError> {
        let n = self.num_vars;
        let scalar_requests = n
            .checked_mul(4)
            .and_then(|v| v.checked_add(1))
            .ok_or_else(|| invalid("range request count overflow"))?;
        let proof_atoms = n
            .checked_mul(11)
            .and_then(|v| v.checked_add(5))
            .and_then(|v| v.checked_add(self.ram_slots.len()))
            .ok_or_else(|| invalid("range proof count overflow"))?;
        let physical_cells = self
            .ram_slots
            .len()
            .checked_mul(128 * 26 * 32 * 16)
            .ok_or_else(|| invalid("range physical count overflow"))?;
        let visits = (3u128 * n as u128 + 4)
            .checked_mul(self.cube_cells as u128)
            .ok_or_else(|| invalid("range scan work overflow"))?;
        let field_work = visits
            .checked_mul(256u128 + 4 * n as u128)
            .ok_or_else(|| invalid("range arithmetic work overflow"))?;
        Ok(RangeArithmeticBudget {
            num_vars: n,
            cube_cells: self.cube_cells,
            physical_cells,
            scalar_requests,
            proof_atoms,
            normalized_degree_charge: self.degree_charge()?,
            max_source_reads: visits,
            conservative_field_operations: field_work,
        })
    }
    /// Original RAM component indices in frozen ledger order.
    pub fn ram_slots(&self) -> &[usize] {
        &self.ram_slots
    }
    /// Exact normalized fixed-source numerator 12n+3, not an admission claim.
    pub fn degree_charge(&self) -> Result<usize, AkitaError> {
        self.num_vars
            .checked_mul(12)
            .and_then(|n| n.checked_add(3))
            .ok_or_else(|| invalid("range degree ledger overflow"))
    }
    /// Canonical public range context, to bind before the first anchor draw.
    pub fn canonical_context(&self) -> Result<Vec<u8>, AkitaError> {
        let mut b = b"akita/global-original-ram-range/v1".to_vec();
        for value in [
            self.num_vars,
            self.cube_cells,
            self.ram_slots.len(),
            self.boundary_aliases.len(),
        ] {
            super::ledger::size(&mut b, value)?;
        }
        for &slot in &self.ram_slots {
            let source = &self.sources[slot];
            super::ledger::size(&mut b, slot)?;
            super::ledger::size(&mut b, source.canonical_identity.len())?;
            b.extend_from_slice(&source.canonical_identity);
        }
        for &alias in &self.boundary_aliases {
            super::ledger::size(&mut b, alias)?;
        }
        Ok(b)
    }
    fn cube_value<F: Field>(
        &self,
        provider: &dyn SourceProvider<F>,
        raw: usize,
    ) -> Result<F, AkitaError> {
        if raw >= self.cube_cells {
            return Err(invalid("global range cube index"));
        }
        match self.ram_slots.get(raw >> 21) {
            Some(&slot) => read_raw(&self.sources[slot], slot, provider, raw & ((1 << 21) - 1)),
            None => Ok(F::zero()),
        }
    }
    fn folded<F: Field>(
        &self,
        provider: &dyn SourceProvider<F>,
        factors: &BoundPrefixFactors<F>,
        suffix: usize,
        helper: Helper,
        alphabet: Option<&RangeAlphabet<F>>,
    ) -> Result<[F; 4], AkitaError> {
        if factors.bound_len > self.num_vars
            || suffix >= cube_len(self.num_vars - factors.bound_len)?
        {
            return Err(invalid("range folded geometry"));
        }
        let mut result = [F::zero(); 4];
        factors.for_each_block(|start, low_weights, high_weight| {
            let mut subtotal = [F::zero(); 4];
            for (offset, &low_weight) in low_weights.iter().enumerate() {
                let prefix = start + offset;
                let g = self.cube_value(provider, prefix | (suffix << factors.bound_len))?;
                // Helpers act on the original Boolean g, before any off-cube fold.
                match helper {
                    Helper::G => subtotal[0] += low_weight * g,
                    Helper::U => {
                        let u = g * (g + F::one());
                        subtotal[0] += low_weight * u;
                    }
                    Helper::Children => {
                        let u = g * (g + F::one());
                        // Defaults Q_1(0), Q_2(0), Q_3(0) are NONZERO.
                        let children = alphabet.map_or_else(
                            || std::array::from_fn(|j| quartic(j, u)),
                            |alphabet| alphabet.children(u),
                        );
                        for (value, child) in subtotal.iter_mut().zip(children) {
                            *value += low_weight * child;
                        }
                    }
                }
            }
            for (value, subtotal) in result.iter_mut().zip(subtotal) {
                *value += high_weight * subtotal;
            }
            Ok(())
        })?;
        Ok(result)
    }
    /// Contract both Boolean values on the current axis, sharing each prefix weight.
    fn folded_pair<F: Field>(
        &self,
        provider: &dyn SourceProvider<F>,
        factors: &BoundPrefixFactors<F>,
        suffix: usize,
        helper: Helper,
        alphabet: Option<&RangeAlphabet<F>>,
    ) -> Result<([F; 4], [F; 4]), AkitaError> {
        if factors.bound_len >= self.num_vars
            || suffix >= cube_len(self.num_vars - factors.bound_len - 1)?
        {
            return Err(invalid("range folded geometry"));
        }
        let mut zero = [F::zero(); 4];
        let mut one = [F::zero(); 4];
        let base = (suffix << 1) << factors.bound_len;
        let axis = 1 << factors.bound_len;
        factors.for_each_block(|start, low_weights, high_weight| {
            let mut zero_subtotal = [F::zero(); 4];
            let mut one_subtotal = [F::zero(); 4];
            for (offset, &low_weight) in low_weights.iter().enumerate() {
                let prefix = start + offset;
                // Keep source reads ascending and interleaved across branches.
                let g_zero = self.cube_value(provider, prefix | base)?;
                let g_one = self.cube_value(provider, prefix | base | axis)?;
                // Helpers act on each original Boolean g, before weighted contraction.
                match helper {
                    Helper::G => {
                        zero_subtotal[0] += low_weight * g_zero;
                        one_subtotal[0] += low_weight * g_one;
                    }
                    Helper::U => {
                        let u_zero = g_zero * (g_zero + F::one());
                        let u_one = g_one * (g_one + F::one());
                        zero_subtotal[0] += low_weight * u_zero;
                        one_subtotal[0] += low_weight * u_one;
                    }
                    Helper::Children => {
                        let u_zero = g_zero * (g_zero + F::one());
                        let u_one = g_one * (g_one + F::one());
                        // Virtual/padded cells retain NONZERO Q_1(0), Q_2(0), Q_3(0).
                        let children_zero = alphabet.map_or_else(
                            || std::array::from_fn(|j| quartic(j, u_zero)),
                            |alphabet| alphabet.children(u_zero),
                        );
                        let children_one = alphabet.map_or_else(
                            || std::array::from_fn(|j| quartic(j, u_one)),
                            |alphabet| alphabet.children(u_one),
                        );
                        for j in 0..4 {
                            zero_subtotal[j] += low_weight * children_zero[j];
                            one_subtotal[j] += low_weight * children_one[j];
                        }
                    }
                }
            }
            for j in 0..4 {
                zero[j] += high_weight * zero_subtotal[j];
                one[j] += high_weight * one_subtotal[j];
            }
            Ok(())
        })?;
        Ok((zero, one))
    }
    /// Construct the normalized degree-four product kernel with initial claim zero.
    pub fn product_kernel<'a, F: Field>(
        &'a self,
        provider: &'a dyn SourceProvider<F>,
        anchor: Vec<F>,
    ) -> Result<RangeEqKernel<'a, F>, AkitaError> {
        RangeEqKernel::new(self, provider, anchor, F::zero(), RangeMode::Product)
    }
    /// Verifier check on all four child claims BEFORE gamma is requested.
    pub fn verify_product_endpoint<F: Field>(
        &self,
        point: Vec<F>,
        terminal: F,
        children: [F; 4],
    ) -> Result<VerifiedRangeChildren<F>, AkitaError> {
        if point.len() != self.num_vars
            || children.iter().copied().fold(F::one(), |p, c| p * c) != terminal
        {
            return Err(AkitaError::InvalidProof);
        }
        Ok(VerifiedRangeChildren { point, children })
    }
    /// Construct the powers-batched normalized quartic kernel after child binding.
    pub fn leaf_kernel<'a, F: Field>(
        &'a self,
        provider: &'a dyn SourceProvider<F>,
        children: &VerifiedRangeChildren<F>,
        gamma: F,
    ) -> Result<RangeEqKernel<'a, F>, AkitaError> {
        if children.point.len() != self.num_vars {
            return Err(invalid("range child point dimension"));
        }
        RangeEqKernel::new(
            self,
            provider,
            children.point.clone(),
            children.batched_claim(gamma),
            RangeMode::Leaf { gamma },
        )
    }
    /// Own t_U through the quartic check, then feed it to the ON-CUBE link.
    pub fn verify_leaf_endpoint<F: Field>(
        &self,
        point: Vec<F>,
        gamma: F,
        terminal: F,
        t_u: F,
    ) -> Result<VerifiedRangeLeaf<F>, AkitaError> {
        if point.len() != self.num_vars || quartic_batch(gamma, t_u) != terminal {
            return Err(AkitaError::InvalidProof);
        }
        Ok(VerifiedRangeLeaf { point, value: t_u })
    }
    /// Construct the standard cubic on-cube realization after binding t_U.
    pub fn link_kernel<'a, F: Field>(
        &'a self,
        provider: &'a dyn SourceProvider<F>,
        leaf: &VerifiedRangeLeaf<F>,
    ) -> Result<RangeLinkKernel<'a, F>, AkitaError> {
        validate_provider(&self.sources, provider)?;
        if leaf.point.len() != self.num_vars {
            return Err(invalid("range link anchor dimension"));
        }
        Ok(RangeLinkKernel {
            range: self,
            provider,
            anchor: leaf.point.clone(),
            claim: leaf.value,
            bound: Vec::with_capacity(self.num_vars),
            pending: false,
        })
    }
    /// Stream every original raw endpoint, including zero-selector sources.
    pub fn source_endpoints<F: Field>(
        &self,
        provider: &dyn SourceProvider<F>,
        point: &[F],
    ) -> Result<Vec<SourceEndpoint<F>>, AkitaError> {
        validate_provider(&self.sources, provider)?;
        if point.len() != self.num_vars {
            return Err(invalid("range endpoint dimension"));
        }
        self.ram_slots
            .iter()
            .map(|&slot| {
                let source = &self.sources[slot];
                Ok(SourceEndpoint {
                    source: slot,
                    local_ownership: source.local_ownership,
                    component: source.component,
                    value: source_partial(source, slot, provider, &point[..21], 0)?,
                })
            })
            .collect()
    }
    /// Independently reconstruct G(z) from EVERY p_C in exact frozen order.
    pub fn verify_link_endpoint<F: Field>(
        &self,
        leaf: &VerifiedRangeLeaf<F>,
        point: &[F],
        terminal: F,
        endpoints: &[SourceEndpoint<F>],
    ) -> Result<(), AkitaError> {
        if leaf.point.len() != self.num_vars
            || point.len() != self.num_vars
            || endpoints.len() != self.ram_slots.len()
        {
            return Err(invalid("range link endpoint size"));
        }
        let mut g = F::zero();
        for (index, (&slot, endpoint)) in self.ram_slots.iter().zip(endpoints).enumerate() {
            let source = &self.sources[slot];
            if endpoint.source != slot
                || endpoint.local_ownership != source.local_ownership
                || endpoint.component != source.component
            {
                return Err(invalid("range endpoint order or foreign source"));
            }
            g += eq_index(&point[21..], index) * endpoint.value;
        }
        if terminal != equality(&leaf.point, point)? * g * (g + F::one()) {
            return Err(AkitaError::InvalidProof);
        }
        Ok(())
    }
    /// Caller binds the complete replayed range transcript before this operation.
    /// Even zero source-index weights append independent raw scalar equations.
    pub fn append_endpoints<F: Field + CanonicalEncoding + CanonicalBytes>(
        &self,
        ledger: &mut QueryLedger<F>,
        leaf: &VerifiedRangeLeaf<F>,
        point: &[F],
        terminal: F,
        endpoints: &[SourceEndpoint<F>],
    ) -> Result<(), AkitaError> {
        self.verify_link_endpoint(leaf, point, terminal, endpoints)?;
        if ledger.is_sealed()
            || ledger.range_endpoints_appended
            || ledger.sources() != self.sources.as_slice()
            || ledger
                .claims
                .len()
                .checked_add(endpoints.len())
                .is_none_or(|n| n > ledger.limits.claim_horizon)
            || ledger.limits.max_terms_per_claim == 0
        {
            return Err(invalid("range append ownership or horizon"));
        }
        let start = ledger.claims.len();
        for (i, endpoint) in endpoints.iter().enumerate() {
            ledger.append(ScalarClaim {
                exponent: start + i,
                value: endpoint.value,
                offset: F::zero(),
                terms: vec![QueryTerm {
                    source: endpoint.source,
                    coefficient: F::one(),
                    recipe: QueryRecipe::Eq {
                        point: point[..21].to_vec(),
                    },
                }],
            })?;
        }
        ledger.range_endpoints_appended = true;
        Ok(())
    }
}
#[derive(Clone, Copy)]
enum Helper {
    G,
    U,
    Children,
}
#[derive(Clone, Copy)]
enum RangeMode<F> {
    Product,
    Leaf { gamma: F },
}
/// Constructible only by the verifier's product endpoint check.
pub struct VerifiedRangeChildren<F> {
    point: Vec<F>,
    children: [F; 4],
}
impl<F: Field> VerifiedRangeChildren<F> {
    /// Product terminal point that owns the four children.
    pub fn point(&self) -> &[F] {
        &self.point
    }
    /// All four child values in base32 balanced-tree order.
    pub fn children(&self) -> &[F; 4] {
        &self.children
    }
    /// Exact powers batch, without dropping zero gamma coefficients.
    pub fn batched_claim(&self, gamma: F) -> F {
        self.children
            .iter()
            .rev()
            .fold(F::zero(), |v, &c| v * gamma + c)
    }
}
/// Constructible only by the verifier's quartic endpoint check.
pub struct VerifiedRangeLeaf<F> {
    point: Vec<F>,
    value: F,
}
impl<F: Field> VerifiedRangeLeaf<F> {
    /// Quartic terminal point that owns the virtual U value.
    pub fn point(&self) -> &[F] {
        &self.point
    }
    /// Virtual U value to realize with the standard cubic linkage.
    pub fn value(&self) -> F {
        self.value
    }
}

/// Fallible normalized equality-factored product or quartic range kernel.
pub struct RangeEqKernel<'a, F: Field> {
    range: &'a GlobalRamRange,
    provider: &'a dyn SourceProvider<F>,
    anchor: Vec<F>,
    claim: F,
    mode: RangeMode<F>,
    bound: Vec<F>,
    pending: bool,
    alphabet: RangeAlphabet<F>,
}
/// Sixteen U=t(t+1) images cover the 32 legal signed digits. This is an
/// arithmetic cache only; arbitrary field values take the exact fallback.
struct RangeAlphabet<F> {
    images: [F; 16],
    children: [[F; 4]; 16],
}
impl<F: Field> RangeAlphabet<F> {
    fn new() -> Self {
        let images = std::array::from_fn(|t| F::from_u64((t * (t + 1)) as u64));
        let children = std::array::from_fn(|t| std::array::from_fn(|j| quartic(j, images[t])));
        Self { images, children }
    }
    fn children(&self, u: F) -> [F; 4] {
        self.images.iter().position(|&v| v == u).map_or_else(
            || std::array::from_fn(|j| quartic(j, u)),
            |i| self.children[i],
        )
    }
}
impl<'a, F: Field> RangeEqKernel<'a, F> {
    fn new(
        range: &'a GlobalRamRange,
        provider: &'a dyn SourceProvider<F>,
        anchor: Vec<F>,
        claim: F,
        mode: RangeMode<F>,
    ) -> Result<Self, AkitaError> {
        validate_provider(&range.sources, provider)?;
        if anchor.len() != range.num_vars {
            return Err(invalid("range anchor dimension"));
        }
        Ok(Self {
            range,
            provider,
            anchor,
            claim,
            mode,
            bound: Vec::with_capacity(range.num_vars),
            pending: false,
            alphabet: RangeAlphabet::new(),
        })
    }
    /// Bound challenges in global raw-then-source low-first order.
    pub fn point(&self) -> &[F] {
        &self.bound
    }
    /// Stream true on-cube helper MLE endpoints after all rounds.
    pub fn helper_endpoint(&self) -> Result<Vec<F>, AkitaError> {
        if self.pending || self.bound.len() != self.range.num_vars {
            return Err(invalid("range helper endpoint before finish"));
        }
        let helper = match self.mode {
            RangeMode::Product => Helper::Children,
            RangeMode::Leaf { .. } => Helper::U,
        };
        let factors = BoundPrefixFactors::new(&self.bound, self.range.num_vars)?;
        let values = self
            .range
            .folded(self.provider, &factors, 0, helper, Some(&self.alphabet))?;
        Ok(match self.mode {
            RangeMode::Product => values.to_vec(),
            RangeMode::Leaf { .. } => vec![values[0]],
        })
    }
}
impl<F: Field> EqFactoredSumcheckKernel<F> for RangeEqKernel<'_, F> {
    fn num_rounds(&self) -> usize {
        self.range.num_vars
    }
    fn degree_bound(&self) -> usize {
        4
    }
    fn input_claim(&self) -> F {
        self.claim
    }
    fn current_tau(&self) -> F {
        self.anchor
            .get(self.bound.len())
            .copied()
            .unwrap_or_else(F::zero)
    }
    fn round_polynomial(
        &mut self,
        round: usize,
        claim: F,
    ) -> Result<OmittedConstantPoly<F>, AkitaError> {
        if self.pending || round != self.bound.len() || round >= self.range.num_vars {
            return Err(invalid("range normalized round progression"));
        }
        let factors = BoundPrefixFactors::new(&self.bound, self.range.num_vars)?;
        let suffix_factors =
            BoundPrefixFactors::new(&self.anchor[round + 1..], self.range.num_vars)?;
        let mut coefficients = [F::zero(); 5];
        let helper = match self.mode {
            RangeMode::Product => Helper::Children,
            RangeMode::Leaf { .. } => Helper::U,
        };
        for suffix in 0..cube_len(self.range.num_vars - round - 1)? {
            let weight = suffix_factors.weight(suffix)?;
            let (a, b) = self.range.folded_pair(
                self.provider,
                &factors,
                suffix,
                helper,
                Some(&self.alphabet),
            )?;
            let mut polynomial = [F::zero(); 5];
            match self.mode {
                RangeMode::Product => {
                    polynomial[0] = F::one();
                    for j in 0..4 {
                        multiply_linear(&mut polynomial, j, a[j], b[j] - a[j]);
                    }
                }
                RangeMode::Leaf { gamma } => {
                    let mut power = F::one();
                    for j in 0..4 {
                        let mut child = [F::zero(); 5];
                        child[0] = F::one();
                        for (degree, t) in (4 * j..4 * j + 4).enumerate() {
                            multiply_linear(
                                &mut child,
                                degree,
                                a[0] - F::from_u64((t * (t + 1)) as u64),
                                b[0] - a[0],
                            );
                        }
                        for d in 0..5 {
                            polynomial[d] += power * child[d];
                        }
                        power *= gamma;
                    }
                }
            }
            for d in 0..5 {
                coefficients[d] += weight * polynomial[d];
            }
        }
        let tau = self.anchor[round];
        if coefficients[0]
            + tau
                * coefficients[1..]
                    .iter()
                    .copied()
                    .fold(F::zero(), |s, c| s + c)
            != claim
        {
            return Err(AkitaError::InvalidProof);
        }
        self.pending = true;
        Ok(OmittedConstantPoly::new(coefficients[1..].to_vec()))
    }
    fn bind_challenge(&mut self, round: usize, challenge: F) -> Result<(), AkitaError> {
        if !self.pending || round != self.bound.len() {
            return Err(invalid("normalized range challenge progression"));
        }
        self.bound.push(challenge);
        self.pending = false;
        Ok(())
    }
    fn finish(&mut self) -> Result<(), AkitaError> {
        if self.pending || self.bound.len() != self.range.num_vars {
            return Err(invalid("unfinished normalized range sumcheck"));
        }
        Ok(())
    }
}

/// Degree-three realization of U as the MLE of ON-CUBE G(G+1).
pub struct RangeLinkKernel<'a, F: Field> {
    range: &'a GlobalRamRange,
    provider: &'a dyn SourceProvider<F>,
    anchor: Vec<F>,
    claim: F,
    bound: Vec<F>,
    pending: bool,
}
impl<F: Field> RangeLinkKernel<'_, F> {
    /// Bound challenges in global raw-then-source low-first order.
    pub fn point(&self) -> &[F] {
        &self.bound
    }
}
impl<F: Field> SumcheckKernel<F> for RangeLinkKernel<'_, F> {
    fn num_rounds(&self) -> usize {
        self.range.num_vars
    }
    fn degree_bound(&self) -> usize {
        3
    }
    fn input_claim(&self) -> F {
        self.claim
    }
    fn round_polynomial(
        &mut self,
        round: usize,
        claim: F,
    ) -> Result<UnivariatePoly<F>, AkitaError> {
        if self.pending || round != self.bound.len() || round >= self.range.num_vars {
            return Err(invalid("range cubic round progression"));
        }
        let factors = BoundPrefixFactors::new(&self.bound, self.range.num_vars)?;
        let suffix_factors =
            BoundPrefixFactors::new(&self.anchor[round + 1..], self.range.num_vars)?;
        let mut coefficients = [F::zero(); 5];
        let prefix_weight = equality(&self.anchor[..round], &self.bound)?;
        for suffix in 0..cube_len(self.range.num_vars - round - 1)? {
            let (a, b) =
                self.range
                    .folded_pair(self.provider, &factors, suffix, Helper::G, None)?;
            let (a, b) = (a[0], b[0]);
            let weight = prefix_weight * suffix_factors.weight(suffix)?;
            let mut polynomial = [F::zero(); 5];
            polynomial[0] = F::one();
            multiply_linear(&mut polynomial, 0, a, b - a);
            multiply_linear(&mut polynomial, 1, a + F::one(), b - a);
            multiply_linear(
                &mut polynomial,
                2,
                F::one() - self.anchor[round],
                self.anchor[round] + self.anchor[round] - F::one(),
            );
            for d in 0..4 {
                coefficients[d] += weight * polynomial[d];
            }
        }
        if coefficients[0]
            + coefficients[..4]
                .iter()
                .copied()
                .fold(F::zero(), |s, c| s + c)
            != claim
        {
            return Err(AkitaError::InvalidProof);
        }
        self.pending = true;
        Ok(UnivariatePoly::new(coefficients[..4].to_vec()))
    }
    fn bind_challenge(&mut self, round: usize, challenge: F) -> Result<(), AkitaError> {
        if !self.pending || round != self.bound.len() {
            return Err(invalid("range cubic challenge progression"));
        }
        self.bound.push(challenge);
        self.pending = false;
        Ok(())
    }
    fn finish(&mut self) -> Result<(), AkitaError> {
        if self.pending || self.bound.len() != self.range.num_vars {
            return Err(invalid("unfinished range cubic sumcheck"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod arithmetic_tests {
    use super::*;
    use akita_types::{full_ring::FullRingCoefficientSource, proof::FullRingSourceId};
    use jolt_field::{One, Ring, Zero};
    type F = akita_config::proof_optimized::fp128::Field;
    struct Material(Vec<F>);
    impl FullRingCoefficientSource<F> for Material {
        fn len(&self) -> usize {
            self.0.len()
        }
        fn coefficient(&self, i: usize) -> Result<F, AkitaError> {
            self.0
                .get(i)
                .copied()
                .ok_or_else(|| invalid("test material index"))
        }
    }
    impl SourceProvider<F> for Material {
        fn source_count(&self) -> usize {
            1
        }
        fn source_id(&self, i: usize) -> Result<(SourceOwnership, usize), AkitaError> {
            if i != 0 {
                return Err(invalid("test source slot"));
            }
            Ok((
                SourceOwnership::NativeInstalled {
                    source_id: FullRingSourceId(1),
                    commitment_id: 1,
                    backend_id: 1,
                    setup_digest: [1; 32],
                },
                0,
            ))
        }
        fn source(&self, i: usize) -> Result<&dyn FullRingCoefficientSource<F>, AkitaError> {
            if i != 0 {
                return Err(invalid("test source slot"));
            }
            Ok(self)
        }
    }
    fn reference_geometry() -> GlobalRamRange {
        // Arithmetic-only reduced cube, inaccessible to production admission.
        GlobalRamRange {
            sources: vec![ApprovedSource {
                local_ownership: Some(SourceOwnership::NativeInstalled {
                    source_id: FullRingSourceId(1),
                    commitment_id: 1,
                    backend_id: 1,
                    setup_digest: [1; 32],
                }),
                component: 0,
                canonical_identity: b"arithmetic reference".to_vec(),
                view: SourceView::Decoded { num_vars: 2 },
            }],
            ram_slots: vec![0],
            boundary_aliases: vec![0],
            num_vars: 2,
            cube_cells: 4,
        }
    }
    fn drive_normalized(kernel: &mut RangeEqKernel<'_, F>, challenges: &[F]) -> F {
        let mut claim = kernel.input_claim();
        for (round, &challenge) in challenges.iter().enumerate() {
            let tau = kernel.current_tau();
            let polynomial = kernel.round_polynomial(round, claim).unwrap();
            claim = akita_sumcheck::advance_eq_factored_claim(claim, tau, &polynomial, challenge);
            kernel.bind_challenge(round, challenge).unwrap();
        }
        kernel.finish().unwrap();
        claim
    }
    #[test]
    fn normalized_product_quartic_and_cubic_use_oncube_realization() {
        let range = reference_geometry();
        let material = Material(vec![-F::from_u64(16), F::from_u64(15), F::zero(), F::one()]);
        let anchor = vec![F::from_u64(3), F::from_u64(7)];
        let product_point = [F::from_u64(5), F::from_u64(11)];
        let mut product = range.product_kernel(&material, anchor).unwrap();
        let product_terminal = drive_normalized(&mut product, &product_point);
        let children: [F; 4] = product.helper_endpoint().unwrap().try_into().unwrap();
        let checked = range
            .verify_product_endpoint(product_point.to_vec(), product_terminal, children)
            .unwrap();
        let gamma = F::zero(); // zero batching weights are legal
        let leaf_point = [F::from_u64(13), F::from_u64(17)];
        let mut leaf = range.leaf_kernel(&material, &checked, gamma).unwrap();
        let leaf_terminal = drive_normalized(&mut leaf, &leaf_point);
        let t_u = leaf.helper_endpoint().unwrap()[0];
        let checked_leaf = range
            .verify_leaf_endpoint(leaf_point.to_vec(), gamma, leaf_terminal, t_u)
            .unwrap();
        let leaf_factors = BoundPrefixFactors::new(&leaf_point, range.num_vars()).unwrap();
        let g_u = range
            .folded(&material, &leaf_factors, 0, Helper::G, None)
            .unwrap()[0];
        assert_ne!(t_u, g_u * (g_u + F::one()));
        let mut link = range.link_kernel(&material, &checked_leaf).unwrap();
        let link_point = [F::from_u64(19), F::from_u64(23)];
        let mut running = link.input_claim();
        for (round, &challenge) in link_point.iter().enumerate() {
            let polynomial = link.round_polynomial(round, running).unwrap();
            assert_eq!(
                polynomial.evaluate(F::zero()) + polynomial.evaluate(F::one()),
                running
            );
            running = polynomial.evaluate(challenge);
            link.bind_challenge(round, challenge).unwrap();
        }
        link.finish().unwrap();
        let link_factors = BoundPrefixFactors::new(&link_point, range.num_vars()).unwrap();
        let g_z = range
            .folded(&material, &link_factors, 0, Helper::G, None)
            .unwrap()[0];
        assert_eq!(
            running,
            equality(&leaf_point, &link_point).unwrap() * g_z * (g_z + F::one())
        );
        assert!(range
            .verify_leaf_endpoint(leaf_point.to_vec(), gamma, leaf_terminal + F::one(), t_u)
            .is_err());
    }
    #[test]
    fn invalid_original_digit_cannot_produce_zero_anchor_round() {
        let range = reference_geometry();
        for digit in [-F::from_u64(17), F::from_u64(16)] {
            let material = Material(vec![digit, F::zero(), F::zero(), F::zero()]);
            let mut product = range.product_kernel(&material, vec![F::zero(); 2]).unwrap();
            assert!(product.round_polynomial(0, F::zero()).is_err());
        }
    }
    #[test]
    fn suffix_factor_rounds_match_direct_boolean_reference() {
        let mut range = reference_geometry();
        range.num_vars = 10;
        range.cube_cells = 1 << 10;
        range.sources[0].view = SourceView::Decoded { num_vars: 10 };
        let material = Material(
            (0..range.cube_cells)
                .map(|i| F::from_u64(((i ^ (i >> 4) ^ (i >> 8)) % 16) as u64))
                .collect(),
        );
        for kind in 0..3 {
            let anchor: Vec<F> = (0..range.num_vars())
                .map(|i| match kind {
                    0 => F::zero(),
                    1 => F::one(),
                    _ => F::from_u64((i % 5) as u64),
                })
                .collect();
            for mode in [
                RangeMode::Product,
                RangeMode::Leaf { gamma: F::zero() },
                RangeMode::Leaf {
                    gamma: F::from_u64(5),
                },
            ] {
                let reference = |t: F| {
                    (0..range.cube_cells / 2).fold(F::zero(), |sum, suffix| {
                        let g_zero = material.0[suffix << 1];
                        let g_one = material.0[(suffix << 1) | 1];
                        let u_zero = g_zero * (g_zero + F::one());
                        let u_one = g_one * (g_one + F::one());
                        let value = match mode {
                            RangeMode::Product => (0..4).fold(F::one(), |p, j| {
                                p * ((F::one() - t) * quartic(j, u_zero) + t * quartic(j, u_one))
                            }),
                            RangeMode::Leaf { gamma } => {
                                quartic_batch(gamma, (F::one() - t) * u_zero + t * u_one)
                            }
                        };
                        sum + eq_index(&anchor[1..], suffix) * value
                    })
                };
                let claim =
                    (F::one() - anchor[0]) * reference(F::zero()) + anchor[0] * reference(F::one());
                let mut kernel =
                    RangeEqKernel::new(&range, &material, anchor.clone(), claim, mode).unwrap();
                let polynomial = kernel.round_polynomial(0, claim).unwrap();
                for t in [F::zero(), F::one(), F::from_u64(13)] {
                    assert_eq!(
                        akita_sumcheck::advance_eq_factored_claim(claim, anchor[0], &polynomial, t),
                        reference(t)
                    );
                }
            }
            let reference = |t: F| {
                let value = (0..range.cube_cells / 2).fold(F::zero(), |sum, suffix| {
                    let g = (F::one() - t) * material.0[suffix << 1]
                        + t * material.0[(suffix << 1) | 1];
                    sum + eq_index(&anchor[1..], suffix) * g * (g + F::one())
                });
                ((F::one() - anchor[0]) * (F::one() - t) + anchor[0] * t) * value
            };
            let claim = reference(F::zero()) + reference(F::one());
            let leaf = VerifiedRangeLeaf {
                point: anchor.clone(),
                value: claim,
            };
            let mut kernel = range.link_kernel(&material, &leaf).unwrap();
            let polynomial = kernel.round_polynomial(0, claim).unwrap();
            for t in [F::zero(), F::one(), F::from_u64(13)] {
                assert_eq!(polynomial.evaluate(t), reference(t));
            }
        }
    }
    #[test]
    fn paired_folds_keep_provider_errors_at_zero_prefix_and_suffix_weights() {
        struct RejectCoefficient {
            material: Material,
            rejected: usize,
        }
        impl FullRingCoefficientSource<F> for RejectCoefficient {
            fn len(&self) -> usize {
                self.material.len()
            }
            fn coefficient(&self, i: usize) -> Result<F, AkitaError> {
                if i == self.rejected {
                    return Err(invalid("rejected test coefficient"));
                }
                self.material.coefficient(i)
            }
        }
        impl SourceProvider<F> for RejectCoefficient {
            fn source_count(&self) -> usize {
                self.material.source_count()
            }
            fn source_id(&self, i: usize) -> Result<(SourceOwnership, usize), AkitaError> {
                self.material.source_id(i)
            }
            fn source(&self, i: usize) -> Result<&dyn FullRingCoefficientSource<F>, AkitaError> {
                if i != 0 {
                    return Err(invalid("test source slot"));
                }
                Ok(self)
            }
        }
        let range = reference_geometry();
        let factors = BoundPrefixFactors::new(&[F::zero()], range.num_vars()).unwrap();
        let expected = invalid("rejected test coefficient");
        for rejected in 0..4 {
            let provider = RejectCoefficient {
                material: Material(vec![F::zero(); 4]),
                rejected,
            };
            for helper in [Helper::G, Helper::U, Helper::Children] {
                assert_eq!(
                    range.folded_pair(&provider, &factors, 0, helper, None),
                    Err(expected.clone())
                );
            }
            let mut product = range.product_kernel(&provider, vec![F::zero(); 2]).unwrap();
            assert_eq!(
                product.round_polynomial(0, F::zero()).unwrap_err(),
                expected
            );
            let leaf = range
                .verify_leaf_endpoint(vec![F::zero(); 2], F::zero(), F::zero(), F::zero())
                .unwrap();
            let mut quartic = RangeEqKernel::new(
                &range,
                &provider,
                vec![F::zero(); 2],
                F::zero(),
                RangeMode::Leaf { gamma: F::zero() },
            )
            .unwrap();
            assert_eq!(
                quartic.round_polynomial(0, F::zero()).unwrap_err(),
                expected
            );
            let mut link = range.link_kernel(&provider, &leaf).unwrap();
            assert_eq!(link.round_polynomial(0, F::zero()).unwrap_err(), expected);
        }
        let short = Material(vec![F::zero(); 3]);
        assert_eq!(
            range.folded_pair(&short, &factors, 0, Helper::G, None),
            Err(invalid("stale source identity or physical accessor bounds"))
        );

        // Both branches must read across the eight-bit block boundary even
        // when the low weight, the shared high weight, or both are zero.
        let mut range = reference_geometry();
        range.num_vars = 10;
        range.cube_cells = 1 << 10;
        range.sources[0].view = SourceView::Decoded { num_vars: 10 };
        let alphabet = RangeAlphabet::new();
        for low in [F::zero(), F::from_u64(3)] {
            for high in [F::zero(), F::one()] {
                let mut bound = [low; 9];
                bound[8] = high;
                let factors = BoundPrefixFactors::new(&bound, range.num_vars()).unwrap();
                for rejected in [0, 255, 256, 511, 512, 767, 768, 1023] {
                    let provider = RejectCoefficient {
                        material: Material(vec![F::zero(); range.cube_cells]),
                        rejected,
                    };
                    for helper in [Helper::G, Helper::U, Helper::Children] {
                        for cache in [None, Some(&alphabet)] {
                            assert_eq!(
                                range.folded_pair(&provider, &factors, 0, helper, cache),
                                Err(expected.clone())
                            );
                            assert_eq!(
                                range.folded(&provider, &factors, rejected >> 9, helper, cache),
                                Err(expected.clone())
                            );
                        }
                    }
                }
                let short = Material(vec![F::zero(); range.cube_cells - 1]);
                for helper in [Helper::G, Helper::U, Helper::Children] {
                    assert_eq!(
                        range.folded_pair(&short, &factors, 0, helper, Some(&alphabet)),
                        Err(invalid("stale source identity or physical accessor bounds"))
                    );
                    assert_eq!(
                        range.folded(&short, &factors, 0, helper, Some(&alphabet)),
                        Err(invalid("stale source identity or physical accessor bounds"))
                    );
                }
            }
        }
    }
    #[test]
    fn bound_prefix_chunks_match_scalar_equality_at_bit_boundaries() {
        for n in [0usize, 1, 7, 8, 9, 15, 16, 17, 21, 23] {
            for boolean in [false, true] {
                let bound: Vec<F> = (0..n)
                    .map(|i| {
                        if boolean {
                            F::from_u64((i & 1) as u64)
                        } else {
                            F::from_u64((i + 3) as u64)
                        }
                    })
                    .collect();
                let factors = BoundPrefixFactors::new(&bound, n).unwrap();
                let tail_cells = if n % 8 == 0 { 0 } else { 1usize << (n % 8) };
                assert_eq!(factors.tables.len(), 256 * (n / 8) + tail_cells);
                assert_eq!(factors.prefix_count, cube_len(n).unwrap());
                let count = factors.prefix_count;
                let indices = (0..count.min(512))
                    .chain((0..count.min(512)).map(|i| count - i - 1))
                    .chain((0..n).flat_map(|i| [1usize << i, (count - 1) ^ (1usize << i)]));
                for prefix in indices {
                    assert_eq!(factors.weight(prefix).unwrap(), eq_index(&bound, prefix));
                }
                assert!(factors.weight(count).is_err());
            }
        }
        assert!(BoundPrefixFactors::new(&[F::zero(); 9], 8).is_err());
        assert!(BoundPrefixFactors::new(
            &vec![F::zero(); usize::BITS as usize],
            usize::BITS as usize
        )
        .is_err());
    }
    #[test]
    fn chunked_contractions_match_boolean_helper_reference() {
        let mut range = reference_geometry();
        range.num_vars = 10;
        range.cube_cells = 1 << 10;
        range.sources[0].view = SourceView::Decoded { num_vars: 10 };
        let digits = [-17i64, -16, -1, 0, 1, 15, 16];
        let material = Material(
            (0..range.cube_cells)
                .map(|i| {
                    let digit = digits[i % digits.len()];
                    if digit < 0 {
                        -F::from_u64((-digit) as u64)
                    } else {
                        F::from_u64(digit as u64)
                    }
                })
                .collect(),
        );
        let alphabet = RangeAlphabet::new();
        for n in [0usize, 1, 7, 8, 9, 10] {
            for kind in 0..5 {
                let bound: Vec<F> = (0..n)
                    .map(|i| match kind {
                        0 => F::zero(),
                        1 => F::one(),
                        3 if i == 8 => F::zero(),
                        4 if i == 8 => F::one(),
                        _ => F::from_u64((i + 3) as u64),
                    })
                    .collect();
                let factors = BoundPrefixFactors::new(&bound, range.num_vars()).unwrap();
                for suffix in 0..cube_len(range.num_vars() - n).unwrap() {
                    for helper in [Helper::G, Helper::U, Helper::Children] {
                        let mut expected = [F::zero(); 4];
                        for prefix in 0..cube_len(n).unwrap() {
                            let g = material.0[prefix | (suffix << n)];
                            let weight = eq_index(&bound, prefix);
                            match helper {
                                Helper::G => expected[0] += weight * g,
                                Helper::U => expected[0] += weight * g * (g + F::one()),
                                Helper::Children => {
                                    let u = g * (g + F::one());
                                    for (j, value) in expected.iter_mut().enumerate() {
                                        *value += weight * quartic(j, u);
                                    }
                                }
                            }
                        }
                        for cache in [None, Some(&alphabet)] {
                            assert_eq!(
                                range
                                    .folded(&material, &factors, suffix, helper, cache)
                                    .unwrap(),
                                expected
                            );
                            if n < range.num_vars() {
                                let (zero, one) = range
                                    .folded_pair(&material, &factors, suffix >> 1, helper, cache)
                                    .unwrap();
                                assert_eq!(if suffix & 1 == 0 { zero } else { one }, expected);
                            }
                        }
                    }
                }
                assert!(range
                    .folded(
                        &material,
                        &factors,
                        cube_len(range.num_vars() - n).unwrap(),
                        Helper::G,
                        None
                    )
                    .is_err());
                let invalid_suffix = if n < range.num_vars() {
                    cube_len(range.num_vars() - n - 1).unwrap()
                } else {
                    0
                };
                assert!(range
                    .folded_pair(&material, &factors, invalid_suffix, Helper::G, None)
                    .is_err());
            }
        }
    }
    #[test]
    fn virtual_planes_and_padded_source_buckets_keep_nonzero_children() {
        let mut range = reference_geometry();
        range.num_vars = 23;
        range.cube_cells = 1 << 23;
        range.sources[0].view = SourceView::OriginalRamPlanesV1;
        // No physical reads are permitted in these wholly virtual contractions.
        let material = Material(Vec::new());
        let alphabet = RangeAlphabet::new();
        let children = std::array::from_fn(|j| quartic(j, F::zero()));
        assert_eq!(children[0], F::zero());
        assert!(children[1..].iter().all(|&child| child != F::zero()));
        let check = |bound: &[F], suffix| {
            let factors = BoundPrefixFactors::new(bound, range.num_vars()).unwrap();
            for helper in [Helper::G, Helper::U] {
                assert_eq!(
                    range
                        .folded(&material, &factors, suffix, helper, None)
                        .unwrap(),
                    [F::zero(); 4]
                );
                assert_eq!(
                    range
                        .folded_pair(&material, &factors, suffix >> 1, helper, None)
                        .unwrap(),
                    ([F::zero(); 4], [F::zero(); 4])
                );
            }
            assert_eq!(
                range
                    .folded(
                        &material,
                        &factors,
                        suffix,
                        Helper::Children,
                        Some(&alphabet)
                    )
                    .unwrap(),
                children
            );
            for cache in [None, Some(&alphabet)] {
                assert_eq!(
                    range
                        .folded_pair(&material, &factors, suffix >> 1, Helper::Children, cache)
                        .unwrap(),
                    (children, children)
                );
            }
        };
        for challenge in [F::zero(), F::one(), F::from_u64(3)] {
            for digit in 26..32 {
                check(&[challenge; 7], digit);
            }
            for bucket in 1..4 {
                check(&[challenge; 9], bucket << (21 - 9));
            }
        }
    }
}
