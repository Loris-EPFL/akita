# Modern Jolt/Akita one-final receipt integration contract

**Current user scope supersedes the historical wrapper selection below:**
direct verification of the actual native accumulation certificate is selected;
an O(T) certificate is allowed. A final ordinary Akita proof or another justified
SNARK wrapper is optional and is not a native completion prerequisite. Retain
actual pre-PCS instance accumulation, execution/transition linkage and the full
native final decider. Wrapper-specific contracts below apply only if that
wrapper is separately selected.

Read-only source audit, persisted by the integration owner. The selected deployment is the verifier-provisioned original lattice `C_static` contract in [receipt setup](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SETUP.md); the candidate ranking in §7 is historical investigation, not a conflicting setup selection. The appendix gives the exact capacity-based advice contract.

## 1. Scope, invariant, and evidence level

Source investigation only: no edits, Git operations, compilation, tests, formatting, tracing, benchmarks, or proof generation were performed. References below identify inspected source locations; the presence of tests is not a claim that they ran. Source revisions are the supplied Jolt `0791f79a55c3fadc3d327c78a72228696c338468` and Akita `83574331d4e51f8cce1d8689d05592fa4ef4c138`; this investigation did not independently query Git.

The invariant is the approved native archive relation under the approved application, profile, setup roles and source maps, followed by its complete final decider. Accumulation happens before complete per-segment PCS proofs. A receipt changes delivery, not the inner predicate. It must not replace the archive with a final pending opening, completed-proof residual accumulation, a Merkle construction, or stronger emulator/halt/advice semantics.

The reviewed inner construction is finite T<=16, K sources, E=K[U]/(U²−5) scalar challenges, four original handles, P_main=56, receiver capacity 2^22, three final shrinking folds and an actual A-only terminal. The ordinary outer proof is a different workload and needs its own setup, manifest and composition accounting. The inner 33-role registry is not an outer proof catalogue.

The current experimental wrapper is [D1](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md), with its [direct guest](AKITA_INSTANCE_ACCUMULATION_GUEST_SPEC.md), [mixed-field migration](AKITA_INSTANCE_ACCUMULATION_EXTENSION_MIGRATION.md), [outer compiler](AKITA_INSTANCE_ACCUMULATION_OUTER_TRANSCRIPT.md) and [full-ring outer carrier](AKITA_INSTANCE_ACCUMULATION_OUTER_CARRIER.md). Independent and integrated design reviews permit implementation with the original source, generated-profile and execution/performance gates retained. The old graph/quotient interface remains historical research, not this selected cutover.

## 2. What examples/recursion actually does

### Real proving exists, but not the desired interface

`jolt/examples/recursion/src/main.rs:320–401` collects completed independent guest proofs. It preprocesses the inner guest, calls `jolt_sdk::prove_program`, verifies each proof, and serializes the inner verifier preprocessing, a u32 count, and each proof/device pair. The earlier `program.trace` is not the proof: `prove_program` is a separate call.

`run_recursion_proof` at approximately lines 504–565 builds `recursion-guest`, selects function `verify`, enables std, and preprocesses it. `RunConfig::Prove` calls `jolt_sdk::prove_program` for the outer guest and then `jolt_verifier::verify`. `RunConfig::Trace` and `TraceToFile` explicitly skip proving and verification and reduce the preprocessing trace bound to zero. The CLI `Verify` branch selects `RunConfig::Prove`; the CLI `Trace` branch selects one of the tracing branches (`main.rs:641–699`). Thus it is incorrect to describe all modern recursion support as trace-only.

However, the real proving branch only logs `is_valid` and the decoded return value. It neither requires `rv == 1` nor converts a failed outer verification into a rejecting delivery API; it does not persist a final receipt there. A proof of a guest returning zero is still potentially a valid Jolt proof of that output.

### Reuse defects

`guest/src/lib.rs:16–80` reads preprocessing supplied in the archive, reads count n, verifies n completed proofs and returns `all_valid as u32`.

* n=0 returns 1.
* There is no count ceiling.
* Each record requires its own bincode decoder to consume the record, but no final `offset == data_bytes.len()` check exists.
* The guest accepts the supplied verifier preprocessing as the program/setup to verify; it does not compare it with an approved application/profile/setup.
* It does not compare the supplied devices with a declared application run statement, link segment boundaries, or reject reordered/dropped segment updates.
* It verifies independent complete proofs, not the selected pre-PCS accumulation archive.
* Normal mode postcard-serializes the entire proof archive into **public input** (`main.rs:596–619`). Embedding mode generates `embedded_bytes.rs` from **all** archive bytes (`main.rs:404–445, 583–595`), despite a diagnostic suggesting only preprocessing should be embedded. It changes the guest image per archive.
* Both supplied recursion memory profiles set trusted/untrusted advice capacities to zero (`main.rs:142–188`).

The example is useful proving and guest-runtime plumbing, not an acceptable receipt design as-is.

## 3. Backend and field separation

### SDK aliases are Dory, not Akita

`jolt/jolt-sdk/src/host_utils.rs:89–123,182–196` sets:

* `PCS = DoryScheme`, `VerifierPCS = DoryScheme`;
* `F = Fr`, `VerifierField = jolt_field::Fr`;
* `VerifierVC = Pedersen<Bn254G1>`;
* `JoltProverPreprocessing = GenericJoltProverPreprocessing<PCS, VerifierVC>`;
* `RV64IMACProof = JoltProof<VerifierPCS, VerifierVC>`;
* trusted advice commitment/hint aliases are Dory types.

The protocol transcript selects Poseidon, Keccak, or the legacy Blake2b facade by features; the default is `LegacyBlake2bTranscript<Fr>`. `preprocess_program` and generated `preprocess_prover_*` call the Dory constructors. `prove_program` ultimately calls `jolt_prover::dory::prove` (`host_utils.rs:273–382`). Both recursion Cargo manifests select these SDK interfaces.

`jolt-sdk/Cargo.toml:17–69` has no Akita SDK feature or aliases. The `jolt-prover` Akita feature explicitly compiles out the Dory module and changes proof shape; Akita and zk are mutually exclusive (`jolt/crates/jolt-prover/Cargo.toml:40–66`). Simply enabling Akita in a transitive dependency while retaining generated Dory SDK host helpers is not a supported cutover.

### Existing native Akita ordinary-proof types

`jolt/crates/jolt-prover/src/akita/preprocessing.rs:27–30` defines:

* `AkitaVc = NoVectorCommitment<AkitaField>`;
* `AkitaTranscript = LegacyBlake2bTranscript<AkitaField>`;
* `AkitaProverPreprocessing = JoltProverPreprocessing<AkitaScheme, AkitaVc>`;
* `AkitaVerifierPreprocessing = JoltVerifierPreprocessing<AkitaScheme, AkitaVc>`.

`AkitaField` is the fp128 preset field, `Prime128OffsetA7F7` (`jolt-akita/src/adapters.rs:38`; `akita-config/src/proof_optimized/fp128.rs:7`). The fp128 presets pass `Field, Field` into their configuration macros (`fp128.rs:110–171`); their present extension type is K itself. The Jolt dense and one-hot delegates inherit that type (`jolt-akita/src/configs.rs:28–33`).

The guest can implement K/E arithmetic as ordinary RISC-V integer code regardless of the outer proof field. That does not convert a Dory receipt into a lattice receipt, change outer frontend challenges to E, or establish comparable security automatically.

## 4. Exact available whole-program Akita route

The existing recipe is shown in `jolt/crates/jolt-prover/tests/akita_e2e.rs:66–143`, with program/trace preparation in `tests/support/mod.rs:102–174`. These are source examples, not exercised results.

1. Build the **fixed** guest using `jolt_host::Program`, the selected function and an explicit std/profile/memory configuration. Obtain `JoltProgram` through `JoltProgramSource::build_jolt_program`.
2. Construct `JoltProgramPreprocessing::new(expanded_bytecode, memory_init, memory_layout, entry_address, max_padded_trace_length, instruction_profile)` from the approved program, not from untrusted archive contents.
3. Supply `TraceInputs::new(public_input_bytes, untrusted_advice_bytes, trusted_advice_bytes, memory_config)` to `TracerBackend`. The ordinary classic path uses `trace_compact`; field-inline fixtures use the rich trace path.
4. Derive `ProverConfig::derive_compact::<AkitaField>(rows, layout, min_bytecode_address, program_image_len_words, max_padded_trace_length)` or the appropriate rich-trace equivalent.
5. Use `akita::preprocessing::preprocess_full_with_advice(&artifacts, program_preprocessing, &config, has_untrusted, has_trusted)`. There is also `preprocess_committed_with_advice`; full-public-program is the simpler fixed-guest starting point.
6. If fixed setup is supplied as trusted advice, create its authentic commitment once through `akita::preprocessing::commit_trusted_advice(&preprocessing, exact_postcard_trusted_bytes) -> AdviceObject<AkitaScheme>`. The object has the original commitment, opening hint and packing plan. A producer creating an arbitrary commitment per proof does not make the advice trusted.
7. Construct `JoltVmWitnessConfig::new(log_trace_length, ram_K, one_hot_config).include_untrusted_advice(...).include_trusted_advice(...)`, then `TraceBackend::<OwnedTrace>::from_compact(config, JoltVmWitnessInputs::new(program, program_preprocessing, trace))` for the classic route.
8. Call `jolt_prover::akita::prove::<AkitaField, AkitaScheme, AkitaVc, AkitaTranscript, _>(&JoltAkitaBackend::optimized(), &preprocessing, &config, trusted_object.as_ref(), &witness, &public_io)`.
9. Externally call `jolt_verifier::verify::<AkitaField, AkitaScheme, AkitaVc, AkitaTranscript>(&approved_verifier_preprocessing, &expected_public_io, &proof, approved_trusted_commitment.as_ref())`.

This is a **complete ordinary outer PCS proof**: the modern Akita prover runs the stage spine and stage8 joint opening; the corresponding verifier runs all stages and its native final opening. Do not use `verify_until_stage1`, a pending frontend object, or a fold-only return as receipt acceptance.

### Fixed preprocessing is subtler than fixed code

`ProverConfig::derive[_compact]` derives trace length, touched RAM size and chunking from execution (`jolt-prover/src/config.rs:62–169`). Akita has a 4096-cycle floor and reserves a row beyond the active trace. Its preprocessing derives grouped setup from exact trace shape and optional advice capacities. Stage8 requires **equality**, not merely <=, between the canonical main column arity/count and setup arity/count (`stage8/akita.rs:48–91`). Therefore an SDK-style maximum trace bound alone does not provide one reusable Akita PCS setup for every execution length.

A fixed bounded deployment must either provision a checked fixed outer geometry and pad the witness consistently, or install an authenticated finite family of preprocessing/setup rows for admitted geometries. The immutable guest image and program statement remain fixed in either case. Explicit fixed-geometry host integration and all affected witness/RAM/chunk parameters are implementation work; do not just mutate `trace_length` and presume the remaining configuration follows. A prover-provided replacement preprocessing blob is not an acceptable geometry selector.

## 5. Private archive and bound public receipt

### Existing executable advice surface

`jolt-sdk/src/lib.rs:259–315` provides `TrustedAdvice<T>` and `UntrustedAdvice<T>`, with `new`, `From`, `Deref`, and serde support. Macro argument classification looks at the final syntactic type name (`macros/src/lib.rs:1041–1103`); these wrappers route to their separate memory regions, while an ordinary `&[u8]` parameter is public. An arbitrary renamed type alias is not automatically recognized as advice. `PrivateInput<T>` is an alias for `UntrustedAdvice<T>` but its macro spelling requires zk (`lib.rs:319–328`; macros:913–926), which is incompatible with the current Akita build. For nonpublic transport without a ZK claim use `UntrustedAdvice`, not `PrivateInput`.

Generated host code separately postcard-serializes public, untrusted and trusted arguments (`macros:686–757`). Generated guest code reads corresponding fixed memory slices and calls `postcard::take_from_bytes` (`macros:763–884`). The wrapper is routing, not a correctness check on the contained archive.

The selected proposed D1 entrypoint is `verify_archive(x_o: ReceiptStatement, archive: UntrustedAdvice<ArchiveBytes>, setup: TrustedAdvice<FixedSetupBytes>) -> u32`. Here x_o is `(D1, authenticated id(I), X)`, not bare X. The fixed guest computes the concrete inner transcript and checks the complete native archive; no graph witness, pre-beta envelope or C_t is present. This is a reviewed parametric algorithm, not an existing compiled guest.

For the lower-level host route, the exact data separation is:

* public `TraceInputs.inputs`: canonical encoding of **x_o**, not X alone;
* `TraceInputs.untrusted_advice`: the bounded canonical archive containing its checked copy of X, under the actual original C_archive;
* `TraceInputs.trusted_advice`: the installed static setup wrapper bytes, under expected C_static;
* no archive bytes embedded in the ELF, preprocessing, public x_o, or a per-run verifier key.

The external party reconstructs the expected device from approved layout, exact x_o, output byte01 and panic=false. It supplies installed C_static and runs the complete ordinary verifier with the actual original C_archive and its checked presence/shape. D1 has no external Vhat equations or quotient opening; its concrete native/outer-FS heuristic boundary is explicit in the model.

### Evidence that advice stays off the public transport

`common/src/jolt_device.rs:68–122` explicitly distinguishes public input/output from private advice. Both serde serialization and its canonical serializer replace trusted/untrusted advice vectors with empty vectors. Deserialization still accepts populated fields for compatibility, so an application should not use deserialized advice fields as an authentication channel.

Advice is not merely ignored by the proof. Stage4's RAM initialization decomposition contains trusted/untrusted advice opening contributions (`jolt-verifier/src/stages/stage4/ram_val_check.rs:1–17,46–82`). The Akita stage8 consumes those original advice claims directly, validates their presence against commitment presence and canonical packing metadata, and includes them in the joint opening (`stage8/akita.rs:159–186,244–377`). It does not use the generic stage6b/7 advice reductions. Stage0 commits untrusted bytes and absorbs their commitment; the verifier gets that commitment from the proof. Trusted advice instead uses the commitment supplied by the external verifier.

The ordinary Akita clear proof is **not ZK**. Omission of archive bytes from public I/O is enough to prevent mandatory O(T) archive transport; it is not a confidentiality theorem about the proof.

### Proposed accepting guest predicate

The guest must be fixed and bounded, and must:

1. Parse canonical x_o and resolve only approved application, profile, setup, source, sampler and archive schemas. Public X is a component of x_o, not its entire contents.
2. Read the bounded archive from original C_archive and require its X to equal public X. Reject zero/excessive segment counts, impossible lengths, overflow, noncanonical field encodings and descriptor mismatches before allocation.
3. Start from the registered zero receiver and the exact approved initialization statement.
4. Replay every frontend/update through the exact **fixed D1 concrete transcript**, computing all component calls and rejected sampler attempts. Retain original source handles, domains, setup roles, binding and endpoint links. This is ordinary concrete computation, not an implementation of a programmable ideal oracle.
5. Apply the original global final/public-I/O convention exactly once, not a new emulator-halt condition.
6. Verify the final domain/normalization prefix, all three shrinking reductions, both E planes and the complete A-only terminal; any remaining setup obligation must be discharged. The final decider is inside the guest by default.
7. Require logical archive EOF, transcript lifecycle completion and no remaining native/source/setup obligation. Return `1u32` only after complete acceptance. No supplied-answer ledger, graph/quotient check, guest-side H_o call or nested PCS verifier is introduced.

Current native `AkitaVerifier::batched_verify` already completes root and suffix replay and calls `VerifierGrinding::finish` (`akita-verifier/src/fold/verify.rs:25–147`); `finish` requires the full grinding plan and `state.check_eof()` (`akita-types/src/transcript_grinding/replay.rs:526–538`). The new accumulation archive needs the analogous whole-container check in addition to each native subproof's EOF.

The SDK does **not** check exhaustion of the fixed input/advice slices after decoding arguments: `check_input_len` is empty and the remainder of `take_from_bytes` is discarded. A byte-vector/slice argument gives the guest a length-delimited logical archive, within which EOF can and must be checked. If the wire contract additionally rejects extra bytes after the postcard argument, a custom checked guest entrypoint or canonical fixed-capacity envelope/padding check is needed; the existing macro does not supply it. Do not confuse allowed zero-filled capacity padding with trailing logical proof bytes.

## 6. Public statement and canonical success binding

The generated SDK verifier is a useful design model even though its aliases are Dory. `make_build_verifier_fn` (`macros:220–290`) builds a fresh `JoltDevice`, serializes the **caller-expected public arguments and output**, sets the caller-expected panic flag, and invokes verification. It does not need advice bytes.

The modern verifier:

* checks device memory layout equals preprocessing, input/output limits, power-of-two trace bounds and legal RAM capacity;
* absorbs `preprocessing_digest`, public input bytes, output bytes, panic, RAM size, trace length, entry address and proof geometry (`verifier.rs:910–989`);
* binds actual I/O memory through the stage2 `RamOutputCheck` relation (`stages/stage2/ram_output_check.rs:1–90`), not just the transcript;
* absorbs trace, untrusted and trusted advice commitments in canonical order (`verifier.rs:708–789`).

It normalizes outputs by removing trailing zero bytes (`verifier.rs:1093–1100`, also the proof-input validation path). Enforce the exact expected encoding rather than deserializing an attacker-selected device. Canonical postcard `1u32` is byte `01`; require panic=false and verify against expected **x_o**. The guest need not return X again.

Selected acceptance is [D1's complete contract](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md): approved installation; reconstructed device `(inputs=encode(x_o), outputs=encode(1u32), panic=false)`; installed C_static; actual original C_archive presence/shape/source binding; and the complete ordinary outer proof. Its guest computes inner transcript answers itself. Never accept an arbitrary proof-supplied success device or a pending native result.

## 7. Setup authentication, detached/native payloads, and cheapest provision

### Three identities must remain separate

1. **Outer program preprocessing** authenticates the fixed archive-verifier program and its memory/trace/profile configuration.
2. **Outer PCS setup** authenticates the ordinary proof system proving that guest.
3. **Inner application/setup registry** authenticates the 33 native matrix roles and exact inner archive relation the guest verifies.

A valid receipt under an attacker-selected instance of any of these does not establish the desired application statement.

`ProgramPreprocessing` can be `Full(Arc<JoltProgramPreprocessing>)` or `Committed(CommittedProgramPreprocessing)` (`jolt-verifier/src/preprocessing.rs:17–75`). The committed form holds metadata, memory layout, max padded trace length and direct program commitments in the Akita build. `JoltVerifierPreprocessing::new` computes a Blake2b-256 digest of the bincode **program preprocessing**; it does not hash the entire PCS/VC setup (`preprocessing.rs:176–291`). Serde omits the stored digest and recomputes it on decoding, which prevents a stale wire digest but does not authorize an untrusted program. The in-memory field is public and absorbed as stored. Install preprocessing from trusted program data, do not mutate it, and authenticate PCS setup separately.

### Modern Jolt setup serialization is a recipe, not a detached native verifier

`jolt-akita/src/adapters.rs:590–623` serializes the adapter's dimension/count/layout metadata, one-hot K and dense/one-hot catalogue artifacts. Its `BackendVerifierCache` is skipped. `dense_verifier` and `one_hot_verifier` lazily reconstruct native verifier state, including calling setup-prover construction to obtain verifier state when the cache is absent (`704–788`). Deserializing adapter preprocessing inside a guest therefore does not preserve precomputed host caches or mean setup expansion is free.

Setup transcript binding includes backend flavor, dimensions/counts, K, chunk profile when applicable, layout digest and validated catalogue digest (`append_verifier_setup`, `adapters.rs:817–850`). These bindings stop some cross-context substitutions relative to an installed setup; they do not make a prover-selected catalogue approved.

A workspace-wide search for detached/native-verifier payload interfaces found no modern detached receipt loader. The concrete native building blocks below exist, but a guest-facing adapter that installs them without re-entering host setup/prover machinery is **proposed work**, not an available SDK call. The public fields of native payloads must not be used to bypass modern group/profile checks.

### Native setup APIs and their trust boundaries

`akita-types/src/proof/setup.rs` exposes:

* `AkitaSetupSeed { derivation, seed }` and `AkitaSetupDescriptor { max_num_vars, max_num_batched_polys, num_field_elements, setup_seed }`;
* `derive_public_matrix_prefix`, currently paged SHAKE256 with `Field::random` sampling;
* `AkitaExpandedSetup::from_verified_parts`, which checks materialized coefficients against the seed;
* `from_trusted_seed_derived_parts_unchecked`, which intentionally does not derive or validate the matrix;
* `AkitaVerifierSetup::from_parts(expanded, prefix_registry)`, which checks the registry names the same seed but explicitly does **not** prove each prefix commitment derives from that matrix.

Even `from_verified_parts` only proves consistency with its supplied seed. The caller must still establish that the seed, descriptor and catalogue are approved. `sample_akita_setup_seed()` returns a fixed reproducible seed with the initial word `0xDEAD_BEEF_CAFE_BABE`; it is not fresh entropy (`setup.rs:215–220`).

The current shared-stream law deliberately ignores ring dimensions when deriving coefficients and shares equal-length prefixes (`setup.rs:223–255`). That is **not** the newly approved inner independent-role provisioning law. `AKITA_INSTANCE_ACCUMULATION_MATRIX_REGISTRY.md:5–11,76–92` requires 33 separately namespaced roles, 4,681,728 K coefficients and newly provisioned 512-bit modular sampling. Do not relabel the current shared seed/stream as that registry. Existing frozen keys cannot be changed by a setup optimization.

For a lean native consumer, `akita-verifier/src/lib.rs` deliberately excludes polynomial backends, commitment hints, witness construction and planner search. Pair it with `akita-types` and `akita-config`. `AkitaVerifier::new` admits rows and prepares terminal matrices; `AkitaVerifier::for_selection(setup, trusted_catalog, selection, Option<&[u8]>)` admits one row and optionally installs a **trusted** precomputed terminal cache (`verifier.rs:37–121`). It still requires the expanded setup.

`build_riscv64_terminal_ntt_cache` and `decode_riscv64_scalar_q128_cache` provide a native prepared cache. The artifact uses magic `AKVNTT01`, target id 1, a 120-byte header, a 64 MiB size cap, setup seed digest, schedule row digest, setup coefficient count, ring dimension, base/tail counts, width and RHS bound (`akita-types/src/ntt_cache/prepared_artifact.rs:1–98,259–332`). It validates identities, lengths, exactness sizing and coefficient residue ranges. The source explicitly warns that this does **not** prove transformed payloads were derived from the named matrix. Fixed code, genuinely authenticated trusted advice, or recomputation must establish provenance.

### Provisioning comparison

No measured cheapest option is established. The sound engineering ranking is conditional:

* **Deterministic generation inside the fixed guest:** simplest trust boundary and good reference/default. Freeze the approved seed(s), exact 33-role namespace/geometry table and generation law in the approved program/profile; reject substitutions. This avoids adding a trusted setup-advice commitment to the outer statement, but pays every required expansion and transform in guest cycles. Existing shared-prefix setup generation must be adapted to the approved independent-role, 512-bit law before reuse. Streaming can reduce peak storage; it does not eliminate expansion/evaluation work.
* **Precomputed fixed setup as trusted advice:** [INFERENCE] strongest candidate for lowest variable guest work without a huge program image. Provision canonical setup bytes once, precompute the **outer PCS commitment to those exact trusted-advice bytes**, and make the external verifier supply only that installed commitment. The guest parses structurally validated matrices and optionally transformed caches. This removes repeated in-guest expansion/NTT, but adds an outer advice source, RAM/advice polynomial geometry and an ordinary outer PCS opening. Authentication must cover the whole payload, catalogue, role table and cache bytes, not just attacker-supplied seed/digest metadata. The commitment is fixed per outer profile, not generated from arbitrary proof advice at verification time. Canonical inner matrix materialization alone is 74,907,648 bytes / 71.4375 MiB; memory/advice padding and outer proof costs are material.
* **Fixed code/rodata embedding:** sound if immutable and included in authenticated program preprocessing. It eliminates a separate setup-advice authentication dependency but increases program/RAM image and preprocessing cost. Fixed setup embedding is not the prohibited per-run archive embedding. With 71.4375 MiB of coefficients plus metadata/caches it is not automatically cheapest.

Recommendation: specify deterministic generation as the sound reference path; make an authenticated trusted-advice setup bundle the explicit optimization candidate. Choose between them only after the full ordinary outer proof costs are measured. Do not adopt a digest-only or header-only shortcut. A complete fixed digest checked against every supplied payload byte can authenticate that payload under its own hash assumption, but a supplied digest compared only with supplied metadata cannot.

## 8. Source commitment routing that a guest adapter must preserve

`AkitaCommitment` contains backend flavor, layout digest, arity, polynomial count, one-hot K, backend coefficient count and serialized native commitment bytes (`adapters.rs:883–893`). All of these are transcript-bound (`984–998`). `AkitaBatchProof` contains a fixed-width selected schedule-row identity plus headerless Spongefish proof bytes (`1001–1038`). The raw backend proof does not identify its own selected row.

The stage8 group order is untrusted advice, trusted advice, optional field increments, direct committed program objects, and finally the native OneHotTrace group. The transcript's initial object absorption order is OneHotTrace first, then the advice/field/program commitments; these are distinct ordered contexts, not interchangeable lists.

`stage8/akita.rs` validates:

* native main flavor/K/layout, exact arity and physical polynomial count;
* auxiliary dense flavor, canonical layout digest, arity and exactly one physical polynomial;
* advice claim/commitment presence;
* direct program object count and canonical plans;
* all main columns sharing their canonical common point.

`native_batching.rs:154–198` binds setup, row selection, ordered role/index, original commitment, group point and every evaluation before the native bridge. The prover imports auxiliary commitment handles onto the trace backend by re-deriving them from retained original sources and checking the recomputed commitment (`312–329`); handles are backend-owned, not freely interchangeable hints. The verifier resolves the selected trusted row and derives coefficient/profile lengths **before** decoding native commitment payloads (`shape_guard.rs:1–145`). Native `batched_verify` checks every committed descriptor, compression-chain length, resolved profile, opening layout, ring dimensions and row admission before root/suffix replay.

A detached/guest-native path must preserve all this routing. Supplying only a native proof body plus a scalar final claim, or rebinding commitments to freshly chosen canonical sources, loses the actual statement.

## 9. Codec contract

There are several formats, not one universal serializer:

* guest arguments/results: postcard;
* SDK verifier objects and complete Jolt proof/preprocessing structures: serde/bincode standard configuration;
* current recursion archive: u64 little-endian record length followed by one bincode record;
* native Akita commitment/setup objects: `AkitaSerialize`/`AkitaDeserialize` compressed format with explicit context;
* native proof body: authoritative Spongefish bytes under the selected row's grammar;
* prepared terminal cache: separate fixed-header RISC-V scalar-Q128 format.

`host_utils.rs:198–217` rejects trailing bytes in an individual verifier object. `adapters.rs:1281–1303` uses a cursor and rejects trailing native object bytes. `shape_guard` supplies the trusted-schedule-derived decode context. Native transcript field atoms use checked canonical little-endian decoding and extension atoms decode all base coefficients (`akita-transcript/src/proof_stream.rs:307–335`).

These primitives do not automatically provide a versioned bounded accumulation archive codec. The receipt needs a new canonical outer frame with profile/version and bounded counts/lengths, complete logical EOF, explicit option/presence rules and admitted geometry. The guest should parse incrementally rather than deserialize unbounded vectors of proofs. Do not rely on serde shape acceptance or bincode default limits as the cryptographic parser contract.

## 10. Remaining E-challenge migration

The available ordinary K Akita route is real source, but it does not implement the approved E-challenge profile. The detailed reviewed map is `AKITA_INSTANCE_ACCUMULATION_FRONTEND_EXTENSION.md:11–17,19–34,83–114`.

Required work includes:

1. Implement the explicit nonresidue-5 quadratic type and its canonical encoding and required arithmetic/accumulator capabilities. Default `FpExt2<K>` uses `TwoNr`; the source says 2 is square for this K. An alias to the default extension is not sound.
2. Keep committed VM/source values, integer representative/decomposition rules and lattice matrices in K; make frontend challenges, working evaluations, sumcheck messages, points and appropriate accumulators E-valued. Preserve the reviewed K-source descent and same-semantics segment consumers.
3. Split current field-coupled PCS/claim interfaces where necessary: `CommitmentScheme::Field` currently governs commitment-side and opening-side values; stage verifiers require `Transcript<Challenge=F>` (`jolt-openings/src/schemes.rs:49–54,111–133`; verifier.rs:174–189).
4. Carry both full S128 planes for E partials and implement the reviewed scalar-extension carrier path. Existing `FpExtEncoding` is a specific ring-subfield embedding contract, not permission to mark arbitrary nonresidue-5 coordinates as native subfield coordinates.
5. Replace all relevant 16-byte frontend draws with the selected full-width, fallible samplers. `DigestTranscript::challenge` and `challenge_scalar` currently each draw16 bytes (`digest.rs:171–182`). Preserve their distinct semantic sites and stage dependencies, not their obsolete base-only decoding laws; version the changed sampling explicitly. Native coefficientwise extension sampling is a reusable arithmetic pattern, not the selected bounded outer compiler.
6. Replace `bridged_akita_session`'s one-K-scalar bridge (`adapters.rs:1337–1350`) with [OUTER_TRANSCRIPT §5](AKITA_INSTANCE_ACCUMULATION_OUTER_TRANSCRIPT.md)'s checked full-instance transition in the same complete context/prefix. Remove the old bridge draw. Neither a wider scalar, a debug-state export nor a fresh transcript is equivalent.
7. Emit/check the complete sampler, exception, field/source, tree/error and matrix manifests. Apply these changes to the control's corresponding frontend/ordinary PCS path as required by the shared security policy. The outer ordinary receipt also needs a fresh manifest/profile or an explicitly different admitted security bound; its K-only legacy transcript cannot inherit the inner compact statistical bound.

Inside the guest, verification of the new archive is ordinary computation over the inner K/E types. If the outer proof also adopts E challenges, that is a second frontend/proof-system integration concern, not an effect of the guest doing E arithmetic.

## 11. RISC-V, std and arithmetic constraints

The SDK can be no_std, but `guest-verifier` explicitly enables `guest-std` (`jolt-sdk/Cargo.toml:42–69`). The recursion guest also enables thread and random. Inspected Jolt/Akita verifier/type code uses `std::io`, `Arc`, collections and std field implementations. There is no evidence that the full existing verifier closure is a no_std library.

`jolt-akita` imports the broad `akita-pcs` umbrella with parallel support and has an unconditional Rayon dependency (`jolt-akita/Cargo.toml:17–42`). Its adapter funnels setup/commit/prove/verify into a thread pool with 64 MiB worker stacks (`adapters.rs:242–265,336–352`). This is an important dependency/cost problem for a lean guest; it is not proof that RISC-V execution is impossible, since the existing runtime has std/thread support.

The native verifier-only crate is the appropriate dependency boundary for a new guest adapter. Disable parallel features where supported and avoid importing planner/prover backends into the guest solely to reconstruct setup. If strict no_std is a deliverable, a real dependency/API port is still needed; changing the guest's crate attribute alone cannot accomplish it.

Fp128 x86_64/AArch64 assembly is cfg-gated; other architectures use portable Rust (`jolt-field/src/solinas/fp128.rs:213–237`). Portable wide multiplication is built from u64/u128 operations (`solinas/word.rs:525–537`); unsupported SIMD targets use `NoPacking` (`solinas/packed/mod.rs:24–49`). This is a plausible RISC-V arithmetic path, not a measured cycle estimate. Checked canonical decoding, bounded allocation, nonzero/exception handling and both E coefficients remain required. Optional field-inline acceleration is a different instruction/profile path and must retain the reviewed field quotient/canonicality distinctions; it cannot silently strengthen guest arithmetic semantics or reuse BN254 execution fields in an fp128 profile.

## 12. Same-semantics recursive control

`AKITA_INSTANCE_ACCUMULATION_SEGMENT_INTERFACE.md:235–242` specifies a legitimate one-shot recursive baseline:

* use the same continued-segment frontend, source encodings, boundaries, setup/security policy and public whole-run statement;
* fully discharge each segment's **ordinary** source-opening ledger, including actual boundary groups and normal suffix, rather than pause/accumulate;
* do not add accumulation-only raw source-range certificates to the control;
* one fixed bounded guest verifies all completed modified segment proofs, ordered adjacency, Init and Final, and returns canonical success for the same X.

The current independent Fibonacci/Muldiv conjunction is not that control. For the control, completed per-segment proofs are intentional; for the selected accumulation design they remain prohibited as inputs. Both wrappers should use the same nonpublic advice delivery policy and external expected-output rule. Each path pays actual setup, frontend, boundary, advice, codec, guest and outer proof costs. A separate wrapper per segment is not mandatory.

## 13. Implementable work and acceptance boundary

The source-backed work remaining is concrete:

* implement the reviewed inner pre-PCS accumulation, E migration, exact segment consumers and final decider;
* define the bounded canonical archive/frame and fixed guest predicate;
* supply modern Akita host proof orchestration rather than reuse Dory SDK aliases;
* install fixed program preprocessing and an admitted fixed geometry or authenticated geometry family;
* implement a guest-oriented native verifier/setup path preserving all source, role, schedule, transcript and original-profile checks;
* implement and authenticate the chosen 33-role setup provisioning law; optionally add trusted setup-advice commitments/caches without digest-only trust;
* deliver a canonical **x_o=(D1,id(I),X)** serializer/verifier with original C_archive and its frozen shape, installed C_static, complete ordinary outer proof and externally expected success1/panic=false;
* implement the same-semantics full-ordinary-PCS recursive control;
* separately review outer proof/inner verifier composition and account for all outer setup, hash, extraction-runtime and statistical terms. Neither a real proving call nor an accepting honest trace is a composition theorem.

After implementation, the main agent should run the selected feature builds and whole-program proof/verify checks once, then adversarial cases for output 0/panic, empty/oversized counts, malformed/trailing bytes, substituted program/setup/cache, dropped/reordered updates, endpoint mismatch and incomplete decider. The source tests named above are useful patterns, not evidence for these new behaviors. Paired end-to-end receipt/control measurements remain required; none were obtained here.


# Appendix: exact advice capacity and authentication

## Addendum: exact trusted/untrusted advice and size semantics

This supplements the earlier complete `modern-receipt-path.md` report and responds to Main's coordination message. No files or code were changed and no checks were run.

### 1. Concrete APIs and ownership

Modern Akita provisioning can call:

`jolt_prover::akita::preprocessing::commit_trusted_advice(&AkitaProverPreprocessing, advice_bytes) -> Result<AdviceObject<AkitaScheme>, PreprocessingError>`.

This obtains `max_trusted_advice_size` from the approved program memory layout and delegates to:

`jolt_prover::akita::witness::commit_advice::<AkitaScheme>(setup_context, JoltAdviceKind::Trusted, advice_bytes, max_advice_bytes)`.

`AdviceObject` has public `plan`, `commitment`, and `hint` fields (`witness.rs:340–378`). It is an original dense bounded-word source object, not merely a digest. The ordinary Akita prover takes `Some(&object)`; the external verifier takes `Some(&object.commitment)` through the fourth argument of `jolt_verifier::verify`.

The verifier does not obtain the trusted commitment from the proof. `JoltProof` exposes `untrusted_advice_commitment: Option<PCS::Output>` (`proof.rs:55`), while trusted commitment presence and contents come from the verifier call. Thus the fixed setup bundle can be committed by the verifier/setup authority during provisioning, with its commitment installed once; the prover must use matching source bytes and retain valid opening material. A prover-supplied replacement C is not trusted merely because it is passed through that argument.

`stage0.rs:66–70` checks trusted object presence agrees with nonempty trusted bytes. This is only a presence check; content binding comes from the RAM initialization relation and ordinary PCS opening under the supplied original commitment, not from this check. Untrusted commitment creation is automatic from `public_io.untrusted_advice` and its configured maximum (`stage0.rs:160–169`). The completed proof stores that untrusted commitment, and the verifier derives advice presence from it.

The source test `trusted_advice_commitment_reused_across_chunk_profiles` (`akita_e2e.rs:362–415`) explicitly commits one trusted object and reuses its original commitment/hint across several outer chunk profiles. This is evidence of the intended API contract only; it was not executed. Reuse still requires admitted grouped rows and compatible original source profiles, not arbitrary changing capacity/layout.

### 2. What exactly is committed

`common::advice::canonical_advice_words(bytes, max_bytes)` (`jolt/common/src/advice.rs:29–44`):

1. rejects `bytes.len() > max_bytes`;
2. packs bytes little-endian into u64 words, zero-filling a partial last word;
3. resizes to `(max_bytes / 8).next_power_of_two()` words with zeros.

`commit_advice` then maps each word with `PCS::Field::from_u64` and embeds that word table in the advice physical layout. There is no separately committed `actual_byte_len` field in this API. Raw payloads differing only by additional zero bytes can induce the same advice polynomial. That is appropriate memory semantics, not a logical frame parser.

The advice layout uses `word_vars = log2(word_capacity)`, then `physical_vars = max(word_vars, 14)`, and rejects above 34. For small capacities the extra physical coordinates are empty prefix slots; the actual word column occupies slot zero. Layout domain is `advice-dense-words-v2`, and the trusted/untrusted roles are distinct (`jolt-claims/.../lattice/packing.rs:183–215`). Here “logical_num_vars” means configured word-domain arity, not actual serialized frame length.

The verifier reconstructs initial advice block selectors and opening points from the approved memory layout's `*_advice_start` and `max_*_advice_size`, and commitment presence (`stage4/ram_val_check.rs:350–383,497–550`). It does not consult serialized private advice bytes or their vector length. Advice commitment present with configured size zero is rejected.

### 3. Logical archive EOF and padding

Main's clarification is correct: **require the entire declared archive frame to be consumed, not all unused advice capacity**. Frame length must be encoded canonically inside the archive input representation, bounded by the approved maximum, and checked by the guest's parser. Postcard's byte-vector/slice framing can delimit that logical frame; a dedicated bounded frame may provide more explicit allocation/resource control.

The existing SDK guest entrypoint decodes arguments from capacity-sized advice memory and leaves the remaining capacity unused. Those unused bytes are not automatically part of the logical archive and need not become a stronger application predicate. The important rejection is any trailing data inside the declared frame, or an incomplete/dropped record required by its count/grammar. Similarly, a fixed setup bundle should have its canonical descriptor/count/length framing included in the committed bytes so that the application interprets one unambiguous payload.

The emulator reads absent advice bytes as zero (`common/src/jolt_device.rs:154–165`), consistent with the commitment constructor's padding. Do not confuse transport length, frame length, word-domain capacity and physical PCS arity.

### 4. Capacity constraints and concrete cost warning

`MemoryLayout::try_new` aligns advice sizes to eight bytes, then requires each nonzero aligned advice capacity to be a power of two (`jolt_device.rs:390–413`). Aggregate input + output + both advice regions + 16 bytes for panic/termination is padded to a power-of-two word region placed below `RAM_START_ADDRESS = 0x80000000`. It rejects a region whose start is zero or underflows (`jolt_device.rs:416–444`; `common/src/constants.rs:21`). Therefore the nominal 34-variable PCS advice cap is not the effective maximum RAM-advice capacity of this VM layout. Aggregate padded I/O must be strictly less than 2 GiB, so the largest permitted padded I/O region is 1 GiB; positive non-advice overhead further constrains any individual power-of-two advice allocation.

The approved inner matrix bundle alone has 74,907,648 bytes (71.4375 MiB). If encoded as those canonical bytes plus modest metadata, it requires at least a 128 MiB trusted-advice capacity. **Derived arithmetic:** 128 MiB / 8 = 2^24 advice words, so the present `commit_advice` materializes 2^24 K field evaluations. At 16 bytes per K element, that evaluation vector alone is 256 MiB, before polynomial ownership, source decomposition, lattice commitment material, archive advice, witness or trace. Each serialized 16-byte inner K coefficient occupies two outer u64-word source coefficients; no implicit field-element zero-copy route exists in this byte-advice API. Prepared caches/static program data can increase the required capacity further.

This does not rule out trusted setup advice, but it prevents calling it categorically cheapest without measuring the complete outer route. An authenticated fixed trusted bundle avoids in-guest matrix/NTT regeneration; it pays the full outer dense advice-source cost. Deterministic in-guest generation trades that source cost for guest cycles and runtime memory. Fixed rodata trades it for program-image/setup costs. These are distinct resources.

### 5. Delivery recommendation to Main / ReceiptSetupSecurity

Verifier-provisioned C_static is supported by the lower-level Akita advice API, conditional on authenticated static bytes and the proved source/reference relation. D1 keeps the bounded archive in original C_archive and computes the inner transcript directly. Construct the external expected **x_o/success1/panic=false** device and enforce [D1 acceptance](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md), including complete ordinary proof verification, installed C_static and actual C_archive shape/source ownership. No graph/quotient source or external Vhat check is selected. The existing setup stream is not the new 33-independent-role law; cache labels/digests alone do not establish matrix provenance.

## Production boundary implementation: shared-prefix native revision

The current implementation seam is `jolt/crates/jolt-verifier/src/receipt.rs`
and its `statement`, `archive`, `static_setup`, and optional `outer` modules,
exported under `akita,transcript-d1,!field-inline`. Direct native verification
may transport this O(T) container; it need not hide the archive inside a guest.
The unselected ordinary wrapper additionally requires `transcript-blake2b`.
This is **bounded boundary/API code, not evidence of an exercised whole native
certificate verifier or optional wrapper**. No build, test, formatter, lint,
trace, or proving run was performed by its implementation worker. The companion
codec tests are present but unexecuted.

### Canonical logical formats and parent integration

- `ReceiptStatement` encodes `AKD1ST01`, the authenticated installation and
  profile ids, run id, original application preprocessing digest, finite
  segment count, complete expanded-row count, application panic flag, and
  bounded application input/output bytes. It uses fixed little-endian
  lengths, rejects noncanonical Boolean flags and the original convention's
  trailing-zero output representation, and checks complete logical EOF.
  Intermediate boundary contexts are untrusted archive records, not assumed
  installed public inputs. `StatementPolicy` comes from authenticated approval.
- `Archive::decode(bytes, expected_statement, statement_policy, archive_limits)`
  verifies `AKD1AR01`, an exact complete-statement echo, `1..16` segments,
  consecutive explicit indices, bounded nonempty context/frontend/source-check/
  Common-K-normalization blocks, one nonempty whole native proof block, and
  EOF. It performs no allocation; all segment views are held in a fixed
  16-slot array. `Archive::segments()` returns borrowed `SegmentRecord`s;
  `Archive::native_proof()` returns the single retained native stream.
  `Archive::encode` is the matching producer. These opaque payload bounds are
  **not** evidence that their nested message grammar has been decoded.
- `NativeFrontendFrame` matches the actual parent's in-memory records:
  `SegmentContext<K,CompactRamEndpoint>`, `ContinuedFrontend<
  JoltProofWithOpening<CompactInnerTrace,NoVectorCommitment<K>,()>,K>`,
  `SegmentMainProof<K>`, and `MainNormalizationProof`. It deliberately does
  not implement generic serde `Deserialize`. `check_frame_chain` uses the
  existing original context/program/predecessor validators, checks each
  context against the complete public run statement, freezes the selected
  4096-row/65536-word profile, and checks the total expanded-row count.
  This is a structural/public-link precheck, not complete native acceptance.
- `StaticSetup` encodes `AKD1SS01`, the inner installation id (excluding the
  later outer commitment), profile id, **actual** native paged-SHAKE derivation
  tag/seed/dimensions/count, exact bounded application/catalog/registry regions,
  and one canonical K coefficient vector. The trusted policy fixes every
  region length and total size; coefficients at least q, truncation, and
  trailing logical bytes are rejected. `expand_verified` calls native
  `AkitaExpandedSetup::from_verified_parts`, not an unchecked constructor or
  prover backend: it validates the coefficients against the actual seeded law.
  This reference path pays in-guest rederivation and does not claim free
  prepared-cache installation. Application/catalog/registry regions still need
  their approved bounded production loaders. The new revision does not inherit
  the old 33-independent-array/512-bit distribution or repeated-array size.
- For the **optional guest wrapper**, `encode_slice_argument`,
  `decode_slice_argument`, and `ReceiptStatement::encode_argument` implement
  borrowed postcard byte-slice transport, with minimal bounded varint lengths.
  Its proposed entrypoint arguments are `statement: &[u8]`,
  `archive: UntrustedAdvice<&[u8]>`, and `setup: TrustedAdvice<&[u8]>`.
  If this wrapper is selected, its caller commits **the wrapped static argument
  bytes**, not the inner logical blob alone, through `commit_trusted_advice`;
  tracing receives the identically wrapped bytes. Logical EOF differs from
  unused memory-capacity padding. Direct certificate transport needs no SDK
  advice wrapper or outer advice commitment.
- The **optional, unselected** `ApprovedReceiptInstallation::from_approved`
  accepts only externally
  authenticated ordinary Akita preprocessing, installed original `C_static`,
  the approved policies, and an exact `OrdinaryGeometry` tuple
  (trace/RAM/read-write/chunk/order). It checks argument/advice capacities and
  canonical dense advice shapes. `verify_receipt(expected_statement, proof,
  transcript, emissions)` requires the actual original proof-carried `C_archive`,
  checks exact installed geometry, reconstructs caller-expected public statement
  bytes, output byte `01`, and outer `panic=false`, and calls the **complete**
  `jolt_verifier::verify<AkitaField,AkitaScheme,NoVectorCommitment<AkitaField>,T>`
  with installed `C_static` and the caller's installed live transcript/emission
  plan. `T` must implement the actual native-batch opening capability; no default
  compiler, reset, fresh owner, or unchecked emission plan is invented here.
  No proof-carried device/preprocessing/trusted commitment is accepted by this
  interface; no SDK Dory alias, Groth16, or hash wrapper is used.

### Native integration and separate optional-guest prerequisites

The parent supplied the real native callback chronology: initial installed
frontend verifies stages 0–8, source checks and Common-K normalization,
seals/authorizes the native instance on one retained compiler, and supplies
`fresh_obligations(0)` to the first native callback. Every later native callback
resumes its corresponding frontend with `resume_frontend_from_native`,
performs the same complete checks, reauthorizes the **same party-local Rc**,
and returns that update's fresh obligations. The whole native stream completes
the final domain prefix, shrinking reductions and A-only terminal.

The direct native consumer still needs its exact shape-derived bounded nested
frontend-record decoder for production byte transport; generic bincode/serde
vectors are not such a decoder. It must validate the actual context chain
against the complete run statement and derive finite installations under the
approved setup, not treat intermediate contexts as trusted installed inputs.
This is a native certificate integration obligation independently of any
wrapper. Canonical application/registry loaders and guest capacities are
additional requirements **only for an optionally selected fixed guest**.

The parent's reported 7,917,436 bytes describe the expanded v1 **installation
description**, not the native certificate. The earlier 4 MiB context/record and
16 MiB compiler limits were implementation contracts, not theorem requirements.
The integration owner now selects conservative expanded current framing with
16 MiB payload/context/seal/event and 512 MiB retained-transcript budgets,
records/sites at most `2^20`, 64 blocks, `2^24` component calls and `2^28` read
bits. Coordinates/powers stay at 256; schema/depth remain 131072/16. The selected
caps are bound in the installation footer and belong to the parent's complete
resource-profile implementation/review, not a receipt-code default.
`ArchiveLimits` remains externally supplied; this worker neither widened native
caps nor omitted binding data. Costs must include the complete retained
transcript/seals, not just the actual certificate. A separate compact
deterministic installation-binding cutover would still need soundness review.

No accepting consumer or guest is synthesized from container validity, an
arbitrary Boolean callback, or generic serde decoding. Complete native replay
and its final decider remain required. An absent ordinary outer proof is not a
native failure: optional-wrapper work and its budgets are separate, with
`optional_wrapper=false` for the current direct path. Native composed-theorem
gaps, setup/source obligations and paired measurements remain independently
reportable. Nothing here establishes a numerical certificate/receipt bound,
performance parity, deployment readiness, or a PQ/QROM theorem; Akita's
published Fiat–Shamir theorem is classical ROM.

