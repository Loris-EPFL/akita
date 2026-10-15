# Akita obligations for relation accumulation before PCS proofs

## Status, sources, and reading coverage

**Conclusion:** the safest reuse is one final ordinary heterogeneous Akita batch over the original retained commitments, preceded by a new, explicitly proved relation/query reduction. Original Akita recursively reduces the work of a PCS opening; it is not an online scheme that folds an unbounded sequence of new application relations into its recursive witness. The existing repeated-root native construction is not an admitted baseline theorem.

The primary indexed document is [Akita: A High-Performance Lattice-Based Polynomial Commitment Scheme](SOURCE_PROVENANCE.md#akita), Quang Dao et al., docid `#bfa1a6`. Sequential retrieval covered lines **1–6659**, including references and **all appendices A–I**. A fresh query from 6660 returned only blank line 6660 and no further text: 6659 is the last nonempty line, a long HTML memory table. The resumed reading recovered long-line/security text and the remaining appendices using exact ranges, rather than treating head/tail tool excerpts as complete readings. In particular, the decisive reread ranges were 855–1000, 1363–1580, 1580–2150, 2151–2250, 3121–3245, 3377–3460, 3531–3827, 3951–4068, 4091–4240, and 4751–6659. The preceding sequential reading also covered the overview, challenge/sumcheck preliminaries, witness layouts, range/norm/relation checks, terminal, planner, and distributed/evaluation sections.

**Coverage limitations:** embedded base64 figures are not semantically read merely by retrieving their bytes. Their captions and surrounding explanations were read; the figures themselves remain an image-reading omission. Some extracted mathematical typography is malformed, including superscripts, interval brackets and subscripts; this note does not infer missing equations from those defects. Long public-weight and norm formulas used here were retrieved with wrapping to avoid the display's 768-character truncation. This is full indexed-text coverage, not an independently established full official-PDF concordance.

Local source identities observed during this assignment:

- `Papers/parsed/Akita.md`: SHA-256 `bfa1a6df3810b21d6d786703dcc3c46efedd09b8471dca82c88e8ff0da82df3b`.
- `Papers/Akita.pdf`: SHA-256 `05a483bd047a76e1dcde1335c7541c1d44a4db9ff346fb6b09ab1e967a51c701`.
- [Official ePrint 2026/1983](https://eprint.iacr.org/2026/1983) confirms the title, authors, ordinary batched-opening capability and public abstract. It does **not**, through the retrieved metadata alone, authenticate every detailed theorem in the local indexed snapshot. Its reported benchmark ranges also differ from some local body tables. `ReviewAkitaPrimary` separately reports that the local PDF reproduces the expanded source-uniqueness/mixed-tree clauses, while full official-PDF provenance remains unresolved. Detailed numbered results below therefore identify the **indexed/local snapshot**, not a silently assumed pristine published version.

An archived lexical-search snippet showed a diff hunk before Fig. 8. Present exact range bytes show line 3357 as the plain Fig. 8 caption and 3359 as Section 9.5; no present literal diff-marker corruption is established. The old snippet is a provenance/rendering discrepancy, not proof of document corruption.

Also read [the existing conditional native source-tree note](AKITA_INSTANCE_ACCUMULATION_NATIVE_SOURCE_TREE.md), lines 1–158, and actual ordinary/full-ring API sections cited below. No repository edits, branch changes, builds, tests, linting or formatting were performed. This note is the only file written by this slice.

## 1. Exact ordinary PCS object and decommitment contract

### Interface

Indexed Definitions 3.12–3.15, lines 1417–1467, distinguish:

1. `Setup(1^lambda, mu_max) -> pp`.
2. `Commit(pp, shape, f) -> (cm, st)`, for base-field Boolean evaluations `f: {0,1}^mu -> K`, with public `cm=(shape,C)`.
3. `Eval.P(pp,cm,st,r,v) -> proof`.
4. `Eval.V(pp,cm,r,v,proof) -> accept/reject`, with `r in E^mu`, `v in E`.

Here `K=F_q`, `E=F_(q^k)` and the supported extension degrees are 1, 2, 4. The commitment table consists of Boolean **evaluations**, not the monomial coefficients of its MLE, and not necessarily Boolean-valued entries. The knowledge experiment retains a resettable handle for the **same stateful execution**, auxiliary state and random tape, with consistent oracle replay except at explicitly programmed challenge entries.

The extracted witness need only satisfy an efficiently decidable algebraic decommitment relation containing the honest commitment state. It is not required to be the state produced by rerunning honest `Commit`. Equation (57) appends the authenticated evaluation equation to that relation. Binding, completeness and knowledge soundness are separate properties; one must not claim all three merely from Theorem 10.33.

### Frozen commitment shape

Equation (59), lines 1581–1587, freezes:

`(Src; M,b,b1; (dA,nA); (dB,nB,SB); F-profile)`.

`Src` includes the ordered polynomial count, natural lengths, common commitment grid, coordinate order, honest padding maps, coefficient contract, and actual decoder. A payload without that shape has no defined meaning. Points, values and later opening projections are not commitment-time data. Different source depths require separate handles. Opening the same handle again at another point is another **group occurrence**, not another invocation of `Commit`.

Honest formation packs base-field coefficients into negacyclic rings, divides them into actual `B=ceil(N/M)` blocks, and canonically zero-extends the last block. Full-field inputs use balanced decomposition; bounded sources may use identity encoding. The exact asymmetric representable interval must fit the packed source, not merely an absolute bound. Equations (69)–(74), lines 1825–1927, define:

- `f_i = G_(b,M) s_i`;
- `t_i = A s_i`, with image digits `hat(t_i)` in base `b1`;
- canonical dyadic B slices, each padded inside each polynomial segment;
- `u_s = B x_(B,s)`, with one physical B matrix reused across slices;
- one stacked outer image `u`, optionally compressed once through F.

An algebraic decommitment, Definition 10.13 and equations (199)–(200), lines 3709–3729, includes **all** source blocks, decoded full-grid vectors, inner-image digits and the B/F path satisfying these equations and their certified digit alphabets. It excludes the point, value, partial evaluations and D/H opening path.

**Padding boundary:** the extractor decodes the full committed grid, including cells in the last committed block beyond the declared natural length. Only absent block indices are definitionally zero. Honest canonical padding is not a predicate proved by an ordinary opening. Equations (80), (202), lines 2067–2075 and 3731–3743, authenticate `MLE(f_star)(r)=phi*v`. Natural-domain interpretation additionally needs either a proved padding-image predicate, a shape with no semantic padding, or a separate restriction/opening reduction. Zero padding of a public query-weight table is different from assuming zeros in a malicious committed source.

The Jolt prefix reduction illustrates legitimate reuse: Lemma E.1, lines 5764–5786, uses a fresh selector after fixing all used-slot values; a false used-slot claim gives a nonzero selector discrepancy regardless of unused slots. Its error is `s/|F|`. It does not establish one-hotness, global unused-slot zeros or execution semantics.

## 2. Setup and A/B/F/D/H roles

Equations (65)–(67), lines 1645–1767, use one flat setup vector `S`. Each matrix is a row-major coefficient prefix view:

`M[i,j] = sum_(ell<d) S[(i*m+j)*d+ell] X^ell`.

All views overlap. They are not independently sampled role matrices. Setup provisioning closes a **finite admitted schedule family** first and provisions the power-of-two envelope of the largest native `n*m*d` view. The setup seed expansion is coefficient-indexed and prefix consistent. Its aggregate bias is bounded by `N_setup*q*2^-w`; exact rejection has zero coefficient bias. Twice the modulus width alone does not prove a chosen aggregate setup-error target.

| Role | Bound object | Essential distinction |
|---|---|---|
| A | Gadget/identity source blocks | Native negacyclic inner commitment; weak-source comparison uses cleared challenge denominators, not a short inverse. |
| B | Digits of A images | One physical matrix reused across canonical slices; collision width is one padded slice, not the whole logical stack. |
| F | Negative-binary digits in the outer compression chain | Commitment-side path, part of source projection; complete input width must be priced. |
| D | Complete ordered partial-evaluation digit vector | One **unsliced shared** opening map across all groups; width grows with the full batch. |
| H | Negative-binary opening compression chain | Opening-local and point dependent; not part of a source's persistent decommitment. |

The optional compression has exactly two stages or zero stages, equations (75)–(77), lines 1929–2025. Each stage takes the full negative-binary digitization of its preceding image, with digits in `{-1,0}`. Two valid digit inputs differ by at most one **only because the complete fused binary predicate is proved**. A shared balanced range check by itself cannot justify collision radius one. Lemma 4.3 walks backward from a common payload until the first differing path vector produces a short matrix kernel vector.

The reported 128-byte compressed output is parameter-specific, not a universal bound on arbitrary source widths. Table 5's 8 KiB input limit applies separately to a complete B-image stack and the complete D image. Enlarging a retained-source/query batch may exceed this limit. One cannot preserve the old F/H ranks just because the final desired payload is unchanged.

Lemma 10.11, lines 3667–3681, reduces a shared-view collision to ordinary MSIS by embedding a uniform MSIS challenge into one prefix view and filling the rest of the flat vector uniformly. This handles correlations between roles without inventing an independent-matrix assumption. Corollary 10.12 permits adaptive selection from a public finite family fixed independently of setup, by taking the union of induced instances; it does not allow arbitrary proof-supplied schedules after inspecting setup.

## 3. Challenges: actual rings and fields, not an assumed homomorphism

Section 3.1, lines 867–995, uses `R_(q,d)=F_q[X]/(X^d+1)`, with power-of-two d and an odd prime q. Partial splitting and LS18-type invertibility conditions must be verified for the actual q/d and challenge shell. A random ring element is not generically a unit in a split ring.

At a direct coefficient-packing fold:

- `S=K[Y]/(Y^dcar+1)` is the challenge ring;
- `S_E=E[Y]/(Y^dcar+1)` is the partial-opening carrier;
- `R=K[X]/(X^dA+1)` is the A ring;
- `dA=k*h*dcar`, and `iota(Y)=X^(k*h)`.

Lemma 3.2, equation (47), proves that this specific unital embedding preserves units, coefficient norms and multiplication-operator norm. The partial map L is **S-linear**: `L(iota(c)F)=c L(F)`, equation (91). It need not be full-R-linear. Equations (87)–(94), lines 2153–2207, fix the low-coordinate contraction and use the **same** challenge in the carrier and embedded A rows. An independent ambient challenge would break this extraction interface.

Evaluation trace instead uses a multiplicative subfield embedding and its trace-compatible packing; arbitrary coefficient-slot packing of extension-field elements is only K-linear. The explicit two-bank packing can increase coefficient infinity/one norms by two and Euclidean norm by sqrt(2). Do not apply these conversion factors again to already packed response coefficients. Evaluation-trace challenges are full A-ring challenges, and the receiving fold owns the tensor reduction converting its incoming ordinary claims. The resulting successor remains an ordinary base-field MLE opening, not an untracked tensor-output type.

Compatibility includes `dB | dA` and `dD | k*dcar` for packing, or `dD | dA` for evaluation trace; it does **not** require `dD | dB`. Every claim-block pair receives an independent fixed-family coordinate draw.

Uniform field challenges for sumcheck, row batching, range anchors and public scalar combinations live in the actual opening/challenge field, with error denominators its actual cardinality. The indexed 32-, 64-, 128-bit coefficient profiles use extension degrees 4, 2, 1 for approximately 128-bit opening fields. A frontend field called K and an Akita base field called K are not automatically identical: any proposed physical decoder/coercion must be the actual admitted field-linear map, not a naming-based identification.

## 4. Weak extraction is not source shortness

Definition 10.14, lines 3745–3773, requires an invertible challenge difference rho for each source block and retains the bounded product certificate

`rho*s = r`, `||rho||_1 <= K`, `||r||_inf <= B`,

and, on admitted Euclidean routes, the corresponding operator/Euclidean bounds. It does not imply `||s|| <= B`, canonical digits for s, one-hotness, or padding-image membership.

For two sources under a common A image, clear both denominators:

`z_A = rho' r - rho r' = rho rho' (s-s')`.

Units give nonzeroness when `s != s'`; A annihilates z_A. Young's inequality gives

`||z_A||_inf <= K'*B + K*B'`.

Equations (193)–(196), (203)–(206), lines 3605–3651 and 3775–3811, also supply three legitimate Euclidean pairings. The correct A radius is the maximum mixed occurrence-pair envelope over its **source-comparison class**, including the diagonal and any canonical setup reference. B/F use the maximum cross-profile alphabet diameter, not only the within-opening diameter. D/H remain opening-local. Within one balanced scanning profile B/D radius is `b_star-1`; F/H radius is one. Honest digit bases or honest response grinding thresholds cannot replace verifier-certified adversarial bounds.

**Two distinct comparisons:**

- Lemma 10.15 compares full weak openings for the same fold instance and payloads. Shared D/H agreement is necessary inside a fold-challenge fork to cancel opening-side terms.
- Corollary 10.17 compares **source projections** under the same original commitment/shape, even at different points, values and D/H payloads. It returns common A/B/F decommitments or a bounded source-path/A collision. Enforcing D/H equality across different outer query points would incorrectly reject legitimate openings.

Remark 10.32, lines 4195–4197, explicitly disclaims canonical-state/source-shortness extraction. This boundary is central to all reuse proposals.

## 5. Strong checks, descendant certification and honest closure

One-fold backward extraction, Definition 10.22/Theorem 10.23, lines 3951–3989, needs a flat coordinate-wise pair structure at the fold node **and a complete accepting strong/recursive subtree under every child**. Authenticate the outgoing witness separately below each fold child, derive its exact native relation and verifier-enforced ranges/norms, compare paths, and only then subtract one-coordinate siblings. Outgoing commitments may differ between fold children. Later challenges need not be equal across those children.

The strong checks include the actual quotient-lift or quotient-free ring relations, row batching, scalar opening row, digit range tree, negative-binary support predicate and any response-norm/linking identities. Theorem 10.27 needs interpolation arity `2*d_max` for quotient lifting or `d_max` for quotient-free checking, and degree-three fused-relation sumcheck rounds. Row batching and a multilinear range anchor require **nested** interpolation, not a flat star. Theorem 10.28 recovers each Boolean range residual and complete binary spans. A sumcheck's endpoint is not authenticated until its descendant PCS opening is authenticated.

The indexed text expressly does not use SuperNeo's strong-then-weak theorem as a black box for Akita's interleaved weak fold and later strong checks (lines 4063–4067). That sequential theorem's common intermediate projection is absent at the folding cut. The actual accepting-tree construction supplies the composition.

Definition 9.2, lines 3194–3244, restricts the analyzed ordinary family:

- new application groups occur only at the root;
- existing nonterminal levels 0/1 use coefficient packing and coefficient bounds; later levels use evaluation trace;
- quotient-free checking is only in the later trace suffix, with no setup-offloading group;
- compression can stop once and cannot resume;
- a pending setup opening is checked by a nonterminal successor; none may remain at the terminal;
- all source/target shapes, certified alphabets, group-local profiles and shared D/H layouts close exactly;
- every induced MSIS instance, challenge-family support, query budget, mixed extraction-tree bound and full FS error passes admission.

Honest closure is separate from accepting-transcript soundness. Theorem 10.9/Corollary 10.10 require bounds conditional on each reachable honest history. The response retry error is `E[1_Reach*q_h(tau)^Ntry]`, not a power of an averaged q. Optimized Gaussian/digit models do not prove these premises. Appendices G.1–G.2 offer deterministic envelopes or proved conditional tails; G.3 explicitly leaves the optimized integer-response admission premise modeled; G.4's pointwise digit-energy bound may force larger successor widths/ranks. A slower implementation can use proved deterministic bounds, but must propagate them through the **entire** schedule. Fixing one response depth while leaving dependent witnesses/ranks unchanged is not closure.

Appendix D separates sound fixed-point operator thresholds from proved accepted-support cardinality. Filtering does not preserve unfiltered independent-sign tails automatically. Appendix C's estimator target is a model for particular MSIS instances, not a concrete end-to-end knowledge-security claim against an extractor with unpriced runtime.

## 6. ROM laws and extraction cost

Definitions 3.16–3.18 and Lemmas 3.19/3.22 distinguish flat coordinate forks from nested interpolation. For each actual stage r:

- `e_r = sum_u (k_(r,u)-1)/|S_(r,u)|`;
- flat `b_r = 1 + sum_u(k_(r,u)-1)`;
- nested `b_r = product_u k_(r,u)`.

Equation (225) prices `Tree = product_r b_r`; equation (176) requires a fixed polynomial in security and total **explicit** instance/witness length. A fold's flat block factor is not a substitute for the product across successive folds. Scalar degree-D sumcheck rounds contribute D+1 each. Range anchors, tensor rows and relation rows retain their complete nested factors.

Lemma 3.22's classical ROM statement additionally requires:

1. independent product challenge answers with exact conditional sampling of the **full** oracle answer, including rejected/unused sampler bits;
2. injectively encoded stage inputs binding the complete preceding transcript and earlier challenges;
3. the original execution's selected query/index and prechallenge prefix retained even on suffix-extraction failure;
4. consistent programming/replay at only selected entries, with weighted cheap-failure versus costly matching-continuation accounting;
5. all allowed probes within the actual public query budget.

Lemma 10.2, lines 3399–3423, supplies this law for **separately indexed programmable ideal coordinate streams**. To prescribe an accepted shell value, sample the first-accept position, rejected prefix and unused suffix with their exact conditional distribution; condition Fisher–Yates/bounded-integer/sign encodings including rejected words and unused bits. For fields, similarly condition the rejected/accepted encoding and retain uniform unused suffix bits. Bounded outer filter exhaustion is distinct from inner integer rejection. These are ideal-XOF answer-tape samplers, **not** preimage samplers for a concrete 256-bit SHAKE seed or chained H1610 compiler.

Fixed witness-independent filtering yields uniform accepted challenges conditional on sampler success. Response-dependent grinding does not define a smaller fixed family; charge every probe through Q. Separate proof-of-work gain applies only to local bad events fixed before the candidate predicate/challenge pair is queried; it does not rescale the entire mixed-tree knowledge error.

Equations (223)–(229), lines 4125–4231, give the indexed ordinary bound

`(Q+1)*epsilon_int + Delta_setup + sum_I Adv_MSIS_I(B_I)`.

The MSIS adversaries run the **whole** tree extractor plus setup embedding and collision work, equations (227)–(228). Ordinary MSIS at honest verification time is not the assumption appearing in that reduction. Classical-ROM native knowledge soundness is the stated model; a quantum lattice cost model does not make it QROM security. No concrete-hash knowledge/game-preservation theorem is supplied by these ideal-stream laws.

## 7. What original Akita supports, and what is new

| Construction | Original theorem support | Additional obligation |
|---|---|---|
| One final ordinary batch of retained original `(C,r,v)` claims | Exact intended root multi-group interface, equations (60)–(64), (202), Appendix E.3 | Admit exact retained profiles/group counts/full D/H width; preserve source/padding semantics and original frontend proof obligations. Delaying proofs alone is not nontrivial accumulation. |
| Appending a roster/list or archive | Storage/framing only | No algebraic reduction of relations occurs; do not call this an accumulation scheme. |
| Online weighted query-relation accumulator, then one query PIOP and one ordinary batch | Akita supplies the final authenticated endpoint/decommitment primitive | New query-reduction theorem, whole augmented transcript/tree/error admission and source-only coalescing; detailed construction below. |
| Repeated online partial native root folding into one receiver | Local algebraic fold equations can be reused | New topology, full source/domain/normalization theorem, lawful compiler/tree construction, mixed-profile MSIS repricing, and polynomial-in-total-work extraction. Ordinary root-only theorem does not admit new application groups at later levels. |
| Balanced b-ary tree of genuine relation reductions, b=2/4/8/etc. | Local ordinary primitives, not an automatic N-relation theorem | Prove each node relation and commitment linkage, authenticate leaves to original sources, and compose extraction along depth rather than all siblings sequentially. Arity/depth/tree cost and norm closure must be shown. |
| Archive of independently verified complete node/segment proofs | Ordinary individual proofs can be verified | This is proof aggregation/archive with linear verification; it misses the goal if every segment already has a full PCS proof. A wrapper alone changes neither that fact nor unproved inner assumptions. |
| Hybrid retaining several unproved relation endpoints until one final ordinary PCS | Final batched endpoint authentication is real reuse | Prove the pre-PCS relation reductions and exact query/history roster. Independent final verification must actually check those reductions, not trust prover output. |

Recursive PCS reductions shrink **one opening proof's witness/checks**. N-proof aggregation combines already-complete proofs. Relation accumulation combines pending relation/evaluation obligations **before** their separate PCS proofs. These are three different constructions.

## 8. Finite one-prefix weighted query relation: viable ordinary reuse

This is a new outer query PIOP, not a new Akita lattice folding theorem. It is specifically the finite candidate requested by Main during this slice.

For each retained original source C, store the public query polynomial and target polynomial online:

`q_C(Z,X) = sum_t Z^t eq(u_t,X)`, `y(Z)=sum_t Z^t v_t`.

More generally, use the actual public decoder's adjoint query table in place of `eq(u_t,X)`. If one index t denotes a whole scalar relation involving several sources, let q_C collect that relation's contribution to each source and let v_t be its complete target. Do not assign unrelated queries the same t if cancellation between them would change the intended relation.

An update performs an actual algebraic addition to q/y, preserving original commitments; it need not create a bounded lattice receiver witness. The state/certificate can grow linearly. A coefficient list with no executed relation reduction remains only a roster.

### Required chronology

1. Bind all original commitments and complete frozen shapes/decoders, source domains, public targets, query points, exact ordered index roster, total degree and complete frontend history.
2. Draw one fresh uniform `a` in the stated challenge field (Main proposes K).
3. Prove by one degree-two sumcheck

   `sum_X sum_C q_C(a,X) P_C(X) = y(a)`.

4. At its final point, return the required endpoint `(C,z_C,P_C(z_C))` for **every retained original source**.
5. Run one full original ordinary heterogeneous Akita batch authenticating those endpoints, with the entire outer prefix bound into the same proof's instance/session transcript. Seal and independently verify the complete query PIOP plus PCS certificate.

q_C is public and multilinear in X after fixing a, even though it is a weighted sum of many equality/decoder-adjoint tables. Its MLE is evaluated exactly, with no witness-polynomial commitment homomorphism. Heterogeneous domains can use cylindrical extensions of each physical P_C and zero-supported **public** q_C factors on extra Boolean coordinates. The endpoint is then the original physical polynomial at its native prefix z_C. This does not assume that a malicious source is the canonical zero padding of some smaller polynomial. Arbitrary coefficient repacking or an unproved field/ring decoder is not permitted.

### Derived accepting-tree theorem

**[INFERENCE — explicit new composition proof, conditional on the indexed ordinary theorem and admitted profiles.]** Below every outer interpolation/sumcheck leaf, run the complete ordinary Akita accepting-tree extractor. Compare the resulting **original C source projections** across the outer a and sumcheck forks using Corollary 10.17. Same C and shape/profile yield one common algebraic P_C, or an ordinary bounded A/B/F collision. D/H and endpoint values may legitimately vary across those forks and must not be coalesced.

On the common P_C vectors, reverse the degree-two sumcheck to recover the exact weighted-query identity at a. Then interpolate its Z-discrepancy to force every coefficient relation. If t ranges from 0 to T−1, degree is at most T−1, the a-tree needs T distinct values, and the one-shot scalar error is at most `(T−1)/|K|`; `(T+1)` is a safe conservative tree-factor overbound. With nv common-domain Boolean variables, sumcheck adds at most `2*nv/|E|` and tree factor `3^nv`.

Thus a conservative supplied-tree size is

`(T+1) * 3^nv * Tree(base ordinary PCS)`.

Since `3^nv = (2^nv)^(log_2 3)`, it is polynomial in the explicit domain size. T must be below the actual challenge-field cardinality, and the degree is the **number/index of coefficient relations**, not necessarily the number of segments. A fresh fixed roster can be accumulated before the one draw without repeated native-root extraction. This supplies a strong reduction before the ordinary weak PCS primitive, unlike the problematic repeated native folding interleave.

### Can the original ROM law cover it?

**Yes in the indexed ideal classical-ROM model, with a new augmented-protocol proof/admission ledger; no automatic theorem for the old concrete chained compiler.** The added a and scalar sumcheck answers are ordinary uniform field challenges. Their complete ideal-answer conditional law is covered by the field-encoding construction in Lemma 10.2 and the mixed-tree game of Lemma 3.22. There is no need for an H1610 seed-preimage or a new repeated-native full-answer assumption if this candidate actually uses the existing independently programmable, complete-prefix indexed interface.

The new transcript must not reset before PCS and lose the original seal/history. All original commitments/queries/targets must be fixed before a. Every sumcheck polynomial is bound before its challenge. The final ordinary profile and commitment-side comparison radii must remain fixed across rewinds. Source coalescing is part of the algebraic proof, not an assumption that a malicious commitment had one known canonical short witness before a.

An augmented error ledger can conservatively use

`epsilon_aug <= epsilon_base + degree_Z/|K| + 2*nv/|E|`,

followed by the same global `(Q+1)` classical-ROM charge, setup bias, and matrix inventory. Its MSIS adversaries pay the augmented tree/source-comparison cost. Outer/history/frontend errors remain separate where not represented by this query relation. The old schedule's nominal 128-bit label cannot simply be retained after adding these coins and Q probes; the actual ledger, query budget and finite profile family need admission.

**Norms are untouched only in a precise sense:** the query PIOP operates on public field weights and does not create lattice witness combinations. The original source commitments remain unchanged. Nevertheless one final batch over all retained sources can enlarge the shared D/H path, F/H complete input widths, successor witness and selected schedule. These must be closed and repriced. A row selected for two groups is not an admitted row for 256 groups. If some source is opened under several actual scanning profiles, its comparison-class maximum must cover them all. Proved deterministic completeness bounds remain preferable when optimized response-model assumptions are not approved.

This construction handles **query/linear relation accumulation**. It does not by itself accumulate arbitrary nonlinear VM constraints. Those must first be soundly reduced by the frontends to the exact queries/targets collected here, on the same original sources. Adjacency, RAM-handle continuity, active lengths and signed/noncanonical encodings cannot be replaced by an unproved Boolean-source shortcut.

## 9. Why this avoids the native H1610/T-exponential boundary

The existing native note derives original-source extraction only **conditional on a supplied accepting tree**. It retains Main Boolean/domain normalization and exact source comparisons, and reports the conservative tree product

`B_N(T)=2^(521*T+343) * 3^(262*T+123) * 11^(2*T+1) * 17^2`.

The native archive is not that counterfactual tree. An optional wrapper checking the same verifier inherits its compiler/tree-building obligations. The remaining concrete boundary includes lawful prefix-preserving **full-answer** conditional sampling, original-index retention, sequential bounded consumption, publication in chained history, all stream/global cutoffs and paid replay. Ordinary ideal coordinate streams do not discharge those concrete chained obligations.

The one-prefix query construction instead retains original sources and invokes **one ordinary PCS**, after a single strong outer relation/query proof. Its T factor comes from one coefficient interpolation stage, not T sequential weak/native-root reductions each multiplying the entire later tree. It still pays polynomial extraction work and does not claim practical 128-bit reduction tightness merely because that polynomial is finite. It removes the specific repeated-root exponential-in-T construction and concrete H1610 sampler dependency, provided the implementation really follows the indexed ordinary interface and the augmented protocol is independently reviewed.

A balanced relation tree may also remove linear-depth multiplication, but only with a theorem extracting relevant children using polynomial total node work and coherent original-source linkage; a tree-shaped byte archive does not establish this. The finite one-prefix candidate has the simpler source-identity argument because no new source commitment is needed for q_C.

## 10. Actual API implications and remaining limits

Inspected ordinary reuse surfaces:

- `akita/crates/akita-pcs/src/scheme/mod.rs:55–188`: `setup_prover`, full/narrowed verifier setup, `batched_prove`, and verifier construction. `batched_prove` consumes selected retained commitment handles, resolves a trusted schedule, imports required setup-prefix slots, and produces one ordinary proof.
- `akita/crates/akita-prover/src/opening.rs:17–86`: `SelectedProverOpeningData::from_committed_claims` resolves the full group profile, checks handle/claim counts and shapes and the final producer contract. This is not an unchecked arbitrary new source pipeline.
- `akita/crates/akita-prover/src/opening.rs:161–208`: ordinary root payloads and each group point are absorbed using actual compression geometry. A new outer seal must bind all query metadata as well, not rely on these loops to authenticate metadata they never receive.
- `akita/crates/akita-verifier/src/verifier.rs:11–84`: immutable trusted-catalog admission and independent verifier setup. Proof-supplied rows are not independently admitted merely because a proof carries a selection digest.
- `akita/crates/akita-types/src/full_ring_accumulation.rs:1–7,24–28,103–109`: the native API itself warns that structural/opaque installation bytes do not authenticate producer/decoder/domain admission, VM adjacency, original RAM-handle continuity or active-row obligations.
- `akita/crates/akita-prover/src/protocol/prove/full_ring_accumulation.rs` and `full_ring_root.rs`, and `akita/crates/akita-verifier/src/full_ring_root.rs`: consuming pending owners expose frontend updates, normalization, suffix resumption and terminal finish. They are concrete experimental state-management APIs, not evidence that their different topology is covered by original root-only Akita.

Main reports the current ordinary control selected an actual catalog root ring dimension **512** for nv=14, not 256. No dimension or setup profile should be hardcoded from a prior native prototype or assumed nominal benchmark description.

Remaining prerequisites before calling any candidate admitted:

1. Authenticate the exact ordinary theorem/source revision; current local-PDF concordance is not official full-PDF provenance.
2. Complete independent algebraic/query and classical-ROM review for the new augmented prefix, including adaptive source coalescing and exact decoder domains.
3. Close the exact retained-source group layout/catalog, full D/H path, all F/H widths, successor shapes, certified comparison radii, and query/error budget for 1, 2, 3 and 256 segments.
4. State conditional optimized completeness premises explicitly or choose proved slower envelopes and reclose the schedule.
5. Retain the real frontend relation checks and original signed/noncanonical source semantics; final query consistency is not execution correctness by itself.
6. Distinguish abstract polynomial extraction from a usable numerical reduction at the actual extractor runtime; do not infer concrete knowledge security from an estimator label.
7. Benchmark real upstream workloads without equating different native interfaces or excluding required decoder/query reductions. Appendices H/I explain why nominal capacity, packed openings and batched univariate measurements are not interchangeable.

No new hash/curve homomorphism, canonical-state extraction, inverse-shortness transfer, QROM theorem, or concrete-hash knowledge/game-preservation assumption is asserted here.
