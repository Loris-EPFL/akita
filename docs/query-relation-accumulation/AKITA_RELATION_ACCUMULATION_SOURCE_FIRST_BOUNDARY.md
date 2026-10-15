# Candidate1: ordinary source-first proof boundary and real admission

## Status and scope

Read-only review of actual ordinary Root/Fold/Suffix/Terminal APIs and their transcript consumers. No repository edits, builds, tests, linters, formatters, proof/control runs, branch/commit operations, or rerun of the known Jolt caller compile defect. Arithmetic below is derived from inspected static rows/ledgers, not newly exercised verification. The parent separately investigates the proposed mathematical source-first theorem; this report identifies its exact code contract and what existing admission does and does not certify.

Companion research:

- `AKITA_RELATION_ACCUMULATION_SOURCE_CERTIFICATES.md`: full original-source geometry, response-difference certificates, shell/unit conditions, terminal bounds, installed setup reference, profile contracts.
- `AKITA_RELATION_ACCUMULATION_SIS_RUNTIME_ADMISSION.md`: actual SIS keys/estimators and absence of full extractor-time admission.
- `AKITA_RELATION_ACCUMULATION_RAW_RAM_BRIDGE.md`: ordinary algebraic source relation versus original signed raw-RAM predicates; do not conflate these relations.

Local indexed Akita text (#bfa1a6, `Papers/parsed/Akita.md`) is not independently authenticated against the official PDF body. References to Theorem 10.23/Corollary 10.17/etc. are conditional paper statements, not a claim that a new theorem or implementation has been proved. `AKITA_INSTANCE_ACCUMULATION_FIELD_POLICY_COMPARISON.md:74–100`, especially line 87, does NOT impose an extra tight end-to-end 128-bit requirement. Require a meaningful finite explicit bound for the actual experiment, without silently adding that threshold.

## Executive answer

**There is a real noncircular descendant boundary:** ordinary `Cf` is transmitted before the current ring-switch/range anchors, and its complete suffix treats it as an incoming source. That suffix can supply its weak source/product certificate from its own deeper ranges, or the direct terminal base, without assuming the current parent's range predicate. This is not the same as a source already certified to be a short balanced digit table.

**This observation alone does not establish the lighter proof:** an outgoing commitment fixed before a challenge is not automatically an efficiently extractable witness fixed independently of that challenge. The source is extracted from a complete continuation that may depend on the current challenge. The proposed replacement of nested nonlinear interpolation trees needs a causal matched-continuation/source-coalescing argument with explicit failure/work bounds. Original shell forks still need full accepting descendants, certified primitive constraints, source/path comparison, and actual unit differences. Existing APIs implement verification, not a knowledge extractor or this source-first admission theorem.

**Existing numerical rows do not price the extractor:** table keys/ranks and estimator ROP prices cover collision geometry and modeled attack work. Local GrindingPlan losses and expanded logical query counts are not a full recursive extraction-tree/runtime or adversarial-ROM-query budget. Neither the historical Main56/RAM row nor the current Dense nv14 row makes that missing runtime-conditioned SIS guarantee available. This remains true if the new analysis substantially reduces the tree.

## 1. Exact runtime entry and acceptance boundary

`akita/crates/akita-prover/src/protocol/prove/root.rs::begin_batched_reduction` takes the expanded setup descriptor, prefix registry, trusted catalog, backend, selected opening data, transcript session, and basis. It resolves the selected row, validates every original handle/profile, binds the descriptor/session and original claims, prepares and proves exactly **one root fold**, and returns `PendingNativeProver`. It does not accept or finish the proof.

`PendingNativeProver::finish` drives all remaining suffix folds, the actual terminal, transcript-grinding completion, and backend proof-scope completion. `akita/crates/akita-verifier/src/fold/verify.rs::batched_verify` is the corresponding complete verifier. The private `verify_root` returns a still-pending outgoing commitment/point/value; it is not an authentication decision for that point.

`akita-types/src/transcript_grinding/replay.rs::VerifierGrinding::finish` requires the complete admitted plan to have been consumed and verifies transcript EOF. A locally successful root replay or completed Stage 2 is not `ProofAcceptance`.

There is no public or private ordinary source-knowledge extractor in these runtime APIs. A formal simulator/extractor may replay them, but its complete accepted continuations, transcript storage, source construction, collision construction, and ROM programming costs are additional proof obligations.

## 2. Precise ordinary fold message/challenge order

### Root preamble

1. Public original source commitments, source profiles, group points and supplied values are bound by `prove/root.rs` and verifier `fold/root.rs`.
2. `prover/protocol/ring_relation.rs::prepare_ring_relation` retains original commitment material, computes opening material, and sends the complete opening-side D/H payload. Payload lengths and group/native-role geometry are checked on the verifier by `fold/relation_instance.rs::validate_fold_payloads` **before** the shell.
3. `opening_claims::finalize_claims` draws `EvaluationBatch` row coefficients after those payloads. For k>1 these are **all k independent unnormalized coefficients**; for k=1 the sole coefficient is exactly one (`akita-types/src/opening_claims.rs::row_coefficients:258–273`). They are not powers of one scalar and are not a vector whose first component is forcibly one.
4. `prover/protocol/fold_grind.rs::select_fold_challenges` uses one level-wide 12-bit `FoldResponse` nonce, then independent ring draws for every group/claim/block. Off-sponge honest probes check planned response envelopes; the accepted nonce and all corresponding draws are replayed through the actual transcript. This honest rejection process is not an adversarial response-range proof.
5. Prover constructs the complete recursive witness Z/E/T/path/tail layout from those shell coordinates.

### Cf cut and strong checks

6. `prover/protocol/prove/fold/mod.rs::commit_next_witness` commits that witness under the frozen next-source parameters and sends `NextWitnessPayload` (ordinary outer commitment) or `NextWitnessInnerState` (the predecessor-bound terminal A state). This is **after the current shell** but **before** the current alpha/tau/range anchors.
7. `prover/protocol/ring_switch.rs::ring_switch_finalize` and verifier `stages/ring_switch.rs` draw alpha, the complete tau0 point, and the complete tau1 row point. These prepare the public relation and range image over the full committed Boolean domain.
8. Stage 1 verifies all required product-tree sumchecks and their child claims, draws interstage powers-batch scalars, and verifies the leaf range sumcheck. The final scalar is `range_image_evaluation = MLE[W(x)(W(x)+1)](u)` at a Stage-1 point u. It is **not** W(u), and off-cube it is generally not W(u)(W(u)+1).
9. If the selected A route is L2, physical response/norm linkage is proved and batched at the scheduled sites. Its verified integer energy bound is not an honest digit-moment estimate.
10. CompressionBinary and Stage2Batch scalars are drawn **after** the range helpers. Stage 2 performs its cubic fused-relation sumcheck and receives the scalar W(z).
11. If setup is offloaded, Stage 3 authenticates the deferred setup-weight evaluation under the installed prefix commitment. Stage-2 public-matrix validation waits for that result.
12. `verify_fold` returns `FoldVerifyOutput { next_witness: Cf, challenges: z, opening: W(z), setup_prefix_opening }`. `fold/suffix.rs` makes Cf the next fold's incoming witness commitment, preserving exactly the endpoint point/value and optional setup group.

Exact producer/consumer sections: `prover/protocol/prove/fold/mod.rs:34–305,483–568,581–820`; `prover/protocol/ring_relation.rs:122–478`; `prover/protocol/fold_grind.rs:225–361`; `verifier/fold/root.rs:1–176`; `verifier/fold/mod.rs:1–248`; `verifier/fold/suffix.rs:83–185,280–324`.

### Recursive differences

Root preparation does not run EOR and the actual verifier root requires coefficient packing. Suffix preparation explicitly dispatches packing, degree-one evaluation trace, or extension-claim reduction. Optional installed setup source and recursive witness are separate groups. A theoretical one-source shortcut must not silently discard the setup group, EOR, or exact native-ring dispatch.

The final terminal binds E partials before its shell and bounded Z after it, then directly checks A, gadget/opening consistency, scalar trace, and the selected Linf or squared-L2 response cap (`verifier/fold/terminal.rs:15–180`, `verifier/terminal/direct.rs:131–295`). This supplies the induction base; there is no fictitious terminal successor range proof.

## 3. Source-first dependency graph: what is and is not circular

For fold h:

- Incoming source(s) and their D/H opening-side payloads are fixed before h's shell.
- Cf is fixed only after that shell, but before h's alpha/tau/range/fused checks.
- Complete suffix h+1 sees Cf as its incoming source. Its local extraction uses the **h+1** shell and **h+1** response certificate, recursively supplied by h+2 and ultimately terminal direct checks.
- Therefore Cf comparison does **not** need h's current range predicate first. Once suffix extraction yields weak sources for two accepted continuations under the same Cf source projection, Corollary 10.17-style comparison can coalesce those sources or output a deeper A/B/F collision.
- With a causally fixed full decoded Cf source, h's own current range/fused checks can then establish the necessary shortness/range and exact row identities on that source. Those current predicates are what certify h's incoming weak sources after the original shell subtraction.

Two different purposes of ranges must stay separate: the deeper scan supplies a bounded **product certificate** for Cf; the current parent range constrains its actual decoded W table for current Z/path/binary/norm identities. Neither supplies raw-source/canonicality assertions absent from the ordinary relation.

The causality gap is real: suffix extraction is run after adaptive current challenges. C-fixed is not by itself source-fixed. A valid source-first argument needs a reference accepted continuation with the same pre-anchor prefix/Cf, obtains its complete source from the suffix, and proves that all matching successful continuations use the same commitment-side source except with a fully priced matrix collision. It must account for unsuccessful suffix extraction, adaptive choice of the selected oracle query, and the probability/work of obtaining the matching continuations. It cannot merely invoke a binding slogan or multiply independent black-box success estimates.

Cf may differ across the original shell children because it is produced after the shell. Coalescing the same Cf within current post-Cf probes is permissible under the source-comparison lemma; identifying different Cf across different shell children is not. Definition 10.22/Theorem 10.23 instead derives each branch's exact native relation independently and then subtracts its one-coordinate shell siblings.

This conditional induction is consistent with the existing code and indexed theorem. The **new lower-cost matched-probe theorem and its exact finite loss/runtime are not established by that consistency**.

## 4. Primitive constraints a fixed-source proof must preserve

### Full source versus endpoint versus helper slices

The suffix authenticates its incoming claim as an evaluation of its actual complete decoded source. The needed W* is that complete Cf source on the selected committed Boolean grid, not a single scalar endpoint, a cropped live prefix, an arbitrary honest constructor buffer, or the Stage-1 U helper.

`RelationRangeImagePlan::new` and `RelationAddressGeometry` validate contiguous group/chunk Z/E/T ranges, quotients, carrier/direct geometry, live extent, and complete power-of-two committed extent. The range image is computed as the MLE of pointwise **on-cube** W*(W*+1). Its product-tree intermediate tables have to be connected to that same W*. Existing intermediate product child claims are not individually committed sources.

`prover/fold/sumcheck.rs:156–470` and `verifier/stages/stage1.rs:1–298` preserve each product of child claims, each powers-batch consistency step, leaf polynomial evaluation, and the final U(u) helper. Source-first must recover these deterministic functions of W*, or explicitly price a first divergent helper. Discarding the helper tree while keeping only the leaf quartic does not prove the original range predicate.

### Fused Stage-2 relation

`verifier/stages/stage2.rs:1–472` checks the cubic endpoint combination containing:

- the current public ring/matrix relation and packing/trace opening row,
- quotient-lift or reduced-evaluation linear compression weights,
- the complete negative-binary support term eta*mask*W*(W*+1),
- optional ET scalar trace structure,
- optional physical-L2 response linkage,
- and beta*eq(u,z)*W*(W*+1), linking the Stage-1 range-image helper to the authenticated Cf endpoint.

The complete relation includes all native source groups, full shared D/H path, A/B/F compression chain digits, all required tail quotient/reduced rows, and every actual source-opening partial. Current producer relation order is not simply public RAM/RAM/Main order: own/final group and precommitted groups are arranged by the authenticated layout. No class may be omitted by assuming one displayed order is universal.

CompressionBinary/Stage2Batch are unrestricted field draws. The verifier retains mandatory constant relation/point terms when a batching scalar is zero; L2 virtual batches begin at eta rather than a constant. A proof must charge the exact zero/cancellation bad sets rather than invent rejection, inverse operations, or an assumption that all batch weights are nonzero.

### Setup reference

Offloaded setup is not an arbitrary second attacker-consistent witness. `fold/suffix.rs::suffix_commitment_payloads` obtains its commitment from the installed verifier registry keyed by exact natural length/profile. Source comparison must include a canonical installed reference occurrence rho_ref=1 and its actual A-input decomposition, giving mixed A radius beta_weak+kappa_weak*||s_ref||inf and the corresponding B/F path bounds.

Important code/paper convention difference: `akita-cpu-backend/src/setup/setup_prefix.rs::commit_validated_setup_prefix:98–144` commits the actual complete shared matrix prefix `[0..n_prefix]`, including beyond natural_len. The indexed paper Eq.(167) defines a zero-padded source beyond active length. Current `validate_setup_prefix_geometry` explicitly treats natural_len as weight support. Use the actual installed full-prefix source and prove zero **weights** outside active support; do not assume zero source entries there. Neither inspected numerical control below selects setup offload.

## 5. Exact source points, fields, basis and padding conditions

`PreparedSubringCoefficientPackingPoint::new` (`akita-types/src/subring_coefficient_packing.rs:39–134`) enforces nonzero live positions, power-of-two M, exact public point length `source_num_vars`, `source_num_vars <= expected`, and

`next_power_of_two(N_live) = ceil(2^source_num_vars / d_A)`.

It derives axes in order: low coefficient/embedding stride, carrier coefficient, position, block; it pads the preparation-only extra point axes with **zero**. All axis weights use the authenticated basis. Source group variables and role/carrier dimensions are not freely interchangeable. For the known Main/RAM root, both points have 16 coordinates, but Main has tail7+position9+block0 and RAM has tail7+position5+block4. This is why distinct group-point components must be retained.

Suffix rejects a point longer than recursive source arity, then the exact method-specific preparer checks the remaining geometry. Stage-2 z is a flat recursive-witness point, not the original Main/RAM point, and Stage-1 u is a different point for the virtual range image. Internal recursive witness openings use the propagated Lagrange basis; an original application basis choice does not permit substituting a different internal W polynomial.

`akita-sumcheck/src/single.rs::eq_factored_advance` performs an additive recurrence, with no division by eq/tau factors. Sumcheck/row/anchor draws may be zero, one or any field element. `grinded_ext_challenge` does not reject zero. Do not invent a nonzero-equality-factor admissibility condition.

EOR's `final_factors` are actual equality/extra-axis factors multiplying claims (`verifier/stages/opening_claims/extension_claim.rs:390–455`); they are not assumed one or inverted. If the selected suffix uses EOR, its exact point bridge, factors, and finite bad-set loss remain part of the source-first proof. Root packing does not magically authorize unsupported EOR or an uncertified extension field. The inspected controls are actual K/K branches; they do not certify the old native E2 prototype or another extension implementation.

Distinguish three kinds of padding:

1. Missing padded block addresses in the logical decoder/prepared query are structural virtual positions.
2. Physical unused positions/ring-tail coefficients inside a real committed source are honest-zero constructor inputs but ordinary weak extraction does not automatically prove them zero.
3. Range/fused proofs act on the exact full committed Boolean table and use public support masks/row weights. If the proposed argument needs a cropped zero-padded W rather than its full decoded source, it must prove that decoder/padding bridge; declarations and honest `pad_base_evals` are insufficient.

A valid proof may instead retain arbitrary padding entries in W* and show all needed row relations use only their certified supports. Either choice must be explicit. It must not rewrite the original RAM26 path to canonical decoded-field digits. The six virtual slots of a 32-slot raw digit query axis are not six stored raw RAM planes.

## 6. Root protocol counts and shell fork dimensions

There is one root protocol per ordinary grouped PCS proof, not one completed root per group/claim, and not 256 roots inside the proposed single-final batch. Original shell fork coordinates are one **ring element per claim/block**: W_total=sum_g(claims_g*B_g). Shell coefficient support weight is not the fork-coordinate count.

For the saved Main56 + two original RAM control (`AKITA_INSTANCE_ACCUMULATION_ORDINARY_CONTROL_LEDGER.txt:1–21`):

- Root groups are two RAMs (one logical polynomial and16 blocks each) plus Main56 (56 polynomials and1 block): shell coordinates16+16+56=88, global coordinate-wise flat arity89. Grinding ledger records group-local multiplicities17,17,57.
- Public scalar batch contains58 claims, not120 frontend emission leaves. It draws58 independent coefficients once; existing extraction-aware loss is58/|E|.
- Common Stage-1 base32 has one product stage of arity4, one quartic leaf stage, then Stage2 cubic: **three sumcheck protocols** at root, plus optional setup Stage3 (none selected here). Product/leaf eq-factored rounds have full degree5.
- Root alpha declared loss255; tau0 arity21, tau1 arity5. These dimensions belong to this selected root, not to a future all-256-source row.
- There are six nonterminal folds (levels0–5) followed by one direct terminal, not a bounded-state accumulator.

For current singleton Dense fp128 nv14 row (`akita/artifacts/schedules/fp128_dense.aks:128–134`):

- Actual root A dimension512, M4/B8, but shell carrier dimension64, packing k1/h8. Eight root ring coordinates yield flat arity9.
- Response/range base8: no product tree, one quartic leaf range sumcheck and one cubic Stage2, so **two root sumcheck protocols**, no setup Stage3. Later base16/base32 and selected-L2 routes have their own stage counts.
- Four nonterminal folds (root, next1–3), then direct terminal. Model proof cost, honest runtime, and recorded PCS bytes are not extraction-tree runtime.

For any future 256-history final batch, derive the ring-coordinate sum from the **actual selected grouped endpoint layout** (including source aliases, row-claim multiplicity and any setup groups). Do not multiply an old per-segment coordinate count or pretend distinct frontend leaves are already grouped PCS scalar claims.

## 7. Response-difference source certificates, not honest shortness

Let certified response digits be in `[-a/2,a/2-1]`, recomposition base b/depth delta, chunks m, raw shell mass omega. Then with S=(b^delta-1)/(b-1):

- accepted response max magnitude Z_inf=m*(a/2)*S;
- weak product `rho*s = r = z'-z` has beta=m*(a-1)*S;
- challenge-difference L1 is kappa<=2*omega;
- same-profile A collision radius is2*kappa*beta, mixed-profile radius is kappa'*beta+kappa*beta'.

In these inspected ordinary rows a=b, so beta=m*(b^delta-1). This does not make s short: inverses of rho need not be short. Frozen A table capacity, producer source norms, honest grinding cap, and accepted max |z| are four different numbers.

| Inspected occurrence | Response certified base/depth | beta (response diameter) | kappa | Same-profile A collision radius |
|---|---|---:|---:|---:|
| Original control root Main56 |32/2|1023|62|126852|
| Original control root each RAM26 |32/3|32767|62|4063108|
| Original control first Cf, scanned by next fold |32/3|32767|62|4063108|
| Dense nv14 root original source |8/7|2097151|102|427818804|
| Dense first Cf, scanned by next fold |16/3|4095|102|835380|

Original frozen root A capacity8126340 is an audited preserved capacity, **not** beta. Main's root accepted interval is[-528,495], RAM's[-16912,15855]; Dense root[-1198372,898779]. Main's honest deterministic envelope1736 and RAM's7936 are not weak certificates. All original RAM26 planes are charged: per-plane Linf16, per-ring L1=2048, energy envelope436207616. Main whole-grid UnitOneHot chunk1 has Linf1/L1=128 and energy3670016. Dense actual inner source is base8192/depth10, not nominal preset base8; its source Linf4096 and selected response depth7 must not be swapped.

Terminal exceptions:

- Original control terminal directly certifies |z|<=16383, hence beta32766, kappa62 and collision4062984<=4063108. Retained fold metadata base32/depth3 does not imply beta32767 for the direct terminal.
- Dense terminal is L2, squared response cap549103459/operator18, no independently selected Linf cap. It certifies weak Euclidean product <=2*sqrt(549103459), difference operator<=36, and diagonal squared collision11386209322624<=2^44. Treating its nominal digit depth as a Linf certificate would use the wrong admitted security route.

For a new Candidate1 row, only its **consuming** source/opening plan plus actual range/terminal constraints determine beta. Frozen original profiles alone do not determine it.

### Nonzero and unit conditions

`draw_group_fold_challenges` dispatches packing in S_s with the approved unital embedding into A, trace in ambient A, and rejects ordinary FullRingScalarExtension dispatch (`akita-types/src/proof/fold_challenges.rs:41–121`). Packing requires Linf and exact production config; selective-L2 trace applies its exact certified filter. Production64 has counts(31,10), omega51; production128(31,0), omega31; selective-L2 64(31,11), omega53/operator18.

For each fork rho must be a **nonzero unit**, not merely a nonzero element in a split ring. Identical draws have zero difference and cannot cancel. Actual q128 production unit premise is the LS18 partial splitting condition with ell4; `akita-challenges/src/config.rs:332–365,413–416` records the exact sufficient integer check `(2*c_max)^4*4^2 < q`, with max c_max2 and q mod16=9. This helper is in cfg(test); it was read, not run. Its static inequality4096<q and the locally cited theorem condition cover production shell differences, not short inverses. Packing's specific embedding preserves their units and norms. Filtered accepted support/cardinality, finite sampler law and exhaustion must still be priced separately; raw shell entropy alone is not a full ROM/extraction certificate.

## 8. Cheap replay before expensive matched-suffix extraction

A formal matched probe can first run the **complete ordinary verifier** on its proposed continuation and discard rejected probes before invoking an expensive suffix source extractor. This aligns with the actual acceptance boundary and may improve weighted failure costs. It does not authenticate source points by a root-only check.

The safe accounting must retain the original selected query/index and frozen pre-anchor prefix, exact Cf identity/profile, full conditional oracle-answer law including rejected/unused sampler bits, and consistency of programmed/replayed entries. It must count all screening verification work, unsuccessful probes, expensive matching suffix extractions, successful source/path comparison work, transcript copying/storage, and collision output work. Cheap failure is not free; expected work depends on the conditional success probability and the matched-probe theorem.

The current verifier contains no extractor hook or full-ROM-fork API. Reusing it as a screening predicate is an analysis/interface proposal, not an observed performance result. No new parity claim follows from nv14 PCS control timings or proof-byte metrics.

## 9. What real SIS/catalog admission currently certifies

Exact reusable APIs:

- `SisTableKey`, `ceil_supported_linf_bound`, `sis_table_key_for_linf_bound`, `min_secure_rank` (`akita-params/src/sis/ajtai_key/table.rs:213–301,337–360`) certify policy/digest/modulus/role/dimension/width/Linf bucket. L2 has its own squared-collision table.
- `InnerCommitMatrixParams::try_new` and other role constructors plus schedule audit certify shape/coverage and selected capacity. The shipped policy is `Quantum128BitADPS16`; do not describe a hypothetical nominal192 threshold as the actual row requirement.
- `FoldSchedule::sis_occurrences` enumerates scheduled A/B/D/F/H matrix occurrences and selected bounds. It is not collision-attempt multiplicity, a complete mixed weak/reference comparison-pair inventory, or extractor runtime.
- `estimate_schedule_security` estimates the minimum modeled ROP across those instances. Default estimator success probability0.99 is a model input, not an adversarial success guarantee at a given extractor time; `ProvenAboveTarget` is a lower bound, not an exact cost. `Adv(time)=time/2^bits` is not provided or justified by these APIs.
- `derive_transcript_grinding_plan_from_public_shape`, `transcript_grinding_cost_for_planner_edge`, and `transcript_instance_descriptor` bind the admitted local sites/order/extension cardinality. `GrindingRun` stores site/loss/grind/nonce/multiplicity; `TranscriptGrindingCost` stores honest nonce bounds and expanded logical query count. None stores complete extraction-tree/probe/runtime, adversarial Q, or runtime-conditioned SIS advantage.
- `independent_batch_loss_factor` explicitly prices existing adaptive coordinate-wise extraction as k/|E| with k+1 children. A source-first fixed-discrepancy1/|E| improvement requires its own proved causal fixed-source lemma; changing the commentary or count is not a theorem.
- `find_schedule_for_frozen_profiles` is now a real implemented planner entry (`akita-planner/src/planner.rs:792–936`), not merely an old design stub. It preserves validated originals and resolves a real complete suffix. Its existence does not certify a new extractor's runtime or add unrepresented Candidate1 FS grammar.

Static numerical controls expose:

- Original Main56/RAM control:561 expanded local logical queries,3786 nonce bits, max780 nonce bytes, modeled proof141280 bytes/setup559104 fields. No full tree/runtime/collision success price.
- Dense nv14 actual singleton row: model setup131072 fields/model proof65073 bytes, actual root geometry and later L2 caps. Historical separately verified proof-byte metrics are measurements of ordinary proof execution only, not a full extractor advantage bound.

The indexed paper itself distinguishes attack-time estimates from runtime-conditioned SIS success assumptions (e.g. lines3292–3322). Existing rows being table-admitted does not make every larger extractor against them sound. Conversely, absence of a tight128 end-to-end bound is not automatically a rejection of the user's slower meaningful finite experimental setting.

## 10. Narrow viable work products before claiming the lighter admission

1. **Exact per-fold source/decoder certificate:** specify actual Cf source projection and complete Boolean domain, scalar endpoint map, all Z/E/T/path spans and padding/support conventions. Include setup canonical reference if present. Reuse existing typed plans rather than inventing a new payload-derived source commitment.
2. **Causal matched-probe/source-coalescing theorem:** establish how a complete descendant extractor fixes Cf's source before applying each current challenge bound, retaining selected ROM query information even on failure. Cover point changes, adaptive continuations and different source handles. Output either the same full source or an explicitly bounded current/deeper matrix collision.
3. **Fixed-source strong-check theorem:** on that source, prove every range helper/product, ring-switch/row/matrix identity, scalar source-point row, negative-binary support and L2 linkage. Use actual polynomial degrees and unrestricted field draws; preserve EOR/setup branches and their errors. Do not import old nested-tree loss or a claimed cheaper1/|E| loss without its own argument.
4. **Flat original-shell extraction inventory:** exact claim/block coordinates, nonzero unit differences, per-consuming-profile beta/kappa (or L2 certificates), full source-path coalescing and mixed/reference collision radii. Shell children may have different Cf.
5. **Finite whole-extractor admission artifact:** for the actual selected row and experiment, explicitly record conditional success/failure, screening and costly probe counts, total ROM Q, source extraction/verification/comparison/transcript work, collision inventory/multiplicity, sampler/setup error, and a justified SIS advantage bound against that complete time. Existing ROP/table floors are inputs, not the finished admission.

No crypto cutover or cheaper-source-first acceptance follows until these reachable contracts and theorem/admission outputs are supplied. This report does not request an extra tight128 threshold, a source rewrite, a new RAM-plane commitment, a fallback planner, or rerunning the known broken Jolt caller.
