# 0023: A paper first, practical if possible, and the order of work

Status: accepted. Date: 2026-11-04.
Proposed by: assistant, for the order. Human, for the goal.
Accepted by: human, who said "go ahead and do 1 through 6".

## Decision

The goal is a paper. It is better still if the result is practical and deployable. The work runs in this order:

1. Fair comparison: the zkVM guest reads the same two records as the circuits.
2. Test the Lean chain idea (E10). The human asked that this start while the zkVM proof runs.
3. Retarget the compiler to GordieHowe, depending on 2.
4. Add request context to the compiler.
5. Signed credentials.
6. A preprint once 1, 3 and 5 are done.

Skipped for now: a Circom backend, phone benchmarks, OOPSLA corpus additions, and the compilation proof. Circom's analyzers stay as a possible soundness experiment, not a backend.

## Reasoning

Priority here is measured in weeks, so measurements matter more than claims. The Lean chain would be the most original result if it is feasible, so its risk is tested early.

## Consequences

Decision 0009's compilation proof waits. GordieHowe's per-program certificates may reduce its importance. A new record decides.

The human's concern on "practical" is noted against the fact that GordieHowe and Noir both warn against production use.

## Update 2026-11-05

The human asked for all six, so step 3 did not wait for the Lean verdict: GordieHowe's speed justifies it as a target on its own. If E10 ends in no-go, GordieHowe stays a target with per-program certificates and no link to Cedar's semantics.
