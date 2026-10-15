# Composed receipt policy: conditional losses, distinct budgets, and runtime-priced hardness

## 1. Scope and conclusion

This is a **conditional accounting theorem and diagnostic policy**, not selection of a receipt profile, compact CM, a concrete oracle, or CH-I/CH-O. The preserved anchor is the original pre-PCS native instance archive, original receipt/accumulator and actual external decision, X-binding original `C_g`, original full-capacity `C_t` and its same ordinary outer quotient opening, installed `C_static`, and complete native final decider. No source is replaced by an honest canonical commitment state.

The equations below make the original → private extraction → authentication/native reductions composable. They also exhibit sufficient inequalities for an **illustrative overall error target** `epsilon=2^-128`. This target is not a new mandatory admission requirement: [DESIGN §13](AKITA_INSTANCE_ACCUMULATION_DESIGN.md#13-evidence-and-admission-checklist) expressly says a tight end-to-end 128-bit reduction is not an extra user requirement. The actual obligation is explicit supported-depth soundness/query/runtime accounting under stated assumptions, without inheriting Q32, and measured parity before cutover. Failure of a loose sufficient inequality is neither a forgery nor an impossibility result for classical-ROM designs.

**Current conclusion:** the reviewed lemmas supply a nontrivial symbolic composition, and the diagnostic inequalities have nonempty abstract parameter ranges. No nonvacuous *instantiated numerical receipt policy* is justified by the current evidence: the selected guest/source/schedule, complete adversarial budgets and reduction-time hardness inputs are missing. This is an unclosed budget/instantiation obligation, not evidence that every valid choice fails. Concrete software security is additionally unestablished; the deterministic degree-two graph is distinguishable from the uniformly sampled ideal extension, but that distinction alone is not a receipt forgery.

**Later experimental route:** the user-authorized [D1 model](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) has independent conditional review. It computes the fixed public inner tuple inside the outer reduction, so the ideal graph branch's private-emulator, quotient-authentication and CM overhead below is not mandatory D1 work. D1 has its own ordinary outer extraction ledger and explicit restricted native-game heuristic with unknown loss/resource translation. Its ideal native comparison may still use these ideal-model results; permission supplies no concrete numerical bound.

## 2. Contracts and source authority

The derivation uses these independently reviewed conditional results, with their hypotheses, rather than only their headlines:

- [PRIVATE_ARO_CONTINUATION §§3,6–10](AKITA_INSTANCE_ACCUMULATION_PRIVATE_ARO_CONTINUATION.md): deterministic original-record extraction, two complete-run coupling losses, distinct actual/private states, and full ordinary-MSIS runtime.
- [NATIVE_SOURCE_FIRST §§3–8,10](AKITA_INSTANCE_ACCUMULATION_NATIVE_SOURCE_FIRST.md): original-index-preserving replay, exact retained/source-first recursion, fixed-source local certificates and expected-work tails. It is a schedule-parametric replacement compiler, not a selected suffix.
- [ZERO_FINDING_TRANSPORT §§3–7](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md): normalized original-output tester, one independent private probe, explicit hidden curve observations, `Q` **outside** the square root, and strict two-branch collision reduction.
- [COMPACT_CM §§2,5–8](AKITA_INSTANCE_ACCUMULATION_COMPACT_CM.md): optional externally recomputed CM root, early-label/late-hit error **inside** that square root, including both independently continued tables.
- [CHAINED_TRANSCRIPT §§2,5–7](AKITA_INSTANCE_ACCUMULATION_CHAINED_TRANSCRIPT.md): native-key correspondence, mandatory finalization, hidden verifier expansion, exact bit-consumption machine, and ARO-to-component-ROM **before** native stream simulation.
- [Current RECEIPT §§3–7,9](AKITA_INSTANCE_ACCUMULATION_RECEIPT.md), [RECEIPT_SETUP §§4–5](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SETUP.md), and [OUTER_PROJECTION](AKITA_INSTANCE_ACCUMULATION_OUTER_PROJECTION.md): complete original-source guest relation and installation boundary.

Primary evidence inspected for this ledger:

1. Chen–Chiesa–Gur–O'Connor–Spooner, *Proof-Carrying Data From Arithmetized Random Oracles*, [accepted manuscript](https://wrap.warwick.ac.uk/id/eprint/176570/1/WRAP-proof-carrying-data-arithmetized-random-oracles-Spooner-2023.pdf), [local primary text](SOURCE_PROVENANCE.md#supplemental-primary-sources), Definition 5.1 (lines 1317–1328), Construction 5.11 (1536–1558), Construction 5.20/Lemma 5.21 (1928–1981), and Lemma 7.1 (2143–2166). These supply the stateful construction and polynomial constraint-solving context. The unrestricted emulator claim and stronger published zero-finding query law are **not** used; the repaired input-local paired theorem and reviewed weaker transport are derived results. The opposite signs in Construction 5.20's displayed constraint/output formulas are handled by the corrected consistent-sign kernel, not copied as an implementation.
2. [Akita supplied primary text](SOURCE_PROVENANCE.md#akita), publication locator [ePrint 2026/1983](https://eprint.iacr.org/2026/1983), Theorem 10.31/Remark 10.32 (4175–4197), Eq. (228) (4169–4173), and Theorem 10.33/Eq. (229) (4199–4227). This is the supplied revision's numbering, not an independently certified publication revision. It explicitly distinguishes algebraic full-grid sources from canonical short gadget sources and prices ordinary MSIS at the reduction's runtime.

Work below is mathematical/source inspection only. No implementation, builds, tests, formatters, benchmarks, or git operations are part of this note.

## 3. Five ledgers that must not be identified

Let `Theta=(C,wo,Vhat)` be the ideal inner ARO, independent of the outer ideal transcript `H_o`. Fix input-local `wo(x)=b(x,mu_x,C(x))`, independent persistent witness coins, represented field `F`, individual degree `d>=2`, and `a=m+lambda_A+w`. Setup, code, codec, schedule family, shapes and all caps are fixed before oracle sampling. Oracle-dependent initialization is counted, not supplied as hidden advice.

### 3.1 Original versus virtual extraction

`P` runs the original prover, complete original outer verifier and both actual external graph checks, retaining the exact record `U` and original accept bit `A0`. **If compact CM is explicitly selected, P also executes the original external RootValid check**; its calls/work are included in h0/t0/L0 and its result in A0. Later normalization may recompute it with its own charged convention. A first retained-head activation records both RO and WO halves; repeated requests and the complementary fetch have their declared logical costs. Let:

| Symbol | Owner and meaning |
|---|---|
| `h0,v0,t0,L0` | Original bounded `P`: distinct actual paired heads, actual extended calls, exposed calls and logical work; includes initialization/verifier/decider |
| `hK,vK,tK` | Private extraction continuation's additional virtual heads/evaluations/logical calls |
| `hFull,vFull,tFull,L` | Strict logical `P+K` caps; safely `hFull<=h0+hK`, `vFull<=v0+vK`; `L>=L0` |
| `q_o` | Original outer ideal-stream entries under its stated prover/verifier/initialization convention; not any of the inner head counts |

The private continuation makes **no new actual Theta requests**. Its known pairs, extended record, failure state and kernel remain private after seeding. Nevertheless `vFull` controls an extraction coupling error and `(hFull,vFull,tFull)` controls emulator CPU. A small `h0` does not make this work cheap.

The extracted relation `R` is oracle-free. It binds the original statement/accumulator, original `C_g,C_t`, installed static reference, all required full-grid source projections, weak unit/product certificates, canonical **consumed** bytes, guest success/output/panic semantics, complete ordered supplied-answer ledger and original quotient claim. It does not ask whether a private answer equals a future actual answer. Checking ledger truth or running the native verifier against actual streams is a later security experiment, not a deterministic clause smuggled into `R`.

### 3.2 Authentication normalization

For the authentication reduction, append the exact source-validity/CM/raw/graph tester and terminal false-polynomial test. Let `Q>=1` bound **main external fresh paired head activations in this normalized experiment**, including WO-first activations, an unqueried output CM, root recomputation if selected, and all actual producer/initialization/predicate requests. The private extraction already inside the producer adds CPU, not external heads. The transport proof's one additional independent probe is a second private continuation with its own bound `Q`; it is not part of the main head count a second time.

For `n>=2` ledger targets and actual full quotient capacity `N_t`, set

\[
D_f=ad(n-1),\quad H=D_f+1,\quad
D_*=\max\{D_f,n+N_t-1\}<M=|F|.                 \tag{1}
\]

All `4N_t` native K coefficients contribute to the polynomial `t`; larger physical backend capacity replaces `N_t`. Honest completeness additionally needs `N_t>=D_f-n+1` (and the actual power-of-two capacity/packing rules), not merely a security degree declaration. No truncation to honest/live coefficients is allowed.

A conservative normalized actual extended bound is

\[
v\le v_{\rm prod}+H+2+v_{\rm extra}.           \tag{2}
\]

Here `v_prod` includes the original producer's actual calls. Rechecking its two graph equations may instead reuse their unchanged cached answers; choose one explicit convention, not both. The `H` terminal interpolation observations are proof-only tester calls, not receipt-verifier work. The hidden-observation game adds another `H` main evaluations. These quantities are distinct from `vFull`; virtual extraction evaluations do not become actual merely because their arguments could be actual queries.

### 3.3 Native normalization and native keys

For the native-language event, let `vN` count actual extended queries in the legal producer plus that game's final predicate. A genuinely C-only final native verifier adds no extended queries: then `vN=v0`, absent other actual producer-side calls. The final verifier still adds native bits, component expansion and CPU.

After the paired ARO-to-C lift, one new external head costs at most one C request; repeated local B gates cost CPU, not additional external requests. Let `Q_C` count the producer's resulting direct component queries. With no later actual producer calls, `Q_C<=h0`. Native high-level verification has distinct full encoded prefixes `P in H_nat` and distinct squeeze-block triples counted by `B_H`. A sufficient compiler ledger is

\[
N_c=Q_C+\sum_{P\in\mathcal H_{\rm nat}}
 (\lfloor |P|/b\rfloor+1)+B_H,\qquad G_c\le Q_C.       \tag{3}
\]

Count direct-component verification in `Q_C` instead if that interface is chosen; do not also expand the same calls as high-level service calls. Other producer-side high-level services require their actual expansions. CM and absorb calls are not automatically native keys. Under the exact key correspondence,

\[
q_{\rm nat,prod}\le q_{\rm rooted\ squeeze}\le Q_C.    \tag{4}
\]

If the native theorem normalizes verification/preprocessing inside its adversary, add their actual distinct keys to obtain `q_nat`. If it uses a prover `Q` and explicit `Q+1` output convention, retain that convention instead. Generating random-access block `j` may cost `lambda_A(j+1)` bits per key in a sequential stream implementation; it does **not** create `j+1` keys. Public byte/index/rejection caps are indispensable. `q_o`, `Q`, `N_c`, and `q_nat` measure different things.

### 3.4 Ordinary hardness resources

Every collision term below means ordinary MSIS for the exact admitted matrix view, dimensions, modulus, certified norm and joint setup, at the **derived reduction's** time, memory and success model. It does not mean a per-matrix table quoted at the original prover's runtime. Authentication forks the already expensive private-extraction producer again; native extraction may do likewise. Shared setup views are embedded consistently from one flat setup, not independently resampled.

## 4. Extraction loss and strict caps

### 4.1 Schedule-parametric source-first compiler

For every retained node `r`, let its mixed family have local loss `e_r`, support `P0_r<=q_o+1`, and work multiplier `b_r`. For each disjoint source-first block `t`, use its certified local loss and root-prequery atom:

\[
\beta_t=q_t\epsilon_{\rm local,t}
       +\binom{q_t}{2}\gamma_t.
\]

With all challenge stages assigned exactly once, the native suffix extraction failure/collision alternative has statistical loss

\[
\kappa_{\rm suffix}\le
 \sum_{r\in\mathrm{retained}}P0_r e_r+
 \sum_{t\in\mathrm{SF}}\beta_t+
 \sum_{u\in\mathrm{owned\ cutoffs}}\tau_u.             \tag{5}
\]

The retained weak-fold factors are `F_j=1+sum_g W_jg` and terminal `F_T=1+B_term`; their losses use the actual accepted challenge sets. The paper-level strong local certificate includes ring/row, range-product/frontier, **additional binary support**, relation-round, norm scalar and setup-product charges. Tensor/scalar replacements have separate source/root contracts. Missing contracts retain the admitted old tree, not an invented zero-cost source-first node.

For a frontend source-first wrapper, one may then use

\[
\kappa=\kappa_{\rm suffix}+
 q_o\epsilon_{\rm FE}+\binom{q_o}{2}\gamma_0+
 \tau_C+\delta_L,                                      \tag{6}
\]

where `tau_C` or `delta_L` is included only if not already owned in (5). For an unchanged mixed PCS tree, substitute its proved `(q_o+1)epsilon_PCS+tau_suffix` for (5). Do not add both suffix methods, multiply (5) by `q_o+1` again, or count a child failure anew at every parent. Formula (6) is conditional on the complete guest/source projection, not supplied by a PCS theorem alone.

Expected logical work obeys the actual recurrence

\[
C_r\le\lambda_r C_{r+1}+A_r,\qquad
C_1\le(\prod_r\lambda_r)C_{\rm base}
 +\sum_r(\prod_{u<r}\lambda_u)A_r,                    \tag{7}
\]

where `lambda_r=b_r` for retained nodes and `lambda_r=2` for an admitted source-first block, with source-first overhead `(q_r+1)T_C,r+A_SF,r`. The coefficient two follows from the exact selected-index gate and success-weighted renewal identity, not from arbitrary repeated extraction. Branch-dependent schedules require their induced-kernel integrals or proved uniform envelopes.

### 4.2 Expected work is not a strict emulator budget

Let `Z` be complete logical extraction work under the same meter in every actual/emulated world, and suppose an **unconditional** bound `E[Z]<=P_work` is supplied from (7). Preserving original `P` requires a strict initial bound `L0`; stop only continuation `K` at `L>=L0`. Then

\[
\delta_L=\Pr[Z>L]\le P_{\rm work}/L.                 \tag{8}
\]

If charging only continuation work after reserving `L0`, use its own expectation and threshold `L-L0` instead. A separate initial cutoff changes the original experiment and needs its own tail. A per-root K-trial cap gives `q/(K+1)` on the source-success-weighted tail, with every failed child/trial charged; it is an alternative tail certificate, not another name for the same global cutoff.

For a desired tail allotment `d_L`, `L>=P_work/d_L` is **sufficient under Markov**, not a necessary runtime lower bound. Better tails may justify smaller caps. A geometric probe has `E[G^2]=(2-p)/p^2`; after original-success weighting this is `(2-p)/p`, so cancellation in the first moment supplies no uniform second moment. In particular, `E[poly(Z)]<=poly(E[Z])` is invalid.

First derive strict logical head/evaluation/source-length caps from the stopped machine, then run the polynomial emulator under those caps. Do not stop it at a different logical step because simulation CPU is expensive. Exact field/conditional samplers need explicit bit-time rejection caps and corresponding separately owned failure terms.

### 4.3 Nonlinear CPU survives private extraction

With field bit-work `c_F`, B cost `T_B`, witness-coin length `r_B`, constraint detector cost `C_CD(a,d,j)` and bounded lookup/serialization work, a sufficient emulator bound is

\[
\begin{split}
\mathsf E(h,u,t)={}&O(h(T_B+r_B+\lambda_A))+T_{IO}(h,u,t)
 +O(c_Fahu^2)\\
 &+\sum_{j=1}^{u}O\!\left(a C_{CD}(a,d,j)+a C_{CD}(a,d,j+1)
                    +c_F(aj+1)^3\right)
 +T_{\rm field\ random}.                              \tag{9}
\end{split}
\]

Thus `E_private` has time at most `L+T_seed+E(hFull,vFull,tFull)+T_wrapper`. A C-only realization additionally pays a **separate** external emulator `E(hExt,vExt,tExt)`, final predicates and wrapper work. Identical numeric head keys do not merge these states. Gaussian elimination, repeated support evaluation, source decoding/copying and sampling are not unit-cost oracle calls.

An outer collision reduction can implement the full-emulator middle world directly:

\[
T_{o,I}\le L+\mathsf E(hFull,vFull,tFull)
 +T_{\rm embed,I}+T_{\rm coll,I}+T_{\rm wrapper}.       \tag{10}
\]

Authentication uses a strict intrinsic normalized producer/tester cost `T0` that **already includes** private-extraction CPU. With the transport kernel bound `W` (the corresponding terms in (9)),

\[
T_{\rm RB,I}\le 2T0+W(Q,v+H)+W(Q,v)+T_{\rm copy}
 +T_{\rm interp}+T_{\rm compare,I}+T_{\rm embed,I}
 +T_{\rm root\ labels}+T_{\rm bounded\ randomness}.   \tag{11}
\]

The optional root-label term is zero/omitted on literal CM; eliminate overlaps already priced elsewhere. There is one private probe, not `Q` probes: hidden evaluations cost `H`, not `QH`. Full source grids and complete producer/emulator snapshots must be copied/compared.

For the native reduction, first form `T_flat` from the private producer, separate external emulator, chain dictionaries/labels, all high-level expansions, exact sampler bits and full final native verification. Then price its actual native extractor/collision reduction at `T_i,I`; Akita Eq. (228) has `T_FS(A_flat,q_nat,sch)+T_embed(I)+T_coll(I)`, not just `T_flat`. If only an expected-time native reduction is available, either state a hardness function for that exact expected-time class or truncate that reduction and charge its proven tail `tau_nat`. Expected polynomial time does not by itself instantiate a strict-time numeric advantage.

## 5. Auditable composed bound

Write `e(u)=min(1,u/2^lambda_A)`. Work first in one ideal joint setup. Define:

- `Delta`: one total distance for changing the provisioned **joint** setup to this ideal law, including all declared coefficient-sampling/setup discrepancies. It is not re-added in each collision term.
- `tau_aux`: separately introduced initial/implementation/simulation cutoffs not owned by `kappa`, native tails, or authentication's internal sampling term. Name the experiment of each cutoff.
- `A_o=sum_I Adv_MSIS,I(T_o,I)`: the outer collision inventory **including installed static-reference comparison**. Hence no extra `epsilon_static` is added below. If static comparison is implemented as a separate disjoint reduction, split `A_o=A_o,other+A_static`; do not add a second copy of its coupling or the same collision event. Comparing a weak source to the reference keeps its actual unit/product bound, not a short inverse unit.
- `S_i(q_nat)+tau_nat+A_i`: a supplied complete-native-archive language-soundness bound in the ideal flat stream game, where `A_i=sum_I Adv_MSIS,I(T_i,I)`. For a single admitted assembled Akita statement the statistical component is `(q_nat+1)epsilon_int(sch)` by Eq. (229); an archive/frontend theorem must supply its **actual** additional stages/aggregation. The PCS theorem alone is not the whole application-language theorem.
- `A_RB=sum_I Adv_MSIS,I(T_RB,I)`: authentication same-handle relation binding, over all admitted mixed C_g/C_t A/B/F views. Different reductions using the same matrix are different possible events and still require their stated union bounds unless a joint reduction is proved. This is not a reason to add setup distance repeatedly.

For literal injective CM let `r_CM=0`. For **optional, unselected** compact CM with exact externally recomputed full message, let

\[
r_{\rm CM}=\min\left(1,\frac{N_r(N_r+1)/2+N_rG_r}{2^{\lambda_A}}\right),
\quad N_r\le2Q,\ G_r\le N_r,
\quad r_{\rm CM}\le\min(1,(6Q^2+Q)/2^{\lambda_A}).       \tag{12}
\]

The variables in this forest include independent same-input suffix answers on the two branches; they are not falsely treated as one global suffix table. The root error covers equality of full CM messages before same-handle comparison. It is **inside** authentication's square root, not an outside additive receipt error. Native compiler collisions are different events in a different experiment; their error is still charged once below.

Let `rho` bound the accepted mass of a nonzero degree-`D_*` root set and `eta_samp` the strict sampling cutoff of the authentication collision reduction. The transport and native compiler terms are

\[
\begin{aligned}
A_{\rm auth}&=\min\{1,e(v)+Q\sqrt{\rho+r_{\rm CM}+A_{\rm RB}
                       +\eta_{\rm samp}+e(v)+2e(v+H)}\},\\
E_c&=\min\left(1,\frac{N_c(N_c+1)/2+2N_cG_c}
 {2^{\lambda_A}-(N_c+G_c+1)}\right),                   \tag{13}
\end{aligned}
\]

with `E_c=1` if its denominator is nonpositive. The composed conditional receipt bound is

\[
\boxed{\Pr[\mathrm{false\ receipt}]\le
 \Delta+\tau_{\rm aux}+\kappa+e(v0)+e(vFull)+A_o
 +A_{\rm auth}+e(vN)+E_c+S_i(q_{\rm nat})+\tau_{\rm nat}+A_i.} \tag{14}
\]

**Compact proof ledger.** (i) In the full-emulator world partition original acceptance without `R` into extraction failure and a verified inventory collision. Actual-fixed-Theta extraction bounds the former by `kappa+e(vFull)`; ordinary MSIS at (10) bounds the latter by `A_o`. (ii) Apply the original-record coupling **once** to that union, adding `e(v0)`. Private continuation retains the original actual receipt and decider literally; it makes no future-oracle preservation claim. (iii) On `R`, false authentication is bounded by the normalized actual-oracle transport game (13). Outside that event the true complete ledger makes the projected native verifier accept. (iv) Bound a false native application by first emulating its actual ARO side (`e(vN)`), then compiling C to ideal native streams (`E_c`), then using the supplied native theorem at its actual derived runtime. (v) Restore the joint setup once and add only independently owned implementation tails. These are event inclusions; no division by receipt acceptance or conditioning a challenge on successful extraction is used.

## 6. A sufficient diagnostic `2^-128` allocation

Put `epsilon=2^-128` and `delta=epsilon/8=2^-131`. The following eight bucket inequalities imply (14) is at most epsilon:

| Bucket | Sufficient bound |
|---|---|
| Joint setup and separately owned implementation tails | `Delta+tau_aux <= delta` |
| Complete outer extraction | `kappa <= delta` |
| Private-continuation and native ARO lifts | `e(v0)+e(vFull)+e(vN) <= delta` |
| Outer plus static collision inventory | `A_o <= delta` |
| Authentication | `A_auth <= delta` |
| Native stream compiler | `E_c <= delta` |
| Native statistical error and its own cutoff | `S_i(q_nat)+tau_nat <= delta` |
| Native collision inventory | `A_i <= delta` |

This deliberately leaves explicit ownership of `kappa`'s node/global tails and the native reduction's tails. If one source/reference collision reduction covers several cases, allocate its aggregate bound once. A per-instance hardness estimate is sufficient only after summing the exact inventory at the exact runtimes; e.g. `M_o` instances each at most `delta/M_o`, not each at most `2^-128`.

For authentication, require

\[
e(v)\le\delta/2,\qquad
\rho+r_{\rm CM}+A_{\rm RB}+\eta_{\rm samp}+e(v)+2e(v+H)
 \le\frac{\delta^2}{4Q^2}.                           \tag{15}
\]

An easy six-way split is to put

\[
\theta(Q)=\frac{\delta^2}{24Q^2},\qquad
\rho,\ r_{\rm CM},\ A_{\rm RB},\ \eta_{\rm samp},\ e(v),\ 2e(v+H)
 \ \le\theta(Q).                                    \tag{16}
\]

For literal CM its zero root bucket can be reassigned; keeping it unused makes (16) uniformly sufficient for either alternative. In general, if the desired authentication error is `epsilon_auth` and its outside error is `e(v)`, the available total radicand is

\[
\left(\frac{\epsilon_{\rm auth}-e(v)}{Q}\right)^2.
\]

Thus a sufficient aggregate relation-binding target scales as **`epsilon_auth^2/Q^2`**, not `epsilon_auth/Q`, `epsilon_auth^2/Q`, or a standalone 128-bit binding assumption. At the allocation (16), `M_RB` instances may each be bounded by `theta/M_RB` at (11). This is inversion of this proof's sufficient upper bound; it does not assert a matching attack or a necessary lower bound on real security.

For the balanced raw sampler use `R=2^1024`, `M=p^4`, `p=2^128-2^32+22537`, `k=floor(R/M)`. Reject raw values `>=kM` and field points in the declared ledger/exception set. For every fixed root set Z,

\[
\Pr[\mathrm{accepted}\ \beta\in Z]=|Z\setminus(D\cup E_\ell)|k/R
 \le |Z|/M,\qquad \rho\le D_*/M.                    \tag{17}
\]

No sampler-success conditioning is used. Its honest one-draw rejection is less than `2^-512+(n+31)/M` under the quotient profile; rejection cannot falsely accept and is not an additional soundness error here. Any retries must have their actual transcript/head/CPU caps. This completeness abort is distinct from `eta_samp`.

For the latter, with a deterministic constraint detector and all extra reduction-owned draws included,

\[
N_F\le(v+H)(a(v+H)+1)+v(av+1)+N_{F,\rm extra},\qquad
\eta_{\rm samp}\le(N_F+1)2^{-L_{\rm trials}}.         \tag{18}
\]

Hence `L_trials>=ceil(log2((N_F+1)/theta))` suffices. The extra bit work is priced in (11); it is not free because its probability is small. Producer-owned sampling caps already belong to the producer's strict contract, not another copy of (18).

## 7. Honest numerical illustrations, not a certified profile

Assume only `lambda_A=1024` and the field profile in (17); `2^511<M<2^512`. Write a chosen *normalized actual head cap* as `Q<=2^q`. A conservative set satisfying the six authentication sub-buckets is

\[
D_*\le2^{244-2q},\qquad v+H\le2^{756-2q},\qquad
A_{\rm RB},\eta_{\rm samp}\le2^{-(267+2q)},            \tag{19}
\]

together with the actual root inequality (12)/(16) if compact CM is selected. Indeed the degree term is `<2^-(267+2q)`, the doubled extended error is at most that value, and `2^-(267+2q)<=theta(2^q)` since `1/32<1/24`. The outside extended term is then below `delta/2` in the rows below. These are sufficient **inputs** to a future policy, not measured counts or a recommended security profile.

| Assumed normalized `Q` | Sufficient full `D_*` cap | Sufficient `v+H` cap | Aggregate RB and sampling target, each |
|---:|---:|---:|---:|
| `2^32` | `2^180` | `2^692` | `2^-331` |
| `2^64` | `2^116` | `2^628` | `2^-395` |
| `2^96` | `2^52` | `2^564` | `2^-459` |
| `2^112` | `2^20` | `2^532` | `2^-491` |

The root bound for optional compact CM is `<7*2^(2q-1024)`; all four rows satisfy (16). For the first row its exact target is `theta=2^-326/24`, whereas the convenient RB target is `2^-331`; quoting only “128-bit relation binding” would not close this particular composed policy. Root error alone would permit much larger Q, but those isolated caps say nothing about the degree, source capacity, extraction or hardness buckets jointly. At very large Q, (19) can leave no room for the required `ad(n-1)`; that rejects only this sufficient allocation/profile combination, not the construction class.

The degree cap must be realized, not declared independently:

\[
n\le1+\left\lfloor\frac{D_{\rm cap}}{ad}\right\rfloor,
\quad D_f-n+1\le N_t\le D_{\rm cap}-n+1,
\quad N_t\text{ is the actual admitted capacity}.       \tag{20}
\]

For the optional compact grammar's illustrative `b=64`, `k=s=j_w=64`, its exact `m=1610`, so `a=2634+w`. Even on a degree-two graph, the first inequality is `n<=1+floor(D_cap/[2(2634+w)])`; **w remains an input**, not zero by convenience. Literal CM instead needs its full injective tuple to fit m and has its own a. Compact CM changes the transcript grammar and external root checks; this note does not adopt it. Root absorb calls affect Q, native compiler budgets and CPU, even though they need not become additional authenticated ledger targets n. Neither alternative compresses original lattice handles out of the public source linkage.

The huge extended caps in (19) are merely permissive error-budget ceilings. The actual normalized tester needs only its genuine v and H; deliberately using the ceilings in a kernel could give enormous CPU and destroy any proposed runtime hardness bound. They are not performance targets or feasible extracted transcript sizes.

Two other conservative numerical checks illustrate this distinction:

- The third bucket is implied by `v0+vFull+vN<=2^893`. This couples original and **virtual** evaluations; it is not permission for that many practical operations.
- If `G_c<=N_c<=2^445`, then the denominator in (13) exceeds `2^1023`, its numerator is at most `3N_c^2`, and `E_c<3*2^-133<2^-131`. This bounds compiler statistical error only. A declared Q32 adversary does not establish this bound because hidden prefix expansion and block production are separate inputs.

The outer/inner challenge fields and schedules remain separate: the ordinary outer quotient opening uses `E_o=F=K^4`, while the reviewed inner scalar field remains `E_i=K^2`. The approximately 512-bit size in (17) cannot silently be used for inner native errors. Root prequery atoms, retained challenge-set sizes, shell filters and exact sampler correspondences still determine `kappa` and `S_i`.

## 8. Inputs still required and present admission status

An auditable numerical policy requires all of the following actual inputs, not just one favorable head number:

1. **Original adversary/model bounds:** declared classical adversary time/memory/query domain, initialization, original complete outer verifier and external decider costs, `q_o,h0,v0,t0,L0`, and any original sampler/nonce/attempt caps. No derivation here imports Q32 from the inner-only analysis.
2. **Guest and projection certificate:** actual selected fixed oracle-free guest/executable; original X/C_g/C_t/C_static linkage; native emission/bit-consumption correspondence; all consumed-byte/instruction/read/canonicality facts; ordered coverage, endpoint and final-decider checks; soundness for every admitted weak source, not just honest encodings. Semantic validity is a theorem premise, not an unexplained numerical error budget.
3. **Extraction selection:** complete outer frontend and native schedules; retained/source-first ownership; full local degrees and prequery root atoms; REPLAY on legal restrictions; exact gate/sampler implementation; deterministic decoding/comparison costs; complete recurrence (7), its unconditional `P_work`, logical cutoffs and tails, and the resulting `hFull,vFull,tFull`. Private continuation does not supply these numbers.
4. **Source/CM/degree geometry:** literal CM or an explicitly selected compact grammar; exact codecs/namespaces/framing and registry meanings; m,w,d,B and input locality; actual n, domain, full C_g grid and quotient `N_t`, all capacity/padding maps, actual source/handle sizes and `D_f,D_*`; proof of the tower/field and balanced sampler contracts for the selected profile. Honest capacity must also satisfy completeness.
5. **Normalized proof budgets:** authentication Q,v,H including tester/output/root calls and branch caps; actual N_r,G_r if compact; native `Q_C,N_c,G_c,q_nat`, verifier/init keys, prefix lengths, maximum stream indices and bit work; strict reduction-owned field-draw counts and rejection limits. Native key counts do not price private CPU.
6. **Kernel/reduction runtime:** represented field operation costs, B/witness generation, a deterministic constraint detector with explicit cost, dictionary/serialization/copy/interpolation/compare/output costs, and bounded randomness. Instantiate (9)–(11), `T_flat` and each later native reduction time. Polynomial existence alone is not a concrete resource estimate; a large upper bound is not a lower bound on the best implementation or proof.
7. **Complete runtime-priced hardness inventory:** all outer/inner/authentication A/B/D/F/H views as actually needed, shared setup embeddings, cross-profile source comparisons and static-reference occurrences, exact dimensions/certified alphabet/unit-product/norm bounds, and ordinary MSIS advantage bounds at those reductions' strict or expressly modeled expected runtimes. The inner 33-role inventory does not supply the outer receipt inventory, and per-matrix attack-model tables do not establish their aggregate composed advantages.
8. **Setup and concrete boundary:** authenticated provisioning law and one joint `Delta`, all actual dependencies/preprocessing, and an explicitly justified realizable oracle interface if claiming concrete software security. [CONCRETE_GRAPH](AKITA_INSTANCE_ACCUMULATION_CONCRETE_GRAPH.md) supplies graph correctness and cost formulas, not indistinguishability from the ideal uniformly sampled extension or a game-specific receipt theorem. Neither CH-I/CH-O nor a concrete ARO heuristic is selected here.

Until these are supplied, (14) is the justified result and (15)–(20) are conditional diagnostics only. There is no currently supported concrete total error, reduction runtime, full guest profile or performance-parity claim to insert into them. The remaining obligation is genuine: choose and justify an explicit soundness/runtime policy under an approved model, then satisfy the independent receipt admission and implementation/measurement gates. It is **not** a newly imposed demand that every acceptable design obtain a tight end-to-end `2^-128` reduction.

## 9. Review and arithmetic evidence

The parent reviewed the complete derivation against the independently reviewed component lemmas, including event ownership, static comparison inside A_o, one joint setup distance, both private-continuation losses, the native language-versus-PCS distinction, and the full reduction runtimes. One clarification was added: an explicitly selected compact-CM profile must include its **original actual RootValid check** in P, not only the later tester.

Exact rational arithmetic verified all four rows of (19), all six authentication sub-buckets and the outside term, the native compiler illustration, the combined ARO-evaluation bucket and the eight-bucket total. No runtime, hardness advantage, guest profile, concrete oracle or performance was instantiated. The numerical target remains an optional diagnostic, not an added user acceptance criterion.

