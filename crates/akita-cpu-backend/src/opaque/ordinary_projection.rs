//! Ordinary native128 authority over the same immutable full-ring source owner.
//! The K-valued projection carries no tensor/EOR or E-origin query capability.
use super::full_ring::{FullRingOwned, FullRingPrivateSource};
use super::openings::PreparedOpeningSource;
use super::owned::{CommitmentHandle, CommittedSource, LegacyPrivateSource, PrivateSource};
use super::*;
use crate::commitment::{CommitmentExecutor, CommitmentSource, PortableStatePolicy};
use crate::sources::full_ring::FullRingSource;
use akita_params::*;
use akita_serialization::AkitaSerialize;
use akita_types::*;
use jolt_field::{ExtField, MulBaseUnreduced, Ring, Unreduced, WithCommitAccumulator};
use std::sync::Arc;

/// A deliberately smaller capability than the legacy tensor/EOR bundle.
pub(super) trait OrdinaryBaseSource<F: Field + CanonicalEncoding>: Send + Sync {
    fn validate_original_ram(
        &self,
        installed: &InstalledFullRingSetup<F>,
        profile: GroupCommitPhaseParams,
        contract: sis::CommittedSourceContract,
    ) -> Result<(), AkitaError>;
    fn validate_original_main(
        &self,
        installed: &InstalledFullRingSetup<F>,
        profile: GroupCommitPhaseParams,
        contract: sis::CommittedSourceContract,
        column: usize,
    ) -> Result<(), AkitaError>;
    fn original_coefficient(&self, plan: &CommitInnerPlan, index: usize) -> Result<F, AkitaError>;
    fn original_source_id(&self) -> FullRingSourceId;
    fn opening(
        &self,
        backend: &CpuBackend<F, F>,
        binding: &OperationBinding,
        plan: &ValidatedRecursiveGroupOpeningPlan<'_, F>,
        source: PreparedOpeningSource<F, F>,
    ) -> Result<PreparedGroupOpening<F, CpuPreparedOpeningHandle<F, F>>, AkitaError>;
    fn probe(
        &self,
        backend: &CpuBackend<F, F>,
        plan: &ValidatedFoldProbePlan<'_>,
    ) -> Result<FoldProbeOutcome<CpuAcceptedFoldHandle<F>>, AkitaError>;
}

fn native_packing(method: OpeningMethod, dimension: usize) -> Result<(), AkitaError> {
    if dimension != 128 || method != (OpeningMethod::SubringCoefficientPacking {
        challenge_subring_dimension: 128,
    }) {
        return Err(AkitaError::InvalidInput("original source requires ordinary native k1h1 packing".into()));
    }
    Ok(())
}

impl<F, P> OrdinaryBaseSource<F> for FullRingOwned<F, P>
where
    F: Field + CanonicalEncoding + ExtField<F> + FpExtEncoding<F> + MulBaseUnreduced<F>,
    P: FullRingSource<F> + RootPolyMeta<F> + RootOpeningSource<F, 128>,
    CpuBackend<F, F>: ComputeBackendSetup<F, PreparedSetup = CpuPreparedSetup<F>>
        + FoldHandleBackend<F, AcceptedFold = CpuAcceptedFoldHandle<F>>
        + for<'a> FoldResponseKernel<<P as RootOpeningSource<F, 128>>::OpeningBatchView<'a>, F, 128>
        + for<'a> SubringCoefficientPackingBatchKernel<<P as RootOpeningSource<F, 128>>::OpeningBatchView<'a>, F, F, 128>,
{
    fn validate_original_ram(
        &self,
        installed: &InstalledFullRingSetup<F>,
        profile: GroupCommitPhaseParams,
        contract: sis::CommittedSourceContract,
    ) -> Result<(), AkitaError> {
        validate_original_ram_profile::<F>(profile, contract)?;
        let source = installed.descriptor().source(self.source_id)?;
        if installed.descriptor() != self.setup.descriptor()
            || self.raw_view != source.raw_view
            || self.raw_view.permutation().is_some()
            || self.raw_view.polynomials() != 1
            || self.raw_view.degree() != 128
            || self.raw_view.blocks() != 16
            || self.raw_view.positions() != 32
            || self.raw_view.digits() != 26
            || source.rank != profile.inner.matrix.output_rank()
            || self.polynomials.len() != 1
        {
            return Err(AkitaError::InvalidInput("original RAM installed source authority mismatch".into()));
        }
        for polynomial in &self.polynomials {
            let descriptor = polynomial.descriptor()?;
            if descriptor.class() != (crate::commitment::CommitSourceClass::OriginalBalancedDigitPlanes {
                log_basis: 5, num_digits: 26,
            }) || RootPolyMeta::<F>::num_vars(polynomial) != 16
                || <P as RootPolyShape<F, 128>>::num_live_ring_elems(polynomial) != 512
                || <P as RootPolyShape<F, 128>>::num_ring_elems(polynomial) != 512
            {
                return Err(AkitaError::InvalidInput("source lacks original RAM plane producer capability".into()));
            }
        }
        Ok(())
    }
    fn validate_original_main(
        &self,
        installed: &InstalledFullRingSetup<F>,
        profile: GroupCommitPhaseParams,
        contract: sis::CommittedSourceContract,
        column: usize,
    ) -> Result<(), AkitaError> {
        profile.validate_frozen_precommit(F::MODULUS_BITS)?;
        let source = installed.descriptor().source(self.source_id)?;
        if profile.group != PolynomialGroupLayout::new(16, 56)
            || profile.blocks != BlockGeometry::new(512, 512, 1)
            || profile.inner.matrix.ring_dimension() != 128
            || profile.inner.matrix.output_rank() != 3
            || profile.inner.matrix.input_width() != 512
            || profile.inner.digits != GadgetDigits::new(5, 1)
            || installed.descriptor() != self.setup.descriptor()
            || self.raw_view != source.raw_view
            || self.raw_view.permutation().is_some()
            || self.raw_view.polynomials() != 56
            || self.raw_view.degree() != 128
            || self.raw_view.blocks() != 1
            || self.raw_view.positions() != 512
            || self.raw_view.digits() != 1
            || source.rank != profile.inner.matrix.output_rank()
            || self.polynomials.len() != 56
            || contract.decomposition().field_bits() != F::MODULUS_BITS
        {
            return Err(AkitaError::InvalidInput("original Main source authority or geometry mismatch".into()));
        }
        let chunk_size = contract.class().required_onehot_chunk_size().ok_or_else(|| {
            AkitaError::InvalidInput("original Main requires its original one-hot producer contract".into())
        })?;
        let polynomial = self.polynomials.get(column).ok_or_else(|| {
            AkitaError::InvalidInput("original Main component out of range".into())
        })?;
        let descriptor = polynomial.descriptor()?;
        // Match the producer's Boolean whole-grid admission: a structurally
        // k-one-hot source satisfies the singleton Boolean contract without
        // changing its physical stride or weakening the charged grid bounds.
        let admitted_class = match descriptor.class() {
            crate::commitment::CommitSourceClass::OneHot { chunk_size: actual } => {
                chunk_size == 1 || actual == chunk_size
            }
            _ => false,
        };
        if !admitted_class
            || RootPolyMeta::<F>::num_vars(polynomial) != 16
            || <P as RootPolyShape<F, 128>>::num_ring_elems(polynomial) != 512
            || <P as RootPolyShape<F, 128>>::num_live_ring_elems(polynomial) != 512
        {
            return Err(AkitaError::InvalidInput("original Main component producer mismatch".into()));
        }
        Ok(())
    }

    fn original_coefficient(&self, plan: &CommitInnerPlan, index: usize) -> Result<F, AkitaError> {
        let width = plan.num_live_blocks.checked_mul(plan.num_positions_per_block)
            .and_then(|n| n.checked_mul(plan.num_digits_inner))
            .and_then(|n| n.checked_mul(plan.ring_dimension))
            .filter(|&n| n != 0)
            .ok_or_else(|| AkitaError::InvalidInput("original source extent overflow".into()))?;
        let polynomial = self.polynomials.get(index / width)
            .ok_or_else(|| AkitaError::InvalidInput("original source index out of range".into()))?;
        polynomial.full_ring_digit(plan, index % width)
    }

    fn original_source_id(&self) -> FullRingSourceId { self.source_id }
    fn opening(
        &self,
        backend: &CpuBackend<F, F>,
        binding: &OperationBinding,
        plan: &ValidatedRecursiveGroupOpeningPlan<'_, F>,
        source: PreparedOpeningSource<F, F>,
    ) -> Result<PreparedGroupOpening<F, CpuPreparedOpeningHandle<F, F>>, AkitaError> {
        native_packing(plan.opening_method(), plan.ring_dimension())?;
        let geometry = SubringCoefficientPackingGeometry::try_new(1, 128, 128)?;
        let polys = self.polynomials.iter().collect::<Vec<_>>();
        let first = polys.first().ok_or_else(|| AkitaError::InvalidInput("empty original source".into()))?;
        let live = <P as RootPolyShape<F, 128>>::num_live_ring_elems(*first);
        let vars = RootPolyMeta::<F>::num_vars(*first);
        if polys.iter().any(|p| {
            <P as RootPolyShape<F, 128>>::num_live_ring_elems(*p) != live
                || RootPolyMeta::<F>::num_vars(*p) != vars
        }) || live.div_ceil(plan.positions_per_block()) != plan.live_blocks() {
            return Err(AkitaError::InvalidInput("original source packing geometry mismatch".into()));
        }
        let point = match plan.source_view() {
            SourceOpeningView::Decoded => PreparedSubringCoefficientPackingPoint::new(
                geometry, plan.basis(), live, plan.positions_per_block(), vars, plan.point(),
            )?,
            SourceOpeningView::OriginalRamPlanesV1 => {
                let (parameters, chunks) = backend.admitted_group(binding)?;
                let committed = match &source {
                    PreparedOpeningSource::Retained(
                        super::openings::RetainedOpeningSource::Commitment(committed),
                    ) => committed,
                    _ => return Err(AkitaError::InvalidInput("raw RAM requires an original commitment".into())),
                };
                if binding.fold_level() != 0 || chunks != 1
                    || parameters.source_view() != plan.source_view()
                {
                    return Err(AkitaError::InvalidInput("raw RAM is root-only single-chunk packing".into()));
                }
                self.validate_original_ram(backend.full_ring_setup()?, committed.parameters, committed.producer_contract)?;
                PreparedSubringCoefficientPackingPoint::new_original_ram_planes(
                    geometry, plan.basis(), live, plan.positions_per_block(), vars, plan.point(),
                )?
            }
        };
        let batch = <P as RootOpeningSource<F, 128>>::opening_batch(&polys)?;
        let partials = SubringCoefficientPackingBatchKernel::coefficient_packing_partials_batch(
            backend, Some(backend.prepared()?), batch, SubringCoefficientPackingPlan { point: &point },
        )?;
        if partials.len() != polys.len() {
            return Err(AkitaError::InvalidInput("original source partial count mismatch".into()));
        }
        let openings = partials.iter().map(|partial| {
            crate::arithmetic::coefficient_packing_fold::coefficient_packing_scalar_opening::<F, F>(
                geometry, point.num_live_blocks(), std::slice::from_ref(partial), &[F::one()],
                point.live_block_weights(), point.tail_weights(),
            )
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(super::prepared_opening::coefficient_packing(
            binding.clone(), source, point, partials, openings,
        ))
    }

    fn probe(
        &self,
        backend: &CpuBackend<F, F>,
        plan: &ValidatedFoldProbePlan<'_>,
    ) -> Result<FoldProbeOutcome<CpuAcceptedFoldHandle<F>>, AkitaError> {
        native_packing(plan.opening_method(), plan.ring_dimension())?;
        let polys = self.polynomials.iter().collect::<Vec<_>>();
        let batch = <P as RootOpeningSource<F, 128>>::opening_batch(&polys)?;
        FoldResponseKernel::probe(backend, Some(backend.prepared()?), batch, plan)
    }
}

/// Retain the complete producer owner, including native exact-row material and
/// its original source Arc. No polynomial or packed digit storage is copied.
struct ProjectedOriginal<F: Field + CanonicalEncoding, E: Field> {
    original: Arc<CommittedSource<F, E>>,
}
impl<F: Field + CanonicalEncoding, E: Field> ProjectedOriginal<F, E> {
    fn native(&self) -> Result<&dyn FullRingPrivateSource<F, E>, AkitaError> {
        self.original.source.full_ring().ok_or_else(|| {
            AkitaError::InvalidInput("projection lost original native capability".into())
        })
    }
}
impl<F: Field + CanonicalEncoding, E: Field> PrivateSource<F, F> for ProjectedOriginal<F, E> {
    fn commitment_sources(&self) -> Vec<&dyn CommitmentSource<F>> {
        self.original.source.commitment_sources()
    }
    fn legacy(&self) -> Option<&dyn LegacyPrivateSource<F, F>> { None }
    fn full_ring(&self) -> Option<&dyn FullRingPrivateSource<F, F>> { None }
    fn original_planes(&self) -> Option<&dyn OrdinaryBaseSource<F>> {
        self.native().ok().map(|native| native.ordinary_base())
    }
    fn ordinary_opening(
        &self,
        backend: &CpuBackend<F, F>,
        binding: &OperationBinding,
        plan: &ValidatedRecursiveGroupOpeningPlan<'_, F>,
        source: PreparedOpeningSource<F, F>,
    ) -> Result<PreparedGroupOpening<F, CpuPreparedOpeningHandle<F, F>>, AkitaError> {
        self.native()?.ordinary_base().opening(backend, binding, plan, source)
    }
    fn probe(
        &self,
        backend: &CpuBackend<F, F>,
        plan: &ValidatedFoldProbePlan<'_>,
    ) -> Result<FoldProbeOutcome<CpuAcceptedFoldHandle<F>>, AkitaError> {
        self.native()?.ordinary_base().probe(backend, plan)
    }
    #[cfg(feature = "response-model-diagnostics")]
    fn source_l2_sq(&self) -> Option<u128> { self.original.source.source_l2_sq() }
}

pub(super) fn validate_original_ram_profile<F: Field + CanonicalEncoding>(
    profile: GroupCommitPhaseParams,
    contract: sis::CommittedSourceContract,
) -> Result<(), AkitaError> {
    profile.validate_frozen_precommit(F::MODULUS_BITS)?;
    const Q: u128 = u128::MAX - (1u128 << 32) + 22538;
    let mut modulus = [0u8; 32];
    modulus[16..].copy_from_slice(&Q.to_be_bytes());
    if field_modulus_be_bytes::<F>()? != modulus
        || profile.group != PolynomialGroupLayout::new(16, 1)
        || profile.inner.matrix.ring_dimension() != 128
        || profile.inner.matrix.output_rank() != 3
        || profile.blocks.live_blocks != 16
        || profile.blocks.positions_per_block != 32
        || profile.inner.digits.log_basis != 5
        || profile.inner.matrix.input_width() != 32 * 26
        || profile.outer.matrix.ring_dimension() != 128
        || profile.outer.matrix.output_rank() != 1
        || profile.outer.matrix.input_width() != 16 * 3 * 26
        || profile.outer_slice_count != CommitmentSliceCount::ONE
        || profile.outer.digits.log_basis != 5
        || profile.outer.digits.num_digits != 26
        || profile.inner.digits.num_digits != 26
        || contract.class() != sis::CommittedSourceClass::OriginalBalancedDigitPlanes
        || contract.original_plane_geometry() != Some((5, 26))
        || contract.decomposition().field_bits() != 128
    {
        return Err(AkitaError::InvalidInput("unsupported original RAM producer profile or contract".into()));
    }
    Ok(())
}

/// Borrowed literal original RAM digits in physical
/// `[block][position][digit][coefficient]` order, with no virtual zero slots.
pub struct CpuOriginalRamPlaneReader<'a, F: Field + CanonicalEncoding> {
    source: &'a dyn OrdinaryBaseSource<F>,
    plan: CommitInnerPlan,
    commitment_id: u128,
    backend_id: u64,
    setup_digest: [u8; 32],
}

impl<F: Field + CanonicalEncoding> akita_types::full_ring::FullRingCoefficientSource<F>
    for CpuOriginalRamPlaneReader<'_, F>
{
    fn len(&self) -> usize { 16 * 32 * 26 * 128 }

    fn coefficient(&self, index: usize) -> Result<F, AkitaError> {
        if index >= 16 * 32 * 26 * 128 {
            return Err(AkitaError::InvalidInput("original RAM physical index out of range".into()));
        }
        self.source.original_coefficient(&self.plan, index)
    }
}

impl<F: Field + CanonicalEncoding> CpuOriginalRamPlaneReader<'_, F> {
    pub fn source_id(&self) -> FullRingSourceId { self.source.original_source_id() }
    /// Retained-handle identity, distinct from the reusable installed source role.
    pub fn commitment_id(&self) -> u128 { self.commitment_id }
    pub fn backend_id(&self) -> u64 { self.backend_id }
    pub fn setup_digest(&self) -> [u8; 32] { self.setup_digest }
}

impl<F: Field + CanonicalEncoding, E: Field> CpuBackend<F, E> {
    /// Validate the retained owner/setup/profile/producer before borrowing any
    /// digits. Works on the ordinary projection as well as its native owner.
    /// Query adapters must supply the registered raw-to-physical injection;
    /// this provider never pretends the six missing planes are physical cells.
    pub fn original_ram_plane_reader<'a>(
        &self,
        handle: &'a CommitmentHandle<F, E>,
    ) -> Result<CpuOriginalRamPlaneReader<'a, F>, AkitaError> {
        if handle.owner != self.owner().backend_id()
            || handle.committed.metadata != SourceMetadata::try_new(1, 16)?
        {
            return Err(AkitaError::InvalidInput("original RAM owner or source metadata mismatch".into()));
        }
        let source = handle.committed.source.original_planes().ok_or_else(|| {
            AkitaError::InvalidInput("source lacks retained original-plane capability".into())
        })?;
        source.validate_original_ram(
            self.full_ring_setup()?, handle.committed.parameters, handle.committed.producer_contract,
        )?;
        Ok(CpuOriginalRamPlaneReader {
            source, plan: CommitInnerPlan::from_profile(&handle.committed.parameters),
            commitment_id: handle.committed.commitment_id,
            backend_id: handle.owner,
            setup_digest: self.owner().setup_digest(),
        })
    }
}

/// Borrowed natural16 coefficient component of the literal original Main56.
pub struct CpuOriginalMainComponentReader<'a, F: Field + CanonicalEncoding> {
    source: &'a dyn OrdinaryBaseSource<F>,
    plan: CommitInnerPlan,
    column: usize,
    commitment_id: u128,
    backend_id: u64,
    setup_digest: [u8; 32],
}

impl<F: Field + CanonicalEncoding> akita_types::full_ring::FullRingCoefficientSource<F>
    for CpuOriginalMainComponentReader<'_, F>
{
    fn len(&self) -> usize { 65536 }

    fn coefficient(&self, index: usize) -> Result<F, AkitaError> {
        if index >= 65536 {
            return Err(AkitaError::InvalidInput("original Main component index out of range".into()));
        }
        self.source.original_coefficient(&self.plan, self.column * 65536 + index)
    }
}

impl<F: Field + CanonicalEncoding> CpuOriginalMainComponentReader<'_, F> {
    pub fn source_id(&self) -> FullRingSourceId { self.source.original_source_id() }
    pub fn column(&self) -> usize { self.column }
    pub fn num_vars(&self) -> usize { 16 }
    /// Shared retained Main commitment identity for all56 component readers.
    pub fn commitment_id(&self) -> u128 { self.commitment_id }
    pub fn backend_id(&self) -> u64 { self.backend_id }
    pub fn setup_digest(&self) -> [u8; 32] { self.setup_digest }
}

impl<F: Field + CanonicalEncoding, E: Field> CpuBackend<F, E> {
    /// Borrow a validated original Main column without decoding or copying it.
    pub fn original_main_component_reader<'a>(
        &self,
        handle: &'a CommitmentHandle<F, E>,
        column: usize,
    ) -> Result<CpuOriginalMainComponentReader<'a, F>, AkitaError> {
        if handle.owner != self.owner().backend_id()
            || handle.committed.metadata != SourceMetadata::try_new(56, 16)?
            || column >= 56
        {
            return Err(AkitaError::InvalidInput("original Main owner, metadata or column mismatch".into()));
        }
        let source = handle.committed.source.original_planes().ok_or_else(|| {
            AkitaError::InvalidInput("source lacks retained original Main capability".into())
        })?;
        source.validate_original_main(
            self.full_ring_setup()?, handle.committed.parameters, handle.committed.producer_contract,
            column,
        )?;
        Ok(CpuOriginalMainComponentReader {
            source, plan: CommitInnerPlan::from_profile(&handle.committed.parameters), column,
            commitment_id: handle.committed.commitment_id,
            backend_id: handle.owner,
            setup_digest: self.owner().setup_digest(),
        })
    }
}

impl<F> CpuBackend<F, F>
where
    F: Field + CanonicalEncoding + AkitaSerialize + Unreduced + WithCommitAccumulator
        + Ring + ExtField<F> + 'static,
    F::Wide: From<F> + jolt_field::AdditiveGroup,
{
    /// Authenticate an original E2-owned K source and mint ordinary receiving
    /// authority. The original profile, producer contract, semantic source ID,
    /// raw view, exact digits and native retained state stay with their Arc.
    /// Only the consuming owner/operation and additional quotient material change.
    /// Registered original-plane queries are admitted separately at opening time.
    pub fn import_full_ring_commitment<EOrigin>(
        &self,
        foreign: &CommitmentHandle<F, EOrigin>,
    ) -> Result<CommitmentHandle<F, F>, AkitaError>
    where EOrigin: ExtField<F> + 'static,
    {
        akita_types::full_ring::validate_full_ring_fields::<F, EOrigin>()?;
        let committed = &foreign.committed;
        let native = committed.source.full_ring().ok_or_else(|| {
            AkitaError::InvalidInput("ordinary projection requires an original full-ring commitment".into())
        })?;
        let setup = self.full_ring_setup()?;
        if setup.shared_prefix().is_none()
            || native.installed_setup().shared_prefix().is_none()
            || setup.descriptor() != native.installed_setup().descriptor()
            || native.raw_view() != &setup.descriptor().source(native.source_id())?.raw_view
        {
            return Err(AkitaError::InvalidInput("original projection setup or source identity mismatch".into()));
        }
        let sources = committed.source.commitment_sources();
        let layout = crate::commitment::resolve_source_layout(&sources)?;
        if layout != committed.parameters.group
            || committed.metadata != SourceMetadata::try_new(layout.num_polynomials(), layout.num_vars())?
        {
            return Err(AkitaError::InvalidInput("original projection profile mismatch".into()));
        }
        crate::commitment::validate_full_ring_commit_source::<F, _>(
            &sources, setup.descriptor(), native.source_id(), committed.parameters,
            committed.producer_contract, layout, true,
        )?;
        let executor = CommitmentExecutor::cpu_full_ring(
            self, Some(native.source_id()), Vec::new(), PortableStatePolicy,
        )?;
        let plan = crate::commitment::CommitmentExecutionPlan::for_full_ring_root(&committed.parameters)?;
        let (payload, retained) = executor.execute_full(&plan, &sources)?.into_parts();
        if Commitment::new(payload) != committed.public || retained != committed.retained {
            return Err(AkitaError::InvalidInput("original projection material or payload mismatch".into()));
        }
        let ordinary = super::compression::derive_original_quotient_lift(
            self, &committed.retained, &committed.parameters, &committed.public,
        )?;
        Ok(CommitmentHandle {
            owner: self.owner().backend_id(),
            committed: Arc::new(CommittedSource {
                source: Arc::new(ProjectedOriginal { original: committed.clone() }),
                metadata: committed.metadata,
                commitment_id: self.owner().next_operation_id()?,
                parameters: committed.parameters,
                producer_contract: committed.producer_contract,
                public: committed.public.clone(),
                retained: ordinary,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use akita_config::{full_ring::CompactInner, SetupRequirements};
    use jolt_field::{One, Prime128OffsetA7F7 as K, Prime128OffsetA7F7Ext2 as E2, Zero};

    #[test]
    fn original_projection_never_retags_another_opening_method() {
        assert!(native_packing(OpeningMethod::EvaluationTrace, 128).is_err());
        assert!(native_packing(OpeningMethod::SubringCoefficientPacking {
            challenge_subring_dimension: 64,
        }, 128).is_err());
        assert!(native_packing(OpeningMethod::SubringCoefficientPacking {
            challenge_subring_dimension: 128,
        }, 256).is_err());
        native_packing(OpeningMethod::SubringCoefficientPacking {
            challenge_subring_dimension: 128,
        }, 128).unwrap();
    }

    fn copied<E: Field>(handle: &CommitmentHandle<K, E>) -> CommittedSource<K, E> {
        let original = &handle.committed;
        CommittedSource {
            source: original.source.clone(),
            metadata: original.metadata,
            commitment_id: original.commitment_id,
            parameters: original.parameters,
            producer_contract: original.producer_contract,
            public: original.public.clone(),
            retained: original.retained.clone(),
        }
    }

    #[test]
    #[ignore = "real compact source authentication and compression products"]
    fn original_projection_authenticates_profile_path_and_preserves_owner_allocation() {
        let catalog = akita_config::test_support::workspace_schedule_catalog::<CompactInner>().unwrap();
        let (key, contracts) = CompactInner::root_inputs().unwrap();
        let profile = key.precommitteds[1];
        let requirements = SetupRequirements::from_catalog(&catalog, 22, 59).unwrap();
        let setup = crate::AkitaProverSetup::<K>::generate_with_capacity(
            22, 59, requirements.matrix_capacity(),
        ).unwrap();
        let installed = Arc::new(crate::install_compact_inner_setup(
            &catalog, [42; 32], Arc::from(&b"original-projection/private-regressions"[..]),
            setup.expanded.clone(),
        ).unwrap());
        let native = CpuBackend::<K, E2>::from_full_ring_setup(installed.clone()).unwrap();
        let ordinary = CpuBackend::<K, K>::from_full_ring_setup(installed).unwrap();
        let plan = CommitInnerPlan {
            ring_dimension: 128, num_live_blocks: 16, n_a: 3,
            num_positions_per_block: 32, num_digits_inner: 26, log_basis_inner: 5,
        };
        let mut digits = vec![0; 16 * 32 * 26 * 128];
        digits[25 * 128 + 127] = 15;
        let source = native.import_source_full_ring::<crate::PackedRamSource>(
            FullRingSourceId(2),
            vec![crate::PackedRamSource::from_signed_digits(16, plan, digits).unwrap()],
        ).unwrap();
        let original = native.commit_full_ring(&source, profile, contracts[1]).unwrap().private_handle;
        let weak = Arc::downgrade(&original.committed);
        let original_material = original.committed.retained.clone();
        let projection = ordinary.import_full_ring_commitment(&original).unwrap();
        assert_eq!(projection.committed.parameters, profile);
        assert_eq!(projection.committed.producer_contract, contracts[1]);
        assert_eq!(projection.committed.public, original.committed.public);
        assert!(projection.committed.source.legacy().is_none());
        assert!(projection.committed.source.full_ring().is_none());
        assert_eq!(original.committed.retained, original_material);
        use akita_types::full_ring::FullRingCoefficientSource;
        for reader in [
            native.original_ram_plane_reader(&original).unwrap(),
            ordinary.original_ram_plane_reader(&projection).unwrap(),
        ] {
            assert_eq!(reader.len(), 16 * 32 * 26 * 128);
            assert_eq!(reader.source_id(), FullRingSourceId(2));
            for plane in 0..26 {
                assert_eq!(reader.coefficient(plane * 128 + 127).unwrap(),
                    if plane == 25 { K::from_u64(15) } else { K::zero() });
            }
            assert!(reader.coefficient(reader.len()).is_err());
        }
        let mut wrong_owner = projection.clone();
        wrong_owner.owner = ordinary.owner().backend_id().wrapping_add(1);
        assert!(ordinary.original_ram_plane_reader(&wrong_owner).is_err());
        let mut wrong_contract = copied(&projection);
        wrong_contract.producer_contract = contracts[0];
        assert!(ordinary.original_ram_plane_reader(&CommitmentHandle {
            owner: projection.owner, committed: Arc::new(wrong_contract),
        }).is_err());

        let reject = |corrupt| {
            assert!(ordinary.import_full_ring_commitment(&CommitmentHandle {
                owner: original.owner, committed: Arc::new(corrupt),
            }).is_err());
        };
        let mut corrupt = copied(&original);
        corrupt.parameters.inner.digits.num_digits -= 1;
        reject(corrupt);
        let mut corrupt = copied(&original);
        corrupt.metadata = SourceMetadata::try_new(1, 15).unwrap();
        reject(corrupt);
        let mut corrupt = copied(&original);
        let mut payload = corrupt.public.0.coeffs().to_vec();
        payload[0] += K::one();
        corrupt.public = Commitment::new(RingVec::from_coeffs_with_ring_dim(
            payload, corrupt.public.0.ring_dim(),
        ).unwrap());
        reject(corrupt);
        let mut corrupt = copied(&original);
        corrupt.retained = crate::commitment::PortableCommitmentHandle::new(
            128, vec![RingVec::from_coeffs_with_ring_dim(vec![K::zero(); 16 * 3 * 128], 128).unwrap()],
        ).unwrap();
        reject(corrupt);
        // A native handle may not be retagged using an already ordinary view.
        let mut corrupt = copied(&original);
        corrupt.retained = projection.committed.retained.clone();
        reject(corrupt);
        drop(original);
        assert!(weak.upgrade().is_some(), "projection must retain the original owner");
        drop(projection);
        assert!(weak.upgrade().is_none());
    }
}
