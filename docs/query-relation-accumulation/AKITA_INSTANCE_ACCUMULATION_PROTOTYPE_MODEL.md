# Direct native-archive receipt: versioned experimental model

## 1. Decision, authorization, and unchanged anchor

**Selected research design:** one fixed guest computes the concrete inner transcript and verifies the complete native instance-accumulation archive; one complete ordinary outer Jolt/Akita proof proves that guest's success. No graph accumulator, curve `g`, quotient `t`, original quotient source `C_t`, extra quotient opening, external graph decision, or graph-imposed four-coordinate outer field is needed.

This selection is **conditional experimental design, not implementation admission or concrete security certification**. The user's new decision permits an explicitly named, protocol/game-specific concrete-ARO heuristic. It does not prove that heuristic, approve an arbitrary CH-I/CH-O assumption, certify a guest, or waive supported-profile and performance gates. This note exposes an important implication rather than concealing it: the native-game premise used here supplies **CH-I's computational native-soundness content restricted to this exact family, setup, auxiliary-input class and resource envelope**. Calling it an ARO heuristic does not make that implication disappear. The authorized research category permits investigating this explicit specialization; it is not evidence that it holds. Final design admission belongs to the parent integration and independent review.

The anchor remains the original whole-run K-table relation and consecutive nonempty segment inputs **before** `PCS::prove_batch`: all original K sources, all-valid-source semantics, weak unit/product certificates, full native fold prefixes including strong checks, boundary/static checks, finite `1<=T<=16`, and one complete native final decision. No completed per-segment proof becomes an accumulation input. No RAM/Merkle predicate, canonical-short-witness restriction, homomorphic lattice commitment, or stronger execution semantics is introduced.

Pinned source baselines are Jolt `0791f79a55c3fadc3d327c78a72228696c338468` and Akita `83574331d4e51f8cce1d8689d05592fa4ef4c138`. The specifications below are proposed protocol semantics, **not a claim that those source revisions already implement them**.

## 2. Versioned concrete family

### 2.1 Identity and parameters

Name the experimental family `akita-instance-archive-direct/1` (`D1`). An admitted installation `I` supplies the exact application, approved guest image, profile/codec registry, matrices, public-input convention and finite capacities. These are parameters of a precisely defined family, not prover-selected runtime alternatives. Changing the hash adapter, circuit simplification/order, transcript grammar, sampler, field, source map, guest image or setup law creates a different installation/version and requires new review.

The fixed algebraic and transcript choices for D1 are:

| Item | D1 choice |
|---|---|
| Source field | `K=F_q`, `q=2^128-2^32+22537` |
| Inner scalar challenges | `E2=K[U]/(U^2-5)`; original sources/matrices remain K-valued |
| Candidate ordinary outer scalar profile | E2, subject to the complete new outer manifest and source/projection certificates; not an already implemented switch |
| Component output/state width | `lambda_A=1024` bits |
| Absorb block | `b=64` bytes |
| Absorb counter/site/output-block index | 64 bits each |
| Block-length field | `ell=7` bits |
| Component input width | `m=2+1024+64+1+7+512=1610` bits |
| Initial state | 1024 zero bits |
| Component primitive | Pinned Spongefish Keccak overwrite duplex, revision `ef9741346a150039427d9e0c02d6b2d73e93ec81` |
| Inner finite profile | `T<=16`, `Nseg=4096`, `Pmain=56`, RAM domain 65536 words, receiver capacity `2^22` K cells; the original 33 matrix roles |

E2 is already the inner profile's mathematical field. D1 removes the **graph reason** for E4; it does not prove that an unchanged K-only outer meets the intended statistical policy. E2 outer adoption must preserve full-width challenges and K-source/E-result separation. There is no requirement that the outer guest's actual trace or advice grid fit the inner `2^22` receiver.

`lambda_A` is an output length and ideal-model parameter, **not a claimed 1024-bit concrete security level**. Neither the permutation name nor this output width certifies a concrete challenge-security strength.

### 2.2 Exact H, W and P

For every positive `m<2^64`, retain the public family in [CONCRETE_GRAPH §§3–4](AKITA_INSTANCE_ACCUMULATION_CONCRETE_GRAPH.md); D1 selects `m=1610`.

`H_m(x)` packs bit coordinate `x_(8j+k)` into bit `k` of byte `j`, padding unused high bits with zero, and forms

`M = ASCII("akita/aro/component/v1") || LE64(m) || PackLSB(x)`.

The ASCII domain is exactly 22 bytes, without NUL. Initialize the pinned 200-byte, 136-byte-rate overwrite duplex to zero, absorb M once, and squeeze 128 bytes once, returning their least-significant-bit-first coordinates. Equivalently, overwrite the rate with each successive nonempty at-most-136-byte chunk, permute with all 24 Keccak-f1600 rounds after every chunk, and return the first 128 state bytes after the last permutation. There is no SHAKE suffix, pad10*1, subsequent absorb or ratchet. For `m=1610`, M is 232 bytes and this uses exactly two permutations. This is neither SHAKE nor Akita's default Blake2b transcript.

`W_m(x)` is the deterministic unique Boolean circuit trace specified there: fixed Keccak gate order, stated constant propagation and dead-ancestor removal, then right-comb SPLIT fanout elimination ordered by source and consumer port. Its coordinates are the G live gate outputs followed by the two children of each of S split nodes in the specified order, so `w=G+2S`. No optimal-circuit choice is left to an implementer.

`P_m` is the factored product of those gate, SPLIT and output-equality indicators, evaluated without polynomial coefficient expansion. Each port variable is used in at most one producing and one consuming factor. Hence individual degree is at most two and, on Boolean inputs,

`P_m(x,y,z)=1 iff y=H_m(x) and z=W_m(x)`;

it is zero on every other Boolean assignment. These are deterministic algebraic facts, not a security assumption. For D1's diagnostic tuple use `F=E2` and the same factors embedded from K; the graph proof works in this odd-characteristic field. This is an explicit field specialization, not reuse of the graph receipt's E4 security accounting.

Define the fixed public tuple `Theta_D1=(H_1610,W_1610,P_1610 over E2)`. All its algorithms and constants are public. W is input-local and has no H_o calls; P evaluation is finite public arithmetic. **The direct guest evaluates H only. It does not materialize W or evaluate P.** W/P remain a precise specification of the candidate concrete-ARO family and of what side computations an adversary can perform, not new runtime receipt objects.

### 2.3 Exact chained transcript and encoding layers

Use the data/finalization/squeeze compiler in [CHAINED_TRANSCRIPT §§2–3](AKITA_INSTANCE_ACCUMULATION_CHAINED_TRANSCRIPT.md), specialized as follows:

```
A(state,counter,final,length,block)
  = 00 || state[1024] || counter[64] || final[1]
       || length[7] || block[512]
S(state,site,index)
  = 01 || state[1024] || site[64] || index[64] || zero[456]
```

Integer fields **within these bitstrings** are fixed-width unsigned big-endian. Packing the completed 1610-bit string for H then uses the distinct LSB-first rule above. The LE64(m) adapter prefix is a third explicitly specified layer. These conventions must not be conflated by a host-endian cast.

The direct protocol has no CM/query-accumulator challenge. Tags `10` and `11`, malformed fields and nonzero padding remain ordinary H inputs, available to adversarial computation, but are not valid protocol operations. Thus 1610 does **not** assume an oversized literal `(C_g,C_t,...)` tuple fits, does not use compact CM, and does not need an external RootValid test.

For a canonical transcript P, absorb its 64-byte full blocks starting from IV. Data calls use `(counter=i,final=0,length=64)`. Always append one final call with `(counter=h,final=1,length=|tail|<64,tail||zero padding)`, including for an empty transcript or exact block multiple. At each challenge snapshot use the finalized state only for S calls; continue later transcript absorption from the pre-finalization appendable state. An S call returns a 1024-bit block; repeated `(P,site,index)` repeats exactly. The native bounded sampler consumes its precise raw bits, including rejected attempts and the specified abort behavior. Counters never wrap. Require `floor(|P|/64)<=2^64-1`, plus the much smaller installed transcript and sampler caps; exceeding any cap rejects.

Canonical native records use `Rec(type,payload)=u32be(type)||u64be(len(payload))||payload`. K atoms are 16-byte little-endian integers less than q; E2 atoms are `LE128(a)||LE128(b)` for `a+bU`. These are canonical values, not raw Montgomery storage. Every array/string is typed and length-delimited, all integer arithmetic is checked, and parse failures reject. The archive container follows [GUEST_SPEC](AKITA_INSTANCE_ACCUMULATION_GUEST_SPEC.md); its container framing need not equal this transcript-record framing.

The installation must supply the complete message-type/site registry and emission table proving an injective correspondence `eta(P,site)=native stream key`. It must bind the original application statement, setup/profile, source handles, preceding messages and precisely the challenge history used by the native theorem. It must not merge native streams or split one vector stream, absorb one coordinate into a sibling independent-coordinate key, reorder messages, or add fresh session salts to identical native keys. The formula is exact; **the full source-level emission table and finite byte bound remain implementation-admission deliverables**. No present document certifies them by inventing counts.

## 3. Setup, auxiliary input and runtime experiment

### 3.1 Setup and installation boundary

Use verifier-provisioned actual static data, not guest regeneration of a concrete setup XOF pretending to be a uniform matrix source. Follow [RECEIPT_SETUP §§3–5](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SETUP.md):

1. Freeze the D1 family, inner profile, program, codecs, capacities and source/key roles independently of challenge-oracle randomness. There are six A, five B, four D and eighteen F/H inner roles, 33 independent registered maps and 4,681,728 K coefficients.
2. Provision each inner coefficient from an independent uniform 512-bit draw reduced modulo q. The existing aggregate statistical-bias bound `<2^-361` applies to that sampling law. If implemented with a CSPRNG, explicitly assume that generator's security; the statistical statement alone does not justify replacing the joint distribution by a short public seed.
3. Form the canonical static blob and validate all proof-independent preparation. Proof-dependent contractions, transcript weights and acceptance equations remain guest work.
4. Fix guest image/capacities and generate independent outer setup according to its admitted ordinary matrix-view distribution. Preserve all required joint/shared-prefix correlations when embedding an ordinary SIS challenge. Outer key coefficients cannot depend on the later C_static or on an archive.
5. Commit the installed static bytes under the actual original trusted-advice source and freeze C_static, its source map/shape and canonical reference decommitment. The verifier obtains these from authenticated installation, not a proof-supplied label.

The D1 public family may be evaluated on inputs containing setup coefficients. That does not make its **algorithm** depend on a hidden setup trapdoor, and does not prevent an SIS reduction from evaluating it after embedding its challenge. Replacing provisioning with concrete public-seed matrix expansion is a separate assumption and is not selected.

Installation identifiers are selectors for independently authenticated records. If a digest is used to bind a variable-size program, profile or input, its canonical preimage/binding obligations and any collision assumption must be stated. A digest alone does not approve arbitrary replacement data. A fixed authenticated selector avoids adding a new hash primitive for this note.

**Security quantifiers:** let J fix the pre-provisioning protocol/application/geometry, auxiliary class and complete installation-generation law, but not the realized random matrices or C_static. Sample `I <- Provision(J; r)` inside each security experiment. Bounds below average over this provisioning randomness together with the remaining experiment coins. Preserve that sampled I throughout the original run and extraction. Conditioning on I to describe deterministic replay is not an ordinary-MSIS security bound for every fixed installation; such a stronger claim would require a separate good-setup theorem and loss.

### 3.2 Adversary and auxiliary class

The experiment gives A the public installation, actual keys, fixed H/W/P algorithms and permitted auxiliary input z. An efficient generator Z may depend on the public keys and may generate earlier transcripts; its initialization, oracle queries and concrete computation are included in the resource envelope. In the ideal-H_o theorem, oracle-independent initial code/coins are fixed before H_o sampling; any H_o-dependent preparation is an explicitly recorded prefix of the resettable experiment, not free advice. Random tapes, ordinary state, local oracle tables and output/auxiliary data needed for replay belong to the same-execution handle.

No setup trapdoor, hardwired solution correlated with the sampled matrices, uncharged fresh-oracle advice, external mutable fork-history service, or invisible preprocessing is allowed. Dependence on the public fixed concrete hash code is allowed; it would be absurd to treat that code as secret oracle advice. The precise auxiliary class is the one supported by the eventual ordinary-SIS reductions, not an unrestricted promise that arbitrary correlated nonuniform advice is harmless.

A may choose its X adaptively. The extractor retains that original X, original receipt, original acceptance bit, original source handles and installed static reference. It cannot switch to a more convenient statement or canonical commitment state.

### 3.3 Two different runtime ledgers

* **Honest cost:** segment frontend work, native updates, one native decider, concrete direct-guest execution and one ordinary outer proof. Hash/permutation work, parsing, rejected samples, matrix arithmetic, source commitments, static reads and peak memory are charged.
* **Security-reduction cost:** the complete outer tree/replay work, all concrete H/W/P computation by A/Z/extractor, exact H_o conditional samplers, setup embedding, source projections/reference comparisons, archive materialization and its final replay. A P call is not a unit-cost hash call. Each SIS term is evaluated at the corresponding complete reduction runtime.

The concrete adversary is bounded by total time, memory and admitted input/auxiliary size, **not by a mandatory instrumented hash API**. It may inline H, partially evaluate the public circuit, share intermediate work, or use another equivalent implementation. An honest transcript emission count or a helper-call counter is not a bound on all adversarial concrete hash work. This applies equally to B_direct and every replay inside its outer extractor. Translating these real computation bounds into ideal component/native query counts, bit-work and memory is part of the explicitly hypothesized resource translation R; it has not been derived by counting guest calls. In particular no native Q32 allowance is inherited from the honest ledger.

Expected-polynomial extraction does not certify a usable concrete bound. Truncate the continuation after an explicitly declared work cap L, without changing the saved original experiment. If `E[Z_work]<=M_o` unconditionally, a compatible cutoff has failure at most `M_o/L`; include separately bounded original work and final projection/check work according to the same meter. Query/memory caps need their own accounting or a proved dominating work cap. Do not substitute the first honest execution's query count for the derived archive producer's work, and do not preserve the old Q32 statistical number without a new bound.

## 4. Exactly which heuristic is being considered

### 4.1 Native-game preservation, not tuple indistinguishability

Let `L_run(I)` be the unchanged oracle-independent whole-run K-table language. Let `V_nat^Theta(I,X,a)` perform the **complete** bounded native archive verification using Chain of Theta's first interface; its final predicate uses no W/P calls. Define the event

`BadNat_Theta(B) := [B outputs (X,a), V_nat^Theta(I,X,a)=1, X notin L_run(I)]`.

The event is a mathematical security predicate; B need not decide language nonmembership. It includes the original handles/all-valid-source conditions, installed static data, endpoints and the complete final decider. Dropping those checks creates another game.

The ideal comparison draws `Theta_id=(C,wo,Vhat)` from the repaired input-local ARO model with `m=1610`, `lambda_A=1024`, `d=2`, `F=E2` and fixed input-local witness circuit B_W implementing W. In that model `wo(x)=W(x)` is allowed as a ro-free choice, while the Boolean graph uses the **sampled C output**, not concrete H(x). Vhat is uniformly sampled among its degree-two extensions. This comparison does not claim that W is an execution trace of sampled C.

**Hypothesis D1-NGP (native-game preservation).** Fix an admitted pre-provisioning descriptor J, setup/auxiliary class and bounded archive-producing machines as above; sample the realized installation I inside each comparison experiment. Replacing the ideal tuple by `Theta_D1` preserves computational hardness of BadNat **averaged over that specified provisioning law**. A quantitative use must supply a resource translation R and instantiation loss `delta_NGP` such that

\[
\Pr_{I\leftarrow\operatorname{Provision}(J),\,\mathrm{coins}}
 [\operatorname{BadNat}_{\Theta_{\mathrm{D1}}}(B)]
\le
\varepsilon_{\mathrm{N,id}}\!\left(\mathcal R(\operatorname{resources}(B)),J\right)
+\delta_{\mathrm{NGP}}\!\left(\operatorname{resources}(B),J\right).
\tag{D1-NGP}
\]

Here `epsilon_N,id` is a bound for the *native* ideal game, not for arbitrary oracle distinguishers or an unrelated receipt language. Resource translation includes every computation needed in the comparison. The approved research treats preservation as a **heuristic conjecture**. Neither a usable R nor a nontrivial numerical delta_NGP has been proved or certified for this tuple. A vacuous delta of 1 proves nothing; no numerical security claim is made by displaying this expression.

For context only, the reviewed ideal bound composes the input-local paired game reduction **before** chained-stream simulation and native soundness: native-flat error at the translated runtime, chain error, and the extended-call term at most `v/2^1024`, with setup/SIS and cutoffs as specified. See [CHAINED_TRANSCRIPT §7](AKITA_INSTANCE_ACCUMULATION_CHAINED_TRANSCRIPT.md). Those are ideal comparison costs. They are **not** newly incurred concrete emulator work in the direct extractor and do not bound delta_NGP. The simpler operational formulation is that the exact concrete native verifier has an unknown soundness bound `epsilon_N,concrete(t,...)` against the declared class.

D1-NGP is deliberately restricted to one native false-acceptance game. It asserts no knowledge extraction of the native whole-run witness, no generic recursion/PCD security, no graph zero-finding security, no preservation under arbitrary protocols, and no indistinguishability of H/W/P from sampled ideal functions.

### 4.2 Why a universal interpretation would be false

The product P is public and has efficiently recognizable zero factors. Fix a gate's Boolean ports inconsistently, making its factor zero, and set an unrelated coordinate j to 2. The concrete P is zero. A uniform ideal degree-two extension can be shifted by `alpha*X_j*(X_j-1)` without changing any Boolean value; its evaluation at that point is uniform. This gives a distinguishing gap `1-1/|F|` ([CONCRETE_GRAPH §7.1](AKITA_INSTANCE_ACCUMULATION_CONCRETE_GRAPH.md)). Public concrete H is also directly computable. No universal black-box emulation/indistinguishability claim can justify D1.

This distinguisher is not itself a false native archive. Conversely, correct Boolean graph semantics do not prove BadNat hardness. Only the explicitly scoped heuristic, or a future theorem establishing it, crosses that boundary.

### 4.3 Comparison with withheld CH-I and CH-O

[RECEIPT_SOUNDNESS §7.1](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SOUNDNESS.md) defines CH-I as concrete native archive soundness at the derived extractor runtime, and CH-O as the full same-execution outer archive-extraction contract (with an extra concrete knowledge premise if outer FS is concrete).

* **D1-NGP implies restricted CH-I.** Combine its inequality with the admitted ideal native bound to obtain computational soundness of this exact concrete native archive verifier. This is CH-I specialized to D1's setup, auxiliaries and resources. This is an implication, not a new proof of CH-I.
* **The minimal consequence actually used is restricted CH-I itself.** Without a specified tightness/resource translation, the qualitative D1 native-security assertion has the same security content as that restricted CH-I assertion. A quantified ideal-to-concrete preservation claim can be stronger; concrete native soundness alone need not prove that quantitative transfer.
* Public efficient W/P access adds no independent information to the concrete game: a machine can evaluate both from the public code. Thus native soundness with this side access is equivalent to native soundness without named W/P interfaces after charging their actual simulation work. It is not a stronger hidden-oracle capability.
* D1-NGP does not imply broad CH-I for other hashes, profiles, setups, auxiliary classes or native knowledge. It does not imply CH-O, full Jolt frontend adequacy or concrete outer extractability.
* **No CH-O is assumed to fill a proof gap here.** With ideal independent H_o, the outer extraction follows conditionally from mixed-tree extraction, ordinary SIS and deterministic source/guest certificates. Those certificates are concrete mathematical obligations. An unproved certificate is a blocker, not a new knowledge assumption or an error term set to zero.
* A deployed concrete outer FS instantiation remains the separately named ordinary multi-round FS heuristic **for the required same-execution extraction/composition use**, not merely collision resistance or language soundness. It is not a proved concrete-hash knowledge theorem and must not be represented as avoiding all of CH-O's concrete knowledge content.

The selected [outer compiler](AKITA_INSTANCE_ACCUMULATION_OUTER_TRANSCRIPT.md) now makes that separate premise precise as **D1-Outer-FS/1**: SHAKE256 on literal complete-context keys, with bounded full-E2 sampling and the specified rank-shell streams. Its independently reviewed ideal compiler theorem remains conditional on the carrier's deterministic extractor. Concrete same-execution extraction/composition is still an explicit heuristic with unknown loss; sharing a concrete Keccak permutation with the inner family does not prove ideal-oracle independence.

Therefore this note does not solve the former model obstruction without assumptions. It identifies which exact part of the newly permitted heuristic supplies the previously missing inner contract and removes unnecessary machinery around that contract. If authorization were interpreted as forbidding even CH-I's explicitly disclosed D1 specialization, D1-NGP could not be used; merely renaming it would be invalid. Returning to a concrete graph wrapper would not automatically repair that lack of native-game security.

## 5. Direct algorithms and public relation

### 5.1 Installation and public statement

Let `I` fix the approved D1 model/guest, source geometry, setup, static reference, application and all bounds. Public statement bytes are

`x_o = CanonicalEncode(receipt-format D1, authenticated id(I), X)`.

X contains exactly the original application's declared public I/O and metadata. A segment count is public only if that application convention requires it; the private count is always checked in `1..16`. There is no public graph accumulator, beta, quotient evaluation, C_t or archive. Output is the existing canonical postcard `1u32` encoding, byte `01`, with panic=false, enforced by the external verifier; it is not trusted because the prover writes a success flag into x_o.

Call the logical original untrusted-advice source `C_archive` (the source previously called C_g). This is the ordinary proof's actual source handle, not a new commitment. It need not be duplicated as an application public-input field. Its presence, shape and transcript binding, and the source-to-consumed-byte projection, remain mandatory. The trusted source is installed C_static. Neither source is replaced by an honest re-commitment during extraction.

### 5.2 Native prover and fixed guest

`Accumulate_D1` retains the existing native construction: execute each legal segment frontend up to pending native opening instances, verify the required source/domain obligations, perform one complete native update with all strong checks, retain the ordered prefix, and finally perform the final domain/normalization reduction, three shrinking folds and full A-only terminal. The private archive contains those prefixes and the single final decision, not full segment PCS proofs.

`G_D1(x_o; archive, static)`:

1. Validate the installation/profile selector and all bounded canonical encodings. Obtain the approved matrices/program from the original trusted static source. Check the archive's X against public X.
2. Parse and replay the common archive algorithm specified in GUEST_SPEC, including every original frontend consumer and stage boundary, deriving the original-handle opening ledger rather than trusting a supplied list.
3. Compute every inner transcript input/challenge internally with the exact D1 compiler, hash and bounded sampler. Supplied raw bytes may be checked if the grammar contains native messages, but no supplied oracle answer substitutes for a computed challenge.
4. Preserve the zero receiver/once-only initialization, predecessor/successor and PC/register/RAM endpoint relations, source order, original weak certificates, all full-prefix strong/domain/range equations and all supported bounds.
5. Execute the complete native final decider, both E2 planes, predecessor-bound terminal type, all equations and the discharge of every pending obligation. Require exact archive consumption. Any abort, malformed object, overflow, capacity exhaustion, missing prefix or leftover obligation rejects.
6. Emit exactly success 1 only after all checks complete; no failure path emits success.

`ProveReceipt_D1` executes this fixed guest with the actual original advice sources and produces **one complete ordinary outer proof**, including all ordinary frontend stages and the complete native PCS suffix. No graph-related extra source/opening is added.

`VerifyReceipt_D1` resolves authenticated I and expected X, constructs the precise public device with expected output `01` and panic=false, supplies the installed C_static, validates source presence/shapes and runs the complete ordinary verifier. It rejects mismatched image/profile/setup, malformed/public-output disagreement and incomplete proof. It does not externally replay the archive, recompute transcript roots, or evaluate P.

### 5.3 Extracted relation

The deterministic outer target is ordinary, finite and oracle-free:

`R_direct(I,x_o; W) = valid outer execution/source witness for G_D1, original C_archive/C_static linkage, installed static equality, correct consumed-byte decoding, and successful complete ArchiveAccept^Theta_D1(I,X,a)`.

The last predicate is ordinary code. It is not an oracle gate disguised as a CPU instruction. Installed-static equality is proved by comparison with the original reference source or a named mixed weak/reference collision, not assumed because the input is called trusted.

The projection need only recover authentic **consumed** archive bytes with the checked logical length, exact parser consumption and machine semantics. It must not invent a theorem that every unused K-valued physical advice word is a canonical u64/byte. The relevant [CONSUMED_BYTES](AKITA_INSTANCE_ACCUMULATION_CONSUMED_BYTES.md) and [OUTER_PROJECTION](AKITA_INSTANCE_ACCUMULATION_OUTER_PROJECTION.md) obligations survive unchanged in substance; graph-specific output clauses are absent.

## 6. Why actual-fixed-Theta extraction needs no concrete emulator

[PRODUCT_EXTRACTION Theorem A](AKITA_INSTANCE_ACCUMULATION_PRODUCT_EXTRACTION.md) conditions on an arbitrary globally fixed Theta and samples/programs only independent H_o. Its combinatorial argument is pointwise in Theta. It does not require Theta to be random to produce the accepting mixed tree; randomness of Theta enters later ideal-model reductions.

For D1 one can make the reduction still simpler: inline H, W and P as ordinary deterministic subroutines of the prover/environment. They are efficiently computable from public code, and any repeated input gives the same answer. Only H_o is an oracle. Rewinding changes inputs to those subroutines as ordinary computation, not their definitions. The extractor's allowed programming concerns H_o coordinate streams only; it never searches for preimages that force a concrete H output, changes the guest image, or evaluates an ideal H inside a fixed guest.

Given the full outer extraction/projection certificate, the saved accepting receipt yields either a witness of R_direct, a named outer matrix collision, or mixed-tree failure. The same original public statement, source handles and acceptance bit are retained. The deterministic decoder then yields an archive accepted by the **actual fixed concrete** native verifier, not a privately emulated continuation.

To turn a matrix collision into ordinary MSIS, embed the challenge into the named admitted joint setup view and sample the other views consistently. Run the same bounded extractor; evaluate all H/W/P calls directly. Their code needs no secret information or hidden ideal extension, so the ordinary reduction has no extra ARO oracle to eliminate. Charge all computations at their real cost. Retain the usual setup-distribution and mixed weak/reference collision lemmas.

Thus the concrete outer reduction has **no paired ARO emulator, no private-ARO seeding/coupling error, no `vFull/2^lambda_A` extraction term, no hidden-curve authentication, and no root/CM term**. This does not make the outer tree cheap or prove its complete frontend certificate. It also does not instantiate the native ROM theorem: that separate step is precisely D1-NGP/restricted CH-I.

## 7. Conditional composition theorem

Fix an admitted pre-provisioning descriptor J and A/Z resource class. Sample `I <- Provision(J)` inside the theorem's experiment, then preserve that original I through the prover run, receipt verification and extraction. All probabilities and ordinary-MSIS advantages below average over this law, not over adversary coins conditional on an arbitrary realized key. Assume:

1. The complete ordinary outer schedule, exact independent ideal H_o coordinate-stream interface/conditional samplers and all K-source/E2-result extraction and deterministic guest-adequacy certificates are valid.
2. Provisioning and source/reference comparisons use the specified setup laws; the relevant ordinary MSIS instances are hard at the derived reduction runtimes. Any identifier binding obligation is separately discharged.
3. D1-NGP holds for the bounded **derived archive producer**, with its original auxiliary distribution, or equivalently for this proof use a supplied concrete native soundness bound `epsilon_N,concrete`.

Let `epsilon_tree,o` be the full outer mixed-tree failure, e.g. `(Q_o+1) sum_r e_r` for the admitted complete stage list, not just the PCS suffix. Let `Delta_o`, `A_o` and `epsilon_bind` denote respectively the admitted outer setup deviation, sum of ordinary outer matrix-collision advantages at actual reduction times (including source/static comparisons), and any genuinely separate binding errors. Let `delta_cut` be the declared extractor cutoff loss. If these terms already include a given event, do not count it again.

Then in the hybrid experiment with concrete Theta_D1 and ideal independent H_o,

\[
\begin{aligned}
&\Pr[\operatorname{VerifyReceipt}_{\mathrm{D1}}=1
       \ \land\ X\notin\mathcal L_{\mathrm{run}}(I)]\\
&\quad\le
\varepsilon_{\mathrm{tree},o}+\Delta_o+A_o
+\varepsilon_{\mathrm{bind}}+\delta_{\mathrm{cut}}
+\varepsilon_{\mathrm{N,concrete}}
  \!\left(\operatorname{resources}(B_{\mathrm{direct}})\right).
\end{aligned}
\tag{D1-Receipt}
\]

Under the quantitative form of D1-NGP, replace the last term by

\[
\varepsilon_{\mathrm{N,id}}
 \!\left(\mathcal R(\operatorname{resources}(B_{\mathrm{direct}})),J\right)
+\delta_{\mathrm{NGP}}
 \!\left(\operatorname{resources}(B_{\mathrm{direct}}),J\right).
\]

**Proof.** Save the original receipt/run before extraction. Outside the mixed-tree, setup/binding, collision and cutoff events, deterministic source projection and guest adequacy return a canonically consumed archive a for the saved X with installed static bytes and actual `ArchiveAccept^Theta_D1=1`. Define B_direct to run this bounded extractor and output `(X,a)` when it succeeds; otherwise output failure. B_direct does not test whether X is false. It is a legal native archive producer: public concrete computations and privately sampled/programmed outer ideal tables are its ordinary resettable computation. Hence the probability of its false accepted output is bounded by the stated native game. Union the excluded events. This is a same-execution failure-event argument, not an unjustified conversion of an AFK success divisor to one.

This gives **conditional language soundness only**. It does not extract a whole-run witness under D1-NGP. Native knowledge would require another explicitly admitted concrete native knowledge premise and nested-runtime/reset analysis; none is selected. Ordinary outer *language* soundness alone is insufficient, because an accepting false inner archive may exist but be computationally difficult to find. The outer extraction is what produces the efficient native forger.

The primary Akita result supports the conditional ideal-H_o tree/PCS mechanism, not the new complete guest certificate. If a future extractor supplies a success factor rather than this additive original-run failure bound, the composition must retain that factor. A concrete h_o deployment adds the separately disclosed FS-instantiation heuristic; it cannot silently retain this as an unconditional concrete theorem.

The same-execution requirement fixes the **sampled** installation within a run; it does not remove setup randomness from the security experiment. A rare realized matrix with an easily found short kernel is consistent with small average MSIS advantage. Neither authenticated installation nor a valid geometry certificate rules out every such matrix, so no per-installed-key security theorem is asserted here.

## 8. Direct versus graph: assumption and implementation comparison

| Obligation | D1 direct | Supplied-answer graph route |
|---|---|---|
| Exact native language game | Concrete D1-NGP/restricted CH-I | Must still justify the relevant native game; graph truth alone is insufficient |
| Outer ordinary extraction/guest adequacy | Required | Required, for larger graph/quotient skeleton |
| Actual-fixed-public tuple calls in outer SIS reduction | Compute H/W/P directly | Also computable if the tuple is concrete; emulation is not inherently needed there either |
| Graph authentication/zero finding | Not used | Ideal theorem or an additional concrete protocol/game preservation claim required |
| Graph curve/quotient, C_t opening | Absent | Required by that design, with original source/full-capacity semantics |
| CM and root-recompute policy | Absent | Must select literal CM or separately admitted compact-CM behavior |
| Graph-specific E4 geometry | Absent | Present in the existing quotient receipt profile |
| Byte/source/static/native-final obligations | All retained | All retained, plus graph constraints |

The least premise actually needed for the **direct** theorem is native concrete language soundness, not preservation of the entire graph authentication composition. A broad concrete graph/receipt security conjecture might imply that native premise via a compiler from accepted archives to graph receipts, but that implication needs a complete honest compiler and its capacities/runtime, and is not automatic at identical resource bounds. Conversely native soundness does not prove graph zero-finding. Do not claim the two hypotheses are equivalent merely because both mention the same H/W/P.

The direct guest may do more concrete hashing than a skeleton, and an ordinary proof of that work may be expensive. Removing quotient work is an architectural simplification, **not a speedup theorem**. It is selected for experimental design because the fixed public tuple eliminates the outer-emulation obstacle and the disclosed native-game hypothesis already supplies the only inner security consequence needed by one-shot composition. Retain the graph construction as a separate ideal-model research result, not as mandatory scaffolding in D1 or an automatic security fallback. Returning to it requires its exact native/authentication, CM, original C_t, capacity, field and runtime gates.

## 9. Remaining guest/profile/parity requirements

Source cutover requires independent integrated review of this model, GUEST_SPEC's complete parametric algorithm/ownership contract and the [field/source migration design](AKITA_INSTANCE_ACCUMULATION_EXTENSION_MIGRATION.md). Implementation then produces the fixed binary, finite codecs/emission artifacts and geometry needed for **installation and accepting-implementation admission**. Requiring an already compiled guest before writing that guest would be circular; omitting its later adequacy/profile certificate would be unsound. The complete admission obligations are:

1. The complete GUEST_SPEC algorithm and finite canonical archive/static codec, actual transcript message/site emission table, all original source/static/boundary/final-decider behavior, and an exact derivation of archive/trace/counter/sampler capacities covering every admitted legal path, including noncanonical valid weak decommitments. No missing count is replaced by a benchmark guess.
2. A fixed guest image and proof that its actual consumed-byte execution implements that verifier, including hash bit/byte order, sampler failures, parser rejection, memory/advice linkage, success byte and panic behavior. High-level Rust correctness and passing honest vectors alone are insufficient.
3. Full outer frontend/tree/source certificates: original K-valued sources, complete E2 challenge entropy, grouping/padding/projection and all mixed weak/reference comparisons. Stage4 advice-claim ownership through the ordinary stage8 opening remains; it cannot be replaced by a newly assumed source claim. The outer matrix/source inventory is newly derived, not the inner 33-role registry reused by name.
4. New bounded outer geometry, actual advice/trace limits and runtime-priced hardness policy. Static coefficients alone occupy 74,907,648 bytes; the known word representation needs at least a `2^24` padded word domain before further static content. These facts do not fix total guest trace, archive advice size, digit expansion or peak memory. Removing C_t does not eliminate static data or source commitments.
5. Explicit complete ideal statistical/completeness accounting and the unknown concrete-instantiation boundary. Retain all supported T16 cases. The prior inner Q32 bound does not automatically cover B_direct. The source hash, the native heuristic and the outer FS heuristic do not confer a certified end-to-end bit-security level.
6. Independent integrated review of the implemented model/guest/source/profile certificates before acceptance claims. Eventual verification must cover producer/verifier agreement and adversarial mutations of encoding, hash/sampler behavior, source ownership, boundaries, all strong checks and final obligations. No checks were executed by this research task.
7. The original paired performance gate: compare the full whole-run receipt against completed-segment recursion at the same application/profile/setup/preprocessing and T16 workload, accounting for prover/verifier latency, proof/public transport, peak memory and preprocessing. Do not lower depth or omit expensive native/static work to manufacture parity.

No ZK, QROM security, concrete security strength, complete guest admission, measured parity, implementation artifact or measured capacity is claimed.

## 10. Primary evidence and provenance

* [CCGOS, *Proof-Carrying Data From Arithmetized Random Oracles*, accepted manuscript](https://wrap.warwick.ac.uk/id/eprint/176570/1/WRAP-proof-carrying-data-arithmetized-random-oracles-Spooner-2023.pdf), §2.2, printed pp.6–8; [local text lines 309–417](SOURCE_PROVENANCE.md#supplemental-primary-sources). It explicitly motivates a concrete H/W/P interpretation, proves in an idealized model, and samples a uniform bounded-individual-degree extension; it leaves weaker modeling choices open. It does not prove D1-NGP.
* The local QMD primary index identifies Akita as `lattice-papers/akita.md`, docid `#bfa1a6`. [Primary parsed Akita](SOURCE_PROVENANCE.md#akita), Definition 3.15, Lemma 3.22, Lemma 10.2, Theorem 10.31, Remark 10.32 and Theorem 10.33: same-execution extraction, complete mixed-tree sampling, ideal coordinate-stream conditional programming, algebraic rather than canonical-short extraction, and ordinary MSIS with full extraction runtimes. In particular lines 3417–3423 explicitly distinguish ideal programming from concrete seed preimages; lines 4175–4229 contain extraction and ordinary shared-view reduction.
* [Pinned Spongefish instantiation](https://github.com/arkworks-rs/spongefish/blob/ef9741346a150039427d9e0c02d6b2d73e93ec81/spongefish/src/instantiations/mod.rs), [duplex](https://github.com/arkworks-rs/spongefish/blob/ef9741346a150039427d9e0c02d6b2d73e93ec81/spongefish/src/duplex_sponge.rs), and [FIPS 202](https://doi.org/10.6028/NIST.FIPS.202) specify the reused primitive/permutation; CONCRETE_GRAPH records the inspected source semantics and degree proof. None supplies an ARO-instantiation theorem.
* [PRODUCT_EXTRACTION](AKITA_INSTANCE_ACCUMULATION_PRODUCT_EXTRACTION.md), [OUTER_PROJECTION](AKITA_INSTANCE_ACCUMULATION_OUTER_PROJECTION.md), [CONSUMED_BYTES](AKITA_INSTANCE_ACCUMULATION_CONSUMED_BYTES.md), [RECEIPT_SOUNDNESS](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SOUNDNESS.md), [RECEIPT_SETUP](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SETUP.md), [CHAINED_TRANSCRIPT](AKITA_INSTANCE_ACCUMULATION_CHAINED_TRANSCRIPT.md) and [CONCRETE_GRAPH](AKITA_INSTANCE_ACCUMULATION_CONCRETE_GRAPH.md) supply the reviewed conditional interfaces used here. Historical statements that the concrete heuristic was unapproved describe the previous user decision; this note changes the authorized experimental scope, not the truth of a mathematical theorem.

Work performed: document/primary-source research and this assigned design document only. No code, build, test, formatter, benchmark, proof execution or Git operation was performed.
