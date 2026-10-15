# Candidate1: global original-RAM signed-plane range review

## Result and scope

**Conditional design pass for ORIGINAL_PLANE_DESIGN §§1–3 and 5.** The proposed global construction preserves the original signed-plane predicate. It does not require a canonical Horner decomposition, homomorphic commitments, cancellation in the formal query variable, or completed per-segment PCS proofs. The original source and its A/B/F explanation remain the anchor.

The requested conservative fixed-source bound `(14 n + 3)/|K|` is valid for `n = 21 + ceilLog2(N_RAM)`. A source-backed refinement is available: the actual reusable balanced-range drivers normalize away the equality factor, so their round residual degrees are four, not five. Reusing that precise convention with a standard cubic linkage gives `(12 n + 3)/|K|`. These are upper bounds, not claims that every residual attains its maximum number of roots. The exact site-resolved bad-set expression is given below.

Main Booleanity, support, and first/last-PC predicates remain owned by the existing frontend certificate and complete leaf roster. This review does not replace them. The selected application theorem remains the base K-valued continued relation, not RV64/SDK byte-word/tape/stopping semantics. The stronger gaps remain disclosed separately in CONTINUED_FRONTEND_CERTIFICATE.

The document explicitly describes an implementation capability still to be built. Existing arithmetic and round drivers are reusable, but the ordinary Stage-1 session is a response-witness session, not an already functioning global original-source range adapter. The final raw endpoint view also remains a separate native-opening implementation gate. No code defect is inferred merely from this disclosed conditional status.

Work was static document/source inspection and the elementary algebraic/probability derivations below. No repository edits, Git/fetch operations, builds, tests, linters, formatters, proof executions, or benchmarks were performed. The only write is this local review artifact. The relevant source paths were read directly; local theorem notes are not used as authenticated mathematical premises.

## 1. Exact source roster and polynomial

Let the history contain H segments. Use H here to avoid conflating the history length with the selected frontend's 4096-row segment size, also called T in some frontend notes.

The canonical boundary list is `C_0, ..., C_H`: segment j consumes C_j and produces C_(j+1). This has H+1 boundary positions, not 2H unrelated RAM sources. Exact predecessor checks are real consumer checks: `jolt/crates/jolt-verifier/src/segment.rs:311–324` compares incoming_ram to the predecessor's outgoing_ram together with run, preprocessing, I/O, counters and public state. Each endpoint is also validated against the original source contract and logical/physical shape at `:280–306`.

Assign a canonical index to each distinct original source identity/profile/view in first-boundary-occurrence order and preserve the boundary-to-index alias map. Thus `N_RAM <= H+1`, with equality when distinct boundaries have distinct identities. Do not deduplicate merely because decoded values or profile bytes agree; do not retain two indices for the mandatory adjacent alias. If the selected contract uses exactly H+1 distinct boundary sources, set N_RAM=H+1 explicitly. Repeated nonadjacent identities can be deduplicated only as exact original identities, with extracted source explanations coalesced under the actual certificate comparison. For a nonempty history N_RAM is positive; a separately admitted empty-history case skips a range instance rather than computing log2(0).

For each original RAM, let

- coefficient a range over 0..127;
- digit ell range over 0..25;
- position p range over 0..31;
- block b range over 0..15;
- s_(C,b,p,ell,a) be the K coefficient of the literal original A-input ring polynomial.

Its logical address domain has 128*32*16=65536 entries. Its real plane domain has 26*65536=1,703,936 coefficients. The raw grid has 32*65536=2^21 entries, with exactly six absent digit addresses per logical address.

Use the low-first source axes

`X = (coefficient[7], digit[5], position[5], block[4])`.

Define f_C(x)=s_(C,b,p,ell,a) for ell<26 and f_C(x)=0 for ell>=26, and let F_C be its multilinear extension. All 26 real planes, including plane25, are included. None is a special canonical high plane. The six zeros are the definition of absent raw addresses, not a theorem about physical source padding.

The actual source bridge already distinguishes these domains:

- `akita/crates/akita-cpu-backend/src/sources/packed_ram.rs:22–45,63–117` fixes d128/base32/depth26, preserves legal noncanonical signed paths, and stores `[block][position][digit][coefficient]` without six raw slots;
- `akita/crates/akita-types/src/full_ring/mod.rs:66–113,254–292` provides RawSourceViewPlan, including physical_index, raw_index and raw_to_physical; the latter returns None on an absent digit;
- `:299–343` supplies FullRingRawPoint in exactly coefficient/digit/position/block/polynomial axis order;
- `akita/crates/akita-config/src/full_ring.rs:79–110` specifies the original RAM profile with 16 blocks, 32 positions and 26 planes.

Those honest source constructors are layout/implementation evidence, not an adversarial proof of range. The extracted A coefficients below can initially be arbitrary K values.

Put k=ceilLog2(N_RAM), n=k+21. For a fixed canonical index map,

`G(I,X) = sum_C eq(I,index(C)) F_C(X)`.

G is multilinear in every one of its n variables. On a Boolean source index it is exactly the corresponding F_C, or zero for an absent source bucket. It is a deterministic view of original sources, not a new source commitment. For implementation, preserve the existing low-first raw axes and put the source-index axes last; the displayed notation G(I,X) is semantic, not permission to reorder the raw point API. A different traversal must explicitly apply the installed coordinate permutation.

Freeze the complete ordered original commitment/profile/view roster, alias map and original payload context **before the first range anchor request**. Source explanations are obtained later by extraction; the source-probe proof fixes an independent hypothetical explanation before the first fresh anchor, rather than assuming the actual extracted source was historically known then.

## 2. Signed predicate: direct factorization proof

For K the selected prime field, with characteristic q>32, define

`P32(V) = product_(j=0..15) (V - j(j+1))`.

For every g in K,

`P32(g(g+1)) = product_(j=0..15) (g-j)(g+j+1)
             = product_(t=-16..15) (g-t)`.

This is an exact polynomial identity: `g(g+1)-j(j+1)=(g-j)(g+j+1)`. Its 32 linear roots are the distinct field images of the signed integers -16..15. Therefore

`P32(g(g+1))=0 iff g is the field image of exactly one integer in [-16,15]`.

Here q denotes prime characteristic/cardinality, not a ROM query count. In a generic extension, cardinality>32 alone would not be enough; the characteristic condition is the one used by this proof. The selected E=K prime field satisfies it.

Applying this identity to every Boolean cell of G proves exactly the desired predicate on every real original plane. The virtual absent source/digit cells have g=0 and satisfy it by definition. This does not impose canonicality or imply that the Horner-decoded K value is an unsigned64 word.

## 3. Deterministic helper DAG and its transcript

Let g_x=G(x) on the full n-dimensional Boolean cube. Define the virtual table and four quartics

`U_table(x) = g_x(g_x+1)`,

`Q_j(V) = product_(t=4j..4j+3) (V - t(t+1)),  j=0,1,2,3`.

Write U for MLE(U_table), and H_j for MLE(x -> Q_j(U_table(x))). In particular, H_j is generally NOT Q_j(U) off the cube. Likewise U(u) is generally NOT G(u)(G(u)+1) off the cube. These are deterministic table lifts, not independently chosen witness polynomials.

Use precisely the existing base32 balanced topology: one arity-four product stage, four child values, one powers batch, and one quartic leaf. `akita/crates/akita-params/src/layout/digit_range.rs:197–255,330–386` establishes this convention: product arities `[4]`, four quartic leaf factors, weights `[1,gamma,gamma^2,gamma^3]`. Do not substitute the separate FullRingRangePlan two-product/affine convention in the same file.

The complete causal protocol is:

1. Bind the frozen range context. Draw independent uniform scalar anchor coordinates tau[0..n-1], each under the registered v8 site/result grammar. No helper or source payload can be revised after this context. The asserted initial claim is zero for
   `sum_x eq(tau,x) product_j H_j(x)`.
2. Run n rounds of the existing normalized equality-factored product sumcheck, inner degree four. Each nonconstant coefficient body is received/bound before that round's challenge. Let r be its terminal point and c_prod its normalized terminal claim.
3. Receive and bind all four c_j. Check `c_prod = product_j c_j`. These values are claims H_j(r), not separately committed helpers.
4. Only after all four child claims, draw gamma. Compute `c_leaf=sum_j gamma^j c_j` and `Q_gamma(V)=sum_j gamma^j Q_j(V)`.
5. Run n normalized equality-factored quartic-leaf rounds for
   `c_leaf = sum_x eq(r,x) Q_gamma(U(x))`.
   Let u be its terminal point. Receive/bind t_U and check the normalized terminal claim equals Q_gamma(t_U). The owned value is t_U=U(u), not a source opening.
6. With u and t_U already bound, run a standard n-round degree-three sumcheck for
   `t_U = sum_x eq(u,x) G(x)(G(x)+1)`.
   This is the required on-cube linkage. Its summand has individual degree at most 1+2=3.
7. At terminal z=(z_I,z_X), receive **all** p_C, in frozen canonical source order, with exact arity N_RAM. Define
   `g_z=sum_C eq(z_I,index(C)) p_C`.
   Check the link's terminal claim equals `eq(u,z) g_z(g_z+1)`.
8. Append every scalar assertion `p_C=F_C(z_X)` to the deferred query relation, one distinct formal Z exponent per source claim. Bind the complete tuple and the append/count update before seal and before any request for a. Zero source-index weights at this particular z do not authorize dropping p_C; all original endpoint claims are owned.

Steps 2–5 follow the real producer/consumer order, not just the producing side: `akita-prover/src/protocol/prove/fold/sumcheck.rs:203–268,375–438` emits child claims before gamma and the leaf before its range-image scalar; `akita-verifier/src/stages/stage1.rs:50–117,155–179` reads all children, checks their product, then draws gamma and replays the leaf. The new linkage's terminal tuple must be a typed original-plane deferred-claim batch consumed by the query accumulator and replayed by its verifier. Passing only the free t_U scalar to the old Stage-2 interface is not this construction.

The raw range is not a predicate on the ordinary PCS's recursive response grid. Its new source adapter must use G over the original roster. L2/norm/source-support proofs selected by the final PCS stay in that PCS; reusing range arithmetic does not disable them.

## 4. Exact first-divergence bound

This argument applies to fixed arbitrary original K sources and arbitrary adaptive messages. It does not assume the source range being proved.

### 4.1 Anchor

Let

`R(Tau)=sum_x eq(Tau,x) P32(g_x(g_x+1))`.

If range is false, its Boolean residual table has a nonzero entry. The multilinear interpolation R is then a nonzero polynomial because its value at that Boolean address is that entry. It has total degree at most n. Independent uniform coordinates give `Pr[R(tau)=0] <= n/|K|` by the elementary multivariate root-count induction. With per-coordinate maximum atoms M_tau,i, the bound is `sum_i M_tau,i`.

### 4.2 Product rounds: normalized degree four

The reusable implementation is not a full-degree-five round verifier. `akita/crates/akita-sumcheck/src/traits.rs:120–127` explicitly removes preceding equality factors. `single.rs:7–15` computes

`q0 = c - tau_i * sum_(d=1..4) q_d`,

`c_next = q0 + sum_(d=1..4) q_d r_i^d`.

Thus `(1-tau_i) q(0)+tau_i q(1)=c`, without division. The verifier carries q(r_i), NOT eq(tau_i,r_i)q(r_i). `proof_stream.rs:335–468` uses this same arithmetic on both sides; the Stage1Verifier terminal check is the unscaled product of the child claims.

Let q* be the true normalized remaining-sum polynomial. If the incoming claim is false, q-q* cannot be zero: their weighted Boolean evaluations differ by the nonzero incoming discrepancy. Its degree is at most four. A false incoming claim becomes a true outgoing claim only at a root of this nonzero residual. Therefore the product rounds contribute at most `4 sum_i M_prod,i`, or `4n/|K|` for uniform scalars. No equality-zero rejection or inversion is required.

If the terminal product claim remains false and its deterministic product check passes, at least one c_j differs from H_j(r). This is a logical implication, not a probabilistic extra cost.

### 4.3 Four-child powers batch

After the four children are bound, their error polynomial is

`B(Gamma)=sum_(j=0..3) Gamma^j (c_j-H_j(r))`.

At least one coefficient is nonzero, so B is nonzero of degree at most three, including when gamma=0. Charge `3 M_gamma`, or `3/|K|`. These are powers of one draw, not four independent affine coefficients.

### 4.4 Quartic leaf

Outside the batch bad set, the leaf's incoming claim is false relative to U. Its normalized residual has degree at most four at each round by the same q0 argument. Charge `4 sum_i M_leaf,i`, or `4n/|K|`.

If it finishes with a false claim but the deterministic Q_gamma(t_U) endpoint comparison passes, t_U cannot be the true U(u). No degree or independent-challenge term is charged for merely receiving this late scalar; its truth is owned by the link.

### 4.5 Cubic linkage

A false t_U starts a false standard sumcheck claim. Each full summand/round residual has degree at most three. The standard driver binds the compressed polynomial before its challenge and reconstructs the omitted linear coefficient from p(0)+p(1)=c (`proof_stream.rs:197–334`). Charge `3 sum_i M_link,i`, or `3n/|K|`.

If all original endpoint claims p_C are true, the reconstructed g_z is the actual G(z), so the final check authenticates the standard link terminal. A zero eq(u,z) is not an unpriced cancellation: any restoration of a false running claim at such a challenge is included in the cubic residual-root event. Do not divide by equality factors.

### 4.6 Complete ledger and the requested conservative candidate

For the actual normalized convention, the site-resolved degree-times-maximum-atom bound is

`epsilon_range <= min(1,
    sum_i M_tau,i
  + 4 sum_i M_prod,i
  + 3 M_gamma
  + 4 sum_i M_leaf,i
  + 3 sum_i M_link,i)`.

The exact local bad-event expression uses the actual anchor/round/batch root sets at each fixed prechallenge transcript: the conditional probability is the sum of the challenge masses on that set. The displayed formula bounds each set by its established degree; it is not an equality for the attack probability.

For independent uniform successful scalar draws this yields

`epsilon_range <= min(1,(12n+3)/|K|)`.

A full, nonnormalized eq-times-product and eq-times-quartic implementation has degree-five residuals, giving

`n + 5n + 3 + 5n + 3n = 14n+3`.

Consequently the requested `(14n+3)/|K|` is conservative and valid also for the normalized implementation. It must not be presented as the exact degree of the normalized round driver. An implementation mixing normalized updates with unnormalized terminal equality factors would change the relation, not merely choose the conservative error bound.

The proof covers adaptive helper claims and free-looking intermediate scalars because it reasons from the first false-to-true divergence, with source-determined helpers. It does not take an honest helper constructor as a soundness premise.

## 5. Query reduction, real claim count, and final endpoint ownership

### 5.1 Decoded adjoint remains exact

For an existing decoded claim at address u,

`v=sum_(a,p,b,ell<26) eq(u,(a,p,b)) 32^ell s_(C,b,p,ell,a)`.

The exact adjoint on the raw grid is

`phi_u(X)=eq(u,X_address) B(X_digit)`,

`B(D)=sum_(ell=0..25) eq(D,ell) 32^ell`.

This follows immediately by evaluating the raw inner product on Boolean addresses. Keep the declared address-to-coefficient/position/block permutation. A new native raw claim at z_X uses `phi=eq(z_X,X)`. Both are multilinear public query polynomials on the same F_C. There is no replacement of B by an equality point or change of the immutable logical16-variable original profile.

Only the public query has zero-supported extra axes. Main stays all56 real components, each of logical arity16. For the selected Main/RAM-only flow a common query dimension n_Q=21 suffices, independently of the range's k source-index axes. The query sums over sources explicitly; it need not carry I as a query axis. Other retained sources require n_Q at least their actual view dimension.

### 5.2 m_horizon counts equations, not source identities or final PCS slots

The formal query residual is

`R_query(Z)=sum_(i=0..m-1) Z^i (actual_source_inner_product_i - asserted_value_i)`.

Every independent scalar assertion has a distinct exponent. Count all old retained equations, current frontend leaves, additional source predicates and the N_RAM new raw range claims. A source alias does not merge distinct scalar equations at different points. Verifier-validated exact aliases can be omitted only according to the declared complete roster, not because they share a commitment.

For the selected no-advice/no-field/full-public continued flow, the existing direct Main roster has 118 scalar claims per segment:56 native stage8,56 support,3 first-PC and3 last-PC. Both RAM endpoint scalars also remain. This is documented with source maps at CONTINUED_FRONTEND_CERTIFICATE §§5–6 and FRONTEND_DEGREE_LEDGER §§6–7, and matches the native/source-check interfaces; these counts are interface facts, not independent soundness theorems.

Therefore, with that direct complete roster:

`m = 120 H + N_RAM`.

If the separately certified existing 118-to-56 normalization is selected, its 118 inputs are owned by that normalization/source cut rather than silently discarded, its actual additional local error is retained, and the query receives56 normalized original Main claims plus two RAM claims per segment:

`m = 58 H + N_RAM`.

Additional advice/program/field/source obligations add their real counts. A symbolic m_horizon must upper-bound the actual selected formula. In the distinct-boundary case it includes the extra H+1 raw claims. The final PCS component roster is a different count: for this selected flow it has56H original Main component endpoints and N_RAM original raw RAM endpoints, irrespective of how many pointed equations led to each source.

Do not substitute58, a segment count, a count of commitments, a helper count, or a root shell count for m. `m_horizon<|K|` and the finite degree/error policy must be enforced. Helpers c_j and t_U are already owned by the range DAG/link and are not separately added as purported original source equations.

### 5.3 Complete order

The verifier order is:

1. Replay all real continued frontends, including Main Booleanity/support/PC and all required source leaves; retain their transcripts and exact public transitions. No completed per-segment PCS inputs.
2. Freeze the canonical original source roster and aliases, then replay the global range proof above.
3. Receive/bind the full p_C tuple; check the link; append every corresponding raw claim with its distinct exponent and update the count.
4. Seal the complete context, frontend/range evidence, original source/view/component identities, every scalar assertion/adjoint/exponent, finite horizon, and the deterministic **schema/order** of final original endpoints.
5. Only now draw a. Run the one weighted degree-two query sumcheck.
6. At its final point w, receive every required original raw RAM endpoint F_C(w_RAM), and every declared Main component endpoint, including components whose current public query weight happens to vanish. Independently evaluate the public query weights and check its terminal sum.
7. Bind those actual endpoint points/values and the entire earlier sealed context into the original-source-view-capable native batch decider before its scalar batching/shell challenges. Finish every shrinking suffix and the actual terminal. Replay and check the whole proof independently.

The final query point and values cannot be known before a; the pre-a seal freezes their ownership schema, not invented future values. The actual endpoint tuple is bound after it is derived and before the native decider challenges that consume it. This is consistent with ORIGINAL_PLANE_DESIGN §5, provided the implementation distinguishes roster schema from future endpoint payload.

The p_C at the range link and the endpoints at the weighted query terminal are generally at different points. It is the query proof/source cut that transfers the former raw assertions to the latter original openings. No p_C is authenticated merely because the prover sent it or because the global sum g_z happened to be correct.

## 6. Separately owned source-probe range cut

### 6.1 Output that the cut consumes

Let D_Q be the complete query-plus-native-PCS extraction map, with the original marked output preserved. It must return:

- the full original A/B/F source projections, not merely decoded Horner tables;
- their actual weak product and path certificates;
- every original final query endpoint under the registered raw/Main views;
- truth of every sealed scalar query equation, including all N_RAM p_C equations, obtained by its separately owned query source cut;
- the exact original context/count/alias map.

The weighted query's fixed-source local error is

`epsilon_query <= (m-1+2 n_Q)/|K|`

under uniform scalar draws. The formal residual is nonzero when any equation is false because exponents are distinct. The sumcheck has degree two because both the public adjoint and source view are multilinear. This derivation requires no range premise on F_C.

### 6.2 The raw range is its own earlier cut

The range cut starts immediately before the **first fresh anchor coordinate**, with the complete original RAM roster frozen. Run D_Q on the original complete continuation first. On a valid original source output, restore that saved pre-anchor ordinary state/tape and the oracle table as it actually stood then. Run independent ordinary continuation trials; gate each expensive D_Q on acceptance and exact selection of that same range root key/header. Retain the first valid matching original-source output.

The gate binds all target original identities/profiles/views and the alias map. Later range messages, raw points/claims, query seal, a, query terminal points, native D/H partials and future transcript can differ. Do not force the original future oracle table into a trial. Compare the full target original A/B/F projections to the original output, not query points or opening-local D/H. On equality, directly scan/test the signed predicate on the original source assignment. On difference, return the corresponding certified original Module-SIS collision.

This proves range of the original assignment rather than outputting a counterfactual source or taking a branch's range scalar as an opening.

### 6.3 No circular range assumption in comparison

Suppose the base native extraction yields, for one A-input tuple, units rho,rho' and product certificates

`r=rho s`, `r'=rho' s'`,

`||rho||_1<=kappa`, `||rho'||_1<=kappa'`,

`||r||_p<=beta`, `||r'||_p<=beta'`.

If the same original A image has different preimages, then

`v=rho' r-rho r'=rho' rho(s-s')`

is nonzero, lies in the A kernel, and has

`||v||_p <= kappa' beta + kappa beta'`.

Nonzero follows from the unit property; the norm follows from convolution/operator bounds. Neither requires `s` or `s'` to lie in [-16,15], nor a short inverse. The certified r bounds come from the final decider's validated response witnesses/deeper-source certificates, not this pending original-RAM range proof. The real terminal base directly decodes and bounds response values and checks ring equations (`akita/crates/akita-verifier/src/terminal/direct.rs:131–275`). Ordinary response-witness range and original-RAM plane range are different predicates.

For compressed sources, walk equal F payloads backward through their certified F preimages/gadget maps, then B, then A. A differing certified preimage yields its actual F/B collision; equality fixes the predecessor image. Use the actual path alphabet/product certificates supplied by the base extractor, all reachable mixed occurrence pairs, dimensions and radii. Original-plane producer envelopes, top-plane residual bounds or decoded-word range are not admissible replacements for those certificates. D/H is opening-local and is not a source comparison requirement.

These are algebraic comparisons under the existing actual-radius Module-SIS assumptions, not a new source-shortness assumption. Availability of the full raw-source native certificates is an implementation/admission prerequisite explicitly required by §4; this review does not pretend the old decoded API already outputs them.

### 6.4 Original-event probability and renewal proof

For a possible first root exposure i and saved prefix sigma, let p_i be the probability that an independent reference continuation returns a valid matching D_Q source output, and a_i the probability of its cheap exact-header acceptance gate. These probabilities use the actual source extraction success bits, including all deeper failures.

For p_i>0, the first successful trial's source has the success-conditioned reference law nu_i and is independent of the original unexposed future given sigma. The original valid-start mass is p_i. Renewal gives `E[trial_count|sigma]=1/p_i`, so original weighting cancels it: `p_i*(1/p_i)=1`. If the total original request budget is Q, the reached-prefix subprobability masses sum to at most Q. Expected additional cheap trials are at most Q. For arbitrary nonnegative expensive trial cost Y, the renewal work is `E[Y]/p_i`; original weighting likewise cancels p_i, and disjoint exact selector events bound additional expensive work by one original gated D_Q work. No lower bound on p_i or cost/success independence is needed.

For the probability proof, install an independent hypothetical source with law nu_i immediately before each possible first fresh root answer. On the original valid selected event, couple the implemented probe to that hypothetical source. A disagreement produces the above collision. Equality lets all original true raw leaves refer to this source fixed before the anchor. The first-divergence proof in §4 then applies without conditioning the original challenges on acceptance or equality.

An older proper-descendant v8 key must contain the canonical completed first anchor scalar and its exact owning request/prefix. It therefore guesses that one fresh scalar, of maximum successful atom at most1/|K|. The old-pair ledger is at most `binom(Q,2)/|K|`. This uses literal decoded-owner syntax, not hash injectivity or the raw answer's 2048-byte entropy. Later range/query/native keys must retain this completed event; a short restarted PCS context is invalid.

Consequently the new range cut has the safe original-event loss

`beta_range(Q) <= Q epsilon_range + binom(Q,2)/|K|`.

With nonuniform scalar families substitute their established fresh maximum atoms and the actual first-root atom. Decoder failures are fatal and separately inventoried; they do not license a fresh-uniform claim after a failed or already exposed request.

The query source cut is separately owned at a, with

`beta_query(Q) <= Q epsilon_query + binom(Q,2)/|K|`.

A safe combined ledger **adds both**. A single deduplicated old-pair term needs a joint parsed unique-owner/coupling proof covering both cuts and the retained PCS cuts; this review does not assume it. Main/frontend source cuts remain separate as well.

## 7. Finite cost/statistical ledger

### 7.1 Range messages and scalar families

With the normalized existing codecs and standard compressed cubic linkage:

| Item | Count |
|---|---:|
| Anchor scalar requests | n |
| Product sumcheck round requests | n |
| Child batching request | 1 |
| Quartic leaf round requests | n |
| Cubic link round requests | n |
| Total new scalar requests | 4n+1 |
| Product nonconstant proof coefficients | 4n |
| Child scalar claims | 4 |
| Leaf nonconstant proof coefficients | 4n |
| Virtual U scalar | 1 |
| Standard cubic compressed coefficients | 3n |
| Original range terminal scalars | N_RAM |
| Total field proof atoms | 11n+5+N_RAM |

These counts exclude separately framed public claims, v8 completed events/context, scheduled PoW/nonces and later query/native proof atoms. They are not a claim about a complete proof's byte size.

For the initial v8 field family, each scalar reserves128*16=2048 raw bytes. Thus one range run adds `(4n+1)*2048` bytes of formal full scalar-answer allocation, plus keys/events and any actually selected grinding predicate work. If delta=(2^128-q)/2^128, one scalar failure has probability delta^128; `S_scalar*delta^128` is a conservative deviation over S_scalar counted original scalar exposures. For accepted outputs the unconditional per-value mass is `(1-delta^128)/q <= 1/q`, and the success-conditioned law is uniform. Do not silently use the frontend legacy modulo sampler's law for these new v8 sites.

Each original/private finite raw answer, conditional draw, copy, saved table and decoding operation is charged in the actual full-answer model. Honest lazy squeezing does not reduce the formal resource charge without another cost-model proof.

### 7.2 No full raw field table is necessary

Let D=2^n=2^k*2^21 and L=26*65536*N_RAM. For N_RAM>0, `2^k<2 N_RAM`, so `D<2^22 N_RAM`. All dimensions/counts/products must be checked before use. Virtual source-index and digit zeros can be computed by address mapping; no D-entry G/U/helper allocation is a mathematical prerequisite.

A concrete allocation-free-in-D baseline is a bounded source callback that re-scans original cells to produce each round's folded endpoint pair:

- at round i, evaluate a source/helper MLE at the bound prefix and each remaining Boolean suffix by summing over its 2^i eliminated cube coordinates;
- obtain the two values for the current coordinate, then form the required affine-pair product/quartic/cubic coefficients using existing arithmetic;
- sum with the exact remaining equality weights;
- re-scan after binding the next challenge rather than retain a full table.

Across all suffixes a round has O(D) contributing original grid entries; computing prefix/remaining equality weights naively costs O(n) each. This gives a conservative O(n^2 D) arithmetic/read bound for the three fixed-width stages, with O(n+N_RAM) auxiliary storage beyond retained source/proof/oracle data. Endpoints F_C(z_X) can be computed by streaming all real cells, in O(21L) naive weighted work and O(N_RAM) result storage. This is a derivation of a possible bounded-memory arithmetic adapter, not an existing measured kernel. Cached split equality tables or compact folded prefixes can reduce work and still avoid a full D-sized table; all allocations are then priced honestly.

Do not compact ell=26..31 out of the raw address space. Their internal holes occur once per position; they are not one trailing live-prefix suffix. Also, U=0 in absent cells does **not** imply every H_j=0: H_0 has a zero root, but the other three quartic defaults Q_j(0) are nonzero. Reuse ExactPrefixTable's actual implicit default and weighted suffix mass, not a blanket zero-helper assumption. Real source alignment/padding elsewhere remains governed by its selected source/GRID semantics, not an all-padding-zero assertion.

### 7.3 Added extraction work

Write C_PCS for the complete selected raw-view-capable native PCS extraction cost, T_Q for a cheap query-plus-PCS continuation, T_R for a cheap range-plus-query-plus-PCS continuation, and A_Q/A_R for all source comparisons, literal-grid scans, exact checks, output and sampler overhead not already included. They must include full key/raw-answer work, not just field operations.

For the separate query source cut,

`C_Q <= 2 C_PCS + Q T_Q + A_Q`.

For the earlier global range source cut,

`C_R <= 2 C_Q + Q T_R + A_R
     <= 4 C_PCS + 2Q T_Q + Q T_R + 2A_Q + A_R`.

The second factor2 is genuine: the range and query cuts have different saved roots and source-map ownership. Do not declare that one later all-sealed-source probe retroactively fixes G before its earlier anchor. Sharing or reducing these work factors would require a different jointly proved source-map construction. No interpolation factor `m*3^n` or helper-tree arity product is needed in this source-probe route.

The native C_PCS still has its actual retained shell/suffix recurrences, native root batch width, source comparisons, setup provenance and terminal. For the simplest selected one-point-per-original-component layout with H Main56 sources and N_RAM RAM sources, the root's claim/block coordinates are56H+16N_RAM before any optional setup groups; this is not a complete catalog-row or security admission. The final actual row, shared D/H widths, chunking, every shrinking suffix and terminal must be priced from its real registered layout. Older two-RAM/Main control rows do not admit this larger source roster.

### 7.4 Full finite advantage statement

Before additional frontend terms, a safe symbolic finite bound is

`kappa_global <= kappa_PCS
               + Q epsilon_query + binom(Q,2)/|K|
               + Q epsilon_range + binom(Q,2)/|K|`.

Append the actual independently owned frontend/source/normalization cuts and their sampler laws; do not multiply kappa_PCS by H or by the number of private source probes. The marked original extraction output is retained, while private continuations use actual success bits. Complete loss also includes all declared scalar/shell/predicate cap terms, setup/identity deviations when selected, and the global strict-work cutoff.

If W is the complete unconditioned expected extraction work and the strict work budget is L_cut, charge `W/L_cut`. Every original Module-SIS advantage is evaluated at the **whole** resulting bounded producer runtime, including private counterfactual continuations, all finite oracle bytes, setup simulation, comparisons and collision reconstruction, with its actual mixed certified radius and dimensions:

`Pr[original accepts and no original relation witness]
 <= kappa_stat + Delta_sampling + Delta_setup + W/L_cut
    + sum_I Adv_ModuleSIS_I(T_I(L_cut), eta_I)`.

This is a finite parameterized ledger, not a numerical end-to-end admission. The declared history/claim/source/request horizons, full selected row and justified actual-radius hardness/resource policy still need concrete values. The existing base Module-SIS/setup contracts are retained; no new hash-knowledge, short-source, source-homomorphism or SDK theorem is assumed.

## 8. Reusable code and precisely required adapters

| Existing authority / arithmetic | Reuse | Required change, not an existing capability claim |
|---|---|---|
| `akita-params/src/layout/digit_range.rs:197–255,330–386` | Base32 topology, four quartics, child order, powers batching, shape counts | Build its global original-source domain/context; do not use FullRingRangePlan or invent a second base32 tree |
| `akita-sumcheck/src/traits.rs:43–80,120–165`; `single.rs:7–15`; `proof_stream.rs:197–468` | Standard cubic and normalized EqFactored kernel/driver contracts, omitted coefficients, verifier replay | Supply real source/helper arithmetic kernels and route the channels through approved v8 typed events/scalar requests; current channels still expose concrete ordinary ProverChannel/VerifierChannel |
| `akita-prover/src/protocol/prove/fold/sumcheck.rs:203–268,375–438`; `akita-verifier/src/stages/stage1.rs:50–179` | Existing product/child/gamma/leaf producer and consumer choreography | Extract/use that choreography against the global G adapter and add the terminal cubic raw-source linkage, not a fake response handle |
| `akita-cpu-backend/src/opaque/sumcheck/digit_range/class_indexed_product.rs:368–465`; `class_indexed_range_leaf.rs:195–353`; `mod.rs:57–103` | Affine pair polynomial arithmetic, degree-four interpolation/composition, per-round accumulation | Add a virtual-source provider or generic folded pair callback under the same kernel contracts; do not pretend current CompactDigitSource stores arbitrary extracted K planes |
| `digit_range/compact_digit_source.rs:8–35,38–76` | Compact honest signed-class optimization when its invariant is actually established | It assumes ring-switch balanced digits and can index a fixed class table; it cannot establish adversarial original-plane range. Keep verifier/proof reasoning over arbitrary K sources, and provide a suitable original source adapter |
| `akita-algebra/src/split_eq.rs:56–152`; `digit_range/round_accumulation.rs:17–126`; `exact_prefix.rs:1–156` | Split equality, implicit default values, exact folded prefix and suffix mass | Preserve structural interior digit holes and correct quartic defaults. A simple FlatBooleanDomain live-prefix declaration alone does not encode the raw interior holes |
| `akita-types/src/full_ring/mod.rs:66–113,254–343` | Existing raw/physical injection and raw point axis weights | Use it for each source, plus an outer canonical source-index adapter; no fabricated physical zero slots |
| `akita-cpu-backend/src/sources/packed_ram.rs:256–277,369–431` | Physical digit callback, original packed source, existing decoded partial preparation | The current ordinary packing partial callback Horner-decodes26 digits. The new raw endpoint needs the separately registered digit-equality weight recipe; do not feed21 raw coordinates to the16-address constructor |

The new cross-boundary objects are the global original-source view, the deterministic helper evaluations, and the all-source raw endpoint claim batch. Their consumers must respectively route to the virtual arithmetic provider, the checked child/batch/leaf/link transitions, and the formal query append/replay followed by the final registered raw native view. No catch-all decoded opening, unused free helper scalar, or unconsumed endpoint tuple can satisfy this contract.

The final raw decider must authenticate the literal original coefficients via the separately reviewed public digit-weight native row, full original A/B/F path and complete suffix/terminal. It need not expose or commit G/U/H tables, and it must not rely on this range proof to justify the weak certificate used to coalesce sources. Existing Main source predicates stay in their original frontend ownership.

## 9. Parent amendment and handoff

The proposed amendment is mathematically sound with the following explicit completions carried into root PROTOCOL:

1. Canonical boundary/source alias roster and immutable original profile/view context are frozen before range anchors; the H+1 boundary positions are not2H independent inputs.
2. Base32 product4 / four children / powers gamma / quartic leaf follows the normalized existing convention, with the standard cubic on-cube U linkage and every original p_C appended before a/seal.
3. The local range ledger is site-resolved as above; `(14n+3)/|K|` is a valid conservative candidate, while `(12n+3)/|K|` matches the normalized reused drivers.
4. m_horizon includes every real sealed scalar equation, including all new N_RAM raw claims, and is distinct from the final native component/shell roster.
5. One earlier raw-range source cut is separately owned and compares full original A/B/F weak certificates without a prior range premise; its actual additional work and old-query loss are not hidden inside the later query cut.
6. The final original-plane endpoint consumer is the registered raw native opening view, with all source/components and complete native suffix/terminal; the old decoded16 API is not a raw-plane authentication fallback.
7. Every new range/query/endpoint event is in the approved v8 literal indexed grammar, with complete inherited context and finite request/raw-answer costs. Preserve all Main/support/PC obligations and the disclosed base frontend semantic scope.

These complete the requested reviewed signed predicate proof, causal endpoint/query ordering, finite statistical/work ledger, source ownership and reusable-infrastructure map. They do not claim an implemented raw-view adapter, an admitted larger catalog row, concrete SHAKE/QROM security, or exercised performance/correctness checks. Parent owns repository documentation amendments and implementation/verification work; none was performed here.
