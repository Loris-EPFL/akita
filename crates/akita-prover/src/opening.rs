//! Public claims paired with immutable reusable commitment handles.
use crate::backend::CommitmentHandleMetadata;
use akita_config::{CommitmentConfig, TrustedScheduleCatalog};
use akita_error::AkitaError;
use akita_params::{
    CommittedGroupBatchProfile, CommittedGroupParams, OpeningClaimsLayout, OpeningScheduleSelection,
};
use akita_types::GrindingReplay;
use akita_types::{Commitment, CommittedGroup, OpeningClaims, PolynomialGroupClaims};
use jolt_field::{CanonicalEncoding, ExtField, Field};

pub struct SelectedProverOpeningData<'a, E: Clone, H, F: Field> {
    selection: OpeningScheduleSelection,
    opening_data: ProverOpeningData<'a, E, H, F>,
}
impl<'a, E: Clone, H: CommitmentHandleMetadata, F: Field> SelectedProverOpeningData<'a, E, H, F> {
    pub fn from_committed_claims<Cfg: CommitmentConfig<Field = F, ExtField = E>>(
        claims: OpeningClaims<'a, E, CommittedGroup<F>>,
        handles: Vec<H>,
        schedules: &TrustedScheduleCatalog<Cfg>,
    ) -> Result<Self, AkitaError> {
        if claims.num_groups() != handles.len() {
            return Err(AkitaError::InvalidInput(
                "claim group count differs from commitment handle count".into(),
            ));
        }
        let profile = CommittedGroupBatchProfile::from_profiles(
            claims
                .groups()
                .iter()
                .map(|group| *group.commitment().profile())
                .collect(),
        )?;
        let opening_layout = claims.committed_layout()?;
        let selection = schedules
            .resolve_profiles(&profile, opening_layout.views())?
            .selection();
        for (group_index, ((layout, handle), claim)) in opening_layout
            .groups()
            .iter()
            .zip(&handles)
            .zip(claims.groups())
            .enumerate()
        {
            opening_layout
                .group_view(group_index)?
                .validate_producer_contract(handle.producer_contract())?;
            if claim.point().len() != opening_layout.group_point_num_vars(group_index)? {
                return Err(AkitaError::Internal(
                    "validated claim point length differs from committed layout".into(),
                ));
            }
            let meta = handle.metadata();
            if meta.num_vars() != layout.num_vars()
                || meta.num_polynomials() != layout.num_polynomials()
            {
                return Err(AkitaError::InvalidInput(
                    "commitment handle shape differs from statement".into(),
                ));
            }
        }
        // The proving catalog plans its final group under `Cfg`'s contract.
        // Precommitted producers are not recorded in the row, so planning
        // code compares those handles against its own producer declarations.
        let final_handle = handles.last().ok_or_else(|| {
            AkitaError::Internal("validated opening has no final commitment handle".into())
        })?;
        if final_handle.producer_contract() != Cfg::committed_source_contract()? {
            return Err(AkitaError::InvalidInput(
                "final group was committed under a different producer contract".into(),
            ));
        }
        let groups = claims
            .groups()
            .iter()
            .map(|g| g.clone().map_commitment(|committed| committed.commitment().clone()))
            .collect::<Vec<_>>();
        Ok(Self {
            selection,
            opening_data: ProverOpeningData::from_parts(
                OpeningClaims::from_groups(groups)?,
                opening_layout,
                handles,
            )?,
        })
    }
    pub fn selection(&self) -> OpeningScheduleSelection {
        self.selection
    }
    pub fn opening_layout(&self) -> &OpeningClaimsLayout {
        self.opening_data.opening_layout()
    }
    /// Borrow public claims without exposing handles or copying commitments.
    pub fn opening_claims(&self) -> &OpeningClaims<'a, E, Commitment<F>> {
        self.opening_data.opening_claims()
    }
    pub(crate) fn into_low_level_parts(
        self,
    ) -> (OpeningScheduleSelection, ProverOpeningData<'a, E, H, F>) {
        (self.selection, self.opening_data)
    }
}

pub struct ProverOpeningData<'a, E: Clone, G, F: Field> {
    opening_claims: OpeningClaims<'a, E, Commitment<F>>,
    opening_layout: OpeningClaimsLayout,
    groups: Vec<G>,
}
impl<'a, PointF: Clone, G, CommitF: Field> ProverOpeningData<'a, PointF, G, CommitF> {
    pub(crate) fn from_parts(
        opening_claims: OpeningClaims<'a, PointF, Commitment<CommitF>>,
        opening_layout: OpeningClaimsLayout,
        groups: Vec<G>,
    ) -> Result<Self, AkitaError> {
        if opening_claims.num_groups() != groups.len()
            || groups.len() != opening_layout.num_groups()
        {
            return Err(AkitaError::Internal(
                "assembled opening claims, handles, and layout have different group counts".into(),
            ));
        }
        for (index, (claims, layout)) in opening_claims
            .groups()
            .iter()
            .zip(opening_layout.groups())
            .enumerate()
        {
            let point_len = opening_layout.group_point_num_vars(index)?;
            let view = opening_layout.group_view(index)?;
            if claims.source_view() != view
                || claims.point().len() > point_len
                || (view == akita_params::SourceOpeningView::OriginalRamPlanesV1
                    && claims.point().len() != point_len)
                || claims.evaluations().len() != layout.num_polynomials()
            {
                return Err(AkitaError::Internal(
                    "assembled opening claim shape differs from group layout".into(),
                ));
            }
        }
        Ok(Self {
            opening_claims,
            opening_layout,
            groups,
        })
    }
    pub fn opening_claims(&self) -> &OpeningClaims<'a, PointF, Commitment<CommitF>> {
        &self.opening_claims
    }
    pub fn opening_layout(&self) -> &OpeningClaimsLayout {
        &self.opening_layout
    }
    pub(crate) fn group(&self, index: usize) -> Result<&G, AkitaError> {
        self.groups.get(index).ok_or_else(|| {
            AkitaError::Internal("assembled opening has no handle for the requested group".into())
        })
    }
    pub fn commitments(&self) -> Vec<&Commitment<CommitF>> {
        self.opening_claims
            .groups()
            .iter()
            .map(PolynomialGroupClaims::commitment)
            .collect()
    }
    pub(crate) fn map_groups<'b, Q>(
        &'b self,
        mut map: impl FnMut(&'b G) -> Q,
    ) -> Result<ProverOpeningData<'a, PointF, Q, CommitF>, AkitaError> {
        ProverOpeningData::from_parts(
            self.opening_claims.clone(),
            self.opening_layout.clone(),
            self.groups.iter().map(&mut map).collect(),
        )
    }
    pub(crate) fn append_to(
        &self,
        root_params: &CommittedGroupParams,
        grinding: &mut akita_types::ProverGrinding<'_>,
    ) -> Result<(), AkitaError>
    where
        CommitF: CanonicalEncoding,
        PointF: ExtField<CommitF>,
    {
        let relation_geometry = akita_params::RelationWitnessGeometry::for_level(
            root_params,
            self.opening_layout(),
            PointF::DEGREE,
        )?;
        let relation_layout = relation_geometry.rhs_layout();
        for (group_index, commitment) in self.commitments().into_iter().enumerate() {
            let compression = relation_layout.compression_plan_for_group(group_index)?;
            if commitment.rows().coeff_len() != compression.terminal_coefficients() {
                return Err(AkitaError::InvalidInput(
                    "root compressed commitment does not match scheduled root params".into(),
                ));
            }
            let ring_dim = compression
                .maps()
                .last()
                .ok_or_else(|| {
                    AkitaError::Internal("root commitment compression plan has no final map".into())
                })?
                .ring_dimension();
            akita_transcript::public_fields_prover(
                grinding.state_mut(),
                akita_types::FoldSite::RootCommitment {
                    group: group_index,
                    ring_dimension: ring_dim,
                }
                .id()?,
                commitment.rows().coeffs(),
            )?;
        }
        for (group_index, group_claims) in self.opening_claims.groups().iter().enumerate() {
            akita_transcript::public_extensions::<CommitF, PointF, _>(
                grinding.state_mut(),
                akita_types::FoldSite::RootPoint { group: group_index }.id()?,
                group_claims.point(),
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use akita_params::{PolynomialGroupLayout, SourceOpeningView};
    use akita_types::RingVec;
    use jolt_field::Prime128OffsetA7F7;

    type K = Prime128OffsetA7F7;

    #[test]
    fn raw_query_keeps_source_geometry_and_view_through_mapping() {
        let layout = OpeningClaimsLayout::from_groups_and_views(
            vec![PolynomialGroupLayout::singleton(16)],
            vec![SourceOpeningView::OriginalRamPlanesV1],
        ).unwrap();
        let claims = OpeningClaims::from_groups(vec![
            PolynomialGroupClaims::original_ram_planes_v1(
                vec![7u64; 21],
                9,
                Commitment::<K>::new(RingVec::from_coeffs(Vec::new())),
            ).unwrap(),
        ]).unwrap();
        let opening = ProverOpeningData::from_parts(claims, layout.clone(), vec![()]).unwrap();
        let mapped = opening.map_groups(|_| 1usize).unwrap();
        assert_eq!(mapped.opening_layout().group_layout(0).unwrap().num_vars(), 16);
        assert_eq!(mapped.opening_layout().group_point_num_vars(0).unwrap(), 21);
        assert_eq!(mapped.opening_claims().groups()[0].source_view(),
            SourceOpeningView::OriginalRamPlanesV1);
        assert_eq!(mapped.opening_claims().group_point(0).unwrap(), &[7; 21]);

        let decoded = OpeningClaimsLayout::new(16, 1).unwrap();
        assert!(ProverOpeningData::from_parts(
            opening.opening_claims().clone(), decoded, vec![()],
        ).is_err());
        let ordinary_claims = OpeningClaims::from_groups(vec![
            PolynomialGroupClaims::new(
                vec![7u64; 16], vec![9],
                Commitment::<K>::new(RingVec::from_coeffs(Vec::new())),
            ).unwrap(),
        ]).unwrap();
        assert!(ProverOpeningData::from_parts(ordinary_claims, layout, vec![()]).is_err());
    }

    #[test]
    fn decoded_suffix_retains_short_point_semantics() {
        let claims = OpeningClaims::from_groups(vec![
            PolynomialGroupClaims::new(
                vec![7u64; 5], vec![9],
                Commitment::<K>::new(RingVec::from_coeffs(Vec::new())),
            ).unwrap(),
        ]).unwrap();
        assert!(ProverOpeningData::from_parts(
            claims, OpeningClaimsLayout::new(16, 1).unwrap(), vec![()],
        ).is_ok());
    }
}
