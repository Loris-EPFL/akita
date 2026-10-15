# Native Akita accumulation: explicit challenge-field and security profile

## 1. Decision and scope

The base-field receiver examples establish arithmetic closure, not 128-bit Fiat–Shamir admission. Akita Corollary 10.34 bounds the raw-ROM strength by the **entire** challenge ledger. Increasing only the lattice folding challenge set does not fix a term `c/q` from an ordinary base-field sumcheck.

This note specifies a concrete alternative: keep Module-SIS and all committed source coefficients over the existing q128 base field, but use a quadratic extension for scalar proof challenges. It supplies a new closing receiver calculation and an explicit sufficient FS budget. It does **not** assert that current Jolt or Akita implements this source/challenge-field separation, that the complete frontend ledger has been instantiated, or that end-to-end 128-bit security/performance has been established.

No Merkle/hash commitment or completed-proof accumulator is introduced. The random oracle remains the existing Fiat–Shamir/XOF role. Quantum lattice-attack parameter sizing is separate from the classical-ROM theorem; no QROM claim is made.

## 2. Base and challenge fields

Set
\[
 q=2^{128}-2^{32}+22537,\quad K=\mathbb F_q,\quad
 E=K[U]/(U^2-5).
\]

This is the **scalar-extended full-ring** interface in SourceQueryLift §4.3, not unmodified Akita Eq. (84) with an invalid fractional carrier ratio. For each source block, its raw and ordinary maps are respectively
\[
 L_{\rm raw}(s)=\sum_{x,\ell}\chi_x\chi_\ell s_{x,\ell}(Y),
 \qquad
 L_{\rm dec}(s)=\sum_{x,\ell}\chi_x b_{\rm com}^{\ell}s_{x,\ell}(Y).
\]
The 128 coefficient equality weights are applied in the final scalar claim row; each map uses its own bound query weights. Write each partial as \(e=e_0+Ue_1\); both S-valued planes use the same folding unit. No extra source plane is introduced.

The exact modular exponentiation `5^((q-1)/2) mod q = q-1` was checked. Thus, under the native prime-modulus premise, 5 is a nonsquare and E is a field of size q squared. Encode an E element as its two canonical K coordinates.

All original A/B/D/F/H matrices and lattice sources remain over K and their scheduled negacyclic rings. The challenge carrier is `S=K[Y]/(Y^128+1)`, embedded identically into the 128-dimensional source ring. An evaluation carrier is `E[Y]/(Y^128+1)` and has **256 base-field coordinates**. Every such partial must carry both coefficient planes. Do not reinterpret it as a 128-coordinate ordinary partial.

The same-handle source-query construction is S-linear over this extension: its equality weights lie in E; splitting the resulting carrier equation into two K planes preserves the unit-fork proof. The ordinary polynomial is still a K-valued Boolean table evaluated at an E point. It is not a different commitment to an E-valued table with twice as many source coefficients.

**Frontend contract.** Keep Boolean-table witness values and the original execution relation in K; embed constants into E and run scalar sumchecks, equality weights and public claim combinations in E. Injectivity of K into E preserves every original Boolean-table equality. Intermediate challenge-dependent virtual polynomials can be E-valued, but every committed original source retains its original K map and role. A generic field alias change does not implement this distinction. The final PCS must likewise support an E-point opening of the original K table; retaining a base-field-only final API is not sufficient.

## 3. Folding family and deterministic response bounds

Choose the unfiltered 128-coordinate challenge shell with 51 magnitude-one coefficients, 51 magnitude-two coefficients, and independent signs. Thus
\[
 s=102,\quad\omega=153,\quad c_{\max}=2,
\]
\[
 |\mathcal C|=\binom{128}{102}\binom{102}{51}2^{102}
 =2015164608959462280558562440154814157940950272528301181540825627181504853323406869790720,
\]
with `log2 |C| = 290.018641945...`.

Since `q mod 16 = 9`, the native splitting parameter is four. The same sufficient unit condition `4 < q^(1/4)/2` holds. Increasing support does not change this coefficient-difference condition. It **does** increase the deterministic response norm through omega; the ranks and witness sizes below account for that.

Use common/response/opening base 32. The source profile and application dimensions are those of receiver-note §10, except the receiver capacity is now `2^23`, its B is 64, and receiver/RAM A ranks are five. Every source has two declared channels: ordinary and raw. This retains the fresh old-source prefix for the generic supplied-output local bounded relation. The optional history-only range reuse is priced separately.

| Group | P | B | M | Source digits | Source bound | A rank | Response bound U | Response digits |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Receiver | 1 | 64 | 1024 | 1 | 16 | 5 | 156672 | 4 |
| Main native columns | 56 | 1 | 512 | 1 | 1 | 3 | 8568 | 3 |
| Each full-field RAM endpoint | 1 | 32 | 16 | 26 | 16 | 5 | 78336 | 4 |

The positive base-32 reaches are 15855 for three digits and 507375 for four. These cover `U=PB*153*source_bound` for every admitted source. Source Booleanity, all physical source bounds, valid supplied noncanonical paths and required zeros remain actual predicates. Honest producer distributions are not used.

## 4. Complete receiver and matrix inventory

Use shared-response channels, direct coefficient maps, no response chunking, no setup offload and the explicitly quotient-free relation. Each E-valued partial has width 256. Opening/inner-image depth is 26. With the notation of SourceQueryLift,
\[
 Z=M\delta_{\rm com}\delta_f128,\quad
 E_g=H P B\cdot26\cdot256,\quad
 T=P B n_A\cdot26\cdot128.
\]

| Allocation | Base-field cells |
|---|---:|
| Receiver response | 524288 |
| Receiver partials | 851968 |
| Receiver inner-image digits | 1064960 |
| Main response | 196608 |
| Main partials | 745472 |
| Main inner-image digits | 559104 |
| Each RAM response | 212992 |
| Each RAM partials | 425984 |
| Each RAM inner-image digits | 532480 |
| Four F chains plus one H, stage 1 | 81920 |
| Four F chains plus one H, stage 2 | 10240 |
| **Successor, counting both RAM endpoints** | **6377472** |
| Required zero suffix | 2011136 |
| **Fixed capacity** | **8388608** |

Every span aligns to 128. The 184 fold coordinates are `64+56+32+32`; extension-valued partials do not double them. Splitting each fold-evaluation channel into its two base-field ring equations gives 49 ring rows, before scalar query rows or their charged combination. There is one shared D/H path, not one per extension coordinate.

The receiver/RAM A collision radius is
\[
4\cdot153(32^4-1)=641727900;
\]
the main radius is `4*153*(32^3-1)=20053404`. The inspected q128 table rounds them to 2080374660 and 32505732 respectively. Rank five at the first bound permits width 136623; rank three at the second permits width 976. Thus A widths 1024, 416 and 512 fit those table entries.

B widths are receiver 8320, main 4368 and each RAM 4160. Shared D width is 19136. Their digit-difference bound is 31, with rank-one table ceiling 28621854. The binary F/H paths retain dimension-16 width 1024 and dimension-8 width 256, with the previously checked rank-one binary ceilings. This is a table-model inventory, not an end-to-end advantage estimate.

Under the separately proved finite-history range reuse, old H becomes one. Save 425984 partial cells and two base-field ring rows: live successor **5951488**, tail **2437120**, D width **15808**, ring rows **47**, fold coordinates still **184**. Do not claim the resulting variant proves bounded knowledge for a naked arbitrary fixed old instance.

The counts exclude no required quotient: quotients are absent only because this is a new quotient-free coefficient relation. An implementation using quotient-lift must recompute all counts. Public setup-weight evaluation is real verifier/prover work when offloading is absent.

## 5. Explicit sufficient FS and setup budgets

Register a finite supported segment bound, for example `Tmax=16`, before setup/proving. For the **entire** chosen execution, source-domain reductions, multipoint prefixes, native folds, outgoing strong checks, final PCS and any included receipt, derive its actual challenge manifest.

Let `c_E` be the sum of all scalar-field coordinate charges, including their interpolation arities and fixed-filter mass bounds. For an unfiltered E coordinate the charge numerator is `k-1`. If a necessary public exclusion set leaves at least half the field, conservatively charge `2*(k-1)`; a smaller or response-dependent accepted set needs its actual proved sampler theorem instead. Let `c_C` sum the unfiltered lattice coordinate charges. Rejection/nonces and all relevant FS queries are counted, not discounted.

A **sufficient**, mechanically checkable profile contract is
\[
 Q_{\max}+1\le2^{64},\qquad c_E\le2^{40},\qquad c_C\le2^{40},
\]
with every scalar challenge in E and no omitted base-field challenge contributing a `c/q` term. Then
\[
 (Q_{\max}+1)\varepsilon_{\rm int}
 \le2^{104}\left(q^{-2}+|\mathcal C|^{-1}\right)
 <2^{-149}<2^{-128}.
\]
The conservative strict inequality uses `q>2^127` and `|C|>2^290`. The cap is not a claim that the full new Jolt manifest has already been generated. **Reject** a profile with an uncharged site, incompatible sampler, narrower field, or excessive actual sum. In particular, do not attach this conclusion to an unchanged base-field-only Jolt frontend or final proof.

For native flat setup expansion, an illustrative sufficient bound is at most `2^40` expanded base coefficients and sampling width `w=512`. Akita Eq. (67) then gives
\[
 \Delta_{\rm setup}\le N_{\rm setup}q2^{-512}<2^{-344}.
\]
This is a new setup/provisioning contract, not permission to change the distribution of already frozen handles. If the existing generator is used, compute its actual bias instead. Setup and FS remain separate namespaces.

These inequalities establish a **conditional statistical FS/setup budget**. The overall bound still includes every runtime-sensitive MSIS advantage. No equality between this FS target and full end-to-end 128-bit security is asserted.

## 6. Exact finite knowledge theorem interface

The actual statement is the conjunction of the unchanged Jolt Boolean-table relation on legally concatenated active segments, the original program/public-I/O statement, exact boundary identities and all specified original-handle source predicates. The final verifier checks the complete ordered archive or a separately admitted receipt, then one complete final opening/decision. It does not generate full segment PCS proofs.

For a fixed finite registered topology, the conditional theorem has the following checkable premises:

1. The source and ordinary extension-point maps are the complete original maps; each carrier relation is S-linear, with all planes retained.
2. Source-domain and multipoint prefixes have their specified full-grid interpolation/claim-separation interfaces. Any omitted old prefix is replaced only by the joint history range-reuse proof.
3. Every native local strong/ring/relation/setup check, original path alphabet, unit-difference condition, mixed source-comparison radius and complete Eq. (109) layout is included.
4. All-valid bounded inputs fit the deterministic encoding/receiver envelope. New APIs preserve the supplied noncanonical raw source and path, rather than silently recomputing another commitment.
5. The actual native/frontend/final stage manifest satisfies the selected FS bound and an **explicit fixed polynomial** mixed-tree ceiling in security and explicit witness size, exactly as required by Akita Eq. (176). Finite depth alone is not that certificate.
6. The complete final decision supplies its actual branch endpoint and certified weak unit/product paths; exact frontend lowering and Boolean-table concatenation supply the application relation.

**Supplied-tree derivation.** Begin at each accepted final branch. Authenticate its final original source/evaluation. Recover the outgoing strong checks by interpolation and same-source uniqueness; reconcile original B/F and shared D/H paths. Unit-coordinate subtraction extracts the incoming weak sources and both extension planes of every query. Same-original-source comparison across the raw-prefix branches followed by range interpolation restores the input bounds. If old range was reused, continue through its preceding outgoing strong record instead. Repeat backward, ending at the independently checked initial state. The original frontend claims then give the exact Boolean-table relations; the segment concatenation lemma yields the whole-run relation. Any inconsistent source/path gives one of the inventoried matrix collisions, at its actual mixed radius.

**ROM step.** The complete stage list, not independent locally sampled transcripts, defines one public-coin protocol. Apply the mixed-tree conversion used in Akita Lemma 3.22 with its actual coordinate streams, filters and paid continuation reruns. The algebraic implication above is its supplied-tree extraction premise. An earlier fork reruns the rest of that branch through final acceptance; it never borrows the published branch's terminal witness. Shared flat-setup collisions reduce to ordinary MSIS using the actual registered views, as in Akita Lemma 10.11. Thus the resulting bound has the form
\[
 (Q+1)\varepsilon_{\rm int}+\Delta_{\rm setup}
 +\sum_{I\in\mathcal I}\operatorname{Adv}_{\rm MSIS_I}(R_I),
\]
where each R_I includes the **whole** mixed-tree ROM extraction and embedding/collision work. Its expected runtime is polynomial only after premise 5; no uniform-growing-depth or tight numerical lattice reduction is inferred.

This gives a conditional proof architecture with explicit field, closure and sufficient FS checks. It is not a substitute for producing and reviewing the actual complete manifest and source/frontend/suffix implementations. The paper's root-only schedule policy has not been silently declared to cover the new topology.

## 7. Implementation and comparison consequence

Required new capability is source/challenge-field separation through Jolt frontend reductions, extension-point original-handle openings, all Akita pending/final transitions, transcript encodings and the manifest compiler. Existing base-field source arithmetic and A/B/F compression can remain base-field operations. Partial storage and challenge-field arithmetic increase, as the exact ledger shows.

A fair performance comparison must use the same mathematical execution statement **and security policy**. If this extension-field path is selected to meet the raw-ROM FS budget, compare to a full-PCS-plus-recursion control meeting that same budget, not to an insecurely relabelled base-field control. Conversely, an independently justified native grinding/work-factor theorem could support a different fair profile; it is not supplied merely by a `TRANSCRIPT_SECURITY_BITS=128` constant.

No source changes or timings were produced. This note does not establish parity or impossibility of parity.

## References and observed verification

- [Akita](SOURCE_PROVENANCE.md#akita), §§3.1–3.3, 4, 9–10: carrier fields, unit differences, original source maps, mixed trees, flat setup, and Eqs. (223)–(231). Live qmd `lattice-papers/akita.md` lines 4105–4250 were retrieved for this note; the source/query and receiver notes retain earlier reads.
- [Same-handle source-query construction](AKITA_INSTANCE_ACCUMULATION_SOURCE_QUERIES.md), especially carrier linearity, plane accounting and paid raw-point normalization.
- [Receiver profile](AKITA_INSTANCE_ACCUMULATION_RECEIVER_PROFILE.md), §10, for the pinned Jolt dimensions and base-32 starting geometry; its matrix-table SHA-256 matches immutable Akita 8357433.
- [Range reuse](AKITA_INSTANCE_ACCUMULATION_RANGE_REUSE.md) and [segment interface](AKITA_INSTANCE_ACCUMULATION_SEGMENT_INTERFACE.md), §7.1, for their distinct conditional history and application arguments.

Observed checks: exact modular nonsquare computation; challenge cardinality; deterministic response depths; complete integer cell/width/row counts; comparison to the displayed table entries; and the sufficient numerical FS inequality. These are arithmetic checks, not execution of a proof system, a full-manifest audit or a cryptographic security test.
