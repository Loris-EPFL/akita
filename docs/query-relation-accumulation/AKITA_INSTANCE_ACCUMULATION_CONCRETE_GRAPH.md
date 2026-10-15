# Concrete degree-two hash graphs: construction, costs, and instantiation boundary

## 1. Result and invariant

This note specifies a concrete candidate `(H_m,W_m,P_m)`, proves its Boolean graph semantics and individual-degree bound, and derives constructive resource bounds. **It does not instantiate the ideal ARO theorem.** The user authorized an explicitly heuristic prototype; the separately reviewed [D1 model](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) states its exact native-game conjecture, restricted CH-I implication and unknown concrete loss. No broad CH-I/CH-O or quantified concrete-security theorem is inferred.

The preserved anchor is the original Akita pre-PCS segment archive, original lattice source handles, all-valid-source semantics and complete native final decider. Graph-witness/quotient costs below belong to the retained ideal graph branch. D1 selects H_1610 and specifies W/P as public diagnostic algorithms, but its guest computes **H only**: no graph witness, C_t, quotient opening or external graph decision. Neither branch adds commitment homomorphism or completed-proof folding.

The parent's AND/XOR/NOT/COPY/SPLIT construction works, with an important bookkeeping convention: **all original gate outputs, including wires later compared with external output bits, belong to the witness**. Constants are literals rather than free unconstrained wires; fanout counts include repeated input ports and output-comparison ports. With these conventions, `w=G+2S` is exact. Efficient graph evaluation is established below; concrete security and practical efficiency of the whole receipt are different questions.

**Independent review:** ConcreteGraphReview approved the graph construction, pinned duplex adapter, counts and security-boundary argument with no findings. It verified the 22-byte domain, 86 round-constant bits, unique witness/degree proof and numerical upper envelopes. Approval explicitly excludes concrete receipt security, full-profile admission and parity.

## 2. Source evidence and representative primitive

### 2.1 What CCGOS actually establishes

[CCGOS, *Proof-Carrying Data From Arithmetized Random Oracles*, accepted manuscript](https://wrap.warwick.ac.uk/id/eprint/176570/1/WRAP-proof-carrying-data-arithmetized-random-oracles-Spooner-2023.pdf), §2.2, printed pp.6–8, constructs a Boolean graph predicate with a unique efficiently computable internal-wire witness, and proposes its arithmetization as a **heuristic** software interpretation. The inspected [local text](SOURCE_PROVENANCE.md#supplemental-primary-sources), lines 309–417, explicitly states:

* the concrete polynomial has total degree `O(|H|)`;
* the analysis is in an idealized model;
* the off-Boolean extension is sampled uniformly among bounded-individual-degree extensions of the graph;
* weakening this modeling choice is left to future work.

The explicit SPLIT family and operation accounting below are derived here, not a claim that CCGOS proves these particular formulas, this Keccak instantiation, or its concrete security.

### 2.2 Pinned Akita is not simply “SHAKE transcript”

The supplied Akita revision is `83574331d4e51f8cce1d8689d05592fa4ef4c138`; this investigation uses the supplied tree and did not query Git. Inspected evidence:

* [`akita-transcript/Cargo.toml:17–28`](../../crates/akita-transcript/Cargo.toml): default `transcript-blake2b`; alternative `transcript-keccak`; Spongefish revision `ef9741346a150039427d9e0c02d6b2d73e93ec81`.
* [`src/sponge.rs:1–9`](../../crates/akita-transcript/src/sponge.rs): `Blake2b512` or `Keccak` respectively. [`src/lib.rs:9–13`](../../crates/akita-transcript/src/lib.rs) requires exactly one backend.
* [`proof_stream.rs:179–220`](../../crates/akita-transcript/src/proof_stream.rs): u64-little-endian length framing of session/instance bytes and distinct native-proof-stream/v7 backend protocol identifiers.
* [`proof_stream/sampling.rs:48–65`](../../crates/akita-transcript/src/proof_stream/sampling.rs): the Keccak option absorbs a rejection-attempt marker because this duplex otherwise forgets squeeze length after a later absorb. That marker is protocol semantics, not an optional optimization.
* SHAKE256 does exist separately in [`akita-types/src/proof/setup.rs`](../../crates/akita-types/src/proof/setup.rs), for paged public-matrix generation. It is not the native transcript primitive.

The exact pinned [Spongefish instantiation source](https://github.com/arkworks-rs/spongefish/blob/ef9741346a150039427d9e0c02d6b2d73e93ec81/spongefish/src/instantiations/mod.rs) defines

`Keccak = DuplexSponge<KeccakF1600,200,136>`

and explicitly warns that this is **not SHA-3**: it is an overwrite-mode duplex. Its [permutation source](https://github.com/arkworks-rs/spongefish/blob/ef9741346a150039427d9e0c02d6b2d73e93ec81/spongefish/src/instantiations/permutations.rs) uses 25 little-endian 64-bit lanes and full Keccak-f1600. Its [duplex implementation](https://github.com/arkworks-rs/spongefish/blob/ef9741346a150039427d9e0c02d6b2d73e93ec81/spongefish/src/duplex_sponge.rs) initializes 200 zero bytes, overwrites rate bytes when absorbing, permutes when another input chunk follows a full rate block, and permutes before the first nonempty squeeze. It applies no SHAKE suffix or pad10*1. The source itself distinguishes an ideal-permutation analysis from concrete instantiation.

The default Blake2b option is also not a bare Blake2b digest: its pinned [hash adapter](https://github.com/arkworks-rs/spongefish/blob/ef9741346a150039427d9e0c02d6b2d73e93ec81/spongefish/src/instantiations/hash.rs) adds absorb/squeeze masks, chaining values, ratcheting and fixed-u64 squeeze indices. Pricing one compression as the whole default transcript would be incorrect.

**Representative chosen here:** the already-present optional Keccak duplex, not a new permutation or hash design, with a fully specified fixed-input/single-squeeze adapter below. This choice does not assert that the default backend was changed or that this adapter reproduces current full-transcript outputs. It is the candidate component for the proposed ARO transcript grammar. The grammar's existing ideal-model theorem and the concrete instantiation decision remain separate.

## 3. Fully specified fixed-input family

For each public integer `1 <= m < 2^64`, define `H_m:{0,1}^m -> {0,1}^1024` as follows.

1. Let `D` be the 22 ASCII bytes `akita/aro/component/v1`, with no NUL terminator. Let `L=ceil(m/8)`. For input coordinates `x_0,...,x_(m-1)`, form `L` bytes `p_j=sum_(0<=k<8,8j+k<m) 2^k x_(8j+k)`. The unused high bits of the last byte are zero. This is a fixed injective packing of the m input bits; it does not discard their typed transcript tags or reinterpret integer fields.
2. Set `M=D || LE64(m) || p`. Its length is exactly `30+L` bytes. The component's native absorb/squeeze/CM operation tags and payloads are already inside x according to the selected transcript compiler. Invalid typed strings remain ordinary inputs to H; the higher-level protocol rejects their use as encodings, not their existence as oracle queries.
3. Initialize the pinned Spongefish `Keccak` to its default zero state, absorb M once, and squeeze exactly 128 bytes once. Reset for each separate H input. Output each byte least-significant-bit first, giving the 1024 y coordinates. There is no ratchet call, subsequent absorb, hidden output truncation, or SHAKE padding.

Equivalently, split M into nonempty chunks of at most 136 bytes. Overwrite state bytes 0 through chunk_length−1 with each chunk, leaving other bytes unchanged, then apply Keccak-f1600. After the last permutation return state bytes 0 through 127. This gives exactly

`p_m = ceil((30+ceil(m/8))/136)`

permutations: the pinned code performs all but the last during absorb, and the last at first squeeze. Exact rate multiples require no extra block. This equivalence follows directly from the absorb/squeeze cursors. Fixed lengths and the explicit m prefix prevent silently identifying distinct-width families; they do not constitute an ARO security proof.

### 3.1 Canonical bit circuit

Use the Keccak-f1600 permutation specified by [FIPS 202](https://doi.org/10.6028/NIST.FIPS.202), §§3.1–3.4, Algorithms 1–7, and the [designers' round pseudocode/constants](https://keccak.team/keccak_specs_summary.html). This is the common permutation only: the **mode** here is the pinned overwrite duplex, not the FIPS SHAKE mode.

For each of the `p_m` permutations, unroll all 24 rounds. Lane/state coordinate order is x=0..4, y=0..4, bit=0..63 with FIPS string index `64*(x+5y)+bit`; use increasing coordinates to assign gate IDs within each phase. Compute:

* θ: each five-bit column parity with four left-associated binary XOR gates; each D bit with one XOR; each new state bit with one XOR.
* ρ and π: fixed index permutations only, no gates.
* χ: for each bit, one NOT on the next lane bit, one AND with the following lane bit, then one XOR with the original bit. No common-subexpression optimization is assumed.
* ι: NOT only those lane-(0,0) bits whose published round constant is one; other bits are aliases.

Overwrites, input placement, lane rearrangements, output selection and fixed zeros are wires/literals, not arithmetic gates. Topologically propagate constants, using literal truth tables and `v XOR 0=v`, `v XOR 1=NOT(v)`, `v AND 0=0`, `v AND 1=v`; retain any resulting nonconstant NOT gate. Make no additional algebraic/CSE optimization. Then remove precisely the gates that are not ancestors of any selected output. Assign the remaining gates their inherited order. This is a finite deterministic circuit specification, not an instruction to find an optimal circuit.

The resulting circuit has G gates of fan-in at most two. A gate result that is used twice still has one producing gate and two consumer **ports** before splitting. Literal output bits are permitted by the general definition below. All live gate outputs are included in the witness, even if they are also selected outputs.

## 4. Fanout elimination, witness, and polynomial

### 4.1 Exact split rule

For each original variable-valued source v (input or live gate output), count its f_v consumers, including every gate input port and every output comparison. Do not merge two equal-valued ports. Constants are substituted literals and have no such consumers. Sort consumers by gate ID/input-port index, followed by output index. If f_v=1, connect directly. If f_v>=2, replace its fanout by a right-comb binary tree of `f_v-1` SPLIT factors: each splits a parent into two fresh child variables, the left child feeds the next consumer and the right child continues; the final split feeds the last two consumers. For f_v=0 add nothing.

Each split enforces **one** ternary relation `parent=left=right`. It is not two independently squared equality factors. Split trees subdivide outgoing edges of an acyclic circuit and do not create cycles.

Let S be the total split nodes. Define W_m(x) to contain, in order, (i) all G original live gate-output bits and (ii) both child bits of every split, with sources ordered inputs then gate IDs and split nodes in tree preorder. Compute it by the deterministic circuit and copies. Hence

`w = G+2S`,

exactly. W is polynomial-time, deterministic, ro-free and input-local. It is an eligible B function for the repaired input-local witness contract. In the ideal experiment, this fixed B is not magically a trace of the independently sampled ro; the replacement of its concrete relation with the ideal graph is part of the model boundary.

### 4.2 Factors

Over the selected field F, use the following polynomials, with distinct variable ports after splitting:

| Relation | Indicator | Total degree |
|---|---|---:|
| z=x AND y | `1-z-xy+2xyz` | 3 |
| z=x XOR y | `1-x-y-z+2xy+2xz+2yz-4xyz` | 3 |
| z=NOT x | `x+z-2xz` | 2 |
| y=COPY x | `1-x-y+2xy` | 2 |
| SPLIT x=u=v | `1-x-u-v+xu+xv+uv` | 2 |
| z=1; z=0 | `z`; `1-z` | 1 |

Use one factor for every original gate, one for every split and one COPY comparison between each variable-valued output wire and its distinct external y_j. A literal output uses y_j or 1−y_j instead. Fixed internal constants are substituted, not additional witness variables. Let P_m be the product of these `Fct=G+S+o` factors, where `o=1024`.

### 4.3 Correctness and uniqueness proof

On Boolean arguments every displayed factor is exactly 0 or 1. For example SPLIT takes value 1 at 000 and 111, while strings of Hamming weight one or two give zero. The other rows are their indicated truth-table predicates (equivalently the AND and XOR rows compare their Boolean result to z). This proof works in the actual odd-characteristic F; no rounding or integer interpretation of field elements is used.

A product of these Boolean-valued factors equals 1 iff every factor equals 1. With x fixed, topological induction first fixes each circuit output from its inputs and each split's two children from its parent. Thus there is **exactly one** satisfying internal assignment W_m(x). The last factors force every y_j to the corresponding circuit output. Conversely that trace and H_m(x) make all factors equal 1. Therefore, on the Boolean cube,

`P_m(x,y,z)=1 iff y=H_m(x) and z=W_m(x)`,

and P_m=0 on every other Boolean assignment. This proves the full graph predicate, not merely existence of some witness for a correct y. There are no unbound dead wires.

Every factor is multilinear in its distinct variables. Each internal/copy variable appears in one producing factor and at most one consuming factor. An input variable appears in at most one consuming factor; each external output variable appears in one comparison. Therefore **every variable has degree at most two in P_m**, even though the total degree grows with circuit size. Repeated uses inside an original gate must be split too; failing that would invalidate this argument. Replacing SPLIT with two equality checks involving the same parent would also invalidate the stated two-factor occurrence count.

If `G_2` is the number of binary gates and `G_1` unary gates, `o_v` the variable outputs and `o_c=o-o_v`, then

`deg_total(P_m) <= δ := 3G_2+2G_1+2S+2o_v+o_c <= 3G+2S+2o`.

These are upper bounds, not claims that all highest-degree coefficients survive multiplication. The ideal ARO still uses the general bound `2a`, not δ: a uniform degree-two extension is not this product.

The parent separately reported checking 72 Boolean gate/SPLIT/COPY/NOT/constant cases across F5 and actual K, and all 1024 assignments for the two-input circuit `a=x0 XOR x1; b=a AND x0; output=(b,a)` with two splits/six witness wires. Its product accepted exactly the unique honest trace and its maximum factor occurrence was two. This is parent-reported finite evidence supplementing the proof, not a hash implementation or ARO experiment; this investigation did not rerun it.

## 5. Width and work: exact expressions and constructive bounds

### 5.1 Exact fanout accounting

Let I be the number of variable-valued original gate input ports, and m_used the number of input bits that have at least one live consumer. Every live gate output has a consumer after dead-gate removal. Thus

`S = sum_v max(f_v-1,0) = I+o_v-G-m_used`,

`w = G+2S`,

`S <= G+o_v-m_used`, and `w <= 3G+2o_v-2m_used <= 3G+2o`.

The exact first equations, together with the deterministic circuit in §3, fix the exact width for each m without pretending that a pre-pruning count is the measured post-pruning gate count. COPY gates, if deliberately added to another circuit, must be counted in G and I; the specified Keccak circuit uses aliases instead. A convention with explicit constant-producing witness wires would require counting those gates/wires too and would be a different w.

### 5.2 Keccak constructive gate counts

For a full round before constant propagation/dead-gate pruning:

| Phase | Binary XOR | AND | NOT |
|---|---:|---:|---:|
| θ column parities | 1280 | 0 | 0 |
| θ D values | 320 | 0 | 0 |
| θ state update | 1600 | 0 | 0 |
| χ | 1600 | 1600 | 1600 |
| ι | 0 | 0 | popcount(RC_round) |

The 24 published round constants have total Hamming weight 86. Thus one full permutation has the literal bound

`115200 XOR + 38400 AND + 38486 NOT = 192086 gates`.

Rotation, overwrite and output selection cost no Boolean logic gates in this representation. Constant propagation or removing unneeded final state outputs can only reduce the count. Consequently

`G <= Gbar := 192086 p_m`,

`w <= wbar := 576258 p_m + 2048`,

`a=m+1024+w <= abar := m+3072+576258 p_m`.

These are transparent constructive **upper** bounds. They are not optimized Keccak circuit counts, constraints-per-hash benchmark results, or lower bounds for every circuit computing H. No such benchmark or optimized graph inventory was found/provided. The exact post-pruning G, I and m_used are combinatorial data of the specification, not unknown cryptographic parameters; obtaining numeric tight counts would require generating/counting that circuit, which was not done here.

Boolean witness generation uses one evaluation of each of the G gates, 2S copy-bit writes, and its output/input handling. Packing the witness writes exactly ceil(w/8) bytes. Materializing the whole circuit trace costs O(m+w) bits plus circuit indexing; computing H alone uses the ordinary 200-byte state, but that is **not** the memory needed to return all w witness bits.

### 5.3 Arbitrary-field evaluation is not a hash call

Evaluate each factor directly and multiply the results; never expand the multivariate polynomial into coefficients. AND and XOR factors each need at most two general field multiplications: compute respectively `t=xy` or `t=x+y-2xy`, then the equality expression `1-t-z+2tz`. NOT/COPY use one, SPLIT uses three, and constants use none. With Fct factors, the final product uses Fct−1 further multiplications. A sufficient explicit count is

`M_P <= 2G_2+G_1+3S+o_v+(G+S+o-1) <= 3G+4S+2o-1`.

At most 12 field additions/subtractions per factor suffice, counting multiplication by 2 or 4 as additions; this is a conservative circuit-arithmetic bound, not machine instruction timing. A straightforward evaluator stores a coordinates plus constant scratch and reads the static factor topology. A topology list takes O((G+S+o) log a) bits with integer wire IDs; procedural generation can avoid storing that list.

The two external decider calls cost at most `2M_P` general F multiplications and O(G+S+o) additional field work each. The first input is Boolean and can instead be checked by H/W recomputation and exact comparison in O(G+S+o) Boolean work. The second point is generally non-Boolean and really requires field evaluation. Neither is accurately priced as “two short hash digests”. No timing for degree-four field arithmetic is asserted.

## 6. Receipt, quotient, and memory accounting

Use the existing quotient sampler width 1024 and `F=K[U,V]/(U^2-5,V^2-U)`. Its canonical four-K-coordinate representation occupies `b_F=64` bytes when K elements occupy 16 bytes. Let n be the actual authenticated component-query ledger size, including transcript absorb/finalize/squeeze calls and every consumed rejected attempt according to the selected compiler. It is not the number of semantic challenge sites. The first accumulator point additionally contains the CM challenge and its graph witness.

### 6.1 Public accumulator and witnesses

The mathematical accumulator has two point/value pairs, each point of width a. Canonically pack the **first Boolean point** and omit its fixed value 1, but leave the second arbitrary-field point/value unpacked:

`B_acc = ceil(a/8)+64(a+1)+B_frame`.

This excludes the original lattice handles, public application statement, proof and any separate repeated serialization of them. One cannot pack the second point as Boolean just because it was interpolated from Boolean points. The first point includes all w CM witness bits, not merely the 1024 raw challenge bits.

A separately materialized ledger of n Boolean graph points costs `ceil(na/8)` bytes under a single packed concatenation (or `n ceil(a/8)` with per-row packing), plus framing. Witness complements alone cost `ceil(nw/8)`; raw x/y can be derived/deduplicated only if the existing skeleton/source-binding contract authenticates that exact reconstruction. A stored vector interpolation curve g has a*n field coefficients, costing `64an` bytes. This is avoidable working storage, not mandatory public data: direct interpolation weights compute g(beta) in O(an) field operations without storing those coefficients. The full original source, its digit expansion, and C_g capacity are separate costs.

A simple arbitrary-field P evaluator materializes `64a` input bytes, plus topology/scratch; streaming/random-access designs can change peak RAM. There is no proved lower bound of `64a` on **resident** memory. There is an exact `64a` second-point wire payload for this chosen canonical codec, regardless of how it is streamed.

### 6.2 Full-capacity quotient: keep the actual source law

For n>=2, in the ideal degree-two ARO theorem retain

`D_f = 2a(n-1)`,

`N_t >= D_f-n+1`, with N_t the actual provisioned power-of-two coefficient capacity,

`D_* = max(D_f,n+N_t-1)`.

The full-grid source has exactly `4N_t` K coordinates, interpreted as **all** N_t coefficients of an F polynomial, as in [QUOTIENT_MLE](AKITA_INSTANCE_ACCUMULATION_QUOTIENT_MLE.md). Its decoded coefficient payload is `64N_t` bytes. Honest padding coefficients may be zero; weak extracted sources are not presumed zero there. A larger physical full grid increases N_t and D_*. The native digit representation, hint/commitment material, source-path authentication and outer profile have additional costs that this decoded payload does not include. There is one additional ordinary outer multilinear claim, not a free quotient opening or a proof of canonical Commit randomness.

The full decoded coefficient count is exact for this representation. Since `N_t>=D_f-n+1`, `64(D_f-n+1)` is a payload lower bound **for a profile provisioning this degree cap**, not a lower bound on every realized quotient's mathematical degree or all possible representations. Taking `N_t=nextpow2(D_f-n+1)` is sufficient only if the backend accepts that grid. It gives `D_*<2D_f`; a larger backend grid need not.

For the **concrete** product P only, composition has the potentially smaller bound `δ(n-1)` from §4. It is algebraically possible to size a concrete quotient from that bound. Doing so changes the family being analyzed: one may not simultaneously import the ideal random-extension theorem with its generic `2a` degree. The figures below deliberately retain the existing ideal-profile `2a` cap so they can be used as honest conditional planning bounds, not as an unauthorized concrete security optimization.

For n=1 retain the direct protocol `g=constant,t=0`, with no newly required quotient source solely to authenticate zero. This exception does not describe a whole-run ledger with many queries.

### 6.3 Host work, not just serialized size

The existing constructive interpolation method takes at most `D_f+1` arbitrary-field graph evaluations to form `f=P_m o g`, followed by interpolation and exact division of f−1 by Z_D for an honest valid ledger. The graph-evaluation portion alone is bounded by `(D_f+1)M_P` general F multiplications for the concrete evaluator. There is additional work to evaluate all a curve coordinates at those points: a straightforward Lagrange algorithm is O(a*n*(D_f+1)) field operations, plus weight computation. Ordinary quadratic interpolation/division gives a finite O(D_f^2+n^2) upper bound for the scalar polynomial stages. Fast polynomial algorithms or structured product composition can improve these *algorithms*; none is assumed measured here.

This simple construction may materialize g, f, quotient and interpolation scratch, with O(an+D_f+N_t) field elements besides graph buffers and source storage. Memory can be scheduled/streamed differently. The quotient's full-capacity source payload and the work to authenticate it do not disappear when g's storage is avoided. The proof/extraction analysis additionally pays the existing D_* dependent private continuation/interpolation budget; its ideal oracle calls are not a measured Keccak host runtime.

### 6.4 Concrete planning illustration, explicitly not lower bounds

For the prior conditional transcript-width example `m=1610` (1024-bit state, 64-byte absorb blocks and 64-bit counters; literal CM must fit, or a separately approved compact-CM compiler must justify this width), the adapter absorbs 232 bytes and uses exactly p_m=2 permutations. Therefore

If the separately researched compact-CM extension is admitted, its proposed `10 || finalized_state || fixed_CM_site || fixed_index` input gives this same m without placing all handles in x. The actual public message `M=Enc(profile,C_g,C_t,shapes,nonce)` still exists and must be reconstructed externally. With 64-byte blocks, root reconstruction additionally costs `floor(|M|/64)+1` component calls for absorb/finalize; authenticating its CM squeeze is the first graph check already charged here (or one further H call if checked by direct recomputation). Those calls and full-message bytes are not included in the displayed accumulator-only payload. This is conditional pricing of that separate ideal compiler extension, not reuse of the literal-CM theorem or approval of its concrete realization.

| Quantity | Constructive upper envelope |
|---|---:|
| G | 384172 |
| w | 1154564 bits |
| a | 1157198 coordinates |
| packed W payload | 144321 bytes |
| materialized one F point | 74060672 bytes |
| public packed-first/unpacked-second accumulator, excluding framing | 74205386 bytes |

The exact pruned circuit is smaller or equal. **These upper bounds do not prove that the actual accumulator needs 74 MB.** They show that a generic graph construction is not evidence of a small accumulator; exact circuit/layout counts or a better construction are needed for a tight resource claim. In contrast the earlier **ideal w=0 illustration**, at this same m, gives only 168970 public bytes. It does not price the concrete family.

One conservative profile can provision capacity using this upper envelope rather than the actual a. For n=2, the envelope gives `D_f=2314396`, `N_t=4194304`, and decoded quotient payload 268435456 bytes. For an **illustrative** n=3872 it gives `D_f=8959026916`, `N_t=17179869184`, and payload 1099511627776 bytes (1 TiB), before native source expansion. These are sufficient-capacity planning values for that conservative envelope, **not lower bounds**, measured usage, or a certificate that n=3872 is the actual whole-run ledger. In particular previous squeeze-only or 512-bit-output counts do not transfer unchanged to this 1024-bit-output component. Absorb/root and retry counts must be regenerated for the final profile.

The missing data for an actual whole-run cost conclusion are precise: selected m/CM grammar; final ledger n and query cutoff; exact pruned gate/port count if tighter pricing is needed; admitted C_g source/advice map; actual C_t full grid/digit geometry; complete guest trace/profile and backend working-set schedule; and measured implementation costs if a timing claim is desired. No absent number is replaced by a fabricated benchmark. None of these changes the user's whole-run target.

## 7. Exact security-instantiation boundary

### 7.1 Correct graph is not a uniformly random extension

For `a=m+1024+w`, the CCGOS ideal Vhat is uniform in the affine space of individual-degree-<=2 extensions of the Boolean predicate `y=ro(x),z=B(x)`. The concrete P above is one publicly specified factored polynomial. Its Boolean correctness, ro-free W, and efficient evaluation establish interface compatibility, **not distributional equivalence or a security reduction**.

In fact full-oracle indistinguishability is the wrong additional claim for this product family. Choose a gate, assign its Boolean ports inconsistently so that its factor is zero, and choose a coordinate j not in that gate (available in this circuit) to be a non-Boolean value c, say 2 in F. Assign the other coordinates arbitrarily as Boolean values. The concrete P evaluates to zero. For any fixed Boolean graph, adding

`α X_j(X_j-1)`

to a degree-two extension preserves every Boolean value and changes its value at the chosen point by `α c(c-1)`. Since this is nonzero for c=2 in the actual odd-characteristic field, a uniformly sampled ideal extension has a uniform value there. A single such test distinguishes the concrete product from the ideal extension with probability gap `1-1/|F|`. This uses no collision attack and holds even before discussing H's own standard-model distinguishability.

This observation **does not exhibit a forged receipt**, refute the conditional ideal ARO theorem, or rule out useful efficient arithmetizations. It rules out describing the missing decision as a proved, general-purpose computational emulation/indistinguishability theorem for this explicit polynomial. Game-specific security may survive structures that make the oracles distinguishable; that is a separate claim requiring evidence or an explicitly accepted heuristic.

### 7.2 What remains to be chosen

There are two honest states of the result:

1. **Keep the existing theorem in the ideal model.** Use sampled `(ro_i,wo_i,Vhat_i)` and the independently modeled outer H_o, with the approved conditional local/paired/continuation/transport/source-first lemmas and all of their source/profile/query/runtime premises. The circuit in this note then gives a candidate implementation and cost study, not an implementation of the sampled ideal distribution.
2. **Explicitly admit a concrete, protocol-specific instantiation claim for the exact versioned tuple.** Name this H_m, exact W ordering, exact factored P_m, F, transcript/CM compiler, source/guest/profile, admissible adversary/runtime/query limits and the concrete outer-oracle instantiation if one is also desired. Assert and account for security preservation of the actual receipt/native-source/zero-finding composition under these replacements, with an explicit concrete loss/assumption. This is stronger/different than collision resistance or ordinary ROM Fiat–Shamir security. It must not be stated as universal oracle indistinguishability, which §7.1 disproves for this family.

The user subsequently selected **Allow a heuristic prototype**. This authorizes experimental research under a clearly named protocol-specific concrete-ARO premise; it does not prove option 2's security-preservation claim or supply its quantitative bound. No inspected primary source proves that claim for this `(H,W,P)` family. CCGOS proposes a heuristic bridge and proves ideal-model results; FIPS 202 specifies the permutation and standard modes, not graph-polynomial security; the pinned Spongefish source is not a receipt-security theorem.

The exact game-specific premise remains **unproved**, and must expose all implications for native language soundness or any restricted CH-I statement. It is not ordinary hash collision resistance, Module-SIS, universal oracle indistinguishability or a grant of arbitrary CH-O knowledge. The authorization does not choose a concrete loss or certify the resulting software.

The research result remains conditional: a precise existing-primitive graph family with unique Boolean witness, degree two per variable and finite resource expressions. [D1](AKITA_INSTANCE_ACCUMULATION_PROTOTYPE_MODEL.md) resolves the model choice by directly computing the fixed public inner oracle and explicitly using a restricted native-game heuristic. Its scoped model review does not prove that heuristic or admit the remaining outer design/implementation/profile/performance. The graph distinguisher is neither a receipt forgery nor impossibility of lattice accumulation.

## 8. Scope of work and handoff

This is a mathematical/source investigation only. No Rust implementation, branch, Git command, build, test, formatter, benchmark or proof execution was performed by this investigator. Numeric tables are arithmetic evaluations of the displayed construction bounds and published round constants, not execution measurements. The parent's small Boolean checks are explicitly attributed in §4.3.

Independent review confirmed the gate/SPLIT identities, unique-trace induction, port-count identity, pinned overwrite-duplex semantics, upper-bound versus exact-cost distinctions, full-capacity quotient semantics and explicit distinguisher. Integration must not replace the ideal theorem's sampled Vhat by this P without a separately justified and approved concrete security premise.
