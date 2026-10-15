//! One finite raw map, shared by deterministic fixtures and all channel roles.

use crate::sampler::{RangeSignSource, ShellSamplingPolicy, SignedSparseScratch, MAX_STACK_RING_DIM};
use crate::{CanonicalShellGroup, CanonicalShellLevel, Challenges, ShellGroupDescriptor,
    ShellLevelDescriptor, SparseChallenge, SHELL_MASK_CAP};
use akita_error::AkitaError;
use akita_transcript::{CompletedResult, FiniteAnswer};

/// Physical range-word width (including the empty deterministic range).
pub(crate) fn range_width(modulus: usize) -> Result<usize, AkitaError> {
    if modulus == 0 { return Err(AkitaError::InvalidProof); }
    let bits = usize::BITS - (modulus - 1).leading_zeros();
    match bits {
        0 => Ok(0), 1..=8 => Ok(1), 9..=16 => Ok(2), 17..=32 => Ok(4),
        _ => Err(AkitaError::InvalidSetup("shell range exceeds the four-byte decoder".into())),
    }
}

trait ShellAnswer {
    fn declared_len(&self) -> u64;
    fn read_exact(&mut self, out: &mut [u8]) -> Result<(), AkitaError>;
    fn skip_exact(&mut self, count: u64) -> Result<(), AkitaError>;
}
impl ShellAnswer for FiniteAnswer {
    fn declared_len(&self) -> u64 { self.declared_len() }
    fn read_exact(&mut self, out: &mut [u8]) -> Result<(), AkitaError> { Ok(self.read_exact(out)?) }
    fn skip_exact(&mut self, count: u64) -> Result<(), AkitaError> { Ok(self.skip_exact(count)?) }
}

struct ByteAnswer<'a> { bytes: &'a [u8], position: usize }
impl ShellAnswer for ByteAnswer<'_> {
    fn declared_len(&self) -> u64 { self.bytes.len() as u64 }
    fn read_exact(&mut self, out: &mut [u8]) -> Result<(), AkitaError> {
        let end = self.position.checked_add(out.len()).ok_or(AkitaError::InvalidProof)?;
        out.copy_from_slice(self.bytes.get(self.position..end).ok_or(AkitaError::InvalidProof)?);
        self.position = end;
        Ok(())
    }
    fn skip_exact(&mut self, count: u64) -> Result<(), AkitaError> {
        let count = usize::try_from(count).map_err(|_| AkitaError::InvalidProof)?;
        let end = self.position.checked_add(count).ok_or(AkitaError::InvalidProof)?;
        self.bytes.get(self.position..end).ok_or(AkitaError::InvalidProof)?;
        self.position = end;
        Ok(())
    }
}

struct FixedSlotSource<'a, A>(&'a mut A);
impl<A: ShellAnswer> RangeSignSource for FixedSlotSource<'_, A> {
    fn sample_range(&mut self, modulus: usize) -> Result<usize, AkitaError> {
        let width = range_width(modulus)?;
        if width == 0 { return Ok(0); }
        let bits = usize::BITS - (modulus - 1).leading_zeros();
        let mask = u32::MAX >> (u32::BITS - bits);
        for word_index in 0..SHELL_MASK_CAP {
            let mut bytes = [0u8; 4];
            self.0.read_exact(&mut bytes[..width])?;
            let digit = (u32::from_le_bytes(bytes) & mask) as usize;
            if digit < modulus {
                // Preserve the next step's fixed offset, even for first-word success.
                self.0.skip_exact(((SHELL_MASK_CAP - word_index - 1) * width) as u64)?;
                return Ok(digit);
            }
        }
        Err(AkitaError::InvalidInput("fatal shell mask cap exhausted".into()))
    }
    fn fill_bytes(&mut self, out: &mut [u8]) -> Result<(), AkitaError> { self.0.read_exact(out) }
}

struct GroupDecoder<'a> {
    descriptor: &'a ShellGroupDescriptor,
    policy: ShellSamplingPolicy,
    scratch: SignedSparseScratch,
    // Positive validated shell weight makes empty support an unambiguous
    // private unfinished marker. Move this same vector into the result.
    coordinates: Vec<SparseChallenge>,
}
impl<'a> GroupDecoder<'a> {
    fn new(descriptor: &'a ShellGroupDescriptor) -> Result<Self, AkitaError> {
        let config = descriptor.config();
        let mut coordinates = Vec::new();
        coordinates.try_reserve_exact(descriptor.coordinate_count()).map_err(|_| AkitaError::InvalidProof)?;
        coordinates.resize_with(descriptor.coordinate_count(), || SparseChallenge {
            positions: Default::default(), coeffs: Default::default(),
        });
        Ok(Self { descriptor, coordinates,
            policy: ShellSamplingPolicy::new(descriptor.ring_dimension(), &config, descriptor.rejection())?,
            scratch: SignedSparseScratch::new(config.count_pm1, config.count_pm2)?,
        })
    }
}

/// Canonicalize the polynomial, never its incidental shuffle representation.
fn canonicalize(challenge: &mut SparseChallenge, descriptor: &ShellGroupDescriptor) -> Result<(), AkitaError> {
    challenge.validate_dyn(descriptor.ring_dimension())?;
    let config = descriptor.config();
    if challenge.coeffs.iter().filter(|&&c| c.unsigned_abs() == 1).count() != config.count_pm1
        || challenge.coeffs.iter().filter(|&&c| c.unsigned_abs() == 2).count() != config.count_pm2
        || challenge.coeffs.len() != config.weight()
    { return Err(AkitaError::InvalidProof); }
    // Each scratch candidate has its magnitude classes already contiguous.
    // Sort position/sign pairs within each class, without a heap scratch vector.
    for (start, len) in [(0, config.count_pm1), (config.count_pm1, config.count_pm2)] {
        let end = start.checked_add(len).ok_or(AkitaError::InvalidProof)?;
        for i in start..end {
            let mut j = i;
            while j > start && challenge.positions[j] < challenge.positions[j - 1] {
                challenge.positions.swap(j, j - 1);
                challenge.coeffs.swap(j, j - 1);
                j -= 1;
            }
        }
    }
    Ok(())
}

fn decode<A: ShellAnswer>(descriptor: &ShellLevelDescriptor, answer: &mut A) -> Result<CanonicalShellLevel, AkitaError> {
    if answer.declared_len() != descriptor.raw_len() { return Err(AkitaError::InvalidProof); }
    let mut states = Vec::new();
    states.try_reserve_exact(descriptor.groups().len()).map_err(|_| AkitaError::InvalidProof)?;
    for group in descriptor.groups() { states.push(GroupDecoder::new(group)?); }
    let max_attempts = states.iter().map(|g| g.descriptor.coordinate_inputs().candidate_cap)
        .max().ok_or(AkitaError::InvalidProof)?;
    let mut remaining = descriptor.coordinate_count();
    'attempts: for attempt in 0..max_attempts {
        for state in &mut states {
            let inputs = state.descriptor.coordinate_inputs();
            if attempt >= inputs.candidate_cap { continue; }
            let config = state.descriptor.config();
            for coordinate in &mut state.coordinates {
                if !coordinate.positions.is_empty() {
                    answer.skip_exact(inputs.candidate_bytes as u64)?;
                    continue;
                }
                state.scratch.sample(&mut FixedSlotSource(answer), state.descriptor.ring_dimension(),
                    config.count_pm1, config.count_pm2)?;
                if state.policy.accepts(state.scratch.positions(), state.scratch.coeffs())? {
                    let mut challenge = state.scratch.take_challenge();
                    canonicalize(&mut challenge, state.descriptor)?;
                    *coordinate = challenge;
                    remaining = remaining.checked_sub(1).ok_or(AkitaError::InvalidProof)?;
                    if remaining == 0 { break 'attempts; }
                } else if attempt + 1 == inputs.candidate_cap {
                    return Err(AkitaError::InvalidInput("shell integer-filter candidate cap exhausted".into()));
                }
            }
        }
    }
    if remaining != 0 { return Err(AkitaError::InvalidProof); }
    let mut groups = Vec::new();
    groups.try_reserve_exact(states.len()).map_err(|_| AkitaError::InvalidProof)?;
    for state in states {
        groups.push(CanonicalShellGroup {
            descriptor: state.descriptor.clone(),
            challenges: Challenges::from_sparse(state.coordinates, state.descriptor.num_live_blocks(), state.descriptor.num_claims())?,
        });
    }
    Ok(CanonicalShellLevel { groups })
}

/// Apply the real finite map to a complete deterministic raw answer.
/// The caller supplies all declared bytes, including slack and unused tail;
/// this does not condition, modify, or invert a concrete SHAKE output.
pub fn decode_shell_level_from_bytes(descriptor: &ShellLevelDescriptor, raw: &[u8]) -> Result<CanonicalShellLevel, AkitaError> {
    decode(descriptor, &mut ByteAnswer { bytes: raw, position: 0 })
}

/// Complete the entire family before writing any canonical result bytes.
pub(crate) fn decode_and_complete(descriptor: &ShellLevelDescriptor, answer: &mut FiniteAnswer,
    completed: &mut CompletedResult) -> Result<CanonicalShellLevel, AkitaError> {
    let level = decode(descriptor, answer)?;
    write_canonical_result(&level, |bytes| Ok(completed.write_all(bytes)?))?;
    Ok(level)
}

fn write_canonical_result(level: &CanonicalShellLevel,
    mut write: impl FnMut(&[u8]) -> Result<(), AkitaError>) -> Result<(), AkitaError> {
    let mut dense = [0u8; MAX_STACK_RING_DIM];
    for group in level.groups() {
        let ring_d = group.descriptor().ring_dimension();
        let out = dense.get_mut(..ring_d).ok_or(AkitaError::InvalidProof)?;
        for challenge in group.challenges().as_slice() {
            out.fill(0);
            for (&position, &coefficient) in challenge.positions.iter().zip(&challenge.coeffs) {
                *out.get_mut(position as usize).ok_or(AkitaError::InvalidProof)? = coefficient.to_le_bytes()[0];
            }
            write(out)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
