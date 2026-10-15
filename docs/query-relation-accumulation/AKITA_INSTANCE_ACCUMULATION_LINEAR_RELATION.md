# Exact quotient-free coefficient relation for the native receiver

## 1. Purpose

The receiver counts assume a quotient-free relation. This note defines that relation explicitly; it does not infer it from the current planner's late-stage reduced-evaluation mode. It reuses the original Akita A/B/F and shared D/H equations and full Eq. (109) witness, but checks **all coefficients of their negacyclic equations** instead of adding quotient witnesses to check polynomial evaluations outside the ring.

This is a proposed public-weight/row compiler, not an existing exported API. It leaves commitment formation unchanged and uses no homomorphism of the compressed commitment. The [extension security profile](AKITA_INSTANCE_ACCUMULATION_SECURITY_PROFILE.md) supplies the example dimensions. Setup offloading is absent; public matrix-weight work is explicitly charged.

## 2. Canonical linear relation

Fix the original source/commitment geometries, ordered claims, source-query maps, digit bases/depths, shared D/H payload, fold challenges, and complete successor layout. Let W be the entire base-field Eq. (109) vector before Boolean padding, with response digits, partial digits, original inner-image digits and every F/H input digit. Extend it to its declared N-cell physical identity source. All source and path coordinates keep their declared permutations; a semantic equation never changes the original physical A/B/F view.

Every native ring equation at this point is linear in W. Its source-query and challenge coefficients are public. The equation families are:

1. `A_g G_rsp zhat_g = sum_i c_gi G_open that_gi`, in the original A ring.
2. Each declared carrier channel: `L_g,h(G_rsp zhat_g) = sum_i c_gi G_open ehat_g,h,i`. For an extension carrier, split into its actual base-field coefficient planes.
3. Each original sliced B image equals the value recomposed from its first F input, or its raw original payload when the source has no F chain.
4. Shared D on every declared partial equals the value recomposed from the first H input, or its raw payload.
5. Every F/H stage maps its full original input to the next recomposed image; the final image equals its bound public payload.
6. Every ordinary/raw scalar claim equals its prescribed scalar functional of the appropriate partials. Retain separate per-polynomial ordinary claims when P values are exposed; do not silently replace them by an uncharged random combination.

Gadget maps are linear field recompositions, not assertions of canonical decomposition. Short/binary/zero predicates are handled in §4. Complete valid noncanonical original paths remain legal.

Flatten each equation into the coefficients of its **native** ring, then each extension coordinate into the fixed base-field basis. With deterministic equation/plane/coefficient order, obtain
\[
 M W=b,\qquad M\in K^{R\times N},\quad b\in K^R.
\tag{L}
\]
M is an implicit public linear map, not a committed new matrix or a new SIS key. Its coefficients depend on the already bound native inputs and fold challenges. Embed scalar constants and original payload coefficients in b at their exact declared positions. Unused final W coordinates have zero columns in M; required padding is independently enforced.

For a ring `K[X]/(X^d+1)`, the exact negacyclic multiplication coefficient map is
\[
 (a v)_t=\sum_{j=0}^{d-1}\sigma(t,j)a_{(t-j)\bmod d}v_j,
 \quad
 \sigma(t,j)=\begin{cases}1&j\le t,\\-1&j>t.\end{cases}
\tag{NC}
\]
Apply the declared gadget coefficient before placing the corresponding W column. Formula (NC), the native digit/plane permutations and the public S-linear query maps determine every entry of M. No polynomial quotient is omitted: ring reduction is performed **inside** this exact coefficient map. Changing ring dimensions or a native slice requires its actual descriptor, not a reused larger-ring formula.

## 3. Row reduction without quotient witnesses

Pad the scalar row list by zero rows to `Rbar=2^ceil(log2 R)` and define `kappa=log2 Rbar`. After binding the outgoing original commitment `C_W`, its complete source/layout and the row descriptor, sample `rho in E^kappa`. Define
\[
 \lambda_\rho(x)=\sum_{h<R}\operatorname{eq}(\rho,h)M_{h,x},
 \qquad b_\rho=\sum_{h<R}\operatorname{eq}(\rho,h)b_h
\]
on Boolean physical column indices x, extending absent rows/columns by the declared zeros. Let `lambda_rho~` be the **MLE of this coefficient table**. The reduced relation is
\[
 \sum_{x\in\{0,1\}^\mu}W(x)\lambda_\rho(x)=b_\rho.
\tag{RL}
\]
This is not an off-cube multiplication claim about original ring polynomials. For a fixed original W, interpolating the complete nested rho tree recovers every row of (L). The row-stage field error charge is at most `kappa/|E|` for the full-field coordinate family, and its tree factor is `2^kappa`, as for ordinary multilinear row batching. Original-source consistency across rho branches is required; one accepted sampled equation is not a deterministic proof of all rows.

A stand-alone degree-two sumcheck could reduce (RL) to `W~(r) * lambda_rho~(r)`. Instead, §4 fuses it into the existing degree-three strong endpoint. Verifying the terminal product never divides by `lambda_rho~(r)`.

## 4. Retain native range/binary checks and add required zeros once

Run the original full-grid range product pipeline for the selected common digit alphabet. Its final virtual claim is
\[
 s_{\rm virt}=\sum_x\operatorname{eq}(r_{\rm virt},x)W(x)(W(x)+1).
\]
This is an MLE of Boolean products, not the square of an off-cube MLE. Retain every product-stage message, interstage check and challenge. Let `tau0` be its original full-grid range anchor. Binding C_W precedes rho, tau0 and the subsequent strong-check challenges.

Let `I_bin` be the exact F/H negative-binary spans and `I_zero` every required zero span. Define restricted weights
\[
 B(X)=\widetilde{[1_{I_{\rm bin}}(x)\operatorname{eq}(r_{\rm virt},x)]}(X),
 \quad
 Z(X)=\widetilde{[1_{I_{\rm zero}}(x)\operatorname{eq}(\tau_0,x)]}(X).
\]
Use these restricted-weight MLEs, not products of separate off-cube masks and equality MLEs. With fresh combination scalars gamma, eta_bin, eta_zero in E, prove the fused identity
\[
\begin{aligned}
 b_\rho+\gamma s_{\rm virt}=\sum_x\bigl[&
 \widetilde W(x)\widetilde\lambda_\rho(x)
 +\gamma\operatorname{eq}(r_{\rm virt},x)\widetilde W(x)(\widetilde W(x)+1)\\
 &+\eta_{\rm bin}B(x)\widetilde W(x)(\widetilde W(x)+1)
 +\eta_{\rm zero}Z(x)\widetilde W(x)\bigr].
\end{aligned}
\tag{F}
\]
At a Boolean x each MLE equals its table entry. Each summand has individual degree at most three. The final check substitutes the **same** pending value `v=W~(r)` in all four terms, leaving one ordinary opening of the original C_W. There is no new witness table, partial channel or quotient plane for this row compilation or zero term.

The binary support uses the final range-stage point/tree, as in the native construction. The zero term uses the initial range anchor/tree; no new independent anchor is asserted free. The complete extraction family must retain those original nested trees and the new combination-scalar separation family. Do not reuse an anchor that was chosen before the relevant C_W binding or whose complete interpolation family is absent. Native response norms other than the coefficient-domain route would require their full additional terms; the present example does not select physical-L2 mode.

## 5. Extraction and completeness

**Completeness.** For every supplied valid bounded original source/path, the declared deterministic response envelope gives encodable zhat. All original partial/image/path equations hold, hence (L) and (RL). W uses the common alphabet and exact binary/zero spans. The native range pipeline and (F) therefore have their prescribed targets. The complete receiver layout remains the one already counted; the supplied original noncanonical digits are not replaced by recomputed paths.

**Conditional supplied-tree extraction.** Authenticate each terminal W evaluation through its actual continuation/final decision. Native same-original-source uniqueness identifies one physical W per prechallenge C_W or yields a scheduled collision. Interpolate the fused sumcheck and combination challenges to separate (RL), the virtual range meaning, binary and zero terms. The existing product tree yields the common alphabet; its final/initial anchor trees yield the binary/required-zero predicates. The nested rho tree gives every coefficient row of (L). Thus the exact ring and scalar equations required for native coordinate-fork subtraction hold. That subtraction recovers weak original inputs and both query planes; the original source-domain bridge then restores the bounded inputs, or preceding history range is used under its distinct joint theorem.

This changes public row interpolation and its ledger, not the source-comparison matrices or their bounded response radii. The complete global ROM theorem still needs the declared manifest, samplers, same-handle bridges and final decision; the existence of M alone is not that theorem.

## 6. Exact scalar-row counts for the extension example

For the two-channel profile in the security note:

- Receiver native ring coefficients: `(4 evaluation planes + 5 A + 1 B)*128 = 1280`.
- Main: `(4 + 3 + 1)*128 = 1024`.
- Each RAM endpoint: `(4 + 5 + 1)*128 = 1280`.
- Shared D: 128.
- Four F chains plus H: `5*16 + 5*8 = 120`.

These total **5112** base-field ring coefficients. Keeping every ordinary scalar value and one raw scalar per handle gives `2 + (56+1) + 2 + 2 = 63` E equations, hence **126** base-field scalar rows. Therefore
\[
 R=5238,\quad\bar R=8192,\quad\kappa=13.
\]
The finite-history old raw-channel removal deletes 256 carrier coefficients and two scalar rows: `R=4980`; row padding and kappa remain 8192 and 13. This compiler does not need an extra public-claim batching stage for those retained scalar rows. It still pays ordinary/raw multipoint reductions that produced their points, and all frontend claim reductions.

These counts are distinct from the security note's **49/47 ring rows**. Flattening changes the row-check representation, not Eq. (109) witness allocation.

## 7. Cost and implementation contract

Do not allocate the dense `R*N` matrix. A correct weight provider derives its entries/adjoints from each native row family, its exact negacyclic coefficient action and layout. It may compute `lambda_rho~(r)` by transposed ring operations or direct streamed products. The latter is a correctness reference, not a performance claim. Public setup matrices, gadget weights and all query-dependent coefficients remain real work; without setup offload the verifier must compute the exact weight evaluation.

Required interfaces:

1. A canonical row manifest binding original views, planes, coefficient ordering, scalar claims, all digit maps and complete W columns.
2. A prover row/weight evaluator and verifier terminal weight evaluator that compute the same map (NC), including wrap signs and all source permutations.
3. The unchanged full-grid range product pipeline plus the specified fused (F) endpoint and declared anchor/combination challenge order.
4. A complete tree/error manifest: row factor 8192 in this example, actual range/product factors, degree-three fused rounds, combination family, source prefixes and final proof.
5. The existing checked native ownership/pause/adoption and original path retention. A new relation mode cannot bypass material authority.

Existing quotient-lift paths are not claimed to implement this mode. A final suffix can use its own admitted relation mode, but must open the original K source at its actual E point and carry every required setup obligation. No implementation, benchmark, or native schedule-admission run was performed.

## References

- [Akita](SOURCE_PROVENANCE.md#akita), original A/B/F and D/H equations, Eq. (109), range pipeline, fused Eq. (160), Theorem 10.23 and source uniqueness. Its native ring equations are the inputs of (L), not replaced commitment maps.
- [SourceQueryLift](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md), full-ring scalar extension and complete witness ledger.
- [Range reuse](AKITA_INSTANCE_ACCUMULATION_RANGE_REUSE.md), full physical domain, required-zero masks and finite-history quantifiers.
- [Explicit security profile](AKITA_INSTANCE_ACCUMULATION_SECURITY_PROFILE.md), the newly provisioned extension-field example and conditional global budgets.
