//! Selected full-ring reduction over the existing owner-bound native backend.
//! No EOR, ring-subfield embedding, scalar root batching or ring-switch alpha.
use crate::backend::{
    AcceptedFoldHandle, FoldProbeGeometry, FoldProbeOutcome, FullRingProverBackend,
    NextWitnessBindingMessage, OpeningSource, ProofScope, RecursiveWitnessFoldInput,
    RecursiveWitnessHandle, ValidatedFoldAcceptancePlan, ValidatedFoldProbePlan,
    ValidatedRecursiveWitnessCommitPlan, WitnessCommitmentParameters,
};
use crate::SelectedProverOpeningData;
use akita_challenges::{Challenges, SparseChallenge};
use akita_config::{CommitmentConfig, PlannerOpeningFamily, TrustedScheduleCatalog};
use akita_error::AkitaError;
use akita_params::{FoldSchedule, NextWitnessBindingPolicy, OpeningClaimsLayout};
use akita_transcript::native::{E, K};
use akita_transcript::native_strong::{guarded, NativeCompilerFamily, NativeStrongTranscript};
use akita_types::full_ring::{FullRingRawPoint, RawSourceViewPlan};
pub use akita_types::full_ring_root::{
    full_ring_raw_claim_binding, FullRingRawClaim, FullRingReductionSites,
};
use akita_types::{FullRingSetupDescriptor, FullRingSetupFamily, RingVec};
use jolt_field::CanonicalBytes;
use std::marker::PhantomData;
#[path = "full_ring_accumulation.rs"]
mod accumulation;
pub use accumulation::{begin_full_ring_accumulation, FullRingFreshProverObligations};

/// A live pending relation. Dropping this value aborts its proof scope; there is
/// no detached witness/commitment constructor and no clone of the authority.
pub struct PendingFullRingProver<'a, B, T, R, D>
where
    B: FullRingProverBackend<K, E>,
    T: NativeStrongTranscript<R, D>,
{
    scope: ProofScope<'a, B>,
    backend: &'a B,
    schedule: FoldSchedule,
    transcript: T,
    proof: Vec<u8>,
    witness: B::WitnessHandle,
    material: B::CommitmentMaterialHandle,
    binding: NextWitnessBindingMessage<K>,
    point: Vec<E>,
    value: E,
    domain: akita_types::full_ring::FinalReceiverDomainPlan,
    final_normalized: bool,
    next_recursive: usize,
    context: crate::backend::ProofContext,
    accumulation: Option<(
        std::sync::Arc<akita_types::full_ring_accumulation::FullRingAccumulationPlan>,
        akita_types::full_ring_accumulation::FullRingUpdateCount,
    )>,
    marker: PhantomData<fn() -> (R, D)>,
}

fn invalid(message: &str) -> AkitaError {
    AkitaError::InvalidInput(message.into())
}

fn base_bytes(values: &[K]) -> Result<Vec<u8>, AkitaError> {
    let len = values
        .len()
        .checked_mul(16)
        .ok_or_else(|| invalid("base message overflow"))?;
    let mut bytes = vec![0; len];
    for (value, out) in values.iter().zip(bytes.chunks_exact_mut(16)) {
        value.to_bytes_le(out);
    }
    Ok(bytes)
}

/// Run one complete root reduction after the installed same-source domain and
/// normalization prefix, keeping the same compiler and every native source.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn begin_full_ring_reduction<'a, Cfg, B, T, R, D>(
    setup: &FullRingSetupDescriptor,
    schedules: &TrustedScheduleCatalog<Cfg>,
    backend: &'a B,
    opening: SelectedProverOpeningData<'_, E, B::CommitmentHandle, K>,
    raw_claims: Vec<FullRingRawClaim>,
    mut transcript: T,
    mut proof: Vec<u8>,
    sites: &FullRingReductionSites<R, D, T::ShellSite>,
) -> Result<PendingFullRingProver<'a, B, T, R, D>, AkitaError>
where
    Cfg: CommitmentConfig<Field = K, ExtField = E>,
    B: FullRingProverBackend<K, E>,
    T: NativeStrongTranscript<R, D>,
{
    akita_config::validate_config_policy::<Cfg>()?;
    if !matches!(
        (Cfg::opening_family(), setup.family(), T::FAMILY),
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
    let (selection, claims) = opening.into_low_level_parts();
    let resolved = schedules.resolve_selection(selection)?;
    let layout = claims.opening_layout();
    resolved.validate_opening_layout(layout)?;
    if layout.views().iter().any(|view| *view != akita_params::SourceOpeningView::Decoded) {
        return Err(invalid("original RAM views require the ordinary packing root"));
    }
    let schedule = resolved.schedule();
    if raw_claims.len() != layout.num_groups() {
        return Err(invalid("raw source group count"));
    }
    let session = backend.begin_full_ring_proof(setup, schedules, schedule, layout)?;
    let scope = ProofScope::admitted(backend, session);
    let context = backend.proof_context(scope.session(), 0)?;
    let mut material = Vec::with_capacity(layout.num_groups());
    let mut sources = Vec::with_capacity(layout.num_groups());
    let mut public = Vec::with_capacity(layout.num_groups());
    for index in 0..layout.num_groups() {
        let group = schedule.root.params.group_params(layout, index)?;
        material.push(backend.validate_commitment(
            scope.session(),
            &context.for_group(index),
            claims.group(index)?,
            &group.profile,
            claims.opening_claims().group_commitment(index)?,
        )?);
        sources.push(OpeningSource::Commitment(claims.group(index)?));
        public.push(
            claims
                .opening_claims()
                .group_commitment(index)?
                .rows()
                .clone(),
        );
    }
    let result = guarded(&mut transcript, |transcript| {
        transcript.public(
            &sites.instance,
            1,
            &akita_types::full_ring_root::full_ring_instance_binding(setup, schedule)?,
        )?;
        reduce(
            backend,
            scope.session(),
            transcript,
            &mut proof,
            0,
            &schedule.root.params,
            layout,
            sources,
            material,
            public,
            raw_claims,
            schedule.root.output_witness_len,
            successor_parameters(schedule, 0),
            sites,
        )
    })?;
    Ok(PendingFullRingProver {
        scope,
        backend,
        schedule: schedule.clone(),
        transcript,
        proof,
        witness: result.witness,
        material: result.material,
        binding: result.binding,
        point: result.point,
        value: result.value,
        domain: result.domain,
        final_normalized: false,
        next_recursive: 0,
        context: result.context,
        accumulation: None,
        marker: PhantomData,
    })
}

fn successor_parameters(
    schedule: &FoldSchedule,
    recursive_index: usize,
) -> WitnessCommitmentParameters {
    schedule.recursive_folds.get(recursive_index).map_or_else(
        || WitnessCommitmentParameters::Terminal(schedule.terminal.clone()),
        |step| WitnessCommitmentParameters::Recursive(step.params.clone()),
    )
}

struct ReductionOutput<B: FullRingProverBackend<K, E>> {
    witness: B::WitnessHandle,
    material: B::CommitmentMaterialHandle,
    binding: NextWitnessBindingMessage<K>,
    point: Vec<E>,
    value: E,
    domain: akita_types::full_ring::FinalReceiverDomainPlan,
    context: crate::backend::ProofContext,
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn reduce<B, T, R, D>(
    backend: &B,
    session: &B::ProofSessionHandle,
    transcript: &mut T,
    proof: &mut Vec<u8>,
    level: u32,
    params: &akita_params::CommittedGroupParams,
    layout: &OpeningClaimsLayout,
    sources: Vec<OpeningSource<'_, B::CommitmentHandle, B::WitnessHandle>>,
    materials: Vec<B::CommitmentMaterialHandle>,
    public: Vec<RingVec<K>>,
    raw_claims: Vec<FullRingRawClaim>,
    expected_len: usize,
    successor: WitnessCommitmentParameters,
    sites: &FullRingReductionSites<R, D, T::ShellSite>,
) -> Result<ReductionOutput<B>, AkitaError>
where
    B: FullRingProverBackend<K, E>,
    T: NativeStrongTranscript<R, D>,
{
    params.validate_full_ring_execution(2)?;
    if !params.opening_method().is_full_ring_scalar_extension()
        || params.ring_relation_mode != akita_params::RingRelationMode::ExactCoefficientRows
        || sources.len() != layout.num_groups()
        || raw_claims.len() != sources.len()
    {
        return Err(invalid("selected reduction shape or method mismatch"));
    }
    let context = backend.proof_context(session, level)?;
    let source_ids = sources
        .iter()
        .map(|source| match source {
            OpeningSource::Commitment(handle) => backend.full_ring_source_id(handle),
            OpeningSource::Witness(handle) => backend.full_ring_witness_source_id(handle),
        })
        .collect::<Result<Vec<_>, _>>()?;
    transcript.public(
        &sites.raw_queries,
        raw_claims.len(),
        &full_ring_raw_claim_binding(&raw_claims, &source_ids)?,
    )?;
    let mut prepared = Vec::with_capacity(sources.len());
    for (index, (source, claim)) in sources.into_iter().zip(raw_claims).enumerate() {
        let opening = backend.prepare_full_ring_opening(
            session,
            &context.for_group(index),
            source,
            claim.point,
        )?;
        if opening.scalar_openings() != [claim.value] {
            return Err(invalid("raw claim disagrees with original source"));
        }
        prepared.push(opening.into_opening_handle());
    }
    let geometry = akita_params::RelationWitnessGeometry::for_level(params, layout, 2)?;
    let start = backend.begin_recursive_witness(
        session,
        &context,
        &prepared,
        materials,
        params,
        layout,
        geometry.rhs_layout(),
        &public,
    )?;
    let (_, payload, _, build) = start.into_parts();
    let bytes = base_bytes(payload.coeffs())?;
    transcript.proof(&sites.opening_payload, payload.coeff_len(), &bytes)?;
    proof.extend_from_slice(&bytes);
    let total = (0..layout.num_groups()).try_fold(0usize, |sum, index| {
        let group = params.group_params_geometry(layout, index)?;
        let count = layout
            .group_layout(index)?
            .num_polynomials()
            .checked_mul(group.num_live_blocks())
            .ok_or_else(|| invalid("fold coordinate count overflow"))?;
        sum.checked_add(count)
            .ok_or_else(|| invalid("fold tuple size overflow"))
    })?;
    let mut shells = transcript.shell_bundle(&sites.shell, total)?.into_iter();
    let mut folds = Vec::with_capacity(layout.num_groups());
    for (index, opening) in prepared.iter().enumerate() {
        let group = params.group_params_geometry(layout, index)?;
        let polynomials = layout.group_layout(index)?.num_polynomials();
        let count = polynomials
            .checked_mul(group.num_live_blocks())
            .ok_or_else(|| invalid("fold size"))?;
        let mut sparse = Vec::with_capacity(count);
        for _ in 0..count {
            let shell = shells.next().ok_or_else(|| invalid("short shell tuple"))?;
            let mut value = SparseChallenge {
                positions: Default::default(),
                coeffs: Default::default(),
            };
            for (position, &coefficient) in shell.coefficients().iter().enumerate() {
                if coefficient != 0 {
                    value.positions.push(position as u32);
                    value.coeffs.push(coefficient);
                }
            }
            sparse.push(value);
        }
        let challenges = Challenges::from_sparse(sparse, group.num_live_blocks(), polynomials)?;
        let (negative, positive) = akita_params::sis::balanced_digit_representable_bounds(
            group.log_basis_open(),
            group.num_digits_fold(),
        );
        let plan = ValidatedFoldProbePlan::new::<128>(
            &challenges,
            polynomials,
            group.num_live_blocks(),
            FoldProbeGeometry::Sparse,
            group.num_positions_per_block(),
            group.num_digits_inner(),
            group.log_basis_inner(),
            group.opening_method(),
            ValidatedFoldAcceptancePlan::new(negative, positive, None),
        )?;
        let accepted =
            backend.probe_full_ring_opening_fold(&context.for_group(index), opening, &plan)?;
        let FoldProbeOutcome::Accepted { fold_handle, .. } = accepted else {
            return Err(invalid("all-valid full-ring response envelope violated"));
        };
        if fold_handle.metadata().ring_dimension() != 128 {
            return Err(invalid("fold native ring mismatch"));
        }
        folds.push(RecursiveWitnessFoldInput::new(
            fold_handle,
            akita_types::OpeningFamily::FullRingScalarExtension(challenges),
        ));
    }
    if shells.next().is_some() {
        return Err(invalid("excess shell tuple"));
    }
    let (ring_dimension, source_encoding, binding_policy) = match &successor {
        WitnessCommitmentParameters::Recursive(next) => (
            next.inner().matrix.ring_dimension(),
            next.source_encoding,
            NextWitnessBindingPolicy::OuterPayload,
        ),
        WitnessCommitmentParameters::Terminal(next) => (
            next.d_a(),
            next.source_encoding,
            NextWitnessBindingPolicy::TerminalInnerState,
        ),
    };
    let successor_view = match &successor {
        WitnessCommitmentParameters::Recursive(next) => RawSourceViewPlan::new(
            ring_dimension,
            1,
            next.own_group().num_live_blocks(),
            next.own_group().num_positions_per_block(),
            1,
        )?,
        WitnessCommitmentParameters::Terminal(next) => RawSourceViewPlan::new(
            ring_dimension,
            1,
            next.blocks.live_blocks,
            next.blocks.positions_per_block,
            1,
        )?,
    };
    let witness = backend.finish_full_ring_recursive_witness(build, folds, ring_dimension)?;
    if witness.manifest().commitment_domain_len() != expected_len {
        return Err(invalid("complete full-ring successor domain mismatch"));
    }
    let plan = ValidatedRecursiveWitnessCommitPlan::new(
        witness.manifest().logical_len(),
        expected_len,
        ring_dimension,
        successor,
        Some(source_encoding),
        binding_policy,
    );
    let output = backend.commit_witness(witness, &plan)?;
    let (binding, mut witness, material) = output.into_commitment_parts();
    let bound_fields = match &binding {
        NextWitnessBindingMessage::OuterPayload(commitment)
            if binding_policy == NextWitnessBindingPolicy::OuterPayload =>
        {
            commitment.coeffs()
        }
        NextWitnessBindingMessage::TerminalInnerState(message)
            if binding_policy == NextWitnessBindingPolicy::TerminalInnerState =>
        {
            message.fields()
        }
        _ => return Err(invalid("successor producer binding mismatch")),
    };
    let bytes = base_bytes(bound_fields)?;
    transcript.proof(&sites.successor, bound_fields.len(), &bytes)?;
    proof.extend_from_slice(&bytes);
    let (point, value, domain) = backend.with_full_ring_relation(&witness, |relation| {
        let source = backend.full_ring_witness_source(&witness)?;
        let strong_binding =
            akita_types::full_ring_root::full_ring_strong_binding(&bytes, relation)?;
        let (point, value) = super::full_ring_strong::prove_full_ring_strong(
            transcript,
            &sites.strong,
            &strong_binding,
            source,
            relation,
            proof,
        )?;
        let domain = akita_types::full_ring::FinalReceiverDomainPlan::from_relation(
            successor_view,
            relation,
        )?;
        Ok((point, value, domain))
    })?;
    let context = backend.complete_full_ring_reduction(
        &mut witness,
        &material,
        &binding,
        crate::backend::FullRingCompletion::new(
            context,
            crate::backend::FullRingCompletionKind::Reduction {
                commitment_domain_len: expected_len,
            },
        ),
    )?;
    Ok(ReductionOutput {
        witness,
        material,
        binding,
        point,
        value,
        domain,
        context,
    })
}

impl<'a, B, T, R, D> PendingFullRingProver<'a, B, T, R, D>
where
    B: FullRingProverBackend<K, E>,
    T: NativeStrongTranscript<R, D>,
{
    pub fn pending_opening(&self) -> (&[E], E) {
        (&self.point, self.value)
    }

    pub fn pending_binding(&self) -> &NextWitnessBindingMessage<K> {
        &self.binding
    }

    pub fn remaining_recursive_folds(&self) -> usize {
        self.schedule.recursive_folds.len() - self.next_recursive
    }

    /// Establish the final inner receiver's full range, binary and required-zero
    /// domain on this same source, then normalize with its pending opening.
    pub fn normalize_final(
        mut self,
        domain_sites: &akita_transcript::native_strong::FinalReceiverDomainSites<R, D>,
        normalization_sites: &akita_transcript::native_strong::NormalizationSites<R, D>,
    ) -> Result<Self, AkitaError> {
        let (point, value) = guarded(&mut self.transcript, |transcript| {
            if let Some((_, count)) = &self.accumulation {
                count.require_complete()?;
            }
            if self.next_recursive != 0
                || self.final_normalized
                || self.schedule.recursive_folds.is_empty()
            {
                return Err(invalid(
                    "final receiver normalization is not at the admitted inner boundary",
                ));
            }
            let fields = match &self.binding {
                NextWitnessBindingMessage::OuterPayload(payload) => payload.coeffs(),
                NextWitnessBindingMessage::TerminalInnerState(_) => {
                    return Err(invalid(
                        "ordinary outer direct terminal has no inner final prefix",
                    ))
                }
            };
            let binding = akita_types::full_ring_root::full_ring_domain_binding(
                &base_bytes(fields)?,
                &self.domain,
            )?;
            let source = self.backend.full_ring_witness_source(&self.witness)?;
            let (domain_point, domain_value) =
                super::full_ring_strong::prove_full_ring_final_receiver_domain(
                    transcript,
                    domain_sites,
                    &binding,
                    source,
                    &self.domain,
                    &mut self.proof,
                )?;
            let claims = vec![
                akita_types::full_ring::NormalizationClaim::Raw {
                    point: &self.point,
                    value: self.value,
                },
                akita_types::full_ring::NormalizationClaim::Raw {
                    point: &domain_point,
                    value: domain_value,
                },
            ];
            super::full_ring_strong::prove_full_ring_normalization(
                transcript,
                normalization_sites,
                &binding,
                source,
                self.domain.view(),
                claims,
                &mut self.proof,
            )
        })?;
        self.point = point;
        self.value = value;
        self.final_normalized = true;
        guarded(&mut self.transcript, |_| {
            self.backend.complete_full_ring_phase(
                self.scope.session(),
                crate::backend::FullRingCompletion::new(
                    self.context,
                    crate::backend::FullRingCompletionKind::FinalNormalization,
                ),
            )
        })?;
        Ok(self)
    }

    /// Consume one already-admitted complete shrinking fold. The predecessor
    /// witness is released only after its successor has been built and checked.
    pub fn resume(
        mut self,
        sites: &FullRingReductionSites<R, D, T::ShellSite>,
    ) -> Result<Self, AkitaError> {
        let result = guarded(&mut self.transcript, |transcript| {
            if let Some((_, count)) = &self.accumulation {
                count.require_complete()?;
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
                .ok_or_else(|| invalid("pending full-ring state is at its terminal edge"))?;
            if self.witness.manifest().commitment_domain_len() != step.input_witness_len
                || !step.input_witness_len.is_power_of_two()
            {
                return Err(invalid("full-ring continuation input geometry"));
            }
            let NextWitnessBindingMessage::OuterPayload(public) = self.binding else {
                return Err(invalid(
                    "an A-only terminal cannot be adopted as a full commitment",
                ));
            };
            let layout =
                OpeningClaimsLayout::new(step.input_witness_len.trailing_zeros() as usize, 1)?;
            let group = step.params.group_params_geometry(&layout, 0)?;
            let view = RawSourceViewPlan::new(
                128,
                1,
                group.num_live_blocks(),
                group.num_positions_per_block(),
                1,
            )?;
            if view.physical_len() != step.input_witness_len {
                return Err(invalid("recursive identity source length"));
            }
            let raw = FullRingRawClaim {
                point: FullRingRawPoint::new(&view, self.point)?,
                value: self.value,
            };
            let successor = successor_parameters(&self.schedule, self.next_recursive + 1);
            reduce(
                self.backend,
                self.scope.session(),
                transcript,
                &mut self.proof,
                u32::try_from(self.next_recursive + 1)
                    .map_err(|_| invalid("fold index overflow"))?,
                &step.params,
                &layout,
                vec![OpeningSource::Witness(&self.witness)],
                vec![self.material],
                vec![public],
                vec![raw],
                step.output_witness_len,
                successor,
                sites,
            )
        })?;
        guarded(&mut self.transcript, |_| {
            self.backend.release_witness_handle(self.witness)
        })?;
        self.witness = result.witness;
        self.material = result.material;
        self.binding = result.binding;
        self.point = result.point;
        self.value = result.value;
        self.domain = result.domain;
        self.context = result.context;
        self.next_recursive += 1;
        Ok(self)
    }

    /// Finish only the genuine A-only terminal already selected at witness
    /// creation. No suffix obligations, compiler state or proof scope escape.
    pub fn finish_terminal(
        mut self,
        sites: &akita_transcript::native_strong::TerminalSites<R, T::ShellSite>,
    ) -> Result<Vec<u8>, AkitaError> {
        let result = guarded(&mut self.transcript, |transcript| {
            if let Some((_, count)) = &self.accumulation {
                count.require_complete()?;
            }
            if self.schedule.recursive_folds.len() != self.next_recursive {
                return Err(invalid("full-ring terminal has unconsumed recursive edges"));
            }
            let NextWitnessBindingMessage::TerminalInnerState(message) = &self.binding else {
                return Err(invalid(
                    "full-ring terminal requires its original A-only binding",
                ));
            };
            let scheduled = &self.schedule.terminal;
            let state = akita_types::full_ring_terminal::FullRingTerminalState {
                predecessor_level: self.next_recursive,
                witness: RingVec::from_coeffs(message.fields().to_vec()),
                witness_len: self.witness.manifest().commitment_domain_len(),
                opening_point: self.point.clone(),
                opening: self.value,
                binding: NextWitnessBindingPolicy::TerminalInnerState,
                pending_setup: false,
            };
            let level = u32::try_from(self.next_recursive + 1)
                .map_err(|_| invalid("terminal level overflow"))?;
            let context = self.backend.proof_context(self.scope.session(), level)?;
            let view = RawSourceViewPlan::new(
                128,
                1,
                scheduled.blocks.live_blocks,
                scheduled.blocks.positions_per_block,
                1,
            )?;
            let point = FullRingRawPoint::new(&view, self.point.clone())?;
            let partials = self.backend.prepare_full_ring_terminal(
                self.scope.session(),
                &context,
                &self.witness,
                point,
            )?;
            let partial_bytes = base_bytes(partials.coefficients())?;
            let challenges = akita_types::full_ring_terminal::bind_terminal_and_draw::<
                T,
                R,
                D,
                K,
                E,
            >(transcript, sites, &state, scheduled, &partial_bytes)?;
            self.proof.extend_from_slice(&partial_bytes);
            let response = self
                .backend
                .encode_full_ring_terminal_response(&self.witness, &challenges)?;
            let coordinates = scheduled
                .blocks
                .positions_per_block
                .checked_mul(128)
                .ok_or_else(|| invalid("terminal coordinate overflow"))?;
            transcript.proof(&sites.response, coordinates, &response)?;
            self.proof.extend_from_slice(&response);
            transcript.finish()
        });
        result?;
        guarded(&mut self.transcript, |_| {
            self.backend.complete_full_ring_phase(
                self.scope.session(),
                crate::backend::FullRingCompletion::new(
                    self.context,
                    crate::backend::FullRingCompletionKind::Terminal,
                ),
            )
        })?;
        guarded(&mut self.transcript, |_| {
            self.backend.release_witness_handle(self.witness)?;
            self.scope.finish()
        })?;
        Ok(self.proof)
    }
}
