# AQRA: preserving the original source-plane relation

## Scope decision

The user selected **preserve the base proof relation**, not strengthen it to unrestricted RV64/SDK byte/tape/emulator semantics. Preserve the existing continued algebraic constraints, Main Boolean/support/PC checks and original signed RAM-plane statement. Report the base word/tape/stopping gaps in [the continued frontend certificate](AKITA_RELATION_ACCUMULATION_CONTINUED_FRONTEND_CERTIFICATE.md); do not label this a stronger SDK theorem.

The earlier decoded-only ordinary-decider candidate does **not** preserve the old RAM-plane statement. An ordinary opening of the Horner-decoded polynomial does not authenticate arbitrary plane queries or prove every plane signed-small. This amendment retains Candidate1's polynomial query accumulation and original commitments but needs an explicit original-plane opening view and one final source-range reduction. It is a required implementation capability, not one already supplied by the ordinary point API. No profile reinterpretation, recommitment, extension-field migration or short-inverse assumption is permitted.

## 1. Exact original-plane source polynomial

For original RAM profile d128, positions M32, blocks B16 and inner digit depth26/base32, retain every original A-input block

$$s_{b,p,\ell}(Y)=\sum_{a=0}^{127}s_{b,p,\ell,a}Y^a.$$

Define a Boolean-grid polynomial $F_C$ with low-first axes

$$X=(X_{\rm coeff}^{[7]},X_{\rm digit}^{[5]},X_{\rm pos}^{[5]},X_{\rm block}^{[4]}),\qquad\mu_C=21.$$

Its cube table is $s_{b,p,\ell,a}$ for $\ell<26$ and zero for $26\le\ell<32$. These six slots are absent digits, not discarded physical source cells. Every one of the26 original planes, including plane25, participates. The original commitment profile still describes one logical polynomial with16 address variables: **the view does not change that profile or its A/B/F payload**.

For original Main, source digit depth1 makes the raw coefficient polynomial identical to each of the56 declared source columns. Keep all components and logical16-variable points. Existing frontend Booleanity, source support and first/last-PC obligations remain mandatory; they cannot be replaced by the honest one-hot constructor.

A source explanation is the literal original A-input tuple plus certified B/F path. The final proof proves its prescribed plane membership, not merely a bounded unit-times-source product. Adjacent RAM references and repeated occurrences use the same original explanation after priced source comparison.

## 2. Lift existing decoded claims without losing plane ownership

For an original decoded RAM claim at address point $u$,

$$v=\sum_{a,p,b,\ell<26}\operatorname{eq}(u,(a,p,b))32^\ell s_{b,p,\ell,a}.$$

Its exact public adjoint on the raw grid is

$$\phi_u(X)=\operatorname{eq}(u,X_{\rm address})B(X_{\rm digit}),\qquad B(D)=\sum_{\ell=0}^{25}\operatorname{eq}(D,\ell)32^\ell.$$

Address axes use the declared permutation into coefficient/position/block order. Evaluate $B$ as26 direct terms; do not materialize a large query table or replace the weighted query by one equality point. Native raw claims use $\phi_u=\operatorname{eq}(u,X)$ instead. All scalar claims receive distinct formal $Z$ exponents.

Use **one raw source view per original RAM** in the weighted query relation: both decoded and plane obligations reduce to the same $F_C$. This avoids reopening the same original under incompatible fake profiles. Main remains56 actual components. Cylinder-source/zero-supported-query transport is as in the main protocol, now with query dimension at least21. The final sumcheck returns one original-plane endpoint per RAM and every Main component endpoint.

## 3. Final signed-plane reduction before query seal

After all original source references are frozen, but **before the query-index challenge $a$**, prove the complete original RAM plane predicate. Its terminal leaves are appended to the deferred query relation before seal. Do not append obligations after $a$.

One global range instance can combine the unique original RAM sources. Let $k=\lceil\log_2N_{\rm RAM}\rceil$ and define

$$G(I,X)=\sum_{C}\operatorname{eq}(I,\operatorname{index}(C))F_C(X).$$

Absent source-index buckets have virtual value zero. This is an uncommitted deterministic view of the original sources, not a new lattice commitment. On the Boolean cube, require

$$\prod_{j=-16}^{15}(G(I,X)-j)=0.$$

For the base field q>32, this proves unique field images of signed integers[-16,15]. It imposes no canonical Horner representation and retains legal noncanonical original planes. Balanced planes do not by themselves prove unsigned64-bit decoded words; that stronger scope was not selected.

Reuse the existing balanced-range product/helper topology with $U=G(G+1)$ and $P_{32}(U)=\prod_{j=0}^{15}(U-j(j+1))$. The product-tree helpers are deterministic on-cube functions of $G$. Its final virtual $[U](u)$ must be linked to the original sources by a degree-three-per-variable sumcheck

$$[U](u)=\sum_{x\in\{0,1\}^{k+21}}\operatorname{eq}(u,x)G(x)(G(x)+1).$$

At its terminal $z=(z_I,z_X)$, supply **every** $p_C=F_C(z_X)$ and check $G(z)=\sum_C\operatorname{eq}(z_I,\operatorname{index}(C))p_C$. Append all these original raw endpoint claims. A virtual range-image scalar is not itself an authenticated source opening. The whole range DAG needs first-divergence proof and its actual finite error ledger; neither a terminal free scalar nor an off-cube identity $[U](u)=G(u)(G(u)+1)$ is valid.

The [independent global-range review](AKITA_RELATION_ACCUMULATION_GLOBAL_ORIGINAL_RANGE_REVIEW.md) closes this fixed-source proof. Use the actual normalized equality-factored degree-four product driver; bind its four quartic child claims before the powers-of-$\gamma$ batch; replay the degree-four normalized leaf; then run the standard cubic link above. Do not mix normalized claims with unnormalized terminal equality factors. For $n=21+k$, the exact conservative local ledger is $(12n+3)/|K|$; $(14n+3)/|K|$ is a valid looser bound. The earlier range source cut is independent of the later query cut: retain both its additional expected work and its old-query loss, without assuming signed range during weak-certificate source comparison.

Place source-index axes after the existing low-first raw axes in the concrete adapter. The six absent digit slots are interior holes per position, not one trailing physical prefix. Although absent $G$ and $U$ are zero, quartic helpers need their actual nonzero defaults where applicable. Freeze final endpoint ownership/order before $a$; bind their eventual point/value payloads only after the weighted query derives them and before native decider challenges.

A per-source range instance is a correct alternative if its ledger and costs are explicit. Global reduction is the initial choice to avoid duplicating repeated boundary RAM aliases. Receipt state can grow; no4MiB limit or completed per-segment PCS input is introduced.

## 4. Registered original-plane native opening view

The final Akita decider must authenticate $F_C(z_X)$ under the original A/B/F tuple. The ordinary decoded16-point API alone cannot do this. Register a distinct view in the complete opening descriptor and trusted row, retaining the exact original profile. Reject an original-plane view on a non-original-plane producer, wrong digit depth, wrong axis order, unsupported packing or arity. Initial original RAM capability is E=K, d128, carrier128, k=h=1.

For raw point $(u_A,u_D,u_P,u_B)$, define the original unit's native partial

$$E_b(Y)=\sum_{p,\ell<26}\operatorname{eq}(u_P,p)\operatorname{eq}(u_D,\ell)s_{b,p,\ell}(Y).$$

Then

$$F_C(u)=\sum_b\operatorname{eq}(u_B,b)\sum_a\operatorname{eq}(u_A,a)\operatorname{coeff}_a E_b(Y).$$

The current decoded view is the same formula with digit weights $32^\ell$. The generalized point preparation stores the authenticated digit-weight recipe separately from the existing address/packing point; it does not pass a21-coordinate raw point to a16-coordinate ordinary constructor.

For native shell coordinates $c_b$ and folded source response $Z_{p,\ell}=\sum_bc_bs_{b,p,\ell}$, the required ring opening row is

$$\sum_{p,\ell<26}\operatorname{eq}(u_P,p)\operatorname{eq}(u_D,\ell)Z_{p,\ell}(Y)=\sum_bc_bE_b(Y).$$

This has the same bilinear native relation as decoded opening, with new public weights. Shared D/H authenticates the actual E partials under the full-field gadget and complete path widths. No new source is committed and source-fold norms depend on the same original $c_b s$ products, not on the unrestricted public query weights.

The [independent native-decider review](AKITA_RELATION_ACCUMULATION_ORIGINAL_PLANE_DECIDER_REVIEW.md) proves this substitution and identifies a mandatory compact-consumer change. Existing geometric digit factors require weight0=1; raw Eq weight0 can vanish. Use exact Eq-pair factors on the physical dyadic cover$[0,16),[16,24),[24,26)$, with high-bit multipliers$1-u_D[4]$, $u_D[4](1-u_D[3])$, and$u_D[4]u_D[3](1-u_D[2])(1-u_D[1])$ respectively. Keep physical26-digit strides and real offsets. Do not divide by weight0, expand the source to32 physical planes or change response/setup gadgets.

In a one-coordinate fork, subtract the two true branch equations. For the same original source/partial payload and point,

$$\rho s=Z'-Z,\qquad\rho E=\sum_{p,\ell}w_{p,\ell}(Z'_{p,\ell}-Z_{p,\ell}).$$

The public position/digit-weight map is **ring-linear**, so unit cancellation gives $E=\sum w_{p,\ell}s_{p,\ell}$. Only then apply the coefficient-tail scalar functional. It need not commute with multiplication by $\rho$; moving that scalar functional through ring cancellation would be wrong. This derives the same original raw endpoint, while weak product/norm certificates and A/B/F coalescence retain their actual radii. Incoming scalar batching must authenticate all unbatched endpoints with its own source cut.

## 5. Complete proof and extraction order

1. Run each continued frontend through the real pre-PCS boundary, retaining every Main/support/PC/program/advice/transition obligation selected by its profile. No per-segment full PCS proof.
2. Freeze original source roster and prove the global original RAM range reduction. Append all raw endpoint leaves and any separately required source predicates.
3. Seal complete context, every unique scalar exponent/adjoint and the original-plane final endpoint roster.
4. Run one weighted query reduction over original source polynomials; no new homomorphic commitment.
5. Run one **original-source-view-capable** Akita batch decider: native root with registered public digit weights, complete shrinking suffix and actual terminal, under the reviewed direct indexed transcript.
6. Independently replay all frontend/history/range/query evidence and the full final decider; verify external expected statement/count and every transition.

Extract final original endpoints first. Coalesce the complete original A/B/F projections with actual product certificates. One sealed query probe yields all old/frontend/raw-range leaf equations on common sources. A separately owned raw-range source cut supplies fixed-source range truth. Continued frontend source cuts supply the original algebraic segment witnesses; exact source/state/context adjacency concatenates them. Raw source range is not inferred from weak unit division or from an honest constructor.

Every added range/helper/query/root site uses the registered v8 request grammar. Apply the actual degree/maximum-atom, old-query, sampler/cutoff and complete runtime-priced Module-SIS terms. Original source payloads/profiles cannot change with a challenge. Existing canonical E=K suffix GRID/LOCAL semantics permit arbitrary in-range real alignment/padding with zero relation weights; a broader zero-padding assertion remains false.

## 6. Required independent review and code gates

The independent [native-view](AKITA_RELATION_ACCUMULATION_ORIGINAL_PLANE_DECIDER_REVIEW.md) and [global-range](AKITA_RELATION_ACCUMULATION_GLOBAL_ORIGINAL_RANGE_REVIEW.md) reviews pass the explicitly conditional amendment and derive its algebra, full path/width obligations and finite source-cut ledgers. It is not a completed implementation or numerical certificate. Implement/migrate the explicit opening view, source/query arity separation, claim layout, retained original partials, real compact/prover/verifier consumers, view-aware planner/catalog/schema and checked driver. Complete suffix/terminal and actual selected-row admission remain mandatory. Do not ship a raw-query placeholder, fake profile or decoded-only fallback.

Adapted Dense upstream benchmark sources can use their existing decoded view; they do not substitute for exercising this original-RAM capability in Akita/Jolt integration. Performance and security remain unclaimed until real source-preserving proofs and tamper checks execute.

## 7. Exercised algebraic fixture

A deterministic arithmetic exercise used the full native128-coefficient,32-position,16-block,26-plane dimensions in the selected prime field. It checked the raw native partial followed by the coefficient-tail functional against the factorized raw MLE, and independently checked all original decoded claims against the powers-of32 adjoint. Signed boundary values$-16,\ldots,15$ satisfied the plane polynomial; $-17$ and$16$ did not. The off-cube example$G(0)=-16,G(1)=15,r=2$ gave$[U](r)=240$ but$G(r)(G(r)+1)=2162$. These observed equalities exercise the proposed algebra, not an implemented cryptographic proof, a native opening kernel or numerical security admission.
