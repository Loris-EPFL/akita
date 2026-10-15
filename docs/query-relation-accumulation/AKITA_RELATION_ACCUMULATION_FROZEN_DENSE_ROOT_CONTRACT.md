# Frozen decoded Dense originals: one-root batch contract

## Result and status

This is a separate benchmark contract, **not** the Main56/original-RAM raw21 AQRA statement. It uses H=1,2,3,256 unique originals of the same upstream fp128 Dense nv14 source, all with the ordinary decoded view. Every original A/B/F/profile/source owner is preserved. Only one consuming D/H root and its complete newly calculated suffix are planned.

**Closed:** common D64 has width344H, hence88064 at H256. The actual q128(d64,bound7) rank-one cutoff503438 covers it. Every original complete B/F image is512 coefficients/8192 bytes—exactly the compression input cap, not64 coefficients. Its F maps have widths4096 and256 and fit their7077/508 cutoffs. Shared rank-one D64 H maps have widths512 and256 and fit. Thus requested root widths do not violate original frozen profiles or root compression capacities.

**Closed honest joint completeness under the approved ideal indexed challenge law:** the existing response depth7 has a much stronger whole-batch bound than its policy's generic per-group1/8 guarantee. For H<=256, one unfiltered nonce's rejection probability is bounded by

$$\epsilon_H\le40960H\exp\!\left(-{898780^2\over19058917376}\right)<{5H\over2^{48}}.$$

At H256 this is<5/2^40<2^-37. No independence between groups or response coefficients is needed for this union bound. Actual root packing forbids the selective-L2 filter; it uses unfiltered production64(31,10). No retries, source changes, smaller soundness radius, or new lattice assumption are introduced. Existing whole-root4096-attempt exhaustion is a subset of nonce0 rejection, so this bound suffices without multiplying a per-group1/8 guarantee or assuming nonce independence.

**Not closed:** the present frozen entry does not accept the Dense producer or actual carrier64 recipe. No H-original full row has been calculated or validated here. Its exact root witness/tails, every suffix row, actual terminal, setup envelope and complete collision/resource certificate must still be emitted and admitted by the executable planner. The literal singleton upstream row is evidence of source/method geometry, not a newly validated generalized row.

Read-only investigation only: no Git operations, repository edits, code, builds/tests/formatters, estimator or proof runs. All numerical entries below are source literals or explicit mathematical deductions. No numerical attack-cost-to-advantage conversion or extra end-to-end target is claimed.

## 1. Exact original profile and decoded carrier recipe

The literal selected singleton row is `akita/artifacts/schedules/fp128_dense.aks:128`. It records:

| Item | Frozen value |
|---|---|
|Field/extension|K=E=Prime128OffsetA7F7|
|Logical group|num_vars14, P1, logical coefficients16384|
|Blocks|live rings32, positions M4, live blocks B8|
|Inner/source decomposition|log_basis13, base8192, source depth10|
|Original A|d512, rank1, width40, selected Linf capacity427818804|
|Original outer slices|8|
|Original B|d64, rank1, width344, outer log_basis3/depth43, selected Linf capacity7|
|Consumer method|SubringCoefficientPacking, carrier s64|
|Packing|k1, h8, embedding Y→X^8, partial physical width64|
|Root challenge|production64, count_pm1=31/count_pm2=10, omega51, squared norm71|
|Consumer opening/response|log_basis_open3, E depth43, response depth7, single chunk|
|Root modes|Compressed, QuotientLift, CanonicalCoefficientTable|

The actual preset is K/K and full-field `BalancedSignedDigit`, not UnitOneHot or OriginalBalancedDigitPlanes: `akita-config/src/proof_optimized/fp128.rs:111–132`; the macro's declaration is `DecompositionParams {log_basis:3,log_commit_bound:128,log_open_bound:None}` (`akita-config/src/proof_optimized.rs:152–166`). The immutable physical producer profile independently has inner base8192/depth10. It is not re-decomposed at base8 when consumed.

Canonical source norm calculation permits Linf4096/per-ring L1=512·4096=2097152. Its nine full13-bit planes have maximum4096 and last11-bit residual plane maximum1024, giving declared aggregate energy

$$16384(9\cdot4096^2+1024^2)=2491081031680.$$

The proof below conservatively uses4096 on **every** plane, so does not rely on residual-plane savings. Source-backed norm calculation is `akita-params/src/sis/committed_source.rs:165–239`. These are honest canonical-source norms; they are not a range theorem for an arbitrary weakly extracted adversarial source.

The decoded14-coordinate point has ordinary packing axes `(packing[3],subring-tail[6],position[2],block[3])`. They represent the same9 coefficient/address bits plus2 position and3 block bits. No raw digit query, typed raw21 extension, P10 profile, new source commitment or aggregation is involved. Distinct handles remain H groups even when their profiles and planning class are equal. Designate the last as final and retain earlier ones as frozen prefix entries in canonical source order.

## 2. Exact response acceptance and unchanged soundness admission

Let b=8, delta7, S7=sum_{f=0}^6 8^f=(8^7−1)/7=299593. Balanced digits[-4,3] give the exact accepted original-plane response interval

$$[-4S_7,3S_7]=[-1198372,898779].$$

The positive reach is **898779**, not898728. Subtracting51 creates an optional conservative51-unit margin; it is not the actual checked gate. The probe's negative/positive bounds are built by `balanced_digit_representable_bounds(log_basis_open,num_digits_fold)` (`akita-prover/src/protocol/fold_grind.rs:443–470`), bound again to the installed group policy (`akita-cpu-backend/src/opaque/openings.rs:190–219`), and checked against centered signed extrema using strict `>` rejection (`opaque/fold.rs:405–420`). Thus equality to either endpoint is accepted. Source interval formulas are `akita-params/src/sis/decomposition_digits.rs:37–98`.

The largest absolute envelope is1198372; the difference diameter is

$$\Delta_7=7S_7=2097151.$$

With omega51, weak unit-product multiplier bound kappa102, source weak numerator boundDelta7, the raw diagonal A radius is

$$\beta_7=2\kappa\Delta_7=4\cdot51\cdot(8^7-1)=427818804.$$

This is **exactly** the frozen A cell; it is not replaced by the honest tail threshold. `rounded_up_role_a_inf_norm` uses actual shell mass, interval difference and chunks (`akita-params/src/sis/norm_bound.rs:190–235`). Table(d512,427818804) rank1 cutoff1538 covers A width40 (`sis/generated_sis_table/q128.rs:240`).

The deterministic honest source cap is

$$8\min(1\cdot2097152,51\cdot4096)=8\cdot51\cdot4096=1671168.$$

It exceeds depth7's positive reach. An all-valid symmetric-cap plan needs delta8. Its positive reach is7190235, but its A radius is

$$\beta_8=204(8^8-1)=3422551860>427818804.$$

Therefore the frozen source **cannot** be converted to an all-valid depth8 base8/omega51/single-chunk root while retaining its A. Increasing response depth, rank, base, chunks or filtering to bypass this is not part of this benchmark. Depth7 remains safe for every accepted proof; honest completeness must use the actual unfiltered distribution, which is sufficient below.

The existing balanced honest policy itself gives tail cap478234: raw unionN20480 makes `ceilLog2(ceil(16N/7))=16`, L=ceil(0.71·16)=12, and

$$C_{\rm policy}=\left\lceil\sqrt{2\cdot8\cdot71\cdot4096^2\cdot12}\right\rceil=478234.$$

Integer checks:478233²=228706802289 <228707008512 <=478234²=228707758756. Depth6 positive reach112347 is too small and depth7 reach898779 covers it. This explains the historical selected delta7, but its generic per-group failure7/8 alone is not the following stronger joint certificate. Sources: `akita-params/src/sis/honest_fold_policy.rs:54–90`, `fold_linf_cap.rs:17–57,67–152`.

## 3. Source-backed whole-batch Rademacher certificate

### 3.1 Actual sampler and conditioning law

For each source group and candidate nonce, the native draw contains8 carrier64 coordinates, one per block. Its sample payload binds group identity/count/config/carrier/packing domain (`akita-challenges/src/fold_draw.rs:75–136`). Each coordinate has a separate indexed SHAKE stream from the group's32-byte root and LE64 coordinate index (`sampler/xof.rs:1–38`).

The source sampler first draws41 distinct positions by unbiased partial Fisher–Yates (`sampler/position_sample.rs:119–202`). It then reads41 fresh sign bytes and uses their independent low bits: the first31 positions get magnitude1, last10 magnitude2 (`sampler/signed_sparse.rs:34–56`). Magnitudes are assigned to the sampled ordered support before signs. Under the approved ideal indexed XOF/ROM law, unused bytes after the variable-length position sampler are fresh; conditioning on the complete positions/magnitudes leaves the signs independent Rademachers. Without-replacement support sampling does not damage this argument.

Fix all honest original digit coefficients and source ownership **before the fresh candidate challenge**. Condition on the block support/magnitude assignments, not on signs, a response-acceptance event, adversarially selected nonce, or the final winner. The statement is uniform over every fixed honest source obeying the norm bound. Source data chosen after reading these signs would not satisfy this conditioning prerequisite. Concrete fixed SHAKE pseudorandomness is not a proof of literal independent signs; the probability statement is under the approved ideal indexed law, with any real/model/setup terms retained separately.

The root has **no operator filter**: coefficient-packing draws reject a requested filter (`fold_draw.rs:85–96`); the Linf route carries response_l2_sq_cap=None (`fold_grind.rs:449–462`); sampler `rejection=None` takes the Unrestricted branch (`sampler/mod.rs:84–92,137–149`). The selective-L2 d64 shell is(31,11), not the actual(31,10) root (`config.rs:15–26`). No accepted-filter conditioning factor belongs in this root tail proof. Future trace/L2 suffix sites require their own actual filtered law and caps.

### 3.2 Embedded coefficient formula and variance

Carrier challenge is embedded by preserving signs/magnitudes and multiplying positions by8 (`akita-challenges/src/challenges.rs:238–288`). Native fold dispatch probes the ambient A512 ring using these embedded challenges (`akita-prover/src/protocol/fold_grind.rs:255–295`). Dense source folds its original canonical digit planes into centered i32 rows (`akita-cpu-backend/src/sources/dense/ops.rs:130–168`).

For fixed source group g, position p<4, source plane ell<10 and A coefficient a<512, write the integer raw folded coefficient as

$$Z_{g,p,\ell,a}=\sum_{b=0}^{7}\sum_{j=1}^{41}\varepsilon_{g,b,j}\,m_j\,\sigma_{a,t_{g,b,j}}\,s_{g,b,p,\ell,\,a-8t_{g,b,j}\bmod512},$$

where m_j is1 for31 terms and2 for10, and sigma∈{−1,+1} is the deterministic negacyclic wrap sign. Conditioning on support and source turns every non-sign factor into a constant. Each output coefficient gets one source coefficient per sparse support entry; h8 does not multiply the number or variance of summands. The eight A cosets contribute to the **union count**, not as an extra factor8 in this variance.

Absorbing sigma/source signs into independent Rademachers,

$$\mathbb E[Z\mid\text{support,source}]=0,\qquad \sum\alpha_{b,j}^2\le8(31+4\cdot10)4096^2=V=9529458688.$$

For any real lambda, `cosh(x)<=exp(x²/2)` gives

$$\mathbb E[e^{\lambda Z}\mid\text{support,source}]\le\exp(\lambda^2V/2).$$

Chernoff minimization at lambda=t/V yields

$$\Pr(Z\ge t)\le e^{-t^2/(2V)},\quad\Pr(Z\le-t)\le e^{-t^2/(2V)}.$$

Remove the support conditioning by averaging: the same bound holds for every support. This integer calculation cannot wrap modulo K because even the deterministic magnitude1671168 is vastly below q/2 and fits i32.

### 3.3 Exact integer gates and all H groups

Actual positive rejection is Z>=898780 and negative rejection is Z<=−1198373. The asymmetric union could use

$$\epsilon_H\le HN\left(e^{-898780^2/(2V)}+e^{-1198373^2/(2V)}\right),$$

but the simpler symmetric sufficient event `|Z|<=898779` gives

$$\epsilon_H\le2HN e^{-898780^2/(2V)}.$$

Here **N=40·512=20480 raw folded coefficients per group**, all4 positions,10 real source planes and512 A coefficients. It is not partial width64, query arity14,8 shells, or the7-times-larger response-digit payload143360. Every raw coefficient is included; no response coefficient independence is assumed.

Exact square898780²=807805488400 and2V=19058917376 give

$$\frac{898780^2}{2V}>\frac{847}{20}=42.35>61\cdot\frac{347}{500}>61\ln2.$$

The elementary rational upper bound ln2<347/500 suffices. Hence the per-tail exponential is<2^-61, giving

$$\epsilon_H<\frac{40960H}{2^{61}}=\frac{5H}{2^{48}}.$$

At H1,2,3,256 the corresponding bounds are5/2^48,10/2^48,15/2^48,5/2^40; the last is<2^-37. This is an honest response-completeness bound, not a cryptographic soundness target or statistical model error.

If a caller deliberately reserves the suggested51-unit margin, take T=898728 and rejection certificate t=T+1=898729. Its square807713815441 still satisfies `20t² >847·2V` (16154276308820 >16142903017472), so the same dyadic bound follows. This margin is conservative and unnecessary; the actual source gate remains898779.

Union over groups requires **no independence between groups**, even if their source values correlate; shared sign dependence between different coefficient outputs is harmless. Independence of fresh signs within each fixed coefficient is the conditional sampler fact used. The existing4096 joint nonce algorithm chooses the minimum accepted nonce (`fold_grind.rs:203–310,404–476`). Complete exhaustion implies nonce0 was rejected, so

$$\Pr(\text{all4096 fail})\le\Pr(\text{nonce0 fails})\le\epsilon_H.$$

Do not raise an inadequate1/8 lower bound to H, and do not assert an eps_H^4096 bound without separately establishing fresh nonce law. No retry change is needed. Do not reuse the unconditioned Rademacher theorem for the selected winner or for adversarial source extraction: winner conditioning/grinding remains owned by the indexed-ROM soundness analysis and enforced beta7.

## 4. Root D/F/H admission at each benchmark size

### 4.1 A/B and source profile invariance

Each original remains A(d512,rank1,width40,capacity427818804), B(d64,rank1,width344,capacity7), with eight original outer slices. A rank-one cutoff1538 covers40 (`sis/generated_sis_table/q128.rs:240`). B/D rank-one cutoff at(d64,7) is503438 (`:21`).

Consuming opening basis must dominate original outer base8 but stay within B capacity7: log_basis_open=3 exactly. Full-field E/D depth is43. Source inner basis13/depth10 stays frozen. Group materialization already validates source norms and compares required rounded A/B against frozen keys without recreating them (`akita-planner/src/planner.rs:106–211`). With actual carrier64/depth7, required A is the original427818804 and required B7. Only shared D/H is new.

### 4.2 Canonical D width, not ambient projection

Packing physical width is k*s=64, not d_A512. With common d_D64, canonical segment width is

$$w_{D,g}=43\cdot P1\cdot B8\cdot(64/64)=344.$$

The canonical API explicitly uses physical packed width (`akita-params/src/layout/params/precommitted.rs:123–156`). Do not apply another d_A/d_D factor8. `root_candidate_for_frozen_profile` already adds D-native widths and prices D64→D64 (`akita-planner/src/planner.rs:506–575`).

| H unique originals | Decoded endpoints | Ring shells | Shared D64 width | Rank-one cutoff503438 | Root D footprint, field elements |
|---:|---:|---:|---:|---|---:|
|1|1|8|344|fits|22016|
|2|2|16|688|fits|44032|
|3|3|24|1032|fits|66048|
|256|256|2048|88064|fits|5636096|

These are mathematical root counts/admission inequalities, not complete validated rows. Flat coordinate-wise shell fork arity is8H+1; existing group-site multiplicities are9 each. Neither count is a full extraction work price. Unique source identities are never merged because their metadata class is equal.

### 4.3 Preserve complete sliced F; check shared H separately

The original B's complete native image has

$$C_F=8\text{ slices}\cdot1\text{ rank}\cdot64=512\text{ coefficients}=8192\text{ bytes}.$$

This is exactly the maximum compression input size. The cap is inclusive (`akita-params/src/compression/chain.rs:56–74`), and canonical profile logic computes the complete sliced source (`layout/params/descriptor.rs:45–70`). Treating only one64-coefficient slice as F would discard seven original paths/image coordinates.

Q128 compression uses rank1/d16 then rank1/d8. For originals, widths are512·128/16=4096 and16·128/8=256. Both fit7077 and508 (`sis/compression.rs:24–32,76–91`). Every original keeps its own complete F chain and final8-coefficient/128-byte payload; do not concatenate H originals into one compression source.

Shared rank-one D64 image has C_H64/1024 bytes, independent of D input width. H maps have widths64·128/16=512 and256 and fit. Root H remains one chain over actual shared D, not a merger of original F chains.

Root view setup footprints are A40·512=20480, B344·64=22016, F0 4096·16=65536, F1 256·8=2048, D344H·64=22016H, H0 512·16=8192, H1 256·8=2048. Thus a static **root-only** envelope lower bound is max(65536,22016H) field elements:65536,65536,66048,5636096. H256 is90177536 bytes at16 bytes/field coefficient. Complete suffix/installed references can increase this; no complete setup envelope is guessed.

## 5. Minimal generic frozen Dense entry

The actual materializer/frozen-final constructor are already generic arithmetic; they do not intrinsically force A=B128. The present public entry has Main-onehot/all-prefix-original-RAM contracts and native-h1 frozen root work. Those restrictions block this Dense benchmark (`akita-planner/src/planner.rs:791–841`; `schedule_params/suffix_dp/candidates.rs:187–250`).

Minimal counterpart:

1. Accept H ordered exact frozen Dense nv14 profiles, authentic full-field BalancedSignedDigit contracts and retained distinct original owners. Designate the last as final; use H−1 frozen prefix groups. All query views are decoded14. No raw-view migration is needed for this separate benchmark.
2. Carry an explicit frozen consuming method/capability recipe for the root: A512/B64/D64, coefficient packing s64/k1/h8, production64(31,10), logbasis_open3/depth43, response depth7, single chunk, compressed quotient lift/canonical source. Do not silently replace it with h1/s512.
3. Validate method/shell using actual **carrier**, not productionShell(d_A). The existing artifact validator already dispatches packing by challenge_subring_dimension and trace/L2 by their genuine hooks (`akita-schedules/src/artifact.rs:754–801`). Geometry checks d_A=khs and carrier entropy (`akita-params/src/layout/subring_packing_geometry.rs:30–82`). The actual Dense K/K kernel supports generic packing; original-RAM-only native128 owner restrictions are unrelated.
4. Reuse `precommitted_group_equivalence_classes`, generic source materialization, `PreparedRootProducers`, frozen-final construction and common D summation (`planner.rs:67–211,234–375,506–575`). All prefixes have one planning class, regardless of H, but each source remains a distinct handle/claim/F path. Bind the actual root recipe in row selection/policy identity.
5. Preserve each source's selected A/B/frozen split/slices/encoding and exact F. Admit actual required beta7, B7 and D64 width/rank/image. Attach the joint Rademacher certificate above as **honest completeness metadata**, not a reduced accepted bound or a new cryptographic assumption.
6. Run the entire finite suffix search from the exact root materialization, then independent full row audit and trusted catalog installation. Never use fresh-final A/B construction or the scalar adaptation fallback. Native one-root completion/independent verification remain the existing full suffix/terminal APIs.

The proof-optimized preset's actual finite dimension domain is A{64,128,256,512,1024}, B/D{64,128}, adaptive search followed by suffix64, with root basis minimum3 and later basis range3…6 (`akita-config/src/proof_optimized/fp128.rs:14–18,111–132`; `proof_optimized.rs:15–40`). Initial root D is explicitly64 because partial physical width64 cannot be split into D128. Freezing these originals does not freeze newly generated suffix profiles.

## 6. Supported root plan versus unresolved complete suffix

**Supported mathematical root plan:** retain all H original profiles/source contracts/F paths; decoded points/values; use the exact carrier64 recipe and depth7; shared D64 rank1, width344H, bound7; shared H chain; all stated A/B/D/F/H admission/image inequalities; joint nonce0 completeness certificate eps_H above. This is a fully specified candidate root envelope, not a serialized/audited complete row.

**Existing singleton reference:** source artifact records nv14 root input16384 and output470976 (`fp128_dense.aks:128`), then its actual suffix (`:130 onward`). That is the historical singleton row. H>1 changes shared D and all root witness/consistency/compression/quotient geometry. Do not multiply470976 by H, reuse its old recursive lengths, or claim a first suffix from D width alone. Even at H1 the generalized entry must reproduce/admit its materialized row rather than treat the artifact as a new execution result.

For H1,2,3,256, executable planning must calculate:

- Complete ordered group/RHS/range/Z/E/T units, root response-digit payloads, F/H/quotient/alignment tails, exact root live next length and padded commitments. Source raw response union20480H is known; full next-witness length is not.
- Exact first suffix input, source declaration/moment and basis; every later A/B/D dimension/rank/source split/slices/basis/response depth/chunks/method/carrier/operator filter; monotone compressed/raw and relation phases.
- Direct versus setup-offloaded edges only under the selected policy, actual installed slot profiles/full source extents/canonical setup references, weighted-bit and padded-support contraction gates.
- Every shrinking edge within current finite domains/depth cap12; actual A-only terminal route, direct Linf or complete physical L2 cap, actual codec/shape and final transcript acceptance.
- Full setup/slot envelope, verifier scan capacity, wire/query/scalar/row byte limits, deterministic complete policy score/selection digest/catalog provenance, complete source-pair/matrix-view/resource ledger.

Reuse existing complete suffix DP and independent `ResolvedScheduleRow::try_new` audit; its domain must honor frozen A/B/root carrier without retaining artificial root h1 at later generated sources. `PendingNativeProver::finish` runs all edges/terminal (`akita-prover/src/protocol/prove/root.rs:151–232`); independent verifier checks complete root/suffix/transcript (`akita-verifier/src/fold/verify.rs:24–154`). A root/pending state or an existing singleton suffix is not final acceptance.

Possible blockers remain: no suffix path within chosen domains/depth/installation budget; newly generated rank growth violating mandatory compressed-image cap; unsupported filtered suffix route; installed setup/reference mismatch; integer/wire/query/artifact limits; unclosed source-cut/mixed-radius/runtime/setup certificate. Initial Dense root norm/D/F/H capacity and honest joint response acceptance are **not** blockers at H<=256. No original rekey, extra retries, source change or new assumption is an approved workaround.

## 7. Deterministic API and emitted/checkable ledger

A minimal `plan_frozen_decoded_dense_roster` request contains exact ordered originals/contracts/owners, decoded-view schema and canonical order, explicit H, immutable root carrier recipe and current policy/digests, finite suffix/installation domain. A result is a complete admitted schedule/selection/catalog provenance plus ledger, or explicit finite-domain exhaustion with actual rejecting gates. Equal metadata shares planning work, never source identity.

The executable ledger must emit:

1. All H original identities/profile bytes/contracts/F payload ownership, decoded14 points/components, H endpoint count,8H shell count, group/native order and local/global fork multiplicities.
2. Source depth/base/Linf/L1/energy, actual s64/h8 embedding/config, N20480 raw coefficients per group versus143360 response digits, exact signed gates1198372/898779, source cap1671168 and historical sizing cap478234.
3. Joint honest law: original data fixed before fresh nonce, unbiased supports, fresh41 signs/block, unfiltered route, V9529458688, integer t898780 (or separately declared conservative margin), exact dyadic inequality, eps_H and unchanged4096 nonce procedure. Keep model/prequery exceptions separate; do not condition the Rademacher calculation on the winning response.
4. Enforced weak bounds: kappa102/Delta2097151/A427818804; exact frozen width/rank/cell inequalities; B7; D width344H/rank1/bound7/cutoff503438; complete original F512 and shared H64 image counts, both map widths/cutoffs/all-coordinate binary alphabet and actual padding.
5. Actual root and full suffix/terminal geometry, setup slot/reference/footprint closure, phases/contractions/filters/codecs, row byte size, deterministic complete score/digest/trust record and exact runtime profile checks.
6. Reachable source-certificate pairs at original A: mixed Linf radius `K1_v Binf_u+K1_u Binf_v` must fit the frozen427818804 (or actual available verified norm pairing). Source F comparison must cover complete source/path alphabets, not a guessed slice. D/H comparisons are local to shared-payload siblings. Joint public setup uses the real flat prefix and per-view stride; the seed/XOF idealization and full collision-finder tree/setup/verification work remain explicitly priced. Quantum128 modeled bits are not `Adv_MSIS(T)`.

No complete suffix row, resource price, estimator advantage or model term has been fabricated. Parent owns any later code/verification work. This separate Dense contract closes the requested mathematical root and honest joint-completeness question while retaining every original source and its actual audited soundness envelope.
