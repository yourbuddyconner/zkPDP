# 0011: The provenance system and AI disclosure

Status: accepted. Date: 2026-11-04.
Proposed by: human for the requirement. "I want to make sure we maintain the provenance of the decisions and reasoning and research we make." Assistant for the shape.
Accepted by: human, batch for the shape.

## Options considered

- A single running design document edited in place. Loses history except in git diffs.
- Git history alone. Does not record reasoning or rejected options.
- Decision records, source notes with trust levels, an append-only dated notebook, a hypotheses registry with pre-set thresholds, one file per experiment, machine-written results.

## Decision

The last. Rules in `docs/provenance.md`. The AI assistant's part is disclosed
in the README and in each decision record's "proposed by" line.

## Reasoning

A paper reader, a reviewer, or the researcher a year from now should be able to
answer "why did we do it this way" and "where did this number come from" from
the repository alone. Disclosure of AI involvement is a venue requirement and
a matter of honesty.

## Consequences

- Overhead on every working day. Accepted.
- Batch-accepted decisions are flagged so they can be re-read.

## Sources

None. Process decision.
