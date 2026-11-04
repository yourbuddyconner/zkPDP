# 0013: Equal effort rule for the comparison

Status: accepted. Date: 2026-11-04.
Proposed by: assistant.
Accepted by: human, batch.

## Context

A two-backend comparison can be biased by giving one backend more
optimization time. The human has said they like the zkVM's properties better,
which is a known pull.

## Decision

Hours per backend are logged daily in the notebook. Optimization work is
time-boxed and the same box is offered to both. If total hours differ by more
than 25 percent, the paper says so beside every comparison.

## Reasoning

The anti-goals in `docs/spec.md` include a comparison skewed by effort. This is
the cheapest control that leaves evidence.

## Consequences

- The notebook template has an hours line per backend.
- E6 computes the difference.

## Sources

None. Process decision.
