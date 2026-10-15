//! Native openings and aggregate extension-opening reduction.
use super::{
    FoldProbeOutcome, OpeningSource, PreparedGroupOpening, ProofContext, ProverHandleFamily,
};
use jolt_poly::UnivariatePoly;

use akita_error::AkitaError;
use akita_params::OpeningClaimsLayout;
use jolt_field::{CanonicalEncoding, Field};

pub struct EorGroupRequest<'a, E: Field, C, W> {
    pub source: OpeningSource<'a, C, W>,
    pub point: &'a [E],
    pub ring_dimension: usize,
}
pub struct PreparedEor<E: Field, H> {
    pub openings: Vec<E>,
    pub proof_partials: Vec<E>,
    pub handle: H,
}
/// One group's native opening preparation at the current fold level.
pub struct GroupOpeningRequest<'a, E: Field, C, W> {
    pub context: ProofContext,
    pub source: OpeningSource<'a, C, W>,
    pub plan: super::ValidatedRecursiveGroupOpeningPlan<'a, E>,
}
pub trait OpaqueOpeningKernel<F: Field + CanonicalEncoding, E: Field>:
    ProverHandleFamily<F, E> + super::ProofScopeConsumer
{
    /// Prepare every group, returning results in request order regardless of execution order.
    fn prepare_openings(
        &self,
        session: &Self::ProofSessionHandle,
        requests: &[GroupOpeningRequest<'_, E, Self::CommitmentHandle, Self::WitnessHandle>],
    ) -> Result<Vec<PreparedGroupOpening<E, Self::PreparedOpeningHandle>>, AkitaError>;
    fn probe_opening_fold(
        &self,
        context: &ProofContext,
        opening: &Self::PreparedOpeningHandle,
        plan: &crate::backend::ValidatedFoldProbePlan<'_>,
    ) -> Result<FoldProbeOutcome<Self::AcceptedFoldHandle>, AkitaError>;
}

/// Full ambient-ring openings over retained base-field sources. This capability
/// deliberately does not imply tensor reduction or a ring-subfield encoding.
pub trait OpaqueFullRingOpeningKernel<F: Field + CanonicalEncoding, E: Field>:
    ProverHandleFamily<F, E> + super::ProofScopeConsumer
{
    fn full_ring_source_id(
        &self,
        source: &Self::CommitmentHandle,
    ) -> Result<akita_types::FullRingSourceId, AkitaError>;

    /// Compare actual immutable committed instances, not just contract IDs or
    /// public payloads. Both handles must belong to this installed backend.
    /// Identical incoming/outgoing RAM is valid; no inequality is required.
    fn same_full_ring_source(
        &self,
        left: &Self::CommitmentHandle,
        right: &Self::CommitmentHandle,
    ) -> Result<bool, AkitaError>;

    fn full_ring_witness_source_id(
        &self,
        source: &Self::WitnessHandle,
    ) -> Result<akita_types::FullRingSourceId, AkitaError>;

    /// Borrow an already admitted original source only for this active operation.
    fn with_full_ring_source<T>(
        &self,
        session: &Self::ProofSessionHandle,
        context: &ProofContext,
        source: &Self::CommitmentHandle,
        use_source: impl FnOnce(
            &dyn akita_types::full_ring::FullRingCoefficientSource<F>,
        ) -> Result<T, AkitaError>,
    ) -> Result<T, AkitaError>;

    fn prepare_full_ring_opening(
        &self,
        session: &Self::ProofSessionHandle,
        context: &ProofContext,
        source: OpeningSource<'_, Self::CommitmentHandle, Self::WitnessHandle>,
        point: akita_types::full_ring::FullRingRawPoint<E>,
    ) -> Result<PreparedGroupOpening<E, Self::PreparedOpeningHandle>, AkitaError>;

    fn probe_full_ring_opening_fold(
        &self,
        context: &ProofContext,
        opening: &Self::PreparedOpeningHandle,
        plan: &super::ValidatedFoldProbePlan<'_>,
    ) -> Result<FoldProbeOutcome<Self::AcceptedFoldHandle>, AkitaError>;
}
pub trait OpaqueEorKernel<F: Field + CanonicalEncoding, E: Field>:
    ProverHandleFamily<F, E> + super::ProofScopeConsumer
{
    fn prepare_eor(
        &self,
        session: &Self::ProofSessionHandle,
        context: &ProofContext,
        layout: &OpeningClaimsLayout,
        groups: &[EorGroupRequest<'_, E, Self::CommitmentHandle, Self::WitnessHandle>],
    ) -> Result<PreparedEor<E, Self::EorPreparationHandle>, AkitaError>;
    fn begin_eor(
        &self,
        preparation: Self::EorPreparationHandle,
        eta: &[E],
        coefficients: &[E],
    ) -> Result<(E, Self::EorSessionHandle), AkitaError>;
    fn eor_round(
        &self,
        session: &mut Self::EorSessionHandle,
        round: usize,
        claim: E,
    ) -> Result<UnivariatePoly<E>, AkitaError>;
    fn bind_eor_round(
        &self,
        session: &mut Self::EorSessionHandle,
        round: usize,
        challenge: E,
    ) -> Result<(), AkitaError>;
    fn finish_eor(&self, session: Self::EorSessionHandle) -> Result<Vec<E>, AkitaError>;
}
