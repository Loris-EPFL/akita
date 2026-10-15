# Lighter full ordinary PCS extraction in classical indexed ROM

## 1. Scope, result, and status

This changes proof analysis only: no protocol, transcript, source commitment, matrix, code or honest computation is changed. It imports neither the old native Theta/ARO environments nor a concrete-hash/preimage assumption. All counterfactual oracle tables are classical private indexed-ROM simulations under the same original prover tape and joint public setup.

The retained user policy does NOT require a tight end-to-end 2^-128 reduction as an additional acceptance criterion: `AKITA_INSTANCE_ACCUMULATION_FIELD_POLICY_COMPARISON.md:74–100` states this explicitly. The objective here is a truthful conditional correctness/soundness theorem, explicit kappa(Q,T), and lattice advantages priced at the complete reduction runtime. No numerical profile, security-bit claim or production admission is established.

**Result:** the source-first route can remove all nonlinear strong-helper interpolation trees from a full ordinary PCS extractor. Start from the actual checked terminal response; retain the original flat unit-difference shell forks. At each nonterminal shell child, first extract the committed successor's source with the already established DEEPER suffix certificates. One matched-successful source probe fixes that source for the local range/ring/row/norm/fused-relation DAG; local soundness then verifies its exact original branch predicate. This is not circular: the certificate used to compare successor sources is not the currently pending range of the successor's decoded witness.

Tensor and post-payload scalar prefixes have a DIFFERENT incoming-source cut. They can also use source probes, but their cost generally multiplies the strong-cut cost rather than sharing its original deeper-source call for free. With one strong cut and one combined incoming-prefix cut, the leading per-receiver coefficient is 4*b_shell; without the incoming-prefix cut it is 2*b_shell. The generic b_shell*(s_root+1) coefficient is valid only for s_root probes sharing one original descendant-source extraction at the SAME cut.

One further all-roster source probe replaces the sealed outer query interpolation tree. Thus the query factor m*3^n is not necessary for this proof route. The retained shell product, source decoding/comparison, strict cutoffs, oracle-query horizon and all induced MSIS instances still have to be priced.

## 2. Evidence actually inspected

The probabilistic fork law is grounded in the published Fenzi–Moghaddas–Nguyen Journal of Cryptology 2024 paper, §8, primary PDF https://infoscience.epfl.ch/server/api/core/bitstreams/9abd4c0d-3e31-4228-b600-28f51a7a511f/content . The marked-original scalar/star proof is independently derived in `AKITA_RELATION_ACCUMULATION_FRONTEND_COMPOSITION_PUBLISHED_ANCHOR.md`. It proves original-event success containment and weighted cheap-mismatch accounting; it does not claim that the missing Figure12/13 artwork was read.

Akita-specific algebraic equations below use `Papers/parsed/Akita.md`, not an independently authenticated official Akita body:

- lines2290–2388, Eqs104–113: fold response, successor layout, certified digit/response envelopes, successor fixed before alpha;
- lines2390–2495, Eqs114–115 and range pipeline: derived Boolean products, initial anchor, sequential product sumchecks and binary support;
- lines2720–2815, Figure6/Eqs138–146: complete native carrier/A/B/D/F/H and scalar opening rows;
- lines3709–3865, Definitions10.13–10.14, Eqs199–208, Lemma10.15/Corollary10.17: algebraic decommitment, bounded products, source-only comparison across points, incoming tensor ownership;
- lines3880–4125, Proposition10.20/Theorem10.23/Lemmas10.24–10.30: terminal, shell subtraction, exact descendant conditions, norm/range/ring/offloading obligations.

Previous source-first notes were inspected as research context: `AKITA_INSTANCE_ACCUMULATION_NATIVE_SOURCE_FIRST.md`, `AKITA_INSTANCE_ACCUMULATION_SOURCE_FIRST_EXTRACTION.md`, and `AKITA_INSTANCE_ACCUMULATION_PCS_EXTRACTION_COST.md`. Their Theta/ARO/emulation assumptions are NOT imported. The probability and cost transformation below is rederived for the ordinary classical indexed-ROM experiment.

Immutable actual verifier source, commit `83574331d4e51f8cce1d8689d05592fa4ef4c138`, was independently read:

- https://raw.githubusercontent.com/LayerZero-Labs/akita/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-verifier/src/fold/mod.rs : `derive_multi_group_stage1_challenges`, then `receive_next_witness`, then `ring_switch_verifier`, Stage1, Stage2 replay, Stage3, and delayed Stage2 final validation. `NextWitnessPlan` dispatches both ordinary outer payload and final `TerminalT`.
- https://raw.githubusercontent.com/LayerZero-Labs/akita/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-verifier/src/stages/ring_switch.rs : alpha, tau0 and tau1 are drawn after the successor; their dimensions come from actual address/row geometry, and no prover message intervenes in this function.
- https://raw.githubusercontent.com/LayerZero-Labs/akita/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-verifier/src/fold/terminal.rs : terminal singleton and no-pending-setup checks, predecessor-bound A-image tuple, tensor-prefix dispatch when needed, partials before shell draws, bounded clear response, direct ring and trace checks.

These reads establish ordering/dispatch facts, not an implementation-wide GRID/sampling/ownership certificate. The current concrete transcript's grinding functions are not silently equated with ideal independently indexed raw-answer samplers.

## 3. Original execution and the source-map contract

Include the actual initialization, ordinary prover and verifier in the original execution C. Its record o0 contains its original public statement, proof, auxiliary output, decision, original exposed oracle answers and resettable ordinary states/tape. Let q bound its fresh external ideal-oracle entries, including allowed oracle-dependent initialization. Alternatively retain the published Q+1 unqueried-output convention consistently.

Every extractor map returns o0 and the ordinary execution's selected root key/header even on failure. Success means an appropriate witness for THIS original record or a verified named matrix collision. The map first follows C's actual execution; private successful reruns never replace o0. All private tables and prefix reconstruction are charged work, not new original q-budget queries.

A deeper suffix source map D must return:

1. a valid algebraic source/decommitment for EVERY owned outgoing handle and original point/value leaf;
2. each source's unit/product certificates rho*s=r with certified bounds;
3. certified commitment-side B/F alphabets/path state, or terminal A-only state;
4. exact full-grid decoded vectors and the actual leaf equation, including any padding factor/map;
5. a verified collision or explicit failure otherwise.

Returning a source satisfying only its commitment, or authenticating only a combination where the parent needs two individual leaves, does not meet this contract. The incoming tensor/batch transformation below supplies the latter upgrade before a receiver is used as its predecessor's D.

The initial ordinary record is the only external oracle execution. Extractor maps run ordinary private continuations; the original external output/view is preserved. Constant auxiliary data is independent of the ideal oracle, or efficiently generated with its initialization queries included. No unmetered oracle-correlated advice or concrete hash programming is introduced.

## 4. Source-probe transformation: exact law and cost

At a cut, all handles whose source projections are compared are bound in the root header. The first fresh root-family entry has an original ordinal i; its actual key, header and first component identity are retained. Stage/family keys parse the complete preceding transcript. Proper descendants bind the required root answer coordinates.

Run B=C plus its gated deeper source procedure once. On rejection/failure, preserve that original result. On collision, return it. On a valid original source W0, restore the ordinary state/tape and ONLY the oracle table as it stood immediately before the selected first root query. Repeated independent trials use a fresh root and fresh private future for entries absent from that prefix. A trial must accept and select the SAME i, exact key and complete root header before invoking D. A failed D restarts; a valid source W' is compared with W0; a verified collision terminates.

The exact gate is more than 'same commitment': it binds source shape/view, point/public context known at the cut, earlier accepted challenge values and all required ordinary ancestors. It is evaluated cheaply before D.

For a reached prefix sigma, let

`a_i = Pr[C accepts and selects this exact root | sigma]`,

`p_i = Pr[C accepts, selects it, and D returns valid sources | sigma]`.

The original and each independent trial have this SAME unconditioned continuation law. On p_i>0, the first successful trial source has distribution

`nu_i(A) = Pr[one trial succeeds with source in A | sigma]/p_i`.

It is independent of the original future conditional on sigma. Prefixes with p_i=0 have zero original valid-source-start mass, so do not start an infinite loop with positive probability. The finite algorithm need not decide p_i.

Let mu_i be the original reached-prefix subprobability measure. Then sum_i ||mu_i||<=q. For a nonnegative trial cost Y with mean c_i, renewal gives expected trial work c_i/p_i. Multiplying by ORIGINAL start mass p_i cancels exactly on p_i>0. Therefore:

- additional cheap trials have unconditional expectation at most q;
- additional expensive invocation count is at most `sum_i integral a_i dmu_i <= 1`, because exact original selected-index/acceptance events are disjoint;
- additional expensive work is at most the expected original gated D work P_D, by the same identity applied to `Y=1_gate*work(D)`.

Consequently one source probe costs at most

`(q+1) T_C + 2 P_D + A_compare/check`,

or conservatively `2 C_B + q T_C + A_compare/check` when C_B includes the original cheap run and D. There is NO independence assumption between D cost and success and NO lower bound on positive p_i.

For s probes sharing the SAME original B/W0 and correct original point leaves, linearity gives

`C_new <= (s+1) C_B + s*q*T_C + A`.

This is not 2^s. It does not apply when probes own different cuts whose original source maps depend on each other.

## 5. Fixed-source probability coupling and old descendant keys

For each candidate root family encountered in the original ordinary execution, before its first fresh answer assign an independent hypothetical source with law nu_i. This is a proof coupling, not an implementation that runs q extra extractors. The hypothetical source depends on the exposed prefix and independent continuation randomness, not the original future. On the original valid-source selected event, couple the actual one implemented probe to that source.

If source projections differ, return the certified collision. Otherwise all original owned leaves and source-determined polynomials refer to this one hypothetical source. This permits fixed-source local soundness WITHOUT conditioning original challenges on source equality, acceptance or successful extraction.

Let epsilon_local be a verified sum of local first-false residual bounds for the chosen protocol DAG relative to that fixed algebraic source. All adaptive messages after a challenge are allowed. Boolean product tables are formed on the cube and then interpolated; no off-cube product shortcut is allowed.

Fresh local families have their source and pre-challenge transcript fixed before their first answer. At most q first-family hits give the safe bound q*epsilon_local (or a sharper actual per-hit degree ledger).

A previously queried proper-descendant key may have guessed the new root before the hypothetical source was assigned. If each such key fixes a candidate fresh root coordinate/tuple with maximum point probability gamma_root, then

`Pr[old used descendant] <= sum_i(i-1)*gamma_root = binom(q,2)*gamma_root`.

Thus the original-event transformation loss is

`beta_cut(q) <= q*epsilon_local + binom(q,2)*gamma_root`.

For a uniform scalar root in a field of size K, gamma_root=1/K. A full atomic tuple permits its full point mass only if every relevant old descendant key fixes that whole tuple. Having no intervening prover message in a function does NOT prove that the actual chained stream keys have this property. For independently indexed components use the earliest exposed component and the actual parsed descendant condition. Sampler failures/bias and nonce grinding are separately owned terms; no proof-of-work discount of the whole extractor is inferred.

Multiple cuts always allow the safe sum of their beta_cut values. A tighter SINGLE old-query term using gamma_max is available when their parsed root families have disjoint cut ownership and the following joint ledger is actually established: install all their hypothetical sources online on the same original ordinary query record; each root/old-key pair belongs to one owner; all failures on the marked original chain are either this joint old event, a fresh local residual event, or a retained-shell threshold event. Then union the old pairs once to obtain `binom(q,2)*gamma_max`, rather than charging the same original pair once per cut. Do not claim this improvement without that unique-owner/joint-coupling certificate.

## 6. Why recursion does not multiply child errors or Q per depth

A retained shell's suffix-map success bit includes the child's actual complete extraction, including its source probes, cutoffs and possible failures. It is NOT ordinary acceptance with an unconditional child error inserted afterwards. The published marked-original game starts from this actual suffix bit, retains its original result, and tests siblings only on an original success.

For a flat shell with b=1+sum_u(k_u-1), root loss e=sum_u(k_u-1)/|C_u| and original support P0<=Q+1,

`s_shell >= s_child - P0*e`,

`C_shell <= b C_child + (b-1) P0*T_cheap + A_shell`.

The scalar weighted proof allows arbitrary cost/success dependence; failures are removed from the good fibre before its threshold is priced. Neither source-probe nor shell error is multiplied by every private counterfactual invocation.

Critically these are UNCONDITIONED original-table map recurrences. One may NOT condition on a matching/accepted/forced ancestor and then assert the same unconditional child bound or trial success probability. At a private source probe keep its actual saved ordinary prefix and use a genuine fresh reference continuation. Successful exact-header matches automatically preserve required decoded ancestor values. Do not retain future original answers just to force a match.

If a separate implementation instead supplies a restricted-game interface, its reference kernel and trial kernel must both honor the same legitimately prescribed context, and its uniform claims must be proved. Conditioning on a selected ancestor key can bias future source roots; 'legal restriction' is not a license to condition on arbitrary success. The unconditioned weighted-map construction above needs no extra assumption that all such success-conditioned kernels are uniform.

Raw-answer conditioning at a retained coordinate fork uses the existing exact conditional sampler and keeps the original full answer as the marked child. It is not a concrete hash preimage operation. Private internal probes are separate worlds; they do not modify the original table supplied to the enclosing sampler. The enclosing game sees a map which still returns its ORIGINAL index/record, whatever occurred privately.

## 7. Terminal base, including its original point

At the actual terminal the predecessor binds its source shape and A-image tuple (t_i). There is one recursive witness claim and no pending setup source. There are no invented terminal B/F/D/H paths or terminal strong range trees.

Retain the terminal flat shell factor b_T=1+B_T. Each accepted child has directly decoded/bounded response z and exact ring equations

`A_T z = sum_i c_i t_i`,

`a_T^T G_T z = sum_i c_i e_i`.

The revealed partials and their trace target are fixed before shell draws. For siblings differing only at i, use the certified unit difference rho_i=c_i'-c_i and r_i=z'-z. Then

`s_i=rho_i^-1 r_i`, `A_T s_i=t_i`,

and subtraction gives the native partial evaluation. The trace equation is checked on an accepting child, not obtained by subtraction. Together they authenticate the reduced packed target.

The weak certificate is `||rho_i||_1<=2*kappa_T`, `||r_i||_inf<=2*Z_T`, or its actually checked Euclidean/operator alternative. A diagonal coefficient collision is bounded by 8*kappa_T*Z_T. This DOES NOT prove the extracted s_i short.

When terminal tensor reduction is nonidentity, finish the incoming-source prefix transformation in §10 using this A-only source certificate to authenticate the original incoming point/value. Alternatively retain the actual terminal tensor tree. The strong-node predecessor cannot use merely the packed/reduced terminal claim as if the original leaf were already true.

## 8. Nonterminal strong cut and the absence of circularity

Fix ONE shell child. Its incoming handles, shared D/H payload, scalar coefficients, shell tuple, response routes and successor Cf are in the header before alpha/tau0/tau1. Outgoing Cf can differ across shell children; that is intentional.

Use the complete deeper D to authenticate Cf's original final source point/value and any separately registered setup handle. Source-probe comparisons for Cf use:

- its DEEPER incoming-source rho*s=r certificate, derived using the next level's already validated response;
- its DEEPER-certified B/F path alphabets;
- or the terminal's directly checked A-only products.

They do not assume that Cf's decoded witness w is in the currently pending range. In particular source gadget blocks can remain long/noncanonical.

For common source paths, the A collision is

`z_A = iota(rho')r - iota(rho)r'`,

nonzero when source blocks differ and annihilated by A. Its coefficient bound is K'*B+K*B'; apply only actually available mixed Euclidean alternatives. B/F disagreements use deeper-certified cross-profile alphabet diameters. Source comparisons discard opening-local D/H; they do not need equal query points.

Now derive the full decoded w from the valid original source, compare it with one matched source probe, and directly test the ORIGINAL branch predicate:

1. every native ring row and scalar opening row, with exact carrier/trace maps;
2. range on every required physical/virtual cell and negative-binary support on every F/H span;
3. reconstructed response norm on every required response coefficient, with the actual integer-lift prerequisites;
4. exact quotient placement/recomposition, or quotient-free transpose weights;
5. installed setup reference and delegated product identities when selected;
6. every used padding/alias/structural-zero condition, with its actual owner.

The test is deterministic and polynomial in the explicit extracted grids. It is not a verifier shortcut or a protocol change. On a false predicate, local soundness plus true owned leaves forces a bad event; outside the beta_cut bound the source-probe transformation succeeds with this exact predicate.

For setup offloading the outgoing setup handle is separately registry-fixed and its source is compared to a canonical installed reference, when that reference is available. Its point/weight may vary later. Stage2's final validation uses Stage3's delegated scalar; the local certificate is a DAG containing that dependence, not two independently assumed true stages. Include the canonical reference's rho'=1,r'=s_static in the mixed source inventory. An unknown setup-reference source or an unowned setup leaf is a missing contract, not free advice.

### Local certificate ledger

For paper-level full-grid geometry, a conservative strong-block epsilon contains:

- `(2*d_max-1)/K` for quotient lifting, or `(d_max-1)/K` for correct quotient-free reduced residuals;
- `ceil(log2 n_rel)/K` for relation-row MLE batching;
- `mu'/K` for the initial nonzero range-residual MLE anchor;
- `sum_s mu'*(d_s+1)/K` for actual range-product rounds;
- actual interstage batch charges, affine 1/K or the proved power degree/K;
- an additional safe `beta_binary*mu'/K` for the fixed Boolean binary residual at the reused final range point;
- `(1+beta_binary)/K` for range/binary/fused separation;
- `3*mu'/K` for fused relation rounds;
- norm scalar charges SUMMED OVER EVERY selected group: direct 2/K, digit-expanded `(N_pair,g+delta_f,g)/K`, or zero on a coefficient-only route;
- `2*nu_setup/K` for an offloaded setup-product sumcheck;
- actual additional range-image/padding/alias/exceptional-branch charges.

This ledger is derived from true residual degrees, not honest message-size counts. Define range helper nodes by Boolean multiplication before interpolation; `MLE[w(w+1)](r)` is not `MLE[w](r)*(MLE[w](r)+1)`. All later adaptive messages are allowed.

For Euclidean norms, integer truth follows only after certified digit bounds and no-wrap/pair-lift hypotheses. This backward truth argument never assumes current digit bounds to establish source comparison. The actual code's range-image/physical-response geometry still needs its own GRID/LOCAL proof; the paper's mu' is not a substitute.

## 9. Retained shell subtraction recovers the original incoming weak source

Apply the marked flat unit-difference shell game to exact branch continuations from §8. It retains b_j=1+sum_g W_j,g children with error `e_shell,j=sum_g W_j,g/|C_j,g^acc|`.

Every accepted noncollision child now has EXACT native row identities and actual certified response/digit bounds. Its successor may differ from siblings. Compare the ORIGINAL incoming B/F paths and their shared D/H opening path across these shell children. Shared D/H comparison is required HERE to cancel the opening terms. It is not required across the separate source-only probes.

After common paths are established, subtract a pair differing in one shell coordinate. The same carrier unit difference and its admitted embedding yield

`iota(delta c)*s_in = z'-z`,

`A*s_in = recomposed inner image`,

and the exact scheduled partial-evaluation row. The scalar trace/opening row remains from its authenticated branch. Thus the incoming weak product certificate has B_inf equal to the verifier-certified response DIFFERENCE envelope

`Delta_f = (b_star-1)*(b^delta_f-1)/(b-1)`

on the coefficient route, or the corresponding difference norm from checked integer Euclidean bounds. No norm of the inverse unit is used. Every per-claim source occurrence is coalesced by the same original A/B/F source comparison class or yields a collision.

The resulting map authenticates the post-tensor/post-payload combined target, with exact native partials. It has not yet automatically authenticated every individual original incoming claim. That final point obligation belongs to the next section.

## 10. Incoming tensor and scalar prefixes: a different cut

The sources here are the INCOMING handles, not the just-created successor. They are fixed before the tensor's row/claim prefix and before any internal post-payload scalar combination. The raw shell map of §9 returns those source projections with current-level weak product certificates and the final weighted native equation.

For fixed incoming sources, canonically construct packed Boolean tables and true column partials. Incorrect head recombination implies a nonzero base-coordinate row discrepancy because the packing/basis regrouping is invertible. Row-selector error is kappa/K, a nontrivial normalized claim batch adds 1/K, and degree-two tensor sumchecks add 2*m_max/K.

For heterogeneous tails retain the actual native prefixes and cylinder factors

`theta_a = A_eta,a(rho_a)*product_{t=m_a}^{m_max-1}(1-rho_t)`.

Each factor is checked nonzero before scalar division. The final incoming-prefix truth includes each forwarded `h_a,i=theta_a*g_a,i(rho_a)`. Post-payload normalized batching then adds at most 1/K when nontrivial: its source-determined claim error vector is fixed before its own coefficients. For the application POWER batch, use its actual degree `(Lambda-1)/K`, not affine 1/K.

One matched source probe at the earliest tensor/batch root can cover this entire prefix DAG, including post-payload batching, because the SAME incoming source tuple is fixed at that cut. Its downstream truth is the raw shell's weighted equation. It upgrades the ORIGINAL native input points and values, not a probe's alternative points. Source comparison ignores D/H, though the batching header retains every public payload required by its local equations.

For a nonidentity internal tensor plus normalized batch, a safe fixed-source charge is

`epsilon_prefix = [kappa + 2*m_max + 1_{tensor batch nontrivial} + 1_{post batch nontrivial}]/K`.

Identity tensor with singleton input has no prefix cut, no error and no factor2. Identity tensor with a nontrivial batch still needs its batch cut or retained tree. A registered setup source remains a separate incoming handle and is authenticated individually before the receiver becomes its predecessor's complete deeper D.

This transformation compares incoming source projections, whereas §8 compares the successor source. Consequently their coefficients generally multiply: they cannot be priced as two extra probes of one original source extraction without a further sharing proof.

## 11. Full ordinary recurrence and polynomiality

Let C_{j+1} price the COMPLETE deeper map, including its failures and its original ordinary replay. Let s_str,j probes share the fixed-successor cut (one whole-strong-DAG probe gives s_str,j=1). Let s_pre,j probes share the incoming-prefix cut (one combined tensor/batch probe gives s_pre,j=1, omitted identity singleton gives 0). Let A terms include actual raw sampling, grid decoding, comparisons, exact predicate tests, interpolation/subtraction and collision output.

Then

`C_str,j <= (s_str,j+1) C_{j+1} + s_str,j*q*T_C + A_str,j`,

`C_raw,j <= b_j C_str,j + (b_j-1)(Q+1)T_cheap,j + A_shell,j`,

`C_j <= (s_pre,j+1) C_raw,j + s_pre,j*q*T_C + A_pre,j`.

Thus the leading coefficient is

`lambda_j = (s_pre,j+1)*b_j*(s_str,j+1)`.

The terminal uses its actual b_T and an optional incoming tensor cut, not a nonterminal strong copy. Expanding `C_j<=lambda_j C_{j+1}+A_j` gives

`C_PCS <= (product_j lambda_j) C_base + sum_j (product_{v<j}lambda_v) A_j`.

Q occurs in additive cheap replay terms, NOT as a multiplicative q*C_suffix factor at every depth. Nonlinear range/row/ring/helper sumcheck arities are absent from the leading product when their source-cut certificates are supplied. Retained shell products and source-cut coefficients remain and can still be expensive.

Define `Lambda_light=(1+s_pre,T)*b_T*product_{j nonterminal} lambda_j`. A family is expected-polynomial only if this coefficient, reconstruction work, sampler costs and explicit output size are bounded by a FIXED polynomial in security parameter and total explicit instance/witness length. No such bound is inferred for every possible depth/fold-width family. Removing range trees does not prove that an arbitrary product of large shell widths is polynomial or practically cheap.

The safe original-event ordinary loss, before global cutoff/setup/MSIS terms, is

`kappa_PCS <= (Q+1)*sum_j e_shell,j + sum_cuts [q*epsilon_cut + binom(q,2)*gamma_cut] + sampler/initialization deviations`.

It includes the terminal shell and all chosen prefix/strong cuts. Under the jointly proved unique-owner ledger in §5, the old-key sum can instead be one `binom(q,2)*gamma_max`. No child extraction error is multiplied by the number of privately tested branches; actual source successes define each parent's good fibre.

This theorem uses the same original output, weak-source equations, source profiles and complete ordinary point roster as the full ordinary PCS. Missing local cuts retain their actual old trees; an unproved source/point/geometry contract is not repaired just by retaining a tree.

## 12. Sealed outer query: one all-sources probe, no m*3^n tree

After sealing, all original source handles, decoders, query points/weights, unique scalar exponents, target coefficients and full frontend prefix are fixed. Let S0 be returned by the lighter FULL ordinary PCS map D, authenticating the ORIGINAL complete terminal endpoint roster and retaining its weak certificates.

Take one matched successful source probe from the first fresh query root a, with the exact sealed header. Each trial runs the original query PIOP and its OWN full ordinary PCS continuation; successful trials return all sealed original source projections. Their query sumcheck points and opening-local D/H can vary. Compare every sealed original A/B/F projection with S0. Any disagreement yields the properly priced original-source collision.

For a fixed common source assignment the false formal residual has degree at most m-1 in Z, because every independent scalar has its unique exponent. After one uniform a, weighted public query times cylindrical source has individual degree at most two in X. Sequential sumcheck therefore has fixed-source error

`epsilon_query = (m-1+2n)/K`.

The public query MLEs/decoder adjoints and all native endpoints are exact; every final source component is owned by D. Cylinder extension leaves the source arbitrary off its natural prefix and gives zero support only to the public query weight. No canonical source-padding claim is inferred.

The source-probe theorem yields

`kappa_acc <= kappa_PCS + q*epsilon_query + binom(q,2)*gamma_a`

before cutoff/setup/MSIS terms. Uniform scalar a gives gamma_a=1/K. Use the same optional global old-pair tightening only when its joint owner ledger is proved. Expected work satisfies

`C_acc <= 2 C_PCS + q*T_C + A_all_sources/query_check`.

There is no multiplication by m*3^n. The deterministic check computes all original coefficient equalities on S0 and returns THIS original assignment/query witness. It may be slow, but is polynomial in the explicit retained query/source state. m and n must still have a declared finite horizon; if the degree reaches field-cardinality scale, the stated bound becomes uninformative and polynomial identities can vanish on the entire challenge field. This is an explicit horizon issue, not a new 128-bit requirement.

Applying the already derived per-target frontend source-probe composition adds its genuine fixed-source frontend/error/old-key terms and its actual expected work. There is no automatic T-fold multiplication of kappa_PCS: original descendant failure is charged once; failures in successful-source rejection probes are included in their p_i and paid as work. All VM/source/advice scope and byte/continuity certificates remain necessary.

## 13. Strict budgets and lattice pricing, without a new bit target

First define the complete almost-surely terminating ideal extractor and its finite expected charged work W. For a strict global budget L covering every original/probe replay, private oracle bit/rejection sample, decoding, source comparison, exact predicate, setup simulation and output operation,

`delta_L = Pr[work>L] <= W/L`.

Choose L and report this loss truthfully; no fixed delta_L=2^-128 is imposed. An explicit cap K_trials at one source probe gives original-success-weighted exhaustion at most `q/(K_trials+1)`. Multiple independently capped nodes need their correctly induced original-reference loss ledger; blindly unioning each unconditional child cap over private invocations is not justified. One total work cutoff avoids that ambiguity.

A first-moment bound does not imply a quadratic-query or second-moment bound. For geometric success p, original weighting of its second moment behaves as 1/p. Use a strict L-derived bound wherever a nonlinear runtime/query expression needs one.

Under the ideal uniform joint setup, a returned named collision is reduced to its ordinary MSIS view by embedding the challenge in the original flat setup coefficient vector and filling all other coordinates uniformly. All overlapping A/B/F/D/H and registry views are derived from that SAME vector. An arbitrary revealed seed cannot be inverted to make an embedded matrix; a valid setup-namespace simulation is still required. Source comparison uses every reachable mixed occurrence pair, not just honest or diagonal radii.

For each induced instance I, let T_I(L) include the WHOLE bounded producer, setup construction, all private counterfactual work and collision extraction. Then the conditional admitted bound has the form

`Pr[original accepts and no original relation witness]`
` <= kappa_stat(Q,T) + delta_L + Delta_setup + Delta_sampling`
`    + sum_I Adv_MSIS_I(T_I(L), eta_I)`.

kappa_stat is the explicit shell/source-cut/query/frontend formula above, optionally tightened only with its joint parsed-owner proof. eta_I is the actual certified mixed radius and dimensions, including terminal and canonical-reference occurrences. Delta_setup and Delta_sampling denote separately owned deviations, not hidden changes of model. Classical ROM is not QROM, and no concrete hash knowledge follows.

If these priced terms are large or the bound reaches one, state that the declared concrete horizon currently has no useful guarantee from this reduction. This is not a measured attack and does not establish impossibility. Select a truthful declared budget/profile and retain the user's policy rather than inventing a compulsory end-to-end 128-bit gate.

## 14. Explicit failure examples and remaining gates

**Weak source is not short.** Over F101, s=51 (centered magnitude50), rho=2 and r=1 satisfy rho*s=r. The product can be short while s is outside a small digit alphabet. The lighter extractor checks actual decoded w by local range soundness; it never grants its range from this weak certificate.

**No deeper product bound means no short collision.** For a public one-row A=(a,b), the easily computed source difference (b,-a) is an algebraic kernel vector. Its coefficients can be field-sized. A source map returning arbitrary decommitments without certified products does not produce the small-norm MSIS collision needed to coalesce Cf. Pricing Cf with its currently pending range in order to establish that range would be circular. Terminal/deeper certificates are essential.

**Source chosen after the root defeats fixed-source soundness.** For a relation asserting a polynomial is identically zero, a source F(X)=X-r selected after seeing r passes its one-point identity test while the polynomial is nonzero. A matching root header must fix the source commitment/view before that challenge; later primitive sources need a separate causal proof.

**Future-answer preservation breaks the reference trial law.** A replay that keeps post-root original answers while changing the root is not the fresh continuation used in p_i. Reusing an unconditional suffix bound after forcing a rare ancestor may likewise introduce an unpriced inverse-success factor. Use the unconditioned map/weighted game and exact prefix reset described above.

**Source-only validity is not owned-point validity.** A valid Cf decommitment with a false original claimed evaluation does not make the strong DAG's last leaf true. Complete native points, scalar combinations, tensor factors and all registered setup leaves must be authenticated before using the map as D.

**Padding/decoder holes remain semantic gaps.** An ordinary equation `MLE(P_full)(r)=phi*v` is not `P_full=Pad(w)` and gives no missing raw-plane predicate. Zero-omitted physical cells or uncontrolled commitment-grid tails cannot be certified short/zero by a logical predicate that never reads them. Supply the exact full-grid/range-image/physical-response maps and all nonzero-factor or used-slot proofs. This is required by both old and new extraction routes.

**Norm field equality is not integer equality.** A sum of integer squares differing by a multiple of q is equal in the field. The direct and digit-expanded no-wrap/pair-lift hypotheses, complete response-coordinate map and canonical integer comparisons remain mandatory.

Remaining concrete gates: actual GRID/LOCAL ownership for every selected range/norm/ring/offload/tensor path; deeper SOURCE certificates and mixed radii at every cut; original-output reset/cheap exact-selection interfaces; exact full raw conditional sampler; canonical/session/descendant-key grammar and all old queries; fixed finite profile family and field/extension horizon; honest closure; explicit retained shell/source-cut runtime and strict L; valid joint setup simulation and runtime-priced MSIS instances. Control benchmarks alone establish none of these proof contracts.

## 15. Conclusion

There is no inherent norm circularity in the proposed successor-source cut. A complete lighter ordinary extractor is derivable by terminal-backward induction, one fixed-successor strong probe per shell child, retained flat shell subtraction, and a separate incoming-source prefix probe when needed. Weighted original-event accounting removes the enormous helper interpolation factors without introducing q times expensive suffix extraction at every depth. The sealed query can similarly use one all-original-sources probe rather than its complete interpolation tree. This is a viable specified conditional proof improvement under the approved classical indexed-ROM/base lattice model; actual implementation/profile admission and nonvacuous concrete-budget pricing are still unestablished. No protocol/code changes or verification commands were performed.
