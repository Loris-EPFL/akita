# D1 ordinary outer PCS: full-ring E2 carrier and finite direct suffix

## 1. Decision, anchor and scope

This is a **proposed protocol and finite generation/validation rule**, not implemented ordinary Akita support, an installed guest profile, a concrete soundness theorem or a performance result. It resolves the carrier/suffix choice left open in [EXTENSION_MIGRATION §§3.2,6](AKITA_INSTANCE_ACCUMULATION_EXTENSION_MIGRATION.md). The selected family is `D1OuterFullRingDirect/v1`: **no setup offloading; one complete grouped nonterminal root; its complete Eq. (109) successor is created as an A-only terminal source; one genuine full-ring direct terminal.** The intervening full-commitment recursive suffix has length zero. This is an intentional, potentially expensive finite family, not an assertion that the production planner's preferred suffix still works. There is no unfinished opening at acceptance.

The D1 anchor is unchanged: the fixed-public inner guest checks the original T<=16 archive language and the unchanged inner accumulator (K=q128, E2, full S128 scalar extension, original weak-source paths, legal segment endpoints, final-domain check, three inner shrinking folds and its genuine terminal). The outer proof proves the complete execution of that guest, with `x_o=(D1,id(I),X)`, exact success `postcard01`, `panic=false`, original untrusted `C_archive`, and installed trusted `C_static`. This note neither creates per-segment proofs nor changes the inner 33-role registry. The outer matrix inventory is independently generated below.

Use the pinned Jolt `0791f79a55c3fadc3d327c78a72228696c338468` and Akita `83574331d4e51f8cce1d8689d05592fa4ef4c138` source interfaces as migration targets. New commitments are provisioned under this new outer family before proving. Once created, their source maps, public payloads, supplied digits and paths are immutable. No already formed commitment is converted into another encoding, and unsupported old keys are not reinterpreted as this family.

Fix a setup-independent finite installation descriptor J (guest/codec bounds, source semantics, candidate bounds, mathematical policies and finite sampler caps). In the security experiment sample `I=Provision(J;r)` and retain that I throughout the execution. Matrix assumptions are statements about this setup distribution, not assertions of hardness for every fixed installed key. The ideal outer oracle is independent of the inner experiment; the concrete outer Fiat–Shamir use remains the separately named extraction-use heuristic, with unknown loss/resource translation. No CH-O assumption hides a carrier gap.

## 2. Exact carrier and source interface

Set

\[
q=2^{128}-2^{32}+22537,\quad K=\mathbb F_q,\quad
E=K[U]/(U^2-5),\quad S=K[Y]/(Y^{128}+1),\quad S_E=E[Y]/(Y^{128}+1).
\]

Primality of q and nonsquareness of 5 are mathematical premises (the latter's exact residue check is recorded in [SECURITY_PROFILE §2](AKITA_INSTANCE_ACCUMULATION_SECURITY_PROFILE.md)). Every source and A/B/D/F/H matrix is over K. Scalar messages and points are over E. A partial is `e=e0+U e1` with **two complete degree-128 S planes**, hence 256 K coefficients. E is not a subfield of S and is not encoded by the pinned `FpExtEncoding` ring-subfield map.

### 2.1 Frozen ordinary source plan

The installation enumerates all original stage-8 handles, not just a guessed three-handle total: Main, Archive, Static, and any additional mandatory source genuinely required by the compiled ordinary frontend. It may not omit a frontend-required handle to make the row fit. Archive and Static are ordinary original byte-advice sources under the actual Jolt adapter/word decoder; trusted ownership of Static and untrusted ownership of Archive are retained.

For each handle g, freeze `P_g,B_g,M_g,delta_g`, natural decoded length, physical layout/permutation, polynomial identity/alias map, source alphabet, decoder, zero mask and producer contract. B and M are powers of two. Its actual source coordinates are

\[
s_{g,p,i,x,\ell,j},\quad p<P_g,\ i<B_g,\ x<M_g,\ \ell<\delta_g,\ j<128,
\]

with physical address `((((p B+i)M+x)delta+ell)128+j)` after the registered permutation. The A column is `x delta+ell`; `m_A=M delta`, not the total source length. Main has the actual number of original native columns, direct scale-one one-digit physical coefficients, and physical Boolean source domain. It does **not** have an assumed outer P=56. Dense field-valued source digits use base32 and depth26; a narrower bounded producer may use the exact smaller base32 depth supplied by its source contract if its entire allowed decoded range is representable. The finite candidate rule below fixes each such depth before choosing matrices. Every accepted noncanonical source decomposition within that contract remains legal.

Fold admission restricts the registered storage permutation. Write a raw block
as `b=pB+i` and a raw column as `a=x delta+ell`. It must have the form
\[
\pi_g(b,a,j)=\bigl(\pi_{B,g}(b),\pi_{A,g}(a),j\bigr),
\]
where \(\pi_{B,g}\) permutes all \(P_gB_g\) blocks and \(\pi_{A,g}\) permutes all
\(M_g\delta_g\) columns, identically for every block and coefficient. Denote the
physical source by \(s^{\mathrm{phys}}_{g,t,a,j}\); then the displayed raw cell is
\(s^{\mathrm{phys}}_{g,\pi_{B,g}(b),\pi_{A,g}(a),j}\).
An arbitrary coefficient permutation does **not** commute with native ring
multiplication and is not admitted merely because it is bijective. The original
physical A columns, source folds and matrix namespaces stay unchanged. General
decoder-adjoint maps remain available for ordinary claims; they do not authorize
a different fold map. The selected identity-layout sources are unaffected.

The decoded K Boolean table is the fixed linear map `f_g=Dec_g(s_g)` (with known public affine constants subtracted from claims, if a frontend alias requires them). Its adjoint, not a new witness, supplies all ordinary evaluations. Main's zero-omission semantics and Jolt paired bit/point reversals remain part of this exact map. One-hot execution constraints are still frontend obligations; source Booleanity alone does not replace them.

The actual physical predicate must be specified from **outer verifier-bound metadata**. Required physical zero cells include only the producer's registered holes/omitted native slots and padding which that contract genuinely requires. An inactive-cycle suffix is included only if its boundary is publicly authenticated by the outer statement/layout. No inner segment ell/Nseg or private honest trace length is imported. If only the padded N-cycle table is exposed, every cycle in that table is treated normally, subject to the full physical Boolean predicate and the actual frontend execution checks. This rule neither assumes honest sparsity nor adds an unsupported inactive mask.

Define the raw Boolean view with `Pbar=nextpow2(P)`, `deltabar=nextpow2(delta)` and low-to-high axes `(j,ell,x,i,p)`:

\[
\operatorname{rawaddr}=j+128[\ell+\bar\delta(x+M(i+Bp))].
\]

Actual cells are the exact original source cells; absent p/ell slots are **virtual zero values of the view**, not new A columns. Physically committed holes remain physical cells and require the declared zero predicate if zeros are claimed. Raw-grid size is `Nraw=Pbar B M deltabar 128`; physical size is `P B M delta 128`.

### 2.2 New opening method and source encoding

Add a genuinely distinct proposed method

`OpeningMethod::FullRingScalarExtension { extension_degree: 2, scalar_basis_id: E2Nonresidue5, query_map_version: RawCoefficientV1 }`.

Its descriptor has a new noncolliding method tag (proposed tag2 after the pinned 0/1 methods); its physical width is `k*d_A=256`, its challenge ring is the entire A/source S128, its map below is not EOR, and `requires_extension_opening_reduction` is **false**. It is legal at both the root and terminal. It does not require `k*d_A | d_A`. D divisibility requires only `d_D | k*d_A`; this family fixes `d_D=128`.

Source encoding remains the existing **`CommittedSourceEncoding::CanonicalCoefficientTable`**, now permitted explicitly for this method at *every* producer, including terminal creation. “Canonical” names the physical coefficient representation; it does not assert canonical gadget paths. No new E-valued source table is introduced. Terminal descriptors must contain the source encoding instead of inferring it from `E::DEGREE`. `TensorSubfieldProjection` is forbidden in this family.

Runtime dispatch must match the declared opening method and source encoding, not `E::DEGREE>1`. Proposed prepared material `FullRingRawPoint { coefficient_weights, digit_weights, position_weights, block_weights, polynomial_weights }` has no trace/tensor factors. Prover partial generation, sparse one-hot kernels, D sizing, Eq109 assembly, coefficient rows, verifier scalar checks, terminal parser, catalogue replay and challenge manifests all use that same map. This design does **not** install a fake `FpExtEncoding<K>` implementation for nonresidue5.

### 2.3 The full-ring opening map

At one raw endpoint `r=(r_j,r_ell,r_x,r_i,r_p)`, define the S-linear map on a
physical response/source-column vector and the raw-order partials by

\[
\begin{aligned}
L_{g,r}(v)
 &=\sum_{x<M_g,\ell<\delta_g}
 \operatorname{eq}(r_x,x)\operatorname{eq}(r_\ell,\ell)
 v_{\pi_{A,g}(x\delta_g+\ell)},\\
e_{g,p,i}
 &=L_{g,r}\!\left(s^{\mathrm{phys}}_{g,\pi_{B,g}(pB_g+i)}\right)\in S_E.
\end{aligned}
\tag{1}
\]

The one scalar endpoint is checked by

\[
y_g=\sum_{p,i,j}\operatorname{eq}(r_p,p)\operatorname{eq}(r_i,i)
 \operatorname{eq}(r_j,j)[Y^j]e_{p,i}.
\tag{2}
\]

For every c in S, `L(c s)=c L(s)` because the E weights commute with S, the
column permutation is common to all blocks, and the coefficient index retains
the same negacyclic wrap signs. The scalar coefficient functional (2) is not
S-linear; it is applied only after authenticating both entire partial planes.
Draw one independent c per **physical** original block. With `b=pB+i`:

\[
\begin{aligned}
z_g&=\sum_{t<P_gB_g}c_{g,t}s^{\mathrm{phys}}_{g,t},&
A_gz_g&=\sum_{t<P_gB_g}c_{g,t}t_{g,t},\\
L_{g,r}(z_g)&=\sum_{b<P_gB_g}c_{g,\pi_{B,g}(b)}e_{g,b}.&&
\end{aligned}
\tag{3}
\]

The last sum pairs raw partial b with its physical block's shell; using c_b
without this permutation is incorrect for a nonidentity block map. All source
geometry, map and original A/B/F role-chain fields must agree when distinct
handles explicitly reuse a source role. Equal matrix dimensions alone do not
authorize reuse.

`L` acts on the **recomposed response** `G_rsp zhat`; the original source digit axis has equality weights, while response digits retain powers of32. This is the reviewed [SOURCE_QUERIES](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md) full scalar-extension construction, generalized in P/B/M/delta rather than borrowing the inner receiver's counts.

## 3. Source-domain prefixes and ordinary-to-raw normalization

Every original source has one authenticated raw-domain endpoint before normalization. This family deliberately does not rely on an already accepted honest Booleanity transcript to skip the standalone source predicate. That is extra outer proof work, not extra source planes.

Bind the original handle/view, actual outer metadata and required-zero mask, then draw an atomic full-field `tau_src in E^mu_raw` and a subsequent scalar `eta_zero`. Put

`Z_tau = MLE_x[1_required_physical_zero(x) eq(tau_src,x)]`.

Virtual raw slots are fixed zeros by definition. Use the following exact per-source predicate; no private witness chooses the variant:

* **Main `BooleanWholeGrid`:** degree-three sumcheck of
  \[
  0=\sum_x[\operatorname{eq}(\tau_{src},x)\widetilde{Raw}(x)(\widetilde{Raw}(x)-1)
       +\eta_{zero}\widetilde Z_\tau(x)\widetilde{Raw}(x)].
  \tag{4}
  \]
  This covers every physical cell, including normal padded cycles. `{0,1}` is already inside `[-16,15]`; there is no redundant degree33 check or eta_plus draw. Required zeros are only those authorized in §2.1.
* **Dense `BalancedBase32`:** replace `Raw(Raw-1)` in (4) by `Q32(Raw)=prod_{a=-16}^{15}(Raw-a)`. Its individual degree is33. The same declared zero term remains; there is no additional Boolean predicate on arbitrary dense digits.

At the sumcheck endpoint send one raw value v; check the explicit public weight expression at v and retain that exact opening. The masks are MLEs of the masked Boolean equality tables, **not products of separately interpolated mask and equality polynomials**. These prefixes establish an actual source predicate via the descendant/source-consistency proof; they do not assert a recovered weak source was already short.

For each ordinary claim a on handle g, including original P-polynomial claims and every source-domain endpoint, compute its **exact coefficient-functional table** `a_ga(u)` on the raw view. For an ordinary claim this is the decoder adjoint

\[
 a_{ga}=Dec_g^*(\operatorname{eq}(r_a,\cdot)),\qquad
 v_a=\sum_u Raw_g(u)a_{ga}(u).
\tag{5}
\]

For a raw endpoint it is `eq(r_a,u)`. For a claim referring to one column, the other p slots have zero weight. Actual permutations, prefix scales and all live claim aliases are included in (5), rather than pretending every decoder is an identity. This is a finite explicit table formula for arbitrary generated guest geometry. It can be evaluated by an adjoint kernel or a correctness-reference scan; either computes the same table.

Bind the complete ordered list of original claims, descriptors, and domain endpoint, then draw one atomic independent vector `(alpha_a)_a in E^a_g`. Prove the degree-two sumcheck

\[
\sum_a\alpha_a v_a=\sum_u\widetilde{Raw}_g(u)\widetilde A_g(u),\qquad
 A_g(u)=\sum_a\alpha_a a_{ga}(u).
\tag{6}
\]

The terminal product is `v_g A_g(r_g)` with one retained raw opening `Raw_g(r_g)=v_g`. Never divide by a possibly zero `A_g(r_g)`; zero branches remain in soundness/extraction. Thus there is exactly one raw channel per handle, while all P*B original blocks and all original claim obligations remain.

For direct positional gadgets (5) has the reviewed [DIGIT_AXIS](AKITA_INSTANCE_ACCUMULATION_DIGIT_AXIS.md) simplification: `u_h=b^(2^h)/(1+b^(2^h))`, `J=prod_h(1+b^(2^h))`, and ordinary evaluation is `J Raw(r_j,u,r_x,r_i,bits(p))`. Use that evaluator only after checking every denominator and exact source permutation/scale. It is an optimization identity for the **selected universal adjoint protocol**, not an unresolved alternate carrier. No denominator condition is required merely to define the adjoint (5).

## 4. Fixed root commitment and opening paths

To make generation concrete rather than leaving compression/offloading open, this outer family selects:

* every A/source ring128; base32 ordinary source gadgets;
* each original B ring128, rank1, **one unsliced B map per original handle**;
* inner-image and partial base32 depth `delta_t=delta_e=26`;
* one shared D ring128, rank1;
* exactly two original F stages per B and two H stages for D: first `(degree16,rank1,width1024)`, second `(degree8,rank1,width256)`;
* compression digits in `{-1,0}`, 128 powers-of-two digits per input K coefficient, with the paper's registered plane/transpose order;
* no quotient witness, no response chunking, no physical-L2 route, no setup source/offload/product sumcheck.

The negative-binary recomposition interval `[-(2^128-1),0]` covers every K residue. Its first input has `128*128=16384` cells, packed into 1024 degree16 elements; its second input has `16*128=2048` cells, packed into256 degree8 elements. Every supplied valid noncanonical binary path is retained. The final payload has8 K coefficients. These concrete compression dimensions must pass their exact admission checks; no inherited table label establishes that fact automatically.

Precisely, at a compression stage whose preceding image has s K coefficients, digit u of coefficient v occupies flat position `v+s*u`, for `0<=u<128`; pack consecutive positions into the stage's native ring degree. Recomposition is `image[v]=sum_u 2^u digits[v+s*u] mod q`. Here s=128 then16, so no ring-completion cells are missing. This fixed permutation applies independently to every F and H chain and is included in the row/source descriptor. It is not the coefficient-major order used for ehat/that.

For handle g let `n_g` be A rank. Then

\[
m_{B,g}=26P_gB_g n_g,
\quad m_{D,g}=26P_gB_g(256/128)=52P_gB_g,
\quad m_D=52\sum_gP_gB_g.
\tag{7}
\]

Both E planes are physically present in D. Its intervals are concatenated in handle order, then p,i,plane,digit,coefficient order, precisely Eq107's block-then-digit decomposition. No extra D padding is introduced by the extension. `t_hat` uses p,i,A-row,digit,coefficient order. Public original commitments are the F-final payloads formed from `B_g t_hat_g`; the opening payload is the H-final payload formed from `D e_hat`. These are the **original** objects created under this installed profile. For an already installed object with a different B slicing/source map, this family is inapplicable; it does not reconstruct/recommit it to conform.

This fixed plan is allowed to be infeasible at some whole-guest geometry. The validator rejects that complete installation instead of silently switching to sliced/offloaded/EOR modes or reducing T16. Supporting another plan would require another named design, not a runtime fallback.

## 5. Shell, all-valid response depths and complete successor

Select the unfiltered d128 shell with24 magnitude-one and16 magnitude-two positions and independent signs:

\[
\omega=\|c\|_1=56,\quad c_{max}=2,\quad
|\mathcal C|={128\choose40}{40\choose24}2^{40}.
\]

The same full-ring unit acts on both extension planes. Assume/prove the native unit-difference criterion for this q/d/family (`q mod16=9`, coefficient differences at most4, sufficient condition `4<q^(1/4)/2`), rather than assuming the ring itself is a field. The sampler is `RankShell/v1` from [OUTER_TRANSCRIPT](AKITA_INSTANCE_ACCUMULATION_OUTER_TRANSCRIPT.md): lex40-support rank, lex24-of40 magnitude-one rank, then40 sign bits in support order. Finite bounded rank rejection, filter=true, nonce budget1; exhaustion rejects. There is no response-dependent retry, honest moment estimate or operator filter.

For a source whose **entire admitted physical alphabet** has absolute bound `beta_g` (1 for Main,16 for balanced dense digits), every allowed source/path and every shell tuple satisfies

\[
U_g=P_gB_g\omega\beta_g.
\]

Choose the least positive integer `f_g` with

\[
U_g\le15(32^{f_g}-1)/31.
\tag{8}
\]

This positive reach guarantees symmetric all-valid response encodability in `[-16,15]` base32 digits. The actual verifier-enforced integer envelope and sibling difference envelope are

\[
 Z_g=16(32^{f_g}-1)/31,\qquad \Delta_g=32^{f_g}-1.
\tag{9}
\]

Honest reach (8) is not substituted for (9) in collision pricing. Range-check every carried digit, not only responses. Feasibility requires integer uniqueness/nontrivial MSIS bounds as in §8; depths and bounds use checked integer arithmetic, not modular inequalities.

After H is bound draw all root shell coordinates on the fixed common prefix; there is **no scalar-claim batching challenge at the root**. We keep every handle's scalar row (2) separately. Form the entire Eq109 vector, in order

`for each g: [zhat_g | ehat_g | that_g]; then [all F1 inputs | H1 input]; then [all F2 inputs | H2 input]`.

Response address is `((x delta_g+ell) f_g+v)128+j`. Partial address is `(((p B+i)*2+plane)*26+v)128+j`. Inner-image address is `(((p B+i)*n_g+a)*26+v)128+j`. Compression transpose maps are the fixed paper §4.5 maps; they are not rebuilt from a preferred path. All quotient intervals are empty by the exact coefficient-row design. In this selected ring/digit plan every main segment is already128-aligned; each compression stage is16/8-aligned and its total is128-aligned. The finite descriptor still represents/checks alignment rather than dropping those requirements by convention.

The exact live cell count is

\[
L=\sum_g[128M_g\delta_g f_g+256\cdot26P_gB_g+128\cdot26P_gB_g n_g]
 +(G+1)18432.
\tag{10}
\]

Set `N=nextpow2(max(128,L))`; append required physical zeros `N-L`. The successor source is the **identity K coefficient table** W of this entire N-cell vector, common digit bound16, with its exact negative-binary compression spans and zero suffix. It is created at this point as `TerminalInnerState`, not an `OuterPayload` later relabeled. The chosen terminal A images in §7 are bound before *any* strong randomness. There is no terminal B/F source and no omitted predecessor F/H input.

## 6. Complete strong checks on W

Use the reviewed [LINEAR_RELATION](AKITA_INSTANCE_ACCUMULATION_LINEAR_RELATION.md) construction with the actual generated dimensions. It checks all coefficients of:

1. each A equation in (3), using `G_rsp zhat` and `G_open that`;
2. both S planes of each L equation in (3), using full recomposed `ehat`;
3. each original B image against recomposition of its supplied F1 digits;
4. the one shared D image against recomposition of H1 digits;
5. every F1/F2/H1/H2 transition, including final equality to the already-bound public payload;
6. each handle's separate E scalar equation (2).

For a native ring degree d the coefficient action is

\[
(av)_t=\sum_{j=0}^{d-1}\sigma(t,j)a_{(t-j)\bmod d}v_j,
\quad\sigma(t,j)=1\ (j\le t),\ -1\ (j>t).
\tag{11}
\]

Apply each declared gadget weight, native permutation and E coordinate to obtain the explicit sparse/streamable `M W=b` over K. Reduction modulo each **actual native ring** is inside (11); products of off-ring evaluations are never substituted. The exact scalar row count for this selected plan is

\[
R=128\sum_g(n_g+3)+128+24(G+1)+2G.
\tag{12}
\]

Here n_g A rows +2 carrier planes +1 B row belong to each g; D contributes128; each two-stage F/H chain contributes16+8; the unbatched scalar equations contribute2G. This is derived from outer G/n_g, not inner ring-row totals. Pad rows with zeros to `Rbar=nextpow2(R)`, set `kappa=log2 Rbar`, and bind the row manifest.

**Fixed order.** After binding W's A images/source plan draw `tau0 in E^mu` (`mu=log2 N`), then `rho in E^kappa`. There is no ring-switch alpha draw in the coefficient-row mode. Define

\[
\lambda_\rho(x)=\sum_{h<R}eq(\rho,h)M_{h,x},\qquad
b_\rho=\sum_{h<R}eq(\rho,h)b_h.
\]

Run the full native base32 product range pipeline on **all N physical cells**. Its Boolean virtual table is `s(x)=W(x)(W(x)+1)`, with

`Qsq(s)=prod_{k=0}^{15}(s-k(k+1))`.

The selected tree is exactly `(4,4)` with node counts `(1,4,16)`: first a degree5 sumcheck of the product of four quarter-node MLEs weighted by `eq(tau0,x)`, target0. Send all four quarter-node values at its endpoint. Draw one atomic vector `(xi1,xi2,xi3)` and use normalized affine weights `(1,xi1,xi2,xi3)`. A second degree5 sumcheck checks their weighted four-child products at that point. Each group of four leaves is the corresponding four consecutive factors `s-k(k+1)`. At the final endpoint all sixteen leaf values must be affine functions of the **same sent scalar** `svirt`; verify the resulting endpoint expression exactly. This leaves `svirt=MLE(s)(rvirt)`, not the false off-cube identity `svirt=W(rvirt)(W(rvirt)+1)`. Use full coefficient messages with declared degree5; no equality-factor codec is assumed for this new family.

After that virtual frontier, draw the atomic independent E triple `(gamma,eta_bin,eta_zero)`. Let

`B=MLE[1_Ibin(x) eq(rvirt,x)]`, `Z=MLE[1_Izero(x) eq(tau0,x)]`.

Prove the degree3 fused sumcheck

\[
b_\rho+\gamma s_{virt}=\sum_x\left[
 \widetilde W(x)\widetilde\lambda_\rho(x)
 +\gamma eq(r_{virt},x)\widetilde W(x)(\widetilde W(x)+1)
 +\eta_{bin}\widetilde B(x)\widetilde W(x)(\widetilde W(x)+1)
 +\eta_{zero}\widetilde Z(x)\widetilde W(x)\right].
\tag{13}
\]

At its last point r, receive `v=MLE(W)(r)` and substitute the **same** v in all terms. The verifier computes `lambda_rho(r)`, B(r), Z(r), and the equality weight directly. Never divide by a weight. All original range product messages, interstage batch, binary-support and zero-support soundness charges remain. No setup scalar is delegated: the actual matrix-dependent weight scan is mandatory. No physical-L2 claim or ignored norm term is present; (9) follows from the full digit alphabet.

The remaining leaf is exactly the identity full-grid opening of W with factor1, actual point r and value v. It is authenticated by the genuine terminal below. In particular, a paper full-grid opening with an unaccounted padding factor is not accepted as this leaf.

## 7. Finite complete suffix and direct terminal

For a candidate terminal choose a power-of-two `M_T` dividing `N/128`, put `B_T=N/(128 M_T)`, source depth1, source bound16, and a registered rank `n_T`. Candidate selection/admission is deterministic in §9. At W creation publish

\[
t_i=A_Tw_i,\quad 0\le i<B_T,\quad w_i\in S^{M_T}.
\]

The terminal source layout is the full identity W table: low-to-high `(j,x,i)`. It uses **no tensor packing**, no change of digit magnitudes, and no hidden zero/half-ring projection. Its source map/encoding are known when t is first bound, not selected after the strong check.

After the strong leaf is known, reabsorb the same A-image tuple, setup role, mode/source descriptor, point r and value v. Receive clear `e_i=L_r(w_i)` in both S planes. Check the exact scalar equation

\[
v=\sum_{i,j}eq(r_i,i)eq(r_j,j)[Y^j]e_i.
\tag{14}
\]

Bind all partial bytes before drawing the terminal's B_T independent shell coordinates. Receive one signed integer coefficient response `z=sum_i c_i w_i` in `S^{M_T}`. Enforce

\[
|z_{x,j}|\le U_T=16\omega B_T=896B_T,
\quad A_Tz=\sum_i c_i t_i,
\quad L_r(z)=\sum_i c_i e_i
\tag{15}
\]

coefficientwise, including both E planes. `L_r(z)=sum_x eq(r_x,x) z_x` here. The codec is fixed-width signed two's complement, little-endian, width `b_T=ceil((1+ceil(log2(U_T+1)))/8)` bytes per coefficient, with exactly that many bytes and a range check before reducing into K. Require `U_T<q/2`; every integer in the checked range has a unique K representative. There are no alternate-width or negative-zero encodings.

There is no terminal D/H, B/F, range check, relation sumcheck, setup pending leaf or tensor sumcheck. The root predecessor's F/H digits and its strong checks have already been paid in W; terminal simplicity cannot remove them. The complete ordinary suffix is now finished. A verifier must consume all proof bytes and reject residual obligations/trailing bytes.

**Terminal extraction.** A coordinate fork uses unit `delta_i=c_i-c'_i`, `r_i=z-z'`, yielding `w_i=delta_i^{-1}r_i`, `A_Tw_i=t_i`, and both partial planes. It has product certificate `||delta_i||_1<=112`, `||r_i||_inf<=2U_T`. It does not assert the weakly extracted w_i is short. Same-A-source comparison across the predecessor's strong trees/probes, followed by those range/zero/binary checks, supplies the actual bounded W source. Exact scalar equation (14) belongs to the accepting child and is not derived by incorrectly subtracting scalar functionals through c.

This is finite closure without an assumed contraction recurrence: (10) is finite for finite original geometry; the successor is fixed once; (14)–(15) are direct, nonrecursive verification. The terminal may be very large. No claim that three inner folds, inner terminal widths, or ordinary offloaded families provide this suffix is made.

## 8. Source, norm, reference and setup contracts

A source identity is `(registered role/view, P,B,M,delta, decoder/raw layout, original public payload)`; for terminal it is `(TerminalInnerState, identity layout, same A-image tuple)`. It excludes query points, claimed values, partials and opening-local D/H state. Compare original B/F paths first and retain all supplied noncanonical digits; an inequality gives the appropriate B/F collision. If they agree, compare original source blocks by their unit/product certificates. Across exact fold siblings also compare pre-fold D/H paths. Across different strong-source probes do **not** require D/H equality merely because the original handle agrees.

For any two occurrences with `delta s=r`, `delta' s'=r'`, `||delta||_1<=k`, `||r||_inf<=b` and analogous k',b', use

\[
z_A=\delta' r-\delta r',\quad A z_A=0,\quad
\|z_A\|_\infty\le k'b+kb'.
\tag{16}
\]

Units ensure nonzero collision when sources differ. Do not bound inverse units. Root weak occurrences have `(k,b)=(112,Delta_g)`; strong bounded sources have `(1,beta_g)`. Root weak–weak radius is `224 Delta_g`, mixed radius `Delta_g+112 beta_g`, strong–strong `2 beta_g`. Terminal weak occurrences use `(112,2U_T)`, so terminal weak–weak radius is `448U_T`, mixed weak–strong `2U_T+112*16`, strong–strong32. Take maxima over **all occurrences sharing a registered view**, including changed claims, legal public geometry variants and source-first probes. The finite installation must not validate only diagonal honest occurrences.

B and D digit-difference radius is31; F/H negative-binary radius is1. Equality of each final payload determines earlier path digits backwards unless one of those collisions occurs. Required zeros do not lower an alphabet diameter unless both compared occurrence contracts enforce them.

There is **no PCS setup-offload reference source** in this chosen family, so no Eq170 setup leaf or setup-reference comparison is silently pending. All outer matrices are directly evaluated. This does not delete the separate trusted **application** Static reference: for the original C_static compare the extracted source/decoded advice against its installed source reference using the same B/F path rule and (16) with `(delta',r')=(1,s_static)`. The radius is `b+k||s_static||_inf` in its actual digit contract, not a guessed bound on 64-bit decoded words. Program/preprocessing commitments have their analogous installed-reference classes if the actual frontend exposes them. Archive has no trusted-content reference; its bytes remain bound to its original untrusted handle and checked by the guest. The inner matrix bytes inside Static are application data, not automatically the outer setup registry.

Require certified alphabets to embed distinctly in K, integer response bounds to be finite, and each selected collision radius to be strictly below q/2. Every A/B/D/F/H tuple must have an explicit security-policy certificate at its actual ring degree/rank/width and rounded-up radius, or admission fails. Attack-model SIS tables only justify that certificate's stated model; the security theorem still carries the corresponding runtime-sensitive MSIS advantages. No numerical advantage or 128-bit claim follows from this rule.

Provision each unequal role as a separate full independent matrix namespace, with repeated occurrences of the same role reusing the same matrix. There are no undeclared shared prefixes. In the ideal setup this is one flat uniform K vector partitioned into these matrices. A concrete finite 512-bit modular provisioning law, if selected, has total variation at most `C_setup q 2^-512`; charge it for the actual emitted coefficient count, not inner33. Any concrete XOF/pseudorandom expansion adds its declared setup assumption. Include J and the resulting I/registry identity in all proof roots.

## 9. Deterministic finite generation and independent validation

The input J contains a bounded guest specification covering **all** original T<=16 inputs, accepted parses, rejected parses and bounded sampler outcomes; public/Archive/Static capacities; maximum physical trace/RAM/code layouts and exact decoder algorithms; resource/byte/integer limits; a finite permitted SIS certificate table with digest; and transcript caps. These are inputs to installation, not proof-controlled fields. Concrete bytecode and resulting geometry may be generated later, but the following algorithm is fixed now.

1. Compile the actual fixed guest and derive its ordinary ProverConfig/one-hot setup geometry and original source ownership using the actual producer maps. Either use one consistently padded envelope or enumerate every publicly selectable shape in a finite authenticated family. Do not infer RAM/trace/program shape from a trace exponent alone. Reject any geometry not covered by its full-domain/decoder proof. No candidate generator is allowed to truncate archive sections, T, address space or valid weak source paths.
2. For each group enumerate all dyadic block splits of its fixed decoded physical table length at ring128; source delta/base/semantic decoder are fixed by its producer contract, not by proof choices. Select the all-valid response depth by (8). Enumerate A ranks from the **finite supplied certificate domain** and retain only candidates with their complete mixed radii. Use the fixed B/F and D/H plan of §4; reject if its exact widths or any certificate fail. The enumeration domain is finite even if large. A root profile cannot change rank after the original sources are committed.
3. For each retained root compute the exact Eq109 offsets, masks, L/N, row count/ordering and every transcript stage arity using §§3–6. Enumerate every dyadic `M_T` dividing N/128 and finite supplied terminal A-rank candidates; calculate B_T,U_T,codec width and mixed radii, and retain only candidates meeting §§7–8. No search depends on random matrix entries or transcript answers.
4. Fully expand the typed matrix role list, source/reference occurrence pairs, message lengths, stages and finite oracle sampler/query bounds. Reject all integer overflows, out-of-resource shapes, unknown families, unbound prefix/scale maps and mismatched source encodings. Never declare a profile feasible from a Rust type alias or from unchanged old EOR rows.
5. Select the lexicographic minimum of `(total expanded K setup coefficients, exact root-plus-terminal fixed payload bytes, N, canonical complete descriptor bytes)` among feasible complete candidates. Variable frontend/prefix/sumcheck bytes are separately emitted exactly; this tie-break is deterministic, not a performance model. Identical identities deduplicate; unequal source roles do not share keys. If no complete candidate exists, return UnsupportedWholeGuestProfile. There is no runtime planner fallback.
6. An independent validator recomputes all formulas, raw/physical injections, source bounds, decoder-adjoint specification, commitments/path dimensions, coefficient-row manifest, masks, terminal factor1 map, exact finite sampler slots and all source/reference maxima. It checks the emitted candidate against the selected minimum or a complete finite-search certificate. Preprocessing freezes its exact grouped lookup rows/producer contracts and catalogue digest. Proving/verifying accepts only those rows. Current registry limits (including row-count/group-category limits) must be explicitly met or deliberately changed with the generated finite-family validator; silently bypassing them is not this design.
7. Before accepting an installation, certify that the **entire** requested guest language fits at least one installed shape and that dispatch has no supported-language gap. A failure here rejects the whole outer installation. It does not turn T16 into a smaller accepted T or make a selected archive parse unsupported at proof time. This is a feasibility boundary, not a proof that the as-yet-uncompiled guest already fits current production ranges.

Every root candidate has a complete terminal checked in step3; no fixed-point/suffix search can diverge. Every loop ranges over an explicitly finite set or finite source table. Thus concrete rows are future artifacts under a specified terminating rule, not a remaining “implement some compatible carrier” decision. Whether the eventual whole-guest profile passes the numerical certificates and resource limits is honestly unestablished.

## 10. Exact costs and extraction obligations

With distinct G original root roles, the selected matrix list is: G A, G B,2G F, one D,two H,one terminal A: **4G+4 roles**, before any additional independently required ordinary frontend commitment roles are included in G. This is not33. Expanded K coefficients are

\[
C_{setup}=\sum_g[128n_gM_g\delta_g+128\cdot26P_gB_g n_g]
 +128\cdot52\sum_gP_gB_g
 +(G+1)(16\cdot1024+8\cdot256)+128n_TM_T.
\tag{17}
\]

At16 bytes per K coefficient literal materialization uses `16 C_setup` bytes before metadata/caches. Streaming changes peak storage, not generation or verifier evaluation work. The exact root Eq109 allocation is (10), raw-domain working lengths are Nraw_g, and the successor commitment/prover range work touches N cells. A correctness-reference verifier may compute `lambda_rho(r)` by streaming all entries of the implicit R-by-N map; optimized transposes may reduce this, but are required to return the same value and still pay the matrix scan. No hidden setup offload or free dense public weight evaluation is claimed. Likewise the universal adjoint normalizer may cost a full decoder/source scan; a digit-axis factorization saves work only where proved equal.

Terminal fixed payload components are

* `16*128*n_T*B_T` bytes for the predecessor-bound A images (sent once, later reabsorption is not retransmission);
* `32*128*B_T` bytes for clear full E partials;
* `b_T*128*M_T` bytes for the signed response.

Add the root H-final payload128 bytes, original F-final payload128 bytes per commitment where transmitted, original frontend messages, all source prefixes, normalization and strong-check messages/values, public fields, statement/registry framing, and sampler metadata. With full coefficient round encodings a degree-d sumcheck sends d+1 E coefficients per round, plus its declared endpoint records; none of these terms is an assumed inner proof-size constant. Main prefix is degree3, dense prefix degree33, normalization degree2, two range stages degree5, and the fused relation degree3. This direct family can be substantially worse than an offloaded ordinary schedule; no parity waiver or performance conclusion is implied.

**Conditional extraction argument.** The terminal's actual direct checks give the base. For each fixed root fold child, a descendant extractor authenticates the successor's full-grid source or emits a listed collision. The native source-first strong-node argument checks the exact coefficient rows, full range tree, binary and zero predicates on this same source; alternatively the full nested supplied-tree argument yields the same branch relation. Row interpolation is over `rho`'s kappa coordinates and has its actual factor/degree ledger; no ring-alpha node remains. Coordinate subtraction through unit differences then recovers the original weak sources and both partial planes. Same-original-source comparison transports the raw normalization and source-domain endpoints back to those very handles, preserving all ordinary claims. Trusted-reference comparisons and the fixed-source frontend certificate yield the original outer execution statement.

The ideal-H_o theorem therefore needs, explicitly: the unit-difference premise; exact source/decoder/full-grid and masked-weight identities; the finite local source-domain/normalization/range/row/fused certificates; original-handle mixed collision inventory and trusted-reference comparisons; complete bounded raw-tape samplers/REPLAY and compatible stage keys from OUTER_TRANSCRIPT; bounded native extraction with charged cutoff failures; and the actual fixed-source Jolt frontend reconstruction/extension certificate of [SOURCE_FIRST_EXTRACTION](AKITA_INSTANCE_ACCUMULATION_SOURCE_FIRST_EXTRACTION.md) and [NATIVE_SOURCE_FIRST](AKITA_INSTANCE_ACCUMULATION_NATIVE_SOURCE_FIRST.md). These are mathematical/source-certification obligations, not all asserted discharged by drawing this schedule. No assumption of same-output frontend extraction or perfect source uniqueness is introduced.

Useful explicit ledger components are root flat factor `1+sum_g P_gB_g`, terminal flat factor `1+B_T`, row multilinear coordinate degree1, sumcheck degrees given above, normalized four-frontier scalar separation, and three independent fused separation coordinates. Source-prefix/zero-mask and all frontend contributions remain additional. The finite sampler exhaustion, setup bias, oracle query bounds, rejection-probe resource costs, weak/strong mixed extraction costs and every MSIS advantage are accounted at the whole ordinary outer proof, not per fold or divided by the role count. No security-number claim is made.

## 11. Exact source cutover and evidence boundary

Inspected source demonstrates why a separate method is necessary:

* [params/precommitted.rs](../../crates/akita-params/src/layout/params/precommitted.rs), lines13–100: current EvaluationTrace width is d_A; packing sizes `degreeE*s` and D widths through a different geometry. Extend the canonical sizing authority with width256; do not disguise this as either existing method.
* [schedule/profiles.rs](../../crates/akita-params/src/schedule/profiles.rs), lines10–79: source encoding is commitment identity and current nonroot EvaluationTrace producers select tensor projection. This family retains coefficient-table identity all the way through terminal.
* [recursive/commit.rs](../../crates/akita-cpu-backend/src/opaque/recursive/commit.rs), lines177–246: current terminal arm tensor-packs every nontrivial extension regardless of explicit encoding. Replace that shortcut with checked source-encoding dispatch for the selected method; never widen response norms after the fact.
* [extension_claim.rs](../../crates/akita-verifier/src/stages/opening_claims/extension_claim.rs), lines208–335: current extension suffix/terminal calls EOR and `ring_subfield_packed_extension_opening_point`. The new method dispatches directly to (1)–(2)/(14)–(15), retaining full planes, not to this unchanged route.
* [fold/terminal.rs](../../crates/akita-verifier/src/fold/terminal.rs), lines26–173: retain the singleton/no-pending-setup/prechallenge-image-and-partial checks, but use the new coefficient-plane checks and signed bounded codec rather than trace/EOR behavior selected solely by extension degree.
* [planner candidate/opening.rs](../../crates/akita-planner/src/schedule_params/candidate/opening.rs), lines3–134: existing candidates are trace/packing and reject the full-s=128 quadratic geometry. Add the genuinely new candidate and its exact source/query/width/certificate validation. [suffix_dp/terminal.rs](../../crates/akita-planner/src/schedule_params/suffix_dp/terminal.rs), lines54–163, shows the ordinary singleton direct-terminal branch and its existing feasibility/cost checks; this note specifies its new carrier/norm/encoding instead of claiming the existing estimate prices it.
* [grouped_setup_params](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/preprocessing.rs), lines90–143, derives actual Main/advice/mandatory dense groups. [schedules/mod.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/schedules/mod.rs), lines33–75, has finite production ranges and offloads from trace exponent21. Neither artifact certifies this new no-offload family or guarantees the guest fits it.

Normative algebra is reused from [SECURITY_PROFILE](AKITA_INSTANCE_ACCUMULATION_SECURITY_PROFILE.md), [LINEAR_RELATION](AKITA_INSTANCE_ACCUMULATION_LINEAR_RELATION.md), [COMPACT_PROFILE](AKITA_INSTANCE_ACCUMULATION_COMPACT_PROFILE.md), [MATRIX_REGISTRY](AKITA_INSTANCE_ACCUMULATION_MATRIX_REGISTRY.md), SOURCE_QUERIES and DIGIT_AXIS, but every outer formula/count here is regenerated from the actual geometry. [MAIN_SOURCE_REUSE](AKITA_INSTANCE_ACCUMULATION_MAIN_SOURCE_REUSE.md) supplies the physical-hole warning; its inner active-length masks are not imported into the ordinary guest.

Primary paper evidence: [Akita indexed text](SOURCE_PROVENANCE.md#akita), §§5.3–5.5,6.1,7.4–8.2 and10; [ePrint locator](https://eprint.iacr.org/2026/1983). Live qmd retrieval of `lattice-papers/akita.md` lines2320–2378 supplied Eq107–113, all-native digit order, complete Eq109 and honest-versus-certified norm distinction. Local primary reads covered lines2259–2323 (shared D, response/main order) and2423–2489 (base32 product tree and virtual-table semantics). This is evidence for the algebraic inputs, not evidence that the paper already implements this full-ring outer family.

Only this design note was created. No source edits, builds, tests, formatters, benchmarks, proof runs or Git operations were performed. Remaining artifacts are the actual guest/geometry and emitted row/codec/matrix/transcript inventories, their independent finite validation and source-correctness certificates, then the explicitly authorized implementation and full correctness/performance gates. This note fixes the algorithmic carrier/suffix choice; it does not pre-admit those future artifacts.
