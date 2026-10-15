//! Canonical method-aware fold-challenge dispatch shared by prover and verifier.

use akita_challenges::{CanonicalShellGroup, CanonicalShellLevel, Challenges, FoldChallengeDrawDomain, ShellGroupDescriptor};
use akita_error::AkitaError;
use jolt_field::{ExtField, Field};

use crate::CoefficientPackingChallenges;
use akita_params::{InnerCommitSecurityRoute, OpeningMethod, SubringCoefficientPackingGeometry};

/// Runtime value carried by one of Akita's typed opening methods.
///
/// The schedule chooses an [`OpeningMethod`]; this family preserves that same
/// method distinction while each protocol stage supplies its own payloads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpeningFamily<Trace, Packing, FullRing = std::convert::Infallible> {
    /// Value belonging to the full-A evaluation-trace method.
    EvaluationTrace(Trace),
    /// Value belonging to subring coefficient packing.
    SubringCoefficientPacking(Packing),
    /// Full ambient-ring action with ordinary extension coordinates, not a subfield map.
    FullRingScalarExtension(FullRing),
}

/// One sampled fold challenge with its method-specific algebraic views.
pub type GroupFoldChallenges = OpeningFamily<Challenges, CoefficientPackingChallenges, Challenges>;

impl OpeningFamily<Challenges, CoefficientPackingChallenges, Challenges> {
    /// Challenges acting in the committed group's ambient A ring.
    #[must_use]
    pub const fn ambient_a(&self) -> &Challenges {
        match self {
            Self::EvaluationTrace(challenges) => challenges,
            Self::SubringCoefficientPacking(challenges) => challenges.ambient_a(),
            Self::FullRingScalarExtension(challenges) => challenges,
        }
    }
}

/// Immutable, method-aware plan frozen before one whole-level indexed draw.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupFoldChallengePlan {
    descriptor: ShellGroupDescriptor,
    packing_geometry: Option<SubringCoefficientPackingGeometry>,
}

impl GroupFoldChallengePlan {
    pub fn descriptor(&self) -> &ShellGroupDescriptor {
        &self.descriptor
    }
}

/// Plan one group's canonical shell shape without touching a transcript.
pub fn plan_group_fold_challenges<F, E>(
    params: &akita_params::GroupOpenPhaseParams,
    group_index: usize,
    num_claims: usize,
) -> Result<GroupFoldChallengePlan, AkitaError>
where
    F: Field,
    E: ExtField<F>,
{
    params.opening.validate_source_view(&params.profile, E::DEGREE)?;
    if params.source_view() == akita_params::SourceOpeningView::OriginalRamPlanesV1
        && (num_claims != 1 || params.setup_natural_len.is_some())
    {
        return Err(AkitaError::InvalidSetup("raw RAM shell plans require one original root scalar".into()));
    }
    let d_a = params.inner_commit_matrix_params().ring_dimension();
    let config = params.fold_challenge_config();
    let (domain, carrier, rejection, packing_geometry) = match params.opening_method() {
        OpeningMethod::EvaluationTrace => {
            let rejection = matches!(
                params.inner_commit_matrix_params().security_route(),
                InnerCommitSecurityRoute::L2 { .. }
            )
            .then(|| akita_challenges::selective_l2_operator_norm_rejection(d_a, &config))
            .flatten();
            (FoldChallengeDrawDomain::EvaluationTrace, d_a, rejection, None)
        }
        OpeningMethod::SubringCoefficientPacking { challenge_subring_dimension } => {
            if !matches!(
                params.inner_commit_matrix_params().security_route(),
                InnerCommitSecurityRoute::Linf(_)
            ) {
                return Err(AkitaError::InvalidSetup(
                    "coefficient packing requires an L-infinity A security route".into(),
                ));
            }
            let geometry = SubringCoefficientPackingGeometry::try_new(
                E::DEGREE, d_a, challenge_subring_dimension,
            )?;
            if config != geometry.fold_challenge_config() {
                return Err(AkitaError::InvalidSetup(
                    "coefficient-packing challenge config is not the audited production family".into(),
                ));
            }
            (
                FoldChallengeDrawDomain::SubringCoefficientPacking { challenge_subring_dimension },
                challenge_subring_dimension,
                None,
                Some(geometry),
            )
        }
        OpeningMethod::FullRingScalarExtension { .. } => {
            return Err(AkitaError::InvalidSetup(
                "full-ring challenges require the selected D1 inner or outer transcript compiler".into(),
            ));
        }
    };
    let descriptor = ShellGroupDescriptor::new(
        domain, carrier, group_index, params.num_live_blocks(), num_claims, config, rejection,
    )?;
    let descriptor = match packing_geometry {
        Some(geometry) => descriptor.with_native_embedding(d_a, geometry.subring_embedding_stride())?,
        None => descriptor,
    };
    Ok(GroupFoldChallengePlan { descriptor, packing_geometry })
}

/// Assemble only from the completed whole-level result and exact frozen plan.
/// The challenge clone shares its immutable allocation; packing derives the
/// ambient-A embedding once, without another oracle query.
pub fn group_fold_challenges_from_level(
    plan: &GroupFoldChallengePlan,
    level: &CanonicalShellLevel,
) -> Result<GroupFoldChallenges, AkitaError> {
    let group = level.group(plan.descriptor.group_index())?;
    if group.descriptor() != &plan.descriptor {
        return Err(AkitaError::InvalidSetup("canonical shell group does not match frozen plan".into()));
    }
    let challenges = group.challenges().clone();
    match plan.packing_geometry {
        Some(geometry) => Ok(OpeningFamily::SubringCoefficientPacking(
            CoefficientPackingChallenges::new(geometry, challenges)?,
        )),
        None => Ok(OpeningFamily::EvaluationTrace(challenges)),
    }
}

/// Consume a completed canonical group without copying its coordinate payload.
pub fn group_fold_challenges_from_group(
    plan: &GroupFoldChallengePlan,
    group: CanonicalShellGroup,
) -> Result<GroupFoldChallenges, AkitaError> {
    if group.descriptor() != &plan.descriptor {
        return Err(AkitaError::InvalidSetup("canonical shell group does not match frozen plan".into()));
    }
    let challenges = group.into_challenges();
    match plan.packing_geometry {
        Some(geometry) => Ok(OpeningFamily::SubringCoefficientPacking(
            CoefficientPackingChallenges::new(geometry, challenges)?,
        )),
        None => Ok(OpeningFamily::EvaluationTrace(challenges)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use akita_challenges::{decode_shell_level_from_bytes, ShellLevelDescriptor, SparseChallengeConfig};
    use akita_params::{GroupOpenPhaseParams, GroupOpeningPlan, SourceOpeningView};
    use jolt_field::Prime128OffsetA7F7 as F;

    fn raw_group() -> GroupOpenPhaseParams {
        GroupOpenPhaseParams {
            profile: akita_params::test_fixtures::original_ram_profile(),
            setup_natural_len: None,
            opening: GroupOpeningPlan {
                source_view: SourceOpeningView::OriginalRamPlanesV1,
                opening_method: OpeningMethod::SubringCoefficientPacking { challenge_subring_dimension: 128 },
                fold_challenge_config: SparseChallengeConfig::production_for_ring_dim(128).unwrap(),
                log_basis_open: 5,
                num_digits_open: 26,
                num_digits_fold: 3,
            },
        }
    }

    #[test]
    fn pure_plan_assembles_only_the_completed_exact_shell_group() {
        let params = raw_group();
        params.validate().unwrap();
        let plan = plan_group_fold_challenges::<F, F>(&params, 0, 1).unwrap();
        let descriptor = ShellLevelDescriptor::new(vec![plan.descriptor().clone()]).unwrap();
        let level = decode_shell_level_from_bytes(&descriptor, &vec![0; descriptor.raw_len() as usize]).unwrap();
        let packed = group_fold_challenges_from_level(&plan, &level).unwrap();
        assert_eq!(packed.ambient_a().num_claims(), 1);
        assert_eq!(packed.ambient_a().num_live_blocks_per_claim(), 16);
        let owned = group_fold_challenges_from_group(&plan, level.clone().into_groups().pop().unwrap()).unwrap();
        assert_eq!(owned, packed);
        let mut trace = params;
        trace.opening.source_view = SourceOpeningView::Decoded;
        trace.opening.opening_method = OpeningMethod::EvaluationTrace;
        let trace_plan = plan_group_fold_challenges::<F, F>(&trace, 0, 1).unwrap();
        assert!(group_fold_challenges_from_level(&trace_plan, &level).is_err());
    }

    #[test]
    fn pure_plan_rejects_wrong_method_and_unreviewed_shell_config() {
        let mut params = raw_group();
        params.opening.opening_method = OpeningMethod::EvaluationTrace;
        assert!(plan_group_fold_challenges::<F, F>(&params, 0, 1).is_err());
        let mut params = raw_group();
        params.opening.fold_challenge_config = SparseChallengeConfig::pm1_only(1);
        assert!(plan_group_fold_challenges::<F, F>(&params, 0, 1).is_err());
        let mut params = raw_group();
        params.opening.source_view = SourceOpeningView::Decoded;
        params.opening.opening_method = OpeningMethod::FULL_RING_E2;
        assert!(plan_group_fold_challenges::<F, F>(&params, 0, 1).is_err());
    }
}
