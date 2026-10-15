# Original raw RAM / ordinary Akita relation bridge

## Status and scope

This is an independent, read-only boundary review of Candidate1 (`AKITA_RELATION_ACCUMULATION_PROTOCOL.md`) and the current local Akita/Jolt implementation. No code, branch, commit, build, test, lint, formatter, or control was run or changed. The known old `akita_e2e.rs` caller-signature compile defect was not rerun. This note is not crypto-cutover approval or numerical admission.

Paper quotations below come from the indexed local Akita Markdown, QMD document `#bfa1a6` (`qmd://lattice-papers/akita.md`, 6,659 lines). Definitions 10.13/10.14, Corollary 10.17, and Theorems 10.31/10.33 are used conditionally on authenticating that text or independently proving the statements. The official PDF body has not been independently authenticated in this work. No non-main refs were needed or surveyed.

## 1. Answer to the fundamental gate

**Ordinary Akita does not prove that its extracted original RAM planes belong to `[-16,15]`, nor that extracted physical-padding planes are zero. It does prove the original algebraic A/B/F decommitment and the decoded full-grid point claims, subject to its complete schedule/extraction/ROM prerequisites.**

For the selected original RAM65536 profile, there is nevertheless no demonstrated *application need* for a raw-plane predicate beyond that algebraic decommitment. The continued Jolt frontend consumes the decoded K-valued memory polynomial, not individual signed planes. Its initialization, transition, final-output, and adjacency obligations must all remain, with their actual randomized soundness price. The selected RAM physical coefficient domain is exactly its logical domain, so physical RAM padding is empty. The six additional raw digit slots are virtual zeros defined by the immutable view, not stored coefficients requiring a proof.

Therefore the decoder kernel is **not by itself a reason to require a raw range prefix** when the selected application relation is explicitly the original algebraic Akita source relation plus the existing decoded-memory frontend relation. It is a decisive reason not to transfer, or claim coverage of, a raw range/zero statement *if that stronger statement is selected*. The old native accumulation implementation does select a strictly additional `BalancedBase32` raw-source predicate. That predicate cannot be called proved by an ordinary PCS, and cannot be silently inherited and then omitted.

This distinction is conditional design guidance, not a proof that the whole Candidate1/Jolt composition is admitted. A representation-parametric frontend theorem for arbitrary extracted decoded K-memory, source causality/coalescence, actual field/basis support, and the 256-source/error-budget admission remain gates.

## 2. Four meanings of “original source” that must not be conflated

### (a) Retained honest private ownership

`akita/crates/akita-cpu-backend/src/sources/packed_ram.rs:21-118` defines `PackedRamSource`, a single logical polynomial backed by immutable signed planes in `[block][position][digit][coefficient]` order. All 26 planes use full balanced base32, including the last plane. Legal noncanonical paths inside the logical domain are retained verbatim. The six unused slots of a 32-slot raw digit axis are not stored.

Exact constructor: `PackedRamSource::from_signed_digits(num_vars: usize, plan: CommitInnerPlan, digits: Vec<i8>) -> Result<Self, AkitaError>` (`packed_ram.rs:57-113`). It requires d=128, depth26, base32, positive blocks/rank, power-of-two positions, complete vector length, sufficient physical extent, and actual zero digits after the logical domain. `PackedSignedDigits::from_i8_digits(..., 5)` and `validate_bounds` enforce `[-16,15]` (`sources/packed_digits/mod.rs:97-100,707-724`). These are honest producer/construction checks, not a remotely verified statement about an adversarially extracted witness.

`CommittedSourceClass::OriginalBalancedDigitPlanes` explicitly charges every plane at the full alphabet. `CommittedSourceContract::try_new` requires a full-field declaration; `source_norms` charges coefficient maximum16 and aggregate energy `26*logical_len*16^2`, rather than inferring a last-plane bound from the decoded coefficient (`akita-params/src/sis/committed_source.rs:51-58,134-143,184-207,309-319`). `ensure_sources_fit_accepted_interval` checks intrinsic original-plane geometry against that immutable declaration and does not substitute decoded coefficient reach (`akita-cpu-backend/src/commitment/api.rs:366-405`). Keep this producer contract unchanged even if the application decommitment relation is the weaker algebraic relation.

`commit_compact_ram(setup: &CompactInnerTraceProverSetup, contract: Arc<CompactRamContract>, source: PackedRamSource) -> Result<(CompactRamEndpoint, AkitaProverHint<Prime128OffsetA7F7Ext2>), OpeningsError>` imports and commits this original source once (`jolt/crates/jolt-akita/src/compact_ram.rs:148-173`). It does not decode/redecompose RAM or create 26 logical source polynomials.

### (b) Extracted algebraic source

Indexed Definition10.13, lines3709-3729, gives these commitment-side predicates:

1. Complete shape fixes ordered source-vector layouts/decoder, block geometry, matrix views, B slice maps, digit alphabets, and active F compression path.
2. For every block/component, `f* = Dec_shape(s)` and `A*s = G_b1*t_hat`.
3. The prescribed slice map places the inner-image digits in the complete B inputs; B and the active F-chain end at the exact public payload.
4. Inner-image and compression-chain digits lie in their certified alphabets.

It explicitly excludes opening-local D/H, points, partial evaluations, and values from source identity. It imposes no zero condition on algebraic-padding cells, does not require the source s to be the canonical gadget decomposition, and does not require membership of f* in the image of honest padding.

Indexed Definition10.14, lines3747-3773, strengthens a weak opening with an invertible challenge difference rho, bounds on rho and on the product `r=rho*s`, and opening-local rows. It does **not** replace those product bounds with an original signed-plane membership theorem. Remark10.32, lines4195-4197, explicitly says division by a unit need not preserve norm and that an additional source-shortness/canonicality argument would be required.

### (c) Decoded memory semantics

The application sees `f_C(a) = sum_(ell=0)^25 32^ell * s_C[a,ell]` in K, with all 26 original planes participating. This is a full-field memory polynomial, not a proof that the plane encoding is canonical, a 64-bit cell range proof, or a unique integer representation.

The actual continued frontend is representation-parametric at this interface: it consumes field values/ordinary endpoint claims, never `PackedRamSource`, `BalancedBase32`, `OriginalBalancedDigitPlanes`, or an original raw coefficient reader. A search of `jolt-verifier/src/stages`, `src/segment`, and `src/segment.rs` found no such raw-plane dependency. This syntactic fact establishes the interface separation, not the complete VM soundness theorem.

### (d) Additional raw source predicates

A theorem/application that explicitly demands `forall a,ell: s[a,ell] in [-16,15]`, source canonicality, or physical plane zeros selects a strictly stronger relation. Those predicates must be directly checked or proved on the same original A/B/F source, not inferred from decoded values or honest Rust constructors. The old native accumulation code does select the first predicate. Its role and theorem are distinct from Candidate1's one ordinary final decider.

## 3. Mandatory predicate inventory and coverage

| Predicate | Mandatory where? | Ordinary full-PCS coverage |
|---|---|---|
| Exact original field, complete profile/shape, matrix views, B slicing/F depth, full payload widths | Original PCS source relation | Yes, conditional indexed Def10.13 / Thm10.31/10.33; implementation verifies exact frozen profile and admitted schedule |
| A consistency, decoded `f*=Dec(s)`, complete B/F commitment path | Original algebraic decommitment | Yes, same conditional theorem |
| Certified carried inner-image and F-chain alphabets | Original PCS collision inventory | Yes; use verifier-certified scanning alphabets/cross-profile diameters, not an assumed honest gadget alphabet |
| Unit challenge differences and verifier-enforced bounded response products; Euclidean response energy when selected | Original extraction/collision proof | Yes, conditional complete accepting tree and admitted route; these are NOT an original source-shortness proof |
| Same A/B/F source across repeated original commitment occurrences | Batch source coherence | Yes, or a priced A/B/F collision under Cor10.17; requires identical complete shape and source-comparison class |
| Ordinary decoded point values, all declared polynomial components | Final batch PCS relation | Yes, conditional complete root + suffix + terminal, Def10.13 batch eq202 / Thm10.31 |
| Original RAM signed-plane ownership, all26 planes, honest producer alphabet | Honest construction/completeness/response sizing | Preserved by constructor, producer contract, native owner, projection, and kernels; NOT adversarial extracted-source membership |
| Extracted original RAM plane membership `[-16,15]` | Old native `BalancedBase32` plan; any separately selected stronger raw relation | NO, not supplied by ordinary algebraic PCS |
| Canonical source encoding / canonical Commit state | Not required by chosen algebraic relation; would be a stronger relation | NO; explicitly not claimed by Remark10.32 |
| Physical RAM A-padding zeros | Constructor for profiles with physical extent greater than logical extent | NO generic extracted-source guarantee; selected RAM65536 has no such cells |
| Six virtual raw digit slots are zero | Immutable raw-view definition | Structural: they have no physical address; not a cryptographic raw-source zero predicate |
| First RAM initialization, each decoded memory transition, last public output relation | Jolt application | Frontend algebraic protocols plus true PCS leaves; NOT PCS alone; frontend error/causality theorem required |
| Exact incoming/outgoing original endpoint identity, VM/program/I/O/counter linkage | Continued application | Direct context checks + transcript binding; same-source extraction needed for cryptographic interpretation |
| Logical memory word/operation constraints implied by the remaining Jolt relations | Full Jolt VM theorem | Must be proved through that frontend; signed-plane range alone is not a u64 word-range proof |
| Main56 Boolean/one-hot/used-slot/inactive physical checks | Main application source | Separate coverage gate; none is discharged by eliminating an unnecessary RAM raw prefix |

Code coverage: `AkitaVerifier::batched_verify(&self, proof: &[u8], session: &[u8], statement: GroupBatchStatement<'_, Cfg::ExtField, Cfg::Field>, basis: BasisMode) -> Result<(), AkitaError>` (`akita-verifier/src/fold/verify.rs:38-156`) validates complete profile/payload-chain lengths, exact selected row/opening layout, root descriptors, setup admission and ring dimensions, then `verify_root`, `verify_suffix`, and `grinding.finish`. `PendingNativeProver::finish` likewise runs every remaining fold and the actual terminal (`akita-prover/src/protocol/prove/root.rs:212-239`). No implemented Rust knowledge extractor was established; the extractor claims here are the conditional indexed theorem, not an observed runtime extraction.

Ordinary digit-range work is run on the committed successor witness and its relation/range-image plan (`akita-prover/src/protocol/prove/fold/mod.rs:617-700`; `fold/sumcheck.rs:170-203`). That verifies the predicates/norm envelope needed by the source-projection argument. It must not be described as directly range-checking every original RAM plane.

## 4. Selected RAM geometry: no hidden padding transfer

`CompactInner::root_inputs() -> Result<(ScheduleLookupKey, [CommittedSourceContract;3]), AkitaError>` (`akita-config/src/full_ring.rs:79-110`) freezes RAM at:

- logical variables16, one logical polynomial;
- 16 live blocks, 32 positions/block, d_A=128, n_A=3;
- 26 base32 original planes; full-field original-plane producer declaration;
- one outer slice, B rank1, d_B=128;
- original A radius8,126,340 and B radius31.

Thus `16*32*128 = 65,536 = 2^16`. Every physical RAM coefficient position is logical. The physical signed-plane extent is `65,536*26 = 1,703,936`; the padded raw Boolean domain is `65,536*32 = 2^21`. The raw coordinate order is coefficient7, digit5, position5, block4, whereas the ordinary coefficient domain is coefficient7, position5, block4.

`RawSourceViewPlan::raw_to_physical` returns None for ell>=26 and the reader returns structural zero there (`akita-types/src/full_ring/mod.rs:66-113,254-292`; `full_ring/strong.rs:47-69`). This does not rewrite any physical signed plane. The old RAM domain plan explicitly has `CoefficientSupport::new(ram_view.physical_len(), Vec::new())`, so even that old selected plan requires no physical RAM-zero support (`jolt-verifier/src/receipt/native.rs:108-124`). Do not generalize this fact to Main physical inactive cells, a larger future RAM geometry, or virtual-polynomial selectors.

## 5. Exact decoded RAM frontend cut and consumers

The endpoint identity contract is checked public data, not private source authority. `CompactRamContract::from_setup` fixes original sourceId2, exact RAM profile/producer contract, complete native instance and the base32 positional decoder. `CompactRamEndpoint` equality includes commitment plus this full contract (`jolt-akita/src/compact_ram.rs:22-129`). SourceId2 identifies an installed source class, not a particular boundary source.

`SegmentContext::check_predecessor(&self, previous: &Self) -> Result<(), VerifierError>` compares exact `incoming_ram == previous.outgoing_ram` along with run/preprocessing/publicI/O/counter/VM state identities (`jolt-verifier/src/segment.rs:305-323`). Equality of decoded K-values alone is insufficient for this boundary identity.

`prepare_continued_instance` validates context/predecessor, absorbs the original endpoint commitments before frontend challenges, requires transparent K-only and CycleMajor trace, replays stages, and returns pending stage8 plus the two original RAM endpoint groups (`jolt-verifier/src/segment/frontend.rs:190-251,288-298,351-381`). It does not complete PCS.

Exact ordinary leaf producer: `SegmentContext::endpoint_groups(&self, address: &[F], incoming: F, outgoing: F) -> Result<[GroupOpeningClaim<F,H::Commitment>;2], VerifierError>` (`segment.rs:325-355`). Each singleton endpoint has the exact16-coordinate point in this selected profile. The generic branch prepends zero physical coordinates only when a larger endpoint physical domain was declared; that branch is not exercised by this exact RAM shape.

Consumption:

1. `RamEndpointEvaluations::validate` requires the same stage2 RAM read/write address; the last segment additionally aliases the original final-output value/point to the outgoing endpoint, while non-last segments must not carry that final-output claim (`stages/stage4/ram_val_check.rs:631-665`).
2. Endpoint address and both field values are bound before the existing RAM gamma (`...:667-704`; `stage4/verify.rs:184-226`).
3. First-segment initialization is independently reconstructed from public program/IO/advice and must equal the incoming field evaluation; later segments clear initialization consumers and use the incoming endpoint instead (`stage4/verify.rs:189-206,248-282`; `segment/frontend.rs:249-251`). All present original program/advice opening leaves must remain covered.
4. `RamValCheck::with_segment(trace_dimensions: TraceDimensions, ram_log_k: usize, incoming: F, initialization: Option<RamValCheckInit<F>>) -> Self` substitutes the incoming decoded field value, retaining the existing output expression (`stage4/ram_val_check.rs:157-179`).
5. The symbolic input is `val + gamma*val_final - (1+gamma)*init`; the output is `LtCyclePlusGamma*inc*ra`, with no original raw digit variable (`jolt-claims/src/protocols/jolt/relations/ram/val_check.rs:95-167`). The frontend sumcheck consumes these expressions; the endpoint leaves eventually need the final PCS.

These are algebraic/probabilistic checks. One accepted frontend transcript with true leaves is not a deterministic complete-execution certificate. The representation-independent implication must be stated with the actual fixed-source frontend error and then composed causally with extraction, as Candidate1 now requires.

## 6. Decoder proof, counterexamples, and what they do NOT prove

For each address a, let `D(s)(a)=sum_(ell=0)^25 32^ell*s(a,ell)` in K.

**Logical query transport.** For an ordinary MLE query at u,

`MLE(D(s))(u) = sum_(a,ell) eq(u,a)*32^ell*s(a,ell)`.

This identity holds for arbitrary algebraic field-valued planes, not only balanced or canonical planes. It justifies the actual positional decoder adjoint; it does not justify an inverse decoder. `DecoderPlan::visit_adjoint<E: ExtField<K>>(&self, point: &[E], visit: impl FnMut(usize,E)) -> Result<(),AkitaError>` emits precisely these weights (`akita-types/src/full_ring/query.rs:426-455`). Its positional affine constant is zero (`...:407-419`).

**Kernel counterexample to inferring range.** At one logical address, use planes `s_0=32, s_1=-1` and all other planes zero. Then D(s)=0, although s_0 is outside `[-16,15]`. The all-zero plane vector has the same decoded polynomial and does satisfy range. Therefore decoded openings cannot establish arbitrary raw range predicates. This is a counterexample to decoder-based implication, NOT a constructed same-commitment attack: A/B/F binding may distinguish their commitments.

**Kernel counterexample within legal original representations.** The retained fixture `jolt-prover/tests/support/compact_segments.rs:315-341` encodes the modulus q at the last logical zero address, using all26 balanced planes and a nonzero plane25. It decodes to zero in K just as the all-zero path does. This is legal and intentionally preserved. Any rewrite to canonical K digits, dropping the high planes, or comparing source identity by decoded values destroys the original source representation.

**Linear raw-query criterion.** A raw linear query L can be derived from decoded data exactly when L annihilates ker(D), equivalently when L belongs to the decoder-adjoint image. A generic raw MLE point query on the21-dimensional raw table does not satisfy this restriction: its digit-axis equality weights are not generally the fixed powers32^ell. No ordinary16-variable endpoint can authenticate that generic raw value merely by adding a descriptor or an extra unused coordinate.

## 7. Why original PCS collision security does not itself demand raw plane range

Indexed Lemma10.15/Cor10.17, lines3775-3825, compare two weak explanations using unit factors rho,rho' and bounded products r=rho*s, r'=rho'*s'. Once carried source-path digits agree, the A-kernel vector is

`z_A = rho'*r - rho*r' = rho*rho'*(s-s')`.

It is nonzero if the source blocks differ and satisfies

`||z_A||_infinity <= ||rho'||_1*||r||_infinity + ||rho||_1*||r'||_infinity`.

Thus the ordinary collision argument prices verified weak-product envelopes, not `||s||<=16`. B/F collisions use certified source-path alphabet diameters. Remark10.16 specifically warns against using honest narrower alphabets when the scanning profile certifies a wider alphabet. Theorem10.31 root extraction supplies Def10.13 and the decoded full-grid claims, explicitly without a padding-image claim. Theorem10.33 invokes the authenticated complete challenge grammar and its global query/extractor prices, not an honest-response distribution.

Raw signed-plane bounds remain important for honest completeness and response sizing. Removing an unnecessary standalone application raw range certificate does not permit changing the original producer declaration, repricing plane25 as a canonical small high digit, dropping original planes, or weakening response checks. It also does not admit a missing256-source catalog row or a new augmented FS grammar.

## 8. Existing source-preserving ordinary API

`CpuBackend<F,F>::import_full_ring_commitment<EOrigin>(&self, foreign: &CommitmentHandle<F,EOrigin>) -> Result<CommitmentHandle<F,F>,AkitaError>`, with `EOrigin: ExtField<F>+'static`, authenticates the original native owner/state, shared setup prefix, exact descriptor/sourceId/raw view/profile/producer contract, and re-executed payload and retained commitment material (`akita-cpu-backend/src/opaque/ordinary_projection.rs:145-202`). It retains the original source Arc and derives only ordinary quotient-lift material. No source bytes are manufactured from a public payload.

Its capability is deliberately only ordinary K/K native k1h1 packing at d128 (`ordinary_projection.rs:15-56`). `PackedRamSource` ordinary partials Horner-decode all26 original planes on demand, while fold kernels use the same signed planes (`sources/packed_ram.rs:152-172,388-437`). It does not grant arbitrary E2/raw-plane queries or the old legacy EOR bundle. Projection must be imported once and cloned for aliases; do not mint separate private authorities for the same repeated boundary and then assume ID equality.

Authenticated raw reader: `CpuBackend<F,E>::full_ring_source_reader<'a>(&self, source: &'a CommitmentHandle<F,E>) -> Result<CpuFullRingSourceReader<'a,F,E>,AkitaError>` (`opaque/full_ring.rs:703-764`). This borrows the exact original K planes after owner and installed source/view/profile checks. It is an arithmetic/ownership interface, not a public proof.

## 9. If the stronger raw relation is selected: real source-preserving options

### 9.1 Existing non-complete raw domain reduction

Exact prover API (`akita-prover/src/protocol/prove/full_ring_strong.rs:106-201`):

`prove_full_ring_source_domain<T,R,D>(transcript: &mut T, sites: &SourceDomainSites<R,D>, binding: &[u8], source: &dyn FullRingCoefficientSource<K>, plan: &SourceDomainPlan, proof: &mut Vec<u8>) -> Result<(Vec<E>,E),AkitaError>`, where `T: NativeStrongTranscript<R,D>`.

Exact verifier API (`akita-verifier/src/full_ring_strong.rs:95-174`):

`verify_full_ring_source_domain<T,R,D>(transcript: &mut T, sites: &SourceDomainSites<R,D>, binding: &[u8], plan: &SourceDomainPlan, proof: &mut &[u8]) -> Result<FullRingPendingEvaluation<E>,AkitaError>`, with the same transcript bound.

Here K/E are the native aliases, with the genuine NR5 quadratic E2 branch, not a generic supported ordinary evaluation field. Balanced range is a (5,5,3) reduction on the original raw source with optional physical-zero support. The return value is the actual raw source MLE value, not the transformed virtual range value. It is **pending**, never acceptance.

The existing native consumer correctly inserts this returned raw endpoint in the same handle's normalization list and then passes the normalized raw claim to its full-ring A/B/F source reduction (`akita-prover/src/protocol/prove/full_ring_accumulation.rs:215-275`). Candidate1's ordinary decoded PCS cannot replace that receiving dispatch: the returned raw functional is generally outside im(D*). Dropping this endpoint, treating reduction success as a proof, or normalizing it onto decoded16 data without another source-binding certificate is unsound.

A one-final **full-ring** all-original-source decider could close these endpoints without committing new plane polynomials or proving complete per-segment PCS proofs. That is a real source-preserving architectural option, but it changes Candidate1's final decider and needs an admitted all-source catalog, proven source comparison, compiler law, native field support, and augmented budget. Existing old H1610/E2 prototype admission is not certified here. There is no demonstrated plug-and-play ordinary EOR route for this original packed source.

### 9.2 Direct, non-complete decommitment certificate: correctness-first baseline

If raw membership must be retained while the final decider must remain ordinary, a mathematically concrete fallback is a **direct certificate of the original RAM decommitment**, not a new commitment and not a fake projected source:

1. Before relevant augmented challenges, seal a certificate containing the exact original signed planes s and the actual original inner-image digits and active B/F path state, under the unchanged public C/profile/shape.
2. The verifier directly checks all required source plane bounds/physical zeros, `A*s=G*t_hat`, complete B slicing and active F equations, certified path alphabets and the exact terminal payload. No canonical re-decomposition of s is done. Supplying actual t/F state avoids falsely assuming any alternate decommitment is a canonical Commit state.
3. The prover streams this data from the retained native source and retained material, never constructs a private source from commitment bytes. The verifier may compute D(s) and its needed logical queries directly. The final ordinary all-source endpoint batch can remain, with RAM extraction compared to this certified original source.
4. For that comparison, give the certificate a reference weak profile with `rho'=1`, `r'=s`, so its infinity source bound is16 (and energy at most `26*65536*16^2`). If an ordinary extraction has r=rho*s*, disagreement yields `z_A=r-rho*s`, bounded by `beta + kappa_1*16`, plus B/F cross-alphabet collisions. These mixed reference envelopes MUST fit or be separately priced against the original fixed A/B/F matrix instances; do not assume the setup-reference theorem automatically installs arbitrary application references.
5. Include certificate transport, coalescence and any required challenge/query costs in the complete classical-ROM theorem. A direct check has no additional algebraic challenge error, but comparing it to an extracted PCS witness still requires the stated lattice/source-comparison argument.

This is fully source-preserving and can be extremely slow/large: one signed-byte listing is1,703,936 bytes per distinct RAM boundary before path-state overhead. At256 segments, up to257 distinct boundaries means437,911,552 signed-plane bytes (about417.6MiB), before A-image/F certificate data. It reveals the RAM witness; it is acceptable only for an explicitly transparent/non-private experiment. The current continued frontend rejects ZK, but this is not permission to assert any user privacy requirement away. Retained path-state export and the mixed-envelope certificate verifier are new narrow interfaces; no such finished certificate implementation is claimed.

This baseline demonstrates that a mandatory raw predicate is not inherently impossible without changing source commitments. It does NOT make it a decoded query or a bounded-state accumulator. A succinct replacement would require a genuine same-source raw-functional binding proof/interface, not the positional decoder alone.

## 10. Narrow viable tasks and unresolved gates

For Candidate1 with the algebraic/logical RAM relation:

1. Name the selected source relation precisely as Def10.13 algebraic A/B/F decommitment; distinguish the unchanged honest `OriginalBalancedDigitPlanes` producer declaration from a claimed extracted-source raw alphabet predicate. Explicitly state that the old native raw domain relation is not inherited.
2. Prove the continued frontend memory theorem over arbitrary extracted decoded K-memory, preserving first initialization/program/advice coverage, every transition/last-output leaf, exact endpoint aliases, program/IO/VM/counters and Main constraints. Provide the actual fixed-source error and prefix-causal coalescing argument, not deterministic single-transcript implication.
3. Reuse a same-retained-source logical reader/decoder and native ordinary projection; preserve all26 planes and full original profile. No raw range endpoint is emitted in this choice, so there is no unmatched raw dispatch to suppress.
4. Independently admit the actual final batch catalog and augmented challenge grammar for1/2/3/256, including the fitting evaluation field/basis, sampling law, error budget and extractor/matrix comparison costs. Current K/K projection alone does not resolve field or256-arity admission.

If raw range is an explicit desired application statement, choose9.1 or9.2 (or prove another real same-source certificate), and retain every raw endpoint until its authenticating consumer closes it. Do not keep an obsolete raw emitter with an ordinary consumer that discards it. No complete per-segment PCS, new plane source commitments, canonical RAM rewrite, or proof from payload reconstruction is part of either option.

Unresolved: official paper-body provenance; full frontend representation-independent/causal theorem; final field/basis compatibility and catalog/FS-budget admission; any selected stronger raw certificate's binding and comparison prices; privacy acceptability of a direct certificate if chosen. No test or control result establishes these proof obligations.
