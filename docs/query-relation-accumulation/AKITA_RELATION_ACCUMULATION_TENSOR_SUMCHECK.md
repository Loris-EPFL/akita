# Akita relation accumulation: Neo, SuperNeo and SALSAA

## Scope and bottom line

This is research, not a theorem admitting the existing native prototype and not an implementation change. It studies accumulation **before** complete per-segment Akita PCS proofs, with one independently checked final proof/certificate. No builds, tests, benchmarks, branches or repository source files were changed.

**Most reusable mechanism:** SALSAA's norm checkpoint + sumcheck batching of constraint rows, with a fixed-shape bounded linear accumulator. **Most useful field frontend mechanism:** SuperNeo's basis-aware constant-term inner-product transform and common-point CCS sumcheck. Neither mechanism permits treating Akita's signed source coefficients, decoder or multi-level paths as an already compatible CCS/linear witness. Neither supplies a theorem for the existing H1610/full-answer sampler.

A certified native Akita source is not short merely because a small challenge difference times that source is short. All three papers distinguish relaxed extraction from actual norm checks. Directly composing their theorems with the existing weak Akita extractor would skip the central obligation.

## 1. Sources, actual titles, versions and coverage

Primary collection searches used lexical QMD queries with reranking disabled. QMD get was used on the three prescribed document IDs; large base64-bearing responses and line truncation required reading the corresponding indexed local markdown in bounded, wrapped ranges. Local hashes confirm that these are the indexed snapshots (their first six hex digits equal the QMD IDs).

| QMD document | Actual title | Public primary record | Coverage |
|---|---|---|---|
| `neo.md`, `#f5c513` | **Neo: Lattice-based folding scheme for CCS over small fields and pay-per-bit commitments** — Wilson Nguyen, Srinath Setty | [ePrint 2025/294](https://eprint.iacr.org/2025/294) | All indexed lines 1–1821, including references, appendices A/B and parameter scripts |
| `superneo.md`, `#77ccd0` | **Neo and SuperNeo: Post-quantum folding with pay-per-bit costs over small fields** — Wilson Nguyen, Srinath Setty | [ePrint 2026/242](https://eprint.iacr.org/2026/242) | All indexed lines 1–2423, including B.1–B.6 composition/extraction proofs and scripts |
| `salsaa.md`, `#81fec4` | **SALSAA: Sumcheck-Aided Lattice-based Succinct Arguments and Applications** — Shuto Kuriyama, Russell W. F. Lai, Michał Osadnik, Lorenzo Tucci | [ePrint 2025/2124](https://eprint.iacr.org/2025/2124) | All indexed lines 1–1801, including full Appendix A.1; decoded/read all five embedded images at lines 195, 211, 335, 525, 603 |

Reading ledger: Neo 1–180, 181–375, 376–580, 581–810, 811–1140, 1141–1420, 1421–1821; SuperNeo 1–260, 261–387, 388–560, 561–760, 761–930, 931–1110, 1111–1390, 1391–1580, 1581–1770, 1771–1960, 1961–2140, 2141–2270, 2271–2423; SALSAA 1–198, 199–370, 371–460, 461–610, 611–760, 761–900, 901–1050, 1051–1195, 1196–1350, 1351–1485, 1486–1640, 1641–1801. These are aggregate consumed intervals; some were assembled from overlapping exact QMD ranges and explicit rereads of elided text. The original PDF was additionally consulted for SuperNeo Definition 21 (PDF extraction lines 763–787) and commitment/extraction definitions (399–503). The long SALSAA comparison table was reread without per-line truncation.

**Version qualification:** publication identifiers and titles are verified, but no exact upload/revision timestamp is exposed by the retrieved primary landing-page renderer or local title pages. Do not call these “latest verified revisions.” The local SALSAA snapshot is demonstrably a later/reworked text than the landing-page abstract: it includes AIR, `salsaa-v2`, Rust 1.95.0, RoKoko 2026/575 and Osa26 2026/721, and reports different timings. Public-record abstract timings must not be mixed with this local table. SuperNeo is a separate 2026 paper, not merely a filename alias for original Neo.

**Extraction omissions/corruption:** full line coverage is not perfect mathematical transcription. SuperNeo markdown Definition 21 is heavily corrupt and several formula/table rows have OCR artifacts; its intended equations were checked against the original PDF and the adjacent algorithms/proofs. SALSAA staircase/AIR algorithm displays contain malformed symbols, but surrounding definitions and full proofs specify their equations. Original Neo has notation/self-reference errors in the extracted MCS definition. Nothing here treats illegible tokens as verified algebra; deployment needs the exact source PDFs plus an independent theorem review. No additional appendices were silently skipped. Neo and SuperNeo local markdown have no base64 image blocks.

Snapshot hashes (SHA-256):

- Neo PDF `aea289d8f2b4bde87e9539ff4cd4dcf14a1262f05ff3de24347e00ae99c6b5c0`; markdown `f5c5138c1c6b384d7cac200d0904d34410cdf024cfb39458f373011677004cd0`.
- SuperNeo PDF `a47a58a164d22515c982bf2da5ffaee87a9cddd0e5dbec9f37f248b1f29bb447`; markdown `77ccd0484ee521413ee39d173f329de6b9a1290e5c685db00412d30ece323d67`.
- SALSAA PDF `d5788e068508afbf730c5b79cd76287246586ae4d902c2e8ac071103f8f522fe`; markdown `81fec4945c3a1c148fe90b1ed34df51e5d76e1aa00e43ce3cf75176451f92eeb`.

## 2. Original Neo: matrix-module folding, not arbitrary source reinterpretation

References: [Neo](https://eprint.iacr.org/2025/294), §§3–5, indexed lines 263–501, 507–698, 700–810; complete extraction/composition proofs in Appendix B, lines 1050 onward.

Let F be a prime field, R=F[X]/Phi(X), and d=deg(Phi). A ring vector's coefficient matrix is Z in F^(d×m). Multiplication by a ring element acts by its d×d rotation matrix; the commutative ring S of these matrices is isomorphic to R. Thus an Ajtai commitment can be viewed as an **S-module** homomorphism on coefficient matrices, not as an arbitrary F-linear homomorphism on scalar evaluations. It is precisely this enlarged module action that supports coefficient-row mixing while retaining all partial evaluation coordinates.

The original matrix evaluation relation ME(b,L), Definition 18, contains commitment c, public coefficient-matrix prefix X, point r, and each vector y_j in K^d, with witness Z satisfying:

- c=L(Z), X=L_x(Z);
- every centered coefficient of Z has magnitude <b;
- y_j=Z M_j^T hat(r), where hat(r) is the multilinear equality vector.

The constraint structure contains scalar CCS matrices M_j and a bounded-degree polynomial f. Fresh scalar witnesses are decomposed and tied to this coefficient-matrix representation; fresh constraints and the evaluation relation are reduced together. The later SuperNeo paper explicitly characterizes original Neo as requiring SIMD constraint systems: the rowwise shared M_j structure does not automatically express arbitrary constraints on all d coefficient rows of an Akita source.

Update: CCS reduction checks fresh constraints, old partial evaluations and coefficient ranges using one field-extension sumcheck; RLC combines coefficient matrices, commitments, public prefixes and all d-coordinate evaluation vectors with rotation-matrix challenges; DEC decomposes the enlarged coefficient matrix into k signed base-b matrices and checks recombination of commitments/evaluations. The accumulator is k small ME instances, not a single unbounded scalar witness.

Security: the CCS reduction is commitment-preserving and has restricted extraction; RLC has relaxed extraction and uniqueness from relaxed binding. Neither is an unrestricted RoK alone. Neo Theorem 5 and Appendix B.10 compose them; Corollaries 2/3 give a RoK for the complete update. DEC is an ordinary RoK (Theorem 4). Inversion is used algebraically in extraction; shortness comes from the range check and relaxed-binding comparison, not from an inverse being short.

**Akita verdict:** existing rotation arithmetic and sparse coefficient adjoints may be reusable, but Neo's scalar CCS decomposition and module commitment are not equal to the original signed raw source plus A/B/F paths. Original Neo is not the preferred arbitrary K-frontend replacement; use the corrected/non-SIMD SuperNeo interface if considering this family.

## 3. SuperNeo: exact maps and theorem interface

References: [SuperNeo](https://eprint.iacr.org/2026/242), Definitions 1–8, 13–15, 19–22; Theorems 8–13, Lemmas 7/8; §§7/8; Appendices B.1–B.6. Indexed landmarks: fields 401–434, commitment bounds 451–500, maps 581–785, strong/weak composition 811–935, relations/update 939–1153, concrete parameters 1155–1187, proofs 1340–2227.

### 3.1 Representation and inner-product identity

Use distinct notation to avoid confusing the paper's K with the Akita frontend K:

- F=F_q is the **commitment base field**.
- L=F_(q^tau) is the **native relation field**.
- K_sc=F_(q^nu) is the **sumcheck challenge field**.
- A=F_(q^lcm(tau,nu)) contains both L and K_sc under specified inclusions.
- For each B in {F,L,K_sc,A}, R_B=B[X]/Phi(X), with the same cyclotomic Phi of degree d.

Choose a fixed ordered F-basis beta of B. Flat_B writes each native field coordinate in that basis, in coordinate-major order. Emb_B maps a length-d coefficient block to its polynomial. Pack_B=Emb_F∘Flat_B is an **F-linear bijection** when d divides delta*n. It is not a field/ring homomorphism and is generally not B-linear. Norm preservation means that the packed coefficients are exactly the flattened F coordinates; it says nothing about preserving arbitrary field products.

Define the constant-term Gram matrix T_ij=ct(X^(i+j)) in R_F. Lemma 5 proves it invertible (determinant ±1 for the cyclotomic constant term). Trans_B(a) takes a⊗beta, splits it into d-coordinate blocks and applies T_B^-1 blockwise. Then Theorem 8 proves

    ct(< Emb_B(Trans_B(a)), Pack_B(z) >) = <a,z>_B.

Theorem 9 extends this to matrices, and Theorem 10 to MLE evaluations. For power-of-two/trinomial cyclotomics Trans preserves sparsity up to a constant factor; this concerns the **public transformed matrices**, not permission to repack an existing committed source.

Theorem 11 is the needed evaluation homomorphism: given exact commitment module linearity and the **full ring-valued** evaluation vectors of transformed matrices at the same point, ring scalar combinations preserve the corresponding evaluations. Taking only ct(y) too early loses the nonconstant coordinates required to combine witnesses.

Compatible lengths: tau*n_L=d*n_R<=m, m a power of two; similarly the public-input prefix must be pack-aligned. Padding may be added with explicitly constrained zeros. Public inputs are projected packed coordinates, not an arbitrary decoder inferred from commitment bytes.

### 3.2 Relation and update

CCS(b,Lcom), Definition 20: z=[x,w] in L^n_L, centered norm ||Flat_L(z)||inf<b, c=Lcom(Pack_L(z)), and f(M_1z,...,M_tz) vanishes on the Boolean constraint domain.

CE(b,Lcom), Definition 21: c, public x, point r in K_sc^log(m), y in R_Ksc and y_j in R_A. The witness z has the same norm/commitment/prefix constraints; define

    h=Emb_F(Trans_F(Pad))*Pack_L(z),
    h_j=Emb_L(Trans_L(M_j))*Pack_L(z).

The claims are \(y=\operatorname{MLE}[h](r)\), \(y_j=\operatorname{MLE}[h_j](r)\). Their constant terms recover \(\operatorname{MLE}[\operatorname{Pad}\cdot\operatorname{Flat}_L(z)](r)\) and \(\operatorname{MLE}[M_jz](r)\), but full ring claims are retained.

A workload-selected update takes a fresh CCS instances and k old CE instances (the paper calls the fresh count K). It performs:

1. **CCS/common-point reduction:** one sumcheck over A with challenges in K_sc, combining fresh CCS identities, flattened-coordinate range polynomial product over integers -(b-1)..b-1, old full-coefficient evaluation claims and equality weights. Outputs a+k CE instances at r'. Degree is controlled by CCS degree and 2b, not segment count multiplicatively.
2. **RLC:** draw independent rho_i in a strong sampling set C⊂R_F; combine packed witnesses, commitments, packed inputs, y and y_j. Output one CE(B) instance.
3. **DEC:** signed base-b decomposition of flattened coordinates into k limbs; disclose limb commitments/evaluations; check base-power recombination. Output k CE(b) instances. DEC is deterministic at verifier level and extracts by recombination.

Norm reset requires B=b^k<q/2 and (a+k)T(b-1)<B, with T the multiplication expansion factor of C. Arity therefore changes k/B/security parameters; it is not a harmless loop constant. b=2 gives flattened coefficients in {-1,0,1}; Akita's [-16,15] source cannot be fed to this relation unchanged.

### 3.3 Soundness/extractor prerequisites

- C differences must be units, |C| sufficiently large, and K_sc sufficiently large for sumcheck/Schwartz–Zippel error. Strong sampling is proved for the specified ring/modulus, not copied from a different cyclotomic.
- Commitment is an R_F-module homomorphism and (2B,C)-relaxed binding. Ajtai specialization requires MSIS infinity-norm hardness at radius **8TB** (Section 8), not merely ordinary short-message binding at radius b.
- CCS reduction is **strong** with phi projecting its input commitments: across rewinds this projected output is fixed; extraction assumes descendant witness uniqueness.
- RLC is **weak** with the **same** phi. Coordinate-wise special extraction gives a baseline plus one challenge-coordinate fork per input. It extracts algebraic z_i=(rho_i-rho_i')^-1(z-z'), potentially large; bounded cross-products give relaxed uniqueness.
- Theorem 12 strong–weak composition supplies the actual RoK for the complete CCS/RLC pair; DEC is Theorem 13. All hypotheses are about those exact relations/oracles, not any vaguely similar commitment-preserving protocol.

Concrete parameter scripts in §8/B.5/B.6 give Almost-Goldilocks degree64 and Goldilocks/Mersenne61 degree54 trinomial settings. They are **not** Akita native d128 or ordinary parent d512 parameter sets. For example the Almost-Goldilocks row has b=2,k=13,T=128, max fresh count50 and MSIS radius8TB; changing dimensions/arity/ring invalidates its quoted lattice-estimator number. The low-norm invertibility theorem cites a theorem/conjecture source; exact applicability and any parameter-specific reliance must be reviewed rather than promoted to an Akita theorem.

### 3.4 Final decider

A folding update is not final acceptance. A transparent non-succinct CE decider may reveal all k final z witnesses and directly check centered norms, packed public prefixes, commitment equations and every transformed evaluation at r. This is a real relation decider, but certifying only that final relation does not authenticate arbitrary unverified prior updates: the complete update transcript must be checked, with an admitted composition/Fiat–Shamir theorem or with all necessary source witnesses included in a deterministic certificate. An optional SNARK wrapper needs its own relation and oracle assumptions. No supplied-tree or concrete-hash knowledge assumption is silently introduced here.

## 4. SALSAA: tensor sumcheck plus exact canonical norm checkpoints

References: [SALSAA](https://eprint.iacr.org/2025/2124), Definitions 3.2–3.8, 4.1/4.3/4.10/4.18/4.19; Lemmas 4.4/4.6/4.8/4.12/4.15/4.20; Corollaries 4.7/4.9/4.13/4.16/4.21/7.5/7.7; Theorems 5.1/7.3; full Appendix A.1. Indexed lines 199–293, 299–965, 967–1079, 1263–1431, 1649–1801.

### 4.1 Relation, geometry and embeddings

R is a cyclotomic integer ring Z[zeta_f], degree phi; R_q=R/qR, q prime not dividing f. Residue degree e gives CRT:R_q≅(F_(q^e))^(phi/e). Canonical norm is ||w||sigma,2=||sigma(w)||2, and matrix norm is maximum **column** canonical norm. It is not a coefficient infinity norm. Trace(<w,conj(w)>) equals its squared canonical norm.

Principal Xi_lin instance (H,F,Y), witness W in R^(m×r), m=2^mu, satisfies HFW=Y mod q and ||W||sigma,2<=beta. H has an identity top part. F's bottom rows are row-tensor products of mu two-entry factors. In the fully structured version all F rows are tensor; in the semi-structured version **top commitment rows may be arbitrary**, but bottom rows remain tensor. The top contains F_com.

Xi_mle appends claims \(\operatorname{MLE}[W](r_i)=s_i\). Each eq-vector \(\widehat{r_i}\) is tensor; appending it to F preserves the bottom-row requirement with zero communication and zero knowledge error.

Xi_sum appends a vector-valued sum over the Boolean cube of h(MLE[W],MLE[conj(W)]). CRT batch coordinates and outputs with powers of a random extension-field scalar; run field sumcheck and emit W evaluations at a ring point r and its conjugate. Specifically each field challenge is injected as CRT^-1 of the same field value in every slot. **This is a genuine diagonal subfield embedding for that specified CRT ring**, not an identification with arbitrary Akita coefficient packing.

### 4.2 Reductions and exact errors

Setting 3.4 requires m=poly(lambda), beta<=beta', **4(beta')^2<q**, and q^e superpolynomial in lambda. Concrete security must separately bound all error terms.

- Sumcheck (Lemma 4.4): error (r_out*phi/e-1+mu*degree_individual)/q^e; proof (degree_individual+1)*mu*e*log q +2r*log|R_q|; prover O(degree_total*m*r) ring ops. If no conjugate input is used, drop that evaluation (Remark 4.5).
- Enhanced batching (Corollary 4.7): random geometric weights aggregate bottom H/F equations, then sumcheck replaces them by one tensor evaluation row. Error (n_hat-n_bar+r*phi/e-1+2mu)/q^e; prover O(m(n_under+r)+n_under*r), verifier O(n_under(mu+r+n_under)). This **removes bottom F rows**, unlike lazy batching that only reduces H and produces cost growing with prior updates.
- Norm checkpoint (Corollary 4.9): disclose t_j=<w_j,conj(w_j)>; check balanced Trace(t_j)<=beta^2; sumcheck proves the identity; batch rows again. Knowledge direction recovers norm beta from a descendant beta' bound. No modular wrap follows from Setting3.4. Error [n_hat-n_bar+2(r*phi/e+2mu)]/q^e. Prover linear in m at fixed widths.
- Binariness (Lemma 4.12/Corollary4.13): power-of-two cyclotomic required. Let 1-circle have all coefficient entries1; Trace(conj(w)*(1-circle-w))=phi*sum a(1-a). Integer terms are nonpositive and vanish iff each a∈{0,1}; preexisting descendant norm bound prevents wrap. It is not the false statement w*(1-w)=0 in the ring. This is highly relevant to Akita physical Main Booleanity.
- Staircase (Corollary4.16): A W_0=Y_0, B W_(j-1)+A W_j=0, B W_last=Y_1; one challenge batches rows into (p⊗d)^T W=s. p geometric allows O(log steps) public evaluation, d remains a dense block vector. Total error [ (steps+1)*m_block+n_hat-n_bar+2(r*phi/e+2mu)-1 ]/q^e. This is not a general arbitrary matrix factorization theorem.
- AIR (Corollary4.21): commit V=[W,shift(W)]; separately check row-local transition, cyclic-shift identity with wrap correction, and boundary constraints. Error [n_hat-n_bar+(|C|+2t+2)*phi/e+m+(d_air+5)mu-1]/q^e; explicit prover O((d_air+2)*mu*m*t). The O(lambda*m) asymptotic AIR SNARK corollary assumes fixed widths, boundary count, degree and evaluation cost; it must not be quoted for a wide growing frontend without these conditions.

All these extract **the appropriate input witness OR a nonzero SIS witness** under the actual commitment key at a specified norm radius. They do not assume commitment witness uniqueness outright. Lemma4.4's rewind extractor: sample once, abort on failure; if successful but invalid, seek a second successful transcript. The unconditional expected number of invocations is (1-epsilon)+epsilon*(1+1/epsilon)=2. Differing extracted witnesses give a short commitment kernel; same witness faces independent Schwartz–Zippel checks. This is a paid two-run extraction argument, not zero-rewind soundness.

### 4.3 Fixed-shape accumulator and arity

Theorem7.3 core:

    norm+ → tensor random projection → fold → join → batch+ → b-decomp.

No witness-height split occurs in the folding core. Starting width r_acc+r is reduced to r_acc=2ell; top commitment rows remain unchanged and bottom rows return to one. Norm checkpoint restores extraction beta before folding. Projection adds a branch with norm m_rp*beta; fold increases norm to (r_acc+r)*gamma*beta; join keeps the larger bound; signed decomposition is chosen so the output limbs again have norm beta. The same parameters govern all reachable SIS-break radii. Corollary7.5 joins a fresh linear relation with the running accumulator under the **same top key**; Corollary7.7 reduces committed AIR first, then folds. At fixed r,n,ell the per-update bounds are proof O(lambda*log^2 m/log lambda), prover O(lambda*m), verifier O(lambda^2), plus AIR evaluation cost if present.

Batching 2/4/8 fresh sources requires their widths, norm growth and decomposition schedule to be priced explicitly. “Constant arity” is legitimate asymptotically when fixed independently of lambda; it is not permission to reuse a binary parameter estimate for arity256. Arbitrary distinct matrices/points require a proved join/common-point/homogenization layer. Appendix A.1 does precisely such a layer for generalized committed linear claims with r_i^T M_i; top rows must retain matrix/point structure. Its real prover cost is O(t*m*L), even though its fixed-t,L statement abbreviates this as O(m).

### 4.4 Final decider and assumption boundary

For Xi_lin a full-reveal decider checks W, HFW=Y and canonical norms. Theorem5.1 instead performs shrinking split/projection/fold/decomp loops, switches to unstructured small rounds, then reveals the constant-height remaining witness. Prover O(lambda*m), verifier O(lambda^2 log m), proof O(lambda log^3 m/log lambda), for its stated fixed-width parameters. Fully tensor keys require **vSIS**, not ordinary unstructured SIS. Semi-structured folding can use unstructured SIS for top keys, but the provided succinct final theorem is for fully structured Xi_lin-tensor. One may instead use the large direct decider; importing Theorem5.1 unchanged for arbitrary Akita matrices is not justified.

The paper's implementation uses BLAKE3 Fiat–Shamir. The interactive RoK theorem and experimental FS implementation are not by themselves an end-to-end classical-ROM knowledge theorem for an arbitrary adaptively generated many-segment application. No concrete-BLAKE3 knowledge/game-preservation assumption is approved by this note.

## 5. Precise Akita applicability tests

Akita facts are grounded in the inspected existing research interfaces [native source-tree note](AKITA_INSTANCE_ACCUMULATION_NATIVE_SOURCE_TREE.md) and [common K normalization note](AKITA_INSTANCE_ACCUMULATION_COMMON_K_NORMALIZATION.md), plus the dedicated Akita reading slice. Native ambient ring is F_q[X]/(X^d+1), d128 in the native shell; ordinary benchmark parent currently uses d512/nv14. q is policy-selected; no numerical q/security assertion is made here.

| Obligation | SuperNeo/Neo compatibility | SALSAA compatibility | Required bridge |
|---|---|---|---|
| Physical Main Boolean coefficients | SuperNeo b2 permits {-1,0,1}, not specifically {0,1} | Power-of-two trace binariness directly matches a coefficient predicate | Tie the check to the **same original physical source**, including all designated zeros |
| Original RAM signed planes [-16,15], 26 base32 digits | Needs declared base-b limb decomposition and recombination, or b17 range handling; neither silently changes source | Canonical norm possible; binariness alone does not enforce signed digit alphabet | Retain digit planes/range relation and fixed decoder Y=Σ32^ell d_ell; never re-decompose decoded Y |
| Ordinary logical K evaluation | L can equal frontend K if it is a compatible extension of the same F, with fixed basis/inclusions | Needs proved map to diagonal CRT field or explicit coefficient arithmetic | K is a field; negacyclic R is generally a product of fields, not K itself |
| Original A/B/F commitment and D/H opening paths | Not a single linear commitment Lcom(Pack(z)) to arbitrary scalar z | Top key may be unstructured, but original multi-level path commitments need equations | Commit/retain source **and** all path witnesses; prove original images, decomposition links and occurrence-specific bounds |
| Native weak source S | Inverse-based extraction is algebraic; strong–weak theorem applies only after relaxed uniqueness and range restoration | Exact norm checkpoint requires descendant beta', not weak product bound alone | No inverse-shortness inference; obtain source-linked norm/domain theorem or reveal full source in final certificate |
| Different opening points across segments | Common-point CCS reduction available within exact CE structure | AppendixA homogenization available for its exact generalized relation | Preserve actual raw-map adjoints, source column geometry and application claim roster |
| Many segments without every PCS | Fold fresh native relations, not completed proofs | Fold fresh bounded linear/AIR relations, not completed proofs | Fresh relation must include every source/decoder/frontend obligation formerly authenticated by PCS |

Existing native weak terminal bounds are on rho*S=r: ||rho||1<=112 and ||r||inf<=14336. They are **not** ||S||inf<=14336. Source comparison uses rho' r-rho r', bounded by K'B+KB', on the actual matrix occurrence/view. Strong alphabets are [-16,15] for source digits, [-1,0] for F/H compression spans, with scheduled gaps zero. Original C binds A inner block images, B sliced digit paths and F compression paths; D/H are point-dependent opening payloads and are shared only within an appropriate shell fork. Canonical honest path reconstruction would discard valid noncanonical sources; both papers' representation changes need a proof preserving this exact relation.

The frontend seam is already a linear-functional normalization over original source/decoder views: bind full ordered roster before coefficients, reduce to one pending original raw/decoded query, and authenticate it later. This is algebraically compatible with sumcheck ideas. It does **not** discharge Main Booleanity, RAM signed ranges, physical zeros or PC constraints. Distinct original signed planes can decode to the same K value modulo q, so decoded equality cannot authenticate the original commitment.

**[INFERENCE — a valid norm upper bound, not a complete protocol theorem.]** For a power-of-two degree-d ring, canonical norm squared is d times coefficient Euclidean norm squared. If one column has N physical coefficients bounded in absolute value by16, then beta^2<=256*d*N. A Main Boolean column has beta^2<=d*N. Whether SALSAA's 4(beta')^2<q holds depends on the larger **extraction** beta', projection/decomposition schedule and path witnesses; checking only honest beta is insufficient.

**[INFERENCE — frontend representability.]** A complete Akita polynomial relation can be expressed in field coordinates using fixed multiplication matrices, decoder adjoints and range/decomposition constraints. This is a new arithmetization that may be much larger than native Akita, not an already proved homomorphic embedding. SuperNeo can address its field CCS form; SALSAA can address ring-native equations only if coefficient nonlinearities are separately encoded. Neither gives cost parity automatically.

## 6. Many-segment costs and paid extraction

Let N be segment count, a arity, M physical per-segment witness/constraint size, k or 2ell accumulator width. With independent equal-shaped segments, a balanced a-ary reduction tree has depth h=ceil(log_a N) and O(N/(a-1)) internal nodes. Each leaf enters once; fixed-dimension limb/path work must be counted per node. A serial accumulator remains O(N) updates but is not automatically safe for nested extraction cost.

For SuperNeo, each update scans a+k witness families, their matrix products, coordinate range polynomials, and limb commitments. Approximate fixed-parameter work is O((a+k)*(nnz matrices + b*M)) plus decomposition/commitment work; the exact transformed-ring evaluation messages scale with d*(a+k)*(t+1). Increasing a can reduce update count while enlarging B, k, MSIS radius and final state. There is no product-of-every-round expansion factor across updates **if DEC and CCS range checks are retained**. Removing norm reset makes that statement false.

For SALSAA, fixed-arity/fixed-width work is O(lambda*M) per update, but raw arity width, join matrices, projection, norm checkpoint, digit decomposition and application sumcheck are all charged. Total may be O(N*lambda*M) with one final O(lambda*M) decider at fixed shape, or O(lambda*N*M) for a single concatenated source proof. This is not a measured speed prediction. Dense arbitrary bottom matrices can destroy tensor evaluation efficiency and require explicit auxiliary representations/proofs.

**Extractor distinction:**

- SALSAA §5.2 explicitly pays 2^O(log m) expected calls for its logarithmic-height final shrinking proof; this is polynomial in m when m=poly(lambda). The justification cannot be extended to N serial relation updates by replacing log m with N and still calling it polynomial.
- SALSAA's local RoK composition and “consecutive steps compose” statement establish fixed compositions; an unbounded-asymptotic serial chain needs a quantitative global extractor/runtime theorem. Repeated nesting with factor c>1 per update gives c^N. A balanced tree gives c^h=N^(log_a c) (when c is fixed for fixed arity), with polynomial full-tree traversal, but this formula itself requires a composable node oracle and paid acceptance/rejection sampling theorem.
- SuperNeo Theorem7 coordinate-wise extraction uses at most (a+k)+1 oracle calls in expectation. This is a powerful local bound, but descendant oracle calls and the strong–weak composition sampler are not free. A balanced tree can keep composition depth logarithmic; cite/pay the complete node extractor, uniqueness failures, ROM query bounds and all recursive calls. Theorem12 cannot be applied if rewinds change the projected commitment roster.
- Error totals must include all node checks, final decider, commitment comparisons and FS compilation; for polynomial N a union bound is possible only after each relevant event and its adaptive conditioning have been defined. Neither “2^-128 hash” nor one-shot sumcheck error is a knowledge/extractor theorem.

**Conservative allowed endpoint:** a large deterministic final certificate containing every original source and path witness can let an independent verifier evaluate the full ordered per-segment relations and all boundaries directly, plus any declared accumulator recombination. This avoids counterfactual-tree extraction as a security premise, at the cost of linear disclosure/verification and no zero knowledge. It is not succinct folding and should not be marketed as such. If the certificate verifies only the final accumulator and hides the inputs, the global extraction/ROM theorem is again necessary. The user permits a large certificate, so this distinction matters.

## 7. Public Rust and exact reusable patterns

Verified public code: [lattice-arguments/salsaa-v2](https://github.com/lattice-arguments/salsaa-v2), Apache2.0 according to README. The fetched main branch has package salsaa0.1.0, edition2024, `rokoko` as an unpinned git dependency and `rayon`1.12.0. No commit hash was pinned in this investigation; URLs describe the observed main branch, not reproducible locked code provenance.

Directly inspected:

- [`src/protocol/config_generator.rs`](https://github.com/lattice-arguments/salsaa-v2/blob/main/src/protocol/config_generator.rs): `build_round_config` and `build_unstructured_round_config` produce shrinking proof schedules; the first folding-scheme round uses8 witness columns (4accumulator+4input), `split_factor=1`, and no recursive `next`; proof mode uses height splitting. This is the exact reusable distinction between **relation update** and **final shrinking proof**. Hardcoded widths are not a general arity2/4/8 API.
- [`sumchecks/runner_prover.rs`](https://github.com/lattice-arguments/salsaa-v2/blob/main/src/protocol/sumchecks/runner_prover.rs): derive field challenge after hashing round coefficients; lift it into the homogenized CRT representation; partially evaluate all subclaims together; preserve original witnesses to compute per-column evaluations at the endpoint and its conjugate. It explicitly reverses LS-first sumcheck challenges to the legacy MS-style outer-first representation. Reuse this ordering discipline, not its point order blindly.
- [`sumchecks/runner_verifier.rs`](https://github.com/lattice-arguments/salsaa-v2/blob/main/src/protocol/sumchecks/runner_verifier.rs): `batch_claims` weights ordinary evaluation, l2, exact-binariness, VDF and projection claims; a separate combination maps CRT slots to quadratic-extension sumcheck. Each round checks p(0)+p(1)=running claim, hashes coefficients, samples the extension challenge and updates the endpoint. This is genuine batched sumcheck bookkeeping. Its `assert_eq!`/`expect` rejection style is demonstration code, not a ready hostile-byte certificate-verifier API.
- Listed builder/context/loader modules in [`src/protocol/sumchecks`](https://github.com/lattice-arguments/salsaa-v2/tree/main/src/protocol/sumchecks) and prover/verifier/executor modules in [`src/protocol/parties`](https://github.com/lattice-arguments/salsaa-v2/tree/main/src/protocol/parties) are implementation handoff targets; they were not all deep-audited here. Heavy ring arithmetic/commitment/projection machinery is delegated to [`rokoko`](https://github.com/lattice-arguments/rokoko), with [`incomplete-rexl`](https://crates.io/crates/incomplete-rexl) backend.

Published SALSAA local benchmark (Table5) is **linear-relation folding**, not AIR or an Akita frontend: at 2^26,2^28,2^30 F_q coefficients, commitment times1.10,4.37,18.64seconds, update prover3.92,16,62.86seconds, verifier0.44,0.45,0.45milliseconds, proof62,62,66KB. The instance includes accumulator half and input half. Local Table3 full SNARK has much larger prover times27.65,112.1,445.3seconds and proofs1092,1295,1498KB. Hardware is a dual Xeon8562Y+ AVX-512 server. Section8.3 expressly leaves full AIR folding implementation open. These are evidence of a mechanism, not comparable Akita1/2/3/256segment timings.

No public Neo/SuperNeo Rust implementation was verified from the primary papers or targeted public search. Do not invent a crate or claim a maintained upstream implementation. Their algebraic maps and theorem parameters are reusable specifications; SALSAA has the concrete Rust implementation evidence.

## 8. Theorem-level prerequisites before admission

1. Freeze original source/path/decoder/frontend relation with all signed planes, physical zeros, PC and boundary constraints; define commitment projection retained across rewinds.
2. Prove a witness-preserving bridge to exact SuperNeo CCS/CE or SALSAA Xi relations, including reverse extraction to original Akita sources. An F-linear coefficient map alone is insufficient.
3. Establish field basis/inclusions or diagonal CRT injection, multiplication and evaluation adjoints; no K=R identification or hash homomorphism.
4. Choose actual q,Phi,d, challenge law, source/path bounds, arity, limbs and all extraction radii; prove unit differences and relaxed-binding/SIS hardness for those parameters. Published estimates for different rings do not transfer.
5. Supply source-linked exact domain/norm checkpoint and no-wrap theorem. Weak rho*S bounds alone fail this gate.
6. Decide final decider: full source/path certificate, revealed final relation plus globally sound transcript, or a proved succinct final protocol. Semi-structured top commitments do not inherit the fully structured vSIS final SNARK theorem automatically.
7. Provide a quantitative many-segment extractor/ROM theorem with paid calls and logarithmic-depth schedule or another proved polynomial bound. Do not resurrect exponential supplied-tree extraction behind a local polynomial claim.
8. Independent verifier consumes the complete ordered manifest and checks every relation it purports to certify; fixed transcript hash authentication is separate from concrete-hash knowledge.
9. Only then adapt **real upstream Akita workloads** to1,2,3,256segments and compare total commit/update/finalization/verification/certificate costs. No performance acceptance criterion was exercised in this slice.

Result: SALSAA and SuperNeo provide substantive native mechanisms worth considering within the protocol-candidate cap, but **neither is an already discharged drop-in theorem for the current Akita source/decoder/path/frontend relation**. A slower explicit-source certificate is a sound conservative reference endpoint; a hidden-source folding endpoint needs the stated bridge, extraction and ROM prerequisites proved rather than assumed.
