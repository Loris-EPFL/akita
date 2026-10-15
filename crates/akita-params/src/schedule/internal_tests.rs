use super::*;
use crate::test_fixtures::committed_params;

#[test]
fn accepts_group_local_packing_subring_dimensions() {
    let packing_group = |s| {
        let mut params = committed_params(128);
        params.own_group_mut().opening.opening_method =
            crate::OpeningMethod::SubringCoefficientPacking {
                challenge_subring_dimension: s,
            };
        params.own_group_mut().opening.fold_challenge_config =
            akita_challenges::SparseChallengeConfig::production_for_ring_dim(s).unwrap();
        params.source_encoding = crate::CommittedSourceEncoding::CanonicalCoefficientTable;
        params
    };
    let s64 = packing_group(64);
    let s128 = packing_group(128);
    let groups = [
        OpeningExecutionGroup {
            opening_method: s64.opening_method(),
            inner_commit_matrix: &s64.inner().matrix,
            fold_challenge_config: s64.fold_challenge_config(),
            source_encoding: s64.source_encoding,
            expected_source_encoding: None,
        },
        OpeningExecutionGroup {
            opening_method: s128.opening_method(),
            inner_commit_matrix: &s128.inner().matrix,
            fold_challenge_config: s128.fold_challenge_config(),
            source_encoding: s128.source_encoding,
            expected_source_encoding: None,
        },
    ];
    validate_level_opening_execution(0, 1, &groups)
        .expect("each root group owns its challenge subring dimension");
}

#[test]
fn full_ring_geometry_keeps_native_ring_and_both_planes() {
    let method = OpeningMethod::FULL_RING_E2;
    assert_eq!(method.physical_coefficient_width(2, 128).unwrap(), 256);
    assert!(!method.requires_extension_opening_reduction(2));
    assert!(method.physical_coefficient_width(1, 128).is_err());
    assert!(method.physical_coefficient_width(2, 256).is_err());
    assert_eq!(
        crate::opening_d_segment_width(method, 2, 128, 128, 26, 8, 3).unwrap(),
        52 * 8 * 3
    );
    assert!(crate::opening_d_segment_width(method, 2, 128, 512, 26, 8, 3).is_err());
    for is_root in [false, true] {
        assert_eq!(
            CommittedSourceEncoding::for_producer(method, 2, 128, 19, is_root),
            CommittedSourceEncoding::CanonicalCoefficientTable
        );
    }
}

#[test]
fn full_ring_integer_codec_and_all_valid_depth_boundaries() {
    assert_eq!(
        TerminalResponseCodec::full_ring(8).unwrap(),
        TerminalResponseCodec::SignedFixedWidth {
            bytes_per_coefficient: 2,
            coefficient_bound: 7168
        }
    );
    assert_eq!(
        TerminalResponseCodec::full_ring(64).unwrap(),
        TerminalResponseCodec::SignedFixedWidth {
            bytes_per_coefficient: 3,
            coefficient_bound: 57344
        }
    );
    assert!(TerminalResponseCodec::full_ring(0).is_err());
    for (bound, depth) in [(0, 1), (15, 1), (16, 2), (495, 2), (496, 3), (7168, 3)] {
        assert_eq!(full_ring_response_depth(bound).unwrap(), depth);
    }
    assert!(full_ring_response_depth(u128::MAX).is_err());
    assert_eq!(FULL_RING_FOLD_CHALLENGE.l1_norm(), 56);
    assert_ne!(
        Some(FULL_RING_FOLD_CHALLENGE),
        akita_challenges::SparseChallengeConfig::production_for_ring_dim(128)
    );
}

#[test]
fn coefficient_rows_are_not_an_alpha_suffix() {
    let topology = crate::RelationCandidateTopology::new(false, OpeningMethod::FULL_RING_E2);
    assert_eq!(
        crate::RingRelationPhase::QuotientPrefix.candidate_modes(0, topology),
        &[crate::RingRelationMode::ExactCoefficientRows]
    );
    assert!(crate::RingRelationPhase::QuotientPrefix
        .candidate_modes(
            0,
            crate::RelationCandidateTopology::new(true, OpeningMethod::FULL_RING_E2)
        )
        .is_empty());
    assert!(crate::ring_switch_alpha_loss_factor(OpeningMethod::FULL_RING_E2, 128).is_err());
    assert!(!crate::RingRelationMode::ExactCoefficientRows.has_quotient_witness());
}
