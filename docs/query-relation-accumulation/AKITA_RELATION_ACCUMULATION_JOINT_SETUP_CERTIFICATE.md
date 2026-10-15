# Candidate1: exact joint setup reduction, mixed radii, and runtime certificate

## 0. Result and selected base contract

**The joint setup reduction closes in the declared base model.** The indexed local base text explicitly models setup expansion as a random oracle (`Papers/parsed/Akita.md:1761`) and separates independent setup and Fiat–Shamir namespaces (`:1767`). Thus the applicable setup model is **B: transparent seeded setup in the approved programmable classical indexed ideal ROM**. No new setup-hardness assumption is required. The actual default seed may remain fixed. The independent reduction below embeds a named original ring Module-SIS challenge into the actual coefficient-prefix/page layout and samples the original canonical-rejection raw bytes conditional on that embedding. It never chooses seed preimages.

**Provenance caveat:** these statements identify the declared contract of the indexed local text, not an independently authenticated official paper body. The root protocol already records that official-body authentication was unavailable. No expanded local Akita extraction theorem is used as proof of the embedding, collision algebra, or resource theorem in this certificate.

All overlapping A/B/F/D/H matrices and installed canonical setup-prefix references must be derived from one shared coefficient vector. Mixed A radii and source B/F alphabets must be inventoried across every comparison made by the complete suffix. Diagonal table admission suffices for one fixed consuming occurrence; across profiles it suffices only under the inequalities proved below. The full runtime-conditioned term is `Adv_MSIS_I(T_I(L), eta_I)`, with one global cutoff loss `W/L`. A numerical Adv API is **not required** for qualitative PPT soundness under the explicit polynomial-family/resource and negligible-statistical-loss hypotheses in §10. No modeled-ROP-to-advantage conversion or additional end-to-end 128-bit target is used.

This is a read-only investigation and mathematical derivation. No repository file was edited, and no Git operation, build, test, linter, formatter, estimator or control was run. The only changed output is this local artifact. Initial arithmetic remains E=K. There are no full per-segment PCS proofs, source/commitment homomorphisms, new commitments or setup trapdoors. This certificate does not recertify the known incompatible legacy Blake2b/seed-shell interface; it conditions transcript extraction on the AQRA boundary specified in `AKITA_RELATION_ACCUMULATION_INDEXED_TRANSCRIPT_DESIGN.md`.

## 1. Evidence: actual setup, consumers and installation boundary

Paths are workspace-relative; observations are source inspection, not exercised checks.

1. **Declared base setup model.** `Papers/parsed/Akita.md:1761` says the advantage formula models expansion as a random oracle and requires prefix consistency; `:1767` names independent restrictions `H_setup` and `H_FS` and excludes setup-recompute calls from the Fiat–Shamir forking Q. Setup calls still incur their actual runtime. This is a model declaration, not a claim about concrete SHAKE.
2. **Actual page law.** `akita/crates/akita-types/src/proof/setup.rs:22–35,227–258` defines `Shake256PagedV1`, page size 4096 and sequential `F::random` calls from each page's SHAKE reader. Neither ring dimension nor requested coefficient-prefix length is absorbed.
3. **Literal page key.** `.../setup.rs:317–377` absorbs length-delimited label/data pairs, each length as LE64, in this order: domain=`akita/commitment/public-field-stream`; derivation=`shake256-paged-v1`; page_field_elements=`LE64(4096)`; seed=32 bytes; field=32-byte big-endian modulus; page=`LE64(page_index)`. One page owns one prefix-consistent XOF stream.
4. **Default seed is constant.** `.../setup.rs:219–225` uses `LE64(0xDEAD_BEEF_CAFE_BABE)` followed by zeros. `akita/crates/akita-cpu-backend/src/setup/prover.rs:42–70` uses it. Its ideal-oracle input is public and fixed; it is not fresh entropy.
5. **Canonical rejection, not modulo reduction.** `jolt/crates/jolt-field/src/solinas/fp128.rs:1089–1095` delegates to `sample_uniform_below`. `jolt/crates/jolt-field/src/solinas/mod.rs:73–100` reads little-endian bytes, masks excess high bits and rejects candidates at least q. For `Prime128OffsetA7F7`, q=2^128−2^32+22537, this consumes 16 bytes per candidate. With iid ideal input bytes its accepted coefficient is exactly uniform.
6. **Exact view layout.** `akita/crates/akita-params/src/layout/flat_matrix.rs:61–135,152–185` uses the first `n*m*d` coefficients and element offset `(row*m+column)*d`. Each view has its own column stride. Different-width views are not slices of a fictitious common rectangle with a fixed global stride.
7. **Consuming dispatch preserves sharing.** A/B/D native relation consumers use the same `shared_matrix().ring_view_dyn` in `akita/crates/akita-cpu-backend/src/opaque/relation_weights/compiler.rs:531–575`. Both F and H are explicitly routed by `akita/crates/akita-types/src/proof/compression_relation_weights.rs:614–649` to the shared-prefix rank-one view `(1,input_width,ring_dimension)`. Reduced compression does the same in `.../compression_relation_weights/reduced.rs:44–55`.
8. **Actual canonical registry source.** `akita/crates/akita-cpu-backend/src/setup/setup_prefix.rs:98–144` commits **all** `shared_matrix[0..n_prefix]`, including beyond `natural_len`, using the frozen profile and its compression plan. `akita/crates/akita-params/src/schedule/profiles.rs:274–310` requires full singleton-prefix geometry and defines natural_len as support of the later weight, not zero source padding.
9. **Verifier-side registry routing.** `akita/crates/akita-verifier/src/fold/suffix.rs:29–81` selects the public payload by its scheduled slot ID from verifier setup, rejects a missing slot and validates/reorders the complete group list. A proof replacement payload is not used there.
10. **Provenance is a premise.** `akita/crates/akita-types/src/proof/setup.rs:143–163` states that `AkitaVerifierSetup::from_parts` binds seed identity but does not prove stored commitments' derivation. Registry insertion (`.../setup_prefix.rs:247–265`) checks structure. The external backend import does recompute/check retained and public material: `akita/crates/akita-cpu-backend/src/opaque/owned_prefix.rs:222–272`. Trust generation or establish equivalent installation provenance; seed equality alone is insufficient.
11. **Capacity closure includes nonresident sources.** `akita/crates/akita-cpu-backend/src/setup/prover.rs:74–132` allows a smaller resident verifier matrix while retaining the full registry. The reduction must cover prover, verifier, registry-source and registry-key footprints together.
12. **Admission exports geometry, not the complete reduction.** `akita/crates/akita-params/src/sis/norm_bound.rs:25–107,182–235` computes digit diameters and diagonal A bounds. `akita/crates/akita-schedules/src/audit.rs:285–380` audits the selected routes and exact widths. `akita/crates/akita-params/src/schedule/sis_occurrences.rs:12–253` enumerates A/B/D and compression occurrences, adding only A at the terminal; it neither prices full runtime nor constructs canonical references/mixed pairs.
13. **Source is not its weight or an honest buffer.** `akita/crates/akita-cpu-backend/src/opaque/sumcheck/stage3/mod.rs:220–251` retains the real full prefix but zeros the weight after active rows. `AKITA_RELATION_ACCUMULATION_GRID_LOCAL_CERTIFICATE.md` independently identifies full decoded source S, current zero-point slice W and physical cells range-checked rather than forced to zero. This certificate keeps those exact domains; no cropping or invented padding zero is used.

## 2. Exact named Module-SIS coefficient embedding

Fix before sampled setup one admitted row/suffix and a named collision instance

`I=(q,d_I,n_I,m_I,p_I,eta_I,view_id)`.

The **original ring** is `R_I=K[X]/(X^d_I+1)`. Its ordinary Module-SIS challenger supplies `A* <- uniform R_I^(n_I x m_I)`. Success means a nonzero modulo-q kernel vector with a coefficient integer lift of p_I-norm at most eta_I. There is no ring replacement, scalarization assumption or source transformation.

Let N_setup cover every needed coefficient, including complete canonical registry sources and their commitment matrices. For this view define

`f_I(r,c,k)=(r*m_I+c)*d_I+k`,

`Phi_I(r,c,k)=(PageKey(seed,q,floor(f_I/4096)), f_I mod 4096)`.

The ranges are `r<n_I,c<m_I,k<d_I`. The second address component is an **accepted field-output ordinal**, not a fixed byte offset. Equality of addresses gives equality of f_I; its mixed-radix expansion recovers (r,c,k), so Phi_I is injective. Variable rejection makes a fixed raw-byte-offset embedding invalid.

Construct one vector S with

`S[f_I(r,c,k)]=coeff_k(A*[r,c])`,

and all other needed coordinates independent uniform in K. The coefficients of a uniform ring matrix are independent uniform, so S has exactly the iid coefficient-prefix law. Derive every other actual view J as

`M_J[r,c](X)=sum_(k<d_J) S[(r*m_J+c)*d_J+k] X^k`.

No other view is independently resampled. Overlaps coincide exactly at shared flat indices, including across different d values. Regrouping coefficients requires no cross-ring multiplication homomorphism. The named view is literally A*. A collision returned for it is returned in its original ring with the same norm—no extra rank, width, sqrt(d), or extension factor.

For a sliced B path, a nonzero difference in the stacked path has a nonzero slice. Restrict to that slice and zero-extend only to B's **actual declared width**; the kernel equation is preserved and norms cannot increase. This does not zero any source or compression-input cell.

This is an independent exact joint distribution proof. It does not invoke an expanded Akita extraction theorem.

## 3. Exact conditional raw bytes for the actual seeded page sampler

The coefficient embedding must also let the adversary read/recompute the public seed's XOF. Programmability supplies this by the **original conditional byte law**, not by inversion of SHAKE.

### Prescribed accepted coefficient

For fp128 let `U=2^128`, `p=q/U`, and prescribe `a in [0,q)`. Conditional on the rejection sampler eventually accepting a:

1. Sample `J>=0` with `Pr[J=j]=(1-p)^j*p`.
2. Sample J independent words uniformly in `[q,U)` and serialize each little-endian.
3. Append the exact 16-byte little-endian word a.
4. Keep the subsequent unconsumed stream iid uniform.

Proof: `Pr[J=j, accepted=a]=(1-p)^j/U`; division by `Pr[accepted=a]=1/q` gives the claimed geometric law. Conditional rejected words are independent uniform over the rejection set and independent of a. Thus inspection of rejected bytes is simulated exactly, not only accepted values.

For a smaller supported prime with b significant bits and h=ceil(b/8), use `U=2^b`; rejected **masked** low words are uniform in `[q,U)` and all discarded `8h-b` high bits are independent uniform. Accepted low bits equal a, with independent high slack. The selected field codec must match this law; E=K A7F7 needs no extension-limb argument.

### Page construction and prefixes

Apply the conditional law sequentially to prescribed accepted values in ordinal order. Fill unassigned required values uniformly. It is safe to fill an entire 4096-value page, then append an iid byte tail. Distinct literal page keys are independent. Every later prefix read returns the saved bytes and extends that same stream; longer coefficient-capacity requests use the same saved page ordinals. Modeling different requested XOF lengths as independent keys would break the real setup.

Mixing over uniform A* and unassigned coefficients gives, on every compatible finite exposed prefix,

`Pr[S=s]*Pr[raw=b | decoded(raw)=s]=Pr[raw=b]`.

The lazy consistent extension gives the whole oracle law. A fixed seed merely names an ideal-oracle input; entropy is in the answers, not in the finite seed. No seed-preimage selection, hash inversion, or seed change is needed.

Program setup before any simulated setup-dependent exposure, including allowed initialization. Every arbitrary adversarial read at a setup page key receives the same raw table, whether or not it goes through `F::random`. Never overwrite already exposed answers. If an application has an immutable prior realized oracle table outside the reduction, the fresh-setup theorem does not silently cover it; include its generating initialization in the modeled experiment or state that different boundary explicitly.

### Cost and termination

Expected words per coefficient are `1/p`. Rejected words can be drawn by sampling an offset below `U-q` directly, with ordinary exact mask/rejection of acceptance at least 1/2. There is no need to wait for a rare rejected word in forward random sampling. Exact Bernoulli-p trials generate J with expected `1/p` iterations. For `p=1`, J=0 and no rejected-word draw exists or is needed. This is expected-polynomial in the field/required-page encoding and exposed length.

The existing setup loop is unbounded, so it has no finite worst-case raw length. Charge all raw draws/bytes/table operations and apply the **single global work cutoff** in §9. Do not silently replace this setup law by the separate transcript design's capped sampler. Laziness can avoid unexposed tails, but not the cost of exposed, replayed or copied bytes.

## 4. Model B is the declared base; distinguish concrete instantiation

The indexed base's explicit `H_setup/H_FS` declaration in §1 resolves the earlier potential ambiguity: **select model B for the Candidate1 theorem**, with zero setup-distribution deviation for the actual exact-rejection implementation under that idealization. Setup and FS are independent namespace restrictions of the same approved programmable classical ideal model. A simulator can implement them with lazy random coins when attacking ordinary Module-SIS. This adds no lattice oracle or new hardness assumption.

**Accounting:** setup-recompute queries are not part of the base Fiat–Shamir Q, shell/source-cut forking loss or old-FS-key ledger. They remain charged in full reduction time, raw-answer storage and work. If an implementation/certificate conservatively counts all queries in Q, that is an explicit looser bound, not a reason to omit setup work. Oracle-dependent initialization contributes to FS Q exactly for its actual FS-namespace calls; its setup-namespace calls contribute time. The namespace boundary must be literal and domain separated, with all adversarial calls answered consistently.

For clarity, two different statements are **not** silently implied:

- **Model A:** directly sample uniform coefficient S and derive all views. §2 alone is exact, but seed metadata cannot falsely claim derivation by an unrelated unprogrammed concrete function.
- **Model C:** concrete SHAKE at the actual fixed seed, or soundness conditioned on one already realized hard-coded matrix/registry. Bare hardness for a uniform Module-SIS matrix does not give this distributional reduction. Even a random **revealed** seed cannot be replaced by independent S using an ordinary PRG slogan: recomputing `Expand(seed)` distinguishes `(seed,Expand(seed))` from `(seed,uniform S)` by equality, with the latter matching with probability `q^(-N)`.

No model-C claim is needed for the approved ideal-model result. A separate seeded/fixed-matrix hardness assumption would be a genuinely different statement and is not introduced here. The model-B theorem averages over its ideal setup oracle, generates the installation once, then reuses it unchanged across all proofs and private source-probe forks. It does not resample setup per fork. The installed shape/profile/selection rules are fixed before sampling; actual registry commitments are generated from that one sampled S. A requirement to preserve arbitrary already hard-coded commitment bytes under every challenger embedding would be a different, generally incompatible fixed-realization boundary.

The indexed text's original-body authentication caveat remains explicit. The model declaration is evidence for the selected base contract; §§2–3 independently prove how the actual paged rejection implementation realizes its joint ideal law.

## 5. Canonical installed source and registry correlation

For each installed slot id set `N_id=2^(profile.group.num_vars)` and define its public source exactly as

`S_id[t]=S[t]`, for `0<=t<N_id`.

`natural_len` is only the later weight support. Retain its canonical A-input blocks `s_ref`, inner-image digits, all B slices, and every F-chain input under the frozen profile. Its registry commitment is the efficient public function

`C_id=Commit_profile(S_id;S)`.

Source/key correlation causes no distributional gap: the reduction knows S and computes **that same function**, even if it is nonlinear in S. No secret, independent-key lemma, homomorphism or commitment-to-key circular-security assumption is needed. N_setup includes N_id and every key footprint used in committing it; the complete installed registry and any allowed lazy growth need a polynomial bound. The verifier's smaller resident prefix is not the complete simulation envelope.

The canonical reference's weak certificate is

`rho_ref=1`, `r_ref=s_ref`.

Here s_ref means the selected A-input **digit blocks**, not a raw full-field table relabeled as short. For canonical balanced A digits of base b_A, a safe profile-only bound on each compared A vector is

`Binf_ref<=b_A/2`, `B2_ref<=sqrt(m_A*d_A)*(b_A/2)`.

Tighter plane-specific canonical bounds are allowed with proof. `akita/crates/akita-algebra/src/ring/cyclotomic/decomposition.rs:14–65,84–138` supplies the balanced decomposition convention. `akita/crates/akita-cpu-backend/src/opaque/owned_prefix.rs:156–177` explicitly treats setup-prefix production as full-width balanced digits.

Canonical B alphabets come from the frozen **outer** base, not b_A or q. Canonical F inputs use the actual negative-binary decomposition, including real padded input columns. This reference needs no evaluation point, D payload or H opening. Two arbitrary weak certificates agreeing on an attacker-consistent auxiliary table do not authenticate the installed setup; the canonical reference comparison cannot be omitted when setup offload is selected. Installation provenance is the trusted-generation/import premise of §1, not structural slot validation alone.

## 6. Independent original A/B/F collision derivation

Compare two returned source projections for the same original handle, exact commitment shape and native A/B/F path. Points, scalar claims and opening-local D/H payloads may differ. Their certificates are

`r_u=rho_u*s_u`, `r_v=rho_v*s_v`,

where rho values denote their **unit images** in the common native A ring. No short inverse is assumed.

Compare source paths backward from the same public payload. Differing F input vectors with equal F outputs give a nonzero F-kernel vector. If later inputs agree, their fixed recomposition gives the preceding output, so continue backward. At B, differing digits give a B collision; choose a nonzero slice on a sliced route. If all path digits agree they give the same native A-image t. If source blocks still differ, form

`z_A=rho_v*r_u-rho_u*r_v=rho_u*rho_v*(s_u-s_v)`.

Then `A*z_A=0`; it is nonzero modulo q because the rho values are units and source blocks differ. Integer convolution lifts obey the triangle/Young/operator bounds in §7. Taking centered coefficient representatives cannot increase those norms. Thus the output is a short collision for the original ring Module-SIS instance, not an arbitrary field-sized algebraic kernel.

D/H use the same backward digit-chain argument **only for compared openings sharing the same actual D/H payload**, as in the appropriate retained shell siblings. They are opening-local and are not source identity. Requiring unrelated point/probe continuations to share D/H would not justify source coalescence.

All multiplication here occurs inside the extraction/collision proof. Original source commitments are never added, multiplied, redecomposed into replacement commitments or rekeyed. This proof uses the explicitly listed extraction/digit/unit certificates as inputs, not an expanded local Akita knowledge theorem.

## 7. Complete mixed-norm ledger and diagonal coverage

### 7.1 Occurrence certificates

Retain for every reachable occurrence u, marking unavailable entries **absent**, not zero:

`K1_u>=||rho_u||_1`,

`Kop_u>=||Mul(rho_u)||_(2->2)`,

`K2_u>=||rho_u||_2`,

`Binf_u>=||r_u||_inf`,

`B2_u>=||r_u||_2` on the complete `m_A*d_A` coefficient vector,

and, when available,

`Bop_u>=||a -> (r_u[j]*a)_j||_(2->2)`.

Also retain the complete source-path input alphabets D_(K,u), including actual padding, original handle/shape/view, consuming row/level, physical response map, unit embedding, sampler/filter and proof ownership.

**Ordinary nonterminal Linf.** Let a be enforced range base, b response recomposition base, delta response depth, m summed response chunks, omega raw shell L1 mass, and `S_delta=sum_(j<delta)b^j`. Then

`Zinf=m*(a/2)*S_delta`,

`Binf=Delta=m*(a-1)*S_delta`, `K1=2*omega`.

The accepted interval is `[-m*(a/2)*S_delta, m*(a/2-1)*S_delta]`. Binf is its **diameter**, not Zinf, the source's honest digit magnitude or an A bucket. In inspected ordinary rows a=b, so `Delta=m*(b^delta-1)`; do not apply that simplification to a future profile without checking it. No raw source norm follows from `rho*s=r` alone.

**Implemented scalar physical L2.** A verified complete response squared cap C and accepted challenge multiplication-operator bound O give

`B2=2*sqrt(C)`, `Kop=2*O`,

and diagonal squared A radius `64*O^2*C`, as in `sis/norm_bound.rs:88–107`. Recursive integer/no-wrap/complete-cell hypotheses remain required; equality of field sums of squares alone is not an integer cap. `akita/crates/akita-types/src/proof/relation_range_image.rs:53–128` rejects precommitted/multigroup L2 and checks complete physical length. `akita/crates/akita-schedules/src/candidate.rs:34–93` currently requires level>=3, one claim, one chunk, basis>=8 and prices its selected operator filter or deterministic L1 bound. This certificate preserves those restrictions.

**Direct terminal.** There is no successor range proof or B/F/D/H source path. A directly checked raw cap Z_T gives `Binf_T=2*Z_T`, `K1_T=2*omega_T`, diagonal radius `8*omega_T*Z_T`. It is not inferred from retained digit-depth metadata. A direct L2 cap C_T gives `B2_T=2*sqrt(C_T)`, and its actual accepted filter gives `Kop_T=2*O_T`, diagonal square `64*O_T^2*C_T`. See `akita/crates/akita-verifier/src/fold/terminal.rs:114–176`, `.../terminal/direct.rs:131–210` and `akita/crates/akita-params/src/sis/norm_bound.rs:133–180`. The signed-i16 wire limit is an additional codec restriction, not a separately selected Linf route for an L2 terminal.

**Canonical reference.** Use `K1=Kop=K2=1` and §5's actual A-input envelopes and path alphabets. Do not invent a reference D/H opening or terminal scan.

### 7.2 Mixed A radii

The one collision in §6 has coefficient bound

`Einf(u,v)=K1_v*Binf_u+K1_u*Binf_v`.

Its available Euclidean bounds are

`E2,1(u,v)=K1_v*B2_u+K1_u*B2_v`,

`E2,2(u,v)=Kop_v*B2_u+Kop_u*B2_v`,

`E2,3(u,v)=K2_v*Bop_u+K2_u*Bop_v`.

Only use pairings whose required certificates exist on both sides. In the third line Bop is the **vector** multiplication-operator certificate defined above, not an unrelated scalar polynomial's operator norm. These inequalities follow directly by bounding each summand of z_A.

For a Linf comparison class C, require its actual selected capacity at least `max_(u,v in C) Einf(u,v)`. For an L2 class require its selected **squared** capacity at least

`max_(u,v in C) (min_(available r) E2,r(u,v))^2`,

with at least one valid pairing for every pair. Keep the cross term on squaring sums of radicals; compare exactly or use a proved rational upper bound.

Valid conversions, with `N_A=m_A*d_A`, include `B2<=sqrt(N_A)*Binf`, `Binf<=B2`, `Kop<=K1` and a crude `Bop<=sqrt(d_A)*B2`. The last follows by summing scalar convolution L1/L2 bounds over the vector blocks. Record these factors and still check the selected route/capacity; they are not a second admission convention. An already complete B2 receives no additional width factor.

A canonical-reference pair specializes to `Binf_u+K1_u*Binf_ref`, or `B2_u+Kop_u*B2_ref` when available (also the K1 variant).

### 7.3 Source B/F and local D/H alphabets

For each source-path view K,

`eta_(K,C)=max_(u,v in C) max_(x in D_(K,u), y in D_(K,v)) ||x-y||_inf`.

Balanced intervals `[-a_u/2,a_u/2-1]` and `[-a_v/2,a_v/2-1]` give `(a_u+a_v)/2-1`; a common alphabet gives `a-1`. Use the **consuming verifier's enforced alphabet**, not the honest B-decomposition base. A canonical B reference uses its canonical outer-base alphabet; frozen admission checks `opening_base>=outer_base`, so that interval is nested when the checks apply. See `akita/crates/akita-params/src/layout/params/precommitted.rs:307–408` and `akita/crates/akita-schedules/src/audit.rs:352–380`.

F/H radius is 1 only when every actual input coordinate on both sides is authenticated in `{-1,0}`, including all real padding columns. Negative-binary padding is not a claim of zero padding. The full support/address proof is the GRID/LOCAL contract; `akita/crates/akita-params/src/sis/compression.rs:9–10` supplies the production constant 1. D gets its local complete input diameter. No D/H term is used to explain a cross-point source mismatch whose D/H payloads differ.

### 7.4 When admitted diagonal capacities really cover the pair inventory

At **one fixed consuming occurrence**, repeated certificates have the same public K1 and Binf, or Kop and B2. Its mixed radius is exactly its diagonal radius. The enforced B and binary F alphabets are also fixed. This closes within-row source-probe collision admission subject to the genuine GRID/LOCAL/unit inputs.

For several occurrences sharing a frozen A view, common K1=K gives

`Einf(u,v)=K*(Binf_u+Binf_v)<=max_w 2*K*Binf_w`.

More generally pairwise co-monotonicity `(K1_u-K1_v)*(Binf_u-Binf_v)>=0` gives

`Einf(u,v)<=K1_u*Binf_u+K1_v*Binf_v<=max_w 2*K1_w*Binf_w`.

The same proof holds for a **common** Euclidean pairing (Kop,B2 or K1,B2). Unrelated diagonal minima from different pairings do not imply this fact. B's balanced intervals nest by base, so the maximum diagonal diameter covers their mixed diameters. F stays 1 only under the complete common binary certificate.

For a reference, `Binf_ref<=Binf_u` and `1<=K1_u` suffice for its mixed radius to fit u's diagonal; otherwise directly prove `Binf_u+K1_u*Binf_ref<=selected_capacity`. The analogous Euclidean conditions apply to B2 and the chosen operator bound. Use the actual reference **A digits**, not full-field setup coefficients.

There is no general maximum-diagonal theorem: algebraic pairs `(K,B)=(2,100)` and `(100,2)` each have diagonal radius 400, while their mixed radius is 10004. This refutes that inference, not an asserted actual Akita row. Different response depths, chunks, filters, methods or references need the real pair inequalities. `sis_occurrences()` alone does not establish them.

## 8. Usable actual-row and complete-suffix inventory template

A unique numerical inventory cannot be filled without selecting the Candidate1 row and frozen installed slot profiles. Neither the Dense PCS-only control nor the saved Main/RAM control is silently substituted for that choice.

### Installation record

Record field/canonical codec, E=K, **model B**, literal setup page grammar, exact catalog/row descriptors, source roster/aliases, installation-generation/provenance rules, all installed slot IDs/profiles, full N_setup closure, allowed initialization, AQRA transcript/compiler version, finite horizon, FS-Q convention and global work policy. Source setup values are generated once by the declared ideal setup oracle, not resampled across probes.

### Occurrence record

For every scan/reference retain:

`(occurrence_id, original_handle, consuming_level, shape, A view(q,d,n,m), B slice geometry, ordered complete F path, response route, a,b,delta,chunks, shell/carrier/unit embedding/filter, K1,Kop,K2,Binf,B2,Bop, B/F input alphabets, decoder/proof owner)`.

Add the canonical installed reference for every setup commitment opened, with its actual whole-prefix source and canonical alphabets. Add the real A-only terminal with direct codec/norm metadata. Do not invent terminal source compression or a successor range scan.

### Comparison record

For each reachable comparison retain:

`(class, handle/shape equality rule, u,v, algorithmic comparison site, selected norm, available pairings, mixed radius, selected table capacity, proved <= inequality, target instance label)`.

Including all pairs in a class is safe; omitting pairs requires a reachability proof. Equal unrelated payload bytes do not establish the application's source aliasing. Equal payloads of different shapes do not permit source coalescence.

### Collision-instance record

Include A at the class's complete mixed radius and every active B/F source-path instance at its complete mixed alphabet radius. Include each nonterminal level's D/H at local radii. Terminal contributes A only. Installed setup authenticity enters through its ordinary source class plus canonical reference. Retain

`(q,d,n,m,p,eta or eta^2, selected table/policy/digest, exact flat map, comparison sites, capacity proof, T_I(L))`.

Use `FoldSchedule::sis_occurrences()` for executable dimensions/capacities, then enrich it. Its row count is not extraction-tree size or collision-attempt count. Choose a canonical first-collision label; repeated opportunities for that label are covered by the full runtime of its collision finder. Summing over possible first labels needs no independence between overlapping matrices. Duplicate instances may be left separate conservatively or combined under a proved common norm envelope.

Frozen original A/B/F paths are never enlarged/rekeyed to make an inequality pass. Failure of a mixed inequality is a missing admission certificate for that path, not a reason to introduce homomorphism or a new commitment.

## 9. Full resource-conditioned Module-SIS theorem

This section records hypotheses needed from the lighter probabilistic extractor, rather than citing an expanded local Akita extraction theorem to establish them.

For nonterminal j let `b_j=1+sum_g W_(j,g)` where `W_(j,g)=P_(j,g)*B_(j,g)` counts protocol shell coordinates, not support weight, A width, chunk count or source digit depth. With s_str,j whole-strong-source probes and s_pre,j incoming-prefix probes, the supplied **unconditioned** expected-work recurrence is

`C_str,j <= (1+s_str,j)*C_(j+1)+s_str,j*q*T_C+A_str,j`,

`C_raw,j <= b_j*C_str,j+(b_j-1)*(Q_FS+1)*T_cheap,j+A_shell,j`,

`C_j <= (1+s_pre,j)*C_raw,j+s_pre,j*q*T_C+A_pre,j`.

Here q bounds original FS candidate sites, Q_FS counts original FS-namespace exposures (all failed nonce probes and allowed initialization's actual FS calls), and T_C includes complete original replay with all setup work. Setup-recompute calls are excluded from q/Q_FS for FS forking but not from work. Actual sampler/operator predicates and the indexed grammar determine these counts; GrindingPlan's logical entries are not an adversarial Q cap.

The leading coefficient is `lambda_j=(1+s_pre,j)*b_j*(1+s_str,j)`. With the actual direct terminal star and any genuine incoming terminal cut,

`Lambda_light=(1+s_pre,T)*b_T*product_(nonterminal j)lambda_j`.

In the intended whole-DAG case s_str,j=1; s_pre,j may be zero or one only when its separate tuple contract allows. Terminal is not another strong block. This product remains. Polynomial local factors do not imply a polynomial product. Add the sealed-query all-original-sources probe and actual frontend probes/reconstruction to get total W. Success-weighted/marked-original hypotheses must justify these unconditioned costs; no hidden unpriced `1/p` rare-branch search is discarded.

Charge W for:

- complete adversary/prover/verifier and permitted oracle-dependent initialization, including failed nonce trials;
- every original/probe/cheap replay, tape/state clone and saved oracle table;
- full source/point reconstruction and decoding, physical response cells, source B/F paths, canonical setup references and all integer unit/norm/range/predicate checks;
- every actually exposed or conservatively reserved raw-family byte, conditional sample, rejection, exhaustion test, lookup, copy and serialization;
- this joint setup raw-page simulation, all setup recomputations, full registry construction and complete footprint;
- all retained shell/source-cut invocations and frontend/query/history output size;
- all comparisons, collision attempts, native ring computation and final collision output.

Define the complete almost-surely terminating ideal extractor with finite **unconditional** expected charged work W. Stop the entire procedure, including setup, before the next charged operation exceeds a single global budget L. Markov gives

`Pr[work>L]<=W/L`.

For each named I, algorithm `B_(I,L)` embeds A* by §§2–3, builds the same registry and installation by §5, simulates the rest of the bounded extractor with lazy random coins, and outputs only if its first label is I. This is an ordinary randomized original-ring Module-SIS algorithm; all independent ideal namespaces are simulated by coins and tables, with setup programming determined by A*. It has no auxiliary lattice oracle.

Let T_I(L) be a justified worst-case **bit-time** bound including challenge parsing, lookup/data-structure overhead, setup/registry operations, the entire bounded producer and all private work, and final collision output. It is polynomial in L and parameter/input/output length under ordinary bounded-size table implementations, but is not automatically numerically equal to L if the charged units omitted simulator overhead.

Define `Adv_MSIS_I(t,eta)` as the supremum success probability of ordinary algorithms of time at most t against the specified uniform original ring instance and actual norm. This is a mathematical definition, not a required software API. A global single cutoff and first-label union bound give

`Pr[original accepts and no original relation witness]`

` <= kappa_stat(Q_FS,horizon)+Delta_sampling+Delta_setup`

`    +W/L+sum_(I in inventory) Adv_MSIS_I(T_I(L),eta_I)`.

Under the selected declared model B and exact rejection simulation, **Delta_setup=0**. `kappa_stat` is the independently proved original-event shell/source-cut/query/frontend ledger, including its proper parsed old-key ownership. `Delta_sampling` contains only actual sampler deviations/failures not already charged there. Do not count the global cutoff once per instance or private fork. Repeated collision attempts already inside T_I(L) need no additional unexplained attempt multiplier.

This is the required full runtime-conditioned form. The lattice estimator's modeled ROP floor and modeled attack success probability do not imply `Adv(t)=t/2^128`, nor any bound against arbitrary algorithms at extractor time. No new negligible-at-128 cutoff requirement is imposed.

## 10. Qualitative PPT soundness without a numerical Adv API

**Yes, in this declared base model, under these explicit asymptotic conditions.**

1. A security-parameter-indexed field/ring/row family has total explicit instance/witness/history horizon, setup/registry, comparison inventory and oracle-answer lengths bounded by polynomials in the size parameter against which base hardness is asserted. Ordinarily these are polynomial in lambda. If n is allowed to grow superpolynomially in lambda, a different resource-quantified hardness statement is needed; “polynomial in explicit n” alone is insufficient.
2. Setup is the already declared model B. All permitted setup-dependent initialization/exposure is part of the modeled execution. The installation is sampled once and reused, not conditioned on arbitrary fixed realized concrete bytes or arbitrary advice depending on them.
3. W has a fixed polynomial upper bound for every PPT adversary in scope. This includes Lambda_light, complete reconstruction/output, conditional sampler work and initialization. Finite W alone, or polynomial local work without a bound on the retained product, does not suffice.
4. The independently proved statistical/sampling errors are negligible over this family. Actual degree/horizon limits, filter acceptance and finite-family laws hold; the inventory is polynomial and original Module-SIS hardness covers its actual rings/dimensions/radii.
5. All required source/GRID/LOCAL/frontend/point/unit/owner and original-output extraction contracts hold for the selected branch. This certificate supplies joint setup and collision algebra, not those separate theorems by assertion.

Proof: if a PPT adversary violates the relation with nonnegligible probability, choose a polynomial p0 such that on an infinite subsequence its violation after negligible statistical terms exceeds `1/p0(lambda)`. With `W<=w(lambda)`, choose a polynomial cutoff such as `L=4*w(lambda)*p0(lambda)`, losing at most `1/(4*p0(lambda))`. The truncated process still produces an original-family collision with nonnegligible probability. A polynomial-size first-label inventory leaves a nonnegligible one-instance success (or guess the target label before embedding, losing only that polynomial factor). This gives a strict PPT algorithm contradicting original Module-SIS at the recorded actual parameters. A growing uniform inventory needs the corresponding uniform guessing/instance-family assumption; arbitrary nonuniform per-lambda target labels are not silently hardwired into a uniform reduction.

The cutoff may depend on the alleged adversary and inverse nonnegligible violation, as appropriate for this contradiction. A **single fixed** polynomial L and a mere first-moment bound do not generally give negligible cutoff failure for a universal strict extractor. That stronger extractor-format claim needs a tail theorem, but is unnecessary for qualitative soundness by the above argument.

A lone finite q128 row is not an asymptotic family. For that row §9 remains an exact finite conditional inequality, but numerical nonvacuity needs the user's justified actual-resource hardness policy. No numerical Adv API is needed for the qualitative theorem; absence of such an API does not refute it. Conversely ADPS16 ROP128 alone does not supply a finite advantage term.

## 11. Remaining selected-profile data, not a new setup assumption or bit target

**Resolved here:** the indexed declared base already includes the setup random-oracle model, so model B is selected; the actual page/canonical-rejection conditional embedding supplies its implementation-specific joint simulation with zero coefficient bias. No further setup-hardness assumption or seed-preimage mechanism is missing for that theorem. Official-body authentication remains a provenance caveat, not a mathematical step delegated to an expanded local theorem.

The smallest remaining admission inputs are:

- **One actual Candidate1 final row and installed slot profiles/provenance**, enough to fill §8 with source-reference alphabets, all reachable mixed pairs, selected capacities and exact view footprints. Frozen Main/RAM producer profiles alone do not determine consuming response certificates. A row using direct setup omits canonical-reference comparisons because it actually selects no offload, not by ignoring a selected group.
- **The complete lighter-extractor resource/family certificate:** a polynomial W bound for qualitative soundness, or a declared finite L and justified T_I(L) for a concrete statement. Do not infer polynomiality from local factors alone. A useful finite numerical claim needs justified bounds on the abstract Adv terms, not a speculative ROP-to-Adv formula or an extra 128-bit target.
- **The separately owned implementation/extraction gates:** the AQRA indexed full-answer/descendant-history boundary and actual selected source/GRID/LOCAL/frontend/original-point contracts. The known legacy transcript incompatibilities are not repaired by this setup reduction. Actual physical padding and the current zero-point slice stay as specified, not strengthened to honest-buffer constraints.

Under those conditions, shared setup correlation, canonical setup-source authenticity and ordinary original-ring collision reduction are compatible with Candidate1. No source homomorphism, per-segment PCS proof, fresh commitment family, extension E, or stronger setup assumption is needed. A concrete fixed-realized SHAKE claim would remain a distinct model-C statement, deliberately outside this approved base-model result.
