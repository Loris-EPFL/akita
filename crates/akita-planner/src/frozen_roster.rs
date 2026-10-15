//! Structural frozen-roster planning and exact selected-row accounting.
//!
//! Source identities, boundary aliases, prequery ownership and extraction work
//! belong to the caller's authenticated protocol ledger. These outputs are
//! inputs to that ledger, not an advantage estimator or a setup-model proof.

use akita_error::{checked, AkitaError};
use akita_params::sis::CommittedSourceContract;
use akita_params::{
    CommittedGroupBatchProfile, FoldScheduleEstimate, GroupCommitPhaseParams,
    SourceOpeningView, ScheduleSisOccurrence,
};
use akita_schedules::ResolvedScheduleRow;

use crate::{planner, PlannerPolicy};

/// Exact structural request in native transcript order: prefix, then final.
/// VM callers designate the last Main; Dense callers designate the last source.
pub struct FrozenRosterRequest<'a> {
    pub profiles: &'a CommittedGroupBatchProfile,
    pub final_source_contract: CommittedSourceContract,
    pub precommitted_source_contracts: &'a [CommittedSourceContract],
    pub source_views: &'a [SourceOpeningView],
    /// Actual sealed query-equation count, separate from endpoint/shell counts.
    /// The frontend authenticates this count; the PCS planner cannot infer it.
    pub query_equation_count: usize,
    pub policy: &'a PlannerPolicy,
}

/// Honest joint response completeness under the approved ideal indexed law.
/// This is not a statement about an adversarial extracted source or winner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrozenJointResponseCertificate {
    pub all_valid_groups: usize,
    pub dense_groups: usize,
    /// Strict joint nonce-zero rejection bound numerator / 2^denominator_log2.
    /// Zero numerator means every registered native128 response is valid.
    pub reject_numerator: u128,
    pub reject_denominator_log2: u32,
    pub dense_variance_bound: u128,
    pub dense_first_rejected_positive: u128,
    pub nonce_attempt_limit: usize,
}

#[derive(Clone, Debug)]
pub struct FrozenGroupLedger {
    pub profile: GroupCommitPhaseParams,
    pub source_contract: CommittedSourceContract,
    pub source_view: SourceOpeningView,
    pub source_num_vars: usize,
    pub point_num_vars: usize,
    pub logical_coefficients: usize,
    pub scalar_endpoints: usize,
    pub shell_coordinates: usize,
    pub carrier_dimension: usize,
    pub packing_h: usize,
    pub source_linf: u128,
    pub source_ring_l1: u128,
    pub source_energy: u128,
    pub deterministic_response_cap: u128,
    pub response_depth: usize,
    pub raw_response_coefficients: usize,
    pub response_digit_coefficients: usize,
    pub response_negative_reach: u128,
    pub response_positive_reach: u128,
    pub response_difference_diameter: u128,
    pub weak_multiplier_l1: u128,
    pub diagonal_a_radius: u128,
    pub d_offset: usize,
    pub d_width: usize,
    /// Complete sliced image, never just one B slice.
    pub original_f_source_coefficients: usize,
}

#[derive(Clone, Debug)]
pub struct FrozenRosterLedger {
    pub groups: Vec<FrozenGroupLedger>,
    pub source_groups: usize,
    pub scalar_endpoints: usize,
    pub shell_coordinates: usize,
    pub query_equation_count: usize,
    pub max_source_num_vars: usize,
    pub max_point_num_vars: usize,
    /// Physical root processing order (final first), distinct from transcript order.
    pub root_group_order: Vec<usize>,
    pub shared_d_width: usize,
    pub shared_h_source_coefficients: usize,
    pub setup_field_elements: usize,
    pub verifier_setup_field_elements: usize,
    pub encoded_row_bytes: usize,
    /// All actual A/B/D/F/H occurrences at every level plus the actual A-only
    /// terminal. Radii/table admission are not numerical runtime advantages.
    pub sis_occurrences: Vec<ScheduleSisOccurrence>,
    /// Actual local losses, sampler sites, finite query count and nonce inputs.
    /// The caller still supplies the source/range/query and extraction-tree cuts.
    pub grinding_plan: akita_params::GrindingPlan,
    pub setup_prefix_slots: Vec<akita_params::SetupPrefixSlotId>,
    pub joint_response: FrozenJointResponseCertificate,
}

/// Complete independently admitted row, not an installed catalog trust claim.
/// The row exposes all selected root/suffix/terminal geometry and its digest.
#[derive(Clone, Debug)]
pub struct FrozenRosterPlan {
    pub row: ResolvedScheduleRow,
    pub estimate: FoldScheduleEstimate,
    pub ledger: FrozenRosterLedger,
}

fn dense_joint_gate(depth: usize, raw_coefficients: usize, variance: u128, first_rejected: u128) -> bool {
    depth == 7 && raw_coefficients == 20_480 && variance <= 9_529_458_688
        && first_rejected >= 898_780
        && first_rejected.checked_mul(first_rejected).and_then(|n| n.checked_mul(20))
            .zip(variance.checked_mul(2 * 847)).is_some_and(|(left, right)| left > right)
}

fn overflow(label: &str) -> AkitaError {
    AkitaError::InvalidSetup(format!("frozen roster {label} overflow"))
}

/// Search the finite policy domain from the real root output through every
/// shrinking successor and the actual terminal. There is no scalar suffix,
/// profile rewrite, operator-filter workaround or decoded retry.
pub fn plan_frozen_source_roster(
    request: FrozenRosterRequest<'_>,
    ring_challenge_config: impl Fn(usize) -> Result<akita_challenges::SparseChallengeConfig, AkitaError>,
) -> Result<FrozenRosterPlan, AkitaError> {
    if request.query_equation_count == 0 {
        return Err(AkitaError::InvalidInput("sealed query roster must be nonempty".into()));
    }
    let planned = planner::find_schedule_for_frozen_profiles(
        request.profiles, request.final_source_contract, request.precommitted_source_contracts,
        request.source_views, request.policy, ring_challenge_config,
    )?;
    let row = ResolvedScheduleRow::try_new(
        request.profiles.clone(), planned.schedule, request.policy,
    )?;
    let layout = row.opening_layout()?;
    let root = &row.schedule().root.params;
    let mut groups = Vec::with_capacity(root.groups().len());
    let mut scalar_endpoints = 0usize;
    let mut shell_coordinates = 0usize;
    let mut d_offset = 0usize;
    let mut all_valid_groups = 0usize;
    let mut dense_groups = 0usize;
    for (index, (group, contract)) in root.groups().iter().zip(
        request.precommitted_source_contracts.iter().copied()
            .chain(std::iter::once(request.final_source_contract)),
    ).enumerate() {
        let profile = &group.profile;
        let p = profile.group.num_polynomials();
        let shell_count = p.checked_mul(profile.blocks.live_blocks).ok_or_else(|| overflow("shell count"))?;
        let logical_coefficients = checked::pow2(profile.group.num_vars())
            .and_then(|n| n.checked_mul(p)).ok_or_else(|| overflow("source length"))?;
        let norms = contract.source_norms(
            profile.inner.digits.log_basis, profile.inner.digits.num_digits,
            profile.inner.matrix.ring_dimension(), logical_coefficients,
        )?;
        let carrier = planner::frozen_root_carrier(profile, contract)?;
        let config = group.fold_challenge_config();
        let omega = config.l1_norm() as u128;
        let response_depth = group.num_digits_fold();
        let (negative, positive) = akita_params::sis::checked_balanced_digit_representable_bounds(
            group.opening.log_basis_open, response_depth,
        );
        let negative = negative.ok_or_else(|| overflow("negative response reach"))?;
        let positive = positive.ok_or_else(|| overflow("positive response reach"))?;
        let diameter = negative.checked_add(positive).ok_or_else(|| overflow("response diameter"))?;
        let diagonal_a_radius = omega.checked_mul(4).and_then(|n| n.checked_mul(diameter))
            .ok_or_else(|| overflow("original A radius"))?;
        let raw_coefficients = profile.inner.matrix.input_width()
            .checked_mul(profile.inner.matrix.ring_dimension()).ok_or_else(|| overflow("response union"))?;
        let deterministic_cap = planner::frozen_deterministic_cap(profile, contract, &config)?;
        if carrier == 128 {
            if response_depth != 3 || deterministic_cap > positive {
                return Err(AkitaError::InvalidSetup("selected native128 root is not all-valid depth3".into()));
            }
            all_valid_groups += 1;
        } else {
            // Source and capability registration fix N=20480, eight blocks,
            // digits<=4096 and unfiltered production64 (31,10). Prove the
            // rational Chernoff-to-dyadic inequality using the selected gates.
            let variance = (profile.blocks.live_blocks as u128)
                .checked_mul(config.challenge_l2_sq_max())
                .and_then(|n| n.checked_mul(norms.fold_witness.infinity_norm()))
                .and_then(|n| n.checked_mul(norms.fold_witness.infinity_norm()))
                .ok_or_else(|| overflow("Dense variance"))?;
            let first_rejected = positive.checked_add(1).ok_or_else(|| overflow("Dense gate"))?;
            if !dense_joint_gate(response_depth, raw_coefficients, variance, first_rejected) {
                return Err(AkitaError::UnsupportedSchedule("selected Dense joint Rademacher certificate gate".into()));
            }
            dense_groups += 1;
        }
        let d_width = group.d_segment_width(request.policy.claim_ext_degree, root.open_matrix.ring_dimension())?;
        groups.push(FrozenGroupLedger {
            profile: *profile, source_contract: contract, source_view: group.opening.source_view(),
            source_num_vars: profile.group.num_vars(), point_num_vars: layout.group_point_num_vars(index)?,
            logical_coefficients, scalar_endpoints: p, shell_coordinates: shell_count,
            carrier_dimension: carrier, packing_h: profile.inner.matrix.ring_dimension() / carrier,
            source_linf: norms.fold_witness.infinity_norm(), source_ring_l1: norms.fold_witness.l1_norm(),
            source_energy: norms.l2_sq, deterministic_response_cap: deterministic_cap, response_depth,
            raw_response_coefficients: raw_coefficients,
            response_digit_coefficients: raw_coefficients.checked_mul(response_depth).ok_or_else(|| overflow("response payload"))?,
            response_negative_reach: negative, response_positive_reach: positive,
            response_difference_diameter: diameter, weak_multiplier_l1: 2 * omega, diagonal_a_radius,
            d_offset, d_width,
            original_f_source_coefficients: profile.outer_slice_count.complete_source_coefficients(
                profile.outer.matrix.output_rank(), profile.outer.matrix.ring_dimension(),
            )?,
        });
        d_offset = d_offset.checked_add(d_width).ok_or_else(|| overflow("D width"))?;
        scalar_endpoints = scalar_endpoints.checked_add(p).ok_or_else(|| overflow("endpoint count"))?;
        shell_coordinates = shell_coordinates.checked_add(shell_count).ok_or_else(|| overflow("shell count"))?;
    }
    if d_offset != root.open_matrix.input_width() {
        return Err(AkitaError::InvalidSetup("selected shared D differs from complete original contributions".into()));
    }
    let root_group_order = layout.root_group_order()?;
    let mut physical_offset = 0usize;
    for &index in &root_group_order {
        let group = groups.get_mut(index).ok_or_else(|| {
            AkitaError::InvalidSetup("root physical group order is out of range".into())
        })?;
        group.d_offset = physical_offset;
        physical_offset = physical_offset.checked_add(group.d_width).ok_or_else(|| overflow("D offset"))?;
    }
    let ledger = FrozenRosterLedger {
        source_groups: groups.len(), groups, scalar_endpoints, shell_coordinates,
        query_equation_count: request.query_equation_count,
        max_source_num_vars: layout.max_num_vars(), max_point_num_vars: layout.max_point_num_vars(),
        root_group_order,
        shared_d_width: d_offset,
        shared_h_source_coefficients: root.opening_payload_geometry()?.source_coefficients(),
        setup_field_elements: akita_params::setup_matrix_field_elements_for_schedule(row.schedule())?,
        verifier_setup_field_elements: akita_params::verifier_setup_matrix_capacity_for_schedule(
            row.schedule(), &layout,
        )?.num_field_elements,
        encoded_row_bytes: row.encoded_row_bytes()?, sis_occurrences: row.schedule().sis_occurrences()?,
        grinding_plan: akita_params::derive_transcript_grinding_plan_from_public_shape(
            row.schedule(), &layout, request.policy.transcript_grinding_order()?,
            request.policy.claim_ext_degree,
        )?,
        setup_prefix_slots: row.schedule().recursive_folds.iter().filter_map(|fold| {
            fold.params.setup_prefix().and_then(|prefix| prefix.slot_id())
        }).collect(),
        joint_response: FrozenJointResponseCertificate {
            all_valid_groups, dense_groups,
            reject_numerator: (dense_groups as u128).checked_mul(5).ok_or_else(|| overflow("Dense joint numerator"))?,
            reject_denominator_log2: 48, dense_variance_bound: 9_529_458_688,
            dense_first_rejected_positive: 898_780,
            nonce_attempt_limit: akita_params::FOLD_RESPONSE_ATTEMPTS as usize,
        },
    };
    Ok(FrozenRosterPlan { row, estimate: planned.estimate, ledger })
}

#[cfg(test)]
mod tests {
    use super::*;
    use akita_config::{proof_optimized::fp128::Dense, CommitmentConfig};
    use akita_params::{
        BlockGeometry, CommitmentRingDims, CommitmentSliceCount, GadgetDigits,
        InnerCommitMatrixParams, OuterCommitMatrixParams, PolynomialGroupLayout, RoleParams,
        ScheduleLookupKey,
    };
    use crate::schedule_params::PlannerOpeningCandidate;

    fn dense_profile(policy: &PlannerPolicy) -> GroupCommitPhaseParams {
        GroupCommitPhaseParams {
            version: GroupCommitPhaseParams::VERSION,
            group: PolynomialGroupLayout::singleton(14),
            blocks: BlockGeometry::new(32, 4, 8),
            outer_slice_count: CommitmentSliceCount::try_new(8).unwrap(),
            inner: RoleParams::new(GadgetDigits::new(13, 10), InnerCommitMatrixParams::try_new(
                policy.sis_security_policy, policy.sis_table_digest, policy.sis_modulus_profile,
                1, 40, 427_818_804, 512,
            ).unwrap()),
            outer: RoleParams::new(GadgetDigits::new(3, 43), OuterCommitMatrixParams::try_new(
                policy.sis_security_policy, policy.sis_table_digest, policy.sis_modulus_profile,
                1, 344, 7, 64,
            ).unwrap()),
        }
    }

    #[test]
    fn dense_frozen_rosters_use_actual_carrier_and_complete_images() {
        let policy = akita_config::policy_of::<Dense>();
        let profile = dense_profile(&policy);
        profile.validate(128).unwrap();
        let source = Dense::committed_source_contract().unwrap();
        let dimensions = CommitmentRingDims { inner: 512, outer: 64, opening: 64 };
        let opening = PlannerOpeningCandidate::coefficient_packing(0, 1, dimensions, 64).unwrap().unwrap();
        let choices = planner::frozen_response_depth_choices(&profile, source, &policy, 64, 64, 3).unwrap();
        assert_eq!(choices.iter().map(|group| group.num_digits_fold()).collect::<Vec<_>>(), vec![7]);
        assert_eq!(opening.challenge_config().count_pm1, 31);
        assert_eq!(opening.challenge_config().count_pm2, 10);
        assert_eq!(planner::frozen_root_carrier(&profile, source).unwrap(), 64);
        for h in [1, 2, 3, 256] {
            let key = ScheduleLookupKey::decoded(profile.group, vec![profile; h - 1]);
            let contracts = vec![source; h - 1];
            let classes = planner::precommitted_classes_for_key(&key, &contracts).unwrap();
            let prepared = planner::PreparedRootProducers::prepare_frozen(
                &key, &contracts, &policy, dimensions, opening, &vec![opening; classes.len()], 3,
            ).unwrap().unwrap();
            let (root, output_len) = planner::root_candidate_for_frozen_profile(
                &profile, source, &policy, dimensions, opening, &prepared, 3,
                &key.opening_layout().unwrap(),
            ).unwrap().unwrap();
            assert_eq!(root.open_matrix.input_width(), 344 * h);
            assert_eq!(root.groups().len(), h);
            assert_eq!(root.open_matrix.output_rank(), 1);
            assert!(root.groups().iter().all(|group| group.profile == profile));
            assert_eq!(profile.outer_slice_count.complete_source_coefficients(1, 64).unwrap(), 512);
            assert!(root.compression_sources_supported().unwrap());
            assert_eq!(Some(output_len), planner::root_batch_next_w_len(128, 1, &root, &key.opening_layout().unwrap()).unwrap());
        }
    }

    #[test]
    fn dense_joint_certificate_rejects_wrong_envelopes_and_union_counts() {
        assert!(dense_joint_gate(7, 20_480, 9_529_458_688, 898_780));
        assert!(!dense_joint_gate(6, 20_480, 9_529_458_688, 898_780));
        assert!(!dense_joint_gate(7, 64, 9_529_458_688, 898_780));
        assert!(!dense_joint_gate(7, 20_480, 9_529_458_689, 898_780));
        assert!(!dense_joint_gate(7, 20_480, 9_529_458_688, 898_779));
        assert!(!dense_joint_gate(7, 20_480, 9_529_458_688, u128::MAX));
    }
}

#[cfg(test)]
mod native_roster_tests {
    use super::*;
    use akita_config::proof_optimized::fp128::OneHot;
    use akita_params::{
        BlockGeometry, CommitmentRingDims, CommitmentSliceCount, DecompositionParams, GadgetDigits,
        InnerCommitMatrixParams, OuterCommitMatrixParams, PolynomialGroupLayout, RoleParams,
        ScheduleLookupKey,
    };
    use akita_params::sis::CommittedSourceClass;
    use crate::schedule_params::PlannerOpeningCandidate;

    fn source(ram: bool) -> CommittedSourceContract {
        CommittedSourceContract::try_new(
            if ram { CommittedSourceClass::OriginalBalancedDigitPlanes }
                else { CommittedSourceClass::UnitOneHot { source_chunk_size: 1 } },
            DecompositionParams { log_basis: 5, log_commit_bound: if ram { 128 } else { 1 },
                log_open_bound: Some(128) },
        ).unwrap()
    }

    fn original(policy: &PlannerPolicy, ram: bool) -> GroupCommitPhaseParams {
        GroupCommitPhaseParams {
            version: GroupCommitPhaseParams::VERSION,
            group: PolynomialGroupLayout::new(16, if ram { 1 } else { 56 }),
            blocks: BlockGeometry::new(512, if ram { 32 } else { 512 }, if ram { 16 } else { 1 }),
            outer_slice_count: CommitmentSliceCount::ONE,
            inner: RoleParams::new(GadgetDigits::new(5, if ram { 26 } else { 1 }),
                InnerCommitMatrixParams::try_new(
                    policy.sis_security_policy, policy.sis_table_digest, policy.sis_modulus_profile,
                    3, if ram { 832 } else { 512 }, 8_126_340, 128,
                ).unwrap()),
            outer: RoleParams::new(GadgetDigits::new(5, 26), OuterCommitMatrixParams::try_new(
                policy.sis_security_policy, policy.sis_table_digest, policy.sis_modulus_profile,
                1, if ram { 1248 } else { 4368 }, 31, 128,
            ).unwrap()),
        }
    }

    #[test]
    fn vm_history_rosters_preserve_all_originals_and_sum_real_d_contributions() {
        let mut policy = akita_config::policy_of::<OneHot>();
        policy.decomposition = source(false).decomposition();
        policy.opening_basis_range = (5, 5);
        let main = original(&policy, false);
        let ram = original(&policy, true);
        main.validate(128).unwrap();
        ram.validate(128).unwrap();
        let dimensions = CommitmentRingDims::uniform(128);
        let opening = PlannerOpeningCandidate::coefficient_packing(0, 1, dimensions, 128).unwrap().unwrap();
        for h in [1, 2, 3, 256] {
            let profiles = vec![ram; h + 1].into_iter().chain(vec![main; h - 1]).collect::<Vec<_>>();
            let contracts = vec![source(true); h + 1].into_iter().chain(vec![source(false); h - 1]).collect::<Vec<_>>();
            let views = vec![SourceOpeningView::OriginalRamPlanesV1; h + 1].into_iter()
                .chain(vec![SourceOpeningView::Decoded; h]).collect();
            let key = ScheduleLookupKey::with_views(main.group, profiles.clone(), views).unwrap();
            let classes = planner::precommitted_classes_for_key(&key, &contracts).unwrap();
            assert_eq!(classes.len(), if h == 1 { 1 } else { 2 });
            let prepared = planner::PreparedRootProducers::prepare_frozen(
                &key, &contracts, &policy, dimensions, opening, &vec![opening; classes.len()], 5,
            ).unwrap().unwrap();
            let (root, next) = planner::root_candidate_for_frozen_profile(
                &main, source(false), &policy, dimensions, opening, &prepared, 5,
                &key.opening_layout().unwrap(),
            ).unwrap().unwrap();
            assert_eq!(root.open_matrix.input_width(), 1456 * h + 416 * (h + 1));
            assert_eq!(root.open_matrix.output_rank(), 1);
            assert_eq!(root.groups().len(), 2 * h + 1);
            assert_eq!(root.groups().iter().map(|group| group.profile).collect::<Vec<_>>(),
                profiles.into_iter().chain(std::iter::once(main)).collect::<Vec<_>>());
            assert!(root.groups().iter().all(|group| group.num_digits_fold() == 3));
            assert_eq!(key.opening_layout().unwrap().num_total_polynomials(), 56 * h + h + 1);
            assert_eq!(Some(next), planner::root_batch_next_w_len(128, 1, &root, &key.opening_layout().unwrap()).unwrap());
        }
    }
}
