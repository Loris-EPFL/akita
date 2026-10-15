//! Commitment execution and persistence admission at the owning backend boundary.
use super::owned::{CommitOutput, CommitmentHandle, CommittedSource, SourceHandle};
use crate::commitment::{CommitmentExecutor, CommitmentSource, GroupContext, PortableStatePolicy};
use crate::opaque::CpuBackend;
use akita_config::{CommitmentConfig, TrustedScheduleCatalog};
use akita_error::AkitaError;
use akita_params::{sis::CommittedSourceContract, GroupCommitPhaseParams};
use akita_serialization::AkitaSerialize;
use jolt_field::{CanonicalEncoding, Field, Unreduced, WithCommitAccumulator};
use std::sync::Arc;

impl<F, E> CpuBackend<F, E>
where
    F: Field + CanonicalEncoding + AkitaSerialize + Unreduced + WithCommitAccumulator + 'static,
    F::Wide: From<F>,
    E: jolt_field::ExtField<F> + 'static,
{
    /// Import a retained commitment from another backend after validating its
    /// source and complete commitment material against this backend's setup.
    pub fn import_commitment(
        &self,
        foreign: &CommitmentHandle<F, E>,
    ) -> Result<CommitmentHandle<F, E>, AkitaError> {
        let committed = &foreign.committed;
        // Keep the same immutable K storage and its capability. Recompute using
        // the receiving setup; never decode/redecompose a transferred source.
        let sources = committed.source.commitment_sources();
        let layout = if committed.source.full_ring().is_some() {
            crate::commitment::resolve_source_layout(&sources)?
        } else {
            crate::commitment::resolve_polynomial_group_layout(
                &sources,
                self.prepared()?.expanded()?,
            )?
        };
        if layout != committed.parameters.group {
            return Err(AkitaError::InvalidInput(
                "transferred source differs from commitment parameters".into(),
            ));
        }
        let (recomputed, retained) = if let Some(full_ring) = committed.source.full_ring() {
            if full_ring.raw_view()
                != &self
                    .full_ring_setup()?
                    .descriptor()
                    .source(full_ring.source_id())?
                    .raw_view
            {
                return Err(AkitaError::InvalidInput(
                    "transferred source view differs from installed source identity".into(),
                ));
            }
            let executor = CommitmentExecutor::cpu_full_ring(
                self,
                Some(full_ring.source_id()),
                Vec::new(),
                PortableStatePolicy,
            )?;
            let plan = crate::commitment::CommitmentExecutionPlan::for_full_ring_root(
                &committed.parameters,
            )?;
            let (payload, retained) = executor.execute_full(&plan, &sources)?.into_parts();
            (
                akita_types::CommittedGroup::new(
                    committed.parameters,
                    akita_types::Commitment::new(payload),
                ),
                retained,
            )
        } else {
            let prepared = self.prepared()?;
            let executor = CommitmentExecutor::cpu(
                self,
                prepared,
                prepared.expanded()?,
                Vec::new(),
                PortableStatePolicy,
            )?;
            executor.execute_full_via_outer_image(self, committed.parameters, &sources)?
        };
        if recomputed.commitment != committed.public || retained != committed.retained {
            return Err(AkitaError::InvalidInput(
                "transferred commitment does not match backend setup".into(),
            ));
        }
        Ok(CommitmentHandle {
            owner: self.owner_id(),
            committed: Arc::new(CommittedSource {
                commitment_id: self.owner().next_operation_id()?,
                source: committed.source.clone(),
                metadata: committed.metadata,
                parameters: committed.parameters,
                // Recomputation above proves the same source and commitment,
                // so the admitting contract carries over unchanged.
                producer_contract: committed.producer_contract,
                public: committed.public.clone(),
                retained,
            }),
        })
    }
}

impl<F, E> CpuBackend<F, E>
where
    F: Field + CanonicalEncoding + AkitaSerialize + Unreduced + WithCommitAccumulator + 'static,
    F::Wide: From<F>,
    E: jolt_field::ExtField<F> + 'static,
{
    /// Commit an imported immutable source under the producer schedule family
    /// `family`, retaining its exact source and parameters.
    ///
    /// `family` supplies the catalog row for scheduler contexts and the
    /// declared committed-source contract that admits the source. The handle
    /// records that contract. One backend can commit groups from different
    /// families that share `F` and `E`.
    pub fn commit<Cfg>(
        &self,
        family: &TrustedScheduleCatalog<Cfg>,
        source: &SourceHandle<F, E>,
        context: GroupContext<'_>,
    ) -> Result<CommitOutput<F, E>, AkitaError>
    where
        Cfg: CommitmentConfig<Field = F, ExtField = E>,
    {
        if source.owner != self.owner_id() {
            return Err(AkitaError::InvalidInput(
                "source belongs to another backend".into(),
            ));
        }
        let sources = source.storage.commitment_sources();
        let (profile, producer_contract) = if let Some(full_ring) = source.storage.full_ring() {
            crate::commitment::resolve_full_ring_commit_params::<Cfg, _>(
                &sources,
                self.full_ring_setup()?.descriptor(),
                full_ring.source_id(),
                family,
                context,
            )?
        } else {
            crate::commitment::resolve_commit_params::<Cfg, _>(
                &sources,
                self.prepared()?.expanded()?,
                family,
                context,
            )?
        };
        self.commit_admitted_source(source, &sources, profile, producer_contract)
    }

    /// Commit an original full-ring source under its frozen producer contract.
    ///
    /// The compact opening family's own source is Boolean Main; it is not the
    /// producer of its signed receiver or 26-plane RAM inputs. This boundary
    /// admits those producers without constructing an unused opening catalog.
    /// It uses the same executor and retained-source handle as [`Self::commit`].
    ///
    /// The caller supplies the installed profile and source contract. Admission
    /// checks their exact source representation and literal matrix geometry;
    /// the consuming opening still validates its complete schedule and domains.
    ///
    /// # Errors
    ///
    /// Rejects foreign or non-native sources, a different K/E tower, mismatched
    /// installed geometry, an invalid producer envelope, or commitment failure.
    pub fn commit_full_ring(
        &self,
        source: &SourceHandle<F, E>,
        profile: GroupCommitPhaseParams,
        producer_contract: CommittedSourceContract,
    ) -> Result<CommitOutput<F, E>, AkitaError> {
        if source.owner != self.owner_id() {
            return Err(AkitaError::InvalidInput(
                "source belongs to another backend".into(),
            ));
        }
        akita_types::full_ring::validate_full_ring_fields::<F, E>()?;
        let native = source.storage.full_ring().ok_or_else(|| {
            AkitaError::InvalidInput("source has no original full-ring capability".into())
        })?;
        let sources = source.storage.commitment_sources();
        let layout = crate::commitment::resolve_source_layout::<F, _>(&sources)?;
        crate::commitment::validate_full_ring_commit_source::<F, _>(
            &sources,
            self.full_ring_setup()?.descriptor(),
            native.source_id(),
            profile,
            producer_contract,
            layout,
            true,
        )?;
        self.commit_admitted_source(source, &sources, profile, producer_contract)
    }

    fn commit_admitted_source(
        &self,
        source: &SourceHandle<F, E>,
        sources: &[&dyn CommitmentSource<F>],
        profile: GroupCommitPhaseParams,
        producer_contract: CommittedSourceContract,
    ) -> Result<CommitOutput<F, E>, AkitaError> {
        let (committed_group, prover_state) = if let Some(full_ring) = source.storage.full_ring() {
            let executor = CommitmentExecutor::cpu_full_ring(
                self,
                Some(full_ring.source_id()),
                Vec::new(),
                PortableStatePolicy,
            )?;
            let plan = crate::commitment::CommitmentExecutionPlan::for_full_ring_root(&profile)?;
            let (payload, retained) = executor.execute_full(&plan, sources)?.into_parts();
            (
                akita_types::CommittedGroup::new(profile, akita_types::Commitment::new(payload)),
                retained,
            )
        } else {
            let prepared = self.prepared()?;
            let executor = CommitmentExecutor::cpu(
                self,
                prepared,
                prepared.expanded()?,
                Vec::new(),
                PortableStatePolicy,
            )?;
            executor.execute_full_via_outer_image(self, profile, sources)?
        };
        let private_handle = CommitmentHandle {
            owner: source.owner,
            committed: Arc::new(CommittedSource {
                commitment_id: self.owner().next_operation_id()?,
                source: source.storage.clone(),
                metadata: source.metadata,
                parameters: *committed_group.profile(),
                producer_contract,
                public: committed_group.commitment().clone(),
                retained: prover_state,
            }),
        };
        Ok(CommitOutput {
            committed_group,
            private_handle,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AkitaProverSetup, DensePoly};
    use akita_config::proof_optimized::fp64;
    use akita_params::{OpeningClaimsLayout, ScheduleLookupKey};
    use akita_prover::CommitmentHandleMetadata;
    use akita_types::full_ring::FullRingCoefficientSource;
    use jolt_field::Ring;

    type Cfg = fp64::Dense;
    type F = <Cfg as CommitmentConfig>::Field;
    type E = <Cfg as CommitmentConfig>::ExtField;

    #[test]
    fn cpu_commit_matches_full_executor_state_after_outer_image_completion() {
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(|| {
                const NUM_VARS: usize = 14;
                let schedules =
                    akita_config::test_support::workspace_schedule_catalog::<Cfg>().unwrap();
                let layout = OpeningClaimsLayout::new(NUM_VARS, 1).unwrap();
                let key = ScheduleLookupKey::single(layout.root_final_group_layout().unwrap());
                let profile = schedules.resolve_key(&key).unwrap().profiles().final_group;
                let capacity =
                    akita_config::SetupRequirements::from_catalog::<Cfg>(&schedules, NUM_VARS, 1)
                        .unwrap()
                        .matrix_capacity();
                let setup =
                    AkitaProverSetup::<F>::generate_with_capacity(NUM_VARS, 1, capacity).unwrap();
                let backend = CpuBackend::<F, E>::new(setup.expanded.clone()).unwrap();
                let prepared = backend.prepared().unwrap();
                let executor = CommitmentExecutor::cpu(
                    &backend,
                    prepared,
                    setup.expanded.as_ref(),
                    Vec::new(),
                    PortableStatePolicy,
                )
                .unwrap();
                let evals = (0..1usize << NUM_VARS)
                    .map(|index| F::from_u64(index as u64 + 1))
                    .collect::<Vec<_>>();
                let poly = DensePoly::<F>::from_field_evals(NUM_VARS, &evals).unwrap();
                let expected = crate::commitment::commit::<Cfg, _, _>(
                    std::slice::from_ref(&poly),
                    setup.expanded.as_ref(),
                    &schedules,
                    &executor,
                    GroupContext::explicit(&profile),
                )
                .unwrap();
                let source = backend.import_source(vec![poly]).unwrap();
                let actual = backend
                    .commit(&schedules, &source, GroupContext::explicit(&profile))
                    .unwrap();
                let reader = backend.decoded_dense_component_reader(&actual.private_handle, 0).unwrap();
                let extent = akita_error::checked::product([
                    profile.blocks.live_blocks,
                    profile.blocks.positions_per_block,
                    profile.inner.matrix.ring_dimension(),
                ]).unwrap();
                assert_eq!(reader.coefficient(0).unwrap(), F::from_u64(1));
                assert_eq!(
                    reader.coefficient(extent - 1).unwrap(),
                    evals.get(extent - 1).copied().unwrap_or_else(|| F::from_u64(0)),
                );
                assert!(reader.coefficient(extent).is_err());
                assert!(reader.coefficient(usize::MAX).is_err());
                assert!(backend.decoded_dense_component_reader(&actual.private_handle, 1).is_err());

                assert_eq!(actual.committed_group, expected.committed_group);
                assert_eq!(
                    actual.private_handle.committed.retained,
                    expected.prover_state
                );

                let receiving_backend = CpuBackend::<F, E>::new(setup.expanded.clone()).unwrap();
                assert!(receiving_backend.decoded_dense_component_reader(&actual.private_handle, 0).is_err());
                let transferred = receiving_backend
                    .import_commitment(&actual.private_handle)
                    .unwrap();
                let transferred_reader = receiving_backend.decoded_dense_component_reader(&transferred, 0).unwrap();
                assert_eq!(transferred_reader.coefficient(0).unwrap(), F::from_u64(1));
                assert!(backend.decoded_dense_component_reader(&transferred, 0).is_err());
                assert_eq!(transferred.committed.retained, expected.prover_state);
                assert_eq!(
                    actual.private_handle.producer_contract(),
                    Cfg::committed_source_contract().unwrap()
                );
                assert_eq!(
                    transferred.producer_contract(),
                    actual.private_handle.producer_contract()
                );
            })
            .unwrap()
            .join()
            .unwrap();
    }
}
