# 0004: RISC Zero as the first zkVM

Status: accepted. Date: 2026-11-04.
Proposed by: assistant.
Accepted by: human, batch.

## Context

Several zkVMs run Rust: RISC Zero, SP1, OpenVM, Jolt, and WASM-based ones.
Vendor benchmarks disagree with each other about cost and speed.

## Options considered

- **RISC Zero.** Prezta's choice, so results are comparable. Mature recursion and a Groth16 wrap. Precompiles for SHA-256, Ed25519, P-256, secp256k1, RSA. `rzup` is already installed on the laptop. Open advisory that the STARK is not provably zero-knowledge.
- **SP1.** Also runs std Rust. Vendor claims it is faster on CPU. Different precompile set. No Prezta comparison.
- **OpenVM, Jolt.** Younger. Less tooling.
- **A WASM zkVM.** Cedar has a WASM build. Fewer precompiles and less mature.

## Decision

RISC Zero first. The guest is kept plain so it can port to SP1 for a second data
point if time allows. No claim about which zkVM is cheaper.

## Reasoning

Comparability with Prezta and the local toolchain outweigh speed claims we
cannot verify. The hypotheses file refuses to register a RISC Zero versus SP1
claim for this reason.

## Consequences

- E1 is on RISC Zero.
- The privacy analysis must quote the zero-knowledge advisory for every RISC Zero format.
- The precompiles are not constant time, which matters for the private mode.

## Sources

S01, S08, S09, S10, S20, S21.
