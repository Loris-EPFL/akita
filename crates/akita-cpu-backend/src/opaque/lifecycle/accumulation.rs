//! Opt-in occurrence cursor. Installed template indices never identify a run step.
use super::{ProofOperations, ScopeLease};
use crate::opaque::OperationBinding;
use akita_error::AkitaError;
use akita_prover::backend::{FullRingCompletion, FullRingCompletionKind};
#[cfg(test)]
#[path = "accumulation_tests.rs"]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    Update,
    FinalNormalization,
    Shrink,
    Terminal,
    Complete,
}
#[derive(Clone, Copy)]
struct Staged {
    occurrence: u32,
    level: u32,
    commitment: u128,
    phase: Phase,
}
pub(super) struct AccumulationState {
    updates: usize,
    completed: usize,
    occurrence: u32,
    level: u32,
    phase: Phase,
    staged: Option<Staged>,
}
fn invalid() -> AkitaError {
    AkitaError::InvalidInput(
        "inactive, staged, retired or out-of-phase accumulation occurrence".into(),
    )
}
impl ProofOperations {
    pub(super) fn validate_current(&self, occurrence: u32, level: u32) -> Result<(), AkitaError> {
        if let Some(run) = &self.accumulation {
            if (run.occurrence, run.level) != (occurrence, level) || run.phase == Phase::Complete {
                return Err(invalid());
            }
        } else if occurrence != 0 {
            return Err(invalid());
        }
        Ok(())
    }
    fn promote(
        &mut self,
        parent: &OperationBinding,
        destination: &OperationBinding,
    ) -> Result<(), AkitaError> {
        self.validate_current(parent.occurrence(), parent.fold_level())?;
        let run = self.accumulation.as_mut().ok_or_else(invalid)?;
        let staged = run.staged.ok_or_else(invalid)?;
        if parent.operation_id() != staged.commitment
            || (staged.occurrence, staged.level, staged.commitment)
                != (
                    destination.occurrence(),
                    destination.fold_level(),
                    destination.operation_id(),
                )
        {
            return Err(invalid());
        }
        if run.phase == Phase::Update {
            run.completed += 1;
        }
        run.occurrence = staged.occurrence;
        run.level = staged.level;
        run.phase = staged.phase;
        run.staged = None;
        self.commitments.clear();
        self.commitments
            .insert((staged.level, 0), staged.commitment);
        Ok(())
    }
}
impl ScopeLease {
    pub(crate) fn enable_accumulation(&self, updates: usize) -> Result<(), AkitaError> {
        self.validate_live(self.scope_id())?;
        if !(1..=16).contains(&updates) {
            return Err(invalid());
        }
        let mut operations = self.state.operations.lock().map_err(|_| invalid())?;
        if operations.accumulation.is_some() || !operations.commitments.is_empty() {
            return Err(invalid());
        }
        operations.accumulation = Some(AccumulationState {
            updates,
            completed: 0,
            occurrence: 0,
            level: 0,
            phase: Phase::Update,
            staged: None,
        });
        Ok(())
    }
    pub(crate) fn is_accumulation(&self) -> Result<bool, AkitaError> {
        Ok(self
            .state
            .operations
            .lock()
            .map_err(|_| invalid())?
            .accumulation
            .is_some())
    }
    pub(crate) fn validate_occurrence(
        &self,
        occurrence: u32,
        level: u32,
    ) -> Result<(), AkitaError> {
        self.validate_live(self.scope_id())?;
        self.state
            .operations
            .lock()
            .map_err(|_| invalid())?
            .validate_current(occurrence, level)
    }
    pub(crate) fn context_occurrence(&self, level: u32) -> Result<u32, AkitaError> {
        self.validate_live(self.scope_id())?;
        let operations = self.state.operations.lock().map_err(|_| invalid())?;
        let Some(run) = &operations.accumulation else {
            return Ok(0);
        };
        if run.level != level || matches!(run.phase, Phase::FinalNormalization | Phase::Complete) {
            return Err(invalid());
        }
        Ok(run.occurrence)
    }
    pub(crate) fn stage_successor(
        &self,
        parent: &OperationBinding,
        commitment: u128,
    ) -> Result<Option<OperationBinding>, AkitaError> {
        self.validate_live(parent.scope_id())?;
        let mut operations = self.state.operations.lock().map_err(|_| invalid())?;
        operations.validate_current(parent.occurrence(), parent.fold_level())?;
        let Some(run) = &mut operations.accumulation else {
            return Ok(None);
        };
        if run.staged.is_some() {
            return Err(invalid());
        }
        let (level, phase) = match run.phase {
            Phase::Update if run.completed + 1 < run.updates => (0, Phase::Update),
            Phase::Update if run.completed + 1 == run.updates => (1, Phase::FinalNormalization),
            Phase::Shrink if run.level < 3 => (run.level + 1, Phase::Shrink),
            Phase::Shrink if run.level == 3 => (4, Phase::Terminal),
            _ => return Err(invalid()),
        };
        let occurrence = run.occurrence.checked_add(1).ok_or_else(invalid)?;
        run.staged = Some(Staged {
            occurrence,
            level,
            commitment,
            phase,
        });
        Ok(Some(
            parent
                .for_level_operation(level, commitment)
                .with_occurrence(occurrence)
                .with_group(Some(0)),
        ))
    }
    pub(crate) fn validate_staged_terminal(
        &self,
        binding: &OperationBinding,
    ) -> Result<(), AkitaError> {
        self.validate_live(binding.scope_id())?;
        let operations = self.state.operations.lock().map_err(|_| invalid())?;
        let run = operations.accumulation.as_ref().ok_or_else(invalid)?;
        let staged = run.staged.ok_or_else(invalid)?;
        if staged.phase != Phase::Terminal
            || (staged.occurrence, staged.level, staged.commitment)
                != (
                    binding.occurrence(),
                    binding.fold_level(),
                    binding.operation_id(),
                )
        {
            return Err(invalid());
        }
        Ok(())
    }
    pub(crate) fn promote(
        &self,
        parent: &OperationBinding,
        destination: &OperationBinding,
        completion: &FullRingCompletion,
    ) -> Result<(), AkitaError> {
        self.validate_context(completion.context())?;
        if !matches!(completion.kind(), FullRingCompletionKind::Reduction { .. })
            || completion.context().occurrence() != parent.occurrence()
            || completion.context().fold_level() != parent.fold_level()
        {
            return Err(invalid());
        }
        self.state
            .operations
            .lock()
            .map_err(|_| invalid())?
            .promote(parent, destination)
    }
    pub(crate) fn complete_phase(&self, completion: FullRingCompletion) -> Result<(), AkitaError> {
        self.validate_completion_context(completion.context())?;
        let mut operations = self.state.operations.lock().map_err(|_| invalid())?;
        let Some(run) = &mut operations.accumulation else {
            return Ok(());
        };
        if run.staged.is_some() {
            return Err(invalid());
        }
        run.phase = match (run.phase, completion.kind()) {
            (Phase::FinalNormalization, FullRingCompletionKind::FinalNormalization)
                if run.completed == run.updates =>
            {
                Phase::Shrink
            }
            (Phase::Terminal, FullRingCompletionKind::Terminal) => Phase::Complete,
            _ => return Err(invalid()),
        };
        Ok(())
    }
    pub(crate) fn validate_reclamation(
        &self,
        binding: &OperationBinding,
    ) -> Result<(), AkitaError> {
        self.validate_live(binding.scope_id())?;
        let operations = self.state.operations.lock().map_err(|_| invalid())?;
        if let Some(run) = &operations.accumulation {
            // Only actual prior occurrences can be dropped; this returns no reader.
            if binding.occurrence() <= run.occurrence {
                return Ok(());
            }
            return Err(invalid());
        }
        drop(operations);
        self.validate(
            binding.scope_id(),
            binding.fold_level(),
            binding.group_index(),
        )
    }
    pub(super) fn validate_finished_accumulation(&self) -> Result<(), AkitaError> {
        let operations = self.state.operations.lock().map_err(|_| invalid())?;
        if operations.accumulation.as_ref().is_some_and(|run| {
            run.phase != Phase::Complete || run.completed != run.updates || run.staged.is_some()
        }) {
            return Err(invalid());
        }
        Ok(())
    }
}
