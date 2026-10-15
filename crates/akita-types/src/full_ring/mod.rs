//! Full-ring scalar-extension semantics on the original K coefficient source.
//!
//! These types do not implement a ring-subfield encoding. The low-to-high raw
//! axes are (coefficient, source digit, position, block, polynomial). Virtual
//! padding is a view, never a new commitment column. Owners must bind this plan
//! and its decoder/domain metadata to the original source handle.

use std::sync::Arc;

use akita_algebra::eq_poly::EqPolynomial;
use akita_error::AkitaError;
use jolt_field::{ExtField, Field};

pub mod native_context;
pub use native_context::*;
pub mod query;
pub mod relation;
pub mod strong;
pub use query::*;
pub use relation::*;

/// Validate the selected D1 scalar algebra, not merely its advertised degree.
/// This is required at selected-family admission; the generic geometric
/// containers also support small reference geometries.
pub fn validate_full_ring_fields<K, E>() -> Result<(), AkitaError>
where
    K: Field + jolt_field::CanonicalEncoding,
    E: ExtField<K>,
{
    const Q: u128 = u128::MAX - (1u128 << 32) + 22538;
    let mut expected = [0u8; 32];
    expected[16..].copy_from_slice(&Q.to_be_bytes());
    if akita_params::field_modulus_be_bytes::<K>()? != expected || E::DEGREE != 2 {
        return Err(invalid(
            "full-ring D1 requires the q128 base field and degree two",
        ));
    }
    let u = E::from_base_fn(|i| if i == 1 { K::one() } else { K::zero() });
    let one = E::from_base_fn(|i| if i == 0 { K::one() } else { K::zero() });
    if one != E::one()
        || u * u != E::lift_base(K::from_u64(5))
        || u.base_coefficient(0) != K::zero()
        || u.base_coefficient(1) != K::one()
        || one.base_coefficient(0) != K::one()
        || one.base_coefficient(1) != K::zero()
    {
        return Err(invalid(
            "full-ring D1 requires the canonical nonresidue-five basis",
        ));
    }
    Ok(())
}
pub use strong::*;

pub(crate) fn invalid(message: &str) -> AkitaError {
    AkitaError::InvalidInput(message.into())
}

pub(crate) fn product(values: &[usize]) -> Result<usize, AkitaError> {
    values.iter().try_fold(1usize, |a, &b| {
        a.checked_mul(b)
            .ok_or_else(|| invalid("full-ring geometry overflow"))
    })
}

/// Checked injection from a padded raw Boolean grid into the frozen physical
/// source. An optional permutation maps canonical physical indices to storage
/// indices; cloning the plan shares that public map rather than copying it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawSourceViewPlan {
    degree: usize,
    polynomials: usize,
    blocks: usize,
    positions: usize,
    digits: usize,
    padded_polynomials: usize,
    padded_digits: usize,
    physical_len: usize,
    raw_len: usize,
    permutation: Option<Arc<[usize]>>,
}

impl RawSourceViewPlan {
    pub fn new(
        degree: usize,
        polynomials: usize,
        blocks: usize,
        positions: usize,
        digits: usize,
    ) -> Result<Self, AkitaError> {
        if !degree.is_power_of_two()
            || !blocks.is_power_of_two()
            || !positions.is_power_of_two()
            || polynomials == 0
            || digits == 0
        {
            return Err(invalid("invalid full-ring raw axes"));
        }
        let padded_polynomials = polynomials
            .checked_next_power_of_two()
            .ok_or_else(|| invalid("raw polynomial padding overflow"))?;
        let padded_digits = digits
            .checked_next_power_of_two()
            .ok_or_else(|| invalid("raw digit padding overflow"))?;
        let physical_len = product(&[degree, polynomials, blocks, positions, digits])?;
        let raw_len = product(&[degree, padded_polynomials, blocks, positions, padded_digits])?;
        Ok(Self {
            degree,
            polynomials,
            blocks,
            positions,
            digits,
            padded_polynomials,
            padded_digits,
            physical_len,
            raw_len,
            permutation: None,
        })
    }

    /// Supply the registered physical permutation, never an honest-source
    /// sparsity map. Every physical cell must occur exactly once.
    pub fn with_permutation(mut self, permutation: Vec<usize>) -> Result<Self, AkitaError> {
        if permutation.len() != self.physical_len {
            return Err(invalid("raw physical permutation length"));
        }
        let mut seen = vec![false; self.physical_len];
        for &index in &permutation {
            let cell = seen
                .get_mut(index)
                .ok_or_else(|| invalid("raw permutation index"))?;
            if *cell {
                return Err(invalid("raw permutation is not injective"));
            }
            *cell = true;
        }
        self.permutation = Some(permutation.into());
        Ok(self)
    }

    /// Admission for one shared full-ring L after the original physical fold.
    /// This is deliberately stronger than raw-view/normalization admission.
    ///
    /// Write b = p*B+i and a = x*delta+ell. The registered unsigned
    /// permutation must factor as pi(b,a,j) = (beta(b),sigma(a),j), where
    /// beta permutes whole original blocks and sigma is the SAME ring-column
    /// permutation in every block. Sigma may mix position and source digit.
    /// Preserving j makes the pullback S-linear: it commutes with every
    /// negacyclic shell, including its wrap sign. An unsigned within-ring
    /// coefficient permutation cannot commute with Y unless it fixes j.
    /// Independence from b is required for a shared L on the one response.
    ///
    /// Partials retain raw block order; shells retain physical block order.
    /// The paired equation is L(z) = sum_b c[beta(b)] * e[b].
    /// General permutations remain legal raw views for normalization, but
    /// need a different carrier/channel construction to cross this boundary.
    pub fn validate_fold_admission(&self) -> Result<(), AkitaError> {
        let Some(map) = &self.permutation else {
            return Ok(());
        };
        let block_len = self.positions * self.digits * self.degree;
        for (block, chunk) in map.chunks_exact(block_len).enumerate() {
            let physical_block = self.fold_block(block);
            for (raw_column, ring) in chunk.chunks_exact(self.degree).enumerate() {
                let start = physical_block * block_len + self.fold_column(raw_column) * self.degree;
                if ring
                    .iter()
                    .enumerate()
                    .any(|(j, &physical)| physical != start + j)
                {
                    return Err(invalid("raw fold map must preserve ring coefficients and factor into a block permutation and a common ring-column permutation"));
                }
            }
        }
        Ok(())
    }

    // Only used after validate_fold_admission, with checked geometry indices.
    // Borrow the original shared map; do not copy source-sized metadata or
    // attach a raw permutation to the physical response/A digit layout.
    pub(super) fn fold_column(&self, column: usize) -> usize {
        self.permutation.as_ref().map_or(column, |map| {
            (map[column * self.degree] / self.degree) % (self.positions * self.digits)
        })
    }
    pub(super) fn fold_block(&self, block: usize) -> usize {
        let block_len = self.positions * self.digits * self.degree;
        self.permutation
            .as_ref()
            .map_or(block, |map| map[block * block_len] / block_len)
    }

    pub fn degree(&self) -> usize {
        self.degree
    }
    pub fn polynomials(&self) -> usize {
        self.polynomials
    }
    pub fn blocks(&self) -> usize {
        self.blocks
    }
    pub fn positions(&self) -> usize {
        self.positions
    }
    pub fn digits(&self) -> usize {
        self.digits
    }
    pub fn padded_polynomials(&self) -> usize {
        self.padded_polynomials
    }
    pub fn padded_digits(&self) -> usize {
        self.padded_digits
    }
    pub fn physical_len(&self) -> usize {
        self.physical_len
    }
    pub fn raw_len(&self) -> usize {
        self.raw_len
    }
    pub fn num_vars(&self) -> usize {
        self.raw_len.trailing_zeros() as usize
    }
    pub fn permutation(&self) -> Option<&[usize]> {
        self.permutation.as_deref()
    }

    fn check_cell(
        &self,
        p: usize,
        i: usize,
        x: usize,
        ell: usize,
        j: usize,
    ) -> Result<(), AkitaError> {
        if p >= self.polynomials
            || i >= self.blocks
            || x >= self.positions
            || ell >= self.digits
            || j >= self.degree
        {
            return Err(invalid("physical raw coordinate out of bounds"));
        }
        Ok(())
    }

    pub fn physical_index(
        &self,
        p: usize,
        i: usize,
        x: usize,
        ell: usize,
        j: usize,
    ) -> Result<usize, AkitaError> {
        self.check_cell(p, i, x, ell, j)?;
        let index =
            ((((p * self.blocks + i) * self.positions + x) * self.digits + ell) * self.degree) + j;
        Ok(self.permutation.as_ref().map_or(index, |map| map[index]))
    }

    pub fn raw_index(
        &self,
        p: usize,
        i: usize,
        x: usize,
        ell: usize,
        j: usize,
    ) -> Result<usize, AkitaError> {
        self.check_cell(p, i, x, ell, j)?;
        Ok(j + self.degree
            * (ell + self.padded_digits * (x + self.positions * (i + self.blocks * p))))
    }

    /// `None` means a virtual zero, not a physical required-zero assertion.
    pub fn raw_to_physical(&self, index: usize) -> Result<Option<usize>, AkitaError> {
        if index >= self.raw_len {
            return Err(invalid("raw index out of bounds"));
        }
        let j = index % self.degree;
        let rest = index / self.degree;
        let ell = rest % self.padded_digits;
        let rest = rest / self.padded_digits;
        let x = rest % self.positions;
        let rest = rest / self.positions;
        let i = rest % self.blocks;
        let p = rest / self.blocks;
        if p >= self.polynomials || ell >= self.digits {
            return Ok(None);
        }
        self.physical_index(p, i, x, ell, j).map(Some)
    }
}

/// A raw point with reusable equality tables for each independent axis. No
/// source digits or tensor/subfield factors are stored here.
#[derive(Clone, Debug)]
pub struct FullRingRawPoint<E> {
    view: RawSourceViewPlan,
    point: Vec<E>,
    coefficient_weights: Vec<E>,
    digit_weights: Vec<E>,
    position_weights: Vec<E>,
    block_weights: Vec<E>,
    polynomial_weights: Vec<E>,
}

impl<E: Field> FullRingRawPoint<E> {
    pub fn new(view: &RawSourceViewPlan, point: Vec<E>) -> Result<Self, AkitaError> {
        if point.len() != view.num_vars() {
            return Err(invalid("raw point length"));
        }
        let mut start = 0;
        let mut axis = |size: usize| -> Result<Vec<E>, AkitaError> {
            let end = start + size.trailing_zeros() as usize;
            let weights = EqPolynomial::evals(&point[start..end])?;
            start = end;
            Ok(weights)
        };
        let coefficient_weights = axis(view.degree)?;
        let digit_weights = axis(view.padded_digits)?;
        let position_weights = axis(view.positions)?;
        let block_weights = axis(view.blocks)?;
        let polynomial_weights = axis(view.padded_polynomials)?;
        Ok(Self {
            view: view.clone(),
            point,
            coefficient_weights,
            digit_weights,
            position_weights,
            block_weights,
            polynomial_weights,
        })
    }
    pub fn view(&self) -> &RawSourceViewPlan {
        &self.view
    }
    pub fn point(&self) -> &[E] {
        &self.point
    }
    pub fn coefficient_weights(&self) -> &[E] {
        &self.coefficient_weights
    }
    pub fn digit_weights(&self) -> &[E] {
        &self.digit_weights
    }
    pub fn position_weights(&self) -> &[E] {
        &self.position_weights
    }
    pub fn block_weights(&self) -> &[E] {
        &self.block_weights
    }
    pub fn polynomial_weights(&self) -> &[E] {
        &self.polynomial_weights
    }

    pub fn scalar_from_partials<K: Field>(
        &self,
        partials: &FullRingPartials<K>,
    ) -> Result<E, AkitaError>
    where
        E: ExtField<K>,
    {
        if partials.view != self.view || partials.extension_degree != E::DEGREE {
            return Err(invalid("full-ring partial/point mismatch"));
        }
        self.scalar_from_coefficients(partials.coefficients(), partials.extension_degree())
    }

    /// Borrowed-wire variant; never copies a retained partial table.
    pub fn scalar_from_coefficients<K: Field>(
        &self,
        coefficients: &[K],
        extension_degree: usize,
    ) -> Result<E, AkitaError>
    where
        E: ExtField<K>,
    {
        if extension_degree != E::DEGREE
            || coefficients.len()
                != product(&[
                    self.view.polynomials,
                    self.view.blocks,
                    extension_degree,
                    self.view.degree,
                ])?
        {
            return Err(invalid("full-ring scalar partial geometry"));
        }
        let mut value = E::zero();
        for p in 0..self.view.polynomials {
            for i in 0..self.view.blocks {
                let block_weight = self.polynomial_weights[p] * self.block_weights[i];
                for j in 0..self.view.degree {
                    let coefficient = E::from_base_fn(|plane| {
                        coefficients[((p * self.view.blocks + i) * E::DEGREE + plane)
                            * self.view.degree
                            + j]
                    });
                    value += block_weight * self.coefficient_weights[j] * coefficient;
                }
            }
        }
        Ok(value)
    }
}

/// Base-field storage of complete scalar-extension ring partials. Layout is
/// [polynomial][block][extension plane][native coefficient], not a ring of
/// degree `degree * extension_degree`.
#[derive(Clone, Debug)]
pub struct FullRingPartials<K> {
    view: RawSourceViewPlan,
    extension_degree: usize,
    coefficients: Vec<K>,
}

impl<K> FullRingPartials<K> {
    pub fn new(
        view: RawSourceViewPlan,
        extension_degree: usize,
        coefficients: Vec<K>,
    ) -> Result<Self, AkitaError> {
        if extension_degree == 0
            || coefficients.len()
                != product(&[view.polynomials, view.blocks, extension_degree, view.degree])?
        {
            return Err(invalid("full-ring partial geometry"));
        }
        Ok(Self {
            view,
            extension_degree,
            coefficients,
        })
    }
    pub fn view(&self) -> &RawSourceViewPlan {
        &self.view
    }
    pub fn extension_degree(&self) -> usize {
        self.extension_degree
    }
    pub fn coefficients(&self) -> &[K] {
        &self.coefficients
    }
    pub fn into_coefficients(self) -> Vec<K> {
        self.coefficients
    }
    pub fn plane(&self, p: usize, i: usize, plane: usize) -> Result<&[K], AkitaError> {
        if p >= self.view.polynomials || i >= self.view.blocks || plane >= self.extension_degree {
            return Err(invalid("full-ring partial coordinate"));
        }
        let start = ((p * self.view.blocks + i) * self.extension_degree + plane) * self.view.degree;
        Ok(&self.coefficients[start..start + self.view.degree])
    }
    pub fn coordinate(&self, p: usize, i: usize, plane: usize, j: usize) -> Result<K, AkitaError>
    where
        K: Copy,
    {
        self.plane(p, i, plane)?
            .get(j)
            .copied()
            .ok_or_else(|| invalid("partial coefficient index"))
    }
}

#[cfg(test)]
mod tests;
