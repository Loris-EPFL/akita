# Akita accumulation protocol design

## 1. Decision, scope, and implementation gate

**Unselected research route.** This document specifies **Akita-LVA**, a public accumulation scheme for the acceptance predicate of completed Akita-backed proofs. LVA means *linear-verifier accumulation*. It accumulates the terminal linear equations of Akita, not its compressed commitments. The source PCS remains lattice-based. No Merkle commitment, code-proximity accumulator, elliptic-curve PCS, or unrestricted homomorphism of Akita commitments is substituted. The user explicitly rejected accumulating already completed segment proofs as the implementation target.

The arithmetic protocol, its strong completeness, and its classical random-oracle residual-soundness bound are specified below. Its integration into Jolt requires exact verifier factorization, authenticated history, an explicit original-proof security ledger, and measured performance. These are admission conditions, not results established by this document.

**Selected target:** accumulate the native instance/relation of consecutive segments before producing a full segment Akita PCS proof. Reuse Akita's existing recursive reductions where their input/output relations and bounds permit this; discharge the accumulated relation at the end. Continued PC/register/RAM/control/advice/I-O semantics remain required. Privacy and zero knowledge are not inferred as separate requirements, but a valid bounded witness relation and the required extraction/soundness argument are necessary for native relation accumulation.

The inspected Jolt lattice wrapper proves a conjunction of completed independent proofs. Akita-LVA is an alternative verifier-work construction for that setting, not the selected segment-instance construction. Its review cannot authorize substituting that easier target. The additional accumulation-weight hash in §§6–8 belongs only to this unselected route; existing Fiat–Shamir hashes have a separate challenge-generation role.

**Implementation gate: CLOSED.** No implementation branch, source change, or commit is authorized by this document for the selected target. Native old-plus-fresh re-entry needs the closure, original-handle, bounded-domain and adaptive extraction obligations in §12, plus the continued-segment relation in §13. Stopping an ordinary Akita proof at a recursive boundary does not by itself prove these properties.

Result labels:

- **Proved here:** elementary algebra and classical oracle bounds under stated hypotheses.
- **Source theorem:** the cited paper's theorem, with its original relations and model.
- **Conditional:** follows only after the listed integration/admission premises hold.
- **Open:** no proof or usable instantiation established by the inspected sources.

This is a new specification. It preserves the earlier [notes](AKITA_ACCUMULATION_NOTES.md), [solution index](AKITA_ACCUMULATION_SOLUTIONS.md), and six obligation documents.

## 2. What transfers from the papers

| Source | Relevant property | Application to Akita | What does not transfer |
|---|---|---|---|
| [BCMS20, §2.2](https://link.springer.com/content/pdf/10.1007/978-3-030-64378-2_1.pdf) | Prover/verifier/decider syntax; completeness for every decider-valid predecessor; backward atomic soundness | Defines the public predicate protocol in §§5–8 | Its discrete-log/pairing PCS accumulators are not lattice constructions |
| [BCLMS21, §§2.1–2.2](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_24.pdf) | Split instance/witness accumulation; extraction of predecessor and fresh witnesses; auxiliary-I/O and expected-time composition | Required for a genuinely private/native protocol | An atomic public residual argument is not the required private extraction theorem; its ROM-to-PCD step has an explicit concrete-hash heuristic |
| [Nova, §§4.1–4.2](SOURCE_PROVENANCE.md#nova) | Relaxed constraints, cross terms, homomorphic witness/error commitments, perfect completeness, knowledge extraction | Shows that the accumulated relation must close and include all cross terms | Replacing Pedersen with Akita's complete commitment does not preserve Nova's commitment equations |
| [Lova, §§1.2.4–1.2.5, 2.3, 3](SOURCE_PROVENANCE.md#lova) | Decomposition; exact integer Gram/norm constraints; public reducibility; EPT RoK | Useful obligations for native bounded witnesses | Its unstructured-SIS relation and integer norm proof are not Akita's Module-SIS A/B/F relation; modular Gram equalities alone are insufficient |
| [LatticeFold, §§1, 3](SOURCE_PROVENANCE.md#latticefold) | Honest norm closure is not enough; extracted source witnesses must also be bounded; challenge differences must be units | Directly identifies Akita's bounded-source and extraction problems | Raw Ajtai linearity does not establish homomorphism of Akita's decomposed/compressed commitment |
| [LatticeFold+, §§1, 4.4, 5.2](SOURCE_PROVENANCE.md#latticefoldplus) | Algebraic range proofs; transformation of nonhomomorphic double commitments; two bounded output instances | Candidate ingredients for a separately proved Akita bridge | Its commitment transformation and two-instance closure cannot be assumed for Akita without matching all relations, maps, bounds, and openings |
| [Neo/SuperNeo, §§1.2, 6–7](SOURCE_PROVENANCE.md#superneo) | Norm-preserving field/ring embedding; evaluation compatibility; strong/weak extraction composition | Requires a common full witness and an exact evaluation bridge | Constant-term projection, an arbitrary NTT embedding, or a weak decommitment does not supply a bounded Akita predecessor |
| [Cyclo, §§1.1, 2.5](SOURCE_PROVENANCE.md#cyclo) | Omitting receiver range checks can give bounded-depth additive norm growth; refresh must be priced | A possible bounded-epoch alternative, with explicit maximum depth and authenticated refresh | Its norm recurrence and supported fold counts are not Akita parameters or an unbounded-depth theorem |
| [SALSAA, §§5.2–5.3](SOURCE_PROVENANCE.md#salsaa) | Composable bounded linear relations; controlled witness/row sizes; logarithmic-depth extraction | Guides relation retirement and extractor accounting | Its complete reduction chain and SIS inventory do not apply to a new Akita topology automatically |
| [Accumulation Without Homomorphism, §§2.1–2.2](https://drops.dagstuhl.de/storage/00lipics/lipics-vol325-itcs2025/LIPIcs.ITCS.2025.23/LIPIcs.ITCS.2025.23.pdf) | Homomorphism is not universally necessary; code distance and local openings can replace it | A comparison and a statement of different sufficient hypotheses | Akita is not a Merkle vector commitment with those local-opening/proximity guarantees; bounded-depth code soundness does not imply lattice norm closure |
| [WARP, abstract](https://eprint.iacr.org/2025/753) | Hash/code accumulation with straightline erasure-based extraction and unbounded depth | A distinct design family, not used by Akita-LVA | Neither its code relation nor its extractor is instantiated by Akita; only the abstract was obtained here |
| [Halo Infinite, §§3–5](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_23.pdf) | Aggregation requires a valid linear-combination/opening relation | Explains why a commitment bridge must be proved | A black-box additive/homomorphic-PCS compiler cannot be invoked when its opening-group and binding hypotheses fail |

**Common obligations, not interchangeable constructions:** fixed live state, strong completeness, a correct accumulated relation, authenticated statements/setup, fresh challenge binding, sound final decision, appropriate knowledge extraction when witnesses are hidden, a lifetime error ledger, and costs for the complete receipt. Zero knowledge and QROM security are separate properties.

Akita-LVA uses the first row's **atomic public predicate** model. Its accumulator witness is empty: all arithmetic state is public. It does not claim the stronger private/split property by renaming public vectors as witnesses.

## 3. Akita-specific facts

### 3.1 The complete commitment is not field-linear

Schematically, Akita forms

$$
C=\operatorname{Compress}_{F}\!\left(B\,\operatorname{Digits}(A\,\operatorname{Pack}(w))\right),
$$

with exact source layouts, digit alphabets, slices, and compression chains. D/H paths authenticate opening-side material. The A, B, F, D, and H maps are locally linear in their own inputs. Digit decomposition and the composition of these maps are not a field-linear commitment to the original vector.

A carry already defeats the inference. In balanced base four,

$$
\operatorname{Digits}_4(1)=(1,0),\qquad
\operatorname{Digits}_4(2)=(-2,1)\ne(2,0).
$$

A toy outer row selecting the low digit gives $C(1)+C(1)=2$ but $C(2)=15$ modulo 17. This is an algebraic carry counterexample, not a secure parameter example or an attack on Akita. Consequently, neither

$$C'=C_1+\alpha C_2$$

nor a summed opaque commitment hint is an admitted Akita commitment operation.

Source evidence: [Akita §§4–6](SOURCE_PROVENANCE.md#akita); `akita-cpu-backend/src/arithmetic/commitment_stage.rs`; [bounded commitment consistency](AKITA_ACCUMULATION_03_COMMITMENT_CONSISTENCY.md).

### 3.2 Exact terminal predicate

For an authenticated terminal source state, Akita §8.2 checks

$$
A_{\rm term}z=\sum_{i<B_{\rm term}}c_i t_i \quad\text{in }R_q^{n_A},
\qquad R_q=\mathbb F_q[X]/(X^D+1). \tag{A163}
$$

Here $z$ is one folded response; $i$ indexes its incoming source blocks, not source proofs. The original $c_i$ are scheduled small ring challenges. Their multiplication is **negacyclic ring multiplication**, not coefficientwise field multiplication.

The same terminal retains

$$
\sum_i c_i e_i=a_{\rm term}^{\mathsf T}Gz, \tag{A164}
$$

and its tensor/trace evaluation equation (Akita Eq. 165). It retains response decoding, coefficient/physical squared-norm bounds, challenge validity, nonces, tensor factors, layouts, and all predecessor authentication. An unfinished setup opening cannot survive into this terminal. Omitting A163 does not justify omitting A164 or A165.

At the fitting historical companion, the comparison is `protocol/core/terminal_direct.rs::check_a_rows`; `verify_terminal_ring_relations` also checks A164. `suffix.rs` then checks the terminal trace, and `verify.rs::batched_verify` finishes transcript replay. Upstream main moved these responsibilities into `terminal/direct.rs` and `fold/{terminal,verify}.rs`.

### 3.3 Fixed coefficients do not imply a fixed shaped matrix

A physical coefficient is addressed as

$$\operatorname{offset}(r,c,k)=(r\,W+c)D+k.$$

Same seed and coefficient stream with different row widths $W$ need not give compatible linear maps. For example,

$$
A_1=\begin{pmatrix}1\\2\end{pmatrix},\qquad
A_2=\begin{pmatrix}1&2\\3&4\end{pmatrix}
$$

come from one stream, but $A_1(1)=(1,2)$ and $A_2(1,0)=(1,3)$. Naive padding is unsound. Cache coverage is not an embedding proof.

**Decision:** different shaped maps occupy different lanes. No cross-width, cross-ring, cross-field, or cross-setup merging is permitted without an explicit commuting-map proof and a new cost analysis.

## 4. Predicate factorization and admission

Let $P$ be a trusted, fixed protocol profile. A source input $u$ contains a complete source statement, device/public I/O, completed proof bytes, and any required authenticated source context. Define

$$\Phi_P(u)=\operatorname{Admit}_P(u)\land V_{\rm full}(P,u).$$

Admission restricts the exact supported source/setup/catalogue/security domain. It cannot be used to silently discard honest workloads required by the application. Unsupported workloads are reported, not relabeled invalid computation.

A retained verifier returns either rejection or a **complete ordered list**

$$\mathcal O(P,u)=\bigl[(\ell_o,x_o,y_o)\bigr]_{o=0}^{b(u)-1}$$

of deferred terminal obligations. An occurrence $o$ corresponds to one native backend invocation, not necessarily one Jolt proof. Advice and committed-program openings may cause additional invocations. Multiple occurrences can name the same lane.

The mandatory exactness contract is

$$
\Phi_P(u)=V_{\rm ret}(P,u)\land
\bigwedge_{o<b(u)}[A_{\ell_o}x_o=y_o]. \tag{F}
$$

For A163, $x_o=z_o$ and $y_o=\sum_i c_{o,i}t_{o,i}$. Both are derived by verifier replay. A prover-supplied $(x,y)$ without this replay is not admitted.

$V_{\rm ret}$ preserves:

1. Every Jolt frontend stage and original public-statement/transcript binding.
2. Source roles, group order/count, commitments, points, evaluation vectors, and advice/program opening obligations.
3. All Akita root/intermediate/range/binary/norm/ring-switch/setup-offloading checks and their exact source identities.
4. A164, A165, nonce/sampler/tensor-factor rules, all original z bounds and decoder budgets.
5. All original matrix-application **preconditions**: dimensions, supported dispatch, authenticated cache/setup, required prefix coverage, checked arithmetic, and CRT capacity where the original path requires it. Skipping a comparison must not turn an original backend-precondition failure into success.
6. Completion of every original proof stream and transcript. No pending result escapes before final trace checks and EOF.

A code refactor must prove (F) for every admitted dispatch path, including zero-recursive-fold schedules, and inventory every native call. Ordinary public full-verification APIs still return full acceptance. A distinct retained-check result must not be confused with `Result<()>` from full verification.

## 5. Indexed profile and accumulator relation

### 5.1 Profile

$P$ fixes:

- One base field $\mathbb F_q$ and its canonical encoding; mixed fields use separate protocols.
- Exact original PCS/frontend versions, setup derivation, selected catalogues/schedules, security policy, source roles, and trusted program/preprocessing policy.
- A finite ordered lane catalogue. Each lane fixes $D_\ell$, rank $r_\ell$, width $W_\ell$, physical offsets/row stride, coefficient order, negacyclic convention, and authenticated matrix bytes or their validated derivation.
- Maximum admitted source size, native-call count, parsing/decoding budgets, and all original source bounds.
- Number $k$ of independent residual channels; challenge sampler; domain tags; lifetime query/error budgets.
- Exact safe decision arithmetic and its checked capacity.
- Receipt version, public statement policy, maximum run length and resource bounds.

The profile identifier is bound to this complete trusted profile. It is not an attacker-controlled label for unrelated matrices. If a digest is used, add its binding failure to the ledger. Prepared matrices/NTT payloads are verifier-owned authenticated data, not trusted solely because a source supplies a seed.

No new SIS hypothesis is needed for the public arithmetic state. Original source proofs retain their original SIS and Fiat–Shamir assumptions. The public state is not an Ajtai commitment to a short witness.

### 5.2 State and decision relation

For each lane and channel,

$$
U_{\ell,c}\in R_{q,D_\ell}^{W_\ell},\qquad
Y_{\ell,c}\in R_{q,D_\ell}^{r_\ell},\qquad 0\le c<k.
$$

The state is

$$a=(P_{\rm id},(U_{\ell,c},Y_{\ell,c})_{\ell,c}).$$

Define

$$
D_P(a)=\operatorname{CanonicalShape}_P(a)\land
\bigwedge_{\ell,c}[A_\ell U_{\ell,c}=Y_{\ell,c}]. \tag{D}
$$

All state coefficients are unrestricted field elements. **No original z norm bound is imposed on $U$.** It usually fails after a field-weighted update.

The maximum arithmetic state is independent of the history length:

$$
|a|_{\rm field}=k\sum_\ell(W_\ell+r_\ell)D_\ell.
$$

This can still be large. Fixed size does not mean 128 bytes or a small circuit.

Operational history state—run identifier, record count, predecessor link, application boundary, and deferred-obligation ledger—is separate from $a$. In particular, a terminal run-length cap is **not** built into the arithmetic decider. Otherwise a decider-valid state at the cap could not accumulate a valid next input, contradicting unrestricted strong completeness. The bounded application relation only permits updates inside its declared run domain.

## 6. Challenges: full binding and independent occurrences

### 6.1 Framing

Before deriving any accumulation weights, the verifier fixes an injectively encoded frame

$$
F=\operatorname{Enc}({\tt Akita\!\!\!-LVA/frame},P_{\rm id},a,u,
\mathcal O(P,u),\operatorname{Context}). \tag{Frame}
$$

Context binds the receipt/run/ordered record position and application metadata used by the surrounding relation. Lengths and integer widths are explicit; field elements are canonical; the occurrence list is ordered and derived, not user-selected. No accumulation challenge can influence a subsequently chosen response, matrix, lane, source proof, or context in this frame.

Use independent indexed 512-bit accumulation blocks whose keys contain the **complete frame**, not a digest of it:

$$B_j=H_{\rm acc}(\operatorname{Enc}({\tt Akita\!\!\!-LVA/weights},P_{\rm id},F,j)).$$

The setup, original-proof FS, profile-binding, and accumulation namespaces are distinct. The deployed candidate is domain-separated Blake2b512, compatible with the fitting Blake branch. Treating the concrete hash as ideal is a stated heuristic, not a consequence of collision resistance alone.

An arbitrary oracle prequery at a well-formed block key already fixes the entire frame and its residuals. Replacing $F$ by $H_{\rm bind}(F)$ would lose this property: a later frame could hit a digest queried before its residuals were chosen. A collision-between-frames bound alone would not cover that guessed-digest event. The specified protocol does not make that replacement.

Hash the full encoded prefix through $F$ incrementally without copying its buffers. A verifier-owned incremental Blake2b state may be cloned before appending the canonical fixed-width block index $j$ and finalizing; this computes exactly the specified full-message hash. Never accept a prover-supplied cached hash state or transmit a prefix digest in place of $F$. Do not replace the frame with an original transcript tail unless a separate proof shows that the tail fixes every required old-state/input/derived obligation. A hash chain alone is not proof of valid updates.

### 6.2 Sampler

For the fitting fp128 field,

$$q=2^{128}-2^{32}+22537,$$

choose $k=2$. Assign one independent 256-bit chunk to every pair $(c,o)$, using canonical occurrence-major/channel-major order across the indexed blocks. For chunk integer $t$,

$$\alpha_{c,o}=\begin{cases}t\bmod q,&t\bmod q\ne0,\\1,&t\bmod q=0.\end{cases}$$

There is no sampler abort or response grinding. The weights are nonzero but **not exactly uniform**.

Let $M=2^{256}$ and $M=aq+r$, $0\le r<q$. The largest probability mass is at the merged residues zero and one:

$$
\mu=\frac{2a+\mathbf1_{r\ge1}+\mathbf1_{r\ge2}}{M}
\le\frac{2\lceil M/q\rceil}{M}. \tag{Mass}
$$

Use this exact mass in the theorem. For this field it is approximately $2^{-127}$, so two independent channels give approximately $2^{-254}$ per fixed candidate frame before lifetime and separate profile/setup/receipt terms. This does not give 254-bit security to the underlying source proofs.

Other fields require a separately admitted $k$ and chunk width. Never reuse the fp128 two-channel claim for fp32/fp64.

### 6.3 Why each occurrence needs its own weight

Do not add same-lane source equations before sampling independent weights. Residuals $e_1=-e_2\ne0$ cancel identically if they receive the same weight. Independent $(c,o)$ weights prevent that deterministic failure. Two channels alone do not repair shared weights within one channel.

## 7. Algorithms

### Index

Validate and freeze $P$; resolve the finite lane catalogue; authenticate source setup/cache material; certify original profile/security admission and decision arithmetic. Produce proving/verifying/decision keys referring to the same $P$. Reject profiles with unpriced source paths, unsupported matrix views, unsafe kernels, or an unmet security budget.

### Init

Output $U_{\ell,c}=Y_{\ell,c}=0$ for every lane/channel. Thus $D_P(a_0)=1$.

### Accumulate

Input: arithmetic state $a$, completed source input $u$, and bound context.

1. Check canonical state shape and replay $V_{\rm ret}(P,u)$ to derive the complete ordered occurrence list. Reject on any retained failure.
2. Form the full fixed frame and derive independent $\alpha_{c,o}$.
3. For each lane/channel, compute

$$
U'_{\ell,c}=U_{\ell,c}+\sum_{o:\ell_o=\ell}\alpha_{c,o}x_o,
\qquad
Y'_{\ell,c}=Y_{\ell,c}+\sum_{o:\ell_o=\ell}\alpha_{c,o}y_o. \tag{Update}
$$

4. Output canonical $a'$. The auxiliary arithmetic accumulation proof is empty: updates are public and recomputable. Source proofs are not omitted.

### VerifyUpdate

Recompute all retained checks, occurrences, context, challenges, and updates from $(P,a,u)$. Accept exactly if canonical $a'$ equals the computed result. Do **not** run the decider per update. The verifier is allowed to receive an old state whose decider is false; backward soundness must cover that case.

### Decide

Validate canonical shape and matrix provenance; compute every equation (D) using the admitted unrestricted-safe arithmetic in §10. Accept iff all hold. Missing lanes, altered channel counts, or substituted profiles reject.

### ReplayReceipt

Check a nonempty, bounded ordered source record stream; initialize $a_0$; check every retained source frontend and update; enforce the chosen statement/link/ledger policy; decide $a_T$; require whole-stream EOF and the exact accepting result. Replay can recompute states rather than transport all intermediate states. The source archive remains linear in $T$.

### Final lattice wrapper

The selected Jolt candidate would prove one execution of `ReplayReceipt`, using the same lattice outer frontend/PCS family as the reference wrapper. Its final verifier must bind:

- approved guest/program/preprocessing/profile/setup policy;
- exact intended source statement list under the declared source I/O semantics;
- nonempty record count and complete framed input;
- no panic and a **canonical accepting output equal to one**;
- successful final decision and an empty deferred-obligation ledger.

Verifying an outer proof of a guest that returns zero is not accepting the accumulated statements. Process success and proof validity are different predicates.

**Composition condition:** the outer proof must soundly authenticate this complete execution and its public records. The authenticated-replay theorem below then applies. The existing ROM theorem for a primitive does not automatically prove a non-black-box concrete-hash wrapper theorem; see §9. This wrapper is a final compression step, not an online recursive receipt compiler.

## 8. Correctness and residual soundness

### 8.1 Strong arithmetic completeness — proved here

For **every** canonical $a$ with $D_P(a)=1$, and every $u$ with $\Phi_P(u)=1$, each $A_{\ell_o}x_o=y_o$. Linearity of the fixed matrix map gives

$$
A_\ell U'_{\ell,c}-Y'_{\ell,c}
=(A_\ell U_{\ell,c}-Y_{\ell,c})+
\sum_{o:\ell_o=\ell}\alpha_{c,o}(A_\ell x_o-y_o)=0.
$$

Thus honest updates pass `VerifyUpdate` and `Decide`, regardless of how the valid predecessor was formed. This is perfect arithmetic completeness. It does not assume that $U$ is short or canonical gadget material. Original proof-generation failure probabilities remain those of Akita; no new sampler rejection is introduced.

### 8.2 Interactive backward atomic soundness — proved here

Flatten ring coefficients into vector spaces over the **base field**. For channel $c$, let $r_c$ be the direct sum of predecessor residuals and let $e_o$ be the corresponding lane-embedded input residual. Then

$$r'_c=r_c+\sum_o\alpha_{c,o}e_o.$$

If every $e_o=0$ and some $r_c\ne0$, the new decider cannot accept. Otherwise select one nonzero $e_{o_*}$. Condition on all other weights. For each channel, at most one value of $\alpha_{c,o_*}$ solves $r'_c=0$. If two did, their nonzero **field-scalar** difference would annihilate $e_{o_*}$, impossible in a field vector space. The quotient ring may have zero divisors; this proof does not require arbitrary ring challenges to be units.

Since selected weights are independent across channels,

$$
\Pr[D_P(a')=1\ \land\ (D_P(a)=0\lor\Phi_P(u)=0)\ \land\operatorname{VerifyUpdate}=1]
\le\mu^k. \tag{Atomic}
$$

Retained-check failures reject deterministically. This theorem is about the **public full-verifier predicate**. It is not extraction of hidden bounded Akita witnesses.

### 8.3 Classical random-oracle lifetime bound — proved here, under framing

Condition on the independent original-proof/setup oracle tables. The complete frame is part of every accumulation block key, so even an arbitrary prequery fixes that frame's predecessor/input residuals before receiving its first accumulation answer. Keys that do not encode a well-formed full frame cannot later become that frame's keys because the encoding is injective. Even if indexed blocks are queried separately, imagine the frame's entire independent block table sampled at first touch. The selection of a bad tuple for that frame is already fixed; adaptive partial queries and abandoned candidates do not evade the per-frame bound.

Let $Q_F$ count distinct candidate frames touched by the adversary across all runs, profiles, retries and abandoned inputs. Let $U_F$ count distinct final-output frames not previously touched. A raw coordinate/block-query budget is a conservative upper bound on $Q_F$; do not confuse coordinates with independent completed frames. Include final verifier evaluations, even if the adversary never queried them.

With no profile/setup/receipt failure,

$$
\varepsilon_{\rm LVA}\le(Q_F+U_F)\mu^k. \tag{Lifetime}
$$

There is no separately compressed frame-binding key, so no frame-digest collision or guessed-digest hit term is needed in this elementary theorem. Profile/setup/receipt authentication still has its own charges. Concrete deployment still requires the stated ideal-oracle heuristic; collision resistance alone is insufficient.

An authenticated history that starts at $a_0$, contains an invalid source predicate, and finishes with a valid decider must contain an atomic bad-input/bad-predecessor-to-valid-output event. Union over candidate frames proves the history claim. Do not multiply an already global $Q_F$ by $T$ again.

For illustration only, $Q_F+U_F\le2^{64}$ with the fp128 two-channel sampler gives approximately 190 residual-error bits before frame/receipt/original-proof terms. The security of the whole proof remains limited by those other terms.

## 9. Original-proof security, extraction, privacy, and quantum limits

The complete conditional ledger is

$$
\varepsilon_{\rm total}\le
\varepsilon_{\rm LVA}+
\varepsilon_{\rm profile/setup\ binding}+
\varepsilon_{\rm source/frontend}+
\varepsilon_{\rm receipt}+
\varepsilon_{\rm statement/link}.
$$

Original Akita schedule admission includes its setup-expansion distance, all comparison-class Module-SIS instances, public claim/tensor/range/relation/terminal errors, challenge filtering, and original FS query loss. Source proofs sharing matrix views are not assumed independent. Use an applicable adaptive multi-instance theorem or a justified global reduction/union budget.

[Akita Theorem 10.33 and Corollary 10.34](SOURCE_PROVENANCE.md#akita) give, in the classical ROM,

$$
(Q+1)\varepsilon_{\rm int}(\mathrm{sch})+\Delta_{\rm setup}
+\sum_I\operatorname{Adv}_{\rm MSIS,I},
$$

with an admitted complete-tree extraction bound. If $\varepsilon_{\rm int}\ge c/|E|$, then

$$
\lambda_{\rm FS}^{\rm eff}\le\log_2|E|-\log_2 c-\log_2(Q_{\max}+1).
$$

A production policy named quantum-128 for lattice attack estimation is not a 128-bit ROM/QROM theorem for every composed protocol. Larger accumulation channel entropy does not repair an inadequate original source or outer profile.

The public arithmetic has an empty hidden witness. It needs no rewinding extractor. If an application claims knowledge of hidden execution witnesses, that claim must come from a valid frontend/PCS/receipt extraction theorem; public predicate acceptance alone does not supply it.

A single final proof of deterministic replay avoids nesting a new Akita opening extractor for every accumulator step. It still needs the outer argument's actual soundness/knowledge and statement-binding premises. Generic online private PCD additionally requires appropriate expected-time, auxiliary-input/output extraction. [BCLMS21 §2.2](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_24.pdf) explicitly identifies this distinction. Do not turn the classical ROM primitive theorem into a proved standard-model non-black-box compiler merely by replacing an oracle with Blake2b code.

**Privacy:** no zero-knowledge theorem is claimed. State, completed source proofs, and runtime source record bytes are public in the selected wrapper. ZK needs its own simulator and hiding/masking construction.

**Quantum:** no QROM theorem is claimed for Akita-LVA, Akita's coordinate-wise extraction tree, or their receipt composition. Quantum lattice attack estimates do not imply quantum FS extraction. Hash/code papers' plausible post-quantum language does not remove this obligation.

## 10. Correct decision arithmetic

### 10.1 Direct reuse of small-RHS CRT is invalid

A terminal z is bounded signed i16 material. $U=\sum\alpha z$ normally has full-field coefficients. Existing `mat_vec_i16`, IFMA small-RHS routines, and i8 digit LUTs cannot consume $U$ as if it remained short.

The exact CRT contract is

$$
2W D\lfloor q/2\rfloor B_{\rm rhs}<\prod_j p_j. \tag{CRT}
$$

For full fp128 centered coefficients, $B_{\rm rhs}=\lfloor q/2\rfloor$. Even $WD=1$ requires a bound of bit length 255. The inspected six-prime inventory has product bit length 180; with tail prime 12289 it has bit length 194. It cannot reconstruct full-field products exactly. The current bound API also takes a u64 RHS bound, not a full fp128 coefficient bound.

The `Q128_MODULUS` constant in the inventory is $2^{128}-275$, not the selected proof field. Use the actual field modulus and `CrtCapacity` contract. Two comments still describe five Q128 primes although the array contains six; those comments are stale, not a five-prime implementation.

### 10.2 Correct reference decision

Use the existing unrestricted native-field negacyclic schoolbook operation, `CyclotomicRing::mul_accumulate_into`, with exact lane geometry. It costs

$$O\!\left(k\sum_\ell r_\ell W_\ell D_\ell^2\right)$$

field operations. Reuse destination/scratch buffers. This is a correct reference path, not presumed fast enough.

### 10.3 Candidate fast decision: certified digit planes

For each centered coefficient $u\in[-(q-1)/2,(q-1)/2]$, compute an exact balanced-radix-256 expansion

$$u=\sum_{j=0}^{L-1}256^j u_j,\qquad u_j\in[-128,127].$$

For the selected fp128 field choose **$L=17$**. Sixteen planes do not cover the complete positive centered domain:

$$
127\frac{256^{16}-1}{255}<\frac{q-1}{2},
\qquad
127\frac{256^{17}-1}{255}>\frac{q-1}{2}.
$$

The negative capacity is at least the positive capacity. Use exact integer carry/recomposition, not a truncated top digit. Reject a selected profile unless each plane passes (CRT) with $B_{\rm rhs}=128$ and the actual matrix width/ring/modulus/prime inventory.

Apply the existing safe small-RHS matvec to each plane and recombine in the source field:

$$A_\ell U_{\ell,c}=\sum_{j<17}256^j(A_\ell U_{\ell,c,j}).$$

This is a deterministic arithmetic identity, **not** a range proof or a new lattice commitment. The decision key authenticates the NTT material. A missing/unsupported kernel cannot silently fall back to an unsafe interpretation. The schoolbook path remains the independent reference.

The adapter, signed decomposition API compatibility, guest field/NTT inline support, and actual costs at the historical pin remain to be exercised. No new fast-kernel performance claim is established.

## 11. Receipt and application semantics

### 11.1 Existing conjunction target

The inspected guest runs the full Jolt verifier independently for each completed proof and returns their conjunction. For this target, the source statement is an ordered list of complete executions under approved preprocessing and the existing public I/O semantics.

The cutover must enforce nonempty input, complete record framing and zero padding, authenticated embedded/input setup, all proof/device records, exact intended statement policy, final decision, and canonical accepting output. The control implementation must enforce the same contract when comparing performance.

Current source issues relevant to this boundary:

- The guest's `all_valid=true` initialization accepts a framed zero proof count.
- The outer runner checks absence of panic but does not require the guest output to equal one.
- Per-record decoding checks consumption; the guest does not enforce whole-stream completion or zero record padding.
- Detached expanded setup/cache payloads are trusted; provenance must be external or bound into the approved image.
- Generic proof verification does not enforce an application-specific input/result list.
- The modern source frontend trims trailing zero output bytes. Exact-length journals require a separately declared relation; stricter wrapper framing alone does not change this equivalence.

These are source-inspected findings, not newly reproduced runtime failures. No claim is made that malicious acceptance has been executed here.

### 11.2 Public transport remains linear

At the fitting wrapper, all source proof/device records enter the outer public input. Total transport includes

$$|\pi_{\rm outer}|+|\mathrm{device}_{\rm outer}|+|\mathrm{setup/ELF\ policy\ material}|,$$

not just $|\pi_{\rm outer}|$. The device contains source record bytes, so the receipt is not constant total size in $T$. Moving these bytes to private advice would require a new statement-binding/privacy relation and is not assumed here.

## 12. Native/private Akita accumulation: exact research obligations

This section records the earlier native research obligations. The user has now selected native instance/relation accumulation before a full segment PCS proof, not this document's public-LVA route. The current specification and source-reuse map are in [the native instance design](AKITA_INSTANCE_ACCUMULATION_DESIGN.md). Privacy is not a separate inferred requirement. The native route remains **not admitted for implementation**.

### 12.1 Required bounded relation

Fix complete receiver capacity $N_*$ and certified alphabet $\mathcal D_{b_*}$. A candidate relation is

$$
\mathcal R_{\rm acc}^{\rm bd}(P,(C,r,v);(w,d)):
\begin{cases}
|w|=N_*,\quad w\in\mathcal D_{b_*}^{N_*},\\
\text{all required physical padding/structural cells are valid},\\
s_*(d)=\operatorname{Pack}_P(w),\\
\operatorname{Rel}_{\rm com}^{\rm Akita}(P,C;w,d),\\
\widetilde w(r)=v.
\end{cases} \tag{NativeRelation}
$$

The candidate receiver requires an identity/one-digit source map $s_*$ on its stored, packed source coordinates. This layout must be instantiated and certified in $P$; a general gadget decoder is not an identity map. `Rel_com` is the actual full A/B/F source/decommitment path. Fresh original commitments retain their own exact geometry, source maps, and separately certified bounded source relations. An extracted Akita weak algebraic decommitment need not be canonical or short after division by a challenge difference; [Remark 10.32](SOURCE_PROVENANCE.md#akita) explicitly says so.

### 12.2 Required update

Consume one bounded old receiver and bounded fresh original opening handles; bind all original statements/setup/context; run a correctly instantiated relation/range reduction; form the **complete** Eq. 109 successor; return one successor of the same receiver relation. Count responses, partials, images, quotients, F/H segments, ring completion and padding. Retire all old relation rows and setup obligations. Never replace this operation with addition of compressed commitments.

All-valid-source completeness needs universal coefficient/operator envelopes, not honest variance or sampler success estimates alone. Representative requirements include

$$U_g=\sum_a K_{1,a}B_a,\qquad
E_g=\left(\sum_a\Gamma_a\sqrt{E_a}\right)^2,$$

and a valid signed decomposition covering every admitted integer lift. Any Gram/energy certificate must mean the exact integer norm, with proved no-wrap where modular constraints are used.

The earlier [closure analysis](AKITA_ACCUMULATION_01_FIXED_SIZE_CLOSURE.md) derives one sufficient fixed-point slab

$$
\gamma_*<c_*,\qquad
\left\lceil\frac{a_*D+C_D}{c_*-\gamma_*}\right\rceil
\le B\le
\left\lfloor\frac{T_f(D)}{P_*K_{1,*}(b_*/2)}\right\rfloor,
\qquad N_*=c_*B.
$$

Every symbol must be instantiated from the actual complete successor and secure matrix/challenge profile. Its earlier dimension-64, $\ell_1$-radius-16 example has fewer than $2^{69}$ challenges and is **not** a 128-bit profile. Increasing challenge norms or ranks changes closure, compression, setup and cost; rerun all admission conditions together.

### 12.3 Missing extraction bridge

A useful private update needs an independently justified reduction

$$
\mathcal R_{\rm acc}^{\rm bd}\times\mathcal R_{\rm fresh}^{\rm bd}
\longrightarrow\mathcal R_{\rm acc}^{\rm bd},
$$

with public reducibility, all-valid-predecessor completeness, bounded predecessor/fresh witness extraction, consistent original commitment projections, and a sound extraction/composition bound for the declared supported depth, including required auxiliary I/O and expected-time adversaries. A **uniform polynomial in arbitrarily growing depth** is a stronger research target, not an extra user requirement. A finite authenticated maximum depth still needs its complete tree, reduction-runtime, sampler, query/error and Module-SIS admission; simply calling the depth fixed does not establish concrete security.

Akita's complete transcript-tree factor is

$$\operatorname{Tree}(\mathrm{sch})=\prod_{r\in\operatorname{Rounds}(\mathrm{sch})}b_r.$$

Its finite-schedule extractor is polynomial in that product and explicit witness size. Repeating a nontrivial factor at every update can give $(1+W)^T$. Polynomial in that tree is not polynomial in $T$. Original application groups occur at the root; repeated old-plus-fresh re-entry is a new topology.

**Possible routes, each with its own required proof:** an independently extractable local bounded bridge; an exact LF+/SuperNeo commitment transformation after proving relation compatibility; or an authenticated bounded-epoch refresh. Merkle/code spot checks do not fill this gap for lattice commitments. The current sources do not establish a secure, cheap, private long-chain Akita bridge.

### 12.4 Fully revealing certification is not the desired solution

Providing complete bounded old/fresh/new $(w,d)$ certificates and checking their commitment/evaluation/range relations at every step makes local witness extraction direct. It also discloses witnesses and costs at least linear reading/evaluation work in their complete sizes. A certificate only at the final state does not certify every counterfactual child of an earlier extractor.

For a fresh source of size $N$, direct certification has $\Omega(N)$ per-update input/work, whereas Akita's source verifier has sublinear $\widetilde O(N^{1/K})$ setup-offloading regimes. Thus this revealing repair does not provide an asymptotic parity claim for large source sizes. This is a reason to reject that repair, **not** an impossibility theorem for every lattice accumulation construction.

## 13. Continued zkVM segments: separate relation required

No inspected lattice wrapper supplies a continued-segment relation. Akita PCS folds and native trace-column batches are not VM segments.

A true execution accumulation target must bind a boundary

$$B_t=(\mathrm{PC},\mathrm{registers/control},\mathrm{RAM\ endpoint},
\mathrm{retirement/cycle\ counters},\mathrm{input/advice\ IDs\ and\ cursors},
\mathrm{output\ journal},\mathrm{status})_t$$

and enforce

$$
\operatorname{Init}(B_0)\land
\bigwedge_{t<T}\operatorname{Segment}(P,B_t,B_{t+1};w_t)
\land\operatorname{HaltSuccess}(B_T). \tag{Execution}
$$

Segments need legal completed-instruction cuts, active-prefix/inert-padding constraints, exact ordered adjacency, approved bytecode, no fresh RAM reset between segments, correct read/write endpoint transitions, and exact declared I/O/journal semantics. All endpoint/opening obligations belong in the retained/deferred ledger and must be discharged at the end.

The [local Jolt paper](SOURCE_PROVENANCE.md#jolt), Definition 3.1 and §§2.4/3.2, models the state as PC, registers, and byte-addressable memory, with program code, registers, and RAM checked as separate memory spaces. A segment frontend must connect those checks to authenticated start/end boundaries; matching only PC and register values does not establish RAM continuity. Legal cuts must follow the fitting branch's actual ISA and virtual-instruction expansion, not the paper's simplified instruction-format overview.

Akita-LVA could accumulate completed proofs of this segment relation once that frontend is sound and available. Its public predicate theorem does not require native/private witness folding. The one-final-replay construction in §7 would then be an offline execution receipt, not online IVC. An online proof-carrying update compiler additionally needs its own compatible succinct-argument knowledge, auxiliary-I/O, history, and concrete-oracle composition theorem; none follows merely from the terminal residual bound.

The [small-space CPU paper](SOURCE_PROVENANCE.md#cpuexecinsmallspaces), §§1.2–1.3, studies streaming/checkpointing of Jolt's sum-check and memory-checking components and explicitly makes PCS-dependent efficiency distinctions. Those comparisons are not an Akita compatibility or accumulation theorem. Streaming without recursion is a comparison route, not a substitute for the requested accumulation design.

A finite-RAM lattice endpoint construction must declare its capacity and price its memory PIOP and endpoint openings. A Merkle RAM construction introduces a separate hash-authentication assumption and circuit cost; it is not a lattice accumulation theorem. Neither option is implemented by merely batching terminal A equations.

See [execution-composition obligations](AKITA_ACCUMULATION_05_EXECUTION_COMPOSITION.md). A performance comparison for continued segments requires a full-verification control of this **same new execution relation**, not the existing independent-execution conjunction.

## 14. Source bases and integration contract

| Purpose | Jolt source | Akita companion | Decision |
|---|---|---|---|
| Existing real lattice outer conjunction | `origin/wrap/fast-recursion-blake`, `5c65167e269fdfc2917ecb82f420d349b18b4c87` | `markosg04/akita`, `d0ee56a0b12db66b4c46a87c527169737694c4c7` | Fitting reference/candidate base for the existing public conjunction |
| Modern native column batching | `origin/amir/akita-batch`, `0791f79a55c3fadc3d327c78a72228696c338468` | `LayerZero-Labs/akita`, `83574331d4e51f8cce1d8689d05592fa4ef4c138` | Better modern frontend seam; not an already validated combined outer delivery |
| Fast-recursion trace only | `141460a7ffac436f4c533c684d31acaf8aaa8ace` | `22530c7b774523078f745b0560403d5c330618ee` | Not a complete lattice outer-proof baseline |

The parent verified the first Jolt pins using local `git rev-parse`, read its manifest and guest via `git show`, fetched only the immutable companion object, and read its terminal implementation. No branch was created or checked out. The companion was absent locally before that fetch. Origin remotes were not changed or pushed.

The Blake source uses epoch-six Blake2b challenge streams, legacy packed inner proofs, and a separate modular q128/field-inline outer profile. Do not mix current main's APIs, schedules, field source package identities or setup serialization into this pair by assumption. The correct native terminal module at the companion is under `protocol/core`, not upstream main's later directory layout.

For the unselected public-conjunction route only, if separately approved and all gates pass:

1. Create local accumulation branches from this exact compatible pair; preserve an exclusive matching-source full-verifier control.
2. Add a distinct retained-verification path at the exact terminal comparison, preserving (F) and full public verifier semantics.
3. Implement the admitted public state, independent occurrence/channel challenge schedule, and exact full-field decision.
4. Integrate the retained backend obligations into the complete Jolt verifier path and replace the conjunction guest loop's repeated A comparisons with updates plus one decision.
5. Enforce the full receipt acceptance contract in both the candidate and fair reference control. Update every affected caller, format/contract and documentation; no unchecked pending result at public acceptance boundaries.
6. Run actual source generation, guest execution, lattice outer proving and independent final verification before claiming delivery.
7. Commit coherent local increments with hooks enabled. Do not push or change original remotes. Fork remotes can be selected later by the user.

These are conditional integration steps, not a claim that branches or code already exist.

## 15. Performance model and stop rule

Let $v_{0,j}$ be retained verification cost, $m_j^{\rm short}$ original bounded terminal matvec cost, $h_j$ full frame/challenge cost, and $u_j$ public update cost. For one final wrapper,

$$
C_{\rm reference}=C_{\rm parse/setup}+\sum_j(v_{0,j}+m_j^{\rm short})
+C_{\rm outer}(\mathrm{reference\ execution}),
$$

$$
C_{\rm LVA}=C_{\rm parse/setup}+\sum_j(v_{0,j}+h_j+u_j)
+C_{\rm decide}^{\rm full}
+C_{\rm outer}(\mathrm{LVA\ execution}).
$$

The useful difference is not simply $T-1$ matrix checks. Full-field state costs more than bounded z. Two channels and 17 digit planes give up to **34 small-RHS plane matvecs per active lane** in the conservative fast decision, plus recomposition. Source matrices must actually share a lane. Hashing old state/source proofs/derived equations, public updates, decoding, setup/ELF transport, and padded outer trace size can dominate any saving.

No performance parity has been measured. In particular, neither small $T$ nor heterogeneous matrices have a parity guarantee. A lane unique to one source pays an unrestricted decision for that source with additional update/hash work. General commuting embeddings would have to prove correctness and justify their enlarged state and decision cost.

### Required paired experiment

Freeze the same source instances, proof bytes, source/public I/O semantics, security policy, authenticated setup mode, hardware/thread settings, and receipt acceptance policy. Compare complete executions, not a partial terminal benchmark alone. Measure:

- active and padded guest trace rows, including parsing/setup;
- source generation and complete outer proving wall time, separately from compilation;
- final independent verification latency, with retrace/setup boundaries stated;
- peak RSS, not only row-storage estimates;
- outer proof **and public device/input/ELF/setup** transport sizes;
- safe schoolbook versus certified digit-plane decision, including actual lane count;
- small-count and meaningful multi-proof workloads, with admitted heterogeneous cases.

Use source-supported Fibonacci and muldiv controls first. They prove independent executions, not continued segments. Do not choose only a terminal microbenchmark or a favorable large batch and call it end-to-end parity. Repeated measurements must distinguish noise from a regression; no permitted regression tolerance is silently added to the user's parity requirement.

The immutable wrapper's documented command family is `cargo run --locked --release -p recursion --features akita,field-inline,ntt-inline -- generate ...`, then `trace --embed ...`, followed by `--features outer-prover,ntt-inline -- outer --elf <frozen ELF> --input <frozen input> ...` **without `--preflight`**. Its replay path includes retrace/setup and is not bare verifier latency. Guest builds require the matching Jolt/ZeroOS toolchain and Rust 1.95. No matching runtime fixture or exercised build is established on this restored host.

**Stop:** do not cut over or commit an accepted implementation if the soundness/scope gates fail or paired end-to-end performance fails the user's requirement. An unmeasured cost model is not evidence that parity is impossible. Failure of one construction is not a general impossibility claim about Akita accumulation.

## 16. Verification obligations before any cutover

The implementation must exercise real producer/guest/outer-verifier paths and keep meaningful adversarial regression coverage for:

- a bad A163 obligation with retained checks valid;
- invalid A164/A165, source norm, challenge, setup-offloading or original EOF despite a zero final public residual;
- two opposite invalid same-lane occurrence residuals; shared weights must not allow deterministic cancellation;
- an invalid predecessor and valid/invalid fresh input across both channels;
- matrix/profile/cache substitution, same seed with changed row stride, unsupported dimensions and malformed state;
- full-field positive/negative endpoints, terminal-bounded inputs, and exact digit-plane/schoolbook agreement;
- frame-field changes, altered occurrence order/count, zero-residue sampler mapping and partially queried candidate frames;
- zero source count, missing/extra records, nonzero padding, truncated framing, wrong output/panic/program/preprocessing and nonempty deferred ledger;
- application statement/link failures. Continued-segment checks require the new relation, not mock endpoint fields.

Tests must establish consumer-visible behavior or plausible soundness failures. A forwarding/mock echo, source-text assertion, or compile-only scaffold is not evidence.

## 17. Evidence, review, and current admission status

### Evidence exercised

- Relevant indexed paper sections were retrieved through the live qmd MCP, including Akita terminal/extraction, Nova relaxed folding, Lova RoK/norm, LatticeFold/LF+, Cyclo, Neo/SuperNeo and SALSAA.
- Lova's primary local PDF was used to check the parsed definition/footnote text. Parsed footnote 5 must not be read as an exponent on its knowledge error.
- Primary online Akita metadata, the published nonhomomorphic accumulation paper, and WARP metadata/abstract were read. Local published BCMS/BCLMS/Halo chapters were consulted; extended eprints are not claimed fully read here.
- Immutable source/ref/manifest/guest/terminal evidence was inspected. No source branch change, build, test, benchmark, proof generation, implementation commit, or push occurred.
- Ephemeral exact arithmetic computed the sampler maximum atom; exhaustive F7 scalar two-channel bad-tuple maximum was one accepting pair out of 36; same-lane opposite residuals cancel for every shared weight; independent occurrence/channel weights remove that deterministic failure.
- Exact boundary calculations checked 17-plane centered recomposition, the failure of 16 planes on the positive fp128 endpoint, the carry/nonlinearity example, the row-stride padding example, and actual 180/194-bit CRT products against the 255-bit minimum full-RHS bound. These calculations validate examples and bounds, not cryptographic security or runtime parity.
- An ephemeral Blake2b512 computation checked exact full-message hashing against verifier-owned incremental-prefix cloning for 42 combinations of frame lengths and uint64 block indices, including compression-block boundaries, a 1-MiB frame, and the maximum uint64 index. This checks the proposed hash-computation identity, not ideal-oracle security or Jolt execution.

The qmd index reports 13 documents, a vector index present, and 13 documents needing embedding. Lexical and vector retrieval worked. No embedding was regenerated and no claim is made that restored embedding metadata is fully healthy. Project `skills/README.md` is absent on this restored tree; the available lattice/proof skill was read with its hash-specific scope restricted.

### Admission checklist

| Gate | Current state |
|---|---|
| Public arithmetic relation, update, sampler and elementary proofs | Independently reviewed; corrected full-frame oracle keys close the prequery finding |
| Exact typed verifier factorization across all selected source paths | Source seam identified; implementation equivalence not exercised |
| Secure original source/outer/lifetime profile and complete error budget | Required; not certified for the new deployment |
| Native/private bounded bridge and uniform adaptive extractor | Open |
| Continued-segment frontend/RAM relation | Not supplied by inspected delivery |
| Authenticated complete receipt and concrete-hash composition premises | Explicit requirements; not newly verified |
| Safe unrestricted decision implementation | Mathematical/reference route identified; no implementation run |
| Paired end-to-end performance parity | Unmeasured |
| Independent design review | Mathematical and source/scope reviews completed; reported findings resolved; implementation gate remains closed |

See the [independent review and correction record](AKITA_ACCUMULATION_PROTOCOL_REVIEW.md). Review acceptance concerns the specified public protocol under its hypotheses; it does not override the closed deployment gate or authorize a narrower execution target.

No stale implementation skill is claimed: no storage, wire, replication or build implementation was changed. This document does not certify production readiness or satisfy the selected native segment-instance target.

## References and read scope

- **Akita:** [indexed local paper](SOURCE_PROVENANCE.md#akita); [official ePrint 2026/1983](https://eprint.iacr.org/2026/1983). Principal sections: 4–6, 8.2, 9.3, 10.2, 12.2; exact local extraction/terminal passages were retrieved again for this design. The online abstract is not substituted for the local theorem text.
- **Nova:** [indexed paper](SOURCE_PROVENANCE.md#nova), §§4.1–4.2, Construction 1 and Theorem 3.
- **Lova:** [indexed paper](SOURCE_PROVENANCE.md#lova); [primary PDF](SOURCE_PROVENANCE.md#lova), §§1.2.4–1.2.5, 2.3, 3.
- **LatticeFold:** [indexed paper](SOURCE_PROVENANCE.md#latticefold), §§1, 3.
- **LatticeFold+:** [indexed paper](SOURCE_PROVENANCE.md#latticefoldplus), §§1, 4.4, 5.2; relation-transformation/closure passages also documented in the prior obligation notes.
- **Neo:** [indexed paper](SOURCE_PROVENANCE.md#neo), §1.1. **Neo/SuperNeo:** [indexed paper](SOURCE_PROVENANCE.md#superneo), §1.2 and prior extraction/embedding notes.
- **Cyclo:** [indexed paper](SOURCE_PROVENANCE.md#cyclo), §1.1 and the bounded-growth discussion.
- **SALSAA:** [indexed paper](SOURCE_PROVENANCE.md#salsaa), §§5.2–5.3.
- **BCMS20:** Bünz, Chiesa, Mishra, Spooner, [Proof-Carrying Data from Accumulation Schemes, published chapter](https://link.springer.com/content/pdf/10.1007/978-3-030-64378-2_1.pdf), §§2.1–2.4; [local copy](SOURCE_PROVENANCE.md#supplemental-primary-sources).
- **BCLMS21:** Bünz, Chiesa, Lin, Mishra, Spooner, [Proof-Carrying Data without Succinct Arguments, published chapter](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_24.pdf), §§2.1–2.3; [local copy](SOURCE_PROVENANCE.md#supplemental-primary-sources).
- **Halo Infinite:** Boneh, Drake, Fisch, Gabizon, [published chapter](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_23.pdf), §§3–5; [local copy](SOURCE_PROVENANCE.md#supplemental-primary-sources).
- **Accumulation Without Homomorphism:** Bünz, Mishra, Nguyen, Wang, [ITCS 2025 primary paper](https://drops.dagstuhl.de/storage/00lipics/lipics-vol325-itcs2025/LIPIcs.ITCS.2025.23/LIPIcs.ITCS.2025.23.pdf), §§1–2; [publication record](https://doi.org/10.4230/LIPIcs.ITCS.2025.23).
- **WARP:** Bünz, Chiesa, Fenzi, Wang, [Linear-Time Accumulation Schemes, ePrint 2025/753](https://eprint.iacr.org/2025/753), abstract only in this pass. No WARP theorem is used in the construction.
- **Pinned Jolt lattice wrapper:** [guest](https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/examples/recursion/guest/src/lib.rs), [outer runner](https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/examples/recursion/src/outer.rs), [delivery contract](https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/specs/fast-recursion-public-delivery.md).
- **Pinned Akita terminal:** [terminal_direct.rs](https://github.com/markosg04/akita/blob/d0ee56a0b12db66b4c46a87c527169737694c4c7/crates/akita-verifier/src/protocol/core/terminal_direct.rs), [full verifier completion](https://github.com/markosg04/akita/blob/d0ee56a0b12db66b4c46a87c527169737694c4c7/crates/akita-verifier/src/protocol/core/verify.rs).
- **Jolt:** [indexed paper](SOURCE_PROVENANCE.md#jolt), §§2.4/3.2 and Definition 3.1; relevant memory/state passages retrieved through qmd for this design, not substituted for the fitting branch's actual ISA/frontend.
- **Proving CPU Executions in Small Space:** [indexed paper](SOURCE_PROVENANCE.md#cpuexecinsmallspaces), §§1.2–1.3; relevant streaming, memory-checking and PCS-dependent tradeoffs retrieved through qmd. No Akita accumulation theorem is attributed to this paper.
