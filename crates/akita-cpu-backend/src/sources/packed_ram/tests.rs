use super::*;
use crate::arithmetic::{ComputeBackendSetup, CpuPreparedSetup};
use crate::commitment::{compile_commitment_request, BackendKindId, CommitmentRequestCapabilities};
use akita_types::full_ring::{FullRingRawPoint, RawSourceViewPlan};
use jolt_field::{ExtField, One, Prime128OffsetA7F7, Prime128OffsetA7F7Ext2, Ring, Zero};

type F = Prime128OffsetA7F7;
type E = Prime128OffsetA7F7Ext2;

fn plan() -> CommitInnerPlan {
    CommitInnerPlan {
        ring_dimension: 128,
        num_live_blocks: 2,
        n_a: 3,
        num_positions_per_block: 2,
        num_digits_inner: 26,
        log_basis_inner: 5,
    }
}

fn field(digit: i32) -> F {
    if digit < 0 {
        -F::from_u64(u64::from(digit.unsigned_abs()))
    } else {
        F::from_u64(digit as u64)
    }
}

fn patterned_digits(plan: CommitInnerPlan, seed: usize) -> Vec<i8> {
    let columns = plan.num_positions_per_block * 26;
    let mut digits = vec![0; plan.num_live_blocks * columns * 128];
    for block in 0..plan.num_live_blocks {
        for column in 0..columns {
            for offset in [0, 63, 127] {
                let j = (column * 17 + block * 11 + seed + offset) % 128;
                digits[(block * columns + column) * 128 + j] =
                    ((column * 7 + block * 3 + seed + offset) % 32) as i8 - 16;
            }
        }
    }
    digits
}

fn prepared(plan: CommitInnerPlan) -> (CpuBackend<F, E>, CpuPreparedSetup<F>) {
    let backend = CpuBackend::<F, E>::for_arithmetic_tests();
    let setup = crate::AkitaProverSetup::<F>::generate_with_capacity(
        9,
        1,
        akita_params::SetupMatrixCapacity {
            num_field_elements: plan.n_a * plan.num_positions_per_block * 26 * 128,
        },
    )
    .unwrap();
    let prepared = backend.prepare_setup(&setup).unwrap();
    (backend, prepared)
}

fn commit(
    backend: &CpuBackend<F, E>,
    prepared: &CpuPreparedSetup<F>,
    source: &PackedRamSource,
) -> Vec<F> {
    let capabilities = CommitmentRequestCapabilities::split::<()>(
        BackendKindId::of::<CpuBackend<F, E>>("original-ram-test").unwrap(),
        vec![PolynomialType::Dense(DenseType::PredecomposedDigits)],
    );
    let refs: [&dyn CommitmentSource<F>; 1] = [source];
    let resolved = compile_commitment_request(&source.plan, &refs, &capabilities)
        .unwrap()
        .materialize()
        .unwrap();
    backend
        .commit_resolved_inner_host::<128>(prepared, &resolved, source.plan)
        .unwrap()
        .remove(0)
        .coeffs()
        .to_vec()
}

// Independent schoolbook A multiplication, taking original signed coefficients
// or folded integer rows. No packed decoder, decomposition or NTT is used.
fn reference_commit(matrix: &[F], plan: CommitInnerPlan, digits: &[i32]) -> Vec<F> {
    let columns = plan.num_positions_per_block * 26;
    let mut result = vec![F::zero(); plan.num_live_blocks * plan.n_a * 128];
    for (index, &digit) in digits.iter().enumerate().filter(|(_, digit)| **digit != 0) {
        let block = index / (columns * 128);
        let column = index / 128 % columns;
        let j = index % 128;
        for row in 0..plan.n_a {
            for k in 0..128 {
                let product = matrix[(row * columns + column) * 128 + k] * field(digit);
                let output = (block * plan.n_a + row) * 128 + (j + k) % 128;
                if j + k < 128 {
                    result[output] += product;
                } else {
                    result[output] -= product;
                }
            }
        }
    }
    result
}

fn challenges() -> Vec<SparseChallenge> {
    vec![
        SparseChallenge {
            positions: vec![0, 1, 127].into(),
            coeffs: vec![1, -1, 2].into(),
        },
        SparseChallenge {
            positions: vec![3, 64, 126].into(),
            coeffs: vec![-2, 1, -1].into(),
        },
    ]
}

fn reference_fold(digits: &[i8], columns: usize, challenges: &[SparseChallenge]) -> Vec<i32> {
    let mut result = vec![0i32; columns * 128];
    for (index, &digit) in digits.iter().enumerate() {
        let block = index / (columns * 128);
        let column = index / 128 % columns;
        let j = index % 128;
        for (&shift, &coefficient) in challenges[block]
            .positions
            .iter()
            .zip(&challenges[block].coeffs)
        {
            let exponent = j + shift as usize;
            let product = i32::from(digit) * i32::from(coefficient);
            result[column * 128 + exponent % 128] +=
                if exponent < 128 { product } else { -product };
        }
    }
    result
}

#[test]
fn reviewed_ram_geometry_visits_original_planes_without_raw_grid_holes() {
    let plan = CommitInnerPlan {
        num_live_blocks: 16,
        num_positions_per_block: 32,
        ..plan()
    };
    let view = RawSourceViewPlan::new(128, 1, 16, 32, 26).unwrap();
    assert_eq!(view.padded_digits(), 32);
    let mut digits = vec![0; 16 * 32 * 26 * 128];
    let cells = [
        (0, 0, 0, 0, -16),
        (0, 0, 25, 127, 15),
        (0, 1, 0, 0, -1),
        (1, 0, 1, 64, 7),
        (15, 31, 25, 127, -9),
    ];
    let mut expected = Vec::new();
    for (block, position, digit, coefficient, value) in cells {
        let physical = (((block * 32 + position) * 26 + digit) * 128) + coefficient;
        digits[physical] = value;
        expected.push((physical, field(i32::from(value))));
        let raw = coefficient + 128 * (digit + 32 * (position + 32 * block));
        assert_eq!(view.raw_to_physical(raw).unwrap(), Some(physical));
    }
    let source = PackedRamSource::from_signed_digits(16, plan, digits).unwrap();
    let mut visited = Vec::new();
    source
        .visit_full_ring_digits(&plan, &mut |index, value: F| {
            visited.push((index, value));
            Ok(())
        })
        .unwrap();
    assert_eq!(visited, expected);
    for (index, value) in expected {
        assert_eq!(
            <PackedRamSource as FullRingSource<F>>::full_ring_digit(&source, &plan, index).unwrap(),
            value
        );
    }
    for block in 0..16 {
        for position in 0..32 {
            for hole in 26..32 {
                for coefficient in [0, 127] {
                    let raw = coefficient + 128 * (hole + 32 * (position + 32 * block));
                    assert_eq!(view.raw_to_physical(raw).unwrap(), None);
                }
            }
        }
    }
    assert_eq!(source.digits.len(), view.physical_len());
    assert_eq!(
        source.digits.encoded_bytes().len(),
        view.physical_len() * 5 / 8
    );
    assert!(<PackedRamSource as FullRingSource<F>>::full_ring_digit(
        &source,
        &plan,
        view.physical_len()
    )
    .is_err());
}

#[test]
fn original_signed_commitment_and_fold_match_schoolbook_and_each_other() {
    let plan = plan();
    let (backend, prepared) = prepared(plan);
    for seed in [3, 19] {
        let digits = patterned_digits(plan, seed);
        let source = PackedRamSource::from_signed_digits(9, plan, digits.clone()).unwrap();
        let allocation = source.digits.encoded_bytes().as_ptr();
        let actual = commit(&backend, &prepared, &source);
        assert_eq!(
            actual,
            reference_commit(
                prepared.matrix().as_field_slice(),
                plan,
                &digits
                    .iter()
                    .map(|&digit| i32::from(digit))
                    .collect::<Vec<_>>()
            )
        );
        let challenges = challenges();
        let refs = [&source];
        let folded = backend
            .decompose_fold_batch(
                None,
                refs.as_slice(),
                DecomposeFoldBatchPlan::Sparse {
                    challenges: &challenges,
                    num_positions_per_block: 2,
                    num_digits: 26,
                    log_basis: 5,
                },
            )
            .unwrap()
            .global;
        assert_eq!(
            folded.centered_coeffs_flat(),
            reference_fold(&digits, 52, &challenges)
        );
        let committed_fold = reference_commit(
            prepared.matrix().as_field_slice(),
            CommitInnerPlan {
                num_live_blocks: 1,
                ..plan
            },
            folded.centered_coeffs_flat(),
        );
        let mut folded_commitment = vec![F::zero(); plan.n_a * 128];
        for block in 0..2 {
            for row in 0..plan.n_a {
                for j in 0..128 {
                    for (&shift, &coefficient) in challenges[block]
                        .positions
                        .iter()
                        .zip(&challenges[block].coeffs)
                    {
                        let exponent = j + shift as usize;
                        let product = actual[(block * plan.n_a + row) * 128 + j]
                            * field(i32::from(coefficient));
                        if exponent < 128 {
                            folded_commitment[row * 128 + exponent] += product;
                        } else {
                            folded_commitment[row * 128 + exponent - 128] -= product;
                        }
                    }
                }
            }
        }
        assert_eq!(committed_fold, folded_commitment);
        assert_eq!(source.digits.encoded_bytes().as_ptr(), allocation);
        // Chunk views must preserve the original block offset and plane order.
        for block in 0..2 {
            let chunk = source
                .fold_chunk(&challenges[block..block + 1], block..block + 1)
                .unwrap();
            assert_eq!(
                chunk.centered_coeffs_flat(),
                reference_fold(
                    &digits[block * 52 * 128..(block + 1) * 52 * 128],
                    52,
                    &challenges[block..block + 1],
                )
            );
        }
    }
}

#[test]
fn chunked_ram_folds_preserve_empty_slots_and_the_global_response() {
    let plan = plan();
    let digits = patterned_digits(plan, 11);
    let source = PackedRamSource::from_signed_digits(9, plan, digits.clone()).unwrap();
    let backend = CpuBackend::<F, E>::for_arithmetic_tests();
    let challenges = akita_challenges::Challenges::from_sparse(challenges(), 2, 1).unwrap();
    let ranges = akita_params::dyadic_block_ranges(2, 8).unwrap();
    let sources = [&source];
    let sparse = backend
        .decompose_fold_batch(
            None,
            sources.as_slice(),
            DecomposeFoldBatchPlan::Sparse {
                challenges: challenges.as_slice(),
                num_positions_per_block: 2,
                num_digits: 26,
                log_basis: 5,
            },
        )
        .unwrap();
    let chunked = backend
        .decompose_fold_batch(
            None,
            sources.as_slice(),
            DecomposeFoldBatchPlan::SparseChunked {
                challenges: &challenges,
                chunk_ranges: &ranges,
                num_positions_per_block: 2,
                num_digits: 26,
                log_basis: 5,
            },
        )
        .unwrap();
    assert_eq!(chunked.global, sparse.global);
    let chunks = chunked.chunks.unwrap();
    assert_eq!(chunks.len(), 8);
    for (range, chunk) in ranges.iter().zip(chunks) {
        // Empty canonical slots have the complete ambient width and zero
        // coefficients; nonempty slots retain their original block offset.
        assert_eq!(
            chunk.centered_coeffs_flat(),
            reference_fold(
                &digits[range.start * 52 * 128..range.end * 52 * 128],
                52,
                &challenges.as_slice()[range.clone()],
            )
        );
    }
}

fn modulus_path() -> [i8; 26] {
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
    digits
}

#[test]
fn modulus_separated_legal_paths_remain_distinct_original_sources() {
    let plan = plan();
    let zero = PackedRamSource::from_signed_digits(9, plan, vec![0; 4 * 26 * 128]).unwrap();
    let mut digits = vec![0; 4 * 26 * 128];
    let modulus_digits = modulus_path();
    let decoded = modulus_digits.iter().rev().fold(F::zero(), |acc, &digit| {
        acc * F::from_u64(32) + field(i32::from(digit))
    });
    assert_eq!(decoded, F::zero());
    for (ell, digit) in modulus_digits.into_iter().enumerate() {
        digits[ell * 128 + 37] = digit;
    }
    let alternate = PackedRamSource::from_signed_digits(9, plan, digits.clone()).unwrap();
    let modulus = akita_params::field_modulus::<F>().unwrap();
    for source in [&zero, &alternate] {
        assert_eq!(
            <PackedRamSource as CommitmentSource<F>>::committed_centered_reach(
                source,
                modulus,
                modulus / 2
            )
            .unwrap(),
            (0, 0)
        );
    }
    assert_ne!(zero.digits, alternate.digits);
    let (backend, prepared) = prepared(plan);
    let zero_commit = commit(&backend, &prepared, &zero);
    let alternate_commit = commit(&backend, &prepared, &alternate);
    assert_eq!(
        alternate_commit,
        reference_commit(
            prepared.matrix().as_field_slice(),
            plan,
            &digits
                .iter()
                .map(|&digit| i32::from(digit))
                .collect::<Vec<_>>()
        )
    );
    assert_ne!(zero_commit, alternate_commit);
    let challenges = challenges();
    assert_ne!(
        zero.fold_chunk(&challenges, 0..2).unwrap(),
        alternate.fold_chunk(&challenges, 0..2).unwrap()
    );
    let mut visited = Vec::new();
    alternate
        .visit_full_ring_digits(&plan, &mut |index, digit: F| {
            visited.push((index, digit));
            Ok(())
        })
        .unwrap();
    assert_eq!(
        visited,
        digits
            .iter()
            .enumerate()
            .filter(|(_, digit)| **digit != 0)
            .map(|(index, &digit)| (index, field(i32::from(digit))))
            .collect::<Vec<_>>()
    );
}

#[test]
fn explicit_physical_padding_is_zero_but_raw_padding_is_not_stored() {
    let plan = plan();
    let mut digits = vec![0; 4 * 26 * 128];
    for ell in 0..26 {
        digits[ell * 128 + ell] = (ell % 32) as i8 - 16;
    }
    let source = PackedRamSource::from_signed_digits(7, plan, digits.clone()).unwrap();
    for index in 26 * 128..digits.len() {
        assert_eq!(
            <PackedRamSource as FullRingSource<F>>::full_ring_digit(&source, &plan, index).unwrap(),
            F::zero()
        );
    }
    let (backend, prepared) = prepared(plan);
    assert_eq!(
        commit(&backend, &prepared, &source),
        reference_commit(
            prepared.matrix().as_field_slice(),
            plan,
            &digits
                .iter()
                .map(|&digit| i32::from(digit))
                .collect::<Vec<_>>()
        )
    );
    let view = RawSourceViewPlan::new(128, 1, 2, 2, 26).unwrap();
    let point = FullRingRawPoint::<E>::new(
        &view,
        (0..view.num_vars())
            .map(|i| E::from_base_slice(&[F::from_u64(i as u64 + 2), F::from_u64(i as u64 + 7)]))
            .collect(),
    )
    .unwrap();
    let partials =
        crate::opaque::full_ring::source_partials(std::slice::from_ref(&source), &plan, &point)
            .unwrap();
    let basis = |index: usize| {
        point
            .point()
            .iter()
            .enumerate()
            .fold(E::one(), |w, (bit, &r)| {
                w * if (index >> bit) & 1 == 0 {
                    E::one() - r
                } else {
                    r
                }
            })
    };
    let expected = (0..26).fold(E::zero(), |sum, ell| {
        sum + basis(ell + 128 * ell).mul_base(field(i32::from(digits[ell * 128 + ell])))
    });
    assert_eq!(point.scalar_from_partials(&partials).unwrap(), expected);
    assert!(partials
        .plane(0, 1, 0)
        .unwrap()
        .iter()
        .all(|value| *value == F::zero()));
    assert!(partials
        .plane(0, 1, 1)
        .unwrap()
        .iter()
        .all(|value| *value == F::zero()));
    for (ell, digit) in modulus_path().into_iter().enumerate() {
        digits[(26 + ell) * 128] = digit;
    }
    assert!(
        PackedRamSource::from_signed_digits(7, plan, digits).is_err(),
        "zero in K is not zero physical padding"
    );
}

#[test]
fn rejects_invalid_shapes_encodings_and_every_plan_axis() {
    let plan = plan();
    let len = 4 * 26 * 128;
    for bad in [-17, 16, i8::MIN, i8::MAX] {
        let mut digits = vec![0; len];
        digits[129] = bad;
        assert!(PackedRamSource::from_signed_digits(9, plan, digits).is_err());
    }
    for len in [len - 1, len + 1, 4 * 32 * 128] {
        assert!(PackedRamSource::from_signed_digits(9, plan, vec![0; len]).is_err());
    }
    for vars in [0, 6, 10, usize::BITS as usize] {
        assert!(PackedRamSource::from_signed_digits(vars, plan, vec![0; len]).is_err());
    }
    for bad in [
        CommitInnerPlan {
            ring_dimension: 64,
            ..plan
        },
        CommitInnerPlan {
            num_digits_inner: 32,
            ..plan
        },
        CommitInnerPlan {
            log_basis_inner: 6,
            ..plan
        },
        CommitInnerPlan {
            num_positions_per_block: 3,
            ..plan
        },
        CommitInnerPlan { n_a: 0, ..plan },
        CommitInnerPlan {
            num_live_blocks: 0,
            ..plan
        },
        CommitInnerPlan {
            num_live_blocks: usize::MAX,
            ..plan
        },
    ] {
        assert!(PackedRamSource::from_signed_digits(9, bad, vec![0; len]).is_err());
    }
    let source = PackedRamSource::from_signed_digits(9, plan, vec![0; len]).unwrap();
    let selection =
        <PackedRamSource as CommitmentSource<F>>::available_polynomial_types(&source, &plan)
            .unwrap()
            .select(0)
            .unwrap();
    for bad in [
        CommitInnerPlan {
            ring_dimension: 64,
            ..plan
        },
        CommitInnerPlan {
            num_digits_inner: 25,
            ..plan
        },
        CommitInnerPlan {
            log_basis_inner: 4,
            ..plan
        },
        CommitInnerPlan {
            num_positions_per_block: 4,
            ..plan
        },
        CommitInnerPlan { n_a: 2, ..plan },
        CommitInnerPlan {
            num_live_blocks: 1,
            ..plan
        },
    ] {
        assert!(
            <PackedRamSource as CommitmentSource<F>>::represent_as(&source, selection, &bad)
                .is_err()
        );
        assert!(<PackedRamSource as FullRingSource<F>>::full_ring_digit(&source, &bad, 0).is_err());
        assert!(source
            .visit_full_ring_digits(&bad, &mut |_, _: F| panic!(
                "visitor must not run under a changed plan"
            ))
            .is_err());
    }
    assert!(source.fold_chunk(&challenges(), 1..3).is_err());
    let (backend, _) = prepared(plan);
    for (positions, digits, log_basis) in [(1, 26, 5), (2, 32, 5), (2, 26, 6)] {
        assert!(backend
            .decompose_fold_batch(
                None,
                [&source].as_slice(),
                DecomposeFoldBatchPlan::Sparse {
                    challenges: &challenges(),
                    num_positions_per_block: positions,
                    num_digits: digits,
                    log_basis,
                }
            )
            .is_err());
    }
}

fn packing_point(
    positions: usize,
    geometry: akita_params::SubringCoefficientPackingGeometry,
) -> akita_types::PreparedSubringCoefficientPackingPoint<F> {
    akita_types::PreparedSubringCoefficientPackingPoint::new(
        geometry,
        akita_params::BasisMode::Lagrange,
        4,
        positions,
        9,
        &(0..9).map(|i| F::from_u64(i + 2)).collect::<Vec<_>>(),
    ).unwrap()
}

#[test]
fn ordinary_native_packing_preserves_all_original_planes_and_nonzero_quotient() {
    use crate::opaque::{SubringCoefficientPackingBatchKernel, SubringCoefficientPackingPlan};
    let geometry = akita_params::SubringCoefficientPackingGeometry::try_new(1, 128, 128).unwrap();
    let point = packing_point(2, geometry);
    let backend = CpuBackend::<F, F>::for_arithmetic_tests();
    for seed in [3, 19] {
        let digits = patterned_digits(plan(), seed);
        let source = PackedRamSource::from_signed_digits(9, plan(), digits.clone()).unwrap();
        let allocation = source.digits.encoded_bytes().as_ptr();
        let partials = backend.coefficient_packing_partials_batch(
            None, [&source].as_slice(), SubringCoefficientPackingPlan { point: &point },
        ).unwrap();
        let mut expected = vec![F::zero(); 256];
        let mut alpha = F::one();
        for ell in 0..26 {
            for block in 0..2 {
                for x in 0..2 {
                    for j in 0..128 {
                        expected[block * 128 + j] += point.position_weights()[x] * alpha
                            * field(i32::from(digits[((block * 2 + x) * 26 + ell) * 128 + j]));
                    }
                }
            }
            alpha *= F::from_u64(32);
        }
        assert_eq!(partials.len(), 1);
        assert_eq!(partials[0].coordinates(), expected);
        let scalar = crate::arithmetic::coefficient_packing_fold::coefficient_packing_scalar_opening::<F, F>(
            geometry, 2, &partials, &[F::one()], point.live_block_weights(), point.tail_weights(),
        ).unwrap();
        let query = (0..9).map(|i| F::from_u64(i + 2)).collect::<Vec<_>>();
        let mut reference_scalar = F::zero();
        for logical in 0..512 {
            let weight = query.iter().enumerate().fold(F::one(), |weight, (bit, r)| {
                weight * if logical >> bit & 1 == 0 { F::one() - r } else { *r }
            });
            let mut alpha = F::one();
            for ell in 0..26 {
                reference_scalar += weight * alpha
                    * field(i32::from(digits[((logical / 128) * 26 + ell) * 128 + logical % 128]));
                alpha *= F::from_u64(32);
            }
        }
        assert_eq!(scalar, reference_scalar);

        let challenges = challenges();
        let product = crate::arithmetic::coefficient_packing_fold::fold_coefficient_packing_partials(
            geometry, &challenges, partials[0].coordinates(),
        ).unwrap();
        let mut ordinary_product = vec![F::zero(); 256];
        for (block, challenge) in challenges.iter().enumerate() {
            for (&shift, &coefficient) in challenge.positions.iter().zip(&challenge.coeffs) {
                for j in 0..128 {
                    ordinary_product[j + shift as usize] += expected[block * 128 + j]
                        * field(i32::from(coefficient));
                }
            }
        }
        let quotient = product.quotient_high_half_base_field_coordinates();
        assert_eq!(quotient, &ordinary_product[128..]);
        assert!(quotient.iter().any(|q| *q != F::zero()));
        let folded = source.fold_chunk(&challenges, 0..2).unwrap();
        let z = folded.centered_coeffs_flat();
        for j in 0..128 {
            let mut reduced = F::zero();
            for x in 0..2 {
                let mut alpha = F::one();
                for ell in 0..26 {
                    reduced += point.position_weights()[x] * alpha * field(z[(x * 26 + ell) * 128 + j]);
                    alpha *= F::from_u64(32);
                }
            }
            assert_eq!(product.reduced_base_field_coordinates()[j], reduced);
            assert_eq!(ordinary_product[j] - quotient[j], reduced);
        }
        assert_eq!(source.digits.encoded_bytes().as_ptr(), allocation);
        let wrong_blocks = packing_point(4, geometry);
        assert!(backend.coefficient_packing_partials_batch(
            None, [&source].as_slice(), SubringCoefficientPackingPlan { point: &wrong_blocks },
        ).is_err());
        let wrong_method = packing_point(2,
            akita_params::SubringCoefficientPackingGeometry::try_new(1, 128, 64).unwrap());
        assert!(backend.coefficient_packing_partials_batch(
            None, [&source].as_slice(), SubringCoefficientPackingPlan { point: &wrong_method },
        ).is_err());
    }
}

#[test]
fn ordinary_equal_logical_claims_do_not_replace_original_fold_paths() {
    use crate::opaque::{SubringCoefficientPackingBatchKernel, SubringCoefficientPackingPlan};
    let geometry = akita_params::SubringCoefficientPackingGeometry::try_new(1, 128, 128).unwrap();
    let point = packing_point(2, geometry);
    let backend = CpuBackend::<F, F>::for_arithmetic_tests();
    let zero = PackedRamSource::from_signed_digits(9, plan(), vec![0; 4 * 26 * 128]).unwrap();
    let mut alternate = vec![0; 4 * 26 * 128];
    for (ell, digit) in modulus_path().into_iter().enumerate() {
        alternate[ell * 128 + 127] = digit;
    }
    let alternate = PackedRamSource::from_signed_digits(9, plan(), alternate).unwrap();
    let partials = backend.coefficient_packing_partials_batch(
        None, [&zero, &alternate].as_slice(), SubringCoefficientPackingPlan { point: &point },
    ).unwrap();
    assert_eq!(partials[0], partials[1]);
    assert!(partials[0].coordinates().iter().all(|v| *v == F::zero()));
    let challenges = challenges();
    assert_ne!(zero.fold_chunk(&challenges, 0..2).unwrap(),
        alternate.fold_chunk(&challenges, 0..2).unwrap());
    assert_ne!(zero.digits, alternate.digits);
}

fn raw_ram_plan() -> CommitInnerPlan {
    CommitInnerPlan { num_live_blocks: 16, num_positions_per_block: 32, ..plan() }
}

fn raw_ram_point(point: &[F]) -> akita_types::PreparedSubringCoefficientPackingPoint<F> {
    akita_types::PreparedSubringCoefficientPackingPoint::new_original_ram_planes(
        akita_params::SubringCoefficientPackingGeometry::try_new(1, 128, 128).unwrap(),
        akita_params::BasisMode::Lagrange, 512, 32, 16, point,
    ).unwrap()
}

fn eq_at(point: &[F], index: usize) -> F {
    point.iter().enumerate().fold(F::one(), |weight, (bit, &r)| {
        weight * if index >> bit & 1 == 0 { F::one() - r } else { r }
    })
}

#[test]
fn raw_native_partials_and_ring_consistency_keep_all_26_original_planes() {
    use crate::opaque::{SubringCoefficientPackingBatchKernel, SubringCoefficientPackingPlan};
    let plan = raw_ram_plan();
    let backend = CpuBackend::<F, F>::for_arithmetic_tests();
    for seed in [2, 17] {
        let digits = patterned_digits(plan, seed);
        let source = PackedRamSource::from_signed_digits(16, plan, digits.clone()).unwrap();
        let allocation = source.digits.encoded_bytes().as_ptr();
        let query = (0..21).map(|i| F::from_u64((i + seed + 2) as u64)).collect::<Vec<_>>();
        let point = raw_ram_point(&query);
        let partials = backend.coefficient_packing_partials_batch(
            None, [&source].as_slice(), SubringCoefficientPackingPlan { point: &point },
        ).unwrap();
        let mut expected = vec![F::zero(); 16 * 128];
        let mut expected_scalar = F::zero();
        for block in 0..16 {
            for position in 0..32 {
                for digit in 0..26 {
                    let position_digit = eq_at(&query[12..17], position) * eq_at(&query[7..12], digit);
                    for coefficient in 0..128 {
                        let value = field(i32::from(digits[((block * 32 + position) * 26 + digit) * 128 + coefficient]));
                        expected[block * 128 + coefficient] += position_digit * value;
                        expected_scalar += eq_at(&query[..7], coefficient) * position_digit
                            * eq_at(&query[17..], block) * value;
                    }
                }
            }
        }
        assert_eq!(partials[0].coordinates(), expected);
        let scalar = crate::arithmetic::coefficient_packing_fold::coefficient_packing_scalar_opening::<F, F>(
            point.geometry(), 16, &partials, &[F::one()], point.live_block_weights(), point.tail_weights(),
        ).unwrap();
        assert_eq!(scalar, expected_scalar);
        let shell = (0..16).map(|block| SparseChallenge {
            positions: vec![0, (block + 1) as u32, 127].into(),
            coeffs: vec![1, -1, 2].into(),
        }).collect::<Vec<_>>();
        let product = crate::arithmetic::coefficient_packing_fold::fold_coefficient_packing_partials(
            point.geometry(), &shell, partials[0].coordinates(),
        ).unwrap();
        let folded = source.fold_chunk(&shell, 0..16).unwrap();
        let z = folded.centered_coeffs_flat();
        for coefficient in 0..128 {
            let mut z_query = F::zero();
            for position in 0..32 {
                for digit in 0..26 {
                    z_query += eq_at(&query[12..17], position) * eq_at(&query[7..12], digit)
                        * field(z[(position * 26 + digit) * 128 + coefficient]);
                }
            }
            assert_eq!(product.reduced_base_field_coordinates()[coefficient], z_query);
        }
        assert!(product.quotient_high_half_base_field_coordinates().iter().any(|&value| value != F::zero()));
        assert_eq!(source.digits.encoded_bytes().as_ptr(), allocation);
    }
}

#[test]
fn raw_top_plane_and_zero_weights_do_not_decode_or_normalize_the_source() {
    use crate::opaque::{SubringCoefficientPackingBatchKernel, SubringCoefficientPackingPlan};
    let plan = raw_ram_plan();
    let backend = CpuBackend::<F, F>::for_arithmetic_tests();
    let mut digits = vec![0; 16 * 32 * 26 * 128];
    // A legal original path for q: decoded coefficient zero, distinct raw endpoints.
    for (digit, value) in [(0, 9), (2, -10), (3, 1), (6, -4), (25, 8)] {
        digits[digit * 128] = value;
    }
    let source = PackedRamSource::from_signed_digits(16, plan, digits).unwrap();
    let zero = PackedRamSource::from_signed_digits(16, plan, vec![0; 16 * 32 * 26 * 128]).unwrap();
    let geometry = akita_params::SubringCoefficientPackingGeometry::try_new(1, 128, 128).unwrap();
    let decoded = akita_types::PreparedSubringCoefficientPackingPoint::new(
        geometry, akita_params::BasisMode::Lagrange, 512, 32, 16, &[F::zero(); 16],
    ).unwrap();
    let old = backend.coefficient_packing_partials_batch(
        None, [&source, &zero].as_slice(), SubringCoefficientPackingPlan { point: &decoded },
    ).unwrap();
    assert_eq!(old[0], old[1]);
    for (digit, expected) in [(0, F::from_u64(9)), (25, F::from_u64(8)), (26, F::zero()), (31, F::zero())] {
        let mut query = [F::zero(); 21];
        for bit in 0..5 { query[7 + bit] = F::from_u64((digit >> bit & 1) as u64); }
        let point = raw_ram_point(&query);
        let partial = backend.coefficient_packing_partials_batch(
            None, [&source].as_slice(), SubringCoefficientPackingPlan { point: &point },
        ).unwrap();
        let scalar = crate::arithmetic::coefficient_packing_fold::coefficient_packing_scalar_opening::<F, F>(
            geometry, 16, &partial, &[F::one()], point.live_block_weights(), point.tail_weights(),
        ).unwrap();
        assert_eq!(scalar, expected);
        assert_eq!(partial[0].coordinates().len(), 16 * 128);
    }
}

#[test]
fn raw_recipe_rejects_decoded_kernels_and_wrong_original_plan_before_contraction() {
    use crate::opaque::{RootOpeningSource, SubringCoefficientPackingBatchKernel, SubringCoefficientPackingPlan};
    let point = raw_ram_point(&[F::zero(); 21]);
    let plan = SubringCoefficientPackingPlan { point: &point };
    assert!(plan.validate_decoded().is_err());
    let backend = CpuBackend::<F, F>::for_arithmetic_tests();
    let dense = crate::DensePoly::from_ring_coeffs(
        vec![akita_algebra::CyclotomicRing::<F, 128>::zero(); 512],
    ).unwrap();
    let polys = [&dense];
    let batch = <crate::DensePoly<F> as RootOpeningSource<F, 128>>::opening_batch(&polys).unwrap();
    assert!(backend.coefficient_packing_partials_batch(None, batch, plan).is_err());
    let onehot = crate::OneHotPoly::<F>::new(128, vec![Some(0); 512]).unwrap();
    let polys = [&onehot];
    let batch = <crate::OneHotPoly<F> as RootOpeningSource<F, 128>>::opening_batch(&polys).unwrap();
    assert!(backend.coefficient_packing_partials_batch(None, batch, plan).is_err());
    let wrong_rank = CommitInnerPlan { n_a: 2, ..raw_ram_plan() };
    let source = PackedRamSource::from_signed_digits(16, wrong_rank, vec![0; 16 * 32 * 26 * 128]).unwrap();
    assert!(backend.coefficient_packing_partials_batch(None, [&source].as_slice(), plan).is_err());
    let geometry = point.geometry();
    for basis in [akita_params::BasisMode::Monomial] {
        assert!(akita_types::PreparedSubringCoefficientPackingPoint::<F>::new_original_ram_planes(
            geometry, basis, 512, 32, 16, &[F::zero(); 21],
        ).is_err());
    }
    for length in [16, 20, 22] {
        assert!(akita_types::PreparedSubringCoefficientPackingPoint::<F>::new_original_ram_planes(
            geometry, akita_params::BasisMode::Lagrange, 512, 32, 16, &vec![F::zero(); length],
        ).is_err());
    }
}
