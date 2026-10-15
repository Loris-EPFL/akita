# Akita Query-Relation Accumulation: a source-preserving protocol

## Status

**Candidate 1: finite-horizon polynomial query accumulation, one strong query reduction, one source-view-capable Akita batch decider.** No complete per-segment Akita PCS proof is an input. No compressed Akita commitments are added or multiplied. Original witnesses, source profiles and A/B/F paths remain unchanged.

This document specifies algorithms, relations, extraction order, implementation contracts and admission conditions. Two independent reviews found the explicitly conditional design and actual source boundary correct; the [lighter extraction derivation](AKITA_RELATION_ACCUMULATION_LIGHT_ORDINARY_EXTRACTION.md) supplies a terminal-backward proof without the former nonlinear helper trees. The candidate is **not concretely security-admitted**. Executed implementation evidence and remaining security limits are recorded in §12. Soundness remains conditional on the stated grid/decoder, frontend, indexed raw-sampler, setup and finite-resource certificates; ordinary PCS correctness alone does not establish them.

This is growing-state accumulation of **deferred opening relations**. It is not bounded-witness Nova IVC, general CCS folding, or aggregation of finished Akita proofs. The verifier retains and checks the segment frontend/history evidence. A final receipt can be linear in the number of segments. No Groth16/curve wrapper or arbitrary 4 MiB ceiling is part of this protocol.

The selected continued frontend rejects ZK mode and exposes its declared segment boundaries. This experimental protocol makes **no zero-knowledge or hidden-history claim**. Retaining the baseline public algebraic relation does not establish private execution; a later privacy change would require its own construction and review.

**Proof-target decision:** the user selected preservation of the base proof relation, including original raw-source predicates, rather than a stronger RV64/SDK byte/tape/halt theorem. The [continued frontend certificate](AKITA_RELATION_ACCUMULATION_CONTINUED_FRONTEND_CERTIFICATE.md) proves the selected algebraic relation and records the stronger semantic gaps. The decoded-only ordinary interface cannot preserve the original RAM-plane predicate; the [independently reviewed original-plane amendment](AKITA_RELATION_ACCUMULATION_ORIGINAL_PLANE_DESIGN.md) therefore adds an explicit source view and final signed-plane reduction. These source-view and predicate obligations are not discharged by ordinary decoded-only PCS correctness.

**Candidate accounting:** one protocol candidate has been selected. The maximum is ten complete candidates; a necessary stronger assumption, fundamental obstruction or unestablished soundness triggers escalation. Corrections to this candidate's source/interface contracts are not counted as completed alternative protocols.

## 1. Source references and security model

Primary local sources were read through qmd MCP, with coverage and extraction limits recorded in:

- [Akita obligations](AKITA_RELATION_ACCUMULATION_AKITA_OBLIGATIONS.md): all indexed Akita text, appendices A–I.
- [Range and folding](AKITA_RELATION_ACCUMULATION_RANGE_FOLDING.md): LatticeFold, LatticeFold+, Cyclo.
- [Tensor and sumcheck](AKITA_RELATION_ACCUMULATION_TENSOR_SUMCHECK.md): Neo, SuperNeo, SALSAA.
- [Generic architecture](AKITA_RELATION_ACCUMULATION_GENERIC_ARCHITECTURE.md): Nova, HyperNova, NeutronNova, and primary accumulation/Rust references.
- [Lova obligations](AKITA_RELATION_ACCUMULATION_LOVA_OBLIGATIONS.md): complete indexed Lova text and both protocol figures recovered from PDF.
- [Actual source-first boundary](AKITA_RELATION_ACCUMULATION_SOURCE_FIRST_BOUNDARY.md), [source certificates](AKITA_RELATION_ACCUMULATION_SOURCE_CERTIFICATES.md) and [runtime admission](AKITA_RELATION_ACCUMULATION_SIS_RUNTIME_ADMISSION.md): independent code review, selected response diameters, shell arities, terminal exceptions and parameter-table limitations.
- [Lighter ordinary extraction](AKITA_RELATION_ACCUMULATION_LIGHT_ORDINARY_EXTRACTION.md) and [actual-law sampler addendum](AKITA_RELATION_ACCUMULATION_LIGHT_ORDINARY_SAMPLER_ADDENDUM.md): independently derived classical-ROM source-probe theorem, full cost recurrence and explicit remaining concrete gates.
- [Exact grid/local certificate](AKITA_RELATION_ACCUMULATION_GRID_LOCAL_CERTIFICATE.md): actual source decoder, zero-point slice, helper DAG and conservative finite degree ledger; no claim that real padding is forced zero.
- [Indexed sampler findings](AKITA_RELATION_ACCUMULATION_INDEXED_RAW_SAMPLER.md), [direct transcript design](AKITA_RELATION_ACCUMULATION_INDEXED_TRANSCRIPT_DESIGN.md) and [independent grammar review](AKITA_RELATION_ACCUMULATION_INDEXED_TRANSCRIPT_REVIEW.md): required seed/chained-interface cutover, not an ideal-law assertion about existing v7 bytes.
- [Joint setup certificate](AKITA_RELATION_ACCUMULATION_JOINT_SETUP_CERTIFICATE.md): original overlapping-prefix matrix embedding, canonical setup references and qualitative PPT/runtime-priced Module-SIS theorem under the stated setup idealization.
- [Original-plane decider review](AKITA_RELATION_ACCUMULATION_ORIGINAL_PLANE_DECIDER_REVIEW.md) and [global original-range review](AKITA_RELATION_ACCUMULATION_GLOBAL_ORIGINAL_RANGE_REVIEW.md): independent raw-view fork proof, inverse-free compact digit factors, signed-range DAG, causal leaf ownership and separately priced cuts.
- [Frontend compiler review](AKITA_RELATION_ACCUMULATION_FRONTEND_HASH_COMPILER_REVIEW.md): existing digest aliases do not certify the active H1610 continued owner; select the reviewed direct-indexed frontend replacement, not an unproved chained-compiler fallback.

Akita citations below refer to the **current indexed local version**, `lattice-papers/akita.md`, `#bfa1a6`, not an authenticated original-author theorem by default. Official identity/revision metadata: [Akita ePrint 2026/1983](https://eprint.iacr.org/2026/1983). The official full PDF could not be retrieved for body-level comparison: the mounted reader returned only its abstract and direct HTTP retrieval returned 403. Agreement between local PDF and Markdown does not independently authenticate the expanded local theorems. Historical qmd search snippets contained diff markers absent from present get/file bytes; this is a retrieval/index discrepancy, **not confirmed source corruption**.

Model: transparent lattice setup as specified by the selected Akita profile; its declared family of Module-SIS assumptions at the actual collision radii/dimensions; programmable **classical indexed ideal ROM** with the exact conditional full-answer law. No concrete chained-H1610 knowledge axiom, QROM theorem, random-oracle-free recursion theorem, DLOG, pairing or new homomorphic commitment is assumed. A concrete transcript implementation must be separately shown to implement this interface.

The indexed base setup contract explicitly models expansion as a random oracle and separates $\mathcal H_{\rm setup}$ from $\mathcal H_{\rm FS}$ [qmd Akita lines 1761–1767]. Select that declared setup model, with the independent joint-embedding proof above. Exact canonical setup rejection has no coefficient bias in this model. Setup-oracle computation is charged in reduction time even when it is not an FS-forking query. This does not authenticate the local expanded paper against the inaccessible official body or give an unconditional concrete-SHAKE theorem.

A hash may instantiate Fiat–Shamir challenge generation, as in base Akita. It is not a hash/Merkle PCS, and does not replace the lattice commitment. Lattice hardness does not remove the need to bind a noninteractive transcript.

## 2. The original source relation

A public source reference is the complete pair

$$C=(\mathsf{shape},\mathsf{payload}),$$

not just payload bytes. Shape fixes the ordered source polynomials, natural/logical grid, coordinate order, packing, decoder, allowed source constraints, source and opening bases, A/B dimensions, B slice count, compression policy and public source view. Evaluation points and values are separate instances.

Let $s_C$ be an admissible algebraic decommitment to $C$ under the **original** source relation, and let

$$f_{C,c}=\operatorname{Dec}_{C,c}(s_C)$$

be its explicitly registered source-view polynomial component. For Main this remains each declared logical column; for original RAM the preservation amendment selects the raw 21-variable original-plane polynomial with six structural absent digit slots, while keeping the original 16-variable commitment profile unchanged. Existing decoded RAM claims lift through the exact public digit-recomposition adjoint. This notation does not imply $s_C$ is a canonical short digit vector: weak extraction certifies bounded products first; the final source-range reduction separately proves the original signed-plane predicate. Source comparison concerns the A/B/F projection, not D/H or points.

The one final PCS must certify all terminal component evaluations under the exact declared source relation. No component, compression path, used-slot predicate or frontend domain obligation may be silently removed. An original point claim cannot be replaced by an arbitrary query table without the reduction in §6.

## 3. Deferred application relation and opening obligations

The segment frontend runs through its existing algebraic protocols, stopping **before** the expensive PCS proof. Its deterministic verifier accepts or rejects a particular transcript; acceptance with true opening leaves does **not** deterministically imply the segment relation. The frontend uses randomized algebraic checks.

For each fixed admissible original source assignment $S$, each prechallenge context and each causal prover strategy, the selected frontend must prove a statement of the form

$$
\Pr\!\left[
 \mathsf{FrontendAccept}_j\ \wedge\
 \bigwedge_{i\in I_j}\mathsf{OpenRel}_i(S)\ \wedge\
 \neg\mathcal R_{\rm segment}(\mathsf{statement}_j,w_j(S))
\right]\le\epsilon_{\rm FE,j}.
$$

The probability is over the specified frontend challenges. The witness reconstruction $w_j(S)$, auxiliary witness coverage, challenge distribution and error bound must be derived from the actual frontend. This fixed-source statement does not by itself handle sources extracted after adaptive challenges; §8 requires a separate causal source-coalescing composition proof.

A deterministic implication is reserved for directly checked constraints or a complete accepting frontend extraction tree with its required interpolation structure. It is false for one accepted transcript: a sumcheck for the false sum $\sum_{x\in\{0,1\}}(1-x)=0$ can send $g(X)=0$ and obtain a genuine terminal opening when the challenge is $1$. Its nonzero soundness error must be charged.

This obligation includes all Main, RAM, range/Boolean, decoder, lookup and used-slot checks required by the selected Jolt frontend; an emission trait does not establish it.

### Selected frontend compiler

Use the [reviewed direct-indexed frontend mode](AKITA_RELATION_ACCUMULATION_FRONTEND_HASH_COMPILER_REVIEW.md), `jolt/continued-k/indexed-h256/mod128/v1`. Keep the existing algebraic stage protocols, original source commitments, polynomial proof codecs and all selected Main/source/boundary checks. Replace the frontend transcript owner and installed event/request schema. The current compact continued caller uses H1610/two-attempt sampling; the legacy Blake2b alias is a different path. Neither is silently declared to implement this new mode.

Each request freezes the canonical header/event prefix $P$; component$j$ uses an independent literal primitive key $\mathsf{Domain}\Vert\mathsf{Frame}(\mathsf{Hdr})\Vert P\Vert\mathsf{REQUEST}\Vert\mathsf{Frame}(R)\Vert\mathsf{COORD}\Vert\mathsf{LE64}(j)$. Decode the first16 bytes of its32-byte ideal answer modulo$q_K$. Count every component as an oracle entry, then append the full canonical request and **whole decoded result** before returning. No seed expansion, changing length before cached$P$, logging-only site or digest-of-prefix replacement is allowed. Finite event/vector/byte limits and errors are checked and sticky.

This retains the exactly reviewed maximum atom$\mu_{\rm FE}=2/2^{128}$ and exact full-answer inverse law; there is no uniformization or rejection-failure term for this frontend modulo decoder. The continued fixed-source degree certificate therefore retains its actual-law bounds. Native/query scalar families remain the separate canonical v8 sampler. Bind literal inherited frontend evidence and the checked complete-run/source/view context into finalization; resetting under a generic label or passing only a fresh digest does not do so.

Initially select the direct118-Main-plus-two-RAM leaf roster, without the optional118-to56 normalization. This removes a protocol boundary rather than dropping its input obligations: all118 original Main scalar assertions still enter the accumulator. Optional advice/field/committed-program modes need their own reviewed complete roster and cannot inherit this selected profile by assertion.

Each unresolved **scalar** obligation is represented as

$$
 v_i=b_i+\sum_{(C,c)}\sum_{x\in\{0,1\}^{\mu_{C,c}}}
       \phi_{i,C,c}(x) f_{C,c}(x).\tag{1}
$$

Here $b_i$ is a public affine offset, and $\phi$ is the exact public decoder adjoint/linear query. For an ordinary MLE opening, $\phi(x)=\operatorname{eq}(u_i,x)$, with

$$\operatorname{eq}(u,x)=\prod_k\big((1-u_k)(1-x_k)+u_kx_k\big).$$

Equation (1) may involve several original source components. The field is a selected evaluation field $E$ over the original source field $K$; scalars/decoders are lifted through the fixed $K$ basis. This is linear transport, not a multiplicative field-to-cyclotomic-ring embedding. Nonlinear source checks must first be reduced by their actual frontend protocol; they cannot be inserted in (1) as if linear.

**Index every independent scalar obligation globally.** If a segment produces 100 unresolved claims, it contributes 100 indices, unless a separately proved batching step has already reduced them to one. Reusing one coefficient $Z^j$ for several independent claims only certifies their sum and permits deterministic cancellation inside the segment.

## 4. Accumulator instance, witness and history

Let $m$ be the number of appended scalar obligations, distinct from segment count $T$. The public accumulator is

$$
 A_m=(\mathsf{context},\mathsf{history}_m,\mathsf{roster}_m,
       \{Q_{C,c,m}(Z,X)\},Y_m(Z),m),
$$

where

$$
Q_{C,c,m}(Z,X)=\sum_{i=0}^{m-1}Z^i\,\phi_{i,C,c}(X),\qquad
Y_m(Z)=\sum_{i=0}^{m-1}Z^i(v_i-b_i).\tag{2}
$$

$Z$ is a **formal bookkeeping variable**, not an extension-field generator. An implementation may retain canonical query descriptors/coefficients instead of dense coefficient tables. The verifier derives (2) from those descriptors; it never trusts a prover-supplied accumulator polynomial.

The accumulator witness comprises the original retained source witnesses/handles and frontend witnesses needed for the final prover. No source witness is folded or made short by multiplication with an evaluation-field challenge. Prover memory may grow with the retained sources; this cost must be measured, not hidden behind a constant public scalar.

The context binds protocol/version, program, workload, original setup IDs, source shapes/constraints, basis, ordered source roster, horizon, all frontend plans, final ordinary batch selection and budgets. History binds initial/final VM and RAM states, public I/O, active lengths, exact segment count and order. Every transition must check VM continuity and the **same original outgoing/incoming RAM source/view**. Payload equality alone is not private-handle provenance.

These are distinct from proof/transcript history. All frontend evidence necessary for public replay stays in the receipt. No prior verifier work disappears merely because its transcript is hashed. [Nova/HyperNova architecture and BCMS backward implication: companion generic note §§2–4.]

### Accumulated relation and adversarial old state

Define the strong deferred-opening relation separately from probabilistic application validity:

$$
\mathcal R_{\rm acc}(A_m;S):=
 \mathsf{AdmissibleSources}_{\mathsf{context}}(S)\ \wedge\
 \mathsf{CanonicalDerivation}(\mathsf{history}_m,A_m)\ \wedge\
 \bigwedge_{i=0}^{m-1}\mathsf{OpenRel}_i(S).
$$

Source admissibility and its certificate bounds use the same frozen horizon/setup policy at every prefix, not a smaller per-prefix decider policy. The canonical derivation includes the source roster, immutable source/view identities and the unique scalar-index assignment. It must not permit an append to replace an old source or reinterpret its shape.

For an append derived by (3), restriction of a new witness to the old roster gives

$$
\mathcal R_{\rm acc}(A_{m+k};S')
\ \Longrightarrow\
\mathcal R_{\rm acc}(A_m;S'|_{\rm old})
\ \wedge\ \bigwedge_{i=m}^{m+k-1}\mathsf{OpenRel}_i(S').
$$

This implication does not assume the old accumulator valid. It follows because the old coefficient equations and source identities are retained unchanged. Application validity still requires the probabilistic frontend theorem and explicit transition checks; it is not included by asserting that a sealed transcript exists.

## 5. Algorithms before finalization

### Setup / index

1. Select source/evaluation fields, original source classes and a finite segment/claim horizon.
2. Derive all source views and exact query adjoints. Include affine offsets and virtual/used-slot semantics.
3. Plan the final source-view-capable decider for **all retained original endpoint occurrences**, with complete shared D/H width, every F/H layer, setup-prefix provenance and the full shrinking-suffix/terminal closure. Register raw-plane native weights explicitly; the old ordinary 16-point API is not that capability.
4. Price the new prefix's tree, statistical errors, source comparisons and ROM budget together with the original PCS. Bind the resulting policy before any finalization challenge.

Runtime selection cannot change source profiles or successor policy after observing challenges. The indexed ordinary theorem allows an ordered final batch, repeated commitments and distinct points, but does not validate an unpriced new catalog row. [Indexed Akita §§5.1, 9–10; Theorems 10.31, 10.33, 10.37.]

### Init

Create an empty query accumulator, $Q=0$, $Y=0$, $m=0$, with the specified application initial state. No unchecked “first segment” or arbitrary zero-source bypass is allowed.

### Append

For each segment, run and bind its real frontend prefix, original source commitments and transition statement. Replay/check all cheap constraints. For every unresolved scalar claim $i$:

$$
Q_{C,c}(Z,X)\leftarrow Q_{C,c}(Z,X)+Z^m\phi_{i,C,c}(X),\quad
Y(Z)\leftarrow Y(Z)+Z^m(v_i-b_i),\quad m\leftarrow m+1.\tag{3}
$$

Only the genuine adjoints for sources used by that claim are updated. The old accumulator is not presumed valid. This is a deterministic update of the deferred relation, not a proof that its new source obligations hold.

### Preserve original source predicates

After freezing the original source roster, run the [global original-plane range reduction](AKITA_RELATION_ACCUMULATION_ORIGINAL_PLANE_DESIGN.md). Its complete helper/linkage evidence and every original terminal raw leaf enter the deferred relation **before seal**. Main Boolean/support/PC checks remain in the real frontend. Weak-product certificates and honest constructors alone do not establish these predicates.

### Seal

Freeze and canonically bind every commitment, profile, decoder/view, scalar claim, frontend/range transcript, query-index assignment, field, schedule, count and horizon. Freeze the **schema/order** of final endpoints, not their future point/value payloads: those are derived after $a$ and bound before native decider challenges. No new source or scalar obligation may be appended after seal. Missing/duplicated/reordered obligations, changed counts and an unbound roster are rejection conditions.

## 6. One strong query reduction

### 6.1 Query-index evaluation

After seal, obtain a uniformly distributed $a\in E$ from the registered indexed ideal interface. Evaluate (2) at $Z=a$.

Define the formal residual

$$
R(Z)=\sum_{i=0}^{m-1}Z^i\left(
\sum_{C,c}\langle\phi_{i,C,c},f_{C,c}\rangle-(v_i-b_i)\right).\tag{4}
$$

If any scalar obligation is false for the common source explanations, $R$ is a nonzero polynomial of degree at most $m-1$. Thus, for fixed explanations,

$$\Pr[R(a)=0]\le(m-1)/|E|.\tag{5}$$

This is not an argument for sources chosen after $a$: extraction/source coalescence in §8 is mandatory. The bound must use $m$, not $T$.

### 6.2 Exact heterogeneous-domain transport

Take $n\ge\max\mu_{C,c}$ and fixed coordinate embeddings $J_{C,c}$. Define

$$
F^\uparrow_{C,c}(X)=f_{C,c}(X_{J_{C,c}}),\quad
Q^\uparrow_{C,c}(a,X)=Q_{C,c}(a,X_{J_{C,c}})
       \prod_{k\notin J_{C,c}}(1-X_k).\tag{6}
$$

This is a cylindrical source extension and a zero-supported **public query** extension. Summing over the extra Boolean coordinates preserves the original sum exactly. It does not assert that the original physical padded source is zero outside its natural grid.

The $f_{C,c}$ in (6) must be the polynomial represented by the original ordinary PCS point interface. Current `OpeningClaims::committed_layout` requires exactly the profile's logical point dimension and all declared polynomial evaluations; ordinary packing appends zero coordinates on its physical axes. A random endpoint on extra physical axes cannot be passed unchanged as an ordinary point. If the frontend uses another natural/physical view, prove its exact restriction/decoder bridge and all used-slot obligations first.

### 6.3 Weighted source sumcheck

Prove one degree-two-per-variable sumcheck for

$$
Y(a)=\sum_{X\in\{0,1\}^n}\sum_{C,c}
       Q^\uparrow_{C,c}(a,X)F^\uparrow_{C,c}(X).\tag{7}
$$

Both factors are public-query or source multilinear extensions; their product has individual degree at most two. At the common terminal point $z\in E^n$, provide every original-source component value

$$p_{C,c}=f_{C,c}(z_{J_{C,c}}).$$

The verifier independently evaluates every public query at $z$ and checks

$$h_n=\sum_{C,c}Q^\uparrow_{C,c}(a,z)p_{C,c}.\tag{8}$$

Public query evaluation must use the exact table/adjoint, not replace a weighted sum of equality polynomials by one equality polynomial. For example, $\operatorname{eq}((0,0),X)+\operatorname{eq}((1,1),X)$ has table $(1,0,0,1)$ with rank two, unlike a scalar multiple of a single rank-one equality table.

### 6.4 One source-view-capable Akita decider

Construct the full original endpoint batch, supplying every Main component and every registered original-plane RAM endpoint. Invoke **one** Akita batch root with the explicitly proved public digit-weight view, and independently verify its complete shrinking suffix and actual terminal. The [native opening equations and cancellation order](AKITA_RELATION_ACCUMULATION_ORIGINAL_PLANE_DESIGN.md) specify the implemented source-view generalization; the decoded-only ordinary API cannot replace it, and original source profiles must remain unchanged. This retains *internal PCS recursion*, not recursion over completed segment proofs.

Bind the complete augmented prefix and endpoint instances into the PCS session/descriptor using canonical length-delimited context. A reset which loses the prefix or accepts endpoints under another context invalidates the composition. A root-only or reduction-only receipt is never acceptance.

## 7. Public receipt and verifier

The receipt contains: context/budget reference; ordered segment statements and source references; all required frontend and original-source range evidence; canonical deferred-claim descriptors; seal binding; query-sumcheck messages and complete original endpoints; one complete source-view-capable Akita proof. It does not contain a PCS proof for each segment.

The verifier:

1. validates the approved context, dimensions, finite budgets and canonical framing;
2. replays each frontend and checks the selected base algebraic VM/RAM/I/O/order/count linkage, retaining Main source predicates;
3. checks the complete original-plane range reduction, derives every deferred scalar claim and accumulator update, and checks complete coverage;
4. reconstructs seal and challenges, runs the weighted sumcheck and independently evaluates its public queries;
5. builds the exact original endpoint batch and verifies the **complete** final Akita proof;
6. rejects trailing data, missing endpoints, premature terminal, context substitution, dimension/profile mismatch, and all statement/transition/terminal mutations.

Nothing is authenticated merely by success in steps 1–4. The externally supplied expected statement/program/count is mandatory; an archive cannot approve its own context. Zero segments require a separately specified no-transition statement, not an unchecked ordinary-proof bypass. The required benchmark cases begin at one.

## 8. Extraction and classical-ROM theorem obligation

### Interactive algebraic reduction

Given a complete accepting tree:

1. At **every** outer leaf, run the entire ordinary PCS extractor with its frozen batch/schedule and original field/basis.
2. Compare original source projections across outer $a,z$ forks. The indexed Corollary 10.17 gives common decoded source polynomials or an original-family bounded A/B/F collision, under its exact cross-profile radii. D/H and points may vary across outer forks. Within an inner shell-coordinate fork, the original extractor still requires its own common D/H data.
3. With those common source polynomials, reverse each degree-two sumcheck by interpolation, obtaining $R(a)=0$ at every query-index child.
4. Interpolate (4) at enough distinct $a$ values to obtain $R\equiv0$. Because **each independent scalar claim has its own coefficient**, all input obligations follow.
5. A complete frontend extraction tree, or a separately proved causal probabilistic frontend-composition theorem, together with the explicit transition checks supplies the ordered application relation. A single accepted frontend transcript with genuine opening leaves is insufficient.

No source or query is divided by a lattice-ring challenge in this prefix. No norm of an inverse is assumed. Query weights are not used to create a new committed source. The source-comparison collision bounds remain the original bounds, **if** all source classes and full path widths are unchanged across forks.

### Source-comparison certificate calculus

For a fixed original matrix view $U$, suppose two certified explanations have the same image, $Us=Us'$, and carry units $\alpha,\alpha'$ with

$$
\|\alpha\|_1\le\kappa,\quad \|\alpha'\|_1\le\kappa',
\qquad \|\alpha s\|_p\le\beta,\quad
\|\alpha's'\|_p\le\beta'.
$$

If $s\ne s'$, then $v=\alpha'\alpha(s-s')\ne0$, $Uv=0$, and

$$\|v\|_p\le\kappa'\beta+\kappa\beta'.$$

This uses unit differences and convolution bounds, not short inverses. Unfold an equal compressed payload backward through its **certified** F preimages and gadget maps. A differing certified preimage gives the corresponding F collision; an equal preimage fixes the preceding image. Continue through the original B slices, then A. A difference in decoded sources implies a difference at some original source layer. Directly alphabet-bounded preimages use $\alpha=1$ in this calculation. Matrix views, certified domains and per-layer radii must match the actual base extractor; this calculus is not a substitute for proving it outputs those certificates. D/H equality is not required for this source-only comparison.

A conservative complete-tree shape for the **new query layer and final PCS only**, excluding frontend extraction, is

$$B_{\rm prefix+PCS}\le(m+1)\,3^n\,B_{\rm PCS}.\tag{9}$$

The query-index interpolation needs $m$ children for degree $m-1$; $m+1$ is a safe overbound. Since $3^n=(2^n)^{\log_2 3}$, this is polynomial in explicit domain size and claim count, not exponential in the segment history. It can still be large concretely and must be priced. Appending $T$ complete native roots sequentially would be a different, potentially exponential tree and is not this protocol.

### Selected lighter source-probe analysis

Equation (9) remains a conservative fallback, not the selected cost estimate. [The independently reviewed lighter derivation](AKITA_RELATION_ACCUMULATION_LIGHT_ORDINARY_EXTRACTION.md#11-full-ordinary-recurrence-and-polynomiality) starts at the actual direct terminal, obtains a successor $C_f$'s weak product certificate from its **next fold or terminal**, and compares that certificate across matched successful continuations before using the current parent's range. A fixed full decoded successor then permits first-divergence local soundness for the entire helper DAG. Retained flat shell-coordinate forks recover the incoming weak source; a separate incoming-source cut handles any nonidentity tensor or post-payload scalar batch.

For nonterminal $j$, with $b_j=1+\sum_g W_{j,g}$, $s_{\rm str,j}=1$ whole-strong-DAG probe and $s_{\rm pre,j}\in\{0,1\}$ incoming-prefix probe, the complete suffix work obeys

$$
C_j\le (1+s_{\rm pre,j})b_j(1+s_{\rm str,j})C_{j+1}+A_j.
$$

Here $A_j$ includes the correctly weighted cheap replays, all raw sampling, decoding, comparisons and exact checks. The leading coefficient is generally $4b_j$, or $2b_j$ when the incoming-prefix cut is provably unnecessary. The two source cuts concern different tuples; they cannot silently share one comparison. Apply this recurrence to **unconditioned original-table maps** with actual success bits, not a child error reused after forced ancestor matching. Optional setup coalescence includes its installed canonical reference and the actual full shared coefficient prefix; zero weights outside active support do not mean zero source entries.

The all-sealed-sources query cut and the earlier original-range cut are separately owned. With $Q$ the original FS-request budget, the [independent original-range review](AKITA_RELATION_ACCUMULATION_GLOBAL_ORIGINAL_RANGE_REVIEW.md) proves

$$
C_{\rm query}\le2C_{\rm PCS}+QT_{\rm query}+A_{\rm query},
$$
$$
C_{\rm range}\le2C_{\rm query}+QT_{\rm range}+A_{\rm range}
\le4C_{\rm PCS}+2QT_{\rm query}+QT_{\rm range}
 +2A_{\rm query}+A_{\rm range}.
$$

For independently uniform successful field requests, their combined original-event statistical ledger, before frontend/cap/cutoff/setup/MSIS terms, is

$$
\kappa_{\rm global}\le\kappa_{\rm PCS}
 +Q\epsilon_{\rm query}+Q\epsilon_{\rm range}
 +2\binom Q2/|K|.
$$

Write $n_{\rm range}=21+\lceil\log_2N_{\rm RAM}\rceil$. The reviewed normalized base32 product/quartic drivers plus cubic linkage give $\epsilon_{\rm range}\le(12n_{\rm range}+3)/|K|$; $(14n_{\rm range}+3)/|K|$ remains conservative. Its helper claims are source-determined through the DAG, not free original openings. The range cut compares weak A/B/F certificates **without assuming the range it is proving**. The second factor two and second old-pair term cannot be dropped without a jointly proved coupling.

These source cuts remove the former outer $m3^n$ factor; they do not remove the retained shell product or automatically admit a256-source schedule. Every actual byte, sampler, source scan and collision reconstruction remains charged.

The separately owned frontend source cuts add $Q\sum_h\epsilon_{\rm FE,h}+\binom Q2(2/2^{128})$ under the reviewed uniquely owned direct-indexed frontend grammar, not $H\kappa_{\rm PCS}$. Their expected work includes the actual matched complete deeper continuations and full literal keys/raw answers. A single cut per segment is permitted only when its complete needed source tuple is causally fixed there; later-bound groups require the stage/ownership ledger.

The actual canonical $E=K$ decoder returns the complete successor source $S$. The current strong table is its declared restriction $W(x)=S(x,0,\ldots,0)$ when the next opening cube is wider. Real cells after the live semantic extent remain part of $W$; they can be nonzero in-range values with zero relation weights. The [grid/local certificate](AKITA_RELATION_ACCUMULATION_GRID_LOCAL_CERTIFICATE.md) gives an explicit one-cell $-1$ counterexample to an all-padding-zero claim and proves the correct native-row/helper/squared-norm lemma. Do not crop the extracted source or claim the current strong check scans a wider successor cube.

### Frontend composition is a separate obligation

Seal binds frontend evidence but does not prove its constraints. Use the selected fixed-source degree/reconstruction certificate, together with the separately owned frontend source probes in the [direct-indexed compiler proof](AKITA_RELATION_ACCUMULATION_FRONTEND_HASH_COMPILER_REVIEW.md#7-independent-proper-descendantsource-probe-theorem-for-the-replacement). Each probe retains the marked original output and compares only original sources fixed at its causal root; later commitments and D/H may differ. The complete deeper continuation now includes the earlier range cut and later query/native extraction. Do not reuse the obsolete query-only tree work formula as the amended full protocol.

The published weighted sampling games of [Fenzi–Moghaddas–Nguyen, §8](https://infoscience.epfl.ch/server/api/core/bitstreams/9abd4c0d-3e31-4228-b600-28f51a7a511f/content) independently anchor the abstract marked-original construction; the [anchoring addendum](AKITA_RELATION_ACCUMULATION_FRONTEND_COMPOSITION_PUBLISHED_ANCHOR.md) records its derivation. Historical marked-frontend-tree and$m3^n$ formulas are comparative notes, **not the selected AQRA theorem or runtime budget**. The reviewed source-probe route uses its own actual-law, unique-owner, renewal-work, old-query and strict-cutoff terms.

### Noninteractive transport: not automatic

The theorem must invoke the **same authenticated full conditional ideal-answer law** as the ordinary indexed Akita compiler, for one augmented, prefix-causal challenge grammar. The selected query sampler is uniform with documented finite rejection/failure treatment. Replaced strong/frontend source cuts may instead use their certified actual fresh field law: a degree-$d$ residual has error at most $d\max_c\Pr[c]$; independent coordinates contribute the corresponding sum. This avoids falsely replacing legacy modulo-$q$ bytes by a uniform sampler. **Retained shell forks still require their exact conditional full raw-answer law**, including rejected candidates and unused bits. Domain separation must preserve full answers and continuation, not just marginal entropy.

The indexed local Lemma 3.22 / Theorems 10.33 and 10.37 remain useful statement templates, not independently authenticated author results. The published-source marked sampler independently supplies the required original-event/compiler recursion at the abstract ideal-answer level. Merely multiplying black-box extractors, prepending bytes to an old transcript, citing interactive LatticeFold, or assuming concrete chained H1610 has this full-answer law is still invalid.

**Remaining admission gates:** instantiate the proved recursion for the executable grammar in §§5–6; certify the actual base extractor's source/path products; verify all source-family and full-runtime prices; establish each concrete frontend reconstruction/decoder contract; generate/check the actual final schedule and numerical policy. Until then, this is a specified candidate, not a deployed sound accumulator.

The [actual sampler review](AKITA_RELATION_ACCUMULATION_INDEXED_RAW_SAMPLER.md) closes the old interface-fit question negatively: the current seed expansion and default Blake2b continuation do not provide this law. The [reviewed direct finite grammar](AKITA_RELATION_ACCUMULATION_INDEXED_TRANSCRIPT_REVIEW.md) supplies the replacement construction, exact successful/failure inverse laws and conservative full-answer work accounting. The [joint setup proof](AKITA_RELATION_ACCUMULATION_JOINT_SETUP_CERTIFICATE.md) supplies the overlapping-view challenge embedding under an explicitly idealized setup namespace. None of these proofs equates bare uniform Module-SIS with hardness of an arbitrary concrete fixed-realized SHAKE matrix.

## 9. Fields, horizon, arity and resource cost

### Fields and concrete error budget

With fixed explanations, the new interactive algebraic error is at most

$$\epsilon_{\rm query}\le\frac{m-1+2n}{|E|}.$$

For the direct complete no-advice/no-field/full-public continued roster, $m=120H+N_{\rm RAM}$:118 Main scalar assertions and two decoded RAM assertions per segment, plus every original raw range endpoint. The independently certified Main normalization instead gives $m=58H+N_{\rm RAM}$ and adds its actual error/source-cut costs. Sharing a source does not merge independent pointed equations. Final decider components are $56H+N_{\rm RAM}$; root shell coordinates are $56H+16N_{\rm RAM}$. Query dimension is21 for this Main/RAM view family; the source-index axes belong to the range DAG, not the explicit-source query sum.

For the selected source-probe construction, before all implementation-specific numerical admission, the conditional finite bound is

$$
\Pr[\mathsf{Accept}\wedge\nexists w:\mathcal R_{\rm run}(\mathsf{statement},w)]
\le \kappa_{\rm PCS}
 +Q(\epsilon_{\rm query}+\epsilon_{\rm range}
       +\sum_h\epsilon_{\rm FE,h})
 +2\binom Q2/|K|+\binom Q2\,2/2^{128}
 +\Delta_{\rm sampling}+\Delta_{\rm setup}+W/L
 +\sum_{I\in\mathcal I}\operatorname{Adv}_{{\rm MSIS},I}(T_I(L),\eta_I).
$$

Here$W$ is the **whole unconditioned expected extraction work**, including every separately owned frontend/range/query/native cut; $L$ is its one strict work cutoff. Each$T_I(L)$ includes full-answer conditional sampling, setup simulation, replay, source comparison, decoding and collision reconstruction; $\eta_I$ is the actual mixed certified radius. The selected ideal setup certificate gives$\Delta_{\rm setup}=0$ for exact rejection. All selected finite field/shell/predicate exhaustion terms remain in$\Delta_{\rm sampling}$. The existing table gate is ADPS16 **modeled quantum cost at least128 bits**, not an extractor-time advantage certificate. No estimator API justifies substituting$T_I/2^{128}$.

**No extra tight end-to-end $2^{-128}$ requirement is imposed.** The approved objective uses base Akita/lattice assumptions and requires a sound, explicit budget; [the earlier field-policy decision](AKITA_INSTANCE_ACCUMULATION_FIELD_POLICY_COMPARISON.md#4-statistical-error-grinding-work-and-baseline-policy) also makes this distinction. Report the actual $\kappa(Q,T)$ and lattice runtime/radii. Do not label a weaker concrete bound “128-bit admitted,” or assume a negligible lattice term at an extraction runtime beyond the declared hardness model.

Initial arithmetic choice for reuse is **$E=K$**, subject to that complete policy. A 128-bit scalar field is not automatically a 128-bit end-to-end bound after degree, queries and history. A genuine supported extension or separately proved repetition is an alternative if the selected bound requires it; neither is intrinsically required by lattice accumulation. The formal $Z$ does not require an extension.

For the selected direct roster at$H=256$ with257 distinct RAM originals, $m=30{,}977$, $n_{\rm query}=21$ and$n_{\rm range}=30$. Thus$\epsilon_{\rm query}\le31{,}018/q_K$ and$\epsilon_{\rm range}\le363/q_K$. As an explicitly illustrative request budget$Q=2^{32}$, the query term is about$2^{-113.08}$ interactively and$2^{-81.08}$ after$Q$; the **new range/query/frontend old-pair terms together** are approximately$2^{-63}$, before native PCS, frontend fresh losses, finite caps, cutoff and runtime-priced lattice terms. This is not a numerical soundness admission.

The generic `Ext2<K>` with `TwoNr`, $U^2=2$, is a **ring, not a field** for A7F7: $q\equiv1\bmod8$ and $2$ is a square in $K$. The repository's explicit `jolt_field::Prime128OffsetA7F7Ext2` instead uses `Prime128OffsetA7F7Ext2Config`, $U^2=5$, and is a field; the current compact original-source owner reuses that existing type. A direct modular-arithmetic calculation confirmed $(2/q)=1$, $(5/q)=-1$. Its canonical coefficient pair is not the ordinary cyclotomic `FpExtEncoding` basis. Returning it through a coefficient shim would be an algebraic error. The final AQRA query/native owner uses $E=K$ without such a shim and projects the authenticated original source capabilities, not new commitments. A real [upstream275 ref](AKITA_RELATION_ACCUMULATION_UPSTREAM_275_REFS.md) provides an alternative *fresh* source-field setup with compatible extension arithmetic; it cannot migrate old A7F7 commitments/planes. [The Jolt ref audit](AKITA_RELATION_ACCUMULATION_JOLT_FIELD_REFS.md) records the same-field frontend limits.

The horizon is mathematical: $m<|E|$ and the concrete error budget must hold. A nonzero polynomial such as $Z^{|E|}-Z$ vanishes at every evaluation-field element; one fixed-field test cannot support unbounded degree. This is not a receipt byte ceiling.

### Arity

Initial setting: **one terminal high-arity reduction over the full sealed workload**, with actual segment counts 1, 2, 3 and 256; scalar fan-in is $m$, endpoint/source fan-in is the actual roster. The online phase grows query relations without per-update cryptographic folding. This avoids an unproved $T$-deep repeated-root theorem.

Arity 2/4/8 blocks or balanced trees are alternatives only if their intermediate commitments, queries, history and global extraction/norm budgets are separately proved and measured. No invented fourth application transition pads the three-segment case. A later optimization cannot silently change high-arity finalization into constant-state online IVC.

### Cost

Public state/receipt can grow $O(T)$ plus the actual deferred-claim metadata. Original witness retention can be $O(\sum_C L_C)$. Dense query tables need not be built online, but final public-query evaluation must not allocate avoidable full tables when the exact descriptors admit direct evaluation. The weighted sumcheck and one larger batch decider must be measured on real sources; no speedup follows from the proof outline.

Published Akita's constant-root cost examples assume additional polylogarithmic group/claim/slice regularity. They cannot establish that a growing 256-segment roster costs one original singleton root. [Indexed Theorem 9.3 and example 9.4.]

### Frozen-roster root selection

The [independent frozen-root contract](AKITA_RELATION_ACCUMULATION_FROZEN_ROOT_ROW_CONTRACT.md) closes source-profile root capacity for the four requested counts. Keep the last Main group designated final; retain unique original RAM groups followed by earlier Main groups in the native prefix, with a bound semantic-to-native permutation. Planning equivalence shares metadata, not commitments. Select base32/native128/single-chunk root with **response depth3 for both classes**: positive reach15855 covers every allowed Main fold ($56\cdot31=1736$) and RAM fold ($16\cdot31\cdot16=7936$). Its A collision radius4063108 fits each frozen capacity8126340. Do not infer a joint batch success guarantee from per-group tail success; the all-valid option avoids that missing argument.

For distinct RAM boundaries, the source-backed derived root geometry is:

| Segments$H$ | Original RAM groups | Final scalar components | Ring shell coordinates | Shared D columns |
|---:|---:|---:|---:|---:|
| 1 | 2 | 58 | 88 | 2288 |
| 2 | 3 | 115 | 160 | 4160 |
| 3 | 4 | 172 | 232 | 6032 |
| 256 | 257 | 14593 | 18448 | 479648 |

All these native128 D widths fit the existing rank-one capacity28621854. Every original F path remains separate; D/H is shared and complete. At256 the D setup footprint alone is61394944 field elements (982319104 bytes at16 bytes/element), not a singleton-root cost. These are derived root inequalities, **not generated or runtime-validated complete rows**. The deterministic planner must still calculate exact next-witness geometry, find/audit the full shrinking suffix and actual terminal, provision the joint setup envelope, bind view-aware keys and emit the full finite resource/error/comparison ledger. It must reject a failed gate rather than resize original profiles, paste a stale suffix or retry decoded semantics.

## 10. Existing-code reuse and no-reuse boundaries

| Existing mechanism | Use / limitation |
|---|---|
| Jolt frontend and original opening emission records | Reuse only after exact coverage and application/decoder contracts are verified. Remove the per-segment PCS call, not the frontend checks. |
| Original CPU source/private handles and source identity | Keep immutable witnesses and actual RAM aliases; do not reconstruct from commitment bytes or invent fresh equivalent sources. |
| Original grouped claims and native partials | Add the registered raw21 query view without changing source16 geometry; preserve all Main components, full Ehat/D/H and inverse-free raw Eq digit factorization in the compact verifier. |
| Akita ordinary `begin_batched_reduction` / `PendingNativeProver::finish` | Reuse the existing single root plus full suffix; do not treat paused states as complete proofs or insert fresh sources into an ordinary shrinking suffix. |
| Original sumcheck drivers | Reuse arithmetic/round infrastructure; implement the exact public-query adjoint and source factors in (7). A generic sumcheck runner is not its soundness theorem. |
| Original EOR / `FpExtEncoding` | Reuse if an already-supported evaluation extension is selected. Basis compatibility and full component widths remain required. |
| Native full-ring normalization prototype | Arithmetic/decoder patterns may help, but its H1610/E2 admission and repeated-root law do not transfer to this candidate. |
| External Nova/LatticeFold/Neo/SALSAA code | Reuse orchestration or vetted low-level patterns only. No Pedersen/Ajtai root substitution, incomplete decider, ignored verifier result, or repeated-entry random setup. |

## 11. Verification and measured control evidence

A new ordinary PCS-only control executable now exists at `akita/crates/akita-pcs/examples/accumulation_pcs_control.rs`, derived from the real `akita_e2e` Dense fp128 nv14 workload, fixed upstream seeds with deterministic per-segment source variation, selected catalog geometry and original prove/verify APIs. It processes proofs separately and **is neither accumulation nor recursive aggregation**.

Executed with offline locked release build, no default features, `transcript-blake2b`. Every proof in these actual controls verified:

| Segments | PCS prover ms | PCS verifier ms | Serialized PCS bytes | Peak RSS bytes |
|---:|---:|---:|---:|---:|
| 1 | 125.099539 | 2.726456 | 62,432 | 40,816,640 |
| 2 | 233.889802 | 5.430839 | 124,901 | 42,913,792 |
| 3 | 346.293291 | 8.144110 | 187,340 | 43,196,416 |
| 256 | 28,628.272160 | 698.134455 | 15,990,351 | 41,713,664 |

These times exclude source generation/import, opening evaluation and claim construction; setup is shared within a process. Actual selected inner ring dimension was **512**, not the benchmark call site's nominal const generic 256. The control was corrected to use the trusted selected catalog dimension rather than impose an unsupported fixed dimension. Peak memory is Linux process-lifetime VmHWM. These are one-run measurements under concurrent research activity, not stable speedup claims.

Affected example `cargo clippy --offline --locked --release -p akita-pcs --no-default-features --features transcript-blake2b --example accumulation_pcs_control -- -D warnings` passed. No final-accumulator run, tamper proof, recursive aggregator comparison or whole-Jolt check is claimed by these controls.

The final `bash autoresearch.sh` must exercise the admitted candidate and controls at all four counts, including false statement, reordered/broken transition, missing source endpoint and altered terminal rejection. Report actual arity, complete prover/verifier time, peak memory and receipt/proof bytes separately. No parity claim or optimization loop begins from compile-only evidence or the PCS-only controls.

## 12. Executed engine baseline and current integration gate

The deterministic `autoresearch.sh` now runs the **original Dense fp128 nv14 AQRA engine** at all four requested counts. Each run uses retained original commitments, seals the scalar ledger before the fresh query challenge, proves the degree-two query, and generates one complete native root, shrinking suffix and actual terminal. The independent public verifier accepted every complete receipt. There were zero complete per-segment PCS proofs.

| Original Dense sources | Accumulated prover ms | Query ms | Native prover ms | Independent verifier ms | Receipt bytes | Peak RSS bytes |
|---:|---:|---:|---:|---:|---:|---:|
| 1 | 139.523102 | 16.633250 | 122.889852 | 4.242287 | 69,408 | 63,184,896 |
| 2 | 203.671489 | 33.358528 | 170.312961 | 5.770518 | 73,151 | 88,858,624 |
| 3 | 253.900236 | 50.175468 | 203.724768 | 6.417560 | 74,928 | 105,316,352 |
| 256 | 20,988.694861 | 14,404.509532 | 6,584.185329 | 185.707216 | 465,756 | 2,164,068,352 |

These are one-run measurements, not a recursive-aggregation comparison. Prover time includes query construction/proving and native proving, but separately excludes source generation/commitment and setup. Peak RSS is whole-process Linux `VmHWM`. The Dense workload does not exercise the VM frontend, transitions or the raw-RAM range DAG. Section 11's older PCS controls used an earlier implementation snapshot and are historical controls, not a same-snapshot speedup denominator.

Every count independently rejected mutations of the scalar endpoint, ordinary claim, source order, source profile, consuming view, query round, completed literal history, full native root, actual native terminal, native truncation/trailing bytes and receipt truncation/trailing bytes. The256-source receipt has five genuine shrinking suffix folds; its315,160-byte schedule row and retained witnesses are not hidden in a constant-state claim.

Two independent code reviews found and closed these integration defects:

- Move the nonterminal native opening payload before the evaluation batch and atomic shell in the trusted grammar.
- Check the complete native context length and budgets before streaming its borrowed framed fields into SHAKE; do not first allocate a complete augmented-context copy.
- Validate the raw original RAM reader against the selected profile/view and original physical geometry, not the obsolete compact-E2 call-order map.
- Preserve the producer's Boolean whole-grid implication: an actual Main `OneHot` chunk of16 still satisfies the singleton Boolean admission contract. This does not change its descriptor, stride or charged norm bound.

The reviewers confirmed the exact fixes statically. The Dense end-to-end runs above subsequently exercised complete native proving and independent verification. The source-preserving Jolt receipt and real RV64 driver subsequently passed the one-, two- and three-cut runtimes described in §12.3. The genuine Jolt256 runtime remains an open acceptance gate. Static review is not a proof of the conditional-ROM theorem or a numerical security admission.

The two arithmetic optimizations have20 passing focused accumulation tests: sealed borrowed per-source recipe weights and bounded eight-bit range-prefix equality factors. Canonical run2 passed every complete proof and mutation check. At256 Dense sources, query time was4,638.261869ms and complete measured prover time11,846.916260ms,43.6% below the run1 baseline. Receipt/native-proof/source/schedule sizes were unchanged. Concurrent catalog generation and Jolt compilation increased several small-case/native timings; repeatability remains to be measured. This is an engine result, not a VM, range-speedup or recursive-parity result.

The independent Jolt prover review also found an omitted mandatory original-handle projection cost. The fix now reports wall time around the **complete** `prove_receipt` call, including projection, installation, roster/ledger/context work and all proof phases; it separately reports projection time. The reviewer confirmed this exact fix. Actual Jolt1 execution subsequently reached the genuine311-row trace but failed before proving because resource sizing attempted decoded EOR geometry on a raw view even though the selected native evaluation degree is one. That admission defect was fixed at the real EOR predicate, without deleting the raw-view guard or relabeling a source. Two real complete proof-stream tests (fp128 and F32/E4) passed after actual regeneration of all17 Akita catalog families.

Run3 retained the exact cylindrical source lift while enumerating only its supported Boolean suffixes and sharing bounded prefix factors. All four Dense proof/rejection workloads passed again. The256-source prover measured10,327.004324ms, with3,220.732073ms query work; receipt shape remained unchanged. These are still engine measurements under concurrent load.

The degree-one EOR admission defect is now fixed with the existing `requires_extension_opening_reduction` predicate. The mixed raw/decoded complete resource-and-registry fixture passes; the actual original-plane full native control also passes. Eleven retained-owner/protocol-soundness/complete-proof tests across three integration suites and strict Clippy on the affected Akita proof libraries passed. Two incidental registry-wiring tests were removed rather than repinned to a different unsupported fixture.

The next genuine Jolt execution identified a separate installation-header sizing defect: its externally compiled finite grammar has11,861 messages,311 draws and11,533 compounds, producing a21,914,447-byte literal context. The old16,777,216-byte per-message cap was incorrectly used for that complete installation header. The implementation now derives exact header and Direct120 allowances before execution, binds both lengths in the `finite-limits/v2` trailer and checks allocation limits before reserving. Full literal grammar/source metadata, normal16MiB payload/request caps and the512MiB complete-prefix cap remain. The exact seal length is header plus approved identities and original Main118/RAM2 payloads plus4028 framing bytes; the derived global seal allowance is620,760,973 bytes. Eight focused consumer-boundary tests passed and independent static review found no concrete defect. The subsequent one-cut runtime passed complete receipt verification. Older `finite-limits/v1` evidence must be regenerated; no compatibility shim is retained.

The genuine Jolt driver includes boundary-counter, initial/final register, adjacent architectural-transition and omitted query/native endpoint wire mutations. All eighteen applicable one-cut mutations and all nineteen two- and three-cut mutations were independently rejected, including the adjacent architectural-transition mutation. No recursive-aggregation parity or stronger RV64 theorem follows from these replays.

Independent static review confirmed canonical field mutations and exact enclosing-block length adjustments for the eighteen H1 checks and nineteen H≥2 checks. These replays have narrower meanings than a proof of resistance to regenerated adversarial proofs: boundary-final and endpoint changes can also change Fiat–Shamir history, the final Golomb-response byte can fail canonical decoding before terminal algebra, and EOF checks test exhaustion. The verifier still reconstructs every native endpoint binding and executes the complete terminal verifier; the mutation matrix alone does not prove a stronger RV64 theorem.

### 12.1 Same-snapshot ordinary PCS controls

The same working tree was also exercised through `accumulation_pcs_control`, with retained Dense nv14 sources and `RAYON_NUM_THREADS=1 AKITA_PARALLEL=0`. These are separate ordinary PCS proofs, **not recursive segment aggregation**. The1/2/3-source controls ran concurrently after Cargo lock serialization; the256-source control ran separately. Concurrent planner generation and other work make these observations non-isolated.

| Sources | Full PCS proofs | PCS prover ms | PCS verifier ms | Sum of PCS payload bytes | Process peak RSS bytes |
|---:|---:|---:|---:|---:|---:|
| 1 | 1 | 135.960786 | 4.516074 | 62,488 | 40,894,464 |
| 2 | 2 | 259.549353 | 8.786390 | 125,022 | 41,775,104 |
| 3 | 3 | 381.219163 | 12.963566 | 187,491 | 41,791,488 |
| 256 | 256 | 31,709.910965 | 1,103.125413 | 15,989,983 | 42,070,016 |

PCS payload totals are not the accumulation receipt's literal public-statement/evidence-inclusive transport size. The control's streamed source lifecycle is also not the256-retained-source engine's memory lifecycle. These observations therefore do not establish recursive-aggregation parity, equal receipt accounting, equal memory use or zkVM end-to-end speedup.

### 12.2 Exact arithmetic caching, not commitment folding

Run 4 passed all four Dense workloads, complete native verification, public replay and thirteen mutations per workload. The full accumulation test module passed 26 tests, and strict Clippy passed on the affected Akita proof libraries. The 256-source prover measured 9455.447236 ms: 1724.753621 ms query work and 7730.693615 ms native proving. The receipt remained 465756 bytes. Concurrent catalog generation and genuine Jolt executions were active; this remains an engine observation, not VM or recursive-aggregation parity.

For a sealed Eq recipe with anchor \(\zeta\), fixed scalar weight \(\omega\), bound prefix \(y_{<r}\), and Boolean suffix \(s\), the new round cache computes exactly

$$
B_r=\omega\prod_{j<r}\bigl((1-\zeta_j)(1-y_j)+\zeta_jy_j\bigr),
\qquad
S_r(s)=\prod_{j>r}(1-\zeta_j)^{1-s_j}\zeta_j^{s_j},
$$

$$
q_{0,s}=B_r(1-\zeta_r)S_r(s),
\qquad
q_{1,s}=B_r\zeta_rS_r(s).
$$

Eight-bit factor tables evaluate \(S_r\) with bounded metadata, without division or a full source table. For an added cylindrical axis \(r\ge\mu\), the exact pair is instead

$$
q_0=\omega\,\operatorname{eq}(\zeta,y_{<\mu})
\prod_{j=\mu}^{r-1}(1-y_j),
\qquad q_1=0.
$$

MaskedEq remains the MLE of the masked Boolean table. Sparse keeps the direct reference fallback; the subsequent exact MaskedEq and DecodedRam preparation is described in §12.4. The two source contractions share each prefix equality weight but still read and validate both original zero/one branches. Range helper pairs likewise compute \(U=G(G+1)\) and each child on Boolean source values **before** contraction. Padded zero cells keep their nonzero higher-child values. No source commitment arithmetic, recommitment, source replacement, endpoint omission, protocol degree or security assumption changes.

### 12.3 Executed genuine Jolt receipts

The full delivery harness completed genuine Jolt **1/2/3/256** legal-cut workloads. Each executed the embedded RV64 ELF with **311 total real trace rows**, no padded empty segments, one complete final native root/suffix/terminal proof and **zero per-segment full proofs**. The independent public verifier accepted every receipt and rejected all 18 mutations for one cut and all 19 for each larger workload, including the architectural-transition, original-source endpoint, terminal and EOF cases. The 256-cut experiment is not a 256-full-segment Reth workload.

| Executed run | Cuts | Source-component endpoints | Producer ms | Verifier ms | Receipt bytes | Process peak RSS bytes |
|---|---:|---:|---:|---:|---:|---:|
| Full harness | 1 | 58 | 177,815.876483 | 1,255.157633 | 95,549,336 | 12,659,769,344 |
| Full harness | 2 | 115 | 384,699.349741 | 2,418.058971 | 190,889,247 | 10,760,871,936 |
| Full harness | 3 | 172 | 523,993.777322 | 3,596.698452 | 286,178,494 | 12,002,816,000 |
| Full harness | 256 | 14,593 | 45,436,157.340374 | 561,446.948924 | 24,397,136,796 | 56,373,907,456 |
| Post-compatibility rebuild | 1 | 58 | 192,900.478441 | 1,690.494924 | 95,549,336 | 12,964,519,936 |

The 256-cut run retained all 256 original Main owners, 30,720 frontend scalars, 257 range scalars and 14,593 original-source component endpoints. Its measured phases were:

| Phase | Time ms |
|---|---:|
| Trusted preprocessing, setup and original RAM construction, excluded from producer metric | 6,808.138089 |
| Original frontends, sum of 256 measured cut phases | 39,654,519.497250 |
| Mandatory original-handle projection | 2,499.359913 |
| Global original-RAM range | 2,767,376.815635 |
| Degree-two query accumulation | 2,803,114.960712 |
| Complete final native root, suffix and terminal proof | 123,127.706517 |

Producer wall time includes unlisted context/preflight work; receipt encoding and independent verification are outside it. Process peak RSS includes frontend and mutation work. Driver `accumulation_arity` counts authenticated source-component endpoints, **not** full segment proofs or a fixed recursive-fold arity. The large public certificate and costly source frontends are real experimental costs, not a succinct-receipt or performance-parity result.

The full harness ran while later compatibility and optional-D1 checks proceeded. The separate post-compatibility one-cut run rebuilt the indexed driver after the shared ordinary-metadata, native setup-transport and caller migrations; it accepted the full proof and rejected all eighteen applicable mutations. The 256-cut result is the completed full-harness observation, **not** a claim that the entire 256-cut workload was repeated after every later compatibility edit. Equal receipt lengths do not establish historical byte-for-byte transcript equality. Concurrent compilation and workload conditions differed; these measurements establish neither isolated speedups, a recursive baseline, equal-workload parity, ZK, QROM security nor deployment admission.

### 12.4 Exact masked/decoded preparation and certificate residency

For a masked-equality recipe at round $r$, set $W=2^r$, $B_{b,s}=(2s+b)W$, $Z_j=(1-\zeta_j)(1-y_j)$ and $O_j=\zeta_jy_j$. With canonical disjoint support intervals $\mathcal I$, the prepared prefix sum is

$$
M_r(B)=\sum_{J\in\mathcal I}
\sum_{\substack{0\le x<W\\B+x\in J}}
\prod_{j<r} Z_j^{1-x_j}O_j^{x_j}.
$$

The exact branch value, before the sealed term weight, is

$$
\phi_r(b,s)=M_r(B_{b,s})
(1-\zeta_r)^{1-b}\zeta_r^b
\operatorname{eq}(\zeta_{>r},s).
$$

Cached binary-prefix sums evaluate partial intervals without division. A full-prefix product $\prod_{j<r}(Z_j+O_j)=0$ does **not** authorize dropping partial intervals: $\zeta=2$, $y=1/3$ gives cancellation of the full sum while a singleton mask remains nonzero. This is not the product of a mask MLE and an equality MLE at the bound point.

Decoded-RAM preparation separately contracts the sixteen permuted raw address axes and

$$
D(d)=\sum_{\ell=0}^{25}32^\ell\operatorname{eq}(d,\ell).
$$

It keeps real plane25, zero weights for virtual planes26–31, and exact base-field powers. At most32 public digit cells and bounded eight-bit address factors are cached; no witness table or commitment is replaced. The complete canonical-mode accumulation module passed29 tests, including all21 decoded rounds, admitted permutations, non-Boolean points, mask cancellation and zero-factor provider errors. Independent static arithmetic review found no concrete defect under sealed-ledger admission.

The public verifier now borrows the five received tail blocks, but still reconstructs and compares literal history and endpoint bindings before full native verification. The driver consumes the owned receipt after encoding, retains only frontend metadata for mutations, and restores each in-place wire edit. Complete verifier and driver modules passed97 and2 tests; strict production-library Clippy passed, with Cargo manifest `default-features` warnings still present. Independent static boundary review found no concrete defect.

Subsequent lifetime pruning releases already-ingested handoffs, redundant public/range rosters, producer range history and obsolete ledger buffers before later phases. It retains original source handles and the complete reconstructed query history. The nested canonical history remains unchanged, including its repeated pre-range context; no hash or digest replaces literal evidence.

The canonical `bash autoresearch.sh` delivery gate executes **both** fixed-seed Dense1/2/3/256 controls and genuine Jolt1/2/3/256 legal-cut workloads. The expanded whole-harness run completed successfully and emitted `METRIC accumulation_dense256_prover_ms=8056.668435` only after both matrices passed independent final verification and their mutation matrices. That primary metric is the Dense engine measurement, **not** the genuine Jolt256 producer time or a recursive-aggregation comparison. The versioned `akita/autoresearch.sh` contains the same exercised workload body; the workspace entrypoint delegates to it.

After that full run returned, the workspace script was replaced by the thin canonical delegate. A separate 35-second entrypoint check rebuilt the current Dense driver and completed Dense1 and Dense2, including all thirteen mutations for each, before its deliberate timeout. This verifies real dispatch through the packaged path; it is **not** another complete harness pass.

### 12.5 Library checks and source-boundary review

The six affected Akita library suites completed with 775 passed and 6 ignored tests. The configuration, CPU-backend and PCS suites passed together; after removing an incidental planner-output fixture, the complete planner, prover and verifier suites passed in a second command. The canonical no-default-features accumulation module separately passed all29 tests. The complete original-plane native control also passed in release mode. The full harness independently completed the genuine256-cut gate described in §12.3.

The checks exposed two concrete fixture/source-boundary defects:

- The CPU Dense reader exposed allocator-only backing cells beyond the committed plan. It now borrows exactly `live_blocks × positions × digit_planes` cells. Real committed plan padding remains present and may be nonzero; allocator slack is not a committed source. Regression cases cover semantic values, real padding, bounds, overflow, producer ownership and foreign/imported handles.
- A coefficient-packing fixture used sixteen live blocks as sixteen opening claims, although its real batch contained one claim. Its scalar count now follows `num_total_polynomials` and the admitted digit role. The production opening guard and independent raw/non-Boolean/plane25 checks remain unchanged.

The removed planner fixture asserted private candidate/default output rather than a supported complete-row consumer contract. It was not repinned by weakening admission. The retained original-plane control exercises a complete native proof, not only planning or a root reduction.

Jolt's complete verifier, prover, driver, transcript and claims library/bin suites passed97,11,2,12 and143 tests respectively. Strict Clippy passed on the six affected Akita proof libraries and on the Jolt prover, verifier and transcript libraries. Cargo still reports the existing ignored `default-features` manifest settings; these results are not a warning-free whole-workspace lint claim. Scoped formatting passed; whole-workspace formatting was not clean because it included unrelated existing linked-workspace differences.

Independent final static reviews found no concrete protocol/source-boundary defect under the document's explicit classical-ROM, indexed-sampler, setup and native extraction/certificate premises. The reviews checked the finite seal/horizon, raw21 versus decoded16 domains, all original endpoints, actual plan padding, same-owner projection, conditional degree-one EOR sizing and complete native terminal/EOF path. They are not formal proof verification, concrete security admission, QROM/PQ security or zero knowledge; the separate runtime evidence is in §12.3.

The enabled `jolt-field` Solinas extension differential suite separately passed32 tests, including the explicit nonsquare-5 A7F7 extension. The initial command without `--features solinas` ran zero tests and is not counted as coverage.

### 12.6 Fork publication and paired-workspace integration

The protocol, companion certificates and research history now have one versioned home in `docs/query-relation-accumulation/`. Original workspace note paths are relative symlinks to those canonical files. The [publication guide](README.md) states the required sibling `akita/` and `jolt/` source layout; the [provenance record](SOURCE_PROVENANCE.md) identifies all13 parsed-paper snapshots without redistributing the papers, QMD database or embeddings. Relative research-file and current Jolt-source link targets were checked; no unresolved targets remained. This does not check every fragment or external server.

The canonical full harness is `akita/autoresearch.sh`; only its workspace-root calculation differs from the original workspace copy. Its shell syntax passed. The original workspace script was not overwritten while the outstanding full run was active. Both still execute the same Dense and genuine Jolt matrices with the final metric gated on complete success; neither relocation nor syntax checking establishes an unobserved proof run.

Eighteen affected Rust workflows now provision matching sibling checkouts. YAML parsing and main checkout-path validation passed for62 paired jobs. Current-repository event revisions, including PR merges, remain the tested source; companion checkouts select the supported accumulation branch. No GitHub Actions execution or actionlint result is claimed.

The broader Jolt all-target publication check exposed an ordinary SDK caller still using the old six-argument Dory prover interface. The compatibility migration now shares one prepared, field-specific frontend compiler between the ordinary Dory and native Akita installers. Dory retains its own homomorphic closing and late BlindFold policy; native Akita retains its own prefix packing, original PCS completion and legacy scalar bridge. Sharing relation metadata does not transfer Dory homomorphism or grant indexed-transcript security.

The ordinary native setup now carries required, ordered `dense_producers` metadata from its actual grouped setup: semantic advice roles, original mandatory indices, bounded/full-width layouts and exact producer profiles. This intentionally changes serialized PCS setup encodings; old setup files must be regenerated. The existing program preprocessing digest remains program-only and does **not** bind these new PCS fields. They belong to the separately trusted canonical PCS setup serialization and equality; no new preprocessing hash or proof-derived profile authority is introduced.

The complete focused `legacy_schema` and `legacy_emissions` test modules passed 3 and 2 tests respectively. The latter exercises the reviewed empty full-round defect: an empty coefficient vector remains `[]`, is admitted with its actual framing, and is not rewritten as `[0]`. Independent static review also exposed and resolved incorrect native schema charges, missing legacy-constructor imports and rejection of an unused-but-declared scalar in the supported single-virtual-polynomial geometry. Commitment installation charges 23 nodes; a prefix charges $4(n_{\rm point}+n_{\rm eval})+20$, including its selector event even at zero length. A declared degree-zero scalar retains its squeeze and receives bounded arity 2. Re-review found no remaining defect in those inspected boundaries.

The ordinary-only Akita verifier library check passed without indexed-feature unification, and the complete `jolt-akita` library suite passed 46 tests, including required setup transport and ordered producer/profile rejection. The complete ordinary native `akita_e2e` file then passed 14 tests, including the real single-virtual-polynomial regression, advice, committed programs and retained rejection cases. The native prefix module passed its actual reduction/mismatched-geometry test, including zero selectors. The three complete Fiat–Shamir attack files passed 5 clear-Dory, 4 ZK-Dory and 4 native-Akita tests: all seven original attacks remain across their modes, alongside the two repeated checked-adapter regressions. Independent static review found no defect in the checked/frozen transcript adapter. These are not indexed-extraction or QROM results.

The real SDK fixture generator built and ran the Fibonacci guest through the current local Jolt CLI, produced its proof and regenerated the fixtures; the independent SDK verifier test passed with `CI=1`, so missing fixtures could not silently skip verification. The local CLI is selected with `JOLT_PATH`, not an unrelated globally installed binary. The standalone Spartan suites passed 9 tests with real Dory and HyperKZG openings; independent static review found no compatibility defect after restoring its historical plain labeled commitment and original sumcheck event sequence. This remains a separate clear baseline, **not** an accumulation wrapper.

The complete default Jolt workspace/all-target check and the selected Fr and ordinary-Akita fixture/profile all-target checks passed. The trace-witness module passed 27 tests and the Dory commit/open/verify file passed 21. Field-inline acceptance and backend/tamper suites passed 5 clear-Dory, 4 ZK-Dory and 5 native-Akita tests. Missing field-geometry imports and the moved full-width commitment capability were corrected without changing their relations or source profiles. One native fixture that asserted internal rejection wording on an intentionally fake statement was deleted rather than repinned; production duplicate-role admission remains unchanged.

Both excluded verifier-fuzz builds passed. The existing fixture generator was migrated to real emission plans and regenerated all six genuine clear/ZK bundles, with EOF round-trips and independent full verification. The actual target binaries replayed all six clear and seven ZK seed files successfully. This was fixed-corpus replay, **not** a coverage-guided fuzz campaign; sanitizer integration was unavailable. Independent source review found no defect in the source-fixture/plan boundary. Real benchmark execution exposed a missing standalone `13 variables, 4 polynomials` dense schedule. The repair uses dense-only setup for dense batches and a benchmark-owned, canonically planned and validated supplemental catalog; production catalogs and registered workload dimensions remain unchanged. Real Criterion test mode then completed all four selected smallest-case paths: direct dense opening, direct one-hot opening, native four-polynomial batch proving and prefix-packed proving. No benchmark timing or parity result follows from that execution mode.

The benchmark adds only existing in-tree `akita-planner` and `akita-schedules` dev-dependency edges. Lockfile comparison found no package/version/source/checksum additions or removals and only those two new edges on `jolt-dory`. The widened D1 check exposed 141 conversion, arithmetic, indexing, exhaustive-match and style findings; those fixes are complete. Strict `--lib` Clippy with `-D warnings` passed for eight affected libraries with `indexed-accumulation,transcript-d1` and seven with the Fr/ZK configuration. The final indexed/D1 complete library regression passed 130 tests, and the current ZK Fiat–Shamir attack suite passed all four cases. Scoped formatting passed. Two independent static reviews found no canonical-encoding, source-order, arithmetic, endpoint-retention or error-state defect; they used the diagnostic inventories and edit history because these new feature files had no committed baseline, and did **not** perform a full historical byte-equivalence comparison. These checks do not license permissive bindings or stronger accumulation assumptions. Cargo's ignored `default-features` settings and a third-party `higher-kinded-types` future-incompatibility warning remain; no warning-free whole-workspace claim is made.

The current clear Fiat–Shamir attack suite also passed all five cases with the existing local Jolt compiler selected through `JOLT_PATH`. The first invocation without that compiler environment failed during guest generation; the configured invocation exercised the actual proofs rather than skipping fixtures.

The D1 owner uses the pinned `shake = 0.1.0` because the existing `sha3 = 0.12.0` removed its SHAKE types into that crate ([upstream change](https://github.com/RustCrypto/hashes/pull/869)). The cached crate declares `MIT OR Apache-2.0` and forbids unsafe code in its own source. Its [RustCrypto repository](https://github.com/RustCrypto/XOFs) was checked for maintenance activity. This is a dependency review, not an audit of all transitive unsafe code or a concrete-hash/QROM theorem.

Both fork origins are configured for `Loris-EPFL`, and the local SSH identity authenticated as that account over GitHub's port443 endpoint. The local runtime and integration gates above are complete. Publication must be identified by the actual commit IDs and fork refs; configured remotes or this document alone do not prove that a push occurred.
