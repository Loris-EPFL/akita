use super::*;
use akita_error::AkitaError;
use akita_sumcheck::SumcheckKernel;
use akita_types::{full_ring::FullRingCoefficientSource, proof::FullRingSourceId};
use jolt_field::{CanonicalBytes, One, Ring, Zero};
type F = akita_config::proof_optimized::fp128::Field;

// Test-only material oracle; production kernels exclusively borrow CPU readers.
struct ReferenceSource {
    id: SourceOwnership,
    values: Vec<F>,
}
impl FullRingCoefficientSource<F> for ReferenceSource {
    fn len(&self) -> usize {
        self.values.len()
    }
    fn coefficient(&self, index: usize) -> Result<F, AkitaError> {
        self.values
            .get(index)
            .copied()
            .ok_or_else(|| invalid("reference source bounds"))
    }
}
impl SourceProvider<F> for ReferenceSource {
    fn source_count(&self) -> usize {
        1
    }
    fn source_id(&self, slot: usize) -> Result<(SourceOwnership, usize), AkitaError> {
        if slot != 0 {
            return Err(invalid("reference slot"));
        }
        Ok((self.id, 0))
    }
    fn source(&self, slot: usize) -> Result<&dyn FullRingCoefficientSource<F>, AkitaError> {
        if slot != 0 {
            return Err(invalid("reference slot"));
        }
        Ok(self)
    }
}
fn limits(n: usize) -> LedgerLimits {
    LedgerLimits {
        claim_horizon: 32,
        max_sources: 8,
        max_terms_per_claim: 8,
        max_recipe_terms: 32,
        max_evidence_bytes: 4096,
        common_num_vars: n,
        max_cube_cells: 1 << n,
    }
}
fn native(commitment_id: u32) -> SourceOwnership {
    SourceOwnership::NativeInstalled {
        source_id: FullRingSourceId(2),
        commitment_id: commitment_id as u128,
        backend_id: 1,
        setup_digest: [1; 32],
    }
}
fn source(id: u32, view: SourceView) -> ApprovedSource {
    ApprovedSource {
        local_ownership: Some(native(id)),
        component: 0,
        canonical_identity: vec![id as u8, 1, 2],
        view,
    }
}
fn claim(exponent: usize, point: Vec<F>, value: F) -> ScalarClaim<F> {
    ScalarClaim {
        exponent,
        value,
        offset: F::zero(),
        terms: vec![QueryTerm {
            source: 0,
            coefficient: F::one(),
            recipe: QueryRecipe::Eq { point },
        }],
    }
}

#[test]
fn scalar_exponents_seal_and_foreign_source_are_checked() {
    let mut ledger = QueryLedger::<F>::new(
        vec![source(1, SourceView::Decoded { num_vars: 1 })],
        limits(2),
        b"approved".to_vec(),
    )
    .unwrap();
    ledger
        .append(claim(0, vec![F::zero()], F::from_u64(9)))
        .unwrap();
    let before = ledger.canonical_bytes().unwrap();
    assert!(ledger.append(claim(0, vec![F::one()], F::one())).is_err());
    assert_eq!(before, ledger.canonical_bytes().unwrap());
    let mut foreign = claim(1, vec![F::zero()], F::zero());
    foreign.terms[0].source = 1;
    assert!(ledger.append(foreign).is_err());
    ledger.bind_evidence(b"full replayed frontend").unwrap();
    let sealed = ledger.seal().unwrap();
    assert!(sealed.windows(22).any(|w| w == b"full replayed frontend"));
    assert!(ledger.append(claim(1, vec![F::one()], F::zero())).is_err());
    assert!(ledger.bind_evidence(b"late").is_err());
}

#[test]
fn weighted_query_is_exact_and_preserves_actual_padding() {
    let provider = ReferenceSource {
        id: native(7),
        values: vec![F::from_u64(3), -F::one()],
    };
    let mut ledger = QueryLedger::new(
        vec![source(7, SourceView::Decoded { num_vars: 1 })],
        limits(2),
        b"trusted".to_vec(),
    )
    .unwrap();
    ledger
        .append(claim(0, vec![F::zero()], F::from_u64(3)))
        .unwrap();
    ledger.append(claim(1, vec![F::one()], -F::one())).unwrap();
    ledger.seal().unwrap();
    for a in [F::zero(), F::from_u64(11)] {
        let mut kernel = WeightedQueryKernel::new(&ledger, &provider, a).unwrap();
        let mut running = kernel.input_claim();
        assert!(kernel.bind_challenge(0, F::one()).is_err());
        for (round, r) in [F::from_u64(5), F::from_u64(13)].into_iter().enumerate() {
            let p = kernel.round_polynomial(round, running).unwrap();
            assert_eq!(p.evaluate(F::zero()) + p.evaluate(F::one()), running);
            running = p.evaluate(r);
            kernel.bind_challenge(round, r).unwrap();
        }
        kernel.finish().unwrap();
        let endpoints = kernel.endpoints().unwrap();
        // Nonzero real padding at address1 was not deleted or forced to zero.
        assert_eq!(endpoints[0].value, F::from_u64(3) - F::from_u64(20));
        ledger
            .verify_query_endpoint(a, kernel.point(), &endpoints, running)
            .unwrap();
        assert!(ledger
            .verify_query_endpoint(a, kernel.point(), &[], running)
            .is_err());
        let mut forged = endpoints.clone();
        forged[0].local_ownership = Some(native(8));
        assert!(ledger
            .verify_query_endpoint(a, kernel.point(), &forged, running)
            .is_err());
    }
    let foreign = ReferenceSource {
        id: native(8),
        values: provider.values.clone(),
    };
    assert!(WeightedQueryKernel::new(&ledger, &foreign, F::zero()).is_err());
}

#[test]
fn signed_plane_boundaries_and_noncanonical_plane25() {
    for value in [-F::from_u64(16), -F::one(), F::zero(), F::from_u64(15)] {
        assert_eq!(signed_plane_residual(value), F::zero());
    }
    for value in [-F::from_u64(17), F::from_u64(16)] {
        assert_ne!(signed_plane_residual(value), F::zero());
    }
    let mut decoded = F::zero();
    let mut power = F::one();
    for ell in 0..26 {
        let digit = match ell {
            0 => F::from_u64(9),
            2 => -F::from_u64(10),
            3 => F::one(),
            6 => -F::from_u64(4),
            25 => F::from_u64(8),
            _ => F::zero(),
        };
        assert_eq!(signed_plane_residual(digit), F::zero());
        decoded += power * digit;
        power *= F::from_u64(32);
    }
    assert_eq!(decoded, F::zero());
    let adjoint = QueryRecipe::DecodedRam {
        point: vec![F::zero(); 16],
        address_axes: (0..7).chain(12..21).collect(),
    };
    let mut raw = vec![F::zero(); 21];
    for bit in 0..5 {
        raw[7 + bit] = if (25 >> bit) & 1 == 0 {
            F::zero()
        } else {
            F::one()
        };
    }
    let expected = (0..25).fold(F::one(), |v, _| v * F::from_u64(32));
    assert_eq!(adjoint.evaluate(&raw).unwrap(), expected);
    let view = SourceView::OriginalRamPlanesV1;
    assert_eq!(view.physical_index(25 << 7).unwrap(), Some(25 * 128));
    assert_eq!(view.physical_index(26 << 7).unwrap(), None);
    assert_eq!(view.physical_index(32 << 7).unwrap(), Some(26 * 128));
}

#[test]
fn masked_equality_is_a_table_lift_not_offcube_product() {
    let recipe = QueryRecipe::MaskedEq {
        point: vec![F::from_u64(3)],
        intervals: vec![0..1],
    };
    let x = F::from_u64(5);
    assert_eq!(
        recipe.evaluate(&[x]).unwrap(),
        (F::one() - F::from_u64(3)) * (F::one() - x)
    );
    assert_ne!(
        recipe.evaluate(&[x]).unwrap(),
        (F::one() - x) * equality(&[F::from_u64(3)], &[x]).unwrap()
    );
}

#[test]
fn all_range_sources_append_even_at_zero_selector_weight() {
    let mut ledger = QueryLedger::<F>::new(
        vec![
            source(1, SourceView::OriginalRamPlanesV1),
            source(2, SourceView::OriginalRamPlanesV1),
        ],
        limits(21),
        b"trusted".to_vec(),
    )
    .unwrap();
    ledger
        .append(claim(0, vec![F::zero(); 21], F::zero()))
        .unwrap();
    assert!(ledger.seal().is_err());
    let range = GlobalRamRange::new(&ledger, vec![0, 1], vec![0, 1, 0], 1 << 22).unwrap();
    assert_eq!(range.degree_charge().unwrap(), 12 * 22 + 3);
    let point = vec![F::zero(); 22];
    let t = F::from_u64(6);
    let gamma = F::from_u64(7);
    let leaf = range
        .verify_leaf_endpoint(point.clone(), gamma, quartic_batch(gamma, t), t)
        .unwrap();
    let endpoints = vec![
        SourceEndpoint {
            source: 0,
            local_ownership: Some(native(1)),
            component: 0,
            value: F::from_u64(2),
        },
        SourceEndpoint {
            source: 1,
            local_ownership: Some(native(2)),
            component: 0,
            value: F::from_u64(9),
        },
    ];
    assert!(range
        .verify_product_endpoint(point.clone(), F::one(), [F::zero(); 4])
        .is_err());
    assert!(range
        .verify_leaf_endpoint(point.clone(), gamma, F::zero(), t)
        .is_err());
    assert!(range
        .verify_link_endpoint(&leaf, &point, F::from_u64(7), &endpoints)
        .is_err());
    assert!(range
        .verify_link_endpoint(&leaf, &point, t, &endpoints[..1])
        .is_err());
    let mut reordered = endpoints.clone();
    reordered.reverse();
    assert!(range
        .verify_link_endpoint(&leaf, &point, t, &reordered)
        .is_err());
    range
        .append_endpoints(&mut ledger, &leaf, &point, t, &endpoints)
        .unwrap();
    assert_eq!(ledger.claims().len(), 3);
    assert_eq!(ledger.claims()[2].value, F::from_u64(9));
    assert!(range
        .append_endpoints(&mut ledger, &leaf, &point, t, &endpoints)
        .is_err());
    ledger.seal().unwrap();
}

#[test]
fn zero_query_weight_does_not_remove_endpoint_ownership() {
    let mut ledger = QueryLedger::<F>::new(
        vec![
            source(1, SourceView::Decoded { num_vars: 1 }),
            source(2, SourceView::Decoded { num_vars: 1 }),
        ],
        limits(1),
        b"trusted".to_vec(),
    )
    .unwrap();
    let mut assertion = claim(0, vec![F::zero()], F::zero());
    assertion.terms[0].coefficient = F::zero();
    ledger.append(assertion).unwrap();
    ledger.seal().unwrap();
    let endpoints = vec![
        SourceEndpoint {
            source: 0,
            local_ownership: Some(native(1)),
            component: 0,
            value: F::from_u64(3),
        },
        SourceEndpoint {
            source: 1,
            local_ownership: Some(native(2)),
            component: 0,
            value: F::from_u64(5),
        },
    ];
    ledger
        .verify_query_endpoint(F::zero(), &[F::from_u64(7)], &endpoints, F::zero())
        .unwrap();
    assert!(ledger
        .verify_query_endpoint(F::zero(), &[F::from_u64(7)], &endpoints[..1], F::zero())
        .is_err());
}

#[test]
fn private_dense_capability_is_not_public_protocol_bytes() {
    let mut original = source(1, SourceView::Decoded { num_vars: 1 });
    original.local_ownership = Some(SourceOwnership::RetainedDenseCommitment {
        commitment_id: 19,
        backend_id: 1,
        setup_digest: [1; 32],
    });
    let mut other_backend = original.clone();
    other_backend.local_ownership = Some(SourceOwnership::RetainedDenseCommitment {
        commitment_id: 23,
        backend_id: 2,
        setup_digest: [2; 32],
    });
    let one = QueryLedger::<F>::new(
        vec![original],
        limits(1),
        b"same approved public C".to_vec(),
    )
    .unwrap();
    let two = QueryLedger::<F>::new(
        vec![other_backend],
        limits(1),
        b"same approved public C".to_vec(),
    )
    .unwrap();
    assert_eq!(
        one.canonical_bytes().unwrap(),
        two.canonical_bytes().unwrap()
    );
}

#[test]
fn reusable_native_role_does_not_conflate_distinct_retained_originals() {
    let sources = vec![
        source(1, SourceView::OriginalRamPlanesV1),
        source(2, SourceView::OriginalRamPlanesV1),
    ];
    // Both have installed role2, but actual retained commitments differ.
    assert!(matches!(
        sources[0].local_ownership,
        Some(SourceOwnership::NativeInstalled {
            source_id: FullRingSourceId(2),
            ..
        })
    ));
    assert!(matches!(
        sources[1].local_ownership,
        Some(SourceOwnership::NativeInstalled {
            source_id: FullRingSourceId(2),
            ..
        })
    ));
    let ledger =
        QueryLedger::<F>::new(sources, limits(21), b"approved two RAM originals".to_vec()).unwrap();
    let range = GlobalRamRange::new(&ledger, vec![0, 1], vec![0, 1], 1 << 22).unwrap();
    assert_eq!(range.ram_slots(), &[0, 1]);
}

#[test]
fn independent_verifier_uses_public_roster_without_fake_private_capability() {
    let original = source(1, SourceView::Decoded { num_vars: 1 });
    let mut public = original.clone();
    public.local_ownership = None;
    let mut prover =
        QueryLedger::<F>::new(vec![original], limits(1), b"approved C".to_vec()).unwrap();
    let mut verifier =
        QueryLedger::<F>::new(vec![public], limits(1), b"approved C".to_vec()).unwrap();
    prover
        .append(claim(0, vec![F::zero()], F::from_u64(3)))
        .unwrap();
    verifier
        .append(claim(0, vec![F::zero()], F::from_u64(3)))
        .unwrap();
    assert_eq!(prover.seal().unwrap(), verifier.seal().unwrap());
    let endpoint = SourceEndpoint {
        source: 0,
        local_ownership: None,
        component: 0,
        value: F::from_u64(3),
    };
    verifier
        .verify_query_endpoint(F::one(), &[F::zero()], &[endpoint], F::from_u64(3))
        .unwrap();
    let fake_provider = ReferenceSource {
        id: native(1),
        values: vec![F::from_u64(3), F::zero()],
    };
    assert!(WeightedQueryKernel::new(&verifier, &fake_provider, F::one()).is_err());
}

#[test]
fn public_repeated_commitments_do_not_invent_private_aliases() {
    let one = source(1, SourceView::Decoded { num_vars: 1 });
    let mut two = one.clone();
    two.local_ownership = Some(native(2));
    let proving = QueryLedger::<F>::new(
        vec![one.clone(), two.clone()],
        limits(1),
        b"approved ordered repeated C".to_vec(),
    )
    .unwrap();
    let mut public_one = one;
    public_one.local_ownership = None;
    two.local_ownership = None;
    let verifying = QueryLedger::<F>::new(
        vec![public_one, two],
        limits(1),
        b"approved ordered repeated C".to_vec(),
    )
    .unwrap();
    assert_eq!(
        proving.canonical_bytes().unwrap(),
        verifying.canonical_bytes().unwrap()
    );
    let duplicate = source(1, SourceView::Decoded { num_vars: 1 });
    assert!(QueryLedger::<F>::new(
        vec![duplicate.clone(), duplicate],
        limits(1),
        b"duplicate actual capability".to_vec()
    )
    .is_err());
}

#[test]
fn canonical_transport_bound_covers_all_admitted_recipe_codecs() {
    let installed = limits(21);
    let mut ledger = QueryLedger::<F>::new(
        vec![source(1, SourceView::OriginalRamPlanesV1)],
        installed,
        b"context".to_vec(),
    )
    .unwrap();
    let recipes = [
        QueryRecipe::Eq {
            point: vec![F::from_u64(3); 21],
        },
        QueryRecipe::MaskedEq {
            point: vec![F::from_u64(5); 21],
            intervals: vec![0..1, 2..3],
        },
        QueryRecipe::DecodedRam {
            point: vec![F::from_u64(7); 16],
            address_axes: (0..7).chain(12..21).collect(),
        },
        QueryRecipe::Sparse {
            entries: vec![(0, F::one()), (1, F::from_u64(9))],
        },
    ];
    for (exponent, recipe) in recipes.into_iter().enumerate() {
        ledger
            .append(ScalarClaim {
                exponent,
                value: F::one(),
                offset: F::from_u64(2),
                terms: vec![QueryTerm {
                    source: 0,
                    coefficient: F::one(),
                    recipe,
                }],
            })
            .unwrap();
    }
    ledger.bind_evidence(b"literal framed evidence").unwrap();
    let bound =
        QueryLedger::<F>::canonical_size_bound(installed, 3, installed.max_evidence_bytes).unwrap();
    assert!(ledger.canonical_bytes().unwrap().len() <= bound);
    assert_eq!(
        bound,
        QueryLedger::<F>::canonical_size_bound(installed, 3, 0).unwrap()
    );
    assert!(QueryLedger::<F>::canonical_size_bound(installed, usize::MAX, 0).is_err());
}

#[test]
fn sealed_weighted_query_matches_ledger_for_mixed_shared_recipes_and_zero_draw() {
    struct Providers(Vec<ReferenceSource>);
    impl SourceProvider<F> for Providers {
        fn source_count(&self) -> usize {
            self.0.len()
        }
        fn source_id(&self, slot: usize) -> Result<(SourceOwnership, usize), AkitaError> {
            Ok((
                self.0
                    .get(slot)
                    .ok_or_else(|| invalid("reference slot"))?
                    .id,
                0,
            ))
        }
        fn source(&self, slot: usize) -> Result<&dyn FullRingCoefficientSource<F>, AkitaError> {
            Ok(self.0.get(slot).ok_or_else(|| invalid("reference slot"))?)
        }
    }
    let provider = Providers(vec![
        ReferenceSource {
            id: native(1),
            values: vec![F::from_u64(3), -F::from_u64(2)],
        },
        ReferenceSource {
            id: native(2),
            values: vec![F::from_u64(5), F::from_u64(7), -F::one(), F::from_u64(11)],
        },
        ReferenceSource {
            id: native(3),
            values: vec![F::from_u64(19)],
        },
        ReferenceSource {
            id: native(4),
            values: vec![-F::from_u64(13), F::from_u64(17)],
        },
    ]);
    let mut ledger = QueryLedger::new(
        vec![
            source(1, SourceView::Decoded { num_vars: 1 }),
            source(2, SourceView::Decoded { num_vars: 2 }),
            source(3, SourceView::Decoded { num_vars: 0 }),
            source(4, SourceView::Decoded { num_vars: 1 }),
        ],
        limits(5),
        b"mixed shared-source queries".to_vec(),
    )
    .unwrap();
    let claims = [
        vec![
            QueryTerm {
                source: 1,
                coefficient: F::from_u64(2),
                recipe: QueryRecipe::Eq {
                    point: vec![F::from_u64(3), F::from_u64(5)],
                },
            },
            QueryTerm {
                source: 1,
                coefficient: -F::one(),
                recipe: QueryRecipe::MaskedEq {
                    point: vec![F::from_u64(7), F::from_u64(11)],
                    intervals: vec![0..1, 2..4],
                },
            },
            QueryTerm {
                source: 1,
                coefficient: F::from_u64(3),
                recipe: QueryRecipe::Sparse {
                    entries: vec![(0, -F::from_u64(4)), (3, F::from_u64(6))],
                },
            },
            QueryTerm {
                source: 1,
                coefficient: F::from_u64(5),
                recipe: QueryRecipe::Eq {
                    point: vec![F::from_u64(3), F::from_u64(5)],
                },
            },
            QueryTerm {
                source: 2,
                coefficient: F::zero(),
                recipe: QueryRecipe::Eq { point: vec![] },
            },
            QueryTerm {
                source: 2,
                coefficient: F::from_u64(5),
                recipe: QueryRecipe::Eq { point: vec![] },
            },
        ],
        vec![
            QueryTerm {
                source: 0,
                coefficient: F::from_u64(13),
                recipe: QueryRecipe::MaskedEq {
                    point: vec![F::from_u64(2)],
                    intervals: vec![0..1],
                },
            },
            QueryTerm {
                source: 1,
                coefficient: -F::from_u64(7),
                recipe: QueryRecipe::Sparse {
                    entries: vec![(1, F::from_u64(17)), (2, -F::one())],
                },
            },
            QueryTerm {
                source: 0,
                coefficient: F::from_u64(5),
                recipe: QueryRecipe::Eq {
                    point: vec![F::one()],
                },
            },
            QueryTerm {
                source: 1,
                coefficient: F::from_u64(17),
                recipe: QueryRecipe::Eq {
                    point: vec![F::zero(), F::one()],
                },
            },
        ],
        vec![],
        vec![
            QueryTerm {
                source: 0,
                coefficient: -F::from_u64(3),
                recipe: QueryRecipe::Eq {
                    point: vec![F::from_u64(9)],
                },
            },
            QueryTerm {
                source: 1,
                coefficient: F::zero(),
                recipe: QueryRecipe::Eq {
                    point: vec![F::from_u64(13), F::from_u64(17)],
                },
            },
        ],
    ];
    for (exponent, terms) in claims.into_iter().enumerate() {
        let offset = F::from_u64(23 + exponent as u64);
        let mut value = offset;
        for term in &terms {
            let mu = ledger.sources()[term.source].view.num_vars();
            for (index, &cell) in provider.0[term.source].values.iter().enumerate() {
                let point: Vec<F> = (0..mu)
                    .map(|axis| {
                        if (index >> axis) & 1 == 0 {
                            F::zero()
                        } else {
                            F::one()
                        }
                    })
                    .collect();
                value += term.coefficient * term.recipe.evaluate(&point).unwrap() * cell;
            }
        }
        ledger
            .append(ScalarClaim {
                exponent,
                value,
                offset,
                terms,
            })
            .unwrap();
    }
    let sealed_bytes = ledger.seal().unwrap();
    let challenges = [
        [29, 31, 37, 41, 43],
        [0, 1, 0, 1, 0],
        [1, 0, 1, 0, 1],
        [3, 0, 5, 7, 0],
    ];
    for (a, challenges) in [F::zero(), F::one(), F::from_u64(7)]
        .into_iter()
        .flat_map(|a| {
            challenges
                .into_iter()
                .map(move |challenges| (a, challenges))
        })
    {
        let mut kernel = WeightedQueryKernel::new(&ledger, &provider, a).unwrap();
        let mut running = kernel.input_claim();
        for (round, challenge) in challenges.into_iter().map(F::from_u64).enumerate() {
            let polynomial = kernel.round_polynomial(round, running).unwrap();
            for t in [F::zero(), F::one(), F::from_u64(2)] {
                let mut point = kernel.point().to_vec();
                point.resize(5, F::zero());
                point[round] = t;
                let mut expected = F::zero();
                for suffix in 0..cube_len(5 - round - 1).unwrap() {
                    for (axis, x) in point.iter_mut().enumerate().skip(round + 1) {
                        *x = if (suffix >> (axis - round - 1)) & 1 == 0 {
                            F::zero()
                        } else {
                            F::one()
                        };
                    }
                    for (slot, source) in ledger.sources().iter().enumerate() {
                        let local_point = &point[..source.view.num_vars()];
                        let endpoint = provider.0[slot]
                            .values
                            .iter()
                            .enumerate()
                            .fold(F::zero(), |sum, (index, &cell)| {
                                sum + cell * eq_index(local_point, index)
                            });
                        expected += ledger.query_at(slot, a, &point).unwrap() * endpoint;
                    }
                }
                let actual = polynomial.evaluate(t);
                assert_eq!(actual, expected, "draw {a:?}, round {round}, point {t:?}");
                let mut actual_bytes = vec![0; F::NUM_BYTES];
                actual.to_bytes_le(&mut actual_bytes);
                let mut expected_bytes = vec![0; F::NUM_BYTES];
                expected.to_bytes_le(&mut expected_bytes);
                assert_eq!(actual_bytes, expected_bytes);
            }
            running = polynomial.evaluate(challenge);
            kernel.bind_challenge(round, challenge).unwrap();
        }
        kernel.finish().unwrap();
        let endpoints = kernel.endpoints().unwrap();
        assert_eq!(endpoints.len(), 4);
        for (slot, endpoint) in endpoints.iter().enumerate() {
            assert_eq!(endpoint.source, slot);
            assert_eq!(endpoint.local_ownership, Some(provider.0[slot].id));
            let local_point = &kernel.point()[..ledger.sources()[slot].view.num_vars()];
            let expected = provider.0[slot]
                .values
                .iter()
                .enumerate()
                .fold(F::zero(), |sum, (index, &cell)| {
                    sum + cell * eq_index(local_point, index)
                });
            assert_eq!(endpoint.value, expected);
        }
        assert_eq!(endpoints[2].value, F::from_u64(19));
        // An entirely unqueried source is still retained in native endpoint order.
        assert_eq!(ledger.query_at(3, a, kernel.point()).unwrap(), F::zero());
        if a == F::zero() {
            assert_eq!(ledger.query_at(0, a, kernel.point()).unwrap(), F::zero());
            assert_ne!(endpoints[0].value, F::zero());
        }
        ledger
            .verify_query_endpoint(a, kernel.point(), &endpoints, running)
            .unwrap();
        assert!(ledger
            .verify_query_endpoint(a, kernel.point(), &endpoints[..3], running)
            .is_err());
        let mut reordered = endpoints;
        reordered.swap(0, 1);
        assert!(ledger
            .verify_query_endpoint(a, kernel.point(), &reordered, running)
            .is_err());
    }
    assert_eq!(sealed_bytes, ledger.canonical_bytes().unwrap());
}

#[test]
fn cached_source_contractions_match_literal_equality_across_chunk_boundary() {
    use super::kernels::{source_pair_cached, source_partial};
    use super::range::BoundPrefixFactors;
    let provider = ReferenceSource {
        id: native(9),
        values: (0..1024)
            .map(|index| F::from_u64(3 * index + 5) - F::from_u64(index % 7))
            .collect(),
    };
    let source = source(9, SourceView::Decoded { num_vars: 10 });
    for mode in 0..3 {
        for bound_len in 0..=10 {
            let bound: Vec<F> = (0..bound_len)
                .map(|axis| match mode {
                    0 => F::zero(),
                    1 => F::one(),
                    _ => match axis % 3 {
                        0 => F::zero(),
                        1 => F::one(),
                        _ => F::from_u64(axis as u64 + 3),
                    },
                })
                .collect();
            let factors = BoundPrefixFactors::new(&bound, 10).unwrap();
            for suffix in 0..cube_len(10 - bound_len).unwrap() {
                let expected = (0..cube_len(bound_len).unwrap()).fold(F::zero(), |sum, prefix| {
                    sum + eq_index(&bound, prefix) * provider.values[prefix | (suffix << bound_len)]
                });
                let actual = source_partial(&source, 0, &provider, &bound, suffix).unwrap();
                assert_eq!(
                    actual, expected,
                    "mode {mode}, bound length {bound_len}, suffix {suffix}"
                );
                if bound_len < 10 && suffix & 1 == 0 {
                    let expected_one =
                        (0..cube_len(bound_len).unwrap()).fold(F::zero(), |sum, prefix| {
                            sum + eq_index(&bound, prefix)
                                * provider.values[prefix | ((suffix | 1) << bound_len)]
                        });
                    let actual_pair =
                        source_pair_cached(&source, 0, &provider, bound_len, suffix >> 1, &factors)
                            .unwrap();
                    assert_eq!(
                        actual_pair,
                        (expected, expected_one),
                        "paired mode {mode}, bound length {bound_len}, suffix {}",
                        suffix >> 1
                    );
                }
            }
        }
    }
    assert!(source_partial(&source, 0, &provider, &[F::zero(); 11], 0).is_err());
    assert!(source_partial(&source, 0, &provider, &[F::zero(); 10], 1).is_err());
    let factors = BoundPrefixFactors::new(&[F::one(); 10], 10).unwrap();
    assert!(source_pair_cached(&source, 0, &provider, 10, 0, &factors).is_err());
    assert!(source_pair_cached(&source, 0, &provider, 11, 0, &factors).is_err());
    assert!(source_pair_cached(&source, 0, &provider, 9, 1, &factors).is_err());
    // Zero prefix factors must not suppress identity/accessor checks.
    let foreign = ReferenceSource {
        id: native(10),
        values: provider.values.clone(),
    };
    assert!(source_partial(&source, 0, &foreign, &[F::one(); 10], 0).is_err());
    let factors = BoundPrefixFactors::new(&[F::one(); 9], 10).unwrap();
    assert!(source_pair_cached(&source, 0, &foreign, 9, 0, &factors).is_err());
}

#[test]
fn raw_cached_contractions_preserve_all_recipe_families_and_virtual_planes() {
    use super::kernels::{read_raw, source_pair_cached};
    use super::range::BoundPrefixFactors;
    struct GeneratedRaw;
    impl FullRingCoefficientSource<F> for GeneratedRaw {
        fn len(&self) -> usize {
            128 * 26 * 32 * 16
        }
        fn coefficient(&self, index: usize) -> Result<F, AkitaError> {
            if index >= self.len() {
                return Err(invalid("generated raw bounds"));
            }
            Ok(F::from_u64((index % 31) as u64) - F::from_u64(15))
        }
    }
    impl SourceProvider<F> for GeneratedRaw {
        fn source_count(&self) -> usize {
            1
        }
        fn source_id(&self, slot: usize) -> Result<(SourceOwnership, usize), AkitaError> {
            if slot != 0 {
                return Err(invalid("generated raw slot"));
            }
            Ok((native(11), 0))
        }
        fn source(&self, slot: usize) -> Result<&dyn FullRingCoefficientSource<F>, AkitaError> {
            if slot != 0 {
                return Err(invalid("generated raw slot"));
            }
            Ok(self)
        }
    }
    let provider = GeneratedRaw;
    let source = source(11, SourceView::OriginalRamPlanesV1);
    let recipes = [
        QueryRecipe::Eq {
            point: vec![F::from_u64(3); 21],
        },
        QueryRecipe::MaskedEq {
            point: vec![F::from_u64(5); 21],
            intervals: vec![0..513, 3200..3328],
        },
        QueryRecipe::Sparse {
            entries: vec![(0, F::one()), (512, F::from_u64(7)), (3200, -F::one())],
        },
        QueryRecipe::DecodedRam {
            point: vec![F::from_u64(7); 16],
            address_axes: (0..7).chain(12..21).collect(),
        },
    ];
    for mode in 0..3 {
        let bound: Vec<F> = (0..9)
            .map(|axis| match mode {
                0 => F::zero(),
                1 => F::one(),
                _ => match axis % 3 {
                    0 => F::zero(),
                    1 => F::one(),
                    _ => F::from_u64(axis + 3),
                },
            })
            .collect();
        let factors = BoundPrefixFactors::new(&bound, 21).unwrap();
        // The low two digit axes are bound. These suffixes cover real planes
        // 24/25 and virtual planes 26..31, as well as nonzero address axes.
        for suffix in [0, 3, 7, 123, 2047] {
            let (f0, f1) =
                source_pair_cached(&source, 0, &provider, bound.len(), suffix, &factors).unwrap();
            let mut point = bound.clone();
            point.resize(21, F::zero());
            for (axis, value) in point.iter_mut().enumerate().skip(10) {
                *value = if (suffix >> (axis - 10)) & 1 == 0 {
                    F::zero()
                } else {
                    F::one()
                };
            }
            for recipe in &recipes {
                point[9] = F::zero();
                let q0 = recipe.evaluate(&point).unwrap();
                point[9] = F::one();
                let q1 = recipe.evaluate(&point).unwrap();
                for t in [F::zero(), F::one(), F::from_u64(11)] {
                    point[9] = t;
                    let literal = (0..1024).fold(F::zero(), |sum, prefix| {
                        let raw = prefix | (suffix << 10);
                        let cell = match source.view.physical_index(raw).unwrap() {
                            Some(physical) => provider.coefficient(physical).unwrap(),
                            None => F::zero(),
                        };
                        sum + eq_index(&point[..10], prefix) * cell
                    });
                    let actual = (q0 + t * (q1 - q0)) * (f0 + t * (f1 - f0));
                    let expected = recipe.evaluate(&point).unwrap() * literal;
                    assert_eq!(
                        actual, expected,
                        "mode {mode}, suffix {suffix}, point {t:?}, recipe {recipe:?}"
                    );
                    let mut actual_bytes = vec![0; F::NUM_BYTES];
                    actual.to_bytes_le(&mut actual_bytes);
                    let mut expected_bytes = vec![0; F::NUM_BYTES];
                    expected.to_bytes_le(&mut expected_bytes);
                    assert_eq!(actual_bytes, expected_bytes);
                }
            }
        }
    }
    for plane in 26..32 {
        assert_eq!(
            read_raw(&source, 0, &provider, plane << 7).unwrap(),
            F::zero()
        );
    }
}

#[test]
fn recipe_round_caches_match_direct_table_at_mask_and_eight_bit_boundaries() {
    // Test-only inverse: eq(2, 1/3) = 0, but each masked address weight is nonzero.
    let third = F::from_u64(3).inverse().unwrap();
    assert_eq!(equality(&[F::from_u64(2)], &[third]).unwrap(), F::zero());
    assert_ne!((F::one() - F::from_u64(2)) * (F::one() - third), F::zero());
    for mu in [0, 1, 10] {
        let len = cube_len(mu).unwrap();
        let common = mu + 2;
        let provider = ReferenceSource {
            id: native(12),
            values: (0..len)
                .map(|index| F::from_u64((3 * index + 5) as u64) - F::from_u64((index % 7) as u64))
                .collect(),
        };
        let mut ledger = QueryLedger::new(
            vec![source(12, SourceView::Decoded { num_vars: mu })],
            limits(common),
            b"recipe suffix and mask boundaries".to_vec(),
        )
        .unwrap();
        if mu > 0 {
            let mut point = vec![F::zero(); mu];
            point[0] = third;
            assert_eq!(
                QueryRecipe::Eq {
                    point: vec![F::from_u64(2); mu],
                }
                .evaluate(&point)
                .unwrap(),
                F::zero()
            );
            assert_ne!(
                QueryRecipe::MaskedEq {
                    point: vec![F::from_u64(2); mu],
                    intervals: vec![0..1],
                }
                .evaluate(&point)
                .unwrap(),
                F::zero()
            );
        }
        let anchor: Vec<F> = (0..mu)
            .map(|axis| match axis % 3 {
                0 => F::zero(),
                1 => F::one(),
                _ => F::from_u64(axis as u64 + 3),
            })
            .collect();
        let boundary_masks = match mu {
            0 => vec![0..1],
            1 => vec![0..1, 1..2],
            _ => vec![0..1, 255..258, 511..513, 767..769, 1023..1024],
        };
        let adjacent_masks = match mu {
            0 => vec![0..1],
            1 => vec![1..2],
            _ => vec![127..129, 255..256, 256..257, 511..512, 768..769],
        };
        let sparse = match mu {
            0 => vec![(0, F::one())],
            1 => vec![(0, F::one()), (1, -F::one())],
            _ => vec![(0, F::one()), (511, -F::one()), (1023, F::from_u64(13))],
        };
        let mut claims = vec![
            vec![QueryTerm {
                source: 0,
                coefficient: F::from_u64(2),
                recipe: QueryRecipe::Eq {
                    point: anchor.clone(),
                },
            }],
            vec![
                QueryTerm {
                    source: 0,
                    coefficient: -F::one(),
                    recipe: QueryRecipe::Eq {
                        point: vec![F::from_u64(3); mu],
                    },
                },
                QueryTerm {
                    source: 0,
                    coefficient: F::from_u64(5),
                    recipe: QueryRecipe::MaskedEq {
                        point: vec![F::from_u64(7); mu],
                        intervals: if mu == 10 {
                            vec![0..257, 768..1024]
                        } else {
                            vec![0..len]
                        },
                    },
                },
                QueryTerm {
                    source: 0,
                    coefficient: F::from_u64(11),
                    recipe: QueryRecipe::Sparse { entries: sparse },
                },
            ],
        ];
        for (point, intervals) in [
            (vec![F::zero(); mu], boundary_masks.clone()),
            (vec![F::one(); mu], boundary_masks),
            (anchor.clone(), adjacent_masks),
            (vec![F::from_u64(2); mu], vec![0..1]),
            (anchor, vec![0..len]),
            (vec![F::from_u64(3); mu], Vec::new()),
        ] {
            claims.push(vec![QueryTerm {
                source: 0,
                coefficient: F::from_u64(17),
                recipe: QueryRecipe::MaskedEq { point, intervals },
            }]);
        }
        for (exponent, terms) in claims.into_iter().enumerate() {
            let mut value = F::zero();
            for (index, &cell) in provider.values.iter().enumerate() {
                let point: Vec<F> = (0..mu)
                    .map(|axis| F::from_u64(((index >> axis) & 1) as u64))
                    .collect();
                for term in &terms {
                    value += term.coefficient * term.recipe.evaluate(&point).unwrap() * cell;
                }
            }
            ledger
                .append(ScalarClaim {
                    exponent,
                    value,
                    offset: F::zero(),
                    terms,
                })
                .unwrap();
        }
        let sealed = ledger.seal().unwrap();
        for a in [F::zero(), F::one(), F::from_u64(7)] {
            for mode in 0..5 {
                let mut kernel = WeightedQueryKernel::new(&ledger, &provider, a).unwrap();
                let mut running = kernel.input_claim();
                for round in 0..common {
                    let polynomial = kernel.round_polynomial(round, running).unwrap();
                    for t in [F::zero(), F::one(), -F::one(), F::from_u64(2)] {
                        let mut point = kernel.point().to_vec();
                        point.resize(common, F::zero());
                        point[round] = t;
                        let mut expected = F::zero();
                        for (index, &cell) in provider.values.iter().enumerate() {
                            for (axis, value) in
                                point.iter_mut().enumerate().take(mu).skip(round + 1)
                            {
                                *value = F::from_u64(((index >> axis) & 1) as u64);
                            }
                            // Direct table contraction includes this round's source bit,
                            // not the optimized pair of bound-prefix contractions.
                            expected += ledger.query_at(0, a, &point).unwrap()
                                * cell
                                * eq_index(&point[..(round + 1).min(mu)], index);
                        }
                        assert_eq!(
                            polynomial.evaluate(t),
                            expected,
                            "dimension {mu}, draw {a:?}, mode {mode}, round {round}, point {t:?}"
                        );
                        let mut actual_bytes = vec![0; F::NUM_BYTES];
                        polynomial.evaluate(t).to_bytes_le(&mut actual_bytes);
                        let mut expected_bytes = vec![0; F::NUM_BYTES];
                        expected.to_bytes_le(&mut expected_bytes);
                        assert_eq!(actual_bytes, expected_bytes);
                    }
                    let challenge = match mode {
                        0 => F::zero(),
                        1 => F::one(),
                        2 => match round % 3 {
                            0 => F::zero(),
                            1 => F::one(),
                            _ => F::from_u64(round as u64 + 3),
                        },
                        4 if round == 0 => third,
                        _ => F::from_u64(round as u64 + 3),
                    };
                    running = polynomial.evaluate(challenge);
                    kernel.bind_challenge(round, challenge).unwrap();
                }
                kernel.finish().unwrap();
                let endpoints = kernel.endpoints().unwrap();
                let expected = provider
                    .values
                    .iter()
                    .enumerate()
                    .fold(F::zero(), |sum, (index, &cell)| {
                        sum + cell * eq_index(&kernel.point()[..mu], index)
                    });
                assert_eq!(endpoints.len(), 1);
                assert_eq!(endpoints[0].value, expected);
                ledger
                    .verify_query_endpoint(a, kernel.point(), &endpoints, running)
                    .unwrap();
            }
        }
        assert_eq!(sealed, ledger.canonical_bytes().unwrap());
    }
}

#[test]
fn paired_query_reads_reject_provider_failures_even_at_zero_prefix_weight() {
    use std::cell::Cell;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Failure {
        Coefficient,
        Identity,
        Extent,
        Accessor,
    }
    struct FallibleSource<'a> {
        material: &'a ReferenceSource,
        failure: Cell<Option<Failure>>,
        fail_after: usize,
        reads: Cell<usize>,
        paired_bound: Option<usize>,
    }
    impl FallibleSource<'_> {
        fn failing(&self, failure: Failure) -> bool {
            self.failure.get() == Some(failure) && self.reads.get() == self.fail_after
        }
    }
    impl FullRingCoefficientSource<F> for FallibleSource<'_> {
        fn len(&self) -> usize {
            let len = self.material.len();
            if self.failing(Failure::Extent) {
                len - 1
            } else {
                len
            }
        }
        fn coefficient(&self, index: usize) -> Result<F, AkitaError> {
            let read = self.reads.get();
            if self.failure.get().is_some() {
                let expected = match self.paired_bound {
                    Some(bound_len) => (read >> 1) | ((read & 1) << bound_len),
                    None => read,
                };
                assert_eq!(index, expected);
            }
            if self.failing(Failure::Coefficient) {
                return Err(invalid("injected coefficient failure"));
            }
            let value = self.material.coefficient(index)?;
            self.reads.set(read + 1);
            Ok(value)
        }
    }
    impl SourceProvider<F> for FallibleSource<'_> {
        fn source_count(&self) -> usize {
            1
        }
        fn source_id(&self, slot: usize) -> Result<(SourceOwnership, usize), AkitaError> {
            if self.failing(Failure::Identity) {
                return Ok((native(14), 0));
            }
            self.material.source_id(slot)
        }
        fn source(&self, slot: usize) -> Result<&dyn FullRingCoefficientSource<F>, AkitaError> {
            if slot != 0 || self.failing(Failure::Accessor) {
                return Err(invalid("injected accessor failure"));
            }
            Ok(self)
        }
    }
    for bound_len in [0, 1, 8, 9, 10] {
        let prefixes: &[usize] = match bound_len {
            0 => &[0],
            1..=8 => &[0, 1],
            _ => &[0, 256, 257],
        };
        for paired in [false, true] {
            let num_vars = bound_len + usize::from(paired);
            let anchor = vec![F::from_u64(2); num_vars];
            let material = ReferenceSource {
                id: native(13),
                values: (0..cube_len(num_vars).unwrap())
                    .map(|index| F::from_u64(3 * index as u64 + 5))
                    .collect(),
            };
            let value = material
                .values
                .iter()
                .enumerate()
                .fold(F::zero(), |sum, (index, &cell)| {
                    sum + cell * eq_index(&anchor, index)
                });
            let mut ledger = QueryLedger::new(
                vec![source(13, SourceView::Decoded { num_vars })],
                limits(num_vars),
                b"fallible cached query reads".to_vec(),
            )
            .unwrap();
            if !paired {
                ledger
                    .append(ScalarClaim {
                        exponent: 0,
                        value: F::zero(),
                        offset: F::zero(),
                        terms: vec![],
                    })
                    .unwrap();
            }
            ledger
                .append(claim(usize::from(!paired), anchor, value))
                .unwrap();
            ledger.seal().unwrap();
            for &prefix in prefixes {
                for side in 0..=usize::from(paired) {
                    let provider = FallibleSource {
                        material: &material,
                        failure: Cell::new(None),
                        fail_after: if paired { 2 * prefix + side } else { prefix },
                        reads: Cell::new(0),
                        paired_bound: paired.then_some(bound_len),
                    };
                    for failure in [
                        Failure::Coefficient,
                        Failure::Identity,
                        Failure::Extent,
                        Failure::Accessor,
                    ] {
                        provider.failure.set(None);
                        let a = if paired { F::one() } else { F::zero() };
                        let mut kernel = WeightedQueryKernel::new(&ledger, &provider, a).unwrap();
                        let mut running = kernel.input_claim();
                        for round in 0..bound_len {
                            let polynomial = kernel.round_polynomial(round, running).unwrap();
                            running = polynomial.evaluate(F::zero());
                            kernel.bind_challenge(round, F::zero()).unwrap();
                        }
                        provider.reads.set(0);
                        provider.failure.set(Some(failure));
                        let rejected = if paired {
                            kernel.round_polynomial(bound_len, running).is_err()
                        } else {
                            kernel.finish().unwrap();
                            kernel.endpoints().is_err()
                        };
                        assert!(
                            rejected,
                            "bound length {bound_len}, paired {paired}, prefix {prefix}, \
                             side {side}, failure {failure:?}"
                        );
                        assert_eq!(provider.reads.get(), provider.fail_after);
                    }
                }
            }
        }
    }
}
