//! Verifier-facing API surface for the Akita PCS.
//!
//! This crate owns verifier replay for already-selected Akita proof schedules.
//! It deliberately avoids prover polynomial backends, commit hints, recursive
//! witness construction, and planner search.
//!
//! Downstream verifier-only integrations should pair this crate with
//! `akita-types` for proof/setup/claim shapes and `akita-config` for concrete
//! runtime schedule policy. The broader `akita-pcs` crate is an umbrella for
//! end-to-end examples and also re-exports prover-facing APIs.
//!
//! Ordinary v8 replay requires an externally approved `InstalledRegistry` and
//! explicit `InheritedAugmentation`. Replay checks its full endpoint binding
//! against the resolved producer profiles, views, ordered commitments, points,
//! and values before creating the channel. It binds literal inherited history,
//! but does not establish frontend/range/query validity: an outer AQRA verifier
//! must replay those obligations before invoking this final PCS conjunct.
//! Empty inherited history is an explicit standalone PCS statement only.
//! Acceptance includes the complete shrinking suffix, terminal, finite grammar,
//! grinding-plan completion, and exact proof EOF; a pending root is not acceptance.
//!
//! # Public surface
//!
//! Only the entry points actually consumed by downstream crates are public.
//! Replay internals (per-level/root verifiers, ring-switch replay, the stage-2
//! verifier, schedule context, prepared-claim shapes) are crate-private. If
//! you need to reach into them, that's a signal to either move the consumer
//! into this crate or expose a narrower entry point.
//!
//! Two replay primitives are public but are not part of the verifier's
//! intended downstream API: [`RelationMatrixEvaluator`], which the
//! `benchmark-support` relation-evaluator bench drives directly, and
//! [`Stage1Verifier`], which downstream range-proof lanes reuse.

#[cfg(any(test, feature = "benchmark-support"))]
mod benchmark_support;
mod coefficient_packing_relation;
mod fold;
pub mod full_ring;
pub mod full_ring_root;
pub use full_ring_root::{
    begin_full_ring_accumulation_verification, begin_full_ring_verification,
    FullRingPendingBinding, PendingFullRingVerifier,
};
pub mod full_ring_strong;
pub use full_ring_strong::{
    verify_full_ring_final_receiver_domain, verify_full_ring_normalization,
    verify_full_ring_source_domain, verify_full_ring_strong,
};
mod prepared_cache;
mod relation;
mod setup_contribution;
mod stages;
mod terminal;
mod verifier;

#[cfg(any(test, feature = "benchmark-support"))]
pub use benchmark_support::{evaluation_trace_benchmark_case, EvaluationTraceBenchmarkCase};
#[cfg(feature = "benchmark-support")]
pub use benchmark_support::{
    relation_evaluator_benchmark_case, relation_evaluator_benchmark_case_with_chunks,
    RelationEvaluatorBenchmarkCase,
};
pub use prepared_cache::build_riscv64_terminal_ntt_cache;
pub use relation::RelationMatrixEvaluator;
pub use setup_contribution::{DirectScan, PreparedCoefficientFunctional, SetupIndexWeightMle};
pub use stages::stage1::Stage1Verifier;
pub use terminal::channel::verify_live_full_ring_terminal;
pub use terminal::full_ring::verify_full_ring_terminal_suffix;
pub use verifier::AkitaVerifier;
