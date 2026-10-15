# Akita accumulation: one-channel receiver and explicit final reduction

Status: independent reviews approved the digit-axis algebra, receiver/suffix counts, matrix-table comparisons, direct-terminal tree argument, global manifest and exact 33-role registry. Final integrated review admitted implementation of the selected T<=16 archive profile, including the source-linked frontend and exact segment consumers. This is not an implemented catalogue, overall concrete-128-bit security claim or performance result.

## 1. Fixed example and security policy

Keep original sources and all lattice matrices over
\[
K=\mathbb F_q,\quad q=2^{128}-2^{32}+22537,
\]
and scalar challenges over \(E=K[U]/(U^2-5)\). Use the full carrier \(E[Y]/(Y^{128}+1)\), with **256 K coefficients per partial**. The source carrier is \(S=K[Y]/(Y^{128}+1)\). This is not native subfield encoding by a field alias.

Select the direct-coefficient [digit-axis normalization](AKITA_INSTANCE_ACCUMULATION_DIGIT_AXIS.md). It converts ordinary and raw-range claims to one raw-view claim per original handle. The main source still has P=56 distinct columns; no compressed-commitment homomorphism or source redecomposition is assumed.

The example uses at most 16 consecutive segments, each with Nseg=4096, committed chunk bits k=4, a full public program of padded length 4096 and a RAM domain of 65536 words. No optional advice objects, committed program or field-inline feature is selected in this numerical example. P=32+16+1+3+4=56. This is a concrete profile, not a restriction silently applied to all Jolt workloads; other configurations need separately compiled counts.

Use the unfiltered challenge shell in dimension 128 with 24 magnitude-one coefficients, 16 magnitude-two coefficients and independent signs:
\[
\omega=56,\quad s=40,\quad c_{max}=2,\quad
|\mathcal C|=\binom{128}{40}\binom{40}{24}2^{40},
\quad\log_2|\mathcal C|=186.844275\ldots .
\]
The native sufficient unit-difference condition is unchanged: q mod 16 is 9 and \(4<q^{1/4}/2\). No response-dependent challenge filter or grinding discount is selected.

A declared **bounded-query** classical-ROM policy is
\[
Q_{max}+1\le2^{32},\quad c_E\le2^{40},\quad c_C\le2^{16}.
\tag{1}
\]
With exactly uniform E and shell coordinates, its statistical FS term obeys
\[
(Q_{max}+1)\varepsilon_{int}
\le 2^{72}/q^2+2^{48}/|\mathcal C|
<2^{-137}<2^{-128}.
\tag{2}
\]
This is a smaller supported query budget than the omega=153 example, not an equivalent security profile. It avoids paying for an unrequested Q=2^64 budget. Both control and accumulation must use the **same** chosen policy. Sampler failure, setup bias, transcript-composition costs and runtime-sensitive MSIS advantages remain separate. No overall 128-bit reduction is claimed.

## 2. Closed receiver

All common, response and opening bases are 32; the inner-image/partial opening depth is 26. There is one S-linear query channel per handle, source/carrier/A/B/D degrees 128, coefficient-domain response bounds, no setup offload, no quotient-lift and no response chunking.

| Original group | P | B | M | Source digit depth | Source bound | A rank | U=PB omega bound | Response depth |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Old receiver | 1 | 32 | 1024 | 1 | 16 | 4 | 28672 | 4 |
| Main native columns | 56 | 1 | 512 | 1 | 1 | 3 | 3136 | 3 |
| Each RAM endpoint | 1 | 16 | 32 | 26 | 16 | 3 | 14336 | 3 |

The positive centered base-32 reaches are 15855 for three digits and 507375 for four. Thus every valid bounded source, not merely honest sources, fits its response encoding. All valid noncanonical source and F/H paths must be retained.

The complete native source allocations are
\[
Z=M\delta_{com}\delta_f128,\quad
E_g=PB\cdot26\cdot256,\quad
T=PBn_A\cdot26\cdot128.
\]

| Allocation | K cells |
|---|---:|
| Old response | 524288 |
| Old partials | 212992 |
| Old inner-image digits | 425984 |
| Main response | 196608 |
| Main partials | 372736 |
| Main inner-image digits | 559104 |
| Each RAM response | 319488 |
| Each RAM partials | 106496 |
| Each RAM inner-image digits | 159744 |
| Four F chains and shared H, first stage | 81920 |
| Four F chains and shared H, second stage | 10240 |
| **Live successor, including both RAM endpoints** | **3555328** |
| Required zero suffix | 638976 |
| **Fixed receiver capacity** | **4194304** |

The recurrence therefore closes at N=2^22 for this example. Source-domain proofs and multipoint reductions do not introduce additional Eq. (109) digit planes; they do add work, messages and extraction trees. The ordinary opening remaining after the fused strong check is the next pending receiver claim.

For the Jolt-specific main role, [existing semantic Booleanity plus explicit physical-hole zeros](AKITA_INSTANCE_ACCUMULATION_MAIN_SOURCE_REUSE.md) replaces the extra full-main raw Boolean prefix without changing these cell counts. The generic direct Boolean role may keep that prefix. The different domain predicates are declared explicitly, not inferred from honest sparse producers.

There are 120 fold coordinates: 32+56+16+16. D width is 6240. Ring rows are 36, counting two carrier planes per group, all A/B rows, shared D and all F/H stages. In the [exact coefficient relation](AKITA_INSTANCE_ACCUMULATION_LINEAR_RELATION.md), the rows are
\[
(7+6+6+6)128+128+120+4\cdot2=3456.
\]
The last eight rows are four E-valued raw scalar claims. Hence Rbar=4096 and kappa=12. Ordinary P-column claims have not disappeared for free: their separation is paid in the prior raw-view multipoint protocol.

### Matrix certificates

The reviewed q128 table rounds the old A collision radius
\[
4\cdot56(32^4-1)=234880800
\]
to 260046724; rank four permits width 18964, above 1024. Main/RAM radius
\[
4\cdot56(32^3-1)=7339808
\]
rounds to 8126340; rank three permits width 23489, above 512 and 32*26=832.

B widths are 3328, 4368 and 1248 respectively. D width is 6240. Their digit-difference radius is 31, with rank-one ceiling 28621854. F/H retain the already reviewed dimension-16 width-1024 and dimension-8 width-256 negative-binary paths. Required zeros and exact path alphabets remain enforced.

Provision distinct registered setup roles for unequal source geometries; carried RAM endpoints retain the **same** RAM role/key on both sides of a cut. This avoids treating incompatible views as interchangeable. Any actual shared-prefix implementation needs its full joint view/collision inventory. The table comparisons are attack-model parameter checks, not the complete runtime-sensitive reduction.

## 3. Final domain check and fixed shrinking suffix

The final pending receiver opening alone does not assert its source is short. First run the same full physical range/binary/zero predicate and normalize its raw endpoint with the pending ordinary evaluation. For the identity receiver, both are evaluations of the same source view. This supplies the final bounded relation through the same descendant-certified source-uniqueness argument; it is not a prover assertion of shortness.

Now perform these three **complete** nonterminal native reductions with one original source group, one full-ring channel and the coefficient relation. Each output carries all outgoing strong checks. Two complete compression paths are needed in each successor witness: the incoming original F path and shared H path, totaling 36864 K cells. These are not eliminated by changing the next commitment mode.

| Incoming source | M | B | A rank | Response / partial / inner digits | Path cells | Live next witness | Next capacity |
|---|---:|---:|---:|---|---:|---:|---:|
| N=2^22 | 1024 | 32 | 4 | 524288 / 212992 / 425984 | 36864 | 1200128 | 2^21 |
| N=2^21 | 1024 | 16 | 3 | 393216 / 106496 / 159744 | 36864 | 696320 | 2^20 |
| N=2^20 | 512 | 16 | 3 | 196608 / 106496 / 159744 | 36864 | 499712 | 2^19 |

The first outgoing full commitment uses P=1, M=1024, B=16, A rank 3. The second uses M=512, B=16, A rank 3. At the third output, select an actual **TerminalInnerState** commitment at creation: M=512, B=8, A rank 3. This is a new scheduled binding mode, not a conversion/relabeling of an already compressed OuterPayload. It publishes its A images and retains the original bounded witness. All dimensions still satisfy N=BM*128.

For the first final step the A radius is the old receiver radius above. Subsequent nonterminal inputs have U=14336 and response depth three, so rank-three width 1024/512 satisfies the same 8126340 table entry. Their D widths are 1664 and 832; B widths follow their actual ranks/depths. Padding is required zero and checked. Final-domain prefix and the shrinking strong checks preserve the binary/zero masks belonging to their actual predecessor layouts, not a guessed uniform mask.

## 4. Full-ring direct terminal candidate

This is an explicitly proposed terminal relation. Existing trace-only terminal APIs are **not** asserted to implement it.

Let \(w_i\in S^{512}\), \(0\le i<8\), be the final physical source blocks, with source coefficient bound 16 and public inner images \(t_i=A w_i\). For the bound E-point opening, publish full-ring partials
\[
e_i=\mathcal L(w_i)\in E[Y]/(Y^{128}+1)
\]
and check their exact scalar recombination against the pending value. Bind the images, point, value, all partials, setup role and terminal-mode descriptor **before** drawing independent \(c_i\in\mathcal C\). Send
\[
z=\sum_i c_iw_i.
\]
Check all coefficients of
\[
Az=\sum_i c_it_i,\qquad
\mathcal L(z)=\sum_i c_ie_i,
\tag{3}
\]
including both extension planes, and enforce
\[
\|z\|_\infty\le8\cdot56\cdot16=7168.
\tag{4}
\]
No D/H commitment is needed for these clear, prechallenge-bound partials. The two existing F/H paths counted in the third nonterminal witness still remain necessary for that preceding reduction.

Coordinate-unit subtraction at this terminal gives a certified weak source. Same-inner-A-source uniqueness across the prior strong-check trees, followed by their range interpolation, supplies the preceding bounded witness. It must use the terminal's actual weak–weak comparison bound, not assume (4) directly bounds an extracted source. The sufficient native comparison radius is
\[
8\omega\cdot7168=3211264.
\]
Its table rounding is 4063108, whose rank-three width ceiling is 109366, above M=512. This is the candidate bridge to be independently checked against the exact supplied-tree proof, including weak unit/product certificates and same-view comparisons.

With a new checked canonical signed-i16 response encoding, (4) fits two bytes per coefficient. The terminal payload components would be:

| Component | Bytes |
|---|---:|
| Eight A images, rank 3, degree 128, canonical K | 49152 |
| Eight full-ring E partials | 32768 |
| 512*128 response coefficients as signed i16 | 131072 |
| **Subtotal** | **212992** |

This excludes earlier reduction records, public statement/setup identifiers, points, bounds, metadata and the full authenticated archive. It is not the proof's total size. The current serializer is not claimed to have this codec. Reject values outside (4) before embedding them in K. No zero-knowledge property is claimed.

## 5. Remaining admission work

For 16 updates plus this final suffix and terminal, the lattice-coordinate count is
\[
16\cdot120+32+16+16+8=1992<2^{16}.
\]
This applies to the **archive replay** deployment without an additional outer receipt. Any receipt has its own full manifest and cost. The scalar ledger must include all fresh and old source prefixes, multipoint reductions, frontend endpoints, native strong checks and the final decision. The frontend source audit alone does not certify their sum.

The digit-axis, direct-terminal, scalar/tree/sampler formulas, exact [matrix registry](AKITA_INSTANCE_ACCUMULATION_MATRIX_REGISTRY.md), [frontend reconstruction](AKITA_INSTANCE_ACCUMULATION_FRONTEND_DESCENT.md), [segment consumers](AKITA_INSTANCE_ACCUMULATION_SEGMENT_LOWERING.md) and physical-main coverage passed final integrated review for this example. Checked pause/adoption APIs, emitted implementation certificates and paired measurements remain implementation/acceptance work; no existing backend is asserted to implement these interfaces.

## References

- [Akita](SOURCE_PROVENANCE.md#akita), native coordinate folding, weak source certificates, terminal extraction, Eq. (109), source uniqueness and §§9–10 security accounting.
- [Source-query lift](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md), [digit-axis normalization](AKITA_INSTANCE_ACCUMULATION_DIGIT_AXIS.md), [exact linear relation](AKITA_INSTANCE_ACCUMULATION_LINEAR_RELATION.md).
- [Original receiver calculations](AKITA_INSTANCE_ACCUMULATION_RECEIVER_PROFILE.md) and [larger-query-budget extension example](AKITA_INSTANCE_ACCUMULATION_SECURITY_PROFILE.md).
- Immutable Jolt 0791f79a55c3fadc3d327c78a72228696c338468 and its Akita pin 83574331d4e51f8cce1d8689d05592fa4ef4c138; no working-tree API equivalence is assumed.
