//! Atomic v8 whole-level drawing for preview, live prover and verifier.
//!
//! All roles execute the same declared family and the same finite decoder.
//! No seed-to-coordinate expansion or successful group-prefix publication exists.

use crate::{CanonicalShellLevel, ShellLevelDescriptor};
use akita_error::AkitaError;
use akita_transcript::{CompletedResultKind, IndexedChannel, IndexedError, IndexedOperation,
    ProtocolSiteId, SITE_FAMILY_FOLD_CHALLENGE};

/// The ordinary fold boundary owns the complete frozen level, not one group.
pub trait FoldDraw {
    /// Decode and append one whole canonical polynomial tuple before returning.
    fn draw_shell_level(&mut self, descriptor: &ShellLevelDescriptor) -> Result<CanonicalShellLevel, AkitaError>;
}

fn draw<C: IndexedChannel>(channel: &mut C, level: u32, descriptor: &ShellLevelDescriptor)
    -> Result<CanonicalShellLevel, AkitaError> {
    let site = ProtocolSiteId { family: SITE_FAMILY_FOLD_CHALLENGE, level, ..ProtocolSiteId::default() };
    let request = channel.request_at(IndexedOperation::ShellLevel, site)?;
    if request.descriptor() != descriptor.encoded_bytes() || request.raw_len() != descriptor.raw_len() {
        channel.invalidate();
        return Err(IndexedError::Request.into());
    }
    Ok(channel.execute_indexed(&request, CompletedResultKind::ShellLevel, descriptor.result_len(),
        |answer, completed| crate::shell_decode::decode_and_complete(descriptor, answer, completed)
            .map_err(|_| IndexedError::Decode))?)
}

/// One complete level against a candidate public state.
pub struct PreviewFoldDraw<'a> {
    preview: &'a mut akita_transcript::FoldPreview,
    level: u32,
}
impl<'a> PreviewFoldDraw<'a> {
    /// Bind an atomic preview to the scheduled level site.
    pub const fn new(preview: &'a mut akita_transcript::FoldPreview, level: u32) -> Self {
        Self { preview, level }
    }
}
impl FoldDraw for PreviewFoldDraw<'_> {
    fn draw_shell_level(&mut self, descriptor: &ShellLevelDescriptor) -> Result<CanonicalShellLevel, AkitaError> {
        draw(self.preview, self.level, descriptor)
    }
}

/// One complete level against the live prover channel.
pub struct ProverFoldDraw<'a> {
    state: &'a mut akita_transcript::ProverChannel,
    level: u32,
}
impl<'a> ProverFoldDraw<'a> {
    /// Bind an atomic live draw to the scheduled level site.
    pub const fn new(state: &'a mut akita_transcript::ProverChannel, level: u32) -> Self {
        Self { state, level }
    }
}
impl FoldDraw for ProverFoldDraw<'_> {
    fn draw_shell_level(&mut self, descriptor: &ShellLevelDescriptor) -> Result<CanonicalShellLevel, AkitaError> {
        draw(self.state, self.level, descriptor)
    }
}

/// One complete level against the live verifier channel.
pub struct VerifierFoldDraw<'a, 'proof> {
    state: &'a mut akita_transcript::VerifierChannel<'proof>,
    level: u32,
}
impl<'a, 'proof> VerifierFoldDraw<'a, 'proof> {
    /// Bind an atomic verified draw to the scheduled level site.
    pub const fn new(state: &'a mut akita_transcript::VerifierChannel<'proof>, level: u32) -> Self {
        Self { state, level }
    }
}
impl FoldDraw for VerifierFoldDraw<'_, '_> {
    fn draw_shell_level(&mut self, descriptor: &ShellLevelDescriptor) -> Result<CanonicalShellLevel, AkitaError> {
        draw(self.state, self.level, descriptor)
    }
}

#[cfg(test)]
mod tests;
