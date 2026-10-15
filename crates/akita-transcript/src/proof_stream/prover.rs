use super::indexed::{Core, IndexedError, InstalledContext};
use akita_error::AkitaError;
use spongefish::Encoding;

/// Ordinary v8 prover owner: literal-prefix compiler plus canonical proof transport.
pub struct ProverChannel {
    pub(super) core: Core,
    proof: Vec<u8>,
}
impl ProverChannel {
    pub(super) fn new(context: &InstalledContext) -> Self { Self { core: Core::from_context(context), proof: Vec::new() } }
    /// Transport bytes so far. This is not an acceptance/completion certificate.
    pub fn narg_string(&self) -> &[u8] { &self.proof }
    /// Append one canonical atom inside the installed current event transaction.
    pub fn prover_message<T: Encoding<[u8]> + ?Sized>(&mut self, message: &T) -> Result<(), AkitaError> {
        self.core.ensure_valid()?;
        let encoded = message.encode();
        self.send_raw(encoded.as_ref()).map_err(Into::into)
    }
    pub(super) fn send_raw(&mut self, bytes: &[u8]) -> Result<(), IndexedError> {
        self.core.ensure_valid()?;
        let Some(total) = self.proof.len().checked_add(bytes.len()) else { return self.core.fail(IndexedError::Budget); };
        if total as u64 > self.core.proof_limit() { return self.core.fail(IndexedError::Budget); }
        self.core.message_bytes(bytes, false)?;
        if self.proof.try_reserve(bytes.len()).is_err() { return self.core.fail(IndexedError::Budget); }
        self.proof.extend_from_slice(bytes);
        Ok(())
    }
    /// Require complete installed grammar and no pending/failed transaction.
    pub fn finish(self) -> Result<Vec<u8>, IndexedError> {
        self.core.finish()?;
        Ok(self.proof)
    }
}
