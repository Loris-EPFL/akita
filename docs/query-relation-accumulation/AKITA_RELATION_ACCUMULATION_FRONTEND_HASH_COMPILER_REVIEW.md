# Candidate1 — continued Jolt frontend hash-compiler review

## 0. Decision, anchor, and scope

**The unchanged frontend does not pass the requested end-to-end compiler design gate.** Its fresh ideal-H256 scalar decoding and exact full-answer inverse are provable, and a bounded, uniquely parsed legacy lowering admits the explicit hash-graph simulation in §4. That is not the same as the current generic `DigestTranscript` already implementing a semantic indexed compiler: operation/site aliases are deterministic, there is no checked round limit, and the selected compact continued installation actually owns a different H1610 compiler. The minimal direct-indexed frontend contract in §§6–8 removes these gaps without replacing the stage protocols or adding completed segment PCS proofs. It preserves the existing modulo-p scalar law; therefore it needs neither a new sampler assumption nor rejection-sampling error.

The anchor is the **same original Main/RAM A/B/F sources**, exact source/view/decoder identities, approved program and continued transition statement, the real stage1,2,3,4,5,6a,6b,7 reductions, every selected deferred leaf, physical Main checks, and any selected normalization. The relation is the chosen base K-valued continued relation. Nothing here adds SDK tape/byte/halt adequacy, an unrestricted RV64 theorem, a new lattice assumption, a concrete-hash preimage/extractor axiom, or a tight end-to-end 2^-128 target. E=K=`Prime128OffsetA7F7` initially.

This is a read-only source/math investigation. No repository edit, Git operation, build, test, lint, formatter, proof generation or execution check was performed. Only this local report is written. The finite probability arguments below are derived here; the supplied frontend algebraic certificate and ordinary suffix/source-comparison theorem remain explicit inputs, not facts established by running code. Root filenames below are portable repository-relative links/references. No expanded local Akita-paper theorem is used as authentication of its unavailable official body.

## 1. There are three distinct compiler/caller paths

1. **Legacy digest-compatible Jolt frontend.** `jolt/crates/jolt-prover/src/akita/preprocessing.rs:26-28` aliases `AkitaTranscript` to `LegacyBlake2bTranscript<AkitaField>`. `jolt/crates/jolt-transcript/src/lib.rs:111-119` makes that `DigestTranscript<Blake2b<U32>, F>`. The continued stage entrypoints are generic over the fallible `Transcript` trait; their genericity does not establish a particular compiler theorem.
2. **Selected compact continued installation.** `jolt/crates/jolt-verifier/src/segment_emissions.rs:1-36` explicitly sets `CompactFrontend = d1::frontend::Inner<Registry, Binding, K>`, not `DigestTranscript`. Its complete context includes the exact message/draw schema, branch schedule, native-plan binding and finite resource profile (`:422-485`). `run` creates a fresh owned inner session (`:510-523`); `resume_frontend` and `resume_frontend_from_native` publish an authenticated complete next context on the *same* retained compiler (`:524-584`). The inner primitive is `h1610`, concretely a domain-separated Keccak construction with 1610-bit inputs and 1024-bit outputs, not an ideal H256 interface (`jolt/crates/jolt-transcript/src/d1/inner.rs:1-49`). Its K sampler is two sequential canonical LE128 attempts (`d1/frontend.rs:320-341,407-454`). A theorem about ideal H256 does not silently cover that path.
3. **Ordinary Akita boundary.** The actual ordinary executor still declares protocol **v7**, not executable v8 (`akita/crates/akita-transcript/src/proof_stream.rs:26-30,204-242`). The root v8 indexed design is a proposed contract (`AKITA_RELATION_ACCUMULATION_INDEXED_TRANSCRIPT_REVIEW.md:294-295`), not an existing verified implementation. A OneHot layout string containing `native-batch/v8` is not the PCS transcript version. §10 states the required v8 inherited-evidence binding and the exact current receiving seams.

Consequently, neither “the legacy state keeps all 32 output bytes” nor “the selected frontend has a finite H1610 ledger” establishes the requested Candidate1 ordinary-v8 frontend compiler. These are useful but different implementation facts.

## 2. Exact legacy compiler and codec

Write `H : {0,1}* -> {0,1}^256` for the idealization of the specific 32-byte digest, `pad32` for right zero-padding, and

`R(r) = 0^224 || BE32(r)`.

The production state is `(s,r)`, `s` 32 bytes and `r:u32` (`digest.rs:24-33`). The actual operations are:

- Constructor, label length at most32: `s = H(pad32(label)); r=0` (`digest.rs:146-173`). The assertion is not a fallible transcript rejection.
- One append with raw payload b: `s' = H(s || R(r) || b); r'=r+1` (`:91-95,186-188`). There is no operation tag, semantic site, codec tag or payload-length field in this primitive key.
- One 32-byte squeeze: `X = H(s || R(r)); s'=X; r'=r+1` (`:115-123`). **All32 bytes, not just the16 decoded bytes, become the next state.**
- Scalar/raw and coefficient/scalar APIs each squeeze16 bytes and invoke their field conversion (`:191-200`). For this K both conversions are LE integer reduction modulo p: default `from_challenge_bytes` is `from_bytes_le_reduced` (`jolt-field/src/algebra.rs:348-356`), and `Fp128` implements both paths accordingly (`jolt-field/src/solinas/fp128.rs:1126-1172`).
- A vector of count n performs n consecutive scalar squeezes and returns the n fields. There is no intermediate public/proof append. The vector is **not** one 256-bit seed expanding to n independent values; it makes n primitive calls. A called n=0 vector performs no hash/update (`digest.rs:230-252`).
- `raw_challenge_bytes(out)` with length m uses `b(m)=max(1,ceil(m/32))` primitive squeezes, including one squeeze when m=0 (`:85-118`). For m>32 its chunks are consecutive chained keys, not independent blocks from one key. The universal frontend scalar APIs only use m=16.
- Deterministic scalar powers consume one coefficient even when requested output length is0 (`jolt-transcript/src/transcript.rs:235-271`); the resulting powers are not fresh coordinates.

### Message encoding is legacy encoding, not `codec.rs` framing

`DigestTranscript::append_message` validates a site/value and then calls `append_legacy_value`; it does **not** absorb the site descriptor (`digest.rs:211-219`). `codec.rs::BytesMsg` belongs to the spongefish path and is not the encoding applied here.

The actual encodings (`legacy.rs:94-182`) are:

|MessageValue|Actual raw appended payload|
|---|---|
|Bytes(b)|exact b, including length0|
|Label(l)|l right-padded to32; reject l longer than32|
|LabelCount(l,n)|l right-padded to24 followed by BE64(n); reject l longer than24|
|U64(n)|24zero bytes followed by BE64(n)|
|Field / Native canonical bytes b|reverse the **entire** b; append exactly `len(b)` bytes, not32 padded bytes|

The K canonical source representation is16-byte LE (`fp128.rs:1114-1122`), so its legacy absorbed payload is16-byte BE. That absorption endianness does not change challenge decoding, which uses the *first*16 digest-output bytes interpreted LE. `validate_legacy_message` checks kind/count and, with the D1 feature for actual K/E identities, canonical decoding; a legacy native family otherwise relies on its caller's declared schema. Type checking is not a cryptographically absorbed type tag.

Every append is a separate hash boundary. Therefore `append(a); append(b)` is not simply `append(a||b)`; payload length is already part of a *single* oracle input's string length. Arbitrary variable bytes do not themselves create an internal concatenation ambiguity between the fixed32-byte state and fixed32-byte round. The real deterministic ambiguities are those in §5, and semantic boundary interpretation still needs a unique schedule.

## 3. Independently derived exact fresh field/full-answer law

Let

`N=2^128, c=0xFFFFA7F7=4294944759, p=N-c`.

The field alias is explicit in `jolt/crates/jolt-field/src/solinas/mod.rs:150-156`. For a fresh primitive answer `X in {0,1}^256`, split

`a=LE128(X[0..16]), u=X[16..32], y=a mod p`.

Since `p>N/2`, the number of low-half preimages is

`n_y = 1 + 1[y<c]`.

Thus, **at a genuinely unexposed primitive key**, before acceptance or future-transcript conditioning,

`Pr[y=z] = n_z/N`,

`mu = max_z Pr[y=z] = 2/N`,

`Pr[X=x | y=z] = 1/(n_z N)` for `decode(x)=z`, and0 otherwise.

An exact successful full-answer inverse is:

1. choose a uniformly from `{z}` if `z>=c`, or `{z,z+p}` if `z<c`;
2. choose the remaining128 bits u uniformly and independently;
3. output `LE128(a)||u`.

This takes constant bounded sampling/serialization work. It has **no rejection/exhaustion event**, no setup dependence, no concrete hash inversion, and no distributional approximation. The high half cannot be set tozero or copied from a nonmarked original branch. When z<c it is also incorrect always to canonicalize the low half to z. A marked original branch retains its literal original complete32-byte answer, including its particular low representative and high half; repeats replay that same table value.

For a sequence/vector of fresh scalar keys, apply this kernel sequentially at the actual chained keys. It samples the whole list of primitive answers conditional on the prescribed fields until an already exposed/incompatible key or graph alias is encountered. It is not a theorem that one n-field vector can be inverted by finding a preimage of one short root seed. A fresh key already present in the saved table is not fresh: replay its actual answer or reject an incompatible programming attempt, never overwrite it.

The exact variation distance from uniform K is

`Delta_mod = c*(p-c)/(N*p)`.

One may instead hybridize a bounded experiment with uniform fields at a cost at most `D*Delta_mod` for D exposed coordinates. **The recommended actual-law analysis does not perform that hybrid**: use mu=2/N in the fixed-source degree ledger, and add no redundant modulo-bias term. `1/p` is not the actual maximum atom.

For context: a degree-d residual fixed before a genuinely fresh scalar answer has root probability at most `d*2/N`. This law does not survive arbitrary conditioning on eventual acceptance, matching sources, a winning nonce, or a fully exposed continuation. In particular, saying “uniform H256” without the word *fresh* is not a Fiat–Shamir theorem.

## 4. A valid primitive-H256 **conditional legacy-lowering theorem**

The legacy chain is not inherently impossible to analyze. The following sufficient theorem explains exactly what would be needed to retain it and gives a conservative explicit price; it does **not** assert literal indexed-prefix ownership.

### 4.1 Additional finite lowering contract

Fix an externally approved compiler mode and installation. Require:

1. A deterministic, prefix-causal parser from a valid lowered raw hash-chain history to **one** semantic event history, branch choice, segment identity, source header and challenge owner. Two different accepted headers/owners cannot lower to the same relevant raw history. Every target Main/RAM/advice/program source identity and transition statement must be recoverable before its root challenge is exposed.
2. Every accepted continuation follows that parser/schedule. Raw APIs, clones and error recovery cannot manufacture an alternative accepted schedule; the verifier never continues after a protocol error. An explicit reset is a new admitted context, not permission to forget an inherited prefix.
3. A strict finite cap on all source/record sizes, oracle calls, inverse-kernel calls and execution work. Each digest instance makes at most `2^32-1` updates. Counter overflow is rejected **before** hashing/updating. Opaque initial oracle-correlated advice is absent, or its generation and queries are included.
4. The simulator retains full H answers, input strings and separate private counterfactual tables. Each new programmed successful scalar answer uses §3's exact kernel; a previously exposed answer is never changed. Original marked records, output and answers are preserved.

These are syntactic/finite implementation contracts, not a hash knowledge assumption.

### 4.2 Reverse-table simulation and graph bad event

The simulator records every primitive pair `(k,H(k))`. For a canonical transition key, its first32 bytes are its predecessor state. A reverse table from output states to their originating keys reconstructs already exposed ancestry; it does not invert a concrete hash.

Stop at the first occurrence of either:

- two **distinct** keys yielding the same32-byte output state, or
- a newly generated output state equaling the state-prefix of a previously queried would-be successor whose ancestry was not already exposed.

It is harmless to overcharge all old/current input-state prefixes and all old output states, including irrelevant/malformed adversarial queries. Replays and intentional clones of the same prefix are not fresh random states and are not charged again.

Let alpha_t bound the maximum probability of a particular complete new state at fresh entry t, conditional on its legitimate past. Ordinary fresh H answers have `alpha_t=2^-256`. A programmed answer conditional on field z has `alpha_t=1/(n_z*N)<=1/N`, by §3. There are at most `2t-1` old/current candidate states to overcharge. Hence

`Delta_graph <= sum_(t=1)^M (2t-1)*alpha_t <= min(1,M^2/N)`.

For an entirely unprogrammed original execution with q_H primitive exposures the sharper bound is

`Delta_graph,orig <= min(1,q_H^2/2^256)`.

An intermediate tighter inventory separates unconditioned and conditioned entries with their displayed alpha_t; one must not give a programmed field fiber256 bits of entropy. The conservative M bound includes all original/private entries across all simulated worlds; summing per-world bounds is at most this square. Copies of a saved entry do not become fresh entropy.

Outside this graph event, a valid accepted successor cannot have been prequeried before its as-yet-unexposed predecessor output: that would require exactly the forbidden state guess. Exposed ancestry is unique, and contract1 converts that ancestry into one semantic owner/header. At a fresh root the simulator can therefore assign the hypothetical source distribution before revealing the answer. Each later local challenge has its real fresh field law; old direct-indexed requests in a later ordinary augmented context can be handled by their parsed decoded owner as in §7. For legacy keys this is a **probabilistic graph/causality argument**, never “a later key literally contains the ancestor field.”

The coupling is an exact online construction up to its stopping bad event. It is incorrect to condition the whole experiment on `not Bad` and then claim the unmodified exact independent-field law; conditioning can bias it. Bound the first bad event and retain the original unconditioned exposure law instead.

### 4.3 Result and precise limit

With contracts1–4, the supplied fixed-source certificate and a bounded original-event sealed-query/PCS source extractor, the normal source-probe proof goes through with §7's error plus `Delta_graph`, and §8's work with full primitive simulation included. This is an independently derived **conditional** primitive-ideal-H256 simulation, not a concrete Blake2b extractor theorem.

The generic digest engine does not implement contract1 or contract3, and the active selected compact installation is not a Digest/H256 installation at all. Therefore this theorem is **not** an unconditional pass for the checked-out continued application. In particular, finite H1610 resource caps do not retroactively instantiate a digest counter guard or its semantic lowering parser.

## 5. Exact obstructions to granting the generic engine semantic ownership

These are algebraic API counterexamples, not claimed attacks on a pinned honest stage schedule.

- `append_bytes(empty)` and one squeeze use exactly the same primitive key `s||R(r)` and produce the same new `(s',r+1)` (`digest.rs:115-123,186-200`). One history has a message event and the other a challenge/result event. No H collision is involved. An actual empty separator is used by the stage4 RAM value-check protocol (`jolt-verifier/src/stages/stage4/ram_val_check.rs:602-625`; the selected roster records bytes empty at `segment_emissions/roster.rs:612-615`). This is not a license to remove it: in the real fixed schedule it is an additional state update before gamma.
- Scalar and coefficient operations have identical primitive keys and identical K decoding. A vector of n equals n consecutive scalar operations at the raw engine; a vector of0 is no operation. Thus changing API-family/site history need not change a later key (`digest.rs:221-252`).
- Distinct checked `MessageSite`s with the same value lower identically. Distinct `DrawSite`s with the same operation/count lower identically. Site/stage/family/sampler/carry metadata is checked externally but never hashed here (`emission.rs:57-131`; `digest.rs:211-252`). Checking metadata does not make it an oracle-input owner.
- `Label(a)` equals `Label(a||zero)` whenever both fit; `LabelCount(empty,n)` equals `U64(n)`; constructor labels have the same zero-padding alias. `Bytes` can reproduce either payload exactly. These are deterministic cross-codec aliases (`legacy.rs:140-181`; `digest.rs:146-157`). They need not matter under one pinned typed schedule, but defeat an unrestricted semantic-compiler injectivity claim.
- Two full answers with the same field decode and different upper16 bytes lead to **different** next chained keys. Thus a decoded-only indexed history is not literally the legacy state machine. The correct full-answer inverse must carry those upper bytes through all descendants, or a compiler change must be declared.
- `n_rounds += 1` is unchecked (`digest.rs:121-123`). After `2^32` updates it overflows: checked builds panic and wrapping builds lose the nonwrapping counter invariant. Finiteness of a particular honest trace is not a declared bound on every accepted/adversarial/extractor execution.
- `reject(error)` simply returns error (`digest.rs:256-258`); there is no sticky poison. The current callers must propagate failure, and a new accepting wrapper must enforce that property. A successful continuation after a caught partial-compound failure is not licensed by the theorem.

The exact conclusion is **failure of the unconditional semantic owner/parser contract**, not proof that a fully pinned fixed schedule cannot have a bounded primitive-ROM simulation. Retaining the chain requires implementing and proving the specific contract in §4; it cannot be replaced by an unsupported concrete preimage/extractor assumption. The direct-indexed cutover below is a smaller and more auditable alternative to building a second implicit lowering parser.

## 6. Minimal direct-indexed frontend migration [PROPOSED, no code changes]

Keep all arithmetic protocols, source commitments, source views, proof polynomial codecs, stage/round ordering, aliases, Main predicates, normalization and returned original leaf obligations. Replace only the frontend transcript owner/constructor and its event/challenge boundary. The existing `Transcript`, `MessageSite`, `DrawSite`, `MessageScope`, typed stage plans and source-check plans already expose those boundaries. Do not rewrite sumcheck or call a native D1/full-ring path the requested ordinary K/K path.

### 6.1 Exact header, event and request grammar

Choose one new installed frontend mode, for example `jolt/continued-k/indexed-h256/mod128/v1`. Freeze it externally, not by a proof-selected downgrade bit. Encode a literal header Hdr containing:

- domain/version, exact approved installation index and field/sampler/result codecs;
- complete approved run/program/preprocessing identity and expected statement linkage;
- segment count/order and exact segment context, or an authenticated ordered context transition;
- source/view identities, setup/profile/reference bindings, selected branch geometry and finite limits.

Use tagged, checked-length frames. Immutable objects may be referenced by an exact externally approved registry index; a fresh prover-supplied digest is not an injective substitute.

Maintain append-only canonical history P of public/proof message events and completed challenge events. A message event contains its exact `MessageDefinition`: occurrence, public/proof kind, codec identity, atom count and canonical payload. A compound is checked for complete ordered consumption using the existing schema; every source append leaf is retained, including empty bytes. Legacy field/native byte reversal is an absorption compatibility convention, not a requirement for the new typed canonical event: choose and install one canonical encoding, preferably existing K LE128 and exact native bytes, identically on both sides. Proof payload transport need not change.

A request R contains complete structural stage/site, operation, scalar/vector count, family, sampler, tree mode/arities, powers bound, deterministic carry origins/projections/arguments, result codec, and a checked ordinal. Raw and coefficient, scalar and vector are distinct operations. No native shell operation is added to this frontend owner. A zero vector still creates its completion event; no fake random coordinate is drawn.

### 6.2 Use primitive H256 directly, not a seed expansion

For scalar/vector request R of count n, freeze P and use the literal component keys

`k_j = Domain || Frame(Hdr) || P || REQUEST || Frame(R) || COORD || LE64(j)`, `0<=j<n`.

P is a stream of complete tagged frames terminated by REQUEST; do not put a changing total-P length before the cached event stream. Hdr and R have their own fixed checked frames. Each block is `X_j=H256(k_j)`, and each result is `y_j=LE128(X_j[0..16]) mod p`, exactly §3. A scalar has n=1. A vector has **n separately indexed primitive H256 blocks** under one frozen P, not one short seed. No public/proof event or caller-visible coordinate return occurs between request formation and completion of the whole vector. Count n primitive oracle entries, not one, even if the API returns one logical vector.

Append before return:

`COMPLETED || Frame(R) || Frame(whole canonical ordered decoded result)`.

The complete request and full decoded tuple, not a state digest or raw-answer hash, enter P. Empty vectors append an empty tuple. Deterministic carried points/powers retain their original owner and are not new draws. Every error poisons the owner; accepting handoff requires its full required event schedule and no pending transaction.

An incremental concrete Blake2b-256 implementation may cache the **unfinalized hash state for precisely the literal header/event prefix**, clone it, absorb the request/component suffix and finalize. That is ordinary incremental evaluation of the full literal key, not replacement of the history by its32-byte digest. This is a conventional instantiation of the approved primitive ideal-H256 model, not a theorem that concrete Blake2b is an ideal oracle. Alternatively the project may deliberately reuse the ordinary-v8 finite SHAKE family, but then it must install that different atomic family/granularity and account for it explicitly; do not silently mix the two query counts.

### 6.3 Reuse the existing frontend source protocol boundary

The verifier already absorbs `SegmentContext` before seeding, and original Main plus optional auxiliary/program commitments before stage1 (`jolt-verifier/src/segment/frontend.rs:203-253`; `verifier.rs:417-452,804-900`). Continue doing so. The staged sumcheck verifier already checks shape/degree, absorbs the actual polynomial, then requests its challenge (`jolt-sumcheck/src/verifier.rs:63-85,109-151`). Compressed missing-linear reconstruction remains fixed before its draw. Reuse this exact sequencing and its `SumcheckEmissionPlan` source-round identities (`jolt-sumcheck/src/emission.rs:1-58`).

Reuse the selected roster's semantic member/round/carry identities, public context encoder, branch/point curation, Main source checks and normalization plans (`segment_emissions/roster.rs`, `public.rs`, `handoff.rs`). Change the installed compiler/sampler/result descriptor and owner-specific registry adapter; the current schema string explicitly names H1610/two-attempt sampling (`segment_emissions/schema.rs:215-243`) and cannot be reused unchanged for modulo-H256. The existing inner `Binding` is tied to an H1610 native registry and is not automatically an indexed-H256 owner. This is a new owner/binding mode, not a `DigestTranscript` alias pretending to absorb descriptors.

## 7. Independent proper-descendant/source-probe theorem for the replacement

### 7.1 Syntactic ownership and exact raw law

A proper descendant key parses P and contains a COMPLETED event with the complete owning R and full decoded result. Reconstructing that event's preceding prefix fixes the ancestor component family. For any designated first fresh component j of that request, an already queried proper descendant therefore specifies one particular field value z_j. Before ordinary acceptance conditioning,

`Pr[compatibility with that old key] <= mu=2/N`.

The component index and complete request prevent a different scalar/vector/site/segment owner from claiming the event. A vector descendant owns the whole tuple. When all n components are unexposed the fixed tuple mass is `product_j n_(z_j)/N^n`; the source-probe root bound only needs the first fresh component and mu. Other components might already be exposed in an adversarial query order, so do not grant an unconditional fresh product law then. A zero vector is not a random root cut.

For each new component the full32-byte inverse is §3. Conditional on a prescribed tuple at fresh components, take the product of those exact kernels; previously exposed components replay their saved bytes. No raw-state collision term is needed: distinct literal indexed keys remain distinct even when their hash outputs happen to coincide. Raw answers are saved in the oracle table, not necessarily in the receipt; the public history deliberately retains decoded results.

This gives an exact conditional full-answer interface in primitive ideal H256. It does not assert the same literal property for the old chain.

### 7.2 Global old-exposure bound

Let q bound **all included original** oracle exposures: adversarial/prover queries, verifier/output evaluation, failed probes/nonces, and allowed oracle-dependent initialization, across frontend component keys and the later indexed query/ordinary domains. Use one consistent ledger; counting total calls rather than distinct entries is conservative. If a later indexed ordinary key imports the frontend evidence, its parsed context also fixes the relevant completed frontend result.

At a first fresh root exposure i there are at most i-1 older candidate descendant entries. Therefore

`Pr[G_old] <= sum_(i=1)^q (i-1)*mu <= binom(q,2)*(2/N)`.

There is no extra T factor merely for listing T published segments when each root/event has a unique installed owner. This is not a bound conditioned on a winning proof. Initialization that freely supplies future answers would invalidate it; included, counted initialization does not.

### 7.3 Fixed-source local errors and source probes

Let `epsilon_FE,j` be the complete existing fixed-source frontend error for segment j under its actual fresh law, including every source predicate/normalization/prefix object selected by its profile. In the certificate's selected no-advice full-public case it is the stated degree ledger with M=2/N; this review does not re-audit the degree inventory or strengthen the application relation. In particular, optional advice/program/field modes cannot inherit the selected coefficient by assertion.

At each first root hit assign a hypothetical source from the bounded matching successful continuation kernel **before** exposing that fresh root answer. Proper ownership makes its target source/header fixed, while later sources may vary. Each actual fresh local residual is determined by that source and its causal past. The fixed-source degree/pivot certificate gives

`Pr[G_fresh] <= q * sum_j epsilon_FE,j`.

Run the original sealed-query-plus-complete-PCS source extractor B once. For each selected segment restore only its legitimate pre-root table and ordinary state/tape, independently replay continuations, and invoke the bounded deeper extractor only on the exact same root key/index/header gate. At the first valid source output, compare only target original A/B/F sources with the original B sources. Return the named collision on disagreement, or use the fixed-source relation test on agreement. Do not identify source explanations from C fixation, equal decoded values or private handle clones. Future commitments/D/H are not coalesced. Do not copy the original future oracle table into private probes.

For each saved prefix sigma let p(sigma) be one trial's valid-matching-source probability and a(sigma) its cheap matching-acceptance probability. The first-success source law is the exact one-trial success submeasure divided by p. A probe starts with original probability p, so `p*(1/p)=1` cancels its conditional expected number of trials. Prefixes with p=0 never start on original valid source output. This is an unconditioned original-event argument, not a lower bound on success conditioned on acceptance.

Given the supplied original-event suffix guarantee kappa_B, original-source comparison certificates and deterministic K-valued segment assembly,

`Pr[original acceptance AND no original run/query witness or named collision]`

`<= kappa_B + q*sum_j epsilon_FE,j + binom(q,2)*(2/N) + delta_cutoff`.

The final lattice/setup experiment additionally charges `Delta_setup + sum_I Adv_MSIS,I(T_I,eta_I)` at the actual mixed collision radii and whole finite computation times. There is no `time/2^128` conversion from a Quantum128 modeled table floor. If retaining §4's legacy simulation, add its explicitly inventoried `Delta_graph`; the direct-indexed replacement has `Delta_graph=0` in the stated ideal model. No modulo-uniformization or sampler-exhaustion term is needed for the recommended modulo decoder. These statistical terms are distinct and must not be hidden inside the local algebraic coefficient.

## 8. Oracle, byte/work, and strict-cutoff accounting

### 8.1 Original frontend calls

For a legacy digest instance, let A be the number of actual raw append leaves, F the number of scalar coordinates, and B the total additional raw byte-squeeze blocks not already counted in F. Then

`Q_H,V <= 1 + A + F + B`.

The1 is the constructor; identical repeats may be one fresh entry but still cost an invocation. Count all context/commitment label leaves and values, empty appends, source-check/normalization messages, and both endpoints. Vector count n contributes n to F; scalar powers contribute1; vector0 contributes0. The round update count is A+F+B, not the number of high-level transcript methods.

For the proposed direct owner, messages/history appends require encoding/storage but **no ideal oracle call**. Its frontend verifier call count is exactly `sum_R count(R)` for its K requests, with scalar count1 and zero vectors count0. Add every adversarial block prequery, ordinary/query-domain exposure and included initialization. The H1610 ledger's `fresh_k_coordinates` is useful structural metadata, but its component-call count is not the proposed H256 count and does not cap the adversary.

### 8.2 Work is not just q

For primitive entry k, charge its complete literal input length `K_k`,32 output bytes, table insertion/replay and decode work. A conservative oracle simulation charge is

`O(sum_k (K_k + 32 + decode_k))`.

For private conditioned entries add the exact inverse's32-byte generation and full key/table costs. For the legacy engine `K_k=64+payload_length` on an append,64 on a squeeze and32 on construction. For the direct owner K_k contains the full literal header/history/request/component suffix; input hashing can be optimized with exact incremental-prefix caching but its encoding, snapshots and retained tables still cost work. Never replace it by an O(1) short-seed input in the analysis. A total per-world table byte bound is `O(sum_k(K_k+32+metadata))`; independent fork worlds and copied snapshots are charged, not free persistent memory.

Let C be the full original computation cost, including ordinary oracle/codec work, P_S the expected gated deeper extraction work in one original execution including its full inverse kernels, and W_aux actual snapshot, source-comparison, reconstruction and assembly work. With T selected segment probes,

`W_expected <= (1+T*q)*C + (T+1)*P_S + W_aux`.

There is no product of T frontend trees. This expected bound is **not** a runtime conditioned on rare accepted receipts.

For a strict K-trial cap per segment, a started probe's exhaustion mass is at most `q/(K+1)`; across T segments,

`delta_trials <= T*q/(K+1)`.

With a strict suffix bound L_S,

`W_strict <= (1+T*K)*(C+L_S) + W_aux,strict`.

All bounded suffix conditional samplers/private entries must already be in L_S. Alternatively impose one strict total work cap L and add `W_expected/L`; select one cutoff construction rather than charging the same event twice. The original suffix cutoff belongs in kappa_B; a failed suffix in a probe is part of that probe's success law, not a separately multiplied loss for every rejected trial.

If §4 is selected, its M is a strict bound on **every newly sampled/programmed primitive table entry in this whole computation**, including private suffix sampling and initialization. For example one may derive it from `(1+T*K)*(Q_H,C+M_H,S)` plus separately counted setup/snapshot operations. It is not merely the honest verifier's call count. Original q and whole-extraction M serve different purposes.

## 9. Required finite guards and reset/statement contract

### 9.1 Guards

The legacy minimal necessary arithmetic guard is

`A+F+B <= u32::MAX` per digest instance,

with rejection before any update that would exceed it. Starting at0, at most `2^32-1` completed updates are allowed. A valid guard cannot wrap the counter, saturate it, rely on debug overflow behavior, or restart it to continue the same proof. `InfallibleLegacyTranscript` cannot report this failure itself; either admission proves an exact upper bound for every reachable call or the accepted owner must be fallible and non-bypassable.

For either route install checked finite limits for: segment count; dimensions and complete source sizes; message/prefix/request/native-handoff bytes; event count and ordinal; coordinates per vector; powers output; aggregate original oracle budget convention; every private oracle/conditional-kernel count; table/snapshot bytes; and strict total work. Checked multiplication/addition must precede allocation and hash exposure. The current selected H1610 profile declares useful finite values (16segments,16MiB payload,512MiB transcript, `2^20` records/sites, `2^24` component calls, `2^28` read bits,256coordinates/powers; `jolt-verifier/src/segment_native_resources.rs:10-53`), but those are a different installed compiler profile and explicitly not a numerical security admission. Rebind and derive the new owner’s costs instead of silently copying a component-call budget.

### 9.2 Context/source fixation and caller responsibilities

The continued generic verifier takes `&mut T`; it does not construct/reset that transcript. It validates the RAM contract, program/predecessor, trace shape, absorbs context, seeds the proof and then runs every stage (`segment/frontend.rs:190-253,258-381`). Main is committed/absorbed before stage1, not at stage8 (`verifier.rs:417-452,804-900`; prover stage0 `jolt-prover/src/akita/stage0.rs:168-187,198-251,284-350`).

`SegmentContext::absorb` binds version, run/preprocessing IDs, inputs/trimmed outputs/panic, index/total/active rows/trace length, expanded counters, both PC/register boundaries, successor flags, and original incoming/outgoing RAM handles (`jolt-verifier/src/segment.rs:359-457`). Its validation/predecessor checks enforce declared counters, register/PC adjacency and exact RAM-handle continuity (`:281-323`). The external expected run/program/IO/index/total/final expanded-row count is checked by archive orchestration (`receipt/archive.rs:229-268`), not conjured by a standalone transcript constructor.

A reset per segment is permissible for the proposed indexed mode only when its full root context fixes the segment, predecessor/expected statement, original source references and mode, and the final verifier checks every ordered segment and inherited evidence. It is not permissible to reset under a generic root label and claim the old prefix is still bound. A same-owner transition is also permissible when it publishes the authenticated full next context before the next root. The currently selected H1610 `run/resume` contracts demonstrate this distinction, not a ready-made H256 implementation.

The source-probe gate includes actual root key, ordinal, segment/context and original target source/view identities. It does not match “the same C somewhere.” Even an invalid old accumulator must retain every old coefficient/query/occurrence: final flat relation restriction gives the old equations without assuming their validity. History claims require retained/replayed historical frontends and externally checked complete-run linkage, not just those old scalar equations.

## 10. Existing ordinary-v8 design must bind **inherited** frontend evidence

The approved v8 specification already has the right semantic location: its literal header's augmented instance includes the complete sealed frontend/query handoff and endpoint roster (`AKITA_RELATION_ACCUMULATION_INDEXED_TRANSCRIPT_REVIEW.md:25-86`). Implement it, including old evidence, rather than merely prepending a final digest to a new session.

### What current ordinary code actually receives

- `AkitaVerifier::batched_verify(proof,session,statement,basis)` is the real ordinary consumer (`akita-verifier/src/fold/verify.rs:25-31`). It validates the statement, derives the descriptor (`:102-108`), constructs its channel (`:109`), then verifies root, full suffix and finish (`:123-145`). Producer channel construction is `akita-prover/src/protocol/prove/root.rs:52-59`.
- The existing descriptor receives setup/layout/selection/schedule/basis, not frontend evidence (`akita-config/src/transcript_binding.rs:38-74`; `akita-types/src/instance_descriptor/mod.rs:55-69,153-260`). Root replay binds actual commitments, points and values (`akita-verifier/src/fold/root.rs:40-83`). Neither alone identifies Jolt's semantic source role, decoder or inherited frontend history.
- Production legacy Jolt binds its grouped endpoint statement, but `bridged_akita_session` exports only a label plus one `raw_challenge_scalar()` (`jolt-akita/src/native_batching.rs:153-202,349-402`; `jolt-akita/src/adapters.rs:1430-1448`). That is not full-prefix v8 transport. Its ordinary capability requires `InfallibleLegacyTranscript` (`jolt-akita/src/scheme.rs:679-715`), deliberately excluding the D1 owners.
- The continued ordinary *control source* does transport full approved statement/context/frontend/source-check/normalization blocks in `ordinary_session` (`jolt-prover/tests/akita_accumulation.rs:617-634,773-796,849-858`). It subsequently proves independent complete per-segment PCS proofs (`:637-638`); that is evidence for the binding technique, not the requested Candidate1 one-decider implementation and not an exercised passing test here.
- The separate native/full-ring path already has a checked receiving dispatch: `SegmentPlan::authorize_native_continuation -> Registry::authorize_native -> NativeContinuation::installation` (`segment_emissions/handoff.rs:38-46`; `registry.rs:143-150,349-369`). The concrete receiver checks complete context, parses the seal, checks preceding outgoing/current incoming RAM payload and retains obligations (`segment_native.rs:554-580`; `segment_native/seal.rs:105-234`). The seal retains original Main/source-check and RAM claims, normalization and exact mappings, not merely a normalization digest (`seal.rs:8-17,137-168`; `handoff.rs:158-293`). It is a useful pattern, **not** the requested ordinary K/K v8: that full-ring installation uses K/E2 (`akita-config/src/full_ring.rs:21-23,154-156`).

### Exact small receiving contract

Introduce a checked, non-forgeable-at-the-API handoff produced by replaying the frontend/query verifier and mirrored by the prover. Its canonical augmented instance contains:

`Frame(existing PCS descriptor)`

`|| Frame(frontend mode + exact approved installation + expected statement)`

`|| Frame(all inherited and newly appended ordered frontend evidence + original source roster)`

`|| Frame(deferred-relation seal and scalar index assignments)`

`|| Frame(complete range/query reduction history)`

`|| Frame(verifier-derived complete final endpoint batch)`.

Required checks:

1. Replay all inherited and new frontends, Main source predicates and selected normalization under their *installed* compiler mode; preserve old challenges/mode at a permitted cutover, never reinterpret old digest bytes as new indexed records. Include exact segment order/count, predecessor/source/view aliases, run/program/IO/final-count linkage.
2. Retain original commitment/profile/setup A/B/F provenance, source role, raw-view/permutation/decoder/adjoint and required predicates. Do not coalesce original RAM from decoded equality. All selected deferred leaves must be represented; an auxiliary group may be rejected by the selected no-advice profile but cannot be silently discarded.
3. Freeze the full scalar-index/query roster and horizon before its seal challenges. Reconstruct the final endpoint batch from that checked prefix; compare the actual ordinary statement's commitments/points/values/order/component counts **exactly** with that derived batch before any PCS challenge.
4. Feed this literal augmentation through the shared v8 constructor at the existing producer/verifier seams above. The v8 consumer explicitly dispatches the registered frontend-handoff mode; unknown, omitted, reordered or incompatible modes/sections reject. A logging-only context record (`proof_stream.rs:663-687`) is not a substitute. No old scalar bridge remains an admitted Candidate1 fallback.
5. Accept only after the one complete final root, shrinking suffix, terminal and exact EOF/plan completion. A pending stage8 object or root reduction is not acceptance.

No completed per-segment PCS proof is a handoff field. The ordinary source-view-capable final endpoint operation is a separate existing Candidate1 obligation; this compiler report does not claim the ordinary point API already supports arbitrary original-plane queries or drop those predicates to make the API fit. Merely binding evidence does not prove it valid, fix a legacy compiler theorem or establish source identity.

## 11. Closure table and handoff

|Question|Result|
|---|---|
|Fresh H256 -> actual K scalar law|**Closed exactly:** low16 LE reduced modulo p; atom n_y/N, maximum2/N.|
|Exact conditional entire32-byte answer|**Closed exactly:** uniform low preimage and independent high128 bits, with marked-original/replay preservation.|
|Legacy arbitrary semantic prefix ownership|**False as an unconditional API property:** deterministic site/op/empty/codec aliases. Not a claimed attack on a pinned schedule.|
|Bounded primitive-H256 chain simulation|**Closed conditional theorem (§4):** unique authenticated lowering plus finite guard, graph-loss inventory and full raw tables. Those implementation contracts are not supplied by the generic engine.|
|Current selected compact owner covered by H256 theorem|**No:** it is the H1610/two-attempt inner owner, not Digest/H256 or ordinary K/K v8.|
|Minimal replacement avoiding chain/parser assumptions|**Specified concretely (§6):** typed literal indexed H256 component keys, atomic vector completion, decoded completed history, same modulo law, existing stage protocols reused.|
|Replacement descendant/source-probe theorem|**Closed in the stated ideal model (§7):** `q*sum epsilon_FE + binom(q,2)*2/N`, plus supplied suffix/setup/source-collision and explicit cutoff terms. No chain birthday term.|
|Finite whole-extractor/numerical admission|**Not supplied by source:** actual q/M/work/cutoff/suffix and MSIS inventories must instantiate the equations; current table floor is not such an admission.|
|Inherited frontend -> ordinary v8|**Exact receiving contract and source seams specified (§10); not implemented end to end by current ordinary-v7 or separate H1610 controls.**|

The recommended design decision is the small direct-indexed frontend owner and checked augmented ordinary-v8 handoff, preserving original source predicates and base K-valued semantics. Retaining legacy instead is mathematically possible only with the precise parser/guard/simulator contract and its extra graph term; “the full digest is retained” alone is not that proof. No unsupported concrete-hash knowledge assumption is needed on either stated ideal route.
