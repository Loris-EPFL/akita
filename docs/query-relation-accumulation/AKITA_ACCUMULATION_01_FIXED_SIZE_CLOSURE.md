# Akita accumulation: fixed-size closure and receiving-profile admission

## 1. Obligation and result classification

**Target.** One fixed-capacity private accumulator opening, plus a bounded batch of fresh original segment openings, must produce one successor of the same shape. Bound **all** live vectors, commitment handles, relation rows, setup obligations, and the setup catalogue independently of the number of updates $T$. Count the complete Akita Equation (109) successor, not only its folded responses. Preserve the original incoming commitment geometry.

**Proposed solution.** Provision a finite receiver profile offline, derive its exact successor and matrix inventory, and admit it only with an integer **shape certificate**, an all-valid-source **encoding/completeness certificate**, and separate **security/composition certificates**. Use a coefficient-packed identity-source receiver, direct setup evaluation, an unchunked coefficient-bound fold, and quotient-free ring checking as the simplest proposed native cut. Compression is a declared option, not free shrinkage. This is a new receiving topology, not an existing admissible Akita schedule.

The deliverables here are:

- **Proved algebra:** exact coordinate accounting; deterministic response envelopes; a sufficient fixed-depth fixed-point inequality; a concrete, reproducible shape-only example, including both compression stages and every padding span.
- **Conditional construction:** native old-plus-fresh re-entry preserves bounded state if the declared source predicates, domain certificates, original-handle bridge, and local reductions hold. Ordinary Akita identities justify the local equations, not the new chain theorem.
- **Open:** a security-admissible useful receiver for the actual zkVM source catalogue; strong all-decider-valid source semantics without expensive independent certification; modular adaptive polynomial-time extraction; and noninteractive quantum security. The numerical example is **not** security-certified and has an explicit challenge-entropy obstruction to a 128-bit claim.

See [strong completeness](AKITA_ACCUMULATION_02_STRONG_COMPLETENESS.md) and [commitment consistency](AKITA_ACCUMULATION_03_COMMITMENT_CONSISTENCY.md) for the exact domain and original-handle obligations. No implementation, source edit, or branch change is proposed by this document.

## 2. Source-backed constraints that shape the solution

1. **Equation (109) is the entire successor.** It contains response, partial-opening, inner-image, main quotient, group-local F-chain, shared H-chain, and stage-local quotient digits. Scheduled alignment and final zeros are committed cells even though the display suppresses them. Akita §5.4, qmd `line:2307-2376` [A].
2. **Incoming commitments cannot be reshaped at opening time.** Rings, ranks, source layout, block split, slices, and compression views of an already formed commitment are frozen. The planner must include compression and setup continuation before scoring a candidate. Akita §12.2, `line:4641-4662` [A].
3. **Bounded decoded values need not bound arbitrary gadget sources.** Identity encoding is available for a bounded recursive digit witness, but trace packing can enlarge coefficient magnitudes; canonical source state is not what Akita's weak extractor promises. Akita §§3.1, 4.5, Remark 10.32, `line:867-996`, `1825-2026`, `4141-4250` [A].
4. **The existing recursion is not streaming old-plus-fresh accumulation.** Application groups occur only at the root; later inputs are the recursive witness, optionally with one registered setup source. Quotient-free coefficient-packing re-entry is also outside the analyzed schedule family, although the local identities are stated more generally. Akita §9.3, `line:3194-3245` [A].
5. **Range auxiliaries cannot simply become the next enlarged witness.** LF+ §4.4 explicitly identifies repeated $dk$ witness blowup from flattening its range matrix, and uses commitment transformation instead. Its §5.2 returns two bounded instances. SuperNeo §7.5 returns a fixed $k$-tuple after signed decomposition. These are alternatives, not an already proved Akita conversion. qmd LF+ `line:634-743`, `1164-1193`; SuperNeo `line:1117-1154` [L, N].
6. **Rows must be retired, not just summed.** SALSAA §4.3 replaces batched bottom rows by bounded evaluation rows; its §7.2 bounds both output columns and rows in its own relation. A scalar sumcheck endpoint alone does not justify retaining an ever-growing old constraint matrix. qmd `line:435-528`, `1297-1362` [S].

These observations rule out both “make $N_*$ sufficiently large” without closure analysis and “the commitment is 128 bytes, so the accumulator is constant-size” without counting private digits and setup.

## 3. Receiver profile, source domain, and fixed state

### 3.1 Units and receiver geometry

Let the base field be $\mathbb F_q$, evaluation field $E/\mathbb F_q$ have degree $k$, and receiver ring be

$$R_{q,d_*}=\mathbb F_q[X]/(X^{d_*}+1).$$

**$N_*$ always counts base-field cells**, not ring elements or extension-field values. Choose $N_*=2^{\mu_*}$ and power-of-two $M_*$ with $M_*d_*\mid N_*$. One old accumulator vector then has

$$P_*=1,\qquad B_*(N_*)=\frac{N_*}{M_*d_*},\qquad m_{A,*}=M_*,\qquad \delta_{\mathrm{com},*}=1.$$

The divisibility requirement avoids undisclosed physical cells in a final source block. If it is relaxed, replace $N_*$ by the full physical grid $M_*d_*\lceil N_*/(M_*d_*)\rceil$ in the domain, norm, and state accounting. Zero conventions of an honest encoder are not domain certificates.

Coefficient packing is the bijection

$$\operatorname{Pack}(w)_i=\sum_{j<d_*}w_{id_*+j}X^j.$$

For the opening method select $d_{\mathrm{car},*}$ with $k d_{\mathrm{car},*}\mid d_*$, put $h_*=d_*/(k d_{\mathrm{car},*})$, and use the **same** challenge through $Y\mapsto X^{kh_*}$. Each partial has base-field width $d_{\mathrm{op},*}=k d_{\mathrm{car},*}$. The original points and coefficient-axis permutation remain bound to their corresponding groups. These are Akita Equations (84)–(97), qmd `line:2125-2228` [A]. Merely making $E$ larger can force different compatible carrier/A geometry; it is not a free scalar security upgrade.

For an evaluation-trace alternative, $d_{\mathrm{op},g}=d_{A,g}$. Its two-bank embedding has worst-case coefficient expansion 2 and energy expansion 2 on integer lifts. The logical alphabet $\mathcal A_{b_*}$ therefore does not automatically give the same ring-coefficient bound. Use the exact map, not the planner's modeled local second moment. Akita Equations (45)–(46), (340)–(342) [A].

### 3.2 Frozen profile

The transcript-bound profile $\mathcal P$ fixes:

- $q,E$, ring/extension bases, source maps, challenge-subring embeddings, all coefficient/address permutations, and lifts;
- $N_*,M_*,b_*$, each admitted fresh source shape, maximum fresh groups and source-vector counts, and maximum segment length;
- original A/B/F views, receiver A/B/F views, B slice counts, shared D/H views, response/opening/quotient radices and depths, compression mode, every segment address and zero span;
- accepted challenge family and difference/unit bounds, witness-independent filters, raw-draw limits, nonce/query budgets, norm route and any integer-energy certificate;
- finite matrix/source-comparison catalogue, setup seed/expansion rules, direct or offloaded setup policy, terminal authentication, $T_{\max}$, and application/session identity.

All of these are held fixed at every update. New commitments, points, claimed values, counters, and machine boundaries vary. An input chooses a previously admitted typed slot; it cannot propose a new ring, source map, or commitment profile after the challenge. A profile family is acceptable only if its finite catalogue and worst-case envelopes are frozen before use and its selection is transcript-bound.

When admitted fresh types produce different successor segment/padding layouts, retain a bounded $\mathrm{layout}_t$ tag (or its equivalent authenticated descriptor inside $\mathcal P_{id}$) identifying the exact output layout. The step verifier derives it from the admitted input; it is not prover-chosen unchecked metadata. It ranges over a finite provisioned catalogue, preserves the same receiver commitment geometry and $N_*$, and determines the domain's reserved-zero spans. A one-layout profile such as the numerical example needs no additional tag. Without either this tag or genuinely fixed segment spans, a final domain checker could not know which advertised padding cells must vanish.

### 3.3 Semantic domain

The intended old relation is

$$\begin{aligned}
R_{\mathrm{dom}}(a;w,d)\Longleftrightarrow{}& |w|=N_*,\quad w_j\in\mathcal A_{b_*}:=\{-b_*/2,\ldots,b_*/2-1\},\\
&\text{all declared reserved zero spans vanish},\\
&\operatorname{Rel}_{\mathrm{com},\mathcal P}(C;\operatorname{Pack}(w),d),\\
&\widetilde w(r)=v.
\end{aligned}$$

Here $d$ includes the **complete linked original receiver** inner-image and compression opening state, not a commitment to a separately chosen replacement vector. In the identity-source receiver, bounding every coefficient of $w$ bounds every old source block. Fresh full-field or one-hot sources still need a same-original-handle bounded-source predicate, and natural-domain padding constraints where their application semantics require them. An arbitrary weak Akita decommitment does not satisfy this predicate just because a response was bounded.

Public state is $a_t=(\mathcal P_{id},t,\mathrm{epoch},C_t,r_t,v_t,h_t,\mathrm{boundary}_t)$, with bounded boundary/IO descriptors. The native pending successor is an obligation, not an asserted proved evaluation. The prover retains $w_t$ and bounded linked hints. A final reveal of $(w_t,d_t)$ with deterministic domain/commitment/evaluation checks can certify this endpoint, at fixed-$N_*$ work and disclosure. **Final-only certification does not authenticate hypothetical rewound child endpoints or prove the long-chain extraction theorem.** Independently completed static certificates at each update are a stronger alternative: transparent reveal/check or a separately complete/extractable private argument. Charge their work and error; if they check the entire opening each time, this becomes a fully checked refresh baseline rather than the target deferred native speedup. The completeness document makes this distinction explicit.

## 4. Exact shape compiler: every segment and every matrix

### 4.1 Main segments

For each incoming group $g$, preserve its original data $(P_g,B_g,M_g,\delta_{\mathrm{com},g},d_{A,g},n_{A,g},S_{B,g},d_{B,g})$. Define

$$m_{A,g}=M_g\delta_{\mathrm{com},g},\qquad B_g=\left\lceil\frac{L_g^{\mathrm{physical}}}{M_gd_{A,g}}\right\rceil.$$

Count

$$\begin{aligned}
L_{z,g}&=m_{A,g}d_{A,g}\delta_{f,g},\\
L_{e,g}&=P_gB_g\delta_{\mathrm{open},g}d_{\mathrm{op},g},\\
L_{t,g}&=P_gB_g\delta_{\mathrm{open},g}n_{A,g}d_{A,g}.
\end{aligned}$$

A response has **no claim axis**. Every claim/block pair does contribute partials and inner images. Native segment dimensions and actual digit depths are retained: do not round each depth to a power of two or pad each main segment to the largest ring. Equation (107) changes coefficient, digit, then native subcolumn; Equation (108) fixes response source-plane and fold-plane order [A].

The shared D map is unsliced:

$$m_D=\sum_g\frac{L_{e,g}}{d_D},$$

with the necessary divisibility conditions. It cannot be assigned a guessed constant width independent of $B_*(N_*)$.

Original sliced B has

$$m_{B,g}=P_g\left\lceil\frac{B_g}{S_{B,g}}\right\rceil n_{A,g}\frac{d_{A,g}}{d_{B,g}}\delta_{\mathrm{open},g}.$$

Its logical outer image has $s_{F,g,0}=S_{B,g}n_{B,g}d_{B,g}$ field coefficients. Slice-padding cells are inside each polynomial-major slice input; they are not an extra copied successor image vector. They still affect matrix widths, exact row weights, and zero constraints. Each slice contributes $n_{B,g}$ relation rows. Akita Equations (72)–(74) [A].

### 4.2 Quotients, if selected

On quotient lifting, enumerate **every** active native ring row $r$, with its own dimension $d_r$, base-field plane count $p_r$, and quotient depth $\delta_{Q,r}$. Its allocation is

$$L_{Q,r}=p_rd_r\delta_{Q,r}.$$

Main quotient rows, in order, are: for each group, one fold-evaluation row ($p_r=k,d_r=d_{\mathrm{car},g}$ under packing; $p_r=1,d_r=d_{A,g}$ under trace), $n_{A,g}$ A rows, $S_{B,g}n_{B,g}$ B rows, then the $n_D$ shared D rows. The scalar opening row has no quotient. Each F/H stage then carries quotients for its own map rows immediately after that stage's binary digits. Do not change a carrier modulus from $Y^{d_{\mathrm{car}}}+1$ to $X^{kd_{\mathrm{car}}}+1$ when flattening its field planes.

Quotient coefficients are field values. Decompose each fixed plane under an audited lift/depth covering that value space; their carry alphabets must fit $\mathcal A_{b_*}$. If optimizing their depth with integer product bounds, prove that bound for every admitted row/source. The simple sufficient choice covers all centered field coefficients. On **quotient-free** checking these allocations are exactly zero for all rows, including compression rows; transpose-convolution weights still check their native ring reductions. This is a justified absence, not a dropped check. Akita §7.3, `line:2727-2946` [A].

### 4.3 Both F/H compression stages

Put $\delta_2=\lceil\log_2q\rceil$. For a compression map at dimension $d_{K,a}$, define $\operatorname{up}_d(x)=d\lceil x/d\rceil$ and

$$\begin{aligned}
L_{F,g,a}&=\operatorname{up}_{d_{F,g,a}}(\delta_2s_{F,g,a-1}),&s_{F,g,a}&=n_{F,g,a}d_{F,g,a},\\
L_{H,a}&=\operatorname{up}_{d_{H,a}}(\delta_2s_{H,a-1}),&s_{H,a}&=n_{H,a}d_{H,a},\\
s_{H,0}&=n_Dd_D.
\end{aligned}$$

Widths are $m_{F,g,a}=L_{F,g,a}/d_{F,g,a}$ and $m_{H,a}=L_{H,a}/d_{H,a}$. Each term includes the completion zeros filling its last ring. Compression mode is 0 or **exactly two** stages on both sides. At mode zero all F/H digit segments are absent and the complete raw images are public payloads. At mode two the successor has, for $a=1,2$, group F digits in group order, then shared H digits, then that stage's quotient digits. The binary constraint applies to each complete digit span, including completion cells; range alone is not enough for collision diameter one.

These digits authenticate **incoming** handles and the shared opening payload; do not recursively insert the successor commitment's own compression digits into the witness committed by that same commitment. Those successor hints are retained by the prover and become incoming-chain digits at the next update. This avoids a circular commitment definition. Akita §4.5, Equations (75)–(77) [A].

### 4.4 Alignment, padding, and exact compiler order

Compute addresses with a cursor. Append the main segments in group order, then main quotients. If compression is active, align the suffix start to the common noncompression relation block (the smallest polynomial-modulus dimension among noncompression rows); align each stage start to the largest ring dimension in that stage; append its F/H digits and quotients. Align the suffix end to the largest group A dimension for multiple groups, or to the common relation block for one group. Preserve the order of Equation (109), not a global dimension sort.

Then complete successor receiver rings and append the final zero suffix to **exactly** $N_*$. The compiler emits all zero intervals and the Boolean-domain padding predicate. Let $L_{\mathrm{aligned}}$ include every preceding cell and $L_{\mathrm{final}}=N_*-L_{\mathrm{aligned}}$. Reject if $L_{\mathrm{aligned}}>N_*$. Capacity is

$$\Phi_{\mathcal P}(N_*,S)=\sum_g(L_{z,g}+L_{e,g}+L_{t,g})+\sum_rL_{Q,r}+\sum_{a=1}^2\left(\sum_gL_{F,g,a}+L_{H,a}\right)+L_{\mathrm{alignment}}\le N_*.$$

Inactive terms are zero. Final padding is the slack, not a second capacity charge. Response chunking is intentionally disabled; if enabled, enumerate response/image segments per actual chunk using §11, and bound the number of chunks independently of $T$. VM segments are not response chunks.

## 5. Deterministic completeness and an actionable fixed point

### 5.1 All-valid-source response depths

For every source occurrence $a=(g,c,i)$ require coefficient bound $\sigma_{g,a}$ and source energy bound $E_{g,a}$ in the **actual packed source**. If accepted challenges satisfy $\|c_a\|_1\le K_{1,g,a}$ and multiplication norm $\Gamma_{g,a}$, then Young's inequality and the triangle inequality give

$$\|z_g\|_\infty\le U_g:=\sum_aK_{1,g,a}\sigma_{g,a},\qquad \|z_g\|_2^2\le D_g:=\left(\sum_a\Gamma_{g,a}\sqrt{E_{g,a}}\right)^2.$$

For the old identity source, $\sigma_*=b_*/2$ and $E_{*,a}=M_*d_*(b_*/2)^2$. Uniform challenges give

$$U_*(N_*)=P_*B_*(N_*)K_{1,*}(b_*/2).$$

With centered reduction one may cap this by $\lfloor q/2\rfloor$, but no cap is needed for the illustrative small-response calculation below.

For even response base $b_z\ge4$, canonical balanced digits have positive reach

$$T_f(D)=\left(\frac{b_z}{2}-1\right)\frac{b_z^D-1}{b_z-1}.$$

Freeze the least allowed $D$ with $T_f(D)\ge U_g$, or conservatively choose a larger one. In particular,

$$\delta_{f,*}(N_*)=\left\lceil\log_{b_z}\left(1+\frac{(b_z-1)U_*(N_*)}{b_z/2-1}\right)\right\rceil.$$

Base two's canonical alphabet $\{-1,0\}$ has positive reach zero at every depth, so it cannot use this symmetric response argument. It is suitable for the prescribed negative-binary **field** compression lift, which is a different encoding.

Security uses the accepted-digit envelopes, not $U_g$:

$$\beta^{\mathrm{cert}}_{f,g}=\frac{b_*}{2}\frac{b_z^{\delta_{f,g}}-1}{b_z-1},\qquad \Delta^{\mathrm{cert}}_{f,g}=(b_*-1)\frac{b_z^{\delta_{f,g}}-1}{b_z-1}.$$

Changing $N_*$ can change both depth and collision radii even if the honest norm is small. Euclidean certification, if added, must enforce $S_{\max,g}\ge D_g$ or another justified bound and include its exact integer no-wrap/carry proof domain and auxiliary cost. The preferred coefficient route does not silently borrow its smaller SIS radius. Akita Equations (111)–(113), Corollary G.6 [A].

These deterministic envelopes cover arbitrary valid coefficients and adaptive histories when the source predicate holds; no independence or uniformity of recursive digits is required. They make response-admission failure zero after successful challenge construction. Sampler exhaustion remains: with per-attempt construction probability at least $\underline a$ and $R$ nonce attempts, failure is at most $(1-\underline a)^R$ per step, hence at most $T_{\max}(1-\underline a)^R$ over the run.

### 5.2 Fixed-depth slab certificate

A useful **sufficient** method fixes a candidate old response depth $D$ and an interval of allowed old block counts, while freezing ranks/slices/compression shapes within that interval. Let

$$K_*=M_*d_*,\quad A_*=M_*d_*\delta_{\mathrm{com},*},\quad \gamma_*=P_*\delta_{\mathrm{open},*}(d_{\mathrm{op},*}+n_{A,*}d_*).$$

Let $C_D$ be a certified upper bound on **all fresh segments, all quotient/compression digits, and alignment** over the interval and admitted fresh catalogue. For block-aligned $N_*=K_*B$,

$$\Phi(N_*,S)\le\gamma_*B+A_*D+C_D.$$

Consequently a sufficient shape/encoding certificate is

$$\boxed{\begin{gathered}
\gamma_*<K_*,\\
\left\lceil\frac{A_*D+C_D}{K_*-\gamma_*}\right\rceil\le B\le\left\lfloor\frac{T_f(D)}{P_*K_{1,*}(b_*/2)}\right\rfloor,\\
N_*=K_*B\text{ is a supported Boolean/full physical grid},\\
\text{every frozen matrix width, rank, alphabet and setup cap is valid at this }B.
\end{gathered}}$$

This is not an existential “large enough” argument: it exhibits a nonempty integer intersection, or rejects the candidate. If $\gamma_*\ge K_*$ and positive fresh/response overhead remains, the old image/partial contribution alone precludes this fixed-shape route. If tail/rank changes with $B$, subdivide into slabs and bound those changes; do not hold them artificially constant.

Without fixing depth, the simple unsliced fixed-rank expression is roughly

$$\Phi(N_*,S)\le\rho N_*+A_*\delta_{f,*}(N_*)+C,\qquad \rho=\frac{\gamma_*}{M_*d_*}.$$

The logarithmic depth term does not prove security feasibility: B/D widths and response collision radii also grow. Security-allowed width/depth caps may eliminate every shape-feasible slab. Modulus centering caps depth eventually, but does not repair this width/radius or query problem.

### 5.3 Admission algorithm

1. Enumerate a **finite** set of receiver capacities, rings, $M_*$, radices, ranks, slices, and direct/offloaded variants within declared resource limits. Existing original commitment geometry is input, never a search variable.
2. Require typed same-handle source bounds and full-grid/padding semantics. Reject any weak-source input for which the uniform bound is unavailable.
3. Derive $B_*(N_*)$, all fresh $B_g$, source widths, canonical interval depths, D width, and every Equation (109) segment. Compute exact addresses and all zero spans.
4. Derive the successor commitment A/B/F hints, their bounded retained size, the full relation row inventory, setup-prefix length, and any next setup group. Check the shape inequality for every admitted fresh shape or a proved worst-case envelope.
5. Derive **all** original/receiver/cross-profile matrix collision instances and challenge cardinality/difference/filter certificates at these exact widths. Security-induced rank changes go back through step 3.
6. Check the full norm/sampler/nonce/history-conditional completeness ledger and the complete classical Fiat–Shamir/error/query and extraction-depth ledgers. An accumulation-only field extension cannot repair original fp128 proof errors.
7. Emit $\operatorname{ShapeCert}$, $\operatorname{CompleteCert}$, and $\operatorname{SecurityCert}$ with distinct statuses. Only the conjunction, including the new topology/composition theorem, admits a production profile. Search failure is a useful result, not permission to omit compression, setup, or domain rows.

The online verifier derives the same shape from $\mathcal P$ and the typed input. It verifies the original frontend/boundary checks; binds the entire old state and ordered fresh original handles/points/values; binds shared D/H partial data before folding challenges; checks the successor reduction prefix after the successor is bound; and outputs only its fully specified pending opening/domain obligation. Complete successful rows are consumed by this reduction rather than appended to the retained state. A receipt must authenticate these checks; a hash chain by itself does not.

## 6. Concrete auditable shape example — deliberately not secure parameters

This example shows that the accounting and fixed-depth admission algorithm are executable algebra. It does **not** claim a useful zkVM receiving profile. In particular, its challenge entropy is insufficient for a 128-bit fold target, its A/B/D ranks are not validated, and its re-entry topology is not covered by Akita Definition 9.2.

Choose the paper's $q=2^{128}-2^{32}+22537$, $E=\mathbb F_q$ ($k=1$), $b_*=b_z=b_1=16$, and $d_A=d_B=d_D=d_{\mathrm{car}}=64$. No setup offloading, no response chunking, no quotients, and $S_B=1$. Each group has $P=1$ and A/B ranks 4; shared D has rank 4.

| Item | Old receiver | Fresh originally provisioned input |
|---|---:|---:|
| Physical base-field cells | $2^{20}$ | $4096$ |
| Ring elements per block $M$ | 1024 | 64 |
| Actual blocks $B$ | 16 | 1 |
| Source gadget depth | 1 (identity) | 33 (bounded full-field gadget opening) |
| Packed-source coefficient bound | 8 | 8 on every source plane |
| Assumed accepted challenge $\ell_1$ cap | 16 | 16 |
| Deterministic response bound $U$ | 2048 | 128 |
| Response depth $\delta_f$ | 4 | 3 |

The fresh source must be provided under this **originally fixed** 33-plane geometry and certified short source; it is not a retrofit to an existing 32-plane handle. Both partials and inner images use depth 33. This depth covers every centered field coefficient: $T_f(32)=158798437896437949616241483468158498679<\lfloor q/2\rfloor$, whereas depth 33 covers it. Depth 32 can represent residues with an asymmetric alternative lift, but one cannot use centered-$q/2$ reasoning with that depth without specifying that lift.

$T_f(2)=119$, $T_f(3)=1911$, and $T_f(4)=30583$, so the chosen response depths cover the deterministic bounds. Use the paper's q128 Table 5 compression shapes on every F/H path: rank one at $d_1=16$, then rank one at $d_2=8$, binary depth 128. Each raw B image and the shared D image has $4\cdot64=256$ field coefficients, or 4096 bytes, below the table's 8 KiB input cap. Each first-stage input has 32768 binary cells ($m_1=2048$ ring elements); each second-stage input has 2048 cells ($m_2=256$). Public compressed payloads have eight field coefficients. This uses the table's **compression-only** per-instance attack-model estimate; it does not certify A/B/D, source comparison, the random oracle, or accumulation.

The exact successor ledger is:

| Equation (109) allocation | Base-field cells |
|---|---:|
| Old response digits $1024\cdot64\cdot4$ | 262144 |
| Fresh response digits $64\cdot33\cdot64\cdot3$ | 405504 |
| All partial-opening digits $17\cdot33\cdot64$ | 35904 |
| All inner-image digits $17\cdot33\cdot4\cdot64$ | 143616 |
| Main quotient digits (quotient-free) | 0 |
| Stage 1 F digits, two groups | 65536 |
| Stage 1 H digits, shared path | 32768 |
| Stage 1 quotient digits | 0 |
| Stage 2 F digits, two groups | 4096 |
| Stage 2 H digits, shared path | 2048 |
| Stage 2 quotient digits | 0 |
| Compression ring completion and scheduled alignment | 0 (all applicable boundaries divide these lengths) |
| **Before final padding** | **951616** |
| Final committed zero suffix to $N_*=1048576$ | **96960** |

In this example $A_*=65536$, $\gamma_*=10560$, $K_*=65536$, and

$$C_D=405504+10560+104448=520512.$$

At $D=4$, the slab inequality becomes $15\le B\le238$. The supported power-of-two choice $B=16$ gives exactly $951616\le1048576$. Choosing $B=8$ with this depth would give 867136 cells against capacity 524288, so admission rejects that candidate. These are bounded calculations, not tests or parameter-estimator output.

The matrix inventory has

$$m_{B,\mathrm{old}}=2112,\quad m_{B,\mathrm{fresh}}=132,\quad m_D=561.$$

A-old uses 262144 setup coefficients; A-fresh and B-old each use 540672; B-fresh uses 33792; D uses 143616; each first compression map uses 32768 and each second map 2048. In Akita's shared-prefix setup convention the longest view is 540672, hence a provisioned power-of-two prefix of 1048576 field coefficients (16 MiB at q128), **not** the sum of every view. The full direct row catalogue has 28 ring rows plus one scalar opening row: two groups each supply $1+4+4$, shared D supplies 4, and both F/H stages supply $2+1$ each. Thus $s_{\mathrm{rel}}=5$, $d_{\max}=64$, and quotient-free ring reduction/row batching alone contributes $68/q$ before range/fused-sumcheck and other errors are added.

### Explicit failure certificates

- **Challenge entropy.** In dimension 64, *all* integer challenges with $\ell_1$ norm at most 16 number at most

  $$\sum_{j=0}^{16}\binom{64}{j}2^j\binom{16}{j}=302557549412667613185<2^{69}.$$

  This counts supports, signs, and positive coefficient compositions with total weight at most 16. Its log-cardinality is about 68.04 bits; any filtered sparse family is smaller. Therefore the example cannot provide a 128-bit special-soundness challenge ledger even before query loss. More entropy generally changes challenge norms or ring dimension, and requires recomputing the entire shape.
- **Fresh-size sensitivity.** Keeping these assumptions but provisioning one full-field fresh input of $2^{16}$ cells with $M=1024$ gives fresh response length 6488064 and total 7034176, exceeding $2^{20}$. The single-group 4096-cell allowance is not a result for a real multi-column zkVM segment.
- **Rank sensitivity.** Larger security-required ranks increase inner images, D/B/F widths, rows, setup, and possibly compression output shapes. The 8 KiB table cap may cease to apply; no constant 128-byte promise survives automatically.

The example certifies **shape feasibility under explicit symbolic bounds**, and simultaneously certifies why it is not a security-admissible answer to the user's native target.

## 7. Bounded rows, handles, catalogue, and setup obligations

For $G\le1+G_{\max}$ incoming groups, the active ring-row count is

$$R_{\mathrm{ring}}=\sum_g(1+n_{A,g}+S_{B,g}n_{B,g})+n_D+\sum_{a=1}^{L_{\mathrm{comp}}}\left(\sum_gn_{F,g,a}+n_{H,a}\right).$$

There is additionally one scalar opening row; public claim combinations, tensor reductions if selected, range/norm protocols, and padding/domain certificates have their own exact bounded proof/degree ledgers. Count those separately rather than declaring them nonexistent because they are not ring rows. The domain range proof scans the complete receiving vector.

**Retained invariant:** after successful update only the successor opening/domain object, bounded application boundary, and a fixed number of receipt/setup obligations survive. The group rows above are *transient inputs to the sound reduction*, not an append-only retained list. Original handles remain authenticated in the receipt/history record, but a public running list of all handles is not the live cryptographic accumulator. If the receipt is absent and replay is required, the history storage/replay cost remains $\Theta(T)$ and must be reported.

The catalogue binds matrix role, source type, ring, source map, original commitment/view reference, radix, slice layout, and comparison-class radius. It contains the receiver, every admitted fresh source profile, any setup registry slot, and any terminal profile. Repeated occurrences use these fixed typed views; their identities and extraction error charges cannot be erased, but their setup entries need not be resampled per step. No arbitrary shape-dependent matrix is introduced online.

With direct setup, there is no pending setup opening. With offloading, a successor includes the bounded registered setup group as a real source, **including its response/partials/images/compression**; it changes $\Phi$. An old setup group must be consumed before replacement by the new one. Enforce a fixed slot count and refuse a final terminal edge with any unresolved setup slot. Freezing a registry is not enough if one accumulates $T$ unevaluated setup claims. Akita §9.3 explicitly allows checking one setup group and producing another without their accumulation [A]. A steady offloaded re-entry profile remains a new composition requiring its own closed receiving certificate.

SALSAA's eager-row-replacement principle supports this invariant; its fully succinct/vSIS backend theorem must not be transplanted into Akita's ordinary Module-SIS direct-setup costs [S].

## 8. Comparing the target and genuine alternatives

### 8.1 Native old-plus-fresh re-entry — preferred research target

Consume the exact original A/B/F opening state for old and fresh groups; form one shared D/H payload before fresh fold challenges; construct complete Equation (109); check the relation/range/binary prefix; defer the fully typed successor ordinary opening. Receiver identity encoding ensures that a *valid bounded full-grid receiver witness* is an admissible old source on the next update. It does not turn the two-tier commitment into $C'=C+\rho C_{\mathrm{fresh}}$.

This avoids appending historical witness columns, but requires all the certificates above and a new reduction/composition theorem. The existing Akita recursive extractor may recover weak algebraic sources rather than bounded predecessor witnesses. A bounded final endpoint and shape fixed point do not solve that source-projection or adaptive extraction problem.

### 8.2 Fixed-tuple decomposition — LF+/SuperNeo alternative

For an **already bridged linear commitment relation**, keep a fixed tuple of bounded instances. Let $\ell$ be the retained tuple size, $f$ the fresh input count, $\Gamma$ a uniform challenge multiplication bound, and $b$ a source coefficient bound. A simple folded norm envelope is

$$\|z'\|_\infty\le(\ell+f)\Gamma b,$$

when the stated norm is the corresponding operator-induced coefficient norm; otherwise substitute the exact proven ring expansion constant. Choose a fixed $k_{\mathrm{dec}}$ and signed decomposition convention whose certified recomposition envelope covers this bound. On SuperNeo's convention $B=b^{k_{\mathrm{dec}}}$ is the input norm ceiling, so $(\ell+f)\Gamma b<B$ is a simple sufficient closure condition with $\ell=k_{\mathrm{dec}}$. Retain all $k_{\mathrm{dec}}$ commitments and **full lifted ring** norm/relation evaluations, not just scalar constant terms. Then tuple cardinality and source norm reset each step while witness length per component stays fixed.

LF+ §5.2 gives the specific two-component reduction from its $B^2$ relation to two $B$ relations and checks both commitment and evaluation recombination. Its zero knowledge error for that reduction is a statement about its own exact relation, not a bridge to Akita [L]. SuperNeo's coefficient/flattening embedding and matrix transforms preserve the shared packed witness and justify its ring evaluation homomorphism; a nonconstant ring fold cannot generally be recovered by folding only scalar projections [N].

**Retained obligations:** prove an exact representation/commitment-consistency reduction from every original Akita handle, a uniform bounded source relation, compatible challenge units, all frontend semantics, and modular auxiliary-I/O extraction. Applying signed decomposition to an unrelated fresh commitment proves none of these. Flattening every LF+ range helper into the next accumulator would recreate the prohibited multiplicative witness blowup. This is a different fixed-tuple target, not evidence that Equation (109) fits the original one-opening target.

### 8.3 Independently completed refresh/checkpoint

At each update, or once every **fixed** $J$ updates, complete a domain-and-opening proof and an exact bounded local bridge before recommitting a canonical fixed-capacity state. Transparent reveal-and-check is a concrete complete method; a private static proof can replace it only with independently justified completeness/extraction and same-handle consistency. Fixed $J$ bounds local expansion, but resets do not come for free: their proofs, verification, privacy loss, and receipt authentication are charged. Choosing $J$ proportional to $T$ does not give a fixed live bound. A checkpoint that only range-checks a new unrelated state is not a refresh of the old opening.

### 8.4 Public verifier-residual catalogue — conservative baseline, not substitution

For a proved exact split $V_{\mathrm{full}}(u)=V_0(u)\wedge\bigwedge_\ell A_\ell x_\ell(u)=y_\ell(u)$ over fixed typed lanes, retain public $(U_\ell,Y_\ell)$ and bind old state/input before nonzero $\alpha\in K$:

$$U'_\ell=U_\ell+\alpha x_\ell(u),\qquad Y'_\ell=Y_\ell+\alpha y_\ell(u).$$

This has no bounded private source, response-digit fixed point, or Ajtai source shortness claim. Catalogue size and lane widths bound state; one-step bad cancellation is at most $1/(|K|-1)$ under the stated ordering. It retains $V_0$, original response/range/norm/challenge/terminal checks, setup handling, frontend/link checks, and a receipt authenticating the update/history. Only the admitted matrix equations are deferred. This may save much less than a full recursive verifier. It is not the native old-plus-fresh receiver, and it is not used to declare the native obligations solved.

## 9. Efficiency and security are multidimensional

Write $S$ for maximum fresh segment/source size, $H$ for admitted fresh handles/claims, $N_*$ for full-grid state, $J_{\mathrm{cat}}$ for finite catalogue size, and $V_{\mathrm{setup}}=\max_I n_Im_Id_I$ for the largest admitted flat setup view.

| Dimension | Native fixed receiver, conditionally admitted | Tuple/refresh alternatives | Public residual baseline |
|---|---|---|---|
| Live private witness | $N_*$ plus bounded original receiver hints; transient fresh source/hints depend on $S,H$ | Fixed $\ell N_0$ or fixed-$J$ checkpoint working set | No private folded witness; public lanes have fixed total widths |
| Live public accumulator | Fixed commitment payload, $\mu_*$ point coordinates, value, counters and bounded boundary/domain/setup descriptors | $\ell$ commitments/evaluations, or one checkpoint | $\sum_\ell(\mathrm{width}(U_\ell)+\mathrm{width}(Y_\ell))$ field cells |
| Per-step prover | At least full live digit/relation work; dense upper envelope $\widetilde O(N_*+\text{fresh source planes})$ times fixed matrix ranks/shape constants | $\widetilde O(\ell N_0+S)$ plus exact Akita bridge/refresh proof work | Derive full verifier data and $V_0$; lane updates linear in admitted widths |
| Per-step verifier | Reduction messages/degree work plus direct public-weight/setup scan; not generically polylogarithmic in $N_*$ | Tuple recombination plus bridge; refresh static verification charged | $V_0$ plus input derivation and public arithmetic |
| Setup | Seed plus provisioned prefix $2^{\lceil\log_2V_{\mathrm{setup}}\rceil}$; finite views and registry metadata | Linear/dual commitment setup and bridge catalogue | Fixed lane matrices plus the original proof setup |
| Final discharge | Ordinary opening/terminal authentication **and** bounded domain; reveal option costs fixed-$N_*$ work/data | $\ell$ deciders or completed refresh receipt | All $A_\ell U_\ell=Y_\ell$ plus receipt |
| Across $T$ updates | Total proving/updating $\Theta(T)$ times per-step cost; bounded *live* state does not imply bounded archived history or receipts | Same; refresh adds $\Theta(T/J)$ checkpoints | Same; replay receipt is $\Theta(T)$ |

More explicitly, old source formation touches $P_*B_*m_{A,*}$ ring elements, response decomposition touches $m_{A,*}d_*\delta_{f,*}$ cells, and successor commitment/range work can touch $N_*$. Retained inner-image hints have $P_*B_*\delta_{\mathrm{open},*}n_{A,*}d_*$ cells; both F hint vectors are also retained. They are bounded functions of the frozen $N_*$, not negligible just because the public payload is short. Direct shared-prefix scanning costs work proportional to active setup views and their derived weight contractions. Akita's sublinear constant-root complete PCS schedule is not a theorem that a new streaming update avoids this cost.

### Mandatory error/depth ledger

For native certification enumerate ring-reduction/row-batching errors, range/binary and fused sumcheck errors, public claim combinations, tensor reductions when used, original frontend/PCS errors, source/domain/static bridge errors, setup-product and terminal errors, accepted challenge cardinalities, all original/receiver/cross-profile Module-SIS instances, setup expansion distance, nonce exhaustion, and receipt/compiler errors. Do not multiply assumed independent probabilities where only union bounds are justified.

Akita's published classical-ROM bound for an **admissible** complete schedule is

$$(Q+1)\varepsilon_{\mathrm{int}}+\Delta_{\mathrm{setup}}+\sum_I\operatorname{Adv}_{\mathrm{MSIS}_I}.$$

Its schedule strength satisfies

$$\lambda_{\mathrm{FS}}^{\mathrm{eff}}=-\log_2((Q_{\max}+1)\varepsilon_{\mathrm{int}}),\qquad \lambda_{\mathrm{FS}}^{\mathrm{eff}}\le\log_2|E|-\log_2c-\log_2(Q_{\max}+1)$$

when any nonzero $c/|E|$ term appears. Thus original fp128 field errors persist even if a separate accumulation residual field $K$ is enlarged. Error coefficients depend on the complete shape and schedule. A naive $T$-step budget costs an additional run-wide sum of update errors; an adaptive composition theorem must justify the exact bound. Nonce probes count toward the same FS query budget [A, Corollary 10.34].

The Akita extractor's $\operatorname{Tree}=\prod_rb_r$ includes range anchors, relation batching, scalar sumcheck rounds, folds, setup reductions, and terminal stages. Repeating a fixed nontrivial fold $T$ times can give $b^T$, which is not polynomial in $T$ or the explicit run length $T(N_*+S)$. Fixed memory and per-step algebra do not remove this obstruction. A new modular extractor is retained as an obligation, not hidden inside the shape admission algorithm.

Quantum lattice-attack cost estimates are not a QROM Fiat–Shamir proof. Akita explicitly analyzes classical ROM and leaves its multi-round QROM extraction theorem open. The receiver cannot inherit a noninteractive post-quantum theorem that the source paper does not establish [A, `line:4141-4250`].

## 10. Required certificates and unresolved production conditions

A deployable target profile needs all of the following, with no silent baseline substitution:

1. **Exact shape certificate:** every source/response/opening/image/quotient/F/H/alignment/padding allocation, original and successor matrix widths, actual block counts, bounded hints and rows, and fixed catalogue/setup-ledger size.
2. **All-valid-source certificate:** same-handle bounded source predicates for fresh inputs and the old full-grid identity receiver; canonical/padding requirements precisely where needed; honest canonical intervals and all-valid domain certification rather than a generated-only moment assumption.
3. **Native relation bridge:** exact original-handle A/B/F, shared D/H, frontend and machine-boundary checks; sound successor domain/range/opening reduction under the new old-plus-fresh topology.
4. **Security-admissible parameters:** accepted family cardinality and pairwise unit/difference certificate; each original/receiver/comparison-class collision radius and rank; setup expansion; full FS query/field ceiling and final terminal ledger. The numerical shape example fails the 128-bit challenge requirement.
5. **Adaptive composition:** independently authenticated fork endpoints and source projection matching, or another valid modular extractor with polynomial dependence on $T$ and explicit witness size. A final domain reveal alone is not this theorem.
6. **Actual efficiency comparison:** same segment/IO/receipt semantics, fixed $S,H,N_*$, direct/offloaded setup costs, source certification/refresh costs, and terminal/domain cost. No timing or speedup is established here.

Deterministic all-valid-source bounds are the preferred first research instantiation because they make the completeness contract auditable. Akita's rigorous filtered-family marginal tails can later reduce conservative depths under their exact source and accepted-distribution premises. Appendix G.3's Gaussian/moment propagation and zero-cross-moment local packing approximation are only modeled history-conditional completeness inputs. They do not establish an all-valid receiver fixed point or secure original-source relation. Any optimized modeled profile must be labeled separately and reclosed after security/rank changes.

## 11. References and exercised research record

**Local primary text was read through qmd MCP**, using TOC-derived addresses before `doc_read`. Addresses refer to the indexed parsed snapshots, whose typography occasionally contains extraction damage. Algebra above is derived with explicit notation, not presented as repaired verbatim quotations.

- **[A] Akita**, *Akita: A High-Performance Lattice-Based Polynomial Commitment Scheme*, ePrint 2026/1983; [primary ePrint page](https://eprint.iacr.org/2026/1983), [local parsed paper](SOURCE_PROVENANCE.md#akita). Read §§3.1, 4.3, 4.5, 5.1–5.4, 7.3, 9.3, 12.2, classical FS section/Corollary 10.34, and G.1–G.3: qmd `line:867-996`, `1645-1768`, `1825-2026`, `2057-2124`, `2125-2228`, `2307-2376`, `2727-2946`, `3194-3245`, `4141-4250`, `4641-4662`, `6003-6132`, `6133-6216`, `6217-6412`. The ePrint landing metadata/abstract were independently read; detailed equation claims rely on this local indexed version, not an assertion that every online revision was compared.
- **[L] Boneh–Chen, LatticeFold+**, *Faster, Simpler, Shorter Lattice-Based Folding for Succinct Proof Systems*, ePrint 2025/247; [primary page](https://eprint.iacr.org/2025/247), [local text](SOURCE_PROVENANCE.md#latticefoldplus). Read §4.4 commitment transformation, §5.2 decomposition, §5.3 efficiency: qmd `line:634-743`, `1164-1193`, `1194-1243`. Primary landing abstract corroborates the transformation scope; no LF+ theorem is asserted for native Akita handles.
- **[N] Nguyen–Setty, Neo and SuperNeo**, *Post-quantum folding with pay-per-bit costs over small fields*, ePrint 2026/242; [primary page](https://eprint.iacr.org/2026/242), [local text](SOURCE_PROVENANCE.md#superneo). Read §5 coefficient/flattening embeddings, matrix/evaluation transforms and homomorphism, and §7.5 fixed-tuple decomposition: qmd `line:591-854`, `1117-1154`. Read primary landing metadata/abstract. Detailed tuple claim is Theorem 13 in this indexed snapshot.
- **[S] Kuriyama–Lai–Osadnik–Tucci, SALSAA**, *Sumcheck-Aided Lattice-based Succinct Arguments and Applications*, ePrint 2025/2124; [primary page](https://eprint.iacr.org/2025/2124), [local text](SOURCE_PROVENANCE.md#salsaa). Read §4.3 and §7.2: qmd `line:435-528`, `1297-1362`. The §4.3 result contained an embedded base64 figure and was truncated after the useful batching text; no claim depends on the unread image. Read the primary landing metadata/abstract. Its benchmark claims were not exercised or imported as receiver performance claims.

**Bounded calculations actually exercised:** exact integer base-16 endpoints for q128; the complete 951616-cell successor and 96960-cell padding; all listed B/D widths and shared-prefix setup lengths; 29-row count; sufficient slab $15\le B\le238$; the larger fresh-input rejection; and the integer $\ell_1$ challenge-ball count. Calculations ran in the tool's ephemeral Python evaluation kernel without writing a script or modifying code. No build, test, formatter, benchmark, proof generation, security estimator, or branch operation was run. No cryptographic profile, static private certificate, native update implementation, or long-chain theorem was validated.
