use crate::full_ring::*;
use akita_types::full_ring::*;
use jolt_field::{ExtField, One, Prime128OffsetA7F7 as K, Prime128OffsetA7F7Ext2 as E, Ring, Zero};

fn e(a: u64, b: u64) -> E {
    E::from_base_fn(|i| K::from_u64(if i == 0 { a } else { b }))
}

#[test]
fn replay_checks_both_product_stages_and_one_virtual_value() {
    let source = vec![
        K::from_i64(-16),
        K::from_i64(-1),
        K::zero(),
        K::from_u64(15),
    ];
    let tau = [e(2, 3), e(5, 7)];
    let mut prover = ProductRangeSession::new(&source, &tau).unwrap();
    let mut verifier = FullRingRangeVerifier::new(&tau).unwrap();
    assert!(verifier.begin_leaf_stage([E::one(); 3]).is_err());
    for challenge in [e(11, 13), e(17, 19)] {
        let coefficients = prover.round_coefficients().unwrap();
        let mut malformed = coefficients;
        malformed[5] += E::one();
        assert!(verifier.receive_coefficients(&malformed).is_err());
        assert!(verifier.receive_coefficients(&coefficients[..5]).is_err());
        verifier.receive_coefficients(&coefficients).unwrap();
        prover.bind(challenge).unwrap();
        verifier.bind(challenge).unwrap();
    }
    verifier
        .receive_quarters(prover.quarter_claims().unwrap())
        .unwrap();
    let xi = [e(23, 29), e(31, 37), e(41, 43)];
    prover.begin_leaf_stage(xi).unwrap();
    verifier.begin_leaf_stage(xi).unwrap();
    for challenge in [e(47, 53), e(59, 61)] {
        verifier
            .receive_coefficients(&prover.round_coefficients().unwrap())
            .unwrap();
        prover.bind(challenge).unwrap();
        verifier.bind(challenge).unwrap();
    }
    let claim = prover.virtual_claim().unwrap();
    let pending = verifier.receive_virtual(claim).unwrap();
    assert_eq!(pending.value(), claim);
    assert_eq!(pending.point(), prover.point());
    assert!(verifier.receive_virtual(claim).is_err());
}

#[test]
fn source_domain_replay_rejects_unbound_endpoint_substitution() {
    let view = RawSourceViewPlan::new(2, 1, 1, 1, 1).unwrap();
    // Required-zero support is a list of intervals, not a vector of indices.
    #[expect(
        clippy::single_range_in_vec_init,
        reason = "one physical zero interval is intentional"
    )]
    let domain = SourceDomainPlan::new(
        view,
        SourceAlphabet::BooleanWholeGrid,
        CoefficientSupport::new(2, vec![0..1]).unwrap(),
    )
    .unwrap();
    let source = vec![K::zero(), K::one()];
    let anchor = [e(2, 3)];
    let eta = e(5, 7);
    let mut prover = SourceDomainSession::new(&source, &domain, &anchor, eta).unwrap();
    let coefficients = prover.round_coefficients().unwrap().to_vec();
    let r = e(11, 13);
    prover.bind(r).unwrap();
    let mut verifier = FullRingSourceDomainVerifier::new(&domain, &anchor, eta).unwrap();
    verifier.receive_coefficients(&coefficients).unwrap();
    verifier.bind(r).unwrap();
    assert_eq!(
        verifier
            .finish(prover.raw_value().unwrap())
            .unwrap()
            .value(),
        r
    );
    let mut verifier = FullRingSourceDomainVerifier::new(&domain, &anchor, eta).unwrap();
    verifier.receive_coefficients(&coefficients).unwrap();
    verifier.bind(r).unwrap();
    assert!(verifier.finish(r + E::one()).is_err());
}

#[test]
fn fused_replay_authenticates_real_matrix_scan_and_same_w_value() {
    let view = RawSourceViewPlan::new(2, 1, 1, 1, 1).unwrap();
    let point = FullRingRawPoint::new(&view, vec![e(2, 3)]).unwrap();
    let matrix = [K::one(), K::from_u64(2)];
    let d = [
        K::from_u64(3),
        K::from_u64(5),
        K::from_u64(7),
        K::from_u64(11),
    ];
    let challenge = [K::one(), K::one()];
    let payload = [K::zero(); 2];
    let group = FullRingRelationGroup {
        view: &view,
        a: NativeMatrix::new(1, 1, 2, &matrix).unwrap(),
        b: NativeMatrix::new(1, 1, 2, &matrix).unwrap(),
        challenges: &challenge,
        response: DigitLayout::new(0, 1, 2, 1, K::from_u64(32)).unwrap(),
        inner_images: DigitLayout::new(6, 1, 2, 1, K::from_u64(32)).unwrap(),
        channels: vec![FullRingRelationChannel {
            point: &point,
            partials: DigitLayout::new(2, 2, 2, 1, K::from_u64(32)).unwrap(),
            scalar_claims: vec![PartialScalarClaim::raw(&point, E::zero())],
        }],
        f: CompressionChain {
            stages: vec![],
            payload: &payload,
        },
    };
    let relation = CoefficientRelationPlan::new(
        16,
        vec![group],
        NativeMatrix::new(1, 2, 2, &d).unwrap(),
        CompressionChain {
            stages: vec![],
            payload: &payload,
        },
    )
    .unwrap();
    let source = vec![K::zero(); 16];
    let tau = vec![e(13, 17); 4];
    let mut range_prover = ProductRangeSession::new(&source, &tau).unwrap();
    let mut range_verifier = FullRingRangeVerifier::new(&tau).unwrap();
    for stage in 0..2 {
        for round in 0..4 {
            let coeffs = range_prover.round_coefficients().unwrap();
            let r = e(19 + round, 23 + stage);
            range_verifier.receive_coefficients(&coeffs).unwrap();
            range_prover.bind(r).unwrap();
            range_verifier.bind(r).unwrap();
        }
        if stage == 0 {
            range_verifier
                .receive_quarters(range_prover.quarter_claims().unwrap())
                .unwrap();
            let xi = [e(29, 31), e(37, 41), e(43, 47)];
            range_prover.begin_leaf_stage(xi).unwrap();
            range_verifier.begin_leaf_stage(xi).unwrap();
        }
    }
    let virtual_claim = range_verifier
        .receive_virtual(range_prover.virtual_claim().unwrap())
        .unwrap();
    let rho = vec![e(53, 59); relation.row_num_vars()];
    let mix = StrongMix {
        gamma: e(61, 67),
        eta_bin: e(71, 73),
        eta_zero: e(79, 83),
    };
    let mut prover = FusedRelationSession::new(
        &source,
        &relation,
        &rho,
        &tau,
        virtual_claim.point(),
        virtual_claim.value(),
        mix,
    )
    .unwrap();
    let mut verifier =
        FullRingFusedVerifier::new(&relation, &rho, &tau, virtual_claim, mix).unwrap();
    for round in 0..4 {
        verifier
            .receive_coefficients(&prover.round_coefficients().unwrap())
            .unwrap();
        let r = e(89 + round, 97);
        prover.bind(r).unwrap();
        verifier.bind(r).unwrap();
    }
    let endpoint = verifier.finish(prover.witness_value().unwrap()).unwrap();
    assert_eq!(endpoint.point(), prover.point());
    assert_eq!(endpoint.value(), E::zero());
}

#[test]
fn repeated_same_source_normalization_keeps_and_checks_previous_opening() {
    // Real nonconstant K source and full coefficient sessions. This exercises
    // the repeated prefix arithmetic, not a compact installed root fixture.
    let view = RawSourceViewPlan::new(2, 1, 1, 1, 1).unwrap();
    let domain = SourceDomainPlan::new(
        view.clone(),
        SourceAlphabet::BooleanWholeGrid,
        CoefficientSupport::new(2, std::iter::once(0..1).collect()).unwrap(),
    )
    .unwrap();
    let source = vec![K::zero(), K::one()];
    let mut point = vec![e(2, 3)];
    let mut value = point[0];
    let mut count = akita_types::full_ring_accumulation::FullRingUpdateCount::new(16).unwrap();
    for occurrence in 0..16 {
        assert!(count.require_complete().is_err());
        let anchor = [e(5 + occurrence, 7)];
        let eta = e(11, 13 + occurrence);
        let mut domain_prover = SourceDomainSession::new(&source, &domain, &anchor, eta).unwrap();
        let mut domain_verifier = FullRingSourceDomainVerifier::new(&domain, &anchor, eta).unwrap();
        domain_verifier
            .receive_coefficients(domain_prover.round_coefficients().unwrap())
            .unwrap();
        let r = e(17 + occurrence, 19);
        domain_prover.bind(r).unwrap();
        domain_verifier.bind(r).unwrap();
        let endpoint = domain_verifier
            .finish(domain_prover.raw_value().unwrap())
            .unwrap();
        assert_eq!(endpoint.value(), r);
        let alpha = vec![E::one(), e(23, 29 + occurrence)];
        let plan = NormalizationPlan::new(
            &view,
            vec![
                NormalizationClaim::Raw {
                    point: &point,
                    value,
                },
                NormalizationClaim::Raw {
                    point: endpoint.point(),
                    value: endpoint.value(),
                },
            ],
            alpha.clone(),
        )
        .unwrap();
        let wrong_prior = NormalizationPlan::new(
            &view,
            vec![
                NormalizationClaim::Raw {
                    point: &point,
                    value: value + E::one(),
                },
                NormalizationClaim::Raw {
                    point: endpoint.point(),
                    value: endpoint.value(),
                },
            ],
            alpha,
        )
        .unwrap();
        let mut prover = NormalizationSession::new(&source, &plan).unwrap();
        let coefficients = prover.round_coefficients().unwrap();
        let mut wrong_verifier = FullRingNormalizationVerifier::new(&wrong_prior).unwrap();
        assert!(wrong_verifier.receive_coefficients(&coefficients).is_err());
        let mut verifier = FullRingNormalizationVerifier::new(&plan).unwrap();
        verifier.receive_coefficients(&coefficients).unwrap();
        let r = e(31 + occurrence, 37);
        prover.bind(r).unwrap();
        verifier.bind(r).unwrap();
        let next = verifier.finish(prover.raw_value().unwrap()).unwrap();
        assert_eq!(next.value(), r);
        (point, value) = next.into_parts();
        count.complete_update().unwrap();
    }
    count.require_complete().unwrap();
    assert!(count.complete_update().is_err());
}

#[test]
fn fresh_range_realization_replay_checks_anchor_presence_and_raw_endpoint() {
    let view = RawSourceViewPlan::new(2, 3, 1, 1, 3)
        .unwrap()
        .with_permutation((0..18).rev().collect())
        .unwrap();
    let source: Vec<K> = (0..18)
        .map(|i| K::from_i64([0, -1, -16, 15][i % 4]))
        .collect();
    let domain = SourceDomainPlan::new(
        view.clone(),
        SourceAlphabet::BalancedBase32,
        CoefficientSupport::new(18, vec![]).unwrap(),
    )
    .unwrap();
    let raw = RawSourceReader::new(&source, &view).unwrap();
    let tau = vec![e(2, 3); 5];
    let mut producer = ProductRangeSession::new(&raw, &tau).unwrap();
    let mut replay = FullRingRangeVerifier::new(&tau).unwrap();
    for stage in 0..2 {
        for round in 0..5 {
            replay
                .receive_coefficients(&producer.round_coefficients().unwrap())
                .unwrap();
            let r = e(5 + round, 11 + stage);
            producer.bind(r).unwrap();
            replay.bind(r).unwrap();
        }
        if stage == 0 {
            replay
                .receive_quarters(producer.quarter_claims().unwrap())
                .unwrap();
            let xi = [e(13, 17), e(19, 23), e(29, 31)];
            producer.begin_leaf_stage(xi).unwrap();
            replay.begin_leaf_stage(xi).unwrap();
        }
    }
    let claim = replay
        .receive_virtual(producer.virtual_claim().unwrap())
        .unwrap();
    drop(producer);
    assert!(FullRingSourceRangeRealizationVerifier::new(
        &domain,
        &[e(3, 2); 5],
        claim.clone(),
        None,
    )
    .is_err());
    assert!(
        FullRingSourceRangeRealizationVerifier::new(&domain, &tau[..4], claim.clone(), None,)
            .is_err()
    );
    assert!(FullRingSourceRangeRealizationVerifier::new(
        &domain,
        &tau,
        claim.clone(),
        Some(E::one()),
    )
    .is_err());
    assert!(FullRingSourceDomainVerifier::new(&domain, &tau, E::one()).is_err());
    let mut producer = SourceRangeRealizationSession::new(
        &source,
        &domain,
        &tau,
        claim.point(),
        claim.value(),
        None,
    )
    .unwrap();
    let mut good =
        FullRingSourceRangeRealizationVerifier::new(&domain, &tau, claim.clone(), None).unwrap();
    let mut wrong =
        FullRingSourceRangeRealizationVerifier::new(&domain, &tau, claim, None).unwrap();
    for round in 0..5 {
        let coefficients = producer.round_coefficients().unwrap();
        assert!(good.receive_coefficients(&coefficients[..3]).is_err());
        good.receive_coefficients(coefficients).unwrap();
        wrong.receive_coefficients(coefficients).unwrap();
        let r = e(37 + round, 41);
        producer.bind(r).unwrap();
        good.bind(r).unwrap();
        wrong.bind(r).unwrap();
    }
    let value = producer.raw_value().unwrap();
    assert!(wrong.finish(value + E::one()).is_err());
    let pending = good.finish(value).unwrap();
    let expected = (0..32).fold(E::zero(), |sum, index| {
        let (p, digit, j) = (index / 8, (index / 2) % 4, index % 2);
        let w = if p == 3 || digit == 3 {
            K::zero()
        } else {
            source[17 - (p * 6 + digit * 2 + j)]
        };
        sum + akita_algebra::offset_eq::eq_eval_at_index(pending.point(), index).mul_base(w)
    });
    assert_eq!(pending.value(), expected);
}

/// A finite component fixture, not a production accumulation registry. It
/// independently checks the exact ordered requests, full canonical payloads,
/// site ownership and native completion for one source-prefix occurrence.
mod fresh_range_native {
    use super::*;
    use akita_transcript::native::{decode_e, encode_e, inner, Error};
    use akita_transcript::native_strong::{
        BalancedSourceDomainSites, BooleanSourceDomainSites, InnerDraw, InnerExecutionRegistry,
        InnerNativeRequest, NativeStrongTranscript, Operation, RoundSite, SourceDomainSites,
    };
    use akita_types::full_ring_accumulation::{
        full_ring_fresh_source_binding, FullRingFreshSourcePlan,
    };
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    #[derive(Default)]
    struct Log {
        completed: Cell<usize>,
        draws: Cell<usize>,
        elements: Cell<usize>,
        history: RefCell<Vec<Vec<u8>>>,
    }
    struct Registry {
        expected: Vec<InnerNativeRequest<'static>>,
        binding: Vec<u8>,
        log: Rc<Log>,
    }
    impl inner::Registry for Registry {
        fn context(&self, kind: u32, bytes: &[u8]) -> Result<(), Error> {
            if kind == 1 && bytes == b"fresh-range-component/v2" {
                Ok(())
            } else {
                Err(Error::Registry)
            }
        }
        fn record(&self, kind: u32, bytes: &[u8]) -> Result<(), Error> {
            if kind == 1 {
                return self.context(kind, bytes);
            }
            let expected = self
                .expected
                .get(self.log.completed.get())
                .ok_or(Error::Registry)?;
            let (record_kind, count, offset) = match *expected {
                InnerNativeRequest::Public {
                    kind: installed,
                    count: 1,
                } => {
                    return if kind == installed && bytes == self.binding.as_slice() {
                        Ok(())
                    } else {
                        Err(Error::Registry)
                    };
                }
                InnerNativeRequest::Proof { kind, count } => (kind, count, 0),
                InnerNativeRequest::Draw {
                    site,
                    result_kind,
                    operation,
                    count,
                } => {
                    if bytes.get(..8) != Some(site.to_le_bytes().as_slice())
                        || bytes.get(8) != Some(&(operation as u8))
                        || bytes.get(9..17) != Some((count as u64).to_le_bytes().as_slice())
                    {
                        return Err(Error::Registry);
                    }
                    (result_kind, count, 17)
                }
                _ => return Err(Error::Registry),
            };
            if kind != record_kind || bytes.len() != offset + count * 32 {
                return Err(Error::Registry);
            }
            for atom in bytes[offset..].chunks_exact(32) {
                decode_e(atom)?;
            }
            Ok(())
        }
        fn site(&self, site: u64) -> Result<(), Error> {
            if self.expected.iter().any(|request|
                matches!(request, InnerNativeRequest::Draw { site: installed, .. } if *installed == site)
            ) { Ok(()) } else { Err(Error::Registry) }
        }
    }
    impl InnerExecutionRegistry for Registry {
        fn preflight(&self, request: InnerNativeRequest<'_>) -> Result<(), Error> {
            if self.expected.get(self.log.completed.get()) == Some(&request) {
                Ok(())
            } else {
                Err(Error::Registry)
            }
        }
        fn published(&self, request: InnerNativeRequest<'_>, bytes: &[u8]) -> Result<(), Error> {
            self.preflight(request)?;
            let kind = match request {
                InnerNativeRequest::Public { kind, .. }
                | InnerNativeRequest::Proof { kind, .. } => kind,
                InnerNativeRequest::Draw { result_kind, .. } => result_kind,
                _ => return Err(Error::Registry),
            };
            inner::Registry::record(self, kind, bytes)?;
            match request {
                InnerNativeRequest::Proof { count, .. } => {
                    self.log.elements.set(self.log.elements.get() + count)
                }
                InnerNativeRequest::Draw { count, .. } => {
                    self.log.draws.set(self.log.draws.get() + count)
                }
                _ => {}
            }
            self.log.history.borrow_mut().push(bytes.to_vec());
            self.log.completed.set(self.log.completed.get() + 1);
            Ok(())
        }
        fn accepting(&self) -> Result<(), Error> {
            if self.log.completed.get() == self.expected.len() {
                Ok(())
            } else {
                Err(Error::Registry)
            }
        }
    }
    struct Grammar {
        next: u32,
        events: Vec<InnerNativeRequest<'static>>,
    }
    impl Grammar {
        fn proof(&mut self, count: usize) -> u32 {
            let kind = self.next;
            self.next += 1;
            self.events.push(InnerNativeRequest::Proof { kind, count });
            kind
        }
        fn draw(&mut self, operation: Operation, count: usize) -> InnerDraw {
            let kind = self.next;
            self.next += 1;
            let draw = InnerDraw {
                site: u64::from(kind),
                result_kind: kind,
            };
            self.events.push(InnerNativeRequest::Draw {
                site: draw.site,
                result_kind: kind,
                operation,
                count,
            });
            draw
        }
        fn rounds(&mut self, mu: usize, degree: usize) -> Vec<RoundSite<u32, InnerDraw>> {
            (0..mu)
                .map(|_| RoundSite {
                    coefficients: self.proof(degree + 1),
                    challenge: self.draw(Operation::RawScalar, 1),
                })
                .collect()
        }
    }
    fn fixture(
        mu: usize,
        zero: bool,
        binding: &[u8],
    ) -> (
        SourceDomainSites<u32, InnerDraw>,
        inner::Transcript<Registry>,
        Rc<Log>,
    ) {
        let mut grammar = Grammar {
            next: 3,
            events: vec![InnerNativeRequest::Public { kind: 2, count: 1 }],
        };
        let sites = SourceDomainSites::Balanced(BalancedSourceDomainSites {
            binding: 2,
            tau0: grammar.draw(Operation::RawVector, mu),
            first: grammar.rounds(mu, 5),
            quarters: grammar.proof(4),
            xi: grammar.draw(Operation::CoefficientVector, 3),
            second: grammar.rounds(mu, 5),
            virtual_value: grammar.proof(1),
            eta_zero: zero.then(|| grammar.draw(Operation::ScalarCoefficient, 1)),
            realization: grammar.rounds(mu, 3),
            value: grammar.proof(1),
        });
        let log = Rc::new(Log::default());
        let compiler = inner::Transcript::new(
            Registry {
                expected: grammar.events,
                binding: binding.to_vec(),
                log: log.clone(),
            },
            inner::Limits {
                transcript_bytes: 65_536,
                record_bytes: 4096,
                records: 128,
                sites: 32,
                blocks_per_site: 100,
                component_calls: 1024,
                read_bits: 1_000_000,
            },
            1,
            b"fresh-range-component/v2",
        )
        .unwrap();
        (sites, compiler, log)
    }
    fn source(zero: bool) -> (SourceDomainPlan, Vec<K>) {
        let view = RawSourceViewPlan::new(2, 3, 1, 1, 3)
            .unwrap()
            .with_permutation((0..18).rev().collect())
            .unwrap();
        let mut source: Vec<K> = (0..18)
            .map(|i| K::from_i64([0, -1, -16, 15][i % 4]))
            .collect();
        source[7] = K::zero();
        let spans = if zero {
            std::iter::once(7..8).collect()
        } else {
            vec![]
        };
        (
            SourceDomainPlan::new(
                view,
                SourceAlphabet::BalancedBase32,
                CoefficientSupport::new(18, spans).unwrap(),
            )
            .unwrap(),
            source,
        )
    }
    fn binding(plan: &SourceDomainPlan, id: u32) -> Vec<u8> {
        let installed = FullRingFreshSourcePlan::new(
            plan.clone(),
            vec![],
            1,
            b"fixture-owner/occurrence-7".to_vec(),
        )
        .unwrap();
        full_ring_fresh_source_binding(&installed, akita_types::FullRingSourceId(id), &[K::one()])
            .unwrap()
    }
    #[test]
    fn typed_source_range_roundtrip_counts_and_original_raw_opening() {
        for zero in [false, true] {
            let (plan, source) = source(zero);
            let binding = binding(&plan, 7);
            let (sites, mut producer, sent) = fixture(5, zero, &binding);
            let mut proof = Vec::new();
            let (point, value) = akita_prover::prove_full_ring_source_domain(
                &mut producer,
                &sites,
                &binding,
                &source,
                &plan,
                &mut proof,
            )
            .unwrap();
            let ledger = plan.range_ledger().unwrap();
            assert_eq!(sent.draws.get(), ledger.literal_draws);
            assert_eq!(sent.elements.get(), ledger.proof_elements);
            assert_eq!(proof.len(), 32 * (16 * 5 + 6));
            NativeStrongTranscript::finish(&mut producer).unwrap();
            let (sites, mut verifier, received) = fixture(5, zero, &binding);
            let mut remaining = proof.as_slice();
            let pending = crate::verify_full_ring_source_domain(
                &mut verifier,
                &sites,
                &binding,
                &plan,
                &mut remaining,
            )
            .unwrap();
            assert!(remaining.is_empty());
            NativeStrongTranscript::finish(&mut verifier).unwrap();
            assert_eq!(pending.point(), point);
            assert_eq!(pending.value(), value);
            assert_eq!(*sent.history.borrow(), *received.history.borrow());
            let expected = (0..32).fold(E::zero(), |sum, index| {
                let (p, digit, j) = (index / 8, (index / 2) % 4, index % 2);
                let w = if p == 3 || digit == 3 {
                    K::zero()
                } else {
                    source[17 - (p * 6 + digit * 2 + j)]
                };
                sum + akita_algebra::offset_eq::eq_eval_at_index(&point, index).mul_base(w)
            });
            assert_eq!(value, expected);
        }
    }
    #[test]
    fn typed_source_range_tamper_and_source_substitution_poison_the_same_compiler() {
        let (plan, source) = source(true);
        let header = binding(&plan, 7);
        let (sites, mut producer, _) = fixture(5, true, &header);
        let mut proof = Vec::new();
        akita_prover::prove_full_ring_source_domain(
            &mut producer,
            &sites,
            &header,
            &source,
            &plan,
            &mut proof,
        )
        .unwrap();
        // Full high coefficient, quarter, virtual frontier, raw endpoint.
        for element in [5, 6 * 5, 12 * 5 + 4, 16 * 5 + 5] {
            let mut tampered = proof.clone();
            let start = element * 32;
            let value = decode_e(&tampered[start..start + 32]).unwrap() + E::one();
            tampered[start..start + 32].copy_from_slice(&encode_e(&value));
            let (sites, mut verifier, _) = fixture(5, true, &header);
            assert!(crate::verify_full_ring_source_domain(
                &mut verifier,
                &sites,
                &header,
                &plan,
                &mut tampered.as_slice(),
            )
            .is_err());
            assert!(NativeStrongTranscript::finish(&mut verifier).is_err());
            assert!(verifier.admission().is_err());
        }
        for malformed in [vec![255; proof.len()], proof[..proof.len() - 1].to_vec()] {
            let (sites, mut verifier, _) = fixture(5, true, &header);
            assert!(crate::verify_full_ring_source_domain(
                &mut verifier,
                &sites,
                &header,
                &plan,
                &mut malformed.as_slice(),
            )
            .is_err());
            assert!(verifier.admission().is_err());
        }
        let other_source = binding(&plan, 8);
        let (sites, mut verifier, _) = fixture(5, true, &other_source);
        assert!(crate::verify_full_ring_source_domain(
            &mut verifier,
            &sites,
            &other_source,
            &plan,
            &mut proof.as_slice(),
        )
        .is_err());
        assert!(verifier.admission().is_err());
    }
    #[test]
    fn typed_source_range_rejects_wrong_grammar_before_any_draw() {
        for zero in [false, true] {
            let (plan, source) = source(zero);
            let header = binding(&plan, 7);
            for corruption in 0..5 {
                let (mut sites, mut producer, log) = fixture(5, zero, &header);
                if let SourceDomainSites::Balanced(balanced) = &mut sites {
                    match corruption {
                        0 => {
                            balanced.eta_zero = if zero {
                                None
                            } else {
                                Some(InnerDraw {
                                    site: 999,
                                    result_kind: 999,
                                })
                            }
                        }
                        1 => {
                            balanced.first.pop();
                        }
                        2 => {
                            balanced.second.pop();
                        }
                        3 => {
                            balanced.realization.pop();
                        }
                        _ => {
                            sites = SourceDomainSites::Boolean(BooleanSourceDomainSites {
                                binding: 2,
                                anchor: InnerDraw {
                                    site: 3,
                                    result_kind: 3,
                                },
                                eta_zero: InnerDraw {
                                    site: 4,
                                    result_kind: 4,
                                },
                                rounds: (0..5)
                                    .map(|_| RoundSite {
                                        coefficients: 5,
                                        challenge: InnerDraw {
                                            site: 6,
                                            result_kind: 6,
                                        },
                                    })
                                    .collect(),
                                value: 7,
                            })
                        }
                    }
                }
                assert!(akita_prover::prove_full_ring_source_domain(
                    &mut producer,
                    &sites,
                    &header,
                    &source,
                    &plan,
                    &mut Vec::new(),
                )
                .is_err());
                assert_eq!(log.completed.get(), 0);
                assert!(producer.admission().is_err());
                let (_, mut verifier, log) = fixture(5, zero, &header);
                assert!(crate::verify_full_ring_source_domain(
                    &mut verifier,
                    &sites,
                    &header,
                    &plan,
                    &mut [].as_slice(),
                )
                .is_err());
                assert_eq!(log.completed.get(), 0);
                assert!(verifier.admission().is_err());
            }
        }
    }

    #[test]
    fn typed_source_range_preserves_site_order_and_rejects_padded_recommit_shape() {
        let (plan, source) = source(false);
        let header = binding(&plan, 7);
        for reuse in [false, true] {
            let (mut sites, mut producer, _) = fixture(5, false, &header);
            if let SourceDomainSites::Balanced(sites) = &mut sites {
                if reuse {
                    sites.first[1].challenge = sites.first[0].challenge;
                } else {
                    sites.first.swap(0, 1);
                }
            }
            assert!(akita_prover::prove_full_ring_source_domain(
                &mut producer,
                &sites,
                &header,
                &source,
                &plan,
                &mut Vec::new(),
            )
            .is_err());
            assert!(producer.admission().is_err());
        }
        let (sites, mut producer, log) = fixture(5, false, &header);
        assert!(akita_prover::prove_full_ring_source_domain(
            &mut producer,
            &sites,
            &header,
            &vec![K::zero(); 32],
            &plan,
            &mut Vec::new(),
        )
        .is_err());
        assert_eq!(log.completed.get(), 0);
        assert!(producer.admission().is_err());
        let (_, mut unfinished, _) = fixture(5, false, &header);
        assert!(NativeStrongTranscript::finish(&mut unfinished).is_err());
    }
}
