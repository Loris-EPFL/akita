//! Fixed-workload inputs and a bounded outer receipt codec. Query grammar,
//! arithmetic, completed results and literal history belong to the production
//! `akita_pcs::accumulation::protocol` owner, not a second benchmark protocol.

use akita_error::AkitaError;
use jolt_field::CanonicalEncoding;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::io;

pub(crate) type Config = akita_config::proof_optimized::fp128::Dense;
pub(crate) type F = akita_config::proof_optimized::fp128::Field;
pub(crate) const NUM_VARS: usize = 14;
pub(crate) const SESSION: &[u8] = b"akita/aqra/dense-originals/indexed-v8";
const MAGIC: &[u8] = b"AQRA-DENSE-v8";

#[derive(Clone)]
pub(crate) struct Receipt {
    pub seal: Vec<u8>,
    pub query: Vec<u8>,
    pub query_history: Vec<u8>,
    pub native: Vec<u8>,
}
pub(crate) struct BorrowedReceipt<'a> {
    pub seal: &'a [u8],
    pub query: &'a [u8],
    pub query_history: &'a [u8],
    pub native: &'a [u8],
}
impl Receipt {
    pub(crate) fn encode(&self) -> Result<Vec<u8>, AkitaError> {
        let mut bytes = MAGIC.to_vec();
        frame(&mut bytes, &self.seal)?;
        frame(&mut bytes, &self.query)?;
        frame(&mut bytes, &self.query_history)?;
        frame(&mut bytes, &self.native)?;
        Ok(bytes)
    }
    /// No attacker-controlled allocations. Every frame is bounded by an
    /// independently installed public shape, and acceptance requires EOF.
    pub(crate) fn decode(
        bytes: &[u8],
        seal_len: usize,
        query_cap: usize,
        history_cap: usize,
        native_cap: usize,
    ) -> Result<BorrowedReceipt<'_>, AkitaError> {
        let mut input = bytes;
        if take(&mut input, MAGIC.len())? != MAGIC {
            return Err(AkitaError::InvalidProof);
        }
        let seal = read_frame(&mut input, seal_len, Some(seal_len))?;
        let query = read_frame(&mut input, query_cap, None)?;
        let query_history = read_frame(&mut input, history_cap, None)?;
        let native = read_frame(&mut input, native_cap, None)?;
        if !input.is_empty() {
            return Err(AkitaError::InvalidProof);
        }
        Ok(BorrowedReceipt {
            seal,
            query,
            query_history,
            native,
        })
    }
}
pub(crate) fn frame(bytes: &mut Vec<u8>, value: &[u8]) -> Result<(), AkitaError> {
    bytes.extend_from_slice(
        &u64::try_from(value.len())
            .map_err(|_| AkitaError::InvalidProof)?
            .to_le_bytes(),
    );
    bytes.extend_from_slice(value);
    Ok(())
}
fn take<'a>(input: &mut &'a [u8], len: usize) -> Result<&'a [u8], AkitaError> {
    let (value, rest) = input
        .split_at_checked(len)
        .ok_or(AkitaError::InvalidProof)?;
    *input = rest;
    Ok(value)
}
fn read_frame<'a>(
    input: &mut &'a [u8],
    cap: usize,
    exact: Option<usize>,
) -> Result<&'a [u8], AkitaError> {
    let len = u64::from_le_bytes(
        take(input, 8)?
            .try_into()
            .map_err(|_| AkitaError::InvalidProof)?,
    );
    let len = usize::try_from(len).map_err(|_| AkitaError::InvalidProof)?;
    if len > cap || exact.is_some_and(|n| len != n) {
        return Err(AkitaError::InvalidProof);
    }
    take(input, len)
}
pub(crate) fn make_dense_evals(segment: usize) -> Vec<F> {
    let seed = 0xdead_beefu64.wrapping_add((segment as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15));
    let mut rng = StdRng::seed_from_u64(seed);
    (0..1usize << NUM_VARS)
        .map(|_| F::from_u128_reduced(rng.gen::<u128>()))
        .collect()
}
pub(crate) fn opening_point() -> Vec<F> {
    let mut rng = StdRng::seed_from_u64(0xcafe_babe);
    (0..NUM_VARS)
        .map(|_| F::from_u128_reduced(rng.gen::<u128>()))
        .collect()
}
pub(crate) fn parse_segments() -> Result<Option<usize>, Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let first = args.next().ok_or_else(|| {
        io::Error::other("usage: accumulation_relation_benchmark --segments <positive-usize>")
    })?;
    if matches!(first.as_str(), "--help" | "-h") && args.len() == 0 {
        println!("Usage: accumulation_relation_benchmark [--segments] <positive-usize>\nFixed Dense K/K nv14 workload; one full final proof, independent replay, mandatory tamper checks.");
        return Ok(None);
    }
    let count = if first == "--segments" {
        args.next()
            .ok_or_else(|| io::Error::other("missing segment count"))?
    } else {
        first
    };
    if args.next().is_some() {
        return Err(io::Error::other("unexpected extra argument").into());
    }
    let count = count.parse::<usize>()?;
    if count == 0
        || count.checked_mul(1usize << NUM_VARS).is_none()
        || u64::try_from(count).is_err()
    {
        return Err(io::Error::other(
            "segment horizon is zero or overflows retained source accounting",
        )
        .into());
    }
    Ok(Some(count))
}
#[cfg(target_os = "linux")]
pub(crate) fn peak_rss_bytes() -> Result<u64, Box<dyn std::error::Error>> {
    let status = std::fs::read_to_string("/proc/self/status")?;
    let line = status
        .lines()
        .find_map(|line| line.strip_prefix("VmHWM:"))
        .ok_or_else(|| io::Error::other("missing VmHWM"))?;
    let mut parts = line.split_whitespace();
    let kib = parts
        .next()
        .ok_or_else(|| io::Error::other("missing VmHWM value"))?
        .parse::<u64>()?;
    if parts.next() != Some("kB") || parts.next().is_some() {
        return Err(io::Error::other("invalid VmHWM format").into());
    }
    Ok(kib
        .checked_mul(1024)
        .ok_or_else(|| io::Error::other("VmHWM overflow"))?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn receipt_codec_rejects_unbounded_lengths_truncation_and_trailing() {
        let receipt = Receipt {
            seal: vec![1, 2],
            query: vec![3],
            query_history: vec![4],
            native: vec![5, 6],
        };
        let encoded = receipt.encode().unwrap();
        let decoded = Receipt::decode(&encoded, 2, 1, 1, 2).unwrap();
        assert_eq!(decoded.seal, receipt.seal);
        assert_eq!(decoded.query, receipt.query);
        assert_eq!(decoded.query_history, receipt.query_history);
        assert_eq!(decoded.native, receipt.native);
        for len in 0..encoded.len() {
            assert!(Receipt::decode(&encoded[..len], 2, 1, 1, 2).is_err());
        }
        let mut trailing = encoded.clone();
        trailing.push(0);
        assert!(Receipt::decode(&trailing, 2, 1, 1, 2).is_err());
        let mut malicious = encoded.clone();
        malicious[MAGIC.len()..MAGIC.len() + 8].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(Receipt::decode(&malicious, 2, 1, 1, 2).is_err());
        assert!(Receipt::decode(&encoded, 2, 0, 1, 2).is_err());
        assert!(Receipt::decode(&encoded, 2, 1, 0, 2).is_err());
        assert!(Receipt::decode(&encoded, 2, 1, 1, 1).is_err());
    }
}
