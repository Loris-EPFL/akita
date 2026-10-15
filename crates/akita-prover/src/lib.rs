//! Backend-neutral protocol sequencing, transcript handling, and proof assembly.
//!
//! Arithmetic implementations own their source storage and mutable proof state.
//! This crate exchanges opaque handles and scheduled public messages with them.
//!
//! Protocol sequencing reports every failure as an `AkitaError`. Outside tests
//! the crate has no explicit panic, and the lints below keep it that way.
//!
//! Ordinary v8 callers supply a verifier-approved finite `InstalledRegistry`
//! and explicit `InheritedAugmentation`. The endpoint binding is the complete
//! `akita_types::encode_opening_endpoint_binding` output, not a digest. The
//! literal inherited history is included in every subsequent indexed key;
//! its frontend/range/query validity belongs to the outer verifier. Standalone
//! PCS callers may explicitly supply empty history, without an AQRA claim.
//! Original RAM opening views retain source arity 16 and public query arity 21;
//! only the root accepts them. Recursive/setup/terminal sources stay decoded.

#![cfg_attr(
    not(test),
    deny(
        clippy::expect_used,
        clippy::unwrap_used,
        clippy::panic,
        clippy::unreachable,
        clippy::todo,
        clippy::unimplemented
    )
)]

pub mod backend;
mod opening;
pub mod protocol;
mod setup;

pub use backend::{
    AcceptedFoldHandle, CommitmentHandleMetadata, CommitmentMaterialMetadata,
    CommitmentRelationMaterial, FullRingProofAdmission, FullRingProverBackend,
    NextWitnessBindingMessage, OpaqueEorKernel, OpaqueFullRingOpeningKernel,
    OpaqueFullRingTerminalKernel, OpaqueFullRingWitnessKernel, OpaqueOpeningKernel,
    OpaqueProverConsumer, OpaqueRecursiveWitnessBuildKernel, OpaqueRelationWitnessKernel,
    OpaqueResourceReleaseKernel, OpaqueStage1Kernel, OpaqueStage2Kernel, OpaqueStage3Kernel,
    OpaqueTerminalFoldKernel, OpaqueWitnessCommitKernel, OpaqueWitnessOpeningKernel, OpeningSource,
    OperationCtx, PreparedGroupOpening, PreparedRelationHandle, ProofAdmission, ProofContext,
    ProofScope, ProofScopeConsumer, ProofScopeId, ProverBackend, ProverHandleFamily,
    RecursiveWitnessBuildStart, RecursiveWitnessFoldInput, RecursiveWitnessHandle,
    RecursiveWitnessManifest, RelationWitnessMetadata, SourceMetadata,
    TerminalCommitmentMaterialKernel, TerminalTFieldsMessage, WitnessCommitmentOutput,
};
pub use opening::{ProverOpeningData, SelectedProverOpeningData};
pub use protocol::{
    batched_prove, begin_batched_reduction, begin_full_ring_accumulation,
    begin_full_ring_reduction, full_ring_raw_claim_binding, prove_full_ring_final_receiver_domain,
    prove_full_ring_normalization, prove_full_ring_source_domain, prove_full_ring_strong,
    FullRingFreshProverObligations, FullRingRawClaim, FullRingReductionSites,
    PendingFullRingProver, PendingNativeProver, ProveLevelOutput, RecursiveSuffixOutcome,
    RingRelationInstance, RingRelationProver, RingSwitchOutput, SuffixProverState,
};
pub use setup::{PreparedSetupPrefix, SetupPrefixProverRegistry};
