//! Selected root/continuation replay over original installed K matrices and the
//! same live D1 compiler. Ordinary callers supply checked prefix raw claims.
//! Opt-in accumulation owns repeated receiver prefixes and fresh-source
//! normalization; application adjacency remains the checked frontend's duty.
use akita_config::{CommitmentConfig, PlannerOpeningFamily, TrustedScheduleCatalog};
use akita_error::AkitaError;
use akita_params::{
    CommittedGroupParams, FoldSchedule, NextWitnessBindingPolicy, OpeningClaimsLayout,
    RelationWitnessGeometry,
};
use akita_transcript::native::{E, K, Q};
use akita_transcript::native_strong::{
    guarded, NativeCompilerFamily, NativeStrongTranscript, NativeTranscriptBorrow, TerminalSites,
};
use akita_types::full_ring::{
    FinalReceiverDomainPlan, FullRingBuildContext, FullRingRawPoint, FullRingWitnessContext,
    RawSourceViewPlan,
};
use akita_types::full_ring_accumulation::{
    full_ring_accumulation_binding, full_ring_fresh_obligation_binding, FullRingAccumulationPlan,
    FullRingApplicationClaim, FullRingFreshObligations, FullRingUpdateCount, FullRingUpdateSites,
};
use akita_types::full_ring_root::{
    full_ring_raw_claim_binding, full_ring_strong_binding, FullRingRawClaim, FullRingReductionSites,
};
use akita_types::{
    FullRingSetupFamily, FullRingSuccessor, GroupBatchStatement, InstalledFullRingSetup, RingVec,
};
use jolt_field::{CanonicalBytes, CanonicalEncoding, Ring, Zero};
use std::{marker::PhantomData, sync::Arc};

pub enum FullRingPendingBinding {
    OuterPayload(RingVec<K>),
    TerminalInnerState(RingVec<K>),
}

/// Owns the compiler, exact admitted schedule, remaining reader and pending
/// obligations. There is no unchecked constructor or conversion to acceptance.
pub struct PendingFullRingVerifier<'p, T, R, D>
where
    T: NativeStrongTranscript<R, D>,
{
    setup: Arc<InstalledFullRingSetup<K>>,
    schedule: FoldSchedule,
    transcript: T,
    proof: &'p [u8],
    binding: FullRingPendingBinding,
    witness_len: usize,
    point: Vec<E>,
    value: E,
    domain: FinalReceiverDomainPlan,
    next_recursive: usize,
    final_normalized: bool,
    accumulation: Option<AccumulationState>,
    marker: PhantomData<fn() -> (R, D)>,
}

struct AccumulationState {
    plan: Arc<FullRingAccumulationPlan>,
    count: FullRingUpdateCount,
}

fn invalid(message: &str) -> AkitaError {
    AkitaError::InvalidInput(message.into())
}
fn take<'p>(proof: &mut &'p [u8], count: usize) -> Result<&'p [u8], AkitaError> {
    let bytes = proof.get(..count).ok_or(AkitaError::InvalidProof)?;
    *proof = proof.get(count..).ok_or(AkitaError::InvalidProof)?;
    Ok(bytes)
}
fn receive_base<T, R, D>(
    transcript: &mut T,
    site: &R,
    count: usize,
    proof: &mut &[u8],
) -> Result<RingVec<K>, AkitaError>
where
    T: NativeStrongTranscript<R, D>,
{
    let len = count.checked_mul(16).ok_or(AkitaError::InvalidProof)?;
    let bytes = take(proof, len)?;
    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| AkitaError::InvalidProof)?;
    for atom in bytes.chunks_exact(16) {
        values.push(K::from_bytes_le_checked(atom).ok_or(AkitaError::InvalidProof)?);
    }
    transcript.proof(site, count, bytes)?;
    Ok(RingVec::from_coeffs(values))
}
fn base_bytes(values: &[K]) -> Result<Vec<u8>, AkitaError> {
    let len = values
        .len()
        .checked_mul(16)
        .ok_or(AkitaError::InvalidProof)?;
    let mut bytes = vec![0; len];
    for (value, atom) in values.iter().zip(bytes.chunks_exact_mut(16)) {
        value.to_bytes_le(atom);
    }
    Ok(bytes)
}

/// Begin after the actual source-prefix and universal normalization replay on
/// this transcript. Raw claims come from that ledger; source and per-level
/// matrix roles come only from the original installed setup descriptor.
#[allow(clippy::too_many_arguments)]
pub fn begin_full_ring_verification<'p, Cfg, T, R, D>(
    setup: Arc<InstalledFullRingSetup<K>>,
    schedules: &TrustedScheduleCatalog<Cfg>,
    statement: GroupBatchStatement<'_, E, K>,
    raw_claims: Vec<FullRingRawClaim>,
    mut transcript: T,
    mut proof: &'p [u8],
    sites: &FullRingReductionSites<R, D, T::ShellSite>,
) -> Result<PendingFullRingVerifier<'p, T, R, D>, AkitaError>
where
    Cfg: CommitmentConfig<Field = K, ExtField = E>,
    T: NativeStrongTranscript<R, D>,
{
    let (schedule, output) = guarded(&mut transcript, |transcript| {
        akita_config::validate_config_policy::<Cfg>()?;
        if !matches!(
            (
                Cfg::opening_family(),
                setup.descriptor().family(),
                T::FAMILY
            ),
            (
                PlannerOpeningFamily::FullRingE2OuterDirect,
                FullRingSetupFamily::OuterDirect,
                NativeCompilerFamily::OuterShake256
            ) | (
                PlannerOpeningFamily::FullRingE2InnerRecursive,
                FullRingSetupFamily::Registered,
                NativeCompilerFamily::InnerH1610
            )
        ) {
            return Err(invalid(
                "full-ring policy, setup and compiler family mismatch",
            ));
        }
        let resolved = schedules.resolve_selection(statement.selection())?;
        let claims = statement.into_claims();
        let layout = claims.committed_layout()?;
        resolved.validate_opening_layout(&layout)?;
        if layout.views().iter().any(|view| *view != akita_params::SourceOpeningView::Decoded) {
            return Err(AkitaError::InvalidProof);
        }
        let schedule = resolved.schedule();
        schedule.validate_structure()?;
        if setup.descriptor().level(0)?.sources.len() != layout.num_groups()
            || setup.descriptor().levels().len() != schedule.recursive_folds.len() + 1
            || raw_claims.len() != layout.num_groups()
        {
            return Err(invalid("installed source role map length"));
        }
        let mut public = Vec::with_capacity(layout.num_groups());
        for (index, group) in claims.groups().iter().enumerate() {
            let expected = schedule.root.params.group_params(&layout, index)?;
            if group.commitment().profile() != &expected.profile {
                return Err(invalid(
                    "original commitment profile differs from admitted root",
                ));
            }
            group
                .commitment()
                .profile()
                .validate_frozen_precommit(128)?;
            public.push(group.commitment().rows().clone());
        }
        let instance =
            akita_types::full_ring_root::full_ring_instance_binding(setup.descriptor(), schedule)?;
        transcript.public(&sites.instance, 1, &instance)?;
        let output = replay_reduction(
            &setup,
            transcript,
            &mut proof,
            &schedule.root.params,
            &layout,
            public,
            raw_claims,
            schedule.root.output_witness_len,
            schedule,
            0,
            sites,
        )?;
        Ok((schedule.clone(), output))
    })?;
    let final_normalized = schedule.recursive_folds.is_empty();
    Ok(PendingFullRingVerifier {
        setup,
        schedule,
        transcript,
        proof,
        binding: output.binding,
        witness_len: output.witness_len,
        point: output.point,
        value: output.value,
        domain: output.domain,
        next_recursive: 0,
        final_normalized,
        accumulation: None,
        marker: PhantomData,
    })
}

/// Start an exact bounded compact run and replay its first root. The compiler
/// is bound to T and installation before invoking the checked application.
/// No receiver is accepted as input: the first one is the registered zero.
///
/// `frontend` must check VM adjacency, original public RAM continuity and row
/// limits, and return concrete fresh commitments/endpoints. Native replay
/// checks their installed profiles, source domains and normalization itself.
#[allow(clippy::too_many_arguments)]
pub fn begin_full_ring_accumulation_verification<'p, Cfg, T, R, D, X>(
    setup: Arc<InstalledFullRingSetup<K>>,
    schedules: &TrustedScheduleCatalog<Cfg>,
    selection: akita_params::OpeningScheduleSelection,
    plan: Arc<FullRingAccumulationPlan>,
    total: usize,
    mut transcript: T,
    proof: &'p [u8],
    sites: &FullRingUpdateSites<R, D, T::ShellSite>,
    frontend: impl FnOnce(
        NativeTranscriptBorrow<'_, T, R, D>,
        &mut &'p [u8],
        usize,
        &RingVec<K>,
    ) -> Result<FullRingFreshObligations, X>,
) -> Result<PendingFullRingVerifier<'p, T, R, D>, X>
where
    Cfg: CommitmentConfig<Field = K, ExtField = E>,
    T: NativeStrongTranscript<R, D>,
    X: From<AkitaError>,
{
    let (schedule, count, domain) = guarded(&mut transcript, |transcript| {
        akita_config::validate_config_policy::<Cfg>()?;
        if Cfg::opening_family() != PlannerOpeningFamily::FullRingE2InnerRecursive
            || T::FAMILY != NativeCompilerFamily::InnerH1610
        {
            return Err(invalid(
                "accumulation requires the original inner compiler and policy",
            ));
        }
        let resolved = schedules.resolve_selection(selection)?;
        resolved.validate_opening_layout(plan.layout())?;
        let schedule = resolved.schedule();
        plan.validate(setup.descriptor(), schedule)?;
        let count = FullRingUpdateCount::new(total)?;
        let view = plan.receiver_view()?.clone();
        let len = view.physical_len();
        // No proof claim establishes the initial zero: its entire source and
        // original canonical commitment are fixed by installation.
        let domain = FinalReceiverDomainPlan::new(
            view,
            akita_types::full_ring::CoefficientSupport::new(len, Vec::new())?,
            akita_types::full_ring::CoefficientSupport::new(
                len,
                std::iter::once(0..len).collect(),
            )?,
        )?;
        transcript.public(
            &sites.reduction.instance,
            1,
            &full_ring_accumulation_binding(&plan, total)?,
        )?;
        Ok((schedule.clone(), count, domain))
    })?;
    let pending = PendingFullRingVerifier {
        setup,
        schedule,
        transcript,
        proof,
        binding: FullRingPendingBinding::OuterPayload(plan.zero_receiver().rows().clone()),
        witness_len: domain.domain_len(),
        point: vec![E::zero(); domain.num_vars()],
        value: E::zero(),
        domain,
        next_recursive: 0,
        final_normalized: false,
        accumulation: Some(AccumulationState { plan, count }),
        marker: PhantomData,
    };
    pending.update_with_frontend(sites, frontend)
}

struct ReductionOutput {
    binding: FullRingPendingBinding,
    witness_len: usize,
    point: Vec<E>,
    value: E,
    domain: FinalReceiverDomainPlan,
}

fn successor_view(
    schedule: &FoldSchedule,
    index: usize,
    witness_len: usize,
) -> Result<RawSourceViewPlan, AkitaError> {
    let (blocks, positions) = if let Some(next) = schedule.recursive_folds.get(index) {
        let layout = OpeningClaimsLayout::new(witness_len.trailing_zeros() as usize, 1)?;
        let group = next.params.group_params_geometry(&layout, 0)?;
        (group.num_live_blocks(), group.num_positions_per_block())
    } else {
        (
            schedule.terminal.blocks.live_blocks,
            schedule.terminal.blocks.positions_per_block,
        )
    };
    let view = RawSourceViewPlan::new(128, 1, blocks, positions, 1)?;
    if view.physical_len() != witness_len {
        return Err(invalid("successor identity view length"));
    }
    Ok(view)
}

#[allow(clippy::too_many_arguments)]
fn replay_reduction<T, R, D>(
    setup: &InstalledFullRingSetup<K>,
    transcript: &mut T,
    proof: &mut &[u8],
    params: &CommittedGroupParams,
    layout: &OpeningClaimsLayout,
    public: Vec<RingVec<K>>,
    raw_claims: Vec<FullRingRawClaim>,
    witness_len: usize,
    schedule: &FoldSchedule,
    successor_index: usize,
    sites: &FullRingReductionSites<R, D, T::ShellSite>,
) -> Result<ReductionOutput, AkitaError>
where
    T: NativeStrongTranscript<R, D>,
{
    params.validate_full_ring_execution(2)?;
    params.validate_opening_batch(layout)?;
    let sources = &setup.descriptor().level(successor_index)?.sources;
    if !witness_len.is_power_of_two()
        || raw_claims.len() != layout.num_groups()
        || public.len() != raw_claims.len()
        || sources.len() != raw_claims.len()
    {
        return Err(invalid("selected root replay geometry"));
    }
    // Installed source roles follow native physical execution order; raw
    // endpoint tuples remain indexed by their original public group index.
    let mut public_sources = vec![None; sources.len()];
    for (index, &source) in layout.root_group_order()?.into_iter().zip(sources) {
        let slot = public_sources
            .get_mut(index)
            .ok_or(AkitaError::InvalidProof)?;
        if slot.replace(source).is_some() {
            return Err(AkitaError::InvalidProof);
        }
    }
    let public_sources = public_sources
        .into_iter()
        .map(|source| source.ok_or(AkitaError::InvalidProof))
        .collect::<Result<Vec<_>, _>>()?;
    let sources = &public_sources;
    let geometry = RelationWitnessGeometry::for_level(params, layout, 2)?;
    let rhs = geometry.rhs_layout();
    let mut count = 0usize;
    for (index, (claim, &source_id)) in raw_claims.iter().zip(sources).enumerate() {
        let group = params.group_params_geometry(layout, index)?;
        let source = setup.descriptor().source(source_id)?;
        let view = claim.point.view();
        if view != &source.raw_view
            || group.a_rows_len() != source.rank
            || view.polynomials() != layout.group_layout(index)?.num_polynomials()
            || view.blocks() != group.num_live_blocks()
            || view.positions() != group.num_positions_per_block()
            || view.digits() != group.num_digits_inner()
            || public
                .get(index)
                .ok_or(AkitaError::InvalidProof)?
                .coeff_len()
                != rhs
                    .compression_plan_for_group(index)?
                    .terminal_coefficients()
        {
            return Err(invalid("original source/profile role mapping mismatch"));
        }
        count = count
            .checked_add(
                view.polynomials()
                    .checked_mul(view.blocks())
                    .ok_or(AkitaError::InvalidProof)?,
            )
            .ok_or(AkitaError::InvalidProof)?;
    }
    transcript.public(
        &sites.raw_queries,
        raw_claims.len(),
        &full_ring_raw_claim_binding(&raw_claims, sources)?,
    )?;
    let payload_len = rhs.opening_payload_geometry()?.transmitted_coefficients();
    let payload = receive_base(transcript, &sites.opening_payload, payload_len, proof)?;
    let mut shells = transcript.shell_bundle(&sites.shell, count)?.into_iter();
    let mut challenges = Vec::with_capacity(raw_claims.len());
    for claim in &raw_claims {
        let view = claim.point.view();
        let coordinates = view
            .polynomials()
            .checked_mul(view.blocks())
            .ok_or(AkitaError::InvalidProof)?;
        let mut dense = Vec::new();
        dense
            .try_reserve_exact(
                coordinates
                    .checked_mul(128)
                    .ok_or(AkitaError::InvalidProof)?,
            )
            .map_err(|_| AkitaError::InvalidProof)?;
        for _ in 0..coordinates {
            let shell = shells.next().ok_or(AkitaError::InvalidProof)?;
            for &coefficient in shell.coefficients() {
                dense.push(if coefficient < 0 {
                    -K::from_u64(u64::from(coefficient.unsigned_abs()))
                } else {
                    K::from_u64(coefficient as u64)
                });
            }
        }
        challenges.push(dense);
    }
    if shells.next().is_some() {
        return Err(AkitaError::InvalidProof);
    }
    let view = successor_view(schedule, successor_index, witness_len)?;
    let (coefficient_count, outer) = match (
        schedule.recursive_folds.get(successor_index),
        setup.descriptor().successor(successor_index)?,
    ) {
        (Some(next), FullRingSuccessor::FullCommitment { source }) => {
            let installed = setup.descriptor().source(source)?;
            if installed.raw_view != view
                || setup
                    .descriptor()
                    .level(successor_index + 1)?
                    .sources
                    .as_slice()
                    != [source]
            {
                return Err(invalid(
                    "installed successor source differs from admitted continuation",
                ));
            }
            (
                next.params
                    .outer_payload_geometry()?
                    .transmitted_coefficients(),
                true,
            )
        }
        (None, FullRingSuccessor::Terminal { a }) => {
            let shape = setup.descriptor().matrix(a)?;
            let terminal = &schedule.terminal;
            if shape.ring_dimension != terminal.d_a()
                || shape.rows != terminal.inner.matrix.output_rank()
                || shape.columns != terminal.blocks.positions_per_block
            {
                return Err(invalid(
                    "installed terminal role differs from admitted suffix",
                ));
            }
            (
                terminal
                    .response_shape
                    .layout
                    .groups
                    .first()
                    .ok_or(AkitaError::InvalidProof)?
                    .t_field_elems,
                false,
            )
        }
        _ => {
            return Err(invalid(
                "installed successor kind differs from admitted schedule",
            ))
        }
    };
    let successor_input = *proof;
    let successor = receive_base(transcript, &sites.successor, coefficient_count, proof)?;
    let successor_bytes = successor_input
        .get(
            ..coefficient_count
                .checked_mul(16)
                .ok_or(AkitaError::InvalidProof)?,
        )
        .ok_or(AkitaError::InvalidProof)?;
    let openings = raw_claims
        .into_iter()
        .zip(sources)
        .map(|(claim, &source)| (claim.point, claim.value, source))
        .collect();
    let build = FullRingBuildContext::new(openings, public, payload)?;
    let context = FullRingWitnessContext::new(
        u32::try_from(successor_index).map_err(|_| AkitaError::InvalidProof)?,
        build,
        params.clone(),
        layout.clone(),
        challenges,
    )?;
    let (pending, domain) = context.with_relation(setup, witness_len, |relation| {
        let binding = full_ring_strong_binding(successor_bytes, relation)?;
        let pending =
            crate::verify_full_ring_strong(transcript, &sites.strong, &binding, relation, proof)?;
        let domain = FinalReceiverDomainPlan::from_relation(view, relation)?;
        Ok((pending, domain))
    })?;
    let (point, value) = pending.into_parts();
    let binding = if outer {
        FullRingPendingBinding::OuterPayload(successor)
    } else {
        FullRingPendingBinding::TerminalInnerState(successor)
    };
    Ok(ReductionOutput {
        binding,
        witness_len,
        point,
        value,
        domain,
    })
}

impl<'p, T, R, D> PendingFullRingVerifier<'p, T, R, D>
where
    T: NativeStrongTranscript<R, D>,
{
    pub fn pending_opening(&self) -> (&[E], E) {
        (&self.point, self.value)
    }
    pub fn pending_binding(&self) -> &FullRingPendingBinding {
        &self.binding
    }
    pub fn remaining_recursive_folds(&self) -> usize {
        self.schedule.recursive_folds.len() - self.next_recursive
    }

    /// Completed and declared roots, absent for the ordinary API.
    pub fn accumulation_progress(&self) -> Option<(usize, usize)> {
        self.accumulation
            .as_ref()
            .map(|state| (state.count.completed(), state.count.total()))
    }

    /// Consume one fresh application replay on the retained compiler/reader.
    /// No replacement compiler, pending claim or previous receiver is accepted.
    /// The callback is the application trust boundary: VM/RAM adjacency and
    /// active-row obligations are not implied by native source normalization.
    pub fn update_with_frontend<X: From<AkitaError>>(
        mut self,
        sites: &FullRingUpdateSites<R, D, T::ShellSite>,
        frontend: impl FnOnce(
            NativeTranscriptBorrow<'_, T, R, D>,
            &mut &'p [u8],
            usize,
            &RingVec<K>,
        ) -> Result<FullRingFreshObligations, X>,
    ) -> Result<Self, X> {
        let result = (|| -> Result<_, X> {
            let state = self
                .accumulation
                .as_ref()
                .ok_or_else(|| invalid("ordinary verification cannot repeat its root"))?;
            state.count.require_update()?;
            if self.final_normalized || self.next_recursive != 0 {
                return Err(invalid("repeated root is outside the update phase").into());
            }
            let index = state.count.completed();
            let plan = Arc::clone(&state.plan);
            let FullRingPendingBinding::OuterPayload(receiver) = &self.binding else {
                return Err(invalid("repeated root requires its retained receiver").into());
            };
            let fresh = frontend(
                NativeTranscriptBorrow::new(&mut self.transcript),
                &mut self.proof,
                index,
                receiver,
            )?;
            if index != 0 {
                let normalized =
                    self.normalize_receiver(&sites.receiver_domain, &sites.receiver_normalization)?;
                (self.point, self.value) = normalized.into_parts();
            } else if receiver != plan.zero_receiver().rows() || self.value != E::zero() {
                return Err(invalid("first receiver is not the installed zero").into());
            }
            let binding = std::mem::replace(
                &mut self.binding,
                FullRingPendingBinding::OuterPayload(RingVec::from_coeffs(Vec::new())),
            );
            let FullRingPendingBinding::OuterPayload(receiver) = binding else {
                return Err(AkitaError::InvalidProof.into());
            };
            let mut public = Vec::with_capacity(4);
            public.push(receiver);
            let mut raw = Vec::with_capacity(4);
            raw.push(FullRingRawClaim {
                point: FullRingRawPoint::new(
                    plan.receiver_view()?,
                    std::mem::take(&mut self.point),
                )?,
                value: self.value,
            });
            for (position, fresh) in fresh.sources.into_iter().enumerate() {
                let group_index = position + 1;
                let expected = self
                    .schedule
                    .root
                    .params
                    .group_params(plan.layout(), group_index)?;
                if fresh.commitment.profile() != &expected.profile {
                    return Err(invalid(
                        "fresh commitment differs from installed original profile",
                    )
                    .into());
                }
                fresh.commitment.profile().validate_frozen_precommit(128)?;
                let prefix = &plan.fresh_sources(index)?[position];
                if fresh.claims.is_empty() || fresh.claims.len() > prefix.max_claims() {
                    return Err(invalid("fresh application normalization claim count").into());
                }
                let source = akita_types::FullRingSourceId([2, 2, 1][position]);
                let binding = full_ring_fresh_obligation_binding(
                    prefix,
                    source,
                    fresh.commitment.rows().coeffs(),
                    &fresh.claims,
                )?;
                let domain = crate::verify_full_ring_source_domain(
                    &mut self.transcript,
                    &sites.fresh[position].domain,
                    &binding,
                    prefix.domain(),
                    &mut self.proof,
                )?;
                let mut claims = Vec::with_capacity(fresh.claims.len() + 1);
                for claim in &fresh.claims {
                    claims.push(match claim {
                        FullRingApplicationClaim::Raw { point, value } => {
                            akita_types::full_ring::NormalizationClaim::Raw {
                                point,
                                value: *value,
                            }
                        }
                        FullRingApplicationClaim::Ordinary {
                            decoder,
                            point,
                            value,
                        } => akita_types::full_ring::NormalizationClaim::Ordinary {
                            decoder: prefix
                                .decoders()
                                .get(*decoder)
                                .ok_or_else(|| invalid("uninstalled application decoder"))?,
                            point,
                            value: *value,
                        },
                    });
                }
                claims.push(akita_types::full_ring::NormalizationClaim::Raw {
                    point: domain.point(),
                    value: domain.value(),
                });
                let normalized = crate::verify_full_ring_normalization(
                    &mut self.transcript,
                    &sites.fresh[position].normalization,
                    &binding,
                    prefix.domain().view(),
                    claims,
                    &mut self.proof,
                )?;
                let (point, value) = normalized.into_parts();
                raw.push(FullRingRawClaim {
                    point: FullRingRawPoint::new(prefix.domain().view(), point)?,
                    value,
                });
                public.push(fresh.commitment.commitment.0);
            }
            let output = replay_reduction(
                &self.setup,
                &mut self.transcript,
                &mut self.proof,
                &self.schedule.root.params,
                plan.layout(),
                public,
                raw,
                self.schedule.root.output_witness_len,
                &self.schedule,
                0,
                &sites.reduction,
            )?;
            self.accumulation
                .as_mut()
                .ok_or(AkitaError::InvalidProof)?
                .count
                .complete_update()?;
            Ok(output)
        })();
        let output = match result {
            Ok(output) => output,
            Err(error) => {
                self.transcript.reject();
                return Err(error);
            }
        };
        self.binding = output.binding;
        self.witness_len = output.witness_len;
        self.point = output.point;
        self.value = output.value;
        self.domain = output.domain;
        Ok(self)
    }

    fn normalize_receiver(
        &mut self,
        domain_sites: &akita_transcript::native_strong::FinalReceiverDomainSites<R, D>,
        normalization_sites: &akita_transcript::native_strong::NormalizationSites<R, D>,
    ) -> Result<crate::full_ring::FullRingPendingEvaluation<E>, AkitaError> {
        let FullRingPendingBinding::OuterPayload(payload) = &self.binding else {
            return Err(invalid("terminal has no inner receiver prefix"));
        };
        let binding = akita_types::full_ring_root::full_ring_domain_binding(
            &base_bytes(payload.coeffs())?,
            &self.domain,
        )?;
        let domain = crate::verify_full_ring_final_receiver_domain(
            &mut self.transcript,
            domain_sites,
            &binding,
            &self.domain,
            &mut self.proof,
        )?;
        let claims = vec![
            akita_types::full_ring::NormalizationClaim::Raw {
                point: &self.point,
                value: self.value,
            },
            akita_types::full_ring::NormalizationClaim::Raw {
                point: domain.point(),
                value: domain.value(),
            },
        ];
        crate::verify_full_ring_normalization(
            &mut self.transcript,
            normalization_sites,
            &binding,
            self.domain.view(),
            claims,
            &mut self.proof,
        )
    }

    /// Replay the final same-receiver domain check and normalize its endpoint
    /// with the incoming pending opening. Masks are retained from the original
    /// complete relation, never supplied by the final proof or caller.
    pub fn normalize_final(
        mut self,
        domain_sites: &akita_transcript::native_strong::FinalReceiverDomainSites<R, D>,
        normalization_sites: &akita_transcript::native_strong::NormalizationSites<R, D>,
    ) -> Result<Self, AkitaError> {
        let result = (|| {
            if let Some(state) = &self.accumulation {
                state.count.require_complete()?;
            }
            if self.next_recursive != 0
                || self.final_normalized
                || self.schedule.recursive_folds.is_empty()
            {
                return Err(invalid(
                    "final receiver normalization is not at the admitted inner boundary",
                ));
            }
            self.normalize_receiver(domain_sites, normalization_sites)
        })();
        let pending = match result {
            Ok(pending) => pending,
            Err(error) => {
                self.transcript.reject();
                return Err(error);
            }
        };
        (self.point, self.value) = pending.into_parts();
        self.final_normalized = true;
        Ok(self)
    }

    /// Consume exactly the next catalog-admitted complete shrinking reduction.
    pub fn resume(
        mut self,
        sites: &FullRingReductionSites<R, D, T::ShellSite>,
    ) -> Result<Self, AkitaError> {
        let result = (|| {
            if let Some(state) = &self.accumulation {
                state.count.require_complete()?;
            }
            if !self.final_normalized {
                return Err(invalid(
                    "inner shrinking suffix requires final same-source domain normalization",
                ));
            }
            let step = self
                .schedule
                .recursive_folds
                .get(self.next_recursive)
                .ok_or(AkitaError::InvalidProof)?;
            if self.witness_len != step.input_witness_len || !self.witness_len.is_power_of_two() {
                return Err(AkitaError::InvalidProof);
            }
            let FullRingPendingBinding::OuterPayload(public) = self.binding else {
                return Err(AkitaError::InvalidProof);
            };
            let layout = OpeningClaimsLayout::new(self.witness_len.trailing_zeros() as usize, 1)?;
            let view = successor_view(&self.schedule, self.next_recursive, self.witness_len)?;
            let raw = FullRingRawClaim {
                point: FullRingRawPoint::new(&view, self.point)?,
                value: self.value,
            };
            replay_reduction(
                &self.setup,
                &mut self.transcript,
                &mut self.proof,
                &step.params,
                &layout,
                vec![public],
                vec![raw],
                step.output_witness_len,
                &self.schedule,
                self.next_recursive + 1,
                sites,
            )
        })();
        let output = match result {
            Ok(output) => output,
            Err(error) => {
                self.transcript.reject();
                return Err(error);
            }
        };
        self.binding = output.binding;
        self.witness_len = output.witness_len;
        self.point = output.point;
        self.value = output.value;
        self.domain = output.domain;
        self.next_recursive += 1;
        Ok(self)
    }

    /// Accept only after actual terminal equations, signed integer bounds,
    /// complete proof exhaustion and finalization of the original compiler.
    pub fn finish_terminal(
        mut self,
        sites: &TerminalSites<R, T::ShellSite>,
    ) -> Result<(), AkitaError> {
        let result = (|| {
            if let Some(state) = &self.accumulation {
                state.count.require_complete()?;
                if !self.final_normalized {
                    return Err(invalid(
                        "accumulation terminal requires final normalization",
                    ));
                }
            }
            if self.schedule.recursive_folds.len() != self.next_recursive {
                return Err(AkitaError::InvalidProof);
            }
            let FullRingPendingBinding::TerminalInnerState(images) = self.binding else {
                return Err(AkitaError::InvalidProof);
            };
            let scheduled = &self.schedule.terminal;
            let group = scheduled
                .response_shape
                .layout
                .groups
                .first()
                .ok_or(AkitaError::InvalidProof)?;
            let partial_len = group
                .e_field_elems
                .checked_mul(16)
                .ok_or(AkitaError::InvalidProof)?;
            let coordinates = scheduled
                .blocks
                .positions_per_block
                .checked_mul(128)
                .ok_or(AkitaError::InvalidProof)?;
            let codec = akita_types::full_ring_terminal::FullRingTerminalCodec::new(
                scheduled.blocks.live_blocks,
                coordinates,
                Q,
            )?;
            let partials = take(&mut self.proof, partial_len)?;
            let response = take(&mut self.proof, codec.payload_bytes())?;
            let state = akita_types::full_ring_terminal::FullRingTerminalState {
                witness: images,
                witness_len: self.witness_len,
                opening_point: self.point,
                opening: self.value,
                binding: NextWitnessBindingPolicy::TerminalInnerState,
                pending_setup: false,
                predecessor_level: self.next_recursive,
            };
            crate::verify_live_full_ring_terminal::<T, R, D, K, E>(
                &mut self.transcript,
                sites,
                &self.setup,
                &state,
                scheduled,
                partials,
                response,
            )?;
            if !self.proof.is_empty() {
                return Err(AkitaError::InvalidProof);
            }
            self.transcript.finish()
        })();
        if result.is_err() {
            self.transcript.reject();
        }
        result
    }
}
