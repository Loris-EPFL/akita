# Quotient evaluation through one ordinary Akita multilinear opening

## 1. Result and exact change

A quotient over a degree-four extension can be opened with **one ordinary multilinear PCS claim**, without the generic $4J^2d_A$ partial-vector compiler. The construction below uses an original commitment to four base-field coefficient planes and a change of evaluation point. Its polynomial identity is proved here. Full outer schedule, field implementation, source and guest admission remain necessary.

**Security-scope correction:** the earlier inherited zero-finding proof was withdrawn after the [ARO audit](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md). The independently reviewed [hidden-curve replacement](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md) now supplies a weaker conditional bound, with Q outside the square root. Use it with this note's enlarged D_* and full source capacity; neither the old formula nor concrete-oracle admission follows from the MLE identity.

This is an alternative to §7 of [the quotient construction](AKITA_INSTANCE_ACCUMULATION_ORACLE_QUOTIENT.md), not a claim that the current K-only Jolt adapter already supports it. Select the **outer** challenge/evaluation field

$$
E_o=F=K[U,V]/(U^2-5,\;V^2-U),
\qquad K=\mathbb F_q,\quad q=2^{128}-2^{32}+22537.
$$

The quotient note proves that this is a field of degree four over K. The **inner** accumulator remains on its reviewed $E_i=K[U]/(U^2-5)$ profile, with T16, its 33 roles and its complete final decider. Changing the outer field is not permission to copy the inner profile into the outer proof. The independent outer $H_o$ must provide this field's exact ideal challenge streams; a short concrete XOF seed is not the same interface.

Akita permits an extension-field evaluation of a base-field full-grid table. Its algebraic batch relation, Eq. (202), and Theorem 10.31 concern that full decoded grid, not canonical short commitment randomness. The new original quotient source must use exactly this full-grid interpretation. No A/B/F commitment is added or scaled, and no PCS verifier runs inside the guest.

## 2. Capacity semantics: permit the whole coefficient table

For a ledger of n>=2 true graph targets, retain

$$
a=m+1024+w,\qquad D_f=ad(n-1),\qquad
Z_D(X)=\prod_{j=0}^{n-1}(X-b_j).
$$

The honest quotient satisfies $\widehat V(g(X))=1+Z_D(X)t(X)$ and $\deg t\le D_f-n$. Choose a provisioned power-of-two coefficient capacity $N_t=2^\ell$ with $N_t\ge D_f-n+1$. If the source layout requires a larger full grid, choose that actual larger capacity and retain it in all bounds. Capacity and decoder are fixed in the source profile before commitment and before beta; an n-dependent capacity is allowed only when that n/profile identity is also bound before beta.

Use an original K-source whose decoded full grid has exactly $4N_t$ elements, ordered with the two plane bits first (least significant in the linear index):

$$
P(h_0,h_1,i_0,\ldots,i_{\ell-1})=c_{i,h},\qquad
h=h_0+2h_1,\quad i=\sum_{j=0}^{\ell-1}2^j i_j.
$$

Define the extracted polynomial using **every** decoded coefficient:

$$
t(X)=\sum_{i=0}^{N_t-1}\big(c_{i,0}+Uc_{i,1}+Vc_{i,2}+UVc_{i,3}\big)X^i.
$$

Thus the adversarial degree cap is $N_t-1$, not $D_f-n$. Honest excess coefficients are zero; no proof of their zero value is required. This deliberately replaces the truncation semantics of the conservative compiler. It is sound only with the enlarged difference-polynomial bound

$$
D_*:=\max\{D_f,\;n+N_t-1\}.
$$

For $N_t=\operatorname{nextpow2}(D_f-n+1)$, $D_*<2D_f$. A larger backend-induced capacity can invalidate that factor-two comparison; use the actual $D_*$. Require $D_*+1\le |F|$ for the interpolation version of the proof and $D_*<|F|$ for a nontrivial root bound.

A physical grid larger than the declared $4N_t$ cannot be ignored by calling it honest padding. Either incorporate that actual full grid by enlarging $N_t$ and its coordinate map, or prove a separate projection/padding reduction. Original source digit recomposition must be the exact linear native decoder.

## 3. The multilinear identity

Let $\widetilde P$ be the F-multilinear extension of P. Fix beta in F such that

$$
1+\beta^{2^j}\ne0\quad(0\le j<\ell).
$$

The two constants $1+U$ and $1+V$ are nonzero: $U\notin K$ and $V\notin E_i$. Define

$$
r_U=\frac{U}{1+U},\quad r_V=\frac{V}{1+V},\quad
r_j=\frac{\beta^{2^j}}{1+\beta^{2^j}},
$$

$$
\kappa(\beta)=(1+U)(1+V)\prod_{j=0}^{\ell-1}(1+\beta^{2^j}).
$$

Then

$$
\boxed{\ t(\beta)=\kappa(\beta)\,
\widetilde P(r_U,r_V,r_0,\ldots,r_{\ell-1})\ }.
$$

**Proof.** For $z\ne-1$ and $b\in\{0,1\}$,

$$
(1+z)\operatorname{eq}\!\left(\frac{z}{1+z},b\right)=z^b.
$$

Multiply this identity for $z=U,V,\beta,\beta^2,\ldots,\beta^{2^{\ell-1}}$ and expand the MLE. Each Boolean index contributes

$$
c_{i,h}\,U^{h_0}V^{h_1}\prod_j\beta^{2^j i_j}
=c_{i,h}\,e_h\beta^i,
\qquad e=(1,U,V,UV).
$$

Summing over the full grid gives the claimed polynomial evaluation. The identity holds for every K-valued grid, including those obtained from noncanonical weak source states. It does not rely on how the honest prover decomposed the coefficients. ∎

## 4. Exact exceptional set and sampler

Let

$$
\mathcal E_\ell=\{\beta\in F:\prod_{j<\ell}(1+\beta^{2^j})=0\}.
$$

The polynomial identity

$$
(X-1)\prod_{j<\ell}(1+X^{2^j})=X^{2^\ell}-1
$$

and the odd characteristic imply that these are precisely the $2^\ell$-th roots of unity other than 1. Since $F^\times$ is cyclic,

$$
|\mathcal E_\ell|=\gcd(2^\ell,q^4-1)-1
=2^{\min(\ell,5)}-1\le31.
$$

Here $q\equiv9\pmod{16}$, so $v_2(q-1)=3$, $v_2(q+1)=1$ and $v_2(q^4-1)=5$. For ell=0 the product and exceptional set are empty, as required.

Use the quotient note's 1024-bit raw-output sampler, but reject beta in $D\cup\mathcal E_\ell$. The first public graph point still contains **all 1024 raw bits**, not the derived field element. Conditioned on acceptance, beta is exactly uniform on the complement. For every accepted beta, its unconditional probability is at most $1/|F|$; this is the quantity used by the adaptive fork/root proof. The one-fresh-output abort probability is bounded by

$$
p_{\rm abort}<2^{-512}+\frac{|D\cup\mathcal E_\ell|}{|F|}
\le 2^{-512}+\frac{n+31}{q^4}.
$$

Retain the quotient note's nonce/prequery and bounded-retry accounting. Do not claim uniformity after conditioning on a successful adversarial fork. The guest and outer verifier check the same sampler, exceptional-set predicate and point derivation.

## 5. Protocol linkage and the single extra claim

1. Freeze the complete outer profile, original shapes/keys, plane order and actual capacity. C_g binds canonical **X**, the archive and every prechallenge graph-defining value; C_t commits the full quotient coefficient grid. Public C_g/C_t must equal the proof's actual original advice/quotient handles with those shapes.
2. Bind the complete C_g/C_t/profile/nonce tuple in the same injective CM input as before. It must really fit m bits. Obtain the raw oracle output and accepted beta. Do not place beta-dependent data into C_g.
3. Derive the public point r and nonzero scalar kappa above. Bind the claimed $t_\beta\in F$ to the outer public statement and the oracle-free guest relation.
4. In the **same complete outer Akita batch**, prove the single ordinary claim
   $$\widetilde P(r)=t_\beta/\kappa(\beta)$$
   against that exact original C_t. The external verifier computes r and the scaled claim from bound public values; it does not trust a privately chosen query descriptor.
5. The guest still checks $v=g(\beta)$ and $\gamma=1+Z_D(\beta)t_\beta$, complete native ledger/archive acceptance, and the exact public accumulator. The external verifier still checks its two Vhat equations. There is no nested PCS verifier in the guest.

One F-valued claim has four K coordinates. This removes the *additional arbitrary-linear-query compiler*; it does not reduce that claim to one K scalar or make the rest of the outer PCS opening free. Standard public application batching, source-path authentication, receiver groups, shared D/H width and the new field's carrier/tensor representation all retain their normal costs. The fixed expected statement and authorized static reference must be preserved throughout.

The order also requires the relevant advice source shape to be known before C_g/beta, rather than selected from a beta-dependent outer trace afterward. An admitted fixed-capacity/profile or a proved shape-invariant source map must discharge that requirement. The current missing outer geometry/guest certificate is not bypassed here.

## 6. Soundness adaptation

Relation binding compares the same original C_g,C_t and frozen profile, projecting C_t to the full N_t-coefficient polynomial. Unequal projected polynomials still yield the quotient note's ordinary B/F or unit-product A collision, without any inverse-norm bound or canonical Commit claim.

After the entire outer extraction, the MLE identity forces the bound scalar to equal that extracted polynomial's value. If even one graph target is false, then

$$
h(X)=\widehat V(g(X))-[1+Z_D(X)t(X)]
$$

is nonzero at the offending domain point, regardless of extra high coefficients, and has degree at most D_*. Apply [the reviewed transport theorem](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md), or corrected (ZF) in [the quotient note](AKITA_INSTANCE_ACCUMULATION_ORACLE_QUOTIENT.md), with

$$
H=D_f+1,\qquad \rho\le D_*/|F|.
$$

H observations determine \(\widehat V\circ g\), whose degree is at most D_f; compare that polynomial against the complete \(1+Z_Dt\) of degree at most D_*. Using D_*+1 observations is safe but more expensive. The replacement has **Q outside the square root**; the earlier inherited bound is not restored.

The exceptional-point rejection only decreases fixed-root-set probability. Count the normalized producer's actual CM/WO activations, terminal observations, hidden-query reduction, strict sampling loss and entire extraction/emulator CPU. Private continuation can reduce actual calls, not virtual work or the outer coupling loss.

For n=1, retain the separate direct protocol: g is constant, t=0, and the second external graph equation authenticates the only target. There is no need to create a quotient source solely to obtain a zero opening. If a common grammar retains such a source, its n=1 scalar is constrained to zero explicitly and its source costs are not hidden.

## 7. What this does and does not remove

The algebraic result removes four separate coordinate occurrences and J generic channels per occurrence. The extra quotient obligation becomes one original full-grid K source and one ordinary E_o=F point/claim. It also removes the truncation-mask compiler by allowing the whole coefficient capacity in the declared degree bound. No claim of homomorphic compressed commitments is made.

It does **not** remove the host's composition/interpolation/division, roughly N_t extension coefficients, original lattice commitment cost, outer source capacity, or the query-accumulator circuit/witness cost. It does not authenticate g-defining advice bytes without the complete source/guest projection. It does not solve the full extraction query/runtime ledger or instantiate the ARO by a concrete hash. Performance parity is unmeasured.

Before admission: support E_o/K of degree four end to end; prove the actual full-grid source/point map and all-valid-source completeness; select and validate the outer schedule, field/ring embedding and all weak-source comparison norms; bind the enlarged degree and sampler profile; prove guest/read adequacy; and close the derived oracle/runtime/SIS bounds. The inspected K-only adapter is not this implementation.

## 8. Evidence

Parent arithmetic checked the displayed identity in the actual q/tower for 63 cases: coefficient capacities 1 through 256 (powers of two), beta=0,1,U,V and three deterministic pseudorandom beta values per capacity, with arbitrary K coefficient planes. Horner evaluation agreed with the scaled MLE in every case. The same calculation found $v_2(q^4-1)=5$. These finite checks supplement the algebraic proof; they are not a PCS/guest execution, security experiment or performance measurement.

Primary source: [Akita](SOURCE_PROVENANCE.md#akita), Eq. (202), Theorems 10.31/10.33 and Remark 10.32; qmd lines 4110–4249 confirm full-grid extraction and its runtime scope. [CCGOS](SOURCE_PROVENANCE.md#supplemental-primary-sources), §§7–8, motivated the original attempt. The [scope audit](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md) and independently reviewed [hidden-curve theorem](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md) replace its disputed premises and weaker probability bound. The MLE algebra itself is unchanged.

QuotientReductionReview independently approved this variant's conditional algebraic/source/sampler construction with no findings. That review does not override the later zero-finding scope correction above. No repository source code, build, proof execution, benchmark, commit or push is part of this result.
