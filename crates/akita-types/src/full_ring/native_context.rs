//! Public relation assembly shared by the native producer and verifier.
//! Inputs are the original bound statements, query endpoints and shell values;
//! witness ranges are derived from admitted parameters, never supplied by the
//! caller. Original matrix roles are resolved only through the installed setup.

use super::*;
use crate::{FullRingSourceId, InstalledFullRingSetup, RingVec};
use akita_error::AkitaError;
use akita_params::{CommittedGroupParams, OpeningClaimsLayout, WitnessLayout};
use jolt_field::{CanonicalEncoding, ExtField, Field};

/// The complete original F payloads and one shared opening H payload, indexed
/// by public group id independently of the physical execution order.
#[derive(Clone)]
pub struct FullRingBuildContext<F: Field, E: Field> {
    openings: Vec<(FullRingRawPoint<E>, E, FullRingSourceId)>,
    commitments: Vec<RingVec<F>>,
    opening_payload: RingVec<F>,
}
impl<F: Field, E: Field> FullRingBuildContext<F, E> {
    pub fn new(
        openings: Vec<(FullRingRawPoint<E>, E, FullRingSourceId)>,
        commitments: Vec<RingVec<F>>,
        opening_payload: RingVec<F>,
    ) -> Result<Self, AkitaError> {
        if openings.is_empty() || openings.len() != commitments.len() {
            return Err(invalid(
                "complete original full-ring group payloads are required",
            ));
        }
        Ok(Self {
            openings,
            commitments,
            opening_payload,
        })
    }
    pub fn openings(&self) -> &[(FullRingRawPoint<E>, E, FullRingSourceId)] {
        &self.openings
    }
    pub fn commitments(&self) -> &[RingVec<F>] {
        &self.commitments
    }
    pub fn opening_payload(&self) -> &RingVec<F> {
        &self.opening_payload
    }
}

/// Checked geometry/row inputs retained by a native successor. This type does
/// not claim transcript authentication: both native drivers construct it from
/// already-bound values under their original owner/scope and installed setup.
pub struct FullRingWitnessContext<F: Field, E: Field> {
    level_index: u32,
    build: FullRingBuildContext<F, E>,
    level: CommittedGroupParams,
    opening_batch: OpeningClaimsLayout,
    layout: WitnessLayout,
    challenges: Vec<Vec<F>>,
}
impl<F: Field + CanonicalEncoding, E: ExtField<F>> FullRingWitnessContext<F, E> {
    pub fn new(
        level_index: u32,
        build: FullRingBuildContext<F, E>,
        level: CommittedGroupParams,
        opening_batch: OpeningClaimsLayout,
        challenges: Vec<Vec<F>>,
    ) -> Result<Self, AkitaError> {
        validate_full_ring_fields::<F, E>()?;
        if level.ring_relation_mode != akita_params::RingRelationMode::ExactCoefficientRows
            || level.source_encoding
                != akita_params::CommittedSourceEncoding::CanonicalCoefficientTable
            || level.witness_chunk.num_chunks != 1
            || build.openings.len() != opening_batch.num_groups()
            || challenges.len() != opening_batch.num_groups()
        {
            return Err(invalid("full-ring successor requires exact rows, original sources and complete unsplit groups"));
        }
        for (index, ((point, _, _), challenge)) in
            build.openings.iter().zip(&challenges).enumerate()
        {
            let params = level.group_params_geometry(&opening_batch, index)?;
            params.validate()?;
            let dims = level.group_role_dims_geometry(&opening_batch, index)?;
            let view = point.view();
            view.validate_fold_admission()?;
            if params.opening_method() != akita_params::OpeningMethod::FULL_RING_E2
                || dims.d_a() != 128
                || view.degree() != 128
                || view.polynomials() != opening_batch.group_layout(index)?.num_polynomials()
                || view.blocks() != params.num_live_blocks()
                || view.positions() != params.num_positions_per_block()
                || view.digits() != params.num_digits_inner()
                || params.fold_challenge_config() != akita_params::FULL_RING_FOLD_CHALLENGE
                || challenge.len() != product(&[view.polynomials(), view.blocks(), 128])?
            {
                return Err(invalid(
                    "retained full-ring point/challenge differs from admitted group geometry",
                ));
            }
            for ring in challenge.chunks_exact(128) {
                validate_dense_shell(ring)?;
            }
        }
        let geometry =
            akita_params::RelationWitnessGeometry::for_level(&level, &opening_batch, E::DEGREE)?;
        let layout = WitnessLayout::new(
            &level,
            &opening_batch,
            &geometry,
            1,
            akita_params::RelationQuotientPlan::for_field_bits(&level, F::MODULUS_BITS)?,
        )?;
        layout.validate_internal_ranges()?;
        if layout.units().len() != opening_batch.num_groups() {
            return Err(invalid(
                "full-ring group count differs from complete witness units",
            ));
        }
        Ok(Self {
            level_index,
            build,
            level,
            opening_batch,
            layout,
            challenges,
        })
    }
    pub fn level_index(&self) -> u32 {
        self.level_index
    }
    pub fn build(&self) -> &FullRingBuildContext<F, E> {
        &self.build
    }
    pub fn level(&self) -> &CommittedGroupParams {
        &self.level
    }
    pub fn opening_batch(&self) -> &OpeningClaimsLayout {
        &self.opening_batch
    }
    pub fn layout(&self) -> &WitnessLayout {
        &self.layout
    }
    pub fn challenges(&self) -> &[Vec<F>] {
        &self.challenges
    }

    pub fn with_relation<T>(
        &self,
        setup: &InstalledFullRingSetup<F>,
        domain_len: usize,
        use_relation: impl FnOnce(&CoefficientRelationPlan<'_, F, E>) -> Result<T, AkitaError>,
    ) -> Result<T, AkitaError> {
        if domain_len < self.layout.live_coeff_len() {
            return Err(invalid(
                "outgoing domain truncates the complete native witness",
            ));
        }
        let level_index = usize::try_from(self.level_index)
            .map_err(|_| invalid("installed fold level index overflow"))?;
        let level_roles = setup.descriptor().level(level_index)?;
        if level_roles.sources.len() != self.layout.units().len()
            || level_roles.compression.len() != self.layout.compression_layers().len()
        {
            return Err(invalid(
                "installed level omits source or shared H occurrences",
            ));
        }
        let matrix = |role| NativeMatrix::from_installed(setup, role);
        let mut groups = Vec::with_capacity(self.build.openings.len());
        // Installed sources are in physical execution order (root final group
        // first). Retained statements remain indexed by public group id.
        for (unit, &installed_source) in self.layout.units().iter().zip(&level_roles.sources) {
            let index = unit.group_index();
            let (point, value, source) = self
                .build
                .openings
                .get(index)
                .ok_or_else(|| invalid("missing original full-ring group"))?;
            let params = self
                .level
                .group_params_geometry(&self.opening_batch, index)?;
            let view = point.view();
            if *source != installed_source {
                return Err(invalid(
                    "installed level source order differs from original public group map",
                ));
            }
            let original = setup.descriptor().source(*source)?;
            let commitment_roles = &original.commitment_roles;
            if &original.raw_view != view
                || commitment_roles.compression.len() != self.layout.compression_layers().len()
            {
                return Err(invalid(
                    "original source injection or complete F path differs from installed role",
                ));
            }
            let outer_role = commitment_roles
                .outer
                .ok_or_else(|| invalid("nonterminal input has no original B role"))?;
            let base = F::from_u64(32);
            // Keep Z/A in physical order. The checked raw-map adjoint is
            // compiled only into L and its matching partial/challenge sum.
            let response = DigitLayout::new(
                unit.z_range().start,
                product(&[view.positions(), view.digits()])?,
                128,
                params.num_digits_fold(),
                base,
            )?;
            let partials = DigitLayout::new(
                unit.e_range().start,
                product(&[view.polynomials(), view.blocks(), E::DEGREE])?,
                128,
                params.num_digits_open(),
                base,
            )?;
            let inner_images = DigitLayout::new(
                unit.t_range().start,
                product(&[view.polynomials(), view.blocks(), params.a_rows_len()])?,
                128,
                params.num_digits_outer(),
                base,
            )?;
            if response.range() != unit.z_range()
                || partials.range() != unit.e_range()
                || inner_images.range() != unit.t_range()
            {
                return Err(invalid(
                    "native digit layouts do not cover the exact original witness spans",
                ));
            }
            let mut stages = Vec::with_capacity(self.layout.compression_layers().len());
            for (stage, layer) in self.layout.compression_layers().iter().enumerate() {
                let span = layer
                    .f_spans()
                    .iter()
                    .find_map(|(group, span)| (*group == index).then_some(span))
                    .ok_or_else(|| invalid("original F input span missing"))?;
                stages.push(CompressionStage {
                    matrix: matrix(commitment_roles.compression[stage])?,
                    input: span.range(),
                    digit_bits: 128,
                });
            }
            groups.push(FullRingRelationGroup {
                view,
                a: matrix(commitment_roles.inner)?,
                b: matrix(outer_role)?,
                challenges: self
                    .challenges
                    .get(index)
                    .ok_or_else(|| invalid("retained fold challenge missing"))?,
                response,
                inner_images,
                channels: vec![FullRingRelationChannel {
                    point,
                    partials,
                    scalar_claims: vec![PartialScalarClaim::raw(point, *value)],
                }],
                f: CompressionChain {
                    stages,
                    payload: self
                        .build
                        .commitments
                        .get(index)
                        .ok_or_else(|| invalid("original F payload missing"))?
                        .coeffs(),
                },
            });
        }
        let mut stages = Vec::with_capacity(self.layout.compression_layers().len());
        for (stage, layer) in self.layout.compression_layers().iter().enumerate() {
            stages.push(CompressionStage {
                matrix: matrix(level_roles.compression[stage])?,
                input: layer.h_span().range(),
                digit_bits: 128,
            });
        }
        let h = CompressionChain {
            stages,
            payload: self.build.opening_payload.coeffs(),
        };
        let relation =
            CoefficientRelationPlan::new_d1(domain_len, groups, matrix(level_roles.opening)?, h)?;
        use_relation(&relation)
    }
}

/// Expand only the public sparse shell, not any source. Shared by producer and
/// verifier so the checked coefficient plan receives the same original S unit.
pub fn full_ring_dense_challenges<F: Field>(
    challenges: &akita_challenges::Challenges,
) -> Result<Vec<F>, AkitaError> {
    let len = challenges
        .len()
        .checked_mul(128)
        .ok_or_else(|| invalid("full-ring challenge extent overflow"))?;
    let mut coefficients = vec![F::zero(); len];
    for (index, challenge) in challenges.as_slice().iter().enumerate() {
        challenge.validate::<128>()?;
        for (&position, &value) in challenge.positions.iter().zip(&challenge.coeffs) {
            coefficients[index * 128 + position as usize] = if value < 0 {
                -F::from_u64(u64::from(value.unsigned_abs()))
            } else {
                F::from_u64(value as u64)
            };
        }
        validate_dense_shell(&coefficients[index * 128..(index + 1) * 128])?;
    }
    Ok(coefficients)
}
fn validate_dense_shell<F: Field>(ring: &[F]) -> Result<(), AkitaError> {
    let mut ones = 0usize;
    let mut twos = 0usize;
    let one = F::one();
    let two = F::from_u64(2);
    for &coefficient in ring {
        if coefficient == one || coefficient == -one {
            ones += 1;
        } else if coefficient == two || coefficient == -two {
            twos += 1;
        } else if !coefficient.is_zero() {
            return Err(invalid(
                "full-ring shell coefficient outside signed alphabet",
            ));
        }
    }
    if ring.len() != 128 || ones != 24 || twos != 16 {
        return Err(invalid(
            "full-ring shell must have exactly 24 unit and 16 double positions",
        ));
    }
    Ok(())
}
