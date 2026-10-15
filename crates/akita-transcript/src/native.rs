//! Native D1 samplers over the shared compiler, never the legacy fold seed.
//! Inner consumption is sequential and conditional on rejection. Outer vectors
//! use the distinct literal SHAKE/RankShell compiler and its reserved spans.
use jolt_field::{ExtField, Ring, Zero};
pub use jolt_transcript::d1::{decode_e, encode_e, inner, outer, shell, wire, Error, E, K, Q};

/// Two 128-bit attempts per K limb, accepting the first canonical candidate.
/// Exhaustion poisons the borrowed compiler session, including future snapshots.
pub fn inner_scalar<R: inner::Registry>(
    frozen: &mut inner::Frozen<'_, R>,
    site: u64,
) -> Result<E, Error> {
    let result = (|| {
        let mut limbs = [K::zero(); 2];
        for limb in &mut limbs {
            let mut accepted = None;
            for _ in 0..2 {
                let mut bytes = [0; 16];
                frozen.read(site, 128, &mut bytes)?;
                let value = u128::from_le_bytes(bytes);
                if value < Q {
                    accepted = Some(K::from_u128(value));
                    break;
                }
            }
            *limb = accepted.ok_or(Error::Exhausted)?;
        }
        Ok(E::from_base_fn(|i| limbs[i]))
    })();
    result.map_err(|error| frozen.reject(error))
}

/// Uniform (24 magnitude-one, 16 magnitude-two) shell in the original S128.
/// Forty partial Fisher–Yates draws use at most 256 attempts each, followed
/// by exactly forty independent sign bits. No response-dependent filtering.
pub fn inner_shell<R: inner::Registry>(
    frozen: &mut inner::Frozen<'_, R>,
    site: u64,
) -> Result<shell::Shell, Error> {
    let result = (|| {
        let mut permutation: [usize; 128] = core::array::from_fn(|i| i);
        for j in 0..40 {
            let remaining = 128usize - j;
            let bits = usize::BITS - (remaining - 1).leading_zeros();
            let mut accepted = None;
            for _ in 0..256 {
                let mut byte = [0];
                frozen.read(site, u64::from(bits), &mut byte)?;
                let candidate = usize::from(byte[0]);
                if candidate < remaining {
                    accepted = Some(candidate);
                    break;
                }
            }
            permutation.swap(j, j + accepted.ok_or(Error::Exhausted)?);
        }
        let mut signs = [0; 5];
        frozen.read(site, 40, &mut signs)?;
        let mut coefficients = [0; 128];
        for j in 0..40 {
            let magnitude = if j < 24 { 1 } else { 2 };
            coefficients[permutation[j]] = if (signs[j / 8] >> (j % 8)) & 1 == 0 {
                magnitude
            } else {
                -magnitude
            };
        }
        shell::Shell::new(coefficients)
    })();
    result.map_err(|error| frozen.reject(error))
}

/// Preserve the outer compiler's atomic bundle and complete-result history.
pub fn outer_shell_bundle<R: outer::Registry>(
    transcript: &mut outer::Transcript<R>,
    stage: &outer::Stage<'_>,
    count: u64,
) -> Result<Vec<shell::Shell>, Error> {
    transcript.shell_bundle(stage, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Registry;
    impl inner::Registry for Registry {
        fn context(&self, kind: u32, bytes: &[u8]) -> Result<(), Error> {
            if kind == 1 && bytes == b"native-sampler-test" {
                Ok(())
            } else {
                Err(Error::Registry)
            }
        }
        fn record(&self, kind: u32, bytes: &[u8]) -> Result<(), Error> {
            self.context(kind, bytes)
        }
        fn site(&self, site: u64) -> Result<(), Error> {
            if site == 7 {
                Ok(())
            } else {
                Err(Error::Registry)
            }
        }
    }
    fn transcript(bits: u64) -> inner::Transcript<Registry> {
        inner::Transcript::new(
            Registry,
            inner::Limits {
                transcript_bytes: 1024,
                record_bytes: 1024,
                records: 2,
                sites: 1,
                blocks_per_site: 100,
                component_calls: 100,
                read_bits: bits,
            },
            1,
            b"native-sampler-test",
        )
        .unwrap()
    }
    #[test]
    fn native_draws_use_live_compiler_cursor() {
        let mut left = transcript(100_000);
        let mut right = transcript(100_000);
        let mut a = left.freeze().unwrap();
        let mut b = right.freeze().unwrap();
        let scalar = inner_scalar(&mut a, 7).unwrap();
        assert_eq!(scalar, inner_scalar(&mut b, 7).unwrap());
        assert!(a.position(7).unwrap() >= 256);
        let shell = inner_shell(&mut a, 7).unwrap();
        assert_eq!(shell, inner_shell(&mut b, 7).unwrap());
        assert_eq!(
            shell
                .coefficients()
                .iter()
                .filter(|c| c.unsigned_abs() == 1)
                .count(),
            24
        );
        assert_eq!(
            shell
                .coefficients()
                .iter()
                .filter(|c| c.unsigned_abs() == 2)
                .count(),
            16
        );
        assert_eq!(a.position(7).unwrap(), b.position(7).unwrap());
    }
    #[test]
    fn rejected_native_draw_poison_survives_snapshot_drop() {
        let mut transcript = transcript(1);
        {
            let mut frozen = transcript.freeze().unwrap();
            assert_eq!(inner_scalar(&mut frozen, 7), Err(Error::Limit));
            assert_eq!(inner_shell(&mut frozen, 7), Err(Error::Failed));
        }
        assert!(matches!(transcript.freeze(), Err(Error::Failed)));
        assert_eq!(transcript.finish(), Err(Error::Failed));
    }
}
