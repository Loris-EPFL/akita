# Native source-first extraction for Akita: local certificates and mixed recursion

## 1. Result, anchor, and admission boundary

**Result.** The strong checks *below one fixed native folding child* admit a source-first replacement: recover a weak algebraic source for that child's already-bound successor, compare it with an independently probed source for the same handle, and check the exact native branch relation on the original source. The range, ring, row, relation, norm, and setup-product checks have the fixed-source local soundness certificates described below. A retained flat fold then uses exact certified branch relations, not a new strong interpolation tree. Tensor reduction has a separate fixed-source certificate and separate source ownership.

The replacement is a theorem for the explicit ideal-stream interfaces in §§6–8, **not** a claim that every deployed suffix already satisfies them. In particular, a schedule must supply the full-grid/source bridge, all mixed comparison classes, and the reset/conditional-answer interface. When an interface is missing, keep that node's original mixed tree. Section 9 gives the retained factors. There is no numerical outer-profile instantiation here.

The anchor is the **original complete public opening statement, receipt/output tuple, decision, and original oracle view**. No successful rerun may replace them. Extracted lattice sources are algebraic, possibly noncanonical and long; their *unit-multiplied products* have certified bounds. This note changes a proof, not Akita's protocol, transcript, implementation, or honest prover.

The theorem is classical, in the product experiment with independent ideal outer streams and the **same actual fixed auxiliary environment Theta** in every branch. It uses no CH assumption. It does not convert private hypothetical oracle values into persistent explicit ones, repair quotient zero-finding transport, or assert an ordinary-SIS reduction from an unrestricted-B ARO emulator. The corrected input-local-B contract in [ARO_WITNESS_SCOPE](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md) remains a separate obligation for any later ordinary-SIS corollary.

**Independent review:** NativeSourceFirstReview approved the conditional proof compiler after one correction: norm-stage error sums over **every selected source group**, as in Akita's `nu_j=sum_g nu_j,g`, rather than charging one group. Approval covers the local algebraic certificates, original-weighted success/work recursion, terminal A-only base and fallback rules under GRID/SOURCE/STATIC/LOCAL/REPLAY/TAIL. It does not instantiate those contracts for the proposed guest/schedule.

### Evidence inspected

* Primary indexed [Akita](SOURCE_PROVENANCE.md#akita), `qmd://lattice-papers/akita.md`, publication locator [ePrint 2026/1983](https://eprint.iacr.org/2026/1983). The supplied indexed text, not an independently downloaded publication revision, is authoritative for the equation numbering here. qmd retrieval directly inspected Lemma 3.22 (lines 1509–1547) and Definition 10.22/Theorem 10.23 (3951–3989); local primary text inspected §§3.7, 5–10 and the actual **Figure 5 image**, including its complete numbered order.
* [SOURCE_FIRST_EXTRACTION](AKITA_INSTANCE_ACCUMULATION_SOURCE_FIRST_EXTRACTION.md), especially §§2–6, supplies the actual-fixed-Theta rejection coupling and root-prequery argument. Its separate unrestricted-emulator citation is not imported.
* [PCS_EXTRACTION_COST](AKITA_INSTANCE_ACCUMULATION_PCS_EXTRACTION_COST.md), §§3–6, supplies the old mixed ledger and the distinction between expected work and strict derived-query caps.
* `jolt/Cargo.toml:304–313` and `jolt/Cargo.lock:182–184` pin Akita to `83574331d4e51f8cce1d8689d05592fa4ef4c138`. Immutable source inspected: [fold orchestration](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-verifier/src/fold/mod.rs), [terminal](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-verifier/src/fold/terminal.rs), [ring switch](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-verifier/src/stages/ring_switch.rs), [Stage 1](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-verifier/src/stages/stage1.rs), [physical norm](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-verifier/src/stages/physical_l2_norm.rs), and [Stage 3](https://github.com/LayerZero-Labs/akita/blob/83574331d4e51f8cce1d8689d05592fa4ef4c138/crates/akita-verifier/src/stages/stage3.rs).

## 2. Actual order and the permitted cuts

Figure 5 and §§5.3–5.5, 6–9 give the following order. The setup identity, schedule, ordered original handles, native points/projections and claims are bound first.

1. A receiving evaluation-trace fold performs its tensor prefix, if nonidentity: bind all column partials, draw row/claim batches, run its degree-two sumcheck, bind each final product, and reject zero transparent factors. Packing and identity cases omit this.
2. Bind the raw D opening image or final H payload. Its partials are fixed before the *subsequent* scalar batch and fold.
3. Draw the scalar claim weights, then the nonce-indexed folding tuple. A retry resamples the whole tuple. Coordinate streams have independent indices under the fixed round root/nonce, not transcript absorption of earlier coordinates' answers.
4. Form Eq. (109), including the selected quotients and all compression digits, and bind the successor. Usually this is an outer commitment. At the final nonterminal fold it is **TerminalInnerState**, the tuple of A images, instead.
5. Bind any required norm claims; draw alpha, then range anchor tau0 and relation-row point tau1 in the prescribed order; run range stages, norm fusion where selected, and the fused relation check. Intermediate evaluations precede the challenges that batch them. Each sumcheck message precedes its challenge.
6. The direct edge leaves one witness opening. An offloaded edge additionally runs the setup-product sumcheck and leaves the ordered pair `(registered setup source, successor witness source)` of Eq. (170). The receiver checks both. The final edge leaves no pending setup opening.

Thus a successor is fixed **before the first strong challenge, but after the weak fold challenge**. Successor handles can differ across folding siblings. There is no common successor across the weak-fold cut; the comparison is first within each strong block, then between the *incoming* source/opening paths of its exact folding branches. This is precisely the obstruction identified by Akita's strong–weak discussion following Theorem 10.27, not an application of a sequential strong-then-weak theorem.

The setup slot is predetermined by setup and schedule (§9.1). Its literal absorption again in Stage 3 does not make it prover-chosen: the registry and selected slot must already be determined by the strong root header. Its evaluation point and claimed value are allowed to change with the strong challenges. It remains a **separate handle**, not a segment of the witness source.

The pinned `verify_fold` confirms the critical cut: `derive_multi_group_stage1_challenges -> receive_next_witness -> ring_switch_verifier -> verify_stage1 -> replay_stage2 -> verify_stage3 -> validate_stage2_replay`. `NextWitnessPlan` distinguishes `OuterPayload` from `TerminalT`. The ring-switch function draws alpha, tau0, tau1 in that order. The delayed Stage-2 final validation uses the Stage-3 claim; a local obligation graph must retain that dependency rather than treating Stage 2 as independently verified first.

**Paper/source distinction.** Figure 5 places norm claims before its strong checks. The pinned source receives `l2_prefix` inside `verify_physical_l2_norm`, after `verify_product_prefix` has run inside Stage 1, and before its norm-specific batching challenges. The local lemma below permits this later placement because norm residuals only need to be fixed before *their own* randomness. This is not evidence of literal paper/code transcript equality. Likewise, the pinned code uses `RelationAddressGeometry`, a range-image plan and a physical-response plan: an implementation-specific full-grid bridge and degree/ordering ledger remain required. It is unsound to substitute the paper's mu-prime merely because a source function is named Stage 1.

## 3. Sources, exact branch predicates, and comparison classes

### 3.1 Strong-node source contract

Fix a fold child and the header immediately before its first strong challenge. It includes the incoming handles and opening payload, fold tuple, scalar weights, successor handle/shape, schedule and setup registry. Define the fixed source tuple `W` to contain:

* the successor's algebraic source blocks and source-path state, with their weak certificates;
* on an offloaded edge, the separately registered setup source and certificate;
* at a final edge, source blocks certified against the predecessor-bound terminal A-image tuple, **without invented B/F digits**.

A source identity excludes challenge differences, evaluation points, claimed values, partials and opening-local D/H state. For ordinary handles it includes source blocks, inner-image digits and the B/F path. Comparison is by the same shape and public payload, using the full occurrence classes of Corollary 10.17. At an A-only handle it is by shape and the same public A-image tuple.

`Decode_j(W)` must supply the **entire actual Boolean table used by every strong equation**, with its exact coefficient/variable order. This is a premise about the decoder and the protocol's leaf equations, not a canonical-shortness assumption. The table is over K; its MLE and all protocol evaluations may be over E.

Define `R_branch,j(header; W)` to mean:

1. every original native ring row of Figure 6 and its scalar opening row holds on the decoded/recomposed source, at the *fixed folding tuple*;
2. the common range predicate holds at every cell in the certified range domain; all required compression cells satisfy negative-binary support;
3. the selected integer response-norm relation and its bounds hold, using every scheduled response coordinate exactly once;
4. the setup source equals the installed canonical reference, when present, and the delegated setup scalar is the correct direct contraction whenever that scalar is used;
5. every claimed structural zero/padding condition needed to interpret (1)–(4) is either a decoder identity, an explicitly checked predicate with its own certificate, or is not asserted at all.

Some scalars in item 4 are sent during the strong block. They are intermediate truth predicates tied to `W` and the transcript, not part of a source chosen before their challenges. The final tested branch relation is the exact ring/range/norm relation needed for folding subtraction; a transcript-dependent delegated scalar is checked at its actual node.

The child source extractor must authenticate **all** outgoing leaves of this block, including both setup and witness claims, with their original points, factors and shapes. An extractor returning only a valid source commitment, or only the combined value of two different handles, is insufficient.

### 3.2 Full-grid and range rules

Eq. (109) is not merely response digits. It contains, in native block order, response/partial/inner-image digits, the main quotient intervals on the quotient-lift route, and each F/H compression interval followed by its own quotient interval. Eqs. (107)–(110) determine local addresses. Compression alignment, ring completion and Boolean-domain completion must be represented in the map. The quotient-free route has **no quotient intervals**. Group widths, digit counts and native ring dimensions are not silently rounded to one common shape.

Akita Eq. (202) proves only

`MLE(decoded full commitment grid)(r) = phi * claimed_value`.

Definition 10.13 explicitly does not impose zeros on algebraic-padding cells. Consequently, to use a child leaf as `MLE(w)(r)=v`, the schedule must provide one of:

* an exact full-grid identification with phi=1;
* a proved embedding/alias identity, including every relevant cell and factor;
* an explicit padding/used-slot reduction, whose leaves are authenticated by the child.

Dividing by a checked nonzero phi proves a scalar equality, **not membership in a padding image**. A range predicate on a used slice does not imply zero or range on other committed cells. Unread physical cells may remain unconstrained only if the relation and collision bounds really do not require them. This distinction also applies to compression ring-completion cells and to the canonical setup prefix. The setup reference comparison, not a fresh range proof, supplies its canonical zero suffix.

With the paper's full-grid range convention, every decoded witness cell is in `A_bstar={-bstar/2,...,bstar/2-1}` and every compression digit cell is additionally in `{-1,0}`. If the selected implementation checks a derived range image instead, it must prove which physical cells and alphabets this certifies; do not import the paper full-grid assertion without that bridge.

The verifier-enforced response bound is Eq. (112), and the sibling difference bound is Eq. (113):

`Z_inf = (bstar/2) * (b^delta-1)/(b-1)`,

`Delta_f = (bstar-1) * (b^delta-1)/(b-1)`.

Use these all-valid-source bounds, not the honest canonical base-b range or a probabilistic honest-response threshold. The incoming A rank must already support the selected successor certificate.

For a Euclidean route, the response digit map pi is injective onto every response cell used by A, after packing; reconstruct each integer coefficient before squaring. The direct route requires `max(U_dir,S_max)<q`, with `U_dir=N_A*(sum_h b^h B_dig,h)^2`. The digit-expanded route requires `|I_t| B_dig,h B_dig,k<q/2` for every segment/pair, centered lifting of every true pair claim, and the exact integer recombination Eq. (123). All `B_dig` are from the **certified** alphabets. Canonical integer encodings and the bound comparison are checked, not assumed. There is no extra packing factor on an already packed response. These are the hypotheses of Lemmas 6.3–6.4, Proposition 6.5 and Lemma 10.25.

### 3.3 Comparison and global collision outputs

For ordinary common handles, compare B/F paths first. A disagreement yields a B/F kernel vector at the mixed certified alphabet diameter. Otherwise inner-image digits agree. For differing source blocks, retain unit factors rho,rho' and bounded products `r=iota(rho)s`, `r'=iota(rho')s'`, and output

`z_A = iota(rho') r - iota(rho) r'`.

It is nonzero because the factors are units, and `A z_A=0`. Its coefficient bound is `k' b + k b'`, where `||iota(rho)||_1<=k`, `||r||_inf<=b`, and similarly for the other occurrence. Use the proved mixed Euclidean alternative only when both profiles certify it. No bound on an inverse unit is used.

Across exact **fold siblings**, also compare the shared D/H opening paths before subtraction; these were bound before the fold. Across **strong-source probes** at one handle, opening-local D/H state is not part of the source identity and need not agree. These are different comparisons. At terminal A-only handles, equality of the public A images replaces the B/F comparison and the same denominator-clearing formula applies.

A registered setup source is compared against its canonical reference with `rho'=1, r'=s_static`; the coefficient radius becomes `b+k||s_static||_inf`. Include that reference occurrence in the same class, even when its norm greatly exceeds an honest witness-digit norm. The inventory is the union of all native A/B/D/F/H instances, all source-probe cross-profile pairs, terminal A comparisons, and all installed setup references. It must cover changes of points, tail arities, opening routes and permitted schedules, not just diagonal instances. This is the mixed-maxima requirement of Eqs. (193)–(196).

## 4. Fixed-source local truth certificates for the strong block

Fix any `W` of the admitted geometry before the strong root answer. No assumption is made that it is honest or canonical. At every challenge stage, a deterministic predicate records the truth of all live obligations relative to `Decode_j(W)`. If a predicate is false, select the first false residual in a fixed order **before** the challenge. A local bad set is determined by that source and pre-challenge transcript. Outside it, verifier checks and true outgoing obligations imply true incoming obligations. Later adaptive messages cannot alter the already-defined residual.

For clarity, the following is a conservative certificate for the paper equations; exact selected schedules may use smaller proved charges. All field challenges here are uniform in E or uniform independent coordinates thereof.

| Node | Fixed-source residual/truth obligation | Bad mass |
|---|---|---:|
| Ring reduction alpha | Canonical reduced ring residual, or unreduced residual minus the *fixed carried quotient* times its modulus | `(d_max-1)/|E|` quotient-free; `(2d_max-1)/|E|` quotient-lift |
| Relation-row point tau1 | MLE of the nonzero vector of evaluated ring and scalar-row residuals, including A/B/D/F/H and opening rows | `ceil(log2 n_rel)/|E|` |
| Initial range anchor tau0 | MLE of `Q_bstar(w[x])` over the complete checked Boolean domain | `mu'/|E|` |
| Range/GKR product round | True partial-sum polynomial for the deterministic Boolean product-table circuit; a false accepted message differs from it | `(d_s+1)/|E|` per round |
| Range interstage frontier | Already-sent errors in child evaluations versus the true Boolean product-node MLEs | `1/|E|` for a nontrivial normalized independent affine batch; `(m_s-1)/|E|` for power batching or as the conservative paper ledger |
| Final binary-support anchor | MLE of `1_Ibin(x) w[x](w[x]+1)` at the final range point `r_virt` | `mu'/|E|` if Ibin is nonempty |
| Carried-range/binary separation | Affine residual `R_rel + gamma R_range + zeta_bin R_binary` | at most `(1+1_{Ibin nonempty})/|E|` conservatively |
| Fused relation rounds | Eq. (160), including actual norm-binding term if present; the true polynomial has individual degree at most three | `3/|E|` per round |
| Norm-specific scalar stages, for each selected source group g | That group's pair-batch, range/norm merge, and norm-to-witness binding of Eqs. (124)–(128) | `2/|E|` direct; `(N_pair,g+delta_f,g)/|E|` digit-expanded; sum over the actual group stages |
| Setup-product rounds | Correct Eq. (168) for the fixed registered source and the now-fixed public weight | `2/|E|` per round |
| Exact maps/final leaf checks | Recomposition, aliases, integer lifting and true original leaf equations | zero, if their stated deterministic hypotheses hold |

**Proof details that matter.**

1. A nonzero ring residual has the stated polynomial degree. The quotient witness is already part of the bound source. The quotient-free rule evaluates the reduced product via the transpose kernel, not a product of arbitrary ring evaluations; it never divides by `alpha^d+1`. Row batching is multilinear in its *row selector*, not a flat coordinate-wise interpolation argument.
2. Set `s[x]=w[x](w[x]+1)` at Boolean cells and define every range-tree internal node by Boolean-row multiplication before interpolation. These are deterministic tables even though they have no separate commitments. Eq. (115) is the MLE of the final Boolean residual at tau0. A nonzero residual table gives a nonzero polynomial. In each subsequent sumcheck, the accepted message polynomial and the true partial-sum polynomial differ if the input claim is false. Their root bound applies uniformly to later messages. Equality-factored message compression does not raise or erase this true degree bound; the relevant summand degree is `d_s+1`.
3. One must not write `s_tilde(r)=w_tilde(r)(w_tilde(r)+1)`. Eq. (160) is what binds the former, defined by Boolean multiplication, to the source.
4. Binary support is a separate predicate, despite reusing range challenges. For the fixed nonzero binary residual polynomial, expose `r_virt` sequentially. At each coordinate choose a nonzero remaining coefficient; making that coefficient vanish costs at most `1/|E|`. Thus adaptive intervening messages do not prevent the `mu'/|E|` bound. This is **an additional safe local-error charge here**, not a second extraction-tree factor. The paper's reuse of a full nested range tree in Lemma 10.28 does not by itself prove zero *additional* single-transcript source-first error.
5. For each affine or power batch, all error coefficients relative to W are fixed before its own draw. If an earlier component is already false, coefficient separation or a first-false choice prevents cancellation except on the declared root set. Shifted powers in Eq. (127) have degree delta_f, not delta_f-1. The norm's field truth is converted to integer truth only through the bounds in §3.2.
6. Offloading is a DAG: the Stage-2 final equation uses a delegated setup scalar, whose truth is supplied by Stage 3. The setup weight may depend on alpha, tau1 and the relation point; it need only be fixed before the Stage-3 sumcheck. Compare the setup source to the canonical reference first. If they agree, Eqs. (166)–(169) are the exact direct scan; if not, return the registered collision. No circular leaf is silently declared true.

One convenient upper bound, with beta indicating nonempty binary support, is

`eps_str,j <= [ (xi_ring-1) + s_rel + mu' + sum_s mu'(d_s+1)`

`                 + sum_frontier(m_s-1) + beta mu' + (1+beta) + 3mu'`

`                 + n_norm + 2 o_j nu_S ] / |E|`,

where `xi_ring=2d_max` or `d_max` and `n_norm=sum_g n_norm,j,g`, with each group's charge `n_norm,j,g=0`, `2`, or `N_pair,j,g+delta_f,j,g` on its coefficient, direct-norm, or digit-expanded route, respectively. The sum is the actual selected group norm-stage ledger: Akita defines `nu_j=sum_g nu_j,g` immediately before Eq. (218). A singleton Euclidean restriction simply leaves one nonzero summand. Only actually selected stages are included. This is an upper bound on *fixed-source false-to-true transport*, not a claim of an optimal native knowledge error. Every extra implemented range-image, padding, selector or exceptional branch must add its actual certificate; it is not hidden in this formula.

The certificate is mechanically finite once the selected address/degree/ownership ledger is fixed. Its first predicate is the branch relation, its final predicates are all owned outgoing source leaves, and every omitted challenge-free map must be an exact identity. These verified algebraic implications instantiate the local contract of SOURCE_FIRST_EXTRACTION at a paper-level full-grid strong node. They do not establish that a presently unselected concrete receipt has that ledger.

## 5. Tensor and scalar prefixes are separate replacements

### 5.1 Tensor certificate

At an evaluation-trace receiver, the **incoming** handles, native points, values, basis, head/tail split and packing bijection are fixed before the tensor prefix (Definition 10.18). At the terminal the handle is the A-image tuple. The successor formed by this receiver's later fold is the wrong source for this cut.

For fixed incoming K tables, canonically form each packed table `g=Pack(f)` and the true column partials. If a claimed column differs while head recombination accepts a false input, its base-field coordinate array differs. Regrouping those coordinates as rows is invertible over K. Hence some row equation `(star)` in §3.7 is false. Its MLE in the row-selector eta is nonzero, with bad mass at most `kappa/|E|`. The normalized independent claim batch adds at most `1/|E|` if nontrivial; the shared degree-two sumcheck adds `2m_max/|E|`.

For unequal tail arities use exactly the cylindrical extensions of Eq. (55), with native prefix `rho_a` and

`theta_a=A_eta,a(rho_a) * product_{t=m_a}^{m_max-1}(1-rho_t)`.

The final truth predicates are **each** `h_a,i=theta_a g_a,i(rho_a)`, authenticated by downstream extraction. The verifier rejects every zero theta. There is no soundness exception obtained by dividing by zero, and the honest theta-zero probability is a completeness term, not a new accepting-soundness error. Thus

`eps_tensor <= (kappa + 1_{N_tensor>1} + 2m_max)/|E|`.

This reproduces the fixed-source soundness statement of Theorem 3.11, rather than its more expensive extraction-tree geometry. Identity tensor reduction is an exact map with no root, no error and no factor-two replacement.

The downstream map must already authenticate per-claim reduced values: a weak source satisfying only the receiver's later scalar combination is not enough. Either retain that internal combination tree or certify it by its own source-first node first. The tensor prefix and post-D/H scalar combination use different challenges and are not double-counted as one operation.

### 5.2 Scalar combinations

For a post-opening-payload scalar combination, the fixed incoming source determines the true partials by the admitted contraction. A successful downstream weak certificate authenticates the weighted scalar equation. Compare sources across the scalar root, then test each original claim against the original source. The error is `(Lambda-1)/|E|` at the application power batch, and `1/|E|` for a nontrivial normalized independent affine batch. Its root header must retain the already-bound D/H payload as well as the original source handles; source comparison itself still discards opening-local D/H state.

It is permissible to replace this node only when its downstream relation really provides those weighted leaves for that source. If the normalized batch's key/prefix, handle ownership or map is not supplied, retain its flat factor `N_clm`. Likewise retain the tensor factor `2^kappa N_tensor 3^m_max` if its own incoming-source cut is not instantiated. Algebraic appearance alone does not delete a factor.

## 6. Resettable maps and the exact weighted recursion

### 6.1 Universal map interface

Let J be the ideal outer table and fix setup, the prover tape and actual Theta. Include the original verifier queries in the cheap original execution so that every used root has an exposed entry; alternatively use the paper's explicit unqueried-output convention and Q+1. Fix each suffix procedure's auxiliary tapes independently of J when applying a sampling lemma, then average over them. Freshness for the enclosing sampling game remains available.

For every cut r, the procedure returns

`(original_output, original I_r, original prechallenge header, status, certificate)`.

It determines `I_r` from the original execution even on rejection, suffix failure, cutoff, or collision. Here I denotes the stage input/key; source probing additionally records its first fresh-query ordinal. It never substitutes the key or receipt selected by a successful rerun. A success certificate is either the required relation witness for that original header or a verified named collision. Collisions are absorbing successes: discovering one in any private attempt can terminate immediately.

The interface is required for **every legal restriction generated by enclosing samplers**. Such a restriction fixes ancestor stage inputs and prescribed answer-coordinate prefixes, using the exact conditional sampler; all remaining random coordinates have their specified conditional product law. It does not mean arbitrary conditioning on successful ancestors. The original run for this restricted game is still defined, still has the same Q bound, and still supplies its own original index on failure. Stage tags and complete transcript keys ensure later matching inputs preserve all earlier headers.

All counterfactual H_o continuations are private tables with the prescribed saved-prefix/override discipline. Every actual Theta call, including a failed trial or a child extractor, goes to the same Theta. Fixing Theta makes it an auxiliary function environment, not a source of freshly reset randomness. A concrete stateful emulator would require a separate composable contract; the actual-fixed-Theta theorem does not assume one.

Call this **REPLAY**. Lemma 3.22 supplies the mathematical original-index support/sampling interface for Akita ideal streams; SOURCE_FIRST_EXTRACTION supplies the source-probe kernel. An implementation that initializes a child with a fresh unrelated oracle or keeps post-root original answers when they should be resampled does not satisfy REPLAY.

### 6.2 Retained flat and nested nodes

For a retained stage r, let `M_{r+1}(J)=(V(J),I_r(J))`, with its entire child extraction already included in V and its cost. This is the suffix map used in Lemma 3.22—not a map of acceptance alone. Its success output keeps the original header.

For candidate input i, define

`A^0_{r,i}(J_{-i})={z:I_r(J_{-i},z)=i}`,

`P_{0,r}=sum_i Pr[A^0_{r,i}(J_{-i}) is nonempty] <= Q+1`.

The support bound uses the original execution: changing entry i cannot affect execution before its first query to i; if that query never happens, only the original output challenge input can be selected. Private attempts do not enlarge this support to their total query count.

For a scalar arity-k extension of a nested family, the **unconditioned restricted-game** recurrences are

`s_u >= s_{u+1} - P_{0,r}(k_u-1)/|S_u|`,

`C_u <= k_u C_{u+1} + (k_u-1)P_{0,r} gamma_u`.

A flat coordinate family has

`b_r=1+sum_u(k_{r,u}-1)`, `e_r=sum_u(k_{r,u}-1)/|S_{r,u}|`,

`s_r >= s_{r+1}-P_{0,r}e_r`,

`C_r <= b_r C_{r+1}+(b_r-1)P_{0,r}gamma_r+a_r`.

A nested family uses `b_r=product_u k_{r,u}`, with coordinate-by-coordinate overhead (or a proved uniform envelope gamma_r giving the displayed form). The term a_r prices deterministic extraction, validation and collision output at this node. Matching-index/prefix checks precede expensive child work. Full raw answers are sampled conditionally, including rejected words and unused tape suffixes; prescribing decoded coordinates is not a preimage operation on a concrete hash.

**Why child failures are not reused conditionally.** The good fiber is

`G_i(J_{-i})={z: I_r(J_{-i},z)=i AND V(J_{-i},z)=1}`.

Child failures remove mass from this fiber *before* the weighted sampling game is applied. Its starting success is `s_{r+1}=Pr[V=1]`, not `Pr[accept]` with a later assertion that the child fails with the same probability on forced answers. Sparse/pruned fibers are charged by the coordinate threshold in the same game. Consequently the loss is additive in the original-reference success recurrence; it is neither a fresh `(Q+1)` multiplication of every child error nor an unproved reuse of a conditional child theorem.

If only a pointwise child failure bound `delta(h,z)` is known, its contribution is the actual integral `integral delta(h,z) dnu(h,z)` for this stage's continuation measure. Under a success-conditioned trial it is generally `Pr[child failure AND gate|h]/Pr[gate|h]`, not the unconditional delta. The theorem never substitutes the latter. Branch-dependent schedules use these actual kernels/integrals or valid uniform bounds; a single homogeneous product is not inferred.

### 6.3 Source-first node, including failed children

Let B be the current downstream map: cheap original execution C followed, only on acceptance, by the expensive child source extractor D. B may fail; on success it authenticates the block's outgoing source leaves. At fresh root ordinal i and saved prefix sigma, define in the **same reference continuation law**

`a_i(sigma)=Pr[C accepts and selects this i/key/header | sigma]`,

`p_i(sigma)=Pr[C accepts, selects it, and D returns a valid source | sigma]`.

The original source-success event has mass p_i, not a_i. On original rejection/failure/collision, return that original result. On original valid source W0, repeat independent private continuations from the saved pre-root state. A candidate not accepting with the identical selected root is a cheap failure. Only an exact match invokes D; a failed D restarts the trial. On success compare W' to W0. A disagreement or discovered child collision terminates with a verified collision; otherwise test the branch/prefix relation using **W0** and return it for the original statement only.

If p_i>0, renewal yields a successful probe source distributed as B's source conditional on that success, independently of the original future. Prefixes with p_i=0 have zero original source-success mass and do not start the loop. In the soundness coupling sample this hypothetical source before answering each encountered root, using independent continuation randomness. It is not asserted that an original challenge conditioned on acceptance is uniform.

With dmu_i the original reached-prefix measure and nonnegative trial cost Y_i,

`E[1_original_source_success * probe work]`

` = sum_i integral_{p_i>0} p_i * E[Y_i|sigma]/p_i dmu_i`

` = sum_i integral_{p_i>0} E[Y_i|sigma] dmu_i`.

This is the correct child-failure weighting. Failed child calls appear in E[Y_i]; p_i in the denominator includes them. There is no replacement of p_i by a_i or by a claimed conditional success lower bound.

For expensive child work, put `Y_i=1_exact_gate * work(D)`. The exact selected-index events are disjoint in one reference execution, so

`sum_i integral E[Y_i|sigma] dmu_i = E[original gated child work]`.

Thus, if T_C bounds cheap execution/replay/sampling and A_SF bounds charged comparison/projection work in expectation,

`C_SF <= (q+1)T_C + 2 C_D + A_SF`.

This is proved afresh as a map transformation and holds for each nonnegative additive resource. It is not a per-acceptance or per-prefix upper bound. Comparison work must include decoding all actual source cells; it need not be cheap relative to D. A loose gate that can match multiple selected roots loses the disjoint-event identity and does **not** inherit the coefficient two.

For soundness, the local truth certificate and hypothetical fixed source give

`s_SF >= s_B - beta_SF`,

`beta_SF = q eps_local + binom(q,2) gamma_root`.

Here gamma_root bounds the probability that a previously queried proper-descendant key guesses the fresh root component bound into that key (1/|E| for the field roots above). Complete key ancestry, atomic-vector or independently indexed root-family conventions, and all prequeries are essential. Collisions in probes can only improve success or shorten work. No child failure probability is charged again: it was already excluded in s_B. This is SOURCE_FIRST_EXTRACTION's original-event theorem applied to the new local predicate.

A bounded child meets the published local compiler contract directly. For an almost-surely terminating finite-mean child, the same equalities follow by truncating nonnegative work, applying Tonelli/monotone convergence to renewal sums, and taking increasing stopping cutoffs; valid finite successful executions are eventually included. Fixing auxiliary tapes is understood almost surely, followed by averaging. If these termination/integrability conditions are unavailable, supply strict child caps and charge their actual original-reference failure events instead. Finite expected time alone is not a strict resource certificate.

## 7. Terminal base and the native induction step

### 7.1 Actual terminal base

Section 8.2 fixes `(shape, (t_i)_i)` with `t_i=A_term s_i` as the predecessor-authenticated public input. The terminal reabsorbs this tuple, performs its tensor prefix if nonidentity, absorbs the clear partials e_i, then draws the fold challenges and reveals a response with directly checked encoding/norm. There is one recursive-witness group, one claim, no pending setup opening, no B/F commitment, no D/H path and no terminal range/relation sumcheck.

The exact accepted equations are Eqs. (163)–(165):

`A_term z = sum_i c_i t_i`,

`a_term^T G z = sum_i c_i e_i` in the terminal ring,

`theta_term sum_i chi_blk(i) T_rho_pack(e_i) = v_tensor` in E.

Keep the terminal flat factor `F_T=1+B_term` and error `e_T=B_term/|C_term^acc|`. For the two children differing only at i, set `rho_i=Delta c_i` and `r_i=Delta z`; the accepted unit-difference property yields `s_i=rho_i^{-1}r_i`. Subtraction gives `A_term s_i=t_i` and the partial relation. The trace equation is checked on an accepting child; it is **not** obtained by subtraction.

The products r_i have the actual difference bounds of the directly accepted clear responses. For coefficient bounds the raw profile gives `K_1=2 kappa_1`, `B_inf=2 Z_inf` and diagonal A collision radius `8 kappa_1 Z_inf`. A Euclidean route uses its actual operator/response certificate and the denominator-clearing inequalities. No terminal source itself is asserted short.

This base yields a certificate for the reduced packed claim. Either retain the terminal tensor tree as in Proposition 10.20 or apply §5.1 using the same A-only source comparison; then it certifies the original incoming full-grid claim. Exact terminal equations add no field error. In the pinned source the checks `setup_prefix_opening.is_none()`, singleton response layout, `TerminalTFields`, `TerminalEFields`, and direct ring/trace calls enforce precisely this structural distinction.

### 7.2 One nonterminal step without a strong tree

Assume a descendant map authenticates the original outgoing source leaves or returns a registered collision. Under a fixed folding child, use the source-first map of §6.3 with §4's certificate. A successful noncollision result is an **exact branch relation**, with certified response and all carried-digit bounds. It is the conclusion of the strong-extraction portion of Theorem 10.23, even though it was obtained by checking an extracted source instead of interpolation.

Apply the retained flat sampling game to these good continuations. Its factor is

`F_j = 1 + sum_g W_j,g`,

and its error is `e_fold,j=sum_g W_j,g/|C_j,g^acc|`.

The flat family need not be a star: for each coordinate it supplies two children agreeing outside that coordinate, as in Definition 10.22. Compare the common incoming source paths and D/H payload paths across certified branches. If they differ, output the proper collision. Otherwise subtract the two exact response relations at each coordinate and divide only by the certified unit difference. This gives exactly the weak opening equations and certified products of Definition 10.14. Later strong challenges and successor handles need not agree across siblings. They have already disappeared from the exact native branch predicates.

Finally authenticate any earlier scalar and tensor prefix by a retained tree or its own admitted source-first map, in reverse protocol order. Compare repeated incoming original handles and setup references. The resulting map retains the same original index/output interface and is suitable as the child at the preceding level. This proves the semantic induction; no unchanged-source assumption is made across different successor commitments.

## 8. Schedule-parametric theorem, success, and expected work

**Theorem (hybrid native source-first extraction).** Fix a setup-independent finite admitted schedule family, or one admitted schedule, with:

* exact shape/address/full-grid and weak-opening interfaces of §3;
* verified local truth certificates for every chosen source-first block;
* unit-difference and fixed-filter conditional samplers for retained folds;
* REPLAY, complete root/descendant-key binding, and original-index preservation under legal enclosing restrictions;
* the mixed collision inventory and installed reference certificates;
* terminating/integrable maps, or explicit bounded maps with charged cutoff failures.

Choose a disjoint partition of the actual challenge ledger into retained mixed stages and source-first blocks; no challenge has two owners. Start at the actual terminal base of §7 and compose the maps backwards. Then the extractor returns either an algebraic batch witness for the **original** Eq. (202) statement or a verified inventory collision, except with the loss below. It preserves the original external output/view and actual Theta, without requiring honest proving, canonical source recovery or CH assumptions.

Let p_acc be the original acceptance probability. For each retained node r set `ell_r=P_{0,r}e_r`, `lambda_r=b_r`; for each source-first node t set `ell_t=beta_t`, `lambda_t=2`. Let `tau_r` denote a cutoff loss in that node's **reference map experiment**, not an unweighted conditional child guarantee. Then recursive application of §6 gives

`Pr[original accept AND neither witness nor collision]`

` <= sum_retained P_{0,r} e_r + sum_SF beta_t + sum_owned_cutoffs tau_r`.

Use `(Q+1)` as a conservative common bound on P0. Child failures are propagated by the unconditioned success s, not multiplied anew by tree sizes. If only nonuniform conditional cutoffs are supplied, replace the final sum by their actual induced-kernel integrals. A global cap applied after defining the complete ideal extractor simply adds its one original-experiment tail delta_L.

Write each proved work recurrence as

`C_r <= lambda_r C_{r+1}+A_r`,

with `A_r=(b_r-1)P_{0,r}gamma_r+a_r` at a retained homogeneous node, and `A_r=(q_r+1)T_C,r+A_SF,r` at a source-first node (absorbing any original cheap work consistently, once). Then

`C_1 <= (product_r lambda_r) C_base`

`       + sum_r (product_{v<r}lambda_v) A_r`.

This is a recurrence in actual supplied cost bounds. Each C prices the complete child procedure including its failures, private raw-stream samplers and actual Theta calls. The derivation uses the exact matching gate, original support and renewal identity; therefore **there is no unexplained 'factor two per level' assertion**. Two independently replaced blocks at a receiver can contribute two coefficients two; an identity contributes none; missing local contracts retain their tree coefficients. Algebraic source decoding/comparison may dominate A_r. Branch-dependent ledgers require the actual measure-valued recursion or a proved uniform envelope, not this homogeneous product by fiat.

If every nonterminal strong block is admitted but tensor and scalar prefixes are retained, the leading continuation coefficient is

`(product_retained_prefix b_r) * (product_nonterminal 2F_j) * F_T`,

with any retained terminal tensor factor included among the prefixes. This replaces the strong interpolation factors in the old ledger, **only under those contracts**. It neither proves this coefficient polynomial for an arbitrary family nor evaluates it for an unselected receipt. The depth, fold widths, remaining factors, output size, decoding cost and sampler inverse-acceptance bounds must all meet the admitted asymptotic/resource bound.

Proof: terminal subtraction supplies the base. At each chosen strong block the fixed-source theorem transforms the descendant source map into an exact-branch map. The weighted flat game transforms that into the necessary branch family with its actual child-success mass; native path comparison and unit subtraction yield the original incoming weak source. Prefix transformations close the input claims. Induction establishes the output relation, preserved original indices and collision alternatives. The two success inequalities telescope additively and the work inequalities expand as displayed. Average over fixed Theta, setup and tapes only after these original-event bounds. For adaptive selection from a fixed family, use uniform admitted bounds and the union inventory without conditioning the setup distribution on the selection. ∎

## 9. Noninstantiable nodes and their retained factors

The following are exact fallback rules, not permission to assume missing predicates.

| Unsupplied contract at a node | Consequence |
|---|---|
| **GRID_j:** exact child full-grid leaf to strong witness/range-image map, all cells/factors/addresses | Do not run source-first at this strong node. Keep its admitted original tree. If the *old* theorem also lacks the same shape bridge, the schedule itself is unadmitted; a tree does not cure that semantic gap. |
| **SOURCE_j:** common-handle weak source certificate and every mixed comparison bound | Keep the old strong extraction construction where its inventory is admitted; otherwise neither route is admitted. |
| **STATIC_j:** registry-selected canonical source/reference and exact setup weight | Keep setup-product tree `3^nu_S` and receiver combination/tensor factors. Without the reference/weight contract the old offloading theorem also remains unavailable. |
| **LOCAL_j:** exact selected range/product/binary/norm/row graph, including actual scalar ordering and degrees | Retain the corresponding strong block, not an invented lower-degree replacement. |
| **TENSOR_h:** incoming-source ownership, invertible packing, native prefixes, nonzero factors and individually authenticated output claims | Retain `2^kappa_h N_tensor,h 3^m_h`; identity remains factor one. |
| **BATCH_h:** correct source-determined scalar leaves and exact root key | Retain Lambda for the application power batch or N_clm for the normalized affine family. |
| **REPLAY/KEY:** legal restricted-game kernel, original key on failure, cheap exact gate, prequery accounting, conditional raw samplers | No source-first replacement at that node. Native Lemma 3.22 still requires its own original-index/key/sampler interface; concrete hash programming is not a fallback proof. |
| **TAIL:** finite mean/termination, sampler bounds, or explicit original-weighted cutoffs | No asserted polynomial/strict-resource reduction; retain a bounded admitted construction if supplied. |

For reference, a retained nonterminal strong block has the old coefficient

`L_str,j = xi_j * 2^s_j * 2^mu'_j * product_s(d_j,s+2)^mu'_j`

`          * product_frontier m_j,s * C_scalar,j * 4^mu'_j * 3^(o_j nu_S,j)`.

Retain the weak-fold factor F_j separately. `C_scalar` is the real scalar-stage ledger (gamma/binary and selected norm stages), not an unspecified polynomial. Sequential stages cannot be fused into a cheaper flat family merely because their formulas are affine. Binary support reuses the final range tree, and norm fusion adds no sumcheck rounds; neither observation removes its scalar separation obligations.

The current outer receipt has no selected complete schedule/geometry or reset implementation in the assigned evidence. The pinned code establishes important **ordering facts**, not GRID/LOCAL/REPLAY for that receipt. In particular its physical norm plan must be related to the certified shared alphabet rather than only an honest positional base, and its range-image geometry to the full source decoder. Accordingly, this note establishes a schedule-parametric proof compiler and the paper-level algebraic certificates; it does not mark every concrete native suffix or outer receipt admitted.

## 10. Cutoffs, moments, and what this does not buy

For K probe trials at one source-first root, the source-success-weighted failure is exactly

`sum_i integral p_i(1-p_i)^K dmu_i <= q/(K+1)`.

For a complete ideal extractor with charged work Z and a strict global cap L,

`delta_L=Pr[Z>L] <= E[Z]/L`.

The cap must include original verifier/decider work, rejected replay trials, every private stream bit/rejection sample, source decoding, all actual ro/wo/Vhat calls in Theta, and any call made by output predicates. Query ceilings derived for another producer are not inherited by expectation cancellation. A cutoff returning failure preserves the recorded original output when available; it cannot manufacture an alternate original success.

For geometric G with success p,

`E[G^2]=(2-p)/p^2`,

so original weighting by p gives `(2-p)/p`. There is no uniform second-moment bound from the first-moment identity. More generally the weighted m-th moment can behave as `p^(1-m)`. A moment claim needs a proved lower bound on positive p, integrability of the corresponding inverse-success quantities under the reached-prefix measure, a suitable tail theorem, or an explicit cap. Nested source-first composition must meet these conditions for its actual kernels; expected child work alone does not control the higher moments of the enclosing sampler.

The same first-moment recurrences can price actual Theta calls separately from ordinary work. They cannot silently replace a random call count inside a quadratic collision/error expression: use a strict cap or the required proved moment. They also do not yield a persistent ARO emulation theorem. Any ordinary-SIS corollary must apply a valid corrected emulation contract to the complete bounded producer, with one consistent history and all assumptions checked; the unrestricted-B assertion criticized in ARO_WITNESS_SCOPE is not available.

No honest proof-size, proving-time, memory, verifier-time or pre-PCS accumulation claim follows from this extractor improvement. No protocol/code changes, builds, tests, formatters, benchmarks, git operations or proof executions were performed. The deliverable is this research theorem and its explicit admission/fallback ledger.
