# Native Akita instance accumulation: adaptive soundness and extraction

## 1. Scope and verdict

**Status: design investigation; the efficient native chain theorem is open.** This document concerns accumulation of a native **instance and private relation witness before a full segment PCS proof**. Each update admits an old accumulator and fresh VM-segment sources, reuses Akita's algebraic recursive reductions, and produces a new pending instance. It is not aggregation of completed segment proofs or public terminal residuals. No source implementation, checking command, or benchmark is part of this investigation.

The invariant to preserve is the exact original commitment-side source relation, together with its evaluation/frontend claims and consecutive execution links. Keeping one bounded public commitment `C`, or pausing an existing prover before its terminal, does not establish that invariant.

**Depth quantifiers.** The user has not required unbounded updates. A deployment may support a declared finite maximum \(T_{\max}\). For each fixed constant \(T_{\max}\), a finite unrolled extractor of cost \(N^{O(T_{\max})}\) can be formally polynomial in \(N,\lambda\), yet potentially concretely unusable. A single polynomial in growing \(T\) is a **stronger research target**, not a prerequisite imposed on every finite deployment. This document distinguishes the finite final-only route in §7.3 from the stronger uniform-depth route in §11. No practical \(T_{\max}\), closing profile, or concrete reduction-security certificate is claimed.

The central answer has three parts:

1. A correctly **SAME-handle** range/domain certificate for old and fresh sources can repair source-shortness and strong/weak projection matching. It does **not**, merely by doing so, provide the descendant-certified counterfactual successor evaluations assumed by Akita Theorem 10.23.
2. Independently authenticating the **complete successor source/domain/evaluation relation in every accepting fork child** removes the need to invoke a history suffix to obtain that child's endpoint witness. It costs real endpoint certification at every update and still requires a local-topology and adaptive-composition theorem. A single published final witness does not supply those children **for free**; a finite assembled-tree extractor can instead obtain their own endpoints by paid continuation replay to that branch's complete final decision (§7.3).
3. Reducing range certificates into the same pending successor is not intrinsically invalid: Lova, LF+, and SALSAA demonstrate genuine norm/range reductions. But calling such certificates “independent” when their only authentication is extraction from that successor is circular **as a proposed independent endpoint oracle**. A joint reduction can break that logical cycle only with an actual proof, explicit leaves, and a uniform extraction-cost analysis. None of the cited Akita theorems supplies this new native old-plus-fresh chain theorem.

The finite-support theorem needed for final-only unrolling is specified in §7.3. The stronger uniform-growing-depth theorem is specified in §11. A conservative, revealing exact-local-witness replay construction is available (§8); it is not the intended cheap, private, final-only solution. The cheaper alternatives remain conditional rather than being presented as a magic local bridge.

This document complements the six obligation documents, especially [commitment consistency](AKITA_ACCUMULATION_03_COMMITMENT_CONSISTENCY.md), [polynomial extraction](AKITA_ACCUMULATION_04_POLYNOMIAL_EXTRACTION.md), and [history/final acceptance](AKITA_ACCUMULATION_06_HISTORY_AND_FINAL_ACCEPTANCE.md). Their proposed native relations remain conditional. The public completed-proof route in [the protocol document](AKITA_ACCUMULATION_PROTOCOL.md) is **unselected** and is not used to satisfy this task.

## 2. Evidence boundaries

Paper claims below were checked against the live qmd MCP collection `lattice-papers`, using its indexed Markdown, not inferred from prior notes. The relevant Akita text expressly separates source decommitment, local descendant-conditioned extraction, fixed-schedule extraction, and classical-ROM knowledge soundness. The online BCLMS21 published paper was also read directly. References and exact retrieved line ranges are in §13.

The following are **not** established by these readings: equivalence between the indexed paper and every online revision; conformity of a particular Rust implementation to the paper; a selected source base; a feasible fixed-capacity native receiver; or a new adaptive chain theorem. This document makes no code-level claim and does not mix the legacy Jolt/markosg04 outer with modern LayerZero/Akita native APIs or transcripts. Source-base selection belongs to the integration design.

## 3. Relations: what must actually be accumulated

Fix public parameters/profile \(P\). They include exact original commitment shapes, matrix/view identities, rings and embeddings, full physical source layouts, digit alphabets, decoder/packing maps, declared zero spans, challenge families and filters, bounds, and setup-registry obligations. Original handles and their frozen ranks must not be replaced by receiver commitments with different source semantics.

Write \(\mathrm{cm}=(\mathrm{shape}_{\rm com},u_{\rm pub})\). A commitment-side source witness is

\[
 \sigma=(s,\widehat t,d_{B/F}),\qquad
 f=\mathrm{Dec}_{\mathrm{shape}_{\rm com}}(s).
\]

Here all physical source blocks and all active inner-image/compression digits are included. Akita Definition 10.13's algebraic decommitment relation includes the original \(A/B/F\) equations and certified source-path alphabets; it does not require canonical source gadget digits, an honest padding-image predicate, or a claimed evaluation. Define the stronger proposed domain relation

\[
 R_{\rm dom}^{\rm bd}(P,\mathrm{cm};\sigma)
 :=\mathrm{Rel}_{\rm com}^{\rm Akita}(P,\mathrm{cm};\sigma)
 \land\mathrm{SourceDomain}_P(\sigma).                    \tag{D}
\]

`SourceDomain` must bound the **actual commitment-side sources**, not only their decoded values. It checks required physical zeros and source alphabets. If the receiver uses an injective identity full-grid source map, a decoded-vector bound may imply a source bound; otherwise that implication requires its own proof. A noninjective gadget map can hide large source planes behind a small decoded vector. Honest canonical padding is not an adversarial-domain certificate.

A pending ordinary accumulator instance has the form

\[
 a=(P_{\rm id},\mathrm{cm},r,v,\mathrm{pendingSetup},
       \mathrm{run},t,\mathrm{epoch},\mathrm{boundary}).
\]

Its private witness \(w=(\sigma,\omega,\zeta)\) carries the exact source, allowed opening auxiliaries, and any additional application relation data. The proposed accumulator relation is

\[
 R_{\rm acc}^{\rm bd}(P,a;w)
 :=R_{\rm dom}^{\rm bd}(P,\mathrm{cm};\sigma)
   \land\widetilde{\mathrm{Dec}(s)}(r)=v
   \land\mathrm{PendingSetupTruth}_P(a;w)
   \land\mathrm{AccSemantics}_P(a;w).                    \tag{A}
\]

The last conjunct contains precisely the retained native relation semantics chosen by the integration design; it cannot be a trust bit or “was honestly generated.” Any extra pending setup opening needs its own authenticated endpoint or a proved combined relation. A proof of the ordinary opening alone does not discharge it.

For fresh segment input \(x\), define

\[
 R_{\rm seg}^{\rm bd}(P,x;\nu)
 :=\bigwedge_g R_{\rm dom}^{\rm bd}(P,\mathrm{cm}_g;\sigma_g)
   \land\mathrm{NativeClaims}_P(x;\nu)
   \land\mathrm{FrontendRelation}_P(x;\nu).             \tag{S}
\]

`NativeClaims` includes **every** admitted evaluation at its actual point, including any frontend sumcheck-to-PCS claims. `FrontendRelation` contains the segment's real constraint/VM relation, not acceptance of a completed segment PCS proof. If an application derives frontend knowledge by separately composing native claim extraction with a frontend reduction, that composition must be proved. A PCS extraction theorem alone does not extract an entire CPU execution.

For a public update record \(j=(a,x,\tau,a')\), the exact local bridge relation is

\[
\begin{split}
 J_P(j;w,\nu,w',\chi):={}&R_{\rm acc}^{\rm bd}(P,a;w)
 \land R_{\rm seg}^{\rm bd}(P,x;\nu)
 \land R_{\rm acc}^{\rm bd}(P,a';w')\\
 &\land\mathrm{NativeStepExact}_P(a,x,\tau,a';w,\nu,w',\chi)
 \land\mathrm{ConsecutiveLinks}_P(a,x,a';\nu,\chi).       \tag{J}
\end{split}
\]

`NativeStepExact` includes the actual old/fresh source maps, original \(A/B/F\) paths, shared \(D/H\) path, all range/norm/quotient/relation identities, tensor and claim reductions when applicable, exact Equation (109) successor construction, physical layout/zeros, and setup obligations. `ConsecutiveLinks` binds program/run identity, segment index, initial/final machine boundaries, memory continuity and IO. These are not inferred from the fact that two commitments use one setup seed.

This defines a finite arithmetic relation once the chosen frontend and layouts are supplied. It is not a claim that an efficient private argument for \(J_P\) is already available. Fixed-shape closure and all-valid-domain completeness remain separate obligations.

## 4. Local atomic soundness is not local knowledge extraction

### 4.1 Algorithms and completeness

A native step has public verifier

\[
 \mathsf{StepV}_P(a,x,\tau)\to a'\text{ or }\bot
\]

and prover \(\mathsf{StepP}_P(a,w,x,\nu)\to(a',w',\tau)\). The verifier reconstructs the successor; it must not merely accept a prover-supplied arbitrary commitment. Input sources and claims are frozen before the challenges to which their binding is relevant.

The all-valid-state completeness requirement is

\[
 R_{\rm acc}^{\rm bd}(a;w)\land R_{\rm seg}^{\rm bd}(x;\nu)
 \Longrightarrow
 \Pr[\mathsf{StepV}(a,x,\tau)=a'\land
             R_{\rm acc}^{\rm bd}(a';w')]\ge1-\delta_{\rm comp}. \tag{C}
\]

It is pointwise over every admitted predecessor/fresh witness, including supplied allowed noncanonical commitment paths. Bounds derived only for generated canonical states do not establish (C). Finite sampler/response-search failure and every retry count belong in \(\delta_{\rm comp}\) and the query ledger.

### 4.2 Atomic acceptance property

Let \(L_{\rm acc}(a):=\exists w\,R_{\rm acc}^{\rm bd}(a;w)\), and similarly \(L_{\rm seg}\). A semantic backward atomic property is

\[
 \Pr[\mathsf{StepV}(a,x,\tau)=a'\land L_{\rm acc}(a')
       \land(\neg L_{\rm acc}(a)\lor\neg L_{\rm seg}(x))]
 \le\varepsilon_{\rm at}.                              \tag{AS}
\]

The adversary, success experiment, computationally generated certificates/witnesses, and oracle access must be specified. In particular, a computational knowledge theorem whose adversary supplies a successor witness does not automatically prove a language statement about an arbitrary unbounded existential witness. One may instead formulate (AS) for efficiently generated complete successor tuples or independently accepted successor certificates, with their soundness/composition assumptions made explicit.

If (AS) is proved for the actual adaptive update experiment, authenticated accepted history plus a true final accumulator yields truth of every fresh instance by identifying the first invalid-to-valid transition. This is an acceptance argument. It does not produce source/VM witnesses in polynomial time.

A random-linear-combination calculation is useful only after the exact native commitment/evaluation link has made its residuals well-defined and prechallenge-fixed. Public residual arithmetic alone is not a proof of (AS) for (A)/(S).

### 4.3 Knowledge/atomic reduction interface

A local reduction of knowledge must handle an EPT malicious process that, from input \((a,x,z)\), interacts with \(\mathsf{StepV}\) and outputs

\[
  (a',w',\tau,z'),\qquad R_{\rm acc}^{\rm bd}(P,a';w')=1.
\]

The auxiliary strings \(z,z'\) include prior records, next chosen states, oracle answers, retries/aborts and application data. For success probability \(\epsilon\), an extractor must recover

\[
 (w,\nu)\in R_{\rm acc}^{\rm bd}(P,a)\times
                  R_{\rm seg}^{\rm bd}(P,x)             \tag{K}
\]

with probability at least \(\epsilon-\kappa_{\rm loc}\), or output an explicitly budgeted short nonzero matrix-view collision. A witness-extended formulation additionally preserves/emulates the actual published update and arbitrary auxiliary output while augmenting it with witnesses; (J) is a sufficient stronger local target.

**The output-witness oracle is substantive.** In a forking proof, the malicious process/oracle must produce \(w'\) for each accepting output sampled by the extractor, using fresh invocation randomness under the relevant experiment. Giving the extractor just one witness for the published successor is not this interface. Even a valid local (K) theorem, considered separately at every depth, does not establish a single uniform polynomial runtime over arbitrary \(T\).

## 5. What Akita actually proves at this cut

### 5.1 One-fold theorem: real premise, real output

Akita Definition 10.22 and Theorem 10.23 require a **descendant-certified nested coordinate-wise accepting family**. For every folding child there is a complete strong-check mixed subtree, and each leaf's outgoing opening has already been authenticated by the recursive suffix. The outgoing commitment is made after the folding challenge, so different fold children may have different outgoing commitments. Later challenges need not agree across those children.

Given that family, the deterministic extractor works backwards through the ring/range/norm/relation checks, then subtracts coordinate siblings. For an incoming source block,

\[
 \rho s=z^{(1)}-z^{(0)},\qquad
 \rho=\iota(\Delta c)\in R^\times.                       \tag{W}
\]

It returns a Definition 10.14 weak opening, satisfying commitment, partial and public evaluation equations, or a scheduled matrix collision. Its time is polynomial in the **supplied tree** and explicit witness length. The theorem explicitly says it is not a standalone knowledge claim for an unauthenticated outgoing opening.

The weak bound is on \(\rho s\), not on \(s\). Unit inversion need not preserve coefficient or Euclidean norm. Remark 10.32 expressly declines canonical/short-source extraction. This is a different problem from authenticating counterfactual endpoints.

### 5.2 Strong–weak composition cannot silently reorder Akita

SuperNeo Definitions 16–18 and Theorem 12 compose a strong reduction followed by a weak one using the **same intermediate public projection**. Independent oracle calls use fresh randomness, and the uniqueness adversary's runtime includes all oracle work. Strong extraction relies on uniqueness of the downstream extracted witness under that shared projection.

Akita §10.2 explicitly does not use this sequential theorem as a black box. Its weak folding challenge occurs **before** its strong checks, and the outgoing commitment depends on that challenge. Its proof obtains the invariant by descendant-certified nested trees instead.

Adding a pre-fold source checkpoint could give a common incoming projection at a newly designed strong–weak cut. It does not prove that the remaining protocol is SuperNeo's weak reduction, or that reordering Akita messages preserves challenge binding. Such a cut needs an actual new relation/protocol proof. Theorem 12 also does not make nested extraction oracles cost-free.

### 5.3 Fixed schedule and Fiat–Shamir

Akita Theorem 10.31 authenticates the entire **admissible fixed schedule** by backward induction from its terminal, returning the full-grid algebraic batch relation (202), or a collision. Its runtime depends on

\[
 \mathsf{Tree}(\mathrm{sch})=\prod_{r\in\mathrm{Rounds}(\mathrm{sch})}b_r,
 \quad
 b_r=\begin{cases}
  1+\sum_u(k_{r,u}-1),&\text{flat coordinate-wise stage},\\
  \prod_u k_{r,u},&\text{nested interpolation stage}.
 \end{cases}                                           \tag{T}
\]

The actual ledger includes folds, tensor prefixes, range anchors/stages, relation-row interpolation, setup checks, claim batches and terminal challenges. For example a fold has factor \(1+\sum_g W_g\), while an initial range anchor has factor \(2^{\mu'}\). Definition 9.2/Equation (176) requires this whole product to lie below **one fixed polynomial in security and explicit instance/witness length**.

Theorem 10.33 supplies classical-ROM adaptive batch knowledge soundness for that assembled admissible schedule, with bound

\[
 (Q+1)\varepsilon_{\rm int}(\mathrm{sch})+
 \Delta_{\rm setup}+\sum_{I\in\mathcal I(\mathrm{sch})}
                  \mathsf{Adv}_{\rm MSIS_I}(\mathcal B_I). \tag{FS}
\]

Equation (227) counts tree, adversary, verifier and fixed-filter programming work. Neither (T) nor (FS) is a theorem that one may insert arbitrarily many fresh VM groups into the existing recursive receiver topology. The paper's shrinking recursive schedule and a fixed-size accumulator repeatedly re-entered for \(T\) fresh segments are different schedules.

## 6. SAME-handle range certification: exact repair and exact limitation

### 6.1 Certificate strength matters

There are four distinct interfaces, ordered by what they establish:

| Certificate | What acceptance/extraction supplies | What is still absent |
|---|---|---|
| Range of a decoded vector under a different handle | A bounded vector somewhere | Original-handle source consistency, original claim truth |
| SAME-handle sound source-domain certificate | Existence of a source in (D) | Efficient source extraction; claimed evaluation truth |
| SAME-handle independently extractable source-domain certificate | A source in (D), with paid extractor cost | Claimed evaluation/frontend truth; exact update-to-successor link |
| SAME-handle independently extractable complete endpoint certificate | A witness in (A), for that exact endpoint | Local old/fresh-to-successor reduction, history/frontend composition |

A proof that is merely sound for (D) is not an opening-witness oracle. Conversely, an independently extractable proof for (D) may establish knowledge of the source **without a full evaluation-opening proof**. An extractor can then evaluate that source. This does not guarantee that the prover's claimed value equals the computed value; proving that guarantee is still the local relation/soundness problem. If all old/fresh certificates already prove (A)/(S), incoming knowledge is supplied by the certificates themselves, but the construction has paid for those truths separately, rather than obtaining them from native final-only deferral.

### 6.2 Projection matching upgrades weak sources conditionally

Let an independent source certificate extract \(\sigma_{\rm bd}\) under the exact original handle. Let the native local fold extract weak \(\sigma_{\rm wk}\) under that same handle. Use Akita's ordered original commitment projection

\[
 \phi_{\rm in}(a,x)=
   (\mathrm{shape}_{\rm com,g},u_{\rm pub,g})_{g\text{ in fixed order}},
 \qquad \mathsf{Src}(\omega)= (s,\widehat t,d_{B/F}).     \tag{P}
\]

Profile/source-map/setup identities are authenticated alongside this projection. Do **not** compare whole weak openings: their points, partials, \(D/H\) paths and challenge differences may legitimately differ. Corollary 10.17 applies to commitment-side \(\mathsf{Src}\), with appropriate source-comparison classes.

First compare the \(B/F\) paths. A disagreement yields their actual cross-profile digit collision. Otherwise the inner images agree, and for one block \(A s_{\rm wk}=A s_{\rm bd}\). With \(r=\rho s_{\rm wk}\), \(\|r\|_\infty\le B_r\), \(\|\rho\|_1\le K_\rho\), and \(\|s_{\rm bd}\|_\infty\le B_s\), form

\[
 z_A=r-\rho s_{\rm bd}
     =\rho(s_{\rm wk}-s_{\rm bd}),\quad
 A z_A=0,\quad
 \|z_A\|_\infty\le B_r+K_\rho B_s.                     \tag{M}
\]

Since \(\rho\) is a unit, different sources make this a nonzero ring kernel vector. This is the mixed weak/strong comparison, with the strong occurrence having multiplier one. General weak/weak comparisons use

\[
 z_A=\rho'r-\rho r',\qquad
 \|z_A\|_\infty\le K_{\rho'}B_r+K_\rho B_{r'}.          \tag{MM}
\]

Use exact matrix widths/rings/radii and the original frozen ranks, all \(B/F\) alphabet diameters, and Akita's shared-prefix-to-ordinary-MSIS reduction. A later receiver cannot retroactively strengthen an original handle's rank. Certifier/native and original/receiver comparisons must be priced before admission.

**Conditional consequence:** if the certificate extractor, exact source maps and mixed collision inventory are valid, unequal source projections break one scheduled assumption; otherwise \(s_{\rm wk}=s_{\rm bd}\), and the already extracted evaluation transfers to the bounded source. No norm-preserving inverse is asserted.

This repairs the weak-to-bounded source implication **after native weak extraction has succeeded**. It does not supply the descendant-certified family needed to start that extraction.

### 6.3 Why old/fresh certification does not fix successor forks

Consider a prefix ending just before a fold challenge. Two coordinate siblings produce \(C'^{(0)}\) and \(C'^{(1)}\); these generally differ. Certificates fixed for the old \(C\) and fresh \(C_g\) authenticate those sources, not either child commitment. Theorem 10.23 still requires complete authenticated outgoing evaluations beneath both children's strong-check trees.

There is no inconsistency between knowing the input sources and not yet knowing an adversarially formed extended successor. That successor also contains response digits, quotient/range auxiliaries, compression paths and claimed partial evaluations. Recomputing an honest successor from the certified inputs does not show that the **published adversarial** successor is that successor. The exact linkage must be checked or reduced soundly.

A certificate for the actual final \(C_T\) says nothing about a different \(C_T^{(b)}\) obtained after rewinding an earlier challenge. Similarly, the next published update's “old source” certificate authenticates only its chosen old handle. It does not hand out certificates for all alternate successors encountered by a prior extractor. If a counterfactual continuation produces an independently verified **complete** certificate for its own old endpoint, that certificate can be used—but its production/extraction cost and evaluation scope must be counted.

Thus SAME-handle source certification is necessary to the proposed strong domain, but not sufficient to eliminate descendant dependence.

## 7. Actual extraction recurrences, with invoked oracle work

### 7.1 Tree/suffix extraction

Let \(d\) be the number of native updates remaining below an extraction node. Let \(M_d\) be the number of endpoint-oracle invocations made by its tree/rewinding procedure, and \(\mathcal F_{d,k}\) the state preceding invocation \(k\). The honest accounting identity is

\[
 \mathbb E[R_d]
 =\mathbb E[W_d^{\rm local}]
 +\mathbb E\!\left[\sum_{k=1}^{M_d}
                W_{d,k}^{\rm endpoint}\right].         \tag{R0}
\]

Endpoint work includes rerunning the adversary/continuation, challenge sampling and conditional tape programming, verification, certificate extraction, and any invoked suffix extractor. Expectations cannot generally be replaced by \(\mathbb E[M_d]\) times an unconditional average endpoint cost: query choice, selective abort and endpoint runtime can be correlated. A uniform conditional bound or a joint adaptive total-work theorem is needed.

For a **supplied fixed mixed tree** with \(B_d\) authenticated-leaf calls, the deterministic structural recurrence has the form

\[
 R_d\le p_d+B_dR_{d-1},\qquad
 R_0=R_{\rm terminal}.                                \tag{R1}
\]

Here \(p_d\) counts local subtree processing and generation/checking work not counted recursively. Expanding it gives

\[
 R_d\le R_0\prod_{i=1}^{d}B_i+
       \sum_{j=1}^{d}p_j\prod_{i=j+1}^{d}B_i.           \tag{R2}
\]

For actual ROM extraction replace the supplied-tree cost by the real expected tree-generation/forking bound, including success/gap dependence where required by its extractor, all \(Q\) probes, and filter costs. Akita Equation (227) is polynomial in the tree rather than an exact universal call-count formula. (R1)–(R2) expose the structural multiplication; they do not claim an exact coefficient for that ROM implementation.

Repeated fixed-capacity nontrivial updates have a tree with at least one factor \(b>1\) per update, hence \(\mathsf{Tree}_T\ge b^T\). If \(B(N)=N^c\), the straightforward iteration has \(N^{cT}\), not a uniform polynomial in \((\lambda,T,N)\). **For fixed constant \(T_{\max}\)**, however, \(N^{cT_{\max}}\) is a polynomial in \(N\). Its supplied-tree deterministic extractor can be PPT, and its justified classical-ROM extractor EPT. That distinction is enough to avoid a formal growing-depth impossibility claim, but not enough to claim practical extraction or a concrete security level.

This is an obstruction to **the inherited tree extraction proof for the stronger growing-depth target**, not a lower bound against all possible lattice accumulation extractors and not exclusion of a finite supported-depth deployment. A finite route must explicitly charge its potentially enormous tree and reduction runtime (§7.3); calling \(T_{\max}\) “a constant” does not make those concrete costs disappear.

### 7.2 Independent local endpoints

Suppose every accepting counterfactual endpoint has an independent complete certificate whose extractor does not call the native history extractor. Let \(C_{\rm end}(\mathcal F_{d,k})\) be its full conditional extraction cost. Under proved uniform bounds for the local tree and its endpoint oracle,

\[
 R_{\rm loc,t}\le p(\lambda,N_t,B_{\rm loc,t},Q_t,T_A,T_{\rm filter})
      +\mathbb E\!\left[\sum_{k=1}^{M_t} C_{\rm end}(\mathcal F_{t,k})\right]. \tag{R3}
\]

There is no \(R_{t-1}\) inside \(C_{\rm end}\). That is the noncircularity condition, not a notational convenience. A clear endpoint witness makes endpoint extraction reading/checking the supplied data; a private endpoint proof must supply its own paid independent extractor.

If independently generated archived local records can be witness-extended in one joint adaptive experiment, then

\[
 R_{\rm hist}(T)=R_{\rm initial}+R_{\rm final}
                  +\sum_{t=1}^{T}R_{\rm loc,t}
                  +R_{\rm projection/link}(T).          \tag{R4}
\]

For uniform polynomial local/certificate work, polynomial total query/adversary budgets and explicit length \(L\), this is bounded by one polynomial in \((\lambda,T,N,L,Q,T_A)\). Regenerating the history adversary once per local extraction can add a polynomial \(T T_A\) factor; it is not zero cost. A stronger streaming witness-extension interface can make the incremental bookkeeping additive directly. Neither bound is supplied merely by per-record standalone EPT claims.

Multi-instance witness-extended emulation with auxiliary input/output is a sufficient contract for independently generated archival certificates, and is needed by the cited general PCD composition. BCLMS21 Remark 2 explicitly notes that single-instance extraction can suffice for path IVC with one node per layer. Therefore this document does **not** claim multi-instance extraction is necessary for every consecutive-segment path. A path-specific theorem can replace it if it proves the actual auxiliary-I/O, expected-time and **uniform quantitative** guarantees; citing the single-instance exception is not that proof.

### 7.3 Finite supported-depth final-only unrolling

**Conditional native variant, distinct from per-update complete endpoint proofs.** Fix \(T_{\max}\) as part of the public supported profile, independent of the adversary's challenges and setup realization. A run may stop at any \(T\le T_{\max}\); the exact chosen length and all source/profile descriptors must be bound by the protocol. The finite family of permitted stage/topology descriptions and matrix views is registered before setup. Fresh instances may be chosen adaptively during a run, but each original fresh handle, claim, shape and source map is frozen before its relevant local challenges. This route accumulates partial native instances, not completed segment PCS proofs.

#### 7.3.1 Exact finite protocol and source domain

1. **Initial anchor.** Independently certify/check the empty initial state and its declared full physical source domain. Bind the program/run and initial machine boundary.
2. **Source-domain reduction.** Use the exact original old/fresh handles. An initial/imported source and every applicable fresh source retain the source predicate (D), not merely response or decoded range. In the finite assembled-history variant, the prior receiver instead reuses its complete outgoing-domain reduction as proved in [range reuse](AKITA_INSTANCE_ACCUMULATION_RANGE_REUSE.md): the next ordinary endpoint authenticates the preceding strong tree, including newly fused required-zero checks. This does not provide local bounded knowledge for an arbitrary fixed old tuple.
3. **Identity bounded source option.** A receiver can commit directly to a full-grid signed digit/bitplane witness with \(\delta_{\rm com}=1\), so the commitment-side physical source itself lies in a finite certified alphabet. Every plane and physical zero span is checked; reconstruction of values and all frontend/native uses of those values must be exact. For fresh handles, identity bitplane encoding is usable only if it is their original committed representation. Re-encoding an existing nonlinear gadget source under a different commitment does not supply SAME-handle certification. Original multi-plane sources instead require their own source range predicate and original-handle bridge.
4. **Native old-plus-fresh step.** Run the actual generalized native carrier/evaluation-trace, range/norm/relation and setup reductions under the scheduled bounded sources and signed alphabets. Form the exact Equation (109) successor. Its extended witness must contain every source-domain, reconstruction, commitment, partial/evaluation, quotient/compression and boundary relation helper required by this topology. The signed alphabet, integer-lifting limits and every auxiliary address are fixed by the admitted profile; helpers may not be silently dropped.
5. **Complete local relation extension.** Prove that the source-range prefix's backward relation, abbreviated \(R_{\rm BR}\), identifies the very same \(\mathsf{Src}\) as the native fold and reconstructs the exact source/domain predicate. The required **BR-identity bridge** includes the original handle, full-grid map, all digit/reconstruction identities and fixed zeros. If its extracted source disagrees with the native source, a priced mixed matrix-view collision must result. It cannot certify only an unrelated short encoding. Every remaining frontend and consecutive VM link must either be retained as a fully checked public predicate or reduced into the specified native successor relation.
6. **Final complete decision, once.** After the last update reveal the complete final bounded source/decommitment witness \(w_T\), or prove its exact complete relation independently. A reveal-once decision checks all final original \(A/B/F\) equations, all physical alphabets/zeros, the full-grid evaluation, every pending setup claim and all other terminal obligations in (A). It is not a range-only reveal. This may replace the final privacy-preserving endpoint decision for this finite variant; it is nonhiding at the final step but does not require full opening/certification at each earlier update.
7. **Authenticate included prefixes.** Replay every actual native source-range/update record and exact derived successor, plus the final decision and execution links, or use a separately justified receipt proving those exact checks. No prefix omitted from the authenticated transcript is certified merely by opening \(C_T\).

The finite source alphabet makes an all-valid-source norm envelope expressible. [The source-query construction](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md) supplies the direct-coefficient raw-gadget observation bridge, while [receiver §10](AKITA_INSTANCE_ACCUMULATION_RECEIVER_PROFILE.md) gives a new Jolt-dimensioned arithmetic closure example. These close specific algebraic/counting gaps, not the exact transcript, finite-tree, collision-inventory or supported-profile admission.

#### 7.3.2 Derived conditional finite-list backward lemma

Akita §9.3 lines 3198–3200 restricts the **analyzed schedule policy** to application groups at the root and specified recursive/setup source lists later; it explicitly distinguishes policy restrictions from intrinsic commitment/offloading identities. Theorem 10.23's local backward algebra itself does not use the fact that a group is an application-root group, nor the absolute depth of its fold. Consequently, for the **direct coefficient-packing, identity-source variant with a fixed supplied incoming list**, the following is a derived conditional local generalization, not a fundamental unknown algebraic mechanism. It remains outside the assembled schedule theorem until its exact interfaces and finite adaptive experiment are instantiated. Evaluation-trace/tensor and raw-gadget variants are not automatically covered by this restricted derivation.

**Conditional finite-list backward lemma (direct coefficient/identity sources).** For every registered stage \(h\le T_{\max}\), let its ordered incoming list be

\[
 \mathcal G_h^{\rm in}
   =(\mathrm{OldReceiver}_h,
          \mathrm{FreshOriginal}_{h,1},\ldots,
          \mathrm{FreshOriginal}_{h,f_h},
          \mathrm{Setup}_h\text{ if present}).
                                                               \tag{FL}
\]

Assume the original source handles are fixed before the source-range prefix; the source decoder/packing is identity on the complete grid up to its declared coefficient-packing bijection; the direct coefficient maps and original \(A/B/F\), shared \(D/H\), unit-difference and range/relation rows satisfy the exact Akita local identities; and the prefix's BR-identity interface attaches its range/zero predicates and input evaluations to those **same** original sources. Assume every descendant leaf is authenticated, the required mixed interpolation family is supplied, and all source-comparison collision radii have been admitted. Then the family yields:

- weak openings/evaluations for **every** member of (FL), at that member's actual point and original commitment shape;
- bounded SAME-handle source witnesses after the proved BR-identity/projection bridge, or a nonzero collision in an explicitly enumerated original/receiver/mixed/setup matrix view;
- all included source-domain/reconstruction equations; frontend/link equations only if explicitly present in the extracted relation, otherwise they remain separately declared application obligations;
- a polynomial-time deterministic extractor in the supplied local tree and explicit local witness length, with the exact local factor \(b_h(N)\), local challenge error and source-comparison radii.

**Proof of the restricted local generalization.**

1. **Authenticate descendants.** For each folding child, begin with that child's actual authenticated outgoing sources/evaluations, obtained through its own suffix/complete final decision. Its later challenges may differ from those of other folding children. Source uniqueness identifies the commitment-side vector across extractions of a common outgoing handle, or gives its scheduled collision.
2. **Recover exact branch identities.** Apply the supplied local ring/range/norm/relation interpolation trees backwards from those leaves. As in Theorem 10.23, this removes later challenge values and yields the exact direct-coefficient native-ring rows, response digit ranges and commitment/opening paths separately for each folding child. Additional BR-identity/reconstruction rows are recovered only where the specified extension includes them. A fixed extra incoming group changes the ordered source indices and row lengths, not this backwards argument.
3. **Subtract unit coordinate siblings.** Compare the original \(B/F\) source paths and prechallenge shared \(D/H\) payloads; disagreement produces their priced digit collisions. Otherwise those digits/partials agree. For every supplied-list coordinate \((g,c,i)\), subtract two folding children differing only at that coordinate:

   \[
     \iota_g(\Delta c_{g,c,i})s_{g,c,i}
          =z_g^{(1)}-z_g^{(0)},\quad
     A_gs_{g,c,i}=t_{g,c,i},\quad
     e_{g,c,i}=\mathcal L_g(s_{g,c,i}).                 \tag{FL-sub}
   \]

   Here the source map is identity; the declared coefficient packing accounts for the carrier/ambient map. The difference is a unit in both required rings, so cancellation yields weak sources, original-handle decommitments and the native partial/evaluation equations for **every** supplied incoming group. No application-only-root or absolute-depth premise enters this coordinate subtraction. Unit inversion still does not establish shortness.
4. **Fix the original source across prefix branches.** Repeat the native extraction beneath each source-range-prefix branch. The BR-identity interface makes each resulting source an opening of the same original handle fixed before that prefix, using the same full-grid identity map. Corollary 10.17's comparison argument equates their \(\mathsf{Src}\) projections, or returns a short nonzero original/receiver/mixed matrix collision. Thus there is one source \(s_g\) for each original handle throughout that prefix's interpolation tree. Its evaluation point, partials and \(D/H\) path need not be identical across prefix branches.
5. **Interpolate source range and zeros.** Let \(\mathcal D_g\) be the declared signed alphabet, whose elements are distinct in the field, and set \(p_g(X)=\prod_{d\in\mathcal D_g}(X-d)\). The specified source-range reduction's exact anchored sumcheck/interpolation identities, now evaluated on the common original source, imply \(p_g(s_g[u])=0\) for every Boolean-indexed physical cell \(u\), and imply each designated zero predicate. This uses the complete nested anchor/interpolation tree and the required degree arities, not only coordinate pairs at a multilinear batching anchor. Consequently every source coefficient belongs to \(\mathcal D_g\), including its required physical zeros. BR-identity links any bitplane reconstruction equations to those same coefficients. Therefore the extracted original inputs are bounded, without a norm-preserving inverse or an independently completed per-step opening.

All operations above are polynomial in the supplied local tree and explicit local dimensions; its full interpolation arities define \(b_h(N)\). This proves the **conditional algebraic implication** for the restricted variant. The particular BR-prefix message/row interface, exact source maps and Equation (109) closure, valid original frozen collision/security parameters, application rows and finite adaptive-ROM tree-generation experiment are still uninstantiated certificates. The derivation does not assert a numerical admitted profile or silently extend to other modes/gadget representations.

The finite assembled theorem then composes this lemma, the actual source-range prefixes and the final **complete** decision through backward induction. A generalized classical-ROM mixed-tree theorem must cover the adaptive fresh choices at later stages: statement data frozen before each local challenge, explicit branch-specific outputs, source-projection uniqueness at each shared handle, the actual auxiliary outputs and every nonce/query. This is a finite-family extension of the assembled proof, not an automatic consequence of the existing fixed-root theorem.

#### 7.3.3 Obtaining counterfactual endpoints correctly

The final published revelation \(w_T\) authenticates only the published final endpoint. In the finite assembled extraction experiment, rewind an earlier challenge and rerun the **entire remaining malicious continuation**, including later fresh choices and the complete final decision. An accepting leaf of the extracted full mixed tree contains its **own** accepted final bounded witness; the final checker authenticates that branch's final source/evaluation. Backward induction along that branch obtains its earlier successor witnesses. No equality of later challenges or outgoing commitments across folding children is assumed.

The source list shared by coordinate siblings is the list fixed at their common prechallenge node. Later fresh lists can differ between continuations; they are bound and extracted in their own nodes, and do not become a fictitious common intermediate projection. Extraction/source matching must still recover the witnesses for the actual published branch and preserve the outputs required by the chosen finite adaptive experiment.

This is well founded because each invoked suffix has strictly fewer remaining update stages and terminates in the complete final decision. It does **not** provide a cheap independent local oracle. Its success/tree-generation guarantees must come from the assembled theorem, not from assuming an adversary accepts every arbitrary fork; conditional success, retries, selective abort and ROM programming costs remain charged.

#### 7.3.4 Formal polynomial bound versus concrete reduction security

Let \(N\) include the maximum local physical instance/witness size, let \(b(N)=\max_h b_h(N)\) include **all** local source-range, fold and strong-check tree factors, and let \(B_{\rm end}(N)\) count any final interactive endpoint factors. A clear exact final revelation has no additional random challenge/interpolation factor, although its complete checking cost remains. Then

\[
 \mathsf{Tree}_{\le T_{\max}}(N)
     \le B_{\rm end}(N)\prod_{h=1}^{T_{\max}}b_h(N)
     \le B_{\rm end}(N)b(N)^{T_{\max}}.                  \tag{FT}
\]

For \(b(N)\le N^c\) and polynomial \(B_{\rm end}\), this is \(N^{O(T_{\max})}\). For **each fixed constant** \(T_{\max}\), it satisfies a formal polynomial ceiling in explicit \(N,\lambda\), provided all parameter/filter/adversary costs are polynomial as well. This does not give one polynomial for growing \(T_{\max}\), and does not show a practical deployment.

Let \(R_{\rm red}\) be the concrete expected whole-tree/ROM reduction runtime, including all continuation replays, final-witness checks, filters, source matching and ordinary-MSIS embedding work. A candidate finite security bound has the form

\[
 \varepsilon_{\rm finite}
 \le (Q+1)\varepsilon_{\rm int}^{\rm extended}
        +\Delta_{\rm setup}
        +\sum_{I\in\mathcal I_{\rm finite}}
                    \mathsf{Adv}_{\rm MSIS_I}(R_{\rm red})
        +\varepsilon_{\rm application/receipt}.         \tag{FE}
\]

This equation is a **conditional target**, not a newly proved generalized ROM bound. \(\varepsilon_{\rm int}^{\rm extended}\) must charge each source-range/native/terminal stage once; any additional certificate or endpoint argument has its own costs/errors, without double counting already included stages. \(Q\) covers the actual shared FS lifetime and all probes. The shared-setup inventory includes the cross-profile comparisons required by the BR-identity lemma and all original frozen matrix views.

In particular, a lattice attack-cost estimate for a small-time adversary cannot be applied unchanged to a reduction whose \(N^{O(T_{\max})}\) runtime is enormous. Concrete extraction/reduction runtime and advantage assumptions at that runtime must meet the chosen security target; any proved forking loss is also charged. “Formal PPT for constant \(T_{\max}\)” is not a numerical 128-bit certificate, nor a verifier/prover performance-parity claim.

Matching the paper's formal EPT/MSIS reduction policy is a legitimate target; the user did not additionally require a tight numerical 128-bit reduction. State the complete runtime-sensitive advantage bound and its assumptions. If a numerical end-to-end bit claim is made, it must follow from those actual terms rather than the per-matrix attack-model label. This distinction does not waive the public \((\lambda_{\rm FS},Q_{\max})\) and exact round-ledger requirements in Akita Eqs. (223)–(231).

**Finite-route verdict.** Final-only revelation plus continuation-replayed full-tree extraction is a coherent conditional finite-depth proof route. It does not need per-update complete endpoint certificates or a new uniform-growing-\(T\) theorem. For the direct coefficient/identity-source supplied-list variant, the local backward/range implication is the conditional derivation in §7.3.2, not a fundamentally missing algebraic idea. Its exact BR-identity interfaces, full shape/source/parameter and Equation (109) closure certificates, application relation, actual assembled adaptive-ROM theorem, authenticated included history and credible concrete reduction-security/runtime ledger remain uninstantiated. Other variants need their own matched derivation. No practical supported \(T_{\max}\) is claimed.


## 8. Can range/domain certificates be reduced into the successor?

### 8.1 Independently completed source-only range proof

A genuine standalone argument for (D) can be verified at each update before folding, while leaving fresh evaluation-opening proofs unfinished. This does not automatically violate the requested pre-full-PCS cut: domain certification and PCS evaluation are different predicates.

It can give an input source checkpoint suitable for §6.2 and deterministic all-valid-source bounds. Its prover/verifier/communication and adaptive extraction costs must be measured separately. Source-only certification does not complete (K), because false claimed evaluations and false frontend claims remain possible until the native reduction binds them.

If the certificate's verifier internally needs an Akita evaluation opening, that opening must actually be completed independently, or have its own proved reduction. Calling an unfinished Akita claim a “verified range certificate” is invalid.

### 8.2 A domain reduction, not an independent certificate

A proposed range reduction can map a source-domain statement into additional successor claims rather than discharge it immediately. That is legitimate syntax for a reduction of knowledge:

\[
 R_{\rm dom}^{\rm bd}(\phi_{\rm in};\sigma)
       \longrightarrow R_{\rm successor}(a';w').        \tag{DR}
\]

But (DR) must specify all witness maps, original-handle consistency equations, public reducibility, challenge order, bounded extraction relations, output projection/uniqueness requirements, and final obligations. Its knowledge guarantee is conditional on access to valid output witnesses. It is not an independently completed proof of the input predicate.

The circular attempted argument is:

1. To extract native fold inputs, invoke Theorem 10.23 with certified successor leaves.
2. To certify each successor leaf, extract its embedded domain/range reduction.
3. To extract that reduction, invoke the same successor/history extractor from step 1.

Without an independently authenticated leaf, a descending well-founded size measure, or a new jointly proved local reduction, this merely restates the original obligation. If one unrolls to the terminal, it becomes the tree in (T)/(R2), not a local bridge.

### 8.3 What a real joint construction must do

A possible research route is to certify old/fresh norms under the original ordered projection before folding, reduce the resulting strong relation into an explicitly bridged weak relation, and give a fixed-shape successor with a bounded number of claims. To establish it, one must show:

- the strong range/norm input and weak reduction share the same commitment-side projection;
- all source/evaluation/packing and nonhomomorphic \(A/B/F\) compression links commute under the actual maps;
- every additional range helper is retired/reduced rather than permanently appended;
- successor claims authenticate every local strong-check leaf with a proved output-oracle contract;
- all relevant inversions/interpolations occur in the correct rings with unit differences;
- the composed extractor has an explicit finite-support total-work bound, and a uniform growing-depth bound only if that stronger target is selected;
- completeness holds for the entire declared bounded domain at the fixed capacity.

These are proof obligations. They are not established by moving Akita's range messages earlier or borrowing a norm proof from another relation.

### 8.4 Concrete expensive escape hatch

The completely explicit local alternative publishes the full witness of (J): old, fresh and successor bounded sources/decommitments, actual frontend witnesses, and local auxiliary data. Verify (J) deterministically on every archived record. Extraction is reading its accepted witnesses; source/evaluation and execution links are already checked. This uses native instances rather than completed per-segment proofs, but reveals witnesses and pays complete source/receiver/frontend checking at every update.

A less revealing variant independently proves (J) with an argument having the exact adaptive witness-extension contract of §7.2. This is a **conditional construction** until such an argument/compiler is fixed and proved. A generic finite-relation proof might relocate rather than save the full proving work; it cannot be described as an efficient native Akita bridge without costs and assumptions.

Full receiver certificates alone are a narrower repair: they authenticate endpoints but still need bounded fresh source certificates, weak/strong source matching, local admissibility and frontend/history arguments. Every required counterfactual child must produce its own accepted certificate for its own exact endpoint. Actual-path-only certificates cannot be substituted.

## 9. Lessons from other native folding systems

### Nova

Nova §§3–4 and Theorem 3 use a relation that is actually closed under folding: committed relaxed R1CS includes witness/error commitments and quadratic cross terms. Its local forking lemma receives folded instance–witness pairs at **every tree leaf**, and its concrete extraction uses three accepting output witnesses. It does not justify using one actual final witness at all counterfactual leaves. Its homomorphic commitment update equations are not the full decomposed/compressed Akita commitment equations.

The indexed Nova IVC definition/proof also explicitly quantifies constant depth \(n\). Its cited induction establishes the guarantee stated there. It is compatible with a fixed finite-support objective; it does not automatically establish the **stronger optional** uniform polynomial-in-growing-\(T\) objective. Its concrete-hash standard-model instantiation is identified as an assumption/heuristic, not a ROM theorem for arbitrary concrete hashes.

### Lova

Lova §1.2.3 warns that \(\|(c-c')^{-1}\|\) can enlarge extracted norms. Its ternary/heavy-row argument seeks differences \(\pm1\), with an initial extraction slack factor; §1.2.5 adds the **integer** Gram relation

\[
 D=S^\top S,\qquad D_{ii}\le\beta^2,
\]

and exact decomposition consistency to remove the slack. This is an example of an actual norm checkpoint and joint reduction, not range certification of unrelated handles. A modular equality \(D\equiv S^\top S\pmod q\) without unique lifting is not this integer guarantee. Lova's unstructured-SIS relation is not Akita's exact original \(A/B/F\) source relation.

### LatticeFold+

LF+ §4.4 explicitly says double commitments are not linearly homomorphic, and gives a sumcheck-based commitment transformation. It also explains why simply flattening range auxiliaries multiplies the witness size by \(dk\) after each fold. Its §5.2 decomposition produces **two** bounded linear-relation instances from a \(B^2\)-bounded instance, with an explicit public recomposition check and extractor. The output witnesses still have to be supplied/authenticated.

These are candidate mechanisms for real relation retirement and fixed-tuple closure. The transformation theorem cannot be imported for an Akita handle without proving the exact commitment/source/evaluation bridge and collision inventory. We do not rely on the image-corrupted display of LF+ relation (17) to infer omitted coordinates.

### SALSAA

SALSAA Setting 3.4 keeps honest \(\beta\) and extracted \(\beta'\) distinct and requires \(4\beta'^2<q\). Lemma 4.8 uses the extracted norm bound to lift a modular trace/norm equality to an integer equality. Theorem 7.3/§7.3 place a norm checkpoint, projection, folding, joining, renewed bottom-row batching and decomposition in one defined RoK chain. Renewed bottom-row batching prevents \(O(T)\) retained rows; the accumulator has \(r_{\rm acc}=2\ell\). Lemma 7.6 proves the chosen commitment-key structure is preserved.

For its constant structural dimensions, the stated core costs are proof size \(O(\lambda\log^2m/\log\lambda)\) bits, prover \(O(m\lambda)\) ring operations and verifier \(O(\lambda^2)\) ring operations. They are SALSAA's costs, **not** an established cost for an Akita bridge. Preserving a commitment **key** is also not by itself preserving the exact original-handle source projection needed in §6.2.

**Combined lesson:** native norm/domain obligations can be reduced rather than fully opened immediately, but only inside an explicitly proved relation chain. None of these references supplies a free independent Akita successor oracle or erases adaptive extraction costs.

## 10. Final acceptance, history, epochs and security model

### 10.1 What an existing final Akita proof certifies

There are two different final-proof arrangements:

**A. Final proof treats the retained accumulator as a new application root.** Subject to its admitted schedule and Theorem 10.33 assumptions, it extracts an algebraic decommitment and true claimed evaluations for that retained root. It does not, without an additional relation/link theorem, certify how that root was produced, the truth of every prior partial prefix, bounded/canonical original sources, zero padding, or consecutive VM execution. A true opening of \(C_T\) alone is not historical authentication.

**B. The prior native reductions are included as the actual earlier stages of one assembled admissible Akita schedule.** Then the theorem can authenticate the included stages and extract their declared root batch relation through its backward tree. This requires every original stage, group topology, transcript order, strong check, setup claim and terminal obligation to match the theorem. It is not achieved by appending a proof whose statement omits those stages. Finite supported-depth old-plus-fresh insertion needs §7.3's topology/BR-identity/admissibility extension; fixed constant \(T_{\max}\) can permit \(N^{O(T_{\max})}\) formal extraction, with concrete reduction costs charged. Only a growing-depth objective requires a uniform polynomial tree bound in \(T\) or a different extractor.

Even arrangement B's extracted relation is the algebraic full-grid relation (202). The stronger bounded/canonical/padding and VM/history predicates must be proved by their own matched relations or a joint theorem. A code-level pause preserves neither claim scope nor knowledge automatically.

### 10.2 Replay archive versus succinct authenticated history

A replay archive stores every exact update record, certificate and original fresh handle/statement. Final verification checks the initial anchor, all \(\mathsf{StepV}\) calls/certificates, exact derived successor identities, all consecutive links, and final discharge. Its public storage and checking work grow with \(T\). It does not require an unproved compressed-history extractor; it does require the local native theorem if local records defer private relation truth.

A fixed-size history digest authenticates an externally provided ordered archive only under an appropriate binding assumption; it does not prove that any update was verified. If such a digest is used, it is a history/FS framing device, not a replacement for the lattice commitment. This task supplies no hash/Merkle/code-commitment substitution.

A succinct receipt must prove exactly those replay checks and link the exact initial/final statements and counts. For computational inner soundness reductions, a sound existential assertion that accepted inner proof bytes exist is not automatically a usable efficient source of those bytes. Use an appropriate receipt knowledge extractor or a proved joint compiler. That extractor must have uniform auxiliary-I/O/expected-time cost bounds, rather than recursively invoking itself without accounting. Memory continuity likewise needs the real VM memory relation, not a newly invented root convention.

BCLMS21 §2.2 supplies the relevant distinction: its PCD compiler is stated in a plain CRS model without oracles; it does not directly carry a ROM-only accumulator into a circuit that evaluates a random oracle. Its concrete instantiation uses a separate random-oracle heuristic. Domain-separating namespaces is necessary hygiene, not a proof of that model change.

### 10.3 Bounded epochs are not unbounded knowledge accumulation

Let \(J\) bound native updates within an epoch. With local tree factor at most \(B(N)\), the epoch tree is at most \(B(N)^J\) times its terminal/other exact factors. For fixed \(J\) this can be polynomial; if \(B(N)=N^c\) and \(J\) grows, the exponent \(cJ\) must still be bounded by one uniform polynomial criterion. The exact admissibility ledger decides, not the word “epoch.”

A deployment with a single fixed total \(T_{\max}\) need not use epochs or claim unbounded renewal at all: it may use §7.3's assembled final-only decision. Epochs are an alternative if that finite tree/security/runtime profile is unacceptable, not a prerequisite inferred from the user request.

Every epoch cut needs an authenticated final endpoint and an exact new initial source relation. Fresh recommitment must be linked to the previous same-handle source rather than silently replacing it. Setup obligations cannot be dropped at a cut. Independent epoch extraction plus replay can have additive total cost over \(\lceil T/J\rceil\) epochs **if** the same adaptive lifting/extraction contract is proved. A bare final-domain checkpoint does not by itself authenticate arbitrary local fork endpoints or preceding deferred claims.

Thus bounded epochs are a priced refresh design, not proof of unrestricted native re-entry. They may pay full endpoint/epoch discharge often enough to lose the intended final-only saving. They also do not reset the security error, oracle-query budget or shared-setup collision exposure.

“Unbounded updates” can mean no algorithmic depth cap while each cryptographic experiment runs polynomially many updates. It cannot mean a fixed field/setup/hash promises the same concrete error bound after infinitely many trials. Lifetime \(T,Q\), repeated comparisons and all retries must be budgeted. Unknown eventual lifetime requires a justified renewal/security policy; no such policy follows from fixed-size `C`.

### 10.4 Classical ROM, QROM and concrete hashing

Akita §10.2's security model expressly provides classical-ROM extraction, not QROM extraction. Quantum attack-cost sizing of Module-SIS parameters is a separate coordinate. A post-quantum noninteractive native chain claim needs a separate QROM theorem for its multi-round, coordinate-wise, certificate-composed adaptive extraction experiment.

With \(Q\) total relevant native FS queries, the fixed-schedule baseline is (FS). Local/certificate/history systems need a **joint** error ledger. If per-invocation bounds hold uniformly under the actual adaptive conditioning, one may charge their sum; if a single global query bound already covers all updates, do not multiply that bound by \(T\) again. Certificates, initial/final arguments, projection matching, frontend composition and receipt binding/errors must all be included. Reusing one setup means setup idealization is charged for the actual shared setup experiment, not independently fabricated at every update.

Concrete BLAKE3/Keccak/etc. are not random oracles by theorem. A concrete implementation needs an explicitly labelled instantiation assumption or a suitable standard-model proof. Existing Fiat–Shamir hashing is not the prohibited commitment replacement, but neither its existence nor collision resistance supplies the missing knowledge-accumulation theorem.

## 11. Stronger growing-depth theorem and conditional proof route

### 11.1 Optional uniform-growing-depth native theorem

**Open stronger theorem: original-handle, bounded-domain, fixed-shape adaptive old-plus-fresh reduction with uniform growing-depth extraction.** This is a research target beyond finite supported \(T_{\max}\), not an additional user requirement. A finite deployment may instead establish §7.3's finite assembled theorem and its concrete parameter/runtime certificates. To support growing depth without the inherited \(N^{O(T)}\) cost, construct a concrete \(\Pi_{\rm native}\) for (A)/(S), using the actual Akita reductions and source maps, with all of the following:

1. **Exact public reducibility and all-valid-source completeness.** The public update fixes the exact successor; any admitted bounded old/fresh witnesses produce a bounded successor with declared failure probability and fixed capacity. All auxiliary claims/rows, setup groups and transcript order are included.
2. **Local knowledge lifting.** In the experiment of §4.3, valid accepting successor witnesses yield bounded old and fresh witnesses, or an enumerated \(A/B/D/F/H\) collision at its declared norm/ring/width. If the successor is supplied by a certificate instead of the malicious process directly, prove that certificate's exact relation and independent extractor, or prove the joint reduction that eliminates its need. No free descendant-certified oracle is assumed.
3. **SAME-handle projection invariance.** Independent successful extractions with the same ordered original commitment projection agree on \(\mathsf{Src}\), including matching any independently extracted source-domain checkpoint. Point-dependent opening paths are excluded. Mixed comparison classes and original frozen security ranks are adequate.
4. **Adaptive witness-extended composition.** Prove the needed path-specific or stronger multi-instance auxiliary-I/O statement for EPT adversaries, fresh oracle invocations, selective abort and arbitrary next-state choices. Preserve the published statements/output while exposing appropriate witnesses.
5. **One uniform quantitative extraction bound.** For total explicit length \(L\), updates \(T\), query budgets \(Q\), adversary work \(T_A\) and maximum local physical size \(N\), exhibit one fixed polynomial

   \[
      \mathbb E[R_{\rm total}]\le
                p(\lambda,T,N,L,Q,T_A),                 \tag{U}
   \]

   including every invoked continuation, endpoint/certificate extractor, conditional sampler and receipt extractor. If the route uses the inherited tree, prove its actual product satisfies (U); otherwise give the different extraction mechanism that avoids (R2).
6. **Application/final composition.** Prove native fresh-claim extraction implies the intended frontend witnesses and exact consecutive execution relation, and that authenticated history plus the chosen final proof implies the claimed run. State the actual ROM/QROM/concrete-hash model and full error ledger.

Items 1–3 are not implied by a fixed-size public commitment; items 2/4/5 are not implied by a source range certificate; item 6 is not implied by a PCS root-opening proof.

### 11.2 Conditional proof route if these premises are instantiated

A sufficient route is:

- prove an exact admitted receiver profile for the bounded relation;
- obtain original-handle source checkpoints and their mixed native/certificate projection matching;
- instantiate either independently authenticated complete local endpoints or an independently extractable exact local bridge (J), **or** prove a genuine joint reduction replacing them;
- prove the adaptive lifting and uniform total-work contract (U);
- replay the complete archive, or instantiate the separate succinct-receipt compiler with the required extraction theorem;
- independently discharge the exact final bounded accumulator relation, with its native terminal/PCS proof and any additional domain predicate actually linked to the same source.

For independent local bridges, extract each record from the original history-generating adversary/emulator, not by using the preceding history extractor as a new local proof oracle. Equation (R4) then bounds total work. Match adjacent sources using (M)/(MM) and the exact \(B/F\) collision inventory. The resulting true native claims and frontend witnesses satisfy (J); consecutive links compose the run by induction. All failure events are charged in the joint security experiment.

This is a concrete **conditional proof architecture**, not an instantiated cheap Akita construction. The explicitly available revealing (J)-replay alternative supplies those witnesses directly and has deterministic additive checking/extraction, but lacks privacy and the desired sublinear per-update certification.

**Precise current obstruction:** the existing Akita one-fold theorem requires counterfactual output witnesses supplied by descendants; its fixed-schedule proof supplies them through a potentially history-multiplicative tree; its extracted inputs are weak algebraic sources. SAME-handle range certification addresses the third issue only after matched extraction. For the direct coefficient/identity-source supplied-list variant, §7.3.2 derives the conditional local backward/range implication. A finite supported-depth route may obtain its branch endpoints through the continuation-replayed assembled tree ending in a complete bounded final decision; exact BR-identity interfaces, full Equation (109)/shape/parameter closure, frontend relation and assembled adaptive-ROM security certificates remain uninstantiated. Eliminating the inherited growing-depth tree cost instead needs independent complete endpoint/local-relation certification or another genuinely proved joint extractor. These are different sufficient proof routes, not an imposed unbounded-update requirement or a claim of fundamental local-algebra impossibility.

## 12. Honest cost ledger

Let \(N_{\rm seg,t}\) be total fresh physical source size, \(N_*\) fixed live receiver size, \(H_t\) full source/path/auxiliary witness length, and \(S_t\) local public transcript bytes. Ring operations and base-field operations must be counted separately for a concrete profile.

| Component | Work/state that must be charged | Proven status for this design |
|---|---|---|
| Native prefix generation | Fresh source/hint access, original \(A/B/F\), response decomposition, Equation (109), range/norm/relation work, all setup handling | Exact costs require selected profile and new old/fresh topology |
| Native prefix checking | Entire retained local transcript, samplers, every public check and derived endpoint | A prover pause is not evidence of this relation's soundness |
| Source-only independent domain certificates | Per-handle certificate prover/verifier/bytes, and extractor work on actual adaptive invocations | Possible logical interface; no cheap Akita instantiation proved here |
| Independently complete receiver certificates | All \(N_*\) coordinates/path checks and evaluation, or real private certificate work and extractor costs at every fork endpoint | Clear witnesses are explicit; private bounded relation/extractor is conditional |
| Clear full local bridge | Read/check old/fresh/successor/frontend/auxiliary witnesses; at least linear in explicit disclosed lengths, plus actual matrix-product/relation work | Concrete deterministic revealing alternative |
| Live native opening | \(O(N_*+\text{receiver path/hints})\), provided fixed-shape closure holds | Shape invariant is a separate open admission certificate |
| Replay archive | \(\sum_t(S_t+\text{certificate bytes}+\text{fresh statement bytes})\); verification is the sum of all local checks | Explicitly grows with \(T\), not a succinct final receipt |
| Succinct receipt | Recursive prover/verifier cost, witness availability, complete receipt relation, and uniform history extractor | Additional compiler/theorem; no free hash digest |
| Final discharge | Complete terminal/PCS proof for retained native relation, plus SAME-handle bounded-domain certificate if not included | Ordinary Akita alone gives only its declared algebraic evaluation relation |
| Suffix/tree extraction | Actual mixed product (T), adversary reruns, filters, FS forking and matrix-collision extraction | Existing fixed-schedule route; generally exponential tree in repeated nontrivial fixed-capacity updates |
| Finite supported-depth final-only route | \(B_{\rm end}(N)b(N)^{T_{\max}}\), paid continuation replays and complete final bounded decision once; security evaluated at full reduction runtime | Derived conditional local lemma for direct coefficient/identity sources; BR/interface/shape/parameter/application/assembled-ROM certificates uninstantiated; formally polynomial for fixed constant \(T_{\max}\), no practical profile claimed |
| Independent local extraction | Sum in (R3)/(R4), including all endpoint oracle work and projection/link checking | Polynomial only under instantiated uniform adaptive guarantees |
| Security | Full query/retry/lifetime, certificate/frontend/history errors, setup idealization and all original/receiver/mixed MSIS views | No concrete deployment security level established |

Clear successor disclosure costs \(\Omega(N_*)\) communication/reading/evaluation work per update. Clear full bridges additionally read fresh sources/frontend witnesses and their original paths. Fixed capacity makes this independent of history depth, not cheap with respect to large source sizes. Private range proofs can have sublinear verification in suitable systems, but their prover/extractor/bridge costs and retained helper sizes cannot be set to zero.

No performance parity or asymptotic improvement is claimed. Comparison must use the **same** source revision, native relation/frontend, segment size, setup offloading, security/query lifetime, final discharge and history/recursion policy as the matching recursive control. The revealing repair may fail that comparison; this is not an impossibility result for a new private native design.

## 13. References and live reading record

The qmd line ranges below are the indexed text actually retrieved during this investigation. Primary links identify the papers; online revision equality is not assumed.

1. **Akita: A High-Performance Lattice-Based Polynomial Commitment Scheme.** [Indexed local paper](SOURCE_PROVENANCE.md#akita); [primary ePrint 2026/1983](https://eprint.iacr.org/2026/1983). Live qmd `lattice-papers/akita.md`: `3191–3205` (analyzed schedule policy versus intrinsic identities); `3210–3239` (admissibility and fixed polynomial tree ceiling); `3310–3374` (assembled protocol/shrinking-schedule discussion; the image-contaminated tail was not used); `3633–3742` (comparison inventory, Definition 10.13 and batch relation); `3743–3786` (Definition 10.14 and source projection); `3810–3989` (Corollary 10.17, tensor/terminal extraction, Definition 10.22, Theorem 10.23); `3991–4250` (strong–weak order, complete error/runtime ledger, Theorems 10.31/10.33 and Remark 10.32). Central claims rely on the fully returned textual theorem passages, not omitted image data.
2. **Neo and SuperNeo: Post-quantum folding with pay-per-bit costs over small fields.** [Indexed local paper](SOURCE_PROVENANCE.md#superneo); [primary ePrint 2026/242](https://eprint.iacr.org/2026/242). Live qmd `lattice-papers/superneo.md`: `855–939` (Definitions 16–18/Theorem 12); `1365–1460` (composition proof, identical projection, fresh oracle randomness and paid EPT oracle work).
3. **Nova: Recursive Zero-Knowledge Arguments from Folding Schemes.** [Indexed local paper](SOURCE_PROVENANCE.md#nova); [primary ePrint 2021/370](https://eprint.iacr.org/2021/370). Live qmd `lattice-papers/nova.md`: `173–186` (explicit constant-depth IVC quantifier); `209–348` (folding definition, fork-leaf output witnesses, relaxed R1CS, cross terms, Theorem 3 and concrete-hash assumption); `997–1058` (constant-depth recursive extraction induction and its invoked extractors).
4. **Lova: Lattice-Based Folding Scheme from Unstructured Lattices.** [Indexed local paper](SOURCE_PROVENANCE.md#lova); [primary ePrint 2024/1964](https://eprint.iacr.org/2024/1964). Live qmd `lattice-papers/lova.md`: `101–128` (inverse norm and ternary/heavy-row warning); `129–238` (decomposition and integer Gram norm relation/argument).
5. **LatticeFold+: Faster, Simpler, Shorter Lattice-Based Folding for Succinct Proof Systems.** [Indexed local paper](SOURCE_PROVENANCE.md#latticefoldplus); [primary ePrint 2025/247](https://eprint.iacr.org/2025/247). Live qmd `lattice-papers/latticefoldplus.md`: `634–705` (nonhomomorphic double commitments, transformation construction and flattening blowup); `1164–1195` (two-instance decomposition and actual output-witness extractor). Relation (17)'s corrupted formula is not relied on for hidden field identities.
6. **SALSAA: Sumcheck-Aided Lattice-based Succinct Arguments and Applications.** [Indexed local paper](SOURCE_PROVENANCE.md#salsaa); [primary ePrint 2025/2124](https://eprint.iacr.org/2025/2124). Live qmd `lattice-papers/salsaa.md`: `247–258` (Setting 3.4); `535–562` (norm checkpoint and integer lifting); `1297–1401` (Theorem 7.3, Corollary 7.5, six reductions, fixed bottom-row batching and Lemma 7.6).
7. **Bünz–Chiesa–Lin–Mishra–Spooner, Proof-Carrying Data without Succinct Arguments (CRYPTO 2021).** [Primary ePrint 2020/1618](https://eprint.iacr.org/2020/1618); [published primary PDF](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_24.pdf). Direct online published-PDF reading, §§1.1, 2.1–2.2, pp. 683–692: atomic versus split accumulation; full predecessor/fresh extraction; multi-instance auxiliary-I/O/expected-time composition; the single-instance IVC exception; plain-model compiler versus ROM/concrete-hash instantiation. The ePrint reader returned only metadata/abstract, so the substantive claims use the published PDF, not an assumed unread full ePrint.

**Bottom line:** SAME-handle range certification is a genuine source-domain/projection repair, not an independent authentication of every challenge-dependent successor. Existing final Akita knowledge certifies its declared assembled relation, not arbitrary earlier partial prefixes. For direct coefficient/identity sources with a fixed supplied list, §7.3.2 gives a conditional local backward/range derivation. A finite supported-depth pre-full-PCS native chain can conditionally use complete bounded revelation **once at the end** and a full continuation-replayed tree, after instantiating the exact BR-interface/Equation (109)/shape/parameter/frontend/assembled-ROM certificates and charging its \(N^{O(T_{\max})}\) reduction costs. Other modes/gadget variants are not automatically covered. A uniform-growing-depth extractor is a stronger optional research target, not a user requirement. Neither route follows merely from fixed-size `C`, a pause point, or a postponed range proof.
