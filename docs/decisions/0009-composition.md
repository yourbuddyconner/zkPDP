# 0009: Compose at the verifier and by compilation proof, not proof-in-proof

Status: accepted. Date: 2026-11-04.
Proposed by: assistant, after the human raised that the two systems might be composable.
Accepted by: human, batch.

## Context

The human expected the zkVM and circuit backends to compose. The question was how.

## Options considered

- **Verify an UltraHonk proof inside the RISC Zero guest.** Millions of cycles. Buys only the ability to nest.
- **Verify a RISC Zero Groth16 or STARK proof inside a Noir circuit.** Millions of gates. Same.
- **Compose at the verifier.** Both proofs expose a shared commitment. The verifier checks both and checks equality. No proof sees the other.
- **Compilation proof.** The zkVM proves that the compiler mapped policy hash H to verification key K. The circuit proves evaluation. The verifier pins the compiler image and allowlists policy hashes.

## Decision

The last two. Proof-in-proof is not planned.

## Reasoning

The verifier-side composition is nearly free. The compilation proof recovers
policy as data for the circuit backend, which was its main weakness against
the zkVM backend. Together they give the circuit path most of the general
properties the human preferred in the zkVM.

## Consequences

- E5 exists.
- `docs/protocol.md` has a `compilation_proof` field.
- Recursive proofs inside Noir remain available for credential composition and are not excluded by this decision.

## Sources

S09 for RISC Zero composition facilities.
