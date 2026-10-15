# Akita accumulation: field policy and final receipt comparison

## 1. Status and decision boundary

This is a research comparison, not a selected replacement profile, implemented accumulator, numerical receipt-security certificate, or performance result. It reopens the field choice in the [extension migration note](AKITA_INSTANCE_ACCUMULATION_EXTENSION_MIGRATION.md#35-why-e2-was-selected-and-what-has-not-been-proved-necessary). Further broad E2 migration remains paused.

The completed independent investigations separated two questions: whether degree one is algebraically valid, and what the actual security policy certifies. Neither found that lattice commitments or accumulation intrinsically require a nontrivial scalar extension. A separate mathematical review conditionally admitted the localized-E2 alternative in §5; it did not select it or certify an implementation.

Retain the full objective: pre-PCS segment-instance accumulation, original source ownership and continued-execution semantics, a succinct final lattice-based receipt, and measured end-to-end parity against a matching full-PCS recursive control. A symbolic comparison is not that measurement.

## 2. What original Akita uses

Let
\[
K=\mathbb F_q,\qquad q=2^{128}-2^{32}+22537,
\qquad S=K[Y]/(Y^{128}+1).
\]

Three objects have different purposes:

- K contains the source coefficients and public lattice matrices.
- Short ring challenges belong to a norm-controlled subset of S or a registered challenge subring. Their support, unit differences and integer norms matter.
- A scalar field E supplies polynomial-testing and evaluation challenges. It may equal K.

[Akita §3.1](SOURCE_PROVENANCE.md#akita), retrieved through qmd at lines 867–903, explicitly permits extension degree one and identifies its coefficient-packing map as
\[
\psi(a_0,\ldots,a_{d-1})=\sum_j a_jY^j.
\]
The partially split cyclotomic ring is not itself a field. Neither treating it as one nor replacing short ring challenges by arbitrary uniform field elements is justified.

At Akita commit `83574331d4e51f8cce1d8689d05592fa4ef4c138`, the [fp128 presets](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-config/src/proof_optimized/fp128.rs) use `(Field, Field)`, while fp64 uses `Ext2<Field>` and fp32 uses `FpExt4<Field>`. The existing fp128 Jolt frontend and ordinary Akita adapter therefore use K, not a nontrivial extension.

## 3. Genuine degree-one specialization of the compact accumulator

Hold the [compact profile](AKITA_INSTANCE_ACCUMULATION_COMPACT_PROFILE.md)'s source prime, original source geometries, base-32 decoders, all-valid-source bounds, shell and continuation relation fixed. Setting E=K makes its full carrier exactly S, rather than introducing another ring.

For each original source block, retain
\[
e_{p,i}=L_{\rm raw}(s_{p,i}),\qquad
z=\sum_{p,i}c_{p,i}s_{p,i},
\]
\[
Az=\sum_{p,i}c_{p,i}t_{p,i},\qquad
L_{\rm raw}(z)=\sum_{p,i}c_{p,i}e_{p,i}.
\]
Here the response and opening gadgets remain implicit in the undecomposed notation. Every original A/B/F equation and the shared D/H checks still applies. S-linearity is sufficient; S need not be a field. Fork cancellation still requires the unchanged unit-difference certificate. No small norm for the inverse of a challenge difference is assumed.

Only the partial planes shrink. For extension degree k, the group allocations in K cells are
\[
Z=M\delta_{\rm com}\delta_f128,\qquad
E_g=PB\cdot26\cdot k\cdot128,\qquad
T_g=PBn_A\cdot26\cdot128.
\]

| Fixed-geometry quantity | E=K | E=K[U]/(U²−5) |
|---|---:|---:|
| K coefficients per partial | 128 | 256 |
| Live root successor cells | 3155968 | 3555328 |
| Receiver capacity | 4194304 | 4194304 |
| Required root zero suffix | 1038336 | 638976 |
| Root D width | 3120 | 6240 |
| Exact coefficient rows | 2940 | 3456 |
| Padded coefficient-row capacity | 4096 | 4096 |
| First final-fold live successor cells | 1093632 | 1200128 |
| Second final-fold live successor cells | 643072 | 696320 |
| Third final-fold live successor cells | 446464 | 499712 |

These counts were independently derived and checked with integer arithmetic. They are not elapsed times or serialized proof sizes. The three outgoing final capacities remain 2^21, 2^20 and 2^19. At degree one, even the fresh Main/RAM/path allocations without an old receiver total 2099200 cells, already above 2^21; deleting a partial plane is not a factor-two receiver reduction.

For k=1, native `OpeningFoldPlan::Base`, identity-source suffix evaluation/folding, and direct terminal ring/scalar checks match the required ordinary maps. The terminal retains eight full S partials, three A rows, the response norm bound 7168, and the exact scalar recombination. Its predecessor F/H paths are not removed. The native checked Golomb response codec must not be reported as the compact note's proposed fixed signed-i16 codec.

This reuse does **not** discharge raw delta-26 RAM queries, original noncanonical source/path retention, digit-point normalization, the exact quotient-free coefficient-row compiler, full strong checks, source uniqueness or continued-segment ownership. A new degree-one profile also needs its own D widths, layouts, transcript encoding and certificates; frozen E2 objects cannot be reinterpreted.

## 4. Statistical error, grinding work and baseline policy

For exact-uniform K scalar and shell sampling and a declared classical oracle-query budget Q, the compact degree-one inner ledger has the symbolic form
\[
B_A(Q,T)=(Q+1)\left(\frac{C_A(T)}q+
\frac{W_A(T)}{|\mathcal C|}\right),
\]
where
\[
|\mathcal C|=\binom{128}{40}\binom{40}{24}2^{40}.
\]
Setup deviation, runtime-priced MSIS reductions and receipt composition remain separate. Degree two changes the scalar denominator to q², not the shell size or these other obligations.

The previous illustrative caps `Q+1 <= 2^32`, `c_E <= 2^40` give a K scalar allowance approximately 2^-56. The tighter conditional whole-archive arithmetic ceiling below 2^26 gives an allowance below `2^58/q`, approximately 2^-70. Neither is an attack, measured security level, exact emitted manifest, or proof that the user must accept E2. The [composed policy](AKITA_INSTANCE_ACCUMULATION_COMPOSED_POLICY.md) explicitly does not impose a tight end-to-end 128-bit reduction as an extra requirement.

Original native grinding is real but has a different theorem. The inspected [exact-cardinality implementation](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-params/src/transcript_grinding/challenge_order.rs) chooses the least supported g satisfying
\[
2^g|E|\ge 2^{128}L,
\]
with g at most 25. For fp128, q is slightly below 2^128: L=1 needs g=1, not zero. The paper's nominal-capacity prose is not this exact implementation rule.

[Akita Proposition 10.3 and Corollary 10.4](SOURCE_PROVENANCE.md#akita), qmd lines 3431–3451, bound eligible local bad events by
\[
\Pr[\text{some protected bad event}]
\le \min\!\left(1,\sum_s(N_s+1)2^{-g_s}\epsilon_s\right).
\]
The bad set must be fixed before either candidate output is first queried. The paper explicitly states that this does not rescale its entire mixed-extraction-tree knowledge error. Response retries are separate. The native logical-plan query ceiling is not an adversarial Q bound.

For a matching full-PCS control, use the same application, continued-segment relation, source handles, public statement, supported depth, sampler law and adversarial resource convention. Write
\[
C_A=F+U_A+D_A,\qquad C_R=F+\sum_{i=1}^T P_i,
\]
where F counts genuinely common frontend/continuation checks, P_i includes each complete ordinary PCS opening/suffix/terminal, and the accumulator-only terms must not be charged to the control. Use each path's actual shell catalogue. Neither an independent-job conjunction nor a backend-only grinding target establishes this comparison.

The [frontend manifest §4](AKITA_INSTANCE_ACCUMULATION_FRONTEND_MANIFEST.md) also matters: the inspected legacy frontend reduces 16 random bytes modulo q; it is not exactly uniform K. Its maximum atom is `2/2^128`, giving a degree-D local bad-set bound at most `2D/2^128`. This observation alone is not a mixed-tree compiler theorem. Alternatively, use the existing bounded exact sampler specialized to K on both compared paths, with an explicitly changed transcript/profile. Keeping frontend algebra over K does not imply preserving every legacy transcript byte.

## 5. Conditionally reviewed alternative: E2 only inside new accumulation checks

Broad E2 migration is not the only way to reduce added scalar error. A separate candidate keeps the original frontend and ordinary outer proof over K while using E2 only for the new accumulator checks.

Its proposed handoff is exact: embed each ordinary point/value `(r,y)` into `(iota(r),iota(y))`, where `iota:K -> E2` is the canonical field inclusion. For a K-coefficient polynomial f,
\[
f_{E2}(\iota(r))=\iota(f_K(r)).
\]
Do not change the underlying source, commitment, decoder or original physical coordinates. Normalize these lifted claims together with the new E2 raw-domain endpoints and the prior E2 pending receiver claim. Keep the reviewed full carrier for this accumulator. The final ordinary K/K Jolt/Akita proof executes the E2 verifier using K-coordinate arithmetic; this does not require its own frontend challenges to be in E2.

Under exact samplers, the candidate ledger must retain the general form
\[
B_{A,\rm mixed}=(Q+1)
\left(\frac{F+A_K}{q}+\frac{A_E}{q^2}+S_A\right),
\qquad
B_R=(Q+1)\left(\frac{F+P_R}{q}+S_R\right).
\]
The objective would be to place new accumulator-only scalar checks in A_E, but A_K may be set to zero only after checking the actual manifest. The independent mathematical review found no algebraically forced accumulator-only K scalar test. Source/setup coefficients, fixed gadget points, canonical encodings and integer norm checks do not themselves draw K challenges. However, a retained K physical-main-hole or endpoint test belongs in F only if it is genuinely common to both paths; otherwise retain its A_K charge or explicitly move that added source-domain test to E2. Neither the common K frontend error nor the outer K proof error disappears. The previous catalogue-wide “new-work” subtotal includes some continued-frontend work and must not be moved wholesale to the q^-2 term.

The complete frontend context must cross the handoff. A 16-byte K session seed cannot be relabeled as a full-context E2 transcript. This is a binding/extraction-composition issue, not a claim that an independent backend ideal oracle cannot produce uniform E2 outputs from a short seed. New sampler, role, coordinate and field identities must be bound before challenges.

The independent review specialized Akita's heterogeneous-coordinate tree framework to the mixed fields: exact-uniform K and E2 change the respective denominators, not the algebraic interpolation arities. The K-to-E2 equality, raw normalization, original-source uniqueness and full two-plane final decider remain compatible. The degree-one frontend does not turn the accumulator's E2-valued pending opening into a K claim or remove a partial plane.

For further comparison, use the existing bounded exact sampler specialized to K on **both** candidate and matching control. Do not add legacy raw-word tree machinery merely for byte compatibility that was not requested. This is a sampler/transcript choice, not an alteration of K frontend arithmetic. It still needs the exact complete-context and conditional-tape certificate.

This alternative avoids a mathematical requirement for frontend-wide E2 arithmetic or an E2 ordinary-outer PCS. Its actual source/API integration, mixed manifest and ordinary K/K outer schedule remain uninstantiated. No source cutover is authorized by this section.

## 6. A linear archive and one succinct lattice receipt

An O(T) archive is compatible with a succinct final receipt when one outer argument proves knowledge of an archive accepted by the complete native verifier and final decider. Archive bytes remain witness/advice rather than linear-size public input. This does not assert zero knowledge of the current backend or constant total prover memory.

The intended outer backend remains Jolt with Akita PCS. A Groth16-only acceptance path would lose post-quantum soundness: [Groth16](https://eprint.iacr.org/2016/260) is pairing-based, and [Shor's algorithm](https://arxiv.org/abs/quant-ph/9508027) solves its underlying discrete-logarithm problem in quantum polynomial time. An inner lattice certificate does not protect a verifier that checks only a forged outer proof. Keeping that certificate available is not equivalent to verifying it.

The lattice route still has an explicit qualification: [Akita §10](SOURCE_PROVENANCE.md#akita), qmd lines 3363–3367, proves noninteractive knowledge soundness in classical ROM and leaves QROM analysis open. Quantum lattice-attack sizing does not fill that gap. The [D1 receipt theorem §7](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) is conditional and needs outer extraction/composition, not merely language soundness. No established end-to-end post-quantum theorem or numerical receipt bound is claimed here.

[Nova §1.2](SOURCE_PROVENANCE.md#nova), qmd lines 35–51, illustrates separating folding from final compression, but its precompression IVC witness is sized by the step circuit, not T. It is not evidence that the new Akita archive is constant-sized or that Nova's group assumptions transfer to this construction.

### Exact wrapper relation and cost boundary

Let s bind the approved setup/profile, program, public input/output, initial/final machine states and update count T. Let w contain the ordered update archive, its pending states and the final decider certificate. The wrapper must prove knowledge for
\[
R_{\rm archive}(s,w)=
\operatorname{Init}(s,a_0)
\land\bigwedge_{i=1}^{T}
\left[
 \operatorname{VerifyUpdate}(s,i,a_{i-1},u_i,a_i)
 \land \operatorname{Continuity}(u_{i-1},u_i)
\right]
\land\operatorname{Decide}(a_T,\pi_{\rm dec})
\land\operatorname{BindEndpoints}(s,w).
\]
The first update uses the declared initial boundary rather than an undefined previous update. The guest must also enforce canonical parsing, declared counts, complete input consumption, successful output and `panic=false`. A sequence of accepted update transcripts without the final decider is not acceptance.

The archive is an O(T) witness for a fixed per-update profile, not O(T) public input. The wrapper's work includes
\[
C_{\rm guest}(T)=T\,C_{\rm update\ verifier}
 + C_{\rm final\ decider}+C_{\rm parsing/binding}(T).
\]
The receipt can be succinct in this guest size; neither constant proof size nor constant wrapper-prover work follows. This route moves linear verification into one final proof. It avoids complete per-segment Akita PCS proofs only if updates stop at the pending relation as specified; wrapping a list of completed segment proofs would be a different construction.

**Composition requirement:** prove knowledge of an accepted archive under the actual joint setup/transcript model and then apply the native extraction argument. Merely invoking generic outer language soundness does not discharge the existing inner-oracle and composed-resource obligations. Nova's separation between folding and final compression supports the architecture, not the transfer of its commitment algebra or its step-independent witness bound.

If external acceptance checks **only Groth16**, it inherits Groth16's non-PQ assumption even if w describes lattice checks. Retaining w or its digest elsewhere does not restore PQ acceptance. Requiring an independently sound PQ receipt as well could preserve that receipt's guarantee, but a Groth16 copy would then be optional, not its security replacement. The selected route instead uses one ordinary Jolt/Akita final proof; its QROM and composed-extraction qualifications remain explicit.


## 7. Actual branch evidence and remaining gates

The existing receipt source map describes its selected Jolt0791/Akita835 base, not every later branch. Two additional immutable source inspections refine that map:

- Jolt `wrap/fast-recursion-blake` at `5c65167e269fdfc2917ecb82f420d349b18b4c87` has a real q128 outer prove/verify/replay runner in `examples/recursion/src/outer.rs`. It is reusable orchestration evidence, not a matching continued-segment control or a performance result. External acceptance must still enforce the intended public statement and success result.
- Jolt `feat/fast-recursion` at `1af59c3894e37caa801863336d52620d2d7dd402` has real Akita inner proof generation, prepared setup payload handling and Akita guest verification. However, its `run_recursion_proof` Akita `RunConfig::Prove` arm explicitly rejects with `packed recursion supports trace-only runs`. Its real Dory outer proving branch must not be relabeled as an Akita receipt.

The control-source investigation also found a concrete reuse boundary: the shipped k16 Main row `(16,56)` uses dA256/M128/B2, while shipped dense-full `(16,1)` uses dA512/M8/B16/delta13. Neither is the compact source geometry in §3. The stock grouped registry permits at most one full-width auxiliary and takes producer profiles from its catalog, not arbitrary frozen compact descriptors. An exact-profile ordinary control therefore needs explicit admitted profiles and a complete suffix, not a stock row relabeled by arity.

The stock native backend uses one shared flat matrix, while the prior compact registry specifies independent role matrices. The [shared-prefix review](AKITA_INSTANCE_ACCUMULATION_SHARED_SETUP_REVIEW.md) now conditionally admits a fresh native shared installation using Akita Lemma 10.11 and Corollary 10.12: 33 semantic roles become 12 numerical views in either fixed field profile, with maximum used prefixes 798720 coefficients for E2 and 559104 for K1. This does not reinterpret existing independently keyed sources or admit the control/outer catalogues. The [native-reuse revision](AKITA_INSTANCE_ACCUMULATION_NATIVE_REUSE_PROFILE.md) records exact setup, source, transcript and integration conditions.

The control investigation corrected an unnecessary restriction: equal source/security policy does not require an identical consuming shell. Ordinary d128 `(31,0)` fits the same frozen source capacities while accumulation uses `(24,16)`. The [frozen-profile planner](AKITA_INSTANCE_ACCUMULATION_FROZEN_CONTROL_PLANNER.md) has now generated and admitted a complete ordinary row and emitted its scalar/shell ledger. Original-plane runtime execution and total receipt dominance remain unestablished; local ledger inequalities and offline payload estimates are not end-to-end performance measurements.

Before selecting a field boundary or claiming parity:

1. Instantiate the conditionally reviewed localized-field or degree-one boundary, including the actual native opening and setup-view contracts.
2. Instantiate the same-semantics ordinary control's original opening geometry and complete scalar/shell ledger; retain legacy-sampler differences or select the same admitted sampler on both sides.
3. Freeze the chosen supported-depth and query/resource policy. Account for each distinct outer workload and its derived inner-forger resources; do not inherit the compact inner Q bound.
4. Generate the selected profiles and complete the actual native/archive/guest implementation. Reuse original kernels where their full contracts match.
5. Measure complete accumulation/update/final-wrapper work against complete ordinary segment proofs plus their matching wrapper. A smaller receiver allocation or a single field microbenchmark is not end-to-end parity.
