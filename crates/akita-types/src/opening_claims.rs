//! Public opening claims and layout-only opening geometry.
//!
//! Public query arity and producer source arity are deliberately distinct.
//! Original RAM plane claims carry an explicit registered root view through
//! owned, borrowed, and commitment-mapped representations. Setup, suffix, and
//! terminal constructors remain decoded; point length alone grants no raw
//! authority.

use crate::proof::scheme::OpeningPoints;
use crate::proof::setup::AkitaSetupDescriptor;
use crate::CommittedGroup;
use akita_error::{checked, AkitaError};
use akita_params::layout::opening_layout::{OpeningClaimsLayout, OpeningGroupLayout, PolynomialGroupLayout, SourceOpeningView};
use akita_params::{GrindingSite, OpeningScheduleSelection};
use jolt_field::{CanonicalEncoding, ExtField, Field};

/// Public claims and commitment payload for one polynomial group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolynomialGroupClaims<'a, F: Clone, C = ()> {
    point: OpeningPoints<'a, F>,
    view: SourceOpeningView,
    evaluations: Vec<F>,
    commitment: C,
}

impl<'a, F: Clone, C> PolynomialGroupClaims<'a, F, C> {
    /// Build one group of public claims.
    pub fn new(
        point: impl Into<OpeningPoints<'a, F>>,
        evaluations: Vec<F>,
        commitment: C,
    ) -> Result<Self, AkitaError> {
        if evaluations.is_empty() {
            return Err(AkitaError::InvalidInput(
                "opening claim groups must be nonempty".to_string(),
            ));
        }
        Ok(Self {
            point: point.into(),
            view: SourceOpeningView::Decoded,
            evaluations,
            commitment,
        })
    }

    /// One scalar on the fixed original RAM coefficient/digit/position/block grid.
    pub fn original_ram_planes_v1(
        point: impl Into<OpeningPoints<'a, F>>,
        evaluation: F,
        commitment: C,
    ) -> Result<Self, AkitaError> {
        let point = point.into();
        if point.len() != 21 {
            return Err(AkitaError::InvalidPointDimension { expected: 21, actual: point.len() });
        }
        Ok(Self {
            point,
            view: SourceOpeningView::OriginalRamPlanesV1,
            evaluations: vec![evaluation],
            commitment,
        })
    }

    pub const fn source_view(&self) -> SourceOpeningView {
        self.view
    }

    pub fn opening_layout(&self) -> Result<OpeningGroupLayout, AkitaError> {
        let source = match self.view {
            SourceOpeningView::Decoded => PolynomialGroupLayout::new(self.point.len(), self.evaluations.len()),
            SourceOpeningView::OriginalRamPlanesV1 => PolynomialGroupLayout::singleton(16),
        };
        OpeningGroupLayout::new(source, self.view)
    }

    /// Transform the payload without losing query authority or copying point data.
    pub fn map_commitment<D>(self, map: impl FnOnce(C) -> D) -> PolynomialGroupClaims<'a, F, D> {
        PolynomialGroupClaims {
            point: self.point,
            view: self.view,
            evaluations: self.evaluations,
            commitment: map(self.commitment),
        }
    }

    /// Complete opening point owned by this group.
    pub fn point(&self) -> &[F] {
        self.point.as_ref()
    }

    /// Number of variables in this group's opening point.
    pub fn num_vars(&self) -> usize {
        self.point.len()
    }

    /// Query arity; unlike `source_num_vars`, this is 21 for raw RAM.
    pub fn point_num_vars(&self) -> usize {
        self.point.len()
    }

    /// Immutable logical source arity, not the raw query-grid dimension.
    pub fn source_num_vars(&self) -> usize {
        match self.view {
            SourceOpeningView::Decoded => self.point.len(),
            SourceOpeningView::OriginalRamPlanesV1 => 16,
        }
    }

    /// Claimed evaluations, one per committed polynomial.
    pub fn evaluations(&self) -> &[F] {
        &self.evaluations
    }

    /// Group commitment.
    pub fn commitment(&self) -> &C {
        &self.commitment
    }

    /// Number of evaluations in this group.
    pub fn num_evaluations(&self) -> usize {
        self.evaluations.len()
    }
}

/// Public opening claims: polynomial groups in transcript order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpeningClaims<'a, F: Clone, C = ()> {
    groups: Vec<PolynomialGroupClaims<'a, F, C>>,
}

/// Public opening statement bound to one exact verifier-approved schedule row.
///
/// The schedule selection is batch-level. Individual commitments remain
/// reusable and carry only their exact algebraic profiles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupBatchStatement<'a, E: Clone, F: Field> {
    selection: OpeningScheduleSelection,
    claims: OpeningClaims<'a, E, &'a CommittedGroup<F>>,
}

impl<'a, E: Clone, F: Field> GroupBatchStatement<'a, E, F> {
    /// Bind ordered self-describing claims to an approved schedule row.
    ///
    /// # Errors
    ///
    /// Returns an error when the claim set is empty or structurally malformed.
    pub fn new(
        selection: OpeningScheduleSelection,
        claims: OpeningClaims<'a, E, &'a CommittedGroup<F>>,
    ) -> Result<Self, AkitaError> {
        claims.check()?;
        Ok(Self { selection, claims })
    }

    /// Exact catalog and row identity selected for this batch.
    pub const fn selection(&self) -> OpeningScheduleSelection {
        self.selection
    }

    /// Ordered public opening claims.
    pub fn claims(&self) -> &OpeningClaims<'a, E, &'a CommittedGroup<F>> {
        &self.claims
    }

    /// Consume the statement into its ordered public claims.
    pub fn into_claims(self) -> OpeningClaims<'a, E, &'a CommittedGroup<F>> {
        self.claims
    }
}

impl<'a, E: Clone, F: Field> OpeningClaims<'a, E, &'a CommittedGroup<F>> {
    /// Layout reconstructed from the frozen commitment profiles.
    pub fn committed_layout(&self) -> Result<OpeningClaimsLayout, AkitaError> {
        self.check()?;
        let mut groups = Vec::with_capacity(self.groups.len());
        for group in &self.groups {
            let declared = group.commitment.profile().group;
            group.view.validate_source_profile(group.commitment.profile())?;
            let opening = OpeningGroupLayout::new(declared, group.view)?;
            if opening.point_num_vars() != group.point.len()
                || declared.num_polynomials() != group.evaluations.len()
            {
                return Err(AkitaError::InvalidInput(
                    "claim group shape does not match its commitment profile".to_string(),
                ));
            }
            groups.push(opening);
        }
        OpeningClaimsLayout::from_opening_groups(groups)
    }
}

impl<'a, E: Clone, F: Field> OpeningClaims<'a, E, CommittedGroup<F>> {
    /// Layout reconstructed from the frozen commitment profiles.
    pub fn committed_layout(&self) -> Result<OpeningClaimsLayout, AkitaError> {
        self.check()?;
        let mut groups = Vec::with_capacity(self.groups.len());
        for group in &self.groups {
            let declared = group.commitment.profile().group;
            group.view.validate_source_profile(group.commitment.profile())?;
            let opening = OpeningGroupLayout::new(declared, group.view)?;
            if opening.point_num_vars() != group.point.len()
                || declared.num_polynomials() != group.evaluations.len()
            {
                return Err(AkitaError::InvalidInput(
                    "claim group shape does not match its commitment profile".to_string(),
                ));
            }
            groups.push(opening);
        }
        OpeningClaimsLayout::from_opening_groups(groups)
    }
}

impl<'a, F: Clone, C> OpeningClaims<'a, F, C> {
    /// Build public claims from ordered groups.
    pub fn from_groups(groups: Vec<PolynomialGroupClaims<'a, F, C>>) -> Result<Self, AkitaError> {
        let claims = Self { groups };
        claims.check()?;
        Ok(claims)
    }

    /// Validate group and count consistency.
    pub fn check(&self) -> Result<(), AkitaError> {
        if self.groups.is_empty() || self.checked_num_total_polynomials()? == 0 {
            return Err(AkitaError::InvalidProof);
        }
        for group in &self.groups {
            if group.evaluations.is_empty() {
                return Err(AkitaError::InvalidProof);
            }
            if group.view == SourceOpeningView::OriginalRamPlanesV1
                && (group.point.len() != 21 || group.evaluations.len() != 1)
            {
                return Err(AkitaError::InvalidProof);
            }
        }
        Ok(())
    }

    /// Validate consistency plus public capacity against the setup limits.
    pub fn validate(&self, seed: &AkitaSetupDescriptor) -> Result<(), AkitaError> {
        self.check()?;
        let max_num_vars = self.layout()?.max_point_num_vars();
        if max_num_vars > seed.max_num_vars {
            return Err(AkitaError::InvalidPointDimension {
                expected: seed.max_num_vars,
                actual: max_num_vars,
            });
        }
        let num_polynomials = self.checked_num_total_polynomials()?;
        if num_polynomials > seed.max_num_batched_polys {
            return Err(AkitaError::InvalidSize {
                expected: seed.max_num_batched_polys,
                actual: num_polynomials,
            });
        }
        Ok(())
    }

    /// Number of polynomial groups.
    pub fn num_groups(&self) -> usize {
        self.groups.len()
    }

    fn checked_num_total_polynomials(&self) -> Result<usize, AkitaError> {
        checked::sum(
            self.groups
                .iter()
                .map(PolynomialGroupClaims::num_evaluations),
        )
        .ok_or(AkitaError::InvalidProof)
    }

    /// Borrow one group's evaluations.
    pub fn group_evaluations(&self, g: usize) -> Result<&[F], AkitaError> {
        self.groups
            .get(g)
            .map(PolynomialGroupClaims::evaluations)
            .ok_or(AkitaError::InvalidProof)
    }

    /// Borrow one group's complete opening point.
    pub fn group_point(&self, g: usize) -> Result<&[F], AkitaError> {
        self.groups
            .get(g)
            .map(PolynomialGroupClaims::point)
            .ok_or(AkitaError::InvalidProof)
    }

    pub fn group_source_view(&self, g: usize) -> Result<SourceOpeningView, AkitaError> {
        self.groups.get(g).map(PolynomialGroupClaims::source_view).ok_or(AkitaError::InvalidProof)
    }

    /// Borrow one group's commitment.
    pub fn group_commitment(&self, g: usize) -> Result<&C, AkitaError> {
        self.groups
            .get(g)
            .map(PolynomialGroupClaims::commitment)
            .ok_or(AkitaError::InvalidProof)
    }

    /// Commitment groups in transcript order.
    pub fn groups(&self) -> &[PolynomialGroupClaims<'a, F, C>] {
        &self.groups
    }

    /// Structural view for setup, planner, and config code.
    pub fn layout(&self) -> Result<OpeningClaimsLayout, AkitaError> {
        self.check()?;
        OpeningClaimsLayout::from_opening_groups(
            self.groups.iter().map(PolynomialGroupClaims::opening_layout)
                .collect::<Result<Vec<_>, _>>()?,
        )
    }
}

impl<'a, F: Clone, C> OpeningClaims<'a, F, C> {
    /// Consume a batch while retaining each group's source-opening view.
    pub fn map_groups<D>(
        self,
        mut map: impl FnMut(C) -> D,
    ) -> Result<OpeningClaims<'a, F, D>, AkitaError> {
        OpeningClaims::from_groups(self.groups.into_iter()
            .map(|group| group.map_commitment(&mut map)).collect())
    }
}

impl<'a, F: Clone, C> OpeningClaims<'a, F, C> {
    /// Claimed openings flattened in canonical claim order.
    pub fn flat_evaluations(&self) -> Vec<F> {
        self.groups
            .iter()
            .flat_map(|group| group.evaluations.iter().cloned())
            .collect()
    }
}

/// Apply the scheduled native work and draw one context-bound coefficient
/// per opening claim.
pub fn row_coefficients<F, L, G>(
    layout: &OpeningClaimsLayout,
    site: GrindingSite,
    grinding: &mut G,
) -> Result<Vec<L>, AkitaError>
where
    F: Field + CanonicalEncoding,
    L: ExtField<F>,
    G: crate::GrindingReplay,
{
    layout.check()?;
    if !layout.requires_row_batch_challenge() {
        return Ok(vec![L::one()]);
    }
    grinding.grinded_ext_challenges::<F, L>(site, layout.num_total_polynomials())
}

/// Canonical complete original endpoint binding for inherited indexed contexts.
/// Profiles remain producer-only; views and all public point/value/payload
/// coefficients are framed separately in commitment order.
pub fn encode_opening_endpoint_binding<F, E, C>(
    instance_descriptor: &[u8],
    profiles: &akita_params::CommittedGroupBatchProfile,
    claims: &OpeningClaims<'_, E, C>,
    commitment: impl for<'c> Fn(&'c C) -> &'c crate::Commitment<F>,
) -> Result<Vec<u8>, AkitaError>
where
    F: Field + CanonicalEncoding,
    E: ExtField<F>,
{
    claims.check()?;
    profiles.validate(F::MODULUS_BITS)?;
    if profiles.precommitteds.len().checked_add(1) != Some(claims.num_groups()) {
        return Err(AkitaError::InvalidProof);
    }
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"akita/opening-endpoint/v8");
    append_endpoint_length(&mut bytes, instance_descriptor.len())?;
    bytes.extend_from_slice(instance_descriptor);
    append_endpoint_length(&mut bytes, E::DEGREE)?;
    append_endpoint_length(&mut bytes, claims.num_groups())?;
    for (profile, group) in profiles.precommitteds.iter()
        .chain(core::iter::once(&profiles.final_group)).zip(claims.groups())
    {
        group.source_view().validate_source_profile(profile)?;
        let opening = OpeningGroupLayout::new(profile.group, group.source_view())?;
        if opening.point_num_vars() != group.point_num_vars()
            || profile.group.num_polynomials() != group.num_evaluations()
        {
            return Err(AkitaError::InvalidProof);
        }
        profile.append_descriptor_bytes(&mut bytes);
        bytes.push(group.source_view().tag());
        append_endpoint_length(&mut bytes, group.point_num_vars())?;
        for &value in group.point() {
            for limb in 0..E::DEGREE {
                append_endpoint_field(&mut bytes, value.base_coefficient(limb))?;
            }
        }
        append_endpoint_length(&mut bytes, group.num_evaluations())?;
        for &value in group.evaluations() {
            for limb in 0..E::DEGREE {
                append_endpoint_field(&mut bytes, value.base_coefficient(limb))?;
            }
        }
        let coefficients = commitment(group.commitment()).rows().coeffs();
        let source_coefficients = profile.outer_slice_count.complete_source_coefficients(
            profile.outer.matrix.output_rank(), profile.outer.matrix.ring_dimension(),
        )?;
        let expected_coefficients = akita_params::CompressionChainPlan::for_complete_source(
            profile.outer.matrix.sis_modulus_profile(), source_coefficients,
        )?.terminal_coefficients();
        if coefficients.len() != expected_coefficients {
            return Err(AkitaError::InvalidSize { expected: expected_coefficients, actual: coefficients.len() });
        }
        append_endpoint_length(&mut bytes, coefficients.len())?;
        for &value in coefficients {
            append_endpoint_field(&mut bytes, value)?;
        }
    }
    Ok(bytes)
}

fn append_endpoint_length(bytes: &mut Vec<u8>, value: usize) -> Result<(), AkitaError> {
    let value = u64::try_from(value).map_err(|_| AkitaError::InvalidProof)?;
    bytes.extend_from_slice(&value.to_le_bytes());
    Ok(())
}

fn append_endpoint_field<F: CanonicalEncoding>(bytes: &mut Vec<u8>, value: F) -> Result<(), AkitaError> {
    let start = bytes.len();
    let end = start.checked_add(F::NUM_BYTES).ok_or(AkitaError::InvalidProof)?;
    bytes.try_reserve(F::NUM_BYTES).map_err(|_| AkitaError::InvalidProof)?;
    bytes.resize(end, 0);
    value.to_bytes_le(bytes.get_mut(start..end).ok_or(AkitaError::InvalidProof)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use jolt_field::{One, Prime128OffsetA7F7, Ring, Zero};

    type F = Prime128OffsetA7F7;

    #[test]
    fn groups_own_independent_points() {
        let first = vec![F::from_u64(1), F::from_u64(2)];
        let second = vec![F::from_u64(3), F::from_u64(4), F::from_u64(5)];
        let claims = OpeningClaims::from_groups(vec![
            PolynomialGroupClaims::new(first.clone(), vec![F::zero()], ()).expect("first group"),
            PolynomialGroupClaims::new(second.clone(), vec![F::zero()], ()).expect("second group"),
        ])
        .expect("claims");

        assert_eq!(claims.group_point(0).expect("first point"), first);
        assert_eq!(claims.group_point(1).expect("second point"), second);
        assert_eq!(claims.layout().expect("layout").max_num_vars(), 3);
    }

    #[test]
    fn layout_digest_binds_group_arity_count_and_order() {
        let baseline = OpeningClaimsLayout::from_groups(vec![
            PolynomialGroupLayout::new(2, 1),
            PolynomialGroupLayout::new(3, 2),
        ])
        .expect("baseline");
        let changed_arity = OpeningClaimsLayout::from_groups(vec![
            PolynomialGroupLayout::new(1, 1),
            PolynomialGroupLayout::new(3, 2),
        ])
        .expect("changed arity");
        let changed_count = OpeningClaimsLayout::from_groups(vec![
            PolynomialGroupLayout::new(2, 2),
            PolynomialGroupLayout::new(3, 1),
        ])
        .expect("changed count");
        let reversed = OpeningClaimsLayout::from_groups(vec![
            PolynomialGroupLayout::new(3, 2),
            PolynomialGroupLayout::new(2, 1),
        ])
        .expect("reversed");

        let digest = baseline.opening_batch_digest();
        assert_ne!(digest, changed_arity.opening_batch_digest());
        assert_ne!(digest, changed_count.opening_batch_digest());
        assert_ne!(digest, reversed.opening_batch_digest());
    }

    #[test]
    fn root_layout_helpers_preserve_group_order_and_lengths() {
        let layout = OpeningClaimsLayout::from_groups(vec![
            PolynomialGroupLayout::new(2, 1),
            PolynomialGroupLayout::new(3, 2),
            PolynomialGroupLayout::new(4, 1),
        ])
        .expect("multi-group layout");

        assert_eq!(layout.root_final_group_index().expect("final index"), 2);
        assert_eq!(
            layout.root_final_group_layout().expect("final layout"),
            PolynomialGroupLayout::new(4, 1)
        );
        assert_eq!(
            layout.root_group_order().expect("group order"),
            vec![2, 0, 1]
        );
        assert_eq!(layout.root_group_claim_range(0).expect("range"), 0..1);
        assert_eq!(layout.root_group_claim_range(1).expect("range"), 1..3);
        assert_eq!(layout.root_group_claim_range(2).expect("range"), 3..4);
    }

    #[test]
    fn from_root_groups_appends_final_group_after_precommitted_groups() {
        let precommitteds = [
            PolynomialGroupLayout::new(2, 1),
            PolynomialGroupLayout::new(3, 2),
        ];
        let final_group = PolynomialGroupLayout::new(4, 3);
        let layout = OpeningClaimsLayout::from_root_groups(&precommitteds, final_group)
            .expect("root layout");

        assert_eq!(
            layout.groups(),
            &[
                PolynomialGroupLayout::new(2, 1),
                PolynomialGroupLayout::new(3, 2),
                PolynomialGroupLayout::new(4, 3),
            ]
        );
        assert_eq!(
            layout.root_final_group_layout().expect("final group"),
            final_group
        );
    }

    #[test]
    fn aggregate_opening_layout_preserves_grouped_claim_count_for_eor() {
        let precommitteds = [PolynomialGroupLayout::new(10, 2)];
        let final_group = PolynomialGroupLayout::new(8, 1);
        let aggregate = OpeningClaimsLayout::from_root_groups(&precommitteds, final_group)
            .expect("root layout")
            .aggregate_polynomial_group_layout()
            .expect("aggregate layout");

        assert_eq!(aggregate, PolynomialGroupLayout::new(10, 3));
        let final_only_bytes =
            akita_params::extension_opening_reduction_level_bytes(128, 4, final_group)
                .expect("final-only EOR bytes");
        let aggregate_bytes =
            akita_params::extension_opening_reduction_level_bytes(128, 4, aggregate)
                .expect("aggregate EOR bytes");
        assert!(final_only_bytes > 0);
        let extra_partial_bytes = 2 * 4 * akita_params::field_bytes(128);
        let extra_round_bytes =
            2 * akita_params::EXTENSION_OPENING_REDUCTION_DEGREE * akita_params::field_bytes(128);
        let extra_terminal_claim_bytes = 2 * akita_params::field_bytes(128);
        assert_eq!(
            aggregate_bytes - final_only_bytes,
            extra_partial_bytes + extra_round_bytes + extra_terminal_claim_bytes
        );
    }

    #[test]
    fn raw_claims_preserve_view_when_copying_and_mapping_payloads() {
        let raw = PolynomialGroupClaims::original_ram_planes_v1(vec![0u64; 21], 7, 11).unwrap();
        assert_eq!(raw.source_num_vars(), 16);
        assert_eq!(raw.point_num_vars(), 21);
        let batch = OpeningClaims::from_groups(vec![raw]).unwrap();
        let copied = batch.clone();
        let mapped = batch.map_groups(|payload| payload.to_string()).unwrap();
        assert_eq!(copied.group_source_view(0).unwrap(), SourceOpeningView::OriginalRamPlanesV1);
        assert_eq!(mapped.group_source_view(0).unwrap(), SourceOpeningView::OriginalRamPlanesV1);
        assert_eq!(mapped.layout().unwrap().group_layout(0).unwrap(), &PolynomialGroupLayout::singleton(16));
        assert_eq!(mapped.layout().unwrap().group_point_num_vars(0).unwrap(), 21);
        assert!(PolynomialGroupClaims::original_ram_planes_v1(vec![0u64; 16], 7, ()).is_err());
        let decoded = PolynomialGroupClaims::new(vec![0u64; 21], vec![7], ()).unwrap();
        assert_eq!(decoded.source_view(), SourceOpeningView::Decoded);
        assert_eq!(decoded.source_num_vars(), 21);
    }

    #[test]
    fn endpoint_binding_preserves_original_profiles_and_authenticates_every_public_value() {
        let profile = akita_params::test_fixtures::original_ram_profile();
        profile.validate_frozen_precommit(128).unwrap();
        let profiles = akita_params::CommittedGroupBatchProfile::from_profiles(vec![profile]).unwrap();
        let group = crate::CommittedGroup::new(profile, crate::Commitment::new(
            crate::RingVec::from_coeffs(vec![F::zero(); 8]),
        ));
        let raw = OpeningClaims::from_groups(vec![
            PolynomialGroupClaims::original_ram_planes_v1(vec![F::zero(); 21], F::zero(), group.clone()).unwrap(),
        ]).unwrap();
        let stripped = raw.clone().map_groups(|group| group.commitment).unwrap();
        let encoded = encode_opening_endpoint_binding(b"instance", &profiles, &raw, |g| g.commitment()).unwrap();
        assert_eq!(encoded, encode_opening_endpoint_binding(b"instance", &profiles, &stripped, |c| c).unwrap());
        let mut changed = raw.clone();
        changed.groups[0].evaluations[0] = F::one();
        assert_ne!(encoded, encode_opening_endpoint_binding(b"instance", &profiles, &changed, |g| g.commitment()).unwrap());
        assert_ne!(encoded, encode_opening_endpoint_binding(b"another instance", &profiles, &raw, |g| g.commitment()).unwrap());
        for axis in [0, 7, 12, 20] {
            let mut changed = raw.clone();
            let mut point = vec![F::zero(); 21];
            point[axis] = F::one();
            changed.groups[0].point = point.into();
            assert_ne!(encoded, encode_opening_endpoint_binding(b"instance", &profiles, &changed, |g| g.commitment()).unwrap());
        }
        let mut changed = raw.clone();
        let mut coefficients = vec![F::zero(); 8];
        coefficients[7] = F::one();
        changed.groups[0].commitment.commitment = crate::Commitment::new(crate::RingVec::from_coeffs(coefficients));
        assert_ne!(encoded, encode_opening_endpoint_binding(b"instance", &profiles, &changed, |g| g.commitment()).unwrap());
        changed.groups[0].commitment.commitment = crate::Commitment::new(crate::RingVec::from_coeffs(vec![F::zero(); 7]));
        assert!(encode_opening_endpoint_binding(b"instance", &profiles, &changed, |g| g.commitment()).is_err());
        let mut changed = raw.clone();
        changed.groups[0].commitment.profile.inner.digits.num_digits = 25;
        assert!(changed.committed_layout().is_err());
        let decoded = OpeningClaims::from_groups(vec![
            PolynomialGroupClaims::new(vec![F::zero(); 16], vec![F::zero()], group).unwrap(),
        ]).unwrap();
        assert_ne!(encoded, encode_opening_endpoint_binding(b"instance", &profiles, &decoded, |g| g.commitment()).unwrap());
        assert_eq!(raw.committed_layout().unwrap().groups(), decoded.committed_layout().unwrap().groups());
    }
}
