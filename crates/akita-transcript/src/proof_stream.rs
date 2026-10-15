//! Owned v8 indexed channels and canonical Akita message codecs.

use akita_error::AkitaError;
use jolt_field::{CanonicalEncoding, ExtField, Field};
use spongefish::{Encoding, NargDeserialize, VerificationError};
use std::marker::PhantomData;

mod nonce;
pub use nonce::{nonce_encoded_len, nonce_max_bytes, NonceAtom};
mod channel;
mod sampling;
pub use channel::{IndexedChannel, ProofChannel};
pub use sampling::{
    field_challenge_bytes, field_request_spec, field_sampling_is_certified, field_vector_challenge_at,
    field_vector_descriptor, prime_descriptor, prover_field_challenge, verifier_field_challenge,
    FIELD_CANDIDATE_CAP, FIELD_CHALLENGE_BYTES, FIELD_SAMPLING_QUERY_LIMIT,
};
mod verifier;
pub use verifier::VerifierChannel;
mod site;

mod indexed;
pub use indexed::{
    predicate_descriptor, scalar_descriptor, CompletedResult, CompletedResultKind, FiniteAnswer,
    IndexedError, IndexedLimits, IndexedOperation, IndexedRequest, InheritedAugmentation, InstalledContext,
    InstalledEvent, InstalledRegistry, RequestSpec, INDEXED_PROVIDER,
};
mod prover;
pub use prover::ProverChannel;
mod preview;
pub use preview::FoldPreview;
/// Proof channel and proof-stream format version.
pub const PROTOCOL_VERSION: u32 = 8;

/// Domain tag stored in every canonical event-context record.
pub const CONTEXT_DOMAIN: [u8; 32] = *b"akita-pcs/native-context/v8\0\0\0\0\0";

/// Stable family identifier for standard and batched sumcheck sites.
pub const SITE_FAMILY_SUMCHECK: u32 = 1;

/// Stable family identifier for extension-opening reduction messages.
pub const SITE_FAMILY_EXTENSION_OPENING_REDUCTION: u32 = 2;

/// Stable family identifier for stage-2 terminal claims.
pub const SITE_FAMILY_STAGE2: u32 = 3;

/// Stable family identifier for stage-1 late oracle claims.
pub const SITE_FAMILY_STAGE1: u32 = 4;

/// Stable family identifier for physical-L2 proof values.
pub const SITE_FAMILY_PHYSICAL_L2: u32 = 5;

/// Stable family identifier for recursive setup-product stage 3.
pub const SITE_FAMILY_STAGE3: u32 = 6;

/// Stable family identifier for indexed sparse fold-challenge roots.
pub const SITE_FAMILY_FOLD_CHALLENGE: u32 = 7;

/// Stable family identifier for ring-relation opening payloads.
pub const SITE_FAMILY_OPENING_PAYLOAD: u32 = 8;

/// Stable family identifier for derived fold-opening values.
pub const SITE_FAMILY_FOLD_BINDING: u32 = 9;

/// Stable family identifier for the successor witness binding.
pub const SITE_FAMILY_NEXT_WITNESS: u32 = 10;

/// Stable family identifier for root public commitments and opening points.
pub const SITE_FAMILY_ROOT_STATEMENT: u32 = 11;

/// Stable family identifier for terminal response messages.
pub const SITE_FAMILY_TERMINAL: u32 = 12;

/// Installed proof-stream operation kind bound by canonical event framing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ProtocolMessageKind {
    /// A public or derived value supplied outside the argument string.
    PublicValue = 1,
    /// A bounded variable payload's length atom.
    ProofLength = 2,
    /// One or more canonical proof atoms.
    ProofAtoms = 3,
    /// A verifier challenge group.
    Challenge = 4,
    /// A proof-of-work nonce atom.
    GrindingNonce = 5,
    /// A proof-of-work predicate challenge.
    GrindingPredicate = 6,
    /// A fold-response search nonce atom.
    FoldResponseNonce = 7,
}

/// Canonical 32-byte identity of one protocol site.
///
/// Unused coordinates are zero. Callers derive every coordinate from validated
/// public schedule state; proof input never selects a site.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ProtocolSiteId {
    /// Protocol family, such as sumcheck or folding.
    pub family: u32,
    /// Invocation within the enclosing public schedule.
    pub invocation: u32,
    /// Recursive fold level.
    pub level: u32,
    /// Stage within a level.
    pub stage: u32,
    /// Round within a stage.
    pub round: u32,
    /// Commitment group within a round.
    pub group: u32,
    /// Base-field limb within an extension challenge.
    pub limb: u32,
    /// Family-specific public discriminator, such as grinding widths.
    pub detail: u32,
}

impl ProtocolSiteId {
    /// Encode this site identity in its fixed canonical layout.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; 32] {
        let mut encoded = [0u8; 32];
        let family = self.family.to_le_bytes();
        let invocation = self.invocation.to_le_bytes();
        let level = self.level.to_le_bytes();
        let stage = self.stage.to_le_bytes();
        let round = self.round.to_le_bytes();
        let group = self.group.to_le_bytes();
        let limb = self.limb.to_le_bytes();
        let detail = self.detail.to_le_bytes();
        encoded[0] = family[0];
        encoded[1] = family[1];
        encoded[2] = family[2];
        encoded[3] = family[3];
        encoded[4] = invocation[0];
        encoded[5] = invocation[1];
        encoded[6] = invocation[2];
        encoded[7] = invocation[3];
        encoded[8] = level[0];
        encoded[9] = level[1];
        encoded[10] = level[2];
        encoded[11] = level[3];
        encoded[12] = stage[0];
        encoded[13] = stage[1];
        encoded[14] = stage[2];
        encoded[15] = stage[3];
        encoded[16] = round[0];
        encoded[17] = round[1];
        encoded[18] = round[2];
        encoded[19] = round[3];
        encoded[20] = group[0];
        encoded[21] = group[1];
        encoded[22] = group[2];
        encoded[23] = group[3];
        encoded[24] = limb[0];
        encoded[25] = limb[1];
        encoded[26] = limb[2];
        encoded[27] = limb[3];
        encoded[28] = detail[0];
        encoded[29] = detail[1];
        encoded[30] = detail[2];
        encoded[31] = detail[3];
        encoded
    }
}

/// Construct an ordinary v8 prover from approved installed context.
pub fn new_prover_channel(context: &InstalledContext) -> Result<ProverChannel, AkitaError> {
    Ok(ProverChannel::new(context))
}

/// Construct an ordinary v8 verifier from the same approved context.
pub fn new_verifier_channel<'proof>(
    context: &InstalledContext,
    proof: &'proof [u8],
) -> Result<VerifierChannel<'proof>, AkitaError> {
    VerifierChannel::new(context, proof).map_err(Into::into)
}

/// A fixed-width, canonical field atom for proof transport.
///
/// This is deliberately an atom rather than a shape-aware container. Runtime
/// proof shapes are enforced by schedule-derived receive loops in the protocol
/// crates, while this decoder only accepts canonical field representatives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldAtom<F: CanonicalEncoding>(F);

impl<F: CanonicalEncoding> FieldAtom<F> {
    /// Wrap one field element for proof transport.
    #[must_use]
    pub const fn new(value: F) -> Self {
        Self(value)
    }

    /// Return the wrapped field element.
    #[must_use]
    pub const fn into_inner(self) -> F {
        self.0
    }
}

/// Canonical extension-field proof atom with transactional decoding.
///
/// All base coordinates are decoded against a local cursor. The caller's
/// cursor advances only after every coordinate is present and canonical.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtensionAtom<F, E> {
    value: E,
    _base: PhantomData<F>,
}

impl<F, E> ExtensionAtom<F, E> {
    /// Wrap one extension element for proof transport.
    #[must_use]
    pub const fn new(value: E) -> Self {
        Self {
            value,
            _base: PhantomData,
        }
    }

    /// Return the wrapped extension element.
    #[must_use]
    pub fn into_inner(self) -> E {
        self.value
    }
}

impl<F, E> Encoding<[u8]> for ExtensionAtom<F, E>
where
    F: Field + CanonicalEncoding,
    E: ExtField<F>,
{
    fn encode(&self) -> impl AsRef<[u8]> {
        let mut out = Vec::new();
        for limb in 0..E::DEGREE {
            out.extend_from_slice(FieldAtom::new(self.value.base_coefficient(limb)).encode().as_ref());
        }
        out
    }
}

impl<F, E> NargDeserialize for ExtensionAtom<F, E>
where
    F: Field + CanonicalEncoding,
    E: ExtField<F>,
{
    fn deserialize_from_narg(buf: &mut &[u8]) -> Result<Self, VerificationError> {
        let mut remaining = *buf;
        let required = E::DEGREE.checked_mul(F::NUM_BYTES).ok_or(VerificationError)?;
        if remaining.len() < required { return Err(VerificationError); }
        let mut coefficients = Vec::new();
        coefficients
            .try_reserve_exact(E::DEGREE)
            .map_err(|_| VerificationError)?;
        for _ in 0..E::DEGREE {
            coefficients.push(FieldAtom::<F>::deserialize_from_narg(&mut remaining)?.into_inner());
        }
        let value = E::from_base_slice(&coefficients);
        *buf = remaining;
        Ok(Self::new(value))
    }
}

// Ordinary admitted field atoms fit a stack buffer. The generic standalone
// codec retains its wider encoding without changing its canonical wire bytes.
enum FieldEncoding {
    Inline { bytes: [u8; 16], len: usize },
    Wide(Vec<u8>),
}
impl AsRef<[u8]> for FieldEncoding {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Inline { bytes, len } => &bytes[..*len],
            Self::Wide(bytes) => bytes,
        }
    }
}

impl<F: CanonicalEncoding> Encoding<[u8]> for FieldAtom<F> {
    fn encode(&self) -> impl AsRef<[u8]> {
        if F::NUM_BYTES <= 16 {
            let mut bytes = [0u8; 16];
            self.0.to_bytes_le(&mut bytes[..F::NUM_BYTES]);
            FieldEncoding::Inline { bytes, len: F::NUM_BYTES }
        } else {
            FieldEncoding::Wide(self.0.to_bytes_le_vec())
        }
    }
}

impl<F: CanonicalEncoding> NargDeserialize for FieldAtom<F> {
    fn deserialize_from_narg(buf: &mut &[u8]) -> Result<Self, VerificationError> {
        let (encoded, remaining) = buf
            .split_at_checked(F::NUM_BYTES)
            .ok_or(VerificationError)?;
        let value = F::from_bytes_le_checked(encoded).ok_or(VerificationError)?;
        *buf = remaining;
        Ok(Self(value))
    }
}

/// Fixed-width little-endian `u128` proof atom.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct U128Atom(u128);

impl U128Atom {
    /// Wrap an integer for proof transport.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Return the wrapped integer.
    #[must_use]
    pub const fn into_inner(self) -> u128 {
        self.0
    }
}

impl Encoding<[u8]> for U128Atom {
    fn encode(&self) -> impl AsRef<[u8]> {
        self.0.to_le_bytes()
    }
}

impl NargDeserialize for U128Atom {
    fn deserialize_from_narg(buf: &mut &[u8]) -> Result<Self, VerificationError> {
        let (encoded, remaining) = buf.split_at_checked(16).ok_or(VerificationError)?;
        let bytes: [u8; 16] = encoded.try_into().map_err(|_| VerificationError)?;
        *buf = remaining;
        Ok(Self(u128::from_le_bytes(bytes)))
    }
}

/// Emit one canonical field atom and absorb the same bytes.
pub fn send_field<F: CanonicalEncoding>(state: &mut ProverChannel, value: F) -> Result<(), AkitaError> {
    state.prover_message(&FieldAtom::new(value))
}

/// Receive one canonical field atom, rejecting noncanonical representatives.
pub fn receive_field<F: CanonicalEncoding>(
    state: &mut VerifierChannel<'_>,
) -> Result<F, AkitaError> {
    state
        .prover_message::<FieldAtom<F>>()
        .map(FieldAtom::into_inner)
}

/// Emit one extension-field proof atom as ordered canonical base coordinates.
pub fn send_extension<F, E>(state: &mut ProverChannel, value: E) -> Result<(), AkitaError>
where
    F: Field + CanonicalEncoding,
    E: ExtField<F>,
{
    state.prover_message(&ExtensionAtom::<F, E>::new(value))
}

/// Receive one extension-field proof atom from canonical base coordinates.
pub fn receive_extension<F, E>(state: &mut VerifierChannel<'_>) -> Result<E, AkitaError>
where
    F: Field + CanonicalEncoding,
    E: ExtField<F>,
{
    state
        .prover_message::<ExtensionAtom<F, E>>()
        .map(ExtensionAtom::into_inner)
}

/// Emit an exact logical event body without introducing transport-chunk events.
pub fn send_bytes(state: &mut ProverChannel, bytes: &[u8]) -> Result<(), AkitaError> {
    state.send_raw(bytes).map_err(Into::into)
}

/// Receive an exact schedule-bounded logical event body.
pub fn receive_bytes(state: &mut VerifierChannel<'_>, len: usize) -> Result<Vec<u8>, AkitaError> {
    state.receive_raw(len).map_err(Into::into)
}

/// Emit a schedule-bounded byte sequence as one installed logical proof group.
pub fn send_byte_group(
    state: &mut ProverChannel,
    site: ProtocolSiteId,
    bytes: &[u8],
) -> Result<(), AkitaError> {
    let len = u64::try_from(bytes.len()).map_err(|_| { state.invalidate(); AkitaError::InvalidProof })?;
    prover_context(
        state,
        ProtocolContextRecord::new(
            site.to_bytes(),
            ProtocolMessageKind::ProofAtoms as u32,
            len,
            len,
            0,
        ),
    )?;
    send_bytes(state, bytes)
}

/// Receive one exact schedule-bounded installed logical proof group.
pub fn receive_byte_group(
    state: &mut VerifierChannel<'_>,
    site: ProtocolSiteId,
    len: usize,
) -> Result<Vec<u8>, AkitaError> {
    let len_u64 = u64::try_from(len).map_err(|_| { state.invalidate(); AkitaError::InvalidProof })?;
    verifier_context(
        state,
        ProtocolContextRecord::new(
            site.to_bytes(),
            ProtocolMessageKind::ProofAtoms as u32,
            len_u64,
            len_u64,
            0,
        ),
    )?;
    receive_bytes(state, len)
}

/// Length-atom record and payload site of one bounded byte payload at `site`.
pub fn bounded_bytes_sites(site: ProtocolSiteId) -> (ProtocolContextRecord, ProtocolSiteId) {
    let length_site = ProtocolSiteId { stage: 0, ..site };
    let length_record = ProtocolContextRecord::new(
        length_site.to_bytes(),
        ProtocolMessageKind::ProofLength as u32,
        1,
        4,
        0,
    );
    (length_record, ProtocolSiteId { stage: 1, ..site })
}

/// Emit a bounded variable byte payload with a `u32` length atom.
pub fn send_bounded_bytes(
    state: &mut ProverChannel,
    site: ProtocolSiteId,
    bytes: &[u8],
    max_len: usize,
) -> Result<(), AkitaError> {
    if bytes.len() > max_len {
        state.invalidate();
        return Err(AkitaError::InvalidProof);
    }
    let len = u32::try_from(bytes.len()).map_err(|_| { state.invalidate(); AkitaError::InvalidProof })?;
    let (length_record, payload_site) = bounded_bytes_sites(site);
    prover_context(state, length_record)?;
    state.prover_message(&len)?;
    send_byte_group(state, payload_site, bytes)
}

/// Receive a bounded variable byte payload after validating its length
/// atom and before allocating its body.
pub fn receive_bounded_bytes(
    state: &mut VerifierChannel<'_>,
    site: ProtocolSiteId,
    max_len: usize,
) -> Result<Vec<u8>, AkitaError> {
    let (length_record, payload_site) = bounded_bytes_sites(site);
    verifier_context(state, length_record)?;
    let len = state.prover_message::<u32>()?;
    let len = usize::try_from(len).map_err(|_| {
        state.invalidate();
        AkitaError::InvalidProof
    })?;
    if len > max_len {
        state.invalidate();
        return Err(AkitaError::InvalidProof);
    }
    receive_byte_group(state, payload_site, len)
}

/// Plan one canonical public byte event without absorbing it.
pub fn public_bytes_record(
    site: ProtocolSiteId,
    len: usize,
) -> Result<ProtocolContextRecord, AkitaError> {
    let len = u64::try_from(len).map_err(|_| AkitaError::InvalidProof)?;
    Ok(ProtocolContextRecord::new(
        site.to_bytes(),
        ProtocolMessageKind::PublicValue as u32,
        len,
        len,
        0,
    ))
}

/// Absorb one public byte string at its installed canonical site.
pub fn public_bytes<S: ProofChannel>(
    state: &mut S,
    site: ProtocolSiteId,
    bytes: &[u8],
) -> Result<(), AkitaError> {
    let record = checked_helper(state, public_bytes_record(site, bytes.len()))?;
    state.context(record)?;
    state.public(bytes)
}

fn checked_helper<S: IndexedChannel, T>(state: &mut S, result: Result<T, AkitaError>) -> Result<T, AkitaError> {
    if result.is_err() { state.invalidate(); }
    result
}

/// Draw a context-bound extension-field challenge.
pub fn ext_challenge<F, E, S>(state: &mut S, site: ProtocolSiteId) -> Result<E, AkitaError>
where
    F: Field + CanonicalEncoding,
    E: ExtField<F>,
    S: ProofChannel,
{
    state.field_vector_challenge_at::<F, E>(site)
}

/// Fixed-width record describing one installed logical event transaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProtocolContextRecord {
    /// Fixed domain for all Akita context records.
    pub domain: [u8; 32],
    /// Context-record format version.
    pub version: u32,
    /// Canonical protocol-site identity.
    pub site_id: [u8; 32],
    /// Message or challenge kind.
    pub kind: u32,
    /// Number of fixed-width atoms in the group.
    pub atom_count: u64,
    /// Total canonical encoded bytes in the group.
    pub encoded_bytes: u64,
    /// Number of challenge bytes squeezed after the record.
    pub challenge_bytes: u64,
}

impl ProtocolContextRecord {
    /// Construct a versioned context record.
    #[must_use]
    pub const fn new(
        site_id: [u8; 32],
        kind: u32,
        atom_count: u64,
        encoded_bytes: u64,
        challenge_bytes: u64,
    ) -> Self {
        Self {
            domain: CONTEXT_DOMAIN,
            version: PROTOCOL_VERSION,
            site_id,
            kind,
            atom_count,
            encoded_bytes,
            challenge_bytes,
        }
    }
}

/// Begin the next installed prover event; diagnostics only observe it.
#[inline(always)]
pub fn prover_context(state: &mut ProverChannel, record: ProtocolContextRecord) -> Result<(), AkitaError> {
    state.core.context(record)?;
    #[cfg(feature = "logging-transcript")]
    {
        crate::logging::record_context(record);
        crate::logging::record_proof_boundary(record, state.narg_string().len());
    }
    Ok(())
}

/// Close the final diagnostic proof range at the authoritative argument end.
#[cfg(feature = "logging-transcript")]
pub fn finish_proof_ranges(state: &ProverChannel) {
    crate::logging::finish_proof_ranges(state.narg_string().len());
}

/// Begin the next installed verifier event; diagnostics only observe it.
#[inline(always)]
pub fn verifier_context(state: &mut VerifierChannel<'_>, record: ProtocolContextRecord) -> Result<(), AkitaError> {
    state.core.context(record)?;
    #[cfg(feature = "logging-transcript")]
    crate::logging::record_context(record);
    Ok(())
}

/// Plan an exact extension group event without transporting or drawing values.
pub fn extension_group_record<F, E>(
    site: ProtocolSiteId,
    kind: ProtocolMessageKind,
    value_count: usize,
) -> Result<ProtocolContextRecord, AkitaError>
where
    F: Field + CanonicalEncoding,
    E: ExtField<F>,
{
    let atom_count = value_count
        .checked_mul(E::DEGREE)
        .ok_or(AkitaError::InvalidProof)?;
    let encoded_bytes = atom_count
        .checked_mul(F::NUM_BYTES)
        .ok_or(AkitaError::InvalidProof)?;
    Ok(ProtocolContextRecord::new(
        site.to_bytes(),
        kind as u32,
        u64::try_from(atom_count).map_err(|_| AkitaError::InvalidProof)?,
        u64::try_from(encoded_bytes).map_err(|_| AkitaError::InvalidProof)?,
        0,
    ))
}

/// Plan an exact base-field group event without transporting or drawing values.
pub fn field_group_record<F>(
    site: ProtocolSiteId,
    kind: ProtocolMessageKind,
    value_count: usize,
) -> Result<ProtocolContextRecord, AkitaError>
where
    F: CanonicalEncoding,
{
    let encoded_bytes = value_count
        .checked_mul(F::NUM_BYTES)
        .ok_or(AkitaError::InvalidProof)?;
    Ok(ProtocolContextRecord::new(
        site.to_bytes(),
        kind as u32,
        u64::try_from(value_count).map_err(|_| AkitaError::InvalidProof)?,
        u64::try_from(encoded_bytes).map_err(|_| AkitaError::InvalidProof)?,
        0,
    ))
}

/// Absorb a fixed-count group of public base-field values.
pub fn public_fields_prover<F>(
    state: &mut ProverChannel,
    site: ProtocolSiteId,
    values: &[F],
) -> Result<(), AkitaError>
where
    F: CanonicalEncoding,
{
    let record = checked_helper(state, field_group_record::<F>(site, ProtocolMessageKind::PublicValue, values.len()))?;
    prover_context(state, record)?;
    for &value in values {
        state.public_message(&FieldAtom::new(value))?;
    }
    Ok(())
}

/// Absorb a fixed-count group of public base-field values.
pub fn public_fields_verifier<F>(
    state: &mut VerifierChannel<'_>,
    site: ProtocolSiteId,
    values: &[F],
) -> Result<(), AkitaError>
where
    F: CanonicalEncoding,
{
    let record = checked_helper(state, field_group_record::<F>(site, ProtocolMessageKind::PublicValue, values.len()))?;
    verifier_context(state, record)?;
    for &value in values {
        state.public_message(&FieldAtom::new(value))?;
    }
    Ok(())
}

/// Emit a fixed-count group of canonical base-field proof atoms.
pub fn send_field_group<F>(
    state: &mut ProverChannel,
    site: ProtocolSiteId,
    values: &[F],
) -> Result<(), AkitaError>
where
    F: CanonicalEncoding,
{
    let record = checked_helper(state, field_group_record::<F>(site, ProtocolMessageKind::ProofAtoms, values.len()))?;
    prover_context(state, record)?;
    for &value in values {
        send_field(state, value)?;
    }
    Ok(())
}

/// Receive a schedule-fixed group of canonical base-field proof atoms.
pub fn receive_field_group<F>(
    state: &mut VerifierChannel<'_>,
    site: ProtocolSiteId,
    value_count: usize,
) -> Result<Vec<F>, AkitaError>
where
    F: CanonicalEncoding,
{
    let record = checked_helper(state, field_group_record::<F>(site, ProtocolMessageKind::ProofAtoms, value_count))?;
    verifier_context(state, record)?;
    let mut values = Vec::new();
    values
        .try_reserve_exact(value_count)
        .map_err(|_| { state.invalidate(); AkitaError::InvalidProof })?;
    for _ in 0..value_count {
        values.push(receive_field(state)?);
    }
    Ok(values)
}

/// Absorb a fixed-count group of public extension-field values.
pub fn public_extensions<F, E, S>(
    state: &mut S,
    site: ProtocolSiteId,
    values: &[E],
) -> Result<(), AkitaError>
where
    F: Field + CanonicalEncoding,
    E: ExtField<F>,
    S: ProofChannel,
{
    let record = checked_helper(state, extension_group_record::<F, E>(site, ProtocolMessageKind::PublicValue, values.len()))?;
    state.context(record)?;
    for value in values {
        for limb in 0..E::DEGREE {
            let coefficient = value.base_coefficient(limb);
            state.public(&FieldAtom::new(coefficient))?;
        }
    }
    Ok(())
}

/// Allocate `count` zeroed extension slots for an in-place exchange.
///
/// # Errors
///
/// Returns [`AkitaError::InvalidProof`] when the allocation cannot be reserved.
pub fn extension_slots<E: Field>(count: usize) -> Result<Vec<E>, AkitaError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| AkitaError::InvalidProof)?;
    values.resize(count, E::zero());
    Ok(values)
}

/// Exchange a schedule-fixed group of extension-field proof values in place.
///
/// The prover emits `values`; the verifier overwrites them with the received
/// atoms. The group length is public and comes from the schedule.
///
/// # Errors
///
/// Returns [`AkitaError::InvalidProof`] when the group record overflows or
/// the verifier cannot decode an atom.
pub fn exchange_extension_group<F, E, S>(
    state: &mut S,
    site: ProtocolSiteId,
    values: &mut [E],
) -> Result<(), AkitaError>
where
    F: Field + CanonicalEncoding,
    E: ExtField<F>,
    S: ProofChannel,
{
    let record = checked_helper(state, extension_group_record::<F, E>(site, ProtocolMessageKind::ProofAtoms, values.len()))?;
    state.context(record)?;
    for value in values {
        let mut atom = ExtensionAtom::<F, E>::new(*value);
        state.exchange(&mut atom)?;
        *value = atom.into_inner();
    }
    Ok(())
}

fn grinding_record_owners(nonce: ProtocolContextRecord, predicate: ProtocolContextRecord) -> Result<(), AkitaError> {
    if nonce.kind != ProtocolMessageKind::GrindingNonce as u32 || nonce.site_id != predicate.site_id {
        return Err(AkitaError::InvalidProof);
    }
    Ok(())
}

/// Validate the predicate site and bounded nonce against its installed descriptor.
fn checked_predicate_request<S: IndexedChannel>(
    state: &mut S,
    predicate_record: ProtocolContextRecord,
    nonce: u32,
) -> Result<IndexedRequest, AkitaError> {
    if predicate_record.version != PROTOCOL_VERSION
        || predicate_record.domain != CONTEXT_DOMAIN
        || predicate_record.kind != ProtocolMessageKind::GrindingPredicate as u32
        || predicate_record.atom_count != 0 || predicate_record.encoded_bytes != 0
        || predicate_record.challenge_bytes != 32
    {
        state.invalidate();
        return Err(AkitaError::InvalidProof);
    }
    let request = state.request_at(IndexedOperation::Predicate32,
        ProtocolSiteId::from_bytes(predicate_record.site_id))?;
    let Some(&nonce_bits) = request.descriptor().get(5) else {
        state.invalidate();
        return Err(AkitaError::InvalidProof);
    };
    if u64::from(nonce) >= (1u64 << nonce_bits) {
        state.invalidate();
        return Err(AkitaError::InvalidProof);
    }
    Ok(request)
}

/// Preview the complete nonce/predicate event sequence without live mutation.
pub fn preview_grinding_predicate(
    state: &ProverChannel,
    nonce_record: ProtocolContextRecord,
    nonce: u32,
    predicate_record: ProtocolContextRecord,
) -> Result<[u8; 32], AkitaError> {
    grinding_record_owners(nonce_record, predicate_record)?;
    let mut preview = FoldPreview::new(state, nonce_record, nonce)?;
    let request = checked_predicate_request(&mut preview, predicate_record, nonce)?;
    preview.predicate_at(request.site()).map_err(Into::into)
}

/// Search the bounded nonce policy without advancing the live grammar.
/// Zero-bit PoW has no transcript event; its logical plan is parent-owned.
pub fn search_grinding_nonce(
    state: &mut ProverChannel,
    grind_bits: u8,
    nonce_bits: u8,
    nonce_record: ProtocolContextRecord,
    predicate_record: ProtocolContextRecord,
) -> Result<Option<(u32, [u8; 32])>, AkitaError> {
    state.status()?;
    if grind_bits == 0 {
        if nonce_bits == 0 { return Ok(Some((0, [0; 32]))); }
        state.invalidate();
        return Err(AkitaError::InvalidProof);
    }
    checked_helper(state, grinding_record_owners(nonce_record, predicate_record))?;
    let descriptor = match predicate_descriptor(grind_bits, nonce_bits) {
        Ok(descriptor) => descriptor,
        Err(error) => { state.invalidate(); return Err(error.into()); }
    };
    let outcome = crate::grinding::search_grinding_nonce_with(grind_bits, nonce_bits, |nonce| {
        let mut preview = FoldPreview::new(state, nonce_record, nonce)?;
        let request = checked_predicate_request(&mut preview, predicate_record, nonce)?;
        if request.descriptor() != descriptor { return Err(AkitaError::InvalidProof); }
        preview.predicate_at(request.site()).map_err(Into::into)
    });
    if !matches!(outcome, Ok(Some(_))) { state.invalidate(); }
    outcome
}

/// Commit a canonical winning nonce and complete its separate predicate family.
pub fn commit_grinding_nonce(
    state: &mut ProverChannel,
    nonce_record: ProtocolContextRecord,
    nonce: u32,
    predicate_record: ProtocolContextRecord,
) -> Result<[u8; 32], AkitaError> {
    checked_helper(state, grinding_record_owners(nonce_record, predicate_record))?;
    prover_context(state, nonce_record)?;
    state.prover_message(&NonceAtom::new(nonce))?;
    let request = checked_predicate_request(state, predicate_record, nonce)?;
    let predicate = state.predicate_at(request.site())?;
    checked_winning_predicate(state, &request, predicate)
}

/// Receive a canonical bounded nonce and complete the same predicate request.
pub fn receive_grinding_nonce(
    state: &mut VerifierChannel<'_>,
    nonce_record: ProtocolContextRecord,
    predicate_record: ProtocolContextRecord,
) -> Result<(u32, [u8; 32]), AkitaError> {
    checked_helper(state, grinding_record_owners(nonce_record, predicate_record))?;
    verifier_context(state, nonce_record)?;
    let nonce = state.prover_message::<NonceAtom>()?.into_inner();
    let request = checked_predicate_request(state, predicate_record, nonce)?;
    let predicate = state.predicate_at(request.site())?;
    Ok((nonce, checked_winning_predicate(state, &request, predicate)?))
}

fn checked_winning_predicate<S: IndexedChannel>(
    state: &mut S,
    request: &IndexedRequest,
    predicate: [u8; 32],
) -> Result<[u8; 32], AkitaError> {
    let bits = request.descriptor().get(4).copied()
        .and_then(std::num::NonZeroU8::new).ok_or(AkitaError::InvalidProof)?;
    if !crate::grinding_predicate_accepts(&predicate, bits) {
        state.invalidate();
        return Err(AkitaError::InvalidProof);
    }
    Ok(predicate)
}

