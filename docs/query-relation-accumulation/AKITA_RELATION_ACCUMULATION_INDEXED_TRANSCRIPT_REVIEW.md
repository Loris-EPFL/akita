# Candidate1 — independent v8 indexed-transcript review and complete proposed contract

## Review result and scope

**Conditional design pass.** Direct finite indexed answers, one atomic shell request per level, attempt-major fixed slots, and decoded-only completed history are mutually compatible. They avoid both obstructions established for the ordinary v7 implementation: the 32-byte seed fibre is not used, and later keys literally contain earlier decoded results rather than relying on a Blake2b continuation state. This conclusion is derived below from the finite raw decoder and an injective grammar; it does not use an expanded local Akita extraction theorem.

The short proposed design leaves several implementation choices implicit. The complete replacement text below settles them rather than declaring that the current executable already has them:

1. A finite capped filtered coordinate has a **fatal mask outcome**, not merely a truncated geometric filter with its original uncapped parameter. Its exact law is given in §4. The existing union bounds remain safe, but they are not the exact success/stopping law.
2. Conditional inverse generation is **per coordinate**, even though the oracle request is atomic for the whole level. Rejection until a whole W-coordinate level succeeds can cost `1 / product(s_c)`; it is unnecessary. The construction below costs a sum of `1/s_c` terms and states the accepted-fraction requirement explicitly.
3. A lazy honest SHAKE reader is an execution optimization, **not** a block-query reformulation of a finite FULL-answer game. The formal oracle returns and saves the entire finite string. All such original and private answers receive a conservative full-length charge. A game that charges only observed bytes would need a separate lifting argument and is not used here.
4. Literal SHAKE prefix caching requires an append-only key encoding. An outer prefix-length/count that changes at each append cannot simply be absorbed once and cached. The concrete stream grammar below has no changing outer prefix length.
5. Atomicity must reach the consumers: the prover must draw/decode the complete bundle before the first witness-dependent group probe, and the verifier must draw it before the successor. Retaining the present interleaved group draw/probe loop is not the proposed atomic protocol.

These are precise completions/implementation requirements, not evidence of an attack on v7 or a claim that the draft's already conservative inequalities are false. This artifact makes no numerical admission, no complete AQRA soundness claim, no concrete-SHAKE security theorem, and no QROM claim. It uses only the approved programmable classical indexed ideal ROM and leaves original actual-norm Module-SIS assumptions unchanged.

Work was static source/document inspection. No repository edits, git operations, builds, tests, linters, formatters, benchmarks, or protocol runs were performed. The only write is this local artifact. Names introduced below describe the **proposed** API, not existing executable symbols.

---

# Complete proposed v8 text

## 1. Scope, anchors, installation, and resource policy

The initial ordinary AQRA profile uses `E = K` with

`q = 2^128 - 2^32 + 22537`.

Keep the original source contracts, original A/B/F commitment algorithms and their native role geometries, signed sparse polynomial families, trace/subring embeddings, checked arithmetic equations, canonical proof-payload codecs, ordinary nonce ranges, and suffix/source-comparison obligations. Source profiles are immutable after commitment. Plan all successor profiles, setup obligations, and the trusted final row before the challenges they govern. Do not change a selected L2 row into Linf by omitting its predicate. An unfiltered Linf/direct-setup baseline is a selection choice, not a bypass of a different selected row.

This is a real protocol/version cutover. It changes challenges and consequently may change selected nonces, response messages, and successor commitments. Original-output preservation below compares an original **v8** execution with its extraction worlds; it does not assert bitwise equivalence between v7 and v8 proofs. The executable anchor for a later implementation is equality of the complete v8 key/event sequence and decoded challenges between preview, live prover, and verifier for the same installed context and selected nonce, together with literal preservation of the marked original raw answers and output in extraction.

A verifier-approved installation fixes:

- protocol version 8 and exact grammar/result-codec versions;
- prime field and canonical encoding, initially the LE128 checked `Fp128` encoding above;
- exact source contracts and trusted schedule/catalog/registry identities;
- scalar, predicate, and shell descriptors, all caps, raw layouts, filter identities and exact integer parameters;
- the permitted opening methods and carrier/native-role embeddings;
- the finite event/request horizon and polynomial bounds on input, history, request, answer, and explicit algebraic-source lengths;
- an original-query budget convention and strict global extraction-work cutoff policy;
- for each selected filtered coordinate, an independently justified lower bound on its actual accepted fraction sufficient for the declared sampler cost. Nonempty accepted support alone is not an efficiency certificate.

An installed unique integer index may name exact externally fixed data. It must not be an arbitrary prover-supplied digest. If digest identities are retained instead of exact registered references/literal data, state their separate collision/identity obligation and term; do not call concrete digest equality injective equality of the underlying installation. No new lattice or concrete-hash knowledge assumption is introduced.

The PCS context includes the entire sealed frontend/query handoff and endpoint roster required by the outer protocol. A short independently restarted PCS session does not supply that binding. This does not by itself prove the legacy frontend's global chained-key compiler; that remains a separate contract unless its challenge boundary is also migrated.

All lengths, counts, products, offsets, conversions, and allocations are checked before use. The installed grammar, not proof-selected values or diagnostic logs, determines permitted sites, counts, optional-event presence, and order. An invalid message, descriptor, cap exhaustion, unsupported method, or illegal transition poisons the affected proof computation. It cannot continue under another interpretation of the same request. A public PoW predicate that fails rejects that nonce branch; it is not a canonical-decoder cap failure.

## 2. Literal key grammar, completed history, and SHAKE caching

### 2.1 Injective append-only syntax

Fix distinct one-byte top-level tags for `PUBLIC`, `PROOF`, `COMPLETED`, and `REQUEST`. Within tagged bodies use fixed-width little-endian integers and `frame(B) = LE64(|B|) || B` for byte strings/vectors, with checked arities and canonical encodings. The event body's length is also framed. The installed grammar fixes all zero/reserved fields and rejects their noncanonical alternatives.

Define the fixed header

`Header = frame(domain) || LE32(8) || LE64(installation_index) || frame(session) || frame(augmented_instance)`.

If a registered exact index is not used, replace that field by framed exact installation data and retain the same injectivity requirements. All data needed to interpret the context is fixed by this header and the approved installation.

A history is an append-only sequence of self-delimiting events:

`P = Event_0 || ... || Event_(n-1)`.

An event is `tag || frame(body)`. Each body includes its sequential `event_index`, schedule-derived site identity, and the canonical typed payload. Public and proof events have distinct tags even when their payload bytes coincide. The grammar defines whether a logical group is one event or several; the 1024/64-byte transport chunking of an implementation must not change that event sequence.

For request descriptor R, the literal key is

`Key(P,R) = Header || P || REQUEST || frame(Encode(R))`.

There is **no changing outer count/length for P before its event bytes**. Parsing consumes complete tagged frames until `REQUEST`, which must terminate the key. This gives both a unique parse and an append-only prefix suitable for caching. Event and request identities contain at least the sequential request ordinal, operation, complete `ProtocolSiteId` coordinates, and descriptor/count information. Shell descriptors additionally enumerate every `(group, claim, block)` owner in canonical order; vector/limb requests bind the exact ordered limb identities. Request raw length and all layout/cap/filter versions are explicit and verified, not inferred from a diagnostic byte count.

Every request is an atomic state transition: no public/proof event or other challenge request can occur between forming its key and either completing it or poisoning the computation. A completed event includes the **full canonical request descriptor R**, its unique owner, result kind, and the complete canonical decoded result. No raw-answer digest is substituted for this result. For a scalar it is the scalar; for an atomic level it is the complete ordered tuple of carrier-ring polynomials; for a predicate it is the entire 32-byte output, not merely its Boolean acceptance bit.

The request owner can be reconstructed without a digest: the history immediately before that completed event and its stored R reconstruct the exact earlier `Key(P,R)`. An explicit owner field must agree with that reconstruction; it is not an alternative freely supplied identity. Request ordinals/event indices increase according to the grammar, so a duplicated completed event cannot create another interpretation of ownership.

A decoder failure produces no successful completed event and has no descendant in the same computation. The complete raw answer, including failure and unused bytes, remains in that world's saved oracle table. A verifier need not transport completed events in the proof: it computes them from the same context, proof messages, requests, and oracle answers.

### 2.2 Ideal oracle and conventional concrete instantiation

The formal oracle at a fresh, well-formed finite-family key returns one independent uniformly random byte string of the declared length L. Old keys replay their **literal full strings**, not merely their decoded values. Malformed keys do not become protocol requests; arbitrary adversarial prequeries to valid future keys still count and remain in the original table. Conditioning/programming is allowed only at the relevant fresh atomic exposure after restoring its legitimate prior table.

The conventional concrete implementation is `SHAKE256(Key(P,R))`, squeezed to exactly L bytes. No 32-byte root seed, digest of P, per-coordinate seed, or alternate rehash is the expansion input. Prefix-state caching means caching the *unfinalized incremental SHAKE absorb state for exactly `Header || P`*, including partial-rate position/buffering. Clone it, absorb `REQUEST || frame(R)`, finalize, and squeeze the request. To advance history, append the canonical completed event to the original unfinalized history state, not to the request-finalized/squeezing clone. Preview snapshots also copy the grammar cursor, event/request indices, and poison status, not merely permutation lanes.

This is literally the same SHAKE input as concatenating the complete key. It is not a theorem equating SHAKE with an ideal oracle, nor an argument that finite-state SHAKE implements information-theoretically injective hashing. The proof's injectivity is the syntax of ideal-oracle inputs; concrete SHAKE is the stated conventional instantiation.

## 3. Scalar and PoW families

### 3.1 Field128

A scalar request reserves `L_f = 128 * NUM_BYTES` iid bytes. For each slot, mask exactly the excess high bits required by the canonical field codec, then call its checked canonical decoder. Return the first accepted representative. If all 128 slots fail, poison this computation. All rejected words, their masked-out bits, and unused reserved words remain in the full raw answer.

For a prime represented by the integers `[0,q)` in b retained bits, put `p_f = q / 2^b`. Then

`Pr[scalar failure] = (1-p_f)^128`,

`Pr[scalar = z] = (1-(1-p_f)^128)/q` for every `z in K`.

Thus success-conditioned output is exactly uniform. Initially b=128, `NUM_BYTES=16`, and the exact failure is `((2^32-22537)/2^128)^128`. Generic canonical field encodings may not be replaced by this interval formula without establishing their actual canonical set.

Use one scalar request/completed event for each installed scalar site. An extension vector is not admitted initially. If later admitted, specify a true field, basis/limb encoding, request atomicity, independent allocations, and compatible original-point/embedding contract; `ExtField` syntax alone is not the proof.

For prescribed z, the full scalar inverse samples the truncated rejection count described in §5.1, uniform rejected canonical failures, all high-bit slack, accepted z, and every unused slot. The full failure inverse for this scalar is direct: sample each of its 128 retained words uniformly from the rejected set and each masked-out high-bit string uniformly. For the initial interval codec the rejected set is `[q,2^128)`.

### 3.2 Public PoW

For `0 < g <= 25`, preserve the canonical nonce range `[0,2^(g+7))` and unsigned canonical LEB128 transport. After the nonce proof event, request a **separate** predicate family of exactly 32 bytes in its own operation/domain. Apply the actual low-bit-first zero predicate. The complete predicate output is appended before a later scalar/shell request. Its raw bytes are not also an allegedly fresh scalar seed/root.

Zero-bit PoW has no nonce/predicate event when that absence is fixed by the installed plan. Do not optionally insert it based on proof data. The verifier checks a selected permitted nonce and its predicate, not honest firstness. Every failed original nonce probe counts toward Q and work even though it is absent from proof transport. Under fresh independent predicate requests, honest PoW exhaustion is `(1-2^-g)^(2^(g+7))`; any use of this law requires that freshness at the actual request keys.

A full predicate inverse conditioned on acceptance fixes its g low bits to zero and samples all remaining bits uniformly. Conditioned on rejection, choose its g-bit tested pattern uniformly among the nonzero patterns and sample remaining bits uniformly. Honest-firstness histories retain these exact full rejected outputs and the winner, not zero-filled placeholders. The original raw output/table is unchanged on the marked branch.

The 12-bit FoldResponse nonce is **not this PoW predicate**. It is a bounded witness-dependent search over shell/response probes. Do not treat the vector selected by that search as an independent uniform vector after conditioning on its winning status.

## 4. Atomic whole-level shell family: layout and exact finite law

### 4.1 Frozen descriptor and physical allocation

Immediately after a FoldResponse nonce, build a checked descriptor for **all** groups at that level before exposing the shell answer. Let `W = sum_g P_g B_g`, ordered first by group, then claim, then block. The request includes each opening method, sampling-carrier dimension, native embedding, shell counts, exact integer-filter descriptor, caps, and claimed length. Preview/live/verifier derive identical descriptors from validated public state. Terminal folding uses the same contract with its actual singleton group and block count.

For coordinate c, define `d_c`, `n1_c`, `n2_c`, `w_c=n1_c+n2_c`, and `J_c=1` without a filter or `J_c=4096` for the existing fixed integer operator predicate. Set mask cap H=128.

At position step i, `m_i=d_c-i`. For m>1 set `b_i=ceil(log2(m_i))`, and use the existing width rule: one byte if `b_i<=8`, two if `b_i<=16`, otherwise four within the installed supported dimension bound. For m=1, width is zero and no raw byte is consumed. The step reserves exactly `H * ell(m_i)` bytes. Decode the first accepted low-b_i masked word within this slot, retaining all discarded high bits and all unused words. If none accepts, the candidate and whole family fail immediately; this failure is not an operator rejection and does not restart at the next candidate.

Apply the ordinary partial Fisher–Yates swap `j=i+a_i`. Reserve w_c further bytes for signs after all position slots, with the existing convention: low bit zero means positive, low bit one means negative, and the first n1 positions get magnitude one. Seven high sign bits per byte remain raw slack. They are not tightly packed sign bits.

Let

`B_c = H * sum_(i=0)^(w_c-1) ell(d_c-i) + w_c`,

`L_shell = sum_c J_c B_c`.

The physical full answer concatenates candidate slots in **attempt-major order**, and within each attempt in canonical coordinate order, omitting coordinates whose public J_c is already exhausted. Within each candidate, position slots are in increasing i, followed by the sign bytes. These rules uniquely determine all offsets from the descriptor, without decoded values.

At each candidate offset, a completed coordinate's allocation is skipped without interpreting it; its bytes are still uniform raw answer bytes. Decode each unfinished coordinate, test the fixed filter only after successful completion, and mark its first accepted polynomial complete. If its last candidate is complete but filtered out, fail with filter exhaustion. Once all coordinates complete, the decoder stops. A streaming SHAKE reader may stop then; all still-unsqueezed tail bytes remain defined by that same finite SHAKE answer. If an unfinished coordinate occurs after skipped bytes, sequential SHAKE must actually advance through those bytes (generate/discard them); skipping is not random access or permission to change offsets. Bounded buffering must never expose bytes beyond L_shell.

Compare/serialize polynomial values, not incidental sampling order. One explicit canonical result codec is the length-d_c coefficient vector with entries in `{0,+1,-1,+2,-2}`. Check counts, distinct support and exact filter. Existing sparse arithmetic can receive an ordered view reconstructed by increasing positions within the magnitude-one class followed by increasing positions within magnitude-two, with signs attached. Trace and coefficient-packing consume their selected carrier and actual native embedding; full-ring scalar-extension remains unsupported in this ordinary profile.

### 4.2 Exact candidate, coordinate, and family probabilities

For m_i>1 put

`p_i=m_i/2^b_i`, `f_i=(1-p_i)^H`, `sigma_i=1-f_i`.

For m_i=1 put `f_i=0`, `sigma_i=1`. Candidate completion is

`rho_c = product_i sigma_i`.

Its complete polynomial is uniform on

`S_c`, with `M_c = binom(d_c,w_c) binom(w_c,n1_c) 2^w_c`.

This follows because each successful accepted digit is uniform independently of that slot's success, each distinct ordered Fisher–Yates target tuple has the same mass, signs are independent, and each polynomial has exactly `n1_c! n2_c!` ordered representations. More precisely a specified complete polynomial has **unconditional** candidate mass `rho_c/M_c`.

Let `A_c` be the subset passing the installed fixed integer predicate (`A_c=S_c` without filtering), and `a_c=|A_c|/M_c>0`. Put

`x_c = rho_c (1-a_c)`.

One attempted candidate is fatal with probability `1-rho_c`, complete-and-filtered-out with probability x_c, and complete-and-accepted with probability `rho_c a_c`. The exact coordinate success probability is

`s_c = rho_c a_c * sum_(r=0)^(J_c-1) x_c^r = rho_c a_c * (1-x_c^J_c)/(1-x_c)`.

Exact filter-exhaustion mass is `x_c^J_c`. Exact fatal-mask mass is

`(1-rho_c) * sum_(r=0)^(J_c-1) x_c^r`.

These three masses sum to one. More specifically, fatal failure at attempt r and first failing range i has mass

`x_c^r * (product_(h<i) sigma_h) * f_i`.

For any accepted target polynomial z in A_c,

`Pr[coordinate=z] = (rho_c/M_c) * sum_(r=0)^(J_c-1) x_c^r = s_c/|A_c|`,

`Pr[r rejected candidates | success,z] = x_c^r * rho_c a_c / s_c`.

In particular, once position masks have a fatal cap, do **not** replace this last formula by the uncapped `(1-a_c)^r a_c / (1-(1-a_c)^J_c)`. The latter is the rho=1 case. Calling `(1-a_c)^J_c` an upper bound is safe, since `x_c<=1-a_c`; calling it the exact capped exhaustion probability is not.

All coordinate allocations are disjoint iid raw bytes. Think of independently running their latent local decoders on their allocations. The attempt-major family decoder returns the tuple exactly when **every** local decoder succeeds, regardless of which failure would have stopped an unsuccessful interleaving first. Hence

`s_family = product_c s_c`,

`Pr[complete tuple=z] = product_c (s_c/|A_c|)` for z in `product_c A_c`.

Conditioned on family success, the complete tuple is the independent uniform product of the accepted polynomial sets. Actual family failure mass is `1-product_c s_c`. This product law is a law of a fresh indexed answer **before** witness-dependent response/nonce selection, not a claim about the vector selected by the honest grind.

A convenient upper bound is

`Pr[family failure] <= sum_c J_c sum_i f_i + sum_c (1-a_c)^J_c`

`<= sum_c J_c w_c 2^-H + sum_c (1-a_c)^J_c`.

The first sum is conservative over every reserved attempt, including attempts never used. For a failure-bearing family, a successful tuple has the smaller unconditional mass above; separating failures and using success-conditioned inverse-cardinality bounds is conservative.

## 5. Exact FULL inverse kernels, including failure/slack semantics

### 5.1 A finite masked range

For a prescribed accepted digit a<m, define p=m/2^b and sigma=1-(1-p)^H. Sample

`Pr[R=r | success,a] = (1-p)^r p / sigma`, `0<=r<H`.

For each rejection draw its low word uniformly in `[m,2^b)` and its `8 ell-b` high bits independently uniformly. Set the accepted low word to a and sample its high bits uniformly. Fill every remaining reserved word iid uniformly. This is exactly the original uniform slot conditional on returning a. For powers of two R=0; for m=1 the slot is empty and the digit is zero.

For the slot failure inverse, all H low words are independently uniform in the rejected interval, all high bits independent uniform. This outcome has mass `(1-p)^H`. A zero-probability failure, such as a power-of-two range, is not a permitted conditioning target. A requested failure at the first failed step of a candidate additionally conditions earlier steps on success using their truncated laws, leaves later steps and all unused signs uniform, and keeps the displayed first-failure weights in §4.2.

The scalar kernel in §3.1 is the same construction for m=q with the actual canonical-representative set. No SHAKE seed preimage or state inversion is involved.

### 5.2 A complete candidate with prescribed polynomial

For z in S_c, choose a uniform permutation of its magnitude-one support and independently a uniform permutation of its magnitude-two support; concatenate these with their signs. Replay the ordinary Fisher–Yates permutation (or its equivalent sparse map). At each step locate the chosen support position in the remaining suffix, obtaining its unique accepted digit. Apply §5.1 to its finite reserved slot. Set each sign's low bit from z and sample all seven high bits uniformly.

The uniform choice over the `n1! n2!` representations plus the product of the exact truncated range kernels is the original **entire candidate-allocation law** conditional on successful completion and polynomial z. This includes candidate-local unused mask words, not just the consumed prefix.

### 5.3 Filtered coordinate with prescribed accepted polynomial

Use an auxiliary forward **coordinate** decoder on fresh complete L_c=`J_c B_c` raw allocations until that coordinate succeeds. Retain the successful trial's stopping index and the *whole raw allocations of all its earlier complete filtered-out candidates*. None of those earlier candidates has a fatal mask failure, because the retained trial is successful. Replace only the last accepted candidate's entire B_c allocation with the exact kernel of §5.2 for the prescribed z in A_c. Fill all later never-used candidate allocations independently uniformly (or retain their already sampled independent bytes).

Proof: the stopping index has the exact law of §4.2, independently of the final accepted z. Earlier candidates have their actual full laws conditional on successful completion and failing the fixed predicate, not the unconditional candidate law. The final candidate law conditional on z is §5.2. Later allocations are independent because they were never consulted. The replacement therefore has exactly the raw-uniform family law conditional on coordinate output z. It needs neither enumeration of A_c nor its exact cardinality to execute.

Complete candidates from **discarded auxiliary unsuccessful trials**, including trials with cap failures, are private sampler work. They are charged but are not spliced into the output answer. Inserting a fatal-cap prefix into an allegedly successful answer would be incorrect. The output answer includes every reserved byte, including slack before and after its first acceptance.

Generate the conditioned raw allocation independently for each coordinate and interleave the allocations into the one attempt-major finite answer. This is the exact full level-answer law conditional on any specified complete successful tuple. Physical contiguity is not probabilistic dependence. The marked original branch instead keeps its literal original entire level answer, including its original unused tail, even when a newly sampled answer would decode to the same tuple.

For a coordinate forward cost bounded by L_c plus polynomial decoder/predicate work, a conservative successful inverse bound is that cost divided by s_c, plus one conditional candidate and serialization. Since `s_c >= rho_c a_c`, a justified inverse-polynomial lower bound on a_c and on rho_c suffices. Initially supported d/weights and H=128 give an explicit rho_c via §4.2; the unfiltered baseline has a_c=1. A selected filtered route needs its actual accepted-fraction certificate. Source comments or raw-shell entropy alone are not authentication of that certificate.

Do not run complete W-coordinate trials until the entire level succeeds: the resulting `1/product_c s_c` cost can be exponential in W even when each coordinate has a constant acceptance probability. Per-coordinate generation has a sum of `1/s_c` costs and supplies the same conditional product law.

### 5.4 What the failure cases require, and do not require

For any finite answer X and deterministic decoder D, the exact inverse law for an outcome y of positive mass is

`Pr[X=x | D(X)=y] = 2^(-8L) * 1[D(x)=y] / Pr[D(X)=y]`.

For a failed shell family y=bottom, its denominator is `1-product_c s_c`; if a detailed first-failure owner is exposed, replace this event by that exact attempt-major first-failure event. The local cap and filter-prefix laws above specify the corresponding partition. Unread bytes remain iid only where the specified outcome/first-failure event does not constrain them. For example, unconditionally rejected filter candidates cannot be replaced by iid raw candidates, and a failure observed only at the end is not a licence to resample previously exposed bytes.

Uniform full-answer rejection until `D(X)=bottom` is an exact generic failure inverse, but its expected work is proportional to `1/(1-product_c s_c)` and **is not claimed polynomial**. Likewise sampling a rare predicate complement by rejection can be expensive. There is no general unconditional polynomial-time inverse for every rare filtered failure promised here.

The retained successful-shell game does not need such an inverse: on a cap/filter failure it preserves the original full answer and original failure, charges that failure at its original exposure, and does not start a successful sibling star. Its conditional inverses are only the successful kernels in §§5.1–5.3. Thus full raw failure semantics are specified and covered, without hiding a potentially exponential failure-conditioned subroutine in the claimed polynomial extractor. An interface that additionally demands fresh programmed failure targets must separately supply and price them; it is not the interface admitted by this text.

## 6. Lazy finite tails, the marked-original game, and original-output preservation

The conservative formal interface remains a finite FULL-answer oracle. At an original fresh request it samples, returns, and records all L bytes. An adversary querying that key can inspect all L bytes, including the unused tail. It is not restricted to the honest decoder's reads. All future repeats of that key replay those same bytes.

An honest concrete reader may avoid squeezing the tail after the decoder stops, because its v8 public history contains only the completed decoded result. The formal oracle can materialize that tail before running the honest decoder; this has exactly the same joint distribution as deferred iid tail generation at previously unobserved positions. This coupling does **not** defer or resample bytes already exposed by an adversarial prequery, return, clone, or read. For concrete SHAKE the tail is deterministic from the literal key; for the ideal oracle it is sampled once and retained. These are distinct descriptions of the stated instantiation and its model, not an information-theoretic claim about concrete SHAKE.

For the finite game and time ledger, charge a full L-byte output/materialization cost for **every** fresh original or private finite-family entry, plus actual copies, stored tables, conditional draws, hashing, replay, predicates, and source decoding/comparisons. If the real honest decoder computes fewer bytes, this is a valid conservative upper bound, not a claimed lower bound on honest execution time. The mathematical simulator that presents a full oracle answer can simply pay O(L) and materialize it. No unpriced full-length output or O(1)-cost gigabyte return is used. Lazy storage is optional and cannot improve the admitted bound without a separately proved model/cost refinement.

At a retained marked fresh family:

1. Keep the original coins, legitimate pre-request table, literal original full root answer, and original returned record/output. Failed roots stay failures rather than acquiring successful siblings.
2. Generate successful siblings using the exact full conditional kernel above with the legitimate prescribed decoded coordinates. All other coordinates being held fixed still get their *conditional raw* laws, not original raw strings by default.
3. Give each sibling its own table extending the pre-request table and its own full root answer. Absent future entries are fresh in that sibling world. Do not copy the original continuation's future table entries to force source/header matches.
4. Preserve any already queried prefix entry literally. If an allegedly fresh prescribed target is incompatible with retained raw bytes, the fibre is empty; do not overwrite it.
5. Return the original branch's original index/output/record to the enclosing marked map. Private probing and collisions do not replace it by a successful sibling. A certified collision is a separate legitimate output.

The product raw law is invoked at the unconditioned fresh exposure before ordinary acceptance, not after forcing a winning response nonce or a successful ancestor. Source-probe renewal and causal fixed-source arguments must use their actual reference continuation kernels; this transcript construction does not authorize arbitrary success conditioning.

## 7. Descendant decoded-owner theorem and old-query charges

Fix a fresh request k=`Key(P,R)` and an older, already queried, proper descendant key k'. Parse k' using §2. Its prefix includes a `COMPLETED` event with stored request R and a uniquely reconstructible owning prefix. It can be a descendant of k only if that reconstruction gives exactly k. In that case the event contains **one specific complete decoded value z** for k. Message contents independent of z do not help omit it: all later keys carry the completed event. The parser must not accept a child with a root-sized short restarted context, duplicate owner, or merely a group fragment of an atomic level result.

Consequently, before arbitrary ordinary-acceptance conditioning, compatibility with this specified older descendant implies `D(X)=z`. For a scalar the successful value mass is `(1-delta_f)/q <= 1/q`. For an atomic shell level it is

`Pr[D(X)=z] = product_c s_c / product_c |A_c| <= 1/product_c |A_c|`.

A bound `1/|A_c|` for any fixed coordinate is also conservative. Use the full product only when the older key really includes the whole atomic result, as this grammar requires; no such claim applies to the current sequential v7 group-root interface. For a fresh full 32-byte predicate the specified-output mass is `2^-256`; after deliberately conditioning on its public g-bit test this would instead be `2^-(256-g)`. The ordinary unconditioned ledger must not borrow the conditioned measure while continuing to use the former mass.

Failures have their exact atoms, but have no successful descendants in a poisoned computation. Count them separately rather than claiming inverse-cardinality entropy for bottom. An already exposed raw family replays with its actual conditional law, possibly a point mass of one; this is not a fresh-root application.

This theorem is **syntactic**: change z while holding the later public messages fixed and the literal descendant key bytes change. It requires no property of the old Blake2b continuation and no raw-history injectivity. Two different full raw answers with the same z intentionally yield the same completed event; the old-key proof uses decoded mass, not `2^(-8L)` raw mass. The full raw answers remain distinct table values for replay/conditional sampling.

After this ownership proof, the ordinary pair ledger can charge `sum_i (i-1) gamma_i`, at most `binom(Q,2) gamma_max` for the declared original-query convention (or the consistent Q+1 convention when output verification adds the extra exposure). Q includes all original adversarial prequeries, prover/failed nonce queries, verifier/output-evaluation queries, and allowed initialization entries at **atomic finite-request granularity**. Private counterfactual entries do not inflate original Q but do cost extraction time. Existing logical GrindingPlan multiplicities are not an adversarial ROM-query cap. Separate cuts may be summed safely; a single globally deduplicated charge across cuts still needs its separate unique-owner/joint-ledger proof.

## 8. Polynomial costs and assumptions retained

Write L_k for the full raw length of request k, K_k for its literal key length, and C_k for its non-hash decode/predicate work. A conservative ordinary replay implementation costs

`O(sum_k (K_k + L_k + C_k))`,

plus proof/source table storage/copies and the actual native arithmetic. Exact incremental prefix caching can reduce repeated prefix absorption, but transcript snapshots, event/request encoding, output squeezing, and grammar validation still have real costs. It does not change the ideal literal keys.

For a successful prescribed shell bundle, the per-coordinate inverse construction has expected work bounded by a sum of coordinate forward costs divided by s_c, plus conditional-candidate work, complete answer assembly and copies. Because H=128 and J<=4096 are fixed public caps and installed dimensions/weights/event lengths are polynomially bounded, this is polynomial provided selected accepted fractions have the stated inverse-polynomial lower bounds. Without that gate one must report the actual larger cost; fixed finite caps and nonempty support alone are insufficient.

The full extractor also pays its existing complete deeper-suffix recurrences, private source-probe trials, explicit grids, comparison instances, collision outputs, and all finite raw conditional kernels. Use the explicit strict total-work cutoff and its proved loss. A declaration of polynomial local sampling does not by itself prove polynomial whole-protocol recursion. Local cap failures are charged at actual original exposures with the bounds in §§3–4; they are not multiplied by privately forced successful ancestors.

All extracted source comparisons keep their original actual mixed-norm Module-SIS radii and complete reduction runtime. No `tau/2^128` advantage conversion follows from a modeled lattice ROP128 table. No full per-segment PCS proofs, commitment/source homomorphism, source-profile weakening, new shortness assumption, or extra tight128 target is introduced. The transcript compiler is one contract, not numerical AQRA admission.

---

# Implementation change map grounded in the ordinary APIs

The following is the precise proposed migration surface. New types/methods are specifications for later implementation; none is asserted to exist now. Existing arithmetic values and codecs should be reused where their boundaries match this contract.

## A. Public transcript/channel boundary

**Current evidence.** `akita/crates/akita-transcript/src/proof_stream.rs:24–28,171–242` uses version 7, aliases `ProverChannel` to Spongefish `ProverState`, and constructs channels from framed session/instance. Its `ProtocolSiteId` is a 32-byte fixed layout at `:84–168`; context hooks at `:663–687` are logging-only. `proof_stream/channel.rs:21–91` has `context`, unlabelled `public`/`exchange`, and unlabelled `field_challenge`; the latter directly invokes the old public sponge sampler. `proof_stream/verifier.rs:10–97` privately wraps the verifier state with sticky invalidation but still exposes a generic `verifier_message` squeeze.

**Required public types and semantics.**

- Replace the ordinary public `ProverChannel` alias and `VerifierChannel` public-randomness path with owner wrappers containing proof transport/cursor, installed grammar cursor, event/request indices, append-only literal history/SHAKE absorb prefix, and sticky invalidation. Prover private RNG may remain private; it is not the public indexed oracle.
- Constructors accept a validated installed-context object carrying the exact registered installation and augmented ordinary context, rather than accepting arbitrary bytes as sufficient admission. Constructor output binds version 8 and selected profile. No backend feature can silently choose v7 challenge generation in an admitted v8 run.
- Define checked `IndexedRequest` with operation, ordinal, owner/site, complete descriptor, and raw length; `CompletedResult` has Scalar, Predicate32, and complete ShellLevel results. Dispatch exhaustively on all admitted result kinds. Decode failures are errors/poison, not a result that a caller may ignore and continue past.
- Define an internal `FiniteAnswer`/reader capability with declared length, bounded sequential reads and `skip_exact`, and no facility for resetting to a new coordinate seed. The formal test/research backend returns a full finite string at one query; concrete production uses the literal-key SHAKE reader. Neither public type is an oracle-assumption shortcut.
- `field_challenge_at<F>(site)` replaces the unlabelled field challenge at ordinary callsites. Scalar descriptor/length and the pending site's expected grammar are checked, then the completed scalar event is appended **before return**. `ext_challenge` must call this labelled operation for its limbs; the initial profile admits degree one only.
- Public/proof operations take a validated site or consume the installed next-event site. Every logical event is recorded even when transcript logging is disabled. The existing `FieldAtom`, `ExtensionAtom`, byte/payload, U128, and NonceAtom codecs remain the proof-transport encodings, but unlabelled codec helpers become internal within a checked event transaction. Chunking is internal to that transaction, not another history convention.
- Existing `send_*`, `receive_*`, `public_*`, `exchange_extension_group`, sumcheck claim/body helpers, nonce receipt/commit, and finalization route through these wrappers. Do not leave direct `state.public_message`, `prover_message`, or generic `verifier_message` public-randomness bypasses in admitted ordinary code. Logging observes the canonical events; it does not define or cryptographically implement them.
- Channel `finish`/verifier EOF require no pending transaction, no poison, complete installed event grammar and complete existing proof/GrindingPlan consumption.

The finite request/result descriptors must be available at the transcript/challenges boundary without creating a transcript-to-types dependency cycle. Put the byte/typed descriptor contract in the lowest shared transcript/challenges-facing layer; method-aware construction/embedding remains in `akita-types`, as it is now. This is an interface-location requirement, not a request to move source profiles or commitments.

## B. Field and grinding functions

**Current evidence.** `proof_stream/sampling.rs:39–68` runs unbounded canonical rejection (with the Keccak retry-count special case), and `:77–100` draws through `duplex_sponge_state`/`sponge_mut`. `proof_stream.rs:595–619` supplies limb IDs to diagnostics but then calls unlabelled `field_challenge`. `:941–995` previews/searches/commits predicate squeezes without a keyed request descriptor. `grinding.rs:5–52` is the real low-bit predicate and nonce-range policy. The field's checked LE128 representative is `jolt/crates/jolt-field/src/solinas/fp128.rs:1126–1157`.

**Required changes.**

- Replace ordinary canonical-squeeze entry points with the field128 finite-family operation. Remove the old ordinary unbounded loop and backend-specific absorbed retry-count behavior from the v8 challenge route; failures and all slack follow §3.1.
- Preserve `grinding_predicate_accepts`, nonce width/range policy, and canonical NonceAtom codec. Give preview/search/commit/receive predicate functions the full scheduled site/request context, not merely `(state, nonce)`. They form the same nonce event and predicate key in preview/live/verifier. Return Result where descriptor, resource, or reader errors can occur; no infallible generic squeeze is the new API.
- `ProverGrinding`/`VerifierGrinding` in `akita-types/src/transcript_grinding/replay.rs` keep plan checks, canonical nonce-byte accounting, poison behavior, and EOF. Their PoW functions call the separate predicate operation and append its full 32-byte completion before the protected scalar request. The zero-bit path consumes its prescribed logical plan event but creates no nonce/predicate history event.
- No public-predicate raw block is reused for a scalar root. Search failure probes remain original oracle work. A preview does not append its events to the live channel or advance the live plan cursor.

## C. Whole-level descriptor, decoder, and method dispatch

**Current evidence.** `akita-challenges/src/fold_draw.rs:53–134` requires `FoldDraw::absorb_and_squeeze -> [u8;32]`, builds a group label/payload, then calls `sample_indexed_challenges_from_seed`. Preview/prover/verifier adapters at `:160–249` all implement that seeded group boundary. `sampler/xof.rs:9–37,80–229` implements `SHAKE256(seed || LE64(coordinate))`, refill buffering, and unbounded masked range loops. `sampler/signed_sparse.rs:36–58` uses Fisher–Yates positions and one byte per sign. `sampler/position_sample.rs:160–205` implements the same partial permutation in sparse and stack modes. `sampler/mod.rs:25–28,137–163` applies the exact integer predicate with 4096 complete candidates.

**Required public replacement.**

- Replace group-root `FoldDraw`/`PreviewFoldDraw`/`ProverFoldDraw`/`VerifierFoldDraw` APIs in the admitted ordinary route by one `draw_shell_level(&ShellLevelDescriptor) -> Result<CanonicalShellLevel, AkitaError>` request on the channel/preview. It performs the complete §4 transition and appends one whole-level completion, not one completion per group.
- Add a checked public method-aware `plan_group_fold_challenges` returning a group descriptor without oracle work, and `plan_shell_level` aggregating every group/claim/block into one frozen descriptor. These are constructed from already validated public params/layout, not proof-selected tuples.
- Split current `draw_group_fold_challenges` into descriptor planning and a pure `group_fold_challenges_from_level` consumption/embedding operation. It verifies owner, group slice, counts and method and returns the existing `GroupFoldChallenges` algebraic family, with no new oracle draw. The old ordinary drawing function and seeded reexports do not remain a v8 compatibility fallback.
- Generalize the reusable position/signed-sparse arithmetic to consume a bounded range/sign source that implements the specified fixed slots and Result-returning cap failure. Keep Fisher–Yates and exact sign convention; do not accidentally keep variable consumption inside a reserved slot or concatenate signs as bits. Reuse the validated integer op-norm policy/table; expose enough of that existing predicate for the common forward/inverse decoder rather than writing a floating approximation or duplicating it.
- `CanonicalShellLevel` contains the full owned polynomial tuple and group views. Its decoder, result codec, inverse kernel, and preview/live paths share one descriptor/layout implementation. Raw offsets do not depend on whether an earlier coordinate completed.

**Consuming dispatch has been checked.** `akita-types/src/proof/fold_challenges.rs:41–121` explicitly routes EvaluationTrace through the selected L2 operator predicate, SubringCoefficientPacking through its validated challenge subring and `CoefficientPackingChallenges::new` ambient embedding, and rejects `FullRingScalarExtension` on the ordinary path. Preserve all three branches in planning and consumption. An atomic bundle must not flatten away the method or sample ambient-A instead of the declared packing carrier. `OpeningFamily::ambient_a` at `:26–35` is the real arithmetic consumer view. Initial E=K does not authorize admitting the separate D1/full-ring compiler through a catch-all branch.

## D. Nonterminal preview, live replay, and verifier consumption

**Current evidence.** `akita-prover/src/protocol/fold_grind.rs:204–223` searches the first jointly accepted nonce. At `:251–310` it previews a group and immediately performs a witness-dependent probe, returning early before later group draws if that probe fails. At `:319–350` it separately repeats group draws live and compares their results. `akita-verifier/src/fold/challenges.rs:22–37` likewise draws and records groups sequentially. `fold/mod.rs:81–99` reads FoldResponse, obtains those groups, assembles the relation, then receives successor before ring-switch/strong challenges.

**Required migrated transaction.**

1. Before nonce search, plan all public group descriptors and their one level descriptor. This does not expose randomness or alter source/profile selection.
2. For each candidate nonce, clone the public v8 history/grammar state with that canonical nonce event. Execute **one complete atomic shell-level draw/decode** before calling any witness-dependent group probe. If all groups' actual response predicates accept, retain the complete canonical bundle and accepted handles. Witness-dependent rejection may stop further arithmetic probes, but it cannot turn the earlier atomic oracle query into a partial group request.
3. Commit the selected canonical nonce live. Execute the same one atomic draw and compare its canonical tuple/descriptor with the retained preview bundle. Feed its group views through the pure method-aware consumer into existing backend `ValidatedFoldProbePlan`/fold arithmetic. Do not draw another oracle answer for a group view.
4. In the verifier, after reading the nonce, build the same public descriptor, draw/decode exactly one bundle, and convert its group views using the same method dispatch. Append completion before receiving the successor. Existing relation assembly and both successor variants remain in their current order.
5. Consume the existing per-group logical GrindingPlan records (or a deliberately versioned equivalent) from this complete bundle on both sides. `record_fold_challenges` currently consumes `coordinate_count+1` at `transcript_grinding/replay.rs:407–427,524–546`; that may remain a schedule audit convention, but is not the number of atomic ideal-RO requests and must not be used as Q. Validate/consume the whole bundle transaction before allowing successor/strong events.

Source/profile/commitment and backend arithmetic ownership stay where they are. No source extraction, per-segment PCS proof, homomorphic source sum, response-norm suppression, or synthetic placeholder handle is part of this migration.

## E. Terminal branch and shared sumcheck/public-message plumbing

**Current evidence.** Terminal prover `fold_grind.rs:109–163` directly uses the group FoldDraw APIs, including preview/live equality. Terminal verifier `fold/terminal.rs:119–151` draws its shell and records it before receiving bounded Z. These are separate callsites, not automatically migrated by the nonterminal group helper. `akita-sumcheck/src/proof_stream.rs:81–105,143–165` exposes concrete ordinary channels and directly emits/absorbs claims; other helpers in that file use the same channel types. `GrindingReplay::ext_challenge_at` is the shared post-grind scalar route at `transcript_grinding/replay.rs:170–190`.

**Required changes.** Terminal uses a one-group `ShellLevelDescriptor` with its actual terminal filter/geometry, executes the same atomic request in preview/live/verifier, appends it before Z, and preserves all existing bounded response/ring/trace checks. Sumcheck claim/body and ordinary public/proof group operations migrate to the canonical event transactions, and their round challenges use the labelled scalar request. Update public reexports and all actual callsites of the replaced channel/fold interfaces; do not leave legacy group roots reachable from an admitted v8 route. The generic arithmetic traits and payload atoms need not change mathematically.

## F. Later implementation evidence required (not run here)

The main implementation owner should exercise, after all integration changes land:

- literal-key/streaming-prefix equivalence and preview/live/verifier event/key equality, including changed logging/backend features;
- checked error/poison paths, version rejection, EOF/plan completion, malformed length/offset/site/owner inputs, and descriptor mismatches;
- exact small-family exhaustive forward/inverse checks with tiny H/J, including a fatal mask, earlier rejected filter candidates, final accepted candidate, untouched signs/slack/tail, and attempt-major heterogeneous J/widths;
- canonical-polynomial equality versus position-permutation inequality, including both trace and packing embedding consumers;
- nonterminal early group-probe rejection and terminal singleton consumption, proving only one atomic level request and complete-result ownership before successor/Z;
- original-table/marked-answer preservation, valid fresh absent-future tables, rejection of old-key incompatible programmed targets, and full-length cost accounting for all original/private outputs;
- use of the actual selected filter and accepted-fraction certificate, without claiming numerical schedule admission from local sampler checks.

These are suggested acceptance checks, not reported verification results. The mathematical contract above is independently established; executable v8 conformance and whole AQRA admission remain unclaimed.
