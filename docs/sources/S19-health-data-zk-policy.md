# S19: Zero-knowledge policy enforcement for health data sharing

Systems, 2026. DOI 10.3390/systems14040385.
Accessed 2026-11-04. Trust: unverified. Phase 0 task to read.

## Expected relevance

A requester proves it satisfies an access policy without revealing identity or
policy details. An on-chain verifier authorizes. Related work for the private
mode.

## Update 2026-11-04: paper and repository read

Albalwy, F. "Zero-Knowledge-Based Policy Enforcement for Privacy-Preserving Cross-Institutional Health Data Sharing on Blockchain." Systems 2026, 14, 385. 34 pages, CC BY.
PDF supplied by the human. Not in this repository. Trust: primary. Abstract, sections 3.1 to 3.3, 5.5 and the evaluation headings read. Repository https://github.com/FbalwyTU/ZK-EHR read at `main` on this date: `circuits/access.circom`, `circuits/input.json`, `public.json`, `contracts/Verifier.sol`, `test/verifier.test.js`, and a search of `lib/`. Nothing was compiled or run.

### What the paper says

- The design describes a policy engine, a circuit generator, a Poseidon hash-equality check against an on-chain policy hash, and attributes bound to a verifiable credential. Theorem 2's soundness argument relies on a signature check inside the circuit.
- The paper discloses that the measured prototype is much smaller: three private scalars checked against the constants 1, 2 and 1, 5 R1CS constraints, one public output `isAccessGranted`. Poseidon hashing, credential binding, expiry and nullifiers are described as envisioned extensions. Revocation and replay protection are stated limitations.
- The 216,631 gas and about 390 ms are measured on that 5-constraint circuit.

### What the code shows, from reading it

- `access.circom` matches the paper's prototype. No signature, commitment or hash. Any prover can choose the inputs.
- The circuit outputs `isAccessGranted` and never asserts it equals 1. A prover with non-matching inputs gets a valid proof whose public output is 0.
- `verifyProofAndEmit` in `Verifier.sol` emits `AccessGranted` when the proof verifies. It does not read `_pubSignals[0]`. Algorithm 1 in the paper has the same shape. A search of `lib/verifierAccess.js` and `lib/secureRecordWorkflow.js` found no check of the output either.
- Consequence, an inference from reading: a proof of a failed policy check would be granted access. No exploit was run.
- The earlier concern about the non-boolean equality template is real but secondary.
- The repository holds one fixed circuit and no generator, so the policy-to-circuit step exists in the design only.

### What we take

- Do not cite ZK-EHR as evidence that credential-bound ZK policy enforcement works at these costs, or as a policy compiler precedent.
- The theorems describe the proposed design, not the measured prototype. The paper says the prototype is a prototype, so this is a gap in the claims, not concealment.
- A design lesson for zk-pdp: the verifier must check the proven decision, not only the proof. `docs/protocol.md` and the envelope verifier return the journal's decision and refuse a mismatch, and a deny must never be read as a permit because its proof verifies.

The human decided on 2026-11-04 not to contact the author. Any citation of the flaw states what was and was not run.
