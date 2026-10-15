# Independent outer oracle: extraction theorem and its exact boundary

## 1. Result and invariant

**The independent-outer-oracle route works at the abstract mixed-tree/Akita-PCS extraction layer under the explicit product experiment below.** It does not require a new knowledge assumption or CCGOS Lemma 9.1. The proof has two distinct parts: a combinatorial extractor using the actual, globally fixed inner oracle; and a bounded standard-Module-SIS reduction which emulates that oracle over the entire extraction computation. Applying a ROM extractor to a prover with a rewound emulator is **not** the proof.

**This is not yet an end-to-end theorem for the proposed outer Jolt guest.** Akita Theorem 10.33 extracts algebraic PCS batch decommitments, not a guest execution, archive, query ledger, or AS proof. Section 9 gives the exact missing deterministic frontend/guest obligations. The conditional receipt corollary states these as mathematical lemmas to prove, not new unproved knowledge assumptions. No concrete hash, concrete arithmetization, numerical security profile, or software admission follows.

**Subsequent scope correction:** actual-fixed-Theta Theorem A is unchanged. Theorem B and downstream ARO-to-ro claims require the independently reviewed [input-local paired emulator](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md); the unrestricted guarantee is invalid under the inspected transcript convention. Query authentication now has the separately reviewed [hidden-curve theorem](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md), with weaker Q-outside-square-root loss and explicit source/budget premises. [Private continuation](AKITA_INSTANCE_ACCUMULATION_PRIVATE_ARO_CONTINUATION.md) supplies a distinct reviewed actual-versus-virtual query route. None supplies a concrete oracle, complete guest or numerical profile.

The invariant is the original application statement and static reference, original K-valued sources and 33 inner matrix roles, extension-field challenges, finiteT16, native pre-PCS instance updates, and one complete final native decider. This note changes none of them. No completed segment proofs, Merkle PCS, or A/B/F homomorphism is introduced.

The starting model is [the parent-owned product proposal](AKITA_INSTANCE_ACCUMULATION_MIXED_ORACLE.md). The same-oracle analysis in [the earlier ARO note](AKITA_INSTANCE_ACCUMULATION_ARITHMETIZED_ORACLE.md) remains applicable to its different experiment.

## 2. Primary-source contracts actually used

1. [Akita](SOURCE_PROVENANCE.md#akita), Definition 3.15 and its preceding paragraph: a resettable execution handle fixes the original auxiliary input, initial state, random tape and oracle transcript. The knowledge event concerns the original statement/proof. Replayed oracle answers agree except where the extractor's challenge programming changes them.
2. Akita Lemma 3.22: mixed flat-coordinate and nested-interpolation transcript trees; statistical tree failure at most `(Q+1) sum_r e_r`; expected-time recurrence retaining the **original query index even when extraction fails**. Its Fiat–Shamir inputs injectively encode the stage and complete preceding transcript. The source's proof explicitly distinguishes a cheap mismatching-index trial from an expensive matching continuation.
3. Akita Lemma 10.2 and the following paragraphs: exact conditioning of **full ideal answer streams**, including shell rejection encodings and field-vector encodings. This is not programming a SHAKE seed preimage. Equations (225)–(228) price tree size, conditional samplers and extraction work.
4. Akita Theorem 10.31: a deterministic accepting-tree algorithm returns an algebraic batch witness or a short nonzero kernel vector for an inventoried matrix view. Remark 10.32 rejects the stronger claim of recovering canonical short commitment states. Theorem 10.33, Eq. (229), combines tree failure, setup distance, and ordinary Module-SIS reductions. Lemma 10.11 embeds each matrix challenge into the **same uniform flat setup vector**; Corollary 10.12 handles a fixed public family of adaptively selected schedules.
5. [AFK extended version](SOURCE_PROVENANCE.md#supplemental-primary-sources), §§2.3–2.6, Definition 10, Remark 5, and §6.3/Theorem 4: adaptive extraction preserves the joint distribution of `(x,pi,aux,v)`. Their constructed extractor first performs the honest original execution and retains its output, then rewinds. Statements are included in challenge inputs. Definition 10 in isolation allows a success divisor `q(n)`; it is not automatically a same-execution failure bound with divisor one. AFK discusses witness-extended emulation in its related-work discussion; the formal noninteractive joint-output contract used here is Definition 10, not an invented separate “WEE theorem.”
6. [CCGOS accepted manuscript](SOURCE_PROVENANCE.md#supplemental-primary-sources), [text](SOURCE_PROVENANCE.md#supplemental-primary-sources), Definition 4.1 supplies the fixed RO/WO/extended-polynomial law. Lemma 6.1 supplies perfect joint RO/WO simulation. The [subsequent scope audit](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md) refutes the unrestricted Construction 5.11/Theorem 5.4 emulator guarantee and proves the narrower paired input-local replacement used below. Its complete-run error is at most `t_Vhat/2^lambda_A`, with augmented persistent state and corrected query/CPU costs. The same-ARO Lemma 9.1 separately requires straightline extraction.

Citations to Akita's mixed-tree theorem are not citations to AFK for flat/nested Akita geometry: that geometry is supplied by Akita's own Lemma 3.22. AFK supplies the adaptive-output interpretation and sampling framework. In particular, the simple AFK scalar-tree call count must not be silently substituted for Akita Eq. (227).

## 3. Exact experiment and hypotheses

Let `s` contain the public schemas, admitted outer schedule, setup/profile descriptions, guest index, and actual provisioned public keys. Let `z` be admitted initial auxiliary input. The clean experiment samples `(s,z)` by an efficient, oracle-independent sampler, then samples

`Theta=(ro,wo,Vhat) <- ARO[F,m,lambda_A,d,B_s]`, and independently `H <- H_outer`.

Here d>=2 and B is a fixed polynomial-size ro-oracle circuit. Actual-Theta Theorem A permits cross-input B; **Theorem B additionally requires input-local B**, whose output at x depends only on x, its private persistent coins and ro(x). The cleanest specialization is one fixed such B independent of s. Controlled public-key dependence is discussed in §7.

`H_outer` means the **classical independent ideal-coordinate-stream interface actually required by Akita**, including uniform field vectors and the full bitstream encodings used by exact samplers. Calling it an “ordinary RO” does not authorize replacing it by a single fixed-output 256-bit oracle followed by a concrete XOF. Such a different interface requires its own reduction.

The prover A may query both H and all of Theta, choose its statement adaptively, and output

`o0=(x0,pi0,aux0)`.

For a receipt, `x0` includes injectively encoded `(X,acc,approved index,original C_static,success1,profile/setup bindings)`. These fields are not hidden in an unauthenticated auxiliary string. The outer verifier uses H only; its target relation is oracle-free. The external `AS.D^Theta(acc)` is evaluated in the same Theta experiment. Recording that bit in aux0, or computing it from the saved acc before extraction, is allowed and costs its actual queries.

The following hypotheses are explicit:

**H1 — Fixed-function auxiliary environment.** Theta is sampled once and thereafter consists of fixed functions, not an evolving service that changes answers upon a replay. B cannot call H; no arithmetization interface constrains H. There is no additional nonresettable mutable environment whose responses depend on fork history. All ordinary prover/environment state required to replay is inside the resettable handle. Previously known oracle-dependent auxiliary data must be generated by a recorded efficient initialization phase, included in the oracle-query/work budget and resettable computation; it cannot be arbitrary advice correlated with unqueried H values.

**H2 — Exact outer transcript interface.** Stage inputs bind the original statement and required preceding transcript injectively. Coordinate substreams that Akita forks separately do not absorb one another's answers. Actual fresh H entries are independent ideal streams; repeated entries reuse their answer. Rejection, nonce search, and unqueried output challenge inputs are treated as in the cited extractor, not as free randomness. The original adversary has a uniform upper bound Q on its H-namespace queries, including initialization/auxiliary generation and response-dependent nonce search.

**H3 — Deterministic mixed-tree extraction.** The admitted oracle-free protocol has a deterministic algorithm D which, from every complete prefix-consistent accepting mixed tree rooted at the bound original statement, returns either a witness of the intended relation R or a named collision in the admitted matrix inventory I. Its stage errors, tree sizes and conditional samplers satisfy Akita Lemma 3.22. For R equal to Akita's batch-decommitment relation, this is supplied by Theorem 10.31. For R equal to the proposed outer skeleton relation, it is **not yet supplied**; see Section 9.

**H4 — Setup and collision law.** Every collision alternative is an ordinary Module-SIS instance with the exact admitted norm, source-comparison class and map dimensions. The ideal outer matrices are views of one uniform flat vector, and required registry/reference data is efficiently computable from it. The joint real setup experiment differs from this ideal setup experiment by at most Delta_setup, or the actually provisioned setup is exactly ideal and this term is zero. This is a hypothesis about the complete public setup and auxiliary-generation law, not a hardness estimate for a seed expander.

**H5 — Work model and cutoffs.** A has a fixed time/query bound per reset execution, the explicit instance length and tree product are polynomially bounded, and full conditional-stream samplers have the source's expected polynomial work bound. Mathematical extraction charges actual bit/field work plus an oracle-call cost for Theta. A strict cutoff L is imposed before obtaining an ordinary bounded-time MSIS adversary. Section 6 states the tail and emulator costs; polynomial expected work alone does not license substituting average query count into a worst-case emulator theorem.

These hypotheses are properties of an experiment/protocol or deterministic lemmas. The only computational assumptions in the extraction reduction are the stated ordinary Module-SIS assumptions. The later receipt composition additionally needs the independently admitted lattice CM and native soundness theorem, not a new knowledge assumption.

## 4. Product mixed-tree theorem with actual Theta

For stage r let

`e_r = sum_u (k_{r,u}-1)/|S_{r,u}|`,

and let `b_r` be `1+sum_u(k_{r,u}-1)` for a flat stage and `product_u k_{r,u}` for a nested stage. Write `eps_tree=(Q+1) sum_r e_r` and `K_tree=product_r b_r`.

**Theorem A (actual-oracle extraction).** Under H1–H3 and H5, there is an expected-polynomial-time resettable extractor E with actual access to the original Theta which never programs or resets Theta. It first retains the exact original output and verifier decision `(x0,pi0,aux0,v0)`, then programs only H. Its output is a witness of R for x0, a named matrix collision, or failure, and

`Pr[v0=1 and E outputs neither a valid R(x0) witness nor a valid named collision] <= eps_tree`.

The original tuple has exactly the original experiment's distribution, jointly with Theta and any original saved external-decider result. E need not return the witness originally used by A, or preserve auxiliary outputs of successful counterfactual forks. Its witness must satisfy R for **the saved x0**.

### Proof

**Step A1: fix the independent environment, not a success event.** Fix s, oracle-independent initial coins/input, A's random tape, and the complete sampled function tuple theta. Given this conditioning, the initial H table J remains a product of independent uniform answer streams. The fact that theta has enormous description complexity is harmless: the combinatorial sampling argument only needs the resulting next-message functions to be deterministic. E continues to call theta; it neither enumerates nor hardwires theta into an efficient reduction.

If auxiliary input was generated with H, do not condition on its output while asserting J is still uniform. Instead include that initialization in the deterministic oracle algorithm whose initial table is J. It is replayed with the same tape and its query counts are included. Likewise, conditioning on acceptance, a chosen adaptive statement, or a selected query index need not preserve fresh-answer uniformity. The weighted sampling theorem handles those selections; no such uniformity claim is made.

**Step A2: fresh entries and resets.** For an H input never queried on a given original execution prefix, no information obtained solely from fixed theta changes its marginal distribution before that first query. Prover computations may choose the input based on theta and previously answered H inputs; this does not reveal an unqueried independent table entry. Until the first query to a changed H entry i, changing that entry cannot change the deterministic execution: every ordinary step and every theta answer is unchanged. Rewinding the prover while retaining theta therefore restores the same next-message behavior for an unchanged H table.

The fixed oracle does not become different merely because another fork queried more of it. It is the same complete theta function throughout. “Nonresettable” here means fixed function answers, not a hidden call counter. For example, an independent oracle returning its incrementing invocation count violates H1 and destroys replay even if it never sees H. Independence alone is not sufficient for that different model.

**Step A3: original-index support.** For the original stage-r selection `I_r(J)`, use exactly Akita's original-index map, including its failure symbol. For any candidate table, determine its original execution's index/prefix before invoking the expensive suffix extractor. Each recursive procedure retains that original index on failure as well as success. Since changing only J_i cannot affect the original execution before querying i, the support bound in Lemma 3.22 still holds:

`P_{0,r} = sum_i Pr_{J_-i}[exists z: I_r(J_-i,z)=i] <= Q+1`.

The pointwise support consists of at most Q queried inputs and the possible unqueried output challenge input. Theta calls add work, not H indices, to this count. This proof works for every fixed theta; it does not assume independent behavior of successive forks. Replacing the original index by an index chosen on a later successful fork invalidates this bound and is forbidden.

**Step A4: exact challenge conditioning.** Apply Akita's conditional-answer samplers to the selected H entry while keeping other entries and theta unchanged. For a shell coordinate with raw allowance L_u and accepted fraction alpha_u, `a_u=1-(1-alpha_u)^L_u`. Sample a successful raw tape, retain its rejected prefix and unused suffix, replace its first accepted shell value by the desired accepted c, then sample the **bit encodings conditional on every decoded shell value**. Bounded-integer rejection has a geometric rejected-word count followed by the prescribed accepted value; unused sign-byte bits remain uniform. The cost is the source's `O(sum_u L_u/a_u)` shell operations with the stated expected bit-decoding overhead. Simply overwriting the decoded c would not be an exact stream sampler.

For scalar/field-vector challenges, condition the full answer on the required coordinate prefix, retain independently uniform remaining coordinates, and sample rejected/accepted words with their correct conditional encodings. Extension coordinates are sampled according to the exact field representation. A single vector answer is still one H entry: no fictitious prover messages or extra scalar queries are inserted. Nested interpolation trees, not flat siblings, certify those vector identities. Fixed filters are witness-independent; response-dependent retry searches remain in Q and do not shrink the challenge denominator a second time.

Theta queries made after a changed H answer may of course have different inputs and different answers in different forks. This is ordinary evaluation of one fixed auxiliary function, not programming that function. B's ro computations and Vhat's defining affine extension space stay unchanged. No conditional sampler for Theta or for a hash-seed preimage is needed.

**Step A5: weighted recursion and coupling.** With the same fixed-theta deterministic maps, Akita's stage recursion applies verbatim at the mathematical level:

`s_r >= s_{r+1}-(Q+1)e_r`,

`C_r <= b_r C_{r+1}+(b_r-1)(Q+1) gamma_r`.

Here gamma_r includes an original replay with its Theta calls and the conditional-stream work. Expensive suffix extraction is performed only on matching indices/prefixes. Thus the original support loss is Q+1, not `(Q+1)^(number of stages)`, and the original Q must not be replaced by all fork-tree queries in the statistical challenge term. The latter queries must nevertheless be charged in computational budgets below.

The base procedure retains the original accepting execution; failure to accept yields no extraction requirement. Matching inputs fix its complete statement/prefix, so the final tree is rooted at x0. Summing stage losses yields eps_tree; D then returns the required witness or collision. Averaging over the independent conditioning proves the probability claim. The same recurrences bound oracle-machine work in expectation because H5 bounds replay and conditional-sampling work uniformly over the fixed environments.

The saved original tuple is never replaced by a convenient fork's output. Theta is exactly the same sampled tuple, so the coupling is stronger than equality of `(x,pi,aux,v)` marginals. If an external observer resumes after extraction, it must receive the saved original prover state/output and original H view, not a forked table or the extractor's private fork transcript. The extractor can restore H overrides; unused underlying H entries remain consistently sampled. Theta needs no restoration because it was never changed. This establishes the claimed coupling. ∎

**AFK/WEE caveat.** AFK Definition 10 alone gives `Pr[v=1 and valid witness] >= (p-kappa)/q(n)`, which only implies a failure bound `p-(p-kappa)/q(n)`, not kappa. Theorem A's additive same-execution bound follows from Akita's actual original-run tree construction and recurrence, not from replacing that divisor by one. An alternative frontend theorem that only supplies a success factor must retain it in the receipt composition.

## 5. Removing the free auxiliary oracle from the SIS reduction

Theorem A is not yet a reduction to standard-model SIS: an actual theta oracle could give a reduction capabilities absent from its SIS game. The following transformation removes it without rewinding an emulator.

Let `E_L` be E stopped after L charged oracle-machine/bit-work steps, retaining the original output; the cutoff also bounds the number `t` of exposed Theta calls and `t_V` of Vhat calls. The original run, initialization and any predicate to be checked must be included. Let `delta_L` bound the probability that the untruncated extraction exceeds L in the actual ideal-setup experiment. Define `Coll_L` as E_L returning a valid named matrix collision.

The reference tuple `(x0,pi0,aux0,v0)` in this probability is always the tuple of the original untruncated experiment. Couple the bounded algorithm to that execution until the cutoff. If the cutoff happens during initialization, the original run, or original verification, the bounded algorithm outputs failure, not a replacement statement. Such executions are charged to delta_L. Exact marginal preservation in Theorem A concerns the untruncated extractor; a strictly bounded algorithm has an additional cutoff-failure symbol. Downstream bounded producers use the same convention.

**Theorem B (bounded ordinary-SIS reduction).** Under H1–H5 **and the input-local paired-emulation contract**, there are ordinary bounded-time Module-SIS algorithms `(B_I)_I`, with no Theta oracle, such that

`Pr[v0=1 and E_L fails to return an R(x0) witness]`

`<= eps_tree + Delta_setup + delta_L + eps_emu(t_V) + sum_{I in I} Adv_MSIS,I(B_I)`,

where `eps_emu(t_V)<=min(1,t_V/2^lambda_A)` from the repaired complete-run theorem; `t/2^lambda_A` is a safe coarse replacement. Internal paired fetches and complete conditional-sampling CPU are charged as in §6.

### Proof

1. **Idealize the admitted joint setup.** Couple the real setup to the ideal flat setup, losing at most Delta_setup. All subsequent oracle sampling/auxiliary generation must be the same efficiently specified conditional experiment in both worlds. Statistical distance is nonincreasing under this extension. One cannot replace an arbitrary public concrete seeded expander by a uniform vector merely because both look pseudorandom.
2. **Use actual-theta tree extraction first.** Theorem A bounds failure apart from collision and cutoff by eps_tree+delta_L. No SIS hardness is used in this step.
3. **One globally emulated computation.** Regard the entire bounded collision producer, including all internal resets, as one serial adaptive oracle computation. Apply the repaired **input-local paired** theorem, retaining its head cache, extended-query transcript and sticky failure state throughout. This changes the aggregate collision probability by at most eps_emu. The oracle-free collision predicate adds no extended query.
4. **Eliminate retained oracles.** For each first activated head x, sample ro(x) lazily, sample/cache mu_x, and compute B using only cached ro(x). Cache both answers for either interface; Vhat sampling uses the same persistent augmented state. This exactly implements the retained pair distribution and adds no statistical loss. All private H programming remains internal computation, not a reset of the emulator.
5. **Embed one ordinary matrix challenge.** In the fully emulated computation, fix a matrix identifier I. Place the uniform ordinary SIS challenge matrix into its coefficient-wise prefix view of the flat vector S and sample all remaining coefficients uniformly. Construct every other view and public reference from that same S. Run C, including any permitted adaptive statement/schedule choice, and output its collision only if it names I. Since the embedded challenge is uniform, S has its exact unconditional ideal distribution. By Akita Lemma 10.11, summing success over I bounds the emulated aggregate collision probability. It is not legitimate to sample A/B/D/F/H views independently, or condition S on the selected schedule.
6. **Combine.** The aggregate `Coll_L` event was emulated **once before partitioning by matrix identifier**, so eps_emu occurs once, not `|I| eps_emu`. Each ordinary SIS advantage uses its corresponding bounded algorithm and its full emulation cost. Add the setup, tree and cutoff losses to finish. ∎

This is a concrete sequence of game transformations, not an assertion that a ROM knowledge theorem “automatically relativizes.” In particular, do not attempt to run a ROM extractor on `A composed with M_ARO` while resetting M_ARO with A. That would generally generate different auxiliary-oracle worlds on different branches. Conversely, leaving M_ARO state outside a black-box prover but citing a deterministic-next-message theorem without Theorem A also skips a necessary step: a lazy emulator's internal state need not itself be a fixed table map on counterfactual histories. The actual-oracle proof and whole-computation emulation avoid both mistakes.

## 6. Query, work, and EPT ledger

Use separate symbols for original and derived computation:

- Q: original adversary's H queries under the admitted query-count convention. This is the Q in eps_tree; there is no additional Q factor for the independent auxiliary oracle.
- `N_run`: all original/replayed prover invocations or resumptions actually made by E, including cheap failed matching trials. Akita Eq. (227), not just the accepting tree's leaf count, controls expected work.
- `t_ro,t_wo,t_V`: total exposed Theta calls across **all** those invocations, initialization, outer-auxiliary computation, projection and any output predicates included in the reduction. For Theorem B's collision predicate, projection to an archive is not needed. For receipt composition it is needed and must be added.
- h: distinct heads activated through either retained interface; v=t_V: extended queries; h+v<=t is a safe exposed-call bound.
- `C_M(h,v)`: complete bit work of the [repaired emulator](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md#7-full-query-storage-and-runtime-ledger), including local B, sparse evaluation, feasibility, exact conditional sampling and tables.

The paired retained implementation makes at most h RO and h WO calls. Its ro-only implementation makes one underlying RO request per head:

`Q_ro,sim <= h`,

`T_sim <= L + C_M(h,v) + T_embed + T_collision`.

Repeated local B gates are CPU work against a cached value, not extra underlying requests. Ro-free B still requires the paired RO fetch. Include all initialization and final predicate heads/extended observations. Conditional uniform-field sampling needs its declared expected-time or separately charged strict-sampling convention.

H is simulated separately with a lazy stream table plus scoped programming overrides. Count every generated bit, field operation, conditional rejection and override lookup in L/work. A complete infinite stream is never materialized; only requested prefixes are generated, with the prescribed conditional unused suffix. The original H-query ceiling does not bound the bit length consumed on every possible ideal rejection tape. The expected sampler cost and cutoff handle that distinction.

Let the source recurrence provide `E[Z] <= P` for charged actual-oracle extraction work Z. Then `delta_L=Pr[Z>L] <= P/L`. This is an **unconditional** tail bound in the same original-run experiment, not a bound conditioned on accepting a rare statement. A sharper admitted tail bound may replace Markov.

One must not conclude `E[poly(Z)] <= poly(E[Z])`: that implication is false. This is why Theorem B first cuts off the oracle-machine extraction and only then uses a polynomial-cost global emulator. The ordinary reduction is strict bounded time with the displayed cost. For asymptotic contradiction arguments, if false success is nonnegligible, choose a polynomial cutoff making the inverse-polynomial tail smaller than that success margin. A fixed concrete `2^-128` cutoff loss may demand an enormous L; EPT alone does not establish useful concrete hardness or performance.

There is only one cutoff event for a single extractor if the same E_L feeds all downstream producers. Do not charge that same event once in beta_outer and again as an independent receipt `delta_cut`. It may be displayed inside beta or separately, with its placement declared.

## 7. Setup, public-key-dependent B, and auxiliary input

**Fixed B is the admitted parent model.** The extension below explains its limits; it is not a silent change to that model.

A controlled key-dependent law first samples s independently of H and ro, sets `B_s=Build(s)`, then samples Theta conditional on s. The actual-Theta proof applies after conditioning on s. For Theorem B, every B_s must additionally be input-local and efficiently computable from the embedded public setup, with uniform size/arity/degree bounds. The pair `(S,Theta)` may be correlated, while H remains independent conditional on S; this is enough for Step A1.

This extension **does not** cover:

- choosing B after observing H, or allowing B to call H;
- choosing the definition of B by adaptive access to an already sampled ro and then claiming the untouched fixed-B ARO sampling law;
- hardwiring a trapdoor or other auxiliary information unavailable to the reduction that embeds the SIS challenge;
- sampling a setup key from H/ro while invoking the oracle-independent setup coupling without a separate analysis;
- adversarially correlated auxiliary input which reveals unqueried H answers, or advice depending on the SIS trapdoor.

The ARO definition permits adaptive cross-input ro queries by a fixed B, but the repaired ordinary-SIS/game-preservation route **does not**. Within an input-local B, adaptive computation may only reuse ro(x) for its current x. Adaptive selection of B after oracle sampling is a different unsupported experiment. Public preprocessing may depend on revealed queries only through the fully counted generation law.

The ordinary SIS embedding is unconditional: install the challenge in one view, fill the other flat coordinates, then compute all efficient public dependencies. A key generated with an unknown trapdoor distribution cannot be reproduced by this procedure merely by saying “the matrix marginal is uniform.” The *joint* law with prover auxiliary information matters. Likewise, if a public seed must be revealed and is required to deterministically expand to S, sampling arbitrary S does not produce a seed preimage. Akita's ideal setup-namespace simulation and its bias term are valid only under the corresponding ideal setup interface. Actual provisioned matrices avoid a seed-preimage problem only if that is the actual declared public interface.

For adaptively chosen outer schedules, use Akita Corollary 10.12 and the accepted schedule-family theorem: a fixed public family independent of setup, a uniform polynomial tree/work bound, maximum admitted challenge error, and the union of its collision inventories. Never condition the uniform setup on the schedule eventually selected. The existing inner finiteT16/33-role registry is not an outer schedule-family certificate.

## 8. Conditional one-final receipt corollary

Suppose Section 9's deterministic frontend and guest lemmas are proved for the exact outer skeleton and admitted outer schedule, so Theorems A/B apply with `R=R_skel`. Suppose also the lattice query accumulator's soundness theorem and the existing native language-soundness theorem apply at the derived budgets.

Keep the original saved `(X,acc,C_static,success1)` and external decider result. On extraction success, the deterministic projection yields `(archive,L,pi_AS,static source)` for that same statement. If the installed original-reference/source binding fails, charge its specified collision event once. Otherwise the static bytes equal the installed approved blob.

Build the AS soundness adversary by running this one E_L, outputting its derived L, exact acc and pi_AS, and testing the external decider in the same Theta. A invalid-query ledger with accepted AS.V and accepted AS.D is then an actual AS forgery. Although E_L programs H internally, H is independent and can be simulated as ordinary private randomness by this Theta-adversary. The AS theorem must use the total extracted-producer time and query budget, not the original archive-prover budget.

Outside that event, the extracted ledger is authentic to the original Theta and guest adequacy gives an accepted native archive. Apply the reviewed **input-local paired** game lift, the corrected analogue of CCGOS Theorem 6.5, to this whole bounded producer and its final ro-only native predicate. The private H simulation is ordinary computation, not programming ro. The lift executes this producer serially even when it internally rewinds a subroutine, retaining one emulator state. Its error is at most `v_complete/2^lambda_A` (capped at one); a total-call bound is conservative. The ROM reduction prices all B/kernel/projection/predicate work and one underlying ro request per activated head.

Thus, with budgets made explicit,

`Pr[false receipt] <= beta_outer,prod(L) + eps_static + eps_AS(B_AS) + eps_native,ARO(B_native)`.

Here beta_outer,prod is Theorem B's full RHS, including its one cutoff loss. If beta is defined without cutoff instead, add that cutoff once outside. Native/AS emulation errors included in their respective terms are distinct game comparisons; the same original setup/static collision must not be counted twice. Keep source theorem success factors if any frontend uses only the weaker AFK Definition-10 guarantee.

This proof needs language soundness of the native archive, not a newly assumed straightline native extractor. It proves soundness of **one final receipt** once its deterministic and accumulator premises are established. It does not prove arbitrary-depth product-model PCD.

## 9. Deterministic outer/frontend and guest obligations still missing

The following are separate from the product-oracle argument and are **not established by it**:

1. **Complete frontend-to-execution tree extraction.** Supply a deterministic accepting-tree theorem for the actual outer Jolt frontend combined with Akita, with every challenge stage, interpolation degree, claim reduction and error term. It must return an outer execution witness or an admitted collision. Akita Theorem 10.33 alone supplies neither this theorem nor its tree/work ledger. Every frontend H challenge and all its nonce/rejection behavior must satisfy H2, not just the PCS suffix.
2. **Weak-source projection, not canonical-state fiction.** Map Akita's valid algebraic decommitments to the exact common-grid tables/used slots needed by the frontend. Prove required one-hot, range, padding/domain and source-consistency predicates on those extracted tables. Do not assume inverse challenge differences preserve shortness or that `Commit(extracted state)` reproduces a canonical state; Remark 10.32 expressly denies that implication.
3. **Original static-reference linkage.** Show the extracted trusted-advice reads and outer RAM initialization refer to the verifier-installed original C_static, under the same original source geometry. Admit the exact weak-source/reference cross-comparison norm envelopes and collision inventory. A prover-supplied replacement commitment or an unproved digest check is not a substitute.
4. **Oracle-free skeleton adequacy.** From every valid extracted outer witness for the approved guest and canonical success1, deterministically recover the actual archive, complete ordered query ledger and AS proof. Prove every actual oracle operation is represented with exact input/output arity, field/byte encoding, transcript framing, rejection attempts and namespaces. The guest consumes supplied answers; there must be no concrete hash or unmodeled oracle evaluation masquerading as an authenticated query.
5. **Exact AS and public statement linkage.** The guest derives the ledger, checks the actual oracle-free AS.V and outputs/binds the exact acc subsequently passed to external AS.D. The original X, guest/index, original C_static, success1 and relevant profile/setup fields must be in the outer statement and challenge binding. Equality of marginally distributed accumulator outputs is insufficient.
6. **Total bounded execution semantics.** Prove canonical parsing, archive EOF, full native final decider, failed-query/sampler/capacity rejection, extension-plane correctness, byte/field conversions and actual advice/trace bounds. The existing whole-run algebraic relation remains the semantic target; no stronger ISA/MMIO/halting theorem is inferred.
7. **Independent construction admission.** Instantiate the auxiliary lattice CM and CCGOS query accumulator on the exact bounded domain, including their message encoding, setup, soundness and costs. Establish the separate outer schedule, field representation and source sizes. The fact that this extraction theorem avoids a same-oracle obstacle does not make that construction fit the existing advice layout.

These are proof and construction prerequisites, not demands to adopt CH-I/CH-O, to assume a new knowledge axiom, or to implement code prematurely. The existing [source/setup contracts](AKITA_INSTANCE_ACCUMULATION_RECEIPT_CODE_MAP.md) and [receipt soundness note](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SOUNDNESS.md) identify relevant application obligations; their conditional status is unchanged.

## 10. Why Lemma 9.1 is unnecessary here, and final decision

CCGOS Lemma 9.1 concerns a proof whose Fiat–Shamir oracle is the **same ro whose graph is arithmetized**. A rewinding extractor programming that ro must reconcile previously observed wo/Vhat constraints; the paper's straightline condition avoids this issue. Theorem A instead programs only H. Theta remains the original fixed function tuple; Theorem B emulates its complete adaptive query history without ever changing its ro. Thus that particular straightline-lift premise is unnecessary for one final outer proof in this product experiment.

This conclusion stops at the model boundary. An outer relation that calls H, a B that depends on H, an arithmetization interface covering H, or a later recursive layer that must authenticate programmed H evaluations changes the experiment and invalidates the stated proof. Nor does public domain separation of concrete hashes prove statistical independence of their idealized instantiations.

**Decision:** retain the independent-outer route as a proved conditional extraction mechanism at the mixed-tree/PCS level, with an explicit standard-SIS reduction after a global cutoff and whole-tree ARO emulation. Do not label the full receipt proved, numerically secure, implementation-admitted, or a concrete-hash construction. The remaining decisive extraction gap is the deterministic complete outer frontend/guest theorem, not a need to postulate same-oracle straightline Akita extraction.

Evidence: primary-paper and existing-note inspection only. No code, Git operations, builds, tests, formatters, benchmarks, or concrete proof runs were performed.
