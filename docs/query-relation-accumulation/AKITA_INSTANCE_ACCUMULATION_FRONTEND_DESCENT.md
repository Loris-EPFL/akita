# Immutable frontend K-descent and exceptional-point review

## 1. Scope and verdict

All Jolt source links in this report are pinned to `0791f79a55c3fadc3d327c78a72228696c338468`. Local working-tree searches were used to discover symbols, then the relevant immutable files were read. In particular the immutable stage-8 module is `stages/stage8/akita.rs`; the current working tree's `packed.rs` is not an immutable-source citation.

I read the full FRONTEND_EXTENSION audit before following the graph. This report concerns the unchanged whole-run Boolean-table relation and its evaluation over E, not emulator equivalence, halting, exact external tape semantics, ZK, QROM, or a concrete 128-bit reduction.

**[INFERENCE—mathematical conclusions from the source identities]**

1. There is a bottom-up, deterministic K reconstruction for every execution-relevant virtual Boolean table. No inspected virtual-table class requires a genuinely existential E-valued execution witness that lacks descent.
2. Several enum entries and kernel buffers are intentionally E-valued after challenges: staged bytecode values, uni-skip values, partial sums, partial bindings, and equality-weighted tables. They are deterministic protocol intermediates, not extra K witness tables. Requiring these scalars themselves to lie in K would be incorrect.
3. K-descent is a statement about a complete source-linked extraction tree. It is not true that one accepting random transcript individually certifies all virtual tables. The complete original frontend reductions, their coefficient/reference challenges, and the same-original-source comparison supply precisely the missing linkage.
4. The inspected Booleanity, instruction-RA, Gruen, Lagrange and inactive-round divisions introduce no additional completeness bad event or rational-function interpolation family. Their exceptional branches implement the same polynomial identities everywhere in E.
5. **The added physical-main question has a concrete negative answer as stated:** the existing complete 6a/6b/7 tree does not constrain physical address-zero cells of the zero-omitting families. It does constrain all their other logical coefficient cells and all RAM coefficient cells. A physical-hole zero check can close this without a fresh full Boolean prefix, but honest sparse-source construction alone cannot.

## 2. Correct extraction interface: four types of nodes

The following distinctions make the descent argument precise.

- **OriginalSourceK:** the original physical K coefficient/decommitment source behind each admitted handle. A frontend opening is not a replacement source or a canonical re-decomposition.
- **SemanticTableK:** a deterministic finite K table obtained from those leaves by a registered reconstruction, linear map, product, shift, lookup contraction, or recurrence.
- **ProtocolValueE:** evaluation or partial reduction of those fixed tables using E challenges. These values generally do not lie in K.
- **Alias:** another name for exactly the same table evaluation, with either a structural point identity or the source's explicit runtime point-equality condition.

For any K table w, use `MLE_E(w,r) = sum_x iota(w[x]) eq_E(x,r)`. K-linear table maps, K-coefficient products on Boolean rows, and fixed shifts commute with the embedding. Partial binding is E-valued but equals the partial binding of this same MLE. Do not identify the MLE of a Boolean-row product with the product of the two MLEs away from the Boolean cube: that distinction is exactly why the product/virtualization sumchecks are present.

**Local extraction lemma.** Suppose a reduction's complete supplied tree has enough distinct challenge children for its registered degree/arity, and its leaf evaluations are all evaluations of the same already-identified original sources. Reverse sumcheck interpolation and separation of the registered RLC/reference coordinates recover the stated Boolean-domain identity. If that identity defines the upstream table by a K-valued expression in downstream tables, reconstruct that table over K and assign its MLE to every upstream claim. Proceed in reverse dependency order. Equality of K results in E implies equality in K by injectivity.

This does not divide by a randomly sampled gamma or equality factor. It separates the entire polynomial family. It also does not independently posit a free virtual E oracle at each branch: the downstream reconstruction is fixed by the same original source across branches. Without that same-source premise, neither this descent lemma nor reuse of frontend Booleanity is justified.

## 3. Exhaustive virtual/source inventory

The ordinary registry is exhaustive in [`JoltVirtualPolynomial` and `JoltCommittedPolynomial`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/ids.rs). [`TraceBackend::shape_of` / `oracle_table`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-witness/src/backend/trace/oracle.rs) use total matches and explicitly distinguish source tables, protocol intermediates and unserved vocabulary. The following inventory covers every ordinary virtual variant and all nine field-inline variants.

### 3.1 Native physical leaves and digit-zero-inclusive semantic columns

Let A=2^k be the digit address alphabet, avoiding confusion with the base field K. Let X_i(a,t) be a physical K main column.

- `InstructionRa(i)`, `BytecodeRa(i)`, `BalancedIncDigit(i)`, `BalancedIncCarry`: their semantic columns R_i are
  - R_i(a,t)=X_i(a,t) for a != 0;
  - R_i(0,t)=1-sum_{a!=0} X_i(a,t).
- `RamRa(i)`: all address rows, including a=0, are physical; semantic R_i=X_i.

These are K-valued deterministic maps even before Booleanity. Source: [`LatticeDigitZeroClaimReduction::{input_expression,output_expression}`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/lattice/relations/digit_zero.rs), and [digit-zero specification §§2–4](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/specs/digit-zero-virtualization.md).

`LatticeBooleanity` checks every semantic R_i, including the balanced carry, by the shared polynomial `eq * sum_i gamma^(2i) (R_i^2-R_i)`. See [`lattice/relations/booleanity.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/lattice/relations/booleanity.rs) and [`geometry/booleanity.rs::booleanity_output`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/geometry/booleanity.rs). Since z(z-1) has exactly roots 0,1 in E, it forces K-valued Boolean semantic entries once extracted; it does not require a subfield-membership test.

The ordinary enum's `Committed(RdInc)` and `Committed(RamInc)` are not independent native-Akita leaves. The selected native path reconstructs them from the digit/carry leaves and bytecode store selector below. Advice, committed-program objects, and optional `FieldRdInc` are separate original K leaves.

### 3.2 `FusedInc`, `RamInc`, `RdInc`

[`lattice.rs::balanced_inc_value`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/lattice.rs) defines the centered digit table d(a): unsigned identity minus 2^k times its high bit. In particular d(0)=0.

Define

`FusedInc(t) = sum_j 2^(k*j) sum_a d(a) X_digit[j](a,t) + 2^64 sum_a d(a) X_carry(a,t)`.

This is K-linear in physical leaves. The stage-7 decode leg enforces its claimed evaluation; zero-address terms vanish because d(0)=0. The native read-RAF extension then enforces

`RamInc(t) = Store(t) * FusedInc(t)` and `RdInc(t) = (1-Store(t)) * FusedInc(t)`.

Source: [`lattice/relations/read_raf.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/lattice/relations/read_raf.rs), especially its four fused-inc consumer stages at outer gamma exponents 5..8; [`geometry/bytecode.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/geometry/bytecode.rs) supplies the store value or staged committed store lane.

The enum comment calling FusedInc gamma-batched must not be read as a challenge-dependent execution table. [`witnesses/increments.rs::FusedInc`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-witness/src/witnesses/increments.rs) is the single selected per-cycle delta; the gammas batch its consumer claims.

Do not infer the honest `|delta|<2^64` or carry in {-1,0,1}. The source explicitly warns that the verified balanced digit/carry alphabet is the entire centered A-symbol alphabet. K-descent needs no stronger bound.

### 3.3 Bytecode pullback class

This class contains `PC`, `UnexpandedPC`, `Imm`, all consumed `OpFlags(CircuitFlags)`, all consumed `InstructionFlags(InstructionFlags)`, `LookupTableFlag(i)`, `InstructionRafFlag`, and the integer access tables `Rs1Ra`, `Rs2Ra`, `RdWa`.

Let B(a,t) be the product of the semantic bytecode digit selectors. Each table is a fixed bytecode-table pullback:

- `PC(t)=sum_a a B(a,t)`;
- scalar/flag column `f(t)=sum_a Bytecode_f(a) B(a,t)`;
- register selector `Rs1Ra(r,t)=sum_a Bytecode_rs1_selector(r,a) B(a,t)`, and similarly rs2/rd.

These are K-valued even if considered as algebraic sums rather than a host-decoded trace. Full-public mode evaluates the original K tables directly. Committed-program mode takes the same lanes from the original K `BytecodeChunk(i)` sources.

Source: [`geometry/bytecode.rs::read_raf_row_values`, `read_raf_stage_values`, `read_raf_public_values`, `read_raf_raf_entry_publics`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/geometry/bytecode.rs); [`geometry/claim_reductions/bytecode.rs::BytecodeLaneLayout`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/geometry/claim_reductions/bytecode.rs).

The five stage folds separate these particular columns, not an abstract generic flag oracle. Stage-2 `write_lookup_output_to_rd` and `virtual_instruction` are intentional wire passengers: [`ProductRemainder::wire_output_openings`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage2/product_remainder.rs) states that they are absorbed there and constrained downstream by stage-6a bytecode read-RAF. They are not unconstrained existential auxiliaries.

### 3.4 Register state and value class

`RegistersVal(r,t)=sum_{s<t} RdWa(r,s) RdInc(s)` in the original zero-initialized whole run. Then

- `Rs1Value(t)=sum_r Rs1Ra(r,t) RegistersVal(r,t)`;
- `Rs2Value(t)=sum_r Rs2Ra(r,t) RegistersVal(r,t)`;
- `RdWriteValue(t)=sum_r RdWa(r,t) (RegistersVal(r,t)+RdInc(t))`.

The exact algebra is single-sourced in [`twist/memory_checking.rs::val_evaluation_output` and `read_write_checking_output`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/twist/memory_checking.rs), with register protocol instantiations in [`relations/registers/val_evaluation.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/relations/registers/val_evaluation.rs). Each table is uniquely K-valued from previous nodes. The degree-three value sumcheck is an explicit prefix-sum definition, not an existential unconstrained memory history.

For the proposed segment variant, adding the authenticated incoming K state preserves the same argument. This observation does not certify the unimplemented segment lowering.

### 3.5 RAM state, address and values

`RamRa(a,t)` is the product of the full physical RAM digit selectors. Stage-6b [`RaVirtualization`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/relations/ram/ra_virtualization.rs) links the virtual table to those factors.

`RamHammingWeight(t)` is the sum over each RAM digit column's addresses; stage7 links these sums to the same hamming table and stage6b checks H^2-H=0. It is thus a K-valued linear function of a physical column, with additional consistency/Booleanity constraints, not a free E activation witness. This is the unchanged full-RAM treatment, not the deferred Load+Store digit-zero scheme.

`RamAddress(t)=sum_a UnmapAddress(a) RamRa(a,t)` uses the original public K unmapping table. [`RafEvaluation`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/relations/ram/raf_evaluation.rs) includes the explicit 2^gap dummy-cycle factor.

Let I(a) be the original initial RAM table assembled from public input/layout data and present original advice/program-image sources. Then

- `RamVal(a,t)=I(a)+sum_{s<t} RamRa(a,s) RamInc(s)`;
- `RamValFinal(a)=I(a)+sum_s RamRa(a,s) RamInc(s)`;
- `RamReadValue(t)=sum_a RamRa(a,t) RamVal(a,t)`;
- `RamWriteValue(t)=sum_a RamRa(a,t) (RamVal(a,t)+RamInc(t))`.

The gamma-separated value/final identity is in [`relations/ram/val_check.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/relations/ram/val_check.rs); [`RamValCheck::derive_output_term`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage4/ram_val_check.rs) evaluates the actual strict-less-than cycle polynomial plus gamma. Read/write identities are explicitly separated in [`ReadWriteChecking::unbatched_relation`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/relations/ram/read_write_checking.rs).

No division, ordering of E, or host-u64 premise occurs in these reconstructions. They preserve the full-field K endpoint domain.

### 3.6 Instruction lookup and operands

`InstructionRa(i)` is the virtual wider selector obtained by multiplying its group of physical/semantic committed instruction digit selectors on Boolean cycle rows. The stage-6b instruction-RA reduction enforces this grouping.

Let Q(a,t) be the product of the virtual instruction-RA factors, H(t)=InstructionRafFlag(t), L(a), R(a) the original operand tables, Id(a) the identity table, and T_j(a) the original fixed lookup tables. Coefficient separation in stage5 gives:

- `LookupOutput(t)=sum_a Q(a,t) sum_j LookupTableFlag_j(t) T_j(a)`;
- `LeftLookupOperand(t)=sum_a Q(a,t) (1-H(t)) L(a)`;
- `RightLookupOperand(t)=sum_a Q(a,t) ((1-H(t)) R(a)+H(t) Id(a))`.

These formulas follow directly by collecting gamma^0, gamma^1 and gamma^2 in [`InstructionReadRaf::derive_output_term`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage5/instruction_read_raf.rs#L160-L220) and the [`symbolic relation`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/relations/instruction/read_raf.rs). The extra Akita gamma^3 coefficient enforces `sum_a Q H upper_half_all_ones(a)=0`; it must remain. Moving challenges to E does not eliminate the K source-representation collision that motivated it.

`LeftInstructionInput = LeftOperandIsRs1Value*Rs1Value + LeftOperandIsPC*UnexpandedPC`, and `RightInstructionInput = RightOperandIsRs2Value*Rs2Value + RightOperandIsImm*Imm`; see [`InputVirtualization::unbatched_relation`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/relations/instruction/input_virtualization.rs).

All constituents are K-valued Boolean tables. The fixed-table interface separates integer `materialize_entry` from field-generic MLE evaluation; challenge points are never bit-decomposed to create a different table.

### 3.7 Product and shifted columns

The ordinary product lanes are exactly:

- `Product = LeftInstructionInput * RightInstructionInput`;
- `ShouldBranch = LookupOutput * InstructionFlags(Branch)`;
- `ShouldJump = OpFlags(Jump) * (1-NextIsNoop)`.

Source: [`ProductUniskip::input_expression`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/relations/spartan/product_uniskip.rs) and [`ProductRemainder::{left_factor_expression,right_factor_expression}`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/relations/spartan/product_remainder.rs). Centered-Lagrange extraction at the lane nodes separates these products; it does not infer a product identity from one random lane combination.

`NextPC`, `NextUnexpandedPC`, `NextIsVirtual`, `NextIsFirstInSequence` are the original successor-shift maps with zero at the missing successor. `1-NextIsNoop` is the corresponding shift of `1-IsNoop`, so `NextIsNoop` has terminal value one. See [`Shift::unbatched_relation`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/relations/spartan/shift.rs). The first four legs use the outer reference and the fifth uses the product reference; do not merge those references.

The R1CS linear forms Az/Bz and optimized product-grid partial products are deterministic linear/product workspaces built from these tables. They add no independent execution witness. Remaining CPU constraints test the reconstructed K tables; they are not the mechanism used to guess an E solution and hope it descends.

### 3.8 Field-inline: all nine virtual tables

[`FieldInlineVirtualPolynomial`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/field_inline/ids.rs) contains exactly:

`FieldRs1Value`, `FieldRs2Value`, `FieldRdValue`, `FieldProduct`, `FieldInvProduct`, `FieldRs1Ra`, `FieldRs2Ra`, `FieldRdWa`, `FieldRegistersVal`.

- `FieldRdInc` is the original committed K increment leaf, including all-zero executions.
- The three access tables are bytecode pullbacks of the public field operand selectors. [`field_inline/geometry/bytecode.rs::read_raf_row_values`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/field_inline/geometry/bytecode.rs) adds the three field read/write selectors at stage4 powers 3,4,5 and FieldRdWa to stage5's appended power.
- `FieldRegistersVal`, the two read values and the post-write value follow the same prefix-sum/read-write equations as integer registers, using FieldRdInc. [`field_inline/relations/registers.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/field_inline/relations/registers.rs) literally instantiates the shared Twist macros.
- `FieldProduct=FieldRs1Value*FieldRs2Value`; `FieldInvProduct=FieldRs1Value*FieldRdValue`. Both lanes are included in [`selected_product_lanes`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/field_inline/geometry/product.rs), not just the ordinary multiplication lane.

Thus even the inverse output is already K-valued via the original K increment recurrence. The execution inverse constraint additionally requires its product with the K input to be one. If the input is nonzero, the unique E inverse is the embedded K inverse; if zero, the guarded inverse equation is unsatisfiable. This is not a challenge bad event.

[`field_constraints.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/composed/r1cs/field_constraints.rs) also makes `FieldAdviceLimb` a field quotient: `FieldRs1Value = RdWriteValue + 2^64 FieldRdValue`. The radix is nonzero in characteristic q. No integer-division or canonical-readout theorem follows from that row.

Decode execution bytes with K, as in [`field_inline/witnesses.rs::decode_value`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-witness/src/field_inline/witnesses.rs), before lifting. Reinterpreting the execution buffer as E coefficients would change the source relation.

### 3.9 Advice and committed-program alternatives

- Trusted/untrusted advice are original K word/decomposition sources. The stage4 initialization contribution is a fixed K table insertion/restriction. On Akita the corresponding claim goes directly to stage8; it does not use the ordinary non-Akita advice cycle/address reductions. Exact source: [`stage8/akita.rs::leaf_claims`, lines 518 onward](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage8/akita.rs#L518-L534).
- In committed-program mode `BytecodeChunk(i)` and `ProgramImageInit` are original K preprocessing sources. `BytecodeValClaim(i)` are challenge-weighted staged evaluations of their K lanes, not new free tables. Akita has six staged values, the sixth being store. [`bytecode::CyclePhase`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/relations/claim_reductions/bytecode/cycle_phase.rs) batches them by eta and reduces through `BytecodeClaimReductionIntermediate` or directly to the original chunk openings.
- `ProgramImageInitContributionRw` is the shifted/embedded image-table contribution to RAM initialization. Its [`cycle phase`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/relations/claim_reductions/program_image/cycle_phase.rs) and optional address phase end at the same original ProgramImageInit leaf. [`precommitted_final_openings`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage8/precommitted.rs) selects the completing phase and checks chunk count/presence.

Field-inline plus committed program is not available at this pin: [`validate_inputs_from_parts`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/verifier.rs) requires `program.as_full()` under field-inline and calls `committed_program_rejection` otherwise. Advice can accompany either supported alternative.

### 3.10 Remaining enum classes

- `UnivariateSkip`: deterministic E reduction values for the outer/product lane polynomial, resolved by their continuation sumchecks.
- `BytecodeReadRafAddrClaim`, `BooleanityAddrClaim`: deterministic E partial sums carrying phase6a into phase6b.
- `BytecodeValClaim(i)`, `BytecodeClaimReductionIntermediate`, `ProgramImageInitContributionRw`: the E reduction nodes just described.
- `Rd`, `InstructionRaf`, `RamValInit`: explicitly unserved vocabulary, with `UNSERVED_REASON = no consumer on the modular stack; no trace derivation is defined`. The actual initialization table is the verifier's public/advice/image construction, not an existential RamValInit opening.

These entries do not create missing K witnesses. In particular a challenge-dependent bytecode staged table can have E entries even at Boolean addresses; its type is a linear-combination/evaluation workspace over fixed K lanes, not an original semantic table that must descend.

## 4. Alias coverage is load-bearing and present

The pinned concrete declarations include:

1. Stage2 InstructionClaimReduction: `LookupOutput`, `LeftInstructionInput`, `RightInstructionInput` alias the product-remainder values at the same reversed batch suffix. [`instruction_claim_reduction.rs::aliased_output_openings`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage2/instruction_claim_reduction.rs).
2. Stage2 field-register reduction: all three field values alias their field-product outputs. [`field_registers_claim_reduction.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage2/field_registers_claim_reduction.rs).
3. Stage3 InstructionInput: UnexpandedPC aliases SpartanShift. [`instruction_input.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage3/instruction_input.rs).
4. Stage3 RegistersClaimReduction: Rs1Value/Rs2Value alias InstructionInput. [`registers_claim_reduction.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage3/registers_claim_reduction.rs).
5. Stage6b: Booleanity's BytecodeRa value is deduplicated only when its complete derived point equals the bytecode-read-RAF point. [`validate_bytecode_ra_aliases`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage6b/verify.rs#L544-L580) explicitly rejects unequal wire values on such aliases.

[`validate_member_aliases`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/relations.rs#L432-L474) compares values; generated batch final-claim evaluation invokes alias validation. Structural aliases do not compare points at runtime: their proof obligation is the identical batch suffix and reversal, which the inspected concrete point constructors supply. E preserves those point identities and exact equality comparisons. Do not remove equality validation merely because an aliased copy is omitted from absorption.

## 5. Exact exceptional-point audit

### 5.1 Booleanity gamma inverse/unscale

Source: [`OptimizedBooleanityCycle::prepare` and `OptimizedBooleanityCycleKernel`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-kernels/src/optimized/booleanity.rs).

Let rho_i=gamma^i and x_i(t) be the address-folded semantic column. For nonzero gamma, the lazy table stores H_i=rho_i x_i. The inner quadratic uses

`H_i(H_i-rho_i)=gamma^(2i)(x_i^2-x_i)`.

Opening recovery multiplies by gamma^(-i). The only denominator is gamma.

At gamma=0 the source does **not** lose all disabled openings:

- `opening_unscale` becomes an all-one vector;
- rho_i=0 columns retain the unscaled address equality table;
- both categorical and ordinary message paths process only the nonzero-rho prefix, namely column zero;
- output extraction returns every retained column, including disabled columns, unscaled.

This exactly implements the original fold at gamma=0. No honest abort, no bogus zero opening, no new exclusion rule. Gamma=0 can make a false aggregate vanish, but that is already a root of the polynomial of degree 2(P-1), not a second independent failure event. Retain interpolation arity **2P-1**, not P. Each address/cycle sumcheck round remains degree3, arity4.

### 5.2 Another inverse site: instruction-RA gamma

[`OptimizedInstructionRaVirtualizationKernel::new`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-kernels/src/optimized/instruction_ra_virtualization.rs#L140-L240) uses `gamma.inverse().unwrap_or_else(F::one)`.

Each virtual group carries its gamma^v coefficient in its first committed-factor table. For gamma=0, the source leaves disabled first-factor tables unscaled, sets inverse powers to one, and restricts messages to `active_virtuals=1`. All required committed openings remain recoverable. This is the analogous exact totalization, not an error suppression.

If v is the number of virtual selectors, coefficient degree is v-1 and arity v. If a virtual selector has c committed factors, cycle degree is c+1 and arity c+2. The c=1 implementation explicitly samples the quadratic at 0,1,2 instead of using an invalid Toom recovery. No extra exceptional charge.

### 5.3 Correction: lazy_ra has no inactive-factor inverse

The complete pinned [`optimized/lazy_ra.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-kernels/src/optimized/lazy_ra.rs) uses only gathers, sums and branch-table multiplication by r or 1-r. At r=0 or r=1 branches disappear normally. Its fourth-bind dense conversion is an exact regrouping of multilinear interpolation. There is no inverse there.

The cited inactive-round error messages actually belong to [`optimized/read_write.rs::RamAddressKernel::wrap`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-kernels/src/optimized/read_write.rs#L65-L164). Exact denominators are **2** and **2^gap**, where gap is `phase3_cycle_rounds()`.

An inactive round emits the constant `claim/2`; its bind multiplies scale by 1/2 and unscale by 2 without altering source tables or opening coordinates. Active messages are inner messages evaluated with claim*unscale, then multiplied by scale. In characteristic q these constants are nonzero for every allowed gap. The errors are invariant checks for unsuitable characteristics, not random challenge bad events. Inactive-round degree is zero; batch padding/round placement stays as already registered.

### 5.4 Gruen: exact zero branches and denominators

The pinned symbols are [`GruenSplitEqPolynomial::gruen_poly_deg_3`, `gruen_poly_from_evals`, `gruen_poly_deg_2`, `recover_q_zero`, `zero_round`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-poly/src/split_eq.rs#L360-L480). The audit's `SplitEqPolynomial`, `gruen_poly_deg3`, and `recover_q_endpoints` are not the exact inspected symbols.

At a round let c be the carried equality scale and u the remaining reference coordinate. The linear factor is

`l(X)=c[(1-u)(1-X)+uX]`, so `l(0)=c(1-u)`, `l(1)=cu`.

- Cubic recovery knows q(0) and q's quadratic coefficient and computes q(1) from `(hint-l(0)q(0))/l(1)` when `l(1)!=0`.
- If `l(1)=0` but c!=0, the actual q(1) is computed by the caller's endpoint callback. The code checks the endpoint sum equals the hint; disagreement returns Err(actual).
- Toom and degree2 recovery know q(1) and recover q(0) by `(hint-l(1)q(1))/l(0)` when `l(0)!=0`.
- If `l(0)=0` but c!=0, the callback computes q(0), and the endpoint sum is checked.
- If c=0, l is identically zero. `zero_round` returns the zero polynomial only for a zero hint, otherwise Err(0). It does not scan the unused inner table.

Binding updates c by multiplication with `1-u-r+2ur`; it never divides by that factor. Thus a zero carried equality factor is handled even when neither coordinate is Boolean.

[`GruenRoundMessage` checked wrappers](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-kernels/src/optimized/support.rs#L524-L638) propagate these errors as `SumcheckError::RoundCheckFailed`. The Booleanity and instruction-RA callbacks inspected compute the actual omitted endpoint.

**Local totalization lemma:** on a correct previous claim the output is the same polynomial lq for every u,c in E. Therefore cubic arity remains4; a degree-d inner polynomial produces degree d+1, arity d+2; the linear-inner case has arity3. There is no additional interpolation factor for the internal branch and no fresh bad-event charge. A sampled zero equality factor can mask a false claim only through the already-counted polynomial test; do not infer soundness by dividing that one transcript's terminal equality.

`UnivariatePoly::from_linear_times_quadratic_with_hint` separately has an explicit panic when l(1)=0. It is not the helper called by the inspected pinned Gruen implementation. Replacing the checked helpers with that unguarded helper would reintroduce a real completeness defect. Do not cite it as proof that the current checked paths abort.

### 5.5 Lagrange: all denominator types

Source: [`lagrange.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-poly/src/lagrange.rs).

For distinct integer nodes a_i and V(X)=product_i(X-a_i):

- `lagrange_evals` first returns the one-hot vector when r=a_i;
- constant denominator `product_{j!=i}(a_i-a_j)=(-1)^(N-1-i) i! (N-1-i)!` is nonzero because all registered domains are smaller than q;
- off-node denominators r-a_i are nonzero by the preceding branch;
- optimized `LagrangePolynomial` batch inversion divides by products of the same constant denominators and by V(r);
- its barycentric sum denominator is `sum_i 1/[V'(a_i)(r-a_i)] = 1/V(r)`, hence cannot vanish off-domain;
- `lagrange_kernel(x,y)` branches when either argument is a node; when both are off-domain its product-of-sums denominator is also nonzero;
- divided differences/Newton interpolation divide only by small integer steps/factorials.

Thus the barycentric routines are exact polynomial evaluations throughout E, not rational tests needing a union bound over an exceptional set. L_i has degree N-1, and the centered kernel has degree N-1 in each argument. Keep the existing outer centered domain N=10+5f and product domain N=3+2f, and the already-registered uni-skip degrees. No resampling of node hits is justified or needed.

## 6. Added question: can existing Booleanity certify EVERY physical coefficient?

### 6.1 What the complete tree actually certifies

Let X_i be the same physical direct scale-one K columns identified through the native fold/terminal tree. Stage7 extracts the semantic-to-physical identities above. Stage6a/6b plus the original ancestor reference tree then extract

`R_i(a,t)(R_i(a,t)-1)=0`

for every column i, every a in the full k-bit digit cube, and every t in the entire n-bit cycle cube.

Consequently:

- For InstructionRa, BytecodeRa, BalancedIncDigit and BalancedIncCarry: **every X_i(a,t) with a!=0 is Boolean**.
- For RamRa: **every X_i(a,t), including a=0, is Boolean**.
- This covers padding cycles too. The sumcheck domain is all T cycles, not just the active prefix.
- It covers nonzero digit rows in a rounded high chunk even if those rows are not meaningful program/RAM addresses. Booleanity uses the complete k-coordinate digit domain. This establishes Booleanity, not a new assertion that every such unused address row is zero.
- It needs no occupancy bound. It derives coefficient Booleanity directly from z(z-1), not from an honest one-hot source representation.

The Booleanity reference address/cycle coordinates are reused from stage5, not independently resampled at stage6a. [`BooleanityAddressPhase::draw_challenges`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage6a/booleanity.rs) reverses/truncates the instruction address; legal k=4,8 needs no extra coordinate draws. The full jointly extracted frontend tree must therefore include those ancestor challenge families. One fixed published stage5 point is insufficient.

### 6.2 Concrete physical address-zero obstruction

For a zero-omitting column the extracted relation is

`R_i(r,t)=eq(r,0)+sum_a [eq(r,a)-eq(r,0)] X_i(a,t)`.

At a=0 the coefficient is identically zero. The increment decode also has d(0)=0. This is visible directly in [`digit_zero.rs::output_expression`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/lattice/relations/digit_zero.rs), not merely in a producer assumption. The optimized stage7 implementation explicitly zeroes those pushforward positions and builds weights zero at address zero: [`hamming_weight_claim_reduction.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-kernels/src/optimized/hamming_weight_claim_reduction.rs#L207-L335).

**Counterexample to the proposed inference:** start from valid semantic tables and physical nonzero-address entries. For any zero-omitting family and one active cycle t0, replace the physical coefficient X_i(0,t0) by 2. Leave all other physical coefficients unchanged. Every reconstructed semantic table is unchanged. Every upstream execution and Booleanity identity is unchanged. Every stage7 initial identity is unchanged. A prover can compute the exact degree-two stage7 sumcheck against this modified physical table and supply its actual final opening. Complete interpolation does not remove this freedom: the Boolean-address product of the added zero-row table with every stage7 weight is zero.

This is a kernel of the physical-to-semantic map **inside K**. It does not rely on free E virtual witnesses, a failed gamma draw, or a PCS collision. It proves that frontend Booleanity alone cannot establish the required all-physical source alphabet. It is not, by itself, a claimed forgery of the complete proposed accumulator, whose other predicates must be considered separately.

The native [batching specification](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/specs/akita-native-trace-batching.md) says exactly P actual columns, no selector-capacity padding, and preserved digit-zero semantics. Its preserved `{0,1}` bound is consistent with [`TraceOneHotColumn::committed_centered_reach`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-akita/src/trace_onehot/source.rs): the streamed honest source emits only zeros/ones. That method and the restricted Rust producer are not an adversarial proof that every extracted full physical source coefficient meets the bound.

### 6.3 Padding and exact conditions for safe reuse

The immutable native layout is exactly P tables of NK entries, ordered `InstructionRa`, balanced digits, carry, `BytecodeRa`, `RamRa`; see [`one_hot_trace_columns`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/lattice/packing.rs) and [`OneHotTraceLayout::plan`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/lattice/strategy.rs). There are no extra physical polynomial slots up to nextpow2(P). A RawView's artificial polynomial-index holes may be defined zero without a proof because they are not committed cells.

But any **actual** ring/block/source padding beyond the logical NK coefficient region is a different matter. Ordinary logical frontend openings do not necessarily observe it. A delta=1, scale-one direct profile removes gadget-digit ambiguity, but not unobserved physical positions. Its admission certificate must say either that the logical-to-raw coefficient map is a bijection onto all actual coefficients, or explicitly list and constrain the additional coefficients. Never turn real source padding into virtual RawView holes by wording.

There are two sound routes:

1. Keep the proposed full fresh-main raw Boolean prefix, plus whatever zero predicates are required by the intended source semantics.
2. Reuse existing Booleanity for the covered cells and separately certify the omitted address-zero cells and all actual padding. To preserve the existing omission rule, require the omitted cells to be **zero**, not merely Boolean.

A useful concrete integration is to strengthen the already-proposed main inactive-zero check. For each column i define a Boolean-domain mask Z_i covering the inactive cycle suffix, plus all digit-zero addresses for the zero-omitting families, plus actual physical padding where applicable. Prove

`sum_i alpha_i sum_x MLE(eq(tau,.) Z_i(.))(x) X_i(x)=0`.

Use the MLE of the restricted equality table, not a product of off-cube masks. With per-column masks the final verifier weights are column-specific; it is not literally one common masked polynomial times the combined source. The integrand remains degree2, with the same P independent coefficients and P source leaves at the resulting point. For an unchanged exact physical domain this costs the same `3*mu+P` numerator and `2^mu(P+1)3^mu` tree factor as the existing main zero-check family. Extending the mask is a real predicate change that the parent must explicitly include, not an already-implemented source fact. If actual source padding lies outside ordinary polynomial coordinates, use the appropriate original-source query view instead.

Once that zero coverage and the same-original-source comparison are in the joint tree, all actual scale-one delta=1 main coefficients are 0/1, and the extra full-main Boolean prefix can be removed. The current inactive-cycle-only mask does not establish this: the counterexample uses an active-cycle zero-address cell.

### 6.4 Same-original-source linkage needed for reuse

[`stage8/akita.rs::leaf_claims` and `one_hot_trace_claim`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage8/akita.rs#L386-L550) enumerate all P stage7 leaves, reject duplicate/missing leaves, permute `(address||cycle)` into `(cycle||address)`, require a common point and correct arity, and assemble one native group against the supplied original commitment. The layout metadata fixes exact count/order/domain.

The new accumulation theorem must identify each such branch leaf with the original source extracted by the fold descendants, not just with some table having the same realized scalar opening. With that established, the local Booleanity/reconstruction proof above is valid. Without it, neither the full-main prefix nor frontend reuse can be promoted to an original-source bound.

## 7. Manifest consequences and actual remaining failures

### Closed mathematical items

- Semantic-table K-descent can be stated as the typed reconstruction/extraction lemma above. Every relevant ordinary/field virtual class has a concrete producer reduction; the dependency graph bottoms out in original K leaves and public K tables.
- The inspected divisions are total or by fixed nonzero constants. The manifest needs no extra exceptional probability or tree factor for gamma-zero unscaling, Gruen endpoint fallbacks, Lagrange node hits/barycentric sums, or inactive-round 2-powers.
- Keep the ordinary degree/arity ledger, especially Booleanity gamma degree `2(P-1)`/arity `2P-1`, instruction-RA gamma arity v, and stage7's decode exponent `2(bI+bB)+3bR+bD+1`/arity one larger. A fallback does not permit reducing these arities.
- Keep the source's actual error semantics and no new rejection sampler for zero/one/node challenges.

### Concrete failures/limitations, not Rust-generic TODOs

1. **All-physical main Booleanity does not follow from existing semantic Booleanity:** omitted physical address-zero cells are an explicit counterexample. Extra physical padding also needs coverage. This is the substantive remaining mathematical obstruction discovered here.
2. **One transcript is not a descent proof:** the complete mixed-tree, source-projection/uniqueness premise remains essential. This report provides the local extraction implication, not a new proof of the global ROM/supplied-tree theorem.
3. **Field-inline/committed-program combined mode is unsupported:** it is rejected by immutable verification and must not appear as a reviewed profile.
4. **Literal E migration still cannot reuse base-only decoding or 16-byte sampling unchanged:** those are implementation/protocol-framing barriers already accurately identified by FRONTEND_EXTENSION, not an unclosed algebraic descent problem.
5. **Segment recurrence/endpoint lowering and exact registry admission remain separate:** the K reconstruction lemma extends to authenticated K incoming states, but this review does not assert that the newly proposed segment predicates are already implemented or fully source-lowered.
6. **Do not strengthen domains from producer facts:** neither u64 intermediate RAM, a narrow balanced carry, physical omission zeros, nor source-block padding zeros follows merely because honest encoders generate them.

Suggested document corrections are therefore precise: replace the open semantic-descent question with the typed DAG/local lemmas and their global source-link premise; correct `lazy_ra` to `read_write::RamAddressKernel` for inactive inverses; name the actual Gruen helpers and record their total branches; add instruction-RA gamma unscaling; distinguish semantic Booleanity from physical-source hole coverage; and only remove the extra fresh-main Boolean prefix after registering the missing zero/padding coverage.

## 8. Evidence boundary

Performed source/document reads and searches only. No code or document edits, branches, builds, tests, linters, formatters, proof executions or benchmarks. Test code encountered in source was not executed. The conclusions labelled mathematical inference are derivations from the pinned identities, not claims of implemented mixed-K/E support or complete admission of the global design.