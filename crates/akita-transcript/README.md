# akita-transcript

Ordinary version-eight indexed proof channels and the separately named native
D1 transcript compilers for Akita.

## Ordinary v8 channel

`ProverChannel`, `VerifierChannel`, and `FoldPreview` own the public transcript,
installed event/request grammar, and sticky error state. Constructors require
an `InstalledContext` derived from a verifier-approved `InstalledRegistry`.
The registered unique index names exact fixed installation data; it is not a
prover-supplied digest or an AQRA catalog-admission certificate. The augmented
instance must bind the complete sealed frontend/query handoff and endpoint
roster. Checking that public statement and frontend validity is the caller's
responsibility.

The actual ordinary provider is `akita-pcs/indexed/v8/shake256`, using RustCrypto
SHAKE256. Legacy `transcript-blake2b`/`transcript-keccak` feature choices do not
select another v8 public-randomness path. The separately exported legacy sponge
is not a v8 channel and cannot mutate its private prefix.

Literal oracle keys use an injective version/domain/installation/session/instance
header, followed by ordered PUBLIC/PROOF/COMPLETED frames and one REQUEST frame.
The implementation caches only the unfinalized SHAKE absorb state for that
literal prefix. A request clone absorbs the complete site, ordinal, operation,
descriptor and declared raw length before squeezing; no digest/seed replaces
the key. Complete canonical decoded results, including all 32 predicate bytes,
enter the live prefix before return. Raw slack remains in the formal finite
answer, not in the completed-history convention.

The selected AQRA scalar operation is checked LE128 A7F7 with at most 128
canonical candidates (2048 declared raw bytes), and its initial profile remains
E=K. Separately tagged `PrimeScalar` and `FieldVector` operations preserve generic
prime/extension arithmetic component contracts, including F32/E4. Canonical
encodings up to16 bytes and modulus bits up to128 are supported. Each coordinate
reserves128 independent fixed16-byte candidate slots; high bytes and masked bits
remain raw slack. Vector requests bind the exact modulus, degree and ordered
limb owners, with one whole canonical vector completion. The shared
`field_request_spec` plans the exact admitted operation and codec lengths.
This generic support is not an AQRA extension-field/security certificate.

Nonzero grinding uses its own predicate request after a canonical bounded nonce.
Zero-bit grinding creates no nonce/predicate event; the caller still audits its
existing logical GrindingPlan. Failed selected live predicates poison the owner;
failed preview predicates reject only that nonce branch.

`IndexedChannel::execute_indexed` exposes only the shell-level decoder
capability. `FiniteAnswer::read_exact` and `skip_exact` enforce the entire raw
budget without a full-answer allocation. Skipping generates/discards intermediate
SHAKE bytes; a successfully decoded family's unobserved final tail need not be
squeezed. `CompletedResult::write_all` streams the entire installed canonical
polynomial codec, not a hash or support-position ordering. Shell planning,
filter checking and canonical tuple encoding remain in `akita-challenges`.
Ignored read/write errors, decoder rejection, incomplete output and failed
transactions poison the channel. Acceptance requires channel `finish` (or
verifier `check_eof`), the entire installed grammar, and exact proof EOF; callers
must additionally complete their algebraic and GrindingPlan obligations.

This is a conventional concrete instantiation of the approved indexed-ROM
syntax. It is not a concrete-SHAKE knowledge theorem, a QROM/ZK claim, or an
admission of a complete AQRA schedule.

## Native D1 accepting boundaries

The selected native strong-check adapter retains the existing D1 compiler and
requires an explicit execution capability: `InnerExecutionRegistry` for H1610
or `OuterExecutionRegistry` for SHAKE256. Neither has default successful
acceptance, and the same requirement applies to `NativeTranscriptBorrow`.

For the outer family, `Registry::record` already admits the complete record at
its compiler-owned ordinal, and `Registry::request` validates the complete
request against installed sampler limits. These checks are not publication:
schema prebinding, repeated validation, and a subsequently failed append cannot
advance the protocol. Native closure calls
`OuterExecutionRegistry::accepting(completed_events, prefix)` with the actual
compiler count and borrowed canonical completed-record prefix. The installed
registry must require its entire finite protocol and terminal obligations,
including source handoff where required. No additional transcript, sampler,
result copy, or production registry is introduced by this capability.

Raw D1 `Transcript::finish` only closes an active compiler; it does not certify
native protocol completion. `NativeStrongTranscript::finish` first requires
registered acceptance, and any failure poisons that same compiler. The run owner
still discharges terminal algebraic obligations and exact proof-reader
exhaustion. Component-test registries certify only their explicitly finite
component protocols, not an accumulation run.

## Logging Checks

With `logging-transcript`, successful context hooks observe the same canonical
logical event transactions. Logging never supplies site ownership or affects
keys. Ordinary scalar/predicate/shell requests use their installed request
descriptors directly, rather than diagnostic challenge records.

Focused compiler tests cover independent materialized-literal-key agreement,
preview/live/verifier equality, changed owners/results, finite reader and sink
failures, canonical decoding, version/profile rejection, budgets, and EOF.
Integration owners must migrate every ordinary caller to the installed v8
grammar before exercising the PCS protocol tests.

For the full design, see [`book/src/how/transcript.md`](../../book/src/how/transcript.md).
