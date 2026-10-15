//! CPU setup construction and portable setup artifacts.

mod full_ring_compact;
mod prover;
pub(crate) mod setup_prefix;
pub use full_ring_compact::{compact_inner_setup_descriptor, install_compact_inner_setup};
pub use prover::AkitaProverSetup;
