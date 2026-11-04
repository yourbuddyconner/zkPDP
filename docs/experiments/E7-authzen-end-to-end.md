# E7: end-to-end with an AuthZEN enforcement point

Status: planned. Written 2026-11-04.

## Question

Does an unmodified AuthZEN PEP accept a proof-carrying response, and does a
modified PEP verify it?

## Hypotheses

H6.

## Setup

- A small PDP that wraps either backend and serves `/access/v1/evaluation` with the response in `docs/protocol.md`.
- An open-source AuthZEN PEP or client library chosen in Phase 0 and recorded here. Candidates seen so far: the Rust `openidauthzen` crate, the Go `authzen` module, the OPA AuthZEN plugin as a reference PDP.
- A modified copy of that PEP that runs the verification procedure.

## Procedure

Strict mode.

1. Point the unmodified PEP at the PDP. Send the TinyTodo requests. Record that each decision is enforced as returned and that no error is logged.
2. Point the modified PEP at the PDP. Send the same requests. Record that each proof verifies and each decision is enforced.
3. Tamper: alter the decision in a response. Record that the modified PEP refuses it and the unmodified PEP accepts it. This is the demonstration of what the proof adds.
4. Replay: send a stored response for a new nonce. Record that the modified PEP refuses it.
5. Record end-to-end latency with and without verification.

## Success and failure

- H6 supported if step 1 shows no behavior change.

## Results

None yet.
