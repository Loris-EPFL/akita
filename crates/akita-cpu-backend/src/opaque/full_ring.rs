//! Full-ring operations over retained original K sources. Legacy subfield/EOR
//! operations are deliberately a different source capability.
use super::openings::{PreparedOpeningSource, RetainedOpeningSource};
use super::owned::{CommitmentHandle, LegacyPrivateSource, PrivateSource, SourceHandle};
use super::*;
use crate::commitment::CommitmentSource;
use crate::sources::full_ring::FullRingSource;
use akita_error::AkitaError;
use akita_serialization::AkitaSerialize;
use akita_types::full_ring::{FullRingPartials, FullRingRawPoint};
use akita_types::InstalledFullRingSetup;
use jolt_field::{CanonicalEncoding, ExtField, Field, Ring, Unreduced};
use std::sync::Arc;

pub(super) trait FullRingPrivateSource<F: Field + CanonicalEncoding, E: Field>:
    Send + Sync
{
    fn partials(
        &self,
        plan: &CommitInnerPlan,
        point: &FullRingRawPoint<E>,
    ) -> Result<FullRingPartials<F>, AkitaError>;
    fn coefficient(&self, plan: &CommitInnerPlan, index: usize) -> Result<F, AkitaError>;
    fn source_id(&self) -> akita_types::FullRingSourceId;
    fn raw_view(&self) -> &akita_types::full_ring::RawSourceViewPlan;
    fn ordinary_base(&self) -> &dyn super::ordinary_projection::OrdinaryBaseSource<F>;
    fn installed_setup(&self) -> &Arc<InstalledFullRingSetup<F>>;
}
pub(super) struct FullRingOwned<F: Field, P> {
    pub(super) polynomials: Vec<P>,
    pub(super) source_id: akita_types::FullRingSourceId,
    pub(super) raw_view: akita_types::full_ring::RawSourceViewPlan,
    pub(super) setup: Arc<InstalledFullRingSetup<F>>,
}

/// Contract the original source x/digit axes with E equality weights, retaining
/// two complete K planes for every original polynomial/block. No E source copy.
pub(crate) fn source_partials<F, E, P>(
    polynomials: &[P],
    plan: &CommitInnerPlan,
    point: &FullRingRawPoint<E>,
) -> Result<FullRingPartials<F>, AkitaError>
where
    F: Field,
    E: ExtField<F>,
    P: FullRingSource<F>,
{
    let view = point.view();
    if E::DEGREE != 2
        || view.degree() != 128
        || plan.ring_dimension != 128
        || view.polynomials() != polynomials.len()
        || view.blocks() != plan.num_live_blocks
        || view.positions() != plan.num_positions_per_block
        || view.digits() != plan.num_digits_inner
    {
        return Err(AkitaError::InvalidInput(
            "full-ring point differs from original source geometry".into(),
        ));
    }
    let per_polynomial = view.physical_len() / view.polynomials();
    let mut coefficients = vec![F::zero(); view.polynomials() * view.blocks() * 256];
    // Production callers have matched this view to a fold-admitted installed
    // source. Invert its block/column factors, not the entire coefficient grid.
    debug_assert!(view.validate_fold_admission().is_ok());
    let blocks = view.polynomials() * view.blocks();
    let columns = view.positions() * view.digits();
    let inverse = view.permutation().map(|map| {
        let mut inverse = vec![0usize; blocks + columns];
        for raw in 0..blocks {
            inverse[map[raw * columns * 128] / (columns * 128)] = raw;
        }
        for raw in 0..columns {
            inverse[blocks + (map[raw * 128] / 128) % columns] = raw;
        }
        inverse
    });
    for (p, source) in polynomials.iter().enumerate() {
        let mut previous = None;
        source.visit_full_ring_digits(plan, &mut |address, value| {
            if address >= per_polynomial || previous.is_some_and(|last| last >= address) {
                return Err(AkitaError::InvalidInput(
                    "native source visitor address is duplicate or out of range".into(),
                ));
            }
            previous = Some(address);
            let storage = p * per_polynomial + address;
            let address = inverse.as_ref().map_or(storage, |inverse| {
                let cell = storage / 128;
                let block = inverse[cell / columns];
                let column = inverse[blocks + cell % columns];
                (block * columns + column) * 128 + storage % 128
            });
            let j = address % 128;
            let rest = address / 128;
            let ell = rest % view.digits();
            let rest = rest / view.digits();
            let x = rest % view.positions();
            let block = rest / view.positions();
            let weighted =
                (point.position_weights()[x] * point.digit_weights()[ell]).mul_base(value);
            for plane in 0..2 {
                coefficients[(block * 2 + plane) * 128 + j] += weighted.base_coefficient(plane);
            }
            Ok(())
        })?;
    }
    FullRingPartials::new(view.clone(), 2, coefficients)
}

impl<F, E, P> FullRingPrivateSource<F, E> for FullRingOwned<F, P>
where
    F: Field + CanonicalEncoding,
    E: ExtField<F>,
    P: FullRingSource<F>,
    Self: super::ordinary_projection::OrdinaryBaseSource<F>,
{
    fn ordinary_base(&self) -> &dyn super::ordinary_projection::OrdinaryBaseSource<F> {
        self
    }
    fn installed_setup(&self) -> &Arc<InstalledFullRingSetup<F>> {
        &self.setup
    }
    fn source_id(&self) -> akita_types::FullRingSourceId {
        self.source_id
    }
    fn raw_view(&self) -> &akita_types::full_ring::RawSourceViewPlan {
        &self.raw_view
    }
    fn partials(
        &self,
        plan: &CommitInnerPlan,
        point: &FullRingRawPoint<E>,
    ) -> Result<FullRingPartials<F>, AkitaError> {
        source_partials(&self.polynomials, plan, point)
    }
    fn coefficient(&self, plan: &CommitInnerPlan, index: usize) -> Result<F, AkitaError> {
        let per_source = plan
            .num_live_blocks
            .checked_mul(plan.num_positions_per_block)
            .and_then(|n| n.checked_mul(plan.num_digits_inner))
            .and_then(|n| n.checked_mul(128))
            .filter(|n| *n != 0)
            .ok_or_else(|| AkitaError::InvalidInput("source digit extent overflow".into()))?;
        let source = self
            .polynomials
            .get(index / per_source)
            .ok_or_else(|| AkitaError::InvalidInput("source digit out of range".into()))?;
        source.full_ring_digit(plan, index % per_source)
    }
}
impl<F, E, P> PrivateSource<F, E> for FullRingOwned<F, P>
where
    F: Field + CanonicalEncoding + AkitaSerialize + Ring + Unreduced + 'static,
    F::Wide: From<F> + jolt_field::AdditiveGroup,
    E: ExtField<F> + 'static,
    P: FullRingSource<F> + RootPolyMeta<F> + RootOpeningSource<F, 128> + 'static,
    Self: super::ordinary_projection::OrdinaryBaseSource<F>,
    CpuBackend<F, E>: ComputeBackendSetup<F, PreparedSetup = CpuPreparedSetup<F>>
        + FoldHandleBackend<F, AcceptedFold = CpuAcceptedFoldHandle<F>>
        + for<'a> FoldResponseKernel<<P as RootOpeningSource<F, 128>>::OpeningBatchView<'a>, F, 128>,
{
    fn commitment_sources(&self) -> Vec<&dyn CommitmentSource<F>> {
        self.polynomials
            .iter()
            .map(|p| p as &dyn CommitmentSource<F>)
            .collect()
    }
    fn legacy(&self) -> Option<&dyn LegacyPrivateSource<F, E>> {
        None
    }
    fn full_ring(&self) -> Option<&dyn FullRingPrivateSource<F, E>> {
        Some(self)
    }
    fn original_planes(&self) -> Option<&dyn super::ordinary_projection::OrdinaryBaseSource<F>> {
        Some(self)
    }
    #[cfg(feature = "response-model-diagnostics")]
    fn source_l2_sq(&self) -> Option<u128> {
        self.polynomials.iter().try_fold(0u128, |sum, p| {
            sum.checked_add(RootPolyMeta::<F>::exact_integer_coeff_l2_sq(p)?)
        })
    }
    fn probe(
        &self,
        backend: &CpuBackend<F, E>,
        plan: &ValidatedFoldProbePlan<'_>,
    ) -> Result<FoldProbeOutcome<CpuAcceptedFoldHandle<F>>, AkitaError> {
        if plan.ring_dimension() != 128 || !plan.opening_method().is_full_ring_scalar_extension() {
            return Err(AkitaError::InvalidInput(
                "full-ring source fold method mismatch".into(),
            ));
        }
        let sources = self.polynomials.iter().collect::<Vec<_>>();
        let batch = <P as RootOpeningSource<F, 128>>::opening_batch(&sources)?;
        let role = backend
            .full_ring_setup()?
            .descriptor()
            .source(self.source_id)?
            .commitment_roles
            .inner;
        FoldResponseKernel::probe(
            backend,
            Some(backend.full_ring_prepared(role)?),
            batch,
            plan,
        )
    }
}

impl<F, E> CpuBackend<F, E>
where
    F: Field + CanonicalEncoding + AkitaSerialize + Ring + Unreduced + 'static,
    F::Wide: From<F> + jolt_field::AdditiveGroup,
    E: ExtField<F> + 'static,
{
    /// Import native original K sources without tensor, EOR or SourceCoefficients.
    /// Retains the installed setup identity with the original allocation.
    /// Native128 ordinary packing is a separate capability; this handle's
    /// native opening and fold entry points still reject ordinary methods.
    #[allow(private_bounds, reason = "the CPU backend owns and seals the kernel bundle")]
    pub fn import_source_full_ring<P>(
        &self,
        source_id: akita_types::FullRingSourceId,
        polynomials: Vec<P>,
    ) -> Result<SourceHandle<F, E>, AkitaError>
    where
        P: FullRingSource<F> + RootPolyMeta<F> + RootOpeningSource<F, 128> + 'static,
        FullRingOwned<F, P>: super::ordinary_projection::OrdinaryBaseSource<F>,
        Self: ComputeBackendSetup<F, PreparedSetup = CpuPreparedSetup<F>>
            + FoldHandleBackend<F, AcceptedFold = CpuAcceptedFoldHandle<F>>
            + for<'a> FoldResponseKernel<
                <P as RootOpeningSource<F, 128>>::OpeningBatchView<'a>,
                F,
                128,
            >,
    {
        akita_types::full_ring::validate_full_ring_fields::<F, E>()?;
        let setup = self.full_ring_setup()?;
        let source = setup
            .descriptor()
            .sources()
            .iter()
            .find(|source| source.id == source_id)
            .ok_or_else(|| {
                AkitaError::InvalidInput("original source id is not installed".into())
            })?;
        let layout = crate::commitment::resolve_source_layout::<F, P>(&polynomials)?;
        if layout.num_polynomials() != source.polynomials {
            return Err(AkitaError::InvalidInput(
                "source polynomial count differs from installed role".into(),
            ));
        }
        let metadata = SourceMetadata::try_new(layout.num_polynomials(), layout.num_vars())?;
        Ok(SourceHandle {
            owner: self.owner().backend_id(),
            storage: Arc::new(FullRingOwned {
                polynomials,
                source_id,
                raw_view: source.raw_view.clone(),
                setup: setup.clone(),
            }),
            metadata,
        })
    }

    /// Prepare one raw endpoint using the original owner-bound commitment.
    fn prepare_retained_full_ring_opening(
        &self,
        session: &CpuProofSessionHandle,
        context: &ProofContext,
        source: &CommitmentHandle<F, E>,
        point: FullRingRawPoint<E>,
    ) -> Result<PreparedGroupOpening<E, CpuPreparedOpeningHandle<F, E>>, AkitaError> {
        let binding = self.binding(session, context)?;
        let (parameters, _) = self.admitted_group(&binding)?;
        if source.owner != self.owner().backend_id()
            || source.committed.parameters != parameters.profile
            || !parameters.opening_method().is_full_ring_scalar_extension()
        {
            return Err(AkitaError::InvalidInput(
                "full-ring opening source differs from admitted group".into(),
            ));
        }
        binding
            .scope_lease()
            .validate_commitment(context, source.committed.commitment_id)?;
        let plan = CommitInnerPlan::from_profile(&source.committed.parameters);
        let capability = source.committed.source.full_ring().ok_or_else(|| {
            AkitaError::InvalidInput("source lacks native full-ring capability".into())
        })?;
        let installed = self
            .full_ring_setup()?
            .descriptor()
            .sources()
            .iter()
            .find(|source| source.id == capability.source_id())
            .ok_or_else(|| {
                AkitaError::InvalidInput("opening source role is not installed".into())
            })?;
        if point.view() != &installed.raw_view || capability.raw_view() != &installed.raw_view {
            return Err(AkitaError::InvalidInput(
                "raw source view differs from installed source identity".into(),
            ));
        }
        let partials = capability.partials(&plan, &point)?;
        let scalar = point.scalar_from_partials(&partials)?;
        let handle = CpuPreparedOpeningHandle::full_ring(
            binding,
            PreparedOpeningSource::Retained(RetainedOpeningSource::Commitment(
                source.committed.clone(),
            )),
            point,
            partials,
            vec![scalar],
        );
        Ok(PreparedGroupOpening::new(vec![scalar], handle))
    }
}

impl<F, E> OpaqueFullRingOpeningKernel<F, E> for CpuBackend<F, E>
where
    F: Field
        + CanonicalEncoding
        + AkitaSerialize
        + jolt_field::PseudoMersenne
        + Ring
        + Unreduced
        + 'static,
    F::Wide: From<F> + jolt_field::AdditiveGroup,
    E: ExtField<F> + 'static,
{
    fn full_ring_source_id(
        &self,
        source: &Self::CommitmentHandle,
    ) -> Result<akita_types::FullRingSourceId, AkitaError> {
        if source.owner != self.owner().backend_id() {
            return Err(AkitaError::InvalidInput(
                "source belongs to another backend".into(),
            ));
        }
        let capability = source.committed.source.full_ring().ok_or_else(|| {
            AkitaError::InvalidInput("source lacks native full-ring capability".into())
        })?;
        let installed = self
            .full_ring_setup()?
            .descriptor()
            .source(capability.source_id())?;
        if capability.raw_view() != &installed.raw_view {
            return Err(AkitaError::InvalidInput(
                "source view differs from installed original identity".into(),
            ));
        }
        Ok(capability.source_id())
    }
    fn same_full_ring_source(
        &self,
        left: &Self::CommitmentHandle,
        right: &Self::CommitmentHandle,
    ) -> Result<bool, AkitaError> {
        self.full_ring_source_id(left)?;
        self.full_ring_source_id(right)?;
        Ok(std::sync::Arc::ptr_eq(&left.committed, &right.committed)
            && left.committed.commitment_id == right.committed.commitment_id)
    }
    fn full_ring_witness_source_id(
        &self,
        witness: &Self::WitnessHandle,
    ) -> Result<akita_types::FullRingSourceId, AkitaError> {
        self.validate_binding(&witness.operation_binding())?;
        let source = witness.full_ring_source_id.ok_or_else(|| {
            AkitaError::InvalidInput("A-only terminal witness is not a recursive source".into())
        })?;
        self.full_ring_setup()?.descriptor().source(source)?;
        Ok(source)
    }
    fn with_full_ring_source<T>(
        &self,
        session: &Self::ProofSessionHandle,
        context: &ProofContext,
        source: &Self::CommitmentHandle,
        use_source: impl FnOnce(
            &dyn akita_types::full_ring::FullRingCoefficientSource<F>,
        ) -> Result<T, AkitaError>,
    ) -> Result<T, AkitaError> {
        self.validate_context(session, context)?;
        session
            .scope_lease()
            .validate_commitment(context, source.committed.commitment_id)?;
        let reader = self.full_ring_source_reader(source)?;
        use_source(&reader)
    }
    fn prepare_full_ring_opening(
        &self,
        session: &Self::ProofSessionHandle,
        context: &ProofContext,
        source: OpeningSource<'_, Self::CommitmentHandle, Self::WitnessHandle>,
        point: FullRingRawPoint<E>,
    ) -> Result<PreparedGroupOpening<E, Self::PreparedOpeningHandle>, AkitaError> {
        match source {
            OpeningSource::Commitment(source) => {
                self.prepare_retained_full_ring_opening(session, context, source, point)
            }
            OpeningSource::Witness(witness) => {
                let binding = self.binding(session, context)?;
                self.validate_binding(&witness.operation_binding())?;
                binding.validate_lineage(&witness.operation_binding())?;
                let (parameters, _) = self.admitted_group(&binding)?;
                if !parameters.opening_method().is_full_ring_scalar_extension() {
                    return Err(AkitaError::InvalidInput(
                        "recursive full-ring opening method mismatch".into(),
                    ));
                }
                let source_id = self.full_ring_witness_source_id(witness)?;
                let descriptor = self.full_ring_setup()?.descriptor();
                let (schedule, root_layout) = binding.scope_lease().proof_plan()?;
                let layout = if binding.fold_level() == 0 {
                    root_layout
                } else {
                    let fold = schedule
                        .recursive_folds
                        .get(binding.fold_level() as usize - 1)
                        .ok_or_else(|| {
                            AkitaError::InvalidInput("incoming witness template".into())
                        })?;
                    akita_params::OpeningClaimsLayout::from_groups(
                        fold.params
                            .groups()
                            .iter()
                            .map(|group| group.profile.group)
                            .collect(),
                    )?
                };
                let group = binding.group_index().ok_or_else(|| {
                    AkitaError::InvalidInput("incoming witness needs an occurrence group".into())
                })?;
                let order = layout.root_group_order()?;
                let position = order
                    .iter()
                    .position(|&index| index == group)
                    .ok_or_else(|| {
                        AkitaError::InvalidInput("incoming witness group absent from layout".into())
                    })?;
                if descriptor
                    .level(binding.fold_level() as usize)?
                    .sources
                    .get(position)
                    != Some(&source_id)
                    || witness
                        .operation_binding()
                        .group_index()
                        .is_some_and(|expected| expected != group)
                    || point.view() != &descriptor.source(source_id)?.raw_view
                    || witness.pending_successor.is_some()
                {
                    return Err(AkitaError::InvalidInput(
                        "recursive source differs from installed continuation view".into(),
                    ));
                }
                let plan = CommitInnerPlan::from_profile(&parameters.profile);
                let partials = witness.full_ring_partials::<F, E>(&plan, &point)?;
                let value = point.scalar_from_partials(&partials)?;
                let handle = CpuPreparedOpeningHandle::full_ring(
                    binding,
                    PreparedOpeningSource::Retained(RetainedOpeningSource::Witness(Box::new(
                        witness.snapshot(),
                    ))),
                    point,
                    partials,
                    vec![value],
                );
                Ok(PreparedGroupOpening::new(vec![value], handle))
            }
        }
    }
    fn probe_full_ring_opening_fold(
        &self,
        context: &ProofContext,
        opening: &Self::PreparedOpeningHandle,
        plan: &ValidatedFoldProbePlan<'_>,
    ) -> Result<FoldProbeOutcome<Self::AcceptedFoldHandle>, AkitaError> {
        if !plan.opening_method().is_full_ring_scalar_extension() {
            return Err(AkitaError::InvalidInput(
                "selected fold requires full-ring method".into(),
            ));
        }
        self.probe_retained_opening_fold(context, opening, plan)
    }
}

impl<F, E> OpaqueFullRingWitnessKernel<F, E> for CpuBackend<F, E>
where
    F: Field
        + CanonicalEncoding
        + AkitaSerialize
        + jolt_field::PseudoMersenne
        + Ring
        + Unreduced
        + 'static,
    F::Wide: From<F> + jolt_field::AdditiveGroup,
    E: ExtField<F> + 'static,
{
    fn finish_full_ring_recursive_witness(
        &self,
        build_handle: Self::WitnessBuildHandle,
        fold_inputs: Vec<RecursiveWitnessFoldInput<Self::AcceptedFoldHandle>>,
        commitment_ring_dimension: usize,
    ) -> Result<Self::WitnessHandle, AkitaError> {
        self.validate_binding(&build_handle.binding)?;
        let parent = build_handle.binding.clone();
        if fold_inputs.len() != build_handle.opening_batch.num_groups() {
            return Err(AkitaError::InvalidInput(
                "full-ring accepted fold count mismatch".into(),
            ));
        }
        for (index, input) in fold_inputs.iter().enumerate() {
            input
                .fold_handle()
                .validate_challenges(input.challenges().ambient_a())?;
            let binding = input.fold_handle().binding();
            self.validate_binding(&binding)?;
            binding.validate_lineage(&parent)?;
            binding.validate_group(index, fold_inputs.len())?;
            binding.validate_computation(build_handle.opening_bindings.get(index).ok_or_else(
                || AkitaError::InvalidInput("missing original opening binding".into()),
            )?)?;
            if !matches!(
                input.challenges(),
                akita_types::OpeningFamily::FullRingScalarExtension(_)
            ) {
                return Err(AkitaError::InvalidInput(
                    "full-ring witness requires ambient-ring challenges".into(),
                ));
            }
        }
        let mut witness = crate::opaque::witness_build::finish_cpu_full_ring_witness(
            self,
            self.full_ring_prepared(
                self.full_ring_setup()?
                    .descriptor()
                    .level(parent.fold_level() as usize)?
                    .opening,
            )?,
            build_handle,
            fold_inputs,
            commitment_ring_dimension,
        )?;
        witness.retain_full_ring_extent()?;
        witness.set_operation_binding(self.next_binding(parent)?);
        Ok(witness)
    }
    fn visit_full_ring_witness(
        &self,
        witness: &Self::WitnessHandle,
        visitor: &mut dyn FnMut(usize, F) -> Result<(), AkitaError>,
    ) -> Result<(), AkitaError> {
        let source = self.full_ring_witness_source(witness)?;
        for index in 0..source.len() {
            visitor(index, source.coefficient(index)?)?;
        }
        Ok(())
    }
    fn full_ring_witness_source<'a>(
        &self,
        witness: &'a Self::WitnessHandle,
    ) -> Result<&'a dyn akita_types::full_ring::FullRingCoefficientSource<F>, AkitaError> {
        self.validate_binding(&witness.operation_binding())?;
        // Probe the representation without copying the source; subsequent reads
        // also reject a tensor-produced witness.
        akita_types::full_ring::FullRingCoefficientSource::<F>::coefficient(witness, 0)?;
        Ok(witness)
    }
    fn with_full_ring_relation<T>(
        &self,
        witness: &Self::WitnessHandle,
        use_relation: impl FnOnce(
            &akita_types::full_ring::CoefficientRelationPlan<'_, F, E>,
        ) -> Result<T, AkitaError>,
    ) -> Result<T, AkitaError> {
        self.validate_binding(&witness.operation_binding())?;
        let context = witness
            .full_ring_context
            .as_ref()
            .and_then(|context| {
                context.downcast_ref::<akita_types::full_ring::FullRingWitnessContext<F, E>>()
            })
            .ok_or_else(|| {
                AkitaError::InvalidInput(
                    "witness has no bound full-ring coefficient relation".into(),
                )
            })?;
        context.with_relation(
            self.full_ring_setup()?,
            witness.manifest().commitment_domain_len(),
            use_relation,
        )
    }

    fn complete_full_ring_reduction(
        &self,
        witness: &mut Self::WitnessHandle,
        material: &Self::CommitmentMaterialHandle,
        binding: &akita_prover::backend::NextWitnessBindingMessage<F>,
        completion: akita_prover::backend::FullRingCompletion,
    ) -> Result<ProofContext, AkitaError> {
        use akita_prover::backend::{FullRingCompletionKind, NextWitnessBindingMessage};
        let parent = witness.operation_binding();
        let result = (|| {
            self.validate_binding(&parent)?;
            parent
                .scope_lease()
                .validate_context(completion.context())?;
            let destination = material.binding();
            destination.validate_owner(
                self.owner_id(),
                self.owner().setup_digest(),
                destination.fold_level(),
            )?;
            if destination.scope_id() != parent.scope_id()
                || material.commitment_id() != Some(parent.operation_id())
                || destination.operation_id() != parent.operation_id()
                || witness.pending_successor != Some(destination.fold_level())
                || completion.kind()
                    != (FullRingCompletionKind::Reduction {
                        commitment_domain_len: witness.manifest().commitment_domain_len(),
                    })
            {
                return Err(AkitaError::InvalidInput(
                    "uncoupled full-ring successor completion".into(),
                ));
            }
            match (
                self.full_ring_setup()?
                    .descriptor()
                    .successor(parent.fold_level() as usize)?,
                binding,
            ) {
                (
                    akita_types::FullRingSuccessor::FullCommitment { source },
                    NextWitnessBindingMessage::OuterPayload(public),
                ) if witness.full_ring_source_id == Some(source) => {
                    material.validate_public_commitment(public)?
                }
                (
                    akita_types::FullRingSuccessor::Terminal { .. },
                    NextWitnessBindingMessage::TerminalInnerState(public),
                ) if witness.full_ring_source_id.is_none() => {
                    let actual =
                        akita_prover::backend::TerminalCommitmentMaterialKernel::terminal_message(
                            self, material,
                        )?;
                    if actual.fields() != public.fields() {
                        return Err(AkitaError::InvalidInput(
                            "terminal successor public binding".into(),
                        ));
                    }
                }
                _ => {
                    return Err(AkitaError::InvalidInput(
                        "successor source role or binding mode".into(),
                    ))
                }
            }
            if parent.scope_lease().is_accumulation()? {
                parent
                    .scope_lease()
                    .promote(&parent, &destination, &completion)?;
                witness.set_operation_binding(destination);
                witness.pending_successor = None;
            } else {
                self.advance_ordinary_witness_level(witness)?;
            }
            let active = witness.operation_binding();
            Ok(ProofContext::new(
                self.owner_id(),
                self.owner().setup_digest(),
                active.scope_id(),
                active.fold_level(),
            )
            .with_occurrence(active.occurrence()))
        })();
        if result.is_err() {
            self.owner().abort_scope(parent.scope_lease());
        }
        result
    }

    fn complete_full_ring_phase(
        &self,
        session: &Self::ProofSessionHandle,
        completion: akita_prover::backend::FullRingCompletion,
    ) -> Result<(), AkitaError> {
        let result = session
            .validate_owner(self.owner())?
            .complete_phase(completion);
        if result.is_err() {
            self.owner().abort_scope(&session.scope_lease());
        }
        result
    }
}

/// Borrowed owner-validated access to the exact K digit source of a commitment.
pub struct CpuFullRingSourceReader<'a, F: Field + CanonicalEncoding, E: Field> {
    source: &'a dyn FullRingPrivateSource<F, E>,
    plan: CommitInnerPlan,
    len: usize,
}
impl<F: Field + CanonicalEncoding, E: Field> akita_types::full_ring::FullRingCoefficientSource<F>
    for CpuFullRingSourceReader<'_, F, E>
{
    fn len(&self) -> usize {
        self.len
    }
    fn coefficient(&self, index: usize) -> Result<F, AkitaError> {
        if index >= self.len {
            return Err(AkitaError::InvalidInput(
                "original source index out of range".into(),
            ));
        }
        self.source.coefficient(&self.plan, index)
    }
}

impl<F, E> CpuBackend<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    /// Borrow the original source; ownership is checked before any digit access.
    pub fn full_ring_source_reader<'a>(
        &self,
        source: &'a CommitmentHandle<F, E>,
    ) -> Result<CpuFullRingSourceReader<'a, F, E>, AkitaError> {
        if source.owner != self.owner().backend_id() {
            return Err(AkitaError::InvalidInput(
                "full-ring source belongs to another backend".into(),
            ));
        }
        let capability = source.committed.source.full_ring().ok_or_else(|| {
            AkitaError::InvalidInput("source lacks native full-ring capability".into())
        })?;
        let plan = CommitInnerPlan::from_profile(&source.committed.parameters);
        let len = source
            .committed
            .metadata
            .num_polynomials()
            .checked_mul(plan.num_live_blocks)
            .and_then(|n| n.checked_mul(plan.num_positions_per_block))
            .and_then(|n| n.checked_mul(plan.num_digits_inner))
            .and_then(|n| n.checked_mul(128))
            .ok_or_else(|| AkitaError::InvalidInput("original source extent overflow".into()))?;
        let view = capability.raw_view();
        if view.degree() != plan.ring_dimension
            || view.physical_len() != len
            || view.polynomials() != source.committed.metadata.num_polynomials()
            || view.blocks() != plan.num_live_blocks
            || view.positions() != plan.num_positions_per_block
            || view.digits() != plan.num_digits_inner
        {
            return Err(AkitaError::InvalidInput(
                "retained source reader differs from authenticated commitment geometry".into(),
            ));
        }
        Ok(CpuFullRingSourceReader {
            source: capability,
            plan,
            len,
        })
    }
}

impl<F, E> OpaqueFullRingTerminalKernel<F, E> for CpuBackend<F, E>
where
    F: Field + CanonicalEncoding + AkitaSerialize + Ring + Unreduced + 'static,
    F::Wide: From<F> + jolt_field::AdditiveGroup,
    E: ExtField<F> + 'static,
{
    fn prepare_full_ring_terminal(
        &self,
        session: &Self::ProofSessionHandle,
        context: &ProofContext,
        witness: &Self::WitnessHandle,
        point: FullRingRawPoint<E>,
    ) -> Result<FullRingPartials<F>, AkitaError> {
        let binding = self.binding(session, context)?;
        let parent = witness.operation_binding();
        self.validate_binding(&parent)?;
        binding.validate_lineage(&parent)?;
        let (schedule, _) = parent.scope_lease().proof_plan()?;
        let terminal = &schedule.terminal;
        validate_terminal_source(witness, terminal, schedule.recursive_folds.len())?;
        let plan = CommitInnerPlan {
            ring_dimension: 128,
            num_live_blocks: terminal.blocks.live_blocks,
            n_a: terminal.inner.matrix.output_rank(),
            num_positions_per_block: terminal.blocks.positions_per_block,
            num_digits_inner: 1,
            log_basis_inner: terminal.inner.digits.log_basis,
        };
        prepare_terminal_partials::<F, E>(witness, &plan, &point)
    }

    fn encode_full_ring_terminal_response(
        &self,
        witness: &Self::WitnessHandle,
        challenges: &akita_challenges::Challenges,
    ) -> Result<Vec<u8>, AkitaError> {
        let parent = witness.operation_binding();
        self.validate_binding(&parent)?;
        let (schedule, _) = parent.scope_lease().proof_plan()?;
        let terminal = &schedule.terminal;
        validate_terminal_source(witness, terminal, schedule.recursive_folds.len())?;
        if challenges.num_claims() != 1
            || challenges.num_live_blocks_per_claim() != terminal.blocks.live_blocks
        {
            return Err(AkitaError::InvalidInput(
                "terminal challenge layout mismatch".into(),
            ));
        }
        let coordinates = terminal
            .blocks
            .positions_per_block
            .checked_mul(128)
            .ok_or_else(|| AkitaError::InvalidInput("terminal response extent overflow".into()))?;
        let modulus = akita_params::field_modulus::<F>()?;
        let codec = akita_types::full_ring_terminal::FullRingTerminalCodec::new(
            terminal.blocks.live_blocks,
            coordinates,
            modulus,
        )?;
        if terminal.response_codec
            != (akita_params::TerminalResponseCodec::SignedFixedWidth {
                bytes_per_coefficient: codec.bytes_per_coefficient(),
                coefficient_bound: codec.coefficient_bound(),
            })
        {
            return Err(AkitaError::InvalidInput(
                "terminal codec differs from admitted signed bounds".into(),
            ));
        }
        let mut response = vec![0i128; coordinates];
        for (block, challenge) in challenges.as_slice().iter().enumerate() {
            challenge.validate::<128>()?;
            if challenge
                .coeffs
                .iter()
                .filter(|&&c| c.unsigned_abs() == 1)
                .count()
                != 24
                || challenge
                    .coeffs
                    .iter()
                    .filter(|&&c| c.unsigned_abs() == 2)
                    .count()
                    != 16
                || challenge.coeffs.len() != 40
            {
                return Err(AkitaError::InvalidInput(
                    "terminal challenge is not the declared full-ring shell".into(),
                ));
            }
            for x in 0..terminal.blocks.positions_per_block {
                for j in 0..128 {
                    let value = i128::from(
                        witness.full_ring_signed_coefficient(block * coordinates + x * 128 + j)?,
                    );
                    if value == 0 {
                        continue;
                    }
                    for (&position, &coefficient) in
                        challenge.positions.iter().zip(&challenge.coeffs)
                    {
                        let exponent = j + position as usize;
                        let term = value * i128::from(coefficient);
                        let target = &mut response[x * 128 + exponent % 128];
                        *target = if exponent >= 128 {
                            target.checked_sub(term)
                        } else {
                            target.checked_add(term)
                        }
                        .ok_or_else(|| {
                            AkitaError::InvalidInput("terminal integer response overflow".into())
                        })?;
                    }
                }
            }
        }
        // The codec enforces the signed fixed-width cap before any K reduction.
        codec.encode(&response)
    }
}

fn prepare_terminal_partials<F, E>(
    witness: &CpuWitnessHandle,
    plan: &CommitInnerPlan,
    point: &FullRingRawPoint<E>,
) -> Result<FullRingPartials<F>, AkitaError>
where
    F: Field + CanonicalEncoding,
    E: ExtField<F>,
{
    let view = akita_types::full_ring::RawSourceViewPlan::new(
        128,
        1,
        plan.num_live_blocks,
        plan.num_positions_per_block,
        1,
    )?;
    if point.view() != &view {
        return Err(AkitaError::InvalidInput(
            "terminal query differs from the scheduled identity source view".into(),
        ));
    }
    witness.full_ring_partials::<F, E>(plan, point)
}

fn validate_terminal_source(
    witness: &CpuWitnessHandle,
    terminal: &akita_params::TerminalFoldParams,
    recursive_folds: usize,
) -> Result<(), AkitaError> {
    let domain = terminal
        .blocks
        .live_blocks
        .checked_mul(terminal.blocks.positions_per_block)
        .and_then(|n| n.checked_mul(128))
        .ok_or_else(|| AkitaError::InvalidInput("terminal source extent overflow".into()))?;
    if witness.operation_binding().fold_level() as usize != recursive_folds + 1
        || witness.pending_successor.is_some()
        || terminal.d_a() != 128
        || !terminal.opening_method.is_full_ring_scalar_extension()
        || terminal.source_encoding
            != akita_params::CommittedSourceEncoding::CanonicalCoefficientTable
        || terminal.inner.digits.num_digits != 1
        || witness.manifest().commitment_domain_len() != domain
    {
        return Err(AkitaError::InvalidInput(
            "terminal source differs from actual admitted canonical A-only successor".into(),
        ));
    }
    witness.full_ring_signed_coefficient(0)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use akita_types::full_ring::RawSourceViewPlan;
    use jolt_field::{Ext2Config, FpExt2, One, Prime128OffsetA7F7, Zero};

    type F = Prime128OffsetA7F7;
    struct Five;
    impl Ext2Config<F> for Five {
        fn non_residue() -> F {
            F::from_u64(5)
        }
    }
    // This is scalar extension arithmetic, not an FpExtEncoding instance.
    type E = FpExt2<F, Five>;

    fn plan() -> CommitInnerPlan {
        CommitInnerPlan {
            ring_dimension: 128,
            num_live_blocks: 2,
            n_a: 1,
            num_positions_per_block: 2,
            num_digits_inner: 2,
            log_basis_inner: 5,
        }
    }
    fn point(polynomials: usize) -> FullRingRawPoint<E> {
        let view = RawSourceViewPlan::new(128, polynomials, 2, 2, 2).unwrap();
        FullRingRawPoint::new(
            &view,
            (0..view.num_vars())
                .map(|i| {
                    E::from_base_slice(&[F::from_u64((i + 2) as u64), F::from_u64((i + 7) as u64)])
                })
                .collect(),
        )
        .unwrap()
    }

    #[test]
    fn native_two_plane_contraction_matches_original_digit_mle() {
        akita_types::full_ring::validate_full_ring_fields::<F, E>().unwrap();
        let sources = (0..2)
            .map(|p| {
                crate::DensePoly::from_field_evals(
                    9,
                    (0..512)
                        .map(|i| {
                            let value = F::from_u64(((17 * i + p * 31) % 131) as u64);
                            value - F::from_u64(65)
                        })
                        .collect::<Vec<_>>(),
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let plan = plan();
        let point = point(2);
        let partials = source_partials(&sources, &plan, &point).unwrap();
        let mut expected = E::zero();
        for (p, source) in sources.iter().enumerate() {
            for b in 0..2 {
                for x in 0..2 {
                    for digit in 0..2 {
                        for c in 0..128 {
                            let physical = (((b * 2 + x) * 2 + digit) * 128) + c;
                            let value = source.full_ring_digit(&plan, physical).unwrap();
                            let weight = point.polynomial_weights()[p]
                                * point.block_weights()[b]
                                * point.position_weights()[x]
                                * point.digit_weights()[digit]
                                * point.coefficient_weights()[c];
                            expected += weight.mul_base(value);
                        }
                    }
                }
            }
        }
        assert_eq!(point.scalar_from_partials(&partials).unwrap(), expected);
        assert!(partials
            .plane(1, 1, 1)
            .unwrap()
            .iter()
            .any(|v| *v != F::zero()));
    }

    #[test]
    fn mapped_native_partials_preserve_original_digits_and_both_planes() {
        let field = |value: i64| {
            let magnitude = F::from_u64(value.unsigned_abs());
            if value < 0 {
                -magnitude
            } else {
                magnitude
            }
        };
        let original = (0..3)
            .map(|p| {
                (0..512)
                    .map(|i| ((i * 257 + p * 17) % 30_000) as i64 - 15_000)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let sources = original
            .iter()
            .map(|values| {
                crate::DensePoly::from_field_evals(
                    9,
                    values.iter().copied().map(field).collect::<Vec<_>>(),
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let plan = CommitInnerPlan {
            num_digits_inner: 3,
            ..plan()
        };
        // Neither map is an involution; the block map crosses polynomial boundaries.
        let blocks = [2usize, 4, 1, 5, 3, 0];
        let columns = [1usize, 4, 0, 5, 2, 3];
        let mut permutation = Vec::with_capacity(6 * 6 * 128);
        for &block in &blocks {
            for &column in &columns {
                permutation.extend((0..128).map(|j| (block * 6 + column) * 128 + j));
            }
        }
        let view = RawSourceViewPlan::new(128, 3, 2, 2, 3)
            .unwrap()
            .with_permutation(permutation)
            .unwrap();
        view.validate_fold_admission().unwrap();
        let point = FullRingRawPoint::new(
            &view,
            (0..view.num_vars())
                .map(|i| {
                    E::from_base_slice(&[F::from_u64(i as u64 + 2), F::from_u64(i as u64 + 7)])
                })
                .collect(),
        )
        .unwrap();
        let partials = source_partials(&sources, &plan, &point).unwrap();
        // Independent Boolean-basis weights and integer base-32 decomposition.
        // Do not use the source visitor, digit accessor, inverse map or cached weights.
        let basis = |coordinates: &[E], index: usize| {
            coordinates
                .iter()
                .enumerate()
                .fold(E::one(), |weight, (bit, &r)| {
                    weight
                        * if (index >> bit) & 1 == 0 {
                            E::one() - r
                        } else {
                            r
                        }
                })
        };
        let coordinates = point.point();
        let mut scalar = E::zero();
        for (raw_block, &physical_block) in blocks.iter().enumerate() {
            let plane0 = partials.plane(raw_block / 2, raw_block % 2, 0).unwrap();
            let plane1 = partials.plane(raw_block / 2, raw_block % 2, 1).unwrap();
            for (j, (&c0, &c1)) in plane0.iter().zip(plane1).enumerate() {
                let mut expected = E::zero();
                for (raw_column, &physical_column) in columns.iter().enumerate() {
                    let index = ((physical_block % 2) * 2 + physical_column / 3) * 128 + j;
                    let mut remaining = original[physical_block / 2][index];
                    let mut digit = 0;
                    for _ in 0..=physical_column % 3 {
                        digit = (remaining + 16).rem_euclid(32) - 16;
                        remaining = (remaining - digit) / 32;
                    }
                    let weight = basis(&coordinates[7..9], raw_column % 3)
                        * basis(&coordinates[9..10], raw_column / 3);
                    expected += weight.mul_base(field(digit));
                }
                assert_eq!(
                    E::from_base_slice(&[c0, c1]),
                    expected,
                    "block {raw_block}, coefficient {j}"
                );
                scalar += expected
                    * basis(&coordinates[..7], j)
                    * basis(&coordinates[10..11], raw_block % 2)
                    * basis(&coordinates[11..], raw_block / 2);
            }
        }
        assert_eq!(point.scalar_from_partials(&partials).unwrap(), scalar);
    }

    #[test]
    fn sparse_one_hot_visitor_matches_dense_original_source() {
        let indices = vec![Some(0u8), None, Some(127), Some(61)];
        let sparse = crate::OneHotPoly::<F, u8>::new(128, indices.clone()).unwrap();
        let mut values = vec![F::zero(); 512];
        for (chunk, index) in indices.into_iter().enumerate() {
            if let Some(index) = index {
                values[chunk * 128 + index as usize] = F::one();
            }
        }
        let dense = crate::DensePoly::from_field_evals(9, values).unwrap();
        let plan = CommitInnerPlan {
            num_digits_inner: 1,
            ..plan()
        };
        let view = RawSourceViewPlan::new(128, 1, 2, 2, 1).unwrap();
        let point = FullRingRawPoint::new(&view, vec![E::from_u64(3); view.num_vars()]).unwrap();
        let sparse = source_partials(&[sparse], &plan, &point).unwrap();
        let dense = source_partials(&[dense], &plan, &point).unwrap();
        assert_eq!(sparse.coefficients(), dense.coefficients());
    }

    fn receiver_point(blocks: usize, positions: usize) -> FullRingRawPoint<E> {
        let view = RawSourceViewPlan::new(128, 1, blocks, positions, 1).unwrap();
        FullRingRawPoint::new(
            &view,
            (0..view.num_vars())
                .map(|i| {
                    E::from_base_slice(&[F::from_u64((i + 2) as u64), F::from_u64((i + 11) as u64)])
                })
                .collect(),
        )
        .unwrap()
    }

    /// Fresh scalar references use the stored prefix and explicit physical zeros,
    /// never the visitor, random-access implementation or ring multiplication.
    fn check_receiver_extent(live: usize, extent: usize, positions: usize) {
        use crate::commitment::{
            compile_commitment_request, BackendKindId, CommitmentRequestCapabilities,
            PolynomialType, ShortNormType,
        };
        use akita_challenges::SparseChallenge;
        use akita_types::full_ring::FullRingCoefficientSource;

        let blocks = extent / (128 * positions);
        let plan = CommitInnerPlan {
            ring_dimension: 128,
            num_live_blocks: blocks,
            n_a: 1,
            num_positions_per_block: positions,
            num_digits_inner: 1,
            log_basis_inner: 5,
        };
        // Nonzero entries span rings, blocks, and the final partially stored ring.
        let mut digits = vec![0i8; live];
        for index in [0, 127, 128, 128 * positions - 1, live / 2, live - 1] {
            digits[index] = (index % 29) as i8 - 15;
        }
        let expected_nonzero = digits
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, digit)| *digit != 0)
            .collect::<Vec<_>>();
        let mut source = RecursiveWitnessFlat::from_i8_digits(digits.clone());
        assert!(source.retain_full_ring_extent(extent / 2).is_err());
        assert!(source.retain_full_ring_extent(extent * 2).is_err());
        source.retain_full_ring_extent(extent).unwrap();
        let storage = source.packed_digits().encoded_bytes().as_ptr();
        assert_eq!(source.packed_digits().len(), live);
        let source = source.align_for_commitment_ring_dim(128).unwrap();
        assert_eq!(source.packed_digits().encoded_bytes().as_ptr(), storage);
        assert_eq!(
            <RecursiveWitnessFlat as RootPolyMeta<F>>::num_vars(&source),
            extent.trailing_zeros() as usize
        );

        let mut visited = Vec::new();
        <RecursiveWitnessFlat as FullRingSource<F>>::visit_full_ring_digits(
            &source,
            &plan,
            &mut |index, value| {
                visited.push((index, value));
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(
            visited,
            expected_nonzero
                .iter()
                .map(|&(index, digit)| (index, F::from_i8(digit)))
                .collect::<Vec<_>>()
        );
        for index in live..extent {
            assert_eq!(
                <RecursiveWitnessFlat as FullRingSource<F>>::full_ring_digit(&source, &plan, index)
                    .unwrap(),
                F::zero()
            );
        }
        assert!(
            <RecursiveWitnessFlat as FullRingSource<F>>::full_ring_digit(&source, &plan, extent)
                .is_err()
        );
        let wrong_plan = CommitInnerPlan {
            num_live_blocks: blocks - 1,
            ..plan
        };
        assert!(
            <RecursiveWitnessFlat as FullRingSource<F>>::visit_full_ring_digits(
                &source,
                &wrong_plan,
                &mut |_, _| Ok(()),
            )
            .is_err()
        );
        assert!(
            <RecursiveWitnessFlat as FullRingSource<F>>::full_ring_digit(&source, &wrong_plan, 0)
                .is_err()
        );

        let point = receiver_point(blocks, positions);
        let partials = source_partials(std::slice::from_ref(&source), &plan, &point).unwrap();
        let mut scalar = E::zero();
        for &(index, digit) in &expected_nonzero {
            let j = index % 128;
            let x = index / 128 % positions;
            let b = index / (128 * positions);
            scalar += (point.coefficient_weights()[j]
                * point.position_weights()[x]
                * point.block_weights()[b])
                .mul_base(F::from_i8(digit));
        }
        assert_eq!(point.scalar_from_partials(&partials).unwrap(), scalar);
        let mut expected_partials = vec![F::zero(); blocks * 256];
        for &(index, digit) in &expected_nonzero {
            let block = index / (positions * 128);
            let position = index / 128 % positions;
            let weighted = point.position_weights()[position].mul_base(F::from_i8(digit));
            for plane in 0..2 {
                expected_partials[block * 256 + plane * 128 + index % 128] +=
                    weighted.base_coefficient(plane);
            }
        }
        assert_eq!(partials.coefficients(), expected_partials);
        for b in live.div_ceil(positions * 128)..blocks {
            assert!(partials
                .plane(0, b, 0)
                .unwrap()
                .iter()
                .all(|value| *value == F::zero()));
            assert!(partials
                .plane(0, b, 1)
                .unwrap()
                .iter()
                .all(|value| *value == F::zero()));
        }

        let challenges = (0..blocks)
            .map(|b| SparseChallenge {
                positions: (0..40)
                    .map(|j| ((j * 3 + b) % 128) as u32)
                    .collect::<Vec<_>>()
                    .into(),
                coeffs: (0..40)
                    .map(|j| {
                        let magnitude = if j < 24 { 1 } else { 2 };
                        if (b + j) % 2 == 0 {
                            magnitude
                        } else {
                            -magnitude
                        }
                    })
                    .collect::<Vec<_>>()
                    .into(),
            })
            .collect::<Vec<_>>();
        let view =
            <RecursiveWitnessFlat as RootOpeningSource<F, 128>>::opening_view(&source).unwrap();
        let folded = view.decompose_fold(&challenges, positions, 1, 5).unwrap();
        assert!(view
            .decompose_fold(&challenges[..blocks - 1], positions, 1, 5)
            .is_err());
        let fold_backend = CpuBackend::<F, E>::for_arithmetic_tests();
        let source_refs = [&source];
        let batch_fold = |challenges: &[SparseChallenge]| {
            OpeningBatchKernel::decompose_fold_batch(
                &fold_backend,
                None,
                <RecursiveWitnessFlat as RootOpeningSource<F, 128>>::opening_batch(&source_refs)
                    .unwrap(),
                DecomposeFoldBatchPlan::Sparse {
                    challenges,
                    num_positions_per_block: positions,
                    num_digits: 1,
                    log_basis: 5,
                },
            )
        };
        assert_eq!(batch_fold(&challenges).unwrap().global, folded);
        assert!(batch_fold(&challenges[..blocks - 1]).is_err());
        let mut expected_response = vec![0i32; positions * 128];
        for &(index, digit) in &expected_nonzero {
            let j = index % 128;
            let x = index / 128 % positions;
            let b = index / (128 * positions);
            for (&shift, &coefficient) in challenges[b].positions.iter().zip(&challenges[b].coeffs)
            {
                let exponent = j + shift as usize;
                let term = i32::from(digit) * i32::from(coefficient);
                expected_response[x * 128 + exponent % 128] +=
                    if exponent < 128 { term } else { -term };
            }
        }
        assert_eq!(folded.centered_coeffs_flat(), expected_response);

        // Exercise the actual source compiler and cached A kernel, retaining every
        // physical output block; the oracle computes only schoolbook products.
        let backend = CpuBackend::<F, E>::for_arithmetic_tests();
        let setup = crate::AkitaProverSetup::<F>::generate_with_capacity(
            extent.trailing_zeros() as usize,
            1,
            akita_params::SetupMatrixCapacity {
                num_field_elements: positions * 128,
            },
        )
        .unwrap();
        let prepared = backend.prepare_setup(&setup).unwrap();
        let capabilities = CommitmentRequestCapabilities::split::<()>(
            BackendKindId::of::<CpuBackend<F, E>>("receiver-test").unwrap(),
            vec![PolynomialType::ShortNorm(
                ShortNormType::new(source.packed_digits().bit_width()).unwrap(),
            )],
        );
        let refs: [&dyn CommitmentSource<F>; 1] = [&source];
        assert!(compile_commitment_request(&wrong_plan, &refs, &capabilities).is_err());
        let resolved = compile_commitment_request(&plan, &refs, &capabilities)
            .unwrap()
            .materialize()
            .unwrap();
        let mut expected_commitment = vec![F::zero(); blocks * 128];
        for &(index, digit) in &expected_nonzero {
            let b = index / (positions * 128);
            let x = index / 128 % positions;
            let j = index % 128;
            for k in 0..128 {
                let term = prepared.matrix().as_field_slice()[x * 128 + k] * F::from_i8(digit);
                let cell = &mut expected_commitment[b * 128 + (j + k) % 128];
                if j + k < 128 {
                    *cell += term;
                } else {
                    *cell -= term;
                }
            }
        }
        for _ in 0..2 {
            let rows = backend
                .commit_resolved_inner_host::<128>(&prepared, &resolved, plan)
                .unwrap();
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].coeffs(), expected_commitment);
        }
        assert!(backend
            .commit_resolved_inner_host::<128>(&prepared, &resolved, wrong_plan)
            .is_err());
        assert_eq!(source.packed_digits().encoded_bytes().as_ptr(), storage);

        // A retained opening owns the same packed allocation after the producer
        // is dropped, and readers/partials keep N rather than truncating to L.
        let mut handle =
            CpuWitnessHandle::from_cpu(source, 128, OperationBinding::unbound()).unwrap();
        handle.retain_full_ring_extent().unwrap();
        let retained = RetainedOpeningSource::<F, E>::Witness(Box::new(handle.snapshot()));
        drop(handle);
        let RetainedOpeningSource::Witness(retained) = retained else {
            panic!("wrong retained source");
        };
        assert_eq!(
            <CpuWitnessHandle as FullRingCoefficientSource<F>>::len(&retained),
            extent
        );
        assert_eq!(
            retained.full_ring_signed_coefficient(live - 1).unwrap(),
            digits[live - 1]
        );
        let last = if live == extent { digits[live - 1] } else { 0 };
        assert_eq!(
            retained.full_ring_signed_coefficient(extent - 1).unwrap(),
            last
        );
        assert!(retained.full_ring_signed_coefficient(extent).is_err());
        assert_eq!(
            retained
                .full_ring_partials::<F, E>(&plan, &point)
                .unwrap()
                .coefficients(),
            partials.coefficients()
        );
    }

    #[test]
    fn terminal_partials_reject_coefficient_and_admissible_source_permutations() {
        let plan = CommitInnerPlan {
            num_digits_inner: 1,
            ..plan()
        };
        let digits = (0..512).map(|i| (i % 31) as i8 - 16).collect::<Vec<_>>();
        let mut witness = CpuWitnessHandle::from_cpu(
            RecursiveWitnessFlat::from_i8_digits(digits.clone()),
            128,
            OperationBinding::unbound(),
        )
        .unwrap();
        witness.retain_full_ring_extent().unwrap();
        let point = receiver_point(2, 2);
        let partials = prepare_terminal_partials::<F, E>(&witness, &plan, &point).unwrap();
        let expected = digits
            .iter()
            .enumerate()
            .fold(E::zero(), |sum, (index, &digit)| {
                let weight =
                    point
                        .point()
                        .iter()
                        .enumerate()
                        .fold(E::one(), |weight, (bit, &r)| {
                            weight
                                * if (index >> bit) & 1 == 0 {
                                    E::one() - r
                                } else {
                                    r
                                }
                        });
                let value = F::from_u64(digit.unsigned_abs() as u64);
                sum + weight.mul_base(if digit < 0 { -value } else { value })
            });
        assert_eq!(point.scalar_from_partials(&partials).unwrap(), expected);
        for width in [1, 128, 256] {
            let mut map = (0..512).collect::<Vec<_>>();
            let stride = if width == 128 { 256 } else { 512 };
            for start in (0..512).step_by(stride) {
                for offset in 0..width {
                    map.swap(start + offset, start + width + offset);
                }
            }
            let view = point.view().clone().with_permutation(map).unwrap();
            if width > 1 {
                view.validate_fold_admission().unwrap();
            }
            let wrong = FullRingRawPoint::new(&view, point.point().to_vec()).unwrap();
            assert!(matches!(
                prepare_terminal_partials::<F, E>(&witness, &plan, &wrong),
                Err(AkitaError::InvalidInput(_))
            ));
        }
    }

    #[test]
    fn selected_receiver_uses_full_extent_without_padded_storage() {
        check_receiver_extent(3_555_328, 1 << 22, 1024);
    }

    #[test]
    fn selected_receiver_handles_partial_rings_and_whole_zero_blocks() {
        check_receiver_extent(5 * 128 + 37, 1024, 1);
    }

    #[test]
    fn selected_unpadded_receiver_preserves_legacy_results() {
        check_receiver_extent(1024, 1024, 1);
        let digits = (0..1024).map(|i| (i % 31) as i8 - 16).collect::<Vec<_>>();
        let legacy = RecursiveWitnessFlat::from_i8_digits(digits);
        let mut selected = legacy.clone();
        selected.retain_full_ring_extent(1024).unwrap();
        let plan = CommitInnerPlan {
            ring_dimension: 128,
            num_live_blocks: 8,
            n_a: 1,
            num_positions_per_block: 1,
            num_digits_inner: 1,
            log_basis_inner: 5,
        };
        let point = receiver_point(8, 1);
        assert_eq!(
            source_partials(std::slice::from_ref(&legacy), &plan, &point)
                .unwrap()
                .coefficients(),
            source_partials(std::slice::from_ref(&selected), &plan, &point)
                .unwrap()
                .coefficients()
        );
    }
}
