# Candidate1: generalized frozen-roster root-row contract

## Status and conclusion

This is a read-only, source-backed planning derivation. No repository edits, Git operations, builds, tests, formatters, estimator runs, or proof/control runs were performed. Numbers below are literal existing-profile/table entries or explicitly identified arithmetic deductions—not newly selected or validated Candidate1 schedule rows.

**Preserved anchor:** every original source handle, producer contract, logical profile, literal A-input plane tuple, and original A/B/F commitment path remains unchanged. Only consuming query-view/public weights, root response parameters, shared D/H and a newly calculated complete suffix are planned. The statement is the base K-valued continued relation, including original Main predicates and signed original RAM planes—not unrestricted RV64/SDK semantics. Initially K=E=Prime128OffsetA7F7, q=2^128−2^32+22537 (`jolt/crates/jolt-field/src/solinas/mod.rs:151–156`). The gate is the existing Quantum128BitADPS16 modeled table floor, not an additional tight end-to-end 2^-128 target.

**Closed mathematical result:** H separate original Main56 handles and N_RAM unique original RAM1/depth26/logical65536 handles have a profile-preserving root geometry at H=1,2,3,256 under base32/native128/single-chunk policy. Endpoints number56H+N_RAM, but shell coordinates number56H+16N_RAM. At H256/N_RAM257, shared native128 D width is479648, below the existing rank-one cutoff28621854. Original A/B widths do not increase with H; each separate original F path and shared root H remain within current compression capacities. Thus this width does **not** intrinsically violate a frozen profile.

**Not closed by that result:** no view-aware full Candidate1 row has been selected here, even at H1. The present frozen entry rejects earlier Main handles; ordinary native claims/semantics lack registered raw21 dispatch; and per-group honest response sizing is not a joint-nonce success certificate. A conservative all-valid root response depth3 fits the existing frozen A for both classes without rekeying. The executable planner must materialize that option or certify its tail-sized joint alternative, calculate exact root witness geometry, close every suffix edge and actual terminal, and install/check the complete row and separately owned certificate/resource ledger.

Relevant root constraints/design inputs, not premises proving executable rows: [AKITA_RELATION_ACCUMULATION_PROTOCOL.md](AKITA_RELATION_ACCUMULATION_PROTOCOL.md), [AKITA_RELATION_ACCUMULATION_SOURCE_FIRST_BOUNDARY.md](AKITA_RELATION_ACCUMULATION_SOURCE_FIRST_BOUNDARY.md), [AKITA_RELATION_ACCUMULATION_SOURCE_CERTIFICATES.md](AKITA_RELATION_ACCUMULATION_SOURCE_CERTIFICATES.md), [AKITA_RELATION_ACCUMULATION_SIS_RUNTIME_ADMISSION.md](AKITA_RELATION_ACCUMULATION_SIS_RUNTIME_ADMISSION.md), [AKITA_RELATION_ACCUMULATION_JOINT_SETUP_CERTIFICATE.md](AKITA_RELATION_ACCUMULATION_JOINT_SETUP_CERTIFICATE.md), [AKITA_RELATION_ACCUMULATION_ORIGINAL_PLANE_DESIGN.md](AKITA_RELATION_ACCUMULATION_ORIGINAL_PLANE_DESIGN.md), [AKITA_RELATION_ACCUMULATION_GLOBAL_ORIGINAL_RANGE_REVIEW.md](AKITA_RELATION_ACCUMULATION_GLOBAL_ORIGINAL_RANGE_REVIEW.md). Evidence below inspects actual upstream source. No locally parsed, unauthenticated source-paper text is used as an implementation or numerical certificate.

## 1. Minimal roster and query-view contract

### 1.1 Identities, aliases and canonical order

Retain H Main entries, each the original `(16 variables,56 polynomials)` profile. Retain one entry per **unique original RAM identity**, each the original `(16 variables,1 polynomial)` profile, base32/depth26/d128, live rings512, positions32, blocks16, logical length65536. Boundary positions are C_0,…,C_H. Segment j's outgoing boundary and segment j+1's incoming boundary must be the same original identity. This is already a real consumer check: `jolt/crates/jolt-verifier/src/segment.rs:286–324` validates endpoint shape/contract and checks `incoming_ram == previous.outgoing_ram` with continuation context/counters.

Assign unique RAM indices in first-boundary-occurrence order. Preserve the whole boundary-to-unique-index alias map and explicit N_RAM. Usually N_RAM=H+1; authenticated nonadjacent aliases can reduce it. Equal profiles, equal decoded values, equal query weights or equal planning classes do not alias distinct originals. Mandatory adjacent aliases must not be double-counted. H is history length, not source polynomial count or segment trace length.

The smallest integration can keep the ordered-prefix-plus-final-group representation: designate the last Main handle as final; put unique RAMs in canonical index order then earlier Main handles in history order in the prefix; bind the permutation from semantic roster to native order. This proposed convention is deterministic, not a source merger. Every original retains its own group and payload. `CommittedGroupBatchProfile::from_profiles` simply designates the last profile as final (`akita/crates/akita-params/src/schedule/profiles.rs:588–618`); `FoldGroups` is a nonempty generic roster, not RAM-only (`layout/params/groups.rs:15–57`). Internal root processing is final-first (`layout/opening_layout.rs:173–185`); retain its mapping to public order.

### 1.2 Raw21 is a query view, not a RAM21 commitment

Preserve each real original RAM ring

$$s_{C,b,p,\ell}(Y)=\sum_{a=0}^{127}s_{C,b,p,\ell,a}Y^a,\qquad 0\le\ell<26.$$

Register raw axes `(coefficient[7],digit[5],position[5],block[4])`. The raw MLE F_C has21 query variables, real entries at digit0…25, and zero at six **absent raw addresses**26…31. Physical source depth remains26, profile arity16 and polynomial count1. Plane25 remains a real signed plane; it is not a canonical residual digit. Main depth1 remains each declared logical16-variable Main component. Main can be viewed as a public-query cylinder in the common21-axis query domain without altering its commitment.

Old decoded RAM equations are lifted by the exact adjoint

$$\phi_u(X)=\operatorname{eq}(u,X_{\rm address})\sum_{\ell=0}^{25}\operatorname{eq}(X_{\rm digit},\ell)32^\ell,$$

with the registered address permutation. New raw equations use equality on all21 axes. Every independent assertion keeps its own Z exponent; sharing a source does not merge equations at distinct points. The endpoint roster56H+N_RAM is separate from the query-equation count: the global-range review distinguishes direct frontend120H+N_RAM, or58H+N_RAM only under its separately certified normalization. All final components remain required even when current query weights vanish. Range indexing adds `ceilLog2(N_RAM)` axes: its n is22,23,23,30 for the distinct-boundary H1,2,3,256 cases; these source-index axes do not enter the final native RAM query point.

The ordinary API cannot simply receive21 coordinates against profile16. `OpeningClaims::committed_layout` requires point length equal to profile arity (`akita/crates/akita-types/src/opening_claims.rs:111–146`); row/layout checks and prepared points assume that ordinary shape (`akita-schedules/src/resolve.rs:68–78`; `akita-types/src/subring_coefficient_packing.rs:39–134`). The view type must explicitly carry immutable commitment geometry separately from query arity/axis recipe. No fake P26 or RAM21 profile is permitted.

### 1.3 Only the consuming functional changes

For raw point `(u_A,u_D,u_P,u_B)` define

$$E_{C,b}(Y)=\sum_{p,\ell<26}\operatorname{eq}(u_P,p)\operatorname{eq}(u_D,\ell)s_{C,b,p,\ell}(Y),$$

$$F_C(u)=\sum_b\operatorname{eq}(u_B,b)\sum_a\operatorname{eq}(u_A,a)\operatorname{coeff}_aE_{C,b}(Y).$$

Decoded view replaces `eq(u_D,ell)` by32^ell. For shell coordinate c_{C,b}, the source response remains

$$Z_{C,p,\ell}=\sum_b c_{C,b}s_{C,b,p,\ell},$$

and consistency is

$$\sum_{p,\ell<26}\operatorname{eq}(u_P,p)\operatorname{eq}(u_D,\ell)Z_{C,p,\ell}=\sum_b c_{C,b}E_{C,b}.$$

Public K weights can be unrestricted field values without enlarging the unweighted original source-fold norm: first form Z from original planes, then apply the linear functional. E partials are full-field values, so D's full-field depth remains necessary. Never substitute query weights into original A/source decomposition, B digits, response recomposition or quotient gadgets. Ring-linear unit cancellation precedes applying the coefficient functional.

**Actual existing producer and consumer paths:** native PackedRam coefficient packing exists and only supports k1h1/native128 (`akita-cpu-backend/src/sources/packed_ram.rs:381–438`). Its coefficient callback currently Horner-decodes26 planes using32. The original-owner adapter retains the same Arc/profile/source ID but prepares an ordinary16-coordinate point and explicitly does not grant arbitrary raw-plane openings (`opaque/ordinary_projection.rs:31–87,105–166`). Shared semantic validation builds `witness_gadget` from the original inner-base powers (`akita-types/src/proof/coefficient_packing_relation.rs:506–512`). Prover packing-Z terms consume that vector (`akita-cpu-backend/src/opaque/sumcheck/relation_range_image/coefficient_packing_terms.rs:280–300`).

Changing just the vector is insufficient: verifier compact factors require geometric weights (`akita-verifier/src/coefficient_packing_relation/compact.rs:281–340,638–715`). Raw digit equality weights are not that gadget and can contain zeros. They need exact bit-product/dense-weight factors on the real26 addresses, not division by a possibly zero initial weight. Preserve the decoded geometric fast path and original A/source equation ownership. This producing-to-consuming dispatch integration is missing now; raw-grid metadata in the separate E2/full-ring system does not register the K/K native raw view.

## 2. Current restrictions versus reusable generic machinery

### Exact current restrictions

1. `find_schedule_for_frozen_profiles` requires final `UnitOneHot {source_chunk_size:1}` with bound1; **every prefix contract must be OriginalBalancedDigitPlanes at basis5/bound128/open128** (`akita-planner/src/planner.rs:791–841`). Earlier Mains are therefore rejected. It does not hardcode two RAMs or polynomial count56: exact application-profile registration remains a separate requirement.
2. The entry requires Legacy, challenge/claim extension degrees1, Q128OffsetA7F7, current SIS digest and default single chunks (`:804–817`). Retain these for initial AQRA.
3. Frozen root work uses carrier equal to each source d_A, degree1, hence h1 (`schedule_params/suffix_dp/candidates.rs:187–250`). The actual original owner and RAM kernels further restrict d128/s128. This restriction is root-only; newly generated suffix sources remain generic.
4. Root basis is the policy minimum rather than its whole interval. The entry rejects an original outer basis above that minimum (`planner.rs:888–890`; documentation`:780–786`). A wider interval merely containing5 does not make current code search5. These profiles' frozen B inequalities force5 anyway.
5. Its supplied challenge-hook precheck compares against productionShell(d_A) (`planner.rs:892–906`). That works for original native128, not Dense d512/s64; existing carrier-aware artifact dispatch is the correct generic pattern (§7).
6. `root_candidate_for_frozen_profile` materializes unchanged final A/B and creates only consumer D (`planner.rs:506–575`). Full suffix constraints use the frozen final only at level0. Scalar `find_adapted_schedule` has unconstrained fallback and is not an all-frozen-original route (`:939–1050`).
7. Fold response grinding uses **one joint nonce**, not separately chosen per-group nonces: every group is probed in one transaction, any rejection discards the nonce, and only the first joint winner commits (`akita-prover/src/protocol/fold_grind.rs:203–310,399–476`). Existing limit4096 (`akita-params/src/transcript_grinding.rs:17–20`).
8. Ordinary claim/layout/native-owner/relation schemas do not dispatch raw21 while preserving profile16. Present root completion APIs are usable only after that migration.

### Existing reusable patterns

- `precommitted_group_equivalence_classes` compares full frozen profile **and producer contract** (`planner.rs:67–102`). Classes share planning metadata, not source identities or commitments. Earlier Main plus RAM give only two classes regardless of H. Add view/method-capability discrimination where it affects assignment or cached semantics.
- `materialize_source_group_for_open_basis` derives genuine source norms and group response depth and admits unchanged A/B (`:106–211`). It already handles a Main contract in a prefix; the public RAM-only guard is what rejects it. Its full-ring E2 branch is not initial K/K sizing.
- `precommitted_groups_for_open_basis` adds per-class D width times multiplicity but restores **every** group (`:234–278`). The width authority is `opening_d_segment_width` (`akita-params/src/layout/params/precommitted.rs:123–158`). Already D-native widths are added once, not projected again.
- `PreparedRootProducers`, frozen-final root construction and complete suffix DP are the required architecture (`planner.rs:293–382,506–575,1100–1350`). MAX_PRECOMMIT_OPENING_PRODUCTS256 bounds **class assignments**, not original count; the frozen h1 branch avoids that product altogether (`suffix_dp/candidates.rs:100–169`).
- Native groups/layout/witness/RHS consumers route arbitrary ordered groups. Row admission independently audits exact profiles, matrix geometry/caps/methods and transition lengths (`akita-schedules/src/audit.rs:143–225,245–380,563–600`; `resolve.rs:20–49,82–164`). Artifact provenance remains separately trusted.
- `begin_batched_reduction` validates/binds every original handle then performs one root; `PendingNativeProver::finish` runs every remaining fold and actual terminal (`akita-prover/src/protocol/prove/root.rs:32–108,151–232`). A pending root is not acceptance. Independent `batched_verify` checks profiles, root, full suffix and transcript finish (`akita-verifier/src/fold/verify.rs:24–154`). Reuse these continuations, not completed per-segment PCS proofs.

## 3. Response bounds: honest sizing and accepted envelopes are separate

Let a_g be the enforced range alphabet base, b_g the response recomposition base, delta_g response depth, m summed response chunks, and omega_g actual shell L1 mass. Set S_delta(b)=sum_{j<delta}b^j. The accepted effective response interval is

$$[-m(a_g/2)S_{\delta_g}(b_g),\;m(a_g/2-1)S_{\delta_g}(b_g)].$$

Its absolute envelope and exact difference diameter are

$$Z_{\infty,g}=m(a_g/2)S_{\delta_g}(b_g),\qquad\Delta_g=m(a_g-1)S_{\delta_g}(b_g).$$

These are weak-certificate/verifier bounds, not honest source bounds. The product certificate has `rho_g s_g=r_g`, `||rho_g||_1<=kappa_g=2omega_g`, `||r_g||_inf<=Delta_g`. In the inspected ordinary route a_g=b_g: the scanner base is `1 << open.log_basis` (`akita-verifier/src/stages/ring_switch.rs:233–243`). Thus

$$\Delta_g=m(b_g^{\delta_g}-1),\qquad\eta_{A,g}=2\kappa_g\Delta_g=4\omega_gm(b_g^{\delta_g}-1).$$

`rounded_up_role_a_inf_norm` uses exactly this diameter/chunk sum before upward rounding to an audited cell (`akita-params/src/sis/norm_bound.rs:186–235`). Do not double Delta again. A symmetric direct-terminal cap Z_T instead gives diameter2Z_T and collision radius8omega_T Z_T.

### 3.1 Source-owned norms and physical union counts

`CommittedSourceContract::source_norms` (`sis/committed_source.rs:165–282`) authenticates the original plane basis/depth and gives:

| One original group | P | B | M | source depth | honest digit Linf / per-ring L1 | complete energy upper bound | full response coefficients/chunk |
|---|---:|---:|---:|---:|---:|---:|---:|
|Main56|56|1|512|1|1 /128|56·65536=3670016|512·128=65536|
|RAM1|1|16|32|26|16 /2048|256·26·65536=436207616|32·26·128=106496|

RAM uses the full alphabet[-16,15] on all26 planes, not residual-plane Linf4. Honest source constructors/norm declarations do not prove an adversarial extracted source already satisfies those predicates.

For the balanced-source policy, the deterministic cap is

$$C_{\rm det}=PB\min(\|c\|_\infty\|s\|_1,\omega\|s\|_\infty),$$

and the current tail cap is

$$C_{\rm tail}=\left\lceil\sqrt{2PB\|c\|_2^2\|s\|_\infty^2L}\right\rceil,$$

where N=`num_fold_coeffs/m` and

$$L=\left\lceil{71\over100}\left\lceil\log_2\left\lceil{16N\over7}\right\rceil\right\rceil.$$

The policy takes min(C_det,C_tail) and the least depth whose **positive** balanced reach contains that symmetric cap. Its logical pre-chunking baseline is deliberate; do not silently substitute a per-window tail. Sources: `honest_fold_policy.rs:54–91`, `fold_linf_cap.rs:17–57,67–152`, `norm_bound.rs:363–421`.

Main uses the distinct canonical one-hot Chernoff policy. At source chunk1/one chunk it has PB56 independent challenge groups and union N65536, evaluates lambda=2^e for e=-12,…,2, and conservatively prices

$$2N\exp(PB\log M_X(\lambda)-\lambda t)\le7/8.$$

It caps by deterministic convolution and maps the threshold to positive gadget reach (`honest_fold_policy.rs:115–234,272–306`). More chunks use their actual source/window incidence and full physical union. Conservative floating-point rounding helpers are part of that policy identity, not an exact symbolic theorem or adversarial resource/advantage estimator.

### 3.2 Initial root: a closed deterministic option

Production s128 config(31,0) has omega31, kappa62, shell squared norm31. At base32/m1:

- Main deterministic cap56·31=1736.
- RAM deterministic cap16·31·16=7936.
- RAM balanced query N106496 gives L13 and tail ceiling1817, hence response depth3.
- The saved original decoded control records Main one-hot response depth2 and RAM depth3 (`AKITA_INSTANCE_ACCUMULATION_ORDINARY_CONTROL_LEDGER.txt:14–16`). These are historical entries, not freshly evaluated thresholds here. Repeating separate equal-profile groups does not change each sizing query.

Depth2 accepts[-528,495], Zinf528, Delta1023, raw A radius126852. Depth3 accepts[-16912,15855], Zinf16912, Delta32767, raw A radius4063108. Frozen A capacity8126340 is neither response magnitude nor response depth.

For one nonce, ideal independent group draws would give joint acceptance product_g p_g. Per-group p_g>=1/8 gives only8^{−(H+N_RAM)}, not a useful4096-attempt guarantee. This is an insufficient batch bound, not proof that actual H256 honest proofs fail. Emit a genuine joint bound for tail-sized root depths, or choose admitted deterministic depths; no new retry policy is needed.

**All-valid root option:** set Main and RAM response depths to3. Positive reach15855 covers1736 and7936 for every allowed unfiltered native128 shell. Its audited A radius4063108<=8126340. Depth4 would require124·(32^4−1)=130023300>8126340. Thus a finite generalized response-depth domain can include the existing policy depth and sufficient deterministic depth, with maximum fixed by frozen A. The current helper returns only the policy depth; exposing the extra option is a minimal materializer extension, not already implemented search. This closes root response feasibility at every H without combining handles; it does not select or prove completeness of the resulting suffix.

## 4. Matrix/path admission and root width closure

### 4.1 Frozen A/B

Original literal profiles are in `AKITA_INSTANCE_ACCUMULATION_ORDINARY_CONTROL_LEDGER.txt:2,14–16`; RAM geometry also appears in `akita-config/src/full_ring.rs:79–111`. Exact width identities are validated in `layout/params/precommitted.rs:437–486`.

| Original matrix | d | rank | width | selected Linf capacity |
|---|---:|---:|---:|---:|
|Main A|128|3|512|8126340|
|Main B|128|1|4368|31|
|RAM A|128|3|832|8126340|
|RAM B|128|1|1248|31|

A width=M·source_depth. Unsliced B width=P·B·rank_A·outer_depth·(d_A/d_B), with outer base32/depth26/one slice. No H factor belongs in an original matrix's width.

For every frozen group require exact profile/producer validation; authentic source norms/geometry; policy/digest/q/role keys; frozen rank covering own table width; and

$$\operatorname{ceil}_A\!\left(4\omega_gm(b_{\rm open}^{\delta_g}-1)\right)\le C_{A,g},$$

$$\operatorname{ceil}_B(2^{\log b_{\rm outer}}-1)\le C_{B,g},\quad \log b_{\rm open}\ge\log b_{\rm outer},\quad\operatorname{ceil}_B(2^{\log b_{\rm open}}-1)\le C_{B,g}.$$

Here log b is the stored log_basis, not a numerical logarithm evaluated at runtime. Require executable method/carrier/role projections and actual challenge config. Source checks: `planner.rs:165–211,844–914`, `layout/params/precommitted.rs:321–429`, `sis/ajtai_key.rs:32–121`, `akita-schedules/src/audit.rs:143–195`.

All B profiles have outer logbasis5/capacity31. Consumer opening logbasis is therefore exactly5:6 needs63; below5 violates domination. Full-field E digit depth is26. At A cell(d128,8126340), rank-three cutoff23489 covers512/832 (`sis/generated_sis_table/q128.rs:95`). At B/D cell(d128,31), rank-one cutoff28621854 covers4368/1248 (`:73`). These are modeled table admission facts, not probability bounds.

### 4.2 Shared D, endpoints and shell coordinates

For physical partial width v_g,

$$w_{D,g}=\delta_{{\rm open},g}P_gB_g(v_g/d_D),\qquad w_D=\sum_gw_{D,g}.$$

Packing geometry has d_A=khs, v_g=ks, embedding Y→X^{kh}. Require v_g divisible by common d_D and executable A/B/D projections. Initial originals have k=h1,v128. At d_D128:

$$w_D=26(56H+16N_{\rm RAM})=1456H+416N_{\rm RAM}.$$

D is newly consuming-owned: select a real admitted key at capacity31 and minimum rank for the **sum**. Check table width and complete image independently:

$$w_D\le W_D(d_D,31,r_D),\qquad16r_Dd_D\le8192\text{ bytes}.$$

The frozen candidate adds D-native widths once and constructs D (`planner.rs:533–556`); independent audit recomputes the sum (`audit.rs:198–225`).

| H | N_RAM, distinct boundaries | originals H+N_RAM | endpoints56H+N_RAM | W=56H+16N_RAM | D width/d128 |
|---:|---:|---:|---:|---:|---:|
|1|2|3|58|88|2288|
|2|3|5|115|160|4160|
|3|4|7|172|232|6032|
|256|257|513|14593|18448|479648|

These are derived counts, not validated schedules. Exact aliases replace N_RAM by its actual smaller count. Shell coordinates are ring draws per claim/block, not digit/position/coefficient counts or endpoint scalars. Flat coordinate-wise root fork arity is W+1. Existing GrindingPlan records H sites of local multiplicity57 and N_RAM sites of17; their **sum**57H+17N_RAM is not W+1 or a complete extraction work price (`akita-params/src/transcript_grinding/plan.rs:392–414`).

All listed d128 widths fit rank1. D's image is therefore128 coefficients, **not128*w_D**. H256 root D alone needs479648·128=61394944 shared setup field elements, 982319104 bytes at16 bytes/element. This is a static lower bound on setup provision, not the selected complete envelope. Original source matrices do not need rekeying: existing prefix coefficients remain unchanged when capacity grows. Cost/memory is substantial, but no4MiB setup cap was requested. An explicit installation budget can still reject it.

### 4.3 Complete F/H compression

Each original B image has `slice_count*rank_B*d_B`=128 coefficients. Q128 compression uses rank1/d16 then rank1/d8, negative-binary alphabet{-1,0}/difference1. For C complete source coefficients,

$$w_0=\lceil128C/16\rceil=8C,\qquad w_1=\lceil128\cdot16/8\rceil=256.$$

Require C·16<=8192 bytes, w_0<=7077, w_1<=508. These actual checks appear in `akita-params/src/compression.rs:18–24,235–315`, `compression/chain.rs:54–114,142–190`, `sis/compression.rs:24–32,76–91`. With C128, widths1024/256 fit. Each original F separately fits; shared H over rank-one/native128 D has the same capacities. Final payload is8 coefficients/128 bytes per path.

The cap applies to complete **output images**, not input matrix widths, total source data, or the sum of separate B images. Merging all originals into one F would make a new commitment and a spurious capacity failure. Preserve every separate F. Later rows must recheck complete sliced B and D images: SIS rank growth can satisfy width yet break8KiB compression. Root and level1 remain mandatory compressed in the current epoch; raw cutover is available only later (`compression.rs:74–97`). Reslicing/rekeying a frozen original is not an escape route.

## 5. Separately required source-comparison and joint-setup conditions

### 5.1 Mixed radii

For two reachable certificates u,v of the same original source and exact A view, source difference extraction gives

$$z_A=\rho_vr_u-\rho_ur_v=\rho_u\rho_v(s_u-s_v),\qquad Az_A=0.$$

Unit action makes a differing source yield nonzero z_A; do not bound inverse units. With `K1_u>=||rho_u||_1`, `Binf_u>=||r_u||_inf`, require

$$K1_vBinf_u+K1_uBinf_v\le C_A.$$

Common root kappa/depth yields its diagonal radius. Different rows/methods/chunks/filters/setup references/source cuts require their **actual mixed pair**, not assumed maxima of unrelated diagonal minima. An installed canonical setup reference has rho_ref1 and the norm of its actual original A-input digits, giving `Binf_weak+K1_weak*Binf_ref`.

For each B/F view, check complete input-alphabet differences across compared occurrences. Balanced bases a_u,a_v give(a_u+a_v)/2−1; common base32 gives31. F/H radius1 requires actual{-1,0} on every real/padded grid coordinate, not an unproved padding assertion. D/H only compares retained siblings sharing that actual payload; unrelated points need not share D/H. Global identity remains original A/B/F.

If generated suffixes use the physical L2 route, retain their complete verified squared cap C and actual accepted operator bound O. Diagonal squared radius is64O^2C (`sis/norm_bound.rs:88–107`); a mixed pair can use `Kop_v B2_u+Kop_u B2_v`, squared with cross terms, or another genuinely available Young pairing. Do not add a width factor to a complete B2. Frozen root groups remain Linf; current scalar/precommit restrictions do not permit multigroup L2. Terminal contributes only its actual A/direct response bound, not fictional B/D/F/H or a successor scanner.

`FoldSchedule::sis_occurrences()` enumerates executable selected matrices/paths (`akita-params/src/schedule/sis_occurrences.rs:61–250`), but does not certify source-cut ownership, reachable mixed pairs or collision-finder runtime. The independently checked comparison inventory of [AKITA_RELATION_ACCUMULATION_JOINT_SETUP_CERTIFICATE.md](AKITA_RELATION_ACCUMULATION_JOINT_SETUP_CERTIFICATE.md) remains necessary. No new concrete knowledge axiom follows from table admission.

### 5.2 Joint prefix law at the approved assumption

A/B/F/D/H are **not independently sampled role keys**. One coefficient prefix is reused and rank/width/d are not absorbed (`akita-types/src/proof/setup.rs:227–258`). Each exact view uses its own row-major index `(row*width+column)*d` over rank*width*d coefficients (`akita-params/src/layout/flat_matrix.rs:61–185`); changing width changes stride. It is not a fixed-stride global rectangular submatrix.

Default seed is fixed (`setup.rs:219–225`). The page grammar binds domain, derivation,4096-field page size, seed, field and page index (`:317–377`). Fp128 uses exact canonical16-byte rejection (`jolt/crates/jolt-field/src/solinas/fp128.rs:1089–1095`, `solinas/mod.rs:73–100`). Under the approved programmable ideal setup-XOF model, embed the named Module-SIS challenge in a coefficient prefix and derive **all** overlapping views/installed references consistently. The simulation must preserve conditional rejected bytes, page tails, repeated reads, extensions and adversarial recomputation—not just idealize accepted field outputs independently. This is a conditional joint reduction at actual original radii, not a new lattice assumption or a statistical guarantee for a concrete fixed SHAKE realization.

Installed setup sources commit actual prefix `[0..n_prefix]`, including real coefficients beyond natural_len (`akita-cpu-backend/src/setup/setup_prefix.rs:98–143`). Zero query support is not zero source padding. Record each slot's actual profile/natural_len/full committed extent/canonical A digits/B/F path. Capacity is the maximum actual footprint over selected views/slots, not their sum or max_rank*max_width (`akita-params/src/layout/setup_envelope.rs:12–20,74–110`).

The hard table gate is modeled ADPS16 quantum LGSA cost>=128 (`sis/generated_sis_table/mod.rs:1–10`); there is no estimator-bits-to-advantage API. The complete conditional ledger has the form

$$\epsilon\le\epsilon_{\rm continued/source/range}+\epsilon_{\rm query}+\epsilon_{\rm indexed\ ROM}+\epsilon_{\rm setup\ model/simulation}+\sum_u\operatorname{Adv}_{{\rm original}\ M\text{-}SIS,u}(T_{X,u};\eta_u),$$

with actual routes/radii and the approved classical indexed-ROM/setup idealizations. This is a symbolic conditional inequality, **not numerical admission**. Keep actual atom/cutoff/sampler terms, finite query horizon, range/source cut and terminal error. Do not replace any computational term by2^-128 or demand a stronger target.

Work must count the executed reduction, not proof cost estimates. A transparent ledger can use

$$T_X=T_{\rm base}+\sum_{v\in\mathcal T_{\rm executed}}(T_{\rm adversary}(v)+T_{\rm verifier}(v)+T_{\rm RO}(v)+T_{\rm setup}(v)+T_{\rm algebra}(v)),$$

with all repetitions/rejected probes, retained source comparisons and actual setup scans. For a layer-uniform extractor grammar with child multiplicities f_i, its full tree size is `1+sum_{j=1}^L product_{i=1}^j f_i`; otherwise emit the actual finite node recurrence. W+1, GrindingPlan local site sums, payload estimates and planner score are not themselves this T_X. No f_i or concrete T_X is guessed here.

## 6. Deterministic finite row-selection interface and required ledger

The following specifies implementation output; it is not an existing callable API or code to run.

### 6.1 Request/result contract

A `plan_frozen_source_roster` request contains:

- Nonempty canonical originals, with semantic identity, authentic exact profile/producer contract and retained owner/installation identity for each.
- Main history order, boundary alias map, H/N_RAM and routing permutation. Validate exact Main56/originalRAM1-depth26-logical65536 registration, not just a class tag.
- Independent authenticated view schema per group: Main logical16; RAM raw21 with exact axis/depth/absence recipe. Query schema is separate from commitment profile and ring method.
- Exact field/basis/policy/table digests; finite root capability and complete suffix domains; setup budget/offload choices only when expressly enabled. Initial root remains K/K/native128/single chunk/base32/compressed quotient lift.
- Root response completeness rule: joint-tail certificate or admitted all-valid deterministic depths; finite maximum from original A. Do not infer joint success from per-group1/8.

The result returns unchanged ordered profiles, ordered view schema, full root/recursive/terminal schedule, canonical selection descriptor/digest, setup/slot closure, completeness evidence and matrix/path/pair/resource ledger, or explicit finite-domain exhaustion with rejecting gates. A reusable row key is structural; runtime identities/points/values are bound later and equality of keys does not alias sources.

### 6.2 Finite deterministic selection sequence

1. Validate originals/contracts/aliases/capabilities first. Reject unsupported view/method; do not decode, relabel or recreate a source.
2. Designate final Main and retain all other originals as frozen groups. Form only interchangeable planning classes using profile/contract/view-capability requirements.
3. Enumerate admitted common D dimensions/method-carriers. Initial original carrier128/h1 is fixed, and B pins root logbasis5. Enumerate admitted finite response-depth choices including all-valid3. Do not change original splits, source digits, slices, ranks or encoding.
4. Materialize every group's source-owned norms/opening against frozen A/B. Sum exact D-native widths; construct admitted shared D/H; retain every original F. Independently reject rank and image-cap failures.
5. Calculate full root witness/range/RHS/tail geometry and exact live next length via canonical output sizing. Do not substitute endpoint count or a guessed power-of-two shape. Existing root input convention is `2^final_group.num_vars`,65536 here, not total source bytes (`akita-planner/src/planner.rs:1158–1182`); root expansion is permitted. This convention is not a work or contraction certificate.
6. Run existing complete suffix search from exact root output with actual source bounds/base, dimension ceilings, response routes/filters/chunks, monotone payload/relation phase and setup direct/offload state. Current depth cap12 (`akita-schedules/src/runtime.rs:344–345`). Original root-only h1 constraints must not leak into generated suffixes. All shrinking edges must use canonical actual next-witness geometry; setup-offloaded edges also enforce weighted-bit contraction and decreasing padded setup support (`suffix_dp.rs:54–77`, `suffix_dp/search.rs:147–180`). Close with the actual admitted A-only terminal, direct cap, codec and physical shape. Root itself need not shrink.
7. Select a **complete** feasible row using current complete policy ordering/canonical descriptor tie-break (`akita-planner/src/schedule_params/objective.rs:1–81,235–287`), not first enumeration hit. Any different objective is explicit in policy identity. Honest score is not adversarial work.
8. Independently rederive profiles/views, widths/caps/methods and every transition via view-aware row admission, then install trusted catalog provenance. Current ordinary row admission is not raw21 acceptance. No scalar-guide/unconstrained/decoded fallback is permitted on exhaustion.

### 6.3 Mandatory emitted/checkable ledger

For each requested H and actual alias roster, emit:

**Roster:** identities/alias map; semantic/native/transcript orders; original profile bytes/contracts; view tags/axes; original commitment versus query arity; components/endpoint slots; W_g/W_total/local grinding multiplicities; all endpoints even at zero weights.

**Every root group:** d/P/M/B/source depths/bases, real logical/physical lengths and authentic norms; selected k/h/s/method/embedding/filter; chunks/physical response union; deterministic/tail/Chernoff inputs, selected depth and batch completeness evidence; range alphabet/recomposition base; Zinf/Delta/kappa; raw and rounded A/B requirements and frozen capacities/ranks/widths with explicit inequalities.

**Root shared/path geometry:** every D contribution/offset/order, common d_D/rank/width/capacity/table cell; every complete B/D image; all separate F/shared H maps, widths/alphabets/rank-one cutoffs, native/final payloads; real padding/coordinate ownership. Rank-width and image checks are independent.

**Full suffix and terminal:** exact input live/committed/padded/output lengths per level; Z/E/T units, quotient/compression tails/alignment, range domains and real padding; every basis/depth/chunk/method/carrier/filter/security route; A/B/D/F/H selected radii/ranks/widths; monotone phase/topology and contraction checks; every setup slot's ID/profile/full extent/canonical reference; integer/codec/no-wrap obligations; actual A-only terminal direct cap/physical shape. Degree/error normalization must match actual reused drivers, not a generic full-degree estimate.

**Selection/installation:** policy/domain/search revisions; deterministic complete score; descriptor/digest; encoded row bytes/catalog digest/provenance; full setup envelope and verifier scan footprint; slots, maximum group count/scalar count and separately query/source arities. Existing `OpeningClaims::validate` bounds setup.max_num_batched_polys/max_num_vars (`akita-types/src/opening_claims.rs:174–193`); raw query provision must not resize the original source profile.

**Certificates/resources:** original source scan/reference owners; reachable certificate pairs/mixed radii; exact flat-prefix view embedding and all enforced alphabets; continued frontend/global range/query losses; direct-indexed transcript site/sampler/cutoff rules; source/terminal cuts; finite extraction tree/work recurrence; original unconverted runtime-conditioned SIS advantage terms and joint setup/model assumptions.

## 7. Dense control: actual d512 source, carrier64

The singleton nv14 Dense literal at `akita/artifacts/schedules/fp128_dense.aks:128` has d_A512, d_B=d_D64, source base8192/depth10, M4/B8, carrier s64, response base8/depth7, production64 config(31,10), omega51, squared shell norm71. For E=K this is k1/h8, partial width64, embedding Y→X^8. D width43·8=344. Root shell coordinate count is8, not512,64 or41.

A generalized ordinary **decoded** frozen-control path must preserve that source's actual `BalancedSignedDigit` full-field producer and A/B and select genuine carrier64 geometry. It is not an originalRAM raw21 capability. Neither its norms nor its source dimensions replace the Main56/RAM26 contract.

Do not compare its challenge to productionShell(512). Geometry derives config from s and checks d_A=khs (`akita-params/src/layout/subring_packing_geometry.rs:30–82`). Existing trusted-artifact dispatch is already correct: packing uses production_for_ring_dim(challenge_subring_dimension), L2 trace its selected filtered family, ordinary trace the ambient hook (`akita-schedules/src/artifact.rs:754–801`). Generalizing only the RAM-only guard while leaving frozen h1/ambient prechecks would still reject the actual Dense control or misprice it.

Keep AQRA originals' native128 capability narrow; separately reuse generic decoded packing for Dense. Do not claim blanket raw d512 support. Previously recorded Dense H1/2/3/256 controls are separate PCS-only proofs, not a one-root Main/RAM Candidate1 implementation.

## 8. Closed conditions, unresolved rows and real blockers

### Closed

- Exact per-original geometry/norm declarations/26-plane ownership and the mathematical raw21/native ring-linear functional identity.
- Canonical aliasing; all four endpoint/shell/D-width counts; no H-dependent enlargement of an original A/B or per-group honest fold bound.
- Forced root base32 and admitted depth3 all-valid response for both classes.
- Native128 rank-one D and separate F/shared H capacity for requested H<=256, including N_RAM257. No original rekey/merger is required.
- Reusable generic metadata/groups/D summation/full suffix DP/canonical audit/catalog/native finish/independent verifier architecture.

### Unresolved executable values

For every H/alias roster calculate: selected root depth/batch completeness; exact root live witness/tails/range/RHS/transcript dimensions; first and all later suffix splits/bases/dimensions/ranks/routes/filters/chunks/modes; direct/offloaded topology/installed setup profiles and canonical references; exact terminal direct cap/codec/shape; full setup closure and budgets; encoded row/catalog selection; local/global error and source occurrence/pair/runtime ledger. The saved H1 decoded control is not an installed raw21 row. Conservative root inequalities neither select the cheapest row nor prove a complete suffix exists.

### Possible blockers and how to distinguish them

1. **Frozen source capacity:** chosen base/depth/chunks/method or mixed certificate exceeds original A/B/F. Fundamental for that source path; do not resize originals or drop predicates. Initial depth3/base32/native128 root closes this gate.
2. **Shared table/image capacity:** no current rank covers summed D width, or a required compressed B/D image exceeds8KiB. Initial native128 rank1 root closes it at H<=256, not automatically later or for much larger histories.
3. **Finite suffix-domain capacity:** no full shrinking/terminal path within policy bases/dimensions/splits/depth12 or setup offload/installation budget. This is a named domain/implementation failure, not proof an original profile is invalid. Report exact rejecting gate; do not paste an old scalar suffix with stale lengths.
4. **Joint honest-response feasibility:** unchanged tail-sized per-group plans lack a adequate4096-attempt batch bound. Admitted all-valid root3 closes the root concern, but needs explicit materializer support and can change suffix costs. No stronger soundness or new retry policy is required.
5. **Provision/encoding/factor limits:** setup/source/query arities unsupported; finite integer/wire/scalar caps fail; compact factor or serialized-row ceilings fail. Current row limit1MiB, artifact64MiB (`akita-schedules/src/artifact.rs:27–32,80–118`). Do not guess513-group encoding/factor acceptance; emit actual checked sizes. Current raw-geometric rejection is integration failure, not a lattice capacity failure.
6. **Unclosed proof inputs:** raw endpoint routing, original range/helper linkage, source cut/unit/GRID-LOCAL ownership, mixed radius, joint setup or resource certificate absent. Row admission alone does not supply them. No H1610 knowledge axiom or estimator=>Adv conversion is introduced.

## 9. Minimal Rust integration map

1. **Frozen roster/depth planning:** `akita-planner/src/planner.rs` and `schedule_params/suffix_dp/{state,candidates}.rs`: generalize prefix source validation to Main and RAM; preserve designated-final convention/all originals; retain existing complete suffix search; add finite admitted root depth/completeness options. Validation is source/class/capability-sensitive, not index-sensitive.
2. **Independent view grammar/key:** `akita-params/src/layout/params/precommitted.rs`, `layout/opening_layout.rs`, schedule profiles/descriptor; `akita-types/src/opening_claims.rs`, `instance_descriptor/mod.rs`; schedule policy/key/serialization/row audit/catalog. Carry profile16 versus raw query21 explicitly. Catalog rejects duplicate profile-derived prover lookup keys (`akita-schedules/src/artifact.rs:163–201`): coexistence of decoded/raw rows needs a view discriminator, not just a different row digest. Clean caller migration; no decoded fallback.
3. **Original owner opening:** `akita-cpu-backend/src/opaque/ordinary_projection.rs`, `opaque/openings.rs`, native preparation authority/handles, `sources/packed_ram.rs` and coefficient-packing partial kernel. Carry authenticated digit recipe independently of the16-address packing geometry. Read weighted original planes directly without decode/redecompose/new storage ownership. Keep Arc/source ID/profile/A/B/F and source fold unchanged.
4. **Both relation consumers:** `akita-types/src/proof/coefficient_packing_relation.rs`; prover `akita-cpu-backend/src/opaque/sumcheck/relation_range_image/coefficient_packing_terms.rs`; verifier `akita-verifier/src/coefficient_packing_relation/compact.rs` and `stages/opening_claims/coefficient_packing.rs`. Replace only consuming query functional weights, keep26 physical lanes/source identity/response gadget, and provide exact raw compact factors rather than assuming geometric weights.
5. **Native entry/replay/setup:** `akita-prover/src/opening.rs`, `protocol/prove/root.rs`; `akita-verifier/src/fold/{verify,root}.rs`; `akita-config/src/transcript_binding.rs`; `akita-schedules/src/{audit,resolve,artifact}.rs`; setup/slot provisioning. Bind complete original/view/endpoint identity before native scalar/shell challenges; check all original paths and finish every suffix/terminal/transcript. Keep setup namespace/shared-prefix law.
6. **AQRA driver:** form H Main/N_RAM alias roster from retained pre-PCS continued handles; append original range leaves before complete query seal; reduce every selected equation; submit all final Main components/raw RAM endpoints to this one decider. Range/frontend/source cuts remain separately owned. No completed per-segment PCS input, fake P26/RAM21, source aggregation or new assumption is part of this contract.

**Handoff:** this read-only contract and mathematical root/profile closure are complete. Implementation must emit the unresolved full rows/ledgers and migrate all producing/consuming raw-view dispatch paths. Parent owns any subsequent build/test/proof/tamper verification; none is reported here.
