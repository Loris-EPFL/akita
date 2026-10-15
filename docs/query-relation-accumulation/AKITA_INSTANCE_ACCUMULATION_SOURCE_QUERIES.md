# Retained Akita handles: an explicit raw source-query lift

**Status:** derived algebra and a conditional, descendant-certified local extraction result. This is a new relation variant, not an existing Akita API, an admitted numerical profile, or an adaptive accumulation theorem. No source code is changed. The invariant is the **same original commitment shape and A/B/F payload**, with the same source coordinates, decoder, slices, matrix views, and lattice-challenge action. Opening-side maps and relation rows may change only through an explicitly bound new opening descriptor.

This resolves the algebraic question posed in [BOUNDED_BRIDGE §5.2](AKITA_INSTANCE_ACCUMULATION_BOUNDED_BRIDGE.md#52-exact-missing-source-observation-interface-for-retained-gadget-handles). The paper evidence is the indexed [Akita text](SOURCE_PROVENANCE.md#akita), retrieved through live qmd; equation and line references below refer to that text. The [primary paper](https://eprint.iacr.org/2026/1983), as registered in the bounded-bridge source inventory, is linked for provenance, not as a claim that its live revision was checked against the indexed copy.

## 1. Answer and exact boundary

**Yes for the direct-coefficient carrier:** replace the *source-decoding contraction* `L(G_com z)` in a separately typed source-query occurrence by an S-linear map `L_raw(z)` on the full physical gadget-source vector. Contract the digit axis using the raw query's equality weights, **not** powers of the commitment base. Keep the A equation on the same `z`, and keep every original B/F row. A full raw-source MLE evaluation then needs the same `k d_car` base-field coordinates per partial as an ordinary coefficient-packing evaluation. It does not multiply the partial width by the source digit depth. It does enlarge the range-check domain to include that depth, and separate query occurrences still incur full occurrence costs.

**Yes for arbitrary linear functionals with an explicit dual lift:** the negacyclic constant-term pairing gives an S-linear carrier-valued lift of every K-linear coefficient functional into E. The final scalar functional is applied only after authenticating the entire carrier. Functionals with block-dependent contractions need separate channels or occurrences; there is no free arbitrary-query batching.

**Not as an unmodified Eq. (138)/(139) opening:** those equations observe `G_com s`, not `s`. Nor can a scalar coordinate projection replace their ring-valued carrier. For full-ring evaluation-trace challenges, a sufficient general raw-source lift uses `R tensor_K E`, equivalently k R-valued channels, costing `k d_A` base coordinates per partial. The existing `d_A`-coordinate trace contract with its E-linear trace row does not represent every raw K-coordinate functional. It must be extended, or a separately justified restricted query family must be used.

The ordinary original-polynomial evaluation claim remains mandatory. A raw range endpoint is an **additional** authenticated query, not its replacement. These conclusions retain existing handles; they do not recommit them as identity tables, widen their A matrices, change their digit depth, or assume a homomorphism of compressed C.

## 2. Frozen geometry and physical ordering

Suppress a handle index. Let K = F_q, let E/K have degree k with fixed basis `(beta_t)`, and let

\[
R=K[X]/(X^{d_A}+1),\quad S=K[Y]/(Y^d+1),\quad
r=d_A/d,\quad \iota(Y)=X^r.
\]

For the native direct-coefficient mode, `d=d_car` and `r=k h_car` as in Eq. (84)–(85). Keep precisely the challenge family already specified in S and its action through iota. The algebra below needs only the actual subring embedding, not that every ring is a field.

The original handle contains P polynomial sources, B actual source blocks per polynomial, M ring positions per block, and commitment gadget depth `delta=delta_com`. The A input width is **m_A = M delta**. Paper §4.5 defines `G_com = I_M tensor (1,b,...,b^(delta-1))`; thus position x precedes digit ell, not conversely. Write

\[
s_{p,i,x,\ell}(X)=\sum_{j=0}^{d-1}\sum_{a=0}^{r-1}
 s_{p,i,x,\ell,a,j}X^{a+rj}.
\]

The original physical coefficient address is

\[
\operatorname{addr}(p,i,x,\ell,a,j)
=\left(\left((pB+i)M+x\right)\delta+\ell\right)d_A+a+rj.
\tag{1}
\]

The A-column address remains `x delta + ell`. Equation (108)'s response digit layout agrees with this source-column order. The full physical source table has **P B M delta d_A cells**. Include the last physical block's cells after the declared natural length; they are algebraic source coordinates, not absent cells. Equation (70)'s honest padding rule does not prove them zero [Akita lines 1835–1865].

For a Boolean raw-query domain use `Pbar=nextpow2(P)`, `Bbar=nextpow2(B)`, `deltabar=nextpow2(delta)`, with a fixed address injection

\[
\operatorname{grid}(p,i,x,\ell,a,j)
=\left(\left((p\bar B+i)M+x\right)\bar\delta+\ell\right)d_A+a+rj.
\tag{2}
\]

Positions outside actual p, i, or ell ranges are **virtual zeros only**, never new A columns or commitment blocks. M, r, and d are dyadic in the supported direct geometry. Use low-bit-first coordinate blocks `(a,j,ell,x,i,p)` for this new query descriptor; each index uses its ordinary little-endian binary expansion. The descriptor translates to the frozen physical layout through (1). This is a view for a new linear query, not a reinterpretation of `shape_com`, `Src`, the natural polynomial, or the B slices. If an implementation uses another bit convention, it must implement this explicit permutation rather than silently moving source cells.

Let W be this zero-extended raw coefficient table. Its MLE is a different polynomial from the original `f=Dec_shape(s)`, which first applies `G_com` and then the original packing decoder. For P=1 omit its zero-length index. A coordinate projection is obtained by a Boolean raw-query point; it observes the chosen physical digit coefficient. A non-Boolean raw point observes a weighted sum of all physical digit coefficients. Neither is in general an ordinary logical opening of f.

## 3. Direct-coefficient raw MLE construction

Split a raw point into `(u_a,u_j,u_ell,u_x,u_i,u_p)` according to (2), and write chi for Boolean equality weights. For each existing polynomial/block pair define

\[
\mathcal L_{\rm raw,u}(s_{p,i})(Y)
=\sum_{x<M,\ell<\delta,a<r,j<d}
 \chi_{u_x}(x)\chi_{u_\ell}(\ell)\chi_{u_a}(a)
 s_{p,i,x,\ell,a,j}Y^j
=:e_{p,i}(Y)\in S_E.
\tag{3}
\]

Only x, ell, and a are contracted. The j axis remains a carrier polynomial. Recover the claimed raw value by the **scalar** equation

\[
y=\widetilde W(u)=\sum_{p<P,i<B,j<d}
 \chi_{u_p}(p)\chi_{u_i}(i)\chi_{u_j}(j)[Y^j]e_{p,i}.
\tag{4}
\]

This is one E-valued row for one MLE of the entire P-source table. Alternatively omit p contraction and retain P separate source-polynomial values and rows. The choice is bound in the new descriptor, not conflated with the original P-polynomial commitment geometry.

**Commutation proof.** Each source polynomial uniquely decomposes as

\[
s_{x,\ell}(X)=\sum_{a<r}X^a s_{x,\ell,a}(X^r),\quad s_{x,\ell,a}\in S.
\]

Multiplication by `iota(c)` multiplies every lane by c, with exactly the same negacyclic wrap sign in S and R. The coefficients in (3) lie in E and commute with S. Therefore, for every c in S,

\[
\mathcal L_{\rm raw,u}(\iota(c)s)=c\mathcal L_{\rm raw,u}(s).
\tag{5}
\]

No claim is made that the scalar map in (4) commutes with c. This is the same mechanism as Eq. (86)–(93), with the additional source digit axis contracted before the fold.

Bind all partial coordinates and their digits using the complete shared D/H path before drawing fold challenges. Sample one original-family challenge `c_{p,i}` per original polynomial/block, and form

\[
z=\sum_{p,i}\iota(c_{p,i})s_{p,i}\in R^{M\delta}.
\tag{6}
\]

The required new ring row is

\[
\sum_{p,i}c_{p,i}e_{p,i}=\mathcal L_{\rm raw,u}(z)\quad\text{in }S_E.
\tag{7}
\]

The unchanged inner row is

\[
Az=\sum_{p,i}\iota(c_{p,i})t_{p,i},\qquad t_{p,i}=As_{p,i}.
\tag{8}
\]

Reconstruct z from its response digits using `G_resp`; (7) is evaluated on `L_raw(G_resp z_hat)`. **Do not remove `G_resp`.** Only the original-source recomposition `G_com` is replaced by the digit-query contraction. On response digit `(x,ell,v,a,j)`, the right-hand coefficient in (7), prior to the chosen ring reduction, is

\[
\chi_{u_x}(x)\chi_{u_\ell}(\ell)\chi_{u_a}(a)b_z^v.
\tag{9}
\]

The ordinary decoded evaluation instead uses `chi_x chi_a b^ell b_z^v`. This is an exact distinction at the existing physical response address, not a new source layout.

Write `e_{p,i}=sum_j sum_t beta_t e_{p,i,t,j}Y^j`; its width remains **k d**. Decompose those base coordinates in the native D-ring block order of Eq. (107), rather than assuming a globally digit-major flattening. Replace each coefficient in (4) by its `b_1` digit recomposition. The resulting scalar digit weight is

\[
\chi_{u_p}(p)\chi_{u_i}(i)\chi_{u_j}(j)\beta_t b_1^v.
\tag{10}
\]

There is no scalar quotient. Ring row (7) has degree d and k coefficient planes, so its quotient, if used, also has d positions per plane, with the schedule's top-zero convention. A/B/F remain exactly the original dimensions and maps.

## 4. General linear queries: explicit dual and its limits

### 4.1 A universal carrier lift for a common block functional

For arbitrary public coefficients `w_{x,ell,a,j} in E`, define

\[
\lambda(s)=\sum_{x,\ell,a,j}w_{x,\ell,a,j}s_{x,\ell,a,j},\qquad
w^\vee_{x,\ell,a}(Y)=w_{x,\ell,a,0}-\sum_{j=1}^{d-1}w_{x,\ell,a,j}Y^{d-j}.
\]

Let `ct` select the constant coefficient in `S_E`, and set

\[
\mathcal L_\lambda(s)=\sum_{x,\ell,a}w^\vee_{x,\ell,a}(Y)s_{x,\ell,a}(Y).
\tag{11}
\]

Then

\[
\operatorname{ct}(\mathcal L_\lambda(s))=\lambda(s),\qquad
\mathcal L_\lambda(\iota(c)s)=c\mathcal L_\lambda(s).
\tag{12}
\]

For the first identity, the only contributions to the constant coefficient are the degree-zero product and terms of degree d. The latter wrap with a minus sign, cancelling the minus in `w^vee`. For the second, commute c through every product. Thus the pairing is perfect without invoking a field trace, a discriminant assumption, division by d, or treating S as a field. This construction is derived here; it is not claimed as Akita's Eq. (133) trace map.

Use (6)–(8) with `L_lambda`, and use `sum_{p,i} b_{p,i} ct(e_{p,i})=y` for a query whose block coefficients separate as `b_{p,i} w`. Direct MLEs in §3 have exactly the needed separation, but use their cheaper factored contraction and late tail evaluation instead of general negacyclic dual multiplication. Coordinate queries also separate.

### 4.2 No free arbitrary block-dependent contraction

If a requested functional has different weights `w_{p,i,x,ell,a,j}` in different source blocks, generally there is no single common L satisfying all those block maps on the one response z. Using `e_{p,i}=L_{p,i}(s_{p,i})` and then asserting `sum c e=L(z)` is incorrect.

An explicit finite construction is to decompose the coefficient tensor as

\[
w_{p,i,x,\ell,a,j}=\sum_{h<H}b_{h,p,i}w_{h,x,\ell,a,j},
\tag{13}
\]

over E. One may always take `H=P B`, selecting one block per h. Form **all** `e_{h,p,i}=L_h(s_{p,i})`, use H consistency rows on the same z, and one scalar row `sum_{h,p,i} b_{h,p,i} ct(e_{h,p,i})=y`. This multi-channel variant costs `H P B k d` base partial coordinates, not `P B k d`, and needs the widened shared D/H path and H quotient rows. Alternatively use H fully charged group occurrences, including source paths, and bind their scalar sum. The rank factorization in (13) is an optional public optimization, not an assumption that every query has H=1.

### 4.3 Preserving full-ring lattice challenges

For an existing full-ring challenge contract, take `S=R`, `d=d_A`, `r=1`, and iota the identity in (11). This retains the original challenge action. The sufficient universal output carrier is

\[
R_E=E[X]/(X^{d_A}+1)=R\otimes_K E,
\]

with `k d_A` base-field coordinates. Equivalently split the public weights into their k K-coordinate planes, create k R-valued lifts, and reconstruct the scalar with the beta basis. A unit delta of R stays a unit of `R_E`, because its existing inverse maps into the scalar extension. Zero divisors elsewhere cause no difficulty.

This is **not** Eq. (84) with an invented fractional `h_car`. It is a new scalar-extended full-ring query carrier, or k full-ring rows, and a new `d_op=k d_A` opening variant. The current Eq. (97)/(99) trace mode has `d_op=d_A` and uses the embedded E structure, tensor reduction, and trace functional of Eq. (133)–(139). Those contracts must not silently be reused for physical coefficient queries. A Boolean coordinate query with K-valued weights can use one R-valued dual channel and a K-valued constant-term row; the k-channel construction is sufficient for arbitrary extension-valued weights, not a lower bound for every individual query.

**Obstruction under the unchanged trace contract.** A composition of R-linear maps with the existing E-linear trace functional is E-linear for the chosen embedding `E subset R`. A raw K-coordinate projection need not be. Choose a nonzero K-coordinate functional `lambda:R -> K`, normalized to `lambda(1)=1`, and `theta in E\K`. E-linearity would require `lambda(theta)=theta`, whereas its value lies in K. Thus arbitrary raw physical coordinate projections cannot be implemented solely by the current R-valued partial and E-linear scalar trace family. The dual construction avoids this by using the explicit K-coordinate carrier/constant-term interface. It does not change the old source packing or promise that a single current trace partial suffices.

## 5. Why neither ordinary openings nor scalar responses suffice

For `delta>=2`, the source recomposition has the nonzero kernel vector `(b,-1,0,...)` at any one ring position: `G_com(b,-1)=0`. Every existing Eq. (138) contraction on `G_com s` annihilates it, but the raw digit-zero coordinate does not. Consequently no map factoring only through `G_com s` can represent every raw query. This is an algebraic observation, not a claimed A-collision or a claim that both vectors meet a particular narrow source alphabet. A binding original A commitment can identify one source, but the ordinary evaluation row still provides no raw evaluation equation for that source.

Likewise `ct` is not S-linear. For d>1, `ct(1)=1` but `ct(Y)=0`, so embedding scalar outputs as constants gives `ct(Y*1) != Y*ct(1)`. Sending only scalar source evaluations and multiplying them by lattice challenges therefore fails even for the constant-coefficient query. The whole carrier in §3 or §4 is what makes fold subtraction valid.

## 6. Exact new relation and conditional local proof

Define `RawQuery(shape, query-view, u or weights, y; s, st_com)` by:

1. `Rel_com^Akita(pp,(shape,C),Dec_shape(s),st_com)=1`, with exactly the original sources s and A/B/F decommitment data inside `st_com`;
2. `y=MLE(RawView_shape(s))(u)`, or the declared full-physical linear functional;
3. any requested source range, zero, or semantic predicates are separate predicates on this same s, rather than assumptions in `Dec_shape`.

The **weak raw-query relation** adds the paper's unit challenge-difference/product certificates, D/H authentication of all partial digits, and the selected new (7)/(11) and scalar equations. Its source projection is exactly

\[
\operatorname{Src}(\omega)=(s,\widehat t,\text{original B/F chain openings}).
\tag{14}
\]

Discard query points, raw-view tags, partials, D/H data, and challenge differences. The public projection remains `phi=(original shape, C)`; do not add a query-dependent reshaping to phi. This permits comparison with ordinary decoded-query occurrences, whose partials are intentionally different.

**Conditional SQ-Lift theorem (derived local algebra).** Fix a finite supplied list of these new occurrences and ordinary occurrences, all dimensions/maps/alphabets/challenge families, exact row reductions, and complete Equation (109) witnesses. Suppose every fold child has a complete descendant-certified accepting subtree that yields the exact ring and scalar rows plus certified bounds; suppose all source-path and D/H binding hypotheses hold at the actual mixed comparison radii. Then the native one-fold subtraction returns original-source decommitments satisfying the raw and ordinary queries, or returns one of the inventoried matrix collisions.

**Proof.** First authenticate every outgoing witness independently using its complete descendant subtree, as in Theorem 10.23. Recover its exact mixed-dimension ring rows and digit bounds. Compare the fixed opening payload across children to identify the complete partial digit vector using D/H binding. Compare the original C paths to identify the complete t/F digits using original B/F binding. Do not require later challenges or successor commitments to agree across fold children.

For the pair of children differing only at one source coordinate `(p,i)`, write `delta=c'_{p,i}-c_{p,i}`. The scheduled guaranteed-unit hypothesis gives `delta^-1 in S` and `iota(delta)^-1 in R`. Define

\[
s^*_{p,i}=\iota(\delta)^{-1}(z'-z).
\]

Subtract (8) to obtain `A s*=t_{p,i}`. Subtract (7), and use (5), to obtain

\[
\delta e_{p,i}=\mathcal L_{raw}(z'-z)
=\delta\mathcal L_{raw}(s^*_{p,i}).
\]

Cancel the guaranteed unit delta in `S_E`, proving `e_{p,i}=L_raw(s*)`. The same proof holds for (11) or each channel of (13). The scalar equation, already exact on each branch, now gives the raw query. It is not extracted by taking the scalar functional through division. Repeating the fork for every original source coordinate gives every block and its original path. Product bounds constrain `iota(delta)s*`; no inverse norm is assumed small.

Across two query occurrences or extractor invocations with the same phi, differing original source-path digits produce a B/F collision. Otherwise their t values agree. For weak factors delta,delta' and bounded products `r=iota(delta)s`, `r'=iota(delta')s'`, the vector

\[
v_A=\iota(\delta')r-\iota(\delta)r'
=\iota(\delta\delta')(s-s')
\tag{15}
\]

is nonzero whenever s differs, since both factors are units, and `A v_A=0`. Its coefficient bound is

\[
\|v_A\|_\infty\le K'_{1}B_\infty+K_1B'_\infty.
\tag{16}
\]

Use the certified mixed Euclidean alternatives when selected. This is exactly the source-only argument of Eq. (193)–(196), Lemma 10.15, and Corollary 10.17; it does not use the decoded query functional. Thus ordinary and raw occurrences identify the **same source**, not merely equal `G_com s`. The additional query rows do not invalidate this argument, but their alphabets, matrices and occurrence profiles must be in the inventory. This proves the stated conditional local theorem, not a way to generate its descendant tree in an adaptive execution.

**Completeness scope.** For every algebraic decommitment with the declared bounded s and valid original path digits, compute (3) or (11), then (6). All modular equations hold without a canonical decomposition of s and without assuming sparsity. Partial base coordinates always admit the scheduled full-field opening digit encoding. Original noncanonical but certified t/F path digits are retained rather than replaced. To obtain protocol completeness, the response must also fit its scheduled accepted interval, and every quotient/auxiliary value must have a valid bounded encoding. A deterministic sufficient response bound is

\[
\|z\|_\infty\le P B\,\kappa_1 B_s
\]

when every `||s_{p,i}||_infty <= B_s` and raw challenges have embedded l1 norm at most kappa_1. Choosing `delta_f` to cover this whole signed interval proves this part for all bounded sources; a smaller probabilistic/grinding bound requires a valid all-source admission certificate, not honest-digit measurements.

## 7. Range restoration and preservation of original claims

Run the bounded bridge's anchored range/zero prefix on the full raw table W of (2). Its endpoint `(original C, raw-view, u, y)` is now a raw-query occurrence from §3, not an ordinary opening of `Dec(s)`. All virtual cells are defined zero; all actual cells, including physical padding, participate. A single raw MLE endpoint is enough for that prefix's single table; several separately proved tables/endpoints are charged separately.

With complete authenticated descendant trees and source uniqueness from §6, substitute the unique W into every prefix endpoint. Backward sumcheck interpolation and anchor separation prove `Q_D(W(a))=0` and each declared masked zero condition at every Boolean a. Distinct roots give the original physical digit bound directly, with no inverse-challenge loss:

\[
\|s_{p,i}\|_{\infty,coef}\le B_s,\qquad
\|s_{p,i}\|_{2,coef}^2\le M\delta d_A B_s^2.
\]

This is the required replacement for the bridge's identity-decoder step. `RawView` is injective on the full original source, although the original G decoder is not.

Retain the ordinary original-polynomial evaluation occurrence(s), including original packing/tensor and semantic padding claims. For direct packing, the decoded relation has its original powers `b^ell`; for trace packing, retain its original trace/tensor reduction and source decoder. A physical source range proof neither changes nor implies those evaluation values.

Range is also **not canonicality**. It does not force a unique balanced expansion of a field element. If canonical integer representatives, byte semantics, or a comparison to q are needed, prove them separately on the same original source/recomposition. Similarly, imposing zero raw digits in a padding position is stronger than requiring only `G_com s=0` there. An arbitrary bounded algebraic decommitment can have cancelling nonzero padding digits. Use no raw-zero mask where the intended input relation permits those sources; if a raw-zero predicate is desired, declare it as an input condition and certify it. Logical decoded padding requires its own predicate or a proven implication from the selected raw-zero mask. This distinction is necessary for completeness on every valid algebraic input.

## 8. Occurrences and complete overhead

### 8.1 Conservative integration: separate occurrences

The baseline construction uses **one source group occurrence per raw MLE point** and keeps every original decoded-point occurrence. This follows Akita §4.2's explicit one-point-per-instance rule [lines 1593–1608]. C need not be transmitted or computed again, but each occurrence has its own query-dependent partials, fold response, full source-path witness slots, rows, and challenge coordinates. Reusing in-memory source buffers is not a proof-size discount.

For a handle containing P polynomials, an occurrence retains all P in its frozen B-slice geometry, even if its scalar row only selects one polynomial. Never replace the old B matrix by a smaller `P=1` view. Point sets that differ on contracted digit/x/a axes cannot be hidden in claim-local tail or block weights; the common-L condition is the analogue of Akita line 2091. A second raw point generally means a second occurrence.

An explicitly different multi-channel relation can share one z/A/B/F path across J queries: authenticate every `e_{h,p,i}`, impose J versions of (7) on that one z, and impose every scalar query. The proof in §6 applies to every channel using the same unit fork. This saves duplicated response/source-path slots but costs J times the partials and fold-evaluation rows and changes the descriptor, challenge/row compiler and soundness bookkeeping. It is not the paper's existing free multipoint operation. The ledger below uses separate occurrences unless H channels are explicitly declared.

#### Paid normalization of several raw points

Several claims on the **same full RawView** can instead be reduced to one raw point before the native fold. Bind that original C, view and every \((r_j,y_j)\), then sample claim-combination coefficients \(\alpha_j\) and run degree-two sumcheck for
\[
 \sum_j\alpha_jy_j=
 \sum_{x\in\{0,1\}^{\mu_{\rm raw}}}
 \widetilde{\operatorname{RawView}(s)}(x)
 \left(\sum_j\alpha_j\operatorname{eq}(r_j,x)\right).
\]
At the resulting point z retain the one raw claim
\(\widetilde{\operatorname{RawView}(s)}(z)=t\);
check the sumcheck terminal as
\(t\sum_j\alpha_j\operatorname{eq}(r_j,z)\).
Do not divide by that factor or assume it is nonzero.

This is an additional public-coin reduction with source-projection consistency across its branches, not homomorphism of C or a free native API. Charge its source traversal, all messages, claim-separation error, degree-two rounds and mixed-tree factors. It can normalize multiple range/padding endpoints on one raw view to one raw channel. Keep the separately normalized **ordinary decoded** channel: generally it observes a different map. A different RawView or unsupported source map is not silently covered by this formula.

**Direct positional-gadget refinement:** [the independently reviewed digit-axis lemma](AKITA_INSTANCE_ACCUMULATION_DIGIT_AXIS.md) converts the ordinary channel itself to this same RawView at a fixed checked digit point. Under its exact scale/basis/order hypotheses, all claims normalize to one channel while retaining the original P/B/A/F geometry. This is a separately proved conversion, not an inference from (6) alone; other source maps keep their distinct channels.

### 8.2 Exact symbolic witness and row ledger

For occurrence g let P_g, B_g, M_g, delta_g, n_A,g, and its **original** B/F shapes be fixed; let H_g be its declared channel count (normally 1). Let `d_op,g=k d_g` for direct raw queries, `k d_A,g` for the full-ring scalar-extended variant, or the original ordinary-opening width. Let response and opening depths be `delta_f,g`, `delta_open,g`. With no response chunking, its main coefficient counts are

\[
Z_g=M_g\delta_g\delta_{f,g}d_{A,g},\quad
E_g=H_gP_gB_g\delta_{open,g}d_{op,g},\quad
T_g=P_gB_gn_{A,g}\delta_{open,g}d_{A,g}.
\tag{17}
\]

For a fixed new direct raw occurrence its raw-vs-decoded **partial** width is identical; its `Z_g` still includes the entire original gadget depth. Do not substitute `M_g` for `M_g delta_g`. Multiple full occurrences multiply all three terms. Source s is a prover input of `P B M delta d_A` field coordinates; it is not an additional carried segment beyond the fold response. Retaining arbitrary valid original t/F decommitments also has memory cost.

Native opening order: before digitization, `(channel,p,i,t,j)` with t then j for scalar-extended carriers; channel is omitted for H=1. For a partial whose base width is D and `s_D=D/d_D`, its local stored address is

\[
((j_{partial}s_D+y)\delta_{open}+v)d_D+k_0,
\]

exactly Eq. (107). Inner-image digits keep the original B-native subdivision and Eq. (73) slice padding. Require `d_D | d_op,g` for every active variant. The shared D width and image size are

\[
m_D=\sum_g E_g/d_D,\qquad |v_D|=n_Dd_D.
\tag{18}
\]

There is one unsliced D and one shared H path, not a D image per source. Size n_D at this total width and its actual scanning alphabet. If no allowed d_D divides all required widths, the profile is not admitted as written; choose a valid new opening-side profile, not a new original A/B/F handle.

The native ring-row inventory, before scalar batching, is:

- per occurrence: H_g fold-evaluation rows (dimension d_g with k coefficient planes for scalar-extended carriers), n_A,g A rows, and `S_B,g n_B,g` original outer rows;
- shared: n_D D rows;
- at each active compression stage a: `sum_g n_F,g,a` F rows and `n_H,a` shared H rows;
- every scalar query row, or one freshly randomized batch with an explicit claim-separation/tree charge. An ordinary trace prefix retains its separate tensor error.

For `k R` channels the scalar-extended row is equivalently k base-coefficient ring rows; count either representation once, not both. A scalar-extended quotient row has k base-coordinate planes. If row r has native dimension d_r, coefficient-plane count p_r, and quotient depth delta_Q,r, charge

\[
Q_r=p_r d_r\delta_{Q,r}
\tag{19}
\]

with all scheduled top-zero coefficients retained; scalar rows have no quotient. The quotient-free route sets all Q terms to zero and must implement the corresponding negacyclic adjoint weights. In the general dual variant the multipliers `w^vee` are public degree-<d polynomials, so products have degree at most `2d-2`; this fits the same native ring-reduction normal form but changes the public weight computation and quotient envelope. Do not import a quotient bound derived for the old sparse/factored multipliers.

For each F or H stage a, let `I_a` be the preceding image's base-field coefficient count and `ell_q=ceil(log2 q)`. Its binary input occupies

\[
C_a=d_a\left\lceil\ell_q I_a/d_a\right\rceil
\tag{20}
\]

cells, in the paper's digit-plane order including ring-completion zeros. For F, `I_1=S_B n_B d_B` and `I_2=n_F,1 d_F,1`; for H, `I_1=n_D d_D` and `I_2=n_H,1 d_H,1`. These stages occur twice on compressed paths and zero times on raw paths. Their final payload sizes are the scheduled final image sizes; neither the 128-byte example nor a fixed n_D survives arbitrarily enlarged inputs without re-admission.

The complete carried witness is

\[
W'=\mathop{\|}_g[\widehat z_g\mid\widehat e_g\mid\widehat t_g]
\|\widehat Q_{main}
\|\mathop{\|}_{a=1}^{L_{comp}}
[\mathop{\|}_g\widehat u_{g,a}\mid\widehat v_a\mid\widehat Q_a],
\tag{21}
\]

plus **all** stage alignment, ring-completion, successor ring padding and final Boolean tail prescribed by Eq. (109). Thus its exact physical length is `sum_g(Z_g+E_g+T_g) + sum_r Q_r + sum_g,a C_F,g,a + sum_a C_H,a + alignment/tail zeros`. Repeated occurrences carry repeated F slots in this baseline even though their payload reference is equal. None of these terms may be removed merely because source uniqueness can later identify them. If response chunking is used, repeat the applicable per-chunk main/auxiliary segments and use its aggregate response bounds; (17) explicitly assumes no such chunking.

The raw range domain has

\[
N_{raw}=\bar P\bar B M\bar\delta d_A,\qquad \mu_{raw}=\log_2 N_{raw}.
\]

For the simple alphabet-D prefix, charge `(|D|+1) mu_raw` compressed extension-field sumcheck coefficients plus its endpoint, the complete mixed prefix tree, public mask/weight evaluations, and the downstream occurrence above. Full physical source traversal replaces traversal of only the logical decoded cells. General dual contractions additionally require actual polynomial products or their adjoints; there is no asserted timing advantage.

## 9. Implementation and security admission still required

The construction removes an unspecified **algebraic** SQ-Lift gap for direct coefficients and supplies a universal, more expensive alternative for full-ring action. It does not certify a concrete profile. The exact remaining work is:

1. **Typed opening variant:** bind ordinary-vs-raw query kind, complete original handle identity, source-view injection, digit axes, scalar coefficients, channel count, points, and all row orders. `shape_com` and original A/B/F geometry stay byte-for-byte fixed. New raw queries must never pass as ordinary decoded openings.
2. **Original source access:** expose all physical gadget digits and arbitrary valid algebraic path digits, not only a canonical polynomial API or natural live positions. Preserve physical last-block and source-padding cells. Raw endpoint generation must use the retained source, not recompute a potentially different canonical decomposition of `Dec(s)`.
3. **Matching prover/verifier rows:** implement (9) versus ordinary `b^ell`, correct native D digit order and extension planes, the scalar row (10), direct or dual carrier multipliers, quotient or quotient-free adjoints, all padding masks, setup scans/offloaded claims, and the complete (21) layout. Audit trace-specific restrictions rather than aliasing the full-ring extension into current trace mode.
4. **Sampler and all-valid-source completeness:** certify accepted challenge-family cardinality and pairwise units in the original action ring; cover every bounded source and every permitted original decommitment alphabet; give signed response/quotient intervals and retry bounds. Small honest digit experiments do not establish this quantifier.
5. **Mixed matrix certificates:** price every original A comparison using (16), every B/F comparison using the full cross-profile certified alphabets, and new D/H maps using their exact total widths. Original frozen A/B/F ranks may fail this inventory; an already formed handle cannot be repaired by widening or reinterpreting its matrices. It is then outside this proposed profile. Include the source-reference profile only if a proof actually compares to it.
6. **Full bounded receiver closure:** count (17)–(21), setup coefficient views, all source/path alphabets, physical padding and prefix/query multiplicity for every permitted old/fresh shape and declared finite Tmax. Prove the resulting receiver stays within its fixed capacity. This document provides no successful numerical solution.
7. **Strong-prefix and schedule composition:** use source uniqueness on the original phi for every complete range-prefix branch and ordinary-query occurrence. Bind prefix order, query descriptors, D/H payload, scalar batching and lattice challenges in that order, under separate native Fiat–Shamir domains. Update field error, claim separation, unit-fork and full tree/runtime accounting. Supplying a descendant-certified tree locally does not establish its availability for the adaptive history. The finite adaptive-chain argument and final complete bounded decision remain separate obligations.
8. **Frontend semantics:** preserve each segment's original opening claims and boundary linkage. Source range alone establishes no Jolt execution constraint, canonical integer/byte value, or decoded padding predicate not included in that frontend relation.

The HashFS use is only the native Fiat–Shamir transcript. No hash or Merkle accumulator replaces any lattice commitment, and no completed-proof/residual accumulation is proposed.

## 10. Retained source queries versus new Boolean bitplanes

| Issue | Retained gadget handle plus SQ-Lift | New identity bitplane producer |
|---|---|---|
| Existing C | Preserved exactly | Cannot retrofit an already formed C |
| A input | Existing `M delta` gadget columns | New identity-source geometry selected before commitment |
| Range table | Every physical gadget coefficient; declared alphabet D | Every physical plane bit, plus physical zeros |
| Endpoint map | New raw-source relation (3) or (11) | Ordinary identity-source evaluation after full physical identity is certified |
| Original value link | Retain existing `G_com` and original evaluation row | Explicit weighted plane recomposition and original frontend value semantics |
| Direct partial width | `k d_car` per source block/query/channel | Its own frozen identity-profile partial width; not assumed equal |
| Prefix degree | `|D|+1` for the simple prefix | Degree 3 for Boolean/zero fusion |
| Additional costs | Raw query occurrence/channel, wider physical domain, exact (21) | More producer cells/planes and frontend recomposition endpoints; exact new commitment and receiver sizes |
| Full-ring arbitrary query | May use `k d_A` carrier width | Still needs the chosen identity packing's actual physical/logical contract |
| Admission | Mixed original profile and new query rows | New original profile, Boolean constraints, source/receiver closure |

Neither route has a demonstrated timing or security advantage here. The direct raw lift avoids changing existing C and avoids an automatic delta-fold increase in partial width, but not the raw-domain, source-column, repeated-occurrence, or proof-admission costs. Bitplanes avoid the new source-observation relation for producers migrated from birth, but are not free encodings and do not remove original frontend semantics or padding obligations.

## 11. Source evidence and exact algebra experiment

### Paper evidence actually retrieved through qmd

- Akita lines 1579–1738: frozen shape, one-point occurrence rule, topology, setup inventory.
- Lines 1830–1994: source digit order, full physical decoded grid, A/B geometry, F/H digit and padding formulas.
- Lines 2090–2233: common contraction restriction; Eq. (84)–(97) carrier split, embedding, unit preservation, ordinary partial widths.
- Lines 2233–2363: shared unsliced D, actual widths, challenges/responses, Eq. (106)–(110) complete witness and native digit order.
- Lines 2650–2782: scalar/trace rows and Eq. (138)/(139), mixed-dimension relations and quotient inventory.
- Lines 3709–3824: algebraic decommitment, weak source projection, denominator-clearing proof, source uniqueness across opening profiles.
- Lines 3850–4004: descendant-certified tree condition and exact one-fold subtraction; local theorem versus full schedule distinction.

### Current-code evidence, not a pinned-modern implementation claim

The inspected current local tree is the separately declared Akita `2607265...` tree, not the source-selected modern pair Akita `8357433...` / Jolt `0791f79...`. No checkout or source modification was made. These observations locate the current implementation's corresponding contracts; they do not establish a modern-revision raw-query API.

- [`coefficient_packing_fold.rs`](../../crates/akita-cpu-backend/src/arithmetic/coefficient_packing_fold.rs), lines 54–100 and 103–220: the reference map contracts low coefficient indices and returns `[block][extension coordinate][subring coefficient]`; the inspected helper is `#[cfg(test)]`, so it is not evidence of a production raw API. Its natural-live-position interface is insufficient as an arbitrary full physical algebraic source interface without an explicit new contract.
- [`witness_build/coefficient_packing.rs`](../../crates/akita-cpu-backend/src/opaque/witness_build/coefficient_packing.rs), lines 59–165: shared group-ordered D input, native D subcolumns before gadget digits, mode/width/divisibility checks. The new variant must update these authorities consistently.
- [`coefficient_packing_terms.rs`](../../crates/akita-cpu-backend/src/opaque/sumcheck/relation_range_image/coefficient_packing_terms.rs), lines 291–336: existing response terms iterate both `witness_gadget` and `fold_gadget`, address the physical source digit, and multiply the position weight by both gadget weights. SQ-Lift replaces the source `witness_weight` by `chi_uell(ell)` while preserving `fold_weight`, not by pretending the source depth is one.
- [`proof/coefficient_packing_relation.rs`](../../crates/akita-types/src/proof/coefficient_packing_relation.rs), located declarations and lines 513–523: source and response gadgets are separately derived from inner and fold depth/base parameters. A typed new semantic authority is required; changing only a prover term would not define a valid new protocol.

### Ephemeral exact check performed

An in-memory Python calculation, with no saved script or permanent test, used `K=F_17`, `R=K[X]/(X^8+1)`, `S=K[Y]/(Y^2+1)`, `iota(Y)=X^4`, and E=K. This is a valid dyadic algebraic subring/carrier example, **not** an Akita security profile. For 200 deterministic pseudorandom trials with six source ring coordinates and arbitrary coefficient weights, it checked:

1. `ct(L_lambda(s))=lambda(s)`;
2. `L_lambda(iota(c)s)=c L_lambda(s)` for arbitrary sampled c;
3. the guaranteed unit `delta=Y`, inverse `-Y`, and exact recovery of s from `iota(delta)s`;
4. cancellation in the carrier with that inverse;
5. the explicit `(4,-1)` gadget-kernel and scalar-projection obstructions.

Observed output: **200 trials, 1003 successful assertions**. The random c values were not asserted to be units; only the explicitly invertible delta was used for recovery. The experiment exercises algebra over K, including a ring with zero divisors; it does not exercise extension-field code, a cryptographic sampler, commitment binding, norms, Fiat–Shamir, Jolt, source revisions, implementation correctness, or any admission estimate. No build, repository test suite, formatter, linter, benchmark, branch change, or commit was run.

**Integration finding:** SQ-Lift is no longer merely an unnamed algebraic lemma for direct-coefficient retained handles. Equations (1)–(10), relation (14), the guaranteed-unit proof, and the complete ledger give a concrete conditional interface. The remaining blockers are the exact implementation/profile certificates and the separate bounded adaptive composition, not a fundamental inability to observe original source digits while preserving C.
