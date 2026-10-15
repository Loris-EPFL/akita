# Akita relation accumulation: generic architecture and source boundaries

## Status and scope

This note supplies relation-level orchestration patterns, not an Akita folding theorem or a selected protocol. The intended cut is **before full per-segment Akita PCS proofs**, followed by one independently verified final proof/certificate. A linear-size certificate is permitted. Nothing here establishes a lattice commitment homomorphism, transfers a curve/DLOG extractor to Akita, or repairs the existing native prototype's extraction gaps.

The useful lesson from Nova is “reduce relation instances, then decide/prove the resulting relation,” not “add commitment bytes.” A standalone final verifier must reconstruct the statement being certified; accepting a prover-maintained running object is not sufficient.

## 1. Sources and actual reading coverage

### Indexed primary papers

The local qmd collection was searched first using lexical queries without embedding-dependent reranking. The indexed documents were read through their extracted endings, including references and appendices. Citations below use stable qmd identifiers plus extracted line ranges; these are extraction coordinates, not PDF page numbers.

| ID / collection path | Actual title | Extracted coverage |
|---|---|---|
| `#319cb0`, `qmd://lattice-papers/nova.md` | **Nova: Recursive Zero-Knowledge Arguments from Folding Schemes**, Kothapalli, Setty, Tzialla | Lines **1–1276**, through Appendix H, Corollary 2. Main passes covered 1–181, 193–243, 245–301, 303–368, 401–523, 601–631, 637–747, 801–1224, 1231–1276; closing reads covered 182–193, 244–245, 302–303, 369–400, 524–636, 748–800, 1225–1232. |
| `#92c0d2`, `qmd://lattice-papers/supernova.md` | **HyperNova: Recursive arguments for customizable constraint systems**, Kothapalli, Setty; **not SuperNova** | Lines **1–3069**, including Appendices A–H and the deferred completeness/knowledge-soundness proofs. Main passes covered 1–65, 67–181, 181–205, 207–405, 401–600, 601–821, 821–1018, 1018–1035, 1039–1280, 1281–1520, 1521–1725, 1725–1895, 1897–2000, 2001–2200, 2201–2440, 2441–2680, 2681–2900, 2901–3069. Closing reads covered 66, 206, 1036–1039 and 1896. |
| `#9cbc77`, `qmd://lattice-papers/neutronnova.md` | **NeutronNova: Group-based folding done right**, Kothapalli, Setty | Lines **1–2040**, including Appendices A–F, the tree-extraction proofs, comparison to ProtoStar, round optimization and unrolled ZeroFold. Passes: 1–181, 181–415, 401–599, 600–819, 820–1039, 1040–1259, 1260–1479, 1480–1699, 1700–1919, 1920–2040. |

Coverage means the extracted text, not independent verification of every theorem. Embedded base64 figure payloads were not visually inspected; textual captions were read. OCR damages some formulas and labels (notably several NeutronNova appendix equations and HyperNova CycleFold equations), so the original PDF must arbitrate implementation-critical algebra. Long prose/formula lines were wrapped for reading rather than silently treated as complete when the rendering truncated them. No already-indexed paper was fetched again from the web.

### Other primary sources located/read

* **BCMS20:** Bünz, Chiesa, Mishra, Spooner, *Recursive Proof Composition from Accumulation Schemes*, published TCC 2020 chapter. Local `ResearchSources/BCMS-2020-Accumulation.pdf`, extracted lines **1–246** read, including the published chapter's references. [Official published PDF](https://link.springer.com/content/pdf/10.1007/978-3-030-64378-2_1.pdf); [extended ePrint 2020/499](https://eprint.iacr.org/2020/499). **The chapter repeatedly defers formal definitions/proofs to the full version; the extended version was located, not fully read.** Thus this note uses the chapter's explicitly informal definitions and theorem prerequisites, not a claimed complete audit of BCMS's full proof.
* **Halo Infinite:** Boneh, Drake, Fisch, Gabizon, *Halo Infinite: Proof-Carrying Data from Additive Polynomial Commitments*, CRYPTO 2021. Local `ResearchSources/Halo-Infinite-2021.pdf`, extracted lines **1–213** inspected, with overlap at 147–210. [Official PDF](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_23.pdf). This covers introduction, PCS/additivity, linear combinations, and the start of private aggregation. **The remainder, including full recursion/security proofs, was not fully read in this slice.** Original Halo was also located via primary-paper references: Bowe, Grigg, Hopwood, [ePrint 2019/1021](https://eprint.iacr.org/2019/1021); that original paper was not read here.
* **ProtoStar:** Bünz, Chen, *ProtoStar: Generic Efficient Accumulation/Folding for Special Sound Protocols*, [ePrint 2023/620](https://eprint.iacr.org/2023/620), ASIACRYPT 2023. The primary ePrint landing-page abstract was read; **the complete paper/appendices were not read**. Its abstract states a generic construction for `(2k−1)`-move special-sound protocols whose verifier checks `ℓ` degree-`d` equations, with an accumulation verifier using `k+2` EC multiplications and `k+d+O(1)` field/hash operations. This is a concrete prerequisite, not a theorem that arbitrary Akita transcripts meet special soundness.
* `ResearchSources/SOURCES.md:1–18` supplied provenance. Public Rust source reads below are separate implementation evidence, not substitutes for paper proofs.

## 2. Folding, accumulation and aggregation are different contracts

Let `R(pp,s,u,w)` be a relation with parameters `pp`, structure/index `s`, public instance `u`, and witness `w`. A structure is fixed/indexed separately from small online instances; it is not an unbound prover-supplied label.

### Folding / multi-folding

A folding protocol reduces a product of relations to another relation. Abstractly:

`R_run^m × R_fresh^n  →  R_run`.

The prover uses input instance–witness pairs and produces a new witness and transcript; the verifier uses only input instances and transcript to compute the output instance. Completeness preserves satisfiability. Knowledge soundness requires extraction of **all input witnesses** from an adversary that produces a satisfied output relation with sufficient probability. It is not merely a test that honest inputs combine correctly. The output is another unresolved relation instance, not automatically a proof of its own satisfiability. A final decider/argument is still needed. Nova Definitions 5–9 and Construction 1 make these distinctions explicit (`#319cb0:193–347`); HyperNova's Definition 7 permits different compatible incoming and running relations (`#92c0d2:275–325`).

Nova's particular output relation is **committed relaxed R1CS**. Naive linear combinations of ordinary R1CS witnesses fail because of quadratic cross terms and the constant coordinate. Its repair adds `u` and an error vector `E`; the prover commits to a cross-term vector `T` before the challenge. The folded error includes `E1 + r T + r² E2`, while the witness is `W1 + r W2`. The proof uses a binding, hiding, succinct, additively homomorphic commitment and interpolation/forking. These premises are substantive (`#319cb0:245–347,749–905`). Neither “both are vector commitments” nor “both are hash based” supplies them for Akita.

### Accumulation

BCMS's predicate formulation has `Φ(q)` and `(P_acc,V_acc,D)`. The key soundness implication is:

`V_acc(acc_old,q,acc_new,π_V)=1 ∧ D(acc_new)=1`

`⇒ Φ(q)=1 ∧ D(acc_old)=1`, except with negligible probability for efficiently generated adversarial inputs.

The old accumulator is not assumed honest; otherwise backward induction cannot establish history validity. Completeness likewise applies to every old accumulator accepted by the decider. In the useful succinct setting, accumulator size does not grow with history and the transition verifier is cheaper than evaluating all deferred predicates. A set/RSA membership accumulator is a different primitive. BCMS chapter §2.1–2.2, lines 113–143.

A predicate may be full proof verification, with `q=(x,π)`, or a deferred expensive subpredicate. In BCMS §2.3, a predicate-efficient argument verifier splits into cheap `V_pe` plus query set `Q` to a predicate `Φ°`; accumulation applies to those queries. **That decomposition still starts from argument proofs. It does not, by itself, eliminate per-segment PCS proof creation.** To meet the present goal, incoming objects must instead be pre-PCS relation instances/witnesses, or a separately justified partial-protocol relation.

Split accumulation exposes distinct input-instance/input-witness and accumulator-instance/accumulator-witness types. It overlaps architecturally with folding but has its own correctness and security definitions. Do not decide a scheme's theorem by whether an implementation names its object `Accumulator`.

### Proof aggregation and batching

Public proof aggregation consumes already-issued proofs and yields a smaller proof/certificate for their collection. It can be valuable but generally does not satisfy the goal of avoiding per-segment proofs. Batching may merely accelerate verification without shrinking retained evidence or support for combining old accumulators.

Terminology is not universal: Halo Infinite **private** PCS aggregation takes committed polynomial/evaluation claims and private openings, whereas **public** aggregation can combine existing noninteractive proofs. Its Definition 4.1 describes reducing many `(C_i,x_i,y_i)` claims to one `(C*,x*,y*)` followed by Eval. Therefore “all aggregation requires old proofs” would be false. The object and security implication must be specified explicitly. Halo Infinite §3–4; local lines 147–213. BCMS's helped-verification comparison explicitly lacks old-accumulator recursion (lines 145–149).

## 3. Three histories that must not be conflated

For a stream of Akita segments, maintain the following conceptual separation even if the eventual implementation shares buffers:

| History | Objects | What it establishes / does not establish |
|---|---|---|
| Instance history | Segment statements `u_i`; running instances `U_i`; structure/key IDs; counters; endpoints; public I/O | States the claims and their order. Commitment bytes or state digests are not witnesses. |
| Witness history | Segment advice/trace `w_i`; commitment randomness; norm-bounded openings; running witness `W_i`; auxiliary-reduction witnesses | Makes relations satisfiable. It may remain private prover state, be consumed into a proved relation, or appear in a deliberately non-ZK final certificate. |
| Proof/transcript history | Fold/accumulation messages `π_i`, challenges, partial-protocol messages, final certificate | Binds reductions and supplies verifier evidence. Honest transcript generation alone does not establish history knowledge or extraction. |

The verifier needs either all necessary reduction evidence, checked directly, or a sound argument/augmented relation that attests to it. Deleting old transcript messages is justified only by an actual recursive/accumulation theorem. The prover may discard old witnesses only when the proved reduction and implementation no longer need them; this is an engineering consequence, not the soundness definition.

Nova's uncompressed IVC proof contains a running pair `(U_i,W_i)` **and a fresh pair `(u_i,w_i)`**. It is not “just a commitment.” The newest augmented-step relation has not yet been absorbed in the same way as the running relation, so the final verifier checks both, or compression folds them before proving the final relation. Nova Construction 3–4, `#319cb0:349–523`. HyperNova generalizes the running pair to an indexed family with a program counter; its final checks cover all running pairs and the latest fresh pair, `#92c0d2:563–717,2390–2556`.

NeutronNova makes another important boundary visible: its running relation is a **product** `NSC × NSC_PC × ZC_PC`, not one naked CCS claim. A final proof must handle all factors. §5.3–5.4, `#9cbc77:749–781`. Dropping the power-check factor would discard a security obligation, not optimize a redundant check.

## 4. Augmented-step linkage: what is reusable

A stream of individually valid segments does not prove a single execution. The wrapper must enforce composition, ordering, program/structure identity, public I/O and the exact segment count. Generic orchestration should bind:

`context = (protocol/version, pp/key digest, relation/structure ID, workload ID, arity/schedule)`;

`statement_i = (context, i, z_0, z_i, running_state_i, public_IO_digest_i)`.

The encoding must be canonical and length-delimited. These fields are a proposed Akita architecture contract, not a claim that every cited protocol uses this exact tuple.

In a Nova-style compiler, an augmented function `F'`:

1. checks the prior fresh instance's public digest against the supplied prior state, index, initial/current endpoints and running instance;
2. enforces that this incoming instance really is in the **fresh**, unrelaxed relation;
3. executes the folding verifier to derive the next running instance;
4. executes the application transition `F` and binds the next state digest.

The fresh relation for the execution of `F'` contains the prior digest preimage and transition transcript as witness material. Hashing avoids growing public input; it does **not** remove the need to constrain the preimage or prove transition correctness. The base case is explicitly constrained, not an unrestricted “first frame” bypass. Nova Construction 3, `#319cb0:377–451`; HyperNova Construction 2, `#92c0d2:611–717`.

HyperNova's generic compiler requires compatible encodings, NP-completeness/representation properties, partial functions, monotonicity and default instances. Its appendix proves these for its own CCS relations (`#92c0d2:2290–2354`). An Akita adaptation must establish analogous properties for the selected relation; a frontend trait implementation alone is insufficient.

For arity 2/4/8 or larger, bind the ordered batch and its actual length into the statement and transcript. Use a shorter supported tail reduction, or explicitly satisfying padding whose segment count is excluded from the application statement. In particular, three segments must not silently become four application transitions. A tree schedule must constrain adjacent boundary states and left/right order; a permutation of valid leaves is not the requested execution. Nonuniform scheduling must bind the permitted selector/program counter. Multi-folding changes verifier work and extraction degree; it is not automatically a free binary-fold replacement.

## 5. Compatible relation-level orchestration, without invented lattice algebra

These are interfaces/orchestration choices for the parent protocol design, **not additional complete protocol candidates**.

### A. Reduce first, prove once

Define `R_fresh` as the exact pre-PCS Akita segment relation and `R_run` as its explicitly specified reduced/deferred relation. Prove a reduction

`(R_run)^m × (R_fresh)^n → R_run`

for the supported arities. The prover constructs relation witnesses, applies reductions, and invokes one final argument/certificate generator on the terminal relation. The final verifier replays or verifies every required link and independently decides/proves the terminal relation. No full segment PCS proof belongs in the incoming type. This most directly fits the approved goal, but **the Akita reduction theorem remains a prerequisite**.

NeutronNova's modular pattern is useful here: first prove a reduction of knowledge from the application's relation into the folding target, then compose it with a separately proved folding reduction. Its Corollary 1 requires an actual `R → ZC^m` RoK; it does not make any polynomial relation foldable by inspection (`#9cbc77:783–825`). Its sum-check/zero-check construction still assumes additive commitments and binding, and the output carries extra power-check relations. Those are not imported into Akita.

### B. Accumulate an explicitly chosen partial-protocol predicate

If Akita exposes a cheap deterministic verifier portion and an expensive deferred relation **before** full PCS answers, specify exactly which messages become incoming instances and which openings remain witnesses. Supply the BCMS backward soundness implication (or a split/RoK version) for adversarial old accumulators. The final decider must resolve every deferred obligation, including any norm, commitment-opening, evaluation and transcript-consistency checks. A scheme that accumulates complete Akita proof-verification queries only after producing every full segment proof misses this architecture target.

Halo Infinite is not a shortcut: it distinguishes fully additive from **bounded additive** PCS, explicitly mentioning lattice-based schemes among bounded cases (local lines 151–155). Even a correct bounded-linear-combination identity needs a valid bound budget, and its aggregation theorem's other premises still need proof. Merkle/hash commitments are not group homomorphisms.

### C. Direct linear final relation certificate when succinctness is not justified

A conservative non-ZK baseline may retain segment relation witnesses and all linkage/reduction evidence in **one final certificate**. The verifier checks every original relation and boundary directly, with no full PCS proof per segment. This can provide sound end-to-end certification without any claim of fixed-size accumulation or recursive extraction. If all witnesses are disclosed, it is intentionally not ZK. Its cost/size can be linear or worse and must be reported honestly. It is not a Nova theorem and should not be advertised as succinct IVC. If the final certificate instead replaces direct checks by reduced checks, their soundness still needs the selected reduction theorem.

This fallback separates an executable, independently checked certificate from an unjustified compression claim. It does not license presenting the existing native prototype as an admitted theorem.

## 6. Final compression and zero-knowledge boundaries

**Final certificate correctness and final succinctness are separate.** A complete design must say whether the verifier receives a running witness, direct linear relation evidence, a polynomial-evaluation argument, or a separate compression SNARK. Serializing a running object or hashing the history is not compression soundness.

* Nova's IVC layer is not itself a ZK proof: its proof includes relation witnesses. Its compression construction folds the terminal running/fresh pairs and proves knowledge of the resulting satisfying witness with a zkSNARK (`#319cb0:455–523`). The paper's Spartan variants add DLOG or SXDH and ROM assumptions (`#319cb0:609–629`). Those assumptions are not base Akita assumptions.
* HyperNova's NovaBlindFold randomizes with suitably sampled satisfying relaxed instances and proves the blinding/folding checks; its relevant security statements are honest-verifier ZK, followed by a Fiat–Shamir discussion. A random scalar alone is not this simulator. Its full ZK architecture has a separate blind circuit and auxiliary folded relation (`#92c0d2:721–977,1481–1725`). Do not copy its blind-instance distribution into a norm-bounded lattice relation without proving it is satisfying, distributed correctly and hides the needed information.
* NeutronNova §5.4 states that the terminal product relation can be proved using sum-check and polynomial evaluation arguments; ZK requires ZK variants or a justified NovaBlindFold-style randomization (`#9cbc77:775–781`). Merely using sum-check does not guarantee ZK.
* BCMS's accumulator ZK definition permits a simulator to sample a fake terminal accumulator without the original inputs; an auxiliary verification proof `π_V` is not required for later accumulation and need not be simulated in that application. This says nothing about revealing `π_V` and the full history to the same observer (chapter lines 139–143).

An optional external wrapper must identify its setup, arithmetic field, commitment, soundness/knowledge, hash/ROM and ZK assumptions. Curve-based or pairing-based wrappers do not preserve a claim of lattice-only/post-quantum security. The approved native target is classical ROM; no unproved concrete-hash knowledge/game-preservation axiom should be smuggled in to make a compiler apply.

BCMS's generic PCD theorem in the published chapter specifically asks for standard-(CRS)-model knowledge soundness and accumulation soundness, **without random oracles**, because a recursive circuit represents a verifier non-black-box (chapter line 121). Its concrete PCS accumulation discussion is separately in the ROM with domain separation (line 177). These statements cannot simply be spliced into a new ROM recursion theorem. Nova/HyperNova discuss Fiat–Shamir and concrete-hash heuristics; copying their architecture is not a proof of Akita recursive game preservation.

The indexed Nova and HyperNova recursive extractors also explicitly fix the iteration count `n` as a global constant/environment parameter; their black-box recursive extraction is not an automatically uniform polynomial-time theorem for arbitrary polynomially growing depth. Nova `#319cb0:181–191`; HyperNova `#92c0d2:2428–2458`; NeutronNova's recalled IVC definition `#9cbc77:1281–1289`. Any claimed 256-segment theorem must state its quantifiers and concrete loss, rather than citing “arbitrary depth” without qualification.

## 7. Public Rust APIs and immutable implementation evidence

These pins identify inspected source, not audited/safe dependencies or reproducible performance claims. No source was cloned, compiled or executed. Branch names should not be used as dependency pins. Pinning these repositories does not automatically pin their transitive dependencies/toolchain.

### Microsoft/Nova

Pin: **`01b1e2d662a847ed7029770ecbaf6687398899dd`**. [Pinned tree](https://github.com/microsoft/Nova/tree/01b1e2d662a847ed7029770ecbaf6687398899dd).

* [`src/traits/circuit.rs`](https://github.com/microsoft/Nova/blob/01b1e2d662a847ed7029770ecbaf6687398899dd/src/traits/circuit.rs): public `StepCircuit<F>` has `arity()` and `synthesize(cs,z) -> Result<Vec<AllocatedNum<F>>, SynthesisError>`. **Here `arity` is state-vector width, not folding fan-in.** Reuse the separation between application state transition and proof orchestration, not the field/group algebra.
* [`examples/hashchain.rs`](https://github.com/microsoft/Nova/blob/01b1e2d662a847ed7029770ecbaf6687398899dd/examples/hashchain.rs): observed call flow `PublicParams::setup`, `RecursiveSNARK::new`, repeated `prove_step`, `verify`, then separately `CompressedSNARK::setup/prove/verify`. It passes the expected step count and initial state to verification. This is a useful lifecycle and final-compression separation. The example chooses BN256/Grumpkin with HyperKZG/IPA and Spartan, so it is not a lattice-native implementation.
* [`src/lib.rs`](https://github.com/microsoft/Nova/blob/01b1e2d662a847ed7029770ecbaf6687398899dd/src/lib.rs) exposes `nova` and feature-gated experimental `neutron`. Module presence is not a claim that every NeutronNova theorem/feature has been reviewed in code.

### arkworks accumulation

Pin: **`4a680af486a061927e2e46f3d7bf8830a96752b7`**. [`src/lib.rs`](https://github.com/arkworks-rs/accumulation/blob/4a680af486a061927e2e46f3d7bf8830a96752b7/src/lib.rs).

The inspected public `AccumulationScheme<CF,S>` trait cleanly separates `InputInstance`, `InputWitness`, `AccumulatorInstance`, `AccumulatorWitness`, `Proof`, and prover/verifier/decider keys. Its methods are `setup`, `index`, `prove`, `verify`, `decide`; `prove` accepts iterators over fresh inputs and old accumulators, `verify` takes their instances, and `decide` receives the accumulator instance–witness pair. `AtomicAccumulationScheme` specializes both witness types to `()`.

Usable pattern: make the transition proof and final decider distinct APIs, with no accidental witness argument on the public transition verifier. `MakeZK` is an explicit mode, not evidence that every instantiation is ZK. The crate's named `ipa_pc_as`, `hp_as`, `r1cs_nark_as`, `trivial_pc_as` modules are implementations for their own predicates; none is an Akita adapter. Only the top-level trait/usage documentation and tree were inspected, not all scheme implementations or gadget proofs.

### Sonobe

The formerly `privacy-scaling-explorations/sonobe` URL resolves to `ethereum/sonobe`. Inspected pin: **`7f042351e73d38f99c05167ba41f1e282178b067`**. [Pinned tree](https://github.com/ethereum/sonobe/tree/7f042351e73d38f99c05167ba41f1e282178b067).

* [`crates/fs/src/definitions/algorithms.rs`](https://github.com/ethereum/sonobe/blob/7f042351e73d38f99c05167ba41f1e282178b067/crates/fs/src/definitions/algorithms.rs): `FoldingSchemeProver<const M,const N>::prove` receives `M` running witnesses/instances and `N` incoming witnesses/instances plus an explicit mutable transcript and RNG; verifier returns the folded instance. `FoldingSchemeDecider::decide_running/decide_incoming` invokes the relevant relation checks. This is an especially good **typed orchestration** pattern for mixed incoming/running relations and workload arity.
* [`crates/ivc/src/lib.rs`](https://github.com/ethereum/sonobe/blob/7f042351e73d38f99c05167ba41f1e282178b067/crates/ivc/src/lib.rs): `IVC` separates preprocessing, key generation, proof update and verification, with `i`, initial/current states and external inputs explicit. `IVCStatefulProver::prove_step` updates state only after a successful proof call. Keys/proofs are parameterized by `FCircuit`, preventing some cross-circuit misuse at the type level; cryptographic relation IDs still need binding.
* Its `Decider` compression trait in that source is explicitly marked **WIP**. It is an interface example, not a ready, reviewed Akita final-compression implementation. Older Sonobe API names/paths must not be assumed at this pin.

### ProtoStar Rust experiment

Pin: **`49e36fa6d992d97b6c42ab730f5066f38d2dbdfb`**, `geometryxyz/protostar`. [`halo2_proofs/src/protostar/accumulator.rs`](https://github.com/geometryxyz/protostar/blob/49e36fa6d992d97b6c42ab730f5066f38d2dbdfb/halo2_proofs/src/protostar/accumulator.rs).

The inspected public `Accumulator<C>` retains gate/lookup/compressed-verifier transcripts, challenges and an error scalar. `Accumulator::fold` forms a paired constraint/error polynomial, sends its quotient coefficients before sampling the interpolation challenge, and merges transcript components. `Accumulator::decide` recomputes commitments, compressed error, linear lookup constraints and beta-power checks. This is useful evidence for the “deferred relations still need a final decider” pattern.

**Critical limitation:** that source explicitly says **permutation and shuffle constraints are not verified** by `decide`. Its fold also contains assertions and ignores transcript scalar-write results. It must not be treated as a complete generic verifier, production-ready API, or a proof of a full Plonk/Akita relation. Source scope here was the accumulator file and repository metadata, not a full repository audit.

## 8. Akita obligations left explicit

Before any reduced native certificate can be called sound, the selected design must discharge:

1. **Exact relation and encoding:** public/secret components, shape/key identity, field/ring embedding, canonical bytes, segment count and endpoint linkage; equality of native and encoded checks; treatment of tails and default instances.
2. **A real reduction theorem:** adaptive malicious inputs, invalid old running states, commitments fixed before challenges, all cross/error terms and product-relation factors, supported arities and compatibility. State whether the claim is language soundness or extraction/knowledge soundness and why the latter is needed.
3. **Lattice algebra:** commitment/opening identities actually supplied by Akita; allowed scalar/ring operations; norms/noise after repeated combinations; rejection/range rules; module dimensions; challenge distributions. If the commitment is only bounded-additive, quantify the operation budget. No field interpolation or inverse-shortness transfer is assumed valid for short lattice witnesses.
4. **Final decision:** independent verification of the terminal relation and every link not already justified by a proved recursive relation. A supplied witness/tree is not an extractor. If extraction consumes accepting transcript trees, prove the required tree size, generation cost, interpolation and witness-shortness conclusions; do not inherit polynomial-size claims from unrelated group protocols.
5. **Transcript/security model:** full domain-separated absorption schedule with structure, parameters, ordered inputs, arity and all prover messages; exact classical-ROM Fiat–Shamir theorem/reduction and losses. A concrete hash implementation is an instantiation, not an extra admitted knowledge axiom.
6. **Recursion/compiler premises:** base case, incoming versus running relation checks, digest preimage linkage, compatibility and knowledge-soundness quantifiers. Avoid recursion entirely in a linear direct certificate when its theorem cannot yet be justified.
7. **ZK and wrapper boundary:** what the verifier learns, which witnesses/randomness are serialized, simulator assumptions, and whether the optional final argument introduces setup, DLOG/pairings, non-native arithmetic, additional knowledge assumptions or a different oracle model.

The parent can implement orchestration modeled on these APIs while keeping the lattice reduction as an explicit security gate. It should benchmark **real Akita workloads at 1, 2, 3 and 256 segments**, distinguish relation construction/reduction/final certificate generation and final verification costs, and report memory and certificate size. No performance parity, security admission or benchmark execution is claimed by this research slice.
