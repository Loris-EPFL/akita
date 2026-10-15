//! Original balanced base-32 RAM digit planes, shared by commitment and opening.

use super::full_ring::FullRingSource;
use super::packed_digits::PackedSignedDigits;
use super::poly::{RootOpeningSource, RootPolyMeta, RootPolyShape};
use super::poly_helpers::packed_tight_digit_fold_partitioned;
use crate::commitment::{
    AvailablePolynomialTypes, CommitSourceClass, CommitSourceDescriptor, CommitmentSource,
    DenseRepresentation, DenseType, DigitPlaneStorage, PolynomialRepresentation, PolynomialType,
    PolynomialTypeSelection, PredecomposedDigitPlanes,
};
use crate::opaque::{
    aggregate_decompose_fold_witnesses, CommitInnerPlan, CpuBackend, CpuFoldResponses,
    DecomposeFoldBatchPlan, DecomposeFoldWitness, OpeningBatchKernel, OpeningFoldOutput,
    OpeningFoldPlan,
};
use akita_challenges::SparseChallenge;
use akita_error::{checked, AkitaError};
use jolt_field::{CanonicalEncoding, Field};
use std::ops::Range;

const RING_DIMENSION: usize = 128;
const DIGIT_PLANES: usize = 26;
const LOG_BASIS: u32 = 5;

/// Owned original RAM source for native full-ring import.
///
/// This is one logical polynomial, not 26 polynomials. Its immutable packed
/// allocation contains exactly `[block][position][digit][coefficient]` signed
/// digits, with 128 coefficients, 26 planes and balanced base 32 (`-16..=15`).
/// The six unused slots in a 32-slot raw query grid are **not** stored here.
/// Declared A-block padding is stored and must consist of zero digits. Even a
/// nonzero digit path representing zero modulo K is forbidden in that padding.
///
/// Legal noncanonical digit paths in the logical domain are preserved verbatim:
/// construction packs signed integers, never reconstructs and redecomposes K.
/// Commitment and fold kernels borrow the same packed allocation. No dense K or
/// E table is allocated. All accesses authenticate the complete supplied A plan,
/// including its row rank. Only the original 128-coefficient, 26-plane base-32
/// encoding is supported; this type does not offer legacy tensor/EOR openings.
/// Ordinary K/K native k1h1 coefficient packing is available through
/// [`CpuBackend::import_full_ring_commitment`], which authenticates the original
/// commitment and derives a separate quotient-lift material view. Decoded
/// partials use Horner; the registered OriginalRamPlanesV1 view contracts all
/// original signed planes with public Eq weights, without redecomposition.
///
/// Construct with [`Self::from_signed_digits`], then move a vector of sources to
/// [`CpuBackend::import_source_full_ring`]. Retain the resulting source and
/// commitment handles for all subsequent openings.
#[derive(Debug)]
pub struct PackedRamSource {
    digits: PackedSignedDigits,
    plan: CommitInnerPlan,
    num_vars: usize,
    logical_len: usize,
    physical_len: usize,
}

impl PackedRamSource {
    /// Pack exactly the supplied original signed planes under an immutable plan.
    ///
    /// `num_vars >= 7` declares `2^num_vars` logical K coefficients. The plan
    /// must have positive block count/rank, power-of-two positions per block,
    /// ring dimension 128, 26 digits and log basis 5. The vector length must be
    /// `blocks * positions * 26 * 128`, with zeroes beyond the logical domain.
    /// Invalid shape, signed encoding, nonzero padding and overflow are errors.
    pub fn from_signed_digits(
        num_vars: usize,
        plan: CommitInnerPlan,
        digits: Vec<i8>,
    ) -> Result<Self, AkitaError> {
        if num_vars < 7
            || plan.ring_dimension != RING_DIMENSION
            || plan.num_digits_inner != DIGIT_PLANES
            || plan.log_basis_inner != LOG_BASIS
            || plan.num_live_blocks == 0
            || plan.n_a == 0
            || !plan.num_positions_per_block.is_power_of_two()
        {
            return Err(AkitaError::InvalidInput(
                "unsupported packed RAM source geometry".into(),
            ));
        }
        let logical_len = checked::pow2(num_vars)
            .ok_or_else(|| AkitaError::InvalidInput("packed RAM logical extent overflow".into()))?;
        let physical_len = checked::product([
            plan.num_live_blocks,
            plan.num_positions_per_block,
            RING_DIMENSION,
        ])
        .ok_or_else(|| AkitaError::InvalidInput("packed RAM physical extent overflow".into()))?;
        let expected = physical_len
            .checked_mul(DIGIT_PLANES)
            .ok_or_else(|| AkitaError::InvalidInput("packed RAM digit extent overflow".into()))?;
        if logical_len > physical_len {
            return Err(AkitaError::InvalidInput(
                "packed RAM A plan truncates the logical source".into(),
            ));
        }
        if digits.len() != expected {
            return Err(AkitaError::InvalidSize {
                expected,
                actual: digits.len(),
            });
        }
        // The logical domain ends on a whole ring (num_vars >= 7).
        if digits[logical_len * DIGIT_PLANES..]
            .iter()
            .any(|&digit| digit != 0)
        {
            return Err(AkitaError::InvalidInput(
                "packed RAM physical padding must be zero".into(),
            ));
        }
        let digits = PackedSignedDigits::from_i8_digits(digits, LOG_BASIS as u8)?;
        Ok(Self {
            digits,
            plan,
            num_vars,
            logical_len,
            physical_len,
        })
    }

    /// Exact source plan; raw query padding never changes this plan.
    pub fn plan(&self) -> CommitInnerPlan {
        self.plan
    }

    fn validate_plan(&self, plan: &CommitInnerPlan) -> Result<(), AkitaError> {
        if *plan != self.plan {
            return Err(AkitaError::InvalidInput(
                "packed RAM original source plan mismatch".into(),
            ));
        }
        Ok(())
    }

    fn fold_chunk(
        &self,
        challenges: &[SparseChallenge],
        range: Range<usize>,
    ) -> Result<DecomposeFoldWitness, AkitaError> {
        if range.start > range.end
            || range.end > self.plan.num_live_blocks
            || challenges.len() != range.len()
        {
            return Err(AkitaError::InvalidInput(
                "packed RAM fold block extent mismatch".into(),
            ));
        }
        // Flatten M*26 only as a kernel view. Output is still [position][digit].
        let positions = self.plan.num_positions_per_block * DIGIT_PLANES;
        let block_width = positions * RING_DIMENSION;
        let digits = self
            .digits
            .view()
            .slice(range.start * block_width..range.end * block_width)?;
        Ok(DecomposeFoldWitness::from_centered_rows(
            packed_tight_digit_fold_partitioned::<RING_DIMENSION>(
                digits,
                range.len() * positions,
                challenges,
                positions,
            ),
        ))
    }
}

impl<F: Field> CommitmentSource<F> for PackedRamSource {
    fn descriptor(&self) -> Result<CommitSourceDescriptor, AkitaError> {
        CommitSourceDescriptor::new(
            self.num_vars,
            self.physical_len,
            self.logical_len,
            CommitSourceClass::OriginalBalancedDigitPlanes {
                log_basis: LOG_BASIS,
                num_digits: DIGIT_PLANES,
            },
            "akita_original_packed_ram",
        )
        .map(CommitSourceDescriptor::with_full_physical_extent)
    }

    fn committed_centered_reach(
        &self,
        modulus: u128,
        centering_threshold: u128,
    ) -> Result<(u128, u128), AkitaError>
    where
        F: CanonicalEncoding,
    {
        let mut reach = (0u128, 0u128);
        let basis = F::from_u64(32);
        // Exact decoded K reach remains available to callers, but producer
        // admission uses the intrinsic original-plane envelope instead.
        // This scalar reconstruction never replaces the committed planes.
        for ring in 0..self.logical_len / RING_DIMENSION {
            for coefficient in 0..RING_DIMENSION {
                let mut value = F::zero();
                for digit in (0..DIGIT_PLANES).rev() {
                    value = value * basis
                        + F::from_i8(
                            self.digits
                                .view()
                                .at((ring * DIGIT_PLANES + digit) * RING_DIMENSION + coefficient),
                        );
                }
                let canonical = value.to_u128_checked().ok_or_else(|| {
                    AkitaError::InvalidInput(
                        "packed RAM field coefficient does not fit u128".into(),
                    )
                })?;
                if canonical <= centering_threshold {
                    reach.1 = reach.1.max(canonical);
                } else {
                    let magnitude = modulus.checked_sub(canonical).ok_or_else(|| {
                        AkitaError::InvalidInput("packed RAM admission modulus mismatch".into())
                    })?;
                    reach.0 = reach.0.max(magnitude);
                }
            }
        }
        Ok(reach)
    }

    fn available_polynomial_types(
        &self,
        plan: &CommitInnerPlan,
    ) -> Result<AvailablePolynomialTypes, AkitaError> {
        self.validate_plan(plan)?;
        AvailablePolynomialTypes::new(vec![PolynomialType::Dense(DenseType::PredecomposedDigits)])
    }

    fn represent_as(
        &self,
        selected: PolynomialTypeSelection,
        plan: &CommitInnerPlan,
    ) -> Result<PolynomialRepresentation<'_, F>, AkitaError> {
        self.validate_plan(plan)?;
        if selected.polynomial_type() != PolynomialType::Dense(DenseType::PredecomposedDigits) {
            return Err(AkitaError::InvalidInput(
                "packed RAM received an unadvertised representation".into(),
            ));
        }
        Ok(PolynomialRepresentation::Dense(
            DenseRepresentation::PredecomposedDigits(PredecomposedDigitPlanes {
                bytes: DigitPlaneStorage::Packed(self.digits.view()),
                ring_dimension: RING_DIMENSION,
                num_digits: DIGIT_PLANES,
                log_basis: LOG_BASIS,
                logical_ring_count: self.physical_len / RING_DIMENSION,
            }),
        ))
    }
}

impl<F: Field> FullRingSource<F> for PackedRamSource {
    fn visit_full_ring_digits(
        &self,
        plan: &CommitInnerPlan,
        visitor: &mut dyn FnMut(usize, F) -> Result<(), AkitaError>,
    ) -> Result<(), AkitaError> {
        self.validate_plan(plan)?;
        for (index, digit) in self.digits.iter().enumerate() {
            if digit != 0 {
                visitor(index, F::from_i8(digit))?;
            }
        }
        Ok(())
    }

    fn full_ring_digit(&self, plan: &CommitInnerPlan, index: usize) -> Result<F, AkitaError> {
        self.validate_plan(plan)?;
        self.digits.get(index).map(F::from_i8).ok_or_else(|| {
            AkitaError::InvalidInput("packed RAM physical digit index out of range".into())
        })
    }
}

impl<F: Field> RootPolyMeta<F> for PackedRamSource {
    fn num_vars(&self) -> usize {
        self.num_vars
    }
}

impl<F: Field> RootPolyShape<F, RING_DIMENSION> for PackedRamSource {
    fn num_ring_elems(&self) -> usize {
        self.physical_len / RING_DIMENSION
    }
    fn num_live_ring_elems(&self) -> usize {
        self.logical_len / RING_DIMENSION
    }
    fn num_vars(&self) -> usize {
        self.num_vars
    }
}

impl<F: Field> RootOpeningSource<F, RING_DIMENSION> for PackedRamSource {
    type OpeningView<'a> = &'a Self;
    type OpeningBatchView<'a> = &'a [&'a Self];

    fn opening_view(&self) -> Result<Self::OpeningView<'_>, AkitaError> {
        Ok(self)
    }

    fn opening_batch<'a>(polys: &'a [&'a Self]) -> Result<Self::OpeningBatchView<'a>, AkitaError> {
        Ok(polys)
    }
}

impl<F: Field + CanonicalEncoding, E> OpeningBatchKernel<&[&PackedRamSource], F, RING_DIMENSION>
    for CpuBackend<F, E>
{
    fn evaluate_and_fold_batch(
        &self,
        _prepared: Option<&Self::PreparedSetup>,
        _source: &[&PackedRamSource],
        _plan: OpeningFoldPlan<'_, F>,
    ) -> Result<Vec<OpeningFoldOutput<F, RING_DIMENSION>>, AkitaError> {
        Err(AkitaError::InvalidInput(
            "original packed RAM sources require native full-ring openings, not legacy L/F".into(),
        ))
    }

    fn decompose_fold_batch(
        &self,
        _prepared: Option<&Self::PreparedSetup>,
        source: &[&PackedRamSource],
        plan: DecomposeFoldBatchPlan<'_>,
    ) -> Result<CpuFoldResponses, AkitaError> {
        let (positions, digits, log_basis) = plan.scalar_params();
        for poly in source {
            if positions != poly.plan.num_positions_per_block
                || digits != DIGIT_PLANES
                || log_basis != LOG_BASIS
            {
                return Err(AkitaError::InvalidInput(
                    "packed RAM fold plan mismatch".into(),
                ));
            }
        }
        let blocks =
            plan.validate_uniform_batch(source.iter().map(|poly| poly.plan.num_live_blocks))?;
        match plan {
            DecomposeFoldBatchPlan::Sparse { challenges, .. } => Ok(CpuFoldResponses::sparse(
                aggregate_decompose_fold_witnesses::<RING_DIMENSION>(
                    source
                        .iter()
                        .zip(challenges.chunks_exact(blocks))
                        .map(|(poly, challenges)| poly.fold_chunk(challenges, 0..blocks)),
                )?,
            )),
            DecomposeFoldBatchPlan::SparseChunked {
                challenges,
                chunk_ranges,
                ..
            } => CpuFoldResponses::chunked::<RING_DIMENSION>(
                chunk_ranges
                    .iter()
                    .map(|range| {
                        aggregate_decompose_fold_witnesses::<RING_DIMENSION>(
                            source
                                .iter()
                                .zip(challenges.as_slice().chunks_exact(blocks))
                                .map(|(poly, challenges)| {
                                    let chunk = challenges.get(range.clone()).ok_or_else(|| {
                                        AkitaError::InvalidInput(
                                            "packed RAM fold chunk out of range".into(),
                                        )
                                    })?;
                                    poly.fold_chunk(chunk, range.clone())
                                }),
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        }
    }
}

/// Native degree-one, stride-one packing over the retained original planes.
/// The public source view chooses Horner decoding or the raw digit functional;
/// neither source ownership nor the fold witness passes through decoded data.
impl<F> crate::opaque::SubringCoefficientPackingBatchKernel<&[&PackedRamSource], F, F, 128>
    for CpuBackend<F, F>
where
    F: Field + CanonicalEncoding + jolt_field::ExtField<F>
        + akita_types::FpExtEncoding<F> + jolt_field::MulBaseUnreduced<F>,
{
    fn coefficient_packing_partials_batch(
        &self,
        _prepared: Option<&Self::PreparedSetup>,
        source: &[&PackedRamSource],
        plan: crate::opaque::SubringCoefficientPackingPlan<'_, F>,
    ) -> Result<Vec<crate::opaque::SubringCoefficientPackingPartials<F>>, AkitaError> {
        use crate::arithmetic::coefficient_packing::{
            coefficient_packing_partials_from_position_source, FusedPackingWeights,
        };
        let point = plan.point;
        if point.geometry().extension_degree() != 1
            || point.geometry().challenge_subring_dimension() != RING_DIMENSION
            || point.geometry().subring_embedding_stride() != 1
        {
            return Err(AkitaError::InvalidInput("packed RAM requires native k1h1 packing".into()));
        }
        if point.source_view() == akita_params::SourceOpeningView::OriginalRamPlanesV1 {
            const Q: u128 = u128::MAX - (1u128 << 32) + 22538;
            let mut modulus = [0u8; 32];
            modulus[16..].copy_from_slice(&Q.to_be_bytes());
            if akita_params::field_modulus_be_bytes::<F>()? != modulus {
                return Err(AkitaError::InvalidInput("raw RAM requires the installed A7F7 prime field".into()));
            }
        }
        for poly in source {
            plan.validate::<RING_DIMENSION>(poly.num_vars)?;
            if point.num_live_positions() != poly.logical_len / RING_DIMENSION
                || point.num_positions_per_block() != poly.plan.num_positions_per_block
                || point.num_live_blocks() != poly.plan.num_live_blocks
                || (point.source_view() == akita_params::SourceOpeningView::OriginalRamPlanesV1
                    && (poly.num_vars != 16 || poly.plan.num_live_blocks != 16
                        || poly.plan.num_positions_per_block != 32 || poly.plan.n_a != 3))
            {
                return Err(AkitaError::InvalidInput("packed RAM packing geometry mismatch".into()));
            }
        }
        let weights = FusedPackingWeights::new(point)?;
        let digit_weights = point.source_digit_weights();
        source.iter().map(|poly| {
            let coordinates = coefficient_packing_partials_from_position_source::<F, F, _, 128>(
                &weights,
                poly.num_vars,
                |position| poly.digits.view().slice(
                    position * DIGIT_PLANES * RING_DIMENSION
                        ..(position + 1) * DIGIT_PLANES * RING_DIMENSION,
                ),
                |_, coefficient, digits| {
                    match digit_weights {
                        Some(weights) => weights.iter().enumerate().fold(F::zero(), |value, (digit, &weight)| {
                            value + weight * F::from_i8(digits.at(digit * RING_DIMENSION + coefficient))
                        }),
                        None => (0..DIGIT_PLANES).rev().fold(F::zero(), |value, digit| {
                            value * F::from_u64(32)
                                + F::from_i8(digits.at(digit * RING_DIMENSION + coefficient))
                        }),
                    }
                },
            )?;
            crate::opaque::SubringCoefficientPackingPartials::new(
                point.geometry(), point.num_live_blocks(), coordinates,
            )
        }).collect()
    }
}

#[cfg(test)]
mod tests;
