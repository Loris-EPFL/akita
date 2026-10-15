use super::*;
use crate::SelectedProverOpeningData;
use akita_config::TrustedScheduleCatalog;
use jolt_field::AdditiveGroup;

/// Complete one admitted root reduction, retaining its live source scope and
/// pending opening instead of proving an entire segment PCS suffix.
#[allow(clippy::too_many_arguments)]
pub fn begin_batched_reduction<'backend, 'source, Cfg, B>(
    expanded: &'backend akita_types::AkitaSetupDescriptor,
    prefix_slots: &'backend SetupPrefixProverRegistry<Cfg::Field, B::CommitmentHandle>,
    schedules: &TrustedScheduleCatalog<Cfg>,
    backend: &'backend B,
    opening: SelectedProverOpeningData<'source, Cfg::ExtField, B::CommitmentHandle, Cfg::Field>,
    transcript_session: &[u8],
    registry: &akita_transcript::InstalledRegistry,
    augmentation: akita_transcript::InheritedAugmentation<'_>,
    basis: BasisMode,
) -> Result<PendingNativeProver<'backend, Cfg, B>, AkitaError>
where
    Cfg: CommitmentConfig,
    Cfg::Field: CanonicalEncoding + AkitaSerialize + Unreduced + PseudoMersenne + Ring + 'static,
    <Cfg::Field as Unreduced>::Wide: From<Cfg::Field> + AdditiveGroup,
    Cfg::ExtField: FpExtEncoding<Cfg::Field>
        + ExtField<Cfg::Field>
        + Unreduced
        + Fold
        + Ring
        + MulBaseUnreduced<Cfg::Field>
        + AkitaSerialize
        + 'static,
    B: ProverBackend<Cfg::Field, Cfg::ExtField>,
{
    let (selection, claims) = opening.into_low_level_parts();
    let resolved = schedules.resolve_selection(selection)?;
    let layout = claims.opening_layout();
    resolved.validate_opening_layout(layout)?;
    layout.validate_basis(basis)?;
    if layout.max_point_num_vars() > expanded.max_num_vars {
        return Err(AkitaError::InvalidSetup(
            "opening query exceeds the installed setup point capacity".into(),
        ));
    }
    let schedule = resolved.schedule();
    for index in 0..layout.num_groups() {
        if !matches!(
            schedule.root.params.group_params_geometry(layout, index)?.opening_method(),
            akita_params::OpeningMethod::SubringCoefficientPacking { .. }
        ) {
            return Err(AkitaError::InvalidSetup(
                "ordinary root requires coefficient packing without opening reduction".into(),
            ));
        }
    }
    let proof_session = backend.begin_proof(expanded, schedules, schedule, layout)?;
    let guard = crate::backend::ProofScope::admitted(backend, proof_session);
    let proof_session = guard.session();
    let context = backend.proof_context(proof_session, 0)?;
    let mut material = Vec::with_capacity(layout.num_groups());
    for index in 0..layout.num_groups() {
        let group = schedule.root.params.group_params(layout, index)?;
        material.push(backend.validate_commitment(
            proof_session,
            &context.for_group(index),
            claims.group(index)?,
            &group.profile,
            claims.opening_claims().group_commitment(index)?,
        )?);
    }
    let (grinding_plan, descriptor_bytes) = akita_config::transcript_instance_descriptor::<
        Cfg::Field,
        Cfg,
    >(expanded, layout, selection, schedule, basis)?;
    let endpoint_binding = akita_types::encode_opening_endpoint_binding(
        &descriptor_bytes,
        resolved.profiles(),
        claims.opening_claims(),
        |commitment| commitment,
    )?;
    let installed = akita_transcript::InstalledContext::new_augmented(
        registry,
        transcript_session,
        &descriptor_bytes,
        augmentation,
        &endpoint_binding,
    )?;
    let channel = akita_transcript::new_prover_channel(&installed)?;
    let mut grinding = akita_types::ProverGrinding::with_owned_plan(channel, grinding_plan);
    claims.append_to(&schedule.root.params, &mut grinding)?;
    let root_claims = claims.map_groups(OpeningSource::Commitment)?;
    let (next_params, next_binding) = schedule.recursive_folds.first().map_or(
        (
            fold::FoldSuccessorParams::Terminal(&schedule.terminal),
            akita_params::NextWitnessBindingPolicy::TerminalInnerState,
        ),
        |next| {
            (
                fold::FoldSuccessorParams::Recursive(next),
                akita_params::NextWitnessBindingPolicy::OuterPayload,
            )
        },
    );
    let prepared = prepare_fold::<Cfg::Field, Cfg::ExtField, B>(
        backend,
        root_claims,
        material,
        false,
        &mut grinding,
        0,
        &schedule.root.params,
        basis,
        proof_session,
        schedule.root.output_witness_len,
        next_params.inner_ring_dimension(),
        false,
    )?;
    let root = prove_fold::<Cfg::Field, Cfg::ExtField, B>(
        expanded,
        prefix_slots,
        backend,
        proof_session,
        &mut grinding,
        0,
        &schedule.root.params,
        next_params,
        schedule.root.output_witness_len,
        next_binding,
        prepared,
    )?;
    Ok(PendingNativeProver {
        scope: guard,
        backend,
        expanded,
        prefix_slots,
        schedule: schedule.clone(),
        grinding,
        state: root.next_state,
        next_recursive: 0,
    })
}

/// An unfinished native relation, including the sole live authority for its
/// retained sources. Dropping it aborts that scope; it is not a completed proof.
pub struct PendingNativeProver<'a, Cfg, B>
where
    Cfg: CommitmentConfig,
    B: ProverBackend<Cfg::Field, Cfg::ExtField>,
{
    scope: crate::backend::ProofScope<'a, B>,
    backend: &'a B,
    expanded: &'a akita_types::AkitaSetupDescriptor,
    prefix_slots: &'a SetupPrefixProverRegistry<Cfg::Field, B::CommitmentHandle>,
    schedule: FoldSchedule,
    grinding: akita_types::ProverGrinding<'static>,
    state:
        SuffixProverState<Cfg::Field, Cfg::ExtField, B::CommitmentMaterialHandle, B::WitnessHandle>,
    next_recursive: usize,
}

impl<'a, Cfg, B> PendingNativeProver<'a, Cfg, B>
where
    Cfg: CommitmentConfig,
    Cfg::Field: CanonicalEncoding + AkitaSerialize + Unreduced + PseudoMersenne + Ring + 'static,
    <Cfg::Field as Unreduced>::Wide: From<Cfg::Field> + AdditiveGroup,
    Cfg::ExtField: FpExtEncoding<Cfg::Field>
        + ExtField<Cfg::Field>
        + Unreduced
        + Fold
        + Ring
        + MulBaseUnreduced<Cfg::Field>
        + AkitaSerialize
        + 'static,
    B: ProverBackend<Cfg::Field, Cfg::ExtField>,
{
    /// The exact still-pending opening. Neither this view nor successful
    /// reduction replay authenticates it without the remaining descendant.
    pub fn pending_opening(&self) -> (&[Cfg::ExtField], Cfg::ExtField) {
        (&self.state.sumcheck_challenges, self.state.opening)
    }

    pub fn remaining_recursive_folds(&self) -> usize {
        self.schedule.recursive_folds.len() - self.next_recursive
    }

    /// Resume exactly one admitted recursive edge with the existing transcript,
    /// commitment material and witness handle; no source is reconstructed.
    pub fn resume(mut self) -> Result<Self, AkitaError> {
        let step = self
            .schedule
            .recursive_folds
            .get(self.next_recursive)
            .ok_or_else(|| {
                AkitaError::InvalidInput(
                    "pending native state is already at its terminal edge".into(),
                )
            })?;
        if self.state.witness_handle.manifest().logical_len() != step.input_witness_len {
            return Err(AkitaError::InvalidProof);
        }
        let level = self.next_recursive + 1;
        let (next_params, binding) = self.schedule.recursive_folds.get(level).map_or(
            (
                fold::FoldSuccessorParams::Terminal(&self.schedule.terminal),
                akita_params::NextWitnessBindingPolicy::TerminalInnerState,
            ),
            |next| {
                (
                    fold::FoldSuccessorParams::Recursive(next),
                    akita_params::NextWitnessBindingPolicy::OuterPayload,
                )
            },
        );
        let prepared = suffix::prepare_suffix::<Cfg::Field, Cfg::ExtField, B>(
            self.backend,
            self.prefix_slots,
            &mut self.grinding,
            self.state,
            level,
            &step.params,
            self.scope.session(),
            step.output_witness_len,
            next_params.inner_ring_dimension(),
        )?;
        self.state = prove_fold::<Cfg::Field, Cfg::ExtField, B>(
            self.expanded,
            self.prefix_slots,
            self.backend,
            self.scope.session(),
            &mut self.grinding,
            level,
            &step.params,
            next_params,
            step.output_witness_len,
            binding,
            prepared,
        )?
        .next_state;
        self.next_recursive += 1;
        Ok(self)
    }

    /// Discharge every remaining edge and the actual terminal before finishing
    /// either the scope or the transcript. There is no detached-handle finish.
    pub fn finish(mut self) -> Result<Vec<u8>, AkitaError> {
        while self.remaining_recursive_folds() != 0 {
            self = self.resume()?;
        }
        if self.state.witness_handle.manifest().logical_len()
            != self.schedule.terminal.input_witness_len
        {
            return Err(AkitaError::InvalidProof);
        }
        suffix::prove_terminal_suffix::<Cfg::Field, Cfg::ExtField, B>(
            self.backend,
            &mut self.grinding,
            self.next_recursive + 1,
            self.state,
            &self.schedule.terminal,
            self.scope.session(),
        )?;
        let proof = self.grinding.finish()?;
        self.scope.finish()?;
        Ok(proof)
    }
}

/// Prove the complete admitted native batch using the same owned reduction
/// continuation as callers that explicitly pause before the suffix.
#[allow(clippy::too_many_arguments)]
pub fn batched_prove<Cfg, B>(
    expanded: &akita_types::AkitaSetupDescriptor,
    prefix_slots: &SetupPrefixProverRegistry<Cfg::Field, B::CommitmentHandle>,
    schedules: &TrustedScheduleCatalog<Cfg>,
    backend: &B,
    opening: SelectedProverOpeningData<'_, Cfg::ExtField, B::CommitmentHandle, Cfg::Field>,
    transcript_session: &[u8],
    registry: &akita_transcript::InstalledRegistry,
    augmentation: akita_transcript::InheritedAugmentation<'_>,
    basis: BasisMode,
) -> Result<Vec<u8>, AkitaError>
where
    Cfg: CommitmentConfig,
    Cfg::Field: CanonicalEncoding + AkitaSerialize + Unreduced + PseudoMersenne + Ring + 'static,
    <Cfg::Field as Unreduced>::Wide: From<Cfg::Field> + AdditiveGroup,
    Cfg::ExtField: FpExtEncoding<Cfg::Field>
        + ExtField<Cfg::Field>
        + Unreduced
        + Fold
        + Ring
        + MulBaseUnreduced<Cfg::Field>
        + AkitaSerialize
        + 'static,
    B: ProverBackend<Cfg::Field, Cfg::ExtField>,
{
    begin_batched_reduction::<Cfg, B>(
        expanded,
        prefix_slots,
        schedules,
        backend,
        opening,
        transcript_session,
        registry,
        augmentation,
        basis,
    )?
    .finish()
}
