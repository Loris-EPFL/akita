# Native archive query accumulation: representation, capacity, and work certificate

## 1. Scope and conclusion

This is proof-first research under the [mixed-oracle contract](AKITA_INSTANCE_ACCUMULATION_MIXED_ORACLE.md), **independent of whether the outer extraction theorem is established**. Preserve the complete native archive, original K sources and handles, E challenges, 33 inner roles, at most 16 updates, three shrinking reductions, and actual A-only terminal. Only inner Theta_i queries are accumulated. The external verifier still authenticates the exact public accumulator using AS.D; an unauthenticated query-answer table is not an alternative. No CH-I/CH-O hypothesis, concrete hash instantiation, implementation, benchmark, or performance parity is approved here.

**Concrete negative result:** the direct one-shot CCGOS construction, using an injectively padded **flat full-transcript RO input**, canonical terminal K/E messages, and a fixed dense coefficient array for its complete polynomial f, does not fit the current byte-advice layout. This already holds for one full-size segment with 16-byte accumulator-field elements, even with the favorable lower bounds w=0 and d=2. Sixteen segments make the failure much larger. This is a capacity theorem for that specified representation, not a lower bound on all polynomial encodings or an impossibility theorem for Akita accumulation.

**Concrete positive information:** the native protocol supplies independently checkable query-count and message-length bounds much sharper than “16 segments” or c_E<=2^40. The direct construction has exact algebraic size formulas. Several parameter and model choices nevertheless remain unresolved: the bounded transcript byte codec, sampler retry caps, lambda_A and F, B_i/w/d, and a separately admitted lattice CM with the required message capacity and output length. A proof in the product ideal model does not select these automatically.

**Later security-scope correction:** the [ARO witness audit](AKITA_INSTANCE_ACCUMULATION_ARO_WITNESS_SCOPE.md) supersedes the unrestricted emulator and the previously imported zero-finding proof. This note's representation arithmetic and capacity lower bounds do not depend on those security claims. The repaired paired input-local emulator and the separately reviewed hidden-curve transport theorem must be used for current security accounting; neither turns these size counts into numerical receipt admission.

Sources distinguish the immutable inner design (Jolt 0791f79a55c3fadc3d327c78a72228696c338468, Akita 83574331d4e51f8cce1d8689d05592fa4ef4c138) from inspected current source facilities. In particular, current transcript machinery is not the unimplemented E-challenge archive grammar.

## 2. Exact CCGOS parameters and costs

Use n_q for the number of query entries passed to AS, ell for old accumulators, and

\[
 a=m+\lambda_A+w,\qquad N=n_q+2\ell+1.
\]

The one-shot parent construction has ell=0. Here **m, lambda_A, and w are bit counts**, hence Boolean-coordinate counts in the ARO graph; they are not byte counts or packed K-element counts. Construction 8.2 interpolates N graph-query points with a vector g of a coordinate polynomials, each of degree at most N-1. Vhat has individual degree at most d, so

\[
 \deg f=\deg(\widehat V\circ g)\le D_*:=ad(N-1)<adN.
\tag{1}
\]

A tight fixed dense coefficient representation therefore uses D_*+1 field elements. CCGOS footnote 13 allows adN evaluation/interpolation points; its §8.5 states the looser storage bound adN+1. We use the **smaller** D_*+1 representation below. Do not read an upper degree bound as a lower bound on the actual degree of every execution.

Let b_F=ceil(log2|F|/8) be a fixed canonical field-element byte width, r_CM the commitment-randomness byte length, and B_arch the archive bytes excluding the new query-accumulation witness. With fixed-width F-coordinate serialization:

| Object | Exact coordinate count or stated bound |
|---|---:|
| Public acc, two extended-oracle query-answer pairs | 2(a+1) F elements |
| Random extra pair in pi_AS | a+1 F elements |
| f, tight fixed dense representation | D_*+1 F elements |
| g, if materialized by the guest | aN F coefficients |
| Complementary witnesses for ro-only input ledger | n_q w bits; n_q w F elements if naively field-serialized |
| General mixed ro/wo/Vhat input complement | at most n_q max(lambda_A,w) F elements in the paper's representation |

Thus the literal field-serialized ro-only AS proof occupies

\[
 B_{AS}=b_F[(a+1)+n_qw+(D_*+1)]+r_{CM}+B_{frame}.
\tag{2}
\]

Its Boolean complements may instead be canonically bit-packed: replace b_F n_qw by ceil(n_qw/8), while checking bit values on reconstruction. This saves complements, **not f's arbitrary F coefficients or the random extra point**. The exact public mathematical object still contains 2(a+1) coordinates. A protocol-specific wire codec may pack the first pair's Boolean x,y,z and omit its fixed output 1, yielding ceil(a/8)+(a+1)b_F bytes plus framing; that changes serialization, not the accumulator relation. It does not pack the second arbitrary F^a point into bits.

A full padded ro ledger, if separately materialized, costs n_q ceil((m+lambda_A)/8) bytes plus identifiers/lengths. There is no need to serialize repeated transcript prefixes: derive x from the archive, record accepted and rejected raw answers and their deterministic indices, and feed the exact same reconstructed entries to AS.V. This is witness deduplication, not replacement of authenticated queries. A ledger may also deduplicate identical (x,y) entries if the skeleton proves every use maps to that same authenticated pair. Subsequent bounds on n_q then use distinct inputs, not instruction count. The sequential flat inputs used in §5 are distinct by typed site/round framing.

### CM is a real additional commitment

CM must be oracle-free, independently keyed, and binding to the canonical interpolation data determining **all of g**, and to the whole canonical f. One unambiguous full-field message encoding has

\[
 L_{CM}=B_{CM,frame}+b_F[aN+(D_*+1)].
\tag{3}
\]

Packing Boolean original graph points can reduce the aN portion, but the extra random point and arbitrary old-accumulator points remain field-valued. The binding reduction in CCGOS §8.3 explicitly views the commitment as a commitment to (g,f); do not omit y/z coordinates on the theory that binding only the original RO inputs automatically binds the interpolation curve.

If the selected lattice commitment outputs s_CM bits, the ARO input arity must accommodate its canonical challenge input: s_CM<=m in the paper's m-succinct convention, or s_CM plus any explicit namespace/framing bits <=m in a typed wrapper. Zero-padding a shorter CM output is injective; truncating a longer output is not. Equations (1)–(3) and this output constraint are **coupled**: increasing m to accommodate CM also increases CM's input message length. Its bounded-message lattice parameters, coefficient/source alphabet, randomness, collision radius, key distribution, and maximum message length require independent admission. The existing 33 Akita matrix roles, or a claim that A/B/F is homomorphic, cannot supply them. Theorem 8.1 also assumes hiding for its zero-knowledge result; merely dropping hiding does not import that conclusion, although Lemma 7.1's binding-only zero-finding argument is useful when no ZK is sought.

### Work, not just storage

From Construction 8.2 and §8.5:

* Prover: obtain n_q complementary wo answers for a ro-only ledger; sample one extra Vhat pair; interpolate g; obtain f using at most adN Vhat evaluations and interpolation; recompute CM and obtain ro(cm),wo(cm), with its stated bounded retries. Each wo evaluation entails the actual work/ro queries of fixed B_i.
* Oracle-free AS.V: reconstruct g, recompute CM, check g(beta), check f at all N interpolation nodes and beta. Naive Horner evaluation uses (N+1)D_* field multiplications for f; evaluating g from a coefficient representation costs at most a(N-1) multiplications. These are algorithms, not lower bounds: barycentric interpolation can avoid storing g and multipoint evaluation can reduce arithmetic.
* An implementation storing g takes aN b_F additional bytes. An implementation using interpolation weights directly can check g(beta) with O(aN) field operations and small working state; it still must process all required interpolation data and the committed f.
* The dense f representation alone requires reading (D_*+1)b_F bytes and commitment processing over the corresponding message. Streaming saves RAM, not byte-advice capacity when all coefficients are supplied as advice. A degree-adN object is not an O(1)-work certificate.
* AS.D uses exactly two Vhat calls, each on a coordinates. They are not two short ordinary hash calls. In a hypothetical concrete realization they evaluate the actual graph polynomial P_H on arbitrary field inputs, and their input parsing already scales with a. This note gives no circuit size or software timing for them.

CCGOS §8.5 has a line referring to ro queries while describing interpolation of f; Construction 8.2 footnote 13 explicitly identifies the required evaluations as **Vhat** queries. Those evaluations must not be counted as ordinary hash output blocks.

## 3. Fixed m versus native variable-length transcript

The primary ARO has ro:{0,1}^m->{0,1}^lambda_A. A finite segment count does not by itself fix m: public-I/O lengths, descriptor lengths, proof atoms, output-block indices and rejection attempts also require caps.

An exact flat implementation is possible after fixing a bound L on transcript-prefix bytes, a maximum output-block/retry index, canonical type/site encoding, and injective length framing. For example, let c be the fixed bit width of the namespace, site and block counter and let h=ceil(log2(L+1)). Define

\[
 \operatorname{Enc}(P,j)=\operatorname{type/site/block}_c(j)
      \Vert |P|_h\Vert P\Vert0^{8(L-|P|)},\qquad
 m=c+h+8L.
\tag{4}
\]

Every queried prefix must satisfy |P|<=L; the skeleton rejects overflow before allocation. Include all actual header, original commitment, evaluation, round-message and sampler-path bindings in P, in their exact order. This is not “hash only the most recent message.” CM inputs get an unambiguous encoding as well, with enough reserved domain capacity; if this modifies the literal ro(cm) interface, state the injective wrapper in the AS instantiation.

**Restriction lemma (proved here):** restricting a uniformly random fixed-input function to any deterministic injective set of encodings gives a uniformly random function on the encoded domain. Distinct Enc inputs receive independent uniform outputs; repeated inputs receive the same output. This proves exact flat variable-length RO emulation over the admitted bounded domain, without a collision term. It also covers arbitrary off-protocol queries by retaining the remaining fixed-input points in the same RO experiment. The auxiliary witness circuit must be fixed before sampling and respect the same encoding. No digest is used in this argument.

The necessary byte certificate is an ordered emission table. For each absorb event e record its concrete emitted length

\[
 b_e=b_{frame,e}+16 k_e+32 e_e+b_{raw,e},
\]

where k_e and e_e count canonical K and E elements. For each query q, record which absorptions/challenge-history framing precede it, and its output block/retry index. Then L=max_q |P_q| and total absorbed bytes=sum_e b_e. Summing full prefix lengths, sum_q |P_q|, instead measures flat-query processing if reconstructed afresh, not the archive's own byte size. Dynamic polynomial compression must use its **actual serialized coefficient count**, not interpolation arity D+1: current Jolt's compressed Boolean sumcheck absorbs coefficients except the linear term (`jolt-sumcheck/src/verifier.rs:106–123`). Current native codecs send all base coordinates of extension atoms and canonically reject out-of-range field bytes (`akita-transcript/src/proof_stream.rs:288–335`).

No complete byte-level archive codec currently exists in the reviewed design. Consequently a numerical **upper** value of L, including labels/framing and every native payload, is unresolved. The terminal payload alone supplies a rigorous useful **lower** value in §5. There is no license to set m=256 or m=512 for an 80 KiB flat query.

### Chaining is not used to evade this result

Current `TranscriptSponge` is feature-selected Blake2b512 or Keccak; its presence is not a theorem that the formal flat RO can be replaced by a small compression-RO chain. This note **does not propose or admit** such a replacement. A future proposal needs a transcript-oracle simulation/indifferentiability theorem for the exact length-delimited absorb/squeeze/retry protocol, with adversarial access to the component oracle, not merely collision resistance of honest transcript digests. It must count all compression calls, cross-session/site collisions, guessing of chaining states, output-block queries, and the added loss in the inner soundness experiment, then extend the argument to ARO side access. A birthday expression alone would not prove that theorem. No unproved collision-free-chaining premise is hidden in (4) or the capacity theorem.

## 4. Output width, exact samplers, graph witnesses and F

### Native E and shell sampling

K has q=2^128-2^32+22537 elements; E=K[U]/(U^2-5). One exact E draw consists of two independently uniform K coordinates. If each coordinate uses a 128-bit candidate and rejects integers >=q, its rejection probability is

\[
 \rho_K=(2^{32}-22537)/2^{128}<2^{-96}.
\]

For C_E scalar draws, an r_K-attempt cap **per coordinate** has union-bound failure at most 2 C_E rho_K^{r_K}. This is a declared bounded implementation with abort, not the unbounded exact sampler. Conditioned on success, rejection preserves uniform coordinates. With a well-defined lambda_A-bit output-block adapter, an unbuffered coordinate attempt consumes ceil(128/lambda_A) blocks; buffering disjoint bits from one block within a simultaneous vector draw is possible, but cannot pre-sample the next adaptive sumcheck round before its message. Every failed candidate and deterministic consumption rule must be checked by the skeleton.

For the selected shell,

\[
 M_C=\binom{128}{40}\binom{40}{24}2^{40},\quad
 \log_2 M_C=186.844275296\ldots.
\]

An explicitly specified alternative exact **sampling algorithm**, not an assumed current source behavior, takes 187 fresh bits, rejects ranks >=M_C, and un-ranks into support, magnitude positions and signs. It is uniform because rank-to-shell is a bijection. Its acceptance probability is 0.8976813305344475. With r_C attempts per shell coordinate, failure is at most C_C(1-M_C/2^187)^{r_C}; it uses at most C_C r_C ceil(187/lambda_A) unbuffered blocks. A Fisher–Yates/support sampler is also possible, but must emit its individual bounded-integer/rejection draws instead; “one shell challenge” need not mean one RO call. The fixed compact archive has exactly

\[
 C_C=120T+32+16+16+8=120T+72\le1992
\tag{5}
\]

shell coordinates, not 16.

Current native `proof_stream/sampling.rs:9–68` supports canonical rejection sampling up to 64-byte candidates. Its query-limit constant is expressly **not a retry cap**; its generic loop is unbounded (the Keccak counter additionally detects u32 overflow and binds the attempt count). Current Jolt frontend uses 16-byte challenge draws, not this new E sampler. Neither source fact supplies an admitted bounded sampler/flat-RO adapter.

Given the scalar bound C_E in §5, an explicit unbuffered adapter certifies

\[
 n_q\le 2C_Er_K\lceil128/\lambda_A\rceil
       +C_Cr_C\lceil187/\lambda_A\rceil+n_{other}.
\tag{6}
\]

Here n_other contains any actually selected inner identity/session/XOF queries, not outer H_o queries. Installed setup coefficients are not regenerated in the guest, so their setup XOF is not silently charged or silently executed here. AS.P's own CM challenge, wo queries and O(adN) extended-oracle evaluations are **additional producer/reduction work**, not recursively included as new native ledger entries in n_q. This distinction avoids a self-referential count. They still enter any security query/work budget.

### ARO field and graph are separate choices

* F need not equal K or E. A clean bit-to-field convention is F=GF(2^lambda_A), using a fixed basis and all lambda_A bits for beta. This is an exact uniform AS challenge but requires characteristic-two field arithmetic inside the integer guest and its own soundness parameters. It does not change native source K or challenge E.
  A concrete **field/bit-interface option**, certified by the parent research arithmetic, is F=GF(2)[X]/(p), p=X^512+X^8+X^5+X^2+1, with lambda_A=512. The reported checks are X^(2^512) mod p=X and gcd(p,X^(2^256)+X)=1. By the finite-field irreducibility criterion these suffice because 2 is the only prime divisor of 512. In the polynomial basis, 512 bits map bijectively to F; coefficients use 64 canonical bytes. There is no bit-to-field bias or rejection, apart from Construction 8.2's separate retry when beta belongs to its interpolation domain. This option closes the mapping issue, not the concrete hash-graph, overall security, implementation or performance gates; native K/E are unchanged. The checks were performed by the parent, not rerun here.
* With F=K or E, arbitrary lambda_A-bit reinterpretation is not exact uniform sampling. One must specify the AS beta sampler and adapt the proof, or carry the exact maximum point probability p_max in the root bound (a nonzero degree-D polynomial has at most D roots, giving at most D p_max). An injective subset map with fewer bits has support only 2^lambda_A, regardless of |F|. Reducing modulo |F| needs its bias term. The native E sampler is not automatically the AS sampler.
* Need N distinct interpolation nodes, and D_*+1 distinct evaluation points if constructing f by direct interpolation over F. Thus |F|>=D_*+1 for that method, in addition to a sufficiently strong root/error bound. A small field cannot be rescued merely by byte-packing coefficients. CCGOS Theorem 8.1 has a square-root security expression; native E soundness numbers do not certify F or lambda_A.
* In the ideal model, B_i is a fixed efficient ro_i-only circuit and w is its exact output-bit length. w=0 is a valid favorable **ideal parameter**, not a concrete hash-graph implementation. For a concrete H one needs a complete Boolean graph relation with unique canonical internal-wire assignment W_H(x), w counting every witness bit, and a graph polynomial of certified individual degree d. CCGOS §2.2 explains the circuit-to-3SAT route and why total degree grows with circuit size. Bounded wire fanout/equality copies can bound variable occurrence, but add witness bits. A gate count alone is not d; the individual-degree certificate must count occurrences in the actual product-of-clause factors. d>=2 is required by the paper's emulator setting. Neither w nor d is currently supplied for the entire padded hash/XOF interface. No numeric “hash circuit size” is invented here.

## 5. Source-grounded native counts and a capacity theorem

### Frontend emissions for the fixed numerical example

The [frontend manifest](AKITA_INSTANCE_ACCUMULATION_FRONTEND_MANIFEST.md) fixes n_trace=12, m_RAM=16, b_bytecode=12, k=4, and no optional field/program/advice objects in the [compact example](AKITA_INSTANCE_ACCUMULATION_COMPACT_PROFILE.md). It retains the 128 instruction-address rounds. The eight sequential batch round counts are

\[
 (13,28,12,19,140,12,12,4),\qquad\sum R_j=240.
\]

Both standalone uni-skips add a round, giving **242 adaptive message/challenge boundaries per segment**. This is backed directly by the source order `append round; challenge; update running sum` (`jolt-sumcheck/src/verifier.rs:63–77,106–123`). With a flat typed transcript oracle and no unexplained pre-sampling, there are at least 242 distinct native inputs per segment even if simultaneous vectors share output blocks:

\[
 n_q\ge242T.
\tag{7}
\]

Removing RamOutputCheck on nonfinal segments does not remove RAM RW's n_trace+m_RAM batch rounds. Its reference-vector and independent member-coefficient draws do disappear. Source-derived frontend E-coordinate counts, charging every simultaneous coordinate but not multiplying by polynomial degree, are:

| Stage | E coordinates, final segment |
|---|---:|
| 1: tau, uni-skip, batch coefficient, rounds | 14+1+1+13=29 |
| 2: tau, uni-skip, two gammas, RAM reference, coefficients, rounds | 1+1+2+16+5+28=53 |
| 3 | 3+3+12=18 |
| 4 | 2+2+19=23 |
| 5 | 2+3+140=145 |
| 6a | 6+1+2+12=21 |
| 6b | 1+5+12=18 |
| 7 | 1+1+4=6 |
| Total | 313 |

Nonfinal total is 296 after removing 16 RAM reference coordinates and its one member coefficient. There is no dense-source stage-8 selector in this particular inner example; do not transfer this zero count to the outer trusted/untrusted advice workload. The old one-scalar PCS bridge is not admitted for the new archive; any new adapter calls go in n_other.

### Full archive scalar certificate, not just frontend

The [global manifest](AKITA_INSTANCE_ACCUMULATION_GLOBAL_MANIFEST.md) retains every range anchor, round, frontier coefficient and source normalization. Reading its actual grammar instead of its degree-weighted error numerator gives the following safe **coordinate upper certificate** for the compact profile:

* Base-32 raw range prefix, arity mu: mu anchor coordinates + 2mu degree-five rounds + mu virtual-meaning rounds + 3 interstage coefficients = 4mu+3. Old-receiver binary/zero combination adds at most two, giving 4mu+5. Reused anchors are not fresh draws.
* A generic full-main Boolean prefix: mu reference + mu rounds = 2mu. Charging this optional prefix even though the selected physical-main reuse omits it is a safe overcount.
* One multipoint normalizer: Lambda independent coefficients + mu rounds, or zero when legitimately bypassed on a singleton.
* Outgoing native strong check: kappa row coordinates + mu range anchor + 2mu range rounds + mu fused rounds + 3 frontier coefficients + 3 fused coefficients = kappa+4mu+6. Shell coordinates are counted separately in (5).

For one update this yields:

| Family | Coordinate upper count |
|---|---:|
| Raw prefixes: old mu=22, generic main mu=22, two RAM mu=21 | 93+44+2*87=311 |
| Multipoint lists <=(2,119,3,3), arities (22,22,21,21) | 24+141+24+24=213 |
| Outgoing mu=22, kappa=12 | 106 |
| Endpoint/physical-hole checks | <=116 |

The last row includes at most two endpoint-combination coordinates, two PC moment protocols (2*3+2*4=14), and physical-main zero check (22 reference+56 coefficients+22 rounds=100). The selected register-only new coefficient and main-list size 118 reduce this bound; no reduction is assumed. Three suffix outgoing checks with mu=(21,20,19) and kappa<=12 cost at most 102+98+94. Their standalone incoming final receiver prefix and two-claim normalization cost at most 93+24. Together these are 411. Once-only initial/final allowances of 22+64=86 safely overcount the reused initial address and the retained final RAM output check.

Accordingly, for the stated source-query/coefficient-relation grammar, with no extra optional sampler/bridge gates,

\[
 C_E\le16(313+311+213+106+116)+411+86
      =17441.
\tag{8}
\]

This is a conservative syntactic certificate for the specified **unimplemented** grammar, not measured executions. It deliberately charges some absent/reused checks. Any new emitted gate changes the certificate. For another catalogue shape use the frontend table, actual Lambda/arity/presence descriptors, and the preceding formulas; do not use 17441 globally. Actual RO calls still require (6), because coordinate counts are not rejection counts or hash-compression calls. In particular, the declared Q32 native policy does not automatically cover AS.P's enormous Vhat workload or the outer extractor's accesses.

### The flat input has a large unavoidable canonical message

Before the terminal shell challenges, the compact full-ring terminal binds eight A images (8*3*128 canonical K elements = 49152 bytes) and eight E-ring partials (8*128 canonical E elements = 32768 bytes). Both are explicit prechallenge material in [compact profile §§3–4](AKITA_INSTANCE_ACCUMULATION_COMPACT_PROFILE.md). For the **flat transcript and canonical field representation being evaluated**,

\[
 L\ge81920\text{ bytes},\qquad m\ge655360\text{ bits}.
\tag{9}
\]

The bound ignores all prior frontend/update/suffix messages, all point/commitment metadata and all framing, and is therefore conservative. It does not count the terminal's later 131072-byte signed-i16 response as prechallenge input. If someone replaces the terminal messages by a digest or changes the field-message codec, (9) must be reconsidered with that changed protocol; that is not the present flat construction.

### Current byte-advice capacity is at most 512 MiB for the untrusted region

Inspected modern source: `jolt/common/src/jolt_device.rs:378–442` eight-byte aligns capacities, requires each nonzero advice capacity to be a power of two, computes

\[
 I=8\operatorname{nextpow2}((P_{in}+C_t+C_u+P_{out}+16)/8),
\]

and requires I<RAM_START_ADDRESS=2^31. The padded I/O region is itself a power of two, so **I<=2^30**, not “almost 2 GiB.” The static coefficient blob alone is 4681728*16=74907648 bytes, hence its power-of-two byte-advice capacity satisfies C_t>=2^27 (128 MiB). Therefore

\[
 C_u\le2^{30}-2^{27}-P_{in}-P_{out}-16
       <939524096.
\]

Since C_u must be zero or a power of two, **C_u<=2^29=536870912 bytes (512 MiB)**. Metadata/caches and public inputs can only tighten this. Raising the configured untrusted capacity to 1 GiB would force total padded I/O to 2 GiB and be rejected. Heap/stack lie separately above RAM and cannot be substituted for authenticated advice transport.

This is a byte-advice limit, not the dense field-vector footprint. `jolt/common/src/advice.rs:30–45` packs eight bytes into each u64 word. At the minimum trusted capacity there are 2^24 advice words; materializing them as 16-byte K elements takes 256 MiB. At the maximum untrusted capacity there are 2^26 words, or 1 GiB if materialized as 16-byte K elements. Those separate witness-memory costs are not additional byte capacity.

### Capacity theorem and arithmetic

Consider a fixed-width coefficient array of length D_*+1 supplied through the single untrusted byte-advice object. Combine (1), (7), (9), ell=0, d>=2, and w>=0. Even discarding lambda_A from a,

\[
 D_*+1\ge655360\cdot2\cdot242T+1.
\tag{10}
\]

Arithmetic evaluation of these integers gives:

| T | Lower n_q | Lower D_* | f bytes if b_F=16 | f GiB if b_F=16 | f GiB if b_F=32 |
|---:|---:|---:|---:|---:|---:|
| 1 | 242 | 317194240 | 5075107856 | 4.726562515 | 9.453125030 |
| 16 | 3872 | 5075107840 | 81201725456 | 75.625000015 | 151.250000030 |

Every entry exceeds the 512 MiB admissible untrusted region before archive, ledger, complements, CM randomness, framing or temporary storage. The 16/32-byte field widths are favorable **representation illustrations**, not admitted AS security parameters. For T=16 even a one-byte-per-coefficient representation exceeds the capacity, although such a tiny field would fail the interpolation requirements. The still larger adN+1 allocation from §8.5 cannot help.
For the certified 512-bit field option, the same favorable polynomial lower bounds become more than 18.90625 GiB at T=1 and more than 302.5 GiB at T=16. Even before polynomial degree is considered, a 512 MiB advice region holds only 2^23 64-byte coefficients if it contains nothing else, and strictly fewer once the required archive and other proof data are included. Retaining lambda_A=512 in a strengthens these already failing lower bounds.

With the naive full-field public codec, (9) alone also gives acc sizes of at least 20971552 bytes for b_F=16 and 41943104 bytes for b_F=32; the packed first-pair codec can reduce this but leaves at least (a+1)b_F bytes in the second point. Thus “two oracle queries” is not an automatic small receipt when m is a full transcript length.

**What has been proved:** that fixed dense coefficient-capacity representation fails the current byte-advice layout for the displayed profiles. **What has not been proved:** every realized f has maximum degree, every f is incompressible, every possible circuit/witness representation needs this many advice bytes, or all succinct lattice receipts are impossible. No hash-circuit size, trace cycle count, speed, or parity claim follows from this arithmetic.

## 6. Alternatives that retain actual query authentication

### A. Keep CCGOS, change physical representation/layout

A changed guest/advice architecture could stream authenticated private coefficients from a larger admitted source domain; it would need a real new outer source/RAM/PCS geometry, not merely a larger number passed to today's MemoryConfig. The formula and the public a-coordinate decider cost persist. A smaller representation for f must come with an oracle-free, sound way to recompute CM and verify **every** f(b_k),f(beta); giving only a claimed commitment and values loses exactly the authentication that CCGOS supplies. Nothing in Construction 8.2 certifies compressibility of the random restriction polynomial.

A lattice PCS for f is a potential separate research direction, not a plug-in improvement: replace full CM recomputation with an authenticated opening relation, retain binding to the same g and f, prove the resulting zero-finding theorem, and account for the PCS verifier's own oracle calls without putting H_o into the inner ledger or making R_skel oracle-dependent. Existing Akita A/B/F maps are not assumed homomorphic, and recursively verifying an ordinary FS PCS here would reintroduce the oracle issue.

### B. CCGOS small batches / accumulation tree

Construction 8.2 explicitly permits old accumulators, so a bounded tree of genuine AS applications is an admissible *ideal-model query-authentication mechanism*, conditional on its stated soundness premises. Each node with n_v fresh queries and ell_v children has degree capacity ad(n_v+2ell_v), public output 2(a+1), and its own complementary/random witness and CM. Soundness can be charged over the actual node applications; each accepted leaf still needs a path to the externally decided root.

This can reduce **peak per-node polynomial memory**, but does not by itself fix total advice in a single skeleton proof: for a tree with J nodes and all n_q original leaf queries, sum_v(n_v+2ell_v)=n_q+2(J-1). Dense coefficients therefore total ad[n_q+2(J-1)]+J, at least the one-shot total and usually more. The guest must verify every node and its links; hiding all intermediate AS.V witnesses without another proof is not possible. Taking multiple independent public accumulators instead increases receipt size and external decider calls. This is not accumulation of completed segment PCS proofs, but it also is not a free compactness repair.

### C. Smaller fixed-input transcript primitive

Potentially addresses the large m factor, unlike batching. It is **not admitted here** for the reasons in §3. Before claiming a candidate fits, supply the new transcript theorem, collision/guessing terms against the full oracle-access adversary, a counted block-level ledger, and B_i/w/d for that primitive. A concrete hash circuit with no new theorem is exactly the unapproved CH route. The exact injective flat encoding above remains available if preserving the original flat oracle is mandatory.

### D. Primary alternative structured-oracle literature

[CCGOS, §2.3 and §8](SOURCE_PROVENANCE.md#supplemental-primary-sources) explicitly builds on the Kalai–Raz interactive query-reduction protocol and on Chen–Chiesa–Spooner. The [CCS22 primary publisher abstract and Notes 2/4](https://link.springer.com/chapter/10.1007/978-3-031-07085-3_12), *On Succinct Non-interactive Arguments in Relativized Worlds*, establish a low-degree random-oracle/linear-code-oracle approach with **actual oracle-query accumulation** and standard-model collision-resistant hashing. This is a genuine positive model alternative, not a table of unauthenticated answers. However it changes the oracle model; the abstract does not give a ready-made Akita/lattice-PCS interface or concrete parameters. Keeping the outer Akita lattice proof and introducing only an independently admitted lattice commitment/CRH for its query accumulator would require the exact alternative construction and a new composition theorem. Importing that paper's generic SNARK wholesale would not establish the requested lattice PCS.

CCGOS §2.2 explains why naively arithmetizing a standard hash as a low-degree function is not a numerical fix (degree growth), motivating graph witnesses instead. Its size discussion supports investigating genuinely smaller authenticated query reductions, not asserting that a standard hash already has the required ideal-oracle behavior. No primary result inspected here proves a turnkey efficient replacement satisfying all of this project's constraints.

## 7. Admission information still needed

A fully instantiated cost/profile certificate must supply, without weakening the native anchor:

1. The exact bounded archive/transcript framing, maximum input/output/descriptor lengths, all absorbs, and L; or a proved alternative transcript construction with its complete oracle-call and collision ledger.
2. lambda_A, F and canonical encodings, beta distribution, distinct interpolation domains, native E/shell bit-consumption algorithms, retry caps and abort bounds.
3. Fixed ro_i-only B_i, its output width w, runtime/query bounds, and certified individual degree d; a concrete graph is a further model-instantiation gate, not a consequence of the ideal theorem.
4. An independently admitted oracle-free lattice CM satisfying (3), output<=m, exact source/collision bounds and any desired hiding property.
5. The chosen AS witness representation and all public/private byte and work counts. The literal dense-flat option is already excluded by §5; it cannot be revived by assuming fewer than the actual adaptive frontend rounds.
6. A full outer workload/source/advice profile for any alternative architecture and an error ledger charging AS production/verification/extraction queries, not just native transcript draws.

Only arithmetic evaluations of the displayed formulas were executed. No code changes, Git operations, source builds, tests, formatters, benchmarks, or empirical performance measurements were performed. Only this research note was created.

## References

* CCGOS accepted manuscript, [PDF](SOURCE_PROVENANCE.md#supplemental-primary-sources), [extracted text](SOURCE_PROVENANCE.md#supplemental-primary-sources): §2.2 pp.6–8, Definition 4.1 p.23, Lemma 7.1 p.38, Theorem 8.1/Construction 8.2 pp.42–43, §8.3 pp.44–45, §8.5 pp.46–47. The full text, not only an abstract, grounds the direct construction formulas.
* [Mixed-oracle contract](AKITA_INSTANCE_ACCUMULATION_MIXED_ORACLE.md); [ARO analysis](AKITA_INSTANCE_ACCUMULATION_ARITHMETIZED_ORACLE.md); [receipt source/advice contract](AKITA_INSTANCE_ACCUMULATION_RECEIPT_CODE_MAP.md).
* [Compact receiver/suffix/terminal profile](AKITA_INSTANCE_ACCUMULATION_COMPACT_PROFILE.md), [frontend manifest](AKITA_INSTANCE_ACCUMULATION_FRONTEND_MANIFEST.md), [complete native manifest](AKITA_INSTANCE_ACCUMULATION_GLOBAL_MANIFEST.md), [digit-axis normalizer](AKITA_INSTANCE_ACCUMULATION_DIGIT_AXIS.md), [matrix registry](AKITA_INSTANCE_ACCUMULATION_MATRIX_REGISTRY.md).
* Current source inspected: [memory layout](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/common/src/jolt_device.rs), [advice packing](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/common/src/advice.rs), [sumcheck verifier](https://github.com/Loris-EPFL/jolt/blob/akita-query-relation-accumulation-v8/crates/jolt-sumcheck/src/verifier.rs), [native canonical atoms](../../crates/akita-transcript/src/proof_stream.rs), [native rejection sampling](../../crates/akita-transcript/src/proof_stream/sampling.rs), [sponge feature selection](../../crates/akita-transcript/src/sponge.rs). These facilities do not claim to implement the new native archive.
