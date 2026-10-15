use super::indexed::{Core, IndexedError, InstalledContext};
use akita_error::AkitaError;
use spongefish::{Encoding, NargDeserialize};

/// Fail-closed ordinary v8 verifier owner. The proof cursor and hash prefix are private.
pub struct VerifierChannel<'proof> {
    pub(super) core: Core,
    proof: &'proof [u8],
}
impl<'proof> VerifierChannel<'proof> {
    pub(super) fn new(context: &InstalledContext, proof: &'proof [u8]) -> Result<Self, IndexedError> {
        let core = Core::from_context(context);
        if proof.len() as u64 > core.proof_limit() { return Err(IndexedError::Budget); }
        Ok(Self { core, proof })
    }
    /// Read one canonical atom within the installed current logical event.
    pub fn prover_message<T: Encoding<[u8]> + NargDeserialize>(&mut self) -> Result<T, AkitaError> {
        self.core.ensure_valid()?;
        let bound = self.core.proof_read_limit()?;
        let original = &self.proof[..self.proof.len().min(bound)];
        let mut remaining = original;
        let value = match T::deserialize_from_narg(&mut remaining) {
            Ok(value) => value,
            Err(_) => return self.core.fail(IndexedError::Decode).map_err(Into::into),
        };
        let Some(consumed) = original.len().checked_sub(remaining.len()) else {
            return self.core.fail(IndexedError::Decode).map_err(Into::into);
        };
        let Some(bytes) = original.get(..consumed) else { return self.core.fail(IndexedError::Decode).map_err(Into::into); };
        // Transport codecs must accept exactly their unique canonical encoding.
        if value.encode().as_ref() != bytes { return self.core.fail(IndexedError::Decode).map_err(Into::into); }
        self.core.message_bytes(bytes, false)?;
        self.proof = &self.proof[consumed..];
        Ok(value)
    }
    pub(super) fn receive_raw(&mut self, len: usize) -> Result<Vec<u8>, IndexedError> {
        self.core.ensure_valid()?;
        let Some((bytes, remaining)) = self.proof.split_at_checked(len) else { return self.core.fail(IndexedError::Decode); };
        self.core.message_bytes(bytes, false)?;
        let mut output = Vec::new();
        if output.try_reserve_exact(len).is_err() { return self.core.fail(IndexedError::Budget); }
        output.extend_from_slice(bytes);
        self.proof = remaining;
        Ok(output)
    }
    /// Accept only after the installed event/request grammar and exact proof EOF.
    pub fn check_eof(self) -> Result<(), AkitaError> { self.finish().map_err(Into::into) }
    pub fn finish(self) -> Result<(), IndexedError> {
        self.core.finish()?;
        if !self.proof.is_empty() { return Err(IndexedError::TrailingProof); }
        Ok(())
    }
}
