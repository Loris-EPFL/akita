//! Exact terminal scalar, A and both-plane L equations. No ring-switch alpha.
use akita_algebra::CyclotomicRing;
use akita_challenges::Challenges;
use akita_error::AkitaError;
use akita_params::{TerminalFoldParams, TerminalResponseCodec};
use akita_types::full_ring::FullRingRawPoint;
use akita_types::full_ring_terminal::{FullRingTerminalCodec, FullRingTerminalState};
use akita_types::TerminalResponse;
use jolt_field::{CanonicalEncoding, ExtField, Field, Ring};

/// Discharge a selected terminal obligation after the native transcript has
/// bound all clear partials and sampled the complete original-shell tuple.
/// The owning continuation must additionally reject trailing proof bytes.
pub fn verify_full_ring_terminal_suffix<F, E>(
    setup: &akita_types::InstalledFullRingSetup<F>,
    state: &FullRingTerminalState<F, E>,
    scheduled: &TerminalFoldParams,
    z_payload: Vec<u8>,
    partials: akita_types::RingVec<F>,
    challenges: &Challenges,
) -> Result<(), AkitaError>
where
    F: Field + CanonicalEncoding + akita_serialization::AkitaSerialize + Ring,
    E: ExtField<F>,
{
    akita_types::full_ring::validate_full_ring_fields::<F, E>()?;
    scheduled.validate_opening_execution(E::DEGREE)?;
    if state.pending_setup
        || state.witness_len != scheduled.input_witness_len
        || state.binding != akita_params::NextWitnessBindingPolicy::TerminalInnerState
        || scheduled.response_shape.layout.groups.len() != 1
    {
        return Err(AkitaError::InvalidProof);
    }
    let group = scheduled
        .response_shape
        .layout
        .groups
        .first()
        .ok_or(AkitaError::InvalidProof)?;
    if state.witness.coeff_len() != group.t_field_elems
        || !state.witness.can_decode_vec(128)
        || partials.coeff_len() != group.e_field_elems
    {
        return Err(AkitaError::InvalidProof);
    }
    let view = akita_types::full_ring::RawSourceViewPlan::new(
        128,
        1,
        scheduled.blocks.live_blocks,
        scheduled.blocks.positions_per_block,
        1,
    )?;
    if state.opening_point.len() != view.num_vars() {
        return Err(AkitaError::InvalidProof);
    }
    let point = FullRingRawPoint::new(&view, state.opening_point.clone())?;
    let width = scheduled.inner.matrix.input_width();
    if state.predecessor_level.checked_add(1) != Some(setup.descriptor().levels().len()) {
        return Err(AkitaError::InvalidProof);
    }
    let akita_types::FullRingSuccessor::Terminal { a: role } =
        setup.descriptor().successor(state.predecessor_level)?
    else {
        return Err(AkitaError::InvalidProof);
    };
    let descriptor = setup.descriptor().matrix(role)?;
    let rank = scheduled.inner.matrix.output_rank();
    if descriptor.ring_dimension != 128 || descriptor.rows != rank || descriptor.columns != width {
        return Err(AkitaError::InvalidProof);
    }
    let matrix = setup.matrix(role)?.ring_view::<128>(rank, width)?;
    let cache = akita_types::prepare_joined_exact_ntt_cache(
        matrix,
        width,
        crate::prepared_cache::TERMINAL_I16_ABS_BOUND,
    )?;
    let response = TerminalResponse {
        layout: scheduled.response_shape.layout.clone(),
        z_payloads: vec![z_payload],
        e_fields: partials,
        t_fields: state.witness.clone(),
    };
    verify(
        &cache,
        scheduled,
        &point,
        state.opening,
        challenges,
        &response,
    )
}
/// Checks the descriptor-bound identity table, integer codec and every terminal
/// equation against the original A images. Transcript replay must absorb those
/// images and these partials before producing `challenges`.
pub(crate) fn verify<F, E>(
    terminal_ntt: &akita_types::PreparedNttCache<128>,
    params: &TerminalFoldParams,
    point: &FullRingRawPoint<E>,
    target: E,
    challenges: &Challenges,
    response: &TerminalResponse<F>,
) -> Result<(), AkitaError>
where
    F: Field + CanonicalEncoding + akita_serialization::AkitaSerialize + Ring,
    E: ExtField<F>,
{
    params.validate_opening_execution(E::DEGREE)?;
    if !params.opening_method.is_full_ring_scalar_extension()
        || params.d_a() != 128
        || E::DEGREE != 2
        || response.layout != params.response_shape.layout
        || response.layout.groups.len() != 1
        || response.z_payloads.len() != 1
    {
        return Err(AkitaError::InvalidProof);
    }
    let view = point.view();
    if view.degree() != 128
        || view.polynomials() != 1
        || view.digits() != 1
        || view.blocks() != params.blocks.live_blocks
        || view.positions() != params.blocks.positions_per_block
        || view.physical_len() != params.input_witness_len
        || params.inner.matrix.input_width() != view.positions()
        || challenges.num_claims() != 1
        || challenges.num_live_blocks_per_claim() != view.blocks()
    {
        return Err(AkitaError::InvalidProof);
    }
    let group = response
        .layout
        .groups
        .first()
        .ok_or(AkitaError::InvalidProof)?;
    let codec =
        FullRingTerminalCodec::new(view.blocks(), group.z_coords, u128::MAX - 0xFFFF_A7F7 + 1)?;
    if params.response_codec
        != (TerminalResponseCodec::SignedFixedWidth {
            bytes_per_coefficient: codec.bytes_per_coefficient(),
            coefficient_bound: codec.coefficient_bound(),
        })
        || group.z_payload_bytes != codec.payload_bytes()
        || group.z_coords
            != view
                .positions()
                .checked_mul(128)
                .ok_or(AkitaError::InvalidProof)?
    {
        return Err(AkitaError::InvalidProof);
    }
    if point.scalar_from_coefficients::<F>(response.e_fields.coeffs(), 2)? != target {
        return Err(AkitaError::InvalidProof);
    }
    let partials = response.e_fields.as_ring_slice::<128>()?;
    let z_values = codec.decode(
        response
            .z_payloads
            .first()
            .ok_or(AkitaError::InvalidProof)?,
    )?;
    let (z, remainder) = z_values.as_chunks::<128>();
    if !remainder.is_empty() {
        return Err(AkitaError::InvalidProof);
    }
    let rank = params.inner.matrix.output_rank();
    let t = response.t_fields.as_ring_slice::<128>()?;
    if t.len()
        != view
            .blocks()
            .checked_mul(rank)
            .ok_or(AkitaError::InvalidProof)?
    {
        return Err(AkitaError::InvalidProof);
    }
    let a_z = super::ntt::wide_centered_rows::<F, 128>(terminal_ntt, rank, z)?;
    let mut folded_a = vec![CyclotomicRing::<F, 128>::zero(); rank];
    let mut folded_l = [CyclotomicRing::<F, 128>::zero(); 2];
    for (block, challenge) in challenges.as_slice().iter().enumerate() {
        challenge.validate::<128>()?;
        if challenge
            .coeffs
            .iter()
            .filter(|c| c.unsigned_abs() == 1)
            .count()
            != 24
            || challenge
                .coeffs
                .iter()
                .filter(|c| c.unsigned_abs() == 2)
                .count()
                != 16
            || challenge.coeffs.len() != 40
        {
            return Err(AkitaError::InvalidProof);
        }
        for (row, out) in folded_a.iter_mut().enumerate() {
            super::direct::sparse_challenge_mul_accumulate(
                challenge,
                t.get(block * rank + row).ok_or(AkitaError::InvalidProof)?,
                out,
            )?;
        }
        for (plane, out) in folded_l.iter_mut().enumerate() {
            super::direct::sparse_challenge_mul_accumulate(
                challenge,
                partials
                    .get(block * 2 + plane)
                    .ok_or(AkitaError::InvalidProof)?,
                out,
            )?;
        }
    }
    if folded_a != a_z {
        return Err(AkitaError::InvalidProof);
    }
    let mut l_z = [CyclotomicRing::<F, 128>::zero(); 2];
    for (values, weight) in z.iter().zip(point.position_weights()) {
        let ring = CyclotomicRing::from_coefficients(core::array::from_fn(|j| {
            let magnitude = F::from_u128(values[j].unsigned_abs());
            if values[j] < 0 {
                -magnitude
            } else {
                magnitude
            }
        }));
        for (plane, out) in l_z.iter_mut().enumerate() {
            *out += ring.scale(&weight.base_coefficient(plane));
        }
    }
    if l_z != folded_l {
        return Err(AkitaError::InvalidProof);
    }
    Ok(())
}
