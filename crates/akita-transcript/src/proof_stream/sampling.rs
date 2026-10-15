//! Finite canonical field families. A7F7/E1 retains the selected Scalar128
//! operation. Generic prime/vector operations are separately tagged component
//! contracts, not an AQRA extension-field security certificate.
use super::{indexed::Core, CompletedResultKind, IndexedChannel, IndexedError, IndexedOperation,
    IndexedRequest, ProtocolSiteId, ProverChannel, RequestSpec, VerifierChannel};
use akita_error::AkitaError;
use jolt_field::{CanonicalEncoding, ExtField, Field, Prime128OffsetA7F7};
use std::{any::TypeId, sync::Arc};

/// Every coordinate's raw candidate slot is sixteen bytes, including high slack.
pub const FIELD_CHALLENGE_BYTES: u64 = 16;
/// Fixed cap per independent canonical prime coordinate.
pub const FIELD_CANDIDATE_CAP: u64 = 128;
/// Legacy logical-query accounting ceiling, not an adversarial ROM budget.
pub const FIELD_SAMPLING_QUERY_LIMIT: u64 = u32::MAX as u64;
/// Supported canonical geometry. Codec/prime identity and an exact installed
/// descriptor are additionally checked at draws; geometry is not a certificate.
pub const fn field_sampling_is_certified(canonical_bytes: usize, modulus_bits: u32, query_budget: u64) -> bool {
    canonical_bytes > 0 && canonical_bytes <= 16 && modulus_bits > 0
        && modulus_bits <= (canonical_bytes * 8) as u32 && query_budget <= FIELD_SAMPLING_QUERY_LIMIT
}
/// Full fixed16-byte/128-slot allocation of one base coordinate.
pub const fn field_challenge_bytes<F: CanonicalEncoding>() -> u64 { FIELD_CANDIDATE_CAP * FIELD_CHALLENGE_BYTES }

/// Canonical version-one generic prime descriptor: LE32(version,width,bits,cap,
/// raw_slot_width), followed by the exact LE128 modulus. This derives the prime
/// characteristic from canonical -1, not a type-name hash or modular sampler.
pub fn prime_descriptor<F: Field + CanonicalEncoding>() -> Result<[u8; 36], IndexedError> {
    if !field_sampling_is_certified(F::NUM_BYTES, F::MODULUS_BITS, FIELD_SAMPLING_QUERY_LIMIT) {
        return Err(IndexedError::Installation);
    }
    let minus_one = F::zero() - F::one();
    let q_minus_one = minus_one.to_u128_checked().ok_or(IndexedError::Installation)?;
    let q = q_minus_one.checked_add(1).ok_or(IndexedError::Installation)?;
    if q <= 1 || 128 - q.leading_zeros() != F::MODULUS_BITS
        || F::from_u128_checked(q_minus_one) != Some(minus_one)
        || F::from_u128_checked(q).is_some() { return Err(IndexedError::Installation); }
    let mut canonical = [0u8; 16];
    minus_one.to_bytes_le(&mut canonical[..F::NUM_BYTES]);
    if canonical != q_minus_one.to_le_bytes()
        || F::from_bytes_le_checked(&q.to_le_bytes()[..F::NUM_BYTES]).is_some() {
        return Err(IndexedError::Installation);
    }
    let mut bytes = [0u8; 36];
    for (out, value) in bytes[..20].chunks_exact_mut(4).zip([1, F::NUM_BYTES as u32, F::MODULUS_BITS, 128, 16]) {
        out.copy_from_slice(&value.to_le_bytes());
    }
    bytes[20..].copy_from_slice(&q.to_le_bytes());
    Ok(bytes)
}

/// Whole-vector descriptor with the exact prime, degree, canonical Jolt base
/// coordinate codec version, and every ordered limb owner (fixed32 bytes each).
pub fn field_vector_descriptor<F, E>(site: ProtocolSiteId) -> Result<Vec<u8>, IndexedError>
where F: Field + CanonicalEncoding, E: ExtField<F> {
    let prime = prime_descriptor::<F>()?;
    let degree = u64::try_from(E::DEGREE).map_err(|_| IndexedError::Budget)?;
    if degree < 2 || site.limb != 0 || u32::try_from(E::DEGREE).is_err() { return Err(IndexedError::Installation); }
    let size = E::DEGREE.checked_mul(32).and_then(|n| n.checked_add(56)).ok_or(IndexedError::Budget)?;
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(size).map_err(|_| IndexedError::Budget)?;
    bytes.extend_from_slice(&36u64.to_le_bytes());
    bytes.extend_from_slice(&prime);
    bytes.extend_from_slice(&degree.to_le_bytes());
    bytes.extend_from_slice(&1u32.to_le_bytes());
    for limb in 0..E::DEGREE {
        bytes.extend_from_slice(&ProtocolSiteId { limb: limb as u32, ..site }.to_bytes());
    }
    Ok(bytes)
}

/// Canonical shared planner for installation and component fixtures. Does no
/// oracle work and does not infer a schedule from an observed execution.
pub fn field_request_spec<F, E>(ordinal: u64, site: ProtocolSiteId) -> Result<RequestSpec, IndexedError>
where F: Field + CanonicalEncoding, E: ExtField<F> {
    let degree = u64::try_from(E::DEGREE).map_err(|_| IndexedError::Budget)?;
    if degree == 0 { return Err(IndexedError::Installation); }
    if degree == 1 { return scalar_request_spec::<F>(ordinal, site); }
    let result_len = (F::NUM_BYTES as u64).checked_mul(degree).ok_or(IndexedError::Budget)?;
    let raw_len = field_challenge_bytes::<F>().checked_mul(degree).ok_or(IndexedError::Budget)?;
    let operation = IndexedOperation::FieldVector;
    let kind = CompletedResultKind::FieldVector;
    let descriptor: Arc<[u8]> = field_vector_descriptor::<F, E>(site)?.into();
    Ok(RequestSpec { request: IndexedRequest::new(operation, ordinal, site, descriptor, raw_len)?, result_kind: kind, result_len })
}

fn scalar_request_spec<F: Field + CanonicalEncoding>(ordinal: u64, site: ProtocolSiteId) -> Result<RequestSpec, IndexedError> {
    let (operation, kind, descriptor): (_, _, Arc<[u8]>) = if TypeId::of::<F>() == TypeId::of::<Prime128OffsetA7F7>() {
        (IndexedOperation::Scalar128, CompletedResultKind::Scalar, super::scalar_descriptor().into())
    } else {
        (IndexedOperation::PrimeScalar, CompletedResultKind::PrimeScalar, prime_descriptor::<F>()?.into())
    };
    Ok(RequestSpec { request: IndexedRequest::new(operation, ordinal, site, descriptor, 2048)?,
        result_kind: kind, result_len: F::NUM_BYTES as u64 })
}

pub(super) fn sample_field<F: Field + CanonicalEncoding>(core: &mut Core, site: ProtocolSiteId) -> Result<F, IndexedError> {
    let operation = if TypeId::of::<F>() == TypeId::of::<Prime128OffsetA7F7>() { IndexedOperation::Scalar128 } else { IndexedOperation::PrimeScalar };
    let request = core.next_request(operation, site)?;
    let kind = if operation == IndexedOperation::Scalar128 {
        if request.descriptor() != super::scalar_descriptor() { return core.fail(IndexedError::Request); }
        CompletedResultKind::Scalar
    } else {
        let descriptor = match prime_descriptor::<F>() { Ok(descriptor) => descriptor, Err(error) => return core.fail(error) };
        if request.descriptor() != descriptor { return core.fail(IndexedError::Request); }
        CompletedResultKind::PrimeScalar
    };
    core.execute(&request, kind, F::NUM_BYTES as u64, |answer, completed| {
        let (value, _) = decode_scalar::<F>(|candidate| answer.read_exact(candidate))?;
        let mut encoded = [0u8; 16];
        value.to_bytes_le(&mut encoded[..F::NUM_BYTES]);
        completed.write_all(&encoded[..F::NUM_BYTES])?;
        Ok(value)
    })
}
pub(super) fn sample_vector<F, E>(core: &mut Core, site: ProtocolSiteId) -> Result<E, IndexedError>
where F: Field + CanonicalEncoding, E: ExtField<F> {
    if E::DEGREE == 1 { return sample_field::<F>(core, site).map(|value| E::from_base_slice(&[value])); }
    let request = core.next_request(IndexedOperation::FieldVector, site)?;
    let prime = match prime_descriptor::<F>() { Ok(prime) => prime, Err(error) => return core.fail(error) };
    // The immutable installed registry already validated every ordered owner.
    // At execution only the requested Rust codec/degree must be matched.
    let Some(degree_bytes) = request.descriptor().get(44..52).and_then(|bytes| <[u8; 8]>::try_from(bytes).ok()) else {
        return core.fail(IndexedError::Request);
    };
    let degree = u64::from_le_bytes(degree_bytes);
    let width = F::NUM_BYTES;
    if degree != E::DEGREE as u64 || request.descriptor().get(8..44) != Some(prime.as_slice()) {
        return core.fail(IndexedError::Request);
    }
    let Some(result_len) = degree.checked_mul(width as u64) else { return core.fail(IndexedError::Budget); };
    core.execute(&request, CompletedResultKind::FieldVector, result_len, |answer, completed| {
        let mut failure = None;
        let value = E::from_base_fn(|limb| {
            if failure.is_some() { return F::zero(); }
            let decoded = (|| {
                let (value, attempts) = decode_scalar::<F>(|candidate| answer.read_exact(candidate))?;
                let mut encoded = [0u8; 16];
                value.to_bytes_le(&mut encoded[..F::NUM_BYTES]);
                completed.write_all(&encoded[..F::NUM_BYTES])?;
                // Fixed-offset independent allocations: later coordinates do
                // not start at the earlier coordinate's stopping position.
                if limb + 1 < E::DEGREE {
                    answer.skip_exact((FIELD_CANDIDATE_CAP - attempts) * FIELD_CHALLENGE_BYTES)?;
                }
                Ok(value)
            })();
            match decoded {
                Ok(value) => value,
                Err(error) => { failure = Some(error); F::zero() },
            }
        });
        // No placeholder coefficient is ever returned or completed: any error
        // discards the locally constructed value and poisons this transaction.
        if let Some(error) = failure { return Err(error); }
        Ok(value)
    })
}

// Called only after the admitted width/bits/prime descriptor was checked.
fn decode_scalar<F: CanonicalEncoding>(mut read_exact: impl FnMut(&mut [u8]) -> Result<(), IndexedError>) -> Result<(F, u64), IndexedError> {
    for attempt in 1..=FIELD_CANDIDATE_CAP {
        let mut candidate = [0u8; 16];
        read_exact(&mut candidate)?;
        let width = F::NUM_BYTES;
        let full_bytes = F::MODULUS_BITS as usize / 8;
        let tail_bits = F::MODULUS_BITS as usize % 8;
        if tail_bits != 0 {
            candidate[full_bytes] &= u8::MAX >> (8 - tail_bits);
            candidate[full_bytes + 1..width].fill(0);
        } else { candidate[full_bytes..width].fill(0); }
        if let Some(value) = F::from_bytes_le_checked(&candidate[..width]) { return Ok((value, attempt)); }
    }
    Err(IndexedError::ScalarExhausted)
}
/// Labelled prover scalar request, with the exact installed field family.
pub fn prover_field_challenge<F: Field + CanonicalEncoding>(state: &mut ProverChannel, site: ProtocolSiteId) -> Result<F, AkitaError> { state.field_challenge_at(site) }
/// Labelled verifier scalar request.
pub fn verifier_field_challenge<F: Field + CanonicalEncoding>(state: &mut VerifierChannel<'_>, site: ProtocolSiteId) -> Result<F, AkitaError> { state.field_challenge_at(site) }
/// Labelled atomic vector draw; its whole decoded result is completed before return.
pub fn field_vector_challenge_at<F, E, S>(state: &mut S, site: ProtocolSiteId) -> Result<E, AkitaError>
where F: Field + CanonicalEncoding, E: ExtField<F>, S: IndexedChannel { state.field_vector_challenge_at::<F, E>(site) }

pub(super) fn validate_prime_descriptor(bytes: &[u8]) -> Option<usize> {
    let bytes: &[u8; 36] = bytes.try_into().ok()?;
    // Use explicit fixed-array conversions, so descriptor parsing is total.
    let version = u32::from_le_bytes(bytes[0..4].try_into().ok()?);
    let width = u32::from_le_bytes(bytes[4..8].try_into().ok()?) as usize;
    let bits = u32::from_le_bytes(bytes[8..12].try_into().ok()?);
    let cap = u32::from_le_bytes(bytes[12..16].try_into().ok()?);
    let slot = u32::from_le_bytes(bytes[16..20].try_into().ok()?);
    let q = u128::from_le_bytes(bytes[20..36].try_into().ok()?);
    (version == 1 && cap == 128 && slot == 16 && q > 1 && 128 - q.leading_zeros() == bits
        && field_sampling_is_certified(width, bits, FIELD_SAMPLING_QUERY_LIMIT)).then_some(width)
}
pub(super) fn validate_vector_descriptor(bytes: &[u8], site: ProtocolSiteId) -> Option<(u64, usize)> {
    let head = bytes.get(..56)?;
    if u64::from_le_bytes(head[..8].try_into().ok()?) != 36 { return None; }
    let width = validate_prime_descriptor(&head[8..44])?;
    let degree = u64::from_le_bytes(head[44..52].try_into().ok()?);
    if degree < 2 || degree > u32::MAX as u64 || site.limb != 0 || head[52..56] != 1u32.to_le_bytes() { return None; }
    let expected = usize::try_from(degree).ok()?.checked_mul(32)?.checked_add(56)?;
    if bytes.len() != expected { return None; }
    for (limb, owner) in bytes[56..].chunks_exact(32).enumerate() {
        if owner != (ProtocolSiteId { limb: limb as u32, ..site }).to_bytes() { return None; }
    }
    Some((degree, width))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_rejected_is_finite_and_last_slot_can_succeed() {
        let q = u128::MAX - (1u128 << 32) + 22538;
        let mut reads = 0;
        let failure = decode_scalar::<Prime128OffsetA7F7>(|out| { reads += 1; out.copy_from_slice(&q.to_le_bytes()); Ok(()) });
        assert_eq!(failure, Err(IndexedError::ScalarExhausted));
        assert_eq!(reads, 128);
        reads = 0;
        let (value, attempts) = decode_scalar::<Prime128OffsetA7F7>(|out| {
            reads += 1;
            out.copy_from_slice(&if reads == 128 { q - 1 } else { q }.to_le_bytes());
            Ok(())
        }).unwrap();
        assert_eq!(value.to_u128_checked(), Some(q - 1));
        assert_eq!(attempts, 128);
        assert_eq!(reads, 128);
    }
    #[test]
    fn scalar_reader_failure_is_not_a_rejection_retry() {
        let mut reads = 0;
        let result = decode_scalar::<Prime128OffsetA7F7>(|_| { reads += 1; Err(IndexedError::AnswerBounds) });
        assert_eq!(result, Err(IndexedError::AnswerBounds));
        assert_eq!(reads, 1);
    }
    #[test]
    fn wide_raw_slack_is_not_canonical_field_entropy() {
        use jolt_field::{CanonicalBytes, Prime48Offset59};
        let descriptor = prime_descriptor::<Prime48Offset59>().unwrap();
        assert_eq!(validate_prime_descriptor(&descriptor), Some(Prime48Offset59::NUM_BYTES));
        let (value, attempts) = decode_scalar::<Prime48Offset59>(|out| {
            out.fill(255);
            out[..6].fill(0);
            Ok(())
        }).unwrap();
        assert_eq!(value.to_u128_checked(), Some(0));
        assert_eq!(attempts, 1);
    }
}
