# Deterministic outer-source projection: theorem and remaining compiler boundary

## 1. Result and scope

**Result:** a deterministic, parametric accepting-tree-to-table projection is proved below. Its source-consistency, interpolation, prefix-slot and trusted-reference steps require no new knowledge assumption. The actual modern Jolt/Akita interfaces support substantial parts of its instantiation, as recorded in §6. **A complete instantiation for the proposed receipt guest is not established.** The missing objects are a complete frontend compiler/tree certificate for the selected outer configuration, the corresponding common-grid and cross-profile source certificate, and a byte/guest adequacy proof. The new guest does not exist. PCS extraction alone does not fill any of those gaps.

This is an exact-gap result, not an admission to implement or an assertion that the architecture is impossible. In particular, the elementary counterexamples in §7 refute tempting intermediate implications, not the soundness of the complete modern verifier.

**D1 specialization:** the later [direct prototype](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) uses `R_direct` and the [direct guest contract](AKITA_INSTANCE_ACCUMULATION_GUEST_SPEC.md), not R_skel/AS/acc or supplied graph answers. The source/table/reference projection lemmas below remain parametric tools; D1 must instantiate them and prove its actual C7 direct-computation/source-version implication. Graph-specific output/ledger clauses below concern the historical supplied-answer guest and are not additional D1 runtime checks. No compiled D1 adequacy certificate is asserted.

The invariant is unchanged: native pre-PCS segment-instance accumulation, K-valued sources, E challenges in the proposed admitted profile, the 33 **inner** roles, T<=16, and one complete final native decider. The ordinary outer proof has its own geometry and role inventory. No completed segment-proof inputs, Merkle PCS, A/B/F homomorphism, canonical-short-state extraction, or stronger application halt/MMIO semantics are used here.

Evidence is paper reading and static source inspection only. Local source links below refer to the supplied checkout as read, not an independently Git-verified immutable revision. Earlier [frontend descent](AKITA_INSTANCE_ACCUMULATION_FRONTEND_DESCENT.md) and [extension](AKITA_INSTANCE_ACCUMULATION_FRONTEND_EXTENSION.md) audits separately identify their pinned source revisions. No source edits, Git commands, builds, tests, formatters, benchmarks or guest executions were performed.

## 2. Keep four relations separate

Fix an approved outer profile p: field embedding K -> E, program preprocessing, bounded geometry, instruction mode, public-memory convention, all layouts, original source handles, admitted PCS schedules and source-comparison classes. Let

`x = (p, guest_index, X, acc, C_static, expected_output=01, panic=false)`.

Here `01` is the expected postcard encoding of `1u32`; the external verifier reconstructs the public device from these expected data, rather than accepting a prover-selected device. Profile and guest identities are installed/authorized, not made authoritative merely by being hashed.

1. **R_PCS:** the algebraic batch decommitment relation of Akita Definition 10.13 and Eq. (202). Its witness contains full commitment-grid K tables and algebraic commitment-side states. It does not contain canonical gadget states, byte strings or a machine execution.
2. **R_tab(p,x;W):** the unchanged whole-run K-valued Boolean-*indexed* table relation of the selected modern Jolt frontend. W contains original source projections and the deterministically reconstructed semantic tables. “Boolean table” here means domain `{0,1}^m`; advice/RAM/register values are not automatically bits or u64s. R_tab contains precisely the selected frontend's R1CS, lookup, RAM/register, shift, Booleanity, hamming and public-I/O equations, with its exact boundaries.
3. **R_skel(p,x;W,a,L,pi_AS):** R_tab for the fixed receipt guest, plus canonical decoding/linkage of its advice to the archive a, complete ordered supplied-answer ledger L and the chosen oracle-free query-accumulator witness/check. This relation contains no H_o call or Theta_i evaluation. It checks supplied answers syntactically/algebraically; external query authentication supplies their truth. If the selected quotient construction replaces pi_AS by curve/quotient data, use that exact interface, not the old dense-polynomial interface.
4. **Native acceptance:** the actual approved native archive verifier, given the authenticated oracle answers, accepts that same X/archive and finishes its native final decider.

The chain to prove is `accepting full tree -> R_PCS -> R_tab -> R_skel -> native acceptance after ledger authentication`. The first arrow is not the last. The [product extraction theorem](AKITA_INSTANCE_ACCUMULATION_PRODUCT_EXTRACTION.md) supplies the oracle-model/tree and PCS parts; its independent H_o/Theta_i experiment is unchanged.

The original [Jolt paper](SOURCE_PROVENANCE.md#jolt), §1.1 (especially its explicit compiler warning) separates ISA execution from correctness of the high-level witness checker. Its lookup construction and §2.1 interpolation lemma justify mathematical primitives, not correctness of all later RV64IMAC, Twist, field-inline, Akita and SDK changes.

## 3. Exact certificate needed by the deterministic theorem

A **frontend projection certificate** C for p consists of finite, checkable data and identities, not a knowledge axiom:

**C1 — Complete ordered challenge graph.** Enumerate every verifier challenge and the message prefix fixing its input, including frontend batch/reference powers, uni-skip, sumcheck rounds, stage-6a carried challenges, stage-8 prefix selectors and the PCS suffix. Declare the actual polynomial space in each challenge, not just the degree of a sumcheck round. Each supplied accepting subtree must be unisolvent for that space. Distinct d+1 scalar children suffice for degree <=d. Nested coordinate-wise children suffice for individual-degree bounds; a vector challenge is still one actual oracle answer. A flat affine family suffices only for an affine identity. For powers `1,gamma,...,gamma^d`, d+1 distinct gamma values suffice; for even powers alone, distinct gamma values without the appropriate unisolvence condition need not suffice. No extra prover messages are invented between vector coordinates.

**C2 — Claim ownership.** Every serialized claim is identified by `(table identity, original handle or defining node, logical point, coordinate order, stage)`. Every consumed claim has one producer or a checked exact alias. Passenger wire values must have a later constraining owner. A handle includes frozen source shape/layout and matrix views, not just a commitment payload or polynomial enum. All original sources compared across a frontend tree were fixed before the ancestor challenges varied. Challenge-dependent protocol workspaces are expressions, not new free witness tables.

**C3 — Complete common-grid map.** For each original PCS group, give the actual `Dec_shape` output grid and a total linear projection to each frontend physical column. Prove the claimed PCS equation is evaluation of that column: either there is no semantic padding and phi=1, or separately prove the padding-image condition and the exact point/equality-factor transfer. Map every coefficient, including capacity slots, zero-omitted addresses, ring/block tails and cycle padding. A producer's sparse constructor is not a proof of this image condition.

**C4 — Source-comparison admission.** Every occurrence of a handle in every descendant PCS extraction belongs to the same admitted comparison class, including all opening profiles reachable when frontend points change. Keep weak multipliers and bounded products until comparisons finish. Include the original installed C_static as a canonical reference occurrence with multiplier 1 and its exact canonical source. Inventory all B/F digit-difference and A cross-multiplication norms, including reference-versus-weak comparisons. A single PCS schedule's internal comparison inventory is not automatically the union needed across all frontend branches.

**C5 — Deterministic reconstruction graph.** Its leaves are C3's K tables and fixed public K tables. Each semantic node is a fixed K-linear map, Boolean-row product, fixed-table lookup contraction, shift, or prefix-sum recurrence. Every defining identity is linked to its consumer by the actual complete frontend reductions. Other algebraic constraints test these reconstructed tables. No existential E-valued virtual witness is silently introduced. Constants/characteristic and all interpolation nodes are valid; exceptional branches implement the same polynomial identity, or are expressly excluded and charged in tree admission.

**C6 — Exact frontend validity.** Include all CPU equations, lookup-table selectors and extra native lookup constraints, memory initialization/update/final relations, one-hot/activation conditions, exact shifts and public output/panic relation. Include only actual domain/range constraints. Integer conclusions need injective integer ranges and no modular-wrap ambiguities; they do not follow from field equations alone. Any requested physical zero predicate must have an actual owner.

**C7 — Byte and guest adequacy, separately.** To claim R_skel rather than only R_tab, provide the deterministic decoding/simulation certificate in §8. It is an additional mathematical compiler obligation, not an assumption that an extractor somehow knows a guest witness.

These are effective proof obligations: a missing entry yields an explicitly unidentified claim, cell, range or instruction case. Merely defining R_tab to mean “whatever the verifier accepts” would evade C5/C6 and is not this certificate.

## 4. Deterministic layered extraction theorem

**Theorem (table projection).** Fix p and C1–C6. Given a complete accepting tree for the full outer frontend and its native PCS suffix, there is a deterministic algorithm, polynomial in the explicit tree and table sizes, returning either:

* W with `R_tab(p,x;W)=1`, with all leaf evaluations owned by their original source handles; or
* a nonzero kernel vector for an explicitly inventoried admitted Akita matrix view, with its registered cross-profile collision bound.

If C7 also holds, the same algorithm followed by deterministic decoding returns `(W,a,L,pi_AS)` satisfying R_skel or the same collision alternative. This second conclusion is conditional on a **proved deterministic certificate**, not a replacement cryptographic knowledge assumption.

### Proof, layer A: native suffix and source identity

At every frontend leaf run Akita Theorem 10.31 on its complete native subtree. Stop with its collision if produced. Otherwise retain the weak certificates and the algebraic source projection, rather than retaining only the decoded evaluation value. Pick one representative occurrence for each fixed original handle. Compare all other occurrences with it using Corollary 10.17 and C4. Different commitment-side digits yield a B/F collision; otherwise a differing semantic source block gives

`z = rho' (rho s) - rho (rho' s') = rho rho' (s-s')`.

It is nonzero because both factors are units, and lies in ker A because the inner images agree. For certified bounds `||rho||_1<=k`, `||rho s||_infty<=b`, and primed counterparts, `||z||_infty<=k' b + k b'`. Reference comparison uses `rho'=1`, `s'=s_static`, giving `b + k ||s_static||_infty`; any Euclidean bounds are those actually inventoried, not inferred. Thus, absent the collision output, every source occurrence has the same algebraic projection. Applying Dec produces one fixed K grid for each handle over the **entire relevant frontend subtree**.

No bound on `rho^{-1}` was used; s can be long/noncanonical. No claim that `Commit(s)` recreates the handle was made. This is exactly the boundary of [Akita](SOURCE_PROVENANCE.md#akita), Definitions 10.13–10.14, Corollary 10.17 and Remark 10.32.

### Proof, layer B: physical/common-grid projection

Apply C3. In a no-padding group, the authenticated full-grid table is already the physical frontend table. With semantic padding, use its proved image identity, not division by a possibly zero equality factor. Akita §5.1/Eq. (80) and Eq. (202) explicitly distinguish these cases.

There is one useful padding predicate the existing frontend really can prove: prefix packing. Suppose stage 8 fixes logical point r and logical claims v_i before sampling selector s, and authenticates the same physical table P at every child with

`P~(s,r) = sum_{i in used slots} eq(s,i) v_i`.

Both sides are multilinear in s. A complete nested binary selector tree forces polynomial equality. At Boolean selector i it gives the used-slot evaluation v_i; at every unused slot it gives zero. If the ancestor logical-point families identify those slot MLEs on a unisolvent set for r as well, interpolation gives the **entire** used-slot table and zero unused slots. At a single r it only gives evaluations: one must not infer all cells are zero. This argument proves slot padding without honest-state canonicality and without dividing by `eq(s,0)`. Different ancestor branches may use different interpolation children; induction works fibre by fibre because layer A fixes P.

Additional backend padding beyond this explicit prefix layout is a different obligation. This argument does not authenticate cells never seen by that layout's query map.

### Proof, layer C: sumcheck, batching and virtual tables

For a fixed true downstream polynomial F and a round message p(Z) of degree <=d, descendant certification at d+1 distinct round challenges gives `p(alpha)=sum_remaining F(prefix,alpha,remaining)` at those children. Equality as polynomials follows. Evaluating at 0 and 1 and using the verifier's round-sum equation recovers the preceding claim. Induct backwards over all rounds. This is a deterministic identity proof, with no Schwartz–Zippel event remaining once the complete tree is supplied.

Separate batch/reference coefficients by their declared unisolvent families. If this yields

`sum_b eq(tau,b) E_j(b)=0`

for a complete multilinear tau family, it is the zero polynomial in tau, hence `E_j(b)=0` for every Boolean b. This is why a zero sum, or a lone sampled equality weight, is not enough. Product lanes and uni-skip lanes require the appropriate polynomial space and complete distinct interpolation nodes; C1 includes them, not just the later scalar sumcheck rounds.

Proceed bottom-up through C5. Define each semantic table over K using its already fixed downstream tables. Interpolation forces all its carried claims to equal the E-MLE of this reconstructed K table. K-linear maps, Boolean-row products, fixed shifts and prefix sums commute with the embedding. This does **not** identify the MLE of a Boolean-row product with the product of MLEs off the Boolean cube; the product sumchecks supply that linkage. Protocol partial bindings and reference-weighted workspaces may remain E-valued.

Finally the separated CPU/lookup/memory/public constraints vanish on every required Boolean row by C6. Injectivity K -> E converts these zero identities back to K. This is R_tab. Every operation is deterministic interpolation, reconstruction or the already specified collision algorithm, proving the theorem.

### Proof, layer D: byte/guest projection

Apply the deterministic C7 decoder and simulation theorem to W. It preserves the original public x, the static reference and the same acc, and returns exactly the declared archive/ledger/AS material. This establishes R_skel. No oracle is queried. Only a later query-authentication theorem can replace ledger answers by Theta_i truth and infer native acceptance. See §8 for exactly what must be proved before this layer can be instantiated.

### Relation to the product-oracle loss

This deterministic theorem has no additional failure probability. For its *random* tree generation, C1 must be added to the product note's actual mixed-stage list: for round r use its true bad-family mass e_r and true conditional sampler. The bound is `(Q_o+1) sum_r e_r` for the whole admitted list, not just the PCS suffix. Scalar polynomial identity bounds may contribute `d/|E|` where appropriate; vector, affine, fixed-filter and rejection cases retain their own laws. The tree/work product, frontend branch source comparisons and table projection work enter the global cutoff. No numerical full-frontend ledger is certified here.

## 5. Semantic reconstruction and domain consequences

The following explicit K reconstructions are the target, not a hypothetical canonical trace supplied by an honest prover. They agree with the detailed [frontend descent inventory](AKITA_INSTANCE_ACCUMULATION_FRONTEND_DESCENT.md), §§3–4.

* For zero-omitting instruction/bytecode/digit/carry columns, `R(0,t)=1-sum_{a!=0} X(a,t)` and `R(a,t)=X(a,t)` for a!=0. RAM uses the full physical column. Booleanity is on R.
* For digit width k and centered digit table d, `Delta(t)=sum_j 2^(kj) sum_a d(a) X_j(a,t) + 2^64 sum_a d(a) X_carry(a,t)`. Then `RamInc=Store*Delta`, `RdInc=(1-Store)*Delta`. The full centered carry alphabet is admitted; it is not merely {-1,0,1}.
* Bytecode selectors are products of semantic bytecode digits. PC, instruction flags, lookup flags and register-access selectors are pullbacks of the approved bytecode table. Instruction selectors similarly reconstruct operands and lookup output from the fixed lookup tables. The native instruction-RAF upper-half constraint must be retained, not dropped when changing challenge field.
* `RegVal(r,t)=sum_{u<t} RdWa(r,u) RdInc(u)` for the original zero-initialized whole run. Read values and post-write values are the corresponding selector contractions.
* For initial RAM I from fixed public memory plus original advice/program sources, `RamVal(a,t)=I(a)+sum_{u<t} RamRa(a,u) RamInc(u)` and `RamValFinal(a)=I(a)+sum_u RamRa(a,u) RamInc(u)`. Read/write values are selector contractions. No u64 premise is used in these field identities.
* Product/branch/jump tables are their actual Boolean-row products. Successor shifts use the existing missing-successor values (including terminal NextIsNoop=1), not a newly imposed emulator halt.

After complete Booleanity and hamming extraction, a digit row with entries in {0,1} and sum one is genuinely one-hot provided the address alphabet size is less than char(K). A RAM digit row with Boolean hamming weight has occupancy zero or one under the same no-wrap bound. One must additionally use the actual activation/address equations to identify when RAM is active. These statements do not range-check arbitrary advice or RAM values.

Zero-omitted physical address-zero cells do not occur in R. Semantic extraction can therefore project them away; it cannot certify their physical zero/Booleanity. If an outer source domain or accumulator theorem requires their physical bound, it needs its own checked zero-mask owner. The inner-only strengthened masks in [main-source reuse](AKITA_INSTANCE_ACCUMULATION_MAIN_SOURCE_REUSE.md) do not automatically exist in the ordinary outer proof.

## 6. Actual source instantiation: checked facts versus open certificates

| Layer | Source-backed fact | Exact remaining condition |
|---|---|---|
| Stage 0/public ownership | [verifier.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/verifier.rs), `absorb_commitments`/`absorb_akita_commitments` (708–784), takes untrusted commitment from proof and trusted commitment from caller, absorbing canonical object order. Preamble (910–983) absorbs preprocessing digest, input/output/panic, RAM/trace/entry/config fields. | Install authorized preprocessing and PCS setup; bind expected X **and acc** in the new public argument. A digest authenticates neither an attacker-chosen program nor an uninstalled PCS setup. No new guest/public schema is implemented. |
| Stages 1–7 | [relations.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/stages/relations.rs), 92–169, exposes symbolic rounds/degrees and typed input/output claims; batch/output structs own order. Existing frontend audits map Spartan outer/product/shift, instruction lookup, RAM/register relations and stage6a/6b/7 reductions. | Produce the complete selected-mode C1/C2 certificate: every emitted wire claim, alias, reference family, exact challenge polynomial degree, vector draw and exception. A Rust trait/derive and a stage inventory are not a proof this whole graph has no missing owner. |
| Main stage 8 | [stage8/akita.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/stages/stage8/akita.rs), 48–92, checks backend, chunk size, digest, **exact** arity and physical count against canonical plan/setup. `one_hot_trace_claim` (372–415) requires every plan column and equality of mapped opening points. `leaf_claims` (441–548) rejects duplicate IDs and sources main columns from stage7. | Prove the backend decoded grid equals that physical grid for each provisioned schedule. Metadata equality alone is not a padding-image proof. |
| Backend schedule boundary | [shape_guard.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/shape_guard.rs), 53–103, derives grouped layout from per-group arity/count, resolves trusted schedules, validates the layout and reconstructs payloads with those profiles. [trace_onehot/source.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/trace_onehot/source.rs), 171–205, reports full physical coefficient counts for the honest streamed source. | Audit `Dec_shape`/selected profile common-grid dimensions, all block/ring tails and comparison classes. Honest descriptor counts alone cannot prove arbitrary weak sources have the honest padding. No concrete new outer schedule family has been selected here. |
| Advice ownership | Stage8 `advice_object` (163–184) requires claim/commitment presence agreement. `leaf_claims` (517–530) takes **stage4** RAM-init advice claims directly. `verify` (269–333) checks metadata and puts untrusted/trusted objects in the joint native opening. It does not substitute generic stage6b/7 advice reductions. | Preserve that owner chain when adding any extra source. In particular a quotient source's four K-linear claims are not existing advice claims and need their own admitted compiler. |
| Advice common grid | [lattice/packing.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/jolt/lattice/packing.rs), 183–215, defines physical advice arity `max(word_vars,14)` with slot-zero embedding; [prefix.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-openings/src/prefix.rs), 154–219, binds logical claims then draws the selector and uses only used-slot equality weights. | §4B proves unused prefix slots zero only with complete selector **and ancestor logical-point** families. This does not prove unrelated backend padding or logical unused-word zeros. For capacities above the floor there are no extra selector slots; the whole advice word domain is still not automatically u64-valued. |
| RAM reads | [stage4/ram_val_check.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/stages/stage4/ram_val_check.rs), 1–83, uses the same advice contributions in initialization and carried opening values/points. | Finish the source-linked complete-tree extraction of the RAM recurrence. Source linkage authenticates field words; byte-read adequacy is separate. |
| Public success | [stage2/ram_output_check.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/stages/stage2/ram_output_check.rs), 1–60, checks masked final RAM against public I/O, not merely a transcript claim. `verifier.rs` normalizes trailing output zeros. | Require exact caller-expected logical encoding 01 and panic=false, then prove this fixed guest reaches that outcome only after its declared checks. Do not read success from a prover-supplied device or infer termination solely from output normalization. |
| Optional modes | Existing audits identify committed-program direct sources and field-inline field-register sources as separate original K handles. | Each selected feature mode needs its own complete graph and static-source comparisons. Do not combine unsupported modes or import an ordinary RV64 proof as field-inline adequacy. |

The current aliases in [Akita preprocessing](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/preprocessing.rs), 27–28, use `AkitaField` and `LegacyBlake2bTranscript<AkitaField>`; [shape_guard.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/shape_guard.rs), 35/60, fixes `ExtField=AkitaField`. Thus the source supports inspection of algebra/ownership but is **not** evidence of an implemented K-source/E-challenge outer frontend. The extension audit's type/accumulator/sampler work remains distinct from this algebraic theorem.

## 7. Exact counterexamples to unjustified shortcuts

### 7.1 Physical one-hot holes

For a zero-omitting family,

`R~(r,t)=eq(r,0)+sum_a (eq(r,a)-eq(r,0)) X(a,t)`.

The coefficient of X(0,t) is identically zero. Changing this cell to 2 leaves **every** semantic evaluation unchanged, even with a complete frontend tree. Hence semantic Booleanity does not imply physical Booleanity or physical zero. This is the source-backed map kernel already identified in the descent/reuse notes. It is harmless to semantic projection if explicitly ignored, but not a certificate for a raw bounded-source predicate.

### 7.2 A weak PCS opening is not natural-domain padding

Let a purported slot-zero table have an extra prefix coordinate z. At one fixed r with r_z!=1, choose a full-grid table P with nonzero slot 1. The opening equation `P~(r_z,r)= (1-r_z) v` defines an admissible scalar v without forcing that extra slot to zero. This is an algebraic counterexample to inferring `P in im(Pad)` from the extracted scalar equation. It is not a forgery of a complete prefix-selector tree: §4B explains why that tree closes this particular hole. Akita §5.1 and Definition 10.13 expressly leave algebraic padding unconstrained.

### 7.3 Honest advice serialization does not range-check malicious advice

[commit_advice](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/witness.rs), 340–374, honestly obtains `canonical_advice_words`, maps little-endian u64 words into the field and commits a zero-padded polynomial. [common/advice.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/common/src/advice.rs), 30–44, is the honest packing algorithm. Neither function is an adversarial verifier range check.

A K element with integer representative 2^64 has no u64 preimage because char(K)>2^64. RAM-init linear equations accept field elements syntactically. The load rows in [rv64.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-claims/src/protocols/composed/r1cs/rv64.rs), 229–235, impose `Load*(RamRead-RamWrite)=0` and `Load*(RamRead-RdWrite)=0`; assigning all three values 2^64 satisfies those rows. They do not range-check the word. The same file, 282–293, expressly disables the ordinary right-input linkage for the Advice flag, consistent with nondeterministic virtual advice rather than an authenticated tape read.

These are counterexamples to the **local inference**, not a constructed globally accepting false guest trace. A particular program may force all consumed words into range through later lookup/byte-decomposition constraints. That requires a read-use/ISA simulation proof for that program. It is not supplied by a dense PCS schedule or by the name `UntrustedAdvice`. In particular, arbitrary decoded capacity padding may never be read and need not be in the honest byte image.

**Subsequent source resolution:** the [consumed-byte note](AKITA_INSTANCE_ACCUMULATION_CONSUMED_BYTES.md) traces the actual LBU/LB/LHU/LH/LWU/LW expansion through LD, register recurrence and the PEXT full-operand equality `W=L(k)`, with k a Boolean 128-bit interleaved lookup index. Thus a completed subword read forces its entire containing word into u64 and the isolated `2^64` assignment cannot extend through it. This closes that local inference, not raw LD/SD canonicality, source-region immutability, whole-capacity canonicality or the unimplemented guest's complete C7 certificate.

### 7.4 Framing and canonical-byte identity

[jolt-sdk/macros/src/lib.rs](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/jolt-sdk/macros/src/lib.rs), 808–827, decodes public/trusted/untrusted arguments with `take_from_bytes` and leaves `check_input_len` empty. Remainders are not checked. Honest word packing also maps a byte string b and b followed by zero bytes to the same padded word table when capacity permits. Therefore the commitment alone does not bind the *external logical length*. A canonical length-bearing envelope and guest checks must do so. Requiring logical archive EOF is not the same as forbidding all unused fixed-capacity bytes.

A byte-vector argument can provide the needed logical envelope. That does not prove that the unimplemented guest checks EOF, rejects alternative encodings, or treats all unused bytes consistently. If the protocol needs whole-capacity canonical zero padding, it must certify that stronger predicate; if it ignores padding, its commitment/message relation must explicitly allow it.

## 8. Trusted source, byte decoding and exact missing adequacy lemma

### Installed C_static

Let B_static be the approved canonical trusted envelope, and let s_static be its original canonical algebraic commitment state under the provisioned outer advice source shape. Comparing every extracted trusted occurrence to this reference by §4A proves equality of the *entire* source projection, or yields the priced collision. Thus the trusted used-slot words, their honest padding and every read can be tied to B_static without assuming the extracted weak state was initially canonical. This is a reference-comparison conclusion, not a general weak-state canonicality theorem.

The premise requires an actual efficiently constructible reference state/registered comparison profile and identical original shape/matrix views. “The verifier passed some C_static” does not supply the mixed norm envelope. The reference is the outer advice commitment to the provisioned static blob; it is not interchangeable with the inner 33-role registry. Trusted RAM initialization must use this same handle through stage4/stage8 as inspected above.

### Precisely required deterministic byte/guest lemma

For the fixed guest index G and unchanged selected whole-run relation, the missing lemma is:

> For every W satisfying R_tab with the expected public `(X,acc,profile,success1,panic=false)` and original C_static linkage, a deterministic procedure recovers a single bounded canonical archive a and ordered ledger L (and the actual AS material), such that the specified oracle-free guest checker accepts `(X,acc,a,L,B_static)`. All bytes read by that checker equal the bytes simulated by W; all required query events are represented in L with the exact encoding/order. Conversely, each valid bounded checker execution has an admitted R_tab witness, subject only to the separately specified honest proof/sampler failures.

To prove it, a certificate must supply:

1. **A byte interpretation for each relevant read.** For full-word bytes, prove the used word is `sum_{i=0}^7 2^(8i) b_i` with each b_i in [0,255] and matching little-endian reads. Since 2^64<char(K), this representation is unique. Alternatively prove the actual byte-load virtual sequence constrains those same bytes; truncating a general field word on the host is not a proof. For unused words, explicitly permit arbitrary field values if the language ignores them, rather than silently canonicalizing the committed source.
2. **A local simulation for every expanded instruction actually reachable in G.** Include PC/register/memory updates, byte/halfword/word load/store expansions, virtual advice and its subsequent checks, no-op/padding behavior, and the actual entry/public-output convention. Preserve existing ISA semantics. Do not introduce a new MMIO/halt language just to make a theorem easier. Field-inline, if selected, needs its actual field limb/readout semantics as well.
3. **Global composition.** Induct on the existing ordered table rows: initialization gives the same memory/register state; local simulation gives the next one; established shift/PC and memory recurrences link them; the unchanged public-I/O relation fixes the declared result. This proves a bounded execution refinement for this guest, not a universal unexamined equivalence with an emulator. Its endpoints remain those of the original application relation.
4. **Canonical bounded parser and static use.** Reject malformed counts (including zero or >16), lengths/overflows, noncanonical field/extension encodings, descriptor/profile mismatches and logical archive trailing bytes. Bound all allocations and accesses; establish which capacity padding is ignored. Trusted envelope parsing must use the installed bytes above, not prover-supplied replacement setup.
5. **Oracle-free exact ledger algorithm.** Represent all native transcript calls, domain/context/framing, rejected sampler attempts, nonce trials, vector coordinates and raw outputs. The guest must execute the native deterministic verifier using these supplied answers and require ledger exhaustion/coverage, with any deduplication justified by exact equality. It must execute every native update prefix and the complete final normalization/shrinking/terminal decider, not a completed segment proof verifier or a pending opening check.
6. **AS and public acc linkage.** The checker must compute/check the chosen oracle-free AS relation on that very ledger and bind exactly the externally checked acc, including profile, query field and encodings. This is not equality in distribution and cannot be replaced by hashing an unapproved identifier. Neither H_o nor unauthenticated concrete hashing may be inserted as a shortcut.

The induction in item 3 is a proved generic composition principle once items 1–2's finite local implications and item 4–6's actual program correctness are supplied. None has been verified for a nonexistent G. Restricting to the instruction set actually used by G is legitimate; announcing a different easier application/ISA is not.

### Curve-source reuse boundary

If the quotient candidate reuses untrusted-advice C_g, it must bind **all** pre-beta bytes/Boolean coordinates determining the curve, including every ledger y and z, and carry a canonical logical length. Used-read adequacy alone may not prove the *whole original source* is the canonical message required by that candidate's relation-binding proof. Its message relation must handle arbitrary ignored capacity cells explicitly or prove them canonical.

Conversely C_g must not be required to contain its own postcommit challenge/accumulator material: with `cm=Enc(C_g,C_t,profile,nonce)` and `beta=ro_i(cm)`, forcing raw_beta, wo(cm), g(beta) or t_beta into the committed message creates a fixed-point completeness problem. Keep such postcommit data in the appropriate separate public/supplied-data interface. Extra **prechallenge** archive bytes may be committed. This changes neither the outer public acc binding nor the obligation to link that acc to the same ledger.

## 9. Final disposition

The deterministic proof closes the generic logical steps often obscured by the phrase “extract the VM witness”: weak-source comparison, complete-tree interpolation, K descent, and the particular prefix-slot padding proof. It also identifies the exact source ownership route for installed trusted advice and the externally expected success value.

It does **not** close the full outer receipt theorem. The remaining deliverables are sharply separated:

* **Frontend certificate:** exhaustive selected-mode challenge/claim/degree/alias graph, actual common-grid decoder/padding audit, cross-frontend-branch and static-reference comparison inventory, and corresponding full-tree work/error ledger.
* **Byte/ISA certificate:** consumed-advice byte range/read semantics and the unchanged relation's simulation for the actual fixed guest, including treatment of unused capacity and any virtual advice.
* **Guest certificate:** an actual fixed program and proof that canonical success1 entails the specified native archive/ledger/AS algorithm accepted with identical X/acc/C_static.

The papers and inspected source do not furnish those exact certificates for the proposed guest. This is an unavoidable missing compiler/program lemma, not a reason to add a cryptographic knowledge assumption or weaken the original application statement. Implementation and end-to-end security admission remain gated. No checks were run; a future main-agent verification should review these proof certificates and the complete selected source graph before any executable verification is meaningful.
