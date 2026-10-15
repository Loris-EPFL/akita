# Final Akita receipt: relativized-oracle limits and alternative models

## 1. Decision and exact scope

**The new lower bound rules out a general-purpose succinct compiler for computations that query an ordinary ideal random oracle. It does not prove that this fixed, bounded Akita receipt is impossible, nor that a concrete-hash Jolt guest is insecure. Conversely, choosing a fixed guest, an independent outer oracle, or Module-SIS assumptions does not establish the missing receipt theorem.**

The preserved anchor is the [native pre-PCS design](AKITA_INSTANCE_ACCUMULATION_DESIGN.md): original K-valued sources, scalar challenges in `E=K[U]/(U^2-5)`, the approved 33 inner matrix roles, `1<=T<=16`, every frontend/update prefix, and one complete final decider. The selected [receipt setup](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SETUP.md) fixes actual inner matrices/program/static data through an independently keyed original outer trusted-advice commitment `C_static`; the archive remains untrusted private advice. The receipt is compact public run `x` plus one complete ordinary outer lattice Jolt/Akita proof that the approved fixed guest returns exactly success=1. No hash/Merkle accumulator, Pedersen substitution, universal recursion, or per-segment completed PCS proof is introduced here.

This note sharpens the model boundary in [receipt soundness §§5–7](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SOUNDNESS.md). It is research, not an implemented or numerically admitted outer profile.

### Evidence boundary

- **BCG:** Annalisa Barbara, Alessandro Chiesa, Ziyi Guan, *Relativized Succinct Arguments in the ROM Do Not Exist*, [ePrint 2024/728](https://eprint.iacr.org/2024/728), TCC 2025; published chapter [DOI 10.1007/978-3-032-12287-2_14](https://link.springer.com/chapter/10.1007/978-3-032-12287-2_14), pp. 400–416. The publisher lists first online 2 December 2025 and book copyright 2026. **Consulted:** primary abstract/publisher notes and [Guan's author slides](https://ziyiguan.github.io/slides/relativized-snargs.pdf), including a rendered inspection of slide 7's equations. The slides are preserved locally as [BCG-Relativized-SNARGs-Slides.pdf](SOURCE_PROVENANCE.md#supplemental-primary-sources). **Not obtained:** full paper. The theorem numbers below are the slide numbers, not an assertion that formal theorem numbering/hypotheses in the paper were inspected.
- **CCS22:** Megan Chen, Alessandro Chiesa, Nicholas Spooner, *On Succinct Non-Interactive Arguments in Relativized Worlds*, EUROCRYPT 2022, [DOI 10.1007/978-3-031-07085-3_12](https://link.springer.com/chapter/10.1007/978-3-031-07085-3_12), pp. 336–366; [ePrint 2022/383](https://eprint.iacr.org/2022/383). **Consulted:** primary Springer abstract/notes and EPFL repository metadata. **Not obtained:** full paper. Thus the positive theorem is stated at the precise scope of the primary abstract, not with invented parameter bounds or extractor guarantees.
- Claims marked **[INFERENCE]** below are applications of these source statements to our receipt, not quoted paper theorems. Exact security-parameter conventions, all admissible time functions, finite lower-bound constants, and arbitrary setup/preprocessing variants cannot be certified from slides/abstracts alone.

## 2. What BCG actually rules out

### 2.1 Ordinary ROM security is not relativized computation

BCG slides 2–3 explicitly distinguish:

1. An argument whose prover/verifier use an ideal oracle `H`, but prove membership in an oracle-independent language `L`.
2. A **relativized** argument for a language family `L_H`, whose meaning itself depends on `H`; for example, `(u,v)` is in the language exactly when `v=H(u)`.

In the second case, correctness must be about the **same sampled oracle** that the experiment and adversary use. Giving a witness an arbitrary finite table of purported oracle answers does not satisfy this requirement. A proof that the table is self-consistent is not a proof that it agrees with `H`.

BCG's primary abstract says the impossibility holds even for **interactive arguments** and even for **computational**, not merely statistical, soundness. Slide 7 explicitly says adversaries are both query-bounded and time-bounded. This is therefore not just a lower bound obtained by letting the attacker break every computational primitive through unlimited local computation.

The publisher's Note 4 is another important quantifier check: the relevant guarantees are averaged over the sampled oracle, rather than demanding soundness with probability one for every oracle. The latter requirement would be too strong even for known non-relativized ROM SNARGs. Do not weaken BCG to a claim solely about one pathological oracle outside the ordinary sampled-oracle experiment.

### 2.2 The two succinctness separations

The author slide 7 displays, in its notation,

```
Theorem 1: DTIME^O[t] is not contained in ARG^O[vq=o(t)].
Theorem 2: NTIME^O[t] is not contained in ARG^O[as=o(t)].
```

Here `vq` is verifier oracle-query complexity and `as` is argument proof size. These are **class noncontainments**, not statements that every oracle-dependent language is hard.

- For deterministic oracle computation, the trivial baseline lets the verifier do the computation itself with `t` queries. The first separation says that a generic sublinear-query argument cannot cover the entire class.
- For nondeterministic oracle computation, the trivial baseline sends the entire witness. The second separation rules out covering the entire class with sublinear argument length.
- The slides state that similar results hold for interactive arguments; the primary abstract independently confirms the interactive impossibility. We do not infer exact interactive round/communication tradeoffs absent the full theorem.

Slides 9–10 make the existential hard-language quantifier concrete. Their example uses a language family of the form

```
L_f = { 0^n : there exists w in {0,1}^{t(n)}
                 such that f(w || i)_1 = 0 for every i in [t(n)] }.
```

This is an explanation of their hard-language mechanism, not a complete formal theorem statement or a replacement for the paper's encodings/runtime conventions. The slides vary oracle answers on a witness-indexed collection of points so that membership changes while a short proof gives the verifier insufficient information to locate the relevant queries. Nothing in the inspected sources embeds that family into the approved Akita archive language.

### 2.3 Correct negative conclusion for a compiler

**[INFERENCE]** A proposed compiler that takes arbitrary oracle-aided computations, preserves their ordinary random-oracle semantics, and yields the succinctness/security covered by the BCG separations cannot be justified merely by choosing a different algebraic PCS or adding an independent Fiat–Shamir namespace. It would supply the relativized argument class that the source says cannot cover all such computations.

This is stronger than the older warning that a particular non-black-box PCD compiler excludes ROM. It says a generic repair that keeps arbitrary ordinary oracle gates and succinct verification encounters an actual lower bound, rather than merely an unproved interface in those older compilers.

It is also narrower than “IVC/PCD in ROM is impossible.” Slide 7 expressly places “Relativized IVC/PCD does not exist in the ROM” beside **“Existence of IVC/PCD in the ROM still remains open.”** This records the source's scope; it is not a claim that this note has surveyed every subsequent result on that open problem.

## 3. Independent outer oracle, lattice assumptions, and setup

### 3.1 Independent `H_o` is useful separation, not oracle authentication

**[INFERENCE]** Two ordinary independent random oracles with compatible output interfaces can be represented by disjoint injective namespaces of one ordinary random oracle, for example `H_i(u)=H(0||u)` and `H_o(v)=H(1||v)`. Independence is an ideal-distribution property of those namespaces. It does not give `H_i` an efficiently checkable algebraic description, a signed evaluation interface, or a low-degree extension.

Accordingly, adding `H_o` does not by itself answer the BCG barrier to a **universal** ordinary-ROM compiler: it changes neither the need to authenticate the inner oracle calls nor the nature of the ordinary oracle model. Applying a formal lower-bound statement to a particular two-oracle setup would still require matching its exact resource/encoding/setup conventions; that full-paper check is not claimed here.

For our conditional composition, separation nevertheless matters. A hypothetical outer extractor may program/fork `H_o` while preserving the original `H_i` environment. The required result is an archive valid under that unchanged inner oracle, not under a table selected by the extractor. Newly issued inner queries must be answered consistently with the original external `H_i` view. Separate namespaces make this contract expressible; they do not prove it, and ordinary NP extraction does not automatically cover an oracle-relative guest relation.

### 3.2 Module-SIS does not turn black-box random answers into constrained guest gates

**[INFERENCE]** BCG already targets computational arguments, so “our proof uses a computational lattice assumption rather than information-theoretic soundness” is not a generic escape. Native lattice source binding and Module-SIS collision reductions concern the committed algebraic data. They do not authenticate a purported external `H_i` evaluation inside an ordinary circuit, and do not identify a deterministic SHAKE/hash computation with an ideal random oracle.

This does **not** assert a source theorem excluding every possible CRS, oracle-correlated preprocessing, auxiliary ideal functionality, hardware interface, or trusted online oracle service. The inspected slides show an oracle-aided argument model, not a fully inspected theorem for arbitrary augmented setup models. An augmentation outside it needs its own stated functionality and proof. Merely postulating that an impossible universal ordinary-ROM compiler is secure would be an inconsistent assumption in that model; a restricted concrete-hash knowledge assumption is a different proposition.

### 3.3 What selected `C_static` provisioning fixes

The selected setup provisions actual inner matrices and authenticated program/static bytes before the run, independently of the outer key coefficients. Its original-source/reference binding mechanism can eliminate the need for the guest to recompute a concrete setup XOF and can preserve the intended inner matrix-sampling/embedding experiment, subject to the setup note's exact distribution and binding premises.

**[INFERENCE]** This repairs a **setup-distribution** boundary, not the Fiat–Shamir oracle boundary. `C_static` contains approved proof-independent material; it is not a commitment to the entire random oracle, an authentication certificate for every future adaptive inner query, or a trusted transcript-response service. Putting run-dependent challenge answers into that blob would change the selected setup contract. Trust in sampled matrices cannot silently become trust in prover-selected oracle answers.

## 4. Why fixed `T<=16` avoids an impossibility conclusion but not the proof obligation

There are three distinct quantifiers.

1. **Generic family:** a compiler handles all admitted oracle computations as their size grows. BCG's class noncontainments obstruct the blanket claim at its stated succinctness thresholds.
2. **Restricted language family:** a fixed verifier/program/profile may define a special oracle-dependent family with exploitable structure. A class separation does not prove this particular language is one of the hard languages. Even the simple equality language `(u,v):v=H(u)` can be verified by one oracle query. Any exclusion of the Akita archive language would need a relevant reduction or a construction-specific lower bound, neither supplied by the consulted sources.
3. **One finite installed profile:** our cap is 16 segments with fixed admitted geometry, not an asymptotic claim of arbitrary-depth succinct IVC. An asymptotic `o(t)` separation is not a numerical lower bound on the proof bytes/verifier work for this one installation. We must still define and substantiate the intended concrete compactness; calling all quantities “constant” under a fixed cap would not meet the user's no-history-exposure goal.

**[INFERENCE]** The finite cap is consequently a reason **not to overclaim impossibility**, not a proof that a missing oracle-aware compression method exists. It bounds the honest guest/advice shape. It does not make the random oracle finitely describable by the chosen hash instructions, bound the adversary's off-transcript queries, or cap the number of counterfactual executions needed by an extractor. A fixed program can also describe an asymptotically difficult language when input parameters are allowed to grow; “fixed program” alone is not a general loophole.

The current concrete guest computes `ArchiveAccept^{h_i}`, an ordinary finite algorithm. It does not compute `ArchiveAccept^{H_i}` by oracle gates. **BCG neither attacks nor rules out the former.** The correct limitation remains that the native ROM theorem is not, by itself, a theorem about the former.

## 5. Positive alternative oracle models

### 5.1 CCS22: low-degree random oracle and the LCROM framework

The [primary CCS22 abstract](https://link.springer.com/chapter/10.1007/978-3-031-07085-3_12) makes the following positive statement:

> Letting O be a low-degree encoding of a random oracle, and assuming the existence of standard-model collision-resistant hash functions, there exist transparent SNARKs relative to O for all languages in NP^O.

It explicitly says such a SNARK can prove a computation about its own verifier. The paper introduces the more general **linear code random oracle model (LCROM)**, obtains SNARKs for oracle-querying computations given an **accumulation scheme for oracle queries**, and constructs that scheme for the low-degree case.

The algebraic structure belongs to the **oracle model**, not merely to the constraint system used by the proof. BCG slide 12 describes the low-degree oracle as sampled from bounded-individual-degree polynomials over a field, and explains why its arbitrary-point modification argument cannot simply be transferred: the changed function need not remain in the allowed polynomial family. Springer CCS22 Note 4 likewise emphasizes that its oracle is guaranteed to be low degree, unlike settings requiring a proximity test to justify that property.

Two distinctions are essential:

- BCG slides 6 and 12 explicitly contrast impossibility of relativized PCPs/IOPs with feasibility of relativized SNARGs in LDROM. It is invalid to infer the latter's impossibility solely from the former's impossibility.
- Slide 12 identifies the CCS22 construction as secure against **query-bounded and time-bounded** adversaries, while stating as an open problem there whether LDROM relativized SNARGs exist against merely query-bounded adversaries with unrestricted local computation. The standard-model collision-resistance assumption is not an unconditional black-box random-oracle property.

**[INFERENCE]** Even if a suitable lattice assumption can instantiate a standard-model collision-resistant hash component, that supplies only that component. It neither creates the low-degree oracle nor proves that the exact Akita scalar/shell sampling, conditional challenge programming, frontend composition, and matrix setup retain their original analysis in LDROM. The inspected abstract supplies no direct LDROM-to-Akita interface or concrete extractor query bound. Adopting CCS22 would be a new oracle-aware construction/model and a new security ledger, not a relabeling of the ordinary Jolt guest.

### 5.2 Other models and why they are not ordinary ROM

BCG slide 6 lists three positive families: **signed random oracle (SROM)**, **low-degree random oracle (LDROM)**, and **arithmetized random oracle (AROM)**; it labels these models “Hard to instantiate!” The signed model provides an augmented oracle interface rather than an unsigned ordinary hash oracle. No theorem about realizing that interface by our lattice setup is claimed here.

For AROM, the relevant primary work is Chen–Chiesa–Gur–O'Connor–Spooner, [*Proof-Carrying Data from Arithmetized Random Oracles*, EUROCRYPT 2023](https://link.springer.com/chapter/10.1007/978-3-031-30617-4_13), examined separately in the companion research. This note uses BCG's slides only to establish that AROM is a recognized positive alternative, not to import its exact extraction theorem or assume it authenticates unrestricted SHAKE calls.

BCG slide 13 also identifies an elementary positive possibility for sufficiently learnable/predictable structured oracles: learn their representation, then use a non-relativized SNARG. **[INFERENCE]** This is not available merely because one accepted archive contains finitely many observed oracle answers: knowing that finite table does not determine or authenticate all adaptive queries on other admissible archives or extraction branches.

These alternatives refute “oracle-preserving succinct proofs are impossible in every model.” They do not refute the ordinary-ROM separation, and they are not authorization to replace the selected native accumulation with a different accumulator.

## 6. What remains about extractor query blowup

BCG's `vq` is the **external verifier's oracle-query complexity**; its `as` is proof communication. Neither is our `q_B`, the query count of the **derived archive-producing algorithm** built from outer extraction. The source lower bounds therefore are not numerical estimates of Akita extraction cost.

The [existing receipt theorem](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SOUNDNESS.md) defines `B` by running a same-execution outer archive extractor, then projecting/checking the archive. Its inner queries include the original adversary's queries, additional queries from every outer fork/replay, and reconstruction/replay queries if actually issued. Outer-namespace queries do not count directly as inner queries, but outer rewinding can cause additional inner queries. The 16-record bound on an honest archive does not limit this adversarial/extraction process.

For the ROM route, retain the exact conditional ledger:

```
p_bad_receipt <= beta_o + epsilon_bind + delta_cut
                 + epsilon_i(t_* + t_proj, q_* + q_proj),

epsilon_i(t_B,q_B)
  <= (q_B+1) (c_E/|E| + c_C/|C|)
     + Delta_i + sum_{I=1}^{33} Adv_MSIS,I(R_i,I(t_B,q_B)).
```

The existing `<2^-137` inner statistical FS figure requires the admitted `q_B+1<=2^32` policy. Neither BCG nor the CCS22 abstract supplies a certificate for that cap, an outer archive extractor with a small `q_B`, or an acceptably small cutoff tail. With only expected work/query bounds, the soundness note's explicit Markov/union-bound cutoff loss remains; no lower bound here proves it small or necessarily large for Akita.

**What these papers close:** the generic hope that an ordinary-ROM oracle-preserving universal SNARG is available just by applying standard succinct-proof composition; and the misconception that every ideal-oracle model has the same obstruction.

**What they do not close:** the concrete inner-hash instantiation, the exact outer frontend/PCS/private-advice extraction, preservation of the original inner environment, the derived `q_B` and full reduction runtime, or the outer field/sampler/matrix/error admission. The inspected positive source does not give a quantitative transplant for this scheme.

Language soundness of the final receipt needs extraction of an accepting archive plus inner computational language soundness; it need not additionally extract the original run witness. A whole-run knowledge claim adds the nested native extractor/reset/expected-time-closure obligations already specified in the soundness note. The impossibility paper does not erase that distinction.

## 7. Permissible final receipt claims

| Proposed claim | Status justified here |
| --- | --- |
| Native archive replay has its reviewed classical-ROM/Module-SIS theorem | Unchanged, within its admitted profile and runtime/query assumptions |
| Ordinary outer Jolt soundness proves correct execution of its fixed concrete hash guest | Requires the complete outer execution-relation theorem/guest adequacy; alone does not establish application soundness of computational inner arguments |
| A generic ordinary-ROM SNARG can succinctly certify arbitrary inner oracle computations | Obstructed by BCG at the stated class/succinctness scope |
| A second independent ordinary RO or standard lattice binding automatically repairs that compiler | No; independence/binding does not authenticate inner oracle gates |
| The fixed 16-segment native Akita receipt is impossible | Not established; no applicable hard-language reduction or construction-specific lower bound was found |
| The fixed 16-segment concrete-hash receipt is automatically secure because the generic lower bound does not apply | False inference; its concrete inner soundness and outer archive extraction still need proof/explicit assumptions |
| LDROM/LCROM provide positive oracle-relative SNARK routes | Yes at the CCS22 primary abstract's scope, with standard-model CRH and a different oracle interface; no direct Akita instantiation established |
| Existing inner Q32 and statistical bounds survive the wrapper without change | Only after bounding the derived archive producer and all cutoff losses, not from these papers |

The most direct realizable conditional route remains the soundness note's **CH-I + CH-O + common-environment/adequacy/binding** contract: fix concrete `h_i` (and concrete `h_o` if deployed), assume/prove native archive soundness at the derived extractor's actual runtime, and assume/prove extraction of an archive for the original public run and approved static data from the exact complete outer proof. An extractor in that concrete experiment cannot program either fixed hash function. A hybrid with concrete `h_i` and ideal outer `H_o` is meaningful, but still uses a concrete inner-soundness assumption and is not a full proof for both deployed concrete hashes.

An oracle-aware alternative must name an actual changed oracle model/construction or prove a restricted-language ordinary-ROM theorem. It must authenticate all relevant oracle calls, preserve the required inner environment, and re-establish the quantitative ledger. It may not simply reinterpret private `(query,answer)` advice as trusted oracle gates or cite an ordinary oracle-free NP SNARK theorem.

**Bottom line:** the available evidence supports neither “apply a generic ROM wrapper and inherit soundness” nor “this receipt cannot exist.” It supports a bounded native core plus an explicit, construction-specific final-receipt assumption/proof obligation, with structured-oracle models as genuine but non-drop-in alternatives.

## 8. Retrieval and activity receipt

Research only; no source edits, builds, tests, formatters, benchmarks, or Git operations.

- The author slide PDF was downloaded successfully and its theorem slide rendered to resolve mathematical symbols lost in text extraction. Only that genuine PDF is retained from this task.
- Full BCG ePrint retrieval was attempted through the static reader, direct download, and a real browser. PDF requests returned HTTP 403; the version-history page also presented an access challenge. The primary ePrint/publisher abstracts and author slides were readable.
- Both Springer full-paper PDF endpoints redirected to HTML subscription/preview pages rather than delivering PDFs. Downloaded HTML files with provisional `.pdf` names were removed; no fake full-paper copy is retained.
- The CCS22 EPFL landing page returned HTTP 429. The public [item API](https://infoscience.epfl.ch/server/api/core/items/6c5d850d-7828-4092-b9ed-a06e524cfd74) was accessible and explicitly labels the record `metadata-only`; its [bundles endpoint](https://infoscience.epfl.ch/server/api/core/items/6c5d850d-7828-4092-b9ed-a06e524cfd74/bundles) contains only a LICENSE bundle, not a full-text attachment. This route therefore does not conceal an uninspected downloadable paper.
- Author publication pages for Guan, Chiesa, Spooner and Chen were checked for official alternate copies; their relevant paper links lead back to ePrint. Search-generated summaries were not used as substitutes for the unavailable formal theorem statements.
- The additional [CCS22 Warwick WRAP record](https://wrap.warwick.ac.uk/id/eprint/166967/) was located and read. Unlike the accessible AROM accepted manuscript in WRAP, this record explicitly says **“Research output not available from this repository”** and reports `full_text_status: none`; it supplies bibliographic metadata and the abstract, not a manuscript.
- An additional BCG deposit was located through EPFL's public discovery API: [item f251b43a-5042-494d-b007-bc1c6d2ea60c](https://infoscience.epfl.ch/server/api/core/items/f251b43a-5042-494d-b007-bc1c6d2ea60c). It is likewise `metadata-only`, and its [bundles endpoint](https://infoscience.epfl.ch/server/api/core/items/f251b43a-5042-494d-b007-bc1c6d2ea60c/bundles) contains only LICENSE. Targeted Bocconi-repository searches did not locate another relevant deposit. These additional routes do not upgrade the evidence to full-paper inspection.

A later full-paper review can strengthen the formal setup/resource scope, but must not retroactively describe the present abstract/slides-based evidence as a full proof inspection.
