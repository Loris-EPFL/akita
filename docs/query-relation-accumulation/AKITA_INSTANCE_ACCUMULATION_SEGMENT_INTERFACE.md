# Native Akita pending frontend and consecutive-segment interface

## 1. Scope, anchor, and revision discipline

**Status: source-backed interface specification and derived endpoint reductions; no implementation, admitted receiver profile, numerical parity claim, or measured baseline.** The anchor is the **existing whole-run Jolt proved relation and public statement**, decomposed into consecutive active trace segments, not a conjunction of independent jobs and not a stronger theorem equating every emulator/MMIO behavior with the verifier relation. Segment boundaries are public; proof-side RAM is carried by original native Akita commitments. There is no Merkle memory replacement, commitment homomorphism, completed-proof accumulator, or new FIFO public-I/O contract.

All `J:` paths below mean immutable Jolt [`0791f79a55c3fadc3d327c78a72228696c338468`](https://github.com/a16z/jolt/tree/0791f79a55c3fadc3d327c78a72228696c338468); all `A:` paths mean immutable Akita [`83574331d4e51f8cce1d8689d05592fa4ef4c138`](https://github.com/LayerZero-Labs/akita/tree/83574331d4e51f8cce1d8689d05592fa4ef4c138). Sources were inspected with `git show`, without checkout or source modification. Jolt's root `Cargo.toml` pins its Akita dependencies to exactly this A revision. The local working-tree heads are not this pair.

This specializes main design §§3–5,10. It supersedes the **choice** of Merkle RAM and exact-length journal in `AKITA_ACCUMULATION_05_EXECUTION_COMPOSITION.md` for this native candidate; those were conditional designs, not requirements inherited here. The local primary paper *Proving CPU Executions in Small Space*, §§1.1.1,1.1.4, retrieved through QMD (`lattice-papers/cpuexecinsmallspaces.md`, lines 33–37,63–67), requires register/memory endpoint matching and explicitly recognizes memory-linear continuation cost. It supplies no Jolt implementation of the interface below.

The minimal witness-generation route executes the existing VM once and partitions its trace at legal complete-instruction/inline boundaries. It derives intermediate RAM/register endpoints from the proof-side prefix recurrences. Online tracing, resumable-emulator equivalence and new termination-event recognition are not required. A host device snapshot is not automatically that proof-side RAM.

## 2. Frozen shape and typed interface

Before any proof, fix the application/profile, supported instruction build, expanded bytecode/preprocessing identity, public input bytes, memory map, **`TracePolynomialOrder::CycleMajor`**, trace/RAM bounds, source catalogue, native transcript domains, and a finite segment bound `Tmax` (or exact nonzero T). The pinned Akita `prove_stage0` and preprocessing validation reject `AddressMajor`; it is not an available choice in this candidate. Choosing the supported shape after a preliminary execution, but before proving, is legitimate. No online/unlimited-depth requirement is imposed.

Use these shape symbols:

- `N=2^n >= 4096`: physical expanded-row capacity of one segment; `1 <= ell < N` is its active prefix, reserving at least one canonical padding row. Capacity is not active execution length. This matches immutable `jolt-prover/src/config.rs::derive_from_rows`, which rounds `rows.len()+1` to a power of two and enforces `MIN_PADDED_TRACE_LENGTH=1<<12`. Its whole-execution final-jump host guard is not a live-segment constructor and must not be confused with a verifier theorem. Internal cuts use the checked external successor; the global final cut retains the native final-row convention and applicable baseline host validation.
- `M=2^m`: fixed word-addressed, remapped **proof-side** RAM capacity of the run. Its ordinary word interpretation and canonicality obligations are those of the unchanged execution/initialization relation; a producer preset alone does not prove them. Keep M/map fixed across boundaries.
- `k in {4,8}`, `K=2^k`: native one-hot chunk width/domain.
- `bI=128/k`, `bD=64/k`, `bB=ra_layout.bytecode()`, `bR=ra_layout.ram()`. The actual geometry constructor fixes bB,bR from padded bytecode/RAM dimensions and k; use that result, including rounded chunk/address padding, not a separately guessed count.
- `P=bI+bD+1+bB+bR <= 64`: native main column count; each has arity `n+k` and physical Boolean table size `NK`.
- `L`: padded expanded bytecode row count; `c`: committed-bytecode chunk count, if selected, a valid power of two `1..256` dividing L. `W`: program-image word count padded to a power of two at least two. `H=2^h`: bytecode lane capacity, with h=`committed_lane_vars()`.
- `a_U,a_T`: logical arities of canonical advice word vectors, determined from declared advice byte capacities. `d(x)=max(14,x)`; dense object arities must satisfy their actual catalogue bounds (advice/program plans reject above 34).
- `f in {0,1}`: field-inline profile; `p in {0,1}`: committed-program profile; `fp=0`. `u,t in {0,1}` are actual untrusted/trusted advice commitment presence.

J: `crates/jolt-claims/src/protocols/jolt/lattice/{packing,strategy}.rs` (`one_hot_trace_columns`, `ONE_HOT_TRACE_LAYOUT.plan`, `MAX_ONE_HOT_TRACE_COLUMNS`); `crates/jolt-claims/src/protocols/jolt/geometry/dimensions.rs` (`JoltOneHotConfig`, `committed_address_chunks`); `crates/jolt-claims/src/protocols/jolt/geometry/claim_reductions/bytecode.rs` (`committed_lane_vars`, `is_valid_committed_bytecode_chunking_for_len`).

Proposed types, not existing Rust APIs:

```
SegmentPublic = {
  profile_id, run_id, index, total_or_cap,
  input_device_contract, program_and_setup_identity,
  shape, active_rows: ell, expanded_count_before, expanded_count_after,
  start: Boundary, end: Boundary, is_first, is_final
}
Boundary = {
  expanded_pc, architectural_pc,
  integer_virtual_registers: [canonical_F; 128],
  field_registers: Option<[canonical_F; 16]>,
  ram: OriginalHandleAndFullShape,
}
FrontPending = {
  statement: SegmentPublic,
  canonical_frontend_messages,        // commitments, stages 1–7, endpoint reductions
  packing_and_multipoint_messages,
  openings: VerifierDerivedLedger,     // never accepted from caller on trust
  transcript_checkpoint
}
FrontPrivate = {
  original_sources_and_opening_hints,
  endpoint_RAM_field_values_and_native_decommitments,
  trace_witness_and_required_virtual_oracles,
  live_backend_authority, transcript_state
}
```

`OriginalHandleAndFullShape` includes original commitment C, source/polynomial role, layout digest, source map/config identity, physical polynomial count and arity, tensor/coefficient packing, matrix/compression identities, signed digit parameters, and padding predicate. C bytes alone are not a shape. Private ownership retains the *original* source, A/inner/compression decommitments and quotient material; a logical dense evaluation vector is not necessarily the committed short source.

Use canonical field encodings for proof-side public register values. Honest integer registers contain u64 words; restricting every algebraically valid cut-state to u64 requires that implication from the unchanged relation. Likewise, new RAM handles default to the full-field dense class, not signed-65. This preserves the algebraic statement without an extra word-domain premise. A smaller representation is an optimization only after that premise is proved. See [endpoint cost and domain analysis](AKITA_INSTANCE_ACCUMULATION_MEMORY_COST.md).

The public mathematical relation is: the revised complete frontend accepts the prefix and derives its exact ledger, **and every ledger claim opens its original committed source with the declared full physical shape**. `FrontPending` establishes only the first conjunct conditionally on the second. Accumulation must discharge the second, under its own authenticated history/finite-tree proof, before acceptance. Endpoint C equality is checked together with shape/setup identity across `end_i=start_(i+1)`; it does not compare merely random endpoint values.

## 3. Exact existing fresh-source ledger

The immutable source order is `[UntrustedAdvice?, TrustedAdvice?, FieldInc OR BytecodeChunk(0..c), ProgramImageInit, OneHotTrace]`, where the image is present only with committed-program mode. Full-program/non-field mode has neither suffix. This is J: `jolt-prover/src/akita/stage8.rs::prove_stage8` and `jolt-verifier/src/stages/stage8/akita.rs::{verify,leaf_claims,one_hot_trace_claim,field_inc_claim}`. In the table, “one query” means one physical point with a vector evaluation, not one polynomial.

| Original group C / ordered role | Physical polynomials and shape | Actual producer alphabet / map | Existing pre-PCS query |
|---|---|---|---|
| `C_U`, untrusted advice (if u) | 1, arity d(a_U), `2^d(a_U)` cells; logical words in zero-prefix slot | canonical padded little-endian u64 words; `JoltDenseBounded`, balanced signed-digit decomposition | 1 physical scalar query from `reduce_precommitted`; logical leaf originally from stage-4 init contribution |
| `C_T`, trusted advice (if t) | 1, arity d(a_T), same embedding | same word/decomposition class; commitment supplied under selected trusted-advice policy | 1 physical scalar query, same reduction family |
| `C_F`, `field_inc` role order 2 (if f) | 1, arity d(n); first N cells are `FieldRdInc`, rest physical padding | arbitrary proof-field elements; `JoltDenseFull`, **not** u64 bounded | 1 at `(0^(d(n)-n) || r_field_cycle)`, from stage6b field-increment reduction |
| `C_B,j`, indexed `bytecode_chunk` role order `2+j` (if p), `0<=j<c` | 1 each, arity d(h+log2(L/c)); lane×chunk-row table, zero-prefix dense floor | Boolean register/flag/lookup lanes; scalar u64 PC; signed immediate of absolute magnitude <=u64::MAX; zero unused lanes; bounded dense decomposition | 1 physical scalar per chunk after staged precommitted reductions and `reduce_precommitted` |
| `C_image`, `program_image_init`, order `2+c` (if p) | 1, arity d(log2 W), original W-word image then physical zeros | u64 words; bounded dense decomposition | 1 physical scalar query after program-image claim reduction and packing |
| `C_X`, native `OneHotTrace` last | P columns, each arity n+k, **not one prefix-packed polynomial** | Boolean physical selector tables with native zero-digit omission rules; unit-one-hot source map | 1 common physical point; P evaluations, in exact column order below |

`C_X` order is `InstructionRa[0..bI)`, `BalancedIncDigit[0..bD)`, `BalancedIncCarry`, `BytecodeRa[0..bB)`, `RamRa[0..bR)`. Increment columns do **not** commit the signed increments as dense words: they select centered digit values in radix K, plus a signed carry with place value `2^64`. J: `jolt-claims/src/lattice.rs::{BalancedIncChunking,balanced_inc_value,balanced_digit_row,balanced_carry_row}` fixes this decoding. Physical values are Boolean. The instruction/bytecode/increment families omit **digit address zero**, not cycle zero; RAM includes digit zero on active accesses, distinguished by `committed_digit_zero_mask`. J: `jolt-prover/src/akita/witness.rs::{fill_trace_row,assemble_one_hot_trace_rows,OneHotTraceRows}`. Do not assert exactly one physical one per row for a zero-omitting family, nor discard RAM address-zero bits.

Dense source semantics are pinned by J: `jolt-akita/src/configs.rs::{JoltDenseBounded,JoltDenseFull}` and A: `akita-config/src/proof_optimized/fp128.rs::{DenseBounded,Dense,OneHot}`. Bounded dense uses **signed 65-bit** commit bound `[-2^64,2^64-1]`, containing unsigned u64 and signed program immediates; opening-side data remains full 128-bit. Full dense uses 128-bit commit/open bounds. Actual balanced radix/depth and physical source dimensions come from the admitted original group schedule; “u64 table” does not mean a one-digit identity source. Do not inherit the main design's identity-source local lemma for these gadgets without the raw-source observation bridge.

Producer evidence: J: `jolt-prover/src/akita/witness.rs::{commit_advice,commit_direct_program,program_image_words_padded}`; `common/src/advice.rs::canonical_advice_words`; `jolt-prover/src/akita/field_inline.rs::commit_field_inc`; `jolt-kernels/src/committed_program.rs::{build_committed_bytecode_chunk_coeffs,for_each_active_lane_value}`. Bytecode lane count is the next power of two of `3*128 + 2 + NUM_CIRCUIT_FLAGS + NUM_INSTRUCTION_FLAGS + LookupTableKind<XLEN>::COUNT + 1`, not an invented workload constant.

### 3.1 Point coordinates are part of the claim

All points below are MSB/high-to-low semantic coordinates after the specified conversion.

- Main trace leaf point is `(address_chunk[k] || cycle[n])`. `ONE_HOT_TRACE_LAYOUT.column_point` permutes it to physical `(cycle[n] || address_chunk[k])`. `one_hot_trace_claim` requires every column to agree *exactly* on this point and validates P and arity against both setup and C. Preserve this common-point vector output.
- A cycle-only sumcheck produces challenges in binding order; `TraceDimensions::cycle_opening_point` reverses them. Do not copy the raw challenge array into a source query.
- For read/write dimensions `(n,m,p1,p2)` and raw challenges z of length n+m, address coordinates use indices `[n+p2 .. n+m)` reversed, then `[p1 .. p1+p2)` reversed; cycle coordinates use `[p1+p2 .. n+p2)` reversed, then `[0 .. p1)` reversed. This is exactly `ReadWriteDimensions::read_write_opening_indices`. The address-only output relation omits phase-1 cycles: indices `[p2+n-p1 .. m+n-p1)` reversed, then `[0 .. p2)` reversed, in its own challenge window. Use the native stage offsets.
- `TracePolynomialOrder::CycleMajor` has **index = address*num_cycles+cycle**; `AddressMajor` has index = cycle*num_addresses+address. Names alone are misleading. Consequently committed bytecode logical points are `(lane[h] || chunk_row[log2(L/c)])` for CycleMajor and the reverse blocks for AddressMajor. J: `geometry/claim_reductions/bytecode.rs` splits them this way. Native `C_X` is independently row-major `(cycle||digit)`.
- Advice and image logical points index words. `PrefixPackedObjectPlan::packed_claims` applies the zero-prefix selector and `packing().reduce_claims` derives the actual physical point/value; they cannot be replaced by attaching an unrelated prefix. FieldInc instead explicitly selects the zero prefix as above.

The `AddressMajor` coordinate formula above documents the generic layout distinction only. The selected immutable native path fixes `CycleMajor`; supporting the other order would be an additional migration and is not covered by this interface or its cost ledger.

## 4. Minimal native RAM endpoint change

Add original full-field dense handles `C^-_R,C^+_R`, each **one** polynomial of physical arity `d(m)`, not `64M` bitplanes. Logical B-/B+ are the first M proof-field entries; decoded physical padding is zero and is authenticated under the original source map. Raw padding zeros are imposed only where the declared source relation requires them. Signed-65/u64 endpoint representations remain conditional on §2's word-domain implication. Define new domain-separated endpoint roles outside the existing role namespace and bind them before frontend challenges; retain the existing relative auxiliary/main order in the revised catalogue. An existing commitment cannot be relabelled or reshaped into these roles. This producer/catalogue is proposed, not currently shipped.

On the Boolean grid, let A(a,s) be the actual RAM write/address selector and I(s) the existing signed `RamInc`; require zero increments for nonwrites and inactive padding through existing CPU/RAM constraints plus segment gating. Define

`V(a,t)=B-(a)+sum_(s<t) A(a,s) I(s)` and `B+(a)=B-(a)+sum_s A(a,s) I(s)`.

Take x,y from the native RAM read/write opening. Bind `b=B-(x)`, `v=V(x,y)`, `f=B+(x)` before fresh gamma, and prove the **existing-shaped** n-round degree-three identity

`v + gamma*f - (1+gamma)*b = sum_s (LT(s,y)+gamma) A(x,s) I(s)`.

This is not a product-of-MLE shortcut: it is obtained by extending the Boolean-grid history identity. The resulting leaves are `RamRa(x||r)` and `RamInc(x||r)` with r the reversed sumcheck cycle point; the existing RAM RA and increment virtualization paths remain responsible for reducing them to the native columns. Original source J: `jolt-claims/src/protocols/jolt/relations/ram/val_check.rs::RamValCheck::{input_expression,output_expression,degree}` and `jolt-verifier/src/stages/stage4/ram_val_check.rs::RamValCheck::{derive_opening_points,derive_output_term}`. The code already has `v+gamma*val_final-(1+gamma)*init` and `LT+gamma`; replace its complete-execution init/final providers, not its algebra with an unrelated memory proof.

The direct endpoint leaves are:

| New ordinary leaf | C / physical point | Purpose |
|---|---|---|
| b | `C^-_R`, `(0^(d(m)-m)||x)` | incoming full RAM in RAM-end |
| f | `C^+_R`, same point | outgoing full RAM in RAM-end |
| optional f_io | `C^+_R`, `(0^(d(m)-m)||x_io)` | final native public-I/O output check, if its point is not already x |
| optional b_init | `C^-_R`, `(0^(d(m)-m)||x_init)` | first-segment initialization comparison, if not already x |

At segment zero, compare B- with the existing public/program/advice initial-RAM reconstruction at a verifier-random address. This gives the original advice/image source queries with their selectors, once for the run. Do not reinitialize later segments. At the final segment use the existing `RamOutputCheck` mask/value relation with B+ replacing its virtual `RamValFinal` oracle; reuse the native aligned address point when possible. J: `jolt-verifier/src/stages/stage2/ram_output_check.rs::{ram_output_check_publics,derive_opening_points}`. A conservative ledger allows both extra endpoint queries even when batching makes them identical. Nonfinal output checks must not demand final output or termination.

**Proof-side endpoint rule:** J: `jolt-witness/src/backend/trace/ram.rs::{initial_ram_state,final_ram_state}` constructs complete-run images and supplies the final public termination convention. An interior endpoint is instead the prefix recurrence from B- and the segment's proved `RamRa/RamInc`. Never invoke the whole-run final-state provider at every cut. At the global end enforce the existing final `RamOutputCheck`/public-I/O/native CPU convention once. A separate marker-event recognizer or self-loop-target theorem would strengthen the statement and is not required merely for accumulation.

## 5. CPU, register, field, device and run boundaries

### 5.1 Registers: a gamma endpoint term, not 128 new commitments

Let public R-/R+ be all 128 register values with canonical proof-field encodings. At the existing register read/write address x in F^7 and cycle y, replace the zero-initial `RegistersValEvaluation` equation with

`val + gamma*R+(x) - (1+gamma)*R-(x) = sum_s (LT(s,y)+gamma) RdWa(x,s) RdInc(s)`.

Public endpoint vector evaluations cost 128 terms or their normal MLE evaluation; there is no register endpoint commitment or bitplane source. The two leaf families are unchanged: `RdWa` and `RdInc`, at `(x||r)`, then the existing bytecode/increment reductions. Endpoints are bound before gamma. Require canonical field encodings, x0=0, the original register read/write constraints and only the domain predicates justified by the common statement; do not silently add a u64 range predicate. This changes J: `jolt-claims/src/protocols/jolt/relations/registers/val_evaluation.rs::ValEvaluation`, `jolt-verifier/src/stages/stage5/registers_val_evaluation.rs`, and J: `jolt-witness/src/backend/trace/registers.rs::materialize_register_read_write_virtual`, whose state currently starts with 128 zeros.

Retain register slots 32–39: two reservation-width state registers and mtvec, mscratch, mepc, mcause, mtval, mstatus. J: `tracer/src/utils/virtual_registers.rs::{VirtualRegisterAllocator,csr_to_virtual_register}` maps 32/33 to reservation W/D and 34–39 to those six CSR values; transient instruction registers start at 40 and larger inline temporaries at 48. Carrying all 128 avoids an unjustified dead-register argument. Do **not** add every host emulator CSR as an unconstrained public state: the target is the supported proved instruction profile, not arbitrary emulator facilities. If a reachable guest-visible state is not represented by its virtual register/RAM/instruction constraints, it is an explicit additional relation blocker.

With field-inline, apply exactly the same equation to the 16 field registers, x in F^4, with `FieldRdWa` and `FieldRdInc`. Values are canonical proof-field elements, not u64. Keep the existing full-field C_F producer. J: `jolt-riscv/src/field_inline.rs::{FIELD_REGISTER_COUNT,FIELD_REGISTER_LOG_K}`, `jolt-claims/src/protocols/field_inline/relations/registers.rs::{ValEvaluation,ReadWriteChecking}`. Host checkpoints in `tracer/src/emulator/cpu.rs::{ChunkCpuState,capture_chunk_state,install_chunk_state}` do include field registers under the feature; they are witness-generation evidence, not cryptographic endpoint checks.

### 5.2 Expanded/architectural PC, legal cuts and active padding

Select cuts only after a complete original source instruction and complete inline/virtual group; no cut inside a field operation, syscall expansion, or inline reset sequence. The public start/end expanded PC must map to the public architectural PC under the fixed expanded program. Boundary legality must be checked against the actual expanded program/inline metadata, not merely an untrusted “first” flag.

The [immutable frontend audit](AKITA_INSTANCE_ACCUMULATION_FRONTEND_MANIFEST.md), §6, closes the metadata question. Canonical instruction/inline expansion stamps the complete group **after** appending temporary-register resets. For an actual interior successor, require Boolean authenticated flags and

\[
\mathrm{NextIsVirtual}\,(1-\mathrm{NextIsFirstInSequence})=0.
\]

This admits a nonvirtual source instruction or the first row of an expanded group, and rejects interior/reset rows. It is not a halt test. Existing guarded row-local R1CS constraints do not already enforce this unguarded boundary predicate. The statement assumes the fixed program's canonical expansion/stamping or an explicit catalogue well-formedness condition; `BytecodePCMapper` alone does not certify arbitrary manually supplied first-row bits.

Full-public-program mode checks the fixed table directly. Committed-program mode uses existing lanes: UnexpandedPC 384, VirtualInstruction 393, IsFirstInSequence 398, and the IsNoop lane determined by the registered instruction-flag order. At expanded row p, select chunk `floor(p/(L/c))`, row offset `p mod (L/c)`, and logical CycleMajor coordinate `bool(lane)||bool(row_offset)` with its original padding map. Bind these same-handle queries to the actual public successor PC used in the corrected shift relation and to the next segment's authenticated first PC. Check range, program identity and flag Booleanity. At most five additional ordinary claims suffice: one start UnexpandedPC mapping and the four successor metadata values (UnexpandedPC, VirtualInstruction, IsFirstInSequence, IsNoop). Repeated or final-padding queries may be omitted only when already authenticated or public constants. No new metadata source or commitment is required; the existing one-opening-per-group adapter still needs the paid multipoint extension.

Minimal trace endpoint authentication can use existing bytecode RA columns without new endpoint trace commitments. For each bytecode chunk j, at active first and last cycles `t=0,ell-1`, prove its address moment

`digit_j = sum_(a in {0,1}^k) index(a) * BytecodeRa_j(t,a)`.

The digit-zero-omitting table correctly contributes zero for digit zero. Its existing one-hot/hamming constraints are essential: this is not a standalone uniqueness test for arbitrary tables. Reconstruct the expanded PC as `sum_(j=0..bB-1) K^(bB-1-j)*digit_j`: J: `jolt-witness/src/witnesses/one_hot.rs::RaChunkSelector::{new,chunk_u128}` uses shift `(chunks-index-1)*chunk_bits`, i.e. most-significant chunk first. `committed_address_chunks` in `geometry/dimensions.rs` pads the high address coordinates with zeros before splitting. `sumcheck` on k address coordinates gives one ordinary leaf per column at physical `(bool(t)||r_addr)`; random-linear batching all digit claims preserves one shared r_addr per endpoint, with degree at most two. These authenticate the public first/last bytecode row indices. They do not authenticate an unexecuted “next” instruction by themselves.

Modify the native `SpartanShift` relation so the last **active** row's `NextPC`, `NextUnexpandedPC`, `NextIsVirtual`, `NextIsFirstInSequence`, `NextIsNoop` use the external successor metadata, not the padded successor. Immutable `JoltTraceRow::no_op` and the PC/flag witnesses give the exact padded tuple `(0,0,0,0,1)`. Thus the first four shifted columns receive `eq(r_outer,ell-1)*external_value`; the fifth is represented as `1-NextIsNoop` and receives `eq(r_product,ell-1)*(1-external_NextIsNoop)`. Preserve the **distinct native outer and product points** and their actual gamma weights in `Shift::unbatched_relation`; do not use one point for all five. These are known public corrections to the native no-wrap shifted relation, not new source commitments or queries. For a live cut the successor is the legal next source/inline group; for a final cut it is the genuine exit convention.

Use public `ell` to construct active/suffix masks; no committed active-selector polynomial is necessary. Keep the original CPU constraints on **all** rows. Canonical padding satisfies them: instantiated `JoltInstruction::Noop` sets DoNotUpdateUnexpandedPC and IsNoop, including the sequential-PC equation `0=0+4-4`; state prefix tables stutter rather than becoming zero. Do not require `!IsNoop` on every active row. Every padded physical C_X cell is zero; in addition, each zero-omitting column's physical address-zero cells must be zero on active rows. Enforce the column-specific union mask in [main-source reuse](AKITA_INSTANCE_ACCUMULATION_MAIN_SOURCE_REUSE.md). Enforce decoded C_F padding zeros if present, without silently canonicalizing valid raw digits. [The exact consumer lowering](AKITA_INSTANCE_ACCUMULATION_SEGMENT_LOWERING.md) gives the source-backed padding and shift proof. Public expanded counters satisfy `end=start+ell` as nonwrapping integers under the total trace cap; an optional architectural counter needs its separately reduced source-instruction-start count.

### 5.3 Native I/O and advice policies

Use the **existing public native** JoltDevice contract at run endpoints: input bytes (bounded by memory layout); final outputs normalized by removing trailing zero bytes; panic boolean; fixed memory layout/preprocessing. J: `common/src/jolt_device.rs::{JoltDevice,load,store}` and `jolt-verifier/src/verifier.rs::{validate_inputs,absorb_preamble}`. `store` writes an indexed output byte and grows/zero-fills its vector; it is not append-only. Loads from input/advice/output regions use an address and return zero beyond supplied bytes. Output length including terminal zeros is not authenticated by the existing normalized contract and is deliberately **not** introduced here.

Proof-side I/O/panic/termination cells travel in B-/B+ under the same word map; there are no separate interior panic/terminal labels or four extra cell queries in the minimal interface. Do not require the interior termination word to be zero: a legal cut can fall after the SDK termination store and before its final jump. Ordered segment indices and the final statement determine when once-only final checks apply. Preserve the existing public panic value and its native final relation; a caller may additionally require nonpanic execution as part of its public statement. `JoltDevice` returns zero for termination loads and ignores termination stores, while the proof's final I/O table encodes its own termination convention. This difference must not be concealed by calling a runtime snapshot the proof-side RAM, nor turned into an unrequested emulator-equivalence theorem.

There are **three distinct advice meanings**:

1. Memory-mapped trusted/untrusted advice word objects C_T/C_U: original immutable run inputs, initialized once and queried through the RAM semantics. “Trusted” means the application/verifier supplies/accepts that commitment; do not require an extra hash/Merkle authentication protocol. Untrusted bytes are existential inputs.
2. Arithmetic/virtual advice (including field inverse/limb witnesses): existential values constrained by the actual instruction/field relations. They are not a new external tape source group.
3. SDK two-pass `AdviceTape`: J: `tracer/src/emulator/cpu.rs::AdviceTape` has byte data and a read cursor; reads are little-endian and fail past length. `jolt-platform/src/advice.rs` describes compute-advice then replay; `jolt-riscv/src/instructions/virt/advice.rs::VirtualAdvice` writes runtime advice. A resumed *witness generator* must preserve its unread suffix/cursor and never restart consumption. The default proof semantics does not certify host compute_advice. For existential trace advice, a global tape can be the concatenation of segment-consumed private bytes; no externally authenticated tape C is required. If cursor/length are exposed, check integer nonwrap and consumption-width/EOF consistency with advice-read events; add the corresponding metadata reduction to the ledger, not a fictional FIFO for JoltDevice inputs.

An application that additionally fixes an externally authenticated tape, insists on exact original tape length, or proves compute_advice execution is selecting a **different statement**. It must add that source and read/length relation explicitly. None is mandatory for the selected native semantics; its costs are not silently charged as unavoidable accumulation overhead.

## 6. Query normalization and receiver-computable caps

### 6.1 Ordinary evaluation batching is not source certification

For one original physical polynomial f, previously bound claims `(r_j,v_j)` satisfy

`sum_j alpha_j v_j = sum_z f(z) * sum_j alpha_j eq(r_j,z)`.

A degree-two multilinear sumcheck over the **same full physical domain** reduces them to one claim f(r*). For C_X use one transcript-random combination across its P separate polynomials and each polynomial's point list; the final query remains P evaluations of the P original native columns at one common r*. This does not prefix-pack C_X, change C, or assume homomorphic commitments. Retain the original `one_hot_trace_claim` result as an input to this new reduction; if no extra claims exist its existing point is retained without this additional protocol. With endpoint queries, the new final common point is derived by this authenticated reduction, not falsely passed off as the unmodified stage7 point.

An ordinary scalar query on a gadget-decoded polynomial is not, by definition, a raw short-source query. For the selected positional/direct-coefficient profile, [digit-axis normalization](AKITA_INSTANCE_ACCUMULATION_DIGIT_AXIS.md) now supplies the exact conversion and a paid same-RawView reduction to one channel. Raw shortness remains a separate predicate; a decoded bound alone still does not fix noninjective gadget maps.

Ordinary/raw normalization requires descendant-certified **same-original-C source uniqueness** across all challenge branches, with every original claim bound. Charge all claim-separation and sumcheck errors/trees. The direct digit-point conversion preserves original A/B/F geometry and both full-ring extension planes; it does not change P native columns into a prefix-packed commitment. Unsupported maps or incompatible raw views retain their explicitly priced occurrences/channels.

The selected compact implementation uses this raw-view route directly: each native ordinary P-column claim enters its own polynomial-index Boolean slice, then one paid raw normalizer yields **one scalar channel per original handle**. The preceding P-value common-point construction is a valid alternative, not an extra mandatory normalization layered before that route. All P native carrier partials and both extension planes remain in Eq. (109).

### 6.2 A concrete bounded ledger for the minimal endpoint design

The following caps refer to ordinary logical/physical evaluation claims *after existing virtual claim reductions*, before the above common-point normalization. All rows identify their original C; no direct RamInc/RdInc commitment is invented.

| Consumer | Native source leaves | Additional scalar ordinary claims, conservative cap |
|---|---|---:|
| Unmodified/revised stage1–7 main output | every C_X column at native common point | P |
| First/last expanded-PC digit moment reductions | C_X.BytecodeRa[j] at `(bool(0)||r0_addr)` / `(bool(ell-1)||r1_addr)` | 2 bB |
| Inactive suffix and omitted-address-zero union check | every C_X column at one random `(cycle||digit)` point, with column-specific weights | P |
| RAM endpoint gamma relation | existing RamRa/RamInc virtualization, then same stage6/7 native leaves | no extra groups; included in the revised P output, provided downstream reductions are updated |
| Integer-register endpoint gamma relation | existing RdWa/RdInc virtualization/bytecode paths | no extra groups; included in revised P output |
| Field-register endpoint gamma relation | existing field-register reductions and C_F | one existing C_F leaf; source count unchanged |
| FieldInc inactive suffix check (if f) | C_F at full padded point | 1 |
| RAM incoming/outgoing value claims | C_R-/C_R+ at embedded x | 2 |
| Committed-program start/successor metadata | existing bytecode chunk handles, exact Boolean coordinates above | at most 5p |
| First init/final output points if not shared | original C_R-/C_R+ | at most 2 |
| Advice/program ordinary reductions actually present | original C_U/C_T, C_B,j/C_image | at most u+t+p(c+1) |

Thus the specified ordinary cap, including committed-program boundary authentication, is

`Qordinary <= 2P + 2bB + u+t + p(c+1) + 2f + 4 + 5p`.

This is a **contract for the proposed minimal modified reductions**, not a claim that immutable stage1–7 already accept the extra inputs. In particular, RAM/register gamma terms must replace the old val-evaluation relations and preserve the existing downstream leaf count; simply appending unchecked claims does not meet the cap or relation. In continuation segments the initialized advice/image leaves may be absent; fixed bytecode groups remain present when queried. Static committed-program proof obligations cannot be dropped just because their C repeats across segments.

There is no additional committed active/PC/register/field-register polynomial in this minimal design. Unique fresh/participating physical polynomial count per segment is at most

`Ppoly = P + u+t + f + p(c+1) + 2`.

Distinct participating original C count is `G=1+u+t+f+p(c+1)+2`. At an interior boundary C_R- is a carried handle rather than a newly generated C; retain its occurrence in the fold source list regardless. After within-C normalization: one ordinary point and P values for C_X, one point/value for each present singleton, hence at most `G` point-groups and `Ppoly` scalar evaluations. Full physical logical-table volume is

`V = P*N*K + u*2^d(a_U) + t*2^d(a_T) + f*2^d(n) + p*(c*2^d(h+log2(L/c)) + 2^d(log2 W)) + 2*2^d(m)`.

This is **not** the complete Akita source length. For each group g the receiver compiler applies its original map to compute `S_g=SourceShape_g(poly_count_g,arity_g,radix_g,digit_depth_g,packing_g,physical_padding_g)`, plus its A/inner/compression-path decommitments. Record `Qraw_g` and `D_g` from the admitted SourceQueryLift profile, and use `Qtotal=Qordinary+sum_g Qraw_g`, `SourceTotal=sum_g(S_g+D_g)`. Then include the full native Eq.109 response/quotient/compression/setup witness in the receiving shape. No claim here identifies V with that witness or omits physical padding/decommitment alphabets.

Optional architectural-count or public tape-cursor reductions have explicit deltas: let `Eextra` be the fixed list of such relations, and let `q[e,g]` be the multiset size of ordinary leaves produced by its specified lowering DAG on group g. Add `sum_(e,g)q[e,g]` before normalization; add genuinely new sources only if the existing handles cannot express the relation. Freeze these counts, coordinates and formulas before admission. The mandatory committed-program boundary metadata is now included in the `5p` cap above; it needs authenticated queries and boundary checks, not an invented metadata lane. Full-public-program mode avoids those extra queries and remains the numerical receiver example.

## 7. Pending cut, branch fit and same-semantics baseline

The pending frontend cut is precisely after J: `jolt-prover/src/akita/stage8.rs` computes `one_hot_trace_claim`, performs all `reduce_precommitted` calls, constructs native auxiliary claims, and (in the modified frontend) finishes endpoint/multipoint reductions; **before** `PCS::prove_batch`. The verifier counterpart performs the exact metadata, presence, role, point, packing and frontend checks and returns `FrontPending`, not success for a complete proof. J: `jolt-verifier/src/stages/stage8/akita.rs::validate_one_hot_trace_metadata` checks native backend, K, layout digest, arity and P; auxiliary metadata requires dense backend and one physical polynomial.

The matching native Akita root is A: `akita-prover/src/protocol/prove/root.rs::batched_prove`: it creates `ProofScope`, calls `prepare_fold`, `prove_fold`, then `suffix::prove_suffix(root.next_state,...)`. A pause after the **complete** fold is source-compatible as a location but not an existing exported accumulation API. Retain/adopt live backend/session authority; dropping scope or pretending the returned opaque source is freely reusable is invalid. Do not omit relation rows, setup-prefix obligations, full Eq.109 witness, or the final verifier replay checks.

Modern branch fit is positive for (i) native multi-column main source, (ii) per-object dense auxiliary producers, (iii) separate full-field increment producer, (iv) an explicit frontend/PCS seam and root/suffix seam. Fit is **not** evidence that the old receiver plus fresh endpoint groups are catalogue-admitted. A: `akita-params/src/sis/generated_sis_table/{q128.rs,policy_audit.csv,policy_review.txt}` and `generated_l2_sis_table/q128.rs` exist at this pin, as do `akita-sis-estimator/src/{schedule_security.rs,config.rs,akita.rs}`; their existence does not certify this extended topology or equality with a different revision's tables.

The cheapest legitimate *architectural baseline path to compare*, after the **same frontend changes**, is:

1. Use the same full-public-program segment frontend, same public boundaries, same native endpoint Cs, same P-column batching, and same optional ISA features required by the application. Do not force committed program, field-inline, authenticated external tape or bitplanes onto only one side.
2. Instead of pausing/accumulating, discharge each segment's entire **ordinary** source ledger with one joint full Akita PCS proof, including the actual endpoint groups, shared application-domain/padding obligations and proper original-profile admission. Complete its normal native suffix. Do **not** add raw source-range certificates needed only by the accumulator's bounded-input/strong-completeness argument. Reuse immutable program/setup preparation and endpoint source material wherever legitimate.
3. A finite one-shot recursive guest verifies those complete modified Jolt segment proofs, checks ordered adjacent boundary identity and `Init/Final`, and returns the same native public run statement. One wrapper over the declared finite list is a legitimate baseline; recursively combining every segment separately is not inherently mandatory.

This avoids redundant independent wrappers and unnecessary online recursion. Existing independent-job conjunction is not this baseline; a wrapper needs modern-pair verification and continued-state checks. The one-shot versus tree comparison is unmeasured. Both paths pay the common segment relation, endpoint-state work and actual public-statement checks. The baseline pays full per-segment PCS and its wrapper/history; accumulation additionally pays any source-domain/raw-query certificates, repeated-fold/profile changes and final-chain work required by its construction. A logical u64/padding predicate must follow from the common execution relation or be checked on both sides; an accumulation-only short-source predicate must not be imposed on the baseline to inflate its cost.

### 7.1 Conditional concatenation lemma

Let \(\mathcal R_{\mathrm{whole}}\) be the original **Boolean-table execution relation**, before randomized sumcheck reduction, for the frozen program, memory map and public I/O. This is not a claim that per-segment challenges or MLE evaluations equal those in one monolithic proof. Let \(o_i=\sum_{j<i}\ell_j\) and concatenate only active rows, mapping segment row s to global row \(o_i+s\). Reapply the original canonical global padding convention after the last active row.

Assume each segment proves every unchanged row-local CPU/lookup/bytecode equation, the complete endpoint identities above, all required source and inactive-row predicates, and the corrected shift relation. Assume its lowering is equivalent to those Boolean-table predicates, not just that some sampled endpoint equalities happen to pass.

These assumptions now have [a reviewed lowering to the exact native consumers](AKITA_INSTANCE_ACCUMULATION_SEGMENT_LOWERING.md), including segment-relative bytecode entry, first-only initialization, last-only original output checking and physical-source zero coverage. This is a specification proof, not evidence that unchanged whole-run binaries implement those consumers.

**Forward direction.** Restrict a valid whole-run witness to legal intervals. Define each RAM/register boundary by the global proof-side prefix state at that cut. The prefix recurrence gives every segment's start-relative value table and end identity. Internal next-row metadata is exactly the first metadata of the next interval. The original initialization holds at the first boundary and final public-I/O/padded-successor relation at the last. Reconstruct each segment's canonical inactive suffix; do not keep internal global trace rows as segment padding. This supplies the semantic segment witnesses. Cryptographic commitment formation and all-valid-source completeness are separate from this trace restriction.

**Reverse direction.** For any linked satisfying segment witnesses:

1. Every active global row satisfies the original row-local equations. Public program/ISA identity and legal cuts preserve instruction expansion and inline-group interpretation.
2. Within intervals, the next-row relation is unchanged. At an internal last active row the checked external successor is the next interval's first metadata. Thus global adjacency holds without introducing a Noop between active intervals. Only the global final row uses the native padded-successor convention.
3. For RAM, with \(I_i(s)\) the segment increment,
   \[
   B_i^-(a)=B_0^-(a)+
      \sum_{j<i}\sum_{s<\ell_j}RA_j(a,s)I_j(s).
   \]
   This follows by induction from the full endpoint identity and exact same-handle/shape boundary binding. Substitution into
   \(V_i(a,s)=B_i^-(a)+\sum_{u<s}RA_i(a,u)I_i(u)\)
   yields the original global pre-write value at \(o_i+s\). Canonical inactive rows have zero contribution. The same argument applies to 128 register addresses and, when selected, 16 field-register addresses.
4. Initialization is imposed once, so the telescoped history starts at the native initial state. The final native output/panic/final-RAM checks apply to the last state once. Intermediate device flags are not additional termination tests.

These facts give a witness for \(\mathcal R_{\mathrm{whole}}\), provided the unchanged relation has no unrepresented cross-row state or absolute-cycle predicate. Such predicates must be carried/lowered explicitly; this lemma does not assert their absence from an unaudited ISA extension. No comparison of independent off-cube segment MLEs is used: interpolate their own identities first, then concatenate Boolean rows. Source uniqueness connects equal endpoint commitments before this algebraic argument.

This establishes the composition argument **conditional on exact kernel/lowering coverage**. It does not certify the yet-unwritten modified kernels, prove emulator equivalence, add a new halt theorem, or remove the PCS/accumulation knowledge theorem.

## 8. Remaining minimal unsupported changes and admission conditions

1. Export/verify the pending frontend object without invoking full PCS, while preserving exact transcript and private source ownership; add pause/adoption across native fold authority correctly.
2. Implement arbitrary-start integer/field/RAM witness generation and the three gamma endpoint identities, including native point permutation and all downstream claim wiring. The algebra is derived above; the APIs and constraints do not exist at this pin.
3. Prove concatenation into the existing whole-run relation: actual first/last rows, authenticated internal successors, legal source/inline cuts, canonical padding, no reset and nonwrapping counters. At the global final cut retain the native padded-successor/final CPU convention; do not silently strengthen it to a self-loop or emulator-halting theorem.
4. Retain original program/advice initialization once and the native normalized public-I/O/panic/final-RAM relation once. Carry proof-side endpoints, not a substituted device snapshot. Verify that all state used by the proved relation is represented in the 128+optional16 registers/RAM/control interface; host checkpoint coverage is not a proof.
5. Implement endpoint moment, padding and within-original-C multipoint reductions. Retain native P-column shape/common-point output. Extend roles/catalogue for dense RAM handles without mutating original source maps. The committed-program legal-cut metadata extension is additional work, not needed for the smallest full-public-program branch.
6. Instantiate SourceQueryLift for the currently decomposed RAM/advice/field/program producers (or select and price different new producers explicitly); Boolean physical one-hot source/padding repair remains necessary on every native column. No mandatory huge bitplane migration follows from the endpoint semantics.
7. Admit the complete fixed receiver shape, alphabets/decommitments, collision inventory, finite declared-depth extraction tree, native FS error and final history relation. The existing identity-source local proof and existing q128 table files are not that admission.

No source files, branches, commits, builds, tests, linters or formatters were changed/run. Evidence is immutable source inspection and the displayed algebra; future implementation review must check the exact generated ledger and receiver shapes against this contract.
