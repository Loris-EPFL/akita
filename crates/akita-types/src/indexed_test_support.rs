//! Fixed, verifier-installed grammars for native channel unit fixtures.

use akita_params::GrindingSite;
use akita_transcript::{
    predicate_descriptor, CompletedResultKind, IndexedLimits,
    IndexedOperation, IndexedRequest, InstalledContext, InstalledEvent, InstalledRegistry,
    ProtocolContextRecord, ProtocolMessageKind, ProtocolSiteId, RequestSpec,
};
use jolt_field::{CanonicalEncoding, ExtField, Field};

pub(crate) fn context(session: &[u8], events: Vec<InstalledEvent>) -> InstalledContext {
    let limits = IndexedLimits {
        max_context_bytes: 1 << 20,
        max_prefix_bytes: 1 << 40,
        max_key_bytes: 1 << 40,
        max_descriptor_bytes: 1 << 20,
        max_answer_bytes: 1 << 60,
        max_result_bytes: 1 << 40,
        max_proof_bytes: 1 << 40,
    };
    let registry = InstalledRegistry::new(1, events, limits).unwrap();
    InstalledContext::new(&registry, session, b"fixed approved test instance/v8").unwrap()
}

pub(crate) fn extension_message<F, E>(
    site: ProtocolSiteId,
    kind: ProtocolMessageKind,
    values: usize,
) -> InstalledEvent
where
    F: Field + CanonicalEncoding,
    E: ExtField<F>,
{
    let atoms = values.checked_mul(E::DEGREE).unwrap();
    InstalledEvent::Message(ProtocolContextRecord::new(
        site.to_bytes(), kind as u32, atoms as u64, (atoms * F::NUM_BYTES) as u64, 0,
    ))
}

pub(crate) fn field_draw<F, E>(events: &mut Vec<InstalledEvent>, site: ProtocolSiteId)
where
    F: Field + CanonicalEncoding,
    E: ExtField<F>,
{
    let ordinal = events.iter().filter(|event| matches!(event, InstalledEvent::Request(_))).count() as u64;
    events.push(InstalledEvent::Request(
        akita_transcript::field_request_spec::<F, E>(ordinal, site).unwrap(),
    ));
}

pub(crate) fn grinding(events: &mut Vec<InstalledEvent>, site: GrindingSite, grind_bits: u8, nonce_bits: u8) {
    if grind_bits == 0 {
        return;
    }
    let site_id = site.site_id(u32::from(grind_bits) | (u32::from(nonce_bits) << 8));
    events.push(InstalledEvent::Message(ProtocolContextRecord::new(
        site_id.to_bytes(), ProtocolMessageKind::GrindingNonce as u32, 1,
        akita_transcript::nonce_max_bytes(nonce_bits) as u64, 0,
    )));
    let ordinal = events.iter().filter(|event| matches!(event, InstalledEvent::Request(_))).count() as u64;
    events.push(InstalledEvent::Request(RequestSpec {
        request: IndexedRequest::new(
            IndexedOperation::Predicate32, ordinal, site_id,
            predicate_descriptor(grind_bits, nonce_bits).unwrap().to_vec(), 32,
        ).unwrap(),
        result_kind: CompletedResultKind::Predicate32,
        result_len: 32,
    }));
}

