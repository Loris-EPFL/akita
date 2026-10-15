# Akita accumulation: solutions to the six obligations

## 1. Decision and evidence status

**Research only. No implementation, source-code change, new tests, or branch change.** Six dedicated agents researched the six obligations in parallel, using qmd MCP for local papers and primary online sources. The detailed solution documents are linked below. This document resolves their shared interfaces and records which claims are actually justified.

**[INFERENCE] Recommended research path:** first establish a completed-proof **public linear-verifier accumulator** with exact history certification. Develop **private native re-entry** separately. The latter needs a new profile/topology, a bounded-source admission interface, and an independently justified local/adaptive extraction theorem. Do not present the public baseline as evidence that the native theorem is already proved.

There is also a concrete **transparent refreshing** alternative: reveal and fully check bounded receiver/source certificates at every step. It supplies genuine local witnesses and removes hidden-domain ambiguity, but performs full fixed-state work and discloses witnesses. It is not the intended cheap, final-only private accumulator.

Classification used throughout:

- **Derived:** algebra or deterministic relation equivalence established under explicitly stated mathematical conditions.
- **Conditional:** a construction whose exact missing profile/compiler/security hypotheses are listed.
- **Open:** no instantiated proof or parameter certificate was obtained. A proposed lemma is not an established theorem.

## 2. Per-obligation solution documents

| Obligation | Concrete repair | Status and limitation | Detailed document |
|---|---|---|---|
| Fixed-size closure | Freeze a receiving profile; count the entire successor; certify its capacity fixed point, source envelopes, compression/setup tails and padding. | Shape admission can be specified and checked. Geometry alone is not an MSIS/security certificate. | [01: fixed-size closure](AKITA_ACCUMULATION_01_FIXED_SIZE_CLOSURE.md) |
| Strong completeness | Identity-source, injective full-grid receiver relation; same-commitment bounded decommitment certificate; universal deterministic response bounds. | Exact reveal-decider equivalence is derived. Native update completeness is conditional on capacity, every original input's source domain, and bounded sampler admission. | [02: strong completeness](AKITA_ACCUMULATION_02_STRONG_COMPLETENESS.md) |
| Original commitment consistency | Preserve original A/B/F handles; one shared D/H prechallenge path; all native relation/recomposition rows; typed pending output. | Existing finite-fold backward link is source-backed under descendant certification. New recurrent topology/extraction remains conditional. | [03: commitment consistency](AKITA_ACCUMULATION_03_COMMITMENT_CONSISTENCY.md) |
| Polynomial-time extraction | Independent bounded local bridge extraction; replay or an authenticated receipt; optionally independently certified receiver endpoints at every fork child. | A polynomial runtime follows from the stated local multi-instance emulation contract. That native contract has not been instantiated. Public residual arithmetic needs no private-opening extractor. | [04: extraction and security](AKITA_ACCUMULATION_04_POLYNOMIAL_EXTRACTION.md) |
| Execution composition | Complete legal-cut boundary; authenticated RAM access/update relation; exact ordered links, initialization, counters, advice/IO and successful halt. | Semantic concatenation and authenticated-memory induction are derived. A new sound segmented Jolt PIOP is required. | [05: execution composition](AKITA_ACCUMULATION_05_EXECUTION_COMPOSITION.md) |
| History and final acceptance | Prove or replay every frontend/update/link; decide every pending claim; bind approved program/setup/IO; require accepting output and a nonempty run. | Deterministic acceptance algorithms are specified. Compact lattice receipt security requires the applicable joint knowledge/composition theorem. | [06: history and final acceptance](AKITA_ACCUMULATION_06_HISTORY_AND_FINAL_ACCEPTANCE.md) |

The documents include preferred constructions, alternatives, equations, challenge ordering, failure examples, costs, source sections/qmd addresses, and remaining proof obligations. They do not claim a newly implemented or audited accumulation scheme.

## 3. One common contract

### 3.1 Frozen profile

A profile $P$ binds:

- native field $F$, evaluation field $E$, public accumulation field $K$ if used, rings and bases;
- original and receiver commitment geometry, physical coefficient orders and source maps;
- exact A/B/D/F/H matrix views, compression, packing and quotient policies;
- all digit alphabets/depths, source/response envelopes, required zero cells and fixed $N_*$;
- maximum fresh groups, segment size, admitted lifetime/query budget and setup registry;
- exact transcript/sampler version, framing and attempt accounting;
- program/ISA, legal cuts, complete boundary schema, RAM/advice/IO semantics and accept policy.

An original handle keeps its original geometry. Changing the receiver cannot change a previously committed polynomial's block split, ring, rank, compression or decoding map. [Akita §12.2](SOURCE_PROVENANCE.md#akita), qmd `line:4641-4662`, explicitly includes all successor and setup obligations before scoring a schedule.

### 3.2 Counts and index conversion

Use $t$ for the number of completed segments. Initial state has $t=0$; the next segment's zero-based index is $j=t$; successor count is $t+1$. The execution document uses zero-based segment indices. The receipt document uses one-based record ordinal $i$ in its loop. The exact conversion is

$$
i=j+1=t+1.
$$

Choose one canonical serialized index convention in $P$ and derive the other, with integer range/overflow checks. Never compare or hash these conventions as if they were the same integer.

### 3.3 Application boundary

Carry a complete boundary $B_t$, or a digest whose entire preimage relation is proved at both endpoints. It includes next PC, all live registers/control state, RAM authentication, active instruction/row/memory counters, advice and input identities/cursors, output/journal length and state, and running/halt status. Cuts occur only at completed instruction/virtual-sequence boundaries under the conservative profile.

Each segment proves the actual first and last **active** states. Padding is inert. The receipt checks initialization once, all exact adjacent links, and final successful halt/output. A per-proof conjunction, matching PC alone, or matching an unproved state digest does not establish continuation.

## 4. Solution A: exact public verifier-residual accumulation

### 4.1 Exact split, not a success flag

For an admitted completed proof $u$, derive

$$
V_{\mathrm{full}}(u)
=V_0(u)\land\bigwedge_{\ell\in\mathcal L(u)}
[A_\ell x_\ell(u)=y_\ell(u)].
$$

For the narrow Akita terminal split, use one terminal response per proof $j$:

$$
x_j=z_j,\qquad y_j=\sum_{i<B_{\mathrm{term},j}}c_{j,i}t_{j,i}.
$$

Block index $i$ is different from proof index $j$. Keep Equations (164) and (165), predecessor-authenticated terminal images, original challenge/tensor checks, every norm/encoding bound, preceding folds, setup discharges and complete stream consumption. Equation (163) alone is deferred. [Akita §8.2](SOURCE_PROVENANCE.md#akita), qmd `line:3026-3097`, supports this split at the paper level; exact code-path equivalence still needs a selected source delivery.

The reducer returns **retained checks plus exact pending equations**, not a Boolean saying the whole PCS verified. Every lane identifies its exact physical matrix/view. Missing mandatory lanes reject.

### 4.2 Update, decision and algebraic proof

Initialize public vectors $U_\ell=Y_\ell=0$. Bind the complete old state and input before fresh $\alpha\in K^\times$. Verify

$$
U'_\ell=U_\ell+\alpha x_\ell,
\qquad Y'_\ell=Y_\ell+\alpha y_\ell.
$$

The decider checks $A_\ell U_\ell=Y_\ell$ for every lane. With $r=A U-Y$ and $e=A x-y$,

$$
r'=r+\alpha e.
$$

For fixed nonzero old/input residual data, at most one nonzero field scalar makes the entire residual tuple zero. Thus the interactive one-step error is at most $1/(|K|-1)$; valid old/input residuals stay zero for every challenge. This is an ordinary field-module argument even if a polynomial quotient ring has zero divisors.

**No short SIS-opening claim is made about $U$.** It is public arithmetic. Original proof norms are still checked individually. Extending $K$ is a new public arithmetic choice, not a change to original lattice commitments or the deployed `ExtField=Field` adapter.

### 4.3 Complete receipt

The preferred final receipt proves all retained checks, the derived residual updates, application links and one final D. It binds the approved guest image/preprocessing/setup, exact public statement, count and accepting output. Native replay is the direct alternative and retains linear history communication/verifier work.

The complete classical security statement is conditional on exact splitting, joint original/accumulator oracle accounting, binding framing/hashes, and history/segment knowledge composition. For adversary-only accumulation queries $Q_A$, replayed output records can require up to $T$ further fresh verifier queries; a conservative fresh-query term is

$$
\frac{Q_A+T}{|K|-1}
$$

before sampler/digest/original-proof/receipt terms. If $Q_A$ already includes all verifier/output queries, do not add them again. This is not a QROM theorem. See [04](AKITA_ACCUMULATION_04_POLYNOMIAL_EXTRACTION.md) for the exact scope.

This route avoids the native whole-history source extractor but does not extract CPU witnesses by itself. It also does not avoid generating the original completed PCS proofs. Savings are limited to the deferred equations and must exceed the added update/receipt work.

## 5. Solution B: native bounded fixed-shape re-entry

### 5.1 Exact bounded relation and final certificate

For a fixed coefficient-packed receiver with identity source map, define

$$
\begin{aligned}
R_{\mathrm{acc}}^{\mathrm{bd}}(P,a;w,d)\iff{}&
\bigl(|w|=N_*\ \land\ w\in\mathcal D_{b_*}^{N_*}\bigr)\\
&\land\mathsf{RequiredZeros}_P(w,d)\\
&\land\bigl(s(d)=\mathsf{Pack}_P(w)\bigr)\\
&\land\mathsf{Rel}_{\mathrm{com}}^{\mathrm{Akita}}(P,C;w,d)\\
&\land\bigl(\widetilde w(r)=v\bigr).
\end{aligned}
$$

Here $d$ contains the complete original receiver A/B/F image/compression chain. Packing is injective on **all physical cells**; extra cells must be included or checked zero. Supplied noncanonical image digits are permitted when the exact original equations and alphabets hold. Do not canonicalize them and thereby replace the old commitment.

A final clear certificate $(w,d)$ can deterministically check this entire relation. Its size/work are fixed in $N_*$ and independent of history length, but it is not succinct in $N_*$ and is not zero knowledge.

Fresh source groups require their own bounded relation at their **original handles**. Small decoded values do not imply small gadget source planes. The unrestricted weak Akita relation is a larger relation; strengthening admission is a genuine new interface, not a consequence of existing proof acceptance.

### 5.2 Universal response admission

For each response summand, certify $\|s_a\|_\infty\le B_a$, $\|s_a\|_2^2\le E_a$, and accepted injected-challenge bounds $\|c_a^A\|_1\le K_{1,a}$ and $\|\mathsf{Mul}(c_a^A)\|_{2\to2}\le\Gamma_a$. Then

$$
\|z\|_\infty\le\sum_a K_{1,a}B_a,
\qquad
\|z\|_2^2\le\left(\sum_a\Gamma_a\sqrt{E_a}\right)^2.
$$

Choose response digit depths from the actual **positive and negative encoding capacities**, not just a nominal security bound. Balanced binary digits $\{-1,0\}$ cannot encode unreduced $+1$ at any depth. Use radix at least four for symmetric integer responses or explicitly provision a full-field negative-lift representation.

Every auxiliary field value needs a universal decomposition rule or its own bound. Norm/Gram relations use exact integer lifting or correctly bounded limbs; a field congruence is not an integer energy certificate. These bounds solve uniform admission only if the entire receiver capacity/security profile closes.

### 5.3 Original-handle native protocol

The ordered protocol is:

1. Bind old state, original fresh handles/claims, application statement and frozen receiver profile.
2. Perform the chosen incoming-point/tensor reduction.
3. Bind one shared D/H partial-opening payload.
4. Sample the admitted claim-combination and joint sparse folding challenges/nonces.
5. Build **all** Equation (109) successor components and commit them before ring/row/range challenges.
6. Verify original A/B/F consistency, D/H consistency, digit/binary/range and all exact reduced relation/sumcheck rows.
7. Derive a typed pending ordinary receiver opening and domain tag; preserve any registered setup obligation explicitly.

Current Akita application groups are root-only. Old-plus-fresh re-entry is a new topology; absolute-level opening policies cannot silently reset. Direct setup processing is the conservative initial research profile. Offloading requires its real consume/produce ledger and final flush.

### 5.4 A checkable capacity certificate and a rejected security example

For a fixed-depth slab, write $c_*=M_*d_*$, $a_*=m_{A,*}d_*$ and $\gamma_*=P_*\delta_{\mathrm{open},*}(d_{\mathrm{op},*}+n_{A,*}d_*)$. Certify an upper bound $C_D$ on all fresh components, quotients, compression and alignment throughout that slab. The sufficient admission condition is

$$
\gamma_*<c_*,
\qquad
\left\lceil\frac{a_*D+C_D}{c_*-\gamma_*}\right\rceil
\le B\le
\left\lfloor\frac{T_f(D)}{P_*K_{1,*}(b_*/2)}\right\rfloor,
\qquad N_*=c_*B.
$$

Every selected matrix width/rank, physical-grid constraint and security/setup cap must also be valid. Subdivide slabs where tails or ranks change. This is a finite integer admission calculation, not a promise that every large receiver works.

[01](AKITA_ACCUMULATION_01_FIXED_SIZE_CLOSURE.md) gives a fully counted shape-only example: capacity $2^{20}$, successor $951616$ cells and $96960$ committed padding cells, including both F/H stages. **It is deliberately rejected as a security profile:** dimension-64 challenges with coefficient $\ell_1\le16$ have fewer than $2^{69}$ possible values, so cannot support a 128-bit special-soundness ledger before query loss. The example proves usable accounting, not a secure zkVM accumulator.

If admitted fresh shapes have different zero/padding spans, derive a bounded layout tag from the verified input or reserve genuinely fixed spans. The final domain checker must know which cells must be zero. This tag does not change the receiving commitment geometry or capacity.

## 6. How to avoid exponential extraction without hiding the cost

### 6.1 Why a final witness alone is insufficient

Akita's current finite-schedule bound is polynomial in

$$
\operatorname{Tree}(\mathrm{sch})=\prod_r b_r.
$$

Repeating a nontrivial factor $b>1$ for $T$ updates can give $b^T$. An actual final witness authenticates the published endpoint, not every counterfactual successor in a rewind. Neither source uniqueness nor a norm check turns this into polynomial total extraction automatically.

### 6.2 Independent local bridge

Define an exact local relation $J_P$ containing bounded predecessor and fresh original openings, true frontend/application checks, the complete update transcript/witness, and the bounded successor relation. A local proof must have auxiliary-I/O-preserving adaptive multi-instance witness-extended emulation, with a uniform polynomial **total** cost and no calls to a descendant accumulator extractor.

If that contract is instantiated, the target recurrence is additive:

$$
\mathcal T_{\mathrm{extract}}(T)
\le p(\lambda,T,N,Q)+T\,p_{\mathrm{local}}(\lambda,N,Q),
$$

rather than $b\,\mathcal T_{\mathrm{extract}}(T-1)$. The exact shared projection and uniqueness conditions from SuperNeo must hold. The native bridge theorem, its mixed-view collision ledger and the applicable receipt compiler are **open**, not supplied by this recurrence. See [04](AKITA_ACCUMULATION_04_POLYNOMIAL_EXTRACTION.md).

### 6.3 Independently certified receiver checkpoints

At every update, alternatively supply

$$
\beta_t:\quad\exists(w_t,d_t)\ R_{\mathrm{acc}}^{\mathrm{bd}}(P,a_t;w_t,d_t).
$$

The certificate covers both bounded source/domain **and** the actual receiver evaluation. Bounds-only certification is insufficient. Every accepted fork child must carry a certificate for its own derived $C',r',v'$; those commitments can differ across fold children.

A fully specified transparent realization is $\beta_t=(w_t,d_t)$ checked by the exact reveal decider. It supplies witnesses by reading them; it has no hidden witness oracle. Use an `AcceptCertifiedOrdinary` mathematical interface, distinct from `AcceptPending`. It pays full fixed-receiver matrix/chain/MLE work and communication per update and leaks the witness.

**Notation alignment:** the extraction document uses $\gamma_t$ for the receiver checkpoint and $\beta_t$ for the stronger complete local bridge. Other issue documents and this section use $\beta_t$ for the receiver checkpoint. These are different predicates; serialized certificates must carry an explicit kind, and a receiver-only certificate cannot stand in for a complete bridge.

This repairs descendant endpoint access, not bounded fresh-source extraction, projection matching or the new topology theorem. Clear source/bridge certificates can check the full strengthened local relation directly; a private compact replacement needs an actually instantiated independently extractable argument for that exact relation. An unchecked ordinary proof called a “checkpoint” is not a solution.

## 7. Execution and final acceptance

### 7.1 Memory choices

The concrete conservative memory solution in [05](AKITA_ACCUMULATION_05_EXECUTION_COMPOSITION.md) authenticates every RAM read/write using a fixed-depth Merkle tree. Its hash computation and CPU membership/update constraints are proved inside a new lattice/Akita-backed PIOP. This adds hash collision resistance and substantial hash-circuit work, but introduces no group assumption and does not change the PCS into a hash PCS.

For a finite, practical RAM capacity $M$, the alternative uses Akita commitments to $V^-,V^+$ and generalized one-hot prefix memory equations. It preserves the exact original outgoing handle as the next incoming handle. It can cost $O(M)$ per segment and needs new endpoint/range/opening reductions. Sparse values do not justify an unpriced $2^{64}$-entry setup. Keep this alternative explicit rather than claiming the existing Jolt memory argument already supports arbitrary continuation.

### 7.2 One final lattice wrapper

A new history guest verifies the agreed `Step` relation for every record and either checks Final/D internally or binds the exact final state for an external decider. The consumer fully verifies the outer proof, binds approved program/setup/preprocessing and exact IO, and requires a canonical output with `accept=1`, successful halt and $T\ge1$.

The inspected Blake guest checks independent inner proofs; $T=0$ is vacuously accepting, and outer execution proving alone does not require an accepting guest result. It also has per-record decoding checks without the proposed global stream-completion check. These source semantics are not proof of an exploit in a consumer already enforcing the needed policy. [06](AKITA_ACCUMULATION_06_HISTORY_AND_FINAL_ACCEPTANCE.md) gives the actual rejection algorithms and source links.

A final wrapper does not make invalid native updates sound. It proves that the selected update program executed. Its prover trace, history transport and storage grow with $T$. A small outer proof is not small total communication if all history records are public input. Cheap online receipts retain recursion and need their own compiler/final receipt-opening discharge.

## 8. Security and privacy ledger

For each original segment, native update, static certificate and receipt, retain the exact supported profile, complete challenge/attempt ledger, setup statistical distance, physical matrix-view inventory and collision radii. Shared setup prefixes are not independent matrices. Mixed original/receiver comparisons must be priced before original commitment formation.

[Akita Corollary 10.34](SOURCE_PROVENANCE.md#akita), qmd `line:4141-4250`, gives the classical ceiling

$$
\varepsilon_{\mathrm{int}}\ge c/|E|
\quad\Longrightarrow\quad
\lambda_{\mathrm{FS}}^{\mathrm{eff}}
\le\log_2|E|-\log_2c-\log_2(Q_{\max}+1).
$$

Increasing only $K$ does not improve original fp128/frontend errors. Quantum lattice attack-cost estimates do not supply QROM knowledge/receipt security. Quantum search and multi-round FS require their own applicable theorem and query/depth budget. The detailed analysis in [04](AKITA_ACCUMULATION_04_POLYNOMIAL_EXTRACTION.md) does not instantiate a QROM theorem for deployed Akita.

Final or per-step witness revelation is nonhiding. Hiding an accumulator alone cannot erase published boundaries, frontend proofs, history bytes, attempt metadata or final certificates. A private receipt needs a complete leakage/simulation contract and an actual zero-knowledge compiler; current clear-only Akita supplies neither automatically.

Primary composition references: [BCMS, TCC 2020](https://link.springer.com/content/pdf/10.1007/978-3-030-64378-2_1.pdf), [BCLMS, CRYPTO 2021](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_24.pdf), and [Halo Infinite, CRYPTO 2021](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_23.pdf). Their exact inspected sections and model limits are recorded in the issue documents. In particular, oracle arithmetization/recursive non-black-box composition is not obtained merely by citing a ROM PCS theorem.

## 9. Research conclusion

The six obligations have concrete solution interfaces and conservative alternatives. They do **not** jointly establish an efficient private native accumulator yet.

The most direct conditional construction is exact public residual accumulation plus authenticated execution/history. The strongest private target is fixed-shape native re-entry plus a same-handle bounded-source bridge and a polynomial adaptive extractor. Transparent certificates provide a useful fully checked relation baseline, but cannot be relabeled as cheap final-only accumulation or zero knowledge.

The key remaining native artifacts are a **secure admitted profile certificate** and an **instantiated local bounded-bridge/adaptive extraction theorem**. Segment PIOP and receipt composition must also implement the stated complete relations. No performance gain, end-to-end PQ theorem, or implementation is claimed.

## 10. Integration verification

Performed bounded in-memory algebra calculations and document/source checks:

- Exhausted 2,400 nontrivial residual-vector pairs over the field of seven elements; each had at most one bad nonzero accumulation scalar.
- Recomputed the closure example's complete successor allocation, final padding and fixed-depth slab bounds.
- Recomputed the dimension-64, radius-16 integer challenge-ball cardinality and the base-16 positive digit capacities.
- Checked all eight research Markdown files for control-character corruption, local-link existence, explicit reference resolution, math delimiter/brace/environment balance and malformed aligned-row endings.
- Checked that all 21 distinct commit-pinned Jolt file targets cited by the documents exist in the fetched Git object store.

The document checks are structural, not a visual LaTeX rendering check. No implementation, build, test suite, benchmark, security estimator, native proof execution, extractor run or branch change was performed. Algebra checks do not establish a new security/composition theorem.
