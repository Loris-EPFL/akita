//! Installed matrix views and original source roles for full-ring protocols.
//!
//! Registered inner sources use one authenticated native shared prefix. The
//! separate outer-direct family retains its independently provisioned law.
//! Structural descriptors are not security certificates or setup authority.
//! Sharing numerical views never merges original source identities or paths.

use akita_error::AkitaError;
use akita_params::FlatMatrix;
use jolt_field::{CanonicalEncoding, Field};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Stable installed source-contract identity, independent of occurrence order.
///
/// This selects the original geometry, raw source map and A/B/F roles, not a
/// committed instance or public handle. Multiple ordered occurrences may use
/// the same contract while retaining separate commitments and opening paths.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FullRingSourceId(pub u32);

/// Original matrix namespace. Compression stages are zero-based.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FullRingMatrixRole {
    /// Original source A matrix.
    A { source: FullRingSourceId },
    /// Original unsliced B matrix.
    B { source: FullRingSourceId },
    /// Original F compression map.
    F { source: FullRingSourceId, stage: u8 },
    /// Shared opening D matrix.
    D,
    /// Shared opening H compression map.
    H { stage: u8 },
    /// Distinct A-only successor matrix.
    TerminalA,
    /// Role in a separately admitted inner/recursive registry. The index is
    /// original installation metadata, never a proof-local group index.
    Registered { index: u32 },
}

/// Exact original matrix shape and authenticated descriptor preimage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FullRingMatrixDescriptor {
    /// Original role, never inferred from shape.
    pub role: FullRingMatrixRole,
    /// Native negacyclic degree.
    pub ring_dimension: usize,
    /// Matrix output rank.
    pub rows: usize,
    /// Matrix input width in native ring elements.
    pub columns: usize,
    /// Original installed role descriptor; retained verbatim, not a certificate.
    pub original_descriptor: Arc<[u8]>,
}

impl FullRingMatrixDescriptor {
    /// Checked literal coefficient count.
    pub fn coefficient_count(&self) -> Result<usize, AkitaError> {
        self.rows
            .checked_mul(self.columns)
            .and_then(|n| n.checked_mul(self.ring_dimension))
            .filter(|&n| n != 0)
            .ok_or_else(|| {
                AkitaError::InvalidSetup("full-ring matrix shape is empty or overflows".into())
            })
    }
}

/// Exact role tuple for one original full commitment or A-only terminal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FullRingCommitmentRoles {
    /// Original A role.
    pub inner: FullRingMatrixRole,
    /// Original B role; absent only for A-only terminal execution.
    pub outer: Option<FullRingMatrixRole>,
    /// Original F roles in stage order.
    pub compression: Vec<FullRingMatrixRole>,
}

/// Installed successor of one nonterminal fold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FullRingSuccessor {
    /// A full commitment under the named original source's A/B/F roles.
    FullCommitment { source: FullRingSourceId },
    /// A-only source under its exact original terminal A role.
    Terminal { a: FullRingMatrixRole },
}

/// Frozen role occurrences for one nonterminal `ProofContext::fold_level`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FullRingLevelRoles {
    /// Ordered source-contract occurrences in physical execution order
    /// (`OpeningClaimsLayout::root_group_order` at the root), not public
    /// claim-vector order. Repeated IDs retain separate D segments, witness
    /// spans and opening paths; explicit group indices map back to public claims.
    pub sources: Vec<FullRingSourceId>,
    /// This level's D role (shared only among its listed source groups).
    pub opening: FullRingMatrixRole,
    /// This level's H roles in stage order.
    pub compression: Vec<FullRingMatrixRole>,
    /// Successor produced before this level's strong challenges.
    pub successor: FullRingSuccessor,
}

/// Structural installation family; this is not a certificate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FullRingSetupFamily {
    /// Ordinary outer grouped-root/direct-terminal inventory.
    OuterDirect,
    /// Separately admitted inner/recursive registry.
    Registered,
}

/// Original source geometry fixed before provisioning.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FullRingSetupSource {
    /// Unique installed source-contract identity, not a committed-instance ID.
    pub id: FullRingSourceId,
    /// Number of original polynomials.
    pub polynomials: usize,
    /// Live blocks per polynomial.
    pub blocks: usize,
    /// Positions per block.
    pub positions: usize,
    /// Original source digit depth.
    pub digit_depth: usize,
    /// Original A rank.
    pub rank: usize,
    /// Exact original physical/raw injection, including its registered storage
    /// permutation and virtual holes. Query preparation must compare this plan.
    pub raw_view: crate::full_ring::RawSourceViewPlan,
    /// Original K-source encoding; selected roles never tensor-pack the source.
    pub source_encoding: akita_params::CommittedSourceEncoding,
    /// Original A/B/F namespaces; never inferred from geometry or group order.
    pub commitment_roles: FullRingCommitmentRoles,
}

impl FullRingSetupSource {
    fn validate_view(&self) -> Result<(), AkitaError> {
        if self.source_encoding != akita_params::CommittedSourceEncoding::CanonicalCoefficientTable
            || (
                self.raw_view.degree(),
                self.raw_view.polynomials(),
                self.raw_view.blocks(),
                self.raw_view.positions(),
                self.raw_view.digits(),
            ) != (
                128,
                self.polynomials,
                self.blocks,
                self.positions,
                self.digit_depth,
            )
        {
            return Err(AkitaError::InvalidSetup(
                "installed source view/encoding disagrees with its original geometry".into(),
            ));
        }
        self.raw_view.validate_fold_admission()
    }
}

/// Complete structural matrix inventory for one authenticated installation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FullRingSetupDescriptor {
    installation_id: [u8; 32],
    original_descriptor: Arc<[u8]>,
    sources: Vec<FullRingSetupSource>,
    matrices: BTreeMap<FullRingMatrixRole, FullRingMatrixDescriptor>,
    coefficient_count: usize,
    shared_prefix: Option<super::setup::AkitaSetupDescriptor>,
    family: FullRingSetupFamily,
    levels: Vec<FullRingLevelRoles>,
}

impl FullRingSetupDescriptor {
    /// Validate the complete 4G+4 role inventory and exact selected geometry.
    /// Authentication, provisioning independence and catalogue certificates are
    /// external prerequisites; this constructor asserts none of them.
    /// `sources` must use the admitted root's physical execution order.
    pub fn outer_direct(
        installation_id: [u8; 32],
        original_descriptor: Arc<[u8]>,
        sources: Vec<FullRingSetupSource>,
        terminal_rank: usize,
        terminal_positions: usize,
        matrices: Vec<FullRingMatrixDescriptor>,
    ) -> Result<Self, AkitaError> {
        let invalid =
            || AkitaError::InvalidSetup("invalid installed full-ring role inventory".into());
        let product = |a: usize, b: usize| a.checked_mul(b).filter(|&n| n > 0).ok_or_else(invalid);
        if sources.is_empty() || original_descriptor.is_empty() {
            return Err(invalid());
        }
        let mut expected = BTreeMap::new();
        let mut blocks = 0usize;
        for source in &sources {
            source.validate_view()?;
            if source.commitment_roles
                != (FullRingCommitmentRoles {
                    inner: FullRingMatrixRole::A { source: source.id },
                    outer: Some(FullRingMatrixRole::B { source: source.id }),
                    compression: vec![
                        FullRingMatrixRole::F {
                            source: source.id,
                            stage: 0,
                        },
                        FullRingMatrixRole::F {
                            source: source.id,
                            stage: 1,
                        },
                    ],
                })
            {
                return Err(invalid());
            }
            if source.digit_depth == 0
                || source.digit_depth > 26
                || !source.positions.is_power_of_two()
            {
                return Err(invalid());
            }
            let pb = product(source.polynomials, source.blocks)?;
            blocks = blocks.checked_add(pb).ok_or_else(invalid)?;
            let width = product(source.positions, source.digit_depth)?;
            if expected
                .insert(
                    FullRingMatrixRole::A { source: source.id },
                    (128, source.rank, width),
                )
                .is_some()
            {
                return Err(invalid());
            }
            expected.insert(
                FullRingMatrixRole::B { source: source.id },
                (128, 1, product(product(26, pb)?, source.rank)?),
            );
            expected.insert(
                FullRingMatrixRole::F {
                    source: source.id,
                    stage: 0,
                },
                (16, 1, 1024),
            );
            expected.insert(
                FullRingMatrixRole::F {
                    source: source.id,
                    stage: 1,
                },
                (8, 1, 256),
            );
        }
        if !terminal_positions.is_power_of_two() {
            return Err(invalid());
        }
        expected.insert(FullRingMatrixRole::D, (128, 1, product(52, blocks)?));
        expected.insert(FullRingMatrixRole::H { stage: 0 }, (16, 1, 1024));
        expected.insert(FullRingMatrixRole::H { stage: 1 }, (8, 1, 256));
        expected.insert(
            FullRingMatrixRole::TerminalA,
            (128, terminal_rank, terminal_positions),
        );
        if matrices.len() != expected.len() {
            return Err(invalid());
        }
        let mut inventory = BTreeMap::new();
        let mut coefficient_count = 0usize;
        for matrix in matrices {
            if matrix.original_descriptor.is_empty()
                || expected.remove(&matrix.role)
                    != Some((matrix.ring_dimension, matrix.rows, matrix.columns))
            {
                return Err(invalid());
            }
            coefficient_count = coefficient_count
                .checked_add(matrix.coefficient_count()?)
                .ok_or_else(invalid)?;
            inventory.insert(matrix.role, matrix);
        }
        let levels = vec![FullRingLevelRoles {
            sources: sources.iter().map(|source| source.id).collect(),
            opening: FullRingMatrixRole::D,
            compression: vec![
                FullRingMatrixRole::H { stage: 0 },
                FullRingMatrixRole::H { stage: 1 },
            ],
            successor: FullRingSuccessor::Terminal {
                a: FullRingMatrixRole::TerminalA,
            },
        }];
        let descriptor = Self {
            installation_id,
            original_descriptor,
            sources,
            matrices: inventory,
            coefficient_count,
            shared_prefix: None,
            family: FullRingSetupFamily::OuterDirect,
            levels,
        };
        descriptor.validate_levels()?;
        Ok(descriptor)
    }

    /// Retain a separately admitted inner/recursive role inventory.
    ///
    /// This constructor does not import outer 4G+4 geometry. The finite inner
    /// catalogue must validate every occurrence against its original registry
    /// (including shared RAM roles and all final shrinking folds) at admission.
    /// Source definitions must have unique IDs; level source lists are ordered
    /// occurrences and may reference the same exact contract more than once.
    pub fn from_registered_inventory(
        installation_id: [u8; 32],
        original_descriptor: Arc<[u8]>,
        sources: Vec<FullRingSetupSource>,
        matrices: Vec<FullRingMatrixDescriptor>,
        levels: Vec<FullRingLevelRoles>,
    ) -> Result<Self, AkitaError> {
        if original_descriptor.is_empty() || matrices.is_empty() {
            return Err(AkitaError::InvalidSetup(
                "empty installed role inventory".into(),
            ));
        }
        let mut inventory = BTreeMap::new();
        let mut coefficient_count = 0usize;
        for matrix in matrices {
            if matrix.original_descriptor.is_empty() || !matrix.ring_dimension.is_power_of_two() {
                return Err(AkitaError::InvalidSetup(
                    "invalid registered role descriptor".into(),
                ));
            }
            coefficient_count = coefficient_count.max(matrix.coefficient_count()?);
            if inventory.insert(matrix.role, matrix).is_some() {
                return Err(AkitaError::InvalidSetup(
                    "duplicate original registered role".into(),
                ));
            }
        }
        let mut ids = std::collections::BTreeSet::new();
        if sources.iter().any(|source| {
            !ids.insert(source.id)
                || source.polynomials == 0
                || source.blocks == 0
                || source.positions == 0
                || source.rank == 0
                || source.digit_depth == 0
        }) {
            return Err(AkitaError::InvalidSetup(
                "invalid registered source inventory".into(),
            ));
        }
        for source in &sources {
            source.validate_view()?;
        }
        let descriptor = Self {
            installation_id,
            original_descriptor,
            sources,
            matrices: inventory,
            coefficient_count,
            shared_prefix: None,
            family: FullRingSetupFamily::Registered,
            levels,
        };
        descriptor.validate_levels()?;
        Ok(descriptor)
    }

    /// Admit the fixed compact inner registry without generating setup material.
    ///
    /// The supplied definitions, in order, have IDs 0 (ReceiverRoot), 1 (Main),
    /// 2 (RAM), 3 (Final2M), and 4 (Final1M). Their original raw maps and descriptor
    /// bytes are retained, not replaced by shape-equivalent aliases. Each source
    /// uses its exact `A`, `B` and two `F` roles. Opening level `l` uses
    /// `Registered` indices `3*l` (D), `3*l+1` (H0), and `3*l+2` (H1).
    /// The terminal uses `TerminalA`.
    ///
    /// Root physical occurrences are Main/receiver/RAM-minus/RAM-plus; both RAM
    /// occurrences reference ID 2. Public order remains receiver/RAM-minus/
    /// RAM-plus/Main. The three shrinking levels and all original F/H paths
    /// remain distinct. Authentication, setup provisioning and catalogue
    /// admission remain the caller's responsibility; this is not a certificate.
    pub fn canonical_compact(
        installation_id: [u8; 32],
        original_descriptor: Arc<[u8]>,
        sources: [FullRingSetupSource; 5],
        matrices: Vec<FullRingMatrixDescriptor>,
    ) -> Result<Self, AkitaError> {
        let occurrences = [
            vec![
                FullRingSourceId(1),
                FullRingSourceId(0),
                FullRingSourceId(2),
                FullRingSourceId(2),
            ],
            vec![FullRingSourceId(0)],
            vec![FullRingSourceId(3)],
            vec![FullRingSourceId(4)],
        ];
        let successors = [
            FullRingSuccessor::FullCommitment {
                source: FullRingSourceId(0),
            },
            FullRingSuccessor::FullCommitment {
                source: FullRingSourceId(3),
            },
            FullRingSuccessor::FullCommitment {
                source: FullRingSourceId(4),
            },
            FullRingSuccessor::Terminal {
                a: FullRingMatrixRole::TerminalA,
            },
        ];
        let levels = occurrences
            .into_iter()
            .zip(successors)
            .enumerate()
            .map(|(level, (sources, successor))| {
                let index = 3 * level as u32;
                FullRingLevelRoles {
                    sources,
                    opening: FullRingMatrixRole::Registered { index },
                    compression: vec![
                        FullRingMatrixRole::Registered { index: index + 1 },
                        FullRingMatrixRole::Registered { index: index + 2 },
                    ],
                    successor,
                }
            })
            .collect();
        let descriptor = Self::from_registered_inventory(
            installation_id,
            original_descriptor,
            sources.into(),
            matrices,
            levels,
        )?;
        descriptor.validate_canonical_compact()?;
        Ok(descriptor)
    }

    /// Check the exact compact source contracts and ordered matrix-role template.
    ///
    /// This validates structural membership, not authentication, setup randomness
    /// or the consuming application's commitment continuity.
    pub fn validate_canonical_compact(&self) -> Result<(), AkitaError> {
        let invalid = || AkitaError::InvalidSetup("invalid compact original registry".into());
        if self.family != FullRingSetupFamily::Registered
            || self.sources.len() != 5
            || self.levels.len() != 4
            || self.matrices.len() != 33
            || self.required_shared_coefficients()? != 798_720
        {
            return Err(invalid());
        }
        let geometry = [
            (1, 32, 1024, 1, 4),
            (56, 1, 512, 1, 3),
            (1, 16, 32, 26, 3),
            (1, 16, 1024, 1, 3),
            (1, 16, 512, 1, 3),
        ];
        for (index, expected) in geometry.into_iter().enumerate() {
            let id = FullRingSourceId(index as u32);
            let source = self.source(id)?;
            if (
                source.polynomials,
                source.blocks,
                source.positions,
                source.digit_depth,
                source.rank,
            ) != expected
                || source.commitment_roles.inner != (FullRingMatrixRole::A { source: id })
                || source.commitment_roles.outer != Some(FullRingMatrixRole::B { source: id })
                || source.commitment_roles.compression.as_slice()
                    != [
                        FullRingMatrixRole::F {
                            source: id,
                            stage: 0,
                        },
                        FullRingMatrixRole::F {
                            source: id,
                            stage: 1,
                        },
                    ]
            {
                return Err(invalid());
            }
        }
        let occurrences: [&[FullRingSourceId]; 4] = [
            &[
                FullRingSourceId(1),
                FullRingSourceId(0),
                FullRingSourceId(2),
                FullRingSourceId(2),
            ],
            &[FullRingSourceId(0)],
            &[FullRingSourceId(3)],
            &[FullRingSourceId(4)],
        ];
        for (level, (roles, sources)) in self.levels.iter().zip(occurrences).enumerate() {
            let index = 3 * level as u32;
            if roles.sources != sources
                || roles.opening != (FullRingMatrixRole::Registered { index })
                || roles.compression.as_slice()
                    != [
                        FullRingMatrixRole::Registered { index: index + 1 },
                        FullRingMatrixRole::Registered { index: index + 2 },
                    ]
            {
                return Err(invalid());
            }
        }
        // Generic construction already checked each full successor against the
        // next singleton occurrence; pin the distinct terminal role and shape.
        if self.levels.last().map(|level| level.successor)
            != Some(FullRingSuccessor::Terminal {
                a: FullRingMatrixRole::TerminalA,
            })
        {
            return Err(invalid());
        }
        let terminal = self.matrix(FullRingMatrixRole::TerminalA)?;
        if (terminal.ring_dimension, terminal.rows, terminal.columns) != (128, 3, 512) {
            return Err(invalid());
        }
        Ok(())
    }

    fn validate_levels(&self) -> Result<(), AkitaError> {
        let invalid = || {
            AkitaError::InvalidSetup("incomplete or inconsistent installed level role map".into())
        };
        if self.levels.is_empty() || self.sources.is_empty() {
            return Err(invalid());
        }
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum MatrixUse {
            A,
            B,
            F(usize),
            D(usize),
            H(usize, usize),
            TerminalA,
        }
        let mut used_roles = BTreeMap::new();
        let mut use_role = |role,
                            kind,
                            owner: Option<usize>|
         -> Result<&FullRingMatrixDescriptor, AkitaError> {
            let matrix = self.matrix(role)?;
            if let Some((previous_kind, previous_owner)) = used_roles.insert(role, (kind, owner)) {
                if previous_kind != kind {
                    return Err(invalid());
                }
                match (previous_owner, owner) {
                    (Some(previous), Some(current)) => {
                        let previous = &self.sources[previous];
                        let current = &self.sources[current];
                        // The raw view includes P/B/M/delta and the complete storage map.
                        // Distinct incoming/outgoing handle IDs may share these original roles.
                        if previous.rank != current.rank
                            || previous.raw_view != current.raw_view
                            || previous.source_encoding != current.source_encoding
                            || previous.commitment_roles != current.commitment_roles
                        {
                            return Err(invalid());
                        }
                    }
                    (None, None) => {}
                    _ => return Err(invalid()),
                }
            }
            Ok(matrix)
        };
        let compression_shapes = [(16, 1, 1024), (8, 1, 256)];
        let mut used_sources = std::collections::BTreeSet::new();
        for (index, source) in self.sources.iter().enumerate() {
            let roles = &source.commitment_roles;
            let a = use_role(roles.inner, MatrixUse::A, Some(index))?;
            let width = source
                .positions
                .checked_mul(source.digit_depth)
                .ok_or_else(invalid)?;
            if (a.ring_dimension, a.rows, a.columns) != (128, source.rank, width)
                || roles.compression.len() != compression_shapes.len()
            {
                return Err(invalid());
            }
            let outer_width = source
                .polynomials
                .checked_mul(source.blocks)
                .and_then(|n| n.checked_mul(source.rank))
                .and_then(|n| n.checked_mul(26))
                .ok_or_else(invalid)?;
            let b = use_role(roles.outer.ok_or_else(invalid)?, MatrixUse::B, Some(index))?;
            if (b.ring_dimension, b.rows, b.columns) != (128, 1, outer_width) {
                return Err(invalid());
            }
            for (stage, (&role, shape)) in
                roles.compression.iter().zip(compression_shapes).enumerate()
            {
                let matrix = use_role(role, MatrixUse::F(stage), Some(index))?;
                if (matrix.ring_dimension, matrix.rows, matrix.columns) != shape {
                    return Err(invalid());
                }
            }
        }
        for (level, roles) in self.levels.iter().enumerate() {
            if roles.sources.is_empty() || roles.compression.len() != compression_shapes.len() {
                return Err(invalid());
            }
            let mut opening_width = 0usize;
            for &source in &roles.sources {
                let descriptor = self.source(source)?;
                let width = descriptor
                    .polynomials
                    .checked_mul(descriptor.blocks)
                    .and_then(|n| n.checked_mul(52))
                    .ok_or_else(invalid)?;
                opening_width = opening_width.checked_add(width).ok_or_else(invalid)?;
                used_sources.insert(source);
            }
            let d = use_role(roles.opening, MatrixUse::D(level), None)?;
            if (d.ring_dimension, d.rows, d.columns) != (128, 1, opening_width) {
                return Err(invalid());
            }
            for (stage, (&role, shape)) in
                roles.compression.iter().zip(compression_shapes).enumerate()
            {
                let matrix = use_role(role, MatrixUse::H(level, stage), None)?;
                if (matrix.ring_dimension, matrix.rows, matrix.columns) != shape {
                    return Err(invalid());
                }
            }
            match roles.successor {
                FullRingSuccessor::FullCommitment { source } => {
                    self.source(source)?;
                    if self
                        .levels
                        .get(level + 1)
                        .map(|next| next.sources.as_slice())
                        != Some(&[source][..])
                    {
                        return Err(invalid());
                    }
                }
                FullRingSuccessor::Terminal { a } => {
                    if level + 1 != self.levels.len()
                        || use_role(a, MatrixUse::TerminalA, None)?.ring_dimension != 128
                    {
                        return Err(invalid());
                    }
                }
            }
        }
        if used_sources.len() != self.sources.len()
            || used_roles.len() != self.matrices.len()
            || !matches!(
                self.levels.last().map(|level| level.successor),
                Some(FullRingSuccessor::Terminal { .. })
            )
        {
            return Err(invalid());
        }
        Ok(())
    }

    /// Original source record, including its raw injection and A/B/F roles.
    pub fn source(&self, id: FullRingSourceId) -> Result<&FullRingSetupSource, AkitaError> {
        self.sources
            .iter()
            .find(|source| source.id == id)
            .ok_or_else(|| AkitaError::InvalidSetup("uninstalled original source id".into()))
    }
    /// Exact role map for a nonterminal predecessor level.
    pub fn level(&self, fold_level: usize) -> Result<&FullRingLevelRoles, AkitaError> {
        self.levels
            .get(fold_level)
            .ok_or_else(|| AkitaError::InvalidSetup("uninstalled full-ring fold level".into()))
    }
    /// Frozen level list in protocol execution order.
    pub fn levels(&self) -> &[FullRingLevelRoles] {
        &self.levels
    }
    /// Successor identity of a nonterminal predecessor.
    pub fn successor(&self, fold_level: usize) -> Result<FullRingSuccessor, AkitaError> {
        Ok(self.level(fold_level)?.successor)
    }
    /// Structural family; catalogue/certificate gates remain external.
    pub fn family(&self) -> FullRingSetupFamily {
        self.family
    }

    /// Authenticated installation selector, not a public random seed.
    pub fn installation_id(&self) -> [u8; 32] {
        self.installation_id
    }
    /// Original complete descriptor bytes.
    pub fn original_descriptor(&self) -> &[u8] {
        &self.original_descriptor
    }
    /// Original source registry order. Each level's `sources` separately fixes
    /// its physical execution order; role-map iteration is never an authority.
    pub fn sources(&self) -> &[FullRingSetupSource] {
        &self.sources
    }
    /// Materialized coefficient count, or minimum required capacity before
    /// installation. Registered views share a prefix; outer-direct roles do not.
    pub fn coefficient_count(&self) -> usize {
        self.coefficient_count
    }
    /// Maximum prefix needed by the complete registered view inventory.
    pub fn required_shared_coefficients(&self) -> Result<usize, AkitaError> {
        self.matrices.values().try_fold(0, |maximum, matrix| {
            Ok(maximum.max(matrix.coefficient_count()?))
        })
    }
    /// Authenticated native expansion identity, absent for structural templates
    /// and the independently provisioned outer-direct family.
    pub fn shared_prefix(&self) -> Option<&super::setup::AkitaSetupDescriptor> {
        self.shared_prefix.as_ref()
    }
    /// Resolve one exact original matrix descriptor.
    pub fn matrix(
        &self,
        role: FullRingMatrixRole,
    ) -> Result<&FullRingMatrixDescriptor, AkitaError> {
        self.matrices
            .get(&role)
            .ok_or_else(|| AkitaError::InvalidSetup("uninstalled full-ring matrix role".into()))
    }
    /// Complete checked matrix inventory.
    pub fn matrices(&self) -> impl Iterator<Item = &FullRingMatrixDescriptor> {
        self.matrices.values()
    }
}

/// Immutable material and its exact original source/view inventory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstalledFullRingSetup<F: Field> {
    descriptor: Arc<FullRingSetupDescriptor>,
    material: FullRingSetupMaterial<F>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum FullRingSetupMaterial<F: Field> {
    SharedPrefix(Arc<super::setup::AkitaExpandedSetup<F>>),
    IndependentOuter(BTreeMap<FullRingMatrixRole, Arc<FlatMatrix<F>>>),
}

impl<F: Field> InstalledFullRingSetup<F> {
    /// Install fresh registered sources on the actual native prefix.
    ///
    /// Rederive and check the supplied material before retaining it. The caller
    /// must approve the seed/descriptor and its setup experiment; seed equality
    /// is not provenance, and this does not migrate already committed sources.
    pub fn from_shared_prefix(
        mut descriptor: FullRingSetupDescriptor,
        expanded: Arc<super::setup::AkitaExpandedSetup<F>>,
    ) -> Result<Self, AkitaError>
    where
        F: CanonicalEncoding + akita_serialization::Valid,
    {
        if descriptor.family != FullRingSetupFamily::Registered
            || descriptor.shared_prefix.is_some()
            || expanded.descriptor.num_field_elements < descriptor.required_shared_coefficients()?
            || descriptor.sources.iter().any(|source| {
                source.polynomials > expanded.descriptor.max_num_batched_polys
                    || source
                        .blocks
                        .checked_mul(source.positions)
                        .and_then(|rings| rings.checked_mul(128))
                        .and_then(usize::checked_next_power_of_two)
                        .is_none_or(|len| len.ilog2() as usize > expanded.descriptor.max_num_vars)
            })
        {
            return Err(AkitaError::InvalidSetup(
                "native shared setup does not cover the fresh registered sources".into(),
            ));
        }
        akita_serialization::Valid::check(expanded.as_ref())
            .map_err(|error| AkitaError::InvalidSetup(error.to_string()))?;
        descriptor.coefficient_count = expanded.descriptor.num_field_elements;
        descriptor.shared_prefix = Some(expanded.descriptor.clone());
        Ok(Self {
            descriptor: Arc::new(descriptor),
            material: FullRingSetupMaterial::SharedPrefix(expanded),
        })
    }

    /// Install the separate independently keyed outer-direct family.
    /// The caller authenticates its law and bytes. Registered inner sources
    /// must instead use a fresh verified native shared-prefix installation.
    pub fn from_provisioned(
        descriptor: Arc<FullRingSetupDescriptor>,
        matrices: Vec<(FullRingMatrixRole, FlatMatrix<F>)>,
    ) -> Result<Self, AkitaError>
    where
        F: akita_serialization::Valid,
    {
        if descriptor.family != FullRingSetupFamily::OuterDirect
            || descriptor.shared_prefix.is_some()
            || matrices.len() != descriptor.matrices.len()
        {
            return Err(AkitaError::InvalidSetup(
                "independent material requires the complete outer-direct inventory".into(),
            ));
        }
        let mut installed = BTreeMap::new();
        for (role, matrix) in matrices {
            akita_serialization::Valid::check(&matrix)
                .map_err(|error| AkitaError::InvalidSetup(error.to_string()))?;
            if matrix.num_field_elements() != descriptor.matrix(role)?.coefficient_count()?
                || installed.insert(role, Arc::new(matrix)).is_some()
            {
                return Err(AkitaError::InvalidSetup(
                    "duplicate or malformed provisioned matrix role".into(),
                ));
            }
        }
        Ok(Self {
            descriptor,
            material: FullRingSetupMaterial::IndependentOuter(installed),
        })
    }
    /// Borrow the retained original installation descriptor.
    pub fn descriptor(&self) -> &Arc<FullRingSetupDescriptor> {
        &self.descriptor
    }
    /// Borrow the backing matrix after validating the semantic role.
    /// Shared views must use the role's own row stride and prefix length.
    pub fn matrix(&self, role: FullRingMatrixRole) -> Result<&FlatMatrix<F>, AkitaError> {
        self.descriptor.matrix(role)?;
        match &self.material {
            FullRingSetupMaterial::SharedPrefix(expanded) => Ok(expanded.shared_matrix()),
            FullRingSetupMaterial::IndependentOuter(matrices) => matrices
                .get(&role)
                .map(Arc::as_ref)
                .ok_or_else(|| AkitaError::InvalidSetup("uninstalled matrix role".into())),
        }
    }
    /// Borrow the exact coefficient prefix for one installed numerical view.
    pub fn matrix_coefficients(&self, role: FullRingMatrixRole) -> Result<&[F], AkitaError> {
        let count = self.descriptor.matrix(role)?.coefficient_count()?;
        self.matrix(role)?
            .as_field_slice()
            .get(..count)
            .ok_or_else(|| AkitaError::InvalidSetup("installed matrix prefix is too short".into()))
    }
    /// Retain an independent outer role's allocation for its prepared cache.
    pub fn shared_matrix(
        &self,
        role: FullRingMatrixRole,
    ) -> Result<Arc<FlatMatrix<F>>, AkitaError> {
        match &self.material {
            FullRingSetupMaterial::IndependentOuter(matrices) => matrices
                .get(&role)
                .cloned()
                .ok_or_else(|| AkitaError::InvalidSetup("uninstalled matrix role".into())),
            FullRingSetupMaterial::SharedPrefix(_) => Err(AkitaError::InvalidSetup(
                "native views share the expanded setup, not separate role allocations".into(),
            )),
        }
    }
    /// Original verified native owner, shared by all registered numerical views.
    pub fn shared_prefix(&self) -> Option<&Arc<super::setup::AkitaExpandedSetup<F>>> {
        match &self.material {
            FullRingSetupMaterial::SharedPrefix(expanded) => Some(expanded),
            FullRingSetupMaterial::IndependentOuter(_) => None,
        }
    }
}
/// Setup identity includes the original semantic registry and its installed law.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AkitaSetupIdentity {
    /// Native seed-derived shared-prefix setup without a full-ring role registry.
    SharedPrefix(super::setup::AkitaSetupDescriptor),
    /// Original full-ring roles and their explicit installed matrix law.
    FullRing(Arc<FullRingSetupDescriptor>),
}

impl From<super::setup::AkitaSetupDescriptor> for AkitaSetupIdentity {
    fn from(value: super::setup::AkitaSetupDescriptor) -> Self {
        Self::SharedPrefix(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jolt_field::{Prime128OffsetA7F7, Ring, Zero};

    fn outer_matrices() -> Vec<FullRingMatrixDescriptor> {
        use FullRingMatrixRole::*;
        let source = FullRingSourceId(7);
        [
            (A { source }, 128, 2, 3),
            (B { source }, 128, 1, 52),
            (F { source, stage: 0 }, 16, 1, 1024),
            (F { source, stage: 1 }, 8, 1, 256),
            (D, 128, 1, 52),
            (H { stage: 0 }, 16, 1, 1024),
            (H { stage: 1 }, 8, 1, 256),
            (TerminalA, 128, 1, 4),
        ]
        .into_iter()
        .map(
            |(role, ring_dimension, rows, columns)| FullRingMatrixDescriptor {
                role,
                ring_dimension,
                rows,
                columns,
                original_descriptor: Arc::from(&b"original-descriptor"[..]),
            },
        )
        .collect()
    }

    fn outer(
        matrices: Vec<FullRingMatrixDescriptor>,
    ) -> Result<FullRingSetupDescriptor, AkitaError> {
        FullRingSetupDescriptor::outer_direct(
            [4; 32],
            Arc::from(&b"authenticated-outer-descriptor"[..]),
            vec![FullRingSetupSource {
                id: FullRingSourceId(7),
                polynomials: 1,
                blocks: 1,
                positions: 1,
                digit_depth: 3,
                rank: 2,
                raw_view: crate::full_ring::RawSourceViewPlan::new(128, 1, 1, 1, 3).unwrap(),
                source_encoding: akita_params::CommittedSourceEncoding::CanonicalCoefficientTable,
                commitment_roles: FullRingCommitmentRoles {
                    inner: FullRingMatrixRole::A {
                        source: FullRingSourceId(7),
                    },
                    outer: Some(FullRingMatrixRole::B {
                        source: FullRingSourceId(7),
                    }),
                    compression: vec![
                        FullRingMatrixRole::F {
                            source: FullRingSourceId(7),
                            stage: 0,
                        },
                        FullRingMatrixRole::F {
                            source: FullRingSourceId(7),
                            stage: 1,
                        },
                    ],
                },
            }],
            1,
            4,
            matrices,
        )
    }

    #[test]
    fn outer_inventory_rejects_missing_duplicate_and_relabelled_roles() {
        let matrices = outer_matrices();
        let descriptor = outer(matrices.clone()).unwrap();
        let sum: usize = matrices
            .iter()
            .map(|m| m.coefficient_count().unwrap())
            .sum();
        assert_eq!(descriptor.coefficient_count(), sum);
        let mut missing = matrices.clone();
        missing.pop();
        assert!(outer(missing).is_err());
        let mut duplicate = matrices.clone();
        duplicate[7] = duplicate[0].clone();
        assert!(outer(duplicate).is_err());
        let mut wrong = matrices;
        wrong[4].columns /= 2;
        assert!(outer(wrong).is_err());
    }

    #[test]
    fn installed_material_requires_every_exact_original_matrix() {
        type K = Prime128OffsetA7F7;
        let descriptor = Arc::new(outer(outer_matrices()).unwrap());
        let matrices = descriptor
            .matrices()
            .map(|matrix| {
                (
                    matrix.role,
                    FlatMatrix::from_flat_data(vec![
                        K::zero();
                        matrix.coefficient_count().unwrap()
                    ]),
                )
            })
            .collect::<Vec<_>>();
        let installed =
            InstalledFullRingSetup::from_provisioned(descriptor.clone(), matrices.clone()).unwrap();
        assert_eq!(
            installed.descriptor().original_descriptor(),
            b"authenticated-outer-descriptor"
        );
        let mut wrong = matrices.clone();
        wrong[0].1 = FlatMatrix::from_flat_data(vec![K::zero(); 1]);
        assert!(InstalledFullRingSetup::from_provisioned(descriptor.clone(), wrong).is_err());
        let mut duplicate = matrices;
        duplicate[1] = duplicate[0].clone();
        assert!(InstalledFullRingSetup::from_provisioned(descriptor, duplicate).is_err());
    }

    // Structural fixtures only: no setup coefficients or security certificate.
    fn compact_inventory() -> ([FullRingSetupSource; 5], Vec<FullRingMatrixDescriptor>) {
        let geometry = [
            (1, 32, 1024, 1, 4),
            (56, 1, 512, 1, 3),
            (1, 16, 32, 26, 3),
            (1, 16, 1024, 1, 3),
            (1, 16, 512, 1, 3),
        ];
        let sources = std::array::from_fn(|index| {
            let (polynomials, blocks, positions, digit_depth, rank) = geometry[index];
            let id = FullRingSourceId(index as u32);
            FullRingSetupSource {
                id,
                polynomials,
                blocks,
                positions,
                digit_depth,
                rank,
                raw_view: crate::full_ring::RawSourceViewPlan::new(
                    128,
                    polynomials,
                    blocks,
                    positions,
                    digit_depth,
                )
                .unwrap(),
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
            }
        });
        let mut matrices = Vec::new();
        let mut push = |role, ring_dimension, rows, columns| {
            matrices.push(FullRingMatrixDescriptor {
                role,
                ring_dimension,
                rows,
                columns,
                original_descriptor: Arc::from(&b"structural-test-role"[..]),
            });
        };
        for source in &sources {
            push(
                source.commitment_roles.inner,
                128,
                source.rank,
                source.positions * source.digit_depth,
            );
            push(
                source.commitment_roles.outer.unwrap(),
                128,
                1,
                source.polynomials * source.blocks * source.rank * 26,
            );
            push(source.commitment_roles.compression[0], 16, 1, 1024);
            push(source.commitment_roles.compression[1], 8, 1, 256);
        }
        for (level, width) in [6240, 1664, 832, 832].into_iter().enumerate() {
            let index = 3 * level as u32;
            push(FullRingMatrixRole::Registered { index }, 128, 1, width);
            push(
                FullRingMatrixRole::Registered { index: index + 1 },
                16,
                1,
                1024,
            );
            push(
                FullRingMatrixRole::Registered { index: index + 2 },
                8,
                1,
                256,
            );
        }
        push(FullRingMatrixRole::TerminalA, 128, 3, 512);
        (sources, matrices)
    }

    #[test]
    fn compact_registry_repeats_canonical_ram_without_deduplicating_occurrences() {
        let (sources, matrices) = compact_inventory();
        let descriptor = FullRingSetupDescriptor::canonical_compact(
            [5; 32],
            Arc::from(&b"structural-compact-test"[..]),
            sources,
            matrices,
        )
        .unwrap();
        assert_eq!(descriptor.sources().len(), 5);
        assert_eq!(descriptor.matrices().count(), 33);
        assert_eq!(descriptor.coefficient_count(), 798_720);
        assert_eq!(
            descriptor.level(0).unwrap().sources,
            [
                FullRingSourceId(1),
                FullRingSourceId(0),
                FullRingSourceId(2),
                FullRingSourceId(2)
            ]
        );
        assert_eq!(
            descriptor
                .matrix(FullRingMatrixRole::Registered { index: 0 })
                .unwrap()
                .columns,
            6240
        );
        let register = |sources, matrices, levels| {
            FullRingSetupDescriptor::from_registered_inventory(
                descriptor.installation_id,
                descriptor.original_descriptor.clone(),
                sources,
                matrices,
                levels,
            )
        };
        let matrices: Vec<_> = descriptor.matrices().cloned().collect();
        let mut duplicate = descriptor.sources.clone();
        duplicate.push(duplicate[2].clone());
        assert!(register(duplicate, matrices.clone(), descriptor.levels.clone()).is_err());
        let mut missing = descriptor.levels.clone();
        missing[0].sources.pop();
        assert!(register(descriptor.sources.clone(), matrices.clone(), missing).is_err());
        let mut deduplicated_width = matrices.clone();
        deduplicated_width
            .iter_mut()
            .find(|matrix| matrix.role == (FullRingMatrixRole::Registered { index: 0 }))
            .unwrap()
            .columns -= 16 * 52;
        assert!(register(
            descriptor.sources.clone(),
            deduplicated_width,
            descriptor.levels.clone(),
        )
        .is_err());
        let mut unknown = descriptor.levels.clone();
        unknown[0].sources[3] = FullRingSourceId(5);
        assert!(register(descriptor.sources.clone(), matrices, unknown).is_err());
    }

    #[test]
    fn compact_registry_rejects_aliases_and_wrong_original_roles() {
        let (sources, matrices) = compact_inventory();
        let construct = |sources, matrices| {
            FullRingSetupDescriptor::canonical_compact(
                [5; 32],
                Arc::from(&b"structural-compact-test"[..]),
                sources,
                matrices,
            )
        };
        let mut alias = sources.clone();
        alias[2].id = FullRingSourceId(5);
        assert!(construct(alias, matrices.clone()).is_err());
        let mut wrong = sources.clone();
        wrong[2].commitment_roles.inner = wrong[1].commitment_roles.inner;
        assert!(construct(wrong, matrices.clone()).is_err());
        let mut wrong = sources.clone();
        wrong[2].source_encoding =
            akita_params::CommittedSourceEncoding::TensorSubfieldProjection {
                extension_degree: 2,
            };
        assert!(construct(wrong, matrices.clone()).is_err());
        let mut wrong = matrices.clone();
        wrong
            .iter_mut()
            .find(|matrix| matrix.role == FullRingMatrixRole::TerminalA)
            .unwrap()
            .rows = 2;
        assert!(construct(sources.clone(), wrong).is_err());
        let mut wrong = matrices;
        wrong[0].original_descriptor = Arc::from(&b""[..]);
        assert!(construct(sources, wrong).is_err());
    }

    #[test]
    fn registered_ram_handles_share_only_matching_original_roles() {
        let original = outer(outer_matrices()).unwrap();
        let mut sources = original.sources.clone();
        let mut outgoing = sources[0].clone();
        outgoing.id = FullRingSourceId(8);
        sources.push(outgoing);
        let mut levels = original.levels.clone();
        levels[0].sources.push(FullRingSourceId(8));
        let mut matrices = outer_matrices();
        matrices
            .iter_mut()
            .find(|matrix| matrix.role == FullRingMatrixRole::D)
            .unwrap()
            .columns *= 2;
        let register = |sources| {
            FullRingSetupDescriptor::from_registered_inventory(
                original.installation_id,
                original.original_descriptor.clone(),
                sources,
                matrices.clone(),
                levels.clone(),
            )
        };
        let descriptor = register(sources.clone()).unwrap();
        let native = super::super::setup::AkitaSetupDescriptor {
            max_num_vars: 7,
            max_num_batched_polys: 1,
            num_field_elements: descriptor.required_shared_coefficients().unwrap(),
            setup_seed: [9; 32].into(),
        };
        let expanded = Arc::new(super::super::setup::AkitaExpandedSetup {
            shared_matrix: super::super::setup::derive_public_matrix_prefix::<Prime128OffsetA7F7>(
                native.num_field_elements,
                &native.setup_seed,
            ),
            descriptor: native,
        });
        let installed =
            InstalledFullRingSetup::from_shared_prefix(descriptor.clone(), expanded.clone())
                .unwrap();
        let incoming = installed.descriptor().source(FullRingSourceId(7)).unwrap();
        let outgoing = installed.descriptor().source(FullRingSourceId(8)).unwrap();
        assert!(std::ptr::eq(
            installed.matrix(incoming.commitment_roles.inner).unwrap(),
            installed.matrix(outgoing.commitment_roles.inner).unwrap(),
        ));
        assert!(Arc::ptr_eq(installed.shared_prefix().unwrap(), &expanded));
        let mut corrupt = expanded.as_ref().clone();
        let mut coefficients = corrupt.shared_matrix.as_field_slice().to_vec();
        coefficients[0] += Prime128OffsetA7F7::from_u64(1);
        corrupt.shared_matrix = FlatMatrix::from_flat_data(coefficients);
        assert!(
            InstalledFullRingSetup::from_shared_prefix(descriptor.clone(), Arc::new(corrupt),)
                .is_err()
        );
        let mut too_small = expanded.as_ref().clone();
        too_small.descriptor.max_num_vars = 6;
        assert!(
            InstalledFullRingSetup::from_shared_prefix(descriptor, Arc::new(too_small),).is_err()
        );
        assert_eq!(
            installed.descriptor().level(0).unwrap().sources,
            [FullRingSourceId(7), FullRingSourceId(8)]
        );

        let mut wrong_geometry = sources.clone();
        wrong_geometry[1].rank += 1;
        assert!(register(wrong_geometry).is_err());
        let mut wrong_map = sources.clone();
        let view = &wrong_map[1].raw_view;
        let rings = view.physical_len() / 128;
        wrong_map[1].raw_view = view
            .clone()
            .with_permutation(
                (0..view.physical_len())
                    .map(|index| (rings - 1 - index / 128) * 128 + index % 128)
                    .collect(),
            )
            .unwrap();
        assert!(register(wrong_map).is_err());
        let mut factored_sources = sources.clone();
        for source in &mut factored_sources {
            source.positions = 2;
            source.digit_depth = 3;
            source.raw_view = crate::full_ring::RawSourceViewPlan::new(
                128,
                source.polynomials,
                source.blocks,
                2,
                3,
            )
            .unwrap();
        }
        let mut factored_matrices = matrices.clone();
        factored_matrices
            .iter_mut()
            .find(|matrix| matrix.role == incoming.commitment_roles.inner)
            .unwrap()
            .columns = 6;
        let register_factored = |sources| {
            FullRingSetupDescriptor::from_registered_inventory(
                original.installation_id,
                original.original_descriptor.clone(),
                sources,
                factored_matrices.clone(),
                levels.clone(),
            )
        };
        let _valid_factored = register_factored(factored_sources.clone()).unwrap();
        let refactored = &mut factored_sources[1];
        refactored.positions = 1;
        refactored.digit_depth = 6;
        refactored.raw_view = crate::full_ring::RawSourceViewPlan::new(
            128,
            refactored.polynomials,
            refactored.blocks,
            1,
            6,
        )
        .unwrap();
        assert!(register_factored(factored_sources).is_err());
        let mut cross_role = sources;
        cross_role[1].commitment_roles.outer = Some(cross_role[0].commitment_roles.inner);
        assert!(register(cross_role).is_err());
    }

    #[test]
    fn registered_level_roles_remain_distinct_at_equal_shapes() {
        let original = outer(outer_matrices()).unwrap();
        let first = original.levels[0].clone();
        let first_roles = [first.opening, first.compression[0], first.compression[1]];
        let second_roles = [100, 101, 102].map(|index| FullRingMatrixRole::Registered { index });
        let mut matrices = outer_matrices();
        for (old, new) in first_roles.into_iter().zip(second_roles) {
            let mut descriptor = original.matrix(old).unwrap().clone();
            descriptor.role = new;
            descriptor.original_descriptor = format!("second-level-{new:?}").into_bytes().into();
            matrices.push(descriptor);
        }
        let mut levels = vec![first.clone(), first];
        levels[0].successor = FullRingSuccessor::FullCommitment {
            source: original.sources[0].id,
        };
        levels[1].opening = second_roles[0];
        levels[1].compression = second_roles[1..].to_vec();
        let register = |matrices, levels| {
            FullRingSetupDescriptor::from_registered_inventory(
                original.installation_id,
                original.original_descriptor.clone(),
                original.sources.clone(),
                matrices,
                levels,
            )
        };
        let _valid = register(matrices.clone(), levels.clone()).unwrap();
        for role in 0..3 {
            let mut aliased = levels.clone();
            if role == 0 {
                aliased[1].opening = first_roles[role];
            } else {
                aliased[1].compression[role - 1] = first_roles[role];
            }
            let mut retained = matrices.clone();
            retained.retain(|matrix| matrix.role != second_roles[role]);
            assert!(register(retained, aliased).is_err());
        }
    }

    #[test]
    fn original_raw_permutation_and_complete_level_map_are_retained() {
        let original = outer(outer_matrices()).unwrap();
        let mut sources = original.sources.clone();
        let view = &sources[0].raw_view;
        let rings = view.physical_len() / 128;
        sources[0].raw_view = view
            .clone()
            .with_permutation(
                (0..view.physical_len())
                    .map(|index| (rings - 1 - index / 128) * 128 + index % 128)
                    .collect(),
            )
            .unwrap();
        let installed = FullRingSetupDescriptor::outer_direct(
            original.installation_id,
            original.original_descriptor.clone(),
            sources.clone(),
            1,
            4,
            outer_matrices(),
        )
        .unwrap();
        assert_eq!(
            installed.source(FullRingSourceId(7)).unwrap().raw_view,
            sources[0].raw_view
        );
        assert_ne!(
            installed.source(FullRingSourceId(7)).unwrap().raw_view,
            original.sources[0].raw_view
        );
        let registered = |levels| {
            FullRingSetupDescriptor::from_registered_inventory(
                original.installation_id,
                original.original_descriptor.clone(),
                sources.clone(),
                outer_matrices(),
                levels,
            )
        };
        assert!(registered(original.levels.clone()).is_ok());
        assert!(registered(Vec::new()).is_err());
        let mut invalid = original.levels.clone();
        invalid[0].successor = FullRingSuccessor::FullCommitment {
            source: FullRingSourceId(7),
        };
        assert!(registered(invalid).is_err());
        let mut invalid = original.levels.clone();
        invalid[0].sources[0] = FullRingSourceId(8);
        assert!(registered(invalid).is_err());
    }
}
