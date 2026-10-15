//! Finite indexed range/query programs shared by actual-source consumers.
//! The anchor is independent replay plus the complete original endpoint roster.
//! Proof bodies precede every fresh draw; no receipt-supplied recipe is decoded.
use super::{GlobalRamRange, QueryLedger, SourceEndpoint, SourceProvider, WeightedQueryKernel};
use akita_config::NativeRegistryResources;
use akita_error::AkitaError;
use akita_sumcheck::{
    prove_eq_factored_sumcheck, prove_sumcheck, verify_eq_factored_sumcheck_rounds,
    verify_sumcheck_rounds, SumcheckProverChannel, SumcheckRole, SumcheckShape,
    SumcheckVerifierChannel,
};
use akita_transcript::{
    field_group_record, field_request_spec, new_prover_channel, new_verifier_channel,
    receive_field, send_field, IndexedChannel, InstalledContext, InstalledEvent, InstalledRegistry,
    ProofChannel, ProtocolMessageKind, ProtocolSiteId, ProverChannel, VerifierChannel,
    INDEXED_PROVIDER, PROTOCOL_VERSION,
};
use jolt_field::{CanonicalBytes, Prime128OffsetA7F7 as K, Zero};

const RANGE_DOMAIN: &[u8] = b"akita/candidate1/global-range/indexed/v1";
const QUERY_DOMAIN: &[u8] = b"akita/candidate1/formal-query/indexed/v1";
const FAMILY: u32 = 0x4151_5241;

fn invalid(message: &str) -> AkitaError {
    AkitaError::InvalidInput(message.into())
}
fn narrow(value: usize) -> Result<u32, AkitaError> {
    u32::try_from(value).map_err(|_| invalid("prefix shape exceeds u32"))
}
fn site(invocation: u32, round: u32, role: u32) -> ProtocolSiteId {
    ProtocolSiteId {
        family: FAMILY,
        invocation,
        round,
        detail: role,
        ..ProtocolSiteId::default()
    }
}
#[expect(
    clippy::as_conversions,
    reason = "fixed sumcheck role discriminants fit the canonical u32 grammar tag"
)]
fn sc_site(invocation: u32, round: u32, role: SumcheckRole) -> ProtocolSiteId {
    site(invocation, round, role as u32)
}
fn request(
    events: &mut Vec<InstalledEvent>,
    ordinal: &mut u64,
    id: ProtocolSiteId,
) -> Result<(), AkitaError> {
    events.push(InstalledEvent::Request(field_request_spec::<K, K>(
        *ordinal, id,
    )?));
    *ordinal = ordinal
        .checked_add(1)
        .ok_or_else(|| invalid("prefix request count overflow"))?;
    Ok(())
}
fn fields(
    events: &mut Vec<InstalledEvent>,
    id: ProtocolSiteId,
    kind: ProtocolMessageKind,
    count: usize,
) -> Result<(), AkitaError> {
    events.push(InstalledEvent::Message(field_group_record::<K>(
        id, kind, count,
    )?));
    Ok(())
}
fn sumcheck(
    events: &mut Vec<InstalledEvent>,
    ordinal: &mut u64,
    invocation: u32,
    shape: SumcheckShape,
) -> Result<(), AkitaError> {
    fields(
        events,
        sc_site(invocation, 0, SumcheckRole::Claim),
        ProtocolMessageKind::PublicValue,
        1,
    )?;
    for round in 0..shape.num_rounds() {
        let round = narrow(round)?;
        fields(
            events,
            sc_site(invocation, round, SumcheckRole::RoundBody),
            ProtocolMessageKind::ProofAtoms,
            shape.degree_bound(),
        )?;
        request(
            events,
            ordinal,
            sc_site(invocation, round, SumcheckRole::Challenge),
        )?;
    }
    Ok(())
}
fn registry(
    events: Vec<InstalledEvent>,
    max_context_bytes: usize,
    index: u64,
) -> Result<InstalledRegistry, AkitaError> {
    let resources = NativeRegistryResources::from_trusted_bounds(
        &events,
        0,
        u64::try_from(max_context_bytes).map_err(|_| invalid("prefix context bound overflow"))?,
    )?;
    InstalledRegistry::new(index, events, resources.limits).map_err(Into::into)
}

/// Shape-only trusted range grammar; construction never runs an honest prover.
pub struct RangeProgram {
    registry: InstalledRegistry,
    rounds: usize,
    endpoints: usize,
}
impl RangeProgram {
    /// Install the grammar of the already-frozen original RAM range.
    ///
    /// # Errors
    /// Rejects an empty roster, invalid sumcheck shape or overflowing resources.
    pub fn new(
        range: &GlobalRamRange,
        max_context_bytes: usize,
        installation_index: u64,
    ) -> Result<Self, AkitaError> {
        Self::from_shape(
            range.num_vars(),
            range.ram_slots().len(),
            max_context_bytes,
            installation_index,
        )
    }
    /// Compile from a trusted finite shape, for admission and transport sizing.
    /// Arithmetic consumers still check this against the actual range kernel.
    ///
    /// # Errors
    /// Rejects empty endpoints, invalid shapes and overflowing finite resources.
    pub fn from_shape(
        rounds: usize,
        endpoints: usize,
        max_context_bytes: usize,
        installation_index: u64,
    ) -> Result<Self, AkitaError> {
        if endpoints == 0 {
            return Err(invalid("empty range endpoint roster"));
        }
        let mut events = Vec::new();
        let mut ordinal = 0;
        for coordinate in 0..rounds {
            request(&mut events, &mut ordinal, site(10, narrow(coordinate)?, 0))?;
        }
        sumcheck(&mut events, &mut ordinal, 0, SumcheckShape::new(rounds, 4)?)?;
        fields(
            &mut events,
            site(11, 0, 0),
            ProtocolMessageKind::ProofAtoms,
            4,
        )?;
        request(&mut events, &mut ordinal, site(12, 0, 0))?;
        sumcheck(&mut events, &mut ordinal, 1, SumcheckShape::new(rounds, 4)?)?;
        fields(
            &mut events,
            site(13, 0, 0),
            ProtocolMessageKind::ProofAtoms,
            1,
        )?;
        sumcheck(&mut events, &mut ordinal, 2, SumcheckShape::new(rounds, 3)?)?;
        fields(
            &mut events,
            site(14, 0, 0),
            ProtocolMessageKind::ProofAtoms,
            endpoints,
        )?;
        Ok(Self {
            registry: registry(events, max_context_bytes, installation_index)?,
            rounds,
            endpoints,
        })
    }
    /// Maximum proof-body bytes admitted by this exact installed grammar.
    ///
    /// # Errors
    /// Rejects a byte count not representable by the host.
    pub fn proof_size_bound(&self) -> Result<usize, AkitaError> {
        proof_size(&self.registry)
    }
    /// Bound the literal ledger/range context with both length frames.
    ///
    /// # Errors
    /// Rejects arithmetic overflow.
    pub fn context_size_bound(
        max_ledger_bytes: usize,
        max_range_context_bytes: usize,
    ) -> Result<usize, AkitaError> {
        RANGE_DOMAIN
            .len()
            .checked_add(16)
            .and_then(|n| n.checked_add(max_ledger_bytes))
            .and_then(|n| n.checked_add(max_range_context_bytes))
            .ok_or_else(|| invalid("range context bound overflow"))
    }
    /// Bound full registered header, grammar, bodies and completed results.
    ///
    /// # Errors
    /// Rejects arithmetic overflow or an unsupported grammar event.
    pub fn literal_history_size_bound(
        &self,
        max_context_bytes: usize,
    ) -> Result<usize, AkitaError> {
        history_size(
            &self.registry,
            max_context_bytes,
            self.proof_size_bound()?,
            3,
            self.rounds
                .checked_mul(4)
                .and_then(|n| n.checked_add(1))
                .ok_or_else(|| invalid("range draws overflow"))?,
        )
    }
    fn context(
        &self,
        ledger: &QueryLedger<K>,
        range: &GlobalRamRange,
    ) -> Result<Vec<u8>, AkitaError> {
        if ledger.is_sealed()
            || self.rounds != range.num_vars()
            || self.endpoints != range.ram_slots().len()
        {
            return Err(invalid("range program does not match frozen ledger"));
        }
        let mut context = RANGE_DOMAIN.to_vec();
        bytes(&mut context, &ledger.canonical_bytes()?)?;
        bytes(&mut context, &range.canonical_context()?)?;
        Ok(context)
    }
}
/// Shape-only degree-two query grammar; all endpoints, including Q=0, are sent.
pub struct QueryProgram {
    registry: InstalledRegistry,
    rounds: usize,
    endpoints: usize,
}
impl QueryProgram {
    /// Install one degree-two query with the entire ordered endpoint roster.
    ///
    /// Shape and index are trusted application inputs, not proof-selected data.
    ///
    /// # Errors
    /// Rejects an empty roster, invalid sumcheck shape or overflowing resources.
    pub fn new(
        num_vars: usize,
        source_count: usize,
        max_context_bytes: usize,
        installation_index: u64,
    ) -> Result<Self, AkitaError> {
        if source_count == 0 {
            return Err(invalid("empty query endpoint roster"));
        }
        let mut events = Vec::new();
        let mut ordinal = 0;
        request(&mut events, &mut ordinal, site(20, 0, 0))?;
        sumcheck(
            &mut events,
            &mut ordinal,
            3,
            SumcheckShape::new(num_vars, 2)?,
        )?;
        fields(
            &mut events,
            site(21, 0, 0),
            ProtocolMessageKind::ProofAtoms,
            source_count,
        )?;
        Ok(Self {
            registry: registry(events, max_context_bytes, installation_index)?,
            rounds: num_vars,
            endpoints: source_count,
        })
    }
    /// Maximum proof-body bytes admitted by this exact installed grammar.
    ///
    /// # Errors
    /// Rejects a byte count not representable by the host.
    pub fn proof_size_bound(&self) -> Result<usize, AkitaError> {
        proof_size(&self.registry)
    }
    /// Bound the literal sealed ledger and its context frame.
    ///
    /// # Errors
    /// Rejects arithmetic overflow.
    pub fn context_size_bound(max_seal_bytes: usize) -> Result<usize, AkitaError> {
        QUERY_DOMAIN
            .len()
            .checked_add(8)
            .and_then(|n| n.checked_add(max_seal_bytes))
            .ok_or_else(|| invalid("query context bound overflow"))
    }
    /// Bound the complete header/grammar/bodies/results for a bounded seal.
    ///
    /// # Errors
    /// Rejects arithmetic overflow or an unsupported grammar event.
    pub fn literal_history_size_bound(&self, max_seal_bytes: usize) -> Result<usize, AkitaError> {
        history_size(
            &self.registry,
            Self::context_size_bound(max_seal_bytes)?,
            self.proof_size_bound()?,
            1,
            self.rounds
                .checked_add(1)
                .ok_or_else(|| invalid("query draws overflow"))?,
        )
    }
    fn context(&self, ledger: &mut QueryLedger<K>) -> Result<Vec<u8>, AkitaError> {
        if self.endpoints != ledger.sources().len() || self.rounds != ledger.limits.common_num_vars
        {
            return Err(invalid("query program source count or dimension"));
        }
        let sealed = ledger.seal()?;
        let mut context = QUERY_DOMAIN.to_vec();
        bytes(&mut context, &sealed)?;
        Ok(context)
    }
}

struct ProverAdapter {
    state: ProverChannel,
    draws: Vec<K>,
}
impl ProverAdapter {
    fn draw(&mut self, id: ProtocolSiteId) -> Result<K, AkitaError> {
        let value = self.state.field_challenge_at::<K>(id)?;
        self.draws.push(value);
        Ok(value)
    }
    fn fields(&mut self, id: ProtocolSiteId, values: &[K]) -> Result<(), AkitaError> {
        self.state.context(field_group_record::<K>(
            id,
            ProtocolMessageKind::ProofAtoms,
            values.len(),
        )?)?;
        for &value in values {
            send_field(&mut self.state, value)?;
        }
        Ok(())
    }
}
impl SumcheckProverChannel<K> for ProverAdapter {
    fn state_mut(&mut self) -> &mut ProverChannel {
        &mut self.state
    }
    fn sumcheck_site(&self, invocation: u32, round: u32, role: SumcheckRole) -> ProtocolSiteId {
        sc_site(invocation, round, role)
    }
    fn round_challenge(&mut self, invocation: u32, round: u32) -> Result<K, AkitaError> {
        self.draw(sc_site(invocation, round, SumcheckRole::Challenge))
    }
}
struct VerifierAdapter<'a> {
    state: VerifierChannel<'a>,
    draws: Vec<K>,
}
impl VerifierAdapter<'_> {
    fn draw(&mut self, id: ProtocolSiteId) -> Result<K, AkitaError> {
        let value = self.state.field_challenge_at::<K>(id)?;
        self.draws.push(value);
        Ok(value)
    }
    fn fields(&mut self, id: ProtocolSiteId, count: usize) -> Result<Vec<K>, AkitaError> {
        self.state.context(field_group_record::<K>(
            id,
            ProtocolMessageKind::ProofAtoms,
            count,
        )?)?;
        let mut output = Vec::new();
        output
            .try_reserve_exact(count)
            .map_err(|_| invalid("endpoint allocation"))?;
        for _ in 0..count {
            output.push(receive_field::<K>(&mut self.state)?);
        }
        Ok(output)
    }
}
impl<'a> SumcheckVerifierChannel<'a, K> for VerifierAdapter<'a> {
    fn state_mut(&mut self) -> &mut VerifierChannel<'a> {
        &mut self.state
    }
    fn sumcheck_site(&self, invocation: u32, round: u32, role: SumcheckRole) -> ProtocolSiteId {
        sc_site(invocation, round, role)
    }
    fn round_challenge(&mut self, invocation: u32, round: u32) -> Result<K, AkitaError> {
        self.draw(sc_site(invocation, round, SumcheckRole::Challenge))
    }
}

/// Full public typed history, not a digest, seed or unauthenticated recipe list.
/// Grammar contains every full request; proof/public bodies and whole completed
/// scalar results are framed separately in their exact canonical order.
#[expect(
    clippy::as_conversions,
    reason = "fixed indexed operation/result discriminants fit canonical u32 tags"
)]
fn history(
    program: &InstalledRegistry,
    context: &[u8],
    proof: &[u8],
    public_claims: &[K],
    draws: &[K],
) -> Result<Vec<u8>, AkitaError> {
    let mut output = b"akita/candidate1/literal-prefix-history/v1".to_vec();
    // This is the complete registered header of InstalledContext::new,
    // including the explicitly empty session, not a prefix digest.
    bytes(&mut output, INDEXED_PROVIDER.as_bytes())?;
    output.extend_from_slice(&PROTOCOL_VERSION.to_le_bytes());
    output.extend_from_slice(&program.index().to_le_bytes());
    bytes(&mut output, &[])?;
    bytes(&mut output, context)?;
    size(&mut output, program.events().len())?;
    for event in program.events() {
        match event {
            InstalledEvent::Message(record) => {
                output.push(0);
                output.extend_from_slice(&record.version.to_le_bytes());
                output.extend_from_slice(&record.domain);
                output.extend_from_slice(&record.site_id);
                for value in [
                    u64::from(record.kind),
                    record.atom_count,
                    record.encoded_bytes,
                    record.challenge_bytes,
                ] {
                    output.extend_from_slice(&value.to_le_bytes());
                }
            }
            InstalledEvent::Request(spec) => {
                output.push(1);
                output.extend_from_slice(&(spec.request.operation() as u32).to_le_bytes());
                output.extend_from_slice(&spec.request.ordinal().to_le_bytes());
                output.extend_from_slice(&spec.request.site().to_bytes());
                bytes(&mut output, spec.request.descriptor())?;
                output.extend_from_slice(&spec.request.raw_len().to_le_bytes());
                output.extend_from_slice(&(spec.result_kind as u32).to_le_bytes());
                output.extend_from_slice(&spec.result_len.to_le_bytes());
            }
            InstalledEvent::BoundedBytes { .. } => {
                return Err(invalid("unexpected variable prefix event"))
            }
        }
    }
    bytes(&mut output, proof)?;
    scalars(&mut output, public_claims)?;
    scalars(&mut output, draws)?;
    Ok(output)
}
#[expect(
    clippy::as_conversions,
    reason = "fixed message kind discriminants fit the canonical u32 grammar tag"
)]
fn proof_size(program: &InstalledRegistry) -> Result<usize, AkitaError> {
    program.events().iter().try_fold(0usize, |size, event| {
        let bytes = match event {
            InstalledEvent::Message(record)
                if record.kind != ProtocolMessageKind::PublicValue as u32 =>
            {
                usize::try_from(record.encoded_bytes)
                    .map_err(|_| invalid("prefix proof size overflow"))?
            }
            _ => 0,
        };
        size.checked_add(bytes)
            .ok_or_else(|| invalid("prefix proof size overflow"))
    })
}
fn history_size(
    program: &InstalledRegistry,
    context: usize,
    proof: usize,
    public: usize,
    draws: usize,
) -> Result<usize, AkitaError> {
    let fixed = history(program, &[], &[], &[], &[])?.len();
    public
        .checked_add(draws)
        .and_then(|n| n.checked_mul(K::NUM_BYTES))
        .and_then(|n| n.checked_add(fixed))
        .and_then(|n| n.checked_add(context))
        .and_then(|n| n.checked_add(proof))
        .ok_or_else(|| invalid("prefix history size overflow"))
}
fn size(out: &mut Vec<u8>, n: usize) -> Result<(), AkitaError> {
    out.extend_from_slice(
        &u64::try_from(n)
            .map_err(|_| invalid("prefix length overflow"))?
            .to_le_bytes(),
    );
    Ok(())
}
fn bytes(out: &mut Vec<u8>, b: &[u8]) -> Result<(), AkitaError> {
    size(out, b.len())?;
    out.extend_from_slice(b);
    Ok(())
}
fn scalars(out: &mut Vec<u8>, values: &[K]) -> Result<(), AkitaError> {
    size(out, values.len())?;
    for value in values {
        let mut atom = [0u8; 16];
        value.to_bytes_le(&mut atom);
        out.extend_from_slice(&atom);
    }
    Ok(())
}
fn endpoints(
    ledger: &QueryLedger<K>,
    slots: &[usize],
    values: &[K],
) -> Result<Vec<SourceEndpoint<K>>, AkitaError> {
    if slots.len() != values.len() {
        return Err(invalid("prefix endpoint arity"));
    }
    slots
        .iter()
        .zip(values)
        .map(|(&slot, &value)| {
            let source = ledger
                .sources()
                .get(slot)
                .ok_or_else(|| invalid("prefix foreign endpoint"))?;
            Ok(SourceEndpoint {
                source: slot,
                local_ownership: source.local_ownership,
                component: source.component,
                value,
            })
        })
        .collect()
}

/// Completed range proof and its full typed public channel history.
pub struct RangeProof {
    /// Canonical proof atoms; excludes independently derived public claims.
    pub proof: Vec<u8>,
    /// Full registered header and completed events, never a digest substitute.
    pub literal_history: Vec<u8>,
}
/// Prove the one global original-plane range before the formal query is sealed.
///
/// Append all original RAM endpoints only after the quartic/cubic links pass.
///
/// # Errors
/// Rejects foreign sources, malformed shapes, failed relations and exhausted
/// installed limits. No finished native PCS proof is produced here.
pub fn prove_range(
    program: &RangeProgram,
    ledger: &mut QueryLedger<K>,
    range: &GlobalRamRange,
    provider: &dyn SourceProvider<K>,
) -> Result<RangeProof, AkitaError> {
    let context = program.context(ledger, range)?;
    let installed = InstalledContext::new(&program.registry, &[], &context)?;
    let mut channel = ProverAdapter {
        state: new_prover_channel(&installed)?,
        draws: Vec::new(),
    };
    let anchor = (0..program.rounds)
        .map(|i| channel.draw(site(10, narrow(i)?, 0)))
        .collect::<Result<Vec<_>, _>>()?;
    let shape = SumcheckShape::new(program.rounds, 4)?;
    let mut product = range.product_kernel(provider, anchor)?;
    let (product_point, product_terminal) =
        prove_eq_factored_sumcheck::<K, K, _, _>(&mut product, &mut channel, shape, 0)?;
    let children: [K; 4] = product
        .helper_endpoint()?
        .try_into()
        .map_err(|_| invalid("range product children"))?;
    channel.fields(site(11, 0, 0), &children)?;
    let children = range.verify_product_endpoint(product_point, product_terminal, children)?;
    let gamma = channel.draw(site(12, 0, 0))?;
    let batched = children.batched_claim(gamma);
    let mut leaf_kernel = range.leaf_kernel(provider, &children, gamma)?;
    let (leaf_point, leaf_terminal) =
        prove_eq_factored_sumcheck::<K, K, _, _>(&mut leaf_kernel, &mut channel, shape, 1)?;
    let t_u = *leaf_kernel
        .helper_endpoint()?
        .first()
        .ok_or_else(|| invalid("range leaf"))?;
    channel.fields(site(13, 0, 0), &[t_u])?;
    let leaf = range.verify_leaf_endpoint(leaf_point, gamma, leaf_terminal, t_u)?;
    let mut link = range.link_kernel(provider, &leaf)?;
    let (point, terminal) = prove_sumcheck::<K, K, _, _>(
        &mut link,
        &mut channel,
        SumcheckShape::new(program.rounds, 3)?,
        2,
    )?;
    let endpoints = range.source_endpoints(provider, &point)?;
    channel.fields(
        site(14, 0, 0),
        &endpoints.iter().map(|e| e.value).collect::<Vec<_>>(),
    )?;
    range.verify_link_endpoint(&leaf, &point, terminal, &endpoints)?;
    let proof = channel.state.finish()?;
    let literal_history = history(
        &program.registry,
        &context,
        &proof,
        &[K::zero(), batched, t_u],
        &channel.draws,
    )?;
    ledger.bind_evidence(&literal_history)?;
    range.append_endpoints(ledger, &leaf, &point, terminal, &endpoints)?;
    Ok(RangeProof {
        proof,
        literal_history,
    })
}
/// Independently replay the range and append every authenticated raw endpoint.
///
/// Public claims, challenges, aliases and weights are derived from the trusted
/// range/ledger; proof bytes cannot supply them. Requires exact proof EOF.
///
/// # Errors
/// Rejects invalid arithmetic, foreign rosters, malformed atoms or unused bytes.
pub fn verify_range(
    program: &RangeProgram,
    ledger: &mut QueryLedger<K>,
    range: &GlobalRamRange,
    proof: &[u8],
) -> Result<Vec<u8>, AkitaError> {
    let context = program.context(ledger, range)?;
    let installed = InstalledContext::new(&program.registry, &[], &context)?;
    let mut channel = VerifierAdapter {
        state: new_verifier_channel(&installed, proof)?,
        draws: Vec::new(),
    };
    let anchor = (0..program.rounds)
        .map(|i| channel.draw(site(10, narrow(i)?, 0)))
        .collect::<Result<Vec<_>, _>>()?;
    let shape = SumcheckShape::new(program.rounds, 4)?;
    let product =
        verify_eq_factored_sumcheck_rounds::<K, K, _>(&anchor, K::zero(), shape, &mut channel, 0)?;
    let children = channel
        .fields(site(11, 0, 0), 4)?
        .try_into()
        .map_err(|_| invalid("range children arity"))?;
    let children =
        range.verify_product_endpoint(product.challenges, product.output_claim, children)?;
    let gamma = channel.draw(site(12, 0, 0))?;
    let batched = children.batched_claim(gamma);
    let leaf_result = verify_eq_factored_sumcheck_rounds::<K, K, _>(
        children.point(),
        batched,
        shape,
        &mut channel,
        1,
    )?;
    let t_u = *channel
        .fields(site(13, 0, 0), 1)?
        .first()
        .ok_or_else(|| invalid("range leaf arity"))?;
    let leaf =
        range.verify_leaf_endpoint(leaf_result.challenges, gamma, leaf_result.output_claim, t_u)?;
    let link = verify_sumcheck_rounds::<K, K, _>(
        &mut channel,
        2,
        t_u,
        SumcheckShape::new(program.rounds, 3)?,
    )?;
    let values = channel.fields(site(14, 0, 0), program.endpoints)?;
    let endpoints = endpoints(ledger, range.ram_slots(), &values)?;
    range.verify_link_endpoint(&leaf, &link.challenges, link.output_claim, &endpoints)?;
    channel.state.finish()?;
    let literal_history = history(
        &program.registry,
        &context,
        proof,
        &[K::zero(), batched, t_u],
        &channel.draws,
    )?;
    ledger.bind_evidence(&literal_history)?;
    range.append_endpoints(
        ledger,
        &leaf,
        &link.challenges,
        link.output_claim,
        &endpoints,
    )?;
    Ok(literal_history)
}

/// Public result of a checked degree-two query; still requires native PCS.
pub struct QueryReplay {
    /// Independent query-index draw sampled only after the ledger is sealed.
    pub a: K,
    /// Checked sumcheck terminal value.
    pub terminal: K,
    /// Low-first common-cube point, with cylindrical source projections.
    pub point: Vec<K>,
    /// All original endpoints in immutable source order, including zero weights.
    pub endpoints: Vec<SourceEndpoint<K>>,
    /// Full typed header, grammar, messages and completed scalar results.
    pub literal_history: Vec<u8>,
}
/// Completed degree-two query proof and its independently reproducible result.
pub struct QueryProof {
    /// Canonical proof atoms in the fixed installed order.
    pub proof: Vec<u8>,
    /// Checked public endpoints/history; not a native PCS validity certificate.
    pub replay: QueryReplay,
}
/// Seal the growing ledger, sample `a`, and prove its degree-two contraction.
///
/// All endpoints are emitted, even when their combined public weight is zero.
///
/// # Errors
/// Rejects invalid recipes, foreign providers, shape mismatches or failed checks.
pub fn prove_query(
    program: &QueryProgram,
    ledger: &mut QueryLedger<K>,
    provider: &dyn SourceProvider<K>,
) -> Result<QueryProof, AkitaError> {
    let context = program.context(ledger)?;
    let installed = InstalledContext::new(&program.registry, &[], &context)?;
    let mut channel = ProverAdapter {
        state: new_prover_channel(&installed)?,
        draws: Vec::new(),
    };
    let a = channel.draw(site(20, 0, 0))?;
    let mut kernel = WeightedQueryKernel::new(ledger, provider, a)?;
    let (point, terminal) = prove_sumcheck::<K, K, _, _>(
        &mut kernel,
        &mut channel,
        SumcheckShape::new(program.rounds, 2)?,
        3,
    )?;
    let endpoints = kernel.endpoints()?;
    channel.fields(
        site(21, 0, 0),
        &endpoints.iter().map(|e| e.value).collect::<Vec<_>>(),
    )?;
    ledger.verify_query_endpoint(a, &point, &endpoints, terminal)?;
    let proof = channel.state.finish()?;
    let literal_history = history(
        &program.registry,
        &context,
        &proof,
        &[ledger.y_at(a)],
        &channel.draws,
    )?;
    Ok(QueryProof {
        proof,
        replay: QueryReplay {
            a,
            terminal,
            point,
            endpoints,
            literal_history,
        },
    })
}
/// Seal the independently rebuilt ledger and replay the degree-two query.
///
/// This derives recipes/weights/challenges and checks the complete endpoint
/// roster and EOF; the caller must discharge these endpoints with native PCS.
///
/// # Errors
/// Rejects invalid relations, malformed atoms, foreign shapes or trailing bytes.
pub fn verify_query(
    program: &QueryProgram,
    ledger: &mut QueryLedger<K>,
    proof: &[u8],
) -> Result<QueryReplay, AkitaError> {
    let context = program.context(ledger)?;
    let installed = InstalledContext::new(&program.registry, &[], &context)?;
    let mut channel = VerifierAdapter {
        state: new_verifier_channel(&installed, proof)?,
        draws: Vec::new(),
    };
    let a = channel.draw(site(20, 0, 0))?;
    let input = ledger.y_at(a);
    let result = verify_sumcheck_rounds::<K, K, _>(
        &mut channel,
        3,
        input,
        SumcheckShape::new(program.rounds, 2)?,
    )?;
    let values = channel.fields(site(21, 0, 0), program.endpoints)?;
    let endpoints = endpoints(ledger, &(0..program.endpoints).collect::<Vec<_>>(), &values)?;
    ledger.verify_query_endpoint(a, &result.challenges, &endpoints, result.output_claim)?;
    channel.state.finish()?;
    let literal_history = history(&program.registry, &context, proof, &[input], &channel.draws)?;
    Ok(QueryReplay {
        a,
        terminal: result.output_claim,
        point: result.challenges,
        endpoints,
        literal_history,
    })
}

#[cfg(test)]
mod tests {
    #![expect(
        clippy::unwrap_used,
        reason = "tests assert trusted grammar shapes and canonical byte bounds"
    )]
    use super::*;
    use crate::accumulation::{
        ApprovedSource, LedgerLimits, QueryRecipe, QueryTerm, ScalarClaim, SourceView,
    };
    use jolt_field::{One, Ring};

    #[test]
    fn fixed_grammars_count_every_round_body_and_original_endpoint() {
        let range = RangeProgram::from_shape(5, 3, 4096, 7).unwrap();
        assert_eq!(
            range.proof_size_bound().unwrap(),
            (11 * 5 + 5 + 3) * K::NUM_BYTES
        );
        let query = QueryProgram::new(3, 2, 4096, 8).unwrap();
        assert_eq!(
            query.proof_size_bound().unwrap(),
            (2 * 3 + 2) * K::NUM_BYTES
        );
        assert!(RangeProgram::from_shape(5, 0, 4096, 7).is_err());
        assert!(QueryProgram::new(3, 0, 4096, 8).is_err());
        assert!(QueryProgram::context_size_bound(usize::MAX).is_err());
    }

    #[test]
    fn literal_bound_includes_full_registered_header_and_whole_results() {
        let program = QueryProgram::new(3, 2, 4096, 8).unwrap();
        let seal = [9, 8, 7];
        let mut context = QUERY_DOMAIN.to_vec();
        bytes(&mut context, &seal).unwrap();
        let proof = vec![0; program.proof_size_bound().unwrap()];
        let literal = history(
            &program.registry,
            &context,
            &proof,
            &[K::zero()],
            &[K::zero(); 4],
        )
        .unwrap();
        assert_eq!(
            literal.len(),
            program.literal_history_size_bound(seal.len()).unwrap()
        );
        let other = QueryProgram::new(3, 2, 4096, 9).unwrap();
        assert_ne!(
            literal,
            history(
                &other.registry,
                &context,
                &proof,
                &[K::zero()],
                &[K::zero(); 4]
            )
            .unwrap()
        );
        assert!(literal
            .windows(INDEXED_PROVIDER.len())
            .any(|value| value == INDEXED_PROVIDER.as_bytes()));
        assert!(literal.windows(context.len()).any(|value| value == context));
    }

    #[test]
    fn foreign_query_dimension_rejects_before_sealing_or_sampling() {
        let limits = LedgerLimits {
            claim_horizon: 1,
            max_sources: 1,
            max_terms_per_claim: 1,
            max_recipe_terms: 1,
            max_evidence_bytes: 256,
            common_num_vars: 2,
            max_cube_cells: 4,
        };
        let mut ledger = QueryLedger::new(
            vec![ApprovedSource {
                local_ownership: None,
                component: 0,
                canonical_identity: vec![1],
                view: SourceView::Decoded { num_vars: 2 },
            }],
            limits,
            vec![2],
        )
        .unwrap();
        ledger
            .append(ScalarClaim {
                exponent: 0,
                value: K::zero(),
                offset: K::zero(),
                terms: vec![QueryTerm {
                    source: 0,
                    coefficient: K::one(),
                    recipe: QueryRecipe::Eq {
                        point: vec![K::from_u64(0), K::from_u64(1)],
                    },
                }],
            })
            .unwrap();
        let foreign = QueryProgram::new(1, 1, 4096, 8).unwrap();
        assert!(verify_query(&foreign, &mut ledger, &[]).is_err());
        assert!(!ledger.is_sealed());
    }
}
