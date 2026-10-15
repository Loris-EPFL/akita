# Published anchoring and additive whole-history frontend extraction

Status: mathematical review addendum; no implementation, benchmark, code-admission or numerical security claim. This addendum supersedes the unresolved anchoring question in §§4–5 of `AKITA_RELATION_ACCUMULATION_FRONTEND_COMPOSITION.md`. The earlier conservative extra original extractor remains correct but is not necessary when the explicitly marked original branch described below is retained. The source-probe alternative remains available and can remove large frontend-tree factors.

## 1. Primary evidence and what was actually read

The full published source is Giacomo Fenzi, Hossein Moghaddas and Ngoc Khanh Nguyen, *Lattice-Based Polynomial Commitments: Towards Asymptotic and Concrete Efficiency*, Journal of Cryptology (2024), article 31, 92 pages, DOI https://doi.org/10.1007/s00145-024-09511-8 . Primary repository PDF: https://infoscience.epfl.ch/server/api/core/bitstreams/9abd4c0d-3e31-4228-b600-28f51a7a511f/content . I independently read its §8 through this URL, not merely the local expanded Akita material.

Exact published evidence, using the parent's extracted `artifact://2432` line numbering:

- §8.1, pp.78–79, lines2338–2345: extraction first requires `M(J_original)=(1,i)`, then enough reprogrammings with the same first-message index `i`.
- Lemma8.2, pp.79–82, lines2365–2437: `J` is explicitly the first sampled entry; the count includes that entry. The proof states that `V=0` implies every coordinate count `X_l=0`, hence success is contained in the original `V=1` event (lines2403–2413). This is precisely the fact missing from an isolated marginal recurrence.
- Lemma8.3, pp.83–88, lines2445–2602: the weighted bound charges full suffix cost for matching indices and only cheap cost for mismatches. Its coefficient is `1+ell*(k-1)` on the expected full suffix cost, not a new Q factor per depth.
- §8.2, pp.88–89, lines2606–2617: the paper explicitly applies the games to multi-round extraction and permits heterogeneous coordinate counts/arities, using Attema–Fehr–Klooß for the recursion/support bounds.
- Definitions2.29–2.30, pp.29–30, extracted lines802–816: a coordinate-wise fork set has a central challenge which is itself included; full transcript trees recurse on these sets.

The reader omits the actual pseudocode artwork in Figures12 and13. I do not pretend to have read those images. The independently defined game below proves the required marked-original guarantee without assuming an unread line of the artwork.

A supplementary local read of `ResearchSources/AFK-Multi-Round-Fiat-Shamir.pdf` agrees with this construction: §6.1 Lemma7/Propositions1–2 preserve the first prover's index vector, retain its first suffix tree, and permit cheap early abort; §6.3 retains the original adaptive statement/proof/auxiliary output. This supplementary read is not being substituted for independently authenticated Akita lemmas. The proof below does not rely on its omitted figures or on interpreting it as independent authentication of the expanded Akita text.

## 2. The scalar game, explicitly retaining the original

First consider a finite ideal oracle with function table J. At a chosen scalar stage, each eligible oracle entry has an independent uniform challenge in a finite set C of cardinality N. Fix the ordinary prover's tape and other oracle entries. A suffix procedure has two outputs:

- an index I(J), obtained by replaying the original ordinary prover/verifier and retained even when suffix extraction fails;
- a bit V(J) and, when V=1, a valid suffix tree.

It has a cheap first phase exposing I, followed by the possibly expensive suffix work. Let Gamma(J) be the full work and gamma a uniform upper bound on the cheap phase. A marked original accepting transcript is retained in each successful suffix tree.

For the real, first table J, run this suffix procedure ONCE and retain its exact output, including its exact original full oracle answers. Let (V,I) be that output. If V=0, return failure and the original index/output; do not try to find a different accepting initial run. If V=1, retain the first successful suffix as child zero. Draw the other challenge values in a uniformly random order without replacement, keeping every other oracle entry unchanged. For each new value c:

1. Replay the cheap phase. If its index differs from I, abort that candidate before the expensive suffix work.
2. Otherwise finish suffix extraction. If it succeeds, retain its suffix tree.
3. Stop when k-1 additional successful children with index I have been retained, or the challenge set is exhausted.

Return success exactly when all k children exist. The original child is never discarded or replaced. This is an independently specified game, not a claimed transcription of Figure12.

The original-source header is part of the index. Equal indices must bind the complete relevant public/source prefix. A hash digest is not silently treated as injective; collisions or ambiguous encodings need their separately proved treatment.

## 3. Exact original-event loss for the scalar game

For an index i and all other oracle values j_{-i}, let

`a_i(j_{-i}) = |{c in C : I(j_{-i},c)=i and V(j_{-i},c)=1}|`.

This count is independent of the original value J_i. Let

`P0 = sum_i Pr[a_i(J_{-i})>0]`.

Conditional on J_{-i}, original success with index i has probability a_i/N. The game succeeds on that original event if and only if a_i>=k: a permutation of the remaining finite set eventually finds k-1 matching successes exactly in that case. Therefore

`Pr[V_original=1 and game fails]`
` = sum_i E[ (a_i/N) * 1_{1<=a_i<=k-1} ]`
` <= P0*(k-1)/N`.

The game success event is a subset of the first suffix success event, by construction. Consequently the displayed bound is JOINT original-success loss, not merely a comparison of two marginal probabilities. In particular the abstract countermodel `Accept=U, Success=not U` cannot describe this game.

There is also a slightly sharper scalar bound compatible with the published denominator: if s is game success and p is first-suffix success, the bad-fibre mass is at most P0-s, so `(1-(k-1)/N)*s >= p-P0*(k-1)/N`. The simpler additive bound above is sufficient for the proposed theorem and avoids relying on reader-damaged mathematical typography.

For randomized suffix procedures, preassign independent private coins to each possible full oracle table, fix that assignment, apply this argument, and then average. Distinct tested tables get the required private randomness. The one original call is retained, not re-invoked with independent coins when its output is reused as child zero. This is a proof coupling; it does not require exponentially allocating a table of private tapes.

## 4. The weighted cost bound, derived without marginal independence

Let d=k-1. Fix i and j_{-i}. Partition challenge values into a matching successes, b matching failures, and N-a-b mismatches. Additional scans occur only when the original value is one of the a successes.

For a particular matching-success value c, its unconditional probability of being an ADDITIONAL visited value is `min(d,a-1)/N <= d/N`: the original successful value is uniform among the a, and the additional good values selected by the random permutation form a uniform subset of size min(d,a-1) of the remaining a-1.

For any particular non-success value c (matching failure or mismatch), its unconditional probability of being visited is `min(d,a)/N <= d/N`. Conditional on an original success, among c and the a-1 remaining successes its rank is uniform. It is inspected before the d-th further success with probability min(d,a)/a, multiplied by a/N for original success.

Thus, for every fixed i,j_{-i}, the expected additional expensive matching work is at most

`(d/N) * sum_{c:I(j_{-i},c)=i} Gamma(j_{-i},c)`.

Summing over i and averaging j_{-i} yields at most d E[Gamma(J)], because each original table has exactly one selected index. Additional mismatches cost at most d P0 gamma. Including the first suffix call gives

`E[Cost_new] <= k E[Gamma(J)] + (k-1) P0 gamma`.

This derivation allows cost to depend arbitrarily on success, index, and table. It does not assume that expensive work and suffix success are independent.

Under the standard stronger weighted support bound, P0 in the cheap-work term can be replaced by a quantity at most Q; the conservative Q+1 bound is enough here. All actual conditional sampling, lookup, tree assembly and algebraic reconstruction work must also be charged, not just ordinary prover invocations.

Sampling without replacement can be implemented lazily, keeping only visited values and selecting a uniform unused rank. Arithmetic/search cost is polynomial in the number of visited values and log N. Expected polynomial query count is not yet a concrete strict-time reduction; cutoff pricing is still required.

## 5. Flat coordinate stars, nested coordinates, and raw oracle answers

For a flat coordinate-wise stage, keep the original tuple as the central vector and restore that same tuple before scanning each coordinate l. Retain the original suffix tree only once. If coordinate l requires k_l distinct values from a set of size N_l, then

`b = 1 + sum_l(k_l-1)`,

`e = sum_l (k_l-1)/N_l`,

`Pr[first suffix succeeds and full star fails] <= sum_l P0_l*(k_l-1)/N_l`,

`E[Cost_star] <= b E[Gamma] + sum_l (k_l-1)P0_l gamma_l`.

Every retained sibling agrees with the ORIGINAL central tuple in the other coordinates. Updating the central tuple to a previously accepted sibling is not the flat-star construction. If the algebraic extractor instead needs nested coordinate interpolation, perform the scalar recursion in the required order, with an explicitly marked original child at every level. Its branching factor is the product of the coordinate arities rather than the flat-star sum.

For a raw-word/bitstream oracle, the game must vary DECODED challenge values, not merely distinct raw answers. It needs the already stated exact conditional full-answer sampler. One way to justify the coupling is to preassign, for each accepted challenge tuple c, an independent full raw answer tau_c with the true conditional law given c. Choose c_original uniformly and use tau_{c_original} as the real initial answer; lazily sample the other assignments only when tested. This gives the correct marginal raw-answer law and the original answer is retained exactly. Flat coordinate stars use the same tuple-level coupling while other decoded coordinates are held fixed.

This step is not implied by distinct hash words, rejection sampling in an unanalysed wrapper, or a reused challenge reference. A stage's polynomial/tree certificate must match its real causal exposure and all dependent uses. Reused challenges are branched at the appropriate ancestor, with all their uses covered by that ancestor's algebraic extraction lemma; they are not new independent coins.

## 6. Why one Q-query ledger suffices

For a scalar root role r, let I_r(J) be the root key selected by the ORIGINAL ordinary prover execution, not a key chosen by the suffix extractor's private forks. Let S(J) be the original producer's queried entries, including any efficiently generated oracle-dependent initialization counted in its budget. If i is not queried, changing only J_i cannot change the producer's output. Therefore any i whose fibre has a matching successful selected index must lie in

`S(J) union {I_r(J)}`.

Pointwise this set has size at most Q+1, giving P0_r<=Q+1. If the verifier's queries are included in q and it actually queries every selected root, the corresponding bound can instead be q.

Private recursive forks do not become original Q-budget queries; their execution and all simulator queries are paid in the extractor/MSIS producer's runtime. This is the same distinction as in the published multi-round construction. An unmetered oracle-dependent auxiliary table, offline precomputation, or another session's unrecorded prior oracle state cannot be hidden behind this ledger.

The oracle table can be implemented by one consistent lazy extension within a sampler invocation, preserving all originally exposed entries except the explicitly reprogrammed target. A reset that inconsistently resamples non-target answers is not justified by this game.

## 7. Marked-original multi-round recursion

Let the ordinary producer/verifier's actual first execution produce o0, including its statement, proof, auxiliary output, all public/source metadata, and its acceptance bit A0. The base procedure returns that record and its one complete transcript, succeeding precisely when A0=1.

Process stages backwards. At each active scalar/star stage r, apply the preceding game to the already defined suffix procedure. Its FIRST call is the suffix extraction for the original table J and is retained as the marked child. Only afterwards are siblings sought. Preserve the original selected root index even on failure. At an inactive stage, use arity one: return the original suffix procedure and mark its original path; no new fork or loss is added.

Induction gives all of the following:

1. A successful current extraction implies success of its first suffix extraction and therefore original acceptance A0=1.
2. Its marked path contains the actual transcript o0, not just a transcript with the same first commitment or the same statement.
3. At each fork, matching root indices preserve the complete root prefix. Hence all newly appended suffix trees have the right trunk. Their future messages/sources may vary.
4. The output o0 itself remains unchanged, including when a counterfactual proof chooses another future statement or auxiliary output.
5. The original-event failure probability telescopes, because every success event is contained in its first suffix success event under the explicit coupling.

For active stages of branch factor b_r and root loss e_r, with P0_r<=Q+1,

`Pr[A0=1 and marked complete-tree extraction fails] <= (Q+1)*sum_r e_r`.

A conservative weighted recurrence is

`C_r <= b_r C_{r+1} + (b_r-1)(Q+1) T_C`,

where T_C includes the cheap original-producer/verifier replay. Let B=product_r b_r and take the one ordinary run as base. Ignoring separately charged algebraic reconstruction overhead, this solves to

`C_tree <= [B + (Q+1)(B-1)] T_C`.

Using the sharper weighted support bound Q yields `[B+Q(B-1)] T_C`. Neither expression multiplies Q once per stage.

This independently derived recursion establishes the original-event property. No new model assumption of an 'anchored compiler' is necessary if the implementation/proof adopts this specified game and verifies the existing ideal-oracle/conditional-sampling, prefix and algebraic-tree conditions. A mere standalone marginal recurrence with a different sampler would still be insufficient.

## 8. Application to one target segment at a time

For each original segment j, activate:

- the dependency-closed frontend stages whose complete accepting trees reconstruct that segment's relation;
- the entire final query accumulator and ordinary PCS suffix.

Use arity one for other segments' frontend stages. Let B_FE,j be the actual segment frontend tree factor, B0 the full query-plus-PCS factor, e_FE,j its actual degree/cardinality root-loss sum, and e0=e_query+e_PCS. Then

`B_j = B_FE,j * B0`,

`Pr[A0=1 and E_j fails] <= (Q+1)(e_FE,j+e0)`.

The deterministic frontend reconstruction theorem is a separate algebraic obligation: true leaf scalars plus a COMPLETE appropriately branched target tree must reconstruct the target relation or a certified source collision. A probabilistic fixed-source frontend certificate alone does not automatically supply this tree reconstruction algorithm or its arities. In particular, a large power-batched root can require many interpolation values; calling every frontend stage degree two is unjustified.

All primitive source commitments/views needed to interpret target j must be in its relevant pre-root header. Across its frontend forks compare only those target-original A/B/F projections with their actual weak decommitment certificates. Future segments' source commitments and query seal can change. Do not require common D/H across frontend/query forks; retain ordinary same-inner-fork D/H obligations. Later deterministic virtual tables are derived from primitive sources rather than assumed committed early. Truly later primitive sources need the separately proved causal decomposition or earlier fixation.

The key improvement over the earlier conservative construction is now proved: the marked original path through ALL frontend stages reaches the literal original complete frontend prefix/seal. Under that prefix, E_j includes a COMPLETE query-plus-PCS subtree, not merely one PCS leaf. Extracting that subtree gives original sealed-query sources satisfying every original coefficient equation.

## 9. Recovering the proposed T coefficient and original-output consistency

Run E_1. If it succeeds without a certified collision, take S0 from the COMPLETE query subtree below its marked original frontend prefix. It covers the entire original source roster, even though E_1's other future-segment branches can have different rosters. Reuse this S0; no independent E0 run is needed.

For each j, run E_j on the same original o0 and retained original exposed oracle record, with its own private forks. In E_j, target sources are common across its frontend branches by the source-coalescing lemma or a returned collision. Its marked original query subtree uses the exact original target commitment/views. Compare those original target projections with S0's target projections. With their certified radii/relaxation factors, disagreement gives the named cross-source A/B/F collision; equality permits transferring E_j's reconstructed original segment relation to S0.

After all successful target reconstructions, check the original public boundary/continuity/VM conditions and assemble a witness for the ORIGINAL full output. Nothing requires agreement on future-source commitments in counterfactual branches. Agreement is only with original targets, via S0 at the marked original query prefix.

Union bound over the T original-event extraction failures requires no independence, giving

`Pr[A0=1 and no original run/query witness or certified collision]`
` <= (Q+1)[sum_j e_FE,j + T*e0]`

before cutoffs, sampling deviations and setup/MSIS terms. Structural tree size is

`sum_j B_FE,j * B0`,

not the product of all frontend factors, and not `[1+sum_j B_FE,j]*B0`. The latter remains a correct conservative alternative only if one deliberately runs an additional independent original query extractor.

This resolves the particular history-composition gap and recovers the parent's proposed coefficients. It does not prove the currently unadmitted candidate's actual frontend algebraic certificates, source coverage, sampler law, dimensional/radius closure or concrete profile.

## 10. Strict-time/MSIS qualification and source-probe comparison

All source coalescing is performed on certified original projections, not by bounding weak source s itself. Any returned collision is converted using the actual mixed-radius formulas and original named uniform setup view. The reduction's strict runtime includes every target tree, simulator/conditional sampler, ordinary producer replay, source decoding/comparison, algebraic reconstruction and stopping rule. Expected polynomiality alone does not prove a finite concrete security claim.

For total expected work W, a strict total budget L loses at most W/L by Markov, in addition to explicitly distinct sampler/setup cutoffs. Each named MSIS call is priced at the resulting full L-dependent producer runtime and its certified collision norm. The source set, session/grammar, RO budget and finite horizon remain admission gates.

The source-probe theorem from the earlier note can materially reduce this runtime. Its approximate expected work is

`W_probe <= (1+Tq) T_C + (T+1) P_S + comparisons/assembly`,

where P_S is the expected work of one original-event query-plus-PCS suffix extraction B without frontend trees. By contrast the selective-tree price is approximately

`W_tree <= sum_j [B_FE,j*B0 + (Q+1)(B_FE,j*B0-1)] T_C`

plus the required reconstruction work. Thus the probes remove every B_FE,j multiplier, exchanging them for T source-only suffix extractions and Tq cheap trials. This is a genuine structural improvement when nonlinear frontend tree factors are large; it is not a numerical security claim and need not improve every small profile.

There is also a statistical tradeoff. The probe bound is

`kappa_B + q*sum_j e_FE,j + binom(q,2)*gamma_root`

before runtime/setup/MSIS cutoffs. The additional old-descendant-key term can be much more expensive than the selective tree's linear-Q degree error. For a single 128-bit first root coordinate, the conservative q^2 term is not negligible for a 2^64-query budget. A larger atomic root alphabet helps only if the actual canonical descendant keys bind that entire uniformly sampled root tuple; partial-coordinate guesses cannot be priced by the full tuple's cardinality. Therefore select the cheaper extractor only after pricing BOTH its strict runtime and its actual root-horizon statistical loss.

## 11. Result

The primary published §8 game supplies the previously missing original-success gate; the independently derived marked-child recursion proves literal original retention and its joint original-event guarantee. The sum-of-target-trees composition can therefore be specified and proved in the already approved classical indexed-ROM model, with the parent's `sum_j B_FE,j*B0` work and `(Q+1)[sum_j e_FE,j+T*e0]` error structure. No extra anchoring MODEL assumption is required. The root protocol remains conditionally specified and unadmitted until its actual frontend/tree/source/sampling/parameter obligations are discharged. The source-probe proof remains a potentially much cheaper alternative, with a distinct old-query security horizon that must not be hidden.
