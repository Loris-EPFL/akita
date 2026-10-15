//! Live D1 reductions on borrowed owner-bound K sources. These functions never
//! complete a proof scope: the returned (point,value) remains a source-opening
//! obligation owned by the root/continuation. Installation headers are supplied
//! from authenticated setup/source/row metadata, not from a proof's assertions.
use akita_error::AkitaError;
use akita_transcript::native::{E, K};
use akita_transcript::native_strong::FinalReceiverDomainSites;
use akita_transcript::native_strong::{
    bind_normalization_claims, guarded, send_values, NativeStrongTranscript, NormalizationSites,
    Operation, SourceDomainSites, StrongSites,
};
use akita_types::full_ring::{
    CoefficientRelationPlan, FullRingCoefficientSource, FusedRelationSession, NormalizationClaim,
    NormalizationPlan, NormalizationSession, ProductRangeSession, RawSourceReader,
    RawSourceViewPlan, SourceAlphabet, SourceDomainPlan, SourceDomainSession,
    SourceRangeRealizationSession, StrongMix,
};
use akita_types::full_ring::{FinalReceiverDomainPlan, FinalReceiverDomainSession};

/// Bind W's actual A-image/source/row-manifest header before tau0 then rho.
/// Both degree-five stages and the fused degree-three check use the original
/// K reader. Every coefficient, quarter and final scalar is canonical and sent.
pub fn prove_full_ring_strong<T, R, D>(
    transcript: &mut T,
    sites: &StrongSites<R, D>,
    binding: &[u8],
    source: &dyn FullRingCoefficientSource<K>,
    relation: &CoefficientRelationPlan<'_, K, E>,
    proof: &mut Vec<u8>,
) -> Result<(Vec<E>, E), AkitaError>
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
        let mut range = ProductRangeSession::new(source, &tau0)?;
        for round in &sites.first {
            send_values(
                transcript,
                &round.coefficients,
                &range.round_coefficients()?,
                proof,
            )?;
            range.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
        }
        send_values(transcript, &sites.quarters, &range.quarter_claims()?, proof)?;
        let xi: [E; 3] = transcript
            .vector(&sites.xi, Operation::CoefficientVector, 3)?
            .try_into()
            .map_err(|_| AkitaError::InvalidProof)?;
        range.begin_leaf_stage(xi)?;
        for round in &sites.second {
            send_values(
                transcript,
                &round.coefficients,
                &range.round_coefficients()?,
                proof,
            )?;
            range.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
        }
        let svirt = range.virtual_claim()?;
        send_values(transcript, &sites.virtual_value, &[svirt], proof)?;
        let mix: [E; 3] = transcript
            .vector(&sites.mix, Operation::CoefficientVector, 3)?
            .try_into()
            .map_err(|_| AkitaError::InvalidProof)?;
        let mut fused = FusedRelationSession::new(
            source,
            relation,
            &rho,
            &tau0,
            range.point(),
            svirt,
            StrongMix::from(mix),
        )?;
        for round in &sites.fused {
            send_values(
                transcript,
                &round.coefficients,
                &fused.round_coefficients()?,
                proof,
            )?;
            fused.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
        }
        let value = fused.witness_value()?;
        send_values(transcript, &sites.value, &[value], proof)?;
        Ok((fused.point().to_vec(), value))
    })
}

/// Typed positive-Boolean or balanced (5,5,3) source prefix. The returned value
/// is the original raw W evaluation, never the range tree's virtual U value.
pub fn prove_full_ring_source_domain<T, R, D>(
    transcript: &mut T,
    sites: &SourceDomainSites<R, D>,
    binding: &[u8],
    source: &dyn FullRingCoefficientSource<K>,
    plan: &SourceDomainPlan,
    proof: &mut Vec<u8>,
) -> Result<(Vec<E>, E), AkitaError>
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
        if source.len() != plan.view().physical_len() {
            return Err(AkitaError::InvalidProof);
        }
        match sites {
            SourceDomainSites::Boolean(sites) => {
                transcript.public(&sites.binding, 1, binding)?;
                let anchor = transcript.vector(&sites.anchor, Operation::RawVector, rounds)?;
                let eta_zero = transcript.scalar(&sites.eta_zero, Operation::ScalarCoefficient)?;
                let mut session = SourceDomainSession::new(source, plan, &anchor, eta_zero)?;
                for round in &sites.rounds {
                    send_values(
                        transcript,
                        &round.coefficients,
                        session.round_coefficients()?,
                        proof,
                    )?;
                    session.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
                }
                let value = session.raw_value()?;
                send_values(transcript, &sites.value, &[value], proof)?;
                Ok((session.point().to_vec(), value))
            }
            SourceDomainSites::Balanced(sites) => {
                transcript.public(&sites.binding, 1, binding)?;
                let tau0 = transcript.vector(&sites.tau0, Operation::RawVector, rounds)?;
                let (rvirt, svirt) = {
                    let raw = RawSourceReader::new(source, plan.view())?;
                    let mut range = ProductRangeSession::new(&raw, &tau0)?;
                    for round in &sites.first {
                        send_values(
                            transcript,
                            &round.coefficients,
                            &range.round_coefficients()?,
                            proof,
                        )?;
                        range.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
                    }
                    send_values(transcript, &sites.quarters, &range.quarter_claims()?, proof)?;
                    let xi: [E; 3] = transcript
                        .vector(&sites.xi, Operation::CoefficientVector, 3)?
                        .try_into()
                        .map_err(|_| AkitaError::InvalidProof)?;
                    range.begin_leaf_stage(xi)?;
                    for round in &sites.second {
                        send_values(
                            transcript,
                            &round.coefficients,
                            &range.round_coefficients()?,
                            proof,
                        )?;
                        range.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
                    }
                    let svirt = range.virtual_claim()?;
                    send_values(transcript, &sites.virtual_value, &[svirt], proof)?;
                    (range.point().to_vec(), svirt)
                }; // Release range work-table capacities before realization.
                let eta_zero = sites
                    .eta_zero
                    .as_ref()
                    .map(|site| transcript.scalar(site, Operation::ScalarCoefficient))
                    .transpose()?;
                let mut session = SourceRangeRealizationSession::new(
                    source, plan, &tau0, &rvirt, svirt, eta_zero,
                )?;
                for round in &sites.realization {
                    send_values(
                        transcript,
                        &round.coefficients,
                        session.round_coefficients()?,
                        proof,
                    )?;
                    session.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
                }
                let value = session.raw_value()?;
                send_values(transcript, &sites.value, &[value], proof)?;
                Ok((session.point().to_vec(), value))
            }
        }
    })
}

/// Universal decoder-adjoint normalization, not the optional positional shortcut.
/// The ordered list MUST include every ordinary alias and the preceding source
/// domain endpoint. The binding header identifies their original source and
/// exact installed decoders. This driver binds the actual points/values itself.
pub fn prove_full_ring_normalization<'a, T, R, D>(
    transcript: &mut T,
    sites: &NormalizationSites<R, D>,
    binding: &[u8],
    source: &dyn FullRingCoefficientSource<K>,
    view: &'a RawSourceViewPlan,
    claims: Vec<NormalizationClaim<'a, K, E>>,
    proof: &mut Vec<u8>,
) -> Result<(Vec<E>, E), AkitaError>
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
        let mut session = NormalizationSession::new(source, &plan)?;
        for round in &sites.rounds {
            send_values(
                transcript,
                &round.coefficients,
                &session.round_coefficients()?,
                proof,
            )?;
            session.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
        }
        let value = session.raw_value()?;
        send_values(transcript, &sites.value, &[value], proof)?;
        Ok((session.point().to_vec(), value))
    })
}

/// Final inner receiver domain check on the unchanged identity source. This is
/// not an outgoing coefficient-relation check: it draws no rho and includes no
/// fabricated linear relation. Its endpoint must join the existing incoming
/// opening in the subsequent same-source normalization.
pub fn prove_full_ring_final_receiver_domain<T, R, D>(
    transcript: &mut T,
    sites: &FinalReceiverDomainSites<R, D>,
    binding: &[u8],
    source: &dyn FullRingCoefficientSource<K>,
    domain: &FinalReceiverDomainPlan,
    proof: &mut Vec<u8>,
) -> Result<(Vec<E>, E), AkitaError>
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
        let mut range = ProductRangeSession::new(source, &tau0)?;
        for round in &sites.first {
            send_values(
                transcript,
                &round.coefficients,
                &range.round_coefficients()?,
                proof,
            )?;
            range.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
        }
        send_values(transcript, &sites.quarters, &range.quarter_claims()?, proof)?;
        let xi: [E; 3] = transcript
            .vector(&sites.xi, Operation::CoefficientVector, 3)?
            .try_into()
            .map_err(|_| AkitaError::InvalidProof)?;
        range.begin_leaf_stage(xi)?;
        for round in &sites.second {
            send_values(
                transcript,
                &round.coefficients,
                &range.round_coefficients()?,
                proof,
            )?;
            range.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
        }
        let svirt = range.virtual_claim()?;
        send_values(transcript, &sites.virtual_value, &[svirt], proof)?;
        let mix: [E; 3] = transcript
            .vector(&sites.mix, Operation::CoefficientVector, 3)?
            .try_into()
            .map_err(|_| AkitaError::InvalidProof)?;
        let mut fused = FinalReceiverDomainSession::new(
            source,
            domain,
            &tau0,
            range.point(),
            svirt,
            StrongMix::from(mix),
        )?;
        for round in &sites.fused {
            send_values(
                transcript,
                &round.coefficients,
                &fused.round_coefficients()?,
                proof,
            )?;
            fused.bind(transcript.scalar(&round.challenge, Operation::RawScalar)?)?;
        }
        let value = fused.witness_value()?;
        send_values(transcript, &sites.value, &[value], proof)?;
        Ok((fused.point().to_vec(), value))
    })
}
