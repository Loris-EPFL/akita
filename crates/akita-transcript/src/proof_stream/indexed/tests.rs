use super::*;
use crate::{commit_grinding_nonce, new_prover_channel, new_verifier_channel,
    preview_grinding_predicate, prover_context, public_bytes, receive_grinding_nonce,
    search_grinding_nonce, send_byte_group, verifier_context, FieldAtom, FoldPreview,
    IndexedChannel, InheritedAugmentation, NonceAtom, U128Atom};
use jolt_field::{CanonicalBytes, CanonicalEncoding, Prime128OffsetA7F7 as K, Prime32Offset99};
use spongefish::{Encoding, NargDeserialize};

fn limits() -> IndexedLimits {
    IndexedLimits { max_context_bytes: 100_000, max_prefix_bytes: 1_000_000,
        max_key_bytes: 1_000_000, max_descriptor_bytes: 100_000,
        max_answer_bytes: 10_000_000, max_result_bytes: 100_000, max_proof_bytes: 100_000 }
}
fn site(stage: u32) -> ProtocolSiteId { ProtocolSiteId { family: 11, stage, ..ProtocolSiteId::default() } }
fn scalar(ordinal: u64, site: ProtocolSiteId) -> RequestSpec {
    RequestSpec { request: IndexedRequest::new(IndexedOperation::Scalar128, ordinal, site,
        Arc::<[u8]>::from(scalar_descriptor()), 2048).unwrap(), result_kind: CompletedResultKind::Scalar, result_len: 16 }
}
fn shell(ordinal: u64, stage: u32, raw_len: u64) -> RequestSpec {
    RequestSpec { request: IndexedRequest::new(IndexedOperation::ShellLevel, ordinal, site(stage),
        Arc::<[u8]>::from(&b"test/component/coefficient-vector/v1"[..]), raw_len).unwrap(),
        result_kind: CompletedResultKind::ShellLevel, result_len: 4 }
}
fn context(events: Vec<InstalledEvent>) -> InstalledContext {
    let registry = InstalledRegistry::new(19, events, limits()).unwrap();
    InstalledContext::new(&registry, b"session", b"complete augmented instance and endpoint roster").unwrap()
}
// A separate, materialized literal encoder; deliberately not the production
// incremental absorb helpers. Its SHAKE input is the executable v8 anchor.
fn frame(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    out.extend_from_slice(bytes);
}
fn header(index: u64, session: &[u8], instance: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    frame(&mut out, b"akita-pcs/indexed/v8/shake256");
    out.extend_from_slice(&8u32.to_le_bytes());
    out.extend_from_slice(&index.to_le_bytes());
    frame(&mut out, session);
    frame(&mut out, instance);
    out
}
fn request_bytes(request: &IndexedRequest) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&(request.operation as u32).to_le_bytes());
    out.extend_from_slice(&request.ordinal.to_le_bytes());
    out.extend_from_slice(&request.site.to_bytes());
    out.extend_from_slice(&request.raw_len.to_le_bytes());
    frame(&mut out, &request.descriptor);
    out
}
fn message(out: &mut Vec<u8>, index: u64, record: ProtocolContextRecord, bytes: &[u8]) {
    let mut body = Vec::new();
    body.extend_from_slice(&index.to_le_bytes());
    body.extend_from_slice(&record.site_id);
    body.extend_from_slice(&record.kind.to_le_bytes());
    body.extend_from_slice(&record.atom_count.to_le_bytes());
    body.extend_from_slice(&record.encoded_bytes.to_le_bytes());
    body.extend_from_slice(&record.challenge_bytes.to_le_bytes());
    frame(&mut body, bytes);
    out.push(if record.kind == ProtocolMessageKind::PublicValue as u32 { 1 } else { 2 });
    frame(out, &body);
}
fn completion(out: &mut Vec<u8>, index: u64, spec: &RequestSpec, bytes: &[u8]) {
    let mut body = Vec::new();
    body.extend_from_slice(&index.to_le_bytes());
    frame(&mut body, &request_bytes(&spec.request));
    body.extend_from_slice(&(spec.result_kind as u32).to_le_bytes());
    frame(&mut body, bytes);
    out.push(3);
    frame(out, &body);
}
fn literal_answer(prefix: &[u8], request: &IndexedRequest) -> Vec<u8> {
    let mut key = prefix.to_vec();
    key.push(4);
    frame(&mut key, &request_bytes(request));
    let mut hash = Shake256::default();
    hash.update(&key);
    let mut raw = vec![0; request.raw_len as usize];
    hash.finalize_xof().read(&mut raw);
    raw
}
fn reference_scalar(raw: &[u8]) -> [u8; 16] {
    let q = u128::MAX - (1u128 << 32) + 22538;
    for candidate in raw.chunks_exact(16) {
        let bytes: [u8; 16] = candidate.try_into().unwrap();
        if u128::from_le_bytes(bytes) < q { return bytes; }
    }
    panic!("reference finite scalar exhausted")
}

#[test]
fn cached_literal_keys_equal_materialized_keys_and_ignore_transport_chunking() {
    for len in [0, 1, 63, 64, 1023, 1024, 1025, 8193] {
        let payload: Vec<u8> = (0..len).map(|n| (n as u8).wrapping_mul(29).wrapping_add(7)).collect();
        let proof_record = ProtocolContextRecord::new(site(1).to_bytes(), 3, len as u64, len as u64, 0);
        let public_record = ProtocolContextRecord::new(site(2).to_bytes(), 1, len as u64, len as u64, 0);
        let first = scalar(0, site(3));
        let second = scalar(1, site(4));
        let ctx = context(vec![InstalledEvent::Message(proof_record), InstalledEvent::Message(public_record),
            InstalledEvent::Request(first.clone()), InstalledEvent::Request(second.clone())]);
        let mut prover = new_prover_channel(&ctx).unwrap();
        send_byte_group(&mut prover, site(1), &payload).unwrap();
        public_bytes(&mut prover, site(2), &payload).unwrap();
        let a = prover.field_challenge_at::<K>(site(3)).unwrap();
        let b = prover.field_challenge_at::<K>(site(4)).unwrap();
        let proof = prover.finish().unwrap();
        assert_eq!(proof, payload);
        let mut literal = header(19, b"session", b"complete augmented instance and endpoint roster");
        message(&mut literal, 0, proof_record, &payload);
        message(&mut literal, 1, public_record, &payload);
        let expected_a = reference_scalar(&literal_answer(&literal, &first.request));
        assert_eq!(a.to_bytes_le_vec(), expected_a);
        completion(&mut literal, 2, &first, &expected_a);
        let expected_b = reference_scalar(&literal_answer(&literal, &second.request));
        assert_eq!(b.to_bytes_le_vec(), expected_b, "descendant includes full completed scalar");
        let mut chunked = new_prover_channel(&ctx).unwrap();
        prover_context(&mut chunked, proof_record).unwrap();
        for chunk in payload.chunks(17) { chunked.send_raw(chunk).unwrap(); }
        prover_context(&mut chunked, public_record).unwrap();
        for chunk in payload.chunks(13) { chunked.core.message_bytes(chunk, true).unwrap(); }
        assert_eq!(chunked.field_challenge_at::<K>(site(3)).unwrap(), a);
        assert_eq!(chunked.field_challenge_at::<K>(site(4)).unwrap(), b);
        chunked.finish().unwrap();
        let mut verifier = new_verifier_channel(&ctx, &proof).unwrap();
        verifier_context(&mut verifier, proof_record).unwrap();
        assert_eq!(verifier.receive_raw(len).unwrap(), payload);
        public_bytes(&mut verifier, site(2), &payload).unwrap();
        assert_eq!(verifier.field_challenge_at::<K>(site(3)).unwrap(), a);
        assert_eq!(verifier.field_challenge_at::<K>(site(4)).unwrap(), b);
        verifier.check_eof().unwrap();
    }
}

#[test]
fn grammar_operation_site_descriptor_and_ordinals_are_not_aliases() {
    let planned = shell(0, 1, 64);
    for altered in [
        IndexedRequest::new(IndexedOperation::ShellLevel, 1, site(1), planned.request.descriptor.clone(), 64).unwrap(),
        IndexedRequest::new(IndexedOperation::ShellLevel, 0, site(2), planned.request.descriptor.clone(), 64).unwrap(),
        IndexedRequest::new(IndexedOperation::ShellLevel, 0, site(1), Arc::<[u8]>::from(&b"different codec"[..]), 64).unwrap(),
        IndexedRequest::new(IndexedOperation::ShellLevel, 0, site(1), planned.request.descriptor.clone(), 65).unwrap(),
        IndexedRequest::new(IndexedOperation::Predicate32, 0, site(1), Arc::<[u8]>::from(predicate_descriptor(1, 8).unwrap()), 32).unwrap(),
    ] {
        let mut channel = new_prover_channel(&context(vec![InstalledEvent::Request(planned.clone())])).unwrap();
        assert!(channel.execute_indexed(&altered, CompletedResultKind::ShellLevel, 4, |_, _| -> Result<(), IndexedError> { panic!("invalid request executed") }).is_err());
        assert!(channel.request_at(IndexedOperation::ShellLevel, site(1)).is_err());
        assert!(channel.finish().is_err());
    }
    let record = ProtocolContextRecord::new(site(1).to_bytes(), 1, 1, 1, 0);
    let ctx = context(vec![InstalledEvent::Message(record)]);
    let mut channel = new_prover_channel(&ctx).unwrap();
    prover_context(&mut channel, record).unwrap();
    assert!(channel.prover_message(&[7u8]).is_err(), "proof cannot stand in for public");
    assert!(channel.public_message(&[7u8]).is_err());
}

#[test]
fn completed_polynomial_bytes_own_descendants_and_unobserved_tail_is_lazy() {
    let first = shell(0, 1, 100_000);
    let second = scalar(1, site(2));
    let ctx = context(vec![InstalledEvent::Request(first.clone()), InstalledEvent::Request(second.clone())]);
    let mut descendants = Vec::new();
    for polynomial in [[1, 0, 0, 0], [255, 0, 0, 0]] {
        let mut channel = new_prover_channel(&ctx).unwrap();
        channel.execute_indexed(&first.request, CompletedResultKind::ShellLevel, 4, |answer, result| {
            let mut exposed = [0u8; 1];
            answer.read_exact(&mut exposed)?;
            assert_eq!(answer.declared_len(), 100_000);
            assert_eq!(answer.position(), 1);
            result.write_all(&polynomial[..2])?;
            result.write_all(&polynomial[2..])?;
            Ok(())
        }).unwrap();
        let value = channel.field_challenge_at::<K>(site(2)).unwrap();
        let mut literal = header(19, b"session", b"complete augmented instance and endpoint roster");
        completion(&mut literal, 0, &first, &polynomial);
        assert_eq!(value.to_bytes_le_vec(), reference_scalar(&literal_answer(&literal, &second.request)));
        descendants.push(value);
        channel.finish().unwrap();
    }
    assert_ne!(descendants[0], descendants[1]);
}

#[test]
fn ignored_reader_decoder_and_completion_errors_poison_even_at_eof() {
    let spec = shell(0, 1, 8);
    for mode in 0..4 {
        let mut channel = new_prover_channel(&context(vec![InstalledEvent::Request(spec.clone())])).unwrap();
        let outcome = channel.execute_indexed(&spec.request, CompletedResultKind::ShellLevel, 4, |answer, completed| {
            match mode {
                0 => { let _ = answer.read_exact(&mut [0; 9]); completed.write_all(&[1, 0, 0, 0])?; },
                1 => { let _ = answer.skip_exact(9); completed.write_all(&[1, 0, 0, 0])?; },
                2 => { let _ = completed.write_all(&[0; 5]); },
                _ => return Err(IndexedError::Decode),
            }
            Ok(())
        });
        assert!(outcome.is_err());
        assert!(channel.status().is_err());
        assert!(channel.finish().is_err());
    }
    let mut channel = new_prover_channel(&context(vec![InstalledEvent::Request(spec.clone())])).unwrap();
    assert_eq!(channel.execute_indexed(&spec.request, CompletedResultKind::ShellLevel, 4,
        |_, completed| completed.write_all(&[1])), Err(IndexedError::Completion));
    assert!(channel.finish().is_err());
}

#[test]
fn skip_exact_is_sequential_and_bounded_not_coordinate_rekeying() {
    let spec = shell(0, 1, 512);
    let ctx = context(vec![InstalledEvent::Request(spec.clone())]);
    let prefix = header(19, b"session", b"complete augmented instance and endpoint roster");
    let raw = literal_answer(&prefix, &spec.request);
    let mut channel = new_prover_channel(&ctx).unwrap();
    channel.execute_indexed(&spec.request, CompletedResultKind::ShellLevel, 4, |answer, completed| {
        answer.skip_exact(273)?;
        let mut bytes = [0; 17];
        answer.read_exact(&mut bytes)?;
        assert_eq!(&bytes, &raw[273..290]);
        assert_eq!(answer.remaining(), 222);
        completed.write_all(&[1, 0, 0, 0])
    }).unwrap();
    channel.finish().unwrap();
}

#[test]
fn preview_nonce_predicate_and_scalar_match_literal_live_and_verifier() {
    let nonce_site = ProtocolSiteId { detail: 1 | (8 << 8), ..site(1) };
    let nonce_record = ProtocolContextRecord::new(nonce_site.to_bytes(), 5, 1, 2, 0);
    let predicate_record = ProtocolContextRecord::new(nonce_site.to_bytes(), 6, 0, 0, 32);
    let predicate = RequestSpec { request: IndexedRequest::new(IndexedOperation::Predicate32, 0, nonce_site,
        Arc::<[u8]>::from(predicate_descriptor(1, 8).unwrap()), 32).unwrap(), result_kind: CompletedResultKind::Predicate32, result_len: 32 };
    let field = scalar(1, site(2));
    let ctx = context(vec![InstalledEvent::Message(nonce_record), InstalledEvent::Request(predicate.clone()), InstalledEvent::Request(field.clone())]);
    let mut channel = new_prover_channel(&ctx).unwrap();
    let before = channel.narg_string().to_vec();
    let (nonce, expected) = search_grinding_nonce(&mut channel, 1, 8, nonce_record, predicate_record).unwrap().unwrap();
    let preview_predicate = preview_grinding_predicate(&channel, nonce_record, nonce, predicate_record).unwrap();
    assert_eq!(preview_predicate, expected);
    assert_eq!(channel.narg_string(), before);
    let mut preview = FoldPreview::new(&channel, nonce_record, nonce).unwrap();
    assert_eq!(preview.predicate_at(nonce_site).unwrap(), preview_predicate);
    let preview_scalar = preview.field_challenge_at::<K>(site(2)).unwrap();
    let live = commit_grinding_nonce(&mut channel, nonce_record, nonce, predicate_record).unwrap();
    assert_eq!(live, preview_predicate);
    assert_eq!(channel.field_challenge_at::<K>(site(2)).unwrap(), preview_scalar);
    let proof = channel.finish().unwrap();
    assert_eq!(proof, NonceAtom::new(nonce).encode().as_ref());
    let mut literal = header(19, b"session", b"complete augmented instance and endpoint roster");
    message(&mut literal, 0, nonce_record, &proof);
    assert_eq!(live.as_slice(), literal_answer(&literal, &predicate.request));
    completion(&mut literal, 1, &predicate, &live);
    assert_eq!(preview_scalar.to_bytes_le_vec(), reference_scalar(&literal_answer(&literal, &field.request)));
    let mut verifier = new_verifier_channel(&ctx, &proof).unwrap();
    assert_eq!(receive_grinding_nonce(&mut verifier, nonce_record, predicate_record).unwrap(), (nonce, live));
    assert_eq!(verifier.field_challenge_at::<K>(site(2)).unwrap(), preview_scalar);
    verifier.check_eof().unwrap();
    let mut out_of_range = new_prover_channel(&ctx).unwrap();
    assert!(commit_grinding_nonce(&mut out_of_range, nonce_record, 256, predicate_record).is_err());
    assert!(out_of_range.finish().is_err());
    let mut losing = new_prover_channel(&ctx).unwrap();
    let nonce = (0..256).find(|&nonce| {
        preview_grinding_predicate(&losing, nonce_record, nonce, predicate_record).unwrap()[0] & 1 != 0
    }).unwrap();
    assert!(commit_grinding_nonce(&mut losing, nonce_record, nonce, predicate_record).is_err());
    assert!(losing.field_challenge_at::<K>(site(2)).is_err());
    assert!(losing.finish().is_err());
}

#[test]
fn truncated_noncanonical_and_trailing_proof_cannot_be_ignored() {
    let record = ProtocolContextRecord::new(site(1).to_bytes(), 3, 1, 16, 0);
    let ctx = context(vec![InstalledEvent::Message(record)]);
    for proof in [&[0u8; 15][..], &[255u8; 16][..]] {
        let mut verifier = new_verifier_channel(&ctx, proof).unwrap();
        verifier_context(&mut verifier, record).unwrap();
        assert!(verifier.prover_message::<FieldAtom<K>>().is_err());
        assert!(verifier.prover_message::<U128Atom>().is_err());
        assert!(verifier.check_eof().is_err());
    }
    let proof = [0u8; 17];
    let mut verifier = new_verifier_channel(&ctx, &proof).unwrap();
    verifier_context(&mut verifier, record).unwrap();
    verifier.prover_message::<FieldAtom<K>>().unwrap();
    assert_eq!(verifier.finish(), Err(IndexedError::TrailingProof));
    let mut incomplete = new_prover_channel(&ctx).unwrap();
    prover_context(&mut incomplete, record).unwrap();
    assert!(incomplete.finish().is_err());
}

#[test]
fn version_profile_and_whole_byte_budgets_are_checked() {
    let mut record = ProtocolContextRecord::new(site(1).to_bytes(), 3, 1, 1, 0);
    record.version = 7;
    assert!(InstalledRegistry::new(19, vec![InstalledEvent::Message(record)], limits()).is_err());
    let field = scalar(0, site(1));
    let mut channel = new_prover_channel(&context(vec![InstalledEvent::Request(field.clone())])).unwrap();
    assert!(channel.field_challenge_at::<Prime32Offset99>(site(1)).is_err());
    assert!(channel.field_challenge_at::<K>(site(1)).is_err());
    let mut too_small = limits();
    too_small.max_answer_bytes = 2047;
    assert!(InstalledRegistry::new(19, vec![InstalledEvent::Request(field.clone())], too_small).is_err());
    let mut too_small = limits();
    let literal = header(19, b"s", b"i");
    too_small.max_key_bytes = (literal.len() + 9 + request_bytes(&field.request).len() - 1) as u64;
    let registry = InstalledRegistry::new(19, vec![InstalledEvent::Request(field)], too_small).unwrap();
    let ctx = InstalledContext::new(&registry, b"s", b"i").unwrap();
    let mut channel = new_prover_channel(&ctx).unwrap();
    assert!(channel.field_challenge_at::<K>(site(1)).is_err());
    assert_eq!(channel.status(), Err(IndexedError::Budget));
}

#[test]
fn installed_header_identity_and_public_proof_tags_are_separate_domains() {
    let proof_record = ProtocolContextRecord::new(site(1).to_bytes(), 3, 1, 1, 0);
    let public_record = ProtocolContextRecord::new(site(1).to_bytes(), 1, 1, 1, 0);
    let first = scalar(0, site(2));
    let mut values = Vec::new();
    for (index, session, instance, record) in [
        (19, &b"a"[..], &b"bc"[..], proof_record),
        (19, &b"ab"[..], &b"c"[..], proof_record),
        (20, &b"a"[..], &b"bc"[..], proof_record),
        (19, &b"a"[..], &b"bc"[..], public_record),
    ] {
        let registry = InstalledRegistry::new(index, vec![InstalledEvent::Message(record), InstalledEvent::Request(first.clone())], limits()).unwrap();
        let ctx = InstalledContext::new(&registry, session, instance).unwrap();
        let mut channel = new_prover_channel(&ctx).unwrap();
        prover_context(&mut channel, record).unwrap();
        if record.kind == 1 { channel.public_message(&[7u8]).unwrap(); }
        else { channel.prover_message(&[7u8]).unwrap(); }
        let value = channel.field_challenge_at::<K>(site(2)).unwrap();
        assert!(!values.contains(&value));
        values.push(value);
        channel.finish().unwrap();
    }
}

#[test]
fn canonical_atom_decoding_is_transactional() {
    let malformed = [255u8; 16];
    let mut cursor = malformed.as_slice();
    assert!(FieldAtom::<K>::deserialize_from_narg(&mut cursor).is_err());
    assert_eq!(cursor, malformed);
    let atom = U128Atom::new(17);
    let encoded = atom.encode();
    let mut cursor = &encoded.as_ref()[..15];
    let before = cursor;
    assert!(U128Atom::deserialize_from_narg(&mut cursor).is_err());
    assert_eq!(cursor, before);
}

#[test]
fn inherited_endpoint_mismatch_rejects_and_full_history_is_length_framed() {
    let augmentation = InheritedAugmentation { literal_history: b"ab", endpoint_binding: b"c" };
    assert!(augmentation.bind(b"native", b"d").is_err());
    let encoded = augmentation.bind(b"native", b"c").unwrap();
    let mut literal = Vec::new();
    frame(&mut literal, b"akita-pcs/inherited-augmentation/v8");
    frame(&mut literal, b"native");
    frame(&mut literal, b"ab");
    frame(&mut literal, b"c");
    assert_eq!(encoded, literal);
    let other = InheritedAugmentation { literal_history: b"a", endpoint_binding: b"bc" };
    assert_ne!(encoded, other.bind(b"native", b"bc").unwrap());
    let changed = InheritedAugmentation { literal_history: b"bb", endpoint_binding: b"c" };
    assert_ne!(encoded, changed.bind(b"native", b"c").unwrap());
}

#[test]
fn atom_decoder_cannot_consume_next_installed_event() {
    let first = ProtocolContextRecord::new(site(1).to_bytes(), 3, 1, 1, 0);
    let second = ProtocolContextRecord::new(site(2).to_bytes(), 3, 1, 15, 0);
    let ctx = context(vec![InstalledEvent::Message(first), InstalledEvent::Message(second)]);
    let proof = [0; 16];
    let mut verifier = new_verifier_channel(&ctx, &proof).unwrap();
    verifier_context(&mut verifier, first).unwrap();
    assert!(verifier.prover_message::<U128Atom>().is_err());
    assert!(verifier.prover_message::<[u8; 1]>().is_err());
    assert!(verifier.check_eof().is_err());
}

#[test]
fn generic_f32_e4_vector_owns_all_independent_coordinates() {
    use crate::{exchange_extension_group, ext_challenge, extension_group_record, field_request_spec};
    use jolt_field::{ExtField, FpExt4, Ring};
    type F = Prime32Offset99;
    type E = FpExt4<F>;
    let record = extension_group_record::<F, E>(site(1), ProtocolMessageKind::ProofAtoms, 1).unwrap();
    let first = field_request_spec::<F, E>(0, site(2)).unwrap();
    let second = field_request_spec::<F, E>(1, site(3)).unwrap();
    assert_eq!(first.request.operation(), IndexedOperation::FieldVector);
    assert_eq!(first.request.raw_len(), 8192);
    assert_eq!(first.result_len, 16);
    let ctx = context(vec![InstalledEvent::Message(record), InstalledEvent::Request(first.clone()), InstalledEvent::Request(second.clone())]);
    let mut prover = new_prover_channel(&ctx).unwrap();
    let mut claim = [E::from_base_slice(&[F::from_u64(3), F::from_u64(5), F::from_u64(8), F::from_u64(13)])];
    exchange_extension_group::<F, E, _>(&mut prover, site(1), &mut claim).unwrap();
    let a = ext_challenge::<F, E, _>(&mut prover, site(2)).unwrap();
    let b = ext_challenge::<F, E, _>(&mut prover, site(3)).unwrap();
    let proof = prover.finish().unwrap();
    let mut literal = header(19, b"session", b"complete augmented instance and endpoint roster");
    message(&mut literal, 0, record, &proof);
    let reference = |raw: &[u8]| {
        let mut result = Vec::new();
        for allocation in raw.chunks_exact(2048) {
            let accepted = allocation.chunks_exact(16).find_map(|slot| {
                let bytes: [u8; 4] = slot[..4].try_into().unwrap();
                (u32::from_le_bytes(bytes) < u32::MAX - 98).then_some(bytes)
            }).unwrap();
            result.extend_from_slice(&accepted);
        }
        result
    };
    let encoded = |value: E| value.to_base_vec().iter().flat_map(|c| c.to_bytes_le_vec()).collect::<Vec<_>>();
    let expected_a = reference(&literal_answer(&literal, &first.request));
    assert_eq!(encoded(a), expected_a);
    completion(&mut literal, 1, &first, &expected_a);
    assert_eq!(encoded(b), reference(&literal_answer(&literal, &second.request)), "descendant owns all four coordinates");
    let mut verifier = new_verifier_channel(&ctx, &proof).unwrap();
    let mut received = [E::default()];
    exchange_extension_group::<F, E, _>(&mut verifier, site(1), &mut received).unwrap();
    assert_eq!(received, claim);
    assert_eq!(ext_challenge::<F, E, _>(&mut verifier, site(2)).unwrap(), a);
    assert_eq!(ext_challenge::<F, E, _>(&mut verifier, site(3)).unwrap(), b);
    verifier.check_eof().unwrap();
    let mut wrong_scalar = new_prover_channel(&ctx).unwrap();
    exchange_extension_group::<F, E, _>(&mut wrong_scalar, site(1), &mut claim).unwrap();
    assert!(wrong_scalar.field_challenge_at::<F>(site(2)).is_err(), "vector is not independent scalar requests");
    assert!(wrong_scalar.finish().is_err());
}

#[test]
fn generic_prime_families_preserve_supported_field_codecs_without_aqra_admission() {
    use crate::{field_request_spec, prime_descriptor};
    use jolt_field::{ExtField, Field, Prime48Offset59, Prime64Offset59, Prime128Offset275};
    fn check<F: Field + CanonicalEncoding + ExtField<F>>(q: u128) {
        let descriptor = prime_descriptor::<F>().unwrap();
        assert_eq!(&descriptor[20..], &q.to_le_bytes());
        let spec = field_request_spec::<F, F>(0, site(1)).unwrap();
        assert_eq!(spec.request.raw_len(), 2048);
        assert_eq!(spec.result_len, F::NUM_BYTES as u64);
        let ctx = context(vec![InstalledEvent::Request(spec.clone())]);
        let mut prover = new_prover_channel(&ctx).unwrap();
        let value = prover.field_challenge_at::<F>(site(1)).unwrap();
        prover.finish().unwrap();
        let raw = literal_answer(&header(19, b"session", b"complete augmented instance and endpoint roster"), &spec.request);
        let mask = if F::MODULUS_BITS == 128 { u128::MAX } else { (1u128 << F::MODULUS_BITS) - 1 };
        let expected = raw.chunks_exact(16).find_map(|slot| {
            let x = u128::from_le_bytes(slot.try_into().unwrap()) & mask;
            (x < q).then_some(x)
        }).unwrap();
        assert_eq!(value.to_u128_checked(), Some(expected));
        let mut verifier = new_verifier_channel(&ctx, &[]).unwrap();
        assert_eq!(verifier.field_challenge_at::<F>(site(1)).unwrap(), value);
        verifier.check_eof().unwrap();
    }
    check::<Prime32Offset99>((1u128 << 32) - 99);
    check::<Prime48Offset59>((1u128 << 48) - 59);
    check::<Prime64Offset59>((1u128 << 64) - 59);
    check::<Prime128Offset275>(u128::MAX - 274);
    check::<K>(u128::MAX - (1u128 << 32) + 22538);
}

#[test]
fn malformed_generic_degree_owner_and_width_are_not_accepted_registries() {
    use crate::field_request_spec;
    use jolt_field::FpExt4;
    let spec = field_request_spec::<Prime32Offset99, FpExt4<Prime32Offset99>>(0, site(1)).unwrap();
    for offset in [4, 44, 52, 56, 56 + 24] {
        let mut descriptor = spec.request.descriptor().to_vec();
        descriptor[offset] ^= 1;
        let request = IndexedRequest::new(IndexedOperation::FieldVector, 0, site(1), descriptor, 8192).unwrap();
        let altered = RequestSpec { request, result_kind: CompletedResultKind::FieldVector, result_len: 16 };
        assert!(InstalledRegistry::new(19, vec![InstalledEvent::Request(altered)], limits()).is_err());
    }
}

#[test]
fn public_only_events_use_prefix_budget_not_proof_budget() {
    let bytes = [7u8; 128];
    let record = ProtocolContextRecord::new(site(1).to_bytes(), 1, 128, 128, 0);
    let mut resources = limits();
    resources.max_proof_bytes = 0;
    let request = scalar(0, site(2));
    let registry = InstalledRegistry::new(19, vec![InstalledEvent::Message(record), InstalledEvent::Request(request)], resources).unwrap();
    let ctx = InstalledContext::new(&registry, b"session", b"instance").unwrap();
    let mut prover = new_prover_channel(&ctx).unwrap();
    public_bytes(&mut prover, site(1), &bytes).unwrap();
    let challenge = prover.field_challenge_at::<K>(site(2)).unwrap();
    assert!(prover.finish().unwrap().is_empty());
    let mut verifier = new_verifier_channel(&ctx, &[]).unwrap();
    public_bytes(&mut verifier, site(1), &bytes).unwrap();
    assert_eq!(verifier.field_challenge_at::<K>(site(2)).unwrap(), challenge);
    verifier.check_eof().unwrap();
    assert!(new_verifier_channel(&ctx, &[0]).is_err());
}

#[test]
fn zero_length_groups_have_owners_and_unlabelled_empty_atoms_do_not() {
    let mut unowned = new_prover_channel(&context(vec![])).unwrap();
    assert!(unowned.public_message(&[] as &[u8]).is_err());
    assert!(unowned.finish().is_err());
    let record = ProtocolContextRecord::new(site(1).to_bytes(), 3, 0, 0, 0);
    let ctx = context(vec![InstalledEvent::Message(record)]);
    let mut prover = new_prover_channel(&ctx).unwrap();
    send_byte_group(&mut prover, site(1), &[]).unwrap();
    prover.finish().unwrap();
    let mut verifier = new_verifier_channel(&ctx, &[]).unwrap();
    assert_eq!(crate::receive_byte_group(&mut verifier, site(1), 0).unwrap(), []);
    verifier.check_eof().unwrap();
}

#[test]
fn ignored_bounded_length_error_is_sticky_before_payload_allocation() {
    let (length, payload) = crate::bounded_bytes_sites(site(1));
    let ctx = context(vec![InstalledEvent::Message(length), InstalledEvent::BoundedBytes { site: payload, max_len: 8 }]);
    let proof = 9u32.encode();
    let mut verifier = new_verifier_channel(&ctx, proof.as_ref()).unwrap();
    assert!(crate::receive_bounded_bytes(&mut verifier, site(1), 8).is_err());
    assert!(verifier.prover_message::<[u8; 1]>().is_err());
    assert!(verifier.check_eof().is_err());
}

#[test]
fn fold_nonce_range_and_noncanonical_encodings_are_fatal() {
    let nonce_site = ProtocolSiteId { detail: 12, ..site(1) };
    let record = ProtocolContextRecord::new(nonce_site.to_bytes(), 7, 1, 2, 0);
    let ctx = context(vec![InstalledEvent::Message(record)]);
    for proof in [&[0x80, 0x20][..], &[0x80, 0x00][..], &[0x80][..]] {
        let mut verifier = new_verifier_channel(&ctx, proof).unwrap();
        verifier_context(&mut verifier, record).unwrap();
        assert!(verifier.prover_message::<NonceAtom>().is_err());
        assert!(verifier.check_eof().is_err());
    }
}

#[test]
fn extension_codec_rejects_truncation_and_noncanonical_limbs_transactionally() {
    use crate::ExtensionAtom;
    use jolt_field::{FpExt4, Ring};
    type E = FpExt4<Prime32Offset99>;
    let encoded = ExtensionAtom::<Prime32Offset99, E>::new(E::from_u64(9)).encode().as_ref().to_vec();
    let mut cursor = &encoded[..encoded.len() - 1];
    let before = cursor;
    assert!(ExtensionAtom::<Prime32Offset99, E>::deserialize_from_narg(&mut cursor).is_err());
    assert_eq!(cursor, before);
    let mut malformed = encoded;
    malformed[4..8].fill(255);
    let mut cursor = malformed.as_slice();
    let before = cursor;
    assert!(ExtensionAtom::<Prime32Offset99, E>::deserialize_from_narg(&mut cursor).is_err());
    assert_eq!(cursor, before);
}

#[test]
fn registered_generic_modulus_cannot_be_decoded_as_a_different_prime() {
    use crate::field_request_spec;
    let spec = field_request_spec::<Prime32Offset99, Prime32Offset99>(0, site(1)).unwrap();
    let mut descriptor = spec.request.descriptor().to_vec();
    descriptor[20] ^= 2;
    let request = IndexedRequest::new(IndexedOperation::PrimeScalar, 0, site(1), descriptor, 2048).unwrap();
    let ctx = context(vec![InstalledEvent::Request(RequestSpec { request, result_kind: CompletedResultKind::PrimeScalar, result_len: 4 })]);
    let mut channel = new_prover_channel(&ctx).unwrap();
    assert!(channel.field_challenge_at::<Prime32Offset99>(site(1)).is_err());
    assert!(channel.finish().is_err());
}
