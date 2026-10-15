# Common-K multipoint reduction for accumulation and its ordinary control

Status: conditional algebra and local pricing independently reviewed; paired implementation and execution pending. `ReviewCommonNormAlgebra` found no defect in the original-source equations (confidence 0.91); `ReviewCommonNormPricing` found no defect in the proposed compressed grammar and its conditional counts (0.94). Neither review establishes a composed security theorem or receipt. The original-plane ordinary driver now completes an actual K/K PCS proof but does not yet discharge these continued frontend queries.

## 1. Why the extra seam is required

The selected continued frontend returns one original Main batch after stage 8, one all-column batch for physical zeros, and two bytecode-column PC batches. They generally have different logical points. The frozen ordinary control contains Main plus two RAM groups and supports one logical point per group. Adding fake group copies changes the frozen profile; retagging a raw digit query as an ordinary opening changes its source relation.

Use a K-valued multipoint sumcheck before the paths diverge. Reuse the generic `NormalizationPlan<K,K>` and `NormalizationSession<K,K>` arithmetic where their views match. Keep their original coefficient source borrowed until binding. A decoded RAM reader is a linear view of the same original signed planes, not a new commitment or a canonical re-decomposition.

## 2. Main relation

Let \(P=56\), \(N=2^{16}\), and let \(X_p(z)\) be each original physical Main column, in the installed physical order. Extend only the polynomial-selector axis to 64 by definition:
\[
X(p,z)=\begin{cases}X_p(z)&p<P,\\0&p\ge P.\end{cases}
\]
These selector holes are virtual raw-view cells, not overwritten physical source cells.

Each checked frontend obligation is a fixed original linear functional
\[
L_i(X)=X_{p_i}(u_i)=v_i,
\]
with the actual source column and physical point retained. The source-check batches contribute 56 zero-check leaves and six PC leaves. The selected stage-8 Main batch contributes another 56, if its actual installed roster has exactly that shape. Do not infer these counts from arbitrary proof arrays.

Bind the entire ordered original claim list, source identity, decoder/view and finite dimensions before drawing independent \(\alpha_i\in K\). Define
\[
W(p,z)=\sum_i\alpha_i\mathbf1_{p=p_i}\operatorname{eq}(u_i,z),
\qquad c=\sum_i\alpha_i v_i.
\]
Run degree-two sumcheck for
\[
\sum_{p,z}\widetilde W(p,z)\widetilde X(p,z)=c.
\tag{1}
\]
The source has \(\mu=22\) variables, including six selector bits. Coordinates must use the existing original raw-view/decoder convention. Jolt's MSB logical points cross into Akita's convention through the existing checked reversal, not by changing the source layout.

At the terminal point, write \(r=(r_p,r_z)\) as conceptual grouping only: the actual raw coordinate vector contains the 16 low coefficient/position/block bits first and the six high selector bits last. Retain
\[
w=\widetilde X(r)=\sum_{p<P}\operatorname{eq}(r_p,p)\widetilde X_p(r_z).
\tag{2}
\]
Transmit the 56 original values \(e_p=\widetilde X_p(r_z)\), bind them before later challenges, and compute \(w\) with (2). No commitment is combined homomorphically.

- **Ordinary control:** discharge every \((C_{\rm Main},p,r_z,e_p)\) using its ordinary native coefficient-packing proof. Its actual source profile and original commitment are unchanged.
- **Accumulation:** feed the single original raw Main claim \((r,w)\), with K coordinates/values lifted into E2, to the native fresh-source obligation. Individual \(e_p\) values are not separately accepted as authenticated evaluations on this path; (1) ends at the one pending original raw query. Their only role is to use the same terminal message grammar as the ordinary control.

Equation (1) still requires source-linked full frontend Booleanity/digit-zero and the new physical-zero/PC checks. This common reduction alone does not permit omission of the generic Main Boolean prefix.

## 3. RAM relation

For one original RAM source let its supplied signed planes be \(d_{a,\ell}\in[-16,15]\), \(0\le\ell<26\), in the installed physical order. Define the ordinary decoded view
\[
Y(a)=\sum_{\ell=0}^{25}32^\ell d_{a,\ell}\pmod q.
\tag{3}
\]
Read (3) on demand from the original owner. Distinct legal paths that differ by a representation of \(q\) remain distinct original sources and commitments even if their decoded K values coincide. Never recreate the source from \(Y\).

For an actual ordered RAM obligation list \(Y(u_i)=v_i\), bind it before independent K coefficients and run the same degree-two reduction over the 16-bit decoded address domain:
\[
\sum_a \widetilde{\left[\sum_i\alpha_i\operatorname{eq}(u_i,a)\right]}(a)\widetilde Y(a)
=\sum_i\alpha_i v_i.
\tag{4}
\]
Its endpoint is the ordinary logical claim \(Y(r_a)=w\), not a random 21-bit raw-digit claim.

- Ordinary control opens \(Y(r_a)\) with its all-26-plane packing capability.
- Accumulation passes that same ordinary claim to its fixed positional decoder, lifting only its challenge/value representation to E2. The native balanced-plane domain prefix and decoder-adjoint normalization still act on the original raw source.

If an installed RAM roster contains exactly one point/value after authenticated alias curation, its original claim is already the required endpoint; a separately reviewed fixed single-claim branch may omit (4). No runtime value-based deduplication is allowed. The last-only RAM output query and stage-four endpoint can alias only after their exact address identity is checked.

## 4. Correctness and conditional soundness

**[INFERENCE — algebraic derivation.]** Honest source values satisfy (1)–(4) by linearity of evaluation and decoding. The existing stock sumcheck driver proves the bilinear products with degree at most two. No division, inverse random weight, commitment homomorphism or source canonicalization is required.

For soundness, apply the ordinary polynomial-IOP random-linear-combination and sumcheck argument to the exact original functional list. An accumulation endpoint is still pending: the native coefficient relation, bounded-domain extraction and final decider must eventually authenticate that original raw/decoded query. The ordinary control instead authenticates the original logical endpoints through the complete ordinary PCS. These are different endpoint consumers of the same frontend reduction, not interchangeable opening APIs.

The preceding frontend claim list must be fixed before its batching coefficients. All proof rounds remain adaptive and enter the retained transcript before the next challenge. The terminal message enters that same compiler before native/ordinary PCS randomness. No fresh seed or digest bridge is introduced.

This note does not establish a numerical security level. Common-K errors enter the K denominator; native E2 errors enter the E2 denominator. Akita's published noninteractive analysis is classical ROM, not an end-to-end QROM theorem.

## 5. Local grammar and pricing boundary

For a Main roster of \(m\) scalar leaves, the normalization sends 44 K round coefficients in compressed degree-two form plus 56 terminal values: 100 K elements, 1,600 unframed bytes. Its literal draws include \(m\) independent batching coefficients and 22 scalar round challenges. The conservative local scalar numerator is \(m+2\mu\), before separate PCS/frontend/composition terms. For the conditional \(m=118\) roster, this is 162. Do not charge the terminal 56-value disclosure as 56 new independent challenge draws.
These compressed counts require the stock Jolt `[c0,c2]` recorder, reusing `NormalizationSession` arithmetic. A direct call to Akita's existing `prove_full_ring_normalization` sends all three coefficients per round: 122 Main or 49 RAM elements with the terminal disclosures proposed here. The proposed integration does not use that uncompressed wire driver. For a fixed false input list the tighter one-shot batching-plus-sumcheck numerator is \(1+2\mu=45\); the conservative 162 numerator is not an extraction-tree or full Fiat–Shamir bound.

**Actual-implementation pricing correction:** the finite K installer now exists, but its vector descriptors currently select nested interpolation. Main118 therefore declares `2^118*3^22`, not the proposed coalesced `119*3^22`. The 100-element/1,600-byte figure counts only rounds and disclosures; the installed ordered claim-list proof binding, input claim and labels/framing are additional wire bytes. The new [composed resource/prerequisite audit](AKITA_INSTANCE_ACCUMULATION_COMPOSED_RESOURCE_ARTIFACT.md) reads exact immutable definitions and preserves these distinctions; no complete runtime-priced numerical admission or performance claim follows. Following the expanded installation-description cap failure, the parent selected full expanded framing with the versioned `segment_native_resources` finite profile (16MiB payload, 512MiB retained transcript and separately bound work/site/schema limits), not compact derivation. Current scope is native certificate/direct verification; optional wrapper absence is not a native blocker.


A RAM reduction with \(m_R\) leaves sends 32 K round coefficients plus one endpoint value, or 33 K elements. Exact optional/single-claim grammar must be frozen from actual consumers before pricing. Include original-plane decoding, public-weight construction, first-bind table allocation, source traversal and verifier endpoint work on both paths. Main normalization covers the 64-column virtual selector grid and must not be priced as one 65,536-cell column.
The selected current consumer roster has exactly one logical point per original RAM handle after checked address/value alias curation. It therefore uses a fixed RAM identity seam and omits (4), while retaining all native balanced-domain and decoder-adjoint obligations.


No performance parity follows from these counts. Complete costs still include native update/finalization, source predicates, the archive verifier guest and final ordinary K/K receipt, versus the same-semantics recursive control.

## 6. References and implementation anchors

- [Akita paper](SOURCE_PROVENANCE.md#akita), qmd `lattice-papers/akita.md`: §3.4, Theorem 3.7 and Figure 2 for degree bounds, compression and conservative batching error; §§3.8 and 10 for coordinate-wise extraction and bounded recursive knowledge analysis; §4.3 for shared-prefix setup; §9.5 and §14 for the classical-ROM/QROM distinction. These results do not supply a general homomorphic commitment API. The §3.4 source was reread through qmd MCP at lines 1135–1217.
- [Original-plane packing contract](AKITA_INSTANCE_ACCUMULATION_NATIVE_PACKING_CONTROL.md).
- [Physical Main coverage](AKITA_INSTANCE_ACCUMULATION_MAIN_SOURCE_REUSE.md).
- [Continued segment consumers](AKITA_INSTANCE_ACCUMULATION_SEGMENT_LOWERING.md).
- [Selected native-reuse profile](AKITA_INSTANCE_ACCUMULATION_NATIVE_REUSE_PROFILE.md).
- `akita-types/src/full_ring/query.rs::{NormalizationClaim,NormalizationPlan,DecoderPlan}`.
- `akita-types/src/full_ring/strong.rs::NormalizationSession`.
- `jolt-verifier/src/segment_source.rs::{MainColumnOpenings,verify_segment_main}` and paired producer.
- `akita-cpu-backend/src/opaque/ordinary_projection.rs` and `sources/packed_ram.rs`: receiving-owner authority and exact original-plane endpoint consumer.
