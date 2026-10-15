//! Frozen whole-level shell ownership and canonical polynomial results.
//!
//! The v8 anchor is the complete indexed key/event sequence and polynomial
//! tuple shared by preview, live prover and verifier, not the former seed bytes.

use crate::{Challenges, OperatorNormRejection, SparseChallengeConfig};
use akita_error::{checked, AkitaError};
use akita_transcript::{IndexedOperation, IndexedRequest, ProtocolSiteId, SITE_FAMILY_FOLD_CHALLENGE};
use std::sync::Arc;

/// Public cap for each independent masked-range slot.
pub const SHELL_MASK_CAP: usize = 128;
/// Public candidate cap for a registered integer operator filter.
pub const SHELL_FILTER_CAP: usize = 4096;
const DESCRIPTOR_DOMAIN: &[u8] = b"akita/shell-level/v8";

/// Scheduled algebraic method of a shell coordinate. Native full-ring drawing
/// belongs to its separately named compiler and is not an ordinary fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldChallengeDrawDomain {
    /// Sample in the ambient A ring for evaluation-trace opening.
    EvaluationTrace,
    /// Sample in the declared subring, then embed with the registered stride.
    SubringCoefficientPacking { challenge_subring_dimension: usize },
}

/// Exact raw polynomial support and selected accepted-set predicate.
///
/// Raw cardinality is `binom(d,n1+n2) * binom(n1+n2,n1) * 2^(n1+n2)`.
/// With a filter, accepted support is exactly the subset passing that integer
/// predicate, not the unfiltered support or its shrunken certificate subset.
/// This descriptor does not authenticate a numerical accepted-fraction certificate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShellSupport {
    /// Sampling carrier degree, not an ambient degree after embedding.
    pub ring_dimension: usize,
    /// Magnitude-one count.
    pub count_pm1: usize,
    /// Magnitude-two count.
    pub count_pm2: usize,
    /// Exact predicate defining accepted support; `None` means the whole shell.
    pub filter: Option<OperatorNormRejection>,
}

/// Checked resource/law inputs for one independent coordinate allocation.
///
/// For range `m=d-i`, mask completion is `1-(1-m/2^ceil(log2(m)))^128`
/// (one for m=1). Their product is rho. With actual accepted fraction a,
/// x=rho*(1-a), success is rho*a*sum(x^r,r=0..J), filter exhaustion is x^J,
/// and fatal-mask mass is (1-rho)*sum(x^r,r=0..J). Neither a nor a certificate
/// is inferred from raw entropy. Full-answer conditional generation must run
/// each coordinate separately, then interleave its allocations attempt-major.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShellCoordinateInputs {
    /// Exact polynomial support/filter identity.
    pub support: ShellSupport,
    /// Fixed independent mask words per range.
    pub mask_cap: usize,
    /// One without a filter, 4096 with a registered filter.
    pub candidate_cap: usize,
    /// Position slots plus one low-bit sign byte per support position.
    pub candidate_bytes: usize,
    /// Complete finite coordinate allocation, including unused candidates.
    pub raw_bytes: usize,
}

/// One scheduled group's claim-major block coordinates, with no oracle work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellGroupDescriptor {
    domain: FoldChallengeDrawDomain,
    ring_d: usize,
    ambient_ring_d: usize,
    embedding_stride: usize,
    group_index: usize,
    num_live_blocks: usize,
    num_claims: usize,
    config: SparseChallengeConfig,
    rejection: Option<OperatorNormRejection>,
    coordinate_count: usize,
    candidate_bytes: usize,
    raw_bytes: usize,
}

impl ShellGroupDescriptor {
    /// Validate all dimensions, counts, registered filters, caps and byte sizes.
    /// Config admission/entropy and schedule authentication remain the planner's
    /// responsibility; this constructor accepts structurally valid test shells.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        domain: FoldChallengeDrawDomain, ring_d: usize, group_index: usize,
        num_live_blocks: usize, num_claims: usize, config: SparseChallengeConfig,
        rejection: Option<OperatorNormRejection>,
    ) -> Result<Self, AkitaError> {
        if ring_d == 0 || ring_d > crate::sampler::MAX_STACK_RING_DIM
            || num_live_blocks == 0 || num_claims == 0
        {
            return Err(AkitaError::InvalidSetup("invalid shell carrier or group counts".into()));
        }
        config.validate_dyn(ring_d).map_err(|e| AkitaError::InvalidSetup(e.into()))?;
        if config.weight() == 0 {
            return Err(AkitaError::InvalidSetup("shell weight must be positive".into()));
        }
        if let FoldChallengeDrawDomain::SubringCoefficientPacking { challenge_subring_dimension } = domain {
            if challenge_subring_dimension != ring_d || rejection.is_some() {
                return Err(AkitaError::InvalidSetup("invalid coefficient-packing shell family".into()));
            }
        }
        if let Some(filter) = rejection {
            filter.validate(ring_d, &config).map_err(|e| AkitaError::InvalidSetup(e.into()))?;
        }
        let coordinate_count = num_live_blocks.checked_mul(num_claims).ok_or(AkitaError::InvalidProof)?;
        let widths = (0..config.weight()).try_fold(0usize, |sum, i| {
            sum.checked_add(crate::shell_decode::range_width(ring_d - i)?)
                .ok_or(AkitaError::InvalidProof)
        })?;
        let candidate_bytes = checked::mul_add(SHELL_MASK_CAP, widths, config.weight())
            .ok_or(AkitaError::InvalidProof)?;
        let cap = if rejection.is_some() { SHELL_FILTER_CAP } else { 1 };
        let raw_bytes = checked::product([coordinate_count, candidate_bytes, cap])
            .ok_or(AkitaError::InvalidProof)?;
        // Every integer below is encoded in the installed descriptor as LE64.
        for value in [ring_d, group_index, num_live_blocks, num_claims, coordinate_count,
            config.count_pm1, config.count_pm2, candidate_bytes, raw_bytes]
        {
            u64::try_from(value).map_err(|_| AkitaError::InvalidProof)?;
        }
        Ok(Self { domain, ring_d, ambient_ring_d: ring_d, embedding_stride: 1,
            group_index, num_live_blocks, num_claims, config, rejection,
            coordinate_count, candidate_bytes, raw_bytes })
    }

    /// Bind the actual native role-ring embedding in the frozen request.
    pub fn with_native_embedding(mut self, ambient_ring_d: usize, embedding_stride: usize) -> Result<Self, AkitaError> {
        if checked::product([self.ring_d, embedding_stride]) != Some(ambient_ring_d)
            || embedding_stride == 0
            || (self.domain == FoldChallengeDrawDomain::EvaluationTrace && embedding_stride != 1)
        {
            return Err(AkitaError::InvalidSetup("invalid native shell embedding".into()));
        }
        u64::try_from(ambient_ring_d).map_err(|_| AkitaError::InvalidProof)?;
        u64::try_from(embedding_stride).map_err(|_| AkitaError::InvalidProof)?;
        self.ambient_ring_d = ambient_ring_d;
        self.embedding_stride = embedding_stride;
        Ok(self)
    }

    /// Scheduled opening method.
    pub const fn domain(&self) -> FoldChallengeDrawDomain { self.domain }
    /// Sampling carrier dimension.
    pub const fn ring_dimension(&self) -> usize { self.ring_d }
    /// Native ambient ring dimension after the method's embedding.
    pub const fn ambient_ring_dimension(&self) -> usize { self.ambient_ring_d }
    /// Native coefficient embedding stride.
    pub const fn embedding_stride(&self) -> usize { self.embedding_stride }
    /// Canonical group owner.
    pub const fn group_index(&self) -> usize { self.group_index }
    /// Live block count per claim.
    pub const fn num_live_blocks(&self) -> usize { self.num_live_blocks }
    /// Claim count.
    pub const fn num_claims(&self) -> usize { self.num_claims }
    /// Exact signed coefficient counts.
    pub const fn config(&self) -> SparseChallengeConfig { self.config }
    /// Exact registered integer predicate.
    pub const fn rejection(&self) -> Option<OperatorNormRejection> { self.rejection }
    /// Number of claim-major block coordinates.
    pub const fn coordinate_count(&self) -> usize { self.coordinate_count }
    /// Actual support, filter, finite caps and full allocation inputs.
    pub fn coordinate_inputs(&self) -> ShellCoordinateInputs {
        let candidate_cap = if self.rejection.is_some() { SHELL_FILTER_CAP } else { 1 };
        ShellCoordinateInputs {
            support: ShellSupport { ring_dimension: self.ring_d,
                count_pm1: self.config.count_pm1, count_pm2: self.config.count_pm2, filter: self.rejection },
            mask_cap: SHELL_MASK_CAP, candidate_cap, candidate_bytes: self.candidate_bytes,
            raw_bytes: self.raw_bytes / self.coordinate_count,
        }
    }

    /// Check membership in the actual accepted polynomial support, using the
    /// same registered integer predicate as the finite forward decoder.
    pub fn accepts(&self, challenge: &crate::SparseChallenge) -> Result<bool, AkitaError> {
        challenge.validate_dyn(self.ring_d)?;
        if challenge.coeffs.len() != self.config.weight()
            || challenge.coeffs.iter().filter(|&&c| c.unsigned_abs() == 1).count() != self.config.count_pm1
            || challenge.coeffs.iter().filter(|&&c| c.unsigned_abs() == 2).count() != self.config.count_pm2
        { return Ok(false); }
        crate::sampler::ShellSamplingPolicy::new(self.ring_d, &self.config, self.rejection)?
            .accepts(&challenge.positions, &challenge.coeffs)
    }
}

/// Frozen roster for exactly one atomic whole-level request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellLevelDescriptor {
    groups: Vec<ShellGroupDescriptor>,
    coordinate_count: usize,
    raw_len: u64,
    result_len: u64,
    encoded: Arc<[u8]>,
}
impl ShellLevelDescriptor {
    /// Freeze every group in contiguous canonical group order.
    pub fn new(groups: Vec<ShellGroupDescriptor>) -> Result<Self, AkitaError> {
        if groups.is_empty() || groups.iter().enumerate().any(|(i, g)| g.group_index != i) {
            return Err(AkitaError::InvalidSetup("shell level requires the complete ordered group roster".into()));
        }
        let coordinate_count = checked::sum(groups.iter().map(|g| g.coordinate_count)).ok_or(AkitaError::InvalidProof)?;
        let raw_len = checked::sum(groups.iter().map(|g| g.raw_bytes)).ok_or(AkitaError::InvalidProof)?;
        let result_len = groups.iter().try_fold(0usize, |sum, g| checked::mul_add(g.coordinate_count, g.ring_d, sum))
            .ok_or(AkitaError::InvalidProof)?;
        u64::try_from(coordinate_count).map_err(|_| AkitaError::InvalidProof)?;
        let mut descriptor = Self { groups, coordinate_count,
            raw_len: u64::try_from(raw_len).map_err(|_| AkitaError::InvalidProof)?,
            result_len: u64::try_from(result_len).map_err(|_| AkitaError::InvalidProof)?,
            encoded: Arc::from([]),
        };
        descriptor.encoded = descriptor.encode()?.into();
        Ok(descriptor)
    }
    /// All scheduled groups in order.
    pub fn groups(&self) -> &[ShellGroupDescriptor] { &self.groups }
    /// Whole tuple arity.
    pub const fn coordinate_count(&self) -> usize { self.coordinate_count }
    /// Full finite raw length, not the honest read prefix.
    pub const fn raw_len(&self) -> u64 { self.raw_len }
    /// Dense canonical coefficient tuple length in bytes.
    pub const fn result_len(&self) -> u64 { self.result_len }

    /// Exact v8 descriptor codec: fixed tags/LE integers, followed by explicit
    /// `(group,claim,block)` owners. Physical allocation is attempt-major;
    /// polynomial completion is group/claim/block-major. All sizes are checked.
    fn encode(&self) -> Result<Vec<u8>, AkitaError> {
        let owners = checked::product([self.coordinate_count, 24]).ok_or(AkitaError::InvalidProof)?;
        let group_bytes = checked::product([self.groups.len(), 118]).ok_or(AkitaError::InvalidProof)?;
        let len = checked::sum([DESCRIPTOR_DOMAIN.len(), 36, group_bytes, owners]).ok_or(AkitaError::InvalidProof)?;
        let mut out = Vec::new();
        out.try_reserve_exact(len).map_err(|_| AkitaError::InvalidProof)?;
        out.extend_from_slice(DESCRIPTOR_DOMAIN);
        out.extend_from_slice(&1u32.to_le_bytes()); // Dense signed-i8 polynomial result codec.
        out.extend_from_slice(&1u32.to_le_bytes()); // Attempt-major fixed-slot raw layout.
        out.extend_from_slice(&(SHELL_MASK_CAP as u32).to_le_bytes());
        for value in [self.groups.len() as u64, self.raw_len, self.result_len] {
            out.extend_from_slice(&value.to_le_bytes());
        }
        for group in &self.groups {
            let (tag, subring) = match group.domain {
                FoldChallengeDrawDomain::EvaluationTrace => (1u32, 0usize),
                FoldChallengeDrawDomain::SubringCoefficientPacking { challenge_subring_dimension } => (2, challenge_subring_dimension),
            };
            out.extend_from_slice(&tag.to_le_bytes());
            out.extend_from_slice(&(subring as u64).to_le_bytes());
            for value in [group.ring_d, group.ambient_ring_d, group.embedding_stride, group.group_index,
                group.num_live_blocks, group.num_claims, group.config.count_pm1, group.config.count_pm2,
                group.coordinate_count, group.candidate_bytes, group.coordinate_inputs().candidate_cap]
            {
                out.extend_from_slice(&(value as u64).to_le_bytes());
            }
            out.push(u8::from(group.rejection.is_some()));
            out.extend_from_slice(&group.rejection.map_or([0; 17], OperatorNormRejection::domain_separator_bytes));
            for claim in 0..group.num_claims {
                for block in 0..group.num_live_blocks {
                    for owner in [group.group_index, claim, block] {
                        out.extend_from_slice(&(owner as u64).to_le_bytes());
                    }
                }
            }
        }
        Ok(out)
    }

    /// Borrow the frozen codec without allocation during repeated nonce probes.
    pub fn encoded_bytes(&self) -> &[u8] { &self.encoded }

    /// Copy the frozen codec for callers needing independently owned bytes.
    pub fn to_bytes(&self) -> Result<Vec<u8>, AkitaError> {
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(self.encoded.len()).map_err(|_| AkitaError::InvalidProof)?;
        bytes.extend_from_slice(&self.encoded);
        Ok(bytes)
    }

    /// Build the exact request installed by the core grammar compiler.
    pub fn indexed_request(&self, ordinal: u64, level: u32) -> Result<IndexedRequest, AkitaError> {
        Ok(IndexedRequest::new(IndexedOperation::ShellLevel, ordinal,
            ProtocolSiteId { family: SITE_FAMILY_FOLD_CHALLENGE, level, ..ProtocolSiteId::default() },
            Arc::clone(&self.encoded), self.raw_len)?)
    }
}

/// Complete canonical polynomial tuple for one group's exact descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalShellGroup {
    pub(crate) descriptor: ShellGroupDescriptor,
    pub(crate) challenges: Challenges,
}
impl CanonicalShellGroup {
    /// Full method, owner, carrier, embedding and shell metadata.
    pub const fn descriptor(&self) -> &ShellGroupDescriptor { &self.descriptor }
    /// Canonical sparse arithmetic view; magnitudes are grouped and positions sorted.
    pub const fn challenges(&self) -> &Challenges { &self.challenges }
    /// Consume the arithmetic view without copying its coordinates.
    pub fn into_challenges(self) -> Challenges { self.challenges }
}

/// All completed groups; no successful prefix value is published on failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalShellLevel {
    pub(crate) groups: Vec<CanonicalShellGroup>,
}
impl CanonicalShellLevel {
    /// Borrow all completed groups in canonical order.
    pub fn groups(&self) -> &[CanonicalShellGroup] { &self.groups }
    /// Get one canonical group, rejecting invalid ownership indices.
    pub fn group(&self, index: usize) -> Result<&CanonicalShellGroup, AkitaError> {
        self.groups.get(index).ok_or(AkitaError::InvalidProof)
    }
    /// Consume all group views for pure method-aware assembly.
    pub fn into_groups(self) -> Vec<CanonicalShellGroup> { self.groups }
}
