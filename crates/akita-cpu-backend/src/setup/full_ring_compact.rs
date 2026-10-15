//! Install the compact source registry on the original native shared setup.

use akita_config::{full_ring::CompactInner, TrustedScheduleCatalog};
use akita_error::AkitaError;
use akita_params::GroupCommitPhaseParams;
use akita_types::full_ring::RawSourceViewPlan;
use akita_types::{
    AkitaExpandedSetup, FullRingCommitmentRoles, FullRingMatrixDescriptor, FullRingMatrixRole,
    FullRingSetupDescriptor, FullRingSetupSource, FullRingSourceId, InstalledFullRingSetup,
};
use jolt_field::Prime128OffsetA7F7;
use std::sync::Arc;

/// Derive the original 33-role inventory from the admitted compact catalog.
///
/// The selector and descriptor must belong to the caller's approved installation.
/// This checks geometry and table admission, not descriptor authentication, setup
/// randomness, or a D1 security claim. Installation binds the actual native prefix.
///
/// # Errors
///
/// Rejects a missing compact row, an invalid source map or an inconsistent registry.
pub fn compact_inner_setup_descriptor(
    catalog: &TrustedScheduleCatalog<CompactInner>,
    installation_id: [u8; 32],
    original_descriptor: Arc<[u8]>,
) -> Result<FullRingSetupDescriptor, AkitaError> {
    let (key, _) = CompactInner::root_inputs()?;
    let row = catalog.resolve_key(&key)?;
    let schedule = row.schedule();
    let [receiver, ram, _] = key.precommitteds.as_slice() else {
        return Err(invalid_registry());
    };
    let [_, final_2m, final_1m] = schedule.recursive_folds.as_slice() else {
        return Err(invalid_registry());
    };
    let profiles = [
        *receiver,
        schedule.root.params.own_group().profile,
        *ram,
        final_2m.params.own_group().profile,
        final_1m.params.own_group().profile,
    ];
    let sources = [
        source(FullRingSourceId(0), profiles[0])?,
        source(FullRingSourceId(1), profiles[1])?,
        source(FullRingSourceId(2), profiles[2])?,
        source(FullRingSourceId(3), profiles[3])?,
        source(FullRingSourceId(4), profiles[4])?,
    ];
    const DOMAIN: &[u8] = b"akita/compact-inner/native-shared-view/v2";
    let mut namespace = Vec::with_capacity(DOMAIN.len() + 32 + 16);
    namespace.extend_from_slice(DOMAIN);
    namespace.extend_from_slice(&installation_id);
    namespace
        .extend_from_slice(&akita_params::field_modulus::<Prime128OffsetA7F7>()?.to_le_bytes());
    let mut matrices = Vec::with_capacity(33);
    for (source, profile) in sources.iter().zip(profiles) {
        let id = source.id;
        push_matrix(
            &mut matrices,
            &namespace,
            source.commitment_roles.inner,
            profile.inner.matrix.ring_dimension(),
            profile.inner.matrix.output_rank(),
            profile.inner.matrix.input_width(),
        );
        push_matrix(
            &mut matrices,
            &namespace,
            FullRingMatrixRole::B { source: id },
            profile.outer.matrix.ring_dimension(),
            profile.outer.matrix.output_rank(),
            profile.outer.matrix.input_width(),
        );
        push_compression(&mut matrices, &namespace, |stage| FullRingMatrixRole::F {
            source: id,
            stage,
        });
    }
    for (index, fold) in [0, 3, 6, 9]
        .into_iter()
        .zip(std::iter::once(&schedule.root).chain(&schedule.recursive_folds))
    {
        let opening = fold.params.open().matrix;
        push_matrix(
            &mut matrices,
            &namespace,
            FullRingMatrixRole::Registered { index },
            opening.ring_dimension(),
            opening.output_rank(),
            opening.input_width(),
        );
        push_compression(&mut matrices, &namespace, |stage| {
            FullRingMatrixRole::Registered {
                index: index + 1 + u32::from(stage),
            }
        });
    }
    let terminal = schedule.terminal.inner.matrix;
    push_matrix(
        &mut matrices,
        &namespace,
        FullRingMatrixRole::TerminalA,
        terminal.ring_dimension(),
        terminal.output_rank(),
        terminal.input_width(),
    );
    FullRingSetupDescriptor::canonical_compact(
        installation_id,
        original_descriptor,
        sources,
        matrices,
    )
}

/// Install fresh compact sources under verified native shared-prefix material.
///
/// The caller approves the exact seed/descriptor and authenticates the installed
/// static data. This checks the supplied expansion, all registered view extents
/// and source capacity; it neither samples entropy nor changes existing keys.
///
/// # Errors
///
/// Rejects catalog/registry mismatch, insufficient capacity and material that
/// does not match its declared native seed.
pub fn install_compact_inner_setup(
    catalog: &TrustedScheduleCatalog<CompactInner>,
    installation_id: [u8; 32],
    original_descriptor: Arc<[u8]>,
    expanded: Arc<AkitaExpandedSetup<Prime128OffsetA7F7>>,
) -> Result<InstalledFullRingSetup<Prime128OffsetA7F7>, AkitaError> {
    let descriptor = compact_inner_setup_descriptor(catalog, installation_id, original_descriptor)?;
    InstalledFullRingSetup::from_shared_prefix(descriptor, expanded)
}

fn invalid_registry() -> AkitaError {
    AkitaError::InvalidSetup("compact catalog and original role registry disagree".into())
}

fn source(
    id: FullRingSourceId,
    profile: GroupCommitPhaseParams,
) -> Result<FullRingSetupSource, AkitaError> {
    let polynomials = profile.group.num_polynomials();
    let blocks = profile.blocks.live_blocks;
    let positions = profile.blocks.positions_per_block;
    let digit_depth = profile.inner.digits.num_digits;
    Ok(FullRingSetupSource {
        id,
        polynomials,
        blocks,
        positions,
        digit_depth,
        rank: profile.inner.matrix.output_rank(),
        raw_view: RawSourceViewPlan::new(128, polynomials, blocks, positions, digit_depth)?,
        source_encoding: akita_params::CommittedSourceEncoding::CanonicalCoefficientTable,
        commitment_roles: FullRingCommitmentRoles {
            inner: FullRingMatrixRole::A { source: id },
            outer: Some(FullRingMatrixRole::B { source: id }),
            compression: vec![
                FullRingMatrixRole::F {
                    source: id,
                    stage: 0,
                },
                FullRingMatrixRole::F {
                    source: id,
                    stage: 1,
                },
            ],
        },
    })
}

fn push_compression(
    matrices: &mut Vec<FullRingMatrixDescriptor>,
    namespace: &[u8],
    role: impl Fn(u8) -> FullRingMatrixRole,
) {
    // The independent canonical registry validator checks these fixed F/H shapes.
    for (stage, dimension, columns) in [(0, 16, 1024), (1, 8, 256)] {
        push_matrix(matrices, namespace, role(stage), dimension, 1, columns);
    }
}

fn push_matrix(
    matrices: &mut Vec<FullRingMatrixDescriptor>,
    namespace: &[u8],
    role: FullRingMatrixRole,
    ring_dimension: usize,
    rows: usize,
    columns: usize,
) {
    // This version fixes K, the canonical source/digit order and semantic role
    // IDs. These literal names describe matrices; they do not authenticate them.
    let (tag, index, stage) = match role {
        FullRingMatrixRole::A { source } => (0, source.0, 0),
        FullRingMatrixRole::B { source } => (1, source.0, 0),
        FullRingMatrixRole::F { source, stage } => (2, source.0, stage),
        FullRingMatrixRole::D => (3, 0, 0),
        FullRingMatrixRole::H { stage } => (4, 0, stage),
        FullRingMatrixRole::TerminalA => (5, 0, 0),
        FullRingMatrixRole::Registered { index } => (6, index, 0),
    };
    let mut original = Vec::with_capacity(namespace.len() + 1 + 4 + 1 + 24);
    original.extend_from_slice(namespace);
    original.push(tag);
    original.extend_from_slice(&index.to_le_bytes());
    original.push(stage);
    for value in [ring_dimension, rows, columns] {
        original.extend_from_slice(&(value as u64).to_le_bytes());
    }
    matrices.push(FullRingMatrixDescriptor {
        role,
        ring_dimension,
        rows,
        columns,
        original_descriptor: original.into(),
    });
}
