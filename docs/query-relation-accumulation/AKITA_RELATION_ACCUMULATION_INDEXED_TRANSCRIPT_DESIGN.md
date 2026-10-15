# AQRA: direct indexed transcript and finite raw-family design

## Decision and reason

This is a required protocol change for Candidate 1, not a reinterpretation of the existing proof bytes. [The independent sampler review](AKITA_RELATION_ACCUMULATION_INDEXED_RAW_SAMPLER.md) establishes two incompatibilities: one 256-bit seed cannot represent the independent shell product, and the pinned ordinary Blake2b post-squeeze state omits the squeezed answer. These do not demonstrate a practical Akita attack. They invalidate the proposed full-answer/descendant-key proof interface for that implementation.

Keep original sources, A/B/F commitments, signed sparse shell families, carrier embeddings, native equations, proof payload codecs, response nonce ranges and suffix arithmetic. Replace the ordinary public challenge boundary with a directly indexed finite raw family and completed-result history. Do not ask for SHAKE seed preimages. Do not add a concrete-hash knowledge assumption. The proof model is the approved programmable classical indexed ideal ROM; SHAKE256 over the specified literal keys is its conventional concrete instantiation, not a theorem about concrete SHAKE or QROM.

The initial AQRA arithmetic is $E=K$. Its baseline planner can select unfiltered Linf suffixes and direct setup; this is an explicit cost choice, not permission to discard a selected row's L2, setup or other predicates. If another row is selected, all its real methods and parameters remain mandatory. Source profiles are frozen; successor profiles and the trusted final row are planned before challenges.

## 1. Installation and canonical keys

An externally approved installation fixes protocol version, field/canonical encoding, source contracts, exact trusted catalog/registry, sampler descriptor, finite horizon, site grammar and resource policy. An installed unique index may refer to this fixed data. An arbitrary prover digest is not an installation. Retain a separately stated collision term if digest identities are used in place of an installed exact reference.

For ordered canonical history $P$ and request $R$, define

$$\mathsf{Key}(P,R)=\operatorname{Encode}(\mathsf{domain},\mathsf{version},\mathsf{installation},\mathsf{session},\mathsf{instance},P,R).$$

Use fixed tags, fixed-width integers and length-delimited variable fields. Counts, lengths and all products are checked before allocation. Bind the entire sealed frontend/query context and endpoint roster into the ordinary PCS context; a reset to a short independent session is invalid. Each request specifies operation, stage, fold level, invocation, round, group/claim/block/limb identities, field or shell descriptor and requested raw length. Request order is verified against the installed grammar, not inferred from optional logs.

$P$ contains every prior public/proof message and canonical completed challenge result. A completed result is sufficient for the decoded-mass old-key bound; the complete raw answer remains in the formal saved oracle table. This convention is deliberate. It avoids appending gigabytes of unused raw allocation to future keys. A failure poisons the current proof computation; it cannot continue under another interpretation of the same request.

For SHAKE implementation, incremental absorption and cloning a SHAKE state are permitted only when they produce exactly SHAKE256 over this literal encoding. Replacing the key by a short seed or digest before expansion is not this construction. Cache the prefix's hash state, not an alternative seed-to-challenge map.

## 2. Field and predicate families

Choose public canonical-rejection cap $H_f=128$ for each field limb. One request returns $H_f\cdot\mathsf{NUM\_BYTES}$ iid bytes per limb. Mask excess high bits exactly as the field codec requires; return the first canonical candidate or fail after $H_f$. For the A7F7 prime $q=2^{128}-2^{32}+22537$, completion-conditioned scalar output is uniform in $K$, with failure

$$\delta_f=(1-q/2^{128})^{H_f}.$$

Each limb's allocation is independent. Append the canonical scalar/vector result and its owner to $P$ before another request. Rejected bytes and unused slots remain part of the full raw answer; they are not zeros. Source probes use the actual cap/failure law. No extension is used by the initial profile; selecting one requires a true field and compatible original-point/encoding proof.

A nonzero PoW site first transports the canonical nonce, then requests a separate 32-byte predicate family with its own domain. Apply the actual low-bit predicate. Append its complete predicate result before the scalar/shell request. Do not reuse predicate bytes as an allegedly fresh field root. Zero-bit PoW has no nonce/predicate event when the installed plan says so. The verifier checks the selected nonce, not the honest prover's firstness; count all attempted original oracle entries. The 12-bit FoldResponse nonce remains witness-dependent prover search, not a built-in uniform accepted-shell filter.

## 3. One atomic shell-level family

Immediately after a FoldResponse nonce, freeze one descriptor containing the **whole level's** ordered groups and all claim-major block coordinates. Let $W=\sum_g P_gB_g$. One indexed request owns all $W$ coordinates; no independent finite seed expands them. Both preview and live draws consume the same descriptor and raw-family decoder. Append the complete canonical ring-polynomial tuple only after the family finishes, before successor/public strong messages.

For coordinate $c$, let carrier $d_c$, shell counts $(n_{1,c},n_{2,c})$, weight $w_c=n_{1,c}+n_{2,c}$, fixed integer filter and candidate cap $J_c$ be installed. Set $J_c=1$ without a filter and $J_c=4096$ for the existing fixed operator predicate. Use a public per-range mask cap $H=128$.

A candidate has one independent fixed slot of $H\ell(d_c-i)$ bytes at each Fisher–Yates step $i<w_c$, where $\ell(m)$ is the actual 1/2/4-byte range width and $\ell(1)=0$. Decode the first accepted masked word in that slot; exhausted slots fail the family. The rest of the slot remains raw slack. Then consume one byte per sign, using bit zero and retaining the seven slack bits. This is a real versioned decoder change from the former variable-position seed stream.

Allocate candidates in **attempt-major, then canonical coordinate order**. A coordinate with $J_c$ has slots only for attempts $0\le t<J_c$. The total finite raw length is

$$L_{\rm shell}=\sum_c J_c\left(H\sum_{i=0}^{w_c-1}\ell(d_c-i)+w_c\right).$$

At attempt $t$, decode unfinished coordinates and skip that attempt's reserved slots for already completed coordinates. Once all coordinates complete, the unconsumed finite answer tail need not be computed by the honest executable. SHAKE's reader still defines exactly the declared finite answer. Never derive another coordinate from a short root seed. This layout avoids scanning every one of 4096 reserved candidates after an early success. Computation, conditional sampling and any adversarial reads of unused bytes remain charged in the reduction.

Compare/serialize **ring polynomials**, not position-vector order: distinct permutations can encode the same polynomial and have zero difference. Canonical sorted support/sign representation or the full carrier coefficient vector is sufficient. Validate shell counts, distinct positions and the exact filter before embedding into the declared native role ring.

## 4. Exact conditional full-answer sampler

For a masked range $m>1$, with $b=\lceil\log_2m\rceil$, $p=m/2^b$, accepted target $a<m$ and cap $H$, sample rejection count

$$\Pr[R=r\mid\mathsf{success},a]=\frac{(1-p)^rp}{1-(1-p)^H},\quad 0\le r<H.$$

Each rejected low word is uniform in $[m,2^b)$; masked-out high bits are independent uniform. Set the accepted low word to $a$, draw its high bits uniformly, and fill all remaining reserved slot bytes uniformly. For $m=1$ consume nothing. This is the exact conditional original raw-slot law.

For a prescribed ring polynomial, choose uniform permutations within its magnitude-one and magnitude-two supports. Replay Fisher–Yates to recover each accepted range target; use the preceding slot sampler. Give each sign byte its prescribed low bit and seven independent high bits. Each polynomial has exactly $n_1!n_2!$ ordered representations. Therefore, conditioned on decoder completion, the unfiltered polynomial is uniform on

$$|\mathcal S|=\binom d{n_1+n_2}\binom{n_1+n_2}{n_1}2^{n_1+n_2}.$$

With fixed filter $\mathcal A\subseteq\mathcal S$, candidate completion probability is independent of its polynomial. Retain the original forward decoder's rejected-prefix/stopping law, replace only the final accepted candidate by its exact conditional raw sampler, and fill unobserved finite slots/tail with their original law. A rejection-loop implementation may repeat whole forward runs until completion; its expected cost is priced using the actual accepted fraction/completion probability. It does not need enumeration of $\mathcal A$ and does not invert concrete SHAKE. All sibling worlds retain literal original raw answers at the marked branch; absent future entries are fresh, not frozen to force a match.

The iid slots and coordinate allocations establish the product shell law before ordinary acceptance/nonce selection. Thus the marked flat star applies to one atomic level family, not to a made-up independent collection conditioned on a winning response nonce.

## 5. Failure, old-key and cost ledger

A conservative mask-cap failure bound per family is

$$\delta_{\rm mask}\le\sum_cJ_cw_c2^{-H}.$$

Add each filter's actual exhaustion bound $(1-a_c)^{J_c}$, where $a_c$ is its certified accepted fraction after the specified completion convention, and field failure above. Count these at actual original exposure sites; do not multiply an unconditional child bound by privately forced successful branches. The explicit strict global extraction cutoff covers every decoded/raw draw, replay, source comparison and collision output.

Every proper descendant key includes its completed ancestor result. Parsing fixes a specific scalar or complete shell tuple and its owner. Consequently old-key compatibility is bounded by the actual result point mass: $1/q$ for a completed uniform scalar, $1/|\mathcal A|$ for a completed accepted ring coordinate, with separate failure atoms. Charge $\binom Q2\gamma$ only after this syntactic owner proof; a shared global tightening requires a separate joint-owner lemma.

$Q$ counts original adversary/prover/verifier and allowed initialization entries, failed nonce probes and the chosen atomic request granularity. Answer lengths, SHAKE work, private counterfactual tables, copies, full source decoding and conditional sampling enter total extraction time. Logical GrindingPlan counts are not an adversarial-query cap. Finite answers may be sampled/replayed lazily, but no cost for actually exposed bytes is omitted; a conservative explicit full-length bound is always available.

For example, the saved Dense singleton's conservative $4b_j$ recurrence and optional terminal incoming cut give leading coefficient $51{,}904{,}512$ for $b=(9,16,11,8,8)$. This is executed arithmetic on inspected geometry, **not** an admitted accumulator schedule or a complete work estimate. All matrix comparison instances retain actual mixed radii and complete reduction time; modeled lattice ROP128 does not imply $\operatorname{Adv}(\tau)=\tau/2^{128}$.

## 6. Integration gate

Implement this boundary in the ordinary proof-stream/channel and level draw plumbing, reuse the existing sparse arithmetic and integer predicate, migrate every affected producer/consumer and bump the protocol version. Do not change original source commitments to fit the transcript. Independently review this new grammar and its actual implementation before calling Candidate1 sound. The older v7 seed/chained interface cannot be silently selected by a backend feature in an admitted AQRA run.

The frontend is a separate compiler contract. Its existing legacy digest state does retain its full output, unlike the ordinary pinned hash backend; its fresh modulo-$q$ scalar has maximum atom $2/2^{128}$. Neither fact alone proves full-prefix ownership over arbitrary older descendant keys. Derive its actual chained-state/context collision and exposure ledger or migrate its challenge boundary too. No unsupported concrete chained-hash knowledge claim replaces this obligation.
