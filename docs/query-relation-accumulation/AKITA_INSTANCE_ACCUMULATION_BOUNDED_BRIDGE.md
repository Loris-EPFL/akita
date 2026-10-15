# Native Akita instance accumulation: the bounded original-handle bridge

**Status: conditional protocol design and exact missing lemmas, not an implementation, an established Akita accumulation theorem, or a security/performance claim.** This document addresses the bounded receiver/source interface for consuming an old native accumulator and fresh VM-segment relations *before* completing their PCS proofs. It does not select a source revision, change any commitment already formed, or solve adaptive long-chain extraction.

**Anchor.** Preserve each incoming `(shape, C)` and its exact commitment-side source projection. The old pending evaluation and every fresh frontend opening must be transferred to the successor, never assumed true. A range prefix is a reduction to additional same-handle queries, not an independently accepted opening proof. The final decider discharges the remaining bounded relation and evaluation once.

Labels below distinguish **elementary derivation**, **source theorem**, **conditional design**, and **open obligation**. References and actual read scopes are in §12. This document refines, rather than upgrades to theorems, obligations [01](AKITA_ACCUMULATION_01_FIXED_SIZE_CLOSURE.md), [02](AKITA_ACCUMULATION_02_STRONG_COMPLETENESS.md), [03](AKITA_ACCUMULATION_03_COMMITMENT_CONSISTENCY.md), and [04](AKITA_ACCUMULATION_04_POLYNOMIAL_EXTRACTION.md).

## 1. Decisive answer

A strong range prefix can restore the bound on an **extracted old receiver witness** without a completed per-segment PCS proof, provided:

1. its endpoint evaluations concern the **same original handle** as the old pending evaluation;
2. that handle has an identity, physically injective source decoder, or an explicitly proved source-observation interface;
3. the downstream weak reduction authenticates all prefix endpoints and the pending evaluation, with a unique commitment-side source across the prefix's rewinds;
4. its complete accepting descendant trees are actually available to extraction; and
5. every comparison class, alphabet, challenge, layout, and cost is certified for this new composition.

The mechanism is **recover a unique source, then interpolate the range identities**, not “invert a short challenge and obtain a short source.” It does not authenticate the old pending evaluation early. It also does not make the existing Akita recursive schedule a streaming accumulator: §9.3's stated recursive interface admits fresh application groups only at the root, while Theorem 10.23 assumes descendant-certified outgoing claims. That root-only interface is a scope policy, not an algebraic prohibition: its local proof applies by a straightforward conditional generalization to a fixed supplied group list satisfying every local hypothesis (§8). Schedule admission and the finite chain experiment still have to be specified.

For a pre-existing **full-field gadget-source** handle, a range prefix on its decoded polynomial alone is insufficient to bound the actual source digits. A new same-original-handle source-query reduction or independent bounded-source certificate is required. An alternative for **newly produced fresh inputs** is a producer migration to identity-source Boolean bitplane commitments (§5): then ordinary same-handle polynomial queries observe the complete source. This is a change at original commitment formation, not a reinterpretation of an existing commitment.

None of these answers removes the independent authenticated-descendant/chain-extraction obligation for the **admitted depth**. A valid final endpoint does not supply every counterfactual child endpoint needed by nested rewinding. A finite authenticated `T_max` may use a fully charged bounded unrolled tree; uniform polynomial extraction for growing T is a stronger research target, not a user requirement for every native construction.

## 2. Exact instances, witnesses, and domains

### 2.1 Frozen profile

Let `P` fix the base field `K = F_q`, evaluation field `E`, all ring bases and dimensions, packing/decoding maps, original A/B/F views, receiver views, B slices, source coefficient order, positional bases/depths, permitted scanning alphabets, challenge families and unit-difference filters, response envelopes, quotient policy, D/H shapes, transcript tags, capacity `N_* = 2^{mu_*}`, fresh-input catalogue, and all zero masks. Include setup expansion/version and finite source-comparison classes. The catalogue is fixed independently of setup sampling; the chosen descriptor is absorbed before its challenges.

The instance also fixes the admission policy for execution depth: either an authenticated finite `T_max` with its explicit bounded-chain certificate, or a proved uniform growing-T interface. Neither depth model is implicit; capacity and depth are different limits.

The old public instance is

\[
a=(P_{id},t,\mathrm{layout},C,r,v,\mathrm{boundary},\mathrm{history\ interface}).
\]

Its witness is `(w,d)`. Here `w` is the full receiver Boolean evaluation table; `d` is its **supplied**, not recomputed, original algebraic decommitment: source blocks, inner-image digits, per-polynomial B slice inputs, active F-chain digits/intermediates, and physical completion cells. The historical validity predicate is separate from this boundedness bridge; a history digest alone proves no VM transition.

Use a coefficient-packed receiver with `delta_com = 1`. Require a coordinate grouping `Pack_P` with

\[
\mathrm{Dec}_P(\mathrm{Pack}_P(w))=w,\qquad
\mathrm{Pack}_P(\mathrm{Dec}_P(s))=s
\tag{ID}
\]

on **all admitted physical source cells**, not just the logical input prefix. If packing allocates extra cells, include them in `d` and explicitly require zero. Absent block indices are public virtual zeros, not secret committed blocks.

Let `D_* = {-b_*/2,...,b_*/2-1}` with its prescribed signed lift. Define

\[
R_{\mathrm{acc}}^{bd}(P,a;w,d):=
\mathrm{Typed}_P(a)\land |w|=N_*\land w\in D_*^{N_*}
\land \mathrm{Zero}_P(w,d)
\land s(d)=\mathrm{Pack}_P(w)
\land \mathrm{Rel}_{com}^{Akita}(P,C,w,d)
\land \widetilde w(r)=v.
\tag{A}
\]

`Rel_com` is Definition 10.13, ending at **this C**. In particular,

\[
A s_i=G_{b_1,n_A}\hat t_i;
\quad B x_{B,\sigma}=u_\sigma;
\quad B_\Pi\hat t=G_{bin}\hat u_1,
\quad F_1\hat u_1=G_{bin}\hat u_2,
\quad F_2\hat u_2=C,
\tag{C}
\]

with the last three rows replacing the raw B-target row when compression is active. The gadget views in (C) include exact native dimensions, coefficient permutations, and stage completion cells. At compression depth zero, `B_Pi hat t = C`. Permit any decommitment satisfying these equations and the **declared certified domain alphabets**; do not require canonical inner-image/F digits unless canonicality is explicitly part of the input relation. The supplied chain is necessary because replacing noncanonical digits can change C.

### 2.2 Fresh native input

Fresh input `u` fixes the segment frontend, every original handle `C_g`, exact source shape, grid/natural lengths, opening queries, and boundary linkage. Its witness `xi` contains sources and supplied chains. Define

\[
R_{in}^{bd}(P,u;\xi):=\mathrm{FrontendAndLink}(u;\xi)
\land\bigwedge_g\left[
\mathrm{Rel}_{com}^{Akita}(C_g,f_g,d_g)
\land\mathrm{SourceDomain}_{P,g}(s_g)
\land\mathrm{NaturalDomain}_{P,g}(f_g)
\land\bigwedge_a\widetilde f_{g,a}(r_{g,a})=v_{g,a}
\right].
\tag{F}
\]

`NaturalDomain` is the exact padding or restriction predicate demanded by the application, not automatically Akita's honest padding. `SourceDomain` specifies all physical coefficient bounds/energies and any occupancy or canonicality predicate used in response sizing. Both must concern the original source under C_g. Existing frontend PIOP equations remain obligations; this bridge does not replace them with terminal residuals.

The final domain decider must recognize (A), including evaluation, not only an ordinary PCS evaluation relation. Conservative realization: reveal `(w,d)` once, check ranges/zeros, (ID), all original A/B/F equations, then compute the MLE. A compact/private replacement must prove and extract **that same joint relation**. No final opening is completed during the range prefix.

## 3. Why the old Equation (109) witness is not automatically the extracted source

**Source theorem.** Akita §5.4 forms the next witness from response digits, partial-opening digits, inner-image digits, main quotients, group-local F-chain digits, shared H-chain digits, and stage-local quotients. All are base-field cells. Its honest range is small. Definition 10.14 instead extracts a source `s` satisfying

\[
p=\rho s,\qquad \rho\in R^\times,
\qquad\|\rho\|_1\leq K_1,
\qquad\|p\|_\infty\leq B_\infty,
\tag{W}
\]

plus the original commitment path and evaluation equations. Thus the extractor gets `s = rho^{-1} p`; it does not get the honest Eq. (109) digit table for free. Remark 10.32 states this limitation explicitly.

**Elementary obstruction.** In the scalar subring of an odd-q ring, `rho = 2` is a unit and `p = 1` is small, but the centered lift of `rho^{-1} p` has magnitude `(q-1)/2`. This is an algebraic example, not a statement that 2 belongs to a selected Akita challenge-difference family. Any proposed family must separately certify

\[
\sup_{\rho\in\Delta\mathcal C}\|M_{\rho^{-1}}\|<\infty
\]

at a useful bound if it intends to use inverse-norm control. Unit-difference certification alone supplies no such useful bound. The trivial full-field centered envelope is too large to infer the intended receiver digit domain.

**Elementary uniqueness repair, not a norm repair.** For two same-handle candidates `s,s'`, after B/F binding identifies their inner-image digits, let `p = rho s`, `p' = rho' s'`. Then

\[
z_A=\rho' p-\rho p'=\rho\rho'(s-s'),\qquad A z_A=0,
\]

and

\[
\|z_A\|_\infty\leq K_1' B_\infty+K_1 B_\infty'.
\tag{U}
\]

The vector is nonzero if `s != s'`, because both multipliers are units. The bounds concern unreduced integer negacyclic products, followed by centered reduction. They do not bound `s`. Once a same-source range reduction certifies `s`, uniqueness can transfer that certification across openings. It cannot create the certification.

## 4. Strong same-C range prefix: concrete polynomial reduction

### 4.1 A simple exact prefix

The following is an **elementary interactive reduction** on a coefficient-packed identity table `w`; it is not a standalone proof of knowledge. It gives an explicit baseline and avoids an unspecified “range certificate.”

Let `Z(x)` be the Boolean mask of reserved zero cells, with its multilinear extension computed from the certified interval/address layout. Let `Q_D(T) = product_{a in D}(T-a)`, with distinct field roots. Bind C, shape, domain D, and Z. Sample independent anchors `tau_R,tau_Z` and a scalar `eta` after this binding. Run sumcheck for

\[
\sum_{x\in\{0,1\}^{\mu}}
\left[\mathrm{eq}(\tau_R,x)Q_D(\widetilde w(x))
+\eta\,\mathrm{MLE}[Z(x)\mathrm{eq}(\tau_Z,x)](x)\widetilde w(x)\right]=0.
\tag{R}
\]

Use the **restricted equality's MLE**, not the off-cube product of the separate mask and equality MLEs. At final sumcheck point `p`, the prover supplies `y`; verify the sumcheck endpoint by substituting `y` for `widetilde w(p)`, yielding **one additional pending ordinary query `(C,p,y)`**. The prefix's degree per sumcheck variable is at most `|D|+1`. It proves no truth about y by itself.

For Boolean planes use `Q_{01}(T)=T(T-1)`, giving degree at most 3. For the receiver use the scheduled signed D_*. Independent anchors and eta permit extraction of range and zeros separately. Any extra segment-specific Boolean predicate must receive its own binding/combination charge or join a specified common predicate; no unpriced cancellation is allowed.

### 4.2 Reusing Akita's optimized signed range reduction

For signed even-base alphabets,

\[
Q_{D_*}(T)=\prod_{k=0}^{b_*/2-1}[T(T+1)-k(k+1)]
=Q_{sq}(T(T+1)).
\tag{S}
\]

Akita §6.1 reduces the anchored range sum to one virtual claim

\[
s_{claim}=\sum_x\mathrm{eq}(r_{virt},x)w(x)(w(x)+1),
\tag{V}
\]

without committing a separate virtual table. Bind (V) to the original w using a degree-3 sumcheck, with the zero term in (R) if desired. Its final w evaluation is a query on **the original C**, not on a new small recommitment. `widetilde s(r)` is generally **not** `widetilde w(r)(widetilde w(r)+1)`; multiplication occurs at Boolean indices before interpolation.

This uses native range/product-tree algebra, but relocates it before a new weak fold and adds its output query to that fold. That relocation and its new transcript composition require §8's lemma. Existing post-fold Eq. (160) is not silently reused with a different commitment argument. Boolean {0,1} narrowing is a separate `w(w-1)` identity, not the existing compression `w(w+1)` identity.

### 4.3 Conditional backward derivation

Assume the downstream native reduction has authenticated **all** added `(C,p,y)` claims for all children of a complete prefix sumcheck tree, and has recovered one unique source `s` under C. For an identity table, (ID) gives a unique `w = Dec(s)`. Substitute its true endpoint values into the sumcheck tree. Backward interpolation gives (R) exactly for every anchor child; varying eta separates its two sums. A nested binary tree in each anchor coordinate then gives every Boolean residual:

\[
Q_D(w(x))=0,\qquad Z(x)w(x)=0.
\]

Distinct roots give `w(x) in D`, and the masked cells are zero. The identity source map therefore gives

\[
\|s_i\|_{\infty,coef}\leq b_*/2,
\qquad \|s_i\|_{2,coef}^2\leq m_A d_A(b_*/2)^2
\tag{B}
\]

for each old receiver block, with no inverse-challenge factor. Noncanonical A-image/F digits remain allowed under their own certified alphabets and original equations.

The pending old query `(C,r,v)` must **also** be included downstream. Certifying the range query does not imply `widetilde w(r)=v`. This derivation is conditional on authenticated downstream witnesses, not on prefix verification alone.

## 5. Fresh onehot, dense, field-inline, and the bitplane alternative

### 5.1 Existing original handles

| Fresh source | What ordinary same-C queries observe | Sufficient bounded-source mechanism | What cannot be inferred |
|---|---|---|---|
| Onehot, coefficient-packed, delta_com=1 | Complete table, including physical cells if (ID) holds | Boolean prefix, padding zeros; exact per-chunk weight check if occupancy sizing is used | Honest sparsity, or onehot semantics from only a shape tag |
| Dense small digits, coefficient-packed identity | Complete digit table | Signed range prefix and zeros at original C | Concentration of recursive/honest digits on all valid witnesses |
| Dense full-field with delta_com>1 | Gadget recomposition Gs | A new source-observation reduction (§5.2), or independently linked source certificate | Range/canonicality of source digits from bounded decoded f |
| Field-inline/full-width field source | Whatever its frozen source decoder specifies | Classify by that decoder: identity case above or decomposed case above | That “field-inline” implies bounded physical source or an existing bitplane API |
| Any evaluation-trace packed source | Transformed logical coordinates | Exact physical inverse/packing expansion certificate plus same-source predicate | Equality of logical digit bound and physical coefficient bound |

Akita Remark 4.1 makes onehot frontend guarantees conditional on the same PCS claims. Even for Boolean coefficient-packed sources, the uniform safe energy is the dense envelope `m_A d_A`; use Hamming weight only if the exact physical occupancy predicate is extracted for every valid input. Trace embedding requires its own operator/energy factor.

For full-field gadget sources, `G = (1,b,...)` is not injective. In a two-plane coordinate, `(b,-1)` is a nonzero recomposition kernel. Therefore decoded range alone cannot bound the digits of an arbitrary algebraic source. This does **not** exhibit two openings of the same C; A can distinguish the sources. It proves the absence of the needed implication `bounded Dec(s) => bounded s`. Replacing s by canonical `G^{-1}(Gs)` can change As, inner-image digits, outer B/F path, and C.

### 5.2 Exact missing source-observation interface for retained gadget handles

Introduce **only as a new relation**, not an existing Akita API:

\[
R_{srcqry}(C,a,y;s,d):=
\mathrm{Rel}_{com}^{Akita}(C,\mathrm{Dec}(s),d)
\land y=\sum_i a_i^\top\mathrm{Coef}(s_i).
\tag{SQ}
\]

The coefficient flattening covers every digit plane and all physical cells. A source range prefix ends in such queries. To absorb them into a native fold using the original A/B/F handle, the corresponding response equation must use the requested linear functional on **s** rather than only the scheduled functional on `G s`:

\[
\sum_i c_i e_i=\mathcal L_{src,a}(z),\qquad z=\sum_i\iota(c_i)s_i.
\tag{SR}
\]

The maps must commute with the native challenge action, or use a proved carrier/trace lifting that does; arbitrary scalar coordinate projections need not be ring-linear. The A equation and original B/F chain remain unchanged. Scalar source-query rows, partial encodings, shared D/H widths, quotient rows, and matching subtraction inversions all need a new derivation. A fresh auxiliary source commitment is insufficient unless its equality to these original sources is itself reduced through these rows.

**SQ-Lift update:** [the retained-source query note](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md) now derives the direct-coefficient map, full raw physical view, descendant-certified extraction and mixed-profile source uniqueness for (SQ)/(SR), preserving original A/B/F. Independent mathematical review approved that conditional interface. The ordinary `L(Gz)` row alone remains insufficient; raw queries use a new carrier map while retaining response recomposition. Full-ring variants, repeated or shared-response channels, exact rows and costs are specified separately. This closes the unnamed algebraic gap, not numerical/profile, implementation or adaptive-chain admission.

### 5.3 Alternative: migrate NEW fresh producers to Boolean identity bitplanes

A useful **conditional source-encoding cutover** avoids SQ-Lift for producers that have not yet committed. For `L = 64` word values or `L = 128` field representatives, set `L = 2^ell` and let `B(j,a)` be the Boolean plane table, plane index j in explicitly little-endian bit order. Form its ORIGINAL native handle `C_B` from birth with delta_com=1 and full physical (ID). Define the frontend value table by

\[
f(a)=\sum_{j=0}^{L-1}2^j B(j,a)\quad\text{in }K.
\tag{BP}
\]

Run a Boolean/zero prefix at **C_B**. No hidden original field handle is replaced; the original handle in this new source profile is C_B. Every producer, frontend query, polynomial identifier, setup/schedule catalogue, hint owner, transcript declaration, prover, verifier, and final decider must be migrated consistently at the selected source revision. Old handles remain old handles and require their old interface or an explicit equality bridge.

**Source class is not sparsity.** Booleanity does not establish at-most-one-hot occupancy for a chunk of size greater than one. A dense Boolean producer must not use that sparsity-priced norm. A `UnitOneHot { source_chunk_size: 1 }` contract is different: it charges the whole Boolean grid and imposes only membership in $\{0,1\}$ on each coefficient. The compact implementation uses that contract for Main, with a restricted admission refinement for actual one-hot storage; see [the source-reuse implementation record](AKITA_INSTANCE_ACCUMULATION_EXTENSION_MIGRATION.md#13-original-source-reuse-and-exposed-migration-defects). Formation with delta_com=1 still requires the actual admitted source bounds, geometry, matrices, schedule and certificate catalogue. Merely selecting a label such as `DenseBounded65` does not establish that profile. The bitplane proposal is a producer migration at commitment formation, not a reinterpretation of any existing handle.

**Elementary MLE identity.** Let

\[
u_k=\frac{2^{2^k}}{1+2^{2^k}},\qquad
K_L=\prod_{k=0}^{\ell-1}(1+2^{2^k})=2^L-1.
\]

For the little-endian Boolean encoding of j,

\[
K_L\,\mathrm{eq}(u,j)=2^j,\qquad
\widetilde f(r)=K_L\,\widetilde B(u,r).
\tag{BE}
\]

Proof: `1-u_k = 1/(1+2^{2^k})`; multiplying the selected bit factors and cancelling denominators gives `product_k 2^{2^k j_k}=2^j`. The product identity telescopes from `(x-1)(x+1)=x^2-1` at x=2.

For Akita's displayed `q128 = 2^128-2^32+22537`, all factors for L<=128 are nonzero, as their positive integer values are at most `1+2^64 < q128`. Also `K_64 != 0`, and `K_128 mod q128 = 2^32-22538 = 4294944758 != 0`. These are algebraic nonzero checks **only**, not validated lattice/challenge/security parameters. Profile admission must certify the field and ordered axes; another modulus may not permit this point.

A frontend claim `f(r)=v` becomes `B(u,r)=v/K_L` under the original C_B. The bitplane range query generally has a **different point**. It and all frontend queries need repeated same-C occurrence groups or a separately proved multipoint reduction; no free multipoint opening is assumed.

For words, Boolean L=64 representation gives a genuine unsigned 64-bit value without modular ambiguity when q>2^64. Signed words require their specified sign interpretation. For field values, (BP) specifies only a field decoding: distinct Boolean integer representatives may decode to the same field element. This is complete for field algebra when canonicality is not required. If the application's bytes, comparison, or transcript semantics require the canonical integer representative `<q`, add and certify that comparison predicate; booleanity alone does not prove it. It is not silently imposed on a decider that admits noncanonical field encodings.

**Exact width accounting.** For a logical table of length n, the new Boolean table has L*n logical cells, plus physical/grid padding, and `mu_B = log2(n)+ell` when n is power-of-two. Its source coefficient count is that full physical count. Relative to an already decomposed original table with physical source count `delta_old * n_packed * d_A`, the ratio is the actual new count divided by that old count; at matching coefficient geometry it is approximately `L/delta_old`, not automatically an L-fold source-matrix cost. Relative to a one-digit n-cell word source it is L-fold logical width. Changed block counts, B slice widths, inner-image counts, response digits, shared D/H widths, compression completion, and receiver capacity must all be recomputed. Boolean planes need not be sparse; use dense energy bounds. No performance parity follows from (BE).

## 6. All padding and source-comparison accounting

### 6.1 Zero inventory

The profile compiler enumerates and binds, separately:

- source coefficient/ring completion and extra physical cells excluded by a logical decoder;
- within-final-block algebraic cells: zero only if the chosen domain explicitly requires it;
- natural-domain embedding holes required by the frontend;
- B slice padding **inside each polynomial segment**, not a single final suffix;
- F/H digit-plane ring completion;
- Eq. (109) compression-suffix start alignment, stage starts, suffix end alignment;
- successor ring completion and the full `N_*` Boolean-domain tail;
- virtual absent-block indices, whose zeros are built into the public extension and are not additional committed witness cells.

Zero constraints must be incorporated in a prefix/domain relation and proved under the corresponding original handle or supplied commitment chain. B slice padding is derived in each row. Compression completion cells have both their binary predicate and explicit zero predicate where required. Range alone allows nonzero padding. The receiver cannot discard unobserved physical cells and then invoke (ID).

The full successor is

\[
W'=\big\Vert_g[\hat z_g\mid\hat e_g\mid\hat t_g]
\Vert\hat Q_{main}
\Vert\big\Vert_{a=1}^{L_{comp}}[(\Vert_g\hat u_{g,a})\mid\hat v_a\mid\hat Q_a],
\tag{109*}
\]

with the scheduled inserted zeros and final capacity tail. Every quotient segment is empty only on the certified quotient-free route; every compression suffix is absent only at depth zero. Incoming compression digits belong here; the successor commitment's own F hints are retained outside W' until that commitment becomes an incoming handle. Putting those hints into their own commitment input would be circular.

### 6.2 Certified alphabets and mixed comparisons

For each source-compatible occurrence class, fix the same A view, source geometry, and B/F path. Compare sources only when the actual `(shape,C)` agrees. Include old weak extractions, range-prefix-associated occurrences, repeated-point occurrences, bounded certificate references if used, and setup reference occurrences where applicable. Original and receiver views that differ are not magically one comparison class; any proposed representation bridge needs its own explicit collision reduction.

The A radius is the maximum mixed envelope (U) over **all ordered occurrence pairs**, including the diagonal. A bounded reference with rho'=1 and source bound B0 gives mixed radius `B_infty + K1*B0`, but that reference exists only after actual source certification. It cannot be hypothesized to obtain binding.

For each B/F physical view, price the maximum cross-profile diameter of its **verifier-certified** complete input alphabets. Akita's carried image/partial digits may be honestly base-b1 digits, but a scanning common base b* permits diameter b*-1, not b1-1. F/H diameter one requires the full `{-1,0}` predicate; common range alone is insufficient. D/H are opening-local and do not belong to source-projection uniqueness.

Response digits recomposed in positional base b_z with depth delta_f, scanned in base b*, have

\[
\beta_{cert}=\frac{b_*}{2}\frac{b_z^{\delta_f}-1}{b_z-1},\qquad
\Delta_{cert}=(b_*-1)\frac{b_z^{\delta_f}-1}{b_z-1}.
\tag{E}
\]

These are the bounds used for weak certificates/collision sizing; the honest asymmetric interval in §9 is different. An original rank is frozen at commitment formation. A larger receiving range or comparison class cannot retroactively upgrade it. Shared setup uses overlapping coefficient-prefix views, not independent matrices; the exact `(q,d,n,m,p,eta)` inventory and finite union must be covered by the shared-prefix reduction.

## 7. Complete proposed transcript order and endpoint types

**Conditional design, ordinary coefficient-packing cut.** Each message/challenge has a distinct typed domain tag and includes an injective encoding of the complete preceding transcript. Actual native representation/tensor requirements remain profile-dependent.

1. **Freeze inputs:** absorb P, setup/version/catalogue identifiers, session/count/boundary data, old `(C,r,v)`, all ORIGINAL fresh `(shape,C_g)` and frontend queries, source/bitplane maps, and zero/domain masks. All producer commitments precede challenges dependent on them. No post-challenge alphabet or capacity change.
2. **Frontend:** execute the segment's genuine frontend/lookup/memory/linkage reductions in their required order, retaining their opening claims rather than finishing segment PCS proofs. Bitplane substitution (BE), if selected, is part of this frontend interface. Onehot occupancy/canonical representation checks cannot be omitted if used by its domain.
3. **Same-C strong prefixes:** run (R), or (S)/(V), on the old receiver and admitted identity fresh sources. Bind their intermediate/frontier values before challenges that batch them. Each scalar sumcheck round absorbs its polynomial before its challenge. Derive `(C_g,p_g,y_g)` query outputs; absorb their values before downstream batching/folding. Do not mark them authenticated.
4. **Opening representation:** collect all old pending, fresh frontend, and prefix endpoint queries. Group compatible points/shapes or use explicitly priced repeated same-C occurrence groups. If evaluation-trace receiving is selected, bind tensor column partials, sample its shared tensor anchors/batching, run tensor sumcheck, bind forwarded claims/factors, and reject zero factors in the paper's order. Direct packing with its canonical identity identification has no tensor prefix.
5. **Shared payload before folding:** from supplied sources/chains prepare true partials and their digits. Absorb the complete shared D/H payload. Sample the scalar claim-combination coefficients in the selected native order (after this payload for the proposed extra-claim packing composition as well). Bind combination values/weights. These are public linear combinations, not additional source commitments.
6. **Native weak fold:** sample the joint independently filtered group challenge tuple with the prescribed nonce budget. Form `z_g = sum_{c,i} iota_g(c_{g,c,i}) s_{g,c,i}`. Prepare its complete (109*) witness, all main/stage quotients if required, and exact padding. Absorb successor `(shape_*,C')` **before** alpha and range/row challenges. Outgoing commitments may differ across fold children.
7. **Strong suffix:** execute ring reduction, complete successor range/binary/zero predicates, optional exact response norm protocol, row batching, and fused relation sumchecks. Scalar opening rows include every query retained in step 4. Setup offloading, if selected, is a separately registered/priced endpoint; the simplest cut evaluates setup weights directly and creates no setup debt. Derive one ordinary pending evaluation `(C',r',v')` on the already-bound successor.
8. **Return:** output `PendingBoundedCandidate(P,a')` with native hints and application-link evidence. This denotes a claimed bounded successor whose truth is deferred to the accumulator decider/reduction; it is not `VerifiedOpening`, and no C/source replacement is permitted.
9. **Final discharge once:** check the chosen final (A) certificate plus all required history/execution acceptance evidence. If using a full native terminal proof instead of revelation, its canonical inner-state transition and a separate bounded-source/domain extraction theorem must match the same receiver handle; terminal response norms alone do not establish (A).

No range prefix contains a completed independent proof for `(C,r,v)`. Its new output queries are consumed along with that old claim. Revealing the entire `(w,d)` and checking the evaluation every update is a fully checked refresh baseline, not this design.

The exact native ring identities retained in step 7 include

\[
A_g z_g=\sum_{c,i}\iota_g(c_{g,c,i})G_{b_{1,g}}\hat t_{g,c,i},\qquad
\mathcal L_g(G_g z_g)=\sum_{c,i}c_{g,c,i}e_{g,c,i},
\]

all original sliced B/F rows (C), the shared D/H rows, and the scalar opening combination of partials. For quotient lifting, each native row is checked as

\[
\sum_j A_{r,j}(X)W_{r,j}(X)-Y_r(X)=(X^{d_r}+1)Q_r(X),
\qquad\deg Q_r<d_r,
\]

after committing the quotient digits and before sampling alpha. The quotient-free route instead evaluates the actual negacyclic residual through its transpose-convolution weights. Substituting alpha into an unlifted ring product is invalid. Ordinary ring/scalar algebra is source-backed; admission of the new repeated-source/pre-range streaming schedule is not.

## 8. Derived conditional local bridge and the noncircular proof cut

**Derived conditional Lemma BR-Identity (local, descendant-certified; not an already admitted Akita schedule).** Fix a finite supplied list of direct-coefficient identity-source groups, including migrated Boolean planes and repeated same-original-C query occurrences. Freeze the native dimensions, maps, rows, original source projections, and transcript of §7. Assume full physical injectivity/zeros, every local matrix/unit/certified-alphabet hypothesis and mixed source comparison from §6, and:

- a prefix-consistent mixed accepting tree for every range/zero/frontend prefix;
- under each prefix leaf, the complete coordinate-wise fold family and all its ring/range/norm/relation trees;
- actual authenticated outgoing `(C',r',v')` sources at every strong-suffix leaf;
- samplers with certified cardinality, unit differences, and conditional extraction support;

then the following direct generalization of Akita's local proof gives a deterministic extractor, polynomial in the **supplied tree size**, returning `(w,d,xi)` satisfying (A) and (F), or a nonzero short collision in the declared physical matrix inventory.

**Conditional derivation, with every authentication cut explicit.**

1. **Authenticate outgoing leaves first.** The hypothesis supplies actual authenticated outgoing sources, not merely accepting terminal residuals. For each fixed fold challenge, apply the existing strong-check extraction arguments upward through its full supplied subtree. Outgoing same-C source uniqueness makes each strong suffix refer to one source. Range/norm/fused extraction and the selected ring extractor recover all native commitment, chain, partial-evaluation, and opening rows exactly. Later challenges need not match between folding children; their recovered native identities no longer depend on those challenges. This is Akita lines 3971–3973.
2. **Extract each fixed incoming group by coordinate subtraction.** Reconcile shared commitment/opening paths with the existing chain comparison argument. For challenge coordinate `(j,c,i)`, subtract its two exact child relations. All other terms cancel, leaving `iota_j(delta_c) s_{j,c,i} = z_j^1-z_j^0`. The prescribed unit difference permits algebraic inversion in the ambient ring and the scheduled opening ring. Subtracting the remaining exact rows gives `A_j s = G_{b1,j} hat(t)` and the group's exact partial/scalar evaluation equations. Certified response-difference bounds give the weak bound on `iota_j(delta_c) s`, **not** a bound on s after inversion. Repeat for the frozen finite list, including old evaluation and fresh prefix/frontend occurrences. These are the steps of Akita lines 3975–3987; neither the application/root designation nor absolute recursion depth occurs in this algebra.
3. **Identify original sources across prefix leaves.** Every extracted weak opening for a repeated occurrence or different prefix leaf has the same original commitment-side projection fixed before the prefix challenges. Apply Corollary 10.17 to the pairs, using the complete certified mixed-class radii and matrix inventory. Either obtain a short nonzero collision or identify one common Src, including its original semantic source and source-side commitment-chain data. Points, evaluations, shared D/H paths, and later fold challenges are not asserted equal. Thus each extracted range endpoint is an evaluation of this **same original source**; each old/fresh evaluation equation is retained rather than assumed.
4. **Interpolate the strong prefix with that now-common source.** Full physical identity/injectivity makes `Dec(s)` the source table w, including all zero-completion cells; there are no unobserved physical coordinates. Each leaf's authenticated endpoint value can therefore be replaced by `w_tilde(p_leaf)` in the strong sumcheck terminal equation. Recover its complete polynomial identity upward through the required `(b+2)`-ary round trees. The two-child eta separation and complete nested binary-anchor trees in §4 recover the range and zero identities separately. Multilinear interpolation in the anchor coordinates forces `Q_D(w(x))=0` at every Boolean x and `Z(x)w(x)=0`. The declared injective root alphabet gives `w(x) in D_*`, with exact zeros at required cells. Fresh Boolean prefixes analogously force `B(j,a)(B(j,a)-1)=0`; any extra frontend occupancy/application predicates require their own authenticated rows/trees, not an honest-input assumption.
5. **Return the bounded original witness, not a replacement.** Together, the recovered original commitment-chain equations, exact pending/frontend evaluations, range and zero constraints, and explicit frontend relations satisfy (A)/(F). For bitplanes the elementary (BE) identity transports field/word evaluations without changing the original handle. Keep a supplied valid noncanonical decommitment path; no canonical recommitment or independent opening proof is performed. The old pending evaluation becomes true only as a conclusion of this extraction. All source/chain disagreements encountered above are accounted for by the declared SIS exits.

This is a **derived conditional local generalization**, not an identified fundamental new cryptographic obstruction or a claim that the paper already admits the streaming schedule. Akita lines 3198–3200 state the root-only analyzed interface and explicitly separate schedule restrictions from intrinsic identities; lines 3951–3990 supply the local descendant-certified proof. The remaining local deliverable is the exact instantiated source/query/maps/row specification, full physical observation, source-comparison and sampler certificates, all-valid-input closure, and admitted topology/security accounting. The finite stateful ROM experiment and production of its authenticated descendants remain separate obligations below. For retained decomposed fresh handles, the linked **SQ-Lift/BR-Source** construction now supplies the conditional raw-source observation algebra; it still needs those concrete certificates. Decoded range alone remains insufficient.

**Two distinct chain obligations; no unlimited-depth requirement.**

1. **Missing bounded-chain certificate BR-Bounded(`T_max`).** A finite authenticated maximum depth is a legitimate construction target. Specify and certify this old-plus-fresh topology and all hypotheses of the derived local BR-Identity (plus BR-Source where needed), the complete final bounded decision, and an auxiliary-input/stateful extraction procedure obtaining actual authenticated descendants for every required counterfactual leaf for every accepted chain of depth `T <= T_max`. A bounded unrolled tree may supply those descendants. Its full branching products, repeated-query occurrences, witness/transcript/oracle work, rewinding support, failure probabilities, SIS exits, query/nonce caps, and concrete runtime must be charged at `T_max`; the resulting concrete security/runtime certificate must satisfy the chosen admission policy. Fixed state size alone is not that certificate. Authenticating `T_max` and enforcing it in the relation/decider is required; an informal intended segment limit is not sufficient.
2. **Separate stronger missing Lemma BR-Adaptive.** For a target allowing growing T, supply an auxiliary-input/stateful extractor interface obtaining the required authenticated descendants for adaptive accepted steps with uniform polynomial total cost in the declared input/security parameters, including all oracle work, and exposing original-source uniqueness at this exact projection. This is a stronger research target, **not mandatory for every finite native construction and not a user demand for unlimited depth**. A proof that recursively invokes the suffix extractor once per fold/prefix child must charge the product of branching factors, which may be exponential in T despite fixed-size states.

Under either target a final endpoint from one realized chain is insufficient by itself; the claimed procedure must actually authenticate the counterfactual leaves. A completed proof for every segment is not silently introduced to obtain them.

This makes the proof noncircular: no statement that “old C is a valid digit opening because it is the accumulator” is used. Old range and old evaluation are both conclusions obtained through actual endpoint extraction. It also identifies precisely what range restoration **does** solve and what it **does not** solve.

**SuperNeo applicability check.** A pre-range reduction with no new source commitments can preserve `phi = ordered(original shapes,C_g)` across its reruns; endpoint points may vary. The downstream reduction must be weak for that same phi and supply adaptive uniqueness of the source projection. Complete weak openings include point-dependent data and are not the unique witness object; use a projected relation/interface which returns the common source separately. Once those definitions and extraction experiments match, sequential strong-then-weak composition is the relevant theorem. Merely moving range messages ahead of Akita's weak fold is not a proof: Akita's ordinary strong suffix remains nested, outgoing C' varies with fold challenges, and descendant authentication/runtime still must be established.

## 9. Strong completeness for EVERY decider-valid input

Completeness is quantified over every `(a,w,d)` satisfying (A) and every `(u,xi)` satisfying (F), not only states generated by a prior honest execution. It does not assume their digits are independent/uniform, their decommitments canonical, their source occupied sparsely unless (F) requires that, or their padding honest beyond explicit zeros.

**Elementary conditional derivation.** Use the supplied original source/chains, not Commit to repair them. True sources imply true partials and scalar evaluation rows. Range/zero prefixes have exact true Boolean residuals and correct endpoint values. The native fold equations follow by linearity. For each response g, if all accepted challenges satisfy `||iota(c_a)||_1 <= K_{g,a}` and admitted physical source bounds are `B_{g,a}`, then

\[
\|z_g\|_\infty\leq U_g:=\sum_a K_{g,a}B_{g,a}.
\tag{HC}
\]

With operator bounds Gamma and source energies E,

\[
\|z_g\|_2^2\leq\left(\sum_a\Gamma_{g,a}\sqrt{E_{g,a}}\right)^2.
\]

These inequalities hold for every valid input and every successfully sampled accepted challenge. They are the deterministic envelope argument of Corollary G.6 generalized to the explicitly larger input domain, not an import of its honest-history qualifier.

For positional even base b_z>2 and depth delta_f, choose the canonical interval

\[
[-M_f,T_f],\quad M_f=\frac{b_z}{2}\frac{b_z^{\delta_f}-1}{b_z-1},\quad
T_f=(b_z/2-1)\frac{b_z^{\delta_f}-1}{b_z-1},
\]

with **both** `M_f >= U_g` and `T_f >= U_g`, or a tighter proved signed envelope. Accepted alphabet b* must contain the honest digits, but (E) cannot justify canonical encodability. Base two's {-1,0} has zero positive reach and is not a symmetric positive-response encoding; Akita's negative-binary field compression is a different full-field lifting rule. Digit depths for partials/images/quotients must cover every field value or an all-valid-input proved smaller envelope.

Assemble (109*) at fixed N_* for every admitted input shape. Construct a fresh successor commitment to that newly formed witness; this is allowed because it is the outgoing handle, not a replacement for an incoming one. Honest source/chain completion zeros and every explicit reserved zero satisfy the successor domain. If receiver capacity, asymmetric encoding, exact norm bounds, field decomposition, challenge samplers, and optional tensor nonzero-factor conditions are all certified, the resulting successor satisfies (A). The only abort probabilities are the specified bounded sampler/nonce failures and any tensor-factor rejection; all must be bounded uniformly over arbitrary valid inputs. With direct packing and deterministic successful challenges there is no tensor-factor abort. No unproved concentration model is used.

**Scope restriction is explicit.** A decider accepting unrestricted Definition 10.13/10.14 sources instead of (A)/(F) does not satisfy these hypotheses. Tightening admission to bounded sources is a genuine new input contract. A final ordinary PCS proof with weak algebraic extraction does not make that contract all-valid-state complete. Migrated bitplane producers are complete for the explicitly selected word/field-decoding relation; legacy handles are not silently included.

## 10. Exact cost/certificate ledger

This design has no validated numeric profile. The following are required symbolic inventories, not a claim of practical constant cost or performance parity.

### 10.1 Prefix and query cost

For the simple (R) prefix on n=2^mu cells and alphabet D of size b, the summand has individual degree at most b+1. Standard compressed sumcheck sends **(b+1)*mu extension-field elements**, plus one endpoint y, because one round coefficient is implied by `g(0)+g(1)=claim`. Boolean range/zero fusion uses 3*mu such elements plus y. Anchors/scalars are public challenges rather than proof payload, but their FS query/entropy/encoding work is counted. Straight-line predicate evaluation is O(b*n); prover folding/polynomial work must additionally be charged, including the chosen sumcheck algorithm's dependence on b. Fixed-profile small b permits linear-in-n work with profile-dependent constants; it is not sublinear in the table size.

For the optimized signed range tree, use Akita's exact stage degrees/frontiers. Equality-factored stage of inner degree d_i sends d_i*mu elements; the final degree-3 binding sends 3*mu elements; add every frontier value and the carried virtual claim. Zero fusion does not increase degree beyond 3 at that binding. Norm fusion changes the selected message format and adds its exact claims/binding costs. Do not count a range proof as zero cost just because its helpers are virtual.

For each original handle h let q_h be the number of **distinct point-contraction occurrences**, including old pending/frontend queries and new prefix endpoints, after only proved deduplication. Native folding has `W_fold = sum_h q_h*P_h*B_h` challenge coordinates when each occurrence opens P_h polynomial claims at one compatible point. Different-point groups repeat source state/hints; no free simultaneous-point API is assumed. Their partial coefficients, response vectors, original B/F rows/digits, main quotients, and shared unsliced D/H width must enter the full successor compiler. Compression can be shared/deduplicated only under a proved relation/address scheme; a cost baseline duplicates occurrence paths and then prices them.

Bitplane migration adds ell=6/7 logical variables for L=64/128; its Boolean prefix scans the full L*n grid and its frontend point becomes `(u,r)`. Its source coefficient width is §5.3's exact ratio. More variables, blocks, and query occurrences enlarge both transcript trees and receiver capacity even though the value recomposition needs only one transformed query.

### 10.2 Shape and matrix certificates

Emit an integer shape certificate counting every source block, response digit, partial/image digit, main quotient plane, F/H stage digit with ring completion, stage quotient, alignment cell, and final tail. Verify `|W'_physical| <= N_*` for every old-layout/fresh-shape combination, and fixed retained hint/row/setup capacity. Added range-prefix helpers are transient unless the chosen alternative explicitly retains them; repeatedly appending LF+ range matrices is not permitted as an uncounted workaround.

List all matrix instances at `(q,d,n,m,p,eta)` with mixed comparison radii and certified alphabets (§6), including canonical/bounded reference occurrences actually used, all D/H local maps, all original frozen views, and any SQ-Lift auxiliary views. Frozen original ranks must support the mixed certificate at formation time. Public setup continuation is the longest needed prefix, but its collision reduction inventories every interpretation, not a single maximum view. Charge direct public matrix scans, structured weights, compression matrices, prover matrix products, source/bitplane materialization, and retained native hint bytes.

### 10.3 Error and extraction certificates

For the simple fused (R), a sufficient complete mixed extraction tree has separate nested binary anchors, a two-child eta separation, and scalar sumcheck trees with arity `(b+2)` per round. Its factor is at most

\[
2^{2\mu}\cdot2\cdot(b+2)^\mu,
\]

before the downstream fold/suffix factors; redundant terms can be removed only by a proved optimized construction. Its corresponding interpolation charge is `(2*mu + 1 + (b+1)*mu)/|E|`, before downstream errors. If the zero mask is empty, omit that anchor/eta/term explicitly. These are conservative **tree accounting derivations**, not certified security levels.

The optimized native range tree replaces that factor with its actual stage/frontier arities and adds the `2^mu` range-anchor factor. The native fold contributes `1+W_fold`, ring reduction contributes `2*d_max` (lifted) or `d_max` (quotient-free), row batching contributes `2^{ceil(log2 n_rel)}`, and a degree-3 suffix contributes `4^{mu_*}`. Count all tensor, batching, norm, setup, and terminal factors. For BR-Bounded(`T_max`), explicitly unroll the complete tree through the authenticated finite depth and certify the actual concrete runtime, total extractor/oracle queries, and security loss at that depth; do not suppress its branching product. For the stronger BR-Adaptive target require uniform polynomial total cost in the declared growing-T model. Multiplying local factors at every VM segment without these charges proves neither target; failure of the stronger uniform target does not rule out a separately admitted finite bounded construction.

Certify challenge-family cardinality, unit differences in each native ring and embedded carrier, all inverse/interpolation uses, fixed witness-independent filter acceptance bounds, exact conditional oracle sampling, nonce/query caps, full classical-FS error, and shared-setup bias. Neither q128 arithmetic nor quantum lattice-attack sizing supplies a QROM theorem. No production/security claim is made from the algebraic examples above.

## 11. Imported theorem audit: useful mechanisms, not substitutions

**Lova [L].** Its exact relation carries an integer Gram matrix `D = S^T S` and bounded diagonals. Decomposition supplies `G^T D_tilde G = D` and `T_tilde G = T mod q`; ternary challenges and its SIS uniqueness argument establish the extracted Gram relation, removing extraction norm slack. This supports the lesson that norm restoration must concern the extracted **same witness**, not only honest forward digits. Importing it requires its unstructured integer commitment, integer equality/Euclidean relation, ternary extraction argument, and modulus/parameter hypotheses. Its visible Lemma 4 additionally requires `t <= beta / (2*k*floor(b/2)*sqrt(m))` for perfect completeness; the forward bound is not automatic. A Gram equality modulo Akita's q is not the integer equality without a no-wrap reconstruction certificate. No Lova theorem is applied directly to nested A/B/F Akita handles.

**LatticeFold+ [LF].** The range theorem retains the **same linear commitment to f**, plus monomial/double-commitment helpers and output evaluation claims. Its openings already permit `a=a'/s`; its double-commitment `pow` is not injective, and helper digits need not be canonical. §4.3's Theorem 4.2/lemmas are a model for a genuine range reduction conditional on valid output witnesses, not evidence that a standalone prefix authenticates its endpoint. §5 Theorem 5.1 requires `B=(d/2)^k`, a large specified folding set, `(2B^2,Sbar-Sbar)` binding, and inequality (28). §5.2 returns **two** bounded linear instances and recombines their commitments/evaluations. Akita's two-tier nonlinear digit pipeline cannot replace that linear commitment identity without a new original-handle consistency reduction. Range-helper transformation is essential to avoid repeatedly flattening dk-expanded helpers into the accumulator. None of these matching hypotheses has been established for this Akita receiver.

**SuperNeo [N].** Definitions 16–18 and Theorem 12 require sequential strong-then-weak reductions with the **same phi**, adaptive uniqueness over fresh independent extractor invocations, and runtime including all oracle computation. §7.5 reduces to a fixed k-tuple using signed decomposition and recombines native commitments plus both flattened norm and relation evaluations. This is not a theorem for one Akita ordinary opening with different original maps. §8 identifies the exact proposed projection and remaining nesting/runtime mismatch.

**SALSAA [S].** Lemma 4.8 restores norm from a sumcheck relation on the same bounded extracted witness using the trace of its conjugate inner product. Its proof lifts equality from mod q using the pre-existing extracted bound beta'; Setting 3.4 requires `4 beta'^2 < q`, explicit polynomial witness height, and superpolynomial residue field. It cannot be applied to an Akita inverse-unit source of unbounded norm merely because the honest source is small. Theorem 7.3's fixed accumulator uses `r_acc=2*ell`, tracks all extraction SIS exits, and §7.3 retires bottom rows through fresh batching each update. Its constant structural dimension complexity is not an Akita performance bound. To import it, map Akita's original commitment projection, norm convention, tensor/linear rows, collision assumptions, and no-wrap premises exactly; structured variants may require vSIS rather than Akita's standard Module-SIS inventory.

## 12. Evidence, read scope, and handoff

### Live qmd paper evidence

The following indexed sections were retrieved through the mounted live qmd MCP in this assignment. Local links identify the parsed text; primary links identify the paper, **not an assertion that an online revision was re-read or matches the indexed revision**.

- **[A] Akita:** [local](SOURCE_PROVENANCE.md#akita), [primary ePrint 2026/1983](https://eprint.iacr.org/2026/1983). Actually read: §4.1 lines 1555–1590; §4.3 1645–1683; §4.5 1825–2025; §5.4 2307–2377; §6.1 and beginning §6.2 2398–2509; native rows/ring identities 2722–2852; quotient-free theorem/fused endpoint/message costs 2918–2971, 2974–2987; endpoint/cost/terminal boundary 2990–3046; §9.3 3194–3248; §10.2 source-comparison inventory/Defs. 10.13–10.14/uniqueness 3603–3744 and 3745–3826; Theorem 10.23 3951–3990; strong-check/interleaved-composition accounting 3991–4145; Remark 10.32 and FS scope 4190–4252; Corollary G.6/G.7 6133–6216. The broad 3603–3992 response elided its middle; needed 3745–3826 was independently re-read. Embedded images at other selected ranges were not used. Claims do not rely on elided text. An unrelated 1380–1519 retrieval was visible but is not the basis for any new source-encoding assertion.
- **[L] Lova:** [local](SOURCE_PROVENANCE.md#lova), [primary ePrint 2024/1964](https://eprint.iacr.org/2024/1964). Actually read §1.2.5 lines 173–237; §2.3/§3.1 readable text 279–317, 321–366; §3.2 readable text 367–379, 383–393. Large embedded images in the latter retrievals were elided; no claim relies on their contents or on an unread full formal extractor proof. The integer Gram derivation cited here is the visible technical overview, and the forward completeness condition is the visible Lemma 4 statement.
- **[LF] LatticeFold+:** [local](SOURCE_PROVENANCE.md#latticefoldplus), [primary ePrint 2025/247](https://eprint.iacr.org/2025/247). Actually read §4.1 lines 332–379; §4.3 478–634; §4.4 introduction/helper-growth warning and beginning of Construction 4.5 634–696; §5 1018–1041; §5.2 1164–1193. The repeated dk helper blowup is explicit at line 646. The rest of the commitment-transformation theorem/proof was not read; no drop-in transformation claim is made.
- **[N] SuperNeo:** [local](SOURCE_PROVENANCE.md#superneo), [primary ePrint 2026/242](https://eprint.iacr.org/2026/242). Actually read §6 lines 855–936 and §7.5 1117–1154. The deferred Appendix B proofs were not read; theorem statements and hypotheses, not a newly verified full composition proof, are used.
- **[S] SALSAA:** [local](SOURCE_PROVENANCE.md#salsaa), [primary ePrint 2025/2124](https://eprint.iacr.org/2025/2124). Actually read Setting 3.4 lines 248–258; §4.4 529–586; §§7.2–7.3 1297–1392. Use its explicit beta/beta' and fixed-row accounting, not a claimed Akita-compatible black box.

### Local/source boundary

Read existing obligation 02's exact relation at lines 98–188; searched obligation 01–04 for source/padding/composition interfaces and cross-profile radii. These documents are local conditional designs. One implementation fact was read from the **current local tree**, whose declared Akita revision is `2607265b21f4726954461869f75acc2a8ab6ada1`: `akita/crates/akita-params/src/sis/committed_source.rs`, `CommittedSourceClass`, lines 37–53, for balanced-magnitude versus per-chunk-one-hot class separation only. This is not independently retrieved immutable remote evidence and does not establish admission of a new bitplane profile. No old genuine outer-recursion API, modern native-pair API, or other current-tree catalogue is conflated with it. `field-inline` here is a source category to classify from the selected producer decoder, not an asserted pinned implementation decoder. Source selection and complete caller/catalogue migration remain the integration owner's decisions.

**Deliverable outcome.** The identity-source range restoration, Boolean-plane recomposition and linked retained-handle SQ-Lift are explicit conditional algebra. BR-Identity generalizes the native one-fold proof under a supplied descendant tree and exact local hypotheses; it is not an already admitted streaming schedule. Still required are complete application-specific interfaces, full-observation and all-valid-input closure, mixed-source/security certificates, and the chosen chain experiment: BR-Bounded(`T_max`) for a finite authenticated construction, or the stronger BR-Adaptive for a uniform growing-T target. The generic independently reviewed receiver calculations do not instantiate Jolt. The selected interface remains a same-original-handle strong prefix whose endpoints are folded, not a completed segment proof or a replacement commitment.

No source files, branches, commits, builds, tests, linters, formatters, or benchmarks were changed/run. Independent review should check the source-observation cut, repeated-point grouping, exact transcript/tree cost, all-valid-domain quantifiers, bitplane producer/byte semantics, and counterfactual descendant authentication before implementation or verification work.
