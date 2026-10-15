# Candidate1: original-source weak certificates, ordinary shells, and static parameter ledger

## Scope and status

Read-only source investigation. No repository edits, Git operations, builds, tests, linters, formatters, proof/control execution, or rerun of the known Jolt caller compile defect. Numerical results below are **static arithmetic deductions**, not exercised verification. The local parsed paper `Papers/parsed/Akita.md` (indexed locally as #bfa1a6) is not officially authenticated; its extraction statements are identified as paper claims, not a newly proved source-first theorem.

The anchor is the original ordered source commitments, their A/B/F paths, complete source layouts/decoders, and selected consuming row. I did not map `prove_fold` send order. No extra end-to-end lambda128/tight128 acceptance requirement is imposed: `AKITA_INSTANCE_ACCUMULATION_FIELD_POLICY_COMPARISON.md:74–100`, especially line 87, expressly rejects importing that additional requirement. A meaningful finite, explicitly parameterized bound and its actual collision inventory remain necessary.

## 1. The three different bounds — do not conflate them

For source occurrence g scanned by fold h, let:

- b be the **response recomposition** base, delta its response digit depth;
- a be the **verifier-certified range alphabet base** for those response digits;
- m be the number of retained equal-envelope response chunks;
- omega be that group's raw shell L1 mass;
- rho = c_i' - c_i be the nonzero coordinate-fork difference.

Set S_delta = sum_{j=0}^{delta-1} b^j = (b^delta-1)/(b-1). If all relevant response digit cells are authenticated and belong to [-a/2, a/2-1], then:

1. **Maximum accepted response magnitude:** Z_inf = m*(a/2)*S_delta. The accepted signed interval is [-m*(a/2)*S_delta, m*(a/2-1)*S_delta].
2. **Weak-product certificate:** r = iota(rho)*s = z_sum' - z_sum and
   beta_weak := ||r||_inf <= Delta = m*(a-1)*S_delta.
3. **Challenge difference:** ||iota(rho)||_1 <= kappa = 2*omega. Distinct completed shell values give rho != 0; unit cancellation additionally needs the selected ring/unit certificate.

Thus beta_weak is the **diameter** of the accepted response interval, not Z_inf, not the source's honest digit bound, and not an audited A collision bucket. There is no implication ||s||_inf <= beta_weak: s = iota(rho)^(-1)*r and no short-inverse bound is supplied.

For equal profiles, cross multiplication gives an A collision with radius 2*kappa*beta_weak = 4*omega*Delta. Across differing certified profiles u,u', the correct radius is kappa_u'*beta_u + kappa_u*beta_u'. B/F source-path disagreement has its own cross-profile alphabet diameter; D/H are opening-local except when comparing the exact fold siblings whose shared opening payload is fixed.

**Exact sources:**

- Paper Eq. (112) and (113), `Papers/parsed/Akita.md:2351–2375`, distinguish accepted magnitude from response difference; Definition 10.14 at `3747–3773` certifies a unit times source, not a short source. Theorem 10.23 at `3963–3989` identifies beta with the certified response difference and derives r by subtraction.
- Mixed source comparison: paper Eqs. (193)–(196), `3598–3662`; Corollary 10.17, `3813–3827`.
- Code `akita/crates/akita-params/src/sis/norm_bound.rs::weak_binding_inf_norm` (48–55), `role_a_collision_inf_norm_for_response_difference` (75–85), `rounded_up_role_a_inf_norm` (182–235). The latter computes **4*omega*(2^(log_basis_response*delta)-1)*m**, then rounds into the audited SIS table.
- `sis/decomposition_digits.rs::balanced_digit_abs_max` (177–196) and `balanced_digit_interval_diameter` (198–224) distinguish negative reach from asymmetric interval diameter.

### Current ordinary implementation specialization

Ordinary Stage 1 sets its common range base to `1 << lp.open().digits.log_basis`: `akita-verifier/src/stages/ring_switch.rs:233–240`, `stages/stage1.rs::verify_stage1:213–225`; the grinding grammar derives the same basis at `akita-params/src/transcript_grinding/plan.rs:278–283`. Response recomposition uses that same opening base. Hence **a=b in these inspected rows**, and Delta = m*(b^delta-1). This is not a general license to use a=b for an arbitrary proposed profile whose common certified alphabet is wider than its recomposition base.

A/B admission uses these selected response parameters, not inner-source decomposition parameters: `layout/params/precommitted.rs::GroupOpenPhaseParams::admit_with_setup_natural_len:307–408`; `akita-schedules/src/audit.rs:309–365`. For frozen A, required rounded bound must be <= its declared bound. Frozen B's basis/depth remain source-owned, while the shared scanner can require the wider consuming alphabet.

`RelationRangeImagePlan::new`, `akita-types/src/proof/relation_range_image.rs:409–552`, checks the exact chunk/group partition, contiguous Z/E/T spans, tail, and common flat address geometry. `PhysicalResponsePlan::new:53–128` derives all L2 response cells from those same Z spans. `akita-cpu-backend/src/opaque/sumcheck/relation_range_image/mod.rs:1–84` explicitly treats the witness as one padded flat Boolean table and its virtual range image as MLE[w(x)(w(x)+1)], not the product of off-cube evaluations. Applying a paper full-grid extraction statement still requires the exact address/decoder/range-image bridge; an honest kernel or constructor is not an adversarial extraction theorem.

## 2. Certificate ownership: successor Cf comes from its own deeper scan

**Answer to Main's ownership question:** if Cf is the outgoing commitment of current fold h and is an incoming source of fold h+1, its weak certificate uses **h+1's** verified response envelope and shell difference, established through h+1's complete descendant suffix. It does not first require h's range proof to establish Cf's source shortness. The source is weak (unit/product certificate), not already a bounded digit table. Current h's fixed-source range argument, if the proposed theorem is admitted, can subsequently establish the requisite range predicates on that common source.

This is the locality stated by paper Definition 10.22/Theorem 10.23 (`3951–3989`): authenticate outgoing claims using descendants, obtain the local branch relation, then return weak sources of **every incoming group**. The terminal provides the base. Corollary 10.17 permits coalescing the same original source across different points/claims and opening-local D/H states, subject to all mixed A/B/F bounds. It does not say two *different* outgoing commitment handles have equal sources.

Concrete examples:

- Known original-source ordinary control: h=0 scans Main/RAM; Cf of h=0 is scanned at h=1 with response base32/depth3, so Cf's weak beta is **32767**, kappa **62**. It is not inferred from Main's h=0 depth2 beta1023 or from the honest outgoing digit magnitude16.
- Current Dense nv14 singleton artifact: h=0 scans the original Dense source with response base8/depth7, beta **2097151**. Its outgoing Cf is scanned by h=1 with response base16/depth3, beta **4095**, kappa **102**. Cf's producing parent uses range alphabet8; this does not change the deeper scan's weak certificate parameters to base8.
- At the last outgoing source, use the terminal's **directly checked raw response** certificate, not a fictitious terminal successor-range proof.

**Unknown for an unselected Candidate1 row:** frozen original source profiles alone do not determine beta. One must know the *consuming* group opening plan, common certified alphabet, delta, chunk count, raw/filtered shell, and any direct/L2 terminal bound. The provided controls determine their own numbers; they do not determine a new Candidate1 schedule or its mixed source-comparison inventory. No depth is inferred by inverting an A table anchor.

## 3. Original Main56 and original RAM26 sources

### Declarations and intrinsic geometry

`akita-config/src/full_ring.rs::CompactInner` (21–66) uses K=`Prime128OffsetA7F7`, E=K's degree-two extension, uniform128, basis5, source bound1/open bound128, `UnitOneHot { source_chunk_size: 1 }`. `CompactInner::root_inputs:79–103` supplies Main shape(16,56), receiver, RAM-minus, RAM-plus; RAM contract is `OriginalBalancedDigitPlanes`, basis5/commit bound128/open bound128. The dedicated full-ring planner `akita-planner/src/planner/full_ring_inner.rs::validate_sources:18–63` requires exactly that Main class/bound and exactly one receiver/two RAMs.

The task's “FullField” is the **full-field endpoint of the decomposition declaration**, not a current enum variant found in these sources. Literal `FullField` search was empty; alternate full-field/declaration search located `DecompositionParams::has_bounded_committed_source`, `akita-params/src/config.rs:62–83`, and `CommittedSourceContract::try_new`, `sis/committed_source.rs:130–141`. Original planes require `log_commit_bound == field_bits`; intrinsic base/depth are immutable and are checked by `original_plane_geometry:309–318` and `source_norms:184–206`.

`akita-cpu-backend/src/sources/packed_ram.rs::PackedRamSource` is **one logical polynomial**, not 26 independent polynomials. Constants at 22–24 are d128, delta26, log basis5. Original storage is [block][position][digit][coefficient], every plane has signed alphabet [-16,15], including the last. Constructor `from_signed_digits:63–117` validates the complete A plan, exact digit count, and zero physical A padding; it never reconstructs and redecomposes a field table. Six unused slots of a padded 32-slot *raw digit-query* axis are not physical stored digits. `fold_chunk:140–168` flattens M*26 only as a kernel view and retains original planes.

Admission `akita-cpu-backend/src/commitment/api.rs::ensure_sources_fit_accepted_interval:365–425` first binds field width, exact original geometry, and intrinsic source descriptor; original planes return before the canonical decoded-field reach check. This deliberately avoids pricing a noncanonical digit path by its decoded K value.

### Exact frozen original source profiles

Known-row data: `AKITA_INSTANCE_ACCUMULATION_ORDINARY_CONTROL_LEDGER.txt:2,14–16` (read untruncated).

| Source | P / variables | N live rings per polynomial | M positions / B blocks | Inner digits | A dimension/rank/width/collision capacity | B dimension/rank/width/digits/capacity |
|---|---:|---:|---:|---:|---|---|
| Main | 56 / 16 | 512 | 512 / 1 | base32, depth1 | 128 / 3 / 512 / 8126340 | 128 / 1 / 4368 / base32 depth26 / 31 |
| Each RAM endpoint | 1 / 16 | 512 | 32 / 16 | base32, depth26 | 128 / 3 / 832 / 8126340 | 128 / 1 / 1248 / base32 depth26 / 31 |

One B slice each. Main width4368=56*1*3*26; RAM width1248=1*16*3*26. Original A capacity **8126340 = 124*(2^16-1)** is a table/profile anchor. It is NOT beta and does NOT determine response depth. The source-owned inner depth26 is also NOT RAM's response depth.

### Source norms used for honest planning

`CommittedSourceContract::source_norms`, `sis/committed_source.rs:158–282`:

- Original RAM at (log basis5, depth26, d128, logical_len65536): per-plane ||s||_inf **16**, per-ring L1 **2048**, coefficient_sq_max **256**, aggregate L2 squared envelope **256*26*65536 = 436207616**, **for each endpoint**. All 26 planes are fully charged. Canonical field decomposition's last-plane residual allowance would be only 4 at basis5/depth26; it is not legal for this original noncanonical source.
- Main `UnitOneHot { chunk:1 }` at (basis5, depth1, d128, total logical length56*65536): per-ring ||s||_inf **1**, L1 **128**, coefficient_sq_max **1**, aggregate L2 squared envelope **3670016** (65536 per polynomial). `sis/onehot_source.rs::compute_canonical_source_classes:56–79` has nonzero_count=d/chunk=128; this is whole-grid Boolean charging, not a sparsity discount.

These are **producer/honest planning** norms. A proof adversary's recovered weak source does not automatically satisfy them. Original RAM paths can decode to zero while having nonzero digits, including a top-plane -16; decoded K reach is therefore insufficient even for honest original-plane admission. An extracted source may additionally be noncanonical beyond the constructor domain unless the complete source-range relation is established.

Main admission caveat: ordinary `ensure_sources_match_declared_class:281–314` normally requires exact structural chunk match. The full-ring Boolean exception `ensure_full_ring_sources_match_declared_class:325–355` accepts structurally one-hot storage with any chunk stride under the singleton whole-grid contract only for canonical full-ring E2/base32/depth1. Its comments explicitly preserve the original source descriptor/stride. Ordinary reopening/import capabilities are separate from that constructor exception; selecting a contract does not authorize reinterpretation.

## 4. Shell geometry, nonzero/unit conditions, coordinate arities

`akita-challenges/src/config.rs::SparseChallengeConfig` supplies:

- support weight w = n1+n2;
- shell L1 omega = n1+2*n2;
- shell squared L2 = n1+4*n2;
- maximum coefficient 1 or 2;
- raw support C(d,w)*C(w,n1)*2^w (`log2_support_bits:240–255`).

Distinct-position sampling is in `sampler/signed_sparse.rs:1–6,43–61`; `SparseChallenge::validate_dyn`, `challenge.rs:77–125`, checks shape/nonzero coefficients/unique positions, not all shell-count policy invariants. The actual production sampler and selected config provide the fixed counts. A nonempty support gives c != 0. **Two identical draws have difference zero and cannot be cancelled.** Different draws give nonzero rho, with ||rho||_inf <=2*c_max and ||rho||_1 <=2*omega; unit status needs the field/ring condition, not merely nonzeroness in a product ring.

| Family | Carrier dimension | (n1,n2) | w | omega | kappa=2 omega | raw squared L2 |
|---|---:|---:|---:|---:|---:|---:|
| Original compact/native shell | 128 | (24,16) | 40 | 56 | 112 | 88 |
| Ordinary production128 | 128 | (31,0) | 31 | 31 | 62 | 31 |
| Ordinary production64 | 64 | (31,10) | 41 | 51 | 102 | 71 |
| Selective-L2 64 raw shell | 64 | (31,11) | 42 | 53 | 106 | 75 |

Original shell: `akita-params/src/schedule.rs::FULL_RING_FOLD_CHALLENGE:88–94`; actual sampler `akita-transcript/src/native.rs::inner_shell:33–67` performs 40 Fisher–Yates draws, at most256 attempts each, then40 signs, with no response-dependent filtering. Ordinary ladder: `akita-challenges/src/config.rs:163–170`. Filtered L2 64 has certified strict operator threshold18 and fixed-point subset/cardinality policy at `config.rs:18–53,104–126`; its raw shell cardinality is not automatically the accepted cardinality.

Packing has **d_A=k*h*s**, carrier S_s, embedding Y -> X^(k*h), partial physical width k*s: `layout/subring_packing_geometry.rs::try_new:30–82`. Sparse mass and shell support are selected by **s**, not by d_A. Embedding preserves coefficient norms and units: paper Lemma3.2 at `963–992`.

Ordinary q128 unit premise: paper `Akita.md:1019` uses the LS18 partial-splitting shortness condition `2*c_max < q^(1/ell)/sqrt(ell)`. `akita-challenges/src/config.rs::assert_ls18_short_difference_bound:332–365` records q128 ell=4 (`413–416`), congruence q mod16=9, and exact sufficient integer check `(2*c_max)^4*4^2 < q`; maximum difference coefficient4 gives4096<q. This was read, not run. The compact/native shell has separate locally reviewed norm argument `AKITA_INSTANCE_ACCUMULATION_SMALL_CHALLENGE_UNITS.md:28–73`: q mod256=9, ord_256(q)=32, nonzero integer h with ||h||_1^128<q^32 is a unit; the recorded inequalities cover56 and112. Neither argument bounds inverse norms or establishes the source-first theorem.

**Protocol fold coordinates:** W_g=P_g*B_g, NOT shell support weight, NOT M, NOT source digit depth. Code: `transcript_grinding/plan.rs::append_fold_queries:392–414`; terminal uses B_term at `365–387`. Paper Definition10.22 needs W_total+1 coordinate-wise children; for separate group roots the grinding ledger records each group's W_g+1 multiplicity.

- Main56: W=56*1=56, local flat arity57.
- Each RAM26: W=1*16=16, local flat arity17, **not26 or416**.
- Ordinary two-RAM+Main control: W_total=88, global coordinate-wise arity89; ledger runs2–4 separately record17,17,57. Adding the compact receiver B32 contributes32 more coordinates (total120, arity121); the ordinary frozen control has no receiver.
- Current Dense nv14 singleton root: W=1*8=8, arity9.
- Terminal ordinary original-source control: B6, arity7. Current Dense singleton terminal: B7, arity8.

Each coefficient of a shell is not a separate fork coordinate. Each W coordinate is one independently drawn ring element.

## 5. Known ordinary original-source control: exact beta, max response, and sizing

The existing public entry is now **implemented**: `akita-planner/src/planner.rs::find_schedule_for_frozen_profiles:792–936`; the older design-only sections in `AKITA_INSTANCE_ACCUMULATION_FROZEN_CONTROL_PLANNER.md` are not authoritative for current API existence.

Entry conditions: Legacy, degrees1/1, Q128OffsetA7F7, current SIS table digest, default single chunk; full frozen profiles validated; Main UnitOneHot chunk1/bound1 and ordered original base32/depth26 RAM contracts; all A/B keys match consuming policy, source_norms validates intrinsic source geometry, B depth/basis is retained, original A/B dimensions remain in the allowed root domain, production shell hook required. It materializes originals without replacing A/B via `materialize_source_group_for_open_basis:106–211`, then full suffix search and `ResolvedScheduleRow::try_new` against the supplied exact profiles. This is not unconstrained root replanning or a scalar-guide fallback.

Known ledger policy `AKITA_INSTANCE_ACCUMULATION_ORDINARY_CONTROL_LEDGER.txt:1`: K/K Legacy, uniform128, basis5 fixed, single chunk, no setup offload, selective L2 disabled, exhaustive recursive split/setup search. The inspected admitted ledger has root packing k1/h1/s128; level1 packing; levels2–5 trace/reduced evaluation; complete trace/Golomb-Rice terminal.

### Root group-specific bounds from that ledger

| Original source scanned at root | response depth | accepted interval | max abs Z | weak beta=diameter | kappa | unrounded A collision |
|---|---:|---|---:|---:|---:|---:|
| Main | 2 | [-528,495] | 528 | 1023 | 62 | 126852 |
| Each RAM | 3 | [-16912,15855] | 16912 | 32767 | 62 | 4063108 |

Source: ledger14–16 records each **GroupOpeningPlan** explicitly; Main depth2 and each RAM depth3. The level0 shorthand `response_digits=2` describes the own/Main group, not the whole heterogeneous batch. D contribution is1456 for Main,416 per RAM (total2288), using `opening_d_segment_width`, `layout/params/precommitted.rs:123–157`: depth_open*B*P*(partial_width/d_D). Here all partial widths128 and open depth26.

Physical folded coefficient count used by honest sizing = m*A_input_width*d_A:

- Main: 1*512*128 = **65536**.
- Each RAM: 1*832*128 = **106496** (M*26*d_A, not M*d_A).

Honest deterministic ceilings, **not verifier weak beta**:

- Main: 56*1*min(1*128,31*1)=**1736**.
- RAM: 1*16*min(1*2048,31*16)=**7936**.

`BalancedSignedDigitFoldPolicy::universal_cap`, `sis/honest_fold_policy.rs:54–91`, selects min(deterministic envelope, ceil sqrt(2*P*B*challenge_L2_sq*source_linf^2*L)), with L the integer union-log envelope in `sis/fold_linf_cap.rs:24–57,67–152`. For the known RAM query, L=13 and t^2=3301376, ceil sqrt=**1817**; this selects depth3. Main uses the distinct `UnitOneHotFoldPolicy::exact_threshold:196–234` Chernoff search, with exact physical coefficient union and canonical source classes; the ledger supplies its resulting depth2, but the exact floating-point threshold was not executed or reverse-engineered. A deterministic1736 cap alone would not explain Main depth2 (positive reach495), hence do not substitute that cap for its actual policy.

All later nonterminal control levels have response base32/depth3, m1, omega31: Z16912, beta32767, kappa62, diagonal collision4063108. This includes the scan supplying the first Cf weak certificate.

For comparison only: original CompactInner full-ring source scanning uses deterministic56 shell mass. The code branch in `materialize_source_group_for_open_basis:143–152` gives Main cap56*1*56=3136 and RAM cap1*16*56*16=14336; `full_ring_response_depth` selects depth3 for both. Their range-envelope weak beta is32767 and kappa112, diagonal collision7339808. These are not the ordinary control's selected Main parameters and not values inferred from its A capacity.

## 6. Ordinary Dense fp128 nv14: current artifact, not nominal preset geometry

Config: `akita-config/src/proof_optimized/fp128.rs::Dense:6–18,110–128`: K=E=`Prime128OffsetA7F7`, q=2^128-2^32+22537, source class BalancedSignedDigit with full bound128. Macro `proof_optimized.rs:150–191` gives nominal decomposition log basis3, log_open_bound=None at full-field endpoint. This **does not fix selected inner basis3**. Actual A-source basis search spans3..16; opening bases3..6; adaptive A dimensions [64,128,256,512,1024], B/D [64,128], suffix64. `proof_optimized.rs:16–39` and `fp128.rs:14–18,119–128`.

The example `akita-pcs/examples/accumulation_pcs_control.rs:51–58,83–158,223–249` uses Dense, nv14, one polynomial per separate proof, original source seed0xdead_beef plus deterministic segment offset, point seed0xcafe_babe, transcript b"bench", full-field random source, resolves the trusted selected row, commits/imports, and calls actual batched prove/verify separately per segment. It is **not** Main56/RAM26, accumulation, or completed-proof aggregation.

Exact singleton row is `akita/artifacts/schedules/fp128_dense.aks:128–134`, **not** the heterogeneous row at116 containing an nv14 precommit and nv16 final source:

| Fold | input -> output live witness length | (d_A,d_B,d_D) | (N,M,B) | inner base/depth | response base/depth | shell | mode / A route |
|---|---|---|---|---|---|---|---|
| root | 16384 ->470976 | (512,64,64) | (32,4,8) | 8192 /10 | 8 /7 | (31,10) in s64 | packing k1,h8; QuotientLift; Linf |
| next1 | 470976 ->318976 | (256,64,64) | (1840,128,15) | 8 /1 | 16 /3 | (31,10) in s64 | packing k1,h4; QuotientLift; Linf |
| next2 | 318976 ->221184 | (64,64,64) | (4984,512,10) | 16 /1 | 16 /3 | (31,10) | trace; ReducedEvaluation; Linf |
| next3 | 221184 ->112128 | (64,64,64) | (3456,512,7) | 16 /1 | 32 /2 | (31,11), operator18 | trace; ReducedEvaluation; L2; raw payload |
| terminal | 112128 ->direct | d_A64 | (1752,256,7) | 32 /1 | retained fold metadata32 /3 | (31,11), operator18 | trace; raw Golomb-Rice; L2 |

Root A rank1/width40/radius427818804; B rank1/physical width344/eight slices/base8 depth43/radius7; D rank1/width344/radius7. Root source encoding canonical. All chunk counts1, no setup prefix in this singleton row.

### Exact original Dense source norms and root response numbers

`source_norms` with full bound128, selected source log basis13/depth10/d512/L16384:

- worst source digit-plane Linf **4096**, ring L1 **2097152**;
- max coefficient square **16777216**;
- first9 planes charge4096; last plane has remaining11 bits and charges1024;
- aggregate source energy envelope (9*4096^2+1024^2)*16384 = **2491081031680**.

Root folded physical coefficient count40*512=**20480**. P*B=8, shell L2 squared71. Static universal honest cap: deterministic **1671168**; integer union-log L12; t^2=**228707008512**, ceil sqrt=**478234**. Base8 depth7 positive reach898779 contains this cap; depth6 positive reach112347 does not. This is the honest selector, not the verifier's accepted response magnitude.

Root accepted interval **[-1198372,898779]**, Z_inf **1198372**, weak beta **2097151**, kappa **102**, unrounded A radius **204*2097151=427818804**, exactly the row's selected Linf radius. None of these use nominal inner base8: the actual inner source is base8192/depth10, while response/range base is8/depth7.

Next1 and next2 accepted response interval **[-2184,1911]**, Z2184, weak beta4095, kappa102, diagonal A radius835380. Thus first successor Cf's weak certificate comes from the next1 row and has beta4095, not the root's beta2097151 and not its honest source bound4.

Next3 has range-derived interval[-528,495], coefficient-product beta1023/kappa106, but its **selected A security route is L2**, with verified complete squared response cap **286397047** and operator bound18. Therefore B2_difference <=2*sqrt(286397047), K_mul2_difference<=36, actual diagonal squared collision ceiling **64*18^2*286397047=5938729166592**, covered by selected table2^43=8796093022208. Do not call its hypothetical coefficient-route radius216876 the selected SIS security certificate. The L2 proof checks complete physical response length32768, not an honest digit-moment estimate (`PhysicalResponsePlan`, `physical_l2.rs::validate_integer_soundness:252–295`).

Known historical control observations are recorded at `AKITA_RELATION_ACCUMULATION_PROTOCOL.md:307–321`: selected root d512; separately verified runs1,2,3,256; respective PCS bytes62432,124901,187340,15990351. These are prior recorded measurements, not results exercised in this investigation and not evidence that a new Candidate1 proof is sound or fast.

## 7. Terminal direct certificate special cases

There is no terminal B/D commitment or successor range scan. Paper Proposition10.20 at `3897–3921` extracts incoming A-only sources from direct accepted terminal responses and coordinate forks; its scalar trace equality is checked on an accepting child, not obtained by subtracting ring equations.

Actual ordinary terminal verifier:

- `akita-verifier/src/fold/terminal.rs:114–176` derives filtered/nonfiltered terminal shell from **this terminal's** norm route/config and reads bounded response bytes.
- `terminal/direct.rs::verify_terminal_ring_relations:131–275` validates dimensions, decodes the raw signed response, checks its exact squared norm on L2, validates challenges, and checks A and fold-evaluation ring equations.
- `TerminalFoldParams::{certified_response_linf_cap,validate_terminal_linf_cap}`, `akita-params/src/schedule.rs:406–464`: Linf cap derives from selected matrix capacity/(8*omega) and wire limit; L2 has no independently scheduled Linf cap and must encode None.
- `sis/norm_bound.rs::certified_terminal_response_linf_cap:133–182` uses the terminal group's own shell and signed-i16 wire bound32767. Matrix rank's incidental slack is not the selected certified capacity.

**Ordinary original-source control terminal** (`ORDINARY_CONTROL_LEDGER.txt:17`): d128, N1496/M256/B6, A rank3/width256/capacity4063108, shell31, response z_coords32768/e_fields768/t_fields2304, scheduled direct Linf cap **16383**. Consequently Z16383, weak beta **32766**, kappa62; diagonal collision **4062984** (<=4063108). Terminal fold metadata base32/depth3 does NOT mean direct response beta32767. Its signed accepted interval is symmetric[-16383,16383].

**Dense nv14 terminal** (`fp128_dense.aks:134`): d64/B7/A rank3/width256, z_coords16384/e448/t1344, no independent Linf cap, verified squared norm **549103459**, operator18, selected squared collision capacity2^44. Weak Euclidean product bound **2*sqrt(549103459)**, difference operator<=36, diagonal squared ceiling **11386209322624** (<17592186044416). An implied integer coefficient bound is floor sqrt(549103459)=23432, giving a valid derived coefficient difference bound46864, but this is not an independently selected Linf route. Raw integer wire representation remains signed-i16; no beta should be inferred from terminal retained fold digit-depth metadata.

**Compact full-ring terminal**, for contrast: `TerminalResponseCodec::full_ring`, `schedule.rs:127–145`, directly bounds coefficients by896*blocks; dedicated terminal B8 means Z7168, weak beta14336, kappa112, diagonal radius3211264. The codec additionally checks unique signed K interpretation (bound<q/2). Again no shortness assertion about the recovered terminal source itself follows.

## 8. Canonical installed setup-source reference is a different certificate

Paper Eqs. (191)–(192) at `3611–3623` add a canonical preprocessing reference occurrence with rho_ref=1 and r_ref=s_static. Corollary10.17 accepts this without an evaluation point or D/H opening. The mixed A radius is **beta_weak + kappa_weak*||s_static||_inf**, where s_static means the reference's actual **A-input source blocks** (including its selected inner decomposition), not an arbitrary renaming of the full-field setup table. Canonical source-path alphabets must also enter the mixed B/F inventory. Numerical reference radius is unknown without that frozen reference profile and its source-block envelope.

Source-first coalescing cannot replace this by two protocol-profile weak certificates and omit the installed canonical reference. Setup opening authenticity says the opening source corresponds to the trusted installed setup, not merely to an attacker-consistent auxiliary table. Paper Lemma9.1 at `3174–3176` uses precisely that comparison before obtaining direct-scan equality.

Actual runtime registry identity is `SetupPrefixSlotId { natural_len, commitment_profile }`, derived by `GroupOpenPhaseParams::slot_id`, `layout/params/precommitted.rs:265–278`; `n_prefix` and geometry checks cover the complete committed power-of-two prefix (`schedule/profiles.rs:274–310`). `akita-verifier/src/fold/suffix.rs::suffix_commitment_payloads:29–56` reads its public commitment from verifier setup, rejecting missing slots; the proof cannot select a replacement payload. Private material is backend-owned and imported against setup (`akita-cpu-backend/src/commitment/setup_prefix.rs:29–66,127–153`).

**Important source-first/code distinction:** current preprocessing `akita-cpu-backend/src/setup/setup_prefix.rs::commit_validated_setup_prefix:98–144` copies **the actual shared matrix coefficients [0..n_prefix]**, including coefficients beyond natural_len, and commits that full prefix. Paper Eq.(167), `Akita.md:3115–3125`, instead defines a canonical zero-padded source beyond N_active. Code `GroupCommitPhaseParams::validate_setup_prefix_geometry` comments explicitly say natural_len is only support of the later weight. Therefore a code-specific canonical reference is the actual full-prefix S source; do **not** assert a canonical zero suffix for the source just because the paper's padded S_flat has one. Weight-zero outside the active support is a separate property. The fixed-source static contraction must use the installed code convention or an explicitly proved bridge.

Neither selected ordinary control here uses setup offload, so this special case introduces no extra setup group into those control rows.

## 9. Descriptor / profile / inventory limits

- `GroupCommitPhaseParams`, `schedule/profiles.rs:82–102`, contains version4, polynomial layout, N/M/B, slice count, and complete A/B digits+matrix identity. Encoding at201–214 includes these. It does **not** store source class, source producer IDs, shell, response depth, D plan, or chunking. `CommittedSourceEncoding` is separately authenticated by the scheduled consuming/production path; a canonical standalone profile constructor rejects tensor source encoding (`113–124`).
- `GroupOpeningPlan`, `layout/params/precommitted.rs:159–211`, owns method, shell, opening basis/open depth, response depth. `PrecommittedGroupAdmissionPolicy` owns consuming chunk count, not the frozen profile. `CommittedGroupParams::append_descriptor_bytes`, `layout/params/descriptor.rs:96–151`, and `FoldSchedule` descriptor bind method/source encoding, exact matrices/lengths, shells, digits, chunking, and setup edge; terminal shape/norm metadata is also bound.
- `ScheduleLookupKey` stores final shape only and ordered full precommitted profiles (`schedule/profiles.rs:360–371`). `CommittedGroupBatchProfile:579–619` stores every exact profile including final. `schedule_row_digest`, `schedule_selection.rs:42–76`, hashes ordered full profiles followed by expanded schedule; its comment explicitly excludes provider/source identity from this digest. Equal profile bytes are not permission to coalesce two different handles.
- `FoldSchedule::sis_occurrences`, `schedule/sis_occurrences.rs:62–139`, enumerates selected A/B/D and compression matrix **occurrences** with each selected Linf/L2 table bound; terminal adds A only. It is useful for actual dimensions and selected table capacities. Its exposed record does not construct the paper's extra canonical-reference occurrence or all cross-profile comparison-pair maxima. A complete source-first collision inventory must supply those comparison classes/mixed radii rather than equating a diagonal occurrence report with Eqs.(193)–(196).

## 10. Hand-off gates (no new lambda128 hard gate)

1. For the actual Candidate1 suffix, retain exact per-occurrence (a,b,delta,m,shell/filter,kappa) and terminal raw bound; use beta=accepted response diameter. Known control numbers above are not substitute row parameters.
2. Certify the selected full-grid/address/decoder/response-cell bridge for every strong node, including padding and original signed/noncanonical RAM planes. Producer source_norms and honest caps cannot discharge adversarial source bounds.
3. Coalescing uses same original handle+shape+A/B/F projection and every mixed comparison radius; canonical installed setup references are included where offload is selected. Opening-local D/H and points are not source identity.
4. Unit cancellation requires nonzero shell difference in the exact carrier and its approved unital embedding; do not bound inverse norms or infer raw support equals filtered accepted support. Supply finite sampler and total extraction work/failure accounting.
5. Distinguish the current full-prefix setup source from the paper's zero-padded setup source; use the actually installed reference and exact active-support weight semantics.
6. No checks were exercised here. Main owns any one final verification pass authorized by its assignment; this report requires no build/control rerun and does not request the known broken Jolt caller compile check.
