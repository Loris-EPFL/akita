# Obligation 03 — original commitment consistency and exact deferred predicates

## 1. Success criterion and status

A successor must certify the **original** Akita handles and their original evaluation claims, not merely a polynomial chosen by the aggregation prover. The update verifier must return the precise unfinished predicate; it must never label a verified reduction prefix as a verified PCS opening. This document gives two interfaces:

- **A: completed-proof public residual accumulation.** An exact terminal verifier split, public arithmetic update, and backward cancellation lemma. This is a justified algebraic construction, conditional on faithful extraction of the retained verifier predicate and an authenticated update receipt. It is not a private short-opening accumulator.
- **B: private native re-entry.** A concrete original-handle-to-successor reduction using Akita's A/B/F source paths and shared D/H opening path. Its equations and local tree-conditioned backward link are source-backed. Repeating it with a fixed receiver and fresh application groups is a **new protocol**, conditional on the receiver/domain certificate and a new adaptive polynomial-time extraction theorem. No current API or completed theorem is claimed.

The preferred B construction does **not** posit additive homomorphism of a full Akita commitment and does **not** substitute an unrelated fresh recommitment. Section 8 gives a fully specified, deliberately expensive fresh-recommitment alternative and lists its actual remaining evaluations; it is not used to hide unverified openings in a consistency theorem.

## 2. Evidence and the nonhomomorphism obstruction

[Akita](SOURCE_PROVENANCE.md#akita), §§4.5, 5.3–5.4, 7 and 8.2, uses an inner Ajtai image for each source block, gadget decomposition, an outer sliced Ajtai map, and optionally two further compression maps. In schematic notation, with all layouts retained,

$$
 s_i\longmapsto t_i=A s_i\longmapsto\hat t_i
 \longmapsto u=B_{\Pi_B}\hat t\longmapsto\hat u_1
 \longmapsto F_1\hat u_1\longmapsto\hat u_2
 \longmapsto F_2\hat u_2=C_{\rm pub}.
$$

The linear matrix maps are homomorphic on their own algebraic inputs. **Canonical digit decomposition is not linear:** carries change the digit vector, and the second-tier image is not the image of an additively combined source under the same commitment algorithm. Even without F compression, generally

$$
 B\operatorname{Decomp}(A(s+s'))\ne
 B\operatorname{Decomp}(As)+B\operatorname{Decomp}(As').
$$

For a simple unsigned radix-2 illustration, the low/high digits of 1 are $(1,0)$, while those of 2 are $(0,1)$, not $(2,0)$. Balanced decompositions also have carries. Changing digits by a carry leaves their gadget recomposition unchanged but changes the input of an unrelated outer matrix. This illustration concerns the nonlinearity of decomposition, not Akita's exact signed alphabet.

Furthermore, bounded lattice binding is not binding on an unrestricted additive hint group: repeated additions need not preserve the admissible norm. [Halo Infinite](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_23.pdf), CRYPTO 2021, §3 explicitly calls lattice schemes with a bounded opening-addition interface **bounded additive**, rather than automatically satisfying Definition 3.1. Its Definition 3.3 permits a nontrivial linear-combination scheme, but requires the **composition with Eval** to prove the consistency relation. A fresh commitment alone is not that scheme.

The native repair is to keep each original outer payload fixed and prove the rows linking its hidden inner images to the responses. Independent opening groups can retain different points, ring dimensions, digit bases and source maps. Akita §5.3 explicitly says another opening point reuses the same committed source data. This is a link through original paths, not $C'=C+\rho C_{\rm seg}$.

### Source uniqueness is narrower than canonicality

Akita Definitions 10.13–10.14 and Remark 10.32 distinguish an algebraic decommitment from an honest canonical commitment state. The weak extractor has products

$$
 r_i=\rho_i s_i,\qquad \rho_i\text{ a unit},\qquad
 \|r_i\|\text{ bounded},
$$

not a bound on $s_i=\rho_i^{-1}r_i$. Division by a small unit need not preserve norm. Nor does an accepted padded opening certify membership in the image of the honest padding map.

[SuperNeo](SOURCE_PROVENANCE.md#superneo), §6, Definitions 16–18 and Theorem 12, supplies the correct conceptual contract: uniqueness must hold against adaptively repeated extraction, with oracle runtime included, and the strong and weak reductions must use the **same public projection**. For Akita, that projection is the ordered original commitment shapes and payloads; the unique object is the source projection, not the point-dependent D/H path. Akita's outgoing commitment is sent *after* the fold challenge and can change across folding children. Akita §10.2 therefore explicitly does **not** apply SuperNeo's sequential theorem as a black box: it proves a nested, descendant-certified tree reduction instead.

## 3. Frozen types and original-handle admission

These are proposed mathematical interfaces, not Rust APIs. Write $F=\mathbb F_q$, $E/F$ for Akita's evaluation field, and $R_d=F[X]/(X^d+1)$. Public residual accumulation below uses a separate field $K/F$; it does not replace Akita's ring-challenge carrier.

### 3.1 Profile and handles

A fixed profile $P$ contains:

1. field/ring definitions, extension bases, packing maps, source decoders, source-recomposition bases and depths;
2. the **exact physical views** of A, B, D, F and H, including setup seed/expansion version, $(n,m,d)$, coefficient order, B slice ranges and per-polynomial slice padding;
3. certified input alphabets, response bounds, challenge families/filters/unit-difference certificates, raw draw allowances, common nonce budget and all transcript domain tags;
4. the admitted opening-method/ring-check policy, the fixed receiver capacity $N_*$, maximum fresh groups/claims, exact segment/quotient/compression address catalogue and zero positions;
5. program/run/session and public-IO binding, permitted step count $T_{\max}$, and any canonical setup registry entries;
6. the collision comparison classes and their radii, including original/receiver/reference cross-profile comparisons.

An original handle is

$$
\mathsf{Handle}=(P_{\rm id},\mathsf{sourceShape},\mathsf{role},
 \mathsf{payload},\mathsf{sourceStatementId}).
$$

`sourceShape` includes the full commitment path, not only a polynomial length. The verifier resolves it from the bound profile/statement and rejects a prover-selected alternate view for the same payload. An opening group is

$$
\mathsf{Group}_g=(\mathsf{Handle}_g,r_g,
 (\pi_{g,c},\phi_{g,c},v_{g,c})_{c<P_g},\mathsf{openingDescriptor}_g).
$$

It binds group order, local polynomial order, role, original transcript identity, absolute level, native point/projections and all witness widths. Derive $\phi$ from the admitted layout; reject $\phi=0$ rather than silently dividing or reinterpreting the natural-domain claim. A separate image certificate is needed when the application requires $f^*_{g,c}=\mathsf{Pad}_{g,c}(F_{g,c})$. Otherwise the theorem's claim is the full-grid equation $\widetilde f^*_{g,c}(r_g)=\phi_{g,c}v_{g,c}$.

Repeated occurrences of one handle, including openings at different points, use the same source family. A profile may not create a new commitment meaning by changing its A/B/F view at reception.

### 3.2 Deferred-state sum types

An update result is exactly one of `Reject`, `AcceptPending`, or `AcceptDecided`. `AcceptPending` records an unfinished predicate, not truth of that predicate:

$$
\begin{aligned}
\mathsf{PendingOrdinary}=(&P_{\rm id},\mathsf{receiverHandle},N_*,
 r',v',\mathsf{domainTag},t',\mathsf{epoch}',h',\mathsf{boundary}'),\\
\mathsf{PendingSetup}=(&P_{\rm id},\mathsf{registrySlot},
 \mathsf{registeredHandle},\rho_S,s_{\rho_S}),\\
\mathsf{PendingInner}=(&P_{\rm id},\mathsf{terminalShape},
 (t_i)_{i<B_{\rm term}},r,v,\mathsf{predecessorBinding}).
\end{aligned}
$$

The distinction between `PendingOrdinary` and `PendingInner` is mandatory. The latter is the specially authenticated terminal interface, not an ordinary outer commitment with B discarded.

For the preferred direct-setup B route, `AccV` outputs

$$
\mathsf{AcceptPending}(\mathsf{PendingOrdinary},
 \mathsf{derivedInputDigest},\mathsf{checkedPrefixDigest})
$$

and **no pending setup claim**. If setup offloading is enabled, it outputs an ordered pair $(\mathsf{PendingSetup},\mathsf{PendingOrdinary})$ at their distinct points, as Akita Eq. (170) specifies; calling this pair one opening is incorrect. The next step must consume the setup claim against the registered commitment, and a terminal-targeting step must create none.

The private state contains the decoded live vector and the actual commitment/opening hint material, including all source blocks, inner-image digits and compression openings needed for the next prover invocation. Proof bytes alone are not this state.

### 3.3 Bounded receiver relation and decider

The proposed old-accumulator relation is

$$
\begin{aligned}
R_{\rm dom}(a;W,\mathsf{st})\iff{}& |W|=N_*,\quad
 W_j\in\mathcal A_*\text{ for all }j,\\
& W_j=0\text{ at the profile's required zero positions},\\
&\mathsf{Rel}_{\rm com}^{\rm Akita}
 (P,\mathsf{receiverHandle},W,\mathsf{st})=1,\\
&\widetilde W(r)=v,
\end{aligned}
$$

with identity source digit depth $\delta_{\rm com}=1$ and an injective coefficient-packed full-grid source for the old lane. This is an explicit receiver proposal, not a repair for every fresh original source. Fresh groups keep their original source maps and their declared weak/full-grid or separately certified bounded-source semantics.

For the preferred all-valid-input B contract, fresh groups also need profile-bounded source openings **linked to their original handles**. A full-field original vector can use a bounded canonical gadget-source certificate with its original $\delta_{\rm com}$; certifying a new unrelated vector is insufficient. The weak/full-grid-only variant preserves evaluation semantics but does not supply the bounded-source admission needed by that all-valid-input completeness contract. A compact original-source checkpoint and its auxiliary/state costs are an additional explicitly declared reduction obligation; native weak product bounds alone do not discharge it.

A conservative final decider reveals $(W,\mathsf{st})$ once and checks every conjunct, including original receiver A/B/F equations, all alphabets and zeros, and the evaluation. It must also authenticate that this is the latest profile/history/boundary. A compact domain proof may replace this revelation only with a sound proof of the **same** relation. An ordinary PCS evaluation verifier does not, by itself, establish this all-valid-state bounded domain. A generated-state-only relation must instead be named and justified explicitly. A terminal inner-state discharge must be linked to this receiver/domain relation before the terminal specialization; $t_i=A_{\rm term}s_i$ alone is not a source-range certificate.

## 4. Concrete native original-handle-to-successor protocol

**Preferred profile:** unchunked; direct setup evaluation; coefficient-bound response certificates; fixed receiving shape; coefficient-packed identity source for the live accumulator. This deliberately avoids treating optional setup offloading, Euclidean norm reconstruction or response chunks as already solved. The opening-method policy must be part of the new certified topology: Akita Eq. (162)'s absolute-level policy cannot silently be reset to root at every step.

The ordered group sequence is old accumulator followed by the bounded number of fresh original groups in their statement order. If an optional setup claim is consumed, put the registered setup group in the explicitly declared position. Empty fresh slots are profile-defined absent slots, not missing required claims.

1. **Authenticate the frontend and old statement.** Replay/prove the original frontend reductions and segment boundary checks. Resolve all original handles from their original statements. Bind $P_{\rm id}$, complete old state, segment/program/session/index/IO identity, ordered groups, points, values, roles and the receiver shape before any update challenge. The receipt must later prove/replay these checks.
2. **Apply the receiving opening-method reduction.** In evaluation-trace mode run the shared tensor reduction on the distinct incoming points and native arities. It fixes each scaled claim $h_{g,c}=\theta_g g_{g,c}(\rho_g)$; derive and check every $\theta_g\ne0$. The coefficient-packing route uses its group-local contraction directly. Neither route identifies different input points.
3. **Bind the original partials through one shared D/H path.** Compute the partials and their base-field digit vectors $\hat e_g$. Concatenate them without per-group common-dimension padding. For

   $$
   d_{{\rm op},g}=\begin{cases}k d_{{\rm car},g}&\text{coefficient packing},\\d_{A,g}&\text{evaluation trace},\end{cases}
   $$

   require $d_D\mid d_{{\rm op},g}$ and set

   $$
   m_{D,g}=P_gB_g\delta_{{\rm open},g}
       \frac{d_{{\rm op},g}}{d_D},\qquad
   m_D=\sum_gm_{D,g},\qquad
   v_D=D(\hat e_0\|\cdots\|\hat e_{G-1}).
   $$

   Absorb the raw D image or terminal H-chain payload. **D is unsliced and has the exact summed width.** There are no independent group-local D payloads. Use the same compression depth, zero or two, for every original B/F path and the shared D/H path at this fold.
4. **Sample scalar claim coefficients, then sparse fold challenges.** Only after the complete D/H payload is fixed, sample the mode-specific public claim combination: powers of a fresh scalar for the admitted packing-root batch, or the separately fresh normalized coefficients for a trace receiver. For the proposed new topology, the selected combination and its error bound must be explicitly certified. Then one common nonce selects the complete domain-separated sparse challenge tape, with one $c_{g,c,i}$ per group/claim/block. Apply the fixed filters. On retry, resample the entire tuple, not only a favorable group. Compute

   $$
   z_g=\sum_{c<P_g,i<B_g}c^A_{g,c,i}s_{g,c,i},\qquad
   c^A=\iota_g(c)\text{ in packing; }c^A=c\text{ in trace mode}.
   $$

   Enforce the common nonce bound and each scheduled response certificate. Challenge differences required by extraction must be units in their native rings. A response-dependent retry is not a new witness-independent challenge filter.
5. **Build and bind the entire successor.** Decompose each $z_g$ in its scheduled positional base. Assemble Eq. (109), with every quotient, compression digit, alignment zero and receiver-padding coordinate described below. Commit it under the fixed receiver. Absorb the successor commitment **before** ring-reduction point $\alpha$ and row/range-batching challenges. Never let the prover select the outgoing witness after those challenges.
6. **Run the exact native strong-check prefix.** Verify digit range, compression binariness, selected norm route, all reduced ring/scalar relations and the fused sumcheck. Evaluate the public weights directly under the preferred profile. The last sumcheck check derives $(r',v')$ and leaves precisely $\widetilde W(r')=v'$ under the already absorbed receiver commitment. Additional required zero/domain rows must be in the certified relation/domain interface, not silently assumed from honest padding.
7. **Return the typed pending state.** Derive $t',h',\mathsf{boundary}'$ from checked input/update data. Output `PendingOrdinary` with its domain tag. Do not report that $R_{\rm dom}$ or the remaining PCS opening has already been verified. Prover state supplies its actual $W$ and hint material. The next update consumes this exact handle, not a prover's replacement commitment.

The transcript order is a security condition: original shapes/claims → tensor prefix if needed → **shared D/H** → scalar combination → joint sparse response challenge/nonce → **successor commitment including quotients** → ring point/range/row challenges → strong-check sumchecks → derived pending evaluation.

## 5. Exact equations and witness geometry

### 5.1 Recomposition and original path equations

Every positional gadget map has its profile-specified radix, digit depth, native block packing and address order. In particular,

$$
 f_{g,c,i}=G_{b_g,M_g}s_{g,c,i},\quad
 t_{g,c,i}=G_{b_{1,g},n_{A,g}}\hat t_{g,c,i},\quad
 z_g=G_{b_{z,g},m_{A,g}}\hat z_g.
$$

Partial $e$ recomposition uses the fixed extension basis where applicable, not an untyped ring scalar. In coefficient packing,

$$
 e_{g,c,i}(Y)=\sum_{j<d_{{\rm car},g}}
 \sum_{t<k}\beta_t\sum_{\ell<\delta_{{\rm open},g}}
 b_{1,g}^{\ell}\hat e_{g,c,i,\ell,t,j}\,Y^j.
$$

Trace partials use the corresponding full A-ring coefficient expansion.

The canonical native relations, Akita Figure 6/Eqs. (138)–(142), are:

$$
\begin{aligned}
\sum_{c,i}c_{g,c,i}(Y)e_{g,c,i}(Y)
 &=\mathcal L_g(G_{b_g,M_g}z_g)(Y)
 &&\text{in }E[Y]/(Y^{d_{{\rm car},g}}+1),\\
\sum_{c,i}c_{g,c,i}(X)e_{g,c,i}(X)
 &=a_g^\top G_{b_g,M_g}z_g(X)
 &&\text{in }R_{d_{A,g}}\quad\text{(trace alternative)},\\
\sum_{c,i}c^A_{g,c,i}t_{g,c,i}&=A_gz_g
 &&\text{in }R_{d_{A,g}}^{n_{A,g}},\\
 B_gx_{B,g,s}&=u_{g,s}&&\text{for every original slice},\\
 D(\hat e_0\|\cdots\|\hat e_{G-1})&=v_D.
\end{aligned}
$$

The two first equations are alternatives, not simultaneously imposed modes. $x_{B,g,s}$ uses the **original** polynomial-major slice input and its original per-polynomial padding. The B-stack is in slice/row/coefficient order. One does not reslice an old handle using the receiver's new width.

For either B/F or D/H path, with original map $M$, chain maps $K_j$, terminal payload $p$, and binary digit vectors $\hat\xi_j$, the exact compression equations are

$$
 M\hat x=G_{\{-1,0\},1}\hat\xi_1,\qquad
 K_j\hat\xi_j=G_{\{-1,0\},j+1}\hat\xi_{j+1}\;(j<L),\qquad
 K_L\hat\xi_L=p.
$$

At $L=0$ use $M\hat x=p$ and no chain variables. At $L=2$ check every chain digit's $w(w+1)=0$ as well as its full-span range. $G_{\{-1,0\},j}$ here denotes the scheduled signed-binary recomposition at that stage, including ring-completion padding; it is not a new matrix commitment.

The scalar opening row is, in packing mode,

$$
 \sum_{g,c}\vartheta_{g,c}\,
 \omega_{{\rm dir},g,c}^{\top}\hat e_{g,c}
 =\sum_{g,c}\vartheta_{g,c}\phi_{g,c}v_{g,c},
$$

where $\omega_{\rm dir}$ has the block equality weight, tail equality weight, extension-basis coefficient and gadget power of Akita Eq. (136). In trace mode it is

$$
 \sum_{g,c}\vartheta_{g,c}\theta_g
 \sum_{i<B_g}\chi_{{\rm blk},g}(i)
 \mathcal T_{\rho_{{\rm pack},g}}(e_{g,c,i})
 =\sum_{g,c}\vartheta_{g,c}h_{g,c}.
$$

$\mathcal T$ is Akita Eq. (133)'s trace functional; scalar projections cannot be multiplied as if they were full ring evaluations. In either case this scalar row is already E-linear and has **no quotient**.

### 5.2 Quotient-lift and quotient-free equations

For every active ring row $r$, record its native coefficient field $F_r\subseteq E$, native dimension $d_r$, and each witness occurrence address $\mathsf{addr}_{r,c}(j)$. Put

$$
 W_{r,c}(X)=\sum_{j<d_r}W_{\mathsf{addr}_{r,c}(j)}X^j.
$$

With scheduled public multipliers $A_{r,c}$ and target $Y_r$, quotient lifting checks

$$
 \sum_cA_{r,c}(X)W_{r,c}(X)-Y_r(X)
 =(X^{d_r}+1)Q_r(X),\quad \deg Q_r<d_r,
$$

by evaluating the ordinary-polynomial identity at $\alpha\leftarrow E$. Every $Q_r$ is already inside the successor commitment before $\alpha$. Its digit layout is digit/field-basis-coordinate/native-ring-coefficient; carrier quotients have $k$ base-field planes but still modulus $Y^{d_{{\rm car},g}}+1$.

In particular, packing fold evaluation uses $c_i(\alpha)$ in the carrier equation, while the A equation uses $c_i(\alpha^{kh_g})$ in the ambient ring. They are not interchangeable unless $kh_g=1$. Each B/D/F/H equation uses its own modulus factor, not a common largest ring modulus.

Quotient-free checking instead uses

$$
 \kappa_{A,\alpha}^{(d)}(j)
 =\bigl(A(X)X^j\bmod(X^d+1)\bigr)(\alpha),
$$

and the residual

$$
 \sum_{c,j}\kappa_{A_{r,c},\alpha}^{(d_r)}(j)
 W_{\mathsf{addr}_{r,c}(j)}-Y_r(\alpha)=0.
$$

This accounts for negacyclic wraparound; substituting $X=\alpha$ into unlifted products is wrong. For example $X^3\cdot X=-1$ in $R_4$, not $\alpha^4$. On this route every quotient segment is empty, but **all** compression, scalar, tensor, range and norm obligations remain.

The fused sumcheck's final public check is the evaluation of Akita Eq. (160):

$$
 V_{\alpha,\tau_1}+\gamma s_{\rm claim}
 =\sum_{x\in\{0,1\}^{\mu'}}\left[
 \widetilde W(x)m_{\tau_1}(x)
 +(\gamma\widetilde{\rm eq}(r_{\rm virt},x)
 +\zeta_{\rm bin}\widetilde{\rm eq}_{I_{\rm bin}}(r_{\rm virt},x))
 \widetilde W(x)(\widetilde W(x)+1)\right].
$$

The relation term does not acquire the range term's equality factor. Its final sumcheck point and declared $v'=\widetilde W(r')$ are public outputs, but authenticating this last value is the unfinished successor opening.

### 5.3 Complete Eq. (109) geometry

Use concatenation, not multiplication, in the following display:

$$
\begin{aligned}
 W_{\rm main}&=\mathop{\|}_{g=0}^{G-1}
 [\hat z_g\mid\hat e_g\mid\hat t_g],\\
 \hat c_a&=(\mathop{\|}_g\hat u_{g,a})\|\hat v_a,\\
 W_{\rm next}&=W_{\rm main}\|\hat Q_{\rm main}\|
 \mathop{\|}_{a=1}^{L_{\rm comp}}[\hat c_a\mid\hat Q_a].
\end{aligned}
$$

`Q_main` lists, per group, fold-evaluation rows, A rows, sliced B rows, then the shared D rows. At each compression depth all group F rows precede the shared H rows. Each quotient remains at its row's native dimension and coefficient field. Inner-image and partial digits are split into native B and D coefficient blocks **before** decomposition; coefficient changes first, then digit, then native block. Response order is position/input-source-digit/response-digit/A coefficient. Actual digit and block counts are not rounded to powers of two.

Add the scheduled alignment zeros between the main/quotient body and compression suffix, at each stage, and the terminal ring-completion/Boolean-domain zeros. These are committed coordinates. Main segments are **not** individually padded to the largest native ring dimension. The exact receiver adds its declared zero suffix to reach $N_*$; a live-domain certificate must enforce required zeros rather than infer them from an honest prover convention.

The base main length is

$$
 L_{\rm main}=\sum_g\left[
 m_{A,g}d_{A,g}\delta_{f,g}
 +P_gB_g\delta_{{\rm open},g}
 (d_{{\rm op},g}+n_{A,g}d_{A,g})\right].
$$

Every quotient, both F/H stages, ring completion, alignment and final padding belongs in the receiving fixed-point calculation. This is the exact contract supplied to the closure obligation; this document does not assert a numerical feasible fixed point.

## 6. Backward-link lemma and collision classes

### 6.1 Tree-conditioned native lemma

**Source-backed local lemma (Akita Theorem 10.23).** Fix an admissible fold and a descendant-certified nested coordinate-wise accepting family. Every fold child must include the complete authenticated strong-check/recursive subtree, not only its asserted output tuple. Then extraction returns either:

1. weak openings of **every original incoming group**, satisfying its commitment path, native partial equations, scalar evaluation equation and certified products; or
2. a nonzero short kernel vector for a scheduled physical A/B/D/F/H view at the correct collision radius.

Reason: authenticate the outgoing source under each child; extract its range/norm/field checks and interpolate back to every exact native-ring equation. Compare the fixed original B/F paths and fixed shared D/H path across children. Any difference in their certified digits gives a matrix collision. Otherwise the digits and partials agree. For the coordinate $(g,c,i)$, subtract siblings that differ only there:

$$
 \iota_g(\Delta c_{g,c,i})s_{g,c,i}
 =z_g^{(1)}-z_g^{(0)},\quad
 A_gs_{g,c,i}=t_{g,c,i},\quad
 e_{g,c,i}=\mathcal L_g(G_{b_g,M_g}s_{g,c,i}).
$$

The same unit difference can be cancelled in the scheduled opening carrier and ambient ring. The common outer/compression digits authenticate those $t$ values to the original handle. Scalar and tensor reductions recover the original full-grid evaluations at their original points. No later challenges need agree between the two fold children.

**Not proved here:** an arbitrary one-shot accepting `AccV` prefix plus an existential successor witness does not hand the extractor this counterfactual family. To turn the proposed recurring B interface into an accumulation reduction, prove an adaptive extractor that supplies all required successor witnesses without recursively unfolding the entire history. Nor does the lemma upgrade weak originals to canonical short original sources. The final bounded-domain predicate, source projection and extractor runtime must match the extraction obligation.

### 6.2 Original-versus-receiver comparison classes

Let occurrence $u$ record an exact commitment shape/view and weak certificate $r_u=\rho_us_u$, with bounds $K_{1,u}$ on $\|\rho_u\|_1$, and $B_{\infty,u}$ on $\|r_u\|_\infty$. For two source extractions of the **same handle**,

$$
 A\bigl(\rho_{u'}r_u-\rho_ur_{u'}\bigr)=0,
$$

and distinct $s_u,s_{u'}$ make the cleared vector nonzero because both denominators are units. Its coefficient bound is

$$
 \eta_{A,C}=\max_{u,u'\in C}
 [K_{1,u'}B_{\infty,u}+K_{1,u}B_{\infty,u'}].
$$

Use the corresponding minimum of available certified Euclidean pairings from Akita Eqs. (194)–(195) only when **every ordered pair**, including the diagonal, has the required bounds. Do not assume centered coefficient reduction contracts a convolution operator norm.

For a B/F view $M$ shared within the class,

$$
 \eta_{M,C}=\max_{u,u'\in C}
 \max_{x\in\mathcal D_{M,u},y\in\mathcal D_{M,u'}}
 \|x-y\|_\infty.
$$

The ledger needs: original/original, receiver/receiver, original/receiver **when the same handle is admitted under both profiles**, and canonical-reference comparisons for setup or explicitly canonical source checkpoints. Different handles are not equated simply because their matrices overlap. Compression layers with signed-binary inputs have diameter one when both compared occurrences enforce the same complete binary span. D/H comparison is opening-occurrence-local: those paths legitimately change with point, but must be fixed across children of the same prechallenge payload.

A receiver's stronger/larger certificate may enlarge an old handle's required collision radius. Its rank was frozen at commitment time; a later rank choice cannot retroactively strengthen that original handle. Reject such cross-profile use unless the original profile already priced the mixed class.

All views read prefixes of the shared coefficient stream, rather than independently sampled matrices. Akita §4.3/Eq. (66) and §10.2's stacked-prefix reduction require an inventory of exact $(n,m,d)$ interpretations and radii. A shared seed does not justify identifying distinct ring lanes or applying an independent-matrix reduction. Original profile maps remain original; the receiver's new source has its own view.

### 6.3 Explicit independently certified successor alternative

The preceding preferred final-only deferral route leaves a counterfactual-successor authentication obligation. An alternative pays for a complete fixed-capacity receiver certificate **at every update**:

$$
\beta_j:\quad R_{\rm dom}(P_\*,C_{j+1},r_{j+1},v_{j+1};W_{j+1},\mathsf{st}_{j+1}).
$$

This is one joint relation: the bounded source, its full-grid domain and the evaluated polynomial must belong to the **same exact already-bound receiver handle**. The certificate includes the original receiver A/B/F equations, the identity-source/full-grid decode, every alphabet/zero predicate and $\widetilde W_{j+1}(r_{j+1})=v_{j+1}$. Bounds-only certification is insufficient. So is range certification of an unrelated fresh commitment. The domain/profile, coefficient ordering, setup views and history/boundary fields remain authenticated exactly as above.

A completely specified, conservative realization is the clear certificate

$$
\beta_j=(W_{j+1},\mathsf{st}_{j+1}).
$$

The verifier checks the exact source equations in §5.1 and all A/B/F recomposition/compression paths under $C_{j+1}$, recomputes the receiver's full-grid decode, checks every coordinate alphabet and required zero, and evaluates its MLE at the derived point. It rejects on any mismatch. This is the §3.3 conservative decider run at each update, rather than only at the end. No opening proof is left inside this certificate. Witness extraction is the deterministic reading of the supplied certificate. Its communication/state size and verification work are the complete fixed receiver witness and commitment path, at least $\Theta(N_\*)$; its costs are not the small prefix-only costs. It reveals the recursive witness and may reveal encoded application information, so it is not a private-aggregation realization.

Require the successor commitment at its usual pre-$\alpha$/row-challenge position; derive $(r_{j+1},v_{j+1})$ from the native strong checks; receive and verify $\beta_j$ before returning an accepted update. Use the output type

```text
AcceptCertifiedOrdinary(
    OrdinaryState(P_*,C',N_*,r',v',domainTag,t',epoch',h',boundary'),
    beta,inputDigest,prefixDigest)
```

instead of claiming that the unchanged `AcceptPending` API verified this additional relation. `OrdinaryState` here is fully certified as a receiver evaluation/domain state, although certification of the fresh original statements still uses the original-handle backward link. If the live old state itself carries a certificate, the next verifier can check it directly in the same fixed-size work; it need not recursively replay its history.

For every accepting counterfactual child in a local native extraction tree, its own verifier acceptance now supplies an independently checked successor witness. Same-outgoing-handle source-projection uniqueness reconciles the certificates across the post-fold strong-check children. The original-handle B/F rows and shared D/H links then supply the descendant-certified folding family for Theorem 10.23. Outgoing handles may differ across folding-challenge children; each has its **own** valid certificate, so no false common-projection assumption is made. Collision budgets still include certified/certificate-source comparisons, mixed original/receiver/reference views and the certificate source alphabets. This addresses the missing descendant-witness interface without recursively constructing the suffix for each child.

There are two precise limitations. First, applying Theorem 10.23 to the new topology and composing the resulting local extractor adaptively remain explicit theorem obligations; the clear certificate gives genuine witnesses, not a theorem automatically covering a new concurrent/ROM experiment. Second, this alternative is **not ideal final-only deferred PCS verification**: it verifies the entire receiver relation at every step. A compact or private proof can replace the clear certificate only after specifying an actually complete, sound, expected-polynomial-time extractable proof for this exact relation, with its own transcript order, errors, setup and costs. Merely naming a full receiver proof, assuming a generic oracle that returns its witness, or importing source uniqueness without an extractor is not such an implementation or reduction.

## 7. Exact public reducibility and the Candidate A audit

### 7.1 Terminal authentication and the corrected indexing

Akita §8.2 has **one terminal recursive-witness group**, one clear vector $z$, one partial $e_i$ and inner image $t_i$ per actual source block $i$. Its direct equations are

$$
\begin{aligned}
 A_{\rm term}z&=\sum_{i<B_{\rm term}}c_it_i
 &&\text{(163)},\\
 \sum_i c_i(X)e_i(X)&=a_{\rm term}^{\top}G_{b_{\rm term},M_{\rm term}}z(X)
 &&\text{(164)},\\
 \theta_{\rm term}\sum_i\chi_{{\rm blk},\rm term}(i)
 \mathcal T_{\rho_{{\rm pack},\rm term}}(e_i)
 &=v_{\rm tensor}&&\text{(165)}.
\end{aligned}
$$

The predecessor binds the exact canonical inner image state $(t_i)$ as its outgoing state; the terminal reabsorbs it, performs its tensor prefix, absorbs clear $(e_i)$ **before** its nonce/fold challenges, absorbs the decoded $z$, then checks these equations and exact scheduled encoding/norm limits. There is no terminal D/H payload, B row, quotient witness, relation sumcheck or new commitment. This absence is sound only at the authenticated `PendingInner` interface.

The predecessor note's $A_{\rm term}z_i=b_i$ notation can be read as indexing **completed proofs**. It is not a family of independent block responses. For an unambiguous split, write proof index $j$ and block index $i$ separately:

$$
 x_j=z_j,\qquad y_j=\sum_{i<B_{\rm term}}c_{j,i}t_{j,i}.
$$

**Audit conclusion:** the algebraic Eq. (163)-only split is supported, not disproved. The real correction is to make the proof/block indexing, authenticated terminal state and exact pending-output contract explicit. Do not expand the claim into deferring Eq. (164), the trace equation, arbitrary nonterminal A/B/D rows or whole original PCS verification without another exact split.

### 7.2 Typed public reduction

For an admitted finite catalogue $\mathcal L$ of **exact** terminal views, define

$$
 \mathsf{ReducePublic}(P,u)\to
 \mathsf{Reject}\;\text{or}\;
 (\mathsf{RetainedOK},\mathsf{InputId},
  (\ell,x_\ell,y_\ell)_{\ell\in\mathcal L},
  \mathsf{Boundary}).
$$

`RetainedOK` means **every original verifier check other than the listed Eq. (163) comparisons** passed. It includes all frontend/prefix/fold checks, predecessor inner-state authentication, setup discharge, original challenge/filter/nonce rules, canonical proof decoding, response bounds, Eqs. (164)–(165), and complete proof-stream consumption. It does not mean the original proof passed.

The exact identity is

$$
 V_{\rm full}(P,u)=V_0(P,u)\land
 \bigwedge_{\ell\in\mathcal L}A_\ell x_\ell(u)=y_\ell(u).
$$

For a proof using one lane, the other lanes contribute explicit zero vectors under the catalogue; a mandatory missing terminal record rejects. Payloads/ring vectors are derived from the decoded original proof and replayed transcript, not supplied as free metadata. Each lane includes exact ring dimension, matrix geometry, setup view, terminal shape and coefficient encoding. The common public arithmetic extension $K/F$ gives lane algebra $K[X]/(X^{d_\ell}+1)$, without a new SIS opening claim.

`AccV_A` replays `ReducePublic`, authenticates the complete old state and input before a fresh $\alpha\leftarrow K^\times$, then derives

$$
 U'_\ell=U_\ell+\alpha x_\ell,\qquad
 Y'_\ell=Y_\ell+\alpha y_\ell.
$$

It returns the **derived** successor public residual state, lane catalogue identity, checked-prefix/input/history bindings and updated boundary. The exact pending predicate is

$$
 D_A(a')=\bigwedge_\ell A_\ell U'_\ell=Y'_\ell.
$$

There is no opening witness or response norm for the public folded state. Each original $z$ still has its own retained norm/encoding check. A final receipt proves/replays $V_0$, challenge/update arithmetic, original frontend/boundary composition and final $D_A$; a hash chain alone does not prove these checks.

Let $R_\ell=A_\ell U_\ell-Y_\ell$ and $d_\ell=A_\ell x_\ell-y_\ell$. The successor residual is $R_\ell+\alpha d_\ell$. If the old state or input is false, there is at most one nonzero scalar that makes every lane vanish (or none). Thus one-step bad cancellation is at most $1/(|K|-1)$ for fixed prechallenge data. A length-$T$ interactive path has union bound $T/(|K|-1)$ for the first bad-to-good step. A Fiat–Shamir deployment needs a stated global query/grinding argument; it is not justified by calling deterministic hash outputs independent. This arithmetic lemma has no growing native extraction tree.

### 7.3 Source cross-check and scope

The inspected local source confirms the proposed split:

- [terminal dispatch](../../crates/akita-verifier/src/fold/terminal.rs), lines 26–47: exact witness length, no pending setup, one group, predecessor state reabsorbed;
- the same file, lines 107–156: clear partials precede fold challenge replay; clear response goes to direct ring checks;
- lines 157–173: separate trace check with the tensor factor/target;
- [direct checks](../../crates/akita-verifier/src/terminal/direct.rs), lines 75–127: A-row computation compares matrix product of clear $z$ with challenge-weighted $t$; lines 225–284 independently check fold evaluation and A rows.

This is evidence about the **inspected working-copy source**, not an assertion about a pinned companion commit. A fetch of the known companion pin `d0ee56a0b12db66b4c46a87c527169737694c4c7` at the terminal-dispatch path returned HTTP 404. No pinned code equivalence, runtime behavior or implementation of `ReducePublic` was established. Removing only the explicit A-row comparison while keeping its decoding, bound checks and the independent consistency/trace checks describes the research split; no source was changed.

## 8. Fresh recommitment: actual reductions, costs and the unsound shortcut

This section is an alternative, not an assumption inside the native theorem.

### 8.1 Counterexample

Suppose an original $C$ is bound to constant polynomial 1 and the claim is $(C,r,0)$. An aggregator instead commits to zero as $C^*$ and supplies a valid opening $(C^*,s,0)$. Every assertion about the new zero polynomial can be true, while the original claim is false. Unless the protocol authenticates a relation to $C$, no matrix collision or PCS forgery occurs. A range proof on the new zero polynomial changes nothing.

Likewise, evaluating all polynomials at an unrelated common point is not alone a reduction of their original claims: generally $f_1(r_1)+f_2(r_2)\ne(f_1+f_2)(s)$.

### 8.2 Fully checked generic LCS

Assume an admitted common full-grid domain with $\mu$ variables, original-handle source uniqueness for that domain, and a commitment profile capable of representing the fresh linear combination. First bind original $C_i$, coefficients $\gamma_i\in F$, and a fresh

$$
 C^*=\mathsf{Commit}(f^*),\qquad f^*=\sum_i\gamma_i f_i.
$$

Then sample independent $\tau\leftarrow F^\mu$, receive $b_i,b^*$, and **fully verify** original-handle PCS openings $(C_i,\tau,b_i)$ and $(C^*,\tau,b^*)$, including their terminal discharges. Check $b^*=\sum_i\gamma_i b_i$. There is no unchecked Eval proof inside this consistency check. Under uniqueness/binding to polynomials fixed before $\tau$, an incorrect multilinear identity passes with probability at most $\mu/|F|$, plus the original evaluation/binding errors and the relevant query losses. This is the generic method discussed after Halo Definition 3.3, specialized here to multilinear vectors; it costs a full batch discharge, not a free consistency operation.

A field-linear combination may leave the small live alphabet. Recommitting it as a bounded receiver requires a separately sound range/decomposition transformation and its original-handle link. If coefficients are in $E\setminus F$, explicitly represent extension coordinate vectors and certify a compatible commitment interface; the current clear-only base-field adapter does not supply it. This alternative is therefore not claimed to satisfy the recurring $R_{\rm dom}$ profile.

### 8.3 Same-point evaluations and exact deferred state

If all original claims use $r$, bind their $v_i$ before fresh independent combination coefficients. The checked LCS relates $C^*$ to that combination. Leave exactly

$$
 \mathsf{PendingEval}(C^*,r,\sum_i\gamma_i v_i).
$$

For fixed original residuals $f_i(r)-v_i$, uniform independent coefficients hide a nonzero residual with probability at most $1/|F|$ (or $(n-1)/|F|$ for successive powers of one scalar). Together with the fully checked LCS and a sound final opening, this gives the conditional backward evaluation reduction. The pending state's witness is the **linked** $f^*$ and its actual opening hints. It is not a completed-proof public aggregation scheme, nor yet a bounded recurrent state.

### 8.4 Different-point evaluations and actual remaining list

For differing $r_i$, use a fresh coefficient vector $\beta$ after the commitments, points and values are fixed, and run degree-two sumcheck on

$$
 \sum_{x\in\{0,1\}^\mu}\sum_i
 \beta_i\operatorname{eq}(r_i,x)f_i(x)
 =\sum_i\beta_i v_i.
$$

At the final common point $s$, receive values $y_i$ and check the terminal sumcheck value against

$$
 \sum_i\beta_i\operatorname{eq}(r_i,s)y_i.
$$

The **actual deferred state is the list**

$$
 ((C_i,s,y_i))_{i=1}^n.
$$

Its remaining evaluations cannot simply be replaced by $(C^*,s,\sum_i\gamma_i y_i)$, because the weights $\beta_i\operatorname{eq}(r_i,s)$ are point-dependent and generally unequal to the fixed LCS coefficients. Even a checked $f^*=\sum_i\gamma_i f_i$ does not eliminate that list. A claimed polynomial $\sum_i\beta_i\operatorname{eq}(r_i,X)f_i(X)$ has individual degree two; treating it as the multilinear interpolation of its Boolean values and evaluating the product formula at $s$ is another invalid shortcut.

To finish a one-state construction, either fully verify the list (leaving no opening deferred), or pass **all its original handles** into the native protocol of Sections 4–6 and return its precise `PendingOrdinary` output. In the latter case the final list-authentication step has the same conditional native extractor/domain obligations already identified; no theorem is gained by renaming it LCS. In an interactive unique-source model the extra error is at most $1/|F|$ for independent claim coefficients plus $2\mu/|F|$ for this degree-two sumcheck, before native/PCS errors. The native fold must have capacity for this list. General domain embeddings and padding-image semantics must also be certified.

### 8.5 What LF+ contributes, and what it does not

[LatticeFold+](SOURCE_PROVENANCE.md#latticefoldplus), §§4.1, 4.4, Construction 4.5, gives a real example of solving the hidden-image problem rather than assuming homomorphism. For its double commitment $C_{M_f}=\mathsf{com}(\tau_D)$, with $\mathsf{pow}(\tau_D)=\mathsf{com}(M_f)$, it sends the challenge-folded inner image $\mathsf{com}(h)=\mathsf{com}(M_f)s'$. Its sumchecks include

$$
 \langle\mathsf{tensor}(c^{(z)}),
 \mathsf{pow}(\tau_D)s'\rangle
 =\langle\mathsf{tensor}(c^{(z)}),\mathsf{com}(h)\rangle
 \quad(z=0,1),
$$

and linked evaluations of $(\tau_D,m_\tau,f,h)$ at the output points. It returns a linear commitment to

$$
 g=s_0\tau_D+s_1m_\tau+s_2f+h
$$

with **two** remaining evaluation claims, one at each independently sampled sumcheck point. Those output evaluations are part of its reduction relation, not already checked openings. Its relaxed-binding bounds and range relation are essential, and §5.2 separately decomposes the enlarged norm back to two bounded instances. The LF+ relation even allows valid noncanonical $\tau$ because `pow` is not injective; cite its Remark 4.1 rather than assuming canonicality.

An Akita-to-LF+ bridge would have to encode the actual original B/F path into this consistency relation, preserve source geometry and range assumptions, and consume the resulting output tuple. Merely applying LF+ to a new commitment of an unrelated vector does not provide that bridge. Native Akita already supplies explicit original A/B/F/D/H rows, so it is the preferred consistency layer here.

## 9. Public versus private aggregation and remaining obligations

Halo Infinite Definition 4.1's private aggregator has polynomials **and opening hints**. Definition 5.1's public aggregator receives completed noninteractive proof bytes, does not know original openings, and must derive a witness for its output from publicly available proof structure. Its Theorem 5.2 compiles an additive PCS into a publicly aggregatable homomorphic PCS using extra homomorphism-preimage structure. It does not say that an efficient nonhomomorphic LCS alone makes arbitrary existing proofs publicly aggregatable. Section 6 distinguishes this public accumulation interface from private accumulation carrying private state.

Thus:

| Interface | Input availability | Exact output | Classification |
|---|---|---|---|
| Native B re-entry | Original sources/opening hints plus bounded old live state | Pending receiver opening, optionally an explicit registered setup claim | Private; new recurrent theorem required |
| Generic checked LCS | Original polynomials and hints; original-handle openings fully discharged | Linked fresh handle, then listed pending evaluation(s) | Private; expensive, bounded closure not implied |
| Public A residuals | Completed canonical proof bytes, original statements and replayed verifier transcript | Public $(U_\ell,Y_\ell)$ state | Public arithmetic accumulation, not an Akita opening |
| Halo public compiler | Compiler-specific NI proof residuals and unrestricted compatible hint/group structure | Publicly derivable output opening witness | Source theorem; not an instantiated Akita result |

### Costs and budgets

- Native per-step work includes all frontend/history update checks, original-handle relation/range reductions, formation of all Eq. (109) coordinates and public-weight work. Direct setup evaluation can retain a large setup scan. No speedup or concrete feasible receiver is asserted.
- Native $L_{\rm main}$ and the exact D width above expose the costs the closure obligation must price; additional Q/F/H/zero coordinates are not optional overhead that may be omitted from the witness.
- For fixed bound polynomials/witnesses, quotient-free ring reduction plus row batching has one-shot error $(d_{\max}-1+\lceil\log_2n_{\rm rel}\rceil)/|E|$. Quotient lifting has ring-evaluation error at most $(2d_{\max}-1)/|E|$, before row batching/fused checks. Powers batching contributes $(\Lambda-1)/|E|$; normalized independent batching contributes at most $1/|E|$. These **are not** the full adaptive native knowledge error.
- Akita's native extraction ledger charges each fold coordinate, tensor reduction, range stage, norm merge, row batch, setup-product and terminal separately. Eq. (225) multiplies the actual extraction-tree factors. Repeating a nontrivial fixed fold $T$ times can yield exponential tree size. Original-handle consistency does not fix that runtime problem. A new local/adaptive extractor is a distinct obligation.
- A finite admitted topology and $T_{\max}$ must budget accepted-family cardinalities, original commitment ranks, all comparison-class SIS radii, and a global transcript-query/nonce allowance. ROM query losses, setup-expansion distance and matrix-view MSIS advantages are separate terms. Nothing here proves QROM security or accumulator privacy.
- Akita Corollary 10.34, Eqs. (230)–(231), limits the declared **classical-ROM** strength to $\lambda_{\rm FS}^{\rm eff}=-\log_2((Q_{\max}+1)\varepsilon_{\rm int})$. If the native ledger contains $c/|E|$, then $\lambda_{\rm FS}^{\rm eff}\le\log_2|E|-\log_2c-\log_2(Q_{\max}+1)$. Enlarging only public accumulation field $K$ improves residual cancellation, **not** the original fp128/native-evaluation error ceiling. A higher target may require a newly provisioned original PCS field/profile, not an opening-time change to already committed handles. The quantum lattice-attack cost target does not turn this classical extraction theorem into a QROM theorem.
- Akita §12.2 requires the complete Eq. (109) successor, compression and pending setup obligations, padded widths and security ranks to be derived **before** candidate costs are scored. A smaller response alone is not a cheaper or secure receiving profile.
- Generic LCS costs full linked opening verification plus new commitments; differing-point sumcheck still leaves $n$ original evaluations until they are actually consumed. It is a correctness fallback, not a justified optimization.
- Public A stores $\sum_\ell(m_\ell+n_\ell)d_\ell[K:F]$ base-field coordinates, independent of $T$ but not necessarily small. Each update still runs nearly the full verifier and reads public responses. It saves only the explicitly deferred matrix equations.

### Explicit remaining theorem gates

1. Certify the new old-plus-fresh topology, fixed receiver and source-range/zero predicates, respecting the actual opening-method policy and original commitment security floors.
2. Establish all-valid-$R_{\rm dom}$ completeness (or explicitly choose a weaker generated-history contract), including encodability and bounded response-sampler failures.
3. Prove a modular adaptive native extractor, with original-handle source uniqueness and the exact comparison classes above, whose runtime is polynomial in total admitted path length rather than the unfolded fork tree.
4. Establish frontend/application witness extraction and authenticated receipt composition. A source-consistent PCS link is not a theorem about CPU execution or cross-segment memory.
5. For public A, realize the exact typed parser/verifier split and history receipt without dropping retained checks. For any wider public residual catalogue, separately prove equivalence to the original verifier before claiming it deferred.

The algebraic/native local link is a useful solution to commitment consistency. It is not a claim that these remaining gates have already been discharged.

## 10. References and validation record

### Local paper evidence actually read via qmd MCP

- [Akita](SOURCE_PROVENANCE.md#akita): `lattice-papers/akita.md`; TOC obtained; §4.5 `line:1825-2026` and precise Eq. (69)–(77) grep anchors (the section contains embedded image data); §5.3 `line:2229-2306`; §5.4 `line:2307-2376`; §7.1 `line:2639-2716`; §7.3 `line:2727-2946`; §7.4 Eq. (160) and its pending output; §8.2 terminal text/Eqs. (163)–(165); §10.2 `line:3603-3990` including Theorem 10.23, source projection and comparison classes; `line:3991-4140` for the nested strong–weak cut and errors; Eq. (225)–(227) grep anchors and Remark 10.32. Formula text was read through qmd; damaged displays were reconstructed as mathematical notation and are not presented as verbatim quotations.
  Additional explicit qmd readings: `line:4141-4250` for the complete classical-ROM theorem, runtime and field/query ceiling, and §12.2 `line:4641-4662` for original-geometry immutability and complete-successor-before-scoring rules.
- [LatticeFold+](SOURCE_PROVENANCE.md#latticefoldplus): `lattice-papers/latticefoldplus.md`; TOC, §4.1 `line:332-379`, §4.4/Construction 4.5 `line:634-743`; targeted transformation/security grep. The theorem proof and full decomposition proof were not audited here; claims about their precise instantiated Akita applicability are therefore not made.
- [SuperNeo](SOURCE_PROVENANCE.md#superneo): `lattice-papers/superneo.md`; TOC, §6 `line:855-936`; targeted uniqueness/source-preservation grep. The §6 definitions and composition statement were read; no Akita-specific adaptive composition was inferred as an existing SuperNeo theorem.

### Primary web evidence actually read

- Boneh, Drake, Fisch, Gabizon, **Halo Infinite: Proof-Carrying Data from Additive Polynomial Commitments**, CRYPTO 2021, pp. 649–680, [official publication PDF](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_23.pdf), [DOI](https://doi.org/10.1007/978-3-030-84242-0_23). Read Definition 3.3 and generic consistency discussion, Definition 4.1 and §§4.2–4.3, Definition 5.1/Theorem 5.2 and public compiler explanation, and Definition 6.1/private accumulation discussion. The published article's deferred full-version IVC exposition was not read. Its notation is univariate/prime-field; this document's multilinear reductions are labeled explicit adaptations with their own assumptions.

### Source and validation scope

Read the two local terminal verifier files and the relevant predecessor note sections; cross-checked the terminal A/fold-evaluation/trace separation against paper equations. Inspected the exact Eq. (109) segment order, D width and both native ring reductions via qmd. Performed the algebraic subtractions and residual-cancellation argument in this document and recorded the carry, unrelated-commitment, different-point and negacyclic-wrap counterexamples. No implementation, proof generation/replay, tests, scripts, builds, formatter/linter, benchmark, Git branch operation or computational security estimator was run. The attempted pinned terminal-source fetch returned 404, and a general Akita web search did not locate the paper; local qmd paper evidence and the inspected local source are the basis for Akita-specific claims. qmd single-line `doc_read` unexpectedly overran into subsequent sections/base64 data; it was reported and subsequent substantive readings used explicit TOC ranges.
