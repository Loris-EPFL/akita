# Candidate1: ordinary Cf full-grid / local-DAG certificate

## Result and precise scope

This is a read-only mathematical/code review, not exercised verification. No repository files were changed and no build, test, formatter, linter, or proof run was performed. The initial review-role `git diff --stat` attempt failed with exit 129; no patch was obtained, and no patch-introduced bug is asserted here.

**A usable fixed-source LOCAL lemma can be derived for the ordinary `CanonicalCoefficientTable`, E=K branch, using the complete decoded source, not a cropped honest buffer.** It covers packing and degree-one trace, quotient-lift and reduced relations, compressed and raw payloads, the actual Stage-1 helper tree and powers batches, Stage-2 linkage, and the schedule-admitted scalar physical-L2 route. Setup offload additionally needs its separately owned canonical installed-source leaf, as specified below. It does not establish the deeper knowledge extractor, causal source coalescence, or the concrete sponge-to-indexed-ROM compiler.

**The stronger claim that these stages force every physical padding/alignment cell to be zero is false.** A one-cell `-1` example below passes all the local algebraic predicates. The correct native lemma keeps such cells in the full decoded W, range-checks them, and gives them zero *relation weights* where the actual relation has no use for them. It must not replace them by zero when evaluating the authenticated successor leaf.

**Not certified by this report:** the extension `TensorSubfieldProjection` + EOR original-point bridge, the separate `FullRingScalarExtension` compiler, or a claim constraining a successor's larger cube outside the predecessor's zero-extended point slice. These exclusions are explicit, not an implicit specialization of a universal ordinary theorem. The inverse tensor decoder is given below; its existence is not a proof of the EOR prefix's original-point soundness.

The read set includes the shared root protocol, lighter extraction note, sampler addendum, source-first boundary, original-source certificates, and runtime-admission report. No expanded local Akita theorem is used as an extraction theorem. All mathematical probability arguments below are elementary polynomial root counting/first divergence.

All source paths below are repository-relative and line ranges are from the inspected current files.

## 1. Actual production/consumption boundary

The commitment is before the strong anchors:

- `akita/crates/akita-prover/src/protocol/prove/fold/mod.rs:483–568`: `commit_next_witness` binds either an ordinary outer payload or predecessor-owned terminal inner state. Its alignment length is computed from the logical length and the successor A dimension.
- `.../fold/mod.rs:624–703`: commit first, then `ring_switch_finalize`, then Stage 1.
- `.../fold/mod.rs:703–806`: Stage-1 helper claims, L2 batching, compression batching, Stage2 batching/rounds, successor W scalar, then optional Stage 3.
- `akita/crates/akita-verifier/src/fold/mod.rs:91–177`: shell, `receive_next_witness`, alpha/tau0/tau1, Stage 1, Stage-2 round replay, Stage 3, and only then Stage-2 final validation.
- `.../fold/mod.rs:183–235`: explicit dispatch for both `OuterPayload` and `TerminalT`; neither is silently dropped.
- `akita/crates/akita-verifier/src/fold/suffix.rs:103–176,280–324`: the previous `next_witness`, `challenges`, and scalar `opening` become the next incoming source commitment and point/value. Setup is a separate group when present. Packing, degree-one trace, and EOR have distinct dispatches.
- `akita/crates/akita-verifier/src/fold/terminal.rs:15–178`: the last source is A-only, one claim, no pending setup. Terminal checks are direct, not another strong range DAG.

Thus the assumed source cut is a complete decoded successor source fixed causally before the **first current alpha answer**, together with all current shell/payload/public-row inputs. Fixing commitment bytes alone does not satisfy this assumption. The source's weak product/path certificate comes from its complete **deeper** suffix, not the current range proof.

## 2. GRID: decoder, domains, and every cell

### 2.1 Committed source versus local flat table

Let L be `WitnessLayout::live_coeff_len()`, d+ the successor A ring dimension, and

`N = d+ * 2^ceil(log2(ceil(L/d+)))`, `n = log2(N)`.

This is exactly `witness_commitment_domain_len` in `akita/crates/akita-params/src/layout/opening_point.rs:145–166`; `RelationAddressGeometry::new_with_coefficient_block`, `.../layout/relation_address.rs:108–146`, constructs the Boolean domain of live length L and complete length N. The low coefficient block c is the checked common relation block; its low-bit width is log2(c), and the remaining n-log2(c) bits are physical lanes (`relation_address.rs:152–207`). It is not necessarily d+ or the current A dimension.

Let the successor's source block parameters be d+, M+, B+, source digit depth δ+, and source basis b+. The admitted full A-input width is **M+ δ+**, not an invented power-of-two zero-padded width:

- `akita/crates/akita-schedules/src/audit.rs:279–302` requires exact A/B/D widths.
- `akita/crates/akita-params/src/layout/params/precommitted.rs:463–493` enforces the frozen A/B widths and actual B slice geometry.
- `.../layout/params/commitment.rs:95–109` enforces the same executable source A/B widths.

For one complete algebraic A source, retain **every** ring block

`s[b,p,ell](X)`, `0 <= b < B+`, `0 <= p < M+`, `0 <= ell < δ+`,

and all its d+ coefficients. Define its decoded canonical coefficient table by

`S[(b M+ + p)d+ + a] = sum_{ell=0}^{δ+-1} (b+)^ell * coeff_a(s[b,p,ell])`.

For `b >= B+` in the padded block cube, define S=0: these are absent source blocks, not discarded coordinates of a real A block. This decoder is linear and applies to arbitrary field source digits; it does not assume the original source is short or its gadget decomposition canonical. In particular, positions p beyond the declared live ring count **inside a real block**, and coefficients beyond L inside a real ring, remain genuine decoded source coordinates.

The actual scheduled physical opening cube has size

`N+ = d+ M+ next_power_of_two(B+)`.

`recursive_opening_num_vars_for_geometry`, `akita/crates/akita-params/src/layout/params.rs:36–60`, gives its bit count. If it is wider than n, the actual suffix preparers append **zero challenge coordinates**, not a zero-source assertion:

- `akita/crates/akita-types/src/proof/batch.rs:143–201`, especially `padded_point.resize(target_num_vars, zero)` at 176–177;
- `akita/crates/akita-types/src/subring_coefficient_packing.rs:39–134`, especially its exact arity/live-position check and zero extension;
- `akita/crates/akita-params/src/schedule.rs:1022–1071`: admission permits `n <= successor_opening_num_vars`, not only equality.

Consequently the true consumed leaf is

`e = MLE[S](z || 0^(log2(N+)-n)) = MLE[W](z)`,

where `W[i] = S[i]` for **all** `0 <= i < N`. The equality follows from eq(0,0)=1 and eq(0,1)=0 on the appended high address bits. The source extractor must return the full S and its source blocks; W is this deterministic public slice. It must not replace W[L..N] by zeros. When n=log2(N+), W is exactly the entire source cube, including structural absent-block zeros. When the successor is wider, this strong block proves predicates only on the indicated slice; a universal full-successor-cube conclusion would need another contract.

This is a full-table point bridge, not a deduction from a constructor. The constructor in `akita/crates/akita-cpu-backend/src/opaque/recursive/witness/opening_and_flat.rs:111–185,293–367` zero-pads its honest stored buffer; that behavior does not constrain an adversarial algebraic decommitment. `akita/crates/akita-cpu-backend/src/commitment/source/builtins.rs:164–229` distinguishes source live extent from physical commitment extent. Those honest facts cannot be used to crop recovered s or W.

### 2.2 Physical Z/E/T/tail address equations

`RelationRangeImagePlan::new`, `akita/crates/akita-types/src/proof/relation_range_image.rs:409–552`, checks chunk-major/group-major ordering, each group's claim range and block partition, contiguous Z/E/T body spans, the exact tail, and quotient row geometry. `WitnessLayout::validate_internal_ranges`, `akita/crates/akita-params/src/witness.rs:465–517`, includes compression alignment gaps in the complete partition; no cell may simply disappear from the full W.

For a group's unit u, let dA be its A dimension, δi its source inner digit depth, δf its folded-response digit depth, and b its **current common range/response basis**. `WitnessUnitLayout::z_coefficient_index`, `witness.rs:318–359`, is

`i_Z(u,p,ell,f,a) = Z_start(u) + (((p δi + ell) δf + f)dA + a)`.

Thus its physical raw folded response is

`Z_u[p,ell,a] = sum_f b^f W[i_Z(u,p,ell,f,a)]`.

All M δi dA coefficients are used, not just a displayed scalar endpoint, and every replicated chunk has its own response. E and T indices retain the scheduled native role-subcolumn/digit/claim/block axes; their role dimensions may differ from A. Quotient rows use their own `RelationRowGeometry`, not the A dimension indiscriminately. F/H spans retain the complete padded map-input coefficient grid.

The Stage-1 range anchor is the permutation

`t = tau0[col_bits..] || tau0[..col_bits]`,

where col_bits=n-log2(c). `akita/crates/akita-types/src/proof/stage1.rs:28–54` and verifier `stages/stage1.rs:201–215` perform exactly this reordering. Fresh-law coordinate masses must be permuted with the coordinates. The later u and z are already flat low-bit-first points.

### 2.3 Padding/used-slot ownership, not honest zero-padding

1. **Absent logical blocks** are zeros by the decoder's domain definition. A block that exists cannot be turned into an absent block merely because the honest producer left its last positions zero.
2. **Physical padding inside real A blocks/rings** is part of W. Stage 1 range-checks it; ordinary local relations generally do not force it to zero.
3. **Compression alignment gaps** are part of the live flat witness partition, hence range-checked, but are not automatically zero. `witness.rs:742–749` names them zero ranges; no zero-value verification follows from that accessor.
4. **Every F/H padded digit slot** is in negative-binary support: `witness.rs:730–739`; `akita/crates/akita-types/src/proof/compression_relation_weights.rs:56–86`. The actual F/H matrix acts on its complete `input_width * ring_dimension` span, not a matrix with invented zero padding. Only the recomposition gadget's actually used field-bit slots carry recomposition weights (`compression_relation_weights.rs:373–417`). A padding digit is still constrained by its real compression matrix column and the negative-binary alphabet; it need not be canonical zero merely because the recomposition map ignores it.
5. **Global W suffix after L** has zero linear relation/opening/L2 weights where no semantic span owns it, but its range term remains present on the whole N cube. Zero weights and zero source values are different assertions.
6. **Setup source beyond active/natural length** is not zero: its commitment contains the actual full installed shared-matrix prefix (`akita/crates/akita-cpu-backend/src/setup/setup_prefix.rs:111–134`). The setup-index weight is zero outside its real role footprints, as shown by the D/B/A footprint guards in `akita/crates/akita-types/src/setup_contribution/plan/setup_index_weight.rs:143–204`.

## 3. Deterministic source functions for the entire strong DAG

Write `[A]` for the MLE of a Boolean table A. All products in table definitions below are performed **on the cube before interpolation**.

### 3.1 Range image and product helpers

Define

`U[i] = W[i](W[i]+1)` and `P_b(Y) = product_{k=0}^{b/2-1}(Y-k(k+1))`.

Then

`P_b(W[i](W[i]+1)) = product_{k=0}^{b/2-1}(W[i]-k)(W[i]+k+1)`.

For an odd prime field with q>b this vanishes iff W[i] is a field image of an integer in `[-b/2, b/2-1]`. This is exact algebraic membership, not a centered-lift assumption about extracted source gadget digits.

`akita/crates/akita-params/src/layout/digit_range.rs:195–257,326–383,427–447` fixes the following topology:

| b | product arities a_s | child counts k_s | final polynomial degree d | leaf count |
|---|---|---|---|---|
|4|none|none|2|1|
|8|none|none|4|1|
|16|2|2|4|2|
|32|4|4|4|4|
|64|2,4|2,8|4|8|

The leaves are `L_j[i] = f_j(U[i])`, where f_j is the consecutive group of up to four roots in `leaf_coeffs`. Each parent helper is the Boolean product of its a_s child helpers. Each helper is uniquely determined by W, even though its scalar evaluation is supplied later by the prover and is not independently committed.

`[U](u)` is **not** `[W](u)([W](u)+1)`. The former is the Stage-1 helper, the latter occurs inside the Stage-2 integrand. Stage 2 proves their cube-sum linkage.

### 3.2 Native rows and the actual matrices

Decode every current W Z/E/T/quotient/F/H span with its own scheduled gadget. This yields a deterministic native residual for every authenticated `RelationRowFamily`, plus the one scalar opening row. Matrices are the installed actual native row-major views, with their checked ranks/widths/slices. They are not fresh matrices, not a paper-specified matrix with extra zero columns, and not scalarized shell challenges.

For quotient lifting, a native row residual is the polynomial

`R_j(X) = lhs_j(X) - rhs_j(X) - (X^(d_j)+1) Q_j(X)`.

Packing consistency has its selected carrier modulus `X^s+1`, basis planes, and actual E/Q addresses. False native ring equality or false quotient placement implies a nonzero lifted residual polynomial. Its degree is at most `2 d_j - 1` for ordinary products/quotients; packing consistency is at most `2s-1`. A single safe alpha degree is the maximum of these actual row degrees (bounding all native/packing/compression dimensions), denoted D_alpha. There is **no** division by alpha^d+1: a root of that modulus is just an alpha bad-set member, not a forbidden challenge.

For reduced evaluation, the polynomial is the **negacyclic reduction** of lhs-rhs, of degree at most d_j-1. The actual transpose identity is

`H_A(a)[k] = sum_j eq(a,j) sign(k+j) alpha^((k+j) mod d_j)`,

`sum_{j,k} eq(a,j) A[k] W[j] sign(k+j) alpha^((k+j) mod d_j)
 = sum_k A[k] H_A(a)[k]`.

This is the coefficient expansion of multiplication modulo `X^d+1`, including the minus sign on every wrapped coefficient. `akita/crates/akita-algebra/src/ring/residue.rs:197–215` defines this exact kernel; verifier `setup_contribution/reduced_role.rs:55–110` combines the common low coefficient bits with the appropriate role-lane bits. `structured_reduced.rs:175–253` retains full ambient sparse challenge multipliers. Reduced compression separately handles unaligned native windows by two equality windows (`akita-types/src/proof/compression_relation_weights/reduced.rs:107–199`); one equality window cannot be reused for different physical starts.

The actual role/index contractions are source-backed at:

- `akita-types/src/setup_contribution/plan/prepare.rs:44–120,139–277`: per-group D ranges, exact physical B slice geometry, Z width Mδi, row weights, active units, and native projection ratios;
- `akita-verifier/src/setup_contribution/structured.rs:180–280`: E/T challenge products and Z consistency terms;
- `akita-verifier/src/relation/evaluation.rs:363–435`: each noncompression, nonpacking quotient row at its own modulus, digit depth, address and row weight; the skipped F/H and packing rows have their separate consumers;
- `akita-verifier/src/coefficient_packing_relation/compact.rs:389–535,537–632,634–745`: E plane/digit/block/claim affine and direct-opening weights, packing quotient addresses, and Z position/source-digit/fold-digit/carrier/embedding-stride axes. They cover exactly all their unit spans, not a scalarized packed endpoint;
- `akita-verifier/src/stages/opening_semantics.rs:44–131`: packing scalar targets use authenticated per-group coefficients; trace uses its separate scalar-row selector and native trace weight;
- `akita-verifier/src/relation/evaluation_trace.rs:416–513`: exact live block/claim E-segment trace support.

For E=K, trace recovery has degree-one coefficients and no EOR. The original application point/basis and each group's prepared axes remain separate; internal successor points use propagated `BasisMode::Lagrange`. This lemma authenticates the current **batched native scalar row**. It does not turn that row into individually true incoming opening claims; the incoming tensor/scalar-prefix source cut has its own error and ownership.

Let R be the sum of all linear native relation/packing/trace weights on the flat N cube, evaluated using the installed actual setup, alpha, tau1, current point/basis, fixed claim coefficients and shell. Let T be its public batched target. Equivalently,

`delta_rel = sum_i W[i] R[i] - T
           = sum_j eq(tau1,j) R_j(alpha)`,

where the scalar opening discrepancy is the trailing row and unused row addresses are zero. The equality is obtained by expanding the above address/gadget/matrix contractions; it is not an assertion that a constructor produced an honest witness. For reduced rows the expansion uses the signed-wrap transpose identity instead of a quotient tail. For packing rows it uses the actual plane basis and carrier coordinates above. This establishes the operational relation/native-row bridge for the admitted E=K canonical branch.

### 3.3 Binary support and physical response functions

Let I be all F/H span intervals. Define the binary residual table

`B[i] = 1_I(i) W[i](W[i]+1)`.

Its true anchor is `[B](u)`. The terminal Stage-2 binary weight is

`K_I(u,z) = sum_{i in I} eq(u,i) eq(z,i)`.

It is a **single** MLE, not `[1_I](z) eq(u,z)`. `compression_relation_weights.rs:105–132` implements this distinction. Every support slot must be 0 or -1 if B is identically zero.

For physical L2, every Z unit contributes a segment, including its full matrix-column/source-digit/coefficient geometry. `PhysicalResponsePlan::new`, `relation_range_image.rs:53–128`, rejects multi-group/precommitted L2, verifies δf*dA divisibility, and matches the complete physical length. A physical row r and coefficient a map to

`V_f[p_u + r dA + a] = W[z_u + r δf dA + f dA + a]`.

In direct mode `V = sum_f b^f V_f`; in LimbGram mode the V_f are separately retained. Each V table has zero *virtual-table* padding beyond its complete physical-response length. Its MLE is bound to W by the three paired axes in `relation_range_image.rs:283–299`:

`(coefficient: left stride 1, right stride 1, length dA)`,
`(fold digit: witness stride dA, physical stride 0)`,
`(physical row: witness stride δf dA, physical stride dA)`.

This accounts for the full matrix column grid, not just a logical main-source coefficient table. Stage-2 virtual coefficients are eta^(j+1), never eta^j (`relation_range_image.rs:17–36`), so a relation discrepancy cannot deterministically cancel an arbitrary virtual discrepancy using the constant coefficient.

## 4. LOCAL proof by first divergence, including adaptive helpers

Assume the full source S and hence W, fixed public rows/shell/payloads, and (when selected) the canonical installed setup reference are causally fixed before the relevant strong root. Assume the complete deeper procedure authenticates the original branch's final W(z), and every separately owned setup leaf, under the decoder/point bridge above. No assumption is made that intermediate helpers, round polynomials, norm claims, or scalar batches are honest. They may be fully adaptive.

### 4.1 Anchors and range propagation

If any W digit is out of range, the Boolean table `P_b(U)` is nonzero. Its multilinear anchor at the reordered tau0 point can vanish with probability at most the sum of the n fresh coordinate maximum atoms. This uses a nonzero table, not the degree b of its defining scalar polynomial.

At each product stage the true child helpers are the deterministic source functions defined in §3.1. A false weighted parent input cannot become a true endpoint unless a nonzero round discrepancy vanishes at one fresh round challenge. Otherwise the accepted endpoint product of alleged child claims requires at least one false child evaluation. The verifier receives all k_s child claims **before** gamma_s (`akita-verifier/src/stages/stage1.rs:70–109`); the discrepancy of the powers batch is a nonzero polynomial of degree <=k_s-1. Outside that root set a false claim therefore propagates to the next stage. This argument works when gamma=0, when some weights vanish, or when the batched leaf's leading coefficient vanishes; those cases are in the same polynomial cancellation set, not implicitly rejected.

The final non-L2 leaf proves the weighted polynomial \(f([U](x))\) with its scheduled coefficients, then receives the alleged \([U](u)\). If all previous discrepancies survive, either a leaf-round bad event occurs or that alleged value is false. No tree interpolation is needed.

The equality-factored codec has **no division**: `akita-sumcheck/src/single.rs:7–16` sets the missing constant to `claim - tau * sum(nonconstant_coefficients)` and evaluates additively. Its normalized discrepancy degree is a_s (or final leaf d); using the safe full-degree a_s+1 / d+1 ledger below also covers it. There is no nonzero tau/eq assumption and no missing exceptional branch at tau=0 or 1. `akita-sumcheck/src/proof_stream.rs:418–466` fixes coefficient counts before reading a challenge and routes each round to the scheduled challenge consumer.

### 4.2 Reused u binary anchor

If B is nonzero, `[B](u)=0` costs at most the sum of the n final-leaf coordinate maximum atoms. Although u is also used in the range/norm protocol and the adversary's earlier messages can be adaptive, B is fixed by W before **every** u coordinate. We count this bad event on the unconditional causally fresh experiment, not conditional on accepting the range proof or on source equality. Thus no independence between “range accepts” and “binary accepts” is required.

After Stage-1 values are fixed, the true Stage-2 input discrepancy is

`Delta(lambda,xi,eta) = delta_rel + xi [B](u)
                      + lambda (claimed_U - [U](u))
                      + sum_j eta^(j+1) (claimed_V_j - [V_j](u))`.

Absent terms are omitted. The verifier's signs may reverse the whole discrepancy, which does not change its zero set. CompressionBinary xi and Stage2Batch lambda are sampled **after** Stage-1 values (`stages/stage1.rs:247–298`). The eta powers reserve the constant term. If any of the coefficients is nonzero, sequential root counting gives charges degree v for eta, 1 for xi, and 1 for lambda, as applicable. A zero batch coefficient does not remove mandatory constant relation/opening terms. The actual zero-lambda branch in `stages/stage2.rs:316–335` simply omits a zero virtual addend; it is correctly accounted for, not a deterministic escape.

### 4.3 Stage-2 cubic and delayed setup dependence

With R as above, the true Stage-2 integrand is

`F(x) = W(x) R(x)
      + xi K_I(u,x) W(x)(W(x)+1)
      + lambda eq(u,x) W(x)(W(x)+1)
      + W(x) H_eta(u,x)`.

R already includes all packing/trace/linear compression contributions. H_eta is the physical-L2 paired-family weight, zero if absent. R, K_I, eq, and H_eta are multilinear in x; F has individual degree <=3. Summing over the cube gives exactly the true relation target plus the true U/binary/virtual anchors. `akita-verifier/src/stages/stage2.rs:242–335,356–463` consumes every addend, obtains W(z), and checks the cubic endpoint. A surviving nonzero input discrepancy therefore costs at most three times the sum of the Stage-2 coordinate maximum atoms, provided W(z) is the owned true full-table leaf.

For setup offload the relation endpoint is **not** assumed true before Stage 3. `fold/mod.rs:146–170` replays Stage3 before `validate_stage2_replay`. Stage3 binds the installed slot and proves

`claim_setup = sum_j S_installed[j] T_z,alpha,tau1[j]`,

where S_installed is the actual complete registry-owned shared-matrix prefix and T has its checked native role footprints. `stages/stage3.rs:50–74,80–135,145–205` uses degree-two rounds and finishes against the separately owned setup source evaluation times the setup-index weight and alpha-power MLE. Outside its degree-two round root events, its claim equals the direct installed-setup contraction. Therefore replacing Stage2's deferred setup scalar by the real R endpoint is justified **after** this part of the DAG. A trusted commitment handle alone does not prove its source equals S_installed: the deeper comparison must include the canonical reference occurrence and its actual source decomposition/path bounds. An unknown setup source cannot be treated as free advice.

### 4.4 L2 integer truth is a consequence, not a source-comparison premise

If the range is true, every W Z digit has its unique balanced integer lift. Define the direct norm as the sum of squares of the complete recomposed physical response, or define every Gram subclaim

`G_(block,a,b) = sum_{t in block} V_a[t] V_b[t]`.

For LimbGram, the claims are sent before gamma; its powers batch has degree J-1, where J is the actual block-major upper-triangular subclaim count. Direct mode has no subclaim batch. The norm/range merge has degree 1. The fused leaf has the **actual full** per-round degree d+1; every false endpoint must then reside in the alleged U/V scalars or a sumcheck root event. Stage2 eta linkage exposes the scalars with degree v, where v=1 for direct and v=δf for LimbGram. This yields direct scalar charges 2, and LimbGram charges `(J-1)+1+δf = J+δf`, in addition to the fused leaf rounds.

`akita-verifier/src/stages/physical_l2_norm.rs:145–217,244–333` checks the base modulus, energy cap, subclaim/virtual count, and centered integer lift. `akita/crates/akita-params/src/sis/physical_l2.rs:252–314` requires:

- direct: `P * ((b/2) sum_f b^f)^2 < q`;
- LimbGram: `block_len * (b/2)^2 < floor(q/2)` and limb_count=δf.

Thus equal field claims yield equal integers only **after** range truth: direct sums cannot wrap, and each Gram sum has a unique centered lift. `reconstruct_l2_sq_from_gram`, `relation_range_image.rs:309–366`, uses diagonal weight b^(2a), off-diagonal weight 2 b^(a+b), all blocks, and checked i128 arithmetic. Overflow or a negative final integer rejects. This certifies the actual physical response energy, not an honest digit moment, not the full source norm, and not a bound on an inverse shell difference.

## 5. Explicit finite error ledger and actual fresh laws

### 5.1 Bound in maximum-atom form

For each fresh challenge coordinate c in the indicated causal experiment, let mu_c be the supremum of its conditional point masses **before its answer is exposed**, including the exact public prior transcript, but not conditioning on later acceptance/source equality. For a nonzero degree-D univariate residual, the root probability is <=min(1,D mu_c). For multilinear coordinates the coefficient-induction argument gives the sum of their mu values.

Let beta=1 iff compressed F/H support exists; r=actual relation-row selector bit count including the trailing scalar row; tS=actual number of Stage3 rounds (zero if direct); J and v as above. One conservative explicit bound is

`epsilon_local <= min(1,
 D_alpha mu_alpha
 + sum_{tau1 coordinates} mu_c
 + sum_{tau0 coordinates} mu_c
 + sum_product_stages [(a_s+1) sum_{rounds s} mu_c + (k_s-1) mu_gamma_s]
 + (d+1) sum_{final_leaf rounds} mu_c
 + beta [sum_{final_leaf rounds} mu_c + mu_xi]
 + mu_lambda
 + 3 sum_{Stage2 rounds} mu_c
 + L2 [1_Gram (J-1) mu_gamma_norm + mu_norm_merge + v mu_eta]
 + 2 sum_{Stage3 rounds} mu_c )`.

The safe final-leaf d+1 works for both the normalized non-L2 codec and the full physical-L2 codec. The normalized non-L2 leaf and product rounds may be sharpened to d and a_s respectively using the additive recurrence, but no sharpening is required for this certificate. The binary anchor uses the **same leaf draws**; the two ledger occurrences are a union bound, not a claim of independent draws.

Under fresh independent uniform field answers this specializes to

`epsilon_local <= min(1, L_local/q)`,

`L_local = D_alpha + r + n
          + n sum_s(a_s+1) + sum_s(k_s-1)
          + n(d+1) + beta(n+1) + 1 + 3n
          + L2[1_Gram(J-1)+1+v] + 2tS`.

This is a finite, explicit conservative bound, not a tight end-to-end security-bit claim. It covers a false actual range predicate, actual native ring/row/weighted-point predicate, F/H negative-binary predicate, or admitted physical integer norm predicate, with owned true leaves. It does **not** cover a nonexistent all-padding-zero check or individually unbatched incoming claims.

For the actual canonical Dense nv14 root row (`akita/artifacts/schedules/fp128_dense.aks:128–134`): L=470976, d+=256, N=524288, n=19, range b=8/no product prefix/d=4, compressed, Linf, no setup. The safe D_alpha<=1023 (maximum actual A dimension512), giving `L_local <= 1215+r`; r is the actual checked row width, not an invented paper row count. Later levels must use their own selected topology and physical-L2 shape.

### 5.2 What the executable sampler establishes, and what it does not

`akita/crates/akita-transcript/src/proof_stream/sampling.rs:41–64` does canonical rejection, **not modular reduction**: squeeze canonical-width bytes, mask excess high bits, accept iff `from_bytes_le_checked` succeeds, otherwise retry. If the fresh input candidate strings are independent uniform h-bit strings, where h=MODULUS_BITS and q is the base-field order, then for any y in K

`Pr[output=y] = sum_{a>=0} (1-q/2^h)^a * 2^-h = 1/q`.

Rejected candidates are part of the raw history; they are not discarded from a retained full-answer fork law. Per base coordinate the expected attempts are 2^h/q<2 for a genuinely h-bit prime. A finite cap R adds abort mass `(1-q/2^h)^R` per coordinate. The Keccak cfg additionally checked-counts attempts in u32 (`sampling.rs:48–56`), so its exceedingly remote overflow is an explicit abort event, not automatic exact uniformity conditional on no abort. `proof_stream.rs:596–619` samples each extension coordinate separately at a distinct limb site; E=K has one coordinate.

This derivation supplies the exact decoded law **given the fresh uniform candidate-stream interface**. It does not prove that an arbitrary concrete duplex has that interface. The actual sponge alias is `spongefish::instantiations::Blake2b512` or `Keccak` (`akita-transcript/src/sponge.rs:3–8`). Domain records and site coordinates are sent to the actual channel; they are not themselves an implementation of a resettable indexed ideal oracle.

Nonzero PoW sites first absorb/read a prover nonce and test a separate predicate, then draw the extension challenge (`akita-types/src/transcript_grinding/replay.rs:174–234,478–499`). The prover can search candidate nonces. Therefore the **selected accepted Fiat–Shamir challenge** must not simply be called an independent uniform draw. The local bound is at the causally fresh raw candidate/key level. To transport it to the original indexed-ROM experiment use the actual per-first-hit/attempt accounting (e.g. q*epsilon_local plus the separately proved old-key term in the lighter note), or prove the exact conditional nonce/full-answer law. No 2^-grind discount or planner logical-query ceiling is silently inserted here. This is the precise remaining sampler/compiler prerequisite, not a request to change the transcript.

## 6. Smallest obstruction to an overstrong zero-padding GRID lemma

Take any parent with L<N and a real decoded successor source cell j in `[L,N)` (not an absent-block structural zero). Preserve all semantic W[0..L] from a valid local witness. Change exactly W[j] from 0 to -1, and commit the actual changed full source instead of pretending its padding is zero.

- Its range image is unchanged at that cell: `(-1)((-1)+1)=0`, just as for zero.
- Every parent semantic linear relation/opening weight at j is zero, because j is outside its complete live Z/E/T/quotient/F/H partition.
- It is outside the F/H binary support and physical-L2 witness Z maps.
- Hence the range helpers, every native row residual, and the Stage-2 cube-sum target are unchanged. Honest mathematical sumcheck messages for the changed **full** W, with its true changed W(z), satisfy all local verifier equations.

The changed W(z) is what its complete deeper continuation must authenticate. No A/B/F collision is used: Cf was allowed to be produced before alpha and can be the commitment to this different source. This is an algebraic counterexample to a purported local zero-padding theorem, not a claim that the honest backend exposes a constructor for a malicious witness or an exercised byte-level forgery.

It occurs in real geometry, not just a toy power-of-two model: Dense root L=470976, successor d+=256, M+=128, B+=15 gives `B+ M+ d+=491520`, within N=524288. Cell j=L is in the last *real* successor A block and is not a structural absent-block address. The decoded source has 20544 real source coordinates after L before the absent-block zeros begin. The source digit depth is1, so setting this one source coefficient to -1 is well-defined without any gadget ambiguity. See `fp128_dense.aks:130`, the full row, and the domain equations above.

The same logical obstruction applies to any genuine compression alignment gap if the actual weight supports omit it. The exact reach of that example depends on a row with a nonempty gap; no such row was invented or run here.

**Actionable smallest contract:** either (A) formulate LOCAL on the actual full W, with arbitrary in-range real padding and its true leaf, as proved above; or (B) if the intended relation truly requires those cells zero, authenticate an explicit zero-cell residual under the same full W and include its finite anchor/batch error. The existing range polynomial alone cannot do (B). For analysis-only Candidate1, (A) avoids changing protocol bytes, but every subsequent source/point/collision calculation must keep those coordinates instead of substituting a cropped buffer.

## 7. Extension/EOR and terminal boundaries retained explicitly

### Tensor decoder

`CommittedSourceEncoding::for_producer`, `akita-params/src/schedule/profiles.rs:25–44`, selects tensor projection for nonroot extension-degree>1 evaluation-trace consumers; packing requires canonical encoding (`schedule.rs:971–988`). `akita-cpu-backend/src/arithmetic/field_reduction.rs:24–34` packs via the actual psi layout, sometimes placing nonzero honest coefficients beyond the original *physical* live index (`:77–101`). Cropping physical coefficients at L is especially invalid here.

For d and extension degree k, put h=d/(2k). The inverse of psi in `akita-types/src/field_reduction.rs:530–561` recovers each residue a<h and low/high slot:

`coord0_low = v[a]`, `coord0_high = v[a+k h]`,

`coordj_low = (v[a+j h] - v[a+(2k-j)h])/2`,

`coordj_high = (v[a+(k-j)h] + v[a+(k+j)h])/2`, `1<=j<k`.

This partitions every physical ring coefficient and is invertible for odd characteristic. Flatten the resulting slots in low/high psi order, with the head coordinate fastest; apply it to the **full recomposed source ring**, not a short honest digit buffer. This yields the complete original base-coordinate W table. The EOR native polynomial table instead groups those head coordinates into extension values. Thus its reduced endpoint is not automatically the original W(z).

A full extension certificate additionally must establish that the actual EOR tensor prefix authenticates the **original** point through its exact factors/extra axes (and that the selected extension really is a field with the `FpExtEncoding` basis). `fold/suffix.rs:167–184` actually dispatches EOR; its factors are not silently set to one, assumed nonzero, or inverted here. The canonical E=K lemma above does not certify that prefix. These are precise additional contracts if the proposed final schedule selects this variant; a known A7F7 E2 ring is not admitted as a field merely because it implements the Rust trait.

### Terminal

`akita-verifier/src/terminal/direct.rs:131–275` directly decodes the bounded Golomb-Rice response and checks exact A rows, native multiplier/gadget consistency, all raw response dimensions, and the selected squared-L2 cap. `fold/terminal.rs:31–62,114–178` enforces a singleton A-only incoming state, no setup, coefficient-width checks, pre-shell E fields, actual shell draw/filter, bounded Z payload and final scalar trace. The Linf cap is the terminal's direct response cap, not a fictitious δf range depth; L2 uses the checked raw integer energy. This direct boundary is the suffix induction base. It is not counted as another Stage-1/Stage-2 LOCAL block and gives no extra zero-padding theorem or short-inverse bound.

## 8. Integration conclusion

For an E=K canonical selected ordinary row, with complete deeper source and original-point leaves in hand and the admitted fresh-answer interface, the full strong-DAG interpolation tree may be replaced by the fixed-source LOCAL proof above. The source decoder is explicit; its W includes every actual physical/virtual address on the checked cube, all helpers are deterministic source functions, and the finite error ledger includes first divergence, interstage powers, zero coefficients, binary anchoring at reused u, physical L2 and delayed setup validation.

Do not integrate an unconditional assertion that accepted continuations yield the honest zero-padded buffer, that a wider successor cube is completely scanned, that a tensor/EOR reduced endpoint already authenticates the original W point, or that concrete nonce-selected sponge challenges are independent uniform merely because the field decoder is canonical rejection. The one-cell padding counterexample refutes the first assertion; the exact slice equation, inverse-psi/EOR boundary, and causal raw-answer accounting specify the smallest remaining contracts for the others.
