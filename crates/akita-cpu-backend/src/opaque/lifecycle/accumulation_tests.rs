//! Structural lease tests only: these do not substitute mock success for any
//! arithmetic proof. Production promotion additionally requires the sealed
//! completion issued by the actual prover strong driver.
use super::super::{BackendIdentity, CpuProofSessionHandle};
use super::*;
use akita_prover::backend::ProofContext;
use std::sync::Arc;

fn fixture(total: usize) -> (Arc<BackendIdentity>, ScopeLease, OperationBinding) {
    let owner = BackendIdentity::new([41; 32]).unwrap();
    let lease = owner.begin_test_scope(vec![4, 1, 1, 1, 1]).unwrap();
    lease.enable_accumulation(total).unwrap();
    let binding = OperationBinding::new(
        owner.backend_id(),
        lease.scope_id(),
        owner.setup_digest(),
        0,
        1,
        lease.clone(),
    );
    (owner, lease, binding)
}
fn context(owner: &BackendIdentity, binding: &OperationBinding) -> ProofContext {
    ProofContext::new(
        owner.backend_id(),
        owner.setup_digest(),
        binding.scope_id(),
        binding.fold_level(),
    )
    .with_occurrence(binding.occurrence())
}
#[test]
fn current_staged_promoted_and_retired_are_distinct() {
    let (owner, lease, parent) = fixture(2);
    let original = context(&owner, &parent);
    lease.admit_commitment(&original.for_group(1), 71).unwrap();
    // Unchanged RAM may use the same immutable ID in two ordered slots.
    lease.admit_commitment(&original.for_group(2), 71).unwrap();
    let destination = lease.stage_successor(&parent, 9).unwrap().unwrap();
    assert_eq!(destination.fold_level(), 0);
    assert_eq!(destination.occurrence(), 1);
    assert!(lease.validate_context(&original).is_ok());
    assert!(lease
        .validate_context(&context(&owner, &destination))
        .is_err());
    assert!(lease.validate_staged_terminal(&destination).is_err());
    assert!(lease.stage_successor(&parent, 10).is_err());
    let producer = parent.for_operation(9);
    let wrong = destination.for_operation(10);
    assert!(lease
        .state
        .operations
        .lock()
        .unwrap()
        .promote(&producer, &wrong)
        .is_err());
    lease
        .state
        .operations
        .lock()
        .unwrap()
        .promote(&producer, &destination)
        .unwrap();
    assert!(lease.validate_context(&original).is_err());
    assert!(lease
        .validate_context(&context(&owner, &destination))
        .is_ok());
    assert!(lease.validate_reclamation(&parent).is_ok());
    // Scope ownership is liveness-only; it must not request old root occurrence 0.
    let session = CpuProofSessionHandle::new(Arc::clone(&owner), lease.clone());
    assert!(session.validate_owner(&owner).is_ok());
    let current = context(&owner, &destination);
    lease.admit_commitment(&current.for_group(1), 71).unwrap();
    assert!(lease.admit_commitment(&current.for_group(1), 72).is_err());
    assert!(lease.validate_finished_accumulation().is_err());
}
#[test]
fn exact_root_count_reaches_only_final_normalization() {
    for total in 1..=16 {
        let (owner, lease, mut parent) = fixture(total);
        for index in 0..total {
            let destination = lease
                .stage_successor(&parent, index as u128 + 20)
                .unwrap()
                .unwrap();
            lease
                .state
                .operations
                .lock()
                .unwrap()
                .promote(&parent.for_operation(index as u128 + 20), &destination)
                .unwrap();
            parent = destination;
            if index + 1 < total {
                assert_eq!(parent.fold_level(), 0);
                assert!(lease.context_occurrence(0).is_ok());
            }
        }
        assert_eq!(parent.fold_level(), 1);
        assert!(lease.validate_context(&context(&owner, &parent)).is_err());
        assert!(lease
            .validate_occurrence(parent.occurrence(), parent.fold_level())
            .is_ok());
        assert!(lease.context_occurrence(1).is_err());
        assert!(lease.stage_successor(&parent, 999).is_err());
        assert!(lease.validate_finished_accumulation().is_err());
        let operations = lease.state.operations.lock().unwrap();
        let run = operations.accumulation.as_ref().unwrap();
        assert_eq!(run.completed, total);
        assert_eq!(run.phase, Phase::FinalNormalization);
    }
}
#[test]
fn only_exact_staged_terminal_can_publish_and_never_compute() {
    let (owner, lease, parent) = fixture(1);
    // Isolate the final shrinking edge; no arithmetic acceptance is asserted.
    {
        let mut operations = lease.state.operations.lock().unwrap();
        let run = operations.accumulation.as_mut().unwrap();
        run.completed = 1;
        run.level = 3;
        run.occurrence = 3;
        run.phase = Phase::Shrink;
    }
    let parent = parent.for_level_operation(3, 21).with_occurrence(3);
    let terminal = lease.stage_successor(&parent, 22).unwrap().unwrap();
    assert!(lease.validate_staged_terminal(&terminal).is_ok());
    assert!(lease
        .validate_staged_terminal(&terminal.for_operation(23))
        .is_err());
    assert!(lease.validate_context(&context(&owner, &terminal)).is_err());
    assert!(lease.validate_reclamation(&terminal).is_err());
    lease
        .state
        .operations
        .lock()
        .unwrap()
        .promote(&parent.for_operation(22), &terminal)
        .unwrap();
    assert!(lease.validate_staged_terminal(&terminal).is_err());
    assert!(lease.validate_context(&context(&owner, &terminal)).is_ok());
    assert!(lease.validate_context(&context(&owner, &parent)).is_err());
    assert!(lease.stage_successor(&terminal, 24).is_err());
    assert!(lease.validate_finished_accumulation().is_err());
    owner.abort_scope(&lease);
    assert!(lease.validate_context(&context(&owner, &terminal)).is_err());
    assert!(lease.validate_reclamation(&parent).is_err());
}
#[test]
fn ordinary_scopes_keep_inventory_semantics_and_reject_occurrence_relabeling() {
    let owner = BackendIdentity::new([42; 32]).unwrap();
    let lease = owner.begin_test_scope(vec![4, 1, 1]).unwrap();
    let context = ProofContext::new(
        owner.backend_id(),
        owner.setup_digest(),
        lease.scope_id(),
        2,
    );
    assert!(lease.validate_context(&context).is_ok());
    assert!(lease.validate_context(&context.with_occurrence(1)).is_err());
    assert!(lease.validate_finished_accumulation().is_ok());
    assert!(lease.enable_accumulation(0).is_err());
    assert!(lease.enable_accumulation(17).is_err());
}
