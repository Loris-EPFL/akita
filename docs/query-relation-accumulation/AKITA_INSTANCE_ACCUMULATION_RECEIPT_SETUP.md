# Final receipt: setup, public statement and outer-profile admission

## 1. Decision and evidence boundary

**Selected setup contract:** the verifier provisions and authenticates a fixed static-data blob containing the approved **inner** matrices, canonical application program and admitted proof-independent prepared data. It commits that blob once under an independently provisioned **outer** ordinary lattice advice key, installing the exact expected commitment `C_static`. A fixed bounded guest reads those bytes through the authenticated trusted-advice source and verifies the complete native accumulation archive **and its final decider** from private/untrusted advice. Neither blob nor archive is public receipt input or embedded into a new executable for each run. This is one final, **complete ordinary Jolt/Akita proof**, not a second pending accumulator. Deterministic in-guest expansion is a separate ideal-XOF-model alternative (§3), not the default.

The security anchor is unchanged acceptance of the exact approved inner run statement under its original source projections and matrices. The wrapper does not strengthen that statement to emulator, word, halt, or advice semantics absent from the reviewed whole-run Jolt Boolean-table relation.

This document specifies an independently checkable installation and admission contract, **not an already admitted numerical outer profile**. The reviewed inner profile does not automatically certify a receipt. In particular, the selected modern ordinary Jolt/Akita field/transcript interfaces currently provide important obstacles to inheriting the inner extension-field Fiat–Shamir bound; §7 makes these explicit. No source changes, executions, builds, tests, or benchmarks were performed for this document. Abstract extraction/composition is a separate obligation.

**Current D1 route:** installation/source evidence below supports the [direct experimental receipt](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) and [guest contract](AKITA_INSTANCE_ACCUMULATION_GUEST_SPEC.md). The guest computes the fixed public inner transcript; original C_archive and installed C_static remain, while graph/acc/C_t/quotient/external-Vhat requirements do not. This note does not supply the still-required ordinary outer carrier/compiler or an installed numeric profile. Security experiments sample the installation from its specified provisioning law, preserving the sample within extraction rather than claiming security for every realized key.

**Native-reuse revision:** the [new candidate](AKITA_INSTANCE_ACCUMULATION_NATIVE_REUSE_PROFILE.md) retains authenticated Static and one complete ordinary K/K outer proof, but changes the proposed inner setup to Akita's shared-prefix seed/page law. Its [independent setup review](AKITA_INSTANCE_ACCUMULATION_SHARED_SETUP_REVIEW.md) conditionally supports a fresh installation. The independent-role law, repeated-array size and 512-bit bias in §3 remain the earlier installation, not facts about that candidate. Source/reference binding and all guest/composition gates remain.

Sources are the selected Jolt `0791f79a55c3fadc3d327c78a72228696c338468` / Akita `83574331d4e51f8cce1d8689d05592fa4ef4c138` trees and the linked research specifications. Modern API evidence is not proof of implementation of this new guest.

## 2. Fixed installation and compact public statement

Separate an **installation record** from the **per-run statement**. The external verifier obtains the installation record through its own authenticated configuration or independently reconstructs and approves it; it never accepts replacement preprocessing supplied by the proof author merely because a digest accompanies it.

An installation record `I` fixes:

1. Exact guest executable and read-only image, toolchain/ISA/feature configuration, ordinary Jolt preprocessing, trace ordering, memory layout, public-input/output codecs, and success output encoding.
2. Exact approved application program and its original public-I/O interpretation; the inner segment consumer/lowering rules and initial/global-final conventions. The program can be a fixed guest constant or an installed authenticated object that the guest actually checks. A caller-chosen program label is insufficient.
3. Inner compact profile, manifest, source/query maps, sampler versions, actual provisioned matrices and their generation provenance (including seed if a seeded law is selected), role namespaces, matrix certificates, and limits. In this profile: `1 <= T <= 16`, `Nseg=4096`, `Pmain=56`, padded public application program length 4096, RAM domain 65536 words, no inner optional advice/committed-program/field-inline objects. The four original handles and unchanged RAM role across cuts remain as specified in the inner design.
4. Outer ordinary proof profile, its complete finite schedule/catalogue and validated matrix-view manifest, independent setup provenance, actual scalar/lattice samplers, transcript domains/bridge, security policy, and every capacity in §8.
5. The chosen setup access mode. The selected mode in §5 installs `C_static`, its exact source layout, byte length and canonical reference decommitment provenance. The alternative deterministic-expansion mode has no trusted-advice setup object and has the separate setup-law assumptions in §3.

An identifier `id(I)` is a compact selector for an already authenticated installation. It does not by itself authenticate its contents. An implementation may use a collision-resistant canonical digest to bind the record in transcripts, with that assumption charged; it must still establish the record's provenance. Two instances using the same label but different executable, source map, seed or matrices are not the same installation.

Let X denote the application's approved public input/output and run metadata. The public receipt statement is canonically encoded as

`x_o = (receipt-format D1, authenticated id(I), X)`.

Input/output lengths and all metadata are bounded. Do not expose segment history or archive merely to authenticate it. Original C_archive is the ordinary proof's actual untrusted-advice handle; it is not duplicated as an application public-input field. Presence, shape, transcript binding and the same-source consumed-byte projection remain mandatory. The former graph branch's public acc/C_t/quotient fields belong to that separate design.

For D1, let `S_static(I)` be the approved canonical blob and a the canonically consumed archive from original C_archive. The intended extracted relation is

`R_direct(I,x_o;W) := ValidOuterExecutionAndOriginalSources_I(W) AND CanonicalStatement_I(x_o) AND ConsumedStatic(W)=S_static(I) AND GuestDirect_I(x_o,a,ConsumedStatic(W))=1`.

The guest does not compare all bytes to a second embedded copy. Installation computes the original commitment to `S_static(I)`; the outer proof binds the guest's trusted-advice source to the installed `C_static`; §5's extracted-source/reference comparison establishes `s = S_static(I)`, except at the charged binding error. For the alternative deterministic mode remove `s` and its equality, and have the guest derive the fixed matrices. In either mode the fixed guest:

- uses only the approved inner matrices, canonical application program and profile;
- reads one bounded canonical archive from original C_archive, checks its X against public X, and computes the exact fixed D1 transcript and bounded samplers itself;
- verifies each frontend/update prefix in its prescribed order, first/last endpoint rules, all links, all old-source domain checks retained by the generic update theorem, and the complete final domain/reduction/terminal decision;
- rejects `T=0`, `T>16`, malformed/oversized counts, unsupported modes, invalid field/ring encodings, reordered/dropped/duplicated updates, inconsistent endpoints, omitted final obligations, and trailing bytes;
- requires exact logical archive consumption and complete native acceptance, then returns the sole canonical success value. D1 uses no supplied graph answers, quotient source, external graph decision or guest-side outer H_o call.

The external verifier resolves approved I and expected X, reconstructs exact x_o/device with output byte `01` and panic=false, supplies installed C_static, preserves actual original C_archive presence/shape/binding, and verifies the **complete ordinary outer proof**. It does not replay the archive or perform graph equations. Full original-source/guest projection and D1's declared conditional composition remain admission obligations; private advice alone is not ZK.

## 3. Inner setup law and deterministic-expansion alternative

Use the original [matrix registry](AKITA_INSTANCE_ACCUMULATION_MATRIX_REGISTRY.md), not the modern backend's default shared-prefix key distribution:

- Six A roles, five B roles, four D roles and eighteen F/H compression roles, **33 independent registered maps**.
- Exactly **4,681,728 K coefficients**, represented canonically in **74,907,648 bytes** (71.4375 MiB) at 16 bytes per coefficient, excluding descriptors and working buffers.
- A matrices retain their original source role, P/B geometry and source decoder. The RAM incoming/outgoing/carried handle uses the same RAM role; final levels deliberately create distinct maps. B/F and D/H paths and all legal noncanonical source digits remain present. The final A-only terminal has no newly invented B/F or D/H map.

**Selected actual-matrix provisioning.** The verifier, or an authenticated setup producer whose generation law the verifier accepts, samples each role coefficient from an independent uniform 512-bit string and reduces it modulo q. This realizes 33 independent role matrices up to the same aggregate statistical bias `< 2^-361`, without identifying a concrete SHAKE evaluation in the guest with an ideal random oracle. True independent samples give that information-theoretic distribution; use of a CSPRNG instead requires the explicit generator assumption. Fix the generated arrays before any application commitments, retain their exact canonical serialization and authenticated provenance, and install their outer commitment through §5. Sampling/approval happens once outside the guest. Merely approving arbitrary adversarial matrices does not establish the required distribution. An approved seeded setup is also possible, but retains its explicit setup-XOF assumption; committing its bytes authenticates them and does not prove that assumption.

**Alternative deterministic guest mode.** The installed descriptor fixes a concrete XOF, seed, length-delimited namespace encoding and coefficient order. A suitable **new provisioning** rule in the declared ideal setup/XOF model is:

`A_role[i] = LE512(XOF("native-inner-setup/v1", seed_inner, canonical_role_descriptor, i)[0..64]) mod q`.

Here `i` enumerates the exact registered row/column/coefficient order, with range fixed by the role. Distinct role descriptors, setup versus FS, and inner versus outer domains are disjoint. This is a specification of the new expansion, not a claim that existing keys already obey it. The field is

`K = F_q, q = 2^128 - 2^32 + 22537`.

The deterministic law models these 512-bit XOF outputs as independent uniform samples, giving the reviewed aggregate bias `< 2^-361` in that model. A concrete public-seed SHAKE computation is not literally a sample from independently uniform matrices: correctness of the guest's SHAKE execution does not supply the missing ideal-oracle/composition theorem. This additional modeled-to-concrete obligation must remain explicit. The application cannot choose seeds adaptively per proof. Pin an approved public seed before commitments; any alternative ceremony/seed selection changes the installation record and its assumptions. Hash/XOF collision/preimage properties used for record identities are separate from the ideal-distribution assumption for the setup stream.

In this alternative mode the guest can stream matrices or cache selected roles to reduce resident memory. It must actually derive every coefficient used; accepting a supplied matrix under a matching seed label is forbidden. Unused roles need not be materialized just to prove their existence, but all 33 maps are mathematically fixed by installation. If every coefficient is materialized once, the expansion consumes 64 XOF-output bytes per coefficient before reduction (299,630,592 output bytes total), plus framing/absorption and modular arithmetic. This is an operation/byte count, **not a cycle or wall-clock estimate**. Repeated expansion due to eviction is additional work. Selected trusted-advice provisioning pays generation outside the guest and pays the distinct source-binding costs in §5; it does not simulate an ideal setup oracle inside the guest.

`E = K[U]/(U^2-5)` remains the **inner scalar challenge field**, while original sources and matrices remain over K. An E partial in the full degree-128 carrier contains 256 K coefficients. The guest emulates the exact two-component arithmetic, transcript absorption, and bounded rejection sampler from the [global manifest](AKITA_INSTANCE_ACCUMULATION_GLOBAL_MANIFEST.md), including abort rules. No field alias, reinterpretation of source coefficients as E sources, or replacement of an E challenge by one K scalar is permitted.

## 4. Dependency order: no self-authenticating setup

Provision in this order:

1. Fix application, inner registry and actual setup/provenance, guest algorithm and codecs, static-blob format, and bounded resource policy.
2. Compile the fixed guest. Select a candidate complete **ordinary** outer schedule from the guest's actual public shape and resource bounds. Any capacity revision creates a new candidate; admission must terminate in a finite fixed manifest.
3. Generate/validate outer public matrices from an independently domain-separated outer setup root under the ordinary profile's specified distribution. Shape may depend on public capacity, but **outer key coefficients must not depend on a commitment later made under those same keys**, the archive, or proof-controlled input.
4. Derive and validate outer preprocessing for that executable and profile. Freeze the advice source geometry/key before computing `C_static`, then commit the approved static bytes under that map. Capacity selection can use a separately admitted finite family of ordinary profiles; the final selected member must preserve the registered advice map. A map change requires recomputing and reinstalling `C_static`.
5. Install the expected executable/preprocessing and commitment. A final installation digest may include the resulting commitment, but that digest must not feed back into its own commitment-key derivation or force recompilation of a guest whose keys depend on the digest. The guest can check an installed profile constant that excludes this later payload; the external verifier binds the final installation and commitment.

The selected dependency graph is `independent inner sampling/approved setup + canonical application/static preparation -> static bytes`, `guest/capacities + independent outer setup -> outer keys`, `(static bytes, outer keys, fixed advice layout) -> C_static`. There is **no** `C_static -> outer keys` edge. The outer key need not be expanded inside the guest: it is used by the external ordinary proof verifier, not by the inner archive verifier.

Ordinary Akita may itself use registered setup-prefix offloading. That is a distinct feature from putting the **inner** setup into guest advice. It is allowed only with the ordinary schedule's complete authenticated preprocessing and source-comparison certificate; a finite prefix registry established outside the proof is not permission for prover-selected self-authentication. The final ordinary terminal must discharge all such obligations.

## 5. Selected committed trusted advice and original-source linkage

Verifier-provisioned trusted advice is selected because it matches the modern native original-advice commitment interface and keeps actual setup generation outside the guest. It introduces a large additional outer source and a source-binding obligation; it is not claimed faster without measurement.

The verifier or authenticated preprocessing producer samples/validates the exact §3 matrices, approves the canonical application program and independently validates every included static preparation. It creates

`Sbytes = CanonicalEncode(blob-version, inner-profile, exact-roles-and-shapes, exact-K-coefficients, canonical-application-program, approved-static-prepared-data)`

and commits it once under the **outer** ordinary lattice advice source map. The installation records the expected byte length, padding, layout, content provenance and original commitment `C_static`; the prover obtains matching bytes and original prover object/hint. The external verifier supplies `C_static` from installation, not from the archive or a freely chosen proof field. Any static item already fixed in the guest may be omitted from the blob only with an explicit installed layout rule.

Prepared data must be a checked deterministic function of these approved static inputs, with a fixed compiler/preprocessor version and canonical decoder. **Proof-dependent table evaluations, transcript-derived weights/challenges, matrix contractions, application-public-input specializations and final acceptance equations stay inside the guest**, unless a separately proved reduction authenticates them. Calling a table “preprocessed” does not make proof-dependent arithmetic free. The same legitimate static preprocessing is available to the completed-segment recursive control.

Modern source evidence:

- [`preprocessing.rs:229–250`](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/preprocessing.rs) builds trusted advice using the program's declared maximum size and immutable PCS setup context.
- [`witness.rs:340–379`](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/witness.rs) canonically packs advice as little-endian u64 words, zero-pads, constructs its physical layout, and makes an **original** lattice commitment. It does not merely hash the bytes.
- [`verifier.rs:283–312`](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/verifier.rs) takes an externally supplied trusted commitment, validates its presence and seeds the transcript with it; the ordinary stage-8 route opens the advice claim in the grouped PCS proof. The existence of this parameter is an interface, not proof that callers provision it correctly.

For 74,907,648 coefficient bytes alone the canonical word vector contains **9,363,456 u64 words** before padding. A single power-of-two padded word domain therefore needs at least `2^24` words; application/static-data bytes, metadata and implementation layout floors must be included in the actual plan and can require a larger domain. At 16 bytes per outer K element, that minimum padded field vector alone occupies 268,435,456 bytes if materialized, before signed-digit source expansion, hints, proofs, matrices and working buffers. These conditional representation counts do not specify the total outer trace or memory profile. Do not pretend this static object fits the inner `N=2^22` receiver.

The extracted outer advice source must be linked as follows:

1. The guest's setup/program/static-data reads must be connected through the ordinary Jolt advice claim reductions to the **same original** `C_static`, fixed layout, exact length and canonical word-to-byte map. Advice length is checked explicitly: zero padding must not make multiple byte lengths acceptable. Source words reconstruct canonical 16-byte K coefficients and exact program/static objects; reject coefficients >=q, malformed objects and absent/extra bytes.
2. Retain the complete original source path and its A/B/F projection, not a freshly committed redecomposition under a cheaper source class. Advice words are u64-bounded, not Boolean, and the correct original balanced-digit encoding and supplied legal decommitment paths matter.
   **Exact source reach, not a bit-count shortcut:** if a newly compiled outer advice profile chooses zero-offset base-32 digits `[-16,15]`, its positive reach at depth δ is `15*(32^δ-1)/31`. At δ=13 this is strictly below `2^64-1`; covering every unsigned u64 therefore requires at least δ=14, or a separately specified and proved affine-offset encoding. `ceil(64/5)` and a preset's “65-bit signed bound” do not alone certify that gadget reach. This is a conditional requirement on a proposed base-32 compiler, **not** a claim that the immutable ordinary preset uses that base/depth or is approved under a substituted encoding. Preserve the actual preset's original gadget, decoder, depth and source path in the emitted outer manifest and check its exact positive and negative reach.
3. Compare the source extracted from the accepted outer opening to the canonical reference source created in preprocessing. Apply source uniqueness with its **mixed** weak/reference collision envelope, including B/F path alphabets, source decoder and all profiles that can open that handle. Different opening points and D/H payloads are allowed; requiring their equality would be an incorrect stronger premise.
4. Either the decoded sources agree, yielding the installed bytes, or the comparison yields a recorded outer matrix collision. Account the known bounded canonical preimage versus extracted weak source comparison in the outer registry/security ledger at the whole extractor runtime. It is not free merely because `C_static` is called trusted.

This is the reference-source mechanism in Akita Lemma 9.1 and Corollary 10.17, applied only after establishing the actual advice source contract. The current API and a binding slogan are not a substitute for that check. Neither a seed label nor an unauthenticated digest proves arbitrary supplied arrays equal the approved matrices. If installed commitment provenance or original-source extraction/linkage is unavailable, reject this profile. The deterministic alternative requires its own explicit model/admission decision, not an automatic security-preserving fallback.

## 6. Complete ordinary outer manifest

Generate a **new** manifest from the actual outer guest, input/advice capacities and complete ordinary proof schedule. The inner `33` roles, `Pmain=56`, `N=2^22`, 1,992 fold coordinates and three final reductions do not describe this outer proof.

For every outer source occurrence and every root/recursive/terminal level, emit:

- original handle identity and role (untrusted archive advice, selected trusted static-data advice, one-hot guest trace, and any explicitly selected additional program/field object), layout, polynomial/claim count, source encoding and alphabets, source and opening bounds;
- ordered group list and exact predecessor/successor state, including all setup-prefix sources introduced by ordinary offloading;
- every A/B/D/F/H map's field, ring degree, rank, width, coefficient-address map, slices, padding, compression path, source-comparison class and weak/strong/reference collision radii;
- setup stream provenance and actual joint-view distribution, table/security-policy version, source reuse and mixed comparisons, coefficient counts, expanded cache sizes and setup-prefix registry provenance;
- all scalar and lattice challenge sites, message order, query/degree/arity counts, abort and exceptional branches, bounded sampler law, grinding policy, terminal acceptance equations, and runtime-sensitive extraction factor/inventory.

**Important modern distribution distinction.** [`akita-types/src/proof/setup.rs:86–95,215–250`](../../crates/akita-types/src/proof/setup.rs) uses a fixed public seed whose first eight bytes encode `0xDEAD_BEEF_CAFE_BABE`, a paged SHAKE256 field stream, and A/B/D prefix views of that stream. Equal field-length requests share coefficients independently of ring shape. This is not the inner independent-role registry or its 512-bit modular sampler. The inspected source explicitly requires validating untrusted materialized matrices against their seed and notes that `AkitaVerifierSetup::from_parts` does **not** prove prefix-commitment provenance (`:139–166,195–211,276–304`).

A supported ordinary outer profile can retain its shared-prefix distribution, provided the emitted **joint-view** certificate matches Akita §4.3 and §10.2 and accounts all embeddings/collision classes. It cannot call overlapping maps independent or sum a fictitious 33-role inventory. Independently sample/provision the inner setup relative to the outer setup; for seeded modes require disjoint inner/outer setup and FS namespaces and demonstrate their concrete encoding separation. Changing the outer backend to independent per-role matrices would require a separately supported and certified ordinary profile, not a relabeling of existing keys.

The planner proposes a schedule; an independent deterministic validator reconstructs every shape, transition, collision envelope and limit before installation. A table label such as “128-bit SIS” or a schedule artifact digest is not the complete security check.

## 7. Outer security admission: supported ordinary proof, no entropy substitution

The outer prover must run ordinary Jolt stages 0–8 and the **complete** Akita root/suffix/terminal. Relevant source paths are [`akita/prover.rs:174–193`](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/prover.rs), [`stage8/akita.rs:360`](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-verifier/src/stages/stage8/akita.rs), and [`protocol/prove/root.rs:99`](../../crates/akita-prover/src/protocol/prove/root.rs). A proof whose stage 8 or suffix is paused is not an outer receipt, even if its bytes are short. At runtime use an actually implemented supported ordinary profile and its exact catalogue; do not substitute the unimplemented native accumulator terminal.

Two field contracts must be audited separately:

- **Inside the guest:** replay inner K sources/E challenges exactly, as §3 requires. This is possible as guest arithmetic even when the outer proof's native scalar field is K; it does not increase the outer Fiat–Shamir entropy.
- **For the outer proof:** record its actual native fields and every challenge sampler/bridge. Modern [`jolt-akita/src/adapters.rs:38,1341–1346`](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-akita/src/adapters.rs) uses the fp128 base field and a scalar transcript bridge. [`fp128.rs:110–155`](../../crates/akita-config/src/proof_optimized/fp128.rs) instantiates Dense/DenseBounded/OneHot with `Field` for both field arguments, while [`jolt-transcript/src/digest.rs:181–184`](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-transcript/src/digest.rs) squeezes 16 bytes for a scalar. These interfaces do **not** establish an outer full-E frontend or 256-bit bridge.

For each candidate calculate, using its actual theorem and samplers,

`outer_error <= outer_FS_loss + outer_setup_bias + sum(actual_outer_MSIS_advantages at whole extraction runtime) + other explicitly justified binding/composition terms`.

Record honest bounded-sampler abort separately from adversarial soundness error. Budget the **joint** system's FS queries, including bridge/grinding probes and composition/extraction accesses as required by the companion theorem; do not reuse the full inner budget independently for each layer without accounting. Setup-XOF recomputation is separate under domain separation, but its runtime is real. Preserve the selected common control/accumulator security policy, including the inner bounded-query policy `Qmax+1 <= 2^32`; any change must be explicit and common to both systems.

In particular, a base-field scalar charge has denominator approximately q, **not q²**. A 16-byte squeeze into an extension-field container still has at most 128 bits of input entropy. Under a simple `(Q+1)c/q` bound, using the allowed query ceiling near `2^32` cannot establish a `<2^-128` statistical contribution even for `c=1`; this diagnoses failure of that admission bound, not a claimed attack. The inner `<2^-137` statistical estimate cannot be copied to the outer layer. Challenge truncation, modular bias, a short Jolt-to-Akita bridge and any grinding credit each need their actual analysis.

**Admission outcome today:** the supported ordinary route is identifiable, but no matching numerical receipt-security certificate has been supplied. Do not announce an admitted 128-bit receipt. Admission requires either a supported ordinary profile whose complete actual bound meets the selected joint policy, or a newly implemented and reviewed ordinary profile (for example genuine K-source/E-challenge frontend, PCS and full-width bridge support) followed by the same admission procedure. That example is an implementation prerequisite, not a profile claimed supported now. A weaker outer bound must be disclosed and separately authorized, not silently selected to make the wrapper appear complete. This document does not choose a weaker security target.

## 8. Concrete capacity and profile admission procedure

Admission is a finite offline procedure, not a guessed trace exponent:

1. **Freeze the language.** Fix canonical archive grammar for `1..16` segments, fixed inner profile and final decider. Derive a maximum serialized byte count `Amax` from every message count and canonical encoding, including all frontend/domain/update/final data, lengths and headers. For variable-length encodings use maximum accepted sizes, not honest averages. Every parser loop has a checked static bound and rejects lengths before allocation; EOF is global.
2. **Freeze the guest artifact.** Fix executable/code-image size, compiler, virtual instruction expansion, arithmetic implementations, statement/static-blob codecs and application preprocessing; fix the XOF implementation too for the deterministic alternative. The actual setup/registry and application are fixed across runs, not rebuilt with each archive.
3. **Bound work and memory.** Derive safe worst-case cycles for accepted archives and bounded rejecting paths, including authenticated static-data reads, proof-dependent matrix/table calculations, E arithmetic, parser work and the whole final decider; charge coefficient generation inside the guest only for the deterministic alternative. Fix guest heap, stack, RAM/address range, advice read limits and interpreter/instruction expansion limits; use checked arithmetic. Streaming can reduce resident setup memory but may increase cycles. No unbounded rejection-sampling or proof-controlled allocation belongs in this guest.
4. **Declare I/O limits.** Install `max_input_size` for `x`, exact output size, `max_untrusted_advice_size >= Amax`, and in selected mode `max_trusted_advice_size >= |Sbytes|`. Determine actual power-of-two word and physical advice arities. The deterministic alternative has no trusted-advice setup source; both modes have untrusted archive advice. Public-input and advice capacities are not interchangeable.
   The exact `|Sbytes|` is still missing: 74,907,648 counts only inner matrix coefficients, not canonical program, static prepared tables, descriptors or framing. Emit each blob region's offset, length, codec, derivation rule and content-provenance check; account any resulting increase beyond the minimum `2^24`-word advice domain.
5. **Choose a supported outer trace.** Select a power-of-two outer trace capacity with all required reserved padding and instruction expansion, then derive the actual outer one-hot column shape, polynomial arities, chunk configuration and RAM/code parameters from the compiled guest. Do not reuse `Nseg`, `Pmain` or the inner receiver capacity. Check the Jolt host/preprocessing numeric widths and all address conversions.
6. **Generate the full ordinary schedule.** Provision the grouped source shapes in the actual canonical order and exact complete suffix; reject missing catalogue rows or incompatible advice/program/field modes. Modern [`preprocessing.rs:90–143,254–270`](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-prover/src/akita/preprocessing.rs) explicitly derives advice physical arities and grouped schedule parameters, so merely raising a byte limit without regenerating the shape is not sufficient. Include any ordinary setup offloading in finite closure.
7. **Emit and independently validate §6–7.** Recompute matrix/sampler/security/shape certificates, verify executable/setup provenance and source linkage, and freeze all manifests. Reject if any public bound, collision class, entropy term or supported transition is missing. A smaller workload may only be chosen by explicit profile change, not by silently dropping accepted inner instances.
8. **Implementation acceptance after research.** Demonstrate the complete prover/verifier on accepted and adversarial archive cases, bounded resource behavior and paired control measurements. This document did not perform those checks. Successful execution is not a substitute for the algebraic/security certificates.

Exact presently missing numeric inputs are: `Amax` from an implemented canonical archive codec; compiled guest bytecode/read-only image and virtual-instruction expansion; worst-case trace/cycle count and live heap/stack/RAM bounds for the chosen setup access strategy; actual public I/O byte limits; resulting advice/trace physical arities and columns; complete ordinary schedule/terminal/setup-prefix catalogue rows; expanded outer coefficient/cache sizes; per-site scalar/lattice counts, actual outer bridge entropy and sampler/abort/grinding parameters; outer shared-view and optional advice reference collision envelopes; complete extractor/composition runtime and quantified MSIS advantages; and measured cold/warm receipt size, verification/proving time and memory. The inner setup byte count and fixed `T<=16` do **not** determine any of these by themselves.

## 9. Fair cost accounting and receipt scope

Report separate ledgers for **installation**, **per-run proof generation**, **retained prover state**, **receipt transmission**, and **external verification**, for both accumulation and the completed-segment recursive control.

| Cost | Deterministic guest setup | Installed committed setup |
|---|---|---|
| Inner setup provisioning/validation outside guest | Fixed registry/seed provenance and explicit ideal-XOF law; reusable caches allowed | Independently sampled or approved-law matrices, canonical program/static preparation, plus constructing/authenticating `C_static` and its original source/hint |
| Setup work inside every guest execution | Exact XOF expansion/recomputation, canonical arithmetic and matrix evaluations | Authenticated advice reads/decoding and all proof-dependent matrix/table evaluations; no free proof-dependent preprocessing |
| Outer committed inputs | Archive advice and trace, plus selected ordinary objects | Same plus a large original trusted-advice source and all its claim reductions/openings |
| Outer preprocessing | Guest/program, complete schedule, matrices/transforms and any certified prefix registry | Same with enlarged source geometry; advice commitment/reference provenance is extra |
| Prover memory/storage | Archive, inner caches, full outer trace/witness and ordinary proof buffers | Also trusted source words/digits/hints and related outer setup/proof buffers |
| Verifier installation/cache | Approved outer preprocessing/keys and application/profile record | Also expected `C_static` and its authenticated derivation record; expanded inner matrices/static blob need not be kept online |
| Per-run verifier/network | Compact `x`, ordinary final receipt and permitted metadata; no archive replay | Same, using installed commitment rather than transmitted setup arrays |

Use the same preprocessing amortization count, cold/warm cache convention, hardware, threads, security policy, workload/public relation and capacity treatment on both sides. The completed-segment recursive control may likewise cache setup/program preprocessing and use the same authenticated setup access strategy, while its guest verifies **complete segment proofs** plus the identical continued-segment links/global boundary predicates. It is not a control if it proves independent jobs or stronger/different VM semantics. The accumulation branch still consumes instances/relations **before** full per-segment PCS proofs; the outer receipt does not change that target.

Each branch has its own actual schedule/shape: fairness does not mean forcing different verifier guests into fictitiously equal trace sizes. Do not credit accumulation with free offline matrix expansion while charging the control per segment, or charge the accumulator for every cold setup while granting the control a warm cache. Report preprocessing totals and amortized totals separately, and retain the cost of the O(T) private archive and its guest replay. A succinct delivered receipt relocates that work; it does not erase it. No measured runtime, final receipt byte count, concrete outer trace exponent, speedup or parity result follows from this design.

## 10. Direct references

- [Inner design](AKITA_INSTANCE_ACCUMULATION_DESIGN.md), [review record](AKITA_INSTANCE_ACCUMULATION_REVIEW.md), [compact profile](AKITA_INSTANCE_ACCUMULATION_COMPACT_PROFILE.md), [matrix registry](AKITA_INSTANCE_ACCUMULATION_MATRIX_REGISTRY.md), [global manifest](AKITA_INSTANCE_ACCUMULATION_GLOBAL_MANIFEST.md), and [segment lowering](AKITA_INSTANCE_ACCUMULATION_SEGMENT_LOWERING.md): exact approved native statement/setup, not a receipt theorem.
- [Akita paper](SOURCE_PROVENANCE.md#akita), §4.3 (flat setup distribution and sampling law, lines 1733–1767), §9.1/Lemma 9.1 (installed canonical prefix provenance, lines 3126 and 3174–3176), §9.3 (complete admissible schedule), §10.2 (shared setup collision inventory), Corollary 10.17 (source uniqueness including canonical references, lines 3813–3825), and §10 security theorem: ordinary source linkage and runtime-sensitive security, not permission for a caller-supplied trusted label.
- Modern code links in §§5–8 are source evidence for setup/context/advice/full-proof interfaces and their limitations. In particular, the `from_parts` provenance caveat and default K-scalar transcript are admission-relevant; neither is papered over by the receipt architecture.
