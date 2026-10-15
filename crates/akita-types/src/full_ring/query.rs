//! Source predicates and ordinary-to-raw decoder-adjoint normalization.
//! All source reads are borrowed K reads. In particular no decoded/redecomposed
//! source, extension-field source copy, or random-weight division is needed.

use super::{invalid, product, RawSourceViewPlan};
use akita_algebra::offset_eq::OffsetEqWindow;
use akita_error::AkitaError;
use jolt_field::{ExtField, Field};
use std::ops::Range;

/// Sorted, disjoint physical support. Empty support is allowed. Its domain need
/// not be dyadic (original physical source lengths often are not).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoefficientSupport {
    domain_len: usize,
    intervals: Vec<Range<usize>>,
}
impl CoefficientSupport {
    pub fn new(domain_len: usize, intervals: Vec<Range<usize>>) -> Result<Self, AkitaError> {
        if domain_len == 0 {
            return Err(invalid("empty support domain"));
        }
        let mut end = 0;
        for interval in &intervals {
            if interval.start < end || interval.start >= interval.end || interval.end > domain_len {
                return Err(invalid("invalid or overlapping coefficient support"));
            }
            end = interval.end;
        }
        Ok(Self {
            domain_len,
            intervals,
        })
    }
    pub fn domain_len(&self) -> usize {
        self.domain_len
    }
    pub fn intervals(&self) -> &[Range<usize>] {
        &self.intervals
    }
    pub fn contains(&self, index: usize) -> bool {
        let i = self.intervals.partition_point(|range| range.end <= index);
        self.intervals
            .get(i)
            .is_some_and(|range| range.start <= index)
    }

    /// MLE of the *masked equality table*, not mask(point)*eq(anchor,point).
    /// Prefix contraction costs O(log N) per interval, with no N-cell mask.
    pub fn masked_equality<E: Field>(&self, anchor: &[E], point: &[E]) -> Result<E, AkitaError> {
        if !self.domain_len.is_power_of_two()
            || anchor.len() != point.len()
            || point.len() != self.domain_len.trailing_zeros() as usize
        {
            return Err(invalid("masked equality domain mismatch"));
        }
        let mut low = Vec::with_capacity(point.len() + 1);
        low.push(E::one());
        for (&a, &r) in anchor.iter().zip(point) {
            low.push(
                *low.last().ok_or(AkitaError::InvalidProof)?
                    * ((E::one() - a) * (E::one() - r) + a * r),
            );
        }
        let prefix = |bound: usize| {
            if bound == self.domain_len {
                return low[point.len()];
            }
            let mut sum = E::zero();
            let mut high = E::one();
            for bit in (0..point.len()).rev() {
                let zero = (E::one() - anchor[bit]) * (E::one() - point[bit]);
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
        Ok(self.intervals.iter().fold(E::zero(), |sum, span| {
            sum + prefix(span.end) - prefix(span.start)
        }))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceAlphabet {
    BooleanWholeGrid,
    BalancedBase32,
}

/// Exact per-source balanced-prefix work and conservative extraction ledger.
/// The factor is `tree_multiplier * tree_round_factor.pow(num_vars)`, kept
/// factored to avoid overflow. Headers and subsequent normalization are extra.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceRangeLedger {
    pub num_vars: usize,
    pub physical_cells: usize,
    pub raw_cells: usize,
    pub round_degrees: [usize; 3],
    pub degree_charge: usize,
    pub tree_multiplier: usize,
    pub tree_round_factor: usize,
    pub literal_draws: usize,
    pub proof_elements: usize,
    pub quarter_mixing_arity: usize,
    pub realization_mixing_arity: usize,
    /// Non-equality columns in the three stages; only bound work tables use E.
    pub work_columns: [usize; 3],
    /// Exact arbitrary-permutation mask scan at the verifier endpoint.
    pub zero_mask_scan_cells: usize,
}

/// The required-zero mask is supplied from authenticated producer metadata,
/// never inferred from a prover's trace length or sparse storage omissions.
#[derive(Clone, Debug)]
pub struct SourceDomainPlan {
    view: RawSourceViewPlan,
    alphabet: SourceAlphabet,
    required_physical_zeros: CoefficientSupport,
}
impl SourceDomainPlan {
    pub fn new(
        view: RawSourceViewPlan,
        alphabet: SourceAlphabet,
        required_physical_zeros: CoefficientSupport,
    ) -> Result<Self, AkitaError> {
        if required_physical_zeros.domain_len() != view.physical_len() {
            return Err(invalid("source zero support has wrong physical domain"));
        }
        Ok(Self {
            view,
            alphabet,
            required_physical_zeros,
        })
    }
    pub fn view(&self) -> &RawSourceViewPlan {
        &self.view
    }
    pub fn alphabet(&self) -> SourceAlphabet {
        self.alphabet
    }
    pub fn has_physical_zeros(&self) -> bool {
        !self.required_physical_zeros.intervals().is_empty()
    }
    pub fn range_ledger(&self) -> Result<SourceRangeLedger, AkitaError> {
        if self.alphabet != SourceAlphabet::BalancedBase32 {
            return Err(invalid("range ledger requires balanced source"));
        }
        let mu = self.view.num_vars();
        let z = usize::from(self.has_physical_zeros());
        let count = |scale: usize, constant: usize| {
            mu.checked_mul(scale)
                .and_then(|n| n.checked_add(constant))
                .ok_or_else(|| invalid("source range ledger overflow"))
        };
        Ok(SourceRangeLedger {
            num_vars: mu,
            physical_cells: self.view.physical_len(),
            raw_cells: self.view.raw_len(),
            round_degrees: [5, 5, 3],
            degree_charge: count(14 + z, 3 + z)?,
            tree_multiplier: 4 * (1 + z),
            tree_round_factor: 288,
            literal_draws: count(4, 3 + z)?,
            proof_elements: count(16, 6)?,
            quarter_mixing_arity: 4,
            realization_mixing_arity: 1 + z,
            work_columns: [4, 1, 1 + z],
            zero_mask_scan_cells: if z == 1 { self.view.raw_len() } else { 0 },
        })
    }
    pub fn required_physical_zeros(&self) -> &CoefficientSupport {
        &self.required_physical_zeros
    }
    pub fn zero_weight<E: Field>(&self, anchor: &[E], point: &[E]) -> Result<E, AkitaError> {
        if anchor.len() != self.view.num_vars() || point.len() != self.view.num_vars() {
            return Err(invalid("source zero point mismatch"));
        }
        if !self.has_physical_zeros() {
            return Ok(E::zero());
        }
        let a = OffsetEqWindow::new(anchor)?;
        let r = OffsetEqWindow::new(point)?;
        let mut sum = E::zero();
        // The registered storage permutation can turn physical intervals into
        // arbitrary raw sets. Scan that exact injection, not a guessed suffix.
        for index in 0..self.view.raw_len() {
            if self
                .view
                .raw_to_physical(index)?
                .is_some_and(|physical| self.required_physical_zeros.contains(physical))
            {
                sum += a.eval(index) * r.eval(index);
            }
        }
        Ok(sum)
    }
    pub fn endpoint<E: Field>(
        &self,
        anchor: &[E],
        eta_zero: E,
        point: &[E],
        value: E,
    ) -> Result<E, AkitaError> {
        if self.alphabet != SourceAlphabet::BooleanWholeGrid {
            return Err(invalid(
                "direct source domain requires positive Boolean alphabet",
            ));
        }
        Ok(
            super::strong::equality(anchor, point)? * value * (value - E::one())
                + eta_zero * self.zero_weight(anchor, point)? * value,
        )
    }

    /// Realizes the checked virtual MLE on the original raw source. The
    /// obligatory equation has leading coefficient one, with no gamma draw.
    pub fn range_endpoint<E: Field>(
        &self,
        tau0: &[E],
        rvirt: &[E],
        eta_zero: Option<E>,
        point: &[E],
        value: E,
    ) -> Result<E, AkitaError> {
        if self.alphabet != SourceAlphabet::BalancedBase32
            || tau0.len() != self.view.num_vars()
            || rvirt.len() != self.view.num_vars()
            || point.len() != self.view.num_vars()
            || eta_zero.is_some() != self.has_physical_zeros()
        {
            return Err(invalid("source range realization shape"));
        }
        let mut result = super::strong::equality(rvirt, point)? * value * (value + E::one());
        if let Some(eta) = eta_zero {
            result += eta * self.zero_weight(tau0, point)? * value;
        }
        Ok(result)
    }
}

/// A sparse linear decoder contribution. `raw_index` is in the registered raw
/// view; `decoded_index` is in the padded ordinary Boolean output table.
#[derive(Clone, Copy, Debug)]
pub struct DecoderTerm<K> {
    pub raw_index: usize,
    pub decoded_index: usize,
    pub coefficient: K,
}

#[derive(Clone, Debug)]
enum DecoderMap<K> {
    Positional {
        polynomial: usize,
        base: K,
        scale: K,
    },
    Sparse {
        terms: Vec<DecoderTerm<K>>,
        constants: Vec<(usize, K)>,
    },
}

/// Exact affine decoder, including aliases and permutations. The positional
/// constructor is only for the direct coefficient decoder (j,x,i), with a
/// selected original polynomial. Other frontend maps use the sparse adjoint.
#[derive(Clone, Debug)]
pub struct DecoderPlan<K> {
    view: RawSourceViewPlan,
    decoded_len: usize,
    map: DecoderMap<K>,
}

impl<K: jolt_field::CanonicalBytes> DecoderPlan<K> {
    /// Literal original affine map for transcript installation binding. This
    /// encodes, but does not authenticate, the decoder or its producer.
    pub fn canonical_descriptor_bytes(&self) -> Result<Vec<u8>, AkitaError> {
        fn size(bytes: &mut Vec<u8>, value: usize) -> Result<(), AkitaError> {
            bytes.extend_from_slice(
                &u64::try_from(value)
                    .map_err(|_| invalid("decoder descriptor size overflow"))?
                    .to_le_bytes(),
            );
            Ok(())
        }
        fn field<K: jolt_field::CanonicalBytes>(
            bytes: &mut Vec<u8>,
            value: &K,
        ) -> Result<(), AkitaError> {
            let start = bytes.len();
            let end = start
                .checked_add(K::NUM_BYTES)
                .ok_or_else(|| invalid("decoder field encoding length overflow"))?;
            bytes.resize(end, 0);
            value.to_bytes_le(&mut bytes[start..end]);
            Ok(())
        }
        let mut bytes = Vec::new();
        for value in [
            self.view.degree(),
            self.view.polynomials(),
            self.view.blocks(),
            self.view.positions(),
            self.view.digits(),
            self.decoded_len,
            K::NUM_BYTES,
        ] {
            size(&mut bytes, value)?;
        }
        if let Some(permutation) = self.view.permutation() {
            bytes.push(1);
            size(&mut bytes, permutation.len())?;
            for &index in permutation {
                size(&mut bytes, index)?;
            }
        } else {
            bytes.push(0);
        }
        match &self.map {
            DecoderMap::Positional {
                polynomial,
                base,
                scale,
            } => {
                bytes.push(0);
                size(&mut bytes, *polynomial)?;
                field(&mut bytes, base)?;
                field(&mut bytes, scale)?;
            }
            DecoderMap::Sparse { terms, constants } => {
                bytes.push(1);
                size(&mut bytes, terms.len())?;
                for term in terms {
                    size(&mut bytes, term.raw_index)?;
                    size(&mut bytes, term.decoded_index)?;
                    field(&mut bytes, &term.coefficient)?;
                }
                size(&mut bytes, constants.len())?;
                for (index, coefficient) in constants {
                    size(&mut bytes, *index)?;
                    field(&mut bytes, coefficient)?;
                }
            }
        }
        Ok(bytes)
    }
}
impl<K: Field> DecoderPlan<K> {
    pub fn positional(
        view: &RawSourceViewPlan,
        polynomial: usize,
        base: K,
        scale: K,
    ) -> Result<Self, AkitaError> {
        if polynomial >= view.polynomials() {
            return Err(invalid("decoder polynomial index"));
        }
        Ok(Self {
            view: view.clone(),
            decoded_len: product(&[view.degree(), view.positions(), view.blocks()])?,
            map: DecoderMap::Positional {
                polynomial,
                base,
                scale,
            },
        })
    }
    pub fn sparse(
        view: &RawSourceViewPlan,
        decoded_len: usize,
        terms: Vec<DecoderTerm<K>>,
        constants: Vec<(usize, K)>,
    ) -> Result<Self, AkitaError> {
        if !decoded_len.is_power_of_two() {
            return Err(invalid("decoder output is not a Boolean table"));
        }
        for term in &terms {
            if term.decoded_index >= decoded_len || view.raw_to_physical(term.raw_index)?.is_none()
            {
                return Err(invalid("decoder term outside original source"));
            }
        }
        if constants.iter().any(|(index, _)| *index >= decoded_len) {
            return Err(invalid("decoder affine constant index"));
        }
        Ok(Self {
            view: view.clone(),
            decoded_len,
            map: DecoderMap::Sparse { terms, constants },
        })
    }
    pub fn view(&self) -> &RawSourceViewPlan {
        &self.view
    }
    pub fn decoded_len(&self) -> usize {
        self.decoded_len
    }
    fn checked_point<E: Field>(&self, point: &[E]) -> Result<OffsetEqWindow<E>, AkitaError> {
        if point.len() != self.decoded_len.trailing_zeros() as usize {
            return Err(invalid("ordinary decoder point length"));
        }
        OffsetEqWindow::new(point)
    }
    pub fn affine_constant<E: ExtField<K>>(&self, point: &[E]) -> Result<E, AkitaError> {
        let eq = self.checked_point(point)?;
        Ok(match &self.map {
            DecoderMap::Positional { .. } => E::zero(),
            DecoderMap::Sparse { constants, .. } => constants
                .iter()
                .fold(E::zero(), |sum, &(index, coefficient)| {
                    sum + eq.eval(index).mul_base(coefficient)
                }),
        })
    }
    pub fn visit_adjoint<E: ExtField<K>>(
        &self,
        point: &[E],
        mut visit: impl FnMut(usize, E),
    ) -> Result<(), AkitaError> {
        let eq = self.checked_point(point)?;
        match &self.map {
            DecoderMap::Positional {
                polynomial,
                base,
                scale,
            } => {
                let mut digit_weight = *scale;
                for ell in 0..self.view.digits() {
                    for i in 0..self.view.blocks() {
                        for x in 0..self.view.positions() {
                            for j in 0..self.view.degree() {
                                let index =
                                    j + self.view.degree() * (x + self.view.positions() * i);
                                visit(
                                    self.view.raw_index(*polynomial, i, x, ell, j)?,
                                    eq.eval(index).mul_base(digit_weight),
                                );
                            }
                        }
                    }
                    digit_weight *= *base;
                }
            }
            DecoderMap::Sparse { terms, .. } => {
                for term in terms {
                    visit(
                        term.raw_index,
                        eq.eval(term.decoded_index).mul_base(term.coefficient),
                    );
                }
            }
        }
        Ok(())
    }

    /// Checked digit-axis optimization. The universal adjoint above has no
    /// denominator condition; this identity is optional and positional-only.
    pub fn positional_raw_claim<E: ExtField<K>>(
        &self,
        point: &[E],
        value: E,
    ) -> Result<(Vec<E>, E), AkitaError> {
        self.checked_point(point)?;
        let DecoderMap::Positional {
            polynomial,
            base,
            scale,
        } = &self.map
        else {
            return Err(invalid("digit-axis shortcut requires positional decoder"));
        };
        let digit_bits = self.view.padded_digits().trailing_zeros() as usize;
        let coefficient_bits = self.view.degree().trailing_zeros() as usize;
        let mut raw = Vec::with_capacity(self.view.num_vars());
        raw.extend_from_slice(&point[..coefficient_bits]);
        let mut power = *base;
        let mut factor = *scale;
        for _ in 0..digit_bits {
            let denominator = K::one() + power;
            let inverse = denominator
                .inverse()
                .ok_or_else(|| invalid("zero positional digit denominator"))?;
            raw.push(E::lift_base(power * inverse));
            factor *= denominator;
            power *= power;
        }
        raw.extend_from_slice(&point[coefficient_bits..]);
        for bit in 0..self.view.padded_polynomials().trailing_zeros() {
            raw.push(if (polynomial >> bit) & 1 == 1 {
                E::one()
            } else {
                E::zero()
            });
        }
        let inverse = factor
            .inverse()
            .ok_or_else(|| invalid("zero positional decoder scale"))?;
        Ok((raw, value.mul_base(inverse)))
    }
}

pub enum NormalizationClaim<'a, K, E> {
    Ordinary {
        decoder: &'a DecoderPlan<K>,
        point: &'a [E],
        value: E,
    },
    Raw {
        point: &'a [E],
        value: E,
    },
}

/// The entire ordered claim list, including the source-domain endpoint. The
/// caller binds that list before sampling the independent alpha vector.
pub struct NormalizationPlan<'a, K, E> {
    view: &'a RawSourceViewPlan,
    claims: Vec<NormalizationClaim<'a, K, E>>,
    alpha: Vec<E>,
}
impl<'a, K: Field, E: ExtField<K>> NormalizationPlan<'a, K, E> {
    pub fn new(
        view: &'a RawSourceViewPlan,
        claims: Vec<NormalizationClaim<'a, K, E>>,
        alpha: Vec<E>,
    ) -> Result<Self, AkitaError> {
        if claims.is_empty() || claims.len() != alpha.len() {
            return Err(invalid("normalization claim count"));
        }
        for claim in &claims {
            match claim {
                NormalizationClaim::Ordinary { decoder, point, .. } => {
                    if decoder.view() != view {
                        return Err(invalid("normalization source view mismatch"));
                    }
                    decoder.checked_point(point)?;
                }
                NormalizationClaim::Raw { point, .. } => {
                    if point.len() != view.num_vars() {
                        return Err(invalid("normalization raw point length"));
                    }
                }
            }
        }
        Ok(Self {
            view,
            claims,
            alpha,
        })
    }
    pub fn view(&self) -> &RawSourceViewPlan {
        self.view
    }
    pub fn target(&self) -> Result<E, AkitaError> {
        self.claims
            .iter()
            .zip(&self.alpha)
            .try_fold(E::zero(), |sum, (claim, &alpha)| {
                let value = match claim {
                    NormalizationClaim::Ordinary {
                        decoder,
                        point,
                        value,
                    } => *value - decoder.affine_constant(point)?,
                    NormalizationClaim::Raw { value, .. } => *value,
                };
                Ok(sum + alpha * value)
            })
    }
    pub fn visit_weights(&self, mut visit: impl FnMut(usize, E)) -> Result<(), AkitaError> {
        for (claim, &alpha) in self.claims.iter().zip(&self.alpha) {
            match claim {
                NormalizationClaim::Ordinary { decoder, point, .. } => {
                    decoder.visit_adjoint(point, |index, weight| visit(index, alpha * weight))?
                }
                NormalizationClaim::Raw { point, .. } => {
                    let eq = OffsetEqWindow::new(point)?;
                    for index in 0..self.view.raw_len() {
                        visit(index, alpha * eq.eval(index));
                    }
                }
            }
        }
        Ok(())
    }
    pub fn accumulate_columns(&self, destination: &mut [E]) -> Result<(), AkitaError> {
        if destination.len() != self.view.raw_len() {
            return Err(invalid("normalization destination length"));
        }
        self.visit_weights(|index, weight| destination[index] += weight)
    }
    pub fn weight_at(&self, point: &[E]) -> Result<E, AkitaError> {
        if point.len() != self.view.num_vars() {
            return Err(invalid("normalization endpoint length"));
        }
        let eq = OffsetEqWindow::new(point)?;
        let mut value = E::zero();
        self.visit_weights(|index, weight| value += eq.eval(index) * weight)?;
        Ok(value)
    }
    pub fn endpoint(&self, point: &[E], raw_value: E) -> Result<E, AkitaError> {
        Ok(raw_value * self.weight_at(point)?)
    }
}

/// Full physical predicate for a final inner receiver before normalization
/// and the fixed shrinking suffix. Unlike a fresh-source prefix this retains
/// the predecessor's exact F/H negative-binary spans as well as required zeros.
/// The original receiver must expose the entire identity K coefficient table;
/// this plan does not convert a gadget-decoded or tensor-packed commitment.
#[derive(Clone, Debug)]
pub struct FinalReceiverDomainPlan {
    view: RawSourceViewPlan,
    binary: CoefficientSupport,
    zeros: CoefficientSupport,
}
impl FinalReceiverDomainPlan {
    pub fn new(
        view: RawSourceViewPlan,
        binary: CoefficientSupport,
        zeros: CoefficientSupport,
    ) -> Result<Self, AkitaError> {
        if view.polynomials() != 1
            || view.digits() != 1
            || view.raw_len() != view.physical_len()
            || view
                .permutation()
                .is_some_and(|map| map.iter().enumerate().any(|(i, &j)| i != j))
            || binary.domain_len() != view.physical_len()
            || zeros.domain_len() != view.physical_len()
        {
            return Err(invalid(
                "final receiver requires the full identity source and exact physical supports",
            ));
        }
        Ok(Self {
            view,
            binary,
            zeros,
        })
    }
    /// Carry support directly from the predecessor's complete relation layout.
    /// No new guessed/uniform binary mask is introduced at finalization.
    pub fn from_relation<K: Field, E: ExtField<K>>(
        view: RawSourceViewPlan,
        relation: &super::CoefficientRelationPlan<'_, K, E>,
    ) -> Result<Self, AkitaError> {
        if view.physical_len() != relation.domain_len() {
            return Err(invalid(
                "final receiver/predecessor relation domain mismatch",
            ));
        }
        Self::new(
            view,
            relation.binary_support().clone(),
            relation.required_zero_support().clone(),
        )
    }
    pub fn view(&self) -> &RawSourceViewPlan {
        &self.view
    }
    pub fn domain_len(&self) -> usize {
        self.view.physical_len()
    }
    pub fn num_vars(&self) -> usize {
        self.view.num_vars()
    }
    pub fn binary_support(&self) -> &CoefficientSupport {
        &self.binary
    }
    pub fn required_zero_support(&self) -> &CoefficientSupport {
        &self.zeros
    }
}
