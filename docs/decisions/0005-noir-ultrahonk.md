# 0005: Noir and UltraHonk as the circuit toolchain

Status: accepted. Date: 2026-11-04.
Proposed by: human. "A cedar evaluator in Noir."
Accepted by: human, in conversation.

## Context

The circuit backend needs a language and a proof system.

## Options considered

- **Circom with Groth16.** Mature, small proofs, trusted setup per circuit, slow iteration.
- **halo2.** Lower level, longer development time.
- **A raw STARK library** such as plonky3 or stwo. No DSL, longest development.
- **Noir with Barretenberg UltraHonk.** A DSL with a standard library, signature and hash libraries, recursion, a zero-knowledge mode, and measured browser and mobile proving.

## Decision

Noir with UltraHonk. Plain and zero-knowledge modes both measured.

## Reasoning

Client-side proving is the point of this backend, and Noir has the measured
client-side story. Its libraries cover the input provenance work in E3.

## Consequences

- `noirup` and `bbup` must be installed. Neither is on the laptop yet.
- Phone proving needs a mobile wrapper. Reported memory limits on iPhone browsers are a known risk.
- On-chain verification cost is high for UltraHonk compared to Groth16. Out of scope here but noted for the paper.

## Sources

S14, S15, S16, S17.
