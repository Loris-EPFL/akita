# Derived native sampler budgets and certificate-size policy

## Decision

**No 4 MiB protocol requirement and no mandatory final Akita receipt.** The native certificate may be linear in the segment count. Its standalone transport accepts explicit finite caller-approved budgets; any wrapper is optional and has its own limits and assumptions.

Finite limits still matter for an untrusted verifier: checked arithmetic, bounded decoding/allocation and explicit abort semantics are required. A limit is a supported configuration, not a cryptographic size theorem. Current framing uses u32 component lengths; widening that representation requires a versioned format change, not deleting overflow checks.

The [native certificate note](AKITA_INSTANCE_ACCUMULATION_NATIVE_CERTIFICATE.md) records the observed 405,409-byte two-frame archive and current profile. This note derives sampler envelopes rather than treating an observed size or a former VM limit as an admission theorem.

## 1. Exact primitive laws in the ideal-word model

Let

$$
q=340282366920938463463374607427473266697,
\qquad r_K=\frac{2^{128}-q}{2^{128}}<2^{-96}.
$$

The selected bounded K sampler takes at most two independent 128-bit words, accepts the first canonical value below q, and rejects the run if both fail. Its primitive failure probability and maximum consumption are

$$
\delta_K=r_K^2<2^{-192},\qquad B_K=256\text{ bits}.
$$

An E2 coordinate uses two such K limbs: at most 512 bits and failure probability at most $2\delta_K$. Conditioned on **primitive success**, the K value, or E2 pair, is exactly uniform. Conditioning on whole-proof acceptance is different and is not asserted here.

For the forty-position shell, at position j the remaining range has size $128-j$ and the candidate width is seven bits. There are at most 256 attempts. Position zero cannot reject. Under independent ideal words,

$$
\delta_{\mathrm{shell}}
\le\sum_{j=1}^{39}\left(\frac j{128}\right)^{256}.
$$

The exact success probability is the product of the forty position-success probabilities; forty independent sign bits are read afterward. The fixed 24 magnitude-one and 16 magnitude-two positions give the existing uniform shell law on primitive success. No response-dependent rejection is introduced.

These laws follow the actual loops in `akita-transcript/src/native.rs::inner_scalar` and `inner_shell`. They do not prove pseudorandomness or the full conditional-answer theorem for concrete H1610, and exclude separate setup-XOF work.

## 2. Per-stream envelope

The conservative implemented shell envelope is

$$
B_{\mathrm{shell}}=40\cdot256\cdot7+40=71720\text{ bits},
\qquad\left\lceil B_{\mathrm{shell}}/1024\right\rceil=71.
$$

Thus the selected 71-block cap does not terminate the unchanged shell retry law early. Because the first position consumes exactly one word, the tighter attainable bound is

$$
7+39\cdot256\cdot7+40=69935\text{ bits},
$$

which fits 69 blocks. The versioned profile deliberately retains the conservative 71; neither number is an archive-size bound. Global transcript/component/read limits remain separate and may still reject a run.

## 3. Whole native read envelope

For the inspected compact geometry with RAM zero adjunct absent, the native manifest has

$$
C_E(T)=513T+289,\qquad W(T)=120T+72.
$$

Let $C_K$ be the actual sum of **fresh** frontend K coordinates over the installed frames. Carries and derived powers introduce no new primitive draws. A sufficient read-budget condition is

$$
256C_K+512C_E(T)+71720W(T)\le2^{28}.
$$

The conservative native-only subtotal and remaining frontend allowance are:

| T | E2 coordinates | Shell coordinates | Native-only maximum read bits | Maximum additional K coordinates under this inequality |
|---|---:|---:|---:|---:|
| 2 | 1,315 | 312 | 23,049,920 | 958,537 |
| 16 | 8,497 | 1,992 | 147,216,704 | 473,510 |

Parent exact integer arithmetic checked these values. They are **upper envelopes**, not measured read counts, proof sizes or an assertion that every T16 configuration fits all other limits. The frontend audit exposes `fresh_k_coordinates`; its actual inventory, rather than a count of carry references or powers, must satisfy the displayed condition.

For a fixed admitted roster, primitive sampler failure is bounded by

$$
\delta_{\mathrm{primitive}}
\le(C_K+2C_E(T))r_K^2
+W(T)\sum_{j=1}^{39}\left(\frac j{128}\right)^{256}.
$$

Exact rational checks give shell terms below $2^{-430}$ for T2 and below $2^{-427}$ for T16. If the table's remaining-K condition holds, the K-limb term is below $2^{-172}$ and $2^{-173}$ respectively. These are ideal primitive-abort bounds, **not** complete honest-abort bounds or composed knowledge-soundness levels.

The executed T2 report has $C_K=1089$, hence 3,719 primitive K limbs including E2. Substitution gives a worst-read envelope of **23,328,704 bits**, below $2^{28}$; exact rational arithmetic gives $\delta_{\mathrm{primitive}}<2^{-180}$ in the ideal-word model. This is not measured bit consumption or a concrete H1610 soundness claim.

## 4. What this does not derive

The sampler inequality does not price or certify:

- all retained installation/context/seal/transcript bytes;
- component calls, snapshot work, source/matrix operations or peak memory;
- every allowed program/IO/segment configuration;
- conditional full-answer programming and original-execution failure semantics;
- extraction replays, adversarial auxiliary hash work or runtime-priced MSIS advantage;
- an optional wrapper's witness/advice, memory, proof size or security.

In particular, caps do not bound adversarial public-function computations: the adversary can evaluate H1610 outside the installed compiler. A larger receipt can address a transport limit, but not an absent source-extraction or Fiat–Shamir theorem. Whole-profile numerical admission remains false until those separate obligations are discharged.

Independent review (`ReviewNativeSamplerNote`, confidence 0.98) found no correction to the sampler laws, finite envelopes, coordinate counts or framing distinction. Approval excludes complete global-cutoff completeness, whole-proof conditional uniformity, ROM extraction, numerical/PQ soundness and performance parity.

## References

- [Akita](SOURCE_PROVENANCE.md#akita), Lemma 10.2 and §3.8: primitive conditional tapes, complete-answer requirements and weighted extraction; equations (225)–(228): actual tree and whole-extractor resources.
- [Native source-tree theorem](AKITA_INSTANCE_ACCUMULATION_NATIVE_SOURCE_TREE.md): algebraic source recovery and remaining compiler boundary.
- [Composed resource artifact](AKITA_INSTANCE_ACCUMULATION_COMPOSED_RESOURCE_ARTIFACT.md): actual fresh-coordinate audit, matrix views and absent numerical admission.
- `jolt-verifier/src/segment_native_resources.rs`: selected versioned limits.
- `jolt-verifier/src/receipt/{archive,frame_codec,native}.rs`: bounded canonical transport and direct decider; no mandatory wrapper or VM-derived total archive ceiling.
