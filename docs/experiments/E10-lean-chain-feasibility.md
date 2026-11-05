# E10: can the chain from policy to proof be machine-checked?

Status: in progress. Started 2026-11-04. Written before any Lean was built.

## Question

Cedar's model and SymCert are in Lean, and GordieHowe's proof system is in Lean. Can they connect, so that a verifier's accepted proof implies, by a machine-checked argument, that Cedar's authorizer gives this decision on the committed facts?

## The chain, link by link

| Link | Statement | State on 2026-11-04 |
|---|---|---|
| A | Cedar's authorizer agrees with `evals` of the symbolic authorizer | Proved in Lean (SymCert Theorem 4). About term evaluation, not SMT. Paper evaluates Cedar 4.4 |
| B | A witness satisfying the GordieHowe source program makes the decision bit equal `evals` of the term under the interpretation read from the committed records | New. Ours to state and prove, per policy |
| C | Record openings mean the facts are the committed ones | New. Needs collision resistance of the hash, stated as a game the way GordieHowe states its assumptions |
| D | The realized program implies the source program | GordieHowe's Lean kernel check, per program. Tooling exists. Not yet run for TinyTodo |
| E | A valid realized program with a complete and round-by-round knowledge sound source gives a sound zero-knowledge argument | Proved in Lean by GordieHowe (Theorem 6.7) |
| F | Round-by-round knowledge soundness of our source program | New. GordieHowe proved it end to end only for ML-DSA-44, in about 47,000 lines of application Lean. By file search, a Keccak gadget has a Lean spec and SHA-256 does not |

## Known obstacles

- **Versions.** GordieHowe pins Lean v4.34.0 and one Mathlib. `cedar-spec` head pins Lean v4.35.0 and another. One theorem importing both needs one side moved.
- **Rust.** Link A covers the Lean compiler. The project reads terms from the Rust crate. Link B should use the Lean compiler's own terms.
- **Hash.** SHA-256 has no Lean gadget proof that we found. A Keccak-based commitment would shorten link F, and would change the commitment for every backend.
- **Remaining trust even if it all works.** The C++ prover and verifier implement the Lean model, the export records the program used, GKR circuits meet three stated assumptions unless the policy logic avoids GKR, and hash specifications match the standards.
- **Machine.** Only 28 GB of disk is free. GordieHowe's build limits itself by memory, at 12 GiB per module by default. Lean builds must wait for the zkVM proof to keep its timing clean.

## Procedure

Strict mode.

1. Install Lean. Download Mathlib's cache, stopping if free disk falls under 10 GB.
2. Export TinyTodo's plan and program from the C++ compiler, and check them with Lean's kernel. This tests link D.
3. State link B for one TinyTodo policy in Lean, and try to prove it. Time-box it.
4. Estimate link F from what step 3 teaches, and decide between SHA-256 and Keccak.

## Success and failure

- Go, if link D checks and link B is proved for one policy within the time box. Then the compiler targets GordieHowe with this chain as its aim.
- Partial, if link D checks but link B does not close. The paper claims per-program certificates, with a Lean model of the front end, and names the gap.
- No-go, if link D cannot run on this machine. Noir stays the compiler target and GordieHowe stays a measured comparison, and the paper says what was tried.

## Results

None yet.
