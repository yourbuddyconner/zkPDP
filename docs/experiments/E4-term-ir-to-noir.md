# E4: compiling Cedar to Noir through the Term IR

Status: planned, conditional on gate B. Written 2026-11-04.

## Question

Can validated Cedar policy sets be translated from the symbolic compiler's Term
IR to Noir automatically, and what fraction of the corpus does the translation
handle?

## Hypotheses

H4.

## Setup

- `cedar-policy-symcc` pinned at the version current when the phase starts. Version 0.8.1 was current on 2026-11-04.
- The circuit shape learned in E2.
- The whole corpus.

## Procedure

Strict mode.

1. Confirm the Term IR is reachable from a downstream crate before the SMT encoding step. If not, record it, and fall back to walking the validated AST. Record the fallback as a decision.
2. Write the translation, one Term operator at a time, in the order of frequency in the corpus.
3. After each operator, run the differential test: for every corpus policy set the translation handles, compare the circuit decision to the oracle on every request.
4. Record, per policy set, whether it compiles and which construct blocked it if not.
5. Record gates per compiled policy set.
6. Compute coverage: compiled policy sets over all policy sets in the corpus. Also compute coverage by construct.
7. Stop adding operators when the time box in the notebook is spent. Report coverage at that point.

## Measurements

Coverage by policy set and by construct. Gates per policy set. Compile time.
Differential test pass rate.

## Success and failure

- H4 supported if coverage by policy set is at least 80 percent.
- Any differential mismatch is a defect and is fixed or reported.
- Gate C in `docs/plan.md` reads the coverage number.

## Results

None yet.
