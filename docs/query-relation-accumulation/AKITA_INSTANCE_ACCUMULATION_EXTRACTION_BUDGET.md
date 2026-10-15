# Receipt extraction: why the inner Q32 certificate does not compose unchanged

## Result and scope

The [product-oracle extraction theorem](AKITA_INSTANCE_ACCUMULATION_PRODUCT_EXTRACTION.md) separates the original outer H_o query count from work and inner-oracle calls made across extraction. This separation is necessary. **Its explicitly enumerated complete-frontend-tree route cannot be certified to keep the derived native adversary below the old Q32 bound for arbitrary allowed provers.**

This is a bound on the specified security reduction. It is not a lower bound on honest proving time, an attack on the receipt, a proof that every extractor has this cost, or a performance-parity impossibility result. A sharper extraction/composition theorem may change the conclusion.

## 1. A source-grounded tree-size lower bound

The [frontend manifest](AKITA_INSTANCE_ACCUMULATION_FRONTEND_MANIFEST.md), §2/§3, gives the ordinary stage-5 batch:

\[
 R_5=n_{\rm trace}+128,\qquad D_5=v+2,\qquad
 k_5=D_5+1=v+3,
 \quad v=128/w\ge1.
\]

These are **sequential scalar sumcheck rounds**, not coordinates of one flat native unit-subtraction challenge. The explicitly configured interpolation tree has k_5 children per round. The modern Akita path has a 4096-cycle floor, hence n_trace>=12. Therefore that portion of a complete accepting tree alone has

\[
 K_5=(v+3)^{n_{\rm trace}+128}
       \ge4^{140}=2^{280}
\tag{1}
\]

distinct scalar-challenge paths. Other frontend stages and the PCS continuation only increase the complete-tree work. Replacing sequential rounds by a flat `1+sum(k-1)` branch count would be a different, presently unproved extraction algorithm.

The declared verifier-polynomial degree, not the apparent degree of one honest prover's messages, determines this tree arity. A special witness may simplify a polynomial, but that does not certify a smaller extractor for arbitrary witnesses and malicious proofs.

## 2. Why forwarding inner queries can exhaust the old budget

Consider any allowed prover that, immediately after its **first 140 stage-5 challenges**, makes one additional off-protocol query to ro_i and ignores its answer. For the current flat-input model, canonically encode a fixed tag, length and exactly that 140-coordinate E prefix, then pad injectively to m bits. The coordinate payload occupies 140*32=4480 bytes, fitting the certified m>=655360-bit (81920-byte) flat domain with framing room. A longer outer trace has more stage-5 rounds; those later coordinates are not encoded in this query.

This changes neither the proof nor its acceptance and adds one inner query to each original execution. Every complete stage-5 tree has at least 4^140 distinct **depth-140 prefixes**; their canonical query inputs are distinct. An extractor that forwards those calls to the actual unchanged Theta_i therefore makes at least 2^280 distinct inner queries whenever it constructs the configured tree. Later full paths may share a depth-140 prefix; they are not counted again. Caching repeated queries cannot remove the distinct prefix calls.

This example is about an allowed black-box prover, not a claim that the honest implementation issues these queries. A security bound must cover the allowed prover class. For a different small-input chaining model, this exact injective tuple encoding no longer applies; a corresponding bound must be re-established rather than copied.

The original Q32 policy bounds the derived native adversary only if its **actual total** relevant inner calls satisfy that policy. It does not forbid the original receipt prover from adding a single ignored query. Thus it cannot by itself rule out this extraction amplification.

## 3. Effect on the available numerical bound

The selected inner shell has

\[
 |\mathcal C|=\binom{128}{40}\binom{40}{24}2^{40},
 \qquad\log_2|\mathcal C|\simeq186.844275296.
\]

The available standalone native bound contains

\[
 (Q_i+1)\left(c_E/|E|+c_{\mathcal C}/|\mathcal C|\right).
\]

Using a bound Q_i>=2^280 in that formula makes even Q_i/|C| exceed one: its base-two logarithm is approximately 93.155724704. The reported upper bound is then trivial; this says nothing about the true attack probability. The old `<2^-137` statistical figure cannot be quoted for this derived producer.

A cutoff enforcing at most 2^32 such inner calls stops this enumerating algorithm before its required tree exists. The resulting cutoff/failure event must be charged. It is not legitimate to call it negligible merely because the original proof was valid or the original prover made few queries. The [product theorem](AKITA_INSTANCE_ACCUMULATION_PRODUCT_EXTRACTION.md) already requires this explicit cutoff accounting.

Separately, nonlinear ARO-emulator work and the complete nested extraction runtime remain in every applicable Module-SIS advantage. Neither expected polynomial time nor a per-matrix attack table supplies a concrete end-to-end bound.

## 4. What could resolve it

A complete design needs a proved route that addresses this issue, for example:

- A sharper complete-frontend extractor or composed security reduction that avoids enumerating these sequential-round trees while preserving the original statement and source semantics.
- A rigorously proved query-locality/resampling argument that does not charge all counterfactual inner queries to the derived native adversary. It must preserve the original externally decided oracle/accumulator and deal with all ARO observations; resetting a lazy emulator with the prover is invalid.
- A separately justified security policy and parameters large enough for the actual composed bound, with all lattice/challenge/closure/performance effects checked. Merely increasing E does not enlarge the existing short-shell challenge space.

These are research obligations, not approved replacements. The current gate cannot be closed by excluding inconvenient malicious-prover queries, assuming that only the honest archive's calls matter, or adopting CH-I/CH-O after the user declined that route.

**Later conditional resolution of the query-locality step:** [PRIVATE_ARO_CONTINUATION](AKITA_INSTANCE_ACCUMULATION_PRIVATE_ARO_CONTINUATION.md) now has independent approval. It retains the actual original decision/transcript, then runs one private continuation of the repaired input-local paired emulator. Its two hybrid errors price all virtual extended queries, and its full CPU remains in ordinary-SIS time. Only the original run and actual final predicates make external native-oracle calls. This does not invalidate the forwarding-route lower bound above or automatically certify Q32; the exact original/predicate/chain counts and selected outer resources still require admission.

## 5. Executed evidence and references

Integer arithmetic computed 4^140=2^280, tuple width4480 bytes, the exact shell cardinality comparison and the logarithmic ratio above. No proof-system execution or benchmark was performed.

- [Frontend manifest](AKITA_INSTANCE_ACCUMULATION_FRONTEND_MANIFEST.md), stages 5 and common interpolation rule; [modern source map](AKITA_INSTANCE_ACCUMULATION_RECEIPT_CODE_MAP.md), fixed-geometry/4096-cycle floor.
- [Product extraction](AKITA_INSTANCE_ACCUMULATION_PRODUCT_EXTRACTION.md), original-versus-derived budgets, global emulation and cutoff placement.
- [Query-cost certificate](AKITA_INSTANCE_ACCUMULATION_ORACLE_QUERY_COST.md), canonical flat input lower bound and explicit distinction between original queries and extractor/AS production work.
- [Akita paper](SOURCE_PROVENANCE.md#akita), Lemma 3.22 and Eqs. (225)–(229): configured mixed trees and runtime-sensitive reduction, not a tight composed Q32 guarantee.
