# Immutable scope and notation

All source findings below concern Jolt `0791f79a55c3fadc3d327c78a72228696c338468`, using its actual `crates/...` paths and the `akita` transparent branch. This pinned revision was subsequently used as the base of the local `akita-instance-accumulation` branch; the findings remain scoped to the pin, not future edits. Akita’s adapter explicitly rejects ZK; BlindFold/Dory-only branches are not silently added to the selected protocol. Field-inline is optional; committed-program mode and field-inline are mutually exclusive in the verifier. The original audit used static inspection, not execution.

Notation:
- N=2^n cycles; RAM address width m=log2(ram_K); padded bytecode length L=2^b.
- k=committed one-hot chunk bits, 4 or 8 in the admitted native profiles; w=lookup virtual chunk bits, with k|w|128; v=128/w.
- b_I=128/k, b_B=ceil(b/k), b_R=ceil(m/k); b_D balanced increment digit columns, plus one carry column. P=b_I+b_B+b_R+b_D+1≤64.
- a_R is RAM phase-1 cycle rounds. Geometry validates a_R≤n and phase-2 address rounds≤m.
- f∈{0,1} denotes field-inline; p∈{0,1} committed program; fp=0.
- h=9 committed-bytecode lane bits. There are 55 lookup-table variants, 14 base circuit flags, 6 instruction flags; total base lanes are 384+2+14+6+55+1=462, padded to 512. Optional 10 field flags still fit 512, although committed program is rejected in that build.
- For committed program, R=max(n+k, h+b−log2(c), log2(program_image_words)); the shared precommitted cycle schedule has R−k rounds and address schedule k rounds. An address relation is omitted when it has no active address variables. These are scheduled totals, including dummy rounds, not merely a dense polynomial’s active-variable count.

Sources: [dimensions](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/geometry/dimensions.rs), [precommitted scheduling](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-claims/src/protocols/jolt/geometry/claim_reductions/precommitted.rs#L130-L190), [lookup enum](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-lookup-tables/src/tables/mod.rs#L144-L200).

# 1. Common batching and interpolation rules

`jolt-verifier-derive` is essential to the manifest. Every present member computes its input claim; all member input claims are absorbed in declaration order under the sumcheck-claim machinery; then **one independent `challenge_scalar()` is drawn per present member**. This is NOT one challenge raised to m batch powers. The input becomes Σ coefficient_i·claim_i·2^(R_batch−R_i). R_batch is the maximum member round count and D_batch the maximum member degree. Usual members are tail aligned; the committed-source cycle reductions explicitly use a head-aligned window. Dummy rounds must remain in the transcript/round accounting.

Each sumcheck round checks polynomial representation/degree and the sum recurrence, absorbs the polynomial, and draws one raw `challenge()`. For a degree-D message the ordinary interpolation arity is D+1, with charge D. Every independent affine batch coefficient may conservatively be assigned arity 2, charge 1. The same is true for each fresh multilinear reference or prefix-selector coordinate. A scalar fold with powers 0..e needs e+1 distinct branches and charge e; do not confuse its number of named semantic claims with its actual highest exponent.

Sources: [generated batch prelude](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier-derive/src/lib.rs#L439-L597), [sumcheck verifier](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-sumcheck/src/verifier.rs#L1-L145).

# 2. Exact stage/member round and degree inventory

Table entries are `rounds / per-round degree`; gamma arity is listed separately in §3. Degree means the actual selected symbolic relation, including native replacements and composed field extensions.

| Stage | Present members, in declaration order | Batch member count | Batch R,D |
|---|---|---:|---|
| 0 | Shape/setup/group selection and commitments; no sumcheck | — | No scalar challenge in stage 0 |
| 1 outer uni-skip | One centered-integer round | — | 1,27 base; 1,42 field-inline |
| 1 batch | Spartan outer remainder: n+1 / 3 | 1 | n+1,3 |
| 2 product uni-skip | One centered-integer round | — | 1,6 base; 1,12 field-inline |
| 2 batch | RAM RW: n+m / 3; product remainder: n / 3; instruction claim reduction: n / 2; optional field-register claim reduction: n / 2; RAM RAF: n+m−a_R / 2; RAM output check: n+m−a_R / 3 | 5+f | n+m,3 |
| 3 | Shift: n / 2; instruction-input virtualization: n / 3; integer register claim reduction: n / 2 | 3 | n,3 |
| 4 | Integer register RW: n+7 / 3; optional field-register RW: n+4 / 3; RAM value check: n / 3 | 2+f | n+7,3 |
| 5 | Instruction ReadRAF: n+128 / (v+2); RAM RA reduction: n / 2; integer register value evaluation: n / 3; optional field-register value evaluation: n / 3 | 3+f | n+128,v+2 |
| 6a | Native bytecode ReadRAF address: b / (b_B+1); Booleanity address: k / 3 | 2 | max(b,k),max(b_B+1,3) |
| 6b | Native bytecode ReadRAF cycle: n / (b_B+2); Booleanity cycle: n / 3; RAM Hamming Booleanity: n / 3; RAM RA virtualization: n / (b_R+1); instruction RA virtualization: n / (w/k+1); optional field increment reduction: n / 2; optional bytecode reduction cycle: (R−k) / 2; optional program-image reduction cycle: (R−k) / 2 | 5+f+2p | max(n,R−k) if p, otherwise n; D=max(b_B+2,3,b_R+1,w/k+1) |
| 7 | Native digit-zero/Hamming/decode reduction: k / 2; optional active bytecode address reduction: k / 2; optional active image address reduction: k / 2 | 1+δ_B+δ_image≤3 | k,2 |
| 8 pending-claim side | Native main common-point check; dense-source prefix packing; field increment deterministic zero-prefix embedding | No sumcheck | Fresh selectors as specified below |
| PCS handoff | Bind grouped statement; one bridge scalar; invoke native opening protocol | Not part of preceding sumcheck table | Must be separately classified |

The source-authoritative stage assembly is under `crates/jolt-verifier/src/stages/stage{1,2,3,4,5,6a,6b,7}/verify.rs` and corresponding `outputs.rs`. Relation implementations are under `crates/jolt-claims/src/protocols/jolt/relations`, the selected native replacements under `.../jolt/lattice/relations`, and optional relations under `.../field_inline` / `.../composed`.

Important departures from a base-Jolt inventory:
1. Bare `geometry/dimensions.rs` constants 27 and 6 are NOT the field-inline uni-skip degrees. Composed outer has 19+10=29 equality rows, centered domain 15, degree 3·14=42; composed product has 3+2=5 lanes, degree 3·4=12. Base domains are 10 and 3.
2. The native bytecode cycle degree is b_B+2, not b_B+1: the fused-increment factor is present.
3. Native integer/RAM increments are consumed in the bytecode read-RAF relation. There is no extra ordinary `IncClaimReduction` member/gamma on this path. Optional field increments still have their separate reduction.
4. Akita does NOT run trusted/untrusted advice cycle/address sumchecks in stages 6b/7. Advice contributions leave RAM value-check and are handled directly by dense-source stage-8 packing. Counting Dory advice phases here would describe the wrong protocol.
5. A committed-bytecode reduction consumes six staged values, not five and not nine: the five ordinary flag stages plus the native Store stage. The four fused increments do not become four independent committed-bytecode staged-value claims.

# 3. Every fresh frontend challenge family and local arity

## Stage 0 / commitment selection

`stage0.rs` selects the registered setup/group schedule from checked shape and auxiliary commitments. Native group roles are U?, T?, then FieldInc OR bytecode chunks plus program image, then OneHotTrace. This is deterministic selection, not a scalar FS draw. The transcript is `T::new(b"Jolt")`.

The preamble absorbs preprocessing digest; memory-layout max input/output and heap sizes; public input/output bytes and panic; RAM size; trace length; entry address; both RAM RW phase counts; both integer-register RW phase counts; committed chunk bits; lookup virtual chunk bits; and trace layout. Commitments are then absorbed in the frontend’s own order: main one-hot first, U?, T?, optional field increment, and committed-program identities/commitments. Group order and transcript order should not be conflated.

These adaptive statement/commitment choices do not receive fake degree-1 scalar rows, but they do contribute to ROM query/grinding opportunities in Q. New segment IDs, cuts, endpoint handles, and accumulation context need an explicit new transcript binding; they are not already fields of this preamble.

## Stage 1

- Fresh raw tau vector: n+2 coordinates. Its final high coordinate is evaluated through a centered-Lagrange kernel of domain D_o=10+5f: degree D_o−1, interpolation arity D_o. The other n+1 coordinates enter multilinear equality weights: arity 2 each. Do not label all n+2 as Boolean multilinear coordinates.
- Outer uni-skip round challenge: arity 28 base or 43 field-inline.
- One independent remainder batch coefficient, despite the batch having just one member.
- n+1 sumcheck challenges, arity 4.

The uni-skip output claim is absorbed as an opening claim before subsequent draws.

## Stage 2

- Product tau_low is reused from stage-1 remainder tail, with the prescribed reversal; no fresh n-vector.
- Fresh raw product tau_high: centered domain D_p=3+2f, degree D_p−1, arity D_p.
- Product uni-skip round: arity 7 or 13.
- RAM RW gamma: folds read/write, powers 0..1, arity 2.
- Instruction claim-reduction gamma: five claims, powers 0..4, arity 5.
- Optional field-register reduction gamma: three claims, arity 3.
- RAM output-check reference: m fresh raw coordinates, arity 2 each. This is a real vector draw, not a reused RAM address.
- 5+f independent batch coefficients; n+m sumcheck challenges of arity 4.

## Stage 3

- Shift gamma: arity 5. First four claims are NextUnexpandedPC, NextPC, NextIsVirtual, NextIsFirstInSequence at the outer point; the fifth is 1−NextIsNoop at the product point. The distinct equality/shift weights must survive the segment boundary lowering.
- Instruction input gamma: arity 2.
- Integer register reduction gamma: arity 3.
- Three batch coefficients; n round challenges of arity 4.

## Stage 4

- Integer register RW gamma: arity 3; optional field-register RW gamma: arity 3.
- RAM value-check gamma: arity 2; its transcript includes the empty labeled append `b"ram_val_check_gamma"` before `challenge_scalar()`.
- 2+f batch coefficients; n+7 round challenges of arity 4.
- Advice/program-image initial-RAM contributions are actual staged source claims/selectors, not trusted replacement evaluations.

## Stage 5

- Instruction ReadRAF gamma has **arity 4 on Akita**, not 3. In the concrete derived public, `EqRafFlag` contains γ²·Identity−γ·Left−γ²·Right **plus γ³·upper_half_all_ones(r_address)** when canonical instruction addresses are required. The symbolic top-level relation alone does not expose this last power. `upper_half_all_ones` is the product of the first 64 address coordinates.
- This term rejects the noncanonical high-address region needed by the source Fp128 encoding. It must remain when challenges move to E: source values and characteristic are still K. It is not made obsolete by E having q² elements.
- RAM RA-reduction gamma: arity 3.
- Integer/field value-evaluation members have `NoChallenges`.
- 3+f batch coefficients; n+128 rounds of arity v+3.

Source: [concrete canonical-address public](https://github.com/a16z/jolt/blob/0791f79a55c3fadc3d327c78a72228696c338468/crates/jolt-verifier/src/stages/stage5/instruction_read_raf.rs#L184-L210).

## Stage 6a

- Bytecode ReadRAF draws six scalar gammas: one outer gamma and five stage gammas.
- Native outer gamma powers: five base stages at 0..4; four RAM/register increment claims at 5..8; PC, shifted PC, entry at 9..11. **Arity 12**.
- Stage gamma arities are **[16+10f, 4, 9, 3+3f, 57+f]**. The final 57 is 2 plus the actual 55 lookup tables. Field extensions reuse stage-4/5 gammas; no extra field-bytecode gamma is drawn.
- Booleanity gamma is a raw `challenge()`. The selected fold uses γ^(2i) for P columns, so highest degree is 2(P−1), interpolation arity **2P−1**, not P.
- Booleanity reference cycle is reused from stage 5. Reference address is the prescribed reversed/truncated stage-5 address; generic padding would draw max(0,k−128) coordinates, which is **zero** for legal k=4,8.
- Two batch coefficients; max(b,k) round challenges of arity max(b_B+1,3)+1.

## Stage 6b

After stage 6a:
- Instruction RA-virtualization gamma: folds v virtual RA claims, arity v.
- Optional field increment gamma: folds the two field-inc claims, arity 2.
- Optional committed-bytecode eta: folds six staged claims, arity 6. It is independent of the bytecode stage gammas.
- Booleanity continuation, RAM Hamming Booleanity, RAM RA virtualization, program-image cycle reduction have no new member gamma.
- 5+f+2p batch coefficients; round count/arity as in the table.

The committed-program reductions use the shared precommitted scheduling/permutation, including head alignment; a compiler cannot replace them by unrelated tail-aligned n-round reductions merely because the output is eventually a dense opening.

## Stage 7

One native Hamming/digit-zero gamma has maximum exponent

`e_H = 2(b_I+b_B) + 3 b_R + (b_D+1)`.

There are two gamma positions per instruction/bytecode RA column, three per RAM RA column, one per increment digit/carry column, and then the fused-increment decode claim at exponent e_H. Thus interpolation arity is **e_H+1**, with e_H≤3P≤192.

This relation proves the selected digit-zero reconstruction/Booleanity/decode conditions. It does not narrow a carry to its honest ±1 distribution or create a u64 cut-state theorem. Preserve the full admitted source alphabet and existing Boolean-table relation.

Committed-bytecode/image address phases have no new member gamma. There are 1+δ_B+δ_image independent batch coefficients and k round challenges of arity 3.

## Stage 8 and the PCS bridge

For each present U, T, bytecode chunk, and image source, prefix packing absorbs logical arity, slot capacity/layout digest, logical point and evaluations, then draws `selector_num_vars` raw coordinates. These selectors are each multilinear, arity 2. In the current singleton padding use this is max(0,14−a) for logical arity a, but the manifest should read the actual layout rather than hardcode a zero draw. There are at most c+3≤259 such dense groups. Main native common-point checking is transcript-neutral. Field increment uses an explicit zero prefix, not a sampled prefix.

The current grouped opening handoff absorbs setup/schedule, roles/counts/order, commitments, points and values, then calls `bridged_akita_session`: exactly one outer `challenge_scalar()` is serialized and appended to the session label. The native grouped label is `b"jolt-akita/precommitted-group-batch/v4"`. This bridge is a **nested-FS session seed**, not automatically a degree-1 algebraic test. There is no later Jolt challenge depending on the terminal PCS proof.

No dedicated frontend proof-of-work nonce was found in the selected stage flow. Prover-chosen statements/messages/commitments and all permitted retries remain ROM queries. The bridge and native nonce/challenge roles must be entered in the complete FS composition; omitting them or relabeling a K-valued bridge as an E test is not justified.

# 4. Samplers, exclusions, and what the numerical bound really proves

Current `challenge()` and `challenge_scalar()` squeeze **16 bytes**. The former uses `from_challenge_bytes`, the latter `from_scalar_challenge_bytes`. Their generic distinction is protocol-significant even where the selected Fp128 decoders coincide. Fp128 reduces the bytes modulo q; its `random()` uniform sampler is a different API. The selected current challenge distribution is therefore not an E distribution and not exact uniform K sampling. A generic `F := E` substitution with the same 16-byte squeeze leaves only 128 bits of support.

No frontend nonzero-gamma or avoid-interpolation-node rejection rule is present in the inspected selected relations. Zero gamma/coefficient is permitted. Centered Lagrange interpolation uses fixed domain denominators; there is no source-backed reason to invent exclusions of the random evaluation points. The new E protocol must specify and account for any exclusions/retries it actually introduces. Merely decoding two independently reduced 16-byte coordinates also requires its own distribution bound; it is not automatically the profile’s exact-uniform or half-field-exclusion sampler.

## A conservative explicit existing-frontend bound

[INFERENCE, arithmetic from the source-backed table] Conditional on replacing every algebraic scalar site above with the profile’s admitted E sampler and preserving the selected relations, an extremely loose upper bound for the **existing frontend through prefix packing** is available, with no leftover base-field algebraic challenge:

- n≤34; k≤8; b_B,b_R≤P≤64, hence b,m≤512.
- Every one of the eight batches has at most 546 rounds. The precommitted schedule also fits this bound for the legal source catalogue; even 64-bit source-size arities are far below it.
- Every batch degree is at most 66. This covers b_B+2; v≤32 and w/k≤32; the other batch degrees are ≤3.
- Two standalone uni-skips cost at most 42+12=54.
- Outer tau has at most 36 coordinates, each charged at most 15 (deliberately above the actual maximum 14).
- At most 22 other member/reference scalars, including product tau_high but excluding the separately counted RAM address vector, can each be charged at most 192. This overcounts the mutually exclusive field/program cases.
- RAM output reference has at most 512 coordinates.
- There are at most 32 independent batch coefficients (the table gives a maximum of 29 even before mutual-exclusion tightening).
- At most 259 dense groups; allowing 64 selector coordinates each is deliberately looser than the actual prefix-padding rule and legal catalogue.

Even charging a factor two everywhere, as allowed by the profile’s half-field-exclusion rule,

`c_E,frontend per segment ≤ 2[8·546·66 + 36·15 + 54 + 22·192 + 512 + 32 + 259·64] = 620452 < 2^20`.

For Tmax=16, `c_E,existing frontend < 2^24`, well below 2^40. The actual manifest is much smaller. This is a conservative arithmetic upper bound, not a measured statistic, and is conditional on the specified E sampling law.

**Answer to the full c_E question:** this proves substantial room for the existing frontend, but does **not** prove the complete execution c_E≤2^40. The complete manifest must also instantiate and add all new endpoint and multipoint reductions, raw/source-domain prefixes, supplied-output native/fold predicates, outgoing strong checks, receiver relations, final PCS and bridge/session composition. Those are not defined by the inspected frontend and cannot be assigned an invented residual budget. The existing code still has K challenges, so its literal current soundness expression cannot be advertised as solely c_E/|E|.

# 5. Mixed-tree compiler implications and missing rows

The table supplies local interpolation arities, message ordering, optional-member counts and dummy-round geometry. It does not itself supply the complete reduction/extraction theorem or an implemented compiler.

Required compiler details:
1. Use D+1 branches for each degree-D sumcheck coordinate, not a universal binary tree. Use the composed uni-skip domains/degrees and independent batch-coefficient coordinates.
2. Include the initial centered-Lagrange tau coordinates, the instruction γ³ canonical-address term hidden in a concrete derived public, Booleanity’s even exponents, native fused increments/decode, and dense prefix selectors. A walk of `SymbolicSumcheck::Expr` alone misses concrete derived-public challenge dependence.
3. Preserve head/tail windows, dummy rounds, address/cycle reversals, and distinct shift reference points. Reused points are not new random draws, but their reuse is part of the extraction wiring.
4. Every branch must retain the fixed original K source/commitment handle and registered role. Derived E-valued polynomials are not new committed E-valued sources.
5. Source-domain, ordinary multipoint, endpoint, receiver/native and final opening gates need their own source-backed local lemmas and bounded supplied-output predicates. Their branch count must be composed, not merely their failure charges added.
6. Under fixed P≤64,c≤256,Tmax=16, the frontend’s local arities are uniformly bounded and its logarithmic-round portion can be expressed as a fixed-degree polynomial tree-size bound in explicit source sizes. But the proof must actually exhibit the product of local arities and all other gate costs. Saying “Tmax is finite” or displaying c_E alone is not that proof, and does not settle the missing native/final portions.

Concrete new frontend constraints still absent from source include segment-specific initial/final register and RAM endpoints, their ordinary opening/query normalizations, modified shift boundary terms, neighboring segment endpoint equality, and new transcript bindings. Keep public register values canonical in K and RAM endpoints full-field K as the parent contract specifies; do not substitute honest emulator integers or introduce a new halt theorem.

# 6. Legal whole-instruction/inline cuts: existing metadata is sufficient, with an authentication obligation

## What canonical expansion actually stamps

`stamp_instruction_sequence` and `stamp_inline_sequence` both use the same stamping routine. For a sequence of length ℓ, row i gets:
- `virtual_sequence_remaining = Some(ℓ−i−1)`;
- `is_first_in_sequence = (i==0)`;
- compressed-source status only on the last row.

For inline expansion, the materializer recursively builds the rows, appends resets for allocated temporary registers, **then stamps the entire resulting sequence once**. Reset rows are therefore inside the same top-level inline group; a nested helper’s internal first-row marker is not an independent legal top-level cut marker.

The instruction macro converts this metadata into circuit flags: `VirtualInstruction` iff the countdown is Some, including Some(0); `IsLastInSequence` iff Some(0); `IsFirstInSequence` from the stamped Boolean; `DoNotUpdateUnexpandedPC` when countdown is nonzero. Ordinary singleton instructions are nonvirtual and need not have First=1.

Consequently, for a successor row from this canonically stamped fixed program,

`nextIsVirtual · (1 − nextIsFirstInSequence) = 0`

accepts exactly the relevant source-event starts: ordinary nonvirtual successor, or first row of an expanded instruction/inline sequence. It rejects an interior virtual row, including any appended inline reset row. Thus **no new metadata lane is needed for the requested whole-source-instruction/inline convention**.

This does not encode arbitrary larger application-level groupings of several source instructions. Such a different grouping would need its own public policy/metadata. It is also not a statement that execution has halted.

## Existing committed-program lanes

Committed-program emission includes every circuit flag in `CIRCUIT_FLAGS` order. Register one-hot blocks occupy lanes 0..383; unexpanded PC is lane 384; immediate lane 385; circuit flags begin at lane 386. Existing relevant base lanes are:
- VirtualInstruction: **393**;
- IsFirstInSequence: **398**;
- IsLastInSequence: **399**;
- DoNotUpdateUnexpandedPC: **395**.

The first two are enough for the cut predicate. Lane 384 is already available when authenticating the expanded-PC→architectural-PC mapping required by the endpoint tuple. IsNoop is also an existing instruction-flag lane if that endpoint reduction needs it. No new program commitment, Merkle root, or hash commitment is required.

A source-backed proposed lowering is to request the needed deterministic Boolean-coordinate evaluations from the existing bytecode chunk handle. For expanded row p, select chunk `j=floor(p/(L/c))` and row offset `p mod (L/c)`; the logical CycleMajor coordinate is `bool(lane) || bool(row_offset)`, with the registered physical padding handled by its existing source map. Feed these ordinary queries through the real same-source multipoint/normalization layer. Do not pretend the current one-opening-per-group adapter already accepts these additional queries.

If the full program is already public to the verifier, it can check its fixed metadata directly; in committed-program mode, claimed boundary flags are not trusted and need the source-bound query path. Booleanity of public claimed flags and equality to the authenticated K table values must be enforced, rather than accepting arbitrary K values satisfying the product equation.

## Why this is not already a segment-cut check

Existing R1CS has:
- `(VirtualInstruction−IsLastInSequence)·(NextPC−PC−1)=0`;
- `(NextIsVirtual−NextIsFirstInSequence)·(1−DoNotUpdateUnexpandedPC)=0`.

The second is guarded and deliberately allows execution inside a virtual sequence. It is NOT the proposed unguarded interior-cut predicate. The new predicate belongs at actual interior segment boundaries.

It is sufficient only with the rest of the parent boundary contract: authenticate the actual successor row selected by expanded PC; bind its flags/unexpanded PC/Noop as needed to the corrected shift relation; check in-range/source identity and next-segment equality; enforce the predicate only where there really is an interior successor. The product equation by itself, applied to unbound flags or a conveniently chosen program row, is not sufficient.

Finally, `BytecodePCMapper` validates contiguous equal-address runs with descending countdown ending at zero and records first-PC/length mappings. It does **not** prove that arbitrary manually supplied `is_first_in_sequence` bits agree with canonical stamping. The claim about whole source/inline groups therefore relies on the fixed canonically expanded/stamped program preprocessing or an explicit catalogue well-formedness condition. Do not silently turn this into a new ISA-expansion correctness theorem for arbitrary adversarial bytecode metadata.

# 7. Integration conclusions

- The existing two program flag lanes are enough; the missing work is authenticated queries and boundary constraints, not a new commitment mechanism.
- The exact frontend ledger is tractable and comfortably below the proposed c_E budget after a proper E lowering. Hidden high-degree gammas, initial reference coordinates, batch coefficients and prefix selectors must all be retained.
- The immutable implementation does not yet separate K source values from E challenge values, does not sample E, and does not implement the pending-instance endpoint/multipoint/accumulation interfaces. The full c_E≤2^40 and polynomial mixed-tree claims remain conditional until those concrete rows and the native/final manifest are instantiated.
- No builds, tests, formatters, branches, or source/document edits were performed; all checks in this report are source inspection or explicitly labeled arithmetic inference.