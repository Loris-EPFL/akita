# Native Akita instance accumulation: reduction boundary and protocol design

## 1. Selected target and status

**Selected target:** accumulate consecutive zkVM segment **instances and their pending relations**, before producing a full per-segment Akita PCS proof. Reuse Akita's native lattice commitment, witness construction and recursive reductions where their exact interfaces permit this. Produce a final proof/decision for the accumulated relation.

This is **not** accumulation of completed segment proofs, a batch of their terminal verifier residuals, a hash/Merkle accumulator, or a Nova construction with Akita substituted for a homomorphic commitment. The earlier [Akita-LVA specification](AKITA_ACCUMULATION_PROTOCOL.md) and its [review](AKITA_ACCUMULATION_PROTOCOL_REVIEW.md) remain separate, unselected research.

**Native-core status:** independent final review admits the finite \(T\le16\) archive construction under its stated classical-ROM/runtime-sensitive Module-SIS assumptions. Source descent, exact segment consumers, physical coverage, receiver, terminal, manifest and all 33 roles are reviewed; \(3555328\) live cells fit \(2^{22}\). This does not by itself admit a concrete succinct receipt. No execution, parity, ZK, QROM, unlimited-depth or overall concrete-128-bit claim is made.

**Subsequent user authorization:** the user selected **Allow a heuristic prototype** under an explicitly named, protocol-specific concrete-ARO instantiation heuristic. This permits experimental research toward implementation, not a claim of concrete soundness or a waiver of guest/source/profile/parity checks. It is not blanket approval of the earlier CH-I/CH-O hypotheses; any restricted implication must be stated explicitly in the revised model.

**Current receipt research:** the [D1 direct model](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md), [bounded guest](AKITA_INSTANCE_ACCUMULATION_GUEST_SPEC.md), [outer transcript](AKITA_INSTANCE_ACCUMULATION_OUTER_TRANSCRIPT.md) and [full-ring outer carrier](AKITA_INSTANCE_ACCUMULATION_OUTER_CARRIER.md) have independent and integrated design approval in [the prototype review](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_REVIEW.md). Implementation of this explicitly heuristic prototype is permitted; acceptance still requires generated source/ISA/profile certificates and complete execution/performance verification. D1 computes the fixed public inner transcript in the guest and removes graph/quotient/AS/C_t and their E4 requirement. Its native-game heuristic contains restricted CH-I content, and concrete outer SHAKE has a separate extraction-use heuristic; both have unknown losses. The [migration map](AKITA_INSTANCE_ACCUMULATION_EXTENSION_MIGRATION.md) preserves K sources and E2 results. The [supplied-answer receipt](AKITA_INSTANCE_ACCUMULATION_RECEIPT.md) remains separate ideal-model research.

The invariant is: every live instance names the **same original commitment/source projection** whose relation is being reduced. A pending result is never exposed as successful verification. Honest transcript agreement alone cannot detect missing constraints.

## 2. Why hashes appear in a lattice proof

A lattice commitment and a Fiat–Shamir transcript solve different problems.

| Role | Present in inspected Akita/Jolt | Required interpretation |
|---|---|---|
| Commitment to a source polynomial | Native A/B/F chain, with D/H opening material | Lattice-based; preserve its exact maps, source bounds and Module-SIS collision inventory |
| Interactive verifier challenges | Public random coins in the interactive protocol | No hash is needed for an interactive experiment |
| Noninteractive challenges | Fiat–Shamir transcript, sparse challenge streams and grinding | Hash/XOF use does not turn the commitment into a hash-based commitment; retain the declared ROM/XOF assumptions and query budget |
| Program, layout, catalogue or setup identity | Authenticated descriptors and, at some interfaces, digests | Identity/binding mechanism; a supplied digest alone does not authenticate an arbitrary setup or program |
| Previous Akita-LVA weights | Additional full-frame hash in the unselected public protocol | Not part of the selected native instance design |
| Merkle RAM or code-proximity accumulation | Not selected here | Do not import WARP/ARC distance or local-opening assumptions into Akita |

No new accumulation-weight hash is proposed here. A native update must still bind its predecessor, fresh instance, program/profile, VM boundaries, source roles and complete reduction messages before the native challenges that depend on them. Existing transcript code is reusable only after its grammar is extended and reviewed; splicing two transcript states is not an admissible protocol operation.

A hash-free **interactive** version is possible at the level of public-coin challenge generation. Removing Fiat–Shamir while retaining noninteractivity requires a separately justified construction, not deletion of hash calls. Neither lattice assumptions nor classical-ROM soundness alone establish QROM or concrete-hash security.

## 3. Source revisions and what has actually been read

The following source generations are distinct:

- **Previously inspected Akita tree:** `2607265b21f4726954461869f75acc2a8ab6ada1`. Native producer `protocol/prove`, verifier `fold`, opaque backend ownership and finite-schedule admission were inspected.
- **Previously inspected Jolt tree:** `629ed77b5c999fb2f52e5781b2b98a04ed6cf5ed`. Its manifest pins Akita `e2c49ed450f1999a743e7fd4a648f230ce45472f`, not that local Akita head. Its pre-PCS stage assembly was interface evidence, not a compatible combined build.
- **Selected implementation base pair:** Jolt `0791f79a55c3fadc3d327c78a72228696c338468`, Akita `83574331d4e51f8cce1d8689d05592fa4ef4c138`. The immutable companion producer root has the reviewed root-to-suffix boundary. This is the reviewed source/API base, not already implemented accumulation.
- **Existing lattice outer wrapper pair:** Jolt `5c65167e269fdfc2917ecb82f420d349b18b4c87`, companion `markosg04/akita@d0ee56a0b12db66b4c46a87c527169737694c4c7`. Actual Git objects, runner, grouped PCS calls and legacy terminal were inspected. Its aggregation relation is completed independent-proof conjunction, not continued segments. Its prover modules are under `protocol/core`, not the modern `protocol/prove` paths.

Use the selected modern pair for implementation. The old wrapper pair supplies **outer delivery** evidence only; its whole-proof independent-job relation needs the common continued-segment frontend before use as the control. Do not port API names, source contracts, transcript epochs or schedule bytes between pins by assumption.

After final design review, clean local branches named `akita-instance-accumulation` were created in both repositories from the selected exact commits. Both post-switch status commands reported clean trees. No implementation source changes, commits or pushes accompanied branch creation.

## 4. Jolt cut: assemble the PCS instance, do not prove it

### 4.1 Existing source boundary

Current [`jolt/crates/jolt-prover/src/akita/prover.rs`](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/prover.rs) runs stage 0 commitments and stages 1–7, then calls `prove_stage8`. Current [`stage8.rs`](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/stage8.rs) performs:

1. Canonical trace layout and leaf-claim assembly.
2. Prefix-packing/claim reductions required by that source generation.
3. Advice, optional field-increment or committed-program group construction, with original commitments and retained hints.
4. Main trace `GroupOpeningClaim` construction.
5. `PCS::prove_batch(...)`, which produces a **complete** native PCS proof.

The desired frontend cut is **after steps 1–4 and before step 5**. Stopping before the packing/claim reductions would leave unsupported semantic claims, not the actual committed instance.

For the inspected current generation, the tuple is ordered as `[UntrustedAdvice?, TrustedAdvice?, FieldInc OR committed-program groups, OneHotTrace]`; field-inline and committed-program suffixes are exclusive. Another pin may use limb groups or different canonical roles. Preserve its actual order and role/point validation, then explicitly migrate the selected format.

**Modern native-column difference, verified at `0791f79`:** its `prove_stage8` obtains the main group from `one_hot_trace_claim`, preserving every native column evaluation at one common point. It does not prefix-pack the main trace into a single polynomial. Auxiliary advice/program objects still use their required `reduce_precommitted` operations. Its verifier validates the actual column count/arity and derives the same group before `PCS::verify_batch`. The proposed cut preserves those native columns; it must not carry current main's main-trace prefix-packing back into this branch.

### 4.2 Proposed typed frontend result

Let

$$
 u_t=(\mathcal P,\mathrm{run},t,B_t^-,B_t^+,\pi_t^{\mathrm{front}},\mathcal O_t),
 \qquad
 \mathcal O_t=((C_{t,g},\mathrm{shape}_{t,g},r_{t,g},v_{t,g}))_{g<G_t}.
$$

`FrontPending` contains the canonical proof messages for stages 1–7, all required packing reductions, the verifier-derived ordered native opening statements, and the segment boundary statement. Its private counterpart retains each original source handle/hint and the witness ownership needed by Akita.

A matching frontend verifier replays **all** frontend checks and derives exactly `O_t`; it returns a pending relation, not `Result<()>` meaning full acceptance. Fresh claims are never copied from a caller-supplied ledger without the original reductions. No full segment PCS proof is generated or verified at this cut.

The existing frontend proves complete executions with fixed initialization/entry/termination assumptions. Its orchestration and unaffected kernels are reusable; actual segment semantics require the changes in §10. A host checkpoint is not a proof of a boundary.

## 5. Akita cut: complete one nonterminal reduction

### 5.1 Exact current producer/verifier sites

Current [`akita-prover/src/protocol/prove/root.rs`](../../crates/akita-prover/src/protocol/prove/root.rs), `batched_prove`, calls `prepare_fold`, then `prove_fold`, then unconditionally `suffix::prove_suffix`. The native cut is **after `prove_fold` succeeds and before `prove_suffix`**, for a scheduled `Recursive` successor with `OuterPayload` binding.

Current [`akita-verifier/src/fold/root.rs`](../../crates/akita-verifier/src/fold/root.rs), `verify_root`, delegates to `verify_fold`. Current [`fold/mod.rs`](../../crates/akita-verifier/src/fold/mod.rs), `verify_fold`, returns only after Stage 3 and `validate_stage2_replay`. The matching verifier cut is after that return, not after merely parsing the outgoing commitment or replaying sumcheck messages.

An ordinary root with no recursive successor produces `TerminalInnerState`. That is not an ordinary outer commitment and cannot be used as the proposed receiver by relabelling it.

### 5.2 Complete reduction work retained before the cut

The reduction must complete, in native order:

1. Trusted setup/catalogue/profile admission; original source geometry and backend ownership validation.
2. Native opening conversion and any required extension/tensor reduction.
3. Shared opening-side D/H payload binding **before** the group folding challenges.
4. Schedule-approved bounded fold sampling and response construction.
5. Full ring relation and complete successor witness construction.
6. Successor commitment binding **before** ring-switch, range and relation challenges.
7. Ring switch, digit range, selected response norm, negative-binary compression checks and fused relation/opening sumcheck.
8. Direct public setup evaluation or the complete Stage 3 setup-prefix reduction.
9. Final Stage 2 output equality validation and witness-lineage advancement.

The existing checked output is essentially `(C_next, r_next, v_next, optional setup opening)`. It is a **pending** ordinary opening obligation. A pause hook does not make it an accumulation scheme.

### 5.3 Full retained witness and ownership

`SuffixProverState` contains the successor witness handle, commitment binding, commitment material, sumcheck point, evaluation and optional setup-prefix opening. The complete Eq. (109) witness is

$$
 w' = w_{\rm main}\Vert Q_{\rm main}\Vert
 \mathop{\Vert}_{a}[c_a\Vert Q_a],
 \qquad
 w_{\rm main}=\mathop{\Vert}_g[\widehat z_g\Vert\widehat e_g\Vert\widehat t_g],
$$

with actual chunk/group order, native dimensions, digit depths, group-local F and shared H stages, quotient families, alignment zeros and physical/Boolean padding. Quotients disappear only for an actual quotient-free relation; compression tails disappear only for actual absent chains.

Opaque commitment material retains the linked inner images and compression-chain witness/quotients, not just the public compressed payload. Tensor-packed committed digits and logical witness digits may differ. The receiving opening conversion must use the actual map.

**Lifetime defect in a naive early return:** `ProofScope::Drop` aborts unfinished scope; finishing a scope also retires its authority. Current CPU sessions freeze complete schedules, group counts, commitment identities and successor lineage. Returning `root.next_state` while dropping the guard/session makes the handles unusable. A same-proof pause must retain live authority and transcript state. A streaming update needs a new, checked adoption/transition contract; bypassing opaque ownership checks is not a fix.

## 6. Native accumulator relation and algorithms

These names are **proposed**, not existing APIs.

### 6.1 Relation

Fix a finite authenticated receiver profile `P_*`, capacity `N_*`, source map, matrix/compression geometry, certified alphabet and padding predicate. Define

$$
\begin{aligned}
R_{\rm acc}(a;w,d)\iff{}&
 a=(P_*,C,r,v,\mathrm{layout},\mathrm{setup\_pending},B),\\
& |w|=N_*,\quad w\in\mathcal A_*^{N_*},\quad Z_{\mathrm{layout}}(w)=0,\\
&\operatorname{Bin}_{\mathrm{layout}}(w)=0,\\
&\operatorname{Rel}_{\rm com,P_*}(C;\operatorname{Pack}_{P_*}(w),d),\\
&\widetilde w(r)=v,\\
&\operatorname{Rel}_{\rm setup}(\mathrm{setup\_pending}).
\end{aligned} \tag{R-acc}
$$

`d` is the complete linked algebraic commitment state, including its certified source/compression alphabets. `C` is the full original handle/geometry, not a hash of an unspecified matrix. `B` is the declared proof-side VM endpoint. History counters and receipt metadata must be authenticated separately; do not make arbitrary-valid-input completeness fail at a counter limit by burying it in the arithmetic relation.

`Bin_layout` requires exactly the declared Eq. (109) F/H spans to lie in `{0,-1}`; it does not constrain every receiver cell to be binary. Include this predicate in the relation because the old/final domain prefixes enforce it. Omitting it from `R_acc` while checking it during `Accumulate` would invalidate completeness for arbitrary relation-valid inputs.

For an identity-source receiver, `Pack_*` is an injective coefficient regrouping with one source digit. A general gadget decoder is **not** identity. Native tensor/evaluation-trace alternatives need their exact expansion bound.

### 6.2 Intended update

`Accumulate(a,w,d,u_t,w_t)`:

1. Replay/check the fresh frontend prefix and VM adjacency, deriving `O_t`.
2. Validate every original handle and receiver profile. Bind the old instance, ordered fresh original handles, profile/program/run context, frontend record and VM boundaries **before any source-range challenge**. Consume any prior registered setup opening as a real source.
3. For the generic local bounded-input scheme, run the same-handle full-domain reduction for the old receiver and applicable fresh sources. The separately proved history-only optimization can reuse the old outgoing domain record, but is not substituted into the generic local theorem. Convert ordinary direct-coefficient claims through the fixed digit-axis point and perform the paid same-RawView normalization, retaining all original claims and P-column geometry.
4. Bind the derived query kinds, points, claimed values and complete opening descriptor; bind native shared D/H partial data before the folding challenges. Do not splice independent transcript states.
5. Perform a newly admitted old-plus-fresh native Akita reduction using the complete machinery in §5. Add the exact required-zero mask once to the outgoing fused strong check; native range alone does not certify honest padding. Retain every binary/norm/relation check and the complete outgoing domain record.
6. Require exact complete successor size `Phi_P(N_*,S) <= N_*`; pad only at the declared authenticated zero spans.
7. Return `(pi_acc,a',w',d')`, where `a'` is still pending. Retire consumed witness/material only after checked successor adoption succeeds.

`VerifyAccumulate(a,u_t,pi_acc)` replays the same messages, requires every retained check and emits the same pending `a'`. It does **not** call a full fresh segment PCS verifier and does **not** omit strong-check leaves that must be discharged by the successor relation.

`Finalize(a,w,d)` uses an admitted final Akita reduction/suffix to discharge the exact pending opening and remaining domain/setup obligations. `VerifyFinal` additionally authenticates initialization, ordered transition history, approved program/public statement, and the **existing native final CPU/public-I/O acceptance relation**. It does not silently add an emulator-halting or self-loop-target theorem.

**Missing admission:** existing root/suffix code does not implement or prove this topology. §8 identifies the adaptive theorem needed before these algorithms can be claimed sound.

### 6.3 A final-only bounded decider using a complete native PCS proof

For the proposed **identity-source, full-grid receiver**, final revelation is not the only construction. The following final-only composition retains the same final `C` and completes native Akita once:

1. Bind the complete final instance and its original physical profile. Run the same-C alphabet/zero prefix for the receiver table `w`. Let its pending endpoint be `(C,u,y)`; retain the incoming `(C,r,v)`.
2. Reduce these two ordinary evaluations on the same table to one point using a public-coin degree-two sumcheck:

   $$\alpha v+\beta y=\sum_{x\in\{0,1\}^{\mu_*}}w(x)\bigl(\alpha\,\mathrm{eq}(r,x)+\beta\,\mathrm{eq}(u,x)\bigr).$$

   Bind both input claims before independent scalar weights. Its endpoint is one ordinary `(C,z,t)` under the **unchanged** receiver geometry. Check the terminal sumcheck value equals `t*(alpha*eq(r,z)+beta*eq(u,z))`, then authenticate `t` by the PCS below. Do not divide by that public factor or assume it is nonzero. Charge the claim-separation, sumcheck, range-prefix and padding errors/trees.
3. Produce one **complete** admitted Akita opening for this final `(C,z,t)`, including its ordinary recursive suffix and terminal. Discharge any remaining setup obligation explicitly; the simplest profile uses direct setup and has none.
4. The final verifier replays those reductions and the complete opening, in addition to authenticating the accumulated history/application statement. It never asks for a full PCS proof for an earlier segment.

**Conditional extraction argument.** Retain the complete final PCS extractor's root weak unit/product certificates and original certified path, not merely an arbitrary algebraic decommitment. Each accepting fork endpoint supplies these under original `C`, or a scheduled collision. Corollary 10.17, with the full mixed occurrence inventory, identifies one commitment-side source across the preceding multipoint/range-prefix branches; endpoint points, D/H data and later challenges need not coincide. Because this receiver's source map is an injective full-grid identity, the ordinary endpoint is an evaluation of that same table `w`. Backward interpolation and independent-weight separation recover both original evaluations, the receiver alphabet and every required zero. The complete PCS supplies certified original commitment-path alphabets. Together these give `R_acc`, without assuming the ordinary weak source was short before the prefix was extracted.

This argument does not apply to an arbitrary gadget-decoded receiver by pretending `G` is injective; use the raw-query construction for that different choice. It also does not make an ordinary final PCS proof alone a bounded-source certificate. The added same-C prefix and claim reduction are essential. A checked ownership transition must expose the retained original witness as an opening source, not recompute a different canonical commitment.

This is a final-only conditional decider construction. Its concrete native schedule, matrix comparison classes, all-valid-source completeness, sampler/FS budget and composition into the finite history still require admission. Its cost can be compared with revealing/checking the full final witness; no timing advantage or zero-knowledge property is claimed here.

### 6.4 Concrete full-ring final decision

[The compact profile](AKITA_INSTANCE_ACCUMULATION_COMPACT_PROFILE.md), §§3–4, instantiates a new final-only suffix: receiver capacities \(2^{22}\to2^{21}\to2^{20}\to2^{19}\), followed by a direct full-ring terminal. At creation, the last output is an actual A-image `TerminalInnerState`, not a relabeled compressed payload. Its partials are bound before the shell challenge; the verifier checks both extension planes, every A/carrier coefficient, scalar recombination and the explicit response norm. Independent review derived its weak unit/product certificates and source uniqueness, which supply the preceding strong trees. The 212992-byte terminal subtotal excludes the entire preceding archive. This is a reviewed mathematical terminal, not an existing trace-terminal API.

## 7. Bounded-source repair without assuming homomorphic commitments

See the companion [bounded bridge investigation](AKITA_INSTANCE_ACCUMULATION_BOUNDED_BRIDGE.md).

### 7.1 Identity receiver range prefix

For an identity-source old receiver, prove the prescribed alphabet and zero spans on the **same `C`**. A range/sumcheck prefix ends in additional ordinary evaluation obligations on that same source; these are reduced together with the predecessor's existing `(r,v)`, not accepted independently. A commitment to a replacement short vector is insufficient.

This can restore boundedness after weak extraction **if** the endpoint evaluations are authenticated and same-source uniqueness applies at the certified cross-profile collision envelopes. It does not make those endpoints authenticated by itself and does not fix §8.

### 7.2 Fresh gadget sources are a different problem

For a decomposed source,

$$G_b s=f$$

is noninjective: the source-plane difference `(b,-1)` decodes to zero. A range bound on `f` does not prove the needed bound on `s`. Akita's weak extractor bounds a challenge multiple of the source and explicitly does not promise the canonical short decomposition. Either add a same-original-handle source-query bridge with a proved relation, or change **new producers at commitment time** to an injective bounded representation. Never reshape an already formed handle.

**Derived retained-handle solution:** [the raw source-query construction](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md) supplies this bridge for fixed direct-coefficient handles. Replace the input gadget contraction by raw-digit query weights, retain response-digit recomposition and authenticate the complete carrier-valued partial through D/H. The map commutes with the original challenge action; coordinate subtraction extracts the original raw source under unchanged A/B/F. A full physical raw-source predicate then restores its bound.

[The independently reviewed digit-axis lemma](AKITA_INSTANCE_ACCUMULATION_DIGIT_AXIS.md) additionally converts an ordinary decoded evaluation into a raw-view evaluation at a fixed digit point. A paid degree-two multipoint reduction combines all ordinary and range endpoints into **one channel per original handle**, including P-column main sources without repacking their commitments. Noninjective decoding still prevents deriving source bounds from decoded bounds; the source predicate remains required. Maps outside that exact positional/direct-coefficient contract retain their separate channels or need another proved adjoint.

**Reuse the old native range reduction, not just its code.** For an injective full-physical receiver, the preceding complete Stage-1/Stage-2 record reduces its domain to the pending opening consumed next. [Range reuse](AKITA_INSTANCE_ACCUMULATION_RANGE_REUSE.md) recovers that domain by backward history extraction. Required zeros are fused through a restricted-mask degree-two term using the same outgoing evaluation. In the one-channel profile this saves the duplicate old prefix/normalization work, not another partial channel. Fresh-source predicates, complete outgoing strong checks and final/history extraction remain.

### 7.3 New fresh-producer option: bounded bitplanes

For a word table, form `B(j,a)=b_j(a)` with Boolean bits and a fixed little-endian plane axis. Commit to this full physical bit table under an admitted **one-digit** source map, with same-handle Boolean/padding obligations. A balanced source alphabet must contain both 0 and 1; base at least four suffices for this elementary representation requirement, not for security admission.

Dense Boolean bitplanes are **not** unit-one-hot sources. An admitted dense/balanced source contract must price their actual density and bound; the current `CommittedSourceClass::UnitOneHot` requires at most one hot coefficient per declared chunk. Merely changing the declared bit width of an existing preset does not establish a one-digit source catalogue or its completeness/security.

For `L=2^ell` bits, set

$$
 u_k=\frac{2^{2^k}}{1+2^{2^k}},\qquad
 K_L=\prod_{k<\ell}(1+2^{2^k})=2^L-1.
$$

If all denominators are nonzero, then

$$K_L\,\operatorname{eq}(u,j)=2^j,\qquad
 \widetilde f(r)=K_L\,\widetilde B(u,r). \tag{BitDecode}
$$

Thus one structured source query supplies the word evaluation; no homomorphism of the full commitment is used. For

$$q=2^{128}-2^{32}+22537,$$

all factors are nonzero for `L<=128`, and

$$K_{64}=18446744073709551615,\qquad K_{128}\bmod q=4294944758.$$

A `u64` decode is an unambiguous integer because `2^64<q`. A 128-bit Boolean table decoded into the field can represent an integer at least `q`; prove `<q` if canonical integer semantics require it. Field semantics alone do not give that comparison.

The Boolean prefix uses, for fresh independent address/plane weights, the identity

$$
\sum_{j,a}\operatorname{eq}(\rho,(j,a))B(j,a)(B(j,a)-1)=0,
$$

plus the exact reserved-zero predicate. It ends in source queries that must be reduced/authenticated, not treated as completed proofs. The Boolean formula is distinct from Akita's negative-binary compression formula `w(w+1)=0`.

This option migrates advice/program/RAM/field-increment producers and their packing/claim reductions. The logical table has `L` times as many cells; the source A-width comparison is approximately `L/delta_old` before geometry/padding, not automatically `L` times the work of an already decomposed old source. Native code presently gives one common point per group: repeated same-`C` queries at different points need explicit occurrence groups or a proved multipoint reduction and catalogue admission. No free batching is assumed.

The identity and denominators were checked by 263 exact weight/tensor evaluations, including 64/128-bit plane endpoints and random Boolean tables. This checks algebra only, not cryptographic binding, completeness of a new catalogue or performance.

## 8. Decisive soundness gap: repeated partial folds

See [adaptive soundness investigation](AKITA_INSTANCE_ACCUMULATION_ADAPTIVE_SOUNDNESS.md).

Akita's analyzed schedule has application groups **only at the root**. A later level receives exactly the recursive witness, optionally with one registered setup source. It does not repeatedly receive an old accumulator plus fresh segment groups. Its admissibility theorem also requires a polynomial mixed extraction tree.

For a local repeated step whose extraction-tree factor is `b>1`, the existing tree method can have

$$\operatorname{Tree}_T\ge b^T.$$

If `b=N^c`, the bound is `N^{cT}`, not a single polynomial in `(lambda,T,N)`. This is an obstruction to invoking the **existing proof method**, not an impossibility theorem for all Akita accumulation.

The one-fold backward theorem assumes **descendant-certified** outgoing openings. The successor commitment changes under a rewind of the folding challenge. Final authentication of the actual `C_T` does not authenticate all those counterfactual children. Same-handle boundedness or an independently known source does not automatically prove the child's claimed evaluation at its reduction endpoint.

**Depth is a profile choice, not an extra user requirement.** The user did not ask for unlimited updates. Two theorem targets must be distinguished:

1. **Finite supported `T_max`:** compile and bind the whole maximum-depth transition family, extend the local proof to this topology, and account for the actual complete accepting tree, samplers, query/error budget and reduction runtime. If each local tree is polynomial in explicit size, fixing `T_max` independently of the asymptotic inputs can give a polynomial whose exponent depends on that fixed bound. This is not automatically a useful concrete security guarantee: every Module-SIS adversary runs with the composed extractor's cost. An enormous reduction cannot be ignored behind the word “fixed.”
2. **Arbitrarily growing depth:** prove one uniform bound

   $$T_{\rm ext}(T)\le\operatorname{poly}(\lambda,T,N,Q),$$

   including every continuation/endpoint/receipt oracle call. This stronger research target needs an extraction argument that avoids uncontrolled multiplication with depth.

The indexed Nova Definition 5 explicitly uses constant depth and explains that scope. It would be wrong to reject every finite-depth native design solely because it lacks the stronger uniform-growing-depth theorem. Neither target is established by naming a local extractor or invoking SuperNeo/Lova/LatticeFold+: their relation, transcript order, commitments, norm certificates and projection hypotheses must match.

A source/range certificate reduced into the same pending successor remains a **pending obligation**, not an independent leaf oracle. A complete independently extractable opening certificate for every accepting fork endpoint can remove that dependence, but its proving/verification cost must be charged; finishing the receiver at every step can reproduce the full proof work that the selected target intends to avoid.

No security-admitted finite native re-entry profile, or stronger uniform adaptive-chain theorem, has been established here. A finite unrolled design may obtain counterfactual leaves by replaying its complete continuation to an authenticated final decision; the actual final witness alone is not every leaf. This still needs the topology extension, full bounded final relation, complete tree/runtime/error ledger and a useful supported depth. These are correctness/security gates, not evidence that native accumulation is impossible.

### 8.1 Derived conditional local extension, not an impossibility claim

There is a concrete local proof route for a **fixed ordered list of direct-coefficient, identity-source groups**, including an old receiver and fresh native application groups. Akita's “application groups only at root” and packing-level policies describe its analyzed schedule; they are not identities forbidding the following local algebra. Do not extend this conclusion to arbitrary raw-source queries or evaluation-trace maps without proving their own map identities.

**Conditional local lemma.** Suppose every group has its original fixed commitment geometry, full-grid identity source map and declared padding; all carrier/ring, unit-difference, alphabet and mixed-collision hypotheses hold; the extended Eq. (109) relation contains every required row; and a complete prefix-consistent mixed tree supplies authenticated full outgoing witnesses at every strong-check leaf. Then the combined source-range/native-fold reduction extracts bounded witnesses for all original incoming handles and their pending evaluations, or a nonzero collision in a priced matrix view.

**Derivation.**

1. For each fold-challenge child, use its authenticated outgoing witnesses and Akita's ring/range/relation interpolation to recover the exact response, commitment-path and evaluation equations. Carried compression paths refer to their actual original handles.
2. Subtract coordinate siblings with the same prechallenge incoming list. Cancel only the prescribed unit difference. This gives the weak source/evaluation witnesses of Akita Theorem 10.23 for each old/fresh occurrence. No equation needs the group to be an application group only at the global root.
3. Match occurrences of each original `(shape,C)` across the range-prefix children using Corollary 10.17 and the complete mixed source/B/F inventory. Either this yields a priced collision, or those occurrences share one commitment-side source. Point-dependent partials need not coincide.
4. Apply the injective full-grid source decoder. Its one table supplies every authenticated endpoint of that handle's range/zero-prefix tree. Backward interpolation gives `Q_D(w(x))=0` and each required zero for every physical coordinate. Retaining the original pending evaluation query gives its truth under this same bounded source.
5. Thus the original old/fresh sources satisfy the bounded input relations. Process a finite chain backward from its complete bounded final decision; counterfactual endpoints come from their own accepting continuations, not from copying the actual final witness.

This is a **conditional algebraic derivation**, independently checked against Akita's one-fold proof, not an admitted parameter set or existing public API. The unresolved work is to instantiate every hypothesis for a complete fixed-capacity receiver and frontend, and to prove the precisely specified finite adaptive-ROM/history experiment with its real tree/runtime/security ledger. The local argument itself identifies no fundamental obstruction to old-plus-fresh native accumulation. Avoiding multiplicative extraction for arbitrarily growing depth is a separate, stronger research problem.

## 9. Fixed-size closure and strong completeness

The shape compiler must count the complete Eq. (109), private commitment material, pending setup and finite catalogue. For every admitted fresh source type `S`, require

$$\Phi_P(N_*,S)\le N_*.$$

It must also certify all digits for **every `R_acc`-valid witness**, not just honest previous outputs. For coefficient-bounded sources and accepted challenges with `ell_1` cap `K_g`, a deterministic group response bound is

$$\|z_g\|_\infty\le P_gB_gK_g\beta_g.$$

Choose digit depths to cover this bound with the actual signed radix endpoints and prove the sampler/grinding completeness budget. Field invertibility does not preserve a short source: modulo 17, `2*(-8)=1`, so a bounded product of magnitude one can decode to a source of magnitude eight after division.

Rank changes required by certified Module-SIS collision radii feed back into `Phi`, D widths, quotient/compression lengths and setup capacity. A shape-only fixed point or a small public payload is not a security/performance certificate. The prior [closure document](AKITA_ACCUMULATION_01_FIXED_SIZE_CLOSURE.md) includes exact counting and a deliberately non-security-admitted example; its conditional status is unchanged.

The newer [receiver calculation](AKITA_INSTANCE_ACCUMULATION_RECEIVER_PROFILE.md) includes sufficient raw folding entropy and exact checks against the pinned q128 attack-model table. A generic 128-polynomial signed-digit input closes at `2^23` cells with repeated full occurrences, or `2^22` cells under the explicitly derived two-channel relation. Both exact full-successor calculations received independent review. They do **not** instantiate Jolt's actual source catalogue, the complete finite-chain security budget or performance parity.

The same note now contains a Jolt-dimensioned, newly provisioned base-32 example: 56 native columns, 4096 segment rows and two full-field 65536-word RAM endpoints. With explicit shared query channels its full successor has 4087808 cells in capacity \(2^{22}\); reusing the old receiver's outgoing range removes one partial channel, leaving 3981312. These are arithmetic/table-model calculations, not approval of existing frozen source profiles or measured runtime.

[The extension-challenge security profile](AKITA_INSTANCE_ACCUMULATION_SECURITY_PROFILE.md) addresses the base-field FS ceiling without changing the lattice modulus: committed sources stay in \(\mathbb F_q\), scalar challenges use \(\mathbb F_{q^2}\), and full-ring partials carry both base-field planes. Its independently checked example closes at \(2^{23}\) cells. Explicit whole-manifest caps imply a statistical FS contribution below \(2^{-149}\), but only if every frontend/native/final challenge site and sampler meets that contract. The required field separation is new implementation work; no unchanged q128 API or end-to-end 128-bit claim is inferred.

The smaller [compact extension profile](AKITA_INSTANCE_ACCUMULATION_COMPACT_PROFILE.md) uses the reviewed digit-axis normalization, omega 56 and a declared \(Q_{\max}+1\le2^{32}\) policy. Its full successor has **3555328** cells at capacity \(2^{22}\), D width 6240 and 120 lattice coordinates per update. Its explicit three-step final suffix and direct terminal also passed independent algebra/count review. Its statistical FS bound is conditional on the complete manifest; it is not the same query-budget policy as the larger omega-153 example.

[The coefficient-relation compiler](AKITA_INSTANCE_ACCUMULATION_LINEAR_RELATION.md) checks exact native ring coefficients, not an unpriced quotient-lift. [The global manifest](AKITA_INSTANCE_ACCUMULATION_GLOBAL_MANIFEST.md) specifies all challenge macros, bounded unbiased samplers, endpoint checks, conservative error sums and an explicit fixed-polynomial mixed-tree ceiling. [The matrix registry](AKITA_INSTANCE_ACCUMULATION_MATRIX_REGISTRY.md) enumerates the compact example's 33 independent roles and every relevant mixed source radius. Independent integrated review approved their mathematical composition; emitted implementation certificates must reproduce these contracts.

Current planner policies also restrict packing to early levels, quotient-free stages to later evaluation-trace direct stages, and supported norm/setup configurations. Repeating an early packing fold by resetting its level label does not make it a theorem-covered schedule.

## 10. Continued-segment relation remains essential

[The pinned segment interface](AKITA_INSTANCE_ACCUMULATION_SEGMENT_INTERFACE.md) specifies the modern `0791f79`/`8357433` source roles, physical point permutations, endpoint reductions and separate ordinary/raw query ledgers. Its main source has `P=128/k+64/k+1+bB+bR <= 64` native polynomials, each over `N*2^k` cells; it must not be substituted into the generic receiver example merely by equating polynomial counts.

Accumulating a correct PCS instance cannot repair a wrong VM relation. The selected frontend must bind:

- Fixed program, actual ISA/virtual expansion profile, address remap and memory capacity for the entire run.
- Expanded PC and architectural PC at both endpoints; legal cuts only after complete source instructions/inline groups; authenticated first/last rows and active-prefix length.
- All 128 integer/virtual registers, including modeled reservation/CSR state; the separate 16 field registers if the selected ISA uses them. Do not silently use only 32 architectural registers or a checkpoint backend lacking field state.
- Full proof-side RAM endpoints under native original Akita handles, not Merkle roots or literal device snapshots; initialization once and exact original-handle/shape equality across boundaries.
- The native indexed input/advice and addressable-output public statement, with its existing panic/final-RAM convention; any exposed extra counters require nonwrapping semantics.
- Nonempty ordered execution and no reinitialization; the existing global final padded-successor/public-I/O checks apply once. An interior proof-side termination word is not itself global completion.

For a RAM table with incoming/outgoing word vectors `B^-`, `B^+`, write address table `A(a,s)` and increment `I(s)`, define the Boolean-grid history first:

$$V(a,t)=B^-(a)+\sum_{s<t}A(a,s)I(s).$$

Its multilinear extension satisfies, at arbitrary address/cycle points `(x,y)`,

$$\widetilde V(x,y)-\widetilde B^-(x)=
 \sum_{s\in\{0,1\}^n}\widetilde{\mathrm{LT}}(s,y)\widetilde A(x,s)\widetilde I(s),$$

and the complete endpoint relation is

$$\widetilde B^+(x)-\widetilde B^-(x)=
 \sum_s\widetilde A(x,s)\widetilde I(s).$$

With claims bound before fresh `eta`, an aligned reduction is

$$v+\eta f-(1+\eta)b=
 \sum_s[\widetilde{\mathrm{LT}}(s,y)+\eta]\widetilde A(x,s)\widetilde I(s). \tag{RAM-end}
$$

This generalizes the actual Jolt RAM value-check relation; it does not add commitments homomorphically. It leaves real native queries on both original endpoint handles and the actual address/increment sources. All differing points need real opening/claim reductions. Do not replace the MLE of a Boolean-grid product by a product of MLEs without the appropriate sumcheck identity.

Current register checking has zero initial values and no full endpoint term; replace that initialization with the incoming public register vector and prove the outgoing vector. Current PC shifting uses padding and a terminal `NextIsNoop` convention; an internal cut needs the authenticated external successor instead. Keep the original global final-row convention. Derive interior RAM by the proof-side prefix recurrence, never by repeatedly invoking a whole-execution final-state provider or by equating it to `JoltDevice` memory.

Jolt I/O is addressable, not an append-only journal or FIFO input stream. Runtime arithmetic advice and a sequential witness tape are different sources. The default SDK trace tape is existential/untrusted, not a proof that host `compute_advice` was honestly executed. Whether a separately authenticated external tape is part of the statement must be explicit; do not invent it as an incidental new requirement.

The source-derived endpoint/query interface and [named consumer lowering](AKITA_INSTANCE_ACCUMULATION_SEGMENT_LOWERING.md) passed independent and integrated review. They establish **concatenation equivalence to the existing whole-run Jolt relation**, with once-only initial/final checks—not an unrequested stronger MMIO/emulator theorem. The [frontend descent audit](AKITA_INSTANCE_ACCUMULATION_FRONTEND_DESCENT.md) reconstructs original K-valued sources beneath E-valued challenges, including total exceptional branches. [Physical main-source reuse](AKITA_INSTANCE_ACCUMULATION_MAIN_SOURCE_REUSE.md) closes omitted-address-zero holes before relying on existing Booleanity. Generating the full existing trace once and partitioning legal instruction groups is sufficient; online/resumable tracing is not assumed.

Default new RAM and public register endpoints retain full proof-field values. Honest word-valued traces are included without assuming every algebraically valid cut-state is u64. [The endpoint reuse specification](AKITA_INSTANCE_ACCUMULATION_MEMORY_COST.md) derives a private changed-source-fibre → A → changed inner-image digits → B → **complete F** update. It preserves unchanged noncanonical decommitments and identifies missing raw-source/inner-digit retention in existing portable hints. The same cache optimization is available to the recursive baseline; it does not remove full-source opening/range scans.

## 11. History, final decision and receipt cost

A final Akita proof of `(C_T,r_T,v_T)` authenticates **that declared endpoint** under its admitted schedule. It does not by itself certify which sequence produced it. A hash chain of prefix records also does not prove the transitions.

A complete deployment must choose and prove one history contract:

1. An authenticated, replayed archive of all frontend/reduction prefixes, plus a sound native chain reduction and final decision. This has `O(T)` archive/verification cost.
2. A succinct receipt for those transitions with a proved auxiliary-I/O/extraction composition theorem. A final Jolt outer proof is a possible carrier, not a substitute for the missing native reduction theorem.

Neither option generates a completed Akita PCS proof for each fresh segment. Do not call archive replay online IVC, or hide its bytes behind the size of the final PCS proof. Public register endpoints reveal state; no zero-knowledge claim is made.

The user selected **option 2**. The current [D1 receipt design](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) uses one fixed guest to compute the concrete inner transcript and verify the native archive/final decider from private advice, with approved inner setup/program data authenticated by an original outer lattice commitment. The transmitted result is compact public application data plus one complete ordinary lattice proof—not full segment proofs or a public archive. The previously reviewed archive core remains its inner component. For that core, the complete accepting mixed tree yields bounded original-source witnesses for the linked whole-run K relation, or a collision in a registered matrix role. Its standalone classical-ROM failure bound is
\[
 (Q+1)\left(\frac{c_E}{|E|}+\frac{c_{\mathcal C}}{|\mathcal C|}\right)
 +\Delta_{\rm setup}
 +\sum_{I=1}^{33}\operatorname{Adv}_{\rm MSIS,I}(R_I),
\]
with **the entire continuation-replayed extraction cost** in every \(R_I\). All-valid noncanonical source/path completeness excludes only the specified bounded-sampler abort events. Formal polynomial extraction does not imply a useful concrete reduction runtime or overall 128-bit security.

This is **not** the receipt's end-to-end bound. [D1 §7](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) gives the conditional one-shot composition with actual fixed-public inner computation, an ideal independent outer oracle and the explicitly heuristic native-language premise. It averages over installation provisioning while preserving the sampled installation within extraction. [Receipt setup](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SETUP.md) retains noncircular static-data authentication and the extra outer matrix/advice/profile inventory. The inner Q32 estimate cannot be silently applied to the derived archive producer, and an unchanged base-field outer cannot inherit the E-challenge bound.

For a fair recursive control, use the **same** continued-segment frontend, source encodings, setup, boundaries and public run statement, but fully discharge each segment's **ordinary** PCS ledger and its actual recursive wrapper/history. Both sides may reuse endpoint material and private caches. Do not charge the control for extra raw short-source predicates needed only by accumulation. Existing independent-job conjunction is useful tooling evidence, not this baseline.

Charge segment frontend proving, the complete one-fold update, bounded-domain queries, new source encoding, full Eq. (109) commitment/material, setup work, native transcript/grinding, history transport, final decision and any outer proving. Performance parity is **unmeasured**. The presence of a cut before `prove_suffix` is not evidence of a speedup. Do not cut over if the required paired end-to-end experiment fails; an unmeasured model is not proof that parity is impossible.

## 12. What can be reused, and what cannot

| Component | Reuse supported by inspected source | Required new work / prohibition |
|---|---|---|
| Jolt stages 0–7 and stage-8 claim assembly | Canonical frontend messages, source handles and native claim construction | Separate `FrontPending`; real segment init/end changes; complete claim ledger |
| Akita root/group machinery | Per-group original profiles, shared D/H, accepted challenges and relation construction | Admit old receiver plus fresh sources; preserve original geometry |
| Native `prove_fold` / `verify_fold` | Complete one-level reduction and pending successor evaluation | Public typed reduction API after final validation; not successful verification |
| Eq. (109) builder and commitment | Full digits, quotients, F/H tails, actual padding and private material | Fixed-point/security certificate; identity-source receiver or exact mapped alternative |
| Internal Commitment/Witness source mixing | Existing suffix already mixes registered setup and witness | Arbitrary fresh application groups are not currently suffix inputs |
| Scope/lineage checks | Reject invalid handle ownership and successor adoption | New admitted update lifetime; no private-binding bypass |
| Native range/norm checks | Scheduled response/digit/physical norm kernels | Same-handle source domain and strong-completeness theorem |
| Existing terminal/suffix | Final discharge of an admitted opening schedule | No arbitrary chain theorem inferred from one final endpoint proof |
| Existing Fiat–Shamir machinery | Challenge generation/transcript primitives | Reviewed update grammar, full binding, budgets and model; no new hash accumulator |
| Nova/Lova/LF+/SALSAA ideas | Relevant closure, norm and extraction obligations | No direct replacement theorem for Akita A/B/F without matching hypotheses |

## 13. Evidence and admission checklist

Observed: live qmd passages on Akita Eq. (109), schedule admissibility, source uniqueness, weak extraction and FS extraction; Nova's explicit constant-depth definition; Jolt/Twist-Shout memory passages; current and immutable native source boundaries. Exact models checked 263 bitplane/tensor identities, 36 RAM endpoint identities, all 101 signed-base-four range values in F_101, 32 digit-axis identities, both extension carrier planes, native ring subtraction/product equations, and 700 column-specific restricted-mask identities. The physical-zero-hole counterexample and inverse-norm counterexample were reproduced. These are algebra checks, not proof-system execution or performance evidence.

Implementation admission requires all of:

- Complete same-semantics continued-segment frontend and query ledger.
- Same-original-handle bounded-source relation, including fresh producers and full-grid/padding semantics.
- All-valid-input completeness and an exact security-admitted fixed-size receiver/catalogue.
- A native reduction/composition theorem and explicit supported-depth security policy, matching Akita's polynomial-tree/EPT assumptions and runtime-sensitive advantage bound. A tight end-to-end 128-bit reduction is not an extra user requirement; per-matrix attack-model tables cannot establish that claim. Exact transcript/query/error and collision inventories are still required. A uniform-growing-depth theorem is needed only if that stronger claim is made.
- Authenticated history/receipt and declared classical/concrete/quantum model.
- Independent review of the **native** design, not reuse of the old public-LVA approval. [The review record](AKITA_INSTANCE_ACCUMULATION_REVIEW.md) now records final integrated implementation admission for the selected finite compact archive profile, not arbitrary source geometries.
- Then matching-source builds, actual producer/verifier execution, adversarial regressions and measured parity before accepted cutover.

The inner archive-core gate passed and matching local branches were created. The [D1 review record](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_REVIEW.md) now records scoped independent carrier/compiler approval and integrated permission to implement the specified heuristic prototype. Implementation must generate and validate exact codecs, the compiled guest/source/ISA, geometry and complete profile/resource certificates before accepting-implementation admission. Do not identify concrete hash code with an ideal oracle, inherit Q32 from honest call counts, replace extraction by language soundness or hide restricted CH-I content behind a new name. Exercise the complete producer/verifier and paired performance before accepted cutover. The earlier [ideal-route review](AKITA_INSTANCE_ACCUMULATION_RECEIPT_REVIEW.md) remains a distinct research result.

## References and read scope

- [Akita local paper](SOURCE_PROVENANCE.md#akita), §§4–6, 8.2, 9.3, 10.2 and 12.2; Eq. (109), Cor. 10.17, Remark 10.32, Thms. 10.23/10.31/10.33. Live qmd retrieval checked the cited passages; existing full-paper read is recorded in prior notes.
- [Native modern producer root](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-prover/src/protocol/prove/root.rs): immutable file read; no blanket API/catalogue equivalence claimed.
- [Modern Jolt native batching specification](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/specs/akita-native-trace-batching.md); [native verifier stage](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage8/akita.rs).
- [Existing lattice outer runner](https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/examples/recursion/src/outer.rs), [grouped frontend PCS](https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/crates/jolt-verifier/src/stages/stage8/packed.rs), [pinned device semantics](https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/common/src/jolt_device.rs).
- [Nova](SOURCE_PROVENANCE.md#nova), [Lova](SOURCE_PROVENANCE.md#lova), [LatticeFold+](SOURCE_PROVENANCE.md#latticefoldplus), [SuperNeo](SOURCE_PROVENANCE.md#superneo), [SALSAA](SOURCE_PROVENANCE.md#salsaa): matched obligations and companion bounded/adaptive derivations, not a claimed Akita instantiation of their constructions.
- [Jolt](SOURCE_PROVENANCE.md#jolt), §§2.4/3.2; [Proving CPU Executions in Small Space](SOURCE_PROVENANCE.md#cpuexecinsmallspaces), §3.3. Parsed Twist formulas contain OCR errors; the RAM-end derivation uses the actual Boolean-grid increment definition and native Jolt relation rather than those malformed formulas.
- [BCMS20 published accumulation paper](https://link.springer.com/content/pdf/10.1007/978-3-030-64378-2_1.pdf), [BCLMS21 published composition paper](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_24.pdf): accumulation/knowledge and history contracts, not lattice commitment replacements.
