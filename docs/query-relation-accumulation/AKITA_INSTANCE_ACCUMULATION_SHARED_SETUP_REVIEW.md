# Compact native setup reuse: conditional security certificate

## Decision and invariant

**CONDITIONAL ADMIT for fresh provisioning; reject a silent replacement of installed keys.** The proposed coefficient-wise, row-major prefix views are exactly the views covered by Akita Lemma 10.11 and Corollary 10.12. Their collision security reduces to the listed ordinary Module-SIS instances; no new generic shared-key assumption is needed. Matrix independence is not used by the deterministic source-uniqueness or backward weak/strong extraction arguments. The remaining conditions concern the exact setup experiment, complete accepted view/occurrence inventory, original-source identities, and the separately conditional receipt/Fiat–Shamir contracts.

The invariant is acceptance under each source's original installed setup, semantic role, physical geometry, decoder, supplied noncanonical decommitment and continuation history. This audit changes no response envelope, source dimension, scalar-field policy, shell, range/binary/zero/relation check or terminal equation. It adds no setup offloading. In particular it does not turn a full compressed commitment into TerminalInnerState, and does not identify distinct sources merely because their matrices or payload bytes coincide. Fresh initialization, original application source creation, receiver creation and every later source must use the selected installation from the outset. Independently provisioned frozen commitments remain under their actual keys.

**Scope:** research only. No repository edits, builds, tests, formatters, executions, numerical receipt-security claim or parity claim. The root is not a Git working tree; there is no candidate code diff to attach bug findings to. The result audits the stated proposed setup convention rather than certifying a patch.

## Evidence and provenance

Primary paper text was retrieved through qmd, document `lattice-papers/akita.md`, docid `#bfa1a6`. Numbering below is that indexed text, not a claim about an independently fetched publication revision.

- [Akita indexed text](SOURCE_PROVENANCE.md#akita), lines 1733–1767: finite maximum view, power-of-two provisioning envelope, Eq. (66) coefficient addressing, Eq. (67), setup sampling and setup/FS domains.
- Same, 1875–1927 and 1977–2025: sliced B, whole-stack compression, complete path binding and Lemma 4.3.
- Same, 3623–3697: source-comparison classes, mixed envelopes (193)–(196), full inventory, exact single-view MSIS embedding and adaptive finite-family reduction.
- Same, 3709–3825: algebraic/noncanonical source relation, weak products, Lemma 10.15 and source-only Corollary 10.17.
- Same, 3875–3989: actual terminal base case, terminal radius, descendant-certified tree and Theorem 10.23 backward extraction.
- Same, 4181–4231 and 4291–4331: complete-tree composition, Eq. (229), page-oracle simulation and adaptive-family Eq. (234).
- `AKITA_INSTANCE_ACCUMULATION_MATRIX_REGISTRY.md`, §§1–5: all 33 role dimensions, alphabets, radii, counts and original-source restrictions.
- `AKITA_INSTANCE_ACCUMULATION_COMPACT_PROFILE.md`, §§1–4: T<=16 geometry, full all-valid-source bounds, unfiltered shell, three complete final reductions and direct terminal.
- `AKITA_INSTANCE_ACCUMULATION_FIELD_POLICY_COMPARISON.md`, §§2–3 and §7: fixed K, degree-one specialization, only partial planes/D widths shrink, and current setup mismatch.
- `AKITA_INSTANCE_ACCUMULATION_RECEIPT_SETUP.md`, §§2–7, and `AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md`, §§3–7: authenticated installation/reference, independent outer setup, NGP game and same-execution composition.

Immutable source references use Akita `83574331d4e51f8cce1d8689d05592fa4ef4c138` under `/tmp/akita-extension-audit-c6qh69gy/akita/83574331d4e5` and Jolt `0791f79a55c3fadc3d327c78a72228696c338468` under `/tmp/akita-extension-audit-c6qh69gy/jolt/0791f79a55c3`. These are not the current FullRing installed-role extensions.

## 1. Exact target-matrix embedding

Put K=F_q, q=2^128-2^32+22537. For a numerical view I=(d,n,m), let L_I=n*m*d and define

M_I(S)[a,b]=sum_{j=0}^{d-1} S[(a*m+b)*d+j] X^j in K[X]/(X^d+1).

Every view starts at coefficient zero. The map from the n*m*d matrix coefficients to the first L_I flat coordinates is a bijection: coefficient index t decomposes uniquely into j=t mod d, b=floor(t/d) mod m, a=floor(t/(m*d)). This is not a top-left crop of a single largest shaped matrix when widths differ. Each view must reshape the flat prefix using its own row stride.

Fix any target I and an ordinary uniformly random MSIS challenge M*. Copy its coefficients into those first L_I positions in exactly that order; sample every remaining coordinate of S independently uniformly. For every s in K^N, the probability of producing s is q^(-L_I)*q^(-(N-L_I))=q^(-N). Hence S is exactly uniform, not merely correct in its I marginal. Derive **all** A/B/D/F/H views from this same S. Every intended coincidence and partial overlap, including cross-ring d=128/16/8 reinterpretations, is preserved automatically. A collision output for I is already a nonzero kernel vector of M*, with unchanged dimension, norm and radius. No inter-ring homomorphism or norm conversion is needed.

Run the whole adversary and extractor, retaining an output only if it names I. Sum these probabilities over the complete collision inventory. This proves the specialized Lemma 10.11 bound. If schedule choice depends on observed setup, first fix the finite admissible family independently of setup, embed each possible target, and run that choice unconditionally. Corollary 10.12 applies without conditioning S on the chosen schedule.

An implementation of the simulator also generates public setup-dependent references/preparation by their actual efficient algorithms, and charges this work. An authenticated reference is not arbitrary correlated advice containing a trapdoor. Independent outer setup can be sampled separately and C_static constructed normally after the inner challenge is embedded.

Source evidence: pinned `akita-params/src/layout/flat_matrix.rs:60–135` takes precisely the first rows*cols*d field elements; the dynamic-view documentation states the same element index. Pinned `akita-types/src/proof/setup.rs:222–250` explicitly omits ring dimensions from expansion and preserves field-length prefixes. The paper explicitly includes compression views under the same rule (1753,1777,1955).

## 2. Complete compact numerical-view inventory

The 33 **semantic matrix roles** remain. They become 12 distinct numerical matrices for either fixed K1 or fixed E2 profile. A numerical collision inventory may merge identical matrices, keeping the largest applicable radius, or retain safe duplicate role entries. It must not merge their protocol/source identities.

### 2.1 A views (identical for K1 and E2)

| View (d,n,m) | Semantic roles | Prefix K coefficients | Required max coefficient radius | Safe audited upward radius |
|---|---|---:|---:|---:|
| (128,4,1024) | ReceiverRoot.A | 524288 | 234880800 | 260046724 |
| (128,3,512) | Main.A, Final1M.A, Terminal512K.A | 196608 | 7339808 | 8126340 |
| (128,3,832) | RAM.A | 319488 | 7339808 | 8126340 |
| (128,3,1024) | Final2M.A | 393216 | 7339808 | 8126340 |

ReceiverRoot and Final2M are not identical matrices: the latter is the first three rows of the former at this common width. RAM.A and rank-one width-832 D, when present, likewise share a first row but remain different MSIS instances. These overlaps are handled by the embedding, not by deleting an instance.

Keep source geometries: Receiver P1/B32/M1024/delta1; Main P56/B1/M512/delta1; RAM P1/B16/M32/delta26; Final2M P1/B16/M1024/delta1; Final1M P1/B16/M512/delta1; Terminal P1/B8/M512/delta1, inner-only. Same A width does not identify these different source families.

### 2.2 B/D views

All rows have d=128, n=1 and coefficient collision radius 31 (the enforced alphabet [-16,15], not an honest concentration bound).

| Width | Roles, E2 | Prefix coefficients |
|---:|---|---:|
| 3328 | ReceiverRoot.B | 425984 |
| 4368 | Main.B | 559104 |
| 1248 | RAM.B, Final2M.B, Final1M.B | 159744 |
| 6240 | Update.D | 798720 |
| 1664 | FinishRoot.D | 212992 |
| 832 | Finish2M.D, Finish1M.D | 106496 |

For K1 replace only the three D widths/prefix lengths: Update.D=(3120,399360), FinishRoot.D=(832,106496), Finish2M.D=Finish1M.D=(416,53248). The three B rows are unchanged. The rank-one width-832 view occurs under different D role labels between the two field profiles; this does not authorize interchange of their partial semantics.

If both field profiles are actually selectable in one accepted setup family, the B/D width union is {416,832,1248,1664,3120,3328,4368,6240}: eight matrices, radius 31 each. With four A and two compression views this is **14**, not 12. If the two profiles are separately fixed before provisioning, each uses its own 12-view inventory. Scalar policy itself is not selected or altered by this audit.

### 2.3 F/H coincidences

For each of the five original B/F chains (ReceiverRoot, Main, RAM, Final2M, Final1M) and each of the four D/H chains (Update, FinishRoot, Finish2M, Finish1M):

| View | Coincident roles | Prefix coefficients | Radius |
|---|---|---:|---:|
| (16,1,1024) | All five F first stages and all four H first stages | 16384 | 1 |
| (8,1,256) | All five F second stages and all four H second stages | 2048 | 1 |

Thus eighteen matrices become two numerical matrices, **not two protocol path occurrences**. Every occurrence retains both relation rows, recompositions and full negative-binary digit spans. Cross-occurrence alphabet diameter remains one. No F path is dropped because an H matrix has equal coefficients; the terminal still has neither its own F nor H, while all predecessor paths remain.

### 2.4 Sliced B

The physical MSIS width is P*ceil(B/S_B)*n_A*(d_A/d_B)*delta_open, not the total stacked source length and not S_B times that width. All displayed compact B widths equal P*B*n_A*26, so this fixed inventory uses S_B=1. If a genuinely admitted occurrence uses slices, the certificate must retain its exact slice count, polynomial-major per-slice padding and complete stacked image. A differing stacked input has a nonzero difference in some slice; restrict to that slice and zero-pad to m_B. This yields the single B kernel vector at the same radius. No independence between slices is required or available. Changing slice counts can change B/F widths and requires extending the inventory, not silently reusing this 12-view count (Akita 1875–1927,2021–2025,3665).

## 3. Mixed occurrence radii, including terminal/nonterminal reuse

For any legitimately compared source occurrences u,u', retain unit/product certificates delta*s=r and delta'*s'=r'. Their collision is

v=delta'*r-delta*r'=delta*delta'*(s-s'), Av=0,

and its coefficient norm is at most k'*b+k*b', where k=||delta||_1 and b=||r||_infinity. Units, not a small inverse norm, guarantee v nonzero when sources differ.

Nonterminal weak profiles have k=112 and b=32767 (three response digits), except ReceiverRoot b=1048575 (four digits). Terminal weak profiles have k=112 and b=14336 (twice accepted response bound 7168). Strong/reference profiles already proved bounded have (k,b)=(1,B_s), with B_s=1 for Main and 16 for other compact sources.

| Pair | Bound |
|---|---:|
| Receiver weak/weak | 224*1048575 = 234880800 |
| Receiver weak/strong | 1048575+112*16 = 1050367 |
| Three-digit nonterminal weak/weak | 224*32767 = 7339808 |
| Main weak/Main strong | 32767+112 = 32879 |
| Other three-digit weak/strong | 32767+112*16 = 34559 |
| Terminal weak/weak | 224*14336 = 3211264 |
| Terminal weak/strong | 14336+112*16 = 16128 |
| Formal three-digit/terminal weak mixed envelope | 112*(32767+14336) = 5275536 |
| Two bounded sources at bound <=16 | <=32 |

The numerical (128,3,512) target must therefore carry radius 7339808 (or 8126340 upward), not terminal-only 3211264/4063108. Even the formal nonterminal/terminal mixed envelope is dominated. Crucially, the envelope calculation is **not** a source-compatibility theorem: OuterPayload and TerminalInnerState remain different modes and no cross-role comparison or substitution is authorized. All actual comparisons are componentwise under an equal original projection and compatible source shape/path. Main/Final1M/Terminal do not become one semantic source class merely because their A matrix coincides.

B/D mixed alphabet bounds remain 31 because every relevant carried digit is certified in [-16,15]. F/H mixed bounds remain 1 because their entire inputs are certified in {-1,0}. All physical padding/completion cells that enter a map must satisfy the actual certified alphabet and any specified zero rule. No canonical-decomposition premise and no honest-only narrowing is introduced.

D/H agreement is required across coordinate-fold siblings where that opening payload was fixed before folding. It is not required across different query/range/source-prefix branches of one original commitment: those branches can change point, values and D/H data. Corollary 10.17 compares A/B/F source projections alone.

## 4. Why independence is unnecessary in source/strong extraction

The relevant arguments are deterministic for fixed public matrices:

1. From equal final payloads, walk backward through the complete compression chain. The last differing digit vector yields an F/H collision; if all chain digits agree, a differing original input yields B/D collision, with slice restriction if necessary (Lemma 4.3).
2. If source paths agree but source blocks differ, clear the two unit denominators to obtain the A collision above (Lemma 10.15).
3. Across different opening profiles, use equal **original source projection** and ignore opening-local D/H state; compare at the maximum mixed profile radius (Corollary 10.17).
4. Below a fixed folding child, use descendant-certified source agreement to recover exact range, norm, binary, zero and fused-relation facts. Do this independently below each child; later challenge values need not coincide. Then compare fixed paths and subtract coordinate-wise fold identities to obtain incoming weak products (Theorem 10.23).
5. The direct terminal's predecessor-bound A images supply the base case. Its full partial/ring/scalar equations and accepted-response bound remain unchanged (Proposition 10.20; compact full-ring specialization).
6. Iterate along the complete assembled continuation/history to recover strong receiver bounds; never infer them by dividing the short response by a unit. Counterfactual descendant authentication and all extraction work remain required (`RANGE_REUSE` §§5–6).

None of these steps samples another matrix independently or conditions a challenge on matrix independence. They output a named concrete per-view collision when uniqueness fails. The shared-prefix reduction handles that output after the whole tree. This is an either-or proof, not an unjustified information-theoretic assumption that commitments have perfectly unique sources. Challenge-oracle independence/conditional-tape requirements are separate and remain in force.

## 5. Setup size and actual sampler bias

### 5.1 Coefficient arithmetic

Recomputed independent-role sums:

- A: (4*1024 + 3*512 + 3*832 + 3*1024 + 3*512 + 3*512)*128 = 1826816.
- B: (3328+4368+3*1248)*128 = 1464320.
- E2 D: (6240+1664+2*832)*128 = 1224704.
- F/H: 9*(1024*16+256*8) = 165888.
- Total E2: **4681728**. K1 halves only D's contribution, giving **4069376**.

For extension degree k in {1,2}, the compact maximum used prefix is

L_max(k)=max(4368*128,3120*k*128)=max(559104,399360*k).

Thus E2 uses at most **798720** K coefficients (Update.D), and K1 at most **559104** (Main.B). At 16 canonical bytes/coefficient these used prefixes occupy 12779520 bytes (12.1875 MiB) and 8945664 bytes (8.53125 MiB), respectively, before descriptors/preparation.

**Do not conflate used prefix with provisioned envelope.** Paper Eq. (65) selects N_setup=2^ceil(log2 L_max); for either compact field profile this is **1048576 coefficients / 16777216 bytes (16 MiB)**. Native exact-prefix storage permits an exact requested length, but a claim to use the paper's rounded provisioning law must record this rounding. No setup-offloading source is implied by either length. If the accepted control/outer catalogue requires larger views, its actual maximum governs instead; the compact maximum alone does not certify the whole pair's provisioning capacity.

### 5.2 Uniform/modular law versus actual native law

For independently uniform flat K coefficients, Delta_setup=0. For independent w-bit raw words reduced modulo q, with r=2^w mod q, the exact one-coordinate statistical distance is epsilon_w=r*(q-r)/(q*2^w), and a sufficient joint bound is min(1,N*epsilon_w) <= N*q*2^(-w). Count unique flat coefficients, not sum of view usages. N is the certified provisioning/revealed-prefix envelope of the setup experiment; Eq. (65) safely uses N_setup. A new 512-bit modular installation would therefore use this formula (and N_setup*q*2^-512 < 2^-364 for compact N_setup=2^20), but **that is not the pinned native sampler**.

Pinned source uses `PublicMatrixDerivation::Shake256PagedV1`, 4096 accepted field coefficients per page (`akita-types/src/proof/setup.rs:18–33,222–250`). The page input length-prefixes domain, derivation, page size, 32-byte seed, field modulus and page index (`:315–334,367–372`). It does not absorb role or ring dimension. It calls the actual F::random sequentially per page. Pinned Jolt fp128 `solinas/fp128.rs:1088–1094` calls `sample_uniform_below(P,128)`; `solinas/mod.rs:71–100` reads 16 little-endian bytes and rejects every candidate >=q. Therefore, in an ideal independent-page byte-stream model, each accepted coefficient is exactly uniform and the joint accepted sequence is independent: **coefficient-sampling bias is zero**, not a 512-bit modulo-reduction error.

Rejection probability per draw is (2^32-22537)/2^128 < 2^-96. It affects expected expansion work, not accepted-output bias. A future bounded-attempt sampler would need explicit abort/tail accounting; no such cap or fallback is silently imposed here. Expected raw-byte work for N accepted coefficients is 16*N*2^128/q, plus page framing. This is not measured running time.

The pinned default `sample_akita_setup_seed` is a **fixed** seed, with its first eight bytes set to little-endian DEAD_BEEF_CAFE_BABE and remaining bytes zero (`setup.rs:214–220`). Its function name is not evidence of random setup entropy. Exact field sampling from ideal XOF pages and concrete SHAKE at this fixed public seed are different security claims.

## 6. Four separate security layers

### A. Uniform flat setup theorem (proved specialization)

Section 1 is an exact reduction from a uniform S. It preserves the complete correlated view distribution and all finite-family outputs. Per-view MSIS advantages use the **whole** adversary/extractor runtime plus setup, collision detection and embedding overhead. Repeated RAM handles, segments, fork branches or compression uses do not create new matrices, but their attack/replay work is not divided by the number of distinct views.

### B. Ideal public seed/page oracle (conditional model with explicit simulator)

Akita Eq. (229)'s proof explicitly requires lazy consistent setup answers, including page queries, while FS remains an independent namespace (qmd 4223). Supplying an embedded S while leaving recomputation queries answered by unrelated concrete SHAKE would fail this requirement.

For this exact rejection sampler, even a raw-byte page interface can be simulated consistently: conditional on each desired accepted coefficient x, sample the number of rejected candidates with the geometric law of success q/2^128, sample those rejected values uniformly from [q,2^128), then append canonical LE128(x). Concatenate these blocks per page and keep an independent trailing stream. Because acceptance count/rejected values are independent of the accepted uniform x, averaging over the target's uniform coefficients reproduces the original ideal byte-stream law. Cache every page and every prefix, handle setup queries from included auxiliary generation before application commitments, and answer other namespaces independently. Expected work is efficient; strict cutoffs, if used, incur their actual tails. This supplies the specific programming interface; it is not a concrete SHAKE programming algorithm.

The setup domain must be injectively disjoint from native FS and from independently provisioned outer setup/FS. Setup recomputation queries are not FS forking queries, but their runtime and any setup-selection/grinding experiment are charged. An adaptive choice of schedule from a fixed family is covered; an arbitrary adversarial search over setup seeds is a different provisioning game and is not covered by silently calling the seed fixed.

### C. Concrete setup and concrete FS (not proved here)

A fixed concrete public-seed SHAKE expansion is a deterministic, efficiently recognizable family, not statistically close to a uniform high-dimensional S merely because its modular sampler is exact. With a fixed default seed it is one fixed vector; even a random 256-bit public seed has tiny output support relative to K^N. A public-seed PRG indistinguishability slogan cannot justify replacing this joint distribution while the adversary recomputes the public function.

Two admissible conditional routes remain distinct:

- Sample actual flat coefficients uniformly (or with an explicitly analyzed private-randomness/CSPRNG generation law), authenticate them as installed data, and use the uniform reduction. This needs a source integration route accepting precisely that installed data; it is not evidence that stock seed-validation accepts arbitrary arrays.
- Retain the native public-seed/page convention and state its setup-XOF idealization/instantiation premise explicitly, preserving the consistent page experiment above. Concrete FS likewise retains the existing native/outer instantiation premises. No new generic shared-key assumption is necessary or supplied.

`PROTOTYPE_MODEL:83–101,116–160` currently fixes 33 independent inner maps and 512-bit provisioning in the pre-provisioning descriptor J, and averages over I<-Provision(J). A shared-prefix installation changes J and that provisioning law. D1-NGP must be stated for the **new exact** game and derived archive producer; it is not automatically inherited from the old independent-role declaration. Its resource translation and nontrivial instantiation loss are not known. This is distinct from proving the shared-prefix MSIS lemma. The hypothesis grants native language soundness only, not native knowledge or generic recursion.

### D. Authenticated C_static reference (unchanged mechanism, new contents)

A new shared installation changes static bytes/descriptors and must be authenticated as such. Freeze the actual independent outer key and original advice source layout, generate the new reference commitment, and install the expected C_static externally. Do not feed C_static back into its own key derivation. A profile/seed/digest label alone does not authenticate the arrays, and a hash check is not the original lattice source comparison.

The guest must read precisely those bytes through the original trusted-advice source, reconstruct canonical K coefficients and approved matrices/views, verify exact lengths/padding and retain the complete archive/final verifier. All consumed-byte/source/guest adequacy certificates and the complete ordinary outer proof remain required. Compare the extracted weak outer advice source with the canonical installed reference under the same original A/B/F shape, using envelope b_weak+k_weak*B_reference and all B/F cross-profile alphabet diameters. Those are **outer** instances at outer dimensions, not one of the 12 compact inner views. The concrete source gadget's exact positive/negative reach, including u64 advice encoding, must be certified; do not assume canonicality of the extracted source or equality of opening-local D/H data.

`RECEIPT_SETUP` §§2,4–6 and `PROTOTYPE_MODEL` §7 explicitly require independent inner/outer provisioning, authenticated provenance, original C_archive/C_static linkage, complete outer frontend/PCS extraction and deterministic guest projection. Shared numerical maps within the inner setup do not authorize sharing the inner setup with the outer advice key. A smaller static blob may change capacities and the resulting outer finite schedule; that must be re-admitted, not guessed from the inner coefficient savings. No receipt numerical strength follows from this audit.

## 7. Finite-family inventory and exact admission obligations

Let S be the setup-independent finite family of schedules actually accepted. For compact-only fixed E2 (respectively K1), record all 33 semantic role descriptors and route every potential collision to the 12 numerical targets above. For a selectable K1/E2 family, use their 14-target union and the maximum applicable occurrence radii. Repeated T<=16 updates do not add numerical targets. Retain role-labelled duplicates if simpler; every entry must still cover its complete occurrence/reference profile and runtime.

For the accumulation/control pair, the actual family is the union of accepted compact accumulation schedules and accepted full-PCS control schedules. Add every control root, recursive and terminal view with its own shape/slices/path/radius; the control is not automatically the compact finalizer. For a receipt add the separate outer catalogue, all archive/static source comparisons, canonical references and any offloading sources **already required by that ordinary catalogue**. This audit adds no offloading. The full N_view is max(n*m*d) over the relevant provisioned family, not necessarily the compact value. No unspecified control/outer catalogue is certified by the compact 12 count.

An installed certificate must fix, before challenge sampling:

1. Setup/profile version, q, actual flat law/seed and generation provenance, exact coefficient order, maximum supported prefix/envelope and field/ring encoding. Validate actual materialized setup against its claimed law or authenticate actual arrays; neither unauthenticated arrays nor a seed label suffice.
2. The role-to-numerical-view map plus the distinct semantic role, P/B geometry, source decoder, mode tag, original handle, slice/padding map, F/H chain and K1/E2 partial ordering. Shared coefficient identity is not source substitutability.
3. Every accepted source occurrence and compatible weak/strong/reference comparison pair, full enforced alphabets, unit/product envelope and maximum mixed radius. All accepted alternate schedules/profiles belong to the fixed finite family.
4. Complete collision routing: A, slice-local B, unsliced D, both F/H stages and terminal A. Alias numerical collision identifiers only after preserving semantic validation. Charge all repeated occurrence work to each relevant ordinary-MSIS solver; do not multiply/divide runtimes by the view count without a proved bound.
5. Unchanged full all-valid-input completeness and continuation/final obligations: preserve supplied noncanonical paths and permitted fillers, full norms/encodings, physical holes/zeros, all strong reductions, predecessor-bound terminal state and every final check.
6. A new installation and source creation from outset; no reinterpretation or recommitment of old independent-key handles, no setup switch mid-run. Authentic C_static and the NGP provisioning descriptor must correspond to this exact new installation.
7. Explicit separation of uniform setup theorem, ideal page/FS experiment and concrete setup/FS assumptions. Inner/outer setup independence remains an obligation. Existing finite-policy and whole-extractor runtime/query/cutoff bounds must be instantiated for this game rather than inferred from honest costs.

Subject to these obligations, the algebraic/setup theorem obstacle to original native prefix reuse is resolved. The candidate is **not rejected for matrix correlation**. Deployment, full native/control integration, all-valid-source APIs, conditional receipt admission and concrete security remain separate gates.


---

# Shared-prefix certificate: source dispatch and later receipt-policy addendum

This is part of [the main conditional security certificate](#compact-native-setup-reuse-conditional-security-certificate). Novel calculations, reductions and simulators in that certificate are **[INFERENCE — explicit mathematical derivation]**, not executed tests or measured behavior. Source facts below were inspected read-only. The two corroborating delegated source/receipt investigations are available at `agent://ReviewSharedSetupReduction.NativeSetupEvidence/report` and `agent://ReviewSharedSetupReduction.ReceiptSetupObligations/report`.

## 1. Original consumers preserve the exact view, not just its producer

All immutable Akita paths below are beneath `/tmp/akita-extension-audit-c6qh69gy/akita/83574331d4e5/crates`.

- `akita-cpu-backend/src/arithmetic/prepared.rs:272–298` prepares `shared_matrix().ring_view::<D>(1, num_ring_elements)`: a flat ring-element prefix rather than a matrix with an inherited maximum row stride.
- The actual multiplication dispatcher in `akita-cpu-backend/src/kernels/linear/ntt_matvec.rs:3–30` constructs row i as `neg[i*nc..(i+1)*nc]` for the requested `nc`, including the q128 branch. Therefore the local-width row-major interpretation survives the preparation/dispatch boundary. Its caller at `:116–139` passes the requested row and column counts.
- `akita-cpu-backend/src/arithmetic/compression_cache.rs:114–132` constructs each compression map with `expanded.shared_matrix().ring_view::<D>(1,input_width)`. Thus F/H are also real shared-prefix consumers; the theorem is not relying on a hypothetical extension of an A/B/D-only convention.
- `akita-params/src/layout/setup_envelope.rs:12–16,43–109` explicitly uses the maximum of complete commitment and scheduled SIS footprints. It is not a sum of independent roles and not `max_rows*max_columns*max_degree`. The pinned exact-capacity API does not round automatically to the paper's power-of-two Eq. (65) envelope. Both exact compact prefix lengths in the main certificate are therefore meaningful source-level capacities, while 2^20 is the separate paper provisioning convention.

Each degree d remains its own native negacyclic ring. Reblocking coefficient storage across d128/d16/d8 is not asserted to be a ring homomorphism. The target MSIS embedding needs none: it embeds the target matrix's coefficients bijectively, then runs each consumer under its own actual ring action. This explains why mixed degrees do not require a new assumption or a norm conversion in the collision output.

## 2. Current installed-role machinery is not upstream shared setup

Current paths are relative to the repository root:

- `akita/crates/akita-types/src/proof/setup_roles.rs:699–805` stores a role map of independently provisioned literal arrays; `coefficient_count` is explicitly a **sum, never maximum**. `from_provisioned` checks inventory/validity/shapes, and explicitly leaves authentication, randomness, independence and the security certificate to its caller. `AkitaSetupIdentity::{SharedPrefix,FullRing}` distinguishes the laws rather than aliasing them.
- `akita/crates/akita-cpu-backend/src/arithmetic/prepared.rs:29–45,106–185` routes `SharedPrefix` to an expanded flat setup and `FullRing` to a named role's literal matrix. Its `expanded()` rejects FullRing; cache identity retains `(installation, role)` and checks installed degree/capacity. Equal new numerical views are not currently evidence that arbitrary installed role arrays equal native prefixes.
- The delegated source audit traced `akita/crates/akita-types/src/full_ring/relation.rs:15–94,862–885`: literal coefficient access uses `(row*columns+column)*degree+index`, and relation rows use the native signed negacyclic wrap. These consuming maps are compatible with prefix arrays **if and only if the supplied arrays actually equal the prescribed prefixes in this order**. No implementation conversion is claimed.

The numerical sharing certificate permits retaining all semantic role tags while selecting coefficients through one backing vector. It does not mandate erasing current role validations or replacing one enum variant with another without integrating its consumers. Conversely, the current sum-based provisioning machinery is not evidence that the original native theorem requires independent roles.

## 3. Frozen imports do not license a key change

Inspected current `akita/crates/akita-cpu-backend/src/opaque/owned_commit.rs:18–99`:

1. Import preserves the immutable source storage rather than decoding/redecomposing it.
2. It checks the frozen group layout and, for FullRing sources, the installed raw source view under the original source ID.
3. It recomputes under the receiving setup and compares **both public commitment and retained material**.
4. It rejects mismatches before issuing new ownership/lineage.

This is evidence that current import is setup-sensitive, not a migration mechanism for independent keys. Passing recomputation for one particular source would not establish equality of the old and new matrices on every source. The target proposal should use fresh initialization and original source creation under the shared setup, not weaken these checks or transplant old independent-key handles. Existing frozen objects may use only their actual installed setup.

## 4. Later outer design must not be silently rewritten

The main certificate cites the conditional ordinary native outer option in `RECEIPT_SETUP` §§4–7. There is a later selected research design, `AKITA_INSTANCE_ACCUMULATION_OUTER_CARRIER.md`, whose §8 (`:340–358`) deliberately specifies **independent outer role namespaces**, direct matrix evaluation, and **no PCS setup offloading**. Its single uniform vector is **partitioned**, not overlapped as Eq. (66) prefixes. Its §9 (`:360–372`) has a finite, deterministic candidate generator/validator; §10 (`:374–397`) counts **4G+4 outer matrix roles**, not inner33, and charges all reference and mixed-source extraction work. The shared inner setup audit does not silently amend that selected outer law or its cost tie-break.

Accordingly:

- Inner shared provisioning can still be authenticated as application Static under an independently provisioned outer key. Whether Static stores one flat vector plus descriptors or expanded repeated role arrays must be fixed in the canonical format; size savings in actual advice require the former representation or another explicitly proved equivalent format.
- If the caller separately chooses the **original ordinary native outer route**, its own actual finite schedule/view certificate must use the native shared-prefix theorem. The old FullRing outer count/formula is not its manifest.
- If the selected FullRing outer design is retained, preserve its independent roles and direct no-offload policy. Sharing those outer roles too is a separate setup-law revision needing its own exact inventory and mixed/reference radii, not something proved by the compact 12/14-view list.
- Sharing an inner flat vector with outer advice matrices would also be a separate cross-layer joint-view certificate. It is outside the stated inner candidate and not justified by summing two standalone MSIS ledgers.

The later outer transcript note, `AKITA_INSTANCE_ACCUMULATION_OUTER_TRANSCRIPT.md:268–284`, separately names **D1-Outer-FS/1** for the required same-execution extraction/composition use. D1-NGP does not establish it. Native concrete setup/FS preservation, concrete outer FS, deterministic guest/source certificates and authenticated Static provenance remain distinct premises.

## 5. Reference binding is byte-accurate, not merely payload equality

The receipt investigation also traced the immutable Jolt stage-8 path under `/tmp/akita-extension-audit-c6qh69gy/jolt/0791f79a55c3`: verifier `stages/stage8/akita.rs:517–530` takes advice claims from stage 4; `:263–278` resolves original trusted/untrusted handles; `:315–368` includes them in complete grouped PCS verification. The exposed trusted-commitment parameter is an interface, not proof that installation authority or consumed-byte semantics are correct.

Retain `AKITA_INSTANCE_ACCUMULATION_GUEST_SPEC.md:260–274` C7 obligations: exact logical length and EOF, read-only/source-preserving Static access, RAM/register read-version/last-write linkage, reachable instruction semantics, no unchecked truncation or substitute VirtualAdvice channel, and output `01` with panic=false. The canonical Static reference comparison authenticates the whole installed source; archive extraction needs its original consumed bytes and accepted parse, not a new canonicalization requirement on unused archive capacity. Changing inner setup does not weaken these obligations.

## Final scope

The main certificate plus this addendum resolve the mathematical shared-prefix prerequisite: **conditionally admit a new correctly provisioned installation; reject silent cutover or numerical receipt/security claims**. No source change, execution, build, test, linter, formatter, benchmark or implementation validation was performed.
