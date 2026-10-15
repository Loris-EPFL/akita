//! Approved finite native context for one exact public opening statement.
//!
//! Inherited history is bound literally. This module does not verify the
//! frontend, range or query evidence; the outer protocol must replay them.

use akita_config::{
    approved_native_registry_from_trusted_bounds, transcript_instance_descriptor, CommitmentConfig,
    TrustedScheduleCatalog,
};
use akita_error::AkitaError;
use akita_params::{BasisMode, OpeningClaimsLayout, OpeningScheduleSelection};
use akita_prover::{CommitmentHandleMetadata, SelectedProverOpeningData};
use akita_transcript::{InheritedAugmentation, InstalledRegistry};
use akita_types::{
    encode_opening_endpoint_binding, AkitaSetupDescriptor, Commitment, GroupBatchStatement,
    OpeningClaims,
};
use jolt_field::{CanonicalEncoding, ExtField};

/// A finite program and exact endpoint binding for one declared native call.
///
/// This is not an authorization or a proof-validity certificate.
pub struct NativeProofContext<'history> {
    registry: InstalledRegistry,
    endpoint_binding: Vec<u8>,
    history: &'history [u8],
}

impl NativeProofContext<'_> {
    /// The program derived from the application's admitted catalog row.
    pub fn registry(&self) -> &InstalledRegistry {
        &self.registry
    }

    /// Borrow the full inherited history and exact original endpoint binding.
    pub fn augmentation(&self) -> InheritedAugmentation<'_> {
        InheritedAugmentation {
            literal_history: self.history,
            endpoint_binding: &self.endpoint_binding,
        }
    }

    /// Transfer the owned registry and endpoint bytes without copying them.
    /// The caller retains ownership of the externally borrowed literal history.
    pub fn into_parts(self) -> (InstalledRegistry, Vec<u8>) {
        (self.registry, self.endpoint_binding)
    }
}

/// Compile the native program and resource envelope from an approved statement.
///
/// Prover and verifier must construct their statements independently. The
/// installation index is assigned by the trusted application within its
/// literal run namespace, never selected by proof bytes.
///
/// # Errors
///
/// Rejects foreign rows, invalid views, incompatible profiles or shapes,
/// unsupported field/route codecs, and overflowing finite resource envelopes.
// The trust source, public statement, session and history remain explicit.
#[allow(clippy::too_many_arguments)]
pub fn prepare_native_proof_context<'history, Cfg>(
    setup: &AkitaSetupDescriptor,
    schedules: &TrustedScheduleCatalog<Cfg>,
    statement: &GroupBatchStatement<'_, Cfg::ExtField, Cfg::Field>,
    session: &[u8],
    history: &'history [u8],
    basis: BasisMode,
    installation_index: u64,
) -> Result<NativeProofContext<'history>, AkitaError>
where
    Cfg: CommitmentConfig,
    Cfg::Field: CanonicalEncoding,
    Cfg::ExtField: ExtField<Cfg::Field>,
{
    let layout = statement.claims().committed_layout()?;
    compile_context::<Cfg, _>(
        setup,
        schedules,
        statement.selection(),
        &layout,
        statement.claims(),
        session,
        history,
        basis,
        installation_index,
        |group| group.commitment(),
    )
}

/// Prepare the same context from borrowed public prover claims.
///
/// No witness, private handle identity or commitment copy enters this binding.
///
/// # Errors
///
/// Rejects incompatible rows, profiles, views, codecs and resource envelopes.
// The public claims are separate from the retained backend capabilities.
#[allow(clippy::too_many_arguments)]
pub fn prepare_native_prover_context<'history, Cfg, H>(
    setup: &AkitaSetupDescriptor,
    schedules: &TrustedScheduleCatalog<Cfg>,
    opening: &SelectedProverOpeningData<'_, Cfg::ExtField, H, Cfg::Field>,
    session: &[u8],
    history: &'history [u8],
    basis: BasisMode,
    installation_index: u64,
) -> Result<NativeProofContext<'history>, AkitaError>
where
    Cfg: CommitmentConfig,
    Cfg::Field: CanonicalEncoding,
    Cfg::ExtField: ExtField<Cfg::Field>,
    H: CommitmentHandleMetadata,
{
    compile_context::<Cfg, _>(
        setup,
        schedules,
        opening.selection(),
        opening.opening_layout(),
        opening.opening_claims(),
        session,
        history,
        basis,
        installation_index,
        |commitment| commitment,
    )
}

// Both public input forms use one codec and one approved finite-program builder.
#[allow(clippy::too_many_arguments)]
fn compile_context<'history, Cfg, C>(
    setup: &AkitaSetupDescriptor,
    schedules: &TrustedScheduleCatalog<Cfg>,
    selection: OpeningScheduleSelection,
    layout: &OpeningClaimsLayout,
    claims: &OpeningClaims<'_, Cfg::ExtField, C>,
    session: &[u8],
    history: &'history [u8],
    basis: BasisMode,
    installation_index: u64,
    commitment: impl for<'c> Fn(&'c C) -> &'c Commitment<Cfg::Field>,
) -> Result<NativeProofContext<'history>, AkitaError>
where
    Cfg: CommitmentConfig,
    Cfg::Field: CanonicalEncoding,
    Cfg::ExtField: ExtField<Cfg::Field>,
{
    let row = schedules.resolve_selection(selection)?;
    row.validate_opening_layout(layout)?;
    layout.validate_basis(basis)?;
    let (_, descriptor) = transcript_instance_descriptor::<Cfg::Field, Cfg>(
        setup,
        layout,
        selection,
        row.schedule(),
        basis,
    )?;
    let endpoint_binding =
        encode_opening_endpoint_binding(&descriptor, row.profiles(), claims, commitment)?;
    let augmentation = InheritedAugmentation {
        literal_history: history,
        endpoint_binding: &endpoint_binding,
    };
    let augmented_bytes = u64::try_from(augmentation.encoded_len(&descriptor)?)
        .map_err(|_| AkitaError::InvalidProof)?;
    let session_bytes = u64::try_from(session.len()).map_err(|_| AkitaError::InvalidProof)?;
    let registry = approved_native_registry_from_trusted_bounds::<Cfg>(
        row,
        basis,
        session_bytes,
        augmented_bytes,
        installation_index,
    )?;
    Ok(NativeProofContext {
        registry,
        endpoint_binding,
        history,
    })
}
