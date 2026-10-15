//! Compile the complete ordinary native grammar from an admitted public row.
//!
//! The anchor is the actual root/suffix/terminal message order. No prover run,
//! witness, observed transcript, or proof-selected length participates in this
//! compiler. Public values and the original endpoint roster are bound separately
//! by `InheritedAugmentation` before either role creates its channel.

use crate::{derive_transcript_grinding_plan, CommitmentConfig, ResolvedScheduleRow};
use akita_challenges::{FoldChallengeDrawDomain, ShellGroupDescriptor, ShellLevelDescriptor};
use akita_error::AkitaError;
use akita_params::{BasisMode, CommittedGroupParams, FoldSuccessor, GrindingPlan, GrindingRun,
    GrindingSite, OpeningClaimsLayout, PolynomialGroupLayout, SumcheckProtocol};
use akita_serialization::AkitaSerialize;
use akita_transcript::{CompletedResultKind, IndexedLimits, IndexedOperation, IndexedRequest,
    InstalledEvent, InstalledRegistry, ProtocolContextRecord, ProtocolMessageKind,
    ProtocolSiteId, RequestSpec};
use jolt_field::{CanonicalEncoding, ExtField, Field};
use std::marker::PhantomData;

/// Application-installed whole-byte resource envelope. These limits are not
/// read from a proof and do not change the schedule's semantic grammar.
#[derive(Clone, Copy, Debug)]
pub struct NativeRegistryResources {
    pub limits: IndexedLimits,
}

impl NativeRegistryResources {
    /// Size one already-approved finite program and externally bounded context.
    /// `max_augmented_instance_bytes` covers the complete framed native statement,
    /// inherited history, and endpoint roster, not just the history payload.
    /// No raw oracle-answer bytes enter the literal prefix.
    pub fn from_trusted_bounds(
        events: &[InstalledEvent],
        max_session_bytes: u64,
        max_augmented_instance_bytes: u64,
    ) -> Result<Self, AkitaError> {
        let max_context_bytes = max_session_bytes.checked_add(max_augmented_instance_bytes)
            .ok_or_else(|| invalid("native context bounds overflow"))?;
        let mut prefix = max_context_bytes
            .checked_add(akita_transcript::INDEXED_PROVIDER.len() as u64 + 36)
            .ok_or_else(|| invalid("native header bound overflow"))?;
        let (mut proof, mut raw, mut result, mut key_max, mut descriptor_max) = (0u64, 0u64, 0u64, 0u64, 0u64);
        for event in events {
            match event {
                InstalledEvent::Message(record) => {
                    // Event frame(9) + index/site/kind/count/byte counts(76).
                    add_cost(&mut prefix, 85)?;
                    add_cost(&mut prefix, record.encoded_bytes)?;
                    if record.kind != ProtocolMessageKind::PublicValue as u32 {
                        add_cost(&mut proof, record.encoded_bytes)?;
                    }
                }
                InstalledEvent::BoundedBytes { max_len, .. } => {
                    add_cost(&mut prefix, 85)?;
                    add_cost(&mut prefix, *max_len)?;
                    add_cost(&mut proof, *max_len)?;
                }
                InstalledEvent::Request(spec) => {
                    let descriptor = u64::try_from(spec.request.descriptor().len())
                        .map_err(|_| invalid("native descriptor size overflow"))?;
                    descriptor_max = descriptor_max.max(descriptor);
                    // Request frame(9) + operation/ordinal/site/raw/descriptor-length(60).
                    let key = prefix.checked_add(69).and_then(|n| n.checked_add(descriptor))
                        .ok_or_else(|| invalid("native literal key bound overflow"))?;
                    key_max = key_max.max(key);
                    add_cost(&mut raw, spec.request.raw_len())?;
                    add_cost(&mut result, spec.result_len)?;
                    // Completion frame(9), index/request-frame/kind/result-frame(28),
                    // and the complete request(60 + descriptor), then its result.
                    add_cost(&mut prefix, 97)?;
                    add_cost(&mut prefix, descriptor)?;
                    add_cost(&mut prefix, spec.result_len)?;
                }
            }
        }
        Ok(Self { limits: IndexedLimits {
            max_context_bytes, max_prefix_bytes: prefix, max_key_bytes: key_max,
            max_descriptor_bytes: descriptor_max, max_answer_bytes: raw,
            max_result_bytes: result, max_proof_bytes: proof,
        } })
    }
}

/// Compile once and derive the resource envelope from the trusted grammar and
/// caller-approved complete context bounds. Both roles use this same builder.
pub fn approved_native_registry_from_trusted_bounds<Cfg: CommitmentConfig>(
    row: &ResolvedScheduleRow,
    basis: BasisMode,
    max_session_bytes: u64,
    max_augmented_instance_bytes: u64,
    installation_index: u64,
) -> Result<InstalledRegistry, AkitaError> {
    let events = compile_events::<Cfg>(row, basis)?;
    let resources = NativeRegistryResources::from_trusted_bounds(
        &events, max_session_bytes, max_augmented_instance_bytes)?;
    InstalledRegistry::new(installation_index, events, resources.limits)
        .map_err(|error| invalid(&format!("native registry installation: {error}")))
}

/// Build one fixed grammar shared by the prover and verifier, including every
/// suffix and the real terminal response. `row` must come from the application's
/// trusted `TrustedScheduleCatalog<Cfg>`; a row digest supplied by a proof is not
/// an installation capability.
pub fn approved_native_registry<Cfg: CommitmentConfig>(
    row: &ResolvedScheduleRow,
    basis: BasisMode,
    resources: &NativeRegistryResources,
    installation_index: u64,
) -> Result<InstalledRegistry, AkitaError> {
    let events = compile_events::<Cfg>(row, basis)?;
    validate_resources(&events, resources.limits)?;
    InstalledRegistry::new(installation_index, events, resources.limits)
        .map_err(|error| invalid(&format!("native registry installation: {error}")))
}

fn compile_events<Cfg: CommitmentConfig>(
    row: &ResolvedScheduleRow,
    basis: BasisMode,
) -> Result<Vec<InstalledEvent>, AkitaError> {
    crate::validate_config_policy::<Cfg>()?;
    row.profiles().validate(Cfg::Field::MODULUS_BITS)?;
    let layout = row.opening_layout()?;
    layout.validate_basis(basis)?;
    let schedule = row.schedule();
    schedule.validate_structure()?;
    schedule.validate_nonterminal_opening_execution(Cfg::EXT_DEGREE)?;
    if Cfg::opening_family().is_full_ring_e2() {
        return Err(invalid("full-ring strong compiler is not the ordinary native provider"));
    }
    let plan = derive_transcript_grinding_plan::<Cfg>(schedule, &layout)?;
    let mut builder = Builder::<Cfg::Field, Cfg::ExtField> {
        events: Vec::new(), plan: &plan, run: 0, ordinal: 0, field: PhantomData,
    };
    let root = &schedule.root.params;
    let relation = akita_params::RelationWitnessGeometry::for_level(root, &layout, Cfg::EXT_DEGREE)?;
    for group in 0..layout.num_groups() {
        let compression = relation.rhs_layout().compression_plan_for_group(group)?;
        let ring_dimension = compression.maps().last()
            .ok_or_else(|| invalid("root commitment has no compression map"))?.ring_dimension();
        builder.fields(akita_types::FoldSite::RootCommitment { group, ring_dimension }.id()?,
            ProtocolMessageKind::PublicValue, compression.terminal_coefficients())?;
    }
    for group in 0..layout.num_groups() {
        builder.extensions(akita_types::FoldSite::RootPoint { group }.id()?,
            ProtocolMessageKind::PublicValue, layout.group_point_num_vars(group)?)?;
    }
    let successor = schedule.recursive_folds.first().map_or(
        FoldSuccessor::Terminal(&schedule.terminal), |step| FoldSuccessor::Recursive(&step.params));
    let mut rounds = builder.nonterminal(0, root, &layout, successor, schedule.root.output_witness_len)?;
    for (index, step) in schedule.recursive_folds.iter().enumerate() {
        let level = narrow(index + 1)?;
        let layout = step.params.opening_layout_for_final_group(PolynomialGroupLayout::singleton(rounds))?;
        let geometry = step.params.outer_payload_geometry()?;
        builder.fields(akita_types::FoldSite::WitnessCommitment {
            level, ring_dimension: geometry.transcript_ring_dimension(),
        }.id()?, ProtocolMessageKind::PublicValue, geometry.transmitted_coefficients())?;
        let successor = schedule.recursive_folds.get(index + 1).map_or(
            FoldSuccessor::Terminal(&schedule.terminal), |next| FoldSuccessor::Recursive(&next.params));
        rounds = builder.nonterminal(level, &step.params, &layout, successor, step.output_witness_len)?;
    }
    builder.terminal(narrow(schedule.recursive_folds.len() + 1)?, rounds, &schedule.terminal)?;
    if builder.run != plan.runs().len() {
        return Err(invalid("native grammar left scheduled queries unconsumed"));
    }
    Ok(builder.events)
}

fn invalid(message: &str) -> AkitaError { AkitaError::InvalidSetup(message.into()) }
fn narrow(value: usize) -> Result<u32, AkitaError> {
    u32::try_from(value).map_err(|_| invalid("native grammar coordinate exceeds u32"))
}
fn site(family: u32, level: u32, stage: u32, detail: u32) -> ProtocolSiteId {
    ProtocolSiteId { family, level, stage, detail, ..ProtocolSiteId::default() }
}

struct Builder<'a, F, E> {
    events: Vec<InstalledEvent>,
    plan: &'a GrindingPlan,
    run: usize,
    ordinal: u64,
    field: PhantomData<(F, E)>,
}
impl<F: Field + CanonicalEncoding, E: ExtField<F>> Builder<'_, F, E> {
    fn message(&mut self, record: ProtocolContextRecord) {
        self.events.push(InstalledEvent::Message(record));
    }
    fn fields(&mut self, site: ProtocolSiteId, kind: ProtocolMessageKind, count: usize) -> Result<(), AkitaError> {
        self.message(akita_transcript::field_group_record::<F>(site, kind, count)?);
        Ok(())
    }
    fn extensions(&mut self, site: ProtocolSiteId, kind: ProtocolMessageKind, count: usize) -> Result<(), AkitaError> {
        self.message(akita_transcript::extension_group_record::<F, E>(site, kind, count)?);
        Ok(())
    }
    fn request(&mut self, spec: RequestSpec) -> Result<(), AkitaError> {
        if spec.request.ordinal() != self.ordinal { return Err(invalid("noncanonical native request ordinal")); }
        self.ordinal = self.ordinal.checked_add(1).ok_or_else(|| invalid("native request count overflow"))?;
        self.events.push(InstalledEvent::Request(spec));
        Ok(())
    }
    fn consume(&mut self, expected: GrindingSite) -> Result<GrindingRun, AkitaError> {
        let run = *self.plan.runs().get(self.run).ok_or_else(|| invalid("native query is absent from grinding plan"))?;
        if run.site() != expected { return Err(invalid("native event order disagrees with grinding plan")); }
        self.run += 1;
        Ok(run)
    }
    fn challenge(&mut self, expected: GrindingSite, count: usize, vector: bool) -> Result<(), AkitaError> {
        let run = self.consume(expected)?;
        if run.multiplicity() != 1 { return Err(invalid("scalar challenge has nonunit grinding multiplicity")); }
        if run.grind_bits() > 0 {
            let (nonce, _) = akita_types::grinding_records(expected, run.grind_bits(), run.nonce_bits());
            self.message(nonce);
            let detail = u32::from(run.grind_bits()) | (u32::from(run.nonce_bits()) << 8);
            self.request(RequestSpec {
                request: IndexedRequest::new(IndexedOperation::Predicate32, self.ordinal,
                    expected.site_id(detail), akita_transcript::predicate_descriptor(run.grind_bits(), run.nonce_bits())?.to_vec(), 32)?,
                result_kind: CompletedResultKind::Predicate32, result_len: 32,
            })?;
        }
        for index in 0..count {
            let mut id = expected.site_id(0);
            if vector { id.group = narrow(index)?; }
            self.request(akita_transcript::field_request_spec::<F, E>(self.ordinal, id)?)?;
        }
        Ok(())
    }
    fn scalar(&mut self, expected: GrindingSite) -> Result<(), AkitaError> {
        self.challenge(expected, 1, false)
    }
    fn sumcheck(&mut self, protocol: SumcheckProtocol, level: u32, stage: u32, rounds: usize, degree: usize) -> Result<(), AkitaError> {
        let id = |round, role| akita_types::grinding_sumcheck_site(protocol, level, stage, 0, round, role);
        self.extensions(id(0, akita_types::SumcheckRole::Claim), ProtocolMessageKind::PublicValue, 1)?;
        for round in 0..rounds {
            let round = narrow(round)?;
            self.extensions(id(round, akita_types::SumcheckRole::RoundBody), ProtocolMessageKind::ProofAtoms, degree)?;
            self.scalar(GrindingSite::SumcheckRound { protocol, level, stage, round })?;
        }
        Ok(())
    }
    fn eor(&mut self, level: u32, layout: &OpeningClaimsLayout) -> Result<(), AkitaError> {
        let (split, width) = akita_types::tensor_opening_split::<F, E>()?;
        let claims = layout.num_total_polynomials();
        let id = |stage| site(akita_transcript::SITE_FAMILY_EXTENSION_OPENING_REDUCTION, level, stage, 0);
        self.extensions(id(1), ProtocolMessageKind::PublicValue, claims)?;
        self.extensions(id(2), ProtocolMessageKind::ProofAtoms,
            claims.checked_mul(width).ok_or_else(|| invalid("EOR partial count overflow"))?)?;
        self.challenge(GrindingSite::ExtensionOpeningPoint { level }, split, true)?;
        if layout.requires_row_batch_challenge() {
            self.challenge(GrindingSite::ExtensionOpeningClaimBatch { level }, claims, true)?;
        }
        self.sumcheck(SumcheckProtocol::ExtensionOpeningReduction, level, 0,
            layout.max_num_vars().checked_sub(split).ok_or_else(|| invalid("EOR split exceeds point"))?, 2)?;
        self.extensions(id(3), ProtocolMessageKind::ProofAtoms, claims)
    }
    fn shell(&mut self, level: u32, descriptor: ShellLevelDescriptor) -> Result<(), AkitaError> {
        let run = self.consume(GrindingSite::FoldResponse { level })?;
        self.message(akita_types::fold_response_record(run.site(), run.nonce_bits()));
        for group in descriptor.groups() {
            let run = self.consume(GrindingSite::FoldChallengeGroup { level, group: narrow(group.group_index())? })?;
            if run.fold_coordinate_count() != u64::try_from(group.coordinate_count()).ok() {
                return Err(invalid("shell roster disagrees with grinding coordinates"));
            }
        }
        self.request(RequestSpec { request: descriptor.indexed_request(self.ordinal, level)?,
            result_kind: CompletedResultKind::ShellLevel, result_len: descriptor.result_len() })
    }
    fn nonterminal(&mut self, level: u32, params: &CommittedGroupParams, layout: &OpeningClaimsLayout,
        successor: FoldSuccessor<'_>, output_witness_len: usize) -> Result<usize, AkitaError> {
        let method = params.uniform_opening_method(layout)?;
        if method.is_full_ring_scalar_extension() { return Err(invalid("strong full-ring method in ordinary native grammar")); }
        let eor = method.requires_extension_opening_reduction(E::DEGREE);
        if eor { self.eor(level, layout)?; }
        if level > 0 {
            for group in 0..layout.num_groups() {
                let point_len = layout.group_point_num_vars(group)?;
                self.extensions(akita_types::FoldSite::GroupPoint { level, group }.id()?, ProtocolMessageKind::PublicValue, point_len)?;
            }
        }
        if !eor {
            self.extensions(akita_types::FoldSite::Openings { level }.id()?, ProtocolMessageKind::PublicValue, layout.num_total_polynomials())?;
        }
        let witness = akita_params::RelationWitnessGeometry::for_level(params, layout, E::DEGREE)?;
        let payload = witness.rhs_layout().opening_payload_geometry()?;
        self.fields(akita_types::FoldSite::OpeningPayload { level,
            ring_dimension: payload.transcript_ring_dimension() }.id()?, ProtocolMessageKind::ProofAtoms,
            payload.transmitted_coefficients())?;
        if layout.requires_row_batch_challenge() {
            self.challenge(GrindingSite::EvaluationBatch { level }, layout.num_total_polynomials(), true)?;
        }
        let groups = (0..layout.num_groups()).map(|group| {
            let geometry = params.group_params_geometry(layout, group)?;
            Ok(akita_types::plan_group_fold_challenges::<F, E>(&geometry, group,
                layout.group_layout(group)?.num_polynomials())?.descriptor().clone())
        }).collect::<Result<Vec<_>, AkitaError>>()?;
        self.shell(level, ShellLevelDescriptor::new(groups)?)?;
        match successor {
            FoldSuccessor::Recursive(next) => self.fields(akita_types::FoldSite::NextWitnessPayload { level }.id()?,
                ProtocolMessageKind::ProofAtoms, next.outer_payload_geometry()?.transmitted_coefficients())?,
            FoldSuccessor::Terminal(next) => self.fields(akita_types::FoldSite::NextWitnessInnerState { level }.id()?,
                ProtocolMessageKind::ProofAtoms, next.response_shape.layout.t_field_elems())?,
        }
        let geometry = params.relation_address_geometry(layout, E::DEGREE, successor.ring_dimension(), output_witness_len)?;
        let rounds = geometry.relation_point_variable_count();
        self.scalar(GrindingSite::RingSwitchAlpha { level })?;
        self.challenge(GrindingSite::Tau0Point { level }, rounds, true)?;
        self.challenge(GrindingSite::Tau1Point { level }, params.relation_row_index_num_vars(layout)?, true)?;
        let wire = akita_params::nonterminal_level_layout(F::MODULUS_BITS,
            F::MODULUS_BITS.checked_mul(narrow(E::DEGREE)?).ok_or_else(|| invalid("field bits overflow"))?,
            params, geometry, match successor { FoldSuccessor::Recursive(next) => Some(next), _ => None })?;
        let mut product_count = 0;
        for (index, stage_shape) in wire.stage1_stages().enumerate() {
            let stage = narrow(index)?;
            self.sumcheck(SumcheckProtocol::Stage1, level, stage, stage_shape.sumcheck_proof.0, stage_shape.sumcheck_proof.1)?;
            if stage_shape.child_claims > 0 {
                self.extensions(site(akita_transcript::SITE_FAMILY_STAGE1, level, stage, 1), ProtocolMessageKind::ProofAtoms, stage_shape.child_claims)?;
                self.scalar(GrindingSite::Stage1InterstageBatch { level, stage })?;
                product_count += 1;
            }
        }
        if let Some(norm) = wire.stage1_norm() {
            self.message(ProtocolContextRecord::new(site(akita_transcript::SITE_FAMILY_PHYSICAL_L2, level, 0, 1).to_bytes(),
                ProtocolMessageKind::ProofAtoms as u32, 1, 16, 0));
            self.extensions(site(akita_transcript::SITE_FAMILY_PHYSICAL_L2, level, 0, 2), ProtocolMessageKind::ProofAtoms, norm.subclaims)?;
            if norm.subclaims > 0 { self.scalar(GrindingSite::L2SubclaimBatch { level })?; }
            self.scalar(GrindingSite::L2NormMerge { level })?;
            let basis = 1usize.checked_shl(params.open().digits.log_basis)
                .ok_or_else(|| invalid("digit-range basis overflow"))?;
            let degree = akita_params::DigitRangePlan::new(basis)?.leaf_degree() + 1;
            if norm.sumcheck.iter().any(|&value| value != degree) { return Err(invalid("nonuniform physical L2 degree")); }
            self.sumcheck(SumcheckProtocol::PhysicalL2, level, 0, norm.sumcheck.len(), degree)?;
            self.extensions(site(akita_transcript::SITE_FAMILY_PHYSICAL_L2, level, 0, 3), ProtocolMessageKind::ProofAtoms, norm.virtual_evaluations)?;
        }
        self.extensions(site(akita_transcript::SITE_FAMILY_STAGE1, level, narrow(product_count)?, 2), ProtocolMessageKind::ProofAtoms, 1)?;
        if wire.stage1_norm().is_some() { self.scalar(GrindingSite::L2VirtualBatch { level })?; }
        if params.payload_mode.is_compressed() { self.scalar(GrindingSite::CompressionBinary { level })?; }
        self.scalar(GrindingSite::Stage2Batch { level })?;
        self.sumcheck(SumcheckProtocol::Stage2, level, 0, wire.stage2_sumcheck().num_rounds(), wire.stage2_sumcheck().degree_bound())?;
        self.extensions(site(akita_transcript::SITE_FAMILY_STAGE2, level, 1, 0), ProtocolMessageKind::ProofAtoms, 1)?;
        if let FoldSuccessor::Recursive(next) = successor {
            if let Some(prefix) = next.setup_prefix() {
                let slot = prefix.slot_id().ok_or_else(|| invalid("recursive setup prefix has no slot"))?;
                let mut encoded = Vec::new();
                slot.serialize_compressed(&mut encoded).map_err(|error| invalid(&format!("setup slot encoding: {error}")))?;
                self.message(akita_transcript::public_bytes_record(site(akita_transcript::SITE_FAMILY_STAGE3, level, 0, 1), encoded.len())?);
                self.extensions(site(akita_transcript::SITE_FAMILY_STAGE3, level, 0, 2), ProtocolMessageKind::ProofAtoms, 1)?;
                self.sumcheck(SumcheckProtocol::Stage3, level, 0, prefix.profile.group.num_vars(), akita_params::SETUP_SUMCHECK_DEGREE)?;
                self.extensions(site(akita_transcript::SITE_FAMILY_STAGE3, level, 0, 3), ProtocolMessageKind::ProofAtoms, 1)?;
            }
        }
        Ok(rounds)
    }
    fn terminal(&mut self, level: u32, rounds: usize, params: &akita_params::TerminalFoldParams) -> Result<(), AkitaError> {
        params.validate_opening_execution(E::DEGREE)?;
        if params.response_shape.layout.groups.len() != 1 { return Err(invalid("ordinary terminal requires one response group")); }
        self.fields(akita_types::FoldSite::TerminalTFields { level }.id()?, ProtocolMessageKind::PublicValue, params.response_shape.layout.t_field_elems())?;
        let layout = OpeningClaimsLayout::new(rounds, 1)?;
        let eor = params.opening_method.requires_extension_opening_reduction(E::DEGREE);
        if eor { self.eor(level, &layout)?; }
        // EOR inserts the extension packing coordinates again after reducing
        // the tensor tail, so the native protocol point retains its full arity.
        let point_len = rounds;
        self.extensions(akita_types::FoldSite::TerminalPoint { level }.id()?, ProtocolMessageKind::PublicValue, point_len)?;
        if !eor { self.extensions(akita_types::FoldSite::TerminalOpening { level }.id()?, ProtocolMessageKind::PublicValue, 1)?; }
        self.fields(akita_types::FoldSite::TerminalEFields { level }.id()?, ProtocolMessageKind::ProofAtoms, params.response_shape.layout.e_field_elems())?;
        let group = params.response_shape.layout.groups.first().ok_or_else(|| invalid("missing terminal group"))?;
        let rejection = if params.response_l2_sq_cap().is_some() {
            Some(akita_challenges::selective_l2_operator_norm_rejection(params.d_a(), &params.fold_challenge_config)
                .ok_or_else(|| invalid("unsupported terminal L2 challenge policy"))?)
        } else { None };
        self.shell(level, ShellLevelDescriptor::new(vec![ShellGroupDescriptor::new(
            FoldChallengeDrawDomain::EvaluationTrace, params.d_a(), 0, params.blocks.live_blocks,
            1, params.fold_challenge_config, rejection)?])?)?;
        let max_len = u64::try_from(group.z_payload_bytes).map_err(|_| invalid("terminal payload bound overflow"))?;
        u32::try_from(group.z_payload_bytes).map_err(|_| invalid("terminal payload exceeds length codec"))?;
        let (length, payload) = akita_transcript::bounded_bytes_sites(akita_types::FoldSite::TerminalZPayload { level }.id()?);
        self.message(length);
        self.events.push(InstalledEvent::BoundedBytes { site: payload, max_len });
        Ok(())
    }
}

fn add_cost(total: &mut u64, value: u64) -> Result<(), AkitaError> {
    *total = total.checked_add(value).ok_or_else(|| invalid("native resource total overflow"))?;
    Ok(())
}

// Check caller-installed policies against the same complete-program accounting
// used by the automatic bounds constructor.
fn validate_resources(events: &[InstalledEvent], limits: IndexedLimits) -> Result<(), AkitaError> {
    let required = NativeRegistryResources::from_trusted_bounds(events, limits.max_context_bytes, 0)?.limits;
    if required.max_prefix_bytes > limits.max_prefix_bytes
        || required.max_key_bytes > limits.max_key_bytes
        || required.max_descriptor_bytes > limits.max_descriptor_bytes
        || required.max_proof_bytes > limits.max_proof_bytes
        || required.max_answer_bytes > limits.max_answer_bytes
        || required.max_result_bytes > limits.max_result_bytes
    {
        return Err(invalid("native complete-program resource budget exceeded"));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
