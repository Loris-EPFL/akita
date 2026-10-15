//! Actual original Dense ordinary evaluations accumulated before any PCS proof.
//!
//! Run `cargo run --release -p akita-pcs --example accumulation_relation_benchmark
//! -- --segments 256` (also 1, 2, 3). This is K/K, decoded nv14, fixed source
//! seeds 0xdeadbeef + segment*0x9e3779b97f4a7c15 and point seed 0xcafebabe.
//! The anchor is independent public-only query replay followed by the complete
//! native root, recursive suffix and actual terminal verifier on every original
//! commitment. It is not a VM relation/global RAM test, ZK claim, or recursive
//! streaming baseline; parity must be measured separately.
//!
//! All timings are actual wall time. `benchmark_prover_ms` is query plus native
//! proving, excluding commit/setup. Receipt bytes include literal query evidence
//! and the one full native proof. Linux peak RSS covers the whole process,
//! including retained sources, setup and mandatory mutation checks.

#![allow(missing_docs)]
#[path = "support/aqra.rs"]
mod aqra;
#[path = "support/workspace_schedules.rs"]
mod workspace_schedules;

use akita_algebra::poly::multilinear_eval;
use akita_config::{
    policy_of, transcript_instance_descriptor, CommitmentConfig, NativeRegistryResources,
    TrustedScheduleCatalog,
};
use akita_cpu_backend::{CpuBackend, DensePoly, GroupContext};
use akita_error::AkitaError;
use akita_params::{
    BasisMode, CommittedGroupBatchProfile, GroupCommitPhaseParams, PolynomialGroupLayout,
    ScheduleLookupKey, SourceOpeningView,
};
use akita_pcs::accumulation::protocol::{prove_query, verify_query, QueryProgram};
use akita_pcs::prepare_native_proof_context;
use akita_pcs::{
    accumulation::{
        ApprovedSource, CpuSourceReader, LedgerLimits, QueryLedger, QueryRecipe, QueryTerm,
        ScalarClaim, SourceOwnership, SourceView,
    },
    AkitaCommitmentScheme,
};
use akita_planner::{plan_frozen_source_roster, FrozenRosterRequest};
use akita_prover::SelectedProverOpeningData;
use akita_schedules::ValidatedScheduleCatalog;
use akita_types::{CommittedGroup, GroupBatchStatement, OpeningClaims, PolynomialGroupClaims};
use akita_verifier::AkitaVerifier;
use aqra::{Config, Receipt, F, NUM_VARS, SESSION};
use jolt_field::{CanonicalBytes, One, Zero};
use std::{
    io,
    time::{Duration, Instant},
};

#[derive(Clone)]
struct PublicSource {
    segment: usize,
    commitment: CommittedGroup<F>,
    evaluation: F,
    view: SourceOpeningView,
}
struct Installation {
    public: Vec<PublicSource>,
    point: Vec<F>,
    context: Vec<u8>,
    setup: akita_types::AkitaSetupDescriptor,
    schedules: TrustedScheduleCatalog<Config>,
    native_cap: usize,
    selection: akita_params::OpeningScheduleSelection,
    query_program: QueryProgram,
    query_cap: usize,
    history_cap: usize,
}
fn invalid() -> AkitaError {
    AkitaError::InvalidProof
}
fn require(value: bool) -> Result<(), AkitaError> {
    if value {
        Ok(())
    } else {
        Err(invalid())
    }
}
fn check_original(profile: &GroupCommitPhaseParams) -> Result<(), AkitaError> {
    require(
        profile.version == 4
            && profile.group == PolynomialGroupLayout::singleton(NUM_VARS)
            && profile.inner.digits.log_basis == 13
            && profile.inner.digits.num_digits == 10
            && profile.inner.matrix.ring_dimension() == 512
            && profile.inner.matrix.output_rank() == 1
            && profile.inner.matrix.input_width() == 40
            && profile.inner.matrix.coeff_linf_bound() == Some(427_818_804)
            && profile.outer.digits.log_basis == 3
            && profile.outer.digits.num_digits == 43
            && profile.outer.matrix.ring_dimension() == 64
            && profile.outer.matrix.output_rank() == 1
            && profile.outer.matrix.input_width() == 344
            && profile.outer.matrix.coeff_linf_bound() == 7
            && profile.outer_slice_count.get() == 8
            && profile.blocks.live_blocks == 8,
    )?;
    require(
        profile
            .outer_slice_count
            .complete_source_coefficients(1, 64)?
            == 512,
    )
}
fn append_field(bytes: &mut Vec<u8>, value: F) {
    let mut encoded = [0u8; 16];
    value.to_bytes_le(&mut encoded);
    bytes.extend_from_slice(&encoded);
}
fn source_identity(source: &PublicSource, index: usize) -> Result<Vec<u8>, AkitaError> {
    let mut bytes = b"akita/aqra/original-dense-source/v8".to_vec();
    bytes.extend_from_slice(&u64::try_from(index).map_err(|_| invalid())?.to_le_bytes());
    bytes.extend_from_slice(
        &u64::try_from(source.segment)
            .map_err(|_| invalid())?
            .to_le_bytes(),
    );
    source
        .commitment
        .profile()
        .append_descriptor_bytes(&mut bytes);
    bytes.push(source.view.tag());
    bytes.extend_from_slice(&0u64.to_le_bytes()); // exact original component
    let coefficients = source.commitment.commitment().rows().coeffs();
    bytes.extend_from_slice(
        &u64::try_from(coefficients.len())
            .map_err(|_| invalid())?
            .to_le_bytes(),
    );
    for &value in coefficients {
        append_field(&mut bytes, value);
    }
    Ok(bytes)
}
fn ledger(
    public: &[PublicSource],
    point: &[F],
    context: &[u8],
    ids: Option<&[SourceOwnership]>,
) -> Result<QueryLedger<F>, AkitaError> {
    require(
        !public.is_empty()
            && point.len() == NUM_VARS
            && ids.is_none_or(|ids| ids.len() == public.len()),
    )?;
    let mut sources = Vec::with_capacity(public.len());
    for (index, source) in public.iter().enumerate() {
        check_original(source.commitment.profile())?;
        require(source.view == SourceOpeningView::Decoded)?;
        sources.push(ApprovedSource {
            local_ownership: match ids {
                Some(ids) => Some(*ids.get(index).ok_or_else(invalid)?),
                None => None,
            },
            component: 0,
            canonical_identity: source_identity(source, index)?,
            view: SourceView::Decoded { num_vars: NUM_VARS },
        });
    }
    let limits = LedgerLimits {
        claim_horizon: public.len(),
        max_sources: public.len(),
        max_terms_per_claim: 1,
        max_recipe_terms: 1,
        max_evidence_bytes: context.len().checked_add(8).ok_or_else(invalid)?,
        common_num_vars: NUM_VARS,
        max_cube_cells: 1usize << NUM_VARS,
    };
    // Identity length is also checked against the evidence cap by the ledger.
    let identity_cap = sources
        .iter()
        .map(|s| s.canonical_identity.len())
        .max()
        .ok_or_else(invalid)?;
    let limits = LedgerLimits {
        max_evidence_bytes: limits.max_evidence_bytes.max(identity_cap),
        ..limits
    };
    let mut ledger = QueryLedger::new(sources, limits, context.to_vec())?;
    for (exponent, source) in public.iter().enumerate() {
        ledger.append(ScalarClaim {
            exponent,
            value: source.evaluation,
            offset: F::zero(),
            terms: vec![QueryTerm {
                source: exponent,
                coefficient: F::one(),
                recipe: QueryRecipe::Eq {
                    point: point.to_vec(),
                },
            }],
        })?;
    }
    require(ledger.claims().len() == public.len())?;
    Ok(ledger)
}
fn opening_claims<'a>(
    public: &'a [PublicSource],
    point: &[F],
    values: &[F],
) -> Result<OpeningClaims<'static, F, &'a CommittedGroup<F>>, AkitaError> {
    require(public.len() == values.len() && point.len() == NUM_VARS)?;
    OpeningClaims::from_groups(
        public
            .iter()
            .zip(values)
            .map(|(source, &value)| {
                PolynomialGroupClaims::new(point.to_vec(), vec![value], &source.commitment)
            })
            .collect::<Result<Vec<_>, _>>()?,
    )
}
/// This verifier has no prover source coefficients, private handles or backend.
fn verify(
    installation: &Installation,
    verifier: &AkitaVerifier<Config>,
    bytes: &[u8],
) -> Result<(), AkitaError> {
    let mut expected = ledger(
        &installation.public,
        &installation.point,
        &installation.context,
        None,
    )?;
    let seal = expected.canonical_bytes()?;
    let receipt = Receipt::decode(
        bytes,
        seal.len(),
        installation.query_cap,
        installation.history_cap,
        installation.native_cap,
    )?;
    require(receipt.seal == seal)?;
    let replay = verify_query(&installation.query_program, &mut expected, receipt.query)?;
    require(receipt.query_history == replay.literal_history)?;
    let values = replay
        .endpoints
        .iter()
        .map(|endpoint| endpoint.value)
        .collect::<Vec<_>>();
    let statement = GroupBatchStatement::new(
        installation.selection,
        opening_claims(&installation.public, &replay.point, &values)?,
    )?;
    let context = prepare_native_proof_context::<Config>(
        &installation.setup,
        &installation.schedules,
        &statement,
        SESSION,
        &replay.literal_history,
        BasisMode::Lagrange,
        2,
    )?;
    verifier.batched_verify(
        receipt.native,
        SESSION,
        context.registry(),
        context.augmentation(),
        statement,
        BasisMode::Lagrange,
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(segments) = aqra::parse_segments()? else {
        return Ok(());
    };
    let setup_started = Instant::now();
    #[cfg(feature = "parallel")]
    rayon::ThreadPoolBuilder::new()
        .num_threads(
            if std::env::var_os("AKITA_PARALLEL").is_some_and(|v| v == "0") {
                1
            } else {
                0
            },
        )
        .stack_size(64 * 1024 * 1024)
        .build_global()?;
    // The checked-in singleton is the original producer only. The consuming
    // H-original root and EVERY suffix/terminal are generated and audited here.
    let producer = workspace_schedules::load_workspace_scheme::<Config>()?;
    let singleton = producer
        .schedules()
        .resolve_key(&ScheduleLookupKey::single(
            PolynomialGroupLayout::singleton(NUM_VARS),
        ))?;
    let original = singleton.profiles().final_group;
    check_original(&original)?;
    let profiles = CommittedGroupBatchProfile {
        final_group: original,
        precommitteds: vec![original; segments - 1],
    };
    let views = vec![SourceOpeningView::Decoded; segments];
    let source_contract = Config::committed_source_contract()?;
    let contracts = vec![source_contract; segments - 1];
    let policy = policy_of::<Config>();
    let plan = plan_frozen_source_roster(
        FrozenRosterRequest {
            profiles: &profiles,
            final_source_contract: source_contract,
            precommitted_source_contracts: &contracts,
            source_views: &views,
            query_equation_count: segments,
            policy: &policy,
        },
        Config::ring_challenge_config,
    )?;
    require(
        plan.ledger.groups.len() == segments
            && plan.ledger.groups.iter().all(|g| {
                g.profile == original
                    && g.carrier_dimension == 64
                    && g.packing_h == 8
                    && g.response_depth == 7
                    && g.original_f_source_coefficients == 512
            }),
    )?;
    let mut rows = vec![(profiles.clone(), plan.row.schedule().clone())];
    if segments > 1 {
        rows.push((singleton.profiles().clone(), singleton.schedule().clone()));
    }
    let catalog = ValidatedScheduleCatalog::try_new(
        Config::schedule_family_name(),
        rows,
        &policy,
        Config::ring_challenge_config,
    )?;
    let scheme = AkitaCommitmentScheme::<Config>::new(TrustedScheduleCatalog::new(catalog)?);
    let setup = scheme.setup_prover(NUM_VARS, segments)?;
    let backend = CpuBackend::new(setup.expanded.clone())?;
    let row = scheme.schedules().resolve_profiles(&profiles, &views)?;
    let selection = row.selection();
    let (_, descriptor) = transcript_instance_descriptor::<F, Config>(
        setup.expanded.descriptor(),
        &row.opening_layout()?,
        selection,
        row.schedule(),
        BasisMode::Lagrange,
    )?;
    let verifier_setup = scheme.setup_verifier(&setup)?;
    let verifier_setup_fields = verifier_setup.expanded().descriptor().num_field_elements;
    let verifier = scheme.verifier(verifier_setup)?;
    let setup_time = setup_started.elapsed();
    let point = aqra::opening_point();
    let mut public = Vec::with_capacity(segments);
    let mut handles = Vec::with_capacity(segments);
    let mut commit_time = Duration::ZERO;
    for segment in 0..segments {
        let evaluations = aqra::make_dense_evals(segment);
        let evaluation = multilinear_eval(&evaluations, &point)?;
        let source =
            backend.import_source(vec![DensePoly::from_field_evals(NUM_VARS, evaluations)?])?;
        let started = Instant::now();
        let committed = backend.commit(
            scheme.schedules(),
            &source,
            GroupContext::scheduler_without_precommitted_groups(),
        )?;
        commit_time = commit_time
            .checked_add(started.elapsed())
            .ok_or_else(invalid)?;
        require(*committed.committed_group.profile() == original)?;
        public.push(PublicSource {
            segment,
            commitment: committed.committed_group,
            evaluation,
            view: SourceOpeningView::Decoded,
        });
        handles.push(committed.private_handle);
    }
    let mut context = b"akita/aqra/fixed-dense-workload/v8".to_vec();
    for value in [
        segments as u64,
        NUM_VARS as u64,
        0xdead_beef,
        0x9e37_79b9_7f4a_7c15,
        0xcafe_babe,
    ] {
        context.extend_from_slice(&value.to_le_bytes());
    }
    aqra::frame(&mut context, &descriptor)?;
    let readers = handles
        .iter()
        .map(|handle| backend.decoded_dense_component_reader(handle, 0))
        .collect::<Result<Vec<_>, _>>()?;
    let ids = readers
        .iter()
        .map(|reader| SourceOwnership::RetainedDenseCommitment {
            commitment_id: reader.commitment_id(),
            backend_id: reader.backend_id(),
            setup_digest: reader.setup_digest(),
        })
        .collect::<Vec<_>>();
    let provider = readers
        .into_iter()
        .map(CpuSourceReader::Dense)
        .collect::<Vec<_>>();
    let query_started = Instant::now();
    let mut accumulated = ledger(&public, &point, &context, Some(&ids))?;
    let seal = accumulated.canonical_bytes()?;
    let query_program = QueryProgram::new(
        NUM_VARS,
        segments,
        QueryProgram::context_size_bound(seal.len())?,
        1,
    )?;
    let query_cap = query_program.proof_size_bound()?;
    let history_cap = query_program.literal_history_size_bound(seal.len())?;
    let query = prove_query(&query_program, &mut accumulated, &provider)?;
    let history = &query.replay.literal_history;
    let values = query
        .replay
        .endpoints
        .iter()
        .map(|endpoint| endpoint.value)
        .collect::<Vec<_>>();
    let native_context = {
        let statement = GroupBatchStatement::new(
            selection,
            opening_claims(&public, &query.replay.point, &values)?,
        )?;
        prepare_native_proof_context::<Config>(
            setup.expanded.descriptor(),
            scheme.schedules(),
            &statement,
            SESSION,
            history,
            BasisMode::Lagrange,
            2,
        )?
    };
    let resources =
        NativeRegistryResources::from_trusted_bounds(native_context.registry().events(), 0, 0)?;
    let native_cap = usize::try_from(resources.limits.max_proof_bytes)?;
    let installation = Installation {
        public,
        point,
        context,
        setup: setup.expanded.descriptor().clone(),
        schedules: scheme.schedules().clone(),
        native_cap,
        selection,
        query_program,
        query_cap,
        history_cap,
    };
    let query_time = query_started.elapsed();
    // Drop borrowed adapters, not the N actual retained handles.
    drop(provider);
    let public_statement_bytes = seal.len();
    let claims = opening_claims(&installation.public, &query.replay.point, &values)?
        .map_groups(Clone::clone)?;
    let opening = SelectedProverOpeningData::from_committed_claims::<Config>(
        claims,
        handles,
        scheme.schedules(),
    )?;
    let native_started = Instant::now();
    let native = scheme.batched_prove(
        &setup,
        opening,
        &backend,
        SESSION,
        native_context.registry(),
        native_context.augmentation(),
        BasisMode::Lagrange,
    )?;
    let native_time = native_started.elapsed();
    let native_bytes = native.len();
    let receipt = Receipt {
        seal,
        query: query.proof,
        query_history: query.replay.literal_history,
        native,
    };
    let encoded = receipt.encode()?;
    let verify_started = Instant::now();
    verify(&installation, &verifier, &encoded)?;
    let verify_time = verify_started.elapsed();
    tamper_checks(&installation, &verifier, &receipt, &encoded)?;
    println!("METRIC benchmark_segments={segments}");
    println!(
        "METRIC benchmark_source_groups={}",
        plan.ledger.source_groups
    );
    println!(
        "METRIC benchmark_scalar_claims={}",
        accumulated.claims().len()
    );
    println!("METRIC benchmark_final_proofs=1");
    println!("METRIC benchmark_per_segment_proofs=0");
    println!("METRIC benchmark_num_vars={NUM_VARS}");
    println!("METRIC benchmark_query_rounds={NUM_VARS}");
    println!("METRIC benchmark_query_degree=2");
    println!(
        "METRIC benchmark_setup_ms={:.6}",
        setup_time.as_secs_f64() * 1000.0
    );
    println!(
        "METRIC benchmark_commit_ms={:.6}",
        commit_time.as_secs_f64() * 1000.0
    );
    println!(
        "METRIC benchmark_query_ms={:.6}",
        query_time.as_secs_f64() * 1000.0
    );
    println!(
        "METRIC benchmark_native_prover_ms={:.6}",
        native_time.as_secs_f64() * 1000.0
    );
    println!(
        "METRIC benchmark_prover_ms={:.6}",
        (query_time + native_time).as_secs_f64() * 1000.0
    );
    println!(
        "METRIC benchmark_verifier_ms={:.6}",
        verify_time.as_secs_f64() * 1000.0
    );
    println!("METRIC benchmark_receipt_bytes={}", encoded.len());
    println!("METRIC benchmark_native_pcs_bytes={native_bytes}");
    println!("METRIC benchmark_public_statement_bytes={public_statement_bytes}");
    println!(
        "METRIC benchmark_row_bytes={}",
        plan.ledger.encoded_row_bytes
    );
    println!(
        "METRIC benchmark_suffix_folds={}",
        row.schedule().recursive_folds.len()
    );
    println!(
        "METRIC benchmark_shared_d_width={}",
        plan.ledger.shared_d_width
    );
    println!(
        "METRIC benchmark_setup_field_elements={}",
        setup.expanded.descriptor().num_field_elements
    );
    println!(
        "METRIC benchmark_setup_matrix_bytes={}",
        setup
            .expanded
            .descriptor()
            .num_field_elements
            .checked_mul(F::NUM_BYTES)
            .ok_or_else(invalid)?
    );
    println!("METRIC benchmark_verifier_setup_field_elements={verifier_setup_fields}");
    println!(
        "METRIC benchmark_row_verifier_setup_field_elements={}",
        plan.ledger.verifier_setup_field_elements
    );
    println!(
        "METRIC benchmark_query_degree_charge={}",
        accumulated.query_degree_charge()?
    );
    println!("SOURCE original A512/rank1/width40/cap427818804 B64/width344; F512K=8192B; carrier64/h8/base8/depth7");
    println!(
        "ROOT input_witness={} output_witness={} terminal={:?}",
        row.schedule().root.input_witness_len,
        row.schedule().root.output_witness_len,
        row.schedule().terminal
    );
    for (level, fold) in row.schedule().recursive_folds.iter().enumerate() {
        println!(
            "SUFFIX level={} input_witness={} output_witness={} role_dims={:?}",
            level + 1,
            fold.input_witness_len,
            fold.output_witness_len,
            fold.params.role_dims()
        );
    }
    println!("JOINT_REJECTION numerator={} denominator_log2={} (ideal indexed law; not end-to-end admission)",
        plan.ledger.joint_response.reject_numerator,plan.ledger.joint_response.reject_denominator_log2);
    #[cfg(target_os = "linux")]
    println!(
        "METRIC benchmark_peak_rss_bytes={}",
        aqra::peak_rss_bytes()?
    );
    println!("BENCHMARK decoded ordinary evaluations only; no VM global RAM predicate, ZK or measured recursive-baseline parity claim");
    Ok(())
}
fn tamper_checks(
    installation: &Installation,
    verifier: &AkitaVerifier<Config>,
    receipt: &Receipt,
    encoded: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let reject = |name: &str, bytes: Vec<u8>| -> Result<(), Box<dyn std::error::Error>> {
        if verify(installation, verifier, &bytes).is_ok() {
            return Err(io::Error::other(format!("fatal: tamper {name} accepted")).into());
        }
        println!("TAMPER {name}=rejected");
        Ok(())
    };
    let mut changed = receipt.clone();
    *changed.query.last_mut().ok_or_else(invalid)? ^= 1;
    reject("scalar_endpoint", changed.encode()?)?;
    let mut public = installation.public.clone();
    public.get_mut(0).ok_or_else(invalid)?.evaluation += F::one();
    let mut changed = receipt.clone();
    changed.seal = ledger(&public, &installation.point, &installation.context, None)?.seal()?;
    reject("ordinary_claim", changed.encode()?)?;
    let mut changed = receipt.clone();
    let mut public = installation.public.clone();
    if public.len() > 1 {
        public.swap(0, 1);
    } else {
        let marker = source_identity(public.first().ok_or_else(invalid)?, 0)?;
        let position = changed
            .seal
            .windows(marker.len())
            .position(|bytes| bytes == marker)
            .ok_or_else(invalid)?;
        let ordinal_offset = b"akita/aqra/original-dense-source/v8".len();
        *changed
            .seal
            .get_mut(position + ordinal_offset)
            .ok_or_else(invalid)? ^= 1;
    }
    if public.len() > 1 {
        changed.seal = ledger(&public, &installation.point, &installation.context, None)?.seal()?;
    }
    reject("source_order", changed.encode()?)?;
    // Mutate the canonical registered source identity's profile/view separately.
    for (name, marker) in [
        ("profile", original_profile_marker(&installation.public)?),
        ("view", source_view_marker(&installation.public)?),
    ] {
        let mut changed = receipt.clone();
        let position = changed
            .seal
            .windows(marker.len())
            .position(|bytes| bytes == marker)
            .ok_or_else(invalid)?;
        *changed
            .seal
            .get_mut(position + marker.len() - 1)
            .ok_or_else(invalid)? ^= 1;
        reject(name, changed.encode()?)?;
    }
    let mut changed = receipt.clone();
    *changed.query.get_mut(0).ok_or_else(invalid)? ^= 1;
    reject("query_round", changed.encode()?)?;
    let mut changed = receipt.clone();
    *changed.query_history.last_mut().ok_or_else(invalid)? ^= 1;
    reject("seal_history_completed_round", changed.encode()?)?;
    let mut changed = receipt.clone();
    *changed.native.get_mut(0).ok_or_else(invalid)? ^= 1;
    reject("full_root", changed.encode()?)?;
    let mut changed = receipt.clone();
    *changed.native.last_mut().ok_or_else(invalid)? ^= 1;
    reject("actual_terminal", changed.encode()?)?;
    let mut changed = receipt.clone();
    changed.native.pop();
    reject("native_truncation", changed.encode()?)?;
    let mut changed = receipt.clone();
    changed.native.push(0);
    reject("native_trailing", changed.encode()?)?;
    let mut truncated = encoded.to_vec();
    truncated.pop();
    reject("receipt_truncation", truncated)?;
    let mut trailing = encoded.to_vec();
    trailing.push(0);
    reject("receipt_trailing", trailing)?;
    Ok(())
}
fn original_profile_marker(public: &[PublicSource]) -> Result<Vec<u8>, AkitaError> {
    let mut bytes = Vec::new();
    public
        .first()
        .ok_or_else(invalid)?
        .commitment
        .profile()
        .append_descriptor_bytes(&mut bytes);
    Ok(bytes)
}
fn source_view_marker(public: &[PublicSource]) -> Result<Vec<u8>, AkitaError> {
    let first = public.first().ok_or_else(invalid)?;
    let identity = source_identity(first, 0)?;
    let mut profile = original_profile_marker(public)?;
    profile.push(first.view.tag());
    require(
        identity
            .windows(profile.len())
            .any(|bytes| bytes == profile),
    )?;
    Ok(profile)
}
