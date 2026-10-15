use super::{indexed::{Core, IndexedError}, NonceAtom, ProtocolContextRecord, ProverChannel};
use spongefish::Encoding;

/// Public v8 snapshot, including exact grammar/index/poison state, but no proof
/// transport or private prover randomness. Preview events never mutate the live owner.
pub struct FoldPreview { pub(super) core: Core }
impl FoldPreview {
    pub fn snapshot(state: &ProverChannel) -> Result<Self, IndexedError> {
        state.core.ensure_valid()?;
        Ok(Self { core: state.core.clone() })
    }
    /// Form the same complete canonical nonce event as live proof transport.
    pub fn new(state: &ProverChannel, record: ProtocolContextRecord, nonce: u32) -> Result<Self, IndexedError> {
        if record.kind != super::ProtocolMessageKind::GrindingNonce as u32
            && record.kind != super::ProtocolMessageKind::FoldResponseNonce as u32 {
            return Err(IndexedError::Grammar);
        }
        let mut preview = Self::snapshot(state)?;
        preview.core.context(record)?;
        preview.core.message_bytes(NonceAtom::new(nonce).encode().as_ref(), false)?;
        Ok(preview)
    }
    /// Preview one canonical proof atom under its installed site.
    pub fn proof_message<T: Encoding<[u8]> + ?Sized>(&mut self, record: ProtocolContextRecord, message: &T) -> Result<(), IndexedError> {
        self.core.context(record)?;
        self.core.message_bytes(message.encode().as_ref(), false)
    }
}
