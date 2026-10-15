//! Owned ordinary v8 indexed transcripts and separate native D1 compilers.

mod grinding;
#[cfg(feature = "logging-transcript")]
mod logging;
pub mod native;
pub mod native_strong;
mod proof_stream;
mod sponge;

#[cfg(not(any(feature = "transcript-blake2b", feature = "transcript-keccak")))]
compile_error!("enable exactly one transcript backend: transcript-blake2b or transcript-keccak");

#[cfg(all(feature = "transcript-blake2b", feature = "transcript-keccak"))]
compile_error!("enable exactly one transcript backend: transcript-blake2b or transcript-keccak");

pub use grinding::{
    grinding_predicate_accepts, GRINDING_LITTLE_ENDIAN_BIT_ORDER, GRINDING_NONCE_SLACK_BITS,
    GRINDING_PREDICATE_BYTES, GRINDING_PREDICATE_LEN, MAX_GRINDING_BITS,
};
#[cfg(feature = "logging-transcript")]
pub use logging::{
    clear_thread_events, thread_events, thread_proof_ranges, ProofMessageRange, TranscriptEvent,
};
#[cfg(feature = "logging-transcript")]
pub use proof_stream::finish_proof_ranges;
pub use proof_stream::{
    commit_grinding_nonce, exchange_extension_group, ext_challenge, extension_slots,
    field_challenge_bytes, field_sampling_is_certified, new_prover_channel, new_verifier_channel,
    field_request_spec, field_vector_challenge_at, field_vector_descriptor, prime_descriptor,
    bounded_bytes_sites, extension_group_record, field_group_record, public_bytes_record,
    nonce_encoded_len, nonce_max_bytes, preview_grinding_predicate, prover_context,
    prover_field_challenge, public_bytes, public_extensions,
    public_fields_prover, public_fields_verifier, receive_bounded_bytes, receive_byte_group,
    receive_bytes, receive_extension, receive_field, receive_field_group, receive_grinding_nonce,
    search_grinding_nonce, send_bounded_bytes, send_byte_group, send_bytes, send_extension,
    send_field, send_field_group, verifier_context, verifier_field_challenge,
    predicate_descriptor, scalar_descriptor, CompletedResult, CompletedResultKind, ExtensionAtom,
    FieldAtom, FiniteAnswer, FoldPreview, IndexedChannel, IndexedError, IndexedLimits,
    IndexedOperation, IndexedRequest, InheritedAugmentation, InstalledContext, InstalledEvent, InstalledRegistry,
    NonceAtom, ProofChannel, ProtocolContextRecord, ProtocolMessageKind, ProtocolSiteId,
    ProverChannel, RequestSpec, U128Atom, VerifierChannel, CONTEXT_DOMAIN, FIELD_CANDIDATE_CAP,
    FIELD_CHALLENGE_BYTES, FIELD_SAMPLING_QUERY_LIMIT, INDEXED_PROVIDER, PROTOCOL_VERSION,
    SITE_FAMILY_EXTENSION_OPENING_REDUCTION, SITE_FAMILY_FOLD_BINDING, SITE_FAMILY_FOLD_CHALLENGE,
    SITE_FAMILY_NEXT_WITNESS, SITE_FAMILY_OPENING_PAYLOAD, SITE_FAMILY_PHYSICAL_L2,
    SITE_FAMILY_ROOT_STATEMENT, SITE_FAMILY_STAGE1, SITE_FAMILY_STAGE2, SITE_FAMILY_STAGE3,
    SITE_FAMILY_SUMCHECK, SITE_FAMILY_TERMINAL,
};
pub use sponge::TranscriptSponge;

/// Byte length of the separate public grinding-predicate block.
pub const TRANSCRIPT_CHALLENGE_BLOCK_LEN: usize = 32;
