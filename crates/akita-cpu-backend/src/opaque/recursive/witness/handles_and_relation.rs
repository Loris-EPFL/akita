use crate::sources::packed_digits::PackedSignedDigits;
use akita_error::AkitaError;
use jolt_field::{CanonicalEncoding, Field};
use std::sync::Arc;

enum OpaquePreparedGroupOpeningKind<F: Field, E: Field> {
    EvaluationTrace {
        point: akita_types::PreparedOpeningPoint<F, E>,
        folded_by_claim: Vec<akita_types::RingVec<F>>,
    },
    CoefficientPacking {
        point: akita_types::PreparedSubringCoefficientPackingPoint<E>,
        partials_by_claim: Vec<crate::opaque::SubringCoefficientPackingPartials<F>>,
    },
    FullRing {
        point: akita_types::full_ring::FullRingRawPoint<E>,
        partials: akita_types::full_ring::FullRingPartials<F>,
    },
}

/// Consumer-private prepared opening state. Its witness-derived rows never
/// appear in a protocol-facing carrier.
#[doc(hidden)]
pub struct CpuPreparedOpeningHandle<F: Field + CanonicalEncoding, E: Field> {
    binding: crate::opaque::OperationBinding,
    kind: OpaquePreparedGroupOpeningKind<F, E>,
    scalar_openings: Vec<E>,
    source: crate::opaque::openings::PreparedOpeningSource<F, E>,
    #[cfg(feature = "response-model-diagnostics")]
    source_l2_sq: Option<u128>,
}

impl<F, E> CpuPreparedOpeningHandle<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    pub(crate) const fn is_terminal_native(&self) -> bool {
        matches!(
            self.source,
            crate::opaque::openings::PreparedOpeningSource::TerminalNative
        )
    }

    pub(in crate::opaque) fn source(
        &self,
    ) -> Result<&crate::opaque::openings::RetainedOpeningSource<F, E>, AkitaError> {
        match &self.source {
            crate::opaque::openings::PreparedOpeningSource::Retained(source) => Ok(source),
            crate::opaque::openings::PreparedOpeningSource::TerminalNative => Err(
                AkitaError::InvalidInput("terminal opening has no retained proving source".into()),
            ),
        }
    }
    #[cfg(feature = "response-model-diagnostics")]
    pub(crate) fn set_source_l2_sq(&mut self, source_l2_sq: Option<u128>) {
        self.source_l2_sq = source_l2_sq;
    }
    #[cfg(feature = "response-model-diagnostics")]
    pub(crate) const fn source_l2_sq(&self) -> Option<u128> {
        self.source_l2_sq
    }
    pub(in crate::opaque) fn evaluation_trace(
        binding: crate::opaque::OperationBinding,
        source: crate::opaque::openings::PreparedOpeningSource<F, E>,
        point: akita_types::PreparedOpeningPoint<F, E>,
        folded_by_claim: Vec<akita_types::RingVec<F>>,
        scalar_openings: Vec<E>,
    ) -> Self {
        Self {
            binding,
            kind: OpaquePreparedGroupOpeningKind::EvaluationTrace {
                point,
                folded_by_claim,
            },
            scalar_openings,
            source,
            #[cfg(feature = "response-model-diagnostics")]
            source_l2_sq: None,
        }
    }

    pub(in crate::opaque) fn coefficient_packing(
        binding: crate::opaque::OperationBinding,
        source: crate::opaque::openings::PreparedOpeningSource<F, E>,
        point: akita_types::PreparedSubringCoefficientPackingPoint<E>,
        partials_by_claim: Vec<crate::opaque::SubringCoefficientPackingPartials<F>>,
        scalar_openings: Vec<E>,
    ) -> Self {
        Self {
            binding,
            kind: OpaquePreparedGroupOpeningKind::CoefficientPacking {
                point,
                partials_by_claim,
            },
            scalar_openings,
            source,
            #[cfg(feature = "response-model-diagnostics")]
            source_l2_sq: None,
        }
    }

    pub(in crate::opaque) fn full_ring(
        binding: crate::opaque::OperationBinding,
        source: crate::opaque::openings::PreparedOpeningSource<F, E>,
        point: akita_types::full_ring::FullRingRawPoint<E>,
        partials: akita_types::full_ring::FullRingPartials<F>,
        scalar_openings: Vec<E>,
    ) -> Self {
        Self {
            binding,
            source,
            scalar_openings,
            kind: OpaquePreparedGroupOpeningKind::FullRing { point, partials },
            #[cfg(feature = "response-model-diagnostics")]
            source_l2_sq: None,
        }
    }

    pub(in crate::opaque) fn into_terminal_evaluation_rows(
        self,
    ) -> Result<Vec<akita_types::RingVec<F>>, AkitaError> {
        match self.kind {
            OpaquePreparedGroupOpeningKind::EvaluationTrace {
                folded_by_claim, ..
            } => Ok(folded_by_claim),
            OpaquePreparedGroupOpeningKind::FullRing { .. }
            | OpaquePreparedGroupOpeningKind::CoefficientPacking { .. } => {
                Err(AkitaError::Internal(
                    "terminal-native opening unexpectedly uses coefficient packing".into(),
                ))
            }
        }
    }

    pub(crate) fn scalar_openings(&self) -> &[E] {
        &self.scalar_openings
    }

    pub(crate) fn operation_binding(&self) -> crate::opaque::OperationBinding {
        self.binding.clone()
    }

    pub(crate) fn full_ring_public(
        &self,
    ) -> Result<
        (
            akita_types::full_ring::FullRingRawPoint<E>,
            E,
            akita_types::FullRingSourceId,
        ),
        AkitaError,
    > {
        let OpaquePreparedGroupOpeningKind::FullRing { point, .. } = &self.kind else {
            return Err(AkitaError::InvalidInput(
                "selected relation requires full-ring opening".into(),
            ));
        };
        let [value] = self.scalar_openings.as_slice() else {
            return Err(AkitaError::InvalidInput(
                "full-ring handle has exactly one scalar row".into(),
            ));
        };
        let source_id = match self.source()? {
            crate::opaque::openings::RetainedOpeningSource::Commitment(source) => source
                .source
                .full_ring()
                .ok_or_else(|| AkitaError::InvalidInput("missing native source capability".into()))?
                .source_id(),
            crate::opaque::openings::RetainedOpeningSource::Witness(witness) => {
                witness.full_ring_source_id.ok_or_else(|| {
                    AkitaError::InvalidInput(
                        "terminal A-only witness has no recursive source role".into(),
                    )
                })?
            }
        };
        Ok((point.clone(), *value, source_id))
    }

    pub(crate) fn relation_opening<const D: usize>(
        &self,
        level: &akita_params::CommittedGroupParams,
        opening_batch: &akita_params::OpeningClaimsLayout,
        geometry: &akita_params::RelationWitnessGeometry,
        group_index: usize,
        group_dims: akita_params::CommitmentRingDims,
    ) -> Result<
        (
            crate::opaque::PreparedOpeningWitness<F>,
            crate::opaque::PublicPreparedRelationOpening<F, E>,
        ),
        AkitaError,
    >
    where
        F: CanonicalEncoding,
    {
        let group = level.group_params_geometry(opening_batch, group_index)?;
        match &self.kind {
            OpaquePreparedGroupOpeningKind::EvaluationTrace {
                point,
                folded_by_claim,
            } => {
                if group.opening_method() != akita_params::OpeningMethod::EvaluationTrace
                    || group.source_view() != akita_params::SourceOpeningView::Decoded
                    || point.ring_multiplier_point.position_len() != group.num_positions_per_block()
                    || point.ring_multiplier_point.fold_len() != group.num_live_blocks()
                    || folded_by_claim.len()
                        != opening_batch.group_layout(group_index)?.num_polynomials()
                {
                    return Err(AkitaError::InvalidInput(
                        "batched prover EvaluationTrace point layout mismatch".into(),
                    ));
                }
                let opening = crate::opaque::PreparedOpeningWitness::evaluation_trace::<D>(
                    folded_by_claim,
                    group_dims.d_a() / group_dims.d_d(),
                    group.num_digits_open(),
                    group.log_basis_open(),
                )?;
                Ok((
                    opening,
                    akita_types::OpeningFamily::EvaluationTrace(point.clone()),
                ))
            }
            OpaquePreparedGroupOpeningKind::CoefficientPacking {
                point,
                partials_by_claim,
            } => {
                if point.num_positions_per_block() != group.num_positions_per_block()
                    || point.num_live_blocks() != group.num_live_blocks()
                    || geometry.group_opening_method(group_index)? != group.opening_method()
                    || point.source_view() != group.source_view()
                    || point.source_view() != opening_batch.group_view(group_index)?
                    || point.source_num_vars() != opening_batch.group_layout(group_index)?.num_vars()
                    || point.point_num_vars() != opening_batch.group_point_num_vars(group_index)?
                {
                    return Err(AkitaError::InvalidInput(
                        "batched prover coefficient-packing point layout mismatch".into(),
                    ));
                }
                let opening = crate::opaque::PreparedOpeningWitness::coefficient_packing::<D>(
                    level,
                    opening_batch,
                    geometry,
                    group_index,
                    partials_by_claim.clone(),
                )?;
                Ok((
                    opening,
                    akita_types::OpeningFamily::SubringCoefficientPacking(point.clone()),
                ))
            }
            OpaquePreparedGroupOpeningKind::FullRing { point, partials } => {
                let view = point.view();
                if !group.opening_method().is_full_ring_scalar_extension()
                    || group.source_view() != akita_params::SourceOpeningView::Decoded
                    || view.degree() != group_dims.d_a()
                    || view.positions() != group.num_positions_per_block()
                    || view.blocks() != group.num_live_blocks()
                    || view.digits() != group.num_digits_inner()
                    || view.polynomials()
                        != opening_batch.group_layout(group_index)?.num_polynomials()
                    || group_dims.d_d() != 128
                    || D != 128
                {
                    return Err(AkitaError::InvalidInput(
                        "full-ring retained opening geometry mismatch".into(),
                    ));
                }
                let opening = crate::opaque::PreparedOpeningWitness::full_ring::<D>(
                    partials.clone(),
                    group.num_digits_open(),
                    group.log_basis_open(),
                )?;
                Ok((
                    opening,
                    akita_types::OpeningFamily::FullRingScalarExtension(point.clone()),
                ))
            }
        }
    }
}

/// D-agnostic owner for the recursive witness vector `w`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[allow(unreachable_pub)]
pub(crate) struct RecursiveWitnessFlat {
    pub(super) digits: PackedSignedDigits,
    pub(super) live_coeff_len: usize,
    pub(super) committed_coeff_len: Option<usize>,
    pub(super) commitment_ring_dim: Option<usize>,
    /// Selected full-ring A extent, distinct from stored/live Eq109 digits.
    pub(super) full_ring_extent: Option<usize>,
}

/// Opaque CPU-consumer handle for a complete recursive witness.
///
/// Protocol orchestration can transport this handle and query public shape
/// metadata, but the packed coefficient representation remains private to the
/// recursive-witness adapter.
pub struct CpuWitnessHandle {
    pub(crate) pending_successor: Option<u32>,
    pub(crate) full_ring_context: Option<Arc<dyn std::any::Any + Send + Sync>>,
    pub(crate) full_ring_source_id: Option<akita_types::FullRingSourceId>,
    pub(in crate::opaque::recursive) relation_plan:
        Option<Arc<akita_types::RelationRangeImagePlan>>,
    pub(in crate::opaque::recursive) manifest: crate::opaque::RecursiveWitnessManifest,
    pub(in crate::opaque::recursive) binding: crate::opaque::OperationBinding,
    pub(in crate::opaque::recursive) logical: RecursiveWitnessFlat,
    pub(in crate::opaque::recursive) committed: Option<RecursiveWitnessFlat>,
}

pub(crate) type OpaqueRecursiveWitness = CpuWitnessHandle;

impl CpuWitnessHandle {
    pub(crate) fn snapshot(&self) -> Self {
        Self {
            pending_successor: self.pending_successor,
            full_ring_context: self.full_ring_context.clone(),
            full_ring_source_id: self.full_ring_source_id,
            relation_plan: self.relation_plan.clone(),
            manifest: self.manifest,
            binding: self.binding.clone(),
            logical: self.logical.clone(),
            committed: self.committed.clone(),
        }
    }

    pub(crate) fn retain_full_ring_extent(&mut self) -> Result<(), AkitaError> {
        if self.committed.is_some() || self.manifest.commitment_ring_dimension() != 128 {
            return Err(AkitaError::InvalidInput(
                "full-ring source requires canonical degree-128 storage".into(),
            ));
        }
        self.logical
            .retain_full_ring_extent(self.manifest.commitment_domain_len())
    }

    pub(crate) fn validate_full_ring_extent(&self) -> Result<(), AkitaError> {
        if self.committed.is_some()
            || self.logical.full_ring_extent != Some(self.manifest.commitment_domain_len())
            || self.manifest.commitment_ring_dimension() != 128
        {
            return Err(AkitaError::InvalidInput(
                "full-ring witness extent differs from its manifest".into(),
            ));
        }
        Ok(())
    }

    pub(crate) fn full_ring_signed_coefficient(&self, index: usize) -> Result<i8, AkitaError> {
        self.validate_full_ring_extent()?;
        if self.committed.is_some() || index >= self.manifest.commitment_domain_len() {
            return Err(AkitaError::InvalidInput(
                "full-ring integer reader requires canonical in-domain witness".into(),
            ));
        }
        Ok(self.logical.packed_digits().get(index).unwrap_or(0))
    }

    pub(crate) fn full_ring_partials<F, E>(
        &self,
        plan: &crate::opaque::CommitInnerPlan,
        point: &akita_types::full_ring::FullRingRawPoint<E>,
    ) -> Result<akita_types::full_ring::FullRingPartials<F>, AkitaError>
    where
        F: Field + CanonicalEncoding,
        E: jolt_field::ExtField<F>,
    {
        self.validate_full_ring_extent()?;
        crate::opaque::full_ring::source_partials(std::slice::from_ref(&self.logical), plan, point)
    }

    #[cfg(feature = "response-model-diagnostics")]
    pub(crate) fn source_l2_sq<F: Field>(&self) -> Option<u128> {
        crate::opaque::RootPolyMeta::<F>::exact_integer_coeff_l2_sq(
            self.committed.as_ref().unwrap_or(&self.logical),
        )
    }

    pub(crate) fn operation_binding(&self) -> crate::opaque::OperationBinding {
        self.binding.clone()
    }

    pub(crate) fn set_operation_binding(&mut self, binding: crate::opaque::OperationBinding) {
        self.binding = binding;
    }

    pub(crate) fn initialize_relation_plan<F>(
        &mut self,
        relation: &akita_types::RingRelationInstance<F>,
        level: &akita_params::CommittedGroupParams,
    ) -> Result<(), AkitaError>
    where
        F: Field + CanonicalEncoding,
    {
        if self.relation_plan.is_some() {
            return Err(AkitaError::Internal(
                "recursive witness relation plan is already initialized".into(),
            ));
        }
        let opening_batch = relation.opening_batch();
        let witness_layout = relation.segment_layout(level, None)?;
        if witness_layout.live_coeff_len() != self.manifest.logical_len() {
            return Err(AkitaError::Internal(
                "recursive witness manifest disagrees with its relation instance".into(),
            ));
        }
        let geometry = level.relation_address_geometry(
            opening_batch,
            relation.extension_degree(),
            self.manifest.commitment_ring_dimension(),
            witness_layout.live_coeff_len(),
        )?;
        let digit_range = akita_params::DigitRangePlan::new(
            akita_error::checked::pow2(level.open().digits.log_basis as usize).ok_or_else(
                || AkitaError::Internal("validated relation digit range basis overflow".into()),
            )?,
        )?;
        self.relation_plan = Some(Arc::new(akita_types::RelationRangeImagePlan::new(
            akita_params::RelationWitnessGeometry::for_level(
                level,
                opening_batch,
                relation.extension_degree(),
            )?,
            geometry,
            digit_range,
            witness_layout,
            opening_batch,
        )?));
        Ok(())
    }
}

/// Consumer-owned state retained between recursive opening preparation and EOR.
pub(crate) struct CpuWitnessOpeningHandle<E: Field> {
    pub(super) source_operation: u128,
    pub(super) point: Vec<E>,
    pub(super) tensor_evals: Vec<E>,
    pub(super) witness_len: usize,
    pub(super) ring_dimension: usize,
}

pub struct CpuRelationHandle {
    pub(crate) relation_plan: Option<Arc<akita_types::RelationRangeImagePlan>>,
    pub(super) binding: crate::opaque::OperationBinding,
    pub(super) packed: PackedSignedDigits,
}

pub(crate) type OpaqueWitnessOpeningState<E> = CpuWitnessOpeningHandle<E>;
pub(crate) type ConsumerRelationWitness = CpuRelationHandle;
macro_rules! impl_bound_handle {
    ($handle:ident $(<$field:ident>)?) => {
        impl$(<$field: Field>)? $handle$(<$field>)? {
            pub(crate) fn operation_binding(
                &self,
            ) -> crate::opaque::OperationBinding {
                self.binding.clone()
            }

            pub(crate) fn set_operation_binding(
                &mut self,
                binding: crate::opaque::OperationBinding,
            ) {
                self.binding = binding;
            }
        }
    };
}

impl_bound_handle!(CpuRelationHandle);
impl CpuRelationHandle {
    pub(crate) fn len(&self) -> usize {
        self.packed.len()
    }
}

impl<F, B>
    crate::opaque::consumer_kernels::RecursiveRelationWitnessKernel<OpaqueRecursiveWitness, F> for B
where
    F: Field + CanonicalEncoding,
    B: crate::opaque::ComputeBackendSetup<F>,
{
    type RelationWitness = ConsumerRelationWitness;

    fn prepare_relation_witness(
        &self,
        prepared: Option<&Self::PreparedSetup>,
        witness: &OpaqueRecursiveWitness,
        plan: &crate::opaque::ValidatedRelationWitnessPlan,
    ) -> Result<crate::opaque::PreparedRelationWitness<Self::RelationWitness>, AkitaError> {
        prepared.ok_or_else(|| {
            AkitaError::Internal("relation witness preparation requires prepared setup".into())
        })?;
        if witness.logical.live_coeff_len() != plan.witness_len() {
            return Err(AkitaError::InvalidInput(
                "relation witness plan has a different operation context".into(),
            ));
        }
        let (packed, column_bits, coefficient_bits) = crate::opaque::build_w_evals_compact(
            witness.logical.packed_digits().clone(),
            plan.coefficient_count(),
            plan.extension_degree(),
            plan.opening_source_len(),
        )?;
        Ok(crate::opaque::PreparedRelationWitness::new(
            ConsumerRelationWitness {
                relation_plan: witness.relation_plan.clone(),
                binding: witness.binding.for_operation(0),
                packed,
            },
            column_bits,
            coefficient_bits,
        ))
    }
}

impl<F: Field> akita_types::full_ring::FullRingCoefficientSource<F> for CpuWitnessHandle {
    fn len(&self) -> usize {
        self.manifest.commitment_domain_len()
    }
    fn coefficient(&self, index: usize) -> Result<F, AkitaError> {
        let digit = self.full_ring_signed_coefficient(index)?;
        Ok(if digit < 0 {
            -F::from_u64(u64::from(digit.unsigned_abs()))
        } else {
            F::from_u64(digit as u64)
        })
    }
}
