# 0010: Results as committed JSON Lines with run metadata

Status: accepted. Date: 2026-11-04.
Proposed by: assistant.
Accepted by: human, batch.

## Options considered

- Spreadsheets. Easy to edit, which is the problem.
- A database. Not in the repository, so not reproducible from a checkout.
- CSV. No nesting for machine and toolchain metadata.
- JSON Lines, one line per measurement, written by the harness, committed to git.

## Decision

JSON Lines in `results/`, schema in `results/README.md`, never hand-edited,
each line carrying commit, toolchain, machine, input hashes and status.

## Reasoning

Every number in the paper must trace to a line a reader can find in the
repository at a known commit.

## Consequences

- The harness writes results as it goes so aborted runs leave a record.
- Failed runs are kept.

## Sources

None. Process decision.
