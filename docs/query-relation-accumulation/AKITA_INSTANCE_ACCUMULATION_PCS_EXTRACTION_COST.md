# Native PCS-only extraction: mixed-tree and derived-producer cost

## 1. Result and invariant

**Source-first extraction can remove the complete frontend interpolation tree from the proposed reduction, but it does not remove the native PCS tree.** For a declared ordinary Akita suffix, the latter has the exact mixed product below. Every complete nonterminal strong suffix contributes at least `8^mu'` from its range anchor and fused relation rounds alone. With the paper's practical range bases, its range rounds strengthen this to at least `32^mu'`. Neither statement is a lower bound on honest proving, on every possible extractor, or on the number of relevant native oracle keys.

No selected outer receipt schedule, concrete extraction implementation, or strict derived-Q32 certificate exists in the named profile/source evidence inspected here. The reviewed finiteT16 inner profile is not such a certificate. The obstruction is therefore an **unclosed admission ledger**, not a universal impossibility result. A smaller proved extractor, tighter query accounting, a suitable new outer schedule or a separately approved policy could change the outcome; none is silently selected here.

**Later results:** the reviewed [paired input-local emulator](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md) replaces the unrestricted citation; §7 uses corrected costs. Reviewed [private continuation](AKITA_INSTANCE_ACCUMULATION_PRIVATE_ARO_CONTINUATION.md) separates actual queries from virtual work. The separately reviewed [native source-first compiler](AKITA_INSTANCE_ACCUMULATION_NATIVE_SOURCE_FIRST.md) replaces only nodes with admitted local contracts and retains other factors. Neither changes this note's exact cost of the original configured tree or establishes a selected numerical receipt.

The anchor stays unchanged: original native K sources, E challenges, original inner commitments and 33 matrix roles, at most 16 segment updates, and the complete final native decider. This note studies the ordinary **outer** native PCS suffix in the product experiment with fixed `Theta=(ro,wo,Vhat)` and independent outer ideal streams `H_o`. It assumes neither CH-I/CH-O nor concrete-hash knowledge. All frontend/source-first conclusions require their separate mathematical compiler and coupling proofs.

## 2. Evidence and meanings of cost

Primary evidence is the indexed [Akita paper](SOURCE_PROVENANCE.md#akita), obtained through qmd at `qmd://lattice-papers/akita.md`:

- Lemma 3.22, lines 1509–1547: mixed-tree sampling and original-index weighted-cost recursion.
- Tensor reduction, lines 1370–1404: nested row selection, normalized claim batching, and degree-two rounds.
- Range pipeline, lines 2440–2489; Table 6, lines 2981–2992: actual product-stage degrees, sequential rounds and equality-factored message compression.
- Norm challenges, lines 2585–2627; Theorem 10.27–Lemma 10.28, lines 4041–4059: ring arities, fused degree-three relation, nested range anchor, and reuse of the final range tree for binary support.
- Admissibility Eq. (176), lines 3216–3234; Eqs. (225)–(228), lines 4143–4173; Theorem 10.31 and Remark 10.32, lines 4175–4197.

The paper's [primary publication locator](https://eprint.iacr.org/2026/1983) is provenance; detailed claims use the indexed text. The later [ARO scope audit](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md) refutes the unrestricted emulator guarantee in the [CCGOS accepted manuscript](SOURCE_PROVENANCE.md#supplemental-primary-sources). Its independently reviewed paired input-local replacement supplies the bounded-emulation resource contract used here.

Keep these quantities separate:

1. **Tree geometry:** how many leaf continuations and interpolation nodes the configured complete-tree proof requires.
2. **Transcript-generation work:** original executions, matching continuations, rejected trials and conditional raw-stream generation.
3. **Algebraic extraction work:** witness reconstruction, source comparisons, decoding and collision production after descendant certification.
4. **Actual Theta calls:** every original/replayed/probe/projection/predicate call, including failed trials.
5. **Mapped native stream keys:** only the keys actually exposed to the native ideal-stream interface after the fixed-input compiler simulation.
6. **Ordinary SIS reduction CPU:** the entire bounded computation, including one persistent ARO emulator, setup embedding and collision extraction.
7. **Honest costs:** installation, a single proving execution, memory and verifier work. Items 1–6 are not measurements or lower bounds for this item.

## 3. Exact schedule geometry

### 3.1 Universal mixed-stage formula

Let `R_PCS=(1,...,R)` be precisely the retained suffix's challenge ledger. Frontend challenges held fixed by a source-first procedure are not in this list. Conversely, an application batching, source-range or multipoint prefix cannot be omitted merely because it is inconvenient: include it if the chosen extraction lemma needs it or prove that a separate fixed-source argument owns it.

For stage r, record coordinate arities `k_(r,u)` and the proved flat/nested organization. Then

\[
 b_r=\begin{cases}
 1+\sum_u(k_{r,u}-1)&\text{flat coordinate family},\\
 \prod_u k_{r,u}&\text{nested family},
 \end{cases}\qquad
 P_0=1,\quad P_r=P_{r-1}b_r,\quad K_{\rm PCS}=P_R.
\tag{1}
\]

This is Eq. (225), not a new approximation. `P_r` is the number of stage-r continuation slots in the homogeneous declared tree. Counting logical stage-boundary nodes gives `V_PCS=sum_(r=0)^R P_r`. For a nested vector stage, internal interpolation nodes also count: if it has a coordinates in its chosen nesting order, its internal per-parent count is `1+sum_(v=1)^a product_(u=1)^v k_(r,u)`. Use these counts to price node work; `K_PCS` alone is not its CPU cost. For branch-dependent schedules use the actual finite recursion `Leaves(node)=sum_child Leaves(child)` and `Work(node)=local_work+sum_child Work(child)`; a uniform product only applies to a uniform ledger or a declared worst-case envelope.

A vector sent in one oracle answer remains one actual protocol stage. A nested tree may branch in its coordinates without creating fictitious prover messages or external H_o queries. Conversely, compressing round messages does not turn sequential sumcheck rounds into one flat stage.

### 3.2 Factors to instantiate

| Native component | Declared geometry | Complete-tree factor |
|---|---|---:|
| Fold at level j, including the terminal fold | `W_(j,g)` challenge coordinates per source group | `F_j=1+sum_g W_(j,g)` |
| Normalized independent affine combination of Lambda claims | Lambda−1 independent coordinates, first coefficient 1 | Lambda, not `2^(Lambda−1)` |
| Nonidentity shared tensor prefix h | row arity kappa_h, N_tensor,h claims, maximal tail arity m_h | `2^kappa_h * N_tensor,h * 3^m_h` |
| Identity tensor prefix | actual identity case | 1 |
| Nonterminal initial range anchor | witness Boolean arity mu'_j | `2^mu'_j`, not `1+mu'_j` |
| Range product stage s | inner product degree d_(j,s), mu'_j rounds | `(d_(j,s)+2)^mu'_j` |
| Interstage range frontier | m_(j,s) normalized claims | m_(j,s) |
| Ring reduction | maximum native active ring dimension d_max,j | `xi_j=2*d_max,j` for quotient lift; `d_max,j` for quotient-free |
| Relation-row batching | n_rel,j actual scalarized rows | `2^ceil(log2 n_rel,j)` |
| Fused relation/range/norm sumcheck | mu'_j sequential degree-three rounds | `4^mu'_j` |
| Producer's setup-product sumcheck | offload flag o_j and padded setup arity nu_S,j | `3^(o_j*nu_S,j)` |
| Receiver's fresh internal claim combination | N_clm,h claims fixed at its required payload boundary | N_clm,h, separately from tensor batching |
| Application batching actually retained | Lambda_a original claims | Lambda_a |

`W_(j,g)` is number of claims times source blocks, not the source's coefficient count. A fold is flat even with many blocks. Relation rows must use the selected quotient/quotient-free route, scalar field rows and all active A/B/D/F/H rows; row batching is not a flat fold. Setup offloading creates both a producer stage and separately owned receiver stages. The terminal has its real fold/tensor/combination factors, **not** an invented copy of the nonterminal range/relation pipeline.

The range product degree is d, but the equality-anchored summand's individual degree is `D=d+1`. Eq. (225)'s ordinary degree-D scalar interpolation therefore has arity `D+1=d+2`. The equality-factored format sends only d coefficients without changing the stated paper extraction ledger. A sharper extractor exploiting additional structure would require a new proved ledger; its smaller message size is not that proof.

Binary support adds its own coefficient-separation obligation, but **no second `2^mu'` anchor**: Lemma 10.28 reuses the final range stage's nested tree. Likewise Euclidean norm fusion does not add mu' rounds. It adds the actual scalar separation stages below.

### 3.3 Exact local product with explicit remaining scalar-stage ledger

Write `s_j=ceil(log2 n_rel,j)`. Let `C_j` be the product of Eq. (1) factors for the declared relation/range/binary and norm **scalar separation stages**, excluding the range-frontier combinations already listed. This is a computable ledger, not an unspecified polynomial:

- The carried-range coefficient gamma has degree one.
- If binary support is nonempty, zeta_bin has degree one. If these are one proved jointly flat stage, their factor is `1+1+beta_j=2+beta_j`; if the schedule supplies separate scalar stages, their product is `2^(1+beta_j)`. Do not select the cheaper stage partition without the corresponding transcript/extraction structure.
- A direct Euclidean norm route adds degree-one norm merge and degree-one norm-to-witness binding.
- A digit-expanded route adds a scalar pair-batch of degree `N_pair−1`, a degree-one norm merge, and a shifted-power binding of degree delta_f. With separate scalar stages these contribute `N_pair * 2 * (delta_f+1)` for that group. If several coordinates share an admitted flat family, apply Eq. (1) to that family instead of multiplying independent-looking scalar factors.
- For independent direct-route scalar stages the contribution is `2*2`. For a coefficient-norm route there is no such norm contribution.

Stages separated by a challenge-dependent prover message cannot be fused just by noting that each coordinate separately has low degree. The exact declared stage list is authoritative.

For every nonterminal level j the native local factor is therefore

\[
 L_j=F_j\,\xi_j\,2^{s_j}\,2^{\mu'_j}
 \left[\prod_{s=1}^{S_j}(d_{j,s}+2)^{\mu'_j}\right]
 \left[\prod_{s=1}^{S_j-1}m_{j,s}\right]
 C_j\,4^{\mu'_j}\,3^{o_j\nu_{S,j}}.
\tag{2}
\]

For the paper's declared tensor and normalized batching geometry,

\[
 K_{\rm PCS}=
 \left(\prod_{a\in\mathrm{retained\ batch}}\Lambda_a\right)
 \left(\prod_{h\in\mathrm{nonidentity\ tensor}}
 2^{\kappa_h}N_{\rm tensor,h}3^{m_h}\right)
 \left(\prod_{h\in\mathrm{internal\ combination}}N_{\rm clm,h}\right)
 \left(\prod_{j\in\mathrm{nonterminal}}L_j\right)F_T.
\tag{3}
\]

All factors occur once at their owner. No factor in (3) asserts that the outer receipt has selected that stage; the formula becomes numeric only after its full geometry is supplied. If the selected variant changes the paper's relation, derive its new stages rather than reusing (2) by name. Original-handle raw-query variants in [SOURCE_QUERIES §§8–9](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md) change physical carried widths, channel/occurrence counts and rows, hence mu', n_rel, W and matrix inventory. Repeated original payloads are not free repeated occurrences.

## 4. What the native suffix alone forces

Every retained ordinary nonterminal suffix contains the initial range anchor and degree-three relation rounds. Dropping all other factors, which are at least one,

\[
 K_{\rm PCS}\ge\prod_{j\in\mathrm{nonterminal}}8^{\mu'_j}
 =2^{3\sum_j\mu'_j}.
\tag{4}
\]

For the paper's practical bases `b* in {4,8,16,32,64}`, at least one inner range degree is at least two. Thus

\[
 K_{\rm PCS}\ge32^{\sum_j\mu'_j}.
\tag{5}
\]

More precisely, including only anchor, range rounds and relation rounds, each level contributes:

| b* | Inner range degrees | Factor per witness variable |
|---:|---|---:|
| 4 | (2) | 32 |
| 8 | (4) | 48 |
| 16 | (2,4) | 192 |
| 32 | (4,4) | 288 |
| 64 | (2,4,4) | 1152 |

Frontier, fold, ring, row, scalar-combination, tensor and offload factors only increase these values. These are lower bounds for the **prescribed complete native tree when extraction actually constructs it**, not unconditional expected-run lower bounds: a prover may reject, extraction may stop at a collision, and a different source extractor need not enumerate that tree.

For orientation only, (4) exceeds `2^32` if `sum mu'>=11`; (5) does so if `sum mu'>=7`. Neither threshold proves native Q32 is exceeded: a tree slot is not necessarily a fresh relevant native query. It proves that replacing extraction cost by one suffix execution, or inferring a small reduction solely from a flat fold factor, is unsound.

The existing [compact inner profile §3](AKITA_INSTANCE_ACCUMULATION_COMPACT_PROFILE.md) has shrinking outputs of arities 21, 20 and 19. Were its three complete strong reductions enumerated by this tree route, (4) alone would give `2^180`; the stated base-32 geometry would give `288^60` before other factors. This is a transparent **inner example**, not a selected outer schedule or a claim that the source-first receipt traverses those inner extraction trees. The guest verifies an inner archive; outer extraction authenticates the guest's own original sources under its own PCS schedule.

Eq. (176) permits a fixed polynomial in explicit instance-and-witness length, not in log length. With `N=2^mu'`, even `32^mu'=N^5` is polynomial. Consequently, asymptotic admissibility is entirely compatible with an unusable concrete derived-query/runtime budget. This is not a contradiction in the paper.

## 5. Calls and runtime: what Eq. (227) actually supplies

Let `Q_o` be the original algorithm's external H_o query ceiling under the admitted stream convention. It includes original nonce/grinding attempts and the original verification queries if that is how the bounded procedure is defined. In a source-first B, PCS counterfactual H_o answers are private simulated streams. They consume computation but do not become additional original external oracle indices. Original H_o and actual Theta are different ledgers.

Lemma 3.22 gives, for a specified charged-work metric,

\[
 C_r\le b_r C_{r+1}+(b_r-1)(Q_o+1)\gamma_r,
\quad
 C_1\le K_{\rm PCS}C_{R+1}
 +(Q_o+1)\sum_{r=1}^R P_{r-1}(b_r-1)\gamma_r.
\tag{6}
\]

Here gamma_r must bound the real cheap original replay, index/prefix check and conditional-answer sampling work. It is not zero because the trial was rejected. With a uniform bound gamma,

\[
 C_1\le K_{\rm PCS}C_{R+1}
 +(Q_o+1)(K_{\rm PCS}-1)\gamma.
\tag{7}
\]

The telescoping identity `sum P_(r−1)(b_r−1)=K_PCS−1` proves (7). This is a recurrence **in supplied cost bounds**, not a recovered concrete Akita implementation count. Algebraic node extraction, transcript checks and witness output must additionally be included at their actual nodes or in the charged continuation costs. A weighted count of Theta calls obeys the same sampling calculation when its replay/continuation bounds are supplied: these costs are nonnegative and additive, and conditional sampling itself need not call Theta. Without an implementation there is no numeric gamma, base continuation cost or node cost.

A conservative realized ledger is

`N_run = original executions + all trial executions/resumptions`,

`t_Theta = t_initial + sum_(all executions/resumptions v) t_Theta(v)`
`          + t_projection + t_output predicates + t_source comparisons`,

with overlaps counted once. Pure algebraic source comparisons make no Theta calls; any auxiliary check that does must be included. With a uniform per-run bound q_Theta, one may upper-bound the sum by `q_Theta*N_run`, but `N_run=K_PCS` is not justified. Rejected trials, sampler retries and original verifier work remain.

The paper's actual theorem is

`E[T_FS] <= (Q_o+1) poly(lambda, |sch|, K_PCS, T_A, T_ver, T_filter)`,

where `T_filter=max_r sum_u L_(r,u)/a_lower_(r,u)` and the polynomial depends on the chosen rewinding implementation and algebraic extraction. **There is no paper-given concrete coefficient or exponent to substitute.** Neither `(Q_o+1)*K_PCS` nor the honest proof-generation time is an exact call/runtime claim licensed by Eq. (227).

Theorem 10.31 returns full-grid algebraic decommitments or a registered matrix collision. Remark 10.32 explicitly denies canonical short-state recovery. Retain weak certificates, not only decoded evaluations; source-first cross-continuation comparisons still require the real B/F digit-difference or unit-product A collision and all admitted comparison norms.

## 6. Source-probe accounting and strict cutoffs

The following is a resource lemma, conditional on a source-first coupling that actually has the stated prefix/selection law. It does not independently prove fixed-source frontend soundness, challenge freshness, original-output preservation, or safety of descendant-key prequeries.

Fix Theta, original prover tape and a reached original external query prefix h at index i. Fresh continuation randomness includes future H_o answers and fresh private PCS-extraction tapes. Let:

- `A_(i,h)` be the cheap event: the replay selects **this same original first-frontend index and exact key/prefix** and accepts;
- `a_(i,h)=Pr[A_(i,h)|h]`;
- `S_(i,h)` be `A_(i,h)` plus successful bounded PCS source extraction; `p_(i,h)=Pr[S_(i,h)|h]`.

A probe runs the expensive PCS procedure only after A, repeats independent continuations until S, and returns its source. Positive original S probability implies p>0; p=0 prefixes carry no successful-original mass. If one trial has nonnegative cost Y, possibly correlated with its success, renewal gives

\[
 \mathbb E[\text{total trial cost}\mid h]
 =\frac{\mathbb E[Y\mid h]}{p_{i,h}}.
\tag{8}
\]

Proof: with fresh IID trials, total expected cost is `E[Y]+(1-p)*E[remaining cost]`. No independence of Y from success is needed. Equivalently use monotone convergence for the nonnegative cost sum.

Let `dmu_i(h)` be the measure of reaching query prefix h in the **original** run. If the coupling triggers probes only on original S, the original weighting p cancels the denominator:

\[
 \mathbb E[\mathbf1_{S_{\rm original}}\,\text{probe cost}]
 =\sum_i\int_{\{h:p_{i,h}>0\}} \mathbb E[Y\mid h]\,d\mu_i(h).
\tag{9}
\]

For unit cost per trial this is at most the original external query ceiling (or Q_o+1 if the proof includes an explicitly treated unqueried-output index). The successful output of a probe is sampled from S conditional on h independently of the original future; the source-first proof must establish that this is the coupling it uses, rather than condition on acceptance and assert independence afterward.

### Tighter expensive-PCS accounting

For unit cost only when the gated PCS procedure is invoked, `E[Y|h]=a_(i,h)`. If A uses the exact unique original selected index, its events over i are disjoint in the same reference original experiment. Thus

\[
 \sum_i\int a_{i,h}\,d\mu_i(h)
 =\Pr[\text{original accepted run selects some admitted index}]\le1.
\tag{10}
\]

Consequently the **unconditional, original-success-weighted expected number of additional expensive PCS invocations is at most one**, even though cheap probes can have a Q_o factor. Failed matching PCS attempts are included in this count. More generally, with Y the complete gated suffix cost, the right side of (9) is at most the expected gated suffix cost in one original-reference run; equality holds if prefixes with p=0 have no gated suffix cost. This can avoid multiplying the expensive PCS expected cost by Q_o a second time. It never turns the PCS tree itself into a single prover execution.

The refinement requires exact selected-index/acceptance gating and identical conditional continuation laws. If the gate is merely “same prefix”, if several candidate indices qualify in one run, or if it invokes PCS before detecting an index mismatch, use the corresponding sum of gate probabilities instead; it need not be at most one. The original B invocation, all cheap probes, source comparisons, projection and final native predicates remain additional costs.

### Cutoffs and tails do not disappear

A per-prefix cap of K independent probe trials has original-weighted failure

`sum_i integral p_(i,h)*(1-p_(i,h))^K dmu_i(h)`.

Since the maximum of `p(1-p)^K` on [0,1] is `K^K/(K+1)^(K+1)`, this is at most `Q_o/(K+1)` (replace by Q_o+1 under the corresponding convention). This bound is unconditional, not conditioned on a rare accepting original run. A strict global work/query cap L has `delta_cut<=E[Z]/L` by Markov, if Z includes all work to which the cap applies. A sharper admitted tail may replace this bound.

For a geometric trial count G, `E[G^2]=(2-p)/p^2`; after original weighting this becomes `(2-p)/p`. Arbitrarily rare p therefore defeats any general higher-moment inference from (9), even with the improved expected expensive-PCS count. Constant-cost trials already exhibit the problem. With a strict cap B is bounded and the moments are finite, but may be enormous and depend on the cap.

A required concrete cutoff loss epsilon may need `L>=E[Z]/epsilon` under Markov, or `K+1>=Q_o/epsilon` for the coarse trial tail. These are sufficient upper-bound admissions, not necessary lower bounds on the best cutoff. In particular, no small failure probability follows merely from acceptance of the original proof. Inner Q32 is a hard maximum convention, not an expected-query allowance. Enforcing that cap and charging its failure event is valid; calling the event negligible without a tail proof is not.

## 7. From actual Theta calls to native keys and ordinary CPU

### 7.1 All-query ledger

Use the reviewed **input-local paired** emulator, not unrestricted CCGOS Construction 5.11. Let h be distinct heads activated through RO or WO and v the extended-query count of the complete bounded producer/predicate. Keep one persistent augmented state through every extraction/probe reset. Its complete-run error is at most `min(1,v/2^lambda_A)`.

The retained implementation uses at most h RO plus h WO calls. The ro-only implementation samples/caches each head's coins and evaluates B against its cached ro(x), giving

`Q_component <= h`.

B's repeated local gates are CPU work; ro-free B still needs the paired RO fetch. Include all initialization, query-authentication production and final predicates. ARO-to-ro transformation precedes fixed-input chain simulation; an old graph cannot remain attached to a replaced ro.

For strict logical-machine cap L, let `C_M(h,v)` be the repaired emulator's complete bit cost, including sparse evaluation, constraint detection, feasibility, sampling and tables. A safe ordinary reduction bound is

\[
 T_{\rm SIS,I}\le L+C_M(h,v)+T_{\rm embed,I}
 +T_{\rm collision,I}+T_{\rm projection/predicate},
\tag{11}
\]

Remove overlaps already charged in L. C_M is the explicit polynomial from the paired-emulator ledger, not a constant. Uniform-field draws need their declared strict-sampling cutoff/loss when strict time is claimed. Eq. (228)'s matrix reduction uses this **full** extractor/emulator time; a lattice estimate does not certify an adversary with an unpriced runtime.

### 7.2 Fixed-input chain accounting is not the flat-input counterexample

In [CHAINED_TRANSCRIPT §§4–7](AKITA_INSTANCE_ACCUMULATION_CHAINED_TRANSCRIPT.md), direct component queries comprise:

- rooted absorb/finalize queries, which cost component work but need not request a native stream;
- rooted final-state squeeze queries with valid native site/key correspondence, which map to native ideal-stream keys;
- repeated blocks of one key, which may consume new bits/work but not a new key;
- malformed, unrooted, wrong-counter, out-of-range and separately simulated irrelevant-domain queries, which get consistent private random answers and do **not** automatically map to native keys.

If Q_comp is the entire component-ROM producer count, native adversarial keys are at most the rooted native squeeze keys, hence at most Q_comp. This is an upper bound, not an equality. For each mapped key k, requesting block j requires at most `lambda_A*(j_max(k)+1)` generated bits in the sequential-stream model; querying a high index is not free CPU. Verifier queries enter the native ceiling under its actual counting convention.

The chain-simulation loss additionally depends on **all** relevant component state interactions and the final high-level verifier expansions. Using that note's notation,

`N=Q_comp + sum_(distinct high-level P)(floor(|P|/b)+1) + B_H`,

`G<=Q_comp`, `d0=2^lambda_A-(N+G+1)`,

`eps_chain <= min(1,[N(N+1)/2+2*N*G]/d0)` when d0>0.

Shared prefixes and caching may reduce actual N; its profile must prove those reductions. Ignored low-level queries can still enlarge component work and collision/guessing accounting, even when they add no native stream key.

The earlier [EXTRACTION_BUDGET](AKITA_INSTANCE_ACCUMULATION_EXTRACTION_BUDGET.md) injectively encodes 140 E coordinates into one large flat oracle input. That **exact** attack on a forwarding budget does not transfer to a much smaller fixed component input. Nor does an ignored malformed squeeze imply a new native key: the simulator handles it privately. In the chain model, a relevant-key amplification example must construct bounded canonical rooted paths, finalization and valid native `(prefix,site)` keys whose eta images are distinct, count all component queries, and account for collision/guessing failure. Distinct external PCS-tree branches alone do not prove these conditions.

Likewise, no universal lower bound `native_keys>=K_PCS` follows from (1). A Theta-independent prover can issue no adversarial inner queries throughout extraction. Conversely, accepting only a small original per-run query budget gives no automatic bound on all repeated continuations. The required admission is the actual worst-case derived native-key count (or a sound cutoff/coupling at that count), not either unsupported extreme.

## 8. What remains unknown for the actual outer receipt

The named notes explicitly distinguish the inner archive profile from the unselected outer guest/schedule. Static inspection here confirms current facilities, not their instantiation:

- [preprocessing.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/preprocessing.rs), 90–143 and 254–270, derives grouped shapes from actual advice capacities, program/trace geometry and optional dense groups. Raising a byte capacity changes physical arities and grouped schedule inputs.
- [shape_guard.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/shape_guard.rs), 53–103, derives per-group layouts and resolves/validates a trusted schedule row. It does not supply a new receipt row from the inner profile. Its inspected constraint is `Field=AkitaField, ExtField=AkitaField`, not evidence of the requested K-source/E-challenge frontend.
- [RECEIPT_SETUP §8](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SETUP.md) and [OUTER_PROJECTION §6](AKITA_INSTANCE_ACCUMULATION_OUTER_PROJECTION.md) expressly leave the bounded guest, outer schedule and full compiler certificates uninstantiated.

The missing numeric/source inputs to (1)–(11) are:

1. The exact receipt guest and canonical archive/static/graph/quotient representations, physical source capacities and all extra retained original handles.
2. Actual compiled trace/RAM/program/feature geometry and all frontend-to-PCS occurrences, including any raw-source/quotient point normalization and setup reference.
3. Every nonterminal/terminal source shape, source block and claim count, padding rule, response/opening digit depth and carried witness width determining mu'.
4. Quotient/quotient-free route, ring dimensions, actual scalarized relation-row counts, range bases/stages/frontiers, norm routes and scalar-stage partition.
5. Tensor dimensions/tail rounds and both tensor and internal claim batches; setup-offload domains and receiver closure; all retained application batching stages.
6. Admitted conditional samplers, actual challenge domains, field/stream conventions and original grinding/query ceilings.
7. The original-source/full-grid projection and cross-continuation comparison classes at their actual mixed norms; fixed-source frontend soundness and complete challenge-key binding.
8. A concrete reset/replay/probe implementation or explicit cost bounds, including cheap rejections, private ideal-stream bits, dense extraction, source comparisons, projection and all Theta predicates.
9. Strict derived cutoffs and their failure probabilities, the component-to-native-key certificate, and full globally emulated SIS runtime at that cutoff.

No inspected finite profile justifies **derived Q32** for this proposed receipt. The inner finiteT16 profile certifies an archive replay deployment under its own stated ceiling; it explicitly excludes an additional receipt. A finite outer schedule family could in principle supply maximum geometry over its members, but finiteness by itself supplies neither a small tree nor a strict actual derived-key bound. Numerical illustrations of chain capacity or output width also do not select a PCS schedule or price rejected source probes.

## 9. Obstruction/admission boundary

A source-first route is admissible only after its original-output/freshness/source-comparison theorem and the complete selected schedule make the following chain quantitative:

`declared mixed geometry -> actual PCS extraction/probe work and calls`
` -> strict whole-producer cutoff with charged tail`
` -> persistent ARO emulation plus chain simulation`
` -> actual mapped native stream-key ceiling and bounded SIS time`.

The native tree lower bounds explain why “only the PCS suffix remains” is not a concrete resource certificate. The weighted probe refinement explains why a crude extra Q_o multiplier on every expensive suffix call can be avoidable. Neither fact alone proves or refutes receipt security at the unchanged Q32 inner ceiling. A profile may pass, or a changed proof may pass, but the current evidence does not establish one.

All conclusions are derived from the cited source equations and explicit conditional cost proofs. No source code, native parameter or other note was changed; no build, test, formatter, benchmark, proof-system execution or concrete hash experiment was performed. This document itself is the requested research deliverable.
