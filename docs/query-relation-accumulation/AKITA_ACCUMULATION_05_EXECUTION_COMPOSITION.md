# Obligation 05 — execution composition, public statements, and memory

## 1. Success criterion and result

Replacing repeated Akita opening verification does **not** by itself turn a list of Jolt proofs into a continued execution. The application-level success criterion is:

> An accepting final receipt for a named setup, program, run, public input, and public output establishes a **nonempty, ordered path of actual VM transitions**, starting in the prescribed initial state and ending in the prescribed successful halt state. Every carried piece of machine, memory, advice, and I/O state is authenticated by the proved relations, not merely by supplied metadata.

**Preferred concrete construction:** explicitly expose complete instruction-boundary state; authenticate mutable RAM with a fixed-format collision-resistant Merkle tree; prove every access and root update in a new segment PIOP supported by the lattice/Akita PCS; and have the history receipt enforce the initialization, links, ordering, counters, and terminal predicate. This is intentionally a conservative memory construction whose costs are acknowledged. It introduces a hash collision-resistance assumption, **not** an elliptic-curve/group assumption and **not** a hash-based PCS. Section 8 gives a lower-hashing, Akita-endpoint-vector alternative, but does not silently treat its missing PIOP and commitment-domain arguments as deployed results.

This is an explicitly selected **new memory frontend**, not a claim that adding hashes to Stage 8 preserves or already implements native Jolt's Twist relations. Native Akita accumulation remains the target for both candidate routes. The hash-based memory option is chosen here for its direct continuation argument; §8 retains the native one-hot/increment path as the performance-oriented alternative with its own missing endpoint reductions.

**Classification.** The semantic path-composition lemma in §5 and the authenticated-memory induction are justified arguments under their stated hypotheses. Compiling this relation into sound segment proofs and into a sound history receipt is conditional. An adaptive native Akita accumulator theorem, fixed receiver shape, bounded decoded domains, and end-to-end extraction are separate open obligations; this document does not solve them by renaming an application digest an accumulator.

Research only: this document contains no implementation or branch changes. Related deliverable: [history and final acceptance solution](AKITA_ACCUMULATION_06_HISTORY_AND_FINAL_ACCEPTANCE.md), owned by ReceiptSolution.

## 2. Source-backed baseline

1. **Jolt has separable semantics and PCS obligations.** [Jolt, §§2.2, 3.2, 7](SOURCE_PROVENANCE.md#jolt) describes a PIOP combined with a multilinear PCS; CPU step constraints alone rely on memory consistency and instruction lookups. Its memory discussion distinguishes bytecode, registers, and RAM. This is a historical architecture statement, not evidence that current Jolt exposes resumable execution endpoints.
2. **Continuation must match registers and memory.** [Proving CPU Executions in Small Space, §1.1.1](SOURCE_PROVENANCE.md#cpuexecinsmallspaces) explicitly requires the end of shard $i$ to match the start of shard $i+1$. Section 1.1.3 explains that both Merkle hashing and elliptic-curve multiset hashes can be expensive; §1.1.4 discusses memory-size-linear continuation costs for Twist/Shout. These are the paper's observations, not measurements made here.
3. **IVC proves a path, not an unordered conjunction.** [Nova, Definition 5 and Construction 3](SOURCE_PROVENANCE.md#nova) binds $(i,z_0,z_i)$ and uses $z_{i+1}=F(z_i,\omega_i)$ plus authenticated recursive bookkeeping. [HyperNova, §§5–6.1](SOURCE_PROVENANCE.md#hypernova) similarly binds state and a designated control function that selects the next relation. The file named `SuperNova.md` actually contains HyperNova. These papers supply an interface pattern; their group-based constructions are not imported as a lattice implementation, and their constant-depth knowledge statements do not prove arbitrary-depth Akita extraction.
4. **A primary deployed interface includes authenticated memory.** RISC Zero [SystemState 3.0.6](https://docs.rs/risc0-zkvm/3.0.6/risc0_zkvm/struct.SystemState.html) exposes PC and a Merkle root of the memory image. Its [recursive proving documentation, version 3.0, “Recursion Programs”](https://dev.risczero.com/api/recursion#recursion-programs) distinguishes joining execution spans within a session from resolving assumptions. This document does not infer that PC plus a root is sufficient for Jolt from that other machine's representation: Jolt's separate register/control state must be accounted for explicitly.
5. **Current Stage 8 is an opening-verification seam, not a segment contract.** At native-batch pin `0791f79a55c3fadc3d327c78a72228696c338468`, [`stage8/akita.rs::verify`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage8/akita.rs#L218-L414) validates metadata and derives ordered native trace and auxiliary claims before calling `PCS::verify_batch`. Its arguments do not include a continuation start/end RAM root or state. [`Stage8Output::Clear`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage8/outputs.rs#L31-L45) is payload-free on Akita because verification presently finishes there.
6. **Current RAM initialization/output relations are not arbitrary continuation endpoints.** [`ram_val_check.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage4/ram_val_check.rs#L1-L57) reconstructs initial RAM from public evaluation and advice/program-image contributions. [`ram_output_check.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage2/ram_output_check.rs#L1-L88) checks the public-I/O memory region; it is not a public commitment to all final RAM. [`ram_read_write_checking.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage2/ram_read_write_checking.rs#L1-L134) exposes reduced value/address/increment claims, not a cross-proof RAM boundary object.
7. **The pinned recursive guest proves independent conjunction.** At Blake pin `5c65167e269fdfc2917ecb82f420d349b18b4c87`, [`examples/recursion/guest/src/lib.rs`, lines 138–207](https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/examples/recursion/guest/src/lib.rs#L138-L207) reads one setup, then $n$ proof/device pairs, calls the full verifier independently, and returns the conjunction. There are no consecutive-state comparisons in the inspected loop. `all_valid` starts true, so $n=0$ returns one. Its verifier result may also be zero without the guest panicking. The [`outer` runner](https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/examples/recursion/src/outer.rs#L116-L124) rejects guest panic; the inspected proving/verification suffix [lines 260–314](https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/examples/recursion/src/outer.rs#L260-L314) proves the observed public device and does not require the guest result to equal one. A consumer must check the intended accepting output and bind the guest image/setup/input. This is an acceptance obligation, not a claim of an exploit in a consumer that already does so.

Response chunks within an Akita PCS argument remain **not** VM segments. None of the above sources supplies the continued-segment implementation specified below.

## 3. Exact application and segment relation

### 3.1 Frozen profile and application identity

Freeze a profile $P$ before proving, containing the ISA/version, instruction and syscall semantics, executable/image identity and allowed preprocessing keys, program-memory map and permissions, field/word encodings, RAM address width and leaf format, hash suite and domain tags, endpoint schema, tape policy, public-I/O format, segmentation policy, and bounds. It also incorporates the accumulator's matrices/layouts, source catalog, challenge domains, and fixed receiving shape. The application must select the acceptable $P$, not accept any prover-supplied identifier.

Use a public run identity $r$ supplied or approved by the consuming application; it prevents cross-run splicing but is not, by itself, a replay-prevention service. The program identity $p$ binds **both** the executed instruction representation and initial program image, plus any trusted-advice authority/profile. A verifier-key digest alone is insufficient if that key is universal across programs.

Define the public application statement

$$
X=(\mathsf{version},P_{\rm id},p,r,x,y,\mathsf{policy},n,N_{\rm ISA},N_{\rm row},N_{\rm mem}).
$$

Here $x,y$ are canonically encoded public input/output byte strings **including lengths**, or typed commitments to them with verified opening relations. The totals may be public exact counts as above or constrained to a public allowed interval; the profile must choose one meaning. We use exact counts and $1\le n\le T_{\max}$, with at least one real ISA instruction per segment. Arbitrary external interaction is excluded unless the profile supplies an explicit authenticated external-event relation. An untrusted host response is otherwise private advice, not a fact about the world.

### 3.2 Complete carried state

Restrict cuts to completed ISA instruction/virtual-sequence boundaries. No cut is allowed halfway through a decomposed instruction, unfinished syscall, precompile, or hash operation. This yields a simpler explicit state; an alternative allowing such cuts must carry and constrain all microcode phase and in-flight state instead.

Let

$$
B=(\mathsf{pc},\mathsf{regs},\mathsf{ctl},\rho,\tau_{\rm ISA},\tau_{\rm row},\tau_{\rm mem},\mathsf{io},\mathsf{adv},\mathsf{status}).
$$

- `pc`: exact next instruction address/index and its interpretation under the bound program.
- `regs`: all architecturally live registers, including a constrained zero register; any virtual/field registers not provably dead at a legal cut are included. Word values have checked canonical encodings, not unconstrained field elements.
- `ctl`: privilege/exception state, syscall state, and any machine-specific flags that affect future transitions. In the restricted profile, unsupported privileged/concurrent/nondeterministic device facilities are absent by specification, not silently ignored.
- $\rho$: authenticated RAM root, covering every mutable byte/cell and permissions that affect execution. Program code can instead be immutable under a separately bound bytecode/image commitment; if self-modification is allowed, fetches must use the authenticated memory and code-change semantics.
- counters: global integer counts of real instructions, expanded trace rows, and ordered memory events. They are distinct: virtual instructions and padding need not equal real ISA cycles. Check integer ranges and no overflow; field wraparound is not an allowed counter transition.
- `io`: public input identity/length and read cursor, output/journal append length and digest, and any device cursor/control state. Complete immutable input may be carried by identity, but each consumed byte is constrained to that input. Output digests are produced by proved append transitions.
- `adv`: fixed private-input/tape commitment and length, trusted-advice identity, and consumed-position state. If the VM permits reusable random-access advice, authenticate addresses instead of pretending it is a sequential tape. Per-instruction nondeterministic arithmetic hints are allowed only when the instruction relations constrain them to the intended semantics; they are not substituted for committed application private input.
- `status`: `Running` or `HaltSuccess`; trap/panic/failure states are explicitly nonaccepting. A pause between segments is not a successful halt.

The profile fixes the entire schema. Equality of `regs` means equality of every live component, not just registers used in the next few rows. Use public $B$ for the conservative contract. If boundary details are private, use $d_B=H(\mathsf{boundary}\|P_{\rm id}\|p\|r\|\operatorname{enc}(B))$ and **prove its preimage relation at both endpoints**. Hashing is not a substitute for constraining the state itself.

### 3.3 Segment statement and witness

For segment $i\in\{0,\ldots,n-1\}$, define

$$
S_i=(P_{\rm id},p,r,i,B_i^-,B_i^+,\ell_i^{\rm ISA},\ell_i^{\rm row},a_i,L_i,\Delta_i^{\rm io},\mathsf{cut}_i).
$$

$L_i$ is the padded proof-trace capacity, $\ell_i^{\rm row}$ the active expanded rows, $\ell_i^{\rm ISA}$ the retired ISA instructions, and $a_i$ the number of memory events. Require $1\le\ell_i^{\rm ISA}$, $1\le\ell_i^{\rm row}\le L_i\le L_{\max}$; legal capacities/layouts come from $P$. Active selectors form one prefix, and inactive padding is provably inert. $\mathsf{cut}_i$ certifies a legal complete-instruction boundary. The output fragment $\Delta_i^{\rm io}$ is included directly or by a length-bound digest with a proved relation to bytes emitted by execution.

A witness $w_i$ contains the actual step/microstep trace, advice/private-input values and their authenticators, RAM paths, intermediate roots, output bytes/hash witnesses, and any commitment hints needed by Candidate B. The relation $R_{\rm seg}(S_i;w_i)$ requires:

1. First and last **active** state equal $B_i^-,B_i^+$, not merely an opening at the last padded row.
2. Every active transition obeys the bound ISA/program, instruction lookups, register updates, permissions, byte/word/endian rules, advice policy, and I/O rules; every memory access obeys §4. All constraints share the same trace witness/commitments.
3. $B_i^-.$status is `Running`; only the last segment may end `HaltSuccess`. The legal cut is established from trace/control state.
4. Endpoint counter differences equal $(\ell_i^{\rm ISA},\ell_i^{\rm row},a_i)$; counters, tape cursors, and output lengths stay in their profile ranges without wraparound. Public input/advice identities remain fixed. Output digest and bytes evolve by the prescribed append relation; no reset is allowed.
5. Program/setup/run/index/endpoints/lengths/cut and public I/O are included in the **original segment proof transcript before its challenges**. A later history hash cannot retrospectively bind an omitted original statement.

`Init(P,p,r,x,B_0^-)` fixes entry PC, register/control values, all counters, cursors, empty output state, status, and RAM initialization. Public program/input/default RAM root can be computed once from the specified initialization map. Private initialized regions require a proved relation connecting their canonical values, fixed private-input commitment, and this root. Merely supplying an arbitrary initial root is insufficient. An application specifying existential private input allows any one committed tape satisfying its format; one specifying a particular private input must bind its commitment publicly.

`Final(P,p,r,x,y,B_{n-1}^+)` requires a real constrained `HaltSuccess`, no panic/trap, expected exit code, prescribed final journal/output and length, and required input/advice consumption policy. All unused-input policies must be explicit: “consume all” and “may leave unread” prove different statements.

## 4. Concrete cross-segment memory method

### 4.1 Authenticated RAM relation

Use a fixed-depth binary Merkle tree with $M=2^h$ canonical byte leaves, indexed by $h$ checked address bits. Leaf format includes value and any mutable permissions; address interpretation and memory-map version are bound by $P$. Node and leaf encodings have distinct tags, fixed widths, and ordered children. One possible definition is

$$
\mathsf{leaf}(v,\pi)=H(\mathsf{RAMLeaf}\|\mathsf{mapId}\|\operatorname{enc}(v,\pi)),
\qquad
\mathsf{node}(l,u)=H(\mathsf{RAMNode}\|\mathsf{mapId}\|l\|u).
$$

Position is authenticated by the ordered path and checked address bits. Default/unallocated cells have an explicitly defined value and permissions; sparse-tree representations use the corresponding canonical default subtrees. Range-check the value as a byte and the permission encoding. Do not reduce a 256-bit hash modulo fp128 and then compare it: that loses binding. Carry its full fixed-width bit/limb representation with limb range checks.

Each memory event $e$ takes root $\rho_e$ and address $a_e$, old value/permissions $(v_e,\pi_e)$, and siblings $s_{e,0},\ldots,s_{e,h-1}$. In the PIOP's arithmetic/lookup constraints, compute the ordered path root and require

$$
\mathsf{Root}(a_e,\mathsf{leaf}(v_e,\pi_e),s_e)=\rho_e.
$$

A read returns **this** $v_e$ to the CPU trace and carries $\rho_{e+1}=\rho_e$. A write checks write permissions and the CPU-computed new byte $v'_e$, then uses the **same path and address** to set

$$
\rho_{e+1}=\mathsf{Root}(a_e,\mathsf{leaf}(v'_e,\pi'_e),s_e).
$$

Read-before-write authentication is mandatory even for a store that never needs the old value in its ISA result. Permissions cannot change unless the specified machine instruction authorizes it. Multi-byte and read-modify-write instructions expand into a bounded, precisely ordered list of events with checked endian/sign-extension/overlap behavior. Every RAM read/write affecting the CPU—including I/O memory if it uses RAM—is accounted for. Events are bound to instruction/microstep indices, not checked in an unrelated auxiliary trace with unproved membership.

Set $\rho_0=B_i^-.$root and $\rho_{a_i}=B_i^+.$root. Consecutive event roots match; the root cannot reset between events or segments. Registers remain separately constrained; placing them inside a memory image is also valid only if all register access semantics are correspondingly authenticated.

### 4.2 Why this is compatible with lattice/Akita

The hash algorithm is a fixed Boolean/arithmetic relation. Bit constraints, bounded-word additions/carries, and hash-round constraints can be reduced by a PIOP to openings of committed witness polynomials. Those polynomials use **Akita**, with a new frozen supported source layout/alphabet for these hash witnesses. No Pedersen commitment, group multiset hash, pairings, or DLOG assumption is needed. Hash witnesses cannot be hidden behind unverified host hash calls or an unconstrained accelerator.

SHA-256 is a concrete conservative **classical** baseline, with cost reflected below; its collision-resistance assumption is additional to Akita's lattice assumptions. An actual post-quantum security profile must choose and justify an appropriate hash/output length and security model. A 256-bit digest must not be advertised as giving generic 128-bit quantum collision security; an idealized $q$-query quantum collision scale is approximately $q^3/2^{256}$, rather than the classical birthday scale. A larger hash suite may be required. This document does not prove QROM security of the accumulator or receipt.

This is hash-based **memory authentication**, not a hash-based **polynomial commitment scheme**. Transcript hashing, history hashing, and RAM hashing have different roles and domains; ReceiptSolution uses a separately bound history/accumulation namespace. The PCS remains lattice-based.

### 4.3 Authenticated-memory induction

Assume the initial root corresponds to the prescribed canonical RAM image. If two different leaves/paths open the same root at the same address, compare their computations from root toward the first different node; either a node hash has different inputs and the same output or the leaf hash does. Thus a computationally produced inconsistent opening gives a hash collision.

Induct on the ordered memory events. A verified read must return the current canonical leaf, except with a collision. A verified update using the old authenticated path produces the root of the image with exactly the authorized leaf change. Untouched subtrees remain the siblings of that authenticated path. Consequently the last root authenticates the state produced by the segment. The next segment starts from that same root and cannot introduce a different value at its first access without a collision. This also carries cells that were not accessed in either adjacent segment: their last write can be arbitrarily far back.

This argument requires paths/CPU operations whose validity is established by the proof system. For a cryptographic reduction from private proofs, those witnesses must be obtainable under the claimed segment/receipt extraction theorem, or covered by a directly applicable authenticated-memory argument theorem. Merely asserting “soundness of each unrelated proof” is not a generic theorem about efficient extraction of a shared hidden execution. The algebraic induction is not a solution to the native accumulator's adaptive extractor problem.

## 5. Compose and the path-composition argument

Define the exact application predicate

$$
\begin{aligned}
\mathsf{Compose}(X;S_0,\ldots,S_{n-1})={}&
(1\le n\le T_{\max})\ \land\ \mathsf{Init}(P,p,r,x,B_0^-)\\
&\land\bigwedge_{i=0}^{n-1}\mathsf{IdsAndIndex}(S_i,P,p,r,i)\\
&\land\bigwedge_{i=0}^{n-2}(B_i^+=B_{i+1}^-\ \land\ B_i^+.\mathsf{status}=\mathsf{Running})\\
&\land\mathsf{TotalsAndLimits}(X,S_0,\ldots,S_{n-1})\\
&\land\mathsf{Final}(P,p,r,x,y,B_{n-1}^+).
\end{aligned}
$$

The complete relation additionally requires $\exists w_0,\ldots,w_{n-1}:\bigwedge_i R_{\rm seg}(S_i;w_i)$. Order/count/active-length policy is checked even when metadata already appears in a hash chain. For each segment, `IdsAndIndex` binds the same program, run and setup and its exact index, and rejects a statement derived from a different proof/device. `TotalsAndLimits` sums actual active counts, checks $N_{\rm ISA},N_{\rm row},N_{\rm mem}$, enforces all capacity bounds, and requires exact I/O/tape identities/cursor links. Integer addition is checked outside field wraparound.

**Semantic lemma.** Suppose all segment witnesses satisfy this relation, RAM authentication is binding for the produced witnesses, and the endpoint schema is complete. Then concatenating the active traces yields an execution with the prescribed initialization and final success predicate.

*Argument:* The first trace starts at `Init`. Its internal transitions and memory operations are valid. Exact endpoint equality supplies the next trace with the same PC/register/control/cursor/counter state; §4 supplies the same RAM contents at every subsequent access. Therefore the next transition is a valid transition of the same machine/run. Repeat through all $n$ traces. Active counts add, no padding introduces additional transitions, no intermediate halt is treated as continuation, and `Final` gives precisely the intended output and successful halt. There is no probabilistic cancellation in these equality links.

Conversely, any execution satisfying the profile can be divided at legal complete-instruction cuts into such witnesses, provided each segment fits its advertised active/proof/hash-witness capacities. This is semantic completeness of the segment relation, **not** all-valid-state completeness of Candidate B's decoded opening domain. Immediate successful halt is supported as one active segment containing its actual halt instruction; zero-step/zero-proof success is disallowed by this profile.

### 5.1 When digest comparison suffices

Comparing adjacent boundary digests suffices when each proof authenticates the entire canonical boundary preimage, its actual trace endpoints, and all state affecting future execution; the RAM root has the verified access/update relation above; both proofs use the same profile/map/hash/encoding; and the receipt authenticates the comparison. Computational binding then replaces explicit endpoint equality. Counters/index included in a boundary digest must use the same boundary convention on both sides; do not include an “end”/“start” tag that makes equal states hash differently. The segment index remains a separately bound statement component.

Digest comparison **does not** suffice when a prover simply supplies the digest, the RAM trace is checked against a freshly initialized private memory unrelated to that root, only the public-I/O region is hashed/checked, a timestamp/tape cursor or virtual-register phase is omitted, or a write updates a claimed root using different siblings from its authenticated old leaf. The digest of an access transcript is not a commitment to the full mutable state: two locally valid transcripts can still start from inconsistent values in unrecorded memory. A shared root with no proved access relation merely repeats the same string.

## 6. Required Jolt PIOP/interface changes, including Stage 8

These are proposed relation/API changes, not changes present at the pinned source.

1. **Add a segment mode and statement type before Stage 1.** Extend validated inputs with the full $S_i$ and application context. Bind them in the original preamble before sumcheck/PCS challenges. Retain current preprocessing, memory-layout, advice-presence, trace-order, role and shape checks; add acceptance-controlled profile/run/program identities. The current [Akita verifier signature and input validation](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/verifier.rs#L173-L195) accepts preprocessing/public device/proof/trusted-advice commitment, not the above continued-state contract.
2. **Constrain actual endpoints and legal cuts.** Endpoint evaluations must be reduced to authenticated trace claims, with active-prefix, first/last-active-row and complete-virtual-sequence constraints. Add all live register/control state and checked counts. Proof capacity (`trace_length`) is not an active cycle count. Current [validation](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/verifier.rs#L355-L383) checks memory layout, public-I/O sizes, and power-of-two padded trace bounds; this does not establish the specified active segment interval.
3. **Make memory resumable.** Add the RAM root/access/update PIOP and bind its events to the CPU memory operations. Replace the per-proof initialization assumption in segment mode with the authenticated incoming root. Existing init reconstruction remains the `Init` case, not a reinitialization applied to every continuation. If existing Twist/increment checks are retained, generalize them to arbitrary authenticated incoming values or prove their compatibility; applying zero/program initialization to every segment can exclude valid paths or prove the wrong relation. Do not merely append root metadata to the current Stage 8 function.
4. **Add an authenticated I/O/advice relation.** Current [public-output normalization](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/verifier.rs#L436-L443) truncates trailing zero bytes, and the [preamble](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/verifier.rs#L634-L670) binds the normalized outputs and panic flag. This is not the exact byte-length journal contract in §3. The new relation must authenticate output length/append state, or explicitly choose the existing zero-padding equivalence as its weaker output semantics. Exact-output callers cannot infer equality of `[3]` and `[3,0]` from that normalized device. Similarly, one per-proof advice commitment is not a proof of fixed global tape consumption across segments.
5. **Extend the claim ledger/catalog.** New endpoint, active-selector, root/hash, tape, and journal witnesses have commitments and reduction claims. Add their typed roles, widths, source maps, canonical handle identities, and range predicates to the frozen profile and PIOP output ledger. New Boolean hash witnesses need not be physical one-hot trace columns; use appropriate supported Akita sources. Every evaluation used by a frontend check must appear in the exact retained/deferred ledger. The existing trace group's common-point invariant remains for that group; unrelated root/hash/endpoint groups can have their own points and must not be forced into a false common-point assertion.
6. **Introduce pending, not accepted, Stage 8 output.** After all earlier reductions and metadata checks, return

$$
\mathsf{PendingSegment}=(S_i,\mathsf{checkedContext},\mathsf{transcriptState},\mathsf{orderedOpeningLedger},\mathsf{unresolvedObligations}).
$$

All fields are deterministically derived from the exact proof/public device/validated context, with statement endpoint claims tied to its frontend proof. The current canonical role order, packing, original handles, points/evaluations and challenge history are preserved. This is a status meaning “frontend checks performed subject to these openings,” not “segment accepted.” Candidate A defers only its exact verifier-residual lanes; Candidate B consumes original prover hints before the backend full PCS proof. Public frontend reduction checks do not magically become established CPU semantics until those obligations are discharged.
7. **Compose in the authenticated receipt.** Run the comparison/counter/Init/Final predicates on the checked statements and derive the next boundary/history state. The final consumer authenticates the selected receipt guest/verifier, profile/program/run/input and expected output, then requires acceptance. A panic-free outer proof of a guest returning zero is not an accepting application receipt.

This is a multi-stage frontend change with a Stage 8 accumulation seam. No rearrangement confined to `PCS::verify_batch` can supply the omitted semantic relations.

## 7. Compatibility and exact coupling to ReceiptSolution

For both candidate routes, use history state

$$
z_t=(P_{\rm id},p,r,x,t,B_0,B_t,N_t^{\rm ISA},N_t^{\rm row},N_t^{\rm mem},h_t,a_t).
$$

For a fresh checked segment $S_t$, the receipt's `Step` must establish the retained frontend predicates, exact statement derivation, boundary/index/cursor/counter checks, and the chosen accumulator update. It derives $z_{t+1}$ rather than accepting a prover-specified successor. The pending-opening ledger binds the original proof's commitments/evaluations and the additional memory/endpoint claims. The history hash is computed **after binding the full old state and new input**, with its suite/domain distinguished from RAM hashing; it is evidence of checks only because `Step` is proved or replayed.

- **Candidate A:** completed segment proofs must first be proofs of this *new* segment relation. For $V_{\rm full}=V_0\land\bigwedge_\lambda A_\lambda x_\lambda=y_\lambda$, the receipt proves/replays $V_0$, `Compose` links and updates, then checks the final residual lanes. Root/hash constraint evaluation claims belong in that split. This does not avoid hash constraints or turn existing independent proofs into continuation proofs.
- **Candidate B:** the segment PIOP yields bounded fresh original application groups, including memory/endpoint witnesses. These enter only as new root-level claims; recursive receiver groups subsequently carry the live opening. Original-handle consistency must ensure this successor establishes the actual frontend's claims. Freeze max hash rows/groups and receiver capacity before challenges. Enlarging the frontend source catalog can change the shape fixed point and domain budget; the other obligations must price it. Retain boundary $B_t$ as authenticated application/history state separate from the folded commitment. A receiver commitment is not itself a RAM root.
- **Finalization:** require $t=n>0$, totals, `Final`, complete final opening/domain decider, and no unresolved obligations. If a recursive receipt construction leaves the latest step or latest ordinary instance outside the running accumulator, its terminal verifier must also authenticate/check that instance. Do not omit the final `Step` under a “history already folded” intuition.

The communicated ReceiptSolution contract is `Init`, all exact links, nonzero count and active lengths, fixed identities, legal counters/cursors, `HaltSuccess`, and a frontend obligation ledger covering all root/access/hash endpoint claims. SHA-based RAM-root domains are separate from its Blake2b history/accumulation domain, with both selected by $P$.

## 8. Lower-hash alternative: full RAM endpoint polynomials

For an explicitly finite RAM size $M$, there is a plausible lattice-only memory interface that avoids per-access Merkle hashing. It is **not assumed deployed or fully reduced here**.

Represent incoming/outgoing RAM by canonical bounded vectors $V^-,V^+\in\{0,\ldots,255\}^{M}$ and original Akita commitments $C^-,C^+$. Carry the **same original outgoing handle/commitment** as the next incoming one; independently recommitting the same vector need not produce equal commitments under randomized formats. Prove incoming/outgoing vector ranges, complete address coverage/map, and the CPU memory relation. For one-hot read/write addresses $r_{a,j},w_{a,j}$ and increments $\delta_j$, the exact Boolean-domain identities are

$$
V(a,j)=V^-(a)+\sum_{k<j}w_{a,k}\delta_k,
\quad
\mathsf{rv}_j=\sum_a r_{a,j}V(a,j),
$$

$$
\delta_j=\mathsf{wv}_j-\sum_a w_{a,j}V(a,j),
\quad
V^+(a)=V^-(a)+\sum_jw_{a,j}\delta_j.
$$

One-hot/range/CPU constraints make these prefix equations precisely sequential memory semantics. Adapt Twist's read/write/prefix sumchecks to the extra $V^-$ term and add the final-vector equality sumcheck. At a fresh verifier-selected address point $u$, a final equality reduction authenticates $\widetilde V^+(u)$ and $\widetilde V^-(u)$ plus the corresponding address/increment claims; all endpoints enter the opening ledger. A random-point equality test is valid only after vector commitments are bound, with its degree/field error included in the new PIOP analysis. Public outputs alone do not constrain $V^+$ over the rest of RAM.

This imports **PIOP algebra**, not Dory/Pedersen/group commitments. [Twist and Shout, ePrint 2025/105](https://eprint.iacr.org/2025/105) describes one-hot addressing/increments and logarithmic verification; [CPU §3.3](SOURCE_PROVENANCE.md#cpuexecinsmallspaces) gives the zero-initialized overview. The endpoint generalization written here is a proposed derivation, not a theorem quoted from either source. Need original commitment binding on the decoded bounded endpoint domain, nonzero-initial-state soundness/completeness, correct packing/range handling, fixed source shapes, and adaptive opening composition. A short Ajtai equation without the appropriate range/domain certificate does not bind an arbitrary full RAM vector.

Cost generally includes $O(M)$ endpoint work per segment plus trace sumchecks/commitments; CPU §1.1.4 explicitly warns about memory-size-linear continuation overhead. This route is attractive when $M$ is moderate and segment length is large, and unattractive for enormous sparse address spaces or tiny segments. It is not a justification for an unacknowledged elliptic-curve multiset hash. The preferred Merkle method avoids dependence on a new endpoint-vector binding theorem but pays many hash constraints.

A third strategy—commit all segment access transcripts, sample common fingerprint challenges only after all are bound, and prove a global memory argument—can also avoid local roots. It requires a real global argument over the same CPU access tuples, correct initial/final memory, global ordering/timestamps, and commitment openings. An incrementally updated fingerprint with later adaptive events is not equivalent to that commit-then-challenge construction. A bare transcript hash supplies none of these conditions.

## 9. Adversarial boundary cases

| Candidate attack | Why the weak contract accepts | Required rejection |
|---|---|---|
| Segment 0 stores `RAM[a]=7`; segment 1 starts at the same PC/registers but reads `RAM[a]=0` | Each proof checks only its own initialized RAM | Incoming root equals outgoing root; each read authenticates that root |
| Both proofs repeat an arbitrary root while their CPU traces use unrelated memory | Root is metadata only | Root/access equations tied to the actual committed CPU event trace |
| A cell last written three segments ago changes at a later first read | Only “touched in adjacent segments” cells are compared | Full authenticated RAM state, or complete global memory argument |
| Store updates root with unrelated siblings, preserving a claimed written byte | New root is computed without authenticating the old path | Same checked address/siblings for old authentication and new update |
| Same PC and RAM root, changed register, virtual phase, tape offset or syscall state | Boundary schema omits future-relevant state | Complete state and legal cuts; constrain cursor consumption |
| Swap/duplicate two individually valid segments or splice a proof from a different run/program | Conjunction does not enforce a sequence | Exact index/run/program/profile equality and previous endpoint checks |
| A later segment consumes the private tape from position zero again | Per-proof advice commitments are independently valid | Same tape commitment/length plus authenticated position evolution |
| Emit bytes, reset output digest, then claim only final suffix | Only final device output is checked | Proved journal append and length transitions, no reset |
| Public outputs `[3]` versus `[3,0]` | Current normalization removes trailing zeros | New exact-length relation, or explicit zero-padding-equivalence policy |
| Claim billion cycles using padding or wrap a field counter | Padded trace capacity is treated as work count | Active-prefix/retirement counts, integer ranges and checked additions |
| Stop at a valid intermediate state or a panic-free guest returning zero | Proof validity is confused with application acceptance | Constrained halt/exit predicate and final expected guest result |
| Submit zero proofs | Pinned guest initializes conjunction to true | Explicit $n\ge1$ and actual active execution |

## 10. Costs, state dimensions, and security budgets

Let $a_i$ be authenticated RAM accesses, $w_i\le a_i$ writes, $h$ tree depth, $\kappa$ digest bits, $C_H$ the actual constrained cost per fixed-format hash, and $H_{\rm tape/io,i}$ the hashes needed for tape authentication/journal updates. A straightforward uncached per-byte method uses approximately

$$
H_i\le (a_i+w_i)(h+1)+H_{\rm tape/io,i}+H_{\rm boundary,i},
\qquad
C_{\rm mem,i}=O(H_i C_H).
$$

The read path costs $h$ node hashes plus one leaf hash; a write additionally computes the updated leaf/path. Exact hash compression-block counts depend on the specified leaf/node encodings and must be measured from the eventual constraints, not assumed one compression per abstract hash. Word/page leaves and authenticated first-touch batching can reduce constants but require a new precise byte/permission/cache relation; they are not priced as an already implemented optimization.

- Public live boundary size: $O(Rb+\kappa+cb_c+\text{IO/advice identities})$ bits, for $R$ live registers of $b$ bits and $c$ counters/cursors of $b_c$ bits. SHA-256 roots occupy 32 bytes but require full limb representation, not one fp128 element. Hashing the complete boundary makes public size smaller while adding endpoint-preimage constraints; it is not a zero-knowledge layer by itself.
- Prover witness size for the direct relation: CPU rows plus $O(a_i h\kappa)$ path bits and hash-intermediate witnesses, and tape/IO witnesses. A segment can stream witness generation, but generic sumcheck/PCS proving may still retain its full bounded witness. Full RAM storage is at least the live memory footprint unless external authenticated storage is used; “one shard at a time” is not constant memory independently of RAM size.
- Initialization: a dense tree construction is $O(M)$ hashes/storage; sparse/default subtrees can be built from the initialized footprint and cached canonical defaults. Include its cost and proof of private initialized regions once, not silently once per segment or nowhere.
- Receipt composition adds $O(n)$ link/statement checks over the history, or a proved bounded incremental `Step` for each segment plus one terminal receipt verifier. Final external verification need not scan the list only when the history proof actually authenticates those checks.
- Akita costs: hash witness sources enlarge commitment/hint memory, layout widths and fresh groups. Candidate A retains their frontend verifier work; Candidate B must include them in max fresh capacity and receiving-shape/domain calculations. Accumulating only a terminal matrix equation does not remove $C_{\rm mem,i}$.
- Full RAM endpoint-vector profiles must bind a practical finite address capacity and admitted Akita setup dimension. A logically sparse vector over all $2^{64}$ addresses is not a practical endpoint commitment merely because its nonzero entries are sparse: setup generation/storage and supported dimensions must be priced separately. The Merkle option uses address depth without requiring an Akita PCS setup for a $2^{64}$-entry RAM snapshot.

A useful conditional security accounting is

$$
\epsilon_{\rm app}\le\epsilon_{\rm receipt}+\epsilon_{\rm acc}(T_{\max})+
\sum_{i<n}\epsilon_{\rm frontend,i}+
\epsilon_{\rm RAMHash}(Q_H)+\epsilon_{\rm tape/ioHash}(Q_H)+\epsilon_{\rm ids}(Q_H).
$$

Avoid double-counting PCS errors already inside $\epsilon_{\rm acc}$ or frontend errors already inside a receipt theorem: use its exact split. Merkle equality has no extra field-fingerprint error; constraint soundness and hash collisions account for failures. Hash advantages are computational assumptions, not a concrete proved number for SHA-256. A ROM idealization may give classical birthday-scale accounting, but that is not a standard-model SHA theorem or QROM recursion theorem. The endpoint-vector alternative needs additional PIOP degree/field-error terms.

Increasing only the accumulation challenge field $K$ does not improve the original fp128 frontend/PCS field-error terms or the Fiat–Shamir query loss of those original proofs. Score the complete native ledger: original application commitment geometry, all Akita commitment tiers and relation/range/setup reductions, successor compression, receipt proof, and hash assumptions. A changed source geometry must be bound in the new profile before challenges; an already committed original polynomial cannot be retroactively reopened under a different geometry. No numerical security strength is claimed without those original-protocol errors and query budgets.

Depth limits are inherited from the chosen accumulator/receipt proof. The semantic induction applies for any finite valid path; this does not imply a polynomial-time extractor for adversarial proofs at growing depth. Freeze $T_{\max}$, max real/expanded steps, memory events, hash witness rows, tape length and integer counter widths. Candidate B needs the receiver fixed-point and all-valid-domain/extraction theorems after adding these sources.

## 11. Remaining proof obligations and reachable solution

The document supplies a complete **application relation** and concrete **memory relation**, with justified path composition. To obtain an end-to-end cryptographic construction one still needs:

1. A sound/complete segment PIOP implementing legal cuts, first/last-active states, every event/hash constraint, nonzero resumed initialization, advice/IO lengths, and terminal semantics, with an exact opening ledger and its field/degree error.
2. A supported Akita source/range/packing catalog for hash/endpoint witnesses, not an assumption that existing one-hot sources encode all such witnesses.
3. A receipt theorem that authenticates every frontend/link/update/endpoint check and the final decider. A history digest alone is not a proof of this theorem.
4. The separate Candidate A exact-split theorem or Candidate B bounded-shape/domain/original-handle/adaptive-extraction theorems, including the latest-step terminal case.
5. An explicit classical/PQ hash and transcript security profile and, if privacy is intended, a separate hiding/simulation theorem. The currently clear-only Akita path is not made zero-knowledge by private advice or by using boundary digests.

No missing theorem is papered over by a claimed implemented continuation API. The conservative route is actionable research: freeze this relation, construct its new frontend reductions and authenticated receipt, and defer only the obligations the chosen accumulator actually covers. The endpoint-vector route is a concrete performance-oriented derivation with clearly identified further theorem/API work, not an interchangeable proved replacement.

## 12. Evidence and validation record

### Local qmd MCP reads

Navigation used `doc_grep` followed by `doc_toc`, then bounded `doc_read` addresses. Single-line reads unexpectedly returned long suffixes; this tool issue was reported, and subsequent substantive section reads used TOC-provided ranges.

- [Jolt](SOURCE_PROVENANCE.md#jolt), qmd `lattice-papers/jolt.md`: §§2.2 (`line:283-294`), 3.2 (`line:419-428`), 7 (`line:820-831`), B.1 (`line:1151-1156`), B.3 (`line:1193-1248`). Primary identifier [ePrint 2023/1217](https://eprint.iacr.org/2023/1217). These sections justify separation of PIOP/PCS, machine memories, Merkle paths and offline-memory chronology; they do not establish current continuation APIs.
- [CPU executions](SOURCE_PROVENANCE.md#cpuexecinsmallspaces), qmd `lattice-papers/cpuexecinsmallspaces.md`: §1.1.1 (`line:33-38`), §1.1.4 (`line:63-78`), §3.3 (`line:568-647`), Appendix C (`line:1406-1479`); continuation/hash-cost motivation was also read at grep-discovered addresses `line:35`, `line:51`, `line:67` with truncation. Primary identifier [ePrint 2025/611](https://eprint.iacr.org/2025/611). Appendix C's parsed timestamp pseudocode/text conventions are not copied as a deployed current Jolt relation; this design uses authenticated roots instead of relying on their transcription.
- [Nova](SOURCE_PROVENANCE.md#nova), qmd `lattice-papers/nova.md`: grep/discovered `line:175` (Definition 5, including fixed environment depth), `line:365` (public-IO digest/preimage bookkeeping), `line:377` (Construction 3). The single-line MCP responses were capped/truncated; claims here concern the displayed definition and construction prefix, not an assertion that the entire suffix was read. The parsed source supplies the version actually inspected.
- [HyperNova](SOURCE_PROVENANCE.md#hypernova), qmd `lattice-papers/supernova.md`: §5 (`line:515-562`) and §6.1 (`line:569-590`), including control-function path semantics and public-IO sizing issue. No group construction is imported into the proposed lattice system.

### Primary web/source reads

Read the RISC Zero versioned SystemState fields/documentation and version-3.0 recursive proving page cited in §2. Read [Twist and Shout, ePrint 2025/105](https://eprint.iacr.org/2025/105) **abstract**, not its full proof: the detailed prefix algebra context comes from local CPU §3.3. Searched primary Jolt/CPU paper identifiers for citations. Source claims come from the exact GitHub pins and bounded files linked throughout §§2 and 6; pinned verifier paths were discovered read-only with `git ls-tree`, without checkout or branch changes. The Blake and native-batch pins are different deliveries and are not treated as an already validated combined binary.

### What was actually validated

Performed source/document inspection and written algebraic derivation of ordered path concatenation, authenticated leaf-update induction, and nonzero-initial-state prefix identities. No executable algebra experiment, implementation, proof generation, saved-proof replay, benchmark, build, test, linter or formatter was run. Cost expressions are accounting formulas, not measured results. Parent integration should review mathematical/security contracts and shared receipt interfaces; no source-code verification is claimed.
