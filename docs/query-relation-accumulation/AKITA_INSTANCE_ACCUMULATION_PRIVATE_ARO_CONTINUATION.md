# Original-transcript/private-ARO continuation: a restricted extraction and composition theorem

## 1. Result and invariant

**The proposed continuation theorem is valid under the input-local, paired-head emulator contract, with the restrictions below.** Its extraction loss is

\[
 \kappa_{\rm actual}(L)+\epsilon_0+\epsilon_{\rm Full},\qquad
 \epsilon_0=\min(1,v_0/2^{\lambda_A}),\quad
 \epsilon_{\rm Full}=\min(1,v_{\rm Full}/2^{\lambda_A}).                 \tag{1}
\]

The initial prover, complete outer verifier, and **two actual external ARO decider evaluations** run against the original actual oracle. Their original output and decision are retained literally. Subsequent extraction uses private computation, including a private continuation of the repaired emulator, with no further access to the actual inner oracle. The conclusion concerns a precisely specified deterministic predicate on that original record and the extracted output. It does **not** preserve the joint law of the extracted output with arbitrary future actual-oracle observations.

This distinction suffices for conditional receipt composition: the transformed extractor is itself a legal adversary in the actual-Theta query-authentication/native experiments. Its original accumulator already has the actual external decider's recorded acceptance. Authentication of its extracted ledger is a separate actual-oracle security event, not an assumption that the private oracle is true.

The benefit is a separation of **actual** inner queries from **virtual** extraction queries. A large extraction tree can remain expensive private computation without contributing its private heads to the external native random-function query count. The two extraction coupling errors still depend on the virtual extended-query budget, and all private work still enters reduction time. This is neither a cheap-PCS-extractor theorem nor an honest-proving/parity result.

The anchor remains native **pre-PCS instance accumulation**, original K sources and handles, E challenges, the 33 native matrix roles, finite T16, and the complete final native decider. No CH assumption, concrete ARO, new commitment, canonical-source assumption, or numerical admission is introduced. Zero-finding/quotient authentication has its own transport obligation; this note does not discharge it.

**Independent review:** PrivateContinuationReview approved the conditional transformer and resource ledger with no findings, including sticky-state replay, original-record preservation, the two hybrids and the explicit future-observer limitation. The separately reviewed [hidden-curve theorem](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md) now supplies a parametric authentication premise; its concrete source/profile/budget instantiation remains required.

## 2. Sources and exact premises

The mathematical argument here is derived, not an author-issued correction to CCGOS.

1. The [ARO witness-scope note](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md), Sections 4–7, supplies the repaired empty-start, complete-bounded-run emulator theorem and its cost ledger. Its unrestricted-B counterexample excludes use of the unqualified published Theorem 5.4. Its Section 8 distinguishes recorded evaluations from nonpersistent implicit curve values.
2. The actual [CCGOS accepted manuscript](SOURCE_PROVENANCE.md#supplemental-primary-sources), [extracted primary text](SOURCE_PROVENANCE.md#supplemental-primary-sources), Definition 5.1, lines 1317–1328, specifies transcript-based lazy answering. Construction 5.11, lines 1536–1558, uses the sparse graph plus a zero-on-the-cube extension. Construction 5.20/Lemma 5.21, lines 1928–1981, provide the constraint-detector/linear-algebra implementation and polynomial cost. The opposite signs in that display must be corrected consistently as in the witness-scope note. Lemma 6.1/Construction 6.2, lines 2001–2013, justify exact persistent witness sampling. These primary statements were read directly; the unrestricted-B emulation claim is not a premise.
3. CCGOS Construction 8.2, lines 2426–2499, distinguishes the oracle-free accumulation verifier from the decider's two extended evaluations, and places the CM challenge at a raw `ro(cm)`/`wo(cm)` head. This is relevant to accounting, not an approval of the quotient adaptation's zero-finding proof.
4. [Akita primary text](SOURCE_PROVENANCE.md#akita), Definition 3.15 and the preceding paragraph, lines 1461–1466, require the original statement and a resettable same-execution handle. Theorem 10.31/Remark 10.32, lines 4175–4197, preserve full-grid source projections and expressly do not promise canonical short gadget sources. Its native ROM theorem, lines 4205–4210, has a particular Q convention and ordinary-MSIS running-time parameters.
5. The [product extraction](AKITA_INSTANCE_ACCUMULATION_PRODUCT_EXTRACTION.md), Sections 3–4, and [source-first extraction](AKITA_INSTANCE_ACCUMULATION_SOURCE_FIRST_EXTRACTION.md), Sections 3–6 and 8.1, provide alternative **conditional actual-fixed-Theta** extraction contracts. The latter's ordinary-SIS/emulator shorthand is not used here. The [chain compiler](AKITA_INSTANCE_ACCUMULATION_CHAINED_TRANSCRIPT.md), Sections 5–7, supplies the component-to-native-stream simulation, after the repaired ARO-to-ro transformation.

The reviewed emulator premise is the following. Fix oracle-independent setup, `a=m+lambda_A+w`, `d>=2`, a represented finite field F, and an input-local witness circuit

\[
 wo(x)=b(x,\mu_x,ro(x)),
\]

where the `mu_x` are independent persistent uniform r-bit strings and every B oracle gate queries x itself. Ro-free B is included. Public-setup dependence is allowed only with the setup conditioning/initialization requirements of the witness-scope theorem. Hidden correlated oracle advice is not free.

For any bounded classical adaptive computation, the paired emulator with one persistent state differs from an actual sampled Theta by at most `v/2^lambda_A`, for the **entire visible transcript/output**, where v bounds all extended calls in that computation. Additional independent H_o randomness is allowed. Pairing both retained interfaces is mandatory; merely using the published `ro => wo` convention is insufficient.

An actual-Theta extraction theorem is a separate premise, not derived from the emulator. Its resettable replay contracts, deterministic source extraction, frontend certificate if source-first is selected, matrix inventory, common shapes, and guest projection must all be supplied. The theorem below is a resource/composition transformer for such a theorem.

## 3. Machines, original record, predicate, and cutoff

### 3.1 Initial machine P

P comprises all oracle-dependent setup/initialization, the original receipt prover, its complete outer verification, and the external ARO decider on the **original public accumulator**. Execute both specified extended checks, even when the first rejects, if the selected interface requires a fixed two-call decider; an explicitly short-circuiting interface instead has an at-most-two-call bound. Malformed input handling is fixed before the experiments.

Retain

\[
 U=(s,z,o_0,x_0,acc_0,V_0,D_0,hdl_0,J_0,\tau_0).
\]

Here `o0` is the entire original public output, `V0` and `D0` are its actual original outer-verifier and external-decider results, `hdl0` is the resettable original execution handle with all required ordinary state/tape, `J0` records the original H_o view, and `tau0` is the **ordered augmented Theta query record**. Every first retained-head activation records both `(ro(x),wo(x))`, fetching the missing component privately if needed. Only the requested component is delivered to the original prover; the additional pairing is not a new prover message. All paired requests are charged. Record every original extended-query answer, including verifier/predicate calls and repeats. The original H_o view and original prover state are not overwritten by later private forks.

The complete record does not include an oracle's hidden polynomial or hidden witness coins. It may include P's ordinary coins: a bounded recorder can expose those independently sampled coins in its output, so complete-run emulation still applies to the required joint record. Its internal kernel coins are not passed to K.

### 3.2 Extraction continuation K and its restricted predicate

K resumes/replays the original handle using the selected outer extraction algorithm, with private H_o branch tables and the original statement/handles fixed. Its output is `W`, a named collision `c`, or failure. Its deterministic relation is

\[
 R(s,x_0,acc_0;W),
\]

and its collision predicate is the oracle-free `Coll(s,c)`. These predicates may use the frozen original record and supplied finite data, but **may not make a new Theta call, inspect an unrecorded oracle value, or ask whether a private answer equals a future actual answer**. The acceptance gate is the frozen bit `A0 = V0 and D0`; using only V0 is also allowed. A semantic false-statement event independent of Theta may be intersected with the event below; that does not require an efficient semantic-language decision procedure.

For receipt composition, R must be the complete certified deterministic source/guest relation, not merely an accepting PCS leaf. It must bind the same original public statement and accumulator, original `C_g,C_t` and static source, exact full-grid shapes and source projections, canonical *consumed bytes* where required, every native source/endpoint/final-decider check, the exact ledger/coverage map, and every declared quotient/source-opening claim. Successful weak lattice states remain weak states: retain their unit factors and bounded products. Do not replace their sources by honest canonical gadget decompositions, truncate commitment grids to logical lengths, or bound a source by an uncertified inverse-unit norm.

The actual-theorem hypothesis is

\[
 \Pr_{\rm actual}[A_0\land\neg R(W)\land\neg Coll(c)]
       \le \kappa_{\rm actual}(L).                                 \tag{2}
\]

The algorithm interpreting a returned tag may treat the unused W or c as failure. A registered collision has its full deterministic equation, nonzero check, shape, and certified norm; an unverified allegation is not a collision.

### 3.3 One logical clock, not one implementation clock

Fix the **same deterministic logical oracle-machine meter** for P followed by K in every world. Charge ordinary bit operations, state copying/restoration, private H_o tape work and conditional sampling, parsing, input/output materialization, comparisons and projection. An exposed Theta call costs one logical call unit; a first-head pairing fetch costs one additional logical call unit. Cached repeated calls still cost their ordinary call/table work. A private first activation is charged the same logical complementary-call unit even though no external oracle is used. This convention is an explicitly normalized version of the extraction machine, and its actual-Theta work/tail premise must use this convention too.

Kernel constraint solving, kernel field randomness, and emulator implementation overhead are **not** charged against this logical L; they have a separate polynomial CPU bound below. Stopping the emulated machine earlier because an oracle simulation is expensive would change the experiment and invalidate the direct hybrid. A different normalization is possible, but all worlds and the actual extraction theorem must use that identical counter.

Assume P is already strictly bounded by L0 and reserve `L>=L0`, so P always finishes before the continuation can hit L. Stop K at L, retaining U and returning extraction failure. This preserves the original bounded experiment's output and actual D0 on every execution, including extraction cutoff. If the selected original prover/sampler itself is unbounded, a separate initial cutoff changes its original distribution: compare to the unbounded original only off that cutoff and add its `tau_init`. One cannot simultaneously abort an arbitrary initial run and claim its unproduced original output was preserved literally.

All budgets are worst-case bounds of these stopped machines across **all worlds**, not conditional means on successful executions. In a source-first instantiation one may use, under the same meter,

\[
 \kappa_{\rm actual}(L)
 \le (q_o+1)\epsilon_{\rm PCS}+\tau_{\rm suffix}+\tau_C
    +q_o\epsilon_{\rm FE}+\binom{q_o}{2}\gamma_0+\delta_L .          \tag{3}
\]

The q_o convention includes all original outer verifier and initialization queries. The source-probe total-loop tail `delta_L` is separate from the native suffix's tail. Alternatively, a K-probe cap has the unconditional `q_o/(K+1)` bound from the source-first note, and strict cost `(K+1)(T_C+L_S)+T_cmp+T_proj`. Use the tails of the selected construction once, not two names for the same cutoff. An unconditional first-moment bound gives `delta_L<=P_work/L`; it does not bound `E[poly(work)]`.

## 4. The seeded private continuation

### 4.1 State and totalization

The repaired emulator state is `(S,T,f)`:

* S maps every activated Boolean head x to the **pair** `(y_x,z_x)`;
* T stores every recorded extended input/answer;
* f is the sticky implementation-failure flag.

Define the sparse Boolean graph `v_S`, its multilinear extension `P_S`, and

\[
 \mathcal A(S,T)=\{P\in L_d:P|_{\{0,1\}^a}=v_S,
                              \ P(u)=T(u)\ (u\in\operatorname{dom}T)\}.
                                                                    \tag{4}
\]

Constraints are simultaneous. No record overwrites a contrary cube condition. On a new extended input, the delayed-failure convention tests the *old* affine space; if empty, sets f. In a nonfailed feasible state, sample the conditional evaluation in (4) using the sign-consistent zero-extension kernel. Repeats return cached T. After failure, new extended inputs return and record zero; retained head behavior stays exact.

**Seed from the ordered record, not only its final maps.** Replay `tau0` without resampling or changing an answer. Insert pairs and cached extended values in their original order. Immediately before each first extended query, run the same old-space feasibility test as the delayed-failure emulator, updating its sticky flag. This reconstructs the emulator's flag on any emulator-generated record. Testing only the final `(S,T)` is insufficient: a prior failure remains sticky even if subsequent head activations happen to make the current space feasible.

For an actual-oracle prefix, this replay can encounter infeasibility or actual answers outside a previous sparse conditional support. Keep every actual answer as the original cached fact; never claim to have sampled it from an empty space, and never replace the original decision. Use the reconstructed flag and the totalized continuation rule thereafter. This is a well-defined algorithm on such records, not an exact posterior sampler for the actual ARO. Contradictory repeats or malformed encodings in arbitrary supplied records have a fixed failure outcome; generated real/emulated records have consistent repeats. A total seed map is all the hybrid needs. Its polynomial consistency-checking cost is charged separately.

### 4.2 Retained known heads and fresh unknown heads

For a known x, return S(x)'s requested component. **Do not attempt to invert B to recover its original `mu_x`, and do not redraw `wo(x)`.** The pair is sufficient for all future retained answers and for (4). No future instruction of the emulator needs that known head's coins. Storing coins for newly sampled private heads is optional when the pair is permanently cached.

For an unknown x, draw fresh private uniform `y_x in {0,1}^lambda_A` and `mu_x in {0,1}^r`; compute `z_x=b(x,mu_x,y_x)` with every local B gate answered by that cached y; insert the pair. All of these values are private computation, including the private random-function table lookup. They are not calls to the actual C/ro, even if a bitstring happens to equal an actual query key.

Keep this one private `(S,T,f)` through **all** K resets, source probes, suffix recursion and comparisons. It is not reset together with a replayed prover. New extended answers use the kernel of the current sparse state; this does not posit one immutable latent sparse polynomial across changes of S.

## 5. Exact emulator continuation lemma

**Lemma.** In the fully emulated initial world, replacing the unused part of the underlying ro/WO-coin tables by fresh private lazy tables at the end of P, retaining S,T and the reconstructed f, gives exactly the law of continuing the same totalized emulator. This includes adaptive future queries, all internal K resets, and cutoff under the fixed logical clock.

**Proof.** Initially the head pairs are independent across heads, with law

\[
 Y_x\leftarrow\{0,1\}^{\lambda_A},\quad
 M_x\leftarrow\{0,1\}^r,\quad Z_x=b(x,M_x,Y_x).
\]

In the efficient emulator, an unactivated head's random variables are not read by any retained response at another head, by any feasibility test, or by the sparse extended-output kernel. Input locality is essential here. Conditional on the *emulator's ordinary finite history*, the not-yet-activated head variables therefore retain their independent prior distribution. This is the deferred-decisions property of independently indexed tables, also when the next unactivated index is chosen adaptively. It does not condition on a hidden BAD monitor, eventual success, or a true-ARO transcript.

Known heads affect subsequent instructions only through their pairs; conditioning on an observed z can bias its own mu, but that mu is never needed again. No inverse sampler is required. For feasible states the next extended answer's distribution is exactly uniform evaluation on (4); the kernel samples that law from the complete S,T, not from an unrecorded old polynomial seed. For failed states the next-answer rule is deterministic apart from fresh head pairs and is likewise state-measurable. The seed replay reconstructs f exactly on this world. Fresh kernel draws use independent private coins in both descriptions.

Thus the transition kernels agree on every emulator-reachable state. Induction on the serial logical execution, including its stopping rule and all K's ordinary random choices, proves equality of the joint recorded/extracted-output law. The argument remains valid after a rare emulator failure. It is neither a uniform conditional-TV guarantee for arbitrary prefixes nor a claim about the actual ARO's posterior. ∎

## 6. Three worlds and the restricted extraction theorem

Use the same normalized logical code for P,K in all worlds, and define:

* **G_private:** P uses the actual sampled Theta. Seed the private emulator from U and run K privately as in Section 4.
* **G_em:** P uses the repaired emulator from empty state. Seed from its record and run the same private K.
* **G_actual:** P and K use one actual sampled fixed Theta throughout; only H_o branch tables are private/programmed.

Let `v0` bound P's extended calls and `vFull` bound those of the complete normalized P+K machine. This includes the initial external decider and every extraction-internal predicate. Deterministic output predicates add work but no extended calls.

**First comparison.** Apply the complete-run paired emulator theorem to P viewed as a recorder of U. Hidden missing halves can be exposed to this recorder by the counted normalization without revealing them to the original prover. Its recorded-output distance is at most epsilon0. The same total randomized seed-and-K postprocessing has no access to the old underlying Theta in either world. Data processing gives

\[
 \Delta(G_{\rm private}^{\rm rec,out},G_{\rm em}^{\rm rec,out})
 \le\epsilon_0.                                                   \tag{5}
\]

This works even on sparse-inconsistent real records because the postprocessing is total. There is no uncharged conditioning on their absence.

**Middle equality.** By Section 5, G_em is exactly the full persistent-emulator execution of P+K. This equality concerns the machine's relevant visible state, not a hidden original full oracle table.

**Last comparison.** Apply the repaired complete-run emulator theorem to the single bounded oracle algorithm P+K, treating every internal reset as an ordinary internal operation and retaining one emulator state. With the same logical cutoff,

\[
 \Delta(G_{\rm em}^{\rm rec,out},G_{\rm actual}^{\rm rec,out})
 \le\epsilon_{\rm Full}.                                         \tag{6}
\]

Now invoke the actual-fixed-Theta theorem (2) **only in G_actual**. Triangle inequality proves (1) for `A0 and not R and not Coll` in G_private. More generally, any deterministic test of the stated recorded/output variables transfers with the same sum. This is an unconditional probability statement over the original experiment; it is not a bound divided by the probability of a rare observed U.

Original output preservation in G_private is stronger and simpler than a hybrid claim: its first phase really was P against the actual original Theta. K never replaces o0, acc0, its actual decider inputs/answers, or the original external H_o view. These facts hold literally even when extraction fails.

### 6.1 Why the stronger future-observer statement is false

Take P empty, and let K privately activate a previously unseen x and output its private y. An external test subsequently asking the actual `ro(x)` sees equality with probability `2^-lambda_A`; if K had used the actual oracle, equality would be certain. The joint distributions can differ by `1-2^-lambda_A`, even with zero extended calls. This does not contradict (5)–(6): the equality test accesses a new actual value, which is excluded from their predicate.

Accordingly, do not move an AS truth test, a final native verifier, or a CM challenge query inside (1) while continuing to call it deterministic. Such predicates are separate actual-oracle computations with their own query/resource bounds and security arguments.

## 7. Legal actual-Theta producer and conditional composition

E_private is an ordinary adaptive actual-Theta algorithm: run P using that interface, perform polynomially bounded private computations, and output the original public accumulator and an extracted skeleton/archive (or a collision/failure). An oracle adversary is permitted to calculate arbitrary private functions; those functions need not be consistent with its external oracle. This establishes legality without any future-oracle indistinguishability claim.

Assume the following independently certified implication and security premises:

1. On R, deterministic projection returns the complete original-handle archive, ordered query ledger and authentication/source witnesses for **acc0**, using the same `C_g,C_t` sources and all required full-grid/shape/weak-state certificates. It does not substitute a counterfactual receipt or a fork's accumulator.
2. For this bounded actual-Theta producer, an accepted actual acc0 together with the valid deterministic authentication relation but a false ledger has probability at most `epsilon_auth(E_private)`. The [reviewed hidden-curve transport theorem](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md) supplies this parametric zero-finding bound under its complete source, ownership, sampler and budget contracts, with Q outside the square root. Those application contracts are not inferred from this transformer.
3. For this bounded actual-Theta producer, a false application statement with a complete accepting native archive whose replay ledger is true has probability at most `epsilon_native^Theta(E_private)`. The archive is then checked with the actual native streams and complete native final decider, not private K answers.

On original receipt acceptance outside the extraction-failure/collision event, R holds. Premise 2 makes its ledger true except with its charged probability; premise 1 then makes the full actual native verifier accept the projected archive. Premise 3 bounds a false application statement. These are event inclusions **within G_private's actual experiment**. No transfer of the whole oracle joint law is invoked.

A useful conditional ledger is therefore

\[
 \Pr[\text{false receipt}]
 \le \kappa_{\rm actual}(L)+\epsilon_0+\epsilon_{\rm Full}
      +\epsilon_{\rm outer\,coll}
      +\epsilon_{\rm auth}(E_{\rm private})
      +\epsilon_{\rm native}^{\Theta}(E_{\rm private})
      +\Delta_{\rm setup}+\tau_{\rm impl}.                         \tag{7}
\]

Here the outer collision term can be realized without another extraction-coupling error as in Section 10. Setup distance is charged once for the chosen joint setup comparison; `tau_impl` covers only separately introduced initial/implementation cutoffs, not tails already in kappa. The authentication and native security terms must use their **complete derived producer runtime**, not just the original prover runtime. Equation (7) is not a claim that its missing guest/authentication/native premises are now admitted.

## 8. ACTUAL and virtual query ledgers

Use distinct quantities:

| Quantity | Meaning |
|---|---|
| `h0,v0,t0` | P's distinct retained heads, extended calls, and exposed Theta calls, including initialization, original outer verification and original external D |
| `hK,vK,tK` | new private heads, extended calls and exposed logical calls in K |
| `hFull<=h0+hK`, `vFull<=v0+vK` | full persistent-emulator bounds used in epsilonFull and CPU |
| `hF,vF,tF` | additional actual requests in the final security-game predicates, beyond P's cached actual record |
| `hExt<=h0+hF`, `vExt<=v0+vF` | actual external interface totals for the legal derived producer plus final predicates |

Counts are worst-case suprema after the declared logical/cpu cutoffs; repeated-call counts may safely overestimate distinct inputs. If seeding stores all pairs, actual retained-oracle pairing uses at most `h0` ro and `h0` wo requests; there are no such requests in K. Vhat calls in P do not automatically activate the Boolean head coordinate mentioned in their input. Virtual Vhat queries do not become actual extended calls merely because their arguments are also possible actual inputs.

When converting the **legal derived producer** to a C-only game using the repaired ro-only emulator, one actual C request suffices per newly exposed external head: obtain y, sample mu, evaluate local B using cached y, and retain both answers. Thus

\[
 Q_{C,\rm external}\le h_{\rm Ext},\qquad
 \epsilon_{\rm lift}\le\min(1,v_{\rm Ext}/2^{\lambda_A}).            \tag{8}
\]

There is no factor equal to B's number of repeated local gates; that is CPU. Even ro-free B still needs the paired C answer. The bound is *not* `hFull`: private K randomness is internal to this adversary and cannot query its external C. In this second emulation there are two distinct state objects: an emulator for E_private's actual external interface, and E_private's own private continuation. They must not be accidentally merged after extraction begins.

For an outer collision predicate alone, or a final native predicate that truly uses C only, `vF=0`; v0 still includes the original two actual D calls and any original prover extended work. For actual ledger truth checking, charge its raw head requests (at most one new head per ledger input if no extra side predicates); checking D again can reuse its unchanged cached inputs/answers but still costs local work, or counts the extra calls if implemented without caching. An authentication proof using appended interpolation/curve checks has additional vF and CPU even if the receipt verifier itself does not.

### 8.1 CM and normalization conventions

An output `cm` that was never activated cannot be treated as a fresh answer already in the transcript. If a CM/zero-finding reduction appends `ro(cm)`, perform and count it before freezing whatever final state its theorem uses. A previous wo-only activation has already fetched ro(cm); the later exposed ro request is not fresh. One selected output contributes at most one new head (and a paired counterpart at the retained-interface level); several candidate outputs/retries contribute their actual bounded total. The CM payload uses its exact injective namespace/encoding, including all commitment handles, profile and nonce.

Appended CM raw queries belong in hExt for that authentication reduction, and in its raw-answer forking budget. They are not automatically native stream keys: the chain compiler's CM tag is separate from rooted native Squeezes. They still enter the direct-component budget/collision bound and CPU. Do not drop them merely because they do not contribute a native challenge coordinate.

The original prover's own native ledger, query-authentication production, arbitrary off-protocol queries, all paired normalization, setup/initialization, the complete original outer verifier, the external decider, final output predicates, and any CM-appending convention must be covered **before** comparing with a Q32 policy. An honest query count or the old standalone native adversary budget proves none of these inequalities.

## 9. After the fixed-input chain compiler

First apply the repaired ARO-to-C game transformation to the whole legal producer plus the required native predicate, with (8). Only then replace component C by the chain simulator. Never swap C while leaving a wo/Vhat sampled for the old C attached.

For the usual split, let `Q_C,prod<=h0` count the producer's actual external heads after ro-only conversion; add any actual post-extraction producer-side requests to this Q. Let the final native verifier use the compiler's high-level interface. With H its distinct full encoded prefixes and `B_H` its distinct block triples, use

\[
 N=Q_{C,\rm prod}+\sum_{P\in H}(\lfloor |P|/b\rfloor+1)+B_H,
 \qquad G\le Q_{C,\rm prod}.                                      \tag{9}
\]

The compiler's loss is

\[
 \epsilon_{\rm chain}(N,G)
 =\min\!\left(1,\frac{N(N+1)/2+2NG}{2^{\lambda_A}-(N+G+1)}\right), \tag{10}
\]

with the trivial value 1 if the denominator is nonpositive. It counts actual component-output variables, hidden verifier absorb/finalization work, squeeze blocks and unrooted guesses, not just logical challenge sites. If a predicate instead issues direct component calls, include those in Q and do not also count them as independent high-level calls. Required external high-level services used by the producer add their own expansions and keys.

In the ideal chain world, the producer's number of distinct native stream keys is at most its simulator-rooted native squeeze-block queries:

\[
 q_{\rm nat,prod}\le q_{\rm rooted\ S}\le Q_{C,\rm prod}.           \tag{11}
\]

Absorb and CM queries do not each create a native key, so this is a conservative upper bound. If the native theorem normalizes by executing the final verifier and preprocessing within the adversary, use

\[
 q_{\rm nat,norm}\le q_{\rm nat,prod}
                  +q_{\rm nat,V}+q_{\rm nat,init},                 \tag{12}
\]

subtracting overlaps only if proved. If its theorem instead explicitly uses an adversary Q and a `Q+1` output-verification convention, match that actual convention rather than counting the same finalization twice. Source-first's outer q_o and these native keys are different objects.

For random-access block index j, a sequential native stream implementation may have to generate every block through j. Its bit-work is bounded by

\[
 \lambda_A\sum_{\text{requested native keys}}(j_{\max}+1),           \tag{13}
\]

plus analogous independent private namespaces. Indices, byte lengths, retries and framing require finite public caps. A large index is not one lambda-bit CPU operation; nor does it create j+1 distinct stream keys. The native sampler/bit-cursor machine must be exactly the one covered by eta and the compiler theorem.

Consequently, conditional on the exact native theorem and compiler premises,

\[
 \epsilon_{\rm native}^{\Theta}(E_{\rm private})
 \le \epsilon_{\rm lift}+\epsilon_{\rm chain}(N,G)
           +\epsilon_{\rm native,flat}(q_{\rm nat},T_{\rm flat}).   \tag{14}
\]

No Q32 figure is asserted. Even a small bound in (11) leaves potentially enormous private extraction/emulation time in `T_flat`, as well as the native reduction's own work.

## 10. Polynomial CPU, storage, and ordinary-SIS time

### 10.1 Explicit emulator overhead

Let `ell_F=ceil(log2|F|)`, c_F bound field-operation bit work, T_B bound B evaluation including all local gates, and `C_CD(a,d,j)` bound the actual constraint detector for the relevant degree vectors on j points. Let `T_IO(h,v,t)` include complete map/key storage/lookups, serializations and randomness bookkeeping over t calls. With bounded encodings it is polynomial; a deterministic linear-scan table already gives a conservative polynomial implementation without a free expected-O(1) dictionary assumption.

A sufficient cost for one fresh lazy paired emulator is

\[
\begin{split}
 \mathsf E(h,v,t)={}&O(h(T_B+r+\lambda_A)) + T_{IO}(h,v,t)
                +O(c_F a h v^2)\\
 &+\sum_{j=1}^{v}O\!\left(a C_{CD}(a,d,j)+a C_{CD}(a,d,j+1)
                              +c_F(aj+1)^3\right)
       +T_{\rm field\,random}.                                   \tag{15}
\end{split}
\]

The `a h v^2` term recomputes the sparse polynomial on all old support points when needed. Gaussian elimination and uniform affine-solution sampling are explicit; the affine space is never enumerated over its full Boolean cube. A failure branch can only reduce later kernel work. Seed replay costs `T_seed`, bounded by the corresponding old-support feasibility part of (15), plus reading its entire ordered transcript; no kernel random draws or B inversion are needed. A bound of the form `T_seed<=poly(a,d,h0,v0,t0,ell_F)` with those explicit detector/elimination operations is therefore available.

Persistent pair storage uses `O(h(m+lambda_A+w+r))` bits if coins are retained; T uses `O(v(a+1)ell_F)` bits. Kernel linear algebra uses `O((av+1)^2)` field elements plus constraint-detector workspace. Add the extractor's full reset handles, source tables, full-grid certificates, H_o branch tables, and output materialization. The native archive's succinct *public receipt* does not make the extracted private witness small.

Using monotone worst-case bounds, E_private's real-oracle CPU obeys

\[
 T_{\rm private}^{\Theta}
 \le L+T_{\rm seed}+\mathsf E(h_{\rm Full},v_{\rm Full},t_{\rm Full})
          +T_{\rm wrapper}.                                      \tag{16}
\]

This overcounts initial private simulation safely. L includes projection/comparison when those occur in P+K; otherwise add their explicitly bounded costs. Actual ideal-oracle response costs are oracle-machine costs here, not an implementation of a concrete graph polynomial.

For its C-only realization with a separate external emulator and actual final predicates,

\[
\begin{split}
 T_{\rm ROM}\le{}&L+T_{\rm seed}
 +\mathsf E(h_{\rm Full},v_{\rm Full},t_{\rm Full})
 +\mathsf E(h_{\rm Ext},v_{\rm Ext},t_{\rm Ext})\\
 &+T_{\rm final}+T_{\rm wrapper}.                                \tag{17}
\end{split}
\]

Counts in the first E describe virtual private continuation; counts in the second describe emulating the external interface. Numerical key equality does not make their states interchangeable. For the native flat reduction add the chain simulator's dictionaries/label propagation and prefix-copying work, exact native image recognition, bounded sampler work, the generated bits in (13), the complete final native verifier and any later native extraction/reduction cost. One explicit safe bound for chain bookkeeping uses O(N^2) bounded-record operations (linear scans and at most one processing of each dependency when rooted), plus all copied prefix bytes and (13); use the actual compiler's implementation bound if sharper. Thus `T_flat <= T_ROM + T_chain + T_native-final`, without asserting any of these terms is small.

### 10.2 Ordinary outer-SIS reduction without another coupling error

There is a sharper accounting than separately emulating G_private again to bound its collision probability. In G_em, run one full persistent emulator with a completely private lazily sampled ro table. This is the law of G_em by Section 5. Every operation, including K and all its internal resets, is now ordinary bounded computation. For each matrix identifier I, embed the ordinary uniform SIS/MSIS challenge into that exact coefficient-wise view of the shared uniform flat setup, sample the complement, construct all dependent views, and return a collision only when its verified identifier is I. Use the full inventory and its actual shape/norm bounds; no selected-schedule conditioning or independent resampling of overlapping views is permitted.

By (6), the event `A0 and not R and not Coll` in G_em has probability at most `kappa_actual(L)+epsilonFull`. Its aggregate valid collision probability is at most

\[
 \sum_{I\in\mathcal I_{\rm outer}}
           \operatorname{Adv}_{\rm MSIS,I}(\mathcal B_I).           \tag{18}
\]

Apply (5) **once to the event `A0 and not R`**, after this partition. Hence

\[
 \Pr_{G_{\rm private}}[A_0\land\neg R]
 \le\kappa_{\rm actual}(L)+\epsilon_0+\epsilon_{\rm Full}
        +\sum_I\operatorname{Adv}_{\rm MSIS,I}(\mathcal B_I),       \tag{19}
\]

in the ideal setup. This is the outer collision meaning in (7). No extra epsilon0 is necessary, and neither coupling error is multiplied by the number of matrix identifiers. Charging an additional separately performed emulation would be safe but weaker, not mandatory.

A sufficient per-instance running-time bound is

\[
 T_{\mathcal B_I}\le L+\mathsf E(h_{\rm Full},v_{\rm Full},t_{\rm Full})
               +T_{\rm embed,I}+T_{\rm collision,I}+T_{\rm wrapper}.
                                                                    \tag{20}
\]

The seed reconstruction can be omitted by this distributionally identical full-emulator implementation; include `T_seed` if it is actually performed. Restoring the actual joint setup adds its single declared `Delta_setup`. An independent authentication/native SIS reduction may fork or extract this derived producer again; its own run-count, work/cutoff and output-verification costs must be applied to (17), not to the original prover cost. Equation (20) does not pretend to supply those separate reductions' runtime polynomials.

### 10.3 Exact sampling versus strict bit-time

Uniform finite-field samples in (15) must not hide an unbounded rejection loop in a strict CPU bound. One valid implementation represents a field element by an index in `[|F|]`, samples `ceil(log2|F|)` bits and rejects out-of-range indices. Each attempt succeeds with probability greater than one half (or exactly one for a power-of-two field size). If at most N_s field draws are needed under the polynomial bound, cap each draw at R_s attempts, returning failure on exhaustion. The additional probability is at most `N_s 2^-R_s`, and bit-work is at most `O(N_s R_s ell_F)` plus representation conversion. The detector/kernel constructions yield polynomial N_s; the displayed linear systems need at most O(a(v+1)^2) uniform field coordinates over a full run, up to fixed implementation constants and the LDSample draws.

State which implementation caps are used in E_private and in each reduction and add their union bound to `tau_impl`/the corresponding reduction error. These CPU caps are separate from the identical logical extraction cutoff. Ideal-uniform-sampler equations (1)–(20) hold exactly under that stated sampler model; a strict bit implementation obtains them with its explicit sampling-failure loss. Do not replace exact samples by reduced biased bitstrings.

All bounds are polynomial in the declared `(L,m,lambda_A,w,r,d,log|F|)` and explicit source/output lengths, assuming the cited polynomial constraint detector and bounded samplers. They are PPT in a security parameter only when these declared lengths and L are polynomial in it. A huge concrete or exponentially selected L remains huge reduction time even if the external native key count is small. There is no inference `E[poly(Z)]<=poly(E[Z])` and no conversion of virtual work into free ordinary-SIS computation.

## 11. Conclusion and remaining boundary

The original-transcript/private-continuation proposal is sound for the restricted deterministic extraction predicate, with a full paired record, input-local B, known-WO caching, ordered sticky-failure reconstruction, a common logical clock, and the two complete-run coupling losses. It yields a legal actual-Theta producer preserving the exact original accumulator and original actual external decision. It can remove extraction-tree amplification from **actual native random-function keys**, not from virtual extended-query error, emulator CPU, source dimensions, or reduction runtime.

To apply it to the selected receipt, the actual-fixed-Theta extractor, deterministic guest/source projection, strict budgets, common original source/shape inventory, query-authentication transport theorem, native stream/compiler interface and security parameters must all be instantiated. The remaining zero-finding work is independent; no theorem here asserts persistence of implicit curve values. No concrete-ARO, Q32, succinct-host-work, honest-proving, implementation, or performance-parity approval follows.

This task creates only this research note. No builds, tests, formatters, benchmarks, source-code edits, or git operations were performed. The paired-emulator interface and failure-state boundary were coordinated with the independent ARO repair reviewer; that review is limited to the repaired empty-start complete-run theorem, not the missing authentication or concrete admission.
