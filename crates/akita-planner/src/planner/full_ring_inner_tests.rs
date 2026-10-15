use super::*;
use akita_config::{full_ring::CompactInner, CommitmentConfig};
use akita_params::sis::CommittedSourceClass;
use akita_params::{
    BlockGeometry, CommittedGroupBatchProfile, InnerCommitMatrixParams, SisTableDigest,
};
use akita_schedules::{PlannerOpeningFamily, ValidatedScheduleCatalog};

fn policy() -> PlannerPolicy {
    akita_config::policy_of::<CompactInner>()
}

fn contract(class: CommittedSourceClass, bits: u32) -> CommittedSourceContract {
    CommittedSourceContract::try_new(
        class,
        DecompositionParams {
            log_basis: 5,
            log_commit_bound: bits,
            log_open_bound: Some(128),
        },
    )
    .unwrap()
}

fn request() -> (
    ScheduleLookupKey,
    CommittedSourceContract,
    Vec<CommittedSourceContract>,
) {
    let (key, sources) = CompactInner::root_inputs().unwrap();
    (
        key,
        CompactInner::committed_source_contract().unwrap(),
        sources.to_vec(),
    )
}

fn generate(policy: &PlannerPolicy) -> PlannedFoldSchedule {
    let (key, main, sources) = request();
    super::find_schedule(&key, main, &sources, policy, |_| {
        Ok(akita_params::FULL_RING_FOLD_CHALLENGE)
    })
    .unwrap()
}

fn profiles(schedule: &akita_params::FoldSchedule) -> CommittedGroupBatchProfile {
    CommittedGroupBatchProfile {
        final_group: schedule.root.params.own_group().profile,
        precommitteds: schedule
            .root
            .params
            .precommitted_groups()
            .iter()
            .map(|group| group.profile)
            .collect(),
    }
}

#[test]
fn checked_compact_inner_row_reaches_catalog_and_three_shrink_terminal() {
    let policy = policy();
    let planned = generate(&policy);
    let schedule = &planned.schedule;
    assert_eq!(schedule.root.output_witness_len, 1 << 22);
    // Equal RAM contracts are still two ordered endpoint occurrences.
    assert_eq!(schedule.root.params.precommitted_groups().len(), 3);
    assert_eq!(
        schedule.root.params.precommitted_groups()[1].profile,
        schedule.root.params.precommitted_groups()[2].profile
    );
    assert_eq!(schedule.root.params.open().matrix.input_width(), 6240);
    assert_eq!(
        schedule
            .recursive_folds
            .iter()
            .map(|step| (step.input_witness_len, step.output_witness_len))
            .collect::<Vec<_>>(),
        vec![(1 << 22, 1 << 21), (1 << 21, 1 << 20), (1 << 20, 1 << 19)]
    );
    assert_eq!(schedule.terminal.blocks, BlockGeometry::new(4096, 512, 8));
    assert_eq!(planned.estimate.estimated_num_setup_field_elements, 798_720);
    let catalog = ValidatedScheduleCatalog::try_new(
        "compact-inner-test",
        [(profiles(schedule), schedule.clone())],
        &policy,
        |_| Ok(akita_params::FULL_RING_FOLD_CHALLENGE),
    )
    .unwrap();
    let (key, _, _) = request();
    let row = catalog.resolve_key(&key).unwrap();
    assert_eq!(row.schedule(), schedule);
    let mut outer = policy;
    outer.opening_family = PlannerOpeningFamily::FullRingE2OuterDirect;
    assert!(catalog
        .validate_binding("compact-inner-test", &outer, |_| Ok(
            akita_params::FULL_RING_FOLD_CHALLENGE
        ))
        .is_err());
    assert!(ResolvedScheduleRow::try_new(profiles(schedule), schedule.clone(), &outer).is_err());
    assert_ne!(
        akita_schedules::policy_digest(&policy),
        akita_schedules::policy_digest(&outer)
    );
}

#[test]
fn compact_planner_consumes_full_original_plane_norms_with_fixed_certified_roles() {
    let policy = policy();
    let (key, main, sources) = request();
    let planned = generate(&policy);
    let moments = crate::response_model::root_group_source_moments(
        &planned.schedule.root.params,
        &key.opening_layout().unwrap(),
        main,
        &sources,
    )
    .unwrap();
    let expected = crate::response_model::SourceMomentEstimate::new(26 * 256 * (1 << 16))
        .unwrap()
        .mean_l2_sq();
    assert_eq!(moments[1].mean_l2_sq(), expected);
    assert_eq!(moments[2].mean_l2_sq(), expected);
    let canonical = crate::response_model::SourceMomentEstimate::new((25 * 256 + 16) * (1 << 16))
        .unwrap()
        .mean_l2_sq();
    assert!(moments[1].mean_l2_sq() > canonical);
    let ram = &planned.schedule.root.params.precommitted_groups()[1];
    let norms = sources[1].source_norms(5, 26, 128, 1 << 16).unwrap();
    assert_eq!(norms.l2_sq, 436_207_616);
    let bound = 16 * 56 * norms.fold_witness.infinity_norm();
    assert_eq!(bound, 14_336);
    assert_eq!(akita_params::full_ring_response_depth(bound).unwrap(), 3);
    let required = akita_params::sis::rounded_up_role_a_inf_norm(
        policy.sis_security_policy,
        policy.sis_table_digest,
        policy.sis_modulus_profile,
        128,
        5,
        &akita_params::FULL_RING_FOLD_CHALLENGE,
        3,
        1,
    )
    .unwrap();
    assert_eq!(required, 8_126_340);
    assert_eq!(ram.profile.inner.matrix.coeff_linf_bound(), Some(required));
    assert_eq!(ram.profile.inner.matrix.output_rank(), 3);
    assert_eq!(ram.profile.inner.matrix.input_width(), 32 * 26);
    assert_eq!(ram.profile.blocks, BlockGeometry::new(512, 32, 16));
    ResolvedScheduleRow::try_new(profiles(&planned.schedule), planned.schedule, &policy).unwrap();
}

#[test]
fn inner_catalog_rejects_topology_source_method_and_norm_mutations() {
    let policy = policy();
    let schedule = generate(&policy).schedule;
    let reject = |bad: akita_params::FoldSchedule| {
        assert!(ResolvedScheduleRow::try_new(profiles(&bad), bad, &policy).is_err());
    };
    let mut bad = schedule.clone();
    bad.recursive_folds.pop();
    reject(bad);
    let mut bad = schedule.clone();
    bad.recursive_folds.swap(0, 1);
    reject(bad);
    let mut bad = schedule.clone();
    bad.recursive_folds[0].params.source_encoding =
        akita_params::CommittedSourceEncoding::TensorSubfieldProjection {
            extension_degree: 2,
        };
    reject(bad);
    let mut bad = schedule.clone();
    bad.root
        .params
        .preceding_group_mut_for_test(0)
        .unwrap()
        .opening
        .num_digits_fold = 3;
    reject(bad);
    let mut bad = schedule.clone();
    bad.root
        .params
        .preceding_group_mut_for_test(1)
        .unwrap()
        .profile
        .inner
        .digits
        .num_digits = 1;
    reject(bad);
    let mut bad = schedule.clone();
    bad.terminal.response_codec = akita_params::TerminalResponseCodec::SignedFixedWidth {
        bytes_per_coefficient: 2,
        coefficient_bound: 7169,
    };
    reject(bad);
    let mut bad = schedule.clone();
    // This is a valid table-backed matrix at a smaller collision radius, but
    // it does not certify the three-digit all-valid Main response envelope.
    bad.root.params.own_group_mut().profile.inner.matrix = InnerCommitMatrixParams::try_new(
        policy.sis_security_policy,
        policy.sis_table_digest,
        policy.sis_modulus_profile,
        3,
        512,
        835_380,
        128,
    )
    .unwrap();
    reject(bad);
    let mut bad = schedule;
    bad.recursive_folds[1].params.ring_relation_mode = akita_params::RingRelationMode::QuotientLift;
    reject(bad);
}

#[test]
fn inner_generation_rejects_source_and_admission_mutations() {
    let policy = policy();
    let (key, main, sources) = request();
    let plan = |key: &ScheduleLookupKey,
                main,
                sources: &[CommittedSourceContract],
                policy: &PlannerPolicy| {
        super::find_schedule(key, main, sources, policy, |_| {
            Ok(akita_params::FULL_RING_FOLD_CHALLENGE)
        })
    };
    assert!(plan(
        &key,
        contract(CommittedSourceClass::BalancedSignedDigit, 1),
        &sources,
        &policy
    )
    .is_err());
    let mut bad_sources = sources.clone();
    bad_sources[1] = contract(CommittedSourceClass::BalancedSignedDigit, 5);
    assert!(plan(&key, main, &bad_sources, &policy).is_err());
    // Full decoded width is still not the original 26-plane envelope.
    bad_sources[1] = contract(CommittedSourceClass::BalancedSignedDigit, 128);
    assert!(plan(&key, main, &bad_sources, &policy).is_err());
    let mut bad_key = key.clone();
    bad_key.precommitteds[0].inner.digits.num_digits = 2;
    assert!(plan(&bad_key, main, &sources, &policy).is_err());
    let mut bad_policy = policy;
    bad_policy.sis_table_digest = SisTableDigest([0; 32]);
    assert!(plan(&key, main, &sources, &bad_policy).is_err());
    bad_policy = policy;
    bad_policy.setup_field_budget = Some(798_719);
    assert!(plan(&key, main, &sources, &bad_policy).is_err());
    bad_policy.setup_field_budget = Some(798_720);
    assert!(plan(&key, main, &sources, &bad_policy).is_ok());
    bad_policy = policy;
    bad_policy.recursive_setup_planning = true;
    assert!(plan(&key, main, &sources, &bad_policy).is_err());
    bad_policy = policy;
    bad_policy.selective_l2_response_model =
        akita_schedules::SelectiveL2ResponseModelId::TypedProtocolMomentsV1;
    assert!(plan(&key, main, &sources, &bad_policy).is_err());
}

fn frozen_native_policy() -> PlannerPolicy {
    let mut policy = policy();
    policy.opening_family = PlannerOpeningFamily::Legacy;
    policy.claim_ext_degree = 1;
    policy.chal_ext_degree = 1;
    policy.inner_basis_range = (4, 5);
    policy.opening_basis_range = (5, 5);
    policy.selective_l2_response_model = crate::SelectiveL2ResponseModelId::Disabled;
    policy.recursive_setup_planning = false;
    policy.selection_policy =
        crate::SelectionPolicyId::for_policy(false, policy.ring_dimension_schedule_mode);
    policy.recursive_split_search_policy = crate::RecursiveSplitSearchPolicy::Exhaustive;
    policy
}

fn production(dimension: usize) -> Result<akita_challenges::SparseChallengeConfig, AkitaError> {
    akita_challenges::SparseChallengeConfig::production_for_ring_dim(dimension)
        .ok_or_else(|| AkitaError::InvalidSetup("no production shell".into()))
}

fn frozen_native_inputs() -> (CommittedGroupBatchProfile, Vec<CommittedSourceContract>) {
    // Obtain Main's complete profile from the actual separately audited compact
    // row, not a scalar guide or a reconstruction from its lookup shape.
    let compact = generate(&policy());
    let (_, _, sources) = request();
    let compact_profiles = profiles(&compact.schedule);
    (
        CommittedGroupBatchProfile {
            final_group: compact_profiles.final_group,
            // The receiver belongs to accumulation, not this ordinary control.
            precommitteds: compact_profiles.precommitteds[1..].to_vec(),
        },
        sources[1..].to_vec(),
    )
}

#[test]
fn frozen_native_materialization_preserves_overprovisioned_profiles_and_order() {
    let policy = frozen_native_policy();
    let (mut profiles, mut sources) = frozen_native_inputs();
    // Frozen B may own a smaller basis than its consumer. Its width continues
    // to use the original A rank, not the minimum rank for the new shell.
    profiles.final_group.inner.matrix = InnerCommitMatrixParams::try_new(
        policy.sis_security_policy,
        policy.sis_table_digest,
        policy.sis_modulus_profile,
        4,
        512,
        8_126_340,
        128,
    )
    .unwrap();
    profiles.final_group.outer.digits = akita_params::GadgetDigits::new(4, 32);
    profiles.final_group.outer.matrix = akita_params::OuterCommitMatrixParams::try_new(
        policy.sis_security_policy,
        policy.sis_table_digest,
        policy.sis_modulus_profile,
        1,
        56 * 4 * 32,
        31,
        128,
    )
    .unwrap();
    let mut distinct_ram = profiles.precommitteds[0];
    distinct_ram.outer.matrix = akita_params::OuterCommitMatrixParams::try_new(
        policy.sis_security_policy,
        policy.sis_table_digest,
        policy.sis_modulus_profile,
        2,
        distinct_ram.outer.matrix.input_width(),
        31,
        128,
    )
    .unwrap();
    profiles.precommitteds.insert(1, distinct_ram);
    sources.insert(1, sources[0]);
    profiles.validate(128).unwrap();
    let key =
        ScheduleLookupKey::decoded(profiles.final_group.group, profiles.precommitteds.clone());
    let dimensions = CommitmentRingDims::uniform(128);
    let opening = PlannerOpeningCandidate::coefficient_packing(0, 1, dimensions, 128)
        .unwrap()
        .unwrap();
    let classes = precommitted_classes_for_key(&key, &sources).unwrap();
    let prepared = PreparedRootProducers::prepare_frozen(
        &key,
        &sources,
        &policy,
        dimensions,
        opening,
        &vec![opening; classes.len()],
        5,
    )
    .unwrap()
    .unwrap();
    let (root, next_len) = root_candidate_for_frozen_profile(
        &profiles.final_group,
        CompactInner::committed_source_contract().unwrap(),
        &policy,
        dimensions,
        opening,
        &prepared,
        5,
        &key.opening_layout().unwrap(),
    )
    .unwrap()
    .unwrap();
    assert_eq!(root.own_group().profile, profiles.final_group);
    assert_eq!(
        root.precommitted_groups()
            .iter()
            .map(|g| g.profile)
            .collect::<Vec<_>>(),
        profiles.precommitteds
    );
    assert_eq!(root.inner().matrix.output_rank(), 4);
    assert_eq!(root.inner().matrix.coeff_linf_bound(), Some(8_126_340));
    assert_eq!(root.outer().digits, akita_params::GadgetDigits::new(4, 32));
    assert_eq!(root.open().digits.log_basis, 5);
    assert_eq!(root.fold_challenge_config(), production(128).unwrap());
    let required = rounded_up_role_a_inf_norm(
        policy.sis_security_policy,
        policy.sis_table_digest,
        policy.sis_modulus_profile,
        128,
        5,
        &production(128).unwrap(),
        root.num_digits_fold(),
        1,
    )
    .unwrap();
    assert!(required < root.inner().matrix.coeff_linf_bound().unwrap());
    let minimum = InnerCommitMatrixParams::try_new_with_min_rank(
        root.inner().matrix.sis_table_key().unwrap(),
        512,
    )
    .unwrap();
    assert!(minimum.output_rank() < root.inner().matrix.output_rank());
    assert_eq!(
        Some(next_len),
        root_batch_next_w_len(128, 1, &root, &key.opening_layout().unwrap(),).unwrap()
    );
    assert_eq!(
        root.open().matrix.input_width(),
        root.precommitted_groups()
            .iter()
            .chain(std::iter::once(root.own_group()))
            .map(|g| g.d_segment_width(1, 128).unwrap())
            .sum::<usize>()
    );
}

#[test]
fn frozen_native_rejects_insufficient_capacity_without_replacing_geometry() {
    let policy = frozen_native_policy();
    let (mut profiles, sources) = frozen_native_inputs();
    profiles.final_group.inner.matrix = InnerCommitMatrixParams::try_new(
        policy.sis_security_policy,
        policy.sis_table_digest,
        policy.sis_modulus_profile,
        3,
        512,
        868,
        128,
    )
    .unwrap();
    profiles.validate(128).unwrap();
    let error = find_schedule_for_frozen_profiles(
        &profiles,
        CompactInner::committed_source_contract().unwrap(),
        &sources,
        &vec![akita_params::SourceOpeningView::Decoded; (&profiles).precommitteds.len() + 1],
        &policy,
        production,
    )
    .unwrap_err();
    assert!(
        matches!(error, AkitaError::UnsupportedSchedule(_)),
        "{error:?}"
    );
    // A well-formed profile outside the chosen finite source-basis domain
    // cannot cause a retry at the policy's otherwise usable basis.
    let (profiles, sources) = frozen_native_inputs();
    let mut incompatible = policy;
    incompatible.inner_basis_range = (4, 4);
    assert!(matches!(
        find_schedule_for_frozen_profiles(
            &profiles,
            CompactInner::committed_source_contract().unwrap(),
            &sources,
            &vec![akita_params::SourceOpeningView::Decoded; profiles.precommitteds.len() + 1],
            &incompatible,
            production,
        ),
        Err(AkitaError::UnsupportedSchedule(_))
    ));
    let mut malformed = profiles;
    malformed.final_group.inner.digits.num_digits = 26;
    assert!(matches!(
        find_schedule_for_frozen_profiles(
            &malformed,
            CompactInner::committed_source_contract().unwrap(),
            &sources,
            &vec![akita_params::SourceOpeningView::Decoded; malformed.precommitteds.len() + 1],
            &policy,
            production,
        ),
        Err(AkitaError::InvalidSetup(_)) | Err(AkitaError::InvalidInput(_))
    ));
}

#[test]
fn frozen_native_complete_compact_control_passes_catalog_and_estimate_admission() {
    let policy = frozen_native_policy();
    let (profiles, sources) = frozen_native_inputs();
    let planned = find_schedule_for_frozen_profiles(
        &profiles,
        CompactInner::committed_source_contract().unwrap(),
        &sources,
        &vec![akita_params::SourceOpeningView::Decoded; (&profiles).precommitteds.len() + 1],
        &policy,
        production,
    )
    .unwrap();
    let schedule = &planned.schedule;
    assert_eq!(
        schedule.root.params.own_group().profile,
        profiles.final_group
    );
    assert_eq!(
        schedule
            .root
            .params
            .precommitted_groups()
            .iter()
            .map(|g| g.profile)
            .collect::<Vec<_>>(),
        profiles.precommitteds
    );
    assert!(!schedule.recursive_folds.is_empty());
    for group in schedule
        .root
        .params
        .precommitted_groups()
        .iter()
        .chain(std::iter::once(schedule.root.params.own_group()))
    {
        assert_eq!(
            group.opening.opening_method,
            akita_params::OpeningMethod::SubringCoefficientPacking {
                challenge_subring_dimension: 128,
            }
        );
        assert_eq!(
            group.opening.fold_challenge_config,
            production(128).unwrap()
        );
        assert_eq!(group.num_digits_fold(), 3);
    }
    assert!(matches!(
        schedule.recursive_folds[0].params.opening_method(),
        akita_params::OpeningMethod::SubringCoefficientPacking { .. }
    ));
    for step in schedule.recursive_folds.iter().skip(1) {
        assert_eq!(
            step.params.opening_method(),
            akita_params::OpeningMethod::EvaluationTrace
        );
    }
    assert_eq!(
        schedule.terminal.opening_method,
        akita_params::OpeningMethod::EvaluationTrace
    );
    let catalog = ValidatedScheduleCatalog::try_new(
        "frozen-native-compact-control-test",
        [(profiles.clone(), schedule.clone())],
        &policy,
        production,
    )
    .unwrap();
    assert_eq!(
        catalog
            .resolve_profiles(
                &profiles,
                &vec![akita_params::SourceOpeningView::Decoded; profiles.precommitteds.len() + 1]
            )
            .unwrap()
            .schedule(),
        schedule
    );
    let key =
        ScheduleLookupKey::decoded(profiles.final_group.group, profiles.precommitteds.clone());
    let estimate =
        akita_schedules::expanded_schedule_proof_estimate_bytes(&key, schedule, &policy).unwrap();
    assert_eq!(
        estimate,
        planned.estimate.estimated_proof_payload_bytes().unwrap()
    );
    let decoded_views =
        vec![akita_params::SourceOpeningView::Decoded; profiles.precommitteds.len() + 1];
    let mut raw_views = vec![
        akita_params::SourceOpeningView::OriginalRamPlanesV1;
        profiles.precommitteds.len() + 1
    ];
    *raw_views.last_mut().unwrap() = akita_params::SourceOpeningView::Decoded;
    let raw = find_schedule_for_frozen_profiles(
        &profiles,
        CompactInner::committed_source_contract().unwrap(),
        &sources,
        &raw_views,
        &policy,
        production,
    )
    .unwrap();
    let combined = ValidatedScheduleCatalog::try_new(
        "frozen-view-coexistence-test",
        [
            (profiles.clone(), schedule.clone()),
            (profiles.clone(), raw.schedule.clone()),
        ],
        &policy,
        production,
    )
    .unwrap();
    let decoded_row = combined
        .resolve_profiles(&profiles, &decoded_views)
        .unwrap();
    let raw_row = combined.resolve_profiles(&profiles, &raw_views).unwrap();
    assert_ne!(decoded_row.selection(), raw_row.selection());
    assert!(raw_row
        .validate_opening_layout(&key.opening_layout().unwrap())
        .is_err());
    assert_eq!(raw_row.opening_layout().unwrap().max_point_num_vars(), 21);
    let raw_layout = raw_row.opening_layout().unwrap();
    assert!(raw_layout
        .views()
        .contains(&akita_params::SourceOpeningView::OriginalRamPlanesV1));
    assert!(raw_layout
        .views()
        .contains(&akita_params::SourceOpeningView::Decoded));
    // Degree-one resource admission must retain raw geometry without entering
    // the decoded-only EOR shape builder, even for a trace-method sizing seed.
    let mut trace_seed = raw.schedule.root.params.clone();
    trace_seed.own_group_mut().opening.opening_method =
        akita_params::OpeningMethod::EvaluationTrace;
    let successor = akita_params::FoldSuccessor::Recursive(&raw.schedule.recursive_folds[0].params);
    let payload = akita_schedules::planner_support::nonterminal_level_payload_bytes(
        &policy,
        &trace_seed,
        &raw_layout,
        successor,
        raw.schedule.root.output_witness_len,
    )
    .unwrap();
    let expected = akita_params::nonterminal_level_layout(
        policy.decomposition.field_bits(),
        policy.challenge_field_bits().unwrap(),
        &trace_seed,
        payload.relation_geometry,
        Some(&raw.schedule.recursive_folds[0].params),
    )
    .unwrap()
    .encoded_len()
    .unwrap();
    assert_eq!(payload.direct, expected);
    assert!(raw_layout.aggregate_polynomial_group_layout().is_err());
    for degree in [2, 4] {
        let mut extension_policy = policy.clone();
        extension_policy.claim_ext_degree = degree;
        assert!(
            akita_schedules::planner_support::nonterminal_level_payload_bytes(
                &extension_policy,
                &trace_seed,
                &raw_layout,
                successor,
                raw.schedule.root.output_witness_len,
            )
            .is_err()
        );
    }
    let raw_key = raw_row.lookup_key().unwrap();
    assert_eq!(
        akita_schedules::expanded_schedule_proof_estimate_bytes(
            &raw_key,
            raw_row.schedule(),
            &policy
        )
        .unwrap(),
        raw.estimate.estimated_proof_payload_bytes().unwrap(),
    );
    // Compile and price the entire native root, all suffix levels, and terminal
    // from the admitted mixed-view row, not a root-only or observed transcript.
    let registry = akita_config::approved_native_registry_from_trusted_bounds::<
        akita_config::full_ring::OrdinaryControl,
    >(raw_row, akita_params::BasisMode::Lagrange, 7, 19, 91)
    .unwrap();
    let resources =
        akita_config::NativeRegistryResources::from_trusted_bounds(registry.events(), 7, 19)
            .unwrap();
    assert!(resources.limits.max_proof_bytes > 0);
    assert!(resources.limits.max_answer_bytes > 0);
    assert!(resources.limits.max_result_bytes > 0);
    let explicit =
        akita_config::approved_native_registry::<akita_config::full_ring::OrdinaryControl>(
            raw_row,
            akita_params::BasisMode::Lagrange,
            &resources,
            91,
        )
        .unwrap();
    assert_eq!(explicit.events(), registry.events());
    let bytes = combined.to_artifact_bytes().unwrap();
    let reloaded = ValidatedScheduleCatalog::from_artifact_bytes(
        &bytes,
        "frozen-view-coexistence-test",
        &policy,
        production,
    )
    .unwrap();
    assert_eq!(
        reloaded
            .resolve_profiles(&profiles, &raw_views)
            .unwrap()
            .selection(),
        raw_row.selection()
    );
    assert!(reloaded
        .resolve_profiles(&profiles, &raw_views[..raw_views.len() - 1])
        .is_err());
    let raw_only = ValidatedScheduleCatalog::try_new(
        "frozen-raw-only-test",
        [(profiles.clone(), raw.schedule)],
        &policy,
        production,
    )
    .unwrap();
    assert!(raw_only
        .resolve_profiles(&profiles, &decoded_views)
        .is_err());
    let mut bad = schedule.clone();
    bad.terminal.input_witness_len += 1;
    assert!(ResolvedScheduleRow::try_new(profiles.clone(), bad, &policy).is_err());
    let mut bad = schedule.clone();
    bad.recursive_folds[0]
        .params
        .own_group_mut()
        .opening
        .opening_method = akita_params::OpeningMethod::EvaluationTrace;
    assert!(ResolvedScheduleRow::try_new(profiles, bad, &policy).is_err());
}

#[test]
fn frozen_response_domain_includes_all_valid_depth_and_stops_at_original_capacity() {
    let policy = frozen_native_policy();
    let (profiles, sources) = frozen_native_inputs();
    for (profile, source) in profiles
        .precommitteds
        .iter()
        .zip(sources.iter().copied())
        .chain(std::iter::once((
            &profiles.final_group,
            CompactInner::committed_source_contract().unwrap(),
        )))
    {
        let choices = frozen_response_depth_choices(profile, source, &policy, 128, 128, 5).unwrap();
        assert!(choices.iter().any(|group| group.num_digits_fold() == 3));
        assert!(choices.iter().all(|group| group.num_digits_fold() <= 3));
        assert!(choices.iter().all(|group| group.profile == *profile));
        let reach = akita_params::sis::checked_balanced_digit_representable_bounds(5, 3)
            .1
            .unwrap();
        assert!(
            frozen_deterministic_cap(profile, source, &production(128).unwrap()).unwrap() <= reach
        );
    }
}

#[test]
fn frozen_prefix_accepts_earlier_main_and_keeps_raw_ram_views_distinct() {
    let policy = frozen_native_policy();
    let (mut profiles, mut sources) = frozen_native_inputs();
    let main = CompactInner::committed_source_contract().unwrap();
    profiles.precommitteds.push(profiles.final_group);
    sources.push(main);
    let mut views = vec![akita_params::SourceOpeningView::OriginalRamPlanesV1; sources.len() + 1];
    views[sources.len() - 1] = akita_params::SourceOpeningView::Decoded;
    views[sources.len()] = akita_params::SourceOpeningView::Decoded;
    let key = ScheduleLookupKey::with_views(
        profiles.final_group.group,
        profiles.precommitteds.clone(),
        views.clone(),
    )
    .unwrap();
    let classes = precommitted_classes_for_key(&key, &sources).unwrap();
    assert_eq!(classes.len(), 2);
    let opening =
        PlannerOpeningCandidate::coefficient_packing(0, 1, CommitmentRingDims::uniform(128), 128)
            .unwrap()
            .unwrap();
    let prepared = PreparedRootProducers::prepare_frozen(
        &key,
        &sources,
        &policy,
        CommitmentRingDims::uniform(128),
        opening,
        &vec![opening; classes.len()],
        5,
    )
    .unwrap()
    .unwrap();
    let (root, next_len) = root_candidate_for_frozen_profile(
        &profiles.final_group,
        main,
        &policy,
        CommitmentRingDims::uniform(128),
        opening,
        &prepared,
        5,
        &key.opening_layout().unwrap(),
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        root.groups()
            .iter()
            .map(|group| group.profile)
            .collect::<Vec<_>>(),
        profiles
            .precommitteds
            .iter()
            .copied()
            .chain(std::iter::once(profiles.final_group))
            .collect::<Vec<_>>()
    );
    assert_eq!(
        root.groups()
            .iter()
            .map(|group| group.opening.source_view())
            .collect::<Vec<_>>(),
        views
    );
    assert!(root
        .groups()
        .iter()
        .all(|group| group.num_digits_fold() == 3));
    assert_eq!(
        Some(next_len),
        root_batch_next_w_len(128, 1, &root, &key.opening_layout().unwrap()).unwrap()
    );
    let decoded = ScheduleLookupKey::decoded(key.final_group, key.precommitteds.clone());
    assert_ne!(key.canonical_order_key(), decoded.canonical_order_key());
    // Counterfeit source class cannot gain raw authority merely by matching
    // numeric profile geometry; rejection precedes the full suffix search.
    sources[0] = contract(CommittedSourceClass::BalancedSignedDigit, 128);
    assert!(matches!(
        find_schedule_for_frozen_profiles(&profiles, main, &sources, &views, &policy, production),
        Err(AkitaError::InvalidInput(_)) | Err(AkitaError::InvalidSetup(_))
    ));
}
