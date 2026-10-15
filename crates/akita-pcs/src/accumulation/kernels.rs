use super::ledger::DecodedRamRoundFactors;
use super::range::BoundPrefixFactors;
use super::*;
use akita_error::AkitaError;
use akita_sumcheck::SumcheckKernel;
use jolt_field::{CanonicalEncoding, Field};
use jolt_poly::UnivariatePoly;

pub(crate) fn validate_provider<F: Field>(
    sources: &[ApprovedSource],
    provider: &dyn SourceProvider<F>,
) -> Result<(), AkitaError> {
    if provider.source_count() != sources.len() {
        return Err(invalid("source provider roster size"));
    }
    for (slot, source) in sources.iter().enumerate() {
        let ownership = source
            .local_ownership
            .ok_or_else(|| invalid("public roster has no private source capability"))?;
        if provider.source_id(slot)? != (ownership, source.component)
            || provider.source(slot)?.len() != source.view.physical_len()?
        {
            return Err(invalid("foreign source provider or physical extent"));
        }
    }
    Ok(())
}
pub(crate) fn read_raw<F: Field>(
    source: &ApprovedSource,
    slot: usize,
    provider: &dyn SourceProvider<F>,
    index: usize,
) -> Result<F, AkitaError> {
    let ownership = source
        .local_ownership
        .ok_or_else(|| invalid("public roster has no private source capability"))?;
    let Some(physical) = source.view.physical_index(index)? else {
        return Ok(F::zero());
    };
    let reader = provider.source(slot)?;
    if provider.source_id(slot)? != (ownership, source.component)
        || reader.len() != source.view.physical_len()?
        || physical >= reader.len()
    {
        return Err(invalid("stale source identity or physical accessor bounds"));
    }
    reader.coefficient(physical)
}

/// Streaming MLE contraction: only eliminated coordinates are summed. Caller
/// supplies a Boolean suffix encoded low-first after the bound prefix.
pub(crate) fn source_partial<F: Field>(
    source: &ApprovedSource,
    slot: usize,
    provider: &dyn SourceProvider<F>,
    bound: &[F],
    suffix: usize,
) -> Result<F, AkitaError> {
    let n = source.view.num_vars();
    if bound.len() > n || suffix >= cube_len(n - bound.len())? {
        return Err(invalid("source contraction geometry"));
    }
    let factors = BoundPrefixFactors::new(bound, source.view.num_vars())?;
    source_partial_cached(source, slot, provider, bound.len(), suffix, &factors)
}

/// The factors are for the exact low-first prefix of `bound_len` coordinates.
/// Keep every raw read and identity check, including zero equality weights.
fn source_partial_cached<F: Field>(
    source: &ApprovedSource,
    slot: usize,
    provider: &dyn SourceProvider<F>,
    bound_len: usize,
    suffix: usize,
    factors: &BoundPrefixFactors<F>,
) -> Result<F, AkitaError> {
    let n = source.view.num_vars();
    if bound_len > n || suffix >= cube_len(n - bound_len)? {
        return Err(invalid("source contraction geometry"));
    }
    let mut result = F::zero();
    factors.for_each_block(|start, low, high| {
        let mut subtotal = F::zero();
        for (offset, &weight) in low.iter().enumerate() {
            let prefix = start + offset;
            subtotal += weight * read_raw(source, slot, provider, prefix | (suffix << bound_len))?;
        }
        result += high * subtotal;
        Ok(())
    })?;
    Ok(result)
}

/// Contract both sides of one round while computing each bound-prefix factor once.
pub(super) fn source_pair_cached<F: Field>(
    source: &ApprovedSource,
    slot: usize,
    provider: &dyn SourceProvider<F>,
    bound_len: usize,
    suffix: usize,
    factors: &BoundPrefixFactors<F>,
) -> Result<(F, F), AkitaError> {
    let n = source.view.num_vars();
    if bound_len >= n || suffix >= cube_len(n - bound_len - 1)? {
        return Err(invalid("source contraction geometry"));
    }
    let mut zero = F::zero();
    let mut one = F::zero();
    factors.for_each_block(|start, low, high| {
        let mut zero_subtotal = F::zero();
        let mut one_subtotal = F::zero();
        for (offset, &weight) in low.iter().enumerate() {
            let prefix = start + offset;
            zero_subtotal +=
                weight * read_raw(source, slot, provider, prefix | (suffix << (bound_len + 1)))?;
            one_subtotal += weight
                * read_raw(
                    source,
                    slot,
                    provider,
                    prefix | (((suffix << 1) | 1) << bound_len),
                )?;
        }
        zero += high * zero_subtotal;
        one += high * one_subtotal;
        Ok(())
    })?;
    Ok((zero, one))
}

/// One Eq term uses at most 256 * ceil((mu - round - 1)/8) cached suffix factors.
struct EqRoundFactors<F> {
    zero: F,
    one: F,
    suffix: BoundPrefixFactors<F>,
}

/// Contract the masked equality table's bound prefix, not a product of MLEs.
/// Each prefix coordinate stores (Z, O, P[coordinate]); suffix tables remain
/// bounded eight-bit factors, and public support intervals stay borrowed.
struct MaskedEqRoundFactors<'a, F> {
    zero: F,
    one: F,
    prefix: Vec<(F, F, F)>,
    full_prefix: F,
    width: usize,
    suffix: BoundPrefixFactors<F>,
    intervals: &'a [std::ops::Range<usize>],
}
impl<'a, F: Field> MaskedEqRoundFactors<'a, F> {
    fn new(
        anchor: &[F],
        bound: &[F],
        intervals: &'a [std::ops::Range<usize>],
        weight: F,
    ) -> Result<Self, AkitaError> {
        let round = bound.len();
        if round >= anchor.len() {
            return Err(invalid("masked equality round dimension"));
        }
        let len = cube_len(anchor.len())?;
        let width = cube_len(round)?;
        let mut end = 0;
        for interval in intervals {
            if interval.start < end || interval.start >= interval.end || interval.end > len {
                return Err(invalid("masked equality support"));
            }
            end = interval.end;
        }
        let mut prefix = Vec::with_capacity(round);
        let mut full_prefix = F::one();
        for (&a, &x) in anchor.iter().zip(bound) {
            let zero = (F::one() - a) * (F::one() - x);
            let one = a * x;
            prefix.push((zero, one, full_prefix));
            full_prefix *= zero + one;
        }
        Ok(Self {
            zero: weight * (F::one() - anchor[round]),
            one: weight * anchor[round],
            prefix,
            full_prefix,
            width,
            suffix: BoundPrefixFactors::new(&anchor[round + 1..], anchor.len() - round - 1)?,
            intervals,
        })
    }
    /// Division-free sum over low-first Boolean addresses below `bound`.
    fn prefix_sum(&self, bound: usize) -> F {
        if bound == self.width {
            return self.full_prefix;
        }
        if bound == 0 {
            return F::zero();
        }
        let mut sum = F::zero();
        let mut high = F::one();
        for (bit, &(zero, one, low)) in self.prefix.iter().enumerate().rev() {
            if (bound >> bit) & 1 == 1 {
                sum += high * zero * low;
                high *= one;
            } else {
                high *= zero;
            }
        }
        sum
    }
    fn masked_prefix(&self, base: usize) -> F {
        // pair() checks the suffix first, so this entire cylinder is in-cube.
        let end = base + self.width;
        let first = self
            .intervals
            .partition_point(|interval| interval.end <= base);
        let mut sum = F::zero();
        for interval in &self.intervals[first..] {
            if interval.start >= end {
                break;
            }
            let low = interval.start.max(base) - base;
            let high = interval.end.min(end) - base;
            if low == 0 && high == self.width {
                sum += self.full_prefix;
            } else {
                // A zero full-prefix sum does not imply a zero partial mask.
                sum += self.prefix_sum(high) - self.prefix_sum(low);
            }
        }
        sum
    }
    fn pair(&self, suffix: usize) -> Result<(F, F), AkitaError> {
        let suffix_weight = self.suffix.weight(suffix)?;
        let base = suffix << (self.prefix.len() + 1);
        Ok((
            self.zero * suffix_weight * self.masked_prefix(base),
            self.one * suffix_weight * self.masked_prefix(base + self.width),
        ))
    }
}

enum QueryRoundFactors<'a, F: Field> {
    Eq(EqRoundFactors<F>),
    Masked(MaskedEqRoundFactors<'a, F>),
    Decoded(usize),
    Fallback,
}

/// Keep fixed 32-digit decoded metadata out of every Eq/Masked/Sparse entry.
struct QueryRoundCache<'a, F: Field> {
    terms: Vec<QueryRoundFactors<'a, F>>,
    decoded: Vec<DecodedRamRoundFactors<F>>,
}

/// All entries belong to prefixes of one common bound point. Cache only actual
/// distinct truncated lengths (at most n + 1), never one entry per source.
fn prefix_factors<'cache, F: Field>(
    cache: &'cache mut Vec<(usize, BoundPrefixFactors<F>)>,
    bound: &[F],
) -> Result<&'cache BoundPrefixFactors<F>, AkitaError> {
    // The caller always supplies prefixes of the same round's bound point.
    let index = match cache.iter().position(|(len, _)| *len == bound.len()) {
        Some(index) => index,
        None => {
            cache.push((bound.len(), BoundPrefixFactors::new(bound, bound.len())?));
            cache.len() - 1
        }
    };
    Ok(&cache[index].1)
}

/// One sealed public recipe borrowed with its formal query-index weight.
struct WeightedRecipe<'a, F: Field> {
    weight: F,
    recipe: &'a QueryRecipe<F>,
}

/// One weighted degree-two sumcheck over the common cube, after ledger seal.
/// Sealed recipe weights are compiled once in ledger order per source; recipes
/// and actual retained material stay borrowed, with no source-sized allocation.
/// For m claims, T terms and S sources, construction takes O(m + T + S) work
/// and O(T + S) recipe metadata; each query visits only its source's terms.
/// Round traversal visits only each source's local Boolean suffix; added
/// unbound axes have zero query support, while bound extra-axis factors remain.
/// Exact eight-bit prefix factors are shared by distinct truncated lengths.
pub struct WeightedQueryKernel<'a, F: Field> {
    ledger: &'a QueryLedger<F>,
    provider: &'a dyn SourceProvider<F>,
    a: F,
    queries: Vec<Vec<WeightedRecipe<'a, F>>>,
    bound: Vec<F>,
    pending: bool,
}
impl<'a, F: Field + CanonicalEncoding> WeightedQueryKernel<'a, F> {
    /// Construct a bounded borrowed query kernel only after immutable seal.
    pub fn new(
        ledger: &'a QueryLedger<F>,
        provider: &'a dyn SourceProvider<F>,
        a: F,
    ) -> Result<Self, AkitaError> {
        if !ledger.is_sealed() {
            return Err(invalid("query kernel before seal"));
        }
        validate_provider(ledger.sources(), provider)?;
        let mut counts = vec![0usize; ledger.sources().len()];
        for claim in ledger.claims() {
            for term in &claim.terms {
                let count = counts
                    .get_mut(term.source)
                    .ok_or_else(|| invalid("query source index"))?;
                *count = count
                    .checked_add(1)
                    .ok_or_else(|| invalid("query term count overflow"))?;
            }
        }
        let mut queries: Vec<Vec<WeightedRecipe<'a, F>>> =
            counts.into_iter().map(Vec::with_capacity).collect();
        let mut power = F::one();
        for claim in ledger.claims() {
            for term in &claim.terms {
                queries
                    .get_mut(term.source)
                    .ok_or_else(|| invalid("query source index"))?
                    .push(WeightedRecipe {
                        weight: power * term.coefficient,
                        recipe: &term.recipe,
                    });
            }
            power *= a;
        }
        Ok(Self {
            ledger,
            provider,
            a,
            queries,
            bound: Vec::with_capacity(ledger.limits.common_num_vars),
            pending: false,
        })
    }
    /// Challenges bound so far, in common low-first coordinate order.
    pub fn point(&self) -> &[F] {
        &self.bound
    }
    fn round_factors(
        &self,
        slot: usize,
        round: usize,
    ) -> Result<QueryRoundCache<'_, F>, AkitaError> {
        let source = self
            .ledger
            .sources()
            .get(slot)
            .ok_or_else(|| invalid("query source index"))?;
        let mu = source.view.num_vars();
        let terms = self
            .queries
            .get(slot)
            .ok_or_else(|| invalid("query source index"))?;
        if round >= mu {
            return Ok(QueryRoundCache {
                terms: Vec::new(),
                decoded: Vec::new(),
            });
        }
        let mut cache = Vec::with_capacity(terms.len());
        let decoded_count = terms
            .iter()
            .filter(|term| matches!(term.recipe, QueryRecipe::DecodedRam { .. }))
            .count();
        let mut decoded = Vec::with_capacity(decoded_count);
        for term in terms {
            let factors = match term.recipe {
                QueryRecipe::Eq { point } => {
                    let prefix = term.weight * equality(&point[..round], &self.bound[..round])?;
                    let suffix = BoundPrefixFactors::new(&point[round + 1..], mu - round - 1)?;
                    QueryRoundFactors::Eq(EqRoundFactors {
                        zero: prefix * (F::one() - point[round]),
                        one: prefix * point[round],
                        suffix,
                    })
                }
                QueryRecipe::MaskedEq { point, intervals } => QueryRoundFactors::Masked(
                    MaskedEqRoundFactors::new(point, &self.bound[..round], intervals, term.weight)?,
                ),
                QueryRecipe::DecodedRam {
                    point,
                    address_axes,
                } => {
                    let index = decoded.len();
                    decoded.push(DecodedRamRoundFactors::new(
                        point,
                        address_axes,
                        &self.bound[..round],
                    )?);
                    QueryRoundFactors::Decoded(index)
                }
                QueryRecipe::Sparse { .. } => QueryRoundFactors::Fallback,
            };
            cache.push(factors);
        }
        Ok(QueryRoundCache {
            terms: cache,
            decoded,
        })
    }
    fn query_pair_at_suffix(
        &self,
        slot: usize,
        point: &mut [F],
        round: usize,
        suffix: usize,
        round_factors: &QueryRoundCache<'_, F>,
    ) -> Result<(F, F), AkitaError> {
        let source = self
            .ledger
            .sources()
            .get(slot)
            .ok_or_else(|| invalid("query source index"))?;
        let mu = source.view.num_vars();
        let terms = self
            .queries
            .get(slot)
            .ok_or_else(|| invalid("query source index"))?;
        if point.len() != self.ledger.limits.common_num_vars
            || (round < mu && round_factors.terms.len() != terms.len())
        {
            return Err(invalid("common query point dimension"));
        }
        let mut zero = F::zero();
        let mut one = F::zero();
        if round >= mu {
            let extra = self.bound[mu..round]
                .iter()
                .fold(F::one(), |product, &value| product * (F::one() - value));
            for term in terms {
                zero += term.weight * term.recipe.evaluate(&self.bound[..mu])?;
            }
            return Ok((zero * extra, one));
        }
        for (term, cached) in terms.iter().zip(&round_factors.terms) {
            let (q0, q1) = match cached {
                QueryRoundFactors::Eq(cached) => {
                    let suffix_weight = cached.suffix.weight(suffix)?;
                    (cached.zero * suffix_weight, cached.one * suffix_weight)
                }
                QueryRoundFactors::Masked(cached) => cached.pair(suffix)?,
                QueryRoundFactors::Decoded(index) => {
                    let factors = round_factors
                        .decoded
                        .get(*index)
                        .ok_or_else(|| invalid("decoded RAM round cache index"))?;
                    let (q0, q1) = factors.pair(suffix)?;
                    (term.weight * q0, term.weight * q1)
                }
                QueryRoundFactors::Fallback => {
                    point[round] = F::zero();
                    let q0 = term.weight * term.recipe.evaluate(&point[..mu])?;
                    point[round] = F::one();
                    let q1 = term.weight * term.recipe.evaluate(&point[..mu])?;
                    (q0, q1)
                }
            };
            zero += q0;
            one += q1;
        }
        Ok((zero, one))
    }
    /// Stream every original component endpoint after the final round.
    pub fn endpoints(&self) -> Result<Vec<SourceEndpoint<F>>, AkitaError> {
        if self.pending || self.bound.len() != self.num_rounds() {
            return Err(invalid("query endpoint before final round"));
        }
        let mut cache = Vec::new();
        self.ledger
            .sources
            .iter()
            .enumerate()
            .map(|(slot, source)| {
                let bound = &self.bound[..source.view.num_vars()];
                let factors = prefix_factors(&mut cache, bound)?;
                Ok(SourceEndpoint {
                    source: slot,
                    local_ownership: source.local_ownership,
                    component: source.component,
                    value: source_partial_cached(
                        source,
                        slot,
                        self.provider,
                        bound.len(),
                        0,
                        factors,
                    )?,
                })
            })
            .collect()
    }
}
impl<F: Field + CanonicalEncoding> SumcheckKernel<F> for WeightedQueryKernel<'_, F> {
    fn num_rounds(&self) -> usize {
        self.ledger.limits.common_num_vars
    }
    fn degree_bound(&self) -> usize {
        2
    }
    fn input_claim(&self) -> F {
        self.ledger.y_at(self.a)
    }
    fn round_polynomial(
        &mut self,
        round: usize,
        claim: F,
    ) -> Result<UnivariatePoly<F>, AkitaError> {
        if self.pending || round != self.bound.len() || round >= self.num_rounds() {
            return Err(invalid("query round progression"));
        }
        let n = self.num_rounds();
        let mut coefficients = [F::zero(); 3];
        let mut point = self.bound.clone();
        point.resize(n, F::zero());
        let mut cache = Vec::new();
        for (slot, source) in self.ledger.sources.iter().enumerate() {
            // F is cylindrical on added axes; Q has Boolean support only at
            // zero there, with every bound extra-axis factor retained.
            let mu = source.view.num_vars();
            let bound_len = round.min(mu);
            let factors = prefix_factors(&mut cache, &self.bound[..bound_len])?;
            let round_factors = self.round_factors(slot, round)?;
            let has_fallback = round_factors
                .terms
                .iter()
                .any(|factors| matches!(factors, QueryRoundFactors::Fallback));
            let constant = if round >= mu {
                Some(source_partial_cached(
                    source,
                    slot,
                    self.provider,
                    bound_len,
                    0,
                    factors,
                )?)
            } else {
                None
            };
            let suffix_count = if round < mu {
                cube_len(mu - round - 1)?
            } else {
                1
            };
            for suffix in 0..suffix_count {
                if has_fallback {
                    for (axis, value) in point.iter_mut().enumerate().take(mu).skip(round + 1) {
                        *value = if (suffix >> (axis - round - 1)) & 1 == 0 {
                            F::zero()
                        } else {
                            F::one()
                        };
                    }
                }
                let (q0, q1) =
                    self.query_pair_at_suffix(slot, &mut point, round, suffix, &round_factors)?;
                if q0 == F::zero() && q1 == F::zero() {
                    continue;
                }
                let (f0, f1) = match constant {
                    Some(value) => (value, value),
                    None => {
                        source_pair_cached(source, slot, self.provider, bound_len, suffix, factors)?
                    }
                };
                let dq = q1 - q0;
                let df = f1 - f0;
                coefficients[0] += q0 * f0;
                coefficients[1] += q0 * df + dq * f0;
                coefficients[2] += dq * df;
            }
        }
        if coefficients[0] + coefficients.iter().copied().fold(F::zero(), |s, c| s + c) != claim {
            return Err(AkitaError::InvalidProof);
        }
        self.pending = true;
        Ok(UnivariatePoly::new(coefficients.to_vec()))
    }
    fn bind_challenge(&mut self, round: usize, challenge: F) -> Result<(), AkitaError> {
        if !self.pending || round != self.bound.len() {
            return Err(invalid("query challenge progression"));
        }
        self.bound.push(challenge);
        self.pending = false;
        Ok(())
    }
    fn finish(&mut self) -> Result<(), AkitaError> {
        if self.pending || self.bound.len() != self.num_rounds() {
            return Err(invalid("unfinished query sumcheck"));
        }
        Ok(())
    }
}

/// Borrow CPU providers directly. Identity is obtained from each actual reader,
/// not supplied by a callback alongside fabricated coefficient material.
pub enum CpuSourceReader<'a, F: Field + CanonicalEncoding> {
    /// Validated borrowed original RAM physical planes.
    Ram(akita_cpu_backend::CpuOriginalRamPlaneReader<'a, F>),
    /// Validated borrowed original Main column.
    Main(akita_cpu_backend::CpuOriginalMainComponentReader<'a, F>),
    /// Validated borrowed retained Dense component.
    Dense(akita_cpu_backend::CpuDecodedDenseComponentReader<'a, F>),
}
impl<F: Field + CanonicalEncoding> SourceProvider<F> for [CpuSourceReader<'_, F>] {
    fn source_count(&self) -> usize {
        self.len()
    }
    fn source_id(&self, slot: usize) -> Result<(SourceOwnership, usize), AkitaError> {
        match self
            .get(slot)
            .ok_or_else(|| invalid("CPU reader source slot"))?
        {
            CpuSourceReader::Ram(reader) => Ok((
                SourceOwnership::NativeInstalled {
                    source_id: reader.source_id(),
                    commitment_id: reader.commitment_id(),
                    backend_id: reader.backend_id(),
                    setup_digest: reader.setup_digest(),
                },
                0,
            )),
            CpuSourceReader::Main(reader) => Ok((
                SourceOwnership::NativeInstalled {
                    source_id: reader.source_id(),
                    commitment_id: reader.commitment_id(),
                    backend_id: reader.backend_id(),
                    setup_digest: reader.setup_digest(),
                },
                reader.column(),
            )),
            CpuSourceReader::Dense(reader) => Ok((
                SourceOwnership::RetainedDenseCommitment {
                    commitment_id: reader.commitment_id(),
                    backend_id: reader.backend_id(),
                    setup_digest: reader.setup_digest(),
                },
                reader.column(),
            )),
        }
    }
    fn source(
        &self,
        slot: usize,
    ) -> Result<&dyn akita_types::full_ring::FullRingCoefficientSource<F>, AkitaError> {
        match self
            .get(slot)
            .ok_or_else(|| invalid("CPU reader source slot"))?
        {
            CpuSourceReader::Ram(reader) => Ok(reader),
            CpuSourceReader::Main(reader) => Ok(reader),
            CpuSourceReader::Dense(reader) => Ok(reader),
        }
    }
}
impl<F: Field + CanonicalEncoding> SourceProvider<F> for Vec<CpuSourceReader<'_, F>> {
    fn source_count(&self) -> usize {
        self.len()
    }
    fn source_id(&self, slot: usize) -> Result<(SourceOwnership, usize), AkitaError> {
        self.as_slice().source_id(slot)
    }
    fn source(
        &self,
        slot: usize,
    ) -> Result<&dyn akita_types::full_ring::FullRingCoefficientSource<F>, AkitaError> {
        self.as_slice().source(slot)
    }
}
