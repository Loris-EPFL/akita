//! Live terminal replay shares the prover's exact generic native chronology.
use super::full_ring::verify_full_ring_terminal_suffix;
use akita_error::AkitaError;
use akita_params::TerminalFoldParams;
use akita_transcript::native_strong::{NativeStrongTranscript, TerminalSites};
use akita_types::full_ring_terminal::{
    bind_terminal_and_draw, decode_terminal_partials, FullRingTerminalState,
};
use akita_types::InstalledFullRingSetup;
use jolt_field::{CanonicalEncoding, ExtField, Field, Ring};

/// Consume canonical terminal payloads on the same live adapter as the strong
/// checks. Both inner and outer use their actual compiler; inner shell-result
/// history is appended by that adapter before the signed response is absorbed.
/// The owning continuation rejects trailing bytes and calls transcript finish.
pub fn verify_live_full_ring_terminal<T, R, D, F, E>(
    transcript: &mut T,
    sites: &TerminalSites<R, T::ShellSite>,
    setup: &InstalledFullRingSetup<F>,
    state: &FullRingTerminalState<F, E>,
    scheduled: &TerminalFoldParams,
    partial_bytes: &[u8],
    response_bytes: &[u8],
) -> Result<(), AkitaError>
where
    T: NativeStrongTranscript<R, D>,
    F: Field + CanonicalEncoding + akita_serialization::AkitaSerialize + Ring,
    E: ExtField<F>,
{
    let result = (|| {
        let challenges = bind_terminal_and_draw::<T, R, D, F, E>(
            transcript,
            sites,
            state,
            scheduled,
            partial_bytes,
        )?;
        let group = scheduled
            .response_shape
            .layout
            .groups
            .first()
            .ok_or(AkitaError::InvalidProof)?;
        let partials = decode_terminal_partials::<F>(partial_bytes, group.e_field_elems)?;
        transcript.proof(&sites.response, group.z_coords, response_bytes)?;
        verify_full_ring_terminal_suffix(
            setup,
            state,
            scheduled,
            response_bytes.to_vec(),
            partials,
            &challenges,
        )
    })();
    if result.is_err() {
        transcript.reject();
    }
    result
}
