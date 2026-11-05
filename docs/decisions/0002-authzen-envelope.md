# 0002: AuthZEN as the wire format, proof in the response context

Status: accepted. Date: 2026-11-04.
Proposed by: human for AuthZEN. Assistant for the placement in `context.proof`.
Accepted by: human, batch.

## Context

The proof has to reach a verifier somehow. The original idea note said
"AuthZen + Proof". AuthZEN 1.0 became Final in January 2026.

## Options considered

- **A new endpoint** beside `/access/v1/evaluation`. Clean but needs every PEP to change.
- **An HTTP header.** Invisible to PEPs that do not look. Proofs are large and headers are a poor fit.
- **A token** that carries the proof, like a JWT with a proof claim. Good for prove-once present-many. Deferred, not rejected. Fits a later phase.
- **The response `context` object.** AuthZEN defines it as implementer-defined, and receivers must ignore unknown fields. Section 11.6 lets a PDP sign its response without saying how.

## Decision

The response `context.proof` object, as in `docs/protocol.md`. Nonce and epoch
ride in the request `context`.

## Reasoning

Backward compatibility is one of the project's goals and this is the only option
that needs no PEP change. The spec's own extension point exists for this.

## Consequences

- E7 must show an unmodified PEP accepting the response.
- No capability URN exists for proofs. The paper uses a placeholder.
- The token form is noted as future work.

## Sources

S02, S23.

## Update 2026-11-04

AuthZEN section 5.5 lets a PEP reject a permit over context it does not understand. "No specification change" holds. "No behavior change" depends on the implementation. E7 reports named PEPs. Decision 0020.
