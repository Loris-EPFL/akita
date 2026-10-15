//! Live selected reductions over the original inner/outer compiler. All
//! parser, registry, source-shape and algebra failures poison that compiler.
//! No routine here accepts a proof: endpoints remain pending source openings.
use crate::full_ring::FullRingFinalReceiverDomainVerifier;
use crate::full_ring::{
    FullRingFusedVerifier, FullRingNormalizationVerifier, FullRingPendingEvaluation,
    FullRingRangeVerifier, FullRingSourceDomainVerifier, FullRingSourceRangeRealizationVerifier,
};
use akita_error::AkitaError;
use akita_transcript::native::{E, K};
use akita_transcript::native_strong::FinalReceiverDomainSites;
use akita_transcript::native_strong::{
    bind_normalization_claims, guarded, receive_values, NativeStrongTranscript, NormalizationSites,
    Operation, SourceDomainSites, StrongSites,
};
use akita_types::full_ring::FinalReceiverDomainPlan;
use akita_types::full_ring::{
    CoefficientRelationPlan, NormalizationClaim, NormalizationPlan, RawSourceViewPlan,
    SourceAlphabet, SourceDomainPlan, StrongMix,
};
use jolt_field::Zero;

/// The binding bytes must be verifier-derived actual W A images, source plan
/// and complete coefficient-row manifest. The registry checks their installed
/// keys/codecs; they must never be accepted as proof-selected public metadata.
pub fn verify_full_ring_strong<T, R, D>(
    transcript: &mut T,
    sites: &StrongSites<R, D>,
    binding: &[u8],
    relation: &CoefficientRelationPlan<'_, K, E>,
    proof: &mut &[u8],
) -> Result<FullRingPendingEvaluation<E>, AkitaError>
where
    T: NativeStrongTranscript<R, D>,
{
    guarded(transcript, |transcript| {
        let rounds = relation.num_vars();
        if sites.first.len() != rounds
            || sites.second.len() != rounds
            || sites.fused.len() != rounds
        {
            return Err(AkitaError::InvalidProof);
        }
        transcript.public(&sites.binding, 1, binding)?;
        let tau0 = transcript.vector(&sites.tau0, Operation::RawVector, rounds)?;
        let rho = transcript.vector(
            &sites.rho,
            Operation::CoefficientVector,
            relation.row_num_vars(),
        )?;
        let mut range = FullRingRangeVerifier::new(&tau0)?;
        let mut coefficients = [E::zero(); 6];
        for round in &sites.first {
            receive_values(transcript, &round.coefficients, &mut coefficients, proof)?;
            range.receive_coefficients(&coefficients)?;
            range.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
        }
        let mut quarters = [E::zero(); 4];
        receive_values(transcript, &sites.quarters, &mut quarters, proof)?;
        range.receive_quarters(quarters)?;
        let xi: [E; 3] = transcript
            .vector(&sites.xi, Operation::CoefficientVector, 3)?
            .try_into()
            .map_err(|_| AkitaError::InvalidProof)?;
        range.begin_leaf_stage(xi)?;
        for round in &sites.second {
            receive_values(transcript, &round.coefficients, &mut coefficients, proof)?;
            range.receive_coefficients(&coefficients)?;
            range.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
        }
        let mut value = [E::zero()];
        receive_values(transcript, &sites.virtual_value, &mut value, proof)?;
        let virtual_claim = range.receive_virtual(value[0])?;
        let mix: [E; 3] = transcript
            .vector(&sites.mix, Operation::CoefficientVector, 3)?
            .try_into()
            .map_err(|_| AkitaError::InvalidProof)?;
        let mut fused =
            FullRingFusedVerifier::new(relation, &rho, &tau0, virtual_claim, StrongMix::from(mix))?;
        let mut coefficients = [E::zero(); 4];
        for round in &sites.fused {
            receive_values(transcript, &round.coefficients, &mut coefficients, proof)?;
            fused.receive_coefficients(&coefficients)?;
            fused.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
        }
        receive_values(transcript, &sites.value, &mut value, proof)?;
        // Includes the actual public coefficient-matrix weight scan and every
        // fused term at this SAME v. No inversion or deferred setup claim.
        fused.finish(value[0])
    })
}

/// Authenticate the source predicate and required physical zero support. Its
/// pending raw endpoint must be included in this handle's normalization list.
pub fn verify_full_ring_source_domain<T, R, D>(
    transcript: &mut T,
    sites: &SourceDomainSites<R, D>,
    binding: &[u8],
    plan: &SourceDomainPlan,
    proof: &mut &[u8],
) -> Result<FullRingPendingEvaluation<E>, AkitaError>
where
    T: NativeStrongTranscript<R, D>,
{
    guarded(transcript, |transcript| {
        let rounds = plan.view().num_vars();
        sites.validate(
            rounds,
            plan.alphabet() == SourceAlphabet::BalancedBase32,
            plan.has_physical_zeros(),
        )?;
        match sites {
            SourceDomainSites::Boolean(sites) => {
                transcript.public(&sites.binding, 1, binding)?;
                let anchor = transcript.vector(&sites.anchor, Operation::RawVector, rounds)?;
                let eta_zero = transcript.scalar(&sites.eta_zero, Operation::ScalarCoefficient)?;
                let mut replay = FullRingSourceDomainVerifier::new(plan, &anchor, eta_zero)?;
                let mut coefficients = [E::zero(); 4];
                for round in &sites.rounds {
                    receive_values(transcript, &round.coefficients, &mut coefficients, proof)?;
                    replay.receive_coefficients(&coefficients)?;
                    replay.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
                }
                let mut value = [E::zero()];
                receive_values(transcript, &sites.value, &mut value, proof)?;
                replay.finish(value[0])
            }
            SourceDomainSites::Balanced(sites) => {
                transcript.public(&sites.binding, 1, binding)?;
                let tau0 = transcript.vector(&sites.tau0, Operation::RawVector, rounds)?;
                let mut range = FullRingRangeVerifier::new(&tau0)?;
                let mut coefficients = [E::zero(); 6];
                for round in &sites.first {
                    receive_values(transcript, &round.coefficients, &mut coefficients, proof)?;
                    range.receive_coefficients(&coefficients)?;
                    range.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
                }
                let mut quarters = [E::zero(); 4];
                receive_values(transcript, &sites.quarters, &mut quarters, proof)?;
                range.receive_quarters(quarters)?;
                let xi: [E; 3] = transcript
                    .vector(&sites.xi, Operation::CoefficientVector, 3)?
                    .try_into()
                    .map_err(|_| AkitaError::InvalidProof)?;
                range.begin_leaf_stage(xi)?;
                for round in &sites.second {
                    receive_values(transcript, &round.coefficients, &mut coefficients, proof)?;
                    range.receive_coefficients(&coefficients)?;
                    range.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
                }
                let mut value = [E::zero()];
                receive_values(transcript, &sites.virtual_value, &mut value, proof)?;
                let virtual_claim = range.receive_virtual(value[0])?;
                drop(range);
                let eta_zero = sites
                    .eta_zero
                    .as_ref()
                    .map(|site| transcript.scalar(site, Operation::ScalarCoefficient))
                    .transpose()?;
                let mut replay = FullRingSourceRangeRealizationVerifier::new(
                    plan,
                    &tau0,
                    virtual_claim,
                    eta_zero,
                )?;
                let mut coefficients = [E::zero(); 4];
                for round in &sites.realization {
                    receive_values(transcript, &round.coefficients, &mut coefficients, proof)?;
                    replay.receive_coefficients(&coefficients)?;
                    replay.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
                }
                receive_values(transcript, &sites.value, &mut value, proof)?;
                replay.finish(value[0])
            }
        }
    })
}

/// Bind all ordered ordinary/raw claims before ONE independent alpha vector.
/// Decoder descriptors/permutations/affine constants are authenticated in the
/// binding header; actual claim points and values are additionally bound here.
pub fn verify_full_ring_normalization<'a, T, R, D>(
    transcript: &mut T,
    sites: &NormalizationSites<R, D>,
    binding: &[u8],
    view: &'a RawSourceViewPlan,
    claims: Vec<NormalizationClaim<'a, K, E>>,
    proof: &mut &[u8],
) -> Result<FullRingPendingEvaluation<E>, AkitaError>
where
    T: NativeStrongTranscript<R, D>,
{
    guarded(transcript, |transcript| {
        if sites.rounds.len() != view.num_vars() || claims.is_empty() {
            return Err(AkitaError::InvalidProof);
        }
        transcript.public(&sites.binding, 1, binding)?;
        bind_normalization_claims(
            transcript,
            &sites.claims,
            claims.iter().map(|claim| match claim {
                NormalizationClaim::Ordinary { point, value, .. } => (false, *point, *value),
                NormalizationClaim::Raw { point, value } => (true, *point, *value),
            }),
        )?;
        let alpha = transcript.vector(&sites.alpha, Operation::CoefficientVector, claims.len())?;
        let plan = NormalizationPlan::new(view, claims, alpha)?;
        let mut replay = FullRingNormalizationVerifier::new(&plan)?;
        let mut coefficients = [E::zero(); 3];
        for round in &sites.rounds {
            receive_values(transcript, &round.coefficients, &mut coefficients, proof)?;
            replay.receive_coefficients(&coefficients)?;
            replay.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
        }
        let mut value = [E::zero()];
        receive_values(transcript, &sites.value, &mut value, proof)?;
        // The endpoint remains v*A(r), including A(r)=0; never divide by A.
        replay.finish(value[0])
    })
}

/// Final receiver alphabet/binary/required-zero replay under its original
/// identity commitment. No rho, no coefficient matrix and no linear term are
/// introduced. The pending endpoint must be normalized with the incoming one.
pub fn verify_full_ring_final_receiver_domain<T, R, D>(
    transcript: &mut T,
    sites: &FinalReceiverDomainSites<R, D>,
    binding: &[u8],
    domain: &FinalReceiverDomainPlan,
    proof: &mut &[u8],
) -> Result<FullRingPendingEvaluation<E>, AkitaError>
where
    T: NativeStrongTranscript<R, D>,
{
    guarded(transcript, |transcript| {
        let rounds = domain.num_vars();
        if sites.first.len() != rounds
            || sites.second.len() != rounds
            || sites.fused.len() != rounds
        {
            return Err(AkitaError::InvalidProof);
        }
        transcript.public(&sites.binding, 1, binding)?;
        let tau0 = transcript.vector(&sites.tau0, Operation::RawVector, rounds)?;
        let mut range = FullRingRangeVerifier::new(&tau0)?;
        let mut coefficients = [E::zero(); 6];
        for round in &sites.first {
            receive_values(transcript, &round.coefficients, &mut coefficients, proof)?;
            range.receive_coefficients(&coefficients)?;
            range.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
        }
        let mut quarters = [E::zero(); 4];
        receive_values(transcript, &sites.quarters, &mut quarters, proof)?;
        range.receive_quarters(quarters)?;
        let xi: [E; 3] = transcript
            .vector(&sites.xi, Operation::CoefficientVector, 3)?
            .try_into()
            .map_err(|_| AkitaError::InvalidProof)?;
        range.begin_leaf_stage(xi)?;
        for round in &sites.second {
            receive_values(transcript, &round.coefficients, &mut coefficients, proof)?;
            range.receive_coefficients(&coefficients)?;
            range.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
        }
        let mut value = [E::zero()];
        receive_values(transcript, &sites.virtual_value, &mut value, proof)?;
        let virtual_claim = range.receive_virtual(value[0])?;
        let mix: [E; 3] = transcript
            .vector(&sites.mix, Operation::CoefficientVector, 3)?
            .try_into()
            .map_err(|_| AkitaError::InvalidProof)?;
        let mut fused = FullRingFinalReceiverDomainVerifier::new(
            domain,
            &tau0,
            virtual_claim,
            StrongMix::from(mix),
        )?;
        let mut coefficients = [E::zero(); 4];
        for round in &sites.fused {
            receive_values(transcript, &round.coefficients, &mut coefficients, proof)?;
            fused.receive_coefficients(&coefficients)?;
            fused.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
        }
        receive_values(transcript, &sites.value, &mut value, proof)?;
        fused.finish(value[0])
    })
}
