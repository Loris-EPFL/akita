//! Source-native physical digit access for the scalar-extended full ring.
//!
//! A visitor observes the exact A-input digits, not decoded polynomial values.
//! Sparse sources may omit zero cells; every reported address is in physical
//! `[block][position][source digit][coefficient]` order for one polynomial.
use crate::commitment::{
    CommitmentSource, DenseRepresentation, PolynomialRepresentation, UnitPositionSlice,
};
use crate::opaque::CommitInnerPlan;
use akita_algebra::{ring::cyclotomic::BalancedDecomposePow2Params, CyclotomicRing};
use akita_error::AkitaError;
use jolt_field::{CanonicalEncoding, Field};

/// Native access to the original K source committed by the supplied A plan.
/// Implementations must preserve supplied legal noncanonical digit paths.
/// Addresses are unique, increasing, and omit only zero-valued cells.
pub trait FullRingSource<F: Field>: CommitmentSource<F> {
    /// Visit physical digits without materializing a decoded or E-valued source.
    fn visit_full_ring_digits(
        &self,
        plan: &CommitInnerPlan,
        visitor: &mut dyn FnMut(usize, F) -> Result<(), AkitaError>,
    ) -> Result<(), AkitaError>;
    /// Read one original physical digit, including declared A-block padding.
    fn full_ring_digit(&self, plan: &CommitInnerPlan, index: usize) -> Result<F, AkitaError>;
}

fn visit_representation<F: Field + CanonicalEncoding>(
    source: &dyn CommitmentSource<F>,
    plan: &CommitInnerPlan,
    visitor: &mut dyn FnMut(usize, F) -> Result<(), AkitaError>,
) -> Result<(), AkitaError> {
    if plan.ring_dimension != 128
        || plan.num_digits_inner == 0
        || !(1..=7).contains(&plan.log_basis_inner)
    {
        return Err(AkitaError::InvalidInput(
            "invalid full-ring physical source plan".into(),
        ));
    }
    let offerings = source.available_polynomial_types(plan)?;
    let selected = offerings.select(0)?;
    let representation = source.represent_as(selected, plan)?;
    let rings = plan
        .num_live_blocks
        .checked_mul(plan.num_positions_per_block)
        .ok_or_else(|| AkitaError::InvalidInput("full-ring source size overflow".into()))?;
    let depth = plan.num_digits_inner;
    let signed = |value: i8| {
        if value < 0 {
            -F::from_u64(u64::from(value.unsigned_abs()))
        } else {
            F::from_u64(value as u64)
        }
    };
    match representation {
        PolynomialRepresentation::Dense(DenseRepresentation::PredecomposedDigits(digits)) => {
            if digits.ring_dimension != 128
                || digits.num_digits != depth
                || digits.log_basis != plan.log_basis_inner
                || digits.logical_ring_count > rings
                || digits.bytes.len() != digits.logical_ring_count * depth * 128
            {
                return Err(AkitaError::InvalidInput(
                    "full-ring source digit plan mismatch".into(),
                ));
            }
            match &digits.bytes {
                crate::commitment::DigitPlaneStorage::Unpacked(bytes) => {
                    for (address, &value) in bytes.iter().enumerate() {
                        if value != 0 {
                            visitor(address, signed(value))?;
                        }
                    }
                }
                crate::commitment::DigitPlaneStorage::Packed(digits) => {
                    for (address, value) in digits.iter().enumerate() {
                        if value != 0 {
                            visitor(address, signed(value))?;
                        }
                    }
                }
            }
        }
        PolynomialRepresentation::Dense(DenseRepresentation::Coefficients(source)) => {
            let coefficients = source.coefficients();
            let params = BalancedDecomposePow2Params::new(depth, plan.log_basis_inner);
            let mut digits = vec![[0i8; 128]; depth];
            for ring in 0..rings {
                let value = CyclotomicRing::<F, 128>::from_coefficients(std::array::from_fn(|j| {
                    coefficients
                        .get(ring * 128 + j)
                        .copied()
                        .unwrap_or_else(F::zero)
                }));
                value.balanced_decompose_pow2_i8_into_with_params(&mut digits, &params);
                for (ell, plane) in digits.iter().enumerate() {
                    for (j, &digit) in plane.iter().enumerate() {
                        if digit != 0 {
                            visitor((ring * depth + ell) * 128 + j, signed(digit))?;
                        }
                    }
                }
            }
        }
        PolynomialRepresentation::ShortNorm(source) => {
            if depth != 1 || source.physical_coefficient_len() > rings * 128 {
                return Err(AkitaError::InvalidInput(
                    "full-ring packed source geometry mismatch".into(),
                ));
            }
            for (address, digit) in source.packed_view().iter().enumerate() {
                if digit != 0 {
                    visitor(address, signed(digit))?;
                }
            }
        }
        PolynomialRepresentation::OneHot(source) => {
            if depth != 1 {
                return Err(AkitaError::InvalidInput(
                    "one-hot full-ring sources require one digit".into(),
                ));
            }
            macro_rules! visit {
                ($positions:expr) => {
                    for (chunk, position) in $positions.iter().enumerate() {
                        if let Some(position) = position {
                            let address = chunk
                                .checked_mul(source.chunk_size)
                                .and_then(|x| x.checked_add(*position as usize))
                                .ok_or_else(|| {
                                    AkitaError::InvalidInput(
                                        "one-hot source address overflow".into(),
                                    )
                                })?;
                            if address >= rings * 128 {
                                return Err(AkitaError::InvalidInput(
                                    "one-hot source exceeds A plan".into(),
                                ));
                            }
                            visitor(address, F::one())?;
                        }
                    }
                };
            }
            match source.positions {
                UnitPositionSlice::U8(values) => visit!(values),
                UnitPositionSlice::U16(values) => visit!(values),
                UnitPositionSlice::U32(values) => visit!(values),
                UnitPositionSlice::Usize(values) => visit!(values),
            }
        }
    }
    Ok(())
}

impl<F: Field + CanonicalEncoding> FullRingSource<F> for crate::DensePoly<F> {
    fn visit_full_ring_digits(
        &self,
        plan: &CommitInnerPlan,
        visitor: &mut dyn FnMut(usize, F) -> Result<(), AkitaError>,
    ) -> Result<(), AkitaError> {
        visit_representation(self, plan, visitor)
    }
    fn full_ring_digit(&self, plan: &CommitInnerPlan, index: usize) -> Result<F, AkitaError> {
        check_digit_index(plan, index)?;
        if let Some(digits) =
            self.cached_digit_parts(128, plan.num_digits_inner, plan.log_basis_inner)
        {
            return Ok(signed_field(digits.get(index).copied().unwrap_or(0)));
        }
        let ring = index / (128 * plan.num_digits_inner);
        let ell = index / 128 % plan.num_digits_inner;
        let j = index % 128;
        let coefficient = self
            .field_coeffs()
            .get(ring * 128 + j)
            .copied()
            .unwrap_or_else(F::zero);
        let ring = CyclotomicRing::<F, 1>::from_coefficients([coefficient]);
        let mut digits = [[0i8; 1]; 128];
        ring.balanced_decompose_pow2_i8_into_with_params(
            &mut digits[..plan.num_digits_inner],
            &BalancedDecomposePow2Params::new(plan.num_digits_inner, plan.log_basis_inner),
        );
        Ok(signed_field(digits[ell][0]))
    }
}
impl<F: Field + CanonicalEncoding, I: crate::OneHotIndex> FullRingSource<F>
    for crate::OneHotPoly<F, I>
where
    Self: CommitmentSource<F>,
{
    fn visit_full_ring_digits(
        &self,
        plan: &CommitInnerPlan,
        visitor: &mut dyn FnMut(usize, F) -> Result<(), AkitaError>,
    ) -> Result<(), AkitaError> {
        visit_representation(self, plan, visitor)
    }
    fn full_ring_digit(&self, plan: &CommitInnerPlan, index: usize) -> Result<F, AkitaError> {
        check_digit_index(plan, index)?;
        if plan.num_digits_inner != 1 {
            return Err(AkitaError::InvalidInput(
                "one-hot source requires depth one".into(),
            ));
        }
        let hot = self
            .indices()
            .get(index / self.onehot_k())
            .copied()
            .flatten();
        Ok(
            if hot.is_some_and(|hot| hot.as_usize() == index % self.onehot_k()) {
                F::one()
            } else {
                F::zero()
            },
        )
    }
}
impl<F: Field + CanonicalEncoding> FullRingSource<F> for crate::opaque::RecursiveWitnessFlat {
    fn visit_full_ring_digits(
        &self,
        plan: &CommitInnerPlan,
        visitor: &mut dyn FnMut(usize, F) -> Result<(), AkitaError>,
    ) -> Result<(), AkitaError> {
        if !self.has_full_ring_extent() {
            return visit_representation(self, plan, visitor);
        }
        <Self as CommitmentSource<F>>::available_polynomial_types(self, plan)?;
        for (address, digit) in self.packed_digits().iter().enumerate() {
            if digit != 0 {
                visitor(address, signed_field(digit))?;
            }
        }
        Ok(())
    }
    fn full_ring_digit(&self, plan: &CommitInnerPlan, index: usize) -> Result<F, AkitaError> {
        check_digit_index(plan, index)?;
        self.validate_full_ring_plan(plan)?;
        if plan.num_digits_inner != 1 {
            return Err(AkitaError::InvalidInput(
                "recursive source requires depth one".into(),
            ));
        }
        Ok(signed_field(self.packed_digits().get(index).unwrap_or(0)))
    }
}

fn signed_field<F: Field>(digit: i8) -> F {
    if digit < 0 {
        -F::from_u64(u64::from(digit.unsigned_abs()))
    } else {
        F::from_u64(digit as u64)
    }
}

fn check_digit_index(plan: &CommitInnerPlan, index: usize) -> Result<(), AkitaError> {
    let size = plan
        .num_live_blocks
        .checked_mul(plan.num_positions_per_block)
        .and_then(|size| size.checked_mul(plan.num_digits_inner))
        .and_then(|size| size.checked_mul(128))
        .ok_or_else(|| AkitaError::InvalidInput("full-ring source extent overflow".into()))?;
    if plan.ring_dimension != 128
        || !(1..=128).contains(&plan.num_digits_inner)
        || !(1..=7).contains(&plan.log_basis_inner)
        || index >= size
    {
        return Err(AkitaError::InvalidInput(
            "full-ring source digit index/plan mismatch".into(),
        ));
    }
    Ok(())
}
