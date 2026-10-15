use super::*;

#[test]
fn same_revision_drift_is_reported_before_baseline_comparison() {
    assert!(should_emit_catalog_drift_report(false, 0));
    assert!(should_emit_catalog_drift_report(false, 3));
    assert!(!should_emit_catalog_drift_report(true, 0));
    assert!(should_emit_catalog_drift_report(true, 3));
}

#[test]
fn positional_family_filters_are_checked_and_ordered() {
    let one = parse_args_from(vec!["generated".into(), "fp32_dense".into()]).expect("known family");
    assert_eq!(
        selected_families(one.family_filter.as_deref())
            .iter()
            .map(|family| family.family_name())
            .collect::<Vec<_>>(),
        vec!["fp32_dense"],
    );

    let multiple = parse_args_from(vec![
        "generated".into(),
        "fp64_dense".into(),
        "fp32_dense".into(),
    ])
    .expect("known families");
    assert_eq!(
        selected_families(multiple.family_filter.as_deref())
            .iter()
            .map(|family| family.family_name())
            .collect::<Vec<_>>(),
        vec!["fp64_dense", "fp32_dense"],
    );

    let all = parse_args_from(vec!["generated".into()]).expect("all families");
    assert!(all.family_filter.is_none());
    assert_eq!(selected_families(None).len(), ALL_GENERATED_FAMILIES.len());

    let report = parse_args_from(vec![
        "generated".into(),
        "--check-catalog".into(),
        "--catalog-report".into(),
        "report.tsv".into(),
    ]);
    if cfg!(feature = "catalog-check") {
        assert_eq!(
            report.expect("catalog report").catalog_report,
            Some(PathBuf::from("report.tsv"))
        );
    } else {
        assert!(report
            .err()
            .expect("catalog check feature")
            .contains("catalog-check"));
    }

    let unknown = parse_args_from(vec!["generated".into(), "not_a_family".into()])
        .err()
        .expect("unknown family must reject");
    assert!(unknown.contains("unknown schedule family"));
}

#[test]
fn explicit_scalar_sweep_replaces_default_catalog_work() {
    let family = family_by_name("fp128_onehot").expect("known family");
    let explicit_rows = ExplicitRows {
        final_group: Some(parse_explicit_group("fp128_onehot:14:1").expect("explicit group")),
        precommitted_groups: Vec::new(),
    };

    let spec = emit_spec_with_overrides(
        family,
        &GenerationPreplans::default(),
        PathBuf::from("generated"),
        &explicit_rows,
    )
    .expect("explicit emit spec");

    assert_eq!(spec.keys, vec![PolynomialGroupLayout::new(14, 1)]);
    assert!(spec.grouped_requests.is_empty());
}

#[test]
fn explicit_rows_accept_adaptive_families() {
    let args = parse_args_from(vec![
        "/tmp/akita-adaptive-explicit-test".into(),
        "--final-group".into(),
        "fp64_dense:8:1".into(),
    ])
    .expect("adaptive explicit family");

    assert_eq!(
        args.family_filter.as_deref(),
        Some(&["fp64_dense".into()][..])
    );
}

#[test]
fn explicit_group_rejects_source_metadata() {
    assert!(parse_explicit_group("fp128_onehot:14:1:256").is_err());
}

#[test]
fn strict_catalog_baseline_requires_a_baseline_path() {
    let error = parse_args_from(vec![
        "out".into(),
        "--require-catalog-baseline-match".into(),
    ])
    .err()
    .expect("strict baseline matching without a baseline must reject");
    assert!(error.contains("requires --catalog-baseline"));
}

#[test]
fn every_legacy_grouped_artifact_precommit_has_a_shipped_scalar_producer() {
    let artifact_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("artifacts/schedules");
    let catalogs = ALL_GENERATED_FAMILIES
        .iter()
        // Full-ring accumulation admits its original endpoint descriptors
        // directly; it does not promise standalone PCS producer catalogs.
        .filter(|family| {
            (family.policy)().opening_family
                != akita_config::PlannerOpeningFamily::FullRingE2InnerRecursive
        })
        .map(|family| {
            let path = artifact_dir.join(format!("{}.aks", family.family_name()));
            let bytes = std::fs::read(&path)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
            let catalog = akita_schedules::ValidatedScheduleCatalog::from_artifact_bytes(
                &bytes,
                family.family_name(),
                &(family.policy)(),
                family.ring_challenge_config,
            )
            .unwrap_or_else(|error| panic!("failed to load {}: {error}", family.family_name()));
            (family.family_name(), catalog)
        })
        .collect::<std::collections::BTreeMap<_, _>>();

    let scalar_producers = catalogs
        .iter()
        .flat_map(|(family, catalog)| {
            catalog
                .rows()
                .filter(|row| row.profiles().precommitteds.is_empty())
                .map(move |row| (*family, row.profiles().final_group))
        })
        .collect::<Vec<_>>();

    for (family, catalog) in &catalogs {
        for row in catalog.rows() {
            for precommitted in &row.profiles().precommitteds {
                assert!(
                    scalar_producers
                        .iter()
                        .any(|(_, producer)| producer == precommitted),
                    "family {family} contains a grouped precommit without an exact shipped scalar producer: {:?}",
                    precommitted.group,
                );
            }
        }
    }

    for (recursive_family, base_family) in [
        ("fp128_onehot_recursive", "fp128_onehot"),
        (
            "fp128_onehot_recursive_multi_chunk_w8r2",
            "fp128_onehot_multi_chunk",
        ),
    ] {
        let recursive = catalogs
            .get(recursive_family)
            .unwrap_or_else(|| panic!("missing recursive artifact {recursive_family}"));
        let base_producers = scalar_producers
            .iter()
            .filter(|(family, _)| *family == base_family)
            .map(|(_, producer)| producer)
            .collect::<Vec<_>>();
        for row in recursive.rows() {
            for precommitted in &row.profiles().precommitteds {
                assert!(
                    base_producers.contains(&precommitted),
                    "recursive artifact {recursive_family} precommit is not produced by base family {base_family}: {:?}",
                    precommitted.group,
                );
            }
        }
    }
}

#[test]
fn compact_inner_generation_publishes_a_trusted_complete_root() {
    use akita_config::{full_ring::CompactInner, CommitmentConfig, TrustedScheduleCatalog};
    use akita_params::sis::CommittedSourceClass;

    let family = family_by_name(CompactInner::schedule_family_name()).expect("compact family");
    let output_dir = std::env::temp_dir().join(format!(
        "akita-compact-catalog-roundtrip-{}",
        std::process::id()
    ));
    let spec = emit_spec_for_family(family, &GenerationPreplans::default(), output_dir.clone())
        .expect("complete compact generation request");
    let (key, contracts) = CompactInner::root_inputs().expect("original endpoint contracts");
    assert!(spec.keys.is_empty());
    assert!(spec.preplanned_scalar.is_empty());
    assert_eq!(spec.grouped_requests.len(), 1);
    assert_eq!(spec.grouped_requests[0].key(), key);
    assert_eq!(spec.grouped_requests[0].source_contracts(), contracts);
    for contract in &contracts[1..] {
        assert_eq!(
            contract.class(),
            CommittedSourceClass::OriginalBalancedDigitPlanes
        );
        assert_eq!(contract.decomposition().log_commit_bound, 128);
    }

    let outputs = render_schedule_artifact_outputs_with_validation(
        &[spec],
        MaterializationDiagnostics::default(),
        |_, entries| {
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].key(), key);
            assert_eq!(
                entries[0]
                    .precommitted_producers()
                    .iter()
                    .map(|producer| producer.source_contract())
                    .collect::<Vec<_>>(),
                contracts,
            );
            Ok(())
        },
    )
    .expect("materialize and validate compact artifact");
    std::fs::create_dir(&output_dir).expect("isolated publication directory");
    let destinations = publish_artifact_outputs(outputs).expect("publish compact artifact");
    assert_eq!(destinations.len(), 1);
    let bytes = std::fs::read(&destinations[0]).expect("published compact artifact");
    std::fs::remove_dir_all(&output_dir).expect("remove roundtrip artifact");
    let catalog = TrustedScheduleCatalog::<CompactInner>::from_artifact_bytes(&bytes)
        .expect("decode and bind actual compact family");
    assert_eq!(catalog.rows().count(), 1);
    let row = catalog.resolve_key(&key).expect("exact original root key");
    let schedule = row.schedule();
    assert_eq!(
        schedule
            .recursive_folds
            .iter()
            .map(|step| (step.input_witness_len, step.output_witness_len))
            .collect::<Vec<_>>(),
        vec![(1 << 22, 1 << 21), (1 << 21, 1 << 20), (1 << 20, 1 << 19)],
    );
    assert_eq!(
        schedule.terminal.blocks,
        akita_params::BlockGeometry::new(4096, 512, 8)
    );
    assert_eq!(
        schedule.terminal.opening_method,
        akita_params::OpeningMethod::FULL_RING_E2
    );
    assert!(catalog
        .resolve_key(&ScheduleLookupKey::single(key.final_group))
        .is_err());
}

#[test]
fn compact_inner_explicit_scalar_and_producer_requests_reject() {
    let family = family_by_name("akita-full-ring-e2-compact-inner-v1").expect("compact family");
    let preplans = GenerationPreplans::default();
    let explicit_rows = ExplicitRows {
        final_group: Some(
            parse_explicit_group("akita-full-ring-e2-compact-inner-v1:16:56")
                .expect("known family and scalar layout"),
        ),
        precommitted_groups: Vec::new(),
    };
    let spec = emit_spec_with_overrides(
        family,
        &preplans,
        PathBuf::from("generated"),
        &explicit_rows,
    )
    .expect("explicit scalar request");
    let error = render_schedule_artifact_outputs_with_validation(
        &[spec],
        MaterializationDiagnostics::default(),
        |_, _| Ok(()),
    )
    .expect_err("scalar requests must fail, not publish an empty catalog");
    assert!(error.contains("not scalar requests"));
    let error =
        (family.explicit_precommitted_group)(&preplans, PolynomialGroupLayout::singleton(16))
            .expect_err("no standalone compact producer");
    assert!(error
        .to_string()
        .contains("no standalone precommitted producer"));
}
