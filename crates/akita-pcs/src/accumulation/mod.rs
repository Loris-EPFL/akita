//! Source-preserving finite-horizon query and original-RAM range arithmetic.
//!
//! The anchor is the literal original source relation and its complete endpoint
//! roster. These kernels do not authenticate sources or prove a frontend: the
//! caller must replay the installed frontend, bind canonical evidence, drive the
//! fallible sumchecks with the indexed channel, and finish the full native PCS.
//! In particular range helpers are on-cube table lifts, never off-cube shortcuts.
//! No source is copied, recommitted, divided by a challenge, or cropped to a live
//! semantic prefix. The streaming baseline uses bounded O(n^2 D) work and O(n)
//! arithmetic scratch rather than D-element field tables.

mod kernels;
mod ledger;
/// Installed indexed range/query finite program, channel adapter and evidence codec.
pub mod protocol;
mod range;
pub use kernels::*;
pub use ledger::*;
pub use range::*;

use akita_error::AkitaError;
use jolt_field::Field;

pub(crate) fn invalid(message: &str) -> AkitaError {
    AkitaError::InvalidInput(message.into())
}

pub(crate) fn cube_len(n: usize) -> Result<usize, AkitaError> {
    1usize
        .checked_shl(u32::try_from(n).map_err(|_| invalid("cube dimension overflow"))?)
        .ok_or_else(|| invalid("cube dimension overflow"))
}

pub(crate) fn eq_index<F: Field>(point: &[F], index: usize) -> F {
    point.iter().enumerate().fold(F::one(), |v, (i, &r)| {
        v * if (index >> i) & 1 == 0 {
            F::one() - r
        } else {
            r
        }
    })
}

pub(crate) fn equality<F: Field>(a: &[F], b: &[F]) -> Result<F, AkitaError> {
    if a.len() != b.len() {
        return Err(invalid("equality dimension mismatch"));
    }
    Ok(a.iter().zip(b).fold(F::one(), |v, (&x, &y)| {
        v * ((F::one() - x) * (F::one() - y) + x * y)
    }))
}

#[cfg(test)]
mod tests;
