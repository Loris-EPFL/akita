//! Reusable source and commitment ownership. No proof state lives in these objects.
use super::source::{
    prepare_and_evaluate_opening_group, scalar_opening_from_folded_ring,
    PreparedExtensionOpeningGroup,
};
use crate::arithmetic::extension_opening_reduction::{
    tensor_column_partials_from_base_evals, tensor_packed_witness_evals,
};
use crate::commitment::CommitmentSource;
use crate::opaque::CpuPreparedOpeningHandle;
use crate::opaque::*;
use crate::sources::poly::SourceCoefficients;
use akita_error::AkitaError;
use akita_params::*;
use akita_serialization::AkitaSerialize;
use akita_types::*;
use jolt_field::{CanonicalEncoding, ExtField, Field, Fold, MulBaseUnreduced, Ring, Unreduced};
use std::sync::Arc;

/// An immutable imported source owned by one backend.
pub struct SourceHandle<F: Field + CanonicalEncoding, E: Field> {
    pub(super) owner: u64,
    pub(super) storage: Arc<dyn PrivateSource<F, E>>,
    pub(super) metadata: SourceMetadata,
}
impl<F, E> Clone for SourceHandle<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    fn clone(&self) -> Self {
        Self {
            owner: self.owner,
            storage: self.storage.clone(),
            metadata: self.metadata,
        }
    }
}
/// Borrowed canonical coefficients through a retained Dense component's committed extent.
/// Genuine plan padding is included; storage beyond the plan is excluded.
pub struct CpuDecodedDenseComponentReader<'a, F: Field> {
    coefficients: &'a [F],
    num_vars: usize,
    column: usize,
    commitment_id: u128,
    backend_id: u64,
    setup_digest: [u8; 32],
}

impl<F: Field> akita_types::full_ring::FullRingCoefficientSource<F>
    for CpuDecodedDenseComponentReader<'_, F>
{
    fn len(&self) -> usize { self.coefficients.len() }
    fn coefficient(&self, index: usize) -> Result<F, AkitaError> {
        self.coefficients.get(index).copied().ok_or_else(|| {
            AkitaError::InvalidInput("Dense component coefficient index out of range".into())
        })
    }
}

impl<F: Field> CpuDecodedDenseComponentReader<'_, F> {
    pub fn column(&self) -> usize { self.column }
    pub fn num_vars(&self) -> usize { self.num_vars }
    /// Actual retained commitment identity, scoped to the validated backend.
    /// This is not an installed native source ID or a public statement digest.
    pub fn commitment_id(&self) -> u128 { self.commitment_id }
    pub fn backend_id(&self) -> u64 { self.backend_id }
    pub fn setup_digest(&self) -> [u8; 32] { self.setup_digest }
}

impl<F: Field + CanonicalEncoding, E: Field> CpuBackend<F, E> {
    /// Borrow actual Dense coefficients, rejecting non-Dense producer recipes.
    pub fn decoded_dense_component_reader<'a>(
        &self,
        handle: &'a CommitmentHandle<F, E>,
        column: usize,
    ) -> Result<CpuDecodedDenseComponentReader<'a, F>, AkitaError> {
        let committed = &handle.committed;
        if handle.owner != self.owner().backend_id()
            || column >= committed.metadata.num_polynomials()
            || committed.metadata != SourceMetadata::try_new(
                committed.parameters.group.num_polynomials(), committed.parameters.group.num_vars(),
            )?
            || committed.producer_contract.class() != sis::CommittedSourceClass::BalancedSignedDigit
        {
            return Err(AkitaError::InvalidInput("Dense component owner, source geometry or producer mismatch".into()));
        }
        committed.parameters.validate_frozen_precommit(F::MODULUS_BITS)?;
        if committed.producer_contract.decomposition().field_bits() != F::MODULUS_BITS {
            return Err(AkitaError::InvalidInput("Dense component contract field mismatch".into()));
        }
        let plan = CommitInnerPlan::from_profile(&committed.parameters);
        let coefficients = committed.source.borrowed_dense_coefficients(&plan, column)?;
        let logical_len = akita_error::checked::pow2(committed.metadata.num_vars())
            .ok_or_else(|| AkitaError::InvalidInput("Dense component logical extent overflow".into()))?;
        let physical_len = akita_error::checked::product([
            plan.num_live_blocks, plan.num_positions_per_block, plan.ring_dimension,
        ]).ok_or_else(|| AkitaError::InvalidInput("Dense component physical extent overflow".into()))?;
        if coefficients.len() < logical_len || coefficients.len() > physical_len {
            return Err(AkitaError::InvalidInput("Dense component physical coefficient extent mismatch".into()));
        }
        Ok(CpuDecodedDenseComponentReader {
            coefficients, num_vars: committed.metadata.num_vars(), column,
            commitment_id: committed.commitment_id,
            backend_id: handle.owner,
            setup_digest: self.owner().setup_digest(),
        })
    }
}

/// A reusable commitment retaining the exact source and parameters it
/// committed, and the producer contract that admitted the source.
pub struct CommitmentHandle<F: Field + CanonicalEncoding, E: Field> {
    pub(super) owner: u64,
    pub(super) committed: Arc<CommittedSource<F, E>>,
}
impl<F, E> Clone for CommitmentHandle<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    fn clone(&self) -> Self {
        Self {
            owner: self.owner,
            committed: self.committed.clone(),
        }
    }
}
pub(super) struct CommittedSource<F: Field + CanonicalEncoding, E: Field> {
    pub(super) source: Arc<dyn PrivateSource<F, E>>,
    pub(super) metadata: SourceMetadata,
    pub(super) commitment_id: u128,
    pub(super) parameters: GroupCommitPhaseParams,
    pub(super) producer_contract: sis::CommittedSourceContract,
    pub(super) public: Commitment<F>,
    pub(super) retained: crate::commitment::PortableCommitmentHandle<F>,
}
/// Public commitment and the reusable private handle used for proving.
pub struct CommitOutput<F: Field + CanonicalEncoding, E: Field> {
    pub committed_group: CommittedGroup<F>,
    pub private_handle: CommitmentHandle<F, E>,
}
impl<F, E> CommitmentHandleMetadata for CommitmentHandle<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    fn metadata(&self) -> SourceMetadata {
        self.committed.metadata
    }
    fn producer_contract(&self) -> sis::CommittedSourceContract {
        self.committed.producer_contract
    }
}

// This object-safe interface never leaves the CPU crate. It retains one whole
// homogeneous group while allowing different representations in one proof.
pub(super) trait PrivateSource<F: Field + CanonicalEncoding, E: Field>: Send + Sync {
    fn commitment_sources(&self) -> Vec<&dyn CommitmentSource<F>>;
    fn legacy(&self) -> Option<&dyn LegacyPrivateSource<F, E>>;
    fn full_ring(&self) -> Option<&dyn super::full_ring::FullRingPrivateSource<F, E>>;
    fn borrowed_dense_coefficients(
        &self,
        _plan: &CommitInnerPlan,
        _column: usize,
    ) -> Result<&[F], AkitaError> {
        Err(AkitaError::InvalidInput("source lacks borrowed canonical Dense coefficients".into()))
    }
    fn original_planes(&self) -> Option<&dyn super::ordinary_projection::OrdinaryBaseSource<F>> {
        None
    }
    /// Opening authority is independent of tensor/EOR and source-table access.
    fn ordinary_opening(
        &self,
        backend: &CpuBackend<F, E>,
        binding: &OperationBinding,
        plan: &ValidatedRecursiveGroupOpeningPlan<'_, E>,
        source: crate::opaque::openings::PreparedOpeningSource<F, E>,
    ) -> Result<PreparedGroupOpening<E, CpuPreparedOpeningHandle<F, E>>, AkitaError> {
        if plan.source_view() != SourceOpeningView::Decoded {
            return Err(AkitaError::InvalidInput("legacy sources reject original RAM opening recipes".into()));
        }
        self.legacy().ok_or_else(|| {
            AkitaError::InvalidInput("source has no ordinary opening capability".into())
        })?.opening(backend, binding, plan, source)
    }
    fn probe(
        &self,
        backend: &CpuBackend<F, E>,
        plan: &ValidatedFoldProbePlan<'_>,
    ) -> Result<FoldProbeOutcome<super::CpuAcceptedFoldHandle<F>>, AkitaError>;
    #[cfg(feature = "response-model-diagnostics")]
    fn source_l2_sq(&self) -> Option<u128>;
}

/// Actual ring-subfield/EOR capability, separate from source ownership.
pub(super) trait LegacyPrivateSource<F: Field + CanonicalEncoding, E: Field>:
    Send + Sync
{
    fn opening(
        &self,
        backend: &CpuBackend<F, E>,
        binding: &OperationBinding,
        plan: &ValidatedRecursiveGroupOpeningPlan<'_, E>,
        source: crate::opaque::openings::PreparedOpeningSource<F, E>,
    ) -> Result<PreparedGroupOpening<E, CpuPreparedOpeningHandle<F, E>>, AkitaError>;
    fn extension(
        &self,
        backend: &CpuBackend<F, E>,
        ring_dimension: usize,
        point: &[E],
    ) -> Result<PreparedExtensionOpeningGroup<E>, AkitaError>;
    #[allow(clippy::too_many_arguments)]
    fn begin_eor(
        &self,
        backend: &CpuBackend<F, E>,
        ring_dimension: usize,
        coefficients: &[E],
        tail: &[E],
        eta: &[E],
        extra: Vec<E>,
        claim: E,
    ) -> Result<Box<dyn crate::opaque::eor::ExtensionOpeningSession<E>>, AkitaError>;
}
pub(super) struct OwnedPolynomials<P> {
    pub(super) polynomials: Vec<P>,
}

impl<F, E, P> LegacyPrivateSource<F, E> for OwnedPolynomials<P>
where
    F: Field + CanonicalEncoding + AkitaSerialize + Ring + Unreduced + 'static,
    F::Wide: From<F> + jolt_field::AdditiveGroup,
    E: ExtField<F>
        + FpExtEncoding<F>
        + MulBaseUnreduced<F>
        + Unreduced
        + Fold
        + AkitaSerialize
        + 'static,
    P: SourceCoefficients<F>
        + RuntimeRootProvePoly<F>
        + CommitmentSource<F>
        + Send
        + Sync
        + 'static,
    CpuBackend<F, E>: ComputeBackendSetup<F, PreparedSetup = CpuPreparedSetup<F>>
        + FoldHandleBackend<F, AcceptedFold = super::CpuAcceptedFoldHandle<F>>
        + RuntimeOpeningProveBackendFor<F, P>
        + RuntimeCoefficientPackingBackendFor<F, P, E>,
{
    fn opening(
        &self,
        backend: &CpuBackend<F, E>,
        binding: &OperationBinding,
        plan: &ValidatedRecursiveGroupOpeningPlan<'_, E>,
        source: crate::opaque::openings::PreparedOpeningSource<F, E>,
    ) -> Result<PreparedGroupOpening<E, CpuPreparedOpeningHandle<F, E>>, AkitaError> {
        if plan.source_view() != SourceOpeningView::Decoded {
            return Err(AkitaError::InvalidInput("decoded sources reject original RAM opening recipes".into()));
        }
        let polys = self.polynomials.iter().collect::<Vec<_>>();
        dispatch_for_field!(
            ProtocolDispatchSlot::Role(RingRole::Inner),
            F,
            plan.ring_dimension(),
            |D| {
                let prepared = Some(backend.prepared()?);
                if let OpeningMethod::SubringCoefficientPacking {
                    challenge_subring_dimension,
                } = plan.opening_method()
                {
                    let geometry = SubringCoefficientPackingGeometry::try_new(
                        E::DEGREE,
                        D,
                        challenge_subring_dimension,
                    )?;
                    let first = polys.first().ok_or_else(|| {
                        AkitaError::Internal(
                            "admitted coefficient packing source has no polynomial".into(),
                        )
                    })?;
                    let live = <P as RootPolyShape<F, D>>::num_live_ring_elems(*first);
                    let vars = RootPolyMeta::<F>::num_vars(*first);
                    if polys.iter().any(|p| {
                        <P as RootPolyShape<F, D>>::num_live_ring_elems(*p) != live
                            || RootPolyMeta::<F>::num_vars(*p) != vars
                    }) || live.div_ceil(plan.positions_per_block()) != plan.live_blocks()
                    {
                        return Err(AkitaError::InvalidInput(
                            "source and opening geometry disagree".into(),
                        ));
                    }
                    let point = PreparedSubringCoefficientPackingPoint::new(
                        geometry,
                        plan.basis(),
                        live,
                        plan.positions_per_block(),
                        vars,
                        plan.point(),
                    )?;
                    let batch = <P as RootOpeningSource<F, D>>::opening_batch(&polys)?;
                    let partials =
                        SubringCoefficientPackingBatchKernel::coefficient_packing_partials_batch(
                            backend,
                            prepared,
                            batch,
                            SubringCoefficientPackingPlan { point: &point },
                        )?;
                    let openings = partials
                        .iter()
                        .map(|p| {
                            crate::arithmetic::coefficient_packing_fold::coefficient_packing_scalar_opening::<F, E>(
                                geometry,
                                point.num_live_blocks(),
                                std::slice::from_ref(p),
                                &[E::one()],
                                point.live_block_weights(),
                                point.tail_weights(),
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    return Ok(crate::opaque::prepared_opening::coefficient_packing(
                        binding.clone(),
                        source,
                        point,
                        partials,
                        openings,
                    ));
                }
                let (point, (folded, by_claim)) =
                    prepare_and_evaluate_opening_group::<F, E, P, CpuBackend<F, E>, D>(
                        backend,
                        prepared,
                        &polys,
                        plan.point(),
                        plan.basis(),
                        plan.positions_per_block(),
                        plan.live_blocks(),
                        plan.alpha_bits(),
                    )?;
                let inner = &plan.point()[..plan.point().len().min(plan.alpha_bits())];
                let openings = folded
                    .iter()
                    .map(|row| {
                        scalar_opening_from_folded_ring::<F, E, D>(row, &point, inner, plan.basis())
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(crate::opaque::prepared_opening::evaluation_trace(
                    binding.clone(),
                    source,
                    point,
                    by_claim
                        .iter()
                        .map(|row| RingVec::from_ring_elems(row).into_compact())
                        .collect(),
                    openings,
                ))
            }
        )
    }
    fn extension(
        &self,
        _backend: &CpuBackend<F, E>,
        _ring_dimension: usize,
        point: &[E],
    ) -> Result<PreparedExtensionOpeningGroup<E>, AkitaError> {
        let mut openings = Vec::with_capacity(self.polynomials.len());
        let mut proof_partials = Vec::new();
        let mut rows = Vec::with_capacity(self.polynomials.len());
        for poly in &self.polynomials {
            let coefficients = poly.source_coefficients()?;
            let partials = tensor_column_partials_from_base_evals::<F, E>(
                RootPolyMeta::<F>::num_vars(poly),
                &coefficients,
                point,
            )?;
            openings.push(derive_tensor_extension_opening_claim_from_partials::<F, E>(
                point, &partials,
            )?);
            rows.push(tensor_row_partials_from_columns::<F, E>(&partials)?);
            proof_partials.extend(partials);
        }
        Ok(PreparedExtensionOpeningGroup {
            openings,
            proof_partials,
            row_partials_by_claim: rows,
        })
    }
    fn begin_eor(
        &self,
        _backend: &CpuBackend<F, E>,
        _ring_dimension: usize,
        coefficients: &[E],
        tail: &[E],
        eta: &[E],
        extra: Vec<E>,
        claim: E,
    ) -> Result<Box<dyn crate::opaque::eor::ExtensionOpeningSession<E>>, AkitaError> {
        let witnesses = self
            .polynomials
            .iter()
            .map(|poly| {
                tensor_packed_witness_evals::<F, E>(
                    RootPolyMeta::<F>::num_vars(poly),
                    &poly.source_coefficients()?,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        crate::opaque::cpu_extension_opening_session_from_witnesses::<F, E>(
            witnesses,
            coefficients,
            tail,
            eta,
            extra,
            claim,
        )
    }
}

impl<F, E, P> PrivateSource<F, E> for OwnedPolynomials<P>
where
    F: Field + CanonicalEncoding + AkitaSerialize + Ring + Unreduced + 'static,
    F::Wide: From<F> + jolt_field::AdditiveGroup,
    E: ExtField<F>
        + FpExtEncoding<F>
        + MulBaseUnreduced<F>
        + Unreduced
        + Fold
        + AkitaSerialize
        + 'static,
    P: SourceCoefficients<F>
        + RuntimeRootProvePoly<F>
        + CommitmentSource<F>
        + Send
        + Sync
        + 'static,
    CpuBackend<F, E>: ComputeBackendSetup<F, PreparedSetup = CpuPreparedSetup<F>>
        + FoldHandleBackend<F, AcceptedFold = super::CpuAcceptedFoldHandle<F>>
        + RuntimeOpeningProveBackendFor<F, P>
        + RuntimeCoefficientPackingBackendFor<F, P, E>,
{
    fn legacy(&self) -> Option<&dyn LegacyPrivateSource<F, E>> {
        Some(self)
    }
    fn full_ring(&self) -> Option<&dyn super::full_ring::FullRingPrivateSource<F, E>> {
        None
    }
    #[cfg(feature = "response-model-diagnostics")]
    fn source_l2_sq(&self) -> Option<u128> {
        self.polynomials.iter().try_fold(0u128, |sum, p| {
            sum.checked_add(RootPolyMeta::<F>::exact_integer_coeff_l2_sq(p)?)
        })
    }
    fn commitment_sources(&self) -> Vec<&dyn CommitmentSource<F>> {
        self.polynomials
            .iter()
            .map(|p| p as &dyn CommitmentSource<F>)
            .collect()
    }
    fn borrowed_dense_coefficients(
        &self,
        plan: &CommitInnerPlan,
        column: usize,
    ) -> Result<&[F], AkitaError> {
        let polynomial = self.polynomials.get(column).ok_or_else(|| {
            AkitaError::InvalidInput("Dense source column out of range".into())
        })?;
        if polynomial.descriptor()?.class() != crate::commitment::CommitSourceClass::Dense {
            return Err(AkitaError::InvalidInput("canonical Dense reader rejects another producer class".into()));
        }
        let offerings = polynomial.available_polynomial_types(plan)?;
        let index = offerings.as_slice().iter().position(|&kind| {
            kind == crate::commitment::PolynomialType::Dense(crate::commitment::DenseType::Coefficients)
        }).ok_or_else(|| AkitaError::InvalidInput("Dense source does not offer borrowed coefficients".into()))?;
        match polynomial.represent_as(offerings.select(index)?, plan)? {
            crate::commitment::PolynomialRepresentation::Dense(
                crate::commitment::DenseRepresentation::Coefficients(coefficients),
            ) => {
                let physical_len = akita_error::checked::product([
                    plan.num_live_blocks, plan.num_positions_per_block, plan.ring_dimension,
                ]).ok_or_else(|| AkitaError::InvalidInput("Dense component physical extent overflow".into()))?;
                coefficients.coefficients().get(..physical_len).ok_or_else(|| {
                    AkitaError::InvalidInput("Dense component physical coefficient extent mismatch".into())
                })
            }
            _ => Err(AkitaError::InvalidInput("Dense source returned a different representation".into())),
        }
    }
    fn probe(
        &self,
        backend: &CpuBackend<F, E>,
        plan: &ValidatedFoldProbePlan<'_>,
    ) -> Result<FoldProbeOutcome<super::CpuAcceptedFoldHandle<F>>, AkitaError> {
        let polys = self.polynomials.iter().collect::<Vec<_>>();
        dispatch_for_field!(
            ProtocolDispatchSlot::Role(RingRole::Inner),
            F,
            plan.ring_dimension(),
            |D| {
                let batch = <P as RootOpeningSource<F, D>>::opening_batch(&polys)?;
                FoldResponseKernel::probe(backend, Some(backend.prepared()?), batch, plan)
            }
        )
    }
}

/// CPU-supported owned source representation. Import consumes the entire group.
#[allow(private_bounds)]
pub trait CpuSource<F: Field + CanonicalEncoding, E: Field>: SourceImport<F, E> {}
trait SourceImport<F: Field + CanonicalEncoding, E: Field>:
    CommitmentSource<F> + Sized + 'static
{
    fn retain_owned(polynomials: Vec<Self>) -> Arc<dyn PrivateSource<F, E>>;
}
impl<F, E, P> SourceImport<F, E> for P
where
    F: Field + CanonicalEncoding + AkitaSerialize + Ring + Unreduced + 'static,
    F::Wide: From<F> + jolt_field::AdditiveGroup,
    E: ExtField<F>
        + FpExtEncoding<F>
        + MulBaseUnreduced<F>
        + Unreduced
        + Fold
        + AkitaSerialize
        + 'static,
    P: SourceCoefficients<F>
        + RuntimeRootProvePoly<F>
        + CommitmentSource<F>
        + Send
        + Sync
        + 'static,
    CpuBackend<F, E>: ComputeBackendSetup<F, PreparedSetup = CpuPreparedSetup<F>>
        + FoldHandleBackend<F, AcceptedFold = super::CpuAcceptedFoldHandle<F>>
        + RuntimeOpeningProveBackendFor<F, P>
        + RuntimeCoefficientPackingBackendFor<F, P, E>,
{
    fn retain_owned(polynomials: Vec<Self>) -> Arc<dyn PrivateSource<F, E>> {
        Arc::new(OwnedPolynomials { polynomials })
    }
}
impl<F, E, P> CpuSource<F, E> for P
where
    F: Field + CanonicalEncoding,
    E: Field,
    P: SourceImport<F, E>,
{
}

impl<F, E> CpuBackend<F, E>
where
    F: Field + CanonicalEncoding + AkitaSerialize + Ring + Unreduced + 'static,
    F::Wide: From<F> + jolt_field::AdditiveGroup,
    E: ExtField<F>
        + FpExtEncoding<F>
        + MulBaseUnreduced<F>
        + Unreduced
        + Fold
        + AkitaSerialize
        + 'static,
{
    /// Consume a homogeneous source group without exposing its storage to proving.
    pub fn import_source<P>(&self, polynomials: Vec<P>) -> Result<SourceHandle<F, E>, AkitaError>
    where
        P: CpuSource<F, E>,
    {
        let prepared = self.prepared()?;
        let layout =
            crate::commitment::resolve_polynomial_group_layout(&polynomials, prepared.expanded()?)?;
        let metadata = SourceMetadata::try_new(layout.num_polynomials(), layout.num_vars())?;
        Ok(SourceHandle {
            owner: self.owner().backend_id(),
            storage: P::retain_owned(polynomials),
            metadata,
        })
    }
}

impl<F, E> core::fmt::Debug for SourceHandle<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SourceHandle")
            .field("metadata", &self.metadata)
            .finish_non_exhaustive()
    }
}
impl<F, E> core::fmt::Debug for CommitmentHandle<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CommitmentHandle")
            .field("metadata", &self.committed.metadata)
            .field("producer_contract", &self.committed.producer_contract)
            .finish_non_exhaustive()
    }
}
impl<F, E> core::fmt::Debug for CommitOutput<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CommitOutput")
            .field("private_handle", &self.private_handle)
            .finish_non_exhaustive()
    }
}

#[cfg(all(test, feature = "response-model-diagnostics"))]
impl<F: Field, E> CpuBackend<F, E> {
    /// Source energy of a witness, checked against the witness's proof binding.
    pub(crate) fn witness_source_l2_sq(
        &self,
        witness: &crate::opaque::CpuWitnessHandle,
    ) -> Result<Option<u128>, AkitaError> {
        self.validate_binding(&witness.operation_binding())?;
        Ok(witness.source_l2_sq::<F>())
    }
}

impl<F: Field + CanonicalEncoding> SourceCoefficients<F> for crate::DensePoly<F> {
    fn source_coefficients(&self) -> Result<std::borrow::Cow<'_, [F]>, AkitaError> {
        let len =
            akita_error::checked::pow2(RootPolyMeta::<F>::num_vars(self)).ok_or_else(|| {
                AkitaError::Internal("dense source coefficient extent overflow".into())
            })?;
        Ok(std::borrow::Cow::Borrowed(
            self.field_coeffs().get(..len).ok_or_else(|| {
                AkitaError::Internal(
                    "dense source coefficients do not cover the declared arity".into(),
                )
            })?,
        ))
    }
}
impl<F: Field + CanonicalEncoding, I: crate::OneHotIndex> SourceCoefficients<F>
    for crate::OneHotPoly<F, I>
{
    fn source_coefficients(&self) -> Result<std::borrow::Cow<'_, [F]>, AkitaError> {
        let len = akita_error::checked::product([self.onehot_k(), self.indices().len()])
            .ok_or_else(|| {
                AkitaError::Internal("one-hot source coefficient extent overflow".into())
            })?;
        let mut values = vec![F::zero(); len];
        for (chunk, index) in self.indices().iter().enumerate() {
            if let Some(index) = index {
                values[chunk * self.onehot_k() + index.as_usize()] = F::one();
            }
        }
        Ok(std::borrow::Cow::Owned(values))
    }
}

#[cfg(test)]
mod component_reader_tests {
    use super::*;
    use jolt_field::{Prime128OffsetA7F7 as K, Zero};

    #[test]
    fn retained_dense_coefficients_preserve_source_values_and_committed_padding() {
        let rings = (0..8).map(|ring| {
            akita_algebra::CyclotomicRing::<K, 128>::from_coefficients(core::array::from_fn(|coefficient| {
                if (ring + coefficient) % 2 == 0 {
                    K::from_u64((ring * 128 + coefficient + 1) as u64)
                } else {
                    -K::from_u64((ring * 128 + coefficient + 1) as u64)
                }
            }))
        }).collect();
        let dense = crate::DensePoly::<K>::from_ring_coeffs(rings).unwrap();
        let owner = OwnedPolynomials { polynomials: vec![dense] };
        let plan = CommitInnerPlan {
            ring_dimension: 128, num_live_blocks: 1, n_a: 1,
            num_positions_per_block: 8, num_digits_inner: 26, log_basis_inner: 5,
        };
        for positions in [8, 16] {
            let plan = CommitInnerPlan { num_positions_per_block: positions, ..plan };
            let coefficients = <OwnedPolynomials<crate::DensePoly<K>> as PrivateSource<K, K>>::borrowed_dense_coefficients(
                &owner, &plan, 0,
            ).unwrap();
            for index in 0..1024 {
                let ring = index / 128;
                let coefficient = index % 128;
                let magnitude = K::from_u64(index as u64 + 1);
                assert_eq!(coefficients.get(index).copied().unwrap(),
                    if (ring + coefficient) % 2 == 0 { magnitude } else { -magnitude });
            }
            for index in 1024..positions * 128 {
                assert_eq!(coefficients.get(index).copied().unwrap(), K::zero());
            }
            assert!(coefficients.get(positions * 128).is_none());
        }
        assert!(<OwnedPolynomials<crate::DensePoly<K>> as PrivateSource<K, K>>::borrowed_dense_coefficients(
            &owner, &plan, 1,
        ).is_err());
        for positions in [1 << 20, 1usize << (usize::BITS - 1)] {
            let oversized = CommitInnerPlan { num_positions_per_block: positions, ..plan };
            assert!(<OwnedPolynomials<crate::DensePoly<K>> as PrivateSource<K, K>>::borrowed_dense_coefficients(
                &owner, &oversized, 0,
            ).is_err());
        }
        let onehot = OwnedPolynomials {
            polynomials: vec![crate::OneHotPoly::<K>::new(128, vec![Some(0); 2]).unwrap()],
        };
        assert!(<OwnedPolynomials<crate::OneHotPoly<K>> as PrivateSource<K, K>>::borrowed_dense_coefficients(
            &onehot, &plan, 0,
        ).is_err());
    }
}
