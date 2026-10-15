# Independent review of the Akita accumulation design

Reviewed document: [AKITA_ACCUMULATION_PROTOCOL.md](AKITA_ACCUMULATION_PROTOCOL.md).

## Verdict

Two independent agents reviewed distinct obligations:

- **Mathematical/security review:** the elementary public arithmetic protocol and its corrected classical ideal-random-oracle lifetime theorem are sound under exact verifier factorization and fixed authenticated lane/profile assumptions. Deployment as a lattice execution receipt remains conditional on the source/outer argument and receipt hypotheses.
- **Source/scope/integration review:** the document correctly distinguishes completed-proof conjunction, continued execution segments, native/private witness accumulation, and online IVC. Source-pin, acceptance-boundary and cost distinctions are supported by inspected repository code and paper passages. No remaining significant finding was reported in the assigned sections, including the final additions to §13.

**Implementation admission remains CLOSED.** Neither verdict proves a continued-segment frontend, a native bounded-witness construction, a concrete-hash compiler theorem, or performance parity. Mathematical review is not deployment approval. The user subsequently selected native instance/relation accumulation before a full segment PCS proof; this review covers only the now-unselected public Akita-LVA route.

## Findings and corrections

| Finding | Why it mattered | Correction | Final status |
|---|---|---|---|
| P1: frame-digest accumulation keys missed prequery hits | An adversary could query weights at a guessed digest before fixing a complete frame. A later binding-hash hit need not be a collision between two frames. The original first-touch argument omitted that event. | Every indexed accumulation-oracle key now contains the full injective frame. The unnecessary frame-binding digest and its collision-only charge were removed. Profile/setup/receipt charges remain separate. | Mathematical reviewer re-read §§6.1/8.3 and explicitly closed the finding. |
| P2: earlier native/private preference attributed to the user as a mandatory target | The user requires correct lattice-based accumulation and recursive aggregation replacement for zkVM use, but did not separately impose private accumulation or zero knowledge. This could add a false implementation blocker. | §§1/12 now identify native/private folding as an earlier research preference, required only if selected. The actual continued-segment semantics remain a separate obligation. | Integration reviewer re-read the corrected sections and explicitly closed the finding. |
| Precision: receiver source map not fully identified | A general Akita gadget decoder is not an identity source map. A candidate equality must use the actual receiver projection. | §12.1 explicitly requires a certified identity/one-digit receiver source map; fresh original handles preserve their exact separate maps. | Mathematical reviewer confirmed the correction. |

The full-frame hash correction permits verifier-owned incremental-prefix cloning only as an optimization of the exact specified full-message hash. A prover-supplied hash state or a transmitted prefix digest is not an admissible replacement.

## Mathematical checks confirmed by review

- Strong completeness covers every canonical decider-valid predecessor, not only honest history. Full-field state avoids requiring a false norm-closure statement for accumulated vectors.
- Independent weights are assigned to each native occurrence and each channel. Two channels do not fix deterministic cancellation caused by sharing one occurrence weight.
- The nonzero sampler's maximum atom is computed from its actual nonuniform distribution; it is not asserted to be exactly uniform.
- Residual equations are flattened over the coefficient field. Nonzero base-field scalars act injectively even when the negacyclic quotient ring has zero divisors.
- Full-field decision must not reuse the small-response CRT kernel without a valid capacity bound. Certified digit-plane decision is an exact field identity, not a new range proof or commitment.
- Original Akita extraction does not automatically yield a canonical short source. Finite scheduled-tree extraction does not establish a uniform adaptive native accumulator extractor.
- Nova, Lova, LF/LF+, Neo/SuperNeo and nonhomomorphic/hash-based accumulation papers are used only with their stated relations and hypotheses. No homomorphic or Merkle/code commitment is silently substituted for Akita.

These findings concern the specified public protocol. They do not certify a not-yet-implemented retained-verifier API.

## Source and application checks confirmed by review

The compatible inspected conjunction pair is:

- Jolt [`5c65167e269fdfc2917ecb82f420d349b18b4c87`](https://github.com/a16z/jolt/tree/5c65167e269fdfc2917ecb82f420d349b18b4c87), `wrap/fast-recursion-blake`.
- Companion Akita [`d0ee56a0b12db66b4c46a87c527169737694c4c7`](https://github.com/markosg04/akita/tree/d0ee56a0b12db66b4c46a87c527169737694c4c7).

The native terminal A comparison is only one acceptance obligation. Eq.164/Eq.165, response checks, setup opening, trace checks and final grinding-transcript completion must remain enforced. The original full verifier must continue to reject invalid proofs; a pending result cannot be exposed as successful verification.

The inspected wrapper proves independent completed-proof conjunction, not continued VM execution. The existing acceptance risks are recorded in design §11: zero-count acceptance, missing accepted-result enforcement in the outer runner, incomplete stream/padding policy, setup/cache provenance, application statement binding and output normalization. They were identified by source inspection, not by an executed malicious acceptance.

Continued segments require a sound frontend for real PC/register/RAM/control/advice/I-O boundaries, initialization, ordered adjacency, legal instruction cuts, active-prefix padding and successful halt. The public accumulator can be applied to completed proofs of that relation if supplied; this does not itself establish online IVC.

## Evidence limits and next gate

The parent exercised exact arithmetic examples and the full-frame hashing optimization. The hash check compared 42 streamed-prefix-clone computations with complete-message Blake2b512 computations, including compression-block boundaries and the maximum uint64 block index. No Akita/Jolt build, source regression, guest execution, outer proof generation or benchmark was run. Runtime compatibility and performance parity remain **unobserved**, not failed.

The target question is resolved: the user requires consecutive VM segments and explicitly rejects accumulating already completed segment Akita proofs. The next design must expose Jolt's pre-PCS instance and reuse an admissible Akita reduction to combine it with the predecessor relation. Freeze the bounded receiver domain, closure certificate, original-handle bridge, adaptive soundness/extraction argument, security profile and complete receipt contract before implementation. This public-protocol review does not establish those native properties.

No implementation branch, source commit or push was made. No implementation skill was made stale by this research-only change.
