//! Complete ordinary K/K control over retained native E2 producer handles.
//!
//! Run explicitly: cargo test -p akita-cpu-backend --release --features parallel
//! --test original_plane_control original_planes_complete_ordinary_proof -- --ignored --nocapture
//! The anchor is the original physical source and A/B/F commitment, not merely
//! equality of its decoded polynomial. No full-ring proof packet is retagged.
//! Native v8 contexts are installed from the trusted ordinary row and exact
//! public endpoint; standalone inherited history is empty. Verifier contexts
//! are prepared independently of retained private source/ownership handles.

#![allow(missing_docs)]

use akita_config::{
    approved_native_registry_from_trusted_bounds,
    full_ring::{CompactInner, OrdinaryControl}, policy_of, transcript_instance_descriptor,
    CommitmentConfig, SetupRequirements, TrustedScheduleCatalog, ValidatedScheduleCatalog,
};
use akita_cpu_backend::custom_source::{CommitInnerPlan, FullRingSource};
use akita_cpu_backend::{
    install_compact_inner_setup, AkitaProverSetup, CpuBackend, OneHotPoly, PackedRamSource,
};
use akita_params::{
    BasisMode, CommittedGroupBatchProfile, GroupCommitPhaseParams, OpeningMethod, SourceOpeningView,
};
use akita_prover::{
    batched_prove, CommitmentHandleMetadata, SelectedProverOpeningData,
    SetupPrefixProverRegistry,
};
use akita_transcript::{InheritedAugmentation, InstalledRegistry};
use akita_types::full_ring::FullRingCoefficientSource;
use akita_types::{
    encode_opening_endpoint_binding, AkitaSetupDescriptor, CommittedGroup, FullRingSourceId,
    GroupBatchStatement, OpeningClaims, PolynomialGroupClaims,
};
use akita_verifier::AkitaVerifier;
use jolt_field::{One, Prime128OffsetA7F7, Prime128OffsetA7F7Ext2, Ring, Zero};
use std::sync::Arc;

type F = Prime128OffsetA7F7;
type E2 = Prime128OffsetA7F7Ext2;
const D: usize = 128;
const NV: usize = 16;
const RAM_LEN: usize = 16 * 32 * 26 * D;
const SESSION: &[u8] = b"akita/original-plane/ordinary-control";


fn signed(digit: i8) -> F {
    let value = F::from_u64(u64::from(digit.unsigned_abs()));
    if digit < 0 { -value } else { value }
}

// Independent Boolean-cube oracle, deliberately not a production opening kernel.
fn weight(point: &[F], index: usize) -> F {
    point.iter().enumerate().fold(F::one(), |value, (bit, r)| {
        value * if index >> bit & 1 == 0 { F::one() - *r } else { *r }
    })
}

fn ram_opening(digits: &[i8], point: &[F]) -> F {
    let mut powers = [F::one(); 26];
    for digit in 1..26 {
        powers[digit] = powers[digit - 1] * F::from_u64(32);
    }
    digits.iter().enumerate().filter(|(_, digit)| **digit != 0).fold(
        F::zero(),
        |sum, (physical, &digit)| {
            let coefficient = physical % D;
            let plane = physical / D % 26;
            let ring = physical / (26 * D);
            sum + signed(digit) * powers[plane] * weight(point, ring * D + coefficient)
        },
    )
}

fn modulus_digits() -> [i8; 26] {
    let mut remaining = akita_params::field_modulus::<F>().unwrap();
    let digits = std::array::from_fn(|_| {
        let low = (remaining & 31) as i8;
        remaining >>= 5;
        if low >= 16 {
            remaining += 1;
            low - 32
        } else {
            low
        }
    });
    assert_eq!(remaining, 0);
    assert_eq!(digits.iter().rev().fold(F::zero(), |acc, &digit| {
        acc * F::from_u64(32) + signed(digit)
    }), F::zero());
    digits
}

fn ram_digits(alternate: bool) -> Vec<i8> {
    let mut digits = vec![0; RAM_LEN];
    // Exercise all blocks, source digits, both signs and coefficient endpoints.
    // A top digit of 15 is legal but is not a canonical centered-field path.
    for block in 0..16 {
        for digit in 0..26 {
            let position = (block * 7 + digit * 3) % 32;
            let coefficient = if digit & 1 == 0 { 0 } else { 127 };
            digits[(((block * 32 + position) * 26 + digit) * D) + coefficient] =
                if digit & 1 == 0 { -16 } else { 15 };
        }
    }
    if alternate {
        // Add exactly q at an otherwise zero logical coefficient. Decoding in K
        // is unchanged, but the original A-input path must remain different.
        for (plane, digit) in modulus_digits().into_iter().enumerate() {
            digits[plane * D + 37] = digit;
        }
    }
    digits
}

fn ram_plan() -> CommitInnerPlan {
    CommitInnerPlan {
        ring_dimension: D,
        num_live_blocks: 16,
        n_a: 3,
        num_positions_per_block: 32,
        num_digits_inner: 26,
        log_basis_inner: 5,
    }
}

#[test]
fn original_ram_paths_are_not_identified_by_decoded_values() {
    let point: Vec<_> = (0..NV).map(|bit| F::from_u64(3 + bit as u64)).collect();
    let first = ram_digits(false);
    let alternate = ram_digits(true);
    assert_ne!(first, alternate);
    assert_eq!(ram_opening(&first, &point), ram_opening(&alternate, &point));
    let first = PackedRamSource::from_signed_digits(NV, ram_plan(), first).unwrap();
    let alternate = PackedRamSource::from_signed_digits(NV, ram_plan(), alternate).unwrap();
    let read = |source: &PackedRamSource, plan: &CommitInnerPlan, address| {
        <PackedRamSource as FullRingSource<F>>::full_ring_digit(source, plan, address)
    };
    assert_ne!(read(&first, &ram_plan(), 37).unwrap(), read(&alternate, &ram_plan(), 37).unwrap());
    for (plane, digit) in modulus_digits().into_iter().enumerate() {
        assert_eq!(read(&alternate, &ram_plan(), plane * D + 37).unwrap(), signed(digit));
    }
    let mut wrong_rank = ram_plan();
    wrong_rank.n_a += 1;
    assert!(read(&alternate, &wrong_rank, 37).is_err());
    assert!(read(&alternate, &ram_plan(), RAM_LEN).is_err());
    let mut invalid = vec![0; RAM_LEN];
    invalid[0] = 16;
    assert!(PackedRamSource::from_signed_digits(NV, ram_plan(), invalid).is_err());
}

fn assert_profile(profile: &GroupCommitPhaseParams, polynomials: usize, positions: usize, blocks: usize, digits: usize) {
    assert_eq!(profile.group.num_vars(), NV);
    assert_eq!(profile.group.num_polynomials(), polynomials);
    assert_eq!(profile.inner.matrix.ring_dimension(), D);
    assert_eq!(profile.inner.matrix.output_rank(), 3);
    assert_eq!(profile.blocks.positions_per_block, positions);
    assert_eq!(profile.blocks.live_blocks, blocks);
    assert_eq!(profile.inner.digits.num_digits, digits);
}

fn statement<'a>(
    catalog: &TrustedScheduleCatalog<OrdinaryControl>,
    profiles: &CommittedGroupBatchProfile,
    commitments: &'a [CommittedGroup<F>],
    points: &[Vec<F>],
    values: &[Vec<F>],
) -> GroupBatchStatement<'a, F, F> {
    let claims = OpeningClaims::from_groups(commitments.iter().zip(points).zip(values).map(
        |((commitment, point), values)| {
            PolynomialGroupClaims::new(point.clone(), values.clone(), commitment).unwrap()
        },
    ).collect()).unwrap();
    let views = vec![SourceOpeningView::Decoded; commitments.len()];
    GroupBatchStatement::new(catalog.resolve_profiles(profiles, &views).unwrap().selection(), claims).unwrap()
}

struct NativeProofContext {
    registry: InstalledRegistry,
    endpoint_binding: Vec<u8>,
}

impl NativeProofContext {
    fn augmentation(&self) -> InheritedAugmentation<'_> {
        InheritedAugmentation {
            literal_history: &[],
            endpoint_binding: &self.endpoint_binding,
        }
    }
}

// Match akita-pcs's public context codec without adding a PCS dev dependency.
// Only approved public inputs enter this helper; ownership IDs never enter
// the endpoint binding or the installed event grammar.
fn native_context(
    setup: &AkitaSetupDescriptor,
    catalog: &TrustedScheduleCatalog<OrdinaryControl>,
    statement: &GroupBatchStatement<'_, F, F>,
) -> NativeProofContext {
    let layout = statement.claims().committed_layout().unwrap();
    let row = catalog.resolve_selection(statement.selection()).unwrap();
    row.validate_opening_layout(&layout).unwrap();
    layout.validate_basis(BasisMode::Lagrange).unwrap();
    let (_, descriptor) = transcript_instance_descriptor::<F, OrdinaryControl>(
        setup, &layout, statement.selection(), row.schedule(), BasisMode::Lagrange,
    ).unwrap();
    let endpoint_binding = encode_opening_endpoint_binding(
        &descriptor, row.profiles(), statement.claims(), |group| group.commitment(),
    ).unwrap();
    let augmentation = InheritedAugmentation {
        literal_history: &[],
        endpoint_binding: &endpoint_binding,
    };
    let augmented_bytes = u64::try_from(augmentation.encoded_len(&descriptor).unwrap()).unwrap();
    let registry = approved_native_registry_from_trusted_bounds::<OrdinaryControl>(
        &row, BasisMode::Lagrange, u64::try_from(SESSION.len()).unwrap(), augmented_bytes, 0,
    ).unwrap();
    NativeProofContext { registry, endpoint_binding }
}

#[test]
#[ignore = "real frozen-profile planning and complete ordinary proof; run explicitly with --release"]
fn original_planes_complete_ordinary_proof() {
    const STACK: usize = 256 * 1024 * 1024;
    #[cfg(feature = "parallel")]
    rayon::ThreadPoolBuilder::new().stack_size(STACK).build_global().ok();
    std::thread::Builder::new().stack_size(STACK).spawn(run_complete_control).unwrap().join().unwrap();
}

fn run_complete_control() {
    let compact = akita_config::test_support::workspace_schedule_catalog::<CompactInner>().unwrap();
    let (key, contracts) = CompactInner::root_inputs().unwrap();
    let compact_row = compact.resolve_key(&key).unwrap();
    let profiles = CommittedGroupBatchProfile {
        final_group: compact_row.schedule().root.params.own_group().profile,
        precommitteds: key.precommitteds[1..].to_vec(),
    };
    assert_profile(&profiles.final_group, 56, 512, 1, 1);
    assert_eq!(profiles.precommitteds.len(), 2);
    for profile in &profiles.precommitteds {
        assert_profile(profile, 1, 32, 16, 26);
    }
    let policy = policy_of::<OrdinaryControl>();
    let source_views = [SourceOpeningView::Decoded; 3];
    let planned = akita_planner::find_schedule_for_frozen_profiles(
        &profiles,
        CompactInner::committed_source_contract().unwrap(),
        &contracts[1..],
        &source_views,
        &policy,
        OrdinaryControl::ring_challenge_config,
    ).unwrap();
    let schedule = &planned.schedule;
    let packing = OpeningMethod::SubringCoefficientPacking { challenge_subring_dimension: D };
    for group in schedule.root.params.precommitted_groups().iter()
        .chain(std::iter::once(schedule.root.params.own_group()))
    {
        assert_eq!(group.opening.opening_method, packing);
        assert_eq!(group.opening.fold_challenge_config, OrdinaryControl::ring_challenge_config(D).unwrap());
    }
    assert!(!schedule.recursive_folds.is_empty());
    assert_eq!(schedule.recursive_folds[0].params.opening_method(), packing);
    for fold in schedule.recursive_folds.iter().skip(1) {
        assert_eq!(fold.params.opening_method(), OpeningMethod::EvaluationTrace);
    }
    assert_eq!(schedule.terminal.opening_method, OpeningMethod::EvaluationTrace);
    let catalog = TrustedScheduleCatalog::<OrdinaryControl>::new(
        ValidatedScheduleCatalog::try_new(
            OrdinaryControl::schedule_family_name(),
            [(profiles.clone(), planned.schedule)],
            &policy,
            OrdinaryControl::ring_challenge_config,
        ).unwrap(),
    ).unwrap();

    // Provision one seed-derived matrix prefix large enough for both consumers.
    // The installer verifies the seed and literal registered roles.
    let requirements = SetupRequirements::from_catalog(&compact, 22, 59).unwrap()
        .union(SetupRequirements::from_catalog(&catalog, 22, 59).unwrap()).unwrap();
    assert!(requirements.prefix_slot_ids().is_empty());
    let setup = AkitaProverSetup::<F>::generate_with_capacity(22, 59, requirements.matrix_capacity()).unwrap();
    let installed = Arc::new(install_compact_inner_setup(
        &compact, [42; 32], Arc::from(&b"original-plane-control/test-installation"[..]),
        setup.expanded.clone(),
    ).unwrap());
    let producer = CpuBackend::<F, E2>::from_full_ring_setup(installed.clone()).unwrap();
    let ordinary = CpuBackend::<F, F>::from_full_ring_setup(installed.clone()).unwrap();
    let point: Vec<_> = (0..NV).map(|bit| F::from_u64(3 + bit as u64)).collect();
    let points = vec![point.clone(), point.clone(), point];
    let mut outputs = Vec::with_capacity(3);
    let mut values = Vec::with_capacity(3);
    for alternate in [false, true] {
        let digits = ram_digits(alternate);
        values.push(vec![ram_opening(&digits, &points[outputs.len()])]);
        let source = producer.import_source_full_ring::<PackedRamSource>(FullRingSourceId(2), vec![
            PackedRamSource::from_signed_digits(NV, ram_plan(), digits).unwrap(),
        ]).unwrap();
        outputs.push(producer.commit_full_ring(&source, profiles.precommitteds[outputs.len()], contracts[1]).unwrap());
    }
    assert_eq!(values[0], values[1]);
    assert_ne!(outputs[0].committed_group.commitment(), outputs[1].committed_group.commitment());
    let mut main_values = Vec::with_capacity(56);
    let main: Vec<_> = (0..56).map(|polynomial| {
        let mut indices = vec![None; 1 << NV];
        let mut value = F::zero();
        for ring in 0..512 {
            let index = ring * D + (ring * 17 + polynomial * 11) % D;
            indices[index] = Some(0u8);
            value += weight(&points[2], index);
        }
        main_values.push(value);
        OneHotPoly::<F, u8>::new(1, indices).unwrap()
    }).collect();
    let main = producer.import_source_full_ring::<OneHotPoly<F, u8>>(FullRingSourceId(1), main).unwrap();
    let mut wrong_profile = profiles.final_group;
    wrong_profile.group = akita_params::PolynomialGroupLayout::new(NV - 1, 56);
    assert!(producer.commit_full_ring(&main, wrong_profile, CompactInner::committed_source_contract().unwrap()).is_err());
    outputs.push(producer.commit_full_ring(&main, profiles.final_group, CompactInner::committed_source_contract().unwrap()).unwrap());
    values.push(main_values);

    let snapshot: Vec<_> = (0..26).map(|plane| {
        producer.full_ring_source_reader(&outputs[1].private_handle).unwrap()
            .coefficient(plane * D + 37).unwrap()
    }).collect();
    let another_owner = CpuBackend::<F, E2>::from_full_ring_setup(installed).unwrap();
    assert!(another_owner.full_ring_source_reader(&outputs[1].private_handle).is_err());
    let uninstalled = CpuBackend::<F, F>::new(setup.expanded.clone()).unwrap();
    assert!(uninstalled.import_full_ring_commitment(&outputs[1].private_handle).is_err());
    let mut wrong_descriptor = setup.expanded.descriptor().clone();
    wrong_descriptor.setup_seed = akita_types::AkitaSetupSeed::shake256_paged_v1([43; 32]);
    let wrong_matrix = akita_types::derive_public_matrix_prefix::<F>(
        wrong_descriptor.num_field_elements, &wrong_descriptor.setup_seed,
    );
    let wrong_setup = Arc::new(akita_types::AkitaExpandedSetup::from_trusted_seed_derived_parts_unchecked(
        wrong_descriptor, wrong_matrix,
    ));
    let wrong_installation = Arc::new(install_compact_inner_setup(
        &compact, [42; 32], Arc::from(&b"original-plane-control/test-installation"[..]),
        wrong_setup.clone(),
    ).unwrap());
    let wrong_receiver = CpuBackend::<F, F>::from_full_ring_setup(wrong_installation).unwrap();
    assert!(wrong_receiver.import_full_ring_commitment(&outputs[1].private_handle).is_err());
    drop(wrong_receiver);
    drop(wrong_setup);

    let handles: Vec<_> = outputs.iter().map(|output| {
        let imported = ordinary.import_full_ring_commitment(&output.private_handle).unwrap();
        assert_eq!(imported.producer_contract(), output.private_handle.producer_contract());
        assert_eq!(imported.metadata(), output.private_handle.metadata());
        imported
    }).collect();
    let left_ram = ordinary.original_ram_plane_reader(&handles[0]).unwrap();
    let right_ram = ordinary.original_ram_plane_reader(&handles[1]).unwrap();
    assert_eq!(left_ram.source_id(), right_ram.source_id(), "installed role is reused");
    assert_ne!(left_ram.commitment_id(), right_ram.commitment_id(), "retained originals remain distinct");
    assert_eq!(left_ram.backend_id(), right_ram.backend_id());
    assert_eq!(left_ram.setup_digest(), right_ram.setup_digest());
    let cloned_handle = handles[0].clone();
    let alias_ram = ordinary.original_ram_plane_reader(&cloned_handle).unwrap();
    assert_eq!(left_ram.commitment_id(), alias_ram.commitment_id());
    let main_identity = ordinary.original_main_component_reader(&handles[2], 0).unwrap().commitment_id();
    for column in 0..56 {
        let native = producer.original_main_component_reader(&outputs[2].private_handle, column).unwrap();
        let projected = ordinary.original_main_component_reader(&handles[2], column).unwrap();
        assert_eq!(native.source_id(), FullRingSourceId(1));
        assert_eq!(projected.source_id(), native.source_id());
        assert_eq!(projected.column(), column);
        assert_eq!(projected.len(), 1 << NV);
        assert_eq!(projected.commitment_id(), main_identity);
        assert_eq!(projected.backend_id(), left_ram.backend_id());
        assert_eq!(projected.setup_digest(), left_ram.setup_digest());
        for ring in [0, 63, 511] {
            for coefficient in 0..128 {
                let index = ring * D + coefficient;
                let expected = if coefficient == (ring * 17 + column * 11) % D {
                    F::one()
                } else { F::zero() };
                assert_eq!(native.coefficient(index).unwrap(), expected);
                assert_eq!(projected.coefficient(index).unwrap(), expected);
            }
        }
        assert!(projected.coefficient(projected.len()).is_err());
    }
    assert!(ordinary.original_main_component_reader(&handles[2], 56).is_err());
    assert!(ordinary.original_main_component_reader(&handles[0], 0).is_err());
    assert!(ordinary.original_ram_plane_reader(&handles[2]).is_err());
    assert!(uninstalled.original_main_component_reader(&handles[2], 0).is_err());
    let commitments: Vec<_> = outputs.iter().map(|output| output.committed_group.clone()).collect();
    let claims = || OpeningClaims::from_groups(commitments.iter().zip(&points).zip(&values).map(
        |((commitment, point), values)| PolynomialGroupClaims::new(point.clone(), values.clone(), commitment.clone()).unwrap(),
    ).collect()).unwrap();
    let prefixes = SetupPrefixProverRegistry::default();
    // Install the complete native grammar before proving, solely from the
    // trusted row and public opening statement, never from observed events.
    let prover_context = native_context(
        setup.expanded.descriptor(), &catalog,
        &statement(&catalog, &profiles, &commitments, &points, &values),
    );
    // A valid projection is still receiver-owned: another ordinary backend
    // sharing exactly the same public prefix cannot use its private handles.
    let foreign_opening = SelectedProverOpeningData::from_committed_claims::<OrdinaryControl>(
        claims(), handles.clone(), &catalog,
    ).unwrap();
    assert!(batched_prove::<OrdinaryControl, _>(
        setup.expanded.descriptor(), &prefixes, &catalog, &uninstalled,
        foreign_opening, SESSION, &prover_context.registry,
        prover_context.augmentation(), BasisMode::Lagrange,
    ).is_err());
    // Equal decoded claims and identical profiles do not authorize replacing
    // either retained A-input path with the other RAM's private handle.
    let mut swapped_handles = handles.clone();
    swapped_handles.swap(0, 1);
    let swapped = SelectedProverOpeningData::from_committed_claims::<OrdinaryControl>(
        claims(), swapped_handles, &catalog,
    ).and_then(|opening| batched_prove::<OrdinaryControl, _>(
        setup.expanded.descriptor(), &prefixes, &catalog, &ordinary,
        opening, SESSION, &prover_context.registry,
        prover_context.augmentation(), BasisMode::Lagrange,
    ));
    assert!(swapped.is_err());
    let opening = SelectedProverOpeningData::from_committed_claims::<OrdinaryControl>(claims(), handles, &catalog).unwrap();
    let proof = batched_prove::<OrdinaryControl, _>(
        setup.expanded.descriptor(), &prefixes, &catalog, &ordinary,
        opening, SESSION, &prover_context.registry,
        prover_context.augmentation(), BasisMode::Lagrange,
    ).unwrap();
    let verifier = AkitaVerifier::new(setup.to_verifier_setup(requirements.matrix_capacity()).unwrap(), catalog.clone()).unwrap();
    // Each expected context is independently rebuilt from the supplied public
    // statement, including mutated statements. No prover handles are consulted.
    let verify = |proof: &[u8], commitments: &[CommittedGroup<F>], values: &[Vec<F>]| {
        let public_statement = statement(&catalog, &profiles, commitments, &points, values);
        let context = native_context(setup.expanded.descriptor(), &catalog, &public_statement);
        verifier.batched_verify(
            proof, SESSION, &context.registry, context.augmentation(),
            public_statement, BasisMode::Lagrange,
        )
    };
    verify(&proof, &commitments, &values).unwrap();

    // The complete unmodified verifier, not merely the reduction, is the anchor.
    let mut bad_values = values.clone();
    bad_values[0][0] += F::one();
    assert!(verify(&proof, &commitments, &bad_values).is_err());
    let mut substituted = commitments.clone();
    substituted.swap(0, 1);
    assert!(verify(&proof, &substituted, &values).is_err());
    assert!(verify(&proof[..proof.len() - 1], &commitments, &values).is_err());
    for (plane, expected) in snapshot.into_iter().enumerate() {
        assert_eq!(producer.full_ring_source_reader(&outputs[1].private_handle).unwrap()
            .coefficient(plane * D + 37).unwrap(), expected);
    }
    eprintln!("complete original-plane ordinary proof: {} bytes; {} recursive folds", proof.len(), catalog.resolve_profiles(&profiles, &source_views).unwrap().schedule().recursive_folds.len());
}
