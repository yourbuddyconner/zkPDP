# 0008: Order of experiments: zkVM, hand circuit, compiler

Status: accepted. Date: 2026-11-04.
Proposed by: assistant.
Accepted by: human, batch.

## Context

Three pieces of engineering with very different cost. The order decides what
is learned before the expensive piece is started.

## Options considered

- **Compiler first.** Highest payoff, highest risk, nothing to compare against for weeks.
- **Hand circuit first.** Learns the circuit shape early but has no baseline numbers.
- **zkVM first, then hand circuit, then compiler.** The zkVM run is the smallest piece and produces the corpus, envelope and baseline that everything else uses. The hand circuit then answers whether the compiler is worth writing.

## Decision

zkVM first. Hand circuit second. Compiler third, gated on the hand circuit's
gate count.

## Reasoning

Each step settles the question the next depends on, and each gate can shrink
the plan with a result in hand rather than an abandoned effort.

## Consequences

- Gates A, B and C in `docs/plan.md`.
- The circuit backend may end as a negative result. That is acceptable and reported.

## Sources

None. Process decision.
