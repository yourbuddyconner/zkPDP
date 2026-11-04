# Provenance rules

This project will become a paper. A reader of that paper must be able to trace
every claim to a measurement or a source, and every design choice to the moment
it was made and the reasoning behind it. These rules make that possible.

## Decisions

- A decision that changes what gets built gets a record in `docs/decisions/` before or with the change.
- Records are numbered `NNNN-<slug>.md` and never renumbered. A reversed decision gets a new record that names the old one. The old record's status changes to `superseded by NNNN`.
- Each record states who proposed the choice and who accepted it. Write `human` or `assistant`. When the human accepted a batch of proposals without reviewing each one, say so.
- Each record lists the options that were rejected and why.

## Sources

- Every external claim used in a design or the paper gets a note in `docs/sources/`.
- A source note records the access date, what the source claims, what we take from it, and a trust level: `primary` (spec, paper, official docs), `secondary` (blog, vendor benchmark, third-party summary), or `unverified` (seen in a search summary, not read in full).
- A number taken from a secondary source is marked as such wherever it is quoted.
- `docs/sources/README.md` is the index. `docs/sources/references.bib` holds the citation keys the paper will use.

## Notebook

- One file per working day in `docs/notebook/`, named `YYYY-MM-DD.md`.
- Entries are written the same day and are append only. Correct a mistake with a later dated note, not an edit.
- Record what was tried, what was observed, what was concluded, and what was left open. Record dead ends.
- Record hours spent on each backend. The benchmark protocol depends on this.

## Hypotheses

- `docs/hypotheses.md` lists every hypothesis with a numeric threshold and the experiment that tests it.
- Thresholds are written before the experiment runs. A changed threshold is a decision record, with the reason.
- Status moves from `open` to `supported`, `refuted` or `inconclusive`. A refuted hypothesis stays in the file and in the paper.

## Experiments and results

- Each experiment has a file in `docs/experiments/` written before it runs: question, setup, procedure, measurements, success and failure criteria.
- Results are written by the harness, not by hand, as JSON Lines in `results/`. Each line carries the git commit, toolchain versions, hardware description, timestamp and the hash of every input.
- Failed and aborted runs are kept and marked.
- A result that is quoted in the paper is quoted with its result file path.

## Paper

- Every sentence in the paper that states a fact links to an experiment ID, a result file or a source key.
- The abstract is rewritten after the results exist. The draft abstract in `paper/outline.md` is a statement of hope and is labeled as one.

## Names

- Name things by what they are. Call the two backends "Cedar in a zkVM" and "Cedar compiled to a circuit", or the "zkVM backend" and the "circuit backend". Do not coin product names.

## Commits

- A commit message names the decision, experiment or source IDs it touches.
- The spec, plan and protocol can change. A change after the first approval gets a decision record and a notebook line.
