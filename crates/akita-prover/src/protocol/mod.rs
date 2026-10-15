//! Prover-side protocol orchestration helpers.

mod fold_grind;
mod prove;
mod ring_relation;
mod ring_switch;

pub use akita_types::RingRelationInstance;
pub use prove::full_ring_root::{
    begin_full_ring_accumulation, begin_full_ring_reduction, full_ring_raw_claim_binding,
    FullRingFreshProverObligations, FullRingRawClaim, FullRingReductionSites,
    PendingFullRingProver,
};
pub use prove::full_ring_strong::{
    prove_full_ring_final_receiver_domain, prove_full_ring_normalization,
    prove_full_ring_source_domain, prove_full_ring_strong,
};
pub use prove::{
    batched_prove, begin_batched_reduction, PendingNativeProver, ProveLevelOutput,
    RecursiveSuffixOutcome, SuffixProverState,
};
pub use ring_relation::{
    validate_chunked_witness_cfg, validate_prepared_relation_groups, RingRelationProver,
};
pub use ring_switch::RingSwitchOutput;
