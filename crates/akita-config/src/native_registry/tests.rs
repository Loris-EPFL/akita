use super::*;
use crate::proof_optimized::{fp128, fp32};
use akita_transcript::{IndexedChannel, IndexedError};

fn resources() -> NativeRegistryResources {
    NativeRegistryResources {
        limits: IndexedLimits {
            max_context_bytes: 1 << 24,
            max_prefix_bytes: 1 << 50,
            max_key_bytes: 1 << 50,
            max_descriptor_bytes: 1 << 40,
            max_answer_bytes: 1 << 60,
            max_result_bytes: 1 << 50,
            max_proof_bytes: 1 << 50,
        },
    }
}

fn row<Cfg: CommitmentConfig>() -> ResolvedScheduleRow {
    crate::test_support::workspace_schedule_catalog::<Cfg>()
        .unwrap()
        .resolve_key(&akita_params::ScheduleLookupKey::single(
            PolynomialGroupLayout::singleton(14),
        ))
        .unwrap()
        .clone()
}

#[test]
fn complete_root_suffix_terminal_has_global_ordinals_and_atomic_shells() {
    let row = row::<fp128::Dense>();
    let registry =
        approved_native_registry::<fp128::Dense>(&row, BasisMode::Lagrange, &resources(), 37)
            .unwrap();
    let mut ordinal = 0;
    let mut shells = 0;
    for event in registry.events() {
        if let InstalledEvent::Request(spec) = event {
            assert_eq!(spec.request.ordinal(), ordinal);
            ordinal += 1;
            match spec.request.operation() {
                IndexedOperation::ShellLevel => {
                    assert_eq!(spec.request.site().level, shells);
                    assert_eq!(spec.result_kind, CompletedResultKind::ShellLevel);
                    assert!(spec.result_len > 0);
                    assert!(spec.request.raw_len() > 0);
                    shells += 1;
                }
                IndexedOperation::Scalar128 => {
                    assert_eq!(spec.request.raw_len(), 2048);
                    assert_eq!(spec.result_len, 16);
                }
                IndexedOperation::Predicate32 => assert_eq!(spec.result_len, 32),
                other => panic!("unexpected selected-field operation {other:?}"),
            }
        }
    }
    assert_eq!(shells as usize, row.schedule().num_fold_levels());
    let level = narrow(row.schedule().recursive_folds.len() + 1).unwrap();
    let (length, payload) = akita_transcript::bounded_bytes_sites(
        akita_types::FoldSite::TerminalZPayload { level }
            .id()
            .unwrap(),
    );
    assert!(
        matches!(registry.events().last(), Some(InstalledEvent::BoundedBytes { site, max_len })
        if *site == payload && *max_len == row.schedule().terminal.response_shape.layout.groups[0].z_payload_bytes as u64)
    );
    assert_eq!(
        registry.events().get(registry.events().len() - 2),
        Some(&InstalledEvent::Message(length))
    );
}

#[test]
fn wrong_order_or_field_record_poison_the_installed_channel() {
    let row = row::<fp128::Dense>();
    let registry =
        approved_native_registry::<fp128::Dense>(&row, BasisMode::Lagrange, &resources(), 2)
            .unwrap();
    let context = akita_transcript::InstalledContext::new(
        &registry,
        b"registry-order",
        b"complete-public-endpoint",
    )
    .unwrap();
    let first = match registry.events().first().unwrap() {
        InstalledEvent::Message(record) => *record,
        _ => panic!("root preamble"),
    };
    let mut prover = akita_transcript::new_prover_channel(&context).unwrap();
    let mut wrong = first;
    wrong.atom_count += 1;
    wrong.encoded_bytes += 16;
    assert!(akita_transcript::prover_context(&mut prover, wrong).is_err());
    assert!(prover.status().is_err());
    assert!(prover.finish().is_err());
    let mut verifier = akita_transcript::new_verifier_channel(&context, &[]).unwrap();
    let point = akita_transcript::extension_group_record::<fp128::Field, fp128::Field>(
        akita_types::FoldSite::RootPoint { group: 0 }.id().unwrap(),
        ProtocolMessageKind::PublicValue,
        14,
    )
    .unwrap();
    assert!(akita_transcript::verifier_context(&mut verifier, point).is_err());
    assert!(verifier.finish().is_err());
}

#[test]
fn field_families_remain_separately_tagged_and_fully_priced() {
    let site = GrindingSite::Stage2Batch { level: 0 }.site_id(0);
    let selected =
        akita_transcript::field_request_spec::<fp128::Field, fp128::Field>(0, site).unwrap();
    let prime = akita_transcript::field_request_spec::<fp32::Field, fp32::Field>(0, site).unwrap();
    let extension =
        akita_transcript::field_request_spec::<fp32::Field, fp32::ExtensionField>(0, site).unwrap();
    assert_eq!(selected.request.operation(), IndexedOperation::Scalar128);
    assert_eq!(prime.request.operation(), IndexedOperation::PrimeScalar);
    assert_eq!(extension.request.operation(), IndexedOperation::FieldVector);
    assert_ne!(selected.request.descriptor(), prime.request.descriptor());
    assert_eq!(extension.request.raw_len(), 4 * prime.request.raw_len());
    assert_eq!(extension.result_len, 4 * prime.result_len);
    let registry = InstalledRegistry::new(
        1,
        vec![InstalledEvent::Request(selected)],
        resources().limits,
    )
    .unwrap();
    let context =
        akita_transcript::InstalledContext::new(&registry, b"field-substitution", b"instance")
            .unwrap();
    let mut channel = akita_transcript::new_prover_channel(&context).unwrap();
    assert_eq!(
        channel.request_at(IndexedOperation::PrimeScalar, site),
        Err(IndexedError::Request)
    );
    assert!(channel.finish().is_err());
}

#[test]
fn raw_view_binds_twenty_one_query_coordinates_not_sixteen_source_coordinates() {
    let decoded = OpeningClaimsLayout::new(16, 1).unwrap();
    let raw = OpeningClaimsLayout::from_groups_and_views(
        vec![PolynomialGroupLayout::singleton(16)],
        vec![akita_params::SourceOpeningView::OriginalRamPlanesV1],
    )
    .unwrap();
    assert_eq!(
        raw.group_layout(0).unwrap(),
        decoded.group_layout(0).unwrap()
    );
    assert_ne!(raw.opening_batch_digest(), decoded.opening_batch_digest());
    let id = akita_types::FoldSite::RootPoint { group: 0 }.id().unwrap();
    let raw_record = akita_transcript::extension_group_record::<fp128::Field, fp128::Field>(
        id,
        ProtocolMessageKind::PublicValue,
        raw.group_point_num_vars(0).unwrap(),
    )
    .unwrap();
    assert_eq!(raw_record.atom_count, 21);
    assert_eq!(raw_record.encoded_bytes, 21 * 16);
    assert!(raw.validate_basis(BasisMode::Monomial).is_err());
}

#[test]
fn whole_program_resource_bounds_reject_before_installation() {
    let row = row::<fp128::Dense>();
    for change in [0, 1, 2, 3, 4, 5] {
        let mut resources = resources();
        match change {
            0 => resources.limits.max_prefix_bytes = 1,
            1 => resources.limits.max_key_bytes = 1,
            2 => resources.limits.max_descriptor_bytes = 1,
            3 => resources.limits.max_answer_bytes = 2048,
            4 => resources.limits.max_result_bytes = 16,
            _ => resources.limits.max_proof_bytes = 1,
        }
        assert!(
            approved_native_registry::<fp128::Dense>(&row, BasisMode::Lagrange, &resources, 1)
                .is_err()
        );
    }
    let events = vec![InstalledEvent::Message(ProtocolContextRecord::new(
        ProtocolSiteId::default().to_bytes(),
        ProtocolMessageKind::ProofAtoms as u32,
        u64::MAX,
        u64::MAX,
        0,
    ))];
    assert!(validate_resources(&events, resources().limits).is_err());
}

#[test]
fn trusted_context_bounds_size_the_same_single_compiled_program() {
    let row = row::<fp128::Dense>();
    let events = compile_events::<fp128::Dense>(&row, BasisMode::Lagrange).unwrap();
    let small = NativeRegistryResources::from_trusted_bounds(&events, 7, 19)
        .unwrap()
        .limits;
    let large = NativeRegistryResources::from_trusted_bounds(&events, 18, 31)
        .unwrap()
        .limits;
    assert_eq!(large.max_context_bytes - small.max_context_bytes, 23);
    assert_eq!(large.max_prefix_bytes - small.max_prefix_bytes, 23);
    assert_eq!(large.max_key_bytes - small.max_key_bytes, 23);
    assert_eq!(large.max_proof_bytes, small.max_proof_bytes);
    assert_eq!(large.max_answer_bytes, small.max_answer_bytes);
    assert_eq!(large.max_result_bytes, small.max_result_bytes);
    let raw_sum: u64 = events
        .iter()
        .filter_map(|event| match event {
            InstalledEvent::Request(spec) => Some(spec.request.raw_len()),
            _ => None,
        })
        .sum();
    assert_eq!(small.max_answer_bytes, raw_sum);
    let registry = approved_native_registry_from_trusted_bounds::<fp128::Dense>(
        &row,
        BasisMode::Lagrange,
        7,
        19,
        91,
    )
    .unwrap();
    assert_eq!(registry.events(), events);
    assert!(akita_transcript::InstalledContext::new(&registry, &[0; 7], &[0; 19]).is_ok());
    assert!(akita_transcript::InstalledContext::new(&registry, &[0; 7], &[0; 20]).is_err());
    assert!(NativeRegistryResources::from_trusted_bounds(&events, u64::MAX, 1).is_err());
    assert!(NativeRegistryResources::from_trusted_bounds(&events, u64::MAX, 0).is_err());
}

#[test]
fn resource_costs_include_full_terminal_max_but_not_raw_answers_in_prefix() {
    let id = ProtocolSiteId::default();
    let spec = akita_transcript::field_request_spec::<fp128::Field, fp128::Field>(0, id).unwrap();
    let request_only = [InstalledEvent::Request(spec.clone())];
    let fixed = NativeRegistryResources::from_trusted_bounds(&request_only, 0, 0)
        .unwrap()
        .limits;
    let header = akita_transcript::INDEXED_PROVIDER.len() as u64 + 36;
    assert_eq!(
        fixed.max_prefix_bytes,
        header + 97 + spec.request.descriptor().len() as u64 + spec.result_len
    );
    assert_eq!(
        fixed.max_key_bytes,
        header + 69 + spec.request.descriptor().len() as u64
    );
    assert_eq!(fixed.max_answer_bytes, spec.request.raw_len());
    let (length, payload) = akita_transcript::bounded_bytes_sites(id);
    let with_terminal = [
        request_only[0].clone(),
        InstalledEvent::Message(length),
        InstalledEvent::BoundedBytes {
            site: payload,
            max_len: 12345,
        },
    ];
    let bounded = NativeRegistryResources::from_trusted_bounds(&with_terminal, 0, 0)
        .unwrap()
        .limits;
    assert_eq!(bounded.max_proof_bytes, 4 + 12345);
    assert_eq!(
        bounded.max_prefix_bytes - fixed.max_prefix_bytes,
        85 + 4 + 85 + 12345
    );
    assert_eq!(bounded.max_key_bytes, fixed.max_key_bytes);
    assert_eq!(bounded.max_answer_bytes, fixed.max_answer_bytes);
}
