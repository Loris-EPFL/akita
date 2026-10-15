//! Ordinary per-segment PCS-only control; no frontend, accumulation, or outer aggregator.
//!
//! Run from the Akita workspace with its checked-in schedule artifacts:
//! `cargo run --release -p akita-pcs --example accumulation_pcs_control -- --segments 256`.
//! Default features are `parallel,transcript-blake2b`. A serial build uses
//! `--no-default-features --features transcript-blake2b`; a parallel build also
//! accepts `AKITA_PARALLEL=0` as in the upstream end-to-end benchmark.
//!
//! The anchor is the existing ordinary PCS verifier accepting each segment's original
//! commitment, evaluated opening, point, and selected schedule. Segment zero uses
//! the upstream source seed `0xdead_beef`; later segments use distinct derived
//! seeds. All segments use the upstream point seed `0xcafe_babe` and `b"bench"`
//! transcript domain, with Dense fp128, nv14, and the catalog's selected ring dimension.
//!
//! Successful runs emit `METRIC control_<name>=number` lines:
//! - `segments`: number of separately produced and verified PCS proofs.
//! - `num_vars`, `ring_dimension`: the fixed source shape and selected root dimension.
//! - `setup_ms`: one shared initialization (thread pool, catalog, prover setup,
//!   backend, and verifier setup/preparation), not repeated for each segment.
//! - `commit_ms`, `prover_ms`, `verifier_ms`: cumulative wall milliseconds around
//!   the respective public calls, excluding source generation/import, opening
//!   evaluation, and claim construction. Proving includes native serialization.
//! - `pcs_bytes`: sum of the actual serialized byte vectors returned by proving;
//!   excludes public commitments, claims, and setup material.
//! - `peak_rss_bytes`: Linux `/proc/self/status` VmHWM in bytes, covering the
//!   whole process lifetime, shared setup, and Rayon workers. It is not a sampled
//!   estimate or a per-phase/incremental peak. Other platforms omit this metric.
//!
//! Deterministic witnesses are benchmark inputs, not a zero-knowledge or security
//! admission claim. Only one segment's source, handle, and proof are held at once;
//! setup/backend caches are shared, so later segments can benefit from warm caches.

#![allow(missing_docs)]

#[path = "support/workspace_schedules.rs"]
mod workspace_schedules;

use akita_algebra::poly::multilinear_eval;
use akita_config::{proof_optimized::fp128, CommitmentConfig};
use akita_cpu_backend::{CpuBackend, DensePoly, GroupContext};
use akita_params::{
    BasisMode, CommittedGroupBatchProfile, PolynomialGroupLayout, ScheduleLookupKey,
    SourceOpeningView,
};
use akita_pcs::prepare_native_proof_context;
use akita_prover::SelectedProverOpeningData;
use akita_types::{GroupBatchStatement, OpeningClaims, PolynomialGroupClaims};
use jolt_field::{CanonicalEncoding, Ring};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::io;
use std::time::{Duration, Instant};
use workspace_schedules::load_workspace_scheme;

type Config = fp128::Dense;
type F = fp128::Field;

const NUM_VARS: usize = 14;
const SOURCE_SEED: u64 = 0xdead_beef;
// An odd step gives distinct u64 seeds for every index in the accepted range.
const SEED_STEP: u64 = 0x9e37_79b9_7f4a_7c15;
const POINT_SEED: u64 = 0xcafe_babe;
const TRANSCRIPT_DOMAIN: &[u8] = b"bench";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(segments) = parse_segments()? else {
        return Ok(());
    };

    let started = Instant::now();
    #[cfg(feature = "parallel")]
    {
        let num_threads = if std::env::var_os("AKITA_PARALLEL").is_some_and(|value| value == "0") {
            1
        } else {
            0
        };
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .stack_size(64 * 1024 * 1024)
            .build_global()?;
    }
    let scheme = load_workspace_scheme::<Config>()?;
    let row =
        scheme
            .schedules()
            .resolve_key(&ScheduleLookupKey::single(PolynomialGroupLayout::new(
                NUM_VARS, 1,
            )))?;
    let ring_dimension = row.schedule().root.params.inner().matrix.ring_dimension();
    let setup = scheme.setup_prover(NUM_VARS, 1)?;
    let backend = CpuBackend::new(setup.expanded.clone())?;
    let verifier = scheme.verifier(scheme.setup_verifier(&setup)?)?;
    let setup_time = started.elapsed();

    let mut point_rng = StdRng::seed_from_u64(POINT_SEED);
    let point: Vec<F> = (0..NUM_VARS)
        .map(|_| F::from_u128_reduced(point_rng.gen::<u128>()))
        .collect();
    let mut commit_time = Duration::ZERO;
    let mut prover_time = Duration::ZERO;
    let mut verifier_time = Duration::ZERO;
    let mut pcs_bytes = 0u64;

    for segment in 0..segments {
        let seed = SOURCE_SEED.wrapping_add(segment.wrapping_mul(SEED_STEP));
        let evaluations = make_dense_evals(seed)?;
        let evaluation = multilinear_eval(&evaluations, &point)?;
        let polynomial = DensePoly::<F>::from_field_evals(NUM_VARS, evaluations)?;
        let source = backend.import_source(vec![polynomial])?;

        let started = Instant::now();
        let commit_output = backend.commit(
            scheme.schedules(),
            &source,
            GroupContext::scheduler_without_precommitted_groups(),
        )?;
        commit_time = commit_time
            .checked_add(started.elapsed())
            .ok_or_else(|| io::Error::other("cumulative commit duration overflow"))?;

        let selection = scheme
            .schedules()
            .resolve_profiles(
                &CommittedGroupBatchProfile {
                    final_group: *commit_output.committed_group.profile(),
                    precommitteds: Vec::new(),
                },
                &[SourceOpeningView::Decoded],
            )?
            .selection();
        let prover_claims = OpeningClaims::from_groups(vec![PolynomialGroupClaims::new(
            point.clone(),
            vec![evaluation],
            commit_output.committed_group.clone(),
        )?])?;
        let native_statement = GroupBatchStatement::new(
            selection,
            OpeningClaims::from_groups(vec![PolynomialGroupClaims::new(
                point.clone(),
                vec![evaluation],
                &commit_output.committed_group,
            )?])?,
        )?;
        let native_context = prepare_native_proof_context::<Config>(
            setup.expanded.descriptor(),
            scheme.schedules(),
            &native_statement,
            TRANSCRIPT_DOMAIN,
            &[],
            BasisMode::Lagrange,
            0,
        )?;
        let prover_data = SelectedProverOpeningData::from_committed_claims::<Config>(
            prover_claims,
            vec![commit_output.private_handle],
            scheme.schedules(),
        )?;

        let started = Instant::now();
        let proof = scheme.batched_prove(
            &setup,
            prover_data,
            &backend,
            TRANSCRIPT_DOMAIN,
            native_context.registry(),
            native_context.augmentation(),
            BasisMode::Lagrange,
        )?;
        prover_time = prover_time
            .checked_add(started.elapsed())
            .ok_or_else(|| io::Error::other("cumulative prover duration overflow"))?;

        let verifier_claims = OpeningClaims::from_groups(vec![PolynomialGroupClaims::new(
            point.clone(),
            vec![evaluation],
            &commit_output.committed_group,
        )?])?;
        let statement = GroupBatchStatement::new(selection, verifier_claims)?;
        let verifier_context = prepare_native_proof_context::<Config>(
            setup.expanded.descriptor(),
            scheme.schedules(),
            &statement,
            TRANSCRIPT_DOMAIN,
            &[],
            BasisMode::Lagrange,
            0,
        )?;
        let started = Instant::now();
        verifier.batched_verify(
            &proof,
            TRANSCRIPT_DOMAIN,
            verifier_context.registry(),
            verifier_context.augmentation(),
            statement,
            BasisMode::Lagrange,
        )?;
        verifier_time = verifier_time
            .checked_add(started.elapsed())
            .ok_or_else(|| io::Error::other("cumulative verifier duration overflow"))?;
        pcs_bytes = pcs_bytes
            .checked_add(u64::try_from(proof.len())?)
            .ok_or_else(|| io::Error::other("cumulative PCS byte count overflow"))?;
        // All segment-specific objects are dropped before the next iteration.
    }

    #[cfg(target_os = "linux")]
    let peak_rss = peak_rss_bytes()?;
    #[cfg(not(target_os = "linux"))]
    eprintln!("peak RSS metric unavailable: this control reads Linux VmHWM");

    println!("METRIC control_segments={segments}");
    println!("METRIC control_num_vars={NUM_VARS}");
    println!("METRIC control_final_proofs={segments}");
    println!("METRIC control_per_segment_proofs={segments}");
    println!("CONTROL separate ordinary PCS proofs; not a recursive streaming baseline");
    println!("METRIC control_ring_dimension={ring_dimension}");
    println!(
        "METRIC control_setup_ms={:.6}",
        setup_time.as_secs_f64() * 1_000.0
    );
    println!(
        "METRIC control_commit_ms={:.6}",
        commit_time.as_secs_f64() * 1_000.0
    );
    println!(
        "METRIC control_prover_ms={:.6}",
        prover_time.as_secs_f64() * 1_000.0
    );
    println!(
        "METRIC control_verifier_ms={:.6}",
        verifier_time.as_secs_f64() * 1_000.0
    );
    println!("METRIC control_pcs_bytes={pcs_bytes}");
    #[cfg(target_os = "linux")]
    println!("METRIC control_peak_rss_bytes={peak_rss}");
    Ok(())
}

fn parse_segments() -> Result<Option<u64>, Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let first = args.next().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: accumulation_pcs_control [--segments] <positive-u64>",
        )
    })?;
    if matches!(first.as_str(), "--help" | "-h") && args.len() == 0 {
        println!(
            "Usage: accumulation_pcs_control [--segments] <positive-u64>\n\
             Ordinary per-segment PCS-only control (for example: 1, 2, 3, or 256).\n\
             Accepts 1..=u64::MAX segments; processes and verifies each proof separately."
        );
        return Ok(None);
    }
    let count = if first == "--segments" {
        args.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "--segments requires a count")
        })?
    } else {
        first
    };
    if args.next().is_some() {
        return Err(
            io::Error::new(io::ErrorKind::InvalidInput, "unexpected extra argument").into(),
        );
    }
    let segments = count.parse::<u64>()?;
    if segments == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "segment count must be positive",
        )
        .into());
    }
    Ok(Some(segments))
}

fn make_dense_evals(seed: u64) -> Result<Vec<F>, Box<dyn std::error::Error>> {
    let mut rng = StdRng::seed_from_u64(seed);
    let len = 1usize << NUM_VARS;
    let decomp = Config::decomposition();
    if decomp.log_commit_bound >= 128 {
        Ok((0..len)
            .map(|_| F::from_u128_reduced(rng.gen::<u128>()))
            .collect())
    } else {
        // Match akita_e2e's signed source-bound branch, with checked arithmetic.
        let shift = decomp
            .log_commit_bound
            .min(62)
            .checked_sub(1)
            .ok_or_else(|| io::Error::other("committed-source bound must be positive"))?;
        let half_bound = 1i64
            .checked_shl(shift)
            .ok_or_else(|| io::Error::other("committed-source bound shift overflow"))?;
        Ok((0..len)
            .map(|_| F::from_i64(rng.gen_range(-half_bound..half_bound)))
            .collect())
    }
}

#[cfg(target_os = "linux")]
fn peak_rss_bytes() -> Result<u64, Box<dyn std::error::Error>> {
    let status = std::fs::read_to_string("/proc/self/status")?;
    let value = status
        .lines()
        .find_map(|line| line.strip_prefix("VmHWM:"))
        .ok_or_else(|| io::Error::other("/proc/self/status has no VmHWM"))?;
    let mut parts = value.split_whitespace();
    let kib = parts
        .next()
        .ok_or_else(|| io::Error::other("VmHWM has no value"))?
        .parse::<u64>()?;
    if parts.next() != Some("kB") || parts.next().is_some() {
        return Err(io::Error::other("unexpected VmHWM unit or format").into());
    }
    let bytes = kib
        .checked_mul(1024)
        .ok_or_else(|| io::Error::other("VmHWM byte count overflow"))?;
    Ok(bytes)
}
