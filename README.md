# zk-pdp

Research project. Question: can an authorization decision be trusted because of
what was computed, not because of who computed it?

We run the Cedar policy language inside a zero-knowledge proof system, in two
ways, and carry the resulting proof inside a standard AuthZEN response. A
verifier that holds a policy hash and an evaluator identity can then accept a
decision made by a party it does not trust: a client, an AI agent's runtime, a
third-party decision service, or a device in someone else's building.

Status on 2026-11-04: design stage. No code yet. Everything here is a plan and
a record of how the plan was reached.

## Read in this order

1. `docs/spec.md`: what we are building and why, goals, non-goals, architecture.
2. `docs/plan.md`: the research plan, phases, decision gates, exit criteria.
3. `docs/hypotheses.md`: the claims we intend to test, with thresholds set before any run.
4. `docs/protocol.md`: how a proof rides inside an AuthZEN response.
5. `docs/threat-model.md`: parties, adversary, what the proof does and does not give you.
6. `docs/benchmark-protocol.md`: how the two backends are measured so the comparison is fair.
7. `paper/outline.md`: the paper this is meant to become.

## Provenance

This project keeps a record of every decision, every source and every day of
work. The rules are in `docs/provenance.md`. The records live in:

- `docs/decisions/`: numbered decision records. Context, options, choice, reasoning, who decided.
- `docs/sources/`: one note per external source. What it claims, what we take from it, how far we trust it.
- `docs/notebook/`: dated lab notebook. Append only.
- `docs/experiments/`: one file per experiment. Question, procedure, pre-stated success criteria, results.
- `results/`: machine-written measurements with commit hash and hardware. Never hand-edited.

## Layout

```
docs/        design, plan, protocol, threat model, provenance records
paper/       outline, related work, later the paper itself
corpus/      policy sets, schemas, entity stores and requests used by every experiment
bench/       the shared harness and the two backends (empty until Phase 1)
results/     measurement output (empty until Phase 1)
```

## Disclosure

An AI assistant (Claude, Anthropic) took part in the ideation, literature
search and drafting recorded here. The notebook and decision records say which
ideas and choices came from the human researcher and which the assistant
proposed. Any venue this work is submitted to will be told the same.
