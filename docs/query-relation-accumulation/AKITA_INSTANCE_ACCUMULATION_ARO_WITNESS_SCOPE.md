# ARO witness-circuit scope: counterexample and a narrow emulator repair

## 1. Result and unchanged boundary

**The counterexample is valid against Construction 5.11 and the unrestricted-oracle-circuit statement of CCGOS Theorem 5.4 in the accepted manuscript actually inspected.** Its Boolean-point distinguishing advantage is one. The manuscript's stated normalization does not exclude it. This is a counterexample to that particular emulator guarantee, **not** an impossibility theorem for ARO emulation, a break of native Akita, or a proof that the paper's downstream conclusions have no alternative proofs.

**A narrow replacement is proved and independently reviewed here:** for an **input-local** witness circuit, a **paired-head** emulator satisfies

$$
\Delta_{\mathrm{TV}}(\operatorname{Real},\operatorname{Emulated})
\le \min\{1,v\,2^{-\lambda}\},
$$

where v bounds all extended-verification queries in the complete experiment. It has the required affine conditional-output kernel on feasible states, an efficient ro-only implementation, and explicit query/work accounting. The replacement uses an augmented internal transcript; it is not unchanged Construction 5.11 on the exposed transcript.

**The fixed-state partial-oracle lemma used in §7 survives. The full §7 zero-finding theorem is not established merely by this emulator replacement.** In particular, sparse-graph partial values, even a whole determined nonconstant curve, can change when a new paired head is activated. Section 8 gives an explicit example and identifies the missing fork-transport argument. Thus the existing quotient notes' full zero-finding conclusion must not be reapproved solely by citing the repaired emulator.

All new theorems and examples below are **derived mathematical arguments**, not claims of an author-issued correction. No CH-I/CH-O, new knowledge assumption, salted witness, extra witness bits, different source commitment, concrete-hash realization, or numerical security admission is adopted. Actual-Theta extraction with independent H_o is distinct from the ordinary-SIS reduction that emulates Theta; Section 10 preserves that distinction.

## 2. Primary-source audit and its limits

The inspected primary document is Chen–Chiesa–Gur–O'Connor–Spooner, *Proof-Carrying Data From Arithmetized Random Oracles*, accepted full manuscript dated 24 April 2023:

- [Official Warwick accepted PDF](https://wrap.warwick.ac.uk/id/eprint/176570/1/WRAP-proof-carrying-data-arithmetized-random-oracles-Spooner-2023.pdf); [local PDF](SOURCE_PROVENANCE.md#supplemental-primary-sources); [local extracted text](SOURCE_PROVENANCE.md#supplemental-primary-sources).
- [ePrint 2023/587 record](https://eprint.iacr.org/2023/587), [current PDF endpoint](https://eprint.iacr.org/2023/587.pdf), [version-history endpoint](https://eprint.iacr.org/archive/versions/2023/587).
- [Published chapter DOI](https://doi.org/10.1007/978-3-031-30617-4_13), [publisher record](https://link.springer.com/chapter/10.1007/978-3-031-30617-4_13).

I independently extracted the relevant pages **from the actual local PDF**, rather than relying only on the prior audit note or its neighboring text. PDF pages 24–26 and 28–31 verify Definition 4.1, Definitions 5.1–5.3, Theorem 5.4, Constructions 5.11/5.13, and the disputed proof steps; printed page numbers are one less than these PDF indices. The primary definitions are:

1. **Definition 4.1, printed p.23:** for *all oracle circuits* `B:{0,1}^{m+r}->{0,1}^w`, sample ro first, independently sample each persistent `mu_x`, set `wo(x)=B^ro(x,mu_x)`, and sample a uniform individual-degree-d polynomial extending the complete Boolean graph. The explanatory paragraph explicitly interprets B as an adversarial strategy for learning information about ro. It does not restrict B to input-local calls.
2. **Definition 5.1, p.24:** tr records queries issued by A to the emulated interfaces. The definition does not insert B's hidden internal ro calls into this transcript.
3. **Construction 5.11, pp.27–28:** pass through ro/wo; on a Vhat query, use `V={(x,y,z):(x,y) in tr_ro and (x,z) in tr_wo}` and `P=sum_{b in V} delta_b`; sample a zero-on-the-cube extension of the residual transcript and add P.
4. **Theorem 5.4, pp.24–25:** B is an arbitrary oracle circuit, the advertised sharper loss is `t_Vhat/2^lambda`, and the text's only normalization says an adversary querying `ro(x)` also queries `wo(x)`. Neither converse pairing nor transitive exposure of B's internal calls is stated.
5. **Proof, pp.28–30:** the H1/H2 argument incorrectly identifies two sparse polynomials literally; the BAD estimate uses freshness of an unqueried `ro(x)` despite possible witness-oracle leakage. These are separate issues. The low-degree shift argument can repair the former, but not the latter for arbitrary B.
6. **§5.4, pp.30–34:** the polynomial-time zero-extension sampling machinery is linear-algebraic and does not require input locality of B. It can be reused with an explicitly corrected state contract.
7. **§6, pp.35–36:** Lemma 6.1's perfect `(ro,wo)` simulation is independent of the disputed Vhat-emulation claim. Lemma 6.3 is a composition statement conditional on having a valid emulator. Corollary 6.4 and the cited instantiation of Theorem 6.5 use Theorem 5.4.
8. **§7, pp.38–41:** Claim 7.3 is a fixed-affine-space statement. Claim 7.4 and the following fork proof additionally reason between a pre-challenge prefix and later transcripts; that is a distinct obligation.

**Revision/correction search.** The live ePrint record's raw metadata reports both publication and modification at `2023-04-24T22:00:09+00:00`, history “received 2023-04-24; approved 2023-04-28,” and describes it as a major revision of the EUROCRYPT publication. The publisher record lists first online 15 April 2023 and supplied no correction notice in the retrieved record. Targeted title/emulator/correction and author-manuscript searches did not locate a newer correction. The current ePrint PDF could not be independently downloaded: the reader returned metadata/abstract rather than its PDF body, and direct retrieval returned HTTP 403. Its detailed version-history endpoint likewise returned HTTP 403 on raw retrieval; the EPFL publication endpoint returned HTTP 429. Consequently **I do not claim byte identity with today's ePrint PDF or that no unlocated correction exists**. The diagnosis and proof are pinned to the accepted PDF; no retrieved convention excludes the example. QMD's indexed collection did not contain this CCGOS manuscript, so no unrelated indexed paper was used as a substitute.

## 3. The exact counterexample

Take `m=1`, `lambda=w=2`, `r=0`, `d=2`, and `F=F_5`. Define the one-query circuit

$$B^{\mathrm{ro}}(x)=\mathrm{ro}(1)\qquad(x\in\{0,1\}).$$

Write `r0=ro(0)` and `r1=ro(1)`, represented by their two Boolean coordinates. The adversary:

1. queries `ro(0)`;
2. queries `wo(0)`, obtaining `r1`;
3. queries `Vhat(1,r1,r1)` and outputs one iff the answer is one.

This obeys the paper's stated `ro => wo` normalization. In the real experiment `wo(1)=r1`, so the third query is a true Boolean graph point and its answer is **one for every sampled oracle**. In Construction 5.11 the only paired graph point is `(0,r0,r1)`. Its multilinear indicator contains the factor `1-X_head`, which is zero at the target head 1. Thus P is zero there. Every Z in the zero-on-the-cube extension space is also zero there. The emulator returns **zero for every sampled oracle**.

The distinguishing advantage is one, contradicting the sharper promised bound `1/4`; with three exposed queries it also contradicts the coarse `3/4` bound. The same example with `lambda=w` gives advantage one versus `2^-lambda` for every positive lambda. If a convention requires positive randomness length, take `r=1` and ignore that bit; nothing changes. No finite-table enumeration or off-cube sampling implementation is needed for this proof.

Even `B^ro(x)=ro(x)` gives a separate counterexample to the literal unpaired emulator: a wo-only call reveals y, followed by the true Boolean query `(x,y,y)`, while `tr_ro` is empty. Pairing both interfaces repairs this second example. It does **not** repair the cross-input example: after activating only head 0, a paired emulator still knows no graph point at head 1, although wo(0) reveals ro(1).

The precise falsified claim is: **unchanged Construction 5.11, on its specified exposed transcript, emulates Definition 4.1 for every permitted oracle circuit B with Theorem 5.4's loss.** One could instead expose/close over B's internal query dependency graph, or choose a different emulator. Such changes are not the stated construction, and their termination, query blowup and security need a separate proof. This example does not rule them out.

## 4. Replacement model, state and algorithms

Let `a=m+lambda+w`, `C={0,1}^a`, and let `L_d` be the vector space of formal polynomials of individual degree at most d over F. Assume `d>=2` and that the fixed circuit B is **input-local**:

> On input `(x,mu_x)`, every ro gate of B, if any, queries exactly x. Equivalently, its result is a deterministic function `b(x,mu_x,ro(x))` with the indicated independent persistent randomness.

Ro-free B is included. The restriction is on the declared ideal-oracle profile, not a computational/knowledge assumption. It must be checked for the selected B; calling an arbitrary concrete hash witness “local” is not such a certificate. Public-setup dependence of B is allowed only after fixing an oracle-independent public setup, or after explicitly including all oracle-dependent initialization in the bounded experiment.

The replacement maintains two different records:

- `S`: an internal paired-head map `x -> (y_x,z_x)`, with the witness coins cached in the ro-only version;
- `T`: the persistent exposed Vhat query-answer map, including all internally invoked producer/predicate queries to that interface.

The external adversary's transcript need not contain the unrequested half of a pair. Define

`v_S(b)=1 iff b=(x,y_x,z_x) for x in dom(S)`, and zero elsewhere on C;

$$
P_S(X)=\sum_{x\in\operatorname{dom}S}
 \delta_{(x,y_x,z_x)}(X),\qquad
\mathcal A(S,T)=
\{P\in L_d:P|_{\mathcal C}=v_S,\ 
 P(u)=T(u)\text{ for every }u\in\operatorname{dom}T\}.
$$

**These are simultaneous constraints.** A contradictory Boolean entry in T is not silently overridden by `v_S`. This is stricter and less ambiguous than applying Definition 5.6's union notation to an inconsistent record.

### 4.1 Paired retained-oracle emulator

Start with `S=T=empty` and an unset failure flag.

- On `ro(x)` or `wo(x)`: if x is not paired, obtain **both** retained answers once and insert `(ro(x),wo(x))` into S. Return only the requested component. Repeats return the cached component. This has the exact retained-oracle answer law.
- On a repeated Vhat query u: return the recorded T(u).
- On a new Vhat query u, if not failed: form the residual map `T_s(q)=T(q)-P_S(q)`. If the affine space of degree-d polynomials zero on C and agreeing with `T_s` is empty, set failure. Otherwise sample a uniform evaluation Z(u) from that space using the zero-extension kernel, return `Z(u)+P_S(u)`, and record it in T.
- After failure, retain exact paired ro/wo behavior, return cached Vhat answers on repeats, and return and record the fixed value zero on new Vhat inputs.

The last rule is an explicit totalization, not a cryptographic approximation justified on inconsistent states. The theorem below charges every execution that can reach inconsistency before a distinguishing output to an earlier BAD event. An implementation may detect the same inconsistency immediately after activating a head, but must not change the retained answer or claim that an empty affine distribution was sampled.

The emulator need not compute BAD, enumerate C, or know the full graph. BAD is used only in the proof. The augmented internal transcript/cache is necessary: Definition 5.1's exposed tr alone is not its complete state.

### 4.2 Ro-only implementation

On the first activation of x:

1. obtain `y_x=ro(x)` once;
2. sample `mu_x` independently and uniformly once;
3. evaluate B on `(x,mu_x)`, answering all its oracle gates with cached `y_x`;
4. store `(y_x,z_x,mu_x)`.

No recursive closure is required, even if B repeats its local oracle query many times. No ro call is free when B is ro-free: the pairing still obtains `ro(x)`. This produces the exact joint `(ro,wo)` distribution of Definition 4.1 at activated heads. It changes neither w nor r nor the graph's meaning. The Vhat algorithm is otherwise identical.

## 5. Algebraic lemmas used by the repair

### 5.1 BAD sets and their cardinality

For a finite set `X subset F^a`, let `BAD_X` contain Boolean b for which there is **no** `Q_b in L_2` with Boolean restriction `delta_b` and `Q_b|_X=0`, as in Definitions 5.7–5.8. BAD is monotone in X.

Here is a self-contained proof of `|BAD_X|<=|X|`, avoiding reliance on the problematic probabilistic calculation. Let M be the multilinear polynomial space and `E_X:M->F^X` its evaluation map. If some `p in ker(E_X)` has `p(b)!=0`, then

`Q_b=p*delta_b/p(b)`

has individual degree at most two, vanishes on X, and restricts to delta_b on the Boolean cube. Therefore, for b in BAD_X, the functional `p->p(b)` annihilates `ker(E_X)` and lies in the row span of E_X. Boolean evaluation functionals on M are linearly independent (multilinear indicators are their dual basis). Thus there can be at most `rank(E_X)<=|X|` such b.

### 5.2 Shift lemma, including feasibility

Let `f,g:C->F` agree on `BAD_X`. For every b where they differ choose a corresponding `Q_b` vanishing on X. The finite formal polynomial

`D=sum_{b in C}(f(b)-g(b))*Q_b`

has degree at most two, restricts to `f-g` on C and vanishes on X. Consequently addition of D bijects the degree-d extension spaces of g and f subject to **any common evaluation constraints on X**, including preserving every specified value. It preserves feasibility and the uniform distribution. This is an existence proof; the efficient emulator never constructs the exponentially written sum.

For `X=dom(T) union {u}`, this proves equality of the next-answer distributions whenever two Boolean restrictions agree on BAD_X. Their sparse interpolating polynomials need **not** be literally equal. In particular it repairs the H1/H2 equality argument without asserting the false identity `V union B = BAD_X`: V may contain known graph points outside BAD_X.

### 5.3 Exact kernel output law

For a feasible nonfailed state `(S,T)`, translation by P_S bijects

`{Z in L_d: Z|_C=0 and Z(q)=T(q)-P_S(q) for q in dom(T)}`

with `A(S,T)`. Therefore for **every** `u in F^a`, not only Boolean u,

$$
\operatorname{Law}[M(S,T,u)]
=\operatorname{Law}[P(u):P\mathrel{\gets}\mathcal A(S,T)].
$$

Each query samples the appropriate conditional evaluation; a single explicit dense polynomial is not stored. When S remains fixed, repeated application is exactly lazy sampling one uniform P subject to T. When S changes, this statement is only about the **new** state; it does not assert that the same latent polynomial survives that change. That distinction is essential in §7.

For formal degrees larger than field size, uniform formal polynomials still induce uniform evaluation-code elements because the linear evaluation map has equal-size fibers. The argument concerns observable evaluations, not a unique formal representative.

## 6. Complete adaptive emulation theorem

**Theorem (paired input-local ARO emulation).** Fix the parameters above and input-local B. For any bounded classical adaptive oracle computation A, with oracle-independent initial auxiliary data or fully accounted oracle initialization, making at most v Vhat queries in total, the complete visible output/transcript distributions in

- **Real:** one `Theta=(ro,wo,Vhat)` sampled according to Definition 4.1, and
- **Emulated:** the paired retained-oracle emulator of §4, with the same retained `(ro,wo)` marginal,

have statistical distance at most `min(1,v/2^lambda)`. The same bound holds for the ro-only implementation. A can choose inputs adaptively, make wo-only queries, repeat inputs, interleave sessions, internally rewind other computations, and use independent additional ideal randomness/oracles. All such operations are one serial computation with one persistent emulator state. The claim does not cover quantum oracle queries or free correlated oracle advice.

### 6.1 Exact real/ideal coupling

Conceptually sample ro and all witness coins once. Let f be their full Boolean graph. Real Vhat access has the exact lazy law of a uniform degree-d extension of f conditioned on the previous Vhat record T; this is the real polynomial's ordinary conditional law, or equivalently Lemma 5.5's perfect emulator.

At a proposed new extended query u, let `X=dom(T) union {u}` and declare BAD if some true graph point for an **unpaired head** lies in BAD_X. On a shared history before BAD, f and v_S agree on BAD_X. Section 5.2 therefore makes the real and sparse next-answer distributions identical, with identical feasibility. Couple them with identical draws. Retained answers are coupled identically by the same sampled functions.

If S grows between extended queries, every newly inserted graph point which was already in `BAD_dom(T)` would have triggered BAD when it first entered that monotone BAD set while its head was unpaired. If this has not occurred, the shift lemma with `X=dom(T)` preserves feasibility after activation. Thus the efficient emulator's inconsistent-space branch cannot be the first uncoupled event: an earlier BAD must already have occurred. A failure flag need not itself be observable in this argument.

These rules give the exact real marginal and the exact totalized emulator marginal, with equal visible transcripts until the first BAD. This is an adaptive coupling, not an assertion that unknown ro entries remain uniform after conditioning on the real transcript or on “no BAD.”

### 6.2 First-BAD-point counting without future conditioning

Run the **efficient emulator** for the probability calculation; its conceptual BAD monitor never affects its answers. Let X_j be the first j distinct extended-query locations, and enumerate each Boolean point when it **first enters** the nested sets `BAD_X_j`, using a fixed ordering inside one batch. Section 5.1 implies at most v enumerated slots on every execution. Pad with dummy slots if fewer appear.

At a slot b=(x,y,z), the slot's existence and identity are determined by the emulator's history and proposed extended query, before inspecting the hidden full graph. If x is paired, this slot contributes no BAD event. If x is unpaired, **the emulator history depends only on other heads' ro values and coins and its own kernel randomness**. Input locality is exactly what makes this assertion true, including when other heads were activated through wo-only calls. Hence, conditional on this history and slot identity,

$$
\Pr[\mathrm{ro}(x)=y\land\mathrm{wo}(x)=z]
\le\Pr[\mathrm{ro}(x)=y]=2^{-\lambda}.
$$

A point which first entered BAD while unpaired and false never later becomes a true graph point: the full graph is fixed. A point which first entered while paired cannot later become unpaired. Thus any BAD event is covered by one of these at-most-v slot successes. Linearity of expectation/union bound over the padded slots gives `Pr[BAD]<=v/2^lambda`.

Crucially, this calculation does **not** condition on future BAD-set sizes, earlier hidden BAD outcomes, eventual success, or the real-oracle transcript. Doing so could bias the hidden ro value. Feasibility/failure decisions in the efficient emulator use only S,T and remain part of its ordinary observable-state history.

The identical-until-BAD coupling gives the theorem. The ro-only implementation changes the retained-pair law by zero, so it adds zero statistical error. ∎

## 7. Full query, storage and runtime ledger

Let:

- `h` be the number of distinct heads activated by either exposed ro or wo calls throughout the complete producer/predicate computation;
- `v` bound its distinct Vhat inputs (counting all Vhat calls instead is safe);
- `T_B` bound circuit evaluation/bit work, including its local oracle gates, and let r be its randomness length;
- `C_CD(a,d,k)` bound the actual bit cost of the source's constraint detector on k points for the relevant degree vectors;
- `c_F` bound bit cost of a field operation; `ell_F=ceil(log_2|F|)`.

Then:

1. **Retained interfaces:** at most `h` ro queries and `h` wo queries, at most two on a first head activation, zero on Vhat calls. This replaces, rather than repeats, the original “at most one retained query per emulated query” claim.
2. **Ro-only implementation:** exactly one ro request per new head, hence at most h. B's repeated local gates are CPU work, not distinct external ro calls. Direct ro access is still charged when B has zero oracle gates. Predicates and auxiliary initialization add their fresh heads to h.
3. **Sparse-polynomial evaluation:** at the j-th new Vhat input, computing P_S on all prior points and the new one takes `O(a*h_j*j)` field operations, with `h_j<=h`. Across all v calls this is `O(a*h*v^2)` field operations, or `O(c_F*a*h*v^2)` bit work. This is not generally O(v) scalar work.
4. **Kernel:** reuse the §5.4 decomposition `Z=sum_i X_i(X_i-1)R_i` with the specified individual-degree vectors. For at most j residual observations there are at most `a*j` evaluation variables and `O(a*j)` linear constraints. Uniform affine-solution sampling and Gaussian elimination cost `O((a*j+1)^3)` field operations, plus field randomness. There are O(a) constraint-detector invocations for the old support and O(a) for the support with the new point (the latter via LDSample). A conservative summed bound is

   `sum_{j=1}^v O(a*C_CD(a,d,j) + a*C_CD(a,d,j+1) + c_F*(a*j+1)^3)`.

   It includes consistency detection and uniform sampling, rather than assuming feasibility. Use the **same sign** in the decomposition constraint and final evaluation: the accepted Construction 5.20 display has `x_i(1-x_i)` in its constraint but `x_i(x_i-1)` in its output; the mathematical zero-extension kernel uses one consistent convention. No odd-characteristic implementation should copy both signs literally.
5. **Total simulation work:** add `O(h*(T_B+r))`, input/output serialization, table lookup, randomness generation and the complete original producer work to the preceding sums. A coarse polynomial bound follows, but is not a concrete speed claim. Field rejection-sampling work needs its usual expected-time convention or an independently charged strict cutoff; an abstract uniform field sampler is not a bounded-time concrete bit implementation for arbitrary |F|.
6. **Storage:** persistent pairs/coins cost `O(h*(m+lambda+w+r))` bits; T costs `O(v*(a*ell_F+ell_F))`; Gaussian-elimination workspace can use `O((a*v+1)^2)` field elements, plus the source constraint detector's workspace. The internal paired record and exposed Vhat record must not be reset on a producer's internal fork.

An arbitrary externally supplied inconsistent initial tr is outside the theorem's feasible-output assertion. Conflicting repeated answers, malformed points, and inconsistent cube constraints are rejected/totalized explicitly; there is no uniform distribution on an empty space. A real-oracle prefix can itself lead to an infeasible *sparse* state after the rare bad event, so “real transcript” is not an excuse to omit this branch. The empty-start statistical theorem, and proper-prefix continuation as part of that same complete run, are the guarantees proved here—not a uniform error bound conditional on every possible rare prefix.

### 7.1 Ro-only games and ordinary reductions

Let p be a bounded predicate accessing ro only. Apply the theorem to the complete computation which runs A and then p on A's output. It preserves the winning probability within `v_total/2^lambda`; p adds ro heads/work but no Vhat queries if it truly is ro-only. A coarse total-query loss is also safe. Include any other output tests or extraction predicates which access Vhat in v_total.

After ro-only replacement, ro can be lazily simulated inside a standard-model collision producer, using independent random values and the same single paired/kernel state. This is an ordinary bounded-time SIS/MSIS adversary once the **whole computation** has a strict cutoff and all work above is included. Independent H_o streams and their private programming are internal randomness/operations of this producer. This is straightline execution of the whole producer, even when that producer internally resets its prover.

This corollary recovers the particular game-preservation use of §6 for **input-local profiles**, without granting the ordinary reduction a free ARO. It does not lift a rewinding extractor by CCGOS Lemma 9.1, and does not license replacing only ro while retaining an old incompatible wo/Vhat graph.

## 8. What §7 can reuse—and what remains unproved

### 8.1 Fixed-state partial oracle: exact reusable lemma

For a feasible nonfailed `(S,T)`, define `partial_(S,T)(u)` to be y if every P in A(S,T) has P(u)=y, and undefined otherwise. The domain is **F^a**, not merely the Boolean cube. Since evaluation is a linear map on an affine space, its image is either a singleton or all of F. Uniform P therefore gives a deterministic y in the first case and a uniform field value in the second. This is precisely the algebraic content of Claim 7.3, now with paired S and explicit nonemptiness.

If g has coordinate degree at most ell, put `D=a*d*ell`. When the partial oracle is not total on g(F), choose `P1,P2 in A(S,T)` differing somewhere on g(F). The nonzero polynomial `(P1-P2) composed with g` has degree at most D. Every beta at which the partial oracle is defined along g is a root of that difference. Hence there are at most D such beta, or the trivial |F| bound when D is too large. If it is total on g(F) and `D<|F|`, the determined function is represented by a unique degree-at-most-D polynomial. These conclusions follow from the kernel alone and remain valid.

### 8.2 A missing transport step cannot be replaced by “same kernel”

The proof of Lemma 7.1 compares a prefix before the selected challenge answer with final transcripts in two forked runs. The spaces A(S,T) are **not nested when S grows**: `v_S` assigns zero at all unpaired graph points and later changes one such point to one. Thus fixed-state uniformity does not imply persistence of previously *implicitly determined* values, nor that the two completed branches sample the same prefix-conditioned curve polynomial. Recorded Vhat answers persist; all implicitly determined values need not.

Here is an explicit no-BAD example, even with ro-free B and paired heads. Work over `F_5`, `d=2`, `m=lambda=w=1`, with `B(x)=0`. Fix x0 and define the affine curve

`g(t)=(x0,t,0)`.

Start with no paired heads and issue the sole extended query `g(2)`, receiving eta. The sparse Boolean restriction on the line is zero at t=0 and t=1. Its entire degree-two restriction is therefore determined:

$$p_0(t)=\eta\,t(t-1)/2.$$

Now activate head x0, obtaining `b=ro(x0) in {0,1}`. The new sparse graph is one at g(b) and zero at g(1-b). It remains feasible with the recorded value eta at g(2), and the entire curve is now

$$
p_1(t)=p_0(t)+L_b(t),\qquad
L_0(t)=(t-1)(t-2)/2,\quad L_1(t)=-t(t-2).
$$

Each correction has \(L_b(b)=1\) and \(L_b(1-b)=L_b(2)=0\). Thus \(p_1\ne p_0\), while all recorded extended answers are preserved.

There is **no prior BAD** here. For the sole off-cube point g(2), every Boolean delta can be adjusted to vanish there by a multiple of the zero-on-the-cube polynomial `Y(Y-1)`; hence `BAD_{g(2)}=empty`. This rules out repairing the transport issue merely by saying it is already charged to the emulator's `v/2^lambda` bad event.

This example refutes the implicit persistence shortcut, **not Lemma 7.1's numerical inequality**. It supplies no full zero-finding attack. It shows why reproducing Claim 7.3 and replacing tr with an augmented transcript is insufficient to certify the complete fork proof.

### 8.3 Exact changes needed for a justified zero-finding use

A complete reuse must do all of the following, not just replace an epsilon symbol:

1. Use A(S,T), or another rigorously defined conditional state, and define the partial oracle on F^a only for nonempty spaces. Specify failure branches. Hidden paired heads and their coins belong to the state.
2. Force any output challenge request **before freezing the final kernel state** used to test the output polynomial identity; otherwise its paired-head activation changes the very distribution being analyzed. Interpolation of a degree-D polynomial requires D+1 evaluations in general. Charge the actual support and final evaluation counts.
3. Fork at the **first fresh underlying ro request at cm**, including one caused by a wo-only activation. A later exposed ro(cm) is not fresh. The deterministic common prefix then fixes the same cm in both branches. For input-local B this request is precisely first activation of that head. Count the complete head budget in the raw-bit forking lemma.
4. Prove the prefix-to-final **curve transport/resampling invariant** across subsequent head activations and extended queries. It must justify both §7 cases: a prefix-determined curve staying usable as the fixed nonzero comparison, and a prefix-undetermined value retaining the claimed guessing bound after an adaptive continuation. Section 8.2 shows the strongest obvious invariant is false. A correct substitute needs its own proof and any extra bad-event cost; neither is supplied by Theorem 6 alone.
5. If the two forks are generated with a common private random tape, justify any purported independence of later kernel draws from the first branch's selected f,g. The ordinary forking lemma supplies a fresh suffix of **ro answers**, not automatically fresh suffixes of every other private coin. A conditional uniformity statement at one state does not settle this joint law.
6. Preserve the exact challenge sampler. A lambda-bit raw output is not automatically uniform in F. With a partial sampler whose every field output has at most kappa raw preimages, a fixed s-point set has probability at most `s*kappa/2^lambda`; rejected outputs cannot win. Charge raw-answer collisions separately and do not condition on accepted samples or successful forks to regain uniformity.
7. In the quotient application, replace canonical commitment binding only by the already specified relation-binding comparison of g and the quotient source. Price that ordinary-SIS collision producer at its complete fork/runtime budget. The algebraic weak-state adaptation does not fix the oracle transport question.

The source's Claim 7.4 text also writes a degree `d*ell` before ultimately using `a*d*ell`, and invokes independence between objects produced in the two executions without a complete transition argument. Here all curve bounds use D. None of these points is resolved by the existence of an efficient ZSample routine.

**Disposition:** the §7 **fixed-state partial-oracle kernel is reusable**, and the emulator/game-preservation theorem of Sections 4–7 is complete. The unrestricted §7 theorem and the existing quotient zero-finding adaptation remain **unadmitted as used here** pending a correct fork-transport proof or an independently proved replacement. This is a precise remaining mathematical prerequisite, not a new knowledge assumption and not a claim of universal impossibility. The quotient identity, all-targets-one construction, scaled-MLE identity, source binding inventory, and source/guest projection are separate algebraic results and are not refuted by it.

## 9. Optional salted-WO alternative is not selected

A different sufficient freshness source is a witness oracle of the form

`wo(x)=(sigma_x, B0^ro(x,sigma_x,mu_x))`,

where each `sigma_x` is an independent persistent lambda-bit nonce available only at head x. Paired activation is still required. Even if other B0 calls leak ro(x), an unpaired head's nonce remains independent of the emulator view; the first-BAD-point bound can instead charge guessing sigma_x. This can repair the **emulation** freshness step for cross-input B0 with the corresponding assumptions on nonce access, while all B0-induced ro calls and work must be counted.

It changes the interface and generally adds at least lambda witness bits (unless an already specified witness format contains such independent nonce bits). It is not the original arbitrary B, is not adopted in this note, and does not by itself supply the missing §7 fork transport. No size estimate, source capacity, field/profile admission or concrete graph contract may silently use it.

## 10. Impact map for integration; other notes intentionally unchanged

Only this note is created by this task. The parent owns corrections to the following existing claims.

| Existing note / location | Required narrowing; what remains intact |
|---|---|
| [ARITHMETIZED_ORACLE](AKITA_INSTANCE_ACCUMULATION_ARITHMETIZED_ORACLE.md), §§2, 4, 5–7 | Definition 4.1 still permits general B. Replace the **claimed available emulator** by a separately declared input-local paired profile and this theorem, or leave it unproved. Correct query/CPU and feasible-state wording. §7/§8 zero-finding-dependent soundness needs its additional audit. Lemma 6.1 and the distinction between language-game preservation and straightline extraction remain valid. |
| [PRODUCT_EXTRACTION](AKITA_INSTANCE_ACCUMULATION_PRODUCT_EXTRACTION.md), source contract 6, experiment B, bounded-emulation/ordinary-SIS theorem | Actual-Theta mixed-tree extraction is unchanged. Its ordinary-SIS step requires the repaired profile/emulator, augmented persistent state, cutoff and revised costs; arbitrary polynomial-size B is insufficient for the cited replacement. |
| [SOURCE_FIRST_EXTRACTION](AKITA_INSTANCE_ACCUMULATION_SOURCE_FIRST_EXTRACTION.md), §7.2, Eq. (20)–(21), §8.3/Eq. (27), primary source 2 | The actual-Theta rejection coupling and PCS/source-comparison conclusions are separate and unchanged. Replace unrestricted CCGOS emulation and grouped-call counting by this profile/theorem/ledger before claiming an ordinary-SIS bound. Receipt composition additionally cannot assume the §7 query-authentication bound repaired merely by this substitution. |
| [CHAINED_TRANSCRIPT](AKITA_INSTANCE_ACCUMULATION_CHAINED_TRANSCRIPT.md), §7 ARO-before-chain lift and source summary | The fixed-input component-to-native-stream simulation is unchanged. The preceding ARO-to-ro game step needs input-local paired emulation and complete derived budgets. No concrete hash admission follows. |
| [MIXED_ORACLE](AKITA_INSTANCE_ACCUMULATION_MIXED_ORACLE.md), §§3–4 | Narrow persistent-emulator/ordinary-SIS and native game-lift claims; preserve the fixed-Theta / independent-H_o argument. |
| [ORACLE_QUOTIENT](AKITA_INSTANCE_ACCUMULATION_ORACLE_QUOTIENT.md), §6.2 (ZF), especially proof steps 1, 2, 5–7 and conclusion | Repaired emulation alone is insufficient to retain the full displayed ZF theorem. Mark the fork-transport obligation explicit. Preserve quotient/nonzero identities, relation binding, source-query construction, raw sampler analysis conditional on a fixed root set, and n=1 direct Boolean checking. |
| [QUOTIENT_MLE](AKITA_INSTANCE_ACCUMULATION_QUOTIENT_MLE.md), enlarged-degree ZF application | Preserve the scaled-MLE/evaluation algebra and degree computation; the inherited ZF security conclusion has the same prerequisite as ORACLE_QUOTIENT. |
| [PCS_EXTRACTION_COST](AKITA_INSTANCE_ACCUMULATION_PCS_EXTRACTION_COST.md), primary-source scope and §7.1 | Replace unrestricted Theorems 5.4/6.5 resource claims, one-retained-call shorthand and B multiplier by the paired/local accounting. Native PCS tree geometry and lower bounds are unchanged. |
| [EXTRACTION_BUDGET](AKITA_INSTANCE_ACCUMULATION_EXTRACTION_BUDGET.md), persistent-emulation resource references; [ORACLE_QUERY_COST](AKITA_INSTANCE_ACCUMULATION_ORACLE_QUERY_COST.md), imported security/AS scope | Preserve arithmetic/capacity/tree lower bounds. Their existence does not certify the affected oracle-security theorem. Use the repaired model wherever an actual emulation loss is claimed. |
| [RECEIPT](AKITA_INSTANCE_ACCUMULATION_RECEIPT.md), §§7, 7.1, 9 | Separate proved actual-Theta extraction from conditional ordinary-SIS and still-unadmitted zero-finding authentication. A declared input-local B is an additional profile choice, not something reviewer approval already established. |
| [RECEIPT_REVIEW](AKITA_INSTANCE_ACCUMULATION_RECEIPT_REVIEW.md), prior ARO review and product/quotient review entries; [REVIEW](AKITA_INSTANCE_ACCUMULATION_REVIEW.md), final research summaries; [RECEIPT_SOUNDNESS](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SOUNDNESS.md), AROM update | Historical review claims need an explicit subsequent scope correction; review approval does not cure a missing hypothesis or transport proof. The generic one-shot implication with explicit premises remains valid. |

The [OUTER_PROJECTION](AKITA_INSTANCE_ACCUMULATION_OUTER_PROJECTION.md) deterministic reconstruction/guest obligations and the native design's original K-valued sources, lattice A/B/F handles, matrix-role inventory, pre-PCS update topology and final decider do not rely on this emulator as algebraic premises. Do not rewrite them as broken native protocols. Any summary which additionally advertises completed receipt security must inherit the narrowed oracle premises above.

### 10.1 Actual-Theta extraction versus emulated ordinary-SIS

Fixing one complete sampled Theta and then applying the independent-H_o mixed-tree or source-first coupling is a mathematical conditioning argument. It needs fixed-function replay and independence of H_o, **not** an efficient description of Theta, a simulator for Theta, or input locality of B. Its conclusion remains “a witness, a registered collision, or the separately bounded extraction failure” in that actual environment.

Turning the collision alternative into an ordinary-SIS advantage is a different step. It must run the **entire bounded collision producer**, including all its counterfactual calls, through one valid emulator. Under the profile and theorem here, charge the aggregate emulation event once, then partition by the fixed matrix inventory and embed the ordinary challenge in the same uniform flat setup. Include complete kernel CPU, B evaluation, setup embedding, source comparison, independent-stream generation and cutoff loss. Without an applicable emulator, the actual-Theta collision is not already an ordinary standard-model SIS adversary.

Likewise native language soundness after ARO-to-ro game preservation is a separate application with its full derived producer/predicate budget. It is not established by a witness extraction argument alone. The selected lattice commitments and fields remain unchanged; no proof in this note instantiates Theta with concrete software or admits the proposed outer E_o=F=K^4 profile numerically.

## 11. Evidence and hand-off

This is mathematical research, not implementation. The accepted-PDF definitions/algorithm and relevant proof/kernel passages were read directly; the newer-version search limitations are recorded above. The prior sixteen-table enumeration was **not rerun**, and no build, test, formatter, benchmark, git operation or source-code modification was performed.

AroRepairReview independently read all original 291 lines and the primary definitions/proofs, finding no actionable defect in the scoped emulator, game preservation, work/state contract or curve counterexample. Approval excludes unrestricted B, uniform arbitrary-prefix guarantees, zero-finding transport and concrete realization.

Parent arithmetic checked all ten `(eta,b)` curve changes over F_5, including preservation of the observed value at t=2. Eight explicit degree-two shift polynomials preserve their Boolean delta values and vanish at the prior off-cube query, confirming the stated empty BAD set. These checks do not execute a full ARO emulator or receipt.
