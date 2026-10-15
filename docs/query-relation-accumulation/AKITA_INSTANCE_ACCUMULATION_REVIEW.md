# Native Akita instance accumulation: independent review record

## Scope and decision

The selected target is **old-instance plus fresh-segment-instance accumulation before a full per-segment Akita PCS proof**, for consecutive VM segments. Completed-proof terminal residual accumulation in `AKITA_ACCUMULATION_PROTOCOL.md` is unselected. Its earlier review is not approval for this target.

**Decision: final independent integrated review admits the native compact T<=16 archive core.** The user subsequently selected a succinct standalone final receipt; its [new design](AKITA_INSTANCE_ACCUMULATION_RECEIPT.md) has separate conditional-composition, concrete-hash-model and outer-profile gates and is not yet implementation-admitted. Neither approval nor the later local branch creation is proof execution, performance parity, ZK, QROM, growing-depth or overall concrete-128-bit security. Earlier conditional reviews below remain scoped evidence.

Reviewed documents:

- [Native design and reuse map](AKITA_INSTANCE_ACCUMULATION_DESIGN.md).
- [Same-handle bounded-source bridge](AKITA_INSTANCE_ACCUMULATION_BOUNDED_BRIDGE.md).
- [Adaptive and finite-depth soundness](AKITA_INSTANCE_ACCUMULATION_ADAPTIVE_SOUNDNESS.md).

## Independent source review

`NativeSourceReview` returned `overall_correctness: correct`, with no remaining findings, for the corrected source/scope sections.

Its verified conclusions:

1. Current Jolt stage 8 first reduces its main trace and auxiliary claims, then calls `PCS::prove_batch`. The immutable `0791f79` variant instead preserves the main trace's native columns through `one_hot_trace_claim`; auxiliary reductions remain necessary. The intended cut is immediately before the full PCS call, after the branch's actual frontend claim reductions.
2. Current Akita root proving completes one `prove_fold`, then calls the suffix. The safe candidate pending cut is after the **complete** fold, including strong/ring/range/relation checks and the verifier's final Stage-2 consistency validation. `OuterPayload` and `TerminalInnerState` are different states.
3. The existing suffix accepts the scheduled witness plus optional registered setup source, not arbitrary old-plus-fresh application groups. Proof-scope/lifecycle ownership is tied to the admitted schedule; ending or aborting that scope invalidates its authority. A valid accumulator needs explicit schedule and checked ownership transitions, not an early return that bypasses the lifecycle checks.
4. The source generations must not be conflated. Jolt `629ed77` pins Akita `e2c49ed`, not current local `2607265`. The modern `0791f79`/`8357433` pair supplies relevant native-column interfaces. Legacy `5c65167`/`d0ee56a` supplies an actual outer recursion example, but verifies completed independent jobs and uses older source layouts. It is not a continued-segment implementation.
5. Consecutive segments need explicit register/RAM/control boundary relations. The corrected target preserves the existing Jolt proved whole-run relation, not a stronger emulator/MMIO/halt theorem. Register initialization becomes an incoming endpoint; internal `NextIsNoop`/PC shifting uses the actual external successor. Initialization and the original global final CPU/public-I/O convention occur once. Proof-side RAM is a prefix recurrence, not a device snapshot.
6. No reviewed passage replaces lattice commitments with hash commitments, assumes homomorphism of Akita's compressed commitment, or claims completed per-segment proofs are the new accumulator inputs.

## Independent mathematical review

`NativeMathReview` returned `overall_correctness: correct`, with no remaining findings, for the corrected conditional research reports.

The reviewer checked:

- Same-original-handle range/zero interpolation, including the degree of the restricted-mask MLE construction.
- The distinction between a bound on a challenge multiple of a source and a bound on the source itself.
- Source-projection uniqueness and mixed weak/bounded collision radii.
- Little-endian bitplane reconstruction and the exact field constant `4294944758` for the 128-bit weight normalization.
- Complete Eq. (109) successor accounting, and the distinction between honest decomposition bounds and verifier-certified alphabets.
- The displayed structural extraction recurrences and their multiplicative depth cost.
- Nova's fixed-depth definition and Akita's actual tree, query, error and reduction-runtime accounting.

### Corrected scope issue

The initial drafts wrongly made a uniform polynomial extractor in arbitrarily growing depth a mandatory implementation condition. The user did not require unlimited updates.

The corrected design distinguishes:

1. A declared finite supported `T_max`, with a complete continuation-replayed accepting tree, a complete bounded final decision once, and exact concrete security/runtime admission for every supported depth.
2. A stronger optional uniform-growing-depth theorem.

A fixed-depth cost `N^{O(T_max)}` can be formally polynomial while still giving unusable concrete security. Neither the word “fixed” nor Nova's fixed-depth definition supplies a practical Akita parameter profile.

### Local algebra versus remaining admission

The reviewer independently read Akita's one-fold proof and confirmed that its subtraction/interpolation algebra does not inherently require application groups to occur only at the global root. The paper describes that restriction as part of its analyzed schedule.

For a fixed supplied list of direct-coefficient identity-source groups, with all existing local matrix, unit-difference, alphabet, map and mixed-collision hypotheses, the conditional extension is:

1. Authenticate each strong-check descendant and recover its exact native equations.
2. Subtract coordinate siblings to extract weak witnesses for all incoming groups.
3. Reconcile the same original commitment-side source across range-prefix children.
4. Interpolate the range/zero predicates to restore the bounded source relation.
5. Compose backward from complete bounded final leaves for a finite chain.

The main design now writes this derivation in §8.1. It is not a claim that arbitrary packing, raw-source queries, fresh-source insertion or a new cyclic receiver are already admitted by current code. No fundamental impossibility was identified for this conditional local algebra. Avoiding multiplicative extraction at growing depth is a distinct stronger problem.

**Addendum:** the mathematical reviewer separately read the new §8.1 and confirmed that its written conditional derivation satisfies the local-proof requirement at this scope. Akita's carried B/F and shared D/H reconciliation occurs before sibling subtraction; the later cross-prefix comparison concerns only commitment-side sources. The remaining gate is exact instantiation and finite composition/security admission, not an unwritten elementary local argument.

## Retained-source query and concrete receiver reviews

`SourceQueryMathReview` reviewed the complete [source-query construction](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md) against live qmd Akita passages and returned `overall_correctness: correct`, with no findings. Its approval covers the S-linear raw digit MLE/carrier map, physical/virtual layout, retained response gadget, dual lift, unit subtraction, source-projection consistency and charged query channels. It confirms that the conditional direct-coefficient interface closes the algebraic observation gap without changing original A/B/F. It does not approve an implemented API or full-chain security.

`ReceiverProfileReview` independently reproduced the complete [generic receiver calculations](AKITA_INSTANCE_ACCUMULATION_RECEIVER_PROFILE.md), then separately reviewed its shared-response alternative. Both reviews returned `overall_correctness: correct`, with no findings:

- Separate occurrences: `7,989,248` successor cells plus `399,360` zeros in capacity `8,388,608`.
- Two explicit query channels per handle: `4,003,840` successor cells plus `190,464` zeros in capacity `4,194,304`.

The reviewer checked source bounds, the asymmetric full-field lift, raw challenge cardinality/unit condition, all listed matrix widths/rank-table entries, B/D versus binary F/H alphabets and the noncanonical-original-path completeness condition. The shared-response review checks both carrier equations and all partials while sharing only the justified z/A/B/F data. These are generic shape and table-model results, **not** a Jolt receiver, full-chain 128-bit certificate or measured speedup.

## Final decider, normalization and native reuse reviews

`SourceQueryMathReview` separately approved main §6.3's final-only bounded decider. The same-C range/domain prefix and pending ordinary claim reduce through degree-two multipoint sumcheck to one complete native opening. The terminal check multiplies by the public equality-weight sum; it never divides by a possibly zero factor. Root weak unit/product certificates remain necessary for cross-prefix source uniqueness. This is conditional on an admitted final PCS profile, not a complete chain theorem.

The same reviewer approved SourceQueryLift §8.1's **paid raw-point normalization**. Multiple endpoints on the same complete RawView can become one raw SQ-Lift channel after a real degree-two reduction and claim-separation tree. The later digit-axis refinement below converts the selected decoded ordinary queries to that same view without altering A/B/F; source traversal, prefix messages, errors and extraction factors remain charged.

`NativeMathReview` reviewed all of [range reuse](AKITA_INSTANCE_ACCUMULATION_RANGE_REUSE.md), relevant qmd passages and immutable Stage-1/Stage-2/domain code; it returned `correct`, no findings. Full-domain outgoing range plus a fused required-zero term can replace the duplicate old-range prefix in a **finite assembled history**. The degree-two restricted-mask term does not add an endpoint or raise the degree-three fused relation. This does not prove bounded local knowledge from a fixed arbitrary old transcript. The reviewer reproduced its saved-cell/row counts and confirmed its all-valid-source qualifications.

`NativeSourceReview` reviewed [endpoint memory cost](AKITA_INSTANCE_ACCUMULATION_MEMORY_COST.md) and returned `correct`, no findings. Exact private source-difference A/B updates require complete F recompression; unchanged noncanonical source/path coordinates must be retained. Existing portable hints lack the raw source/original inner-image digit material needed for the all-valid variant. Full-field endpoints avoid an unproved word-domain premise. Private cache reuse is available to both baseline and accumulator; raw-source proof scans and copying remain real costs.

## Continued-segment and Jolt-dimensioned reviews

The modern segment-interface review fixed reserved padding (`ell<N`, `N>=4096`) and CycleMajor-only support. It confirmed the native column/query shape, endpoint gamma identities, distinct PC-shift points, and the same-semantics baseline excluding accumulation-only source-range proofs.

The reviewer approved §7.1's **conditional Boolean-table concatenation lemma**: local prefix recurrences telescope across exact bound endpoints, internal external-successor checks restore adjacency, and global initialization/finalization apply once. It does not glue independent MLE evaluations or prove unaudited kernel coverage. Review found three stale bounded/u64 requirements after the full-field cutover; they were corrected to proof-field RAM/register values and conditional word-domain optimization. A follow-up review returned `correct`, no findings.

`ReceiverProfileReview` reproduced receiver-note §10's newly provisioned base-32 Jolt example and returned `correct`, no findings: 56 main columns, full-field RAM, **4087808** live cells at capacity **4194304**, or **3981312** with finite-history old-range reuse. It checked response reaches, all matrix-table bounds, original path preservation, both raw/ordinary channels, and the complete quotient-free/non-offloaded accounting. This approves arithmetic closure, not a shipped catalogue, full security manifest or speedup.

**Security-policy clarification:** independent review confirms that matching Akita's formal polynomial-tree/EPT policy and explicit runtime-sensitive MSIS advantage bound is a legitimate target. The user did not add a tight end-to-end 128-bit reduction requirement. Exact FS/query and collision inventories remain required; a 128-bit per-matrix attack-model label is not an end-to-end claim.

## Extension-challenge profile review

Three independent reviews covered [the explicit security profile](AKITA_INSTANCE_ACCUMULATION_SECURITY_PROFILE.md):

- `SourceQueryMathReview` approved the K-source/E-challenge separation and full-ring scalar-extended carrier. Each partial has 256 base coordinates; two S-valued planes share the same unit fork. It expressly distinguished this new interface from unmodified native Eq. (84).
- `ReceiverProfileReview` reproduced all response/rank/width counts: **6377472** live cells plus **2011136** zeros at capacity **8388608**, D width **19136**, 49 base-field ring rows, 184 fold coordinates. Conditional history range reuse gives **5951488** live cells, D width **15808**, 47 rows.
- `NativeMathReview` approved the conditional finite supplied-tree/ROM proof architecture and sufficient statistical bounds: the stated complete-manifest caps give FS error below \(2^{-149}\), and the illustrative setup contract gives bias below \(2^{-344}\). Runtime-sensitive MSIS advantages remain separate.

All returned `correct`, no findings. This is **not** approval of an ungenerated manifest, unchanged base-field frontend/final APIs, a concrete native catalogue, full end-to-end 128-bit security or performance. Remaining work is exact field/source/frontend/final lowering, complete sampler/tree/error/shared-view inventory and application instantiation, then implementation. A smaller receiver with base-field-only sumchecks cannot inherit this profile's FS bound.

## Evidence limits

Observed algebra checks: 263 bitplane/tensor identities, 36 generalized RAM endpoint identities over `F_101`, all 101 signed-base-four range-factorization values in that field, and an inverse-norm counterexample. These check elementary equations only; they do not prove cryptographic security or exercise a native accumulator.

The reviews used indexed papers and source inspection. No Rust build, native accumulation execution, cryptographic adversarial run or performance comparison was performed. No parity or performance failure is established.

## Remaining implementation gates

- Implement the reviewed exact continued-segment consumers, K-source/E-challenge reconstruction and full-width transcript for the selected matching source pair.
- Emit and check the physical-source, relation-row, complete-manifest, setup-role and fixed-capacity certificates specified by the reviewed documents.
- Preserve checked witness ownership, original noncanonical paths, complete transition archives and all native replay checks.
- Run the complete producer/verifier, behavioral adversarial regressions and the same-semantics recursive control before any accepted cutover.
- Measure the full archive, setup, frontend, update and final-decision costs. Stop if the required parity cannot be reached; no such performance result exists yet.

No storage-engine, wire-protocol, replication or build-system skill became stale: this work changed research documents only.

## Final native implementation-admission review

The following later reviews returned `overall_correctness: correct` with no remaining findings in their stated scope:

- `DigitAxisReview`: all 32 positional weights, ordinary-to-raw equality, unchanged P/B/F geometry, both E planes, all-valid noncanonical sources, and the paid independent-coefficient claim tree.
- `CompactProfileReview`: every root/suffix cell count, response reach, matrix radius/table comparison, and direct-terminal payload arithmetic.
- `DirectTerminalReview`: the 642 K equations, unit/product certificates and backward finite-chain extraction from the actual A-only terminal.
- `GlobalManifestReview`: global message order, error sums, mixed-tree polynomial bound, bounded unbiased samplers and conditional raw-tape sampling. Its follow-up approved the complete compact registry.
- `SegmentLoweringReview`: exact RAM address aliases, first-only initialization, last-only output, register/field endpoints, segment-relative bytecode entry, five distinct-point shift corrections, valid padded CPU equations, legal-cut metadata and physical source coverage.
- `IntegratedNativeReview`: the complete compact T<=16 construction and all of the above together. The accepted theorem is the final bound in the main design, with all continuation-replay cost inside every MSIS reduction runtime.

The selected root uses four original handles, P_main=56 and one channel per handle. Its **3555328** live cells fit capacity **4194304**; three full final reductions lead to a separately created A-only terminal. All 33 setup roles remain distinct where specified. The old-range prefix is retained in the generic local update; optional finite-history reuse is not silently substituted for that theorem.

### Defects found and resolved in the research design

1. The accumulator relation omitted its explicit binary-region predicate while its final/domain checks required it. `Bin_layout(w)=0` is now part of R_acc, preserving the declared all-valid-input completeness domain.
2. Existing semantic Booleanity cannot see physical address-zero cells of zero-omitting main columns. The new role declares those cells zero from birth and proves the column-specific union of these holes with inactive cells. Arbitrary broader Boolean handles retain the complete raw prefix.
3. Native bytecode entry is a whole-run global consumer at gamma^11. It now uses the authenticated segment-relative entry, with original program entry required only on the first segment.
4. The instantiated Noop flag dispatch, not the empty marker flag array, is required for the padding proof. Keeping DoNotUpdateUnexpandedPC resolves the sequential-PC row without weakening CPU constraints.

The frontend descent audit also established typed reconstruction of K semantic tables and total inspected exceptional branches; no new zero-challenge failure term or free E-valued witness is inserted. All these findings concern proposed accumulation changes, not a claim that existing whole-run Jolt is unsound.

Additional executed arithmetic evidence: the 32 digit-axis identities, exact shell cardinality, receiver/terminal/registry totals, both carrier planes and 700 restricted-mask identities. The reviews used paper/document/immutable-source inspection, not Rust execution. Implementation and performance remain the next gates.

## Succinct receipt extension

The user explicitly chose a succinct standalone final receipt rather than external O(T) archive replay. The proposed one-final guest reads the update archive and complete decider as untrusted advice, and actual approved inner matrices/program/static data through an installed original outer lattice commitment. It exposes compact public application data and canonical success1, not the archive or a per-run embedded executable. It still accumulates pre-PCS instances.

New separate notes specify [architecture](AKITA_INSTANCE_ACCUMULATION_RECEIPT.md), [conditional soundness](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SOUNDNESS.md), [setup](AKITA_INSTANCE_ACCUMULATION_RECEIPT_SETUP.md) and the [modern source map](AKITA_INSTANCE_ACCUMULATION_RECEIPT_CODE_MAP.md). [Two independent receipt reviews](AKITA_INSTANCE_ACCUMULATION_RECEIPT_REVIEW.md) found no actionable defect in the conditional theorem or interfaces. Both retain the explicit security-model, extraction/guest, outer-profile and performance gates.

Research found two distinct composition gaps: ordinary outer language soundness does not extract an accepting computational inner transcript, and fixed hash instructions do not implement a programmable ideal oracle. BCMS/BCLMS explicitly limit their non-black-box compiler model; live qmd Nova §1.1 calls concrete instantiation heuristic. The outer extractor's extra inner queries also cannot inherit the standalone Q32 bound for free.

The modern SDK recursion path is Dory/BN254, despite the lower-level ordinary Akita path existing at the same source revision. Its public/per-run-embedded archives, unapproved supplied preprocessing, zero-count acceptance and missing application-success enforcement are not the new receipt contract. Static setup advice uses a capacity-padded original source with externally installed C_static; this does not mean arbitrary supplied prepared caches are authentic.

Exact arithmetic checked 74,907,648 inner coefficient bytes, 9,363,456 u64 words, minimum 2^24 padded words and the base-32 depth13 unsigned-u64 reach failure. No source implementation or receipt execution accompanied these checks.

Later primary research supplies a [genuine AROM alternative](AKITA_INSTANCE_ACCUMULATION_ARITHMETIZED_ORACLE.md), but not a drop-in Akita theorem: its generic one-shot compiler needs an AROM-secure base knowledge argument, and the paper's existing knowledge lift requires straightline extraction. The modified receipt must bind a query accumulator and verify it with two external extended-oracle evaluations. Separate reviews found no actionable defect in this conditional mapping or [the carefully scoped oracle limits](AKITA_INSTANCE_ACCUMULATION_ORACLE_LIMITS.md). Neither full BCG/CCS22 paper was obtained; no stronger theorem scope is claimed from slides or abstracts.

The user then selected proof-first research and withheld CH-I/CH-O approval. The [expanded receipt review record](AKITA_INSTANCE_ACCUMULATION_RECEIPT_REVIEW.md) now records independent approval of the conditional product-oracle extraction, fixed-input transcript simulation, weak-state quotient reduction, deterministic outer projection and one ordinary degree-four-field quotient opening. It also records precise dense-advice and full-tree-query obstructions. These results do not admit the missing complete guest/profile, derived resource bound or concrete oracle realization; no implementation or parity conclusion follows.

**Subsequent correction:** the accepted-manuscript audit refuted the unrestricted-B emulator and exposed a curve-transport gap; historical approval of the imported receipt bound is superseded. The native archive theorem is unaffected. [ARO_WITNESS_SCOPE](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md), [hidden-curve transport](AKITA_INSTANCE_ACCUMULATION_ZERO_FINDING_TRANSPORT.md), [private continuation](AKITA_INSTANCE_ACCUMULATION_PRIVATE_ARO_CONTINUATION.md) and [native source-first extraction](AKITA_INSTANCE_ACCUMULATION_NATIVE_SOURCE_FIRST.md) now have independent conditional approval. The weaker authentication bound has Q outside the square root. [The current receipt record](AKITA_INSTANCE_ACCUMULATION_RECEIPT_REVIEW.md) retains the unresolved complete-guest, selected-profile, resource, concrete-interface and performance gates.

## Direct heuristic prototype review

The user later authorized **Allow a heuristic prototype**. The [D1 model](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) fixes a public concrete inner transcript, so the ordinary outer extractor can evaluate it as code instead of emulating an ideal ARO. D1 removes the graph/quotient/C_t wrapper and explicitly states that its native-game conjecture contains restricted CH-I language-soundness content. Its concrete loss/resource translation remains unknown; no broad CH-I/CH-O or concrete security theorem is inferred.

Independent model review corrected setup quantifiers to sample the installation inside the security experiment, then preserve that same sample during extraction. Independent [guest](AKITA_INSTANCE_ACCUMULATION_GUEST_SPEC.md) review corrected the cumulative active-row update. Independent reviews now also approve [OUTER_TRANSCRIPT](AKITA_INSTANCE_ACCUMULATION_OUTER_TRANSCRIPT.md) and [OUTER_CARRIER](AKITA_INSTANCE_ACCUMULATION_OUTER_CARRIER.md), including their exact sequencing, finite direct terminal and all original source/strong-check obligations. D1IntegratedReview found no remaining integrated algorithm-design blocker and permits implementation of the explicitly heuristic prototype. [The separate prototype record](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_REVIEW.md) records approval limits and arithmetic evidence; no compiled guest/profile, accepting deployment, numerical security or performance parity is admitted.
