//! Version-eight literal-key compiler. This is the ordinary SHAKE256 provider,
//! not the separately named native D1/H1610 compiler or a SHAKE seed expander.
use super::{ProtocolContextRecord, ProtocolMessageKind, ProtocolSiteId, PROTOCOL_VERSION};
use akita_error::AkitaError;
use sha3::{digest::{ExtendableOutput, Update, XofReader}, Shake256, Shake256Reader};
use std::{fmt, sync::Arc};

/// Conventional concrete provider and protocol epoch, independent of old backend features.
pub const INDEXED_PROVIDER: &str = "akita-pcs/indexed/v8/shake256";
const DOMAIN: &[u8] = b"akita-pcs/indexed/v8/shake256";
const PUBLIC: u8 = 1;
const PROOF: u8 = 2;
const COMPLETED: u8 = 3;
const REQUEST: u8 = 4;
const MESSAGE_FIXED: u64 = 76;
const REQUEST_FIXED: u64 = 60;

/// A fail-closed indexed-channel error. The first failure remains sticky.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexedError {
    /// Invalid installation, profile, version, or canonical descriptor.
    Installation,
    /// Event/site/operation order disagrees with the installed grammar.
    Grammar,
    /// Request ordinal or literal descriptor disagrees with the installed request.
    Request,
    /// Overflow or violation of an installed whole-byte resource budget.
    Budget,
    /// A finite answer was read or skipped beyond its declared length.
    AnswerBounds,
    /// All finite canonical candidates were rejected.
    ScalarExhausted,
    /// A decoder rejected its answer or a proof atom was not canonical.
    Decode,
    /// Completed-result byte count does not match the installed codec length.
    Completion,
    /// A transaction or installed plan remains unfinished.
    Incomplete,
    /// Extra proof transport bytes remain at acceptance.
    TrailingProof,
    /// A protocol consumer explicitly invalidated this computation.
    Invalidated,
}
impl fmt::Display for IndexedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "indexed transcript: {self:?}") }
}
impl std::error::Error for IndexedError {}
impl From<IndexedError> for AkitaError {
    fn from(_: IndexedError) -> Self { Self::InvalidProof }
}

/// Exactly the admitted ordinary finite oracle operations. No generic squeeze exists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum IndexedOperation { Scalar128 = 1, Predicate32 = 2, ShellLevel = 3, PrimeScalar = 4, FieldVector = 5 }
/// Canonical completed result kind. Shell bytes are the complete canonical polynomial tuple.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CompletedResultKind { Scalar = 1, Predicate32 = 2, ShellLevel = 3, PrimeScalar = 4, FieldVector = 5 }

/// Complete literal descriptor for one scheduled finite request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexedRequest {
    operation: IndexedOperation,
    ordinal: u64,
    site: ProtocolSiteId,
    descriptor: Arc<[u8]>,
    raw_len: u64,
}
impl IndexedRequest {
    /// Construct a structurally checked descriptor; admission is checked against the registry.
    pub fn new(operation: IndexedOperation, ordinal: u64, site: ProtocolSiteId,
        descriptor: impl Into<Arc<[u8]>>, raw_len: u64) -> Result<Self, IndexedError> {
        let descriptor = descriptor.into();
        if descriptor.is_empty() || raw_len == 0 { return Err(IndexedError::Request); }
        let request = Self { operation, ordinal, site, descriptor, raw_len };
        request.encoded_len()?;
        Ok(request)
    }
    pub const fn operation(&self) -> IndexedOperation { self.operation }
    pub const fn ordinal(&self) -> u64 { self.ordinal }
    pub const fn site(&self) -> ProtocolSiteId { self.site }
    pub fn descriptor(&self) -> &[u8] { &self.descriptor }
    pub const fn raw_len(&self) -> u64 { self.raw_len }
    fn encoded_len(&self) -> Result<u64, IndexedError> {
        REQUEST_FIXED.checked_add(u64::try_from(self.descriptor.len()).map_err(|_| IndexedError::Budget)?)
            .ok_or(IndexedError::Budget)
    }
    fn absorb(&self, hash: &mut Shake256) {
        hash.update(&(self.operation as u32).to_le_bytes());
        hash.update(&self.ordinal.to_le_bytes());
        hash.update(&self.site.to_bytes());
        hash.update(&self.raw_len.to_le_bytes());
        hash.update(&(self.descriptor.len() as u64).to_le_bytes());
        hash.update(&self.descriptor);
    }
}

/// Installed request and the exact canonical completion codec's length.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestSpec {
    pub request: IndexedRequest,
    pub result_kind: CompletedResultKind,
    pub result_len: u64,
}
/// An exact ordered protocol grammar, not a diagnostic or proof-selected schedule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstalledEvent {
    Message(ProtocolContextRecord),
    /// A proof byte group whose exact length is bound by its preceding length
    /// atom. The site and upper bound remain installed, not proof-selected.
    BoundedBytes { site: ProtocolSiteId, max_len: u64 },
    Request(RequestSpec),
}
/// Public finite resource policy. Bounds apply to whole bytes, including framing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndexedLimits {
    pub max_context_bytes: u64,
    pub max_prefix_bytes: u64,
    pub max_key_bytes: u64,
    pub max_descriptor_bytes: u64,
    pub max_answer_bytes: u64,
    pub max_result_bytes: u64,
    pub max_proof_bytes: u64,
}

/// Exact verifier-installed data under a unique registry index.
///
/// The application must install this from approved configuration, never from a
/// prover digest/index. Construction validates the compiler grammar, not the
/// AQRA arithmetic, catalog admission, filter certificate or extraction ledger.
#[derive(Clone)]
pub struct InstalledRegistry {
    index: u64,
    events: Arc<[InstalledEvent]>,
    limits: IndexedLimits,
}
impl InstalledRegistry {
    pub fn new(index: u64, events: Vec<InstalledEvent>, limits: IndexedLimits) -> Result<Self, IndexedError> {
        let mut ordinal = 0u64;
        for event in &events {
            match event {
                InstalledEvent::Message(record) => {
                    check_record(*record)?;
                    if (record.kind != ProtocolMessageKind::PublicValue as u32 && record.encoded_bytes > limits.max_proof_bytes)
                        || record.challenge_bytes != 0 {
                        return Err(IndexedError::Installation);
                    }
                }
                InstalledEvent::BoundedBytes { max_len, .. } => {
                    if *max_len > limits.max_proof_bytes { return Err(IndexedError::Installation); }
                }
                InstalledEvent::Request(spec) => {
                    let request = &spec.request;
                    if request.ordinal != ordinal || request.descriptor.len() as u64 > limits.max_descriptor_bytes
                        || request.raw_len > limits.max_answer_bytes || spec.result_len > limits.max_result_bytes
                        || spec.result_len == 0 { return Err(IndexedError::Installation); }
                    match (request.operation, spec.result_kind) {
                        (IndexedOperation::Scalar128, CompletedResultKind::Scalar) if request.raw_len == 2048
                            && spec.result_len == 16 && request.descriptor() == scalar_descriptor() => {},
                        (IndexedOperation::Predicate32, CompletedResultKind::Predicate32) if request.raw_len == 32
                            && spec.result_len == 32 && valid_predicate_descriptor(request.descriptor()) => {},
                        (IndexedOperation::ShellLevel, CompletedResultKind::ShellLevel) => {},
                        (IndexedOperation::PrimeScalar, CompletedResultKind::PrimeScalar) => {
                            let width = super::sampling::validate_prime_descriptor(request.descriptor())
                                .ok_or(IndexedError::Installation)?;
                            if request.raw_len != 2048 || spec.result_len != width as u64 {
                                return Err(IndexedError::Installation);
                            }
                        },
                        (IndexedOperation::FieldVector, CompletedResultKind::FieldVector) => {
                            let (degree, width) = super::sampling::validate_vector_descriptor(request.descriptor(), request.site)
                                .ok_or(IndexedError::Installation)?;
                            if degree.checked_mul(2048) != Some(request.raw_len)
                                || degree.checked_mul(width as u64) != Some(spec.result_len) {
                                return Err(IndexedError::Installation);
                            }
                        },
                        _ => return Err(IndexedError::Installation),
                    }
                    ordinal = ordinal.checked_add(1).ok_or(IndexedError::Budget)?;
                }
            }
        }
        Ok(Self { index, events: events.into(), limits })
    }
    pub const fn index(&self) -> u64 { self.index }
    pub fn events(&self) -> &[InstalledEvent] { &self.events }
}

/// Literal inherited history and endpoint identity, supplied as an external
/// native PCS statement. This does not attest frontend/query proof validity.
#[derive(Clone, Copy, Debug)]
pub struct InheritedAugmentation<'a> {
    pub literal_history: &'a [u8],
    pub endpoint_binding: &'a [u8],
}
impl InheritedAugmentation<'_> {
    const DOMAIN: &'static [u8] = b"akita-pcs/inherited-augmentation/v8";

    /// Exact checked augmented-instance length without copying the history.
    pub fn encoded_len(&self, native_statement: &[u8]) -> Result<usize, AkitaError> {
        [Self::DOMAIN, native_statement, self.literal_history, self.endpoint_binding]
            .iter()
            .try_fold(0usize, |size, field| {
                u64::try_from(field.len()).map_err(|_| AkitaError::InvalidProof)?;
                size.checked_add(8)
                    .and_then(|n| n.checked_add(field.len()))
                    .ok_or(AkitaError::InvalidProof)
            })
    }

    /// Compare the complete canonical endpoint binding before constructing the
    /// injectively framed native statement/history/endpoint augmented instance.
    pub fn bind(&self, native_statement: &[u8], expected_endpoint_binding: &[u8])
        -> Result<Vec<u8>, AkitaError> {
        if self.endpoint_binding != expected_endpoint_binding { return Err(AkitaError::InvalidProof); }
        let domain = Self::DOMAIN;
        let fields: [&[u8]; 4] = [domain, native_statement, self.literal_history, self.endpoint_binding];
        let size = self.encoded_len(native_statement)?;
        let mut encoded = Vec::new();
        encoded.try_reserve_exact(size).map_err(|_| AkitaError::InvalidProof)?;
        for field in fields {
            encoded.extend_from_slice(&(field.len() as u64).to_le_bytes());
            encoded.extend_from_slice(field);
        }
        Ok(encoded)
    }
}

/// Validated installed context with the complete sealed frontend/PCS handoff.
/// The caller supplies the complete augmented instance and endpoint roster;
/// this compiler cannot infer whether a shorter context omitted public data.
#[derive(Clone)]
pub struct InstalledContext { core: Core }
impl InstalledContext {
    pub fn new(registry: &InstalledRegistry, session: &[u8], augmented_instance: &[u8]) -> Result<Self, IndexedError> {
        let mut installed = Self::start_header(registry, session, augmented_instance.len())?;
        installed.core.prefix.update(augmented_instance);
        Ok(installed)
    }

    /// Bind the full native statement, inherited history and expected endpoints.
    /// All installed byte budgets are checked before absorbing the borrowed
    /// fields; no augmented-instance allocation or history copy is needed.
    ///
    /// # Errors
    /// Rejects a mismatched endpoint binding, length overflow or installed budget.
    pub fn new_augmented(
        registry: &InstalledRegistry,
        session: &[u8],
        native_statement: &[u8],
        augmentation: InheritedAugmentation<'_>,
        expected_endpoint_binding: &[u8],
    ) -> Result<Self, AkitaError> {
        if augmentation.endpoint_binding != expected_endpoint_binding {
            return Err(AkitaError::InvalidProof);
        }
        let length = augmentation.encoded_len(native_statement)?;
        let mut installed = Self::start_header(registry, session, length)?;
        for field in [InheritedAugmentation::DOMAIN, native_statement,
            augmentation.literal_history, augmentation.endpoint_binding] {
            absorb_frame(&mut installed.core.prefix, field);
        }
        Ok(installed)
    }

    fn start_header(registry: &InstalledRegistry, session: &[u8], augmented_len: usize) -> Result<Self, IndexedError> {
        let session_len = u64::try_from(session.len()).map_err(|_| IndexedError::Budget)?;
        let augmented_len = u64::try_from(augmented_len).map_err(|_| IndexedError::Budget)?;
        let context_len = session_len.checked_add(augmented_len).ok_or(IndexedError::Budget)?;
        if context_len > registry.limits.max_context_bytes { return Err(IndexedError::Budget); }
        let prefix_bytes = (DOMAIN.len() as u64).checked_add(36).and_then(|n| n.checked_add(context_len))
            .ok_or(IndexedError::Budget)?;
        if prefix_bytes > registry.limits.max_prefix_bytes { return Err(IndexedError::Budget); }
        let mut prefix = Shake256::default();
        absorb_frame(&mut prefix, DOMAIN);
        prefix.update(&PROTOCOL_VERSION.to_le_bytes());
        prefix.update(&registry.index.to_le_bytes());
        absorb_frame(&mut prefix, session);
        prefix.update(&augmented_len.to_le_bytes());
        Ok(Self { core: Core { registry: registry.clone(), prefix, prefix_bytes,
            cursor: 0, event_index: 0, request_ordinal: 0, active: None, empty_chunk: None, error: None } })
    }
}
fn absorb_frame(hash: &mut Shake256, bytes: &[u8]) {
    hash.update(&(bytes.len() as u64).to_le_bytes());
    hash.update(bytes);
}
fn check_record(record: ProtocolContextRecord) -> Result<(), IndexedError> {
    if record.version != PROTOCOL_VERSION || record.domain != super::CONTEXT_DOMAIN {
        return Err(IndexedError::Installation);
    }
    match record.kind {
        k if k == ProtocolMessageKind::PublicValue as u32 || k == ProtocolMessageKind::ProofLength as u32
            || k == ProtocolMessageKind::ProofAtoms as u32 => {},
        k if k == ProtocolMessageKind::GrindingNonce as u32 || k == ProtocolMessageKind::FoldResponseNonce as u32
            => {
                let bits = nonce_bits(record)?;
                if record.atom_count != 1 || record.encoded_bytes != super::nonce_max_bytes(bits) as u64 {
                    return Err(IndexedError::Installation);
                }
            },
        _ => return Err(IndexedError::Installation),
    }
    Ok(())
}
fn is_nonce(record: ProtocolContextRecord) -> bool {
    record.kind == ProtocolMessageKind::GrindingNonce as u32 || record.kind == ProtocolMessageKind::FoldResponseNonce as u32
}
fn nonce_bits(record: ProtocolContextRecord) -> Result<u8, IndexedError> {
    let detail = ProtocolSiteId::from_bytes(record.site_id).detail;
    if record.kind == ProtocolMessageKind::GrindingNonce as u32 {
        if detail > u16::MAX as u32 { return Err(IndexedError::Installation); }
        let grind = (detail & 0xff) as u8;
        let nonce = (detail >> 8) as u8;
        predicate_descriptor(grind, nonce)?;
        Ok(nonce)
    } else {
        // The retained FoldResponse range is exactly twelve bits.
        if detail != 12 { return Err(IndexedError::Installation); }
        Ok(12)
    }
}

#[derive(Clone)]
struct ActiveMessage { record: ProtocolContextRecord, remaining: u64, started: bool }
#[derive(Clone)]
pub(super) struct Core {
    registry: InstalledRegistry,
    prefix: Shake256,
    prefix_bytes: u64,
    cursor: usize,
    event_index: u64,
    request_ordinal: u64,
    active: Option<ActiveMessage>,
    empty_chunk: Option<bool>,
    error: Option<IndexedError>,
}
impl Core {
    pub(super) fn from_context(context: &InstalledContext) -> Self { context.core.clone() }
    pub(super) fn ensure_valid(&self) -> Result<(), IndexedError> {
        self.error.map_or(Ok(()), Err)
    }
    pub(super) fn fail<T>(&mut self, error: IndexedError) -> Result<T, IndexedError> {
        let first = *self.error.get_or_insert(error);
        Err(first)
    }
    pub(super) fn status(&self) -> Result<(), IndexedError> { self.ensure_valid() }
    pub(super) fn invalidate(&mut self) { let _ = self.fail::<()>(IndexedError::Invalidated); }
    pub(super) fn proof_limit(&self) -> u64 { self.registry.limits.max_proof_bytes }
    pub(super) fn proof_read_limit(&mut self) -> Result<usize, IndexedError> {
        self.ensure_valid()?;
        let Some(active) = &self.active else { return self.fail(IndexedError::Grammar); };
        if active.record.kind == ProtocolMessageKind::PublicValue as u32 { return self.fail(IndexedError::Grammar); }
        match usize::try_from(active.remaining) {
            Ok(bound) => Ok(bound),
            Err(_) => self.fail(IndexedError::Budget),
        }
    }
    pub(super) fn context(&mut self, record: ProtocolContextRecord) -> Result<(), IndexedError> {
        self.ensure_valid()?;
        self.empty_chunk = None;
        let admitted = match self.registry.events.get(self.cursor) {
            Some(InstalledEvent::Message(expected)) => record == *expected,
            Some(InstalledEvent::BoundedBytes { site, max_len }) => {
                record.version == PROTOCOL_VERSION && record.domain == super::CONTEXT_DOMAIN
                    && record.site_id == site.to_bytes() && record.kind == ProtocolMessageKind::ProofAtoms as u32
                    && record.atom_count == record.encoded_bytes && record.encoded_bytes <= *max_len
                    && record.challenge_bytes == 0
            },
            _ => false,
        };
        if self.active.is_some() || !admitted { return self.fail(IndexedError::Grammar); }
        self.active = Some(ActiveMessage { record, remaining: record.encoded_bytes, started: false });
        if record.encoded_bytes == 0 {
            let public = record.kind == ProtocolMessageKind::PublicValue as u32;
            self.message_bytes(&[], public)?;
            self.empty_chunk = Some(public);
        }
        Ok(())
    }
    pub(super) fn message_bytes(&mut self, bytes: &[u8], public: bool) -> Result<(), IndexedError> {
        self.ensure_valid()?;
        let Some(active) = self.active.as_ref() else {
            // One optional transport chunk belongs to the immediately completed
            // zero-length event. An unowned empty message is still illegal.
            if bytes.is_empty() && self.empty_chunk == Some(public) {
                self.empty_chunk = None;
                return Ok(());
            }
            return self.fail(IndexedError::Grammar);
        };
        let record = active.record;
        if public != (record.kind == ProtocolMessageKind::PublicValue as u32) { return self.fail(IndexedError::Grammar); }
        let n = bytes.len() as u64;
        let nonce = is_nonce(record);
        if n > active.remaining || (nonce && (n == 0 || active.started)) { return self.fail(IndexedError::Budget); }
        // A nonce is one complete canonical atom. All other chunks are internal
        // to the declared logical event, never independently framed messages.
        if nonce {
            let mut cursor = bytes;
            use spongefish::NargDeserialize;
            let value = match super::NonceAtom::deserialize_from_narg(&mut cursor) {
                Ok(value) if cursor.is_empty() => value.into_inner(),
                _ => return self.fail(IndexedError::Decode),
            };
            let bits = match nonce_bits(record) { Ok(bits) => bits, Err(error) => return self.fail(error) };
            if u64::from(value) >= (1u64 << bits) { return self.fail(IndexedError::Decode); }
        }
        if !active.started {
            let payload_len = if nonce { n } else { record.encoded_bytes };
            let event_len = MESSAGE_FIXED.checked_add(payload_len).and_then(|n| n.checked_add(9))
                .ok_or(IndexedError::Budget);
            let total = event_len.and_then(|n| self.prefix_bytes.checked_add(n).ok_or(IndexedError::Budget));
            let Ok(total) = total else { return self.fail(IndexedError::Budget); };
            if total > self.registry.limits.max_prefix_bytes { return self.fail(IndexedError::Budget); }
            self.prefix_bytes = total;
            self.prefix.update(&[if public { PUBLIC } else { PROOF }]);
            self.prefix.update(&(MESSAGE_FIXED + payload_len).to_le_bytes());
            self.prefix.update(&self.event_index.to_le_bytes());
            self.prefix.update(&record.site_id);
            self.prefix.update(&record.kind.to_le_bytes());
            self.prefix.update(&record.atom_count.to_le_bytes());
            self.prefix.update(&record.encoded_bytes.to_le_bytes());
            self.prefix.update(&record.challenge_bytes.to_le_bytes());
            self.prefix.update(&payload_len.to_le_bytes());
        }
        self.prefix.update(bytes);
        if let Some(active) = self.active.as_mut() {
            active.started = true;
            active.remaining = if nonce { 0 } else { active.remaining - n };
        }
        if self.active.as_ref().is_some_and(|a| a.remaining == 0) {
            self.active = None;
            self.advance_event()?;
        }
        Ok(())
    }
    fn advance_event(&mut self) -> Result<(), IndexedError> {
        let Some(cursor) = self.cursor.checked_add(1) else { return self.fail(IndexedError::Budget); };
        let Some(index) = self.event_index.checked_add(1) else { return self.fail(IndexedError::Budget); };
        self.cursor = cursor;
        self.event_index = index;
        Ok(())
    }
    pub(super) fn next_request(&mut self, operation: IndexedOperation, site: ProtocolSiteId) -> Result<IndexedRequest, IndexedError> {
        self.ensure_valid()?;
        self.empty_chunk = None;
        match self.registry.events.get(self.cursor) {
            Some(InstalledEvent::Request(spec)) if spec.request.operation == operation && spec.request.site == site => Ok(spec.request.clone()),
            _ => self.fail(IndexedError::Request),
        }
    }
    pub(super) fn execute<T>(&mut self, request: &IndexedRequest, kind: CompletedResultKind, result_len: u64,
        decode: impl FnOnce(&mut FiniteAnswer, &mut CompletedResult) -> Result<T, IndexedError>) -> Result<T, IndexedError> {
        self.ensure_valid()?;
        self.empty_chunk = None;
        if self.active.is_some() || request.ordinal != self.request_ordinal {
            return self.fail(IndexedError::Request);
        }
        let admitted = matches!(self.registry.events.get(self.cursor), Some(InstalledEvent::Request(spec))
            if &spec.request == request && spec.result_kind == kind && spec.result_len == result_len);
        if !admitted { return self.fail(IndexedError::Request); }
        let request_len = match request.encoded_len() { Ok(n) => n, Err(e) => return self.fail(e) };
        let Some(key_len) = self.prefix_bytes.checked_add(9).and_then(|n| n.checked_add(request_len)) else { return self.fail(IndexedError::Budget); };
        // Completion body: index, framed full request, kind, framed result.
        let completion_body = request_len.checked_add(28).and_then(|n| n.checked_add(result_len));
        let total = completion_body.and_then(|n| n.checked_add(9)).and_then(|n| self.prefix_bytes.checked_add(n));
        let Some(total) = total else { return self.fail(IndexedError::Budget); };
        if key_len > self.registry.limits.max_key_bytes || total > self.registry.limits.max_prefix_bytes {
            return self.fail(IndexedError::Budget);
        }
        let mut query = self.prefix.clone();
        query.update(&[REQUEST]);
        query.update(&request_len.to_le_bytes());
        request.absorb(&mut query);
        let mut answer = FiniteAnswer { reader: query.finalize_xof(), declared: request.raw_len, position: 0, error: None };
        // Mutate the original unfinalized prefix, never the squeezing clone.
        // Failure need not roll it back: poison forbids every descendant.
        self.error = Some(IndexedError::Incomplete);
        let mut completed = CompletedResult { prefix: &mut self.prefix, remaining: result_len, error: None };
        completed.prefix.update(&[COMPLETED]);
        completed.prefix.update(&(request_len + 28 + result_len).to_le_bytes());
        completed.prefix.update(&self.event_index.to_le_bytes());
        completed.prefix.update(&request_len.to_le_bytes());
        request.absorb(completed.prefix);
        completed.prefix.update(&(kind as u32).to_le_bytes());
        completed.prefix.update(&result_len.to_le_bytes());
        // Unwinding, an ignored decoder error, and an incomplete result cannot
        // leave a usable owner. Only a complete successful transaction clears it.
        let value = match decode(&mut answer, &mut completed) {
            Ok(value) => value,
            Err(error) => { self.error = Some(error); return Err(error); }
        };
        if let Some(error) = answer.error.or(completed.error) { self.error = Some(error); return Err(error); }
        if completed.remaining != 0 { self.error = Some(IndexedError::Completion); return Err(IndexedError::Completion); }
        let Some(ordinal) = self.request_ordinal.checked_add(1) else { self.error = Some(IndexedError::Budget); return Err(IndexedError::Budget); };
        self.prefix_bytes = total;
        self.request_ordinal = ordinal;
        self.error = None;
        self.advance_event()?;
        // Deliberately do not squeeze the unused tail. The reader defines the
        // full declared answer; the completed event owns the decoded result.
        Ok(value)
    }
    pub(super) fn finish(&self) -> Result<(), IndexedError> {
        self.ensure_valid()?;
        if self.active.is_some() || self.cursor != self.registry.events.len() { return Err(IndexedError::Incomplete); }
        Ok(())
    }
}

/// Bounded sequential capability for exactly one full literal-key answer.
/// There is no reset, seed, coordinate rekey, or unbounded reader accessor.
pub struct FiniteAnswer {
    reader: Shake256Reader,
    declared: u64,
    position: u64,
    error: Option<IndexedError>,
}
impl FiniteAnswer {
    pub const fn declared_len(&self) -> u64 { self.declared }
    pub const fn position(&self) -> u64 { self.position }
    pub const fn remaining(&self) -> u64 { self.declared - self.position }
    pub fn read_exact(&mut self, out: &mut [u8]) -> Result<(), IndexedError> {
        if let Some(error) = self.error { return Err(error); }
        let n = out.len() as u64;
        if n > self.remaining() { self.error = Some(IndexedError::AnswerBounds); return Err(IndexedError::AnswerBounds); }
        self.reader.read(out);
        self.position += n;
        Ok(())
    }
    /// Generate/discard the skipped prefix, without allocating or crossing the bound.
    pub fn skip_exact(&mut self, mut n: u64) -> Result<(), IndexedError> {
        if let Some(error) = self.error { return Err(error); }
        if n > self.remaining() { self.error = Some(IndexedError::AnswerBounds); return Err(IndexedError::AnswerBounds); }
        let mut scratch = [0u8; 136];
        while n != 0 {
            let count = n.min(scratch.len() as u64) as usize;
            self.read_exact(&mut scratch[..count])?;
            n -= count as u64;
        }
        Ok(())
    }
}
/// Streaming canonical decoded-result sink inside one atomic request.
/// The trusted decoder must emit its entire installed canonical codec, not a
/// digest or support-position permutation. Chunk boundaries are not framed.
pub struct CompletedResult<'prefix> { prefix: &'prefix mut Shake256, remaining: u64, error: Option<IndexedError> }
impl CompletedResult<'_> {
    pub const fn remaining(&self) -> u64 { self.remaining }
    pub fn write_all(&mut self, bytes: &[u8]) -> Result<(), IndexedError> {
        if let Some(error) = self.error { return Err(error); }
        let n = bytes.len() as u64;
        if n > self.remaining { self.error = Some(IndexedError::Completion); return Err(IndexedError::Completion); }
        self.prefix.update(bytes);
        self.remaining -= n;
        Ok(())
    }
}

/// Frozen canonical LE128 A7F7 scalar descriptor (version, width, bits, cap, q).
pub fn scalar_descriptor() -> [u8; 32] {
    let mut descriptor = [0u8; 32];
    descriptor[..4].copy_from_slice(&1u32.to_le_bytes());
    descriptor[4..8].copy_from_slice(&16u32.to_le_bytes());
    descriptor[8..12].copy_from_slice(&128u32.to_le_bytes());
    descriptor[12..16].copy_from_slice(&128u32.to_le_bytes());
    let q = u128::MAX - (1u128 << 32) + 22538;
    descriptor[16..].copy_from_slice(&q.to_le_bytes());
    descriptor
}
/// Separate low-bit predicate descriptor: version, target, nonce width, bit order, bytes.
pub fn predicate_descriptor(grind_bits: u8, nonce_bits: u8) -> Result<[u8; 8], IndexedError> {
    if grind_bits == 0 || grind_bits > crate::MAX_GRINDING_BITS
        || nonce_bits != grind_bits + crate::GRINDING_NONCE_SLACK_BITS { return Err(IndexedError::Installation); }
    Ok([1, 0, 0, 0, grind_bits, nonce_bits, crate::GRINDING_LITTLE_ENDIAN_BIT_ORDER, 32])
}
fn valid_predicate_descriptor(bytes: &[u8]) -> bool {
    let Ok(bytes) = <&[u8; 8]>::try_from(bytes) else { return false; };
    predicate_descriptor(bytes[4], bytes[5]).is_ok_and(|expected| expected == *bytes)
}

#[cfg(test)]
mod tests;
