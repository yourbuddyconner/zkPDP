# 0003: Two backends built as peers under one envelope

Status: accepted. Date: 2026-11-04.
Proposed by: human. "I think we can try both and benchmark + compare."
Accepted by: human, in conversation.

## Context

The first design ran Cedar in a zkVM. The human then asked about a Cedar
evaluator in Noir. The assistant argued the sane form of that is a compiler
from validated policies to circuits.

## Options considered

- **zkVM only.** Fastest to a result. Loses the client-side proving story.
- **Circuit only.** Best client-side numbers if it works. Loses the unmodified verified evaluator and policy as data.
- **Both, as separate projects.** Numbers would not be comparable.
- **Both, as backends of one envelope and one harness.**

## Decision

Both, as peers. Same inputs by hash, same journal, same proof object.

## Reasoning

The comparison is the contribution no prior work has. The human stated a
preference for the zkVM's general properties and willingness to settle for the
compiler. Building both under one envelope is what makes the comparison honest.

## Consequences

- Twice the engineering. The equal effort rule (0013) exists because of this.
- Phase gates B and C can cancel the circuit path and leave a negative result.

## Sources

S01, S14, S15.
