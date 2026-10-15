//! Runtime schedule shapes shared by configs, prover, verifier, and planner.

use crate::{
    CommittedGroupParams, InnerCommitSecurityRoute, OpeningMethod, RelationAddressGeometry,
    SetupContributionMode, TerminalResponseShape,
};
use akita_error::AkitaError;

mod descriptor;
mod profiles;
mod sis_occurrences;
mod sizing;

pub use profiles::{
    CommittedGroupBatchProfile, CommittedSourceEncoding, GroupCommitPhaseParams,
    PrecommittedGroupProfiles, ScheduleLookupKey, ScheduleLookupOrderKey,
};
pub use sis_occurrences::{ScheduleSisBound, ScheduleSisOccurrence, ScheduleSisRole};
pub use sizing::{detect_field_modulus, r_decomp_levels};

/// Transcript binding used for one fold's outgoing witness state.
///
/// This is schedule-owned because the same intermediate proof body may either
/// recurse through an outer commitment or hand its witness to the final
/// suffix fold as a public inner `t` state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NextWitnessBindingPolicy {
    /// Bind the terminal compressed commitment payload and recurse.
    OuterPayload,
    /// Bind canonical inner-state `t` bytes for the following suffix-terminal
    /// fold. No outer `u` is present on this edge.
    TerminalInnerState,
}

/// Parameters for one fold level: the root fold or one recursive fold.
///
/// Replaces `RootFoldParams` and `RecursiveFoldParams`. Their nested
/// `RootFinalGroupParams`, `RootPrecommittedGroupParams`, `WitnessPartition`,
/// and `ScheduledSetupPrefix` types split one level across several owners.
/// The overlap was kept honest by equality audits. Each audit compared a
/// field with a copy of itself:
///
/// - both fold types stored `open_commit_matrix` beside the same matrix in
///   `params`;
/// - `sparse_challenge_config` on both duplicated `params.fold_challenge_config`;
/// - root precommitted entries duplicated the groups already held by `params`;
/// - `RecursiveFoldParams::incoming_setup_prefix` duplicated the setup-prefix
///   group already held by `params`;
/// - `RootFinalGroupParams` was a one-field wrapper.
///
/// Root and recursive folds share this type because after the merge they hold
/// identical fields. What separates them stays a validated constraint, as it
/// already was: `FoldSchedule` names the three positions, so no role is inferred
/// from an array index.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FoldParams {
    /// This fold's own parameters, including its final/new group, its
    /// precommitted groups, the shared D matrix, and any incoming setup prefix.
    pub params: CommittedGroupParams,
    /// Witness field length entering this fold.
    pub input_witness_len: usize,
    /// Witness field length leaving this fold.
    pub output_witness_len: usize,
}

impl FoldParams {
    /// The incoming setup prefix, when this fold consumes one.
    #[inline]
    #[must_use]
    pub fn incoming_setup_prefix(&self) -> Option<&crate::GroupOpenPhaseParams> {
        self.params.setup_prefix()
    }

    /// Setup-contribution mode of the fold that produces this witness.
    ///
    /// Presence of the consumer-owned prefix is the sole authority; there is no
    /// separately stored mode that could disagree with adjacency.
    #[must_use]
    pub fn predecessor_setup_contribution_mode(&self) -> SetupContributionMode {
        if self.params.setup_prefix().is_some() {
            SetupContributionMode::Recursive
        } else {
            SetupContributionMode::Direct
        }
    }
}

/// The full-ring shell has forty positions and L1 mass 56. The historical
/// production d128 ladder uses (31, 0) and is a different protocol family.
pub const FULL_RING_FOLD_CHALLENGE: akita_challenges::SparseChallengeConfig =
    akita_challenges::SparseChallengeConfig {
        count_pm1: 24,
        count_pm2: 16,
    };

/// Least base32 response depth whose positive balanced reach covers every
/// signed response in [-bound, bound]. This is not a probabilistic cap.
pub fn full_ring_response_depth(bound: u128) -> Result<usize, AkitaError> {
    let mut reach = 15u128;
    let mut depth = 1usize;
    while reach < bound {
        reach = reach
            .checked_mul(32)
            .and_then(|n| n.checked_add(15))
            .ok_or_else(|| {
                AkitaError::InvalidSetup(
                    "full-ring response depth exceeds checked integer range".into(),
                )
            })?;
        depth += 1;
    }
    Ok(depth)
}

/// Terminal response wire representation, authenticated with the method/source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TerminalResponseCodec {
    /// Legacy canonical Golomb-Rice, with the shape's signed-i16 bound.
    GolombRice,
    /// Exact little-endian two's-complement width, checked before K reduction.
    SignedFixedWidth {
        bytes_per_coefficient: usize,
        coefficient_bound: u128,
    },
}

impl TerminalResponseCodec {
    /// All-valid d128 shell response for a source alphabet bounded by 16.
    pub fn full_ring(blocks: usize) -> Result<Self, AkitaError> {
        let bound = (blocks as u128)
            .checked_mul(896)
            .filter(|&n| n > 0)
            .ok_or_else(|| AkitaError::InvalidSetup("full-ring terminal bound overflow".into()))?;
        let modulus = u128::MAX - (1u128 << 32) + 22538;
        if bound >= modulus / 2 {
            return Err(AkitaError::InvalidSetup(
                "terminal response is not a unique signed K representative".into(),
            ));
        }
        Ok(Self::SignedFixedWidth {
            bytes_per_coefficient: (1 + (128 - bound.leading_zeros()) as usize).div_ceil(8),
            coefficient_bound: bound,
        })
    }

    pub(crate) fn append_descriptor_bytes(self, bytes: &mut Vec<u8>) {
        match self {
            Self::GolombRice => bytes.push(0),
            Self::SignedFixedWidth {
                bytes_per_coefficient,
                coefficient_bound,
            } => {
                bytes.push(1);
                crate::descriptor_bytes::push_usize(bytes, bytes_per_coefficient);
                bytes.extend_from_slice(&coefficient_bound.to_le_bytes());
            }
        }
    }
}

/// Exact terminal committed-witness parameters.
///
/// The terminal relation binds the source decomposition through the inner
/// commitment matrix. It also retains the terminal fold basis and digit count
/// needed to audit a calibrated L2 route. It has no outer/open commitment
/// matrix and no outer/open response decomposition.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TerminalFoldParams {
    /// Declared terminal algorithm; extension degree never selects a method.
    pub opening_method: OpeningMethod,
    /// The original representation used when the terminal A images were bound.
    pub source_encoding: CommittedSourceEncoding,
    /// Method-bound integer response representation.
    pub response_codec: TerminalResponseCodec,
    /// Exact `(N, M, B)` block split of the terminal source.
    pub blocks: crate::BlockGeometry,
    /// A/source role: gadget decomposition and audited matrix identity.
    pub inner: crate::InnerRoleParams,
    /// Response basis and depth this terminal fold was planned against.
    pub fold: crate::GadgetDigits,
    /// Fold-challenge family for the terminal response.
    pub fold_challenge_config: akita_challenges::SparseChallengeConfig,
    /// Shape of the clear terminal response payload.
    pub response_shape: TerminalResponseShape,
    /// Witness field length entering the terminal fold.
    pub input_witness_len: usize,
}

/// Minimum fraction of the unconstrained terminal-response target that a
/// fixed inner matrix must admit. This is a planner completeness heuristic,
/// not a security assumption: security always uses the matrix's exact
/// SIS-certified capacity.
pub const TERMINAL_RESPONSE_MIN_TARGET_RETAIN_NUM: u128 = 1;
pub const TERMINAL_RESPONSE_MIN_TARGET_RETAIN_DEN: u128 = 2;

impl TerminalFoldParams {
    /// Project a fold's params into terminal parameters.
    ///
    /// `response_shape` and `input_witness_len` are placeholders: the response
    /// shape is derived from the admission cap this projection computes, so it
    /// cannot be known here. Callers assemble it and assign both, mirroring the
    /// existing `params_only` then `with_decomp` idiom on `CommittedGroupParams`.
    pub fn from_expanded_group(params: CommittedGroupParams) -> Self {
        Self {
            opening_method: if params.opening_method().is_full_ring_scalar_extension() {
                params.opening_method()
            } else {
                OpeningMethod::EvaluationTrace
            },
            source_encoding: params.source_encoding,
            response_codec: TerminalResponseCodec::GolombRice,
            fold_challenge_config: params.fold_challenge_config(),
            response_shape: TerminalResponseShape {
                layout: crate::TailSegmentLayout {
                    ring_dimension: params.d_a(),
                    groups: Vec::new(),
                    logical_num_elems: 0,
                },
            },
            input_witness_len: 0,
            blocks: params.blocks(),
            inner: crate::RoleParams::new(
                crate::GadgetDigits::new(
                    params.inner().digits.log_basis,
                    params.inner().digits.num_digits,
                ),
                params.inner().matrix,
            ),
            fold: crate::GadgetDigits::new(
                params.open().digits.log_basis,
                params.num_digits_fold(),
            ),
        }
    }

    /// Project an ordinary scalar group into terminal parameters and validate
    /// the directly checked response bound against its fixed inner matrix.
    pub fn try_from_expanded_group(
        params: CommittedGroupParams,
    ) -> Result<(Self, u128), AkitaError> {
        let sparse = params.fold_challenge_config();
        let num_fold_coeffs = usize::try_from(params.num_fold_coeffs()).map_err(|_| {
            AkitaError::InvalidSetup("terminal fold coefficient count exceeds usize".into())
        })?;
        let cap_config =
            crate::sis::FoldWitnessLinfCapConfig::for_fold_coeffs(&sparse, num_fold_coeffs)?;
        let challenge = crate::sis::FoldChallengeNorms::new(&sparse);
        let witness =
            crate::sis::FoldWitnessNorms::bounded(params.inner().digits.log_basis, params.d_a());
        let (unconstrained_target, _) = crate::sis::fold_witness_linf_cap(
            params.blocks().live_blocks,
            1,
            challenge,
            witness,
            &cap_config,
        )?;
        let terminal = Self::from_expanded_group(params);
        let admission_cap = terminal.certified_response_linf_cap()?;
        let minimum_usable_cap = unconstrained_target
            .checked_mul(TERMINAL_RESPONSE_MIN_TARGET_RETAIN_NUM)
            .ok_or_else(|| AkitaError::InvalidSetup("terminal target ratio overflow".into()))?
            .div_ceil(TERMINAL_RESPONSE_MIN_TARGET_RETAIN_DEN);
        if admission_cap < minimum_usable_cap {
            return Err(AkitaError::InvalidSetup(format!(
                "terminal response capacity {admission_cap} retains less than \
                 {TERMINAL_RESPONSE_MIN_TARGET_RETAIN_NUM}/\
                 {TERMINAL_RESPONSE_MIN_TARGET_RETAIN_DEN} of target {unconstrained_target}"
            )));
        }
        Ok((terminal, admission_cap))
    }

    /// Construct the selected terminal from an admitted A-role candidate. The
    /// integer envelope is all-valid, not an honest-response retry target.
    pub fn try_full_ring_from_expanded_group(
        params: CommittedGroupParams,
        input_witness_len: usize,
        response_shape: TerminalResponseShape,
    ) -> Result<Self, AkitaError> {
        let mut terminal = Self::from_expanded_group(params);
        terminal.input_witness_len = input_witness_len;
        terminal.response_shape = response_shape;
        terminal.response_codec = TerminalResponseCodec::full_ring(terminal.blocks.live_blocks)?;
        terminal.validate_opening_execution(2)?;
        Ok(terminal)
    }

    /// Validate algorithm, basis, source identity, codec and actual installed A
    /// matrix together. Does not install a catalog row or certify a guest family.
    pub fn validate_opening_execution(&self, extension_degree: usize) -> Result<(), AkitaError> {
        self.source_encoding.validate(self.d_a())?;
        if !self.opening_method.is_full_ring_scalar_extension() {
            if self.opening_method != OpeningMethod::EvaluationTrace
                || self.response_codec != TerminalResponseCodec::GolombRice
            {
                return Err(AkitaError::InvalidSetup(
                    "legacy terminal requires trace/Golomb-Rice".into(),
                ));
            }
            return Ok(());
        }
        self.opening_method
            .physical_coefficient_width(extension_degree, self.d_a())?;
        let codec = TerminalResponseCodec::full_ring(self.blocks.live_blocks)?;
        if self.source_encoding != CommittedSourceEncoding::CanonicalCoefficientTable
            || self.response_codec != codec
            || self.inner.digits.num_digits != 1
            || self.inner.digits.log_basis != 5
            || self.inner.matrix.sis_modulus_profile() != crate::SisModulusProfileId::Q128OffsetA7F7
            || !matches!(
                self.inner.matrix.security_route(),
                InnerCommitSecurityRoute::Linf(_)
            )
            || self.inner_width() != self.blocks.positions_per_block
            || !self.blocks.live_blocks.is_power_of_two()
            || !self.blocks.positions_per_block.is_power_of_two()
            || self
                .blocks
                .live_blocks
                .checked_mul(self.inner_width())
                .and_then(|n| n.checked_mul(128))
                != Some(self.input_witness_len)
        {
            return Err(AkitaError::InvalidSetup(
                "full-ring terminal source/codec/identity geometry mismatch".into(),
            ));
        }
        let expected = FULL_RING_FOLD_CHALLENGE;
        if self.fold_challenge_config != expected {
            return Err(AkitaError::InvalidSetup(
                "full-ring terminal requires the d128 shell".into(),
            ));
        }
        self.inner.matrix.validate()?;
        let TerminalResponseCodec::SignedFixedWidth {
            coefficient_bound, ..
        } = codec
        else {
            return Err(AkitaError::InvalidSetup(
                "full-ring terminal codec mismatch".into(),
            ));
        };
        if coefficient_bound > self.certified_response_linf_cap()? {
            return Err(AkitaError::InvalidSetup(
                "full-ring terminal exceeds installed A certificate".into(),
            ));
        }
        if self.response_shape != TerminalResponseShape::derive(self, coefficient_bound)? {
            return Err(AkitaError::InvalidSetup(
                "full-ring terminal shape disagrees with its fixed codec".into(),
            ));
        }
        Ok(())
    }

    /// Exact fixed payload, including the once-bound A images. Signed fixed
    /// width carries no redundant length prefix; the descriptor fixes its size.
    pub fn fixed_payload_bytes(&self, field_bits: u32) -> Result<usize, AkitaError> {
        if matches!(self.response_codec, TerminalResponseCodec::GolombRice) {
            return crate::terminal_response_max_bytes(field_bits, &self.response_shape);
        }
        if field_bits != 128 {
            return Err(AkitaError::InvalidSetup(
                "full-ring terminal codec requires the q128 base field".into(),
            ));
        }
        self.validate_opening_execution(2)?;
        self.response_shape
            .layout
            .e_field_elems()
            .checked_add(self.response_shape.layout.t_field_elems())
            .and_then(|n| n.checked_mul(crate::field_bytes(field_bits)))
            .and_then(|n| n.checked_add(self.response_shape.layout.z_payload_bytes()))
            .ok_or_else(|| AkitaError::InvalidSetup("terminal fixed payload overflow".into()))
    }

    #[inline]
    pub fn d_a(&self) -> usize {
        self.inner.matrix.ring_dimension()
    }

    #[inline]
    pub fn inner_width(&self) -> usize {
        self.inner.matrix.input_width()
    }

    /// Logical opening-point width for the witness entering the terminal fold.
    pub fn recursive_opening_num_vars(&self) -> Result<usize, AkitaError> {
        crate::layout::params::recursive_opening_num_vars_for_geometry(
            self.d_a(),
            self.blocks.positions_per_block,
            self.blocks.live_blocks,
        )
    }

    /// Largest raw response admitted by a terminal Linf route's selected
    /// inner-matrix SIS bucket and signed coefficient representation.
    ///
    /// The matrix rank can incidentally support a larger collision bucket. The
    /// terminal wire does not consume that slack because doing so would change
    /// its admission and encoding bounds when an unrelated rank frontier moves.
    /// Takes no challenge family. Every production caller passed this terminal's
    /// own `fold_challenge_config` -- the prover and verifier suffixes both bind
    /// `params = &scheduled` and then passed `&scheduled.fold_challenge_config`,
    /// and `try_from_expanded_group` derives both the cap and the stored config
    /// from one `CommittedGroupParams`. Nothing forced the argument to agree with
    /// the receiver, so a caller could only ever pass the same value or silently
    /// move the admission cap that gates acceptance. Reading the field removes
    /// that gap by construction.
    pub fn certified_response_linf_cap(&self) -> Result<u128, AkitaError> {
        if let TerminalResponseCodec::SignedFixedWidth { .. } = self.response_codec {
            let key = self.inner.matrix.sis_table_key().ok_or_else(|| {
                AkitaError::InvalidSetup(
                    "full-ring terminal requires an installed Linf A matrix".into(),
                )
            })?;
            let challenge = crate::sis::FoldChallengeNorms::new(&self.fold_challenge_config);
            return crate::sis::max_response_linf_for_role_a_collision(
                key.coeff_linf_bound,
                challenge.l1_norm,
            )
            .filter(|&bound| bound > 0)
            .ok_or_else(|| {
                AkitaError::InvalidSetup("terminal matrix has no response capacity".into())
            });
        }
        if matches!(
            self.inner.matrix.security_route(),
            crate::sis::InnerCommitSecurityRoute::L2 { .. }
        ) {
            return Err(AkitaError::InvalidSetup(
                "terminal L2 route has no independent Linf cap".into(),
            ));
        }
        crate::sis::certified_terminal_response_linf_cap(
            &self.inner.matrix,
            &self.fold_challenge_config,
        )
    }

    /// Validate that the wire carries exactly the norm cap required by the
    /// selected terminal security route.
    pub fn validate_terminal_linf_cap(
        &self,
        scheduled_cap: Option<u128>,
    ) -> Result<(), AkitaError> {
        match self.inner.matrix.security_route() {
            crate::sis::InnerCommitSecurityRoute::Linf(_) => {
                let cap = scheduled_cap.ok_or_else(|| {
                    AkitaError::InvalidSetup("terminal Linf route is missing its cap".into())
                })?;
                if cap == 0 || cap > self.certified_response_linf_cap()? {
                    return Err(AkitaError::InvalidSetup(
                        "terminal Linf cap exceeds its matrix-certified capacity".into(),
                    ));
                }
            }
            crate::sis::InnerCommitSecurityRoute::L2 { .. } => {
                if scheduled_cap.is_some() {
                    return Err(AkitaError::InvalidSetup(
                        "terminal L2 route must not carry an independent Linf cap".into(),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Verifier-enforced complete physical L2 cap for a clear terminal route.
    #[must_use]
    pub fn response_l2_sq_cap(&self) -> Option<u128> {
        match self.inner.matrix.security_route() {
            crate::sis::InnerCommitSecurityRoute::Linf(_) => None,
            crate::sis::InnerCommitSecurityRoute::L2 {
                response_l2_sq_cap, ..
            } => Some(response_l2_sq_cap),
        }
    }

    /// Role-atomic order, per the plan's encoding rule: declared field order,
    /// each field written by its own encoder.
    ///
    /// This is the one encoder in the plan whose byte order changes. It used to
    /// interleave the A basis, the fold decomposition, the A matrix, the block
    /// triple, and the A depth; it now writes geometry, then the A role as
    /// `basis, depth, matrix`, then the fold decomposition.
    pub(crate) fn append_group_descriptor_bytes(&self, bytes: &mut Vec<u8>) {
        // Terminal occurrences have no registered original-RAM view.
        bytes.push(crate::SourceOpeningView::Decoded.tag());
        // The separate full-ring family additionally binds its typed header.
        if self.opening_method.is_full_ring_scalar_extension() {
            self.opening_method.append_descriptor_bytes(bytes);
            self.source_encoding.append_descriptor_bytes(bytes);
            self.response_codec.append_descriptor_bytes(bytes);
        }
        self.blocks.append_descriptor_bytes(bytes);
        self.inner.append_descriptor_bytes(bytes);
        self.fold.append_descriptor_bytes(bytes);
    }
}

/// Successor consumed by one nonterminal fold.
///
/// Recursive and terminal successors expose different wire payloads, but both
/// determine the outgoing relation domain. Proof shape, proof sizing, and
/// transcript planning therefore share this one schedule-owned distinction.
#[derive(Clone, Copy)]
pub enum FoldSuccessor<'a> {
    Recursive(&'a CommittedGroupParams),
    Terminal(&'a TerminalFoldParams),
}

impl FoldSuccessor<'_> {
    #[inline]
    #[must_use]
    pub fn ring_dimension(self) -> usize {
        match self {
            Self::Recursive(params) => params.d_a(),
            Self::Terminal(params) => params.d_a(),
        }
    }

    pub fn recursive_opening_num_vars(self) -> Result<usize, AkitaError> {
        match self {
            Self::Recursive(params) => params.recursive_opening_num_vars(),
            Self::Terminal(params) => params.recursive_opening_num_vars(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FoldSchedule {
    pub root: FoldParams,
    pub recursive_folds: Vec<FoldParams>,
    pub terminal: TerminalFoldParams,
}

/// Borrowed nonterminal step used to encode a checked planner candidate
/// without constructing a temporary [`FoldSchedule`].
#[derive(Clone, Copy)]
pub struct FoldScheduleDescriptorStep<'a> {
    pub params: &'a CommittedGroupParams,
    pub payload_mode: crate::CommitmentPayloadMode,
    pub input_witness_len: usize,
    pub output_witness_len: usize,
}

impl FoldSchedule {
    /// Sum of all expanded role coefficients for the selected direct outer
    /// family. Unlike the legacy maximum shared-prefix capacity, this prices
    /// every original A/B/F role, shared D/H, and terminal A independently.
    pub fn full_ring_expanded_setup_coefficients(&self) -> Result<usize, AkitaError> {
        if !self
            .root
            .params
            .opening_method()
            .is_full_ring_scalar_extension()
            || !self.recursive_folds.is_empty()
        {
            return Err(AkitaError::InvalidSetup(
                "expanded full-ring cost requires the direct outer family".into(),
            ));
        }
        self.validate_structure()?;
        self.sis_occurrences()?
            .iter()
            .try_fold(0usize, |total, role| {
                role.output_rank
                    .checked_mul(role.input_width)
                    .and_then(|n| n.checked_mul(role.ring_dimension))
                    .and_then(|n| total.checked_add(n))
                    .ok_or_else(|| {
                        AkitaError::InvalidSetup("expanded full-ring setup cost overflow".into())
                    })
            })
    }

    pub fn num_fold_levels(&self) -> usize {
        self.recursive_folds.len() + 2
    }

    pub fn root_fold(&self) -> &FoldParams {
        &self.root
    }

    pub fn validate_structure(&self) -> Result<(), AkitaError> {
        let root_commitment = &self.root.params;
        root_commitment.validate_group_topology()?;
        root_commitment.own_group().opening.validate_source_view(
            &root_commitment.own_group().profile, 1,
        )?;
        if root_commitment.groups().iter().any(|group| {
            group.opening.source_view == crate::SourceOpeningView::OriginalRamPlanesV1
        }) && (root_commitment.witness_chunk.num_chunks != 1
            || root_commitment.source_encoding != crate::CommittedSourceEncoding::CanonicalCoefficientTable)
        {
            return Err(AkitaError::InvalidSetup("raw roots require unchunked canonical source encoding".into()));
        }
        if root_commitment.setup_prefix().is_some() {
            return Err(AkitaError::InvalidSetup(
                "root fold cannot consume a setup prefix".into(),
            ));
        }
        root_commitment
            .validate_commitment_request(0, root_commitment.commitment_polynomial_count()?)?;
        for group in self.root.params.precommitted_groups() {
            group.validate()?;
        }
        if !self.root.params.payload_mode.is_compressed() {
            return Err(AkitaError::InvalidSetup(
                "root fold payload must be compressed".into(),
            ));
        }
        let full_ring = root_commitment
            .opening_method()
            .is_full_ring_scalar_extension();
        let expected_root_mode = if full_ring {
            crate::RingRelationMode::ExactCoefficientRows
        } else {
            crate::RingRelationMode::QuotientLift
        };
        if root_commitment.ring_relation_mode != expected_root_mode {
            return Err(AkitaError::InvalidSetup(
                "root relation mode disagrees with its opening family".into(),
            ));
        }
        let mut payload_phase = crate::CommitmentPayloadPhase::CompressedPrefix;
        let mut relation_phase = crate::RingRelationPhase::QuotientPrefix.after(expected_root_mode);
        for (index, step) in self.recursive_folds.iter().enumerate() {
            step.params.validate_group_topology()?;
            if step.params.groups().iter().any(|group| {
                group.opening.source_view != crate::SourceOpeningView::Decoded
            }) {
                return Err(AkitaError::InvalidSetup("recursive/setup openings require Decoded view".into()));
            }
            if !step.params.precommitted_groups().is_empty() {
                return Err(AkitaError::InvalidSetup(format!(
                    "recursive fold {index} cannot consume precommitted groups"
                )));
            }
            step.params.validate_commitment_request(index + 1, 1)?;
            let consumes_setup_prefix = step.params.setup_prefix().is_some();
            let absolute_level = index + 1;
            if !relation_phase
                .candidate_modes(
                    absolute_level,
                    crate::RelationCandidateTopology::new(
                        consumes_setup_prefix,
                        step.params.opening_method(),
                    ),
                )
                .contains(&step.params.ring_relation_mode)
            {
                return Err(AkitaError::InvalidSetup(format!(
                    "nonterminal level {absolute_level} ring relation mode disagrees with the reduced-evaluation suffix policy"
                )));
            }
            relation_phase = relation_phase.after(step.params.ring_relation_mode);
            if payload_phase == crate::CommitmentPayloadPhase::RawSuffix && consumes_setup_prefix {
                return Err(AkitaError::InvalidSetup(format!(
                    "recursive fold {index} cannot resume compression by consuming a setup prefix after the raw suffix"
                )));
            }
            if !payload_phase
                .candidate_modes(index + 1, consumes_setup_prefix)
                .contains(&step.params.payload_mode)
            {
                return Err(AkitaError::InvalidSetup(format!(
                    "recursive fold {index} payload mode disagrees with the compression cutover policy"
                )));
            }
            payload_phase = payload_phase.after(step.params.payload_mode);
        }
        if self.root.input_witness_len == 0 || self.root.output_witness_len == 0 {
            return Err(AkitaError::InvalidSetup(
                "root fold witness lengths must be nonzero".to_string(),
            ));
        }
        let first_successor_len = self
            .recursive_folds
            .first()
            .map_or(self.terminal.input_witness_len, |step| {
                step.input_witness_len
            });
        if self.root.output_witness_len != first_successor_len {
            return Err(AkitaError::InvalidSetup(
                "root output witness length does not match its successor".to_string(),
            ));
        }
        let (first_successor_d, first_successor_opening_num_vars) =
            self.recursive_folds.first().map_or_else(
                || -> Result<_, AkitaError> {
                    Ok((
                        self.terminal.d_a(),
                        self.terminal.recursive_opening_num_vars()?,
                    ))
                },
                |step| Ok((step.params.d_a(), step.params.recursive_opening_num_vars()?)),
            )?;
        validate_stage2_successor_capacity(
            "root fold",
            &self.root.params,
            self.root.output_witness_len,
            first_successor_d,
            first_successor_opening_num_vars,
        )?;
        for (index, step) in self.recursive_folds.iter().enumerate() {
            if step.input_witness_len == 0 || step.output_witness_len == 0 {
                return Err(AkitaError::InvalidSetup(
                    "recursive fold witness lengths must be nonzero".to_string(),
                ));
            }
            if let Some(prefix) = &step.params.setup_prefix() {
                prefix.validate().map_err(|error| {
                    AkitaError::InvalidSetup(format!(
                        "recursive fold {index} setup-prefix geometry is invalid: {error}"
                    ))
                })?;
            }
            let successor_len = self
                .recursive_folds
                .get(index + 1)
                .map_or(self.terminal.input_witness_len, |next| {
                    next.input_witness_len
                });
            if step.output_witness_len != successor_len {
                return Err(AkitaError::InvalidSetup(format!(
                    "recursive fold {index} output witness length does not match its successor"
                )));
            }
            let (successor_d, successor_opening_num_vars) =
                self.recursive_folds.get(index + 1).map_or_else(
                    || -> Result<_, AkitaError> {
                        Ok((
                            self.terminal.d_a(),
                            self.terminal.recursive_opening_num_vars()?,
                        ))
                    },
                    |next| Ok((next.params.d_a(), next.params.recursive_opening_num_vars()?)),
                )?;
            validate_stage2_successor_capacity(
                &format!("recursive fold {index}"),
                &step.params,
                step.output_witness_len,
                successor_d,
                successor_opening_num_vars,
            )?;
        }
        if self.terminal.input_witness_len == 0
            || self.terminal.response_shape.logical_num_elems() == 0
        {
            return Err(AkitaError::InvalidSetup(
                "terminal fold and response lengths must be nonzero".to_string(),
            ));
        }
        if full_ring != self.terminal.opening_method.is_full_ring_scalar_extension() {
            return Err(AkitaError::InvalidSetup(
                "terminal and root opening families disagree".into(),
            ));
        }
        if full_ring {
            self.terminal.validate_opening_execution(2)?;
            for fold in std::iter::once(&self.root).chain(self.recursive_folds.iter()) {
                fold.params.validate_full_ring_execution(2)?;
            }
        }
        Ok(())
    }

    /// Validate the opening methods currently admitted by nonterminal proving
    /// and verification.
    ///
    /// Subring coefficient packing is required at absolute levels 0 and 1.
    /// Evaluation trace is required at later nonterminal levels. Every group
    /// consumed by one fold uses the same method family. Packing requires the
    /// audited production challenge family under the L-infinity A route.
    pub fn validate_nonterminal_opening_execution(
        &self,
        extension_degree: usize,
    ) -> Result<(), AkitaError> {
        self.terminal.validate_opening_execution(extension_degree)?;
        self.validate_structure()?;
        for group in self.root.params.groups() {
            group.opening.validate_source_view(&group.profile, extension_degree)?;
        }
        if !extension_degree.is_power_of_two() {
            return Err(AkitaError::InvalidSetup(
                "opening extension degree must be a nonzero power of two".into(),
            ));
        }
        if !self.root.input_witness_len.is_power_of_two() {
            return Err(AkitaError::InvalidSetup(
                "root input witness length must be a power of two".into(),
            ));
        }
        // Canonical transcript order: earlier groups first, the fold's own
        // final/new group last. This is the ordering `preceding_group_iter`
        // already uses, and the one `FoldParams::groups` makes structural.
        let root_final = &self.root.params;
        let mut root_groups: Vec<OpeningExecutionGroup> = self
            .root
            .params
            .precommitted_groups()
            .iter()
            .map(|group| {
                let commitment = group;
                OpeningExecutionGroup {
                    opening_method: commitment.opening.opening_method,
                    inner_commit_matrix: &commitment.profile.inner.matrix,
                    fold_challenge_config: commitment.opening.fold_challenge_config,
                    // Precommitted groups are canonical by admission.
                    source_encoding: crate::CommittedSourceEncoding::CanonicalCoefficientTable,
                    expected_source_encoding: Some(crate::CommittedSourceEncoding::for_producer(
                        commitment.opening.opening_method,
                        extension_degree,
                        commitment.profile.inner.matrix.ring_dimension(),
                        group.profile.group.num_vars(),
                        true,
                    )),
                }
            })
            .collect();
        root_groups.push(OpeningExecutionGroup {
            opening_method: root_final.opening_method(),
            inner_commit_matrix: &root_final.inner().matrix,
            fold_challenge_config: root_final.fold_challenge_config(),
            source_encoding: root_final.source_encoding,
            expected_source_encoding: Some(crate::CommittedSourceEncoding::for_producer(
                root_final.opening_method(),
                extension_degree,
                root_final.d_a(),
                self.root.input_witness_len.trailing_zeros() as usize,
                true,
            )),
        });
        validate_level_opening_execution(0, extension_degree, &root_groups)?;
        for (index, step) in self.recursive_folds.iter().enumerate() {
            let witness = &step.params;
            let mut groups: Vec<OpeningExecutionGroup> = Vec::new();
            // An incoming setup prefix is group 0 in canonical order.
            if let Some(prefix) = &step.params.setup_prefix() {
                groups.push(OpeningExecutionGroup {
                    opening_method: prefix.opening.opening_method,
                    inner_commit_matrix: &prefix.profile.inner.matrix,
                    fold_challenge_config: prefix.opening.fold_challenge_config,
                    source_encoding: crate::CommittedSourceEncoding::CanonicalCoefficientTable,
                    expected_source_encoding: None,
                });
            }
            groups.push(OpeningExecutionGroup {
                opening_method: witness.opening_method(),
                inner_commit_matrix: &witness.inner().matrix,
                fold_challenge_config: witness.fold_challenge_config(),
                source_encoding: witness.source_encoding,
                expected_source_encoding: Some(crate::CommittedSourceEncoding::for_producer(
                    witness.opening_method(),
                    extension_degree,
                    witness.d_a(),
                    0,
                    false,
                )),
            });
            validate_level_opening_execution(index + 1, extension_degree, &groups)?;
        }
        Ok(())
    }
}

/// One group admitted by a fold, paired with the source encoding its producer
/// must have used.
///
/// Formerly a borrowed view over `&dyn LevelParamsLike`, needed because a fold's
/// final group and a precommitted group had different types. Rather than replace
/// the trait object with a group, this names the three things admission actually
/// reads, so a fold and a standalone group can each supply them from what they
/// already hold.
#[derive(Clone, Copy)]
struct OpeningExecutionGroup<'a> {
    /// What admission reads off the group itself.
    ///
    /// Carried directly rather than as a whole group, for the same reason
    /// `source_encoding` is: a fold's own new witness has no
    /// `PolynomialGroupLayout` of its own, so handing this check a group would
    /// mean inventing one. `final_group_scalar` is the natural way to invent it
    /// and it rejects any fold whose live-ring-element count times `d_a` is not
    /// a power of two — true of every multi-chunk fold, and never a condition
    /// this admission check meant to impose.
    opening_method: OpeningMethod,
    inner_commit_matrix: &'a crate::InnerCommitMatrixParams,
    fold_challenge_config: akita_challenges::SparseChallengeConfig,
    /// Encoding the committing fold actually used for this group.
    ///
    /// Carried explicitly rather than read from `params`: a group's own
    /// `source_encoding` accessor returns a hard-coded canonical value, because a
    /// precommitted group has nowhere to store one. Reading it from the group
    /// would make both tensor-projection rejections below unreachable.
    source_encoding: crate::CommittedSourceEncoding,
    expected_source_encoding: Option<crate::CommittedSourceEncoding>,
}

fn validate_level_opening_execution(
    absolute_level: usize,
    extension_degree: usize,
    groups: &[OpeningExecutionGroup<'_>],
) -> Result<(), AkitaError> {
    // Which group is `first` does not affect the accept/reject outcome. The
    // family taken from it is checked against *every* group below, so for a
    // fold that passes, all groups share that family and any element would give
    // the same answer; for a fold that fails, both orderings reject. That is why
    // moving the prefix to index 0 is behaviour-preserving here, and it settles
    // the ordering disagreement between this check and
    // `preceding_group_iter`, which already put the prefix first.
    let first = groups
        .first()
        .ok_or_else(|| AkitaError::InvalidSetup("nonterminal fold has no opening groups".into()))?;
    if first.opening_method.is_full_ring_scalar_extension() {
        let expected = FULL_RING_FOLD_CHALLENGE;
        for group in groups {
            group.opening_method.physical_coefficient_width(
                extension_degree,
                group.inner_commit_matrix.ring_dimension(),
            )?;
            if group.opening_method != OpeningMethod::FULL_RING_E2
                || group.source_encoding != CommittedSourceEncoding::CanonicalCoefficientTable
                || group.expected_source_encoding.is_some_and(|encoding| {
                    encoding != CommittedSourceEncoding::CanonicalCoefficientTable
                })
                || group.inner_commit_matrix.sis_modulus_profile()
                    != crate::SisModulusProfileId::Q128OffsetA7F7
                || group.fold_challenge_config != expected
                || !matches!(
                    group.inner_commit_matrix.security_route(),
                    InnerCommitSecurityRoute::Linf(_)
                )
            {
                return Err(AkitaError::InvalidSetup(
                    "full-ring opening group violates its method/source/shell contract".into(),
                ));
            }
            group.inner_commit_matrix.validate()?;
        }
        return Ok(());
    }
    let packing_family = matches!(
        first.opening_method,
        OpeningMethod::SubringCoefficientPacking { .. }
    );
    let packing_required = absolute_level <= 1;
    if packing_family != packing_required {
        let required = if packing_required {
            "subring coefficient packing"
        } else {
            "evaluation trace"
        };
        return Err(AkitaError::InvalidSetup(format!(
            "nonterminal level {absolute_level} requires {required}"
        )));
    }
    if groups.iter().any(|group| {
        group.opening_method.is_full_ring_scalar_extension()
            || matches!(
                group.opening_method,
                OpeningMethod::SubringCoefficientPacking { .. }
            ) != packing_family
    }) {
        return Err(AkitaError::InvalidSetup(
            "all groups consumed by one fold must use the same opening-method family".into(),
        ));
    }
    for group in groups {
        let opening_method = group.opening_method;
        match (opening_method, group.source_encoding) {
            (
                OpeningMethod::EvaluationTrace,
                crate::CommittedSourceEncoding::TensorSubfieldProjection {
                    extension_degree: encoded_degree,
                },
            ) if encoded_degree != extension_degree => {
                return Err(AkitaError::InvalidSetup(
                    "tensor source encoding does not match the protocol extension degree".into(),
                ));
            }
            (
                OpeningMethod::SubringCoefficientPacking { .. },
                crate::CommittedSourceEncoding::TensorSubfieldProjection { .. },
            ) => {
                return Err(AkitaError::InvalidSetup(
                    "coefficient packing requires the canonical coefficient source encoding".into(),
                ));
            }
            _ => {}
        }
        if group
            .expected_source_encoding
            .is_some_and(|expected| expected != group.source_encoding)
        {
            return Err(AkitaError::InvalidSetup(
                "committed source encoding does not match its producer geometry and opening method"
                    .into(),
            ));
        }
        let OpeningMethod::SubringCoefficientPacking {
            challenge_subring_dimension,
        } = opening_method
        else {
            continue;
        };
        if absolute_level > 1 {
            return Err(AkitaError::InvalidSetup(
                "subring coefficient packing is restricted to nonterminal levels 0 and 1".into(),
            ));
        }
        let expected = akita_challenges::SparseChallengeConfig::production_for_ring_dim(
            challenge_subring_dimension,
        )
        .ok_or_else(|| {
            AkitaError::InvalidSetup(
                "coefficient-packing challenge subring is not in the production ladder".into(),
            )
        })?;
        let matrix = group.inner_commit_matrix;
        if !matches!(matrix.security_route(), InnerCommitSecurityRoute::Linf(_)) {
            return Err(AkitaError::InvalidSetup(
                "coefficient packing requires the L-infinity A security route".into(),
            ));
        }
        if group.fold_challenge_config != expected {
            return Err(AkitaError::InvalidSetup(
                "coefficient packing requires its audited production challenge family".into(),
            ));
        }
        crate::SubringCoefficientPackingGeometry::try_new(
            extension_degree,
            matrix.ring_dimension(),
            challenge_subring_dimension,
        )?;
    }
    Ok(())
}

fn validate_stage2_successor_capacity(
    predecessor_name: &str,
    predecessor: &CommittedGroupParams,
    output_witness_len: usize,
    successor_ring_dimension: usize,
    successor_opening_num_vars: usize,
) -> Result<(), AkitaError> {
    // Stage 2 owns the predecessor-derived point. A successor may expose a
    // wider scheduled cube; preparation derives that wider representation by
    // zero-extension. The schedule must reject only points that do not fit.
    if successor_ring_dimension == 0 || !successor_ring_dimension.is_power_of_two() {
        return Err(AkitaError::InvalidSetup(format!(
            "{predecessor_name} successor ring dimension {successor_ring_dimension} is invalid"
        )));
    }
    let role_dims = predecessor.role_dims();
    let shared_d = role_dims.d_d();
    let mut relation_coefficient_block_len = role_dims.common_relation_coeff_count();
    if let OpeningMethod::SubringCoefficientPacking {
        challenge_subring_dimension,
    } = predecessor.opening_method()
    {
        relation_coefficient_block_len =
            relation_coefficient_block_len.min(challenge_subring_dimension);
    }
    for group in predecessor.preceding_group_iter() {
        let group_dims = group.role_dims(shared_d);
        relation_coefficient_block_len =
            relation_coefficient_block_len.min(group_dims.common_relation_coeff_count());
        if let OpeningMethod::SubringCoefficientPacking {
            challenge_subring_dimension,
        } = group.opening.opening_method
        {
            relation_coefficient_block_len =
                relation_coefficient_block_len.min(challenge_subring_dimension);
        }
    }
    let geometry = RelationAddressGeometry::new_with_coefficient_block(
        role_dims,
        relation_coefficient_block_len,
        successor_ring_dimension,
        output_witness_len,
    )?;
    let stage2_num_vars = geometry.relation_point_variable_count();
    if stage2_num_vars > successor_opening_num_vars {
        return Err(AkitaError::InvalidSetup(format!(
            "{predecessor_name} Stage 2 point has {stage2_num_vars} variables, exceeding \
             successor opening capacity {successor_opening_num_vars}"
        )));
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldScheduleEstimate {
    /// Maximum bytes used by independently encoded canonical native nonces.
    pub nonce_max_bytes: usize,
    pub estimated_root_direct_payload_bytes: usize,
    pub estimated_root_stage3_payload_bytes: usize,
    pub estimated_recursive_direct_payload_bytes: Vec<usize>,
    pub estimated_recursive_stage3_payload_bytes: Vec<usize>,
    pub estimated_terminal_direct_payload_bytes: usize,
    pub estimated_terminal_response_payload_bytes: usize,
    /// Maximum flat setup-matrix capacity required by the schedule.
    pub estimated_num_setup_field_elements: usize,
    /// Natural (unpadded) setup length at the first direct edge for setup-first
    /// schedule selection.
    pub first_direct_setup_field_len: Option<usize>,
    /// Number of recursive successors that consume an offloaded setup prefix.
    pub selected_offload_edges: usize,
}

impl FoldScheduleEstimate {
    pub fn estimated_direct_proof_payload_bytes(&self) -> Result<usize, AkitaError> {
        self.estimated_recursive_direct_payload_bytes
            .iter()
            .try_fold(self.estimated_root_direct_payload_bytes, |sum, value| {
                sum.checked_add(*value).ok_or_else(|| {
                    AkitaError::InvalidSetup("fold schedule estimate overflow".to_string())
                })
            })?
            .checked_add(self.estimated_terminal_direct_payload_bytes)
            .ok_or_else(|| AkitaError::InvalidSetup("fold schedule estimate overflow".to_string()))
    }

    pub fn estimated_stage3_payload_bytes(&self) -> Result<usize, AkitaError> {
        self.estimated_recursive_stage3_payload_bytes
            .iter()
            .try_fold(self.estimated_root_stage3_payload_bytes, |sum, value| {
                sum.checked_add(*value).ok_or_else(|| {
                    AkitaError::InvalidSetup("fold schedule estimate overflow".to_string())
                })
            })
    }

    pub fn estimated_proof_payload_bytes(&self) -> Result<usize, AkitaError> {
        self.estimated_direct_proof_payload_bytes()?
            .checked_add(self.estimated_stage3_payload_bytes()?)
            .and_then(|value| value.checked_add(self.nonce_max_bytes))
            .ok_or_else(|| AkitaError::InvalidSetup("fold schedule estimate overflow".to_string()))
    }
}

#[derive(Clone, Debug)]
pub struct PlannedFoldSchedule {
    pub schedule: FoldSchedule,
    pub estimate: FoldScheduleEstimate,
}

/// Witness length entering the root fold, in field elements.
pub fn root_input_witness_len(lp: &CommittedGroupParams) -> usize {
    lp.blocks()
        .live_blocks
        .checked_mul(lp.blocks().positions_per_block)
        .and_then(|len| len.checked_mul(lp.d_a()))
        .unwrap_or(0)
}

#[cfg(test)]
#[path = "schedule/internal_tests.rs"]
mod internal_tests;
