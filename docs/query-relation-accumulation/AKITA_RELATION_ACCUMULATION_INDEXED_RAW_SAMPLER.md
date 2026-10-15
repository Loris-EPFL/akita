# Candidate1: indexed FULL raw-answer sampler and prefix-grammar certificate

## 0. Result and scope

**The retained ordinary implementation does not currently supply the exact indexed full-answer interface required by the lighter extraction note.** There are two independent, source-level obstructions:

1. Ordinary shell coordinates are deterministic expansions of one **32-byte group seed** through concrete SHAKE256. They are not independent ideal raw coordinate tapes conditional on that seed. An inverse decoder for iid tapes does not invert this seeded map, and its joint support cannot equal the large product shell support used by the coordinate-wise game.
2. Ordinary stage/site records are diagnostic, not absorbed. More importantly, the default pinned Spongefish Blake2b backend's post-squeeze continuation binds the old chaining value and **number of bytes read**, not the squeezed answer contents. Consequently a programmed root answer can change while a later primitive query key remains literally unchanged. This violates the descendant-key ownership premise needed for the stated old-query point-mass argument, even if Blake2b itself is idealized as a random oracle.

These observations do **not** prove an attack on ordinary Akita, nor refute the explicitly conditional research theorem. They refute identifying that theorem's indexed interface with the current concrete ordinary transcript merely from sampler comments, entropy counts, or schedule replay. A different proof of the existing compressed/chained construction could conceivably be given, but is not supplied by these artifacts. There is no claim that exhaustive finite-seed conditional sampling is mathematically nonexistent: it is a preimage/rejection computation with potentially exponential cost, expressly outside the requested method.

I give below an exact constructive **raw-tape decoder** lemma, including all masked bits, rejected candidates, sign-byte slack, operator-filter attempts, prefetched bytes and remaining tails. It becomes a complete retained-shell sampler once the protocol obtains its bytes directly from an indexed ideal raw-tape family. I also specify a finite-answer version and a small, explicit API/grammar cutover; this is a transcript/profile change, not a proof-only reinterpretation of existing bytes. No new lattice assumption, inverse-shortness assumption, concrete-hash knowledge axiom, or frozen-future-answer rule is used.

Research was static source inspection. No repository files were modified; no builds, tests, linters, formatters, benchmarks, or protocol executions were run. The only deliverable write is this scratch artifact. No unauthenticated expanded local Akita theorem is used as a proof of a sampler fact; the probability arguments below are derived directly.

## 1. Source map and actual consuming boundaries

Paths below are workspace-relative unless explicitly marked as the pinned dependency checkout.

- `akita/crates/akita-transcript/src/proof_stream/sampling.rs:9–66`: dimensions/query-ceiling predicate and canonical masked rejection loop. Each candidate squeezes exactly `F::NUM_BYTES`; upper excess bits in its last byte are cleared; canonical decoding rejects candidates outside the field. Under `transcript-keccak` an attempts counter is checked as `u32` and its LE32 encoding is absorbed on successful completion.
- `akita/crates/akita-transcript/src/proof_stream.rs:188–238`: protocol v7 backend-specific identifier; session and instance each use `LE64(length) || bytes`; construction absorbs protocol, framed session and framed instance.
- Same file `:595–619`: extension challenges consist of `E::DEGREE` successive base-field draws and `E::from_base_slice`. Different limb IDs are constructed, but `:663–687` shows `prover_context` / `verifier_context` only log metadata. Neither absorbs those IDs, in either logging mode.
- Same file `:699–739`: preview/live fold roots absorb a group payload then squeeze a 32-byte seed. `:941–995` contains clone-based PoW previews, bounded nonce search, live nonce commitment and verifier receipt.
- `akita/crates/akita-transcript/src/proof_stream/nonce.rs:25–89`: canonical unsigned LEB128, transactional decoding, overflow/redundant terminal-zero rejection. Schedule-dependent ranges are checked in replay, not in this atom.
- `akita/crates/akita-challenges/src/fold_draw.rs:34–51,64–134`: group payload includes a fixed domain, group index, live blocks, claim count, total count, ring dimension, shell policy, optional packing and fixed operator-filter domain. It squeezes one seed and calls `sample_indexed_challenges_from_seed`.
- `akita/crates/akita-challenges/src/sampler/xof.rs:9–37,80–229`: every coordinate is **SHAKE256(seed[32] || LE64(coordinate))**; concrete Keccak-f1600 is called. Cursor refills 128 bytes; range sampling masks 1, 2 or 4 little-endian bytes and rejects a value >= modulus. Modulus one consumes no byte.
- `akita/crates/akita-challenges/src/sampler/position_sample.rs:160–205`: sparse and dense partial Fisher–Yates implement the same sequential choices `j = i + U[0,d-i)`.
- `akita/crates/akita-challenges/src/sampler/signed_sparse.rs:36–61`: first `n1` positions receive magnitude one, next `n2` magnitude two. **One full byte per sign is consumed**, only bit zero is used; this is not forty tightly packed sign bits.
- `akita/crates/akita-challenges/src/sampler/mod.rs:27–28,137–163`: fixed operator predicate, at most **4096 complete sparse candidates** per coordinate, then an error. Position-mask rejection within each candidate has no such cap. The filter depends on the ring challenge, not on a witness response.
- Producer/consumer routing really agrees: `akita-types/src/proof/fold_challenges.rs:41–121` dispatches trace or coefficient packing; packing samples in its challenge subring and embeds into A. Ordinary `FullRingScalarExtension` is explicitly rejected, not silently routed through the ordinary seed sampler. `akita-verifier/src/fold/challenges.rs:12–38` draws every group and records `P_g B_g` coordinates; `fold/mod.rs:81–99` reads the response nonce, draws shell groups, receives successor, then enters the strong checks. `fold/terminal.rs:119–144` likewise draws the terminal shell before receiving its bounded response.
- `akita-prover/src/protocol/fold_grind.rs:204–223,225–361`: honest prover tries the first nonce for which all witness-dependent response probes accept, then replays that nonce and every group through the live transcript. Earlier nonce candidates are not proof messages, but their oracle computations belong to the original computation/query ledger.
- `akita-types/src/transcript_grinding/replay.rs:18–87,327–451,484–576`: cursor checks exact order/sites/multiplicity, nonce ranges, full plan completion and verifier EOF. These are real runtime checks, but are not an indexed-RO fork API.
- `akita-config/src/transcript_binding.rs:38–74` and `akita-types/src/instance_descriptor/mod.rs:55–100,156–282`: actual algebra, setup identity, schedule, grinding plan and batch layout are bound. Large setup/schedule/layout identities use descriptor digests, not literal full structures.

The path named `akita-types/src/proof_stream/` in the assignment is not present in this checkout: ordinary proof-stream codecs are in **akita-transcript/src/proof_stream/**; their typed scheduling consumers are **akita-types/src/transcript_grinding/replay.rs** and **akita-types/src/proof/**. I traced these real locations rather than assuming the missing path implements the contract.

The `native` / `native_strong` D1 inner-H1610 and outer-SHAKE paths are separate APIs, not the ordinary backend selected here. In particular the packed forty-sign-bit native shell comments must not be imported as the ordinary signed-sparse byte law. No ChaCha expansion occurs on the inspected ordinary public challenge path. The pinned Spongefish prover RNG is private prover randomness; public `verifier_message` squeezes `duplex_sponge_state` directly and the ordinary field sampler also squeezes that public state directly.

## 2. Concrete primitives versus the protocol ROM

Three experiments must be kept distinct.

1. **Actual executable:** Blake2b/Keccak/SHAKE are deterministic algorithms. There is no literal random oracle table to program. Source code by itself proves no exact iid law for their outputs.
2. **Primitive-level idealization of the existing executable:** replace a named digest or XOF primitive by a random oracle, retaining all concrete input encodings, state transitions, seed routing and output-block reuse. Any compressed-state collisions, direct primitive queries, shared blocks and prequeries remain part of this experiment. Its table keys are not automatically the theorem's full-prefix keys.
3. **Protocol indexed-ROM experiment:** an oracle is keyed by a canonical, injectively encoded full context/history/request, and returns a specified independent raw-answer family. Fresh answers have their declared law, old keys replay exactly, and later-stage keys explicitly contain the owned earlier results. The lighter note assumes this experiment. A direct SHAKE call over those literal keys is a conventional concrete instantiation, not an unconditional equivalence theorem for concrete SHAKE.

Replacing `SHAKE(seed || index)` by a programmable independent tape **without naming that SHAKE call as an additional ideal interface** is a change from (1)/(2) to (3), not an inverse decoder. Conversely, explicitly idealizing that SHAKE call gives an exact iid tape at each fresh `(seed,index)` key without a seed preimage. This is a legitimate *different, explicitly stated primitive-level ROM model*, but still has only seed/index ownership, seed collisions, guessed old keys, and the ordinary chained transcript problem below. It does not establish the required full-prefix grammar for free.

The claimed lighter proof is already conditional on such an implementation/compiler argument (`AKITA_RELATION_ACCUMULATION_LIGHT_ORDINARY_EXTRACTION.md:35,55–59,109–131,341–344`; sampler addendum `:27–33`). This certificate resolves the current code fit negatively and supplies a constructive route for an explicit cutover, rather than changing those conditions into assumptions about a concrete hash.

## 3. Why the current seed interface cannot supply product-coordinate forks

At a fixed ordinary group header, let `s` be the 256-bit seed and let `F_j(s)` be coordinate j including its fixed filtering rule. The joint decoded answer is the deterministic tuple

`F(s) = (F_0(s), ..., F_(W-1)(s))`.

Even granting a uniform seed, its support has at most `2^256` tuples. It is not the full independent product of the admitted ring shell families. For example production d128 has shell cardinality

`M = binom(128,31) * 2^31`.

The exact 2-adic valuation of `binom(128,31)` is seven: use `binom(128,31) = (128/31) binom(127,30)` and `binom(127,30)` odd. The binomial has an odd factor greater than one, so M does not divide `2^256`. Thus a total deterministic map from all 256-bit seeds cannot even produce an exactly uniform single-coordinate law on that full unrestricted shell. More fundamentally, the admitted shell policy records floor(log2 M)=129 (`akita-challenges/src/config.rs:163–170`), so two independent full-shell coordinates have more than `2^258` possible tuples, already beyond the seed support. The root Main56 group has 56 such coordinates; this is not a marginal detail.

Given prescribed ring values, a seed-level FULL-answer conditional sampler would need the law of

`s | {F_j(s)=c_j for every prescribed j}`,

plus any other raw backend output bytes. A coordinate change with all other decoded coordinates fixed can have empty support. Where nonempty, finding/sampling that fibre is a finite-seed preimage/counting problem. Sampling independent raw tapes and placing the resulting decoded values beside the old seed produces an inconsistent transcript: the actual verifier recomputes F(s).

This argument is about the seeded map, not an allegation that the SHAKE PRG has a practical distinguishing attack. Statistical equality and computational pseudorandomness are different claims. The abstract uniform-shell game cannot be transported through this map with an assertion of exact equality. Neither one-coordinate entropy nor the `W+1` logical query ledger repairs the joint-support obstruction.

In a model that idealizes **both** group-root generation and coordinate SHAKE as separate oracle namespaces, a fresh coordinate SHAKE tape really can be programmed. But a `(seed,index)` key does not identify the preceding session/header; distinct headers with the same seed share tapes. Root seed collisions and old coordinate-key seed guesses would need their own q-dependent union bounds (a fresh uniform seed gives `2^-256` per specified seed, not a shell-family inverse cardinality). This alternative does not fix the default backend's missing answer-dependent descendants and is not an exact, zero-deviation full-prefix certificate for the existing code.

## 4. A concrete descendant-ownership counterexample in the default backend

Pinned dependency: `/home/hybr1d/.cargo/git/checkouts/spongefish-16a376885384d822/ef97413/spongefish/src/instantiations/hash.rs`, revision ef9741346a150039427d9e0c02d6b2d73e93ec81 (also pinned in `akita/crates/akita-transcript/Cargo.toml:18–29`). The default is `transcript-blake2b`; `src/sponge.rs:3–8` chooses its Blake2b512 wrapper.

In that dependency:

- `hash.rs:66–102` squeezes digest block `D(mask_squeeze || cv || BE64(block_index))`, caching the unused part of the full digest.
- `hash.rs:142–160` ends squeezing by setting

  `cv_next = D(mask_squeeze_end || cv || BE64(bytes_read))`.

  It clears cached leftovers. **The squeezed digest/answer bytes do not occur in this update.**
- `hash.rs:40–50` then starts the next absorb with `mask_absorb || cv_next || next_message_bytes`; ratcheting hashes that absorb state (`:54–63`).

Fix a pre-squeeze `cv`, a successful rejection-path length, and the following prover/public messages. In a primitive-RO fork, replace a squeezed digest block by another full block that yields a different accepted challenge but uses the same number of candidate bytes. The subsequent `cv_next`, the next absorb input, and hence its primitive query key are identical. Both branches can have one accepted field candidate; there is no need to exploit a rare rejection. The same phenomenon occurs for the group seed squeeze followed by the next group payload or successor message.

Therefore such a descendant key does **not** fix even the decoded root challenge. Relative to the desired root, its compatibility probability can be **one**, not `1/q`, `1/|shell|`, or `2^-256`. One can compute/query this next-state path from the known old cv and a chosen next message before obtaining the earlier squeeze answer. This is a structural counterexample to parsing every old descendant key as a guess of the root; it is not a hash collision and is not repaired by charging a collision probability.

Nor may independently indexed logical field slots be identified with primitive queries. A Blake2b512 primitive answer is 64 bytes. With no rejection, an ordinary base-field draw uses 16 bytes for A7F7, so several logical field draws can be determined by one already exposed full primitive answer. At a nonzero-PoW site the predicate consumes the first 32 bytes, and the immediately following field challenge can consume bytes from the **same** 64-byte digest answer. Conditional on that full answer the later chunk is a point mass. Independence between the predicate's tested bits and untested fresh bits is still true *before exposing that whole block*, but a second fresh-oracle-root ordinal cannot be claimed at the later squeeze. Even without intervening prover messages, one must choose the actual earlier primitive exposure or change the interface.

The Keccak option is different: the pinned `duplex_sponge.rs:192–241` overwrites rate bytes, tracks squeeze/absorb positions, and invokes a concrete permutation. The field sampler absorbs its retry count because the duplex otherwise forgets the squeeze length on later absorption (`sampling.rs:59–64`). That count is neither the full rejected history nor a literal indexed full-prefix key. An ideal-permutation analysis would be a separate construction/reduction; it is not supplied by calling it an ideal independent RO. The default Blake2b counterexample already rules out the requested universal existing-interface certificate.

Finally, `ProtocolSiteId` is only diagnostic (`proof_stream.rs:595–687`). Real schedule validation prevents an ordinary honest verifier from arbitrarily changing protocol order, but it does not turn these diagnostics into hash-key domain separation or injective complete-history syntax. Adding only an absorb of site IDs would fix that omission, **not** the seed fibre problem or the answer-dependent continuation problem.

## 5. Constructive exact FULL raw-tape decoder lemma

### 5.1 One masked integer draw

Let iid raw bytes form an unbounded tape. For a range size m>1 put `b=ceil(log2 m)`, and let ell be the actual code width (1, 2 or 4 bytes). Each attempt reads `8 ell` bits, keeps the low b bits and accepts iff that integer is <m. Write `p=m/2^b`.

For a prescribed accepted integer a in `[0,m)`, sample its exact raw preimage law as follows:

1. Draw rejection count R with `Pr[R=r]=(1-p)^r p`.
2. For each rejected attempt draw low bits uniformly from `[m,2^b)` and high `8 ell-b` bits uniformly and independently.
3. Set final low bits to a; draw its high `8 ell-b` bits uniformly.
4. Append a fresh iid tail, lazily as requested.

This is exact because each raw word has the same probability, accepted a is uniform over m possibilities, and R and the masked-out bits are independent of a. For m a power of two, R=0. For m=1 the code returns zero without consuming bytes; the conditional tape remains unchanged. Expected attempts are `1/p<2` (or exactly one for a power of two).

For a cap H on attempts, retain an explicit failure outcome of mass `(1-p)^H`; conditional on success replace the geometric law by

`Pr[R=r | success,a] = (1-p)^r p / (1-(1-p)^H),  0<=r<H`.

The same formula applies to ordinary prime-field canonical rejection with `m=q`, `b=MODULUS_BITS`, `ell=NUM_BYTES`. For generic field codecs the injective canonical representable set replaces `[0,q)`; the proof requires that actual canonical-set specification, not only a numerical width check. The current A7F7 field is explicitly checked LE128 (`jolt/crates/jolt-field/src/solinas/fp128.rs:1120–1172`), so m=q applies.

Default ordinary field sampling is unbounded. Keccak's checked attempts counter allows at most `2^32-1` candidates before error: use that H and account for its failure outcome. Rejected candidates, their excess masked bits and the final candidate's excess bits must all be retained. Absorbing the retry count does not replace them in the sampler state.

### 5.2 One unfiltered ring polynomial

Let `w=n1+n2`, and S be the ring-polynomial shell with n1 coefficients of magnitude one and n2 of magnitude two. Its size is

`M = binom(d,w) binom(w,n1) 2^w = (d)_w 2^w/(n1! n2!)`.

The partial Fisher–Yates path chooses a uniform ordered list of w distinct positions, then independent signs. Each ring polynomial has exactly `n1! n2!` such ordered representations. Hence the **ring polynomial** is uniform on S under iid raw bytes.

For prescribed c in S:

1. Choose a uniform permutation of c's magnitude-one positions and a separate uniform permutation of its magnitude-two positions. Concatenate them, retaining each position's prescribed sign. If the prescribed output is instead the exact ordered `SparseChallenge` representation, use that fixed order rather than drawing a permutation.
2. Replay an initially identity Fisher–Yates array. At step i, locate the chosen target position in the unchosen suffix. This uniquely determines accepted range digit `a_i=j_i-i` for range `m_i=d-i`. Use §5.1 for all its raw attempts, including rejected low bits and unused high bits. Apply the swap.
3. For every sign generate a byte with the prescribed low sign bit and **seven fresh independent high bits**. This matches the actual one-byte-per-sign decoder.
4. Append an independent raw tail. For the current 128-byte refill abstraction, sample and retain every unconsumed byte of the final prefetched buffer as part of that tail. Those bytes are not zeros and not freely discarded if the full primitive answer exposed them.

Every ordered preimage of c has the same conditional probability, and §5.1 gives the exact conditional distribution of every local range draw. Their product is the original FULL tape law conditioned on decoding c. No SHAKE state or finite seed is solved for.

An important representation detail: `SparseChallenge` derives vector-order `Eq` (`challenge.rs:60–68`), not canonical ring-polynomial equality. A future extractor's **distinct unit-difference test must compare polynomials**, e.g. sorted/coefficient form. Two different position orders can represent the same ring element and have difference zero. Current preview/live equality is a byte/algorithm replay check and is not an existing extractor test; this is a required sampler-interface rule, not a reported patch bug.

### 5.3 Fixed operator filter and all rejected candidates

Let A be the subset of S passing the installed deterministic operator predicate, let `a=|A|/M>0`, and let J=4096. Under iid tapes, consecutive complete sparse candidates are independent uniform S, because each consumes a stopping-time prefix and leaves an independent iid suffix. The unbounded position rejection has finite expectation. The accepted output, conditional on success before J candidates, is uniform A, and for any prescribed c in A:

`Pr[R=r | success,final=c] = (1-a)^r a / (1-(1-a)^J),  0<=r<J`.

The rejected polynomials are iid uniform `S\A`, independently of final c. All their latent position orders, mask rejections, sign slack and byte histories retain their original conditional laws.

There is a constructive implementation **not requiring enumeration of A or its exact cardinality**:

- Repeatedly run the original forward J-attempt filter on a fresh iid tape until one whole run succeeds. Retain that successful run's entire rejected prefix and its stopping count; discard only the raw segment of its final accepted candidate.
- Replace that final segment by the exact raw inverse decoder of §5.2 for c. Append a fresh iid tail, preserving the real buffering semantics.

The rejected-prefix/stopping distribution is independent of the final accepted ring polynomial, and the final candidate's raw law given c is exactly §5.2, so this is exact. Failure probability of one original run is `(1-a)^J`. Expected total complete-candidate work in the auxiliary forward sampling is `1/a`, plus one conditional final candidate, not an unpriced constant. A certified lower bound on a is needed for a useful runtime bound; nonempty support alone does not make the extractor cheap. If A=S, no operator rejection is needed. The actual approved fixed integer predicate, not a floating approximation or witness-dependent response test, defines A.

For W independently keyed coordinate tapes, success/failure and accepted laws factor by coordinate before any ordinary acceptance/nonce-selection gate. Thus, conditioned on coordinate completion, use the product of these exact samplers for a coordinate-wise star: preserve prescribed decoded values in all unchanged coordinates, change the chosen ring coordinate, and retain the literal original FULL raw answer as the marked branch. Do not substitute the *winning honest response nonce's* acceptance-conditioned vector law for this product law; that gate belongs to the prover computation being rewound.

### 5.4 Partial exposure and forbidden conditions

The lemma applies at a restored prefix **before the relevant fresh raw family is exposed**. If a raw block is already fixed in the retained table, it must be replayed exactly. If it determines an incompatible prescribed decoded value, that conditional fibre is empty; a sampler cannot overwrite it under the guise of resampling unused bits. Arbitrary success-conditioned ancestors also cannot be treated as fresh iid kernels.

For a finite vector of raw blocks, all unobserved slack blocks/bytes have their original independent law except for the explicit decoded-value and decoder-success conditions just described. For an infinite ideal tape, the statement means equality of every finite-dimensional exposed distribution with a lazy iid unused tail, not materializing an infinite array. This tail is freshly generated in each private world. The prefix table is restored; **absent future entries are not preserved from the original continuation**.

## 6. A fully finite sampler interface, including exhaustion and cost

If the retained sampling-game implementation requires one finite FULL raw oracle answer per family rather than a lazy tape family, make this explicit instead of leaving an infinite-XOF/compiler gap:

- Retain the existing J=4096 fixed-filter cap (J=1 without filtering).
- Give each Fisher–Yates range draw a public maximum H mask attempts. Exhaustion rejects/poisons the whole coordinate/family. This is a small but real decoder change; current ordinary position rejection has no cap.
- The maximum bytes needed for one coordinate are

  `L = J * ( H * sum_(i=0)^(w-1) ell(d-i) + w )`,

  where ell(m)=0 for m=1 and otherwise the actual 1/2/4-byte range width. An oracle family answer reserves that many uniform bytes per coordinate. All bytes unused after success remain uniform raw answer bytes. Define the read/buffer behavior so it cannot prefetch beyond this bound; alternatively round each coordinate allocation up to a 128-byte buffer multiple.
- Field families analogously reserve `H_f * F::NUM_BYTES` per limb and retain a failure atom.

Capping a range draw preserves its uniform accepted digit law because successful completion is independent of which accepted digit is returned. Sparse candidate-completion probability is therefore independent of its ring polynomial. A complete accepted filtered coordinate is still uniform A. To sample the finite full answer conditioned on c and success, run the finite forward decoder until a successful whole run, retain all its rejected-prefix state, replace its final candidate by §5.2 using the **truncated** local laws, and fill every remaining byte uniformly. This is the same independence argument as §5.3, including completion failures; it requires no exact |A| computation. If s is the actual per-coordinate success probability, a conservative expected byte cost for the auxiliary whole-run sampler is `L/s`, plus conditional-final decoding and raw-tail materialization. For a group use independent coordinate samplers; no need to reject and redo all coordinates merely because one auxiliary coordinate run failed.

For every nontrivial range, rejection probability is <1/2. A conservative new cap deviation for one family is

`delta_pos <= W * J * w * 2^-H`.

Add the original operator exhaustion `(1-a)^J` per coordinate and the chosen field failure `(1-q/2^b)^H_f` per limb. Union/charge these at the actually exposed original family hits or use the global cutoff analysis; do not hide them in a declaration of exact uniformity. This formula is a bound, not a numerical profile admission. Existing certified accepted-subset information may bound a, but raw-shell support is not itself |A|.

Using unbounded direct indexed tapes instead avoids the extra protocol cap, with the §5 exact expected-time sampler and a strict global extraction-work cutoff. Then the formal compiler must explicitly support the **block-query/lazy-family** interface, choose a family at its first exposure, charge every block, and prove its marked-family-game lifting. I do not silently identify a finite-answer theorem with that interface. The finite version above is a direct way to avoid that extra lifting prerequisite.

## 7. Smallest sound cutover and executable grammar contract

Two changes are essential; either one alone is insufficient:

1. **Replace the 32-byte-seed boundary with directly indexed raw bytes.** `FoldDraw::absorb_and_squeeze -> [u8;32]` currently forces seeded expansion. Replace that cryptographic boundary by a raw-family request/read capability. Let `SignedSparseScratch` and the masked-integer decoder consume that capability, not a SHAKE stream initialized from a short seed. Keep existing fixed integer filtering, source commitments, shell family, ring embedding, response checks and nonce transport. A concrete instantiation may compute `SHAKE256(literal canonical key)` directly, modeled as the protocol's indexed XOF/RO; no intermediate short root is the expansion input.
2. **Make full context, stage ownership and completed challenge results part of the actual key/history.** Logging IDs are not enough, nor is reseeding the old chained public sponge with one short digest.

A sufficient literal key grammar is:

`Key = Encode(domain/version, installation/context, session, augmented instance, P, request)`.

Use fixed tags, fixed-width integers, and length-delimited byte/vector fields; reject noncanonical encodings, overflow, unused/reserved fields, duplicate occurrences, mismatching arities or reordered scheduled occurrences. The installation binds the complete sampler descriptor (field and canonical encoding, d/n1/n2, filter parameters and exact integer predicate identity, all caps/raw layout and result codec). `P` is the canonical ordered protocol prefix, not a digest of it. It contains every prior public/proof message and every completed challenge result at its actual owner. `request` includes operation, stage, fold level, invocation, round, group/claim/block/limb identities and counts. A block-tape version also includes coordinate/block index with a separate block domain.

To remove all ambiguity about FULL-answer ownership in a finite-answer implementation, append a canonical completed-challenge event containing **both the full finite raw answer and the full decoded result** to P. This is reproducible by the verifier and adds no proof message. An interface that appends only the complete decoded result can also prove the old-key condition with its decoded point mass, but must state that convention explicitly; raw answers still live in the saved oracle table and the full-answer conditional sampler still applies. Do not claim that a digest of either representation is an injective full prefix.

A shell level can be one atomic request at the frozen prefix immediately after its response nonce, listing all `(group,claim,block)` coordinate descriptors in canonical claim-major order. Its raw answer is the product of the independent coordinate allocations, and its complete result is appended before the successor. This removes ambiguities caused by sequential group roots and gives the retained flat star exactly its product law. Existing group arithmetic dispatch and verifier consuming loops can still consume views of that bundle. Alternatively keep separate group requests and prove the corresponding sequential conditional compiler; do not merely declare all sequential root seeds an atomic independent tuple.

PoW is an explicit predicate-family request after the canonical nonce message; field/shell requests use separate domains after its completed result. The ordinary prover may search many nonce candidates; only its selected nonce goes in the proof. Keep response-nonce search as witness-dependent prover work, not as an oracle's built-in shell filter. A zero-bit PoW remains no nonce/predicate event if that is the installed grammar's policy; its absence must be predetermined by the plan, not an ambiguous optional record.

The descriptor already binds substantial identities, but setup/schedule/layout are digest-compressed (`instance_descriptor/mod.rs:156–282`). For a pure injective grammar with no extra concrete-hash collision premise, key literal canonical installation data or a verifier-installed unique index referring to a fixed registry. If using digest identities instead, explicitly separate the fixed registry assumption or a domain-specific ideal-hash collision/deviation term; do not call equality of concrete digests literal equality of complete source/plan descriptors. Bind the full sealed frontend/query prefix and endpoint roster into the PCS session/context as required by `AKITA_RELATION_ACCUMULATION_PROTOCOL.md:210–212`; an arbitrary caller-provided short session is not proof that this augmentation occurred.

This is an explicit proof-stream/protocol version and sampler-grammar revision. It changes challenges and therefore proof bytes. It need not change original Akita A/B/F commitments or sources, arithmetic identities, ordinary proof-message ordering, LEB128 transport, or ring-shortness assumptions. It is research, not an implemented cutover or authorization to modify dirty work.

### Cutover costs

- Direct coordinate raw bytes replace one 32-byte seed plus local SHAKE expansions. Logical oracle queries and concrete hash input processing increase; no universal speedup is asserted.
- Finite family answer size is `sum_coordinates L_j`, plus field allocations. Full-answer conditional sampling/storage/replay includes rejected/slack bytes; a strict work cap must pay for all of them. Direct literal-key hashing costs at least its actual absorbed prefix/request bytes per fresh request. With naive recomputation this is `O(sum_requests |Key| + total_raw_bytes)` hashing, not O(1) per challenge. Exact prefix-state caching/streaming may improve implementation cost only if it hashes the same literal key, not by replacing it with a short seed in the ideal interface.
- Appending raw answer history increases future literal-key input size (not proof transport). Appending only decoded completed results saves that cost and yields decoded-mass old-key bounds, but requires the explicitly stated history convention above.
- The selected family ledger, local sampler failures, private replay work, conditional-sampler work, and transcript copying all enter the full extractor runtime. Existing modeled SIS ROP values are not an arbitrary-algorithm `Adv(time)` conversion.

## 8. Old keys, root point masses and Q

For the proposed grammar, a request at a proper descendant is syntactically distinct from a root request and its prefix explicitly contains the root's completed result. A parse extracts **one specified root value** (or the required complete tuple) and its unique owner. All later requests, including proofs whose message contents happen not to depend on that value, contain that same root event. Thus a previously asked descendant key can be compatible with a fresh root only with the actual root point mass.

Safe masses, **before arbitrary ordinary acceptance conditioning**, are:

- Exactly uniform scalar K: `gamma = 1/q`.
- Uniform accepted fixed-filter ring challenge: `gamma = 1/|A|`, conditional on decoder success; include separate decoder-failure probability in the total bound. For prescribed tuples, multiply only when the fresh raw families are independent and every relevant descendant key fixes the whole tuple.
- Legacy modulo scalar: `gamma <= 2/2^128`; a full fresh 32-byte digest answer guessed literally has mass `2^-256`. Use the value actually fixed by the parsed key, not whichever denominator is most attractive.
- A root already determined by retained raw bytes is a point mass of one. The current default Blake2b descendant counterexample has compatibility one even for distinct successful root answers; no `binom(Q,2)/q` argument follows there.

If a fresh family has a decoder failure atom, the unconditioned mass of a successful decoded c is `s/|A|` (s=completion probability); using `1/|A|` plus separate failures is safe. A transcript with already fixed prefix bits has the corresponding conditional fibre law, not the fresh prior above.

For Q fresh original oracle entries, the usual pair ledger gives `sum_i (i-1) gamma_i`, bounded by `binom(Q,2) gamma_max`, **only after** the syntactic owner/point condition is proved. Separate cuts can be summed. Charging old pairs only once globally additionally requires a unique-owner joint ledger; no such joint ledger is inferred merely from stage names. One group bundle is not a license to use the full tuple mass at keys that contain only one component.

Q must count original prover/verifier and allowed initialization oracle work, all adversarial prequeries, all failed nonce candidates, explicit field/raw rejection reads at the chosen oracle granularity, and verifier queries needed to evaluate an output proof. If using a convention with an unqueried output, retain its `Q+1` treatment consistently. Private counterfactual extraction tables do not inflate the *original* Q in a marked-original theorem, but their queries/bit generation/storage/replays are charged to total extractor time and its strict cutoff. Block-query and atomic-finite-answer models have different Q counts; declare which is used, and also charge the answer lengths.

Actual `GrindingRun::fold_challenge_group` charges `coordinate_count+1` (`akita-params/src/transcript_grinding.rs:461–475`); `GrindingPlan::new` checks expanded logical count `<u32::MAX` (`:642–666`). This is a schedule audit count, not an enforced limit on adversarial ROM queries. `field_sampling_is_certified` checks widths and a supplied ceiling; descriptor creation passes the constant ceiling, not an observed execution query count. Neither can supply Q for this theorem.

## 9. Grinding and rejection laws

Actual ordinary PoW uses a canonical nonce in `[0,2^(g+7))`, `0<g<=25`, and tests the first g low-order bits of a 32-byte predicate for zero (`akita-transcript/src/grinding.rs:5–52`). Prover chooses the first passing candidate; verifier checks canonical range and the predicate, **not** firstness. Thus the verifier permits adversarial nonce selection within the range; every attempted oracle entry must be counted.

For independent fresh ideal predicate answers at each distinct candidate key, honest exhaustion is exactly

`(1-2^-g)^(2^(g+7)) <= exp(-128)`.

Given a successful selected predicate, its tested bits are zero and its other bits retain their correct uniform law. Given firstness at candidate n, each earlier full predicate has its own exact law conditioned on *failing* the tested-bit predicate, and the winner is conditioned on passing. These conditional strings can be sampled directly by uniform selection of a nonzero g-bit pattern / zero pattern plus independent remaining bits. They must not be replaced by all-zero rejected candidates or omitted from a full original oracle view.

At a default Blake2b primitive granularity a predicate block may also contain future field bytes (§4), so its FULL conditional sampler must include those bytes and the whole block's later constraints. The direct-domain cutover intentionally removes this reuse rather than assuming it absent. A single tested predicate can justify a local product probability only after joint independence of the relevant residual challenge and predicate at the right exposure is established. No blanket factor `2^-g` discounts the whole extractor or its runtime.

The 12-bit `FoldResponse` nonce has no such independent public PoW predicate. The honest accepted shell vector is biased by the source's response probes and firstness. The verifier merely reads a permitted nonce and then checks the real range/norm protocols. The correct fork experiment includes this selection in the ordinary adversarial computation and charges its queries; it does not treat winner vectors as fresh independent shell samples after conditioning on winner status.

## 10. Jolt legacy frontend: exact local law that can be kept

Actual alias: `jolt/crates/jolt-prover/src/akita/preprocessing.rs:15,27–28` selects `LegacyBlake2bTranscript<AkitaField>`, not the new D1 compiler and not the Spongefish Blake2b512 ordinary PCS backend. `jolt/crates/jolt-transcript/src/lib.rs:113–119` maps it to `DigestTranscript<Blake2b<U32>,F>`. `jolt/crates/jolt-akita/src/adapters.rs:41` aliases its field to ordinary fp128; A7F7 is `q=2^128-2^32+22537` (`jolt-field/src/solinas/mod.rs:156`).

`jolt-transcript/src/digest.rs:91–137,180–201` hashes

`D(state[32] || round_word[32] || optional_append_payload)`

with BE32 round in the last four bytes of the round word. A raw scalar request uses the **first 16 bytes of a full 32-byte digest output**, and `update_state` retains that **full 32-byte output** and increments the round. Raw and coefficient scalar paths both reduce the sixteen bytes little-endian for this Fp128 (`jolt-field/src/algebra.rs:345–356`; `solinas/fp128.rs:1120–1172`). Unlike the default ordinary Spongefish Hash continuation, this next legacy state really depends on all raw answer bytes.

Put `D0=2^32-22537`, so `2^128=q+D0<2q`. For a fresh ideal uniform 128-bit low word X,

`Pr[X mod q = z] = 2/2^128  if 0<=z<D0,`

`                         1/2^128  if D0<=z<q.`

Hence `mu=2/2^128` exactly. For a fixed nonzero univariate residual of degree d, at the fresh exposure its root probability is at most `min(1,d*mu)`. Sequential fresh independent coordinate draws give the coefficient-induction bound `sum_t d_t mu_t`. A powers vector is deterministic from one draw and must instead be charged by its actual univariate degree; it is not a vector of independent draws.

**Independence proof and its scope:** model this *named digest* as an ideal 32-byte RO. Conditional on the entire table/history immediately before a request whose primitive input is fresh, its full output is uniform independent of that history, regardless of how the input was chosen. The decoder therefore has exactly the mass formula above. Repeating at fresh input keys proves the sequential local conditional laws; the full previous 32-byte output is retained in the next state. Do not condition first on a later acceptance/no-old-query event and then assert the same law. Previously queried keys replay point masses and must be separately owned/charged. This is a genuine local fresh-query proof, not a global full-prefix compiler theorem for the compressed legacy chain and not a claim about concrete Blake2b output statistics.

There is also an explicit FULL raw conditional sampler if ever required locally: for prescribed z, choose X uniformly from `{z}` or `{z,z+q}` according to whether `z>=D0` or `z<D0`; append an independent uniform upper 128-bit half. This samples the original 256-bit digest output conditioned on its decoded value. Update the legacy state with that complete word. No 128-bit seed/ChaCha preimage is needed; the challenge is direct ideal digest output, not a seeded PRNG expansion. Future absent table entries must still be fresh in the trial world. The initial label's concrete digest and compression of whole history to state require their own global ROM/old-key argument if a full frontend compiler is claimed.

Causality at the inspected continued verifier boundary is real: `jolt/crates/jolt-verifier/src/segment/frontend.rs:230–263` absorbs segment context and calls `validate_and_seed_transcript` before stage one. Its generic bound is `PCS::SourceField=PCS::EvalField=F`, `T::Challenge=F`; no extension-coordinate independence is obtained from it. This inspection does not establish a whole-run/source-comparison theorem or the complete Candidate1 handoff. It establishes the correct fresh local law to use **where** that theorem has supplied a fixed-source residual and an honestly fresh exposure. No frontend sampler change is mathematically necessary just to replace degree/q by degree*maximum-atom.

## 11. Integration conclusions

1. The raw decoder and fixed-filter FULL conditional law are constructively proved for iid indexed raw tapes (§§5–6). All rejected candidates and unused bits are explicitly accounted for; fixed acceptance filtering preserves uniformity on the actual accepted set, not on the raw set.
2. The actual ordinary seeded shell sampler is not that interface (§3). At seed level the prescribed product fibre may be empty; at separately idealized SHAKE level tapes are programmable but full-prefix ownership and the chained-backend issue remain separate.
3. The default ordinary backend has a specific answer-independent descendant-state transition (§4). Thus the lighter note's parsed-descendant/root-point-mass premise is genuinely unfulfilled by the present code, not merely undocumented. Nonce grinding and logging metadata do not repair it.
4. The minimal sound cutover is an actual direct raw-family boundary **and** a canonical key/history binding completed challenges, with explicit finite caps or an explicitly proved lazy/block-family compiler. Version the transcript and sampler grammar; keep original sources/commitments and mathematical relations. Price raw lengths, exact conditional sampling, old queries and strict total work. This does not import a new lattice assumption or an extra tight end-to-end bit target.
5. Legacy Jolt's selected modulo-q local law can remain `degree * 2/2^128` at certified fresh exposures; its full raw direct-digest conditional sampler is easy. That local success does not certify ordinary shell forks or the whole combined session grammar.

**Parent integration:** preserve the conditional status of `AKITA_RELATION_ACCUMULATION_PROTOCOL.md`, LIGHT_ORDINARY_EXTRACTION and its SAMPLER_ADDENDUM. Replace a generic 'actual full-answer/compiler law not yet instantiated' placeholder by the two concrete obstructions and the constructive cutover above. Do not state that the current code already meets an exact uniform-product indexed shell theorem. No code, checks or production cutover have been performed by this research slice.
