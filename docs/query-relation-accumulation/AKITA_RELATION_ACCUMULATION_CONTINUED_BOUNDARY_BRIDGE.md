# Candidate1 actual continued boundary bridge

## Scope and verdict

Read-only source investigation of the actual transparent, `CycleMajor`, K/K continued Jolt path. No repository edits, git/fetch actions, builds, tests, lint, formatting, or execution checks were performed. The anchor is the unchanged original Main/RAM/advice/program source assignment and the actual stage1,2,3,4,5,6a,6b,7 continuation, including its native stage8 leaves. The original Main commitment is already absorbed before stage1; this is **not** a missing-commitment finding.

**Key result:** the implemented endpoint reductions are real algebraic links, not merely transcript binding. With original-source leaf truth, Main Booleanity/digit-zero/virtualization, the additional `verify_segment_main` physical-zero/PC checks, and a fixed approved program, their underlying identities authenticate the initial and final **K-valued prefix states** and concatenate them. However, neither `SegmentContext` nor these identities imply actual RV64 byte-memory/register execution for every original algebraically decoded source. In particular, an original advice word `-1` is permitted by the algebraic source relation, cannot be the embedding of any `u64`, and can remain unused without violating any field frontend identity. An executed LD also has no loaded-value range check: its value can be `-1`, with the valid signed increment `-1`. The canonical instruction-address guard protects lookup/identity-RAF addressing, not LD data. SDK tape-length/cursor/compute-advice and emulator stop/device policy are additional, genuinely unfulfilled adequacy predicates if the application claims them.

The root maps are `AKITA_RELATION_ACCUMULATION_FRONTEND_COMPOSITION.md:171-189,346-357` and `AKITA_RELATION_ACCUMULATION_RAW_RAM_BRIDGE.md:29-69,92-111`. Their warning that decoded field memory is not automatically u64 memory survives inspection. Their old physical address-zero obstruction does **not** remain an unowned predicate when the current `verify_segment_main` is included: that implementation expressly checks active address-zero cells in the zero-omitting columns.

All deterministic implications below concern truth of the complete underlying algebraic identities on Boolean domains, **not** acceptance of one randomized transcript. A fixed-source probabilistic certificate and its actual sampler/FS loss remain required before transferring these implications to acceptance.

## 1. Actual entry points and fixation order

- Verifier: `jolt/crates/jolt-verifier/src/segment/frontend.rs:190-251` validates the endpoint contract, program metadata, predecessor, trace length/order; absorbs `SegmentContext`; calls `validate_and_seed_transcript`; checks preprocessing digest/public IO; rejects ZK; installs the segment-specific entry PC and removes initialization consumers after segment zero.
- The same function replays **all** stage1,2,3,4,5,6a,6b,7 (`:258-349`), prepares native stage8, derives both original RAM endpoint groups, and derives original committed-program boundary leaves (`:350-383`). It returns `ContinuedFrontPending`, not segment acceptance. `:33-57` exposes native, RAM, and program obligations separately.
- `jolt/crates/jolt-verifier/src/verifier.rs:417-452,804-900` calls input/config validation and absorbs native Main, optional untrusted/trusted advice, optional field increments, and the preprocessing-held direct program objects **before stage1**. There is no post-stage1 primitive Main fixation.
- `jolt/crates/jolt-verifier/src/segment.rs:359-457` absorbs run ID, preprocessing identity, normalized IO, index/total/active length, expanded counters, both PC/register boundaries, successor metadata, and the original incoming/outgoing RAM handles before frontend challenges.
- Prover: `jolt/crates/jolt-prover/src/akita/segment.rs:78-133` performs the corresponding validation, compares incoming/outgoing proof-side register vectors with public vectors, and uses the same context absorption and stage0. `:134-309` runs the continued stage spine and prepares stage8; `:321-357` retains the original Main rows in `PreparedContinuedFrontend` for source checks.
- Prover stage0 uses original retained auxiliary objects, assembles original Main, commits it, and calls the shared absorption routine: `jolt/crates/jolt-prover/src/akita/stage0.rs:168-187,198-251,284-350`. A malicious prover is not constrained by this honest constructor; the verifier predicates and fixed original source relation are authoritative.

The context is fixed public input, but fixation alone is not endpoint authentication. The reductions below provide the latter, subject to their randomized soundness and eventual opening discharge.

## 2. Exact public context contract

`jolt/crates/jolt-verifier/src/segment.rs:23-68` uses the convention:

- start/end PC name the **first/last active rows**, not the PC of the post-state;
- start/end registers name the **pre-first/post-last prefix states**;
- the outgoing post-state PC is represented separately by `successor`.

`SegmentState::validate` (`:94-140`) requires `0 <= index < total`, `total > 0`, power-of-two `N >= 4096`, `1 <= ell < N`, 128 field registers with x0=0, the feature-selected presence/length of 16 field registers, zero registers on the first segment, padded successor `(0,0,false,false,true)` on the last segment, and `!successor.is_virtual || successor.is_first_in_sequence` on interior successors. “Canonical field entries” here means elements of F, **not** canonical unsigned 64-bit values; `:106-109` contains no word-range test.

`SegmentContext::validate` (`:281-307`) checks, as nonwrapping host integer arithmetic,

```
c_after = c_before + ell,
first => c_before = 0.
```

It also validates both handles against the installed RAM contract and logical/physical dimensions, singleton dense commitment type, and registered RAM capacity. It does not inspect source coefficients.

`SegmentState::check_predecessor` (`:79-92`) and `SegmentContext::check_predecessor` (`:310-323`) check exactly:

```
index_j = index_(j-1) + 1; total and N identical;
B_j.registers = F_(j-1).registers;
B_j.field_registers = F_(j-1).field_registers;
start_PC_j = successor_PC_(j-1), in both coordinate systems;
c_before_j = c_after_(j-1);
incoming_RAM_handle_j = outgoing_RAM_handle_(j-1);
run_id, preprocessing_digest, inputs, outputs, panic identical.
```

Handle equality is exact original commitment/contract identity, not just equality of decoded memory. Extraction still needs coherent explanations for repeated original commitments or a priced source collision. A private prover Arc is not a public equality proof.

These counters count declared active **expanded rows**, including deliberately active no-ops. They do not count retired ISA instructions, memory accesses, tape consumption, emulator ticks, or non-noop rows. `jolt/crates/jolt-witness/src/backend/trace/continued.rs:61-80` explicitly retains active trailing no-ops. The source-counter bridge therefore requires the declared active-prefix convention, not an inferred non-noop rule.

The complete run's external expected statement must also be checked. `jolt/crates/jolt-verifier/src/receipt/archive.rs:229-268` compares every frame with external run/program/IO/index/total and checks the last expanded count equals `statement.expanded_rows`; the standalone continued function does not itself supply that complete-run assertion. This existing receipt path is evidence of boundary orchestration, not a claim that Candidate1's different final ordinary decider has already been admitted.

## 3. Proof-side field witness construction is not emulator replay

`jolt/crates/jolt-witness/src/backend/trace/continued.rs:10-30` expressly calls `ContinuedState` proof-side state, not an emulator checkpoint. It contains full K RAM, 128 K registers, and optional 16 K field registers.

`try_into_continued` (`:83-150`) validates vector dimensions and x0, then reconstructs rows from the incoming K state and **original raw row increments**:

```
R_(t+1)(a) = R_t(a) + Wa_t(a) * d_t^R,
M_(t+1)(a) = M_t(a) + Ra_t(a) * d_t^M.
rs1_t = R_t(rs1_t_address), rs2_t = R_t(rs2_t_address),
rd_write_t = R_t(rd_t_address) + d_t^R,
ram_read_t = M_t(ram_address_t),
ram_write_t = M_t(ram_address_t) + d_t^M.
```

It does not replay the instruction against an integer machine and does not prove the row is semantically valid. Raw `u64` row values remain authoritative for increments/selectors/instruction data, but reconstructed value columns use the incoming full-field prefix. `prefix_table` (`:167-188`) emits the pre-cycle state and stutters through padding; `oracle_table` (`:203-258`) overrides RAM/register values, final memory, instruction inputs, and product with recurrence-aware K values.

`jolt/crates/jolt-witness/src/witnesses/increments.rs:24-54,58-117` defines raw differences `rd_post-rd_pre` and `ram_post-ram_pre`, then a fused increment selected by Store. Honest compact normalization checks load/store value aliasing and program PC mapping (`jolt/crates/jolt-witness/src/backend/trace/mod.rs:403-474`), but those are producer checks, not an adversarial VM theorem. The row-window constructor checks complete active rows and canonical inactive rows, not public authentication (`jolt/crates/jolt-witness/src/consumer.rs:198-236`; `backend/trace/mod.rs:163-217`).

In short: the K witness construction correctly implements the intended prefix recurrence on its supplied data. The actual instruction equations are enforced by the stage spine, not by claiming this constructor performs execution validation.

## 4. Register endpoints have a genuine trace link

For address point x in K^7 define

```
Bhat(x) = sum_a eq(x,a) * B[a],
Fhat(x) = sum_a eq(x,a) * F[a].
```

`SegmentState::register_evaluations` computes exactly these two values from the full public vectors (`jolt/crates/jolt-verifier/src/segment.rs:142-176`). They are derived at the original stage4 register address, not at a substituted point (`stages/stage5/registers_val_evaluation.rs:62-91,179-218`).

Write `LT(y,i)` for the MLE of `1[i < t]` in the fixed pre-cycle argument y. The underlying continued identity is

```
V_R(x,y) + gamma*Fhat(x) - (1+gamma)*Bhat(x)
  = sum_i (LT(y,i)+gamma) * d_i^R * Wa(x,i).
```

Equations are implemented in `jolt/crates/jolt-claims/src/protocols/jolt/relations/registers/val_evaluation.rs:118-192`; concrete LT-plus-gamma derivation is `jolt-verifier/src/stages/stage5/registers_val_evaluation.rs:199-218`. Comparing the constant and gamma coefficients gives

```
V_R(x,y) = Bhat(x) + sum_i LT(y,i)*d_i^R*Wa(x,i),
Fhat(x) = Bhat(x) + sum_i d_i^R*Wa(x,i).
```

Stage4 read/write checking authenticates the values consumed by the CPU:

```
rd_write_i = sum_a Wa(a,i)*(V_R(a,i)+d_i^R),
rs1_i = sum_a Rs1Ra(a,i)*V_R(a,i),
rs2_i = sum_a Rs2Ra(a,i)*V_R(a,i).
```

See `jolt/crates/jolt-claims/src/protocols/jolt/relations/registers/read_write_checking.rs:65-104`. Stage3 reduces upstream register-value claims into these stage4 inputs; actual wiring is `jolt-verifier/src/stages/stage3/verify.rs:84-134`. Stage5 constructs endpoint evaluations before its independent endpoint gamma (`stages/stage5/verify.rs:135-159`). Full vectors were already absorbed pre-stage1.

Thus outgoing registers cannot be changed arbitrarily while maintaining the true polynomial identities. They are **not merely transcript-bound assertions**. Conversely, one accepting sumcheck transcript does not deterministically establish every coefficient identity. If the gamma residual is nonzero affine, it has at most one root; the ordinary subsequent coordinate/sumcheck soundness must also be charged. Under field-inline the identical construction uses 4 address bits and 16 full-field registers (`stages/stage5/field_registers_val_evaluation.rs:66-100,160-223`). Field values there intentionally are not RV64 words.

## 5. PC endpoints and corrected stage3 shift

### 5.1 Program metadata and initial PC

`SegmentState::validate_program` checks first entry, boundary PC ranges, start/end architectural mapping, legal start, and interior successor metadata against the full public table (`jolt/crates/jolt-verifier/src/segment.rs:179-233`). The committed alternative returns original chunk leaves for start/end architectural PC and the four successor values, with the actual zero-prefix physical embedding (`segment/frontend.rs:60-177`): two leaves on a final segment, six on an interior segment. Full-table mode needs no such PCS leaves.

The initial trace PC also has an existing native read-RAF owner: `checked.segment_entry_bytecode_index` is consumed in `stages/stage6a/verify.rs:58-71` and `stages/stage6b/batch.rs:210-218`. `jolt-claims/src/protocols/jolt/geometry/bytecode.rs:354-380` derives

```
Entry(k,t) = eq(bits(start_expanded_pc), k) * eq(0, t).
```

The read-RAF input has a constant entry leg and its output multiplies Entry by bytecode RA (`geometry/bytecode.rs:83-105,106-151`). With true one-hot identities this enforces selection of the declared entry at t=0. It would be incorrect to classify the start PC as having only context binding.

### 5.2 First/last PC moments and physical source masks

The additional source owner is `jolt/crates/jolt-verifier/src/segment_source.rs`, not the standalone continued function. For physical Main column H_c(t,a), radix k=2^b, active length ell, it checks:

```
H_c(t,a) = 0 whenever t >= ell;
H_c(t,0) = 0 for t < ell when column c omits address zero;
D_c(t*) = sum_a a * H_c(t*,a), for t*=0 or ell-1;
D_c(t*) = the matching base-k digit of the declared PC.
```

The zero-omitting set is all non-RAM columns (`segment_source.rs:127-139`), and the weighted union-mask implementation includes **active** address-zero cells (`:165-210`). Main Booleanity/hamming/digit-zero identities remain indispensable: a moment alone does not identify a selected address in an arbitrary field table. Under those identities each digit is unique, including implicit digit zero, and the declared PC is `sum_c k^(bB-1-c) D_c`, MSB chunk first (`:140-164`).

Actual verifier sumchecks and original leaf return: `segment_source.rs:319-415`. It binds original Main and the plan before anchors/coefficient vectors (`:257-290`), runs degree-two checks, derives physical `(cycle||address)` points, and returns original Main column obligations. The masked equality is the MLE of the **restricted equality table**, not the invalid product of two independently extended masks (`:165-210,366-379`). Prover agrees (`jolt-prover/src/akita/segment_source.rs:249-310`).

The current existing receipt consumer calls `verify_segment_main` and includes those leaves in original Main normalization (`jolt-verifier/src/receipt/native.rs:268-304`). This closes the earlier physical-zero counterexample **when these obligations are included and discharged**. The generic `prepare_continued_instance` returns neither these checks nor their proof; an application using only that function omits them.

A concrete omission consequence is that a final segment's shift correction is padded zero regardless of ell. Most of its stage spine then constrains the full N-row trace and register/RAM endpoints, not the separately declared final active length/end-row PC. Context counters can be adjusted to another ell, and the full table merely authenticates that the claimed end PC is a program row. The missing predicates are the actual physical inactive-prefix mask and last-row PC moment on the same original Main, not another transcript absorption.

### 5.3 External successor correction

Let A be the native outer anchor and P the **distinct** product-remainder anchor. For last active row L=ell-1 and successor metadata `(u_s,p_s,v_s,f_s,n_s)`, define

```
C(gamma) = eq(A,L)*(u_s + gamma*p_s + gamma^2*v_s + gamma^3*f_s)
           + gamma^4*eq(P,L)*(1-n_s).
```

`jolt-verifier/src/stages/stage3/spartan_shift.rs:94-124,153-187,225-237` implements precisely this correction. The symbolic input subtracts C from the folded Next claims (`jolt-claims/src/protocols/jolt/relations/spartan/shift.rs:72-125,166-173`), and the ordinary no-wrap shift kernel checks the result against current-row columns with `EqPlusOne(A,.)` for the first four legs and `EqPlusOne(P,.)` for `1-IsNoop`. Prover uses the same concrete relation (`jolt-prover/src/stages/stage3.rs:99-136`).

With the physical suffix-zero/inertness and one-hot identities, this gives

```
NextQ(t)=Q(t+1), 0 <= t < ell-1;
NextQ(ell-1)=q_successor,
```

for UPC, expanded PC, virtual, first, and the complement-noop convention. The public legal-cut check ensures the next segment begins outside a virtual group's interior; program well-formedness must ensure those metadata actually describe canonical instruction/reset groups. Final successor is the original padded convention, **not** an authenticated architectural halt state.

## 6. Continued RAM, original leaf ownership, and IO

For decoded original memories M^- and M^+, ordinary endpoint groups query those same commitments at address x. `SegmentContext::endpoint_groups` derives `[0_physical_padding || x]` and one evaluation each (`jolt-verifier/src/segment.rs:325-355`). The selected RAM65536 geometry has no physical RAM coefficient padding; the ordinary query covers its decoded 16-variable polynomial, not arbitrary raw planes.

The prover evaluates its incoming/outgoing K state at the stage2 RAM address (`jolt-prover/src/akita/segment.rs:155-176`). Verifier checks that exact address and requires the last stage2 final-output point/value to alias outgoing RAM (`stages/stage4/ram_val_check.rs:631-665`), then absorbs these endpoint values before the existing RAM gamma (`:667-682`; `stages/stage4/verify.rs:184-186,238-254`).

Writing B_M(x)=MLE(M^-)(x), F_M(x)=MLE(M^+)(x), the identity is

```
V_M(x,y)+gamma*F_M(x)-(1+gamma)*B_M(x)
  = sum_i (LT(y,i)+gamma)*d_i^M*Ra(x,i).
```

This is the existing symbolic RAM equation, with the incoming endpoint substituted (`jolt-claims/src/protocols/jolt/relations/ram/val_check.rs:95-164`; `jolt-verifier/src/stages/stage4/ram_val_check.rs:157-179,300-327`). Its coefficients assert both pre-cycle prefix values and the complete final update. Stage2 read/write checking links `ram_read_i` to `Ra*V_M` and `ram_write_i` to `Ra*(V_M+d_i^M)` (`jolt-claims/.../relations/ram/read_write_checking.rs:60-83`).

First initialization is separately reconstructed and required equal to B_M(x) (`jolt-verifier/src/stages/stage4/verify.rs:189-206,248-264`). The exact reconstructed value is

```
I(x)=I_public(x)+I_program_image(x)+sum_kind selector_kind(x)*Advice_kind(x_local).
```

`ram_val_check.rs:433-455` implements addition of all present contributions. Full public program and input initialization is `jolt-program/src/preprocess/ram.rs:140-169`. Committed mode's image remains an original source leaf. Later segments clear initialization consumers and use incoming RAM instead (`segment/frontend.rs:249-251`); native stage8 omits only the already-unneeded program-image initialization group, retaining bytecode groups (`stages/stage8/akita.rs:435-453`).

Last-only output is not context-only. Stage2 enables it only on the last segment (`stages/stage2/verify.rs:159-200`) and proves

```
0 = sum_a eq(z,a)*IoMask(a)*(M^+(a)-ValIO(a)).
```

See `jolt-claims/src/protocols/jolt/relations/ram/output_check.rs:88-146`. The concrete IO table covers `[input_start,RAM_START_ADDRESS)` in the remapped word domain, preserving inputs, outputs, panic, zero gaps, and termination convention (`jolt-program/src/preprocess/public_io.rs:20-59`). For nonpanic it includes termination word 1; on panic the sparse table omits that term (the full masked value there is consequently zero).

Native IO binds inputs, outputs with trailing zero bytes removed, panic, and memory layout (`segment.rs:258-278`; `verifier.rs:501-505,568-579`). It does not authenticate the original output vector length including trailing zeros, append-only IO events, or a host journal. `JoltDevice::store` is indexed, grows/zero-fills output, has sticky panic, and ignores termination stores; termination loads return zero (`jolt/common/src/jolt_device.rs:143-207`). This is a distinct device/emulator policy, not automatically the word-RAM algebraic relation.

## 7. Trusted/untrusted advice and retained program/setup

| Object | Public authority / existential content | Actual cryptographic use | Not automatically implied |
|---|---|---|---|
| Trusted memory-mapped advice C_T | Verifier/application supplies an accepted commitment; bytes are private | First RAM initialization contribution and original dense stage8 leaf | Unsigned word range, exact private byte length, provenance of arbitrary accepted commitment |
| Untrusted memory-mapped advice C_U | Proof supplies commitment; source contents are existential | Same first initialization/direct dense leaf | Byte/u64 membership for arbitrary extracted original source; canonical zero padding |
| Arithmetic `VirtualAdvice` | Existential runtime witness output | RangeCheck lookup plus write-to-rd equation | Execution of host advice computation or linkage to C_T/C_U |
| `VirtualAdviceLoad` and `VirtualAdviceLen` | Also arithmetic Advice flag and RangeCheck relation | Same lookup/write-to-rd mechanism | FIFO cursor, remaining-length law, width/EOF semantics, global fixed tape |
| Field inverse/limb advice, if enabled | Existential field witnesses | Selected field equations; limb instruction also RangeCheck | Canonical integer field readout without terminal quotient zero and integer < modulus check |
| Full program/preprocessing | Approved public table/image/layout | Verifier evaluates fixed tables and compares metadata directly | Arbitrary manually supplied metadata being canonical expansion without approval |
| Committed program | Approved commitments plus metadata; prover retains original objects | Bytecode/image reductions and boundary original chunk leaves | Algebraic commitment consistency alone being a canonical program/setup certificate |
| PCS setup/catalog/transcript installation | Application-approved immutable authority | Validated shapes/roles/layouts, complete native opening | A proof supplying its own arbitrary setup and calling it registered |

Evidence: `jolt-verifier/src/preprocessing.rs:15-35,54-75`; `jolt-prover/src/akita/witness.rs:345-388,398-455`; `jolt-prover/src/akita/stage0.rs:168-187`; `jolt-verifier/src/stages/stage8/akita.rs:655-675`. Honest advice construction is bytes -> canonical little-endian u64 -> K, with zero padding (`jolt-witness/src/backend/trace/advice.rs:25-48`), but stage8 checks metadata/point coverage, not word range (`stage8/akita.rs:165-203`). Algebraic A/B/F extraction does not inherit the honest constructor.

The program must be retained as a canonical approved reference, or be covered by a separate same-source well-formedness/reference certificate. In committed mode the verifier has no table to scan; `validate_program` deliberately defers to source leaves. A leaf opening of a First flag says that the original lane has that value, not that the lane was canonically stamped. `BytecodePCMapper::validate_run` checks descending sequence counters, not the meaning of arbitrary supplied first flags (`jolt-program/src/preprocess/bytecode.rs:173-210`). Honest preprocessing checks instruction support/store-rd disjointness (`:31-55`), and canonical expansion is an external retained-program adequacy fact. Full-program checks use that approved concrete table; committed source equality to the approved concrete lane/image assignment must be established or a collision returned.

Existing concrete `receipt/native.rs:268-283` rejects auxiliary/program-boundary groups altogether and uses a full approved program/no-advice profile. The generic continued API supports advice/committed program, but that does not demonstrate admission of every such combination in Candidate1's final catalog. Field-inline additionally requires full public program (`verifier.rs:549-566`).

## 8. Exact adequacy counterexamples

### 8.1 Strong, whole-field-relation counterexample: unused original advice word -1

Choose a canonical approved program/ordinary valid trace that never accesses one logical untrusted advice word a*. Set that original dense advice coefficient to `-1` in K, insert the same value at a* in the original incoming RAM, preserve it at every RAM boundary, and leave all actual execution values/selectors/increments and IO unchanged. Use consistent new original A/B/F commitments to these sources, fixed before challenges; **no same-commitment forgery is being asserted**.

All selected algebraic identities remain true:

1. First initialization inserts exactly the original advice word `-1`.
2. At a*, every RAM access selector is zero and every update is zero.
3. The prefix and outgoing memories preserve `-1`; endpoint MLEs simply include its equality-weighted contribution.
4. RAM read/write relations are unchanged because a* is never selected.
5. Final IO mask excludes the advice region, so final public IO is unchanged.
6. Original Main, its physical masks, Booleanity, and PC moments remain the ordinary valid ones; registers and counters are unchanged.
7. Adjacent original RAM handle aliases remain exact by reusing each outgoing handle as the next incoming handle.

Yet no u64 byte word w satisfies `embed_K(w)=-1`: the selected modulus is greater than 2^64, so its canonical value q-1 is outside `[0,2^64)`. This is a counterexample to **fixed-original-source byte-memory initialization**, not to the declared K-algebraic continued relation. It cannot be repaired by selecting balanced RAM planes: plane0=-1 and the remaining planes zero is already a legal balanced representation. Nor can a witness bridge silently replace that inaccessible original word by 0: that would violate the question's fixed original source contract. A weaker observational theorem can ignore/change unobserved cells only if its application statement explicitly permits that different projection.

Missing predicate for byte-memory: at minimum `forall logical advice word a, Advice(a) in embed_K([0,2^64))`, plus any selected byte-length/tail-zero contract. It is a predicate on the original decoded advice source (or equality to a certified canonical reference), not arbitrary raw digit range. For trusted advice honestly generated and fixed by the application, the retained canonical-reference fact plus source binding can provide this premise; the word “trusted” alone cannot.

### 8.2 Executed LD is not rescued by InstructionReadRaf

Actual LD constraints are only

```
Load*(RamRead-RamWrite)=0;
Load*(RamRead-RdWrite)=0;
(Load+Store)*(RamAddress-Rs1-Imm)=0.
```

See `jolt-claims/src/protocols/composed/r1cs/rv64.rs:231-252`. LD has **no lookup** (`jolt-lookup-tables/src/instructions/riscv/ld.rs:1-16`), only the Load flag (`jolt-riscv/src/instructions/i/ld.rs:3-8`), and no `WriteLookupOutputToRD` guard. The dedicated fixture explicitly documents unused instruction/lookup inputs and output all zero (`rv64.rs:634-658`). SD likewise has no lookup (`instructions/riscv/sd.rs:1-16`), and its data equation is `Store*(Rs2-RamWrite)=0` (`rv64.rs:254-256`).

Thus with word-address base registers and original advice cell -1, an LD into an initially zero nonzero register has

```
RamRead=RamWrite=RdWrite=-1; RdInc=-1; RamInc=0.
```

This is a valid K-valued load transition, not an RV64 word transition. Main can encode delta -1 with a valid centered low digit k-1 and zero higher digits/carry. Subsequent SD can copy -1 to ordinary RAM with delta -1; an overwrite by an ordinary lookup instruction returning 0 uses delta +1. Keep the loaded register unused by a word-operands lookup until overwrite, or retain it as a nonword public final register. A canonical program can otherwise execute its ordinary IO/termination sequence. The field identities admit this behavior; no actual runtime acceptance experiment was run or claimed.

The balanced increment reduction explicitly does **not** establish honest `|delta|<2^64`; it covers the full centered carry alphabet (`jolt-claims/src/protocols/jolt/lattice/relations/digit_zero.rs:16-22`; `relations/booleanity.rs:45-52`). This fact is not needed for the -1 example, whose increment is already an honest-size signed delta, but independently blocks inferring word bounds from the delta encoding.

The Akita InstructionReadRaf guard is real: `stages/stage5/instruction_read_raf.rs:199-230` adds `gamma^3 * U(k)` on identity-RAF cycles, where U is the all-ones upper-half address predicate. It forces surviving identity addresses below `2^128-2^64<q` (`jolt-claims/.../geometry/instruction.rs:24-59`). Arithmetic advice uses RangeCheck, whose table output is the low 64-bit word (`jolt-lookup-tables/src/tables/range_check.rs:13-29`), and write-to-rd equals that output (`rv64.rs:306-311`). So arithmetic advice output is range-constrained by the complete lookup relation. **LD's loaded data never enters that leg.** It is incorrect to generalize the guard to every write or every memory advice coefficient.

### 8.3 Tape/length/host-computation mismatch

`VirtualAdviceLen` and `VirtualAdviceLoad` both have `[Advice, WriteLookupOutputToRD]`, no additional tape flags (`jolt-riscv/src/instructions/virt/advice_len.rs:3-8`, `advice_load.rs:3-8`). Their lookup implementations only range-check the supplied rd output (`jolt-lookup-tables/src/instructions/virt/advice_len.rs:6-25`, `advice_load.rs:6-25`). No fixed tape, cursor, or equality with remaining length appears in these equations.

Concrete field-relation counterexample: a program executes two consecutive `VirtualAdviceLen` instructions without consuming bytes, with range-checked outputs 1 and 2. Both individual arithmetic-advice instructions and register recurrences satisfy the relation. No single actual `AdviceTape` can produce these outputs because `remaining=len-read_position` is unchanged between them. The emulator implementation is exact at `jolt/tracer/src/emulator/cpu.rs:17-91`, and tracer instruction uses that remaining count (`tracer/src/instruction/virtual_advice_len.rs:19-23`). This is stronger than merely an externally authenticated tape being absent: arbitrary existential outputs need not admit *any* FIFO tape witness when length queries are exposed.

Missing adequacy predicate, if tape semantics is selected: there exists one byte tape and a nonwrapping cursor such that every read consumes its instruction width, returns the corresponding little-endian bytes, fails/passes EOF correctly, and every length query equals the then-remaining bytes; adjacent cuts must carry the same tape identity and cursor/unread suffix. Host `compute_advice` requires a further execution relation. SDK macro's compute build writes the computed result, while the proved build reads an existential result (`jolt/jolt-sdk/macros/src/lib.rs:1262-1275`); proving the latter does not prove the former.

The purely arithmetic existential-advice relation intentionally does not require these tape predicates. For length-free, full-width reads an arbitrary sequence of u64 outputs can be encoded in a concatenated private tape, but that limited observation cannot establish the length-query example or exact SDK policy.

### 8.4 Final padded successor is not emulator halt

The final convention sets NextIsNoop=1. The RV64 product relation sets `ShouldJump=Jump*(1-NextIsNoop)` (`rv64.rs:392-394`), so the jump-target equation is disabled on a final Jump row, and the sequential-PC equation is disabled by Jump (`:316-342`). A final jump need not target its own PC. Thus an otherwise valid trace ending after termination word 1 at a forward JAL can satisfy the padded-successor relation although the emulator would execute its target.

This is not a missing source leaf: it is the intentionally weaker terminal convention. Actual tracer termination detects a PC stall (and trace mode also a no-row trap), not a merely padded successor (`jolt/tracer/src/lib.rs:110-120,201-208,327-336`). The SDK clean stop writes termination 1 then emits `j .` (`jolt/jolt-sdk/macros/src/lib.rs:881-887`). Exact emulator-stop adequacy therefore needs a final policy proving that stopping row has the intended stall/shutdown behavior, or a theorem restricted to the approved program where reaching this native termination convention entails the desired application end. Final public IO/termination word truth alone does not prove it for arbitrary supported programs.

## 9. Precisely closed selected relation and remaining theorem contract

For fixed original algebraic sources and an approved full/certified program, define `R_K_cont` to require:

1. All selected Main Boolean/one-hot/virtualization/increment decoding identities, and current same-original-Main physical masks/first-last PC moments.
2. Original program-lane/image use, actual RV64/field row equations, product and corrected shift identities.
3. K-valued register/RAM prefix recurrences with declared public register endpoints and original decoded RAM endpoints.
4. First-only zero registers and exact public/program/advice initialization; last-only native normalized IO relation and padded successor convention.
5. Host-integer expanded-row counters and exact predecessor/run/program/IO/original-RAM alias checks.
6. Every deferred native Main/advice/program/field-increment/RAM/source-check/normalization leaf interpreted and eventually discharged on those same original sources; no acceptance from pending carriers.

**Boundary assembly lemma [derived from the displayed identities]:** true identities in (1)-(6) imply that each segment's declared pre/post **K prefix state** is its real state in this selected algebraic relation, PC metadata describes its first/last rows, and consecutive segments concatenate exactly. Proof: constant/endpoint-gamma coefficients give the prefix and final recurrences; one-hot read/write equations identify CPU values with those prefixes; PC moments/native entry identify trace boundaries; corrected shift identifies the external successor; exact context equalities join all components; initialization and final IO occur once. This is deterministic assembly of relation witnesses, not a deterministic accepting-transcript implication.

To derive probabilistic soundness for this selected relation, one still needs the fixed-source backwards stage certificate for the actual stage list/aliases and all original leaves, with the actual sampler law and FS challenge causality. Boundary-local residuals are explicit: endpoint folding is affine in its independent gamma; shift folding has degree at most 4; Main source checks use independent coefficient vectors and degree-two sumcheck rounds. For genuinely uniform independent field draws, the nonzero affine/degree-four cancellation probabilities are at most `1/|K|` and `4/|K|` respectively, before coordinate/sumcheck losses. Main zeros additionally have an anchor residual of total degree at most the physical column arity; PC vector batching has a nonzero linear residual if any digit moment is wrong. These belong in the real whole-stage ledger, not as an assertion that one accepted leaf is a universal identity or as an unproved numerical admission of an actual sampler.

**Genuinely unfulfilled if ordinary fixed-source VM adequacy is claimed:** `R_K_cont => R_RV64_bytes` is false under arbitrary original decoded advice/RAM sources, by §8.1 and §8.2. A complete stronger relation must name and prove unsigned decoded-word/reference predicates wherever word execution is claimed, canonical program/reference adequacy, any tape/cursor/EOF/compute-advice policy, and any required device/stop semantics. Balanced raw RAM range is not the missing unsigned-word predicate. Alternatively, select and name `R_K_cont` itself and do not call its K-valued memory/register witnesses emulator byte states.

No check execution was performed. This report identifies source-backed equations and mathematical counterexamples, not a tested exploit, completed cryptographic extractor, finalized field/catalog admission, or a proof of the global Candidate1 composition theorem.
