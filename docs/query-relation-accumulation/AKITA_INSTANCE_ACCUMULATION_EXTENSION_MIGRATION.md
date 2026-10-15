# D1 implementation boundary: K sources, E2 challenges and ordinary outer profiles

## 1. Scope and decision

This note specifies the source-level cutover required by the conditional [D1 model](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) and [bounded archive guest](AKITA_INSTANCE_ACCUMULATION_GUEST_SPEC.md). It is a design and inspected-source map, **not end-to-end implemented support, a generated profile, a concrete security certificate or a performance result**. The [independent and integrated review](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_REVIEW.md) admits implementation of the disclosed heuristic prototype. Component implementation evidence is recorded in §8.

Source baselines are Jolt `0791f79a55c3fadc3d327c78a72228696c338468` and Akita `83574331d4e51f8cce1d8689d05592fa4ef4c138`. Local inspected paths below identify the implementation seams; the quadratic accumulator and diagnostic-context findings were also checked at those immutable source pins by the source investigation. No new Git-state assertion follows.

**Field-choice gate reopened:** a nontrivial scalar extension is not an intrinsic
Akita or lattice-accumulation requirement. Akita §3.1 explicitly admits `E=K`,
and the original fp128 presets select it. The E2 requirements in this note are
conditional on the chosen experimental profile, not a user requirement.
Further E2-specific migration is paused pending a baseline-matched K-only
soundness/cost comparison. The conditional retention decisions in §3.4 do not
authorize that migration before this comparison.

The [field-policy comparison](AKITA_INSTANCE_ACCUMULATION_FIELD_POLICY_COMPARISON.md)
and [native-reuse revision](AKITA_INSTANCE_ACCUMULATION_NATIVE_REUSE_PROFILE.md)
now give a narrower candidate: K frontend, E2 accumulation checks, ordinary K/K
outer proof and original shared-prefix setup. The localized-field and setup
reductions passed conditional independent reviews. Exact-profile ordinary
control admission and integrated revision review remain gates. The all-E2
migration described below is not a requirement of that candidate.

D1's guest computes the fixed public inner transcript and verifies the native archive. Removing supplied answers, graph/quotient authentication, C_t and the graph-induced E4 requirement does **not** remove:

- original K-valued sources and their all-valid-source/weak-reference semantics;
- full-width E2 frontend challenges and E2-valued opening claims;
- complete inner strong checks, original endpoints and the genuine final decider;
- the original archive/static advice sources, deterministic guest adequacy and ordinary outer extraction;
- a complete ordinary outer PCS, including any internal setup-offloading recursion selected by its actual schedule.

The last item is PCS-internal recursion, **not** a return to completed per-segment proof aggregation. The inner accumulation cut stays before `PCS::prove_batch`; the one final outer proof must complete that operation and its full suffix.

## 2. Field and source interfaces

Let
\[
K=\mathbb F_q,\qquad q=2^{128}-2^{32}+22537,
\qquad E_2=K[U]/(U^2-5).
\]
The source's integer interpretation, digit decomposition, norm bounds and matrix arithmetic remain over K. An arbitrary E2 value is not an integer residue modulo q².

| Existing seam | Source-side contract | Challenge/result-side contract |
|---|---|---|
| PCS commitment | Original `MultilinearPoly<K>`, source plan, K payload and retained handle | No challenge-dependent replacement source |
| PCS opening | Same original commitment/handle and source geometry | Points, values, grouped claims and scalar messages in E2 |
| Frontend preparation | Borrowed `JoltWitnessPlane<K>` and source-derived integer/field values | Kernel state, interpolation, RLCs and sumchecks in E2 |
| Proof/preprocessing | Program/source commitments and source metadata | Stage messages/claims and transparent proof algebra in E2 |
| Transcript | Canonical public/source bytes and installed descriptors | Full E2 draws, actual framed site binding and fallible bounded samplers |
| Native carrier | Original S=K[Y]/(Y^128+1) source | Full E2[Y]/(Y^128+1), two S planes per partial |

Conceptually, the PCS separates `SourceField=K` and `EvalField=E2:ExtField<K>`. `commit`, batch commit and full-width commit consume K sources; `prove_batch` continues consuming the **retained original handles, with no replacement-polynomial argument**. Verification consumes matching original public commitments and E2 claims. These are contracts for changing the existing interfaces, not new parallel APIs to leave alongside the old path.

For an original Boolean table w over K,
\[
\widetilde w(r)=\sum_{x\in\{0,1\}^n}\iota(w_x)\operatorname{eq}_{E_2}(x,r).
\]
Its first binding obeys
\[
w'(u)=(1-r_0)\iota(w(0,u))+r_0\iota(w(1,u)).
\]
Compute this from borrowed K rows into the half-sized E2 working table; do not first allocate a full E2 copy of every source. Subsequent bindings operate on E2 state. Dense, one-hot, packed, shifted and precommitted implementations must satisfy this identity in their **actual** indexing convention. Preserve Jolt/Akita's paired evaluation-bit and point reversals, prefix padding, lane maps and aliases. This optimization does not authorize changing the committed source or skipping physical-hole checks.

The complete stage semantics and source-descent argument remain [FRONTEND_EXTENSION §§3–7](AKITA_INSTANCE_ACCUMULATION_FRONTEND_EXTENSION.md) and [FRONTEND_DESCENT](AKITA_INSTANCE_ACCUMULATION_FRONTEND_DESCENT.md). E-valued intermediates are deterministic algebraic state, not arbitrary new execution witnesses. Execution field-inline decoding remains K first if a separately admitted profile enables it; the selected T16 inner profile does not.

### Inspected coupling points

- [PCS traits and grouped claims](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-openings/src/schemes.rs), lines49–166 and404–440: one field currently serves sources, points and values.
- [Multilinear interfaces](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-poly/src/multilinear.rs), lines26–66 and source row/fold methods: same-field evaluation/binding must become mixed-source/result operations.
- [Proof](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/proof.rs), lines39–64; [verifier preprocessing](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/preprocessing.rs), lines217–248; [Akita prover](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/prover.rs), lines34–50: stages, PCS, witness and vector commitment are currently tied through `PCS::Field`.
- [Kernel backend](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-kernels/src/backend.rs), lines111–121, and [kernel](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-kernels/src/kernel.rs), lines80–128: witness preparation and E-result kernels need separate roles.
- [Akita scheme](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/scheme.rs), lines420–426; [adapters](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/adapters.rs), point reversal/native groups/bridge; [shape guard](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/shape_guard.rs), lines25–145: generalize evaluation types without changing K payload lengths or trusted shape checks.

Migrate the complete caller families together: generated claims/points, stage drivers, proof/serde bounds, witness-facing optimized kernels and carries, mixed evaluators, prefix reduction, adapters/native batching/custom source opening, transcript facades and registries. Source integer/address/shape operations do not migrate to E2. Avoid unrelated backend changes; a same-field backend is the K=E specialization of the separated roles, not an obsolete-interface shim.

## 3. Reusable arithmetic and real gaps

### 3.1 Quadratic field capabilities

[Jolt extension configuration](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-field/src/extension.rs) and [solinas extension arithmetic](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-field/src/solinas/ext.rs) already provide configured `FpExt2<F,C>`, conjugate/norm inversion, canonical coordinate bytes, serde, `ExtField` lifting, base scaling and Frobenius. Use an explicit nonresidue-5 configuration. The default `TwoNr` is invalid over this K, where 2 is square; a Rust `Field` bound does not prove irreducibility.

**Inventory correction:** [unreduced.rs:648–721](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-field/src/solinas/unreduced.rs) already implements `Unreduced`, `MulBaseUnreduced<K>` and `Fold` for `FpExt2<Fp128<P>,C>`. These immediate-reduction identity implementations are exact but are not deferred optimized extension accumulators. `Unreduced` is not `WithAccumulator`.

The selected `Prime128OffsetA7F7Ext2` now provides the `CanonicalEncoding`/`WithAccumulator` bundle required by [JoltField](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-field/src/algebra.rs), reusing configured quadratic arithmetic and three base accumulator lanes. Representative conversion succeeds only if nonconstant coordinates vanish. Canonical E2 decoding validates both K coordinates; modular decoding is not a uniform sampler. This closes the field-capability gap, not the native carrier or frontend migration. Neither the existing naive accumulator nor the new arithmetic checks establishes performance parity.

[Akita field serialization](../../crates/akita-serialization/src/field_impls.rs), lines75–125, supports configured quadratic types; [extension transcript atoms](../../crates/akita-transcript/src/proof_stream.rs), lines290–335, are coefficientwise. Reuse those semantics rather than inventing a second coefficient order.

### 3.2 Full-ring carrier is not subfield packing

Native genericity already separates the fields: [opening claims](../../crates/akita-types/src/opening_claims.rs) has `GroupBatchStatement<E,K>` and original K committed groups; [owned handles](../../crates/akita-cpu-backend/src/opaque/owned.rs) retain K/E types, source ownership and producer contracts. [CommitmentConfig](../../crates/akita-config/src/lib.rs) separates matrix/source field from scalar field and checks declared degree agreement.

The actual backend/verifier also requires [FpExtEncoding](../../crates/akita-types/src/field_reduction.rs), which describes a **specific ring-subfield basis**. It does not certify arbitrary nonresidue-5 coordinates. A marker implementation claiming that embedding without proof is unsound.

The selected inner carrier is the scalar extension of the full degree-128 ring, with **256 K coefficients per partial**, the same lattice folding unit and no extra original source plane. Implement its source/query/receiver maps explicitly. The [one-hot opening fold](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/trace_onehot/opening.rs) currently has only base K and cyclotomic-subfield weights; it needs the selected plane-aware output path. Existing extension-opening tensor helpers may be reused only where their maps equal the selected contract.

The independently reviewed [ordinary outer carrier](AKITA_INSTANCE_ACCUMULATION_OUTER_CARRIER.md) resolves this algorithm choice with `FullRingScalarExtension`, canonical K coefficient sources through terminal, per-handle responses and both complete partial planes. Its finite family has one grouped root, the complete Eq109 successor created as an A-only terminal source, and a genuine direct terminal; no setup offload or intervening full-commitment recursion is selected. It does not inherit the inner receiver schedule or claim compatibility with existing EOR/tensor rows.

**Implementation reuse requirement:** retain the existing setup/matrix code, `CommitmentExecutor`, original source/commitment handles, [fold/decomposition interfaces](../../crates/akita-cpu-backend/src/opaque/fold_kernels.rs), complete Z/E/T/F/H witness construction, compression-state retention, range/sumcheck storage, proof-scope leases, and the [actual A-only terminal producer](../../crates/akita-cpu-backend/src/opaque/recursive/commit.rs). Extend their typed algebra and lifecycle boundaries where required; do not construct a second proof pipeline. Reuse does not authorize the old subfield embedding, tensor packing, alpha-only relation check, gamma-power range mixing, or unbounded transcript sampling for D1. Those incompatible semantics require the selected full-ring, coefficient-row, range and bounded-transcript changes.

### 3.3 Refreshed upstream branch audit and attribution

The follow-up audit inspected 20 Jolt branch/base snapshots, five Jolt revisions
pinned by Akita, and 21 Akita snapshots after refreshing both remotes. Coverage
was targeted at field, PCS, transcript, source and kernel seams, not every line
of every historical branch. This audit does not establish that the same branch
comparison happened before the migration. No upstream tests were run by the
read-only audit.

The original Jolt base already contains real extension-field sumcheck
[prover/verifier fixtures](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-sumcheck/tests/extension_fields.rs),
including nonbase coefficients. Original Akita already has concrete fp32/E4 and
fp64/E2 configurations and
[PCS integration tests](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-pcs/tests/akita_small_field_e2e.rs).
The gap is not a missing extension engine or missing native K/E PCS. The
inspected Jolt frontend contracts still use one field for sources, points,
claims and transcript challenges, including
[refreshed main](https://github.com/a16z/jolt/blob/a104dc63aa11a19915301cd73650511ac72355e8/crates/jolt-openings/src/schemes.rs).

Additional upstream reuse candidates:

- Jolt `feat/fast-recursion` at `1af59c3894e37caa801863336d52620d2d7dd402`
  already has `dot_base`, `dot_base_rows` and `weighted_dot_base_rows` in
  [the mixed arithmetic interface](https://github.com/a16z/jolt/blob/1af59c3894e37caa801863336d52620d2d7dd402/crates/jolt-field/src/extension.rs).
- Jolt `refactor/transcript-unification` at
  `31103b01c5a70261dd92b192dbb9d138f9dda083` already has generic quadratic
  canonical decoding and [exact field sampling](https://github.com/a16z/jolt/blob/31103b01c5a70261dd92b192dbb9d138f9dda083/crates/jolt-transcript/src/state.rs).
  The latter samples extension coordinates but does not implement D1's bounded
  attempts, descriptor registry or finite acceptance contract.
- Jolt `quang/binary-kernel-primitives` at
  `4d1970f468777655cc7e21cd4c5e06e56ce8ea38` has a `WitnessPlane` associated-view
  seam. Its default frontend is still same-field; binary-field arithmetic is
  not interchangeable with the selected odd-prime extension.
- Akita `quang/labinius-pcs` at
  `a8d275661cb36340c3b1818136cf518fdf6fedad` has coefficient-table flattening and
  degree-two opening conversion. Those paths retain the ordinary
  `FpExtEncoding` geometry; they do not certify the selected full-ring map.

No concrete A7F7/NR5 full-ring frontend pipeline was found in the inspected
snapshots. NR5 is a small configuration of the original generic arithmetic.
Offset275 with NR2 already supplies valid alternative field arithmetic, but
changing K changes source values, setup identities, commitment parameters and
security/profile obligations. It is not an alias preserving the original
handles. The same missing frontend separation must still be addressed.

This audit supports the source/evaluation distinction and full-width sampling,
not every current capability split, transcript abstraction or optimized
accumulator. Reconcile these existing upstream implementations before further
broad refactoring; justify each additional mechanism by its separate protocol
or measured performance requirement.

### 3.4 Reconciliation decisions to review before further migration

Keep the configured upstream `FpExt2` engine. Keep the NR5 selection and only
the `CanonicalEncoding`/`WithAccumulator` adapters required by the pinned
`JoltField` contract. Upstream's newer `CanonicalDecode` expresses a different
trait interface; importing that whole field/transcript refactor is not required
to decode two canonical coefficients. Reuse the same coefficient order and
base decoder. Compare the deferred adapter against the existing
`NaiveAccumulator<E2>` before attributing a performance benefit to it.

Keep `SourceField`/`EvalField`, mixed MLE binding and borrowed K witness ownership:
these implement §2's source identity and avoid a full E copy. Existing mixed
dot/row helpers can replace equivalent local loops only after checking layout,
length, reduction and guest-dispatch semantics. The binary witness-view seam
does not by itself supply the original-source or continued-state contract.

The PCS capability split has a separate reason from extension arithmetic:
`CompactInnerTrace` commits only its admitted original Main producer and does
not implement arbitrary-polynomial commitment or a completed opening proof.
Retained source data must not imply either capability. Ordinary legacy openings
also require their legacy transcript recipe; a D1 transcript must not reach
that recipe through a nominally generic opening method. Review the trait split
against those actual consumers, not as a general redesign opportunity.

The emission-plan and fallibility changes implement the reviewed D1 transcript
contract, not new extension sumcheck arithmetic. Full-width draws, bounded
exhaustion, actual site/context binding, identity-based opening curation and
finite retained-compiler acceptance remain necessary even with upstream's exact
sampler. Existing claim/stage identities and sumcheck algebra remain the
implementation substrate. A missing production finite registry remains missing;
test registries and typed loans are not a substitute for the installed schedule.

These decisions retain the reviewed field and full-ring geometry. They do not
authorize a switch to a newer branch's field, setup distribution, subfield
encoding, security policy or transcript without a corresponding design review.

### 3.5 Why E2 was selected, and what has not been proved necessary

The [Akita paper](SOURCE_PROVENANCE.md#akita), read through qmd, separates three
objects in §§3.1–3.4: the cyclotomic commitment ring over K, short norm-controlled
folding challenges in a ring/subring, and scalar polynomial-testing challenges
in a field E. Lines 879 and 895–901 allow extension degree one. Original
`proof_optimized::fp128` uses `(Field, Field)`; fp64 uses a quadratic extension
and fp32 a quartic one. Neither choosing a lattice commitment nor choosing
accumulation alone forces `E != K`.

The E2 choice came from the selected sufficient statistical ledger.
For scalar charge sum c and classical oracle-query budget Q, Akita Corollary
10.34 bounds the strength certified by its conservative raw-ROM ledger by
`log2(|E|) - log2(c) - log2(Q+1)` when a `c/|E|` term is present.
Our compact example chose `Q+1 <= 2^32` and `c_E <= 2^40`, giving the scalar
allowance `2^72/q^2` with E2 instead of `2^72/q` with K. This comparison concerns
a chosen sufficient error allowance, not a demonstrated attack or a lower
bound on actual security. The full statement also contains ring challenges,
setup bias and runtime-priced MSIS reductions.

The [composed policy](AKITA_INSTANCE_ACCUMULATION_COMPOSED_POLICY.md) already
states that a tight end-to-end 128-bit reduction is not an extra user
requirement. Therefore failure of that illustrative budget with K does not
establish that this task needs E2. Compare the actual complete K-only
accumulation ledger with the matching recursive baseline and its disclosed
security policy first. A smaller ring challenge error cannot cancel a scalar
`c/q` term. Existing grinding is also not free credit: Akita Appendix E.5
distinguishes its classical work accounting from a QROM or complete system
security theorem.

Both independent reuse reviewers confirmed that the field/adaptor decisions
are conditional on E2, not a proof of its necessity. No K-only replacement
profile, complete security certificate or performance-parity result has yet
been established. Do not discard the existing changes or silently weaken the
policy merely because the field choice is reopened.

## 4. Transcript and finite sampler cutover

D1's concrete inner compiler is specified in [PROTOTYPE_MODEL §2](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md), not by the current native transcript defaults. It uses real framed A/S component inputs and keeps an appendable state distinct from finalized snapshots. The guest and native prover must implement the same component bit/byte order, canonical records, vector-stream semantics and challenge-history binding.

Two inspected facts prevent false reuse claims:

1. [Native ext_challenge](../../crates/akita-transcript/src/proof_stream.rs), lines596–619, samples one K coordinate per extension limb. **ProtocolSiteId records are diagnostic only**: prover/verifier context functions at662–686 log or do nothing. They do not absorb the site. Use the D1 S input and complete emission map for actual site binding; do not rely on the “context-bound” comment.
2. [Native sampling](../../crates/akita-transcript/src/proof_stream/sampling.rs), lines10–99, uses an unbounded rejection loop. `FIELD_SAMPLING_QUERY_LIMIT` is not a retry cap. Its Keccak retry-count absorption is part of that old implementation, not automatically part of D1.

D1's inner bounded profile instead uses [GLOBAL_MANIFEST §5](AKITA_INSTANCE_ACCUMULATION_GLOBAL_MANIFEST.md): at most two 128-bit candidates per K coordinate, two coordinates per E2 sample; abort on exhaustion. The lattice shell has 40 partial Fisher–Yates positions, at most256 candidate attempts each and 40 signs. No modulo/default-zero fallback, response-dependent filter or hidden unbounded retry is allowed. The complete emission descriptor fixes actual bit consumption, retry positions and vector cursors. Honest abort and adversarial soundness are separate ledgers.

Current frontend [DigestTranscript](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-transcript/src/digest.rs) and [legacy facade](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-transcript/src/legacy.rs) squeeze16 bytes for a scalar. E2 needs two independently uniform K coordinates, not an E2 container filled from one128-bit draw. Replace the existing infallible `Transcript::challenge` seam with fallible draws and propagate bounded exhaustion through stage, adapter and top-level APIs as rejection. Do not continue with a fabricated challenge, default zero or an unchecked failed transcript.

The independently reviewed [outer compiler](AKITA_INSTANCE_ACCUMULATION_OUTER_TRANSCRIPT.md) specifies H_o through literal full-context keys, atomic field-vector entries, separately indexed shell-coordinate entries, bounded samplers and the exact conditional full-tape law. It preserves stage1/2 uni-skip families, batch coefficients, adaptive rounds, stage6a-to6b carries, curated stage4/6b opening absorption and prefix selectors. Powers remain deterministic references. The concrete adapter is SHAKE256 under the separately disclosed D1-Outer-FS/1 extraction-use heuristic. Actual emitted descriptors/caps and alignment with the finalized carrier remain required; no inner honest cap or Q32 policy transfers automatically.

[bridged_akita_session](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/adapters.rs), lines1337–1350, currently compresses the frontend context to one K scalar. Replace it with the outer compiler's checked full-instance transition while preserving the same complete context and prefix; widening a scalar or copying `state()->[u8;32]` is not that transition. The facade documents state as a debug peek. Bind and compare the verifier-derived native group instance, including setup, selected row, ordered roles, original commitments, points and evaluations; preserve the paired indexing conversion. There is no replacement bridge draw or transcript reset.

## 5. Pending inner endpoint and original outer advice

### 5.1 Inner cut

[prove_stage8](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/stage8.rs), lines55–180, currently assembles groups and then proves the complete batch. Factor the **shared canonical assembly**, not just the final proof return type. The pending output contains verifier-derived ordered role/original commitment/source-shape/E2 point/evaluations; the prover retains matching original handles. The endpoint, source-domain and raw multipoint obligations of the reviewed inner design must also be present.

The pending verifier keeps every mode/presence/layout/packing/program check and derives its ledger from checked frontend outputs. It cannot accept an archive-supplied list as equivalent to executing those checks. It returns pending status only. The ordinary outer path uses the same generalized assembly but completes full PCS proving/verification.

### 5.2 Advice ownership

D1 needs original untrusted C_archive and installed trusted C_static. They are outer byte-advice sources even though the selected inner application has no optional advice objects.

- [Advice commitment](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/witness.rs), lines340–378, packs canonical LE u64 words into K coefficients and a configured physical slot. [Advice packing](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/common/src/advice.rs), lines29–44, pads the word capacity. Choose an aligned byte capacity sufficient for the logical frame/envelope; never round a required partial word down.
- Physical advice arity is `max(word_vars,14)`, capped at34, in [lattice packing](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/jolt/lattice/packing.rs), lines183–215. Preserve distinct trusted/untrusted roles and the `advice-dense-words-v2` map. Serialized inner K coefficients are outer byte words; this is not zero-copy K-source storage.
- [commit_trusted_advice](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/preprocessing.rs), lines229–251, returns the original plan/commitment/hint. The verifier supplies the externally installed commitment; the proof's untrusted advice commitment is a different source.
- Actual advice claims come from **stage4 RAM initialization directly to stage8**, not generic Dory stages6b/7. Preserve the address/capacity/presence/metadata checks in [stage8 verification](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/stages/stage8/akita.rs), including its direct advice-claim path.
- Final batch order is UntrustedAdvice?, TrustedAdvice?, then FieldInc **or** committed bytecode/program-image groups, then OneHotTrace. Initial stage0 absorption has its own order; do not replace it with batch order. The two optional program modes are mutually exclusive.
- [Native batching](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/native_batching.rs), lines312–329, imports auxiliary handles by retaining their source, recomputing against the target setup and comparing the original commitment. Preserve owner-bound handles; a hint is not freely transferable.
- Trusted selected-row/layout/expected-size checks precede payload decode in shape_guard. Generalizing E must not widen K payloads or remove those checks.

There is no separately committed actual byte length in the padded advice commitment. D1's checked logical framing/EOF and envelope policy determine consumed bytes. Unread physical capacity can remain arbitrary K where the admitted source relation permits it. Do not re-encode an extracted weak source as an honest byte Vec or infer u64 canonicality from raw LD. The [consumed-byte lemma](AKITA_INSTANCE_ACCUMULATION_CONSUMED_BYTES.md), [guest C7 obligations](AKITA_INSTANCE_ACCUMULATION_GUEST_SPEC.md) and same-original static-reference comparison remain required.

Do not embed the broad host adapter in the guest and omit its setup cost: deserialized adapter preprocessing is a recipe with a skipped cache, and lazy verifier construction can build prover-side setup. The guest uses the approved native static representation/verifier boundary and performs every proof-dependent contraction itself.

## 6. Actual outer profile generation

Profile generation is deterministic installation work after the fixed guest implementation supplies its bounded geometry. A complete parametric design can precede that implementation; **accepting an installed profile** requires the resulting artifacts and proofs. A compiled guest is not a circular prerequisite to writing its implementation, and missing artifacts are not permission to accept arbitrary rows.

1. **Freeze the guest and source contract.** Generate the exact codec/emission/section maxima and instruction/provenance inventory in GUEST_SPEC. Derive actual archive/static/public capacities, maximum live RAM/stack/heap, expanded code and bounded execution paths, including rejected parses/samplers. Keep the accepted T16 language unchanged.
2. **Derive ordinary configuration.** [ProverConfig](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/config.rs), lines113–169, derives trace and RAM geometry from execution: Akita floor4096, otherwise `(rows.len()+1).next_power_of_two()`, plus touched RAM/image and one-hot/read-write parameters. A declared trace exponent alone is insufficient. Use [one_hot_trace_setup_shape](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/setup.rs) and the actual compiled program.
3. **Derive grouped producers.** [grouped_setup_params](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/preprocessing.rs), lines90–143, uses exact main shape, advice capacities, dense shapes, polynomial count and chunk profile. Stage8 requires exact main/setup arity and P equality. Use consistently padded fixed geometry or an authenticated finite family; never proof-controlled uninstalled variants.
4. **Generate the selected complete E2 family.** Implement OUTER_CARRIER §9's finite candidate/validator rule, including exact source splits, all-valid response depths, unsliced B/F, shared D/H, full coefficient rows and every complete direct-terminal candidate. Current fp128 `(Field,Field)` families and their EOR rows are not this construction. Bind the new method, basis, source encoding and complete row descriptors into installed identities.
5. **Respect actual geometry without inheriting offload.** [Existing family generation](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/schedules/mod.rs) has production K_onehot=16 trace exponents12..30 and dense arities14..34, with offloading from trace exponent21. The selected new outer family explicitly has no offload and directly pays matrix evaluation. Existing ranges are not proof that the guest fits; extend finite validation deliberately where necessary or reject the whole installation without shrinking T16.
6. **Freeze validated grouped rows.** [schedule_registry](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/schedule_registry.rs) generates program-specific groups at preprocessing, not during proving/verifying. It currently limits groups to at most one full-width producer and at most three auxiliaries with one full-width producer; bounded-only Single grouping rejects a guide without a recursive child; provisioning is capped at128 rows. Apply actual producer contracts rather than assuming any two advice objects fit any small row.
7. **Certify installation.** Emit and independently check program/preprocessing identity, source maps/roles, setup provenance, all A/B/D/F/H/shared-prefix views, source/reference collision envelopes, field/carrier/sampler/bridge rules, full extraction/guest certificates and complete runtime/statistical ledgers. The inner33 roles are not the outer catalogue.

The new outer family deliberately has zero intervening full-commitment recursive nodes. It must implement actual terminal creation/encoding and every predecessor strong check; it must not drop B/F from a previously created OuterPayload. Existing `RecursiveCommitmentConfig` and direct/recursive/chunked families remain relevant to other selected paths, including the inner final folds and recursive control, but are not a hidden setup-offload dependency of this outer family. Any reused path must retain its actual source/norm/terminal contract.

Static coefficients alone occupy74,907,648 bytes:9,363,456 u64 words require at least a2^24-word padded source, or268,435,456 bytes when stored as K coefficients, **before** metadata/other regions, source digits, hints and caches. These are storage facts, not an exact C_static capacity, outer trace, proof size or measured memory peak.

## 7. Verification and performance acceptance

The inspected [ordinary Akita e2e recipe](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/tests/akita_e2e.rs), lines66–143, shows configuration, `preprocess_full_with_advice`, retained trusted object, witness, ordinary proving and complete verification. It was inspected, not executed. The SDK recursion example's Dory aliases and per-run embedded archive do not supply the fixed D1 host/guest integration.

Implementation acceptance must exercise the actual changed paths: configured E2 arithmetic/canonical codec; mixed MLE/binding and bit order; prover/verifier transcript agreement including rejected candidates/carries; original-handle/shape/catalog/advice substitution rejection; all selected inner recursive openings and the outer root/direct terminal; bounded guest framing/source/static/endpoints/final decision; one complete ordinary receipt and its externally expected success/panic values. Deterministic algebraic/source certificates remain required even if honest executions agree.

Compare against completed-segment recursive aggregation for the **same original whole-run relation**, T16 workload, common disclosed security policy, authenticated setup and cold/warm preprocessing treatment. Count private archive retention/replay, all static/source commitments, guest hash/matrix work, full outer proof, final transport and verification. Different verifier guests can have different actual trace capacities; equality of semantics/policy, not artificial equality of shapes, defines the control. There is no performance conclusion from removing C_t or from generic E2 arithmetic existing.

The original design review did not execute a complete proof or benchmark. Generated implementation/profile evidence and end-to-end execution/performance remain acceptance gates.

## 8. Component implementation evidence

The field/transcript increment has exercised the following paths:

| Execution | Observed result | Scope |
|---|---|---|
| Field/transcript package tests with default features plus `solinas,asm,allocative,bytemuck,parallel,transcript-d1` on their respective packages | 223 passed | Includes the configured field and D1 compiler regressions; not a native accumulation proof |
| Field/transcript package tests with no defaults and `jolt-field/solinas,jolt-field/bn254,jolt-transcript/transcript-d1` | 153 passed | Includes the feature-minimal documentation path |
| Direct component executable over borrowed K source values `[1,2,4,8]` and nonbase E2 challenges | Evaluation `363 + 111 U`, canonical round trip accepted; RankShell had 24 unit and 16 double coefficients, with squared norm 88 | Executes the library components; temporary executable removed afterward |

The inner raw-block reader initially bypassed the shared bit budget. A regression reproduced that defect before the fix and passes afterward. Cached block rereads now consume budget; sequential bit reads charge only the bits returned.

Generic `--all-features` testing is not admissible for this workspace: it enables certified field proof-linkage features that require the registered matrix scripts. That guard was not weakened. Cargo also reports existing ignored `default-features` inheritance for `jolt-kernels`, `jolt-poly` and `jolt-profiling` in the prover/kernels manifests. These warnings are not a clean full-workspace result.

The native samplers and semantic emission registry, source/evaluation caller migration, full-ring algorithms, installed profiles, archive guest, complete receipt and parity comparison are not established by this component evidence.

A second cap defect was reproduced after direct record appending was introduced: a complete 129-byte outer record was accepted under a 128-byte wire-object limit. The append path now checks both the complete record and cumulative prefix before allocation/mutation. The boundary regression also checks preservation of the prior prefix and permanent rejection. After this fix, all 12 D1 regression tests, the selected field/transcript `clippy --all-targets -- -D warnings` command, and the scoped format check passed. The independent correction review found no remaining component defects; it did not establish production registry/emission rules, native sampling integration or protocol security.

## 9. Reused implementation and correction evidence

The selected implementation retains the original packed `RecursiveWitnessFlat`,
owner-bound proof scopes, source commitment compiler, cached A-matrix kernels,
accepted source folds, Eq109 witness producers and original F/H path storage.
The changes are at the incompatible boundaries: K-source/E-evaluation APIs,
full-ring two-plane partials, exact coefficient rows, source-domain and
decoder-adjoint normalization, physical receiver extent and the live D1 compiler.
There is no replacement hash commitment or unrestricted commitment-homomorphism
assumption.

The kernel migration keeps raw Spartan, RAM, register and reference source
tables over K until their first active binding. Inactive logical axes retain
the original K table. The sparse register binder allocates only its merged,
already-bound E cells; its previous full-size allocation failed the retained
capacity regression before correction.

| Execution | Observed result | Scope |
|---|---|---|
| Mixed polynomial package tests and selected strict Clippy run | 141 tests passed | Original K MLE/binding and both variable orders; not a frontend proof |
| Witness, derive and kernel tests with `akita,field-inline,allocative` selected on kernels | 231 tests passed at that revision | Includes mixed optimized/reference consumers and sparse allocation boundaries; an incidental macro token-snapshot test was subsequently deleted |
| Direct mixed register-reduction executable | All three round polynomials and all three final claims matched independent Boolean-basis sums | Real K/E consumer with nonbase challenges; temporary executable removed |
| Kernel tests with its default features disabled and `akita,field-inline,allocative` enabled | 184 tests passed | Exercises the kernel's nonparallel branches; does not assert that all transitive dependencies lack parallel features |
| Selected witness/derive/kernel strict Clippy and scoped format check after corrections | Passed | Cargo still reports the manifest inheritance warnings in §8 |
| Native source-domain and normalization executable using the real H1610 compiler | 9536 proof bytes; eight degree-33 domain rounds and eight degree-two normalization rounds; prover/verifier endpoints matched direct original-K MLE evaluations | A mutated proof was rejected and permanently poisoned its compiler; this is a component execution, not an accumulation proof or final receipt |
| Native type suite | 319 passed; one ignored | Includes matching original RAM A/B/F reuse and rejection of unequal raw maps, unequal source factorizations and cross-level D/H aliases |
| Focused CPU full-ring, inner planner and native sampler suites | Seven, three and two tests passed, respectively | CPU coverage includes noninvolutive block/column maps, both E planes, independent signed-integer digit references and rejection of nonidentity terminal views; the terminal regression also passed in release mode |
| Native prover, verifier, CPU backend and PCS all-target check | Passed | API/type integration, not complete installed-profile execution |
| Native strict Clippy over the seven selected packages and all targets | Passed | The later mapped-source correction also passed the CPU all-target strict check |
| Real regeneration of all existing legacy schedule families, followed by seven-package native tests | 16 artifacts published; 1158 tests passed in 36 suites, with 12 ignored | Repairs required opening-method metadata without fallback defaults; these are not the new D1 production profile artifacts |
| Fallible transcript package with `transcript-d1` | 115 tests passed in 12 suites | Includes real inner/outer compiler transport, canonical K/E admission, atomic and empty vectors, accepting boundaries, restricted native borrowing and exact outer result-payload caps; not a complete frontend or receipt |
| Polynomial, crypto, claims, derive and sumcheck package tests after fallible capability migration | 400 tests passed in 23 suites | Includes real generic extension fields; not a complete Jolt proof |
| Opening and HyperKZG package tests | 27 tests passed in eight suites | Includes real Dory homomorphic opening and tamper rejection; this does not grant homomorphic accumulation to Akita |
| Transcript/poly/crypto/claims/derive/sumcheck/openings/HyperKZG all-target strict Clippy with `transcript-d1` | Passed | A later committed-only sumcheck configuration exposed one argument-count lint, now explicitly justified; the earlier configuration did not exercise that feature |
| Bare transcript and Dory library strict Clippy | Passed | Confirms genuine legacy capability compilation without default transcript backends |
| Native transcript tests and seven-package native all-target strict Clippy after restricted-borrow integration | 24 transcript tests passed in two suites; Clippy passed | The fixed test registry now admits its mandatory context record; no production registry was weakened |
| BlindFold tests after typed emission migration | 73 tests passed in seven suites | Includes actual proof/verification and final transcript-state agreement; full frontend integration remains incomplete |

The native correction review identified three installation/representation
boundaries that require explicit checks:

1. A fold-admitted raw permutation has the S-linear block/column form stated in
   [OUTER_CARRIER §2](AKITA_INSTANCE_ACCUMULATION_OUTER_CARRIER.md). Its response
   row uses the registered column permutation and pairs each raw partial with
   its physical block's shell. A mere coefficient bijection is insufficient.
2. Distinct RAM handles can reuse original A/B/F only with matching complete
   source geometry, raw map, encoding and role chain. Separate opening levels
   need separate D/H namespaces even at equal dimensions. Both the unequal-map
   alias and cross-level alias were accepted before correction and have
   failing-before regression evidence.
3. Receiver live storage L and committed extent N are different. The exercised
   selected receiver has L=3,555,328 and N=4,194,304, so B remains32 and the
   original source includes638,976 required zeros. No padded source copy is
   needed. A nonzero-imaginary-part assertion was removed from M=delta=1 test
   cases: those partials are correctly K-valued. Independent scalar, plane,
   centered-response and A-product checks remain.

For admitted nonidentity views, the CPU partial kernel now inverts only the
block and column factors: $PB+M\delta$ indices, rather than all $128PBM\delta$
coefficient addresses. The independent mapped-source execution covers signed
base-32 digits, virtual padding and both planes. This is a representation-cost
reduction, not an end-to-end performance result.

An unused legacy-style folded-partial scratch calculation was removed rather
than retained with an incorrect identity-map formula. Exact coefficient rows
use the actual retained partial digits; they never consumed this scratch.
The real terminal arithmetic remains separate. Its preparation now checks the
exact scheduled identity view before entering the factored partial kernel.
A coefficient permutation previously reached the kernel's debug assertion;
both that permutation and fold-admitted block/column permutations are now
rejected before terminal partial evaluation.

The two catalog policies are now explicit:
`FullRingE2OuterDirect` selects the ordinary outer/direct terminal and outer
SHAKE compiler; `FullRingE2InnerRecursive` selects the compact inner continuation
and H1610 compiler. Root admission checks the policy, installed setup family and
actual compiler together. This does not install a production source/site
registry or certify the profile.

The independent focused correction review found no remaining defect in the
corrected raw-map algebra, original-role alias checks, per-level D/H ownership,
receiver extent and removed unused scratch. That review was static. Its limits
remain explicit: no complete mapped-source plus noncanonical-F/H pipeline,
bound receiver scope-transfer test, installed-profile proof or performance result.

Remaining acceptance work is unchanged: fallible frontend operation/site
plumbing, the selected Jolt Akita adapter, repeated segment-instance updates,
installed and independently checked profiles, the archive guest, the complete
ordinary outer receipt and the same-semantics performance comparison. These
component results establish none of those end-to-end or security claims.

## 10. Repeated-update ownership contract

The native source APIs now implement this ownership direction and have passed
an independent static review. Execution of the complete installed root/update/
suffix/terminal protocol remains required. The implementation reuses the
existing mixed-source `reduce`, packed receiver, commitment material,
source-domain and normalization drivers, complete strong check, verifier replay
and final suffix.

One `ProofScope` owns the bounded inner run and the same actual H1610 compiler.
An operation occurrence is distinct from its installed reduction template:
successive update occurrences reuse Update and ReceiverRoot keys; finishing
uses the three existing distinct final templates. Do not append T artificial
levels to the schedule, reset a level counter, or import/recommit a receiver.

The selected lease mode has one active occurrence and at most one staged
successor. Ordinary legacy and ordinary outer-direct scopes keep their current
semantics. Context issuance and all proof computations require the active
occurrence. Scope/plan access checks liveness, not an artificial root-zero
request. A staged destination is not a second active occurrence.

Successor commitment retains the producing witness at the active occurrence
for the outgoing strong check. Its destination material is dormant; only exact
staged A-only terminal-image publication can use the pending-material exception.
Terminal consumption, openings and folds cannot. After complete strong success,
a checked consuming promotion verifies the coupled witness, material, public
binding, original role, extent and committed-instance identity; then it retires
the predecessor's computational authority. Drop-only reclamation must still
work after retirement. Old snapshots do not acquire new permission. Errors
abort the scope and poison the same compiler; no partial retry is admitted.

Admit the exact registered zero receiver before the first update. The native
owner checks declared $1\le T\le16$ and the update index. The application driver
must establish VM adjacency, original public and private RAM continuity and
active-row totals. Exactly T complete updates precede final normalization; then
reuse the three shrink steps and real A-only terminal once. The verifier owns
the same public phase machine and obtains fresh obligations from actual
frontend replay, not a success ledger.

The registry's source ID identifies an immutable **source contract**, not an
actual committed instance. Use one canonical RAM contract in two ordered root
slots, as required by [MATRIX_REGISTRY §1](AKITA_INSTANCE_ACCUMULATION_MATRIX_REGISTRY.md).
Definitions remain unique. Each occurrence retains its own commitment ID,
public payload, endpoint, source digits and original F path. Incoming and
outgoing handles may be identical when RAM is unchanged; contract equality
alone never proves boundary continuity. Count all four occurrences, 120 fold
coordinates and Update.D width 6,240 even though RAM uses one contract.

Semantic, public-group and physical-witness order are separate checked maps.
The guest's semantic order is receiver/Main/RAM-minus/RAM-plus; the existing
compact public order is receiver/RAM-minus/RAM-plus/Main, while its physical
witness order is Main/receiver/RAM-minus/RAM-plus. The installation and emission
plan must bind the complete mapping; do not reorder only keys or handles.

The canonical descriptor defines ReceiverRoot=0, Main=1, RAM=2, Final2M=3 and
Final1M=4. It stores five definitions, while both RAM occurrences remain in
every ordered root ledger. Per-update source prefixes form an exact T-entry
table of checked domain/decoder plans, with immutable RAM/decoder data shared
where applicable. Each entry's literal inputs and decoder selections are bound.
Nonempty installation bytes are transport, not authentication; the application
must check each domain mask against the actual segment's active and hole rows.

Frontend callbacks receive `NativeTranscriptBorrow` by value. It retains a
private reference to the original compiler and permits emission, sampling and
rejection, not replacement, extraction or closure. Strong completion uses
driver-issued consuming authority rather than a caller-supplied success flag.

## 11. Frontend identity and fallibility contract

Keep kernels and claim algebra transcript-independent. Reuse existing stage,
member, challenge, opening and layout identities, but do not discard them before
emission. Typed fallible challenge callbacks replace anonymous scalar iterators.
Input sums retain their member; curated output claims retain their original
opening ID through alias filtering. Round sites use the sequential protocol
round, not worker order or the diagnostic thread-local scope.

Each explicit borrowed message descriptor fixes structural occurrence,
Public/Proof kind, codec and count. Draw descriptors fix the existing complete
stage/request metadata, operation, arities, sampler and carries. An atomic
vector stays one request; powers use one coefficient draw plus deterministic
expansion; adaptive rounds remain separate. Preserve called empty events,
compressed-polynomial omission of c1, curated stage4/stage6b order and original
carry projections. The accepting installed event schedule must be checked before
compiler closure; `finish` alone does not establish completion.

All frontend transcript operations return errors. Initialization receives full C;
there is no universal Default/new/state requirement or digest bridge. Typed K/E
atoms use their actual canonical encodings, never generic reversed blobs. The
same actual compiler crosses the checked frontend/native boundary.

Keep commitment/setup/source ownership independent of transcripts. Put opening
operations behind transcript-specific capabilities. Dory's upstream callbacks
can use only a genuinely infallible legacy capability on the same legacy state;
they cannot unwrap a D1 error, stash it while continuing, or fabricate a scalar.
The selected Akita capability consumes the actual checked D1 handoff. Production
site/context/profile artifacts and the complete caller cutover remain required.

The universal transcript and sumcheck libraries now use checked borrowed
message, draw and round plans. Batch-opening data remains independent of T;
`OpeningScheme<T>`, `NativeBatchOpeningScheme<T>` and the batch execution
capabilities express the supported transport. Homomorphic batching receives
separate clear or hiding emission scopes and is not granted to the selected
Akita scheme. Full frontend caller migration is still in progress.

Review found two defects in the initial frontend wrapper: its mutable native
callback could replace or close the compiler, and outer draw results bypassed
the additional frontend payload cap. Both had failing-before regressions.
The corrected callback receives a private, non-replaceable inner/outer native
capability; only the owner can close after its accepting predicate. Outer
scalar/vector paths check the complete request/result payload before sampling
or result allocation, including empty vectors and zero-length powers.
The 115-test transcript run passed after correction, and a focused independent
static re-review found no remaining defect in those changes.

The stage recipes now pass explicit batch/member plans and retain typed
`OpeningOrigin` values through curation. Generated member occurrences retain
absent optional slots. A literal, allocation-free v1 identity mapper encodes
the complete Jolt and field-inline identities; it is not admission or
authentication. In particular, an advice-opening variant remains distinct
from a polynomial-opening variant that resolves to the same advice column.

BlindFold now uses caller-owned emission plans and propagates typed transcript
failures. Its existing verifier appended the final witness opening, but its
prover omitted that final append. The prover now emits the same terminal
boundaries. No later challenge or proof field changes; its final transcript
state intentionally changes. This is a correction to retained non-Akita
behavior, not a lattice-compatible BlindFold assumption.

The selected Akita adapter, production installations and complete caller
cutover remain incomplete. The default prover library and the
`akita,field-inline` prover library passed checks with the typed pending-opening
carrier and integrated frontend cut. Their dependencies include the matching
verifier paths. Finite legacy-only field-inline fixtures retain real stage
proof tests; they cannot admit D1 fields.

## 12. Retained compiler loans and the pre-PCS cut

`NativeTranscriptBorrow::with_frontend` now lends `BorrowedInner` or
`BorrowedOuter` over the native owner's actual compiler. It uses the same
codecs, samplers, caps and failure handling as the owned frontend. Admission
reads the live registry, limits and complete original context. A successful
callback returns its value and complete native instance; the helper enforces
the installed accepting boundary and handoff before returning to the owner.
It grants no reset, replacement, extraction or final closure.

Native begin/update callbacks retain their caller error type through
`X: From<AkitaError>` rather than converting frontend errors into an opaque
native proof error. Returned or ignored transcript failures poison the same
retained compiler. Outer draw sites remain single-use across the loan: native
continuation uses its own site, not the preceding frontend draw's identity.

| Focused verification | Passed |
|---|---:|
| Actual inner/outer frontend integration scenarios | 18 |
| Frontend doctests, including capability compile failures | 6 |
| Native borrowed-compiler tests | 6 |
| Native transcript capability doctests | 4 |

The native prover and verifier library check also passed. An independent,
read-only authority review found no blocking issue in this loan scope; it did
not approve a complete production installation or protocol security claim.
Later integrated caller changes are not covered by those results.

The inner compiler retains the original context once. `new_owned` moves its
`Vec<u8>` without a context copy; the borrowed constructor makes one retained
copy. Storage adds one vector header and a context-type word, with layout
padding. Loans neither construct a compiler nor copy the context.

The frontend cut reuses the original stage-zero commitments, stages one
through seven, native stage-eight claim assembly and commit-time opening
handles. `akita::prepare` stops before `PCS::prove_batch`.
`prepare_native_instance` replays the frontend and derives its pending native
obligation without verifying a segment PCS proof. Ordinary complete
proving/verification consume these same pending instances through the real
PCS, rather than a second implementation of frontend algebra.

`JoltProof` carries a typed final-opening state. Its default remains the
complete PCS proof; preparation carries the actual pending instance and
handles, not an empty proof or success flag. Frontend-only readers accept
either state, while complete verification still requires `PCS::Proof`.
These integrated cut APIs are not yet end-to-end verified. They do not by
themselves establish VM adjacency, source admission, the archive guest,
the final receipt or performance parity.

## 13. Original-source reuse and exposed migration defects

The CPU full-ring backend already owns source import, A/B/F commitment,
retained handles, normalization, mixed-source reduction and the final
A-only opening. The selected Jolt adapter must call those paths. A legacy
K/K hint is not an E2 handle, even when its public commitment bytes contain K.

Main's packed one-hot rows are also reusable. Their actual chunk size remains
16 or 256, with the original row owner and hot-position stride. The compact
producer instead charges a Boolean whole grid (`UnitOneHot { source_chunk_size: 1 }`).
An at-most-one-hot chunk has coefficients in $\{0,1\}$, so it satisfies that
weaker coefficient-domain contract without a storage conversion or a
sparsity-priced norm bound. The new admission refinement is restricted to
the full-ring E2 family, canonical encoding, one base-32 digit and the exact
installed geometry. Legacy matching remains exact. The CPU admission and
Jolt source regressions passed. Selected producer execution is recorded below;
it does not establish a complete segment relation.

RAM requires the original 26 balanced base-32 digit planes, not values
reconstructed in K and decomposed again. `PackedRamSource` now owns those
planes and reuses the existing packed-digit commitment and fold kernels.
The six virtual holes in the 32-slot raw digit axis are not physical source
digits. Neither source work establishes segment boundary adjacency by itself.

The Jolt-owned legacy catalogs were missed by the earlier 16-file Akita
regeneration. Eleven `jolt-akita` test targets reached catalog decoding and
failed because old terminal records lacked the required `opening_method`,
`source_encoding` and `response_codec` metadata. The real Jolt generator has
now regenerated all ten catalogs. No deserialization default or JSON repair
was used. These legacy artifacts are not the selected E2/D1 installation.

The old Dory stage-zero field-inline test compared the prover's second
challenge with a stripped transcript's first challenge. That negative check
was vacuous; its preceding equality also called the same encoder twice.
Those transcript assertions were removed. The actual full-column commitment,
layout and hint-identity checks remain. Stage-driver twin tests now share the
finite legacy fixture schemas instead of copying another emission convention.
All 23 prover-library field-inline tests passed, including these stage paths.

Source-text census tests and their blessable snapshots were removed, not
re-pinned to the new APIs. The concrete Fiat-Shamir attack suites remain.
Full caller migration and a real selected T=1,2,16 run are still required.

## 14. Corrected original-plane envelope and concrete producer execution

The [Akita source contract](../../crates/akita-params/src/sis/committed_source.rs)
now separates `OriginalBalancedDigitPlanes` from canonical field
decomposition. An original RAM path may decode to zero in K while retaining
nonzero digits, including a top-plane digit of $-16$. Decoded K reach is
therefore not its committed norm. For $L$ logical coefficients and $\delta$
original base-$B$ planes,

$$
\lVert w\rVert_2^2 \le L\delta(B/2)^2.
$$

| RAM envelope, $L=2^{16}$, $\delta=26$, $B=32$ | Squared norm bound |
|---|---:|
| Canonical decomposition, invalid for arbitrary original paths | 420,478,976 |
| Full original digit alphabet in every plane | 436,207,616 |

The corrected class requires a full-field declaration and the exact intrinsic
base/depth. Admission binds the declared field width to K before successful
early returns; it rejects canonical or narrow contracts for the original
packed source. Canonical legacy norms keep their separate residual-bit rule.
This preserves the noncanonical-path requirement in
[Akita, Remark 10.32](SOURCE_PROVENANCE.md#akita) and
[the original-source cost model](AKITA_INSTANCE_ACCUMULATION_MEMORY_COST.md).

The all-valid RAM response bound remains $16\cdot56\cdot16=14{,}336$, which
fits three balanced base-32 digits. The accepted-response difference is
$32^3-1$; its collision target $4\cdot56(32^3-1)=7{,}339{,}808$ remains below
the existing certified radius $8{,}126{,}340$. No rank, geometry, table gate,
or claimed security level was weakened.

`CpuBackend::commit_full_ring` admits a frozen producer contract and calls the
same A/B/F executor and retained-source implementation as the ordinary commit
API. Receiver and RAM do not need fabricated standalone opening catalogs:
the compact opening family's own producer is Boolean Main. The consuming
opening still requires its complete admitted schedule and domain obligations.
Canonical empty chunk slots now yield full-width zero responses; reversed
or out-of-bounds ranges still fail.

The native run passed 891 tests in 16 suites, with six ignored, followed by
strict Clippy checks. This includes corrected norm/admission, packed RAM,
empty chunk and compact-planner cases. Independent read-only re-review found
no actionable defect in this scope; it did not approve complete protocol
soundness, authenticated setup, guest execution, or performance parity.

`akita_config::full_ring::CompactInner` is the concrete K/NR5-E2 config.
The existing artifact generator produced
`akita/artifacts/schedules/akita-full-ring-e2-compact-inner-v1.aks` from its
one mixed root request. The existing render/materialize/publish/decode path
passed 14 generator tests and strict planner/backend checks. It retains the
three shrinking folds and A-only terminal.

The [compact provisioner](../../crates/akita-cpu-backend/src/setup/full_ring_compact.rs)
derives source and matrix shapes from that admitted row and checks them through
the existing canonical registry. It uses independent 512-bit modular samples
for each original coefficient, not the legacy shared-prefix key distribution.
An actual release-mode execution supplied OS-CSPRNG bytes, provisioned all
33 roles and 4,681,728 K coefficients, committed the registered zero receiver,
56 Boolean Main columns and original 26-plane RAM, then imported the retained
RAM handle through destination-key recomputation. The CSPRNG assumption and
lack of installation authentication were explicit. The temporary driver was
removed after execution.

The actual inner and outer D1 loan tests also cover a caller that catches a
semantic protocol error and then supplies the correct native handoff. Both
owners remain poisoned. `Transcript::with_protocol` retains the original
error type; frontend roots and independently callable stage-zero/stage-eight
boundaries use it. The 18 D1 frontend tests, the 23 field-inline prover tests,
the `akita,field-inline` prover-library check, and the selected Jolt Boolean
source test passed. Strict BlindFold/transcript checks passed after correcting
the shared fixture's crate namespace. Cargo still reports the existing ignored
workspace `default-features` declarations for kernels, poly and profiling.

This is producer and component execution, not a T=1 accumulation run.
Production frontend installations, continued-segment consumers, full update
and suffix execution, authenticated static artifacts, archive guest, complete
outer receipt and the same-semantics recursive performance comparison remain
unverified.

## 15. Reusing declarations, curation and descriptor storage

The frontend metadata now comes from the existing claim declarations rather
than fabricated zero-valued challenge or output objects:

- `SumcheckChallenges::visit_scalar_declarations` visits the original typed
  scalar IDs in declaration order. Vector/reference containers reject before
  any callback; their existing concrete frontend operations remain authoritative.
- `OutputClaims::visit_opening_declarations` accepts one typed scalar,
  vector-length or optional-presence entry per declared field. It preserves
  original vector indices and composed base-then-extension order. It checks
  declaration structure, not the protocol correctness of supplied dimensions.
- Static alias filtering uses a borrowed iterator over the existing alias
  declarations. Stage 4 reuses the same RAM-contribution/register interleaving
  for values and identities, inserting into the existing RAM vector. Stage 6b
  still uses actual derived-point equality and separately checks equal values
  at aliased points. An eager shape-only plan cannot replace that decision.

The existing trusted-profile commitment-length calculation is exposed as
`jolt_akita::commitment_transcript_shapes`. It checks the selected row against
the ordered opening layout before returning the original nine-leaf native
schemas. It does not construct a commitment, invent an auxiliary catalog or
assume an E2 payload width. The generated compact K/E2 catalog was loaded in
the boundary regression: Main still serializes eight K coefficients, 128
bytes, not eight E2 coefficients.

[`schema::Schema`](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-transcript/src/schema.rs) owns metadata and
schema-local handles, then lends the **existing** `MessageSite`, `DrawSite`
and `MessageScope` types through `with_bound`. The callback starts only after
all stored definitions pass the supplied binding. Nested view storage has a
finite depth bound; it uses no unsafe lifetime extension, leaked strings,
global pool or self-referential struct. Runtime lookups allocate nothing.
This is storage, not a registry, source authentication or an accepting ledger.
Actual D1 owners continue to revalidate against their installed registry.

The existing outer-compiler continuity regression now uses this owned storage,
then compares the real challenge result, complete context/prefix and work
accounting with direct native execution after handoff. The native compiler can
leave the callback; borrowed descriptors cannot. The transcript run passed 126
tests across 12 suites, including the compile-fail lifetime check.

An additional release-mode component run exercised two atomic E2 draws, an
indexed carry with argument bytes, and the native handoff through this storage.
It compared both challenge vectors and the complete context, prefix and work
against direct execution: 1,353 prefix bytes, two queries and 2,048 raw sampled
bits matched. The temporary driver was removed. This verifies descriptor
transport and continuation, not an accumulation update or performance parity.

Additional observed checks: 163 base claim/derive cases, 165 field-inline claim
cases, 67 native field-inline verifier cases, and three commitment shape cases
passed. The native prover/verifier all-target check passed with
`akita,field-inline`. The regenerated ten legacy Jolt catalogs also passed
90 Jolt-Akita tests. The obsolete source-text `protocol_boundaries` suite was
removed rather than adding an exception for legitimate feature-specific tests.

Independent read-only reviews found no actionable defect in the scoped
declaration/curation/profile-size changes or the descriptor ownership design.
They did not approve a complete production frontend installation or protocol.
The late-resolution and concrete producer work below advances this boundary.
Production frontend installation, native update/decider, archive guest, final
receipt and parity measurement remain required. No completed segment PCS proof
or legacy scalar bridge substitutes for those missing executions.


## 16. Concrete Main producer and honest capability boundaries

`CommitmentScheme` now contains source/evaluation types, real setup and retained
commitment data. Arbitrary-polynomial commitment belongs to
`PolynomialCommitmentScheme`; complete proof data belongs to `OpeningProofData`.
Opening execution requires that proof capability. Bounded auxiliary setup and
full-width field-object commitment are separate capabilities. Existing Akita,
Dory and HyperKZG implementations retain their actual operations.

The explicit `JoltProofWithOpening` carrier does not require a completed PCS
proof. The complete `JoltProof` alias selects the real opening-proof type.
The carrier preserves the old serialized name and field order. Public setup
serialization is required at actual transport boundaries, not by an in-memory
source producer. A missing setup serializer is not replaced with key regeneration.

[`CompactInnerTrace`](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/compact_trace.rs) is a real
K-source/E2 producer. It derives the canonical trace plan, checks the selected
4096-row/k16 column geometry, and resolves Main from the actual four-group
CompactInner root catalog. Column counts alone do not fix bytecode/RAM lengths;
the frontend installation must admit the actual 4096/65536 table sizes. The
producer reuses `TraceOneHotColumn`, full-ring source import,
the original CPU commitment executor and K commitment serialization. An empty
Jolt auxiliary list does **not** select a fictitious single-group catalog.
The original stride stays 16; Boolean-whole-grid admission does not rewrite it
to 1. This type has no arbitrary-polynomial or complete-opening capability.

Generic `AkitaProverHint<E>` retains the original public group and opaque backend
handle. Consuming it requires exact equality with the expected public commitment.
Verifier setup shares only the original public matrix installation; it does not
retain the prover backend's private source/session state.

The original stage-zero recipe now accepts an optional real `AuxiliaryProducer`.
Absence rejects requested advice, committed-program objects or mandatory
field-inline production **before** the preamble. The ordinary complete prover
supplies its genuine existing transparent producer. The native no-auxiliary
route uses the same preparation, extraction-error check and stage 1–7 spine;
it does not implement all-error auxiliary methods or construct and strip a PCS
proof.

Late output validation receives actual `OutputCuration` points. The finite
fixture compares the complete source-origin roster after the shared stage-6b
predicate, not an arbitrary omission mask. The clear prover now checks unequal
dynamic-alias values as the verifier already did. Hidden verification separates
round verification from output absorption: actual points and the alias-adjusted
roster/count are checked before absorbing the original output commitments once.
BlindFold retains the corresponding equality constraint.

A release-mode producer execution used freshly provisioned literal matrices.
It committed 56 original n16 columns to eight K coefficients / 128 bytes,
consumed the checked hint, and recomputed under a destination backend while
retaining the original source owner. Wrong expected commitment, unavailable
Jolt auxiliary groups and wrong trace geometry were rejected. The temporary
driver was removed.

| Observed component, one run | Time |
|---|---:|
| Provisioning and selected setup | 2169.043 ms |
| Main commitment | 23.691 ms |

These are component timings, not a VM segment, accumulation update or recursive
parity comparison. Native all-target checks passed both with and without
field-inline; four engine-twin/curation cases and 20 native field-inline prover
unit cases passed. Two obsolete BlindFold shape-copy/golden-dimension audits
were removed rather than repinned; real acceptance tests remain.

Independent source review found no actionable capability/ownership defect.
The output-sequencing review identified an undeclared field parameter in stage 4;
it was corrected to the actual `PCS::EvalField`. That review found no additional
semantic defect, but neither review certifies a production registry, full
integration, concrete soundness or performance parity.


## 17. Reused commitment decoding and explicit native acceptance

The compact public setup now decodes Main through the existing K commitment
codec and resolved profile guard. It checks flavor, layout, stride, source
dimensions, polynomial count, coefficient count and exact payload length before
decoding. Existing single/grouped decoders share that guard; no opening point,
E-valued replacement commitment or regenerated key is used.

A release execution committed the original 56-column source, decoded its eight
K coefficients / 128 bytes, and compared the complete profile and coefficient
vector with the retained original group. The installed public allocation and
private source owner remained shared; releasing the last handle released that
source. This exercised commitment transport, not a segment proof or update.
The existing wire codec deliberately omits the prover's `RingVec` storage
dimension tag. A first execution incorrectly compared that non-wire tag too;
the corrected comparison follows the backend's existing public-commitment
validation contract. No production equality or wire representation was weakened.
The temporary driver was removed.

The inner native adapter requires `InnerExecutionRegistry`, in addition to
primitive schema/site admission. Pure preflight checks complete logical requests,
including public/proof counts, scalar/vector operations and empty vectors or
shell bundles. Only successful complete publication advances the native ledger.
Native closure checks its finite protocol accepting state; any failure poisons
the same compiler. Primitive cursor lookup and schema admission do not count as
execution. The original sampler, bytes and scoped compiler loan are reused.

A separate release execution completed a public record, a two-coordinate E2
draw and an empty coefficient vector through the retained native loan. It
observed three completed logical requests, closed the same compiler, and
rejected premature closure with sticky failure. Its finite component registry
does not certify an accumulation run. The temporary driver was removed.

Verification at this boundary:

- 46 Akita transcript unit cases and its strict all-target Clippy check passed,
  including inner and outer native acceptance.
- 126 Jolt transcript cases and 94 Jolt-Akita cases passed, including malformed
  source shape/count, trailing bytes and noncanonical K rejection.
- The earlier hidden/field-inline verifier and engine runs passed 89 and four
  cases respectively; the native full-ring source/component run passed eight
  cases with strict CPU/PCS Clippy.
- Claims/derive and committed sumcheck runs passed 169 and 66 cases across their
  suites; two producer capability compile-fail examples passed.

The aggregate Jolt strict check exposed two complex return signatures and a
test-only unchecked slice. Named borrowed result aliases and a checked test
mutation address them. Aggregate strict verification and additional PCS caller
checks remain pending at this point. The outer native adapter now also requires
explicit acceptance of the completed event count and the original borrowed
prefix. A production registry must still implement the finite installed protocol
and terminal obligations. Raw outer closure remains liveness-only.

## 18. Source-derived continued-consumer integration requirements

The reuse audit identified the following remaining changes; these are not
implemented by the ordinary `prepare` cut alone:

- Instantiate the existing stage-2 RAM output consumer only on the last
  segment. Preserve its real dummy rounds and derived address equality with
  RAM read/write. Bind original incoming/outgoing RAM evaluations before the
  existing stage-4 gamma; reconstruct global initialization only once.
- Reuse register value-evaluation kernels with the continued endpoint target
  and one independent gamma. Preserve incoming canonical K state, including
  untouched registers. Current zero-initial replay, address-first i128-only
  checkpoints and untouched sparse-column zeros cannot serve that contract.
- Supply the authenticated segment entry to the existing bytecode builders.
  Correct shift inputs at distinct outer/product anchors, and change actual
  successor delivery in materialized, streamed and random-access row paths.
  The `ShouldJump` witness depends on that successor too.
- Add the two original-Main PC moment reductions and the column-specific
  physical-zero relation. The active RAM address-zero cell remains allowed;
  omitted-family zero cells and inactive rows do not.
- Replace the selected Main's repeated generic Boolean-whole-grid prefix with
  explicit source-linked reuse of frontend Booleanity/digit-zero plus the new
  physical-zero obligations. Keep generic source-domain and RAM/receiver
  algorithms available where their contracts actually apply.

RAM reconstruction also currently starts from a whole-run image and imposes
whole-run final I/O. Its sparse u64 checkpoint representation does not itself
admit arbitrary canonical-K cut states. These source assumptions must change
together with the consumers; casting K state to u64 or changing only the
sumcheck target is not a solution.

Exact governing equations remain in
[continued-segment lowering](AKITA_INSTANCE_ACCUMULATION_SEGMENT_LOWERING.md),
[segment interface](AKITA_INSTANCE_ACCUMULATION_SEGMENT_INTERFACE.md) and
[Main source reuse](AKITA_INSTANCE_ACCUMULATION_MAIN_SOURCE_REUSE.md).
The production finite registry, authenticated full context, typed handoff,
continued frontend, full native update, archive guest, outer receipt and
same-semantics recursive parity remain unverified integration work.

## 19. Corrected stream rows and empty outer requests

Independent review found two integration defects after the earlier component
checks. A segment split at an unaligned active length delivered short raw
chunks to consumers that require complete PCS rows. The fused stream now
extracts with the correct fragment lookahead and reassembles full consumer
chunks using bounded reusable scratch. The original source grid resumes after
the split. Ordinary aligned deliveries and the final requested tail retain
their existing behavior.

The real Dory commitment/hint regression passed for dense and one-hot columns,
compact and physically padded sources, active lengths 3 and 5 in an eight-row
trace with width 4, reference consumers, forced optimized superchunks and the
selected optimized route. The successor-view regression and five existing
consumer tests also passed. These runs verify row delivery and commitment
equivalence, not boundary authentication or a native accumulation update.

An empty outer shell bundle previously bypassed logical metadata admission and
published no descriptor. The corrected compiler validates a logical
`ShellBundle` request before any coordinate, including count zero. Its tag 7
does not reuse reserved tag 6. Empty completion records the complete logical
descriptor and empty outputs without an oracle draw; nonempty keys/history
remain unchanged. The outer transcript contract now specifies this case.
The actual retained native loan rejects invalid family, sampler, tree, arity
and count metadata, premature closure and duplicate execution with sticky
failure. All 48 Akita transcript tests and its strict all-target Clippy command
passed after this correction. No production installed registry or end-to-end
receipt is certified by these component fixtures.
