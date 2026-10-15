//! Canonical signed terminal responses for the full S128 scalar extension.
//!
//! Integers are checked before field reduction. The descriptor fixes the width;
//! neither redundant sign octets nor a proof-selected bound are accepted.
use akita_error::AkitaError;
mod channel;
pub use channel::{bind_terminal_and_draw, decode_terminal_partials, FullRingTerminalState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FullRingTerminalCodec {
    coordinates: usize,
    bound: u128,
    width: usize,
    payload_bytes: usize,
}

impl FullRingTerminalCodec {
    pub fn new(blocks: usize, coordinates: usize, modulus: u128) -> Result<Self, AkitaError> {
        let bound = (blocks as u128)
            .checked_mul(896)
            .ok_or(AkitaError::InvalidProof)?;
        if blocks == 0
            || coordinates == 0
            || modulus < 3
            || bound > (modulus - 1) / 2
            || bound > i128::MAX as u128
        {
            return Err(AkitaError::InvalidProof);
        }
        let bits = 128 - bound.leading_zeros() as usize;
        let width = (bits + 1).div_ceil(8);
        let payload_bytes = coordinates
            .checked_mul(width)
            .ok_or(AkitaError::InvalidProof)?;
        Ok(Self {
            coordinates,
            bound,
            width,
            payload_bytes,
        })
    }

    pub fn coefficient_bound(self) -> u128 {
        self.bound
    }
    pub fn bytes_per_coefficient(self) -> usize {
        self.width
    }
    pub fn payload_bytes(self) -> usize {
        self.payload_bytes
    }
    pub fn coordinates(self) -> usize {
        self.coordinates
    }

    pub fn encode(self, values: &[i128]) -> Result<Vec<u8>, AkitaError> {
        if values.len() != self.coordinates || values.iter().any(|v| v.unsigned_abs() > self.bound)
        {
            return Err(AkitaError::InvalidProof);
        }
        let mut out = Vec::with_capacity(self.payload_bytes);
        for value in values {
            out.extend_from_slice(&value.to_le_bytes()[..self.width]);
        }
        Ok(out)
    }

    pub fn decode(self, payload: &[u8]) -> Result<Vec<i128>, AkitaError> {
        if payload.len() != self.payload_bytes {
            return Err(AkitaError::InvalidProof);
        }
        let mut out = Vec::with_capacity(self.coordinates);
        for encoded in payload.chunks_exact(self.width) {
            let sign = encoded.last().ok_or(AkitaError::InvalidProof)? & 0x80 != 0;
            let mut bytes = [if sign { 0xff } else { 0 }; 16];
            bytes[..self.width].copy_from_slice(encoded);
            let value = i128::from_le_bytes(bytes);
            if value.unsigned_abs() > self.bound {
                return Err(AkitaError::InvalidProof);
            }
            out.push(value);
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_signed_width_and_integer_bounds() {
        let codec = FullRingTerminalCodec::new(100, 5, u128::MAX - 100).unwrap();
        let values = [-89600, -1, 0, 1, 89600];
        let encoded = codec.encode(&values).unwrap();
        assert_eq!(codec.bytes_per_coefficient(), 3);
        assert_eq!(codec.decode(&encoded).unwrap(), values);
        assert!(codec.decode(&encoded[..14]).is_err());
        let mut extra = encoded.clone();
        extra.push(0);
        assert!(codec.decode(&extra).is_err());
        assert!(codec.encode(&[-89601, -1, 0, 1, 89600]).is_err());
        let mut outside = encoded;
        outside[..3].copy_from_slice(&89601i128.to_le_bytes()[..3]);
        assert!(codec.decode(&outside).is_err());
        assert!(FullRingTerminalCodec::new(1, 1, 1792).is_err());
        assert!(FullRingTerminalCodec::new(1, 1, 1793).is_ok());
    }
}
