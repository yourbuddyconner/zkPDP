# 0007: Policy as data in the zkVM backend

Status: accepted. Date: 2026-11-04.
Proposed by: assistant.
Accepted by: human, batch. The human separately said they prefer the zkVM's general properties, which this decision preserves.

## Context

A zkVM guest can have the policy set compiled in, or take it as input.

## Options considered

- **Policy compiled into the guest.** One image ID per policy version. Smaller input. Verifier pins many image IDs. Every policy edit rebuilds and redistributes the guest.
- **Policy as input.** One image ID forever. Journal commits to the policy hash. Verifier allowlists hashes. More cycles to parse the policy at run time.

## Decision

Policy as input. Image ID is the Cedar evaluator plus the shell, fixed.

## Reasoning

Policy change cost is one of the comparison axes and the zkVM backend's natural
advantage. Parsing cost is measured in E1 and the protobuf input form is the
mitigation if it is high.

## Consequences

- The verifier allowlists policy hashes.
- E1 measures parse cycles separately.

## Sources

S01 for the contrasting approach, where the compiled policy is the image.
