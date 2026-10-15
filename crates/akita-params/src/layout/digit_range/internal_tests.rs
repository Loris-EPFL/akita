use super::*;
use crate::sis::{
    PhysicalL2NormProofShape, SisL2TableDigest, SisL2TableKey, SisModulusProfileId,
    SisSecurityPolicyId,
};

#[test]
fn compact_route_shape_matches_materialized_shapes_and_rejects_bad_norms() {
    let key = SisL2TableKey {
        policy: SisSecurityPolicyId::Quantum128BitADPS16,
        table_digest: SisL2TableDigest::CURRENT,
        modulus_profile: SisModulusProfileId::Q128OffsetA7F7,
        ring_dimension: 64,
        collision_l2_sq: 1,
    };
    let shapes = [
        PhysicalL2NormProofShape::Direct {
            physical_response_len: 512,
        },
        PhysicalL2NormProofShape::LimbGram {
            physical_response_len: 512,
            block_len: 32,
            limb_count: 3,
        },
    ];
    for basis in [4, 8, 16, 32, 64] {
        let plan = DigitRangePlan::new(basis).unwrap();
        for rounds in [0, 1, 7, 32] {
            for norm in shapes {
                let route = InnerCommitSecurityRoute::L2 {
                    table_key: key,
                    response_l2_sq_cap: 1,
                    norm_proof_shape: norm,
                };
                let compact = plan.route_shape(rounds, route).unwrap();
                let (stages, wire_norm) = plan.proof_shapes_for_route(rounds, route).unwrap();
                // Reconstruct the former materialized rule independently.
                let expected: Vec<_> = plan
                    .stage_shapes(rounds)
                    .into_iter()
                    .take(plan.product_stage_arities().len())
                    .collect();
                assert_eq!(compact.stages().collect::<Vec<_>>(), expected);
                assert_eq!(stages, expected);
                let compact_norm = compact.norm.unwrap();
                let wire_norm = wire_norm.unwrap();
                assert_eq!(wire_norm.subclaims, norm.subclaim_count().unwrap());
                assert_eq!(
                    wire_norm.virtual_evaluations,
                    norm.virtual_evaluation_count()
                );
                assert_eq!(wire_norm.sumcheck, vec![plan.leaf_degree() + 1; rounds]);
                assert_eq!(compact_norm.subclaims, wire_norm.subclaims);
                assert_eq!(
                    compact_norm.virtual_evaluations,
                    wire_norm.virtual_evaluations
                );
                assert_eq!(
                    vec![compact_norm.degree; compact_norm.rounds],
                    wire_norm.sumcheck
                );
            }
        }
        let bad = InnerCommitSecurityRoute::L2 {
            table_key: key,
            response_l2_sq_cap: 1,
            norm_proof_shape: PhysicalL2NormProofShape::Direct {
                physical_response_len: 0,
            },
        };
        assert!(plan.route_shape(7, bad).is_err());
        assert!(plan.proof_shapes_for_route(7, bad).is_err());
    }
}

#[test]
fn full_ring_range_wire_is_two_full_coefficient_stages() {
    assert_eq!(FullRingRangePlan::PRODUCT_ARITIES, [4, 4]);
    assert_eq!(FullRingRangePlan::NODE_COUNTS, [1, 4, 16]);
    assert_eq!(FullRingRangePlan::ROUND_DEGREES, [5, 5]);
    assert_eq!(FullRingRangePlan::FUSED_DEGREE, 3);
    assert_eq!(FullRingRangePlan::ENDPOINT_VALUES, 6);
    assert_eq!(
        FullRingRangePlan::full_coefficient_count(7).unwrap(),
        (6 + 6 + 4) * 7
    );
    assert!(FullRingRangePlan::full_coefficient_count(usize::MAX).is_err());
}
