# Fixed-original-source continued frontend theorem (derived locally)

This theorem is about the actual continued transparent K frontend, not deterministic frontend acceptance, not PCS acceptance and not a supplied Akita theorem. All probability claims are conditional on the explicitly selected fresh challenge law, and all original source assignments are fixed independently of those fresh future answers.

## Precise application predicate

Let h bind the expected preprocessing/program, public I/O, field and full original source profiles/views, trace geometry, active length ell, segment identity/count, public start/end registers, first/last-active PC and external successor metadata. Let S contain one original algebraic A/B/F source explanation per distinct original `(shape,payload,role,view)`; repeated occurrences refer to the same explanation. S includes Main, incoming/outgoing RAM, initial memory-mapped advice sources if present, retained original program objects in committed mode, and FieldRdInc in field-inline mode. No D/H opening-local state or point is part of source equality.

Define the source-backed K witness w_h(S) by categorical semantic Main reconstruction, fixed lookup/program-table pullback, fused balanced increment decoding and the exact register/RAM prefix recurrences. The application predicate R_alg(h,w_h(S)) requires:

* physical Main inactive and omitted-zero-row support and all semantic Booleanity/Hamming equations;
* the selected per-row RV64/composed R1CS equations and all fixed lookup/operand equations, including the native canonical instruction-address gamma^3 coefficient;
* exact first/last-active categorical PCs and expected start/last/successor program lane values;
* register values `Reg(r,t)=Reg_in(r)+sum_(s<t) RdWa(r,s)*RdInc(s)` and outgoing values at ell;
* RAM values `Mem(a,t)=M_-(a)+sum_(s<t) RamRa(a,s)*RamInc(s)` and `M_+(a)=M_-(a)+sum_(s<ell) RamRa(a,s)*RamInc(s)` on the COMPLETE decoded RAM domain;
* first-segment decoded initialization from public program/IO and present original advice/image sources; only last segment has the native public output/panic/termination relation;
* the actual public geometry/order/counter/adjacency constraints.

This predicate is defined over decoded K state. Adding a claim about every RAM cell being a u64 byte word, raw RAM planes being balanced, exact SDK tape length/cursor or host compute_advice execution selects a strictly stronger predicate and requires its own bridge. Source masks and raw plane bounds must not be silently dropped from any theorem that explicitly includes them.

## Local theorem

Let V_FE include actual continued stages1,2,3,4,5,6a,6b,7, native stage8 object selectors, current `verify_segment_main` and, if selected, the 118-to-56 Main normalization. Let I be its COMPLETE derived original-source scalar leaf roster: every canonical Main component, both decoded RAM endpoints, every auxiliary object, every committed-program boundary leaf, and every source-check/normalization terminal component not soundly reduced away.

For any fixed S, fixed prechallenge h and causal prover strategy,

`Pr[V_FE accepts AND forall i in I: OpenRel_i(S) AND NOT R_alg(h,w_h(S))] <= min(1, epsilon_spine + epsilon_prefix_objects + epsilon_Main + epsilon_normalization)`.

Here epsilon_spine is the explicit first-false residual-degree ledger for stages1..7, including initial/reference tests, uniskips, powers separators and sumcheck rounds; epsilon_prefix_objects is the sum of actual singleton selector-coordinate masses. For selected Main n12,k4, epsilon_Main<=67*mu_max. If existing normalization is retained, epsilon_normalization<=45*mu_max, otherwise0 and all118 source claims remain individually in I.

The numerical shorthand uses a common certified fresh scalar maximum atom mu_max. The sharper version sums each actual degree times that scalar's conditional maximum atom. Independent-vector linear separators cost one pivot-coordinate atom, not vector length times that atom. A powers separator costs its maximum exponent times its scalar atom. Multilinear random-reference tests charge each referenced coordinate's own exposure, even if those coordinates came from an earlier stage's sumcheck. When a reference is reused, account at that original draw; do not add an imaginary later independent reference.

## Proof, including adaptive later messages

1. Construct EVERY true virtual execution table over the Boolean domain from S and h, before interpreting any protocol claim. Products, shifts, lookup contractions and prefix sums are computed on cube rows and then multilinearly interpolated. Intermediate partial sums may depend on already exposed coordinates but are deterministic reductions of these same tables; they are not new free source witnesses.
2. Starting from true original leaves, traverse the actual reduction DAG backwards. Exact point/value aliases and challenge-free source/domain/context checks preserve truth deterministically.
3. For a sumcheck round with a false incoming partial-sum claim, the sent degree-d polynomial cannot equal the source-backed true partial-sum polynomial because their sums at0 and1 differ. The sent message is fixed before that round's challenge. Truth can resume only at a root of a nonzero degree<=d difference. Thus its conditional probability is <=d*mu. Compression supplies the missing linear term from the prior claim; this still fixes a polynomial satisfying the round-sum equation before the challenge (`jolt-sumcheck/src/verifier.rs:90-151`). No extra 'compression error' is introduced.
4. For independent batching, all input residuals are fixed before its coefficient vector; a nonzero residual coordinate yields a nonzero affine function of that coordinate. For powers batching, use the entire residual polynomial in gamma, including Booleanity's EVEN powers and native gamma^3 canonicality, rather than divide a sampled coefficient. With nonzero public powers-of-two padding in this odd characteristic, source-backed shorter-member identities lift exactly to the batch domain (`jolt-sumcheck/src/batch.rs:15-85`).
5. Initial equality/reference tests are fixed nonzero multilinear or centered-Lagrange residual polynomials in their ancestor coordinates. Coefficient induction bounds their roots by the sum of coordinate degree*maximum-atom terms. This remains uniform over messages sent later; the residual is determined by fixed source tables, not chosen after the coordinate is exposed.
6. If some R_alg identity is false but all final leaves and direct checks are true, the backwards traversal finds one of these first-false-to-true events. Union-bound the actual exposure ledger. No deterministic single-transcript acceptance implication or accepting-tree knowledge axiom appears.

## Challenge-law specialization and noninteractive limitation

K here has q=2^128-0xFFFFA7F7 (`jolt-field/src/solinas/mod.rs:156`). The actual legacy Akita frontend alias is `LegacyBlake2bTranscript<AkitaField>` (`jolt-prover/src/akita/preprocessing.rs:26-27`), whose raw and coefficient challenges both consume16 bytes (`jolt-transcript/src/digest.rs:190-201`); scalar decoding is modular reduction (`jolt-field/src/solinas/fp128.rs:1128-1137,1169-1172`). Under FRESH independent ideal 128-bit answers, maximum atom is exactly2/2^128 for residues below the offset and1/2^128 elsewhere. Therefore a common degree ledger D yields `epsilon_FE<=2D/2^128`, not D/q by default.

The D1 reserved rejection sampler `jolt-transcript/src/d1/outer.rs:367-390,738-790` uses disjoint fixed128-bit candidate spans. In the selected ideal independent BitTape model, conditional on success each accepted scalar is uniform K, independently across its reserved spans; per-coordinate exhaustion probability is `(0xFFFFA7F7/2^128)^attempts`. Exhaustion rejects. This is not a claim that a concrete SHAKE expansion of a finite seed has exact arbitrary-length independent-field law. Instantiation must supply the actual indexed raw-answer/reset grammar; the fixed-source degree proof cannot manufacture that contract. Retained ordinary PCS unit-difference shell forks require their separately exact conditional full-answer law, not this scalar maximum-atom argument.

## Arbitrary old accumulators and causal composition

Nothing in this local theorem assumes an adversarial old query accumulator is valid. A canonical append preserves each old scalar coefficient and original source identity, so a new strong query witness restricts to an old strong query witness. It does not retrospectively prove an old segment's application relation. The final proof must retain and replay the complete old FE evidence and charge its own historical FE errors, check expected initial/final/count/order, and discharge every old and new scalar coefficient.

For actual extracted-after-challenge sources, the fixed-source theorem alone is insufficient. The source-probe construction may instantiate this local certificate only after matched probes coalesce the SAME original Main/RAM/advice/program projections or return their certified A/B/F collisions. The frontend composition adds its original-query ownership/old-descendant terms and bounded extraction runtime. It must not substitute decoded equality for original source equality, assume C-fixed implies S-fixed, or price coalescence from a pending parent source range. No extra tight end-to-end128 policy is asserted.
