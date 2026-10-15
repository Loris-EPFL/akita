use super::*;
use crate::{FoldChallengeDrawDomain, OperatorNormRejection, SparseChallengeConfig, SHELL_FILTER_CAP};

fn group(d: usize, index: usize, blocks: usize, claims: usize, n1: usize, n2: usize) -> ShellGroupDescriptor {
    ShellGroupDescriptor::new(FoldChallengeDrawDomain::EvaluationTrace, d, index, blocks, claims,
        SparseChallengeConfig { count_pm1: n1, count_pm2: n2 }, None).unwrap()
}

/// Deterministic full candidate fixtures, not a probabilistic inverse kernel.
fn candidate_for_order(group: &ShellGroupDescriptor, order: &[u32], coeffs: &[i8]) -> Vec<u8> {
    assert_eq!(order.len(), group.config().weight());
    assert_eq!(order.len(), coeffs.len());
    let mut permutation: Vec<u32> = (0..group.ring_dimension() as u32).collect();
    let mut bytes = Vec::new();
    for (i, &position) in order.iter().enumerate() {
        let j = permutation.iter().enumerate().skip(i).find(|(_, p)| **p == position).unwrap().0;
        let width = range_width(group.ring_dimension() - i).unwrap();
        if width != 0 {
            bytes.extend_from_slice(&((j - i) as u32).to_le_bytes()[..width]);
            bytes.resize(bytes.len() + (SHELL_MASK_CAP - 1) * width, 0xff);
        }
        permutation.swap(i, j);
    }
    for &coefficient in coeffs { bytes.push(0xfc | u8::from(coefficient < 0)); }
    assert_eq!(bytes.len(), group.coordinate_inputs().candidate_bytes);
    bytes
}

struct GuardedAnswer {
    prefix: Vec<u8>,
    declared: u64,
    position: usize,
}
impl ShellAnswer for GuardedAnswer {
    fn declared_len(&self) -> u64 { self.declared }
    fn read_exact(&mut self, out: &mut [u8]) -> Result<(), AkitaError> {
        let end = self.position.checked_add(out.len()).ok_or(AkitaError::InvalidProof)?;
        let bytes = self.prefix.get(self.position..end).ok_or(AkitaError::InvalidProof)?;
        out.copy_from_slice(bytes);
        self.position = end;
        Ok(())
    }
    fn skip_exact(&mut self, count: u64) -> Result<(), AkitaError> {
        let end = self.position.checked_add(count as usize).ok_or(AkitaError::InvalidProof)?;
        self.prefix.get(self.position..end).ok_or(AkitaError::InvalidProof)?;
        self.position = end;
        Ok(())
    }
}

#[test]
fn range_width_and_first_accepted_word_use_exact_fixed_slot() {
    for (m, width) in [(1, 0), (2, 1), (256, 1), (257, 2), (65536, 2), (65537, 4)] {
        assert_eq!(range_width(m).unwrap(), width);
    }
    assert!(range_width(0).is_err());
    let mut bytes = vec![0xff; SHELL_MASK_CAP]; // low two bits 3 reject m=3.
    bytes[SHELL_MASK_CAP - 1] = 0xfe; // low two bits 2: last permitted acceptance.
    bytes.push(0x7f); // low sign bit 1, all other bits slack.
    let descriptor = ShellLevelDescriptor::new(vec![group(3, 0, 1, 1, 1, 0)]).unwrap();
    let level = decode_shell_level_from_bytes(&descriptor, &bytes).unwrap();
    assert_eq!(level.group(0).unwrap().challenges().as_slice()[0].positions.as_slice(), &[2]);
    assert_eq!(level.group(0).unwrap().challenges().as_slice()[0].coeffs.as_slice(), &[-1]);
    bytes[SHELL_MASK_CAP - 1] = 0xff;
    let error = decode_shell_level_from_bytes(&descriptor, &bytes).unwrap_err();
    assert!(matches!(error, AkitaError::InvalidInput(s) if s.contains("fatal shell mask")));
}

#[test]
fn deterministic_range_consumes_no_position_bytes_and_sign_slack_is_ignored() {
    let descriptor = ShellLevelDescriptor::new(vec![group(1, 0, 1, 1, 0, 1)]).unwrap();
    assert_eq!(descriptor.raw_len(), 1);
    for slack in 0u8..128 {
        let positive = decode_shell_level_from_bytes(&descriptor, &[slack << 1]).unwrap();
        let negative = decode_shell_level_from_bytes(&descriptor, &[(slack << 1) | 1]).unwrap();
        assert_eq!(positive.group(0).unwrap().challenges().as_slice()[0].coeffs.as_slice(), &[2]);
        assert_eq!(negative.group(0).unwrap().challenges().as_slice()[0].coeffs.as_slice(), &[-2]);
    }
}

#[test]
fn polynomial_codec_and_arithmetic_view_ignore_shuffle_permutations_and_slack() {
    let g = group(8, 0, 1, 1, 2, 2);
    let first = candidate_for_order(&g, &[5, 1, 7, 3], &[1, -1, -2, 2]);
    let second = candidate_for_order(&g, &[1, 5, 3, 7], &[-1, 1, 2, -2]);
    let descriptor = ShellLevelDescriptor::new(vec![g]).unwrap();
    let level_a = decode_shell_level_from_bytes(&descriptor, &first).unwrap();
    let level_b = decode_shell_level_from_bytes(&descriptor, &second).unwrap();
    assert_eq!(level_a, level_b);
    let ch = &level_a.group(0).unwrap().challenges().as_slice()[0];
    assert_eq!(ch.positions.as_slice(), &[1, 5, 3, 7]);
    assert_eq!(ch.coeffs.as_slice(), &[-1, 1, 2, -2]);
    let mut result_a = Vec::new();
    let mut result_b = Vec::new();
    write_canonical_result(&level_a, |bytes| { result_a.extend_from_slice(bytes); Ok(()) }).unwrap();
    write_canonical_result(&level_b, |bytes| { result_b.extend_from_slice(bytes); Ok(()) }).unwrap();
    assert_eq!(result_a, vec![0, 255, 0, 2, 0, 1, 0, 254]);
    assert_eq!(result_a, result_b);
    assert_eq!(result_a.len() as u64, descriptor.result_len());
}

#[test]
fn every_tiny_signed_polynomial_has_a_distinct_canonical_result() {
    let descriptor = ShellLevelDescriptor::new(vec![group(3, 0, 1, 1, 1, 0)]).unwrap();
    let mut results = std::collections::BTreeSet::new();
    for position in 0..3 {
        for sign in [1, -1] {
            let candidate = candidate_for_order(&descriptor.groups()[0], &[position], &[sign]);
            let level = decode_shell_level_from_bytes(&descriptor, &candidate).unwrap();
            let mut result = Vec::new();
            write_canonical_result(&level, |bytes| { result.extend_from_slice(bytes); Ok(()) }).unwrap();
            assert!(results.insert(result));
        }
    }
    assert_eq!(results.len(), 6); // binom(3,1) * binom(1,1) * 2.
}

#[test]
fn group_claim_block_order_and_full_lengths_are_frozen_and_checked() {
    let first = group(4, 0, 2, 2, 1, 1);
    let second = group(1, 1, 1, 1, 1, 0);
    let descriptor = ShellLevelDescriptor::new(vec![first.clone(), second]).unwrap();
    assert_eq!(descriptor.raw_len(), 4 * (2 * SHELL_MASK_CAP + 2) as u64 + 1);
    assert_eq!(descriptor.result_len(), 17);
    assert_eq!(descriptor.coordinate_count(), 5);
    assert_eq!(descriptor.encoded_bytes().len(), b"akita/shell-level/v8".len() + 36 + 2 * 118 + 5 * 24);
    let mut raw = Vec::new();
    for (position, sign) in [(0, 1), (1, -1), (2, 1), (3, -1)] {
        raw.extend(candidate_for_order(&first, &[position, (position + 1) % 4], &[sign, 2]));
    }
    raw.push(0);
    let level = decode_shell_level_from_bytes(&descriptor, &raw).unwrap();
    assert_eq!(level.group(0).unwrap().challenges().num_claims(), 2);
    assert_eq!(level.group(0).unwrap().challenges().num_live_blocks_per_claim(), 2);
    assert!(level.group(2).is_err());
    assert!(decode_shell_level_from_bytes(&descriptor, &raw[..raw.len() - 1]).is_err());
    raw.push(0);
    assert!(decode_shell_level_from_bytes(&descriptor, &raw).is_err());
    assert!(ShellLevelDescriptor::new(vec![group(4, 1, 1, 1, 1, 0)]).is_err());
    assert!(ShellLevelDescriptor::new(vec![]).is_err());
    assert!(ShellGroupDescriptor::new(FoldChallengeDrawDomain::EvaluationTrace, 4, 0,
        usize::MAX, 2, SparseChallengeConfig::pm1_only(1), None).is_err());
    assert!(ShellGroupDescriptor::new(FoldChallengeDrawDomain::EvaluationTrace, 4, 0,
        1, 1, SparseChallengeConfig { count_pm1: usize::MAX, count_pm2: 1 }, None).is_err());
}

fn filtered(index: usize) -> ShellGroupDescriptor {
    ShellGroupDescriptor::new(FoldChallengeDrawDomain::EvaluationTrace, 128, index, 1, 1,
        crate::D128_SELECTIVE_L2_CHALLENGE_CONFIG, Some(OperatorNormRejection::D128_SELECTIVE_L2)).unwrap()
}

fn accepted_candidate(g: &ShellGroupDescriptor) -> (Vec<u8>, SparseChallenge) {
    let mut state = 0x123456789abcdef0u64;
    for _ in 0..256 {
        let mut permutation: Vec<u32> = (0..g.ring_dimension() as u32).collect();
        let mut positions = Vec::new();
        let mut coeffs = Vec::new();
        for i in 0..g.config().weight() {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            let j = i + (state as usize % (g.ring_dimension() - i));
            permutation.swap(i, j);
            positions.push(permutation[i]);
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            coeffs.push(if state >> 63 == 0 { 1 } else { -1 });
        }
        let challenge = SparseChallenge { positions: positions.clone().into(), coeffs: coeffs.clone().into() };
        if g.accepts(&challenge).unwrap() { return (candidate_for_order(g, &positions, &coeffs), challenge); }
    }
    panic!("bounded test fixture search found no accepted registered polynomial");
}

#[test]
fn real_filter_is_per_coordinate_and_completed_slots_are_skipped_attempt_major() {
    let first = filtered(0);
    let second = filtered(1);
    let third = group(1, 2, 1, 1, 1, 0); // Its J=1 slot is omitted at attempt 1.
    let (accepted_first, _) = accepted_candidate(&first);
    let (accepted_second, _) = accepted_candidate(&second);
    let rejected = candidate_for_order(&second, &(0..31).collect::<Vec<_>>(), &[1; 31]);
    let rejected_polynomial = SparseChallenge { positions: (0..31).collect(), coeffs: vec![1; 31].into() };
    assert!(!second.accepts(&rejected_polynomial).unwrap());
    let mut prefix = accepted_first.clone();
    prefix.extend(rejected);
    prefix.push(0);
    // This reserved completed-coordinate candidate would fatally exhaust masks
    // if interpreted; it must instead be skipped without replacing coordinate 0.
    prefix.extend(vec![0xff; first.coordinate_inputs().candidate_bytes]);
    prefix.extend(accepted_second);
    let descriptor = ShellLevelDescriptor::new(vec![first, second, third]).unwrap();
    let mut answer = GuardedAnswer { prefix, declared: descriptor.raw_len(), position: 0 };
    let level = decode(&descriptor, &mut answer).unwrap();
    assert_eq!(answer.position, answer.prefix.len());
    assert!((answer.position as u64) < descriptor.raw_len()); // No unused 4094-attempt tail read.
    let first_only = ShellLevelDescriptor::new(vec![descriptor.groups()[0].clone()]).unwrap();
    let mut first_answer = GuardedAnswer { prefix: accepted_first, declared: first_only.raw_len(), position: 0 };
    assert_eq!(level.group(0).unwrap(), decode(&first_only, &mut first_answer).unwrap().group(0).unwrap());
}

struct ZeroAnswer { declared: u64, consumed: u64 }
impl ShellAnswer for ZeroAnswer {
    fn declared_len(&self) -> u64 { self.declared }
    fn read_exact(&mut self, out: &mut [u8]) -> Result<(), AkitaError> {
        self.skip_exact(out.len() as u64)?;
        out.fill(0);
        Ok(())
    }
    fn skip_exact(&mut self, count: u64) -> Result<(), AkitaError> {
        self.consumed = self.consumed.checked_add(count).ok_or(AkitaError::InvalidProof)?;
        if self.consumed > self.declared { return Err(AkitaError::InvalidProof); }
        Ok(())
    }
}

#[test]
fn registered_filter_exhausts_exactly_4096_candidates_without_whole_vector_retry() {
    let descriptor = ShellLevelDescriptor::new(vec![filtered(0)]).unwrap();
    assert_eq!(descriptor.groups()[0].coordinate_inputs().candidate_cap, SHELL_FILTER_CAP);
    let mut answer = ZeroAnswer { declared: descriptor.raw_len(), consumed: 0 };
    let error = decode(&descriptor, &mut answer).unwrap_err();
    assert!(matches!(error, AkitaError::InvalidInput(s) if s.contains("integer-filter candidate cap")));
    assert_eq!(answer.consumed, descriptor.raw_len());
}

#[test]
fn fatal_mask_is_not_filter_rejection_or_permission_to_publish_a_group_prefix() {
    let descriptor = ShellLevelDescriptor::new(vec![group(1, 0, 1, 1, 1, 0), filtered(1)]).unwrap();
    let mut prefix = vec![0];
    // The first 128-range slot always accepts; the next 127-range slot rejects
    // all masked low-seven-bit words. No later candidate or sign is consulted.
    prefix.extend(vec![0; SHELL_MASK_CAP]);
    prefix.extend(vec![0xff; SHELL_MASK_CAP]);
    let mut answer = GuardedAnswer { prefix, declared: descriptor.raw_len(), position: 0 };
    let error = decode(&descriptor, &mut answer).unwrap_err();
    assert!(matches!(error, AkitaError::InvalidInput(s) if s.contains("fatal shell mask")));
    assert_eq!(answer.position, 1 + 2 * SHELL_MASK_CAP);
}

#[test]
fn method_carrier_embedding_and_exact_filter_are_not_interchangeable() {
    let config = SparseChallengeConfig::pm1_only(1);
    assert!(ShellGroupDescriptor::new(FoldChallengeDrawDomain::SubringCoefficientPacking {
        challenge_subring_dimension: 64 }, 128, 0, 1, 1, config, None).is_err());
    assert!(ShellGroupDescriptor::new(FoldChallengeDrawDomain::SubringCoefficientPacking {
        challenge_subring_dimension: 128 }, 128, 0, 1, 1,
        crate::D128_SELECTIVE_L2_CHALLENGE_CONFIG, Some(OperatorNormRejection::D128_SELECTIVE_L2)).is_err());
    assert!(ShellGroupDescriptor::new(FoldChallengeDrawDomain::EvaluationTrace, 128, 0, 1, 1,
        config, Some(OperatorNormRejection::D128_SELECTIVE_L2)).is_err());
    let trace = group(64, 0, 1, 1, 1, 0);
    assert!(trace.clone().with_native_embedding(128, 2).is_err());
    let packing = ShellGroupDescriptor::new(FoldChallengeDrawDomain::SubringCoefficientPacking {
        challenge_subring_dimension: 64 }, 64, 0, 1, 1, config, None).unwrap();
    assert!(packing.clone().with_native_embedding(128, 3).is_err());
    let embedded = packing.with_native_embedding(128, 2).unwrap();
    assert_ne!(ShellLevelDescriptor::new(vec![trace]).unwrap().encoded_bytes(),
        ShellLevelDescriptor::new(vec![embedded]).unwrap().encoded_bytes());
}
