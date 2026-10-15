# Hidden-curve transport for the paired input-local ARO

## 1. Result and boundary

**The explicit-observation route works, with a bounded one-probe reduction and a deliberately weaker query loss than the published zero-finding bound.** The proof below fixes a candidate source using an independent private continuation, records its entire curve restriction before the original fresh challenge, and pays for changing the emulator's observation history by a real-ARO sandwich. It never assumes that an implicitly determined value persists when a sparse Boolean graph changes.

The invariant is the original public accumulator and its original weak `C_g,C_t` handles, including their full source shapes and application ownership. The original real-ARO output marginal is unchanged by the analysis experiment. An alternative source is only a comparison witness, never a replacement output or a canonical recommitment.

For the notation defined below, the resulting bound is

\[
\boxed{\quad
\alpha_R\le \min\!\left\{1,\ e(v)+Q\sqrt{\rho+\varepsilon_{\rm RB}(T_{\rm red})+
\eta_{\rm samp}+e(v)+2e(v+H)}\right\},\qquad
 e(u)=\min\{1,u/2^\lambda\}.
\quad} \tag{1}
\]

Here `H=D_f+1`, `rho=min(1,D_* kappa/2^lambda) <= D_*/|F|` for the specified sampler, `v` includes the complete terminal false-polynomial test, and `Q` counts all first underlying RO head activations, including WO-triggered ones and the appended output-CM lookup. The reduction uses **one** private probe and at most `H` hidden main-oracle observations. Its `Q` outside the square root is intentional; this note does not recover the earlier claimed square-root-in-Q bound. There is no rejection-probing loop or uncharged expected number of source probes.

This is a parametric theorem under the repaired input-local emulator and certified weak-source comparisons. It is not numerical admission of the quotient receipt, an unconditional security claim for the unchanged outer API, an extraction theorem, a claim for arbitrary cross-input witness circuits, or a concrete-hash result. Existing quotient formulae cannot be retained merely because their kernel is unchanged.

**Independent review:** TransportProofReview approved the complete weaker theorem and its strict resource ledger with no findings. It checked the two Q factors, original joint marginal, real-oracle sandwich, augmented-BAD transport, h'=0 case, source comparisons and sampling cutoff. This is conditional mathematical approval, not selected-profile or concrete-oracle admission.

## 2. Sources and exactly what is used

The assigned [transport contract](SOURCE_PROVENANCE.md#historical-session-evidence) is answered by the bounded two-fork option, not its possible exact-index rejection option.

Primary statements inspected:

- Chen–Chiesa–Gur–O'Connor–Spooner, *Proof-Carrying Data From Arithmetized Random Oracles*, accepted 24 April 2023 [official PDF](https://wrap.warwick.ac.uk/id/eprint/176570/1/WRAP-proof-carrying-data-arithmetized-random-oracles-Spooner-2023.pdf), [local PDF](SOURCE_PROVENANCE.md#supplemental-primary-sources), [local text](SOURCE_PROVENANCE.md#supplemental-primary-sources): Definition 4.1, lines 1260–1278; Lemma 7.1 and its proof, lines 2143–2394. The published proof's passage from prefix partial curves to later fork values is **not** used as a transport lemma here.
- [Akita primary text](SOURCE_PROVENANCE.md#akita), Definitions 10.13–10.14, Lemma 10.15 and Corollary 10.17, lines 3709–3824: noncanonical algebraic decommitments, bounded unit products, full commitment-grid semantics and cross-occurrence A/B/F comparison. These statements do not make decoded sources perfectly unique.
- [ARO witness scope](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md), §§4–7 supplies the paired input-local theorem; §8 supplies the counterexample to implicit persistence. The independent `AroRepairReview` confirmed the narrow theorem and the permissibility of private local simulations, provided actual hidden and terminal observations share the main persistent state and are counted. That review does not itself certify (1).
- [Quotient](AKITA_INSTANCE_ACCUMULATION_ORACLE_QUOTIENT.md), §§2–4 and 6.1, and [Quotient MLE](AKITA_INSTANCE_ACCUMULATION_QUOTIENT_MLE.md), §§2,4,6, supply the source grammar, raw sampler and actual full-capacity degree. [Source queries](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md), §6, gives the mixed-profile comparison algebra.

## 3. Complete theorem contract

### 3.1 Oracle and setup

Fix before oracle sampling an oracle-independent joint setup, admitted comparison inventory, finite field `F`, arities `m,lambda,w`, individual degree `d>=2`, and witness circuit

`wo(x)=b(x,mu_x,ro(x))`.

The independent persistent `mu_x` and input-local dependence are mandatory. Let `a=m+lambda+w`. A real ARO samples its full Boolean graph and then one uniformly random individual-degree-at-most-d extension `Vhat`. Oracle-dependent initialization, if present, is part of the counted computation, not auxiliary advice supplied for free.

The paired emulator retains `S:x -> (ro(x),wo(x))`, exposed evaluation map `T`, and the exact sticky-failure/totalization state. On a new head activation through either interface it obtains both retained halves; in its RO-only form it samples the head's witness coins locally. Its feasible affine space is

`A(S,T)={P : individual degree <=d, P|_{Boolean cube}=v_S, P|_{dom(T)}=T}`.

Every main evaluation uses this same persistent state. The independently proved theorem gives full-output statistical distance at most `e(v)` for a complete computation with at most `v` actual extended-oracle queries. On its conceptual first-BAD monitor, the probability of any BAD is also at most `e(v)`. Outside BAD all these affine spaces remain feasible, including after head activation. The monitor is for the proof; it does not condition the experiment or alter answers.

### 3.2 Sources, ownership and full degrees

A strict bounded producer returns the **original** `(acc,X,C_g,C_t,omega_g,omega_t)` or failure/collision. An oracle-free source-validity relation checks the original handles, profile and certified weak source data. Its deterministic projection fixes `X,n,D,g,t`, where `D={b_0,...,b_(n-1)}` consists of distinct public field elements,

\[
\deg g_i\le n-1,\quad D_f=ad(n-1),\quad
\deg t\le N_t-1,\quad D_*=\max\{D_f,n+N_t-1\}<|F|.
\]

Every coefficient in the actual `4N_t` decoded coefficient grid contributes to `t`. All physical source cells, admitted padding and actual native shape bounds remain present. If the backend's actual capacity is larger, use that capacity. No small representative of `delta^{-1}r` is required. No canonical Commit coins are postulated for extracted sources.

The main theorem concerns `n>=2`; the `n=1,t=0` direct graph protocol remains separate. For multiple admitted profiles, take public upper bounds for `Q,v,D_f,D_*` and comparison work; a smaller profile's interpolation can use its own degree. Padding the accounting does not shrink its physical source relation.

The raw head is the injective m-bit encoding

`cm=Enc_m(tag,setup/profile identities,original C_g,original C_t,source/decoder/domain shapes,nonce)`.

All identities must have their declared injective registry meaning and the tuple must fit m bits. The projection through `C_g` binds the full application statement `X`; a canonical `C_g` that omits `X` does not meet this contract. The two compared witnesses must project to the same source interpretation, or their exact mixed comparison must be inventoried. Equal `cm` implies the same handles, shapes and nonce, not equal witnesses. Postchallenge data must not be made a mandatory component of the prechallenge `C_g` source.

### 3.3 Normalized complete experiment and event

Normalize the bounded producer by appending a tester. It preserves the producer's exact returned `acc` and witnesses. It validates sources and ownership; explicitly obtains the actual pair `(ro(cm),wo(cm))`; checks that the accumulator's first Boolean graph point contains precisely `bits(cm)||bits(raw)||bits(z_cm)`; applies the declared sampler; checks its first graph equation and the equation

`Vhat(g(beta)) = 1+Z_D(beta)t(beta)`.

It also checks

\[
Vhat\circ g\not\equiv 1+Z_Dt. \tag{2}
\]

For (2), query the actual oracle at `g(xi_0),...,g(xi_Df)` for fixed distinct scalar points, interpolate the degree-at-most-`D_f` curve polynomial, and compare coefficients against the full `1+Z_Dt` of degree at most `D_*`. This uses `D_f+1`, not `D_*+1`, oracle evaluations; `D_*+1` is a safe more expensive choice. Scalar points must be distinct; their curve images may repeat. A degree-D_f composition is still determined by these scalar evaluations.

The first retained lookup and first graph check happen **before** the terminal curve-evaluation phase. After the final required retained lookup, the tester makes only extended evaluations and oracle-free computations. Thus the terminal evaluation phase has one fixed Boolean restriction. In the real world it checks the actual returned false-polynomial predicate, not a polynomial from an unrelated fresh kernel. Count all producer/internal-extractor calls and all tester calls. A conservative total is

`v = v_producer + (D_f+1) + 2`,

where the two terms are the two external graph/evaluation equations. Additional checks increase `v`. Repeated calls may be counted again safely.

Let `Q>=1` bound distinct underlying RO head activations in this normalized experiment. The order includes activations through WO and the output lookup. Index `I` is the first activation of the **returned** `cm`, even if that occurred much earlier or first through WO. The snapshot at index `i` is taken before its new raw answer, new witness coins and requested interface response. No assumption that the adversary first explicitly called RO at cm is allowed. If the tester activates a previously unqueried output cm, it receives an index in this same bound. A guessed raw output is tested against this actual lookup; it is not excluded by an unproved query assumption.

`Win` is source validity, the exact original-acc checks, sampler acceptance, the equation at beta, and (2). Let its real probability be `alpha_R` and fully emulated probability be `alpha_E`. The complete-output emulator theorem gives

\[
\alpha_R\le\alpha_E+e(v). \tag{3}
\]

Other producer failure/collision modes simply do not win this game. An application must separately account for its own extraction failure and collision outputs when reducing receipt soundness to this game.

### 3.4 Exact raw sampler

For the proposed field, let `p=2^128-2^32+22537`, `K=F_p`, `F=K[U,V]/(U^2-5,V^2-U)`, and `M=|F|=p^4`. The native fieldhood premise and the tower argument in the quotient note are retained. Set `lambda=1024`, `R=2^1024`, `kappa=floor(R/M)`, `cut=kappa M`. Reject raw integer `r>=cut`; otherwise decode `r mod M` in the ordered basis `(1,U,V,UV)` using base-p digits. Also reject beta in `D union E_ell`, where `N_t=2^ell` and `E_ell={beta: product_(j<ell)(1+beta^(2^j))=0}`.

For any fixed set `Z subset F`, independently of acceptance elsewhere,

\[
\Pr[S(r)\in Z\text{ and accepted}]
=|Z\setminus(D\cup E_\ell)|\,\kappa/R
\le |Z|/M. \tag{4}
\]

The one-draw rejection probability is less than `2^-512+|D union E_ell|/M`, with `|E_ell|<=31`. Neither rejection nor exceptional MLE roots can win. This proof uses only (4); it never conditions a challenge on sampler acceptance or probe success. General balanced raw samplers may replace (4) by their actual unconditional maximum atom.

## 4. One private continuation and three joint games

Choose `J` uniformly in `{1,...,Q}`, independently at the start. The producer cannot inspect J. If fewer than J activations occur, no probe is made. At the J-th main pre-activation snapshot `sigma`, with pending head `x_J`, do the following once:

1. Copy the complete producer state, consumed randomness state, already sampled auxiliary-oracle tables, paired S, evaluation record T, head-index history, and sticky failure state. A fixed full producer random tape may instead be included in the prefix; importantly **future emulator and new-head coins must not be shared**.
2. Run one private continuation to its ordinary normalized termination. Its fresh ordinary-RO entries are independent local draws, preserving only the prefix dictionary; its future witness/kernel coins and any not-yet-fixed producer/auxiliary randomness are independently sampled from the original continuation law. This is not an oracle call to the actual future and not a reset of the main persistent emulator. Existing prefix values/coins stay fixed. The private kernel has its own cloned state.
3. Call the result a `ProbeHit` if it returns a source-valid output with `I'=J` and `cm'=x_J`. It need not be a zero-finding winner. Retain its entire weak source certificates and projected `(g',t')`. Otherwise retain a dummy.
4. Resume the main computation, not the private one. All private outputs and coin draws are hidden from the main producer.

There is no recursive probing inside the probe, no exact-index rejection loop and no requirement that the two raw answers differ. A coincident answer is allowed and covered by the root bound.

Define games on the joint output `(J,probe record,original output,original Win bit)`:

- `E0`: the paired-emulated main computation with this private probe, but no hidden curve queries.
- `R0`: the corresponding real-ARO main computation. Its explicit retained-interface wrapper records both halves at each activation and all main evaluation answers, so the same private sparse simulator is computable from the available prefix. It never requires the real dense polynomial or hidden witness coins of an already paired head.
- `EH`: after a `ProbeHit`, before answering the pending original raw query, insert the `H=D_f+1` actual main-emulator queries `g'(xi_j)`. Record them in the main persistent T, interpolate `p'(X)` from their answers, and keep this polynomial hidden from the main producer. Then resume normally. If no hit, insert nothing.

Define `RH` identically with the real ARO in place of the main emulator. **The joint laws of R0 and RH are equal.** Couple them with one fixed actual ARO, the same main randomness and the same private probe. Their prefixes and private probes agree. Extra queries only observe that already fixed polynomial; their answers are not used in the main producer or terminal Win computation. They cannot change its output, its future genuine oracle answers or the stored probe. This is exact marginal preservation in the real world, not in the sparse emulator.

The paired theorem applies to each entire wrapper, private simulations included as local computation. A deterministic shadow of the sparse state in a real wrapper can record returned answers and reproduce the exact failure convention for the private clone; after an uncoupled rare prefix it is merely a total algorithm, and no conditional-on-prefix emulation promise is needed. Therefore

\[
\operatorname{TV}(E0,EH)\le e(v)+e(v+H). \tag{5}
\]

Virtual private-kernel evaluations do not enter either actual-oracle support. They do enter runtime and sampling counts. Actual hidden evaluations in EH do enter main T and its BAD support. This distinction is essential.

## 5. Correlation lower bound with the original-output marginal

E0's main computation has exactly the original fully emulated marginal: the private probe has no effect on it. For each i let `mu_i` be the subprobability law of the original main prefix just before fresh activation i, including setup and any fixed producer tape. Its mass is at most one. For a prefix sigma define

- `a_i(sigma)=Pr[Win and I=i | sigma]`;
- `p_i(sigma)=Pr[source-valid output and I=i | sigma]`.

The pending input fixes cm on the indexed event. In particular `p_i>=a_i`, and `alpha_E=sum_i integral a_i dmu_i`. Independent private and original future coins give, for

`C = {original Win, original I=J, ProbeHit}`,

\[
\begin{aligned}
\Pr_{E0}[C]
&=\frac1Q\sum_i\int a_i p_i\,d\mu_i\\
&\ge\frac1Q\sum_i\int a_i^2\,d\mu_i
\ge\frac{\alpha_E^2}{Q^2}. \tag{6}
\end{aligned}
\]

The last inequality is Cauchy–Schwarz on the disjoint union of prefix spaces, whose total measure is at most Q. Zero-mass or zero-success prefixes contribute zero; no conditioning on a nonzero success probability is required. The two factors of Q have distinct origins: the sum of prefix masses, and guessing one online index. Omitting either would claim a stronger, unproved theorem.

## 6. Explicit observations, source comparison and the root event

### 6.1 Source comparison is computational, not perfect uniqueness

On C, both branches have valid weak sources for exactly the same `cm`, hence the same original handles and frozen interpretations. If their source projections differ, compare the full sources and original paths for each of `C_g,C_t`. A difference in certified original path digits produces the registered B/F collision. Otherwise the A images agree. With certified units and bounded products `r=delta s`, `r'=delta' s'`, a differing source block gives

\[
v_A=\delta'r-\delta r'=\delta\delta'(s-s')\ne0,\qquad Av_A=0,
\]

\[
\|v_A\|_\infty\le K'_1 B_\infty+K_1B'_\infty. \tag{7}
\]

Use the corresponding admitted mixed Euclidean envelope if that is the selected route. Unit inverses need no shortness bound. The difference may occur in any full-grid cell. Matrices, dimensions, slices and alphabet diameters are the actual cross-profile ones. There is no replacement by two honest short canonical encodings, no independence assumption between shared setup views, and no D/H equality requirement across the two different opening instances.

A bounded ordinary collision producer executes EH by lazily simulating all of its RO tables internally and running both paired kernels. When original/probe valid sources differ semantically it performs this comparison, outputting a registered collision. Let `epsilon_RB(T_red)` bound its strict runtime-T_red success under the declared joint setup/inventory. It may be written as the appropriate sum of ordinary SIS/MSIS advantages and an explicitly charged setup distance. If such a comparison class is not admitted, (1) is conditional on that missing certificate; the proof does not manufacture it.

### 6.2 Hidden observations really persist

In EH, the probe source, its curve, its t and the hidden answers all precede the original fresh raw answer. Since the private branch uses independent future values, none is a read of the unqueried actual `ro(x_J)`. The hidden main kernel uses only the main paired history and its own fresh randomness and makes no retained-oracle query. Consequently, conditional on this entire augmented past, the original raw answer is still uniform on `{0,1}^lambda`. This statement is made in the efficient emulator, **not** after conditioning on no BAD or in the true-ARO prefix law.

Let `Bad_H` be the paired emulator's conceptual first-BAD event for the main support, including producer calls, hidden H calls and all terminal tester calls. Then

\[
\Pr_{EH}[Bad_H]\le e(v+H). \tag{8}
\]

Outside this event every later affine space is feasible and preserves the H explicitly recorded observations. Every degree-d extension composed with `g'` has degree at most D_f. Thus every later feasible extension has restriction exactly `p'` on that curve. This remains true when S grows. It does **not** assert nested global affine spaces or a single dense polynomial common to all stages. A newly activated graph point that would contradict these observations is charged to Bad_H by the shift/first-entry argument with the augmented support.

Define the prechallenge polynomial

\[
h'(X)=p'(X)-(1+Z_D(X)t'(X)),\qquad \deg h'\le D_*. \tag{9}
\]

On C with equal projected sources and outside Bad_H, the main curve and t equal the probe's. The equation at beta gives `h'(beta)=0`. The coupled terminal test (2), whose actual queries use this same persistent main state, gives `h' != 0`. In particular **h'=0 is not discarded**: if the original terminal false-polynomial bit nevertheless holds, either source equality failed or Bad_H occurred. Repeated curve images do not change this interpolation argument.

For every augmented prechallenge history, the event `h'!=0` is already determined and its accepted root mass is at most rho by (4). Uniformity is used without conditioning on C, source equality, future validity or Bad_H. Hence

\[
\Pr_{EH}[C]\le \rho+\varepsilon_{\rm RB}(T_{\rm red})+
\eta_{\rm samp}+e(v+H). \tag{10}
\]

The cutoff term concerns the strict implementation of the otherwise ideal-sampling collision producer; its exact construction is below. Equations (5), (6) and (10) imply

\[
\alpha_E^2/Q^2\le \rho+\varepsilon_{\rm RB}(T_{\rm red})+
\eta_{\rm samp}+e(v)+2e(v+H).
\]

Combining with (3) proves (1). No term from CCGOS Lemma 7.1, no same-kernel transport claim and no hidden acceptance-conditioning step is used. There is no separate `1/|F|` guess term or `Q/2^lambda` missing-query term: the actual evaluation and output raw lookup were appended and fully included in the game and its bounds.

## 7. Strict runtime, sampling and complete work ledger

Let `T_0` bound the normalized producer plus terminal tester, excluding paired-kernel overhead, but including all its own extraction/fork calls, source validation, ordinary auxiliary-oracle work and source decoding. Let `h<=Q` bound main activated heads, `r_B` be witness-coin length and `T_B` the bit cost of input-local B. Let `c_F` bound a field operation, and `C_CD(a,d,j)` the actual deterministic constraint-detector work on j points. Define a conservative emulator work function

\[
\begin{aligned}
\mathcal W(h,u)={}&O(h(T_B+r_B))+O(c_F a h u^2)\\
&+\sum_{j=1}^{u}O\!\left(a C_{CD}(a,d,j)+a C_{CD}(a,d,j+1)
+c_F(aj+1)^3\right), \tag{11}
\end{aligned}
\]

plus serialization, dictionary costs and bounded random generation. This includes affine consistency tests, not just sampling from presumed nonempty spaces. Existing prefix work is not rerun for the private continuation, but upper-bounding its kernel work by a full `W(Q,v)` is safe.

| Resource | Bound and ownership |
|---|---|
| Main producer/tester executions | One, at most T_0 intrinsic work |
| Private source probes | At most one continuation; at most T_0 intrinsic work; no recursive probe |
| Main underlying fresh RO heads | At most Q, including WO pairing and appended cm/predicate heads |
| Private new RO heads | At most Q additional locally sampled entries, not actual-oracle requests |
| Real retained interfaces in a comparison wrapper | At most Q RO and Q WO calls; both halves cached per activated head |
| Actual main Vhat calls in E0/R0 | At most v |
| Actual main Vhat calls in EH/RH | At most v+H |
| Virtual private Vhat/kernel calls | At most v; full cloned prefix support retained; CPU and random sampling charged |
| Hidden curve evaluations | At most H=D_f+1, only at the one guessed index |
| Terminal false-polynomial observations | D_f+1 in each completed main/private normalized execution; included in v |
| Source comparisons | At most one pair for C_g and one pair for C_t, each across complete actual shapes |
| Source witness/acc storage | Both original and private weak witnesses, original acc, snapshot and full source tables |

Let `T_copy` bound copying the full producer snapshot, dictionaries and paired record, not just an adversary-visible transcript; let `T_int` bound evaluating the hidden curve at H scalar points and interpolating its restriction; and let `T_cmp` bound both full-grid comparisons and collision construction. The strict ordinary reduction may use

\[
T_{\rm red}\le 2T_0+\mathcal W(Q,v+H)+\mathcal W(Q,v)
+T_{\rm copy}+T_{\rm int}+T_{\rm cmp}+T_{\rm bounded\ randomness}. \tag{12}
\]

Account for maximum source sizes and all registered matrix views in these quantities. The two source relations are not evaluated on merely natural/live coefficient prefixes. If the producer is itself a capped extractor, its cap and all its internal calls are part of T_0,Q,v, and its preexisting cutoff failure belongs to the application reduction separately.

For a concrete strict coin implementation, the kernels can sample an affine solution by at most `aj+1` independent field elements at their j-th evaluation (with unused draws omitted). A conservative bound across both computations is

\[
N_F\le (v+H)(a(v+H)+1)+v(av+1), \tag{13}
\]

provided the chosen constraint detector is deterministic. Add explicitly any other reduction-owned field draws or random subroutines to N_F; producer-owned sampling is already in its strict T_0 contract. Each uniform field draw is sampled by at most L trials of `ceil(log_2|F|)` bits, rejecting integers outside the field's canonical range. Its trial acceptance probability exceeds 1/2, so cutoff probability is at most `2^-L`. The same bounded rejection method samples J in `[Q]`. Fixed-length RO outputs and witness bit coins need no rejection. Thus

\[
\eta_{\rm samp}\le (N_F+1)2^{-L}. \tag{14}
\]

On exhaustion return failure, never substitute a biased field element. Couple the ideal and strict collision producers with the same candidate bit strings; their complete outputs agree unless a cutoff occurs. This proves that ideal collision probability is at most `epsilon_RB(T_red)+eta_samp`, as used in (10). The extra bit work is `O(L(N_F ceil(log_2|F|)+ceil(log_2 Q)))` plus its declared bookkeeping and all ordinary-oracle/witness coins. Set T_red to include it. No expected-time assertion is substituted for this strict SIS runtime.

The main paired state uses `O(Q(m+lambda+w+r_B)+(v+H)(a+1)ceil(log_2|F|))` bits, plus the kernel's `O((a(v+H)+1)^2)` field-element workspace and detector workspace. The private clone has the corresponding v-support bound, plus the complete producer and source snapshot. These are proof-resource bounds, not honest proving cost or benchmark claims.

Only one online index is probed, so there is no uncharged `QH` hidden-query cost. If a different proof attaches hypothetical sources to every target index, it must instead charge their total hidden observations and their probing work. Formula (6) explicitly pays for the single-index choice.

## 8. Admission consequence and remaining external premises

The missing transport step is supplied by explicit observations plus (5), not by fixed-state partial-oracle uniformity alone. The §8 nonpersistent-curve counterexample from the witness-scope note does not refute this proof: the vulnerable Boolean point enters the **augmented** support when its entire candidate curve restriction is recorded, and a contradictory future activation is covered by (8).

The following remain application premises, not conclusions of this note:

1. The full outer producer actually returns valid same-original-acc source witnesses under its declared strict cutoff and actual-fixed-Theta experiment; its independent extraction/projection proof must supply that statement.
2. The selected witness circuit is input-local and its initialization is fully accounted.
3. The source grammar binds X, complete g data including graph witnesses, the full t capacity, original shapes and nonce; m really accommodates the injective cm encoding.
4. Every required same-handle weak-source comparison is admitted at its actual A/B/F dimensions, unit-product bounds, certified alphabets and joint setup law.
5. Any numerical security choice uses the **weaker bound (1)**, its complete Q,v,T_red and strict cutoff ledger. The previous quotient bound is not recovered by this result.

No implementation, build, test, formatter, benchmark, concrete-oracle admission or honest-cost conclusion was performed or implied. The deliverable is the complete conditional zero-finding theorem and reduction above; parent integration must decide whether its quantitative loss and external source/profile premises are sufficient for the intended receipt.
