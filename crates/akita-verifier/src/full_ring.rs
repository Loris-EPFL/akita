//! Verifier replay for full-ring coefficient rows and the selected strong
//! reductions. Transcript drivers absorb full coefficient messages BEFORE
//! deriving each challenge, and supply the independent vector draws at the
//! typed boundaries below. No legacy alpha, EOR or setup-deferred branch is
//! used here. Returned evaluations are pending openings, not proof acceptance.

use akita_error::AkitaError;
use akita_params::FullRingRangePlan;
use akita_types::full_ring::{
    affine_range_weights, equality, fused_endpoint, range_leaf_endpoint, CoefficientRelationPlan,
    NormalizationPlan, SourceAlphabet, SourceDomainPlan, StrongMix,
};
use jolt_field::{ExtField, Field};

fn invalid(message: &str) -> AkitaError {
    AkitaError::InvalidInput(message.into())
}

/// Complete univariate-coefficient replay. Unlike the legacy compressed
/// message format this checks exactly degree+1 coefficients, including the
/// constant and all zero high coefficients. Point order is low-bit-first.
pub struct FullCoefficientReplay<E> {
    rounds: usize,
    degree: usize,
    claim: E,
    point: Vec<E>,
    pending: Option<Vec<E>>,
}
impl<E: Field> FullCoefficientReplay<E> {
    pub fn new(rounds: usize, degree: usize, claim: E) -> Result<Self, AkitaError> {
        if rounds >= usize::BITS as usize || degree == 0 || degree > 5 {
            return Err(invalid("full-ring sumcheck shape"));
        }
        Ok(Self {
            rounds,
            degree,
            claim,
            point: Vec::with_capacity(rounds),
            pending: None,
        })
    }
    /// Check the sent message before asking the transcript for the challenge.
    pub fn receive_coefficients(&mut self, coefficients: &[E]) -> Result<(), AkitaError> {
        if self.point.len() >= self.rounds
            || self.pending.is_some()
            || coefficients.len() != self.degree + 1
        {
            return Err(AkitaError::InvalidProof);
        }
        let sum = coefficients
            .iter()
            .copied()
            .fold(coefficients[0], |a, b| a + b);
        if sum != self.claim {
            return Err(AkitaError::InvalidProof);
        }
        self.pending = Some(coefficients.to_vec());
        Ok(())
    }
    pub fn bind(&mut self, challenge: E) -> Result<(), AkitaError> {
        let coefficients = self.pending.take().ok_or(AkitaError::InvalidProof)?;
        self.claim = coefficients
            .iter()
            .rev()
            .fold(E::zero(), |value, &coefficient| {
                value * challenge + coefficient
            });
        self.point.push(challenge);
        Ok(())
    }
    pub fn point(&self) -> &[E] {
        &self.point
    }
    pub fn remaining_rounds(&self) -> usize {
        self.rounds - self.point.len()
    }
    pub fn finish(&self, expected: E) -> Result<(), AkitaError> {
        if self.point.len() != self.rounds || self.pending.is_some() || self.claim != expected {
            return Err(AkitaError::InvalidProof);
        }
        Ok(())
    }
}

/// Checked virtual frontier. The value is MLE(W(W+1)), not W(r)(W(r)+1).
/// It is only constructed after both degree-five product stages pass.
#[derive(Clone, Debug)]
pub struct FullRingVirtualClaim<E> {
    point: Vec<E>,
    value: E,
    tau0: Vec<E>,
}
impl<E: Copy> FullRingVirtualClaim<E> {
    pub fn point(&self) -> &[E] {
        &self.point
    }
    pub fn value(&self) -> E {
        self.value
    }
    pub fn range_anchor(&self) -> &[E] {
        &self.tau0
    }
}

/// The two degree5 stages with four intermediate claims, independent
/// normalized affine weights (1,xi1,xi2,xi3), and sixteen affine leaves tied
/// to the ONE sent svirt. No unchecked child evaluations are discarded.
pub struct FullRingRangeVerifier<E> {
    tau0: Vec<E>,
    anchor: Vec<E>,
    replay: FullCoefficientReplay<E>,
    quarters: Option<[E; 4]>,
    xi: Option<[E; 3]>,
    finished: bool,
}
impl<E: Field> FullRingRangeVerifier<E> {
    pub fn new(tau0: &[E]) -> Result<Self, AkitaError> {
        Ok(Self {
            tau0: tau0.to_vec(),
            anchor: tau0.to_vec(),
            replay: FullCoefficientReplay::new(
                tau0.len(),
                FullRingRangePlan::ROUND_DEGREES[0],
                E::zero(),
            )?,
            quarters: None,
            xi: None,
            finished: false,
        })
    }
    pub fn receive_coefficients(&mut self, coefficients: &[E]) -> Result<(), AkitaError> {
        if self.finished || (self.quarters.is_some() && self.xi.is_none()) {
            return Err(AkitaError::InvalidProof);
        }
        self.replay.receive_coefficients(coefficients)
    }
    pub fn bind(&mut self, challenge: E) -> Result<(), AkitaError> {
        self.replay.bind(challenge)
    }
    pub fn point(&self) -> &[E] {
        self.replay.point()
    }
    pub fn remaining_rounds(&self) -> usize {
        self.replay.remaining_rounds()
    }
    pub fn receive_quarters(&mut self, quarters: [E; 4]) -> Result<(), AkitaError> {
        if self.quarters.is_some() || self.xi.is_some() || self.finished {
            return Err(AkitaError::InvalidProof);
        }
        let product = quarters.iter().copied().fold(E::one(), |a, b| a * b);
        self.replay
            .finish(equality(&self.anchor, self.replay.point())? * product)?;
        self.quarters = Some(quarters);
        Ok(())
    }
    /// Invoke only after the four quarter values are transcript-bound.
    pub fn begin_leaf_stage(&mut self, xi: [E; 3]) -> Result<(), AkitaError> {
        if self.xi.is_some() || self.finished {
            return Err(AkitaError::InvalidProof);
        }
        let quarters = self.quarters.ok_or(AkitaError::InvalidProof)?;
        let weights = affine_range_weights(xi);
        let claim = weights
            .iter()
            .zip(quarters)
            .fold(E::zero(), |sum, (&weight, value)| sum + weight * value);
        self.anchor = self.replay.point().to_vec();
        self.replay = FullCoefficientReplay::new(
            self.anchor.len(),
            FullRingRangePlan::ROUND_DEGREES[1],
            claim,
        )?;
        self.xi = Some(xi);
        Ok(())
    }
    pub fn receive_virtual(&mut self, svirt: E) -> Result<FullRingVirtualClaim<E>, AkitaError> {
        if self.finished {
            return Err(AkitaError::InvalidProof);
        }
        let xi = self.xi.ok_or(AkitaError::InvalidProof)?;
        self.replay.finish(
            equality(&self.anchor, self.replay.point())? * range_leaf_endpoint(svirt, xi),
        )?;
        self.finished = true;
        Ok(FullRingVirtualClaim {
            point: self.replay.point().to_vec(),
            value: svirt,
            tau0: self.tau0.clone(),
        })
    }
}

/// One checked reduction endpoint which still needs the original source
/// commitment's opening/descendant continuation. It cannot mean acceptance.
#[derive(Clone, Debug)]
pub struct FullRingPendingEvaluation<E> {
    point: Vec<E>,
    value: E,
}
impl<E: Copy> FullRingPendingEvaluation<E> {
    pub fn point(&self) -> &[E] {
        &self.point
    }
    pub fn value(&self) -> E {
        self.value
    }
    pub fn into_parts(self) -> (Vec<E>, E) {
        (self.point, self.value)
    }
}

pub struct FullRingFusedVerifier<'p, 'a, K, E> {
    relation: &'p CoefficientRelationPlan<'a, K, E>,
    rho: Vec<E>,
    tau0: Vec<E>,
    virtual_claim: FullRingVirtualClaim<E>,
    mix: StrongMix<E>,
    replay: FullCoefficientReplay<E>,
}
impl<'p, 'a, K: Field, E: ExtField<K>> FullRingFusedVerifier<'p, 'a, K, E> {
    /// The mix is the fresh independent (gamma,eta_bin,eta_zero) vector drawn
    /// only after the verified virtual frontier's scalar was bound.
    pub fn new(
        relation: &'p CoefficientRelationPlan<'a, K, E>,
        rho: &[E],
        tau0: &[E],
        virtual_claim: FullRingVirtualClaim<E>,
        mix: StrongMix<E>,
    ) -> Result<Self, AkitaError> {
        if tau0.len() != relation.num_vars()
            || virtual_claim.point.len() != relation.num_vars()
            || tau0 != virtual_claim.range_anchor()
        {
            return Err(invalid("full-ring fused point geometry"));
        }
        let target = relation.rhs_at(rho)? + mix.gamma * virtual_claim.value;
        Ok(Self {
            relation,
            rho: rho.to_vec(),
            tau0: tau0.to_vec(),
            virtual_claim,
            mix,
            replay: FullCoefficientReplay::new(
                relation.num_vars(),
                FullRingRangePlan::FUSED_DEGREE,
                target,
            )?,
        })
    }
    pub fn receive_coefficients(&mut self, coefficients: &[E]) -> Result<(), AkitaError> {
        self.replay.receive_coefficients(coefficients)
    }
    pub fn bind(&mut self, challenge: E) -> Result<(), AkitaError> {
        self.replay.bind(challenge)
    }
    pub fn point(&self) -> &[E] {
        self.replay.point()
    }
    pub fn remaining_rounds(&self) -> usize {
        self.replay.remaining_rounds()
    }
    pub fn finish(self, value: E) -> Result<FullRingPendingEvaluation<E>, AkitaError> {
        // This is the mandatory real public matrix scan. Every term uses the
        // same received value; neither a weight division nor a deferred setup
        // claim may bypass it.
        let expected = fused_endpoint(
            self.relation,
            &self.rho,
            &self.tau0,
            &self.virtual_claim.point,
            self.mix,
            self.replay.point(),
            value,
        )?;
        self.replay.finish(expected)?;
        Ok(FullRingPendingEvaluation {
            point: self.replay.point,
            value,
        })
    }
}

pub struct FullRingSourceDomainVerifier<'a, E> {
    plan: &'a SourceDomainPlan,
    anchor: Vec<E>,
    eta_zero: E,
    replay: FullCoefficientReplay<E>,
}
impl<'a, E: Field> FullRingSourceDomainVerifier<'a, E> {
    pub fn new(plan: &'a SourceDomainPlan, anchor: &[E], eta_zero: E) -> Result<Self, AkitaError> {
        if plan.alphabet() != SourceAlphabet::BooleanWholeGrid
            || anchor.len() != plan.view().num_vars()
        {
            return Err(invalid("source-domain anchor length"));
        }
        Ok(Self {
            plan,
            anchor: anchor.to_vec(),
            eta_zero,
            replay: FullCoefficientReplay::new(anchor.len(), 3, E::zero())?,
        })
    }
    pub fn receive_coefficients(&mut self, coefficients: &[E]) -> Result<(), AkitaError> {
        self.replay.receive_coefficients(coefficients)
    }
    pub fn bind(&mut self, challenge: E) -> Result<(), AkitaError> {
        self.replay.bind(challenge)
    }
    pub fn point(&self) -> &[E] {
        self.replay.point()
    }
    pub fn remaining_rounds(&self) -> usize {
        self.replay.remaining_rounds()
    }
    pub fn finish(self, value: E) -> Result<FullRingPendingEvaluation<E>, AkitaError> {
        self.replay.finish(self.plan.endpoint(
            &self.anchor,
            self.eta_zero,
            self.replay.point(),
            value,
        )?)?;
        Ok(FullRingPendingEvaluation {
            point: self.replay.point,
            value,
        })
    }
}

/// The fresh-source driver alone pairs this checked frontier with its bound
/// source/plan/occurrence. It is not a public cross-source token conversion.
pub(crate) struct FullRingSourceRangeRealizationVerifier<'a, E> {
    plan: &'a SourceDomainPlan,
    tau0: Vec<E>,
    virtual_claim: FullRingVirtualClaim<E>,
    eta_zero: Option<E>,
    replay: FullCoefficientReplay<E>,
}
impl<'a, E: Field> FullRingSourceRangeRealizationVerifier<'a, E> {
    pub(crate) fn new(
        plan: &'a SourceDomainPlan,
        tau0: &[E],
        virtual_claim: FullRingVirtualClaim<E>,
        eta_zero: Option<E>,
    ) -> Result<Self, AkitaError> {
        if plan.alphabet() != SourceAlphabet::BalancedBase32
            || tau0.len() != plan.view().num_vars()
            || virtual_claim.point.len() != plan.view().num_vars()
            || tau0 != virtual_claim.range_anchor()
            || eta_zero.is_some() != plan.has_physical_zeros()
        {
            return Err(invalid("source range anchor/domain/presence mismatch"));
        }
        let replay = FullCoefficientReplay::new(tau0.len(), 3, virtual_claim.value)?;
        Ok(Self {
            plan,
            tau0: tau0.to_vec(),
            virtual_claim,
            eta_zero,
            replay,
        })
    }
    pub(crate) fn receive_coefficients(&mut self, coefficients: &[E]) -> Result<(), AkitaError> {
        self.replay.receive_coefficients(coefficients)
    }
    pub(crate) fn bind(&mut self, challenge: E) -> Result<(), AkitaError> {
        self.replay.bind(challenge)
    }
    pub(crate) fn finish(self, value: E) -> Result<FullRingPendingEvaluation<E>, AkitaError> {
        self.replay.finish(self.plan.range_endpoint(
            &self.tau0,
            self.virtual_claim.point(),
            self.eta_zero,
            self.replay.point(),
            value,
        )?)?;
        Ok(FullRingPendingEvaluation {
            point: self.replay.point,
            value,
        })
    }
}

pub struct FullRingNormalizationVerifier<'p, 'a, K, E> {
    plan: &'p NormalizationPlan<'a, K, E>,
    replay: FullCoefficientReplay<E>,
}
impl<'p, 'a, K: Field, E: ExtField<K>> FullRingNormalizationVerifier<'p, 'a, K, E> {
    pub fn new(plan: &'p NormalizationPlan<'a, K, E>) -> Result<Self, AkitaError> {
        Ok(Self {
            plan,
            replay: FullCoefficientReplay::new(plan.view().num_vars(), 2, plan.target()?)?,
        })
    }
    pub fn receive_coefficients(&mut self, coefficients: &[E]) -> Result<(), AkitaError> {
        self.replay.receive_coefficients(coefficients)
    }
    pub fn bind(&mut self, challenge: E) -> Result<(), AkitaError> {
        self.replay.bind(challenge)
    }
    pub fn point(&self) -> &[E] {
        self.replay.point()
    }
    pub fn remaining_rounds(&self) -> usize {
        self.replay.remaining_rounds()
    }
    pub fn finish(self, value: E) -> Result<FullRingPendingEvaluation<E>, AkitaError> {
        self.replay
            .finish(self.plan.endpoint(self.replay.point(), value)?)?;
        Ok(FullRingPendingEvaluation {
            point: self.replay.point,
            value,
        })
    }
}

#[cfg(test)]
#[path = "full_ring_tests.rs"]
mod tests;

/// Final inner receiver's domain-only endpoint: native range plus exact
/// predecessor F/H binarity and zeros, with no coefficient relation or rho.
/// Its returned endpoint must still be normalized with the original pending
/// ordinary claim and authenticated by the actual shrinking continuation.
pub struct FullRingFinalReceiverDomainVerifier<'a, E> {
    domain: &'a akita_types::full_ring::FinalReceiverDomainPlan,
    tau0: Vec<E>,
    virtual_claim: FullRingVirtualClaim<E>,
    mix: StrongMix<E>,
    replay: FullCoefficientReplay<E>,
}
impl<'a, E: Field> FullRingFinalReceiverDomainVerifier<'a, E> {
    pub fn new(
        domain: &'a akita_types::full_ring::FinalReceiverDomainPlan,
        tau0: &[E],
        virtual_claim: FullRingVirtualClaim<E>,
        mix: StrongMix<E>,
    ) -> Result<Self, AkitaError> {
        if tau0.len() != domain.num_vars()
            || virtual_claim.point.len() != domain.num_vars()
            || tau0 != virtual_claim.range_anchor()
        {
            return Err(invalid("final receiver range anchor/domain mismatch"));
        }
        let replay = FullCoefficientReplay::new(
            domain.num_vars(),
            FullRingRangePlan::FUSED_DEGREE,
            mix.gamma * virtual_claim.value,
        )?;
        Ok(Self {
            domain,
            tau0: tau0.to_vec(),
            virtual_claim,
            mix,
            replay,
        })
    }
    pub fn receive_coefficients(&mut self, coefficients: &[E]) -> Result<(), AkitaError> {
        self.replay.receive_coefficients(coefficients)
    }
    pub fn bind(&mut self, challenge: E) -> Result<(), AkitaError> {
        self.replay.bind(challenge)
    }
    pub fn point(&self) -> &[E] {
        self.replay.point()
    }
    pub fn remaining_rounds(&self) -> usize {
        self.replay.remaining_rounds()
    }
    pub fn finish(self, value: E) -> Result<FullRingPendingEvaluation<E>, AkitaError> {
        let expected = akita_types::full_ring::final_receiver_domain_endpoint(
            self.domain,
            &self.tau0,
            &self.virtual_claim.point,
            self.mix,
            self.replay.point(),
            value,
        )?;
        self.replay.finish(expected)?;
        Ok(FullRingPendingEvaluation {
            point: self.replay.point,
            value,
        })
    }
}
