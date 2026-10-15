use super::commitment_material::CpuCommitmentMaterial;
use super::compression_witness::{
    materialize_compression_witness, CompressionSourceId, CompressionSourceWitness,
    CompressionWitnessMaterialization,
};
use super::finalize::{cpu_recursive_witness_build, RelationDQuotientWitness, RingRelationWitness};
use super::prepared_openings::{
    prepare_group_opening_witness, prepare_opening_relation_rows, PreparedOpeningWitness,
};
use crate::opaque::OperationCtx;
use akita_error::AkitaError;
use akita_params::{dispatch_for_field, CommittedGroupParams};
use akita_serialization::AkitaSerialize;
use akita_types::{RingRelationInstance, RingVec};
use jolt_field::{CanonicalEncoding, Field, Ring};

pub(crate) struct PreparedRelationPayload<F: Field + CanonicalEncoding> {
    inner: Vec<crate::opaque::OpaqueInnerRelationState<F>>,
    relation_rhs: RingVec<F>,
    opening_payload: RingVec<F>,
    opening_payload_ring_dimension: usize,
    compression: Option<CompressionWitnessMaterialization<F>>,
}

/// CPU consumer state retained between public payload publication and the
/// transcript-owned fold grind. No protocol module can inspect its E/T/R or
/// compression material.
pub(crate) struct CpuRecursiveWitnessAssemblyState<F: Field + CanonicalEncoding> {
    group_openings: Vec<PreparedOpeningWitness<F>>,
    d_quotients: RelationDQuotientWitness<F>,
    inner_relation: Vec<crate::opaque::OpaqueInnerRelationState<F>>,
    compression: Option<CompressionWitnessMaterialization<F>>,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn begin_cpu_recursive_witness<F, E>(
    backend: &crate::opaque::CpuBackend<F, E>,
    prepared: &crate::opaque::CpuPreparedSetup<F>,
    binding: crate::opaque::OperationBinding,
    opening_bindings: Vec<crate::opaque::OperationBinding>,
    prepared_group_openings: &[crate::opaque::CpuPreparedOpeningHandle<F, E>],
    commitment_material: Vec<CpuCommitmentMaterial<F>>,
    level: &CommittedGroupParams,
    opening_batch: &akita_params::OpeningClaimsLayout,
    relation_rhs_layout: &akita_params::RelationRhsLayout,
    group_commitments: &[RingVec<F>],
) -> Result<
    crate::opaque::RecursiveWitnessBuildStart<F, E, crate::opaque::CpuWitnessBuildHandle<F>>,
    AkitaError,
>
where
    F: Field + CanonicalEncoding + AkitaSerialize + Ring + Send + Sync + 'static,
    E: jolt_field::ExtField<F> + Send + Sync + 'static,
{
    let ctx = if level.ring_relation_mode == akita_params::RingRelationMode::ExactCoefficientRows {
        OperationCtx::for_full_ring(backend, prepared)?
    } else {
        OperationCtx::new(
            backend,
            prepared,
            crate::opaque::ComputeBackendSetup::prepared_expanded_setup(backend, prepared)?,
        )?
    };
    if prepared_group_openings.len() != opening_batch.num_groups()
        || commitment_material.len() != opening_batch.num_groups()
        || opening_bindings.len() != opening_batch.num_groups()
    {
        return Err(AkitaError::Internal(format!(
            "retained witness build group count mismatch: expected {}, actual {}",
            opening_batch.num_groups(),
            prepared_group_openings
                .len()
                .min(commitment_material.len())
                .min(opening_bindings.len()),
        )));
    }
    let geometry =
        akita_params::RelationWitnessGeometry::for_level(level, opening_batch, E::DEGREE)?;
    let mut group_openings = Vec::with_capacity(opening_batch.num_groups());
    let mut public_groups = Vec::with_capacity(opening_batch.num_groups());
    for (group_index, opening) in prepared_group_openings.iter().enumerate() {
        let group_dims = level.group_role_dims_geometry(opening_batch, group_index)?;
        let (opening, public_kind, scalar_openings) = dispatch_for_field!(
            ProtocolDispatchSlot::Role(RingRole::Opening),
            F,
            group_dims.d_d(),
            |D_D| prepare_group_opening_witness::<F, E, D_D>(
                opening,
                level,
                opening_batch,
                &geometry,
                group_index,
                group_dims,
            )
        )?;
        group_openings.push(opening);
        public_groups.push(crate::opaque::PreparedRelationGroupPublic::new(
            public_kind,
            scalar_openings,
        ));
    }
    if !level.ring_relation_mode.is_exact_coefficient_rows() {
        crate::arithmetic::requirements::warm_relation_ntt_cache(backend, prepared, level)?;
    }
    let dims = level.role_dims();
    let (v, d_quotients) = dispatch_for_field!(
        ProtocolDispatchSlot::Role(RingRole::Opening),
        F,
        dims.d_d(),
        |D_D| prepare_opening_relation_rows::<F, crate::opaque::CpuBackend<F, E>, D_D>(
            &ctx,
            opening_batch,
            &group_openings,
            level.has_preceding_groups(),
            level.open().matrix.output_rank(),
            level.shared_d_digit_log_basis(),
            level.ring_relation_mode,
        )
    )?;
    let payload = prepare_relation_payload(
        &ctx,
        binding.fold_level(),
        level,
        opening_batch,
        relation_rhs_layout,
        group_commitments,
        commitment_material,
        &v,
    )?;
    let PreparedRelationPayload {
        inner,
        relation_rhs,
        opening_payload,
        opening_payload_ring_dimension,
        compression,
    } = payload;
    let full_ring_context =
        if level.ring_relation_mode == akita_params::RingRelationMode::ExactCoefficientRows {
            Some(Box::new(akita_types::full_ring::FullRingBuildContext::new(
                prepared_group_openings
                    .iter()
                    .map(|opening| opening.full_ring_public())
                    .collect::<Result<Vec<_>, _>>()?,
                group_commitments.to_vec(),
                opening_payload.clone(),
            )?) as Box<dyn std::any::Any + Send + Sync>)
        } else {
            None
        };
    let build_handle = crate::opaque::CpuWitnessBuildHandle {
        full_ring_context,
        binding,
        opening_bindings,
        assembly_state: CpuRecursiveWitnessAssemblyState {
            group_openings,
            d_quotients,
            inner_relation: inner,
            compression,
        },
        relation_rhs,
        v,
        level: level.clone(),
        opening_batch: opening_batch.clone(),
    };
    Ok(crate::opaque::RecursiveWitnessBuildStart::new(
        public_groups,
        opening_payload,
        opening_payload_ring_dimension,
        build_handle,
    ))
}

pub(crate) fn finish_cpu_recursive_witness<F, E>(
    backend: &crate::opaque::CpuBackend<F, E>,
    prepared: &crate::opaque::CpuPreparedSetup<F>,
    build_handle: crate::opaque::CpuWitnessBuildHandle<F>,
    fold_inputs: Vec<crate::opaque::RecursiveWitnessFoldInput<crate::opaque::CpuAcceptedFold<F>>>,
    relation: &RingRelationInstance<F>,
    plan: &crate::opaque::ValidatedRecursiveWitnessPlan<'_, F>,
) -> Result<crate::opaque::CpuWitnessHandle, AkitaError>
where
    F: Field + CanonicalEncoding + AkitaSerialize + Ring + Send + Sync + 'static,
{
    for (group_index, fold_input) in fold_inputs.iter().enumerate() {
        let group_params = build_handle
            .level
            .group_params_geometry(&build_handle.opening_batch, group_index)?;
        fold_input.fold_handle().validate_for_build(
            &build_handle.binding,
            &group_params,
            build_handle
                .opening_batch
                .group_layout(group_index)?
                .num_polynomials(),
            build_handle.level.witness_chunk.num_chunks,
        )?;
    }
    let crate::opaque::CpuWitnessBuildHandle {
        binding,
        assembly_state,
        relation_rhs,
        v,
        level,
        opening_batch,
        ..
    } = build_handle;
    let CpuRecursiveWitnessAssemblyState {
        group_openings,
        d_quotients,
        inner_relation,
        compression,
    } = assembly_state;
    if group_openings.len() != fold_inputs.len() || inner_relation.len() != fold_inputs.len() {
        return Err(AkitaError::Internal(
            "retained witness build opening or inner-relation count differs from the fold count"
                .into(),
        ));
    }
    let mut group_witnesses = Vec::with_capacity(fold_inputs.len());
    for (group_index, ((fold, opening), inner_relation)) in fold_inputs
        .into_iter()
        .zip(group_openings)
        .zip(inner_relation)
        .enumerate()
    {
        let (fold_handle, challenges) = fold.into_fold_handle_and_challenges();
        let group_dims = level.group_role_dims_geometry(&opening_batch, group_index)?;
        let source_count = opening_batch.group_layout(group_index)?.num_polynomials();
        if inner_relation.ring_dimension() != group_dims.d_a()
            || inner_relation.source_count() != source_count
        {
            return Err(AkitaError::Internal(
                "inner-relation state shape does not match its commitment group".into(),
            ));
        }
        let witness =
            opening.into_relation_witness(fold_handle, challenges, inner_relation, group_dims)?;
        group_witnesses.push(witness);
    }
    let witness = RingRelationWitness::from_groups(group_witnesses, d_quotients, compression);
    RingRelationInstance::check_v_shape_for_level(&v, &level)?;
    if relation_rhs != *relation.rhs() {
        return Err(AkitaError::InvalidInput(
            "backend relation RHS disagrees with the transcript-bound relation".into(),
        ));
    }
    if relation.segment_layout(&level, None)?.live_coeff_len() != plan.logical_len() {
        return Err(AkitaError::InvalidInput(
            "recursive witness plan disagrees with its relation instance".into(),
        ));
    }
    let expanded = crate::opaque::ComputeBackendSetup::prepared_expanded_setup(backend, prepared)?;
    let ctx = OperationCtx::new(backend, prepared, expanded)?;
    let witness_handle = cpu_recursive_witness_build(
        relation,
        witness,
        &ctx,
        &ctx,
        &level,
        plan.commitment_ring_dimension(),
        binding,
    )?;
    Ok(witness_handle)
}

/// Finish the same retained assembly under the exact coefficient-row family.
/// No alpha-era gamma, ring-switch point, or synthetic relation instance exists.
pub(crate) fn finish_cpu_full_ring_witness<F, E>(
    backend: &crate::opaque::CpuBackend<F, E>,
    prepared: &crate::opaque::CpuPreparedSetup<F>,
    build_handle: crate::opaque::CpuWitnessBuildHandle<F>,
    fold_inputs: Vec<crate::opaque::RecursiveWitnessFoldInput<crate::opaque::CpuAcceptedFold<F>>>,
    commitment_ring_dimension: usize,
) -> Result<crate::opaque::CpuWitnessHandle, AkitaError>
where
    F: Field + CanonicalEncoding + AkitaSerialize + Ring + Send + Sync + 'static,
    E: jolt_field::ExtField<F> + 'static,
{
    let crate::opaque::CpuWitnessBuildHandle {
        binding,
        assembly_state,
        level,
        opening_batch,
        full_ring_context,
        ..
    } = build_handle;
    if level.ring_relation_mode != akita_params::RingRelationMode::ExactCoefficientRows
        || commitment_ring_dimension != 128
        || level.witness_chunk.num_chunks != 1
    {
        return Err(AkitaError::InvalidInput(
            "full-ring successor requires exact rows and unsplit native128 geometry".into(),
        ));
    }
    let build_context = *full_ring_context
        .ok_or_else(|| {
            AkitaError::InvalidInput(
                "selected witness is missing original full-ring relation metadata".into(),
            )
        })?
        .downcast::<akita_types::full_ring::FullRingBuildContext<F, E>>()
        .map_err(|_| AkitaError::InvalidInput("full-ring metadata field mismatch".into()))?;
    let challenges = fold_inputs
        .iter()
        .map(|fold| {
            akita_types::full_ring::full_ring_dense_challenges::<F>(fold.challenges().ambient_a())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let CpuRecursiveWitnessAssemblyState {
        group_openings,
        d_quotients,
        inner_relation,
        compression,
    } = assembly_state;
    if fold_inputs.len() != opening_batch.num_groups()
        || group_openings.len() != fold_inputs.len()
        || inner_relation.len() != fold_inputs.len()
        || compression.is_none()
    {
        return Err(AkitaError::InvalidInput(
            "complete full-ring successor source set is missing".into(),
        ));
    }
    let context = akita_types::full_ring::FullRingWitnessContext::new(
        binding.fold_level(),
        build_context,
        level,
        opening_batch,
        challenges,
    )?;
    let level = context.level();
    let opening_batch = context.opening_batch();
    let layout = context.layout();
    let mut groups = Vec::with_capacity(fold_inputs.len());
    for (index, ((fold, opening), inner)) in fold_inputs
        .into_iter()
        .zip(group_openings)
        .zip(inner_relation)
        .enumerate()
    {
        let parameters = level.group_params_geometry(opening_batch, index)?;
        let dims = level.group_role_dims_geometry(opening_batch, index)?;
        let sources = opening_batch.group_layout(index)?.num_polynomials();
        if !parameters.opening_method().is_full_ring_scalar_extension()
            || inner.ring_dimension() != dims.d_a()
            || inner.source_count() != sources
        {
            return Err(AkitaError::InvalidInput(
                "full-ring successor original A material mismatch".into(),
            ));
        }
        fold.fold_handle()
            .validate_for_build(&binding, &parameters, sources, 1)?;
        let (accepted, challenges) = fold.into_fold_handle_and_challenges();
        groups.push(opening.into_relation_witness(accepted, challenges, inner, dims)?);
    }
    let witness = RingRelationWitness::from_groups(groups, d_quotients, compression);
    let ctx = OperationCtx::for_full_ring(backend, prepared)?;
    let mut witness = super::finalize::cpu_witness_build_with_layout(
        None,
        opening_batch,
        layout,
        witness,
        &ctx,
        &ctx,
        level,
        commitment_ring_dimension,
        binding,
    )?;
    witness.full_ring_context = Some(std::sync::Arc::new(context));
    Ok(witness)
}

#[expect(
    clippy::too_many_arguments,
    reason = "keep original role level, public commitments and retained material separate from geometry"
)]
fn prepare_relation_payload<F, E>(
    ring_switch_ctx: &OperationCtx<'_, F, crate::opaque::CpuBackend<F, E>>,
    fold_level: u32,
    level: &CommittedGroupParams,
    opening_batch: &akita_params::OpeningClaimsLayout,
    layout: &akita_params::RelationRhsLayout,
    commitments: &[RingVec<F>],
    materials: Vec<CpuCommitmentMaterial<F>>,
    v: &RingVec<F>,
) -> Result<PreparedRelationPayload<F>, AkitaError>
where
    F: Field + CanonicalEncoding + AkitaSerialize,
    crate::opaque::CpuBackend<F, E>: crate::opaque::CompressionComputeBackend<F>,
{
    if commitments.len() != opening_batch.num_groups()
        || materials.len() != opening_batch.num_groups()
    {
        return Err(AkitaError::Internal(format!(
            "retained relation payload group count mismatch: expected {}, actual {}",
            opening_batch.num_groups(),
            commitments.len().min(materials.len()),
        )));
    }
    let mut inner = Vec::with_capacity(materials.len());
    let mut outer = Vec::with_capacity(materials.len());
    for material in materials {
        inner.push(crate::opaque::OpaqueInnerRelationState::new(
            material.inner,
            material.source_count,
        ));
        outer.push(
            material
                .compression
                .map(crate::opaque::OpaqueCompressionState::new),
        );
    }
    let order = if level.has_preceding_groups() {
        opening_batch.root_group_order()?
    } else {
        (0..opening_batch.num_groups()).collect()
    };
    if level.payload_mode.is_compressed() {
        let sources = order
            .iter()
            .enumerate()
            .map(|(relation_group_index, &group_index)| {
                let (planned_group_index, plan) =
                    layout.group_compression_plan(relation_group_index)?;
                let commitment = commitments.get(group_index).ok_or_else(|| {
                    AkitaError::Internal(
                        "compressed relation payload public commitment is missing".into(),
                    )
                })?;
                if planned_group_index != group_index
                    || commitment.coeff_len() != plan.terminal_coefficients()
                {
                    return Err(AkitaError::InvalidInput(
                        "batched prover received a malformed compressed commitment".into(),
                    ));
                }
                let material = outer
                    .get_mut(group_index)
                    .and_then(Option::take)
                    .ok_or_else(|| {
                        AkitaError::Internal(
                            "compressed relation payload outer material is missing or consumed"
                                .into(),
                        )
                    })?
                    .into_material();
                CompressionSourceWitness::from_outer_state(
                    group_index,
                    plan,
                    material,
                    commitment.coeffs().to_vec(),
                    level.ring_relation_mode,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let (compression, report) = if level.ring_relation_mode.is_exact_coefficient_rows() {
            (
                super::compression_witness::materialize_full_ring_compression_witness(
                    ring_switch_ctx.backend(),
                    &ring_switch_ctx
                        .backend()
                        .full_ring_setup()?
                        .descriptor()
                        .level(fold_level as usize)?
                        .compression,
                    layout,
                    sources,
                    v,
                )?,
                None,
            )
        } else {
            let (compression, report) = materialize_compression_witness(
                ring_switch_ctx,
                layout,
                sources,
                v,
                level.ring_relation_mode,
            )?;
            (compression, Some(report))
        };
        let opening = compression.source(CompressionSourceId::Opening)?;
        let opening_ring_dimension = opening
            .witness()
            .plan()
            .maps()
            .last()
            .ok_or_else(|| {
                AkitaError::Internal("opening compression witness has no terminal map".into())
            })?
            .ring_dimension();
        let opening_payload = RingVec::from_coeffs_with_ring_dim(
            opening.terminal.coefficients().to_vec(),
            opening_ring_dimension,
        )?;
        let group_payloads = (0..layout.groups.len())
            .map(|relation_group_index| {
                let (group_index, _) = layout.group_compression_plan(relation_group_index)?;
                Ok(compression
                    .source(CompressionSourceId::Outer { group_index })?
                    .terminal
                    .coefficients())
            })
            .collect::<Result<Vec<_>, AkitaError>>()?;
        let relation_rhs = akita_types::assemble_compressed_relation_rhs(
            layout,
            &group_payloads,
            opening.terminal.coefficients(),
        )?;
        if let Some(report) = report {
            tracing::info!(
                sources = report.sources,
                maps = report.maps,
                batches = report.batches.len(),
                source_bytes = report.source_bytes,
                terminal_bytes = report.terminal_bytes,
                retained_bytes = report.retained_packed_witness_bytes,
                peak_scratch_bytes = report.executor_peak_scratch_bytes,
                "materialized compression witness"
            );
        }
        return Ok(PreparedRelationPayload {
            inner,
            relation_rhs,
            opening_payload,
            opening_payload_ring_dimension: opening_ring_dimension,
            compression: Some(compression),
        });
    }

    let mut coefficients = Vec::new();
    for &group_index in &order {
        let commitment = commitments.get(group_index).ok_or_else(|| {
            AkitaError::Internal("raw relation payload public commitment is missing".into())
        })?;
        let dims = level.group_role_dims_geometry(opening_batch, group_index)?;
        let params = level.group_params_geometry(opening_batch, group_index)?;
        if !commitment.can_decode_vec(dims.d_b())
            || commitment.coeff_len() / dims.d_b() != params.logical_b_rows_len()?
        {
            return Err(AkitaError::InvalidInput(
                "batched prover received a malformed raw commitment".into(),
            ));
        }
        coefficients.extend_from_slice(commitment.coeffs());
    }
    let commitment_rows = RingVec::from_coeffs(coefficients);
    Ok(PreparedRelationPayload {
        inner,
        relation_rhs: akita_types::assemble_relation_rhs(layout, v, &commitment_rows)?,
        opening_payload: v.clone(),
        opening_payload_ring_dimension: level.role_dims().d_d(),
        compression: None,
    })
}

pub(super) type GroupFoldedOpening<F> = akita_types::OpeningFamily<
    RingVec<F>,
    crate::arithmetic::coefficient_packing_fold::CoefficientPackingFoldProduct<F>,
    (),
>;
