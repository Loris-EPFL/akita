# Akita PCS

Akita is a high-performance, modular lattice polynomial commitment scheme with transparent setup and post-quantum security.

Akita is the public scheme name for this implementation and the intended repository/package name is `akita-pcs`.
The codebase is being decomposed into a focused `akita-*` crate family rather than remaining a single monolithic package.

The current workspace exposes the main ownership boundaries under `crates/`:

- `akita-error` owns the shared protocol error and reusable checked integer formulas for exact sizes, offsets, and ranges.
- Jolt's `jolt-field` package owns shared field arithmetic; `akita-serialization` and `akita-algebra` own Akita encoding, NTT, ring, and polynomial utilities.
- `akita-transcript`, `akita-challenges`, and `akita-sumcheck` own Fiat-Shamir transcripts, challenge sampling, and generic sumcheck machinery.
- `akita-types` owns shared proof, setup, schedule, layout, SIS, and commitment data shapes used by both roles.
- `akita-planner` is the `Cfg`-free offline schedule engine: candidate expansion, schedule-search DP, and external artifact emitter. It sits *below* `akita-config`.
- `akita-schedules` owns versioned schedule artifacts, semantic row validation, and validated owned runtime catalogs.
- `akita-config` owns concrete runtime config presets and the single `CommitmentConfig` policy trait. A config identifies the expected artifact family and policy but does not own rows.
- `akita-setup` owns config-backed setup construction and optional setup cache persistence.
- `akita-verifier` owns verifier replay without prover-only polynomial backends. It receives the validated schedule catalog explicitly.
- `akita-prover` owns commitment, proving, setup expansion, recursive/ring-switch witness construction, and polynomial backends.
- `akita-pcs` is the umbrella package: it owns the end-to-end `AkitaCommitmentScheme` orchestration, re-exports the broad public surface, and hosts examples, benches, and integration tests. (There is no separate `akita-scheme` crate.)

Verifier-only consumers should prefer the slim role crates directly:
`akita-verifier` for verification, `akita-types` for proof/setup/claim shapes,
and `akita-config` for concrete schedule/config policy. The umbrella
`akita-pcs` package is convenient for examples and end-to-end use, but it also
pulls in prover-facing APIs.

## Documentation

The [Akita Book](book/README.md) is the **canonical target** for narrative
documentation (how the scheme works, how to use it, and the foundations). Most
chapters are still stubs that cite source paths and specs to fold; until prose
lands, integrators should read the [Akita Book](book/README.md) (start with
[`book/src/how/architecture.md`](book/src/how/architecture.md)),
[`book/src/usage/commitment-api.md`](book/src/usage/commitment-api.md),
and [`profile/akita-recursion/README.md`](profile/akita-recursion/README.md).
Build the book locally with `./scripts/serve-book.sh` (see
[`book/README.md`](book/README.md) for the toolchain). `AGENTS.md` is the
agent command runbook; `docs/` holds maintainer contracts (crate graph,
verifier contract, CI timing). `specs/` holds design records (lifecycle in
[`specs/PRUNING.md`](specs/PRUNING.md)). Documentation guardrails (CI + PR
comments) are in [`docs/documentation.md`](docs/documentation.md).

## External schedule artifacts

The runtime schedule contract is `TrustedScheduleCatalog<Cfg>`. A versioned
JSON artifact stores complete expanded rows. The application loads that
artifact as a config-bound trusted parameter for both proving and verification.
A proof contains only the 32 byte `OpeningScheduleSelection` row digest. It
cannot provide or replace schedule content.

Artifact loading checks the expected family, protocol epoch, config policy,
runtime challenge hooks, every expanded schedule invariant, every committed
profile, and every row digest. Honest prover key selection and verifier digest
selection then use the same owned catalog.

`AkitaCommitmentScheme<Cfg>` instance owns one validated catalog and uses it
for setup, commitment, proving, and verification. Direct verifier and prover
orchestration accept `&TrustedScheduleCatalog<Cfg>`, so an integration can load
an external trusted artifact without compiling its rows into Akita.

Full-ring execution keeps two explicit setup laws. Registered inner sources use
the native seed-derived shared prefix; `FullRingSetupDescriptor::outer_direct`
retains the separately provisioned outer `4G+4` inventory (original per-handle
A/B/F, shared D/H, distinct terminal A). `from_registered_inventory` checks the
inner/recursive view inventory without importing outer geometry.
Each source retains its exact registered raw view (including legal storage
permutations), canonical K encoding and A/B/F roles. The installed
`FullRingLevelRoles` list fixes physical source order, D/H roles and the full
commitment or A-only successor of each nonterminal level; it cannot be chosen
from proof-supplied geometry. Missing, unused or inconsistent role occurrences
reject structurally, before the separate trusted catalogue admission.
`InstalledFullRingSetup::from_shared_prefix` verifies the complete native
expansion against its seed and checks capacity before retaining one shared owner.
Each role reshapes its own exact prefix; equal numerical views do not merge source
identities. `from_provisioned` is restricted to independent outer-direct matrices.
`CpuBackend::from_full_ring_setup` retains the selected material and its cache
ownership. The caller must authenticate the installation, approve its setup law,
and install a trusted catalog; these constructors issue no security certificate.

`akita_config::full_ring::CompactInner` selects the fixed NR5-E2 mixed root
(Boolean Main, signed receiver and two original RAM endpoints). Regenerate its
single complete catalog through the same emitter:

```bash
cargo run --release -p akita-planner --features catalog-gen \
  --bin gen_schedule_artifacts -- artifacts/schedules akita-full-ring-e2-compact-inner-v1
```

It has no scalar producer catalogs. `CpuBackend::commit_full_ring` admits the
receiver/RAM frozen producer contracts and reuses the existing A/B/F executor;
the consuming opening must still admit the full mixed schedule. RAM uses
`PackedRamSource` and `OriginalBalancedDigitPlanes`, preserving every original
plane instead of reconstructing and redecomposing K values.

`compact_inner_setup_descriptor` derives the 33 semantic roles from the checked
catalog; `install_compact_inner_setup` installs their views on an authenticated
native expansion. The minimum shared capacity is 798,720 K coefficients, not the
sum of 33 separate arrays. Bind the seed/derivation descriptor and actual static
data in the complete transcript and final receipt. This installs fresh sources;
it cannot reinterpret existing independently keyed commitments.

Fresh balanced sources use the existing two degree-five product-range stages
and a degree-three raw-source realization. Virtual holes are zero before
transforms; required physical zeros remain explicit constraints. The versioned
typed sites bind the variant, round count and optional-zero presence before
draws. The endpoint is a pending opening of the original source, not proof
acceptance. `SourceDomainPlan::range_ledger` exposes its exact local accounting.

For an ordinary comparison with already frozen final and precommitted profiles,
`akita_planner::find_schedule_for_frozen_profiles` preserves their full ordered
contracts and rejects instead of falling back to changed source geometry.
Successful offline schedule admission does not supply a missing runtime source
adapter or establish accumulation/zkVM security or performance parity.

Load artifact bytes from application-owned storage and construct the scheme:

```rust
let bytes = std::fs::read("parameters/fp128_onehot.aks")?;
let scheme = AkitaCommitmentScheme::<fp128::OneHot>::from_schedule_artifact(&bytes)?;
```

Git tracks deterministic family artifacts under `artifacts/schedules/`.
Regenerate them after changing planner policy, candidate search, or artifact
structure:

```bash
scripts/generate-schedule-artifacts.sh
```

For a faster planner iteration loop, pass one or more family names, for example
`scripts/generate-schedule-artifacts.sh fp32_dense`. Run the unfiltered command
before committing planner changes.

The schedule-artifact drift job regenerates every family into a temporary
directory and rejects any byte difference from the tracked artifacts.

## Experimental pre-PCS query-relation accumulation

`akita_pcs::accumulation` accumulates equations about retained **original**
sources before complete per-source PCS proofs. It does not add commitments
homomorphically or replace them with recommitted folded sources. Finalization
uses one sealed degree-two query and one complete ordinary native root,
shrinking suffix and terminal proof. Raw original RAM additionally needs the
global range program before the query seal.

Run the deterministic original Dense nv14 engine with:

```bash
cargo run --release --offline --locked -p akita-pcs --no-default-features \
  --features transcript-blake2b --example accumulation_relation_benchmark \
  -- --segments 256
```

The executable independently verifies the complete receipt and rejects its
mutation matrix before emitting metrics. Counts 1, 2, 3 and 256 are supported.
This engine benchmark is not a VM or recursive-aggregation parity result.
Witnesses and public evidence remain linear in the retained workload; this is
not zero-knowledge or constant-state IVC.

With the matching sibling Jolt checkout, `bash autoresearch.sh` from this
repository runs both Dense and genuine Jolt matrices at all four counts. Its
primary metric remains the Dense engine time. Engine-only success does not
establish the full delivery gate or recursive-aggregation parity.

Native proving and verification require an explicitly installed indexed
registry and inherited augmentation. `prepare_native_proof_context` and
`prepare_native_prover_context` compile that context from the trusted row,
ordered endpoint statement, session and literal completed prefix history.
Do not derive the accepted grammar by recording an honest proof.

See the [protocol and measured gates](docs/query-relation-accumulation/AKITA_RELATION_ACCUMULATION_PROTOCOL.md)
for source views, lattice/conditional-ROM assumptions and current acceptance
limits, and the [paired-checkout instructions](docs/query-relation-accumulation/README.md).
The public query verifier never needs private source handles.

## Lineage

Akita keeps the earlier implementation lineage explicit while giving the improved scheme its own name.
This is also the line where planned protocol improvements over the original design live: faster verifier-oriented reductions via matrix-claim delegation, smaller large-field proofs via field-size lowering, and efficient zero-knowledge techniques under the Whiteout design direction.

## Contributing

Major features and architectural changes should start with a short spec.
See [CONTRIBUTING.md](CONTRIBUTING.md) and [specs/TEMPLATE.md](specs/TEMPLATE.md) for the review workflow.

## Acknowledgements

The CRT/NTT and small-prime arithmetic design in this repository is informed by the Labrador/Greyhound C implementation family. In particular, the pseudo-Mersenne profile uses moduli of the form `q = 2^k - offset`. Akita provides a Rust-native architecture and APIs, while drawing algorithmic inspiration from those implementations.
