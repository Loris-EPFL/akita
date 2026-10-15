# Candidate1 actual continued frontend: fixed-source randomized error ledger

## 1. Selected result and scope

Read-only static investigation of the live sources. No edit, build, test, lint, formatter, Git/fetch, proof generation, emulator run, or rerun of a reported failure was performed. All numerical budgets below are mathematical deductions from inspected formulas, not measured security or exercised verification.

**Selected path:** transparent continued frontend, evaluation/challenge field E=K=Prime128OffsetA7F7, CycleMajor, full-public program, no advice objects, no field-inline, t=log2(T)=12, a=log2(RAM_K)=16, w=committed chunk bits=4. Main has 56 logical columns: instruction32, increment digits16, carry1, bytecode3, RAM4. Write b=log2(padded public bytecode size); the selected three-bytecode-chunk admission fixes 9<=b<=12, but does not determine b more precisely. Instruction virtual chunks v=8, committed chunks per virtual q=4, full instruction address bits=128. These follow from `jolt/crates/jolt-verifier/src/segment_emissions/roster.rs:399-431`, `jolt/crates/jolt-prover/src/config.rs:229-242`, and `jolt/crates/jolt-prover/tests/support/compact_segments.rs:67-97`. Do not infer a particular b by running the fixture.

Let M denote a common upper bound on all relevant **fresh conditional field point masses**, with the qualifications in §2. A conservative local error budget is:

| Certificate component | Common-M coefficient |
|---|---:|
| Actual stages1,2,3,4,5,6a,6b,7, including both uni-skips, all powers folds, all independent batch coefficients | **2059+4b** |
| Conservative conversion of these local weighted identities to their fixed-source unweighted algebraic identities | **245+b+16·last** |
| Actual `verify_segment_main` physical zeros and first/last active expanded-PC moments | **67** |
| Total continued algebraic source certificate, with every original deferred leaf true | **2371+5b+16·last** |
| Optional existing Main normalization, 118 claims -> 22-round degree2 reduction | **45** |
| Total including that optional normalization | **2416+5b+16·last** |

Here `last` is 1 exactly for the last segment. The all-b, all-position upper bounds are respectively **2447 M** and **2492 M**. These are deliberately conservative, not asserted optimal. The site-resolved formulas in §§3-7 are the actual useful ledger; replacing every distinct mass with M loses information. The 245+b reference charge is a sufficient all-reference bottleneck bound, rather than a claim that every one of those coordinates is necessary for every failed predicate.

**Important distinction:** 2059+4b is the closed reduction/local-protocol budget. It alone is NOT a per-row relation theorem. The source-reference charge, physical-zero/PC checks, original-leaf truth, deterministic source lift, and boundary predicates are separate. A stage-only `sum rounds*max_degree` leaves both powers cancellation and semantic-reference errors unpriced.

The certificate concerns the algebraic continued relation of the decoded original sources. It does NOT prove SDK emulator/tape policy, original RAM plane membership, canonical/balanced source ownership, or 64-bit increment range. It is not deterministic `acceptance => relation`, a numerical Candidate1 admission, a source-comparison/collision theorem, or a Fiat–Shamir reduction.

Root maps read: `AKITA_RELATION_ACCUMULATION_FRONTEND_COMPOSITION.md:36-89` fixes primitive sources before frontend randomness and requires extraction of the complete original deferred roster, not PCS-only endpoints. `AKITA_RELATION_ACCUMULATION_RAW_RAM_BRIDGE.md:12-106` distinguishes decoded RAM semantics from honest producer raw-plane policy. Main's native commitment is already absorbed before stage1; no new commitment is proposed.

## 2. Exact probability convention and sampler laws

For a draw coordinate c, condition on the complete reached prefix immediately before the relevant fresh draw (including bound statements, source commitments, earlier answers, and the prover message that must precede that draw). Let

`mu_c = sup_x Pr[X_c=x | that reached prefix and earlier coordinates of this draw]`.

For scalar coefficient draws use `nu_c` with the same convention. These may depend on the reached prefix; a deterministic ledger can use the supremum over allowed reached prefixes, or integrate prefix-wise conditional bounds. Never condition these masses on eventual acceptance. For a fresh nonzero polynomial in the currently drawn scalar with degree d, the local root bound is `min(1,d*mu_c)`. Multivariate testing uses the sum of per-coordinate degrees times these conditional masses, by leading-coefficient induction in the actual coordinate order. No independence or uniformity is invented for a generic Transcript implementation.

Independent batch coefficients are **separate coefficient draws**, not gamma powers. If the fixed error vector at the start of the coefficient draws is nonzero, use its **last nonzero coefficient**: all later error coefficients are zero, and at this pivot there is at most one rejecting root with the earlier prefix fixed. Thus one pivot mass, not m masses and not `(m-1)` times a powers-sampler mass, suffices. For an atomically returned vector the same argument requires the actual joint law's coordinate-conditional masses. Treating every coordinate of one external vector entry as a separately fresh external oracle query is wrong.

Actual field: `jolt/crates/jolt-transcript/src/d1/mod.rs:18-24` fixes

`p = 340282366920938463463374607427473266697 = 2^128 - 4294944759`.

* D1 inner scalar: `d1/frontend.rs:320-341` consumes up to **two** 128-bit canonical rejection candidates on the native inner cursor; it does not reserve a 256-bit slot for every accepted limb. Under independent fresh ideal bits, with delta=(2^128-p)/2^128, each accepted x has subprobability `(1-delta^2)/p`, and the law conditional on this sampling success is uniform K. An exhaustion rejects, so it does not itself increase false acceptance.
* D1 outer: `d1/outer.rs:366-389,705-720,759-790` reserves exactly the installed number L of candidates for each coordinate. Under fresh ideal bits, each x has subprobability `(1-delta^L)/p`, and is uniform conditional on that coordinate's success. The count/caps and joint reserved layout are installed data, not guessed.
* Legacy digest: `jolt/crates/jolt-transcript/src/digest.rs:192-201` squeezes 16 bytes for raw and scalar challenges. `jolt-field/src/algebra.rs:348-350` and `solinas/fp128.rs:1131-1141,1154-1171` reduce these modulo p. Under fresh uniform 128-bit bytes, values below 4294944759 have mass **2/2^128**, other values mass **1/2^128**. Thus M=2/2^128 is valid in that ideal-byte model, NOT 1/p. The deterministic concrete hash alone does not establish a fresh-ideal-bit law.
* `jolt/crates/jolt-transcript/src/transcript.rs:246-275` expands one scalar deterministically to `[1,gamma,...]`. Each power is NOT a fresh sample. `legacy.rs:354-377` instead implements a CoefficientVector by distinct sequential scalar squeezes.

Finite sampling failure/conditioning and any global ROM root selection losses belong to the surrounding compiler/composition theorem. The fixed-source local ledger is not multiplied by an unsubstantiated Q or interpolation factor here.

## 3. Sumcheck and independent batching: exact implementation

`jolt/crates/jolt-sumcheck/src/verifier.rs:53-95` checks round count, transmitted degree, domain sum, absorbs the message, then draws r and evaluates the message at r. `:98-157` implements compressed Boolean proofs. Compressed encoding reconstructs the linear coefficient from the running claim, so round-sum equality is automatic; non-linear tampering is ruled out only through the terminal oracle equation. This is explicitly documented in `jolt-sumcheck/tests/soundness.rs:407-430`. For zero variables there is no round interaction; the caller must check the terminal oracle (`verifier.rs:39-44`).

`jolt/crates/jolt-verifier-derive/src/lib.rs:488-615` performs the batch head:

1. evaluate member input claims in declaration order;
2. absorb **all** present input claims;
3. draw one independent coefficient lambda_i per present member;
4. form `S_batch = sum_i lambda_i * 2^(N-n_i) * S_i`.

`jolt-verifier/src/stages/relations.rs:355-374` calls `challenge_scalar` once per declared present member. The padded factor is invertible in odd characteristic, so it cannot erase a nonzero member error. Offset does not change `2^(N-n_i)`. The generated terminal check uses member output expressions at the appropriate active-window subpoints (`jolt-verifier-derive/src/lib.rs:665-698`). Alias validation is hard equality, not another probabilistic fold (`stages/relations.rs:507-550`).

For each batch s, let `B_s` be the supremum of the pivot coefficient mass over present members and allowed prefixes. Its reduction budget is

`B_s + D_s * sum_{r=0}^{N_s-1} mu_{s,r}`.

Use the **batch maximum transmitted degree D_s in every common round**. A shorter member's standalone degree is NOT an enforced degree restriction on the aggregate proof. Inactive rounds and windows affect the true terminal expression, not the transmitted global degree cap. This is why summing standalone member bounds or charging only active low-degree slots understates this verifier's adversarial message allowance.

### Selected batches, canonical member order, dimensions, windows

| Stage | Present instances in order | Member rounds/degrees | N,D |
|---|---|---|---|
| 1 | OuterRemainder | (t+1,3) | 13,3 |
| 2 | RamReadWrite; ProductRemainder; InstructionClaimReduction; RamRafEvaluation; optional last-only RamOutputCheck | (t+a,3); (t,3); (t,2); (a,2); (a,3) | 28,3 |
| 3 | SpartanShift; InstructionInput; RegistersClaimReduction | (t,2); (t,3); (t,2) | 12,3 |
| 4 | RegistersReadWrite; RamValCheck | (t+7,3); (t,3) | 19,3 |
| 5 | InstructionReadRaf; RamRaClaimReduction; continued RegistersValEvaluation | (128+t,v+2); (t,2); (t,3) | 140,10 |
| 6a | BytecodeReadRafAddress; BooleanityAddress | (b,3+1); (w,3) | b,4 |
| 6b | lattice BytecodeReadRafCycle; lattice BooleanityCycle; RamHammingBooleanity; RamRaVirtualization; InstructionRaVirtualization | (t,3+2); (t,3); (t,3); (t,4+1); (t,q+1) | 12,5 |
| 7 | lattice DigitZeroClaimReduction | (w,2) | 4,2 |

Selected roster: `segment_emissions/roster.rs:439-550,555-660,663-779,814-913`. Actual optional slots: `stages/stage6b/outputs.rs:85-119`. Full-public selected execution has no advice cycle reductions, no IncClaimReduction, no bytecode reduction, no program-image reduction.

The common round total is

`3*13 + 3*28 + 3*12 + 3*19 + 10*140 + 4*b + 5*12 + 2*4 = 1684+4b`.

Most members tail-align. Stage2 is exceptional: RAM read/write is active through all 28 rounds; ProductRemainder and instruction reduction tail-align at offset16; RAM RAF and last output-check use phase offset12 and the same 16-coordinate **address window r2[12..28]**, not an independently sampled point. `stages/stage2/mod.rs:24-57` derives this offset; `geometry/dimensions.rs:135-193` gives read/write/address point permutation. Register read/write stage4 has cycle r4[0..12] and address r4[12..19], while RamValCheck uses the tail r4[7..19]. Stage5 binds instruction address first128, cycle last12; both t-round reducers use this same last12-coordinate window. Stage6a Booleanity uses its tail w coordinates. Stage6b all five selected members use the common t coordinates. Stage7 reverses its w round challenges, appends the already chosen stage6b cycle point, and then stage8 rotates `(address||cycle)` to original CycleMajor `(cycle||address)` (`geometry/claim_reductions/hamming_weight.rs:26-45`; `lattice/strategy.rs:108-150`).

### Two real uni-skips

They are not Boolean rounds and are not batch members.

* Stage1: centered integer domain m_O=ceil(19/2)=10; admitted degree `3*(m_O-1)=27`; zero input; terminal is the outer uni-skip opening, then absorbed before the remainder coefficient.
* Stage2: m_P=3 product lanes; admitted degree `3*(m_P-1)=6`; input is the Lagrange-weighted fold of the three stage1 product claims; terminal is the product uni-skip opening, then absorbed before member/batch draws.

`stages/uniskip.rs:37-91,122-182`; `protocols/composed/r1cs/mod.rs:54-65`; `protocols/composed/geometry.rs:5-19`. The true outer summand is centered-kernel times the two centered interpolations of A·z and B·z, integrated over stream/cycle; the product summand is centered-kernel times left and right lane interpolations, weighted by cycle equality (`jolt-kernels/src/reference/spartan_outer.rs:157-201`; `reference/spartan_product.rs:1-27,173-247`). Their budget is `27*mu_U1 + 6*mu_U2`, NOT the degree of the later remainder.

Thus the selected round-plus-uni-skip budget has common-M coefficient **1717+4b**, and the independent-batch contribution is **8**.

## 4. Every powers/virtualization/fusion fold

This table prices actual scalar polynomial cancellation, not coordinate round degree. Use the listed exponent e times that scalar's conditional mass. Raw vs coefficient draw provenance must remain as installed.

| Draw/fold | Selected maximum exponent e | Reason/source |
|---|---:|---|
| Product tau_high | 2 | Lagrange fold of 3 lanes, raw draw; `stages/uniskip.rs:112-119`, `stage2/product_uniskip.rs:1-40`; do not call it a coefficient power vector |
| Stage2 RAM RW gamma | 1 | read/write two legs; `relations/ram/read_write_checking.rs:54-85` |
| Stage2 instruction claim gamma | 4 | 5 claims; `geometry/claim_reductions/instruction.rs:16-32` |
| Stage3 shift gamma | 4 | 5 shifted legs incl next-noop; `relations/spartan/shift.rs:70-123` |
| Stage3 input gamma | 1 | right/left routing; `relations/instruction/input_virtualization.rs:73-109` |
| Stage3 register reduction gamma | 2 | 3 value claims, common value-reduction fold; `twist/claim_reductions.rs:25-99` |
| Stage4 register RW gamma | 2 | rd,rs1,rs2; `twist/memory_checking.rs:41-64` |
| Stage4 RAM value gamma | 1 | incoming/current and outgoing identities; `relations/ram/val_check.rs:136-167` |
| Stage5 instruction RAF gamma | **3** | output and 2 operand legs PLUS canonical-address penalty; `stages/stage5/instruction_read_raf.rs:191-218` |
| Stage5 RAM RA gamma | 2 | RAF/RW/value 3 claims; `relations/ram/ra_claim_reduction.rs:83-100` |
| Stage5 continued register endpoint gamma | 1 | outgoing-endpoint leg, mode-specific independent scalar; `relations/registers/val_evaluation.rs:168-191` |
| Stage6a bytecode outer gamma | 11 | 5 base stages + 4 fused-inc stages + two PC/shift RAF legs + entry = powers0..11; `geometry/bytecode.rs:81-108`, `lattice/relations/read_raf.rs:5-15,83-132` |
| Stage6a beta1..beta5 | 15,3,8,2,56 | arities16,4,9,3,57; C=14 circuit flags, L=55 lookup tables; `geometry/bytecode.rs:25-47,650-734`, `jolt-riscv/src/flags.rs:24-47`, `jolt-lookup-tables/src/tables/mod.rs:139-194` |
| Stage6a Booleanity raw gamma | **110** | weights are gamma^(2i), i=0..55, NOT powers0..55; `geometry/booleanity.rs:41-58`, `lattice/relations/booleanity.rs:159-181` |
| Stage6b instruction virtualization gamma | 7 | 8 virtual instruction chunks; `relations/instruction/ra_virtualization.rs:76-95` |
| Stage7 digit-zero gamma | **99** | 2*(32+3)+3*4+(16+1)=99; decode term is at exponent99, not98; `lattice/relations/digit_zero.rs:148-161,186-231,240-281` |

The exponent sum is

`2+(1+4)+(4+1+2)+(2+1)+(3+2+1)+(11+15+3+8+2+56+110)+7+99 = 334`.

Bytecode beta5's arity is **57**, not 55; it includes RdWa and InstructionRafFlag before the 55 table flags. Booleanity gamma is raw (`stage6a/booleanity.rs:106-126`); it is reused in stage6b and must be charged once, not sampled again. Bytecode's six gammas are likewise carried from 6a to 6b (`stage6a/verify.rs:79-88`, `stage6b/verify.rs:94-98`). Stage6b instruction gamma is fresh; inc_gamma and optional eta are absent in this selected branch (`stage6b/batch.rs:145-177`).

Total reduction budget, site-resolved:

`epsilon_reduce <= 27 mu_U1 + 6 mu_U2 + sum_s D_s sum_r mu_s,r + sum_s B_s + sum_g e_g mass_g`.

With a common M this is **(2059+4b) M**.

## 5. True terminal equations, original-source interpretation, and source-reference price

Let P_c(a,j) be the decoded original Main column, written logically as address/cycle but evaluated against original CycleMajor storage by the verified point permutation. No assumption P_c is an honest bit table is allowed initially. RAM endpoints are the fixed decoded original polynomials V_minus(k), V_plus(k), not their signed-plane encodings.

For instruction/bytecode/increment columns, digit zero is reconstructed:

`R_c(0,j)=1-sum_{a!=0} P_c(a,j)`, `R_c(a,j)=P_c(a,j)` for nonzero a.

RAM retains all rows, including zero. This reconstruction is a polynomial definition, not an assumed source range predicate (`lattice/relations/digit_zero.rs:1-11,186-214`).

**Booleanity terminal:** at a fixed full address/cycle point, the verifier uses `eq(reference,point)*sum_c gamma^(2c)*(R_c(point)^2-R_c(point))`. This is a product of off-cube evaluations; the source predicate is the Boolean-cube residual table. Summing its restriction over the hypercube is what connects it to the sampled reference. The two phases preserve the same expression through the intermediate, and stage7 reconstructs omitted rows using `eq(r,a)-eq(r,0)`, including the input subtraction at digit zero. It is not correct to claim Booleanity of stored nonzero cells alone proves one-hotness: reconstructed digit zero's Boolean residual and constant mass1 are essential. See `geometry/booleanity.rs:41-58`, `stage6b/booleanity.rs:134-178`, and `lattice/relations/digit_zero.rs:186-281`.

**RAM Hamming:** stage6b checks `eq(tau_cycle,j)*(H(j)^2-H(j))`. Stage7 folds a separate mass H, Booleanity, and virtualization leg for each RAM chunk. Thus the required fixed-source RAM predicate is each chunk Boolean with a COMMON per-cycle mass H(j) in {0,1}; merely Boolean rows would allow multiple addresses. The equality of different chunk mass polynomials is tested at the common stage6b cycle point. Sources: `relations/ram/hamming_booleanity.rs:75-90`, `stage6b/ram_hamming_booleanity.rs:43-88`, `lattice/relations/digit_zero.rs:190-208,247-255`.

**Virtualization:** instruction terminal is an EqCycle-weighted powers sum over 8 products of 4 committed chunks. RAM terminal is EqCycle times the product of its 4 chunks. These are degree5 in cycle, not linear MLEs in cycle of pre-multiplied off-cube claims. Stage7 supplies every committed chunk evaluation against Main. `relations/instruction/ra_virtualization.rs:67-95`; `relations/ram/ra_virtualization.rs:61-86`; `stage6b/ram_ra_virtualization.rs:103-136`.

**Bytecode:** the 6a summand is the address MLE of the cycle-integrated staged trace/table discrepancies plus PC/shift and first-entry discrepancies. 6b evaluates each public Val_s(r_address), the product of 3 bytecode chunks, and the consuming stage's cycle equality. The four extra increment stages multiply by FusedInc: selected cycle degree is 3 chunks+FusedInc+EqCycle=5. The public store selector is substituted from the selected public bytecode row, relying on preprocessing's per-row store/rd disjointness. There is no independent Store opening or extra IncVirtualization in this branch. `lattice/relations/read_raf.rs:1-23,114-132,168-182`; `geometry/bytecode.rs:25-108,131-161,480-604`; reference polynomial explanation `jolt-kernels/src/reference/bytecode_read_raf.rs:1-32`.

**Instruction RAF/canonicality:** its terminal uses the product of 8 virtual chunks, table flag MLEs, an Eq-weighted public table MLE, operand MLEs, identity MLE, and the RAF flag. Actual code has a gamma^3 penalty `RafFlag*U(k)` where U is the product of the leading64 address bits. No gamma^3 input term exists. Consequently, outside the priced cancellation/reference/reduction errors, identity-RAF source rows must have U(k)=0. Since `U(k)=0 => k<2^128-2^64<p`, the modulo-p identity leg has a canonical representable address. The old alias concern is CLOSED by current code; omitting this leg would allow the local RAF identity to confuse k and k+p. Exact live sources: `geometry/instruction.rs:23-73`, `stage5/instruction_read_raf.rs:191-218`. It is still not a byte-oriented advice tape theorem.

**Registers and RAM:** actual read/write terminal products are EqCycle·access·Val, with Inc on the write leg (`twist/memory_checking.rs:41-64`; `relations/ram/read_write_checking.rs:58-85`). Current RAM value terminal is `(Lt_cycle+gamma)*Inc*Ra`, and its input is `Val + gamma*V_plus - (1+gamma)*V_minus`, with public initialization independently compared on the first segment (`relations/ram/val_check.rs:136-167`; `stages/stage4/ram_val_check.rs:157-179,631-704`). Continued registers use the same transition form at the original seven-coordinate register address, with an independent endpoint gamma (`relations/registers/val_evaluation.rs:168-191`; `stage5/registers_val_evaluation.rs:61-76,202-213`). RAM RAF uses `(8*Identity(address)+lowest_address)*Ra` (`stage2/ram_raf_evaluation.rs:135-153`). Last-only output-check uses `eq(reference_address,address)*IoMask(address)*(V_plus(address)-ValIo(address))` (`relations/ram/output_check.rs:121-132`).

**Spartan:** outer terminal is `tau_kernel*(Az_constant+sum_i Az_weight_i z_i)*(Bz_constant+sum_i Bz_weight_i z_i)`. The later source reductions must interpret those z_i through the deterministic original-source algebraic lift; they are not free virtual tables. Its row-stream construction is explicit at `protocols/composed/r1cs/mod.rs:142-176,313-337`. Shift must retain the segment's supplied successor correction, not silently use whole-run padding (`stages/stage3/spartan_shift.rs:44-75,207-239`).

### Conservative unweighted-source reference bottleneck

Once powers cancellations are separately charged, pick a fixed first violated unweighted source identity. Its truth-test is a nonzero polynomial in the earlier reference points. Cube equality reference coordinates appear multilinearly, including range masks/shift weights/identity operands; the exception is the centered outer row reference, degree m_O-1=9. The deterministic source lift is defined before these draws. Leading-coefficient induction therefore permits a common conservative site set covering whichever predicate is chosen:

* outer tau: 13 coordinates degree1, final row-reference coordinate degree9: cost22;
* stage1 remainder cycle reference: its 12 cycle coordinates (stream coordinate not reused): cost12;
* stage2 common point: all28 coordinates, covering RAM address, cycle/product/instruction points and incoming/outgoing/first-init comparisons: cost28;
* stage3 point: all12: cost12;
* stage4 point: all19, covering continued register address and value/read-write cycle references: cost19;
* stage5 point: all140, covering all instruction address slices and cycle, including Booleanity's derived reference and instruction virtualization: cost140;
* stage6a address point: all b: cost b;
* stage6b point: all12, including the RAM chunk common-mass comparison and increment decode cycle: cost12;
* last-only public output address reference: all16: cost16·last.

Let R be the weighted sum of these actual sites' conditional masses (tau_high weight9, others1). Then **R <= (245+b+16·last)M**. This is a sufficient broad bottleneck charge: points used only in a reduction may be unnecessary for a particular chosen residual, but are harmless to include. Do NOT add a further per-predicate union over all cube rows or every reused reference. A fixed violated identity can be chosen independently of randomness; the powers ledger handles selecting individual components inside a random fold, and the local reduction ledger handles all false intermediate claims. Reused references are not newly sampled again.

The conclusion is conditional on a deterministic algebraic lift with these terminal equations and on the complete original-leaf truth premise. Honest Rust witness construction alone is NOT a proof that adversarial original sources lie in its constructor domain. For arbitrary decoded sources use algebraic reconstruction (prefix recurrences, reconstructed one-hots, public row tables), not `JoltTraceRow` short-word decoding as an assumed inverse. The native continued backend demonstrates that memory values are full K and recurrence-based (`jolt-witness/src/backend/trace/continued.rs:83-142,169-192`), but is an honest producer implementation rather than the extractor for this lift.

## 6. Actual Main source check: 67, not an interpolation numerator

`jolt/crates/jolt-verifier/src/segment_source.rs:133-158,166-211,313-397` gives the exact source checks:

1. draw a raw anchor h of length n=t+w=16;
2. draw an independent coefficient vector alpha of length56 for PhysicalZeros;
3. degree2 sumcheck for n=16 rounds, target0; terminal is `sum_c alpha_c*P_c(r)*(inactive_weight(h,r)+omitted_zero_weight_c(h,r))`;
4. independent coefficient vector of length3 and w=4 degree2 rounds for FirstPc;
5. another independent vector of length3 and w=4 degree2 rounds for LastPc.

The restricted equality-mask MLE at `:166-211` must not be replaced by a product of two unrestricted off-cube MLEs. Non-RAM source columns omit address zero; RAM does not (`:128-133`). First/last moments use actual expanded-PC chunk digits and active cycles0 and active_rows-1, not a guessed halting cycle (`:141-163`). With Booleanity/digit-zero closure already present, a moment pins the chosen digit because 0..15 injects into K. Without that closure it is only a field moment.

Site-resolved budget:

`epsilon_source <= sum_i mu_anchor_i + B_zero_vector + 2 sum_i mu_zero_round_i + B_first_vector + 2 sum_i mu_first_round_i + B_last_vector + 2 sum_i mu_last_round_i`.

Common-M coefficient = `16+1+32+1+8+1+8=67`. No factor56 is needed for independently sampled coefficients. No `3^P`, `m3^n`, or accepting-tree interpolation numerator belongs in this local root bound. Returned 56+3+3=62 claims remain original Main obligations, not PCS acceptance.

## 7. Optional Main normalization and native leaf boundary

`jolt-verifier/src/segment_normalization.rs:25-29,147-181,198-219,300-343` fixes 56 stage8 claims plus 62 source-check claims =118, independent coefficients alpha[118], original source1 view (56 polynomials, padded selector64, raw arity22), a 22-round degree2 sumcheck, and 56 ordinary original-column values at the new logical low16 point. Terminal comparison is the actual normalization arithmetic endpoint evaluated on the decoded original raw view. All56 returned values must be authenticated against the ORIGINAL Main commitment; inventing a fresh normalized source would break the theorem's anchoring premise.

`epsilon_normalization <= B_alpha_vector + 2 sum_{r=0}^{21} mu_norm,r`, hence coefficient45, not118+44. This is optional transport of already bound claims, not a new source-range proof.

Native stage8 selected path itself adds **no prefix selector draw** for Main: it collects the 56 stage7 leaves at one common rotated point and returns a native group claim (`stages/stage8/akita.rs:521-549`). Advice/direct-program auxiliary objects, when present, use a different prefix packing reduction (`:209-240,349-374`; `jolt-openings/src/prefix.rs:172-250`). There are none in the selected path.

`prepare_continued_instance` runs stages1,2,3,4,5,6a,6b,7 and then constructs pending native stage8 and both original RAM endpoint groups (`segment/frontend.rs:237-381`). `PendingNativeStage8::verify` is the separate PCS discharge (`stages/stage8/akita.rs:55-113`). Metadata/common-point checks do not authenticate values. The selected certificate premise is truth of every original stage8 Main leaf, each Main source-check leaf (or their correctly priced normalization), and both original RAM endpoint evaluations, plus any retained direct auxiliary objects in other paths. Main/RAM sources must be coherent across all these occurrences under the ordinary source-comparison theorem. No extra PCS/SIS/ROM error is silently set to zero; those belong to the surrounding composition budget.

## 8. Formal local statement and the precise remaining/non-selected predicates

For fixed original algebraic source assignment S (original A/B/F decommitments and decoded Main/RAM, public program, public endpoint state, and any retained original auxiliary source), let R_alg(S,u) consist of the unweighted original-source identities represented above, Main support/first-last moments, first RAM initialization, decoded memory/register transitions, and last public output. Use the deterministic algebraic virtual-table lift, not assumed honest short-word trace rows. Under the original true-leaf premise and fresh conditional laws of §2,

`Pr[the entire checked frontend and source certificate accepts AND not R_alg(S,u)] <= epsilon_reduce + R + epsilon_source (+ epsilon_normalization if used)`.

Each nonzero current-round discrepancy is bounded against an absorbed prover polynomial and the actual degree cap; each fold is bounded in its ONE drawn scalar; independent-coefficient mismatches use the last nonzero pivot; each reference test is a nonzero fixed-source residual polynomial. No deterministic implication is made for one randomized accepting transcript.

The following stronger predicates are NOT included, and not proved by this budget:

* Original RAM signed-plane range [-16,15], source canonicality, honest producer `OriginalBalancedDigitPlanes` or old `BalancedBase32` policy. Decoder-equivalent noncanonical original A/B/F sources are possible; ordinary algebraic source extraction does not establish those stronger raw predicates. The bridge's selected application consumes decoded K-memory and does not need them merely because the honest producer checks them.
* 64-bit range of FusedInc, or the honest carry set {-1,0,1}. Actual stage7 only supplies one-hot digits/carry in [-8,7] for w4 and the decode `sum_{i=0}^{15}16^i*d_i + 2^64*c`. Its min/max are `-(128*2^64-8)/15` and `(112*2^64-7)/15`, a set of 2^68 integer values, much wider than honest |delta|<2^64, still injective into p. A stronger delta/carry predicate needs its own argument; it cannot be quoted from Booleanity. `lattice/relations/booleanity.rs:44-50`; `digit_zero.rs:16-23,261-281`.
* SDK advice/input cursor bounds, canonical padding/consumption of a byte tape, exact emulator termination discipline, or a semantic theorem that every algebraic accepted state sequence is the expected byte ISA run. No-advice OBJECTS in this selected installation do not themselves imply every advice-related instruction semantic policy. The canonical address penalty closes the modulo-address issue but does not supply those tape invariants.
* Original statement/predecessor/program authenticity supplied by external context. Exact commitment/source identity and adjacency checks must remain (`segment.rs:305-355` as traced in RAW_RAM_BRIDGE); decoded value equality alone is weaker than original endpoint identity.

**Proven local counterexample if the original-leaf premise is omitted:** for a one-variable Boolean sumcheck and arbitrary purported sum S, a transmitted affine `g(X)=c+(S-2c)X` satisfies g(0)+g(1)=S. After the verifier chooses r, a free terminal oracle claim set to g(r) passes regardless of the intended source polynomial. Compressed mode constructs the linear coefficient this way automatically. Therefore a sumcheck-only theorem or staged pending frontend cannot establish source truth. This is a counterexample to the missing theorem predicate, not an attack claim against the complete PCS-authenticated continued protocol.

No code-level lack of the current canonical-address leg, Booleanity gamma-even powers, common RAM mass leg, fused-inc decode term, original Main physical source checks, or continued endpoint gammas was found in the inspected selected path. Missing broader ISA/tape adequacy must be named as a separate theorem/predicate rather than inferred from field arithmetic or honest constructors.

## 9. Optional branches: parameterization, not selected-path conflation

General round ledger: for every actual present batch member compute its full geometry rounds n_i, common N_s=max_i n_i, common D_s=max_i declared_degree_i, independent-coefficient pivot B_s, and both uni-skips `3(m_O-1)mu_U1 + 3(m_P-1)mu_U2`. Add `sum_g e_g mass_g` using the FULL evaluated expression including challenge-dependent derived publics; standalone symbolic max-degrees do not determine e_g. Examples already demonstrated: current canonical RAF gamma3; Booleanity even powers; digit-zero decode power; continued endpoint gamma.

For arbitrary selected Akita geometry set N_columns=nI+nB+nR+d+1, d=64/w (when supported), v=num_virtual_instruction_chunks, q=num_committed_per_virtual. The selected fold replacements are:

* Booleanity exponent `2*(N_columns-1)`;
* digit-zero exponent `2*(nI+nB)+3*nR+d+1`;
* instruction virtualization exponent v-1;
* bytecode address degree nB+1, full-public fused cycle degree nB+2;
* instruction RAF degree v+2, rounds t+128.

**Precommitted two-phase reductions:** optional committed bytecode/program-image members run degree2 over their actual global cycle and address schedules; there may be inactive rounds inside either phase. The coefficient input scale remains batch `2^(N-n_i)`. The reduction's own terminal has the separate skip factor `2^-(cycle_gap+address_gap)`; if it ends in cycle phase use only cycle gap. The verifier carries active cycle challenges, reverses/permutates them, and appends active address challenges, not a freshly sampled final point (`geometry/claim_reductions/precommitted.rs:274-309,316-389,478-493`). Each eta lane fold must be priced by its real maximum exponent and present layout. The committed bytecode chunk weights use eq over dropped address bits and the lane/reference grid, not gamma powers for chunks (`geometry/claim_reductions/bytecode.rs:224-311`). These members are absent in the selected full-public path.

**Akita advice:** original initialization advice leaves go directly from stage4 into native auxiliary objects (`stages/stage8/akita.rs:665-683`); non-Akita's advice 6b/7 reduction members are cfg-excluded (`stage6b/outputs.rs:105-112`). Optional physical prefix packing binds logical point/evaluations before drawing its selector and uses `sum_slot eq(selector,slot)*value_slot` (`jolt-openings/src/prefix.rs:172-250`). Its degree1-per-selector-coordinate cancellation charge must be included when such an object actually exists; the selector is not a powers draw.

**Field-inline:** adds 5 outer opening columns, extra equality rows and product lanes, field register claim/read-write/value/inc reductions, and an original field-inc terminal object. Use actual m_O from the composed row count and m_P=3+selected_product_lanes.len() (`composed/r1cs/mod.rs:40-65`; `composed/geometry.rs:5-19`; `field_inline/geometry/spartan.rs:3-13`). Stage4 beta now extends to powers3..5, and beta5 adds the field val RdWa at exponent2+L (`composed/relations.rs:249-287`); do not reuse the no-field-inline scalar334. Field registers address arity4 versus ordinary7 (`field_inline/config.rs:1-25`). Committed-program mode with field-inline is explicitly rejected, not a supported branch to price (`stages/stage6b/field_inline.rs:13-33`). No field-inline numerical closure is claimed without selecting its actual original group/dimension roster.

## 10. Evidence/hand-off

This report closes the selected frontend's reduction ledger, supplies a conservative conditional unweighted algebraic source-certificate bound and exact original-leaf premises, and separates unproved stronger source/ISA policies. It does not certify source-collision or ROM admission. No verification commands were run. Parent integration should preserve the exact leaf/witness formulas, original A/B/F/source coherence, sampler model, full root+suffix+terminal discharge, and external expected-statement boundary predicates. No build/test rerun is requested for this investigation.
