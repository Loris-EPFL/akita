# Continued boundary bridge: bounded-dense source refinement

Companion to `AKITA_RELATION_ACCUMULATION_CONTINUED_BOUNDARY_BRIDGE.md`, adding the exact producer-class evidence requested during the investigation. Same read-only/no-check scope.

## Negative advice counterexample does not depend on unrestricted extraction

`akita/crates/akita-config/src/proof_optimized/fp128.rs:63-95` expressly defines `DenseBounded::LOG_COMMIT_BOUND=65` as the **centered signed** interval

```
[-2^64, 2^64-1],
```

not the unsigned interval `[0,2^64-1]`. `jolt/crates/jolt-akita/src/configs.rs:245-255` delegates `JoltDenseBounded`, the preset for advice and committed-program objects, to this class.

Consequently the report's exact original advice word `-1` is inside the admitted bounded-dense producer class even before any weak/algebraic extraction relaxation. The honest Jolt `commit_advice` API is narrower: it accepts bytes and embeds canonical u64 words (`jolt/crates/jolt-prover/src/akita/witness.rs:345-388`). That API policy is not a verifier proof of the unsigned source predicate. Merely proving membership in the preset's centered signed range would still admit `-1`.

For an unused original advice word, all field identities, fixed-source PC/Main masks, first RAM initialization, RAM stuttering, exact endpoint aliases and final IO remain true, as detailed in primary report §8.1. The decoded value cannot equal the embedding of any u64 because q-1 exceeds 2^64-1. A balanced RAM representation with plane0=-1 and remaining planes zero is itself legal. Neither signed dense source bounds nor balanced original RAM plane bounds close the fixed-original-source byte-memory contract.

For an executed LD, the report's loaded value `-1` requires register delta `-1`, already in the ordinary signed-difference scale; it does not need a large carry or a forbidden positive value 2^64. LD has no lookup and enforces only load equality/address relations (`jolt/crates/jolt-lookup-tables/src/instructions/riscv/ld.rs:1-16`; `jolt/crates/jolt-claims/src/protocols/composed/r1cs/rv64.rs:231-252`). The canonical identity-RAF guard does not range this loaded data. The required repair, if actual fixed-source RV64 memory is selected, is **unsigned decoded-word membership or equality to a certified canonical original reference**, not a claim that the source preset is already unsigned.

This counterexample is not a commitment collision or an assertion that the canonical byte constructor honestly emits -1. It chooses distinct valid original commitments to permitted original signed coefficients, fixes them before challenges, and refutes the proposed relation implication on that fixed assignment. It is not a counterexample to the selected K-valued algebraic relation.

The existing concrete full-public/no-advice native receipt profile rejects auxiliary groups (`jolt/crates/jolt-verifier/src/receipt/native.rs:268-283`), so this advice counterexample does not assert a defect in that restricted profile. It identifies the adequacy obligation when claiming the generic continued advice path or all-original-source byte semantics.

For the independent terminal-policy example in primary §8.4, choose a final forward JAL with a **nonzero** destination register and correct ordinary word link value PC+4, avoiding any x0 writeback convention. Its target is correctly range-checked by JAL's lookup (`jolt/crates/jolt-lookup-tables/src/instructions/riscv/jal.rs:6-35`), but final NextIsNoop=1 still zeros ShouldJump and disables the next-PC target equality (`jolt-claims/src/protocols/composed/r1cs/rv64.rs:316-342,392-394`). It remains a field-relation vs actual emulator-stop counterexample, not a lookup-address alias attack.
