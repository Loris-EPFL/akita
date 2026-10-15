# Lova obligations for Akita relation accumulation

## Decision

Lova does not provide a drop-in Akita accumulator. Its exact integer Gram relation is useful for norm refresh and source-domain checks. Its opening-fold theorem requires a fixed linear commitment map, whereas Akita's full A/B/F compressed commitment is not such a map. Lova also explicitly leaves general R1CS folding as future work. The current proposal therefore does not substitute Lova's Ajtai commitment for Akita.

## Reading coverage and provenance

Read through qmd MCP: `lattice-papers/lova.md`, document `#a5e893`, all extracted text through line 680, including references. Main ranges: 1–200, 201–420, 319–378, 379, 421–550, 551–680. Large base64 payloads caused middle elision: figure text was separately recovered from `Papers/lova.pdf`, extracted lines 249–253 (Figure 1) and 303–330 (Figure 2); surrounding proofs at 274–415 were also read. No separate raster/geometry inspection was performed. Some qmd/PDF formulas contain transcription errors, so the equations below are dimension-checked reconstructions, not verbatim OCR.

Primary implementation link supplied by the paper: [lattirust/lova](https://github.com/lattirust/lova). No Lova implementation was executed in this slice.

## 1. The relation, not only a commitment equation

For a fixed matrix $A\in\mathbb Z_q^{n\times m}$, Lova uses

$$
\mathcal R_{q,\beta,t}=\{((A,T,D),S):\ AS=T\pmod q,\ D=S^\top S\text{ over }\mathbb Z,\ D_{ii}\le\beta^2\}.
$$

Here $S\in\mathbb Z^{m\times t}$. The integer equality is essential: a Gram equation only modulo $q$ does not establish the same Euclidean norm. The public $D$ is part of the accumulated instance, not a prover-only optimization. Final decision must check both the opening and the exact Gram relation. [Lova, equation (1), qmd 173–205, 323–383.]

The two-input concatenation sends the cross Gram blocks and creates

$$
S=[S_0\ S_1],\quad T=[T_0\ T_1],\quad
D=\begin{pmatrix}D_0&S_0^\top S_1\\S_1^\top S_0&D_1\end{pmatrix}.
$$

A concatenation without those blocks does not specify the output relation. [Lemma 3; Figure 1.]

## 2. Refresh and fold

Let $k=\lfloor\log_b\beta\rfloor+2$ and $G=I_{2t}\otimes(1,b,\ldots,b^{k-1})^\top$. Balanced decomposition gives

$$
S=\widetilde S G,\qquad |\widetilde S_{ij}|\le\lfloor b/2\rfloor.
$$

The prover sends $\widetilde T=A\widetilde S\bmod q$ and $\widetilde D=\widetilde S^\top\widetilde S$. A ternary challenge matrix $C\in\{-1,0,1\}^{2kt\times t}$ gives

$$
S'=\widetilde S C,\quad T'=\widetilde T C\bmod q,\quad D'=C^\top\widetilde D C.
$$

The verifier checks $\widetilde T G=T\bmod q$ and $G^\top\widetilde D G=D$. The honest norm bound is

$$
\|S'_{*,j}\|_2\le2kt\lfloor b/2\rfloor\sqrt m\le\beta.
$$

This is a parameter condition, not automatic closure. [Lemma 4; Figure 2; qmd 367–418.]

For an arity $L$ generalization, honest completeness requires at least

$$
Lkt\lfloor b/2\rfloor\sqrt m\le\beta.
$$

Changing $L$ also changes challenge dimensions, transmitted Gram blocks, extraction bounds and lattice estimates. An arity label alone supplies none of these checks.

## 3. Why ternary challenges do not require short modular inverses

Lova's coordinate extractor obtains two accepting matrices differing in one row, with at least one entry difference $\pm1$. It deliberately does **not** extract through an arbitrary difference $\pm2$. This works even for an even modulus. The extracted decomposed column is an integer signed difference of terminal columns:

$$
\overline S_{*,j}=\pm(Z^{(0)}_{*,i}-Z^{(1)}_{*,i}),\qquad\|\overline S_{*,j}\|_2\le2\beta.
$$

The extractor retries accepting fibers; it does not assume that changing one oracle coordinate preserves acceptance. Its relaxed knowledge threshold in Lemma 5 is $\epsilon>4\,2^{t(\delta+2/3)}/3^t$. [qmd 415–479.]

The Gram proof is intended to remove this factor-two slack from the *final extracted input witness*: equality of $\overline S^\top\overline S$ with $\widetilde D$, followed by recomposition, recovers $S^\top S=D$. The polynomial identity test is over the integral domain $\mathbb Z$, not directly over a cyclotomic quotient ring. [Lemma 6; qmd 483–510.]

## 4. Issues that require resolution before reusing the proof

These are review obligations, not a blanket claim that Lova is unsound.

1. **SIS-radius accounting.** Lemma 5 gives each extracted column norm at most $2\beta$, with $2kt$ columns. The straightforward bound for $z=S'_{*,i}-\overline S C_{*,i}$ is
   $$\|z\|_2\le(4kt+1)\beta.$$
   Lemma 6 prints $(2kt+1)\beta$ in both qmd and the PDF extraction. Reusing the smaller radius requires an additional tighter argument; it does not follow from the displayed triangle bound. A conservative transplant must price the larger radius or establish the missing improvement.
2. **Symmetry.** A nonzero antisymmetric matrix has $x^\top\Delta x=0$ identically. The diagonal identity-test argument needs a symmetric Gram-error matrix, or a separate analysis of the off-diagonal tests. Canonical symmetric encoding/checks must be explicit, not inferred from the honest prover.
3. **Challenge independence.** The polynomial being tested must be fixed independently of the fresh challenge. An extracted matrix that itself depends on that challenge needs a uniqueness/reordering argument. Conditioning on accepting fibers must remain in the probability analysis.
4. **Exact integer arithmetic.** Overflow, modular Gram reduction and ambiguous signed encodings would change the relation. Bounds must cover Gram entries and matrix multiplication before selecting integer widths.
5. **Uniform ROM composition.** An interactive RoK and an implementation using a Fiat–Shamir framework do not by themselves establish a polynomial-time extractor for every growing-depth Akita history.

## 5. What can and cannot transfer to Akita

| Ingredient | Transfer condition |
|---|---|
| Exact integer Gram invariant | Preserve its integer semantics, dimensions, symmetry and final decision. It can complement Akita bounds but does not replace the full source/path relation. |
| $\pm1$ extraction | Scalar integer challenges commute with a flattened linear map. A structured Akita map still requires its own Module-SIS distribution and norm profile; it is not uniform unstructured SIS. |
| Decompose-and-fold | Use all original digit/recomposition equations, including signed RAM conventions and source gaps. Recomposition alone is not canonical decomposition. |
| Linear opening fold | Requires an actual fixed linear map. Akita's compressed root cannot simply be placed in the $A$ slot. A lifted relation would include all original A/B/F and D/H equations and bounds. |
| General VM/CCS folding | Not supplied by Lova: qmd 513–525 explicitly leaves general R1CS folding open because cross-term norms depend on arbitrary matrix entries. |
| Variable opening points | The row $\ell_u$ changes with $u$. Folding $A S=T$ does not automatically preserve $\ell_u(S)=v$ for heterogeneous queries. |

Lova's subset-sum sketch uses the integer identity $\sum_j s_j(s_j-1)=0$ to characterize binary integer coefficients. Every summand is nonnegative for an integer $s_j$. That reasoning is invalid if only the equation modulo $q$ is checked without a no-wrap proof. Akita's Main one-hot, signed RAM, decoder and path obligations remain separate. [qmd 513–525.]

## 6. Performance evidence from the paper

The paper uses $q=2^{64}$, ternary challenge matrices and exact integer Gram information; it does not require a finite-field extension. Its printed example selects $t=330$, $k=4$, with an explicit query/security budget. These are Lova parameters, not Akita parameters.

For witness length $2^{17}$, the paper reports a 17.53 MB IVC-type step proof with perfect completeness and 725.35 seconds prover time on its stated AWS machine. These are published Lova results, not local measurements or an Akita parity comparison. The large proof follows from the Gram matrices, not an arbitrary receipt-size limit. [qmd 531–581, Table 2.]

## 7. Consequence for the proposed design

Prefer an Akita-native query-relation reduction which leaves original source commitments and witnesses unchanged, followed by one ordinary Akita batch decider. This avoids requiring compressed-commitment homomorphism, Lova's general R1CS extension, or inverse-shortness. Its new query PIOP and global ROM budget still require proof and independent review. See the new protocol document and the companion [range/folding](AKITA_RELATION_ACCUMULATION_RANGE_FOLDING.md), [tensor/sumcheck](AKITA_RELATION_ACCUMULATION_TENSOR_SUMCHECK.md), and [generic architecture](AKITA_RELATION_ACCUMULATION_GENERIC_ARCHITECTURE.md) notes.
