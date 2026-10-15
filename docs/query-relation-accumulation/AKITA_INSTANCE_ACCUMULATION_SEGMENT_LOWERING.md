# Exact continued-segment lowering of the native Jolt consumers

Status: independently reviewed exact relation/consumer design for the selected compact profile; composed numerical admission remains pending. The implementation update below does not establish end-to-end execution. The construction uses the immutable source pair and the [K-source reconstruction audit](AKITA_INSTANCE_ACCUMULATION_FRONTEND_DESCENT.md). The target is the original Boolean-table execution relation on concatenated active rows, not emulator equivalence or a stronger halting statement.

Implementation update: typed continued producer/verifier paths now span stages 0–8 and stop before PCS. They retain the original Main row-owner Arc and derive the RAM/register recurrence over full K values. The public context binds exact finite counters, program/preprocessing identity, native inputs/normalized outputs/panic, first/last active PCs, register endpoints, successor metadata and original RAM endpoint identities. Compilation and a complete two-segment execution are still pending; this is not an executed cutover or final receipt.

Whole-run wrappers retain their existing relation/challenge behavior. The stage-two RAM-output carrier is now `Option`, with no member on interior segments and `Some` on whole-run/final segments. This changes binary serialization; the continued protocol uses `jolt/continued-k/v1` and needs its own exact installed event plan. The committed-program boundary roster also includes the last-active architectural-PC mapping, for at most six metadata leaves; the selected full-public profile adds no such committed leaves.

A scoped review found a field-axis mismatch: ordinary RV64IMAC programs in field-enabled builds originally omitted the 16 continued field-register endpoints, although the selected verifier required them. The continued backend now selects endpoint presence by the compiled proof axis. Explicit `TraceMode::{WholeRun,Continued}` keeps ordinary whole-run profile rejection unchanged while permitting zero-activity continued field witnesses only after all raw rows are checked for absent field payload/instructions. Independent source re-review found no remaining defect; the field-enabled runtime cases have not yet run.

## 1. Public controls and source timing

Bind a finite segment count T, each active length ell, the ordered counter, program/layout identity and proof-side start/end state before that segment's frontend challenges. Derive `first := index==0` and `last := index+1==T`; they are not prover-selected optional checks. Require `1<=ell<Nseg`, with the registered CycleMajor geometry. All segment source handles, including C_Rminus and C_Rplus, are bound at stage 0. Public register vectors contain canonical K entries.

The incoming endpoint must equal the preceding outgoing endpoint as a **full typed original handle/state**, not merely an unverified digest. First/last PC moments authenticate the actual first and last active row indices. External successor metadata refers to the next segment's first row for an interior cut. At the final cut use the original padded successor `(NextPC,NextUnexpandedPC,NextIsVirtual,NextIsFirstInSequence,NextIsNoop)=(0,0,0,0,1)`; do not infer an architectural halt-PC theorem from that tuple.

The whole-run relation is reconstructed by concatenating active rows and applying the original global padding rule once. Source and virtual reconstruction, then frontend MLE evaluation, are over K in the selected profile. Only completed native accumulation endpoints cross into E2; this does not change original source ownership or physical layout.

## 2. Stage 2 RAM output consumer

Keep `RamReadWriteChecking` and its phase ordering in every segment. Its input/output source states now come from the segment-relative RAM recurrence in §3. Its full batch round count and address/cycle permutation are unchanged.

The original `RamOutputCheck` is a **global final** consumer. Instantiate it only on the last segment. It checks the exact original Boolean identity
\[
\operatorname{Mask}_{IO}(a)
\bigl(B^+(a)-\operatorname{Val}_{IO}(a)\bigr)=0,
\]
using its existing equality-weighted degree-three sumcheck and actual input/output/panic/conditional-termination table. Its `RamValFinal` leaf is now an alias of the original outgoing endpoint source C_Rplus, not a second uncommitted final-memory table. Keep its original dummy-round scaling and derived address point. Nonfinal segments do not check final output bytes or reimpose termination.

The original stage-2 batch aligns the RAM read/write address phase with the output-check address phase: the latter omits the former's initial cycle phase and uses the same batch address coordinates. Register and check this point identity explicitly. Thus the final segment's output-check value is already the outgoing endpoint evaluation at the address used by the stage-4 RAM value relation. A changed phase layout that breaks this equality must use a real extra query/reduction, not alias unequal points.

On an interior segment, append the outgoing endpoint evaluation at that same read/write-derived address before the stage-4 gamma. It is an ordinary C_Rplus obligation. There is no fabricated `RamOutputCheck` proof or successful proof result for the omitted global check.

## 3. Stage 4 RAM value and initialization consumers

The source-authoritative original consumer is [`RamValCheck`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage4/ram_val_check.rs), with the symbolic formula in [`relations/ram/val_check.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/relations/ram/val_check.rs). It already reduces a current value and a final value using `LtCyclePlusGamma` in n cycle rounds of degree three.

For each segment define the unique table
\[
V(a,t)=B^-(a)+\sum_{s<t}RA(a,s)\,Inc(s).
\tag{1}
\]
At the RAM read/write reference `(x,y)`, bind
\[
v=\widetilde V(x,y),\quad b=\widetilde B^-(x),\quad f=\widetilde B^+(x)
\]
before the existing value-check gamma. Replace the old input expression by
\[
v+\gamma f-(1+\gamma)b.
\]
Its output expression is still
\[
[\widetilde{LT}(s,y)+\gamma]\,
\widetilde{RA}(x,s)\widetilde{Inc}(s).
\tag{2}
\]
The n-round sumcheck leaves exactly the same `RamRa` and native `RamInc` virtual consumers at `(x,r_cycle)`. Preserve their existing stage-5/6/7 routing and fused-inc reconstruction. The two endpoint evaluations are additional original-handle leaves, not new RamInc or per-cycle memory commitments. Use the existing gamma domain label and typed site under the new protocol version; a second RAM gamma is unnecessary.

For the first segment only, additionally require
\[
\widetilde B^-(x)=\widetilde I_{original}(x).
\tag{3}
\]
Construct I_original using the current `RamValCheckInitialEvaluation` public/advice/program-image decomposition at x. Its staged auxiliary claims keep their actual stage-4 absorption order and original downstream reduction paths. The C_Rminus endpoint value b is the left side. Equation (3) is deterministic once those claims are bound; the complete existing ancestor address family supplies the interpolation needed for all addresses. It needs no invented unrelated init point. The global manifest's separately charged initial anchor is a safe upper bound when this existing family is reused.

Interior segments do not reconstruct I_original again. Their incoming state is the preceding C_Rplus. Advice/program-image objects required only for initialization need no new opening occurrence there; immutable bytecode groups still participate whenever used by bytecode pullbacks. Presence is derived from the consumer list, not from a prover-supplied empty-claim exemption. Static input/program identities remain bound in the run statement throughout.

Interpolate (2) over gamma and cycle coordinates, with the same-source premise, to obtain both (1) and
\[
B^+(a)=B^-(a)+\sum_s RA(a,s)Inc(s)
\tag{4}
\]
for every address a. The full ancestor address tree is essential. Equalities at one realized address do not establish the endpoint vector.

## 4. Stage 5 register consumers

The original [`RegistersValEvaluation`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage5/registers_val_evaluation.rs) consumes the stage-4 register read/write point, uses its seven address coordinates, and leaves `RdWa`/native `RdInc` at that address and a new cycle point.

Replace zero initialization by the public incoming K vector Rminus. Bind the incoming and outgoing vector evaluations at that same address, then draw one independent endpoint gamma. Use (1)–(4) with `RA=RdWa`, `Inc=RdInc`, `B-=Rminus`, `B+=Rplus`. The integrand remains degree three, the number of cycle rounds is unchanged, and the two downstream virtual leaves are unchanged. The original read/write relation then reconstructs Rs1Value, Rs2Value and RdWriteValue from this unique K prefix state.

Apply the identical change to the optional field-register consumer with its four address coordinates and original `FieldRdInc` K source. It adds one gamma and no new committed register table. The original field-byte decoding remains K arithmetic. Field products/inverses are still reconstructed from these K values; E is only the challenge/evaluation field.

Set the original initial register vectors once, on the first segment. Later vectors are linked by exact equality. Do not assert that every algebraically valid cut-state register is a u64; the selected interface keeps full canonical K values.

## 5. Bytecode entry, shift and CPU consumers

The native read-RAF entry leg must also change. `geometry/bytecode.rs::read_raf_raf_entry_publics` uses `eq(bits(entry_bytecode_index),r_address)*eq(0,r_cycle)`, and its input fold has constant target one at outer gamma power 11. Supply the authenticated **segment-start expanded index** here, not the program's initial index on every segment. On the first segment require equality to the original registered program entry index; later starts equal the preceding external successor. Keep the gamma exponent and constant target. This preserves first-row authentication without restarting the program or changing the degree/leaf ledger.

For an interior cut, replace the last active row's five successor values in `SpartanShift` by authenticated external metadata. Keep the first four legs at the native outer point and `1-NextIsNoop` at the distinct product point. Relative to the padded successor, the corrections are
\[
\operatorname{eq}(r_{outer},\ell-1)\,external_j
\quad(j=PC,UnexpandedPC,IsVirtual,IsFirst),
\]
\[
\operatorname{eq}(r_{product},\ell-1)
(1-external_{IsNoop}).
\]
They are public coefficients in the shift relation, not new private columns. Bind successor flags, expanded/architectural PC and program identity using the exact full-public or committed-program metadata queries in the segment interface. Check the unguarded interior-cut predicate only on a real interior successor. At the final segment keep the original padded tuple.

Keep the unchanged row-local CPU, product, lookup and bytecode equations on the **full padded domain**; a new multiplicative CPU mask is unnecessary. Canonical inactive rows use the original no-op metadata and zero increment/access sources. The instantiated `JoltInstruction::Noop` flags in `jolt-riscv/src/instructions/mod.rs` include `DoNotUpdateUnexpandedPC=1` and `IsNoop=1` (the marker declaration alone is insufficient evidence). With PC/operand/lookup/increment data zero, the only ungated sequential-PC row in `composed/r1cs/rv64.rs` reads `0=0+4-4`; the other active ungated rows have zero operands. Loads/stores/writes and field-operation constraints have zero guards. Prefix states remain constant through these no-op rows; any unused register read values are not new CPU constraints. Product/recurrence definitions remain enforced, not replaced by arbitrary zero virtual witnesses. Thus padding can stutter at an interior proof-side K state without changing the row-local relation or raising its sumcheck degree. Actual active noops remain legal; do not assert `!IsNoop` on every active row.

The main-source zero check uses the **column-specific** inactive/omitted-address masks in [main source reuse](AKITA_INSTANCE_ACCUMULATION_MAIN_SOURCE_REUSE.md). Complete native Booleanity, digit-zero reconstruction and that mask jointly prove every physical main coefficient's source bound. Optional field-increment padding is checked on its full padded physical polynomial; gadget-decoded zeros do not justify forcing a supplied noncanonical raw decomposition to canonical zero digits.

## 6. Leaf ledger and extension descent

The revised consumers terminate at:

- the same P original main column leaves after stages 6/7;
- existing advice/program/image/field leaves when actually consumed;
- two original RAM endpoint leaves at the shared read/write address;
- existing first/last PC moment and source-zero-check leaves;
- up to five committed-program boundary metadata leaves.

The earlier ordinary cap, including allowance for separate initial/final points, remains a conservative upper bound. Reusing their existing aligned address does not create extra leaves. The generic global scalar bound also overcounts the reused RAM gamma and initial/final checks; do not advertise that upper bound as an exact transcript length.

Every execution table is still reconstructed bottom-up over K: digit-zero columns; balanced fused increments; bytecode/lookup pullbacks; RAM/register prefix states; read/write values; shifts; and product lanes. The modified recurrences add only authenticated K inputs. The original alias equalities and point identities remain explicit. Therefore the [fixed-source extension lemma](AKITA_INSTANCE_ACCUMULATION_FRONTEND_DESCENT.md) applies to the revised consumers too, once their complete source-linked trees are supplied. Serialized E claims are not additional existential execution witnesses.

## 7. Concatenation theorem and completeness boundary

Forward: restrict a valid original whole-run Boolean-table witness to legal source/inline intervals; choose proof-side prefix RAM/register states at each cut; encode each segment's main columns with their canonical omission/inactive zeros. Equations (1)–(4), external successor metadata, original init and final I/O all hold. Form each source under the new registered profile and preserve every supplied valid noncanonical commitment path. This does not reuse an incompatible old commitment by changing its metadata.

Backward: recover the unique K tables for each accepted segment through the complete frontend/native tree. Equations (4) and exact endpoint equality make each next initial state equal the preceding final prefix state. The corrected shift metadata supplies the actual next active row across each interior boundary. Every unchanged row-local equation therefore holds on the concatenated active table. Apply original initialization once, original final I/O/padded successor once, and canonical global padding. This reconstructs a witness for the original whole-run relation, not independent sampled MLE equalities glued together.

The statement assumes legal canonical program preprocessing and the frozen profile/point/layout rules. Other segment/source geometries need their own admission certificate. No stronger host-advice, emulator-memory, canonical-u64 or halting theorem is inferred.

## References and implementation boundary

- [Immutable complete virtual-table reconstruction](AKITA_INSTANCE_ACCUMULATION_FRONTEND_DESCENT.md).
- [Original stage/challenge ledger](AKITA_INSTANCE_ACCUMULATION_FRONTEND_MANIFEST.md).
- [Segment interface](AKITA_INSTANCE_ACCUMULATION_SEGMENT_INTERFACE.md), including source roles, PC moments, metadata lanes, memory-map and advice semantics.
- [Global manifest](AKITA_INSTANCE_ACCUMULATION_GLOBAL_MANIFEST.md), [compact registry](AKITA_INSTANCE_ACCUMULATION_MATRIX_REGISTRY.md).

The paired `prepare_continued` / `prepare_continued_instance` paths now implement these named continued consumers and return original pending source obligations before PCS. Ordinary whole-run paths keep their initialization/finalization. The integrated two-segment driver, common multipoint reduction, complete accumulation update and final receipt still require execution; none is established by this relation audit alone.
