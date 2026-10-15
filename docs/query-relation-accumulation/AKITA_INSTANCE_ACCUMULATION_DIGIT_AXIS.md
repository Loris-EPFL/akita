# Akita digit-axis normalization on original source handles

Status: independently reviewed algebraic refinement of SourceQueryLift; no mathematical defect was found under the stated source-map and descendant-certificate hypotheses. The selected compact finite profile also passed integrated review. This is not an implemented backend or a claim that compressed Akita commitments are homomorphic.

## 1. Problem and scope

An ordinary opening concerns the decoded polynomial. A raw-source opening concerns its short gadget digits. Noninjectivity of gadget decoding prevents inferring a short-source bound from a bound on the decoded value. That remains true here.

However, *evaluating a decoded polynomial* is a particular linear functional of those same digits. For a positional gadget, its digit weights have a separable multilinear form. This permits converting ordinary evaluation claims to raw-view claims on the **same original commitment**, before a paid multipoint reduction. It does not make the commitment additive or make the source-domain proof unnecessary.

Use the original maps and point order in [SourceQueryLift](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md). Let

\[
K=\mathbb F_q,\quad E/K\text{ a challenge extension},\quad
S=K[Y]/(Y^d+1).
\]

The selected direct-coefficient case has source and carrier degree \(d_A=d\), no basis change, and public gadget scale one. For source column \(p\), block \(i\), position \(x\), digit \(\ell\), coefficient \(j\), the raw source is \(s_{p,i,x,\ell,j}\). The decoded coefficient is

\[
f_{p,i,x,j}=\sum_{\ell=0}^{\delta-1}b^\ell s_{p,i,x,\ell,j}.
\tag{1}
\]

Here \(B,M,d\) are powers of two. Virtual padding of the polynomial and digit axes to \(\bar P,\bar\delta\) supplies **zero** entries, not additional physical A columns. Define the raw MLE in low-to-high address axes \((j,\ell,x,i,p)\); its integer index is

\[
j+d\,[\ell+\bar\delta(x+M(i+Bp))].
\tag{2}
\]

The implementation must transpose from its registered physical source layout if needed. Equation (2) is the raw-query view, not permission to change the commitment's physical matrix columns or F path.

## 2. The fixed digit point

Let \(\bar\delta=2^k\). Require \(1+b^{2^h}\ne0\) in K for every \(0\le h<k\). Define

\[
u_h=\frac{b^{2^h}}{1+b^{2^h}},\qquad
J_{b,k}=\prod_{h=0}^{k-1}(1+b^{2^h}).
\tag{3}
\]

For the little-endian bit vector of \(\ell\), the multilinear equality weight satisfies

\[
J_{b,k}\operatorname{eq}(u,\ell)
=\prod_h(b^{2^h})^{\ell_h}=b^\ell.
\tag{4}
\]

Indeed, \(1-u_h=(1+b^{2^h})^{-1}\). Multiplying cancels exactly the fixed public denominators. They are checked at profile admission, not sampled during proving. For \(k=0\), use the empty point and \(J_{b,0}=1\).

Consequently, for every valid source, including noncanonical gadget decompositions,

\[
\widetilde f_p(r_j,r_x,r_i)
=J_{b,k}\,\widetilde{\mathrm{Raw}}(r_j,u,r_x,r_i,\operatorname{bits}(p)).
\tag{5}
\]

Terms \(\ell\ge\delta\) vanish by the raw-view definition. A non-power-of-two P causes no problem: an ordinary column claim uses that column's Boolean index, and the unused polynomial slots are zero. A registered decoder with a nonunit fixed scale must carry that scale explicitly; (5) is stated for scale one. Nonseparable tensor/basis maps are outside this lemma and need their actual adjoint, not this formula by analogy.

### Exact selected constants

For \(q=2^{128}-2^{32}+22537\), \(b=32\), \(\delta=26\), \(\bar\delta=32\), the five denominator exponents are \(1,2,4,8,16\). The largest denominator is \(2^{80}+1<q\), so every denominator is nonzero. Exact arithmetic gives

\[
J_{32,5}=43907402183346898511998228289933053821\pmod q.
\]

The five-coordinate identity (4) was checked for all 32 digit indices. These checks verify the stated finite-field identity, not the protocol's implementation or security.

## 3. One raw query per original handle

Absorb all original claims, their roles/coordinates, the source-domain prefix endpoint, the original commitment and the raw-view descriptor before drawing independent coefficients \(\alpha_a\). Convert each ordinary claim \((r_a,v_a)\) to a raw claim \((\widehat r_a,v_a/J_{b,k})\) by (5). Raw-domain claims are already in that view. The fixed inverse \(J_{b,k}^{-1}\) is admissible because (3) was checked.

For the resulting list \((t_a,z_a)\), prove

\[
\sum_a\alpha_a z_a
=\sum_{x\in\{0,1\}^{\mu_{raw}}}
\widetilde{\mathrm{Raw}}(x)
\left[\sum_a\alpha_a\operatorname{eq}(t_a,x)\right].
\tag{6}
\]

A degree-two sumcheck leaves one endpoint \(w=\widetilde{\mathrm{Raw}}(r_*)\) and verifies

\[
\text{terminal claim}=w\sum_a\alpha_a\operatorname{eq}(t_a,r_*).
\tag{7}
\]

Do **not** divide by the random factor in (7). Its zero branches must remain in the error/extraction analysis. Keep all original claims; (6) does not replace them by unauthenticated hints.

The native fold authenticates this single raw endpoint with the full-ring SourceQueryLift map

\[
\mathcal L_{raw}(s_{p,i})=
\sum_{x,\ell}\operatorname{eq}(r_x,x)
\operatorname{eq}(r_\ell,\ell)\,s_{p,i,x,\ell}(Y).
\tag{8}
\]

Coefficient \(j\), block \(i\), and polynomial \(p\) weights enter the terminal scalar equation. This is S-linear; a scalar coefficient functional alone is not. In a quadratic E/K extension retain both K-coordinate planes of the **full ring** E[Y]/(Y^d+1).

Thus H=1 query channel per original handle suffices for this selected direct-coefficient map. P, B, original A/B/F geometry, all \(PB\) partials, raw shortness prefixes and complete decommitments remain. In particular, C_X still has its original P native columns. This is not prefix-packing them into a new polynomial commitment.

## 4. Extraction and completeness obligations

1. Range-prefix and multipoint counterfactual branches must refer to the same original handle and raw source. Descendant-certified source uniqueness, with the actual source/projection radii and shared matrix view, is what supplies this consistency. One accepting realized endpoint is insufficient.
2. With that source fixed across branches, interpolation of the degree-two sumcheck and the independent affine claim coefficients recovers every claim in (6). Equation (5) then recovers the ordinary decoded evaluations. Independently, the full source-domain prefix recovers the short-digit predicate. This is the supplied-output local proof contract, not an honest-input assumption.
3. Honest completion works for every valid noncanonical source: (1)–(8) use its actual digits. No canonical redecomposition is allowed. All physical padding/decommitment predicates still apply.
4. An identity receiver has P=1 and delta=1. Its ordinary witness MLE and raw MLE coincide. Its pending opening and its raw-range endpoint therefore normalize to one channel even in the generic local bounded-input construction. History-only range reuse can save that prefix work but is not necessary to obtain H=1.

For \(\Lambda\) claims, the independent affine batching family has mixed-tree factor \(\Lambda+1\) and conservative interactive error \(\Lambda/|E|\) if all \(\Lambda\) coefficients are independently drawn. A normalized combination with the first coefficient fixed to one may instead use factor \(\Lambda\) and charge \((\Lambda-1)/|E|\), **only if its nonzero-polynomial/extraction lemma and transcript grammar are chosen consistently**. This document selects the simpler all-independent form. The sumcheck adds tree factor \(3^{\mu_{raw}}\) and charge \(2\mu_{raw}/|E|\). A singleton authenticated claim needs no multipoint protocol. Source scans, weights, message bytes and source-prefix trees are real costs.

## 5. References and limits

- [Akita](SOURCE_PROVENANCE.md#akita), gadget decoding and recursive fold relations, Eq. (109), source uniqueness and supplied-output extraction in §§9–10. The digit-point identity is derived here; it is not quoted as an Akita theorem.
- [Source-query lift and normalization](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md), §§2–4 and §8.1.
- [Range reuse and padding](AKITA_INSTANCE_ACCUMULATION_RANGE_REUSE.md).
- [Coefficient-level native relation](AKITA_INSTANCE_ACCUMULATION_LINEAR_RELATION.md).

This reduction does not establish a complete frontend extension lowering, final decider, matrix collision inventory, global FS manifest or measured performance parity. It removes duplicated query channels under an explicit source-map lemma, not by changing the lattice commitment assumption.
