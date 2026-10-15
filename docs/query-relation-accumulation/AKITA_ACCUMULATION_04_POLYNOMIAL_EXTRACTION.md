# 04 — Polynomial extraction, backward soundness, and Fiat–Shamir budgets

## 1. Obligation and conclusion

The desired result is a knowledge-secure Akita accumulation system whose live opening has fixed capacity $N_*$, whose $T$ steps do not cause exponential extraction, and whose final decision authenticates the original zkVM statements and their execution links. “Polynomial” means a **single uniform bound polynomial in $(\lambda,T,N)$**, where $N$ includes explicit source, receiver, hint, and extracted-witness lengths. It does not mean polynomial in the logarithmic variable count, nor one separately chosen polynomial for every fixed $T$.

**Result of this investigation.** The current Akita theorem does **not** establish that result for native long-chain re-entry. It establishes extraction for an admissible fixed schedule with an explicit mixed transcript-tree bound. Repeating a nontrivial fold at every step generally violates that bound. SuperNeo's strong–weak theorem explains projection/uniqueness requirements but does not erase recursive oracle costs or apply directly to Akita's interleaved order.

The preferred native solution is an **independently extractable local bridge**, specified in §4, together with authenticated replay, or a separately justified recursive receipt compiler. This is a fully specified **conditional construction**, not a claimed existing Akita theorem. Its exact missing native lemma is stated in §4.4. Replay makes the conditional extraction recurrence additive and exposes its linear archive/verification cost. Replacing replay with a succinct recursive receipt requires an additional uniform auxiliary-I/O extraction theorem; it must not be silently folded into the native lemma.

An explicit **every-update certified receiver** alternative (§4.5) supplies each counterfactual child's complete bounded opening and evaluation without a history suffix. Clear disclosure makes that child extraction trivial but pays full receiver work at every step and is nonhiding. Receiver certification alone does not establish bounded fresh original sources or authenticate execution/history; full clear local-bridge replay is the conservative direct-witness version.

There is also a fully justified **public-residual atomic arithmetic route** (§7), under explicit classical random-oracle framing and trial-count assumptions. It needs no opening extractor for its public arithmetic. It does not upgrade the original proof system, prove frontend knowledge, or automatically authenticate a compressed history.

A bounded-epoch alternative is specified in §6. It authenticates every cut and pays full refresh/replay costs; knowledge extraction additionally needs the stated adaptive lifting, while acceptance checking can directly replay completed verifier predicates. A fixed maximum depth without authenticated refresh costs is not a solution to unbounded EPT.

Classification used throughout:

- **[PROVED ALGEBRA]**: elementary equations under the stated hypotheses.
- **[SOURCE THEOREM]**: the cited paper's result, with its own scope.
- **[CONDITIONAL CONSTRUCTION]**: an algorithm/security consequence of precisely listed extra guarantees.
- **[OPEN NATIVE LEMMA]**: a guarantee not supplied by current Akita or transferred merely by analogy.

## 2. What the papers actually provide

### 2.1 Akita: polynomial tree admissibility is a premise

Akita §9.3 Eq. (176), §10.2 Eqs. (225)–(227), and Theorem 10.31 give the relevant boundary. For the **actual** stage list, including tensor prefixes, folding, range anchors, relation rows, setup products, internal combinations, and terminal challenges,

$$
\operatorname{Tree}(\mathrm{sch})=\prod_{r\in\operatorname{Rounds}(\mathrm{sch})}b_r,
\qquad
b_r=\begin{cases}
1+\sum_u(k_{r,u}-1),&\text{flat coordinate-wise stage},\\
\prod_u k_{r,u},&\text{nested interpolation stage}.
\end{cases}
$$

Examples from the paper: a fold contributes $1+\sum_g W_{j,g}$; an affine combination of $\Lambda$ claims contributes $\Lambda$; a tensor-row stage contributes $2^\kappa$; an initial range anchor contributes $2^{\mu'}$; relation-row batching contributes $2^{\lceil\log_2 n_{\rm rel}\rceil}$; a degree-$D$ scalar sumcheck round contributes $D+1$. None may be dropped because a verifier circuit is small.

The schedule must satisfy

$$
\operatorname{Tree}(\mathrm{sch})\leq T_{\rm poly}(\lambda,N_{\rm exp,max}),
$$

with $N_{\rm exp,max}$ the total explicit instance-and-witness length. For filtered challenges,

$$
T_{\rm filter}=\max_r\sum_u L_{r,u}/\underline a_{r,u},
$$

where $L$ is the raw-tape cost and $\underline a$ a public acceptance lower bound. These quantities and their reciprocals must be polynomially controlled. The classical-ROM extraction bound is

$$
T_{\rm FS}\leq(Q+1)\operatorname{poly}
(\lambda,|\mathrm{sch}|,\operatorname{Tree}(\mathrm{sch}),T_A,T_{\rm ver},T_{\rm filter}).
$$

This is the paper's polynomial bound, **not** an exact universal call count.

For $T$ repetitions of a step with product factor $B(N)>1$, the evident induction has

$$
R_T\geq B(N)R_{T-1}\quad\text{for the recursively replicated subtree work},
\qquad \operatorname{Tree}_T\geq B(N)^T.
$$

The inequality describes the tree method, not a lower bound against every conceivable extractor. Even $B=2$ gives $2^T$. If $B(N)=N^c$, the tree is $N^{cT}$, not a uniform polynomial in $(T,N)$. Including $T$ in explicit statement length does not make this exponential polynomial.

Theorem 10.23 assumes **descendant-certified** counterfactual outgoing openings. Those are authenticated by the recursive suffix, not supplied for free by the accepted actual successor. The outgoing commitment changes with the fold challenge. Akita explicitly says in §10.2 that SuperNeo's sequential theorem is not used as a black box for this order.

The extracted relation is Akita's algebraic decommitment relation. Definition 10.14 bounds a multiplier times source; Remark 10.32 explicitly does not promise canonical commitment state or a short source recovered after division. Corollary 10.17 gives computational uniqueness of the **source projection**, across allowed profiles and points, not uniqueness of every opening path, hint, or proof byte.

**Sources read:** qmd `lattice-papers/akita.md`, §9.3 `line:3194-3245`, §10.2 `line:3813`, `line:3961`, `line:4067`, `line:4141-4250`; exact equation/theorem matches `line:3225`, `4148`, `4156`, `4159`, `4162`, `4175`, `4181`, `4195`, `4225`, `4227`. See [Akita](SOURCE_PROVENANCE.md#akita).

### 2.2 SuperNeo: adaptive oracle uniqueness, not cost-free composition

SuperNeo Definitions 16–18 require adaptive uniqueness experiments over **fresh independent invocations**. The running time includes all work of the oracle being invoked. Theorem 12 composes a strong reduction followed by a weak reduction using the **same projection** $\phi$.

For this application, a useful projection is the ordered tuple

$$
\phi(u)=\big(P_{\rm id},\mathrm{shape},\mathrm{original\ handles},
\mathrm{source\ maps},\mathrm{source\ comparison\ class}\big),
$$

restricted to data the chosen strong reduction keeps unchanged across its reruns. Opening points and D/H evaluation paths may vary. Binding two extractions to the same receiver alone is insufficient: original handles, map identity, packing, and comparison-class membership must match. Conversely, demanding identical complete hints is stronger than the source theorem and usually false.

Theorem 12 gives EPT for the reductions it composes. Iterating a two-module EPT argument $T$ times without an explicit uniform cost analysis can still produce $R_T=cR_{T-1}+p$ or worse. “Each module has an EPT extractor” is not the desired long-chain theorem.

**Sources read:** qmd `lattice-papers/superneo.md`, `line:855-936`, `line:1365-1460`, exact matches `857`, `859`, `881`, `887`, `929`, `1390`, `1423`, `1441`; [SuperNeo](SOURCE_PROVENANCE.md#superneo), §6 and Appendix B.1.

### 2.3 Lova and SALSAA: extraction norms require an actual checkpoint

Lova's strawman extraction has

$$
\|s\|\leq2\beta_{\mathcal C}\beta',
\qquad
\beta_{\mathcal C}=\max_{c\neq c'}\|(c-c')^{-1}\|.
$$

An invertible difference need not have a small inverse. Lova uses ternary challenges and a parity-sensitive heavy-row argument to find a coordinate difference $\pm1$, not arbitrary $\pm2$ division at an even modulus. Its preliminary bound still has factor two; its **integer** Gram relation $D=S^\top S$, with bounded diagonal, is the additional mechanism that eliminates norm slack. The introductory error is roughly $(2/3)^t$ for its challenge-row parameter $t$; this is not Akita's depth $T$ or an Akita challenge-set theorem.

SALSAA Theorem 7.3 and §7.3 track both honest $\beta$ and extraction $\beta'$, all SIS exit parameters, a norm checkpoint, projection expansion, fold expansion, and decomposition. Setting 3.4 includes $4\beta'^2<q$ and a superpolynomial residue field. Its fixed-shape core uses $r_{\rm acc}=2\ell$ and batches bottom rows anew, rather than retaining linearly many rows. For the theorem's constant structural dimensions it gives proof size $O(\lambda\log^2m/\log\lambda)$ bits, prover $O(m\lambda)$ ring operations, verifier $O(\lambda^2)$ ring operations. These are **SALSAA** costs, not costs demonstrated for the Akita bridge proposed here.

Thus, adopting an exact norm certificate or a SALSAA-style checkpoint is a plausible component repair. It is not a license to replace Akita's original A/B/F equations or to assert polynomial recursive extraction. A field equality $D=S^\top S\bmod q$ without sufficient lifting bounds is not Lova's integer norm certificate.

**Sources read:** qmd `lattice-papers/lova.md`, `line:117-128`, `173-238`, `279-322`, extraction section `367-526` with partial image-corrupted retrieval; exact matches `109`, `119`, `121`, `123`, `127`, `184`, `227`, `373`; [Lova](SOURCE_PROVENANCE.md#lova). qmd `lattice-papers/salsaa.md`, `line:257`, `1297-1362`, `1363-1392`, exact theorem navigation/read at `line:1301` and structure/checkpoint match `1377`; [SALSAA](SOURCE_PROVENANCE.md#salsaa). No claim here relies on the unread tail of an image-truncated extraction proof.

## 3. Relations and challenge/operator ledger

Use the bounded relation coordinated with the completeness obligation:

$$
R_{\rm acc}^{\rm bd}(P,a;w,d).
$$

It requires: a full physical grid of length $N_*$; signed alphabet $\mathcal D_{b_*}$; zeros only at designated tail/alignment positions; an identity commitment-side source map $\delta_{\rm com}=1$ for the receiver; coefficient packing/decoding bijective on **all physical cells**; the original Def. 10.13 A/B/F commitment equations and scheduled digit ranges for supplied $d$; and the declared multilinear evaluation. Noncanonical inner-image/F digits are allowed only where those equations and ranges allow them. No “recommit a canonical witness” operation is permitted as an extraction repair. The original handle must stay the original handle.

Fresh $R_{\rm in}^{\rm bd}$ requires original-handle source bounds **separately**, unless a proved injective identity packing makes a decoded-vector bound sufficient. Neither current Def. 10.14 nor bounded decoded evaluations alone establishes these relations. Initial and final certificates must use this same domain. All-valid-state completeness is distinct from generated-state-only completeness.

Maintain three different ledgers; one scalar “norm bound” is inadequate:

1. **Honest forward expansion:** for centered coefficient $\ell_\infty$ norm, let $\Gamma(c)$ be the operator bound of multiplication by a challenge (e.g. its coefficient $\ell_1$ bound). Then $\|\sum_i c_is_i\|_\infty\leq\sum_i\Gamma(c_i)\|s_i\|_\infty$, followed by the specified exact decomposition/range repair. Check width/capacity costs at every intermediate object.
2. **Inverse/interpolation requirement:** every coordinate difference used in extraction must be a unit in the actual ring, or the extractor must use a separately proved integer/divisibility rule. Generalized interpolation additionally needs the required Vandermonde determinants to be units. Distinct field elements work in the field but are not automatically a valid set of ring challenges under a particular embedding/filter.
3. **Collision radius:** extracted source uniqueness should be proved by cross-multiplying short products, not by falsely asserting inverse multiplication is norm preserving.

For example, for matched A-side openings with bounded products $p=\rho s$, $p'=\rho's'$, and unit multipliers,

$$
z_A=\rho'p-\rho p'=\rho\rho'(s-s'),
\qquad
\|z_A\|_\infty\leq\Gamma(\rho')B_p+\Gamma(\rho)B_{p'}.
$$

If $s\neq s'$ and both multipliers are units, $z_A$ is nonzero in the ring. This elementary bound **does not** bound $s=(\rho-\rho')^{-1}(p-p')$. B-side digit differences have the relevant direct difference bound; F/compression exits require their actual descent/view bounds. Use Akita's $\eta_I$ for every matrix view $I$, including setup and original source-versus-protocol comparisons, and its ordinary-MSIS embedding lemma. Do not substitute an invented single $\eta=2B$ for all exits.

A challenge sampler must freeze: raw distribution; accepted family; version; dimensions; all witness-independent filters; every nonce/retry rule; and per-coordinate $L,\underline a$. A verifier-enforced response bound is a soundness premise. A Gaussian/model tail claimed for honest prover responses is not a soundness premise for an adversarial transcript. Witness-dependent rejection is not a fixed-filter theorem and must be charged as adaptive grinding and separately analyzed.

## 4. Preferred conditional native construction: independent local bridges

### 4.1 State and exact local relation

Public state is $a_t=(P_{\rm id},t,\mathrm{epoch},\mathrm{cm}_t,h_t,\mathrm{boundary}_t)$. Private live state is $(w_t,d_t)$ in $R_{\rm acc}^{\rm bd}$. The prover also retains the original source openings/hints of fresh groups. The history archive stores the complete local transcript, fresh segment statement/proof or frontend witness required by its stated relation, and the bridge proof. Only the archive/receipt is allowed to grow with $T$; fixed live opening is a separate invariant.

Define $J_P$ on the public local statement

$$
j_t=(a_{t-1},\mathrm{segment}_t,\mathrm{fresh\ original\ handles}_t,
\tau_t,a_t)
$$

and witness $(w_{t-1},d_{t-1},\omega_{\rm fresh},w_t,d_t,\zeta_t)$ as follows:

- $R_{\rm acc}^{\rm bd}$ holds for both predecessor and successor;
- $R_{\rm in}^{\rm bd}$ holds for every original fresh handle, with all evaluation claims used by $\tau_t$;
- the verifier-reconstructed Eq. (109) successor, all range/relation reductions, commitment views, digit identities, and source maps in $\tau_t$ match $a_t$ **exactly**;
- the frontend checks not discharged by these opening relations hold; their public program/run/IO data match the segment statement;
- machine-state/memory and index/time/length links match the predecessor boundary; $\zeta_t$ contains whatever actual relation witness is necessary, not a trust bit;
- the receipt record encoding/hash update is correct, binds the complete predecessor and derived successor, and cannot substitute handles, projections, or challenge profiles.

Use a noncircular record convention: compute $h_t=H_{\rm hist}(\mathrm{Enc}({\tt Hist-v},P_{\rm id},\mathrm{run},t,a_{t-1},\mathrm{segment}_t,\mathrm{fresh\ handles}_t,\tau_t,a_t^{-h}))$, where $a_t^{-h}$ contains all successor fields except $h_t$. Freeze that value in $j_t$, then produce $\beta_t$. The receipt/replay authenticates the ordered $(j_t,\beta_t)$ list; $h_t$ is not a claim that $\beta_t$ was verified and does not hash its own as-yet-unproduced proof bytes. Every native challenge must likewise follow a fully framed, acyclic prefix defined by its chosen protocol.

This is a concrete polynomial-time relation when the bounded frontend relation and physical shapes are specified. It does not posit commitment homomorphism. The local bridge must be an argument of knowledge **for this relation**, not merely a proof that one folded response has a small norm.

### 4.2 Algorithms and challenge order

1. **Setup:** bind all shape/ring/source-map/operator/challenge parameters and the frontend statement format in $P$. Publish an independently verifiable initial bounded-domain certificate, or choose a directly checked empty initial state. Distinguish a lifetime security cap from an algebraic recursion-depth cap.
2. **Freeze local input:** authenticate predecessor state and full original fresh group/segment data. Freeze receiving shape, digit widths, and all local commitments that the relevant challenge must follow. No fresh root group may appear later without a newly proved protocol rule.
3. **Native step:** execute the proposed fixed-capacity Eq. (109)/range/relation pipeline using the supplied bounded witnesses and exact hints. Reconstruct the successor publicly. Original Akita root-only application topology does not establish completeness or soundness of this new insertion topology.
4. **Bridge:** generate an **independently fresh**, nonrecursive proof $\beta_t$ for $J_P(j_t;\cdot)$. Domain-separate its random oracle from native step, original proof, digest, setup, and receipt namespaces. Its statement includes the completed native transcript, so it cannot use a future suffix as its sole witness authentication mechanism.
5. **Receipt:** preferred evidence-bearing mode publishes/replays every $(j_t,\beta_t)$ and checks all local statements/links. A succinct recursive receipt may instead prove the same checks, but only subject to §5's additional compiler theorem.
6. **Final:** run the exact final domain decision. A conservative final decision reveals/checks full $(w_T,d_T)$ once, including all original equations/ranges; if it is hidden behind an ordinary proof, that proof needs its own extractor for this bounded relation. Accept only together with the authenticated receipt and initial certificate.

The bridge may be expensive. Its required cost is $C_{\rm br}(\lambda,N)$ prover work, $V_{\rm br}(\lambda,N)$ verifier work, and $S_{\rm br}(\lambda,N)$ bytes per step. No theorem currently establishes that it is cheap enough to beat full-verifier recursive aggregation. A generic NP argument for $J_P$ is a logically adequate conditional instantiation but may simply relocate the original verifier work. That limitation is explicit, not presented as the target efficiency solution.

### 4.3 Why it avoids whole-history branching, conditionally

Assume the bridge system has **adaptive multi-instance witness-extended emulation with auxiliary input/output**, against expected-time adversaries, with a uniform polynomial total-work bound. The emulation must preserve the adversary's published statement/transcript and auxiliary output, while augmenting every accepted independently generated bridge with a witness for its *own* $J_P$. Auxiliary data includes prior proofs, oracle answers, boundary values, program input/output, abort/retry behavior, and chosen next statements. Selective abort must be included in the experiment, not conditioned away.

Let $M$ be the number of distinct accepted archived bridge statements and $L$ their total explicit input/output length. Require a bound of the form

$$
\mathbb E[R_{\rm bridges}]\leq
p(\lambda,N,M,Q_{\rm br},T_A,L),
$$

for one fixed polynomial $p$, counting all adversary and oracle work. For a stronger per-record constant-cost interface, an additive schedule has

$$
R_{t}=R_{t-1}+C_{\rm ext}(\lambda,N,Q_{\rm br},T_A,L_t),
$$

not $bR_{t-1}$. More generally, the uniform multi-instance bound above is the requirement; independently asserting per-record EPT is insufficient to derive it for arbitrary adaptive histories.

Extract every bridge directly from the original history-generating adversary/emulator, with fresh extraction randomness. Do **not** invoke a predecessor extractor to answer a new local bridge query. Match source projections of witnesses extracted at adjoining local statements; unequal full-grid sources for the same comparison class trigger the explicitly budgeted MSIS collision. Proof paths and allowed noncanonical digits need not be identical. Bound initial/final certificates separately.

**[CONDITIONAL CONSTRUCTION]** This yields all $T$ local source/frontend witnesses in polynomial time, plus $O(T)$ link validation. It does not require the one actually supplied $w_T$ to authenticate all counterfactual successors. The bridge provides the missing local independent witness authentication instead.

### 4.4 Exact missing native lemma

**[OPEN NATIVE LEMMA: fixed-shape independent bridge extraction.]** Construct a concrete Akita-compatible bridge for $J_P$ such that, for every adaptive expected-time adversary producing polynomially many independent local invocations under one $P$:

1. accepted statements are augmented with witnesses in the **bounded original-handle** relations above, or a nonzero collision for an enumerated matrix view with its declared $\eta_I$;
2. augmentation preserves the actual public transcript and arbitrary auxiliary input/output, including the chosen successor and next-step state;
3. success/failure is quantified jointly over adaptive fresh independent oracle invocations, with source-projection uniqueness across the exact comparison classes;
4. all oracle work satisfies one polynomial bound in total explicit length, $\lambda$, $M$, and query/adversary costs, independent of history depth except through those quantities;
5. no extraction query assumes a free descendant-certified suffix witness; its cost/probability is paid locally;
6. bridge prover/verifier/communication costs are stated and sufficiently smaller than the displaced verifier work if an efficiency improvement is claimed.

Akita Theorem 10.23 is **not** this lemma: it is a backward algebraic operation given descendant certification. SuperNeo Theorem 12 is **not** this lemma for Akita's existing interleaved order. An added bounded-source certificate repairs the domain mismatch but, without items 2–5, does not repair exponential extraction.

One protocol direction worth pursuing is strong original-source/range certification **before** weak folding under an invariant ordered original-handle projection, followed by a genuinely independent local terminal authentication. Lova/SALSAA suggest norm checkpoints, but the A/B/F maps and source bounds must be proved compatible; simply moving messages can invalidate challenge binding. This direction is concrete because $J_P$ and its required oracle contract are fixed; no proof of such a cheap bridge is claimed here.

### 4.5 Fully certified receiver checkpoints: repairing descendant access explicitly

The following is a different, costly alternative to final-only native deferral. **Every update** publishes a receiver checkpoint $\gamma_t$, not merely a range certificate and not merely an epoch-boundary digest. It proves the exact $R_{\rm acc}^{\rm bd}(P,a_t;w_t,d_t)$, including

$$
\mathrm{Rel}_{\rm com}(\mathrm{cm}_t;w_t,d_t)
\quad\wedge\quad
\mathrm{GridDomain}_P(w_t,d_t)
\quad\wedge\quad
\widetilde w_t(r_t)=v_t.
$$

The checked handle, receiving shape, source maps, point/value, and digit/view identities are exactly the pending endpoint derived by the native step. If setup offloading creates additional pending claims, **each** needs its own exact authenticated endpoint or an explicit combined certificate; certifying only the ordinary opening leaves the setup descendant obligation outstanding.

**Concrete clear instantiation.** Let $\gamma_t=(w_t,d_t)$ and verify the displayed relation deterministically, using the original A/B/F equations, full physical grid, ranges/zeros and MLE evaluation. Extraction is simply reading this accepted witness. Cost is $C_{\rm dom}(P,N_*)$ operations and $|w_t|+|d_t|$ coordinates per update. A linear full-grid scan/MLE evaluation is necessary; matrix/view checking adds the actual scheduled matrix-product costs. This full scan is $\Omega(N_*)$ work, not a constant-time check or a claimed cheap PCS proof. It is nonhiding and exposes the whole live state at every checkpoint.

**Private/compact instantiation.** A standalone finite-schedule proof may replace disclosure only if its extracted relation is this exact bounded original-handle domain **and evaluation**, with an independent extractor and an explicit per-proof tree/filter/FS budget. An ordinary Akita proof extracting only Definition 10.14 does not automatically satisfy this condition. Every proof must bind its chosen endpoint before its own challenge sequence. Its independent verifier cannot defer truth of that endpoint to the later accumulator.

**Challenge order and fork requirement.** Freeze predecessor and fresh source inputs before the native folding challenge; derive the child endpoint; then generate and verify the child checkpoint. In **every accepting counterfactual child queried by the one-fold extractor**, a fresh accepted checkpoint authenticates that child's derived endpoint. Verification of only the checkpoint on the actual published path does not suffice to certify other fork children. Clear disclosure gives local endpoint witnesses directly; private checkpoints require their own fresh extraction randomness and paid oracle work. They must preserve source projections under adaptive independent invocations and cannot obtain their witnesses by traversing the history being accumulated.

Once the full child endpoint is authenticated, Akita's local one-fold algebra can be applied **if** the entire local prefix/strong-check subtree and comparison classes meet Theorem 10.23's hypotheses. Count its finite mixed tree, including range, relation, tensor and any setup subtrees, as $B_{\rm loc}(P,N_{\rm seg},N_*)$. Authenticating those leaves no longer entails a $T$-step suffix. For clear checkpoints and a valid local protocol, the tree work is schematically

$$
R_{\rm local,t}\leq
\operatorname{poly}(\lambda,N_{\rm seg},N_*,B_{\rm loc},Q_t,T_A,T_{\rm filter}),
\qquad
R_{\rm history}\leq\sum_{t=1}^{T}R_{\rm local,t}+R_{\rm matching/link}.
$$

This is polynomial in $(\lambda,T,N_{\rm seg},N_*)$ **provided** $B_{\rm loc}$ and all sampling/emulation costs have one uniform polynomial bound. No $B_{\rm loc}^{\,T}$ factor is incurred by suffix authentication. For private checkpoints, replace the leaf cost by the actual checkpoint extractor cost and prove an adaptive simultaneous emulation bound for all local invocations; nesting an unproved EPT oracle is not an analysis.

There remain three separate issues; they must not be hidden by this recurrence:

1. **Bounded inputs and projection matching.** The local backward algebra still returns a weak predecessor/fresh-source opening. The predecessor's checkpoint supplies a bounded decommitment for its **same original handle**, but a corresponding bounded source certificate is also required for each fresh original handle. A comparison-to-certificate lemma must put these sources in the same source-comparison class, match maps/packing, and prove that disagreement yields a nonzero enumerated matrix-view collision at an adequate radius. A certificate's bounded decoded vector alone does not establish all original-handle source bounds. A multiplication-by-challenge product bound alone does not establish shortness either. Where the certificate supplies a direct strong source, its comparison with the extracted weak source must include the mixed strong/weak cross-multiplier radius, not just the weak/weak radius. Once that matching is justified, the weak opening's evaluations transfer to the bounded source. Without it, checkpoint extraction proves only receiver validity plus weak input knowledge.
2. **Local protocol admissibility.** A fixed-shape re-entry prefix must be shown to be a valid one-fold/local reduction with the claimed fresh root groups and exact successor. This is not obtained by treating the original paper's root-only application groups as arbitrarily insertable later groups. The checkpoint changes endpoint authentication, not completeness, source maps, or shape closure.
3. **History/application/FS.** Full receiver truth is not truth of frontend execution, boundary links, or all prior proof bytes. Replay must check every local record. A compressed receipt still needs §5's compiler contract. The native classical-ROM reduction, adaptive extraction experiment, checkpoint extraction and source-uniqueness comparisons must be quantified together with all retries/queries; simple standalone bounds are not automatically auxiliary-I/O witness-extended emulation.

Thus receiver checkpoints are a precise repair of the **descendant-access** obstruction, not a blanket proof of native long-chain knowledge. The strongest completely conservative version publishes the **whole local bridge witness** of §4.1 and directly verifies $J_P$ on every record. Its extractor reads every accepted witness, making the total relation-verification/extraction cost additive without an FS knowledge assumption for the bridge. This version also proves bounded fresh input sources and the exact old/update/new linkage; if $\zeta_t$ is a completed original proof rather than a VM witness, only that proof's acceptance—not additional VM knowledge—is obtained. Full execution knowledge requires the actual frontend relation witness or its separately proven extractor.

**What this achieves and costs.** Clear checkpoints/full local witness replay provide an evidence-bearing non-ZK alternative with explicit full receiver/source work per step, $T$ archived certificates, and no final-only deferred receiver check saving. A private finite-schedule checkpoint gives a conditional refreshing-accumulation alternative after its source/domain/extraction hypotheses are instantiated. Neither is the intended cheap, private, final-only native scheme. Their benefit for this research obligation is that the formerly free suffix oracle and refresh cost are exposed rather than assumed away.

## 5. Receipts, path versus DAG, and recursive overhead

### 5.1 Path with replay

Replay checks each local record, exact indices, program/run identity, memory/state boundary, IO, challenge profile, and final domain. Its cost is

$$
V_{\rm replay}=\sum_{t=1}^T(V_{\rm local,t}+V_{\rm br,t}+V_{\rm link,t})+V_{\rm final},
$$

and archive size is $\sum_t(|j_t|+S_{\rm br,t})$. This is a real authenticated proof of checks, not only a hash chain. It provides polynomial extraction under §4.3, but **not a succinct single receipt**. The clear final decision is not zero knowledge.

### 5.2 Recursive receipt and the actual modular compiler condition

BCMS distinguishes accumulator verification from final decision; BCLMS distinguishes atomic from split accumulation and interleaves NARK and accumulation extraction. BCLMS §2.2 explicitly requires multi-instance witness-extended emulation with auxiliary input/output for PCD, extraction against expected-time adversaries, and notes that its PCD extraction proceeds layer by layer. In a single-node-per-layer IVC path, its cited single-instance flavor can suffice for that construction; that does not make arbitrary Akita receipt compositions uniformly efficient.

For a recursively compressed receipt, its relation must expose the predecessor receipt/statement, the complete local bridge statement/proof, and all verified links as extractor auxiliary output. The runtime must then be proved. If receipt extraction costs $c$ executions of the prior recursive emulator, the naive recurrence

$$
R_{\rm rec}(T)\leq cR_{\rm rec}(T-1)+p(\lambda,N,T)
$$

is exponential for constant $c>1$. Calling the receipt “modular” or its circuit “constant-size” does not change that recurrence.

**[ADDITIONAL OPEN/ASSUMED COMPILER CONTRACT]** The selected receipt/NARK plus bridge accumulation must satisfy the appropriate BCMS/BCLMS-style witness-extended emulation theorem with a **uniform polynomial total extraction bound**, preserving the auxiliary local records. If a concrete compiler theorem supplies that bound, cite and instantiate every model/index/degree/size premise. Otherwise use replay and report its cost. Do not assert that a ROM proof of a PCS automatically satisfies a standard-model recursive NARK theorem. BCLMS explicitly flags this random-oracle-to-recursion gap in its discussion of PC aggregation versus NARK accumulation.

Halo Infinite proves aggregation/accumulation properties for its additive PCS setting and studies PCD; its group/PCS-specific constructions are not an Akita A/B/F compiler. The relevant takeaway is the distinction between aggregating evaluations and proving knowledge of their original openings, not a transferable post-quantum recursion theorem.

### 5.3 DAG

For a DAG, let $M$ count distinct nodes, $E$ authenticated edges, and $D$ depth. The target bound is $\operatorname{poly}(\lambda,N,M,E)$, not a polynomial only in $D$ that hides exponentially many explicit outputs. Every incoming edge and its ordering/multiplicity must appear in the local relation; duplicate nodes require stable identifiers plus statement equality, not just equal hashes.

Use adaptive multi-instance emulation layerwise or across the replay archive, match sources on every repeated original handle, and charge all resulting work. Memoization may share witnesses for the **same fixed statement/comparison class** once extracted; it cannot identify different challenge-dependent successors in the Akita tree. Without a proved sharing invariant, “turn the extraction tree into a DAG” is just a label.

## 6. Bounded epochs and refresh: a specified costly alternative

A safe available cutover does **not** pretend that arbitrary native re-entry inside an epoch is already a valid Akita schedule. It uses completed segment proofs with their original admissible PCS schedules, or one legal root-only Akita batch of the epoch's original claims, plus all frontend/application checks. Every epoch begins with an empty PCS obligation set and ends by fully discharging that set. Machine boundary state persists; a private unresolved folded opening does not cross the cut.

Let an epoch contain at most $e$ segments. Choose a public legal schedule family with exact maximum tree $K_e$ and $K_e\leq\operatorname{poly}(\lambda,N,e)$. Freeze its shape/root commitment geometry before challenges. Verify the complete epoch frontend, original-handle opening/PCS proofs, and all memory/IO links. The cut record contains $(P_{\rm id},\mathrm{run},\mathrm{epoch},t_{\rm start},t_{\rm end},\mathrm{boundary}_{\rm start},\mathrm{boundary}_{\rm end},\mathrm{IO\ digest},h_{\rm prev},h_{\rm epoch})$ and the epoch verification evidence. **Authenticate every cut by final replay** or a separately proven receipt; a digest or an unauthenticated new short commitment is not authentication.

For $m=\lceil T/e\rceil$ epochs,

$$
R_{\rm epoch,total}\leq\sum_{j=1}^m
(Q_j+1)\operatorname{poly}(\lambda,|\mathrm{sch}_j|,K_e,T_A,T_{{\rm ver},j},T_{{\rm filter},j})
$$

subject to an adaptive multi-instance lifting for the opening/frontend extraction guarantees actually used. The available single-batch Akita theorem alone must not be relabeled the required auxiliary-I/O lifting. Public replay of completed verifier predicates can instead establish **acceptance soundness**, with no claim to frontend witness extraction beyond the original system's theorem.

Refresh costs per epoch are at least

$$
C_{\rm refresh}=V_{\rm full\ PCS,epoch}+V_{\rm frontend/link,epoch}+C_{\rm evidence/cut},
$$

and total replay verification is the sum over epochs. If refresh is proved in an additional argument, add its prover, verifier, proof-size, extraction, and model costs; that extra argument is not free. There are $\Theta(T/e)$ authenticated cuts/evidence objects. This is unbounded-length **replay/checkpointing with periodic full discharge**, not the target of one indefinitely deferred native opening.

For a hypothetical native epoch protocol whose per-step tree product is $B(N)$, the equation $B(N)^e\leq(\lambda N)^c$ requires

$$
e\leq c\log(\lambda N)/\log B(N).
$$

If $B(N)$ is polynomial in $N$, this often allows only constant-size epochs. Even a genuine short-source refresh certificate at a cut does not prove the incoming epoch's history unless that epoch is discharged or certified. Thus neither “take $T$ fixed” nor “reset the counter” solves the unbounded EPT obligation.

## 7. Public residual route: atomic soundness without a knowledge extractor

### 7.1 Predicate and one-step lemma

Fix a finite typed lane catalog and exact decomposition

$$
V_{\rm full}(u)=V_0(u)\ \wedge\ \bigwedge_\ell[A_\ell x_\ell(u)=y_\ell(u)].
$$

The completed proof $u$, all original terminal/norm/challenge checks, and full metadata are public; $V_0$ retains every nondeferred check. Matrices/lane identifiers are fixed by $P$. With common $\alpha\in K^*$,

$$
U'_\ell=U_\ell+\alpha x_\ell(u),\quad
Y'_\ell=Y_\ell+\alpha y_\ell(u),\quad
e'_\ell=e_\ell+\alpha r_\ell(u),
$$

where $e_\ell=A_\ell U_\ell-Y_\ell$ and $r_\ell=A_\ell x_\ell-y_\ell$. Treat the lane family as one direct-sum vector $e,r$ over $K$.

**[PROVED ALGEBRA]** If $(e,r)\neq(0,0)$, at most one $\alpha\in K^*$ satisfies $e+\alpha r=0$. Two solutions would give $(\alpha-\alpha')r=0$, hence $r=0$ and $e=0$. If $e=0,r\neq0$, there is in fact no nonzero solution. Consequently the interactive bad-to-good transition probability is at most $1/(|K|-1)$, or the largest accepted challenge point mass for a nonuniform sampler.

For an honestly initialized zero state, a history containing an invalid input and ending at zero residual has some bad-to-good transition after an invalid input. A union bound over eligible accepted transitions gives $T/(|K|-1)$. It is not generally legitimate to announce the one-step error as the entire adaptively chosen lifetime error. This proves an **atomic public predicate**, not knowledge of a hidden bounded opening.

### 7.2 Classical random oracle: candidates, retries, and digest budget

Use an independent accumulation oracle $H_{\rm acc}$ with injective length-framed input

$$
\mathrm{Enc}({\tt Acc-v},P_{\rm id},\mathrm{run},t,\mathrm{epoch},a_{t-1},u,
\mathrm{lane\ catalog},x(u),y(u),\mathrm{boundary/link\ metadata},\mathrm{nonce}).
$$

The verifier recomputes every $x,y$ and all metadata. Including prover-chosen vectors in the hash without checking their derivation is unsound. The **whole predecessor residual state** must be bound, not only its digest unless digest binding is explicitly reduced.

For the cleanest argument, fix the independent original-proof-oracle table first. Conditioned on any transcript prefix, a complete candidate's $e,r$ is fixed before a fresh Acc answer, even if it uses previous Acc answers. That candidate has at most one bad answer. Distinct candidates need fresh Acc inputs. Repeating one input is not a new independent chance. Responses/norms may not be chosen after its Acc answer in this completed-proof route.

Let $Q_{\rm acc}$ count the adversary's distinct Acc queries, **including abandoned proofs, chosen nonces, retries, all runs/sessions covered by the security experiment, and profile searches**. Let $U_{\rm out}$ count distinct unqueried Acc inputs evaluated by the verifier on the final output/receipt. For a replayed $T$-record output, $U_{\rm out}\leq T$; a single final candidate may require $+1$ even when adversary query count is zero. Then, for uniform $K^*$ answers and exact candidate framing,

$$
\Pr[\mathrm{atomic\ residual\ failure}]
\leq\frac{Q_{\rm acc}+U_{\rm out}}{|K|-1}.
$$

This counts candidates rather than adding a second $T$ factor to adversary queries already counted. A protocol may instead define $Q_{\rm total}$ to include all output/verifier evaluations and write $Q_{\rm total}/(|K|-1)$, but it must say so. Fixed-filter challenge sampling adds sampler point-mass/statistical-bias and raw-tape/query costs. If $(e,r)$ is not determined by the frozen framed input, the argument does not apply.

If digests hide complete candidates/history, add digest binding. For an $h$-bit ideal independent digest oracle and at most $M_d$ distinct digest evaluations, including verifier/output evaluations, the classical pair-collision budget is at most

$$
\varepsilon_{\rm dig}\leq\binom{M_d}{2}2^{-h}.
$$

For a deployed concrete hash, replace this by its appropriate adaptive binding/collision advantage; do not call it an ideal birthday theorem automatically. Add receipt failure $\varepsilon_{\rm rec}$, frontend/original-proof acceptance failure $\varepsilon_{\rm front}$, challenge bias, and any statement-binding failures separately. The receipt must authenticate every $V_0$, derivation, update, link, and final decision. A hash chain is an identifier, not proof that these predicates ran.

### 7.3 Field extension costs and original-proof ceiling

An accumulation-only extension $K/\mathbb F_q$ of degree $k$ can enlarge the public atomic challenge space while keeping original matrices over $\mathbb F_q$. Store each residual/input coordinate as $k$ base-field coordinates; multiplication by fixed base-field matrix entries is componentwise. Thus storage and matrix applications expand by factor $k$; challenge/vector multiplication uses extension arithmetic with its selected multiplication cost $M(k)$ (naively $O(k^2)$ base-field multiplications). Bind the irreducible polynomial/basis and canonical encoding in $P$.

For a classical residual target $2^{-s}$, the arithmetic budget requires roughly

$$
\log_2(|K|-1)\geq s+\log_2(Q_{\rm acc}+U_{\rm out}),
$$

plus the separately allocated digest/receipt/frontend terms. This changes no original SIS assumption when it is only public residual arithmetic, but changes the accumulator arithmetic/encoding and its receipt circuit cost.

It does **not** enlarge original fp128 segment sumcheck/FS challenge domains, original native ring challenges, or the original PCS proof's knowledge strength. Akita Corollary 10.34 Eqs. (230)–(231) records

$$
\lambda_{\rm FS}^{\rm eff}=-\log_2((Q_{\max}+1)\varepsilon_{\rm int}),
\qquad
\lambda_{\rm FS}^{\rm eff}\leq\log_2|E|-\log_2c-\log_2(Q_{\max}+1)
$$

whenever $\varepsilon_{\rm int}$ contains $c/|E|$. Enlarging only $K$ cannot repair this separate original-proof ceiling. Original root commitment geometry is fixed when opening; successor compression/setup obligations must be included before scoring any profile.

## 8. Unified native security/runtime ledger

For each chosen native/bridge/frontend/receipt module enumerate:

| Item | Required bound and accounting |
|---|---|
| Explicit shapes | $N_*,N_{\rm fresh},N_{\rm hints},N_{\rm exp,max}$; every intermediate width, padding and physical source block |
| Original/native interactive error | Actual Akita Eq. (223) sum, including all tensor/range/relation/setup/terminal/application terms; no use of only fold error |
| Native classical FS | $(Q_{\rm native}+1)\varepsilon_{\rm int}$ for an admissible schedule or the public-family maximum theorem; every nonce probe counted |
| FS filters | Exact accepted family, $L/\underline a$, conditional tape programming, witness-independent versus adaptive rejection |
| Matrix collisions | $\sum_{I\in\mathcal I_P}\operatorname{Adv}^{\rm MSIS}_{I,\eta_I}(T_{B_I})$ at reduction runtime; include original and successor/setup views |
| Setup | $\Delta_{\rm setup}$ for idealization, counted once for a shared experiment when the reduction justifies this |
| Bridge | Adaptive multi-instance failure $\varepsilon_{\rm br}(M,Q,\lambda,N)$ and **total** emulator runtime; not merely local single-instance soundness |
| Domain | Initial/final source/domain-certificate failures $\varepsilon_{\rm init},\varepsilon_{\rm final}$; no weak-to-short-source implication |
| Frontend/execution | Original proof/frontend knowledge or acceptance error, and all boundary/IO binding errors |
| Receipt/digest | $\varepsilon_{\rm rec}+\varepsilon_{\rm dig}$ with auxiliary-output extraction and output-query budgets |
| Native tree | Exact $\prod b_r$; replay bridges give independent local extraction, not cancellation of this product for the original protocol |
| Lifetime | Actual $T,M,E$, all session/retry/query totals; per-step errors may accumulate |

Under the stated independent-bridge/replay hypotheses a conservative union ledger is

$$
\varepsilon_{\rm total}\leq
\varepsilon_{\rm br}+\varepsilon_{\rm front}+\varepsilon_{\rm init}
+\varepsilon_{\rm final}+\varepsilon_{\rm rec}+\varepsilon_{\rm dig}
+\Delta_{\rm setup}+\sum_I\operatorname{Adv}^{\rm MSIS}_{I,\eta_I}(T_{B_I}),
$$

with interactive/FS errors included **inside** their corresponding instantiated module term, not counted twice. For separate epoch/module reductions with errors $\varepsilon_j$, replace their term by a justified joint bound or $\sum_j\varepsilon_j$; do not assert independence to multiply them. Collision advantages must be evaluated at the extractor/reduction runtime, not the prover runtime. A negligible local term yields negligible total error for polynomially bounded $T$; fixed finite field/query budgets do not give a fixed numerical security level for an unlimited lifetime.

“Unbounded” here means no constant recursion-depth ceiling in the uniform polynomial theorem, with polynomially bounded adversaries/output lengths and declared concrete lifetime budgets. It cannot mean infinitely many adaptive trials against fixed finite challenges with the same numerical failure probability.

## 9. Classical FS versus QROM: what can and cannot be claimed

### 9.1 Classical special soundness controls query loss, not tree size

Attema–Fehr–Klooß, *Fiat–Shamir Transformation of Multi-Round Interactive Proofs*, J. Cryptology 36:36 (2023), Theorem 2 and introduction, prove a $(Q+1)\kappa$ knowledge-error bound for their multi-round special-sound class. Their extractor uses expected $K+Q(K-1)$ queries with $K=\prod_i k_i$ in that theorem's setting. Their useful-subtree check avoids an additional naive $Q^{\mu}$ runtime factor, but does **not** eliminate the necessary transcript product $K$.

Akita supplies its own mixed coordinate-wise/nested-tree adaptation and states a polynomial Eq. (227) bound rather than the exact AFK query formula. AFK's favorable classical query loss therefore does not transform $B^T$ into polynomial work. AFK also gives negative results for parallel repetition; multiplying an error by repetition without instantiating the correct FS theorem is unsafe.

Primary publication read: [AFK extended version](https://ir.cwi.nl/pub/33324/33324.pdf), DOI [10.1007/s00145-023-09478-y](https://doi.org/10.1007/s00145-023-09478-y), pp. 2–6 and Theorem 2 discussion. For the complementary distinction between state restoration and ordinary interactive soundness, see Holmgren–Lombardi, [*On Round-by-Round Soundness and State Restoration Attacks*, ePrint 2019/1261](https://eprint.iacr.org/2019/1261); only its primary abstract/search result was inspected, and no theorem from its full text is used here.

### 9.2 Quantum search: a necessary budget warning, not an instantiated Akita theorem

For a family of complete fixed-input candidates with a reversibly computable predicate marking a fraction $p$ as bad-to-good, Grover amplitude amplification gives success

$$
\sin^2((2q_G+1)\arcsin\sqrt p)
$$

in the standard search model. At small $q_G\sqrt p$ this scales as $\Theta(q_G^2p)$, and constant success requires $\Theta(1/\sqrt p)$ search iterations. Zalka proves optimality in the relevant oracle-search setting. Candidate generation, a reversible oracle call, residual-predicate evaluation, uncomputation, and access to a sufficiently large candidate family all have to be charged. A family of different nonces gives a search space only when it is otherwise a valid fixed candidate and at least one challenge answer actually permits acceptance; the algebraic lemma gives **at most one**, not existence for every malformed proof.

Thus, if a concrete residual attack family realizes $p\approx1/(|K|-1)$, preserving a small attack success against $Q_q$ coherent query work requires a rough sizing warning $\log_2|K|\gtrsim s+2\log_2Q_q$, plus constants and the entire remaining ledger. This is a **search-family analysis/necessary precaution**, not a proved $O(Q_q^2/|K|)$ upper bound for arbitrary adaptive recursive Acc adversaries. A general QROM upper bound needs the precise oracle/query/output experiment, candidate determination under quantum side information, digest binding, and a valid reduction; none is instantiated here for deployed Akita.

Primary source read: [Zalka, arXiv:quant-ph/9711070](https://arxiv.org/abs/quant-ph/9711070), v2 (1999), published [Phys. Rev. A 60, 2746](https://doi.org/10.1103/PhysRevA.60.2746); abstract and full-paper optimal-search discussion. The retrieved conversion has a misleading later date inside its rendered header; the arXiv version/publication metadata, not that header, identifies the citation.

The classical digest birthday term is likewise not a quantum bound. Brassard–Høyer–Tapp show an $O((N/r)^{1/3})$-evaluation collision algorithm for promised $r$-to-one black-box functions; this is a warning about quantum collision budgets, **not** a theorem that this exact promised-function cost or a universal $Q_q^3/2^h$ bound has been proved for this concrete receipt/hash composition. Primary [arXiv:quant-ph/9705002](https://arxiv.org/abs/quant-ph/9705002), v1 (1997), abstract inspected. Use a concrete quantum digest-binding advantage and its reduction, not the classical birthday expression.

### 9.3 Quantum FS and extraction: missing prerequisites remain

Don–Fehr–Majenz–Schaffner (DFMS), [*Security of the Fiat–Shamir Transformation in the Quantum Random-Oracle Model*](https://arxiv.org/abs/1902.07556), arXiv v4 (2020), CRYPTO 2019 version, establish quantum FS preservation for the appropriate interactive quantum soundness/knowledge notions, not by classical rewinding. Classical extractors cannot copy arbitrary quantum prover state; classical MSIS hardness assumptions plus a classical Akita extractor are not those prerequisites.

Don–Fehr–Majenz (DFM), [*The Measure-and-Reprogram Technique 2.0: Multi-Round Fiat–Shamir and More*](https://arxiv.org/abs/2003.05207), arXiv v3 (2022), §3 Theorems 2–3 and §§4–5, give the improved single-input $(2Q_q+1)^2$ multiplicative loss, general multi-input $(2Q_q+1)^{2n}$ loss, and a time-ordered reduction with additional order/error accounting. Their intro describes generic multi-round $O(Q_q^{2n})$ losses and Grover lower bounds. These are not a linear classical $(Q+1)$ result, and with growing $n$ they are not a uniform polynomial-loss solution by themselves.

A QROM claim for this proposal needs all of the following, none currently supplied as an Akita instantiation here:

- quantum knowledge/uniqueness for the original source/range/A/B/F relation and required collision reductions at declared radii;
- a quantum, auxiliary-I/O-compatible independent-bridge or recursive-receipt extractor with uniform total cost;
- applicability of a chosen multi-round FS theorem to the exact domain-separated, filtered/retry transcript (with its explicit round/query losses);
- quantum digest/statement binding and coherent setup/FS oracle modeling;
- final bounded-domain extraction, frontend/VM knowledge, and authenticated history in the same quantum model.

An accumulation-only $K$ extension addresses at most one public arithmetic challenge space. It does not supply quantum sumcheck soundness for original fp128 statements, quantum Akita PoK, or a recursive QROM receipt theorem. A lattice estimator's nominal quantum SIS work factor addresses only its lattice attack model, not these missing reductions.

## 10. Failure cases and integration requirements

1. **Unit inverse counterexample:** invertibility of $\Delta$ gives an algebraic source but may make its centered norm nearly the modulus. Division is not a shortness proof.
2. **Actual-successor-only extraction:** a valid actual terminal witness says nothing about the counterfactual successor commitments required by coordinate sibling interpolation. Calling its descendants free recreates the missing theorem.
3. **Wrong projection:** identical receiving shapes or commitments do not authenticate original handles/maps. Conversely full hint equality is not required source uniqueness.
4. **Postchallenge source change:** allowing a fresh group, shape, alphabet, source map, or nonce-dependent response after the challenge can defeat the frozen-candidate argument or change the relevant interpolation family.
5. **Free refresh:** reinitializing with an unverified short commitment discards unresolved obligations. Full discharge/certified cut evidence and its cost are mandatory.
6. **Free receipt:** a digest does not establish $V_0$, domain reductions, or VM links; recursion over a concretely arithmetized hash is not automatically a ROM recursive compiler.
7. **Unsupported QROM inheritance:** no current classical Akita theorem plus fp128/MSIS parameter choice proves quantum 128-bit accumulation/recursive knowledge security.

**Integration interface:** distinguish `R_acc^bd`, `R_in^bd`, weak algebraic `R_weak`, and frontend execution relation. A final ordinary opening must explicitly establish the bounded relation or be accompanied by the exact full-domain decision. Keep supplied noncanonical digits where allowed; no recommitment. A local step/bridge binds the original ordered handles, physical source map, commitment view, completed transcript, full predecessor, derived successor, program/run/IO and boundary. The receipt authenticates all records plus initial/final domain checks. The extraction proof must select one of: independent bridges with replay; a concretely instantiated uniform recursive compiler; or full-discharge epochs with authenticated cuts. The existing global Akita tree is not a fourth polynomial long-chain option.

For the certified-output interface, require an independently accepted receiver checkpoint on **every** update and every extraction fork child, covering all pending ordinary/setup endpoints, exact A/B/F/full-grid domain and evaluation. In this document $\gamma_t$ denotes that receiver certificate and $\beta_t$ the stronger local bridge; either notation may be renamed by integration, but their scopes must not be conflated. A receiver-only certificate repairs suffix access; its source-matching/local-protocol/emulation premises remain explicit. A full clear local bridge plus replay has direct witness extraction at full per-step verification/communication cost.

## 11. Source and validation record

Local primary papers were navigated and read through qmd MCP; addresses are recorded in §§2–3. Case-correct links: [Akita](SOURCE_PROVENANCE.md#akita), [SuperNeo](SOURCE_PROVENANCE.md#superneo), [Lova](SOURCE_PROVENANCE.md#lova), [SALSAA](SOURCE_PROVENANCE.md#salsaa). The predecessor [notes](AKITA_ACCUMULATION_NOTES.md) were used as a baseline/reference locator, not as proof of the proposed construction.

Primary accumulation/recursive publications read:

- Bünz–Chiesa–Mishra–Spooner (BCMS), *Recursive Proof Composition from Accumulation Schemes*, TCC 2020, [published chapter](https://link.springer.com/content/pdf/10.1007/978-3-030-64378-2_1.pdf), [ePrint 2020/499](https://eprint.iacr.org/2020/499). Published abstract/technical overview inspected for atomic accumulation and the distinction between cheap accumulation verification and terminal decision. No claim that its instantiation applies to Akita.
- Bünz–Chiesa–Lin–Mishra–Spooner (BCLMS), *Proof-Carrying Data without Succinct Arguments*, CRYPTO 2021, [published chapter](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_24.pdf), [ePrint 2020/1618](https://eprint.iacr.org/2020/1618). Published §2.1 Definitions 1–2, §2.2, Remarks 1–3 and standard-model-versus-ROM caveat inspected, including auxiliary-I/O/multi-instance and expected-time extraction.
- Boneh–Drake–Fisch–Gabizon (Halo Infinite), *Proof-Carrying Data from Additive Polynomial Commitments*, CRYPTO 2021, [published chapter](https://link.springer.com/content/pdf/10.1007/978-3-030-84242-0_23.pdf), [ePrint 2020/1536](https://eprint.iacr.org/2020/1536). Abstract/technical overview and batch-opening/knowledge distinction inspected. No lattice-native or QROM compiler theorem attributed to it.

The AFK, DFMS, DFM, Zalka and BHT primary links, versions, sections and limits are recorded in §9. Secondary summaries were used only to locate these sources.

**Validation actually exercised:** document/source retrieval; exact Akita tree/filter/runtime statements and strong–weak oracle requirements checked against their text; symbolic derivation of the residual at-most-one-root lemma, cross-multiplied collision norm, exponential/additive recurrences, epoch depth inequality, and classical candidate/output-query union budget. No numerical benchmark, deployed protocol execution, implementation, branch change, build, test, lint, formatter, or quantum/native extractor verification was performed. qmd singleton addresses sometimes returned excessively long/truncated tails, and embedded image data interrupted part of Lova's extraction-section retrieval; those tool limitations were reported and narrower exact theorem matches were used. The unproved bridge/recursive/QROM obligations are deliberately not presented as verified results.
