# Native Akita receiver: concrete closure and parameter admission

## 1. Result and limits

This note advances the receiver obligation beyond the earlier deliberately insecure shape example. It gives an exact **full-successor shape calculation**, including repeated original-handle queries, deterministic all-valid-source response bounds, a sufficiently large unfiltered challenge shell, and hand lookups against Akita's checked-in coefficient-SIS rank table.

**Result:** the specified generic direct-coefficient instance closes into `2^23` base-field cells. Its complete Eq. (109) allocation is `7,989,248` cells, leaving `399,360` committed zero cells. Its listed matrix widths fit the selected table entries.

**Not established:** a complete Jolt segment profile; an implementation-supported re-entry schedule; full finite-chain soundness/error/runtime admission; exact sampler/transcript-grinding admission; performance parity. The fresh catalogue below is a precise generic signed-digit workload, not a claim about Jolt's actual columns or a replacement for that application ledger. Existing gadget-decomposed Jolt objects cannot be reinterpreted as these identity sources.

The calculation shows that repeated queries and adequate raw folding entropy do not by themselves make shape closure impossible. It does not authorize implementation branches.

## 2. Source provenance

- [Akita](SOURCE_PROVENANCE.md#akita), §§3.1–3.3: ring splitting, unit criterion, challenge cardinality, signed decomposition; §§4–7: original commitment maps and Eq. (109); §§10–12: complete security/completeness and schedule admission.
- Live qmd retrievals used here: `867–901`, `996–1155`, `4500–4695`.
- `akita/crates/akita-params/src/sis/generated_sis_table/q128.rs`: ADPS16 quantum LGSA coefficient-infinity **attack-model** table, with widths indexed by module rank.
- `akita/crates/akita-params/src/sis/ajtai_key.rs::audit_commit_matrix_fields`: runtime admission rounds to an audited bound and rejects ranks below the lookup floor. The calculations here are hand table lookups, not execution of that function.
- `akita/crates/akita-sis-estimator/src/schedule_security.rs`: estimates individual scheduled matrix occurrences; it does not itself prove the new accumulation theorem.

The q128 table at immutable Akita `8357433` and the inspected local table have the same SHA-256:

`97430a0c7963b69ed057ebe2f5b3b296486f1a01f6398a76ed41806db1afd5b1`.

This equality was checked using immutable `git show | sha256sum` and `sha256sum` on the local file. It establishes table identity only, not equivalence of the complete revisions or their supported schedule catalogues.

## 3. Fixed profile

Use

\[
q=2^{128}-2^{32}+22537,\quad E=\mathbb F_q,\quad
b_*=b_z=b_{\rm open}=16,
\]

and `d_A=d_B=d_D=d_car=128` for the old and fresh groups. All original sources use direct coefficient packing and one source digit. No setup offloading, response chunking or main/compression quotients are included in this candidate. Direct quotient-free coefficient re-entry is a **new admitted-topology obligation**, not a current planner option inferred from this arithmetic.

| Quantity | Old receiver | Fresh generic group |
|---|---:|---:|
| Number of polynomials `P` | 1 | 128 |
| Physical cells per polynomial | 8,388,608 | 4,096 |
| Ring elements per block `M` | 1,024 | 32 |
| Blocks per polynomial `B` | 64 | 1 |
| Total fold coordinates `W=P B` | 64 | 128 |
| Original source alphabet | `{-8,...,7}` | `{-2,-1,0,1}` |
| Uniform source coefficient bound | 8 | 2 |
| Source gadget depth | 1 | 1 |
| A rank | 4 | 3 |
| B rank | 1 | 1 |
| Query occurrences | 2 | 2 |

The two occurrences preserve the **same original commitment**: one retains the original pending evaluation claim, the other authenticates the source-range/zero reduction endpoint. For the fresh group, each occurrence retains the vector of 128 evaluations at its common point. The range prefix can combine that vector with the public polynomial-axis weights. No free multi-point API is assumed; the conservative ledger duplicates the full group opening machinery.

Each source's complete physical grid is covered. Range predicates are those of its declared alphabet; zero masks enforce every reserved span. The fresh signed-four alphabet is a generic dense contract, **not** a `UnitOneHot` declaration. Claiming a Jolt one-hot occupancy improvement would need its actual physical occupancy proof and source profile.

The source-range prefix is the direct polynomial reduction from the bounded-bridge note. It commits no separate range table: its endpoint becomes the second native query. Its arithmetic, transcript and error costs remain real, but it does not add an unaccounted range table to Eq. (109).

## 4. Challenge and deterministic completeness

Choose an unfiltered shell in the 128-dimensional carrier with 24 magnitude-one coefficients, 16 magnitude-two coefficients and independent signs:

\[
s=40,\quad \omega=24+2\cdot16=56,\quad c_{\max}=2,
\]

\[
|\mathcal C|=\binom{128}{40}\binom{40}{24}2^{40}
=176088657428812661245951631789034160673997161842606080000,
\]

with `log2 |C| ≈ 186.844275`.

The selected prime satisfies `q ≡ 9 (mod 16)`, so the paper's splitting parameter is 4. Its sufficient nonzero-difference unit condition is

\[
2c_{\max}=4<q^{1/4}/2,
\]

which is checked by the exact integer inequality `8^4 < q`. The same carrier is used in A, so no new embedding norm factor is hidden here.

For **every** allowed input source and every challenge in this shell,

\[
\|z_g\|_\infty\le U_g=W_g\omega\sigma_g.
\]

Thus `U_old=28,672` and `U_fresh=14,336`. Four base-16 response digits cover both, because

\[
T_4=7\frac{16^4-1}{15}=30,583.
\]

No honest-distribution concentration assumption or response-dependent retry is needed for these coefficient bounds. Exact sampling/XOF and any transcript grinding still need their separate bounded completeness ledger.

The common verifier-certified response alphabet gives

\[
\Delta_{\rm rsp}=16^4-1=65,535,
\quad
\eta_A=4\omega\Delta_{\rm rsp}=14,679,840.
\]

This is the weak/weak source-comparison bound, not just a collision between honest short sources. Mixed weak/bounded comparisons are smaller under the stated source bounds. Their exact physical comparison inventory must still be registered.

### Full-field opening digits

Use **32**, not 33, base-16 digits with Akita's asymmetric full-field lift:

\[
T=\min\left(7\frac{16^{32}-1}{15},\lfloor q/2\rfloor\right),
\qquad \operatorname{lift}(x)=
\begin{cases}x&x\le T,\\x-q&x>T.\end{cases}
\]

This covers every residue. Centering all residues first would require rechecking the asymmetric interval; the earlier example's extra digit was specific to its centered-lift choice. Using the specified native lift here does not weaken the certified digit alphabet.

## 5. Full successor and compression ledger

Each B and the shared D image has `1 × 128` field coefficients. Use two binary compression stages: rank one at dimension 16, then rank one at dimension 8. The first input has `128 × 128 = 16,384` binary cells; the second has `16 × 128 = 2,048`. The binary source alphabet is `{-1,0}` and must be enforced separately from the common base-16 range. Four source occurrences have four F paths; one shared H path binds the joint partials.

| Eq. (109) allocation | Base-field cells |
|---|---:|
| Old responses, two occurrences | 1,048,576 |
| Old partials, two occurrences | 524,288 |
| Old inner images, two occurrences | 2,097,152 |
| Fresh responses, two occurrences | 32,768 |
| Fresh partials, two occurrences | 1,048,576 |
| Fresh inner images, two occurrences | 3,145,728 |
| Compression stage 1, four F and one H | 81,920 |
| Compression stage 2, four F and one H | 10,240 |
| Main and compression quotients, conditional quotient-free route | 0 |
| Extra alignment/ring-completion cells | 0 |
| **Complete unpadded successor** | **7,989,248** |
| Final committed zero suffix | 399,360 |
| **Fixed capacity** | **8,388,608** |

Every displayed segment length is divisible by 128, so the stated boundaries require no extra alignment. The final tail is part of the full committed and range/zero-checked receiver domain.

The transient ring-row count is

\[
2(1+4+1)+2(1+3+1)+1+2(4+1)=33,
\]

plus the scalar opening row. Tensor/claim combinations and source-range prefixes have their own costs and error charges; they are not absent because they are not listed as ring rows.

Successor commitment A/B/F hints are retained outside this witness until their commitment becomes an incoming source. Including them in their own commitment input would be circular. Their geometry is fixed by the same receiver profile, so their size is bounded, but this note does not report the accumulator's full live byte count as merely `N` cells or the compressed payload.

## 6. Matrix table gates actually checked

For A, round the required `14,679,840` upward to the available bound `32,505,732`. For B/D use the full base-16 digit diameter 15. For F/H use diameter 1 only because the complete binary predicate is required.

| Matrix view | Ring dimension | Rank | Input width | Audited coefficient bound | Table width cap |
|---|---:|---:|---:|---:|---:|
| Old A | 128 | 4 | 1,024 | 32,505,732 | 1,820,660 |
| Fresh A | 128 | 3 | 32 | 32,505,732 | 976 |
| Old B, each occurrence | 128 | 1 | 8,192 | 15 | 133,850,702 |
| Fresh B, each occurrence | 128 | 1 | 12,288 | 15 | 133,850,702 |
| Shared D | 128 | 1 | 12,288 | 15 | 133,850,702 |
| F/H first compression stage | 16 | 1 | 1,024 | 1 | 7,077 |
| F/H second compression stage | 8 | 1 | 256 | 1 | 508 |

These are exact lookups of the specified checked-in table entries. They provide the table's modeled per-instance attack-cost gate, **not** a complete accumulated-protocol 128-bit security statement. Reusing overlapping setup prefixes, repeated original handles and source comparisons must be included in the exact schedule inventory. A table estimate alone is not a proof against all lattice attacks or a bound on the composed extractor's concrete runtime.

## 7. Remaining full-protocol budget

There are 384 fold coordinates across the four occurrences. For a raw-ROM illustration with `T=16` and `Q+1=2^32`, the folding contribution alone is

\[
\frac{2^{32}\cdot16\cdot384}{|\mathcal C|},
\]

whose negative binary logarithm is about 142.26. This is **only that term**; it excludes range, sumcheck, ring, batching, frontend, setup and terminal errors.

In particular, base-field sumchecks over this roughly 128-bit field do not automatically give a 128-bit query-budget guarantee. The inspected native code has transcript grinding with `TRANSCRIPT_SECURITY_BITS=128`; its work-factor accounting is not interchangeable with silently dropping the `(Q+1)` factor from the paper's raw-ROM theorem. The accumulation transcript must use one specified model, rederive every added site, and account for the complete history. No new completed-proof weighting hash is proposed.

Likewise, the matrix table is not an admission of the finite accepting-tree reduction at an arbitrary supported depth. The complete local tree includes the range prefixes and strong suffixes, and the composed reduction must be charged as described in [adaptive soundness](AKITA_INSTANCE_ACCUMULATION_ADAPTIVE_SOUNDNESS.md). The exact finite adaptive experiment and concrete advantage/runtime ledger remain separate gates.

## 8. Explicit shared-response two-channel alternative

The [raw source-query construction](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md), §8.1, specifies a different relation that shares one response and original A/B/F path across multiple query channels. For each handle, bind both complete partial vectors before the same fold challenges and impose both carrier equations:

\[
\sum_i c_i e_{i,0}=L_0(z),\qquad
\sum_i c_i e_{i,1}=L_1(z),\qquad z=\sum_i\iota(c_i)s_i.
\]

One channel retains the ordinary query and the other the range endpoint. Each map must commute with the same challenge action. Unit-coordinate subtraction recovers both partials from the same extracted original source. The scalar claim equations and row-batching charges remain separate. This is an explicit new multi-channel relation, **not** an optimization already admitted by ordinary `prove_batch`.

Under that conditional local relation, keep the preceding ranks, source alphabet, fresh geometry and four response digits, but reduce the old receiver to `N=2^22`, hence `B_old=32`. Now `U_old=14,336`. There are two source groups, each with two partial channels, two F chains and one H chain.

| Shared-response allocation | Base-field cells |
|---|---:|
| Old response | 524,288 |
| Old partials, two channels | 262,144 |
| Old inner images | 524,288 |
| Fresh response | 16,384 |
| Fresh partials, two channels | 1,048,576 |
| Fresh inner images | 1,572,864 |
| Compression stage 1, two F and one H | 49,152 |
| Compression stage 2, two F and one H | 6,144 |
| **Complete unpadded successor** | **4,003,840** |
| Final committed zero suffix | 190,464 |
| **Fixed capacity** | **4,194,304** |

Main/compression quotients remain zero only on the same conditional quotient-free route. All boundaries still align to 128. The original old B width is now 4,096; the shared D width is 10,240. Their selected table ranks remain sufficient at those smaller widths. The A and compression widths are unchanged. The ring-row count is

\[
(2+4+1)+(2+3+1)+1+2(2+1)=20,
\]

plus the scalar claim rows or their explicitly charged public combination. There are 160 fold coordinates, not 384; two channels do not double the lattice challenge vector.

This alternative has a factor-two smaller **admitted-by-this-arithmetic capacity** than the separate-occurrence example. It does not establish a factor-two speedup. It requires the new multi-channel partial/row descriptor, matching verifier/prover weights, source-projection proof, D/H binding and full finite-chain security admission. The fresh instance remains the generic signed-digit example, not the actual Jolt segment catalogue.

## 9. Cost consequence and next admission input

The separate-occurrence receiver contains 8,388,608 logical cells; the shared-response alternative contains 4,194,304; their common fresh workload contains 524,288. Those are cell-count ratios of 16 and 8, not measured runtime ratios. Both update routes scan and commit real receiver data and authenticate every query. Neither can be priced as only the fresh frontend plus a tiny public commitment.

The next calculation must substitute the pinned Jolt segment ledger: native columns, advice/program or field-inline groups, incoming/outgoing memory commitments and all boundary queries. It must retain actual original gadget geometry or use a proved same-handle raw-source query interface. Adding those groups to this example without rederiving its state, ranks and security would invalidate this certificate.

Verification performed: ephemeral exact integer calculations of shell cardinalities, response depths, full successor lengths, widths and the unit-criterion inequality; table/source inspection and the table-identity checks above. No accumulator was run, no source was changed, no new schedule was validated, and no performance result is claimed.

## 10. Jolt-dimensioned full-field RAM example

This is a **newly provisioned coefficient-profile calculation**, not a reinterpretation of existing frozen handles or a claim that the native catalogue currently generates it. It instantiates the application dimensions rather than substituting a generic signed-digit workload. It remains an example, not a restriction of the requested implementation to this workload.

Choose the full-public-program, non-field-inline, no-advice case, with segment capacity \(N_{\rm seg}=4096\), \(1\le\ell<N_{\rm seg}\), k=4, padded bytecode capacity 4096 and fixed proof-side RAM capacity 65536 words. The inspected modern `protocols/jolt/geometry/dimensions.rs::TryFrom<JoltOneHotDimensions>` computes address chunks by ceiling division of address bits by k. Thus bB=3, bR=4 and
\[
 P=128/4+64/4+1+3+4=56.
\]
The main commitment retains 56 native tables of 65536 Boolean entries each, not a prefix-packed single table. New incoming/outgoing RAM sources are full-field dense tables, as required by the [domain decision](AKITA_INSTANCE_ACCUMULATION_MEMORY_COST.md). This does not assert a whole-run u64 invariant from a producer preset.

Use the same q, 128-dimensional carrier and challenge shell as §4, but choose \(b_*=b_z=b_{\rm open}=32\). Opening digits and full-field RAM source digits have depth 26. Indeed the balanced depth-26 interval contains the centered interval \([-(q-1)/2,(q-1)/2]\); no 65-bit premise is needed. Main source depth is one with **proved Boolean coefficients**; no occupancy improvement is used in its response bound.

| Group | P | B | M | Source digits | A rank | Source absolute bound | Query channels |
|---|---:|---:|---:|---:|---:|---:|---:|
| Receiver, capacity \(2^{22}\) | 1 | 32 | 1024 | 1 | 4 | 16 | 2 |
| Main native columns | 56 | 1 | 512 | 1 | 3 | 1 | 2 |
| RAM incoming | 1 | 32 | 16 | 26 | 4 | 16 | 2 |
| RAM outgoing | 1 | 32 | 16 | 26 | 4 | 16 | 2 |

The two channels are the normalized ordinary and raw-source endpoints. If several raw predicates leave different points, use the **paid** same-RawView normalization in SourceQueryLift §8.1. Neither existing ordinary point batching nor commitment homomorphism supplies that step. Every main source Boolean predicate, physical source range, logical padding requirement and original decommitment alphabet remains in the complete relation. The honest sparse one-hot producer is not the proof of that domain or a restriction of completeness to honestly generated sources.

The deterministic response bounds \(U=PB\omega B_s\) are 28672 for receiver and RAM, and 3136 for the main group. The centered positive reaches of base-32 depth three and four are 15855 and 507375. Therefore receiver/RAM use four response digits and main uses three, covering **every** admitted source, not a Gaussian estimate.

| Full shared-response allocation | Base-field cells |
|---|---:|
| Receiver response | 524288 |
| Receiver two-channel partials | 212992 |
| Receiver inner-image digits | 425984 |
| Main response | 196608 |
| Main two-channel partials | 372736 |
| Main inner-image digits | 559104 |
| Each RAM response | 212992 |
| Each RAM two-channel partials | 212992 |
| Each RAM inner-image digits | 425984 |
| Four F chains plus shared H, stage 1 | 81920 |
| Four F chains plus shared H, stage 2 | 10240 |
| **Complete successor, counting both RAM groups** | **4087808** |
| Required final zero suffix | 106496 |
| **Fixed receiver capacity** | **4194304** |

All spans align to 128. The conditional quotient-free/direct-coefficient route retains no main or compression quotient planes; a quotient-lift implementation must add them and recompute closure. No setup offload is assumed or priced as free.

The new A collision bounds \(4\omega(32^{\delta_f}-1)\) are 234880800 for receiver/RAM and 7339808 for main. The checked-in q128 table rounds these respectively to 260046724 and 8126340:

| Matrix role | Width | Rank | Applicable table width ceiling |
|---|---:|---:|---:|
| Receiver A | 1024 | 4 | 18964 |
| Each RAM A | \(16\cdot26=416\) | 4 | 18964 |
| Main A | 512 | 3 | 23489 |
| Receiver / each RAM B | 3328 | 1 | 28621854 |
| Main B | 4368 | 1 | 28621854 |
| Shared D | 7904 | 1 | 28621854 |

B/D use digit-difference bound 31. Binary F/H widths remain 1024 at dimension 16 and 256 at dimension 8, within §6's unchanged rank-one ceilings. The full ring-row count is 38 and the fold coordinate count is \(32+56+32+32=152\). Scalar query rows/batching, public matrix-weight computation, fresh domain prefixes, multipoint reductions and final proof costs remain additional.

For the **finite assembled-history** variant, [range reuse](AKITA_INSTANCE_ACCUMULATION_RANGE_REUSE.md) removes the receiver's second channel after its first independently anchored occurrence. Fuse its required-zero mask into the outgoing strong check. At the same fixed capacity this removes 106496 partial cells and one ring row, yielding **3981312** live cells, **212992** tail cells, D width **7072**, 37 ring rows and still 152 fold coordinates. It does not eliminate fresh raw-source scans or establish arbitrary-fixed-input local-AS knowledge.

This calculation proves arithmetic closure for the specified new source maps, ranks and shared-channel relation. It does not certify the native planner, frontend lowering, final suffix, exact shared-setup inventory, finite mixed-tree/FS admission, or any end-to-end security-bit or performance claim. A larger trace/RAM, advice, committed program, field-inline mode or offloaded setup changes the source ledger and needs its own calculation. In particular, setup evaluation without offloading and source-domain work must be charged when comparing against recursion.
