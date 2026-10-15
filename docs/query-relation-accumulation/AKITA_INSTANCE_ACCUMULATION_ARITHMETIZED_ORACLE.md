# Arithmetized random oracles for one final native Akita receipt

**Historical scope:** this is the earlier same-ARO source study leading to the [ideal supplied-answer receipt](AKITA_INSTANCE_ACCUMULATION_RECEIPT.md). The accepted-manuscript audit refuted unrestricted emulation; paired input-local emulation and the weaker hidden-curve theorem are separately reviewed replacements. The later user-authorized [D1 prototype](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) instead computes a fixed public inner transcript, removes graph/quotient machinery and discloses restricted CH-I content. Historical source claims below do not override these corrections or constitute concrete security.

## 1. Decision, source, and invariant

**Result: a genuine ideal-model route, but not a theorem for the unchanged concrete Jolt guest or unchanged Akita extractor.** Chen–Chiesa–Gur–O'Connor–Spooner (CCGOS), *Proof-Carrying Data From Arithmetized Random Oracles*, EUROCRYPT 2023, supplies a concrete oracle interface, a nontrivial emulator, and an oracle-query accumulation compiler. Its generic compiler does not intrinsically demand a Merkle PCS or a homomorphic commitment. However, its proved ROM-to-AROM **knowledge** lift requires straightline extraction, which the reviewed Akita conditional-programming/mixed-tree extractor does not supply. Its software instantiation is explicitly a modeling proposal, not a reduction from collision resistance establishing security of concrete SHAKE/Blake arithmetizations.

The useful conclusion is narrower and more precise than either “ROM recursion is impossible” or “AROM solves our wrapper”: one may seek an **AROM-secure lattice outer knowledge argument plus the paper's separately admitted oracle-query reduction**, retaining the native archive. Neither component is presently established for our ordinary Jolt/Akita receipt. Replacing the outer proof by the paper's Micali/Merkle construction would demonstrate existence of a different system, not satisfy the lattice-only assignment.

Primary source actually read: [Warwick WRAP accepted full manuscript](https://wrap.warwick.ac.uk/id/eprint/176570/1/WRAP-proof-carrying-data-arithmetized-random-oracles-Spooner-2023.pdf), available locally as [PDF](SOURCE_PROVENANCE.md#supplemental-primary-sources) and [extracted text](SOURCE_PROVENANCE.md#supplemental-primary-sources). It has 57 PDF pages including the repository cover; citations below use its printed paper pages and theorem numbering. Metadata: [DOI 10.1007/978-3-031-30617-4_13](https://doi.org/10.1007/978-3-031-30617-4_13), [ePrint 2023/587](https://eprint.iacr.org/2023/587), [repository record](https://wrap.warwick.ac.uk/id/eprint/176570/). The full manuscript, not merely the abstract, grounds this note. The Springer PDF endpoint returned an HTML access page and direct ePrint downloads returned 403; the retained PDF is the successful Warwick author-manuscript copy.

The invariant is the existing [native design](AKITA_INSTANCE_ACCUMULATION_DESIGN.md): original K-valued source tables and original commitment handles; scalar challenges in `E=K[U]/(U²−5)`; exactly 33 registered inner matrix roles; at most 16 segments; native pre-PCS update prefixes and exactly one complete final decider. No completed per-segment PCS proofs, Merkle RAM, replacement source commitment, or Pedersen-style homomorphism is introduced. Actual approved matrices/program/static data remain provisioned outside the guest and authenticated through the independently keyed original outer `C_static`, as in [receipt setup](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SETUP.md). The present note analyzes a possible *additional outer oracle-authentication layer*, not a replacement of native instance accumulation.

## 2. Exact ideal functionality

Write `λ_A` for the paper's security/output-length parameter, to avoid conflating it with an Akita profile parameter. Fix a field F, input arity m, witness length w, individual degree d≥2, and a polynomial-size oracle circuit B with r random bits and w output bits.

**Definition 4.1, printed p.23**, samples a triple Θ=(ro,wo,V̂):

1. `ro:{0,1}^m→{0,1}^{λ_A}` is a uniformly random function.
2. For each input x independently sample μ_x∈{0,1}^r once, and set `wo(x)=B^ro(x,μ_x)`. Repeated queries return the same answer. B is fixed before oracle sampling, may depend on ro through polynomially many queries, and is not an arbitrary unbounded witness function.
3. On Boolean inputs define

   `v(x,y,z)=1 ⇔ ro(x)=y ∧ wo(x)=z`.

4. V̂ is a uniformly random polynomial over F of **individual degree at most d**, subject to agreeing with v on `{0,1}^{m+λ_A+w}`. Its query domain is the full `F^{m+λ_A+w}`, not just the Boolean cube.

Definitions 4.2–4.3 quantify over the corresponding families of fields and polynomial-size B. The individual-degree condition is not multilinearity and is not a degree bound only on the hash's Boolean restriction. Let `a=m+λ_A+w`; the total degree can be a·d.

Two important distinctions follow.

* The graph-verification oracle V̂ is **not** a low-degree extension of the hash output itself. It extends the Boolean predicate that checks a hash evaluation together with a particular witness.
* F is independent of ro's bit-string codomain (§2.2, pp.7–8). Choosing F does not change the inner K/E source semantics. Conversely, this independence does not license treating λ_A random bits as a uniform element of an arbitrary F without a specified conversion.

**Why d≥2 matters:** Remark 2.1, pp.8–9, explains that access to the multilinear extension of the graph predicate can permit hash inversion; allowing an adversarially selected higher-degree extension also admits bad examples. The theorem assumes a *random* extension. A fixed low-degree circuit with the correct Boolean truth table is not automatically distributed as that extension.

## 3. What “plausible software instantiation” actually means

Section 2.2, pp.6–9, proposes the following implementation model. Given a concrete hash circuit H, convert its computation to a bounded-width Boolean formula Φ_H with an efficiently computable internal-wire witness W_H, satisfying

`Φ_H(x,y,z)=1 ⇔ H(x)=y ∧ W_H(x)=z`.

Arithmetize that formula gate by gate to obtain a polynomial P_H. The graph-predicate arithmetization has total degree O(|H|), rather than the potentially enormous degree of directly arithmetizing the iterated hash computation. The candidate interfaces are `(H,W_H,P_H)` for `(ro,wo,V̂)`.

The paper explicitly says, p.7, that this still only supports a proof in an **idealized model**. Its modeling choices are:

* replace H by random ro;
* model circuit-dependent wire information using any allowed efficient B;
* replace the particular off-Boolean behavior of P_H by a uniformly random degree-d extension consistent with the Boolean graph.

These are not consequences of H being collision resistant. P_H has a concrete algebraic structure known from H's public code, whereas an ideal V̂ is sampled from an enormous affine space. Efficiently evaluating H and P_H does not establish that this structure is harmless for the relevant proof system. Remark 2.1 explicitly leaves weakening the random-extension modeling choice to future work.

**For SHAKE/Blake Jolt code:** proving that a fixed guest computes a hash correctly still proves an ordinary concrete relation. The guest does not thereby expose V̂ evaluation at non-Boolean field points, nor does its execution become a sample of Θ. Even if W_H is available as a hash execution trace, one must construct P_H, certify its exact individual degree and Boolean graph equivalence, and provide the extended-evaluation interface used by the query reduction. ARO software instantiation remains an additional security heuristic for the exact `(H,W_H,P_H)` family. This is more structured than the bare RO heuristic, but it is not a standard-model security theorem for the deployed hash.

## 4. Emulator and the scope of ROM security preservation

### 4.1 Pass-through emulation

Definition 5.2 (p.24) defines a stateful emulator that answers a specified subset of oracle interfaces while retaining the others. A **pass-through** emulator forwards a retained-oracle query to the original oracle and records its answer.

**Theorem 5.4 / Construction 5.11, pp.24–34, originally claimed** statistical error `q_Vhat/2^lambda_A` for arbitrary ro-oracle B. The later audit gives a counterexample to that guarantee under the stated exposed-transcript convention. The independently reviewed replacement requires input-local B and internally pairs both RO- and WO-triggered heads:

$$\varepsilon_{\rm emu}\le\min\{1,q_{\widehat V}/2^{\lambda_A}\}.$$

It is an adaptive **complete-run** bound, not a uniform arbitrary-prefix conditional guarantee. Retained implementation uses at most h RO and h WO calls; ro-only implementation uses one underlying RO request per new head. Complete polynomial CPU and feasibility handling are in the replacement note, not a unit-cost oracle assumption.

The actual mechanism matters for extraction. From recorded `(ro(x),wo(x))` pairs, form the sparse Boolean graph v_tr and its multilinear interpolation P_tr. Subtract P_tr from previously recorded V̂ answers; sample a random degree-d extension of zero on the Boolean cube consistent with these residual answers; add P_tr back. Thus the emulator's polynomial distribution is conditioned on the **known sparse graph plus recorded extended evaluations**, not computed by finding a preimage of a concrete hash seed.

The paper realizes this conditional sampling using:

* low-degree evaluation-code constraint detection (Theorem 3.11);
* exact conditional `LDSample` (Lemma 5.15 / Construction 5.16);
* a combinatorial-nullstellensatz decomposition `Ẑ(X)=Σ_j X_j(X_j−1)R_j(X)`;
* conditional sampling of the R_j through linear constraints (Lemmas 5.17, 5.21 / Constructions 5.18, 5.20).

Section 5.4 explicitly includes Gaussian elimination on a system with up to a·q variables/constraints, giving an `O(a³q³)` elimination term for that subroutine. These are *reduction/simulation* costs, not a claim that the honest guest should run the emulator.

**Lemma 6.1** still gives perfect joint RO/WO simulation by caching each input's independent coins and B evaluation. Composing it with Vhat emulation requires a *valid* emulator. For the repaired input-local profile, use one underlying RO fetch per paired head, evaluate B locally against that cached value and charge its CPU. This narrower proof replaces the former unrestricted Theorem 5.4/Corollary 6.4 citation; no extra witness bits or knowledge assumption is introduced.

### 4.2 Game preservation is not an automatic knowledge theorem

The source's **Theorem 6.5** reduces a bounded ARO producer followed by a ro-only predicate to a ro-only computation. Its cited unrestricted emulator is not available as stated. The repaired input-local theorem proves the needed narrower game-preservation result: run the complete producer/predicate through one paired emulator and charge all its Vhat queries and CPU, with the stated complete-run statistical loss.

This is genuinely useful for the native archive's **language-soundness** event: its verification consumes ro-derived challenges, not V̂ answers, and its target whole-run language is oracle-independent. A game comparison can preserve the output and its ro-verification environment, then apply the native ROM soundness theorem to the emulated archive producer. For a concrete bound, include all auxiliary-input generation, B calls, final archive-verification queries, and emulator work in the derived adversary's query/runtime ledger. This observation does not supply outer witness extraction.

**Lemma 9.1, pp.48–49, has an explicit extra hypothesis:** the starting ROM SNARK has a deterministic polynomial-time **straightline extractor from the original proof and oracle transcript** (definition in §3.3, pp.15–16). It then retains straightline knowledge extraction in the AROM. The statement's error is

`κ(λ_A,q·poly(λ_A)) + (q+poly(λ_A,|x|))/2^{λ_A}`.

The proof's useful parameterized form is Eq.(14):

`δ ≤ κ(λ_A,q·q_M) + ε_ARO(q+q_p,λ_A)`.

This reduction turns extraction failure into an ro-only predicate precisely because the straightline extractor is an ordinary computation on the recorded transcript. The manuscript's opening proof paragraph contains a different coarse polynomial expression; Eq.(14) and actual emulator costs, rather than an optimistic informal polynomial, are the appropriate basis for admission.

Akita's reviewed extractor instead constructs counterfactual continuations and conditionally programs ideal coordinate streams. It is not the §3.3 straightline transcript extractor. Therefore **Lemma 9.1 cannot be cited as a black-box lift of Akita knowledge soundness**. A rewinding AROM lift for this particular extractor might be provable, but requires a new proof of coherent emulator states, programmed ro answers, corresponding wo values and extension constraints across all forks. It must preserve the receipt's original statement and external oracle view, exactly as [receipt soundness](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SOUNDNESS.md), §5 and §7.2, requires.

**Lemma 9.2, pp.49–52:** ROM zero knowledge lifts with error `ε_ZK+2q/2^{λ_A}` using transcript-initialized emulation and the original ROM simulator. It is a zero-knowledge result, not a replacement for the missing rewinding knowledge theorem. Nor does it assert that arbitrary programming after arbitrary off-Boolean observations is always feasible. Conditional extension spaces must be nonempty and all recorded information must be respected.

## 5. Oracle-query accumulation: actual machinery and costs

### 5.1 Interface and construction

Definition 3.6 (p.19) requires a query accumulator whose **accumulation verifier makes no oracle calls**; only its prover and final decider use the oracle. Section 8, Theorem 8.1 and Construction 8.2 (pp.42–47), construct it for Θ.

The additional commitment CM has `Setup,Commit`, is binding, and has a commitment encoding of at most m bits. Theorem 8.1 additionally assumes hiding for its zero-knowledge conclusion. CM's algorithms themselves are oracle-free (§3.6); they are not calls to the very ideal hash being accumulated. There is no additive/homomorphic operation on commitments and no polynomial evaluation-opening API.

For n new queries and ℓ prior accumulators, set `N=n+2ℓ+1` and `a=m+λ_A+w`:

1. Translate each ro or wo query into a V̂ graph check with answer 1, supplying the complementary wo or ro value as advice. Retain existing extended-oracle queries unchanged.
2. Add both pairs from each prior accumulator and one fresh random V̂ query-answer pair, giving N pairs `(u_j,ζ_j)` in `F^a×F`.
3. Interpolate a vector of univariate polynomials g with `g(b_j)=u_j`, `deg g<N`.
4. Compute the univariate polynomial `f=V̂∘g`; it has degree `<adN` and can be obtained by O(adN) extended-oracle evaluations and interpolation.
5. Commit to the interpolation data and f. Derive `β=ro(cm)` and obtain `wo(cm)`. The construction includes bounded resampling of commitment randomness when β hits the interpolation domain.
6. Output two pending extended-oracle equations:

   `V̂(cm,β,wo(cm))=1`, and `V̂(g(β))=f(β)`.

The accumulation proof contains the random extra pair, complementary query witnesses, the *whole polynomial f*, and commitment randomness. The oracle-free accumulation verifier reconstructs g, recomputes CM, checks all interpolation values and the claimed g(β),f(β). The final decider makes exactly two V̂ queries.

This is why neither unauthenticated hash-answer advice nor an ordinary NP proof alone suffices: authentication is discharged by **two genuine extended-oracle evaluations outside the compressed relation**, not by trusting the prover's answers.

### 5.2 Soundness and parameter costs

Theorem 8.1's displayed bound is

`ε_AS = O(sqrt(q·d·(n+ℓ)·a/|F| + ε_CM + ε_ARO(q,λ_A)))`.

The square root covers the commitment and emulator terms as well; this was checked against the PDF display. Section 8.3 derives a more explicit expression from the zero-finding game of Lemma 7.1, which itself uses the emulator plus a forking lemma. The theorem is an asymptotic expression, not an admitted concrete 128-bit estimate for this project. A concrete profile must reconcile the exact reduction/runtime factors in §7–§8, specify the challenge-to-field map, and allocate the commitment and emulator errors at their actual positions in the bound.

For example, **as a scale implication of the stated bound, not a parameter recommendation**, making a square-root term at most approximately `2^{-s}` asks for a radicand approximately `2^{-2s}`. One cannot inherit the native inner E soundness budget or set λ_A=s by analogy. If `q/2^{λ_A}` lies under that root, it alone suggests `λ_A` on the order of `2s+log₂q`, before constants and other terms.

There is a further concrete interface issue: Construction 8.2 says to interpret the λ_A-bit β as an element of F, while the root estimates use field-uniform sampling. For general F, particularly a chosen K or E representation, an exact mapping/sampler and its entropy/bias/abort accounting must be supplied; raw bit reinterpretation is not a proof of uniformity. The interpolation domain also needs enough distinct field elements, and evaluation-interpolation of f must respect its degree and field capacity. This note does not silently instantiate those details.

Section 8.5 gives the following meaningful costs:

* final accumulator: `2(a+1)` field elements, independent of n;
* private accumulation proof: O(adN+n·max(λ_A,w)+a) field elements plus commitment randomness;
* honest accumulation prover: interpolation, O(adN) V̂ evaluations to construct f, and commitment work;
* accumulation verifier: interpolation, a full CM recomputation over the committed data, evaluation of g and N+1 evaluations of f; **not** O(1) work;
* external decider: two V̂ evaluations. In software these are evaluations of P_H, not two ordinary calls to H.

The private proof/verifier work therefore scales with the full archive's number of hash queries, not merely its ≤16 segment count. A short public accumulator does not imply cheap proving or a small Jolt trace.

### 5.3 Compatibility with lattice-only requirements

The query accumulator itself needs neither Merkle authentication nor homomorphic commitments. A separately proved standard-model lattice message commitment could in principle instantiate CM. That is an interface observation, not an identification of CM with current Akita PCS:

* CM must bind the **entire canonical polynomial/interpolation message**, with all sizes fixed, under its exact message domain and randomness convention.
* If hiding is claimed, it needs an actual hiding construction; deterministic matrix images or a PCS binding theorem alone do not supply it.
* Its m-bit output, source bounds, collision norms, independent key distribution, runtime-sensitive hardness and maximum message length require their own admission.
* Akita's bounded-source commitment/extraction theorem does not automatically imply a general-purpose hiding commitment to arbitrary-length messages. For this finite receipt one can formulate a bounded-message version, but must prove that adaptation and fix capacity.

A standard-model CRH used only for this auxiliary CM is still a new commitment choice. Taking a hash-based PCS or Micali proof as the **outer proof** would be a different construction and is not proposed here. The 33 existing inner roles are unchanged; any auxiliary lattice CM and revised outer trace/source geometry get separate roles and setup, not unrecorded reuse of inner maps.

## 6. The composition theorem and a precise one-shot mapping

**Lemma 9.5, printed p.53** (restating CCS22 Lemma 8.2) says: given (i) a SNARK in oracle model O for an **oracle-free** relation R, and (ii) an O-query accumulator whose accumulation verifier is oracle-free, construct a SNARK in O for the oracle-relative relation R^O. Zero knowledge transfers when both components have it; straightline extraction transfers if the base SNARK has it.

The important distinction is that **the lemma's basic compiler is generic in the base SNARK**; it does not require straightline extraction just to obtain a one-shot oracle-relative SNARK. Straightline extraction is used by the paper to avoid unbounded-depth PCD extraction blowup. **Theorem 9.4** obtains arbitrary-depth PCD from standard-model CRH in the AROM with `d≥2`, `m≥2λ_A`, and superpolynomial |F|, specifically instantiating the base with the Micali SNARK (Corollary 9.3). The latter uses a Merkle tree. Its existence theorem is not evidence that ordinary Akita already meets the compiler premise.

For our single final receipt, the useful specialization is the following **conditional design**, not an implementation or new proved Akita theorem:

1. Freeze the existing approved native verifier and actual static provisioning. Replace its inner challenge evaluations in the *formal relation* by ro queries with precise bounded encodings/samplers.
2. The private witness contains the native archive a, its final native decider, execution trace τ, a complete ordered oracle-query ledger L, complementary graph witnesses, and one ARO accumulation proof π_AS for L, taking ℓ=0 if there is no need to carry an older oracle accumulator.
3. Prove an ordinary oracle-free relation R_skel which checks the guest computation using L as its supplied query answers, derives every ledger entry from the executed instruction/verification sequence, and checks `AS.V(L,acc,π_AS)=1`. It also enforces the original-source `C_static` linkage and canonical success output 1.
4. The public receipt packages `(π_outer,acc)` for compact X. The ordinary outer statement binds X, the approved index, `C_static`, success, and **the exact acc**. The external verifier verifies the full lattice outer proof and runs `AS.D^Θ(acc)`. Acc is a compact proof component, not a replacement application statement or a revealed archive.

This specifies the **ideal supplied-answer branch**, with original lattice-held quotient sources and a separately modeled outer oracle. In that branch public x_o includes the exact acc and both external arithmetization equations are required; hiding acc in the witness would leave supplied answers unauthenticated. The later [D1 direct prototype](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) is a different, expressly heuristic branch and does not execute this graph wrapper.

The mathematical soundness argument, **if all its premises are independently established**, is straightforward. Same-execution outer extraction yields the same X and acc together with `(a,τ,L,π_AS)` satisfying R_skel. Query-accumulation soundness plus the externally accepted decider makes all L entries authentic to the **same Θ** except at ε_AS. Guest adequacy then gives `ArchiveAccept^ro(X,a)=1`. Native ROM language soundness, lifted against ARO adversaries through pass-through emulation, excludes false whole-run statements. Schematically, with cutoffs and distinct errors:

`Pr[false receipt] ≤ β_outer,AROM + ε_static/bind + δ_cut + ε_AS(B_AS) + ε_native,AROM(B_native)`.

Here `B_AS` and `B_native` include the actual outer extractor and all projection/simulation work. If the available extractor supplies a success-factor rather than a same-execution failure-event bound, retain that factor as in the existing receipt theorem. The equation is a conditional reduction ledger, **not** a bound licensed by simply adding the two papers' advertised errors.

Only one outer extraction is needed; arbitrary-depth PCD, recursive outer verifier cycles and a straightline-native extractor are not necessary for this goal. This avoids overstating the obstruction. Nonetheless, the one outer extractor must actually be proved valid in the AROM and preserve the original Θ when producing a witness for the externally decided acc. The ordinary ROM Akita theorem does not establish that premise.

## 7. Exact remaining obligations

| Obligation | What is supplied | What remains for this receipt |
|---|---|---|
| Native semantics | Reviewed finite native archive, K sources, E challenges, 33 roles | Unchanged; retain every original-handle, history, final-decider and sampler check |
| Setup outside guest | Existing independent outer `C_static` plan | Actual source/reference binding and outer profile admission; AROM does not replace them |
| Native soundness against extra ARO access | Independently reviewed input-local paired game-preservation theorem | Exact bounded encoding, full derived query/runtime and initialization/predicate ledger |
| Base lattice knowledge | Reviewed actual-Theta product-oracle mixed-tree/source-first arguments | Full frontend/guest source projection, ordinary-SIS embedding with scoped emulation, and resources; not a same-ARO straightline lift |
| Oracle-relative guest proof | Conditional generic compiler structure, Lemma 9.5 | Exact executed-query coverage, original-source projection and public-decider linkage |
| Query authentication | Quotient/source-binding/MLE algebra and reviewed weaker hidden-curve transport | Complete selected source/profile/sampler/error/runtime admission; do not reuse the old bound |
| Software ARO | Candidate `(H,W_H,P_H)` of §2.2 | Exact circuit/witness/arithmetization implementation and a clearly declared instantiation heuristic or stronger concrete security assumption |
| Privacy | Lemma 9.2 plus hiding-CM query-accumulator theorem | Actual outer ZK and admitted hiding CM; private advice alone is not a ZK proof |

The product model separates actual-fixed-Theta extraction from bounded ordinary-SIS emulation. Use the reviewed paired/local contract for the latter. Query authentication uses the separately reviewed hidden-curve proof, whose explicit observations and coupling losses address the fact that implicit curve values can change on head activation. No result licenses programming a concrete SHAKE seed.

The outer profile remains independent. ARO field arithmetic and the potentially large private query proof may require new outer execution/advice capacities and therefore new matrices/weak-source envelopes. Using E as the new F is a design choice requiring explicit representation and soundness analysis, not a benefit inherited from the inner scalar extension. There is no established receipt size, proving speed, memory saving, or 128-bit assurance here.

## 8. Assurance boundary and hand-off

**Source-stated results, subject to correction:** ideal ARO distribution, emulator/game-preservation claims, straightline-knowledge and zero-knowledge lifts, query accumulation, and a Merkle/Micali PCD existence instantiation. The audit rejects the unrestricted emulator guarantee and its imported transport argument. The independently reviewed input-local paired emulator and weaker hidden-curve theorem are **derived replacements**, not author-issued corrections.

**Not established by that source:** ordinary Akita's AROM knowledge soundness, oracle-relative correctness of an unchanged Jolt SHAKE/Blake guest, standard-model security of fixed SHAKE/Blake arithmetizations, a lattice CM instantiation at our parameters, or an implementation/performance improvement.

**Current action:** retain this as ideal-model research with its corrected conditional lemmas. The user now permits an explicitly heuristic concrete-ARO prototype; the exact experimental model and any restricted concrete-native-soundness implication must be stated and reviewed separately. No permission or change of terminology makes a concrete hash an ideal ARO or supplies the missing guest/profile/resource certificate.

Research activity only: primary PDF retrieval, text extraction, theorem/definition and proof reading, and this note. No source implementation edits, builds, tests, benchmarks, formatters or Git operations were run. Parent review should check the proposed contracts against the selected outer extractor and independently rederive the exact concrete bounds before any profile admission.
