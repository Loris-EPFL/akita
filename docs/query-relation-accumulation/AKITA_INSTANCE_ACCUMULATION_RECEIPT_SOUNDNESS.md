# One-final succinct lattice receipt: soundness, extraction, and oracle boundary

## 1. Decision and scope

**Research result, not implementation admission.** A single final Jolt/Akita proof of a fixed, bounded archive-verification guest is a coherent receipt architecture. It can keep the entire native archive and final decider out of the public input. The elementary composition theorem below is complete **conditional on its explicit component contracts**. The currently reviewed native classical-ROM theorem and Akita PCS theorem do **not** establish those contracts for a guest that computes a concrete inner Fiat–Shamir hash.

**Historical and current scope:** the broad CH-I/CH-O contracts below were withheld during proof-first research. The user later authorized a protocol-specific heuristic prototype; [D1](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) explicitly uses the restricted native-language-soundness content of CH-I and derives its ideal-independent-H_o outer composition conditionally rather than assuming an arbitrary CH-O certificate. Its separately disclosed concrete outer-FS premise retains the required extraction-use scope. D1's reviewed model supersedes this note as the experimental contract; neither user permission nor the separate ideal supplied-answer results prove the concrete hypotheses.

The decisive obstruction is not unlimited recursion: it already occurs at this one wrapper. An ordinary circuit/zkVM can evaluate a specified finite hash algorithm, not an ideal random oracle. Replacing the oracle with that algorithm changes the inner security premise. Independently, ordinary outer *language soundness* is not enough to turn a proof that some accepting computational inner transcript exists into an efficiently generated inner forgery. We need outer extraction of an accepting archive, or a separately proved joint theorem with the same implication.

The recommended scope remains **one wrapper at the end**, not new IVC, not a recursively accumulated wrapper, and not a return to completed-segment-proof accumulation. Establishing a ROM theorem for the native archive remains valuable evidence for a concrete instantiation, but is not a proof of the concrete instantiation.

### Fixed target

The anchor is the existing reviewed whole-run Jolt Boolean-table relation over K, with the same original sources and approved program/public-I/O semantics. The selected input to accumulation is the segment frontend **after its original claim reductions have constructed the native opening instances, and before `PCS::prove_batch`**. An update combines the old pending receiver and fresh pending source instances. No complete per-segment PCS proofs occur in this construction.

This note uses the admitted finite compact profile from the [design](AKITA_INSTANCE_ACCUMULATION_DESIGN.md), [review](AKITA_INSTANCE_ACCUMULATION_REVIEW.md), [compact profile](AKITA_INSTANCE_ACCUMULATION_COMPACT_PROFILE.md), [global manifest](AKITA_INSTANCE_ACCUMULATION_GLOBAL_MANIFEST.md), [segment interface](AKITA_INSTANCE_ACCUMULATION_SEGMENT_INTERFACE.md), and [matrix registry](AKITA_INSTANCE_ACCUMULATION_MATRIX_REGISTRY.md):

- `1 <= T <= 16`, main source P=56, original four update handles (old receiver, main, two RAM endpoints), capacity N=2^22 and 3,555,328 live successor cells.
- K=F_q, q=2^128-2^32+22537; E=K[U]/(U^2-5). Sources and lattice setup stay over K; challenges and both carrier planes follow the approved extension specification.
- Generic old-source domain checks stay in the archive. All frontend and update prefixes are retained, followed by the final domain/normalization prefix, three actual shrinking folds, and the complete A-only terminal decider.
- The original 33 matrix roles, source maps, noncanonical valid decommitment paths, full-grid predicates and final terminal type remain unchanged.
- The semantic claim is concatenation equivalence to the existing whole-run relation. It is **not** an added emulator, canonical-word, host-advice, MMIO, or halting theorem. No Merkle RAM predicate is introduced.

The older completed-proof residual/history construction is not a premise of this note.

## 2. Exact relations and receipt interface

Fix an approved index P containing the application/profile/setup specification, transcript grammar, bounded samplers, source/query maps, inner verifier algorithm, serialization, segment limit, and final-decider policy. Let pp_i be its actual inner public setup. Setup identities resolve to these approved objects; an arbitrary prover-supplied digest does not approve a new setup.

The selected [receipt setup contract](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SETUP.md) provisions the actual inner matrices, canonical program and approved proof-independent preparation as a static blob S_static. The independently provisioned outer advice key/source geometry is frozen first; installation commits S_static under that original map and installs C_static. The archive remains untrusted private advice; S_static is a separate committed trusted-advice source. The outer extractor must link the guest's static reads to that **same original** C_static and compare the extracted source with the known canonical reference source. A mismatch yields a charged mixed weak/reference A/B/F collision, rather than permission to accept replacement matrices. This is the selected mode; concrete setup-XOF evaluation inside the guest is only the alternative analyzed in §7.3.

Let x be the compact public application statement: approved application/profile identity, actual original public input and output statement, and any run identifier or endpoint/count fields that the selected application actually declares public. If program or I/O is represented by a digest, the guest must check the prescribed preimage/binding relation; a digest alone does not establish an application predicate. The entire program can instead be fixed in P. A public segment count costs a bounded scalar and is optional unless it is part of the intended statement; in either case the private count must be checked and bound to the chosen statement convention.

Define R_run(P,x;w) to be the exact approved whole-run Boolean-table relation, with w containing its K-valued source tables and legal boundary data. Write L_run(P)={x: exists w, R_run(P,x;w)=1}. This semantic relation has no dependence on the Fiat–Shamir oracle. Its meaning is not enlarged by wrapping a verifier.

For an inner challenge implementation χ (an ideal oracle in the ROM experiment, or the designated concrete hash/stream algorithm h_i in the concrete experiment), define

    ArchiveAccept^{χ}(P,pp_i,x,a) ∈ {0,1}.

The private byte string a contains the whole ordered native archive **and its one final decider**. `ArchiveAccept`:

1. Parses bounded, canonical, length-delimited bytes and rejects malformed fields, unsupported versions/shapes, oversize allocations/counts, integer overflow, trailing bytes, and counts outside 1..16.
2. Binds P, pp_i and x before the first challenge; starts from the registered zero receiver and declared initial state.
3. Replays every original frontend check and derives, rather than trusts, the ordered original-handle opening ledger. Preserves the modern native-column claim semantics at Jolt `0791f79` / Akita `8357433`.
4. Checks indices/order, exact predecessor-successor and source-handle links, complete PC/register/RAM endpoint predicates, once-only initialization, no reset or dropped prefix, and the original once-only global final/public-I/O checks.
5. Replays every update prefix in the admitted transcript order, including original-source domain checks, bounded samplers and all strong/relation/range checks. A pending opening is never accepted as a completed proof.
6. Executes the complete final domain/normalization reduction, three shrinking folds and full-ring terminal, checks the actual predecessor-bound terminal type, both extension planes, all equations/bounds, and emptiness of every pending obligation. Rejects an incomplete decider.
7. Accepts only after consuming the entire archive. All failures, sampler exhaustion and capacity exhaustion mean rejection, not alternate success paths.

For concrete h_i, define the ordinary finite relation

    R_wrap(P,pp_i,x; a,τ) = 1

iff τ is a valid bounded execution witness of the **fixed approved guest** G_{P,h_i}, whose public input is x, private untrusted advice is a, and canonical public success result is exactly 1, with `ArchiveAccept^{h_i}(P,pp_i,x,a)=1`. The guest has fixed advice-size and execution bounds covering every admitted archive; the guest image is not generated from the run's archive. In the selected mode τ also contains static advice s and the intended relation includes `s=S_static`; the original-source/reference binding reduction establishes this equality, not a second embedded byte copy. The notation suppresses s only after this binding obligation is discharged.

The external verifier uses its approved guest index and outer verification key pp_o, verifies one **complete** outer Jolt/Akita proof π_o for the exact x and canonical success output, and rejects mismatched image/profile/setup, panic/failure, output 0, malformed output, or trailing output data. The default puts the final decider inside G; it is not an unverified external attachment. Public statement bytes plus π_o are the receipt; neither a nor τ is public input or per-run code.

### Guest adequacy is a real premise

Let R_VM be the actual outer Jolt algebraic execution relation. We require a deterministic projection D which maps every valid outer witness for the approved guest and output 1 to its read archive a and static bytes s, correctly executing the byte verifier on those bytes. The separate source/reference binding lemma forces `s=S_static` except at its charged matrix-collision event; only then does the projection yield `ArchiveAccept^{h_i}(P,pp_i,x,a)=1` for the approved pp_i. These requirements include byte ranges, parser control flow, private/trusted-advice access, hash implementation, memory layout, and success encoding. Proving only the existence of some K-valued tables is not automatically a proof that the intended byte verifier ran. The inner semantic audit does not establish this separate outer-guest adequacy lemma. Adequacy needs exactly the selected guest semantics, not a new stronger theorem about all emulator features. Charge static-source mismatch in the outer matrix ledger or ε_bind once, not both.

## 3. What the source theorems actually supply

### 3.1 Native Akita

[Akita, Definition 3.15](SOURCE_PROVENANCE.md#akita) fixes the original prover's auxiliary input, initial state, random tape and oracle transcript in a resettable same-execution handle. The extractor must recover the witness for that execution's statement, not switch to a convenient statement.

[Akita, Theorem 10.31](SOURCE_PROVENANCE.md#akita) extracts an algebraic batch decommitment or a short nonzero collision in an inventoried matrix view from a complete accepting mixed transcript tree. [Remark 10.32](SOURCE_PROVENANCE.md#akita) expressly does not claim canonical short gadget decomposition. [Theorem 10.33, Eq. (229)](SOURCE_PROVENANCE.md#akita) gives adaptive batch knowledge soundness for the admitted assembled PCS schedule in the **classical ROM**:

    (Q+1) ε_int + Δ_setup + Σ_I Adv_MSIS,I(B_I).

Equations (227)–(228) include the entire mixed-tree/Fiat–Shamir extraction runtime in each B_I. The theorem is a PCS statement, not by itself a full Jolt private-advice execution/guest extractor. The reviewed finite native-archive extension supplies its separately reviewed frontend/source/history argument; it is not literally Akita's original root-only schedule.

A particularly relevant limitation is [Akita, Lemma 10.2 and the discussion immediately after it](SOURCE_PROVENANCE.md#akita): the conditional sampler programs an **ideal coordinate stream**, not a preimage of the concrete 256-bit SHAKE seed. This is directly incompatible with claiming that concrete hash evaluation in a guest automatically implements the extractor's oracle programming.

### 3.2 AFK

Attema–Fehr–Klooß, [*Fiat–Shamir Transformation of Multi-Round Interactive Proofs*, extended journal version](https://ir.cwi.nl/pub/33324/33324.pdf), supplies:

- **Theorem 2**, §6.1: the (Q+1) knowledge-error loss for multi-round special-sound protocols with a common challenge space.
- **Theorem 3**, §6.2: the corresponding arbitrary challenge-space formulation. Remark 11 warns that bitstrings need not map injectively or uniformly to challenges.
- **Theorem 4**, §6.3: adaptive knowledge soundness for the adaptive transform that binds the statement into challenges.
- **Definition 10 and Remark 5**, §2.5: the extractor preserves the original `(x,π,aux,v)` distribution, begins with an honest oracle execution, and subsequently rewinds/programs the oracle. Auxiliary output is present specifically for compositional use.
- **Proposition 2**, §6.1: in its model, expected calls are at most K+Q(K-1), where K is the product of special-soundness arities. This is not a universal exact bound for the Akita mixed-tree construction or nested receipt extractor.
- **§2.4 and Remark 4**: an argument can be treated as a proof for “valid witness OR computational-assumption break.” Ordinary language soundness of that disjunctive relation can be vacuous because a hard-problem witness exists for every statement. Knowledge is what makes the reduction meaningful.

AFK starts from an NP relation R and algorithms with explicit oracle access. Its theorem does not say that a circuit may compute an ideal oracle, nor prove concrete-hash instantiation, nor automatically supply extraction for oracle-dependent relations with a preserved external oracle. Its auxiliary-output guarantee is useful but is not the whole oracle-preservation contract below.

### 3.3 BCMS and BCLMS: a directly relevant negative scope statement

[BCMS20, *Recursive Proof Composition from Accumulation Schemes*, published TCC chapter](https://link.springer.com/content/pdf/10.1007/978-3-030-64378-2_1.pdf), **Theorem 1 (informal)** and the paragraph immediately after it, explicitly place the PCD compiler in a CRS model **without oracles**, because it uses the accumulation verifier non-black-box. Section 1.1 then calls its instantiated PCD constructions **heuristic**, after concrete instantiation of the random oracle. **Theorem 2 (informal)** is a different, black-box predicate-efficient accumulation compiler that can preserve oracle-relative security; it does not close the non-black-box verifier-in-guest step. Section 2.4 additionally requires domain separation between PCS and accumulation oracles.

[BCLMS21, *Proof-Carrying Data Without Succinct Arguments*, published CRYPTO chapter](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_24.pdf), **Theorem 1 (informal)** repeats the no-oracle restriction. **Remark 1** states that PCS aggregation alone does not imply PCD without additional heuristic assumptions; its standard-model NARK accumulation security is an assumption conjectured from ROM evidence. In **§2.2**, the recursive proof requires multi-instance witness-extended emulation with auxiliary input/output and extraction from expected-time adversaries; **Remark 2** explains that single-instance extraction suffices for path-only IVC.

These are authoritative scope warnings, not impossibility theorems for this particular one-shot wrapper. We do not import their DL/Pedersen constructions or claim an uninspected formal theorem from their extended versions. The published Theorems 1–2 cited here are explicitly informal. The elementary one-shot lemma next needs only one outer extraction, not the full PCD multi-instance machinery.

## 4. Why ordinary outer soundness does not suffice

Fix concrete h_i and pp_i. Let

    L_acc = {x : exists a, ArchiveAccept^{h_i}(P,pp_i,x,a)=1}.

Outer language soundness says that no efficient adversary proves x outside L_acc (assuming guest adequacy). Inner computational soundness instead says that no efficient adversary **finds** an accepting a for x outside L_run. It does not establish the set inclusion L_acc ⊆ L_run. A rare accepting transcript, or a commitment collision enabling one, can exist for a false run even when finding it is infeasible.

Consequently, the outer theorem's bad event `x notin L_acc` can fail to occur precisely when we need to exclude `x notin L_run`. Even if the public statement is fixed, this quantifier mismatch remains. This does not exhibit an efficient forgery against Akita: it shows that the proposed black-box argument has no implication at that point.

A generic separation makes the logical issue explicit. Let an inner verifier for an empty target language accept a preimage of a public one-way-function image y. Such an accepting witness exists, but producing it is computationally hard. The outer language “there exists an accepting inner witness” contains its sole statement, so an outer language-sound proof system may accept it without revealing or knowing the preimage. Composition is false although both relevant language-soundness claims hold. AFK §2.4/Remark 4 gives the analogous witness-or-assumption-break phenomenon for computational arguments.

Sufficient replacements are:

1. **Outer archive knowledge/extractability**, as formalized below; or
2. a new direct composed soundness theorem which turns an accepting receipt for a false run into an efficiently generated inner forgery; or
3. an inner verifier with a justified statistical/perfect language-inclusion property strong enough to union over *existence* of accepting witnesses, rather than only efficient attempts.

The reviewed lattice archive provides computational soundness, not item 3. “Akita is binding” and “the guest returns true” are not substitutes for items 1 or 2.

## 5. Conditional one-shot composition theorem

### 5.1 Oracle and auxiliary-input interface

Use separate logical interfaces:

- H_i: inner frontend/update/final challenge streams, with their internal stage/coordinate namespaces;
- H_o: outer frontend/PCS challenge streams;
- S_i and S_o: inner and outer setup expansion/materialization, separately indexed;
- H_id, if used: profile/program/public-I/O identity binding, distinct from challenge and setup domains.

For a concrete implementation the corresponding named functions are ordinary deterministic algorithms. In an ideal experiment, independence of these namespaces follows from disjoint, injectively encoded domains of an ideal oracle. It is not a consequence of merely using different strings with a concrete hash.

A setup experiment samples the approved setup and gives the adversary its public keys and allowed auxiliary input z. z may include efficiently generated prior transcripts and application data. Its generation, oracle queries and costs must be included in the security experiment. It cannot contain adversarially chosen trapdoors for an allegedly uniform setup or uncharged nonuniform advice encoding fresh-oracle answers. The index can be fixed; if an admitted index family is selectable, the same-execution statement includes the selected index and the reduction inventories its entire allowed family.

The outer extractor gets a resettable handle for the **same** adversary execution that produced `(x,π_o,aux)`. It must preserve x, the approved index, acceptance bit and auxiliary output, including the original external oracle view needed by the application. Extracting a proof of a different statement is not success. Single-instance extraction suffices here: there is one outer receipt, regardless of the number of records inside its witness. No simulation-extractability or universal concurrent-PCD claim is asserted.

### 5.2 Contracts

Let `RecV(x,π_o)=1` abbreviate all external admission, input, full-proof, and success-output checks. Fix the relevant experiment, either concrete inner h_i or an explicitly oracle-aware model described in §7.

**O — outer archive extraction.** For each permitted receipt adversary A there is an emulator/extractor X_o which first obtains the original `(x,π_o,aux,v)` and, except with total probability β_o, whenever v=1 returns a such that `ArchiveAccept(P,pp_i,x,a)=1` in the **same inner verification environment**. It preserves the joint distribution of the original output/auxiliary/external-oracle view. Let M_o bound its expected total work, including A, oracle simulation, extraction and the deterministic guest-witness projection. If the theorem supplies only success `(p-κ)/d` for some d>1 rather than this failure-event contract, retain that factor in the final bound; do not silently set d=1.

β_o includes outer frontend/PCS knowledge error and failures of the admitted guest-witness projection. Correct deterministic adequacy contributes zero; an unproved adequacy claim is a missing premise, not a small guessed error. Binding reductions for program/setup/I/O identities can instead be exposed as a separate ε_bind term below.

**I — inner computational language soundness.** For every allowed archive-producing algorithm B with runtime at most t and inner-oracle query count at most q,

    Pr[ArchiveAccept(P,pp_i,x,a)=1 and x notin L_run(P)] <= ε_i(t,q).

The experiment supplies pp_o, the approved guest index and z to B in exactly the distribution supplied to A, or proves they are simulatable independent auxiliary data. The source native theorem is sufficient for I only in its stated ideal-oracle/setup model, at the derived B's runtime/query bounds.

**C — common environment and admission.** The wrapper relation is the actual admitted guest relation; all identifiers bind their intended preimages; both schemes use the same pp_i and same inner verification function; outer extraction never changes that function/oracle/setup; all bound/index/shape checks are enforced. In an oracle-aware variant this is an additional relativized extraction contract, not implicit in ordinary O for an oracle-free NP relation.

### 5.3 Theorem A: receipt language soundness

Under O, I and C, choose explicit work and inner-query cutoffs t_* and q_*. Let

    δ_cut = Pr[X_o exceeds t_* work or q_* inner queries].

Then

    Pr[RecV(x,π_o)=1 and x notin L_run(P)]
      <= β_o + ε_bind + δ_cut + ε_i(t_* + t_proj, q_* + q_proj).       (1)

Here projection/check costs and any inner queries needed after extraction are either already in X_o or charged explicitly as t_proj,q_proj, not both. If X_o has deterministic bounds within the policy, δ_cut=0. With only expected runtime M_o, a time cutoff contributes at most M_o/t_* by Markov; a separate expected-query bound M_Q gives at most M_Q/q_* for a query cutoff (use their union bound, or one dominating cutoff). These are explicit reduction losses, not implementation retry policies.

**Proof.** Couple the real receipt experiment to the original run produced by X_o. The bad receipt event has identical probability under this coupling. Exclude identifier-binding failure, outer extraction failure and the cutoff event; their probabilities are bounded by ε_bind, β_o and δ_cut. On the remaining bad event, X_o outputs the original x and an accepting a for that x in the original inner environment. Define B to run this truncated emulator/extractor and output `(x,a)` when it succeeds; otherwise output failure. B never needs to decide whether x is false. It is an allowed archive forger at the stated work/query bounds, so the probability of the remaining false-accepting output is at most ε_i. Summing excluded events proves (1). There is no union over T outer proofs and no recursive extraction chain. ∎

If the available O contract guarantees only `Pr[bad original receipt and valid extracted archive] >= (p_bad-β_o)/d_o`, the same argument yields `p_bad <= β_o + d_o(ε_i+δ_cut+ε_bind)` with the corresponding conditioning/coupling contract. An unconditional extraction-success statement about all accepting receipts cannot simply be substituted for the stronger guarantee restricted to the bad-original-statement event. Same-execution extraction or a suitable witness-extended emulation statement is the safe interface.

**Minimum knowledge needed.** The conclusion of Theorem A is language soundness for R_run. It needs extraction of a, not extraction of the original VM witness w, and only *language* soundness of the inner archive. Thus a stronger inner knowledge theorem need not be composed just to rule out false runs. This can avoid imposing an unnecessary expected-time closure hypothesis on the inner extractor.

### 5.4 Theorem B: optional whole-run knowledge

For a knowledge claim, strengthen I to same-execution native archive knowledge extraction on the derived archive-producing algorithm B. Require that its algorithm class includes B, either by deterministic truncation as above or an explicitly justified expected-time closure theorem. The inner extractor must preserve the statement selected by B and be able to replay B from a complete resettable handle, including the outer extractor's random tapes, simulated outer oracle states and auxiliary input. Its output is w satisfying R_run or a charged setup/matrix failure.

Then compose `X_run = X_i[X_o[A]]`, retaining the original receipt's x and accepting bit and projecting the native original-source witnesses to the approved whole-run witness. On each inner counterfactual fork, rerun the **whole derived archive producer**, not merely the stored a from the original run. A fixed accepting a alone is not a resettable prover and cannot supply the native continuation tree. If `X_i` meets a same-run failure-event bound β_i(B), the total failure probability is at most β_o+β_i(B)+ε_bind+δ_cut; otherwise retain its actual success loss rather than asserting an additive formula.

The runtime is nested:

    T_run = T_Xi(B; schedule_i),   where one call to B costs T_Xo(A),

plus projection and setup/collision work. Independent EPT components alone are not enough without the required conditional runtime bounds/reset interfaces; hence the explicit closure/truncation premise. No canonical honest gadget state is promised, only the bounded/source-certified witnesses that the reviewed native relation actually extracts.

## 6. Security ledger: preserve, do not inherit silently

### 6.1 Inner ROM expression

For the reviewed archive, when I is applied in its actual classical-ROM model,

    ε_i(t_B,q_B)
      <= (q_B+1)(c_E/|E| + c_C/|C|)
         + Δ_i + Σ_{I=1}^{33} Adv_MSIS,I(R_i,I(t_B,q_B)).             (2)

Use the complete manifest, not just native PCS challenges. Its supported policy is

    q_B+1 <= 2^32,   c_E <= 2^40,   c_C <= 2^16,
    |E|=q^2,
    |C|=binom(128,40) binom(40,24) 2^40.

Thus its statistical FS contribution is `<2^-137`, as originally reviewed. Δ_i is below 2^-361 for the 4,681,728-coefficient, 512-bit modular-sampling setup in the **declared ideal setup/XOF model**. No total 128-bit claim follows from (2).

**New query accounting caused by the wrapper:** q_B is the query count of the *derived archive-producing algorithm*, including original adversarial inner queries, queries on alternative executions during outer extraction, advice/guest reconstruction and final archive replay if performed. It is not necessarily the query count of A's first execution. The inner extractor's own subsequent forks are instead priced in R_i,I; do not count them again in q_B. Outer-oracle queries do not directly count as inner queries if domains are disjoint, but their rewinds can cause new inner queries. Counting only the actual accepted archive's transcript is unsound.

Consequently, (2) is applicable at the old numerical strength only if `q_B+1<=2^32` is certified. If it is not, choose and justify a cutoff with δ_cut explicitly small enough, improve the extraction interface, or re-admit a stronger profile. Do **not** retain the old `<2^-137` number while replacing q_B with the smaller original adversary budget. A verifier cannot enforce an adversary's query limit; it is the bound in the security statement.

The known loose tree ceiling

    2^(2048 H) (λ+N_exp+1)^(16 H),  H=2^16,

is formally polynomial for this fixed grammar but not a useful concrete runtime certificate. Every R_i,I includes the full inner extraction of the derived B, all continuation replays, the outer extractor on every needed replay, setup embedding and collision computation. The 33 advantage terms do not divide this runtime by 33.

### 6.2 Outer additions

There is an additional, independently admitted outer ledger, schematically

    β_o <= ε_front,o + ε_PCS,o + ε_comp,o,
    ε_PCS,o <= (q_o+1) ε_int,o + Δ_o
                  + Σ_{J in I_o} Adv_MSIS,J(R_o,J).

This split is only valid when a proved frontend-to-PCS extraction composition justifies it. A joint outer manifest may instead give a single combined error. ε_comp,o must be derived or stated as a missing theorem; naming it is not an upper bound. Include all outer frontend challenge errors, private-advice commitment/opening obligations, batching/padding/setup checks, final terminal, query losses and extraction runtime. An unchanged K-only outer has its own field-size ceiling; using the inner E profile does not automatically move the outer frontend or PCS challenges to E. An ordinary q128 outer cannot be advertised as satisfying the reviewed statistical policy without its own count/query calculation.

For an ideal/oracle-aware instantiation of (1), the complete bound is the sum of the applicable outer extraction ledger, identity binding, cutoff loss, and (2), with all runtimes and query interfaces matched. For a **concrete inner hash**, (2) cannot be substituted: use the separately assumed/proved `ε_i^{h_i}(t_B)` instead. A symbolic “hash-instantiation error” may be named only as an additional assumption with an explicit bound; ROM analysis provides no such small error by itself.

The inner `<2^-137` policy remains intact. The outer stage and any cutoff/binding contributions must be separately budgeted at the selected statistical target; no silent downgrade to a single-base-field error is permitted. Neither this note nor the prior review demands a tight overall 128-bit computational reduction.

### 6.3 Completeness and privacy

Bounded sampler aborts remain completeness losses, not automatic soundness losses: inner E sampling has the reviewed `<2^-151` total honest-abort bound and shell sampling `<2^-234`, under their sample-count caps. A verifier rejects on abort. Add the admitted outer sampler/completeness loss and ensure its trace/advice capacities cover **all** valid supported archives, not just a sample honest run. Canonical wire encoding does not force canonical gadget witnesses; retain all admitted noncanonical source/decommitment paths.

Private/untrusted advice means “not public input,” not zero knowledge. A clear outer proof may leak witness information. No ZK, QROM, measured speedup, bounded peak-memory result, or proof-size measurement is established here.

## 7. The concrete-hash versus random-oracle obstruction

### 7.1 The implemented guest proves a different relation

A guest containing concrete h_i proves `ArchiveAccept^{h_i}=1`. The native ROM theorem concerns `ArchiveAccept^{H_i}=1` for a uniformly sampled ideal H_i accessible through oracle calls. Identical honest behavior on tested inputs is irrelevant to this model change.

The inner extractor's conditional programming changes selected oracle answers while keeping other answers consistent. If h_i is computed by fixed guest instructions, changing an external oracle table does not change those instructions' output. Changing the guest code or its constants changes its approved image/index and the outer statement. Solving for new hash preimages that implement all counterfactual challenge assignments is neither supplied by AFK nor Akita and may be computationally infeasible. A collision-resistant or preimage-resistant hash alone does not imply the Fiat–Shamir knowledge/soundness property needed here.

Thus the simplest realizable theorem route is:

- **Assumption CH-I:** the exact concrete-hash native archive verifier, with the exact approved setup generation and auxiliary-input class, has computational soundness I at the derived extractor runtime; add concrete native knowledge if Theorem B is desired.
- **Outer theorem/assumption CH-O:** the exact outer Jolt/Akita execution relation has O for this fixed guest, with its full frontend/PCS composition, same-instance extraction, guest adequacy, setup and error policy. If outer FS is also instantiated concretely, its knowledge security is an additional concrete-hash/composable-knowledge premise, not supplied by the outer PCS ROM theorem.

More explicitly, a sufficient **fixed-concrete-hash auxiliary-input knowledge assumption** is the following. Fix the actual public algorithms h_i, h_o and setup expansion before the experiment; their code is available to the adversary. For every permitted efficient auxiliary-input generator Z and receipt adversary A, there exists a specified admissible extractor X_o such that, over approved setup generation, `z <- Z(pp_i,pp_o)` and the original run `(x,π_o,aux;hdl) <- A(pp_i,pp_o,z)`,

    Pr[RecV(x,π_o)=1 and
       (a <- X_o(pp_i,pp_o,z,x,π_o,aux;hdl);
        a=⊥ or ArchiveAccept^{h_i}(P,pp_i,x,a)=0)] <= β_o(A,Z).

The extractor has the allowed access to that original execution, preserves its x and auxiliary output, and has an explicit expected-runtime bound M_o(A,Z) or deterministic cutoff bound. It may use whatever extraction access the assumed outer scheme actually supplies (with any extraction-key/setup simulation distribution proved compatible with the inner experiment); it **cannot program h_i or h_o**, change the guest image, or substitute a setup. CH-I must cover the resulting archive-producing algorithm, including any admissible auxiliary information that this extraction procedure receives. A reduction using unavailable setup trapdoors or an altered setup distribution does not meet the assumption. This is a concrete same-execution knowledge hypothesis, not the weaker assertion that the outer verifier recognizes the correct language, and not an implication of AFK or Akita's ROM theorems.

One may keep H_o ideal for a clean **hybrid** theorem: concrete h_i is ordinary guest code, outer extraction is a classical-ROM claim for a fixed oracle-free guest relation, and CH-I supplies inner security. This is a meaningful conditional theorem; it is not a classical-ROM proof of the entire deployed concrete system.

### 7.2 Oracle-aware alternative: valid formal contract, missing implementation/theorem

A different mathematical construction could prove an oracle-relative relation

    R_wrap^{H_i}(x;a,τ): τ correctly runs G^{H_i}(x;a) and outputs 1.

It needs an outer proof system which authentically constrains oracle gates and whose extractor is sound for this **oracle-dependent relation**, while preserving H_i. None of the cited standard NP-relation/PCS theorems establishes that extension for the proposed ordinary Jolt guest.

Supplying `(query,answer)` pairs as private advice is insufficient: a polynomial witness can invent answers unless they are authenticated to the actual H_i. Having the external verifier replay every query restores that authentication but may expose/read O(T) transcript work, contrary to the selected succinct public interface. Replacing H_i with a succinctly evaluable pseudorandom function or a committed oracle is a new cryptographic construction with new trust/security assumptions, not an ideal-oracle implementation for free.

Even in this hypothetical oracle-aware construction, O must satisfy all of the following:

1. The relation is fixed relative to the original H_i, including verifier-only queries made on the extracted a; extraction cannot validate a against its own modified inner oracle.
2. The outer extractor may fork/program H_o, but never program, reset to a different table, or discard previously fixed H_i answers. Newly queried H_i points are answered consistently by the original external oracle.
3. Auxiliary input/output and original H_i query history are coupled with the original receipt experiment; equality of `(x,π_o)` marginals alone is insufficient for later verification.
4. The native reduction receives the entire derived B as a resettable algorithm. Inner extraction may now fork H_i **at its own legitimate native challenge points** and rerun B, including all outer extraction, in each counterfactual environment. Within each such invocation, B's outer extractor still treats that supplied H_i as immutable.
5. An independently simulated H_o can be included in B's internal random state/local oracle simulator. Its programming is confined to the outer namespace. All conditional simulation, EPT closure, query counts and costs must be proved, not inferred from separate theorem labels.

There is no contradiction between items 2 and 4: item 2 controls the *outer* extractor within one inner environment; item 4 describes the *inner* extractor exploring different environments while replaying the entire outer procedure. But the resettable simulation that implements both is additional composition work. Merely freezing an already extracted archive gives no counterfactual continuations.

### 7.3 Setup expansion is part of the same model boundary

The inner registry's `<2^-361` figure bounds coefficient-sampling bias **when the expansion bytes have the assumed ideal distribution**. A finite public-seed concrete XOF is not statistically close to the uniform distribution over an enormous flat matrix vector merely because each coefficient is obtained from 512-bit output. The paper's setup hybrid programs the setup namespace consistently with a sampled flat vector (Theorem 10.33, proof Step 1).

If the guest recomputes concrete setup expansion from a seed, it cannot in general reproduce an arbitrary challenge-embedded flat matrix supplied by the reduction. Either keep the ideal setup interface in the formal model, assume the required concrete setup-generation/matrix hardness property, or use a separately specified honestly generated public setup and bind its actual matrices as global preprocessing. Any digest that authenticates externally supplied setup pages needs a binding argument; a proof of correct setup expansion/offloading needs its own admitted relation and matrix/PCS ledger. None of these issues changes the intended RAM relation or authorizes Merkle RAM.

The selected static-blob route avoids that **inner setup-XOF-in-the-guest** obstruction: independently sample the 33 actual matrices with uniform 512-bit modular sampling outside the guest, then authenticate their exact canonical encoding through C_static. In this mode `<2^-361` is the actual aggregate modular-sampling bias under genuine independent samples. A CSPRNG implementation adds its own distributional premise; mere correct PRG execution is not statistical uniformity. For an inner matrix embedding, the reduction constructs the static blob and its original outer commitment from the embedded public matrix, using the independently generated outer key. For an outer matrix embedding, it constructs C_static under that public outer map from the known approved static source. Both directions require the ordinary setup/source-comparison embedding certificates but no fixed-point dependence of keys on C_static. All proof-dependent weights, table evaluations, matrix contractions and final equations remain guest work. This setup choice does **not** resolve the separate concrete-inner-FS-hash or outer knowledge-composition obligations.

## 8. Domain separation, cycles, and fixed indexing

Use length-delimited canonical tags for inner FS, outer FS, inner setup, outer setup, and identity binding, including scheme/version, profile, source role, absolute level, stage, coordinate/limb and sampler retry identifiers. Retain each inner challenge's exact prechallenge-message binding. Do not hash a completed transcript before deriving its own internal challenges. Domain separation prevents accidental equality of query inputs and, **in the ideal model**, permits independent oracle simulations. It does not prove concrete-hash security, resolve oracle-dependent relations, or prevent extraction-induced query amplification.

There is no need for a curve cycle, a self-referential verification key, or a proof of the outer proof inside this guest. Adopt the acyclic dependency order:

    inner application/profile/setup → fixed inner verifier guest → outer preprocessing/key → receipt.

The guest can bind the inner profile and actual inner setup without embedding its own outer verification key. The external verifier binds the guest image/index and outer key. If a unified receipt profile ID contains the outer key, keep that ID external or separate the inner guest index from the outer envelope; do not require the guest binary to contain a hash of itself or its own yet-to-be-generated key.

The archive and its inner challenges do not depend on π_o. Receipt output does not include a hash of π_o that the guest must compute. The outer system may use the same arithmetic or hash *family* as the inner system while keeping independent roles/namespaces; this is reuse of an algorithm, not recursion. The guest's trace grows with archive verification work and must fit its fixed bound. Succinct public communication does not mean constant proving work or unchanged setup geometry.

## 9. Complete setup/matrix additions required for admission

The inner inventory remains exactly the approved 33 roles: six A, five B, four D and eighteen F/H compression matrices. No new inner matrix is needed merely to hide the archive as advice; no inner per-segment completed proofs are added.

The outer execution is a **new full proof instance**, and its complete registry I_o must be added. It includes, where used by the selected outer schedule:

| New outer obligation | Required inventory/certificate |
| --- | --- |
| Guest main trace and every auxiliary/advice source | Original A/B/F roles, exact source alphabet/decoder/padding, column/plane order, root source geometry and original-handle query map |
| Selected installed C_static source | Same-original-handle advice extraction, exact byte length/word decoder, complete A/B/F path and known canonical reference; mixed weak/reference collision bounds and setup-embedding compatibility |
| Outer opening batches | Every D/H role and full partial/carrier geometry; multiple points and sources remain distinct until their admitted reductions |
| Outer recursive PCS successors | Actual per-level A/B/F and D/H maps, source/range/relation rows, response bases/depths, mixed weak/strong comparison radii, and successor capacities |
| Outer final terminal | Actual terminal A role and bounds, correct inner-mode creation, and complete terminal equations; no invented B/F omission |
| Any outer setup offloading | Registered setup-source commitments, source matrix views/prefixes and their embedding/comparison classes, immediate-successor consume/produce rules, and final empty obligation ledger |
| Optional program or private-advice commitments | Every root/query/normalization obligation required by that outer source generation; private advice is not an unauthenticated hint channel |
| Program/setup/I/O identifier digests | Separate binding assumptions/reductions, unless exact trusted preprocessing eliminates the digest substitution point |

For **every** new J, fix modulus, ring degree, rank, width, source map, alphabet, all collision radii (including mixed weak/strong comparisons), sampling law, setup namespace and shared-view embedding. Independence is the default: provision outer roles separately from inner ones. Reusing the same field or hash does not justify reusing a matrix. Deliberate matrix sharing requires a joint view certificate covering cross-layer comparisons and adaptive choices; two standalone inventories cannot be summed unchanged.

If the outer coefficient count is N_o and uniform w_o-bit modular samples are justified in the chosen setup model, its additional sampling bias is bounded by `Δ_o <= N_o q_o 2^(-w_o)`. Additional concrete expansion assumptions are separate. The total role count is `33 + |I_o|` for independent registries, **not 33**. Neither |I_o| nor its numerical tuples can honestly be specified before the bounded guest trace/private-advice geometry and complete outer schedule are selected. This is an exact missing prerequisite for numerical admission, not permission to treat those terms as zero.

The direct fixed-guest route requires only one admitted ordinary outer schedule. It does not require a receiver fixed point for that outer layer. Akita's asymptotic succinctness result for regular families (Theorem 9.3) is not a concrete proof-size or verification-cost certificate for an unspecified guest schedule.

## 10. Established lemmas, added assumptions, and decisive remaining obligations

| Item | Status in this note |
| --- | --- |
| Pre-PCS target, original-source semantics, finite native archive and final decider | Previously reviewed conditional construction, preserved verbatim in scope |
| Ordinary outer language soundness is insufficient as a generic composition premise | Established quantifier obstruction and generic separation, §4; no claimed Akita exploit |
| One outer archive extractor + inner language soundness implies receipt language soundness | Derived Theorem A, explicit cutoffs/query/runtime ledger and proof, §5.3 |
| Whole-run knowledge via nested extraction | Conditional Theorem B; needs exact native/outer same-run and reset/EPT interfaces |
| Native Akita classical-ROM batch extraction and runtime formula | Paper Theorems 10.31/10.33, subject to their schedule/setup hypotheses; finite archive uses its reviewed extension |
| AFK adaptive auxiliary-output extraction and Q+1 loss | Paper Theorem 4 / Definition 10; not a concrete-hash or oracle-gate theorem |
| BCMS/BCLMS generic compiler closes this guest automatically | False: their cited PCD compilers explicitly exclude ROM; no application claimed |
| Concrete inner hash and concrete setup expansion preserve the native proof | Added CH-I/setup premise or new proof required; not established by collision resistance or ROM evidence |
| Exact outer Jolt/Akita archive extraction and guest adequacy | Unresolved instantiation/proof prerequisite; PCS knowledge alone does not establish it |
| Independent-oracle wrapper theorem for ordinary concrete-hash guest | Not applicable: guest computes h_i, not H_i |
| New outer schedule, matrix registry, statistical policy and derived q_B cap | Required numerical/admission certificates; not supplied by the inner 33-role certificate |

**Decisive obligations before claiming a sound implemented succinct receipt:**

1. Choose the security claim explicitly: (a) concrete inner verifier with stated CH-I and setup assumptions plus a proved/assumed outer archive-extraction contract, or (b) a genuinely oracle-aware outer construction and its preserving-oracle theorem. The current sources alone do not establish either complete instantiation. Do not label the concrete guest “proved in ROM” by identifying h_i with H_i.

   **Approval boundary:** selecting a succinct receipt does not itself approve CH-I, CH-O, or a new setup-instantiation hypothesis. If a new proof does not discharge them, adopting them changes the reviewed assumption set and requires explicit user approval of that qualified security claim. No consulted supplied paper closes this for the specific fixed-hash Akita/Jolt wrapper. Without that approval or a suitable new theorem, the correct status is a conditional design with this exact security-model prerequisite, not an admitted implementation. A derived query count above the inner Q32 cap gives no right to quote the old bound; if the only justified cutoff has a large or vacuous δ_cut, report that fact rather than presenting (1) as a useful numerical security guarantee.
2. Prove the exact outer frontend/PCS-to-execution-witness extraction and guest-adequacy projection, including private advice, byte parsing, concrete hash/setup behavior, and exact success=1. State auxiliary-input/output and same-execution guarantees.
3. Instantiate the guest's advice/trace bound and outer schedule; generate the full added matrix/setup/error inventory. Preserve the selected statistical policy for the additional stage rather than inheriting the inner E label or accepting a base-field downgrade.
4. Bound the derived archive producer's work and **inner** queries across outer extraction. Preserve the native `q_B+1<=2^32` policy or obtain a separately reviewed replacement; include every cutoff loss and every nested-reduction runtime in its proper term.
5. Retain the exact original-handle pre-PCS archive grammar, endpoint links, once-only initial/final checks and full inside-receipt decider. A wrapper cannot repair omission of any of these native checks.

The note therefore provides a complete conditional composition proof and an evidence-backed exact obstruction to stronger claims. It does not postpone the fundamental model issue behind the phrase “apply SNARK compression.”

### 10.1 Later structured-oracle research

The full [CCGOS accepted manuscript](https://wrap.warwick.ac.uk/id/eprint/176570/1/WRAP-proof-carrying-data-arithmetized-random-oracles-Spooner-2023.pdf) is now available. [Its dedicated analysis](AKITA_INSTANCE_ACCUMULATION_ARITHMETIZED_ORACLE.md) identifies a genuine conditional one-shot AROM route: an oracle-free guest skeleton checks a private query ledger/reduction proof; the outer statement binds a public accumulator; the external verifier checks two extended-oracle evaluations. Lemma 9.5's basic compiler is generic, but the available ROM-to-AROM knowledge lift in Lemma 9.1 requires straightline extraction, absent from the reviewed Akita extractor. A new suitable lattice extraction theorem, complete query-accumulator admission and declared software-instantiation model are still needed.

The [oracle-limits analysis](AKITA_INSTANCE_ACCUMULATION_ORACLE_LIMITS.md) does not conclude that this fixed receipt is impossible. It records the actual class-level scope and the full-paper retrieval limits of BCG/CCS22. [Independent reviewers](AKITA_INSTANCE_ACCUMULATION_RECEIPT_REVIEW.md) approved both conditional analyses without closing the instantiation prerequisites above.

The subsequent [product extraction](AKITA_INSTANCE_ACCUMULATION_PRODUCT_EXTRACTION.md) and [source-first theorem](AKITA_INSTANCE_ACCUMULATION_SOURCE_FIRST_EXTRACTION.md) handle independent H_o with actual fixed Theta. Their ordinary-SIS route uses reviewed input-local paired emulation. [Hidden-curve transport](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md), [private continuation](AKITA_INSTANCE_ACCUMULATION_PRIVATE_ARO_CONTINUATION.md) and [native source-first extraction](AKITA_INSTANCE_ACCUMULATION_NATIVE_SOURCE_FIRST.md) now have independent conditional approval. They do not establish a concrete arithmetization, complete guest certificate or numerical outer profile.

## 11. Source and activity receipt

Primary sources actually consulted:

- [Local Akita paper](SOURCE_PROVENANCE.md#akita), Definition 3.15, Lemma 10.2 and following sampler discussion, Theorems 10.31/10.33, Remark 10.32, Eqs. (227)–(229); live qmd search and `get` retrieved the indexed Akita theorem (docid `#bfa1a6`, lines 4195–4231). Theorem numbering refers to this local snapshot, not an inferred online edition.
- [AFK primary CWI PDF](https://ir.cwi.nl/pub/33324/33324.pdf), [local copy](SOURCE_PROVENANCE.md#supplemental-primary-sources), J. Cryptology 2023, DOI [10.1007/s00145-023-09478-y](https://doi.org/10.1007/s00145-023-09478-y), §§2.4–2.6 and 6.1–6.3, Definition 10, Remark 5, Proposition 2 and Theorems 2–4.
- [BCMS published primary PDF](https://link.springer.com/content/pdf/10.1007/978-3-030-64378-2_1.pdf), [local copy](SOURCE_PROVENANCE.md#supplemental-primary-sources), §1.1, Theorems 1–2 and their model qualifications; §2.4 domain separation. No formal extended-version theorem number is attributed to this 18-page chapter.
- [BCLMS published primary PDF](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_24.pdf), [local copy](SOURCE_PROVENANCE.md#supplemental-primary-sources), §1.1, Theorem 1, Remark 1, §2.2 and Remark 2. No full-version security game is claimed inspected.
- The linked current design/review/compact/global/segment/matrix notes identify the exact target and prior conditional admission. Older SHA-256/Merkle/halt/residual predicates are not imported.

Activity was document retrieval and mathematical analysis only. No repository source edit, build, test, formatter, benchmark, proof execution or Git operation was performed. This new note is the assigned deliverable; implementation verification is outside this research assignment.
