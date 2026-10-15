//! Bind reusable commitments to a validated public proof request.
use super::CpuCommitmentMaterialHandle;
use crate::opaque::lifecycle::CpuProofSessionHandle;
use crate::opaque::CpuBackend;
use akita_config::{CommitmentConfig, TrustedScheduleCatalog};
use akita_error::AkitaError;
use akita_params::{FoldSchedule, GroupCommitPhaseParams, OpeningClaimsLayout};
use akita_prover::backend::{ProofAdmission, ProofContext};
use akita_serialization::{AkitaSerialize, Valid};
use akita_types::{AkitaSetupDescriptor, Commitment};
use jolt_field::{CanonicalEncoding, Field};

impl<F: Field, E> akita_prover::backend::ProofScopeConsumer for CpuBackend<F, E> {
    type ProofSessionHandle = CpuProofSessionHandle;

    fn finish_scope(&self, session: &Self::ProofSessionHandle) -> Result<(), AkitaError> {
        let proof = session.validate_owner(self.owner())?;
        self.owner().finish_scope(proof)
    }
    fn abort_scope_best_effort(&self, session: &Self::ProofSessionHandle) {
        if session.belongs_to(self.owner()) {
            self.owner().abort_scope(&session.scope_lease());
        }
    }
}

impl<F: Field, E> CpuBackend<F, E> {
    pub(crate) fn admitted_group(
        &self,
        binding: &crate::opaque::OperationBinding,
    ) -> Result<(akita_params::GroupOpenPhaseParams, usize), AkitaError> {
        self.validate_binding(binding)?;
        let (schedule, _) = binding.scope_lease().proof_plan()?;
        let level = if binding.fold_level() == 0 {
            &schedule.root.params
        } else {
            &schedule
                .recursive_folds
                .get(binding.fold_level() as usize - 1)
                .ok_or_else(|| {
                    AkitaError::InvalidInput(
                        "group operation requires a nonterminal fold level".into(),
                    )
                })?
                .params
        };
        let index = binding.group_index().ok_or_else(|| {
            AkitaError::InvalidInput("group operation requires an ordered group context".into())
        })?;
        Ok((
            *level.groups().get(index).ok_or_else(|| {
                AkitaError::Internal(
                    "admitted group index is missing from the validated fold".into(),
                )
            })?,
            level.witness_chunk.num_chunks,
        ))
    }
}

impl<F, E> ProofAdmission<F, E> for CpuBackend<F, E>
where
    F: Field + CanonicalEncoding + AkitaSerialize + Send + Sync + 'static,
    E: Field + jolt_field::ExtField<F> + Send + Sync + 'static,
{
    fn begin_proof<Cfg>(
        &self,
        setup: &AkitaSetupDescriptor,
        schedules: &TrustedScheduleCatalog<Cfg>,
        plan: &FoldSchedule,
        layout: &OpeningClaimsLayout,
    ) -> Result<Self::ProofSessionHandle, AkitaError>
    where
        Cfg: CommitmentConfig<Field = F, ExtField = E>,
    {
        let prepared = self.prepared()?;
        setup
            .check()
            .map_err(|error| AkitaError::InvalidSetup(error.to_string()))?;
        if setup != &prepared.expanded()?.descriptor {
            return Err(AkitaError::InvalidSetup(
                "proof setup differs from backend setup".into(),
            ));
        }
        Self::validate_proof_configuration(schedules, plan, layout)?;
        plan.validate_structure()?;
        plan.validate_nonterminal_opening_execution(E::DEGREE)?;
        plan.root.params.validate_opening_batch(layout)?;
        if layout.max_point_num_vars() > setup.max_num_vars {
            return Err(AkitaError::InvalidSetup("opening view exceeds backend public-point capacity".into()));
        }
        let required = akita_params::setup_matrix_field_elements_for_schedule(plan)?;
        if required > prepared.expanded()?.shared_matrix.as_field_slice().len() {
            return Err(AkitaError::InvalidSetup(
                "proof plan exceeds backend setup capacity".into(),
            ));
        }
        for (fold_level, parameters) in std::iter::once(&plan.root.params)
            .chain(plan.recursive_folds.iter().map(|fold| &fold.params))
            .enumerate()
        {
            parameters.witness_chunk.validate()?;
            for group in parameters.groups() {
                group.opening.validate_source_view(&group.profile, E::DEGREE)?;
                if group.opening.source_view != akita_params::SourceOpeningView::Decoded
                    && (fold_level != 0 || parameters.witness_chunk.num_chunks != 1)
                {
                    return Err(AkitaError::InvalidSetup("raw RAM opening is root-only single-chunk".into()));
                }
                akita_params::validate_role_dims_for_field::<F>(
                    group.role_dims(parameters.open().matrix.ring_dimension()),
                )?;
            }
        }
        akita_params::dispatch_for_field!(
            akita_params::ProtocolDispatchSlot::Role(akita_params::RingRole::Inner),
            F,
            plan.terminal.d_a(),
            |D| {
                let _ = D;
                Ok::<(), AkitaError>(())
            }
        )?;
        let proof = self.owner().begin_proof(plan, layout)?;
        Ok(CpuProofSessionHandle::new(
            std::sync::Arc::clone(self.owner()),
            proof,
        ))
    }

    fn proof_context(
        &self,
        session: &Self::ProofSessionHandle,
        level: u32,
    ) -> Result<ProofContext, AkitaError> {
        let lease = session.validate_owner(self.owner())?;
        let context = ProofContext::new(
            self.owner_id(),
            self.owner().setup_digest(),
            session.scope_id(),
            level,
        )
        .with_occurrence(lease.context_occurrence(level)?);
        self.validate_context(session, &context)?;
        Ok(context)
    }

    fn validate_commitment(
        &self,
        session: &Self::ProofSessionHandle,
        context: &ProofContext,
        handle: &Self::CommitmentHandle,
        parameters: &GroupCommitPhaseParams,
        commitment: &Commitment<F>,
    ) -> Result<Self::CommitmentMaterialHandle, AkitaError> {
        let binding = self.binding(session, context)?;
        if handle.owner != self.owner_id()
            || handle.committed.parameters != *parameters
            || handle.committed.public.0.coeffs() != commitment.0.coeffs()
            || (commitment.0.ring_dim() != 0
                && commitment.0.ring_dim() != handle.committed.public.0.ring_dim())
        {
            return Err(AkitaError::InvalidInput(
                "commitment does not match its retained source, owner, or statement".into(),
            ));
        }
        let group_index = context.group_index().ok_or_else(|| {
            AkitaError::InvalidInput("commitment admission requires an ordered group".into())
        })?;
        let (schedule, root_layout) = binding.scope_lease().proof_plan()?;
        let (level, layout) = if context.fold_level() == 0 {
            (&schedule.root.params, root_layout)
        } else {
            let step = schedule
                .recursive_folds
                .get(context.fold_level() as usize - 1)
                .ok_or_else(|| {
                    AkitaError::InvalidInput(
                        "terminal state does not admit reusable commitments".into(),
                    )
                })?;
            let layout = step.params.opening_layout_for_final_group(step.params.group())?;
            (&step.params, layout)
        };
        let expected = level.group_params(&layout, group_index)?;
        if expected.profile != *parameters {
            return Err(AkitaError::InvalidInput(
                "commitment parameters do not match the admitted ordered group".into(),
            ));
        }
        if expected.source_view() == akita_params::SourceOpeningView::OriginalRamPlanesV1 {
            if context.fold_level() != 0 || E::DEGREE != 1 {
                return Err(AkitaError::InvalidInput("raw RAM requires native root authority".into()));
            }
            // The selected row fixes this occurrence's original profile/view.
            // Authenticate its retained RAM role, not the compact E2 call order:
            // an accumulation roster can contain many independent RAM handles.
            self.original_ram_plane_reader(handle)?;
        }
        if expected.opening_method().is_full_ring_scalar_extension() {
            let setup = self.full_ring_setup()?;
            let order = layout.root_group_order()?;
            let position = order
                .iter()
                .position(|&group| group == group_index)
                .ok_or_else(|| {
                    AkitaError::InvalidInput(
                        "source group is not in the frozen handle order".into(),
                    )
                })?;
            let source_id = handle
                .committed
                .source
                .full_ring()
                .ok_or_else(|| {
                    AkitaError::InvalidInput("selected source lacks native capability".into())
                })?
                .source_id();
            if setup
                .descriptor()
                .level(context.fold_level() as usize)?
                .sources
                .get(position)
                .copied()
                != Some(source_id)
            {
                return Err(AkitaError::InvalidInput(
                    "commitment original matrix role differs from frozen handle order".into(),
                ));
            }
        }
        let plan = crate::commitment::CommitmentExecutionPlan::for_relation_group(
            level,
            &layout,
            group_index,
            context.fold_level() as usize,
        )?;
        let mut material = CpuCommitmentMaterialHandle::from_state(
            handle.committed.retained.clone(),
            &plan,
            parameters.group.num_polynomials(),
        )?;
        material.bind(binding.clone());
        material.bind_commitment(
            handle.committed.commitment_id,
            Some(handle.committed.public.0.clone()),
        );
        binding
            .scope_lease()
            .admit_commitment(context, handle.committed.commitment_id)?;
        Ok(material)
    }
}

impl<F, E> akita_prover::backend::FullRingProofAdmission<F, E> for CpuBackend<F, E>
where
    F: Field + CanonicalEncoding + AkitaSerialize + 'static,
    E: jolt_field::ExtField<F> + 'static,
{
    fn begin_full_ring_proof<Cfg>(
        &self,
        setup: &akita_types::FullRingSetupDescriptor,
        schedules: &TrustedScheduleCatalog<Cfg>,
        plan: &FoldSchedule,
        layout: &OpeningClaimsLayout,
    ) -> Result<Self::ProofSessionHandle, AkitaError>
    where
        Cfg: CommitmentConfig<Field = F, ExtField = E>,
    {
        akita_types::full_ring::validate_full_ring_fields::<F, E>()?;
        if self.full_ring_setup()?.descriptor().as_ref() != setup {
            return Err(AkitaError::InvalidSetup(
                "full-ring proof setup differs from installed matrices".into(),
            ));
        }
        Self::validate_proof_configuration(schedules, plan, layout)?;
        plan.validate_structure()?;
        plan.validate_nonterminal_opening_execution(E::DEGREE)?;
        plan.root.params.validate_opening_batch(layout)?;
        plan.root.params.validate_full_ring_execution(E::DEGREE)?;
        if setup.family() == akita_types::FullRingSetupFamily::OuterDirect
            && (!plan.recursive_folds.is_empty()
                || setup.coefficient_count() != plan.full_ring_expanded_setup_coefficients()?)
        {
            return Err(AkitaError::InvalidSetup(
                "full-ring direct family matrix inventory mismatch".into(),
            ));
        }
        if setup.levels().len() != plan.recursive_folds.len() + 1 {
            return Err(AkitaError::InvalidSetup(
                "installed continuation count differs from admitted schedule".into(),
            ));
        }
        validate_full_ring_level(setup, 0, &plan.root.params, layout)?;
        for (index, fold) in plan.recursive_folds.iter().enumerate() {
            let layout = fold.params.opening_layout_for_final_group(fold.params.group())?;
            validate_full_ring_level(setup, index + 1, &fold.params, &layout)?;
        }
        let akita_types::FullRingSuccessor::Terminal { a } =
            setup.successor(plan.recursive_folds.len())?
        else {
            return Err(AkitaError::InvalidSetup(
                "last installed transition is not terminal".into(),
            ));
        };
        let terminal = setup.matrix(a)?;
        if terminal.rows != plan.terminal.inner.matrix.output_rank()
            || terminal.columns != plan.terminal.blocks.positions_per_block
            || terminal.ring_dimension != plan.terminal.d_a()
        {
            return Err(AkitaError::InvalidSetup(
                "installed terminal A differs from actual successor plan".into(),
            ));
        }
        let proof = self.owner().begin_proof(plan, layout)?;
        Ok(CpuProofSessionHandle::new(
            std::sync::Arc::clone(self.owner()),
            proof,
        ))
    }

    fn begin_full_ring_accumulation<Cfg>(
        &self,
        setup: &akita_types::FullRingSetupDescriptor,
        schedules: &TrustedScheduleCatalog<Cfg>,
        plan: &FoldSchedule,
        accumulation: &akita_types::full_ring_accumulation::FullRingAccumulationPlan,
        updates: usize,
    ) -> Result<Self::ProofSessionHandle, AkitaError>
    where
        Cfg: CommitmentConfig<Field = F, ExtField = E>,
    {
        accumulation.validate(setup, plan)?;
        if updates != accumulation.num_updates() {
            return Err(AkitaError::InvalidInput(
                "declared roots differ from prefix-plan table".into(),
            ));
        }
        let session = self.begin_full_ring_proof(setup, schedules, plan, accumulation.layout())?;
        session.scope_lease().enable_accumulation(updates)?;
        Ok(session)
    }
}

fn validate_full_ring_level(
    setup: &akita_types::FullRingSetupDescriptor,
    index: usize,
    parameters: &akita_params::CommittedGroupParams,
    layout: &OpeningClaimsLayout,
) -> Result<(), AkitaError> {
    parameters.validate_full_ring_execution(2)?;
    parameters.validate_opening_batch(layout)?;
    let roles = setup.level(index)?;
    let order = layout.root_group_order()?;
    if roles.sources.len() != order.len() {
        return Err(AkitaError::InvalidSetup(
            "installed original source count differs from level".into(),
        ));
    }
    for (&group_index, &source_id) in order.iter().zip(&roles.sources) {
        let source = setup.source(source_id)?;
        let group = parameters.group_params_geometry(layout, group_index)?;
        if source.polynomials != group.profile.group.num_polynomials()
            || source.blocks != group.num_live_blocks()
            || source.positions != group.num_positions_per_block()
            || source.digit_depth != group.num_digits_inner()
            || source.rank != group.a_rows_len()
        {
            return Err(AkitaError::InvalidSetup(
                "original source geometry differs from admitted group".into(),
            ));
        }
    }
    let d = setup.matrix(roles.opening)?;
    if (d.ring_dimension, d.rows, d.columns)
        != (
            parameters.open().matrix.ring_dimension(),
            parameters.open().matrix.output_rank(),
            parameters.d_matrix_width(),
        )
    {
        return Err(AkitaError::InvalidSetup(
            "installed D differs from admitted exact opening geometry".into(),
        ));
    }
    let geometry = akita_params::RelationWitnessGeometry::for_level(parameters, layout, 2)?;
    let chain = geometry.rhs_layout().opening_compression_plan()?;
    if roles.compression.len() != chain.maps().len() {
        return Err(AkitaError::InvalidSetup(
            "installed H stage count mismatch".into(),
        ));
    }
    for (&role, map) in roles.compression.iter().zip(chain.maps()) {
        let matrix = setup.matrix(role)?;
        if (matrix.ring_dimension, matrix.rows, matrix.columns)
            != (map.ring_dimension(), map.output_rank(), map.input_width())
        {
            return Err(AkitaError::InvalidSetup(
                "installed H geometry mismatch".into(),
            ));
        }
    }
    Ok(())
}
