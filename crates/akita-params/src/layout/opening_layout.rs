//! Layout-only opening geometry shared by planning, sizing, and claims.
//!
//! Source arity describes the immutable producer profile, not the public query.
//! Registered original RAM planes retain source arity 16 while their root query
//! has 21 coordinates. Opening views are explicit consuming policy: decoded
//! constructors never infer a raw view from a point length. Ordered views and
//! query arities are bound by the v2 layout digest and opening lookup key.

use crate::descriptor_bytes::{digest_descriptor_bytes, push_usize, DescriptorDigest};
use akita_error::{checked, AkitaError};
use jolt_field::Field;

/// Per-group opening geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct PolynomialGroupLayout {
    num_vars: usize,
    num_polynomials: usize,
}

impl PolynomialGroupLayout {
    /// Build a per-group layout. Runtime callers should pair this with `validate`.
    pub const fn new(num_vars: usize, num_polynomials: usize) -> Self {
        Self {
            num_vars,
            num_polynomials,
        }
    }

    /// Scalar default: one polynomial at `num_vars`.
    pub const fn singleton(num_vars: usize) -> Self {
        Self::new(num_vars, 1)
    }

    /// Active variable count for this group.
    pub const fn num_vars(self) -> usize {
        self.num_vars
    }

    /// Number of polynomials in this group.
    pub const fn num_polynomials(self) -> usize {
        self.num_polynomials
    }

    /// Validate that the group carries at least one polynomial.
    pub fn validate(self) -> Result<(), AkitaError> {
        if self.num_polynomials == 0 {
            return Err(AkitaError::InvalidSetup(
                "opening group layouts must be nonempty".to_string(),
            ));
        }
        Ok(())
    }
}

/// Registered consuming functional; never a change to commitment geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SourceOpeningView {
    Decoded,
    /// Original RAM axes: coefficient[7], digit[5], position[5], block[4].
    OriginalRamPlanesV1,
}

impl SourceOpeningView {
    pub const fn tag(self) -> u8 {
        match self {
            Self::Decoded => 0,
            Self::OriginalRamPlanesV1 => 1,
        }
    }

    pub fn from_tag(tag: u8) -> Result<Self, AkitaError> {
        match tag {
            0 => Ok(Self::Decoded),
            1 => Ok(Self::OriginalRamPlanesV1),
            _ => Err(AkitaError::InvalidInput("unknown source opening view".into())),
        }
    }

    /// Check the separate producer declaration; equal profiles alone are not authority.
    pub fn validate_producer_contract(self, contract: crate::sis::CommittedSourceContract) -> Result<(), AkitaError> {
        if self == Self::OriginalRamPlanesV1
            && (contract.class() != crate::sis::CommittedSourceClass::OriginalBalancedDigitPlanes
                || contract.original_plane_geometry() != Some((5, 26))
                || contract.decomposition().log_commit_bound != 128
                || contract.decomposition().field_bits() != 128)
        {
            return Err(AkitaError::InvalidSetup("raw RAM view requires original balanced plane provenance".into()));
        }
        Ok(())
    }

    /// Admit the fixed numerical source geometry without granting producer provenance.
    pub fn validate_source_profile(self, profile: &crate::GroupCommitPhaseParams) -> Result<(), AkitaError> {
        OpeningGroupLayout::new(profile.group, self)?;
        if self == Self::Decoded {
            return Ok(());
        }
        if profile.version != crate::GroupCommitPhaseParams::VERSION
            || profile.blocks != crate::BlockGeometry::new(512, 32, 16)
            || profile.inner.matrix.ring_dimension() != 128
            || profile.inner.digits != crate::GadgetDigits::new(5, 26)
            || profile.outer.matrix.ring_dimension() != 128
            || profile.outer.digits != crate::GadgetDigits::new(5, 26)
            || profile.outer_slice_count != crate::CommitmentSliceCount::ONE
            || profile.inner.matrix.sis_modulus_profile() != crate::SisModulusProfileId::Q128OffsetA7F7
            || profile.outer.matrix.sis_modulus_profile() != crate::SisModulusProfileId::Q128OffsetA7F7
        {
            return Err(AkitaError::InvalidSetup("raw RAM view requires the registered original source geometry".into()));
        }
        Ok(())
    }
}

/// Immutable source shape paired with an explicitly registered query view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct OpeningGroupLayout {
    source: PolynomialGroupLayout,
    view: SourceOpeningView,
}

impl<'de> serde::Deserialize<'de> for OpeningGroupLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Wire {
            source: PolynomialGroupLayout,
            view: SourceOpeningView,
        }
        let wire = <Wire as serde::Deserialize>::deserialize(deserializer)?;
        Self::new(wire.source, wire.view).map_err(serde::de::Error::custom)
    }
}

impl OpeningGroupLayout {
    pub fn new(source: PolynomialGroupLayout, view: SourceOpeningView) -> Result<Self, AkitaError> {
        let layout = Self { source, view };
        layout.validate()?;
        Ok(layout)
    }

    pub const fn decoded(source: PolynomialGroupLayout) -> Self {
        Self { source, view: SourceOpeningView::Decoded }
    }

    pub const fn source(self) -> PolynomialGroupLayout {
        self.source
    }

    pub const fn view(self) -> SourceOpeningView {
        self.view
    }

    pub const fn point_num_vars(self) -> usize {
        match self.view {
            SourceOpeningView::Decoded => self.source.num_vars(),
            SourceOpeningView::OriginalRamPlanesV1 => 21,
        }
    }

    pub fn validate(self) -> Result<(), AkitaError> {
        self.source.validate()?;
        if self.view == SourceOpeningView::OriginalRamPlanesV1
            && self.source != PolynomialGroupLayout::singleton(16)
        {
            return Err(AkitaError::InvalidSetup("original RAM view requires source16/P1".into()));
        }
        Ok(())
    }
}

/// Batch structure without point values, evaluations, commitments, or routing values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpeningClaimsLayout {
    groups: Vec<PolynomialGroupLayout>,
    views: Vec<SourceOpeningView>,
}

impl OpeningClaimsLayout {
    /// Build a one-group layout for `num_total_polynomials` at `num_vars`.
    pub fn new(num_vars: usize, num_total_polynomials: usize) -> Result<Self, AkitaError> {
        Self::from_groups(vec![PolynomialGroupLayout::new(
            num_vars,
            num_total_polynomials,
        )])
    }

    /// Build a layout from group sizes, all sharing the same active variable count.
    #[cfg(any(test, feature = "test-support"))]
    pub fn from_group_sizes(
        num_vars: usize,
        polynomials_per_group: &[usize],
    ) -> Result<Self, AkitaError> {
        Self::from_groups(
            polynomials_per_group
                .iter()
                .map(|&num_polynomials| PolynomialGroupLayout::new(num_vars, num_polynomials))
                .collect(),
        )
    }

    /// Build a validated layout from per-group geometry.
    pub fn from_groups(groups: Vec<PolynomialGroupLayout>) -> Result<Self, AkitaError> {
        let views = vec![SourceOpeningView::Decoded; groups.len()];
        let layout = Self { groups, views };
        layout.check()?;
        Ok(layout)
    }

    /// Build ordered opening geometry without changing any source shape.
    pub fn from_opening_groups(groups: Vec<OpeningGroupLayout>) -> Result<Self, AkitaError> {
        let layout = Self {
            groups: groups.iter().map(|group| group.source()).collect(),
            views: groups.iter().map(|group| group.view()).collect(),
        };
        layout.check()?;
        Ok(layout)
    }

    /// Source and view vectors are in public commitment order.
    pub fn from_groups_and_views(
        groups: Vec<PolynomialGroupLayout>,
        views: Vec<SourceOpeningView>,
    ) -> Result<Self, AkitaError> {
        let layout = Self { groups, views };
        layout.check()?;
        Ok(layout)
    }

    pub fn group_view(&self, g: usize) -> Result<SourceOpeningView, AkitaError> {
        self.views.get(g).copied().ok_or(AkitaError::InvalidProof)
    }

    pub fn opening_group_layout(&self, g: usize) -> Result<OpeningGroupLayout, AkitaError> {
        OpeningGroupLayout::new(*self.group_layout(g)?, self.group_view(g)?)
    }

    pub fn group_point_num_vars(&self, g: usize) -> Result<usize, AkitaError> {
        Ok(self.opening_group_layout(g)?.point_num_vars())
    }

    pub fn views(&self) -> &[SourceOpeningView] {
        &self.views
    }

    pub fn max_point_num_vars(&self) -> usize {
        self.groups.iter().zip(&self.views)
            .map(|(&source, &view)| OpeningGroupLayout { source, view }.point_num_vars())
            .max().unwrap_or(0)
    }

    /// Raw batches use Lagrange semantics; decoded batches retain both bases.
    pub fn validate_basis(&self, basis: crate::BasisMode) -> Result<(), AkitaError> {
        self.check()?;
        if basis != crate::BasisMode::Lagrange
            && self.views.contains(&SourceOpeningView::OriginalRamPlanesV1)
        {
            return Err(AkitaError::InvalidInput("original RAM view requires Lagrange basis".into()));
        }
        Ok(())
    }

    /// Build a root-opening layout from precommitted groups plus the final/new group.
    pub fn from_root_groups(
        precommitteds: &[PolynomialGroupLayout],
        final_group: PolynomialGroupLayout,
    ) -> Result<Self, AkitaError> {
        let mut groups = Vec::with_capacity(precommitteds.len() + 1);
        groups.extend_from_slice(precommitteds);
        groups.push(final_group);
        Self::from_groups(groups)
    }

    /// Validate layout count consistency.
    pub fn check(&self) -> Result<(), AkitaError> {
        if self.groups.len() != self.views.len()
            || self.groups.is_empty() || self.checked_num_total_polynomials()? == 0 {
            return Err(AkitaError::InvalidProof);
        }
        for (&source, &view) in self.groups.iter().zip(&self.views) {
            OpeningGroupLayout::new(source, view)?;
        }
        Ok(())
    }

    /// Maximum active variable count across groups.
    pub fn max_num_vars(&self) -> usize {
        self.groups
            .iter()
            .map(|group| group.num_vars())
            .max()
            .unwrap_or(0)
    }

    /// Commitment groups in transcript order.
    pub fn groups(&self) -> &[PolynomialGroupLayout] {
        &self.groups
    }

    /// Number of commitment groups represented by the batch.
    pub fn num_groups(&self) -> usize {
        self.groups.len()
    }

    /// Total polynomials opened across all groups.
    pub fn num_total_polynomials(&self) -> usize {
        self.groups
            .iter()
            .map(|group| group.num_polynomials())
            .sum()
    }

    /// Whether transcript batching needs a sampled row coefficient challenge.
    pub fn requires_row_batch_challenge(&self) -> bool {
        self.num_total_polynomials() > 1
    }

    /// Collapse this batch into the single group shape used by extension
    /// opening reduction sizing.
    ///
    /// The opening point uses the maximum group-local arity, while the partial
    /// count uses the checked sum of polynomials across every group.
    pub fn aggregate_polynomial_group_layout(&self) -> Result<PolynomialGroupLayout, AkitaError> {
        self.check()?;
        if self.views.contains(&SourceOpeningView::OriginalRamPlanesV1) {
            return Err(AkitaError::InvalidSetup("raw RAM views cannot enter decoded EOR sizing".into()));
        }
        Ok(PolynomialGroupLayout::new(
            self.max_num_vars(),
            self.checked_num_total_polynomials()?,
        ))
    }

    fn checked_num_total_polynomials(&self) -> Result<usize, AkitaError> {
        checked::sum(self.groups.iter().map(|group| group.num_polynomials()))
            .ok_or(AkitaError::InvalidProof)
    }

    /// Borrow one group layout by index.
    pub fn group_layout(&self, g: usize) -> Result<&PolynomialGroupLayout, AkitaError> {
        self.groups.get(g).ok_or(AkitaError::InvalidProof)
    }

    /// Commitment-group index used as the final/new group for multi-group root schedules.
    pub fn root_final_group_index(&self) -> Result<usize, AkitaError> {
        self.check()?;
        self.groups
            .len()
            .checked_sub(1)
            .ok_or(AkitaError::InvalidProof)
    }

    /// Group processing order for multi-group root schedules: final/new group first.
    pub fn root_group_order(&self) -> Result<Vec<usize>, AkitaError> {
        let final_group_index = self.root_final_group_index()?;
        let mut order = Vec::with_capacity(self.num_groups());
        order.push(final_group_index);
        for group_index in 0..self.num_groups() {
            if group_index != final_group_index {
                order.push(group_index);
            }
        }
        Ok(order)
    }

    /// Final/new group layout for multi-group root schedule lookup.
    pub fn root_final_group_layout(&self) -> Result<PolynomialGroupLayout, AkitaError> {
        Ok(*self.group_layout(self.root_final_group_index()?)?)
    }

    /// Flat claim range covered by one commitment group.
    pub fn root_group_claim_range(
        &self,
        group_index: usize,
    ) -> Result<std::ops::Range<usize>, AkitaError> {
        self.check()?;
        if group_index >= self.groups.len() {
            return Err(AkitaError::InvalidProof);
        }
        let start = checked::sum(
            self.groups[..group_index]
                .iter()
                .map(|group| group.num_polynomials()),
        )
        .ok_or(AkitaError::InvalidProof)?;
        let end = start
            .checked_add(self.groups[group_index].num_polynomials())
            .ok_or(AkitaError::InvalidProof)?;
        Ok(start..end)
    }

    /// Digest layout-only opening geometry.
    pub fn opening_batch_digest(&self) -> DescriptorDigest {
        let mut bytes = Vec::new();
        push_usize(&mut bytes, self.num_groups());
        bytes.extend_from_slice(b"akita/opening-layout/v2");
        for (&group, &view) in self.groups.iter().zip(&self.views) {
            push_usize(&mut bytes, group.num_vars());
            push_usize(&mut bytes, group.num_polynomials());
            bytes.push(view.tag());
            push_usize(&mut bytes, OpeningGroupLayout { source: group, view }.point_num_vars());
        }
        digest_descriptor_bytes(&bytes)
    }

    /// Sum batched public opening claims under per-slot gamma coefficients.
    pub fn batched_eval_target<E>(
        &self,
        row_coefficients: &[E],
        openings: &[E],
    ) -> Result<E, AkitaError>
    where
        E: Field,
    {
        if row_coefficients.len() != self.num_total_polynomials() {
            return Err(AkitaError::InvalidSize {
                expected: self.num_total_polynomials(),
                actual: row_coefficients.len(),
            });
        }
        if openings.len() != self.num_total_polynomials() {
            return Err(AkitaError::InvalidSize {
                expected: self.num_total_polynomials(),
                actual: openings.len(),
            });
        }
        row_coefficients
            .iter()
            .zip(openings.iter())
            .try_fold(E::zero(), |acc, (&coefficient, &opening)| {
                Ok(acc + coefficient * opening)
            })
    }

    /// Scale flat row coefficients by one transparent reduction factor per
    /// opening group.
    ///
    /// The returned coefficients remain in canonical flat-claim order. This is
    /// the shared prover/verifier definition of the final grouped
    /// extension-opening relation.
    pub fn scale_row_coefficients_by_group<E>(
        &self,
        row_coefficients: &[E],
        group_factors: &[E],
    ) -> Result<Vec<E>, AkitaError>
    where
        E: Field,
    {
        if row_coefficients.len() != self.num_total_polynomials() {
            return Err(AkitaError::InvalidSize {
                expected: self.num_total_polynomials(),
                actual: row_coefficients.len(),
            });
        }
        if group_factors.len() != self.num_groups() {
            return Err(AkitaError::InvalidSize {
                expected: self.num_groups(),
                actual: group_factors.len(),
            });
        }
        let mut scaled = Vec::with_capacity(row_coefficients.len());
        for (group_index, &factor) in group_factors.iter().enumerate() {
            let range = self.root_group_claim_range(group_index)?;
            scaled.extend(
                row_coefficients
                    .get(range)
                    .ok_or(AkitaError::InvalidProof)?
                    .iter()
                    .map(|&coefficient| coefficient * factor),
            );
        }
        Ok(scaled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registered_view_separates_query_arity_without_changing_source_counts() {
        let sources = vec![PolynomialGroupLayout::singleton(16), PolynomialGroupLayout::new(16, 56)];
        let decoded = OpeningClaimsLayout::from_groups(sources.clone()).unwrap();
        let raw = OpeningClaimsLayout::from_groups_and_views(sources, vec![
            SourceOpeningView::OriginalRamPlanesV1, SourceOpeningView::Decoded,
        ]).unwrap();
        assert_eq!(raw.groups(), decoded.groups());
        assert_eq!(raw.max_num_vars(), 16);
        assert_eq!(raw.max_point_num_vars(), 21);
        assert_eq!(raw.group_point_num_vars(0).unwrap(), 21);
        assert_eq!(raw.group_point_num_vars(1).unwrap(), 16);
        assert_eq!(raw.root_group_claim_range(0).unwrap(), 0..1);
        assert_eq!(raw.root_group_claim_range(1).unwrap(), 1..57);
        assert_ne!(raw.opening_batch_digest(), decoded.opening_batch_digest());
        assert!(raw.validate_basis(crate::BasisMode::Monomial).is_err());
        assert!(decoded.validate_basis(crate::BasisMode::Monomial).is_ok());
        assert!(raw.aggregate_polynomial_group_layout().is_err());
    }

    #[test]
    fn registered_view_rejects_unknown_tags_shapes_and_order_counts() {
        assert!(SourceOpeningView::from_tag(2).is_err());
        for source in [PolynomialGroupLayout::singleton(21), PolynomialGroupLayout::new(16, 26)] {
            assert!(OpeningGroupLayout::new(source, SourceOpeningView::OriginalRamPlanesV1).is_err());
        }
        assert!(OpeningClaimsLayout::from_groups_and_views(
            vec![PolynomialGroupLayout::singleton(16)], vec![],
        ).is_err());
    }

    #[test]
    fn original_plane_authority_is_not_inferred_from_dense_magnitude() {
        use crate::sis::{CommittedSourceClass, CommittedSourceContract};
        let decomposition = crate::DecompositionParams {
            log_basis: 5, log_commit_bound: 128, log_open_bound: Some(128),
        };
        let original = CommittedSourceContract::try_new(
            CommittedSourceClass::OriginalBalancedDigitPlanes, decomposition,
        ).unwrap();
        let canonical = CommittedSourceContract::try_new(
            CommittedSourceClass::BalancedSignedDigit, decomposition,
        ).unwrap();
        assert!(SourceOpeningView::OriginalRamPlanesV1.validate_producer_contract(original).is_ok());
        assert!(SourceOpeningView::OriginalRamPlanesV1.validate_producer_contract(canonical).is_err());
        assert!(SourceOpeningView::Decoded.validate_producer_contract(canonical).is_ok());
    }

    #[test]
    fn raw_source_profile_and_plan_reject_unregistered_digits_methods_and_degrees() {
        let profile = crate::test_fixtures::original_ram_profile();
        profile.validate_frozen_precommit(128).unwrap();
        SourceOpeningView::OriginalRamPlanesV1.validate_source_profile(&profile).unwrap();
        let plan = crate::GroupOpeningPlan {
            source_view: SourceOpeningView::OriginalRamPlanesV1,
            opening_method: crate::OpeningMethod::SubringCoefficientPacking { challenge_subring_dimension: 128 },
            fold_challenge_config: akita_challenges::SparseChallengeConfig::production_for_ring_dim(128).unwrap(),
            log_basis_open: 5,
            num_digits_open: 26,
            num_digits_fold: 3,
        };
        plan.validate_source_view(&profile, 1).unwrap();
        assert!(plan.validate_source_view(&profile, 2).is_err());
        let mut wrong = plan;
        wrong.opening_method = crate::OpeningMethod::EvaluationTrace;
        assert!(wrong.validate_source_view(&profile, 1).is_err());
        let mut wrong = plan;
        wrong.opening_method = crate::OpeningMethod::SubringCoefficientPacking { challenge_subring_dimension: 64 };
        assert!(wrong.validate_source_view(&profile, 1).is_err());
        let mut wrong = profile;
        wrong.inner.digits.num_digits = 25;
        assert!(plan.validate_source_view(&wrong, 1).is_err());
        let mut wrong = profile;
        wrong.inner.digits.log_basis = 4;
        assert!(plan.validate_source_view(&wrong, 1).is_err());
        let mut wrong = profile;
        wrong.group = PolynomialGroupLayout::singleton(21);
        assert!(plan.validate_source_view(&wrong, 1).is_err());
        let mut wrong = profile;
        wrong.blocks = crate::BlockGeometry::new(512, 16, 32);
        assert!(plan.validate_source_view(&wrong, 1).is_err());
        let decoded = crate::GroupOpeningPlan { source_view: SourceOpeningView::Decoded, ..plan };
        assert_ne!(plan.canonical_descriptor_bytes(), decoded.canonical_descriptor_bytes());
        let mut raw = crate::CommittedGroupParams::params_only(
            crate::SisModulusProfileId::Q128OffsetA7F7,
            128, 5, 5, 5, 3, plan.fold_challenge_config,
        );
        raw.own_group_mut().profile = profile;
        raw.own_group_mut().opening = plan;
        let raw = raw.with_decomp(32, 512, 26, 26, 26).unwrap();
        let mut decoded = raw.clone();
        decoded.own_group_mut().opening.source_view = SourceOpeningView::Decoded;
        let copied = decoded.with_layout(&raw).unwrap();
        assert_eq!(copied.source_view(), SourceOpeningView::OriginalRamPlanesV1);
        assert_eq!(copied.own_group().profile, profile);
        assert!(copied.with_decomp(16, 512, 26, 26, 26).is_err());
    }
}
