# Reusing Jolt Booleanity without leaving physical source holes

Status: independently reviewed coverage design for the selected compact profile. Its physical predicates are now implemented but execution and composed numerical admission remain pending. The construction follows the [complete frontend descent audit](AKITA_INSTANCE_ACCUMULATION_FRONTEND_DESCENT.md). It can replace one accumulation-only full-Main Boolean prefix after joint source-linked extraction is verified; it does not delete existing frontend Booleanity, digit-zero or alias checks. Other physical geometries need the stated coverage certificate.

Implementation status: the paired producer/verifier modules now exist as `jolt-prover::akita::segment_source` and `jolt-verifier::segment_source`. They reuse the stock degree-two sumcheck driver and return three original-Main opening batches: all columns for physical zeros, bytecode columns for each PC endpoint. The producer borrows the exact stage-zero `TraceOneHotRows` owner until the first bind; it retains only endpoint scalars in the completed proof. A scoped independent review found no remaining algebra defect after fixing trace-sized terminal allocation retention. Compilation and real transcript execution are still in progress. The generic native Main Boolean prefix has **not** been removed.

For compact `(mu,P,k,bB)=(16,56,4,3)`, these three checks contribute the conservative common-K numerator
\[
(3\mu+P)+2(3k+b_B)=134.
\]
They transmit `2mu+P+2(2k+bB)=110` K elements, or 1,760 unframed bytes. These are grammar counts, not a full composed security certificate or measured runtime. The common frontend charge applies to both accumulation and its matched continued control.

## 1. Actual obstruction

For instruction RA, bytecode RA, balanced digit and carry columns, the native semantic table is
\[
R_p(a,t)=
\begin{cases}
X_p(a,t),&a\ne0,\\
1-\sum_{a'\ne0}X_p(a',t),&a=0.
\end{cases}
\tag{1}
\]
RAM RA columns instead use their full physical X table. The existing Booleanity argument proves `R_p(R_p-1)=0` on the complete address/cycle cube, once its full source-linked extraction tree is available.

It therefore proves physical Booleanity at every nonzero address of the zero-omitting columns and every address of the RAM columns. It **does not observe** physical X_p(0,t) for zero-omitting columns. Indeed
\[
\widetilde R_p(r,t)=\operatorname{eq}(r,0)+
\sum_a[\operatorname{eq}(r,a)-\operatorname{eq}(r,0)]X_p(a,t),
\]
and the coefficient at a=0 vanishes identically. Changing one such physical value to 2 preserves every upstream semantic/Booleanity identity. This is an actual physical-map kernel, not a PCS collision or an extension-field artifact.

A sparse honest producer and its `{0,1}` bound do not prove the missing physical predicate.

## 2. Explicit Jolt main-role domain

For the newly provisioned Jolt main role, register:

1. Exactly P original native columns, each containing NK direct scale-one, one-digit coefficients. The logical-to-physical map is a bijection onto those actual coefficients. Any different packing with additional physical positions needs explicit coverage; it cannot inherit this certificate.
2. Physical address-zero entries of the zero-omitting families are **required zero**, matching the native column definition. RAM address-zero entries are not included in this requirement.
3. All physical entries on cycles t>=ell are required zero, matching the canonical inactive trace encoding.
4. Every other physical entry must be 0 or 1, certified by the existing complete semantic Booleanity and digit-zero reconstruction trees.

This is a declared input-domain predicate, not an unstated assumption or a verifier-only restriction. Strong completeness is for every source/path satisfying this main-role relation; noncanonical inner-image/F paths remain allowed. An arbitrary dense Boolean role that allows nonzero omitted cells can keep the generic full raw Boolean prefix instead. Do not remove that prefix for such a different role.

No new constraint on the **whole-run execution language** is introduced: the registered Jolt trace column definition already emits zero at these omitted addresses, and replacing ignored cells by zero leaves the semantic tables unchanged. The public execution statement does not fix an arbitrary pre-existing main commitment. This does not authorize changing an already formed commitment, or claiming completeness for a broader raw-source relation while silently imposing these zeros.

## 3. Strengthen the existing inactive-zero check

For each column p let O_p be its public zero-omission flag. On the Boolean `(cycle,address)` domain define
\[
Z_p(t,a)=1_{t\ge\ell}\lor(O_p\land1_{a=0}).
\]
Bind the original main handle, exact layout, ell and omission flags before sampling the full point tau and independent coefficients alpha_p. Define the restricted-weight MLE
\[
W_p(X)=\widetilde{[\operatorname{eq}(\tau,x)Z_p(x)]}(X).
\]
Prove the degree-two identity
\[
0=\sum_x\sum_{p<P}\alpha_p W_p(x)\widetilde X_p(x).
\tag{2}
\]
At its endpoint r, check `sum_p alpha_p W_p(r) v_p`, leaving exactly P ordinary openings `X_p(r)=v_p` on the original main group. Because masks vary by column, this is **not** one common masked polynomial times a combined source. Do not multiply separately interpolated masks and equality polynomials away from the cube.

The previous inactive-only check already had the same P leaves, degree two and point arity mu_X=log2(NK). Enlarging its registered masks to Z_p adds no challenge or leaf. Its charge remains `3mu_X+P`, and tree factor remains `2^mu_X*(P+1)*3^mu_X`.

### Exact public weight evaluation

Let
\[
E_t=\sum_t\operatorname{eq}(\tau_t,t)\operatorname{eq}(r_t,t),\quad
E_a=\sum_a\operatorname{eq}(\tau_a,a)\operatorname{eq}(r_a,a),
\]
\[
S_t=\sum_{t\ge\ell}\operatorname{eq}(\tau_t,t)\operatorname{eq}(r_t,t),\quad
A_0=\operatorname{eq}(\tau_a,0)\operatorname{eq}(r_a,0).
\]
Then the exact restricted-table MLE is
\[
W_p(r)=S_tE_a+O_p(E_t-S_t)A_0.
\tag{3}
\]
The full E kernels factor coordinatewise. S_t is the original public suffix-weight sum and can use a correct dyadic-interval/bit-prefix evaluator. Equation (3) follows from disjointly partitioning inactive cycles and active address-zero holes; it is not an off-cube indicator-product shortcut.

## 4. Extraction and reuse

First authenticate every source leaf through the native descendants and terminal. Same-original-handle source uniqueness fixes one physical K source across the complete frontend and zero-check trees, or produces its priced collision.

Interpolate (2), the independent alpha family and the complete tau family to force each registered zero. Separately, the complete original Booleanity/digit-zero trees give physical 0/1 values on all remaining positions by (1). Since these cases cover every actual coefficient in the registered direct main map, the original source has coefficient bound one. There is no challenge-inverse norm inference and no reliance on honest occupancy.

This supplies the same bounded-source conclusion as the full main raw Boolean prefix, now from an already required frontend plus the strengthened zero masks. The prefix can therefore be removed for this exact Jolt role in the jointly extracted frontend-plus-native update. This is not a standalone local statement about an arbitrary old accepting Booleanity transcript; all ancestor reference families and later branch-specific continuations are retained.

The generic old receiver and fresh gadget RAM sources keep their same-handle domain prefixes. No analogous inference is made from a bound on a gadget-decoded RAM value.

## 5. Effect on the compact example

Eq. (109) storage, source/matrix roles, response bounds, one-channel partials and the 3456 coefficient rows do not change. The main source-domain conclusion comes from the revised frontend instead of another raw-prefix endpoint.

The main multipoint list loses that extra endpoint: at most 118 instead of 119 claims. Its original-column Boolean queries still convert through the digit-axis lemma and normalize on the same raw view; P remains 56 and no new commitment is formed.

Thus the separately listed prefix/normalizer/native numerator changes from `1039+299+370=1708` to
\[
(1039-88)+(299-1)+370=1619.
\]
The expanded zero-check mask has the same original frontend charge. The global conservative catalogue ceiling can retain the generic 1708-style overcount. This is an algebraic/proof-work reduction, not a measured speedup or parity result.

An exact F_101 model checked the restricted-weight identity for 700 cases spanning every proper nonempty active prefix of eight cycles, both column classes and non-Boolean evaluation points. The explicit physical source `(2,0,0,0)` still reconstructs the Boolean semantic table `(1,0,0,0)` when address zero is omitted; the new zero predicate detects residual 2. This checks the algebra and the gap, not a native proof implementation.

## References and verification limits

- [Immutable frontend reconstruction and hole counterexample](AKITA_INSTANCE_ACCUMULATION_FRONTEND_DESCENT.md), §§3, 5 and 6.
- [Native digit-zero relation](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/lattice/relations/digit_zero.rs).
- [Native Booleanity relation](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/lattice/relations/booleanity.rs).
- [Native column layout](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/lattice/strategy.rs).
- [Global manifest](AKITA_INSTANCE_ACCUMULATION_GLOBAL_MANIFEST.md), [compact profile](AKITA_INSTANCE_ACCUMULATION_COMPACT_PROFILE.md).

This note does not assert that current source already enforces the new zero mask, that arbitrary external main handles satisfy it, or that native mixed-K/E execution has been implemented.
