# E5: proving the compilation

Status: planned, conditional on gate C. Written 2026-11-04.

## Question

How much does it cost to run the Cedar-to-Noir compiler inside RISC Zero so
that a verifier can accept a verification key by proof instead of by allowlist?

## Hypotheses

H5.

## Setup

- The compiler from E4, compiled as a guest.
- Inputs: policy set text, schema text. Output journal: policy hash, schema hash, compiler hash, verification key hash, per `docs/protocol.md`.

## Procedure

Strict mode.

1. Build the compiler for the guest. If the key generation step cannot run in the guest, prove only the Term IR to Noir source translation and hash the Noir source. Record this reduction as a decision and in the paper.
2. Run the executor on each compiled corpus policy set. Record cycles.
3. Prove the largest policy set 3 times on the laptop and, if available, the server. Record time and memory.
4. Verify the compilation proof 20 times. Record the median.
5. Run the full verification procedure with a compilation proof attached and the verification key not on the allowlist. Confirm acceptance.

## Measurements

Cycles. Proving time. Memory. Proof size. Verification time.

## Success and failure

- H5 supported if the largest policy set proves in under 1 hour on a server CPU.

## Results

None yet.
