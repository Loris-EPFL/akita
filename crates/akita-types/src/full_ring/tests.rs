use super::*;
use akita_algebra::{eq_poly::EqPolynomial, offset_eq::eq_eval_at_index};
use jolt_field::{ExtField, One, Prime128OffsetA7F7 as K, Prime128OffsetA7F7Ext2 as E, Ring, Zero};

fn k(value: i64) -> K {
    K::from_i64(value)
}
fn e(a: i64, b: i64) -> E {
    E::from_base_fn(|i| if i == 0 { k(a) } else { k(b) })
}
fn mle(values: &[K], point: &[E]) -> E {
    values
        .iter()
        .enumerate()
        .fold(E::zero(), |sum, (i, &value)| {
            sum + eq_eval_at_index(point, i).mul_base(value)
        })
}

#[test]
fn raw_injection_keeps_virtual_and_physical_zeros_distinct() {
    let view = RawSourceViewPlan::new(2, 3, 2, 1, 3).unwrap();
    let permutation = (0..view.physical_len()).rev().collect();
    let view = view.with_permutation(permutation).unwrap();
    let mut physical = vec![false; view.physical_len()];
    let mut live = 0;
    for index in 0..view.raw_len() {
        if let Some(index) = view.raw_to_physical(index).unwrap() {
            assert!(!physical[index]);
            physical[index] = true;
            live += 1;
        }
    }
    assert_eq!(live, view.physical_len());
    assert!(physical.into_iter().all(|seen| seen));
    assert_eq!(view.raw_len(), 64);
    assert!(view.raw_to_physical(64).is_err());
    assert!(RawSourceViewPlan::new(2, 1, 3, 1, 1).is_err());
    assert!(RawSourceViewPlan::new(2, 1, 1, 1, 1)
        .unwrap()
        .with_permutation(vec![0, 0])
        .is_err());
}

#[test]
fn positional_adjoint_and_digit_shortcut_preserve_noncanonical_digits() {
    let view = RawSourceViewPlan::new(2, 1, 1, 1, 3).unwrap();
    // First two digits cancel in one coefficient; do not redecompose them.
    let source = [k(32), k(3), k(-1), k(2), k(0), k(-1)];
    let decoder = DecoderPlan::positional(&view, 0, k(32), k(7)).unwrap();
    let point = [e(3, 5)];
    let decoded = [k(0), k(7 * (3 + 64 - 1024))];
    let value = mle(&decoded, &point);
    let mut adjoint_value = E::zero();
    decoder
        .visit_adjoint(&point, |raw, weight| {
            let physical = view.raw_to_physical(raw).unwrap().unwrap();
            adjoint_value += weight.mul_base(source[physical]);
        })
        .unwrap();
    assert_eq!(adjoint_value, value);
    let (raw_point, raw_value) = decoder.positional_raw_claim(&point, value).unwrap();
    let mut raw = vec![K::zero(); view.raw_len()];
    for (index, value) in raw.iter_mut().enumerate() {
        if let Some(physical) = view.raw_to_physical(index).unwrap() {
            *value = source[physical];
        }
    }
    assert_eq!(mle(&raw, &raw_point), raw_value);
}

#[test]
fn masked_equality_is_one_interpolation_not_a_product_of_masks() {
    let support = CoefficientSupport::new(8, vec![1..3, 5..8]).unwrap();
    let anchor = [e(2, 3), e(5, 7), e(11, 13)];
    let point = [e(17, 19), e(23, 29), e(31, 37)];
    let expected = (0..8)
        .filter(|&i| support.contains(i))
        .fold(E::zero(), |sum, i| {
            sum + eq_eval_at_index(&anchor, i) * eq_eval_at_index(&point, i)
        });
    assert_eq!(support.masked_equality(&anchor, &point).unwrap(), expected);
    let mask_mle = (0..8)
        .filter(|&i| support.contains(i))
        .fold(E::zero(), |sum, i| sum + eq_eval_at_index(&point, i));
    assert_ne!(expected, mask_mle * equality(&anchor, &point).unwrap());
}

#[test]
fn exact_rows_cover_both_planes_and_every_compression_transition() {
    let view = RawSourceViewPlan::new(2, 1, 1, 1, 2).unwrap();
    let point = FullRingRawPoint::new(&view, vec![e(7, 11), e(2, 3)]).unwrap();
    let a = [k(3), k(4), k(5), k(6)];
    let b = [k(7), k(8)];
    let d = [k(19), k(23), k(29), k(31)];
    let f1 = [k(11), k(13)];
    let f2 = [k(17)];
    let h1 = [k(37), k(41)];
    let h2 = [k(43)];
    let fpayload = [k(-40630)];
    let hpayload = [k(950902)];
    let challenges = [k(1), k(1)];
    let group = FullRingRelationGroup {
        view: &view,
        a: NativeMatrix::new(1, 2, 2, &a).unwrap(),
        b: NativeMatrix::new(1, 1, 2, &b).unwrap(),
        challenges: &challenges,
        response: DigitLayout::new(0, 2, 2, 1, k(32)).unwrap(),
        inner_images: DigitLayout::new(8, 1, 2, 1, k(32)).unwrap(),
        channels: vec![FullRingRelationChannel {
            point: &point,
            partials: DigitLayout::new(4, 2, 2, 1, k(32)).unwrap(),
            scalar_claims: vec![PartialScalarClaim::raw(&point, e(12, 17))],
        }],
        f: CompressionChain {
            stages: vec![
                CompressionStage {
                    matrix: NativeMatrix::new(1, 2, 1, &f1).unwrap(),
                    input: 10..12,
                    digit_bits: 1,
                },
                CompressionStage {
                    matrix: NativeMatrix::new(1, 1, 1, &f2).unwrap(),
                    input: 14..15,
                    digit_bits: 1,
                },
            ],
            payload: &fpayload,
        },
    };
    let h = CompressionChain {
        stages: vec![
            CompressionStage {
                matrix: NativeMatrix::new(1, 2, 1, &h1).unwrap(),
                input: 12..14,
                digit_bits: 1,
            },
            CompressionStage {
                matrix: NativeMatrix::new(1, 1, 1, &h2).unwrap(),
                input: 15..16,
                digit_bits: 1,
            },
        ],
        payload: &hpayload,
    };
    let plan =
        CoefficientRelationPlan::new(32, vec![group], NativeMatrix::new(1, 2, 2, &d).unwrap(), h)
            .unwrap();
    // Independently calculated native ring products, including signed wraps.
    let mut witness: Vec<K> = [
        -1, 3, -1, 7, 5, 6, 6, 6, -14, 48, -482, 224, -55, 589, -2390, 22114,
    ]
    .into_iter()
    .map(k)
    .collect();
    witness.resize(32, K::zero());
    assert_eq!(plan.row_count(), 16);
    assert!(plan
        .residuals(&witness)
        .unwrap()
        .iter()
        .all(|value| value.is_zero()));
    for column in 0..16 {
        witness[column] += K::one();
        assert!(
            plan.residuals(&witness)
                .unwrap()
                .iter()
                .any(|value| !value.is_zero()),
            "unconstrained live column {column}"
        );
        witness[column] -= K::one();
    }
    let rho = vec![e(2, 3), e(5, 7), e(11, 13), e(17, 19)];
    let r = vec![e(23, 29), e(31, 37), e(41, 43), e(47, 53), e(59, 61)];
    let mut columns = vec![E::zero(); 32];
    plan.accumulate_columns(&rho, &mut columns).unwrap();
    let eval = columns
        .iter()
        .enumerate()
        .fold(E::zero(), |sum, (i, &value)| {
            sum + eq_eval_at_index(&r, i) * value
        });
    assert_eq!(eval, plan.evaluate_column_mle(&rho, &r).unwrap());
    assert_eq!(
        columns
            .iter()
            .zip(&witness)
            .fold(E::zero(), |sum, (&weight, &value)| sum
                + weight.mul_base(value)),
        plan.rhs_at(&rho).unwrap()
    );
    for index in 0..32 {
        assert_eq!(plan.required_zero_support().contains(index), index >= 16);
        assert_eq!(
            plan.binary_support().contains(index),
            (10..16).contains(&index)
        );
    }
}

#[test]
fn range_tree_keeps_the_virtual_table_off_cube() {
    let source = vec![k(-16), k(-1), k(0), k(15)];
    let tau = [e(2, 3), e(5, 7)];
    let mut session = ProductRangeSession::new(&source, &tau).unwrap();
    let r1 = [e(11, 13), e(17, 19)];
    for r in r1 {
        let _full_six = session.round_coefficients().unwrap();
        session.bind(r).unwrap();
    }
    let quarters = session.quarter_claims().unwrap();
    let eq = EqPolynomial::evals(&r1).unwrap();
    for (quarter, &claim) in quarters.iter().enumerate() {
        let expected = source
            .iter()
            .zip(&eq)
            .fold(E::zero(), |sum, (&w, &weight)| {
                let s = w * (w + K::one());
                let q = (4 * quarter..4 * quarter + 4)
                    .fold(K::one(), |p, k| p * (s - K::from_u64((k * (k + 1)) as u64)));
                sum + weight.mul_base(q)
            });
        assert_eq!(claim, expected);
    }
    session
        .begin_leaf_stage([e(23, 29), e(31, 37), e(41, 43)])
        .unwrap();
    let r2 = [e(47, 53), e(59, 61)];
    for r in r2 {
        let _full_six = session.round_coefficients().unwrap();
        session.bind(r).unwrap();
    }
    let svirt = session.virtual_claim().unwrap();
    let virtual_table: Vec<K> = source.iter().map(|&w| w * (w + K::one())).collect();
    assert_eq!(svirt, mle(&virtual_table, &r2));
    let w = mle(&source, &r2);
    assert_ne!(svirt, w * (w + E::one()));
    assert!(session.begin_leaf_stage([E::one(); 3]).is_err());
}

#[test]
fn source_prefix_and_normalization_compute_real_messages() {
    let view = RawSourceViewPlan::new(2, 1, 1, 1, 1).unwrap();
    let source = vec![K::zero(), K::one()];
    // Required-zero support is a list of intervals, not a vector of indices.
    #[expect(
        clippy::single_range_in_vec_init,
        reason = "one physical zero interval is intentional"
    )]
    let domain = SourceDomainPlan::new(
        view.clone(),
        SourceAlphabet::BooleanWholeGrid,
        CoefficientSupport::new(2, vec![0..1]).unwrap(),
    )
    .unwrap();
    let anchor = [e(2, 3)];
    let mut prefix = SourceDomainSession::new(&source, &domain, &anchor, e(5, 7)).unwrap();
    let round = prefix.round_coefficients().unwrap().to_vec();
    assert_eq!(round.len(), 4);
    assert_eq!(
        round.iter().copied().fold(round[0], |a, b| a + b),
        E::zero()
    );
    prefix.bind(e(11, 13)).unwrap();
    assert_eq!(prefix.raw_value().unwrap(), e(11, 13));
    let decoder = DecoderPlan::positional(&view, 0, k(32), K::one()).unwrap();
    let ordinary = [e(17, 19)];
    let raw_point = [e(23, 29)];
    let plan = NormalizationPlan::new(
        &view,
        vec![
            NormalizationClaim::Ordinary {
                decoder: &decoder,
                point: &ordinary,
                value: ordinary[0],
            },
            NormalizationClaim::Raw {
                point: &raw_point,
                value: raw_point[0],
            },
        ],
        vec![e(31, 37), e(41, 43)],
    )
    .unwrap();
    let mut normalization = NormalizationSession::new(&source, &plan).unwrap();
    let coeffs = normalization.round_coefficients().unwrap();
    assert_eq!(
        coeffs.iter().copied().fold(coeffs[0], |a, b| a + b),
        plan.target().unwrap()
    );
    normalization.bind(e(47, 53)).unwrap();
    assert_eq!(normalization.raw_value().unwrap(), e(47, 53));
    let forbidden = vec![K::one(), K::one()];
    let mut invalid_prefix =
        SourceDomainSession::new(&forbidden, &domain, &anchor, E::one()).unwrap();
    assert!(invalid_prefix.round_coefficients().is_err());
}

#[test]
fn compression_digits_use_the_native_transpose() {
    validate_full_ring_fields::<K, E>().unwrap();
    assert!(validate_full_ring_fields::<K, K>().is_err());
    let view = RawSourceViewPlan::new(2, 1, 1, 1, 1).unwrap();
    let point = FullRingRawPoint::new(&view, vec![e(2, 3)]).unwrap();
    let identity = [K::one(), K::zero()];
    let d = [K::one(), K::zero(), K::one(), K::zero()];
    let f = [k(3), k(5), k(7), k(11)];
    let fpayload = [k(-14)];
    let hpayload = [k(-1), k(-2)];
    let group = FullRingRelationGroup {
        view: &view,
        a: NativeMatrix::new(1, 1, 2, &identity).unwrap(),
        b: NativeMatrix::new(1, 1, 2, &identity).unwrap(),
        challenges: &identity,
        response: DigitLayout::new(0, 1, 2, 1, k(32)).unwrap(),
        inner_images: DigitLayout::new(6, 1, 2, 1, k(32)).unwrap(),
        channels: vec![FullRingRelationChannel {
            point: &point,
            partials: DigitLayout::new(2, 2, 2, 1, k(32)).unwrap(),
            scalar_claims: vec![PartialScalarClaim::raw(&point, e(-3, -3))],
        }],
        f: CompressionChain {
            stages: vec![CompressionStage {
                matrix: NativeMatrix::new(1, 4, 1, &f).unwrap(),
                input: 8..12,
                digit_bits: 2,
            }],
            payload: &fpayload,
        },
    };
    let relation = CoefficientRelationPlan::new(
        16,
        vec![group],
        NativeMatrix::new(1, 2, 2, &d).unwrap(),
        CompressionChain {
            stages: vec![],
            payload: &hpayload,
        },
    )
    .unwrap();
    // Digit0 of both coefficients, then digit1 of both coefficients:
    // (-1,0) + 2*(0,-1) = (-1,-2).
    let mut witness: Vec<K> = [-1, -2, -1, -2, 0, 0, -1, -2, -1, 0, 0, -1, 0, 0, 0, 0]
        .into_iter()
        .map(k)
        .collect();
    assert!(relation
        .residuals(&witness)
        .unwrap()
        .iter()
        .all(|value| value.is_zero()));
    // Move the high digit into coefficient-major rather than transposed
    // storage. The original B/F equations must detect that reinterpretation.
    witness.swap(9, 11);
    assert!(relation
        .residuals(&witness)
        .unwrap()
        .iter()
        .any(|value| !value.is_zero()));
}

// Schoolbook multiplication, independent of the relation's row visitor and
// its output-coefficient/sign formula.
fn reference_negacyclic<F: Field>(left: &[F], right: &[F]) -> Vec<F> {
    assert_eq!(left.len(), right.len());
    let degree = left.len();
    let mut result = vec![F::zero(); degree];
    for (i, &a) in left.iter().enumerate() {
        for (j, &b) in right.iter().enumerate() {
            if i + j < degree {
                result[i + j] += a * b;
            } else {
                result[i + j - degree] -= a * b;
            }
        }
    }
    result
}

// This uses physical source and response order throughout. Only the independent
// partial oracle pulls back the registered raw map. Nonzero A/B/D rows also
// detect accidentally applying the raw permutation to the physical Z layout.
fn check_registered_fold_rows(view: &RawSourceViewPlan, source: &[K]) {
    view.validate_fold_admission().unwrap();
    let degree = view.degree();
    let blocks = view.polynomials() * view.blocks();
    let columns = view.positions() * view.digits();
    assert_eq!(degree, 128);
    assert_eq!(source.len(), view.physical_len());
    let coordinates: Vec<_> = (0..view.num_vars())
        .map(|i| e(3 + i as i64, 5 + 2 * i as i64))
        .collect();
    let point = FullRingRawPoint::new(view, coordinates.clone()).unwrap();
    let mut challenges = vec![K::zero(); blocks * degree];
    for block in 0..blocks {
        for position in 0..40 {
            let index = (position * 3 + block * 7) % degree;
            let magnitude = if position < 24 { 1 } else { 2 };
            challenges[block * degree + index] = k(if position % 2 == 0 {
                magnitude
            } else {
                -magnitude
            });
        }
    }
    // Include both carrier boundaries in the first valid (24,16) shell.
    challenges.swap(39 * 3, 127);
    let mut partials = vec![E::zero(); blocks * degree];
    let mut scalar = E::zero();
    for p in 0..view.polynomials() {
        for i in 0..view.blocks() {
            let block = p * view.blocks() + i;
            for x in 0..view.positions() {
                for ell in 0..view.digits() {
                    let weight = point.position_weights()[x] * point.digit_weights()[ell];
                    for j in 0..degree {
                        let physical = view.physical_index(p, i, x, ell, j).unwrap();
                        partials[block * degree + j] += weight.mul_base(source[physical]);
                        scalar += eq_eval_at_index(
                            &coordinates,
                            view.raw_index(p, i, x, ell, j).unwrap(),
                        )
                        .mul_base(source[physical]);
                    }
                }
            }
        }
    }
    let mut response = vec![K::zero(); columns * degree];
    let mut images = vec![K::zero(); blocks * degree];
    let mut folded_partials = vec![E::zero(); degree];
    for block in 0..blocks {
        let shell = &challenges[block * degree..(block + 1) * degree];
        for column in 0..columns {
            let start = (block * columns + column) * degree;
            let ring = &source[start..start + degree];
            let folded = reference_negacyclic(shell, ring);
            for j in 0..degree {
                response[column * degree + j] += folded[j];
                images[block * degree + j] += k(column as i64 + 1) * ring[j];
            }
        }
        let physical_block = view
            .physical_index(block / view.blocks(), block % view.blocks(), 0, 0, 0)
            .unwrap()
            / (columns * degree);
        let shell: Vec<_> = challenges[physical_block * degree..(physical_block + 1) * degree]
            .iter()
            .copied()
            .map(E::lift_base)
            .collect();
        let folded = reference_negacyclic(&shell, &partials[block * degree..(block + 1) * degree]);
        for j in 0..degree {
            folded_partials[j] += folded[j];
        }
    }
    let mut a = vec![K::zero(); columns * degree];
    for column in 0..columns {
        a[column * degree] = k(column as i64 + 1);
    }
    let mut b = vec![K::zero(); blocks * blocks * degree];
    for block in 0..blocks {
        b[(block * blocks + block) * degree] = K::one();
    }
    let mut d = vec![K::zero(); blocks * 2 * degree];
    let mut opening_payload = vec![K::zero(); degree];
    for block in 0..blocks {
        for plane in 0..2 {
            let weight = k((block * 2 + plane + 1) as i64);
            d[(block * 2 + plane) * degree] = weight;
            for j in 0..degree {
                opening_payload[j] += weight * partials[block * degree + j].base_coefficient(plane);
            }
        }
    }
    let e_start = columns * degree * 2;
    let t_start = e_start + blocks * 2 * degree;
    let domain = (t_start + blocks * degree).next_power_of_two();
    let group = FullRingRelationGroup {
        view,
        a: NativeMatrix::new(1, columns, degree, &a).unwrap(),
        b: NativeMatrix::new(blocks, blocks, degree, &b).unwrap(),
        challenges: &challenges,
        response: DigitLayout::new(0, columns, degree, 2, k(32)).unwrap(),
        inner_images: DigitLayout::new(t_start, blocks, degree, 1, k(32)).unwrap(),
        channels: vec![FullRingRelationChannel {
            point: &point,
            partials: DigitLayout::new(e_start, blocks * 2, degree, 1, k(32)).unwrap(),
            scalar_claims: vec![PartialScalarClaim::raw(&point, scalar)],
        }],
        f: CompressionChain {
            stages: vec![],
            payload: &images,
        },
    };
    let relation = CoefficientRelationPlan::new_d1(
        domain,
        vec![group],
        NativeMatrix::new(1, blocks * 2, degree, &d).unwrap(),
        CompressionChain {
            stages: vec![],
            payload: &opening_payload,
        },
    )
    .unwrap();
    let mut witness = vec![K::zero(); domain];
    for column in 0..columns {
        for j in 0..degree {
            // Noncanonical response digits: preserve G_rsp, not G_com.
            witness[column * 2 * degree + j] = response[column * degree + j] - k(32);
            witness[(column * 2 + 1) * degree + j] = K::one();
        }
    }
    // First evaluate actual L rows with no partials: this is L(z), and must
    // equal the independent negacyclic sum, coefficient by coefficient.
    let response_rows = relation.residuals(&witness).unwrap();
    for block in relation.row_manifest() {
        if let CoefficientRowFamily::L { plane, .. } = block.family {
            for j in 0..degree {
                assert_eq!(
                    response_rows[block.rows.start + j],
                    folded_partials[j].base_coefficient(plane),
                    "L mismatch at plane {plane}, coefficient {j}"
                );
            }
        }
    }
    for block in 0..blocks {
        for plane in 0..2 {
            for j in 0..degree {
                witness[e_start + (block * 2 + plane) * degree + j] =
                    partials[block * degree + j].base_coefficient(plane);
            }
        }
    }
    witness[t_start..t_start + images.len()].copy_from_slice(&images);
    assert!(relation
        .residuals(&witness)
        .unwrap()
        .iter()
        .all(|value| value.is_zero()));
    // Both partial planes and both boundary coefficients really participate.
    for plane in 0..2 {
        for j in [0, degree - 1] {
            let index = e_start + plane * degree + j;
            witness[index] += K::one();
            let residuals = relation.residuals(&witness).unwrap();
            assert!(relation
                .row_manifest()
                .iter()
                .filter(|block| matches!(block.family, CoefficientRowFamily::L { .. }))
                .any(|block| residuals[block.rows.clone()]
                    .iter()
                    .any(|value| !value.is_zero())));
            witness[index] -= K::one();
        }
    }
}

#[test]
fn whole_position_ring_swap_matches_native_fold_at_nonbase_point() {
    let canonical = RawSourceViewPlan::new(128, 1, 1, 2, 1).unwrap();
    let swapped = canonical
        .clone()
        .with_permutation((128..256).chain(0..128).collect())
        .unwrap();
    let mut physical = vec![K::zero(); 256];
    // The reported counterexample: physical rings (1,0), not a raw copy.
    physical[0] = K::one();
    check_registered_fold_rows(&canonical, &physical);
    check_registered_fold_rows(&swapped, &physical);
    // Force actual negacyclic wraps, not just scalar multiplication of c.
    physical[127] = k(7);
    physical[128 + 1] = k(-3);
    check_registered_fold_rows(&swapped, &physical);
}

#[test]
fn common_position_digit_permutation_factors_across_original_polynomials_and_blocks() {
    // Non-dyadic P and delta retain virtual zeros. This cycle mixes x and
    // ell, so admission cannot incorrectly require separability of those axes.
    let canonical = RawSourceViewPlan::new(128, 3, 2, 2, 3).unwrap();
    let sigma = [1, 4, 0, 5, 2, 3];
    // Non-involutive block map catches beta versus inverse-beta mistakes.
    let beta = [2, 4, 1, 5, 3, 0];
    let permutation = (0..canonical.physical_len())
        .map(|index| {
            let j = index % 128;
            let column = index / 128 % 6;
            let block = index / (128 * 6);
            (beta[block] * 6 + sigma[column]) * 128 + j
        })
        .collect();
    let view = canonical.with_permutation(permutation).unwrap();
    let source: Vec<_> = (0..view.physical_len())
        .map(|index| k((index * 17 % 31) as i64 - 15))
        .collect();
    check_registered_fold_rows(&view, &source);
}

#[test]
fn fold_admission_rejects_noncommuting_and_block_dependent_maps_not_raw_views() {
    let canonical = RawSourceViewPlan::new(128, 2, 2, 2, 3).unwrap();
    let identity: Vec<_> = (0..canonical.physical_len()).collect();
    let block_len = 128 * 2 * 3;
    let mut invalid_maps = Vec::new();
    // A coefficient swap is a valid raw normalization map but not S-linear.
    let mut map = identity.clone();
    map.swap(0, 127);
    invalid_maps.push(map);
    // Even a uniform unsigned coefficient rotation does not commute with the
    // signed Y^128 = -1 wrap; whole-ring moves must preserve coefficient j.
    invalid_maps.push(
        (0..canonical.physical_len())
            .map(|index| (index / 128) * 128 + (index + 1) % 128)
            .collect(),
    );
    // Same ring swap only in one i, or only in one p: no shared L exists.
    for affected in [1..2, 2..4] {
        let mut map = identity.clone();
        for block in affected {
            for j in 0..128 {
                map.swap(block * block_len + j, block * block_len + 128 + j);
            }
        }
        invalid_maps.push(map);
    }
    // Moving only one column across original blocks also violates factoring.
    let mut map = identity;
    for j in 0..128 {
        map.swap(j, block_len + j);
    }
    invalid_maps.push(map);
    for map in invalid_maps {
        let view = canonical.clone().with_permutation(map).unwrap();
        assert!(view.validate_fold_admission().is_err());
        // Preserve the more general raw/source-domain boundary.
        let point = FullRingRawPoint::new(&view, vec![e(2, 3); view.num_vars()]).unwrap();
        let domain = SourceDomainPlan::new(
            view.clone(),
            SourceAlphabet::BalancedBase32,
            CoefficientSupport::new(view.physical_len(), vec![]).unwrap(),
        )
        .unwrap();
        assert_eq!(domain.view(), point.view());
    }
}

#[test]
fn coefficient_relation_itself_rejects_a_noncommuting_raw_map() {
    let view = RawSourceViewPlan::new(2, 1, 1, 1, 1)
        .unwrap()
        .with_permutation(vec![1, 0])
        .unwrap();
    let point = FullRingRawPoint::new(&view, vec![e(2, 3)]).unwrap();
    let identity = [K::one(), K::zero()];
    let d = [K::one(), K::zero(), K::one(), K::zero()];
    let payload = [K::zero(); 2];
    let group = FullRingRelationGroup {
        view: &view,
        a: NativeMatrix::new(1, 1, 2, &identity).unwrap(),
        b: NativeMatrix::new(1, 1, 2, &identity).unwrap(),
        challenges: &identity,
        response: DigitLayout::new(0, 1, 2, 1, k(32)).unwrap(),
        inner_images: DigitLayout::new(6, 1, 2, 1, k(32)).unwrap(),
        channels: vec![FullRingRelationChannel {
            point: &point,
            partials: DigitLayout::new(2, 2, 2, 1, k(32)).unwrap(),
            scalar_claims: vec![PartialScalarClaim::raw(&point, E::zero())],
        }],
        f: CompressionChain {
            stages: vec![],
            payload: &payload,
        },
    };
    assert!(CoefficientRelationPlan::new(
        8,
        vec![group],
        NativeMatrix::new(1, 2, 2, &d).unwrap(),
        CompressionChain {
            stages: vec![],
            payload: &payload
        }
    )
    .is_err());
}

#[test]
fn borrowed_raw_range_preserves_permuted_physical_cells_and_transformed_holes() {
    let view = RawSourceViewPlan::new(2, 3, 1, 1, 3)
        .unwrap()
        .with_permutation((0..18).rev().collect())
        .unwrap();
    let mut source: Vec<K> = (0..18).map(|i| k([0, -1, -16, 15][i % 4])).collect();
    source[7] = K::zero();
    let original = source.clone();
    let plan = SourceDomainPlan::new(
        view.clone(),
        SourceAlphabet::BalancedBase32,
        CoefficientSupport::new(18, std::iter::once(7..8).collect()).unwrap(),
    )
    .unwrap();
    let raw = RawSourceReader::new(&source, &view).unwrap();
    assert_eq!(raw.len(), 32);
    assert!(raw.coefficient(32).is_err());
    assert!(RawSourceReader::new(&vec![K::zero(); 32], &view).is_err());
    // Independent axis/permutation oracle, not merely a decoded-source check.
    let expected: Vec<K> = (0..32)
        .map(|index| {
            let (p, digit, j) = (index / 8, (index / 2) % 4, index % 2);
            if p == 3 || digit == 3 {
                K::zero()
            } else {
                source[17 - (p * 6 + digit * 2 + j)]
            }
        })
        .collect();
    for (index, &value) in expected.iter().enumerate() {
        assert_eq!(raw.coefficient(index).unwrap(), value);
    }
    let tau = vec![e(2, 3); 5];
    assert!(ProductRangeSession::new(&source, &tau).is_err());
    assert!(SourceDomainSession::new(&source, &plan, &tau, E::one()).is_err());
    let mut range = ProductRangeSession::new(&raw, &tau).unwrap();
    // Boolean point 6 is a padded digit hole. Higher quarter transforms must
    // be the nonzero transforms of W=0, not zeroed after transformation.
    for bit in 0..5 {
        assert_eq!(range.round_coefficients().unwrap().len(), 6);
        range.bind(E::from_u64((6 >> bit) & 1)).unwrap();
    }
    let quarters = range.quarter_claims().unwrap();
    assert_eq!(quarters[0], E::zero());
    for (quarter, &claim) in quarters.iter().enumerate().skip(1) {
        let expected = (4 * quarter..4 * quarter + 4)
            .fold(E::one(), |p, i| p * -E::from_u64((i * (i + 1)) as u64));
        assert_ne!(expected, E::zero());
        assert_eq!(claim, expected);
    }
    range
        .begin_leaf_stage([e(5, 7), e(11, 13), e(17, 19)])
        .unwrap();
    let rvirt: Vec<E> = (0..5).map(|i| e(23 + i, 29)).collect();
    for &r in &rvirt {
        range.round_coefficients().unwrap();
        range.bind(r).unwrap();
    }
    let svirt = range.virtual_claim().unwrap();
    let virtual_table: Vec<K> = expected.iter().map(|&w| w * (w + K::one())).collect();
    assert_eq!(svirt, mle(&virtual_table, &rvirt));
    let wvirt = mle(&expected, &rvirt);
    assert_ne!(svirt, wvirt * (wvirt + E::one()));
    drop(range);
    let eta = e(31, 37);
    let mut realization =
        SourceRangeRealizationSession::new(&source, &plan, &tau, &rvirt, svirt, Some(eta)).unwrap();
    let point: Vec<E> = (0..5).map(|i| e(41 + i, 43)).collect();
    for &r in &point {
        assert_eq!(realization.round_coefficients().unwrap().len(), 4);
        realization.bind(r).unwrap();
    }
    let value = realization.raw_value().unwrap();
    assert_eq!(value, mle(&expected, &point));
    assert_ne!(value, svirt);
    assert_eq!(source, original);
    // Continue with the original physical reader, not the raw adapter. Both
    // the ordinary obligation and the new raw endpoint survive normalization.
    let decoder = DecoderPlan::positional(&view, 1, k(32), K::one()).unwrap();
    let ordinary_point = [e(47, 53)];
    let decoded: Vec<K> = (0..2)
        .map(|j| expected[8 + j] + k(32) * expected[10 + j] + k(1024) * expected[12 + j])
        .collect();
    let normalization = NormalizationPlan::new(
        &view,
        vec![
            NormalizationClaim::Ordinary {
                decoder: &decoder,
                point: &ordinary_point,
                value: mle(&decoded, &ordinary_point),
            },
            NormalizationClaim::Raw {
                point: &point,
                value,
            },
        ],
        vec![e(59, 61), e(67, 71)],
    )
    .unwrap();
    assert!(NormalizationSession::new(&raw, &normalization).is_err());
    let mut normalized = NormalizationSession::new(&source, &normalization).unwrap();
    let next: Vec<E> = (0..5).map(|i| e(73 + i, 79)).collect();
    for &r in &next {
        normalized.round_coefficients().unwrap();
        normalized.bind(r).unwrap();
    }
    assert_eq!(normalized.raw_value().unwrap(), mle(&expected, &next));
    drop(normalized);
    // Physical index 7 pulls back to raw 12, not to a physical-prefix mask.
    let weight = eq_eval_at_index(&tau, 12) * eq_eval_at_index(&point, 12);
    assert_eq!(plan.zero_weight(&tau, &point).unwrap(), weight);
    assert_ne!(
        weight,
        eq_eval_at_index(&point, 12) * equality(&tau, &point).unwrap()
    );
    assert!(
        SourceRangeRealizationSession::new(&source, &plan, &tau, &rvirt, svirt, None,).is_err()
    );
    drop(realization);
    // A required physical zero is read verbatim, even when it is invalid.
    source[7] = K::one();
    let invalid = RawSourceReader::new(&source, &view).unwrap();
    assert_eq!(invalid.coefficient(12).unwrap(), K::one());
    let new_virtual: Vec<K> = (0..32)
        .map(|i| {
            let w = invalid.coefficient(i).unwrap();
            w * (w + K::one())
        })
        .collect();
    let mut bad = SourceRangeRealizationSession::new(
        &source,
        &plan,
        &tau,
        &rvirt,
        mle(&new_virtual, &rvirt),
        Some(eta),
    )
    .unwrap();
    assert!(bad.round_coefficients().is_err());
}

#[test]
fn source_range_boundary_alphabet_and_exact_ledger() {
    let view = RawSourceViewPlan::new(2, 1, 1, 1, 1).unwrap();
    for invalid in [-17, 16] {
        let source = vec![k(invalid), K::zero()];
        let mut range = ProductRangeSession::new(&source, &[e(2, 3)]).unwrap();
        assert!(range.round_coefficients().is_err());
    }
    // Main's alphabet remains positive Boolean, never the RAM paired roots.
    let main = SourceDomainPlan::new(
        view,
        SourceAlphabet::BooleanWholeGrid,
        CoefficientSupport::new(2, vec![]).unwrap(),
    )
    .unwrap();
    let source = vec![k(-1), K::zero()];
    let mut direct = SourceDomainSession::new(&source, &main, &[e(2, 3)], E::one()).unwrap();
    assert!(direct.round_coefficients().is_err());
    for z in [false, true] {
        let view = RawSourceViewPlan::new(128, 26, 32, 16, 1).unwrap();
        let support = if z {
            std::iter::once(7..8).collect()
        } else {
            vec![]
        };
        let plan = SourceDomainPlan::new(
            view.clone(),
            SourceAlphabet::BalancedBase32,
            CoefficientSupport::new(view.physical_len(), support).unwrap(),
        )
        .unwrap();
        let ledger = plan.range_ledger().unwrap();
        assert_eq!(
            (ledger.physical_cells, ledger.raw_cells),
            (1_703_936, 2_097_152)
        );
        assert_eq!(ledger.num_vars, 21);
        assert_eq!(ledger.round_degrees, [5, 5, 3]);
        assert_eq!(ledger.degree_charge, if z { 319 } else { 297 });
        assert_eq!(ledger.literal_draws, if z { 88 } else { 87 });
        assert_eq!(ledger.proof_elements, 342);
        assert_eq!(ledger.tree_multiplier, if z { 8 } else { 4 });
        assert_eq!(ledger.tree_round_factor, 288);
        assert_eq!(ledger.work_columns, [4, 1, if z { 2 } else { 1 }]);
        assert_eq!(ledger.zero_mask_scan_cells, if z { 2_097_152 } else { 0 });
    }
}

#[test]
fn borrowed_raw_reader_propagates_physical_failures_but_never_reads_holes() {
    struct FailingSource(std::cell::Cell<usize>);
    impl FullRingCoefficientSource<K> for FailingSource {
        fn len(&self) -> usize {
            3
        }
        fn coefficient(&self, index: usize) -> Result<K, AkitaError> {
            self.0.set(self.0.get() + 1);
            if index == 1 {
                Err(AkitaError::InvalidProof)
            } else {
                Ok(k(15))
            }
        }
    }
    let view = RawSourceViewPlan::new(1, 3, 1, 1, 1)
        .unwrap()
        .with_permutation(vec![2, 0, 1])
        .unwrap();
    let source = FailingSource(std::cell::Cell::new(0));
    let raw = RawSourceReader::new(&source, &view).unwrap();
    assert_eq!(raw.coefficient(3).unwrap(), K::zero());
    assert_eq!(source.0.get(), 0);
    assert_eq!(raw.coefficient(0).unwrap(), k(15));
    assert_eq!(source.0.get(), 1);
    assert!(raw.coefficient(2).is_err());
    assert_eq!(source.0.get(), 2);
    assert!(raw.coefficient(4).is_err());
    assert_eq!(source.0.get(), 2);
}

#[test]
fn receiver_keeps_negative_binary_and_zero_supports_separate_from_fresh_ram() {
    let domain = FinalReceiverDomainPlan::new(
        RawSourceViewPlan::new(4, 1, 1, 1, 1).unwrap(),
        CoefficientSupport::new(4, std::iter::once(0..1).collect()).unwrap(),
        CoefficientSupport::new(4, std::iter::once(1..2).collect()).unwrap(),
    )
    .unwrap();
    let tau = [e(2, 3), e(5, 7)];
    let rvirt = [e(11, 13), e(17, 19)];
    let mix = StrongMix {
        gamma: e(23, 29),
        eta_bin: e(31, 37),
        eta_zero: e(41, 43),
    };
    for invalid in [None, Some(0), Some(1)] {
        let mut source = vec![k(-1), K::zero(), k(15), k(-16)];
        if let Some(index) = invalid {
            source[index] = K::one();
        }
        let virtual_table: Vec<K> = source.iter().map(|&w| w * (w + K::one())).collect();
        let mut session = FinalReceiverDomainSession::new(
            &source,
            &domain,
            &tau,
            &rvirt,
            mle(&virtual_table, &rvirt),
            mix,
        )
        .unwrap();
        if invalid.is_some() {
            assert!(session.round_coefficients().is_err());
        } else {
            for r in [e(47, 53), e(59, 61)] {
                assert_eq!(session.round_coefficients().unwrap().len(), 4);
                session.bind(r).unwrap();
            }
            assert_eq!(
                session.witness_value().unwrap(),
                mle(&source, session.point())
            );
        }
    }
    let padded = RawSourceViewPlan::new(2, 3, 1, 1, 3).unwrap();
    assert!(FinalReceiverDomainPlan::new(
        padded,
        CoefficientSupport::new(18, vec![]).unwrap(),
        CoefficientSupport::new(18, vec![]).unwrap(),
    )
    .is_err());
}
