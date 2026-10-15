//! Subring coefficient packing opening points.
//!
//! The checked packing geometry lives in `layout/subring_packing_geometry.rs`.

use akita_error::{checked, AkitaError};
use jolt_field::Field;

#[cfg(test)]
use jolt_field::{Ring, Zero};

use akita_params::layout::subring_packing_geometry::SubringCoefficientPackingGeometry;
use akita_params::{basis_weights, basis_weights_prefix, BasisMode, SourceOpeningView};

/// Canonical opening-point split for subring coefficient packing.
///
/// The decoded source address order is `[r_pack | r_tail | r_M | r_B]`.
/// Original RAM planes instead take a public query in
/// `[r_coeff(7) | r_digit(5) | r_position(5) | r_block(4)]` order.
/// The digit axis is a separate, inverse-free Eq recipe; projecting it away
/// leaves the unchanged 16-coordinate source address. It never replaces any
/// physical source, opening, response, or quotient digit gadget.
///
/// Full address domains are retained except for the exact live block prefix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedSubringCoefficientPackingPoint<E: Field> {
    geometry: SubringCoefficientPackingGeometry,
    basis: BasisMode,
    source_num_vars: usize,
    source_digit_point: Option<[E; 5]>,
    source_digit_weights: Option<[E; 26]>,
    num_live_positions: usize,
    num_positions_per_block: usize,
    num_live_blocks: usize,
    packing_point: Vec<E>,
    tail_point: Vec<E>,
    position_point: Vec<E>,
    block_point: Vec<E>,
    packing_weights: Vec<E>,
    tail_weights: Vec<E>,
    position_weights: Vec<E>,
    live_block_weights: Vec<E>,
}

impl<E: Field> PreparedSubringCoefficientPackingPoint<E> {
    /// Split one public opening point into the canonical packing axes.
    pub fn new(
        geometry: SubringCoefficientPackingGeometry,
        basis: BasisMode,
        num_live_positions: usize,
        num_positions_per_block: usize,
        source_num_vars: usize,
        point: &[E],
    ) -> Result<Self, AkitaError> {
        if num_live_positions == 0 || !num_positions_per_block.is_power_of_two() {
            return Err(AkitaError::InvalidSetup(
                "coefficient-packing opening requires live positions and a power-of-two position domain"
                    .into(),
            ));
        }
        let num_live_blocks = num_live_positions.div_ceil(num_positions_per_block);
        let block_domain = num_live_blocks.checked_next_power_of_two().ok_or_else(|| {
            AkitaError::InvalidSetup("coefficient-packing block domain overflow".into())
        })?;
        let axis_bits = [
            geometry.subring_embedding_stride().trailing_zeros() as usize,
            geometry.challenge_subring_dimension().trailing_zeros() as usize,
            num_positions_per_block.trailing_zeros() as usize,
            block_domain.trailing_zeros() as usize,
        ];
        let expected = axis_bits.iter().try_fold(0usize, |sum, &bits| {
            sum.checked_add(bits).ok_or_else(|| {
                AkitaError::InvalidSetup("coefficient-packing point length overflow".into())
            })
        })?;
        if point.len() != source_num_vars {
            return Err(AkitaError::InvalidPointDimension {
                expected: source_num_vars,
                actual: point.len(),
            });
        }
        if source_num_vars > expected {
            return Err(AkitaError::InvalidSetup(
                "coefficient-packing source exceeds prepared opening domain".into(),
            ));
        }
        let source_domain = checked::pow2(source_num_vars).ok_or_else(|| {
            AkitaError::InvalidSetup("coefficient-packing source domain overflow".into())
        })?;
        let padded_ring_positions = source_domain.div_ceil(geometry.a_ring_dimension());
        if num_live_positions
            .checked_next_power_of_two()
            .filter(|&positions| positions == padded_ring_positions)
            .is_none()
        {
            return Err(AkitaError::InvalidSetup(
                "coefficient-packing live source prefix disagrees with source arity".into(),
            ));
        }
        let mut padded_point = point.to_vec();
        padded_point.resize(expected, E::zero());
        let mut offset = 0usize;
        let mut take_axis = |bits: usize| -> Result<&[E], AkitaError> {
            let end = offset.checked_add(bits).ok_or(AkitaError::InvalidProof)?;
            let axis = padded_point
                .get(offset..end)
                .ok_or(AkitaError::InvalidProof)?;
            offset = end;
            Ok(axis)
        };
        let packing_point = take_axis(axis_bits[0])?.to_vec();
        let tail_point = take_axis(axis_bits[1])?.to_vec();
        let position_point = take_axis(axis_bits[2])?.to_vec();
        let block_point = take_axis(axis_bits[3])?.to_vec();
        let packing_weights = basis_weights(&packing_point, basis)?;
        let tail_weights = basis_weights(&tail_point, basis)?;
        let position_weights = basis_weights(&position_point, basis)?;
        let live_block_weights = basis_weights_prefix(&block_point, basis, num_live_blocks)?;
        Ok(Self {
            geometry,
            basis,
            source_num_vars,
            source_digit_point: None,
            source_digit_weights: None,
            num_live_positions,
            num_positions_per_block,
            num_live_blocks,
            packing_point,
            tail_point,
            position_point,
            block_point,
            packing_weights,
            tail_weights,
            position_weights,
            live_block_weights,
        })
    }

    /// Prepare raw21 by projecting the registered address16 axes explicitly.
    /// Source digit depth remains 26; six virtual addresses never become planes.
    pub fn new_original_ram_planes(
        geometry: SubringCoefficientPackingGeometry,
        basis: BasisMode,
        num_live_positions: usize,
        num_positions_per_block: usize,
        source_num_vars: usize,
        point: &[E],
    ) -> Result<Self, AkitaError> {
        if geometry.extension_degree() != 1
            || geometry.a_ring_dimension() != 128
            || geometry.challenge_subring_dimension() != 128
            || geometry.subring_embedding_stride() != 1
            || basis != BasisMode::Lagrange
            || num_live_positions != 512
            || num_positions_per_block != 32
            || source_num_vars != 16
        {
            return Err(AkitaError::InvalidSetup(
                "original RAM point requires native128/k1h1/Lagrange/source16/N512/M32".into(),
            ));
        }
        if point.len() != 21 {
            return Err(AkitaError::InvalidPointDimension { expected: 21, actual: point.len() });
        }
        let digit_point: [E; 5] = point.get(7..12)
            .ok_or(AkitaError::InvalidProof)?.try_into()
            .map_err(|_| AkitaError::InvalidProof)?;
        let mut address = [E::zero(); 16];
        address[..7].copy_from_slice(point.get(..7).ok_or(AkitaError::InvalidProof)?);
        address[7..].copy_from_slice(point.get(12..).ok_or(AkitaError::InvalidProof)?);
        let mut prepared = Self::new(
            geometry, basis, num_live_positions, num_positions_per_block, source_num_vars, &address,
        )?;
        let mut weights = [E::zero(); 26];
        weights[0] = E::one();
        for (bit, &coordinate) in digit_point.iter().enumerate() {
            let width = 1usize << bit;
            let complement = E::one() - coordinate;
            // Five fixed bits give width <= 16; all lower indices are real,
            // and upper indices 26..31 are virtual, never physical planes.
            for lower in 0..width {
                let value = weights[lower];
                weights[lower] = value * complement;
                if lower + width < weights.len() {
                    weights[lower + width] = value * coordinate;
                }
            }
        }
        prepared.source_digit_point = Some(digit_point);
        prepared.source_digit_weights = Some(weights);
        Ok(prepared)
    }

    pub const fn source_view(&self) -> SourceOpeningView {
        match self.source_digit_point {
            Some(_) => SourceOpeningView::OriginalRamPlanesV1,
            None => SourceOpeningView::Decoded,
        }
    }

    pub const fn point_num_vars(&self) -> usize {
        match self.source_view() {
            SourceOpeningView::Decoded => self.source_num_vars,
            SourceOpeningView::OriginalRamPlanesV1 => 21,
        }
    }

    /// Exact canonical Eq recipe, including legitimate zero factors.
    pub fn source_digit_point(&self) -> Option<&[E; 5]> {
        self.source_digit_point.as_ref()
    }

    pub fn source_digit_weights(&self) -> Option<&[E; 26]> {
        self.source_digit_weights.as_ref()
    }

    /// Query factor only. Never use this to recompose source/A or response digits.
    pub fn source_digit_weight(&self, digit: usize, log_basis: u32) -> Result<E, AkitaError> {
        if let Some(weights) = &self.source_digit_weights {
            if log_basis != 5 {
                return Err(AkitaError::InvalidSetup("raw digit weights require original base32".into()));
            }
            return weights.get(digit).copied().ok_or(AkitaError::InvalidProof);
        }
        if log_basis >= 128 {
            return Err(AkitaError::InvalidSetup("source digit basis overflow".into()));
        }
        let mut power = E::one();
        for _ in 0..log_basis {
            power += power;
        }
        let mut exponent = digit;
        let mut weight = E::one();
        while exponent != 0 {
            if exponent & 1 == 1 {
                weight *= power;
            }
            exponent >>= 1;
            if exponent != 0 {
                power *= power;
            }
        }
        Ok(weight)
    }

    /// Checked coefficient-packing geometry.
    pub const fn geometry(&self) -> SubringCoefficientPackingGeometry {
        self.geometry
    }

    /// Polynomial basis used for every tensor-product opening axis.
    pub const fn basis(&self) -> BasisMode {
        self.basis
    }

    /// Authenticated public-point arity before preparation-only padding.
    pub const fn source_num_vars(&self) -> usize {
        self.source_num_vars
    }

    /// Number of live A-ring positions in the source.
    pub const fn num_live_positions(&self) -> usize {
        self.num_live_positions
    }

    /// Fixed position domain within each block.
    pub const fn num_positions_per_block(&self) -> usize {
        self.num_positions_per_block
    }

    /// Number of live partial-opening blocks.
    pub const fn num_live_blocks(&self) -> usize {
        self.num_live_blocks
    }

    /// Public point coordinates for the low A-ring coefficient axis.
    pub fn packing_point(&self) -> &[E] {
        &self.packing_point
    }

    /// Public point coordinates for the challenge-subring coefficient axis.
    pub fn tail_point(&self) -> &[E] {
        &self.tail_point
    }

    /// Public point coordinates for positions within one block.
    pub fn position_point(&self) -> &[E] {
        &self.position_point
    }

    /// Public point coordinates for the padded block domain.
    pub fn block_point(&self) -> &[E] {
        &self.block_point
    }

    /// Weights for the low A-ring coefficient axis.
    pub fn packing_weights(&self) -> &[E] {
        &self.packing_weights
    }

    /// Weights for the challenge-subring coefficient axis.
    pub fn tail_weights(&self) -> &[E] {
        &self.tail_weights
    }

    /// Weights for positions within one block.
    pub fn position_weights(&self) -> &[E] {
        &self.position_weights
    }

    /// Exact live prefix of the padded block-domain weights.
    pub fn live_block_weights(&self) -> &[E] {
        &self.live_block_weights
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jolt_field::{One, Prime128OffsetA7F7};

    #[test]
    fn opening_point_uses_pack_tail_position_block_order() {
        type F = Prime128OffsetA7F7;
        let geometry = SubringCoefficientPackingGeometry::try_new(1, 128, 64).unwrap();
        // log(kh)=1, log(s)=6, log(M)=2, log(B-domain)=1.
        let point = (1..=10).map(F::from_u64).collect::<Vec<_>>();
        for basis in [BasisMode::Lagrange, BasisMode::Monomial] {
            let prepared =
                PreparedSubringCoefficientPackingPoint::new(geometry, basis, 6, 4, 10, &point)
                    .unwrap();
            assert_eq!(prepared.basis(), basis);
            assert_eq!(
                prepared.packing_weights(),
                basis_weights(&point[..1], basis).unwrap()
            );
            assert_eq!(
                prepared.tail_weights(),
                basis_weights(&point[1..7], basis).unwrap()
            );
            assert_eq!(
                prepared.position_weights(),
                basis_weights(&point[7..9], basis).unwrap()
            );
            assert_eq!(
                prepared.live_block_weights(),
                basis_weights_prefix(&point[9..], basis, 2).unwrap()
            );
            assert_eq!(prepared.num_live_blocks(), 2);
        }
        assert!(PreparedSubringCoefficientPackingPoint::new(
            geometry,
            BasisMode::Lagrange,
            6,
            4,
            10,
            &point[..9],
        )
        .is_err());
        assert!(PreparedSubringCoefficientPackingPoint::new(
            geometry,
            BasisMode::Lagrange,
            6,
            4,
            10,
            &[point.as_slice(), &[F::zero()]].concat(),
        )
        .is_err());

        let short_source_geometry = SubringCoefficientPackingGeometry::try_new(4, 256, 64).unwrap();
        let short_source_point = point[..9].to_vec();
        let padded = PreparedSubringCoefficientPackingPoint::new(
            short_source_geometry,
            BasisMode::Lagrange,
            2,
            4,
            9,
            &short_source_point,
        )
        .unwrap();
        assert_eq!(padded.num_live_blocks(), 1);
        assert!(PreparedSubringCoefficientPackingPoint::new(
            short_source_geometry,
            BasisMode::Lagrange,
            2,
            4,
            9,
            &short_source_point[..8],
        )
        .is_err());

        let low_arity_geometry = SubringCoefficientPackingGeometry::try_new(1, 128, 64).unwrap();
        let low_arity_point = point[..6].to_vec();
        assert!(PreparedSubringCoefficientPackingPoint::new(
            low_arity_geometry,
            BasisMode::Lagrange,
            1,
            1,
            6,
            &low_arity_point,
        )
        .is_ok());
    }

    #[test]
    fn raw_point_preserves_address_geometry_and_all_physical_digit_weights() {
        type F = Prime128OffsetA7F7;
        let geometry = SubringCoefficientPackingGeometry::try_new(1, 128, 128).unwrap();
        let point = (1..=21).map(F::from_u64).collect::<Vec<_>>();
        let raw = PreparedSubringCoefficientPackingPoint::new_original_ram_planes(
            geometry, BasisMode::Lagrange, 512, 32, 16, &point,
        ).unwrap();
        assert_eq!(raw.source_view(), SourceOpeningView::OriginalRamPlanesV1);
        assert_eq!(raw.source_num_vars(), 16);
        assert_eq!(raw.point_num_vars(), 21);
        assert_eq!(raw.tail_point(), &point[..7]);
        assert_eq!(raw.position_point(), &point[12..17]);
        assert_eq!(raw.block_point(), &point[17..]);
        let expected = basis_weights(&point[7..12], BasisMode::Lagrange).unwrap();
        for (digit, &weight) in expected[..26].iter().enumerate() {
            assert_eq!(raw.source_digit_weight(digit, 5).unwrap(), weight);
        }
        assert!(raw.source_digit_weight(26, 5).is_err());
        assert!(raw.source_digit_weight(0, 4).is_err());

        // Boolean digit 25 makes weight0 vanish and preserves the last real plane.
        let mut point = vec![F::zero(); 21];
        for bit in [0, 3, 4] {
            point[7 + bit] = F::one();
        }
        let raw = PreparedSubringCoefficientPackingPoint::new_original_ram_planes(
            geometry, BasisMode::Lagrange, 512, 32, 16, &point,
        ).unwrap();
        assert_eq!(raw.source_digit_weight(0, 5).unwrap(), F::zero());
        assert_eq!(raw.source_digit_weight(25, 5).unwrap(), F::one());
        // All six absent addresses evaluate to zero under the physical recipe.
        for absent in 26..32 {
            for bit in 0..5 {
                point[7 + bit] = F::from_u64(((absent >> bit) & 1) as u64);
            }
            let raw = PreparedSubringCoefficientPackingPoint::new_original_ram_planes(
                geometry, BasisMode::Lagrange, 512, 32, 16, &point,
            ).unwrap();
            assert!(raw.source_digit_weights().unwrap().iter().all(|w| w.is_zero()));
        }
    }

    #[test]
    fn raw_point_rejects_unregistered_geometry_and_basis() {
        type F = Prime128OffsetA7F7;
        let native = SubringCoefficientPackingGeometry::try_new(1, 128, 128).unwrap();
        let point = vec![F::zero(); 21];
        for (basis, live, positions, source_vars, len) in [
            (BasisMode::Monomial, 512, 32, 16, 21),
            (BasisMode::Lagrange, 256, 32, 16, 21),
            (BasisMode::Lagrange, 512, 16, 16, 21),
            (BasisMode::Lagrange, 512, 32, 21, 21),
            (BasisMode::Lagrange, 512, 32, 16, 16),
        ] {
            assert!(PreparedSubringCoefficientPackingPoint::new_original_ram_planes(
                native, basis, live, positions, source_vars, &point[..len],
            ).is_err());
        }
        for geometry in [
            SubringCoefficientPackingGeometry::try_new(2, 128, 64).unwrap(),
            SubringCoefficientPackingGeometry::try_new(1, 128, 64).unwrap(),
        ] {
            assert!(PreparedSubringCoefficientPackingPoint::new_original_ram_planes(
                geometry, BasisMode::Lagrange, 512, 32, 16, &point,
            ).is_err());
        }
        assert!(PreparedSubringCoefficientPackingPoint::new(
            native, BasisMode::Lagrange, 512, 32, 16, &point,
        ).is_err());
        let decoded = PreparedSubringCoefficientPackingPoint::new(
            native, BasisMode::Monomial, 512, 32, 16, &point[..16],
        ).unwrap();
        assert_eq!(decoded.source_view(), SourceOpeningView::Decoded);
        assert_eq!(decoded.source_digit_weight(2, 5).unwrap(), F::from_u64(1024));
    }
}
