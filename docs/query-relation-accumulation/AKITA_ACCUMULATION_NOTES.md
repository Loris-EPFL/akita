# Akita accumulation for zkVM segment aggregation

## Research entry: feasibility, interfaces, and candidate schemes

**Status:** research design, not an implemented or audited accumulation scheme. Statements marked **[INFERENCE]** are proposed constructions or deductions, not theorems established by the cited papers. The elementary public-residual lemma below is proved algebraically under its stated interactive assumptions; it does not prove a complete Fiat–Shamir, recursive, or quantum-secure zkVM system.

**Goal:** replace repeated full-verifier recursive aggregation of zkVM segment proofs with accumulation of Akita verification obligations, followed by one final discharge. Keep the original segment semantics, public-input binding, and cross-segment memory/state consistency.

**Decision:** pursue two distinct constructions, not an unproved claim that native batching already implements accumulation:

1. **Public verifier-residual accumulation:** a conservative, completed-proof-only baseline. Accumulate selected public linear equations from the Akita verifier and check them once. This does not require commitment homomorphism or a new lattice witness-norm invariant. It retains every other verifier check and can have a large accumulator.
2. **Native fixed-shape re-entry:** the more ambitious witness-based design. Reduce an old accumulated opening plus a bounded fresh segment opening batch to one new ordinary Akita opening. This could defer substantially more PCS work, but needs new capacity, completeness, and adaptive extraction theorems.

**[INFERENCE] Feasibility:** there is no algebraic obstruction to the first construction. The second is a credible research target, not a consequence of Akita's current finite-schedule PCS theorem. A single compact final receipt still has to certify all segment-prefix checks, update checks, and composition checks. Accumulation can replace recursive verification of the expensive PCS part; it cannot replace authentication of the entire history with an unverified digest.

## 1. Actual repository baseline

The local papers were read through qmd MCP. Implementation observations below come from fetched, commit-pinned Jolt branches, not from the historical Jolt paper.

`git fetch --all` completed in `jolt/`. It updated `origin/amir/akita-batch` and `origin/refactor/transcript-unification`. The checkout stayed on clean `main`; no source files or branch checkouts were changed.

| Jolt ref | Snapshot | Observed scope |
|---|---|---|
| `origin/main` | `629ed77b5c999fb2f52e5781b2b98a04ed6cf5ed` | Akita is already integrated. Trace columns are selector-packed into a physical `OneHotTrace` polynomial. |
| `origin/amir/akita-batch` | `0791f79a55c3fadc3d327c78a72228696c338468` | Actual trace columns form one streamed native batch; the trace-selector reduction is removed. This is the relevant newer integration seam. |
| `origin/wrap/fast-recursion-blake` | `5c65167e269fdfc2917ecb82f420d349b18b4c87` | Generic full-verifier guest; separate modular q128 Akita outer-proving/replay route exists in source. |
| `origin/feat/fast-recursion` | `141460a7ffac436f4c533c684d31acaf8aaa8ace` | Akita verifier-guest tracing; the Akita outer-prove mode is explicitly unavailable. |
| `origin/refactor/transcript-unification` | `31103b01c5a70261dd92b192dbb9d138f9dda083` | Fetched. No protocol conclusion in this note depends on an uninspected change in this branch. |

The main branch pins upstream Akita `e2c49ed450f1999a743e7fd4a648f230ce45472f`; the native-batch branch pins `83574331d4e51f8cce1d8689d05592fa4ef4c138`. The Blake recursion branch pins the public Akita fork `d0ee56a0b12db66b4c46a87c527169737694c4c7`. These are different source snapshots, not one already-validated combined delivery. See [main manifest][jolt-main-manifest], [native-batch manifest][jolt-batch-manifest], and [Blake manifest][jolt-blake-manifest].

### 1.1 Field, privacy, and batching

The current Akita path uses `Prime128OffsetA7F7`:

$$
q=2^{128}-2^{32}+22537.
$$

It is lattice-based, not a historical hash-based PCS. Jolt still has a separate Dory option. The Akita path inspected here is **clear-only**: `akita` and `zk` are mutually exclusive. Existing BlindFold support is not an Akita zero-knowledge layer. The deployed streamed source describes `ExtField = Field`; extension-field arithmetic proposed later in this note is not claimed to be an existing Jolt accumulator configuration. See [Akita integration documentation][jolt-akita-doc] and [native-batch source contract][jolt-batch-source].

On the native-batch branch, the trace column order is instruction RA, balanced increment digits, signed carry, bytecode RA, RAM RA. Stage 8 checks a common point after mapping address/cycle coordinates to the native cycle/address order. Advice and committed-program objects retain independent commitments and group-local points. The grouped-opening role order, initial commitment-absorption order, and trace column order are separate contracts; an accumulator must preserve each, not substitute one ordering for another.

`AkitaChunkProfile::{Single,Two,Four,Eight}` partitions responses within a PCS proof. **These chunks are not zkVM execution segments.** The streamed batch also requires columns sharing one trace owner; it is not an API for inserting independently owned segment traces. Later-fold owner alignment is explicitly outside the pinned native-batch dependency. See [column ownership][jolt-batch-owner] and [native batching specification][jolt-batch-spec].

### 1.2 Concrete insertion point

The current driver validates the statement and transcript, verifies stages 1–7, then assembles the Stage 8 opening statement and immediately calls `PCS::verify_batch`.

On `origin/amir/akita-batch`:

- `crates/jolt-verifier/src/stages/stage8/akita.rs::verify` calls `PCS::verify_batch` at line 360.
- `one_hot_trace_claim` constructs the canonical commitment, native point, and ordered evaluation vector.
- `AkitaNativeBatching::verify_trace_batch` resolves the trusted row, checks descriptors and shapes, binds the groups/session, then calls the complete backend `batched_verify`.
- The prover already has `OpeningHint`/retained commitment handles and constructs `SelectedProverOpeningData::from_committed_claims` before calling backend `batched_prove`.

See [Stage 8 assembly][jolt-batch-stage8], [native statement boundary][jolt-batch-adapter], and [prover handles and prepared opening][jolt-batch-prover-data].

**[INFERENCE] Proposed seam:** preserve all canonicalization, metadata, profile/catalog validation, auxiliary reductions, and session binding, but export a typed *pending opening obligation* instead of immediately discharging the full PCS argument. The current `Stage8Output::Clear` is payload-free and does not implement this contract. Returning a pending obligation must not mean that the segment is already fully verified.

## 2. What must be accumulated

Let $X_i$ be the public statement of segment $i$, and $\pi_i$ its proof. Define a predicate-efficient verifier split:

$$
(b_i,Q_i)=V_{\mathrm{front}}(X_i,\pi_i),
\qquad
V_{\mathrm{seg}}(X_i,\pi_i)
=b_i\land\bigwedge_{u\in Q_i}\Phi_{\mathrm{PCS}}(u).
$$

$V_{\mathrm{front}}$ must replay all original PIOP/Fiat–Shamir checks needed to derive $Q_i$. It cannot trust claimed points or values supplied by the aggregator. A member of $Q_i$ includes the original commitment descriptor, ordered evaluations, point, role, admitted schedule/profile, setup identity, and statement-bound Akita session—not merely an untyped triple $(C,r,v)$.

The predicate-efficient compiler is described in [BCMS, §2.3][bcms-published]. It supports accumulating the PCS obligations rather than folding each CPU instruction. It does **not** remove the need to authenticate the front-end checks.

### 2.1 Segment composition remains a separate relation

A suitable target statement is

$$
\operatorname{Compose}(X_1,\ldots,X_T)
=\operatorname{Init}(X_1)
\land\bigwedge_{i=1}^{T-1}\operatorname{Link}(X_i,X_{i+1})
\land\operatorname{GlobalMemory}(X_1,\ldots,X_T)
\land\operatorname{Final}(X_T).
$$

`Link` must cover the actual machine boundary: PC, registers, memory commitment or authenticated memory interface, timestamp/counter state, instruction/program identity, and advice/public-I/O state as required by the VM. Equality of a PC alone does not establish memory continuity. A state digest is useful only if each segment proves that its boundary witness opens that digest and the digest scheme is binding.

The statement must also enforce segment order/count, verifier-key and program identity, initial input, final output/journal, and the intended halt/accept condition. A nonempty execution requirement must be explicit. A committed list or history hash does not prove these checks were performed.

The inspected recursion guests verify independent proof/device pairs and return their conjunction. They do not implement this segment-link relation. The need for memory/register matching is also explicit in [Proving CPU Executions in Small Space, §1.1.1][cpu]. See [Blake recursion guest][jolt-blake-guest].

### 2.2 Which receipt is required?

| Receipt architecture | What it establishes | Remaining cost |
|---|---|---|
| Native replay | Final verifier checks every front-end/update/composition record and one final decider. | Per-segment transcript communication and verifier work remain. |
| One final nonrecursive wrapper | Proves all front-end/update/composition checks, and either proves or exposes one final decider check. | Wrapper prover work grows with the number of records; proving-space claims need separate analysis. |
| Cheap recursive receipt | Each step proves the front-end/composition/update logic, while its own PCS obligations are also accumulated. | Some recursion remains, but not repeated full PCS verification. Requires a sound recursive compiler. |

**[INFERENCE] Preferred target:** keep the same externally useful single receipt, but replace the full PCS portion of the wrapped verifier with an accumulation update. A one-final-wrapper design can remove repeated recursive full-verifier aggregation without asserting that history certification disappeared. A cheap recursive receipt is another option if online receipts are required.

Neither the final accumulator nor its decider alone proves that every prior update was checked. This distinction is central in [BCMS, §§2.1–2.3][bcms-published] and [Halo Infinite, §6][halo-published].

## 3. Formal accumulation contract

For an atomic predicate $\Phi$, use indexed keys and interfaces

$$
\begin{aligned}
\mathsf{Gen}(1^\lambda)&\to\mathsf{pp},\\
\mathsf{Index}(\mathsf{pp},\mathrm{descriptor})&\to(\mathsf{apk},\mathsf{avk},\mathsf{dk}),\\
\mathsf{AccP}(\mathsf{apk},a,u)&\to(a',\pi_{\mathrm{acc}}),\\
\mathsf{AccV}(\mathsf{avk},a,u,a',\pi_{\mathrm{acc}})&\to\{0,1\},\\
\mathsf{Dec}(\mathsf{dk},a)&\to\{0,1\}.
\end{aligned}
$$

The descriptor fixes the setup/profile family, input capacity, field, serialization, transcript domains, and intended application relation. These are setup inputs, not proof-chosen security parameters.

### 3.1 Strong completeness

For every valid input and decider-valid old accumulator,

$$
\Phi(u)=1\land\mathsf{Dec}(a)=1
\Longrightarrow
\Pr[\mathsf{AccV}(a,u,a',\pi_{\mathrm{acc}})=1
\land\mathsf{Dec}(a')=1]=1
$$

in the perfect formulation. A construction allowing abort must specify its alternative completeness guarantee and total failure budget. Completeness only on an honestly generated history is weaker than the all-valid-input condition needed by mutually distrustful parties. See [BCMS, §2.2][bcms-published] and [Halo Infinite, Definition 6.1][halo-published].

### 3.2 Backward soundness

An efficient adversary must have negligible probability of

$$
\mathsf{AccV}(a,u,a',\pi_{\mathrm{acc}})=1
\land\mathsf{Dec}(a')=1
\land\bigl(\Phi(u)=0\lor\mathsf{Dec}(a)=0\bigr).
$$

This is stronger than saying that an honest accumulator usually passes. Sequential soundness then relies on acceptance/authentication of every transition.

### 3.3 Split/knowledge accumulation

Witness-based accumulation maintains $a=(a^x,a^w)$ and inputs $u=(u^x,u^w)$. The update verifier sees only the public projections. Given an accepted transition and a valid output witness, an extractor must recover valid old/input witnesses for the declared relations.

Atomic accumulation of actual verifier bits does not by itself extract CPU witnesses. The final zkVM proof composition needs the original proof system's appropriate soundness/knowledge guarantees. Split accumulation and PCD require stronger adaptive composition conditions, including multi-instance witness-extended emulation for the intended topology. Path IVC and general DAG/PCD are not interchangeable. See [BCLMS, §§2.1–2.2][bclms-published] and [SuperNeo, §6, Definitions 16–18 and Theorem 12][superneo].

### 3.4 Efficiency is relative to the right variables

Let $T$ be the segment count, $N$ a maximum segment/PCS size, and $g$ the admitted fresh group count. An accumulator can be independent of $T$ while still depending on $N$, $g$, or the setup width. State that dependence explicitly. A cheap update verifier, a short receipt, and a cheap final decider are three separate targets.

## 4. Candidate A: public linear-verifier residual accumulation

**[INFERENCE] Proposed completed-proof-only construction.** This is the conservative algebraic baseline, not a full native PCS-witness accumulator.

### 4.1 Exact predicate split

For a fixed admitted setup/profile family, split the complete verifier as

$$
\Phi_{\mathrm{Akita}}(u)
=V_0(u)\land\bigwedge_{\ell\in\mathcal L}
\bigl(A_\ell x_\ell(u)=y_\ell(u)\bigr).
$$

$V_0$ performs every check except the explicitly listed linear equations. It derives $x_\ell,y_\ell$ from authenticated proof bytes and original challenges. It checks shapes, expected lane presence, descriptors, source statement, schedule identity, and all nondeferred equations. A missing mandatory lane is a rejection, not an implicitly zero contribution.

For Akita's terminal check, [§8.2, Equation (163)][aki] supplies

$$
A_{\mathrm{term}}z_j=b_j,
\qquad
b_j=\sum_i c_{j,i}t_{j,i}.
$$

Here $j$ indexes completed proofs and $i$ indexes terminal source blocks: each terminal proof has one clear response $z_j$. Terminal partials and predecessor-bound $t_{j,i}$ are available from the completed proof. Keep the predecessor authentication, original tensor reduction, challenge sampling/nonce rules, response norm and canonical encoding, Equation (164), Equation (165), all preceding folds, registered setup openings, and final proof-stream completion checks. Do not omit the outer relation while retaining an ordinary outer commitment: the terminal's omission of B relies on its specially authenticated inner state.

For heterogeneous shapes use a **fixed finite lane catalogue**, one lane per exact matrix/view/geometry, or an exact coefficient-level representation over a common fixed setup vector. Different ring dimensions or unrelated matrix views are not the same lane merely because they came from one setup seed.

### 4.2 Scalar extension and public state

Let $F=\mathbb F_q$ and choose a finite field $K=\mathbb F_{q^\nu}$ for fresh accumulation challenges. For a ring lane of dimension $d_\ell$, use

$$
R_{K,\ell}=K[X]/(X^{d_\ell}+1)
$$

only as a **public verification algebra**. Lift the original $F$ coefficients into $K$. This does not posit a new SIS assumption over the extended ring.

Keep public vectors

$$
a_t=\bigl(t,h_t,\mathrm{boundary}_t,
(U_{\ell,t},Y_{\ell,t})_{\ell\in\mathcal L}\bigr),
\qquad U_{\ell,0}=0,\quad Y_{\ell,0}=0.
$$

The common parameter/catalog/program identity is fixed and bound to the state. State size is $O(\sum_\ell(m_\ell+n_\ell)d_\ell\nu)$ base-field coordinates, independent of $T$, not necessarily small in $N$.

### 4.3 Update and verifier

1. Replay $V_0$ and the segment front-end checks; reject malformed inputs or a false nondeferred check.
2. Derive all current $x_{\ell,t},y_{\ell,t}$ and check the composition boundary.
3. Bind the full old state, current segment identity/public statement, complete original proof, derived lane descriptors, and versioned accumulator domain **before** sampling the fresh challenge.
4. Draw $\alpha_t\leftarrow K^\times$ independently of the fixed incoming data.
5. Compute and check, for every lane,

$$
U_{\ell,t+1}=U_{\ell,t}+\alpha_t x_{\ell,t},
\qquad
Y_{\ell,t+1}=Y_{\ell,t}+\alpha_t y_{\ell,t}.
$$

6. Update the authenticated count, boundary, and history digest. The update verifier recomputes the arithmetic; it does not trust a prover-supplied folded vector.

A history digest may compress identity binding, but the receipt must prove or replay Steps 1–6. Full canonical proof bytes can be absorbed directly. If only a digest is absorbed, the proof needs an explicit collision/second-preimage binding argument for those bytes.

### 4.4 Decider

$$
\mathsf{Dec}(a_T)=1
\Longleftrightarrow
\bigwedge_{\ell\in\mathcal L}A_\ell U_{\ell,T}=Y_{\ell,T},
$$

together with the fixed descriptor/format checks. The application receipt separately authenticates the terminal state and intended accepting execution.

The original prover need not disclose CPU witness traces to this accumulator; completed proof bytes contain the public residual data. The state itself is public arithmetic, not a new Ajtai opening. **No norm bound on $U_{\ell,T}$ is used to claim SIS binding.** Every original response bound is still checked individually. Committing this state under a lattice commitment, hiding it, or turning it into a private split accumulator would require a new bounded encoding and consistency proof.

### 4.5 Interactive backward-soundness lemma

Define the old and new residuals

$$
r_\ell=A_\ell U_\ell-Y_\ell,
\qquad e_\ell=A_\ell x_\ell-y_\ell.
$$

Linearity gives

$$
r'_\ell=r_\ell+\alpha e_\ell.
$$

If some old/new residual is nonzero, choose a nonzero coefficient coordinate across all lanes. Its equation is $r+\alpha e=0$ over $K$. If $e=0$, a nonzero $r$ can never vanish. If $e\ne0$, at most one challenge satisfies it. Therefore, for data fixed before uniform $\alpha\in K^\times$,

$$
\Pr[\mathsf{Dec}(a')=1\land
(\mathsf{Dec}(a)=0\lor\exists\ell:e_\ell\ne0)]
\le\frac1{|K|-1}.
$$

Valid old/input residuals remain zero for every challenge, so the arithmetic update has perfect completeness. Conditional on any prior interactive history, the same one-step bound applies if the current data are fixed before the new independent challenge. A union bound over at most $T$ accepted transitions gives the conservative arithmetic error $T/(|K|-1)$.

This lemma is a derived algebraic argument, not a theorem quoted from Akita. It has no challenge-unit or norm-expansion requirement for private lattice witnesses: multiplication by a nonzero field scalar is invertible in the public $K$-module even when the quotient ring has zero divisors.

### 4.6 Fiat–Shamir and quantum limits

**[INFERENCE / receipt-composition proof obligation]** In a classical independent lazy-sampled accumulator oracle model, a fresh query input must fix the entire old state and incoming residual data. Each such query has at most one bad challenge. If $Q_{\mathrm{acc}}$ counts adversary queries/attempts and $U_{\mathrm{out}}$ counts distinct unqueried inputs evaluated on the final output, the derived arithmetic bound is $(Q_{\mathrm{acc}}+U_{\mathrm{out}})/(|K|-1)$, plus sampler/digest errors. A replayed $T$-record output has $U_{\mathrm{out}}\le T$; a single final candidate can require $+1$. If a total query budget already includes verifier/output queries, do not add them again. Joint original-proof oracle access, exact framing/derivation, retries, state serialization, and receipt composition remain separate obligations. Do not count only the final successful nonce.

This is not a QROM theorem. Quantum oracle search and extraction need a separate analysis. Choosing an extension field can enlarge the cancellation budget; it does not repair the original segment/PCS security budget or prove quantum-safe recursion.

### 4.7 What this baseline does not save

Deferring only Equation (163) does **not** remove Akita's preceding fold, sumcheck, norm, sampling/retry, transcript, and decoding costs. The decider saves repeated applications of a fixed public matrix, while the update still reads and combines public vectors.

A broader public-setup scan accumulator is possible only after identifying an exact residual from the original verifier endpoint. If a proof does not explicitly carry a standalone claim $\sigma=\langle S,\omega\rangle$, do not invent one or divide by a possibly zero witness evaluation. Instead move the already-derived public terms to one side. Schematically,

$$
L=a(M_{\mathrm{structured}}+\langle S,\omega\rangle+M_{\mathrm{compression}})+B
\quad\Longleftrightarrow\quad
\langle S,a\omega\rangle
=L-a(M_{\mathrm{structured}}+M_{\mathrm{compression}})-B.
$$

This is an endpoint-rearrangement template, not a verbatim Akita equation. The actual range/norm/binary and relation endpoint terms must come from the exact native verifier. Accumulate the resulting dense weight vector and target. Building dense weights may cost as much as the old scan and discard Akita's factored-weight advantage. Offloaded schedules already authenticate setup products through a registered setup-opening protocol; those checks cannot simply be deleted. See [Akita §7.5][aki].

## 5. Candidate B: native fixed-shape re-entry

**[INFERENCE / UNPROVED] Proposed witness-based construction.** Target the Stage 8 boundary before completing each segment's Akita proof. The aggregator needs the original polynomial/opening hints. This is not a public aggregation algorithm for arbitrary completed proof bytes.

### 5.1 State and declared witness domain

Maintain one fixed-capacity opening:

$$
a_t=(\mathrm{profile},t,C_t,r_t,v_t,h_t,\mathrm{boundary}_t),
\qquad \widetilde w_t(r_t)=v_t.
$$

The prover retains $w_t$ and its commitment-side hint state. Fix capacity $N_*$, admitted alphabets, commitment geometry, source map, matrix views, and maximum fresh group count before the first challenge.

**Do not define the old-state domain as any ordinary Akita proof that happens to verify.** Akita's extracted algebraic decommitment need not be a canonical short source state. A useful proposed domain is:

$$
\begin{aligned}
R_{\mathrm{acc}}(a_t;w_t,\mathrm{hints}_t)
\Longleftrightarrow{}&
|w_t|=N_*,
\quad (w_t)_j\in\mathcal A_{b_*}\text{ for every stored coordinate},\\
&\mathrm{hints}_t\text{ open the frozen }C_t\text{ to this full-grid }w_t,\\
&\widetilde w_t(r_t)=v_t .
\end{aligned}
$$

Here $\mathcal A_{b_*}$ is the declared balanced integer alphabet, interpreted through prescribed field lifts. For an injective coefficient-packed full-grid representation, choose the identity source map $\delta_{\mathrm{com}}=1$. Then $s=\operatorname{Pack}(w)$, and an authenticated range of the decoded vector controls the old lane's source directly. Trace packing requires its own coefficient-expansion accounting.

**[INFERENCE / proof obligation]** The next range/relation reduction must certify this domain for produced states. A final decider that checks only an ordinary PCS evaluation does not establish the bounded-domain predicate for every decider-accepted old state. Add a source-range/bounded-decommitment certificate to the final decider, or explicitly define a weaker generated-state completeness contract and justify why the receipt enforces it. Revealing a fixed $N_*$ witness and checking its linked bounded algebraic decommitment once is a conservative decider option; a compact source-range proof is additional protocol work.

Fresh original segment sources remain linked to their original handles and declared PCS decommitment semantics. The identity-source choice for the accumulator does not make their gadget decompositions canonical.

### 5.2 Update sequence

For one old accumulator and at most $g_{\max}$ fresh segment groups:

1. Verify or prove the original Jolt front-end checks and boundary composition.
2. Resolve and bind the fixed re-entry profile, original ordered handles, points, evaluation vectors, roles, original transcript/session, old state, segment count, and application statement.
3. Form the original partial-opening data. Commit their concatenated digits through the shared D/H path before sampling the combination and sparse folding challenges.
4. Form the group responses

$$
z_g=\sum_{c,i}c^A_{g,c,i}s_{g,c,i}.
$$

5. Decompose responses and build **all** successor segments from Akita Equation (109): response digits, partial-opening digits, inner-image digits, quotient digits where required, both compression stages, alignment, and output padding.
6. Commit the successor with the same fixed receiving geometry. Perform the native relation/range/binary and opening-method reductions, including required public setup evaluations.
7. Verify the reduction prefix and defer only the resulting ordinary opening $(C_{t+1},r_{t+1},v_{t+1})$, plus any explicitly declared domain certificate. Old input rows are consumed through the sound reduction, not retained in a growing list.

This uses original-handle A/B/F consistency, not $C_{t+1}=C_t+\rho C_{\mathrm{seg}}$. Native independent-group batching therefore remains useful even though the full commitment is not homomorphic. See [Akita §§4.5, 5.3–5.4, 7][aki].

The existing admitted topology allows application groups only at the root. Later levels consume exactly the recursive witness, optionally with the registered setup source. The proposed old-plus-fresh re-entry is a **new topology/profile/API**, not a currently valid schedule. Start with a conservative coefficient-bound, direct-setup, unchunked profile; extend other routes only after proving their compatibility. See [Akita §9.3][aki].

### 5.3 Capacity fixed point

For group $g$, let $P_g$ be the local source-vector count, $B_g$ the actual block count, $m_{A,g}=M_g\delta_{\mathrm{com},g}$ the source width, $d_{A,g}$ its ring dimension, $d_{\mathrm{op},g}$ the partial-opening base-field width, and $\delta_{f,g},\delta_{\mathrm{open},g}$ the response/image digit depths.

Before quotient, compression, and alignment terms, Equation (109) gives the base-field coordinate count

$$
L_{\mathrm{main}}
=\sum_g\left[
m_{A,g}d_{A,g}\delta_{f,g}
+P_gB_g\delta_{\mathrm{open},g}
\left(d_{\mathrm{op},g}+n_{A,g}d_{A,g}\right)
\right].
$$

Let $L_{\mathrm{tail}}$ contain the exact quotient digits, F/H binary digits, ring completion, alignment, and output-domain padding. The receiver must satisfy

$$
\Phi_{\mathrm{profile}}(N_*,N_{\mathrm{segment}})
:=L_{\mathrm{main}}+L_{\mathrm{tail}}\le N_*,
$$

with the old group's own block count and all dependent widths evaluated **at $N_*$**. Commit/pad to exactly $N_*$ after every update. Do not silently change the output shape at every step.

This is a real fixed-point admission problem. Increasing $N_*$ also changes block counts, digit depths, D widths, response bounds, and SIS parameters. “Choose a large enough accumulator” is not a proof that a suitable receiver exists. A deterministic norm envelope may remove modeled-history assumptions but also make the fixed point fail. No concrete feasible profile or performance number is claimed here.

### 5.4 Norm, binding, and source semantics

Akita Definition 10.14 certifies bounds on products such as $\Delta c\,s$, not on $s$ itself:

$$
s=(\Delta c)^{-1}(z_1-z_0).
$$

Small response differences and small unit differences do not imply a small extracted source. A toy example over $\mathbb F_{101}$ is

$$
\Delta c=2,\quad \Delta z=1,\quad
s=2^{-1}=51\equiv-50\pmod{101}.
$$

Its centered source is large even though the product is small. Akita's weak binding clears denominators and prices the resulting collision; its exact response-energy check does not prove a canonical source opening. See [Akita Definitions 10.13–10.14, Lemma 10.15, Corollary 10.17, Remark 10.32][aki].

For evaluation-only obligations, a declared weak/full-grid decommitment relation can be sufficient for the application if the PIOP supplies the needed semantics. For an LF+/SuperNeo-style exact-short-source relation, add a source checkpoint linked to the **same** original commitment. A proof about a freshly recommitted unrelated witness does not provide that link.

### 5.5 The long-chain extraction obstruction

Akita's current theorem uses

$$
\operatorname{Tree}(\mathrm{sch})
=\prod_{r\in\operatorname{Rounds}(\mathrm{sch})}b_r
$$

and requires it to be bounded by a fixed polynomial in the security parameter and explicit instance/witness length. Each nontrivial fold contributes a factor $1+\sum_g W_g$. Repeating a fixed fold $T$ times would give

$$
\operatorname{Tree}\ge(1+W)^T.
$$

The existing runtime bound is polynomial in this tree, not automatically polynomial in $T$. Therefore simply repeating the finite-schedule backward induction does **not** establish expected-polynomial extraction for a long stream. See [Akita §9.3, Equation (176), and §10.2, Equations (225)–(227)][aki].

**Required new theorem:** modular adaptive extraction/uniqueness with runtime polynomial in the declared $(\lambda,T,N_{\mathrm{segment}},N_*)$, without materializing the exponential whole-history fork tree. Match commitment/source projections across original and receiving profiles; handle adaptive extractor calls and independently sampled extractions; state the path-versus-DAG model.

[SuperNeo §6][superneo] supplies a useful strong–weak composition contract. [BCLMS §2.2][bclms-published] supplies the multi-instance/auxiliary-I/O warning. Neither is a ready theorem for repeatedly applying Akita. Merely citing strong–weak composition does not prove source shortness or fix the runtime bound.

### 5.6 Final decision

The decider checks one final opening under a fixed admitted suffix, plus the bounded-domain predicate required by the chosen relation and the authenticated history/composition receipt.

If using Akita's clear terminal instead, preserve its predecessor-authenticated $t_i=A_{\mathrm{term}}s_i$ state. An ordinary two-tier commitment cannot be replaced by a terminal $t$ list without proving the transition. Discharge every pending registered setup opening before the terminal.

The distinction is explicit in [Akita §§8.1–8.2][aki]: ordinary receiver openings and terminal inner states are different public interfaces.

## 6. Other paper mechanisms worth using

### 6.1 Halo's nonhomomorphic consistency route

[Halo Infinite §3, Definition 3.3][halo-published] gives a linear-combination-consistency template even when the full commitment is not additively homomorphic.

**[INFERENCE / UNPROVED Akita adaptation]:** given original committed polynomials, form a fresh

$$
f_*=\sum_i\gamma_i f_i,\qquad \gamma_i\in F,
$$

commit to it, then sample an independent evaluation point after all commitments are fixed. Authenticate openings of **both** the originals and the new polynomial and check their linear-combination identity there. Use a proved degree/field soundness bound.

The original input evaluation points can differ. A standard multilinear reduction starts from

$$
\sum_{x\in\{0,1\}^{\mu}}
\sum_i\gamma_i\operatorname{eq}(r_i,x)f_i(x)
=\sum_i\gamma_i v_i
$$

and sumcheck reduces it to evaluations at a common fresh point. This explains why adding values from different points is not valid:

$$
f_1(r_1)+f_2(r_2)
\not\equiv(f_1+f_2)(r_*)
$$

in general. A bounded-capacity embedding is required when polynomial domains differ.

Akita's multipoint original-handle batch could authenticate these linked openings. However:

- Generic consistency may cost as much as the full opening verifier.
- A consistency proof is not by itself a recurring evaluation accumulator; specify what deferred claims remain after each step.
- Additive lattice commitments valid only on bounded messages do not satisfy Halo's unrestricted additive-group interface automatically.
- A base-field polynomial cannot silently acquire extension-field coefficients. Represent extension coordinates explicitly or prove an appropriate extended commitment interface.
- Private aggregation uses polynomials/opening hints. Halo's public aggregation compiler needs extra public residual structure; it is not supplied merely by an efficient consistency check.

See [Halo Definitions 4.1, 5.1, and 6.1][halo-published]. The old commitments must be opened or otherwise linked; fresh recommitment alone is unsound.

### 6.2 Lova: exact source-norm checkpoints

Lova's relation adds an integer Gram matrix:

$$
AS\equiv T\pmod q,\qquad
D=S^\top S\text{ over }\mathbb Z,\qquad
D_{ii}\le\beta^2.
$$

Its decomposition/folding and integer identity checks restore the original extraction norm. This addresses both honest norm growth and inverse-challenge extraction growth. Its special $\pm1$ forks show that pairwise unit differences are not a universal requirement: a valid alternative extraction method can suffice.

**[INFERENCE] Use as a source-checkpoint design reference**, not as a drop-in Akita theorem. Lova uses unstructured SIS and a power-of-two modulus, whereas the current Akita deployment uses Module-SIS and a prime field. Its paper also leaves general relaxed-R1CS folding unresolved. Integer/no-wrap and field/ring conversion need new proofs. See [Lova §§1.2, 2.3, 3][lova].

### 6.3 LatticeFold, LatticeFold+, Neo/SuperNeo

- [LatticeFold §3][lf] uses decomposition, relaxed binding, and exact range checks to obtain recurring bounded opening witnesses. Invertible challenge differences and low operator expansion are separate requirements.
- [LatticeFold+ §4.4][lfp] explicitly handles nonhomomorphic double commitments with a transformation. Flattening every range auxiliary into the next witness causes repeated witness-size blowup. Its final closure returns two bounded instances after folding to $B^2$ and decomposing.
- [Neo/SuperNeo §§5–7][superneo] provide coefficient-preserving small-field packing, compatible matrix/evaluation transforms, strong–weak composition, and signed decomposition returning a fixed tuple of bounded claims.

**[INFERENCE] These are alternatives to relying on native Akita re-entry.** They offer stronger bounded-source relations, but require a commitment-consistency and representation bridge to the original Akita handles. SuperNeo folds full lifted ring evaluations; retaining only their scalar projections is insufficient. Do not combine ring challenges with scalar evaluation values as if ring multiplication preserved every coordinate separately.

### 6.4 SALSAA: retire old rows, not just their scalar sum

[SALSAA §4.3 and §7][salsaa] makes eager row removal part of the accumulator invariant. Lazy batching can leave a linearly growing public constraint system even when one sumcheck combines its values.

**[INFERENCE] Apply this invariant to Akita re-entry:** count retained commitment handles, setup obligations, relation rows, and successor segments after every update. They must fit the fixed profile, not just yield one scalar endpoint.

SALSAA distinguishes correctness and extraction norm budgets. Its ordinary ring/module-SIS folding result must not be conflated with its vSIS-based fully succinct backend.

### 6.5 Nova-family limits

Nova's relaxed algebra closes because

$$
AZ\circ BZ=uCZ+E,\qquad
Z'=Z_1+\rho Z_2,\qquad
E'=E_1+\rho T+\rho^2E_2,
$$

with

$$
T=AZ_1\circ BZ_2+AZ_2\circ BZ_1-u_1CZ_2-u_2CZ_1.
$$

Its commitment/randomness updates and extraction must match these equations. [HyperNova][hypernova] instead uses CCS linearization and common-point sumcheck; [NeutronNova][neutronnova] folds structured sumclaims and preserves deferred helper-vector power checks.

**[INFERENCE] Borrow their relation-closure and deferred-obligation discipline, not their group theorems.** Replacing the final PCS of a group accumulator does not make its group commitment layer post-quantum. NeutronNova explicitly does not establish constant-overhead lattice recursion.

## 7. Required properties and how to satisfy them

| Requirement | Public-residual baseline | Native re-entry |
|---|---|---|
| Exact input predicate | Complete proof, with an exact $V_0$ plus deferred-linear-equation split. | Original typed opening claims and witness/hints; not arbitrary proof-only input. |
| Strong completeness | Arithmetic update preserves zero residuals for every decider-valid old state and valid new input. | Define the bounded old-state domain; certify it in the decider/receipt. Prove admission for that full domain. |
| Backward soundness / knowledge | One-step affine cancellation bound under fresh, pre-input-fixed field randomness; compose with actual segment proof guarantees. | Extract original committed claims under the exact weak/bounded relations; prove modular expected-polynomial adaptive composition. |
| Public reducibility | Verifier recomputes lane vectors and their updates from authenticated proof data. | Verifier derives the exact successor statement from the reduction prefix without private hints. |
| Fixed state | Fixed finite lane catalogue; public vectors can be large in setup/terminal width. | Certify $\Phi(N_*,N_{\mathrm{segment}})\le N_*$ and bounded rows/groups/auxiliaries. |
| Commitment consistency | Preserve original transcript/terminal-state authentication; no new folded commitment is assumed. | Check original A/B/F paths and all successor compression/recomposition rows. |
| Norm security | Check each original response norm/encoding; do not use an accumulated vector as a short SIS opening. | Price enforced alphabets, challenge products, weak-opening cross products, source checkpoints, and all shared-prefix views. |
| Challenge security | Nonzero field scalars with sufficient query/depth budget; exact or charged sampling distribution. | Short carrier challenges with adequate accepted support, unit-difference or alternate extraction conditions, operator bounds, and separately sized sumcheck fields. |
| Transcript binding | Original proof FS plus independent accumulation domain; bind old state and complete new input before the challenge. | Same requirement, plus pre-challenge shared D/H and receiving commitment binding. |
| Application semantics | Verify/prove segment continuity, global memory, count/order, input/output, program, and acceptance. | Identical requirement; norm and commitment proofs do not establish CPU semantics. |
| Final receipt | Replay history or prove front-end/update/composition and one final matrix check. | Authenticate history and decide the complete final opening/domain relation once. |
| Privacy | Not provided by the public construction; current Jolt Akita is clear-only. | Needs a separate accumulator/receipt hiding and simulation theorem. |
| Quantum/security model | Classical algebra/oracle argument only as stated; no QROM claim. | Akita's classical ROM theorem is not a quantum recursive-accumulation theorem. |

A finite maximum $T_{\max}$ and a fixed supported profile family are acceptable design choices if they are explicit and priced. They are not an unbounded-depth theorem. Cyclo illustrates bounded additive norm growth; unbounded use needs a refresh mechanism. See [Cyclo §§2.2, 2.5, 6][cyclo].

## 8. Cost model and research sequence

### 8.1 Account for all work

For full recursive verification,

$$
C_{\mathrm{old}}
\approx T\left(C_{\mathrm{JoltFront}}+C_{\mathrm{AkitaFull}}\right)
+C_{\mathrm{outer}}.
$$

For an accumulation receipt,

$$
C_{\mathrm{new}}
\approx T\left(C_{\mathrm{JoltFront}}+C_{\mathrm{AccUpdate}}\right)
+C_{\mathrm{Dec}}+C_{\mathrm{receipt}}+C_{\mathrm{Link/Memory}}.
$$

These are accounting templates, not measured timings. Separate native field operations, guest cycles, circuit rows, hash/encoding work, proof bytes, public-state bytes, and peak prover memory. Shared helper attribution is not an additive decomposition of savings.

For Candidate A, $C_{\mathrm{AccUpdate}}$ still includes nearly the whole Akita verifier if only the terminal A equation is deferred. Dense public-weight formation can retain $O(N_{\mathrm{setup}})$ per-update cost. For Candidate B, do not first generate every full segment PCS proof and then redo its first fold unless the extra work is deliberate: enter before backend `batched_prove` where original hints are available.

For native replay, final verifier work includes $T$ front-end/update checks. For a compact wrapper, those checks move into the wrapper prover. A fixed-size online accumulator does not automatically make the final wrapper's witness storage or proving space constant in $T$.

### 8.2 Concrete research order

1. **Specify the segment relation first.** Define boundary and global-memory objects; distinguish independent-proof conjunction from one continued execution.
2. **Freeze a source pair and setup family.** Use the fetched native-batch branch for the modern Stage 8 seam. Porting it to the Blake recursion delivery is separate work; do not mix their pins or transcript epochs implicitly.
3. **Derive an exact verifier split.** List every deferred equation and every retained check; require that their conjunction equals the original verifier predicate. Keep an explicit pending obligation instead of reporting success.
4. **Establish Candidate A's complete classical atomic contract.** State field/query budgets, lane addressing, serialization, digest binding, and receipt composition. This supplies a conservative baseline.
5. **Certify a native receiving profile.** Compute all Equation (109) lengths, deterministic norm envelopes, matrix widths/radii, and comparison classes at the same $N_*$. A failed fixed point is useful evidence, not permission to drop auxiliaries.
6. **Prove the long-chain theorem.** Address strong completeness, source projection/uniqueness, adaptive extraction, and polynomial runtime before claiming a native scheme.
7. **Choose final discharge and history certification.** Retain a lattice Akita outer if that is the target; terminal BN254 R1CS fragments are not an already-integrated post-quantum wrapper.
8. **Measure the actual replacement.** Compare equal segment semantics and equal receipt guarantees. No speedup is claimed by this research note.

These are research acceptance gates, not implementation tasks performed in this session.

## 9. Appended branch-specific recursion findings

This work remains research-only. Branch switching or creating a branch from a fitting base is allowed if later inspection needs it. This entry used fetched refs and pinned branch blobs; no new working branch was needed.

### 9.1 The lattice outer already exists in one source branch

`origin/wrap/fast-recursion-blake` contains a separate `outer` command that traces a supplied verifier guest, builds a modular q128 Akita outer proof, and verifies it. It is **not** a BN254/Groth16 outer wrapper. Its ordinary Akita `verify` command remains trace-only; the two command paths must not be conflated. See [outer source][jolt-blake-outer].

The inspected code proves execution of a guest checking a conjunction of independent inner proof/device statements. It does not establish PC/register/memory continuation between them. `origin/feat/fast-recursion` separately rejects Akita outer proving and supports guest tracing only. See [fast-recursion scope][jolt-fast-spec].

Read-only source inspection is not a new proof run. The fetched Blake delivery note retains compilation/profile/replay qualifications for its particular companion pin; historical timing/proof evidence must not be relabeled as measurements of the fetched delivery pair. See [public delivery record][jolt-blake-delivery].

### 9.2 Transcript epochs differ

The Blake companion uses epoch-6 indexed Blake2b streams with domain `akita/sparse-challenge/blake2b512/v1`. Other source branches/paper descriptions use different stream constructions. Preserve the exact selected branch's frame, nonce, first-accepted sampling, byte-consumption, and transcript rules. A proof that checks only sparse-shell membership is not a proof of native challenge derivation. See [pinned Blake stream][akita-blake-stream].

### 9.3 Terminal circuit records are conditional components

The related terminal/norm/context branches build `R1csBuilder<jolt_field::Fr>` constraints over BN254, with foreign-field fp128 arithmetic. They are separate from the lattice guest/outer route.

The later `wrap/akita-terminal-context` record reports a conditional full terminal-suffix fixture with 22,375,450 rows. That is **producer-reported source evidence**, not a benchmark reproduced here. It excludes authentication of the incoming prefix and backend proof generation. It demonstrates why terminal arithmetic, retry sampling, norm checks, and transcript context must be separated when estimating an accumulator's savings. See [conditional validation record][jolt-terminal-context-record].

The old unvalidated-context checkpoint is superseded by that later conditional-fragment record, not by an end-to-end verifier theorem. Reproduction also depends on external archived instrumentation/checkpoint artifacts. This investigation did not obtain or execute them.

### 9.4 Application acceptance and observed documentation defects

- The Blake outer runner rejects guest panic but does not itself require the verifier guest's public result to equal one. Consumers must bind the intended guest image/setup/input and check an accepting output.
- The conjunction guest returns true for zero inner proofs. A receipt for a nonempty continued execution must reject $T=0$ or explicitly allow the empty case in its statement.
- These are source semantics and application acceptance obligations, not proof of an exploit in a consumer that already performs the checks. See [outer acceptance boundary][jolt-blake-outer] and [guest count/conjunction][jolt-blake-guest].
- The fast-recursion README says `tamper-opening` changes a typed Stage 1 opening by one; source instead flips a bit in serialized `joint_opening_proof`. This documentation mismatch was observed, not fixed during research. See [README][jolt-fast-readme] and [implementation][jolt-fast-tamper].
- Current modular `jolt-openings` is stateless; old “opening accumulator” names refer to intra-proof claim collection or legacy code, not a cryptographic accumulator/decider scheme. See [current opening contract][jolt-openings-contract].

### 9.5 Paper/source-quality cautions

`Papers/parsed/SuperNova.md` is actually HyperNova. Neo's earlier parsed range endpoints conflict with its strict bound; the revised SuperNeo gives the intended interval. Cyclo's parsed text has inconsistent decomposition bases, reconstruction signs, range-error expressions, and a theorem/benchmark parameter mismatch; its practical approximate challenge analysis is explicitly heuristic. Some displays in HyperNova, SuperNeo, and SALSAA are damaged by extraction. Do not silently repair these into verbatim theorem statements. This note uses the supported prose, revised formulations, and explicit security boundaries.

## 10. Verification and remaining theorem limits

Performed algebra checks, using deterministic finite loops and no new dependencies:

- 30 relaxed-R1CS folding identities over $\mathbb F_{101}$, including $\rho=0,1,-1$ and nontrivial relaxation scalars.
- Balanced base-4 carry: $\operatorname{Dec}(1)=(1,0)$ but $\operatorname{Dec}(2)=(-2,1)\ne(2,0)$.
- Short-product/inverse counterexample over $\mathbb F_{101}$.
- Multipoint counterexample: equal declared evaluations at different points need not determine the combined polynomial at another point.
- 2,400 nontrivial old/new residual-vector pairs over $\mathbb F_7^2$; no pair has more than one nonzero challenge that makes both output residual coordinates zero.

These checks support the stated algebra examples. They are not proofs of lattice binding, Fiat–Shamir extraction, recursion, zero knowledge, or performance.

The local relevant accumulation papers, including their appendices, were read through qmd. The external sources read were the official published versions:

- BCMS: §§1.1–1.2 and §§2.1–2.4.1, plus the opening of §2.4.2.
- Halo Infinite: relevant definitions and construction/composition sections §§1–6, including Definition 6.1.
- BCLMS: §§1.1, 2.1–2.5 and the setup of §2.6.

The full ePrint-only BCMS/BCLMS technical games/proofs and Halo's deferred full-version IVC exposition were not read: the ePrint PDF route returned metadata or HTTP 403; actual text was obtained from official publication PDFs. Do not claim an instantiated formal WEE/recursive reduction from those unread details.

Akita's native knowledge theorem is classical-ROM only. Its Module-SIS quantum attack-cost estimates are a separate coordinate. A QROM theorem, an accumulator privacy theorem, an all-valid-state native completeness theorem, and the native long-chain polynomial-extraction theorem remain new requirements. Candidate A's field cancellation lemma does not remove them from a stronger end-to-end claim.

## 11. References

### Local papers

- **[Akita]** [Akita: A High-Performance Lattice-Based Polynomial Commitment Scheme][aki]. §§4.2, 4.5, 5.3–5.4, 6.2, 7.5, 8.1–8.2, 9.3, 10.2; Definitions 10.13–10.14, Corollary 10.17, Remark 10.32; Equations (109), (163)–(165), (176), (225)–(227); Appendix E.3. MCP addresses include `line:2307-2376`, `3026-3097`, `3194-3245`, `3603-3990`, `4141-4250`.
- **[Lova]** [Lova: Lattice-Based Folding Scheme from Unstructured Lattices][lova]. §§1.2, 2.3, 3; Definitions 2–5 and integer Gram relation. MCP `line:173-238`, `279-322`, `329-526`.
- **[Nova]** [Nova: Recursive Zero-Knowledge Arguments from Folding Schemes][nova]. §§3–6, Definition 6, relaxed-R1CS construction, final compression, Appendix B. MCP `line:193-348`, `357-630`, `749-906`.
- **[HyperNova]** [HyperNova: Recursive arguments for customizable constraint systems][hypernova]. File named `SuperNova.md`; §§2–4, 6–9 and compiler proofs. MCP `line:207-514`, `591-1034`.
- **[NeutronNova]** [NeutronNova: Group-based folding done right][neutronnova]. §§3–5, terminal decider §5.4, Appendix A.3. MCP `line:205-782`, `1223-1256`.
- **[LatticeFold]** [LatticeFold: A Lattice-based Folding Scheme and its Applications to Succinct Proof Systems][lf]. §§2.3, 2.5, 3, 4; Remark 3.3 distinguishes interactive analysis from deferred Fiat–Shamir analysis. MCP `line:256-271`, `308-353`, `362-1092`.
- **[LatticeFold+]** [LatticeFold+: Faster, Simpler, Shorter Lattice-Based Folding for Succinct Proof Systems][lfp]. §§4.1, 4.4, 5.1–5.3, Appendices A–B. MCP `line:332-379`, `634-1017`, `1018-1243`, `1398-1482`.
- **[Cyclo]** [Cyclo: Lightweight Lattice-based Folding via Partial Range Checks][cyclo]. §§2.2, 2.5, 6; heuristic approximate sampling Appendix B.2. MCP `line:140-155`, `216-223`, `514-633`, `1043-1080`.
- **[Neo]** [Neo: Lattice-based folding scheme for CCS over small fields and pay-per-bit commitments][neo]. §§3–5. MCP `line:337-781`.
- **[SuperNeo]** [Neo and SuperNeo: Post-quantum folding with pay-per-bit costs over small fields][superneo]. §§5–7; Definitions 16–18, Theorem 12, Lemmas 7–8, Theorem 13. MCP `line:591-1154`, full decomposition proof `2140-2228`.
- **[SALSAA]** [SALSAA: Sumcheck-Aided Lattice-based Succinct Arguments and Applications][salsaa]. §§3.2–3.3, 4.3–4.5, 7; Corollary 4.7, Theorem 7.3, Lemma 7.6. MCP `line:231-294`, `435-648`, `1267-1432`.
- **[Jolt]** [Jolt: SNARKs for Virtual Machines via Lookups][jolt-paper]. §§2.2, 3.2, 7 and Appendix B.3 for PIOP/PCS and memory semantics; not evidence of the current repository backend. MCP `line:283-294`, `419-428`, `820-831`, `1193-1248`.
- **[CPU]** [Proving CPU Executions in Small Space][cpu]. §1.1 for continuation matching and recursion costs. MCP `line:21-78`.

### Primary online accumulation papers

- **[BCMS]** Bünz, Chiesa, Mishra, Spooner. [Proof-Carrying Data from Accumulation Schemes, ePrint 2020/499][bcms-eprint]; published as **Recursive Proof Composition from Accumulation Schemes**, TCC 2020. [Official publication PDF][bcms-published], [DOI][bcms-doi]. §2.2 Definitions 1–2 are explicitly informal in this published version; §2.3 is the predicate-efficient verifier compiler.
- **[Halo Infinite]** Boneh, Drake, Fisch, Gabizon. **Halo Infinite: Proof-Carrying Data from Additive Polynomial Commitments**, CRYPTO 2021. [ePrint 2020/1536][halo-eprint], currently titled **Recursive zk-SNARKs from any Additive Polynomial Commitment Scheme**; [official publication PDF][halo-published], [DOI][halo-doi]. Definitions 3.3, 4.1, 5.1, 6.1 distinguish consistency, private/public aggregation, and accumulation.
- **[BCLMS]** Bünz, Chiesa, Lin, Mishra, Spooner. [Proof-Carrying Data without Succinct Arguments, ePrint 2020/1618][bclms-eprint], CRYPTO 2021. [Official publication PDF][bclms-published], [DOI][bclms-doi]. §§2.1–2.2 distinguish split/knowledge accumulation and multi-instance extraction; Remark 1 warns about the oracle-instantiation gap.

### Repository citations

Branch citations are pinned to the inspected commits. Their linked producer records describe their own validation scope; no build, test, benchmark, proof generation, or saved-proof replay was run during this research.

[aki]: SOURCE_PROVENANCE.md#akita
[lova]: SOURCE_PROVENANCE.md#lova
[nova]: SOURCE_PROVENANCE.md#nova
[hypernova]: SOURCE_PROVENANCE.md#hypernova
[neutronnova]: SOURCE_PROVENANCE.md#neutronnova
[lf]: SOURCE_PROVENANCE.md#latticefold
[lfp]: SOURCE_PROVENANCE.md#latticefoldplus
[cyclo]: SOURCE_PROVENANCE.md#cyclo
[neo]: SOURCE_PROVENANCE.md#neo
[superneo]: SOURCE_PROVENANCE.md#superneo
[salsaa]: SOURCE_PROVENANCE.md#salsaa
[jolt-paper]: SOURCE_PROVENANCE.md#jolt
[cpu]: SOURCE_PROVENANCE.md#cpuexecinsmallspaces
[bcms-eprint]: https://eprint.iacr.org/2020/499
[bcms-published]: https://link.springer.com/content/pdf/10.1007/978-3-030-64378-2_1.pdf
[bcms-doi]: https://doi.org/10.1007/978-3-030-64378-2_1
[halo-eprint]: https://eprint.iacr.org/2020/1536
[halo-published]: https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_23.pdf
[halo-doi]: https://doi.org/10.1007/978-3-030-84242-0_23
[bclms-eprint]: https://eprint.iacr.org/2020/1618
[bclms-published]: https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_24.pdf
[bclms-doi]: https://doi.org/10.1007/978-3-030-84242-0_24
[jolt-main-manifest]: https://github.com/a16z/jolt/blob/629ed77b5c999fb2f52e5781b2b98a04ed6cf5ed/Cargo.toml#L304-L315
[jolt-batch-manifest]: https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/Cargo.toml#L309-L318
[jolt-blake-manifest]: https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/Cargo.toml#L295-L315
[jolt-akita-doc]: https://github.com/a16z/jolt/blob/629ed77b5c999fb2f52e5781b2b98a04ed6cf5ed/book/src/how/akita.md#L3-L20
[jolt-batch-source]: https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-akita/src/trace_onehot/source.rs#L184-L194
[jolt-batch-owner]: https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-akita/src/trace_onehot/source.rs#L270-L306
[jolt-batch-spec]: https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/specs/akita-native-trace-batching.md#L73-L79
[jolt-batch-stage8]: https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage8/akita.rs#L218-L414
[jolt-batch-adapter]: https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-akita/src/native_batching.rs#L345-L395
[jolt-batch-prover-data]: https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-akita/src/native_batching.rs#L242-L342
[jolt-blake-guest]: https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/examples/recursion/guest/src/lib.rs#L138-L207
[jolt-blake-outer]: https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/examples/recursion/src/outer.rs#L81-L314
[jolt-fast-spec]: https://github.com/a16z/jolt/blob/141460a7ffac436f4c533c684d31acaf8aaa8ace/specs/recursion-guest.md#L1-L39
[jolt-blake-delivery]: https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/specs/fast-recursion-public-delivery.md#L3-L40
[akita-blake-stream]: https://github.com/markosg04/akita/blob/d0ee56a0b12db66b4c46a87c527169737694c4c7/crates/akita-challenges/src/sampler/xof.rs#L1-L27
[jolt-terminal-context-record]: https://github.com/a16z/jolt/blob/854382589e4e0f30f774d5ee96b65c1bdc9a8d99/specs/akita-wrapper/full-terminal-context-validation.md#L39-L123
[jolt-fast-readme]: https://github.com/a16z/jolt/blob/141460a7ffac436f4c533c684d31acaf8aaa8ace/examples/recursion/README.md#L32-L35
[jolt-fast-tamper]: https://github.com/a16z/jolt/blob/141460a7ffac436f4c533c684d31acaf8aaa8ace/examples/recursion/src/main.rs#L918-L928
[jolt-openings-contract]: https://github.com/a16z/jolt/blob/629ed77b5c999fb2f52e5781b2b98a04ed6cf5ed/crates/jolt-openings/src/lib.rs#L7-L16

## Follow-up: per-obligation solution research

The six-obligation investigation is now in [Akita accumulation solutions](AKITA_ACCUMULATION_SOLUTIONS.md), with a separate new document for each issue. It adds exact bounded-domain/revelation algorithms, deterministic response admission, typed original-handle links, independent local-bridge extraction contracts, certified receiver checkpoints, full execution/memory composition, and final receipt acceptance. These are research constructions and explicit conditional/open theorems, not an implementation. A final-only domain certificate does not authenticate counterfactual fork children; per-update certified endpoints pay full fixed-state work. Native efficient accumulation still requires a secure admitted receiver profile and an instantiated bounded adaptive extraction/composition theorem.
