# Preferred ordinary control: native K/K coefficient packing with k=1,h=1

## Decision: retain the stock root method policy

**Use native `SubringCoefficientPacking { challenge_subring_dimension: 128 }` for the exact compact K/K root, not a new root EvaluationTrace exception.** For k=[E:K]=1 and dA=s=128, h=dA/(k*s)=1 and the packing map is the identity on the A-ring coefficient vector. The ordinary base-field partials derived in `AKITA_INSTANCE_ACCUMULATION_ORIGINAL_PLANE_OPENING.md` can therefore be supplied directly as the existing typed coefficient-packing partials. No algebraic obstacle from delta26/noncanonical RAM planes was found.

This supersedes the earlier contract's §6D recommendation to introduce a root EvaluationTrace admission exception and the ET-specific source seam in §6A/B. The prior artifact remains the evidence for source ownership, exact digit folding, logical-versus-raw query semantics, cross-E handle barriers and compression material. Those barriers still exist. The stock schedule validator can stay unchanged, as can the ordinary verifier's method policy and algebraic kernels. A source-aware frozen-profile planner/materializer and narrow source capability are still required; no complete schedule admission or execution is claimed here.

**Anchor:** original source allocation, original noncanonical A-input digit path, source ID/raw map/decoder, frozen producer contract/profile, literal A/B/F keys and public payload remain fixed. Only query-derived opening material and consuming proof schedule are ordinary K/K. No source edits, builds, tests, linters, formatters or proof runs were performed.

## 1. Exact identity specialization

`akita/crates/akita-params/src/layout/subring_packing_geometry.rs:6-10,30-81` defines

    dA = k*h*s,
    stride = k*h = dA/s,
    partial_width = k*s.

For `(k,dA,s)=(1,128,128)`, this yields `h=1`, `stride=1`, `partial_width=128`; challenge embedding Y -> X is the identity. Do not confuse h=1 with s=1. `try_new` uses the production sparse challenge family for s and audits it (:44-58). The companion planner investigation confirms native s128 at uniform dD128 gives production `(31,0)`, omega31; this is the consuming shell, not a modification of the source profile or accumulation shell.

`akita-types/src/subring_coefficient_packing.rs:14-18,39-126` splits the source query into `[r_pack | r_tail | r_M | r_B]`. Here r_pack is empty and packing_weights=[1], while r_tail contains all seven source coefficient bits:

- Main: packing 0, tail 7, position 9, block 0; source nvars16; 512 live ring positions per polynomial.
- RAM: packing 0, tail 7, position 5, block 4; source nvars16; 512 live ring positions per polynomial.

Every block remains exact: Main B1/M512, RAM B16/M32. The digit axis is not part of the logical query; it is fixed-gadget recomposition.

In general the packing map is

    Pack_r(a)[t] = sum_(u < stride) packing_weight[u] * a[t*stride+u].

With stride1 it is `Pack_r(a)[j]=a[j]`. Therefore, for original supplied rings d[p,b,x,l] and beta32, the typed opening partial is

    e[p,b][j] = sum_(x,l) position_weight[x] * beta^l * d[p,b,x,l][j].

Main uses only l=0; RAM uses all l=0..25, including its legal noncanonical final-plane values. The scalar claim is

    v[p] = sum_(b,j) block_weight[b] * tail_weight[j] * e[p,b][j].

These are precisely the same logical K claims as in the prior ET algebra, but under the **native coefficient-packing packet and transcript semantics**. No FullRing packet or EvaluationTrace handle is retagged.

## 2. Existing typed outputs and production kernels

All paths below are under `akita/crates/akita-cpu-backend/src/`.

### Producing the partials

`opaque/operation_plans.rs:82-158` defines:

- `SubringCoefficientPackingPlan<'_,K>`, with a `PreparedSubringCoefficientPackingPoint<K>` and dA/arity validation;
- `SubringCoefficientPackingPartials<K>::new(geometry, num_live_blocks, coordinates)`, checking exactly B*(k*s) coordinates;
- physical order `[block][extension coordinate][subring coefficient]`, hence `[block][0][j]` here.

The output is **one typed partial object per original polynomial**, not one object per digit plane. Each Main polynomial returns 128 K coordinates; each RAM polynomial returns 16*128=2048. These are query-derived partials, not a new source commitment.

`arithmetic/coefficient_packing.rs:69-105,107-236` already provides

    coefficient_packing_partials_from_position_source::<K,K,P,128>(
        fused_weights,
        source_num_vars,
        position_at,
        coefficient,
    ) -> Vec<K>.

Its callback borrows a position source and reads each coefficient; no SourceCoefficients trait or decoded source table is requested. `FusedPackingWeights::new` (:20-66) multiplies position and packing weights, which reduces to the position weights for stride1. The exact same helper is present in pinned upstream `/tmp/akita-extension-audit-c6qh69gy/akita/83574331d4e5/crates/akita-cpu-backend/src/arithmetic/coefficient_packing.rs:69-110` (same read snapshot content as current).

`source/dense` is not necessary: the existing dense kernel at `sources/dense/kernels.rs:178-235` is just one callback implementation. Main already has a sparse one-hot implementation at `sources/onehot/ops.rs:319-481`, updating the coefficient slot for each real hot index and returning the same typed partials without a dense table. Reuse that original kernel for Main.

### Computing scalar openings and retaining their source

Current `opaque/owned.rs:168-229` and pinned upstream `opaque/owned.rs:184-245` already implement the coefficient-packing branch:

1. `SubringCoefficientPackingGeometry::try_new(E::DEGREE,D,s)`;
2. exact live ring count/arity/group-block consistency;
3. `PreparedSubringCoefficientPackingPoint::new`;
4. borrowed `RootOpeningSource::opening_batch`;
5. `SubringCoefficientPackingBatchKernel::coefficient_packing_partials_batch`;
6. `coefficient_packing_scalar_opening` with point block/tail weights;
7. `prepared_opening::coefficient_packing` retaining the original source/binding.

`arithmetic/coefficient_packing_fold.rs:245-326` performs scalar extraction directly from the typed coordinates. `opaque/prepared_opening.rs:29-48` constructs the correctly tagged `CpuPreparedOpeningHandle::coefficient_packing`. Reuse this exact branch through a source-specific native128 capability; do not route through the ET helper just because the numeric base partial arrays match.

## 3. Fold response and ordinary packing consistency

Reuse `PackedRamSource::decompose_fold_batch`/`fold_chunk` unchanged (`sources/packed_ram.rs:134-162,321-374`) over the original plane allocation. For the native s128 shell, canonical subring challenges already live in R=K[X]/(X^128+1), so no nontrivial embedding is needed. For each position/digit the folded response remains

    z[x,l] = sum_(p,b) c[p,b] * d[p,b,x,l] in R.

[INFERENCE, exact algebra] Let `e[p,b]` be the typed partial rings above. Then

    sum_(p,b) c[p,b] * e[p,b]
      = sum_(x,l) position_weight[x] * beta^l * z[x,l]  (mod X^128+1).

This is the k1h1 packing consistency equation. A/B consistency remains `A z = sum c*t` with the same original A-input digits; source canonicality is not needed for either identity.

The existing packing path also provides the *polynomial* quotient needed by ordinary ring switching:

    sum_(p,b) c[p,b](X) e[p,b](X)
      - sum_(x,l) position_weight[x] beta^l z[x,l](X)
      = (X^128+1) Q_pack(X),

where the representative on the right of the subtraction is the degree<128 reduced response. Since its degree is <128, Q_pack is the positive high half of the first ordinary product. It is generally nonzero even though the packing map is identity.

Evidence and typed continuation:

- `opaque/witness_build/coefficient_packing.rs:19-57`: `fold_coefficient_packing_group` checks claim/block/geometry consistency, flattens typed partials in challenge order, and calls the original fold helper.
- `arithmetic/coefficient_packing_fold.rs:414-443,466-599`: `CoefficientPackingFoldProduct` carries both reduced coordinates and positive high-half quotient, each length k*s and polynomial modulus Y^s+1. `fold_coefficient_packing_partials` simultaneously accumulates negacyclic wraps and Q_pack. For k1h1 this is exactly the R128 product above.
- `opaque/witness_build/prepared_openings.rs:145-171`: a correctly tagged packing opening and its canonical subring challenges produce `RingRelationGroupWitness::from_coefficient_packing_parts`, together with the same accepted fold handle and original inner relation material.
- `opaque/witness_build/relation_quotient.rs:303-361`: accepted-fold A quotients use the existing kernels; packing's consistency quotient uses the typed product's high-half coordinates. Do not drop it on the argument that the map is identity.
- `opaque/sumcheck/relation_range_image/coefficient_packing_terms.rs:75-137`: direct scalar weights use tail weights; Z consistency uses packing weights and alpha powers plus `witness_gadget`/`fold_gadget`. All 26 source gadget powers still participate. This is a native coefficient-packing relation, not an ET-specific approximation.

The normal D-input construction is also reusable: `materialize_coefficient_packing_d_input` (`opaque/witness_build/coefficient_packing.rs:96-218`) checks the packing geometry and decomposes the **new query partials** into scheduled D-input digits. That decomposition is required proof material. It does not redecompose the original supplied RAM source planes or manufacture an alternative A/B/F source path.

## 4. What SourceCoefficients actually does here

**Nothing at runtime in this k1 packing opening branch.** It is an overstrong ownership/import trait bound inherited from the combined ordinary+tensor/EOR capability object.

- `opaque/owned.rs:132-152` requires P:SourceCoefficients for the entire LegacyPrivateSource implementation, but its packing `opening` branch (:168-229) calls only RootPolyShape/RootPolyMeta/RootOpeningSource and SubringCoefficientPackingBatchKernel.
- Calls to `poly.source_coefficients()` occur in `extension` (:261-280) and `begin_eor` (:290-308), not in coefficient-packing opening preparation.
- `akita-params/src/layout/params/precommitted.rs:58-62` selects EOR only for EvaluationTrace with extension_degree>1, not coefficient packing and not degree-one ET.
- RuntimeRootProvePoly and RuntimeCoefficientPackingBackendFor also blanket-require many dimensions (`opaque/capabilities.rs:58-143`), while this source is intentionally native128.

Consequently, implementing SourceCoefficients by allocating a decoded DensePoly is neither necessary nor the minimal reuse seam. It would also be the wrong place to confer commitment authority.

**Permissible distinction:** a callback may compute one logical coefficient `a[j]=sum_l32^l d[l,j]` on demand, or use a transient 128-coefficient ring scratch, solely to evaluate the authenticated source query. That arithmetic decoding does not change source storage, source identity or its noncanonical digit witness. Better, the shared callback can borrow the original 26*128 signed-digit slice and use Horner26 for the requested coefficient, allocating no decoded source table. The resulting B*128 query partial table is legitimate and required.

**Forbidden substitution:** decode the whole source, construct a new canonical polynomial/source handle, let its commitment/fold path redecompose it, and treat that as the old noncanonical source. Also forbidden is using query partials as an original source without the ordinary protocol's explicit D/recursive witness commitment and constraints. “Canonical partial coordinates” in these type names refers to their field-coordinate layout, not to canonicalizing the producer's digit path.

## 5. Narrow implementation contract

Proposed changes only; no implementation was made.

1. **RAM packing kernel:** add the specific `SubringCoefficientPackingBatchKernel<&[&PackedRamSource], K,K,128>` implementation in `sources/packed_ram.rs`. Authenticate the complete original CommitInnerPlan and requested arity/live positions/block geometry; for the requested control require geometry(1,128,128). Build FusedPackingWeights and use the original `coefficient_packing_partials_from_position_source`. A per-position borrowed packed slice of length26*128 supplies `coefficient(j)=sum_l32^l d[l,j]`; finish through `SubringCoefficientPackingPartials::new`. Do not implement the all-dimension CpuSource bundle. Main reuses its existing one-hot packing kernel.
2. **Owned packing capability:** expose only the native128 coefficient-packing operation plus ordinary fold probing from the same FullRingOwned original allocation, independent of whether its originating handle's E type is E2. Use the existing `OwnedPolynomials::opening` packing branch's construction sequence. Split its opening-only capability from tensor/EOR if necessary, instead of advertising unsupported SourceCoefficients/all-dimension operations. Existing full-ring legacy/probe rejection remains valid for the full-ring entry; do not globally remove its method guard.
3. **Cross-E receiving authority:** retain original Arc storage, source ID/raw map/decoder, profile, producer_contract and public payload while minting an ordinary K/K receiving-owner handle only after original setup/material authentication. This remains missing in current import_commitment's same-E signature. Prefer a narrow owner-preserving adapter over rebuilding source storage.
4. **Material:** derive an additional QuotientLift compression material view from retained original F stage bytes using existing compression product kernels, as in the primary contract §5. Preserve the original exact-row material and all common rows/path bytes. The identity packing specialization does not remove ordinary compression quotient obligations.
5. **Native schedule:** use the existing coefficient-packing root purpose/method (s128) and production shell128, exact compact precommitted profiles and original producer envelopes. Keep native level1/suffix choices. No root EvaluationTrace purpose or `validate_level_opening_execution` exception is needed. The frozen-profile planner materializer still must construct/admit a complete ordinary schedule; this arithmetic reuse contract does not certify one.
6. **Normal proof:** enter `prepared_opening::coefficient_packing`, native fold challenge/acceptance, original D material, Q_pack/A/B/F/H relation checks, ordinary recursive suffix and terminal verifier. The output is a complete ordinary K/K proof on the original logical frontend claims, not a raw-plane proof or a shortened control.

## 6. Remaining blockers and acceptance

**Resolved algebraic point:** k1h1 packing and base partial evaluation are exactly compatible with supplied delta26 planes. The same raw-plane fold response satisfies native packing consistency; the original typed kernels already calculate the nonzero quotient and scalar relation. Root ET policy weakening is avoidable.

**Still unavailable as a current ready API:** RAM packing kernel/capability; owner-preserving E2->K/K projection; ordinary compression quotient material; exact frozen-root planner materializer and full schedule admission. Original `FullRingOwned::legacy=None`, full-ring import's E2 validation and full-ring probe guard still prevent direct reuse of the unmodified handles.

**Still not the same mathematical query:** arbitrary full-ring raw digit/E2 queries. k1h1 packing opens the fixed base32-decoded K polynomial at its original16-coordinate point. Noncanonical paths that share a decoded value need not share raw-plane evaluations; no type adapter can erase that distinction.

Eventual verification should compare native typed packing partials/scalars and raw-plane z against independent streamed formulas for canonical and noncanonical legal inputs; ensure Q_pack is retained; mutate source identity/profile/mode/path material to confirm rejection; and run the complete ordinary prover/verifier under the separately admitted exact compact profile. None of these checks was run in this research pass.
