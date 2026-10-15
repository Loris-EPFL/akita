# Consumed advice bytes: a finite local Jolt certificate for the Akita receipt

## 1. Result and scope

**The existing subword-load constraints close the previously identified local word-range gap.** A completed `LBU/LB/LHU/LH/LWU/LW` expansion feeds its **entire containing RAM word** into an interleaved PEXT lookup operand. That operand has a unique 64-bit Boolean decomposition. The isolated assignment `RamRead=2^64` therefore cannot extend through this actual expansion.

This is a derived local implication of the existing ordinary RV64 K-table relation, conditional on the complete original-source/frontend projection. It is not an executed guest, a universal ISA-equivalence theorem, or a complete C7 certificate. Raw `LD/SD`, memory ownership after writes, the exact fixed guest, parser/ledger correctness and all reachable instructions remain separate obligations. No globally accepting false trace was found.

**Independent review:** ConsumedByteReview approved the complete conditional local certificate with no findings after checking the pinned/local sources. Approval covers full-operand range, signed/unsigned lanes, narrow stores, source ownership and the stated limitations. It does not supply the missing complete frontend/profile or fixed-guest C7 certificate.

The invariant is unchanged: native pre-PCS instance accumulation; the original application relation; original lattice handles and all-valid weak sources; one final native decider; and the exact original public receipt. No range-check protocol, new commitment, honest-source restriction or cryptographic assumption is added.

## 2. Evidence and prerequisites

The source anchor is modern Jolt revision `0791f79a55c3fadc3d327c78a72228696c338468`, as recorded in the [frontend manifest](AKITA_INSTANCE_ACCUMULATION_FRONTEND_MANIFEST.md). ConsumedByteScout inspected the source chain and decisive immutable files. Parent inspection checked the actual memory recipes, R1CS rows, instruction ReadRAF contraction, PEXT recurrence and operand polynomial. Links below point to the local sources; the decisive immutable references are:

- [Memory expansion recipes](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-program/src/expand/memory/shared.rs).
- [RV64 R1CS](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/composed/r1cs/rv64.rs).
- [Instruction ReadRAF](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage5/instruction_read_raf.rs).
- [PEXT table](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-lookup-tables/src/tables/pext.rs).

Assume C1–C6 of [OUTER_PROJECTION](AKITA_INSTANCE_ACCUMULATION_OUTER_PROJECTION.md) have recovered the **same-original-source** K-valued table relation, including the full-grid decoder, all separated frontend identities and required source/static-reference comparisons. Do not infer those identities from one random evaluation without the extraction theorem. Assume authentic fixed expanded bytecode and the selected ordinary RV64 feature mode. Committed-program and field-inline modes need their own additional certificates.

The prime characteristic is

$$q=2^{128}-2^{32}+22537>2^{64}.$$

Write $\iota$ for the injective embedding of integers in $[0,2^{64})$ into K. Extension-field challenges do not change the base-field meaning of the recovered tables.

## 3. Selectors and lookup operands are constrained, not honest hints

### 3.1 One-hot semantic indices

For a zero-omitting instruction or bytecode digit, the physical-to-semantic map is

$$R(0,t)=1-\sum_{a\ne0}X(a,t),\qquad R(a,t)=X(a,t)\quad(a\ne0).$$

Thus $\sum_aR(a,t)=1$. The recovered Booleanity identities give $R(a,t)^2=R(a,t)$. Since a digit alphabet has 16 or 256 elements, less than q, the integer count of ones must be exactly one. The product of these digit selectors chooses one Boolean instruction address $k\in\{0,1\}^{128}$, not an arbitrary integer reduced modulo q. Unused physical zero cells remain in the decoder's kernel; no physical whole-grid Booleanity is claimed.

Owners: [digit_zero.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/jolt/lattice/relations/digit_zero.rs), 1–22 and 183–279; [booleanity.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/jolt/geometry/booleanity.rs), 39–55.

### 3.2 Exact full-operand equations

With k indexed most-significant-bit first, the interleaved operand polynomials are

$$L(k)=\sum_{i=0}^{63}2^{63-i}k_{2i},\qquad
R(k)=\sum_{i=0}^{63}2^{63-i}k_{2i+1}.$$

Both have unique representatives in $[0,2^{64})$. For an interleaved instruction, the separated ReadRAF relation gives

$$\mathrm{LeftLookupOperand}=L(k),\quad
\mathrm{RightLookupOperand}=R(k),\quad
\mathrm{LookupOutput}=T_{\rm selected}(k).$$

Owners: [instruction_read_raf.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/stages/stage5/instruction_read_raf.rs), 160–213; [identity.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-poly/src/identity.rs), 28–56. Operand-to-register selection is in [input_virtualization.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/jolt/relations/instruction/input_virtualization.rs), 69–101, and [rv64.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/composed/r1cs/rv64.rs), 249–300.

PEXT/PEXTSigned has no Add/Sub/Mul/Advice flag and selects its two register operands. Consequently

$$\mathrm{Rs1Value}=L(k),\quad\mathrm{Rs2Value}=R(k),\quad
\mathrm{RdWriteValue}=T_{\rm PEXT}(k).$$

Flags, immediate, register selectors and lookup-table choice are pullbacks of the **same authenticated bytecode row**, not independently selectable fractional values: [bytecode.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/jolt/geometry/bytecode.rs), 544–625. Public constant One is fixed by [r1cs/mod.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/composed/r1cs/mod.rs), 243–262. It cannot be set to zero to disable these guards.

## 4. Consumed-subword theorem

**Theorem.** Under §2's hypotheses, consider a completed supported subword-load expansion at a declared nonzero mapped RAM address. Let W be the containing RAM-word version read by its LD. Then there is a unique integer $W_{64}\in[0,2^{64})$ with $W=\iota(W_{64})$. Under the recipe's address/alignment contract, its output is exactly the requested little-endian lane, with the specified zero/sign extension.

**Proof of word range.** LD imposes

$$\mathrm{RamRead}=\mathrm{RamWrite}=\mathrm{RdWriteValue}=W.$$

The recipe writes W to the virtual register consumed by PEXT. The register prefix recurrence and authentic intervening rows preserve that value. Section 3 therefore gives

$$\boxed{W=\mathrm{PEXT.Rs1Value}=L(k).}$$

The right side is the embedding of a 64-bit integer. Since q exceeds $2^{64}$, the representation is unique. In particular the isolated LD assignment $W=2^{64}$ cannot satisfy this equation: the difference from any 64-bit value is nonzero and has magnitude less than q. No eight-load coverage argument is needed; **one** such subword load constrains all eight bytes of its containing word. ∎

Define the unique bytes by

$$b_j=\sum_{h=0}^{7}2^h\operatorname{bit}_{8j+h}(W_{64}),\qquad
W=\sum_{j=0}^{7}2^{8j}b_j,\quad 0\le b_j<256.$$

Repeated reads of the same unchanged word version therefore cannot select inconsistent byte decompositions.

### 4.1 Exact supported macros

[shared.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-program/src/expand/memory/shared.rs), 30–137, uses the following fused recipes:

| Source instruction | Alignment | Extraction |
|---|---|---|
| LBU / LB | none beyond containing-word alignment | AlignAddr; LD; WindowMaskB; PEXT / PEXTSigned |
| LHU / LH | AssertHalfwordAlignment | AlignAddr; LD; WindowMaskH; PEXT / PEXTSigned |
| LWU / LW | AssertWordAlignment | AlignAddr; LD; WindowMaskW; PEXT / PEXTSigned |

For width $n\in\{8,16,32\}$ and legal byte offset o, the mask is $M=(2^n-1)2^{8o}$. Allowed offsets are respectively $0,\ldots,7$; $0,2,4,6$; and $0,4$. The unsigned lane and signed result are

$$v=\left\lfloor W_{64}/2^{8o}\right\rfloor\bmod2^n,
\qquad v_{\rm signed}=v+\sigma(2^{64}-2^n),$$

where $\sigma$ is v's top bit. The signed result is its u64 two's-complement encoding, not a negative field representative.

On Boolean operands the actual PEXT table evaluates, in MSB order,

$$z_{i+1}=z_i(1+y_i)+x_i y_i,\qquad z_0=0.$$

For $y_i=0$ it retains z; for $y_i=1$ it appends $x_i$. A contiguous mask therefore gives the stated lane. [pext_signed.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-lookup-tables/src/tables/pext_signed.rs), 12–79, adds the sign-extension term $\sigma(2^{64}-2^{\operatorname{popcount}(y)})$.

### 4.2 Address, sequence and boundary qualifications

The word-range proof uses interleaved lookup linkage. Correct address/lane semantics additionally needs the fixed guest's bounded effective-address and alignment proof. [operands.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-program/src/expand/operands.rs), 43–45, formats normal recipe immediates via signed-to-u64 conversion, while alignment assertions use the original immediate. Preserve this distinction; do not silently infer wrap/underflow behavior from a host emulator.

For identity-RAF address/mask instructions, the Akita guard excludes an all-ones upper 64-bit address half. Its recovered equation is $H(k)U(k)=0$. Thus a surviving identity index satisfies

$$k<2^{128}-2^{64}<q,$$

excluding a second modulo-q representative. Retain this guard and prove the intended address arithmetic in the selected bounded domain. Sources: [instruction.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/jolt/geometry/instruction.rs), 24–58, and ReadRAF 198–209.

RAM selectors have a common Boolean hamming value h: either no selected address or one selected address. The actual nonzero mapped address and no-wrap contract make h=1 on the relevant access. Load=1 alone does not prove h=1 at effective address zero. Owners: [hamming_booleanity.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/jolt/relations/ram/hamming_booleanity.rs), 49–91; [ram_raf_evaluation.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/stages/stage2/ram_raf_evaluation.rs), 135–153; rv64 213–227.

Virtual rows are not freely reorderable. The recipe's allocated virtual registers, authenticated metadata, actual successor shift and equation

$$(\mathrm{Virtual}-\mathrm{Last})(\mathrm{NextPC}-\mathrm{PC}-1)=0$$

link the LD to its PEXT row. The entry/sequence rule and actual final-row convention remain part of global composition; the local theorem assumes a completed macro, not an invented halt rule. Sources: [metadata.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-program/src/expand/metadata.rs), 47–52; [allocator.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-program/src/expand/allocator.rs), 135–146; rv64 342–374; [shift.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/jolt/relations/spartan/shift.rs), 69–121.

## 5. Stores, raw words and nondeterministic advice

### 5.1 Narrow stores preserve canonical words

The actual [narrow-store recipe](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-program/src/expand/memory/shared.rs), 475–518, loads the containing word, clears its lane by ANDN, shifts the source lane into place, adds the two disjoint parts and writes by SD. SH/SW also assert alignment.

ANDN's full interleaved left operand forces the old word into u64. ShiftData's full operand forces the source register into u64. Under the same address/mask hypotheses,

$$W_{\rm new}=(W_{\rm old}\mathbin{\&}\neg M)
 +((S\bmod2^n)\ll8o)<2^{64}.$$

The bit sets are disjoint, so the ADD has no overflow and preserves all other bytes. SD writes this exact field value. Relevant tables: [andn.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-lookup-tables/src/tables/andn.rs), 14–31; [shift_data_b.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-lookup-tables/src/tables/shift_data_b.rs), 12–46; [shift_data_h.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-lookup-tables/src/tables/shift_data_h.rs), 12–48; [shift_data_w.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-lookup-tables/src/tables/shift_data_w.rs), 12–45.

### 5.2 Raw LD/SD are not independent range checks

A direct LD copies the field word. A direct SD equates the written field word to Rs2. Neither alone proves u64 range. A raw-word parser path must obtain canonicality from a canonical initializer/reference, a canonical previous write, or a linked full-operand check. Sources: [ld.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-riscv/src/instructions/i/ld.rs), [sd.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-riscv/src/instructions/i/sd.rs), and rv64 229–239.

A later subword read constrains the current word version. It does not retroactively constrain an overwritten initial advice coefficient.

### 5.3 Virtual advice is a different interface

VirtualAdvice/AdviceLoad is bounded nondeterminism, not an authenticated read of C_g. It disables right-input linkage but retains RangeCheck lookup-output writeback:

$$\mathrm{RdWrite}=\sum_{i=0}^{63}2^{63-i}k_{64+i}\in\iota([0,2^{64})).$$

Sources: [range_check.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-lookup-tables/src/tables/range_check.rs), 13–29; [advice.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-riscv/src/instructions/virt/advice.rs); [advice_load.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-riscv/src/instructions/virt/advice_load.rs); shared.rs 139–172. The intended role of each such value still needs its downstream semantic constraints. No source-index/value equation here equates this tape to the initial committed advice RAM. A guest certificate must cover all reached nondeterministic instructions or prove they are unreachable.

## 6. Source ownership and memory versions

The recovered memory/register identities are strict-prefix recurrences, not host snapshots:

$$\mathrm{Mem}(a,t)=I(a)+\sum_{u<t}\mathrm{RamRa}(a,u)\mathrm{RamInc}(u),$$
$$\mathrm{Reg}(r,t)=\sum_{u<t}\mathrm{RdWa}(r,u)\mathrm{RdInc}(u).$$

Read/write contractions select these same versions. Owners: [RAM val_check](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/jolt/relations/ram/val_check.rs), 94–157; [read_write_checking.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/jolt/relations/ram/read_write_checking.rs), 59–84; [twist/memory_checking.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/twist/memory_checking.rs), 36–87. The native digit/carry increment alphabet is not a u64 bound; range comes from the checked value use above.

The initial advice owner chain is:

1. [Stage4 RAM initialization](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/stages/stage4/ram_val_check.rs), 350–383, 425–475 and 497–550, embeds the checked advice block in I using its actual start/capacity and derives the logical original-source claim.
2. [Native stage8](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/stages/stage8/akita.rs), 163–184, 267–333 and 517–530, checks claim/commitment presence, validates packing metadata and opens those stage4 values against the **same original** TrustedAdvice/UntrustedAdvice handles in the complete batch. The non-Akita stage6b/7 path is not its owner.
3. Trusted static words additionally use the installed original-reference comparison from OUTER_PROJECTION §8. Honest packing alone supplies no adversarial range check.
4. SDK argument slices point into these RAM regions, not the VirtualAdviceLoad tape: [macros/src/lib.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/jolt-sdk/macros/src/lib.rs), 792–803.

If a PEXT-certified word version is still its initial advice value, its unique bytes are the original committed word's bytes. Repeated unchanged reads agree. After a write, the actual write semantics and recurrence determine the new version. Advice memory is not inherently immutable: the fixed guest must prove its archive/static regions remain read-only, or specify a source-preserving copy/provenance argument.

Unread capacity can remain arbitrary K data. Do not truncate or re-encode the entire weak source to bytes. Consumed-read adequacy alone does not make all original C_g cells canonical, nor fix an external logical length. The quotient relation must handle ignored cells explicitly. Generic SDK `take_from_bytes` leaves remainders; its empty length hook is not an EOF proof. Canonical framing remains a guest obligation.

## 7. Exact remaining C7 certificate

A full certificate for the selected receipt needs these finite objects:

1. **Fixed program/profile.** Authentic expanded bytecode, entry, feature mode, memory layout/no-wrap bounds, original handles and installed static reference. No unimplemented guest's instruction set or trace bound is inferred here.
2. **Exhaustive local coverage.** Use §§3–5 for the actual supported memory macros; prove all other reachable instruction cases, runtime/library calls, source/register selection and raw-word canonicality provenance. Cover virtual advice and PC/padding boundaries.
3. **Original-version data flow.** Annotate each consumed word with its initializer/last write, each register use with its preceding write, and each parser byte with its original source location. Preserve all source/shape identities and protected regions.
4. **Global induction.** Compose local implications along the original ordered rows, with actual entry, zero-initial register state, source-owned RAM, shifts and write increments. No new ISA/MMIO/halting language is introduced.
5. **Exact algorithm.** Prove bounded canonical parsing, logical EOF, every native update-prefix/final-decider check, exact supplied-answer ledger coverage, AS relation, and original X/acc/C_g/C_t/C_static linkage. Successful output must imply this algorithm accepted, not merely that a generic function returned a byte.
6. **Output binding.** The expected public success is canonical `01`, panic=false. [public_io.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-program/src/preprocess/public_io.rs), 20–59, also fixes termination=1 when panic=false; [output_check.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/jolt/relations/ram/output_check.rs), 80–121, links the final RAM public values. Retain the actual verifier's output normalization and wrapper contract rather than trusting prover-chosen output length.

The local lemma is positive progress, not a request for new cryptographic assumptions. It removes the specific unsupported inference that a subword read might silently host-truncate an arbitrary field word. It does not prove arbitrary raw LD words are canonical, virtual advice equals C_g, or an entire capacity belongs to the honest packing image.

## 8. Verification and disposition

Evidence is static source inspection, the algebraic derivation and parent-executed finite arithmetic: all 65,536 eight-bit PEXT input/mask pairs, 336 unsigned/signed u64 lane cases and 672 masked-store cases satisfied the stated identities. These checks supplement the proof; they do not execute a Jolt guest or establish original-source/whole-run adequacy. No source implementation, build, formatter, permanent test, proof execution, benchmark or Git operation is part of this note. Complete guest/profile admission and performance remain open.
