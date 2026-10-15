# Native accumulation: exact compact-profile matrix registry

Status: independent review approved the specified role/view inventory, mixed radii, table ceilings, coefficient totals and setup-bias bound. The compact schedule generator and literal matrix provisioner now execute; [the implementation record](AKITA_INSTANCE_ACCUMULATION_EXTENSION_MIGRATION.md#14-corrected-original-plane-envelope-and-concrete-producer-execution) states their verification scope. Review of the finite archive specification is not admission of a compiled guest, authenticated installation, concrete security estimate or complete accumulation run.

**Revision gate:** this file records the earlier independent-role installation.
The [native-reuse revision](AKITA_INSTANCE_ACCUMULATION_NATIVE_REUSE_PROFILE.md)
and [independent shared-prefix certificate](AKITA_INSTANCE_ACCUMULATION_SHARED_SETUP_REVIEW.md)
conditionally admit a fresh shared-prefix alternative with the same semantic
roles and source contracts. It has 12 numerical views for fixed E2, not 33
independent matrices. Existing objects retain their actual keys; the counts and
512-bit bias below must not be attached to the native seed/page law.

## 1. Setup rule

Use distinct canonical setup namespaces for unequal source roles and final levels. Each row below specifies one full independent matrix over its stated negacyclic K ring; there are no unlisted shared-prefix views. Repeated occurrences of a role use the **same** matrix and source map. In particular, RAM incoming/outgoing/carried handles share the RAM role, not segment-specific replacement keys. Fix keys before source commitments and absorb the registry/profile identity before proof challenges.

A role namespace includes scheme/version, K modulus, ring degree, module rank, input width, source/plane/digit ordering and the semantic role below. The registered map, not merely a caller-supplied digest, determines the matrix. Setup expansion and Fiat–Shamir challenge streams have separate domains. A future shared-prefix optimization needs a new joint view certificate; it cannot silently replace this registry.

The main source is a **newly provisioned direct scale-one Boolean source** retaining P=56 native columns. Its raw values are 0/1. It is not a reinterpretation of a previously frozen signed or tensor source. Receiver source values are the full common base-32 digits; RAM sources have 26 base-32 digit planes. Original F/H paths and noncanonical legal digits are retained.

## 2. A matrices and source-comparison classes

| Setup role | Ring degree | Rank | Width | Source geometry | Largest collision radius | Audited upward radius | Width ceiling |
|---|---:|---:|---:|---|---:|---:|---:|
| ReceiverRoot.A | 128 | 4 | 1024 | P1 B32 M1024 delta1 | 234880800 | 260046724 | 18964 |
| Main.A | 128 | 3 | 512 | P56 B1 M512 delta1 | 7339808 | 8126340 | 23489 |
| RAM.A | 128 | 3 | 832 | P1 B16 M32 delta26 | 7339808 | 8126340 | 23489 |
| Final2M.A | 128 | 3 | 1024 | P1 B16 M1024 delta1 | 7339808 | 8126340 | 23489 |
| Final1M.A | 128 | 3 | 512 | P1 B16 M512 delta1 | 7339808 | 8126340 | 23489 |
| Terminal512K.A | 128 | 3 | 512 | P1 B8 M512 delta1, inner only | 3211264 | 4063108 | 109366 |

These are actual matrix widths M*delta, not total polynomial/source lengths. P and B count separate source blocks sharing that original role's A map. All claims on one original handle retain its P/B/source map.

For a nonterminal extraction retain its **unit/product certificate**
\[
\delta s=r,\qquad \|\delta\|_1\le112,\qquad
\|r\|_\infty\le\Delta_f=32^{\delta_f}-1.
\]
The three- and four-digit values are 32767 and 1048575. Comparing two weak sources under the same A projection produces
\[
v=\delta'r-\delta r',\quad Av=0,\quad
\|v\|_\infty\le224\Delta_f.
\tag{1}
\]
The units make v nonzero whenever the sources differ. This gives the first five table radii.

A source already strongly certified with bound Bs has certificate `(1,s)`. Its mixed comparison bound is `Delta_f+112*Bs`, and two strong sources have bound `2*Bs`. Bs=1 for Main and 16 for the other roles. Every such bound is below its listed weak–weak radius. No inverse-challenge norm bound is assumed.

For the direct terminal, accepted responses have norm at most 7168. Its product certificate has bound 14336; (1) becomes `224*14336=3211264`. The mixed weak/strong radius is `14336+112*16=16128`, also dominated. The terminal uses its fixed A-image tuple from creation. No comparison between an OuterPayload and a TerminalInnerState is inferred from equal dimensions.

## 3. Original B/F and opening-local D/H matrices

All B and D roles have ring degree 128 and rank one:

| Setup role | Width | Original fixed object / comparison scope |
|---|---:|---|
| ReceiverRoot.B | 3328 | Original receiver inner-image digits, P1 B32 rank4 |
| Main.B | 4368 | Original main inner-image digits, P56 B1 rank3 |
| RAM.B | 1248 | Original RAM inner-image digits, P1 B16 rank3 |
| Final2M.B | 1248 | Original full commitment at N=2^21 |
| Final1M.B | 1248 | Original full commitment at N=2^20 |
| Update.D | 6240 | Four-handle, one-channel update partials |
| FinishRoot.D | 1664 | First final reduction, B32 full-ring E partials |
| Finish2M.D | 832 | Second final reduction, B16 partials |
| Finish1M.D | 832 | Third final reduction, B16 partials |

Their certified digit alphabet is [-16,15], so every compared difference has coefficient bound 31. The q128 degree-128 rank-one ceiling at radius 31 is 28621854; all listed widths fit.

Each of the five B roles has its own original F chain. Each of the four D roles has its own opening-local H chain. For **each** of these nine chains register two distinct matrices:

| Compression stage | Ring degree | Rank | Width | Difference bound | Width ceiling |
|---|---:|---:|---:|---:|---:|
| First | 16 | 1 | 1024 | 1 | 7077 |
| Second | 8 | 1 | 256 | 1 | 508 |

All source digits are the actual negative-binary alphabet {0,-1}; no canonicality premise is imposed. A difference of two such vectors has bound one. Equality of the final compressed payload plus collision-freedom fixes each preceding digit vector backward through the **complete** chain. Their gadget recompositions then identify the original B or D image. B comparison fixes the original inner-image digit tuple; A comparison fixes the raw source using its weak certificates.

D/H consistency is needed within a coordinate-fold family where that opening payload was bound before the fold. It is **not** demanded across different source-prefix/query branches: those may change point, values, partials and D/H payload. Only the original source projection remains fixed there, as in Akita Corollary 10.17. This distinction prevents an invalid same-opening assumption in range recovery.

The terminal has no B/F or D/H role. It publishes A images and complete full-ring partials. All B/F and D/H paths of its predecessor reduction remain in that predecessor's witness and registry.

## 4. Complete setup size and collision inventory

There are six A matrices, five B matrices, four D matrices and eighteen compression matrices: **33 full independent matrix roles**. Repeated segments and repeated RAM handles do not generate new matrix roles.

| Matrix class | Expanded K coefficients |
|---|---:|
| Six A matrices | 1826816 |
| Five B matrices | 1464320 |
| Four D matrices | 1224704 |
| Nine two-stage compression chains | 165888 |
| **Total** | **4681728** |

Canonical materialization would use 74907648 bytes (71.4375 MiB) at 16 bytes per coefficient, before metadata and working buffers. Streaming/cached setup evaluation may avoid holding all matrices simultaneously, but does not make their generation or verifier weight work free.

With newly provisioned 512-bit flat modular samples, the native bias bound is
\[
\Delta_{setup}\le4681728\,q\,2^{-512}<2^{-361}.
\]
This is a new provisioning law, not a relabeling of existing frozen keys. Under the declared ideal setup/XOF model, distinct namespaces provide the independent views used by the proof.

The reduction's inventory has one MSIS advantage term for each of these 33 roles at its exact dimension/rank/width/radius (or the stated audited upward radius). Each runtime parameter includes the **whole** mixed-tree extraction, all repeated occurrences and collision/embedding work. Independence of roles does not divide the runtime by 33 or eliminate repeated attack opportunities. The sum is not replaced by a bare “128-bit” table label.

## 5. Source projection and all-valid-input contract

An original-source projection is `(registered role/view, P, B, decoder/source layout, original compressed payload)`; for the terminal it instead contains the original A-image tuple and inner-mode tag. The proof fixes this projection before any query/range branch that requires source uniqueness. It retains the complete supplied noncanonical source and decommitment path.

All public source shape/role fields are verified against the registry. Reject incompatible re-entry, mixed keys, changed digit order, missing F/H stages, aliased coordinate planes and unsupported profiles. The private backend must preserve raw source digits and original inner-image decompositions; current portable hints/canonical reconstruction do not already provide that all-valid-input API.

For the selected receiver every output is its actual N=2^22 source commitment, including required zeros and negative-binary spans. Subsequent roots use ReceiverRoot.A/B/F unchanged. Finishing deliberately creates Final2M, then Final1M, then Terminal512K sources under their distinct registered maps. It never changes the map of an already formed commitment.

## References

- [Compact receiver and final decision](AKITA_INSTANCE_ACCUMULATION_COMPACT_PROFILE.md).
- [Akita](SOURCE_PROVENANCE.md#akita), Lemma 10.15, Corollary 10.17, Proposition 10.20, Corollary 10.21, Eqs. (191)–(209), and the complete runtime-sensitive MSIS inventory.
- `akita/crates/akita-params/src/sis/generated_sis_table/q128.rs`, independently compared to immutable Akita 83574331d4e51f8cce1d8689d05592fa4ef4c138 by CompactProfileReview. Table checks are parameter-model evidence, not a security experiment.
