# Akita upstream field and original-source investigation

**Suggested artifact name:** `AKITA_UPSTREAM_FIELD_SOURCE_REF_INVESTIGATION.md`.

This Markdown is the installable artifact body. It was not written to a file: the investigation role prohibits all file writes, including scratch artifacts. Main can install the body without repeating the source investigation. No builds, tests, lints, formatters, git mutations, fetches, or repository edits were run.

## 1. Exact refs and dependencies

Local ref storage establishes:

| Jolt ref | Exact Jolt commit | Selected Akita source |
|---|---|---|
| HEAD / cached `origin/amir/akita-batch` | `0791f79a55c3fadc3d327c78a72228696c338468` | LayerZero-Labs/akita `83574331d4e51f8cce1d8689d05592fa4ef4c138` |
| cached `origin/feat/akita-protocol-integration` | `58e43b8bae1fe3c8c26ce231034a9174398e6907` | LayerZero-Labs/akita `b7b15524aac9f24b636358482f42dbc0fef1adc3` |
| cached `origin/feat/akita-metal` | `6f8d715f206ba8df431c711e135fc5d7b6ae1692` | markosg04/akita `50c26d4568e1d0959a372279e712c69f115b2f8a` |

Evidence: local `.git/HEAD`, its named loose branch ref, `.git/refs/remotes/origin/amir/akita-batch`, and `.git/packed-refs:41-42`; immutable [batch Cargo.toml](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/Cargo.toml#L303-L312), [protocol adapter Cargo.toml](https://github.com/a16z/jolt/blob/58e43b8bae1fe3c8c26ce231034a9174398e6907/crates/jolt-akita/Cargo.toml#L11-L19), and [Metal Cargo.toml](https://github.com/a16z/jolt/blob/6f8d715f206ba8df431c711e135fc5d7b6ae1692/Cargo.toml#L298-L309).

Historical statements below use immutable commit URLs, not dirty worktree bytes. Local-only observations are marked explicitly. A local file changed during the investigation; the relevant compact adapter sections were re-read, and stale earlier line ranges are not used as its current layout.

## 2. Actual upstream field choice

All three selected integrations use the default Akita fp128 prime

`q = 2^128 − 2^32 + 22537`, alias `Prime128OffsetA7F7`.

Actual declarations:

- Protocol integration: `AkitaField = akita_config::proof_optimized::fp128::Field`, `AkitaConfig = ...::D64Full`, and `AkitaScheme::Field = AkitaField`. See [types.rs](https://github.com/a16z/jolt/blob/58e43b8bae1fe3c8c26ce231034a9174398e6907/crates/jolt-akita/src/types.rs#L16-L21) and [scheme.rs](https://github.com/a16z/jolt/blob/58e43b8bae1fe3c8c26ce231034a9174398e6907/crates/jolt-akita/src/scheme.rs#L227-L235).
- Batch/Metal integrations: `AkitaField` remains the default fp128 field, but `AkitaConfig = JoltDenseBounded`; the Jolt policy delegates Field/ExtField to Akita’s `DenseBounded`. See [batch adapters.rs](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-akita/src/adapters.rs#L32-L39), [batch configs.rs](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-akita/src/configs.rs), and [Metal adapters.rs](https://github.com/a16z/jolt/blob/6f8d715f206ba8df431c711e135fc5d7b6ae1692/crates/jolt-akita/src/adapters.rs#L32-L39).
- The selected Akita fp128 presets instantiate `Field, Field`, not an extension evaluation field. See [batch-pinned fp128.rs](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-config/src/proof_optimized/fp128.rs#L1-L7), [protocol-pinned fp128.rs](https://github.com/LayerZero-Labs/akita/blob/b7b15524aac9f24b636358482f42dbc0fef1adc3/crates/akita-config/src/proof_optimized/fp128.rs), and [Metal-pinned fp128.rs](https://github.com/markosg04/akita/blob/50c26d4568e1d0959a372279e712c69f115b2f8a/crates/akita-config/src/proof_optimized/fp128.rs).

The historical Jolt `CommitmentScheme` has one `Field`. Batch `prove` requires `PCS: CommitmentScheme<Field=F>`, `Transcript<Challenge=F>`, and `JoltWitnessPlane<F>`. Its stage-eight call completes the ordinary PCS proof. This is genuine source/eval coupling, not a missing alias. See [batch prover.rs](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-prover/src/akita/prover.rs#L34-L59).

The original protocol ref still also contains the legacy `jolt-core` tree. Its adapter is under root `crates/jolt-akita`, whereas several sibling crates are still root directories. This explains the initial non-fitting `crates/jolt-field` lookup; the actual recursive tree and declarations were used afterward.

## 3. Fresh Prime128Offset275: algebraically fitting, not admitted production setup

`Prime128Offset275 = Fp128<2^128−275>` is an explicitly registered arithmetic prime at the batch ref. See [solinas/mod.rs](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-field/src/solinas/mod.rs#L120-L162).

Existing fitting evaluation types are:

1. **`Ext2<Prime128Offset275>`**, default `U²=2`.
2. **`FpExt4<Prime128Offset275>`**, the existing cyclotomic ring-subfield basis `[1,e1,e2,e3]`, with defining polynomial `x⁴−4x²+2`. This is not `x⁴+1` and not an arbitrary tower basis.

The source explicitly states the field condition `p≡±3 (mod 8)`. Here `2^128−275≡5≡−3 (mod 8)`, so both fit. See [ext.rs header/default Ext2](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-field/src/solinas/ext.rs#L1-L52), [quartic declaration](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-field/src/solinas/ext.rs#L271-L285), and the existing [Ext2<Prime128Offset275> codec fixture](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-field/src/solinas/ext.rs#L876-L882). These are observed source declarations and tests, not executed checks.

**Cardinality precision:** the quadratic has a 256-bit integer cardinality/32-byte encoding, but `(2^128−275)^2 < 2^256`. If “≥256 bits” literally means `log2 |E| ≥256`, quadratic does not meet that strict inequality; the existing quartic does. Quartic cardinality is between `2^511` and `2^512` and uses four base coordinates. The requirement should not be silently reinterpreted.

Akita’s existing [FpExtEncoding implementations](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-types/src/field_reduction.rs#L13-L73) cover the default `Ext2<F>` and `FpExt4<F>`. Its typed `embed_ring_subfield_vector` consumes that encoding. Thus this alternative does not require inventing extension arithmetic or a new ψ encoding.

By contrast, A7F7 is `1 mod 8`, so its default NR2 Ext2 and existing quartic are rings. Local NR5 `Prime128OffsetA7F7Ext2` is a distinct full-ring path; ordinary `FpExtEncoding` is not generically implemented for arbitrary `FpExt2<F,C>`. Adding that trait with the same coordinates would not establish compatibility with ψ.

**Concrete production blocker, not a blanket gate list:** the batch-pinned `SisModulusProfileId` is exhaustively `Q32Offset99`, `Q64Offset59`, `Q128OffsetA7F7`. There is no Q128Offset275 profile. `validate_config_policy` calls `validate_sis_modulus_profile`, which computes the actual field modulus and requires exact profile equality before trusted catalog admission. See [SIS enum](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-params/src/sis/ajtai_key/table.rs#L154-L205) and [config validation](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-config/src/lib.rs#L302-L321), [exact modulus check](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-config/src/lib.rs#L392-L413).

Therefore:

- **Yes:** the actual existing arithmetic/encoding pair is fitting for fresh setup.
- **No:** none of these fitting refs supplies a production Jolt+Akita 275/Ext2-or4 configuration and admitted catalog that can be selected unchanged.
- Registering an enum name alone is not enough; the exact new modulus needs its admitted security data and matching setup/catalog policy. No equivalence with the A7F7 SIS evidence was established.
- Fix Base/Eval before creating fresh source owners, matrices, commitments, or signed planes. This does not authorize migration of existing A7F7 commitments or reinterpretation of old planes under 275.

## 4. Real ordinary batched source API already exists

The batch-pinned Akita backend owns sources through `SourceHandle<Base,Eval>` and `CommitmentHandle<Base,Eval>`. `CommittedSource` retains `Arc<dyn PrivateSource<Base,Eval>>`, source metadata, exact profile, producer contract, public commitment, and portable retained material. `import_source(Vec<P>)` moves owned sources into this owner. See [owned.rs](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-cpu-backend/src/opaque/owned.rs#L37-L69), [import_source](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-cpu-backend/src/opaque/owned.rs#L403-L428), and [commit](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-cpu-backend/src/opaque/owned_commit.rs#L87-L137).

The actual ordinary constructor is:

`SelectedProverOpeningData::from_committed_claims<Cfg>(claims, handles, trusted_catalog)`.

It resolves the commitment-group profile, validates points/evaluation arities against group layouts, checks handle metadata, and requires the **final handle’s producer contract** to equal `Cfg::committed_source_contract()`. See [opening.rs:19–87](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-prover/src/opening.rs#L19-L87).

Completion is:

`AkitaCommitmentScheme<Cfg>::batched_prove(setup, opening, backend, session_bytes, BasisMode::Lagrange)`.

Its opening type carries `Cfg::ExtField` claims and `CommitmentHandle<Cfg::Field,Cfg::ExtField>`. It supports retained heterogeneous source groups at their group-local points; a batch is not restricted to one precombined polynomial. See [scheme/mod.rs:113–153](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-pcs/src/scheme/mod.rs#L113-L153). The Jolt [native_batching.rs](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-akita/src/native_batching.rs) wraps these existing operations and states that it does not itself perform claim combination/reduction.

Thus the ordinary retained-source batch primitive is present. The current NR5 full-ring → ordinary bridge is not that upstream primitive; it is local receiving-capability work.

## 5. Historical extension frontend versus local mixed frontend

Upstream Akita already has mixed Base/Eval PCS paths: fp32 presets use `Prime32Offset99` with `FpExt4<Base>`; fp64 uses `Prime64Offset59` with `Ext2<Base>`. See [fp32.rs](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-config/src/proof_optimized/fp32.rs#L5-L8) and [fp64.rs](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-config/src/proof_optimized/fp64.rs#L5-L8).

Upstream Jolt also has generic extension sumcheck tests, with `Quadratic=Ext2<Prime64Offset59>`, `Quartic=FpExt4<Prime32Offset99>`, and a separate base `Prime128Offset275` fixture. See [extension_fields.rs](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-sumcheck/tests/extension_fields.rs#L9-L48). These prove existing source-level support, not a mixed source/eval zkVM frontend.

No complete mixed Base/Eval Jolt frontend was found in the three selected refs. Their ordinary frontend and witness interfaces are single-field; [historical reference views](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-kernels/src/reference/views.rs#L19-L27) read dense `JoltWitnessOracle<F>` directly. The immutable producer directories lack the local continued/pending source modules.

**Local only:** current general `akita/prover.rs::prepare` accepts evaluation F extending `PCS::SourceField`, and borrows one concrete witness through `WitnessViews<K,E>`. That handle requires one owner implementing both witness planes and does not materialize tables. Current reference mixed kernel fixtures instantiate K=A7F7, E=NR5 E2. The field-inline Akita end-to-end tests inspected still use `AkitaField`; “mixed batch” there refers to object kinds, not source/eval fields.

**Local continued path remains K/K:** prover `prepare_continued` and verifier `prepare_continued_instance` explicitly require `PCS::SourceField=F, EvalField=F`. Reusing the local borrowed-view convention is the fitting interface direction; the refs do not provide an already-finished mixed continued frontend.

## 6. Pending pre-PCS cut and challenge ordering

**Upstream:** `akita/prover.rs::prove` runs through `prove_stage8` and returns a completed proof.

**Local only:** `PendingNativeStage8Prover` holds the checked public instance plus original auxiliary hints and Main hint. `into_parts()` moves them to the inner owner without completing a segment PCS. Its separate `prove()` calls ordinary `PCS::prove_batch`. `prepare_continued` returns this pending boundary. Local Akita also has `begin_batched_reduction` / `PendingNativeProver`, which retains a live proof scope and pending native opening; this is not the upstream Jolt frontend interface.

**No hoisting defect:** current `segment/frontend.rs:230–239` absorbs context and calls `validate_and_seed_transcript` before `stage1::verify` at line 263. `validate_and_seed_transcript` validates inputs, absorbs preamble, then absorbs commitments. The Akita order is Main/OneHotTrace, untrusted advice, trusted advice, field increment when enabled, then direct bytecode chunks/program image from preprocessing. See local `verifier.rs:443–477,828–920`; the historical batch verifier also already contains the corresponding commitment-seeding functions. Continued mode clearing initialization consumers on nonfirst segments happens after this seeding and does not erase commitment binding.

## 7. Typed Main56/RAM and genuine original ownership — local additions

The current compact adapter is not a generic arbitrary-polynomial commitment scheme. Its admitted Main geometry is 4096 cycle rows, k16 one-hot chunks, three bytecode and four RAM address columns, and a 56-polynomial/16-variable final group. `TraceOneHotColumn` stores `Arc<dyn TraceOneHotRows>` and each column clones the Arc, not a full dense K/E coefficient grid. The compact adapter imports those columns as full-ring source ID 1.

The registered compact source geometry is enforced in `akita-types/src/proof/setup_roles.rs:448–475`:

- Main: 56 polynomials, 1 block, 512 positions, 1 plane, rank 3.
- RAM: 1 polynomial, 16 blocks, 32 positions, 26 planes, rank 3.
- Source IDs distinguish receiver, Main, RAM, and both shrinking descendants; the two RAM endpoint occurrences share source ID 2 but are distinct physical occurrences. Matrix roles and both compression planes stay distinct.

Local `PackedRamSource` owns exactly the input signed base-32 digit paths, including legal noncanonical logical representations. It does not reconstruct a K coefficient and redecompose it. It stores no six-slot raw-grid padding; declared physical A padding must be zero. Commitment and fold operations borrow the same packed digit storage. See local `packed_ram.rs:20–125,234–280`.

`CompactRamContract` binds the complete installed source/profile/raw positional decoder. A RAM endpoint’s ordinary logical/physical domain is the original coefficient polynomial, not its 21-variable padded raw-digit query grid. `commit_compact_ram` moves the supplied original source into the full-ring importer and commits that exact owner. See local `compact_ram.rs:18–76,97–151`.

The local ordinary bridge stores `ProjectedOriginal { original: Arc<CommittedSource<F,EOrigin>> }`; it neither clones packed polynomial data nor supplies legacy tensor/EOR capability. `import_full_ring_commitment` checks shared installed descriptors, raw source identity, geometry, original producer contract, recomputes commitment execution and retained material, then derives **additional quotient-lift material** for the ordinary receiving owner. This is real retained ownership, but not a zero-cost handle cast. See local `ordinary_projection.rs:106–205`.

Its admitted ordinary opening is native k1h1 coefficient packing with ring dimension 128. It explicitly does not project arbitrary E2/raw-plane queries to K. Candidate1 must perform its valid final query reduction before using this capability; this import function is not that reduction theorem.

## 8. “Jolt64bit” semantics and historical bounded-source correction

No exact `Jolt64bit`/`Jolt64Bit` declaration was located in the selected refs or inspected related preset history after case-insensitive searches, broader declaration searches, and an exact-name web search. Do not invent that type or treat it as the selected field’s name.

The actual batch/Metal source-width policy is `JoltDenseBounded` delegating to `fp128::DenseBounded`. It means **u64-valued source coefficients within a 128-bit field**, not a 64-bit proof field. Its declared signed width is **65**, giving `[-2^64,2^64−1]`; width 64 would omit the upper half of unsigned u64. Openings retain the full 128-bit field width. See [DenseBounded docs/constants](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-config/src/proof_optimized/fp128.rs#L50-L107).

This policy was introduced by exact Akita commit [`2322d485bac3efd21014b34dda6a600cd9d6ae1a`](https://github.com/LayerZero-Labs/akita/commit/2322d485bac3efd21014b34dda6a600cd9d6ae1a), “bounded committed dense sources”. Source class and magnitude bound are now independent; one-hot status cannot be inferred from `log_commit_bound==1`. The earlier protocol pin used D64Full, not this later bounded policy. Its D64 is a ring dimension, not a source/field bit width.

RV64 instruction semantics are a separate concept again. Main Boolean columns and original 26-plane full-width RAM contracts must not be silently replaced by DenseBounded65 because the VM executes 64-bit instructions.

## 9. Integration, statement, and measurement risks

1. **Fresh field choice:** 275+default Ext2 or quartic is an existing algebraic option. It is not an already-admitted production alternative in these refs. Fix it at setup; keep existing A7F7 source owners unchanged.
2. **Small interfaces already present:** ordinary retained-source constructor/batch PCS, current pending frontend handoff, and same-owner borrowed K/E witness views. Do not reimplement these primitives under new names.
3. **Interfaces still missing at the continued seam:** source/eval separation remains explicitly closed there, and compact source/setup adapters are concretely A7F7/NR5. Completing an E frontend is not merely changing a transcript alias.
4. **Evidence not supplied by the refs:** an admitted 275 SIS policy/catalog and the accumulator’s final-query/extraction composition argument. Presence of `Field`/`FpExtEncoding` traits or successful pending replay would not supply either.
5. **Statement binding:** commitments are already absorbed before stage 1. The remaining concern is exact whole-run/context/program/RAM adjacency binding through pending handoff and final receipt, not a fabricated early-challenge hoist defect.
6. **Source geometry:** Main56 logical columns, physical holes, RAM digit plane order, noncanonical signed paths, both F/H planes, original profiles, and public group order are authenticated inputs. They cannot be replaced by shape-equivalent aliases or a decode/recommit step.
7. **Performance:** ordinary projection currently replays the full commitment executor and constructs quotient-lift material. Report this separately or include it symmetrically; Arc retention alone does not make import free. Dense reference witness views materialize tables; shared source retention is not a claim that all frontend evaluation is streaming. Existing A7F7 x86 multiplication has a special BMI2/ADX dispatch, while 275 follows the general dispatch ([fp128.rs](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-field/src/solinas/fp128.rs#L339-L352)); field changes need new measured evidence, not transferred timings.
8. **Control inspected, not measured:** the local ordinary-control test imports the same original Main/RAM sources, checks endpoint evaluations, then proves one ordinary batch per segment. This establishes what its source-level control is intended to measure; no execution or timing result is claimed here.

Main should exercise its requested 1/2/3/256-segment acceptance cases after integration, including malformed chain/pending-finalization/source-owner cases and matching complete-PCS controls. This investigation ran no checks and produced no performance numbers.