# Candidate1 — actual continued Jolt fixed-original-source frontend certificate

## 1. Result and precise scope

This is a source/math investigation, with no source edits, builds, tests, lint, formatting, proof generation, emulator execution, or verification runs. The bounds are mathematical deductions, not measurements or exercised cryptographic extraction.

**Closed selected certificate.** Transparent `prepare_continued_instance`, E=K=`Prime128OffsetA7F7`, CycleMajor, original compact Main56/RAM65536, full-public approved program, no advice commitment objects, no field-inline; T=4096, committed address alphabet16, RAM address arity16. Include the actual `verify_segment_main` source checks and every deferred original leaf. This is a probabilistic fixed-source **K-valued algebraic continued relation**, not deterministic acceptance implying a valid source or emulator witness.

Let b be the actual bytecode address arity (the three-bytecode-chunk admission has9<=b<=12), `last` the last-segment indicator, M a bound on relevant fresh conditional field atom masses. Then:

|Component|Bound|
|---|---|
|Actual eight stage batches, two uni-skips, scalar folds|`epsilon_reduce <=(2059+4b)M`|
|Conservative unweighted fixed-source reference conversion|`R_ref <=(245+b+16*last)M`|
|Actual Main support and first/last PC source checks|`epsilon_source <=67M`|
|Selected algebraic FE theorem with every original leaf true|`epsilon_alg <=min(1,(2371+5b+16*last)M)`|
|Optional existing 118-to-56 Main normalization|add45M|

All-position maxima are2447M, or2492M with normalization. These local bounds assume complete original-source leaf truth/coherence; they are not Candidate1's full PCS/ROM/SIS/runtime admission. Nonuniform source probes can use degree-times-maxatom, but retained shell forks still need their exact conditional raw-answer law.

**Stronger bridge not established and false without extra predicates.** The field relation does not imply arbitrary fixed-original-source RV64 byte-memory/emulator/tape execution. §10 gives signed advice-1, executed LD-1, inconsistent AdviceLen outputs, and final forward-JAL stopping counterexamples. The existing full-public/no-advice native receipt rejects advice/committed-program auxiliary objects; the advice examples are not alleged attacks on that restricted receipt profile. Root integration must name its application relation rather than silently inherit stronger raw/SDK predicates.

Durable detailed companion reports:

- `AKITA_RELATION_ACCUMULATION_FRONTEND_DEGREE_LEDGER.md`
- `AKITA_RELATION_ACCUMULATION_CONTINUED_LEAF_DERIVATION.md`
- `AKITA_RELATION_ACCUMULATION_CONTINUED_FIXED_SOURCE_THEOREM.md`
- `AKITA_RELATION_ACCUMULATION_CONTINUED_BOUNDARY_BRIDGE.md`
- `AKITA_RELATION_ACCUMULATION_CONTINUED_BOUNDARY_ADDENDUM.md`

Citation shorthand: `verifier/...` below means `jolt/crates/jolt-verifier/src/...`; `protocols/...` means `jolt/crates/jolt-claims/src/protocols/...`. Other paths are fully written or anchored locally by a full path.

## 2. Actual preparation and source timing

`verifier/segment/frontend.rs:203-253` validates RAM contract, approved program and predecessor; enforces trace length/CycleMajor; absorbs context; calls `validate_and_seed_transcript`; checks preprocessing digest/public IO; rejects ZK; sets native entry bytecode index; removes initialization consumers only after first. It runs stages1,2,3,4,5,6a,6b,7 (`:258-349`), native stage8 preparation (`:350-370`), and separately returns RAM endpoint/program-boundary groups (`:373-381`).

**Main already prebound before stage1.** `verifier/verifier.rs:417-452,804-900` validates and absorbs native Main, supplied U/T advice, optional original FieldRdInc and retained direct program commitments during seeding. Producer stage0 commits before stage protocols (`jolt/crates/jolt-prover/src/akita/stage0.rs:168-187,198-251,284-350`). No extra stage1 Main commitment is required; stage8 assembly is not fixation.

`prepare_native_instance` (`verifier/verifier.rs:231-372`) is ordinary whole-run preparation, not by itself the continued boundary theorem. `ContinuedFrontPending` (`verifier/segment/frontend.rs:28-58`) returns pending native groups, two original RAM groups and program-boundary groups; it is not an acceptance certificate. Main source support/PC checks are separately invoked by the application.

The producer binds context/public register endpoints and retains original Main rows, explicitly using the full-field reference backend (`jolt/crates/jolt-prover/src/akita/segment.rs:78-133,155-176,321-357`). `jolt/crates/jolt-witness/src/backend/trace/continued.rs:10-30,83-150,167-258` builds K-valued prefix states and memory/register/input/product evaluations, stuttering through padding. It is not emulator replay; honest byte construction cannot be assumed as the inverse of arbitrary extracted sources.

## 3. Source assignments, geometry and adapters

Fix literal original A/B/F algebraic explanations S, complete profiles/views/source roles, retained setup/preprocessing and context before frontend randomness. This is **source-fixed**, not merely C-fixed.

Define physical Main columns X_c(t,a), ordered InstructionRa32, BalancedIncDigit16, BalancedIncCarry1, BytecodeRa3, RamRa4 (`protocols/jolt/lattice/packing.rs:73-109`; `protocols/jolt/lattice/strategy.rs:48-87`). Each has65536 physical coefficients; there are56 physical columns, not64. Eight padded selector slots are structural holes, not stored source cells.

For original RAM C define `M_C(a)=sum_{ell=0}^{25}32^ell s_C(a,ell)` in K. The profile is d128*positions32*blocks16=65536 exactly (`akita/crates/akita-config/src/full_ring.rs:79-110`), so there is no actual physical RAM padding. All26 planes including full plane25 participate. Six unused raw digit slots are absent addresses (`akita/crates/akita-types/src/full_ring/mod.rs:66-113,254-292`), not extracted-source zero predicates. The decoder is affine with constant0, exact adjoint `eq(u,a)*32^ell` (`akita/crates/akita-types/src/full_ring/query.rs:407-455`). Logical decoded queries are not arbitrary raw-plane queries.

**Coordinate conventions must not be conflated.**

- Main logical `(address||cycle)` becomes actual CycleMajor `(cycle||address)` (`protocols/jolt/lattice/strategy.rs:110-144`), then native backend reverses the point for low-first ordinary evaluation (`jolt/crates/jolt-akita/src/native_batching.rs:350-402`).
- Dense advice/direct-program coefficients are already bit-reversed by `jolt_to_akita_index` (`jolt/crates/jolt-akita/src/adapters.rs:1307-1390`); auxiliary points are unchanged (`native_batching.rs:350-402`). Logical coefficient at MSB index i is `p_raw[bitreverse(i)]`, so logical MSB point u equals raw low-first point u. Reversing every auxiliary point again is incorrect.
- Original RAM uses low packing/coefficient, then carrier, positions, blocks (`akita/crates/akita-types/src/subring_coefficient_packing.rs:39-134`). Additional physical preparation axes are pinned by zero prefix, not certified zero by metadata.
- Ordinary paths use `BasisMode::Lagrange`, not Monomial (`native_batching.rs:219,239,665,745`; `akita/crates/akita-params/src/layout/opening_point.rs:50-104`).

`akita/crates/akita-cpu-backend/src/opaque/ordinary_projection.rs:145-202` authenticates original setup/descriptor/sourceId/raw view/profile/producer contract, re-executes and compares full payload/retained material, retains original source Arc, and derives only ordinary quotient-lift material. Receiving scope is K/K,k1h1,d128 coefficient packing (`:31-83`), not raw-plane/E2/EOR authority. `akita/crates/akita-cpu-backend/src/sources/packed_ram.rs:383-434` Horner-decodes all26 planes for logical partials while retaining original planes for folds. Receiving operation IDs can change; original semantic authority/A/B/F do not.

Repeated public occurrences require one deeper-certified original explanation under the surrounding source-comparison theorem. Import-once/clone provenance is not adversarial source coherence. Never coalesce from pending parent range, decoded equality, or C fixation.

## 4. Deterministic algebraic witness reconstruction

Define all true tables on their cube, then extend multilinearly. An off-cube product of MLEs is not automatically the MLE of a cube product.

### Main and increments

For nonRAM: `R_c(a,t)=X_c(a,t)` for a!=0 and `R_c(0,t)=1-sum_{a!=0}X_c(a,t)`. RAM retains `R_c=X_c`. Thus nonRAM evaluation is

`R_c(r,t)=eq(r,0)+sum_a(eq(r,a)-eq(r,0))X_c(a,t)`.

Set `d(a)=a-16*highbit(a)`, d(0)=0, and

`FusedInc(t)=sum_{j=0}^{15}16^j sum_a d(a)X_Dj(a,t)+2^64 sum_a d(a)X_carry(a,t)`.

`RamInc=Store*FusedInc`, `RdInc=(1-Store)*FusedInc` are enforced by actual full-public bytecode RAF legs, not free increment sources (`protocols/jolt/lattice/relations/digit_zero.rs:16-23,167-281`; `protocols/jolt/lattice/relations/read_raf.rs:1-24,108-236`). Actual carry alphabet[-8,7] does not imply honest{-1,0,1} or a64-bit delta bound.

Booleanity is `eq*sum_c gamma^(2c)(R_c^2-R_c)` (`protocols/jolt/geometry/booleanity.rs:39-58`; `protocols/jolt/lattice/relations/booleanity.rs:93-181`). Structural nonRAM mass1 and16<p give exactly one hot. RAM has Boolean common mass H(t), every chunk sum H, hence zero/one-hot with common activity. Virtual selectors are products of these actual committed chunks.

### Program, lookup, CPU rows

B(b,t) is the cube product of three bytecode chunks. Architectural PC, immediate, flags, register selectors, table flags and RAF flag are `sum_b P_f(b)B(b,t)` from approved original program. Full-public P_f is retained preprocessing; committed P_f needs retained canonical-reference coherence plus §5 obligations.

Instruction selector multiplies32 four-bit chunks grouped into eight virtual chunks over128-bit address. Lookup operands/output use actual public tables/RAF routing. Current Stage5 has gamma^3 canonical identity-RAF penalty with upper64 all-ones U (`verifier/stages/stage5/instruction_read_raf.rs:191-230`; `protocols/jolt/geometry/instruction.rs:23-73`). Identity-RAF U=0 gives address `<2^128-2^64<p` outside priced bad events. It closes modulo-p **address** aliases, not all RAM/LD value ranges.

InstructionInput uses approved register/PC/immediate pullbacks. Product/ShouldBranch/ShouldJump are cube products, `ShouldJump=Jump*(1-NextIsNoop)`. All19 RV64 guarded equations and three product equations apply (`jolt/crates/jolt-claims/src/protocols/composed/r1cs/rv64.rs:200-440`), including inert padding. Next columns use the actual successor-corrected shifts, not whole-run shifts silently substituted.

### Register/RAM states and optional field mode

For public incoming B:

`RegVal(r,t)=B(r)+sum_{s<t}RdWa(r,s)RdInc(s)`;

Rs1/Rs2/RdWrite are its selected pre-read/post-write values.

`MemVal(a,t)=M_minus(a)+sum_{s<t}RamRa(a,s)RamInc(s)`;

`M_plus(a)=M_minus(a)+sum_{s<ell}RamRa(a,s)RamInc(s)`.

Read/write checks select these prefixes/deltas. Keep actual dummy-coordinate factors/windows;2-powers are invertible in K.

Field-inline adds sixteen field-register endpoints, original FieldRdInc, selector/prefix recurrences, field product/inverse-product and row equations. Nonzero inverse guard fixes the inverse. FieldAdviceLimb is a field quotient, not integer canonicality (`jolt/crates/jolt-claims/src/protocols/composed/r1cs/field_constraints.rs:162-173`). Its larger roster is not priced by the selected no-field numeric coefficient.

Temporary uniskip/staged BytecodeVal/address/Booleanity/program-reduction carriers are deterministic reductions, not free K sources. Actual virtual dispatch covers served variants (`jolt/crates/jolt-witness/src/backend/trace/oracle.rs:24-147,159-274`; `jolt/crates/jolt-claims/src/protocols/jolt/ids.rs:363-407`); unserved Rd/InstructionRaf/RamValInit do not silently substitute for consumers.

**Aliases:** preserve value and exact point. LookupOut/left/right use product-remainder references; UPC/register aliases preserve shift/input references; outer/product references differ. Generated value comparisons occur in `verifier/stages/relations.rs:505-550`; equal-point bytecode aliases reject unequal values before omission (`verifier/stages/stage6b/verify.rs:639-665`). Polynomial ID alone does not permit merging different points.

## 5. Complete original leaf roster and consuming-side coverage

|Obligation|True fixed-source leaf|Producer/consumer|
|---|---|---|
|Main56 stage8|Every `v_c=MLE(X_c)(r_cycle,r_address)`|`verifier/stages/stage8/akita.rs:525-566` visits all canonical IDs/checks point and arity; `:598-689` takes five Main families from Stage7|
|Main support|56 original X_c values at actual source SC point|`verifier/segment_source.rs:308-409`, separate from stage8|
|First/last PC|3 bytecode-column values at fixed cycle0/ell-1 and sampled address|`segment_source.rs:136-229,370-388`|
|RAM minus/plus|`v_minus=MLE(M_minus)(u)`, `v_plus=MLE(M_plus)(u)` at shared Stage2 address|`verifier/segment.rs:325-355`; `segment/frontend.rs:373-381`|
|U/T advice, first consumer only|Original dense used-slot restriction at Stage4 init point|`stage8/akita.rs:252-270,352-368,665-683`; first-only passing `segment/frontend.rs:352-368`|
|Direct bytecode/first-only image|Every precommitted final leaf, carried two-phase point, original prefix reduction|`verifier/stages/stage8/precommitted.rs:67-165`; `stage8/akita.rs:368-466`; bytecode retained, image skipped only without init consumer|
|Program boundaries|Start/last-active architectural PC; nonlast successor PC,virtual,first-in-sequence,noop|`segment/frontend.rs:79-184`:2 leaves last,6 nonlast, exact original lane/row/zero-prefix points|
|Optional FieldRdInc|Stage6b rd_inc at exact FieldIncLayout point|`stage8/akita.rs:272-292,425-438`|
|Optional normalization|All56 original columns at new common logical point|`verifier/segment_normalization.rs:236-266`|

Native duplicate IDs reject; advice needs both commitment and leaf; exact arity/all declared components are checked (`akita/crates/akita-types/src/opening_claims.rs:111-153`). Prefix binds semantic points/evaluations before selector (`jolt/crates/jolt-openings/src/prefix.rs:207-267`). Singleton equation `P(s,u)=eq(s,0)v`, general used slots `sum_i eq(s,i)v_i`. False v yields nonzero multilinear residual at Boolean used slot, regardless of unused coefficients. Charge `sum_j mu_selector,j`, not deterministic inversion of eq. Different-arity packing has actual zero-prefix scaling (`protocols/jolt/lattice/packing.rs:310-354`).

Two-phase precommitted reductions preserve cycle/address schedules/inactive windows and terminal skip `2^-(cycle_gap+address_gap)` (only cycle gap when ending there); carried coordinates are permuted, not refreshed (`protocols/jolt/geometry/claim_reductions/precommitted.rs:274-309,316-389,478-493`). Chunk weights are equality weights on dropped bits/lane grids, not powers (`protocols/jolt/geometry/claim_reductions/bytecode.rs:224-311`). Field-inline+committed-program explicitly rejects (`stage8/akita.rs:368-374`).

Existing compact native receipt calls full continued preparation, **rejects** auxiliary/program-boundary groups, then verifies support/normalization (`verifier/receipt/native.rs:268-310`). It does not silently drop them. A generic Candidate1 caller retaining only Main/two RAM scalars would omit real returned leaves. No complete generic Candidate1 caller was established here: design must append every selected row or use a declared sound reduction. `PendingNativeStage8::verify` (`stage8/akita.rs:55-113`) separately discharges PCS; preparation alone is not acceptance.

## 6. Current Main source predicates and normalization

Bare semantic reconstruction cannot see physical nonRAM X_c(0,t): coefficient `eq(r,0)-eq(r,0)=0`. Setting it to2 is a kernel of bare prepare. Current `verify_segment_main` closes it.

Mask `Z_c(t,a)=1[t>=ell]+1[t<ell,a=0,c nonRAM]`; predicate all masked original coefficients zero. Initial identity:

`0=sum_c alpha_c sum_x eq(tau,x)Z_c(x)X_c(x)`.

Terminal weight is `MLE_x[eq(tau,x)Z_c(x)](r)`, **not** `eq(tau,r)MLE(Z_c)(r)` (`verifier/segment_source.rs:115-132,161-229,308-409`). Source/plan precede anchor and independent alpha. Violated coefficient produces nonzero anchor polynomial:16 anchor masses,one pivot,16 degree2 rounds.

First/last moments `sum_a a X_Bc(t,a)=digit_c(declared expanded PC)` at0/ell-1 use separate coefficient vectors/four degree2 rounds each. Categorical meaning requires one-hot/Booleanity; MSB-chunk digits/actual point conversion are retained.

`epsilon_source <=sum_{i=1}^{16}mu_anchor,i+B_zero+2sum_{i=1}^{16}mu_zero,i+B_first+2sum_{i=1}^{4}mu_first,i+B_last+2sum_{i=1}^{4}mu_last,i <=67M`.

Semantic Booleanity+mask implies all actual selected Main **field coefficients** Boolean, omitted/inactive rows zero, structural selector holes zero. This closes selected decoded Main BooleanWholeGrid, not arbitrary integer-lift shortness/canonical gadget decomposition or another geometry's physical padding.

Existing normalization fixes118 pointed scalars:56 native+56 support+3 first+3 last; raw arity22=16logical+6selector; independent alpha,22 degree2 rounds (`verifier/segment_normalization.rs:22-27,66-137,142-195,293-340`).

`epsilon_norm <=B_norm+2sum_{i=1}^{22}mu_norm,i <=45M`.

All56 original terminal values remain; scalar `sum_{c<56}eq(selector,c)v_c` checks the actual original normalization endpoint. New normalized commitments break anchoring. Directly accumulating118 inputs avoids this reduction but is a different declared flow.

## 7. Actual boundary/state/counter/I/O bridge

`verifier/segment.rs:23-68` defines pre-first/post-last registers, first/last active architectural/expanded PC and separate successor. Validation (`:94-140`) requires N power-of-two>=4096,1<=ell<N,128K registers,x0zero,optional sixteen field registers,first registers zero,last padded successor `(0,0,false,false,true)`,legal interior virtual boundaries. Host nonwrapping `after=before+ell`,first before0 (`:281-307`) counts active expanded rows including noops, not automatically ISA steps/tape consumption.

Predecessor checks (`segment.rs:79-92,310-323`) enforce next index,total/N,registers,successor-to-start PCs/flags,counter,exact original RAM handle,run/program/preprocessing/normalized IO/panic. Whole archive separately checks expected external run/program/IO/index/total/final expanded-row count (`verifier/receipt/archive.rs:229-268`). Standalone preparation cannot assert the full external execution statement.

Registers at address x/reference y obey

`V_Reg(x,y)+gamma Fhat(x)-(1+gamma)Bhat(x)=sum_i(Lt(y,i)+gamma)RdInc(i)RdWa(x,i)`.

Inputs precede independent endpoint gamma (`verifier/stages/stage5/verify.rs:135-159`; `verifier/stages/stage5/registers_val_evaluation.rs:62-91,179-218`). Constant/gamma separation yields prefix/final recurrences; Stage4 read/write selects CPU values. Field registers analogous. This is deterministic assembly **after true unweighted identities**, not after one sampled acceptance.

RAM has the same endpoint relation (`verifier/stages/stage4/ram_val_check.rs:157-179,300-327,631-682`): alias points checked,endpoints bound before gamma,first init compared (`verifier/stages/stage4/verify.rs:184-206,238-264`). Init is public+approved image+original advice; cleared exactly after first.

Entry `eq(start expanded PC,k)eq(0,t)` is a real bytecode leg (`verifier/stages/stage6a/verify.rs:58-71`; `verifier/stages/stage6b/batch.rs:210-218`; `protocols/jolt/geometry/bytecode.rs:354-380`). Source moments identify first/last categories. Actual successor correction is

`eq(A,ell-1)(u_s+gamma p_s+gamma^2 v_s+gamma^3 first_s)+gamma^4 eq(P,ell-1)(1-noop_s)`

with distinct outer A/product P (`verifier/stages/stage3/spartan_shift.rs:94-124,153-187,225-237`). Its subtraction plus true padding shifts identifies successor at ell-1. Bare prepare without support/moments does not establish the full active/counter boundary.

Last-only output:

`0=sum_a eq(z,a)IoMask(a)(M_plus(a)-ValIO(a))`

(`verifier/stages/stage2/verify.rs:159-200`; `protocols/jolt/relations/ram/output_check.rs:88-146`). Public IO interval covers input/output/device zeros,panic,termination; nonpanic termination1 (`jolt/crates/jolt-program/src/preprocess/public_io.rs:20-59`). Outputs trim trailing zeros. Actual indexed device behavior/sticky panic is not an unrestricted journal (`jolt/common/src/jolt_device.rs:143-207`); equality alone is not complete emulator-device/stop adequacy.

**Assembly lemma:** true unweighted identities+source predicates yield concrete segment K-prefix states; metadata are first/last rows; corrected shifts give declared successor; exact context/source equalities concatenate; initialization/output occur once. Every transition component has a real relation or deterministic check. This closes the algebraic witness bridge.

## 8. Explicit local error-degree ledger

Define `mu_c=sup_x Pr[X_c=x | reached prefix immediately before c and earlier coordinates of this draw]`; B is a coefficient-pivot bound. Never condition on eventual acceptance. Nonzero degree d costs d*mu_c; multivariate testing sums coordinate-degree masses in actual order. These may be nonuniform. Independent vectors use last-nonzero pivot, not length or powers; an atomic vector needs its coordinate-conditional law, not invented external-query independence.

### Sumchecks and uni-skips

Generated batch absorbs every input, draws independent coefficients, forms `sum lambda_i2^(N-n_i)S_i`, checks actual terminal windows (`jolt/crates/jolt-verifier-derive/src/lib.rs:488-615,665-698`; `verifier/stages/relations.rs:355-374`). It enforces common maximum transmitted degree in **every common round**, not individual active degrees.

|Stage|Selected instances|N,D|D*N|
|---|---|---|---:|
|1|OuterRemainder|13,3|39|
|2|RamReadWrite;ProductRemainder;InstructionClaimReduction;RamRafEvaluation;last-only RamOutputCheck|28,3|84|
|3|SpartanShift;InstructionInput;RegistersClaimReduction|12,3|36|
|4|RegistersReadWrite;RamValCheck|19,3|57|
|5|InstructionReadRaf;RamRaClaimReduction;continued RegistersValEvaluation|140,10|1400|
|6a|BytecodeReadRafAddress;BooleanityAddress|b,4|4b|
|6b|BytecodeReadRafCycle;BooleanityCycle;RamHammingBooleanity;RamRaVirtualization;InstructionRaVirtualization|12,5|60|
|7|DigitZeroClaimReduction|4,2|8|

Roster `verifier/segment_emissions/roster.rs:399-431,439-550,555-660,663-779,814-913`. Total1684+4b. Stage2 address shares r2[12..28],product/instruction offset16; Stage4 registers r4[0..12]/r4[12..19],RAM value r4[7..19]; Stage5 address128+cycle12,reducers share tail12; Stage6a Booleanity tail4. Reuse is not fresh sampling.

Two separate centered uni-skips (`verifier/stages/uniskip.rs:37-91,122-182`):outer domain10 degree27,product domain3 degree6. Add `27mu_U1+6mu_U2`. Eight independent batch pivots add8M. Round+uni coefficient1717+4b.

### Scalar folds

|Draw/fold|Max exponent|
|---|---:|
|Product tau_high Lagrange fold3 lanes|2|
|Stage2 RAM RW;instruction claim|1,4|
|Stage3 shift;input;register reduction|4,1,2|
|Stage4 register RW;RAM value|2,1|
|Stage5 RAF including canonical penalty;RAM RA;independent register endpoint|3,2,1|
|Stage6a bytecode outer|11|
|Stage6a beta1..5|15,3,8,2,56|
|Stage6a raw Booleanity even powers|110|
|Stage6b instruction virtualization|7|
|Stage7 digit-zero including final decode|99|

Sum334. Formula anchors:

- `protocols/jolt/geometry/claim_reductions/instruction.rs:16-32`
- `protocols/jolt/relations/spartan/shift.rs:70-123`
- `protocols/jolt/relations/instruction/input_virtualization.rs:73-109`
- **`jolt/crates/jolt-claims/src/twist/claim_reductions.rs:25-99`**
- **`jolt/crates/jolt-claims/src/twist/memory_checking.rs:41-64`**
- `protocols/jolt/relations/ram/val_check.rs:136-167`
- `protocols/jolt/relations/ram/ra_claim_reduction.rs:83-100`
- `protocols/jolt/relations/registers/val_evaluation.rs:168-191`
- `protocols/jolt/geometry/bytecode.rs:25-108,650-734`
- `protocols/jolt/lattice/relations/read_raf.rs:5-15,83-132`
- `protocols/jolt/geometry/booleanity.rs:41-58`
- `protocols/jolt/relations/instruction/ra_virtualization.rs:76-95`
- `protocols/jolt/lattice/relations/digit_zero.rs:148-161,186-281`
- current RAF guard `verifier/stages/stage5/instruction_read_raf.rs:191-218`.

Beta5 arity57=RdWa+RAFflag+55tables. Booleanity exponent2*(56-1),not55. Digit-zero decode `2*(32+3)+3*4+(16+1)=99`,not98. Carried gammas priced once.

`epsilon_reduce <=27mu_U1+6mu_U2+sum_s D_s sum_r mu_s,r+sum_s B_s+sum_g e_g mass_g <=(2059+4b)M`.

### Unweighted fixed-source reference bottleneck

Choose a fixed first violated source truth-table identity independently of randomness. Equality-reference coordinates are multilinear; centered outer row-reference degree9 is the exception. Sufficient site set:

|Sites|Coefficient|
|---|---:|
|Outer tau13 multilinear coordinates+centered row degree9|22|
|Stage1 cycle|12|
|Stage2|28|
|Stage3|12|
|Stage4|19|
|Stage5|140|
|Stage6a|b|
|Stage6b|12|
|Last output address|16*last|

Hence `R_ref <=(245+b+16*last)M`. Powers cancellations/false intermediates are separately priced. No union over all cube rows or imaginary fresh derived references. This deliberately broad sufficient bound is not optimal; detailed terminal/reference derivation is in the companion degree ledger §5.

Main stage8 adds no selector in selected path. Optional advice/program prefixes add actual selector terms, optional members can change common N,D and exponents. Field-inline changes outer/product lane counts/geometry/folds. General ledger is the site formula above with actual roster,uni-degrees `3(m_O-1)`,`3(m_P-1)`,real fold/reference degrees.2447M is not claimed for unselected optional branches; no numeric field-inline closure without its complete selected roster.

## 9. Fixed-source probabilistic theorem, sampling and old-state composition

Let S be fixed original algebraic assignment,h validated context/preprocessing,w_h(S) the deterministic cube lift. Define `R_K_cont(S,h)` by unweighted one-hot/Booleanity/virtualization/decode/approved rows; physical support/PC moments; K-register/RAM recurrences/init/output; corrected entry/shifts; deterministic context/counter/source adjacency.

For any causal prover strategy under §8 laws:

`Pr[checked FE/source protocols accept AND all complete original deferred leaves are true on S AND not R_K_cont(S,h)] <=min(1,epsilon_reduce+R_ref+epsilon_source [+epsilon_norm if used])`.

Optional original-prefix/branch protocols add their actual terms. This is probability over frontend randomness for **fixed S**, not accepting-transcript implication/extraction/source validity.

Proof: true cube tables are defined before draws; follow actual backward claim/alias DAG. Exact checks have no probability. False independent batches/powers folds survive only pivot/degree-e roots;2-powers cannot erase errors. A false running sumcheck claim cannot agree with true partial-sum polynomial at both Boolean endpoints; absorbed prover polynomial has common capD and restores a true child only at discrepancy roots. Compressed reconstruction fixes the polynomial before challenge (`jolt/crates/jolt-sumcheck/src/verifier.rs:90-151`), not an extra soundness escape. Fixed-source reference/prefix residuals are nonzero and priced. Union actual bad events; outside them identities hold and §7 assembles witness.

Without original leaf truth, `g(X)=c+(S-2c)X` has Boolean sumS for arbitraryS; a free late terminal g(r) passes. Preparation is not authentication. Candidate1 must supply literal A/B/F leaf truth/coherence through correctly matched deeper certificates/source probes, not C fixation/decoded equality/bounded products. This local proof does not invoke an unauthenticated expanded Akita theorem.

**Sampler:** p=2^128-4294944759. Legacy Blake2b alias (`jolt/crates/jolt-prover/src/akita/preprocessing.rs:26-27`) squeezes16 bytes (`jolt/crates/jolt-transcript/src/digest.rs:190-201`),reduces modp (`jolt/crates/jolt-field/src/solinas/fp128.rs:1128-1137,1169-1172`). Fresh ideal uniform128-bit bytes give low4294944759 values mass2/2^128,others1/2^128. Thus M=2/2^128,not1/p.

Fresh ideal BitTape: D1 inner at most two canonical128-bit candidates (`jolt/crates/jolt-transcript/src/d1/frontend.rs:320-341`) gives each accepted value subprobability `(1-delta^2)/p`; outer installed reservedL (`jolt/crates/jolt-transcript/src/d1/outer.rs:366-389,705-720,759-790`) gives `(1-delta^L)/p`,delta=(2^128-p)/2^128. Exhaustion rejects; conditional on success uniform. Concrete seed-expanded SHAKE is not established independent ideal bits. No global reset/fork law, arbitrary-algorithm Adv(time),accepting-tree interpolation numerator or tight128 admission is inferred.

**Old accumulator:** preserve every old coefficient/query and original identity even if old state false. Final strong flat opening relation restricted to old entries gives their old equations, without assuming old validity/honest explanations. Any weighted-query reduction to this flat relation has its own cancellation/sumcheck error in Candidate1 composition. Repeated originals,especially adjacent RAM endpoints,must share deeper-certified explanations; equal decoded values do not coalesce distinct originals. Historical execution needs retained/replayed historical FE/boundaries and sum of their epsilons plus expected run/program/IO/index/total/final-count checks. Old scalar equations alone do not prove historical VM execution. Whole extractor runtime/ROP and global ROM composition remain separate priced obligations.

## 10. Advice variants and precise missing byte/tape/stop predicates

**Memory advice:** untrusted is existential original dense words in first init; trusted uses externally supplied original commitment. Same local K equation; trusted canonical-reference provenance may be an explicit premise, not a property of a naked commitment. Honest conversion is padded little-endian u64 (`jolt/crates/jolt-prover/src/akita/witness.rs:345-388`; `jolt/common/src/advice.rs:28-43`). `JoltDenseBounded` (`jolt/crates/jolt-akita/src/configs.rs:245-255`) explicitly admits centered signed[-2^64,2^64-1] (`akita/crates/akita-config/src/proof_optimized/fp128.rs:63-95`),not unsigned. Subsequent segments carry original RAM state,not repeated initialization semantics.

**Arithmetic advice:** VirtualAdvice/Load/Len range-check existential lookup outputs/write-to-rd; they are not automatically memory-advice source,tape or host compute outputs. Field-native advice has actual field equations/quotients,not integer canonicality.

### Unsigned fixed-source word counterexamples

Choose original permitted advice word-1,matching original init/outgoing RAM,never accessed. All algebraic identities and final IO may be true; original balanced plane0=-1,others0 is legal. Since p-1>2^64-1,no u64 word equals this fixed decoded coefficient. This chooses different original commitments before randomness,not same-C tampering/honest constructor behavior.

Executed LD: `jolt/crates/jolt-lookup-tables/src/instructions/riscv/ld.rs:1-16` has no lookup; `jolt/crates/jolt-riscv/src/instructions/i/ld.rs:3-8` Load without WriteLookupOutputToRD. Actual `rv64.rs:231-252` (full path `jolt/crates/jolt-claims/src/protocols/composed/r1cs/rv64.rs`) constrains address and `RamRead=RamWrite=RdWrite`,not value range. Loading-1 into zero nonzero register uses valid delta-1,RamInc0; SD copies it (`jolt/crates/jolt-lookup-tables/src/instructions/riscv/sd.rs:1-16`; `rv64.rs:254-256`); overwrite0 uses+1. Canonical RAF ranges lookup addresses,not loaded data. Arithmetic RangeCheck does range its outputs (`jolt/crates/jolt-lookup-tables/src/tables/range_check.rs:13-29`; `rv64.rs:306-311`).

A selected fixed-source byte-memory statement needs decoded membership `embed_K([0,2^64))` or equality to certified canonical originals wherever it claims word memory,plus selected exact-length/tail-zero constraints. Balanced RAM or centered dense bounds do not repair it.

### Tape/cursor/length/compute counterexample

VirtualAdviceLoad/Len flags and lookup only range-check supplied outputs (`jolt/crates/jolt-riscv/src/instructions/virt/advice_load.rs:3-8`,`advice_len.rs:3-8`; `jolt/crates/jolt-lookup-tables/src/instructions/virt/advice_load.rs:6-25`,`advice_len.rs:6-25`). Two consecutive AdviceLen outputs1,2 without consumption satisfy individual field relations, but no actual tape:remaining=len-read_position is unchanged (`jolt/tracer/src/emulator/cpu.rs:17-91`; `jolt/tracer/src/instruction/virtual_advice_len.rs:19-23`).

If tape semantics selected: require one tape identity/nonwrapping cursor,width/EOF/little-endian read relation,each length=remaining,consistent cursor/unread suffix across cuts. Host compute-advice needs its computation relation; SDK compute build writes while proved build reads existential result (`jolt/jolt-sdk/macros/src/lib.rs:1262-1275`). Pure arithmetic existential advice intentionally selects less. Length-free full-word answers can fit a private concatenated tape,but that does not prove length queries/SDK policy.

### Padded final successor is not emulator halt

After termination1,a valid final forward JAL with nonzero rd/correct linkPC+4 may satisfy padded successor while emulator executes target. Final NextIsNoop1 zeros ShouldJump/disables target equation; Jump disables sequential-PC equation (`rv64.rs:316-342,392-394`). JAL target still range-checks (`jolt/crates/jolt-lookup-tables/src/instructions/riscv/jal.rs:6-35`),so not an address alias. Actual tracer stops on PC stall (trace mode also no-row trap),not declared padded successor (`jolt/tracer/src/lib.rs:110-120,201-208,327-336`). SDK clean stop writes1 then `j .` (`jolt/jolt-sdk/macros/src/lib.rs:881-887`). Need selected stopping-row predicate or approved-program theorem that native convention entails desired end; IO word alone cannot prove arbitrary-program emulator stopping. The current convention intentionally states something weaker.

## 11. Retained program/setup and old raw-plane predicates

Retain approved full bytecode/image and admitted setup,public pullbacks/entry/boundary validation. Preprocessing checks supported rows/store-rd disjointness (`jolt/crates/jolt-program/src/preprocess/bytecode.rs:31-55`),expanded sequence mapping (`:173-210`); arbitrary lane tables are not certified programs. Committed mode needs same-original-source canonical-reference coherence,every direct layout/trace order and2/6 boundary leaves. Hash/commitment alone is not program well-formedness. Import preserves exact installed descriptor/owner/profiles (§3). This is not global setup/SIS proof.

Old native explicitly selects RAM BalancedBase32 and Main BooleanWholeGrid (`verifier/receipt/native.rs:108-132`). Their statements cannot silently disappear:

- Main decoded Boolean/one-hot/support is needed. Actual semantic stages **plus** `verify_segment_main` close it on selected full geometry; bare prepare does not. Keep obligations/error.
- Original RAM balanced planes are unnecessary for chosen decoded K-prefix relation. At one address s0=32,s1=-1 decodes0 but violates range: decoder-kernel example,not same-C attack. Honest all26-plane ownership,legal noncanonical handling,plane25 alphabet,response/norm contracts remain (`akita/crates/akita-cpu-backend/src/sources/packed_ram.rs:21-118`),not extracted-source range proof.
- If old stronger raw range remains part of target,ordinary algebraic extraction/ordinary final PCS does not close it:need same-original-source raw-range certificate. Choosing only R_K_cont is a relation change,not inheritance. Unit division of bounded products cannot establish original shortness.
- Balanced planes are not byte repair:legal-1 already refutes unsigned words. Decoded byte correctness need not demand canonical raw representation if actual word/operation predicates hold.
- Honest carry{-1,0,1},narrow delta,padding/canonical encoding are separate policies unless application needs them. Current carry[-8,7] suffices for selected K recurrence,but not a blanket byte/ISA adequacy claim.

## 12. Handoff

The selected actual continued frontend is certified as a probabilistic fixed-original-source algebraic FE,with complete selected leaves,true equations,real boundary assembly and explicit local error ledger. Original Main/RAM stay on literal A/B/F; optional advice/program/field branches have explicit consuming interfaces.

The unrestricted stronger byte/tape/emulator bridge is genuinely unfulfilled,with exact missing predicates/counterexamples above. Root must name its chosen relation:retaining raw range or claiming SDK/byte/stop needs additional certificates;selecting algebraic semantics must disclose exclusions. No executed verification,numeric/global crypto admission or concrete extractor implementation is claimed.
