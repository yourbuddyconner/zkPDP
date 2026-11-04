# 0001: Cedar as the policy language

Status: accepted. Date: 2026-11-04.
Proposed by: human. The original idea note named Cedar.
Accepted by: human, in conversation.

## Context

The project needs one policy language to run inside a proof system. Prezta
already did this for XACML in RISC Zero.

## Options considered

- **XACML 3.0.** Prezta's choice. Verbose XML. No verified evaluator. Prezta had to write a compiler to Rust and reached 83 percent of the conformance suite.
- **Rego (OPA).** Widely deployed. Go runtime, dynamic, large. No formal model. Hard to compile to a circuit.
- **Zanzibar-style (OpenFGA).** Relationship tuples only. No attribute conditions. Too narrow for the policy half of the question.
- **Cedar.** Rust evaluator. Lean formal model with differential testing. Validator that removes runtime errors from validated policies. Symbolic compiler with a typed IR. Small language designed to be analyzable.

## Decision

Cedar.

## Reasoning

Cedar is the only option where both backends are possible without writing a
policy compiler from scratch: the Rust crate runs in a zkVM as-is, and the
symbolic compiler's Term IR gives a static form for circuits. Its verified
components become part of the trust story. It also differentiates from Prezta.

## Consequences

- The paper's claims are about Cedar. Generalization to other languages is future work.
- We depend on the Cedar crate building for the guest target. E1 step 0 checks this.
- We depend on the Term IR being reachable. E4 step 1 checks this.

## Sources

S03, S04, S05, S01.
