# One-shot graph quotient held by the original outer lattice PCS

## 1. Result and invariant

**The quotient identity, weak-source comparison and evaluation compiler are valid under the stated source contracts. Adaptive zero finding now uses the independently reviewed [hidden-curve transport theorem](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md), not the withdrawn direct CCGOS adaptation.** Its weaker loss has Q outside the square root. The [ARO audit](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md) records why the unrestricted emulator and implicit-curve-persistence argument could not be used. No new knowledge assumption, concrete oracle or unchanged outer-API/profile admission follows.

The fixed anchor is the existing native **pre-PCS segment-instance** archive: K-valued sources, E challenges, the 33 inner roles, at most 16 updates, and the one final native decider. There are no old oracle accumulators here. This note changes only authentication of the final archive's oracle ledger and the additional sources/claims of the single **outer** ordinary proof. It neither folds completed segment proofs nor assumes A/B/F commitment homomorphism. No Merkle PCS, nested PCS verifier, or H_o call appears inside the guest.

The product model is the one in [Mixed oracle](AKITA_INSTANCE_ACCUMULATION_MIXED_ORACLE.md) and [Product extraction](AKITA_INSTANCE_ACCUMULATION_PRODUCT_EXTRACTION.md): fixed inner `Theta_i=(ro_i,wo_i,Vhat_i)` and independent outer ideal streams H_o. This is not CH-I/CH-O and is not a concrete-hash instantiation theorem.

**Source basis.** [CCGOS accepted manuscript](SOURCE_PROVENANCE.md#supplemental-primary-sources), §7, Lemma 7.1 and Claims 7.3–7.4, printed pp.38–41; §8, Construction 8.2 and §§8.2–8.4, pp.42–46. The corresponding [local text](SOURCE_PROVENANCE.md#supplemental-primary-sources) is at lines 2143–2394 and 2422–2693. Native source comparisons and the new common-functional carrier are derived in [Source queries](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md), §§2, 4, 6 and 8. The source's CM theorem is **not** invoked directly on an Akita weak extracted state.

## 2. Precisely bounded graph instance

Fix before oracle sampling the field F, ARO arities m and lambda_A, witness circuit B with output length w, individual degree d>=2, and all source/encoding/setup profiles. The repaired emulation route requires **input-local** B; its output on x may depend on ro(x), not other oracle inputs. The actual ARO definition alone does not require this restriction, and general B is not admitted by the repaired reduction. Put

`a=m+lambda_A+w`, `M_F=|F|`.

The canonical guest reconstructs the entire ordered native ledger from its archive execution. Each entry is a raw fixed-input oracle equation `ro_i(x_j)=y_j`, with x_j an m-bit string and y_j a lambda_A-bit string. All rejection attempts, output-block queries, framing and namespaces are part of this reconstruction. The additional witness z_j is a w-bit string supplied by the host. The graph point is the **coordinatewise Boolean embedding**

`u_j=bits_F(x_j)||bits_F(y_j)||bits_F(z_j) in F^a`.

On Boolean points the ARO definition gives

`Vhat_i(u_j)=1 iff y_j=ro_i(x_j) and z_j=wo_i(x_j)`.

Thus a false native answer gives a false graph target for every z_j. Honest z_j is obtained by querying wo_i. Merely parsing a claimed y_j or z_j as field values is not this graph equation.

The clean baseline keeps every ledger occurrence, in order, with n>=1. An optional deduplicated list needs an oracle-free exact coverage map: every occurrence maps to a tuple with identical x,y,z, indices are in range, and interpolation order and n are canonical. Dropping a conflicting repeated answer or deduplicating only by input without checking equality is invalid. An empty ledger is a separate trivial case, not a reason to insert an unverified fake graph point into this construction.

Choose public distinct `D={b_0,...,b_(n-1)} subset F`, derived deterministically from the bound profile and n. Require n<M_F and, for the interpolation-based host algorithm below, `D_f+1<=M_F`, where

`ell=n-1`, `D_f=a*d*ell`, `Z_D(X)=product_j(X-b_j)`.

Let `g in (F[X]_{<=ell})^a` be the unique coordinatewise interpolation of `g(b_j)=u_j`. Then

`deg(Vhat_i o g)<=D_f`.

Use D_f, not d*ell, for the degree bound: Vhat has individual degree d in a coordinates. CCGOS §7 itself displays this a factor in its final bounds.

### Lemma 1: quotient identity, including n=1

For a true graph list, define `f=Vhat_i o g`. Each b_j is a root of f-1. Since the b_j are distinct, the factor theorem gives the **unique** quotient

`f=1+Z_D*t`.

For n>=2, `deg(t)<=L_t:=D_f-n`; zero is allowed, with degree -infinity. In the present a,d domain, L_t>=0 for n>=2. If parameters outside that domain make D_f<n, divisibility forces f=1 and t=0 instead of a negative-length array.

For n=1, g is constant, f=1 and t=0. There are no polynomial coefficients of negative degree. Conversely, for *any* polynomial t, `f_t:=1+Z_D*t` has f_t(b_j)=1. Therefore if even one graph target is false,

`h(X):=Vhat_i(g(X))-[1+Z_D(X)t(X)]`

is nonzero: evaluate it at the offending b_j. Under the declared quotient cap, `deg(h)<=D_f`. This last argument uses neither zero knowledge nor an honest source decomposition. ∎

## 3. What is committed, and what is not inside the guest

Use two **original** outer lattice source commitments:

- `C_g`: a source whose extracted semantic projection fixes every byte/bit determining n, D and g, including all native x,y answers and **all z witnesses**. It may bind extra prechallenge archive bytes.
- `C_t`: a separately provisioned original source whose declared K-linear decoding fixes the quotient coefficient vector in §7. It is not a commitment manufactured by adding/scaling C_g or another commitment.

The binding input is a fixed-length injective encoding

`cm=Enc_m(tag, profile/setup identities, C_g, C_t, nonce)`.

The framing contains all shape/decoder/domain identities needed to interpret the two handles. An identity may be a fixed approved index with an injective registry meaning; an unproved hash of arbitrary setup/profile bytes is not an injective replacement. The actual encoded tuple **must fit m bits**, including nonce and framing. There is no compression theorem that lets one silently hash a too-large cm into the native ARO domain. In particular, a future small-input transcript-compression theorem must still accommodate this tuple, or supply a separately proved authentication reduction for its encoding.

### Reusing original untrusted advice as C_g

Reusing the existing advice handle is legitimate only if the complete outer extraction/source projection proves that its same weak source determines the entire relevant logical byte string and that the guest's reads use exactly those bytes. Required conditions include canonical framing, range/read semantics, exact EOF/capacity semantics and inclusion of z. A commitment just to original inputs x, or a relation permitting different read values for the same advice source, is insufficient. Padding that the guest ignores need not determine g, but all bytes that the guest actually uses must.

This is a real current-source distinction: [Receipt code map §5](AKITA_INSTANCE_ACCUMULATION_RECEIPT_CODE_MAP.md) records the stage4-to-stage8 advice-opening path and the SDK's discarded decoding remainder. Authentication of a K-valued advice word table is not by itself canonical-byte range/read adequacy. An honest u64 packing algorithm cannot prove that an arbitrary extracted K word is a canonical u64, nor that ignored logical capacity is zero. Prove exactly the consumed-byte projection (and any padding predicate actually needed); do not silently strengthen all ignored capacity into a canonical message.

There is also a completeness dependency constraint. A reused C_g source cannot obligatorily contain `raw_beta`, `wo_i(cm)`, `g(beta)` or `t_beta`, since those depend on cm and cm contains C_g. That would ask the honest prover to find a random-oracle/commitment fixed point. Put such postcommitment values in the bound public accumulator or a distinct postcommitment source, not in C_g's committed prechallenge data. The archive and its native-query z witnesses are prechallenge data. “Extra bytes may be included” does not license this circularity.

The guest sees the archive, its query/witness data, the public handle identities and small claimed `t_beta`; it holds neither the dense polynomial f nor dense t. It derives g(beta) by interpolation and Z_D(beta) directly. The *whole outer proof*, outside the guest, proves the native linear opening of the original C_t. The public accumulator, C_t, beta, t_beta and query descriptor must be bound in that same outer statement/transcript. A detached proof about an unrelated C_t is not adequate.

## 4. Exact beta sampler and true graph equation

A field compatible with K-linear source claims is

`K=F_q`, `q=2^128-2^32+22537`,
`E=K[U]/(U^2-5)`, `F=E[V]/(V^2-U)`.

Here primality/fieldhood of the native K is the existing native field premise. One has q=1 mod4 and q=2 mod5. Quadratic reciprocity gives `(5/q)=(q/5)=-1`, so E is a quadratic field. `Norm_(E/K)(U)=-5` is a K nonsquare because -1 is a K square. For finite fields of odd characteristic, the quadratic character of u in E equals that of its norm in K. Hence U is an E nonsquare and F is a field of degree four over K, with basis `(1,U,V,UV)` and cardinality `M_F=q^4<2^512`.

Select **lambda_A=1024** for this sampler; that is an ARO parameter choice, not a claim about the output length of an existing concrete hash. Let `R=2^1024`, `kappa=floor(R/M_F)`, `cut=kappa*M_F`. Decode the raw 1024 bits as a canonical unsigned integer r. Define the partial map

`S(r)=bottom` if `r>=cut`;
otherwise write `r mod M_F=c_0+q*c_1+q^2*c_2+q^3*c_3` with each `0<=c_h<q`, and set `S(r)=c_0+c_1 U+c_2 V+c_3 UV`.

For the selected variant also reject S(r) in D. This map is exactly uniform on F\D conditioned on success, and has unconditional point mass

`Pr[S(r)=b and accepted]=kappa/R<=1/M_F`

for every b outside D. Its rejection probability is

`p_abort=(R-cut)/R + (cut/R)*(n/M_F) < 2^-512+n/M_F`.

If this field choice is combined with a transcript-compression construction, the shared ARO output width in that construction must also be lambda_A=1024. A separate shorter chaining-state length may be declared, with its own truncation/collision analysis. A numerical example using a 512-bit-output ARO is a different parameter choice and cannot supply these 1024 graph bits for free.

The exact four K coordinates, tower arithmetic and integer cutoff must be checked by the guest, not approximated by reducing a biased candidate modulo q^4. Native inner challenge samplers and outer E challenge samplers are unchanged; F is the ARO/evaluation field, not a new native challenge ring.

**Crucial graph distinction:** the first accumulator point is

`(bits_F(cm), bits_F(raw_beta), bits_F(z_cm))`, target 1,

where `z_cm=wo_i(cm)`. It contains all **1024 raw output bits**, not four K coordinates and not the accepted field beta substituted for those bits. Only the guest's deterministic S maps the graph-authenticated raw output to beta. The external graph equation thereby enforces `raw_beta=ro_i(cm)` and `z_cm=wo_i(cm)`.

Excluding D is not needed for non-ZK completeness or soundness: at beta in D the quotient expression is well-defined and the same root bound applies. CCGOS uses exclusion in its ZK argument. This note keeps the contract's exclusion and charges its completeness error; a non-ZK variant allowing D may remove precisely the n/M_F term. It must change the declared sampler consistently, not claim the old distribution.

## 5. Explicit algorithms and completeness

### Host prover

1. Complete the unchanged native archive. Reconstruct its exact ledger, obtain each z_j from wo_i and form u_j. The ordinary native archive uses ro_i only; this host authentication preparation may use all of Theta_i.
2. Compute g. For n>=2 compute `f=Vhat_i o g` by interpolation from D_f+1 distinct evaluation points; this takes at most D_f+1 Vhat_i evaluations. Divide f-1 exactly by Z_D. On a valid archive this produces degree-<=L_t quotient t by Lemma 1. For n=1 set t=0 without this interpolation.
3. Encode the complete graph-defining data and the coefficient planes of t in admitted original sources. Compute C_g and C_t with their ordinary original commitment algorithms. No homomorphic operation is used.
4. Select a fresh nonce after those sources are fixed, form cm, obtain raw_beta=ro_i(cm) and apply S. On rejection, either abort (one-attempt profile) or use the explicitly bounded fresh-nonce retry policy. Do not alter the quotient cap, data projection or source handles during an honest retry.
5. Obtain z_cm, compute `v=g(beta)`, `t_beta=t(beta)` and `gamma=1+Z_D(beta)*t_beta`. Output the bound accumulator

   `acc=(((cm,raw_beta,z_cm),1),(v,gamma))`

   with the bit/field convention of §4, and the ordinary outer proof described below.

### Oracle-free guest and outer verifier

The guest checks the expected native statement/profile/setup, exact native archive and final decider with supplied answers, and complete ordered ledger reconstruction/coverage. It checks Boolean/canonical graph data and cm's exact handle/profile/nonce encoding; it checks S(raw_beta)=beta is accepted; it computes the interpolation v and Z_D(beta), and checks `gamma=1+Z_D(beta)*t_beta`. It binds success1 and the exact public acc and handle/claim identities. It makes no Theta_i or H_o call and runs no PCS verifier.

The ordinary outer proof additionally proves four K-coordinate linear claims `Eval_t(C_t,beta)=t_beta` by the declared native compiler. These claims and the guest relation must refer to a **common extracted source and the same saved public statement**. The external verifier accepts iff the whole ordinary proof accepts and the two exact Vhat_i equations of acc hold. The ordinary outer verifier computes/binds the deterministic linear-query weights from that public beta; it does not trust private prover-supplied weights.

### Lemma 2: honest completeness, with exact scope

For every true ledger and valid native archive, every admitted original source encoding and every admitted original decommitment satisfying the compiler's all-valid-source completeness bounds, a nonaborting execution satisfies the oracle-free relation, all native opening equations, and both external Vhat equations. Lemma 1 proves the polynomial identities; interpolation proves v=g(beta); native linear-query completeness proves t_beta; and the Boolean first graph equation follows from the actual ro/wo calls. No dummy point is used in any step. ∎

This is full conditional completeness of the specified algebraic protocol, **not perfect unconditional completeness of a bounded rejection sampler**. For one attempt with a fresh uniform nu-bit nonce, an explicit bound is

`Pr[honest abort] <= Q_pre/2^nu + p_abort`,

where Q_pre counts all previously exposed/underlying ro inputs that could equal this tuple, including preparation and admitted auxiliary computation. On the complement of that prequery event the current raw output is uniform. A sufficient bounded R_try-attempt policy samples distinct nonces without replacement; if Q_hist bounds all previously made relevant queries over the process, the conservative bound is

`Pr[all attempts abort] <= R_try*Q_hist/(2^nu-R_try+1) + p_abort^R_try`.

The first term charges any prequery/guess hit; fresh distinct inputs then have independent uniform outputs. The actual nonce space must satisfy R_try<=2^nu and fit Enc_m. No hiding assumption is needed to make **different nonces give different cm**: injectivity already does that. An unbounded retry algorithm has a different runtime contract and is not used here.

### Why the CCGOS dummy can be removed here

Construction 8.2 adds a random pair so that g(beta) is uniform in the ZK simulation, by Fact 8.3. The completeness and nonzero-polynomial arguments of §§8.2–8.3 instead need only interpolation at the actual target points and binding of the resulting curve/polynomial. Lemmas 1–2 and §6 below prove those properties directly for n points of target 1. Consequently omission is legitimate for this **one-shot, non-ZK, all-targets-one** variant. It does not establish hiding or ZK; an arbitrary old accumulator or direct Vhat query can have target !=1 and would require a different target interpolation polynomial. Those inputs are intentionally outside this construction.

## 6. Binding and adaptive zero finding, not postchallenge Schwartz–Zippel

### 6.1 The required relation-binding game

Let `R_bind(pp,C_g,C_t;omega_g,omega_t)` be the exact, oracle-free weak-source relation extracted by the entire outer proof. Its deterministic projection returns `(g,t)` with the degree and source-layout bounds above. Define relation binding as infeasibility of outputting two valid such witnesses for the **same original handles and profile** but different projected `(g,t)`. Nonce equality follows from equal cm. Witness equality, equal unit certificates or equal path-decomposition randomness is not required.

This is weaker than canonical CM binding and sufficient for this proof. In particular an extracted source `s=delta^-1*r` need not have a small representative, and need not equal the result of a canonical `Commit(message;coins)` call. CCGOS Lemma 7.1 cannot be applied to it by renaming its state “coins.”

**Native reduction to relation binding.** Compare the two exact source projections, using [Source queries §6](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md). A difference in original path digits gives its B/F collision at the declared cross-profile alphabet/radius. If paths agree, their A images agree. For certified unit factors delta,delta' and bounded products r=delta*s, r'=delta'*s', unequal sources give

`v_A=delta'*r-delta*r'=delta*delta'*(s-s') !=0`, `A*v_A=0`.

The infinity bound is `||v_A||_inf <= K'_1*B_inf+K_1*B'_inf`, where the K_1 quantities are the certified multiplication l1 bounds; alternatively use the admitted mixed Euclidean bound. No inverse norm is bounded. If sources agree, deterministic projections make g and t agree. Apply this comparison separately to C_g and C_t and union over the **actual** matrix inventory and shared setup law. This is an ordinary collision reduction, not knowledge extraction and not commitment homomorphism.

All bits determining g must be bound by C_g, including **canonical public application X** when ledger inputs depend on X. Bind the complete source shapes and require public C_g/C_t to equal the outer proof's actual original advice/quotient handles. Otherwise source equality need not imply equality of the claimed g, or may compare unrelated commitments.

Write `epsilon_RB(T_fork)` for the resulting sum of ordinary Module-SIS advantages, setup distance if needed, and any other *explicitly inventoried* source-comparison failure, at the full forking runtime. The current outer source profiles have not been shown to admit these new comparisons/claims. If the commitment and t/source profiles differ between outputs, the cm encoding must reject the mismatch or the inventory must price that exact mixed comparison; comparing only two honest short encodings is insufficient.

### 6.2 Corrected relation-binding zero finding

Use the complete theorem and strict reduction in [ZERO_FINDING_TRANSPORT](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md). It assumes the reviewed **paired input-local** emulator, the exact oracle-free R_bind relation above, common full-grid/source interpretations, an injective CM tuple, and admitted mixed A/B/F comparisons.

The normalized producer preserves its original accumulator/witnesses. Its tester obtains the actual RO/WO pair at output cm, checks the two external equations and raw sampler, then uses \(H=D_f+1\) curve evaluations to test that

$$h(X)=\widehat V_i(g(X))-[1+Z_D(X)t(X)]\ne0.$$

The tester's retained lookups occur before its terminal curve observations. Let Q bound all first underlying head activations, including WO-triggered and appended-CM activations; let v include all producer and terminal extended queries; let D_* bound the full h degree. For truncation semantics D_*=D_f; the ordinary-MLE variant enlarges it to include the complete coefficient capacity.

Define \(e(u)=\min\{1,u/2^{\lambda_A}\}\) and the unconditional root mass \(\rho=D_*\kappa/2^{\lambda_A}\le D_*/|F|\), capped at one. The reviewed replacement is

$$
\varepsilon_{\rm ZF}\le
\min\!\left\{1,\ e(v)+Q\sqrt{\rho+\varepsilon_{\rm RB}(T_{\rm red})
 +\eta_{\rm samp}+e(v)+2e(v+H)}\right\}. \tag{ZF}
$$

Here \(\eta_{\rm samp}\) is the explicitly bounded sampling-cutoff loss of the strict collision reduction. Its full time includes both producer continuations, both emulator kernels, hidden interpolation, snapshots and complete source comparison. The extra Q factor is real; **the former square-root-in-Q bound is withdrawn**.

The proof guesses one online pre-activation index and runs one independent private continuation. This fixes a comparison source before the main raw challenge. It records H **explicit** hidden curve observations in the persistent main kernel. A real-ARO sandwich relates the augmented and unaugmented joint experiments; the augmented BAD event covers later incompatible graph activation. Equal source projections then give a fixed nonzero polynomial before the raw answer, while unequal projections yield the registered lattice collision. The correlation lower bound is \(\alpha_E^2/Q^2\), not \(\alpha_E^2/Q\). Rejecting sampler outputs never increases unconditional root mass.

The full proof, including h=0 handling and all error terms, is in the reviewed transport note. It does not assume persistence of implicit sparse-graph values, canonical commitment coins, short inverse units, a homomorphic commitment or an efficient exact posterior sampler for the true ARO.

For n=1 the direct soundness argument is stronger and avoids the lemma's positive-degree convention. g is constant and t=0, so the second final equation is exactly Vhat(u_0)=1. A false graph tuple is rejected with probability one whenever the deterministic relation/projection holds.

### 6.3 Feeding the lemma the one complete outer extraction

The [product extraction theorem](AKITA_INSTANCE_ACCUMULATION_PRODUCT_EXTRACTION.md), conditional on its complete deterministic frontend/guest projection, must be applied to the **enlarged** ordinary outer proof: guest/source data and the C_t linear claims together. On success it supplies an R_bind witness and exact v,beta,t_beta,acc for the saved original statement. The first external equation authenticates raw_beta; the guest authenticates S; the opening supplies t(beta); the second external equation supplies h(beta)=0. If a ledger equation is false, Lemma 1 gives h!=0, which is exactly the event bounded above.

The producer in (ZF) is the complete cutoff extraction/projection algorithm, not the original host prover. The reviewed [private-continuation transformer](AKITA_INSTANCE_ACCUMULATION_PRIVATE_ARO_CONTINUATION.md) may keep all post-original extraction queries private while preserving the actual original accumulator/decision. Then Q,v price its **actual external** interface and the appended tester, while T_red still includes all virtual extraction/emulation CPU and the separate outer coupling losses. A forwarding extractor instead prices every forwarded call. Count each selected setup/cutoff event once. The resulting conditional ledger is

`Pr[accepted outer receipt with false ledger] <= beta_outer + epsilon_ZF(whole extracted producer)`.

Native ro-only language soundness uses the separately reviewed **input-local paired** game lift and fixed-input compiler. Neither this corrected authentication theorem nor the PCS theorem supplies the missing complete source/guest certificate.

## 7. F evaluation as actual K-source linear claims

### Coefficients, degree and ignored capacity

Let `r_t=L_t+1` for n>=2. Encode

`t(X)=sum_(i<r_t) sum_(h<4) c_(i,h)*e_h*X^i`, `e=(1,U,V,UV)`, `c_(i,h) in K`.

Provision a fixed original K-source capacity, for example 4*N_t logical coefficients with `N_t=nextpow2(max(1,r_t))`, subject to the actual native commitment shape. This capacity is not a byte-advice allocation. Its frozen decoder and physical map must be explicit.

A sound simple choice is **truncation semantics**: the projected t uses exactly i<r_t; all remaining logical capacity and any unused physical cells have coefficient weight zero in every t claim. Honest encodings may zero them, but weak extracted sources need not. There is no inference that honest zero padding is an extracted zero predicate. Source binding still compares the full original source. An alternative is to include all capacity in the polynomial, explicitly prove high coefficients zero, and charge those additional claims/prefixes. Treating an ordinary degree-(N_t-1) evaluation as if it had degree L_t without either truncation or a zero proof is unsound bookkeeping and invalidates (ZF).

For n=1 define the projection t=0 with zero coefficient count. If retaining the common C_t grammar, use at least one physical capacity slot and a zero projection, and require t_beta=0 in the relation; a zero-functional source occurrence can retain source authentication if the surrounding protocol requires it. A specialized n=1 protocol can omit the otherwise useless C_t and openings altogether, but must identify that different public grammar. No negative array length or automatic degree claim is used.

### Exact linear forms

For beta=`a_0+a_1 U+a_2 V+a_3 UV`, multiplication by beta in the declared basis is

```
M_beta = [ a0  5a1  5a3  5a2
           a1   a0   a2  5a3
           a2  5a3   a0  5a1
           a3   a2   a1   a0 ].
```

Therefore, for each output coordinate v in {0,1,2,3},

`[t(beta)]_v = sum_(i<r_t,h<4) (M_beta^i)_(v,h) * c_(i,h)`.

This is a K-linear functional of the quotient's coefficient planes. Compose it with the **actual linear native decoder** from source digits to c; for positional gadget decoding this includes the b^digit factors and actual packing/permutation. If a proposed decoder has nonlinear canonicalization, it needs a separate constraint relation and cannot be silently treated as this map. Beta is public for weight generation and is fixed in the outer opening descriptor.

For a binary exponent i, `M_beta^i=product_j M_beta^(2^j*i_j)`. This gives the familiar bond-four matrix-product/tensor representation. It does **not** make this an ordinary E-valued MLE point, because generally beta is not in E. Across a compatible contiguous coefficient cut the matrix indices give at most four separated terms; plane axes, native packing, masks and truncation must also be accounted for. No rank-four claim for an arbitrary packed source is assumed.

### A finite native linear compiler, with all its extra channels

Here is a conservative algebraic path requiring no low-rank guess. It specializes [Source queries §§4.1–4.3](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md) to K-valued weights and full-ring lattice action.

Write the original source blocks as `s_b in R^(m_A)`, b=1,...,J with `J=P*B`, `m_A=M_src*delta_com`, `R=K[Y]/(Y^d_A+1)`. After composing decoding and masking, each coordinate claim is a sum of arbitrary K-coefficient functionals on these blocks. For every output v and block h define public coefficient weights `w_(v,h,c,j)` and the common R-linear map

`L_(v,h)(s)=sum_c w^vee_(v,h,c)(Y)*s_c(Y)`,
`w^vee_(v,h,c)(Y)=w_(v,h,c,0)-sum_(j=1)^(d_A-1) w_(v,h,c,j)*Y^(d_A-j)`.

The negacyclic constant coefficient satisfies `ct(L_(v,h)(s))=sum_(c,j) w_(v,h,c,j)*s_(c,j)`, and `L_(v,h)(delta*s)=delta*L_(v,h)(s)` for every native ring challenge delta. It is **L**, not ct, that commutes with ring challenge multiplication.

For one output v make H=J channels. Authenticate every partial `e_(v,h,b)=L_(v,h)(s_b)`, not just the diagonal. For a shared folded response `z=sum_b challenge_b*s_b`, impose all J ring equations

`L_(v,h)(z)=sum_b challenge_b*e_(v,h,b)`

and the scalar equation

`sum_h ct(e_(v,h,h))=[t_beta]_v`.

Keep the original A equation and every original B/F row. D/H authenticate the entire partial vector. Do this as four explicitly charged source occurrences (one per v), or as a newly declared shared multi-output relation with equivalent full authentication. The separate-occurrence baseline has, before digitization, **4*J^2*d_A K partial coordinates**, four responses/source-path occurrences, and 4J fold-evaluation ring rows plus four scalar query rows. K weights need one R-valued carrier per channel here, not a fictitious E trace contraction. Outer scalar batching challenges remain in the admitted E.

For each of those four occurrences the symbolic source-query ledger is

`Z=m_A*delta_f*d_A`,
`E_part=J^2*delta_open*d_A`,
`T_path=J*n_A*delta_open*d_A`,

plus its original B/F compression slots, all selected quotient rows or quotient-free adjoints, and the enlarged shared D/H path, alignment and terminal padding of [Source queries §8.2](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md). The four occurrences multiply the applicable quantities; shared D/H is sized at their total width, not charged as an unchanged tiny commitment. A proved rank factorization may replace H=J, but this is an optimization, not a completeness/soundness assumption.

**Local extraction proof.** Descendant-certified D/H binding fixes all partials across fork children, and original B/F binding fixes their A images. Subtract the two native unit-difference children. For each channel, cancel the unit in the R-linear equation to recover `e_(v,h,b)=L_(v,h)(s_b^*)`. The exact scalar row gives the declared coordinate functional. The original-source comparison in §6.1 identifies the same s across all four occurrences and the other uses of C_t. Thus the extracted t satisfies all four claims. This is precisely the source-query local proof with explicit public weights; it does not turn an accepting transcript into a complete descendant tree for free.

**Local completeness proof.** Given any valid bounded original source/path witness, compute all L partials and z from that very source, without canonical redecomposition. The ring and scalar equations hold identically. This extends to protocol completeness only when all response, partial-digit, quotient, path and compression alphabets admit those computed values for **every** valid source. A deterministic sufficient response interval is the source-query bound `||z||_inf<=J*kappa_1*B_s`; general dense dual products require their own quotient envelope. The old sparse/trace opening bound cannot be reused automatically.

This supplies an actual finite **algebraic** query path. It is not an implemented API or an admitted parameter set. If the outer PCS accepts only its present ordinary E-point openings, the candidate route as stated stops here: those openings do not prove the required F evaluation. The narrow correction is precisely a declared linear-query extension such as this one, or a separately proved lower-cost compiler, **outside the guest**.

## 8. Additional admission requirements and remaining host work

A complete instantiation must supply all of the following; none follows from moving t out of guest advice:

1. **Source capacity and setup:** an original C_t source shape for 4*r_t live K coefficient values, its original A/B/F maps, bounded source digits and path alphabets, canonical handle encoding and admitted setup distribution. All added outer matrices/widths and weak/weak comparisons enter the outer inventory. The inner 33-role inventory and T16 remain unchanged; they are not spare outer source capacity.
2. **Complete C_g projection:** all g-defining y,z and framing/read semantics from a common original weak source. If original advice reuse fails, use a separately provisioned source/commitment and an explicit link to the guest-consumed bytes. Simply adding C_g without proving this link leaves the same hole.
3. **Opening compiler and schedule:** four F-coordinate K-linear claims, actual decoding adjoints/masks and typed query descriptors, all new partial/response/path rows, common-source comparisons, scalar batching if chosen, accepted lattice unit challenges, all-valid-source response bounds and complete deterministic accepting-tree extraction. No current ordinary E-point API is asserted to do this.
4. **Degree/padding:** exactly the truncation semantics above or authenticated coefficient zeros; explicit n=1 behavior; distinct interpolation domain and D_f+1<=M_F; bounded n and coefficient capacity in the public profile.
5. **Public linkage/order:** original C_g,C_t and all interpretations fixed in cm before beta; postchallenge values excluded from a cyclic C_g source; exact graph/raw sampler mapping; same acc and t claim in the outer statement and external verifier; bounded retry and canonical nonce grammar.
6. **Weak binding law:** the actual native path alphabets and unit-product radii across both forks, under the joint setup law, sufficient for the ordinary SIS relation-binding reduction. A canonical Commit theorem alone does not supply this.
7. **Full outer projection and resources:** a single outer extraction for the guest and new source claims, deterministic guest adequacy, expected work plus a strict cutoff, total inner/underlying oracle queries at the cutoff, and a standard-model reduction cost for the complete emulated ZF fork. Existing inner Q bounds cannot be recycled for this extracted producer.

The host still constructs/retains g and a degree-D_f composition polynomial or equivalent interpolation material, obtains up to D_f+1 Vhat evaluations, computes polynomial division, represents at least 4*r_t K quotient coefficients, commits their full original lattice source and produces its openings. Generic interpolation/division is finite and polynomial work; faster structured-domain algorithms need the actual domain/profile. The guest still parses/authenticates the native archive and z witnesses, handles a graph list of a*n Boolean coordinates, and computes v and Z_D(beta), for example by barycentric interpolation in O(a*n) field work after its public weights are available. Nothing here promises current advice/trace limits or performance parity.

The [dense-flat cost note](AKITA_INSTANCE_ACCUMULATION_ORACLE_QUERY_COST.md) rules out storing its original dense f representation in the existing byte-advice layout. The present change genuinely removes that particular guest storage requirement. It does **not** erase the degree, host polynomial work, commitment/source capacity, arity m, or frontend costs, and is not an impossibility result for other constructions.

## 9. Narrow conclusion

There is no algebraic obstruction to the one-shot non-ZK all-targets-one quotient construction. The random dummy is dispensable, f is replaced by `1+Z_D*t`, and a relation-binding version of the CCGOS fork proof suffices for weak native states with explicitly priced source comparisons. The 1024-bit raw sampler gives a mathematically valid degree-four F/K interface and the required root/fork bounds without identifying raw bits with field coordinates.

There **are concrete obstructions to claiming this for the unchanged receipt**: current ordinary E-point openings do not automatically open t at F points; generic advice binding does not automatically bind every graph-defining byte under weak extraction; a C_g source containing beta-dependent data creates a fixed-point completeness problem; and cm may not fit a proposed small fixed ARO input. The narrow sound corrections are the explicit outer linear-claim/source extension, complete same-source guest linkage with noncyclic data placement, and an admitted injective cm encoding of sufficient arity. They add no unproved cryptographic knowledge assumption, but their numerical/source/frontend admission is not supplied by the existing notes.

This is proof-first research only. No implementation, Git operation, build, test, formatter, benchmark or executable check was performed.
