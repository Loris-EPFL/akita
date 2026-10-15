# Source-first outer extraction: rejection coupling, soundness, and resource boundary

## 1. Result and invariant

**The rejection-probe route is valid as a parametric classical product-oracle theorem.** The probabilistic step can be proved without assuming that a computationally binding commitment has a perfectly unique source, without conditioning an original challenge on acceptance, and without assuming same-instance frontend extraction. A local **fixed-source frontend soundness certificate**, a bounded native PCS-only extractor, and the source-comparison inventory suffice.

**Independent review and emulation correction.** SourceFirstReview approved the actual-Theta probability, original-output, stopping-cost, cutoff/moment and two-fork arguments under their local/compiler/source contracts. It rejected the unrestricted-B ordinary-SIS citation; §§7.2 and 8.3 now use the reviewed [paired input-local emulator](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md). Arbitrary ro-only B is allowed for the actual-fixed-Theta argument, not this ordinary-SIS corollary. Receipt authentication uses the separately reviewed [weaker hidden-curve theorem](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md), with its complete application contracts.

For the query convention defined below, a conservative frontend contribution is

\[
 \beta_{\rm FE}(q)
 \;\le\;q\varepsilon_{\rm FE}
       +\binom q2\gamma_0.                                      \tag{1}
\]

Here `q` bounds the external ideal-stream entries exposed in **one original prover-and-verifier execution**, `epsilon_FE` is the sum of the certified local frontend soundness charges, and `gamma_0` is the maximum point probability of the first exposed root-challenge coordinate. The second term explicitly covers earlier queries to descendant keys which guessed that coordinate. It is not silently omitted. For a single ideal uniform E coordinate, `gamma_0=1/|E|`; an atomic uniform root vector can give a smaller value. This is an upper bound, not a claim of optimal Fiat–Shamir loss.

The naive expected cost is at most `(q+1) T_B`, apart from comparison/projection. With the exact selected-index/acceptance precheck, the more useful bound is

\[
 \mathbb E[Z_{\rm SF}]
 \le (q+1)T_C+2P_S+T_{\rm cmp}+T_{\rm proj}.                    \tag{2}
\]

`T_C` bounds a cheap original run/replay and verification, and `P_S` is the expected cost of the **accepted original execution's bounded PCS suffix extraction**, with zero cost when that execution rejects. In particular, the expected number of additional gated PCS-extractor invocations is at most one, not `q`. The expensive PCS tree has not disappeared; its cost occurs in `P_S`.

These are proved below. They are **not yet an instantiated receipt theorem or a numerical security certificate**. The selected outer compiler, receipt guest, source/leaf ownership and common-grid certificates, cross-profile inventory, concrete outer schedule, and strict total budget are still missing. Expected oracle-machine work is not bounded emulator CPU work. No old Q32 certificate is inherited.

The invariant is unchanged: native pre-PCS instance accumulation, original K sources and handles, E challenges, the 33 inner roles, finite T16, and a complete final native decider. Neither CH-I/CH-O nor knowledge of a concrete hash is used. The theorem programs only independent outer ideal streams. It never resets, resamples, or programs the actual inner `Theta=(ro,wo,Vhat)`.

All new mathematical claims in this note are derived arguments. Primary-source statements are separately identified. No implementation, execution test, benchmark, or performance-parity claim is made.

## 2. Fixed sources: the exact frontend contract

### 2.1 Objects and quantifiers

Fix an approved profile `p`, public statement `x`, and an ordered original-handle list

\[
 h=(h_1,\ldots,h_m),\qquad
 h_j=(\mathrm{shape}_{\rm com,j},u_{\rm pub,j}).
\]

They include the authorized program/setup identities, public receipt fields including the exact accumulator, original static commitment, source layouts and matrix views. They are bound before the first frontend challenge. Authorization is an external verifier policy, not a consequence of hashing an attacker-selected identifier.

Let `W` consist of one **algebraic source projection** per original handle, together with the weak certificates needed for comparisons. Its semantic source identity excludes the weak multiplier, evaluation point, partial evaluations, and opening-local D/H state. It includes the original source blocks and commitment-side B/F path. It need not be the canonical short state produced by `Commit`.

Apply `Dec_shape` to obtain full Boolean-indexed K grids. A fixed, total reconstruction graph produces the execution-relevant K tables `KTab_p(W,x)`. Its allowed nodes are registered K-linear maps, Boolean-row products, lookup contractions against fixed public K tables, shifts, and specified prefix-sum recurrences. The resulting relation

\[
 R_{\rm tab}(p,x;W)
\]

is the selected whole-run frontend table relation, not the tautology “the verifier accepts.” It includes its actual CPU, lookup, RAM/register, initialization/finalization, activation, public-output and boundary equations, and only those padding/range predicates that are actually checked.

For every fixed `p,x,h,W` in the admitted geometry, every adaptive prover strategy, and every fixed auxiliary function environment, the certificate below must apply. `W` is fixed **before the relevant fresh challenges**, but the strategy may know W, choose arbitrary messages and make auxiliary-oracle calls. There is no honest-source distribution premise.

### 2.2 Deterministic K reconstruction and ownership

The reconstruction supported by the inspected frontend identities includes:

* For zero-omitting columns, `R(0,t)=1-sum_{a!=0} X(a,t)` and `R(a,t)=X(a,t)` for `a!=0`. RAM uses its full physical column.
* Balanced increments are fixed digit-table contractions. `RamInc=Store*Delta` and `RdInc=(1-Store)*Delta` are Boolean-row products.
* Bytecode selectors, flags, PC values and instruction operands are fixed-table contractions of the reconstructed semantic selector columns.
* `RegVal(r,t)=sum_{u<t} RdWa(r,u) RdInc(u)` for the selected zero-initialized whole-run relation.
* `RamVal(a,t)=I(a)+sum_{u<t} RamRa(a,u) RamInc(u)`, with I reconstructed from the actual public and original advice/program sources. The final table uses the complete sum.
* Product and successor tables are their registered Boolean-row products and shifts, including the existing missing-successor convention.

For a K table w, its E-valued evaluation is

\[
 \widetilde w(r)=\sum_{b\in\{0,1\}^n}\iota(w[b])\operatorname{eq}_E(b,r).
\]

Partial bindings, equality-weighted workspaces, staged bytecode values and sumcheck messages can be E-valued. They are protocol values, not new existential E-valued execution tables. In particular, the MLE of a Boolean-row product is not generally the product of its factors' MLEs away from the Boolean cube. The actual product reduction must own that distinction.

Every leaf is identified by `(original handle, full-grid/source map, logical table, point, coordinate order, stage)`. Every consumed claim has a producer or a checked exact alias. No serialized passenger value may become a trusted leaf merely because its enum name resembles a committed polynomial.

**Padding is not inferred from PCS extraction.** Akita's algebraic batch relation authenticates the full-grid equation `f_tilde(r)=phi*v`. It does not imply `f=Pad(F)`. A frontend certificate must either:

1. use a proved no-padding/full-grid identity;
2. include padding/packing consistency in `R_tab` and provide its actual randomized reduction to full-grid leaves; or
3. define the semantic projection by used slots and prove the exact corresponding leaf relation, leaving genuinely unused cells unconstrained.

It must not divide by a possibly zero equality factor and declare the missing image condition proved. The zero-omitted physical address-zero cells similarly cannot be certified zero by semantic Booleanity: their coefficient in the semantic map is identically zero. If their physical zero/range is required by another relation, it needs a separate owner.

### 2.3 A local certificate, rather than a full frontend tree

A suitable finite certificate supplies, for every frontend challenge stage r:

* its injectively encoded pre-challenge transcript and exact challenge distribution `D_r`;
* deterministic truth predicates for its incoming and outgoing obligations relative to `KTab_p(W,x)`;
* a bad-challenge set `Bad_r(W,t)` determined by W and the **pre-challenge** transcript t, with

\[
 \Pr_{c\leftarrow D_r}[c\in\operatorname{Bad}_r(W,t)]\le e_r;    \tag{3}
\]

* the deterministic implication: if the verifier's checks pass and the outgoing obligations are true, then the incoming obligations are true, unless `c in Bad_r(W,t)`.

The implication is uniform over all later prover messages satisfying those checks. Challenge-free reductions must satisfy it with an empty bad set. Initially truth means `R_tab`; at the end it means all correctly owned full-grid leaf equations. A DAG or batch is allowed, but all consumers, aliases and carried obligations must be included. A deterministic first-false-obligation rule makes any needed choice before the challenge.

These are ordinary algebraic/compiler obligations, not a knowledge assumption. The following templates prove (3).

**Sumcheck.** For fixed W and a fixed previous challenge prefix, let `g*(Z)` be the true next partial-sum polynomial, of certified degree at most d. If the incoming scalar claim is false, a verifier-accepted message g satisfying the round sum equation cannot equal `g*`. Thus `g-g*` is a nonzero degree-at-most-d polynomial. Truth can be restored only at a root, with probability at most `d/|E|`. Apply the argument at each sequential round; no interpolation tree is needed for this soundness proof.

**Independent affine batching.** If the already-fixed claim-error vector is nonzero, a uniform independent coefficient vector cancels it with probability at most `1/|E|`. The same bound holds for normalized batching with first coefficient one; if only the fixed coefficient's error is nonzero, cancellation is impossible. The `(number of claims-1)/|E|` extraction charge may be conservative but is not needed as the ordinary affine cancellation probability.

**Power batching.** If errors are combined using powers through degree d, the nonzero residual polynomial has at most d roots. Even powers through `2(P-1)` require that degree bound; “P columns” is not a bound of P on the polynomial degree.

**Reference and selector vectors.** A nonzero polynomial residual with individual degrees `d_1,...,d_k`, evaluated at independent uniform coordinates, vanishes with probability at most `sum d_j/|E|`. To prove this, write it as a polynomial in the last coordinate. Outside the event that a chosen nonzero coefficient polynomial vanishes, the univariate root bound applies; induction proves the sum bound. This covers multilinear references and correctly specified Lagrange-coordinate spaces. It does not reclassify a centered-Lagrange high coordinate as multilinear.

**Virtual tables.** Substitute the deterministic K reconstruction into the exact reduction identity. The downstream truth predicates then refer to known polynomials. The same sumcheck/batching proof applies. A free E-valued replacement table, or an unowned output claim, would invalidate this substitution and is a missing certificate entry, not a new randomness loss.

Let `epsilon_FE=sum_r e_r`, including all selected frontend stages and any genuine exceptional events. If an accepting interactive transcript has true owned leaves but false `R_tab`, follow the obligations backwards: some implication must fail. At a challenge-free stage that is impossible; at a challenge stage it requires `Bad_r`. Fresh challenges and a union bound give

\[
 \Pr[\mathrm{accept}\land\mathrm{true\ leaves}\land\neg R_{\rm tab}(W)]
 \le\varepsilon_{\rm FE}.                                    \tag{4}
\]

This proves the fixed-source interactive contract from the local certificate. Section 5 proves the stronger adaptive oracle statement needed for source probing; it does not apply (4) after conditioning on acceptance.

### 2.4 What is and is not already supported

The [frontend manifest](AKITA_INSTANCE_ACCUMULATION_FRONTEND_MANIFEST.md), [descent audit](AKITA_INSTANCE_ACCUMULATION_FRONTEND_DESCENT.md), and [outer projection audit](AKITA_INSTANCE_ACCUMULATION_OUTER_PROJECTION.md) identify the source reconstruction, stage degrees, advice ownership and several exceptional branches. The pinned Jolt source includes the [digit-zero reduction](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/lattice/relations/digit_zero.rs), [sumcheck verifier](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-sumcheck/src/verifier.rs), and [batch prelude generator](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier-derive/src/lib.rs). These support the templates, not an already-completed certificate for a new receipt guest.

The exact remaining compiler objects are:

1. The complete selected-mode obligation/ownership graph, with the actual independent batch coefficients, reused references, head/tail alignment, stage-6a carried challenges, stage-8 selectors and all polynomial degrees.
2. The common-grid/source map for every selected PCS group, including ring/block tails, capacity slots, zero-omitted addresses, and the actual owned padding or used-slot predicates.
3. The complete root/key-prefix certificate of Section 3. Current concrete transcript types are not themselves evidence for the independent ideal-stream interface or for a K-source/E-challenge implementation.
4. The weak-source comparison profiles across **all** allowed frontend-dependent PCS occurrences and the installed static reference.
5. For `R_skel`, a deterministic byte/ISA/guest adequacy theorem: canonical logical framing, every consumed advice byte, exact original accumulator, complete ordered supplied-answer ledger, and success/panic semantics. Field RAM equations alone do not range-check arbitrary advice into u64 or prove SDK decoding consumed every logical input byte.

Unlike the old complete-tree route, this route needs local soundness and deterministic reconstruction, not deterministic interpolation of a complete frontend tree. It does not eliminate any of the above semantic ownership obligations.

## 3. Product experiment and bounded base algorithm B

### 3.1 Joint setup and actual environment

Sample the provisioned joint public setup/auxiliary data according to the declared experiment. For the ideal-setup proof all admitted outer matrix views and public dependencies are functions of a single uniform flat setup vector S. Sample

\[
 \Theta=(ro,wo,\widehat V)\leftarrow
       \operatorname{ARO}[F,m,\lambda_A,d,B_{\rm aro}],\qquad
 H_o\leftarrow\text{independent ideal outer streams}.
\]

`d>=2`; the parent model has a fixed, declared oracle circuit `B_aro` whose only oracle is ro. A publicly setup-dependent `B_aro=Build(S)` is possible only with an efficient conditional sampler and uniform size/query bounds; it is not an automatic extension. Oracle-dependent auxiliary input must be generated by an included efficient algorithm with its calls counted.

Theta consists of fixed functions. A mutable server returning its invocation counter is excluded. For the mathematical proof fix S, initial coins/auxiliary input, the whole tape of B, and the complete theta. Unrevealed outer table entries remain independent. This conditioning is mathematical, not an efficient attempt to enumerate theta. The algorithm continues to query the actual theta.

### 3.2 Root and query convention

A root header `kappa` injectively binds `p,x,h` and all pre-first-challenge messages. The first frontend challenge can be an atomic vector stream or a family of independently indexed coordinate streams with no intervening prover message. A full frontend key must expose its root header and stage/coordinate identity. Every proper descendant stage key must bind the complete previous transcript, including the root challenge.

For a particular receipt output, `I` is the ordinal of the **first fresh external H_o entry belonging to its root family**, in the original execution. With a scalar/atomic root this is its ordinary first-query index. With coordinate streams it is the earliest queried component, not a newly invented query for the vector. The actual key and component identity are retained alongside the ordinal.

Run the original verifier as part of the execution. It queries any challenge inputs the prover omitted. Hence every accepting output has an actual I, including a prover that never queried its root. Define q to bound all fresh external entries exposed by the included initialization, original prover and original verification. Conservatively,

\[
 q\le Q_A+Q_V+Q_{\rm init}.                                   \tag{5}
\]

Counting all calls instead of distinct entries is a safe upper bound. This convention replaces, rather than silently forgets, the special unqueried-output case often denoted `Q+1`. It can cost more than one verifier query. It does not count a full vector as several queries when the actual interface returns it in one answer.

For every valid descendant key already in the table before I, its encoded root value fixes a candidate value of the fresh first root component. Let `gamma_0` uniformly upper-bound the probability that this fresh component equals any such candidate. Exact uniform E sampling gives `1/|E|`. For an atomic vector it is the vector's point mass if the key fixes the whole vector. Filtered or bounded-abort samplers use their actual distribution; sampler-abort losses are recorded separately.

If the keys do not bind a fresh root component in this manner, the old-query term in (1) is not justified. An unbounded or unidentified `epsilon_old` must replace it; “commitments were absorbed” does not prove this property.

### 3.3 The base algorithm

The strictly bounded B has a cheap part C and a gated suffix part S:

1. Run the original prover and verifier; save **exactly** `o0=(x0,pi0,aux0)`, its decision, original oracle view, and any included external final-decider result in actual Theta. Save reset states before fresh outer queries, or retain enough transcript/tape to reconstruct them with charged replay work.
2. Determine the original I and root header even if later extraction fails. A rejected execution needs no source witness.
3. On acceptance, run **only the native PCS suffix extractor**, with a declared strict cutoff. Its successful leaves must keep the original complete frontend transcript/PCS statement. Counterfactual matching checks can reject changed frontend prefixes; the algorithm must not replace the original statement by a successful fork's statement.
4. Return the original tuple and I with either valid source/weak certificates satisfying every original PCS leaf, a verified named matrix collision, or failure. Retain I on suffix failure or cutoff whenever I exists.

The required native guarantee, in the ideal setup with actual theta, is

\[
 \Pr[V(o_0)=1\land B\text{ returns neither valid PCS sources nor collision}]
 \le\kappa_B.                                                \tag{6}
\]

Akita's original-index mixed-tree construction and Theorem 10.31 supply the PCS-only version for an admitted suffix schedule and exact key/sampler interface. A conservative instantiation is

\[
 \kappa_B\le(q+1)\varepsilon_{\rm PCS}
                 +\tau_{\rm suffix}+\tau_C,                  \tag{7}
\]

where the suffix cutoff tail is measured in the **original execution experiment**, and `tau_C` is any separately necessary initial-run/sampler cutoff loss. If the declared original protocol is already strictly bounded, `tau_C=0`. A cutoff during initial execution gives a failure symbol coupled to the conceptual original output; it cannot manufacture a replacement original output. No claimed unchanged marginal is made on that cutoff event.

Crucially, the native extractor is not simply restarted with a possibly adversarially chosen frontend transcript and a falsely “fresh” PCS oracle. It uses the original execution handle and full query record, including prequeries, and the native matching-index recursion. Programs of later-stage inputs preserve the complete matched prefix. Native private trials may take other frontend paths and be rejected; every successfully used suffix must certify the original frontend leaves.

**Three query ledgers differ.** B's original C makes the external calls bounded by q. After C, all H_o access by the suffix extractor goes through private lazy tables initialized from the appropriate saved original view, with the exact conditional samplers and scoped overrides. Fresh counterfactual entries are private; they do not call the external original H_o oracle. B therefore has at most q actual external H_o entries, not the suffix tree's entries. Nevertheless every privately generated bit, rejection sample, replay and lookup is charged as work, and **every Theta call in every suffix branch is forwarded to the same actual Theta**. Private outer simulation does not make inner queries free.

For a family of schedules, use a setup-independent finite accepted family with a uniform suffix/error bound and the union of its inventories. A point-dependent choice of schedule cannot be conditioned out of the setup law.

## 4. Rejection-probe coupling and expected cost

### 4.1 The algorithm

Run B once. On original rejection or original B failure, return the corresponding result. On an original suffix collision, return it. Otherwise retain `(o0,I=i,W0)` and restore the resettable B execution immediately before its original fresh query i. Retain the exact ordinary state/tape and the outer table **only as it stood at that prefix**.

Repeatedly perform independent trials:

1. Supply a fresh root answer and an independently lazy-sampled continuation table for all outer entries absent from the saved prefix. Do not retain the original run's later H_o answers in that trial. Entries present before i are retained, including arbitrary old prequeries.
2. Run C to determine its own original receipt, acceptance and selected index. If it does not accept with **the same i/key/root header**, stop the trial cheaply. Merely producing a transcript with a matching commitment is not the gate.
3. Only on that exact gate, invoke B's bounded PCS suffix. If it returns no valid source, try again. A verified collision can be returned immediately; for the coupling analysis, one may instead continue a dominating ghost process until a valid source is obtained.
4. At the first successful trial obtain W'. Compare `Src(W0)` and `Src(W')` for every original handle. A discrepancy gives the registered source collision. If they agree, reconstruct the tables from **W0**, not from the alternate receipt output. Return W0 if `R_tab(p,x0;W0)` holds; otherwise return extraction failure.

The original receipt, accumulator, auxiliary output and final-decider bit are never replaced. The original H_o view is restored for any subsequent original-world observer. Theta is unchanged and needs no restoration.

### 4.2 Conditional law and zero-success prefixes

Fix the setup, theta and all B coins as above. Let `mu_i` be the subprobability distribution of saved execution prefixes immediately before fresh external query i. Thus

\[
 \sum_{i=1}^q\|\mu_i\|
   =\mathbb E[\text{number of fresh external queries}]\le q.    \tag{8}
\]

For a reached prefix sigma define

\[
\begin{aligned}
 a_i(\sigma)&=\Pr[C\text{ accepts and selects }i\mid\sigma],\\
 p_i(\sigma)&=\Pr[B\text{ returns valid PCS sources and selects }i\mid\sigma].
\end{aligned}                                                \tag{9}
\]

Both probabilities use a fresh outer continuation and the saved tape/theta. The selected key is fixed at the saved query. Injective binding makes equality of that selected index/key preserve the original public/source prefix. Usually `p_i<=a_i`; no positive lower bound is assumed.

Conditional on `sigma`, the original continuation and the independent trials have the same unconditioned continuation law. When `p_i>0`, the first successful trial has precisely the law of a continuation conditioned on the valid-source event with selected index i. It terminates almost surely, and its source distribution is **independent of the original continuation**. For any measurable source set A, this follows by summing the geometric series

\[
 \sum_{n\ge1}(1-p_i)^{n-1}
       \Pr[\text{one trial succeeds with source in }A\mid\sigma]
 =\frac{\Pr[\text{success and source in }A\mid\sigma]}{p_i}.   \tag{10}
\]

The set `p_i=0` contributes zero probability to an original valid selection of i. It therefore never starts an infinite probe loop with positive probability. A hypothetical source at such a prefix can be any fixed dummy of the required table dimensions. No actual algorithm needs to decide whether `p_i=0`.

This is not a claim that the original challenge is uniform conditioned on success. Instead, sample a hypothetical `W_i` from (10) before revealing the original fresh answer at prefix sigma, using independent private continuation randomness. On the original event `I=i` with valid source output, the actual probe source has exactly this law. For every candidate i, not merely the eventually chosen one, `W_i` depends on the saved prefix/tape/theta and independent private randomness, never on the original future H_o. Section 5 uses this simultaneous online coupling.

### 4.3 Expected trials and the cheap gate

Conditional on the prefix and an original valid selection of i, the number of additional trials is geometric with mean `1/p_i`. Consequently

\[
 \mathbb E[N_{\rm trials}]
 =\sum_i\int_{p_i>0}p_i\frac1{p_i}\,d\mu_i
 \le q.                                                       \tag{11}
\]

This is an **unconditional original-event** expectation. The conditional cost given acceptance of a rare receipt can be arbitrarily large.

For any nonnegative trial cost Y with conditional mean `c_i`, correlation between its cost and its success is harmless. The event that trial n is reached depends only on earlier trials. Therefore

\[
 \mathbb E\Big[\sum_{n=1}^{N}Y_n\mid\sigma\Big]
 =\sum_{n\ge1}(1-p_i)^{n-1}c_i=\frac{c_i}{p_i}.               \tag{12}
\]

Multiplying by the original `p_i` gives the exact cancellation on `p_i>0` prefixes; it does not justify equality on prefixes with `p_i=0` and positive failed-trial cost.

A cheap replay costs at most `T_C`, so (11) bounds its additional expected work by `q T_C`. An expensive suffix is invoked only on the exact gate in (9). For its invocation count, `c_i=a_i`. Since the events “the original C accepts and selects i” are disjoint,

\[
 \mathbb E[N_{\rm extra\ suffix}]
 =\sum_i\int_{p_i>0}a_i\,d\mu_i
 \le\sum_i\int a_i\,d\mu_i
 =\Pr[C\text{ accepts}]\le1.                                \tag{13}
\]

For actual suffix work use
`c_i=E[1_{C accepts, I=i} Y_suffix | sigma]` in (12). The same tower-property argument bounds the extra suffix work by the expected original gated suffix work `P_S`. Add the original cheap run and original suffix to obtain (2). If one invokes the suffix on a looser prefix-matching event, the disjoint-event identity fails; only the cruder `(q+1)T_B` bound is available without another analysis.

The argument applies separately to any additive resource: bit/field work, direct ro calls, wo calls, Vhat calls, or private outer-table requests. For instance, with uniform cheap Theta counts `t_{C,u}` and expected original gated suffix counts `P_{S,u}`,

\[
 \mathbb E[t_u]\le(q+1)t_{C,u}+2P_{S,u}+t_{\rm cmp/proj,u}.
                                                                    \tag{14}
\]

Deterministic source comparison and table reconstruction are oracle-free; guest projection is so only if its specified certificate is oracle-free. Prefix storage/reconstruction and full private-stream work belong in these costs.

## 5. Why the hypothetical source gives additive original-event soundness

This section proves (1), rather than replacing the difficulty by an assumed same-instance extraction theorem.

### 5.1 Online hypothetical experiment

Run the original external H_o execution exactly as before. Whenever it first queries a new **root family**, at ordinal i and saved prefix sigma, sample `W_i` using the distribution in (10), or the dummy when `p_i=0`, before answering that fresh root entry. Do this for every encountered root family. These hypothetical samples do not affect B's execution. They can be inefficient; this is a probability coupling, not an implementation that runs q actual probes.

Root families with `p_i=0` are ignored in the local bad-event ledger: their probability of an original valid selected output is zero. Thus no algebraic validity or local certificate is required of the dummy. For positive p, the sampled source is an actual valid extracted source for the fixed root header.

Fresh original table entries remain independent of this augmented past. A hypothetical sample uses theta, the actual past table and independent trial tables, not unexposed original entries. Even fixing a complete theta does not reveal an independent future outer answer. The marginal law of the original run is unchanged.

If the original B validly selects i, the actual algorithm's single implemented probe can be coupled to `W_i`. Equality of source projections then means all original leaf equations are evaluations of the deterministic tables of this fixed `W_i`. This is exactly the event to which the local certificate applies.

### 5.2 Old descendant keys

At a root-family first query i, at most `i-1` previous external keys can be candidate descendants of its header. Each such key specifies the relevant root component value. Conditional on the saved prefix and hypothetical `W_i`, that component is fresh, so a particular candidate matches with probability at most `gamma_0`.

A union bound over previous keys and all possible root first-query ordinals gives

\[
 \Pr[G_{\rm old}]
 \le\sum_{i=1}^q(i-1)\gamma_0
 =\binom q2\gamma_0.                                         \tag{15}
\]

This is a bound in the unconditioned augmented experiment. It is not a bound asserted after conditioning on the final selected i, acceptance, or successful source extraction. Multiple candidates with the same root guess only improve the bound. Invalid keys are irrelevant. Queries to independent siblings within the same root vector are not descendants: using the **earliest** component query ensures that no such sibling was already exposed for this root family.

### 5.3 Fresh local errors

Consider the first external query to a particular frontend challenge-stage family. If its root family has already been exposed, its assigned `W_i` is part of the augmented past. The key fixes its complete pre-challenge transcript. Thus the local residual and `Bad_r(W_i,t)` are determined before that family's fresh challenge answer. By (3), its bad event has probability at most `e_r`.

For an atomic vector, this uses the vector distribution directly. For separately indexed independent coordinates, charge the family at its **first** queried component: no other component of that same family has been exposed yet, so its full not-yet-revealed vector still has the required product law. All components share one pre-vector transcript with no intervening prover message. Conceptually fill its unused independent coordinates immediately; later queries only reveal them. This changes no distribution or actual query count. If the actual interface instead interleaves messages between coordinates, those coordinates must be separate certified stages.

This argument also covers a key with guessed intermediate earlier challenges. Its entire parsed transcript determines the local polynomial; consistency with the eventual proof is not needed for the root bound. The exceptional case is a descendant key queried **before its root source was assigned**. If such a key is used in the original selected proof, it lies in `G_old` by (15).

There are at most q first-family hits. Summing conditional local bounds gives at most `q max_r e_r`, and hence the safe stated bound `q epsilon_FE`. A more detailed query/stage ledger may use the sum of the actual conditional charges. There is no extra factor q for separately enumerating potential original indices: all hypothetical sources were installed in a single online experiment, and each fresh stage family is charged only once.

Now suppose the original output accepts, its source W0 is validly extracted, `Src(W0)=Src(W_I)`, and `R_tab(W0)` is false. Its leaves are true for `W_I`. The backwards local-certificate argument of Section 2 finds a bad challenge in this proof. Either its key was prequeried before the selected root, giving `G_old`, or it is one of the charged fresh local errors. Therefore

\[
 \Pr[\mathrm{original\ valid\ source},\ \mathrm{accept},\
       \operatorname{Src}(W_0)=\operatorname{Src}(W_I),\
       \neg R_{\rm tab}(W_0)]
 \le q\varepsilon_{\rm FE}+\binom q2\gamma_0.                 \tag{16}
\]

This proves the probabilistic step. It never conditions a verifier challenge on source equality or on successful extraction. It never declares commitment sources unique without producing the corresponding collision when they differ.

## 6. Same-original-output theorem

**Theorem.** For every bounded classical receipt prover in the product experiment, assume the local frontend certificate, exact root/key interface, bounded native B satisfying (6), and valid source-comparison algorithms at the full occurrence inventory. The rejection-probe algorithm has the original-output coupling described in Section 4 and expected cost (2). In the ideal setup with actual Theta,

\[
 \Pr[V(o_0)=1\land E_{\rm SF}\text{ returns neither an }R_{\rm tab}(x_0)
       \text{ witness nor a valid collision}]
 \le\kappa_B+\beta_{\rm FE}(q).                              \tag{17}
\]

**Proof.** Charge original B failure by (6). On its valid-source event, an independent successful source probe exists almost surely by (10). Any different source projection gives a collision. In the remaining event, failure to return a table witness is exactly an event bounded by (16). The algorithm returns the original W0 for the original x0 and retains the original output/decider data. Equations (11)–(14) prove its cost. Average the proof over theta, tapes and setup. No computational assumption was used in these probability and oracle-machine statements. ∎

With a proved deterministic guest/byte certificate, substitute `R_skel` by decoding the reconstructed W0. That deterministic substitution adds its actual work but no probability term. Without that certificate the theorem stops at `R_tab`, explicitly; it does not output an archive merely because a PCS witness exists.

## 7. Source collisions and the actual joint setup

### 7.1 Concrete comparison, not perfect uniqueness

Retain each weak factor `rho`, the bounded product `r=iota(rho)s`, and certified commitment-side digits. Compare all occurrences against the representative of the **same original handle and shape**.

If the B/F source paths differ, descend the compression path and output a nonzero kernel vector with the actual cross-profile input-alphabet diameter. If their inner-image digits agree but source blocks differ, output

\[
 z_A=\iota(\rho')r-\iota(\rho)r'
     =\iota(\rho')\iota(\rho)(s-s').                         \tag{18}
\]

The unit factors make it nonzero; equality of the inner images gives `Az_A=0`. For the coefficient norm,

\[
 \|z_A\|_\infty\le k' b+k b',\qquad
 \|\iota(\rho)\|_1\le k,\ \|r\|_\infty\le b.               \tag{19}
\]

Use only actually certified Euclidean alternatives if that profile chooses the Euclidean norm. For a canonical installed static reference, set `rho'=1` and `r'=s_static`, obtaining `b+k||s_static||_infty`. No bound on `rho^{-1}` is claimed.

Akita Definitions 10.13–10.14, Lemma 10.15 and Corollary 10.17 prove precisely these alternatives. The opening-local D/H payload and evaluation point may differ across probes; they are not compared as if fixed. D/H collisions internal to a native suffix remain in its native inventory. The source-only cross-probe comparison uses A/B/F.

The inventory must cover every pair of opening profiles reachable when frontend points change, diagonal pairs, mixed pairs and the installed reference occurrence. The source's Eqs. (195)–(196) take these mixed maxima. A native schedule's diagonal norm, the honest short-digit alphabet, or the inner 33-role inventory alone is not this outer inventory.

### 7.2 Bounded ordinary-Module-SIS reduction

Let `E_SF,L` be the entire algorithm stopped at total charged actual-oracle work L. Let

\[
 \delta_L=\Pr[Z_{\rm SF}>L],
\]

in the ideal-setup actual-Theta original-run experiment. Count all original/probe/suffix calls and initialization/predicates. The original suffix cutoff already contributes to `kappa_B`; this is an additional total-loop cutoff, not a second name for the same event.

First prove (17) with actual Theta. For the ordinary-SIS step **add the input-local B_aro profile requirement**, then run the complete bounded collision producer through the [paired emulator](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md), with one persistent augmented paired-head and extended-query state throughout all internal resets. The unrestricted CCGOS Construction 5.11 citation is insufficient. The repaired complete-run theorem changes the aggregate collision event by at most

\[
 \varepsilon_{\rm emu}\le t_{\widehat V}/2^{\lambda_A},       \tag{20}
\]

or the safe coarse `t_Theta/2^lambda_A`. Its ro-only version queries ro once per newly activated head, samples and caches that head's mu, and evaluates B_aro using only its cached ro(x). Pair both RO- and WO-triggered activations internally. All calls use one table, and neither actual Theta nor the emulator state is rewound with the prover. Complete kernel/feasibility work is priced separately.

Only after this whole-computation replacement embed an ordinary Module-SIS challenge into a chosen matrix view of one uniform flat S. Fill every other flat coordinate uniformly and construct **all** remaining overlapping views, registry values, public references and allowed auxiliary data from that same S. Run the full emulated computation and output a collision only when it names the embedded view. This is Akita Lemma 10.11. For adaptive schedule selection use the fixed public family's union inventory, as in Corollary 10.12; do not condition S on the chosen schedule.

Let `Delta_setup` bound the distance of the **actual joint provisioned experiment** from this ideal one, including the public setup interface and efficiently generated dependencies. Mere uniform matrix marginals do not justify the joint replacement. A revealed seed required to expand to an arbitrary embedded S needs a valid setup-namespace simulation, not an unavailable seed preimage. Trapdoor-correlated advice cannot be generated merely by filling unused coordinates.

For the resulting ordinary bounded-time algorithms `B_I`,

\[
\begin{split}
 \Pr[V(o_0)=1\land E_{\rm SF,L}\text{ fails to return }R_{\rm tab}(x_0)]
 \le{}&\kappa_B+\beta_{\rm FE}(q)+\delta_L+\Delta_{\rm setup}\\
      &+\varepsilon_{\rm emu}
       +\sum_{I\in\mathcal I_{\rm outer}}
                    \operatorname{Adv}_{\rm MSIS,I}(B_I).    \tag{21}
\end{split}
\]

The aggregate collision event is emulated **once**, before partitioning by matrix identifier, so (20) is not multiplied by the inventory size. All collision advantages use the complete reduction runtime and admitted mixed norms. Equation (21) additionally requires the scoped input-local paired-emulation contract, joint setup law, strict cutoff and stated certificates; it is not a new binding/knowledge axiom or an unrestricted-B corollary.

## 8. Cutoffs, moments, and emulator runtime

### 8.1 Explicit retry tail

If at most K additional probes are permitted, the exact probability mass of an originally valid source event that exhausts that retry budget in the ghost process is

\[
 \delta_{\rm retry}(K)
 =\sum_i\int p_i(1-p_i)^K\,d\mu_i
 \le q\frac{K^K}{(K+1)^{K+1}}
 \le\frac q{K+1}.                                           \tag{22}
\]

For `K>=1`, maximize `p(1-p)^K` at `p=1/(K+1)` to obtain the middle expression. A verified collision can only terminate earlier. For `K=0` use the original valid-source probability directly, at most one. A fixed retry cap gives a strict cost bound at most

\[
 (K+1)(T_C+L_S)+T_{\rm cmp}+T_{\rm proj},                    \tag{23}
\]

when each gated suffix has strict cost at most `L_S`. Skipped suffixes reduce the actual cost, but not this worst-case bound. A retry tail is not a high-probability statement conditioned on a rare accepted original receipt.

For a total work cutoff, (2) gives the independent unconditional Markov bound

\[
 \delta_L\le P_{\rm SF}/L,
 \quad P_{\rm SF}=(q+1)T_C+2P_S+T_{\rm cmp}+T_{\rm proj}.    \tag{24}
\]

Use one declared cutoff construction or combine distinct cutoff events by a union bound; do not charge the same event twice. A measured or proved stronger tail may improve these bounds. The native suffix's own tail is separate and must be supplied by its actual extractor, not by counting successful tree leaves.

### 8.2 Why higher moments do not cancel

For a geometric variable `N` counting the successful trial,

\[
 \mathbb E[N^2\mid p]=(2-p)/p^2,
 \qquad p\,\mathbb E[N^2\mid p]=(2-p)/p.                     \tag{25}
\]

The first moment's cancellation has left an inverse p in the second moment. There is no general polynomial bound in the first moment.

A finite strictly bounded family already demonstrates the problem. Choose a saved-prefix parameter `n in {1,...,N}` with probabilities proportional to `2^{-n}`; let a bounded trial succeed with probability `p_n=2^{-2n}` by testing at most `2N` fresh bits. There is one selected index. The original-success-weighted expected number of additional trials is at most one, but its weighted second moment is proportional to

\[
 \sum_{n=1}^N2^{-n}/p_n=\sum_{n=1}^N2^n=\Omega(2^N).        \tag{26}
\]

Every individual B is strictly bounded with only O(N) inspected bits. Thus even finite support can give exponentially large higher moments. With an unbounded prefix family the second moment can diverge, but divergence is not needed for the counterexample.

The same problem can affect the count of gated suffix invocations when the conditional suffix success probability is very small. The expectation bound (13) does not by itself give a small second moment.

### 8.3 CPU and native-oracle budgets after a cutoff

Let t bound all exposed Theta calls after the strict cutoff, h<=t their distinct activated RO/WO heads, and v<=t their extended queries. Use the repaired emulator's complete cost `C_M(h,v)` from [its explicit ledger](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md#7-full-query-storage-and-runtime-ledger), including local B evaluation, sparse-polynomial work, consistency detection, conditional sampling and storage operations. For an input-local profile,

\[
\begin{aligned}
 Q_{ro,\rm sim}&\le h,\\
 T_{B_I}&\le L+C_M(h,v)+T_{\rm embed,I}+T_{\rm coll,I}.         \tag{27}
\end{aligned}
\]

Retained-oracle pairing may make h RO and h WO calls, but its **ro-only implementation** makes one underlying RO request per head, even for ro-free B. Repeated local oracle gates inside B are CPU work against the cached value. Final predicates and initialization add their actual fresh heads and extended observations; none is free.

One cannot replace the strict t in (27) by `E[t]`, or assert `E[poly(Z)]<=poly(E[Z])`. Equations (25)–(26) refute the latter inference. The safe order is: actual-oracle coupling, strict cutoff, whole-computation persistent emulation, ordinary lattice reduction. All private outer-stream bit generation and conditional-sampler work must already be in L.

For final receipt composition, the producer must return the original accumulator's archive/ledger. Authentication uses the separately reviewed [hidden-curve theorem](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md); the emulator alone is insufficient. All original runs, failed probes, gated PCS subtrees, decoding and final predicates enter their budgets. A forwarding producer keeps one persistent emulator across all internal rewinds. The distinct [private-continuation transformer](AKITA_INSTANCE_ACCUMULATION_PRIVATE_ARO_CONTINUATION.md) separates actual calls from virtual work under its two-hybrid and deterministic-predicate restrictions.

## 9. A bounded two-fork alternative and its weaker loss

The rejection procedure is useful because it avoids a success-square loss, not because two successful continuations are automatic. For comparison, a one-probe algorithm gives a rigorous square-root bound.

Let alpha be the probability that the original B accepts with valid PCS source W0 but false `R_tab(W0)`. At saved prefix i let `b_i` be this same bad-success conditional probability. Then

\[
 \alpha=\sum_i\int b_i\,d\mu_i,
 \qquad\sum_i\int b_i^2\,d\mu_i\ge\alpha^2/q,                \tag{28}
\]

by Cauchy–Schwarz and (8). The second expression is the probability that the original bad output and one independent continuation are both bad valid outputs with the same selected i. Distinct sources yield a collision.

For a fixed prefix and a source drawn from the alternate bad-success subdistribution, the original future is independent. A fixed-source FS union bound for that one root header gives the conservative uniform

\[
 \delta=q\varepsilon_{\rm FE}+(q-1)\gamma_0.                  \tag{29}
\]

There are at most q fresh stage families and at most `i-1` old root guesses. Hence equal-source bad pairs have probability at most
`sum_i integral b_i*delta dmu_i = alpha*delta`.
If c is the collision probability of this actual bounded one-probe algorithm,

\[
 \alpha^2/q\le c+\alpha\delta,
 \qquad
 \alpha\le\frac{q\delta+\sqrt{q^2\delta^2+4qc}}2
          \le q\delta+\sqrt{qc}.                            \tag{30}
\]

This argument uses no rejection loop and costs at most two bounded B executions plus comparison. It is weaker statistically: the collision contribution is square-rooted and the simple per-prefix frontend bound is multiplied again by q. It is a genuine alternative if a concrete cutoff for rejection sampling is unacceptable, not a justification for claiming the additive theorem at two-run cost.

## 10. Remaining numerical boundary

This result removes the need to **enumerate a complete frontend interpolation tree** in the source-first proof. It does not prove that the remaining native PCS suffix is inexpensive. Akita Eq. (225) still prices every retained native range, row-batching, tensor and sequential sumcheck stage; Eq. (227) gives an unspecified polynomial expected-work bound, not an exact number of prover calls. The [PCS extraction cost analysis](AKITA_INSTANCE_ACCUMULATION_PCS_EXTRACTION_COST.md) separately prices that suffix and the same cheap-gate cancellation. The old [full-tree forwarding example](AKITA_INSTANCE_ACCUMULATION_EXTRACTION_BUDGET.md) remains a correct obstruction for the enumerating frontend-tree route; it is not a lower bound on this new algorithm or on honest proving.

A concrete admission would have to provide all of:

* the selected frontend local ledger and root distribution, so that (1) is numerical;
* the actual bounded PCS suffix algorithm, error, work, Theta-query counts and cutoff tail, so that `kappa_B`, `P_S` and `L_S` are numerical;
* the cross-probe/source-reference matrix inventory and actual joint setup distance;
* a strict total cutoff/retry policy, its failure budget, persistent emulator parameters and CPU bound (27);
* the actual derived native/AS query budgets, guest/byte adequacy, and independently admitted query-accumulator commitment/security parameters.

The unchanged native statistical bound has the form

\[
 (Q_i^{\rm derived}+1)
       \left(c_E/|E|+c_{\mathcal C}/|\mathcal C|\right),\qquad
 |\mathcal C|=\binom{128}{40}\binom{40}{24}2^{40},
 \quad\log_2|\mathcal C|\simeq186.844275296.                  \tag{31}
\]

`Q_i^derived` is the actual query ceiling after the chosen product-oracle/ARO conversion, including any separately required transcript-oracle compilation. It is not q, the original archive's honest call count, or the expected number of suffix invocations. If the shell term is allocated error `epsilon_nat`, it requires

\[
 Q_i^{\rm derived}+1
 \le\epsilon_{\rm nat}|\mathcal C|/c_{\mathcal C},             \tag{32}
\]

in addition to the E term and all other losses. In particular, at `epsilon_nat=2^{-128}` its shell-only ceiling is approximately `2^{58.844275296}/c_C`; this is a necessary condition for that displayed upper bound, not an admitted replacement security policy. Larger E does not enlarge the shell. No `<2^-137` Q32 figure transfers without establishing its actual query hypothesis for this derived producer.

For the outer statistical terms alone, a target beta requires at least

\[
 q\varepsilon_{\rm FE}+\tfrac12q(q-1)\gamma_0
 +(q+1)\varepsilon_{\rm PCS}+\tau_C+\tau_{\rm suffix}
 +\delta_L\le\beta,                                         \tag{33}
\]

before setup/emulation/lattice/query-accumulator/native terms are added. With an exact E root, the old-guessing term scales as `q^2/(2|E|)`, not zero. Equation (33) is deliberately symbolic: no selected outer guest/schedule currently supplies all of its quantities.

**Conclusion.** The source-first probabilistic route is proved, with an additive original-output failure bound and an improved expected-cost identity. The unresolved issue is now a concrete compiler/common-grid/comparison certificate and a bounded PCS/emulation/native resource admission, not an unproved assertion that postselected extracted sources were fixed before the original challenges. Neither an implementation admission nor numerical receipt security follows yet.

## Primary references and evidence scope

1. **Akita**, [local primary text](SOURCE_PROVENANCE.md#akita), [registered paper](https://eprint.iacr.org/2026/1983). Live qmd passages used: lines 1455–1547 (Definition 3.15; Lemma 3.22; original index on failure; matching-index cheap gate), 3639–3697 (mixed comparison classes; shared flat setup; adaptive schedule reduction), 3699–3825 (algebraic/weak sources; Eqs. (203)–(206); Corollary 10.17), and 4110–4249 (Eqs. (221)–(229); Theorem 10.31; Remark 10.32; native FS ceiling). The indexed text's version is the authority for these equation numbers; the external link is provenance, not a claim of independently checking its live revision.
2. **CCGOS, Arithmetized Random Oracles**, [accepted manuscript](SOURCE_PROVENANCE.md#supplemental-primary-sources), [text](SOURCE_PROVENANCE.md#supplemental-primary-sources), [primary repository](http://wrap.warwick.ac.uk/176570). Definition 4.1 supplies the fixed oracle law; Lemma 6.1 supplies perfect joint RO/WO simulation. The original Theorem 5.4/Construction 5.11 guarantee is **not valid for the unrestricted B used in its statement under the inspected transcript convention**. The [scope audit and independently reviewed input-local paired replacement](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md) supply the narrower emulation/game-preservation theorem used here. The complete receipt zero-finding transport remains separate.
3. **Pinned modern Jolt source**, revision `0791f79a55c3fadc3d327c78a72228696c338468`, through the primary links in Section 2 and the [frontend manifest](AKITA_INSTANCE_ACCUMULATION_FRONTEND_MANIFEST.md), [descent](AKITA_INSTANCE_ACCUMULATION_FRONTEND_DESCENT.md), and [projection](AKITA_INSTANCE_ACCUMULATION_OUTER_PROJECTION.md) source audits. These are evidence for the enumerated algebraic templates and known gaps; they are not a complete certificate for a nonexistent receipt guest.
4. The mathematical product-oracle and bounded-emulation setup is aligned with [PRODUCT_EXTRACTION](AKITA_INSTANCE_ACCUMULATION_PRODUCT_EXTRACTION.md). This note supplies a different frontend route and its own coupling proof; it does not reuse that note's full frontend tree as an unpriced subroutine.
