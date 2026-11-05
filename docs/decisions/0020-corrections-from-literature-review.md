# 0020: Corrections from the literature review

Status: applied. The human approved committing and pushing on 2026-11-04. Date: 2026-11-04.
Proposed by: the literature review (`docs/literature/reports/ZK policy evaluation prior art.md`) and assistant.
Accepted by: not yet reviewed by the human. The assistant told the human it would make these. The human did not reply either way.

## Context

The review found statements in the project's documents that its sources contradict or do not support. Many were written by the assistant.

## Decision

Apply the corrections of fact, and leave choices of direction to the human.

Applied:

- "Unchanged" and "unmodified" now say one dependency is replaced (0015).
- "Lean-verified evaluator" and "verified front end" now say a Lean model with proofs, and differential testing of the Rust.
- "Ignores it and works as before" now reflects AuthZEN section 5.5. H6's threshold is unchanged, with a note.
- "Accepts any K" and "without an allowlist" now match `docs/protocol.md`, which keeps the policy allowlist.
- UltraHonk's zero-knowledge mode is described as designed to hide the witness, with no published proof found.
- Prezta's cost breakdown, read status and authors. POLARIS and ZK-EHR reclassified. The agents line updated. `in` and `like` in the spec match 0018 and 0019.
- The verification procedure says a verified deny is a deny.

Left for the human:

- The title "Proof-carrying authorization" in `docs/spec.md` and `paper/outline.md`. Cite the 1999 lineage and keep it, or retitle.
- Whether to add adversarial-witness tests, handle equivocation, or treat TEEs in the design. Listed as known gaps in `docs/threat-model.md`, not decided.
- Whether GordieHowe becomes a third backend, and whether its certificates change the compilation proof (0009).
- Whether to run a zkVM proof to the end.
- Whether to tell ZK-EHR's author about the access check.
- Whether the request is public in the private mode.

## Reasoning

A public README and a paper outline should not say things the project's own review contradicts. Direction is the human's call.

## Consequences

Answers from the human on 2026-11-04: keep the title (0021), add adversarial-witness tests (E9), add GordieHowe (0022), run a zkVM proof to the end, do not contact ZK-EHR's author, and the private-mode question stays open.

## Sources

The review, S01, S02, S05, S06, S08, S18, S19, S25, S26.
