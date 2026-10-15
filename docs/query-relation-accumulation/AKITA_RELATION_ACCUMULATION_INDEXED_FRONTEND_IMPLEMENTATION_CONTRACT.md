# Candidate1 — indexed frontend Rust implementation contract

## Decision

**The correction in `AKITA_RELATION_ACCUMULATION_PROTOCOL.md:78–86` is coherent with the available generic frontend/source-check APIs.** Direct Main118 + RAM2 is exactly the existing pending Main56, physical-zero56, first-PC3, last-PC3, and two original RAM endpoint assertions. It removes the optional normalization protocol; it does not remove any of those120 original scalar obligations. The selected local algebraic error therefore omits the optional normalization's45mu term while retaining all original stage/source terms, with `mu=2/2^128`.

It is **not** a type-alias substitution into the current `CompactSegmentInstallation`: that installation owns H1610, installs its two-attempt descriptor, unconditionally schedules normalization, and seals the normalized native instance. Implement a new indexed owner and direct-roster installation using the existing generic traits; preserve the old H1610/full-ring implementation as a separately named/profiled path. No arithmetic stage or sumcheck algorithm needs modification.

Inspection only: no code changes, Git operations, builds, tests, linters, formatters or execution checks. Proposed names below are implementation interfaces, not claims that those APIs already exist.

## 1. Exact existing APIs usable unchanged

|Existing API|Use in new mode|
|---|---|
|`jolt_transcript::Transcript`, `TranscriptError`, `MessageValue`, `AppendToTranscript` (`jolt/crates/jolt-transcript/src/transcript.rs`)|Implement `Transcript<Challenge=Prime128OffsetA7F7>`; retain generic append/compound/powers/error propagation. Do **not** implement `InfallibleLegacyTranscript` or `LegacyReplayTranscript` for the indexed owner.|
|`MessageSite::bind`, `DrawSite::bind`, `MessageScope::bind`, `EmissionBinding`, `CompoundScope` (`jolt-transcript/src/emission.rs:44–278`)|Bind exact immutable descriptors and compound shape. They are checked descriptor capabilities, not automatic event absorption or schedule advancement. Owner must revalidate them against its actual installation when executed.|
|`schema::{Schema, Message, Draw, CarrySource, Entry, Node, Limits, BoundSchema}` (`jolt-transcript/src/schema.rs:1–385`)|Owned finite metadata and lifetime-safe borrowed descriptors. `Schema::with_bound` admits descriptors before calling the protocol; lookup allocates no per-message descriptor. Handles are schema-local indexes, never cryptographic identities.|
|`structure::{Component, Site, Stage, Carry, Operation, TreeMode}` (`jolt-transcript/src/structure.rs`)|Frozen semantic path, complete stage/carry metadata, distinct raw/coefficient and scalar/vector operations. Preserve explicit interpolation arities.|
|`FrontendStageEmissionPlan`, `FrontendPreambleEmissionPlan`, `FrontendEmissionPlan`, `BatchEmissionPlan`, `MemberEmissionPlan`, `MemberOccurrence`, `OpeningOrigin`, `OutputCuration` (`jolt-verifier/src/stages/emissions.rs:1–264`)|New plan implements these with the existing source identities and member slots, including absent slots and result-derived alias filtering.|
|`SegmentEmissionPlan` (`jolt-verifier/src/segment.rs:352–357`)|Supply the complete segment-context and RAM-endpoint message scopes in the actual order used by the generic continuation.|
|`SumcheckEmissionPlan` and `RoundShape` (`jolt-sumcheck/src/emission.rs`)|Source-global round/shape -> checked message scope and raw challenge. The existing sumcheck prover/verifier owns degree checks, compressed reconstruction, append-before-draw and output absorption.|
|`prepare_continued` (`jolt-prover/src/akita/segment.rs:54–80`) and `prepare_continued_instance` (`jolt-verifier/src/segment/frontend.rs:190–211`)|Both already accept `&mut T: Transcript<Challenge=F>` and a `SegmentEmissionPlan`; both use PCS SourceField=EvalField=F. Select F=K and the admitted ordinary/source setup, without installing an E2 sampler/compiler.|
|`MainSourceCheckPlan`, `SegmentSourceEmissionPlan`, `bind_main_source`, `verify_segment_main` (`jolt-verifier/src/segment_source.rs:18–37,246–414`)|Original Main geometry/context, raw zero anchor, coefficient vectors, three degree2 sumchecks and returned original leaves. No normalization dependency.|
|`prove_segment_main` (`jolt-prover/src/akita/segment_source.rs:348–421`)|Runs those same source checks against retained original Main rows through generic Transcript/source-emission traits. Producer `PreparedContinuedFrontend.main_rows` already retains that exact Arc (`akita/segment.rs:38–50,345–353`).|
|`ContinuedFrontPending::{native,ram_endpoints,program_boundaries,into_parts}` (`segment/frontend.rs:30–62`), `PendingNativeStage8::{main_group,auxiliary_groups,into_groups}` (`stages/stage8/akita.rs:46–83`)|Read/move verifier-derived original obligations. These are pending claims, not PCS success. Never call their PCS `verify` to manufacture completed segment proofs for this accumulator.|

`segment_emissions/public.rs`, `schema.rs`, `plans.rs`, `registry.rs` and `handoff.rs` are **private, compiler-bound implementations**, not callable generic constructors ready to instantiate with another owner. Reuse their source maps/validation logic, or factor the compiler-neutral part once; do not assume changing a generic parameter reaches their H1610 record dispatcher.

## 2. Single source of stage roster truth

The existing `segment_emissions/roster.rs` is value-free and compiler-independent: it derives present members, actual dimensions/round bounds, private draws, output origins, carried coordinates and schedules from existing symbolic protocol declarations (`:1–170,360–440,905–942`). It has no H1610 sampler call. Its currently private visibility and parent module location are the only obstacles to reuse.

**Recommended small structural extraction:** move that file, without arithmetic changes, to `jolt/crates/jolt-verifier/src/stages/emissions/continued_roster.rs`, expose it `pub(crate)` under the existing emissions module, and have both installations import the one module. Change the moved metadata visibility only as needed from `pub(super)` to `pub(crate)`. Existing H1610 parent can keep its local name `roster` via an import. Do not duplicate the ~940-line roster or create a second hand-maintained member list.

New installation calls the same `selected_roster::<K>(one_hot_config, bytecode_size, first,last)`. This preserves the actual tau/uni-skip/vector counts and branch geometry; do not reconstruct them from certificate shorthand. For example current outer prelude count is14 (`roster.rs:919–925`), not a guessed scalar count inferred from a soundness coefficient. Tree arities come from this source metadata, not a blanket binary default or the fact that vector completion is atomic.

Admit exactly the existing transparent, full-public, no-advice/no-field, CycleMajor N4096/Main56/RAM65536 profile, four-bit committed chunks, three bytecode/four RAM/32 instruction chunks and fixed RW phase splits (`roster.rs:360–421`; `segment_emissions.rs:77–112`). First changes initialization, last controls the RAM-output member. Reject optional advice, committed program, field-inline or extra pending auxiliary/program groups; do not silently ignore their claims.

## 3. Registry/plan construction contract

### 3.1 Immutable installed definitions

Build finite owned definitions before running the callback:

- Exact message definitions, compound schemas, draw definitions and their full canonical descriptor bytes.
- A required event queue containing Message, Draw and admitted Choice nodes.
- Per-choice complete event alternatives, not a permissive “optional event” skip.
- Source-derived carry references and stage6b point-coordinate maps.
- The complete approved context/header, original source contracts/metadata, branch/profile inputs, and checked resource caps.

Use `Schema` storage and `with_bound`. `EmissionBinding::{message,draw,compound}` must be **pure admission lookups** during prebinding; they must not consume queue slots. They compare every field, not just site or byte width. A borrowed descriptor constructed against another installation must fail actual-owner revalidation.

Message descriptor bytes include occurrence, Public/Proof kind, codec name/kind and atom count. Draw descriptor bytes include full Stage site/mode/arities/family/sampler/carries/result type, Operation, count and powers limit. Existing `registry.rs::{path,message_key,stage_key,compound_key}` (`:16–107`) shows the complete inputs; preserve them in the new canonical encoding. Source-local vector indexes, Rust Debug names and schema handle numbers are not semantic identities.

### 3.2 Runtime checks and commit ordering

The new owner and its registry must implement the following execution sequence:

1. Check active/not poisoned/not closed, match descriptor to **this** installation and next required event, validate payload or complete draw geometry, and resolve every carry to a previously completed result and valid coordinate.
2. Check all lengths/counts/ordinal/query budget arithmetic and reserve the bounded storage needed for that transaction **before** exposing a hash or appending a partial event.
3. For a message, append the canonical typed event to literal history; retain the canonical payload needed for source/seal checks; only then complete its queue event.
4. For a challenge, freeze history, execute all component queries, decode all components, append the complete request + whole decoded result; retain results for carries/curation; only then complete the one logical Draw event and return.
5. Any failure poisons the owner permanently, including failures after a compound/protocol callback. No accepting handoff is possible with a partial transaction, nonempty required queue, unresolved branch or poison.

The existing D1 `FrontendBinding` callbacks illustrate publication ordering, but the new owner needs its own concrete indexed registry interface; it must not implement `InnerBinding`, `inner::Registry`, `InnerExecutionRegistry` or numeric H1610 `MESSAGE_BASE/DRAW_BASE` dispatch to get that ordering. Those interfaces address a different compiler. Reusing `TranscriptError` is sufficient; no fake D1 error conversion is required.

### 3.3 Actual dynamic branches that must survive

**Round representation:** retain `RoundShape` alternatives for trimmed Full and Compressed messages. Existing construction at `segment_emissions.rs:755–817` retains original coefficient exponents, omits exponent1 only for compressed form, and includes the actual header count. `SumPlan::round` (`plans.rs:68–89`) selects the admitted Choice at the current queue position and returns that scope plus the same source-round challenge. The generic verifier enforces degree/round count (`jolt-sumcheck/src/verifier.rs:63–85,109–151`). Do not force maximal-length padded polynomials or change compression arithmetic.

**Stage6b aliases:** retain the eight permitted bytecode-alias masks and canonical original output origins. The actual branch must be derived from the checked challenge-generated points, not a proof flag or equality of scalar values. `BatchPlan::validate_output_openings` checks the source-provided point context against completed draw results before selecting the branch (`plans.rs:187–251`; `segment_emissions.rs:292–318,818–920`). Preserve this receiving-side check in the new plan. Never advance a Choice by merely accepting whichever next message happens to arrive.

### 3.4 Public/source message schedule

Preserve order: complete segment context; common proof preamble; original Main commitment; stage1..7/uni-skips with RAM endpoint evaluations at the original stage4 position; original Main source context; source zero anchor; physical-zero/first-PC/last-PC source protocols; checked direct handoff. The generic stage driver already performs the arithmetic sequence and absorption calls.

`SegmentContext::absorb` and the common preamble encode the exact existing public fields (`segment.rs:363–459`; `verifier.rs:1088–1185`). Source identities and both RAM commitments precede frontend randomness. Main's actual backend payload is canonical/shape-constrained but prover-supplied; it must be consumed before stage1 and matched exactly when rebound for source checks and sealed. Do not incorrectly install one public fixed Main payload before the producer has committed its actual rows.

Use `public.rs:22–339` as the existing field-by-field schema map. Its source-check constructor (`:372–497`) already specifies:

- Original Main commitment + cycles/address_bits/active_rows/first_pc/last_pc.
- RawVector zero anchor count16.
- CoefficientVector counts56,3,3 in PhysicalZeros,FirstPc,LastPc order.
- Degree2 compressed round counts16,4,4.
- Original-column output scopes in each actual `plan.columns(check)` range.

Thus the source suite has24 rounds,62 returned original Main scalars and102 fresh K coordinates: `16+56+3+3+16+4+4`. This is structural accounting, not its soundness coefficient. Do not add normalization's coefficient vector,22 rounds or56 terminal values.

## 4. Exact new descriptors and indexed owner behavior

Install mode `jolt/continued-k/indexed-h256/mod128/v1`. Use actual Rust `jolt_field::Prime128OffsetA7F7` and its canonical LE128 identity. The existing field identity string `q128/K/p=340282366920938463463374607427473266697/le128/v1` is mathematically reusable; referencing it must not make the new module depend on `d1::ChallengeField::sample_reserved` or the D1 owner.

Choose and freeze a new sampler descriptor, e.g. `h256/first16-le128/mod-p/full32/component-indexed/no-rejection/v1`, and a new completed-result codec, e.g. `jolt/continued-k/indexed-result/le128/v1`. Their exact bytes, version and parser are part of installation admission. **Do not retain** the current `h1610/native/K/sequential-le128/reject-ge-p/max-attempts=2/abort/v1` sampler or `q128-k-vector/v1` owner-specific result convention merely because K is unchanged (`segment_emissions/schema.rs:212–278`). Family, sampler and result codec are distinct identities.

Request R binds full descriptors including result type and powers bound; the current D1 outer Request encoder is not automatically this encoding. Freeze literal header/events P and query each component j at:

`Domain || Frame(Hdr) || P || REQUEST || Frame(R) || COORD || LE64(j)`.

Hash is a direct32-byte `Blake2b<U32>` conventional instantiation of the approved ideal H256 interface, not `DigestTranscript`, a digest-of-P seed, H1610, SHAKE expansion from one seed or the ordinary-v8 canonical rejection sampler. Decode `LE128(answer[0..16]) mod p`. Every component is a separate primitive entry. A vector returns only after all components and whole-result completion; output count0 still appends its completed empty tuple but makes0 oracle entries. The saved ideal table retains all32 raw bytes per answer; completed public history stores full canonical decoded fields with full R and owner.

Dispatch explicit allowed frontend operations: RawScalar, ScalarCoefficient, RawVector, CoefficientVector. Reject ShellCoordinate/ShellBundle. Scalar APIs require the matching operation/count1; vector count1 remains a **vector operation**, not a scalar alias. Powers are one coefficient plus deterministic expansion, including output length0, bounded by the installed powers limit.

Canonical message payloads can use the existing typed conventions: K exact canonical LE128 with actual TypeId; U64 LE8; Bytes/Label exact bytes; LabelCount a count1 tuple with framed label and LE64 count; original commitment leaves use their installed exact/native/K-vector schema. Reject E, LegacyField and unregistered Native payloads for this profile. Do not call `append_legacy_value`, reverse fields or pad labels to32. Compound children/empty leaves remain explicit. Public/Proof and codec/count/site are cryptographic event bytes, not logging metadata.

Caps/ordinals are checked and nonwrapping; no resetting counters to extend a proof. Enforce finite context/event/prefix/request/payload bytes, event count, coordinates/powers, aggregate component calls and sticky failure. `Schema::Limits` only caps metadata, not those runtime resources. Hash-prefix caching is permitted only for the unfinalized state of the exact append-only literal input. Finalizer receives the inherited evidence; no raw-state or one-scalar bridge capability is exposed.

## 5. Direct120 seal: exact original-leaf contract

The new plan/owner ends after source checks, not after `verify_main_normalization`. Build a new checked direct-roster seal, reconstructed from completed events and generic verifier results, containing:

1. **Main native56:** `pending.native().main_group()` has all56 canonical IDs in `MainSourceCheckPlan.layout().ids()` order; retain the original commitment, exact physical point and56 evaluations. Existing `one_hot_trace_claim` checks that coverage/common point (`stages/stage8/akita.rs:525–566`).
2. **Physical-zero56:** first `MainColumnOpenings`, columns=`plan.columns(PhysicalZeros)`, returned exact physical point and56 values.
3. **First-PC3 and last-PC3:** next two source openings, each exact bytecode-column range and physical point. `MainSourceCheckPlan::physical_point` injects fixed cycle0/active_rows-1 and reverses low-first challenge coordinates into the physical high-first `(cycle||address)` convention (`segment_source.rs:209–232`). Do not double-reverse it.
4. **RAMminus/RAMplus:** `pending.ram_endpoints()` on the original exact registered handles/contracts, points and singleton values derived by `context.endpoint_groups`. Preserve decoder/view identity; no E2 lift, reconstructed RAM commitment or decoded-equality coalescing.

A recommended stable local scalar order is MainNative56, MainZeros56, MainFirst3, MainLast3, RAMminus, RAMplus, with local indexes0..119. If the parent already owns another canonical global order, use that one authoritative mapping rather than introducing a second convention. Each item retains its source role, original column ID, query/point convention, value and scalar index. Same commitment or accidentally equal point/value does not authorize dropping a pointed equation.

Require exact roster counts/coverage and equality of the Main commitment consumed before stage1, the commitment rebound in `bind_main_source`, the pending Main group and all source leaves. Check seal values/points against the registry's completed draws/messages, not a caller-supplied endpoint ledger. Reject any pending auxiliary/program groups in the selected no-advice/full-public profile. Carry source predicates and complete externally approved context through the seal.

**Do not reuse `SegmentPlan::seal_native_instance` unchanged.** It belongs to the H1610/full-ring path, requires the normalization schedule and calls result-derived normalization handling (`segment_emissions/handoff.rs:117–155,158–293`). Likewise current `PlanData`/`SegmentPlan` require `NormalizationData`/`normalization` fields (`plans.rs:45–65,254–313`). The new indexed plan has no such fields/events and exposes a direct120 handoff. Passing dummy normalization data, marking its events complete without running them, or ignoring missing results would violate the selected protocol.

The parent accumulation/ordinary-v8 owner receives this checked direct120 seal and the full inherited canonical frontend evidence. It appends all120 scalar queries and later binds the complete query/endpoint statement. This slice does not create finished segment PCS proofs or a final decider.

## 6. Exclusive Rust slice filenames and integration boundary

Give **one implementation owner** the following exact source files. This is a concrete ownership proposal; new paths are intentionally new, not claims that they exist.

### New owner and feature/export seam

- `jolt/crates/jolt-transcript/src/indexed_frontend.rs` **new**: direct H256 owner, complete canonical event/request/result encoding, exact modulo decoder, bounded state/history, concrete indexed binding contract, sticky rejection and checked handoff/evidence API. Keep the exact full-answer inverse/kernel contract available to the analysis/test oracle without adding a concrete preimage API.
- `jolt/crates/jolt-transcript/src/lib.rs`: export/gate the new owner, separate from legacy/D1 names.
- `jolt/crates/jolt-transcript/Cargo.toml`: a distinct opt-in indexed-frontend feature with existing digest/Blake2b and solinas dependencies; do not enable H1610 as the implementation of the new feature. No new cryptographic dependency is required for direct Blake2b256.

### New selected installation and direct seal

- `jolt/crates/jolt-verifier/src/indexed_segment_emissions.rs` **new**: admitted selected profile, source-derived stage installation, complete context, event schedule/branch selection, public plan runner and completion boundary.
- `jolt/crates/jolt-verifier/src/indexed_segment_emissions/schema.rs` **new**: metadata builders and exact public/preamble/commitment/RAM/source schemas with new sampler/result descriptor. Derive shapes from existing generic emitters/source plan, not from an observed transcript.
- `jolt/crates/jolt-verifier/src/indexed_segment_emissions/registry.rs` **new**: pure full descriptor admission plus runtime event queue, canonical payload validation, successful publication/result retention, carry checks and no permissive default.
- `jolt/crates/jolt-verifier/src/indexed_segment_emissions/plans.rs` **new**: implementations of the existing frontend/member/batch/sumcheck/segment/source-emission traits, with exact RoundShape and stage6b curation behavior, **no normalization capability**.
- `jolt/crates/jolt-verifier/src/indexed_segment_emissions/seal.rs` **new**: checked direct Main118/RAM2 roster/evidence handoff and exact commitment/point/value/source-context equality.
- `jolt/crates/jolt-verifier/src/lib.rs` and `jolt/crates/jolt-verifier/Cargo.toml`: export/gate the new selected installation for Akita/non-field profile independently of `transcript-d1`; enable only actually used existing source/type dependencies. Do not type its constructor as `CompactInnerTraceVerifierSetup` merely to obtain metadata: take admitted original commitment shape/setup metadata and registered RAM context appropriate to the selected original sources, leaving the ordinary K/K finalization choice to its owner.

### One shared-roster extraction, owned by the same slice

- `jolt/crates/jolt-verifier/src/segment_emissions/roster.rs` **move source, no arithmetic changes**.
- `jolt/crates/jolt-verifier/src/stages/emissions/continued_roster.rs` **new destination**.
- `jolt/crates/jolt-verifier/src/stages/emissions.rs`: declare the shared crate-private roster module with the correct Akita/non-field gating.
- `jolt/crates/jolt-verifier/src/segment_emissions.rs`: replace its private roster module declaration with import of the shared module; keep its H1610 owner, normalization and other behavior unchanged.

Tests for this slice can live in `jolt/crates/jolt-transcript/tests/indexed_frontend_tests.rs` and `jolt/crates/jolt-verifier/tests/indexed_segment_emissions_tests.rs` **new**; their feature declarations belong to the same owner. No checks were run for this report. Later main-owner checks should cover exact literal-key equality, changed-operation/site/count sensitivity, whole-vector completion/empty events, exact modulo/full32 replay, foreign-descriptor rejection, sticky/cap failures, trimmed-round branches, source-derived stage6b aliases, absence of normalization events, and full direct120 original-roster equality.

### Explicitly reserved to parent/integration owner

Do not edit arithmetic implementations in `segment_source.rs`, producer `akita/segment_source.rs`, generic `segment/frontend.rs`, producer `akita/segment.rs`, sumcheck algorithms, stage1..8 relation arithmetic, source decoders or original commitment implementation for this compiler slice. Their generic interfaces already suffice. Do not patch `d1/inner.rs`, `d1/frontend.rs` or `DigestTranscript` into a compatibility shim.

Parent owns actual Candidate1 orchestration/transport and ordinary-v8 integration, including new frame schema if needed: current native receipt/control frame and seal code requires normalization and/or H1610/full-ring ownership. It cannot be advertised as migrated merely because the new owner compiles. Parent must call `prepare_continued[_instance]` then `prove/verify_segment_main` on the **same indexed owner**, obtain the direct120 seal, retain/replay every old/new frontend record, and feed its checked augmentation into final query/ordinary-v8 context. Existing `MainCheckProof`/`SegmentMainProof` arithmetic codecs can be reused unchanged; omit normalization only under an explicitly versioned direct-roster transport, not an unexplained missing field.

## Implementer summary

Build checked MessageSite/DrawSite storage from the one source-derived roster and generic source-check plan. Separate pure descriptor binding from stateful event execution; preserve trimmed-round and result-derived alias Choices. The new owner hashes the literal complete context/history/request/component index, applies first16 LE modulo p, counts every component, and publishes the full request/whole decoded result before completing its slot. End after the62 source leaves and seal **all118 Main + two RAM assertions**. Do not install any H1610/two-attempt sampler metadata, mandatory normalization event, E2/native extension owner, scalar digest bridge, or completed per-segment PCS proof in this selected mode.
