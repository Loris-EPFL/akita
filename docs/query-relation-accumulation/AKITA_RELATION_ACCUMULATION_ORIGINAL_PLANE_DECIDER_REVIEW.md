# Candidate1 original-plane decider review

## Decision, scope and evidence

**The proposed native raw opening is algebraically valid for the stated narrow capability. It is not an existing ordinary Akita capability.** Preserve each literal original P1/nv16/base32/depth26 RAM A-input tuple, its A/B/F matrices and its original commitment/path. Add a registered original-plane opening view with query arity21. The necessary arithmetic changes are original-plane partial contraction and packing-Z query weights, including the compact verifier's non-geometric digit factorization. Public claims, typed operation authority, planner, catalog and descriptor must distinguish source geometry from view/query geometry.

This report independently proves the native-row extension and its deterministic coordinate-fork extraction, and gives a concrete counterexample to decoded fallback. The extraction result is conditional on the **complete owned descendant extractor and true native rows** described below. It is not a claim that current unmodified code accepts raw proofs or that an unselected accumulation roster has a numerical security certificate. Complete suffix/terminal, incoming scalar-batch cut, original range/query/frontend source cuts, actual mixed-SIS inventory and the approved v8 indexed-ROM experiment remain mandatory.

Read-only source inspection and handwritten algebra only. No git operations, repository edits, tests, builds, lints, formatters, proof executions or runtime checks were performed. Local research prose is context, not mathematical authority. The published Fenzi–Moghaddas–Nguyen paper was directly inspected: [Journal of Cryptology 2024, DOI 10.1007/s00145-024-09511-8](https://doi.org/10.1007/s00145-024-09511-8), [primary repository PDF](https://infoscience.epfl.ch/server/api/core/bitstreams/9abd4c0d-3e31-4228-b600-28f51a7a511f/content), §§8.1–8.2, Lemmas8.2–8.3. Its weighted coordinate-wise sampling analysis supports the retained shell game, not by itself this new raw integration/source-cut composition. The [official Akita ePrint record](https://eprint.iacr.org/2026/1983) was located, but the PDF reader returned only the abstract; no body theorem from that response is used. The algebra below does not rely on unauthenticated `Papers/parsed/Akita.md` theorems.

### Fixed anchor and target

Anchor: ordered original public commitments and semantic source identities; installed setup; exact producer contracts/profiles; original A-input tuples and certified B/F paths; selected source views/decoder adjoints; full opening descriptor and final endpoint roster. A K/K receiving handle/quotient-lift material view may change consumer authority, not these source objects. Current `opaque/ordinary_projection.rs:143–201` authenticates original setup/source identity/profile/payload/material and retains the original Arc. Its validation recomputation is not authority to publish a new source commitment. Its own documentation does not grant arbitrary raw-plane queries.

The user selected **preserve the base proof relation**, not stronger RV64/SDK execution. Keep all Main Booleanity/support/first-last-PC and selected continued frontend/program/advice/transition obligations. Original signed RAM membership must be proved separately. Base word/tape/stopping gaps remain disclosed in `AKITA_RELATION_ACCUMULATION_CONTINUED_FRONTEND_CERTIFICATE.md`.

## 1. Source polynomial, exact axes and decoded-fallback counterexample

Let K be the installed A7F7 prime field, q=2^128-2^32+22537, and R=K[Y]/(Y^128+1). Keep

\[
s_{b,p,\ell}(Y)=\sum_{a<128}s_{b,p,\ell,a}Y^a,
\]

for b<16, p<32, ell<26. Physical storage is `[block][position][source_digit][coefficient]`. Each A block has832=32*26 input ring columns, rank3 and dimension128; there are16 blocks. B has1248=16*3*26 input ring columns, rank1/dimension128/one slice. The profile remains **P1, logical nv16, logical length65536**. Evidence: `sources/packed_ram.rs:20–123,142–169,322–434`; frozen profile/plan in `AKITA_INSTANCE_ACCUMULATION_ORDINARY_CONTROL_LEDGER.txt:2,14–16`.

Define F_C as the multilinear extension of the original digit table with low-first axes

`[coefficient7 | digit5 | position5 | block4]`.

Raw cube index is a+128*(ell+32*(p+32*b)). For ell<26 its physical address is a+128*(ell+26*(p+32*b)); for ell>=26 it is virtual zero, with no physical source cell. The ordinary decoded address is a+128*(p+32*b). Thus raw arity21 and2097152 cube slots do not change the1703936 physical digits or commitment arity16. Plane25 retains the full signed alphabet; no canonical last-plane reach discount is legal. The producer declaration remains full-field bound128 with intrinsic base32/depth26 (`sis/committed_source.rs:130–141,184–206,309–318`). Honest source-plane16 is not a new producer profile bound16.

Main retains56 actual logical16-variable components and source depth1. The inspected original Main split is coefficient7/position9/block0, M512/B1. It keeps its own natural16 points, not RAM's address projection. Honest one-hot storage is not an adversarial Booleanity/support/PC proof.

### Current implementation

`PreparedSubringCoefficientPackingPoint::new`, `akita-types/src/subring_coefficient_packing.rs:35–125`, prepares ordinary `[pack|tail|position|block]`, ties public point length to source arity and has no digit query recipe. At k=h=1 a RAM point is7+5+4=16. `opaque/ordinary_projection.rs:68–85` calls this preparer. `packed_ram.rs:391–434` uses all26 original planes but Horner-decodes each coefficient before position contraction. Source ownership/folding is preserved; arbitrary raw evaluation is not.

`opaque/full_ring.rs:38–105` does raw digit/position contraction, but requires extension degree2, emits256 K coordinates/block and belongs to the separately admitted E2/native full-ring method. It cannot be retagged as the requested K/K packing capability.

### Explicit counterexample

At one live (b,p,a), set t0=9, t2=-10, t3=1, t6=-4, t25=8 and all other digits zero. All are in[-16,15], and

9 -10*32^2 +32^3 -4*32^6 +8*32^25
=22537 -2^32 +2^128=q.

The decoded K coefficient is zero, just as for the zero table. Its raw Boolean digit0 endpoint is9 and digit25 endpoint is8. This is a legal original noncanonical path. It proves that even all decoded openings do not determine the raw endpoint. It does **not** assert equal A/B/F commitments for these different paths; the issue is decoder noninjectivity, not a lattice collision.

## 2. Exact decoder adjoint and one raw source view

For decoded claim v at ordinary address u=(u_A,u_P,u_B), define

B(D)=sum_(ell<26)eq(D,ell)*32^ell,
phi_u(A,D,P,B)=eq(u_A,A)*eq(u_P,P)*eq(u_B,B)*B(D).

Summing phi_u(x)F_C(x) over the cube gives exactly the decoded claim. For a raw claim, phi_u(x)=eq(u_raw,x). Both use the **same F_C and original handle/path**. Evaluate B as26 direct public terms; do not use a denominator-based single-digit-point shortcut. Related existing exact-adjoint patterns are in `full_ring/query.rs:268–326,392–456`; its optional `positional_raw_claim` at462–503 uses inverses and is not this required representation.

Installed affine/permuted decoders need their actual adjoint and authenticated affine constant subtraction. The positional formula does not erase aliases/permutations/basis conventions. Raw uses Boolean-grid/Lagrange Eq semantics. Existing decoded Monomial semantics are separate and remain supported for decoded-only calls.

Give every independent scalar obligation a unique formal Z exponent, including frontend and global original-range leaves. At common query dimension n>=21, Main keeps its natural16-coordinate prefix/cylinder transport, while raw RAM uses its21-coordinate view. Zero support belongs to public query weights, not to invented zero source padding. Each RAM final native point is raw21; each Main component final point is natural16. For RAM the ordinary address projection is `[raw[0..7],raw[12..17],raw[17..21]]`; this is not Main's point. Keep all56 Main endpoints.

For fixed sources, the formal query residual has degree<=m-1 in Z. After its challenge, each query summand is a product of multilinear source and public weight, with individual degree<=2 in X. Direct26-term B preserves this bound. No26-polynomial source reinterpretation is needed.

## 3. Native point rows and exact arithmetic substitution

For raw u=(u_A,u_D,u_P,u_B), write v_p=eq(u_P,p), d_ell=eq(u_D,ell), t_a=eq(u_A,a), beta_b=eq(u_B,b). Prepare the old **address16** point `[u_A,u_P,u_B]`, retaining/authenticating digit point5 separately. Set

\[
E_b(Y)=\sum_{\substack{p<32\\\ell<26}}v_p d_\ell s_{b,p,\ell}(Y).
\]

Then

F_C(u)=sum_(b<16)beta_b*sum_(a<128)t_a*coeff_a(E_b).

E_b is one complete128-coefficient K ring partial. Existing `arithmetic/coefficient_packing_fold.rs:245–334` implements the separate tail/block scalar functional; its arithmetic need not change.

Canonical shell coordinates c_b fold the original source:

Z[p,ell]=sum_b c_b*s[b,p,ell] in R.

That is the existing plane-preserving fold in `packed_ram.rs:142–169,322–379`. The raw packing row is

\[
\sum_{b<16}c_b E_b-\sum_{\substack{p<32\\\ell<26}}v_p d_\ell Z_{p,\ell}=0
\quad\text{in }R.
\]

The quotient-lift polynomial row is

\[
\sum_{b<16}c_b(Y)E_b(Y)-\sum_{\substack{p<32\\\ell<26}}v_p d_\ell Z_{p,\ell}(Y)
=(Y^{128}+1)Q_{\mathrm{pack}}(Y).
\]

Z is negacyclic-reduced. Existing partial-fold multiplication computes the reduced product and its positive high half, the packing quotient (`coefficient_packing_fold.rs:469–550`; `opaque/witness_build/relation_quotient.rs:240–368`). Raw changes the partial and Z contraction, not the product/high-half algorithm.

At alpha, with Zhat response digit index f, the contribution is

eta_consistency*[sum_b c_b(alpha)E_b(alpha)
 -sum_(p,ell,f,a)v_p*d_ell*g_fold[f]*alpha^a*Zhat[p,ell,f,a]
 -(alpha^128+1)sum_(r,a)g_quotient[r]*alpha^a*Rhat[r,a]].

The scalar row separately applies gamma-weighted tail/block coefficients to Ehat recomposed with g_open. Its accepting-branch truth is not obtained by subtracting scalar evaluations.

### Only the query functional changes

`proof/coefficient_packing_relation.rs:487–545` distinguishes opening, quotient, source and response gadgets. The current packing-Z contribution `opaque/sumcheck/relation_range_image/coefficient_packing_terms.rs:291–336` uses v_p*32^ell*g_fold[f]. **Only its source-query factor becomes d_ell.** All26 source indices and all scheduled response digits stay. A columns/source inputs, B/T, Ehat full-field gadget, quotient and response gadgets retain their meanings. Ordinary setup/A/B/D consumers must not receive a globally replaced gadget. Introduce a typed source-opening recipe/weights separate from genuine source recomposition.

### Compact verifier is a separate arithmetic gate

`akita-verifier/src/coefficient_packing_relation/compact.rs:282–315,638–715` uses geometric source-digit axes. `geometric_axis` constructs `[1,weights[2^bit]]` factors and validates every coordinate, forcing weight0=1. Raw d0 generally is not1 and can be0. Blind substitution rejects valid raw points; removing validation computes the wrong polynomial. Dividing by d0 both changes the permitted domain and adds a forbidden inverse.

Use the existing physical26-digit dyadic cover without normalization:

| Live ell segment | Fixed-high-bit scalar h | Low-bit tensor factors |
|---|---|---|
| [0,16) | 1-u_D[4] | `[1-u_D[i],u_D[i]]`, i=0..3 |
| [16,24) | u_D[4]*(1-u_D[3]) | same pairs, i=0..2 |
| [24,26) | u_D[4]*u_D[3]*(1-u_D[2])*(1-u_D[1]) | pair i=0 |

For segment start t/length2^r, use `EqPairTensorAxis::bit_product(0,delta_fold*128,low_r_pairs)` and multiply the family scalar by h. Keep physical offset t*delta_fold*128 and position stride26*delta_fold*128, not32*delta_fold*128. Keep response geometric axis, alpha/coefficient128 axis and position32 axis. Do not use eq(u_D,t) as h: it would double-count low zero-bit factors. `offset_eq/tensor_pair/mod.rs:54–97` already supports arbitrary zero/one bit factors, including zero values. This preserves three source-digit segments. A26-public-weight dense axis is also exact, but means only a public functional vector, never a Dense source or26 polynomials; the factored recipe is the narrow preferred implementation.

## 4. All physical widths, gadgets and complete shared D/H

General packing has d_A=k*h*s and partial physical width k*s. D-native subcolumns are (k*s)/d_D, requiring exact divisibility. Initial raw sets k=h=1 and d_A=s=d_D=128. Keep delta_source, delta_open, delta_fold and delta_quotient distinct even when several equal26.

| Object | Initial original P1 RAM extent |
|---|---:|
| Logical source/profile | 65536 coefficients, nv16, P1 |
| Literal original source digits | 16*32*26*128=1703936 cells |
| Raw query cube | 128*32*32*16=2097152 virtual slots, nv21 |
| E_b | 128 K coefficients |
| All unbatched E_b | 16*128=2048 K coefficients |
| A width per original block | 32*26=832 ring columns |
| A image tuple | 16*3*128=6144 K coefficients |
| B input width | 16*3*26=1248 ring columns |
| B image | rank1*128=128 K coefficients |
| Ehat/D input width | 16*delta_open;416 ring columns at26 |
| Ehat cells | 2048*delta_open;53248 at26 |
| Raw response per chunk | 32*26*128=106496 physical coefficients |
| Zhat per chunk | 106496*delta_fold;319488 at3 |
| T/B digits over all owned blocks | 16*3*26*128=159744 cells |
| Packing quotient | 128 K coefficients |
| Packing quotient digits | 128*delta_quotient;3328 at26 |

Authorities: `layout/subring_packing_geometry.rs:8–81,113–117`; `layout/params/precommitted.rs:123–158`; `witness.rs:212–255,318–359,907–937`; `witness_build/coefficient_packing.rs:102–259`. With chunks, every Z chunk retains its full M*26*delta_fold*128 span; E/T blocks follow authenticated ownership partitions. Initial frozen planner supports the specified single chunk, not arbitrary chunk reinterpretation.

Shared D native input width is

sum_g P_g*B_g*delta_open,g*(partial_width_g/d_D).

For the observed Main56/M512/B1 plus two RAM roster, full-field open depth26 gives1456+416+416=2288 D columns. This is an observed illustrative roster, not a future accumulation row. Shared rank n_D is receiving-row-owned and cannot be guessed from P1/nv16; the full D image has N_D=n_D*d_D coefficients.

Raw public Eq weights may make E coefficients arbitrary K values even when source digits are signed-small. Require full-field Ehat coverage: K128/base32/depth26. The materializer uses the scheduled depth; raw admission must verify coverage rather than assume it. No depth1, short-source or small-public-weight shortcut.

H compresses the **complete D image**. At q128 the dimensions are16 then8 (`compression.rs:226–237,286–295`; `compression/chain.rs:154–188`): H0 has128*N_D real negative-binary cells and native width ceil(128*N_D/16), full padded16*width span,16 output coefficients; H1 has2048 input cells/width256/dimension8 and8 outputs. Its quotient digits occupy delta_quotient*(16+8),624 at26. The transmitted opening payload is8 K values/128 bytes, not a smaller private path.

Original B/F likewise consumes the complete128-coefficient B image: F0 has16384 binary cells/width1024/dimension16; F1 has2048/width256/dimension8; final payload8 K values. Original F matrices/material remain source-owned. No query-specific source commitment replaces them. Every real F/H padded matrix cell retains its binary/matrix constraints. Recursive GRID/LOCAL range covers actual full W, including real in-range alignment/suffix padding where semantic weights vanish; it does not prove all such cells zero. Six virtual raw digit slots are a different decoder fact.

### E payload cut precedes gamma and shell

Prover `ring_relation.rs:220–260` obtains complete backend Ehat/D/H material, checks scheduled payload geometry and sends OpeningPayload. It deliberately batches claims only at264–283; shell/source folds are later at307–340. Backend `witness_build/assembly.rs:75–128,471–528` constructs every group's full Ehat and D/H path. Verifier `fold/root.rs:54–92` binds original point/scalars, receives payload, then finalizes claims; `stages/opening_claims/mod.rs:59–63` owns EvaluationBatch. The same canonical shell drives source and partial folds.

E_b is fixed through D/H rather than necessarily transmitted as plaintext. Proving shared D/H explanations equal across shell siblings remains necessary; message order alone is not binding. Do not introduce early scalarization or gamma-dependent E preparation.

## 5. Raw extraction extension lemma and unit cancellation

### Premises

For one original RAM root, take a marked original accepting continuation and a star with one distinct changed ring coordinate for each of its16 blocks. Original source commitment/profile/identity, view/point and complete opening payload are fixed before the shell. Require:

1. Every child's complete suffix/actual terminal extractor owns all outgoing native point/value leaves and any separately selected setup source.
2. The full current strong relation is true on its causally fixed complete outgoing source: all A/B/D/F/H, quotients, response ranges, binary, scalar and selected norm/setup rows.
3. Original B/F and shared opening D/H sibling paths coalesce, or a correctly priced named matrix collision is returned.
4. Shell differences compare actual ring polynomials, not ordered sampler representations; nonzero differences are admitted units in the carrier/embedding.

Neither honest constructors nor current source shortness substitute for these premises.

### Reconstruction

Let branch0 be original and branch b change only shell coordinate b. Set

rho_b=c_b^(b)-c_b^(0),
r_b[p,ell]=Z^(b)[p,ell]-Z^(0)[p,ell],
s_b[p,ell]=rho_b^(-1)*r_b[p,ell].

Subtraction of true A/response rows and common original B/F path gives A*s_b=t_b. Each block has its **own rho_b**, not one unit certificate for the entire source. Assembling16 blocks of832 rings yields the literal original-layout A-input explanation and B/F path.

Packing-row subtraction gives

rho_b*E_b=sum_(p,ell<26)v_p*d_ell*r_b[p,ell].

L_w(s)=sum_(p,ell)v_p*d_ell*s[p,ell] is R-linear because weights lie in K. Thus rho_b*E_b=rho_b*L_w(s_b), and legitimate unit cancellation gives E_b=L_w(s_b). Only now apply the coefficient-tail K-linear functional and block weights. The accepting branch's scalar row yields the gamma-weighted original endpoint; Section6 upgrades nontrivial scalar batches.

This is an explanation of the **original raw endpoint under C**, not a decoded replacement source. Public query weights do not enter r_b's response-difference certificate. No inverse-norm bound is used. Original signed range truth comes later from its independently owned source-range relation.

The tail functional is not R-linear: at coefficient0, lambda(Y)=lambda(Y^127)=0, but lambda(Y*Y^127)=-1. An actual allowed shell sign change may give rho=2Y: lambda(rho*1)=0 although lambda(1)=1. Scalarizing before ring cancellation destroys the needed equation.

### Independent unit argument at native128

q mod256=9 and ord_256(9)=32: 9^16=129 mod256 and9^32=1 mod256. Consequently each irreducible factor of Phi256(Y)=Y^128+1 over K has degree32. For nonzero integer h of degree<128, the integer resultant Res(Phi256,h) is nonzero, since Phi256 is irreducible over Q, and the complex-root product bounds its magnitude by ||h||_1^128.

If h is a nonunit modulo q, multiplication by h on the128-dimensional quotient has kernel dimension at least32. Its integer multiplication matrix therefore has determinant divisible by q^32 (Smith normal form), and that determinant is the resultant up to sign. This contradicts 0<|Res|<q^32 whenever ||h||_1^4<q.

Ordinary native128 shell omega31 gives difference mass<=62, with62^4=14776336<q. Even the separate compact/native omega56 shell has difference mass<=112 and112^4=157351936<q. Distinct such ring-polynomial differences are units. Equal polynomials give rho=0 and are excluded/charged by the actual shell game. This proves invertibility only, not short inverses. Other carriers/embeddings/filtered distributions retain their own real unit and sampler arguments.

## 6. Incoming scalar batching is a distinct source cut

`akita-types/src/opening_claims.rs:258–273` uses coefficient1 for a singleton; otherwise it draws **all coefficients independently, unnormalized**. They are neither powers nor a vector with forced first coefficient1. Verifier `stages/opening_semantics.rs:53–77` checks the gamma-weighted supplied openings, and Stage2 authenticates that target.

Without a prefix upgrade, the raw shell lemma establishes only sum_j gamma_j*(v_j-true_endpoint_j)=0. For every individual endpoint, use an incoming-source cut after original claims/view/point and full D/H payload, before EvaluationBatch. Its matched header retains the original point/payload; source-only comparisons coalesce incoming A/B/F while ignoring opening-local D/H across separate probes. Inside exact shell siblings, D/H must agree to cancel E terms. These are distinct comparisons.

For common fixed sources, the true original endpoints and discrepancy vector are determined before gamma. If that vector is nonzero, after fixing the other coefficients a nonzero coordinate leaves at most one bad gamma value. Fixed-source error is<=1/q for completed uniform coefficients; the existing conservative k_claim/q planner charge is also valid. Singleton needs no batch cut/error. Zero coefficients are allowed and accounted for, never inverted or rejected.

The complete source probe supplies actual product/path certificates. Compare its source explanation to the marked original continuation or return the priced collision. Check **all original unbatched point/value equations** on that common source. Keep the original external transcript/output; do not substitute a probe's different endpoint. This incoming-source cut is not the successor Cf strong cut or sealed outer query cut. Its cost generally adds a multiplicative source-probe factor. With one full strong-source probe and one incoming-prefix probe, the conservative level coefficient is4*b_level, b_level=1+sum_g P_g B_g. A P1 RAM contributes16 coordinates, not26,416 or shell support31. The saved Main56 plus two RAM roster has88 coordinates/global star89; scalar endpoint count58 is different.

## 7. Weak product certificates and actual mixed-SIS inventory

For response base b, depth delta_f, chunks m, certified alphabet[-a/2,a/2-1], shell mass omega and S=sum_(f<delta_f)b^f:

accepted interval=[-m*(a/2)*S,m*(a/2-1)*S],
Z_abs<=m*(a/2)*S,
beta_product<=m*(a-1)*S,
kappa_difference<=2*omega.

The certificate is rho_b*s_b=r_b with ||r_b||inf<=beta, **not** ||s_b||inf<=beta. Honest original plane16 and decoded K reach do not bound adversarial extracted sources. `sis/norm_bound.rs:48–85,182–235` uses response differences and audited rounding. Source depth26 does not determine response depth.

The inspected ordinary128 saved row has a=b=32, m1, omega31: RAM response depth3 gives[-16912,15855], beta32767/kappa62 and diagonal A radius4063108; Main depth2 gives[-528,495], beta1023/kappa62 and diagonal126852. Frozen original A capacity8126340 is a profile admission anchor, not beta. These are static deductions for that control, not new accumulation-row parameters.

For two same-original-source block explanations with common A image t and certificates (rho,r), (rho',r'), a difference yields

v=rho'*r-rho*r'=rho*rho'*(s-s'),
A*v=0,
v!=0,
||v||inf<=kappa'*beta+kappa*beta'.

Nonzeroness uses both units. Euclidean alternatives use ||v||2<=Kop'*B2+Kop*B2' with real checked difference/operator bounds and complete physical vectors, then safe squared rounding. Never use honest source energy or inverse norms.

Coalesce fixed terminal paths backward: F/H negative-binary input differences have coefficient diameter1 over all actual padded matrix cells; B/D balanced base32 digits have diameter31 over complete gadget widths. Mixed alphabets need the actual cross-interval diameter. D/H is opening-local except for same-payload root shell siblings. Every instance keeps actual native dimensions/rank/width and shared installed setup namespace. A128-byte terminal commitment does not shrink the source/SIS width to8 coefficients.

If a selected suffix offloads setup, the canonical installed reference uses rho_ref=1, r_ref=s_ref for its actual full A-input decomposition. Mixed A price is beta_weak+kappa_weak*||s_ref||inf, plus actual reference B/F alphabets. Installed prefix beyond natural length is an actual source with zero query weights, not fictitious zero entries. Selected L2/setup methods remain mandatory; direct setup/unfiltered Linf are admissible cost choices, not permission to discard another selected row's constraints.

Terminal supplies its actual direct response bound and A-only certificate, no invented terminal successor range. The saved original ordinary terminal cap16383 gives beta32766/kappa62/diagonal4062984, not32767 inferred from fold digit metadata. Broader rows retain checked L2/operator/no-wrap conditions.

## 8. Complete original-output order and finite error/cost contract

1. Execute real continued frontends to pre-PCS boundary, retaining every selected Main/support/PC/program/advice/transition obligation. No per-segment completed PCS prerequisite.
2. Freeze original source roster/identity/decoders; prove global original signed-plane reduction before query-index challenge. Append **every raw terminal leaf before seal**. A virtual range-image scalar is not a source opening.
3. Seal source/view/decoder context, unique scalar exponents and full endpoint roster.
4. Run weighted query over original polynomials, with no new source commitment.
5. Run registered raw/decoded root, complete shrinking ordinary suffix, actual terminal and transcript/grinding EOF completion. Root Stage2 alone is not acceptance.
6. Independently replay frontend/history/range/query/full decider against external expected statement/count/adjacency.

Extract backward: actual terminal; complete descendant source/point maps; strong Cf cut using **its deeper scan's** product certificate; root shell plus incoming batch cut; original A/B/F coalescence for sealed query and separately owned original-range/frontend cuts. Cf can differ between shell children. Never coalesce different handles or profile-equal sources as if identical. Original aliases/repeated references require exact identity/projection and priced comparison.

The outer fixed-source query charge is (m-1)+2n. Incoming independent scalar batch charge is0 singleton or at most1 nontrivial. A safe source-cut loss is

Q_c*epsilon_c+binom(Q_c,2)*gamma_c,

with actual original-query horizon Q_c and causally fresh maximum atom gamma_c. Proper descendant keys must parse/fix the completed ancestor result and owner. v8 binds complete canonical view/point/roster history; the old seed/chained interface cannot be selected silently. Completed uniform scalar atom is1/q. A complete gamma tuple permits a smaller atom only when the actual parsed grammar fixes that whole tuple;1/q is conservative. Shell terms use the actual accepted polynomial support/full finite conditional raw sampler, not independent draws conditioned on a winning response nonce.

Matched probes restart from genuine saved prefixes with fresh future entries. Do not keep original future answers to force matches, condition a child bound on a rare matched acceptance, or omit failed continuation work. Original marked record/raw answers/leaves remain owned. Published weighted coordinate sampling controls the retained star; source-cut composition must explicitly preserve this marked-original law.

Raw Eq weights are constants in Y, so packing alpha degree remains<=255; no higher native polynomial degree or five automatic extra root sumcheck rounds follows from point21. External weighted-query dimension/cost, context bytes and actual new range/helper/query sites must be registered/priced. Existing LOCAL proves actual full W constraints, not all-padding-zero or individual unbatched claims. Global range/helper first-divergence/error ledger is a separately required §3 review, not replaced here by off-cube U=G(G+1).

With strict total charged extraction budget L, the conditional final bound has the form

Pr[original accepts without an original base-relation witness]
<= statistical shell/source-cut/frontend/range/query terms
 +E[charged_work]/L+setup/sampler/identity deviations
 +sum_I Adv_ModuleSIS_I(T_I(L),eta_I).

T_I(L) prices setup namespace simulation, original/private replay, finite raw/conditional sampling, complete source decoding/comparison, helper checks and collision/output construction. `FoldSchedule::sis_occurrences` enumerates installed occurrences but not all mixed comparison pairs. No assertion Adv(T)=T/2^128, compulsory end-to-end128-bit gate, concrete SHAKE knowledge or QROM claim is introduced. An infeasible/large bound does not authorize source-profile changes.

## 9. Exact narrow Rust interface [design proposal, not existing API]

Prefer a **nonparametric registered view** under existing `OpeningMethod::SubringCoefficientPacking { challenge_subring_dimension:128 }`: `SourceOpeningView::Decoded` versus `SourceOpeningView::OriginalRamPlanesV1`. The raw variant fixes P1/nv16/N512/M32/B16/depth26/base32, axes7/5/5/4, virtual slots26…31, identity or explicitly implemented installed injection, and Lagrange semantics. Unsupported shapes require an error or a separately reviewed future view identity, not free caller parameters.

Keep `GroupCommitPhaseParams`, its VERSION4 producer bytes, `PolynomialGroupLayout`, committed-group profile/payload and source metadata unchanged. Use opening-only `OpeningGroupLayout { source, view }`, with explicit source geometry versus point arity accessors. Add view to public claim groups and consuming `GroupOpeningPlan`; bind it canonically. Existing decoded constructors intentionally construct Decoded; this is not a fallback after a raw failure.

A prepared raw point retains the old address16 geometry from `[coeff7,pos5,block4]`, digit point5/derived26 Eq weights, raw query arity21, view identity and original source operation binding. The verifier derives weights from the canonical recipe/point; a free prover weight array is not raw authority. Use a fixed small array or factored bit recipe, not a2^21 source table. Carry the view through all claim-copying, selected-row, plan, opaque handle and semantic validation boundaries. Geometry equality alone is not view equality.

The current root API supplies one external BasisMode. Narrow rule: **a batch containing raw rejects Monomial before transcript mutation**; decoded-only batches retain both old conventions. Future per-group basis support would need its own descriptor contract, not an implicit override.

### A. Public statement, layout, root and preamble

1. `akita-types/src/opening_claims.rs:13–60,111–147,172–188,239–249`: `PolynomialGroupClaims` gains view/point arity; explicit raw constructor requires21 coordinates and one evaluation. Both borrowed/owned `committed_layout` validate source16 against the view. Generic layout cannot derive nv21 from point.len(). `GroupBatchStatement` preserves views; structural checks do not replace full admission. All `map_groups`/copy conversions preserve view.
2. `akita-params/src/layout/opening_layout.rs:9–159,164–232`: store opening-only source/view layout. Add group_view/group_point_num_vars/max_point_num_vars; source shape/count/claim range retain original meanings. Digest now binds canonical view/point dimensions/order, not only arity/count.
3. `akita-prover/src/opening.rs:17–87,109–132,152–208`: `SelectedProverOpeningData` resolves profiles **plus views**, aligns handles with source16 and public points with view21, preserves view when rebuilding claims, binds complete raw point and scalar values. `from_parts` raw needs exact21; decoded suffix short-point semantics remain separate.
4. `akita-prover/src/protocol/prove/root.rs:34–100`: exact view-aware trusted-row/setup/handle/basis admission before channel mutation; root stays no-EOR packing. `akita-verifier/src/fold/verify.rs:35–141` mirrors admission and preserves view in reconstructed claims. `fold/root.rs:29–105` preserves original commitment/point/value/payload order and packing root requirement.
5. `layout/params/precommitted.rs:162–210,399–407`: consuming `GroupOpeningPlan` includes view descriptor. `layout/params/builder.rs:65–71` supplies Decoded. **Own/final group is manually encoded** in `layout/params/descriptor.rs:100–151`, so add its view there too, not only preceding groups. `schedule/descriptor.rs:69–85,114–170` and `schedule_selection.rs:52–77` bind complete view-aware schedule/row identity. Do not bump producer profile bytes to impersonate an opening change.
6. `akita-types/src/instance_descriptor/mod.rs:216–259,634–740`: `CallSection` separates source and point dimensions/view and binds raw Lagrange semantics; canonical serialization/validation/digest migrates. `descriptor_bytes.rs:8` currently names instance version5. Deliberately update incompatible instance/schema/schedule-row domain epochs and reject old/missing/unknown view tags in the new grammar; no serde default that accepts an unspecified raw meaning. Config transcript binding and grinding binding consume the new canonical descriptor.

### B. Original owner, point and opaque witness boundaries

7. `prove/fold/mod.rs:87–238` and `backend/plans/mod.rs:307–365`: `ValidatedRecursiveGroupOpeningPlan` carries approved view and public query point; source logical_len remains2^16. Raw cannot enter EOR/short-point paths. Retain scalar-opening equality/binding checks.
8. `opaque/openings.rs:51–87` currently rejects point21 against metadata16; replace this coupling with separately validated source/query authority. Check view against admitted group/capability before dispatch. `opaque/admission.rs:66–115,138–249` checks owner/profile/payload/scope/order plus raw original class/layout/basis. `opaque/backend.rs:353` and lifecycle scope layout retain views.
9. `opaque/ordinary_projection.rs:16–86,108–135` explicitly dispatches Decoded versus raw over the retained original owner. A generic native FullRingOwned is not itself proof of the registered original-RAM producer. Projected handles expose no `full_ring()` facade; inspect their retained original native capability instead of treating that facade absence as source loss. Keep source Arc/setup/sourceID/profile/public payload/original retained state/receiving quotient material.
10. `opaque/owned.rs:86–110,174–239` generic legacy/Dense dispatch explicitly rejects original-RAM raw; do not ignore the recipe or fall through to the current decoded default. Dense/onehot kernel branches remain Decoded.
11. `sources/packed_ram.rs:260–279,291–316,338–433`: keep original digit visitor/fold and decoded Horner branch. Raw partial uses exact original digits weighted by Eq(u_P,p)*Eq(u_D,ell), ell<26, then existing tail/block scalar. No decoded SourceCoefficients, redecomposition, new commitment or26 Dense sources.
12. `subring_coefficient_packing.rs:39–119`, `opaque/operation_plans.rs:90–103`, `arithmetic/coefficient_packing.rs` packing plan/`FusedPackingWeights`: check source16 and public21 separately; retained address preparation is unchanged. Raw point projection must be explicit, never silent truncation.
13. `opaque/prepared_opening.rs:29–49`, `backend/metadata.rs`, `opaque/recursive/witness/handles_and_relation.rs:181–276`, `witness_build/prepared_openings.rs:85–107,188–211`, `assembly.rs:96–100`: preserve point/view/recipe/arities and compare against admitted source/group/method. Still use the coefficient-packing carrier, not FullRingScalarExtension.
14. `protocol/ring_relation.rs:52–109,232–400`: validate complete prepared view, keep full E-derived D/H before gamma/shell and exact M*26*d_A response coordinates. `witness_build/coefficient_packing.rs:19–57,102–259`, `assembly.rs:75–128,471–528`, `relation_quotient.rs:265–368,532–590`, `finalize.rs:381–459,749–766`: full widths/fold/high-half/materialization remain, with authority propagation only.
15. `verifier/stages/opening_claims/coefficient_packing.rs:15–87,94–132` replays raw preparation from public view/row and leaves suffix decoded. `opening_claims/mod.rs:43–95` keeps all scalar batching after payload. `fold/relation_instance.rs:113–133` point/challenge join must retain view authority.

### C. Shared relation and real consuming compiler

16. `types/proof/coefficient_packing_relation.rs:390–405,506–545,565–748`; `expanded.rs:157–175,311–314`: add a validated source-opening recipe distinct from actual gadgets. Preserve source depth26 and E/open, quotient, fold gadgets. The current packing-semantic `witness_gadget` query use should have an unambiguous name; ordinary A/source gadget carriers keep genuine semantics.
17. `opaque/sumcheck/relation_range_image/coefficient_packing_terms.rs:81–84,288–336`: consume raw query factor only in packing-Z. `opaque/recursive/witness/stage_sessions.rs:407–413`, `prover/protocol/ring_switch.rs:172–183`, `backend/plans/relation.rs:79–84`, `opaque/relation_weights/stage2.rs:205–240`, `verifier/stages/relation_claim.rs:70–84` preserve shared validated semantics, not a second unchecked recipe.
18. `verifier/coefficient_packing_relation/mod.rs:21–49`, `compact.rs:638–745`: explicit decoded geometric versus raw Eq factorization from Section3. All E/direct-opening/response/quotient factors stay. `stages/stage2.rs:45–103,263–309` consumes complete compact/ordinary/compression factors at the actual owned endpoint. An unhandled raw factor must reject, never vanish.
19. Ordinary `relation_weights/compiler.rs:449–513`, `sinks.rs:130–159`, setup/A/B/D compilers, `verifier/relation/evaluation.rs:391–394` and compression relation compiler keep source/setup meanings and full matrix spans. Ordinary evaluation skips packing consistency because its separate packing consumer handles it; ensure the new raw arm reaches that consumer. Do not globally substitute Eq weights in setup matrices.

### D. Explicit decoded suffix/setup/terminal constructions

20. `prove/suffix.rs:140–149,327–355`, `verifier/fold/suffix.rs:116–145`, `params/layout/setup_prefix_helpers.rs:34–39`, `params.rs:359–369`, `setup_prefix_slots.rs:571–578`, `opaque/recursive/witness/recursive_kernels.rs:149–164` explicitly create Decoded successor/setup claims/plans. Do not propagate root raw21 or a batch-global raw flag into canonical Cf. Existing admitted EOR/tensor paths remain available for their decoded sources.
21. `schedule.rs:760–850,888–1020` enforces root-only original raw capability alongside existing method/source encoding admission. Recursive/setup/terminal raw occurrences reject for this initial view. `RingRelationInstance`, family/challenge carriers remain packing typed; their embedded opening layout retains the view. `fold/terminal.rs` and direct terminal remain actual canonical recursive semantics and direct bounds.
22. `sources/dense/kernels.rs:221–240`, `sources/onehot/ops.rs:467–483`, `opaque/recursive/witness/coefficient_packing.rs:12–72` explicitly preserve decoded functionality and reject raw-RAM-only recipes. Main retains56 real columns. Existing E2 raw helpers remain a separate protocol, not ordinary raw support.

## 10. Planner, trusted catalog, setup and grammar gates

### View-aware lookup is mandatory

`ScheduleLookupKey` currently identifies producer shapes/profiles, while catalog uniqueness/`resolve_profiles` is producer-key-only (`schedules/artifact.rs:165–190,388–420,730–734`). `validate_opening_layout` derives solely from profiles (`schedules/resolve.rs:69–78`). Two decoded/raw rows over identical literal producers cannot be selected unambiguously merely by changing expanded row bytes: duplicate producer lookup keys are rejected.

Add a canonical ordered view vector to the opening lookup key, or a checked opening-key wrapper around unchanged producer fields. Use precommitted transcript order then final, with exact checked group count. Every runtime/planner/catalog cache/hash/order/key includes that vector. Replace/migrate producer-only resolution for opening selection; decoded callers explicitly use Decoded keys. Exact-digest `resolve_selection` and external trusted ownership remain unchanged in trust level. A proof-supplied planner search or view digest is not an installed approved row.

### Exact planning/admission surfaces

- `planner/planner.rs:791–937` frozen-profile entry already enforces K/K/Q128/current tables/single chunks/original contracts and preserves A/B material. Add requested views through root candidates (`515` onward), materialization (`715`, `1310` onward), `schedule_params/suffix_dp/candidates.rs:629–632`, and row admission. Recursive candidates are Decoded. Do not retry decoded after raw planning failure.
- `planner/emit/mod.rs:83–125`, `emit/materialize.rs:19–42`, generation preplans/families, render/gen_schedule_artifacts key reconstruction: generation request, deduplication/conflict checks and caches retain views; standalone producer preplans remain commitment-only.
- Consuming plan constructors: `params/builder.rs:65`, `params/precommitted.rs:399`, `planner.rs:715`, `schedule_params/candidate/recursive.rs:364`, `candidate/setup_prefix.rs:162` supply explicit view. Trace convenience and recursive/setup constructors use Decoded.
- `schedules/resolve.rs:33–45,69–78,95–105` derives authorized row opening layout from consuming views plus immutable profiles. `audit.rs:525–603` checks raw capability/shape including own/final group. Preserve frozen-group full-field Ehat/SIS audit (`149–198`) and original full-plane source norms (`sis/committed_source.rs:183–207,309–318`). Public profiles alone do not record every precommitted producer class; offline contracts and retained-handle admission check actual producer capability. Metadata is not a proof of signed membership.
- `artifact.rs` serialization/reload/catalog digest includes typed views and new semantic row identity. Preserve source profile VERSION4 identity. Artifact/instance protocol epochs must agree; old/missing view tags reject in new raw grammar.
- `config/lib.rs:219–291` and `config/setup_requirements.rs:118–140` currently reconstruct opening layout from profiles. Use the approved row opening layout and distinguish source capacity16 from supported point capacity21. For the narrow change, require setup.max_num_vars>=21 for raw admission on both sides, **without resizing source matrices to2^21**. A future explicit split of source/opening capacity is another full interface migration, not a one-sided exception.
- `schedules/runtime.rs:597–640,650–748,768–787`, `params/transcript_grinding/plan.rs:84–157,226–277,395–448`, `config/transcript_grinding_plan.rs:8–27`, `config/transcript_binding.rs:38–75`, `schedule/sizing.rs` and descriptor materialization use view-aware root layout; successor layouts remain Decoded.

All21 raw coordinates are public transcript atoms (`transcript/proof_stream.rs:867–889`), not automatically five extra proof elements/rounds. Source/witness/relation sizing remains its actual geometry:128 E coefficients/block, original26 source planes, exact response digit depths. EvaluationBatch count is all scalar endpoints; shell count is sum_g P_g B_g. Alpha loss is native-method degree; tau0/tau1/range/sumcheck lengths are complete relation/successor geometry. Deterministic Eq weights do not create a random-challenge stage. Recompute metadata/estimates from the actual new row rather than copy decoded digests.

v8 request grammar binds sealed frontend/query context, original view/point/roster and all actual helper/query/root sites, including operation/stage/fold/round/group/claim/block/limb ownership. Full raw answers/conditional sampling/completed-result history retain approved indexed-ROM semantics. No seed/chained fallback.

### Caller/schema migration inventory

Production and named direct callers of profile resolution: `prover/opening.rs:34`; catalog implementation `schedules/artifact.rs:388`; PCS `benches/akita_e2e.rs:155,386`, `examples/accumulation_pcs_control.rs:117`, `examples/profile/workload/batched_onehot.rs:116,232`, `single_group.rs:102,220`; native Jolt `jolt-akita/src/native_batching.rs:329–333,396–398,558–559,735–736` uses selected data/group statements and must preserve its decoded convention. New final accumulation raw claims are explicit, not a global adapter change.

Known migration fixtures/callers, not checks requested here: `config/tests/committed_b_scanning_bound.rs:161`, `trusted_schedule_artifact.rs:351`; `cpu-backend/tests/original_plane_control.rs:169,346`; PCS `src/scheme/tests/mod.rs:136`, `tests/protocol_soundness.rs:48`, `tests/common/mod.rs:167,194`; `planner/full_ring_inner_tests.rs:537`; `prover/tests/orchestration_dim.rs:30`; `jolt-prover/tests/akita_accumulation.rs:127` and its present decoded control claims at768–788/support `compact_segments.rs:111–127`; catalog-ownership spec line376.

Runtime lookup constructors: `config/full_ring.rs:97`, `config/lib.rs:290`, `cpu-backend/commitment/api.rs:458,524`, `planner/planner.rs:915`, `planner/emit/mod.rs:102`, `emit/materialize.rs:26`, `schedules/artifact.rs:731`. Commitment API catalog-guided choice may request an intended consuming key; it must not turn view into producer metadata. Other key builders requiring static migration include PCS profile multi_group/proof_size, benchmark/test common proof_size/recursive/support/matrix_drivers and distributed fixtures; planner examples/adapted_schedule/planner_totality/schedule_params_candidate/full_ring_inner fixtures; `types/schedule_tests.rs`; Jolt `schedule_registry.rs:465,530`, `check_grouped_schedules.rs:144` and grouped schedule fixtures. Existing explicit decoded/single constructors may remain decoded; none infers raw from point length or failed lookup.

Implementation-time documentation/schema updates include Akita book usage commitment-api/proof-artifacts/integrations/verifier-only/quickstart, architecture, external-schedule-catalog-ownership spec and original-plane design/status. Parent owns root protocol amendment. This read-only investigation does not update repository docs or run listed tests.

## 11. Rejection rules and normal capability restrictions

Reject before transcript/private proof work wherever applicable:

1. Unknown/missing/incompatible view; view-vector group count/order mismatch; raw claim selecting a decoded row or vice versa.
2. Raw Monomial/tensor/EOR, unsupported permutation/injection or wrong axis convention. Exact21 length alone cannot discover caller intent; the API exposes one fixed registered low-first grammar and callers transport into it.
3. Raw producer other than original P1/nv16/N512/M32/B16/d128/base32/depth26 with original approved A/B/F profile/contract/capability. Canonical Dense with a similar numerical profile cannot acquire original-plane authority.
4. E degree!=1, carrier!=128, h!=1, nonnative packing, unsupported slicing/chunks or raw recursive/setup/terminal occurrence. Preserve independently supported decoded/E2 variants.
5. Owner/setup/sourceID/raw-layout/profile/public payload/retained path/operation lineage mismatch. Equal public profiles are not equal sources or capabilities.
6. Nonexact raw point21/evaluation count1. Main remains its own natural16/all56 components. Source metadata16 is not a cap on separately authorized query point21.
7. Truncated source/E/D/H/path/quotient widths, omitted plane25, canonical-last-plane norm discount, or virtual digit slots represented as real A columns.
8. Infeasible original A/B capacity/shared D/setup/suffix/runtime row: explicit UnsupportedSchedule/capability error, no fake profiles, profile-changing replan, decoded fallback or reduced obligation roster.
9. Do **not** reject legitimate zero Eq weights or add inverses/nonzero factors. Virtual26…31 zero slots are decoder-defined, not new physical zero predicates.

These are ordinary capability/admission restrictions, not a weakening of the target. Unsupported stronger claims are decoded-only raw authentication, source shortness from unit division, all-real-padding-zero, unrestricted SDK byte/tape/stop execution, concrete-hash knowledge/QROM, or universal numerical128-bit security without the actual finite row/horizon/work inventory.

## Final handoff

The raw row and fork lemma preserve the original target and are usable after the explicit interfaces above land. The current implementation still lacks the registered raw21 view. The concrete q-path counterexample prevents any decoded fallback. The most easily missed integration gates are source/view arity separation, producer-only catalog duplicate keys, own/final descriptor encoding, claim-copying view loss, Monomial/raw basis mismatch, compact Eq-versus-geometric factorization, setup point capacity and root-only propagation.

Implementation owner must wire public APIs, typed planner/catalog, source partials, shared semantics and real verifier consumption end to end, then independently check raw row/compact equality, all26 planes including25, malformed view/axis/depth/arity rejection, decoded Dense/Main/canonical suffix preservation, full D/H tamper rejection and complete real root/suffix/terminal proofs. These are future implementation-review criteria, **not exercised verification**. Original global range/helper linkage/finite ledger remains separately owned; parent owns protocol/docs integration. No test was run or requested during this assignment.
