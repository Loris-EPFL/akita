# Fixed-input compilation of the native ideal transcript

## 1. Result, invariant, and scope

**Result.** This construction has a bounded classical ideal-oracle theorem under the finalization, native-key and lifecycle contracts below. A fixed-input random function implements the native ideal answer streams with an explicit simulator and loss. ARO-side language soundness additionally uses the independently reviewed **input-local paired** game-preservation theorem **before** stream simulation; the unrestricted CCGOS emulator citation was superseded by [the scope audit](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md). Neither step instantiates a concrete hash or arithmetization.

The anchor is unchanged: native pre-PCS segment-instance accumulation, original K sources, E challenges, the 33 inner roles, finite T16, and one complete native final decider. Only the formal entropy source is compiled. No completed segment proof, Merkle PCS, A/B/F homomorphism, or CH-I/CH-O hypothesis is introduced. The independent outer H_o remains independent and is not compiled by this note.

This is a new **formal construction**, not a claim that today's transcript code implements it. The previous [flat-input capacity result](AKITA_INSTANCE_ACCUMULATION_ORACLE_QUERY_COST.md) remains correct for its representation. The construction reduces input arity but adds authenticated absorb queries; the same dense CCGOS proof still fails a useful concrete T16 capacity comparison below.

The theorem covers ordinary adaptive classical oracle access, arbitrary component-oracle inputs, repeated prefixes, interleaved sessions, and a single bounded complete game. It does not assert a quantum-query theorem, a rewinding extractor that programs this component oracle, or a general theorem for unbounded multi-stage environments with inaccessible retained state.

## 2. Exact native interface and canonical encodings

### 2.1 The flat reference must be fixed first

Let H_nat be the **native ideal-coordinate-stream interface used in the native soundness proof**, not a short random seed followed by a concrete XOF. For each native stream key k it supplies an independent infinite unbiased bitstream; repeated k supplies the same stream. Different coordinates that the native proof treats as independent streams have different keys. Coordinates drawn from one simultaneous vector stream retain that one stream and its original consumption rule.

Require a fixed, efficiently invertible, injective correspondence

`eta(P, site) = native stream key`,

on the admitted native keys. P is a bounded byte encoding of exactly the context and preceding transcript that the native proof binds; site encodes the native stream's role/stage/coordinate namespace. This is an oracle-key renaming, not a new independence assumption. The condition forbids:

* omitting public statement, provisioned setup/profile bindings, commitments, round messages, or already-bound challenge history;
* assigning a fresh implementation-session identifier to two executions that have the same native key;
* merging different native keys, splitting a single native stream into independent streams, or absorbing earlier coordinates of a simultaneously drawn vector when the native interface does not;
* moving challenge-dependent data before its challenge or pre-sampling a later adaptive round.

A concrete canonical record codec available for this correspondence is

`Rec(type, payload) = u32be(type) || u64be(len(payload)) || payload`.

The type registry and field/atom codecs are fixed public profile data. A transcript begins with a versioned context record; records for ordered prover/verifier messages and required boundaries follow. Tuples and arrays are recursively typed and length-framed, with fixed-width counts. Canonical K integers occupy 16 bytes, and an E atom is its ordered pair of canonical K coordinates; reject values outside the field rather than reducing them. Other public objects retain their actual canonical native codec. Variable strings and arrays are length-framed, not concatenated ambiguously. Every length fits its declared field and the total P length is at most L. The codec is uniquely parsed by induction on records: the fixed header determines exactly the next payload boundary, and each payload codec is injective.

This supplies an actual byte grammar, but not a source-level emission table for the unimplemented native archive. Applying the theorem to that archive still requires demonstrating that its ordered emission table implements eta. If its native keys use another injective encoding, eta is a renaming between those encodings. There is **no** assertion that merely adding the above record wrapper to arbitrary existing code automatically establishes eta.

Define an auxiliary flat block oracle

`F(P, site, j) = bits [j*lambda, (j+1)*lambda) of H_nat(eta(P,site))`.

Outside the image of valid native keys, F has independent private streams. The reduction can simulate those unused namespaces locally; they cannot alias native keys. This extension lets the component simulator handle nonsensical but well-framed prefixes without discarding adversarial queries. Require efficient recognition of the native-key image, or alternatively define the native oracle on all injectively framed byte keys from the start.

The F interface is random-access blocks for convenience. When the native interface materializes a sequential prefix, a query at block j costs generation through j, not one lambda-bit operation. Section 7 retains this distinction.

### 2.2 Fixed component domain

Fix before oracle sampling:

* output/state length lambda bits;
* payload block size b bytes, b >= 1;
* absorb counter width k, site width s, output-index width j_w;
* maximum admitted transcript length L and output blocks J <= 2^j_w;
* ell = ceil(log2(b+1)); a public root IV = 0^lambda;
* the full CM-challenge payload encoding and maximum bit length c_CM, including any length fields, complete commitment tuple, profile identity, and nonce.

For variable CM payloads, its declared c_CM-bit encoding must itself include length and canonical zero padding. It is not a digest. Set

`m = max(2 + lambda + k + 1 + ell + 8b, 2 + lambda + s + j_w, 2 + c_CM)`.

Choose one uniformly random function C : {0,1}^m -> {0,1}^lambda. Its typed inputs, padded on the right with zeros to exactly m bits, are:

```
A(state, counter, final, length, block)
  = 00 || state[lambda] || counter[k] || final[1]
       || length[ell] || block[8b] || zero_pad

S(state, site, index)
  = 01 || state[lambda] || site[s] || index[j_w] || zero_pad

CM(payload)
  = 10 || payload[c_CM] || zero_pad
```

All integers use fixed-width unsigned big-endian encodings. Tag 11, nonzero padding, out-of-range lengths, and other malformed strings are still ordinary inputs to C; an adversary obtains their random outputs. They are simply not accepted encodings of protocol operations. Namespace restrictions apply to the honest algorithm, **not** to adversarial oracle access. These domains are disjoint by the first two bits, and injective within each domain by fixed field boundaries. Native context/version is in P, so a single primitive can serve several explicitly distinguished contexts without extra public roots.

The CM clause is important: shortening native transcript queries does not show that a tuple such as `(C_g,C_t,profile,nonce)` fits. If it does not fit, m grows. Hashing this tuple down would be a separate change to the query-accumulation theorem and is not performed here.

## 3. Actual absorb, finalize, and squeeze algorithms

### 3.1 Stateless specification

For a byte string P with |P| <= L, write uniquely

`P = B_0 || ... || B_(h-1) || tail`,

where each B_i has b bytes and 0 <= |tail| < b. Require h <= 2^k-1; data steps must leave room for this final counter. Define:

```
s_0 = IV
for i = 0,...,h-1:
    s_(i+1) = C(A(s_i, i, 0, b, B_i))
s_fin = C(A(s_h, h, 1, len(tail), tail || zeros(b-len(tail))))
Chain^C(P, site, index) = C(S(s_fin, site, index))
```

There is **always one final absorb**, including for the empty string and exact multiples of b. A canonical data edge has final=0 and length=b. A canonical final edge has final=1, length<b, and exactly zero trailing padding. No final node has a canonical absorb successor. Thus complete encoded paths are prefix-free as paths of typed component queries, while common data prefixes can be shared. The counter is the number of preceding full payload blocks, never a sampler-attempt count.

The high-level answer is only the squeeze block. Neither s_h nor s_fin is returned by that interface. If an adversary computes them through its component queries, the simulator sees those queries; exposing them for free through the high-level interface is a different construction.

### 3.2 Incremental implementation as a mathematical algorithm

An appendable state is `(s,h,buf,total,closed)` with initial `(IV,0,empty,0,false)`. AbsorbBytes appends canonical record bytes, checks total<=L, and processes each completed b-byte block by the data step above; buf remains shorter than b. Counter overflow or malformed native records rejects before accepting an archive. Work is linear in newly supplied bytes; it is unnecessary to rehash an entire prefix at every challenge.

Freeze creates the final state by the single final absorb on a **snapshot** of `(s,h,buf)`. It does not overwrite the appendable data state. A frozen handle identifies `(P,s_fin)` and has one deterministic block cursor/bit cursor per native stream site. SqueezeBlock uses exactly the S encoding and advances that site's cursor; repeats of a named block reproduce its value. Random access is permitted at the oracle interface even if the honest sampler uses sequential access.

When the protocol continues, it appends exactly the next native records, including only the challenge history the reference interface requires, to the retained appendable state. It never continues an absorb chain from s_fin. Finishing the native archive marks its logical session closed; no further protocol operations are accepted. A reset creates IV and reabsorbs the same context; it does not introduce hidden salt. Copying a transcript state is allowed as an algorithmic optimization, but both copies preserve their exact bytes, counters, and native-key identities. Interleaving several sessions has no special cryptographic rule: identical keys agree; different keys are separated by their encoded context/transcript/site.

The oracle-free skeleton must reconstruct every actual A and S input and every use of its answer, including partial-block padding, finalization, counters, failed sampling attempts, and declared overflow/abort outcomes. Merely supplying a claimed digest plus challenge answers is not this algorithm. Identical query/answer entries may be deduplicated only with checked coverage of all uses.

## 4. Full simulator, including unrooted component queries

The simulator Sim^F receives only direct component queries. It is **not** notified of the distinguisher's F queries.

Its state consists of:

* an exact input/answer cache T for all m-bit component inputs;
* a rooted-label map R with `R[IV] = Data(empty,0)`;
* persistent parent/block descriptions for labels, so bytes need only be reconstructed when a rooted squeeze requires P;
* independent random coins. It can equivalently pre-sample an independent random table for all A inputs; laziness is an implementation of this table, not a changed distribution.

On a component query x:

1. If x is cached, return T[x]. In particular, an earlier unrooted answer is never overwritten.
2. If x is a correctly padded A input, sample y uniformly from {0,1}^lambda. If its parent has a Data label, the counter equals that label's block count, and the edge is canonical and stays within L/counter bounds, derive its child Data or Final(P) label. Install that label at y **only if y is not already a labeled state**. Otherwise keep the old label. Cache y and return it. Unrooted, wrongly counted, oversized, or otherwise invalid A inputs still get an independent random answer, but do not install a rooted label.
3. If x is a correctly padded S input with index<J and its state has a Final(P) label, set y=F(P,site,index). Cache and return y. An out-of-profile site can be treated as a separately simulated private F namespace, consistently with Section 2.
4. For every other input—including an S query on an arbitrary guessed state, a Data state, an out-of-range index, CM input, or malformed encoding—sample an independent uniform y, cache, and return it.

This is a total algorithm even on a collision. It does not search for an unqueried transcript preimage, reinterpret an unrooted query as rooted later, or discard adversarial queries. Its intentional arbitrary collision behavior will be charged in the theorem.

For Q direct component queries it makes at most Q F block requests, and no more than the number of first rooted S queries. Repeated requests to one `(P,site,index)` are cached by F. Its number of different F stream keys is no larger than its number of block requests. Using balanced tables and parent pointers gives work

`O(Q*m + Q*log(Q+1) + sum_rooted_squeezes |P| + generated_F_bits)`

in a bit-cost model up to dictionary/encoding implementation factors, and storage `O(Q*m + stored_F_bits)`. One may instead use a linear-search table with an explicit O(Q^2*m) bound. The theorem does not depend on a unit-cost unlimited-word hash-table assumption. These bounds include reconstructing the actual flat query, not just a short state lookup.

## 5. Simulation theorem and proof

### 5.1 Statement and resource variables

A classical distinguisher D adaptively accesses both interfaces in either experiment:

`Real = (Chain^C, C)` versus `Ideal = (F, Sim^F)`.

The oracle descriptions, IV, encodings, and D's initial auxiliary input are independent of the sampled oracles. Oracle-dependent preprocessing must be included as queries in D. All actual high-level P lengths and indices obey the bounds; off-domain high-level inputs receive a fixed rejection in both worlds.

Let Q bound direct component queries. Let H be the set of different high-level prefixes P queried by D, and B_H the number of different high-level block triples `(P,site,index)` queried. A sufficient bound on the number of distinct component-output variables touched by the complete interaction is

`N = Q + sum_(P in H) (floor(|P|/b)+1) + B_H`.

Shared prefixes/cache hits make this an overcount. Count low-level outputs, all hidden high-level absorb outputs, and high-level squeeze outputs; do not count the same component entry twice. Let G<=Q bound direct queries carrying an A/S state field; one may simply take G=Q. All these are worst-case bounds after the declared cutoff, not expected counts conditioned on success.

Write M=2^lambda and d0=M-(N+G+1). If d0>0, define

`eps_chain(N,G) = min(1, [N(N+1)/2 + 2NG] / d0)`.

If d0<=0 use the trivial bound 1. Then

`|Pr[D^Real=1] - Pr[D^Ideal=1]| <= eps_chain(N,G)`.

This deliberately conservative bound includes output collisions which need not actually be harmful. It prices hidden-state guesses as well as collisions, and includes high-level evaluation length. For example, when N+G+1<=M/2 it is at most `[N(N+1)+4NG]/M`; it is not a loss in just the number of logical challenge sites.

### 5.2 Augmented experiment and bad events

For the proof only, augment the ideal experiment with an invisible observer. At each high-level F query it evaluates the corresponding data/final chain using the same independent random A table that supplies simulator coins. It records the virtual S entry associated with that block. It does not give the simulator the path, change the simulator's labels, or expose any hidden state to D. Thinking of the A table as sampled in full at initialization makes this legitimate: the observer only reveals entries to itself. The actual simulator still lazily queries its own table when it needs an A answer.

A rooted path can now be known to the simulator, known only to the observer, or shared. Stop the coupling at the first of:

1. **Output/root collision:** two distinct output variables recorded so far have the same lambda-bit value, or one equals IV. Include outputs from all domains, not only honest A states. Reusing the same entry is not a collision.
2. **Forward prequery hit:** a newly assigned output value equals a previously submitted direct A/S state field that was not already that value's legitimate exposed predecessor. This includes an unrooted state queried before a later honest chain reaches it. Charging all such equalities, even harmless ones, only enlarges bad.
3. **Hidden-state guess:** a direct query's state field equals an observer-known A output whose rooted path has not yet been exposed to the simulator. This includes queries after a high-level call but before the adversary computes its compression path.

A namespace/encoding alias would also stop the proof, but Sections 2–3 exclude it deterministically. A cross-session final-state alias or a second rooted interpretation of a state implies item 1; an old unrooted squeeze later becoming a valid squeeze implies item 2 or 3. Those are not omitted independent events.

### 5.3 Counting bad without an invalid freshness assertion

Here is the deferred-value fact used in the count. Before the first bad event, an unexposed A-table answer is never used in an adversary-visible operation except as an internal index into a fresh independent random-function entry. Its numeric value therefore remains hidden; the only restrictions placed on it by the good history are inequalities with other recorded output values, IV, and submitted state fields. After fixing the other deferred values, at most N+G+1 values are excluded. Its conditional point probability is at most `1/d0`, **not** asserted to be exactly `1/M` after conditioning on good.

This fact follows by induction over operations. A fresh A entry has a uniform value independent of its input. A hidden-state-indexed new entry produces independent output; if the entry had already been directly queried, the corresponding state-field equality is precisely a prequery/guess bad event. High-level squeezes reveal independent F values rather than A states. Simulator-rooted squeezes either reuse that same recorded F value or reveal a fresh F value. Distinct native block keys cannot coalesce without a state collision or encoding alias. Thus the induction introduces no constraint on a deferred state other than the displayed inequalities. Conditioning on the other deferred labels leaves at least d0 equiprobable candidates. This also explains why revealing the final chaining state through the high-level interface would invalidate the argument.

For the r-th distinct output variable there are at most r forbidden values for collision/root hit (IV and r-1 preceding outputs). Summing gives `N(N+1)/(2*d0)`. Forward prequeries have at most G forbidden state fields per fresh output, giving `NG/d0`. Each of at most G direct state-bearing queries can guess at most N hidden values, giving another `NG/d0`. A union bound at the **first** bad event gives eps_chain. Cached/reused ideal values are counted at their first assignment; their later equality with a newly generated state is charged when that new state is assigned. Values first revealed by the high-level interface are included in the observer's output table, so they do not disappear from the count.

### 5.4 Joint distribution before bad

Couple the two experiments using the common A table and common fresh values for other component entries. Whenever a high-level or rooted-low squeeze first needs `(P,site,index)`, use its F block as the output of its virtual/real S entry. Before bad this entry has not already been assigned an unrelated answer: a prior unrooted query would trigger item 2 or 3. Distinct triples have distinct S inputs because their finalized states/sites/indices cannot alias. Consequently the assigned value is either the same already-used entry or a fresh independent block, exactly as required by a random component function.

Inductively:

* both interfaces agree on every previously returned answer;
* every simulator-rooted path has the same bytes, counters, state, and final/data status as its real/observer path;
* a path first used by a high-level query can later be exposed by direct A queries, in root order, with the same A-table answers;
* an attempt to enter such a path without exposing its predecessors is charged as a hidden-state guess, rather than silently granting the simulator knowledge of a high-level query;
* all unrelated/unrooted/malformed/CM entries retain independent random answers.

Thus the complete visible transcripts, including arbitrary low-level queries and all returned high-level blocks, are identical up to bad. The fresh-entry/deferred-table argument gives the correct random-function marginal on the real side and the actual Sim^F marginal on the ideal side; it is not merely equality of challenge marginals after conditioning on success. The fundamental identical-until-bad argument and Section 5.3 finish the theorem. No concrete compressor is programmed in this proof. ∎

The separated final absorb/squeeze and snapshot lifecycle are essential parts of this proved construction. Raw Merkle–Damgård output does not satisfy the same theorem: querying H(M)=u, then the component at `(u,next_block)`, exposes the extension relation although an ideal-world simulator has not learned M. Likewise, ambiguous framing or wraparound counters creates deterministic aliases not bounded by eps_chain.

## 6. Exact samplers, vectors, and aborts

The theorem is at the bitstream layer. Applying any **identical deterministic** native sampler/bit-consumption machine to the two coupled streams preserves the bound. It does not authorize replacing the native shell sampler by a different procedure just because both produce uniform accepted challenges.

For a vector, freeze P once and use the native stream identities and coordinate order. Maintain a precise bit cursor, including rejected candidates and unused bits. Buffering across coordinates of the same vector is allowed only when it reproduces that native stream machine. Unused bits from one adaptive round must not become the next round's independent stream. Advancing a rejection counter changes only the selected bits/blocks unless the native key itself includes that counter. S block-index overflow, counter overflow, transcript-length overflow, or exhausted declared attempts returns reject/abort, never a default field value or successful transcript.

Two distinct comparisons should not be merged:

1. The bounded compiled protocol and the **same bounded** flat-stream protocol differ by at most eps_chain. Abort is one of their coupled outcomes.
2. Comparing a bounded sampler with an unbounded native exact sampler needs an explicit truncation/abort analysis. For language soundness, if the bounded verifier only rejects earlier and otherwise runs the exact same sampler prefix, its acceptance event is a subset of the unbounded verifier's event; no positive soundness loss is required. Honest completeness loses the honest abort probability. For total transcript distance, add the probability of reaching the cap in the relevant experiment, including any adaptive search budget; an honest single-execution bound is not that adversarial bound.

For clarity, the following is a fully specified **optional flat adapter**, not an assertion about existing source behavior. Read successive 128-bit integers for each of two K coordinates of an E draw, rejecting candidates >=q, where q=2^128-2^32+22537. With r_K attempts per coordinate, an honest execution with at most C_E draws aborts with probability at most

`2 C_E rho_K^r_K`, where `rho_K=(2^32-22537)/2^128 < 2^-96`.

For the selected shell let `M_C=binom(128,40) binom(40,24) 2^40`. Read 187-bit ranks, reject ranks>=M_C, and use a fixed lexicographic support/magnitude/sign unranking bijection. With r_C attempts and C_C shell coordinates the honest abort bound is

`C_C rho_C^r_C`, where `rho_C=1-M_C/2^187 < 1/8`.

Conditional on the stopping attempt and success, each accepted candidate is exactly uniform on its target set by equal-sized candidate fibers. This is a local sampler fact, not a claim of uniformity conditioned on an adversary's final accepting event. For an unbuffered implementation each attempt discards unused bits in its last block. Then

`S_native <= 2 C_E r_K ceil(128/lambda) + C_C r_C ceil(187/lambda) + S_other`.

To use this optional adapter with the exact native theorem, either the reference native stream machine is precisely this adapter, or an additional distribution/interface equivalence argument is supplied. The core compilation theorem instead works directly with the already admitted native machine, whatever its counted consumption rule is.

For the cost note's syntactic profile, C_E<=17441 and C_C<=1992. Taking lambda=512, r_K=2, r_C=48 in this optional adapter gives at most `165380 + S_other` squeeze blocks and honest abort less than `2^-176 + 2^-133 < 2^-132`. This is a bound, not a measured execution or an admission of these caps into the native protocol.

## 7. Reduction to the native flat game and then the ARO-side lift

### 7.1 Component-ROM language-soundness game

Let A^C output a purported native archive and its public statement. Let p_chain^C be the full native false-acceptance predicate, using the above compilation for every native stream and rejecting all declared parse/cap failures. Include all actual native verification work and the one final decider.

Apply Theorem 5 to the distinguisher which runs A with the **low** interface and evaluates p with the **high** interface. Its ideal version is exactly a flat native adversary A_flat=A composed with Sim^F, followed by the native flat verifier, under eta. Independent private F namespaces do not alter that verifier. Hence

`Pr[p_chain^C(A^C)=1] <= eps_native_flat(B_flat) + eps_chain(N,G)`.

Here Q is all A component queries, including queries implementing its own chain computations, off-protocol trials, and any included preprocessing. The high-level expansion term counts the final verifier's full absorb/squeeze work. If A uses a provided high-level service too, add those service calls and expansions explicitly.

A_flat's number of native stream keys is at most the number of simulator-rooted squeeze block queries, hence at most Q. The verifier's queries enter the native theorem exactly according to that theorem's convention. A block query at index j does **not** add j+1 distinct native stream keys, but can cost j+1 generated blocks in a sequential stream model. More precisely, for each requested key let j_max be its largest requested index; the total generated bits are at most

`lambda * sum_keys (j_max+1)`.

Use the actual native theorem's query/bit/work convention when forming B_flat, and include simulator work from Section 4. The old Q32 policy for an original archive prover is not a bound for a derived receipt/extraction producer.

### 7.2 Correct order with ARO side access

Let `Theta=(C,wo,Vhat) <- ARO[F_ar,m,lambda,d,B]`, with d>=2 and **input-local** fixed B: `wo(x)=b(x,mu_x,C(x))`. Cross-input B is not covered by this lift. F_ar is the graph field, distinct from the flat block oracle F. The final native predicate p_chain uses only C; independent H_o programming inside the producer is private computation, and B never accesses H_o.

Apply the [repaired paired theorem](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md) to the entire producer/predicate computation. It yields a C-ROM simulation with error

`eps_ARO <= min(1,v_total/2^lambda) <= (t_A+t_p)/2^lambda`.

Here v_total counts every extended observation and the coarse t counts include the complete initialization/producer/predicate interface. Then apply §7.1. The following conservative bound retains that coarse term:

`Pr[false native output from A^Theta]`
`  <= eps_native_flat(B_flat) + eps_chain(N_ROM,G_ROM)`
`     + (t_A+t_p)/2^lambda + delta_cut`.

If cutoff is already included in a parent extraction term, do not add delta_cut again. This is the requested ARO-side lift; it never attempts to couple Vhat directly to a newly programmed compression table. The reverse informal move—first swapping C for Sim^F while leaving its original wo/Vhat attached—has no justification from this theorem, since their graph constraints refer to C.

For exact resources, let h_A be the producer's activated RO/WO heads and h_p the additional predicate heads. The ro-only paired implementation makes at most h_A underlying C requests during production and at most h_A+h_p in the complete cached experiment. It makes none on Vhat queries. B's repeated local C gates use the cached value, but their CPU remains charged; even ro-free B needs the first paired C fetch.

Include the final predicate's actual absorb/squeeze evaluations in N_ROM and its native-query convention. With h=h_A+h_p and v=v_total, a safe complete-work ledger is

`T_whole <= T_A + T_p + C_M(h,v) + T_chain + T_projection`,

with overlaps removed if already charged. C_M is the repaired emulator's full polynomial bit-cost bound, not a unit-cost field/oracle operation or a substitute for a strict cutoff.

If A includes an expected-time outer extractor, first truncate the **whole actual-oracle producer** at a declared charge L_work. From an unconditional bound E[Z]<=P_work, Markov gives `delta_cut<=P_work/L_work`; only then apply the polynomial-cost emulator and the bounded chain theorem. It is invalid to infer `E[poly(Z)]<=poly(E[Z])`, or to use an honest/native query budget instead of the derived producer's total budget. This compilation establishes no new complete-frontend or guest-adequacy theorem; those premises remain the separate obligations in the [product-extraction note](AKITA_INSTANCE_ACCUMULATION_PRODUCT_EXTRACTION.md).

## 8. Concrete size/query tradeoff

Let B_abs be the total canonical bytes absorbed over the honest verifier's append-only transcript lifecycles, and V the number of distinct prefix snapshots actually finalized. With lifecycle lengths B_l, a direct incremental ledger before optional query deduplication has

`A_native = sum_l floor(B_l/b) + V`,

`n_q = A_native + S_native`.

If a lifecycle forks and recomputes shared blocks, count its actual branches or use a correctly proved shared-prefix cache. If the verifier absorbs bytes after its last challenge, those data calls need not be executed merely to verify an unused digest; B_l in this formula counts the bytes actually processed by the declared transcript algorithm. Every required prechallenge byte is still bound. Finalization adds a query even for a short/empty tail. The ledger does not recursively include AS.P's own CM/wo/Vhat work, but security producer budgets do.

For a one-shot CCGOS archive with no old accumulators, the original construction has

`a = m+lambda+w`, `N_AS=n_q+1`, `D_dense=ad*n_q`,

`public accumulator bytes = 2(a+1)b_F`,

`dense f bytes = (ad*n_q+1)b_F`,

where b_F is the chosen canonical F_ar element width. Its curve has a(n_q+1) coordinates/coefficients if materialized; complementary witnesses and actual CM inputs are additional. The construction's random dummy point is included here. A separate quotient/PCS-held polynomial variant is not assumed in these formulas. Packing or deriving ledger inputs does not remove the a*n_q-degree capacity or authenticate unproved answers.

As a transparent numerical comparison, choose lambda=512, b=64 bytes, k=s=j_w=64. Then ell=7 and

`m_abs=1098`, `m_squeeze=642`, `m=max(1098,2+c_CM)`.

**Only if c_CM<=1096**, m=1098. With favorable w=0,d=2 and 64-byte F_ar elements, a=1610 and

`public accumulator = 206208 bytes`,

`dense f = 206080*n_q + 64 bytes`.

A 512 MiB advice region holds this dense polynomial for at most n_q=2605, before any archive, witnesses, CM data, or other advice. The existing source-grounded T16 lower bound already has 242*16=3872 adaptive native frontend stream inputs and therefore at least 3872 squeeze blocks in this construction. **Even ignoring every new absorb query**, dense f requires at least `797941824` bytes, exceeding 512 MiB. The terminal alone binds 81920 prechallenge bytes, which costs at least 1280 full-block data absorptions plus a finalization on that lifecycle. These observations are lower bounds for this stated representation/profile, not an impossibility result for quotient-held polynomials or all accumulation constructions.

For one segment, the lower bounds alone do not decide fit. The optional capped adapter's T16 upper certificate `165380+S_other` squeeze blocks is much larger than the capacity threshold, but an upper bound is not a lower-bound impossibility argument. No choice of w,d,CM tuple width, actual message lengths, or sampler codec is silently replaced by these favorable illustrations.

The separate quotient-graph proposal uses one 1024-bit raw ARO output for its exact sampler into an F_ar of size q^4. To combine that proposal with **this theorem unchanged**, set both component-output and chain-state widths to lambda=1024. With the same b=64 and counters, `m=max(1610,2+c_CM)` and `a=m+1024+w` (hence `a=2634+w` when CM fits). The 512-bit comparison above is not the joint quotient profile. Truncating the 1024-bit output to a shorter chain state would need the corresponding two-width version of the simulation bound; that variant is neither necessary nor silently used here. The quotient representation's cost and soundness are not the original dense-f formulas.

The general tradeoff, when absorb arity dominates CM/squeeze arity, is approximately

`a*n_q = (2lambda + 8b + c + w) * (B_abs/b + V + S_native)`,

where `c=2+k+1+ell` and floors/caps are retained for exact calculations. Increasing b reduces data-absorb count but increases every ARO graph point and public accumulator. Ignoring the slowly varying ell and rounding, the stationary choice is

`b_opt = sqrt((2lambda+c+w)*B_abs / (8*(V+S_native)))`.

This is an accounting tradeoff, not a security/profile recommendation. It does not remove processing of the underlying native bytes, and CM domination can invalidate the displayed optimization. With 512-bit outputs, even an illustrative N=G=2^64 gives eps_chain below 2^-381 from Section 5's conservative bound; whether the **derived** producer respects that enormous budget must still be proved. At lambda=256 the same illustrative budget gives only roughly a 125-bit bound, so output/state width cannot be selected independently of total reduction work.

## 9. Source scope and remaining admission boundaries

The proof above is given explicitly for this construction; it does not import a theorem about a named practical sponge. Its primary methodological precedent is Coron–Dodis–Malinaud–Puniya, [*Merkle–Damgård Revisited: How to Construct a Hash Function*](https://cs.nyu.edu/~dodis/ps/merkle.pdf), CRYPTO 2005 / extended manuscript dated September 4, 2007: Definition 1 explicitly gives the simulator no view of high-level oracle queries; §3.2 gives the raw-MD extension counterexample; Theorems 3.1 and 3.4 treat prefix-free and independently finalized constructions; §5 treats output expansion/domain separation. Those statements do not themselves certify our record grammar, finite counters, exact native samplers, or ARO game—the preceding sections supply those distinctions.

The ARO distribution comes from [CCGOS](SOURCE_PROVENANCE.md#supplemental-primary-sources), Definition 4.1. Its original unrestricted-B emulation claim is not used: [ARO_WITNESS_SCOPE](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md) supplies the independently reviewed input-local paired replacement and complete-run game preservation. The order remains ARO-to-ro first, then this stream simulation; no unchanged arithmetization is attached to a replaced random function. Dense-size formulas are algebraic counts from Construction 8.2 and the [cost certificate](AKITA_INSTANCE_ACCUMULATION_ORACLE_QUERY_COST.md), not a repair of the separate zero-finding transport theorem.

The formal compilation closes the narrow question “can a proved fixed-input ideal primitive replace bounded flat native streams?” affirmatively. Still required for a deployed or numerically admitted receipt are the native byte-emission/eta certificate, actual sampler/cap contract, complete derived query/runtime bounds, B/w/d, an admitted CM tuple fitting m, the remaining outer projection/guest obligations, and a query-accumulation representation that fits. A concrete primitive/graph instantiation is a further separate cryptographic claim; none is supplied or presumed.

Evidence: primary-paper and existing-note inspection plus symbolic arithmetic in this note. No implementation, Git operations, builds, tests, formatters, benchmarks, or concrete hash-security experiments were performed.
