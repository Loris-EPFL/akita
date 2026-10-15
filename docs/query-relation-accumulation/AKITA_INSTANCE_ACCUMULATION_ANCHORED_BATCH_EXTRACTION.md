# Anchored extraction for Main scalar batching

Status: independently reviewed **conditional** mathematical solution. This does not instantiate the native source extractor, complete conditional-tape/Fiat–Shamir theorem or numerical security certificate. The implemented resource artifact still records the literal nested tree.

## Problem and scope

The actual Common-K Main normalizer samples $n=118$ independent coefficients and declares a nested tree with $22$ degree-two sumcheck rounds. Its literal extraction inventory contains

$$
2^{118}3^{22}.
$$

Replacing this by $119\,3^{22}$ without a new theorem is invalid. This note gives two conditional ways to avoid Cartesian coefficient interpolation. Neither changes the implemented joint coefficient distribution or performs unrestricted homomorphic folding of lattice witnesses.

Fix an authenticated adversary state and public prefix $\tau$ immediately before the coefficients. It fixes the complete original Main commitment $C$, source/view/decoder contracts, and the ordered points and values $(L_i,v_i)_{i=0}^{117}$. The coefficient vector and subsequent messages remain fresh continuation data.

## Required native source contract

A native procedure $\mathcal N$ must execute/extract the complete accepted continuation, including the retained native Main Boolean domain, relevant accumulation updates, final normalization and existing strong/terminal suffix. It returns:

1. a certified bounded original-source projection $X$, the necessary original source/path data, and the authenticated raw opening used by the normalizer;
2. an inventoried matrix collision; or
3. failure.

Its expected resource is $C_N$, failure mass is $\nu$, and collision-output mass is $c_N$. These are contracts for the actual conditional prefix family, not counts of successful honest proof leaves. $\mathcal N$ must preserve the required earlier prefix and must not derive Main boundedness from the earlier normalization/zero/PC claims whose soundness is being proved. Its resource cannot conceal the original Cartesian coefficient extraction.

A comparison procedure $\mathcal C$ with cost $C_{\mathrm{cmp}}$ takes two successful native extractions for the same original source-comparison class and complete $C$. It returns equality of their relevant $X$ projections or a correctly bounded nonzero Module-SIS collision. Opening points, successor commitments, partial evaluations and D/H payloads need not coincide. A lemma that requires equal fold payloads cannot be used here without proving that condition.

These contracts are substantive Akita obligations. They are not implied by owning a CPU handle or observing that an honest source reader returns Boolean coefficients.

## Route A: deterministic coefficient star

Use a base vector $\boldsymbol\alpha^{(0)}$ and $118$ siblings. Sibling $j$ differs from the base only in coordinate $j$, with

$$
\alpha_j^{(j)}\ne\alpha_j^{(0)}.
$$

Below each vector, obtain the complete $22$-round ternary sumcheck subtree and the complete native extraction subtree required at each leaf. Apply $\mathcal N$ at the leaves and compare their source projections to an anchor with $\mathcal C$.

If any comparison gives a collision, return it. Otherwise there is a common $X$ at every leaf. Its endpoint polynomial is then fixed. Backward sumcheck interpolation forces

$$
\sum_i\alpha_i^{(j)}\delta_i=0,
\qquad \delta_i:=v_i-L_i(X)
$$

for every star vector. Subtracting the base equation from sibling $j$ gives

$$
\bigl(\alpha_j^{(j)}-\alpha_j^{(0)}\bigr)\delta_j=0,
$$

so $\delta_j=0$ for each $j$.

Division occurs only in scalar field equations. It does not divide an integer/ring lattice witness by an arbitrary challenge difference or assert preservation of its norm.

Let

$$
L=119\,3^{22}.
$$

A conservative resource bound is

$$
C_{\mathrm{star}}
\le L C_N+(L-1)C_{\mathrm{cmp}}+C_{\mathrm{SC/functional}}.
$$

All native descendant tree factors, extraction errors and matrix targets remain inside $C_N$ and its contracts. The corresponding flat coordinate counting charge is $118/|K|$, and the sumcheck charge is $44/|K|$, for a total $162/|K|$. This is not an unpriced replacement of $2^{118}$ by $119$.

## Route B: two complete anchored continuations

Work first in a prefix-preserving public-coin/ideal-tape model. Independently invoke $\mathcal N$ twice from the same $\tau$ using lawful fresh coefficient and suffix tapes.

Let $s$ be the probability that one invocation returns a valid source. Let $b$ be the probability that it returns a source for which the fixed original roster is incorrect. Define

$$
\kappa=\frac{1+2\cdot22}{|K|}=\frac{45}{|K|}.
$$

Condition on a successful first invocation. Its source $X$ is fixed independently of the second coefficient vector. If its roster is correct, a second invocation returning the same $X$ cannot have an incorrect roster. Otherwise

$$
P(\boldsymbol\alpha)=\sum_i\alpha_i\bigl(v_i-L_i(X)\bigr)
$$

is a nonzero linear form and vanishes with probability $1/|K|$. A false subsequent degree-two, $22$-round sumcheck that nonetheless reaches this same authenticated source costs at most $44/|K|$.

Thus the probability that the second invocation is bad but has the anchor source is at most $\kappa$. Comparing two successful differing projections gives

$$
\Pr[\text{comparison collision}]
\ge s\,(b-\kappa)_+.
$$

At resource

$$
R\le2C_N+C_{\mathrm{cmp}}+C_{\mathrm{tape/bookkeeping}},
$$

this implies, for $s>0$,

$$
b\le\kappa+\frac{\operatorname{Adv}_{\mathrm{pair}}(R)}{s}.
$$

For a roster that has no valid bounded original-$C$ source, every returned valid source is bad, so $b=s$ and

$$
\operatorname{Adv}_{\mathrm{pair}}(R)\ge s(s-\kappa),
\qquad
s\le\frac{\kappa+\sqrt{\kappa^2+4\operatorname{Adv}_{\mathrm{pair}}(R)}}2.
$$

Proof acceptance probability $p$ is not automatically $s$. A separate native contract is needed, for example

$$
s\ge p-\nu-c_N.
$$

Failure and native collision-output mass must be retained. This is a concrete reduction loss, not an assertion that a Module-SIS target of 128 bits automatically gives the complete protocol 128-bit security.

### Paid-density anchor

If a justified lower bound $s\ge\rho>0$ is available, invoke $\mathcal N$ until it returns a witness, retaining any collision outputs. Then run one fresh second invocation and compare. Expected resource is bounded by

$$
C_{\mathrm{paid}}
\le \frac{C_N}{\rho}+C_N+C_{\mathrm{cmp}}+C_{\mathrm{tape/bookkeeping}}.
$$

Its comparison-collision success is at least $(b-\kappa)_+$. Removing the $1/s$ probability loss therefore requires paying inverse density in runtime. Coefficients are uniform in the **unconditioned second run**, not after conditioning on whole-proof acceptance.

If a Module-SIS advantage is defined at a hard resource cap, an expected-time symbol cannot be inserted directly. A cutoff $R_0$ needs its own failure charge; for example Markov gives $\Pr[\mathrm{work}>R_0]\le\mathbb E[\mathrm{work}]/R_0$.

## Original source and norm compatibility

The compared object must be Akita's original source blocks, inner-image digits and B/F chain, not merely the later accumulator or final normalized value. D/H remain internal extraction targets even though the relevant original-source comparisons can permit distinct opening-side D/H paths.

For weak certified products $r=\rho S$ and $r'=\rho'S'$, a conservative A-side comparison uses

$$
z=\rho'r-\rho r',
\qquad
\|z\|_\infty\le\kappa'_1\beta_\infty+\kappa_1\beta'_\infty,
$$

or the corresponding scheduled cross-profile Euclidean envelope. The implementation must establish the exact source/path map, nonzero comparison property, field/integer lifts and alphabet diameters. Do not infer shortness of $S$ by dividing $r$ by $\rho$, and do not replace a proved alphabet by the smaller alphabet observed in honest traces.

References: [Akita](SOURCE_PROVENANCE.md#akita), Definitions 10.13–10.14, Lemma 10.15, Corollary 10.17 and Theorem 10.31. These references support the required source-comparison structure; they do not by themselves instantiate $\mathcal N$ for this mixed repeated-root protocol.

## Exact conditional coefficient tapes

For $q=|K|$ and two independent raw 128-bit attempts, one primitive coordinate succeeds with

$$
a=1-\left(1-\frac q{2^{128}}\right)^2.
$$

Under the ideal-word law, the accepted vector is uniformly distributed in $K^{118}$ conditional on primitive vector success, whose probability is $a^{118}$. This conditioning is not whole-proof acceptance.

To condition a coordinate on a prescribed accepted coefficient, sample its first-success index $j\in\{1,2\}$ with weights

$$
\Pr[j\mid\mathrm{success}]
=\frac{(1-q/2^{128})^{j-1}(q/2^{128})}{a}.
$$

Fill earlier rejected words uniformly from $[q,2^{128})$, insert the prescribed canonical accepted encoding, and leave the unused suffix uniform. Preserve all raw rejected/unused bits, indexed/sequential consumption and finite compiler/sampler cutoffs.

This is a tape-level conditional law. It does not say how to find a concrete H1610/hash seed with a prescribed output. The composed Fiat–Shamir theorem must supply the actual prefix/oracle/query-index and complete-answer simulation, and price its work. [Akita, Lemma 3.22](SOURCE_PROVENANCE.md#akita) is relevant weighted-sampling machinery, not a license to post-select rewinds.

## Earlier ancestors and admission

The same idea can apply to zero/PC coefficient families only at their own prefixes. Their residuals depend on the actual zero anchor, physical support, cycle/PC contracts and source. Fixing the later Main normalizer roster does not fix counterfactual earlier points, Booleanity gamma or stage-7 challenges. Each earlier rewind needs its own source anchor, comparison, exception and runtime accounting. Retain genuinely multilinear ancestor trees until separately justified.

[Lova, §§1.2.2–1.2.3](SOURCE_PROVENANCE.md#lova) illustrates coordinate-wise star extraction and the need to control challenge-difference/norm effects. Its proof does not automatically prove Akita's source extraction. Here field division is confined to scalar equations, while lattice bounds are established through the independent native source/collision contracts.

Independent review: `ReviewInstallationBinding`, conditional native certificate/anchored-extraction review. It found the proposal correct as an open conditional refinement, but retained NO-GO gates for actual bounded native source extraction, failure/density/runtime contracts, original matrix compatibility, full conditional compiler/ROM law and earlier-ancestor composition.

Until those gates are instantiated and reviewed, the artifact keeps $2^{118}3^{22}$, `numerical_security_admitted=false`, and the native Main Boolean prefix. An optional wrapper does not repair an unsound inner relation.

Related: [native certificate scope](AKITA_INSTANCE_ACCUMULATION_NATIVE_CERTIFICATE.md), [Main domain extractor](AKITA_INSTANCE_ACCUMULATION_MAIN_DOMAIN_EXTRACTOR.md), [composed resource artifact](AKITA_INSTANCE_ACCUMULATION_COMPOSED_RESOURCE_ARTIFACT.md).
