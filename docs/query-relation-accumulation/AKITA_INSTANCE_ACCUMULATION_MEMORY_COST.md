# Native RAM endpoint commitment cost and incremental reuse

## 1. Decision, scope, and anchor

**Result:** native endpoint **commitment formation** has a correct changed-block incremental algorithm. It need not redo the A multiplication or B multiplication over all RAM on every segment. It still needs retained private material, all affected inner-image decompositions, the complete F compression chain, and an admitted backend path that preserves that material. This is not homomorphism of the transmitted commitment and does not eliminate full-source opening, range-prefix, multipoint, or copying costs.

**Statement-preserving profile decision:** the default new endpoint polynomial is a full proof-field polynomial, using the existing `JoltDenseFull` / Akita `Dense` source class. Honest RAM values are words, but an honest producer's u64 representation does not prove that *every algebraically valid existing Jolt witness*, including unobserved advice cells, has u64-valued intermediate RAM. A `JoltDenseBounded` **signed 65-bit** endpoint remains a conditional optimization: it requires a word-domain lemma for the unchanged statement, or an explicitly strengthened statement used by both accumulation and the baseline. Do not silently add a fresh logical-word range predicate to preserve a cheaper producer.

Both alternatives bind the **entire proof-side, word-addressed RAM table**, not just touched words. The full-field choice changes the allowed values, not addresses or the RAM recurrence. Its raw source digits are still short balanced digits and can have source-bound certification; this is distinct from imposing a u64 bound on the decoded RAM polynomial. Once a profile is selected, the original A/B/F matrices, gadget depths, ring geometry, slicing, source ordering and padding remain fixed. No existing bounded handle is reinterpreted as full-field, or vice versa.

The anchor is the existing Jolt proved relation, partitioned into consecutive legal segments. For segment i,

\[
 B_i^+(a)=B_i^-(a)+\sum_s RA_i(a,s)\,Inc_i(s),\qquad
 (C_{R,i}^+,\mathrm{shape})=(C_{R,i+1}^-,\mathrm{shape}).
\]

The first initialization and existing final-RAM/output/native padded-final convention occur once, as described in the [segment interface](AKITA_INSTANCE_ACCUMULATION_SEGMENT_INTERFACE.md). A host emulator snapshot is not substituted for this recurrence. No stronger MMIO, panic, halt, online-input, or resumable-emulator theorem is assumed. Generating the full existing trace once, selecting a sufficient fixed RAM shape, and then legally partitioning it is allowed.

For canonical producer inputs, the optimization anchor is equality with a fresh commitment under the **same** source/profile/setup. For arbitrary valid noncanonical decommitments, the anchor is instead equality with the path-aware full recomputation in §4: unchanged raw source and path coordinates are retained. Canonically recommitting only the decoded logical vector is not an equivalent reference in that general case.

This document is research, not an implementation, an admitted accumulation catalogue, a public update-verification protocol, or a performance measurement. No source edits, branches, builds, tests, benchmarks, or formatters were performed.

## 2. Evidence and revision boundaries

Use these immutable pins, deliberately without claiming a compatible combined build:

- **J**: Jolt [`0791f79a55c3fadc3d327c78a72228696c338468`](https://github.com/a16z/jolt/tree/0791f79a55c3fadc3d327c78a72228696c338468), the selected modern frontend.
- **A**: Akita [`83574331d4e51f8cce1d8689d05592fa4ef4c138`](https://github.com/LayerZero-Labs/akita/tree/83574331d4e51f8cce1d8689d05592fa4ef4c138), its modern companion. Immutable preset, executor, portable-material and material-authority sources were read with `git show`.
- **Acur**: explicitly current local Akita [`2607265b21f4726954461869f75acc2a8ab6ada1`](https://github.com/LayerZero-Labs/akita/tree/2607265b21f4726954461869f75acc2a8ab6ada1). Detailed arithmetic, source admission, compression and witness assembly observations below are **Acur**, not an assertion that those implementations are identical at A.
- **Paper**: local QMD `lattice-papers/akita.md`, §4.5, especially lines 1825–1921, Eqs. (69)–(74); §4.4, lines 1781–1789, Eq. (68). Retrieved through QMD. The paper defines the algebra; implementation pin references define the observed APIs and byte layouts.

Source map:

| ID | Pinned path / symbols | Observed fact |
|---|---|---|
| J-config | [J `crates/jolt-prover/src/config.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-prover/src/config.rs), `derive_from_rows`, `remap_address`, `one_hot_config`, `commitment_total_vars` | RAM is trace/image-derived; Akita defaults to k=4; setup embedding currently does not include new endpoint groups |
| J-defaults | [J `common/src/constants.rs`](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/common/src/constants.rs) | 32 MiB heap, 4096-byte stack, 128-byte canary, 4096-byte input/output and each advice capacity |
| A-presets | [A `crates/akita-config/src/proof_optimized/fp128.rs`](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-config/src/proof_optimized/fp128.rs), `Dense`, `DenseBounded` | q128 full source versus signed-65 source; both have full-width opening data; different catalogue identities |
| A-executor | [A `crates/akita-cpu-backend/src/commitment/executor.rs`](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-cpu-backend/src/commitment/executor.rs), `execute_uncompressed_stages`, `execute_full`, `execute_full_via_outer_image`, `validate_output_state` | Explicit inner/outer/compression stages, owned state, setup/plan/owner checks; the outer-image path retains A rows and recompresses the complete image |
| A-hint | [A `crates/akita-cpu-backend/src/commitment/portable.rs`](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-cpu-backend/src/commitment/portable.rs), `PortableCommitmentHandle` | Retains semantic A rows, two packed F stages, and mode-dependent quotient rows; it is not an exposed source/digit cache |
| A-authority | [A `crates/akita-cpu-backend/src/opaque/witness_build/commitment_material.rs`](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-cpu-backend/src/opaque/witness_build/commitment_material.rs), `CpuCommitmentMaterialHandle` | Operation binding, commitment ID, admitted public commitment, private inner/compression material; public commitment equality is checked |
| C-A | [Acur `crates/akita-cpu-backend/src/arithmetic/commitment.rs`](https://github.com/LayerZero-Labs/akita/blob/2607265b21f4726954461869f75acc2a8ab6ada1/crates/akita-cpu-backend/src/arithmetic/commitment.rs), lines 158–257,503–525 | Cached dense digit planes, original block decomposition, A rows and exact block geometry |
| C-B | [Acur `crates/akita-cpu-backend/src/arithmetic/commitment_stage.rs`](https://github.com/LayerZero-Labs/akita/blob/2607265b21f4726954461869f75acc2a8ab6ada1/crates/akita-cpu-backend/src/arithmetic/commitment_stage.rs), lines 18–134,151–230 | Existing outer route decomposes all A rows, builds all sliced B inputs, and stacks all outputs; host export clones rows, unique consume can avoid cloning |
| C-F | [Acur `crates/akita-cpu-backend/src/opaque/compression.rs`](https://github.com/LayerZero-Labs/akita/blob/2607265b21f4726954461869f75acc2a8ab6ada1/crates/akita-cpu-backend/src/opaque/compression.rs), lines 14–120,178–279 | Complete chain digitization, bounded batches, cyclic/negacyclic quotient-lift work, retained packed material and measurement fields |
| C-Fplan | [Acur `crates/akita-params/src/compression.rs`](https://github.com/LayerZero-Labs/akita/blob/2607265b21f4726954461869f75acc2a8ab6ada1/crates/akita-params/src/compression.rs), lines 20–24,226–298,397–439 | Two rank-one maps; q128 dimensions 16 then 8; exact negative-binary input widths and 128-byte terminal target |
| C-admit | [Acur `crates/akita-cpu-backend/src/commitment/api.rs`](https://github.com/LayerZero-Labs/akita/blob/2607265b21f4726954461869f75acc2a8ab6ada1/crates/akita-cpu-backend/src/commitment/api.rs), lines 84–94,108–144,310–344; [source/builtins.rs](https://github.com/LayerZero-Labs/akita/blob/2607265b21f4726954461869f75acc2a8ab6ada1/crates/akita-cpu-backend/src/commitment/source/builtins.rs) | Exact profile widths and asymmetric source interval admission; ordinary dense admission computes centered reach over coefficients; explicit commit profile is not opening-schedule admission |
| C-witness | [Acur `crates/akita-cpu-backend/src/opaque/witness_build/finalize.rs`](https://github.com/LayerZero-Labs/akita/blob/2607265b21f4726954461869f75acc2a8ab6ada1/crates/akita-cpu-backend/src/opaque/witness_build/finalize.rs), lines 527–569 | Existing witness builder redecomposes all semantic A rows into canonical inner-image digits |

The Jolt names `JoltDenseFull` and `JoltDenseBounded`, their native source roles, and their frontend use are documented with J/A source references in segment-interface §3. The new RAM endpoint role itself is not an existing producer in that pin.

## 3. Fixed physical geometry and original-source equations

Avoid confusing RAM size with the paper's within-block symbol M. Here:

- R is the run's fixed remapped RAM word-address capacity, a power of two.
- L is the number of packed A-ring positions per source block.
- J is the number of actual source blocks; absent MLE block indices are virtual zeros, not commitments.
- d_A, d_B are original ring dimensions; n_A, n_B are original matrix ranks.
- δ_s, b_s are the original source gadget depth/base; δ_t, b_t are the full-field inner-image gadget depth/base.
- S_B is the original B slice count; I_h its canonical block ranges; L_B=ceil(J/S_B).

The endpoint is one physical polynomial, P=1, of the admitted dense arity, nominally `max(14,log2 R)` in the selected frontend design. Its native physical block capacity can exceed both R and that dense table's declared natural extent. All actual final-block completion coordinates remain part of the source relation. The RAM table occupies its specified coordinate injection and all required physical padding is zero; absent-block and raw-query virtual zeros are accounted separately.

Let `s_i in R_(q,d_A)^(L δ_s)` be the **actual committed short source**, not merely the logical RAM vector. In column order `(position x, digit ell)`, let G_s decode it. Let `hat(t)_i` be a valid, possibly noncanonical, original B-source decomposition of the A image. The source path obeys

\[
 G_s s_i=f_i,\quad t_i=A s_i=G_t\widehat t_i,
 \quad u_h=B\,x_h(\widehat t),\quad u=\Vert_{h<S_B}u_h.
 \tag{1}
\]

For one block,

\[
 m_A=L\delta_s,\qquad
 g_B=n_A(d_A/d_B)\delta_t,\qquad m_B=L_Bg_B.
 \tag{2}
\]

`x_h` is the exact B-native permutation and per-slice zero completion, not a new matrix. The permutation is the paper's

\[
 (a\delta_t+v)d_A+y d_B+k
 \longmapsto ((a(d_A/d_B)+y)\delta_t+v)d_B+k.
\]

A compressed path additionally supplies negative-binary vectors h_1,h_2 and images u_1,u_2 such that

\[
 G_{F,1}h_1=u,\quad u_1=F_1h_1,\qquad
 G_{F,2}h_2=u_1,\quad C=u_2=F_2h_2.
 \tag{3}
\]

All equations use the scheduled coefficient packing, ring views, completion cells and relation mode. Quotient-lift relations additionally retain the exact required quotient images. A raw-payload profile omits (3) and transmits the complete u; it is a different admitted payload configuration, not an optimization silently applied to a compressed handle.

### 3.1 Full-field and signed-65 alternatives

The observed modulus is `q = 2^128 - 2^32 + 22537`. `Dense` and `DenseBounded` both keep full-width inner-image/opening data. The latter changes the source domain and therefore A width, source digits, response geometry and admission, **not** t's field width.

For an explicitly selected base-16 source geometry, full-field source encoding can use δ_s=32 under the native prescribed lift, whereas signed-65 needs δ_s=17 to cover every u64. These are conditional geometry calculations, not a claim that every selected catalogue row chooses base 16. The exact representable integer interval for depth δ and balanced base b is

\[
 -\frac b2\sum_{j<\delta}b^j\ \le z\le
 \left(\frac b2-1\right)\sum_{j<\delta}b^j.
\]

For full-field encoding the native lift/centering threshold must cover all residues; assuming the usual symmetric centered interval without checking these asymmetric reaches is wrong. For a retained signed-65 handle, retain its actual δ_s and all of its columns—never reinterpret it as a one-digit word or a 64-bitplane commitment. A 65-bit signed declaration is `[-2^64,2^64-1]`, not a proof of unsigned-word semantics. For a retained full-field handle, do not switch later boundaries to bounded geometry because the honest trace happened to have small words.

The 17/32 comparison applies to source coefficient count at identical block/ring geometry. Planner-selected n_A, L, J, slicing and later schedules may change, so it is not a 17/32 end-to-end time ratio.

## 4. Exact changed-coordinate update, including noncanonical paths

The following is a **derived private prover algorithm**, valid in either source profile, subject to the declared domain and backend support. It commits anew to the outgoing state. It never claims a publicly verifiable update from old C plus a sparse delta.

### 4.1 Retain the original decommitment

Retain old `s`, `t`, `hat(t)`, u and the supplied F path/quotients, together with the logical proof-side RAM state and the setup/profile/owner binding. The immutable incoming C and its complete decommitment remain available until every pending claim that needs them has been consumed.

For a canonical producer, source digits can be recovered from the logical vector and `hat(t)` from t. For an arbitrary valid original decommitment this reconstruction is **not** licensed: `G_s` and the path gadgets need not be injective on the whole accepted domain. In particular, an unchanged logical word can have noncanonical original source digits, and an unchanged t can have a noncanonical valid `hat(t)` with a different B image.

The completeness reference therefore treats the retained digits as authoritative. This is stronger than simply keeping a `DensePoly` and a commitment payload.

### 4.2 Patch only changed logical-coordinate digit fibres

Let D be the set of coordinates whose **proof-side** final value changes. Derive the net increments from the selected trace's RA×Inc recurrence; repeated writes that cancel need no source change. With ordinary concrete selectors a trace pass can build D without scanning all RAM. This is a witness-generation optimization, not a proof that an arbitrary incoming public C has a sparse valid transition.

For each a in D, compute a valid native source decomposition of its new value under the **same** b_s,δ_s and lift. Replace only that coordinate's entire digit fibre. For a not in D, keep every original digit coefficient exactly, even inside a block containing another changed word. Keep all physical padding coordinates exactly; their original zero predicate still belongs to the authenticated source relation. Thus

\[
 s'=s+\Delta s,\qquad G_s s'=f',\qquad
 \Delta s\text{ is zero outside changed-coordinate digit fibres}.
 \tag{4}
\]

If bounded65 is selected, the new value must meet the selected interval and any independently required logical-word predicate. Full-field encoding does not require a fresh logical-word bound. Canonicalization of only changed fibres is allowed because C' is a **new** outgoing commitment. Canonicalizing all source digits of a changed block would silently replace unchanged original-source coordinates and is not this sparse algorithm.

### 4.3 Apply A to the exact digit difference

For affected source blocks i,

\[
 t'_i=t_i+A\Delta s_i.
 \tag{5}
\]

For unaffected blocks retain t_i. Equation (5) is exact linearity of A on its original digit source, not linearity of the logical-word commitment. In general

`decomp(f+delta_f) != decomp(f)+decomp(delta_f)`.

In particular, do not compute A on just the logical RAM increment, or omit source-digit carries. Differences of two allowed signed digits can exceed the kernel's admitted digit alphabet. The arithmetic implementation must handle that exact wider difference, or compute `A s'_i - A s_i` using two valid bounded inputs. It cannot pass an out-of-range difference to an i8/balanced kernel and assume its optimized accumulator bounds still apply. Native field/ring reduction remains exact.

A change to one input coefficient can affect every coefficient of every A output row in its block. Counting changed **words** as changed A-output **coefficients** is not sound.

### 4.4 Replace affected inner-image digits and update B

For a block whose t changes, derive a valid new full-width decomposition `hat(t)'_i` under the original b_t,δ_t. If its t does not change, retain its original `hat(t)_i`; in particular do not normalize a noncanonical old path gratuitously. For each affected B slice,

\[
 \Delta x_h=x_h(\widehat t')-x_h(\widehat t),\qquad
 u'_h=u_h+B\Delta x_h.
 \tag{6}
\]

Use the exact old `hat(t)`, not a newly canonicalized surrogate. All unaffected blocks and slice padding keep their original coordinates. Differences in (6) also need an exact wider-digit arithmetic route or the two-products fallback. The complete stack u' contains every slice, changed or not.

This proves (1) for s',t',`hat(t)'`,u'. It is valid even if the original path was noncanonical. If instead the desired reference is a *wholly canonical* fresh commitment, one may deliberately canonicalize and recompute everything, but cannot claim changed-coordinate cost or equality to the old handle along the way.

### 4.5 Complete F compression and publish a newly bound handle

Run **every F stage** on the complete u', with the admitted plan, producing a fresh valid full compression path and any required quotient images. C' is the terminal image. Do not compress only the affected slices, attach independent leaf commitments, or replace the full chain by `C + Commit(delta)`.

It is legitimate to canonicalize the newly formed outgoing F chain: no incoming C is being replaced. The incoming noncanonical F path remains intact for its own openings. If absolutely nothing changes and the complete old path can be retained, publish/reuse the identical old handle; a needless canonical recompression of the same u can yield a different payload when the old F path was noncanonical.

The backend must bind the newly owned material to the new public commitment and source role, with the original frozen profile. The next segment uses exactly this outgoing handle and shape as its incoming handle. Adjacent handle equality is public and cheap; it neither proves RAM transition validity nor certifies cache contents. Those are discharged by the frontend endpoint claims and actual PCS/accumulation proof.

### 4.6 Completeness argument and its limit

For any supplied valid original decommitment satisfying the selected domain, (4) retains all unchanged source coordinates. Equation (5) gives `A s'=t'`; new inner-image digits recompose to t', and retaining unchanged inner-image digits preserves their old recompositions. Equation (6) is the exact sliced B image of this mixed retained/new digit vector. Fresh complete F formation yields (3). Therefore the method produces a valid new commitment path without requiring incoming canonicality.

This is an algebraic formation/completeness argument, **not** a proof that existing code accepts every such private representation or that the public verifier validates the incremental update. It also does not prove bounded65 completeness for unrestricted algebraic RAM, source-domain extraction, new source-query admission, or the finite accumulation/history theorem.

## 5. Existing reusable material versus required backend work

| Component | What the observed code offers | What sparse/all-valid reuse additionally needs |
|---|---|---|
| Setup matrices / NTT preparation | A-executor prepares checked routes and shared resources; C-F exposes compression cache reports | Retain same setup identity and exact matrix prefix/NTT geometry; charge cold preparation separately |
| Dense source digits | C-A accepts cached predecomposed dense planes and coefficients | Stable mutable/versioned source-coordinate storage; preserve original digits rather than always redecompose logical values |
| Semantic A images t | A-hint and A-authority retain polynomial-ordered A rows | Changed-block row patching without rereading/cloning every row; preserve old version while its proof still uses it |
| Original B-source digits `hat(t)` | A-hint retains t, **not original `hat(t)`**; C-witness canonically redecomposes all t at witness construction | For arbitrary noncanonical paths, retain their exact original digits and route them into witness generation. Semantic t alone is insufficient |
| B image u | A-executor has uncompressed-stage/outer-image execution; C-B normally computes all slice inputs | Cache complete u plus original slice coordinate map; exact sparse delta kernel or changed-block two-products route |
| F path and quotients | A-hint retains packed F stages and quotient rows; C-F handles both quotient-lift and reduced-evaluation modes | Generate a complete new chain; preserve old supplied chain for old claims, not just its terminal C |
| Material authority | A-authority checks operation binding and admitted public C; stage executor checks owner/setup/plan | An admitted backend-owned update operation or equivalent stage route. Do not manufacture authority from public bytes, relabel old material, or expose private internals as a bypass |
| Retained-state policy | A-executor has resident/portable policies; C-B can consume uniquely owned rows and clones on shared export | Explicit version/lifetime handling. A full immutable clone per endpoint is still memory-linear work |

**Concrete obstruction in the existing API path:** a portable hint containing only semantic A rows cannot represent which noncanonical `hat(t)` was originally committed. C-witness's all-row canonical redecomposition may change the B image for that input. Existing canonical producers are useful building blocks, but their hints do not by themselves implement the general all-valid path-preserving update. The solution requires preserving and consuming that missing private material, not assuming all valid inputs came from the honest canonical committer.

Likewise, C-B's ordinary route currently decomposes every inner image and materializes every B slice. Reusing its prepared matrices or a `PortableCommitmentHandle` alone does not make that call sparse. Calling the ordinary full commit API on the new complete logical RAM vector is a correct **canonical recommit** when that is the chosen new-source policy, but still scans/recomputes RAM and does not preserve arbitrary unchanged original source coordinates as in §4.

Acur C-admit shows another cost trap: bounded-source admission invokes `committed_centered_reach`; the `DensePoly` implementation traverses its coefficient storage. A trusted private mutation cache can maintain known bounds for unchanged coordinates and check changed ones, but adopting that cache is implementation work and is not a public range certificate. Existing source-admission and ownership checks must not be skipped merely because the prover says it has a delta.

## 6. Exact arithmetic and memory ledger

The following counts are operation/element counts, **not timings**. They refer to one endpoint P=1. Read all parameters from the selected original profile, not from the receiver witness profile.

Define

\[
 N_{\rm dec}=JLd_A,\quad
 N_s=JL\delta_s d_A,\quad
 N_t=Jn_A d_A,\quad N_{\widehat t}=Jn_A\delta_t d_A,
 \quad U=S_Bn_Bd_B.
 \tag{7}
\]

N_dec includes actual block completion; N_s is the raw source volume. The raw-query Boolean grid may further pad J and δ_s virtually, as specified by [SourceQueryLift §2](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md).

Let W be the number of changed logical coordinates, J_D the number of affected A blocks, K_s the number of nonzero *ring entries* in Δs across those blocks, and K_t the number of nonzero B-native ring entries in all Δx_h. Then

\[
 J_D\le\min(J,W),\quad
 K_s\le\min(W\delta_s,J_D L\delta_s),\quad
 K_t\le J_Dg_B.
 \tag{8}
\]

Use actual supports when cancellations occur. Bounds count ring entries, not individual changed scalar coefficients. A monomial/small-support ring kernel may be cheaper, but none is assumed here.

| Work | Full canonical formation | Path-preserving incremental formation |
|---|---|---|
| Source digitization | Up to N_s digit writes, plus reading the full logical/physical source | Wδ_s digit writes plus exact old-coordinate reads, assuming authoritative indexed storage |
| A multiplication | J n_A m_A ring products at dimension d_A | n_A K_s ring products for an exact sparse difference implementation, plus output additions; conservative two-products affected-block fallback costs at most 2 J_D n_A m_A |
| Inner-image digitization | N_hat(t) digit writes | At most J_D n_A δ_t d_A writes; often all output coefficients in an affected block |
| B multiplication | S_B n_B m_B ring products at dimension d_B, including padded slice widths | n_B K_t exact sparse difference products plus changed image additions; conservative full affected-slice recompute can be much more expensive |
| Complete F formation | Full two-map cost below | **Same full two-map cost** on complete u'; only skip if reusing the entire unchanged old commitment/path |
| Retained material serialization/copy | Depends on API, potentially all source/images/path | Not automatically reduced; full cloning/export erases sparse memory traffic savings |

For exact-difference products whose coefficient range is not supported by existing optimized kernels, the table's sparse product count is an algebraic target, not a claim of an existing entrypoint. Using the two bounded products separately preserves correctness but must include both passes and their scratch space. B can similarly multiply old/new changed block contributions with exact zero placement, or recompute full affected slices when only dense kernels are available.

### 6.1 Complete q128 F-chain cost at Acur

C-Fplan fixes q128 compression dimensions 16 then 8 and rank one. For a valid compressed profile with complete input stack U base-field coefficients:

\[
 m_{F,1}=\lceil128U/16\rceil=8U,\quad
 |u_1|=16,\qquad
 m_{F,2}=\lceil128\cdot16/8\rceil=256,\quad |C|=8.
 \tag{9}
\]

Thus the ring-product inventory is `8U` products at dimension 16 and `256` at dimension 8 in reduced-evaluation mode. Quotient-lift C-F computes **both negacyclic and cyclic** products, then `(cyclic-negacyclic)/2`; count both transforms/products and quotient assembly, not just the reduced commitment. The full chain digitization visits `128(U+16)` binary digit positions. Packed path bytes are `16(U+16)` before container overhead; expanded i8 RHS bytes total `128(U+16)` across the two stages, while peak scratch depends on stage lifetimes and batching. Quotient storage is `(16+8)*sizeof(F)` for these rank-one maps in quotient-lift mode and zero in reduced mode. The terminal payload is 8 field elements, 128 bytes, **only under this valid admitted compression plan**.

These formulas do not license an arbitrarily wide U at fixed security. C-Fplan validates the compression SIS authority and the complete chain has an input envelope. Full endpoint/receiver catalogue admission may require different parameters or may reject a shape. Acur's current fixed target is an implementation fact, not a theorem of constant-size commitments for every unbounded parameter choice.

The F cost is linear in U, not necessarily in R. Therefore sparse A/B reuse can still be asymptotically useful when U is much smaller than the RAM source volume. Conversely, if slice/rank choices make U large, complete compression can dominate even though only one RAM word changed. Do not infer an O(W)-only endpoint bound from (5)–(6).

### 6.2 Private memory, versions, and amortization

A direct cache can retain:

- Logical RAM: R field elements for unrestricted proof-side values; optionally an 8R-byte word buffer for an honest word-producing path. Do not store all-valid full-field RAM only in u64. A dense field-expanded physical table costs up to N_dec field elements separately if the source API requires it.
- Original source: N_s small signed coefficients in a supported packed/i8 representation. Raw source preservation is required for general noncanonical inputs even if a canonical producer could regenerate digits.
- Semantic A rows: N_t field elements.
- Original inner-image digits: N_hat(t) small coefficients, in the precise original B view or with an explicit permutation.
- Complete B stack: U field elements.
- Complete F path: the packed bytes above; quotient images if required; small terminal image and descriptors.
- Setup expansion/NTT caches, trace witness, old and successor accumulation witnesses, and source-query working memory: **additional**, not hidden in the endpoint cache count.

Retaining both t and `hat(t)` trades memory for avoided recomposition/decomposition. A block-addressed cache can avoid a full source scan per update, but it does not remove the linear-in-R stored state. If old and new endpoint versions must coexist, eager cloning costs Θ(N_s+N_t+N_hat(t)+U) elements even for W=1. Copy-on-write/block versions or delayed destructive update after the old proof consumer releases it can reduce this, but are proposed ownership choices, not a property of the existing hints. Such private block storage is **not Merkle RAM** and supplies no public authentication shortcut.

For a finite T-segment run, the endpoint commitment-formation ledger can therefore be

\[
 \mathrm{FullCommit}(R)
 +\sum_{i=1}^{T}\bigl[
   \mathrm{PatchSource}(W_i)+\mathrm{AUpdate}(K_{s,i})
   +\mathrm{Digits}(J_{D,i})+\mathrm{BUpdate}(K_{t,i})
   +\mathrm{FullF}(U)+\mathrm{VersionCost}_i
 \bigr].
 \tag{10}
\]

There is one initial handle and normally one new outgoing handle per segment, **not two newly committed RAM tables per segment**. Each next incoming handle is reused exactly. Initial preprocessing/material recovery may require a full pass; repeated preparations, source validation, exports or copies must be included if the actual integration performs them. If all blocks change, the safe fallback is full formation; sparse updates do not promise a win in that case.

## 7. Public proof/certification costs remain separate

Nothing in §4 certifies an untrusted cache. The public statement binds original endpoint handles before the relevant frontend challenges, and the frontend RAM identities leave genuine evaluation obligations on both originals. Even though adjacent segments share the same boundary handle, evaluations at different points remain different obligations unless a proved batching or multi-channel reduction combines them.

The accumulation-specific source-range/domain prefix concerns the **actual short source**, not merely f=G_s s. Its input volume is N_s (or its zero-extended raw grid), and its randomized queries generally depend on every raw coordinate. Maintaining cached A/B images does not provide these arbitrary evaluations or a proof of their boundedness.

In particular:

1. Native range-prefix witness construction and sumcheck may scan the full source; fresh random points and nonlinear digit predicates make an old scan result insufficient without a separately proved reusable reduction.
2. Ordinary within-C multipoint normalization of the logical RAM table can scan its full physical volume. A sparse delta can update an evaluation at a *fixed known point* cheaply, but the next random point is generally new. Obtaining the old table's evaluation there remains real work. No free dynamic multipoint data structure is assumed.
3. SourceQueryLift partials and the folded response can read the entire retained raw source. One shared z/A/B/F path across H channels removes duplication of those slots, **not** the H partials/evaluation rows or their source contractions. Its ledger is
   `Z=L*δ_s*δ_f*d_A`, `E=H*J*δ_open*d_op`, `T=J*n_A*δ_t*d_A`,
   plus all D/H and original F material, quotient rows, alignment and tail cells. Distinct raw and ordinary query channels are retained; neither is substituted for the other.
4. The ordinary pending claim on a carried handle is not discharged by range certification alone. Conversely, an ordinary opening does not prove bounds on a noninjectively encoded source.
5. Avoiding a repeated *old receiver* source-range proof by reusing the preceding strong suffix is a separate same-handle/theorem question addressed in the range-reuse research. The endpoint commitment cache is not that proof and does not automatically remove fresh endpoint source obligations.
6. Cache storage is prover-private. Public proof bytes, verifier arithmetic, Fiat–Shamir/tree/error charges, final decider cost and finite-history evidence must be counted independently. A concrete finite execution may pay the existing full extraction/tree cost; no free independent-leaf or uniform growing-depth claim follows from sparse RAM updates.

For full-field logical endpoints, the source domain can still be `[-b_s/2,b_s/2-1]` per original digit and zero-completion predicates. That does not require the decoded state to fit 65 bits. For bounded65 endpoints, add only the logical/domain predicates actually justified by the common statement; the mere native producer preset is not a proof of them.

**Conclusion of the separation:** endpoint commitment arithmetic can be amortized below a full RAM scan, but the total accumulated segment prover may still be RAM-linear due to its proof/certification machinery. Claiming an end-to-end sublinear-R update from the cache alone would be unsupported.

## 8. Fair full-PCS-plus-recursion baseline

Use the same continued-segment frontend, run statement, legal cuts, endpoint profile, RAM capacity/map, original native endpoint handles, logical padding obligations, public program/optional ISA choices, and final checks on both sides.

The baseline completes each segment's ordinary native PCS opening ledger and its full native suffix, then uses the selected finite recursive wrapper/history procedure to check the segment proofs and adjacency. A one-shot wrapper over the finite list is permitted; an online recursive wrapper per segment is not compulsory.

Charge to **both** paths:

- Initial RAM formation and the chosen outgoing endpoint commitment algorithm, including the same legitimate private A/B/setup caches and handle reuse.
- The RAM transition/frontend endpoint identities and any logical padding/domain checks required by the actual common statement.
- Full-field endpoints by default, or the same justified bounded65 statement/profile on both sides.
- All source material needed by their actual openings, truthful copy/serialization/retention costs, and the same initialization/final public output relation.

Charge only where actually required:

- Baseline: full per-segment PCS suffixes and the real recursive verification/history cost.
- Accumulation: the additional raw-source norm/range/domain restoration, source-query lifting, old-plus-fresh reduction, complete successor witness commitment, final decider and its authenticated finite-history obligations.

Do **not** make the baseline prove the accumulator's auxiliary bounded-short-source relation merely because the native accumulation completeness proof needs it. Existing full PCS soundness can establish the common logical endpoint/evaluation relation without separately asserting the stronger accumulator input contract. If some raw-source predicate is genuinely necessary for that common logical relation, identify it and charge it to both; otherwise it is accumulation overhead.

Similarly, comparing a sparse-cache accumulator against a baseline that needlessly recommits *both* RAM endpoints from scratch is not a fair architectural comparison. The private sparse commitment optimization is available to either proof-delivery design once its backend support exists.

## 9. Actual default memory geometry and measurement points

### 9.1 What the repository does and does not fix

J-defaults declares 32 MiB of heap, a 4096-byte stack, a 128-byte canary, and 4096-byte maxima for input, output, trusted advice and untrusted advice. These are byte-layout defaults, **not a fixed `ram_K` or a selected endpoint commitment geometry**. Program image size and touched addresses remain workload dependent.

J-config computes `remap_address(a)=(a-layout.get_lowest_address())/8` for nonzero a. Its exact observed sizing expression is

```
touched = maximum remapped address from trace RAM accesses, or 0
image_end = remap_address(min_bytecode_address).unwrap_or(0)
            + program_image_len_words + 1
ram_K = next_power_of_two(max(touched, image_end))
```

Report this actual source rule rather than converting 32 MiB/8 into a claimed default polynomial length. A continued run must freeze one sufficient map/capacity across all segments; re-deriving a smaller shape from each segment's touched set would break handle identity and the statement. A complete trace prepass is allowed and its cost should be reported. Any capacity-edge validation required by the chosen frontend remains real work; this document does not infer its correctness just from the sizing expression.

The modern Akita `one_hot_config` default uses **k=4 at every trace length**; the broader geometry type's allowance of k=8 is not the actual default. Akita minimum padded trace length is 4096, and `AkitaChunkProfile::Single` is selected in `derive_from_rows`. These are trace/source defaults, not values of endpoint n_A, L, δ_s, S_B or U. Those require a selected admitted endpoint profile.

`commitment_total_vars` currently takes the maximum of main one-hot geometry and present advice/program candidates. A new endpoint source can be wider and must be included in setup/catalogue provisioning rather than assuming the old trace setup already covers full RAM. Committing under an explicit profile does not establish that the combined old-plus-fresh opening schedule is admitted.

### 9.2 Proper measurements, without invented timings

A future implementation comparison should record, for every measured profile:

1. J/A revision pair, proof field, source class, decoded arity and full physical/raw lengths, b_s/δ_s, d_A/L/J/n_A, d_B/δ_t/n_B/S_B, exact F plan and relation mode. Record setup identity and cold/warm cache state.
2. R, active/padded segment rows, writes and distinct changed coordinates, J_D, K_s, K_t, affected B slices, cancellations, and U. These distinguish a locality effect from a faster cryptographic algorithm.
3. Separate initial source formation/admission, changed-fibre digitization, exact A update, changed-t digitization, exact B update, and full F chain. For the unoptimized reference, measure C-A and C-B stage boundaries rather than just terminal payload serialization.
4. C-F already records per-batch digitization and kernel-including-prepare, input/output/packed/expanded bytes, mode and width; its aggregate report contains peak scratch/current-image, cache-before/after, retained packed witness and quotient-row counts. These are appropriate measurement points, but no report was executed here.
5. Track retained source/t/`hat(t)`/u/F allocations, old/new versions, clone versus unique-consume paths, source-admission scans, setup/NTT allocation, whole trace retention, and peak resident memory. `sizeof(F)` storage, serialized field bytes, packed digits and expanded i8 buffers are distinct quantities.
6. Separately measure ordinary endpoint evaluations/multipoint reductions, raw source-range prefixes, source contractions, D/H paths, complete native fold/strong suffix and successor commitment, final PCS/decider, and wrapper/history. Do not hide these under a single “endpoint update” timer.
7. Use the same sparse update/cache policy for baseline and accumulator. Report both full-field-default and conditionally bounded65 results only if their statement/admission hypotheses are made explicit. Report cold-start totals, steady-state segment costs and total finite-run cost; do not assume sparse workloads or amortize unreported setup.

**Evidence boundary:** the exact linear-difference equations establish a useful private formation algorithm; the source map identifies existing reusable stages and concrete missing state/API support. There are no measured timings or speedups. An implementation still needs a path-preserving backend, exact wider-difference kernel bounds, profile/setup admission, and whole-pipeline accounting before it can claim end-to-end memory-scan savings.
