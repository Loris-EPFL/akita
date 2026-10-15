# Range-aware lattice folding and the native Akita commitment graph

## Scope and decision

This is a research slice, not an implementation or an admitted protocol theorem. It studies **LatticeFold**, **LatticeFold+**, and **Cyclo** for accumulation of segment relations before complete per-segment Akita PCS proofs. The intended final object may be a large certificate. Nothing here substitutes an Ajtai commitment for Akita's root commitment or assumes shortness of an inverse.

**Decision-relevant result:** these papers supply useful range-check, norm-refresh, and short-denominator techniques, but none is a drop-in theorem for folding Akita's compressed root payloads. Akita's inner A map is linear; its B-of-digits-of-A construction and subsequent F compression are not. LatticeFold requires an actual module homomorphism with relaxed binding. LatticeFold+ explicitly solves a particular nonhomomorphic double-commitment problem by a substantial commitment-transformation protocol, not by declaring compression homomorphic. Cyclo's partial-range optimization only omits checks on a distinguished accumulator, never checks on arbitrary fresh source coordinates. It also has material proof and parameter qualifications described below.

A compatible, conservative fragment is to keep the original native commitment graph and discharge **all** its source, digit, ring-consistency, and domain obligations in the one final decider/certificate. A final clear certificate can enforce these obligations exactly without any new hash-knowledge assumption. Folding the compressed payloads alone cannot enforce them.

## 1. Sources, version identity, and coverage

Primary collection: `lattice-papers`, accessed using qmd `get` by these document IDs. Exact section/lemma numbers below refer to these versions, not an assumed later revision.

| ID / qmd file | Actual title | Version printed in document | Full extraction extent |
|---|---|---|---|
| `#7add65`, `lattice-papers/latticefold.md` | *LatticeFold: A Lattice-based Folding Scheme and its Applications to Succinct Proof Systems* | December 15, 2025 | lines 1–2039 |
| `#f50024`, `lattice-papers/latticefoldplus.md` | *LatticeFold+: Faster, Simpler, Shorter Lattice-Based Folding for Succinct Proof Systems* | August 9, 2025 | lines 1–1482 |
| `#051ac2`, `lattice-papers/cyclo.md` | *Cyclo: Lightweight Lattice-based Folding via Partial Range Checks* | No date on extracted opening | lines 1–1169 |

LatticeFold was retrieved sequentially at starts 1, 181, 481, 781, 981, 1181, 1361, 1561, 1801, and 1981, with overlapping requests through its end. LatticeFold+ was retrieved at starts 1, 181, 401, 581, 761, 941, 1121, and 1321 through its end. Cyclo was retrieved at starts 1, 181, 381, 581, 781, and 981 through its end; the final appendix was additionally retrieved exactly as 981–1080 and 1081–1169, and line 1169 was checked as its end. The three local parsed counterparts have the same line extents and were used for compact, numbered reading and targeted rereads of long mathematical passages.

Coverage includes LatticeFold §§1–7, Appendix A and all of B.1–B.4; LatticeFold+ §§1–6, Appendix A and the complete tensor-of-rings Appendix B; Cyclo §§1–7, Appendix A.1–A.3, B.1–B.2, and C.1–C.4, including bibliographies and parameter/benchmark tables. No appendix was deliberately excluded. **Retrieval caveat:** some large initial tool responses were display-elided; complete extraction was loaded locally and key mathematical passages were reread in smaller ranges. This is extraction-level coverage, not an independent visual inspection of every PDF page. It does not certify that the parser preserved every symbol. In particular, the parser loses LatticeFold+ Eq. (17) and much of Cyclo Eq. (6); these were recovered from local PDF text as specified below. Dependencies such as LS18, FMN24, KP23, ALS20, and BL25 were not independently read in full in this slice.

Supplemental primary sources:

- LatticeFold ePrint: <https://eprint.iacr.org/2024/257> (version-specific claims here come from the collection's December 2025 text).
- LatticeFold+ publication: <https://doi.org/10.1007/978-3-032-01907-3_11>.
- Cyclo author/publication record: <https://research.aalto.fi/en/publications/cyclo-lightweight-lattice-based-folding-viapartial-range-checks/>.
- Cyclo supplementary Rust repository, pinned to **`31158a781c16a967c714c7fbde31ae1558e23118`**, author date 2026-02-22: <https://github.com/osdnk/cyclo/tree/31158a781c16a967c714c7fbde31ae1558e23118>.
- Akita comparison uses local `Papers/parsed/Akita.md`, principally §§2.1, 2.5–2.7, 4.5, 7, and the extraction contract in Definition 10.13 / Lemma 10.15. The Akita deep-reading slice owns full Akita coverage.

PDF text recoveries: `Papers/LatticeFoldPlus.pdf` lines 500–545 recover Eq. (17). `Papers/Cyclo.pdf` lines 245–320 recover principal relation (6); lines 395–500 confirm that the sign/denominator/challenge-coefficient disagreements in its extraction are also present in PDF text, not merely qmd's TeX formatting. No repository source, branch, build, test, lint, or formatter was changed or run.

## 2. Common algebraic prerequisite: short numerators, not short inverses

Let

\[
 R=\mathbb Z[X]/(X^d+1),\qquad R_q=R/qR,
\]

with power-of-two d and odd prime q for the original LatticeFold and LatticeFold+ analyses. The norm is the coefficient infinity norm on a centered integral lift. For a short challenge family S, put

\[
 \gamma=\max_{c\in S}\|\operatorname{Mul}_c\|_{\infty\to\infty}.
\]

A **strong sampling set** has unit differences between every distinct pair. This gives algebraic division during extraction. It does **not** give a small norm for that division. Even the unit 2 has a modular inverse with centered magnitude about q/2. Therefore

\[
 \|\Delta w\|_\infty\le D\quad\not\Rightarrow\quad\|w\|_\infty\le D.
\]

The standard soundness move is to represent an extracted opening as a short numerator divided by a short unit, then cross-multiply *two* inconsistent openings. If

\[
 A(z_1/s_1)=A(z_2/s_2),\quad z_1/s_1\ne z_2/s_2,
\]

then

\[
 x=s_2z_1-s_1z_2\ne0,\qquad Ax=0,
\]

and

\[
 \|x\|_\infty<2b\|S_{\rm denom}\|_{\rm op}
\]

when both numerators have norm <b. This is the precise relaxed-binding reduction in LatticeFold+ Definition 2.3, §2.3, lines 246–252. LatticeFold §2.3 uses the equivalent weak-opening equation `Δ cm = A x`; its B-relaxed binding requires Module-SIS radius `4 γ B`. Its fold needs 2B-relaxed binding, hence the advertised **8 γ B** radius (Theorem 3.1; Theorem 3.3).

For Akita, Definition 10.13 / Lemma 10.15 use the same distinction. From forked responses z,z′ one recovers

\[
 s_i=(c_i-c_i')^{-1}(z-z'),
\]

while the controlled object is `(c_i-c_i′)s_i = z-z′`. Akita's overview Eq. (11) cross-multiplies weak openings to produce the short A kernel vector. A theorem about these bounded products is not a theorem that every extracted source block is short, binary, or one-hot.

### Exact LS18 challenge condition

For power-of-two d and factor count t>1, LatticeFold Lemma 2.3 / LatticeFold+ Lemma 2.4 use

\[
 q\equiv1+2t\pmod{4t},\quad t\mid d,\qquad
 0<\|y\|_\infty<q^{1/t}/\sqrt t\ \Longrightarrow\ y\in R_q^\times.
\]

For coefficients bounded by β, **differences** are bounded by 2β, so exact admissibility requires

\[
 2\beta<q^{1/t}/\sqrt t.
\]

The uniform cube has `(2β+1)^d` elements. Sparse support of h prescribed coefficients has `(2β+1)^h` elements and infinity-operator norm ≤βh. This is an algebraic guarantee at the selected q,d,t, not a consequence of entropy alone. LatticeFold Lemma 2.2 gives the coarse `γ≤d·max coefficient` bound. Challenge filtering changes the distribution, and entropy/accepting-fiber/response bounds must be priced for the filtered family rather than an unfiltered nominal family.

All SIS radii in this note are coefficient-infinity radii unless explicitly stated otherwise. For an m-column degree-d module map, converting a certified infinity radius D to the usual coefficient-Euclidean estimator input gives radius at most `sqrt(d m)·D`. Changing digit expansion or helper width changes m and the resulting security estimate; retaining only D while ignoring this dimension change is unsound parameter accounting.

## 3. LatticeFold: exact relation and where every range check enters

### 3.1 Relation, setup, and coefficient embedding

Construction 2.1 samples a uniform `A∈R_q^{κ×m}` and commits `cm=A f` for coefficient-short `f∈R_q^m`. Plain binding for norm <B follows from Module-SIS radius 2B. This is a deterministic binding commitment, not by itself a hiding commitment; §2.3 says one can append a small random vector for hiding.

The committed opening relation is

\[
 \mathcal R_{\rm cm}^{B}=
 \{(cm;f):Af=cm,\ \|f\|_\infty<B\}.
\]

The accumulated evaluation relation additionally carries a random MLE point r and an evaluation of a **coefficient-embedded** f. The paper constructs \(\widehat f\) such that its NTT coordinates equal the coefficient columns of f (Eq. (2), Eq. (9), and §3.3). In the partially split case `R_q≅F_{q^τ}^{d/τ}`, this needs τ ring-valued embedded vectors, not a claim that coefficientwise MLE and ring multiplication coincide automatically. `RotSum` / the rotation matrix are used so that multiplication of actual ring witnesses corresponds to the correct operation on the coefficient evaluation claims; Lemma 3.2 and Appendix A establish that compatibility.

The generic form is

\[
 \mathcal R_{\rm hom}^{B}=\{(r,\widehat v,y;f):
  \mathcal L(f)=y,\quad
  \operatorname{MLE}[\widehat f](r)=\widehat v,\quad
  \|f\|_\infty<B\},
\]

where **\(\mathcal L\) is a sampled \(R_q\)-module homomorphism** satisfying 2B-relaxed binding. This last premise is indispensable to Theorem 3.2 / Theorem 3.3. Calling a nonlinear compressed commitment “binding” does not meet it.

### 3.2 Decompose, range-check, and fold

Theorem 3.1 composes expansion, decomposition, and folding:

\[
 \mathcal R_{\rm eval}^{B}\times\mathcal R_{\rm cm}^{B}
 \longrightarrow(\mathcal R_{\rm eval}^{b})^{2k}
 \longrightarrow\mathcal R_{\rm eval}^{B}.
\]

Choose `b^k=B`. Each source, including the input accumulator, is decomposed coefficientwise as

\[
 f=\sum_{j=0}^{k-1} b^j f_j,\qquad\|f_j\|_\infty<b.
\]

The decomposition publishes commitments/evaluations of **all k chunks** and checks their linear recomposition. Its extraction is direct recomposition, not division. Its range polynomial is

\[
 P_b(T)=\prod_{j=1-b}^{b-1}(T-j).
\]

Using the coefficient embedding, batched sum-check checks `P_b` on every Boolean coefficient location of every decomposed witness. One evaluation of the range polynomial is not enough without authenticating the underlying coefficient MLE and tying it to the same committed witness. Figure 3's sum-check includes those ties, previous evaluation claims, separator/equality weights, and the range polynomial. Its range degree is 2b−1; separator multiplication gives degree at most 2b in the corresponding range summand. Theorem 3.3 prices its complete extraction argument, not only the honest range equation.

With `2k` independent short challenges,

\[
 f_o=\sum_{i=1}^{2k}\rho_i f_i,\qquad
 y_o=\sum_i\rho_i y_i.
\]

Completeness requires

\[
 2k\gamma(b-1)<B<q/2.
\]

Theorem 3.3's extracted **actual input chunks** satisfy norm <b, except for its stated knowledge error:

\[
 \kappa_{\rm LF}=\frac{2k}{|C_{\rm small}|}+\epsilon_{\rm bind}
 +\frac{(2b+1)\log m+4k}{|C|}.
\]

Here `C_small` is for low-norm folding; C is the large strong sampling set for sum-check. The theorem gives expected extractor time

\[
 \left(1+\frac1{\epsilon_{\rm fold}-2k/|C_{\rm small}|}\right)(1+2k).
\]

The proof runs a real accepting-fiber extraction and uses two successful extractions plus relaxed binding to stabilize the actual divided witnesses. It then restores their range and previous evaluation obligations. Merely solving the forked linear system and bounding its short numerators omits this essential second part.

### 3.3 CCS, arity growth, and final decider

§4 linearizes CCS to the committed linear/evaluation relation by sum-check, then uses decomposition plus range checks plus folding. Appendix B.1 proves the CCS linearization; B.2 proves its decomposition; B.3 proves Theorem 4.2; B.4 proves the optimized batch fold, including the original CCS gate-check. Witness/public-prefix obligations are carried explicitly through `L_w`, not inferred from a folded commitment.

For two sources, arity inside the fold is 2k because of decomposition. For p sources decomposed to k chunks each, the natural completeness envelope becomes `p k γ(b−1)<B`; this is an adaptation of the norm calculation, not an independently proved p-ary theorem in this slice. The original theorem is the 2k-input construction. Remark 5.1 reduces k by choosing the smallest k* for which `b^{k*}≥2k*(b−1)γ`; Remark 5.2 explicitly leaves non-power-of-two cyclotomic rings conjectural.

The final decider is a proof or clear verification of the final accumulated relation **plus the folding verifier logic/public binding needed for IVC**. The discussion headed “IVC proof compression” at lines 1484–1488 allows witnesses in the clear, or a separate STARK/LaBRADOR proof of the final statements; it does not make a PCS opening alone into a relation decider. The last online statement may be kept as uncommitted CCS, or the final two statements may be folded once more. This is a compiler decision with its own soundness and optional zero-knowledge conditions.

**ROM admission blocker:** December 2025 **Remark 3.3**, line 1047, explicitly says Theorem 3.3 applies only to the interactive protocol and leaves knowledge analysis of the Fiat–Shamir version to future work. Therefore this text alone does not admit a noninteractive classical-ROM Rust accumulation protocol. A valid ROM compilation theorem covering its adaptive multi-coordinate extraction and transcript order must be supplied independently; a concrete BLAKE3 transcript is not that theorem.

## 4. LatticeFold+: algebraic range proof and double-commitment transformation

### 4.1 Exact committed linear relation

Definition 3.1 (Eq. (5)) uses a **linear** commitment `com(f)=A f`, common fixed matrices `M^(i)`, and

\[
 \mathcal R_{\rm lin,B}=\{(cm_f,r,v;f):
 \|f\|_\infty<B,\quad cm_f=\operatorname{com}(f),\quad
 \langle M^{(i)}f,\operatorname{tensor}(r)\rangle=v_i\ \forall i\}.
\]

There are two independent evaluation points: `M_C=C×C` and `M_q=R_q×R_q`. These are not expendable syntactic duplication. Remark 4.9 explains that its extraction union bound over as many as `μ|S̄|` adversary calls becomes meaningless with only one final sum-check.

Appendix A reduces committed R1CS to this relation. The original witness z is gadget-recomposed from coefficient-short f with `z=G_{B,ℓ}^T f`, `ℓ=ceil(log_B q)`; the R1CS constraint stays explicit in the reduction. The prefix `(1,x)` and commitment opening are not implied by a single linear evaluation.

### 4.2 Ring identities needed by the range proof

Put `d′=d/2`, `B=(d′)^k`, and

\[
 \mathcal M=\{0,1,X,\ldots,X^{d-1}\}\subset R_q.
\]

**Lemma 2.1 is a polynomial-ring theorem:** over `F_q[X]` with q>2,

\[
 a(X^2)=a(X)^2\iff a\in\{0,1,X,X^2,\ldots\}.
\]

It is **false** in the quotient `R_q`. Remark 2.1 gives a nonmonomial counterexample in `F_q[X]/(X^4+1)` when q admits a square root of −1. The monomial check therefore evaluates the canonical coefficient polynomial at β and β² in the field and authenticates those evaluations. An implementation that substitutes quotient-ring squaring for this step accepts false monomials.

For integer a in `(-d′,d′)`, define `exp(a)=sgn(a)X^a` in R_q. Negative exponents reduce using `X^d=−1`, making `−X^a=X^{a+d}` for a<0. Let

\[
 \psi=\sum_{i=1}^{d'-1}i(X^{-i}+X^i).
\]

Lemma 2.2 says

\[
 \operatorname{ct}(\psi m)=a,\quad m\in\mathcal M
 \iff a\in(-d',d'),\quad m\in\operatorname{EXP}(a).
\]

`EXP(0)={0,1,X^{d/2}}`; the range argument must allow these representations. The sign pattern and negacyclic reduction are part of the proof, not a cyclic-convolution optimization.

Decompose **all coefficients** of f to `D_f∈F_q^{n×dk}`, `||D_f||∞<d′`, and introduce `M_f∈EXP(D_f)⊆M^{n×dk}`. The range proof binds M_f, certifies each monomial, and checks the constant-term gadget reconstruction of cf(f). This really proves the full coefficient range `||f||∞<B`, not only a projection norm. Construction 4.4 / Theorem 4.2 / Lemma 4.7 reduce this to delayed linear/evaluation obligations with error

\[
 \epsilon_{\rm rg}=\epsilon_{\rm mon,dk+1}
 +\epsilon_{\rm bind}+\log n/|C|.
\]

### 4.3 What double commitments do—and do not—buy

For `M∈R_q^{n×m}`, first compute `com(M)=A M`. The published double commitment is

\[
 \operatorname{dcom}(M)=\operatorname{com}(\operatorname{split}(\operatorname{com}(M))).
\]

`split` is injective gadget-decomposition plus coefficient flattening and padding; it puts the intermediate image in `(-d′,d′)^n`. `pow` recomposes it and satisfies `pow(split(D))=D`, but is **not injective**. A valid opening is a pair `(τ,M)` with `com(τ)=C_M` and `pow(τ)=com(M)`; τ need not be the canonical split. Lemma 4.1 reduces a collision to the appropriate inner or outer linear commitment collision.

Section 4.4 opens by stating that **double commitments are not linearly homomorphic**. Its construction does not form an alleged homomorphic sum of C_M payloads. It instead obtains a new linear commitment to

\[
 g=s_0\tau_D+s_1m_\tau+s_2f+h,\qquad h=M_f s',
\]

where `s∈S̄^3`, `s′∈S̄^{dk}`. The prover sends `com(h)` after s′; the earlier fixed double commitment and two independent batched sum-checks enforce its consistency. `pow(τ_D)s′=com(h)` is checked through structured recomposition weights (Eqs. (19)–(21)). The final Eq. (17), recovered from PDF text, requires

\[
 \langle g,\operatorname{tensor}(r_o)\rangle=v_o,
 \quad\|g\|_\infty<b/2,
 \quad(cm_g,g)\in\mathcal R_{\rm open}.
\]

Completeness of Theorem 4.3 / Lemma 4.8 needs

\[
 b\ge2\gamma(d'+1+B+dk),\qquad
 \|g\|_\infty<\gamma(d'+1+B+dk).
\]

The extraction returns divided openings with denominator in `S̄−S̄`, then proves the original range/monomial/double-commitment relations. It does not assume `||M_f s′||` is bounded for an arbitrary extracted M_f: lines 902–908 explicitly flag that missing bound before the subsequent soundness argument restores the necessary structure.

Lemma 4.9 is the FMN24 coordinate-wise accepting-fiber theorem. With `μ=3+dk`, it outputs μ+1 accepting challenge/response pairs with error μ/|S̄|, `1+μ` expected oracle calls, and exponential-in-security-parameter worst-case number of calls. Lemma 4.10 uses this expected-time extractor and two independent final sum-checks. Its displayed error is

\[
 \epsilon_{\rm cm,k}=
 \frac{\mu+dk}{|\bar S|}+3\epsilon_{\rm bind}
 +\epsilon_{\rm rg}'+\epsilon_{\rm rg}+2\epsilon_{\rm sum}
 +\frac{\mu(\log^2\kappa+(2\log n)^2)}{|C|},
 \quad\epsilon_{\rm sum}=(2\log n/|C|)^2.
\]

This repeated-query analysis, including the two repetitions and commitment stability events, cannot be replaced by a claim that one fresh coordinate preserves acceptance.

### 4.4 Arity, refresh, assumptions, and decider

Section 5 uses **L total inputs**, consisting of L−2 fresh relations and **two** accumulated relations. The L-input transform first gives one witness of norm <B², then decomposes that witness into two B-bounded witnesses:

\[
 \mathcal R_{\rm lin,B}^{(L)}\to\mathcal R_{\rm lin,B^2}
 \to\mathcal R_{\rm lin,B}^{(2)}.
\]

The completeness constraint is Eq. (28):

\[
 \gamma L(d'+1+B+dk)\le B^2.
\]

Theorem 5.1 requires `|S̄|≥2^λ` and **(2B²,S̄−S̄)-relaxed binding**. Since each denominator has operator norm ≤2γ, the simple reduction in Definition 2.3 prices an infinity-SIS radius **8γB²** at the corresponding commitment width. This is a sufficient coarse radius from its declared premise, not an Akita-certified parameter profile. The decomposition `f=F^(0)+B F^(1)` is checked through commitments and evaluation values and has **zero knowledge error** (Lemma 5.1). The L-input error is at most L times the single-input transform error (Theorem 5.2). Remark 5.2 allows delayed refresh when the actual fold norm is much smaller than B².

For workload arity a fresh inputs, use `L=a+2`; selecting a=2/4/8 is not simply choosing a commitment tree fan-out. It changes the norm envelope, total helper witnesses, extraction calls, and SIS premise. Theorem 5.3 estimates dominant prover work `L n κ` ring multiplications plus `O(L n κ d k)` ring additions; verifier work excluding hashing is `O(L d k)` ring multiplications. It reports two accumulated instances, not a single magically compressed relation. Its ∼100 KB example is **a folding proof estimate**, not an end-to-end final SNARK or Akita PCS measurement.

There is no independent concrete implementation in the paper: line 1230 expressly leaves that for future work. Its formal definitions and theorems are interactive RoKs. References to Fiat–Shamir circuit costs do not supply a version-specific ROM knowledge theorem for a new native Akita transplant. The final decider must prove/check the two final committed linear relations and the required folding/public-input linkage, optionally inside another proof system. Neither an unverified final witness nor a commitment-only receipt is sufficient.

Appendix B handles small q with `K=F_{q^t}` and the tensor ring `K⊗_{F_q}R_q`. Column interpretation gives coefficientwise field sum-checks; row interpretation gives t ring inner products sharing scalar coefficient weights and hence permits ring-challenge folding. These are controlled scalar/module operations, not an arbitrary ring–field multiplicative homomorphism. §6 leaves composite moduli and a general ℓ2 variant for future work.

## 5. Cyclo: what “partial” actually means

### 5.1 Principal relation and generalized cyclotomics

Cyclo uses the ring of integers of a cyclotomic field,

\[
 R=\mathbb Z[X]/\Phi_f(X),\qquad q\nmid f,
 \qquad R_q=R/qR,
\]

with specified coefficient/powerful basis. For extension-field checks it defines

\[
 R_{q^e}=\mathbb F_{q^e}[X]/\Phi_f(X),
\]

not the integer ring modulo q^e. When e is the order of q modulo f, Lemma 1 gives `R_q≅(F_{q^e})^{φ/e}` and `R_{q^e}≅(F_{q^e})^φ`.

The principal linear relation Eq. (6) is the bounded solution to

\[
 \begin{pmatrix}
 A\\
 \operatorname{tensor}(r_0)^T M_0\\
 \vdots\\
 \operatorname{tensor}(r_{k-1})^T M_{k-1}\\
 \operatorname{tensor}(b_0)^T\\
 \vdots\\
 \operatorname{tensor}(b_{n-1})^T
 \end{pmatrix}w=y,
 \qquad\|w\|_\infty\le B.
\]

A is the random SIS map; the fixed M_i and r_i express linearized constraints; b_i are delayed evaluation/auxiliary constraints. Appendix A.1's slacked relation applies these equations to `w/s` but bounds w and a unit s, **not w/s**. Appendix A.2 defines the nonzero bounded SIS kernel relation. Appendix A.3 defines interactive RoK and its error.

Lemma 2 requires unramified q and a dual basis modulo q with `Tr(b_i b_j^∨)=δ_ij`. Consequently

\[
 \operatorname{Tr}\langle\operatorname{cf}_{\vee}^{-1}(a),w\rangle
 =\langle a,\operatorname{cf}(w)\rangle.
\]

In the power-of-two negacyclic case, the correct dual of `X^j` is `d^{-1}X^{-j}`; for j>0 this is `−d^{-1}X^{d-j}`. The extracted text's explicit positive `d^{-1}X^{d-j}` formula in §3 would give trace −1 rather than +1 on the diagonal. The defining trace-duality condition, not this unchecked sign formula, must govern implementation. Also Figure 1 attributes the dual-basis identity to Lemma 1 although the relevant result is Lemma 2. These issues matter if reusing the formulation beyond its overview constant-term identity.

### 5.2 Extension commitment and range check

Fresh witness w can be decomposed vertically to one longer witness v, then recommitted under a different random map R:

\[
 w=Gv,\qquad t=Rv,\qquad
 G=((2b)^0I,\ldots,(2b)^{\ell-1}I),
 \quad\ell=\lceil\log_{2b}(2B)\rceil.
\]

A randomly row-batched equality `c^T A Gv=c^T y` links the new short witness to the old instance, alongside transformed M_i/b_i checks. Theorem 2 claims completeness and a RoK back to the original B-bounded witness or an R-SIS solution of norm 2b, with error `ceil(log a)/|C|`. Its cost is `a′mℓ` ring multiplications and a′ ring elements transmitted.

**Base/endpoints are not consistent in the source:** Figure 2 Step 1 and the proof recompose with b^i, while transformed rows, ℓ, and the overview use (2b)^i. The overview sometimes uses `<b`; Figure 2 permits `≤b`; Theorem 2 stipulates b≥2, while Appendix C benchmarks b=1. A transplant must choose a single complete decomposition convention and prove its recomposition, digit endpoints, and output B bound. If digits are merely in [−b,b], the geometric envelope is

\[
 \|Gv\|_\infty\le b\sum_{i=0}^{\ell-1}(2b)^i,
\]

which is not automatically ≤B for `ell=ceil(log_{2b}(2B))`. Thus the asserted exact input B relation needs an endpoint/canonical-decomposition constraint or a correctly enlarged bound. A projection equality by itself does not repair the norm implication.

The range test itself is the familiar **all-coefficient** polynomial check for the **fresh** v:

\[
 f=\operatorname{MLE}[\operatorname{cf}(v)],\quad
 \widehat f(X)=\operatorname{eq}(X,\eta)
       \prod_{j=-b}^{b}(f(X)-j).
\]

It checks `Σ_Boolean widehat f=0`, reduces to a random evaluation by sum-check over F_{q^e}, and carries the full ring inner product obligation `\tilde t=<cf_∨^{-1}(tensor(u)),v>` forward. The trace of that inner product gives the authenticated coefficient evaluation. Theorem 1 states error `ell(2b+2)/q^e`, and extraction outputs an actual b-bounded input witness **or a bounded SIS kernel** from inconsistent slacked openings. A meaningful coefficient range needs an unambiguous centered alphabet (in particular 2b<q for distinct signed representatives), not merely the paper proof's weaker phrase b<q.

### 5.3 Sequential accumulation and the two norm directions

There is one distinguished accumulator v, with coefficient exactly **1**, and L fresh, fully range-tested v_j:

\[
 v_o=v+\sum_{j=0}^{L-1}s_jv_j,\qquad s_j\in D.
\]

Completeness gives

\[
 \|v_o\|_\infty\le\beta+Lb\gamma.
\]

After fresh witnesses have been extracted with their actual b bound, reconstruct

\[
 v=v_o-\sum_j s_jv_j,
\]

giving an extraction-direction bound `hatβ+Lbγ`. This is why omitting the accumulator range check can work. Every fresh vector is nevertheless range-checked in **all** coordinates. “Partial range checks” means fresh versus accumulated, not checking a random fraction of fresh digits or accepting an arbitrary full supplied tree.

For T rounds with total N fresh witnesses, a zero-start honest accumulator has bound at most `N b γ`. Working backward may add another `N b γ`, so a conservative bound for extracted intermediate accumulators is `2N b γ`. Every binding/SIS reduction must be priced at its own intermediate bound. Setting a large depth limit is not a substitute for checking those widths/radii and no-wrap conditions.

This is intrinsically asymmetric. Multiplying several already-grown child accumulators by fresh challenges instead gives something like `Σ_j γ β_j`, not the stated additive-fresh-input recurrence. §2.5 explicitly leaves merging accumulators/PCD to another protocol such as LatticeFold+, which can also refresh the norm. A balanced arity-8 accumulator tree therefore does not inherit Cyclo's sequential theorem merely by relabeling nodes.

### 5.4 Arity extraction bound and concrete source issues

Theorem 3 states knowledge error

\[
 \kappa\le\frac L{|D|}+\frac{\ell_0+\ell_1}{q^e}
 +\frac{L\ell_1(2b+2)}{q^e}
 +\frac{L\ell_C}{|C|}+L\kappa_{\rm nu},
\]

where

\[
 \ell_0=\lceil\log(\varphi(2+k+L(2+n+k)))\rceil,
 \quad\ell_1=\lceil\log(m\varphi\log_{2b}(2B))\rceil,
 \quad\ell_C=\lceil\log a\rceil.
\]

It also introduces

\[
 \bar\beta=\hat\beta(2\gamma)^L+2L\hat\beta(2\gamma)^{L-1},
 \qquad\delta=(2\gamma)^L,
\]

and an R-SIS-break radius **2 barβ δ**. Consequently high arity still has a steep extraction/SIS cost, despite linear completeness-direction growth. For L fresh inputs, the displayed radius is

\[
 2\hat\beta(2\gamma)^{2L}
 +4L\hat\beta(2\gamma)^{2L-1}.
\]

Arity 2/4/8 is not free under this theorem; even if a tighter reconstruction avoids storing a giant denominator, the published reduction currently prices this expression.

Additional proof disagreements, confirmed in PDF text:

1. Lemma 4's ROM sketch says resampling one challenge coordinate preserves acceptance except with probability `1/|D|`. That statement is false for an arbitrary malicious prover. A prover may accept iff the coordinate lies in a half-sized subset: conditional resampling fails with probability about 1/2, not 1/|D|. Expected-time retry on accepting fibers can address an interactive special-soundness problem, as in LatticeFold/LatticeFold+; domain-separated RO tags alone do not prove the ROM reduction or its loss.
2. Theorem 3 proof reconstructs `v*/Δ* = vhat^(L)−Σ s_i^(L) vhat_i*/Δ_i`, but its displayed numerator has a **plus** sign and omits the s_i multipliers. Its displayed slack relation uses `(2γ)^{L−1}` although `Δ*=ΠΔ_i` has norm `(2γ)^L`.
3. The later reconstruction is displayed as `w*=v*−Σw_i*`, again without challenges and without distinguishing numerator from actual divided accumulator. The algebraically correct reconstruction after restoring actual bounded fresh witnesses is the equation in §5.3 above.
4. The range-error factor is 2b+2 in Theorem 1 and Theorem 3, 2b+1 in extraction stage (c), and b+1 in the final displayed union bound. The conservative statement is the larger explicit Theorem 3 error, but source agreement is required before admission.
5. A fresh unit-probability bound for independent samples does not automatically remain the same after conditioning on accepting fibers. If an approximate family is used, the extractor/ROM proof must account for that conditioning.

These findings do not assert an attack on every possible corrected Cyclo construction. They do mean **the collected theorem/sketch must not be treated verbatim as a completed classical-ROM native Akita security proof**. Correcting an equation in Rust is not independently reviewing the repaired theorem.

### 5.5 Exact versus heuristic cyclotomic challenge families

Appendix B.1 specializes LS18. For power-of-two φ, factor count t, and `q≡2t+1 mod 4t`, uniform challenges with

\[
 \|c\|_\infty\le\beta<\frac{q^{1/t}}{2\sqrt t}
\]

form an exact strong sampling set (Lemma 7). For general cyclotomics, Theorem 6 takes `f=Πp_i^{e_i}` and `z=Πp_i^{f_i}`, `1≤f_i≤e_i`, with `q≡1 mod z` and order `ord_f(q)=f/z`, and claims the irreducible factorization `Φ_f(X)=Π_{j<φ(z)}(X^{f/z}−r_j)`. Its stated infinity bound is `β<q^{1/φ(z)}/(2s_1(z))`; the basis-dependent multiplication constant γ∞ then gives operator norm βγ∞. **Text qualification:** Theorem 6 actually prints `ord_m(q)` with undefined m (in both qmd and PDF extraction); Lemma 8 states the order modulo f in words. The displayed Euclidean bound also contains unexplained `q^{φ(m)}`. Use the unambiguous power-of-two Corollary 1 for a native profile, or independently verify LS18's original general theorem and basis constants before relying on the generalized formulas.

Appendix B.2 **labels Lemma 9 heuristic**. It uses coefficientwise ternary sampling and expects nonunit difference probability approximately `t/q^{φ/t}`. The benchmark uses φ=128 and q≈2^50, splitting to quadratic extension factors: t=64. The exact LS18 cube bound is then far below 1, so ternary coefficient differences are **not** justified by the exact theorem in that profile. The heuristic probability is around `64/q²≈2^−94`; this is not a standard-assumption proof of pairwise invertibility. Choosing partial splitting down to degree 8, with t=16, can support exact ternary differences for a sufficiently large 50-bit q, at an arithmetic cost. For an approved standard-assumption protocol, use an actual exact family (or prove a statistical replacement, including accepting-fiber conditioning); do not silently adopt the quadratic-splitting benchmark family.

### 5.6 Hybrid finite-field encoding: module linearity only

Section 7 defines

\[
 \theta_k\Big(\sum_{j=0}^{\varphi-1}a_jX^j\Big)=\sum_j a_j k^j.
\]

This is a well-defined F_q-linear map on the **canonical degree-<φ coefficient representation** (Lemma 5). It is generally **not a ring homomorphism**. It descends from polynomial evaluation to the quotient ring as a multiplicative map only if `Φ_f(k)=0 mod q`. Remark 4 gives the sufficient special case q=Φ_f(k), but explicitly says it is not needed.

Example: in `F_17[X]/(X^4+1)` with k=3, θ_3(X³X)=θ_3(−1)=−1, but θ_3(X³)θ_3(X)=3⁴=13 mod17. Thus θ_3 is not multiplicative in this perfectly valid coefficient representation.

Encode each field element c by its ordinary base-k digits as a polynomial p_c with coefficients in [0,k−1], using `ceil(log_k q)` coefficients and enough ring degree. Then θ_k(p_c)=c. The source's overview and §7.1 have indexing differences; implementation must fix digit count explicitly. The hybrid R1CS constraint is

\[
 (M_0\theta_k(z))\circ(M_1\theta_k(z))=M_2\theta_k(z),
 \qquad Az=y,\quad\|z\|_\infty\le k.
\]

The nonlinear arithmetic stays over F_q. Sum-check over F_{q^e} yields linear ring inner products because the public matrix/equality weights are **field scalars** and θ commutes with those scalar combinations (Lemma 6), not because it preserves arbitrary ring products. Theorem 5 adds a separate public-prefix evaluation obligation. Its stated error is `(log ell+1)/|C|+4log m/q^e`; the statement's direction/types and degree accounting need care before a new theorem uses them. CCS extension is said to be straightforward but omitted in this paper (§7.2), so full generic CCS/zkVM integration is not supplied as explicit pseudocode.

### 5.7 Decider and measurement evidence

Cyclo is a folding RoK, not a complete final independently verified compressed proof system. One may disclose and directly check the final bounded linear witness together with the transcript, or use a separately justified decider/compressor. For IVC one must also bind the step count, public inputs/outputs, and recursive verifier/manifest. The final accumulator alone does not certify unrelated source commitments or chronology.

Appendix C benchmarks **ring arithmetic components / estimated commitment work**, not a full prover/verifier pipeline. Its Table 4's 129.4 s versus 36.7 s comparison estimates double commitment versus extension commitment at matched arithmetic settings. The paper explicitly excludes full-protocol measurements and treats sum-check costs as preliminary estimates. The ∼31.8 KB figures are folding-message estimates with a finite-depth parameter choice and heuristic challenge profile, not a measured final Akita receipt. No parity claim against Akita follows.

At pin `31158a781c16a967c714c7fbde31ae1558e23118`:

- [`README.md`](https://github.com/osdnk/cyclo/blob/31158a781c16a967c714c7fbde31ae1558e23118/README.md) explicitly describes ring-arithmetic evaluation and HEXL bindings, not a complete folding implementation.
- [`benches/bench.rs`](https://github.com/osdnk/cyclo/blob/31158a781c16a967c714c7fbde31ae1558e23118/benches/bench.rs) fixes `N=128`, `MOD_Q=1125899906839937`, and separately measures incomplete NTT, addition without modular reduction, NTT multiplication, and reduction. Its Criterion sample size is 10, warm-up 30 s.
- [`src/cyclotomic_ring.rs`](https://github.com/osdnk/cyclo/blob/31158a781c16a967c714c7fbde31ae1558e23118/src/cyclotomic_ring.rs), inspected through the incomplete-NTT multiply, splits even/odd coefficients, applies two half-size transforms, and multiplies quadratic factors using shift factors. The source/tree and benchmarks do not provide range/fold transcript or final decider algorithms.
- [`invertibility.ipynb`](https://github.com/osdnk/cyclo/blob/31158a781c16a967c714c7fbde31ae1558e23118/invertibility.ipynb) and [`estimates.ipynb`](https://github.com/osdnk/cyclo/blob/31158a781c16a967c714c7fbde31ae1558e23118/estimates.ipynb) are the paper's referenced experimental/parameter artifacts. Their filenames/pins were confirmed; the notebooks were not independently audited or executed in this slice.

## 6. Transplant into the original Akita A/B/F graph

### 6.1 The nonhomomorphism is a concrete algebraic obstacle

For one block or segment, the original mechanism is schematically

\[
 t=A s,\qquad\widehat t=D_{b_1}(t),\qquad
 u=B\widehat t,
\]

possibly with sliced B images, then negative-binary compression

\[
 y_1=F_1\xi_0,\qquad G_2\xi_0=u,\qquad
 p=F_2\xi_1,\qquad G_2\xi_1=y_1,
\]

where every coefficient of ξ_0 and ξ_1 lies in `{−1,0}`. Akita §2.7 / Eq. (76) describes the analogous H chain for opening images; both chains' digits are carried into the recursive witness. The public root payload is p, not A s.

Digit decomposition is not additive. Even ordinary binary illustrates it: `D(1)=(1,0,...)`, whereas `D(2)=(0,1,...)≠2D(1)`. In Akita's negative binary, `D(−1)=(-1,0,...)` but a valid representation of −2 is `(0,-1,...)`, not `(-2,0,...)`. Thus

\[
 B D(A(s+s'))\ne B D(As)+B D(As')
\]

as a general construction identity, and the F chain introduces the same issue again. Summing public p values is linear in their **last digit openings**, not homomorphic in source s with its original chain and alphabets. Any accidental equality for a particular setup/input is irrelevant to the required universal folding law.

LatticeFold's `\mathcal L` cannot be instantiated with this composition. LatticeFold+'s own §4.4 explicitly diagnoses the analogous double-commitment obstruction. Its transformation has a plausible *structural analogy* to one Akita chain, but adapting it requires a proof for all A/B/F/H maps, complete image widths, slices, repackings, and intermediate digits. One cannot cite its single-com scheme as if that adaptation had already been proved.

### 6.2 Ring dimension, convolution, and packing constraints

The current Akita source is **negacyclic**, using `F_q[X]/(X^d+1)`, so there is no reason to manufacture a cyclic-to-cyclotomic equivalence. However, Akita may use distinct A/B/D/F/H ring dimensions. Coefficient repacking between them is F_q-linear, not automatically an R_q-module homomorphism for a common short challenge ring. A field-linear reordering alone does not permit arbitrary ring-challenge multiplication to commute through the graph.

Akita's direct packing method is only linear over its declared **challenge subring**: its overview has `L(ι(c)F)=c L(F)` for the selected embedding ι (Lemma 3.2 / §2.5). A transplant must retain that subring action and its challenge entropy/difference conditions. Full A-ring challenges cannot be imported just because all objects are stored as field coefficients.

Changing `X^d+1` to `X^d−1` would break LF+'s monomial/sign lookup and the cited cyclotomic SIS/LS18 argument. A cyclic ring even has elementary zero divisors such as `(X−1)(1+X+...+X^{d−1})=0` in the quotient. Likewise evaluating at an arbitrary field α is not multiplicative on a negacyclic quotient. Akita §2.6 handles this through a quotient witness or transpose-convolution weights

\[
 \kappa_{a,\alpha}^{(d)}(j)=
 (a(X)X^j\bmod(X^d+1))(\alpha).
\]

Those exact reduced weights are a compatible way to turn each native ring equation into a coefficient-field linear relation. Replacing them with `a(α)α^j` without the quotient/correction proves a different statement.

### 6.3 Explicit compatible lifted relation fragment

A conservative native construction fragment does **not** fold p as if it were a source-linear commitment. For each fresh source k, keep all native digit witnesses and intermediate images. For example let

\[
 W_k=(s_k,\widehat t_k,\xi_{0,k},\xi_{1,k},\ldots),
\]

and retain the exact graph equations

\[
 A s_k=G_{b_1}\widehat t_k,\qquad
 B\widehat t_k=G_2\xi_{0,k},\qquad
 F_1\xi_{0,k}=G_2\xi_{1,k},\qquad
 F_2\xi_{1,k}=p_k.
\]

Use the actual sliced B maps, H/opening chains, ring dimensions, padding, and source/public statement in place of the schematic ellipsis. The final decider must enforce:

1. Every source digit coefficient lies in its exact alphabet, with all source coordinates covered.
2. Every inner-image, outer-image, response, quotient (if used), and compression digit lies in its assigned alphabet. In particular every binary compression digit satisfies `x(x+1)=0`.
3. Every native graph equation, using reduction modulo its own `X^{d_j}+1` and the actual setup map/slice.
4. All scalar evaluation/public-prefix equations and any surrounding Boolean/one-hot/domain constraints. Akita PCS binding alone does not establish source one-hotness (Akita Remark 4.1).
5. Segment identity/order/count, public transition consistency, setup/schedule identity, and source commitment linkage in the statement manifest.

The ring-equation rows may be **linearly accumulated as coefficient-field residual obligations**, provided the constituent witness values are fixed/authenticated before the batching challenges. Define each row residual `e_{k,j}(W_k)` using the correct native maps. Then after those values are fixed, an ordinary field randomization of `Σ_{k,j} λ_{k,j}e_{k,j}` is valid algebraic batching. Range/domain residuals use their actual polynomials, not norms of inversely extracted blocks. Authenticating every needed witness evaluation remains an obligation; a scalar batch equation by itself is not a new PCS.

There is an especially simple **fully deterministic final-certificate specialization** allowed by the large-certificate goal: disclose every W_k and directly check every equation/alphabet/domain/manifest condition in one final independent verifier invocation. Extraction is then reading the explicit source witnesses; no ROM extraction, concrete-hash knowledge preservation, short inverse, or folding homomorphism is needed for these checks. The native root payloads still bind the disclosed sources through the complete A/B/F/H graph under the ordinary per-map bounded-input SIS assumptions. This is a compatible fragment and conservative reference decider, not a claimed short-certificate folding theorem or a claim of privacy/performance parity. A future compact variant must prove that its openings authenticate exactly this complete relation.

### 6.4 Why partial checks do not suffice for the supplied-tree prototype

To import Cyclo's omission of accumulated range checks, one would need an inductively proved accumulated relation whose *only* enlarged witness is `v_acc`, added with coefficient 1. Every fresh input must first become an actually range-bounded witness to that same compatible linear relation. Akita's compressed root commitment does not provide such a witness relation by itself.

Checking only the final accumulated response or its short numerator leaves these gaps:

- Divided source witnesses can have arbitrarily large centered coefficients.
- Source-to-A, A-to-B digit, and B-to-F digit links can be omitted or inconsistent.
- Legal final binary digits do not bound the hidden source witness; recomposed images are full-field values.
- Source one-hot/domain claims are not consequences of PCS opening correctness.
- A supplied tree may contain many hidden leaves which were never covered by an authenticated all-source range check or by a proved inductive relation.

Consequently “accepts two frames,” “all final response digits are small,” or “root payload matches” are not sufficient theorem prerequisites. The existing native prototype must not be treated as an approved baseline theorem on that evidence.

## 7. Selection guidance and exact prerequisites for parent integration

Only three published families were considered in this slice; the explicit native lifted relation above is a construction fragment, not an additional complete protocol candidate.

| Family | Useful transplant ingredient | What must be proved before native admission |
|---|---|---|
| LatticeFold | Batched exact source coefficient range checks; stabilization of divided openings; decomposition/fold norm accounting | Native graph-to-common-linear-relation reduction; complete all-source bounds; per-map relaxed binding; ROM knowledge compilation (explicitly absent in collected version) |
| LatticeFold+ | Real nonhomomorphic double-commitment transformation; two-point sum-check; bounded refresh to two accumulators | Transformation for actual heterogeneous A/B/F/H/sliced graph, not substitute com; every helper's width/radius/range; challenge-subring compatibility; ROM extraction; final two-relation decider |
| Cyclo | Keep grown accumulator coefficient 1; range-check every fresh witness; delay refresh under finite-depth bound | Corrected base/endpoint/sign/denominator/error statements; exact challenge family or independently proved statistical family; accepting-fiber/ROM extraction; full-source native linkage; nonsequential merges handled separately |

For any selected native construction, the admissibility sheet should contain at least:

- Exact source relation (including domain/one-hot if claimed), witness layout, setup/schedule/ring identity, and manifest binding.
- A typed action for each challenge on each source/commitment/packing object, with explicit commuting diagrams or reduced coefficient weights.
- Exact range coverage set, including every source leaf and every compression digit, not only the final response.
- Forward and backward norm recurrences, maximum depth, and arity convention (total inputs versus fresh inputs).
- Each SIS instance's full column count/ring rank and collision bound; short numerator/denominator accounting without short-inverse inference.
- Exact unit-difference theorem and challenge support entropy after any filter; classical-ROM transcript and extraction theorem with losses.
- A concrete final verifier/decider for the whole accumulated relation. An optional SNARK/Akita wrapper must state its additional soundness assumptions and what it proves.
- End-to-end measurements on real upstream Akita workloads at 1/2/3/256 segments. Neither LF+'s anticipated performance nor Cyclo's arithmetic-only benchmark supplies those numbers.

No implementation or verification commands were exercised in this slice. Parent should review the exact native graph/parameter obligations and independently review any repaired ROM/norm theorem before selecting a compact Rust protocol. The clear all-source native decider is the sound comparison point if no compact transplant meets these prerequisites.
