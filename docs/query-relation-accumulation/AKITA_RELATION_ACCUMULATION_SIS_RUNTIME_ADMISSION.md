# Candidate1: SIS admission, ordinary schedule budgets, and missing runtime price

## Finding

**The existing interfaces certify collision geometry against generated SIS floors and enumerate local transcript losses. They do not price the FULL knowledge extractor/collision-finder runtime, global extraction-tree size, or a runtime-conditioned lattice advantage.** There is one important qualification: independent claim batching explicitly uses an extractor-aware *local* tree loss (`values / |E|`), not just a fixed-discrepancy Schwartz–Zippel loss. This still does not export the composed tree count or cost.

Current shipped SIS admission is **Quantum128BitADPS16**, not a 192-bit requirement. The generated header says its sole hard gate is ADPS16 quantum LGSA >=128 modeled attack-cost bits (`akita/crates/akita-params/src/sis/generated_sis_table/mod.rs:1–10`). This is a lattice-table gate, NOT a tight end-to-end 128-bit soundness requirement. `AKITA_INSTANCE_ACCUMULATION_FIELD_POLICY_COMPARISON.md:74–100`, found by glob, explicitly separates setup deviation, runtime-priced MSIS and ROM losses and rejects the extra tight128 requirement. A meaningful explicit finite bound remains necessary; no extra global bit threshold is inferred here.

Read-only source/static-artifact investigation. No estimator, control, build, test, lint, formatter, or git command was run. No theorem about Candidate1/source-first extraction is asserted.

## 1. What actually admits SIS parameters

All paths below are relative to the repository root.

### Coefficient-L∞ route

`akita/crates/akita-params/src/sis/ajtai_key/table.rs`:

- `SisSecurityPolicyId` has one variant, `Quantum128BitADPS16` (`:76–117`).
- `SisTableKey { policy, table_digest, modulus_profile, role, ring_dimension: u32, coeff_linf_bound: u128 }` (`:213–229`). There is no time, adversarial query, collision-count or tree-count axis.
- `ceil_supported_linf_bound(policy: SisSecurityPolicyId, table_digest: SisTableDigest, sis_modulus_profile: SisModulusProfileId, role: SisMatrixRole, d: u32, linf: u128) -> Option<u128>` (`:231–274`). Rejects zero; A rounds to an audited role-specific collision cell; B/D round to gadget anchors; unsupported role/profile/dimension/table combinations return `None`.
- `sis_table_key_for_linf_bound(...) -> Option<SisTableKey>` has those same argument types (`:276–301`).
- `min_secure_rank(key: SisTableKey, width: u64) -> Option<usize>` (`:337–360`) finds the first table rank whose maximum ring width covers `width`, restricted by role-cell coverage. It is a width/rank lookup, not an advantage calculation.

`akita/crates/akita-params/src/sis/ajtai_key.rs`:

- `InnerCommitMatrixParams::try_new(policy, table_digest, sis_modulus_profile, output_rank: usize, input_width: usize, coeff_linf_bound: u128, ring_dimension: usize) -> Result<Self, AkitaError>` (`:153–179`); `try_new_with_min_rank(key: SisTableKey, input_width: usize) -> Result<Self, AkitaError>` (`:181–189`).
- Their shared `audit_commit_matrix_fields` (`:33–93`) requires nonzero rank/width, checked coordinate conversions, supported table key and covered width, and `output_rank >= floor`. It does not require equality with the minimum rank.
- B/D use `LinfCommitMatrix<R>::try_new` and `try_new_with_min_rank` with the same geometry (`:522–551`); `validate` replays the canonical key/floor audit (`:563–583`). A `validate` checks its selected route (`:275–315`).

Coverage limits are explicit: B/D anchors `[3,7,15,31,63,127,255]`, maximum searched module rank 20, generated width cap 6,400,000,000,000; exact A response-difference exponents and profile ceilings are finite (`akita/crates/akita-params/src/sis/coverage.rs:27–94`). `sis_role_cell(role, profile, d, bound) -> Option<SisRoleCell>` rejects unsupported geometry/bounds and `bound >= (q-1)/2` (`:228–276`). These are admission/domain limits, not extractor budgets.

### “Complete collision price” means norm, not computational cost

`akita/crates/akita-params/src/sis/norm_bound.rs:25–107`:

- `weak_binding_inf_norm(challenge_l1_norm: u128, z_inf_norm: u128) -> Option<u128>` computes `2*c*z`.
- `role_a_collision_inf_norm_for_response_bound(challenge_l1_norm: u128, response_linf_bound: u128) -> Option<u128>` computes `8*c*z`, including two differing challenges and two accepted responses.
- `role_a_collision_inf_norm_for_response_difference(challenge_l1_norm: u128, response_difference_linf: u128) -> Option<u128>` computes `4*c*diameter`; the response difference already includes both endpoints.
- `role_a_collision_l2_sq_for_response_bound(challenge_operator_bound: u128, response_l2_sq_bound: u128) -> Option<u128>` computes `64*c²*response_l2_sq_bound` for the COMPLETE physical vector. No additional width/embedding factor is intended.

`akita/crates/akita-schedules/src/candidate.rs:114–141` exposes `projected_collision_role_price(policy: &PlannerPolicy, role: SisMatrixRole, source_dimension: usize, role_dimension: usize, native_width: usize, log_basis: u32) -> Option<(SisTableKey, usize)>`. It rejects A/nondividing dimensions, prices B/D norm and projected physical width, and returns a key plus width. Despite “price” in the name, there is no runtime or probability output.

### Euclidean route

`akita/crates/akita-params/src/sis/l2_table.rs:31–108`:

- `SisL2TableKey { policy, table_digest, modulus_profile, ring_dimension: u32, collision_l2_sq: u128 }`.
- `ceil_supported_l2_collision_sq(u128) -> Option<u128>` rounds nonzero complete squared norm to powers of two in `[2,2^84]`.
- `sis_l2_table_key_for_collision_sq(policy, table_digest, modulus_profile, ring_dimension: u32, collision_l2_sq: u128) -> Option<SisL2TableKey>` requires current policy/digest and an existing generated cell.
- `min_secure_l2_rank(key: SisL2TableKey, width: u64) -> Option<usize>` rejects zero width and unsupported identity, then finds the first covered rank.

`InnerCommitMatrixParams::try_new_l2_with_min_rank(table_key: SisL2TableKey, input_width: usize, response_l2_sq_cap: u128, norm_proof_shape: PhysicalL2NormProofShape) -> Result<Self, AkitaError>` (`akita/crates/akita-params/src/sis/ajtai_key.rs:191–221`) requires nonzero width/cap and norm-proof physical response length exactly `input_width*d`. The schedule-level canonical helper `selective_l2_inner_matrix(policy: &PlannerPolicy, geometry: SelectiveL2CandidateGeometry<'_>) -> Result<Option<InnerCommitMatrixParams>, AkitaError>` (`akita/crates/akita-schedules/src/candidate.rs:15–95`) additionally requires fold level >=3, singleton claim/chunk, fold basis >=8, explicit response cap, derives the operator/L1 collision norm, and uses that table. Runtime row audit replays the supplied cap rather than rerunning the offline response model.

## 2. What the estimator supplies (and does not supply)

- `SisParameters { n: u32, q: BigUint, m: Option<u64>, length_bound: Bound, norm: SisNorm }`; `try_new(n, q, m, length_bound, norm) -> Result<Self>` (`akita/crates/akita-sis-estimator/src/params.rs:111–175`). `Bound` supports exact integer/rational/square-root and floating input (`:17–33`).
- `scalar_sis_from_ring_wide(family: AkitaModulusProfileId, ring_dimension: u32, rank: u32, width: u64, coeff_linf_bound: u64) -> Result<SisParameters>` maps `n=rank*d, m=width*d, B=bound` (`akita/crates/akita-sis-estimator/src/akita.rs:93–138`). The Euclidean counterpart replaces B by `sqrt(collision_l2_sq)` without multiplying the already complete norm by width (`:140–185`).
- `EstimateConfig` fields are reduction model, basis-shape model, optimizer, `success_probability`, optional lattice dimension, optional proven-pruned decision target, numeric policy (`akita/crates/akita-sis-estimator/src/config.rs:194–214`). The production infinity profile sets ADPS16 quantum and a 128.0 decision target (`:216–233`; policy constraint `:37–71`). Default success probability is 0.99 (`numeric.rs:32–36`), not a declared attack-time budget.
- `LatticeCost { rop, red, sieve, delta, beta, eta, zeta, d, prob, repetitions }` (`akita/crates/akita-sis-estimator/src/cost.rs:53–77`) is a modeled attack cost, not `Adv_SIS(time,queries)`. `CostValue::ProvenAboveTarget` is only a certified lower bound; the exact larger cost is intentionally absent (`:21–32`). One cannot treat it as an exact attack-cost number.
- `FoldSchedule::sis_occurrences(&self) -> Result<Vec<ScheduleSisOccurrence>, AkitaError>` (`akita/crates/akita-params/src/schedule/sis_occurrences.rs:42–91`) validates structure and enumerates A/B/D, compression maps, frozen groups and setup prefixes. An occurrence has `location, role, output_rank, input_width, ring_dimension, modulus_profile, bound`. These are matrix occurrences, NOT the number of collision attempts made by extraction. Topology is explicit at `:94–184`.
- `estimate_schedule_security(schedule: &FoldSchedule) -> Result<ScheduleSecurityEstimate>` (`akita/crates/akita-sis-estimator/src/schedule_security.rs:72–149`) directly estimates every occurrence and retains the minimum modeled `log2(rop)`. No summation over reduction attempts or extraction time occurs. Its inputs are just each occurrence's geometry/norm. `instances()`, `minimum()`, `minimum_security_bits()` expose that diagnostic (`:44–69`).
- Only the optional planner `catalog-security` feature pulls in the estimator (`akita/crates/akita-planner/Cargo.toml:17,37,63–66`). `examples/catalog_security.rs:130–141,163–217` loads an admitted catalog, calls the estimator and optionally checks the minimum against the policy's 128-bit modeled floor. It does not produce a runtime-conditioned knowledge bound. No estimator run is claimed here.

**Unsupported:** no mapped estimator/schedule/config API accepts the full extractor runtime or adversarial Q, nor returns a lattice advantage conditioned on them. Configurable attack-model success probability is not such an interface. In particular, no source-backed rule here justifies substituting `time / 2^security_bits` for an arbitrary algorithm's advantage.

## 3. Ordinary transcript/error-budget interface

### Public shape to local loss plan

`akita/crates/akita-params/src/transcript_grinding/plan.rs:27–64`:

```rust
pub fn transcript_grinding_cost_for_planner_edge(
    params: &CommittedGroupParams,
    relation_geometry: RelationAddressGeometry,
    layout: &OpeningClaimsLayout,
    successor: FoldSuccessor<'_>,
    challenge_order: ChallengeFieldOrder,
    claim_extension_degree: usize,
    level: u32,
) -> Result<TranscriptGrindingCost, AkitaError>
```

Full-plan function (`:67–155`):

```rust
pub fn derive_transcript_grinding_plan_from_public_shape(
    schedule: &FoldSchedule,
    root_layout: &OpeningClaimsLayout,
    challenge_order: ChallengeFieldOrder,
    claim_extension_degree: usize,
) -> Result<GrindingPlan, AkitaError>
```

Typed adapter: `akita_config::derive_transcript_grinding_plan<Cfg: CommitmentConfig>(schedule: &FoldSchedule, root_layout: &OpeningClaimsLayout) -> Result<GrindingPlan, AkitaError>` validates concrete field modulus and extension tower (`akita/crates/akita-config/src/transcript_grinding_plan.rs:8–34`).

`GrindingRun` stores `site, loss_factor: u64, grind_bits: u8, nonce_bits: u8, multiplicity: u64`, with public accessors (`akita/crates/akita-params/src/transcript_grinding.rs:381–445`). `TranscriptGrindingCost` stores only `total_nonce_bits: usize, nonce_max_bytes: usize, expanded_query_count: u64` (`:543–564`). `GrindingPlan` adds runs/challenge order and validates expanded logical queries `< u32::MAX` (`:642–698`; constant `:27–31`).

Losses are polynomial degree, multilinear point width, powers-batch degree, and independent-batch width; ring-switch alpha is `2*d-1` for ET or `2*s-1` for packing (`transcript_grinding.rs:719–771`). **Independent batching is extractor-aware locally:** its comments explicitly charge a coordinate-wise tree with `values+1` children and loss `values/|E|`, rather than the fixed-vector `1/|E|` (`:736–747`). `append_claim_batch` uses the total opened-polynomial count (`plan.rs:450–466`). This qualification prevents claiming the plan contains no extraction reasoning at all.

Ordinary nonterminal plan assembly (`plan.rs:233–370`) records alpha, tau0 relation-point width, tau1 row-point width, Stage1 full round degree and interstage batch loss, optional L2 sites, compression-binary loss1, Stage2 batch loss1, Stage2 degree3 rounds, and successor-induced setup Stage3 degree2 rounds. Fold-coordinate runs derive each group's `num_polynomials*live_blocks`; terminal appends a response search and its fold group (`:373–418`). These are local ordered sites, not global extraction-tree multiplicities.

`grind_bits_for_loss(loss_factor: u64, challenge_order: ChallengeFieldOrder) -> Result<u8, AkitaError>` finds the least `g<=25` satisfying `2^g*|E| >= 2^128*L` (`transcript_grinding/challenge_order.rs:101–106,238–255`). `ChallengeFieldOrder::from_field(modulus_bits: u32, extension_degree: usize, field_modulus: u128)` retains exact cardinality, with nonzero power-of-two extension degree (`:24–56`). Its internal 2^192 arithmetic cap (`:6–14`) is NOT a 192-bit SIS/security gate. Nonzero PoW uses `nonce_bits=g+7`; response searches instead have 12-bit nonces/4096 attempts (`transcript_grinding.rs:17–20`; `akita-transcript/src/grinding.rs:5–8,31–54`).

### What “query budget” and “work” mean

- Expanded query count counts *logical plan entries*, including fold group root/coordinates. It does not count all nonce candidates, adversarial alternative histories, extractor rewinds, or field-rejection draws. Thus `<u32::MAX` is not an adversarial ROM Q limit.
- `field_sampling_is_certified(canonical_bytes: usize, modulus_bits: u32, query_budget: u64) -> bool` merely checks encoding support and `query_budget <= u32::MAX`; exact field rejection sampling loops until a canonical value (`akita/crates/akita-transcript/src/proof_stream/sampling.rs:9–63`). Descriptor construction passes the constant plan ceiling (`akita/crates/akita-config/src/transcript_binding.rs:38–59`). It does not measure or constrain a malicious oracle algorithm's total queries.
- Planner `ProofCost { payload_bytes, nonce_bytes, nonce_bits, expanded_query_count, work_elements }` (`akita/crates/akita-planner/src/schedule_params.rs:398–487`) uses score `proof_bytes*2^18 + work_elements` (`:503–510`). Direct setup work is a scan model (`:14–28`). This is honest protocol selection/accounting, not collision-search/extraction time.
- `FoldScheduleEstimate` has payload/nonces, setup capacity, first direct setup length, offload count, but no knowledge error or tree/collision resource field (`akita/crates/akita-params/src/schedule.rs:1075–1121`). `PlannerPolicy` has geometry, decomposition, table identities, selection/search policy and optional **host setup-field** budget; no adversary/extractor budget (`akita/crates/akita-schedules/src/runtime.rs:230–256`).

### Metadata consumers

1. A trusted JSON artifact row contains only `schedule: FoldSchedule` (`akita/crates/akita-schedules/src/artifact.rs:34–38`). `ValidatedScheduleCatalog::try_new` audits rows and rejects duplicate keys/row identities (`:139–205`); `ResolvedScheduleRow::try_new` validates executable methods, semantic geometry, dimensions and canonical transition lengths before hashing (`resolve.rs:20–49,82–164`). `audit.rs:71–139,143–197,288–341` checks matrix policy/digest/role/floors, exact widths and accepted-response collision envelopes. This is semantic admission, not authentication of a mathematical security theorem.
2. `resolve_key(&ScheduleLookupKey)` selects an honest-prover row; `resolve_selection(OpeningScheduleSelection)` resolves a proof's digest in that same catalog and rejects missing rows (`artifact.rs:365–406`). Resolution runs no estimator or planner search.
3. Materialization rederives the COMPLETE native grinding plan and compares all three cached nonce/query coordinates (`runtime.rs:850–878`). This authenticates planner-accounting consistency, not extractor resource bounds.
4. `transcript_instance_descriptor<F,Cfg>(setup, opening_batch, selection, schedule, basis) -> Result<(GrindingPlan,Vec<u8>),AkitaError>` binds setup identity, schedule and plan digest (`akita-config/src/transcript_binding.rs:38–79`). Ordinary prover and verifier both call it (`akita-prover/src/protocol/prove/root.rs:32–60`; `akita-verifier/src/fold/verify.rs:68–110`).
5. `GrindingPlanCursor` replays site/multiplicity/nonce targets and checks completion; `GrindingReplay::grind_query` is prover search/verifier checking (`akita-types/src/transcript_grinding/replay.rs:18–87,145–180`). Runtime consumes these concrete nonce/site properties, not a global extracted-tree cost.

## 4. Setup bias must stay separate

For the **current ordinary native setup**, the inspected path is not a modulo-reduced sampler: `PublicMatrixDerivation::Shake256PagedV1` declares exact field sampling in 4096-field pages (`akita/crates/akita-types/src/proof/setup.rs:22–35`). `sample_akita_setup_seed()` is a FIXED reproducible public seed, not fresh entropy (`:219–225`). `derive_public_matrix_prefix<F: Field+CanonicalEncoding>(num_field_elements: usize, id: &AkitaSetupSeed) -> FlatMatrix<F>` calls `F::random` from domain/seed/field/page-separated SHAKE streams (`:227–258,317–345`). Prover setup uses this path (`akita/crates/akita-cpu-backend/src/setup/prover.rs:42–70`).

For fp128, `F::random` invokes exact `sample_uniform_below` (`jolt/crates/jolt-field/src/solinas/fp128.rs:1091–1094`); that helper masks unused bits and rejects `candidate>=modulus` (`solinas/mod.rs:73–100`). **[INFERENCE]** Relative to independent ideal-uniform XOF bytes there is no modular-reduction bias in these coefficient samples. This does not turn a fixed deterministic setup into an unconditional statistically uniform matrix distribution. The XOF/fixed-seed/setup-law assumption and any resulting setup deviation are separate; these schedule/estimator APIs have no explicit setup-advantage/deviation input or composition output. This observation is limited to the current ordinary path, not external installations or legacy frontend sampling.

## 5. Static numerical evidence: do not merge two different controls

### A. Checked-in runnable PCS-only example (Dense fp128, nv14)

`akita/crates/akita-pcs/examples/accumulation_pcs_control.rs:38–85` fixes `Config=fp128::Dense`, `NUM_VARS=14`, resolves a singleton catalog key, and reads the root A dimension. It is expressly PCS-only, without frontend/accumulation/outer aggregation (`:1–31`). Metrics are cumulative wall time, proof bytes and peak RSS (`:167–190`), NOT security or extractor cost. Loader binds the workspace catalog (`examples/support/workspace_schedules.rs:10–24`).

`akita/artifacts/schedule-catalog.tsv:1–2` gives the selected singleton row:

- row digest `d6405b862b45e01f423c0d28c082c2bcaafd7ffd144c0d9a54fb121ae612e974`;
- setup fields 131072; first-direct setup capacity 32768; modeled proof bytes 65073; fold levels 5;
- witness lengths `16384 -> 470976 -> 318976 -> 221184 -> 112128`, then terminal.

Full static row is `akita/artifacts/schedules/fp128_dense.aks:126–134`:

| Stage | A `(d,rank,width)` | A collision certificate |
|---|---|---|
| Root | `(512,1,40)` | Linf 427818804 |
| L1 | `(256,1,128)` | Linf 835380 |
| L2 | `(64,4,512)` | Linf 835380 |
| L3 | `(64,3,512)` | L2² 8796093022208; response cap 286397047 |
| Terminal | `(64,3,256)` | L2² 17592186044416; response cap 549103459 |

Root B and D are each `(d=64,rank=1,width=344,Linf=7)`; root A decomposition is basis-log13/depth10, opening basis-log3/depth43, response depth7. Root/L1/L2 shell `(count_pm1,count_pm2)=(31,10)`; L3/terminal `(31,11)`. This is NOT the uniform128 frozen Main+RAM row below. The static root table cell has rank1 capacity1538 at `(d512,B427818804)` (`generated_sis_table/q128.rs:240`), covering width40. No exact security-bit minimum or full extraction cost is stored in this row/snapshot.

### B. Saved frozen-source ordinary Main+two-RAM ledger

`AKITA_INSTANCE_ACCUMULATION_ORDINARY_CONTROL_LEDGER.txt` is a saved public-shape output record, NOT a newly executed control or an authenticated security certificate. Its body is abbreviated; use its recorded summary as a record, not as a freshly recomputed full manifest.

- Policy: Legacy, uniform128, basis-log5, degrees1/1, one chunk, selective L2 disabled, direct setup (`:1`).
- Main `(nv16,p56)`: A `(d128,rank3,width512,Linf8126340)`; B `(d128,rank1,width4368,Linf31)` (`:2,16`). Each ordered RAM `(nv16,p1)` has 26 original balanced planes, A `(128,3,832,8126340)`, B `(128,1,1248,31)` (`:2–4,14–15`). D segment widths are 416,416,1456 (`:14–16`), giving total2288 by static addition.
- Six nonterminal transitions `65536 -> 2071040 -> 700416 -> 379904 -> 294912 -> 218112 -> 191488`; packing at levels0/1 then ET; shell `(31,0)` (`:8–13`). Terminal A `(128,3,256,Linf4063108)`, response Linf cap16383, GolombRice (`:17`).
- Recorded setup capacity559104; nonce maximum780 bytes; modeled proof/parser-bound bytes141280; terminal response estimate110596 (`:18,179–180`).
- Recorded plan summary: expanded logical queries561, total nonce bits3786 (`:179`). Visible local sites: root claim batch loss58/g6/nonce13; alpha loss255/g8/nonce15; tau0 loss21/g5; tau1 loss5/g3; Stage1 loss5/g3 (`:19–29`); terminal fold group has6 coordinates+root, multiplicity7 (`:178`).
- Rank-floor values can be checked statically without estimating: `(d128,B8126340)` table capacities start `[5,14,23489,...]`, so widths512/832 need rank3; terminal `(d128,B4063108)` starts `[5,16,109366,...]`, so width256 needs rank3. B/D `(d128,B31)` rank1 capacity28621854 exceeds these widths (`akita/crates/akita-params/src/sis/generated_sis_table/q128.rs:73,93,95`). These establish table coverage, not an actual 192-bit or runtime-adjusted advantage claim.

## 6. Narrow missing requirement for Candidate1

Existing reusable inputs are: exact admitted occurrence geometry/norms; immutable schedule/row/table identities; local loss factors and actual native nonce policy; setup sampler implementation. Missing is a **finite end-to-end reduction resource statement** for the selected extractor, tied to that exact schedule and setup law:

1. Total extractor/collision-finder work including complete suffix extraction, branching/rewinds, successor-source coalescence, collision processing and number of possible outputs/attempts—not just honest proof work or the number of matrix occurrences.
2. Declared adversarial oracle budget and the reduction's additional queries, with local grinding applicability conditions and per-site candidate-query counts when those are used. The logical-plan count is insufficient.
3. A justified runtime-conditioned SIS advantage bound for the ACTUAL collision norms/matrices at that full reduction cost, with any multi-instance/collision multiplicity loss explicitly charged; the modeled minimum attack-cost floor alone supplies no such bound.
4. Separately accounted setup deviation/assumption and ROM/algebraic error, combined into a meaningful explicit finite bound. No extra tight128 criterion is required.

This is missing analysis/certificate data, not a request for a speculative code redesign. Current admission and catalog diagnostics cannot by themselves establish that source-first extraction is sound or resolves source-first circularity; those mathematical obligations remain with Main's separate review.
