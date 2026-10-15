# Reusing native outgoing range checks at the next old-source boundary

## 1. Decision and scope

**Conditional construction, not an implemented accumulation scheme or an admitted security profile.** For a finite, authenticated, assembled chain of complete native old-plus-fresh reductions, **rerunning a full same-C source-range prefix on every old receiver is not mandatory**. The preceding update already reduces the range of its outgoing Equation (109) witness to the very pending evaluation consumed as the next update's old opening. In a complete accepting extraction tree, authenticate that evaluation, interpolate the preceding strong checks, and obtain the old source bound. This is the order already used inside Akita's recursive proof.

Three qualifications are essential:

1. The receiver must expose the entire physical source through an injective, full-grid identity view, or through an independently proved source-view bridge. A bound on a gadget-decoded polynomial is not a bound on its source digits.
2. Native common range and compression binariness are real checks; **honest padding zeros are not automatically proved zeros**. Required zeros must be checked once on the outgoing receiver, preferably in its existing fused sumcheck. Only demonstrably unused, fully range-certified cells can instead become bounded filler variables (§4).
3. This is a **finite assembled-history extraction construction**, not a standalone local bounded-AS knowledge theorem for a fixed arbitrary old instance with no prior generator/history. Merely attaching one old transcript does not produce its counterfactual range tree (§6).

The preserved anchor is the exact original handle, physical source, complete Eq. (109) receiver, pending evaluation, and existing application statement. No public homomorphism of compressed C is used. No online, unlimited-depth, zero-knowledge, adaptive-fresh-input, or stronger emulator/halt semantics are imposed. One full trace followed by a legal partition is compatible with this target.

### Minimal integration change

Replace “always prove old source range again” by:

- **Prior native receiver:** reuse its authenticated outgoing-domain reduction and retain its original pending ordinary query. Check exact old-C/view/layout/endpoint linkage. Do not add another old range query solely for a bound already certified by that reduction.
- **Initial/imported old source without that reduction:** retain the same-C domain prefix, or an independent exact domain certificate/initial check.
- **Fresh nonreceiver source:** retain its applicable ordinary/raw source-domain prefix unless the actual frontend proves the same physical predicate under the same handle.
- **Every new outgoing receiver:** retain all native range/norm/binary/relation checks and any genuinely required zero checks; output one fused pending evaluation and every setup obligation.

This saves duplicated incoming old-domain work, not the previous or current outgoing strong checks, receiver scan/commitment, or history authentication.

## 2. Paper and native source evidence

Primary paper: [Akita, local indexed text](SOURCE_PROVENANCE.md#akita), [ePrint 2026/1983](https://eprint.iacr.org/2026/1983). Live qmd retrievals for this investigation used `lattice-papers/akita.md`:

- `2307–2350`: §§5.4, Eqs. (106)–(110), complete receiver order and physical alignment.
- `2393–2492`: §6, full Boolean-domain range, product tree, virtual claim, and restricted negative-binary predicate.
- `2947–2971`: §7.4, Eq. (160), one fused endpoint. The image following these lines is not evidence for additional checks.
- `3951–3989`: Definition 10.22/Theorem 10.23, descendant-certified mixed tree, source uniqueness, incoming **weak** extraction.
- `4141–4231`: complete-tree cost, backward induction, classical ROM, and Remark 10.32's explicit inverse-norm limitation.
- `1839–1860`: full physical decoded grid versus honest zero-padding image.

The immutable native pin inspected here is **Akita `83574331d4e51f8cce1d8689d05592fa4ef4c138`**, the companion of the design's modern Jolt `0791f79` candidate. Exact source links:

- [`fold/mod.rs`](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-verifier/src/fold/mod.rs): `verify_fold` receives the outgoing binding before ring switch, runs Stage 1, Stage 2 rounds, Stage 3 when scheduled, then **`validate_stage2_replay`**, and only then returns the outgoing point/evaluation/setup opening.
- [`stages/stage1.rs`](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-verifier/src/stages/stage1.rs): `verify_product_prefix`, `verify`, `Stage1Replay`; every range stage, child evaluation and interstage batch is replayed. The final range image and its point are retained for Stage 2. A physical-L2 route retains its additional claims.
- [`stages/stage2.rs`](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-verifier/src/stages/stage2.rs): `replay_stage2` combines the carried virtual claim, relation/opening claims and physical-L2 claim; `expected_output_claim` substitutes the same `w_eval` in the linear relation and quadratic virtual term; `evaluate_compression_oracle` adds the restricted negative-binary term. `validate_stage2_replay` checks equality only after setup evaluation is available.
- [`layout/relation_address.rs`](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-params/src/layout/relation_address.rs) and [`stages/ring_switch.rs`](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-verifier/src/stages/ring_switch.rs): the range point uses the **committed Boolean-domain** variable count, not merely the live-prefix length.

Local producer evidence was also read in [`prove/fold/sumcheck.rs`](../../crates/akita-prover/src/protocol/prove/fold/sumcheck.rs), `prove_stage1`: product-stage messages, child claims, interstage challenges, final virtual image and optional norm claims are already produced. This local file is evidence for the inspected local generation, not a claim that its bytes equal the immutable pin.

“rangeStages” and “fused endpoint” in this note name these protocol components, not new/existing APIs with those spellings. Returning only a `FoldVerifyOutput` loses replay details; they remain in the proof transcript and can be retained by reference. A source pause/adoption API and re-entry schedule are still new work.

## 3. Exact retained object and semantic relation

Index update t so it consumes `C_(t-1)` and fresh handles and produces `C_t` committing to `W_t`. Its final fused ordinary claim is

\[
 e_t=(P_t,C_t,\mathrm{view}_t,\mathrm{layout}_t,r_t,v_t),
 \qquad \widetilde W_t(r_t)=v_t.
\]

The receiver source is `s_t=Pack_t(W_t)` with `Dec_t(Pack_t(W))=W` and `Pack_t(Dec_t(s))=s` on **all** admitted physical coordinates, not just honest ones. The source projection includes the frozen shape/setup/A/B/F handle and certified original decommitment path. It excludes point-dependent D/H data. Paths need not be canonical.

### 3.1 Domain reduction record `kappa_t`

Retain the following information, either explicitly or as exact recoverable ranges of the authenticated update transcript:

| Component | Required content |
|---|---|
| Binding context | Protocol/version, setup/profile and schedule identifiers, previous state, ordered fresh handles, complete outgoing C_t, original source geometry, actual commitment/opening view, Eq. (109) layout, all source widths and domain sizes, application context; all absorbed in their original order. |
| Range product pipeline | Initial anchor `tau0`, basis/alphabet and exact stage degree/arities, every round message, child claims, interstage batching challenges, final point `r_virt` and `s_claim`. Fiat–Shamir challenges may be rederived; they must not be independently resampled on replay. |
| Virtual meaning | `s_claim = sum_x eq(r_virt,x) W_t(x)(W_t(x)+1)`. This is the MLE of Boolean products, **not** `W_t~(r_virt)(W_t~(r_virt)+1)`. There is no separately committed s table. |
| Fused relation | Exact quotient-lift or quotient-free row descriptor; ring challenge(s), row-batching point, carried-claim coefficient, compression-binary coefficient; all Stage 2 messages and endpoint `r_t,v_t`; scalar opening/claim combinations and any tensor conversions. |
| Strong domain adjuncts | Negative-binary support intervals, selected response-norm claims and their complete reductions, and the precise required-zero mask/rows if any. Do not infer checks from honest layout labels. |
| Setup and lineage | Direct setup evaluation or complete Stage 3 record and pending setup opening(s); final Stage 2 equality; exact binding/adoption provenance linking this C_t to the next old handle. |

The full transcript before these challenges, or an authenticated means of reconstructing it, is required for ROM replay. A tuple of points plus `s_claim` is not enough to verify or extract the pipeline. A hash pointer without available authenticated bytes is not a certificate. Retaining the archive once is sufficient; it need not be recopied into every arithmetic witness.

Private live state retains `(W_t,d_t)` and native opening authority. `d_t` contains all original A/B/F path data, noncanonical valid digits included. The successor's own commitment path does not occur circularly inside its own Eq. (109) input; it is retained externally and becomes incoming commitment material on the next fold.

### 3.2 Separate replay from truth

Define `Replay(kappa_t,e_t)=1` as the public transcript checks above, with an as-yet unauthenticated endpoint. Define the actual bounded witness predicate

\[
\begin{aligned}
R_{\rm bd}(e_t;W_t,d_t)\iff{}&
 \operatorname{Rel}_{\rm com}(C_t;\operatorname{Pack}_t(W_t),d_t)\land
 \widetilde W_t(r_t)=v_t\\
 &\land\ W_t\in\mathcal A_{b_*}^{N_t}
 \land\ W_t|_{I_{\rm bin,t}}\in\{-1,0\}^{I_{\rm bin,t}}
 \land\ Z_{\rm req,t}(W_t)=0\\
 &\land\text{all original path alphabets/equations and pending setup obligations}.
\end{aligned}
\tag{1}
\]

`I_bin,t` refers to actual Eq. (109) F/H compression digits. It is distinct from the compression path of C_t itself, whose validity belongs to `d_t`. Any additional semantic domain predicate is explicit.

`Replay(kappa,e) AND one true endpoint` is **not a deterministic implication** of (1). Special soundness requires its full mixed challenge tree, with authenticated endpoints and commitment-source uniqueness. A single accepting randomized check can lie on its error event. The retained record is therefore a **pending reduction certificate**, not an independently discharged range proof.

## 4. Physical coverage and exactly which zeros are needed

### 4.1 What native Stage 1/2 actually covers

Let L be the live complete Eq. (109) length, including its internal alignment/compression-completion spans, and let d_next be the outgoing commitment ring dimension. Native `RelationAddressGeometry` computes

\[
 N= d_{\rm next}\,2^{\lceil\log_2\lceil L/d_{\rm next}\rceil\rceil},
 \qquad \texttt{FlatBooleanDomain}(L,\log_2N).
\tag{2}
\]

The local [`witness_commitment_domain_len`](../../crates/akita-params/src/layout/opening_point.rs) gives this rounding explicitly. At the immutable pin, ring-switch replay derives `tau0` with the resulting full domain's variable count; Stage 1 verifies that many sumcheck rounds. The Stage 2 virtual oracle is

\[
 \gamma\,\operatorname{eq}(r_{\rm virt},r_t)\,v_t(v_t+1),
\]

with **no L-prefix mask**. Thus if the authenticated endpoint is genuinely the MLE of the entire N-cell original receiver source, interpolation constrains **all N Boolean cells**. The honest prover's compact representation treats the tail as zeros, but the verifier's full-range polynomial need not rely on that honest convention.

This statement does not cover an original commitment with additional physical block cells outside N, a decoder that truncates such cells, or an alias that opens only its live prefix. The admission obligation is equality of the range domain, endpoint domain, complete physical source observation and SIS-width inventory. Choosing a larger fixed N_* than (2) also requires updating the actual range/relation domain and point geometry; merely reserving a larger buffer does not enlarge the proved domain. The illustrative receiver sizes close at their stated next-power-of-two domains, but are not source-level admitted schedules.

### 4.2 No generic native zero proof was found

Eq. (109) specifies honest zeros; §4.5 expressly says honest zero extension is not a padding-image predicate proved by an ordinary opening. Eq. (160) and the inspected native endpoint include range/binary/relation terms, not a generic mask forcing every unused cell to zero. A relation may already force particular zeros; that requires its actual row evidence. A zero-filled honest backend buffer is not that evidence.

Consequently it would be incorrect to remove the old range/zero prefix while claiming the original stronger `Z_layout(W)=0` follows from native range alone.

### 4.3 Two precise repairs that do not rerun old range

**A. Keep required zeros; fuse them once at the outgoing endpoint.** Let `Z_req(x)` be the fixed Boolean indicator of every cell actually required to be zero. Add to the outgoing Stage 2 integrand

\[
 \eta_Z\,\widetilde{[Z_{\rm req}(x)\operatorname{eq}(\tau_Z,x)]}(X)
 \,\widetilde W_t(X),
\tag{3}
\]

with target zero, a fresh bound-context anchor `tau_Z`, and an independent combination scalar `eta_Z`, sampled in an admitted order after C_t is bound. The restricted-weight MLE, not the product of separate off-cube MLEs, is used. This term has individual degree two; it does not raise the existing degree-three fused check. Its endpoint uses **the same `v_t`**. It needs no additional source query, response, D/H channel or witness table. It costs the new challenge/weight computation, prover work over the mask, transcript binding and full interpolation/error factors. Required zero rows may alternatively be included in the explicit linear relation; their row count and batching are then charged.

This is a small new relation term, not a claim that source already implements it. If a separate outgoing mask sumcheck is chosen instead, retain its endpoint too: use a charged multi-query/channel or claim-combination reduction on C_t. Do not suppress that endpoint. There is still no reason to duplicate the entire old alphabet proof in the next update.

**B. Permit only proved-unused padding as bounded fillers.** Split declared padding into required encoding/map zeros and a set U of genuinely unused auxiliary coordinates. Replace the condition on U by `W_t[U] in A_b*`. This is safe as an explicitly changed receiver relation only if:

1. the full N-cell source is injectively observed and covered by native range and future ordinary queries;
2. **every semantic/native row map that is meant to ignore U has zero coefficients there**, not just the honest prover's implementation;
3. source packing, recomposition, compression maps, claim embeddings and application extraction need no zero identity on U;
4. A/source widths, response bounds and mixed collision classes include U, and the next fold consumes those actual values;
5. every valid nonzero-filler witness can be supplied to the honest update without being silently truncated, recanonicalized or recommitted.

Under these conditions U is ordinary bounded NP auxiliary data; restriction to the used coordinates gives the same semantic relation. The complete physical tuple and capacity still include U. The next fold's response can depend on U, and its worst-case bound must cover that dependence. Honest production may choose zero fillers, but completeness quantifies over all permitted fillers.

This reasoning does **not** license dropping compression-map ring-completion zeros, quotient/encoding-required zeros, or padding needed for a natural-domain evaluation identity. Nor does it establish that the existing opaque source/decoder accepts arbitrary fillers: native compact buffers and “public zero padding” contracts require an explicit all-valid-witness adoption interface. Without that interface and the zero-column map proof, choose A. Both alternatives preserve full Eq. (109) accounting; neither hides physical cells.

## 5. Finite assembled-chain backward derivation

Fix before setup a finite family of admitted update descriptions and source geometries, a supported T_max, and a final opening/decision protocol. For a selected T≤T_max, authenticate the entire ordered history, including every `kappa_t`. Use direct coefficient/identity receivers, exact domain coverage from §4, and the established ordinary/raw carrier relations for every fresh source occurrence. The following is a **derived conditional algebraic argument**; it is not already Theorem 10.33 for Akita's unchanged root-only application policy.

1. **Final weak authentication.** At every accepting leaf of the complete assembled mixed tree, extract the final ordinary PCS opening of `(C_T,r_T,v_T)`, or check a revealed final witness. A native PCS extractor must retain its weak unit/product certificates and original certified path. An arbitrary unbounded algebraic decommitment is insufficient for the comparison argument.
2. **Recover the preceding strong facts about W_T.** Fix the outgoing C_T at its binding node. Across its range/fused/zero/norm branches, use Corollary 10.17 at the complete mixed comparison radii to identify one source, or return a scheduled collision. Full-grid injectivity identifies one W_T. Backward interpolation first binds the virtual claim to Boolean products, then the product tree and anchors force `Q_b*(W_T(x))=0` for every x. The binary term forces the stated negative-binary spans; (3) or existing exact rows force required zeros. Row/ring interpolation recovers every native relation. This proves bounded W_T without bounding an inverse unit.
3. **Extract weak inputs of update T.** Apply the Theorem 10.23 subtraction to its independently strongly authenticated folding children. Recover a weak source under each incoming original handle, including C_(T-1), with its true ordinary evaluation. Specifically `iota(delta) s_old = z^(1)-z^(0)`; the small right side does **not** imply small `s_old`.
4. **Recover bounds using the earlier record, not inversion.** The old evaluation just obtained is exactly the endpoint of `kappa_(T-1)`. In the complete assembled tree, variations at those earlier range/fused challenges have their own authenticated suffixes. Identify the same original C_(T-1) source across those branches, then interpolate `kappa_(T-1)` as in step 2. Now W_(T-1), and therefore its identity source, is bounded. No new range transcript on C_(T-1) is needed.
5. Repeat steps 3–4 until the independently checked initial anchor. Fresh inputs not previously covered by such outgoing certificates use their own same-C physical-domain reductions. Extract and check the exact frontend relations and consecutive boundaries; retain existing Jolt init once and final-output/native-padded-final convention once, rather than inventing a stronger machine statement.

This is well founded: the endpoint authentication comes from strictly later continuation stages, ending at a complete final decision. An actual published final opening is not copied into alternate branches. Rewind and rerun the entire remaining malicious continuation to get **that branch's** final endpoint and final certificate. If that branch rejects, the mixed-tree extractor's success/error analysis handles it; one cannot assume arbitrary requested continuations succeed.

For identity packing this yields `||s_old,i||_infty,coef ≤ b*/2` and `||s_old,i||_2,coef^2 ≤ m_A d_A (b*/2)^2` for each block. Supplied noncanonical A-image/F digits remain admissible under their exact alphabets and equations. No canonical re-Commit step is used.

### Exact asymptotic boundary

Let b_t(N) be the **complete** local mixed-tree factor, including native range anchors, all sumcheck rounds, fold forks, row/ring checks, actual fresh prefixes, zero terms, setup and frontend reductions. Let B_end(N) count final proof factors. Then

\[
 B_{\rm assembled}(N,T)=B_{\rm end}(N)\prod_{t=1}^{T}b_t(N).
\tag{4}
\]

Extraction is polynomial in this supplied tree and explicit lengths. If each factor is polynomial and T_max is an external fixed constant, this is formally polynomial in the explicit instance/security parameters, with an exponent depending on that constant. It can still be impractically large. If T or per-stage factors grow so that the product is superpolynomial, this reasoning does not give uniform polynomial knowledge extraction. Akita's exact tree-admission condition, not the word “finite”, controls the asymptotic claim.

Classical ROM tree generation additionally charges all prover/generator queries, nonce trials, conditional tape/filter sampling, continuation work and auxiliary outputs, together with the actual matrix-collision adversary runtime. The schematic bound is `(Q_total+1)*epsilon_interactive + setup_bias + sum_I Adv_MSIS_I(actual reduction runtime)`, only under the instantiated assembled tree theorem. This is not QROM security, a concrete hash theorem, a numerical security admission, or an oracle-free local extractor.

## 6. Local AS, carried certificates, and universal completeness

### 6.1 Why a fixed retained transcript is not a local range oracle

A local extractor starting with an arbitrary fixed old tuple `(C,r,v,kappa)` can rewind the new update's challenges, but those challenges occur **after kappa was fixed**. It does not thereby obtain distinct old `tau0`, product-tree or fused challenges. All new weak extractions can identify one old source, yet one fixed old accepting transcript plus one true evaluation need not force its global range polynomial to vanish.

Adding only `Replay(kappa,e)=1` to the old relation is therefore not an information-theoretic substitute for the bounded predicate. Nor does declaring `has_valid_range_certificate=true` solve it. A soundness theorem that treats kappa as free fixed advice cannot silently rewind the party that originally constructed that advice. This is the concrete obstruction to promoting §5 to the usual local bounded-AS knowledge contract.

### 6.2 What carrying kappa does and does not solve

A precise public state can be `(e,kappa_ref,setup_pending,boundary)`; its private valid relation remains (1), with exact replay/link consistency when the chosen interface requires kappa. This makes the retained domain reduction available to history extraction and avoids retransmitting it. It does **not**, on its own, prove local bounded knowledge.

There are three honest local formulations:

- **History-generating experiment:** the adversary/generator produces the previous certificate and current update in one joint classical-ROM experiment, beginning before the certificate's random challenges. The extractor has rewind access to that generation and the paid continuation, including whatever further prior context is needed to replay the original transcript. This is the assembled/path-specific theorem, not an arbitrary fixed-input local theorem.
- **Independent certified input:** the old tuple has an independently completed domain proof with the exact auxiliary-input, source-projection, and adaptive witness-extraction guarantee. Its extraction and verification are charged. This may be composed locally if the successor-leaf oracle is also supplied under its actual contract; kappa's pending native transcript is not such a proof.
- **Arbitrary bounded input:** keep the semantic bounded relation and a fresh same-C old prefix (or another actual bounded-source reduction) inside the local protocol. This puts the necessary range challenges within that local extractor's control. The outgoing endpoint authentication oracle is still not free.

For the first formulation, quantify over EPT generation/update processes with arbitrary auxiliary strings and outputs, including application witnesses, prior transcript/oracle answers, rejected attempts and selective aborts. Charge generation queries in Q_total; reproduce the published branch and required output. If certificates are generated adaptively, the theorem must cover the actual conditional oracle state. If inputs are fixed in advance, say so; adaptive fresh choices are optional, not needed for the user's finite target. Giving a successful one-run certificate as arbitrary nonuniform advice is not the same experiment.

One can define a more expensive exact relation additionally requiring that all retained transcript partial claims equal their actual W-derived table evaluations. This is a deterministic predicate on `(kappa,W)` and does not depend on honest provenance. But extracting that semantic consistency from an arbitrary fixed kappa still requires a proof; including it in a definition is not an extractor. Reproving it in the successor or checking it explicitly pays its genuine work. A receipt that merely checks public replay also does not prove this strengthened predicate.

### 6.3 Strong completeness is independently all-valid-input

For every admitted original tuple `(a,W,d)` satisfying its declared semantic relation and every valid fresh source tuple, the honest update must use **those supplied sources and decommitments**, including noncanonical paths and permitted nonzero fillers, and produce a valid successor with the declared abort bound. Nothing in range reuse justifies restricting this quantifier to sources generated by an honest prior update.

Universal deterministic response envelopes, encodable signed-depth capacity (not just the larger verifier acceptance radius), full path alphabets, quotient closure and fixed receiver capacity must cover this entire domain. A state can have many valid path decompositions; use the supplied valid one rather than recomputing a canonical C. If the relation has no certificate component, rejecting an otherwise valid input because a separately attached certificate is absent or invalid violates that completeness claim. If it has a certificate component, its validity and semantic requirements must be formally in the relation and preserved for **every** satisfying tuple; an “honest history” predicate or unchecked validity flag is not a substitute.

Dropping the new old prefix does not by itself impair this algebraic completeness: a bounded supplied source is still folded, and the new honest outgoing witness still has range/binary/required-zero checks. What is lost at a naked local boundary is the old-domain **knowledge proof**, not the existence of an honest update on bounded inputs. Strong completeness and local knowledge must not be conflated.

## 7. Fresh sources and final decider

### Fresh sources

A fresh ordinary commitment is not a previous Eq. (109) receiver merely because both are Akita handles. For gadget sources, native weak extraction only returns the original algebraic digits; their decoded polynomial's range does not certify those digits. Keep the [raw source-query bridge](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md) and its same-C raw-range endpoint where applicable. Keep every ordinary frontend query too.

A frontend one-hot argument may already prove Booleanity, occupancy or padding. Reuse it only after matching the actual original physical source, all committed cells, decoder, query endpoint and extraction projection. A one-hot producer type or honest trace sparsity is not that proof. Occupancy-based response bounds need the actual occupancy predicate, not just Booleanity. Nothing in this note asserts that modern Jolt's frontend already covers this complete domain.

### Two exact final requirements

**Bounded standalone decider, unchanged interface:** retain [main design §6.3](AKITA_INSTANCE_ACCUMULATION_DESIGN.md#63-a-final-only-bounded-decider-using-a-complete-native-pcs-proof), with the exact domain chosen here. It reduces final same-C range/required zeros plus the pending evaluation to one ordinary endpoint and runs one complete admitted Akita proof. It discharges all setup claims and yields the bounded final relation. This duplicates the last outgoing range once, not every old source range. An exact reveal-once decider can instead check the complete final source/path/evaluation/domain deterministically. Neither authenticates the history by itself.

**History-aware assembled decider:** for the narrower §5 target, the final proof may authenticate only the final ordinary `(C_T,r_T,v_T)` and all pending setup claims, retaining the weak opening certificates required for source uniqueness. The final update's already-generated `kappa_T` supplies the final range/binary/required-zero interpolation as the last strong stage of the assembled tree. Thus a second final range prefix and its two-claim reduction are not algebraically necessary **inside this jointly proved assembled verifier**. They cannot be removed while continuing to call the remaining ordinary PCS verifier a standalone bounded decider for arbitrary imported `(C,r,v)`.

In both choices, final verification authenticates the initial anchor, every exact update/domain record and derived endpoint, original fresh frontend records, ordered adjacency, approved program/public statement, and the existing Jolt final convention. A plain replay archive may pay O(T) transcript storage/checking; a succinct receipt needs its own exact relation and extraction/compiler argument. A final C alone does not prove that the advertised segments were included.

## 8. Saved and retained cost

Let the removed old range prefix produce one extra point on a single old handle. Write, as in the source-query ledger,

\[
 Z=M\delta_{\rm com}\delta_f d_A,\quad
 E_1=PB\delta_{\rm open}d_{\rm op},\quad
 T=PBn_A\delta_{\rm open}d_A.
\]

These are full base-field cell counts before all alignment, quotient and compression terms.

| Integration | Exact kind of saving from removing the duplicated old prefix/query |
|---|---|
| Separate occurrences | Remove one entire old query occurrence: Z+E_1+T, its original F-chain slots, its applicable quotient slots, `PB` folding coordinates, one fold-evaluation row, `n_A` A rows, all original B rows and its F rows; remove its scalar claim or reduce the claim-batch inventory. |
| Shared response with two old channels | Change old H from 2 to 1: remove E_1 partial cells, one carrier fold-evaluation row and the corresponding scalar query. **Do not** remove Z, T, A/B/F, or `PB` challenges: they were already shared. |
| Shared opening path | D width decreases by E_1/d_D for either removed channel/occurrence. There remains one D and one H path. Do not automatically delete H compression cells or lower its rank; its image size may remain unchanged and all re-admission gates still apply. |
| Prefix arithmetic/transcript | Remove the duplicate old polynomial/product-tree range computation, its round messages/endpoint and its new challenges/error/tree factors. The previous and new outgoing Stage 1/2 proofs remain in full. |
| Retained state/history | Keep old `(C,r,v)`, exact view/layout and cert reference; keep its transcript once in the archive, full live private receiver/path state and source adoption authority. No history-size discount follows from constant public C. |

For the simple baseline prefix of alphabet D, the removed transcript has at most `(|D|+1) log2(N_old)` compressed extension-field sumcheck coefficients plus its endpoint and explicitly serialized context/challenges. For the optimized native-style range tree, use its actual stage degrees, round counts, intermediate child evaluations, virtual-binding stage and endpoint; there is no universal one-number discount. Queries here mean **opening obligations**; ROM queries/nonce trials must be priced separately.

### Hand-derived consequences for the existing generic examples

These are arithmetic consequences of the [receiver profile](AKITA_INSTANCE_ACCUMULATION_RECEIVER_PROFILE.md), not newly validated profiles and not Jolt workload/timing claims. Hold all its original source geometries and ranks fixed, assume required-zero checks are fused at the outgoing point, and retain the fresh range endpoint.

- **Separate-occurrence example, capacity 2^23:** one removed old occurrence saves `524,288 + 262,144 + 1,048,576 + 16,384 + 2,048 = 1,853,440` cells. The complete live Eq. (109) count becomes `6,135,808`, leaving `2,252,800` tail cells at the same capacity. Ring rows become `33−(1+4+1)−2=25`; fold coordinates become `384−64=320`. Shared D width decreases by 2,048, with its image/H path retained. Required tail predicates/filler semantics still apply.
- **Shared-response example, capacity 2^22:** remove only one old partial channel, `131,072` cells. Live count becomes `3,872,768`, tail `321,536`, D width `10,240−1,024=9,216`, ring rows `20−1=19`. Fold coordinates stay 160; old response/A/B/F and both fresh channels stay. These numbers do not justify shrinking N again without recomputing closure and every security/completeness bound.

If a separate outgoing mask endpoint is necessary rather than fused (3), it consumes a channel/query and its downstream partials/rows. That may eat the opening-count saving but still avoids recomputing the whole alphabet range prefix. Under the fused option, (3) introduces no Eq. (109) table cells and no additional endpoint; its transcript, public weight, prover scan and strong-tree factors remain charged.

Replaying a retained certificate when an independently supplied old instance enters a new verification session costs its public Stage 1/Stage 2/Stage 3 checks again unless an authenticated verification context/receipt justifies reuse. It does **not** require regenerating a new proof. In one assembled verification pass, each archived record can be replayed once. Cryptographic extraction may revisit it many times and charges that work in (4).

## 9. Review-ready integration contract

A range-reuse profile must expose these exact proof interfaces:

1. `ReceiverSourceView`: bijection between every physical source coordinate and the full range/MLE domain, original C/A/B/F shape and widths, all source/path alphabets, no truncating aliases.
2. `OutgoingDomainReduction`: complete native range-product and fused record, exact binary/norm predicates, required-zero mask/rows or proved-unused filler set, checked endpoint and setup obligations.
3. `OldReceiverLink`: old C, view/layout and ordinary endpoint equal the previous outgoing record; use the already authenticated record, not a digest asserted to exist.
4. `StrongRecover`: supplied descendant-authenticated mixed tree yields the entire receiver domain and exact rows, or a collision at the **mixed** certified radius. It never derives a source norm by inverse-unit cancellation.
5. `WeakFoldRecover`: complete fork families yield the original weak input paths and all ordinary/raw channels; every endpoint source is obtained from its own paid continuation/final certificate.
6. `CompleteAllValid`: update algorithm accepts every declared bounded witness/path/filler tuple and fits the exact physical Eq. (109) capacity, subject only to its stated uniformly proved sampler/encoding failure bound.
7. `FiniteAssembledAdmission`: exact family, T_max, stage list/tree product, frontend semantics, ROM experiment/query bound/auxiliary I/O, collision inventory and runtime-sensitive concrete reduction ledger; final standalone-bounded or history-aware choice explicitly selected.

The algebra shows that a **second old alphabet-range proof is redundant for the finite native history target**, once these interfaces are met. It does not establish the arbitrary-input local-AS theorem, default native support for zero masks/nonzero filler adoption, a secure numerical chain schedule, or a free counterfactual endpoint oracle. Those limits do not force duplicate old-range work; they specify the actual protocol and proof boundary to admit.

**Work performed:** paper/qmd and source inspection; symbolic derivation and hand cell/row arithmetic. No code edits, branches, builds, tests, formatters, benchmark, accumulator run, or schedule-validation command. This document is the only created file.
