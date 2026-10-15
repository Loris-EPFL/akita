# T16 native accumulation archive guest specification

## 1. Status, anchor and selected wrapper

This is a **research algorithm, wire-contract and provenance specification**, not source implementation, a compiled guest, a generated schedule, an outer-profile admission, a concrete soundness theorem or a performance result. Source pins are Jolt `0791f79a55c3fadc3d327c78a72228696c338468` and Akita `83574331d4e51f8cce1d8689d05592fa4ef4c138`. Local source inspection below is evidence about the supplied trees, not an independently performed Git pin check.

The anchor is the original K-valued whole-run Jolt Boolean-table relation, obtained by concatenating consecutive active segments and applying original global padding once. Inputs are **pre-PCS segment frontend/native update prefixes**, never completed per-segment proofs. Every update includes the full outgoing strong check; one final domain/normalization prefix, three shrinking reductions and one actual A-only terminal discharge the accumulated opening. A pending frontend or fold is never reported as a complete proof.

PrototypeModel selected the **direct fixed-public-concrete-transcript** research route under the explicitly approved protocol-specific instantiation heuristic; its security statement belongs to [PROTOTYPE_MODEL](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md). The common algorithm below is unchanged if a separately selected supplied-answer wrapper is reinstated. Direct mode computes the specified concrete transcript itself; it has no supplied answers, graph witnesses, g/t/AS accumulator, C_t, quotient opening or external graph equations. Those checks in the earlier [RECEIPT](AKITA_INSTANCE_ACCUMULATION_RECEIPT.md) apply to that older wrapper, not to this direct entrypoint. This does not waive guest adequacy, full outer extraction, profile or parity gates. In particular it is not an independent proof of CH-I/CH-O; the exact scoped implication of the heuristic is the model note's responsibility.

The proposed entrypoint is conceptually:

`verify_archive(x_o, UntrustedAdvice<ArchiveBytes>, TrustedAdvice<StaticBytes>) -> u32`.

The selected direct public statement is `x_o=(D1, approved id(I), X)` under PrototypeModel's conditional-review D1 version; the approved installation selects the bound profile/setup/guest identities. Successful return is `1u32`; the external verifier constructs the expected device with canonical postcard output **01** and **panic=false**. The actual original untrusted-advice handle is called `C_archive` here (the earlier `C_g` role); installed trusted advice is `C_static`. They are outer sources, not the compact inner application's optional advice sources. No duplicate public archive handle or public archive/boundary list is selected. The complete proof and adequacy argument must retain the same actual original advice handle, not a reconstructed canonical replacement.

## 2. Fixed profile and installation inputs

The selected inner profile is exactly [COMPACT_PROFILE](AKITA_INSTANCE_ACCUMULATION_COMPACT_PROFILE.md):

- K = F_q, q = 2^128 - 2^32 + 22537; E = K[U]/(U²-5), ordered basis (1,U). Sources/matrices remain K; frontend challenges/evaluations are E. The default nonresidue-2 type is not this extension.
- 1 <= T <= 16; each segment has Nseg=4096 and 1 <= ell <=4095, CycleMajor, k=4; full public padded program length 4096; proof-side RAM domain 65536 words; P=56 original main columns. No inner advice objects, committed program or field-inline feature is selected. Such records reject in this profile, rather than activating an uncounted optional branch.
- Source/carrier/A/B/D degree 128; all common/response/opening bases 32; full E[Y]/(Y^128+1) partials have 256 K coordinates. Inner-image and partial digit depth is 26. One S-linear raw query channel per original handle.
- Update group order is fixed as old receiver, Main, RAM-minus, RAM-plus. This native order is distinct from the frontend commitment absorption order and from the original auxiliary/main PCS order. Bind all three ordered contexts explicitly.
- Registered zero receiver is an actual ReceiverRoot commitment/path to a correctly sized zero source and its fixed zero-evaluation claim, not a bare zero digest.
- Main is a newly provisioned direct scale-one Boolean role, not a relabelled legacy signed/tensor commitment. Receiver is P1/B32/M1024/depth1; Main P56/B1/M512/depth1; each RAM endpoint P1/B16/M32/depth26. RAM endpoints retain full-field decoded semantics, not a u64 or signed-65 restriction.
- The final schedule creates Final2M, then Final1M, then Terminal512K under their distinct registered maps. It cannot reinterpret an existing OuterPayload as TerminalInnerState.

Installation supplies one approved, immutable static blob: exact 33 independent matrix roles from [MATRIX_REGISTRY](AKITA_INSTANCE_ACCUMULATION_MATRIX_REGISTRY.md), canonical public program/expansion/memory-layout preprocessing, source maps, masks, digit orders, equation-row manifests, sampler/transcript emission descriptors, and any proof-independent prepared tables. Matrix identity is determined by this installed data, not an attacker-provided label. There are 4,681,728 expanded K coefficients, or 74,907,648 bytes before metadata. Streaming is permitted; setup regeneration, contractions or cache validation are not free.

The outer verifier authenticates `C_static` as its expected original trusted-advice commitment under an independently provisioned outer setup. The guest checks static schema and installed-selector consistency, but cannot self-authenticate a prover-selected setup merely by hashing it. Prepared data must have been validated against those exact matrices before installation or must be checked by the guest; proof-dependent weights/claims never enter the approved static-reference shortcut. The full ordinary outer proof, its field split, source map, matrix distribution and suffix have their own admission and cost ledger.

## 3. Canonical data grammar

### 3.1 Atoms and frames

This is a new versioned archive frame using existing canonical patterns, **not a claim that bincode or the present native proof body already implements this container**.

- `Rec(tag,payload) := BE32(tag) || BE64(byte_length(payload)) || payload`, the typed record pattern of [CHAINED_TRANSCRIPT §2](AKITA_INSTANCE_ACCUMULATION_CHAINED_TRANSCRIPT.md). Tags below are fixed integer constants, not serialized strings. This outer frame is distinct from native scalar and transcript-internal integer encodings.
- Payload unsigned counters/lengths use fixed-width LE64; version is LE32; Boolean is one byte exactly 00 or 01. Every conversion to a machine index is checked. Identifiers are exactly 32 bytes selecting installed descriptors, never free-form replacement descriptors.
- K atom is exactly 16 little-endian bytes encoding an integer <q; do not reduce invalid bytes modulo q. E atom is `K(c0)||K(c1)` for c0+c1 U, exactly 32 bytes. Decode both coefficients transactionally. Ring coefficients are increasing Y-power order, with negacyclic reduction in the declared native ring. Extension coefficient planes and physical layout are explicit descriptor data; no plane is inferred from a field alias.
- A point has exactly the descriptor's arity, with canonical E atoms in its specified semantic order. No caller-selected point length, basis or permutation is accepted. Counters/points/field values are different atom types.
- Variable byte arrays have one LE64 length inside their enclosing typed payload where specified. Fixed arrays have no extra count. Every record length must equal actual consumption; logical EOF follows the final record. No alternate varint representation, duplicate tag, optional unknown field or ignored record is accepted.
- Public X uses the installed canonical application-statement codec, including its native normalized output/panic convention. The archive's X is compared byte-for-byte to canonically encoded public X. The enclosing SDK postcard argument is a separate envelope and must have a checked length/padding rule; the current macro's discarded remainder is not an EOF proof.

Native foundations: [proof_stream.rs:261–335](../../crates/akita-transcript/src/proof_stream.rs), [field_impls.rs:20–124](../../crates/akita-serialization/src/field_impls.rs), [adapters.rs:1281–1303](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/adapters.rs). Require validated field decode; `Validate::No` is not acceptable for adversarial bytes.

### 3.2 Original handles and path representation

`Handle := mode:u8 || source_descriptor_id:32 || payload_length:LE64 || payload`.

Mode 0 is the original compressed OuterPayload; mode 1 is the scheduled TerminalInnerState A-image tuple. The selected installed source descriptor supplies **all** original metadata: backend flavor, native layout digest, physical arity/P/B/M, logical-to-physical map, raw-view map, ring/plane/digit order, bases/depths, alphabet and zero masks, A/B/F keys and comparison class, and native commitment profile/version. Resolve this descriptor and its trusted schedule before decoding/reserving payload storage. Require payload length exactly `16*c_descriptor`, where c_descriptor is the expected native coefficient count; the length word is redundancy, not allocation authority. All payload coefficients are canonical K. Terminal ordering is `(block i, A-output row, Y coefficient)`.

This compact descriptor reference preserves the full typed original handle; it is not equality of commitment bytes alone. Repeated RAM-plus/RAM-minus handles require equality of mode, descriptor and original payload. Compatible-looking dimensions do not authorize changing keys, source maps, compression modes or source occurrences. The native metadata model is [AkitaCommitment, lines883–998](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/adapters.rs) and [CommittedGroup, lines48–112](../../crates/akita-types/src/proof/commitment.rs). The allocation guard follows [shape_guard.rs:25–143](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/shape_guard.rs).

Private source/path objects retained by the accumulation producer are not transmitted as naked replacement witnesses to this verifier. Their mathematical encoding is the registered physical K array: raw source digits, original A-image digit decompositions, and every F/H input digit in exact stage/plane/coefficient order. Base-32 digits lie in [-16,15]; Main raw values are 0/1; F/H digits are 0 or -1. A signed digit represented as a K atom is its unique residue; alphabet checks are relation obligations, not permission to accept arbitrary residues. Gadget recomposition preserves every valid noncanonical decomposition; no canonical redecomposition, hash-only path or shortened chain is allowed. F/H each have the exact two stages (degree16/rank1/width1024, then degree8/rank1/width256) in this registry. If a future transport carries these arrays, it must carry their exact fixed descriptor lengths and canonical K atoms, not an unconstrained vector of paths.

The archive contains their commitment/opening payloads and the full native relation certificates; the successor source contains the complete Eq.(109) digits. A verifier does **not** check untransmitted raw digits by assertion: strong range/binary/zero reductions and the final decision are what discharge these obligations.

### 3.3 Container schema

The following tags are assigned only in this new archive namespace:

```
1 Header(version:u32=1, model_id, profile_id, setup_id,
         program_id, guest_schema_id, statement_codec_id, T:u64,
         Rec(2, canonical_X))
10 Segment(
     Rec(11, SegmentPublic),
     Rec(12, FrontendPrefix),
     Rec(13, OriginalDomainPrefixes),
     Rec(14, RawNormalizers),
     Rec(15, CompleteUpdate)) repeated exactly T times
20 Finish(
     Rec(21, FinalDomainPrefix),
     Rec(22, FinalRawNormalizer),
     Rec(23, ShrinkRoot), Rec(24, Shrink2M), Rec(25, Shrink1M),
     Rec(26, DirectTerminal))
EOF
```

IDs in Header are six fixed 32-byte identifiers. A first/last flag is not supplied: derive it from index and T. `SegmentPublic` is fixed in this order:

```
index:u64, ell:u64, expanded_before:u64, expanded_after:u64,
Boundary_minus, Boundary_plus, ExternalSuccessor, MainHandle
Boundary := expanded_pc:u64, architectural_pc:u64,
            registers:K[128], RAMHandle
ExternalSuccessor := expanded_pc:u64, architectural_pc:u64,
                     is_virtual:u8, is_first:u8, is_noop:u8
```

Boundary-plus PC fields designate the successor state, not the last executed row. Last-active expanded PC is a verifier-consumed frontend endpoint claim, separately authenticated by its moment reduction; confusing these two PCs rejects. The frontend carries first/last active-PC values in its fixed endpoint-message schedule. Public first/last digit reconstructions and successor metadata must agree with these fields and installed program. No field-register option occurs in T16.

Each proof-section payload begins with one 32-byte installed **emission/schedule descriptor ID** and then exactly its scheduled prover-message atoms. There are no serialized challenge answers in direct mode. Conditional presence depends only on verified index/first/last/profile and the prior verifier state, never on attacker-chosen omission flags. References to earlier claims/handles are typed ordinal references into the verifier-derived ledger, checked for range, role and prior binding. Section lengths are bounded by the descriptor, not trusted because their frame parses.

Native sumcheck representation and labels follow the selected native message descriptor. Preserve compressed round omission of c1 when that codec is selected: reconstruct `c1=running_sum-2*c0-sum(c2..cD)`, then check the prescribed degree, domain and recurrence. Full centered uni-skip polynomials use their actual centered-domain functional, not the ordinary `p(0)+p(1)` shortcut. Every polynomial count/representation is fixed by the descriptor; no attacker degree declaration is authoritative. [round_proof.rs:98–163](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-sumcheck/src/round_proof.rs) is the compressed-label pattern, not evidence that every new section uses that codec. Native headerless Spongefish bodies are usable only under their exact resolved grammar; [wire.rs](../../crates/akita-types/src/proof/wire.rs) has conditional extension/setup/next-witness members, so blindly invoking its old decoder is not this new coefficient-mode grammar.

**Unemitted constants:** the complete revised atom-emission table, per-site native codec selection, section descriptor IDs and exact section maxima are not generated at the pins. They are explicitly `EMIT_T16_v1`, `CODEC_SITE_T16_v1`, `SCHEDULE_IDS_T16_v1`, and `SECTION_MAX_T16_v1`. This specification fixes the required checks and their source owners below; it does not invent byte counts or claim an executable parser without those finite implementation artifacts. No profile may be installed until these artifacts are produced and checked against every listed consumer.

## 4. Complete verifier state machine

### 4.1 Open and initialize

1. Resolve the installed identities, canonical X, static schema, exact source/sampler/field/matrix schedules and outer advice capacities. Reject unsupported identity, feature, geometry, source view, trailing data or overflow before proof-sized allocation. Verify byte regions and C7 provenance conditions in §9.
2. Decode Header and require public/archived X and all expected identities to agree; require 1<=T<=16. Initialize the registered zero receiver and fixed zero claim. Initialize one versioned transcript context with the **whole** public run statement, installed inner setup/profile, geometry, original program/I/O, bounded history mode and sampler law.
3. Maintain state `(i, active_total, previous_boundary, receiver_handle, pending_point/value, transcript_state, obligations)`. Start i=0, active_total=0. An obligation is identified by original typed handle, view, point, value, source consumer and discharge site; two equal-looking field values are not interchangeable obligations.

### 4.2 Bind a segment and derive its pending frontend

4. Read the next Segment, require index=i and `expanded_before=active_total`, checked `expanded_after=expanded_before+ell`, 1<=ell<=4095 and total<=65520. Check fixed program/layout/source descriptors. Bind Main, both RAM endpoints, boundary vectors and external successor before challenges consuming them. Old receiver comes from authenticated verifier state, never an archive-selected replacement.
5. If i=0 require original entry and initial register state once. Otherwise require full Boundary_minus equality to previous Boundary_plus, including all 128 K registers and the same typed original RAM handle. Require x0=0 and canonical K entries; do not require all cut-state registers to be u64. Preserve virtual/CSR/reservation slots 32–39 and the full 128-vector.
6. Authenticate legal cuts against the installed canonically expanded program, including temporary resets. At a real interior successor require Boolean flags and `NextIsVirtual*(1-NextIsFirstInSequence)=0`; successor expanded/architectural mapping must be correct. Require next segment's first PC equal this successor. Final successor is exactly `(0,0,0,0,1)` in `(NextPC,NextUnexpandedPC,NextIsVirtual,NextIsFirstInSequence,NextIsNoop)` order, not an architectural halt theorem.
7. Run **all** stage0–7 frontend consumers and stage8 pending metadata/packing checks in §5, with the continued-segment replacements in §6. Preserve every absorption, centered interpolation, gamma power, batch coefficient, dummy round, phase window, alias and point permutation. Each sumcheck consumes exactly its scheduled rounds and validates its final relation expression. Reject malformed/extra/missing messages. Derive the opening ledger from verified outputs; never deserialize a purported pending ledger as trusted success.
8. Authenticate first and last active PCs with both BytecodeRa moment reductions and column-specific physical zero checks. Resolve all ordinary leaves to original handles. On T16 there is Main plus two RAM endpoints; there is no direct committed RamInc, RdInc, active-selector, PC or register source. Do **not** invoke a full segment `PCS::verify_batch` or complete proof suffix.

### 4.3 Certify sources, normalize and perform one complete update

9. Run the old receiver's full range/binary/required-zero prefix on its actual layout. Run each RAM original raw-source domain prefix. For Main use the source-linked native Booleanity/digit-zero relations **plus** explicit column-specific inactive/physical-hole zero check; this is the reviewed reuse, not a blanket waiver of Main shortness. Generic positive Boolean prefixes use s(s-1); negative-binary path predicates use s(s+1). Virtual padded polynomial/digit axes are defined zero; actual physical zeros require the declared relation.
10. Bind all ordinary/raw claims and descriptors before independent normalization coefficients. Convert decoded gadget evaluations to the same original RawView as in §7; verify each degree-two multipoint reduction and its full endpoint equation without dividing by a random factor. Preserve P=56 original Main columns. The result is one S-linear channel per original handle, not a homomorphic compressed commitment or one invented main polynomial.
11. Bind partial/D/H opening payloads before the 120 lattice fold coordinates. Derive the exact shell challenges using §8; reject sampler exhaustion. Verify all original group roles/layouts, points, values, coefficient counts, complete F/H chain expectations and native statement binding. Fix the successor commitment to an actual N=2^22 ReceiverRoot source before its row/range challenges.
12. Verify the complete coefficient relation and full range pipeline in §7, in the declared transcript order: bind the coefficient-row descriptor and sample the row challenge before the range pipeline; retain its initial anchor, both degree-five range stages, four-child interstage frontier and final virtual-range meaning; then sample the independently fused virtual/binary/zero coefficients and check the degree-three fused sumcheck and exact terminal expression. Enforce all Eq.(109) paths, both E planes, padding and original payload equalities. Compute every transcript-dependent matrix/query weight from approved setup; no setup offload is selected.
13. Check replay has consumed every expected event and subrecord, returned the exact successor handle and one pending ordinary opening, and left no forgotten local/setup obligation. Adopt that pending opening as verifier state. Set `active_total=expanded_after` from the checked addition in step4, save Boundary_plus, then increment i. This is a pending accumulated relation, not complete acceptance.

### 4.4 Finish, decide and output

14. After exactly T segments, require original first initialization and original final output relation were each instantiated exactly once. Run the final receiver full physical range/binary/zero prefix and normalize it with the pending opening. An opening alone does not certify shortness.
15. Verify all three **complete** nonterminal shrinking folds, each with its actual predecessor layout/masks and outgoing strong checks:

| Input | Fold coordinates | Response/partial/image K cells | Original F + shared H cells | Live successor | Capacity / binding |
|---|---:|---:|---:|---:|---|
| ReceiverRoot 2^22, B32/M1024/rank4 | 32 | 524288 / 212992 / 425984 | 36864 | 1200128 | 2^21 Final2M OuterPayload |
| Final2M 2^21, B16/M1024/rank3 | 16 | 393216 / 106496 / 159744 | 36864 | 696320 | 2^20 Final1M OuterPayload |
| Final1M 2^20, B16/M512/rank3 | 16 | 196608 / 106496 / 159744 | 36864 | 499712 | 2^19 Terminal512K inner mode |

The third commitment is created in inner mode. Its A images are bound at the point required by the preceding strong-check/fold schedule, not first supplied after a terminal challenge. If terminal framing repeats them, require exact equality to that already authenticated tuple; byte accounting charges each actual occurrence.

16. For Terminal512K, verify the predecessor-authenticated eight rank-three degree-128 A images, exact pending point/value and full-ring partials `e_i=L(w_i)`. Check scalar recombination of all partials to the pending value. Bind images, partials, point/value, terminal mode and role before the eight shell challenges. Receive 512*128 signed response coefficients z, range-check each integer in [-7168,7168] **before** embedding in K. Check every coefficient of `Az=sum_i c_i t_i` and every coefficient in **both E planes** of `L(z)=sum_i c_i e_i`. No trace-only terminal, D/H replacement, detached suffix or caller-asserted final Boolean is sufficient.
17. Require no pending opening/setup/consumer obligation, exact finish/section/logical EOF and transcript lifecycle completion. In supplied-answer mode additionally require the separately selected wrapper's exact query/answer/graph coverage and its AS/public-source equations; direct mode instead has executed every concrete call internally and rejects such extra ledger sections. Return `1u32` only here. All failures return non-success or abort; neither abort nor panic is accepted by the external expected-output verifier. A defensive implementation should use checked errors, not proof-controlled indexing/panics.

## 5. Frontend consumer and original-source ownership ledger

The normative detailed degree/challenge inventory is [FRONTEND_MANIFEST §§1–3](AKITA_INSTANCE_ACCUMULATION_FRONTEND_MANIFEST.md), with only [SEGMENT_LOWERING](AKITA_INSTANCE_ACCUMULATION_SEGMENT_LOWERING.md)'s named replacements. This table records every stage member and eventual source owner. “Virtual” means a claim must continue through named later consumers, never that its value is unconstrained.

| Stage / consumer, in native order | Exact responsibility and eventual original owner |
|---|---|
| 0 preamble/commitments | Validate source shape/setup selection; bind preprocessing, memory layout, actual public input/output/panic, n/m/phase geometry, k and trace order. Frontend absorbs Main first then any auxiliary objects; native group order differs. New boundaries/receiver/history context are also bound. |
| 1 outer uni-skip; Spartan outer remainder | Full row-local CPU/R1CS relation, centered domain 10 for T16; outer round degree27 then n+1 degree3 rounds. Claims route to stage2/3/4/5/6 main/bytecode/RAM/register consumers, not a new R1CS commitment. |
| 2 product uni-skip/remainder | Centered domain3, degree6 initial round; n degree3 remainder. Product/operand lanes route to instruction and bytecode/main reconstruction. Reuse the prescribed reversed outer tail for tau_low. |
| 2 RamReadWriteChecking | n+m degree3, with actual cycle/address phases; current/final states route to stage4 RAM value recurrence and original RAM endpoints. Preserve actual increment/selector read-write constraints. |
| 2 instruction claim reduction | n degree2, five-claim gamma; original operand/lookup/flag virtual claims route to instruction input/ReadRAF and Main.InstructionRa. |
| 2 RamRafEvaluation | n+m-a_R degree2; address claim routes through RAM RA reductions to Main.RamRa, retaining address-zero/hamming handling. |
| 2 RamOutputCheck, last only | n+m-a_R degree3 and native dummy rounds; final `RamValFinal` is original C_Rplus at the aligned RW address; expected table is exact native public I/O/panic/conditional-termination. No such member on nonfinal segments. |
| 3 SpartanShift | n degree2; first four successor legs use outer point, `1-NextIsNoop` uses product point. Corrected external successor is installed-program authenticated. PC/flags route through bytecode Main.BytecodeRa. |
| 3 instruction-input virtualization | n degree3; preserves operand/lookup pullbacks to Main.InstructionRa and subsequent instruction ReadRAF. |
| 3 integer-register claim reduction | n degree2; three-claim gamma; routes register state claims to stage4 RW and stage5 value evaluation. |
| 4 RegistersReadWriteChecking | n+7 degree3; all seven address coordinates and native phase ordering; leaves route through stage5 RegistersValEvaluation and bytecode register selectors. |
| 4 RamValCheck and initial evaluation | n degree3 RAM gamma recurrence; leaves original C_Rminus/C_Rplus and virtual RamRa/RamInc; initialization public/advice/image decomposition occurs first only. RamRa/RamInc end in Main via stages5–7. |
| 5 InstructionReadRAF | n+128 rounds, degree v+2; retains gamma-cubed canonical-address guard `upper_half_all_ones`, lookup equations and original bit/operand semantics. Ends at Main.InstructionRa and native bytecode flag/operand pullbacks. |
| 5 RAM RA reduction | n degree2, arity3 gamma; virtual RAM addresses reduce to Main.RamRa. |
| 5 RegistersValEvaluation | n degree3 plus one new endpoint gamma; public Rminus/Rplus, original virtual RdWa/RdInc leaves. RdWa routes through bytecode; RdInc through Main.BalancedIncDigit/Carry and native bytecode fused increment relation. |
| 6a BytecodeReadRAF address | b rounds, degree b_B+1; six gammas with native powers, staged flag/lookup/register/immediate/PC/entry/increment consumers; full-public program table here. Leaves Main.BytecodeRa and the later native fused increment decoder. Entry is authenticated segment-start expanded index, at unchanged outer gamma exponent11 and constant target1. |
| 6a Booleanity address | k degree3; gamma exponents 2i, preserved reused cycle/address references; every physical main selector column, followed by cycle stage. |
| 6b BytecodeReadRAF cycle | n degree b_B+2, retaining fused-increment factor; same original Main bytecode/increment sources. |
| 6b Booleanity cycle | n degree3; Main columns. |
| 6b RAM Hamming Booleanity | n degree3; Main.RamRa and no-access/one-access constraints. |
| 6b RAM RA virtualization | n degree b_R+1; Main.RamRa chunk pullback. |
| 6b instruction RA virtualization | n degree w/k+1; Main.InstructionRa chunk pullback, actual instruction gamma folding v claims. |
| 7 native Hamming/digit-zero/decode | k degree2; exponent `e_H=2(b_I+b_B)+3b_R+(b_D+1)=99` for T16 (32,3,4,16). Retains final decode claim at exponent e_H. Main.InstructionRa, BalancedIncDigit, Carry, BytecodeRa, RamRa are the final columns. |
| 8 pending main/packing | Validate exact flavor/layout/K=16/arity16/P56 and common native point; derive original P values, do not create successful PCS result. No auxiliary packing selectors occur in selected T16. |
| Extra first/last PC moments | Two degree2 address sumchecks, k=4, b_B=3 independent digit coefficients each; Main.BytecodeRa[0..3) at physical `(bool(cycle)||r_address)`. |
| Extra Main physical zero union | Column-specific inactive/omitted-address masks, degree2 over physical arity16; every Main column at one common physical point. Do not factor differing masks into one shared mask. |
| Original-domain/raw normalization/native update | Original old receiver, Main, C_Rminus, C_Rplus; never replace them with decoded logical tables or commitment hashes. |

For each batch, absorb all present member input claims in declaration order, then draw one independent coefficient per present member; multiply tail-aligned input claims by `2^(Rbatch-Rmember)` and use the actual phase windows. T16 has no field-inline/committed-program/advice branch. The complete general catalogue ownership, useful for rejecting accidental reuse, is: stage2 optional field-register reduction -> stage4 field RW -> stage5 field value -> stage6b FieldInc -> original C_F; initial RAM advice claims -> stage8 original C_U/C_T packing, **not** Dory stage6b/7 advice reductions; committed bytecode six staged values -> head-aligned stage6b cycle and stage7 address -> stage8 original C_B,j; initial program-image contribution -> corresponding stage6b/7 -> stage8 C_image. Committed-program boundary queries use those same C_B,j. These are not enabled merely because an archive supplies them.

Point obligations: native main semantic `(digit[k]||cycle[n])` becomes physical `(cycle[n]||digit[k])`; reverse cycle-only challenge binding order; use exact `ReadWriteDimensions::read_write_opening_indices` and native offsets for RAM/register phased batches. Public-program CycleMajor conventions do not authorize guessing the separate main physical layout. Dense zero-prefix embedding/packing and optional selector draws must use actual source plans. See [SEGMENT_INTERFACE §§3.1,6](AKITA_INSTANCE_ACCUMULATION_SEGMENT_INTERFACE.md).

## 6. Continued-segment equations and global endpoints

At the native RAM reference (x,y), bind v, b=Bminus(x), f=Bplus(x) before the existing RAM gamma and verify

\[
v+\gamma f-(1+\gamma)b
=\sum_{s\in\{0,1\}^{n}}
 \bigl(\widetilde{\mathrm{LT}}(s,y)+\gamma\bigr)
 \widetilde{\mathrm{RamRa}}(x,s)\widetilde{\mathrm{RamInc}}(s).
\]

The degree-three cycle sumcheck retains native phase conversion and leaf routing. Its Boolean-table meaning is `V(a,t)=Bminus(a)+sum_(s<t) RamRa(a,s)RamInc(s)` and `Bplus(a)=Bminus(a)+sum_s RamRa(a,s)RamInc(s)`. Initial `b=I_original(x)` uses the same original first-segment RAM initial-table decomposition and its ancestor address family. It is not an unrelated sampled initial-state assertion or a per-segment reset.

At the register RW address, evaluate both public 128-entry vectors and use the same equation with RdWa/RdInc and one independent new register gamma. Keep original register RW constraints and x0. No new endpoint register commitment or universal u64 predicate is introduced.

On interior segments bind the outgoing RAM evaluation at the shared read/write-derived address before RAM gamma. On the final segment RamOutputCheck supplies that same C_Rplus evaluation; check exact phase/point identity. If an implementation changes the phases, it must carry and normalize an actual second obligation rather than alias unequal points. Preserve the original initial input/program table and final `eq*Mask_IO*(Bplus-Expected_IO)` check, including normalized outputs and panic/termination semantics, exactly once.

For each interior last-active row, SpartanShift adds `eq(r_outer,ell-1)*external_j` to PC/UnexpandedPC/IsVirtual/IsFirst and `eq(r_product,ell-1)*(1-external_IsNoop)` to the fifth leg. The distinct points/gamma weights remain. Final padded successor is unchanged. Keep all row-local constraints on the full padded domain: no new CPU activity mask. Canonical inactive increments/access cells are zero, while RAM/register prefix states stutter. Native no-op metadata makes the sequential-PC row `0=0+4-4`. Do not demand every active row is non-noop or every interior termination word is zero.

Full public program metadata authenticates the first/last moments, successor mapping and legal-cut predicate. The moments are `digit_j=sum_a index(a)*BytecodeRa_j(t,a)` at t=0 and ell-1; reconstruct expanded index most-significant chunk first. Existing one-hot/digit-zero constraints remain essential. Source maps prove physical-hole zeros in addition to inactive zeros; decoded-zero gadget padding is not permission to canonicalize raw digit witnesses.

These conditions give the same whole-run relation through source-linked extraction/concatenation, not deterministic correctness from one sampled MLE equality. No emulator equivalence, host-compute-advice authenticity, raw-LD range lemma or stronger halt/output-length theorem is added.

## 7. Raw query and native strong relation details

The RawView index is `j+128*(digit+delta_bar*(x+M*(block+B*poly)))`, with virtual zero padding in digit/poly axes and the registered permutation back to original physical source columns. For base32/depth26, delta_bar=32 and five fixed digit coordinates are `u_h=32^(2^h)/(1+32^(2^h))`; all denominators are nonzero in K. Let `J=product_h(1+32^(2^h))`. A decoded column claim becomes `(r_j,u,r_x,r_block,bits(poly)), v/J`; depth1 uses empty u and J=1. Keep the actual scale if a different registered map is ever admitted.

For each original handle bind all converted/raw claims `(t_a,z_a)`, draw all independent alpha_a and verify degree-two normalization of

\[
\sum_a\alpha_a z_a
=\sum_{x\in\{0,1\}^{\mu_{\mathrm{raw}}}}
 \widetilde{\mathrm{Raw}}(x)\sum_a\alpha_a\operatorname{eq}(t_a,x),
\]
where \(\mu_{\mathrm{raw}}\) is the arity of that original RawView.

The final check is `terminal=w*sum_a alpha_a*eq(t_a,r*)`; never divide by the factor. Full-ring `L_raw(s_poly,block)=sum_(x,digit) eq(r_x,x)eq(r_digit,digit)s_poly,block,x,digit(Y)` retains all PB partials and both E planes. Coefficient/block/poly weights belong in the final scalar recombination.

The update's Eq.(109) source W includes all response digits, E-partial digits, original inner-image digits and **all original F plus shared H chain input digits**. It has live size 3,555,328 K cells and capacity 4,194,304, with 638,976 required-zero suffix cells. Its exact linear row families are:

1. Original `A_g G_rsp zhat_g = sum_i c_gi G_open that_gi`.
2. Every full-ring channel `L_g(G_rsp zhat_g)=sum_i c_gi G_open ehat_gi`, both E planes.
3. Original B image versus first F recomposition.
4. Shared D on declared partials versus first H recomposition.
5. Every F/H transition, and last image versus its already bound original payload.
6. Every normalized scalar claim versus the correct partial functional.

Flatten native negacyclic coefficient actions with wrap sign minus, exact digit weights/permutations, then fixed extension planes. Pad the 3456 scalar rows to 4096; row arity kappa=12. Bind outgoing C_W and the row descriptor before rho in E^12. Compute `lambda_rho(x)=sum_h eq(rho,h)M[h,x]` and `b_rho=sum_h eq(rho,h)b[h]`. Never allocate M as a dense R*N matrix; this formula specifies an implicit exact weight provider, not an off-cube ring evaluation shortcut.

Retain the initial full range anchor tau0, two actual degree-five product stages with four-child frontier, and virtual endpoint `svirt=sum_x eq(rvirt,x)W(x)(W(x)+1)`. With three independent coefficients gamma/eta_bin/eta_zero, the final degree-three sumcheck has input `b_rho+gamma*svirt` and integrand

\[
\begin{aligned}
&W(x)\lambda_\rho(x)
+\gamma\operatorname{eq}(r_{\mathrm{virt}},x)W(x)(W(x)+1)\\
&\quad+\eta_{\mathrm{bin}}B(x)W(x)(W(x)+1)
+\eta_{\mathrm{zero}}Z(x)W(x).
\end{aligned}
\]

where B and Z are the MLEs of **restricted equality-weight tables** on exact negative-binary spans and required-zero spans (Z uses tau0). They are not products of separately evaluated off-cube masks. Evaluate all four terms using the same pending W(r). Zero weight branches remain total; do not divide away a failed constraint. The row/source tree and weak-source arguments are proof obligations, not extra prover-supplied inverse-norm bounds.

All-valid-input extraction retains nonterminal certificates `delta*s=r`, delta a unit, `||delta||_1<=112`, `||r||_infinity<=32^delta_f-1`, rather than asserting the extracted source is already short. Same-original comparisons use the registered weak–weak radius `224*(32^delta_f-1)`, mixed radius `Delta_f+112*B_s`, and terminal radius 3,211,264 from product bound14,336. Complete F/H negative-binary path consistency recovers the same original projection. D/H opening payload need only remain fixed inside its coordinate-fold family; different source-prefix/query branches may use different D/H payloads while retaining the same original source. The verifier preserves the data/order needed by these proofs; it does not demand an untransmitted extraction tree or canonical path certificate from the prover.

## 8. Transcript and finite-loop contract

Direct mode uses the exact concrete H_m family of [CONCRETE_GRAPH §3](AKITA_INSTANCE_ACCUMULATION_CONCRETE_GRAPH.md), backed by the pinned Spongefish Keccak revision `ef9741346a150039427d9e0c02d6b2d73e93ec81`: fixed 22-byte ASCII domain `akita/aro/component/v1`, LE64(m), LSB-packed m bits with zero unused high bits; one absorb and one 128-byte squeeze using its overwrite duplex. It is not SHAKE or default Akita Blake2b. The [PROTOTYPE_MODEL](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) owns the concrete-family heuristic and precise selection of compiler parameters. No ideal security claim follows from simply executing this code.

Use [CHAINED_TRANSCRIPT](AKITA_INSTANCE_ACCUMULATION_CHAINED_TRANSCRIPT.md)'s injective native-key/emission contract, full context and typed sites; never bridge the frontend to native folding through one 16-byte K seed. PrototypeModel selects state/output width1024, b=64, IV=0^1024 and 64-bit counters/sites/block indices, giving m=1610. The exact component strings are `A=00 || state[1024] || counter[64] || final[1] || length[7] || block[512]` and `S=01 || state[1024] || site[64] || index[64] || zero[456]`. Fixed-width component integers are big-endian bits, whereas H packs that completed bit vector LSB-first into bytes and prefixes LE64(m); canonical K atoms remain LE128. These are distinct codec layers, not contradictory endian conventions. No CM call or compact-CM/root theorem is needed in direct mode.

Maintain appendable state, block counter, <64-byte tail and total-length bound. Finalize a **snapshot**, always with one final absorb including empty tails; continue later appends from the unfinalized state, not from the final node. Preserve per-site block/bit cursors, vector coordinate order, required challenge-history binding and repeats of identical native keys. All absorbed records include their type/length; all source/statement choices precede the challenges that use them. Close the lifecycle at finish. Site IDs, retry positions and state counters cannot wrap or alias. The finite `EMIT_T16_v1` must implement this correspondence; a generic “hash these records” claim is insufficient.

The selected native sampler is the [GLOBAL_MANIFEST §5](AKITA_INSTANCE_ACCUMULATION_GLOBAL_MANIFEST.md) sampler, **not** the optional 187-bit rank sampler in CHAINED_TRANSCRIPT:

- Each E draw samples two independent K coordinates, each from at most two successive 128-bit uniform candidates, rejecting >=q; exhaustion aborts. No modulo reduction/default-zero fallback.
- Each shell coordinate selects 40 distinct positions by partial Fisher–Yates on128; first24 have magnitude1, next16 magnitude2; 40 independent sign bits. At step j use `ceil(log2(128-j))` bits with at most256 rejection attempts. No response-dependent filtering/grinding. Exact sign/order/retry consumption is in the frozen emission descriptor.
- Every scalar/native/sampler loop bound is fixed by T, stage geometry and descriptor; matrix loops by the 33-role registry; parser loops by certified section maxima; no unbounded EOF-search, recursion depth, vector allocation or rejection loop.
- Direct verification computes the actual consumed tape and every rejected attempt. Supplied-answer mode, if separately chosen, must reconstruct and authenticate exactly that same tape and operation order; accepting prover-supplied “challenge already sampled” values changes the algorithm.

Source correction from the outer-extension investigation: current native `ProtocolSiteId` is diagnostic context, not absorbed domain separation (`akita-transcript/src/proof_stream.rs`, prover/verifier context helpers). The actual framed site in S above and the checked emission mapping supply the new binding. Current `sample_field` is an unbounded rejection loop; `FIELD_SAMPLING_QUERY_LIMIT` is a query budget, not a retry cap (`akita-transcript/src/proof_stream/sampling.rs`). The finite two-candidate/256-attempt rules here are the **designed profile's** fallible sampler contract, not behavior already provided by those helpers. Preserve the selected compiler's exact rejected-bit consumption; do not silently inherit the legacy Keccak retry-count absorption or an infallible challenge API.

## 9. C7 source-region/read/version and output obligations

This specification supplies the exact algorithm target for C7, **not its compiled program adequacy proof**. The local consumed-subword theorem in [CONSUMED_BYTES](AKITA_INSTANCE_ACCUMULATION_CONSUMED_BYTES.md) is not a general ISA theorem.

The eventual finite certificate must enumerate every reachable instruction/runtime/library path of the fixed expanded guest and record:

1. **Region ownership.** Which public-input, C_archive, C_static, heap/stack and output RAM intervals each pointer may denote, exact starts/capacities, nonzero mapped addresses, checked endpoint arithmetic/no wrap, widths and alignment. Source offsets are tied to the original capacity-based source decoder, not a host Vec or VirtualAdvice tape. Public preprocessing/entry/expansion/ISA features are authenticated.
2. **Read/write provenance.** For every consumed parser/static byte, give the containing word's original source index and initializer/last-write version; for each register use give its preceding write. Prove archive/static regions remain read-only or prove a source-preserving copy relation before modification. RAM/register strict-prefix recurrences and actual load/store contractions select these versions. A later canonical read does not constrain an overwritten initial coefficient.
3. **Local coverage.** Completed LBU/LB/LHU/LH/LWU/LW recipes constrain the entire containing word through interleaved PEXT/PEXTSigned operands; retain the canonical-address guard, hamming/no-zero-address condition, alignment and ordered virtual expansion. Raw LD/SD alone do not prove u64 canonicality: initializer/reference, previous canonical write or linked full-operand checks must supply it. Narrow stores require the actual clear/insert/disjoint-lane proof. Cover every other reachable operation, including multi-limb K/E arithmetic, comparisons, carry/overflow and error branches; no unchecked host truncation may implement canonical decoding.
4. **Nondeterminism.** VirtualAdvice/AdviceLoad is not an authenticated C_archive read. Prove such instructions unreachable in source parsing/static access, or account for every occurrence with its actual downstream constraints and a source equation when needed. Boundedness alone is not provenance.
5. **Global induction and exact parsing.** Compose the local lemmas along original rows from entry/initial registers/source-owned RAM through shifts and writes. Prove the exact §4 algorithm, every original-handle consumer, transcript computation, bounded frame length and logical EOF. In supplied-answer mode its additional ledger/AS equations are also part of this implication; direct mode has no such ledger.
6. **Capacity slack.** Unread C_archive cells may remain arbitrary K elements if the admitted outer source/decoder relation permits them. Do not truncate/re-encode the whole weak source into canonical bytes. Logical EOF authenticates the consumed logical object, not unused capacity zeros. Canonical padding is a separate proved source relation if required. C_static words additionally use the installed same-original-reference comparison, not merely honest packing.
7. **Output.** Successful public output is exactly canonical postcard `1u32` (01), panic=false, enforced by the complete outer proof against a newly constructed expected device. Original final RAM output check includes the native termination=1 convention when panic=false. Preserve trailing-zero normalization; do not infer a stronger byte-vector length theorem from a decoded output or accept a proof-supplied “success device.”

The actual outer source chain is stage4 RAM initial decomposition -> original logical TrustedAdvice/UntrustedAdvice claims -> native stage8 packing/metadata -> the **same** original advice handles in the complete ordinary outer PCS batch. It is not Dory stage6b/7 or a replacement commitment. SDK argument slices point to those RAM regions. C1–C6 full-source/static comparisons and extraction remain prerequisites to this C7 induction; one realized sampled relation does not establish them.

### 9.1 A source-backed arithmetic lemma, not a full C7 certificate

Assume the recovered source-linked table relation, authentic MUL/MULHU bytecode, and **already established canonical u64 input registers** a,b. The multiplication lane gives `Product=a*b` in K; the [R1CS routing](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/composed/r1cs/rv64.rs), lines241–301, ties the identity lookup operand to Product and rd to its lookup output. The [existing Akita ReadRAF guard](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/stages/stage5/instruction_read_raf.rs), lines198–209, enforces that the selected 128-bit identity index k has no all-ones upper 64-bit half. The authentic opcode/bytecode flag linkage must be retained; honest witness production alone is not that proof.

\[
0\le ab\le(2^{64}-1)^2
=2^{128}-2^{65}+1
<2^{128}-2^{64}<q.
\]

The guard also implies \(0\le k<2^{128}-2^{64}<q\). Thus the identity equation \(k\equiv ab\pmod q\) forces **integer equality** \(k=ab\). The [RangeCheck table](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-lookup-tables/src/tables/range_check.rs) returns the low 64 bits for MUL; the [UpperWord table](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-lookup-tables/src/tables/upper_word.rs) returns \(\lfloor ab/2^{64}\rfloor\) for MULHU. Both outputs are canonical u64. This discharges that local implication when its input/source hypotheses hold, not an arbitrary-register or raw-LD range theorem.

Without the existing guard, an identity index `ab+q<2^128` could represent the same K value but a different low/high-word lookup. Increasing challenge field cardinality to q² does **not** remove that risk: E2 still has characteristic q. Keep the gamma-cubed guard and its complete source/bytecode extraction path. No new constraint or change to the original application relation is proposed.

## 10. Exact resource equations and deliberately missing constants

### 10.1 Structural archive Amax, not an invented byte estimate

Let x=|canonical_X| and let `h_R`, `h_X` be exact RAM/Main Handle byte lengths from §3.2. Then `h_role=41+16*c_role`. Each Boundary has 2064+h_R bytes; ExternalSuccessor has19. Thus the exact SegmentPublic payload is

`Bpub = 32 + 2*(2064+h_R) + 19 + h_X = 4179 + 2*h_R + h_X`.

For this proposed compact codec the registered final F compression image has rank1 and degree8, so c_role=8 for each nonterminal compressed source: h_R=h_X=169 and **Bpub=4686 bytes**. The terminal image tuple has c_terminal=8*3*128=3072, so a full typed inner Handle has49,193 bytes. These are mathematical wire sizes for §3's explicit descriptor-reference codec, not measurements of legacy adapter/bincode objects; generated descriptors must enforce exactly those payload shapes.

It duplicates boundary handles/vectors deliberately; equality is checked. Header payload is `4+6*32+8+(12+x)=216+x`. Each proof section includes its 32-byte emission descriptor ID. Let f_i,d_i,n_i,u_i denote full payload lengths (including that ID) of frontend/domain/normalizer/update for record i. Let d_F,n_F,s_0,s_1,s_2,t_F be the six finish payload lengths. The exact logical archive length is

`A(T,X,messages) = 312+x + sum_(i<T)(72+Bpub+f_i+d_i+n_i+u_i) + d_F+n_F+s_0+s_1+s_2+t_F`.

Here312 is216 Header payload plus12 Header framing plus84 Finish/child framing;72 is Segment framing plus its five child frames. No graph/answer/quotient bytes occur in this direct formula. Static blob, SDK postcard envelope, outer proof and public receipt transport are separate.

For an admitted schedule, define `bytes(e)=16*nK(e)+32*nE(e)+2*ni16(e)+nRaw(e)+nMeta(e)` for each scheduled message event, counting arrays/lengths according to that event's **actual** codec. For proof section s,

`Len_s = 32 + sum_(e in EMIT_s) bytes(e)`.

All non-message framing already counted above must not be counted again. Exact Amax is the maximum of this formula over 1<=T<=16, allowed X lengths, legal first/interior/last schedules and bounded parser branches. A safe installed ceiling replaces each Len_s by its proved `SECTION_MAX_T16_v1` and x by Xmax, then maximizes over T; it must not assume “16 times one full-final segment” is the exact transcript.

Names still requiring finite generation/admission are **Xmax**, **EMIT_T16_v1**, **CODEC_SITE_T16_v1**, **SECTION_MAX_T16_v1**, maximum transcript bytes **L_transcript**, outer postcard envelope maximum **EnvelopeMax**, and capacities/trace geometry **OuterProfile(G)**. They are not derivable from scalar soundness charges alone. Generated source descriptors must match the c_role values derived above rather than accept prover-supplied counts. This document does not assert a numeric Amax or that an uncompiled G fits 512 MiB advice or any trace bound.

Terminal mathematical payload is exactly 49,152 A-image bytes +32,768 full E-partial bytes +131,072 signed-i16 response bytes =212,992 bytes, excluding descriptors, point/value and prior occurrence of the images. Signed-i16 little-endian response encoding with [-7168,7168] validation is a **new specified codec**, not present-source compatibility. If the images are already carried by the last shrinking section, count their wire occurrence there; if repeated in terminal count them again and compare. Do not add this subtotal blindly on top of a terminal Len_s that already includes it.

### 10.2 Derived finite dimensions and message-work counts

- Active rows <=16*4095=65,520, before canonical global padding. Per-segment main logical physical table volume is56*4096*16=3,670,016 K entries; two logical RAM endpoints add131,072. These are not receiver witness lengths or guest trace counts.
- Exactly `120*T+32+16+16+8 =120*T+72` shell coordinates, at most1992. One update has four original handles, PB counts32/56/16/16, D width6240, 3456 exact scalar relation rows and kappa12. Each suffix uses its registered rows, not guessed update kappa.
- Ordinary leaf cap specializes to `2P+2b_B+4=122`; aligned endpoints can reduce actual occurrences. For Main, reviewed reuse has at most118 raw-normalizer inputs; old receiver<=2, each RAM<=3. Their raw arities are22,22,21,21. These are list caps, not proof-byte counts.
- Original-domain + raw-normalization + strong-update scalar **charge** is1619 under Main reuse (generic overcount1708). Complete frontend/endpoint scalar charges are separately registered; c_E charge <2^26 for the conservative covered T16 grammar, and the policy ceiling2^40 is not an exact count of E draws or bytes.
- A degree-D, r-round sumcheck has exactly r adaptive challenge sites and r round messages; its atom count is r*D only if the selected codec emits exactly D compressed coefficients per round, or r*(D+1) for fixed full coefficient encoding. Add its actual endpoint/frontier/input messages, labels and counts separately. A batch has one independent coefficient per present member and max member rounds, **not** sum of member rounds.
- Let S_E be the actual number of E sample requests in EMIT, and S_C=120*T+72. Bounded candidate work is at most4*S_E 128-bit candidates and10,240*S_C position candidates plus40*S_C sign bits. Shell position candidate bit widths are7 for j=0..39. Hence total sampled bits are bounded by `512*S_E+71,720*S_C` (before any explicitly specified discard/alignment); accepted-only counts undercharge retries.
- For b=64 transcript blocks, append lifecycles of lengths B_l and V finalized snapshots, exact direct component-call count is `N_H=sum_l floor(B_l/64)+V+S_blocks`. S_blocks counts actual 1024-bit squeeze blocks under the frozen bit-cursor rule, including rejected candidates and any discarded tails. Recomputed forks count again unless a correct cache shares them. Absorbed bytes are canonical transcript emissions, not automatically the archive file length.
- Each H_1610 call absorbs232 bytes and uses exactly2 Keccak-f1600 permutations; thus the selected direct computation uses exactly `2*N_H` permutations. This eliminates graph/quotient work but is not a guest-cycle or end-to-end performance estimate.
- Expanded static matrix storage is74,907,648 bytes before metadata; one canonical coefficient pass touches4,681,728 K cells. Actual weight contractions may revisit matrices. Let `W_weight(e)` be the exact operation count of the emitted native weight-provider implementation, `W_front(e)` the native frontend terminal/lookup weight work, `W_codec` canonical parse/address work, and `W_field` the actual limb-arithmetic lowering cost. A complete guest work ledger is `W_codec + sum_e(W_front(e)+W_weight(e)+W_sumcheck(e)) + W_terminal + 2*N_H*W_KeccakPerm`, converted through W_field and the compiled instruction inventory. None of these uncompiled conversion constants is claimed to be one cycle or free.

A conservative finite correctness implementation can stream exact native coefficient actions using the negacyclic formula in §7; no dense R*N allocation is necessary. Performance admission still needs the actual weight-provider schedule, peak live buffers, C7 coverage and compiled fixed geometry. The formal macro cap38,871<65,536 and polynomial extraction ceiling in GLOBAL_MANIFEST are security/compiler limits, **not** feasible runtime estimates, Amax or guest instruction bounds.

## 11. Admission and hand-off boundary

An implementation of this specification must emit every finite descriptor/codec/count named above and demonstrate equality to the complete named native consumers, not substitute a generic proof parser. Required independent evidence is: source/K-to-E separation; exact transcript site/bit-consumption correspondence; all-valid original-path ownership and weak-unit comparison; complete final-decider replay; installed-static reference relation; full ordinary outer extraction; C7 compiled source-region/read/version induction; bounded capacities and no remaining source/setup obligations. The authorized heuristic replaces none of these gates.

No code/source edits, builds, tests, formatters, proof runs, benchmarks or Git operations were performed for this document. After implementation and explicit verification authorization, the integration owner should run the selected whole-program build/proof/verify path and negative cases for malformed/trailing/cross-frame lengths, noncanonical K/E atoms, changed basis/roles, missing/reordered stages, zero/oversized T or ell, endpoint/source substitution, wrong PC phase/mask/gamma, path/plane/zero-span omission, sampler exhaustion, partial final suffix, terminal response overflow, setup/cache substitution, source overwrite/VirtualAdvice substitution, output0 and panic. Same-semantics recursive-control and paired end-to-end performance remain required, not measured here.

**Implementation versus admission order:** this complete parametric algorithm/ownership contract is a design input for implementing the guest, not a demand that a compiled guest already exist before implementation may begin. Implementation produces the finite emission/codecs and actual guest instruction/region inventory; then its measured or proved worst-case resource bounds determine fixed outer geometry (or an authenticated finite family), generated rows and installation. Semantic/source/adequacy certificates must be discharged before claiming an admitted accepting implementation. They are neither optional heuristic assumptions nor circular prerequisites to writing that implementation.
