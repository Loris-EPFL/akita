//! One terminal chronology for both native compilers and both proof parties.
use crate::RingVec;
use akita_challenges::Challenges;
use akita_error::AkitaError;
use akita_params::{NextWitnessBindingPolicy, TerminalFoldParams};
use akita_transcript::native_strong::{NativeStrongTranscript, TerminalSites};
use jolt_field::{CanonicalEncoding, ExtField, Field};

/// Already authenticated predecessor leaf and original A-only source binding.
/// These fields are retained verifier/prover state, not terminal proof input.
pub struct FullRingTerminalState<F: Field, E: Field> {
    /// Installed nonterminal occurrence that produced this A-only source.
    pub predecessor_level: usize,
    pub witness: RingVec<F>,
    pub witness_len: usize,
    pub opening_point: Vec<E>,
    pub opening: E,
    pub binding: NextWitnessBindingPolicy,
    pub pending_setup: bool,
}

fn base_bytes<F: Field + CanonicalEncoding>(values: &[F]) -> Result<Vec<u8>, AkitaError> {
    let length = values
        .len()
        .checked_mul(16)
        .ok_or(AkitaError::InvalidProof)?;
    let mut bytes = vec![0; length];
    for (value, out) in values.iter().zip(bytes.chunks_exact_mut(16)) {
        value.to_bytes_le(out);
    }
    Ok(bytes)
}
fn extension_bytes<F: Field + CanonicalEncoding, E: ExtField<F>>(
    values: &[E],
) -> Result<Vec<u8>, AkitaError> {
    let length = values
        .len()
        .checked_mul(32)
        .ok_or(AkitaError::InvalidProof)?;
    let mut bytes = vec![0; length];
    for (value, out) in values.iter().zip(bytes.chunks_exact_mut(32)) {
        value.base_coefficient(0).to_bytes_le(&mut out[..16]);
        value.base_coefficient(1).to_bytes_le(&mut out[16..]);
    }
    Ok(bytes)
}

/// Decode the exact canonical [block][plane][coefficient] partial payload.
pub fn decode_terminal_partials<F: Field + CanonicalEncoding>(
    bytes: &[u8],
    expected: usize,
) -> Result<RingVec<F>, AkitaError> {
    if expected.checked_mul(16).ok_or(AkitaError::InvalidProof)? != bytes.len() {
        return Err(AkitaError::InvalidProof);
    }
    let mut coefficients = Vec::with_capacity(expected);
    for atom in bytes.chunks_exact(16) {
        coefficients.push(F::from_bytes_le_checked(atom).ok_or(AkitaError::InvalidProof)?);
    }
    Ok(RingVec::from_coeffs(coefficients))
}

fn prechallenge<F: Field + CanonicalEncoding, E: ExtField<F>>(
    state: &FullRingTerminalState<F, E>,
    scheduled: &TerminalFoldParams,
    bytes: &[u8],
) -> Result<(), AkitaError> {
    if state.pending_setup
        || state.witness_len != scheduled.input_witness_len
        || state.binding != NextWitnessBindingPolicy::TerminalInnerState
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
        || group
            .e_field_elems
            .checked_mul(16)
            .ok_or(AkitaError::InvalidProof)?
            != bytes.len()
    {
        return Err(AkitaError::InvalidProof);
    }
    let view = crate::full_ring::RawSourceViewPlan::new(
        128,
        1,
        scheduled.blocks.live_blocks,
        scheduled.blocks.positions_per_block,
        1,
    )?;
    if view.physical_len() != state.witness_len {
        return Err(AkitaError::InvalidProof);
    }
    let point = crate::full_ring::FullRingRawPoint::new(&view, state.opening_point.clone())?;
    let mut scalar = E::zero();
    let basis = [
        E::one(),
        E::from_base_fn(|i| if i == 1 { F::one() } else { F::zero() }),
    ];
    for (index, atom) in bytes.chunks_exact(16).enumerate() {
        let block = index / 256;
        let plane = (index / 128) % 2;
        let coefficient = index % 128;
        let value = F::from_bytes_le_checked(atom).ok_or(AkitaError::InvalidProof)?;
        scalar += (*point
            .block_weights()
            .get(block)
            .ok_or(AkitaError::InvalidProof)?
            * *point
                .coefficient_weights()
                .get(coefficient)
                .ok_or(AkitaError::InvalidProof)?
            * basis[plane])
            .mul_base(value);
    }
    if scalar != state.opening {
        return Err(AkitaError::InvalidProof);
    }
    Ok(())
}

/// Bind the full public terminal instance and all clear partials, then draw
/// through the same adapter used by strong checks. The inner adapter publishes
/// complete shell result history; the outer adapter retains literal RankShell.
/// Prover and verifier call this one chronology, with no verifier dependency.
pub fn bind_terminal_and_draw<T, R, D, F, E>(
    transcript: &mut T,
    sites: &TerminalSites<R, T::ShellSite>,
    state: &FullRingTerminalState<F, E>,
    scheduled: &TerminalFoldParams,
    partial_bytes: &[u8],
) -> Result<Challenges, AkitaError>
where
    T: NativeStrongTranscript<R, D>,
    F: Field + CanonicalEncoding,
    E: ExtField<F>,
{
    let result = (|| {
        crate::full_ring::validate_full_ring_fields::<F, E>()?;
        scheduled.validate_opening_execution(E::DEGREE)?;
        if !scheduled.opening_method.is_full_ring_scalar_extension() {
            return Err(AkitaError::InvalidProof);
        }
        prechallenge(state, scheduled, partial_bytes)?;
        let group = scheduled
            .response_shape
            .layout
            .groups
            .first()
            .ok_or(AkitaError::InvalidProof)?;
        // Bind the occurrence selecting the original registered terminal A role.
        let level = u64::try_from(state.predecessor_level).map_err(|_| AkitaError::InvalidProof)?;
        let mut descriptor = level.to_le_bytes().to_vec();
        descriptor.extend_from_slice(&scheduled.canonical_descriptor_bytes());
        transcript.public(&sites.descriptor, 1, &descriptor)?;
        transcript.public(
            &sites.a_images,
            state.witness.coeff_len(),
            &base_bytes(state.witness.coeffs())?,
        )?;
        transcript.public(
            &sites.point,
            state.opening_point.len(),
            &extension_bytes::<F, E>(&state.opening_point)?,
        )?;
        transcript.public(&sites.value, 1, &extension_bytes::<F, E>(&[state.opening])?)?;
        transcript.proof(&sites.partials, group.e_field_elems, partial_bytes)?;
        let shells = transcript.shell_bundle(&sites.shell, scheduled.blocks.live_blocks)?;
        Challenges::from_full_ring_shell_coefficients(shells.iter().map(|s| s.coefficients()))
    })();
    if result.is_err() {
        transcript.reject();
    }
    result
}
