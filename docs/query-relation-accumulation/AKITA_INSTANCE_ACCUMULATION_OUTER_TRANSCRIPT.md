# D1 ordinary outer transcript: exact independent-stream compiler

## 1. Decision, anchor, and scope

This note resolves the outer-transcript design obligation in [EXTENSION_MIGRATION §4](AKITA_INSTANCE_ACCUMULATION_EXTENSION_MIGRATION.md). The selected compiler is **`akita/d1/outer-transcript/1`**, using literal, variable-length, injectively encoded complete-prefix keys for an independent ideal stream oracle `H_o`. Its selected concrete instantiation is SHAKE256 applied directly to those complete keys, under the separately stated, protocol-specific, extraction-use Fiat–Shamir heuristic in §10. It does **not** use the inner chained compiler, a scalar bridge, a seed preimage sampler, a debug-state export, or an assumed collision-free digest.

**Unchanged anchor.** D1 accumulates original pre-PCS segment instances, never completed segment proofs; K sources and original `C_archive`/installed `C_static` remain the same sources. The inner T16 profile, full `E2[Y]/(Y^128+1)` two-plane carrier, all-valid weak source paths, full strong checks, final domain reduction, three inner shrinking folds and genuine inner A-only terminal are unchanged. One fixed guest is proved by **one complete ordinary outer proof**, not a pending receipt. The public statement remains `x_o=(D1,authenticated id(I),X)`, with canonical output byte `01` and panic=false checked by the ordinary verifier. This note changes the proposed outer challenge protocol, not those relations or their correctness/performance gates.

The outer carrier is the separately specified [OUTER_CARRIER](AKITA_INSTANCE_ACCUMULATION_OUTER_CARRIER.md): finite no-setup-offload, full two-plane coefficient-row opening, ordinary root nonterminal followed by its genuine direct terminal. This does not inherit the inner fold count, challenge budget, query cap, matrix inventory, or performance estimate. No numerical security, concrete-hash extraction theorem, parity, supported guest geometry, source implementation, or execution evidence is claimed here.

The theorem below is a **compiler and conditional-answer theorem**. Deterministic frontend/source/guest adequacy, carrier extraction and setup/MSIS certificates remain their named obligations; this theorem does not assume CH-O to replace them. Generated numeric site IDs, emission lists and actual capacity values instantiate the semantic rules below; they cannot select a different grammar or change a vector into scalar queries.

## 2. Evidence and source differences

Source pins: Jolt `0791f79a55c3fadc3d327c78a72228696c338468`; Akita `83574331d4e51f8cce1d8689d05592fa4ef4c138`.

* [Pinned Jolt legacy facade](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-transcript/src/legacy.rs#L33-L119): infallible `challenge`, scalar-family distinction, a default vector implemented as a scalar loop, scalar powers from one draw, and a debug-only 32-byte state peek. The new API must expose the semantic vector call atomically rather than mistake that implementation loop for independent stage inputs.
* [Pinned digest sampler](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-transcript/src/digest.rs#L170-L190): both challenge families consume 16 bytes. Merely substituting E2 does not produce uniform E2.
* [Pinned native batching](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-akita/src/native_batching.rs#L152-L199) binds verifier setup, selected schedule row, group count, ordered roles/indices/presence, commitments, points and every evaluation before the bridge. [Pinned bridge](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-akita/src/adapters.rs#L1332-L1350) then compresses that context to one K scalar. §5 replaces precisely that compression, not the preceding bindings.
* [Pinned native extension challenges and contexts](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-transcript/src/proof_stream.rs#L596-L686): coefficientwise extension sampling exists, but `ProtocolSiteId` context records are diagnostic, not absorbed separation. Our site encodings are actual key bytes.
* [Pinned native field sampler](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-transcript/src/proof_stream/sampling.rs#L10-L99): the query limit is not a retry cap; the loop is unbounded and the Keccak branch absorbs its attempt count. Neither behavior is inherited by this version.
* [FRONTEND_MANIFEST](AKITA_INSTANCE_ACCUMULATION_FRONTEND_MANIFEST.md) gives challenge families, arities, actual source-dependent terms and carries; [FRONTEND_EXTENSION](AKITA_INSTANCE_ACCUMULATION_FRONTEND_EXTENSION.md) preserves K/E source separation, curated opening order and exceptional-point behavior. [PROTOTYPE_MODEL](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) fixes D1 and its assumption boundary.
* [Akita, Lemma 3.22](SOURCE_PROVENANCE.md#akita), lines1509–1547, requires complete-prefix injectivity and conditioning the **full** answer of a vector query, with nested trees. Lemmas10.1–10.2 and lines3377–3425 require separately indexed shell-coordinate streams, exact rejected-encoding/unused-suffix laws, and distinguish witness-independent filters from response search. The qmd `lattice-papers/akita.md` retrieval at lines3200–3349 additionally records the ideal-XOF abstraction and schedule admission. [PRODUCT_EXTRACTION §§3–4](AKITA_INSTANCE_ACCUMULATION_PRODUCT_EXTRACTION.md) supplies the independent-outer, fixed-environment composition; [CHAINED_TRANSCRIPT §2](AKITA_INSTANCE_ACCUMULATION_CHAINED_TRANSCRIPT.md) supplies useful native-key cautions, **not** the compiler selected here.

This is a versioned replacement of old concrete transcript bytes. It does not assert a byte-for-byte renaming of the old sponge's compressed state. “Preserve native dependencies” below means preserve the complete ordered semantic public/proof/challenge history and native joint-stream structure required by the ideal proof, not preserve the old hash outputs or sponge-specific retry-count absorption. Raw sampler encodings are oracle answers, not extra public-coin messages; their entire distribution nevertheless remains visible to an oracle-querying adversary and is preserved in §7.

## 3. Exact byte grammar and installed descriptor

All concatenations below are byte concatenations. Literal ASCII domains have no NUL terminator. `u8`, `u32`, `u64` are unsigned little-endian of exactly 1, 4, 8 bytes. Every count/length must fit u64 and the tighter installed cap; checked overflow, unknown tags, inconsistent lengths and noncanonical values reject. No machine `usize`, JSON, serde defaults, map iteration order, native-endian cast or debug formatting is a wire encoding.

Define:

```
B(s)       = LE64(byte_length(s)) || s
List(x[])  = LE64(number_of_items) || B(x[0]) || ... || B(x[n-1])
Pair(a,b)  = B(a) || B(b)
K(a)       = LE128(canonical representative a), 0 <= a < q
E(a0+a1U)  = K(a0) || K(a1)
q          = 2^128 - 2^32 + 22537; U^2 = 5
```

K/E proof decoding rejects noncanonical limbs; it never reduces malformed proof values. Field sampling is separate. Ring E partials use coefficient order `Y^0,...,Y^127` in plane0 followed by that order in plane1, each coefficient encoded K; a scalar E atom still uses `(a0,a1)`. These are distinct registered atom types, never an implicit conversion to subfield packing.

A semantic site path is `List(component[])`; a component is either `u8(0)||B(ASCII_role)` or `u8(1)||LE64(index)`. ASCII roles are nonempty immutable registry strings, indices are declared structural stage/level/group/round/claim positions. They are not timestamps, RNG salts, memory addresses, worker IDs or fresh implementation-session counters. The installation fixes the unique path of every semantic occurrence. A reference to an earlier result is `(site_path, coordinate_index, projection_code, projection_arguments)` with explicit framed fields; projections are registry-defined deterministic reversal, truncation, selection, power or field-linear maps, never fresh randomness. Numeric IDs may replace role strings only in a *new version* or through a proven one-to-one installed encoding; this version uses the role strings themselves.

The installation's canonical descriptor `D` is the ordered tuple, encoded with `List`, of: compiler/version; field polynomial/basis and atom registry; message/site semantic registry; outer carrier/stage descriptor; source and matrix-view registry; actual outer public setup and authenticated guest/program/profile bindings; all dimensions and finite byte/event/site/coordinate/attempt/work limits; deterministic nonce policy; and selected concrete adapter identification. Digest-valued objects required by existing source semantics retain their bytes **and** their authenticated canonical referents in the context when those referents identify proof-system parameters. A collision-resistant identifier is not used as a mathematical injective encoding of arbitrary setup/program data. Installed finite selectors are resolved to their unique authenticated records before encoding.

The context is

```
C = List([ASCII("D1"), CanonicalInstallation(D,I), CanonicalPublicStatement(x_o),
          CanonicalExpectedOutput(01, panic=false)])
```

`CanonicalInstallation(D,I)` includes the actual provisioned public parameters and `C_static` identity/shape/source map; it is not a hash of them. It excludes private archive contents and prover witnesses. Their *original commitments*, metadata and claims enter at the source-defined events. Parameters duplicated by a native instance are encoded again where that instance requires them; there is no destructive “already bound” optimization.

The `Canonical...` names above are structured encodings, not unspecified hash/serialization hooks. Their schemas are part of the frozen pre-provisioning descriptor and use only these constructors: `Bytes` encoded B; fixed u8/u32/u64; canonical K or E; fixed-length ordered tuples encoded List; variable arrays encoded List; `Option(T)` encoded `u8(0)` for none or `u8(1)||B(encode_T(value))`; and a finite tagged union encoded `LE32(variant)||B(encode_variant(value))`. Tuple/union field order is the declared schema order. A source-native object may be preserved as `Bytes` of its exact canonical source wire encoding; its decoder must validate that encoding and the complete encoded bytes are retained, not reconstructed from a lossy projection. No unordered map constructor is admitted. `CanonicalInstallation(D,I)=List([encode(D),encode_I_schema(I)])`; `CanonicalPublicStatement(x_o)=List([ASCII("D1"),B(authenticated_id(I)),encode_X_schema(X)])`; and `CanonicalExpectedOutput=List([B([0x01]),u8(0)])`. The nested B wrappers here are intentional schema fields. Schemas themselves encode as tagged constructor trees with these same length rules; schema tags are 0=Bytes,1=u8,2=u32,3=u64,4=K,5=E,6=Tuple,7=Array,8=Option,9=Union, followed by a List of child schemas (and the declared finite tag/field-name list for Tuple/Union). Required native source codecs and source semantic identities are validated before they are admitted as Bytes. The parameterized family thus specifies one framing algorithm, not an implementer-selected serialization convention.

For avoidance of a schema-wire ambiguity, each schema node is exactly `LE32(constructor_tag)||List(child_nodes)||List(field_specs)`. Primitive nodes have both lists empty; Array/Option have one child and no field specs; Tuple has one child and `B(ASCII_field_name)` field spec per ordered field; Union has one child and `LE32(variant_tag)||B(ASCII_variant_name)` spec per variant, sorted by distinct increasing variant_tag. Schema trees are finite and acyclic. Fixed-size arrays use Tuple or an Array whose exact checked length is an installed dimension. A carry reference is exactly `List([site_path,LE64(coordinate_index),B(ASCII_projection_code),List(encoded_projection_arguments)])`; its allowed argument schema and deterministic formula are fixed by that projection's entry in D. Thus neither schema descriptions nor carry metadata have an implicit or unordered serialization.

Every transcript record is

```
R(kind,site,type,count,payload) =
    u8(kind) || B(site) || B(type) || LE64(count) || B(payload)
```

`type` is the exact ASCII atom/tuple-codec name from D. Counts are checked against that codec; labels and empty byte strings are real payloads. Record kinds are fixed:

| kind | Meaning and canonical payload |
|---|---|
| 1 | Public event: exact public atoms/bytes and labels, in source order. |
| 2 | Proof event: checked canonical prover message, including round polynomial representation/degree, opening payload and explicit length/shape atoms. |
| 3 | Challenge result: `List([RequestDescriptor, canonical ordered output])`; count is the number of algebraic coordinates, not raw limbs/bits. |
| 4 | Protocol transition: `List([from_role,to_role,new_native_instance])`. |
| 5 | Public nonce event: `List([nonce_role,LE64(nonce)])`. |

`P` is the literal concatenation of all completed records. Canonical parsing of each record is unique, including record order and boundaries. A key is

```
k = ASCII("akita/d1/outer-transcript/1") || LE32(1) ||
    B(C) || B(P) || B(RequestDescriptor)
```

`RequestDescriptor` is the ordered `List` of:

1. semantic site path;
2. operation tag `u8`: 1=RawScalar, 2=ScalarCoefficient, 3=RawVector, 4=CoefficientVector, 5=ShellCoordinate, 6=PredicateBits (reserved), 7=ShellBundle (logical admission only, no oracle entry);
3. exact algebraic family descriptor (E2, selected shell, or a fixed bit width);
4. `LE64(algebraic_coordinate_count)`;
5. sampler descriptor including all finite caps and raw layout;
6. stage tree mode `u8(0)=scalar`, `u8(1)=nested`, `u8(2)=flat bundle` and ordered interpolation arities;
7. for a shell coordinate only, `LE64(coordinate_index)`; otherwise the empty byte string;
8. the ordered deterministic carry-reference list used by this stage.

All eight fields are present, even when empty. None is prover-chosen metadata. Stage, family, cardinality, vector/scalar distinction and native scope therefore occupy real bytes. `RawScalar` and `ScalarCoefficient` have the same full E2 distribution here, but different semantic/API families and different keys; they must not be interchanged. `challenge_scalar_powers(n)` is one ScalarCoefficient followed by deterministic powers, not n oracle queries. The selected ordinary outer family uses no PredicateBits or nontrivial nonce search; those tags are reserved and invalid for its accepting schedule.

`H_o(k)` is one independent infinite fair-bit tape for **each byte string k**. Ill-formed/adversarial keys also have tapes, independent of valid ones. Repeating k returns the same tape. This defines the oracle on all keys without an image-recognition assumption. The protocol consumes bounded spans only; the infinite tail specifies the adversary/extractor distribution, not an unbounded production loop.

## 4. Compiler operations and exact stream boundaries

The state is `(C,P,closed_or_failed,completed_results)`. It is not a cryptographic digest. An implementation may retain P in append-only storage and stream its bytes to a hash without copying the entire prefix. `Public` and `Proof` append kinds1/2 after canonical validation, preserving every source event including empty labels. Incoming proof parse failure is sticky. No challenge-dependent message may be moved before its challenge.

`DrawScalar` freezes the current P, evaluates the key with the corresponding scalar-family descriptor, runs the bounded E2 sampler, and appends one kind3 result **after** successful completion. Failure returns `Exhausted` and permanently rejects this execution; there is no valid placeholder E value.

`DrawVector(n)` freezes P once and obtains **one** H_o entry. All n E2 coordinates use the disjoint tape spans in §6. No coordinate result is appended until the entire vector succeeds. Then one kind3 record contains the whole ordered vector. There are no fictitious prover messages, coordinate salts, coordinate keys, or absorptions of sibling outputs inside this vector. This applies to frontend `challenge_vector` calls, and to native tensor/row/range-anchor vectors when present. The legacy implementation's scalar loop is replaced at its semantic vector boundary. A profile cannot silently group separate adaptive scalar calls into one vector.

`DrawShellBundle(n)` freezes P once. It obtains n entries, distinguished only by the declared ShellCoordinate index in otherwise identical descriptors. Each is sampled as in §6. All sibling keys use that **same** P, irrespective of evaluation order or thread scheduling. On complete success append one kind3 bundle record containing the canonical n shell outputs and the bundle descriptor. Failure rejects without continuing the protocol. Changing one coordinate entry leaves every sibling key and answer fixed. The accepted tuple is a product distribution conditional on all coordinates succeeding.

Precisely, a shell coordinate's RequestDescriptor has coordinate_count=1, the common bundle site path and its index u; the shared stage descriptor fixes n and the ordered arities. Its successful bundle-history payload is `List([List([RequestDescriptor_0,...,RequestDescriptor_(n-1)]), List([encode(c_0),...,encode(c_(n-1))])])`, with record count n and type `shell-bundle-result/v1`. For a scalar/vector history the first component remains its single RequestDescriptor and the output encoding is respectively E or `List([E(c_0),...,E(c_(n-1))])`. A shell element's canonical output is `List([coeff_0,...,coeff_127])` in increasing Y-exponent order, each centered signed coefficient encoded as `u8(sign)||LE64(magnitude)` with sign in {0,1} and zero sign required when magnitude=0; this output is distinct from its raw random rank word. This removes any ambiguity about the completion record of several separately indexed entries.

Before sampling any shell coordinates, the registry admits the complete logical `ShellBundle` request with count n and no coordinate index; this binds the registered site, family, sampler, tree mode, ordered arities and carries even when n=0. This admission is not execution and does not advance a scalar slot or consume an oracle entry. For n>0 the coordinate keys and bundle-history payload above remain unchanged. For n=0 the descriptor component is instead `List([ShellBundleRequestDescriptor])`, with descriptor count 0 and no coordinate index; the output component is `List([])` and the record count remains 0. Thus the empty logical result publishes once without sampling a fictitious scalar, and its metadata remains available to native accepting-prefix validation. Rejected admission publishes no completion and permanently poisons the same compiler.

Separate scalar calls retain sequential dependencies: append the completed scalar before building the next scalar key, even when no prover message intervenes. A sumcheck round polynomial is appended before its scalar challenge and the resulting scalar precedes the next polynomial. One scalar's powers, a reversed point or a stored `ProofSession` carry introduce no oracle site or arbitrary new history record. The original kind3 record remains in P and the later stage descriptor references its result/projection. Recomputing the value from that same reference is allowed; resampling it under the later stage's path is not.

A repeated request to the *same frozen* `(C,P,descriptor)` reuses exactly the tape and output. Calling `DrawScalar` a second time after appending the first result is a different native prefix, not a retry of that frozen request. Parallel/interleaved executions sharing an identical prefix and descriptor share answers. Installed structural occurrence indices distinguish genuinely different source positions; they never distinguish two implementations of the same semantic position.

Raw tape bits use index0 first, and bytes use least-significant bit first. `Read(k,j,t)` means bits `j,...,j+t-1` of **the same tape**, not an oracle key with j appended. Candidate, limb and coordinate indices identify offsets below, not extra independent scalar queries. Arbitrary bit reads by an adversary are allowed and accounted for (§9).

### Existing frontend site and carry contract

The complete source ordering is the one in [FRONTEND_MANIFEST §§1–3](AKITA_INSTANCE_ACCUMULATION_FRONTEND_MANIFEST.md), not a reordering by final batch role. In particular:

* Stage0 binds the full preamble (preprocessing, memory bounds, public I/O/panic, RAM and trace size, entry, phase counts, chunk/layout data), then main commitment, U?, T?, field-inc/program objects in the original frontend order. No random setup-selection scalar is inserted.
* Stage1 raw tau is an atomic vector; its final centered-Lagrange coordinate has its own non-multilinear arity. Uni-skip, opening claim, remainder coefficient and adaptive rounds retain their order.
* Stage2 reuses the reversed stage1 remainder tail as product tau-low; only tau-high is fresh. RAM output-reference coordinates are a real fresh raw vector, not a carry. Stage2/3/4/5 member gammas and batch coefficients retain their distinct scalar sites. Every present batch member gets an independent scalar coefficient in declaration order, not powers of one scalar.
* Stage4's empty labeled append `ram_val_check_gamma` survives. Curated opening absorption/aliases remain as in the source; direct trusted/untrusted advice claims carry from stage4 to stage8, not through generic Dory reductions.
* Stage5's canonical-address gamma-cubed term, stage6a Booleanity even powers, fused increments and stage7 decode term retain their actual interpolation degrees. `NoChallenges` members get no synthetic draw.
* Stage6a bytecode draws are one outer gamma plus five stage gammas, with existing field-stage reuse. Booleanity cycle and reversed/truncated address references come from stage5, with zero padding draws for legal k=4,8. Every 6a→6b and 6b→7 `ProofSession` carry retains its original origin, value and permutation; no stage-name-based resampling.
* Stage6b committed-bytecode eta is independent of carried bytecode gammas. Precommitted head-aligned scheduling, dummy windows and stage7 optional address phases remain unchanged.
* Stage8 prefix packing first binds logical arity, capacity/layout, logical point and all evaluations, then draws the actual selector vector. Field-inc uses the explicit zero prefix, not random padding. Common-point validation remains transcript-neutral.

The member degrees/arities are those of the manifest (including instruction ReadRAF arity4, Booleanity arity `2P-1`, Hamming/decode arity `e_H+1`), not an invented binary bound. This note changes no exceptional-point rule: full E2, including zero and interpolation nodes, is sampled, and the existing exact node/zero-scale branches remain required.

## 5. Full frontend-to-native transition

After the existing grouped-statement binding, append kind4 at the unique `frontend/to-native-pcs` site with from=`jolt-frontend`, to=`ordinary-native-pcs`, and the full canonical native group instance. That instance contains actual setup/schedule selection, source maps/shapes, ordered roles/presence/counts, original commitments, all E2 points and ordered evaluations. Validate equality between the native instance and the verifier-derived frontend group ledger. Preserve paired point reversal and evaluation-bit reversal at the Jolt/Akita indexing boundary; serialize the actual native instance after that checked conversion.

Then continue **the same `(C,P)` record history** into native proof events under the native scope's structural paths. Native instance/public records are appended where required by the native protocol; they do not replace the frontend history. There is no bridge draw, no 16/32-byte state export, no separate transcript reset, and no digest of P pretending to equal P. The old `precommitted-group-batch/v4` label can survive as a recorded source label, but the actual namespace is this version's key grammar. The deletion of the old bridge removes a seed derivation, not an algebraic degree-one test; it has no replacement interpolation-error row.

Original group order remains U?, T?, field-inc **or** committed bytecode/image groups, then OneHotTrace. Original source handles and owner-bound hints stay with those groups. The full outer native proof must complete; no pending status is accepted. The selected frontend has no later challenge depending on the terminal proof, but every terminal equation and exact proof consumption is still verified. Any future post-PCS challenge must retain the full native events; starting over with only a receipt digest would be another protocol version.

For the carrier's new sites, the descriptor contract is: exact structural path and ordered predecessor events; scalar/vector/shell-bundle mode; scalar family; coordinate count/order and arity/tree mode; source-handle/view and claim-role bindings; sampler/caps; carry origins/projections; and the boundary at which the entire returned challenge is appended. In the chosen carrier this covers original-source domain/raw multipoint normalization, root opening/D/H payload before its shell tuple, successor commitment and strong coefficient-row/range/fused-round messages, and terminal A images plus complete clear E2 partials before its shell tuple. There is no setup-offload stage, inherited subfield tensor-prefix site or phantom recursive full-commitment fold. The carrier's exact algebraic sequencing determines these descriptors; the compiler does not move a commitment, payload or successor after the challenge that is meant to bind it.

The agreed carrier sequence and its compiler modes are:

1. For each original raw source, bind its original handle, exact source view, `SourceDomain` variant and physical zero mask. Draw the source-domain anchor `tau_src` as one RawVector, then `eta_zero` as one ScalarCoefficient; there is no `eta_plus` site. For main `BooleanWholeGrid`, check `sum_x eq(tau_src,x) Raw(x)(Raw(x)-1) + eta_zero Z_tau(x)Raw(x)=0` by degree3 sumcheck. The actual main coefficients have the direct Boolean alphabet; required holes/inactive entries are zero, while virtual padded polynomial slots are zero by the source view. For dense archive/static `BalancedBase32`, check `sum_x eq(tau_src,x) Q32(Raw(x)) + eta_zero Z_tau(x)Raw(x)=0`, where `Q32(T)=product_(a=-16)^15(T-a)`, by degree33 sumcheck; no Boolean-support term is imposed on those sources. `Z_tau` is the carrier's masked equality MLE for the bound required-zero mask. The `SourceDomain` enum fixes the polynomial roots, view and mask before challenges; it cannot replace main's Boolean identity by a redundant uniform base32 check. Both variants use scalar round challenges and retain their terminal raw evaluation on the same original source. Existing frontend Boolean/source checks remain unchanged.

   **Mask authority:** the SourceDomain header binds the actual verifier-authenticated **outer** source/layout metadata; neither inner segment length/Nseg nor private guest execution data defines a public outer mask. Main BooleanWholeGrid covers every physical N-cycle coefficient. RequiredZero includes only registered physical omitted-address slots/packing holes and an inactive suffix **if** the outer metadata publicly authenticates that suffix; otherwise all padded N cycles remain ordinary Boolean-domain cells. Virtual raw polynomial/digit padding is zero by the fixed decoder map, not an extra set of physical cells. The preceding “required holes/inactive” clause is restricted exactly by this authority rule.
2. Bind all raw endpoint/domain claims for one original handle; draw its raw-normalization coefficients as one CoefficientVector, then run its degree2 adaptive sumcheck. Its weights are the exact decoder adjoints, including dense/one-hot permutations and holes. Keep the resulting two K endpoint rows separately. There is **no** extra root scalar-claim batching draw after D/H.
3. Bind the root opening and complete D/H payload, then draw the root ShellBundle. Bind its full successor TerminalInnerState (the complete native W, not a response-only projection) before later strong-check draws.
4. Draw the range anchor `tau0` as a RawVector, then coefficient-row `rho` as a CoefficientVector. There is no ring-evaluation alpha. Preserve the base32 product range pipeline with inner degrees(4,4): canonical uncompressed degree5 round messages, four intermediate evaluations bound before the three fresh affine coefficients `(xi1,xi2,xi3)` drawn as one CoefficientVector, with the first batch coefficient deterministically one. Every actual subsequent range-stage instance repeats this rule under its structural path.
5. Once the virtual frontier is bound, draw `(gamma,eta_bin,eta_zero)` as one CoefficientVector. These are independent coordinates, not powers, and its extraction is nested. Run the degree3 fused relation sumcheck, preserving all row/range/Boolean/zero terms, and bind the raw successor evaluation v.
6. At the genuine terminal, reabsorb the same A images, point and v together with **all** clear two-plane partials. Only then draw its ShellBundle. Parse the bounded signed response and execute the exact A, two-plane and scalar terminal equations. Those checks are mandatory even when no further challenge follows.

Vectors in this list are atomic one-entry field vectors; the shell bundles alone use separate coordinate entries. Source masks, original-handle roles, full successor layouts and the algebraic identities remain the carrier's authoritative definitions. This sequencing was exchanged with the carrier design owner; later code-generated IDs/counts must realize it, not select an alternate placement of a challenge or intermediate evaluation.

## 6. Bounded production samplers

All sampler caps are positive finite public integers in D, fixed before proving and checked at installation admission. They are independent of the witness and of oracle answers. No value is inherited automatically from the inner `A=2`, shell256-attempt or Q32 policy. Their numerical instantiation is a later profile artifact; the algorithm for every value is fixed here.

### 6.1 Full E2

Let `A_E` be the per-K-limb cap. In a scalar tape, limb `l in {0,1}` owns `128*A_E` bits beginning at `128*A_E*l`. In an n-vector tape, coordinate u, limb l owns the span beginning at

`offset(u,l)=128*A_E*(2*u+l)`.

Candidate t (`0<=t<A_E`) is the next 128-bit little-endian unsigned integer at `offset+128*t`. Accept the first candidate `<q` as that limb. If none exists, return Exhausted. Sample both limbs independently and return `a0+a1U`. Reserved unused candidates after the first acceptance are not consumed by the production decoder and cannot spill into the next limb/coordinate. The oracle still defines them. There is no modulo fallback, default zero, nonzero-gamma rejection or exclusion of interpolation nodes.

Put `p=q/2^128`, `a_K=1-(1-p)^A_E`. Each produced limb is exactly uniform K and its success event is independent of its value. A scalar succeeds with probability `a_K^2`; an n-vector with `a_K^(2n)`; conditional on success its law is exactly uniform `E2^n`. Honest exhaustion is at most `2n(1-p)^A_E` by union bound. This is a completeness bound, not a statistical bias term in an accepted field value.

### 6.2 Selected outer full-ring shell

The carrier selects degree128 and the same algebraic shell of 24 magnitude-one and 16 magnitude-two nonzero coefficients, with 40 independent signs. Its one-norm is56 and maximum coefficient magnitude2. This is **not** a reuse of the inner raw encoding or challenge budget.

Let

`M = binom(128,40) * binom(40,24) * 2^40`, `b=ceil(log2 M)`.

`RankShell/v1` is the following explicit bijection. Enumerate 40-element subsets of `{0,...,127}` in lexicographic order of their increasing tuples. For an increasing support tuple, enumerate 24-element subsets of its 40 positions lexicographically; those positions receive magnitude1, the others magnitude2. Enumerate sign masks `s in [0,2^40)`; bit j gives sign `(-1)^bit_j` at the jth increasing support position. Set

`rank(c)=((support_rank*binom(40,24)+magnitude_one_rank)*2^40)+s`.

Unranking uses exact integer binomial counts: at each increasing-tuple position scan the candidate next index in increasing order, subtracting the binomial number of completions until the residual rank lies in that interval. Then assign magnitudes/signs as above. Ranking is the inverse sum of skipped intervals. All loops have bounds128 or40 and exact finite integer widths determined by M. This is an implementable bijection, not a request to invert a hash or enumerate M elements.

Each ShellCoordinate tape has `L_S` candidate slots of b bits, slot t at offset `b*t`. Decode its b-bit little-endian integer v. If `v>=M`, reject that slot. Otherwise unrank and apply the installed witness-independent fixed filter F; for the **selected outer family F is identically true**. Accept the first successful slot or return Exhausted after L_S. No Fisher–Yates integer-rejection loop exists in this version. The selected outer nonce budget is one (canonical nonce0 if an explicit nonce field is required), with no response-dependent retries or proof-of-work search.

For clarity, the same specified algorithm admits a separately certified fixed subset `C_acc={c:F(c)=true}` in a future installed family; it must be nonempty, efficiently recognizable and supply a cardinality/acceptance certificate. It is not enabled in the selected unfiltered family. Let `h=|C_acc|`, `beta=h/2^b`, `a_S=1-(1-beta)^L_S`. Conditional on success the output is exactly uniform C_acc, since every accepted c has one rank word and each slot gives it mass `2^-b`. Here `h=M` and `beta>1/2`, so honest failure is `<2^-L_S` per coordinate. Distinct shell entries are independent and bundle success is the product of their a_S values.

`L_S` counts **raw b-bit slots**, not completed shell draws. Thus the paper's `alpha=h/M`, `1-(1-alpha)^L` expression is not copied literally. Incorporating out-of-range integers changes alpha to beta as above. The accepted family, its unit-difference/collision conditions and its soundness denominator are unchanged. The bounded raw encoding replaces the paper's almost-surely terminating inner decoder, and §7 proves the precise adjusted conditional law.

Every failure propagates through fallible transcript, stage, adapter and top-level proof APIs and is rejection. Proof verification cannot panic, continue with zero, retry indefinitely, silently change caps, or report success with a sticky failed transcript. A prover may restart a complete attempt only within its declared total resources; such restarts and their oracle queries are not free completeness repair.

## 7. Exact conditional raw-tape sampler

The extractor programs **ideal complete entries**, never a concrete SHAKE input/preimage. The following describes an exact distribution on the entire infinite tape: materialize only inspected bits, and maintain the remaining independent bits lazily. Existing observations at other keys are unchanged. Observations to the selected key in a replay come from the programmed complete answer, as in the resettable oracle experiment; it is not permissible to leave an earlier observed raw word inconsistent with the replacement.

### 7.1 A generic finite-slot lemma

Take L independent uniform b-bit words, let `A subset [0,2^b)` be the accepted words, let h=|A|>0, and suppose the decoder maps A bijectively to accepted values. Put `beta=h/2^b` and `a=1-(1-beta)^L`. For prescribed accepted value c, with word w(c), the exact law conditional on decoder output c is:

1. Draw first-success slot `J in {0,...,L-1}` with probability `beta*(1-beta)^J/a`.
2. For each t<J draw a word independently uniformly from `[0,2^b)\A`.
3. Set slot J to w(c).
4. Draw every slot t>J uniformly from `[0,2^b)`; every other unused bit, padding bit and infinite suffix is independent fair randomness.

For beta=1, J=0 and there are no rejected slots. Empty-complement cases therefore never invoke an impossible sampler. To implement the general fixed-filter case without counting or unranking the complement, sample a uniform full L-slot tape until its decoder succeeds, retain J and its rejected prefix/unused suffix, and replace its accepted word by w(c). The accepted value is independent of those retained objects, so this is the same conditional law and costs O(L/a) expected slot/filter operations. Production remains strictly bounded; this **extractor-only** expected loop is truncated under §9 when constructing a bounded SIS adversary.

Proof: a particular prefix of J rejected words, accepted word w(c), and any specified suffix has uniform original tape mass. Conditioning on output c normalizes by `a/h`. The displayed J distribution and uniform factors give precisely this normalized mass. Equivalently the accepted word, conditional on success, is uniform A independently of J, rejected prefix and unused suffix. Replacement preserves that product law. This proves the distribution of **raw words**, not merely decoded c.

### 7.2 E2 and vectors

For each prescribed K limb use the lemma with `L=A_E`, `b=128`, `A=[0,q)`. This gives a truncated geometric rejected-candidate count; rejected candidates are uniform in `[q,2^128)` and the accepted candidate is the canonical limb itself. All remaining candidate slots are uniform, even if never read by the verifier. The two limb tapes are independent. Applying this to a prescribed E2 coordinate preserves both coordinates' full support; setting only a base limb or hashing a scalar seed would not.

For an atomic field vector and a prescribed algebraic prefix `(c0,...,c_{v-1})`, condition their limb spans as above. Conditional on the vector succeeding, draw the remaining n-v values independently uniformly from E2 and condition their spans as above as well. This is exactly the full answer conditional on success and that coordinate prefix. If the query being simulated is not conditioned on full-vector success, leave the remaining spans genuinely uniform, including their possible exhaustion. The extractor must use the appropriate event; it must not accidentally erase failures. Coordinates fixed by a surrounding nested-tree prefix remain those values, while suffix coordinates may differ between complete suffix trees. There is still one oracle entry and one vector delivered to the prover.

### 7.3 Shell coordinates

Use the lemma with A equal to accepted rank words. For the selected shell A=`[0,M)`, rejected words lie in `[M,2^b)`, the accepted word is the unique rank(c), and every unused candidate and suffix bit stays uniform. For a certified fixed filter, A additionally excludes in-range ranks failing F; the same lemma applies with beta=h/2^b. A proof that only replaces the sparse coefficient vector while retaining an incompatible old rank word is invalid.

To fork a flat shell bundle, replace the selected coordinate entry by this conditional tape and leave every other coordinate's entry fixed. Their keys do not include the replaced answer. To resample a full accepted bundle, apply the sampler independently to all coordinates. No sampler attempts, signs, unused high bits or rejected candidates become deterministic zero. Unlike the old byte-oriented Fisher–Yates codec, RankShell/v1 has no unused sign-byte high bits: exactly40 sign bits are represented in the accepted rank, and all higher/raw-unused tape bits obey the uniform rule above.

### 7.4 Abort and adversarial search

Sampler success masks have probabilities independent of the accepted values. One can sample those masks and ancillary encoding randomness together with independent uniform accepted values, using the conditional laws above; an exhausted mask simply rejects that attempt. This supplies exactly the bounded fixed-filter/abort interface of Akita's tree argument. The accepted-value interpolation denominators remain `q^2` or h; they are not multiplied by `a_K`, `a_S`, or by a response-conditioned acceptance probability. Abort affects honest completeness and conditional-sampler work, not a bias toward any accepted value.

If an adversary probes raw tapes, chooses a statement, retries a commitment or selects among candidates using those answers, the resulting selected value need not be uniform. That adaptivity is handled by the original-query-index support and Q in §8, not by asserting uniformity after conditioning on acceptance. A response-dependent good-nonce set must never be substituted for a fixed C_acc. The selected family has no such nonce grind; malicious whole-proof search is still charged.

## 8. Prefix preservation and mixed-tree theorem

**Proposition (exact key identity).** For valid requests, equality of emitted keys implies equality of the full context, every preceding public/proof/challenge record including its order and codec, and the complete request descriptor. Conversely, identical native semantic prefix and request yield identical keys, regardless of replay, process, worker, storage layout or number of previously failed implementations.

Proof. The literal domain/version is fixed. Length framing uniquely recovers C, P and the descriptor. Recursive framing and fixed kind tags uniquely recover each record/atom, including empty messages, type, site and count. Canonical field decoding is injective. Installed paths are deterministic structural names, and C resolves parameter selectors to their authenticated contents. There is neither digest compression nor arbitrary salt. For a vector request its siblings have no records in P; for a shell bundle the shared P is frozen and the only coordinate discriminator is the declared index. At a later request the complete preceding challenge result is in P. The frontend/native transition appends, rather than overwrites, that prefix. Induction over the operations proves all claims. ∎

The source-to-record map is a deterministic instrumentation rule: each original public append and prover message becomes its corresponding typed event in the original order, each scalar operation becomes its distinct family, each semantic vector call becomes one atomic vector, and original carry/power operations stay deterministic references. Native shell-coordinate groups are emitted against a shared frozen prefix. New carrier operations obey §5's metadata contract. Consequently every original semantic dependency remains recoverable from the emitted key, including setup, original statement, commitments, earlier polynomials, all completed challenge vectors and carried draws. The adapter cannot forget the frontend prefix. Source installation still must establish that its generated event table is the instrumentation of *all* selected branches; a missed append is an implementation violation, not a permitted alternative compiler.

**Theorem (outer ideal-stream compilation).** Fix an admitted finite descriptor and the actual independent setup experiment. Suppose its oracle-free frontend/carrier/guest protocol has the required deterministic complete mixed-tree extractor, with stage coordinate spaces S_ru, arities k_ru and admitted polynomial tree product. The compiler above supplies PRODUCT_EXTRACTION H2 and the conditional-answer portion of H5 exactly. For an original reset execution with at most Q charged H_o queries, the mixed-tree construction has the same error

`epsilon_tree <= (Q+1) * sum_r sum_u (k_ru-1)/|S_ru|`,

with `b_r=1+sum_u(k_ru-1)` for a flat shell stage and `b_r=product_u k_ru` for a nested field-vector stage. Scalar stages have one coordinate. The complete tree cost is `product_r b_r`, not a sum of error charges and not a product of fake scalar query counts.

Proof. Fix the initial oracle-independent description/coins and the independent environment (for D1 the inner H/W/P algorithms may simply be inlined as fixed deterministic code). Distinct literal keys index independent tapes. Identical keys are identical histories by the proposition; changes to one table entry cannot affect the original deterministic execution before its first access to that entry. Preserve the **original** selected stage input even on suffix-extraction failure. The support is at most Q queried inputs plus the possible unqueried output input, as in Akita Lemma3.22; cheap index/prefix mismatch is tested before expensive continuation extraction. §7 supplies the exact full-entry sampler for every prescribed coordinate prefix. Atomic vectors remain one query and are handled by nested interpolation, while independent shell entries allow flat sibling changes without contamination. Bounded aborts use the independent masks/conditional raw encodings just proved. The weighted stage recurrence is therefore

`success_r >= success_{r+1}-(Q+1)e_r`,

`work_r <= b_r*work_{r+1}+(b_r-1)*(Q+1)*gamma_r`,

where gamma includes actual prefix processing, replay and conditional sampling. Induction preserves the saved original statement and prefix, including the full frontend history at native stages. The assumed deterministic accepting-tree algorithm finishes the extraction or identifies its named matrix collision. No theorem for programming the inner component oracle or inverting SHAKE is used. ∎

This theorem supplies no missing deterministic source/guest lemma and no unconditional bound on its tree product. In particular, frontend degrees, new raw-multipoint/domain sites, every strong-check round and the genuine terminal all belong to the final admitted list. Algebraically distinct rows cannot disappear because their challenges share a type. The source's complete decommitment—not necessarily the honest canonical short state—remains the extraction target.

## 9. Strict admission and resource ledger

An installation supplies finite maxima for: canonical context bytes; prefix/event/proof bytes; number of events/sites and coordinates; dimensions and all integer widths; A_E and L_S; oracle-query and raw-bit work; and the complete mixed-tree product and conditional-sampler work. Every cap is enforced with checked arithmetic before allocation/decoding. In particular the native handoff does not authorize unbounded duplication of context or proof bytes.

The original-execution ledger distinguishes:

1. **Q:** H_o-namespace entries/requests of the original adversary including recorded initialization/auxiliary generation, all statements/commitments/restarts, malformed keys, and any allowed candidate search. One atomic vector request is one complete entry, not 2n limb queries. A shell bundle costs its actual coordinate entries. Repeated stream accesses may conservatively be counted again; they never create independent answers. The possible unqueried output challenge is the theorem's explicit +1, not a free general verification budget.
2. **B_o:** raw bits materialized/read, including rejected and unused bits the adversary explicitly asks to see. A range ending at bit j in the selected sequential concrete XOF costs squeezing through j, not one fixed-cost call. Honest E-vector reserved extent is `256*n*A_E` bits and shell extent `b*L_S` per coordinate; production can skip querying unused slots mathematically but concrete traversal costs remain charged.
3. **L_key:** sum of bytes of keys constructed/hashed, not merely proof bytes. Literal complete-prefix keys may cause quadratic total absorption work. This cost is retained, even when it defeats a performance target. Streaming the key avoids an unnecessary full copy; cloning an already absorbed exact prefix can save work only when byte-for-byte equivalent and charged for its actual state operations.
4. **Ordinary work/storage:** field/ring operations, exact integer rank/unrank, parsing/canonical validation, retained histories, native/source map work, original proof verification and inner guest/environment computation. Ho's formal oracle call does not make all of these free.

Strict honest production bounds follow from the event caps and finite samplers; there is no loop with a success-only exit. Honest abort is separately union-bounded using §6 across the actual draw list. The actual outer Q and whole-extractor costs are not the inner profile's Q32 nor its honest sample count.

For extraction, add all replay queries, tape-programming bits, key bytes and environment work across the complete tree. Conditional E/shell samplers cost O(`A_E/a_K`) per K limb and O(`L_S/a_S`) per shell coordinate, with their actual bit-width/unranking overhead. In the selected unfiltered shell `a_S>=1/2` for L_S>=1 and `a_K>=q/2^128`; these factors are bounded without an unproved filter-density estimate. The admitted tree polynomial is still required. A global strict cutoff `L_ext` on **all** extraction work (including initialization and the original run) yields a bounded MSIS reduction with a separate tail-failure term. Expected work can bound that tail by Markov; it is not substituted into a worst-case runtime assumption. The original Q appears in the statistical tree factor; all fork queries/work appear in the computational runtime. No claim of a practical extractor follows.

## 10. Setup, concrete SHAKE256, and assumption boundary

Fix a pre-provisioning descriptor J independently of H_o; sample `I=Provision(J;r)` **inside** the experiment, preserving the same I, original sources and saved output through that execution and its extraction. The joint outer matrix law and shared matrix views are those of the carrier/setup certificate. Provisioning randomness, inner setup/compiler namespace and H_o are independent in the ideal experiment. Merely conditioning on a single favorable installed key is not a per-key MSIS theorem. Oracle-correlated initialization is handled as recorded bounded computation, not arbitrary advice about unqueried tapes.

The concrete adapter is fully determined:

```
ConcreteTape(k) = SHAKE256(k), standard SHAKE256 domain padding and squeeze
```

The input is the literal §3 key, not `SHAKE256(hash(k))`, a K/E scalar, the inner1610-bit component input, an old native group seed or a debug sponge state. Bits/offsets are as in §4. Arbitrarily long admitted keys are absorbed as a stream; the implementation may cache exact common-prefix sponge snapshots privately but cannot export them as authenticated context or rely on their injectivity. Reset/repeat starts from the same key; parallel output chunks must equal standard sequential SHAKE256 output. Context and key-length costs, Keccak permutations, squeeze-through-offset cost, memory and any cached-state copying are included in the ordinary resource ledger. A selected concrete cap prevents requesting an infeasible suffix for free.

Real namespace separation is the actual ASCII domain/version and typed records in the input bytes. Setup and inner H_m use their separate specified algorithms/domains; they do not call this outer namespace. Because the inner component and SHAKE may share a permutation family, domain separation in a concrete implementation is **not** a proof of statistical independence. That is part of the explicit concrete assumption, not a hidden premise of the ideal theorem.

Call the unproved concrete premise **D1-Outer-FS/1**: for this precise complete outer protocol, literal-key SHAKE256 adapter, setup/auxiliary class, finite capacities and full derived extraction/composition resource range, the concrete Fiat–Shamir instantiation supports the **same-original-execution extraction/composition consequence used by D1**, up to an unspecified loss/resource translation. This is the ordinary multi-round concrete-FS extraction-use heuristic already exposed in PROTOTYPE_MODEL §4.3. It is not implied by D1-NGP, collision resistance, pseudorandom output on a few tests, language soundness, or the ideal mixed-tree theorem. No numerical value is assigned to its loss. It carries the concrete outer knowledge content rather than avoiding that content by renaming it.

In particular, the ideal key map is *mathematically injective* because it contains the literal prefix. SHAKE's finite state need not be injective; this note neither assumes exact hash injectivity nor adds a collision-resistance term and calls that an exact proof. Replacing ideal programmable tapes by these concrete outputs has no proven reduction here. This concrete heuristic must be admitted separately for implementation experimentation, and does not waive any deterministic source/guest, supported-profile, setup, correctness, performance or T16 acceptance gate.

## 11. Integration boundary

The mathematical deliverable is the selected grammar/compiler, finite E2 and RankShell algorithms, exact conditional raw-tape law, complete-context handoff and prefix-preservation theorem above. The required implementation is a clean versioned cutover of transcript APIs and all frontend/native callers, with fallible exhaustion and atomic vector operations; legacy scalar-seed bridging and debug-state handoff cannot remain as alternate accepting paths.

Later generated artifacts are: concrete finite descriptor/caps and registry strings, full source emission/carry table for the compiled guest and carrier, canonical codec/bit-offset vectors, profile/tree/work calculations, and implementation correctness/performance evidence. Their role is to establish that the installed implementation realizes these semantics, not to decide them. This work ran no builds, tests, formatters, benchmarks, proof runs or source implementation. Independent review should check the emitted operation map, byte/bit codec, rejection/exhaustion behavior, native transition, carrier stage agreement and exact conditional distribution before source cutover.
