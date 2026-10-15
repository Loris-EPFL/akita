# Compact CM inputs through an externally recomputed transcript root

## 1. Result and invariant

**Result: the proposed extension has a conditional classical ideal-oracle compiler theorem.** The missing source-identity step is supplied by an early-rooted-label/late-hit lemma for the **complete two-branch ordinary-RO execution after paired ARO emulation**. Its additional error enters **inside** the square root of the independently reviewed transport bound. It does not replace that bound by the stronger published square-root-in-query law.

The invariant is the exact original accumulator, application statement, original lattice handles `C_g,C_t`, all-valid weak sources, and complete source interpretations. The compiler changes the encoding of the CM head and adds an external correspondence check; it does not replace an output by a canonical recommitment. The underlying native pre-PCS accumulation, T16, 33 inner roles, and single complete native decider are unchanged. No CH-I/CH-O assumption, computational hash commitment, concrete hash realization, new PCS, or homomorphic A/B/F operation is introduced.

The literal-injective-CM theorem in [ZERO_FINDING_TRANSPORT](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md), §3.2, **does not apply unchanged**. A short finalized state is not an injective encoding of arbitrary full tuples. This note proves the replacement implication, with its own bad event and costs. Existing literal-CM profiles remain valid alternatives; this extension does not silently change their grammar.

The honest compiler is straightforward. The nontrivial point is adversarial: an attacker may first request `wo(cm)` on a guessed state, and only later select the handles and compute a root. Further, the two transport continuations use distinct suffix tables. A same-function collision-resistance slogan cannot establish their source identity. Sections 4–6 address precisely those issues.

**Independent review:** CompactCmReview approved the conditional compiler with no findings. It checked WO-first/late-root handling, distinct suffix-table variables, global first-bad counting, same-original-handle recovery, placement of the root error inside the square root, noncircular external RootValid and complete resource accounting. This does not select this grammar in an existing profile or admit a concrete oracle/guest.

## 2. Explicit grammar and dependency order

### 2.1 Parameters and full message

Fix all profiles, codecs, setup and bounds independently of oracle sampling. Retain [CHAINED_TRANSCRIPT](AKITA_INSTANCE_ACCUMULATION_CHAINED_TRANSCRIPT.md), §§2–3:

- component output and chain state width `lambda`;
- payload block width `b` bytes, `b>=1`;
- absorb counter width `k`, site width `s`, block-index width `j_w`;
- `ell_b=ceil(log2(b+1))`, root `IV=0^lambda`;
- canonical typed records `Rec(type,payload)=u32be(type)||u64be(len(payload))||payload`;
- bounded byte lengths and the same mandatory finalization/counter-overflow rules.

Use a disjoint registered **full-message** CM context/version record, not merely a different output tag. Define the uniquely parsed full bytes

`M=Enc(CM-context/version,setup/profile identities,C_g,C_t,source/decoder/domain shapes,nonce)`.

Here `shapes` includes every interpretation needed by the source theorem: actual capacities, coefficient order, domain and ledger-length identities, original handle type/shape metadata, and the selected permitted mixed-comparison class when needed. Registry identities have their declared injective meanings; an unproved digest of arbitrary profile data is not a substitute. `C_g`'s source projection continues to bind the full application statement `X`. If that projection does not bind X, explicitly including X in M cannot repair the missing source/guest linkage by itself.

The full record encoding is injective **as bytes** and is bounded by `L_CM`; it is not required to fit in one component input. Native transcript bytes start in their own context namespace. Require `floor(L_CM/b)<=2^k-1` and the corresponding bound for every native lifecycle. All lengths, integer codecs, zero padding, and registry values are fixed profile data.

### 2.2 Component inputs

Choose one uniform random function `C:{0,1}^m -> {0,1}^lambda`, where

`m=max(2+lambda+k+1+ell_b+8b, 2+lambda+s+j_w)`.

Pad the following inputs on the right with zeros to exactly m bits:

```
A(state,counter,final,length,block)
  = 00 || state[lambda] || counter[k] || final[1]
       || length[ell_b] || block[8b] || zero_pad

S_nat(state,site,index)
  = 01 || state[lambda] || site[s] || index[j_w] || zero_pad

S_CM(state)
  = 10 || state[lambda] || CM_SITE[s] || CM_INDEX[j_w] || zero_pad
```

`CM_SITE` and `CM_INDEX` are fixed public constants, for example both zero. There is only one accepted CM squeeze slot per finalized full M. Different honest nonce attempts change M, not this slot. Tag `11`, noncanonical padding, nonfixed CM fields, and malformed inputs are invalid **protocol operations**, but remain ordinary component-oracle inputs with independent cached random answers. Adversarial queries are never suppressed or assumed well formed.

For `M=B_0||...||B_(h-1)||tail`, with `|tail|<b`, compute

```
s_0 = IV
s_(i+1) = C(A(s_i,i,0,b,B_i))                  for 0<=i<h
s_fin = C(A(s_h,h,1,len(tail),tail||zero_pad))
cm = S_CM(s_fin)
raw = C(cm)
```

There is one final absorb for an empty tail as well. A final node has no legitimate absorb successor. Native tag-01 squeezes retain their original stream identities, consumption order, rejected attempts and bounded-abort behavior. This is a new formal transcript grammar, not a claim about the currently implemented transcript codec.

### 2.3 No `C_g` cycle; oracle-free versus external predicates

The construction order is:

1. Finish the unchanged native archive and all graph-defining prechallenge query answers/witnesses. Fix their source geometry and form `g`.
2. Compute the honest quotient and its full-capacity original coefficient source. Produce the original `C_g,C_t` under their provisioned original lattice maps.
3. Form public M from those handles, identities/shapes and a nonce. Compute the absorb/final root and `cm`; then obtain raw, `wo(cm)` and beta.
4. Form the public accumulator and quotient claim, and prove the ordinary outer relation with these exact public values.

**Do not add the CM-root absorb calls, root transcript data, root state, raw, `wo(cm)`, beta or beta-dependent values to the mandatory pre-beta source committed by `C_g`.** In particular the root calls are not appended to the graph ledger defining g. If they were, `C_g -> M -> root calls -> g -> C_g` would be a genuine circular commitment requirement. The host and external verifier evaluate those calls directly instead. They enter security query budgets even though they do not enter n.

Separate the predicates:

- `SourceValid` is the same oracle-free original-source relation and deterministic projection to `X,n,D,g,t`.
- The oracle-free guest binds the full public handle/shape tuple, claimed cm/raw/witness bits, exact accumulator and quotient claim to its accepting computation. It checks the fixed tag/site/index syntax and all prior skeleton obligations, but does **not** claim to derive the root with no oracle.
- `RootValid^C(M,cm)` canonically reconstructs M from the public tuple, evaluates all its actual A calls using C, constructs `S_CM(s_fin)`, and checks exact equality with the first accumulator head. This is an additional **external** acceptance condition.

The external verifier retains the complete ordinary outer proof, the original-handle equality checks, ordinary full-grid quotient opening, and both actual `Vhat` equations. It does not accept a supplied root trace or a root merely attested by an oracle-free guest. The first graph equation still authenticates all raw and witness bits at cm; an extra external `C(cm)` query is unnecessary for the actual receipt. The normalized security tester does make that lookup, as required below.

M need not be transmitted a second time if its fields are already in the public statement/proof envelope. Its complete handle and metadata bytes nevertheless remain actual public transport and decoding costs. Neither this compiler nor a short state makes the original lattice handles short.

## 3. Normalized game and first activation

Use the source, field, full-degree, balanced sampler and strict-cutoff contracts of ZERO_FINDING_TRANSPORT, §§3 and 7, with the CM clause replaced by §2 above. In particular

`a=m+lambda+w`, `D_f=ad(n-1)`, `H=D_f+1`,

`deg(t)<=N_t-1`, `D_*=max(D_f,n+N_t-1)<|F|`,

and the selected witness circuit is input-local: `wo(x)=b(x,mu_x,C(x))`. The entire actual source-producing/extracting computation, all initialization, both external root and graph checks, and the terminal false-polynomial test are inside the strict experiment. No oracle-dependent preprocessing is free.

Append a tester which preserves the producer's exact output. It validates `SourceValid` and public syntax, performs `RootValid`, and explicitly gets the actual retained pair at the root-checked cm before the graph checks and terminal curve observations. All retained lookups, including root absorb calls, precede the terminal phase that uses only extended evaluations and oracle-free work. A failed preliminary check cannot win. For probing, retain the source and preliminary validity flags even if the later zero-finding conditions fail.

Let Q bound all first underlying C-head activations in this normalized run. A first `WO(x)` activation fetches `C(x)` in the paired implementation, so it occupies the same index as a first RO activation. The pre-activation snapshot is **before** the raw C answer, new witness coins and requested response. I denotes the first activation of the eventually returned cm, not its last verification lookup and not its first explicitly named RO call. Append a previously unqueried output head into this same count. Root recomputation may use cached entries, including entries first activated by WO.

Use the repaired paired theorem on the **whole computation first**. Let v be its complete extended-query bound, including `H+2` tester observations when using the transport note's conservative convention. Then

`alpha_R <= alpha_E + e(v)`, where `e(u)=min(1,u/2^lambda)`.

Only after this transformation does the following root proof operate on ordinary independently lazy-sampled C tables. It does not replace C while leaving a genuine WO/extended oracle attached, and does not assume the original real-ARO prefix exposes a fresh independent raw answer.

In the one-probe transport construction, redefine `ProbeHit` to require **both** `SourceValid` and `RootValid`, an actual indexed retained lookup, and `I'=J,cm'=x_J`. It still need not be a zero-finding winner. This is computable by the private normalized continuation. Since every Win satisfies these conditions, `p_i>=a_i` still holds; the identical calculation gives

`Pr_E0[C_event] >= alpha_E^2/Q^2`,

where `C_event={original Win, original I=J, ProbeHit}`. Original output marginals remain unchanged.

## 4. Early rooted labels in a single ordinary-C execution

### 4.1 Online labels and a finite bad event

Maintain the exact input cache and online label map from CHAINED_TRANSCRIPT, §4: initially `R[IV]=Data(empty,0)`; a correctly counted, bounded canonical A edge from a labeled Data state produces a Data or Final full-byte label. Cache hits never overwrite their prior answer. An unrooted edge obtains no retroactive label. Store labels as parent/block pointers; do not copy every prefix string at every edge.

For this section every C request is explicit low-level computation. There are no hidden high-level chain expansions. Register the state field of every fresh, correctly padded typed A, tag-01 or tag-10 input, **including fields on semantically invalid/unrooted edges**. Register it before that request's output is sampled. Taking a larger set of syntactically located fields only makes the bound more conservative.

Stop at either of:

1. **Output/root coincidence:** a fresh C output equals IV or an earlier distinct fresh-output variable.
2. **Late-hit coincidence:** a fresh C output equals any previously registered state field, including the currently pending request's field.

Reusing the same cached C entry is not a fresh output variable. These events intentionally include harmless coincidences. There is no restriction on how the next input depends on earlier answers, kernel evaluations, auxiliary randomness or source computation.

### 4.2 Lemma: valid late computation implies an early label or bad

Outside the two events, every legitimate rooted A path, even if the algorithm evaluates its entries much later or in a different order, consists of entries that were first activated in root order and received their correct labels online.

**Proof.** Induct on the path from IV. Its first edge has a known Data parent at any first activation. For a later edge e let the preceding edge p produce its parent state. The two inputs are distinct: counters increase, and the final flag terminates the path. If e was activated before p, the state field of e was registered before the fresh output of p equaled it, a late hit. Thus p was activated earlier, and induction supplies its proper label at e's first activation. Fresh-output uniqueness and exclusion of IV ensure that installing e's label never aliases another output/root label. Fixed counters, block lengths, padding and finalization determine its unique successor label. This proves the assertion for the entire path. ∎

Now consider the **first** activation of an eventual successful CM head `x=S_CM(s)` and its pre-answer prefix. Its successful external root path ends with an A entry that outputs s. If that final A entry was not activated before x, its later output hits the state field registered at x, a late hit. Otherwise the preceding lemma says that, before x, s already carried its unique `Final(M)` label. Tag/canonicality and full-byte injectivity fix the entire message interpretation.

Therefore:

> At the first underlying C activation of the eventual root-checked CM head, either the common prefix already fixes one unique full message M at its finalized state, or a charged coincidence occurs.

This includes first activation through WO, a guessed head, an earlier unrooted absorb DAG, malformed/foreign-context prequeries, and later verification using old cached entries. No hidden assumption that the producer queried the root honestly before choosing cm is needed. A Data label cannot become a different Final label without one of the charged events.

## 5. Two branches: the actual coupling and bound

### 5.1 Do not merge the suffix functions

The transport experiment copies the complete pre-activation state at J. Its C table prefix is shared. The private continuation samples independent fresh suffix entries, returns its complete preliminary source, and stops. The main continuation then resumes with its own independent fresh entries; in EH it first inserts the hidden main-curve evaluations. Those evaluations use only the main paired record/kernel and fresh local randomness, not a retained-oracle request.

Model this execution as a chronological forest of output variables:

- a prefix-table entry is one random variable reused by both branches;
- a new private-table entry is a new random variable;
- a new main-table entry is a new random variable, **even if its input string also appeared in the private suffix**.

Thus `C_private(x)` and `C_main(x)` for a postfork x are not declared equal. This is exactly the private-continuation law, not one global random function with two descriptions. Each new variable is uniform on `R=2^lambda` values conditional on the entire previously executed augmented history. The producer/kernel may choose its input adaptively, but neither has read that new variable. Fixed-length raw draws remain independent even if the sparse kernel has entered its declared totalized failure state.

Run the two bad-event monitors globally on these **variables**, registering state fields in chronological order across both branches. Prefix cache copies are not resampled or charged twice. A coincidence between independent same-input suffix variables is charged as a random-variable coincidence, **not** called a same-oracle hash collision. Cross-branch field hits/coincidences are conservatively charged even when unnecessary for the branch-local induction.

Let N bound the total number of fresh variables in the forest and G the number of registered state fields. With at most Q main and Q private activations, `N<=2Q`, `G<=N`. At fresh draw r, at most r values cause an output/root coincidence (IV plus r-1 older outputs), and at most G registered values cause a late hit. The first-bad union bound gives

`eps_root(N,G)=min(1, [N(N+1)/2+NG]/2^lambda)`.

In particular the convenient whole-run bound is

`eps_root(Q)=min(1,(6Q^2+Q)/2^lambda)`.

No denominator correction is necessary here: each newly sampled low-level output is uniform **before** conditioning on a good history, and the argument bounds bad at its first occurrence. This is not the hidden-high-level-state conditional argument of CHAINED_TRANSCRIPT, §5.3. If high-level services are present, expand their actual component calls into this normalized experiment and N before using this formula; do not treat their internal state evaluations as absent.

### 5.2 Corollary: the same prefix fixes the same complete tuple

On `C_event` both branches return the pending head `x_J`. Apply §4.2 to each branch outside the global bad event. Both finalized states already have their unique full labels in the **shared pre-J prefix**, before either future is run. That prefix's input cache, labels, counters and bytes are identical. Hence

`M_main=M_private`.

Consequently the original `C_g,C_t`, all interpretation/profile/source-shape identities and nonce agree exactly. This is stronger than merely finding roots with numerically equal states in two arbitrary functions: their complete rooted paths were already present in the same prefix. If either branch only acquires the root afterward, that branch incurs a late hit. Distinct already rooted prefix messages would require a prefix output coincidence. There is no uncharged third cross-branch case.

The argument does **not** assert equal witnesses or perfect uniqueness of decoded weak sources. It only restores the exact same-original-handle premise for the ordinary lattice source comparison.

## 6. Conditional compact-CM transport theorem

Retain the real-ARO sandwich and hidden-curve insertion of ZERO_FINDING_TRANSPORT, §§4–6. Root checking is just additional retained-oracle computation in both wrappers; in the real ARO, hidden observations still cannot change either its fixed polynomial or C table. Therefore

`TV(E0,EH)<=e(v)+e(v+H)`

with the enlarged actual Q and complete v. Root checking itself adds no extended observations.

On `C_event` outside root bad, §5 gives equal full M. Compare the original weak sources under those same handles using the already certified all-valid-source comparison inventory. A different original path yields the registered B/F collision. With the same A image, a semantically different source block yields

`v_A=delta' r-delta r'=delta delta'(s-s') != 0`, `A v_A=0`,

`||v_A||_infty <= K'_1 B_infty + K_1 B'_infty`,

or the actual admitted mixed Euclidean envelope. Include every physical source cell, legal noncanonical digit, unit product and actual registered matrix view. No source is replaced by an honest short commitment encoding; inverse units need no shortness claim.

An ordinary bounded collision producer simulates both tables/kernels, compares the two full public messages first, and only invokes same-handle source comparison if they agree. If they differ it may return failure. Its collision success is bounded by `eps_RB(T_red)+eta_samp`; source differences on `C_event` with unequal messages are already covered by root bad, not charged to an inapplicable lattice comparison.

Outside root bad, source-comparison failure and main paired BAD, the probe curve/polynomial is exactly the original output's curve/polynomial. The H hidden observations fix its full restriction before the original fresh raw answer. The transport proof therefore supplies the same prechallenge nonzero polynomial `h'`, degree at most D_*, and accepted root mass at most rho. For the 1024-bit balanced sampler,

`rho=min(1,D_* kappa/2^1024)<=D_*/|F|`.

Do not condition raw uniformity on root-good, `C_event`, source equality or future acceptance. As in the transport proof, the accepted root event is bounded unconditionally for each augmented prechallenge history, and the bad-event probabilities are added separately. In particular a coincident private/main raw answer is allowed, and `h'=0` is handled by the original terminal false-polynomial test, not discarded.

Thus

`Pr_EH[C_event] <= rho+eps_root(N,G)+eps_RB(T_red)+eta_samp+e(v+H)`.

Combining this with the correlation lower bound, sandwich and real-to-emulated step proves the new theorem:

\[
\boxed{\displaystyle
\alpha_R\le\min\!\left\{1,\ e(v)+Q\sqrt{\rho+\varepsilon_{\rm root}(N,G)
+\varepsilon_{\rm RB}(T_{\rm red})+\eta_{\rm samp}
+e(v)+2e(v+H)}\right\}. \tag{CM}
\]

`Q` remains **outside** the square root. The new root error is inside it; it is not merely added once outside to the old real-world bound. Consequently its standalone contribution can be bounded by `Q*sqrt(eps_root)`, approximately `sqrt(6)*Q^2/2^(lambda/2)` under the simple two-Q bound. Numerical admission must use the entire actual reduction budget, not honest ledger n or an old Q32 policy.

All sources, setup distances, source-extraction failure/cutoff terms and outer/guest premises remain those of the application reduction. Formula (CM) proves only this extended zero-finding component. No selected concrete oracle, full guest/profile adequacy, or complete numerical receipt policy follows.

## 7. Relation to native-stream compilation

The tag-00 algorithm, tag-01 native squeezes, native key correspondence eta and mandatory finalization are unchanged. In the ordinary-C native-soundness reduction, CM-root computations are additional adversarial low-level calls. Tag-10 entries can simply remain independent cached entries of the component simulator when only the native high-level interface is compared. All their A calls and state fields are counted in that theorem's Q,N,G. Replacing literal CM by this protocol does not shrink those budgets to logical challenge sites.

If a joint ideal high-level CM interface is desired, explicitly extend the simulator: a first well-formed tag-10 query on `Final(M)` in the CM namespace obtains an independent flat `F_CM(M)` block; unrooted/invalid queries get cached random answers. CM and native flat namespaces are disjoint. Extend the invisible observer to the CM chains and count their hidden expansion and squeeze entries. The same label, collision, forward-prequery and hidden-state-guess proof of CHAINED_TRANSCRIPT, §§4–5 applies, with

`N_chain=Q_direct+sum_(P in high-level prefixes)(floor(|P|/b)+1)+B_high`,

and its original `eps_chain(N_chain,G_chain)` formula. The high-level interface returns only the raw block, not the finalized state. A receipt publishing that state computes/exposes it by the counted low-level path, not for free through that interface.

Neither version is used to claim that a programmed component simulator still has the original ARO extension. For (CM), first perform complete paired ARO emulation, then analyze ordinary-C roots in EH. For the separate native flat-language comparison, first emulate that entire producer/predicate and only then perform stream simulation. The zero-finding theorem above uses ordinary random C directly and has **no extra eps_chain term**; an application that also invokes native-stream compilation must include its separate loss once in the appropriate native reduction.

## 8. Exact sizes and cost ledger

### 8.1 Arity, graph dimension and full source capacity

For literal CM the previous width was

`m_old=max(m_abs,m_nat,2+c_CM)`.

For this extension it is exactly

`m_new=max(m_abs,m_nat)`,

`m_abs=2+lambda+k+1+ceil(log2(b+1))+8b`,

`m_nat=2+lambda+s+j_w`, `a=m_new+lambda+w`.

The full M length affects absorb/query/CPU/public costs, not a single input's width. The witness width w may itself depend on the selected m and concrete candidate graph; it cannot silently be kept at zero. This is an ideal-model theorem for the stated B,w,d, not admission of any efficient concrete arithmetization.

For the quotient field profile `lambda=1024`, `b=64`, `k=s=j_w=64`, one has

`m_abs=1610`, `m_nat=1154`, `m_new=1610`, `a=2634+w`.

For n graph targets, the graph ledger still has n targets, not `n+A_CM`: CM-root calls are externally evaluated. But every target now has the selected a coordinates. The curve has a(n-1)-degree coordinate budget in aggregate, and a*n coefficients if densely materialized. Its composed polynomial degree is `D_f=ad(n-1)` and its honest quotient needs at least `D_f-n+1` extension coefficients. Provision the actual full power-of-two capacity

`N_t>=D_f-n+1`, with `4N_t` native K coefficients and `D_*=max(D_f,n+N_t-1)`.

At 16 bytes per K coefficient this full decoded coefficient table occupies exactly `64N_t` bytes, before original lattice source digit expansion, commitment hints and workspace. A larger physical backend grid enlarges N_t and D_*; it cannot be ignored as honest padding. This is the quotient source cost, not the original CCGOS dense-f accumulator representation.

### 8.2 Public bytes, without fictional handle compression

Specify the wire representation before quoting a size. With canonical F atoms of `b_F` bytes, the first point is Boolean and may be packed; the second point is arbitrary in `F^a` and remains a field vector.

- Packing the first point and explicitly storing both target values costs `ceil(a/8)+(a+2)*b_F` bytes.
- In the equally explicit codec where the first target is the fixed implicit constant 1, it costs `ceil(a/8)+(a+1)*b_F` bytes.
- The original all-field encoding of both points and both targets costs `2(a+1)*b_F` bytes.

Reject nonzero unused bit padding. These formulas store the full first-point cm/raw/z bit vector; omitting reconstructible cm fields is a separate wire optimization, not included here.

For the favorable **illustration** `w=0,a=2634,b_F=64`, the three sizes are respectively **169,034**, **168,970**, and **337,280 bytes**. Actual w increases the packed representations by `64w+ceil((2634+w)/8)-330` bytes. There is no claim that w=0 implements an existing hash.

Add the exact original `Enc(C_g,C_t)`, public shape/profile/nonce and application bytes, the quotient claim if separately transmitted, the ordinary outer proof and its actual metadata. Reuse duplicate fields only with an explicitly linked wire codec. `t_beta` alone is one F atom (64 bytes in this profile); no size is invented for the original lattice handles or complete outer proof. The public root reduces component input width, **not** total handle transport to a digest.

### 8.3 Honest and external calls

For one public message M, define exactly

`A_CM(M)=floor(|M|/b)+1`.

Without cross-invocation caching:

| Work item | Required work |
|---|---|
| Honest root construction | A_CM component A calls, including finalization |
| Honest CM raw/witness | One `C(cm)` and one WO request; paired caching needs at most one new underlying C activation |
| External receipt root check | A_CM actual C calls, plus exact encoding/input equality |
| External receipt graph checks | The same two Vhat calls as before; no extra raw C call is needed |
| Normalized security tester | A_CM root calls and an explicit retained pair at cm, cached when already present |
| Private probe | At most one complete private continuation, including its own root tester and retained pair |
| Root graph witnesses stored in C_g | None; these new external calls are not accumulated into g |

Within each root construction, prefix sharing/cache hits can reduce **fresh** activations; calls and CPU are still charged. In a nonce-retry profile, sum A_CM over every actually tried message/branch unless a precisely maintained common-prefix cache removes that work. Public verification must not trust producer-side caches. Bound Q by all derived producer/extractor/root/tester activations, and the probe by its separately bounded private Q. A previous bound on native challenge sites is not such a bound.

Native absorb/squeeze ledger counts from CHAINED_TRANSCRIPT remain the counts for n. For security, add all external/prover CM-root calls and output lookups to the complete underlying-head bound. For native stream-simulation security also add any high-level expansion counts; never silently amortize a private branch or final verifier away.

### 8.4 CPU, memory and strict reduction runtime

A streamed honest/external root check uses `O(|M|+A_CM*m)` bit work for canonical encoding, packing and state handling, plus the actual sum of costs of its A_CM calls to C; working memory is `O(m+b*8)` bits beyond already held/publicly streamed M. An ideal C call has no established wall-clock software cost. Root-dependent raw/witness work adds its own input packing, one raw call and the selected B/WO cost. Outer public-input size/guest cycles also reflect the selected actual a and full public tuple.

A direct honest quotient method still costs H extended evaluations, curve construction, composition interpolation and exact division. For example, straightforward interpolation of g uses `O(a*n^2)` field work, evaluating it at H points costs `O(a*n*H)`, scalar interpolation costs `O(H^2)`, and schoolbook division/product preparation costs `O(D_f*n+n^2)`; this is an explicit conservative algorithmic upper bound, not a lower bound or benchmark. Storing the materialized g and composition/quotient takes `O(a*n+H+N_t)` F atoms beyond sources. Fast algorithms may replace these counts only with their actual costs. The external verifier does not perform this composition/interpolation.

The root-label/collision analysis is implementable with exact balanced dictionaries and persistent parent/block labels. A safe bit-cost bound for N activations is

`T_labels=O(N*m*log(N+1)+sum_checked |M|)`,

with `O(N*(m+log(N+1)))` bits for input/output/state dictionaries and labels, excluding source witnesses and the paired kernels. This counts bit-string key comparisons rather than assuming unit-cost unlimited-word hashing. Checking exact full-message equality costs `O(|M_main|+|M_private|)` in the reduction. Labels can also be proof-only monitors: the actual collision producer simply compares the already returned full messages before trying source comparison.

Let `T_0_new` include the strict normalized producer/tester, actual CM root processing, all its extraction/fork calls, source decoding and auxiliary work. With the paired work function W from ZERO_FINDING_TRANSPORT, §7, a safe collision-reduction ledger is

`T_red <= 2*T_0_new + W(Q,v+H)+W(Q,v)`

`         + T_copy + T_int + T_cmp + T_labels + T_bounded_randomness`.

Remove any overlap already included in the chosen W/dictionary ledger. Copy the full prefix table, paired record, producer/source snapshot and root labels, not just visible answers. The main/probe fresh tables together require at most `2Q*lambda` random output bits, plus their witness/kernel/producer randomness. No additional main Vhat calls beyond H are introduced by the root proof. Retain the transport note's field-sampling cutoff bound

`N_F <= (v+H)(a(v+H)+1)+v(av+1)`,

`eta_samp <= (N_F+1)*2^(-L_trials)`,

with all other reduction-owned random sampling explicitly added. Enlarged m,a,Q and actual polynomial work must enter these formulas. Root outputs themselves are fixed-length unbiased bit draws and need no rejection sampler.

A representative external verification ledger becomes

`V_receipt = V_outer + 2*Cost(Vhat)`

`            + sum_(i=1)^A_CM Cost(C,A_input_i)`

`            + Cost(canonical public decoding/root-input assembly/equality)`.

The corresponding whole-run prover ledger adds root construction/nonce attempts and retains original C_g/C_t creation, guest trace, native frontend/update/decider, quotient computation and complete outer proof work. This is not a performance parity claim. In particular reducing m can improve every graph coordinate cost while a long public M still requires linear external hashing/decoding work.

## 9. Evidence and admission boundary

The mathematical arguments in §§4–6 are **derived here**, not attributed to a cited paper as an existing compact-CM theorem.

- [CHAINED_TRANSCRIPT](AKITA_INSTANCE_ACCUMULATION_CHAINED_TRANSCRIPT.md), §§2–5, 7–8 supplies the exact 00/01 absorb/finalize grammar, rooted-label method, fixed-input simulation order and old literal-CM width. Its §2.2 explicitly forbids the unproved digest substitution that this new theorem replaces.
- [ZERO_FINDING_TRANSPORT](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md), §§3–7 supplies the first paired activation, one independent private continuation, real-ARO sandwich, full-capacity source comparison, actual H hidden observations, Q-outside-square-root probability law and strict sampling ledger. The only changed logical premise is same-full-message recovery, plus the enlarged root/query/runtime resources.
- [QUOTIENT_MLE](AKITA_INSTANCE_ACCUMULATION_QUOTIENT_MLE.md), §§2–5 supplies the `4N_t` full-grid decoding, enlarged D_* and ordinary same-original-C_t opening. None of its physical-capacity requirements is shortened by hashing public handle bytes.
- CCGOS, *Proof-Carrying Data From Arithmetized Random Oracles*, [official accepted manuscript](https://wrap.warwick.ac.uk/id/eprint/176570/1/WRAP-proof-carrying-data-arithmetized-random-oracles-Spooner-2023.pdf), Definition 4.1, Lemma 7.1 and Construction 8.2 ([local text](SOURCE_PROVENANCE.md#supplemental-primary-sources), lines 1260–1278, 2143–2187 and 2422–2476) defines the ideal graph extension and the original generic CM-based accumulator. It does not prove this externally recomputed same-handle compact encoding. The unrestricted emulator/prefix-persistence route is not used.

Admission still requires the complete selected source/profile/guest projection, native transcript emission and sampler contract, certified weak-source comparison inventory, actual complete Q/v/runtime bounds, and any chosen concrete-oracle hypothesis **separately**. In particular there is no claim that a fixed practical hash plus its deterministic arithmetic circuit is a sampled ideal ARO. A protocol that omits external root recomputation, uses unframed messages, drops finalization, permits counter wraparound, places postcommitment root data back into C_g, or allows uncounted/programmed component-oracle access does not satisfy this theorem.

Evidence consists of source-note/primary-text inspection and the proofs/counts above. No source implementation, branches, Git operations, builds, tests, formatters, benchmarks or concrete-hash experiments were performed. The conditional theorem has independent mathematical approval; application/profile/model admission remains separate.
