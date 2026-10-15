//! Fixed compact inner route from COMPACT_PROFILE and MATRIX_REGISTRY.
//! Matrix constructors and the independent catalog audit remain authoritative;
//! this generator does not provision matrices or certify a security profile.

use super::*;
use akita_params::sis::CommittedSourceClass;
use akita_params::{
    BlockGeometry, CommittedGroupBatchProfile, FoldParams, FoldSchedule, FoldScheduleEstimate,
    FoldSuccessor, GadgetDigits, InnerRoleParams, TailSegmentLayout, TerminalFoldParams,
    TerminalResponseCodec, TerminalResponseShape,
};

fn unsupported(detail: &str) -> AkitaError {
    AkitaError::UnsupportedSchedule(format!("compact inner: {detail}"))
}

fn validate_sources(
    key: &ScheduleLookupKey,
    main: CommittedSourceContract,
    sources: &[CommittedSourceContract],
) -> Result<(), AkitaError> {
    // A one-coefficient chunk is the existing direct Boolean contract: it
    // imposes no sparsity restriction beyond membership in {0,1}.
    if key.final_group != PolynomialGroupLayout::new(16, 56)
        || main.class()
            != (CommittedSourceClass::UnitOneHot {
                source_chunk_size: 1,
            })
        || main.decomposition().log_commit_bound != 1
        || key.precommitteds.len() != 3
        || sources.len() != 3
    {
        return Err(unsupported(
            "requires direct Boolean Main and three original receiver/RAM handles",
        ));
    }
    let mut receiver_count = 0;
    let mut ram_count = 0;
    for (profile, source) in key.precommitteds.iter().zip(sources) {
        let (bound, class) = if profile.group == PolynomialGroupLayout::singleton(22) {
            receiver_count += 1;
            (5, CommittedSourceClass::BalancedSignedDigit)
        } else if profile.group == PolynomialGroupLayout::singleton(16) {
            ram_count += 1;
            (128, CommittedSourceClass::OriginalBalancedDigitPlanes)
        } else {
            return Err(unsupported("unsupported original source layout"));
        };
        if source.class() != class
            || source.decomposition().log_basis != 5
            || source.decomposition().log_commit_bound != bound
            || source.decomposition().field_bits() != 128
        {
            return Err(unsupported(
                "original source contract disagrees with compact digit alphabet",
            ));
        }
    }
    if receiver_count != 1 || ram_count != 2 {
        return Err(unsupported("requires one receiver and two RAM endpoints"));
    }
    Ok(())
}

fn level(
    policy: &PlannerPolicy,
    source: CommittedSourceContract,
    group: PolynomialGroupLayout,
    position_bits: usize,
    block_bits: usize,
    producers: &PreparedRootProducers,
) -> Result<CommittedGroupParams, AkitaError> {
    let dimensions = CommitmentRingDims::uniform(128);
    let opening = PlannerOpeningCandidate::full_ring_scalar_extension(2, dimensions)?;
    let source_basis = match source.class() {
        CommittedSourceClass::UnitOneHot { .. } => crate::InnerBasisSource::UnitOneHot,
        CommittedSourceClass::BalancedSignedDigit
        | CommittedSourceClass::OriginalBalancedDigitPlanes => {
            crate::InnerBasisSource::RawCoefficients {
                log_bound: source.decomposition().log_commit_bound,
            }
        }
    };
    root_final_group_level_params_candidate(
        &MultiGroupRootCandidateCtx {
            policy,
            dimensions,
            opening,
            final_source_contract: source,
            final_num_vars: group.num_vars(),
            main_num_polys: group.num_polynomials(),
            source: source_basis,
        },
        RootFinalGroupCandidateInput {
            log_basis_inner: 5,
            log_basis_open: 5,
            position_index_bits: position_bits,
            block_index_bits: block_bits,
            outer_slice_count: akita_params::CommitmentSliceCount::ONE,
            precommitted_groups: &producers.groups,
            precommitted_d_width: producers.d_width,
        },
    )?
    .ok_or_else(|| unsupported("no checked A/B/D candidate for required geometry"))
}

fn terminal(policy: &PlannerPolicy) -> Result<TerminalFoldParams, AkitaError> {
    let codec = TerminalResponseCodec::full_ring(8)?;
    let TerminalResponseCodec::SignedFixedWidth {
        coefficient_bound, ..
    } = codec
    else {
        return Err(unsupported("terminal requires a signed fixed-width codec"));
    };
    let collision =
        akita_params::sis::role_a_collision_inf_norm_for_response_bound(56, coefficient_bound)
            .ok_or_else(|| unsupported("terminal collision bound overflow"))?;
    let key = akita_params::sis::sis_table_key_for_linf_bound(
        policy.sis_security_policy,
        policy.sis_table_digest,
        policy.sis_modulus_profile,
        SisMatrixRole::Inner,
        128,
        collision,
    )
    .ok_or_else(|| unsupported("no checked terminal A collision certificate"))?;
    let matrix = akita_params::InnerCommitMatrixParams::try_new_with_min_rank(key, 512)?;
    let mut params = TerminalFoldParams {
        opening_method: akita_params::OpeningMethod::FULL_RING_E2,
        source_encoding: akita_params::CommittedSourceEncoding::CanonicalCoefficientTable,
        response_codec: codec,
        blocks: BlockGeometry::new(4096, 512, 8),
        inner: InnerRoleParams::new(GadgetDigits::new(5, 1), matrix),
        fold: GadgetDigits::new(
            5,
            akita_params::full_ring_response_depth(coefficient_bound)?,
        ),
        fold_challenge_config: akita_params::FULL_RING_FOLD_CHALLENGE,
        response_shape: TerminalResponseShape {
            layout: TailSegmentLayout {
                ring_dimension: 128,
                groups: Vec::new(),
                logical_num_elems: 0,
            },
        },
        input_witness_len: 1 << 19,
    };
    params.response_shape = TerminalResponseShape::derive(&params, coefficient_bound)?;
    params.validate_opening_execution(2)?;
    Ok(params)
}

pub(super) fn find_schedule(
    key: &ScheduleLookupKey,
    main: CommittedSourceContract,
    sources: &[CommittedSourceContract],
    policy: &PlannerPolicy,
) -> Result<PlannedFoldSchedule, AkitaError> {
    validate_sources(key, main, sources)?;
    let dimensions = CommitmentRingDims::uniform(128);
    let opening = PlannerOpeningCandidate::full_ring_scalar_extension(2, dimensions)?;
    let classes = crate::planner::precommitted_classes_for_key(key, sources)?;
    let producers = PreparedRootProducers::prepare(
        key,
        sources,
        policy,
        dimensions,
        opening,
        &vec![opening; classes.len()],
        5,
    )?
    .ok_or_else(|| unsupported("original producer admission failed"))?;
    let root = level(policy, main, key.final_group, 9, 0, &producers)?;
    let root_layout = key.opening_layout()?;
    let root_output = root.output_witness_len_for_field_bits(128, 2, &root_layout)?;
    let receiver_source = CommittedSourceContract::try_new(
        CommittedSourceClass::BalancedSignedDigit,
        DecompositionParams {
            log_commit_bound: 5,
            ..policy.decomposition
        },
    )?;
    let no_precommits = PreparedRootProducers {
        groups: Vec::new(),
        d_width: 0,
    };
    let mut recursive_folds = Vec::with_capacity(3);
    for (vars, position_bits, block_bits) in [(22, 10, 5), (21, 10, 4), (20, 9, 4)] {
        let params = level(
            policy,
            receiver_source,
            PolynomialGroupLayout::singleton(vars),
            position_bits,
            block_bits,
            &no_precommits,
        )?;
        let layout = OpeningClaimsLayout::new(vars, 1)?;
        let output_witness_len = params.output_witness_len_for_field_bits(128, 2, &layout)?;
        recursive_folds.push(FoldParams {
            params,
            input_witness_len: 1 << vars,
            output_witness_len,
        });
    }
    let schedule = FoldSchedule {
        root: FoldParams {
            input_witness_len: akita_params::root_input_witness_len(&root),
            output_witness_len: root_output,
            params: root,
        },
        recursive_folds,
        terminal: terminal(policy)?,
    };
    let profiles = CommittedGroupBatchProfile {
        final_group: schedule.root.params.own_group().profile,
        precommitteds: key.precommitteds.clone(),
    };
    // Includes the fixed route, every accepted norm, matrix table and canonical
    // Eq109 transition check. No estimated shape is substituted for admission.
    ResolvedScheduleRow::try_new(profiles, schedule.clone(), policy)?;
    let expanded_setup = akita_params::setup_matrix_field_elements_for_schedule(&schedule)?;
    if policy
        .setup_field_budget
        .is_some_and(|limit| expanded_setup > limit)
    {
        return Err(unsupported(
            "native shared-prefix setup exceeds policy budget",
        ));
    }
    let root_cost = akita_schedules::planner_support::nonterminal_level_payload_bytes(
        policy,
        &schedule.root.params,
        &root_layout,
        FoldSuccessor::Recursive(&schedule.recursive_folds[0].params),
        root_output,
    )?;
    let mut recursive_costs = Vec::with_capacity(3);
    for (index, step) in schedule.recursive_folds.iter().enumerate() {
        let successor = schedule
            .recursive_folds
            .get(index + 1)
            .map_or(FoldSuccessor::Terminal(&schedule.terminal), |next| {
                FoldSuccessor::Recursive(&next.params)
            });
        let layout = OpeningClaimsLayout::new(step.params.own_group().profile.group.num_vars(), 1)?;
        recursive_costs.push(
            akita_schedules::planner_support::nonterminal_level_payload_bytes(
                policy,
                &step.params,
                &layout,
                successor,
                step.output_witness_len,
            )?
            .direct,
        );
    }
    let terminal_bytes = schedule.terminal.fixed_payload_bytes(128)?;
    let estimate = FoldScheduleEstimate {
        nonce_max_bytes: 0,
        estimated_root_direct_payload_bytes: root_cost.direct,
        estimated_root_stage3_payload_bytes: root_cost.stage3,
        estimated_recursive_direct_payload_bytes: recursive_costs,
        estimated_recursive_stage3_payload_bytes: vec![0; 3],
        estimated_terminal_direct_payload_bytes: terminal_bytes,
        estimated_terminal_response_payload_bytes: terminal_bytes,
        estimated_num_setup_field_elements: expanded_setup,
        first_direct_setup_field_len: None,
        selected_offload_edges: 0,
    };
    Ok(PlannedFoldSchedule { schedule, estimate })
}
