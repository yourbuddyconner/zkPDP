# 0021: Keep "proof-carrying authorization" and cite its lineage

Status: accepted. Date: 2026-11-04.
Proposed by: assistant.
Accepted by: human.

## Context

The spec and paper outline use "proof-carrying authorization" as a title. The review found that the term is Appel and Felten's 1999 idea, with Bauer's Grey, SD3 and Evidence-Based Audit following it. The project's documents did not cite any of them.

## Options considered

- Retitle to avoid the term. Hides the lineage, and a reviewer would still raise it.
- Keep the term and cite the lineage.

## Decision

Keep it and cite it. The paper states what is new against that work: succinctness, hidden premises, and proof of non-monotonic outcomes over a committed closed world. The third point is the project's own argument, with no source stating it.

## Consequences

`docs/spec.md`, `paper/outline.md` and `paper/related-work.md` name the lineage. The related-work section opens with it.

## Sources

`appel1999pca`, `bauer2001pca`, `jim2001sd3`, `vaughan2008evidence` in `docs/literature/references.bib`.
