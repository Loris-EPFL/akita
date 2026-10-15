//! Role-generic operations. Logical event context is mandatory and installed;
//! no generic public-randomness squeeze or unlabelled scalar operation exists.
use super::{indexed::{CompletedResult, CompletedResultKind, FiniteAnswer, IndexedError,
    IndexedOperation, IndexedRequest}, FoldPreview, ProtocolContextRecord, ProtocolSiteId,
    ProverChannel, VerifierChannel};
use akita_error::AkitaError;
use jolt_field::{CanonicalEncoding, ExtField, Field};
use spongefish::{Encoding, NargDeserialize};

/// The bounded atomic indexed-request interface, independent of shell types.
pub trait IndexedChannel {
    /// Check the next installed operation/site and obtain its full request.
    fn request_at(&mut self, operation: IndexedOperation, site: ProtocolSiteId) -> Result<IndexedRequest, IndexedError>;
    /// Execute one indivisible shell-level finite request and append its full
    /// canonical result before returning. Scalar/predicate decoding is compiler
    /// owned and cannot be replaced with this external decoder capability.
    /// Ignored reader/sink errors and unfinished completion poison the owner.
    fn execute_indexed<T>(&mut self, request: &IndexedRequest, kind: CompletedResultKind,
        result_len: u64, decode: impl FnOnce(&mut FiniteAnswer, &mut CompletedResult) -> Result<T, IndexedError>)
        -> Result<T, IndexedError>;
    /// Return the first sticky error, if any.
    fn status(&self) -> Result<(), IndexedError>;
    /// Permanently poison this computation on a consumer's semantic error.
    fn invalidate(&mut self);
    /// Draw one labelled finite canonical prime coordinate (128 fixed16-byte slots).
    fn field_challenge_at<F: Field + CanonicalEncoding>(&mut self, site: ProtocolSiteId) -> Result<F, AkitaError>;
    /// Draw one whole canonical base-coordinate vector with independent fixed
    /// allocations and a single completed vector event.
    fn field_vector_challenge_at<F, E>(&mut self, site: ProtocolSiteId) -> Result<E, AkitaError>
    where F: Field + CanonicalEncoding, E: ExtField<F>;
    /// Draw and complete the separate full32-byte predicate operation.
    fn predicate_at(&mut self, site: ProtocolSiteId) -> Result<[u8; 32], IndexedError>;
}

macro_rules! indexed_owner {
    ($ty:ty) => {
        impl IndexedChannel for $ty {
            fn request_at(&mut self, operation: IndexedOperation, site: ProtocolSiteId) -> Result<IndexedRequest, IndexedError> {
                self.core.next_request(operation, site)
            }
            fn execute_indexed<T>(&mut self, request: &IndexedRequest, kind: CompletedResultKind,
                result_len: u64, decode: impl FnOnce(&mut FiniteAnswer, &mut CompletedResult) -> Result<T, IndexedError>) -> Result<T, IndexedError> {
                if request.operation() != IndexedOperation::ShellLevel || kind != CompletedResultKind::ShellLevel {
                    return self.core.fail(IndexedError::Request);
                }
                self.core.execute(request, kind, result_len, decode)
            }
            fn field_challenge_at<F: Field + CanonicalEncoding>(&mut self, site: ProtocolSiteId) -> Result<F, AkitaError> {
                super::sampling::sample_field(&mut self.core, site).map_err(Into::into)
            }
            fn field_vector_challenge_at<F, E>(&mut self, site: ProtocolSiteId) -> Result<E, AkitaError>
            where F: Field + CanonicalEncoding, E: ExtField<F> {
                super::sampling::sample_vector::<F, E>(&mut self.core, site).map_err(Into::into)
            }
            fn predicate_at(&mut self, site: ProtocolSiteId) -> Result<[u8; 32], IndexedError> {
                let request = self.core.next_request(IndexedOperation::Predicate32, site)?;
                self.core.execute(&request, CompletedResultKind::Predicate32, 32, |answer, completed| {
                    let mut output = [0u8; 32];
                    answer.read_exact(&mut output)?;
                    completed.write_all(&output)?;
                    Ok(output)
                })
            }
            fn status(&self) -> Result<(), IndexedError> { self.core.status() }
            fn invalidate(&mut self) { self.core.invalidate(); }
        }
    };
}
indexed_owner!(ProverChannel);
indexed_owner!(VerifierChannel<'_>);
indexed_owner!(FoldPreview);

/// Canonical public/proof transactions for role-generic protocol code.
pub trait ProofChannel: IndexedChannel {
    /// Begin the exact next installed logical event. Chunking is internal.
    fn context(&mut self, record: ProtocolContextRecord) -> Result<(), AkitaError>;
    /// Append a public atom within the current installed public event.
    fn public<T: Encoding<[u8]> + ?Sized>(&mut self, message: &T) -> Result<(), AkitaError>;
    /// Emit/read a canonical proof atom within the current installed proof event.
    fn exchange<T: Encoding<[u8]> + NargDeserialize>(&mut self, message: &mut T) -> Result<(), AkitaError>;
}
impl ProofChannel for ProverChannel {
    fn context(&mut self, record: ProtocolContextRecord) -> Result<(), AkitaError> { super::prover_context(self, record) }
    fn public<T: Encoding<[u8]> + ?Sized>(&mut self, message: &T) -> Result<(), AkitaError> {
        self.core.ensure_valid()?;
        self.core.message_bytes(message.encode().as_ref(), true).map_err(Into::into)
    }
    fn exchange<T: Encoding<[u8]> + NargDeserialize>(&mut self, message: &mut T) -> Result<(), AkitaError> { self.prover_message(message) }
}
impl ProofChannel for VerifierChannel<'_> {
    fn context(&mut self, record: ProtocolContextRecord) -> Result<(), AkitaError> { super::verifier_context(self, record) }
    fn public<T: Encoding<[u8]> + ?Sized>(&mut self, message: &T) -> Result<(), AkitaError> {
        self.core.ensure_valid()?;
        self.core.message_bytes(message.encode().as_ref(), true).map_err(Into::into)
    }
    fn exchange<T: Encoding<[u8]> + NargDeserialize>(&mut self, message: &mut T) -> Result<(), AkitaError> {
        *message = self.prover_message()?;
        Ok(())
    }
}
impl ProverChannel {
    /// Checked public transport helper; cannot create an unowned event.
    pub fn public_message<T: Encoding<[u8]> + ?Sized>(&mut self, message: &T) -> Result<(), AkitaError> { self.public(message) }
    pub fn invalidate(&mut self) { self.core.invalidate(); }
    pub fn status(&self) -> Result<(), IndexedError> { self.core.status() }
}
impl VerifierChannel<'_> {
    pub fn public_message<T: Encoding<[u8]> + ?Sized>(&mut self, message: &T) -> Result<(), AkitaError> { self.public(message) }
    pub fn invalidate(&mut self) { self.core.invalidate(); }
    pub fn status(&self) -> Result<(), IndexedError> { self.core.status() }
}
impl FoldPreview {
    /// A whole preview public event, not an unowned absorb.
    pub fn public_at<T: Encoding<[u8]> + ?Sized>(&mut self, record: ProtocolContextRecord, message: &T) -> Result<(), IndexedError> {
        self.core.context(record)?;
        self.core.message_bytes(message.encode().as_ref(), true)
    }
}
