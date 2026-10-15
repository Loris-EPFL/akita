//! Full-coefficient native strong reductions for the selected (4,4) range
//! tree. The virtual table is MLE_x[W(x)(W(x)+1)], never the square of W's
//! off-cube value. Sources stay in K; only half-sized bound work tables use E.

use super::{
    invalid, CoefficientRelationPlan, CoefficientSupport, NormalizationPlan, RawSourceViewPlan,
    SourceAlphabet, SourceDomainPlan,
};
use akita_algebra::offset_eq::eq_eval_at_index;
use akita_error::AkitaError;
use akita_params::FullRingRangePlan;
use jolt_field::{ExtField, Field};

/// Read capability on an already owned, original K coefficient source. Sparse
/// and compact backends implement the same read without materializing a second
/// table. Ownership and commitment authority remain with the calling backend.
pub trait FullRingCoefficientSource<K> {
    fn len(&self) -> usize;
    fn coefficient(&self, index: usize) -> Result<K, AkitaError>;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<K: Copy> FullRingCoefficientSource<K> for [K] {
    fn len(&self) -> usize {
        <[K]>::len(self)
    }
    fn coefficient(&self, index: usize) -> Result<K, AkitaError> {
        self.get(index)
            .copied()
            .ok_or_else(|| invalid("full-ring source index"))
    }
}
impl<K: Copy> FullRingCoefficientSource<K> for Vec<K> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
    fn coefficient(&self, index: usize) -> Result<K, AkitaError> {
        self.as_slice()
            .get(index)
            .copied()
            .ok_or_else(|| invalid("full-ring source index"))
    }
}

/// Borrowed raw injection over the retained physical K source. Virtual holes
/// read zero before any range transform; required physical zeros read actual data.
pub struct RawSourceReader<'a, K> {
    source: &'a dyn FullRingCoefficientSource<K>,
    view: &'a RawSourceViewPlan,
}
impl<'a, K> RawSourceReader<'a, K> {
    pub fn new(
        source: &'a dyn FullRingCoefficientSource<K>,
        view: &'a RawSourceViewPlan,
    ) -> Result<Self, AkitaError> {
        if source.len() != view.physical_len() {
            return Err(invalid("raw reader physical extent"));
        }
        Ok(Self { source, view })
    }
}
impl<K: Field> FullRingCoefficientSource<K> for RawSourceReader<'_, K> {
    fn len(&self) -> usize {
        self.view.raw_len()
    }
    fn coefficient(&self, index: usize) -> Result<K, AkitaError> {
        match self.view.raw_to_physical(index)? {
            Some(physical) => self.source.coefficient(physical),
            None => Ok(K::zero()),
        }
    }
}

pub fn equality<E: Field>(left: &[E], right: &[E]) -> Result<E, AkitaError> {
    if left.len() != right.len() {
        return Err(invalid("equality point lengths"));
    }
    Ok(left.iter().zip(right).fold(E::one(), |p, (&a, &b)| {
        p * ((E::one() - a) * (E::one() - b) + a * b)
    }))
}
pub fn affine_range_weights<E: Field>(xi: [E; 3]) -> [E; 4] {
    FullRingRangePlan::affine_weights(xi)
}
pub fn affine_range_leaves<E: Field>(svirt: E) -> [E; 16] {
    FullRingRangePlan::affine_leaves(svirt)
}
pub fn range_leaf_endpoint<E: Field>(svirt: E, xi: [E; 3]) -> E {
    let leaves = affine_range_leaves(svirt);
    affine_range_weights(xi)
        .iter()
        .zip(leaves.chunks_exact(4))
        .fold(E::zero(), |sum, (&weight, factors)| {
            sum + weight * factors.iter().copied().fold(E::one(), |p, value| p * value)
        })
}

#[derive(Clone, Copy)]
enum SourceTransform {
    Identity,
    Virtual,
    Quarter(usize),
}
impl SourceTransform {
    fn apply<K: Field>(self, value: K) -> K {
        match self {
            Self::Identity => value,
            Self::Virtual => value * (value + K::one()),
            Self::Quarter(quarter) => {
                let s = value * (value + K::one());
                (4 * quarter..4 * quarter + 4)
                    .fold(K::one(), |p, k| p * (s - K::from_u64((k * (k + 1)) as u64)))
            }
        }
    }
}

enum Column<'a, K, E> {
    Base {
        source: &'a dyn FullRingCoefficientSource<K>,
        view: Option<RawSourceViewPlan>,
        transform: SourceTransform,
    },
    Dense(Vec<E>),
    Equality {
        anchor: Vec<E>,
        bound: usize,
        scale: E,
    },
    Masked {
        anchor: Vec<E>,
        support: CoefficientSupport,
    },
    RawMasked {
        anchor: Vec<E>,
        view: RawSourceViewPlan,
        physical: CoefficientSupport,
    },
}
impl<K: Field, E: ExtField<K> + jolt_field::Fold> Column<'_, K, E> {
    fn base_value(
        source: &dyn FullRingCoefficientSource<K>,
        view: &Option<RawSourceViewPlan>,
        transform: SourceTransform,
        index: usize,
    ) -> Result<K, AkitaError> {
        let value = if let Some(view) = view {
            match view.raw_to_physical(index)? {
                Some(index) => source.coefficient(index)?,
                None => K::zero(),
            }
        } else {
            source.coefficient(index)?
        };
        Ok(transform.apply(value))
    }
    fn value(&self, index: usize) -> Result<E, AkitaError> {
        match self {
            Self::Base {
                source,
                view,
                transform,
            } => Ok(E::lift_base(Self::base_value(
                *source, view, *transform, index,
            )?)),
            Self::Dense(values) => values
                .get(index)
                .copied()
                .ok_or_else(|| invalid("bound column index")),
            Self::Equality {
                anchor,
                bound,
                scale,
            } => Ok(*scale * eq_eval_at_index(&anchor[*bound..], index)),
            Self::Masked { anchor, support } => Ok(if support.contains(index) {
                eq_eval_at_index(anchor, index)
            } else {
                E::zero()
            }),
            Self::RawMasked {
                anchor,
                view,
                physical,
            } => Ok(
                if view
                    .raw_to_physical(index)?
                    .is_some_and(|i| physical.contains(i))
                {
                    eq_eval_at_index(anchor, index)
                } else {
                    E::zero()
                },
            ),
        }
    }
    fn bind(&mut self, challenge: E, len: usize) -> Result<(), AkitaError> {
        match self {
            Self::Equality {
                anchor,
                bound,
                scale,
            } => {
                let a = *anchor.get(*bound).ok_or(AkitaError::InvalidProof)?;
                *scale *= (E::one() - a) * (E::one() - challenge) + a * challenge;
                *bound += 1;
            }
            Self::Dense(values) => {
                if values.len() != len {
                    return Err(invalid("sumcheck dense column length"));
                }
                akita_sumcheck::fold_evals_in_place(values, challenge);
            }
            Self::Base {
                source,
                view,
                transform,
            } => {
                // First nonbase binding allocates only the half table. No
                // original K source is copied, lifted wholesale, or replaced.
                let mut values = Vec::with_capacity(len / 2);
                for index in 0..len / 2 {
                    let left = Self::base_value(*source, view, *transform, 2 * index)?;
                    let right = Self::base_value(*source, view, *transform, 2 * index + 1)?;
                    values.push(E::lift_base(left) + challenge.mul_base(right - left));
                }
                *self = Self::Dense(values);
            }
            Self::Masked { .. } | Self::RawMasked { .. } => {
                let mut values = Vec::with_capacity(len / 2);
                for index in 0..len / 2 {
                    let left = self.value(2 * index)?;
                    values.push(left + challenge * (self.value(2 * index + 1)? - left));
                }
                *self = Self::Dense(values);
            }
        }
        Ok(())
    }
}

struct ProductTerm<E> {
    weight: E,
    factors: Vec<(usize, E)>,
}

/// Internal full-coefficient sumcheck engine. Each factor is one MLE column
/// plus a constant; terms may reuse a column (e.g. W(W+1)). Message degree is
/// enforced structurally, including all zero high coefficients on the wire.
struct FullCoefficientSession<'a, K, E> {
    columns: Vec<Column<'a, K, E>>,
    terms: Vec<ProductTerm<E>>,
    degree: usize,
    len: usize,
    point: Vec<E>,
    claim: E,
    pending: Option<Vec<E>>,
}
impl<'a, K: Field, E: ExtField<K> + jolt_field::Fold> FullCoefficientSession<'a, K, E> {
    fn new(
        len: usize,
        degree: usize,
        columns: Vec<Column<'a, K, E>>,
        terms: Vec<ProductTerm<E>>,
        claim: E,
    ) -> Result<Self, AkitaError> {
        if !len.is_power_of_two()
            || degree == 0
            || degree > 5
            || terms.iter().any(|term| {
                term.factors.len() > degree
                    || term
                        .factors
                        .iter()
                        .any(|(column, _)| *column >= columns.len())
            })
        {
            return Err(invalid("full-coefficient sumcheck shape"));
        }
        Ok(Self {
            columns,
            terms,
            degree,
            len,
            point: Vec::new(),
            claim,
            pending: None,
        })
    }
    fn round_coefficients(&mut self) -> Result<&[E], AkitaError> {
        if self.len == 1 {
            return Err(invalid("sumcheck has no remaining round"));
        }
        if self.pending.is_none() {
            let mut coefficients = vec![E::zero(); self.degree + 1];
            let mut pairs = vec![(E::zero(), E::zero()); self.columns.len()];
            for index in 0..self.len / 2 {
                for (column, pair) in self.columns.iter().zip(&mut pairs) {
                    let left = column.value(2 * index)?;
                    *pair = (left, column.value(2 * index + 1)? - left);
                }
                for term in &self.terms {
                    let mut polynomial = [E::zero(); 6];
                    polynomial[0] = term.weight;
                    for (degree, &(column, offset)) in term.factors.iter().enumerate() {
                        let (constant, linear) = pairs[column];
                        let constant = constant + offset;
                        polynomial[degree + 1] = polynomial[degree] * linear;
                        for coefficient in (1..=degree).rev() {
                            polynomial[coefficient] = polynomial[coefficient] * constant
                                + polynomial[coefficient - 1] * linear;
                        }
                        polynomial[0] *= constant;
                    }
                    for (sum, coefficient) in coefficients.iter_mut().zip(polynomial) {
                        *sum += coefficient;
                    }
                }
            }
            let sum = coefficients
                .iter()
                .copied()
                .fold(coefficients[0], |a, b| a + b);
            if sum != self.claim {
                return Err(AkitaError::InvalidProof);
            }
            self.pending = Some(coefficients);
        }
        self.pending.as_deref().ok_or(AkitaError::InvalidProof)
    }
    fn bind(&mut self, challenge: E) -> Result<(), AkitaError> {
        let coefficients = self
            .pending
            .take()
            .ok_or_else(|| invalid("bind requires a sent full-coefficient message"))?;
        for column in &mut self.columns {
            column.bind(challenge, self.len)?;
        }
        self.claim = coefficients
            .iter()
            .rev()
            .fold(E::zero(), |value, &coefficient| {
                value * challenge + coefficient
            });
        self.len /= 2;
        self.point.push(challenge);
        Ok(())
    }
    fn endpoint(&self) -> Result<(), AkitaError> {
        if self.len != 1 || self.pending.is_some() {
            return Err(invalid("unfinished sumcheck"));
        }
        let mut value = E::zero();
        for term in &self.terms {
            let mut product = term.weight;
            for &(column, offset) in &term.factors {
                product *= self.columns[column].value(0)? + offset;
            }
            value += product;
        }
        if value != self.claim {
            return Err(AkitaError::InvalidProof);
        }
        Ok(())
    }
}

/// Producer for the exact two-stage degree-five (4,4) range tree. Four quarter
/// claims are sent before the independent affine triple xi is sampled.
pub struct ProductRangeSession<'a, K, E> {
    source: &'a dyn FullRingCoefficientSource<K>,
    session: FullCoefficientSession<'a, K, E>,
    leaf_stage: bool,
}
impl<'a, K: Field, E: ExtField<K> + jolt_field::Fold> ProductRangeSession<'a, K, E> {
    pub fn new(
        source: &'a dyn FullRingCoefficientSource<K>,
        tau0: &[E],
    ) -> Result<Self, AkitaError> {
        check_source_point(source, tau0)?;
        let mut columns = Vec::with_capacity(5);
        columns.push(Column::Equality {
            anchor: tau0.to_vec(),
            bound: 0,
            scale: E::one(),
        });
        for quarter in 0..4 {
            columns.push(Column::Base {
                source,
                view: None,
                transform: SourceTransform::Quarter(quarter),
            });
        }
        let terms = vec![ProductTerm {
            weight: E::one(),
            factors: (0..5).map(|column| (column, E::zero())).collect(),
        }];
        let session = FullCoefficientSession::new(
            source.len(),
            FullRingRangePlan::ROUND_DEGREES[0],
            columns,
            terms,
            E::zero(),
        )?;
        Ok(Self {
            source,
            session,
            leaf_stage: false,
        })
    }
    pub fn round_coefficients(&mut self) -> Result<[E; 6], AkitaError> {
        self.session
            .round_coefficients()?
            .try_into()
            .map_err(|_| AkitaError::InvalidProof)
    }
    pub fn bind(&mut self, challenge: E) -> Result<(), AkitaError> {
        self.session.bind(challenge)
    }
    pub fn remaining_rounds(&self) -> usize {
        self.session.len.trailing_zeros() as usize
    }
    pub fn point(&self) -> &[E] {
        &self.session.point
    }
    pub fn quarter_claims(&self) -> Result<[E; 4], AkitaError> {
        if self.leaf_stage {
            return Err(invalid("quarter claims already consumed"));
        }
        self.session.endpoint()?;
        Ok([
            self.session.columns[1].value(0)?,
            self.session.columns[2].value(0)?,
            self.session.columns[3].value(0)?,
            self.session.columns[4].value(0)?,
        ])
    }
    pub fn begin_leaf_stage(&mut self, xi: [E; 3]) -> Result<(), AkitaError> {
        let claims = self.quarter_claims()?;
        let weights = affine_range_weights(xi);
        let target = weights
            .iter()
            .zip(claims)
            .fold(E::zero(), |sum, (&weight, value)| sum + weight * value);
        let columns = vec![
            Column::Equality {
                anchor: self.session.point.clone(),
                bound: 0,
                scale: E::one(),
            },
            Column::Base {
                source: self.source,
                view: None,
                transform: SourceTransform::Virtual,
            },
        ];
        let terms = weights
            .iter()
            .enumerate()
            .map(|(quarter, &weight)| {
                let mut factors = vec![(0, E::zero())];
                factors.extend(
                    (4 * quarter..4 * quarter + 4).map(|k| (1, -E::from_u64((k * (k + 1)) as u64))),
                );
                ProductTerm { weight, factors }
            })
            .collect();
        self.session = FullCoefficientSession::new(
            self.source.len(),
            FullRingRangePlan::ROUND_DEGREES[1],
            columns,
            terms,
            target,
        )?;
        self.leaf_stage = true;
        Ok(())
    }
    pub fn virtual_claim(&self) -> Result<E, AkitaError> {
        if !self.leaf_stage {
            return Err(invalid("missing range leaf stage"));
        }
        self.session.endpoint()?;
        self.session.columns[1].value(0)
    }
}

/// Fused endpoint's independently sampled coefficients; not powers of a
/// single challenge. The protocol samples the triple AFTER svirt is bound.
#[derive(Clone, Copy, Debug)]
pub struct StrongMix<E> {
    pub gamma: E,
    pub eta_bin: E,
    pub eta_zero: E,
}
impl<E> From<[E; 3]> for StrongMix<E> {
    fn from([gamma, eta_bin, eta_zero]: [E; 3]) -> Self {
        Self {
            gamma,
            eta_bin,
            eta_zero,
        }
    }
}

pub fn fused_endpoint<K: Field, E: ExtField<K>>(
    relation: &CoefficientRelationPlan<'_, K, E>,
    rho: &[E],
    tau0: &[E],
    rvirt: &[E],
    mix: StrongMix<E>,
    point: &[E],
    value: E,
) -> Result<E, AkitaError> {
    let linear = relation.evaluate_column_mle(rho, point)?;
    let bin = relation.binary_support().masked_equality(rvirt, point)?;
    let zero = relation
        .required_zero_support()
        .masked_equality(tau0, point)?;
    let virtual_weight = equality(rvirt, point)?;
    Ok(value * linear
        + (mix.gamma * virtual_weight + mix.eta_bin * bin) * value * (value + E::one())
        + mix.eta_zero * zero * value)
}

/// Producer of degree-three full coefficient messages for all linear,
/// virtual-range, negative-binary and required-zero terms on the SAME W.
pub struct FusedRelationSession<'a, K, E> {
    session: FullCoefficientSession<'a, K, E>,
}
impl<'a, K: Field, E: ExtField<K> + jolt_field::Fold> FusedRelationSession<'a, K, E> {
    pub fn new(
        source: &'a dyn FullRingCoefficientSource<K>,
        relation: &CoefficientRelationPlan<'_, K, E>,
        rho: &[E],
        tau0: &[E],
        rvirt: &[E],
        svirt: E,
        mix: StrongMix<E>,
    ) -> Result<Self, AkitaError> {
        check_source_point(source, tau0)?;
        check_source_point(source, rvirt)?;
        if source.len() != relation.domain_len() {
            return Err(invalid("fused source/relation domain mismatch"));
        }
        let weights = relation.weights(rho)?;
        let mut lambda = vec![E::zero(); source.len()];
        weights.accumulate_columns(&mut lambda)?;
        let target = weights.rhs() + mix.gamma * svirt;
        let columns = vec![
            Column::Base {
                source,
                view: None,
                transform: SourceTransform::Identity,
            },
            Column::Dense(lambda),
            Column::Equality {
                anchor: rvirt.to_vec(),
                bound: 0,
                scale: E::one(),
            },
            Column::Masked {
                anchor: rvirt.to_vec(),
                support: relation.binary_support().clone(),
            },
            Column::Masked {
                anchor: tau0.to_vec(),
                support: relation.required_zero_support().clone(),
            },
        ];
        let zero = E::zero();
        let terms = vec![
            ProductTerm {
                weight: E::one(),
                factors: vec![(0, zero), (1, zero)],
            },
            ProductTerm {
                weight: mix.gamma,
                factors: vec![(2, zero), (0, zero), (0, E::one())],
            },
            ProductTerm {
                weight: mix.eta_bin,
                factors: vec![(3, zero), (0, zero), (0, E::one())],
            },
            ProductTerm {
                weight: mix.eta_zero,
                factors: vec![(4, zero), (0, zero)],
            },
        ];
        Ok(Self {
            session: FullCoefficientSession::new(
                source.len(),
                FullRingRangePlan::FUSED_DEGREE,
                columns,
                terms,
                target,
            )?,
        })
    }
    pub fn round_coefficients(&mut self) -> Result<[E; 4], AkitaError> {
        self.session
            .round_coefficients()?
            .try_into()
            .map_err(|_| AkitaError::InvalidProof)
    }
    pub fn bind(&mut self, challenge: E) -> Result<(), AkitaError> {
        self.session.bind(challenge)
    }
    pub fn point(&self) -> &[E] {
        &self.session.point
    }
    pub fn remaining_rounds(&self) -> usize {
        self.session.len.trailing_zeros() as usize
    }
    pub fn witness_value(&self) -> Result<E, AkitaError> {
        self.session.endpoint()?;
        self.session.columns[0].value(0)
    }
}

/// Direct positive-Boolean source-domain producer, with the declared physical
/// required-zero term. Balanced sources use the separate product-range grammar.
pub struct SourceDomainSession<'a, K, E> {
    session: FullCoefficientSession<'a, K, E>,
}
impl<'a, K: Field, E: ExtField<K> + jolt_field::Fold> SourceDomainSession<'a, K, E> {
    pub fn new(
        source: &'a dyn FullRingCoefficientSource<K>,
        plan: &SourceDomainPlan,
        anchor: &[E],
        eta_zero: E,
    ) -> Result<Self, AkitaError> {
        if plan.alphabet() != SourceAlphabet::BooleanWholeGrid
            || source.len() != plan.view().physical_len()
            || anchor.len() != plan.view().num_vars()
        {
            return Err(invalid("source domain geometry"));
        }
        let zero = E::zero();
        let columns = vec![
            Column::Base {
                source,
                view: Some(plan.view().clone()),
                transform: SourceTransform::Identity,
            },
            Column::Equality {
                anchor: anchor.to_vec(),
                bound: 0,
                scale: E::one(),
            },
            Column::RawMasked {
                anchor: anchor.to_vec(),
                view: plan.view().clone(),
                physical: plan.required_physical_zeros().clone(),
            },
        ];
        let terms = vec![
            ProductTerm {
                weight: E::one(),
                factors: vec![(1, zero), (0, zero), (0, -E::one())],
            },
            ProductTerm {
                weight: eta_zero,
                factors: vec![(2, zero), (0, zero)],
            },
        ];
        Ok(Self {
            session: FullCoefficientSession::new(plan.view().raw_len(), 3, columns, terms, zero)?,
        })
    }
    pub fn round_coefficients(&mut self) -> Result<&[E], AkitaError> {
        self.session.round_coefficients()
    }
    pub fn bind(&mut self, challenge: E) -> Result<(), AkitaError> {
        self.session.bind(challenge)
    }
    pub fn point(&self) -> &[E] {
        &self.session.point
    }
    pub fn remaining_rounds(&self) -> usize {
        self.session.len.trailing_zeros() as usize
    }
    pub fn raw_value(&self) -> Result<E, AkitaError> {
        self.session.endpoint()?;
        self.session.columns[0].value(0)
    }
}

/// Degree-three realization of a balanced source's checked virtual frontier.
/// The physical reader is mapped once, and no relation or receiver is fabricated.
pub struct SourceRangeRealizationSession<'a, K, E> {
    session: FullCoefficientSession<'a, K, E>,
}
impl<'a, K: Field, E: ExtField<K> + jolt_field::Fold> SourceRangeRealizationSession<'a, K, E> {
    pub fn new(
        source: &'a dyn FullRingCoefficientSource<K>,
        plan: &SourceDomainPlan,
        tau0: &[E],
        rvirt: &[E],
        svirt: E,
        eta_zero: Option<E>,
    ) -> Result<Self, AkitaError> {
        if plan.alphabet() != SourceAlphabet::BalancedBase32
            || source.len() != plan.view().physical_len()
            || tau0.len() != plan.view().num_vars()
            || rvirt.len() != plan.view().num_vars()
            || eta_zero.is_some() != plan.has_physical_zeros()
        {
            return Err(invalid("source range realization geometry"));
        }
        let zero = E::zero();
        let adjunct = usize::from(eta_zero.is_some());
        let mut columns = Vec::with_capacity(2 + adjunct);
        columns.extend([
            Column::Base {
                source,
                view: Some(plan.view().clone()),
                transform: SourceTransform::Identity,
            },
            Column::Equality {
                anchor: rvirt.to_vec(),
                bound: 0,
                scale: E::one(),
            },
        ]);
        let mut terms = Vec::with_capacity(1 + adjunct);
        terms.push(ProductTerm {
            weight: E::one(),
            factors: vec![(1, zero), (0, zero), (0, E::one())],
        });
        if let Some(eta) = eta_zero {
            columns.push(Column::RawMasked {
                anchor: tau0.to_vec(),
                view: plan.view().clone(),
                physical: plan.required_physical_zeros().clone(),
            });
            terms.push(ProductTerm {
                weight: eta,
                factors: vec![(2, zero), (0, zero)],
            });
        }
        Ok(Self {
            session: FullCoefficientSession::new(plan.view().raw_len(), 3, columns, terms, svirt)?,
        })
    }
    pub fn round_coefficients(&mut self) -> Result<&[E], AkitaError> {
        self.session.round_coefficients()
    }
    pub fn bind(&mut self, challenge: E) -> Result<(), AkitaError> {
        self.session.bind(challenge)
    }
    pub fn point(&self) -> &[E] {
        &self.session.point
    }
    pub fn raw_value(&self) -> Result<E, AkitaError> {
        self.session.endpoint()?;
        self.session.columns[0].value(0)
    }
}

/// Degree-two normalization producer on the original source, using exactly
/// the same adjoint plan as the verifier (including affine public constants).
pub struct NormalizationSession<'a, K, E> {
    session: FullCoefficientSession<'a, K, E>,
}
impl<'a, K: Field, E: ExtField<K> + jolt_field::Fold> NormalizationSession<'a, K, E> {
    pub fn new(
        source: &'a dyn FullRingCoefficientSource<K>,
        plan: &NormalizationPlan<'_, K, E>,
    ) -> Result<Self, AkitaError> {
        if source.len() != plan.view().physical_len() {
            return Err(invalid("normalization original source length"));
        }
        let mut weights = vec![E::zero(); plan.view().raw_len()];
        plan.accumulate_columns(&mut weights)?;
        let columns = vec![
            Column::Base {
                source,
                view: Some(plan.view().clone()),
                transform: SourceTransform::Identity,
            },
            Column::Dense(weights),
        ];
        let terms = vec![ProductTerm {
            weight: E::one(),
            factors: vec![(0, E::zero()), (1, E::zero())],
        }];
        Ok(Self {
            session: FullCoefficientSession::new(
                plan.view().raw_len(),
                2,
                columns,
                terms,
                plan.target()?,
            )?,
        })
    }
    pub fn round_coefficients(&mut self) -> Result<[E; 3], AkitaError> {
        self.session
            .round_coefficients()?
            .try_into()
            .map_err(|_| AkitaError::InvalidProof)
    }
    pub fn bind(&mut self, challenge: E) -> Result<(), AkitaError> {
        self.session.bind(challenge)
    }
    pub fn point(&self) -> &[E] {
        &self.session.point
    }
    pub fn remaining_rounds(&self) -> usize {
        self.session.len.trailing_zeros() as usize
    }
    pub fn raw_value(&self) -> Result<E, AkitaError> {
        self.session.endpoint()?;
        self.session.columns[0].value(0)
    }
}

fn check_source_point<K, E>(
    source: &dyn FullRingCoefficientSource<K>,
    point: &[E],
) -> Result<(), AkitaError> {
    if !source.len().is_power_of_two() || point.len() != source.len().trailing_zeros() as usize {
        return Err(invalid("full-grid source/point mismatch"));
    }
    Ok(())
}

/// Degree-three endpoint for the final receiver domain alone. This protocol
/// has no coefficient relation or rho: its target is gamma*svirt and its
/// three terms restore full-grid base32, exact F/H binarity and required zeros.
pub fn final_receiver_domain_endpoint<E: Field>(
    domain: &super::FinalReceiverDomainPlan,
    tau0: &[E],
    rvirt: &[E],
    mix: StrongMix<E>,
    point: &[E],
    value: E,
) -> Result<E, AkitaError> {
    if point.len() != domain.num_vars() {
        return Err(invalid("final receiver endpoint domain"));
    }
    let bin = domain.binary_support().masked_equality(rvirt, point)?;
    let zero = domain
        .required_zero_support()
        .masked_equality(tau0, point)?;
    Ok(
        (mix.gamma * equality(rvirt, point)? + mix.eta_bin * bin) * value * (value + E::one())
            + mix.eta_zero * zero * value,
    )
}

/// Final receiver's domain-only fused producer, following the complete native
/// ProductRangeSession. There is deliberately no linear-weight column, no
/// fabricated zero relation and no discarded pending ordinary opening: the
/// driver must normalize this endpoint with that original pending evaluation.
pub struct FinalReceiverDomainSession<'a, K, E> {
    session: FullCoefficientSession<'a, K, E>,
}
impl<'a, K: Field, E: ExtField<K> + jolt_field::Fold> FinalReceiverDomainSession<'a, K, E> {
    pub fn new(
        source: &'a dyn FullRingCoefficientSource<K>,
        domain: &super::FinalReceiverDomainPlan,
        tau0: &[E],
        rvirt: &[E],
        svirt: E,
        mix: StrongMix<E>,
    ) -> Result<Self, AkitaError> {
        check_source_point(source, tau0)?;
        check_source_point(source, rvirt)?;
        if source.len() != domain.domain_len() {
            return Err(invalid("final receiver source domain"));
        }
        let zero = E::zero();
        let columns = vec![
            Column::Base {
                source,
                view: None,
                transform: SourceTransform::Identity,
            },
            Column::Equality {
                anchor: rvirt.to_vec(),
                bound: 0,
                scale: E::one(),
            },
            Column::Masked {
                anchor: rvirt.to_vec(),
                support: domain.binary_support().clone(),
            },
            Column::Masked {
                anchor: tau0.to_vec(),
                support: domain.required_zero_support().clone(),
            },
        ];
        let terms = vec![
            ProductTerm {
                weight: mix.gamma,
                factors: vec![(1, zero), (0, zero), (0, E::one())],
            },
            ProductTerm {
                weight: mix.eta_bin,
                factors: vec![(2, zero), (0, zero), (0, E::one())],
            },
            ProductTerm {
                weight: mix.eta_zero,
                factors: vec![(3, zero), (0, zero)],
            },
        ];
        Ok(Self {
            session: FullCoefficientSession::new(
                source.len(),
                FullRingRangePlan::FUSED_DEGREE,
                columns,
                terms,
                mix.gamma * svirt,
            )?,
        })
    }
    pub fn round_coefficients(&mut self) -> Result<[E; 4], AkitaError> {
        self.session
            .round_coefficients()?
            .try_into()
            .map_err(|_| AkitaError::InvalidProof)
    }
    pub fn bind(&mut self, challenge: E) -> Result<(), AkitaError> {
        self.session.bind(challenge)
    }
    pub fn point(&self) -> &[E] {
        &self.session.point
    }
    pub fn remaining_rounds(&self) -> usize {
        self.session.len.trailing_zeros() as usize
    }
    pub fn witness_value(&self) -> Result<E, AkitaError> {
        self.session.endpoint()?;
        self.session.columns[0].value(0)
    }
}
