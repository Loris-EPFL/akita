use super::{
    AcceptedFoldHandle, FoldProbeOutcome, PreparedGroupOpening, PreparedRelationHandle,
    RecursiveWitnessBuildStart, RecursiveWitnessFoldInput, RecursiveWitnessHandle,
    WitnessCommitmentOutput,
};
use akita_error::AkitaError;
use jolt_field::{CanonicalEncoding, Field};
use jolt_poly::UnivariatePoly;

/// Coherent opaque handles for a backend implementation.
pub trait ProverHandleFamily<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    type CommitmentHandle: super::CommitmentHandleMetadata + Clone + Send + Sync + 'static;
    type EorPreparationHandle: Send;
    type EorSessionHandle: Send;
    type PreparedOpeningHandle: Send + 'static;
    type CommitmentMaterialHandle: crate::backend::CommitmentRelationMaterial<F> + Send + 'static;
    type AcceptedFoldHandle: AcceptedFoldHandle;
    type AcceptedTerminalFoldHandle: Send + 'static;
    type WitnessBuildHandle: Send + 'static;
    type WitnessHandle: RecursiveWitnessHandle;
    type RelationHandle: Send + 'static;
    type Stage1SessionHandle: Send + 'static;
    type Stage2SessionHandle: Send + 'static;
}

/// An identity-bearing consumer that operates on handles within a proof scope.
pub trait OpaqueProverConsumer<F, E>: ProverHandleFamily<F, E> + super::ProofScopeConsumer
where
    F: Field + CanonicalEncoding,
    E: Field,
{
}

/// Admission and canonical encoding of the terminal public response.
pub trait OpaqueTerminalFoldKernel<F, E>: OpaqueProverConsumer<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    fn probe_terminal_fold(
        &self,
        witness_handle: &Self::WitnessHandle,
        plan: &crate::backend::ValidatedTerminalFoldProbePlan<'_>,
    ) -> Result<FoldProbeOutcome<Self::AcceptedTerminalFoldHandle>, AkitaError>;

    fn encode_terminal_fold(
        &self,
        terminal_fold_handle: Self::AcceptedTerminalFoldHandle,
        plan: &crate::backend::ValidatedTerminalZEncodingPlan,
    ) -> Result<Vec<u8>, AkitaError>;
}

/// Two-phase construction of a complete private recursive witness.
pub trait OpaqueRecursiveWitnessBuildKernel<F, E>: OpaqueProverConsumer<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    #[allow(clippy::too_many_arguments)]
    fn begin_recursive_witness(
        &self,
        session: &Self::ProofSessionHandle,
        context: &super::ProofContext,
        prepared_opening_handles: &[Self::PreparedOpeningHandle],
        commitment_material_handles: Vec<Self::CommitmentMaterialHandle>,
        level: &akita_params::CommittedGroupParams,
        opening_batch: &akita_params::OpeningClaimsLayout,
        relation_rhs_layout: &akita_params::RelationRhsLayout,
        group_commitments: &[akita_types::RingVec<F>],
    ) -> Result<RecursiveWitnessBuildStart<F, E, Self::WitnessBuildHandle>, AkitaError>;

    fn finish_recursive_witness(
        &self,
        build_handle: Self::WitnessBuildHandle,
        fold_inputs: Vec<RecursiveWitnessFoldInput<Self::AcceptedFoldHandle>>,
        relation: &akita_types::RingRelationInstance<F>,
        plan: &crate::backend::ValidatedRecursiveWitnessPlan<'_, F>,
    ) -> Result<Self::WitnessHandle, AkitaError>;
}

/// Exact coefficient-row witness construction using the native Z/E/T/F/H
/// producers, without manufacturing a legacy ring-switch instance.
pub trait OpaqueFullRingWitnessKernel<F, E>: OpaqueRecursiveWitnessBuildKernel<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    fn finish_full_ring_recursive_witness(
        &self,
        build_handle: Self::WitnessBuildHandle,
        fold_inputs: Vec<RecursiveWitnessFoldInput<Self::AcceptedFoldHandle>>,
        commitment_ring_dimension: usize,
    ) -> Result<Self::WitnessHandle, AkitaError>;

    /// Visit the same retained canonical source in physical order, including
    /// the authenticated zero suffix. No decoded/redecomposed replacement.
    fn visit_full_ring_witness(
        &self,
        witness: &Self::WitnessHandle,
        visitor: &mut dyn FnMut(usize, F) -> Result<(), AkitaError>,
    ) -> Result<(), AkitaError>;

    fn full_ring_witness_source<'a>(
        &self,
        witness: &'a Self::WitnessHandle,
    ) -> Result<&'a dyn akita_types::full_ring::FullRingCoefficientSource<F>, AkitaError>;

    /// Construct exact row weights only from the admitted native role matrices,
    /// original fold inputs and complete retained witness layout.
    fn with_full_ring_relation<T>(
        &self,
        witness: &Self::WitnessHandle,
        use_relation: impl FnOnce(
            &akita_types::full_ring::CoefficientRelationPlan<'_, F, E>,
        ) -> Result<T, AkitaError>,
    ) -> Result<T, AkitaError>;

    /// Promote exactly the material coupled to a completed strong reduction.
    /// The producing witness remains current until this consuming operation.
    fn complete_full_ring_reduction(
        &self,
        witness: &mut Self::WitnessHandle,
        material: &Self::CommitmentMaterialHandle,
        binding: &super::NextWitnessBindingMessage<F>,
        completion: super::FullRingCompletion,
    ) -> Result<super::ProofContext, AkitaError>;

    /// Complete a once-only selected normalization or terminal phase.
    fn complete_full_ring_phase(
        &self,
        session: &Self::ProofSessionHandle,
        completion: super::FullRingCompletion,
    ) -> Result<(), AkitaError>;
}

/// Direct full-ring terminal over the already-created A-only source.
pub trait OpaqueFullRingTerminalKernel<F, E>: OpaqueProverConsumer<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    fn prepare_full_ring_terminal(
        &self,
        session: &Self::ProofSessionHandle,
        context: &super::ProofContext,
        witness: &Self::WitnessHandle,
        point: akita_types::full_ring::FullRingRawPoint<E>,
    ) -> Result<akita_types::full_ring::FullRingPartials<F>, AkitaError>;

    fn encode_full_ring_terminal_response(
        &self,
        witness: &Self::WitnessHandle,
        challenges: &akita_challenges::Challenges,
    ) -> Result<Vec<u8>, AkitaError>;
}

/// Commitment transition that consumes and replaces a witness handle.
pub trait OpaqueWitnessCommitKernel<F, E>: OpaqueProverConsumer<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    fn commit_witness(
        &self,
        witness_handle: Self::WitnessHandle,
        plan: &crate::backend::ValidatedRecursiveWitnessCommitPlan,
    ) -> Result<
        WitnessCommitmentOutput<F, Self::WitnessHandle, Self::CommitmentMaterialHandle>,
        AkitaError,
    >;

    /// Transfer a fully consumed current-level witness to the next fold level.
    fn advance_witness_level(
        &self,
        witness_handle: &mut Self::WitnessHandle,
    ) -> Result<(), AkitaError>;
}

/// Prepare private packed relation state from a recursive witness handle.
pub trait OpaqueRelationWitnessKernel<F, E>: OpaqueProverConsumer<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    fn prepare_relation_witness(
        &self,
        witness_handle: &Self::WitnessHandle,
        plan: &crate::backend::ValidatedRelationWitnessPlan,
    ) -> Result<PreparedRelationHandle<Self::RelationHandle>, AkitaError>;
}

/// Stage 1 interactive kernel over a private mutable session.
pub trait OpaqueStage1Kernel<F, E>: OpaqueProverConsumer<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    fn begin_stage1(
        &self,
        relation_handle: &Self::RelationHandle,
        plan: &crate::backend::ValidatedStage1Plan<E>,
    ) -> Result<Self::Stage1SessionHandle, AkitaError>;

    fn stage1_round_polynomial(
        &self,
        session_handle: &mut Self::Stage1SessionHandle,
        step: crate::backend::Stage1Step,
        round: usize,
        previous_claim: E,
    ) -> Result<crate::backend::Stage1RoundPolynomial<E>, AkitaError>;

    fn bind_stage1_challenge(
        &self,
        session_handle: &mut Self::Stage1SessionHandle,
        step: crate::backend::Stage1Step,
        round: usize,
        challenge: E,
    ) -> Result<(), AkitaError>;

    fn stage1_public_transition(
        &self,
        session_handle: &mut Self::Stage1SessionHandle,
        step: crate::backend::Stage1Step,
    ) -> Result<crate::backend::Stage1PublicTransition<E>, AkitaError>;

    fn bind_stage1_batch_challenge(
        &self,
        session_handle: &mut Self::Stage1SessionHandle,
        transition: crate::backend::Stage1Transition,
        challenge: E,
    ) -> Result<(), AkitaError>;

    fn finish_stage1(
        &self,
        session_handle: Self::Stage1SessionHandle,
    ) -> Result<crate::backend::Stage1FinalClaims<E>, AkitaError>;
}

/// Stage 2 interactive kernel over a private mutable session.
pub trait OpaqueStage2Kernel<F, E>: OpaqueProverConsumer<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    fn begin_stage2(
        &self,
        relation_handle: Self::RelationHandle,
        plan: crate::backend::ValidatedRelationSessionPlan<'_, F, E>,
    ) -> Result<Self::Stage2SessionHandle, AkitaError>;

    fn stage2_input_claim(
        &self,
        session_handle: &Self::Stage2SessionHandle,
    ) -> Result<E, AkitaError>;

    fn stage2_num_rounds(
        &self,
        session_handle: &Self::Stage2SessionHandle,
    ) -> Result<usize, AkitaError>;

    fn stage2_round_polynomial(
        &self,
        session_handle: &mut Self::Stage2SessionHandle,
        round: usize,
        previous_claim: E,
    ) -> Result<UnivariatePoly<E>, AkitaError>;

    fn bind_stage2_challenge(
        &self,
        session_handle: &mut Self::Stage2SessionHandle,
        round: usize,
        challenge: E,
    ) -> Result<(), AkitaError>;

    fn finish_stage2(
        &self,
        session_handle: Self::Stage2SessionHandle,
    ) -> Result<crate::backend::RelationWitnessFinalClaims<E>, AkitaError>;
}

/// Witness opening and extension-opening-reduction lifecycle.
pub trait OpaqueWitnessOpeningKernel<F, E>: OpaqueProverConsumer<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    /// Prepare a native-ring opening without exposing the witness storage.
    fn prepare_native_witness_opening(
        &self,
        _witness_handle: &Self::WitnessHandle,
        _plan: &crate::backend::ValidatedRecursiveGroupOpeningPlan<'_, E>,
    ) -> Result<PreparedGroupOpening<E, Self::PreparedOpeningHandle>, AkitaError> {
        Err(AkitaError::InvalidInput(
            "consumer does not support native recursive-witness opening".into(),
        ))
    }

    /// Consume a retained native opening as terminal evaluation-trace rows.
    fn terminal_native_witness_opening(
        &self,
        _opening_handle: Self::PreparedOpeningHandle,
    ) -> Result<Vec<akita_types::RingVec<F>>, AkitaError> {
        Err(AkitaError::InvalidInput(
            "consumer does not support terminal native witness opening".into(),
        ))
    }
}

/// Explicit early release for a long-lived recursive witness handle.
pub trait OpaqueResourceReleaseKernel<F, E>: OpaqueProverConsumer<F, E>
where
    F: Field + CanonicalEncoding,
    E: Field,
{
    fn release_witness_handle(&self, witness_handle: Self::WitnessHandle)
        -> Result<(), AkitaError>;
}
