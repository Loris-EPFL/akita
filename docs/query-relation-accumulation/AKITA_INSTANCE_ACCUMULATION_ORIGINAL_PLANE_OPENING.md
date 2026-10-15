# Ordinary K/K opening of the exact compact original sources

**Preferred-path correction:** see [the native k1,h1 coefficient-packing addendum](AKITA_INSTANCE_ACCUMULATION_NATIVE_PACKING_CONTROL.md). It establishes a stock-supported root alternative with the same base partial arithmetic and original26-plane fold response. The ET root-admission exception proposed in §6D below is **not necessary and is superseded** by that route; source ownership/cross-E/material findings below remain applicable.

## Decision and fixed anchor

**Not already callable on the current full-ring handles. Algebraically reusable for the ordinary logical K-polynomial claims, without changing any supplied A-input digits.** The blockers are source capability/ownership, field-typed handles, setup identity, retained relation-material mode, and an explicit root opening-method admission policy. They are not an inability of EvaluationTrace to handle 26 source planes.

The anchor is the original source allocation and its physical signed digit path, the exact frozen producer profile/contract, original source ID and decoder, literal A/B/F matrices, original public commitment, and retained B/F path. Only the consuming ordinary opening procedure, its query-derived partials, and its separately admitted proof schedule may differ. Equality of decoded K tables is **not** source identity. No application source was edited; no build/test/lint/formatter or proof execution was run. This is a source inspection and integration contract, not an admission or performance result.

The target remains:

| Source | nvars | P | dA | B | M | A digits | A rank | B width/rank/dimension |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| Main | 16 | 56 | 128 | 1 | 512 | 1 | 3 | 4368 / 1 / 128 |
| each RAM | 16 | 1 | 128 | 16 | 32 | 26 | 3 | 1248 / 1 / 128 |

Main is BooleanUnitOneHot chunk 1. RAM is the original supplied balanced base-32 alphabet [-16,15] in every one of its 26 planes, including legal noncanonical paths. A's frozen rounded radius is 8126340; B uses base 32/depth 26. This document does not select or certify the complete ordinary continuation. The consuming shell may differ from accumulation only as separately admitted; source geometry does not change with the shell.

## 1. What the current code actually owns

Paths below are relative to the working root, under `akita/crates/`, unless explicitly marked upstream.

1. `akita-cpu-backend/src/opaque/owned.rs:20-65,84-127`: SourceHandle owns `Arc<dyn PrivateSource<F,E>>`; CommitmentHandle retains this source plus metadata, full GroupCommitPhaseParams, producer_contract, public commitment and PortableCommitmentHandle. `legacy()` and `full_ring()` are separate optional capabilities. Both storage and handles are parameterized by E, although the committed coefficients are F.
2. `opaque/full_ring.rs:14-30,107-162`: FullRingOwned retains the original `Vec<P>`, source ID and raw view. Its `commitment_sources()` borrows these same polynomials; `legacy()` returns **None**, and `full_ring()` returns Some(self).
3. `opaque/full_ring.rs:169-193`: FullRingOwned::probe rejects any method other than full-ring scalar extension and then invokes the existing native-d128 FoldResponseKernel with the installed source's A role. This guard must not simply be deleted: an ordinary capability must select/check its own admitted method and the same literal A matrix.
4. `opaque/full_ring.rs:202-245`: import_source_full_ring retains the input vector without dense conversion, but calls validate_full_ring_fields. `akita-types/src/full_ring/mod.rs:25-51` requires the q128 base field and extension degree 2/nonresidue-five coordinates. Thus `CpuBackend<K,K>::import_source_full_ring(...)` is rejected by the current API even though its source digits are K-valued.
5. `sources/packed_ram.rs:26-44,62-117,165-177,222-249`: PackedRamSource packs the supplied signed digits directly, preserves noncanonical paths, authenticates the exact immutable CommitInnerPlan, enforces zero physical padding, and exposes only PredecomposedDigits for commitment. The plan includes nA, not just d/M/delta. It is one polynomial, not 26 separately committed polynomials.
6. `sources/packed_ram.rs:253-273,294-319`: raw reads and borrowed RootOpeningSource views exist. But `OpeningBatchKernel::evaluate_and_fold_batch` unconditionally returns `original packed RAM sources require native full-ring openings, not legacy L/F`. This is a deliberately unsupported operation, not an algebraic counterexample.
7. `sources/packed_ram.rs:134-162,321-374`: the existing decompose_fold_batch already folds **the exact packed planes**, with Sparse and SparseChunked paths. M*26 is flattened only as a kernel view, and the output still means [position][source digit]. This is the fold implementation to reuse.

### Existing borrowed seams

- `sources/full_ring.rs:14-25`: FullRingSource's visitor and indexed accessor expose exact physical A-input digits in `[block][position][digit][coefficient]` order. The visitor requires increasing unique addresses and may omit only zero cells. The PredecomposedDigits branch at :57-83 directly visits the supplied packed/unpacked bytes, not a reconstruction.
- `opaque/full_ring.rs:683-749`: CpuFullRingSourceReader and `full_ring_source_reader` borrow a committed source after owner, exact physical-length, d/P/B/M/delta and raw-view checks. The reader's coefficients are physical original digits; they are **not decoded logical polynomial coefficients**. The reader is a sufficient slow/reference read seam for ordinary base-field partials, not itself ordinary PCS admission. It is also not a source-identity-changing import API.
- `opaque/full_ring.rs:357-366` provides the session/context-scoped `with_full_ring_source` entry. Its native admission is not a replacement for ordinary proof admission.

## 2. Original/current ordinary capabilities: what can be reused

`opaque/source.rs:16-129` is identical in the inspected current and pinned upstream copies (read snapshots both identify the same content). It already separates query preparation, borrowed batch evaluation, and scalar extraction:

- `prepare_and_evaluate_opening_group` calls `prepare_opening_point` and `evaluate_claims_at_prepared_point`.
- The latter constructs OpeningFoldPlan::Base for K/K and calls `RootOpeningSource::opening_batch` + `OpeningBatchKernel::evaluate_and_fold_batch`.
- It requires one folded ring per polynomial and one partial ring per live block, checking those lengths.
- `scalar_opening_from_folded_ring` has a degree-one branch using a direct coefficient inner product. It does not need any ring-subfield encoding or EOR when E=K.

Current `opaque/owned.rs:154-259` retains the original ordinary opening path: prepare the point/partials; derive scalar openings; construct `prepared_opening::evaluation_trace` retaining the source and binding. Dense `sources/dense/ops.rs:59-79,103-113` implements the per-block position contraction, followed by `fused_evaluate_and_fold_base`. Main can use its existing one-hot kernel over its original allocation, or the exact digit visitor with delta=1; it must not be densified for ownership convenience.

The ordinary import bound is substantially stronger than the computation requires:

- `opaque/owned.rs:378-445`: CpuSource -> SourceImport requires SourceCoefficients, RuntimeRootProvePoly and runtime opening/coefficient-packing backend bundles.
- `opaque/capabilities.rs:58-143`: those bundles require RootOpeningSource at **64,128,256,512,1024,2048**, not just native128.
- `sources/poly.rs:13-19,155-189`: SourceCoefficients is a canonical-table capability used by tensor reductions, distinct from RootOpeningSource and RootPolynomialEvaluator. Existing Dense/OneHot evaluators use their own fold kernels (:192-275). PackedRamSource has only native128 opening views and no SourceCoefficients/RootPolynomialEvaluator implementation in its source file.
- `opaque/owned.rs:261-280` actually calls source_coefficients in the tensor extension path, not in degree-one EvaluationTrace.
- `akita-params/src/layout/params/precommitted.rs:58-62`: EvaluationTrace requires extension-opening reduction only when extension_degree > 1.

Therefore: **do not implement all-ring/tensor capabilities merely to make PackedRamSource satisfy CpuSource**; do not implement SourceCoefficients by creating a decoded DensePoly. A native128 ordinary read/fold capability is sufficient. If a frontend independently needs a scalar-evaluation hook, it can compute the same streamed contraction below without acquiring commitment authority or a dense table.

## 3. Algebraic contract for delta26

Let R=K[X]/(X^128+1), and write the exact supplied plane rings as d[p,b,x,l] in R. For RAM l=0..25 and beta=32; for Main there is only l=0. Define the logical K ring value only as notation:

    a[p,b,x] = sum_l beta^l d[p,b,x,l].

There is no requirement to allocate the a table. For the ordinary 16-coordinate logical query, use the existing prepared point's coefficient weights u[j], position weights v[x] and live-block weights w[b]. Compute directly from the original planes:

    partial[p,b][j] = sum_(x,l) v[x] beta^l d[p,b,x,l][j]
    folded[p][j]    = sum_b w[b] partial[p,b][j]
    claim[p]        = sum_j u[j] folded[p][j].

These are precisely the outputs of the ordinary OpeningFoldPlan::Base contract. Storage required beyond query weights is only the query-derived P*B rings plus P folded rings, not a 2^16 decoded source or a 26-plane replacement. Main uses 7 coefficient + 9 position + 0 block coordinates; RAM uses 7 + 5 + 4. There is no digit-coordinate added to the ordinary frontend's nvars16 query.

[INFERENCE, algebraic derivation] For ordinary sparse ring challenges s[p,b], form with the existing packed digit fold:

    z[x,l] = sum_(p,b) s[p,b] * d[p,b,x,l] in R.

Then linearity gives both ordinary relations:

    A z = sum_(p,b) s[p,b] * t[p,b]
    sum_(p,b) s[p,b] * partial[p,b]
       = sum_(x,l) v[x] beta^l z[x,l].

The second equation uses v[x] as a base-field scalar, so it commutes with ring multiplication. Neither equation assumes canonical digits or injectivity of beta-recomposition. Two legal paths decoding to the same K polynomial may correctly give identical scalar openings under different commitments. Their commitments/fold responses still refer to the distinct original plane paths.

Evidence that the ordinary relation code has exactly this variable-depth structure:

- `opaque/relation_weights/compiler.rs:397-401,435-439,508-512` derives depth_witness from group.num_digits_inner and builds its full gadget row at group.log_basis_inner.
- `opaque/relation_weights/sinks.rs:129-156` loops every witness digit, uses setup column `position*depth_witness+digit`, and combines witness/fold gadget powers in Z. No delta1/13 restriction or canonical top-plane test occurs here.
- `opaque/fold.rs:307-345` reconstructs each folded position with gadget_row_scalars(depth_commit, log_basis), not a fixed digit count. `a_relation_from_fold` (:357-406) reuses accepted-fold A quotient kernels. Its consistency high-half is zero when the multiplier point is constant (the ordinary base-field case); nonconstant subfield weights have their own existing path.
- `akita-types/src/trace_weight/evaluation_trace.rs:205-259` uses the full 128 packed coefficient weights directly for E::DEGREE==1 and reconstructs query-derived opening partials with num_digits_open/log_basis_open. **Opening-partial digit depth is not source digit depth.** The source delta26 lives in the Z/A consistency relation; the EvaluationTrace scalar relation concerns the query-derived e partials.
- `arithmetic/ring_switch.rs:113-141,143-233` is width/row-driven over e_hat/t_hat/z and existing matrix-product/quotient kernels, not over a canonical source table.

No 26->32 plane padding is committed or folded. The six dummy cells in a full-ring raw Boolean digit grid do not become source A columns. Ordinary A width remains M*26=832 for RAM.

### Important mathematical boundary: logical versus raw queries

EvaluationTrace above opens the ordinary decoded K polynomial, with **fixed** digit weights (1,32,...,32^25). It does not directly open an arbitrary raw 26-plane multilinear query with independent digit-coordinate weights, or an E2 query with two coordinate planes. `akita-types/src/full_ring/query.rs:180-209` explicitly distinguishes raw coordinates from the ordinary decoder; the full-ring source_partials at `opaque/full_ring.rs:34-104` instead contracts arbitrary E2 position/digit weights and returns two K planes per block.

A raw query need not factor through the logical decoder: a difference between two noncanonical digit paths can have zero beta-recomposition and nonzero raw evaluation. If the requested ordinary control were to reproduce arbitrary raw-plane claims rather than the original K frontend claims, fixed-gadget EvaluationTrace would be insufficient. It would need an authenticated query/decoder-adjoint relation, not merely a source capability adapter. For the stated K frontend / ordinary K/K control, this is a distinction to preserve, not a missing property of delta26 logical openings. Arbitrary frontend affine aliases/permutations likewise need their installed decoder mapping; do not silently treat them as the direct positional decoder.

## 4. Current import is already source-preserving: do not regress

Pinned upstream at `/tmp/akita-extension-audit-c6qh69gy/akita/83574331d4e5/crates/akita-cpu-backend/src/opaque/owned_commit.rs:32-73` imports by calling `committed.source.dense_polynomials()` and then import_source. Pinned `opaque/owned.rs:159-168` implements that using source_coefficients -> DensePoly::from_field_evals. This is **not** an acceptable path for supplied noncanonical A planes.

Current `opaque/owned_commit.rs:20-99` fixes this:

- borrow `committed.source.commitment_sources()` from the existing immutable allocation;
- validate source layout and, for native handles, the installed source ID/raw view;
- recompute with the receiving setup from the same CommitmentSource representations;
- require exact public commitment and retained-material equality;
- retain `committed.source.clone()` and the same producer_contract/parameters.

This is authentication recomputation, not substitution of a decoded source. Its method signature is still `foreign: &CommitmentHandle<F,E>`: it does **not** transfer an E2-typed handle into a K/K backend. The native branch also requires a native installed setup and uses `for_full_ring_root`, so it does not derive ordinary quotient material. Current import is a building block and invariant model, not an already complete cross-field ordinary adapter.

Fresh shared-prefix provisioning is a prerequisite when expecting ordinary shared-setup kernels to match native role matrices. It is not valid to feed an old independently keyed FullRing commitment into the ordinary default matrix and assume equal dimensions suffice. Preserve each source ID/mode/raw-view identity and validate literal per-role matrices against the installed prefix derivation. See `AKITA_INSTANCE_ACCUMULATION_SHARED_SETUP_REVIEW.md`, especially its decision and coefficient-address invariant. This contract does not modify inner/outer key independence or source installation.

## 5. A separate hard API barrier: retained compression material

It is insufficient to add an ordinary partial-evaluation kernel and return Some from FullRingOwned::legacy.

- `commitment/plan.rs:159-183`: ordinary `for_root` selects QuotientLift, while `for_full_ring_root` selects ExactCoefficientRows. The full-ring method explicitly says payload arithmetic is unchanged but retains inputs rather than quotient images.
- `akita-params/src/schedule.rs:598-614`: the current ordinary root must be compressed and QuotientLift. Choosing ReducedEvaluation solely to avoid missing material is rejected and is not the complete ordinary control.
- `opaque/admission.rs:138-238`: validate_commitment checks owner, exact profile/public statement and schedule group, then builds material through `CommitmentExecutionPlan::for_relation_group` and `CpuCommitmentMaterialHandle::from_state`.
- `opaque/witness_build/commitment_material.rs:86-122`: from_state validates mode-specific compression material before admitting it.
- `commitment/portable.rs:235-275`: QuotientLift requires one correctly shaped quotient per compression map; ReducedEvaluation/ExactCoefficientRows require no such quotients. Thus full-ring retained state cannot be re-labelled ordinary.

[INFERENCE, minimal derivation path] Derive an **additional ordinary-mode material view**, preserving the original retained source and original compression bytes. For each retained PackedNegativeBinary stage, use expand_rows::<D> and the existing `compression_rows_products` kernel under that stage's actual installed F matrix; recover quotient rows using the same `(cyclic-negacyclic)/2` formula as `opaque/compression.rs:104-124,219-271`. Validate the map shape, original stage bytes, chained negacyclic outputs and final public payload. Existing `outer_compression_witness` reconstructs checked PackedNegativeBinary objects from retained bytes (`portable.rs:169-187`). No A source decoding/recommit is needed to create these F quotients.

Do **not** regenerate arbitrary F stage paths by decoding and passing values through PackedNegativeBinary::from_coefficients. The current chain executor does that when it *originally produces* a commitment (`opaque/compression.rs:188-206`); material adaptation should instead consume the already retained stage bytes. Likewise preserve original B/inner-row material and its deterministic producer encoding; this adapter does not authorize manufacturing an alternative B/F path for a foreign/noncanonical proof object.

Native ExactCoefficientRows material and ordinary QuotientLift material remain separate views. Do not weaken the current mode-equality validation or overwrite one with the other. Any cross-mode import cannot use equality of the entire PortableCommitmentHandle as its sole criterion: the added quotients intentionally differ, while the common original rows/stage bytes/public payload must be equal and the new quotients independently checked.

## 6. Minimal integration design (proposed, not present code)

The design is one native128 ordinary capability adapter, not a replacement prover, a second storage format or a new all-dimension source hierarchy.

### A. Borrow from the existing owner

Add a narrowly scoped ordinary-K projection/import entry on `CpuBackend<K,K>` accepting a reference to the authenticated producer's `CommitmentHandle<K,EOrigin>` and retaining its underlying Arc in an ordinary wrapper. The wrapper must preserve original source ID, raw view, profile, producer contract, public commitment and original storage lifetime. It must mint a receiving-owner handle/binding only after the setup/material checks above; a reader alone cannot mint proof authority.

Since the current PrivateSource vtable is E-typed, supply only the missing cross-field operations from the original owner (native128 base partials and native128 ordinary fold probe) rather than converting its data. Put those operations next to FullRingOwned in `opaque/full_ring.rs` so they still borrow the original `polynomials`. Their implementation dispatches to existing source kernels at d128 and receiving `CpuBackend<K,K>`. Their public semantics must be explicitly ordinary; the existing full-ring probe guard remains intact.

For ordinary opening dispatch, introduce a narrow EvaluationTrace capability in `opaque/owned.rs`/`opaque/openings.rs` (or equivalently split the current ordinary-opening method from the tensor/EOR part of LegacyPrivateSource). The current legacy path can delegate its EvaluationTrace branch to the same helper. This avoids claiming tensor/EOR/all-dimension capability for PackedRamSource just to enter `prepare_openings`. Do not change FullRingOwned::legacy to Some unless every operation advertised by that trait genuinely has its promised semantics.

### B. Implement only the missing native base contraction

At `sources/packed_ram.rs::OpeningBatchKernel::evaluate_and_fold_batch`, support the validated Base plan by streaming its original packed digits into the partial formula in §3; validate M/B/delta/base against its frozen plan. Preserve the error for unsupported subfield semantics unless separately implemented. Reuse `fused_evaluate_and_fold_base` and the existing `prepare_and_evaluate_opening_group::<K,K,_,_,128>`, `scalar_opening_from_folded_ring`, and `prepared_opening::evaluation_trace` pipeline. Main delegates to its existing one-hot kernel. The raw-reader formula is a reference oracle and fallback design seam, not an excuse to add an avoidable dense source allocation or field-to-signed conversion in the fast path.

The ordinary probe forwards to `FoldResponseKernel::probe` with the original source's native128 OpeningBatchView and the ordinary admitted plan. PackedRamSource::decompose_fold_batch/fold_chunk and Main's original one-hot fold kernel remain the source of z. Do not call FullRingOwned::probe with an ordinary method and do not delete its guard. Check exact producer geometry first; shell choice does not alter the source plan.

### C. Attach the complete ordinary relation material

Add a mode-specific material-preparation helper near current `import_commitment`/`CpuCommitmentMaterialHandle::from_state` that uses the retained source rows/F bytes and derives missing ordinary compression quotients as in §5. Leave the original FullRing handle/material unchanged. Validate original public payload, literal A/B/F projection and all shared base material. Use the ordinary receiving backend's normal scope/commitment-ID lineage checks and retain exact producer_contract.

Where import authentication recomputation is required, run the existing CommitmentExecutor over the same original `CommitmentSource` representations and the exact original profile. That is not permission to mint a different public commitment, use a different setup, convert the source, or hide work from control accounting. Material derivation should avoid recomputing A when an authenticated retained image suffices.

### D. Re-enter the original ordinary prover

First admit a narrowly scoped K/K root EvaluationTrace schedule; the current validator does not allow it. `akita-params/src/schedule.rs::validate_level_opening_execution`, :925-939, sets `packing_required = absolute_level <= 1` without considering extension_degree, and rejects a root EvaluationTrace schedule even for K/K. The full-ring branch at :897-923 does not authorize an ordinary exception. This is a schedule-policy barrier in addition to the source/material barriers above. The change must allow the explicitly selected degree-one **root** trace control while retaining native suffix method choices and the every-group/source-encoding/matrix/shell validation that follows. It must not globally enable arbitrary root trace profiles or force the ordinary suffix into trace. Planner generation also needs an explicit root trace FoldOnly candidate purpose; the companion planner artifact owns that design and complete admission.

After that narrow shared admission change, run normal ordinary ProofAdmission, `prepare_openings`, accepted-fold probe, ordinary ring-switch witness construction, compression range/binary constraints, EvaluationTrace and recursive/terminal continuation. Reuse the existing ordinary verifier's algebraic checks; its shared schedule validator necessarily participates in the new root admission, so an entirely unmodified current verifier entrypoint is **not** claimed. No FullRingScalarExtension packet is re-tagged EvaluationTrace: `akita-prover/src/protocol/ring_switch.rs:151-190` expressly rejects that, and the rejection is correct because the partial and query semantics differ.

The complete schedule, all its source envelopes, ordinary shell capacity, root/suffix quotient/reduced modes, setup-prefix maximum, transcript/grinding plan and terminal execution must receive their own admission. Nothing in this source-reuse argument supplies that certificate. Preserve OriginalBalancedDigitPlanes: `akita-params/src/sis/committed_source.rs:54-58,130-141,183-207,306-318` freezes base/depth, requires full-field declaration and charges the entire alphabet in **every** plane. Do not replace it with canonical BalancedSignedDigit or discount the final plane based on decoded reach.

## 7. Acceptance invariants for the eventual implementation

These are checks to run after implementation approval, not checks executed in this research pass:

1. Exact handles retain the same source allocation, source ID/raw map and compact producer profile; no import via dense_polynomials/SourceCoefficients and no decoded source re-commitment occurs.
2. Ordinary claims equal an independently streamed original-digit contraction for both canonical and deliberately noncanonical RAM inputs, including two paths with equal decoded K coefficients but different raw digits. z uses each actual path, not the common decoded table.
3. Main's chunk1 one-hot semantics and RAM's full alphabet including plane25 survive admission. No d256/M128 Main or d512/M8/delta13 RAM substitution.
4. Native and ordinary commitment public payload and common A/B/F retained path agree under the same fresh installed matrices. Quotient rows are derived from retained F stage bytes and validate the complete ordinary compression relation; deliberately mismatched stage/matrix/source-ID/mode/profile inputs are rejected.
5. All prepared openings/fold probes stay bound to the correct source handle, operation, receiving owner, ordered group and full ordinary schedule. E2/K type crossing conveys base-source authority only, not a reinterpretation of E2 points or partials.
6. Same 16-variable frontend claims are proven end-to-end by the complete ordinary K/K PCS and verified by its ordinary verifier. Arbitrary raw/E2 claims are not silently substituted. Keep application scalar claims, frontend decoder semantics and all ordinary proof obligations identical across the comparison.

**Bottom line:** native source access and native26-plane folding are already present; the ordinary algebra and relation kernels already express the required fixed-gadget logical evaluation. What is absent is the narrow borrowed base-opening capability, cross-E owner-preserving handle projection, authenticated QuotientLift material view, and explicit degree-one root EvaluationTrace planner/runtime admission. Implementing those seams is a reuse cutover; claiming the existing unmodified full-ring handles already run through ordinary EvaluationTrace would be false.
