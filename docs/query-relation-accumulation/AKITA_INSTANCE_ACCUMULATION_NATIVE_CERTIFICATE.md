# Native Akita accumulation certificate and optional compression

## Selected scope

The selected deliverable is a native accumulation certificate for the claimed Jolt execution. It accumulates segment instances before their full PCS proofs. A final ordinary Akita proof is **not mandatory**. The certificate may have a linear update archive; a succinct SNARK wrapper is optional and must be evaluated separately.

This scope follows the user's clarification. Earlier notes that require an ordinary K/K outer receipt describe an unselected wrapper route, not a prerequisite for the native certificate.

There are three distinct objects:

- **Accumulator instance:** the current reduced relation instance and its original-source provenance.
- **Native certificate:** the evidence needed to verify the claimed execution's segment instances, their accumulation transitions, and the final decider.
- **Optional compressed proof:** a proof of acceptance of the native certificate verifier. Its protocol and assumptions need not be Akita's.

A small accumulator does not by itself imply a small independently verifiable execution certificate. The verifier must establish that this accumulator is the result of the claimed transitions, not an unrelated satisfiable instance chosen by the prover.

## Native verifier contract

Let the public statement be

$$
S=(v,\mathrm{pp},\mathrm{program},\mathrm{runID},\mathrm{input},\mathrm{output},\mathrm{panic},T,N_{\mathrm{expanded}}),
$$

where the parameters and program are approved inputs, not proof-selected replacements. Let a certificate contain ordered segment contexts and frontend/source-check/normalization frames, together with the complete native accumulation proof stream. Intermediate contexts are untrusted certificate data; they are not automatically trusted merely because they are encoded or used to create an installation.

The direct verifier must:

1. Decode under an explicit supported finite resource profile; reject invalid lengths, encodings, unsupported profiles, omitted records and trailing bytes.
2. Validate the program, public IO, initial state, terminal state and complete context chain. Match shared RAM commitments exactly, preserve original signed/noncanonical RAM representations, and check registers, successor/entry semantics, segment indices and expanded-row counts.
3. Derive the actual frozen native plan from approved setup/profile data and $T$. Check the canonical plan field, not a matching substring in an unrelated IO payload.
4. Execute the first K frontend on its installed compiler: actual stages 0–8, Main physical-zero and PC checks, Common-K normalization, authenticated full instance seal and native authorization.
5. Perform the first native root update with the authenticated frontend obligations and the prescribed zero receiver.
6. For every remaining segment, reenter the next K frontend through the sealed loan on the **same** compiler, verify its actual frame, seal and authorize its obligations, and perform the corresponding native update.
7. Verify the final receiver domain/normalization and the selected existing recursive suffix and terminal equations. These terminal reductions operate on the accumulated relation; they are not completed per-segment recursive proof aggregation.
8. Require exhaustion of the finite frontend/native event inventories and exact EOF of every input stream.

The actual two-segment driver has completed native proof generation and verification, including the selected three-row suffix, terminal equations, event exhaustion and native EOF. The v2 run also encoded the complete archive, decoded its actual bytes through the bounded nested codec, and accepted through the public native verifier. Container decoding alone was not used as the decider.

The implemented public consumer currently admits the explicit finite compact profile: $N=4096$, RAM size 65,536, 56 Main columns, transparent/full-public execution, no advice and no `field-inline`. It derives plans for one to sixteen frames; the executed native case has two. Other configurations are unsupported, not silently reinterpreted. This is not yet a complete replacement of every ordinary Jolt mode or an admitted numerical-security profile.

Public transport uses `receipt::frame_codec::{encode_native_frame,decode_native_frame}`, `receipt::archive::Archive` and `receipt::native::verify_native_certificate`. The codec derives the actual 140-round Stage 5 limit from approved instruction dimensions, separately bounds coefficients/claims/commitment payloads, checks canonical K encodings, and charges nested vector allocation before the first element. The final static boundary review found no surviving concrete defect after the Stage 5 limit and VM-ceiling repairs; the actual v2 byte-transport run then passed native verification and its adversarial checks.

## Lattice-specific obligations remain

Allowing a larger certificate does not remove the need for:

- original-source and matrix-view compatibility, including original A/B/F/H and decoder paths;
- fixed-size receiver closure with the prescribed coefficient/norm bounds;
- fresh source domains and physical/virtual zero semantics;
- bounded extraction from the complete native suffix;
- correct frontend-to-native opening linkage;
- a finite, correctly framed challenge/sampler law and its abort accounting;
- sound composition across all repeated roots and shared setup roles.

Do not infer ordinary homomorphic folding from the existence of a lattice commitment. In particular, unrestricted field coefficients can destroy shortness. The selected protocol uses its reviewed packing, decomposition, range and coefficient-relation machinery instead of assuming that componentwise commitment addition is sufficient.

Lova states that folding must return to the same bounded relation and that both completeness and extraction norm growth must be controlled. Its naive random-linear-combination example fails these conditions; its remedies include decomposition and exact norm checks. See [Lova, §§1.2.1–1.2.4](SOURCE_PROVENANCE.md#lova), especially the bounded SIS relation and discussion of inverse challenge differences. This is a relevant obligation, not a license to substitute Lova's commitment algebra for Akita's.

## Resource limits versus certificate size

The observed 7,917,436-byte failure concerned the **expanded installation description**, not the proof/certificate size. The former 4 MiB record and 16 MiB total-transcript caps were implementation limits. No cited paper requires them.

The current selected finite prototype resource profile retains the full expanded framing rather than changing its semantics:

| Resource | Selected cap |
|---|---:|
| Segments | 16 |
| Context/record/seal/event payload | 16 MiB |
| Retained transcript bytes | 512 MiB |
| Records and sites per snapshot | $2^{20}$ each |
| Native blocks per stream | 71 |
| Evaluated compiler component calls | $2^{24}$ |
| Read bits | $2^{28}$ |
| Frontend coordinates and powers | 256 each |
| Schema nodes and depth | 131,072 and 16 |

These caps are bound before the frontend challenges and enforced consistently. They are not security levels, observed certificate sizes, or a promise that every sixteen-segment execution fits. Actual context/seal sizes, proof bytes, retained memory, compiler calls, sampler reads, prover work and verifier work must be measured and included in the comparison. A larger supported profile may be selected with the same complete accounting; it must not silently widen a sampler law or numerical admission claim.

The v2 resource profile derives its stream cap from the conservative sparse-shell envelope $40\cdot256\cdot7+40=71{,}720$ bits, rounded up to 71 tape blocks. The width-128 first draw always accepts, so this is conservative rather than the exact attainable maximum. It removes the former 64-block compiler cutoff before completion of the unchanged bounded sampler. The 256-attempt law, global read/component caps, honest-abort probability and full conditional-answer obligations remain explicit. The versioned profile changes transcript/challenge bytes; old-profile byte continuity and numerical approval are not claimed.
Standalone archive limits now use caller-approved finite total/component budgets and the actual u32 component-length representation, not a VM-derived 1 GiB total ceiling. An optional wrapper can impose its own separately justified VM/advice limits.

## Main coefficient-vector extraction: open refinement

The actual Main normalization declaration has 118 independently sampled K coefficients and declares a nested tree. The literal inventory therefore has the factor

$$
2^{118}\,3^{22},
$$

not the earlier proposed coalesced $119\,3^{22}$. Source-check coefficient vectors have the same distinction. The current resource audit keeps the literal factors; it must not silently replace them with a smaller tree.

A possible refinement is **conditional on a proved bounded common-original-source extractor**. Suppose the complete native suffix yields a bounded Main source $X$ for the original commitment $C$, and different accepted continuations with the same $C$ yield either the same $X$ or a correctly bounded Module-SIS collision. With the original claim roster fixed before $\boldsymbol\alpha$, define

$$
\delta_i=v_i-L_i(X),\qquad
P(\boldsymbol\alpha)=\sum_{i=0}^{117}\alpha_i\delta_i.
$$

For a fixed common $X$, $P$ is linear in the scalar batching coefficients. A nonzero $\boldsymbol\delta$ cannot vanish with more than $1/|K|$ probability under independent uniform coefficients. Ordinary sumcheck soundness can then link the batched claim to the final raw opening. The coefficients weight scalar functionals of the **same original source**; they do not create a lattice witness scaled by arbitrary field elements, so this argument is not unrestricted lattice homomorphic folding.

This does not yet prove the refinement: $X$ is extracted after these challenges and can depend on them. Required missing steps are original A/B/F/H/source compatibility, bounded same-commitment collision extraction across counterfactual continuations, accepted-density and conditional-tape analysis, a concrete extractor and its complete runtime/call bounds, and the treatment of all earlier source-check and frontend ancestors. Keeping the native Main Boolean prefix is important here. Omitting that prefix and then assuming a bounded common source obtained only from the frontend would reintroduce the circularity the domain-extractor note forbids.

Lova's coordinate-wise special-soundness discussion gives a useful comparison: a linear matrix challenge can admit a star of accepting transcripts rather than a full Cartesian tree, but its norm/extraction hypotheses must be established for the actual protocol. See [Lova, §1.2.2–1.2.3](SOURCE_PROVENANCE.md#lova). This comparison alone is not a proof that the current Akita declaration has a coalesced extractor.

## Optional wrapper

A wrapper may prove acceptance of the direct native verifier over the larger certificate. Its public statement must still bind the approved setup/program, run IO, counts and success condition. The wrapper's soundness and any zero-knowledge property are additional requirements, not consequences of the lattice accumulator.

[Nova, §§1.1–1.2](SOURCE_PROVENANCE.md#nova) distinguishes folding NP instances from SNARK-based IVC and then separately discusses succinct compression. In particular, its augmented-circuit construction checks prior folding, and its succinct proof of the running witness is an additional mechanism. The distinction supports separating accumulation, execution-history linkage and compression. Nova's additive-commitment theorem does not automatically apply to Akita.

A pairing-based Groth16 wrapper adds a non-post-quantum assumption. A lattice-based or otherwise post-quantum candidate requires its own composition/security review. The published [Akita](SOURCE_PROVENANCE.md#akita) Fiat–Shamir theorem is classical ROM; no complete QROM or end-to-end post-quantum theorem is claimed for this prototype.

## Current acceptance gates

- Verified: actual native prove/verify, altered statement/frame/frontend/native/terminal rejection, truncation and EOF checks.
- Verified for the selected two-frame profile: complete canonical archive transport and direct execution/transition/final-decider linkage.
- Derived and independently reviewed: bounded original Main extraction from a supplied complete native accepting tree, with the native Boolean prefix retained. This is not an extractor for the actual chained compiler.
- Open: concrete conditional-answer/tree-building reduction, full ancestor/alias composition, usable extraction/runtime-priced MSIS and whole-profile resource admission.
- Measured: independently generated continued frontends plus two complete ordinary PCS proofs passed after the point-convention repair. This control has no outer aggregator; recursive-aggregation parity remains unestablished.

Absence of an ordinary Akita outer receipt is not a blocker. Absence of native soundness or actual complete verification remains a blocker.

## Observed native execution and ordinary control

The repaired v2 run passed the complete canonical archive, public native verification, native adversaries, independently generated ordinary frontends, both complete ordinary PCS proofs and ordinary adversaries. Both original Main owners and all three signed/noncanonical RAM handles were retained. The public native verifier used decoded certificate data, not private source handles.

| Measured native object | Result |
|---|---:|
| Segment active rows | 155 and 156 |
| Root updates / final relation folds | 2 / 3 |
| Native events | 1,971 |
| Native proof stream | 357,504 bytes |
| Complete archive, including contexts/statement/framing | 405,409 bytes |
| Archive encoding/decoding | 0.00115 s |

| Measured path | Producer | Honest verifier |
|---|---:|---:|
| Native accumulation | 213.84 s | 103.94 s |
| Independent complete ordinary segment control | 43.95 s | 1.60 s |

The ordinary producer timing includes independent source/endpoint cross-checks, so it is conservative against that control. It produced 192,233 PCS bytes and 47,698 frontend-component bytes, plus a 156-byte statement; these are component counts, not a measured complete framed ordinary archive. Native adversarial verification (106.31 s) and ordinary adversarial verification (0.0181 s) are excluded from honest-verifier timing. Setup, resource-audit and build time are also excluded from the table.

The former ordinary failure was an integration-boundary error: Jolt points are MSB-first, while the ordinary PCS physical coefficient indexing is low-bit-first. The driver now reverses points on both parties and independently evaluates all three original sources before proving. It changes no source coefficients, commitments, paths or claimed values.

The native archive contains no completed per-segment PCS proof and uses no wrapper. **The selected prototype is much slower than this unaggregated control and is not approved for production cutover.** This is not a proof that parity with every possible recursive aggregate is impossible. A separate baseline audit found no matching recursive continued-segment aggregation driver in the inspected current/recorded branches; the historical Akita outer guest checks independent complete-proof conjunction, not the same continued execution.

The actual resource report was streamed to `/tmp/akita-composed-resources-v2.json` (26,229,796 bytes). It records 1,089 fresh frontend K coordinates, 1,315 native E2 coordinates, 312 shells, twelve numerical matrix targets and the shared prefix of 798,720 coefficients. All scheduled matrix capacities cover the derived comparison radii. Its flags remain `numerical_security_admitted=false` and `optional_wrapper=false`. The report describes prerequisites, not an admission certificate.

Verification evidence: the complete repaired integration test passed; the prover/verifier library suites passed 11/91 tests; reused Akita CPU/transcript suites passed 539 tests with four ignored. The separate pre-PCS driver passed 240 original-source leaves, two raw normalizations, 112 column values and 20 rejected mutations. These checks are not cryptographic soundness proofs.

**Known repository defect:** checking existing `akita_e2e` with `akita,transcript-d1,prover-fixtures` failed with ten E0061 errors: whole-run callers still omit the new transcript/emission arguments. The selected native path does not fix that unfinished caller migration. Cargo also reports ignored workspace `default-features` settings and an unused `instantiate_val_evaluation` macro/reexport. No clean full-suite or warning-free claim is made.

Related notes: [native source-tree theorem](AKITA_INSTANCE_ACCUMULATION_NATIVE_SOURCE_TREE.md), [derived sampler budgets](AKITA_INSTANCE_ACCUMULATION_NATIVE_RESOURCE_BOUNDS.md), [native reuse](AKITA_INSTANCE_ACCUMULATION_NATIVE_REUSE_PROFILE.md), [Main domain/extractor](AKITA_INSTANCE_ACCUMULATION_MAIN_DOMAIN_EXTRACTOR.md), [installation/resource binding](AKITA_INSTANCE_ACCUMULATION_INSTALLATION_BINDING.md), [composed resource artifact](AKITA_INSTANCE_ACCUMULATION_COMPOSED_RESOURCE_ARTIFACT.md).
