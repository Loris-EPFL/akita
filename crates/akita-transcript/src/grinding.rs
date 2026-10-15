//! Akita's bounded proof-of-work policy shared by the indexed v8 channel.

use std::num::NonZeroU8;

/// Extra nonce bits used to make honest proof-of-work exhaustion negligible.
pub const GRINDING_NONCE_SLACK_BITS: u8 = 7;
/// Largest proof-of-work target supported by the current transition.
pub const MAX_GRINDING_BITS: u8 = 25;
/// Byte length of one proof-of-work predicate.
pub const GRINDING_PREDICATE_LEN: usize = crate::TRANSCRIPT_CHALLENGE_BLOCK_LEN;
/// Byte length bound into the grinding policy encoding.
pub const GRINDING_PREDICATE_BYTES: u8 = GRINDING_PREDICATE_LEN as u8;
/// Low-bit-first predicate bit order.
pub const GRINDING_LITTLE_ENDIAN_BIT_ORDER: u8 = 0;

/// Return whether the first `grind_bits` low-order predicate bits are zero.
#[must_use]
pub fn grinding_predicate_accepts(
    predicate: &[u8; GRINDING_PREDICATE_LEN],
    grind_bits: NonZeroU8,
) -> bool {
    const _: () = assert!((u8::MAX as usize / u8::BITS as usize) < GRINDING_PREDICATE_LEN);
    let grind_bits = usize::from(grind_bits.get());
    let whole_bytes = grind_bits / u8::BITS as usize;
    let remaining_bits = grind_bits % u8::BITS as usize;
    predicate[..whole_bytes].iter().all(|&byte| byte == 0)
        && (remaining_bits == 0 || predicate[whole_bytes] & ((1u8 << remaining_bits) - 1) == 0)
}

pub(crate) fn search_grinding_nonce_with(
    grind_bits: u8,
    nonce_bits: u8,
    mut predicate_for_candidate: impl FnMut(u32) -> Result<[u8; GRINDING_PREDICATE_LEN], akita_error::AkitaError>,
) -> Result<Option<(u32, [u8; GRINDING_PREDICATE_LEN])>, akita_error::AkitaError> {
    use akita_error::AkitaError;
    let Some(grind_bits_nonzero) = NonZeroU8::new(grind_bits) else {
        return if nonce_bits == 0 { Ok(Some((0, [0; GRINDING_PREDICATE_LEN]))) }
            else { Err(AkitaError::InvalidProof) };
    };
    if grind_bits > MAX_GRINDING_BITS || nonce_bits != grind_bits + GRINDING_NONCE_SLACK_BITS {
        return Err(AkitaError::InvalidProof);
    }
    // The checked policy gives nonce_bits<=32, so every candidate fits u32.
    for candidate in 0..(1u64 << nonce_bits) {
        let counter = candidate as u32;
        let predicate = predicate_for_candidate(counter)?;
        if grinding_predicate_accepts(&predicate, grind_bits_nonzero) {
            return Ok(Some((counter, predicate)));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predicate_checks_low_bits_at_byte_boundaries() {
        for grind_bits in [1, 8, 9, 16, 17, MAX_GRINDING_BITS] {
            let bits = NonZeroU8::new(grind_bits).unwrap();
            let mut predicate = [0u8; GRINDING_PREDICATE_LEN];
            assert!(grinding_predicate_accepts(&predicate, bits));
            let rejected_bit = usize::from(grind_bits - 1);
            predicate[rejected_bit / 8] = 1 << (rejected_bit % 8);
            assert!(!grinding_predicate_accepts(&predicate, bits));
        }
    }

    #[test]
    fn bounded_search_enforces_canonical_width_and_exhaustion() {
        assert_eq!(
            search_grinding_nonce_with(0, 0, |_| unreachable!()),
            Ok(Some((0, [0; GRINDING_PREDICATE_LEN])))
        );
        assert!(search_grinding_nonce_with(1, 7, |_| Ok([0; 32])).is_err());
        assert_eq!(
            search_grinding_nonce_with(1, 8, |_| Ok([u8::MAX; 32])),
            Ok(None)
        );
        assert_eq!(
            search_grinding_nonce_with(1, 8, |_| Ok([0; 32])),
            Ok(Some((0, [0; 32])))
        );
        let mut probes = 0;
        assert!(search_grinding_nonce_with(1, 8, |_| {
            probes += 1;
            Err(akita_error::AkitaError::InvalidProof)
        }).is_err());
        assert_eq!(probes, 1, "decoder failure cannot be treated as a losing nonce");
    }
}
