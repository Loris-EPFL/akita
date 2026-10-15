//! Explicit NR5 E2 configuration for the reviewed compact inner accumulation row.
//!
//! This is not a legacy subring/tensor preset. Its frozen geometry and table
//! admission do not certify the D1 knowledge or Fiat–Shamir assumptions.

use crate::{policy_of, CommitmentConfig, PlannerOpeningFamily, RingDimensionScheduleMode};
use akita_challenges::SparseChallengeConfig;
use akita_error::AkitaError;
use akita_params::sis::{CommittedSourceClass, CommittedSourceContract};
use akita_params::{
    BlockGeometry, CommitmentSliceCount, DecompositionParams, GadgetDigits, GroupCommitPhaseParams,
    InnerCommitMatrixParams, InnerRoleParams, OuterCommitMatrixParams, OuterRoleParams,
    PolynomialGroupLayout, ScheduleLookupKey, SisModulusProfileId,
};
use jolt_field::{Prime128OffsetA7F7, Prime128OffsetA7F7Ext2};

/// Boolean Main with signed receiver and original 26-plane RAM precommits.
#[derive(Clone)]
pub struct CompactInner;

impl CommitmentConfig for CompactInner {
    type Field = Prime128OffsetA7F7;
    type ExtField = Prime128OffsetA7F7Ext2;

    const RING_DIMENSION_SCHEDULE_MODE: RingDimensionScheduleMode =
        RingDimensionScheduleMode::UniformDimension {
            ring_dimension: 128,
        };

    fn opening_family() -> PlannerOpeningFamily {
        PlannerOpeningFamily::FullRingE2InnerRecursive
    }

    fn decomposition() -> DecompositionParams {
        DecompositionParams {
            log_basis: 5,
            log_commit_bound: 1,
            log_open_bound: Some(128),
        }
    }

    fn ring_challenge_config(d: usize) -> Result<SparseChallengeConfig, AkitaError> {
        if d != 128 {
            return Err(AkitaError::InvalidSetup(
                "compact inner challenges require degree128".into(),
            ));
        }
        Ok(akita_params::FULL_RING_FOLD_CHALLENGE)
    }

    fn sis_modulus_profile() -> SisModulusProfileId {
        SisModulusProfileId::Q128OffsetA7F7
    }

    fn opening_basis_range() -> (u32, u32) {
        (5, 5)
    }

    fn committed_source_class() -> CommittedSourceClass {
        CommittedSourceClass::UnitOneHot {
            source_chunk_size: 1,
        }
    }

    fn schedule_family_name() -> &'static str {
        "akita-full-ring-e2-compact-inner-v1"
    }
}

impl CompactInner {
    /// Exact public root key and producer contracts in receiver/RAM-minus/RAM-plus order.
    ///
    /// Main's producer is [`Self::committed_source_contract`]. These are inputs
    /// to the existing planner, not an installed catalog or provisioned matrices.
    ///
    /// # Errors
    ///
    /// Returns an error if a frozen source contract or SIS matrix profile is invalid.
    pub fn root_inputs() -> Result<(ScheduleLookupKey, [CommittedSourceContract; 3]), AkitaError> {
        let receiver = Self::precommit(22, 32, 1024, 1, 4, 260_046_724)?;
        let ram = Self::precommit(16, 16, 32, 26, 3, 8_126_340)?;
        let receiver_contract = CommittedSourceContract::try_new(
            CommittedSourceClass::BalancedSignedDigit,
            DecompositionParams {
                log_commit_bound: 5,
                ..Self::decomposition()
            },
        )?;
        let ram_contract = CommittedSourceContract::try_new(
            CommittedSourceClass::OriginalBalancedDigitPlanes,
            DecompositionParams {
                log_commit_bound: 128,
                ..Self::decomposition()
            },
        )?;
        Ok((
            ScheduleLookupKey::decoded(
                PolynomialGroupLayout::new(16, 56),
                vec![receiver, ram, ram],
            ),
            [receiver_contract, ram_contract, ram_contract],
        ))
    }

    fn precommit(
        vars: usize,
        blocks: usize,
        positions: usize,
        digits: usize,
        rank: usize,
        radius: u128,
    ) -> Result<GroupCommitPhaseParams, AkitaError> {
        let policy = policy_of::<Self>();
        Ok(GroupCommitPhaseParams {
            version: GroupCommitPhaseParams::VERSION,
            group: PolynomialGroupLayout::singleton(vars),
            blocks: BlockGeometry::new(blocks * positions, positions, blocks),
            outer_slice_count: CommitmentSliceCount::ONE,
            inner: InnerRoleParams::new(
                GadgetDigits::new(5, digits),
                InnerCommitMatrixParams::try_new(
                    policy.sis_security_policy,
                    policy.sis_table_digest,
                    policy.sis_modulus_profile,
                    rank,
                    positions * digits,
                    radius,
                    128,
                )?,
            ),
            outer: OuterRoleParams::new(
                GadgetDigits::new(5, 26),
                OuterCommitMatrixParams::try_new(
                    policy.sis_security_policy,
                    policy.sis_table_digest,
                    policy.sis_modulus_profile,
                    1,
                    blocks * rank * 26,
                    31,
                    128,
                )?,
            ),
        })
    }
}

/// Ordinary K/K consumer of the frozen compact Main and original RAM planes.
///
/// The planner must preserve the producer's exact profiles and contracts.
/// This configuration does not authorize retagging raw full-ring openings.
#[derive(Clone)]
pub struct OrdinaryControl;

impl CommitmentConfig for OrdinaryControl {
    type Field = Prime128OffsetA7F7;
    type ExtField = Prime128OffsetA7F7;

    const RING_DIMENSION_SCHEDULE_MODE: RingDimensionScheduleMode =
        RingDimensionScheduleMode::UniformDimension {
            ring_dimension: 128,
        };

    fn decomposition() -> DecompositionParams {
        CompactInner::decomposition()
    }

    fn ring_challenge_config(d: usize) -> Result<SparseChallengeConfig, AkitaError> {
        SparseChallengeConfig::production_for_ring_dim(d)
            .ok_or_else(|| AkitaError::InvalidSetup("no production challenge shell".into()))
    }

    fn sis_modulus_profile() -> SisModulusProfileId {
        CompactInner::sis_modulus_profile()
    }

    fn opening_basis_range() -> (u32, u32) {
        (5, 5)
    }

    fn inner_basis_range() -> (u32, u32) {
        (4, 5)
    }

    fn committed_source_class() -> CommittedSourceClass {
        CompactInner::committed_source_class()
    }

    fn schedule_family_name() -> &'static str {
        "akita-original-plane-ordinary-control-v1"
    }
}
