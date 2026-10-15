# Obligation 6 — Certified history, final acceptance, and receipt boundaries

## 1. Success criterion and scope

**Research proposal, not implemented code.** The deliverable is an explicit accepting predicate for a single continued execution, including every frontend check, accumulator transition, setup obligation, boundary link, and final decider/domain check. A final accumulator equation or an authenticated digest is insufficient by itself. This document specifies a preferred **one-final lattice-Akita wrapper of a certified history** for public verifier residuals (route A), and the interfaces a private native re-entry accumulator (route B) must satisfy. Route A is a conditional alternative, not a replacement for the native target.

The complete conservative baseline is native replay of all original full verifiers plus the execution-link relation. A one-final wrapper can certify that same baseline without a new native accumulator extractor. This baseline does **not** meet the accumulation savings target. Neither the residual wrapper nor the continued-execution guest specified below exists in the inspected mixed source pins; no speedup or proof-size measurement is asserted.

Classification used throughout:

- **Source observation:** a fact read in a paper or pinned source, with the exact scope listed in §12.
- **Derived algebra:** the elementary field-residual argument in §7; not an Akita theorem.
- **Conditional construction:** a complete algorithm whose security requires the explicitly listed component contracts.
- **Open obligation:** a missing instantiation or theorem; never discharged merely by writing the verifier relation.

The externally useful receipt must attest to

\[
\text{one execution of the selected program, from the selected initialization,
through exactly }T\text{ linked nonempty segments, to an intended successful halt.}
\]

It must not attest only to the conjunction of independently valid proof/device pairs. The CPU small-space paper explicitly distinguishes these statements and requires register/memory continuity [CPU, §1.1.1].

## 2. Source-backed acceptance risks

### 2.1 What the Blake outer actually proves

The inspected Jolt revision is [`5c65167e269fdfc2917ecb82f420d349b18b4c87`](https://github.com/a16z/jolt/tree/5c65167e269fdfc2917ecb82f420d349b18b4c87), with Akita fork [`d0ee56a0b12db66b4c46a87c527169737694c4c7`](https://github.com/markosg04/akita/tree/d0ee56a0b12db66b4c46a87c527169737694c4c7).

In [`outer.rs`, lines 81–314][outer], the runner accepts a supplied ELF and framed input, constructs preprocessing from that program/profile/memory configuration, traces it, rejects `trace_output.device.panic`, and proves its execution using the modular q128 Akita backend. It then runs the full outer verifier. Saved replay additionally compares the saved device with the device obtained by tracing the supplied ELF/input. The code does **not** require the guest result to equal one. Thus a verified execution that returns false, without panicking, is not an application-accepting receipt unless its consumer performs a separate result check.

In [`guest/src/lib.rs`, lines 138–207][guest], the guest reads preprocessing and detached setup payloads from its image or input, reads `n`, initializes `all_valid = true`, verifies each independent proof/device pair, and returns `all_valid as u32`. Consequently:

1. A failed inner verification can return zero without a guest panic.
2. `n = 0` returns one by vacuous conjunction.
3. The guest does not check PC/register/memory continuity between the pairs.
4. Setup supplied as input is not automatically the consumer's intended setup. Detached payloads and image-embedded bytecode are part of the setup identity.
5. Its per-record decoder rejects leftover bytes inside a decoded record, but the displayed loop does not enforce global end-of-input consumption after the last record. A new canonical history relation must enforce that explicitly.

These are acceptance-boundary requirements, not evidence of an exploit against a consumer that already checks the right statement. The safe policy here requires an approved guest image/setup, exact IO binding, a successful result, and a nonempty count.

The [public delivery record][delivery] says compilation, profile validation, and saved replay remain gates for that delivery pair, and that historical full-proof/replay timings belong to other pins. This document performs read-only inspection, not those gates. The newer native-batch revision `0791f79a55c3fadc3d327c78a72228696c338468` is a separate integration seam, not an already compatible continued-execution delivery.

### 2.2 The paper's terminal is a special authenticated state

Akita §8.2 binds terminal inner images \(t_j=A_{\rm term}s_j\) in the predecessor before clear checking. The terminal checks Eq. (163), (164), and (165), its tensor reduction, challenges, response bounds, and encoding. Omitting the outer B relation is permitted because that predecessor authenticated the terminal inner state; an ordinary commitment handle cannot simply be relabeled as \((t_j)_j\).

Akita §9.2/Eq. (170) carries a registered setup opening separately from the recursive-witness opening. The receiver checks that setup occurrence; the last nonterminal fold must discharge it and produce none. The terminal admits exactly one recursive-witness group and **no pending setup opening**. Hence “we checked the final A matrix” is not a discharge of an unauthenticated setup evaluation.

## 3. Fixed identities, public statement, and state

### 3.1 Profile admission, not prover-selected security

An approved profile \(P\) freezes:

- protocol and serialization versions; exact Jolt/Akita source revisions; inner and outer verifier program identities;
- inner application program/image, preprocessing identity, ISA/configuration, initial-memory rules, and IO/advice semantics;
- fields/rings and extension basis; exact matrices and views, their ranks/widths, source maps, original handles, compression tiers, registered setup slots, packing/gadget/alphabet rules;
- all admitted schedules and their digests, terminal state kind, fixed lane catalog for A or fixed receiver capacity \(N_*\) for B;
- native transcript epoch, native samplers/filters, allowed nonces/attempt counts, security/query budgets, and completeness bounds;
- \(T_{\max}\), maximum segment size, fresh group count, proof/input byte bounds, active-length limits, memory-address limits, and outer trace/IO capacities;
- receipt mode (`Replay`, `FinalWrapper`, or `RecursiveReceipt`), decider placement (`InsideReceipt` or `ExternalBoundDecider`), accumulator route, nonempty policy, and accepted halt/result encoding;
- history hash, accumulator challenge hash/sampler, SHA-256 RAM authentication convention, and any private-state hiding policy.

\(P_{\rm id}=H_{\rm profile}(\operatorname{Enc}(P))\). Admission means matching a trusted profile or independently validating its entire allowed family, **not** merely accepting a prover's digest. Akita §10.3 binds declarations and schedules before the first challenge and accounts for all matrix instances in the accepted family. Exact setup expansion and all detached prepared payloads must match \(P\); an approved seed does not allow replacing an unrelated matrix view.

The verifier also receives an externally selected run/session identifier \(r\), not one inferred from an untrusted receipt. It binds the desired initial input, expected output/journal predicate, and requested receipt policy. Fresh run IDs prevent unintended cross-run substitution; one-time use/replay prevention is a consumer policy, not a cryptographic conclusion from including an ID.

### 3.2 Final public statement

Use the typed public statement

\[
X=(P_{\rm id},\operatorname{ProgramID},r,\operatorname{ReceiptMode},
\operatorname{EpochPolicy},T,L,B_0,B_T,I,O,J,h_T,
\operatorname{FinalAccID},\operatorname{DeciderPlacement},\operatorname{AcceptPolicy}).
\]

Here \(T\) is the **segment count**, \(L=\sum_{i=1}^{T}\ell_i\) is the total active CPU-cycle/instruction count under the profile's convention, \(I\) is initial public input, and \(O,J\) are the final output/journal or binding commitments with a specified verification relation. A digest is not a replacement for an application predicate on its preimage: the receipt must prove the preimage relation when the predicate needs it. `FinalAccID` hashes the complete final accumulator descriptor/state, including pending obligations and domain tag, or the state is included verbatim in \(X\).

The accepting language is `HistoryAccept(P, X, history, final_certificate)=1` defined below. Public boundary leakage is intentional in the clear proposal; an eventual private variant must change the statement and prove authenticated openings of hidden boundaries rather than pretending they are absent.

### 3.3 Segment and boundary contract

For segment \(i\), the authenticated statement is

\[
S_i=(P_{\rm id},\operatorname{ProgramID},r,i,e_i,
B_{i-1},B_i,\ell_i,\operatorname{counter}_{i-1},\operatorname{counter}_i,
\operatorname{IO}_{i-1},\operatorname{IO}_i,\operatorname{Advice}_{i-1},\operatorname{Advice}_i).
\]

The boundary contains canonical PC, registers, control/status, authenticated RAM root, and IO/advice cursor/transcript state. The intended continuation relation proves every RAM read/write path and root update against a SHA-256 authenticated memory tree, with the canonical address/leaf/internal-node encoding fixed in \(P\). This is an application-hash assumption and additional lattice-backed PIOP work, not a hash PCS or an elliptic-curve commitment. Endpoint/root claims are authenticated by the segment proof, not an attached metadata record. Equality of two arbitrary supplied RAM roots proves nothing without this segment relation.

A frontend interface is therefore required:

\[
\operatorname{Front}(P,S_i,\pi_i,\operatorname{device}_i)
\longrightarrow (\operatorname{ok},\mathcal Q_i).
\]

`Front` derives or verifies **every** field of \(S_i\) against authenticated proof/device data, including endpoint/memory claims, and outputs a typed list of pending PCS obligations. It must not output unconditional “proof verified” when \(\mathcal Q_i\) is pending. This segment interface is a proposed extension; the current independent-conjunction guest does not supply it.

History state is

\[
a_i=(P_{\rm id},\operatorname{ProgramID},r,i,e_i,L_i,h_i,B_i,
\operatorname{AccState}_i,\operatorname{Pending}_i,\operatorname{DomainTag}_i).
\]

`Pending` is an ordered typed ledger, not a bit called “valid.” It names every original opening, registered setup slot, successor handle, or externally retained final certificate required by the selected route. No proof can drop a ledger entry without an authenticated discharge or exact successor transfer.

## 4. Canonical byte and challenge binding

The following is a **proposed new envelope**, not a claim that existing bincode bytes are already this canonical format. Native proof internals must still use their selected pinned parser and transcript semantics.

### 4.1 Encoding

`Enc` is injective on all admitted typed values:

- begin each top-level object with a distinct type tag and fixed protocol version;
- fixed field order; no unordered maps; sorted unique lane/slot identifiers where catalogs require sets;
- integer lengths, indices, counts and epochs are fixed-width unsigned LE-u64, checked for overflow before addition/allocation; booleans are a single byte 0 or 1; enum tags are fixed and unknown tags reject;
- byte strings and vectors have LE-u64 byte/count prefixes and exact bounded payloads; no ignored extension fields, duplicate entries, trailing bytes, or nonzero padding;
- field elements use the fixed-width least nonnegative residue \([0,q)\), rejecting oversized representatives rather than silently reducing; extension fields use the fixed basis and ordered base-field coordinates;
- ring polynomials have exactly the declared coefficient count, in coefficient order; all matrix/view geometry is explicit or resolved through the approved catalog;
- native signed response encoding uses the selected native bounded encoding, with exact canonical re-encoding if the parser admits multiple byte strings for one value. Profile limits must not silently change native soundness bounds.

For a completed native proof, hash its **entire admitted transport byte string**, together with its program/setup/device/statement and transport version. Verify exact parser consumption. Either reject noncanonical transport aliases by decode/re-encode equality, or define byte-string semantics consistently throughout; never hash one representation and verify a different unbound representation. Record-level padding in the current aligned guest transport is not a substitute for these rules.

### 4.2 Domain-separated hashes

Use full 64-byte Blake2b-512 outputs for the new clear history/identity/challenge envelope, with distinct fixed namespaces such as:

- `akita-acc/profile/v1`;
- `akita-acc/history-init/v1` and `akita-acc/history-step/v1`;
- `akita-acc/proof-record/v1`;
- `akita-acc/challenge-A/v1`;
- `akita-acc/final-statement/v1` and `akita-acc/epoch-close/v1`.

Each hash input is `Enc(tag, version, tuple)`; concatenation without framing is prohibited. These proposed namespaces are separate from the original PCS, frontend, setup expansion, outer proof transcript, and SHA-256 RAM-node namespaces. Collision/second-preimage security is an assumption; fixed output length is not a theorem of history correctness.

Initialize

\[
h_0=H_{\rm init}(P_{\rm id},\operatorname{ProgramID},r,B_0,I,
\operatorname{AcceptPolicy},\operatorname{EpochPolicy}).
\]

For route A, before its fresh challenge derive

\[
d_i=H_{\rm record}(S_i,\operatorname{device}_i,\pi_i,
\operatorname{nativeScheduleID}_i,\operatorname{typedLaneData}_i,
\operatorname{updatePrechallengeMessages}_i),
\]

\[
\alpha_i=\operatorname{Sample}_{K^{\times}}
\bigl(H_{\rm acc}(\operatorname{Enc}(a_{i-1}),d_i,i,e_i)\bigr).
\]

The entire old state and incoming data are fixed before \(\alpha_i\). The successor state is **not** part of this prechallenge message, avoiding a circular definition. `Sample` is a specified exact rejection sampler into the nonzero field elements, including stream domain/counter/byte-consumption rules. A profile-defined finite expansion limit causes rejection, not biased modulo reduction. No prover-selected nonce is allowed for this new A challenge. Even without a nonce, changing proof bytes or old state lets an adversary probe many inputs; the total query/attempt budget remains relevant.

After deriving the arithmetic successor `core_i` (all of \(a_i\) except \(h_i\)), set

\[
h_i=H_{\rm step}(h_{i-1},d_i,\alpha_i,\operatorname{Enc}(\operatorname{core}_i),
\operatorname{updateProofBytes}_i).
\]

A B update instead retains its native prechallenge ordering and hashes its full checked transcript into the history digest. No universal `hash the whole B proof before its own challenges` rule is imposed: its prechallenge messages and successor derivation must follow the actual protocol.

### 4.3 Native challenges and attempts must be replayed, not guessed

The pinned Blake sparse stream [XOF] uses epoch-6 domain `akita/sparse-challenge/blake2b512/v1`, an exactly 32-byte root, and a claim-major LE-u64 coordinate index; its bounded-integer rejection consumes bytes from the same stream. This differs from the paper §9.3's SHAKE implementation description. A mixed epoch/pin is rejected.

`V0`/`AccV_B` must replay the exact selected native transcript frame, nonce domain, sampler and filter, **first accepted shell draw within each sampler**, complete coordinate tuple, and response/norm checks. Mere sparse-shell membership is not challenge derivation. The paper terminal allows a selected response nonce within an admitted set and a bounded number of probes; it does not imply that the verifier can determine whether the prover tried other admissible response nonces first. Do not invent a “first globally successful response nonce” requirement unless the selected profile really enforces it. Separate sampler-internal first acceptance from prover grinding over response nonces.

Every allowed response-nonce probe and schedule/proof attempt contributes to the adversary's native FS budget. Accepted transcripts cannot reveal how many failed probes an adversary made; the budget is a security-model bound, not an observable count certified by the receipt. A receipt counts submitted records/declared nonces and enforces admitted ranges; it does not magically certify all hidden oracle work.

## 5. Exact per-step acceptance relation

All algorithms are deterministic once the profile and transcript/hash inputs are fixed. Any exception, malformed encoding, capacity exhaustion, sampler failure, or unrecognized tag is rejection.

### 5.1 Shared `Step`

`Step(P, a_old, record_i, claimed_a_new)` executes:

1. **Decode and identity.** Decode exactly one bounded canonical record. Check old-state IDs and admitted route/profile, \(i=a_{\rm old}.i+1\le T_{\max}\), approved epoch policy, and unchanged program/run identity. Decode proof/device and approved preprocessing/payload identity. No untrusted verifier key is accepted merely because it is well formed.
2. **Frontend and application.** Run `Front` to authenticate \(S_i\), its CPU constraints, IO and memory claims. Require \(\ell_i>0\), bounded size, correct counter increment, no overflow, no segment following an already terminal halt, and the proof's declared geometry/schedule within \(P\). For \(i=1\), require the profile's `Init(B0,I)` predicate, including program-initialized RAM. Require exact equality between the authenticated segment start and `a_old.B`; check IO/advice cursors and transcript continuity.
3. **Retained checks.** Run the exact route-specific retained verifier. Every failed frontend, tensor/sum-check, challenge, commitment-handle, norm/range/encoding, setup, or proof-stream-completion check rejects. Derive \(\mathcal Q_i\) from verified data rather than trusting a prover list.
4. **Accumulator verification.** Run `AccV` as below. Authenticate all pending-ledger transfers/discharges. Derive the exact successor PCS/residual state and descriptor. Missing mandatory lane/group/slot is rejection, not an implicit zero source.
5. **Successor derivation.** Set count \(i\), total \(L_i=L_{i-1}+\ell_i\), authenticated boundary \(B_i\), epoch, pending ledger and domain tag. Recompute the digest by §4. Compare **every byte** of the derived \(a_i\) to `claimed_a_new`; only the derived state is used by the next step.
6. **Return.** Return the derived successor or reject. There is no “accept bit” standing in for a pending final check.

The formal relation is

\[
R_{\rm step}(P,a_{i-1},\operatorname{record}_i,a_i)=1
\iff \operatorname{Step}(P,a_{i-1},\operatorname{record}_i,a_i)=a_i.
\]

The receiver's bounded-domain assumptions are explicit in B's relation. Merely checking a well-formed old commitment does not assert that it has a short, canonical, or in-range source.

### 5.2 Route A: conservative terminal-equation deferral

Fix a finite catalog of exact matrix/view lanes. For the narrow proposal defer **only** the Akita terminal Eq. (163) from each completed proof:

\[
A_{\ell}x_{\ell,i}=y_{\ell,i},\qquad
x_{\ell,i}=z_i,\quad y_{\ell,i}=\sum_j c_{i,j}t_{i,j}
\]

for its matching terminal lane. Multiple occurrences are combined only with an explicitly sound typed combination or processed as separate subupdates; they must not be silently added with coefficient one. The simplest profile permits exactly one occurrence per active lane per record. Lanes absent by the authenticated schedule contribute zero; a required lane missing from the record rejects.

The exact split requirement is

\[
V_{\rm full}(P,S_i,\pi_i,\operatorname{device}_i)
=V_0(P,S_i,\pi_i,\operatorname{device}_i)
\land \bigwedge_{\ell\in\mathcal L_{\mathrm{required}}}[A_\ell x_{\ell,i}=y_{\ell,i}].
\]

`V0` preserves Eq. (164)/(165), predecessor authentication of \((t_j)_j\), all preceding folds and tensor reduction, compression relations, registered setup checks, native challenges/nonces, every original response bound, and exact proof completion. This mathematical split is justified for the paper terminal; its exact source-level realization for a chosen pinned adapter remains an integration obligation. Broader savings cannot be claimed by deleting unnamed equations.

Work in the public \(K\)-module obtained by coefficient-wise lifting the original matrices, with \(K=\mathbb F_{q^\nu}\). `AccV_A` derives \(\alpha_i\in K^\times\), recomputes

\[
U_{\ell,i}=U_{\ell,i-1}+\alpha_i x_{\ell,i},\qquad
Y_{\ell,i}=Y_{\ell,i-1}+\alpha_i y_{\ell,i},
\]

and verifies exact equality of the successor. Start all lanes at zero. This is public arithmetic, not a bounded Ajtai opening or a new commitment homomorphism; no SIS claim is made about the norm of \(U\).

### 5.3 Route B: private native successor

`AccV_B` must accept only the exact reduction prefix that derives a typed

\[
\operatorname{PendingOrdinary}(P_{\rm id},C',N_*,\mathbf r',v',
\operatorname{DomainTag},\operatorname{boundary/history})
\]

and, only if permitted, an exact `PendingSetup(slot, sourceProfile, point, value)` occurrence. It authenticates original handles and fresh groups, Eq. (109)'s **entire** successor vector, range and relation reductions, source/receiver maps and comparison classes, and all native challenge/norm constraints. The verifier derives \((C',\mathbf r',v')\); it does not accept these as free metadata.

The preferred B receipt profile uses direct setup processing, so its final ordinary pending claim has no setup companion. An offloaded variant must implement the ordered consume-before-produce ledger from Akita §9.2 and a final flush that produces no new setup claim. B's bounded re-entry domain, shape closure, all-valid-state completeness, source consistency and polynomial-time adaptive extraction are **open native obligations**. A history wrapper can prove that a specified update verifier ran; it cannot make an unsound native update verifier sound.

**Independent certified-output alternative (not an epoch-final shortcut).** A strengthened B step may require an independently authenticated certificate \(\beta_i\) for **every** derived receiver, binding its exact original A/B/F handle, full-grid bounded domain and \(\operatorname{Eval}(C_i,\mathbf r_i)=v_i\). Extend `Step` to check that certificate and include its bytes/statement in successor and history binding. A clear \(\beta_i\) reveals the complete receiver vector and decommitment and pays complete receiver checking work on every update, not only at the final decider. A compact private \(\beta_i\) instead requires its own independent adaptive knowledge extractor. Crucially, each counterfactual fork child used by extraction must produce its **own accepted** \(\beta_i\); certifying only the actual execution path, final state, or an epoch checkpoint does not grant access to descendant witnesses on those children. This alternative addresses local suffix traversal only. It does not discharge fresh original-source bounded matching, receiver closure, frontend/application semantics, or certified history, and it is not asserted as an instantiated native long-chain theorem.

## 6. Deterministic history and final acceptance

### 6.1 Final decider/domain predicate

Define `Final(P, X, aT, certificate)`:

1. Match the full final state with `FinalAccID` and all fields of \(X\). Require \(1\le T\le T_{\max}\), exact count \(a_T.i=T\), \(a_T.L=L\), intended initial input/boundary, final output/journal relation, approved epoch chain, and \(a_T.h=h_T\). Require `HaltSuccess(BT,O,J,AcceptPolicy)` and no panic/failure status. No prover metadata can replace the authenticated boundary.
2. Require `Pending` empty **after** the declared final discharges. An ordinary final opening is explicitly consumed here; all registered setup occurrences are already proved in a receiver or directly evaluated against approved setup. Reject any dangling original opening, endpoint/memory claim, domain certificate or registry slot.
3. For A, check every catalog lane's shape/encoding and \(A_\ell U_{\ell,T}=Y_{\ell,T}\). Check all descriptors match \(P\). No private folded-vector range predicate is invented for this public module.
4. For B, verify the complete final opening relation **and the required bounded-domain certificate** under the exact receiver profile. If the chosen state is `TerminalInnerState`, first verify its predecessor transition and then all native terminal checks; never omit B for an ordinary handle. Natural-domain padding/canonical-decomposition requirements are included if the selected re-entry relation needs them.
5. Accept only if all branches above succeed.

A conservative B domain certificate may reveal the fixed-capacity decoded vector and its algebraic opening state. For the concrete identity-source repair, `DomainTag = FullGridIdentityBounded-v1` means a physical coefficient grid of length \(N_*=2^\mu\), a bijective coefficient `Pack`, \(\delta_{\rm com}=1\), every \(w_j\) in the profile's certified alphabet \(\mathcal A_{b^\star}\), and zero reserved alignment/padding cells. The deterministic checker recomputes the full original receiver A/B/F compression/source equations, including supplied inner images and F chains, checks decoded source/value consistency and every additional integer norm required by the chosen \(R_{\rm acc}\), and verifies the linked final evaluation. Fresh original segment sources require their own same-original-handle bounded opening predicate; the identity choice does not make their gadget decompositions canonical. The certificate size/cost is \(O(N_*)\) coefficient work plus actual setup/chain work and opening-hint bytes. Alternatively a separate sound proof certifies the same predicate. “An ordinary PCS evaluation verified” does not imply this additional domain predicate. Such a certificate fixes final membership only; it does not solve closure or adaptive extraction across B history.

### 6.2 Native replay

`AcceptReplay(P, X, records, final_certificate)`:

1. Check profile admission, canonical \(X\), capacities, and exact number of records \(T\). Construct the unique initial state with zero A lanes (or the specified B base accumulator), empty pending ledger and \(h_0\). Validate the B base domain if applicable.
2. For \(i=1,\ldots,T\), execute `Step` and use only its derived successor. Reject any failure.
3. Require total stream consumption: no extra record, hidden suffix, or ignored padding.
4. Run `Final` on the derived final state/certificate. Accept iff it returns one.

No recursive history proof is used. For the fully conservative `FullReplay` profile, Step 3 runs the **full original verifier** on every record, with no deferral. Its accumulator state can be just the authenticated count/boundary/digest; `Final` retains application/count/ledger checks. This gives deterministic end-to-end acceptance conditional on the original segment relation and proof system, without a new accumulator theorem.

### 6.3 One final lattice-Akita history wrapper — preferred receipt architecture

The approved **new history guest** executes `AcceptReplay` on all records and either (a) `Final` itself, or (b) emits the full bound final accumulator for one external decider. It returns a canonical public result

\[
\operatorname{ReceiptResult}=(\operatorname{version},\operatorname{accept}=1,
H_{\rm stmt}(\operatorname{Enc}(X)),\operatorname{FinalAccID}).
\]

Its program/preprocessing proves execution of this relation, not the old independent conjunction. The wrapper uses one ordinary finite-schedule lattice Akita proof of that guest trace. No accumulator for the wrapper's own history is needed in this nonrecursive architecture; its complete native verifier is checked once at the end.

`AcceptWrapper(P, X, outer_proof, outer_device, optional_aT, certificate)`:

1. Check profile admission and intended guest program/preprocessing/ISA/memory-layout/schedule/trace capacities, including exact embedded setup and prepared payload identity. Publicly supplied replacement preprocessing is rejected.
2. Bind the guest's exact public input to \(X\) and its declared history transport. In the current all-public-input style, the consumer checks the actual input bytes (or an authenticated input commitment/preimage relation whose verification is inside the new guest). A log path or runner JSON is not an authenticated public input.
3. Run the **full ordinary outer verifier** with the approved verifier preprocessing. Reject on any error.
4. Independently require `outer_device.panic = false` and exact output decoding as `ReceiptResult(accept=1, H_stmt(Enc(X)), FinalAccID)`, with no trailing output. Merely proving an execution that emits zero is rejection. Require \(T\ge1\) regardless of the old guest's vacuous conjunction.
5. If `ExternalBoundDecider` is selected, match `optional_aT` to the bound final state and execute `Final`/the agreed externally placed parts, including D and B domain certification. If `InsideReceipt` is selected, reject inconsistent external substitutions; D is proved by the guest and not silently skipped.
6. Accept only on success of all applicable checks.

This removes repeated recursive full-inner-verifier aggregation **at the architecture level**; it does not remove verification work from the final wrapper prover, nor does it certify history with a digest alone. A final wrapper of `FullReplay` is the conservative short-receipt option that needs no new native extractor. Its theoretical security still needs the existing outer zkVM's applicable knowledge/extraction guarantee or a joint composed soundness theorem; merely saying “Akita-backed” is not that theorem.

### 6.4 Cheap recursive receipts

For online receipts, a recursive statement must prove `R_step`, predecessor receipt authentication, all frontend/boundary/ledger checks, and the exact accumulator update of both:

- the current segment's deferred PCS obligations; and
- the previous **receipt proof's** deferred obligations.

The base case is explicitly distinguished from recursion and binds initialization and the empty accumulator. The last recursive receipt's full remaining verifier/PCS obligations and all final accumulators are decided once; omitting its own unresolved opening would leave the last link unauthenticated. The public statement includes the count, run/program/profile, boundaries and final digest, not just the folded PCS handle.


The deterministic final algorithm `AcceptRecursive(P, X, receipt_payload)` is: (1) decode the bounded payload exactly and match its profile/program/run/count/statement digest and all final accumulator descriptors to \(X\); (2) run the **full** selected verifier on the last receipt proof, with the approved recursive-step program/key and an authenticated public result equal to one; (3) validate the final certified-history state and execute `Final` for the segment accumulator; (4) execute the full D/domain/discharge predicates for every separate receipt-proof accumulator carried by the compiler, rejecting all unresolved setup or original-opening ledger entries; (5) require the nonempty/initialization/terminal application policy and exact payload consumption. Accept iff every check succeeds. The recursive step program itself distinguishes \(i=1\) (unique initialization and base state) from \(i>1\), binds the predecessor count \(i-1\), proves the approved receipt-prefix checks and derives every successor accumulator; the compiler theorem must authenticate that recurrence. A verifier that checks only the last segment accumulator omits steps (2) and (4) and is unsound.
BCMS's predicate-efficient compiler retains the fast verifier before accumulating its queries. Its recursive PCD compiler assumes the requisite argument/accumulation security model; BCLMS's split compiler adds adaptive extraction requirements. Neither paper instantiates this recursive lattice/Akita receipt. The route is **conditional on a new sound recursive compiler and its admitted recursion threshold/fixed shape**. It removes repeated expensive PCS checks, not recursion itself.

### 6.5 Epochs and checkpoints

The simplest admitted policy is one frozen native transcript epoch for the whole run. If bounded \(T_{\max}\) or norm budgets require checkpoints, a refresh transition must authenticate the previous complete statement, count, boundaries, digest, pending ledger and its decider/domain result. It either discharges every old obligation or transfers each named obligation through a proved map into the next profile. A new zero accumulator seeded only with `h_old` is unsound.

A checkpoint wrapper can verify a prior checkpoint proof plus the current epoch records, but then cross-checkpoint recursion or final replay of checkpoint proofs remains. Every epoch's reset has a cost and security error; it is not an unbounded-depth theorem. Switching native challenge epochs/pins without an approved authenticated migration is rejection.

Neither an authenticated checkpoint nor an epoch-final domain certificate substitutes for the every-update, every-counterfactual-child receiver certificates of the independent certified-output alternative in §5.3. If that stronger alternative is selected, its full per-update certificate costs and separate extractor premises apply in every epoch.

## 7. Algebraic and conditional soundness statements

### 7.1 The public residual lemma

Let \(R_{\ell,i}=A_\ell U_{\ell,i}-Y_{\ell,i}\) and \(r_{\ell,i}=A_\ell x_{\ell,i}-y_{\ell,i}\). Then

\[
R_{\ell,i}=R_{\ell,i-1}+\alpha_i r_{\ell,i}.
\]

If the old/current residual data are fixed before a uniform \(\alpha_i\in K^\times\), and either is nonzero, at most one scalar makes the entire successor residual tuple zero. Indeed, two distinct successful scalars imply \((\alpha-\alpha')r=0\); multiplication by a nonzero field scalar is invertible on the coefficient \(K\)-module, so \(r=0\), and then the old residual is zero. This remains valid when the polynomial quotient ring has zero divisors: the scalar is in \(K^\times\).

Thus one invalid-to-valid transition has probability at most \(1/(|K|-1)\). A union bound for \(T\) genuinely independent interactive challenges gives \(T/(|K|-1)\). Valid old/current data preserve zero residuals for **every** allowed challenge, giving perfect arithmetic completeness.


The native security ledger is separate from this A challenge. For every admitted inner/outer Akita schedule, retain

\[
\varepsilon_{\rm native}\le (Q_{\rm native}+1)\varepsilon_{\rm int}
+\Delta_{\rm setup}+\sum_{I\in\mathcal I_P}\operatorname{Adv}_{{\rm MSIS},I},
\]

with the exact coordinate/tensor/group/terminal/range/relation/setup terms in \(\varepsilon_{\rm int}\), every accepted matrix/view in \(\mathcal I_P\), and all nonce probes included in \(Q_{\rm native}\). Akita Corollary 10.34, Eq. (230)–(231), states

\[
\lambda_{\rm FS}^{\rm eff}=-\log_2((Q_{\max}+1)\varepsilon_{\rm int}),
\qquad
\varepsilon_{\rm int}\ge c/|E|
\Longrightarrow
\lambda_{\rm FS}^{\rm eff}\le\log_2|E|-\log_2 c-\log_2(Q_{\max}+1).
\]

Increasing only the public accumulation field \(K\) improves the residual-cancellation term; it does **not** improve the original fp128 opening/frontend query ceiling, setup error, matrix security or outer proof. Akita §12.2 further fixes original root commitment geometry: opening it cannot change its ring/rank/block split/compression map. All successor compression digits and setup obligations must be included **before** comparing candidate costs/security. These constraints are part of profile admission, not an optimization to omit later.
For noninteractive hash challenges, \(T/(|K|-1)\) is not a sufficient bound when the adversary grinds candidate records/states. A conditional classical lazy-oracle argument would charge every complete prechallenge query/attempt, yielding at most \(Q_A/(|K|-1)\), plus digest/sampler errors. The joint proof must handle shared original-proof oracle access, repeated queries, abort/retry, challenge reconstruction by a wrapper, and receipt extraction. This document does **not** assert that this joint ROM theorem has been established. There is no QROM bound here.

### 7.2 End-to-end conditional soundness — route A

Fix an admitted profile, polynomially bounded count/query budgets, and the algorithms above. Assume:

1. The segment proof frontend and its opening reduction establish the full stated CPU/memory/IO relation, with joint adaptive soundness error \(\varepsilon_{\rm seg}\) for the admitted family. Every required endpoint claim is included.
2. The source-level split is **exact**, including all original norm/challenge/setup/terminal/proof-completion checks.
3. The route-A challenge sampler and joint classical FS argument satisfy an atomic backward-soundness bound \(\varepsilon_A\). Interactively use \(T/(|K|-1)\); noninteractively \(Q_A/(|K|-1)\) is a candidate component only after the joint argument is proved.
4. Serialization is injective on admitted values; program/setup/profile/history/IO hashes are binding against the adversary, with total error \(\varepsilon_{\rm bind}\); the RAM-root mechanism has the required authenticated-transition binding, included here or in \(\varepsilon_{\rm seg}\).
5. History authentication is exact native replay, or the selected wrapper has an efficient knowledge extractor for the history witness (or an adequate joint composed theorem), or the recursive compiler supplies its claimed certified-history contract, with error \(\varepsilon_{\rm hist}\). A merely sound proof of the existence of accepted *computational* inner proof bytes is not automatically enough: extraction must expose efficiently generated bytes for the underlying computational reduction, unless a joint proof avoids that issue.
6. Final D and application acceptance are actually executed/proved and tied to \(X\).

Then, except with error at most

\[
\varepsilon_{\rm hist}+\varepsilon_{\rm bind}+\varepsilon_A+
\varepsilon_{\rm seg},
\]

acceptance implies a valid execution of the selected program from \(B_0,I\), through exactly \(T\) ordered nonempty segments with total length \(L\), continuous authenticated machine/memory/IO/advice state, to the selected successful halt and output/journal.

**Argument.** Extract or replay all accepting `Step` records. The hash and exact successor checks fix their identities, order, counts and common state. Final zero residual and atomic backward soundness imply all original deferred equations. Together with exact `V0`, every full segment verifier accepts. The segment relation supplies genuine authenticated transitions; exact links and initialization compose them by induction. Final acceptance supplies the intended terminal predicate. This proof does not extract a private folded Akita opening or materialize a whole-history Akita fork tree.

For `FullReplay`, omit \(\varepsilon_A\) and the split assumption. Native replay has no cryptographic history-proof error (\(\varepsilon_{\rm hist}=0\), abstracting correct deterministic implementation). A final full-replay wrapper adds only its applicable extraction/composition assumption, not the new native re-entry theorem. Original segment/outer source guarantees still must match the selected code and profile; paper theorem numbering alone does not certify an adapter implementation.

### 7.3 Route B is conditional on the missing native theorem

Substitute a sound split/native accumulation contract for assumption 3: final bounded-domain validity plus accepted `AccV_B` must yield valid old bounded accumulator and original incoming opening witnesses, with polynomial expected-time adaptive extraction in \((\lambda,T,N_{\rm segment},N_*)\). Keep all other assumptions unchanged. The same history/application induction then applies, with a native error \(\varepsilon_B\).

This does not follow by repeating Akita's finite-schedule theorem. Akita's extraction tree is a product of stage branching factors; repeated nontrivial folds can make it exponential in \(T\). Akita Remark 10.32 also explicitly does not promise a canonical short source decomposition. The wrapper authenticates update checks but cannot fix either gap.

### 7.4 Strong completeness versus generated-history completeness

BCMS §2.2 and Halo Definition 6.1 quantify over **all decider-valid old accumulators**, not just honest history states. BCLMS's split definition includes valid witnesses to that same broad domain. For A, zero-residual arithmetic updates are complete on that algebraic domain, subject to the original proof frontend's own completeness and sampler limits.

For B, a history receipt can restrict the language to generated valid states. That yields a useful explicitly weaker contract, but is not all-valid-state strong completeness. To claim strong completeness, final D must certify the exact re-entry domain and the receiving profile must admit every state in it. Even a revealed domain certificate does not prove that Eq. (109)'s successor fits \(N_*\) for the full accepted domain. Completeness errors include all native tensor-factor rejection, bounded filter exhaustion, response-grinding exhaustion, segment proof failure, wrapper failure and any epoch-refresh abort. Profile capacity exhaustion is rejection, not an ignored record.

## 8. Costs, sizes, and what recursion removal actually saves

These are explicit accounting formulas, not timings. Let

- \(b_i\): segment proof/device/statement/transport bytes;
- \(f_i\): retained frontend and `V0` work; \(m_i\): deferred terminal matrix work;
- \(u_i\): accumulator verification/update work; \(g_i\): boundary/memory checking work; \(h_i^{\rm cost}\): canonical parsing/hash/challenge work;
- \(d\): final decider plus final domain/discharge work;
- \(S_A=\nu\sum_\ell (m_\ell+n_\ell)d_\ell\): public A state in base-field coordinates, excluding fixed-size count/digest/boundary fields;
- \(c_i\): native B/update proof bytes; \(W_B\): fixed-capacity B opening witness bytes; \(w_D\): final domain certificate bytes;
- \(C\): history guest active trace length; \(\bar C\): admitted padded geometry; \(P_{\rm out}(\bar C)\), \(V_{\rm out}(\bar C)\), \(B_{\rm out}(\bar C)\), \(W_{\rm out}(\bar C)\): ordinary outer prover work, verifier work, proof bytes and witness/storage for its exact admitted schedule.

| Architecture | Prover/receipt work beyond segment proving | Final verifier work | Proof/input/witness footprint |
|---|---|---|---|
| Full native replay | Archive/transport all records; no cryptographic history prover | \(\sum_i(f_i+m_i+g_i+h_i^{\rm cost})\) plus final endpoint checks | Communicates \(\sum_i b_i\); can process records sequentially; original segment proving costs remain |
| A native replay | \(\sum_i(u_i+h_i^{\rm cost})\), if frontend replay shared with prover, plus archive | \(\sum_i(f_i+u_i+g_i+h_i^{\rm cost})+d\) | \(\sum_i b_i\) plus final \(S_A\); no short lattice-opening witness for A; intermediate states may be recomputed, not sent |
| B native replay | Native accumulation proving for each fresh group plus live witness maintenance | \(\sum_i(f_i+u_i+g_i+h_i^{\rm cost})+d\), conditional on B | Records/update proofs \(\sum_i(b_i+c_i)\) under this transport, final public instance plus \(W_B/w_D\) if revealed; short instance does not mean short witness |
| One final lattice wrapper | Trace history checks, then \(P_{\rm out}(\bar C)\); FullReplay uses full-verifier trace, A uses retained checks/update plus one D | \(V_{\rm out}(\bar C)+O(|X|)\), plus external \(d\) if selected; public input costs charged separately | Proof \(B_{\rm out}(\bar C)\); witness includes history-check trace and records; all-public-input style exposes/transmits \(\sum_i b_i\), not a constant-size total receipt |
| Cheap recursive receipt | Per-step receipt proving for fixed local trace/circuit \(c_*\) including `Front`, links, `AccV`, hashes and preceding receipt checks | Last receipt verifier plus every remaining D/domain/ledger obligation | Conditional receipt size \(B_{\rm rec}(c_*)+\) final accumulator/certificate; private live witnesses may be \(\Theta(N_*)\); recursion remains |

The final-wrapper guest must at least read \(\sum_i b_i\) bytes in the completed-proof route. Its active trace generally grows with \(T\), hence so do \(\bar C\), proving work, setup/shape requirements and possibly its proof/verifier cost. A fixed profile can pad to \(T_{\max}\), but then pays that capacity. Do not call this constant-in-\(T\) receipt proving. A finite ordinary Akita schedule has a paper asymptotic PCS bound dependent on its size and chosen schedule; the pinned wrapper's end-to-end numbers are not deducible from that PCS asymptotic alone.

The pinned outer streams emulator cycles into reserved modular **rows**, then stores/pads those rows and constructs witness/PCS buffers. Its source explicitly reports row storage as **not peak RSS**, excluding emulator/preprocessing/witness/sum-check/PCS overlap. Thus streaming tracing does not prove an \(O(N_{\rm segment})\)-memory final wrapper. The CPU paper's nonrecursive small-space prover is a different construction, not automatically an implementation of this history guest prover.

The pinned runner defaults `max_input_size` to 16,000,000 bytes and `max_output_size` to 4096; replay bounds each saved artifact at 32 MiB. These are source transport/capacity limits, **not measured proof sizes or universal protocol limits**. Profile selection must explicitly cover the history input and final state. Large public A state may require internal D and only a bound final digest in output; that is safe only because the approved guest proves D on the exact derived state.

For narrow A the savings term is at most \(\sum_i m_i-d\), before additional update/hash/receipt costs. All preceding Akita folds and frontend work remain in \(f_i\). There may be no net saving. Extension-field A challenges cost \(\Theta(S_A)\) state operations; they are not deployed fp128 adapter support merely because the public algebra is defined. With a common ring lane, scalar update cost is proportional to its coordinate count; dense matrix multiplication in D is priced once per lane. Setup variants/more lanes can make D larger than one original terminal scan.

A short proof only becomes a short **total receipt** when history bytes are private witness or separately available and the public input is \(O(|X|)\). The current all-public framed input does not achieve that communication claim. Private history advice/input commitments are a changed guest/IO relation and need their own binding and privacy arguments.

## 9. Privacy and security-model boundaries

### 9.1 Clear route leakage

A exposes \((U,Y)\), final boundary and count, program/run identifiers and digests. Replay exposes complete proofs/devices/records. Original clear Akita responses, opening values, IO, per-segment geometry, proof lengths, aborts/attempt metadata and setup choices may leak witness-dependent information. A single nonzero-scaled contribution can preserve essentially the information of its clear source; folding it is not a hiding argument.

A deterministic hash chain does not establish hiding. It can reveal equality/linkability, count/profile information, and permit dictionary tests on low-entropy records. Collision resistance addresses binding, not simulation or confidentiality. A declared private output digest also permits dictionary tests unless its secrecy specification and randomization address them.

### 9.2 Why accumulator-only ZK is insufficient

Even a perfectly hiding final accumulator cannot erase already published frontend proofs, boundaries, public inputs, update transcripts or witnesses revealed in final domain certification. BCMS §2.2's accumulator ZK notion deliberately separates a verification proof from the state that must be simulatable; the PCD theorem additionally requires the underlying argument's ZK. Nova §5.2 similarly compresses the whole IVC proof-validity relation with a ZK argument and explicitly qualifies its randomized-witness premise. Neither result says that hiding one handle makes an otherwise clear history private.

An end-to-end private receipt must choose its permitted leakage \(X\), keep all other records and B witnesses private, authenticate committed boundaries/IO through proofs, hide every accumulator and update object that the consumer sees, avoid revealing \(W_B\) in D, and prove a simulator for the whole receipt/history distribution. If an approved ZK outer proves the **entire** history relation with only \(X\) public, it can hide clear inner proof bytes from the final consumer who never received them; it cannot undo disclosure to other parties. The inspected lattice Akita outer/adapter is clear-only, and Akita §14 lists ZK as future work. No such ZK wrapper is instantiated here.

### 9.3 Post-quantum is not implied by lattice matrices

Akita's §10.2 FS theorem is classical ROM; its quantum lattice-attack parameter model is not a QROM knowledge theorem. The A cancellation argument is classical and supplies no quantum oracle-search/rewinding reduction. A recursive receipt additionally needs a quantum-safe history compiler and all component proof/hash/commitment assumptions in the claimed model.

BCMS/BCLMS preserve PQ and ZK only **if their premises hold**, and their non-black-box PCD compilers are stated in a no-oracle CRS model. Halo §6 explains the heuristic ROM-instantiation gap; BCLMS Remark 1 warns that PCS aggregation alone does not resolve it. A one-final nonrecursive wrapper avoids a long recursive extraction chain, but it is not thereby a QROM proof. BN254/Pedersen/DLOG components would also rule out the intended PQ claim; none is introduced as a hidden substitute here.

## 10. Concrete rejection cases

| Proposed attack/error | Required rejection or limitation |
|---|---|
| Publish a well-formed `hT` with fabricated records | Digest alone never accepts; replay/wrapper proves every `Step` |
| Set all final A lanes to zero after skipping updates | Exact derived successor/history relation fails; matrix checks alone would wrongly accept |
| Splice independently valid proofs from different runs/programs | Fixed run/program/profile and authenticated exact segment indices/boundaries fail |
| Reorder/drop/duplicate a segment | Count/index/linked state/digest/total-length and exact stream checks fail |
| Valid proofs with mismatched RAM or advice cursors | Authenticated boundary equality and access/cursor relation fail |
| A failed inner proof returns zero without panic | Wrapper full verification may pass execution, but result check rejects |
| `n=0` produces `all_valid=1` | Explicit nonempty `T≥1` policy rejects |
| Supply a verifier for a different guest/setup | Approved image/preprocessing/payload identity fails |
| Use sparse challenge in the shell but not from the selected Blake transcript | Exact native sampler derivation fails |
| Search many nonces and charge only accepted segments | Declared security claim must use total query/attempt bound; cannot assert the smaller error |
| Drop a setup-prefix opening just before terminal | Nonempty pending ledger or wrong terminal input rejects |
| Rename ordinary handle as terminal inner images | Missing authenticated predecessor `TerminalInnerState` transition rejects |
| Ordinary final Eval succeeds on an out-of-domain B source | B domain predicate/certificate rejects; Eval alone is intentionally insufficient |
| Reset/refresh to zero state with only the old history hash | Missing prior D/history/ledger proof or exact transfer rejects |
| Hide final handle but publish all clear records | Not a rejection of soundness; privacy claim is false under that leakage model |

## 11. Remaining obligations and integration contract

The history design itself is not a license to defer unspecified checks. The following ownership-independent gates account for all dependencies:

1. **Execution/frontend gate:** instantiate the authenticated segment relation, including PC/register/control/RAM/IO/advice endpoints, and prove its opening reduction. Current independent conjunction is inadequate.
2. **Source/profile gate:** freeze one delivery pair, exact transcript epoch and admitted profile family. Mixed-pin porting is separate engineering, with no historical performance reuse.
3. **A split gate:** map the paper Eq. (163) deferral to the selected code and show `Vfull = V0 ∧ typed equations`, preserving every named retained check. Exact proof consumption and lane geometry are essential.
4. **A FS/composition gate:** prove the joint classical query/attempt/digest bound and wrapper history-extraction contract. The elementary interactive lemma is already sufficient algebra, not that theorem.
5. **B relation gate:** specify whether the accumulator relation requires short source, decoded range, canonical decomposition, weak opening, or a combination. `DomainTag` must resolve to that full predicate.
6. **B closure/completeness gate:** certify exact Eq. (109) successor width, all auxiliary/setup/range/norm data, profile fixed point and admitted full old-state domain. Generated-history certification is a separate weaker option.
7. **B consistency/extraction gate:** authenticate original handles and receiver projections across profiles and prove expected-polynomial adaptive extraction without an exponential history fork tree. A final wrapper does not supply it.
8. **Final discharge gate:** verify all registered setup obligations and native terminal requirements, or direct-check them against approved setup; certify final domain in addition to Eval where required.
9. **Receipt gate:** implement the approved history guest with exact output/IO binding; supply finite outer soundness/knowledge guarantees. Cheap recursion separately needs a sound compiler/fixed point and final receipt-proof obligation discharge.
10. **Privacy/PQ gate:** either label delivery clear/classical, or supply the complete hiding/simulation and QROM/composition arguments. No measured speedup follows from any of these gates.

Exact interfaces for integration are:

- `Front(P, authenticated SegmentStmt, proof, device) -> (ok, typed PendingClaims)`;
- `Step(P, oldState, canonical Record, claimedNewState) -> derivedNewState | Reject`;
- `AccV_A(P, full oldState, derived typed residuals, fixed prechallenge bytes) -> derived public lanes | Reject`;
- `AccV_B(P, typed old opening instance/domain, fresh original claims, native update proof) -> derived PendingOrdinary[/PendingSetup] | Reject`;
- `Final(P, X, derived finalState, final opening/domain certificate) -> 0/1`;
- `ReceiptVerify(P, X, receipt) -> 0/1`, requiring history authentication **and** final D in the declared location.

These interfaces intentionally never return a verified continued execution before the final obligations and accepting result are discharged.

## 12. References and validation record

### Local papers read using qmd MCP navigation

All addresses below came from `doc_toc` or `doc_grep` before `doc_read`. They identify the indexed local snapshots, not an invented publication revision/date.

- **[Akita]** Dao et al., *Akita: A High-Performance Lattice-Based Polynomial Commitment Scheme*, [local parsed paper](SOURCE_PROVENANCE.md#akita), qmd `lattice-papers/akita.md`: title/abstract `line:1-20`; §8.2 `line:3026-3097`; §9.2 `line:3148-3193`; §9.3 `line:3194-3245`; “From One Fold to Fiat–Shamir Knowledge Soundness,” Theorems 10.31/10.33, Remark 10.32 and Corollary 10.34, `line:4141-4250`; §10.3 `line:4251-4332`; §12.2 `line:4641-4662`; §14 `line:4807-4816`. The broad §10 request `line:3363-3452` included an embedded-image/truncated payload and is not relied on for unread text. The separately read theorem subsection supplies the security claims cited here.
- **[CPU]** Nair, Thaler, Zhu, *Proving CPU Executions in Small Space*, [local parsed paper](SOURCE_PROVENANCE.md#cpuexecinsmallspaces), qmd `lattice-papers/cpuexecinsmallspaces.md`: metadata/abstract `line:1-4`, `line:5-8`; §§1.1–1.1.3 `line:21-32`, `line:33-38`, `line:39-44`, `line:45-62`. Read for sharding/continuity/recursion-cost distinctions; not used as a theorem about this proposed wrapper's memory.
- **[Nova]** Kothapalli, Setty, Tzialla, *Nova: Recursive Zero-Knowledge Arguments from Folding Schemes*, [local parsed paper](SOURCE_PROVENANCE.md#nova), qmd `lattice-papers/nova.md`: §5.1 `line:357-454` (returned text opening explains augmented function and randomized IO hashing; later embedded-image content truncated); §5.2 `line:455-522` (read complete returned text), Definition 13, Construction 4, Theorem 4. Used as an analogy for history/whole-proof validity and qualified privacy, **not** an instantiated lattice recipe.

### Primary publications actually inspected

- **[BCMS]** Bünz, Chiesa, Mishra, Spooner, *Recursive Proof Composition from Accumulation Schemes*, TCC 2020, [official publication PDF][bcms], DOI `10.1007/978-3-030-64378-2_1`. Read §§1.1–1.2, §§2.1–2.3 and the opening of §2.4. Definitions 1–2 are explicitly informal in this publication. Supports retention of frontend/predicate-efficient checks, all-decider-valid completeness, separate ZK premises and no-oracle PCD compiler boundary. No uninspected full-version formal game is asserted.
- **[BCLMS]** Bünz, Chiesa, Lin, Mishra, Spooner, *Proof-Carrying Data Without Succinct Arguments*, CRYPTO 2021, [official publication PDF][bclms], DOI `10.1007/978-3-030-84242-0_24`. Read §§1.1, 2.1–2.2 and Remark 1. Supports split witnesses/decider size, interleaved extraction, multi-instance auxiliary-I/O WEE for general PCD, and heuristic oracle-instantiation limits. Remark 2 distinguishes single-path IVC from layer-wise PCD; no unnecessary DAG guarantee is claimed for this path-only proposal.
- **[Halo]** Boneh, Drake, Fisch, Gabizon, *Halo Infinite: Proof-Carrying Data from Additive Polynomial Commitments*, CRYPTO 2021, [official publication PDF][halo], DOI `10.1007/978-3-030-84242-0_23`. Read §§1–4 definitions, especially bounded-additive caveat/Definition 3.3, and targeted §6/Definition 6.1 from the fetched text. Supports private/public distinctions and final accumulation/history checks. Its unrestricted additive interface is not assumed for bounded Akita hints; its full-version IVC proof was not read.

### Pinned source actually inspected

- **[Outer]** [Jolt `outer.rs`][outer], immutable revision above: trace/setup/prove/replay paths, panic check, full outer verification, public device and exact artifact decoding/capacity scope. Source was read through line 377; no command was run.
- **[Guest]** [Jolt recursion guest][guest], same immutable revision: aligned record framing, setup input/image handling, exact record decode, conjunction/count loop and return value.
- **[XOF]** [Akita Blake sparse sampler stream][xof], fork revision above: full displayed file, epoch/domain/root/index and byte-consumption behavior.
- **[Delivery]** [Pinned public delivery note][delivery]: exact dependency pin and qualifications concerning compatibility/public-fetch/proof measurements.
- Baseline context: [prior notes](AKITA_ACCUMULATION_NOTES.md), §§2, 4, 5, 9. This document's source claims were checked against the papers/pinned blobs listed above rather than treating the prior notes as a proof.

**Validation exercised:** qmd MCP navigation and targeted paper reads; primary publication retrieval and targeted §6 reading; read-only pinned source inspection; symbolic derivation of the two-distinct-scalars residual lemma and the acceptance counterexamples. No implementation, build, test, formatter, benchmark, branch change, guest execution, proof generation, or saved-artifact verification was performed. No numerical speedup/security strength is claimed. Parent integration should check relative-link spelling, completeness of the cross-document obligation ledger, and algebra/interface consistency; runtime verification requires a future implementation assignment.

[outer]: https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/examples/recursion/src/outer.rs#L81-L314
[guest]: https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/examples/recursion/guest/src/lib.rs#L138-L207
[xof]: https://github.com/markosg04/akita/blob/d0ee56a0b12db66b4c46a87c527169737694c4c7/crates/akita-challenges/src/sampler/xof.rs
[delivery]: https://github.com/a16z/jolt/blob/5c65167e269fdfc2917ecb82f420d349b18b4c87/specs/fast-recursion-public-delivery.md
[bcms]: https://link.springer.com/content/pdf/10.1007/978-3-030-64378-2_1.pdf
[bclms]: https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_24.pdf
[halo]: https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_23.pdf
