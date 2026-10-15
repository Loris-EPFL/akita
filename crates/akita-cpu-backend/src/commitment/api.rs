//! Top-level prover commitment API.

use crate::commitment::CommitmentSource;
#[cfg(test)]
use crate::commitment::{
    CommitmentExecutionPlan, CommitmentExecutor, CommitmentStatePolicy, PortableCommitmentHandle,
};
use crate::validation::{signed_digit_kernel_for_setup, validate_i8_setup_log_basis};
use akita_algebra::ring::cyclotomic::decompose_centering_threshold;
use akita_config::{ensure_prover_schedule_fits_setup, CommitmentConfig, TrustedScheduleCatalog};
#[cfg(test)]
use akita_error::checked;
use akita_error::AkitaError;
use akita_params::sis::CommittedSourceContract;
#[cfg(test)]
use akita_params::CommittedGroupParams;
use akita_params::{
    validate_role_dims, validate_role_dims_for_field, CommitmentRingDims, GadgetDigits,
    GroupCommitPhaseParams, PrecommittedGroupProfiles, ScheduleLookupKey,
};
use akita_types::AkitaExpandedSetup;
#[cfg(test)]
use akita_types::{Commitment, CommittedGroup, FpExtEncoding};
use jolt_field::{CanonicalEncoding, Field};
#[cfg(test)]
use jolt_field::{Ring, Unreduced};

/// Ordered groups committed before the current group.
#[derive(Debug, Clone, Copy)]
enum PrecommittedGroupContext<'a> {
    /// The current group has no earlier groups in its opening batch.
    NoPrecommittedGroups,
    /// Exact precommitted profiles in opening-claim and transcript order.
    WithPrecommittedGroups(&'a PrecommittedGroupProfiles),
}

impl PrecommittedGroupContext<'_> {
    /// Borrow the ordered precommitted profiles, empty when there are none.
    fn as_slice(&self) -> &[GroupCommitPhaseParams] {
        match self {
            Self::NoPrecommittedGroups => &[],
            Self::WithPrecommittedGroups(profiles) => profiles.as_slice(),
        }
    }
}

/// Authority for the current group's commitment parameters.
#[derive(Debug, Clone, Copy)]
enum GroupParameterSource<'a> {
    /// Select an existing scalar or grouped row from the generated catalog.
    Scheduler,
    /// Use a caller-supplied commit profile without catalog selection.
    Explicit(&'a GroupCommitPhaseParams),
}

/// Complete context for committing one polynomial group.
#[derive(Debug, Clone, Copy)]
pub struct GroupContext<'a> {
    precommitted_groups: PrecommittedGroupContext<'a>,
    parameter_source: GroupParameterSource<'a>,
}

impl<'a> GroupContext<'a> {
    /// Select the scalar row, the generated row for a group with no precommitted groups.
    #[must_use]
    pub const fn scheduler_without_precommitted_groups() -> Self {
        Self {
            precommitted_groups: PrecommittedGroupContext::NoPrecommittedGroups,
            parameter_source: GroupParameterSource::Scheduler,
        }
    }

    /// Select the grouped row keyed on these exact ordered precommitted profiles.
    #[must_use]
    pub const fn scheduler_with_precommitted_groups(
        precommitteds: &'a PrecommittedGroupProfiles,
    ) -> Self {
        Self {
            precommitted_groups: PrecommittedGroupContext::WithPrecommittedGroups(precommitteds),
            parameter_source: GroupParameterSource::Scheduler,
        }
    }

    /// Commit a caller-supplied frozen commit profile without catalog lookup.
    ///
    /// The profile fully determines the A/B commitment. Any opening schedule the
    /// caller intends to use later — including a grouped root over precommitted
    /// groups — is validated where the opening consumes it, not at commit time.
    #[must_use]
    pub const fn explicit(profile: &'a GroupCommitPhaseParams) -> Self {
        Self {
            precommitted_groups: PrecommittedGroupContext::NoPrecommittedGroups,
            parameter_source: GroupParameterSource::Explicit(profile),
        }
    }
}

/// Result of committing one polynomial group with policy-selected private state.
#[derive(Debug)]
#[cfg(test)]
pub(crate) struct CommitOutput<F: Field, S = PortableCommitmentHandle<F>> {
    /// Self-describing committed group.
    pub committed_group: CommittedGroup<F>,
    /// Prover-private state selected by the commitment executor's policy.
    pub prover_state: S,
}

pub(super) fn validate_commitment_geometry<F>(
    profile: &GroupCommitPhaseParams,
    setup: &AkitaExpandedSetup<F>,
) -> Result<(), AkitaError>
where
    F: Field + CanonicalEncoding,
{
    signed_digit_kernel_for_setup(
        profile.inner.digits.log_basis,
        "for signed witness commitment decomposition",
    )?;
    validate_i8_setup_log_basis(
        profile.outer.digits.log_basis,
        "for i8 outer commitment decomposition",
    )?;

    // A/B geometry is independent of the D/opening matrix. Mirroring B into
    // the opening slot lets the shared role validator enforce only the two
    // dimensions represented by this borrowed view.
    let dims = CommitmentRingDims {
        inner: profile.inner.matrix.ring_dimension(),
        outer: profile.outer.matrix.ring_dimension(),
        opening: profile.outer.matrix.ring_dimension(),
    };
    validate_role_dims(dims)?;
    validate_role_dims_for_field::<F>(dims)?;

    let expected_a_width = profile
        .blocks()
        .positions_per_block
        .checked_mul(profile.inner.digits.num_digits)
        .ok_or_else(|| AkitaError::InvalidSetup("A commit width overflow".to_string()))?;
    if profile.inner.matrix.input_width() != expected_a_width {
        return Err(AkitaError::InvalidSetup(format!(
            "commit profile A width {} does not match num_positions_per_block * num_digits_inner = {expected_a_width}",
            profile.inner.matrix.input_width()
        )));
    }
    if profile.outer.matrix.input_width() == 0 {
        return Err(AkitaError::InvalidSetup(format!(
            "commit profile requires nonzero B width, got B={}",
            profile.outer.matrix.input_width()
        )));
    }

    let required = akita_params::commit_only_setup_field_elements(
        &profile.inner.matrix,
        &profile.outer.matrix,
        profile.outer_slice_count,
    )?;
    let available = setup.shared_matrix.num_field_elements();
    if required > available {
        return Err(AkitaError::InvalidSetup(format!(
            "commit profile requires {required} setup field elements for commitment, but setup has {available}"
        )));
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn validate_commit_level_params<F>(
    params: &CommittedGroupParams,
    setup: &AkitaExpandedSetup<F>,
    fold_level: usize,
    num_polynomials: usize,
) -> Result<(), AkitaError>
where
    F: Field + CanonicalEncoding,
{
    params.validate_commitment_request(fold_level, num_polynomials)?;
    if params.blocks().live_blocks == 0 || params.blocks().positions_per_block == 0 {
        return Err(AkitaError::InvalidSetup(
            "commit params require nonzero num_live_blocks and num_positions_per_block".to_string(),
        ));
    }
    if params.inner().digits.num_digits == 0 || params.outer().digits.num_digits == 0 {
        return Err(AkitaError::InvalidSetup(
            "commit params require nonzero A/B digit depths".to_string(),
        ));
    }
    validate_commitment_geometry::<F>(&params.own_group().profile, setup)?;

    // D/opening geometry is level-only: standalone commitment profiles freeze
    // only the A/B matrices used to materialize the commitment.
    if params.open().digits.num_digits == 0 {
        return Err(AkitaError::InvalidSetup(
            "commit params require nonzero opening digit depth".to_string(),
        ));
    }
    validate_i8_setup_log_basis(
        params.open().digits.log_basis,
        "for i8 opening decomposition",
    )?;
    let dims = params.role_dims();
    validate_role_dims(dims)?;
    validate_role_dims_for_field::<F>(dims)?;
    if params.open().matrix.input_width() == 0 {
        return Err(AkitaError::InvalidSetup(format!(
            "commit params require nonzero D width, got D={}",
            params.open().matrix.input_width()
        )));
    }
    // Commitment materialization uses only A and B. In particular, a
    // standalone group extracted from an approved multi-group row may retain
    // that row's shared D geometry, which is consumed only if the group later
    // participates in the selected opening schedule. Charging D here would
    // reject a setup that exactly fits the standalone commitment profile.
    Ok(())
}

/// Validate a singleton commitment request against prover setup capacity.
///
/// # Errors
///
/// Returns an error if the request is empty, mixes polynomial dimensions, or
/// exceeds the prover setup capacity.
pub(crate) fn resolve_source_layout<F: Field, P: CommitmentSource<F>>(
    polys: &[P],
) -> Result<akita_params::PolynomialGroupLayout, AkitaError> {
    let first = polys.first().ok_or_else(|| {
        AkitaError::InvalidInput("commit requires at least one polynomial".into())
    })?;
    let num_vars = first.descriptor()?.num_vars();
    for source in &polys[1..] {
        if source.descriptor()?.num_vars() != num_vars {
            return Err(AkitaError::InvalidInput(
                "batched source variable counts differ".into(),
            ));
        }
    }
    Ok(akita_params::PolynomialGroupLayout::new(
        num_vars,
        polys.len(),
    ))
}

pub(crate) fn resolve_polynomial_group_layout<F, P>(
    polys: &[P],
    setup: &AkitaExpandedSetup<F>,
) -> Result<akita_params::PolynomialGroupLayout, AkitaError>
where
    F: Field,
    P: CommitmentSource<F>,
{
    let layout = resolve_source_layout::<F, P>(polys)?;
    let num_vars = layout.num_vars();
    if polys.len() > setup.descriptor.max_num_batched_polys {
        return Err(AkitaError::InvalidInput(format!(
            "commit received {} polynomials but setup supports at most {}",
            polys.len(),
            setup.descriptor.max_num_batched_polys
        )));
    }
    if num_vars > setup.descriptor.max_num_vars {
        return Err(AkitaError::InvalidInput(format!(
            "commit received a polynomial with {} variables but setup supports at most {}",
            num_vars, setup.descriptor.max_num_vars
        )));
    }
    Ok(layout)
}

#[cfg(test)]
fn checked_commit_b_input_len(total_polys: usize, per_poly: usize) -> Result<usize, AkitaError> {
    checked::product([total_polys, per_poly]).ok_or_else(|| {
        AkitaError::InvalidInput(format!(
            "commit B digit input length overflow for {total_polys} polynomials with {per_poly} digits each"
        ))
    })
}

/// Reject a group whose logical source representation differs from the class
/// whose honest-response bounds the schedule uses.
fn ensure_sources_match_declared_class<F, P>(
    polys: &[P],
    contract: CommittedSourceContract,
) -> Result<(), AkitaError>
where
    F: Field,
    P: CommitmentSource<F>,
{
    let Some(required_chunk_size) = contract.class().required_onehot_chunk_size() else {
        return Ok(());
    };
    for poly in polys {
        match poly.descriptor()?.class() {
            crate::commitment::CommitSourceClass::OneHot { chunk_size }
                if chunk_size == required_chunk_size => {}
            crate::commitment::CommitSourceClass::OneHot { chunk_size } => {
                return Err(AkitaError::InvalidInput(format!(
                    "committed source is a unit one-hot representation with chunk size \
                     {chunk_size}, but this schedule is priced for one hot position per \
                     {required_chunk_size} coefficients"
                )))
            }
            _ => {
                return Err(AkitaError::InvalidInput(format!(
                    "committed source is not a unit one-hot representation, but this schedule \
                     is priced for one hot position per {required_chunk_size} coefficients; \
                     a dense source can satisfy the digit envelope while carrying far more \
                     energy than the frozen response caps allow"
                )))
            }
        }
    }
    Ok(())
}

/// Admit a structurally one-hot source to the full-ring Boolean whole grid.
///
/// `OneHot { chunk_size: k }` guarantees every coefficient is zero or one,
/// hence at most one unit in each singleton chunk, regardless of `k`. The
/// singleton contract charges the entire grid (ring L1 <= 128 and squared L2
/// <= logical length), so it does not rely on the stronger source sparsity.
/// This implication changes admission only: the descriptor and its actual
/// stride remain untouched for commitment and original-source digit access.
/// Non-Boolean contracts retain the legacy exact structural check.
fn ensure_full_ring_sources_match_declared_class<F, P>(
    polys: &[P],
    contract: CommittedSourceContract,
    digits: GadgetDigits,
    encoding: akita_params::CommittedSourceEncoding,
    full_ring_e2: bool,
) -> Result<(), AkitaError>
where
    F: Field,
    P: CommitmentSource<F>,
{
    if contract.class().required_onehot_chunk_size() != Some(1) {
        return ensure_sources_match_declared_class::<F, P>(polys, contract);
    }
    if !full_ring_e2
        || encoding != akita_params::CommittedSourceEncoding::CanonicalCoefficientTable
        || digits != GadgetDigits::new(5, 1)
    {
        return Err(AkitaError::InvalidSetup(
            "Boolean whole-grid admission requires canonical full-ring E2 with one base32 digit"
                .into(),
        ));
    }
    for poly in polys {
        if !matches!(
            poly.descriptor()?.class(),
            crate::commitment::CommitSourceClass::OneHot { .. }
        ) {
            return Err(AkitaError::InvalidInput(
                "Boolean whole-grid producer requires a structurally unit one-hot source".into(),
            ));
        }
    }
    Ok(())
}

/// Admit original plane envelopes before considering decoded coefficient reach.
/// Canonical sources still use the exact declared/representable intersection.
fn ensure_sources_fit_accepted_interval<F, P>(
    polys: &[P],
    inner_digits: GadgetDigits,
    contract: CommittedSourceContract,
) -> Result<(), AkitaError>
where
    F: Field + CanonicalEncoding,
    P: CommitmentSource<F>,
{
    if contract.decomposition().field_bits() != F::MODULUS_BITS {
        return Err(AkitaError::InvalidSetup(
            "producer contract field width differs from the backend field".into(),
        ));
    }
    let original_geometry = contract.original_plane_geometry();
    if original_geometry
        .is_some_and(|geometry| geometry != (inner_digits.log_basis, inner_digits.num_digits))
    {
        return Err(AkitaError::InvalidInput(
            "original source contract differs from the committed plane geometry".into(),
        ));
    }
    for poly in polys {
        let intrinsic_geometry = match poly.descriptor()?.class() {
            crate::commitment::CommitSourceClass::OriginalBalancedDigitPlanes {
                log_basis,
                num_digits,
            } => Some((log_basis, num_digits)),
            _ => None,
        };
        if intrinsic_geometry != original_geometry {
            return Err(AkitaError::InvalidInput(
                "original digit planes require their full-width original-plane producer contract"
                    .into(),
            ));
        }
    }
    if original_geometry.is_some() {
        return Ok(());
    }
    let modulus = (-F::one())
        .to_u128_checked()
        .expect("Akita field element must fit in u128")
        + 1;
    let threshold =
        decompose_centering_threshold(inner_digits.num_digits, inner_digits.log_basis, modulus);
    let (negative_reach, positive_reach) =
        contract.accepted_bounds(inner_digits.log_basis, inner_digits.num_digits);
    let exceeds = |negative_abs: u128, positive: u128| {
        negative_reach.is_some_and(|reach| negative_abs > reach)
            || positive_reach.is_some_and(|reach| positive > reach)
    };
    if !exceeds(modulus.saturating_sub(threshold + 1), threshold) {
        return Ok(());
    }
    for poly in polys {
        let (negative_abs, positive) = poly.committed_centered_reach(modulus, threshold)?;
        if exceeds(negative_abs, positive) {
            return Err(AkitaError::InvalidInput(
                "committed source exceeds the scheduled coefficient bound".into(),
            ));
        }
    }
    Ok(())
}

/// Resolve the frozen commit params of one root commitment and admit its
/// sources, returning the producer contract that admitted them.
pub(crate) fn resolve_commit_params<Cfg, P>(
    polys: &[P],
    expanded: &AkitaExpandedSetup<Cfg::Field>,
    schedules: &TrustedScheduleCatalog<Cfg>,
    context: GroupContext<'_>,
) -> Result<(GroupCommitPhaseParams, CommittedSourceContract), AkitaError>
where
    Cfg: CommitmentConfig,
    Cfg::Field: Field + CanonicalEncoding,
    P: CommitmentSource<Cfg::Field>,
{
    let polynomial_group_layout =
        resolve_polynomial_group_layout::<Cfg::Field, P>(polys, expanded)?;

    // The frozen commit params fully determine the A/B commitment. D/opening
    // geometry and source encoding are validated where the opening schedule
    // consumes them, so the commit path works from the commit params alone.
    let commit_params: GroupCommitPhaseParams =
        if let GroupParameterSource::Explicit(commit_params) = context.parameter_source {
            if commit_params.group != polynomial_group_layout {
                return Err(AkitaError::InvalidSetup(
                    "explicit commit params do not match the polynomial group being committed"
                        .into(),
                ));
            }
            *commit_params
        } else {
            let key = ScheduleLookupKey::decoded(
                polynomial_group_layout, context.precommitted_groups.as_slice().to_vec(),
            );
            let scheduled_row = schedules.resolve_key(&key)?;

            // A final group with precommitted groups consumes the row's whole
            // schedule. A standalone group is admitted on its own A/B footprint.
            if matches!(
                context.precommitted_groups,
                PrecommittedGroupContext::WithPrecommittedGroups(_)
            ) {
                ensure_prover_schedule_fits_setup::<Cfg>(
                    expanded,
                    scheduled_row.schedule(),
                    &key.opening_layout()?,
                )?;
            }

            scheduled_row.profiles().final_group
        };

    // The commit params fully determine the A/B commitment, so this is the
    // complete commit-time gate. D/opening geometry and source encoding are not
    // part of the commit params: they are validated wherever the opening
    // schedule consumes them, never at commit time.
    commit_params.validate_frozen_precommit(
        commit_params
            .inner
            .matrix
            .sis_modulus_profile()
            .field_bits(),
    )?;
    validate_commitment_geometry::<Cfg::Field>(&commit_params, expanded)?;

    // Both admission gates read the `Cfg` declaration rather than commitment
    // geometry, so they run here instead of inside the commit kernel.
    let contract = Cfg::committed_source_contract()?;
    ensure_sources_match_declared_class::<Cfg::Field, P>(polys, contract)?;
    ensure_sources_fit_accepted_interval::<Cfg::Field, P>(
        polys,
        commit_params.inner.digits,
        contract,
    )?;

    Ok((commit_params, contract))
}

/// Resolve the same source admission gates against literal installed roles,
/// without fabricating a legacy shared-prefix setup descriptor.
pub(crate) fn resolve_full_ring_commit_params<Cfg, P>(
    polys: &[P],
    setup: &akita_types::FullRingSetupDescriptor,
    source_id: akita_types::FullRingSourceId,
    schedules: &TrustedScheduleCatalog<Cfg>,
    context: GroupContext<'_>,
) -> Result<(GroupCommitPhaseParams, CommittedSourceContract), AkitaError>
where
    Cfg: CommitmentConfig,
    P: CommitmentSource<Cfg::Field>,
{
    let layout = resolve_source_layout::<Cfg::Field, P>(polys)?;
    let params = match context.parameter_source {
        GroupParameterSource::Explicit(params) => *params,
        GroupParameterSource::Scheduler => {
            schedules
                .resolve_key(&ScheduleLookupKey::decoded(
                    layout, context.precommitted_groups.as_slice().to_vec(),
                ))?
                .profiles()
                .final_group
        }
    };
    let contract = Cfg::committed_source_contract()?;
    validate_full_ring_commit_source::<Cfg::Field, P>(
        polys,
        setup,
        source_id,
        params,
        contract,
        layout,
        Cfg::opening_family().is_full_ring_e2()
            && Cfg::EXT_DEGREE == 2
            && <Cfg::ExtField as jolt_field::ExtField<Cfg::Field>>::DEGREE == 2,
    )?;
    Ok((params, contract))
}

/// Admit a frozen producer independently of the opening family's own source.
pub(crate) fn validate_full_ring_commit_source<F, P>(
    polys: &[P],
    setup: &akita_types::FullRingSetupDescriptor,
    source_id: akita_types::FullRingSourceId,
    params: GroupCommitPhaseParams,
    contract: CommittedSourceContract,
    layout: akita_params::PolynomialGroupLayout,
    native_e2: bool,
) -> Result<(), AkitaError>
where
    F: Field + CanonicalEncoding,
    P: CommitmentSource<F>,
{
    params.validate_frozen_precommit(F::MODULUS_BITS)?;
    if contract.decomposition().field_bits() != F::MODULUS_BITS {
        return Err(AkitaError::InvalidSetup(
            "producer contract field width differs from the backend field".into(),
        ));
    }
    let source = setup.source(source_id)?;
    if params.group != layout
        || source.polynomials != layout.num_polynomials()
        || source.blocks != params.blocks.live_blocks
        || source.positions != params.blocks.positions_per_block
        || source.digit_depth != params.inner.digits.num_digits
        || source.rank != params.inner.matrix.output_rank()
        || params.inner.matrix.ring_dimension() != 128
        || params.outer.matrix.ring_dimension() != 128
        || params.outer.matrix.output_rank() != 1
        || params.outer_slice_count != akita_params::CommitmentSliceCount::ONE
        || params.inner.digits.log_basis != 5
        || params.outer.digits.log_basis != 5
        || params.outer.digits.num_digits != 26
    {
        return Err(AkitaError::InvalidSetup(
            "source commitment differs from installed full-ring geometry".into(),
        ));
    }
    ensure_full_ring_sources_match_declared_class::<F, P>(
        polys,
        contract,
        params.inner.digits,
        source.source_encoding,
        native_e2,
    )?;
    ensure_sources_fit_accepted_interval::<F, P>(polys, params.inner.digits, contract)?;
    Ok(())
}

/// Commit one homogeneous polynomial group in its complete parameter context.
///
/// Scheduler contexts select an existing S or G catalog row. Explicit
/// contexts commit a caller-supplied frozen profile without catalog lookup.
/// Root commitments always consume the canonical coefficient table.
///
/// # Errors
///
/// Returns an error for an empty or mixed-arity group, unsupported role
/// parameters, insufficient setup, or commitment execution failure.
#[cfg(test)]
pub(crate) fn commit<Cfg, P, SP>(
    polys: &[P],
    expanded: &AkitaExpandedSetup<Cfg::Field>,
    schedules: &TrustedScheduleCatalog<Cfg>,
    executor: &CommitmentExecutor<'_, Cfg::Field, SP>,
    context: GroupContext<'_>,
) -> Result<CommitOutput<Cfg::Field, SP::State>, AkitaError>
where
    Cfg: CommitmentConfig,
    Cfg::Field: Field + CanonicalEncoding + Ring + Unreduced + 'static,
    <Cfg::Field as Unreduced>::Wide: From<Cfg::Field>,
    Cfg::ExtField: FpExtEncoding<Cfg::Field>,
    P: CommitmentSource<Cfg::Field>,
    SP: CommitmentStatePolicy<Cfg::Field>,
{
    executor.validate_setup(expanded)?;
    let (commit_params, _) = resolve_commit_params::<Cfg, P>(polys, expanded, schedules, context)?;
    let execution_plan = CommitmentExecutionPlan::for_root(&commit_params)?;
    let source_refs: Vec<&dyn CommitmentSource<Cfg::Field>> = polys
        .iter()
        .map(|poly| poly as &dyn CommitmentSource<Cfg::Field>)
        .collect();
    let (payload, prover_state) = executor
        .execute_full(&execution_plan, &source_refs)?
        .into_parts();

    Ok(CommitOutput {
        committed_group: CommittedGroup::new(commit_params, Commitment::new(payload)),
        prover_state,
    })
}

#[cfg(test)]
#[path = "tests/api.rs"]
mod tests;
