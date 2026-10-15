//! Selected root record geometry shared by native prover and verifier.
use crate::full_ring::FullRingRawPoint;
use akita_error::AkitaError;
use akita_serialization::{AkitaSerialize, Compress};
use akita_transcript::native::{encode_e, E};
use akita_transcript::native_strong::StrongSites;

fn invalid(message: &str) -> AkitaError {
    AkitaError::InvalidInput(message.into())
}
/// Installed occurrences for one full native reduction. These are structural
/// registry keys, not runtime-selected seeds or fresh session identifiers.
pub struct FullRingReductionSites<R, D, S> {
    pub instance: R,
    pub raw_queries: R,
    pub opening_payload: R,
    pub shell: S,
    pub successor: R,
    pub strong: StrongSites<R, D>,
}

/// One already-normalized original raw view. Its original handle remains owned
/// by the backend; normalization never replaces that source.
pub struct FullRingRawClaim {
    pub point: FullRingRawPoint<E>,
    pub value: E,
}
fn push_size(out: &mut Vec<u8>, value: usize) -> Result<(), AkitaError> {
    out.extend_from_slice(
        &u64::try_from(value)
            .map_err(|_| invalid("descriptor size overflow"))?
            .to_le_bytes(),
    );
    Ok(())
}

/// Canonical raw-view and endpoint binding, shared with selected replay.
/// The permutation is the registered original injection, never honest sparsity.
pub fn full_ring_raw_claim_binding(
    claims: &[FullRingRawClaim],
    source_ids: &[crate::FullRingSourceId],
) -> Result<Vec<u8>, AkitaError> {
    if claims.len() != source_ids.len() {
        return Err(invalid("raw source-role count"));
    }
    let mut bytes = Vec::new();
    push_size(&mut bytes, claims.len())?;
    for (claim, source_id) in claims.iter().zip(source_ids) {
        bytes.extend_from_slice(&source_id.0.to_le_bytes());
        let view = claim.point.view();
        for value in [
            view.degree(),
            view.polynomials(),
            view.blocks(),
            view.positions(),
            view.digits(),
        ] {
            push_size(&mut bytes, value)?;
        }
        if let Some(permutation) = view.permutation() {
            bytes.push(1);
            push_size(&mut bytes, permutation.len())?;
            for &index in permutation {
                push_size(&mut bytes, index)?;
            }
        } else {
            bytes.push(0);
        }
        push_size(&mut bytes, claim.point.point().len())?;
        for value in claim.point.point() {
            bytes.extend_from_slice(&encode_e(value));
        }
        bytes.extend_from_slice(&encode_e(&claim.value));
    }
    Ok(bytes)
}

/// Bind the actual successor and the complete exact-row descriptor before
/// any strong randomness. Both lengths are part of the installed wire grammar.
pub fn full_ring_strong_binding(
    successor: &[u8],
    relation: &crate::full_ring::CoefficientRelationPlan<'_, akita_transcript::native::K, E>,
) -> Result<Vec<u8>, AkitaError> {
    let descriptor = relation.canonical_descriptor_bytes()?;
    let mut bytes = Vec::new();
    push_size(&mut bytes, successor.len())?;
    bytes.extend_from_slice(successor);
    push_size(&mut bytes, descriptor.len())?;
    bytes.extend_from_slice(&descriptor);
    Ok(bytes)
}

/// The final receiver's actual source binding and retained binary/zero domain.
pub fn full_ring_domain_binding(
    successor: &[u8],
    domain: &crate::full_ring::FinalReceiverDomainPlan,
) -> Result<Vec<u8>, AkitaError> {
    let descriptor = domain.canonical_descriptor_bytes()?;
    let mut bytes = Vec::new();
    push_size(&mut bytes, successor.len())?;
    bytes.extend_from_slice(successor);
    push_size(&mut bytes, descriptor.len())?;
    bytes.extend_from_slice(&descriptor);
    Ok(bytes)
}

/// Bind the authenticated installation, its actual setup law and the complete
/// schedule before the first root shell draw. This is the v2 binding grammar.
pub fn full_ring_instance_binding(
    setup: &crate::FullRingSetupDescriptor,
    schedule: &akita_params::FoldSchedule,
) -> Result<Vec<u8>, AkitaError> {
    let descriptor = schedule.canonical_descriptor_bytes();
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"akita/full-ring-instance/v2");
    bytes.extend_from_slice(&setup.installation_id());
    match setup.shared_prefix() {
        Some(prefix) => {
            bytes.push(1);
            prefix
                .serialize_with_mode(&mut bytes, Compress::No)
                .map_err(|error| invalid(&error.to_string()))?;
        }
        None if setup.family() == crate::FullRingSetupFamily::OuterDirect => bytes.push(0),
        None => return Err(invalid("registered source setup is not installed")),
    }
    push_size(&mut bytes, setup.original_descriptor().len())?;
    bytes.extend_from_slice(setup.original_descriptor());
    push_size(&mut bytes, descriptor.len())?;
    bytes.extend_from_slice(&descriptor);
    Ok(bytes)
}
