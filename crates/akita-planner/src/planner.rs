//! Root schedule planning.

use std::time::Instant;

use akita_error::{checked, AkitaError};
use akita_params::sis::{
    decomposed_s_block_ring_count, num_digits_open, rounded_up_collision_inf_norm,
    rounded_up_role_a_inf_norm, CommittedSourceContract, HonestFoldPolicy, HonestFoldSizingQuery,
    OpenCommitMatrixParams, SisMatrixRole,
};
use akita_params::ScheduleLookupKey;
use akita_params::{
    CommitmentRingDims, CommittedGroupParams, DecompositionParams, GroupCommitPhaseParams,
    GroupOpenPhaseParams, OpeningClaimsLayout, PlannedFoldSchedule, PolynomialGroupLayout,
    PrecommittedGroupAdmissionPolicy,
};

use akita_schedules::planner_support::projected_collision_role_price;
use akita_schedules::ResolvedScheduleRow;

use crate::schedule_params::{
    derive_ab_commitment_candidate, derive_selected_suffix_schedule,
    materialize_candidate_schedule, recursive_split_search_domain, select_complete_candidate,
    AbCommitmentCandidateRequest, CandidateMaterializationCost, PlannerOpeningCandidate,
    RingChallengeConfigFn, ScheduleMemo, SuffixCtx, SuffixState,
};
use crate::PlannerPolicy;

mod full_ring_inner;
#[cfg(test)]
mod full_ring_inner_tests;

#[cfg(test)]
#[path = "test/adapted_schedule.rs"]
mod adapted_schedule_tests;
#[cfg(all(test, feature = "catalog-gen"))]
#[path = "test/root_candidates.rs"]
mod root_candidates;
#[cfg(all(test, feature = "catalog-gen"))]
pub(crate) use root_candidates::{
    exhaustive_root_candidates_for_reference, root_level_candidates_with_fresh_preparation,
};

type PrecommittedGroupSeed = (GroupCommitPhaseParams, CommittedSourceContract);

#[derive(Clone, Copy)]
pub(crate) struct ScheduleSearchOptions<'a> {
    pub(crate) relation_traversal_order: super::schedule_params::RelationTraversalOrder,
    pub(crate) relation_mode_filter: super::schedule_params::RelationModeFilter,
    pub(crate) root_main_constraint: Option<&'a CommittedGroupParams>,
    pub(crate) root_final_profile: Option<&'a GroupCommitPhaseParams>,
    pub(crate) adaptation_guide: Option<&'a akita_params::FoldSchedule>,
    #[cfg(test)]
    pub(crate) query_prefix_count: u64,
}

impl ScheduleSearchOptions<'_> {
    const fn canonical() -> Self {
        Self {
            relation_traversal_order: super::schedule_params::RelationTraversalOrder::Canonical,
            relation_mode_filter: super::schedule_params::RelationModeFilter::All,
            root_main_constraint: None,
            root_final_profile: None,
            adaptation_guide: None,
            #[cfg(test)]
            query_prefix_count: 0,
        }
    }
}

/// Interchangeable metadata classes include producer profile, contract and
/// consuming view. Class multiplicity never aliases original identities.
pub(crate) fn precommitted_classes_for_key(
    key: &ScheduleLookupKey,
    contracts: &[CommittedSourceContract],
) -> Result<Vec<Vec<usize>>, AkitaError> {
    let layout = key.opening_layout()?;
    let mut classes: Vec<Vec<usize>> = Vec::new();
    if key.precommitteds.len() != contracts.len() {
        return Err(AkitaError::InvalidInput("one contract per original is required".into()));
    }
    for index in 0..contracts.len() {
        let view = layout.group_view(index)?;
        let mut matching = None;
        for (class, indices) in classes.iter().enumerate() {
            let representative = indices[0];
            if key.precommitteds[representative] == key.precommitteds[index]
                && contracts[representative] == contracts[index]
                && layout.group_view(representative)? == view
            {
                matching = Some(class);
                break;
            }
        }
        if let Some(class) = matching {
            classes[class].push(index);
        } else {
            classes.push(vec![index]);
        }
    }
    Ok(classes)
}

/// Admit an existing source profile without deriving a replacement A/B matrix.
fn materialize_source_group_for_open_basis(
    (layout, source_contract): &PrecommittedGroupSeed,
    policy: &PlannerPolicy,
    opening: PlannerOpeningCandidate,
    shared_opening_ring_dimension: usize,
    log_basis_open: u32,
) -> Result<Option<GroupOpenPhaseParams>, AkitaError> {
    let honest_fold_policy = source_contract
        .class()
        .honest_fold_policy(source_contract.decomposition().field_bits());
    let ring_dimension = layout.inner.matrix.ring_dimension();
    opening.validate_for(
        0,
        policy.claim_ext_degree,
        CommitmentRingDims {
            inner: ring_dimension,
            outer: layout.outer.matrix.ring_dimension(),
            opening: shared_opening_ring_dimension,
        },
    )?;
    let num_chunks = policy.chunks_at_level(0);
    let num_fold_coeffs = layout
        .inner
        .matrix
        .input_width()
        .checked_mul(ring_dimension)
        .and_then(|count| count.checked_mul(num_chunks))
        .ok_or_else(|| AkitaError::InvalidSetup("precommitted fold width overflow".into()))?;
    let group_claims = layout.group.num_polynomials();
    let witness_norms = source_contract
        .source_norms(
            layout.inner.digits.log_basis,
            layout.inner.digits.num_digits,
            ring_dimension,
            akita_error::checked::pow2(layout.group.num_vars())
                .and_then(|len| len.checked_mul(group_claims))
                .ok_or_else(|| AkitaError::InvalidSetup("source length overflow".into()))?,
        )?
        .fold_witness;
    let num_digits_fold = if opening.method().is_full_ring_scalar_extension() {
        let bound = (group_claims as u128)
            .checked_mul(layout.blocks.live_blocks as u128)
            .and_then(|n| n.checked_mul(56))
            .and_then(|n| n.checked_mul(witness_norms.infinity_norm()))
            .ok_or_else(|| {
                AkitaError::InvalidSetup("full-ring precommit response bound overflow".into())
            })?;
        akita_params::full_ring_response_depth(bound)?
    } else {
        honest_fold_policy.num_digits_fold(HonestFoldSizingQuery {
            ring_dimension,
            challenge_dimension: opening.challenge_dimension(ring_dimension),
            num_claims: group_claims,
            num_live_ring_elements_per_claim: layout.blocks.live_ring_elements_per_claim,
            num_positions_per_block: layout.blocks.positions_per_block,
            num_live_blocks: layout.blocks.live_blocks,
            num_chunks,
            num_fold_coeffs,
            witness_norms,
            log_basis_response: log_basis_open,
            challenge_config: &opening.challenge_config(),
        })?
    };
    let Some(required_a_bound) = rounded_up_role_a_inf_norm(
        policy.sis_security_policy,
        policy.sis_table_digest,
        policy.sis_modulus_profile,
        ring_dimension,
        log_basis_open,
        &opening.challenge_config(),
        num_digits_fold,
        num_chunks,
    ) else {
        return Ok(None);
    };
    let declared_a_bound =
        layout.inner.matrix.coeff_linf_bound().ok_or_else(|| {
            AkitaError::InvalidSetup("precommitted A cannot use an L2 route".into())
        })?;
    let Some(required_b_bound) = rounded_up_collision_inf_norm(
        policy.sis_security_policy,
        policy.sis_modulus_profile,
        SisMatrixRole::Outer,
        layout.outer.matrix.ring_dimension(),
        log_basis_open,
    ) else {
        return Ok(None);
    };
    if required_a_bound > declared_a_bound
        || required_b_bound > layout.outer.matrix.coeff_linf_bound()
    {
        return Ok(None);
    }
    GroupOpenPhaseParams::admit(
        *layout,
        num_digits_fold,
        PrecommittedGroupAdmissionPolicy {
            decomposition: policy.decomposition,
            sis_security_policy: policy.sis_security_policy,
            sis_table_digest: policy.sis_table_digest,
            sis_modulus_profile: policy.sis_modulus_profile,
            num_response_chunks: num_chunks,
        },
        opening.method(),
        opening.challenge_config(),
        log_basis_open,
    )
    .map(Some)
}

/// Finite response depths admitted by the unchanged source A/B capacities.
///
/// The lower endpoint is the producer's honest sizing policy. Larger depths
/// are consumers only: no source digit, matrix, slice or profile is recreated.
pub fn frozen_response_depth_choices(
    profile: &GroupCommitPhaseParams,
    source_contract: CommittedSourceContract,
    policy: &PlannerPolicy,
    challenge_subring_dimension: usize,
    opening_ring_dimension: usize,
    log_basis_open: u32,
) -> Result<Vec<GroupOpenPhaseParams>, AkitaError> {
    let dimensions = CommitmentRingDims {
        inner: profile.inner.matrix.ring_dimension(),
        outer: profile.outer.matrix.ring_dimension(),
        opening: opening_ring_dimension,
    };
    let Some(opening) = PlannerOpeningCandidate::coefficient_packing(
        0, policy.claim_ext_degree, dimensions, challenge_subring_dimension,
    )? else {
        return Ok(Vec::new());
    };
    let Some(first) = materialize_source_group_for_open_basis(
        &(*profile, source_contract), policy, opening, opening_ring_dimension, log_basis_open,
    )? else {
        return Ok(Vec::new());
    };
    let mut choices = Vec::new();
    // Every additional digit strictly increases the collision radius. Once a
    // cell fails, no later depth can fit the same immutable A.
    for depth in first.num_digits_fold()..=policy.decomposition.field_bits() as usize + 1 {
        let Some(required) = rounded_up_role_a_inf_norm(
            policy.sis_security_policy, policy.sis_table_digest, policy.sis_modulus_profile,
            dimensions.d_a(), log_basis_open, &opening.challenge_config(), depth,
            policy.chunks_at_level(0),
        ) else {
            break;
        };
        if required > profile.inner.matrix.coeff_linf_bound().ok_or_else(|| {
            AkitaError::InvalidSetup("frozen response requires a Linf source A".into())
        })? {
            break;
        }
        choices.push(GroupOpenPhaseParams::admit(
            *profile, depth,
            PrecommittedGroupAdmissionPolicy {
                decomposition: policy.decomposition,
                sis_security_policy: policy.sis_security_policy,
                sis_table_digest: policy.sis_table_digest,
                sis_modulus_profile: policy.sis_modulus_profile,
                num_response_chunks: policy.chunks_at_level(0),
            },
            opening.method(), opening.challenge_config(), log_basis_open,
        )?);
    }
    Ok(choices)
}

/// Exact initial frozen capabilities, authenticated by producer class and
/// geometry, never by prefix/final position.
pub(crate) fn frozen_root_carrier(
    profile: &GroupCommitPhaseParams,
    source: CommittedSourceContract,
) -> Result<usize, AkitaError> {
    use akita_params::sis::CommittedSourceClass;
    let native = profile.inner.matrix.ring_dimension() == 128
        && profile.outer.matrix.ring_dimension() == 128;
    match source.class() {
        CommittedSourceClass::UnitOneHot { source_chunk_size: 1 }
            if native && profile.inner.digits.num_digits == 1
                && source.decomposition().log_commit_bound == 1 => Ok(128),
        CommittedSourceClass::OriginalBalancedDigitPlanes
            if native && profile.group == PolynomialGroupLayout::singleton(16)
                && profile.inner.digits.log_basis == 5
                && profile.inner.digits.num_digits == 26
                && profile.blocks.live_ring_elements_per_claim == 512
                && profile.blocks.positions_per_block == 32
                && profile.blocks.live_blocks == 16
                && profile.outer_slice_count == akita_params::CommitmentSliceCount::ONE => Ok(128),
        CommittedSourceClass::BalancedSignedDigit
            if profile.group == PolynomialGroupLayout::singleton(14)
                && profile.inner.matrix.ring_dimension() == 512
                && profile.outer.matrix.ring_dimension() == 64
                && profile.inner.digits.log_basis == 13
                && profile.inner.digits.num_digits == 10
                && profile.outer.digits.log_basis == 3
                && profile.outer.digits.num_digits == 43
                && profile.blocks.positions_per_block == 4
                && profile.blocks.live_blocks == 8
                && profile.outer_slice_count.get() == 8
                && source.decomposition().log_commit_bound == 128 => Ok(64),
        _ => Err(AkitaError::UnsupportedSchedule(
            "frozen root producer/profile has no registered native128 or decoded Dense64 capability".into(),
        )),
    }
}

pub(crate) fn frozen_deterministic_cap(
    profile: &GroupCommitPhaseParams,
    source: CommittedSourceContract,
    config: &akita_challenges::SparseChallengeConfig,
) -> Result<u128, AkitaError> {
    let logical_len = checked::pow2(profile.group.num_vars())
        .and_then(|n| n.checked_mul(profile.group.num_polynomials()))
        .ok_or_else(|| AkitaError::InvalidInput("frozen logical source length overflow".into()))?;
    let norms = source.source_norms(
        profile.inner.digits.log_basis, profile.inner.digits.num_digits,
        profile.inner.matrix.ring_dimension(), logical_len,
    )?.fold_witness;
    (config.infinity_norm() as u128).checked_mul(norms.l1_norm())
        .zip((config.l1_norm() as u128).checked_mul(norms.infinity_norm()))
        .map(|(a, b)| a.min(b))
        .and_then(|n| n.checked_mul(profile.group.num_polynomials() as u128))
        .and_then(|n| n.checked_mul(profile.blocks.live_blocks as u128))
        .ok_or_else(|| AkitaError::InvalidSetup("frozen deterministic response cap overflow".into()))
}

fn materialize_frozen_source(
    profile: &GroupCommitPhaseParams,
    source: CommittedSourceContract,
    policy: &PlannerPolicy,
    opening: PlannerOpeningCandidate,
    d_d: usize,
    log_basis_open: u32,
) -> Result<Option<GroupOpenPhaseParams>, AkitaError> {
    let carrier = frozen_root_carrier(profile, source)?;
    if opening.challenge_dimension(profile.inner.matrix.ring_dimension()) != carrier {
        return Ok(None);
    }
    let depth = if carrier == 64 { 7 } else { 3 };
    if carrier == 128 {
        let cap = frozen_deterministic_cap(profile, source, &opening.challenge_config())?;
        let (_, positive) = akita_params::sis::checked_balanced_digit_representable_bounds(
            log_basis_open, depth,
        );
        if positive.is_some_and(|reach| cap > reach) {
            return Err(AkitaError::UnsupportedSchedule(format!(
                "all-valid root response gate: deterministic cap {cap} exceeds positive reach {positive:?} at depth {depth}"
            )));
        }
    }
    Ok(frozen_response_depth_choices(
        profile, source, policy, carrier, d_d, log_basis_open,
    )?.into_iter().find(|candidate| candidate.num_digits_fold() == depth))
}

struct MultiGroupRootCandidateCtx<'a> {
    policy: &'a PlannerPolicy,
    dimensions: CommitmentRingDims,
    opening: PlannerOpeningCandidate,
    final_source_contract: CommittedSourceContract,
    final_num_vars: usize,
    main_num_polys: usize,
    source: crate::InnerBasisSource,
}

struct RootFinalGroupCandidateInput<'a> {
    log_basis_inner: u32,
    log_basis_open: u32,
    position_index_bits: usize,
    block_index_bits: usize,
    outer_slice_count: akita_params::CommitmentSliceCount,
    precommitted_groups: &'a [GroupOpenPhaseParams],
    precommitted_d_width: usize,
}

fn precommitted_groups_for_open_basis(
    seeds: &[PrecommittedGroupSeed],
    equivalence_classes: &[Vec<usize>],
    class_openings: &[PlannerOpeningCandidate],
    policy: &PlannerPolicy,
    shared_opening_ring_dimension: usize,
    log_basis_open: u32,
) -> Result<Option<(Vec<GroupOpenPhaseParams>, usize)>, AkitaError> {
    if class_openings.len() != equivalence_classes.len() {
        return Err(AkitaError::InvalidSetup(
            "root precommit opening candidate count mismatch".into(),
        ));
    }
    let mut groups = vec![None; seeds.len()];
    let mut d_width = 0usize;
    for (indices, &opening) in equivalence_classes.iter().zip(class_openings) {
        let Some(materialized) = materialize_source_group_for_open_basis(
            &seeds[indices[0]],
            policy,
            opening,
            shared_opening_ring_dimension,
            log_basis_open,
        )?
        else {
            return Ok(None);
        };
        d_width = checked::mul_add(
            materialized.d_segment_width(policy.claim_ext_degree, shared_opening_ring_dimension)?,
            indices.len(),
            d_width,
        )
        .ok_or_else(|| AkitaError::InvalidSetup("root batch D width overflow".to_string()))?;
        for &index in indices {
            groups[index] = Some(materialized);
        }
    }
    let groups = groups
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| {
            AkitaError::InvalidSetup(
                "precommitted equivalence classes must cover every group".into(),
            )
        })?;
    Ok(Some((groups, d_width)))
}

pub(crate) fn root_batch_next_w_len(
    field_bits: u32,
    extension_degree: usize,
    params: &CommittedGroupParams,
    opening_batch: &OpeningClaimsLayout,
) -> Result<Option<usize>, AkitaError> {
    if !params.compression_sources_supported()? {
        return Ok(None);
    }
    params
        .output_witness_len_for_field_bits(field_bits, extension_degree, opening_batch)
        .map(Some)
}

pub(crate) struct PreparedRootProducers {
    groups: Vec<GroupOpenPhaseParams>,
    d_width: usize,
}

impl PreparedRootProducers {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn prepare(
        key: &ScheduleLookupKey,
        precommitted_source_contracts: &[CommittedSourceContract],
        policy: &PlannerPolicy,
        dimensions: CommitmentRingDims,
        opening: PlannerOpeningCandidate,
        precommitted_class_openings: &[PlannerOpeningCandidate],
        candidate_log_basis_open: u32,
    ) -> Result<Option<Self>, AkitaError> {
        dimensions.validate_role_projection()?;
        opening.validate_for(0, policy.claim_ext_degree, dimensions)?;
        let alpha = dimensions.d_a().trailing_zeros() as usize;
        if key.final_group.num_vars() < alpha
            || (key.final_group.num_vars() == alpha
                && !opening.method().is_full_ring_scalar_extension())
        {
            return Ok(None);
        }
        let equivalence_classes = precommitted_classes_for_key(
            key, precommitted_source_contracts,
        )?;
        if precommitted_class_openings
            .iter()
            .any(|candidate| candidate.is_coefficient_packing() != opening.is_coefficient_packing())
        {
            return Ok(None);
        }
        let shared_opening_ring_dimension = dimensions.d_d();
        if !crate::schedule_params::precommitted_groups_support_opening_dimension(
            key.precommitteds.iter(),
            shared_opening_ring_dimension,
        ) {
            return Ok(None);
        }
        let seeds = key
            .precommitteds
            .iter()
            .copied()
            .zip(precommitted_source_contracts.iter().copied())
            .collect::<Vec<PrecommittedGroupSeed>>();
        let Some((mut groups, d_width)) = precommitted_groups_for_open_basis(
            &seeds,
            &equivalence_classes,
            precommitted_class_openings,
            policy,
            shared_opening_ring_dimension,
            candidate_log_basis_open,
        )?
        else {
            return Ok(None);
        };
        for (index, group) in groups.iter_mut().enumerate() {
            group.opening.source_view = key.source_views[index];
            group.opening.validate_source_view(&group.profile, policy.claim_ext_degree)?;
        }
        Ok(Some(Self { groups, d_width }))
    }

    pub(crate) fn prepare_frozen(
        key: &ScheduleLookupKey,
        sources: &[CommittedSourceContract],
        policy: &PlannerPolicy,
        dimensions: CommitmentRingDims,
        opening: PlannerOpeningCandidate,
        class_openings: &[PlannerOpeningCandidate],
        log_basis_open: u32,
    ) -> Result<Option<Self>, AkitaError> {
        let Some(mut prepared) = Self::prepare(
            key, sources, policy, dimensions, opening, class_openings, log_basis_open,
        )? else {
            return Ok(None);
        };
        let classes = precommitted_classes_for_key(key, sources)?;
        prepared.d_width = 0;
        for (indices, &class_opening) in classes.iter().zip(class_openings) {
            let index = indices[0];
            let Some(mut group) = materialize_frozen_source(
                &key.precommitteds[index], sources[index], policy, class_opening,
                dimensions.d_d(), log_basis_open,
            )? else {
                return Ok(None);
            };
            group.opening.source_view = key.source_views[index];
            group.opening.validate_source_view(&group.profile, policy.claim_ext_degree)?;
            prepared.d_width = checked::mul_add(
                group.d_segment_width(policy.claim_ext_degree, dimensions.d_d())?,
                indices.len(), prepared.d_width,
            ).ok_or_else(|| AkitaError::InvalidSetup("frozen D contribution overflow".into()))?;
            for &index in indices {
                prepared.groups[index] = group;
            }
        }
        Ok(Some(prepared))
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn root_level_candidates_for_prepared_producers(
    key: &ScheduleLookupKey,
    final_source_contract: CommittedSourceContract,
    policy: &PlannerPolicy,
    dimensions: CommitmentRingDims,
    opening: PlannerOpeningCandidate,
    prepared_producers: &PreparedRootProducers,
    candidate_log_basis_inner: u32,
    candidate_log_basis_open: u32,
    guide: Option<crate::schedule_params::CandidateLayoutGuide>,
) -> Result<Vec<(CommittedGroupParams, usize)>, AkitaError> {
    dimensions.validate_role_projection()?;
    opening.validate_for(0, policy.claim_ext_degree, dimensions)?;
    let field_bits = policy.decomposition.field_bits();
    let alpha = dimensions.d_a().trailing_zeros() as usize;
    let reduced_vars = key.final_group.num_vars().saturating_sub(alpha);
    if key.final_group.num_vars() < alpha
        || (reduced_vars == 0 && !opening.method().is_full_ring_scalar_extension())
    {
        return Ok(Vec::new());
    }
    let candidate_ctx = MultiGroupRootCandidateCtx {
        policy,
        dimensions,
        opening,
        final_source_contract,
        final_num_vars: key.final_group.num_vars(),
        main_num_polys: key.final_group.num_polynomials(),
        source: crate::schedule_params::root_inner_basis_source(final_source_contract),
    };
    let opening_batch = key.opening_layout()?;
    let full_ring = opening.method().is_full_ring_scalar_extension();
    let min_block_index_bits: usize = if full_ring || reduced_vars < 3 { 0 } else { 1 };
    let max_block_index_bits: usize = (if full_ring {
        reduced_vars
    } else {
        reduced_vars - 1
    })
    .min(usize::BITS as usize - 1);
    let num_ring_elems = checked::pow2(reduced_vars).ok_or_else(|| {
        AkitaError::InvalidSetup("root reduced-variable domain is too large".into())
    })?;
    let delta_commit = candidate_ctx
        .source
        .num_digits_inner(policy.decomposition, candidate_log_basis_inner)?;
    let delta_open = num_digits_open(DecompositionParams {
        log_basis: candidate_log_basis_open,
        ..policy.decomposition
    });
    let mut split_domain = if let Some(guide) = guide {
        reduced_vars
            .checked_sub(guide.position_index_bits)
            .into_iter()
            .collect()
    } else {
        recursive_split_search_domain(
            policy.recursive_split_search_policy,
            num_ring_elems,
            reduced_vars,
            delta_commit,
            delta_open,
            policy.chunks_at_level(0),
        )
    };
    if full_ring && guide.is_none() {
        split_domain = (0..=reduced_vars).collect();
    }
    if guide.is_none() && min_block_index_bits == 0 {
        split_domain.push(0);
    }
    split_domain.retain(|&split| min_block_index_bits <= split && split <= max_block_index_bits);
    split_domain.sort_unstable_by(|left, right| right.cmp(left));
    split_domain.dedup();

    let mut candidates = Vec::new();
    for block_index_bits in split_domain {
        let position_index_bits = reduced_vars - block_index_bits;
        let num_live_blocks = 1usize << block_index_bits;
        let mut slice_candidates = Vec::new();
        for outer_slice_count in akita_params::CommitmentSliceCount::ALL {
            if opening.method().is_full_ring_scalar_extension()
                && outer_slice_count != akita_params::CommitmentSliceCount::ONE
            {
                continue;
            }
            if guide.is_some_and(|guide| outer_slice_count != guide.outer_slice_count) {
                continue;
            }
            if outer_slice_count
                .validate_for_commitment(
                    0,
                    akita_params::CommitmentPayloadMode::Compressed,
                    num_live_blocks,
                )
                .is_err()
            {
                continue;
            }
            let Some(mut candidate_params) = root_final_group_level_params_candidate(
                &candidate_ctx,
                RootFinalGroupCandidateInput {
                    log_basis_inner: candidate_log_basis_inner,
                    log_basis_open: candidate_log_basis_open,
                    position_index_bits,
                    block_index_bits,
                    outer_slice_count,
                    precommitted_groups: &prepared_producers.groups,
                    precommitted_d_width: prepared_producers.d_width,
                },
            )?
            else {
                continue;
            };
            candidate_params.witness_chunk = crate::policy::witness_chunk_at_level(policy, 0);
            if !candidate_params.compression_sources_supported()? {
                continue;
            }
            if opening.method().is_full_ring_scalar_extension()
                && candidate_params
                    .validate_full_ring_execution(policy.claim_ext_degree)
                    .is_err()
            {
                continue;
            }
            slice_candidates.push(candidate_params);
        }
        for candidate_params in crate::schedule_params::prune_locally_unprofitable_slices(
            policy,
            &opening_batch,
            slice_candidates,
        )? {
            let Some(output_witness_len) = root_batch_next_w_len(
                field_bits,
                policy.claim_ext_degree,
                &candidate_params,
                &opening_batch,
            )?
            else {
                continue;
            };
            candidates.push((candidate_params, output_witness_len));
        }
    }

    Ok(candidates)
}

/// Frozen A/B admission, followed only by consumer-owned D construction.
#[allow(clippy::too_many_arguments)]
pub(crate) fn root_candidate_for_frozen_profile(
    profile: &GroupCommitPhaseParams,
    source_contract: CommittedSourceContract,
    policy: &PlannerPolicy,
    dimensions: CommitmentRingDims,
    opening: PlannerOpeningCandidate,
    producers: &PreparedRootProducers,
    log_basis_open: u32,
    opening_batch: &OpeningClaimsLayout,
) -> Result<Option<(CommittedGroupParams, usize)>, AkitaError> {
    if profile.inner.matrix.ring_dimension() != dimensions.d_a()
        || profile.outer.matrix.ring_dimension() != dimensions.d_b()
        || log_basis_open < profile.outer.digits.log_basis
    {
        return Ok(None);
    }
    let Some(mut main) = materialize_frozen_source(
        profile,
        source_contract,
        policy,
        opening,
        dimensions.d_d(),
        log_basis_open,
    )?
    else {
        return Ok(None);
    };
    main.opening.source_view = opening_batch.group_view(opening_batch.root_final_group_index()?)?;
    main.opening.validate_source_view(&main.profile, policy.claim_ext_degree)?;
    let main_width = main.d_segment_width(policy.claim_ext_degree, dimensions.d_d())?;
    let Some((open_key, main_width)) = projected_collision_role_price(
        policy,
        SisMatrixRole::Open,
        dimensions.d_d(),
        dimensions.d_d(),
        main_width,
        log_basis_open,
    ) else {
        return Ok(None);
    };
    let width = main_width
        .checked_add(producers.d_width)
        .ok_or_else(|| AkitaError::InvalidSetup("frozen root batch D width overflow".into()))?;
    let Ok(open_matrix) = OpenCommitMatrixParams::try_new_with_min_rank(open_key, width) else {
        return Ok(None);
    };
    let groups = producers
        .groups
        .iter()
        .copied()
        .chain(std::iter::once(main))
        .collect();
    let params = CommittedGroupParams::try_new(
        groups,
        open_matrix,
        akita_params::CommitmentPayloadMode::Compressed,
        akita_params::RingRelationMode::QuotientLift,
        akita_params::CommittedSourceEncoding::CanonicalCoefficientTable,
        akita_params::ChunkedWitnessCfg::default(),
    )?;
    let Some(next_len) = root_batch_next_w_len(
        policy.decomposition.field_bits(),
        policy.claim_ext_degree,
        &params,
        opening_batch,
    )?
    else {
        return Ok(None);
    };
    Ok(Some((params, next_len)))
}

fn root_final_group_level_params_candidate(
    ctx: &MultiGroupRootCandidateCtx<'_>,
    input: RootFinalGroupCandidateInput<'_>,
) -> Result<Option<CommittedGroupParams>, AkitaError> {
    let RootFinalGroupCandidateInput {
        log_basis_inner,
        log_basis_open,
        position_index_bits,
        block_index_bits,
        outer_slice_count,
        precommitted_groups,
        precommitted_d_width,
    } = input;
    let policy = ctx.policy;
    let dimensions = ctx.dimensions;
    let d_a = dimensions.d_a();
    let decomp = ctx.policy.decomposition;
    let level_decomp = DecompositionParams {
        log_basis: log_basis_open,
        ..decomp
    };
    let num_digits_inner = ctx
        .source
        .num_digits_inner(ctx.policy.decomposition, log_basis_inner)?;
    let num_digits_outer = num_digits_open(level_decomp);
    let num_digits_open = num_digits_outer;
    let Some(num_live_blocks) = checked::pow2(block_index_bits) else {
        return Ok(None);
    };
    let Some(num_positions_per_block) = checked::pow2(position_index_bits) else {
        return Ok(None);
    };
    let Some(num_live_ring_elements_per_claim) =
        num_live_blocks.checked_mul(num_positions_per_block)
    else {
        return Ok(None);
    };
    let Some(width_s) = decomposed_s_block_ring_count(num_positions_per_block, num_digits_inner)
    else {
        return Ok(None);
    };
    let num_chunks = policy.chunks_at_level(0);
    let witness_norms = ctx
        .final_source_contract
        .source_norms(
            log_basis_inner,
            num_digits_inner,
            d_a,
            akita_error::checked::pow2(ctx.final_num_vars)
                .and_then(|len| len.checked_mul(ctx.main_num_polys))
                .ok_or_else(|| AkitaError::InvalidSetup("source length overflow".into()))?,
        )?
        .fold_witness;
    let Some(ab_candidate) = derive_ab_commitment_candidate(AbCommitmentCandidateRequest {
        policy,
        fold_policy: &ctx
            .final_source_contract
            .class()
            .honest_fold_policy(decomp.field_bits()),
        ring_challenge_cfg: &ctx.opening.challenge_config(),
        challenge_dimension: ctx.opening.challenge_dimension(d_a),
        dimensions,
        payload_mode: akita_params::CommitmentPayloadMode::Compressed,
        num_claims: ctx.main_num_polys,
        num_live_ring_elements_per_claim,
        num_live_blocks,
        num_positions_per_block,
        num_chunks,
        outer_slice_count,
        witness_norms,
        log_basis_open,
        width_s,
        num_digits_outer,
        modeled_linf_cap: None,
    })?
    else {
        return Ok(None);
    };
    let num_digits_fold = ab_candidate.num_digits_fold;
    let inner_commit_matrix = ab_candidate.inner_commit_matrix;
    let outer_commit_matrix = ab_candidate.outer_commit_matrix;

    let Ok(main_d_width) = akita_params::opening_d_segment_width(
        ctx.opening.method(),
        policy.claim_ext_degree,
        d_a,
        dimensions.d_d(),
        num_digits_open,
        num_live_blocks,
        ctx.main_num_polys,
    ) else {
        return Ok(None);
    };
    let Some((open_key, main_d_width)) = projected_collision_role_price(
        policy,
        akita_params::SisMatrixRole::Open,
        dimensions.d_d(),
        dimensions.d_d(),
        main_d_width,
        log_basis_open,
    ) else {
        return Ok(None);
    };
    // Every group width is already expressed in shared D-native subcolumns.
    // The collision-role pricing above only selects the D key and bound.
    let d_width = main_d_width
        .checked_add(precommitted_d_width)
        .ok_or_else(|| AkitaError::InvalidSetup("root batch D width overflow".to_string()))?;
    let Ok(open_commit_matrix) = OpenCommitMatrixParams::try_new_with_min_rank(open_key, d_width)
    else {
        return Ok(None);
    };

    let groups = precommitted_groups
        .iter()
        .copied()
        .chain(std::iter::once(akita_params::GroupOpenPhaseParams {
            profile: akita_params::GroupCommitPhaseParams {
                version: akita_params::GroupCommitPhaseParams::VERSION,
                group: akita_params::PolynomialGroupLayout::new(
                    ctx.final_num_vars,
                    ctx.main_num_polys,
                ),
                blocks: akita_params::BlockGeometry::new(
                    num_live_ring_elements_per_claim,
                    num_positions_per_block,
                    num_live_blocks,
                ),
                outer_slice_count,
                inner: akita_params::RoleParams::new(
                    akita_params::GadgetDigits::new(log_basis_inner, num_digits_inner),
                    inner_commit_matrix,
                ),
                outer: akita_params::RoleParams::new(
                    akita_params::GadgetDigits::new(log_basis_open, num_digits_outer),
                    outer_commit_matrix,
                ),
            },
            opening: akita_params::GroupOpeningPlan {
                source_view: akita_params::SourceOpeningView::Decoded,
                opening_method: ctx.opening.method(),
                fold_challenge_config: ctx.opening.challenge_config(),
                log_basis_open,
                num_digits_open,
                num_digits_fold,
            },
            setup_natural_len: None,
        }))
        .collect();
    let params = CommittedGroupParams::try_new(
        groups,
        open_commit_matrix,
        akita_params::CommitmentPayloadMode::Compressed,
        if ctx.opening.method().is_full_ring_scalar_extension() {
            akita_params::RingRelationMode::ExactCoefficientRows
        } else {
            akita_params::RingRelationMode::QuotientLift
        },
        akita_params::CommittedSourceEncoding::for_producer(
            ctx.opening.method(),
            policy.claim_ext_degree,
            d_a,
            ctx.final_num_vars,
            true,
        ),
        // Root folds use the ordinary single-chunk precommit path before the
        // schedule-level chunk policy is applied.
        akita_params::ChunkedWitnessCfg::default(),
    )?;

    Ok(Some(params))
}

/// Build the fold schedule selected by a full schedule lookup key.
///
/// `precommitted_source_contracts` must contain the actual producer declaration
/// for each frozen precommit, in lookup-key order. Frozen commit geometry does
/// not authenticate source class or bound by itself: callers must obtain these
/// contracts from the configurations that produced the commitments. The
/// planner uses them only to size the offline honest-response model; the
/// verifier continues to enforce the response caps frozen into the schedule.
pub fn find_schedule(
    key: &ScheduleLookupKey,
    final_source_contract: CommittedSourceContract,
    precommitted_source_contracts: &[CommittedSourceContract],
    policy: &PlannerPolicy,
    ring_challenge_config: impl Fn(usize) -> Result<akita_challenges::SparseChallengeConfig, AkitaError>,
) -> Result<PlannedFoldSchedule, AkitaError> {
    find_schedule_in_relation_order(
        key,
        final_source_contract,
        precommitted_source_contracts,
        policy,
        ring_challenge_config,
        ScheduleSearchOptions::canonical(),
    )
}

/// Plan one complete row over exact ordered originals and authenticated views.
///
/// A/B/F, source digits, blocks and slicing are immutable. Initial VM sources
/// use native128/base32 and all-valid response depth3; decoded Dense originals
/// use their actual carrier64/h8/base8/depth7 recipe. These root capabilities
/// never restrict the freshly generated full shrinking suffix or terminal.
///
/// The finite domain is the validated policy's dimension/basis/split/setup
/// domain and the existing recursion cap. As in ordinary search, the root
/// opening basis is the configured minimum, not the entire opening range.
/// There is no scalar guide, geometry rewrite, or unconstrained fallback.
///
/// Contracts must come from the actual producers: profile equality does not
/// authenticate physical source identity. Malformed/incompatible inputs return
/// `InvalidInput`/`InvalidSetup`; exhausted capacity or policy domains return
/// `UnsupportedSchedule`. The result passes ordinary row admission, but must
/// still enter a separately trusted catalog before runtime use.
pub fn find_schedule_for_frozen_profiles(
    profiles: &akita_params::CommittedGroupBatchProfile,
    final_source_contract: CommittedSourceContract,
    precommitted_source_contracts: &[CommittedSourceContract],
    source_views: &[akita_params::SourceOpeningView],
    policy: &PlannerPolicy,
    ring_challenge_config: impl Fn(usize) -> Result<akita_challenges::SparseChallengeConfig, AkitaError>,
) -> Result<PlannedFoldSchedule, AkitaError> {
    use akita_params::sis::CommittedSourceClass;

    akita_schedules::planner_support::validate_policy(policy)?;
    profiles.validate(policy.decomposition.field_bits())?;
    let key = ScheduleLookupKey::with_views(
        profiles.final_group.group, profiles.precommitteds.clone(), source_views.to_vec(),
    )?;
    key.validate(policy.decomposition.field_bits())?;
    for ((profile, source), &view) in profiles.precommitteds.iter()
        .zip(precommitted_source_contracts.iter().copied())
        .chain(std::iter::once((&profiles.final_group, final_source_contract)))
        .zip(source_views)
    {
        view.validate_producer_contract(source)?;
        frozen_root_carrier(profile, source)?;
    }
    if policy.opening_family != akita_schedules::PlannerOpeningFamily::Legacy
        || policy.claim_ext_degree != 1
        || policy.chal_ext_degree != 1
        || policy.sis_modulus_profile != akita_params::SisModulusProfileId::Q128OffsetA7F7
        || policy.sis_table_digest != akita_params::SisTableDigest::CURRENT
        || policy.witness_chunk != akita_params::ChunkedWitnessCfg::default()
    {
        return Err(AkitaError::InvalidSetup(
            "frozen original-source planning requires ordinary K/K, Q128OffsetA7F7, current SIS tables and single chunks".into(),
        ));
    }
    if profiles.precommitteds.len() != precommitted_source_contracts.len()
        || final_source_contract.decomposition().field_bits() != 128
        || final_source_contract.decomposition().log_commit_bound
            != policy.decomposition.log_commit_bound
        || precommitted_source_contracts.iter().any(|source| {
            source.decomposition().field_bits() != 128
        })
    {
        return Err(AkitaError::InvalidInput(
            "frozen planning requires one authentic 128-bit producer contract per ordered original and a final bound matching policy".into(),
        ));
    }
    // The final VM source is the designated last Main. Decoded Dense batches
    // designate their last original instead. Prefixes are not RAM-only.
    if !matches!(final_source_contract.class(),
        CommittedSourceClass::UnitOneHot { source_chunk_size: 1 }
            | CommittedSourceClass::BalancedSignedDigit)
    {
        return Err(AkitaError::InvalidInput(
            "frozen final source must be the designated Main or decoded Dense original".into(),
        ));
    }
    let dimension_domain = crate::schedule_params::dimension_candidates(
        policy,
        0,
        crate::schedule_params::initial_dimension_ceiling(policy)?,
    )?;
    let mut outside_domain = false;
    for (profile, source) in std::iter::once((&profiles.final_group, final_source_contract)).chain(
        profiles
            .precommitteds
            .iter()
            .zip(precommitted_source_contracts.iter().copied()),
    ) {
        let a_key = profile.inner.matrix.sis_table_key().ok_or_else(|| {
            AkitaError::InvalidSetup("frozen original sources require a Linf A certificate".into())
        })?;
        for key in [a_key, profile.outer.matrix.sis_table_key()] {
            if key.policy != policy.sis_security_policy
                || key.table_digest != policy.sis_table_digest
                || key.modulus_profile != policy.sis_modulus_profile
            {
                return Err(AkitaError::InvalidSetup(
                    "frozen A/B certificate does not match the consuming SIS policy".into(),
                ));
            }
        }
        source.source_norms(
            profile.inner.digits.log_basis,
            profile.inner.digits.num_digits,
            profile.inner.matrix.ring_dimension(),
            checked::pow2(profile.group.num_vars())
                .and_then(|len| len.checked_mul(profile.group.num_polynomials()))
                .ok_or_else(|| AkitaError::InvalidInput("frozen source length overflow".into()))?,
        )?;
        if profile.outer.digits.num_digits
            != num_digits_open(DecompositionParams {
                log_basis: profile.outer.digits.log_basis,
                ..policy.decomposition
            })
        {
            return Err(AkitaError::InvalidSetup(
                "frozen B depth does not match its source-owned basis".into(),
            ));
        }
        let required_b = rounded_up_collision_inf_norm(
            policy.sis_security_policy,
            policy.sis_modulus_profile,
            SisMatrixRole::Outer,
            profile.outer.matrix.ring_dimension(),
            profile.outer.digits.log_basis,
        )
        .ok_or_else(|| AkitaError::InvalidSetup("frozen B has no certified norm".into()))?;
        if profile.outer.matrix.coeff_linf_bound() < required_b {
            return Err(AkitaError::InvalidSetup(
                "frozen B cannot cover its own basis".into(),
            ));
        }
        outside_domain |= !dimension_domain.iter().any(|dims| {
            dims.d_a() == profile.inner.matrix.ring_dimension()
                && dims.d_b() == profile.outer.matrix.ring_dimension()
        }) || profile.outer.digits.log_basis > policy.opening_basis_range.0;
        let dimension = frozen_root_carrier(profile, source)?;
        let production =
            akita_challenges::SparseChallengeConfig::production_for_ring_dim(dimension)
                .ok_or_else(|| {
                    AkitaError::InvalidSetup("frozen A has no production shell".into())
                })?;
        if ring_challenge_config(dimension)? != production {
            return Err(AkitaError::InvalidSetup(
                "frozen native root requires the production challenge shell".into(),
            ));
        }
        let dims = CommitmentRingDims {
            inner: profile.inner.matrix.ring_dimension(),
            outer: profile.outer.matrix.ring_dimension(),
            opening: dimension,
        };
        let opening = PlannerOpeningCandidate::coefficient_packing(0, 1, dims, dimension)?
            .ok_or_else(|| AkitaError::UnsupportedSchedule("frozen carrier projection gate".into()))?;
        if materialize_frozen_source(profile, source, policy, opening, dimension,
            policy.opening_basis_range.0)?.is_none()
        {
            return Err(AkitaError::UnsupportedSchedule(format!(
                "frozen response capacity gate: original {profile:?} cannot admit carrier {dimension}, base {}, required response depth {} without changing A/B",
                policy.opening_basis_range.0, if dimension == 64 { 7 } else { 3 },
            )));
        }
    }
    let inner_basis = profiles.final_group.inner.digits.log_basis;
    if final_source_contract.class() != CommittedSourceClass::BalancedSignedDigit {
        outside_domain |=
            !(policy.inner_basis_range.0..=policy.inner_basis_range.1).contains(&inner_basis);
    }
    if outside_domain {
        return Err(AkitaError::UnsupportedSchedule(format!(
            "frozen profiles {profiles:?} are outside the root dimension/basis domain of {policy:?}"
        )));
    }
    let planned = find_schedule_in_relation_order(
        &key,
        final_source_contract,
        precommitted_source_contracts,
        policy,
        ring_challenge_config,
        ScheduleSearchOptions {
            root_final_profile: Some(&profiles.final_group),
            ..ScheduleSearchOptions::canonical()
        },
    )?;
    // Admission checks the exact ordered input profiles, not a reconstructed
    // profile derived from whichever candidate the search happened to select.
    ResolvedScheduleRow::try_new(profiles.clone(), planned.schedule.clone(), policy)?
        .validate_opening_layout(&key.opening_layout()?)?;
    Ok(planned)
}

/// Adapt a validated scalar main row to an exact grouped-root key.
///
/// The main row's root A/B geometry and opening plan are frozen. Its recursive
/// depth, per-level dimensions, block split, slicing, bases, opening family,
/// payload/relation phases, and direct-vs-offloaded edges form a guide. The
/// grouped root's shared D matrix and every length-, setup-, rank-, response-,
/// and relation-derived suffix value are rebuilt under `policy`. This guided
/// search is a conditional search, not the globally optimal search performed
/// by [`find_schedule`].
///
/// When that frozen skeleton is no longer feasible, the guided search reports
/// [`AkitaError::UnsupportedSchedule`] and adaptation falls back to
/// [`find_schedule`] for the same key. Adaptation therefore fails only when the
/// full search fails too, or when the request itself is invalid. Invalid
/// requests (no precommitted group, a mismatched final group, or a main row
/// that does not pass audit under `policy`) are rejected without a fallback.
/// A fallback row is the full search's optimum for the key and need not retain
/// any part of the main row's skeleton. When both searches fail, the error
/// carries both reasons.
///
/// The supplied row is expected to come from a caller-approved
/// [`akita_schedules::ValidatedScheduleCatalog`]. The resulting expanded row must
/// still be admitted into the caller's final trusted catalog before runtime
/// proving or verification.
pub fn find_adapted_schedule(
    main_row: &ResolvedScheduleRow,
    request: &crate::emit::GroupedGenerationRequest,
    final_source_contract: CommittedSourceContract,
    policy: &PlannerPolicy,
    ring_challenge_config: impl Fn(usize) -> Result<akita_challenges::SparseChallengeConfig, AkitaError>,
) -> Result<PlannedFoldSchedule, AkitaError> {
    let key = request.key();
    let precommitted_source_contracts = request.source_contracts();
    match find_adapted_schedule_for_key(
        main_row,
        &key,
        final_source_contract,
        &precommitted_source_contracts,
        policy,
        &ring_challenge_config,
    ) {
        Err(AkitaError::UnsupportedSchedule(guided)) => find_schedule(
            &key,
            final_source_contract,
            &precommitted_source_contracts,
            policy,
            ring_challenge_config,
        )
        .map_err(|error| match error {
            AkitaError::UnsupportedSchedule(full) => AkitaError::UnsupportedSchedule(format!(
                "{full}; the guided search was also infeasible: {guided}"
            )),
            error => error,
        }),
        adapted => adapted,
    }
}

fn find_adapted_schedule_for_key(
    main_row: &ResolvedScheduleRow,
    key: &ScheduleLookupKey,
    final_source_contract: CommittedSourceContract,
    precommitted_source_contracts: &[CommittedSourceContract],
    policy: &PlannerPolicy,
    ring_challenge_config: impl Fn(usize) -> Result<akita_challenges::SparseChallengeConfig, AkitaError>,
) -> Result<PlannedFoldSchedule, AkitaError> {
    akita_schedules::planner_support::validate_policy(policy)?;
    key.validate(policy.decomposition.field_bits())?;
    if key.precommitteds.is_empty() {
        return Err(AkitaError::InvalidInput(
            "adapted planning requires at least one precommitted group".into(),
        ));
    }
    if key.precommitteds.len() != precommitted_source_contracts.len() {
        return Err(AkitaError::InvalidInput(
            "adapted planning requires one source contract per precommitted group".into(),
        ));
    }
    if !main_row.profiles().precommitteds.is_empty()
        || !main_row
            .schedule()
            .root
            .params
            .precommitted_groups()
            .is_empty()
    {
        return Err(AkitaError::InvalidInput(
            "adapted planning requires a scalar main-group-only row".into(),
        ));
    }
    if key.final_group != main_row.profiles().final_group.group {
        return Err(AkitaError::InvalidInput(
            "adapted final-group layout does not match the main row".into(),
        ));
    }
    // Re-audit the baseline under the caller's current policy. Adaptation never
    // treats a row produced for another policy epoch as a geometry hint.
    ResolvedScheduleRow::try_new(
        main_row.profiles().clone(),
        main_row.schedule().clone(),
        policy,
    )?;

    find_schedule_in_relation_order(
        key,
        final_source_contract,
        precommitted_source_contracts,
        policy,
        ring_challenge_config,
        ScheduleSearchOptions {
            root_main_constraint: Some(&main_row.schedule().root.params),
            adaptation_guide: Some(main_row.schedule()),
            ..ScheduleSearchOptions::canonical()
        },
    )
}

/// Build a schedule under a test-only relation-mode restriction.
#[cfg(feature = "test-support")]
pub fn find_schedule_for_test_relation_mode(
    key: &ScheduleLookupKey,
    final_source_contract: CommittedSourceContract,
    precommitted_source_contracts: &[CommittedSourceContract],
    policy: &PlannerPolicy,
    ring_challenge_config: impl Fn(usize) -> Result<akita_challenges::SparseChallengeConfig, AkitaError>,
    relation_mode_filter: super::schedule_params::TestRelationModeFilter,
) -> Result<PlannedFoldSchedule, AkitaError> {
    find_schedule_in_relation_order(
        key,
        final_source_contract,
        precommitted_source_contracts,
        policy,
        ring_challenge_config,
        ScheduleSearchOptions {
            relation_mode_filter: relation_mode_filter.into(),
            ..ScheduleSearchOptions::canonical()
        },
    )
}

/// Canonical schedule search with an internal traversal-order seam used to
/// prove that candidate enumeration does not affect selection.
pub(crate) fn find_schedule_in_relation_order(
    key: &ScheduleLookupKey,
    final_source_contract: CommittedSourceContract,
    precommitted_source_contracts: &[CommittedSourceContract],
    policy: &PlannerPolicy,
    ring_challenge_config: impl Fn(usize) -> Result<akita_challenges::SparseChallengeConfig, AkitaError>,
    options: ScheduleSearchOptions<'_>,
) -> Result<PlannedFoldSchedule, AkitaError> {
    let diagnostics = crate::diagnostics::active();
    let diagnostics = diagnostics.as_deref();
    akita_schedules::planner_support::validate_policy(policy)?;
    key.validate(policy.decomposition.field_bits())?;
    if options.root_final_profile.is_none()
        && key.source_views.iter().any(|view| {
            *view != akita_params::SourceOpeningView::Decoded
        })
    {
        return Err(AkitaError::InvalidInput(
            "original RAM views require exact frozen-profile planning; no decoded/adapted fallback".into(),
        ));
    }
    if options.root_final_profile.is_some()
        && (options.root_main_constraint.is_some() || options.adaptation_guide.is_some())
    {
        return Err(AkitaError::InvalidInput(
            "frozen source planning cannot also use a scalar main constraint or adaptation guide"
                .into(),
        ));
    }
    if final_source_contract.decomposition().field_bits() != policy.decomposition.field_bits()
        || final_source_contract.decomposition().log_commit_bound
            != policy.decomposition.log_commit_bound
    {
        return Err(AkitaError::InvalidInput(
            "final source contract field and bound must match the planner policy".into(),
        ));
    }
    if key.precommitteds.len() != precommitted_source_contracts.len() {
        return Err(AkitaError::InvalidInput(
            "schedule planning requires one source contract per precommitted group".into(),
        ));
    }
    if precommitted_source_contracts
        .iter()
        .any(|contract| contract.decomposition().field_bits() != policy.decomposition.field_bits())
    {
        return Err(AkitaError::InvalidInput(
            "precommitted source contract field must match the consuming planner field".into(),
        ));
    }
    match policy.opening_family {
        akita_schedules::PlannerOpeningFamily::FullRingE2OuterDirect => {
            return find_full_ring_direct_schedule(
                key,
                final_source_contract,
                precommitted_source_contracts,
                policy,
            )
        }
        akita_schedules::PlannerOpeningFamily::FullRingE2InnerRecursive => {
            return full_ring_inner::find_schedule(
                key,
                final_source_contract,
                precommitted_source_contracts,
                policy,
            )
        }
        akita_schedules::PlannerOpeningFamily::Legacy => {}
    }
    let ring_challenge_config: RingChallengeConfigFn<'_> = &ring_challenge_config;
    let scalar_policy;
    let active_policy = if key.precommitteds.is_empty() && !policy.recursive_setup_planning {
        // Ordinary scalar families use the direct objective. Recursive
        // companion families retain their setup-aware objective so a scalar
        // root may carry its setup opening into the first suffix fold.
        scalar_policy = crate::policy::direct_only_policy(*policy);
        &scalar_policy
    } else {
        policy
    };
    let setup_field_budget = if matches!(
        active_policy.selection_policy,
        crate::SelectionPolicyId::MinFirstDirectSetupThenExactProofAndWorkV5
            | crate::SelectionPolicyId::MinPaddedSetupEnvelopeThenFirstDirectThenExactProofAndWorkV6
    ) {
        active_policy.setup_field_budget
    } else {
        None
    };
    let root_input_witness_len = checked::pow2(key.final_group.num_vars()).ok_or_else(|| {
        AkitaError::InvalidSetup("multi-group root-fold witness length overflow".to_string())
    })?;
    let suffix_ctx = SuffixCtx {
        policy: active_policy,
        challenge_order: active_policy.transcript_grinding_order()?,
        diagnostics,
        ring_challenge_config,
        key: PolynomialGroupLayout::singleton(key.final_group.num_vars()),
        setup_field_budget,
        root_lookup_key: Some(key),
        root_main_constraint: options.root_main_constraint,
        root_final_profile: options.root_final_profile,
        adaptation_guide: options.adaptation_guide,
        root_source_contract: Some(final_source_contract),
        precommitted_source_contracts,
        level_zero_is_root: true,
        relation_traversal_order: options.relation_traversal_order,
        relation_mode_filter: options.relation_mode_filter,
    };
    let dimension_ceiling = super::schedule_params::initial_dimension_ceiling(active_policy)?;
    let initial_state = SuffixState {
        level: 0,
        current_witness_len: root_input_witness_len,
        current_lb: 0,
        source_moment: None,
        dimension_ceiling,
        topology: super::schedule_params::SuffixTopology::Direct {
            payload_phase: akita_params::CommitmentPayloadPhase::CompressedPrefix,
            relation_phase: super::schedule_params::RingRelationPhase::QuotientPrefix,
        },
    };
    let mut memo = ScheduleMemo::new();
    let suffix_started = diagnostics.map(|_| Instant::now());
    #[cfg(test)]
    let query_prefix_count = options.query_prefix_count;
    #[cfg(not(test))]
    let query_prefix_count = 0;
    let suffix = derive_selected_suffix_schedule(
        &suffix_ctx,
        &mut memo,
        initial_state,
        0,
        super::schedule_params::QuerySearch::Root(query_prefix_count),
    );
    if let (Some(diagnostics), Some(started)) = (diagnostics, suffix_started) {
        diagnostics.add_suffix_dp_time(started.elapsed());
        let (hits, misses) = memo.setup_prefix_cache_diagnostics();
        diagnostics.record_setup_prefix_cache(hits, misses);
    }
    let suffix = suffix?;
    let best = match active_policy.selection_policy {
        crate::SelectionPolicyId::MinEstimatedExactProofAndWorkV5 => {
            select_complete_candidate(active_policy, suffix.payload_candidates(), diagnostics)?
        }
        crate::SelectionPolicyId::MinFirstDirectSetupThenExactProofAndWorkV5 => {
            select_complete_candidate(active_policy, suffix.setup_candidates(), diagnostics)?
        }
        crate::SelectionPolicyId::MinPaddedSetupEnvelopeThenFirstDirectThenExactProofAndWorkV6 => {
            select_complete_candidate(active_policy, suffix.setup_candidates(), diagnostics)?
        }
    };

    let Some(best) = best.cloned() else {
        if let Some(profile) = options.root_final_profile {
            return Err(AkitaError::UnsupportedSchedule(format!(
                "finite frozen root-output/shrinking-suffix/actual-terminal closure gate exhausted: \
                 final profile {profile:?}, ordered prefix profiles {:?}, ordered views {:?}, \
                 policy domain {active_policy:?}; no adaptation or decoded fallback attempted",
                key.precommitteds, key.source_views
            )));
        }
        if key.precommitteds.is_empty()
            && matches!(
                active_policy.ring_dimension_schedule_mode,
                crate::RingDimensionScheduleMode::AdaptiveDimension { .. }
            )
        {
            return Err(AkitaError::UnsupportedSchedule(format!(
                "no mixed-D schedule in the audited fold domain for num_vars={}, num_polynomials={}",
                key.final_group.num_vars(),
                key.final_group.num_polynomials()
            )));
        }
        let class_count = crate::planner::precommitted_classes_for_key(key, precommitted_source_contracts)?
        .len();
        let bounded_note = if u32::try_from(class_count)
            .ok()
            .and_then(|classes| {
                akita_challenges::PRODUCTION_FOLD_CHALLENGE_RING_DIMS
                    .len()
                    .checked_pow(classes)
            })
            .is_none_or(|products| {
                products > crate::schedule_params::MAX_PRECOMMIT_OPENING_PRODUCTS
            }) {
            format!(
                "; {class_count} distinct precommitted classes can exceed the {}-assignment \
                 root opening bound, so some coefficient-packing root openings may not have \
                 been searched",
                crate::schedule_params::MAX_PRECOMMIT_OPENING_PRODUCTS
            )
        } else {
            String::new()
        };
        return Err(AkitaError::UnsupportedSchedule(format!(
            "no multi-group schedule in the audited fold domain for num_vars={}{bounded_note}",
            key.final_group.num_vars()
        )));
    };
    let first_direct_setup_field_len = if matches!(
        active_policy.selection_policy,
        crate::SelectionPolicyId::MinFirstDirectSetupThenExactProofAndWorkV5
            | crate::SelectionPolicyId::MinPaddedSetupEnvelopeThenFirstDirectThenExactProofAndWorkV6
    ) {
        Some(
            best.first_direct_setup_field_len
                .ok_or_else(|| {
                    AkitaError::InvalidSetup(
                        "setup-first schedule is missing its first direct setup size".into(),
                    )
                })?
                .get(),
        )
    } else {
        None
    };
    if let Some(diagnostics) = diagnostics {
        let metrics = best.metrics();
        let folds = best.folds.to_vec();
        let root_output_witness_len = folds
            .first()
            .ok_or_else(|| {
                AkitaError::InvalidSetup("selected schedule is missing its root fold".into())
            })?
            .output_witness_len;
        diagnostics.record_selected(
            active_policy.selection_policy,
            metrics,
            root_output_witness_len,
            folds
                .iter()
                .map(|fold| crate::diagnostics::SelectedFoldDiagnostics {
                    dimensions: fold.params.role_dims(),
                    relation_mode: fold.params.ring_relation_mode,
                })
                .collect(),
        );
    }
    let materialization_started = diagnostics.map(|_| Instant::now());
    let root_layout = key.opening_layout()?;
    let planned = materialize_candidate_schedule(
        CandidateMaterializationCost {
            proof_bytes: best.cost.proof_bytes(),
            grinding: best.cost.grinding_cost(),
            num_setup_field_elements: best.setup_field_elements,
            first_direct_setup_field_len,
        },
        active_policy,
        &root_layout,
        best.folds.to_vec(),
        best.terminal.as_ref().clone(),
    );
    if let (Some(diagnostics), Some(started)) = (diagnostics, materialization_started) {
        diagnostics.add_final_materialization_time(started.elapsed());
    }
    planned
}

#[cfg(all(test, feature = "catalog-gen"))]
#[path = "test/planner_totality.rs"]
mod totality_tests;

/// Finite ordinary-outer selection. Reuses original producer admission and
/// native Eq109 sizing, but enumerates actual A-only terminals: constructing
/// fictitious terminal B/D/F/H matrices would reject valid direct closures.
fn find_full_ring_direct_schedule(
    key: &ScheduleLookupKey,
    source: CommittedSourceContract,
    precommitted_sources: &[CommittedSourceContract],
    policy: &PlannerPolicy,
) -> Result<PlannedFoldSchedule, AkitaError> {
    use akita_params::{
        BlockGeometry, CommittedGroupBatchProfile, FoldParams, FoldSchedule, FoldScheduleEstimate,
        GadgetDigits, InnerRoleParams, OpeningMethod, TailSegmentLayout, TerminalFoldParams,
        TerminalResponseCodec, TerminalResponseShape, FULL_RING_FOLD_CHALLENGE,
    };
    let dimensions = CommitmentRingDims::uniform(128);
    let opening =
        PlannerOpeningCandidate::full_ring_scalar_extension(policy.claim_ext_degree, dimensions)?;
    let classes = crate::planner::precommitted_classes_for_key(key, precommitted_sources)?;
    let precommit_openings = vec![opening; classes.len()];
    let producers = PreparedRootProducers::prepare(
        key,
        precommitted_sources,
        policy,
        dimensions,
        opening,
        &precommit_openings,
        5,
    )?
    .ok_or_else(|| {
        AkitaError::UnsupportedSchedule(
            "full-ring original producer contracts are infeasible".into(),
        )
    })?;
    let roots = root_level_candidates_for_prepared_producers(
        key, source, policy, dimensions, opening, &producers, 5, 5, None,
    )?;
    let layout = key.opening_layout()?;
    // Expanded setup, fixed wire bytes, successor capacity, canonical descriptor.
    type CandidateKey = (usize, usize, usize, Vec<u8>);
    let mut best: Option<(CandidateKey, PlannedFoldSchedule)> = None;
    for (root, capacity) in roots {
        root.validate_full_ring_execution(2)?;
        if capacity < 128 || !capacity.is_power_of_two() {
            return Err(AkitaError::InvalidSetup(
                "full-ring successor is not a physical identity cube".into(),
            ));
        }
        let ring_count = capacity / 128;
        for position_bits in 0..=ring_count.trailing_zeros() {
            let positions = 1usize.checked_shl(position_bits).ok_or_else(|| {
                AkitaError::InvalidSetup("terminal position count overflow".into())
            })?;
            let blocks = ring_count / positions;
            let codec = TerminalResponseCodec::full_ring(blocks)?;
            let TerminalResponseCodec::SignedFixedWidth {
                coefficient_bound, ..
            } = codec
            else {
                return Err(AkitaError::InvalidSetup(
                    "full-ring fixed codec mismatch".into(),
                ));
            };
            let collision = akita_params::sis::role_a_collision_inf_norm_for_response_bound(
                56,
                coefficient_bound,
            )
            .ok_or_else(|| AkitaError::InvalidSetup("terminal collision bound overflow".into()))?;
            let Some(table_key) = akita_params::sis::sis_table_key_for_linf_bound(
                policy.sis_security_policy,
                policy.sis_table_digest,
                policy.sis_modulus_profile,
                akita_params::SisMatrixRole::Inner,
                128,
                collision,
            ) else {
                continue;
            };
            // At fixed source geometry every larger certified rank increases
            // setup and payload without changing any response/witness bound.
            let Ok(matrix) =
                akita_params::InnerCommitMatrixParams::try_new_with_min_rank(table_key, positions)
            else {
                continue;
            };
            let mut terminal = TerminalFoldParams {
                opening_method: OpeningMethod::FULL_RING_E2,
                source_encoding: akita_params::CommittedSourceEncoding::CanonicalCoefficientTable,
                response_codec: codec,
                blocks: BlockGeometry::new(ring_count, positions, blocks),
                inner: InnerRoleParams::new(GadgetDigits::new(5, 1), matrix),
                fold: GadgetDigits::new(
                    5,
                    akita_params::full_ring_response_depth(coefficient_bound)?,
                ),
                fold_challenge_config: FULL_RING_FOLD_CHALLENGE,
                response_shape: TerminalResponseShape {
                    layout: TailSegmentLayout {
                        ring_dimension: 128,
                        groups: Vec::new(),
                        logical_num_elems: 0,
                    },
                },
                input_witness_len: capacity,
            };
            terminal.response_shape = TerminalResponseShape::derive(&terminal, coefficient_bound)?;
            terminal.validate_opening_execution(2)?;
            let schedule = FoldSchedule {
                root: FoldParams {
                    input_witness_len: akita_params::root_input_witness_len(&root),
                    output_witness_len: capacity,
                    params: root.clone(),
                },
                recursive_folds: Vec::new(),
                terminal,
            };
            let profiles = CommittedGroupBatchProfile {
                final_group: root.own_group().profile,
                precommitteds: key.precommitteds.clone(),
            };
            // Geometry checks never establish installation. This is the same
            // independent semantic audit used before emitted rows enter catalogs.
            ResolvedScheduleRow::try_new(profiles, schedule.clone(), policy)?;
            let expanded_setup = schedule.full_ring_expanded_setup_coefficients()?;
            if policy
                .setup_field_budget
                .is_some_and(|limit| expanded_setup > limit)
            {
                continue;
            }
            let terminal_bytes = schedule.terminal.fixed_payload_bytes(128)?;
            let root_payload = akita_schedules::planner_support::nonterminal_level_payload_bytes(
                policy,
                &root,
                &layout,
                akita_params::FoldSuccessor::Terminal(&schedule.terminal),
                capacity,
            )?
            .direct;
            let fixed_payload = root
                .opening_payload_geometry()?
                .transmitted_coefficients()
                .checked_mul(16)
                .and_then(|n| n.checked_add(terminal_bytes))
                .ok_or_else(|| {
                    AkitaError::InvalidSetup("full-ring fixed payload overflow".into())
                })?;
            let score = (
                expanded_setup,
                fixed_payload,
                capacity,
                schedule.canonical_descriptor_bytes(),
            );
            if best
                .as_ref()
                .is_some_and(|(incumbent, _)| incumbent <= &score)
            {
                continue;
            }
            let estimate = FoldScheduleEstimate {
                nonce_max_bytes: 0,
                estimated_root_direct_payload_bytes: root_payload,
                estimated_root_stage3_payload_bytes: 0,
                estimated_recursive_direct_payload_bytes: Vec::new(),
                estimated_recursive_stage3_payload_bytes: Vec::new(),
                estimated_terminal_direct_payload_bytes: terminal_bytes,
                estimated_terminal_response_payload_bytes: terminal_bytes,
                estimated_num_setup_field_elements:
                    akita_params::setup_matrix_capacity_for_schedule(&schedule)?.num_field_elements,
                first_direct_setup_field_len: None,
                selected_offload_edges: 0,
            };
            best = Some((score, PlannedFoldSchedule { schedule, estimate }));
        }
    }
    best.map(|(_, planned)| planned).ok_or_else(|| {
        AkitaError::UnsupportedSchedule(
            "no finite installed-certificate full-ring root/A-only terminal candidate".into(),
        )
    })
}
