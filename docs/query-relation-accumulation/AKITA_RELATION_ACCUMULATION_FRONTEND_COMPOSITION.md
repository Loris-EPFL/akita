# Candidate 1: frontend composition without multiplying segment trees

## Status and decision

This is a new derived composition argument for Candidate 1, not an independently authenticated Akita theorem, an implementation, or numerical security admission. It does not change the query accumulator, introduce a commitment, fold a witness, or alter an original A/B/F path. It concerns how an extractor for the existing finite sealed query-plus-PCS continuation can be composed with many preceding segment frontends.

**Supersession:** [Published marked-original anchoring](AKITA_RELATION_ACCUMULATION_FRONTEND_COMPOSITION_PUBLISHED_ANCHOR.md) independently establishes the retained-original subtree contract discussed in §§4–5 below. Under that proved sampler, the extra independent $E_0$ extraction in the conservative route is unnecessary: the history coefficients become $\sum_j B_{\mathrm{FE},j}$ and $T$, not $1+\sum_j B_{\mathrm{FE},j}$ and $T+1$. The source-probe alternative remains separate. [Lighter ordinary extraction](AKITA_RELATION_ACCUMULATION_LIGHT_ORDINARY_EXTRACTION.md) subsequently removes the query $m3^n$ interpolation factor under its explicitly stated source, local-soundness and sampler contracts. Neither improvement grants concrete profile admission.

**Decision:** a sum of per-segment extraction costs is sufficient; a product of all segment frontend trees is unnecessary. However, the compiler must guarantee extraction for the **same original execution and target prefix**, not merely a lower bound on the extractor's marginal success probability. An independent extraction of the original sealed query relation gives a clean common source assignment. With that extra extraction, the conservative tree/error coefficients are `1 + sum_j B_FE,j` and `T+1`, rather than `sum_j B_FE,j` and `T`, unless a frontend extractor explicitly provides the original full-frontend leaf's complete query subtree.

There is a cheaper composition method: one original sealed-query extraction, followed by one source-only matched successful continuation probe for each segment. A hypothetical-source coupling and the frontend's ordinary fixed-source local soundness certificate replace all frontend interpolation trees. This method has a complete probability and expected-cost derivation below. Its rejection-loop cutoff, frontend certificate, actual query grammar, original-source comparison profiles, and ordinary suffix extractor remain explicit admission conditions.

## 1. Evidence and provenance

Read for this derivation:

- `AKITA_RELATION_ACCUMULATION_PROTOCOL.md`, complete candidate and its frontend-composition clause.
- `Papers/parsed/Akita.md:1452–1547`: the local same-execution knowledge experiment, mixed-tree definitions and the expanded Lemma 3.22 recurrence.
- `Papers/parsed/Akita.md:3709–3824`: algebraic decommitments, weak-product certificates and source-only comparison.
- `Papers/parsed/Akita.md:4110–4183`: whole-tree, query-budget and MSIS-runtime accounting.
- `AKITA_INSTANCE_ACCUMULATION_SOURCE_FIRST_EXTRACTION.md:34–139,139–349,350–453`: the earlier source-probe construction. The proof below rederives its needed probability step in the simpler ordinary classical-ROM setting; it does not import that note's separate ARO/emulation assumptions.
- `AKITA_INSTANCE_ACCUMULATION_FRONTEND_DESCENT.md:17–32,163–221,265–325`: fixed K-table reconstruction, advice ownership, aliases and the physical address-zero obstruction.
- `AKITA_INSTANCE_ACCUMULATION_SEGMENT_INTERFACE.md:160–176`: distinct memory-mapped advice, arithmetic advice and SDK tape semantics.
- Actual current `jolt/crates/jolt-verifier/src/verifier.rs:247–280,420–452,804–912`: `validate_and_seed_transcript` absorbs the native Main, untrusted/trusted advice, optional field-increment and direct-program commitments before `stage1::verify`.
- Actual current `akita/crates/akita-types/src/opening_claims.rs:111–153`, `akita/crates/akita-cpu-backend/src/opaque/ordinary_projection.rs:31–96,151–213`, and `sources/packed_ram.rs:26–46`: ordinary endpoint dimensions/components and the logical, not arbitrary raw-plane, RAM opening capability.

The expanded local Akita body is **not official-body-authenticated**. Its agreement with a local PDF is not independent original-author provenance. The official full-body retrieval failure recorded by the main protocol remains unresolved. Every use of the local compiler/source theorem below is therefore an explicitly named premise, which can instead be independently proved. No build, test, lint, formatter, benchmark or source edit was performed for this derivation.

## 2. Objects and exact contracts

Fix an approved finite protocol/profile family, a security parameter, an external expected run statement, exact segment count `T`, and a bounded classical adversary. Include its original verifier and any oracle-dependent initialization in the original execution. Let `q` bound all fresh entries exposed by that included execution; conservatively `q <= Q_A + Q_V + Q_init`. All nonce probes are counted. A vector returned by one actual oracle query is one entry, not several invented queries.

Use one joint transparent setup and one independent classical indexed ideal-RO interface. Prover initial auxiliary data must be oracle-independent, or generated by an included efficient initialization whose oracle accesses are counted. Prover adaptation to previously queried answers is allowed. Free advice correlated with unexposed future answers is not.

The original record is

`o0 = (expected statement, published receipt, original decision, original tape/state, original oracle transcript)`.

Every extraction and probe below preserves `o0` literally. Private counterfactual tables are separate from the original table; after a probe, subsequent work starts again from the retained original record. Private sampling, copying, replay and comparisons are charged computation.

### 2.1 Primitive sources, not free virtual tables

For each segment `j`, let `h_j` be its exact target source references and source views, and let `u_j` be its bound transition statement. They include every primitive original source needed for the segment's deterministic K-table reconstruction and its boundary relation. For the one-root probe construction, these primitive sources and `u_j` are fixed in the injectively encoded header **before the segment's first relevant frontend challenge**.

Later segments' commitments may change under a fork. Later deterministic partial-evaluation workspaces may also change. They are not coalesced. A purported new primitive source inside segment `j` cannot be used to explain an earlier randomized constraint if its commitment was not fixed before that constraint's challenge. Such a source needs an earlier binding point or a separately proved causal decomposition; the construction does not make it fixed by assertion.

The inspected ordinary Jolt verifier binds its native primitive objects before stage 1. This is useful implementation evidence, but it does not authenticate the new continued-segment endpoint objects or their complete semantic certificate. Their analogous prechallenge fixation must be checked in the selected continued mode.

### 2.2 Algebraic source assignment and comparison

Let `S` contain one original algebraic source projection per retained commitment, together with its weak comparison certificates. A projection includes the original source blocks, inner-image digits and B/F path. It excludes weak multipliers, partial evaluations, points and D/H payloads. It need not be an honestly produced canonical short source.

For two certified projections of the same original `(shape,payload)` and admitted source-comparison class, require a deterministic algorithm returning either equality or a named original A/B/F collision. For A, use the two weak certificates `rho*s=r` and `rho'*s'=r'` to form

`z_A = iota(rho')*r - iota(rho)*r' = iota(rho')*iota(rho)*(s-s')`.

Unit factors establish nonzeroness if the sources differ. The coefficient bound is `k'*b + k*b'`, not a bound on an inverse. B/F comparisons use the cross-profile certified alphabet diameters and complete path widths. The inventory must cover all profiles reachable in the original execution, every allowed counterfactual continuation, and all cross-segment/base comparisons. D/H may differ between these probes; ordinary opening-internal D/H comparisons still belong to the ordinary PCS extractor.

### 2.3 The common sealed-query extractor B

Let `C` be the bounded original prover-plus-verifier computation, costing at most `T_C`. On original acceptance, invoke a bounded extractor `Suf` only for the **original sealed query-plus-complete-PCS continuation**. This is the weighted query reduction and full ordinary PCS, not only the final PCS endpoint extraction.

The combined algorithm `B = C followed by gated Suf` returns:

1. the same original record and every selected segment-root index, even on suffix failure;
2. a source assignment satisfying every original deferred scalar equation and its original algebraic decommitments, with comparison certificates; or
3. a verified named collision; or
4. failure.

Its required original-event guarantee is

`Pr[ V(o0)=1 and B returns neither valid original query sources nor a collision ] <= kappa_B`.  (B1)

A schematic ideal-ROM instantiation is

`kappa_B <= (Q+1)*(epsilon_query + epsilon_PCS) + suffix/initial sampler and cutoff losses`,

with `epsilon_query <= (m-1+2*n)/|E|`. This instantiation requires the actual ordinary/query compiler and exact sampler law; it is not supplied by a benchmark or by the marginal recurrence alone.

Successful source output of `B` certifies the **original** sealed claim roster. A full PCS extraction alone certifies only original terminal endpoint claims and is not this primitive.

Let `P_S` denote the expected gated suffix work in one original `B` execution. A strictly bounded suffix can also have bound `L_S`; its original-execution cutoff contribution is included in `kappa_B`. Later probes use this same bounded algorithm, not a falsely fresh suffix oracle.

### 2.4 Root/key grammar

A segment root-family header binds the protocol/profile, segment identity, `u_j`, `h_j`, and the complete pre-root transcript. Descendant keys bind that root family and all previous challenge answers and prover messages.

For the published original segment `j`, `I_j` is the ordinal of the **first fresh external entry** in its root family, with its actual key/component identity retained. The verifier's calls are included, so an accepting output has such an index even if the prover omitted the root query. A separately indexed vector uses its earliest queried component and has no intervening prover message inside the vector.

Every old descendant key of the header fixes a candidate value of that fresh root component. Let `gamma_0` bound the point probability of that component; for exact uniform field sampling, `gamma_0 = 1/|E_root|`, using the maximum over approved root profiles. This is a statement about parsed injective indexed keys, not a claim that a concrete hash-chain digest exposes its preimage.

## 3. Selective per-segment accepting trees: algebraic composition

For segment `j`, define a selective tree as follows:

- branch with the actual mixed interpolation/CWSS arities at all frontend stages needed by the segment's reduction;
- give all other frontend stages arity one, retaining a checked continuation for each prefix;
- attach a complete sealed-query-plus-PCS extraction tree at every resulting frontend leaf;
- preserve the original segment target header and every original primitive source occurrence on which that segment's extraction depends.

The selected stage set must contain reused references and other ancestors required to separate the target residual polynomial. A stage called “other” cannot be assigned arity one merely because its implementation lies in another module or segment. The selection must be dependency-closed for the local deterministic extraction argument.

Let `B_FE,j` be the actual selected frontend tree factor and `B0 <= (m+1)*3^n*B_PCS` the common query-plus-PCS overbound. Every complete selective tree then has at most `B_FE,j*B0` terminal PCS subtrees, with full continuations below each branch.

### Algebraic lemma

From any such complete accepting tree, extract every terminal PCS source, coalesce original sources below each sealed-query tree, reverse its degree-two sumcheck, and interpolate its query-index residual. Thus every scalar leaf obligation emitted by that counterfactual frontend is true for its extracted source assignment.

Now compare only the segment target sources that were fixed before the relevant frontend forks. Either one comparison returns an A/B/F collision, or the same primitive segment sources explain every required frontend leaf. Reverse the selected frontend interpolation tree using the exact table reconstruction and degrees. This establishes the segment relation for the **original anchored transition statement** and these original target source projections. Future segment sources and opening-local paths can vary and are irrelevant to this local extraction.

This is a deterministic supplied-tree argument. It does not by itself sample an accepting tree or establish an original-execution probability guarantee.

## 4. Quantified sum-of-trees composition and the base-leaf correction

Assume, explicitly, that for each `j` the actual compiler provides a resettable extractor `E_j` with

`Pr[ V(o0)=1 and E_j fails to return an anchored valid segment witness or collision ] <= kappa_j`,

where

`kappa_j <= (Q+1)*(epsilon_FE,j + epsilon_0) + cutoff/sampler losses`,

and `epsilon_0 = epsilon_query + epsilon_PCS`. Its success output must refer to the original target header and carry certified original source projections. Initializing every `E_j` from the retained original execution is part of the contract; rerunning a new independently sampled prover is not.

Run `B` once to obtain the original sealed-query assignment `S0`. Run each `E_j`, comparing its target projections against the corresponding components of `S0`. A discrepancy returns the registered source collision. Otherwise all segment witnesses use `S0`, so checked canonical RAM/source aliases and public boundary equations permit deterministic assembly of the declared ordered run witness. No product of segment frontend trees is needed.

A union bound, without any independence assumption between these extractor failures, gives

`Pr[original acceptance and no run witness/collision] <= kappa_B + sum_j kappa_j`.

Ignoring separately stated cutoffs/samplers, the conservative statistical price is therefore

`(Q+1) * [ sum_j epsilon_FE,j + (T+1)*epsilon_0 ]`.  (T1)

The structural work factor is

`[1 + sum_j B_FE,j] * B0`,  (T2)

plus compiler sampling/replay, comparison and assembly work. The same `Q+1` governs each original execution experiment; one must not multiply a new `Q+1` down each recursive tree level. Summing the `T` extractor calls is nevertheless real work and real union-bound loss.

### Why the proposed coefficient T is not automatic

The local Lemma 3.22 explicitly retains the original selected query input and prechallenge prefix. It does **not explicitly state** that the returned tree contains a complete continuation at every literal original challenge value, or that it contains the literal original full-frontend leaf's complete query subtree.

A tree anchored at segment 1's header can contain only counterfactual segment-1 challenge children. All later segment commitments may consequently differ from the commitments in the published original receipt. Its valid final PCS leaves therefore need not supply a decommitment for the published original later sources.

The independent `B` extraction removes this issue. The coefficient `T` is recoverable if one of the `E_j` routines is additionally proved to return a complete query subtree for the **literal original full frontend transcript**, so that its extracted assignment can be reused as `S0`. That strengthened marked-leaf guarantee must be proved, including its probability and cost. Merely writing “retain the base leaf” does not derive it from prefix anchoring.

If only the run relation is required, rather than the original deferred-query relation, a common PCS-only base can sometimes suffice for source comparison; it still needs an original-event extraction guarantee and contributes its own cost/error. It does not certify the original accumulator's full coefficient roster. For Candidate 1's full claim, use `B` as defined above.

## 5. What the local mixed-tree recurrence does and does not establish

The inspected local text defines original selected inputs, keeps them on suffix failure, uses cheap mismatch gates, and states

`s_r >= s_(r+1) - (Q+1)*e_r`,

`C_r <= b_r*C_(r+1) + (b_r-1)*(Q+1)*gamma_r`.

This addresses a serious cost/index pitfall and is a plausible template for the intended compiler. The source text's Definition 3.15 asks for a joint failure event on the same original accepted execution.

Nevertheless, a lower bound on `Pr[extractor succeeds]` alone does not prove

`Pr[original accepts and extractor fails] <= kappa`.

A logical countermodel is `A = 1_U` and `Success = 1_(not U)` for a fair bit `U`: the success and original-acceptance probabilities are both 1/2, so a zero-loss marginal recurrence can hold, but original-acceptance-and-failure has probability 1/2. This is a counterexample to the inference, not an asserted attack on the actual Akita implementation.

The tree compiler needs a construction whose successful output is gated by original acceptance and anchored to the original output/target header, or a direct proof of the joint failure event. A complete proof must check this property at the scalar weighted game and each recursion step; original-index retention is necessary but is not equivalent to this event property. With that guarantee, the union/assembly proof in §4 is complete. Without it, the asserted quantified per-segment extractor remains a missing premise.

This is not an impossibility of the protocol. The next construction closes frontend/history composition while using an original-event query-plus-PCS extractor as its only rewinding primitive, rather than assuming a new frontend-tree compiler.

## 6. Fixed-source local frontend certificate

For every fixed admissible primitive segment source assignment `W`, fixed transition statement/header, and adaptive prover strategy, give each relevant frontend stage `r`:

1. a parsed prechallenge transcript and exact challenge law;
2. deterministic incoming/outgoing truth predicates relative to the K tables reconstructed from `W`;
3. a bad set `Bad_r(W,t)` fixed by `W` and the prechallenge transcript;
4. a bound `Pr[c in Bad_r(W,t)] <= e_r`;
5. the implication: checked stage plus true outgoing obligations implies true incoming obligations unless the challenge is in that bad set.

It is uniform over all later messages. Challenge-free stages have empty bad sets. Define `epsilon_FE,j = sum_(r in j) e_r`.

For a sumcheck round, if the incoming claim is false, the sent polynomial and the true partial-sum polynomial differ nontrivially with degree at most `d`; truth can be restored only at one of at most `d` roots. This gives `d/|E|`. Power batching uses its actual polynomial degree, including even powers. Independent affine batching can use `1/|E|`. Reused reference coordinates must be represented in the certificate and charged at their actual ancestor draw. Deterministic aliases need exact table/point/value ownership, not a similarly named serialized passenger.

Virtual tables are deterministic reconstructions from the same original K sources. They are not independent free E-valued witnesses. Off-cube products of MLEs are not substituted for MLEs of Boolean-row products.

If all final owned leaves of an accepted segment transcript are true for `W` but its segment relation is false, traversing the obligations backward finds one of these bad stage events. This is the ordinary fixed-source soundness proof. It contains no accepting frontend tree and no knowledge axiom.

The certificate must genuinely cover the intended source predicates. Existing semantic Main Booleanity does not constrain active physical address-zero cells in zero-omitting columns; setting such a cell to 2 leaves its semantic reconstruction unchanged. Ordinary decoded RAM openings do not authenticate arbitrary raw digit-plane predicates. These omissions cannot be repaired by the composition argument.

## 7. Source-only rejection probes

Run `B` once. Original rejection or original `B` failure returns the corresponding result; a verified collision returns it. On valid original source output, retain `S0` and `o0`.

For each segment `j`, restore the original execution immediately before `I_j`, retaining its ordinary state/tape and the oracle table **only as it stood at that prefix**. Perform independent continuation trials:

1. sample the fresh root answer and all previously unexposed continuation entries from their exact ordinary laws;
2. run the cheap original prover/verifier continuation `C`;
3. if it does not accept with the same `I_j`, actual root key and target header, reject this trial cheaply;
4. only on that gate, invoke the bounded `Suf` for this trial's own sealed query-plus-PCS continuation;
5. at the first valid source output, compare the trial's `j` target projections with `S0`;
6. return a named collision on disagreement; on equality, check the declared segment relation on `S0` by deterministic reconstruction.

A trial's later sources, claim roster, seal and frontend transcripts can differ from the original ones. Its target sources cannot: the matched root header binds their original identities and views. The comparison ignores future sources and D/H.

A verified collision in a trial can terminate early. For the probability/cost upper bounds, use a dominating ghost process that continues until a valid source output. The ghost process is not a claim that a collision is ignored by the actual extractor.

After all comparisons, output the original source assignment and the deterministically assembled original run witness if its complete relation holds. Otherwise return failure, whose probability is bounded below. Do not output a counterfactual run's witness or statement.

## 8. Successful continuation law and expected work

Fix a segment `j`, a fresh query ordinal `i`, and a saved prefix `sigma`. Let `mu_(j,i)` be the subprobability measure of reached prefixes at the first query of a possible `j` root family. It is not conditioned on the eventual published proof or acceptance. Then

`sum_i ||mu_(j,i)|| <= q`.

Let

`a_i(sigma) = Pr[C accepts and selects the matching j root i/header | sigma]`,

`p_i(sigma) = Pr[B returns valid sources and selects that same root | sigma]`.

Both probabilities use a fresh independent continuation and the saved ordinary tape. No lower bound on `p_i` is assumed. A source output on the original branch selecting `i` has probability `p_i`. Prefixes with `p_i=0` have zero probability of starting the corresponding implemented probe on a valid original `B` output.

For `p_i>0`, the first successful trial's source distribution is

`nu_i(A) = Pr[one trial succeeds with source in A | sigma] / p_i`.

Proof: sum `(1-p_i)^(k-1)` times the numerator over trial counts `k>=1`. The distribution is independent of the original future continuation conditional on `sigma`, since all trial continuations are independent private samples.

The expected number of additional cheap trials for this segment, in the unconditional original execution experiment, is

`sum_i integral p_i*(1/p_i) dmu_(j,i) <= q`.  (P1)

For any nonnegative trial cost `Y` with conditional mean `c_i`,

`E[sum_(k=1)^N Y_k | sigma] = c_i/p_i`.

Reaching trial `k` depends only on earlier trials; independence of its current cost and success is not needed. Multiplying by the probability `p_i` of starting the probe cancels this denominator.

The expensive suffix is invoked only with probability `a_i` per trial. Therefore

`E[additional suffix invocations for j] <= sum_i integral a_i dmu_(j,i) = Pr[C accepts] <= 1`.  (P2)

The last equality uses the disjoint original selector events for the **same segment j**. A looser gate such as “same commitment somewhere” does not have this property.

The same argument bounds the additional expected suffix work for `j` by `P_S`, the expected gated suffix work of one original execution. Summing all segments and including the initial `B` gives

`W <= (1 + T*q)*T_C + (T+1)*P_S + W_compare + W_reconstruct/assemble`.  (P3)

Use a ghost computation performing every probe regardless of an earlier collision to dominate a stopping implementation. No independence between the completed segment tests is required. This is polynomial if `q,T,T_C,P_S` and the explicit source/reconstruction sizes are polynomial. It has no factor `product_j B_FE,j` or `sum_j B_FE,j`.

This is **unconditional expected work**, not work conditioned on a rare accepted receipt. The latter can be arbitrarily large.

## 9. Hypothetical-source coupling and frontend failure bound

The implemented probes occur after original acceptance. The probability proof uses an equivalent hypothetical experiment, not a claim that the actual extracted `S0` was fixed before the original frontend challenges.

### 9.1 Assign sources before the fresh root answer

At every first encounter with a new segment root family, before revealing its fresh first component, independently sample a hypothetical target source from `nu_i`; for `p_i=0`, use an arbitrary dummy. These samples do not affect the original execution. They may be computationally inefficient as a joint hypothetical construction; they are a coupling for the probability proof, not `q` actual probes.

For every selected original root on which `B` returns valid sources, the implemented probe can be coupled to this hypothetical source. It depends on the saved past, setup, ordinary tape and independent private continuation tables, not on unexposed original future oracle entries.

When its target projections equal those of `S0`, every original segment leaf certified by `B` is a true leaf for this pre-root hypothetical source as well.

### 9.2 Old descendant queries

A descendant key queried before the root's first fresh component was exposed contains a guessed value of that component. At the first-root query ordinal `i`, at most `i-1` earlier entries can be such candidate descendant keys. Conditional on the augmented past and hypothetical source, matching a particular guess has probability at most `gamma_0`.

A union bound over the entire original query ledger gives

`Pr[G_old] <= binom(q,2)*gamma_0`.  (P4)

Use a global ledger and unique canonical root-family tags. There is no extra factor `T` merely for enumerating the `T` published roots. A less expressive grammar in which descendant keys do not expose/bind the fresh root component does not justify this bound.

### 9.3 Fresh local challenges

At a fresh stage-family hit after its root source was assigned, the hypothetical source and parsed prechallenge transcript already determine the local residual and its bad set. The frontend certificate therefore bounds that event by the stage's `e_r`.

An atomic vector is charged once at its first query. Independently indexed coordinates with one common pre-vector transcript can be conceptually filled at their first hit; no adversarial message is inserted into that vector. If the real protocol interleaves messages, split it into the actual sequential certified stages.

There are at most `q` first-family hits. Thus the safe global bound is

`Pr[G_fresh] <= q * sum_j epsilon_FE,j`.  (P5)

A tighter bound is `q*max_(j,r) e_(j,r)` when every hit has the corresponding uniform certificate. The displayed sum is conservative and easier to inventory. This is not a further factor `q` for each possible final selected index: all hypothetical roots were assigned in one online experiment.

### 9.4 Backward truth argument

Suppose the original receipt accepts, `B` returns valid original query sources, every actual target probe agrees with `S0`, and some original segment relation is false. All its original deferred leaves are true by `B`, hence true for its hypothetical pre-root source. The backward local-certificate proof must find a bad stage. If its key was queried before that root assignment, it is covered by `G_old`; otherwise it is a fresh local bad event covered by `G_fresh`.

Consequently, with `beta_FE(q) = q*sum_j epsilon_FE,j + binom(q,2)*gamma_0`,

`Pr[original acceptance, valid B sources, all target comparisons equal, false run relation] <= beta_FE(q)`.  (P6)

There is no conditioning of the original challenge on acceptance, extraction success or source equality. All bounds are on the original joint event in the unconditioned augmented experiment.

## 10. Whole-history theorem for the probe construction

**Conditional theorem.** Assume the exact contracts in §2, the genuine fixed-source certificate in §6, the original-event bounded query-plus-PCS extractor (B1), the admitted source-only comparison inventory, and deterministic ordered-run assembly from linked segment relations. Then the source-probe extractor preserves the original published statement and receipt, has expected work bounded by (P3), and in the ideal joint setup satisfies

`Pr[V(o0)=1 and extractor returns neither an original run/query witness nor a valid named collision] <= kappa_B + beta_FE(q)`.

Proof: charge original `B` failure by `kappa_B`. On original valid source output, successful matched probes exist almost surely on all positive-probability prefixes by the geometric law. A source discrepancy produces the registered collision. In the remaining event, failure of the deterministic relation check is exactly the event bounded by (P6). The output is `S0` and its declared deterministic reconstruction, never a counterfactual output. Equation (P3) supplies the expected work. Average the proof over the setup, auxiliary data and ordinary tape; no perfect commitment uniqueness was assumed.

This proves polynomial whole-history **composition**, conditional on the original-event query-plus-PCS primitive and actual local frontend certificate. It replaces full nonlinear frontend interpolation trees by their fixed-source soundness errors and paid successful-source probes. It adds no hash-knowledge, canonical-state, inverse-shortness, curve, pairing, QROM or ARO assumption.

It does not prove the original primitive's compiler law. That law, and its expanded local provenance, remain the same independent obligations already present in Candidate 1.

## 11. Strict cutoffs and ordinary MSIS pricing

For a cap of `K` additional trials per segment, the probability of a started probe exhausting its cap is

`sum_i integral p_i*(1-p_i)^K dmu_(j,i) <= q/(K+1)`.

This follows from maximizing `p*(1-p)^K`; the exact maximum is `K^K/(K+1)^(K+1)`. Across all segments the safe additional cutoff error is `T*q/(K+1)`. This is an original-event bound, not a high-probability statement conditioned on acceptance.

If every suffix has strict cost `L_S`, the capped algorithm has work at most

`(1 + T*K)*(T_C + L_S) + W_compare + W_reconstruct/assemble`,

plus explicit saved-prefix and conditional-sampler work. The original suffix cutoff loss is already in `kappa_B`; a probe's failed suffix is part of its success probability `p_i`, not a new independent failure event charged once for every unsuccessful trial.

Alternatively stop the entire computation at work `L`. Markov gives additional loss `W/L` using (P3). Choose and price one stated cutoff construction; do not count the same cutoff event twice. Tiny concrete cutoff loss can require a very large paid runtime. Expected polynomiality is not 128-bit numerical admission.

After this strict cutoff, embed an ordinary MSIS challenge into one admitted view of a single uniform flat setup vector and construct all overlapping public matrix views and allowed dependencies from it. Simulate the classical ideal oracle and the entire bounded original/probe/suffix computation internally. No concrete setup-seed or hash preimage is assumed. The actual joint setup's statistical distance contributes `Delta_setup`.

For a named inventory `I`, collision advantages must be evaluated at the whole bounded computation time `T_I`, including replay, private tape sampling, copying, comparison, setup embedding and reconstruction. A final bound is

`kappa_B + beta_FE(q) + delta_cutoff + Delta_setup + sum_I Adv_MSIS,I(T_I,eta_I)`.

If a sampler changes a joint law rather than merely causing a recorded failure, include its explicit statistical-distance term. Internal D/H collisions remain in the ordinary inventory; cross-probe coalescence uses A/B/F only.

## 12. A strictly bounded two-run alternative

One can avoid the geometric rejection loop by making just one fresh matching continuation attempt per segment. This gives a worse collision reduction rather than the additive collision term above.

For a fixed segment and saved prefix, let `delta_i` be the probability that two independent valid-source samples from `nu_i` disagree on target projections. The hypothetical/implemented-source difference mass is `p_i*delta_i`. A two-run producer that starts on original valid source output and makes one independent matching continuation attempt produces that comparison collision with probability `p_i^2*delta_i`.

Let `a_j` be its total named-collision probability for segment `j`, including any direct suffix collision. Cauchy–Schwarz and `sum_i ||mu_(j,i)|| <= q` give

`sum_i integral p_i*delta_i dmu_(j,i) <= sqrt(q*a_j)`.

Thus a safe whole-run bound uses `kappa_B + beta_FE(q) + sum_j sqrt(q*a_j)` before replacing each `a_j` with its priced matrix-collision advantages and setup/cutoff terms. This route costs at most one additional bounded `B` execution per segment, plus comparison, but square-roots the lattice advantage. It is not a justification for claiming the additive probe theorem at two-run cost. Its separate inventory/reduction losses must be priced before selection.

## 13. Counterexamples and non-transferable shortcuts

1. **Deterministic frontend implication:** with true one-variable sum polynomial `g*(X)=1-X`, false claimed sum 0, and sent `g(X)=0`, the round check passes and the correct endpoint is 0 at challenge 1. True PCS leaves do not make one randomized frontend transcript a deterministic VM proof.
2. **Marginal success instead of the original joint event:** the fair-bit example in §5 shows that `s_r >= s_(r+1)-epsilon` without original-acceptance gating cannot establish the required accepted-original-output failure bound.
3. **Unanchored successful rerun:** a rerun can choose a different source/transition statement and prove that different relation. If the extractor returns that witness instead of the published original header's witness, individual success says nothing about the original run. The actual key, selected index, complete prefix and target source/view all belong in the gate.
4. **Target source chosen after its challenge:** a residual of the form `F(X)=X-r` selected after seeing challenge `r` vanishes at `r` identically as an experiment although it is a nonzero polynomial in X. The fixed-source root bound is inapplicable. Future primitive sources cannot be silently inserted into earlier target reconstructions.
5. **Future-table reuse in probes:** retaining all original later oracle answers while claiming a fresh independent continuation does not give the law `nu_i`; prequeried descendant values also invalidate a fresh-stage argument unless the old-key event is charged.
6. **Decoded equality instead of original source comparison:** RAM `D(d)=sum_l 32^l d_l` has a nontrivial kernel. Matching decoded values does not identify original supplied signed planes or A/B/F paths. The ordinary projection explicitly does not grant arbitrary raw-plane queries.
7. **Physical-source constraints inferred from semantic frontend truth:** zero-omitting Main maps give coefficient zero to the physical address-zero cells. Altering an active such cell to 2 preserves semantic frontend equations; its required zero/Boolean predicate needs its own owner and reduction.
8. **Unmetered oracle-dependent advice:** advice that supplies a future root or descendant answer can let a prover fix a residual with that known root before the nominal challenge call. Oracle-independent initial advice or included counted initialization is necessary; privately supplied execution advice is not a free correlated-RO oracle.
9. **Source handles mistaken for public semantics:** private Arc/pointer identity is a prover-ownership invariant. Public continuity is a canonical original `(shape,payload,role,view)` alias relation, with equality of extracted original source projections proved or a collision returned. A serialized receipt cannot inspect private pointers.
10. **SDK advice silently strengthened:** memory-mapped immutable advice objects, existential arithmetic advice and the SDK two-pass tape/cursor are different objects. Declared witness reconstruction and concatenation certify only the selected relation; no host compute_advice, exact tape-length or emulator adequacy theorem follows automatically.

## 14. Final recommendation

For a supplied-tree theorem, use the selective per-segment argument and the corrected independent-base accounting (T1)–(T2), making the original-event anchoring contract explicit. Do not claim literal-base-leaf retention from the local Lemma 3.22 wording alone.

For the simpler polynomial whole-history composition proof, use the fixed-source local certificate and the source-only rejection-probe construction of §§6–11. Its proof above is complete conditional on the original sealed query-plus-PCS extractor and actual frontend/source contracts. It has one globally priced classical oracle ledger, one original output, immutable original source identities, polynomial expected work, and an explicit cutoff/MSIS reduction.

Neither method admits the candidate numerically or substitutes for official-body provenance, source-domain/decoder coverage, the concrete indexed full-answer compiler, deterministic honest closure, or the exact final schedule for workloads 1/2/3/256. No performance or security conclusion is inferred from PCS-only controls.