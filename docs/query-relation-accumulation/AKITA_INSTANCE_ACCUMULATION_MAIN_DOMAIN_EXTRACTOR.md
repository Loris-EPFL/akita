# Main domain extraction from the existing lattice frontend

Status: exact physical coverage and conditional strong/weak composition independently reviewed. This is the obligation for removing the accumulation-only Main Boolean prefix, not permission to remove it. Both native parties retain that prefix; the complete two-frame public native certificate has now executed. The [supplied native source-tree theorem](AKITA_INSTANCE_ACCUMULATION_NATIVE_SOURCE_TREE.md) derives bounded Main extraction with the prefix retained, but does not provide the concrete compiler/tree-building or numerical admission needed for its removal.

## 1. Exact source, not a reconstructed commitment

The selected original Main source has 56 columns of 65,536 direct scale-one, one-digit K coefficients, with native raw view `(degree,polynomials,blocks,positions,digits)=(128,56,1,512,1)`. Its physical map is
\[
\operatorname{index}(p,x,j)=p\cdot65536+128x+j,
\quad t=\left\lfloor(128x+j)/16\right\rfloor,
\quad a=(128x+j)\bmod16.
\]
No physical coefficient is discarded. Six selector bits inject this source into a 64-column raw table; the extra eight columns are virtual zeros, not physical padding that a malicious source can fill.

| Physical columns | Original family | Omitted address zero |
|---|---|---|
| 0–31 | Instruction RA | Yes |
| 32–47 | Balanced increment digits | Yes |
| 48 | Balanced increment carry | Yes |
| 49–51 | Bytecode RA | Yes |
| 52–55 | RAM RA | No |

There are 3,670,016 actual coefficients and 524,288 virtual selector cells. For active length \(\ell\), the disjoint domain partition is
\[
\underbrace{52\ell+896(4096-\ell)}_{\text{required physical zeros}}
+\underbrace{844\ell}_{\text{remaining Boolean cells}}
=56\cdot4096\cdot16.
\]
The arithmetic was checked separately. The coverage review also inspected the actual column order, source geometry and coordinate map.

## 2. What the frontend already proves, and what it cannot see

For an omitted-zero family, the semantic table is
\[
R_p(a,t)=\begin{cases}X_p(a,t)&a\ne0,\\1-\sum_{a'\ne0}X_p(a',t)&a=0.\end{cases}
\]
RAM uses its complete physical table. The lattice Booleanity address/cycle argument covers every semantic column on the whole 16-by-4096 cube:
\[
\operatorname{eq}(\tau,x)\sum_{p=0}^{55}\gamma^{2p}(R_p(x)^2-R_p(x)).
\]
After source-linked extraction, it proves every nonzero-address physical entry Boolean in omitted families, and every RAM address Boolean. It does not observe \(X_p(0,t)\) in an omitted family: setting that physical cell to 2 leaves the reconstructed semantic table unchanged. Honest sparse construction is not a proof against that attack.

The new physical-zero predicate is
\[
Z_p(t,a)=\mathbf1_{t\ge\ell}\lor(O_p\land\mathbf1_{a=0}),
\]
where \(O_p\) is immutable and true exactly for columns 0–51. Its source-linked sumcheck tests
\[
0=\sum_x\sum_p\alpha_p\widetilde{[\operatorname{eq}(\tau,\cdot)Z_p]}(x)\widetilde X_p(x).
\]
The public endpoint weight is the MLE of the restricted Boolean table, not a product of separately interpolated masks. With \(S_t\) the equality-weighted inactive suffix,
\[
W_p(r)=S_tE_a+O_p(E_t-S_t)A_0.
\]
This supplies exactly the missing address-zero cells and all inactive cells. Complete Booleanity plus this predicate yields coefficient alphabet \(\{0,1\}\) on the entire original physical Main source.

The signed digit/carry semantic decoder still uses the full centered alphabet \(\{-8,\ldots,7\}\). Do not infer the smaller honest carry set \(\{-1,0,1\}\). First/last PC moments certify execution boundaries; they are not a substitute for the Boolean source-domain argument.

## 3. The circular proof to avoid

Do not argue:

1. bounded commitment binding fixes Main;
2. frontend Booleanity proves Main bounded;
3. therefore bounded binding applies.

Also do not divide a bounded response by a short challenge difference and claim that the inverse preserves its norm. Native challenges live in a cyclotomic ring with the certified unit-difference law; that ring is not an arbitrary scalar field.

Use two explicit relations on the same original source projection:

- \(R_{\rm relaxed}\): original K source, original A/B/F decommitment equations and path, decoder and endpoint equations; source may initially be long. Weak extraction retains unit factors \(\rho\) and bounded products \(r=\rho S\).
- \(R_{\rm bounded}\): the same relation plus every reconstructed frontend identity, alias, physical zero and \(S[x]\in\{0,1\}\).

The public projection is the original ordered commitment handles with complete shapes/payloads. It is not a later accumulator commitment, endpoint, random normalization weight or D/H path.

## 4. Noncircular backward extraction

**[INFERENCE — conditional construction from the cited paper interfaces.]**

1. Start from the complete terminal/decider accepting subtree.
2. Extract each descendant weak K source and its bounded-product/path certificates.
3. Compare repeated extractions of the same original source. Return the same source projection or an inventoried short matrix collision.
4. Extract the original native root source under every required frontend/reference/normalization branch, without assuming that Main is already short.
5. Reverse the common-K normalizer against that source projection to recover all 118 original Main leaves.
6. Reverse stage-seven reconstruction/aliases and the complete Booleanity address/cycle trees, including their stage-five reference ancestors.
7. Separate the physical-zero coefficient and anchor families on that same source.
8. Conclude \(R_{\rm bounded}\) and Main coefficient bound one.

Forward execution and backward extraction have opposite order. This is consistent with Akita's strong/weak invariant. Across an internal folding challenge the successor can differ: extract separately below each folding child, then compare incoming sources. Do not apply a sequential strong/weak theorem across that cut as if the successor were fixed.

The common-K normalizer's 56 terminal column disclosures authenticate only their raw aggregate on accumulation. Individual disclosures are not new authenticated source witnesses. Source-projection uniqueness, not uniqueness of those redundant message values, is the required hypothesis. No cancellation of a possibly zero public terminal weight is used.

## 5. Keep the actual weak-binding envelopes

For two successful weak extractions,
\[
\rho S=r,\qquad\rho'S'=r',
\]
compare the B/F digit paths first. If they agree but the source differs, use
\[
z_A=\rho'r-\rho r'=\rho\rho'(S-S')\ne0,
\qquad \|z_A\|_\infty\le K'B+KB'.
\]
Units establish nonzeroness. No inverse-norm claim is required. Maximize the mixed envelope over all permitted occurrence pairs; include diagonal and cross-profile pairs and canonical setup references where applicable. Preserve the actual admitted matrix bounds when they exceed a derived envelope.

For the inspected compact algebra, the derived three- and four-digit comparison envelopes are
\[
4\cdot56(32^3-1)=7{,}339{,}808,
\qquad4\cdot56(32^4-1)=234{,}880{,}800.
\]
These are derived comparison envelopes, not a claim that every stored producer profile has exactly that radius or that a numerical SIS certificate has been regenerated. B/D alphabet diameter 31 and F/H negative-binary diameter 1 remain load-bearing. Noncanonical legal inner-image/F paths remain accepted; RAM still retains every original signed plane.

Only after domain extraction, Main's dense bound-one response sizing is
\[
U=P\,B\,\omega=56\cdot1\cdot56=3136,
\]
within the positive centered three-digit base-32 reach \(15(1+32+1024)=15855\). This does not use one-hot occupancy to improve the all-valid-source guarantee.

## 6. Exact new local trees and charges

All child transcripts share the required common prefix and own complete accepting continuations. Multilinear stages require nested interpolation, not just flat coordinate pairs.

| New common-K stage | Coordinate extraction numerator | Literal current tree | Conditional coalesced tree |
|---|---:|---|---|
| Physical zeros | \(3\mu+P=104\) | \(2^{16}2^{56}3^{16}\) | \(2^{16}(56+1)3^{16}\) |
| One PC endpoint | \(b_B+2k=11\) | \(2^3 3^4\) | \((3+1)3^4\) |
| Two PC endpoints | \(22\) | \((2^3 3^4)^2\) | \(((3+1)3^4)^2\) |
| 118-leaf Main normalization | \(118+2\cdot22=162\) | \(2^{118}3^{22}\) | \(119\cdot3^{22}\) |

The emitted vectors currently use nested mode. The rightmost column requires a proved common-original-source extractor/comparison at each stage's own prefix; it is not the installed tree. The Main star has the separately stated [anchored-extraction conditions](AKITA_INSTANCE_ACCUMULATION_ANCHORED_BATCH_EXTRACTION.md). Its theorem does not coalesce earlier zero/PC anchors or other multilinear ancestors.

The three source checks have literal coordinate charge 126 and 102 fresh K coordinates. Older notes use conservative 134 by adding an eight-coordinate address/reference allowance; the current PC implementation draws no fresh anchor, so 134 must not be labeled the exact new-coordinate charge. The normalizer adds 140 fresh K coordinates and 100 K algebraic proof elements. Its fixed-false-list one-shot numerator 45 is not its coordinate-extraction numerator 162; the 100 elements also exclude its transmitted claim-list binding and record framing.

These terms do not replace the full frontend ledger. Booleanity gamma degree 110 needs arity 111. Stage-seven's inspected decode gamma exponent 99 needs arity 100. Reused reference challenges retain their original ancestor trees, not duplicate fresh draws. Batched sumchecks use actual maximum degree. Native E2, shell, receiver and final-suffix factors keep their separate laws. The current three-mix receiver contributes \(16\mu+6\) and \(16\cdot288^\mu\), not the historical two-adjunct factor.

## 7. Smallest safe cutover gate

Removing only the selected Jolt Main native prefix requires one admitted deterministic mixed-tree certificate that:

1. authenticates every pending original endpoint through native descendants/final decider without initial Main boundedness;
2. prices all same-source comparisons with the preserved product/path envelopes;
3. covers the complete physical map, ancestor references, reconstruction and point/value aliases;
4. instantiates challenge laws, prefix-preserving conditional full-answer sampling and polynomial extraction runtime;
5. maps the verified normalized Main endpoint into the same native owner through the complete final certificate.

The papers give conditional interfaces, not the complete Jolt-specific mixed K/E2 repeated-root Fiat–Shamir theorem. Keep the generic native Main prefix until the removal gate and joint execution review pass. RAM, receiver and generic non-Jolt predicates remain unchanged; an outer wrapper is optional.

## References

- [Akita](SOURCE_PROVENANCE.md#akita), qmd `lattice-papers/akita.md`: §3.8, Definitions 3.16–3.18, Lemmas 3.19/3.22 and Corollary 3.21; Definitions 10.13/10.14, Lemma 10.15 and Corollary 10.17; Definition 10.22/Theorem 10.23; Theorem 10.27/Lemma 10.28; strong/weak discussion after Remark 10.29; Theorem 10.31/Remark 10.32; Lemma 10.11 and equations (193)–(196). There is no §10.5 source-uniqueness heading in this version; Definition 10.5 concerns honest response admission.
- [SuperNeo](SOURCE_PROVENANCE.md#superneo), qmd local retrieval: §6 Definitions 16–18 and Theorem 12, Appendices B.1–B.2.
- [Lova](SOURCE_PROVENANCE.md#lova), §§1.2.1–1.2.4: short-message binding and extraction norm growth in its own lattice protocol, not authorization for homomorphic folding of Akita commitments.
- [Physical coverage design](AKITA_INSTANCE_ACCUMULATION_MAIN_SOURCE_REUSE.md), [frontend descent](AKITA_INSTANCE_ACCUMULATION_FRONTEND_DESCENT.md), [common-K normalization](AKITA_INSTANCE_ACCUMULATION_COMMON_K_NORMALIZATION.md), [selected native reuse](AKITA_INSTANCE_ACCUMULATION_NATIVE_REUSE_PROFILE.md).
- Source reviews: `ReviewMainDomainCoverage` (no coverage defect, confidence 0.95) and `ReviewMainStrongWeakReuse` (conditional construction and precise remaining gate, 0.94). Neither ran checks or approved a security level. Akita's cited Fiat–Shamir theorem is classical ROM, not a QROM/PQ receipt theorem.
