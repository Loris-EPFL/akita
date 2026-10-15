# Fresh balanced source prefix: independent repair review

**Implementation update:** the historical mismatch reviewed below has been repaired. The fresh balanced producer and verifier now use `RawSourceReader`, two degree-five `ProductRangeSession` stages, and degree-three raw realization with optional authenticated physical-zero support. Typed `SourceDomainSites` bind the grammar before draws, and `SourceDomainPlan::range_ledger()` exposes its local costs. A separate independent implementation review found no scoped defect in the algebra, source preservation, transcript replay, parsing or ledger. This does not admit the complete production registry, final receipt, numerical security or performance parity. The original review below remains the pre-change evidence and specification.


## Verdict and scope

**The repair direction in native-reuse revision §8 is sound as a design direction, subject to the exact contract below. The current fresh balanced implementation does not implement the manifest-priced protocol.** This is a source/algorithm review, not implementation admission, numerical security, deployment, or a performance measurement. No source changes, builds, tests, formatters, linters or commits were performed; the 19 known downstream Jolt transcript API errors were not rerun.

The invariant/anchor is the original committed K coefficient table, its exact raw injection/decoder, source ID and owner, public commitment and retained A/B/F path, plus the same pending-opening/normalization/continuation semantics. Replacing the degree-33 proof changes the proof transcript and is a versioned protocol change, not a bit-identical transcript optimization. Do not change original physical planes or turn the endpoint into proof acceptance.

The requested bounded diff command in the containing directory failed because that directory was not a Git worktree; the same source-path diff and scoped history commands within `akita/` produced no output. Thus the substantive finding below is independently established against the requested current files and §8, not represented as proof of when a tracked source change was introduced.

## 1. Independently confirmed current mismatch

- `akita/crates/akita-types/src/full_ring/query.rs:123-128`: `SourceDomainPlan::degree()` returns 33 for `BalancedBase32`.
- `akita/crates/akita-types/src/full_ring/strong.rs:625-651`: `SourceDomainSession` constructs equality times all 32 factors for roots -16 through 15, plus the physical-zero term, on the raw grid.
- `akita/crates/akita-prover/src/protocol/prove/full_ring_strong.rs:103-140`: the fresh-source driver runs that one sumcheck, and samples `eta_zero` even when its support is empty.
- `akita/crates/akita-verifier/src/full_ring_strong.rs:105-126`: the consuming replay reads `plan.degree()+1` coefficients, so this is not an unused producer or a degree-label-only discrepancy.
- `AKITA_INSTANCE_ACCUMULATION_GLOBAL_MANIFEST.md:57-69`: balanced fresh sources are instead priced as an anchor, two degree-five stages and a degree-three realization, `14*mu+3` before adjuncts.

For one compact RAM source, mu=21: the current direct alphabet check alone incurs 33*21 degree charge plus 21 anchor charge = 714, versus the manifest's 297. Conservatively counting its unconditional `eta_zero` draw gives 715 before any separately charged required-zero anchor family; the transmitted proof values are also 34*21+1 = 715 E elements (22,880 bytes), versus 16*21+6 = 342 E elements (10,944 bytes) in the repaired range protocol. Proof values, sampled challenges, bad-set charges and extraction factors are distinct quantities; their coincident current number 715 must not conflate them. This mismatch invalidates attaching the exact manifest estimate to the current driver; it is not evidence that the direct algebraic predicate is unsound.

## 2. Exact borrowed raw reader

Add one small, lifetime-bounded reader in `akita-types/src/full_ring/strong.rs`, implementing the existing `FullRingCoefficientSource<K>` trait. Suggested name: `RawSourceReader<'a, K>`. It holds references to the already borrowed physical source and its installed `RawSourceViewPlan`.

Construction must check `physical_source.len() == view.physical_len()`. The reader reports `view.raw_len()` as its length. For every in-range raw index x, call the existing `view.raw_to_physical(x)` exactly once: return the original `source.coefficient(i)` for `Some(i)`, or fixed K zero for `None`; propagate out-of-range/mapping/read failures. No raw-domain Vec, decoded table, redecomposition, commitment, source handle, dense K copy, wholesale E lift, or changed physical plane is introduced. The plan's public permutation is borrowed (or Arc-shared by its existing cheap clone), not copied per cell/session.

The distinction between fixed holes and required physical zeros is essential:

- A virtual hole is not an address in the committed source and always reads zero.
- A required physical-zero cell **must still return its actual retained coefficient**, even when it should have been zero. Its value is constrained by the zero adjunct; the reader must not overwrite it with zero, hide it as sparsity, or omit it.
- Do not reinterpret the physical table as a flat power-of-two prefix. The existing raw axes are low-to-high `(j, ell, x, i, p)` with separately padded digit and polynomial axes. Use the exact map and original storage permutation.
- Compact RAM has physical `128*1*16*32*26 = 1,703,936` cells and raw `128*1*16*32*32 = 2,097,152` cells (mu=21). Its original 26 planes stay intact. Calling the existing range session on the physical reader directly fails its dyadic-length check; padding the physical commitment to work around that failure is not authorized.

This is a read adapter, not new ownership or commitment authority. Keep it local to `with_full_ring_source` and the fresh-domain driver; neither it nor a session borrowing it escapes that callback. The original physical reader—not this raw-length adapter—continues into `NormalizationSession`, whose length check and `Column::Base(view=Some(...))` already apply the raw injection. Supplying the adapter there would either fail the physical-length check or apply the view twice.

## 3. Reuse the existing range algorithm without changing its semantics

Let W be the raw multilinear extension of the reader, N=2^mu. Define Boolean tables

- U(x) = W(x)(W(x)+1),
- Q_j(x) = product over k=4j,...,4j+3 of `(U(x)-k(k+1))`, for j=0,1,2,3.

These are tables whose multilinear extensions are used after binding. In particular U(r) means MLE of the Boolean table, **not** W(r)(W(r)+1) at an off-cube point. `ProductRangeSession` already preserves this distinction (`SourceTransform::Virtual/Quarter`, `begin_leaf_stage`, `virtual_claim`); `full_ring/tests.rs:203-240` includes an explicit off-cube distinction check.

Run existing `ProductRangeSession::new(&raw_reader, &tau0)` without changing its algorithms:

1. First degree-five sumcheck, target zero, for `sum_x eq(tau0,x) * product_j Q_j(x)`.
2. Send all four q_j=Q_j(r1) before sampling the three independent xi coordinates. Verify first endpoint `eq(tau0,r1)*product_j q_j`.
3. Use the existing normalized weights `(1,xi0,xi1,xi2)`, not powers of a scalar and not four independently drawn coefficients. Second degree-five sumcheck has target `q0+xi0*q1+xi1*q2+xi2*q3`.
4. Send svirt=U(rvirt); verify second endpoint `eq(r1,rvirt) * range_leaf_endpoint(svirt,xi)`.

At a virtual padded cell, the adapter returns W=0 **before** transformation. Thus U=0, Q0=0, and Q1/Q2/Q3 are the corresponding generally nonzero products of `-k(k+1)`. Do not force every transformed/quarter hole to zero. Doing so would change the table and break the second-stage identity. The generic existing session is correct on the proposed adapter because it transforms the reader's zero value, not a post-transform hole mask.

The exact polynomial identity `product_{k=0}^{15}(w(w+1)-k(k+1)) = product_{a=-16}^{15}(w-a)` gives the intended balanced alphabet over the installed q128 field. This uses the existing field policy; it is not an assertion about arbitrary small-characteristic generic fields. The repair does not add a negative-binary predicate on RAM: w(w+1) here is the paired-root virtual transform, not a constraint that every RAM digit lies in {0,-1}.

## 4. Source-domain-only realization, not a fabricated relation

Following the range session, add a thin source realization session using the existing `FullCoefficientSession` engine and `Column` representations. Suggested name: `SourceRangeRealizationSession`. Do not construct a dummy `CoefficientRelationPlan`, zero matrix, rho draw, or `FusedRelationSession` with a fake linear term. Do not coerce the raw view into `FinalReceiverDomainPlan`, which explicitly requires an identity physical source and has receiver-specific binary semantics.

Choose the minimal normalized grammar: the virtual realization has fixed leading coefficient one. There is **no gamma draw** for that single obligatory equation. For a nonempty authenticated physical-zero support Z, draw one fresh independent eta_zero **after svirt has been sent and replay-checked**. If Z is empty, omit both the draw and the zero column/term, as dictated by the installed public grammar, not by the proof.

Define the masked-equality MLE

`Z_tau0(r) = sum_{x in Boolean raw grid, raw_to_physical(x) in Z} eq(tau0,x)*eq(r,x)`.

This is the existing `SourceDomainPlan::zero_weight(tau0,r)` semantics. It is **not** `mask(r)*eq(tau0,r)` and not a guessed physical suffix. It correctly pulls back physical supports through the registered permutation. `Column::RawMasked` supplies the corresponding prover table without synthesizing witness cells.

Run degree-three sumcheck with target svirt for

`sum_x [eq(rvirt,x)*W(x)*(W(x)+1) + eta_zero*Z_tau0(x)*W(x)]`.

For empty Z, drop the second term. Its terminal equation at the new point r, using the one sent raw value v, is

`eq(rvirt,r)*v*(v+1) + eta_zero*Z_tau0(r)*v`.

Both sides must use the same v. The target is svirt (not zero); the optional zero term has target zero. Source/session shape checks must enforce the installed balanced alphabet, mu-coordinate tau0/rvirt, physical length and raw extent. The verifier's constructor consumes a checked `FullRingVirtualClaim` from `FullRingRangeVerifier`, checks `virtual_claim.range_anchor()==tau0`, and checks both point dimensions. The typed driver keeps the virtual claim paired with the same bound source/plan/occurrence; do not permit a cross-source token substitution based solely on equal dimensions.

Return only the pending raw evaluation `(r,v)`. Do not return `(rvirt,svirt)` as if it were a W evaluation, replace the endpoint with an ordinary decoded value, divide by an equality/normalization weight, resample a zero coefficient, or claim acceptance. Zero equality weights or eta_zero=0 are handled by the polynomial protocol and its stated bad-set bound, not an inverse or fallback.

On the prover, an implementation may read the raw adapter as a `Column::Base(view=None, Identity)` and use the original plan for `RawMasked`; alternatively the existing physical source plus `Column::Base(view=Some(plan.view()), Identity)` has identical semantics. Apply the map exactly once. Copy only the O(mu) rvirt point and svirt when needed, then drop the finished range session before starting realization so its retained half-table capacities need not overlap realization state.

## 5. Positive-Boolean Main and the old receiver stay distinct

Main keeps its direct equality-weighted positive-Boolean polynomial W(W-1), not W(W+1). The current `SourceDomainSession` and `FullRingSourceDomainVerifier` can remain the direct **Boolean-only** implementation after their balanced branch is cut over. Restrict their constructors/plan accessors accordingly; merely changing `degree()` from 33 to 5 while leaving the 32 factors is invalid. An enum/stage descriptor is clearer than pretending the new three-stage balanced protocol has one degree.

A physical-zero adjunct on Main still requires its exact installed column-specific support. This repair does not remove the direct Main prefix using frontend reuse: that separate optimization requires the documented frontend/physical-hole coverage certificate. The current direct Main driver unconditionally samples eta_zero; if left unchanged, its actual grammar must record that fact rather than quote a no-adjunct/no-draw 4mu protocol as an exact execution ledger.

The old receiver is **not missing** its range/binary/zero checks. `PendingFullRingProver::update_with_frontend` at `full_ring_accumulation.rs:328-364` calls `prove_full_ring_final_receiver_domain`, then normalizes that endpoint with the retained prior pending claim. Verifier `full_ring_root.rs:712-747` does the corresponding replay. `FinalReceiverDomainSession` uses the retained exact predecessor binary and zero supports; `FinalReceiverDomainPlan::from_relation` carries them, and its identity-view admission must remain intact. These checks must not be weakened or replaced by the fresh RAM plan.

One ledger distinction matters even though those checks are present: the current receiver driver draws all three `(gamma,eta_bin,eta_zero)`, while manifest §2's receiver `16mu+5`/factor `12*288^mu` describes normalized leading-one realization plus two adjunct coefficients. Leaving the current receiver unchanged conservatively requires `16mu+6` and factor `16*288^mu`, and its literal draw count is `4mu+6`. Do not silently price three independent coordinates as two. Normalizing the receiver is a separate optional protocol change, not required for this fresh-source repair.

## 6. Exact ordering, sites and charges

For the proposed balanced branch, per source and structural occurrence:

1. Public binding: authenticated installation/version, owner/source identity, exact view/permutation, alphabet, support/presence, decoder/ordered-claim metadata and original public commitment; all relevant frontend messages remain on the same canonical transcript.
2. tau0: `Operation::RawVector`, count mu.
3. First stage: mu occurrences of proof coefficients count 6, then `Operation::RawScalar` count 1.
4. Quarter frontier: proof count 4; then xi `Operation::CoefficientVector`, count 3.
5. Second stage: mu occurrences of proof coefficients count 6, then `Operation::RawScalar` count 1.
6. Virtual frontier svirt: proof count 1, endpoint replay check.
7. Only if the installed physical-zero adjunct is present: eta_zero `Operation::ScalarCoefficient`, count 1. No gamma, eta_bin or rho is requested.
8. Realization: mu occurrences of proof coefficients count 4, then `Operation::RawScalar` count 1.
9. Raw endpoint v: proof count 1; terminal equation replay check.
10. Existing normalization binds **all** ordered ordinary/raw application claims and the new `(r,v)` before its independent alpha vector; then degree-two rounds, each with all three coefficients, and one raw endpoint. Preserve decoder affine constants and existing terminal multiplication even when its weight vanishes.

Every round sends the constant and all high zero coefficients. Each challenge is sampled after the preceding canonical proof message is absorbed; no early sampling of future rounds. Each occurrence/source/stage/round has a distinct installed site identity. Retain `guarded` poisoning and terminal/exhaustion ownership. An optional stage is controlled by authenticated plan metadata and exact registry presence, not by prover input or a runtime optimization that skips a registered event.

**Derived charges under the existing manifest conventions** (not numerical security claims):

| Source protocol | Conservative numerator / |E| | Mixed-tree factor |
|---|---:|---:|
| Balanced proposed, no adjunct | 14mu+3 | 4*288^mu |
| Balanced proposed, required physical zeros | 15mu+4 | 8*288^mu |
| Positive Boolean, no adjunct and no zero draw | 4mu | 8^mu |
| Positive Boolean, physical-zero adjunct | 5mu+1 | 2*8^mu |
| Receiver as currently implemented, both adjuncts/three independent mix coefficients | 16mu+6 | 16*288^mu |
| Receiver normalized leading-one variant described in old manifest, not current driver | 16mu+5 | 12*288^mu |

Derivation: balanced has mu anchor + 5mu + normalized four-child combination cost 3 + 5mu + 3mu realization. Factors are `2^mu * 6^mu * 4 * 6^mu * 4^mu`. Each added independent normalized adjunct coefficient adds one to the affine dimension; zero-anchor reuse is conservatively charged mu, and the receiver binary anchor reuse another mu. These reused anchors do not create fresh interpolation trees; retain their original full families. One zero adjunct multiplies the base factor by two. This is the same conservative accounting convention used for the original manifest, not a tighter independent proof that its extra anchor term is necessary. Empty-support draws retained by old code still appear in the actual sampler/work ledger; counting such a draw by an extra unit/factor two is safe, whereas eliminating its algebraic charge requires explicitly recording that the polynomial does not depend on it.

For proposed balanced sources, literal challenge count is `4mu+3+z`, where z=1 exactly when the physical-zero adjunct is present; transmitted proof values are `16mu+6` E elements, regardless of z. At mu=21 these are 87+z challenge elements and 342 proof elements. Public headers/result frames and normalization are additional. If an implementer instead copies the receiver's all-independent mixing, these exact numbers and factors change: it is not the minimal contract above, and cannot retain its price silently.

Normalization with Lambda total input claims (including the domain endpoint) has charge `Lambda+2mu` and factor `(Lambda+1)*3^mu`. Current universal driver runs it even for one claim; do not claim a singleton bypass merely because the manifest permits a separate optimization.

The current direct balanced branch has degree-33 sumcheck factors `(34)^mu` and the original anchor factor `2^mu`, i.e. `(68)^mu` before adjunct mixing, not the range protocol's `4*288^mu`. Thus replacing it reduces the displayed scalar/message counts but does **not** establish a uniformly better extraction factor or end-to-end runtime. Emit the actual profile's factor product and work sizes; do not repin the manifest to bless the slower unreviewed implementation.

The work ledger must distinguish original physical cells, raw padded extent, each stage's columns/transforms, retained E work state and verifier zero-mask work. Existing range stage one has four transformed K-read columns (first nonbase bind materializes four half-sized E tables); the leaf stage has one virtual column; realization has one raw column and, only when present, one masked-zero column. Equality columns use their existing compact representation. `SourceDomainPlan::zero_weight` currently scans the entire raw injection for arbitrary permutations; price that verifier work and omit the absent-zero path instead of promising interval-time evaluation on arbitrary maps. This review does not benchmark those scans or prescribe a new memory algorithm.

## 7. Required owner/lifetime and integration checks

Preserve the existing fresh-source chain, in order:

- `full_ring_accumulation.rs::update_root` validates exact profile/public commitment against retained material, then checks contract IDs `[2,2,1]` for RAM-minus, RAM-plus, Main. Contract-ID equality alone is not an instance/continuity check.
- `FullRingAccumulationPlan::new` matches each domain view to the installed original raw view and validates its role's alphabet. Physical supports/decoders/active masks still require the caller's authenticated producer/application admission; nonempty binding bytes do not authenticate them.
- CPU `opaque/admission.rs::validate_commitment` checks backend owner, exact frozen parameters/public commitment, admitted ordered group and source role, binds retained material and admits that commitment ID into the operation's scope.
- CPU `opaque/full_ring.rs::with_full_ring_source` validates session/context and the scope lease's admitted commitment, then borrows `CpuFullRingSourceReader`. Its constructor checks the original coefficient extent and source geometry; `full_ring_source_id` checks installed raw-view equality.
- Construct and consume the adapter within that borrow. A session cannot outlive the reader, physical source, plan or operation. Retain the actual handle/material in `sources/materials` for subsequent opening/reduction; never create a replacement commitment from the adapter.
- `full_ring_accumulation.rs:230-282` already appends the source-domain endpoint to every original application obligation, normalizes under the same view, wraps the normalized result as `FullRingRawClaim`, and passes all original handles/material/public payloads into `reduce`.
- Verifier `full_ring_root.rs:610-699` is the matching consuming dispatch: it selects the installed fresh plan, replays the source domain, appends its raw endpoint, replays normalization, then feeds the result into `replay_reduction`. Both branches of the new domain-site enum must reach this path; neither may silently no-op or return a virtual-table endpoint.
- Keep application RAM-handle continuity via the existing instance-identity seam (`same_full_ring_source`), VM adjacency, active lengths, and ordered aliases. This repair does not add these properties merely by reusing a reader.
- The old receiver continues to use the prior pending claim and predecessor supports. Endpoints remain pending until the actual outgoing reduction, shrinking suffix and direct terminal, and ultimately the full authenticated archive/final receipt, discharge them.

## 8. Minimal file/symbol change map for the implementer

1. `akita-types/src/full_ring/strong.rs`: add borrowed raw adapter and thin balanced source-realization session on `FullCoefficientSession`; reuse `ProductRangeSession` unchanged; make `SourceDomainSession` Boolean-only or route by a checked protocol enum. Remove the degree-33 balanced production path. Preserve receiver/strong sessions and their existing exact supports.
2. `akita-types/src/full_ring/query.rs`: retain `SourceDomainPlan` view/alphabet/physical-support semantics and exact `zero_weight`; replace ambiguous single `degree()`/direct balanced endpoint use with explicit Boolean versus range-stage grammar; add the source-realization endpoint helper. Do not weaken `FinalReceiverDomainPlan` to admit padded raw sources.
3. `akita-transcript/src/native_strong.rs`: make `SourceDomainSites` an explicit Boolean/direct versus Balanced/product-range grammar (or equivalent checked typed variants). Balanced includes binding, tau0, first, quarters, xi, second, virtual_value, optional eta_zero, realization rounds and value—no rho/eta_bin/gamma. Validate variant/alphabet/presence and all mu round counts before draws. Keep separate receiver sites; their three-coordinate mix remains explicit.
4. `akita-prover/src/protocol/prove/full_ring_strong.rs::prove_full_ring_source_domain`: explicit checked dispatch; adapter + existing range stage order + source-only realization; return actual raw endpoint. Keep direct positive Boolean behavior, including its literal site pricing if unchanged.
5. `akita-verifier/src/full_ring.rs`: reuse `FullRingRangeVerifier`; add corresponding balanced source-realization replay consuming `FullRingVirtualClaim`, checking its initial anchor and exact raw geometry; retain/limit `FullRingSourceDomainVerifier` to the Boolean direct branch.
6. `akita-verifier/src/full_ring_strong.rs::verify_full_ring_source_domain`: mirror all actual site counts/order, full-coefficient lengths and optional-zero presence; no discarded frontier/endpoint. Do not retain a balanced degree-33 fallback.
7. `akita-types/src/full_ring_accumulation.rs`: migrate `FullRingFreshSourceSites`; version/bind the changed source-prefix grammar in `fresh_prefix_descriptor`/installation descriptor and include deterministic adjunct presence. Preserve decoder selection, physical support, claim bounds and occurrence binding. The new protocol must not be indistinguishably labeled as an old installation.
8. `akita-prover/src/protocol/prove/full_ring_accumulation.rs::update_root` and `akita-verifier/src/full_ring_root.rs::update_with_frontend`: wire/check typed variants for each installed fresh role, retaining the existing normalization and owner chain. Existing receiver-domain calls are intentionally unchanged. `full_ring_root.rs` final normalization/terminal paths do not need a fabricated fresh-prefix replacement.
9. Registry/admission integration: `native_strong.rs` explicitly says it transports installed keys and does not emit/certify a registry. The current searches found the source sites' declaration and driver/container uses, not an implemented production constructor emitting this new source grammar. The integration owner must register the actual per-occurrence stage/degree/count/presence/sampler/dependency ledger and its accepting state through `InnerExecutionRegistry` (and any selected outer verifier-side registration), including mixed K/E2 frontend context separately. A generic struct alone is not the certificate.
10. `akita-types/src/full_ring/tests.rs`, `akita-verifier/src/full_ring_tests.rs`, and appropriate typed transcript/registry tests: extend the existing source/range/replay coverage for the new grammar; do not run them in this research assignment. `AKITA_INSTANCE_ACCUMULATION_NATIVE_REUSE_PROFILE.md` §8 and `AKITA_INSTANCE_ACCUMULATION_GLOBAL_MANIFEST.md` §2 should record the implemented branch/optional adjunct charges after integration, not erase the intended repair by pricing the direct degree-33 branch as the selected optimized protocol.

No CPU source import, commitment matrix/plane layout, physical decoder, ordinary K/K frontend/receipt, receiver path, setup law, final decider or authentication weakening is necessary for this local repair. Those existing revision gates remain mandatory and are not admitted by this review.

## 9. Parent-owned acceptance checks to add/run after implementation

Not executed here:

- Range/realization equivalence on a small non-power-of-two physical view with both digit and polynomial padding and a nontrivial registered permutation; verify virtual holes become W=0 while higher quarter transforms remain their true nonzero constants.
- Preserve original noncanonical 26-plane RAM digits, original handles, physical indices, public commitment and retained decommitment path; do not compare only decoded values.
- Out-of-range alphabet (-17,16), nonzero required physical-zero cells, substituted virtual values, quarter claims, raw endpoints, initial anchors and cross-source/prefix tokens must fail the relevant algebra/replay or final opening. Include valid 0, -1, -16, 15; a nonconstant off-cube example distinguishes U(r) from W(r)(W(r)+1).
- Positive Boolean Main accepts 1 and rejects -1 in its positive-Boolean role; ordinary RAM is not constrained to {0,-1}. Receiver binary/zero regression preserves its existing full-domain call and incoming-claim normalization.
- Full coefficients (including high zeros), complete stage ordering, optional adjunct presence, exact round counts, site reuse/reordering, wrong typed variant, canonical E encoding, poisoned compiler on failures, and terminal/EOF completion checks.
- Raw/ordinary normalization includes every original alias, the true raw endpoint, affine decoder constants and zero-weight cases without division. Owner/scope/view mismatches reject before a borrow; the same physical reader supplies subsequent opening/reduction.
- Emit exact proof-element/draw/degree/arity/mixed-tree/work ledgers and compare those counts—not an assumed 14mu+3 attached to the old driver. Then measure complete profiles only under the parent-approved verification plan.
