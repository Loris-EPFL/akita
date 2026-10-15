#![allow(missing_docs)]

#[cfg(feature = "logging-transcript")]
#[path = "transcript_hardening/catalog_events.rs"]
mod catalog_events;
mod common;

use akita_cpu_backend::CpuBackend;
#[cfg(feature = "logging-transcript")]
use common::mutations::{
    assert_ranges_match_context, representative_mutation_ranges, selected_sumcheck_protocols,
};
use common::*;
use jolt_field::One;

const NUM_VARS: usize = 14;
const LABEL: &[u8] = b"hardening/onehot/native";

#[test]
fn stream_binds_session_statement_basis_and_eof() {
    init_rayon_pool();
    run_on_large_stack(|| {
        let scheme = load_workspace_scheme::<OneHotCfg>().expect("workspace schedule catalog");
        let layout = scheme
            .schedules()
            .resolve_key(&akita_params::ScheduleLookupKey::single(
                akita_params::PolynomialGroupLayout::singleton(NUM_VARS),
            ))
            .expect("layout")
            .schedule()
            .root
            .params
            .final_group();
        let poly = make_onehot_poly::<OneHotCfg>(NUM_VARS, 0x5151);
        let point = random_point(NUM_VARS, 0x6161);
        let opening = opening_from_poly_for_layout(&poly, &point, &layout, BasisMode::Lagrange);
        let setup = scheme.setup_prover(NUM_VARS, 1).expect("setup");
        let stack = CpuBackend::new(setup.expanded.clone()).expect("backend");
        let verifier_setup = scheme.setup_verifier(&setup).expect("verifier setup");
        let akita_cpu_backend::CommitOutput {
            committed_group: commitment,
            private_handle: hint,
        } = stack
            .commit(
                scheme.schedules(),
                &stack.import_source(vec![poly.clone()]).expect("source"),
                akita_cpu_backend::GroupContext::scheduler_without_precommitted_groups(),
            )
            .expect("commit");
        #[cfg(feature = "logging-transcript")]
        akita_transcript::clear_thread_events();
        let proof = { let native_setup = &setup; let native_opening = prove_input::<OneHotCfg>(&point, &[opening], &commitment, hint, scheme.schedules()); let native_session = LABEL; let native_basis = BasisMode::Lagrange; akita_pcs::prepare_native_prover_context(native_setup.expanded.descriptor(), scheme.schedules(), &native_opening, native_session, &[], native_basis, 0).and_then(|native_context| scheme.batched_prove(native_setup, native_opening, &stack, native_session, native_context.registry(), native_context.augmentation(), native_basis)) }
            .expect("prove");
        #[cfg(feature = "logging-transcript")]
        let prover_events = akita_transcript::thread_events();
        #[cfg(feature = "logging-transcript")]
        let prover_ranges = akita_transcript::thread_proof_ranges();
        let verify = |candidate: &[u8], session: &[u8], claimed: F, basis| {
            scheme
                .verifier(verifier_setup.clone())
                .and_then(|verifier| {
                    { let native_statement = verify_input::<OneHotCfg>(
                        &point,
                        &[claimed],
                        &commitment,
                        scheme.schedules(),
                    ); let native_session = session; let native_basis = basis; akita_pcs::prepare_native_proof_context(verifier.setup().expanded().descriptor(), verifier.schedules(), &native_statement, native_session, &[], native_basis, 0).and_then(|native_context| verifier.batched_verify(candidate, native_session, native_context.registry(), native_context.augmentation(), native_statement, native_basis)) }
                })
        };

        #[cfg(feature = "logging-transcript")]
        akita_transcript::clear_thread_events();
        verify(&proof, LABEL, opening, BasisMode::Lagrange).expect("honest proof");
        #[cfg(feature = "logging-transcript")]
        {
            let verifier_events = akita_transcript::thread_events();
            assert!(!prover_events.is_empty());
            assert_eq!(verifier_events, prover_events);
            assert_ranges_match_context(&prover_ranges);

            let mut ordered_ranges = prover_ranges.clone();
            ordered_ranges.sort_unstable_by_key(|range| range.start);
            let mut cursor = 0usize;
            for range in &ordered_ranges {
                assert_eq!(range.start, cursor, "native proof ranges must be gap-free");
                cursor = range.start.checked_add(range.len).expect("range end");
            }
            assert_eq!(
                cursor,
                proof.len(),
                "native proof ranges must cover the proof"
            );

            let role_ranges = representative_mutation_ranges(prover_ranges.clone());
            for family in [
                akita_transcript::SITE_FAMILY_SUMCHECK,
                akita_transcript::SITE_FAMILY_OPENING_PAYLOAD,
                akita_transcript::SITE_FAMILY_STAGE1,
                akita_transcript::SITE_FAMILY_STAGE2,
                akita_transcript::SITE_FAMILY_NEXT_WITNESS,
                akita_transcript::SITE_FAMILY_TERMINAL,
            ] {
                assert!(
                    role_ranges
                        .iter()
                        .any(|(bucket, _)| bucket.family == family),
                    "workload must exercise native proof-message family {family}"
                );
            }
            let sumcheck_protocols = selected_sumcheck_protocols(&role_ranges);
            for protocol in [
                akita_params::SumcheckProtocol::Stage1,
                akita_params::SumcheckProtocol::Stage2,
            ] {
                assert!(
                    sumcheck_protocols.contains(&protocol),
                    "workload must exercise native {protocol:?} sumcheck messages"
                );
            }
            for (family, protocol) in [
                (
                    akita_transcript::SITE_FAMILY_PHYSICAL_L2,
                    akita_params::SumcheckProtocol::PhysicalL2,
                ),
                (
                    akita_transcript::SITE_FAMILY_STAGE3,
                    akita_params::SumcheckProtocol::Stage3,
                ),
            ] {
                if role_ranges
                    .iter()
                    .any(|(bucket, _)| bucket.family == family)
                {
                    assert!(
                        sumcheck_protocols.contains(&protocol),
                        "fixture with {protocol:?} messages must mutate its sumcheck"
                    );
                }
            }
            assert!(
                role_ranges.iter().any(|(bucket, range)| {
                    bucket.family == akita_transcript::SITE_FAMILY_SUMCHECK
                        && akita_transcript::ProtocolSiteId::from_bytes(range.context.site_id).round
                            > 0
                }),
                "workload must exercise and mutate a later sumcheck round"
            );
            for (bucket, range) in role_ranges {
                let end = range.start.checked_add(range.len).expect("range end");
                assert!(end <= proof.len(), "recorded proof range must be in bounds");
                let mut mutated = proof.clone();
                mutated[range.start] ^= 1;
                assert!(
                    verify(&mutated, LABEL, opening, BasisMode::Lagrange).is_err(),
                    "fixed-shape mutation in native bucket={bucket:?} must reject",
                );
            }
        }
        // Context is application input, not authority carried by the proof.
        // Each changed endpoint/history compiles successfully before replay.
        let verifier = scheme.verifier(verifier_setup.clone()).expect("verifier");
        let statement = verify_input::<OneHotCfg>(
            &point, &[opening], &commitment, scheme.schedules(),
        );
        for (history, installation_index) in [
            (b"unapproved inherited history".as_slice(), 0),
            (&[][..], 1),
        ] {
            let context = akita_pcs::prepare_native_proof_context(
                verifier.setup().expanded().descriptor(), verifier.schedules(), &statement,
                LABEL, history, BasisMode::Lagrange, installation_index,
            ).expect("changed tuple has a valid native grammar");
            assert!(verifier.batched_verify(
                &proof, LABEL, context.registry(), context.augmentation(),
                statement.clone(), BasisMode::Lagrange,
            ).is_err(), "literal history and installation tuple must bind replay");
        }
        let mut changed_point = point.clone();
        changed_point[0] += F::one();
        let changed_statement = verify_input::<OneHotCfg>(
            &changed_point, &[opening], &commitment, scheme.schedules(),
        );
        let changed_context = akita_pcs::prepare_native_proof_context(
            verifier.setup().expanded().descriptor(), verifier.schedules(), &changed_statement,
            LABEL, &[], BasisMode::Lagrange, 0,
        ).expect("changed source point has a valid native grammar");
        assert!(verifier.batched_verify(
            &proof, LABEL, changed_context.registry(), changed_context.augmentation(),
            changed_statement, BasisMode::Lagrange,
        ).is_err(), "original public source point must bind replay");
        assert!(verify(
            &proof,
            b"hardening/onehot/different-session",
            opening,
            BasisMode::Lagrange,
        )
        .is_err());
        assert!(verify(&proof, LABEL, opening, BasisMode::Monomial).is_err());
        assert!(verify(&proof, LABEL, opening + F::one(), BasisMode::Lagrange).is_err());
        assert!(verify(
            &proof[..proof.len() - 1],
            LABEL,
            opening,
            BasisMode::Lagrange,
        )
        .is_err());
        let mut trailing = proof.clone();
        trailing.push(0);
        assert!(verify(&trailing, LABEL, opening, BasisMode::Lagrange).is_err());
    });
}

#[test]
fn stream_mutations_reject_without_panicking() {
    init_rayon_pool();
    run_on_large_stack(|| {
        let scheme = load_workspace_scheme::<DenseCfg>().expect("workspace schedule catalog");
        let poly = make_dense_poly(NUM_VARS, 0x7171);
        let point = random_point(NUM_VARS, 0x8181);
        let row = scheme
            .schedules()
            .resolve_key(&akita_params::ScheduleLookupKey::single(
                akita_params::PolynomialGroupLayout::singleton(NUM_VARS),
            ))
            .expect("layout");
        let opening = opening_from_poly_for_layout(
            &poly,
            &point,
            &row.schedule().root.params.final_group(),
            BasisMode::Lagrange,
        );
        let setup = scheme.setup_prover(NUM_VARS, 1).expect("setup");
        let stack = CpuBackend::new(setup.expanded.clone()).expect("backend");
        let verifier_setup = scheme.setup_verifier(&setup).expect("verifier setup");
        let akita_cpu_backend::CommitOutput {
            committed_group: commitment,
            private_handle: hint,
        } = stack
            .commit(
                scheme.schedules(),
                &stack.import_source(vec![poly.clone()]).expect("source"),
                akita_cpu_backend::GroupContext::scheduler_without_precommitted_groups(),
            )
            .expect("commit");
        let proof = { let native_setup = &setup; let native_opening = prove_input::<DenseCfg>(&point, &[opening], &commitment, hint, scheme.schedules()); let native_session = LABEL; let native_basis = BasisMode::Lagrange; akita_pcs::prepare_native_prover_context(native_setup.expanded.descriptor(), scheme.schedules(), &native_opening, native_session, &[], native_basis, 0).and_then(|native_context| scheme.batched_prove(native_setup, native_opening, &stack, native_session, native_context.registry(), native_context.augmentation(), native_basis)) }
            .expect("prove");
        for offset in [
            0,
            proof.len() / 4,
            proof.len() / 2,
            proof.len() * 3 / 4,
            proof.len() - 1,
        ] {
            let mut mutated = proof.clone();
            mutated[offset] ^= 1;
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                scheme
                    .verifier(verifier_setup.clone())
                    .and_then(|verifier| {
                        { let native_statement = verify_input::<DenseCfg>(
                            &point,
                            &[opening],
                            &commitment,
                            scheme.schedules(),
                        ); let native_session = LABEL; let native_basis = BasisMode::Lagrange; akita_pcs::prepare_native_proof_context(verifier.setup().expanded().descriptor(), verifier.schedules(), &native_statement, native_session, &[], native_basis, 0).and_then(|native_context| verifier.batched_verify(&mutated, native_session, native_context.registry(), native_context.augmentation(), native_statement, native_basis)) }
                    })
            }));
            assert!(matches!(outcome, Ok(Err(_))));
        }
    });
}
