use super::{cube_len, eq_index, equality, invalid};
use akita_error::AkitaError;
use akita_types::{
    full_ring::{CoefficientSupport, FullRingCoefficientSource},
    proof::FullRingSourceId,
};
use jolt_field::{CanonicalBytes, CanonicalEncoding, Field};

/// Source geometry, distinct from the unchanged commitment profile geometry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SourceView {
    /// All original cells, including actual padding, remain in this component.
    Decoded {
        #[doc = "Dimension of the original component cube."]
        num_vars: usize,
    },
    /// Literal 26 real signed digit planes in the padded raw21 cube.
    OriginalRamPlanesV1,
}
impl SourceView {
    /// Number of coordinates in this public source view.
    pub fn num_vars(&self) -> usize {
        match self {
            Self::Decoded { num_vars } => *num_vars,
            Self::OriginalRamPlanesV1 => 21,
        }
    }
    /// Exact retained coefficient count, excluding structural virtual cells.
    pub fn physical_len(&self) -> Result<usize, AkitaError> {
        match self {
            Self::Decoded { num_vars } => cube_len(*num_vars),
            Self::OriginalRamPlanesV1 => Ok(128 * 26 * 32 * 16),
        }
    }
    pub(crate) fn physical_index(&self, raw: usize) -> Result<Option<usize>, AkitaError> {
        if raw >= cube_len(self.num_vars())? {
            return Err(invalid("source raw index out of bounds"));
        }
        match self {
            Self::Decoded { .. } => Ok(Some(raw)),
            Self::OriginalRamPlanesV1 => {
                let digit = (raw >> 7) & 31;
                Ok((digit < 26).then_some((raw & 127) + 128 * (digit + 26 * (raw >> 12))))
            }
        }
    }
}
/// Local retained capability identity. Never serialized as public evidence:
/// original public C/profile/view/column comes from the approved statement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceOwnership {
    /// Actual installed role PLUS distinct retained handle and backend/setup.
    NativeInstalled {
        /// Reusable installed A-role, not a distinct commitment identity.
        source_id: FullRingSourceId,
        /// Actual retained original handle identity.
        commitment_id: u128,
        /// Backend owner that validated the retained handle.
        backend_id: u64,
        /// Checked setup namespace of that backend.
        setup_digest: [u8; 32],
    },
    /// Actual backend-owned Dense commitment capability.
    RetainedDenseCommitment {
        /// Actual retained Dense handle identity.
        commitment_id: u128,
        /// Backend owner that validated the retained handle.
        backend_id: u64,
        /// Checked setup namespace of that backend.
        setup_digest: [u8; 32],
    },
}

/// Installed original identity/profile/view bytes are literal, not a trace hash.
/// Construction is a host contract: the outer verifier must compare this roster
/// with its approved public workload, not accept a proof-selected installation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApprovedSource {
    /// Some actual private capability for proving; None for public verification.
    pub local_ownership: Option<SourceOwnership>,
    /// Original source polynomial/column index.
    pub component: usize,
    /// Full approved public C/profile/setup/view identity, not a trace digest.
    pub canonical_identity: Vec<u8>,
    /// Exact view and coordinate geometry.
    pub view: SourceView,
}

/// Adapter to actual borrowed CPU readers; no synthetic decommitment table.
pub trait SourceProvider<F: Field> {
    /// Total number of approved component readers.
    fn source_count(&self) -> usize;
    /// Checked local source capability and original component number.
    fn source_id(&self, slot: usize) -> Result<(SourceOwnership, usize), AkitaError>;
    /// Borrow the actual fallible retained coefficient accessor.
    fn source(&self, slot: usize) -> Result<&dyn FullRingCoefficientSource<F>, AkitaError>;
}

/// Exact public MLE recipes, each acting on one approved source component.
#[derive(Clone, Debug, PartialEq)]
pub enum QueryRecipe<F> {
    /// Equality table for an original view opening.
    Eq {
        #[doc = "Opening anchor in source-view axes."]
        point: Vec<F>,
    },
    /// MLE of the masked equality TABLE, not mask(X)*eq(point,X).
    MaskedEq {
        /// Equality anchor in source-view axes.
        point: Vec<F>,
        /// Sorted disjoint Boolean-address support intervals.
        intervals: Vec<std::ops::Range<usize>>,
    },
    /// Address permutation maps decoded axes onto [a7,p5,b4].
    DecodedRam {
        /// Ordinary decoded address point with exactly 16 coordinates.
        point: Vec<F>,
        /// Maps decoded coordinates to raw coefficient/position/block axes.
        address_axes: Vec<usize>,
    },
    /// Exact sparse linear map on the source cube (affine constants live in b).
    Sparse {
        #[doc = "Sorted unique raw addresses and exact linear weights."]
        entries: Vec<(usize, F)>,
    },
}
impl<F: Field> QueryRecipe<F> {
    fn validate(&self, view: &SourceView, max_terms: usize) -> Result<(), AkitaError> {
        let n = view.num_vars();
        match self {
            Self::Eq { point } if point.len() == n => Ok(()),
            Self::MaskedEq { point, intervals }
                if point.len() == n && intervals.len() <= max_terms =>
            {
                CoefficientSupport::new(cube_len(n)?, intervals.clone()).map(|_| ())
            }
            Self::DecodedRam {
                point,
                address_axes,
            } if *view == SourceView::OriginalRamPlanesV1
                && point.len() == 16
                && address_axes.len() == 16 =>
            {
                let expected: Vec<usize> = (0..7).chain(12..21).collect();
                let mut axes = address_axes.clone();
                axes.sort_unstable();
                if axes == expected {
                    Ok(())
                } else {
                    Err(invalid("decoded RAM axis permutation"))
                }
            }
            Self::Sparse { entries } if entries.len() <= max_terms => {
                let len = cube_len(n)?;
                let mut previous = None;
                for &(index, _) in entries {
                    if index >= len || previous.is_some_and(|p| p >= index) {
                        return Err(invalid("sparse adjoint is not canonical"));
                    }
                    previous = Some(index);
                }
                Ok(())
            }
            _ => Err(invalid("query recipe does not match approved source view")),
        }
    }
    /// Evaluate the exact public table MLE, without challenge division.
    pub fn evaluate(&self, point: &[F]) -> Result<F, AkitaError> {
        match self {
            Self::Eq { point: anchor } => equality(anchor, point),
            Self::MaskedEq {
                point: anchor,
                intervals,
            } => {
                let len = cube_len(point.len())?;
                if anchor.len() != point.len() {
                    return Err(invalid("masked equality dimension"));
                }
                let mut low = [F::one(); usize::BITS as usize + 1];
                for (i, (&a, &x)) in anchor.iter().zip(point).enumerate() {
                    low[i + 1] = low[i] * ((F::one() - a) * (F::one() - x) + a * x);
                }
                let prefix = |bound: usize| {
                    if bound == len {
                        return low[point.len()];
                    }
                    let mut sum = F::zero();
                    let mut high = F::one();
                    for bit in (0..point.len()).rev() {
                        let zero = (F::one() - anchor[bit]) * (F::one() - point[bit]);
                        let one = anchor[bit] * point[bit];
                        if (bound >> bit) & 1 == 1 {
                            sum += high * zero * low[bit];
                            high *= one;
                        } else {
                            high *= zero;
                        }
                    }
                    sum
                };
                let mut end = 0;
                let mut sum = F::zero();
                for interval in intervals {
                    if interval.start < end || interval.start >= interval.end || interval.end > len
                    {
                        return Err(invalid("masked equality support"));
                    }
                    end = interval.end;
                    sum += prefix(interval.end) - prefix(interval.start);
                }
                Ok(sum)
            }
            Self::DecodedRam {
                point: anchor,
                address_axes,
            } => {
                if point.len() != 21 || anchor.len() != 16 || address_axes.len() != 16 {
                    return Err(invalid("decoded RAM endpoint dimension"));
                }
                let mut address = F::one();
                for (&a, &axis) in anchor.iter().zip(address_axes) {
                    let x = *point.get(axis).ok_or_else(|| invalid("decoded RAM axis"))?;
                    address *= (F::one() - a) * (F::one() - x) + a * x;
                }
                let mut digit = F::zero();
                let mut power = F::one();
                for ell in 0..26 {
                    digit += power * eq_index(&point[7..12], ell);
                    power *= F::from_u64(32);
                }
                Ok(address * digit)
            }
            Self::Sparse { entries } => {
                let len = cube_len(point.len())?;
                if entries.iter().any(|&(index, _)| index >= len) {
                    return Err(invalid("sparse query endpoint index"));
                }
                Ok(entries
                    .iter()
                    .fold(F::zero(), |sum, &(index, coefficient)| {
                        sum + coefficient * eq_index(point, index)
                    }))
            }
        }
    }
}

/// Unweighted decoded-RAM recipe factors at one low-first sumcheck round.
/// Address suffix tables cover at most eight axes each; the digit table has
/// at most 32 cells, including the six zero-weight virtual planes.
pub(super) struct DecodedRamRoundFactors<F: Field> {
    round: usize,
    suffix_count: usize,
    address_pair: (F, F),
    address_suffix: super::range::BoundPrefixFactors<F>,
    digit: [F; 32],
}
impl<F: Field> DecodedRamRoundFactors<F> {
    /// The sealed ledger has already admitted the address-axis permutation.
    pub(super) fn new(
        anchor: &[F],
        address_axes: &[usize],
        bound: &[F],
    ) -> Result<Self, AkitaError> {
        let round = bound.len();
        if anchor.len() != 16 || address_axes.len() != 16 || round >= 21 {
            return Err(invalid("decoded RAM round dimension"));
        }
        // Compact address coordinates are ascending raw axes, not decoded order.
        let mut address = [F::zero(); 16];
        for (&a, &axis) in anchor.iter().zip(address_axes) {
            let index = match axis {
                0..=6 => axis,
                12..=20 => axis - 5,
                _ => return Err(invalid("decoded RAM round address axis")),
            };
            address[index] = a;
        }
        let mut prefix = F::one();
        for (index, &a) in address.iter().enumerate() {
            let axis = if index < 7 { index } else { index + 5 };
            if let Some(&x) = bound.get(axis) {
                prefix *= (F::one() - a) * (F::one() - x) + a * x;
            }
        }
        let (address_pair, suffix_start) = if round < 7 {
            let a = address[round];
            ((prefix * (F::one() - a), prefix * a), round + 1)
        } else if round < 12 {
            ((prefix, prefix), 7)
        } else {
            let index = round - 5;
            let a = address[index];
            ((prefix * (F::one() - a), prefix * a), index + 1)
        };
        let suffix_anchor = &address[suffix_start..];
        let address_suffix =
            super::range::BoundPrefixFactors::new(suffix_anchor, suffix_anchor.len())?;
        let mut digit = [F::zero(); 32];
        let mut power = F::one();
        let radix = F::from_u64(32);
        for ell in 0..26 {
            if round < 7 {
                digit[ell] = power;
            } else if round < 12 {
                // Removing the bound low digit bits leaves b | (suffixDigit << 1).
                digit[ell >> (round - 7)] += power * eq_index(&bound[7..round], ell);
            } else {
                digit[0] += power * eq_index(&bound[7..12], ell);
            }
            power *= radix;
        }
        Ok(Self {
            round,
            suffix_count: cube_len(20 - round)?,
            address_pair,
            address_suffix,
            digit,
        })
    }
    pub(super) fn pair(&self, suffix: usize) -> Result<(F, F), AkitaError> {
        if suffix >= self.suffix_count {
            return Err(invalid("decoded RAM round suffix"));
        }
        if self.round < 7 {
            let coefficient_bits = 6 - self.round;
            let low_mask = (1usize << coefficient_bits) - 1;
            // Remove raw digit axes 7..12 from the Boolean address suffix.
            let address = (suffix & low_mask) | ((suffix >> 5) & !low_mask);
            let digit = (suffix >> coefficient_bits) & 31;
            let factor = self.address_suffix.weight(address)? * self.digit[digit];
            Ok((self.address_pair.0 * factor, self.address_pair.1 * factor))
        } else if self.round < 12 {
            let digit_bits = 11 - self.round;
            let digit = (suffix & ((1usize << digit_bits) - 1)) << 1;
            let address = self.address_pair.0 * self.address_suffix.weight(suffix >> digit_bits)?;
            Ok((address * self.digit[digit], address * self.digit[digit + 1]))
        } else {
            let factor = self.address_suffix.weight(suffix)? * self.digit[0];
            Ok((self.address_pair.0 * factor, self.address_pair.1 * factor))
        }
    }
}

/// One public adjoint contribution to one approved source column.
#[derive(Clone, Debug, PartialEq)]
pub struct QueryTerm<F> {
    /// Frozen component roster index.
    pub source: usize,
    /// Public linear scaling, including zero.
    pub coefficient: F,
    /// Exact public source-view adjoint.
    pub recipe: QueryRecipe<F>,
}
/// One independent affine scalar equation with its own formal exponent.
#[derive(Clone, Debug, PartialEq)]
pub struct ScalarClaim<F> {
    /// Globally unique contiguous formal Z exponent.
    pub exponent: usize,
    /// Asserted scalar value.
    pub value: F,
    /// Public affine decoder offset.
    pub offset: F,
    /// Exact source contributions, retained even with zero weights.
    pub terms: Vec<QueryTerm<F>>,
}

/// All limits are installed BEFORE any challenge, not derived from observed work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LedgerLimits {
    /// Maximum independent scalar equation count, strictly below q.
    pub claim_horizon: usize,
    /// Maximum retained source component count.
    pub max_sources: usize,
    /// Maximum source contributions in one scalar equation.
    pub max_terms_per_claim: usize,
    /// Maximum sparse entries or mask intervals in one public recipe.
    pub max_recipe_terms: usize,
    /// Maximum literal inherited/context evidence bytes.
    pub max_evidence_bytes: usize,
    /// Common query cube dimension, not the global range dimension.
    pub common_num_vars: usize,
    /// Trusted cap on common-cube cells.
    pub max_cube_cells: usize,
}
/// Immutable-source append-only formal query relation and literal evidence.
#[derive(Clone, Debug)]
pub struct QueryLedger<F> {
    pub(crate) sources: Vec<ApprovedSource>,
    pub(crate) claims: Vec<ScalarClaim<F>>,
    pub(crate) limits: LedgerLimits,
    evidence: Vec<u8>,
    sealed: bool,
    pub(crate) range_endpoints_appended: bool,
}
impl<F: Field + CanonicalEncoding> QueryLedger<F> {
    /// Admit the selected prime field, immutable sources and trusted limits.
    pub fn new(
        sources: Vec<ApprovedSource>,
        limits: LedgerLimits,
        installed_context: Vec<u8>,
    ) -> Result<Self, AkitaError> {
        // This selected implementation is E=K, not an advertised degree-two ring.
        const Q: u128 = u128::MAX - (1u128 << 32) + 22538;
        let mut modulus = [0u8; 32];
        modulus[16..].copy_from_slice(&Q.to_be_bytes());
        if akita_params::field_modulus_be_bytes::<F>()? != modulus
            || limits.claim_horizon as u128 >= Q
        {
            return Err(invalid(
                "query ledger requires selected prime field and m_horizon < q",
            ));
        }
        let context_len = installed_context
            .len()
            .checked_add(8)
            .ok_or_else(|| invalid("installed context framing overflow"))?;
        if sources.is_empty()
            || sources.len() > limits.max_sources
            || context_len > limits.max_evidence_bytes
            || cube_len(limits.common_num_vars)? > limits.max_cube_cells
        {
            return Err(invalid("query ledger installed limits"));
        }
        for (i, source) in sources.iter().enumerate() {
            if source.canonical_identity.is_empty()
                || source.canonical_identity.len() > limits.max_evidence_bytes
                || source.view.num_vars() > limits.common_num_vars
                || (source.view == SourceView::OriginalRamPlanesV1
                    && (source.component != 0
                        || !matches!(
                            source.local_ownership,
                            Some(SourceOwnership::NativeInstalled { .. }) | None
                        )))
                || sources[..i].iter().any(|old| {
                    if source.local_ownership.is_none() {
                        // The public ordered roster can contain repeated C's.
                        // Private handle aliases cannot be inferred from C equality.
                        false
                    } else {
                        old.local_ownership == source.local_ownership
                            && (old.component == source.component
                                || old.canonical_identity != source.canonical_identity
                                || old.view != source.view)
                    }
                })
            {
                return Err(invalid("query roster identity, component or view mismatch"));
            }
            source.view.physical_len()?;
        }
        let mut evidence = Vec::with_capacity(context_len);
        size(&mut evidence, installed_context.len())?;
        evidence.extend_from_slice(&installed_context);
        Ok(Self {
            sources,
            claims: Vec::new(),
            limits,
            evidence,
            sealed: false,
            range_endpoints_appended: false,
        })
    }
    /// Immutable source/view/column roster in final endpoint order.
    pub fn sources(&self) -> &[ApprovedSource] {
        &self.sources
    }
    /// All independent scalar equations in exact exponent order.
    pub fn claims(&self) -> &[ScalarClaim<F>] {
        &self.claims
    }
    /// Whether future appends and evidence changes are permanently forbidden.
    pub fn is_sealed(&self) -> bool {
        self.sealed
    }
    /// Fixed-source query error numerator (m-1)+2n under uniform draws.
    pub fn query_degree_charge(&self) -> Result<usize, AkitaError> {
        if !self.sealed || self.claims.is_empty() {
            return Err(invalid("query degree ledger before seal"));
        }
        self.limits
            .common_num_vars
            .checked_mul(2)
            .and_then(|v| v.checked_add(self.claims.len() - 1))
            .ok_or_else(|| invalid("query degree ledger overflow"))
    }
    /// Append one equation atomically after checking next exponent and shape.
    pub fn append(&mut self, claim: ScalarClaim<F>) -> Result<(), AkitaError> {
        if self.sealed
            || claim.exponent != self.claims.len()
            || self.claims.len() >= self.limits.claim_horizon
            || claim.terms.len() > self.limits.max_terms_per_claim
        {
            return Err(invalid(
                "sealed ledger, exponent collision or claim horizon",
            ));
        }
        for term in &claim.terms {
            let source = self
                .sources
                .get(term.source)
                .ok_or_else(|| invalid("foreign query source"))?;
            term.recipe
                .validate(&source.view, self.limits.max_recipe_terms)?;
        }
        self.claims.push(claim);
        Ok(())
    }
    /// Retain full canonical FE/range evidence, never replace it with a digest.
    pub fn bind_evidence(&mut self, evidence: &[u8]) -> Result<(), AkitaError> {
        if self.sealed {
            return Err(invalid("evidence after query seal"));
        }
        let end = self
            .evidence
            .len()
            .checked_add(8)
            .and_then(|n| n.checked_add(evidence.len()))
            .ok_or_else(|| invalid("evidence overflow"))?;
        if end > self.limits.max_evidence_bytes {
            return Err(invalid("evidence byte horizon"));
        }
        size(&mut self.evidence, evidence.len())?;
        self.evidence.extend_from_slice(evidence);
        Ok(())
    }
    /// Freeze endpoint ownership/order ONLY: values and points come after a.
    pub fn seal(&mut self) -> Result<Vec<u8>, AkitaError>
    where
        F: CanonicalBytes,
    {
        if self.sealed
            || self.claims.is_empty()
            || (self
                .sources
                .iter()
                .any(|s| s.view == SourceView::OriginalRamPlanesV1)
                && !self.range_endpoints_appended)
        {
            return Err(invalid("duplicate, empty or missing-range query seal"));
        }
        let bytes = self.canonical_bytes()?;
        self.sealed = true;
        Ok(bytes)
    }
    /// Evaluate the formal scalar ledger Y at the query-index draw.
    pub fn y_at(&self, a: F) -> F {
        self.claims
            .iter()
            .rev()
            .fold(F::zero(), |y, claim| y * a + claim.value - claim.offset)
    }
    /// Evaluate Q for one source, with zero support on extra query axes.
    pub fn query_at(&self, source: usize, a: F, point: &[F]) -> Result<F, AkitaError> {
        let view = &self
            .sources
            .get(source)
            .ok_or_else(|| invalid("query source index"))?
            .view;
        if point.len() != self.limits.common_num_vars {
            return Err(invalid("common query point dimension"));
        }
        let mut sum = F::zero();
        let mut power = F::one();
        for claim in &self.claims {
            for term in &claim.terms {
                if term.source == source {
                    sum += power
                        * term.coefficient
                        * term.recipe.evaluate(&point[..view.num_vars()])?;
                }
            }
            power *= a;
        }
        Ok(sum
            * point[view.num_vars()..]
                .iter()
                .fold(F::one(), |v, &r| v * (F::one() - r)))
    }
    /// Check full terminal ownership and the independently derived weighted sum.
    pub fn verify_query_endpoint(
        &self,
        a: F,
        point: &[F],
        endpoints: &[SourceEndpoint<F>],
        terminal: F,
    ) -> Result<(), AkitaError> {
        if !self.sealed || endpoints.len() != self.sources.len() {
            return Err(invalid("query endpoint ownership or size"));
        }
        let mut expected = F::zero();
        for (slot, (source, endpoint)) in self.sources.iter().zip(endpoints).enumerate() {
            if endpoint.source != slot
                || endpoint.local_ownership != source.local_ownership
                || endpoint.component != source.component
            {
                return Err(invalid("query endpoint source order"));
            }
            expected += self.query_at(slot, a, point)? * endpoint.value;
        }
        if expected != terminal {
            return Err(AkitaError::InvalidProof);
        }
        Ok(())
    }
}
/// An original source component evaluation with local capability provenance.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceEndpoint<F> {
    /// Frozen source component roster index.
    pub source: usize,
    /// Some actual retained capability for proving; None for public verification.
    pub local_ownership: Option<SourceOwnership>,
    /// Original polynomial/column index.
    pub component: usize,
    /// Claimed evaluation at the derived source-view point.
    pub value: F,
}

pub(crate) fn size(bytes: &mut Vec<u8>, value: usize) -> Result<(), AkitaError> {
    bytes.extend_from_slice(
        &u64::try_from(value)
            .map_err(|_| invalid("canonical size overflow"))?
            .to_le_bytes(),
    );
    Ok(())
}
fn scalar<F: CanonicalBytes>(bytes: &mut Vec<u8>, value: &F) -> Result<(), AkitaError> {
    let start = bytes.len();
    let end = start
        .checked_add(F::NUM_BYTES)
        .ok_or_else(|| invalid("scalar encoding overflow"))?;
    bytes.resize(end, 0);
    value.to_bytes_le(&mut bytes[start..end]);
    Ok(())
}
impl<F: Field + CanonicalEncoding + CanonicalBytes> QueryLedger<F> {
    /// Shape-only canonical transport bound for every admitted query recipe.
    ///
    /// `max_source_identity_bytes` bounds EACH identity in the trusted public
    /// roster (not a proof-selected identity). Evidence includes the installed
    /// context's LE64 frame and all later LE64-framed evidence records. The
    /// installed evidence cap is retained even if the supplied cap is smaller.
    /// Private ownership contributes no bytes. Every size operation is checked.
    pub fn canonical_size_bound(
        limits: LedgerLimits,
        max_source_identity_bytes: usize,
        max_evidence_bytes: usize,
    ) -> Result<usize, AkitaError> {
        cube_len(limits.common_num_vars)?;
        let add = |a: usize, b: usize| {
            a.checked_add(b)
                .ok_or_else(|| invalid("canonical ledger bound overflow"))
        };
        let mul = |a: usize, b: usize| {
            a.checked_mul(b)
                .ok_or_else(|| invalid("canonical ledger bound overflow"))
        };
        let n = limits.common_num_vars;
        let r = limits.max_recipe_terms;
        let point_bytes = mul(n, F::NUM_BYTES)?;
        let equality_bytes = add(9, point_bytes)?;
        let masked_bytes = add(add(equality_bytes, 8)?, mul(r, 16)?)?;
        let decoded_bytes = add(add(equality_bytes, 8)?, mul(n, 8)?)?;
        let sparse_bytes = add(9, mul(r, add(8, F::NUM_BYTES)?)?)?;
        let recipe_bytes = equality_bytes
            .max(masked_bytes)
            .max(decoded_bytes)
            .max(sparse_bytes);
        let term_bytes = add(add(8, F::NUM_BYTES)?, recipe_bytes)?;
        let claim_bytes = add(
            add(16, mul(2, F::NUM_BYTES)?)?,
            mul(limits.max_terms_per_claim, term_bytes)?,
        )?;
        let source_bytes = add(25, max_source_identity_bytes)?;
        let fixed_bytes = add(b"akita/query-ledger/v1".len(), 8 * 8 + 3 * 8)?;
        let with_evidence = add(
            fixed_bytes,
            max_evidence_bytes.max(limits.max_evidence_bytes),
        )?;
        add(
            add(with_evidence, mul(limits.max_sources, source_bytes)?)?,
            mul(limits.claim_horizon, claim_bytes)?,
        )
    }
    /// Canonical literal public ledger encoding; excludes private capability IDs.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, AkitaError> {
        let mut b = b"akita/query-ledger/v1".to_vec();
        for limit in [
            self.limits.claim_horizon,
            self.limits.max_sources,
            self.limits.max_terms_per_claim,
            self.limits.max_recipe_terms,
            self.limits.max_evidence_bytes,
            self.limits.common_num_vars,
            self.limits.max_cube_cells,
            F::NUM_BYTES,
        ] {
            size(&mut b, limit)?;
        }
        size(&mut b, self.evidence.len())?;
        b.extend_from_slice(&self.evidence);
        size(&mut b, self.sources.len())?;
        for source in &self.sources {
            size(&mut b, source.component)?;
            b.push(if source.view == SourceView::OriginalRamPlanesV1 {
                1
            } else {
                0
            });
            size(&mut b, source.view.num_vars())?;
            size(&mut b, source.canonical_identity.len())?;
            b.extend_from_slice(&source.canonical_identity);
        }
        size(&mut b, self.claims.len())?;
        for claim in &self.claims {
            size(&mut b, claim.exponent)?;
            scalar(&mut b, &claim.value)?;
            scalar(&mut b, &claim.offset)?;
            size(&mut b, claim.terms.len())?;
            for term in &claim.terms {
                size(&mut b, term.source)?;
                scalar(&mut b, &term.coefficient)?;
                match &term.recipe {
                    QueryRecipe::Eq { point }
                    | QueryRecipe::MaskedEq { point, .. }
                    | QueryRecipe::DecodedRam { point, .. } => {
                        b.push(match &term.recipe {
                            QueryRecipe::Eq { .. } => 0,
                            QueryRecipe::MaskedEq { .. } => 1,
                            _ => 2,
                        });
                        size(&mut b, point.len())?;
                        for value in point {
                            scalar(&mut b, value)?;
                        }
                        if let QueryRecipe::MaskedEq { intervals, .. } = &term.recipe {
                            size(&mut b, intervals.len())?;
                            for span in intervals {
                                size(&mut b, span.start)?;
                                size(&mut b, span.end)?;
                            }
                        }
                        if let QueryRecipe::DecodedRam { address_axes, .. } = &term.recipe {
                            size(&mut b, address_axes.len())?;
                            for &axis in address_axes {
                                size(&mut b, axis)?;
                            }
                        }
                    }
                    QueryRecipe::Sparse { entries } => {
                        b.push(3);
                        size(&mut b, entries.len())?;
                        for (index, value) in entries {
                            size(&mut b, *index)?;
                            scalar(&mut b, value)?;
                        }
                    }
                }
            }
        }
        // Endpoint ownership is the same full source order, regardless of Q=0.
        Ok(b)
    }
}

#[cfg(test)]
mod arithmetic_tests {
    use super::*;
    use jolt_field::{One, Ring, Zero};
    type F = akita_config::proof_optimized::fp128::Field;

    fn direct_pair(recipe: &QueryRecipe<F>, bound: &[F], suffix: usize) -> (F, F) {
        let round = bound.len();
        let mut point = [F::zero(); 21];
        point[..round].copy_from_slice(bound);
        for (i, x) in point[round + 1..].iter_mut().enumerate() {
            *x = F::from_u64(((suffix >> i) & 1) as u64);
        }
        let zero = recipe.evaluate(&point).unwrap();
        point[round] = F::one();
        (zero, recipe.evaluate(&point).unwrap())
    }

    #[test]
    fn decoded_rounds_match_direct_across_permutations_and_chunk_boundaries() {
        let natural: [usize; 16] = std::array::from_fn(|i| if i < 7 { i } else { i + 5 });
        let permutations = [
            natural,
            std::array::from_fn(|i| natural[15 - i]),
            std::array::from_fn(|i| natural[(5 * i + 3) & 15]),
        ];
        let anchor: [F; 16] = std::array::from_fn(|i| F::from_u64((3 * i + 2) as u64));
        for axes in permutations {
            let recipe = QueryRecipe::DecodedRam {
                point: anchor.to_vec(),
                address_axes: axes.to_vec(),
            };
            for kind in 0..3 {
                let point: [F; 21] = std::array::from_fn(|i| match kind {
                    0 => F::zero(),
                    1 => F::one(),
                    _ => F::from_u64((i + 3) as u64),
                });
                for round in 0..21 {
                    let bound = &point[..round];
                    let factors = DecodedRamRoundFactors::new(&anchor, &axes, bound).unwrap();
                    let address_bits = (round + 1..21)
                        .filter(|axis| !(7..12).contains(axis))
                        .count();
                    let address_count = 1usize << address_bits;
                    for address in [0, 1, 127, 128, 255, 256, 257, 511, 512, address_count - 1] {
                        if address >= address_count {
                            continue;
                        }
                        for digit in [0usize, 1, 15, 24, 25, 26, 30, 31] {
                            // Pack raw axes independently of the helper's hole extraction.
                            let mut suffix = 0;
                            let mut compact_axis = 0;
                            for axis in round + 1..21 {
                                let bit = if (7..12).contains(&axis) {
                                    (digit >> (axis - 7)) & 1
                                } else {
                                    let bit = (address >> compact_axis) & 1;
                                    compact_axis += 1;
                                    bit
                                };
                                suffix |= bit << (axis - round - 1);
                            }
                            assert_eq!(
                                factors.pair(suffix).unwrap(),
                                direct_pair(&recipe, bound, suffix),
                                "round={round}, kind={kind}, axes={axes:?}, address={address}, digit={digit}"
                            );
                        }
                    }
                    assert!(factors.pair(cube_len(20 - round).unwrap()).is_err());
                }
            }
        }
    }

    #[test]
    fn decoded_digit_powers_use_field_arithmetic_and_zero_virtual_planes() {
        let natural: [usize; 16] = std::array::from_fn(|i| if i < 7 { i } else { i + 5 });
        let axes: [usize; 16] = std::array::from_fn(|i| natural[(5 * i + 3) & 15]);
        let address = 0xa57busize;
        let raw_address = (address & 127) | ((address >> 7) << 12);
        let anchor: [F; 16] =
            std::array::from_fn(|i| F::from_u64(((raw_address >> axes[i]) & 1) as u64));
        let recipe = QueryRecipe::DecodedRam {
            point: anchor.to_vec(),
            address_axes: axes.to_vec(),
        };
        let mut power = F::one();
        for digit in 0..32 {
            let raw = raw_address | (digit << 7);
            let point: [F; 21] =
                std::array::from_fn(|axis| F::from_u64(((raw >> axis) & 1) as u64));
            for round in 0..21 {
                let bound = &point[..round];
                let factors = DecodedRamRoundFactors::new(&anchor, &axes, bound).unwrap();
                let suffix = raw >> (round + 1);
                let pair = factors.pair(suffix).unwrap();
                assert_eq!(pair, direct_pair(&recipe, bound, suffix));
                let value = if (raw >> round) & 1 == 0 {
                    pair.0
                } else {
                    pair.1
                };
                assert_eq!(
                    value,
                    if digit < 26 { power } else { F::zero() },
                    "round={round}, digit={digit}"
                );
            }
            // Repeated field addition is independent of integer/extension encodings.
            power = (0..32).fold(F::zero(), |sum, _| sum + power);
        }
    }

    #[test]
    fn decoded_round_geometry_rejects_invalid_dimensions_axes_and_suffixes() {
        let anchor = [F::from_u64(3); 16];
        let mut axes: [usize; 16] = std::array::from_fn(|i| if i < 7 { i } else { i + 5 });
        assert!(DecodedRamRoundFactors::new(&anchor[..15], &axes, &[]).is_err());
        assert!(DecodedRamRoundFactors::new(&anchor, &axes[..15], &[]).is_err());
        assert!(DecodedRamRoundFactors::new(&anchor, &axes, &[F::zero(); 21]).is_err());
        for axis in [7, 11, 21, usize::MAX] {
            axes[0] = axis;
            assert!(DecodedRamRoundFactors::new(&anchor, &axes, &[]).is_err());
        }
    }
}
