# 0012: Journal contents and canonicalization

Status: accepted. Date: 2026-11-04.
Proposed by: assistant.
Accepted by: human, batch.

## Context

The journal is what the verifier reads. Its fields and their byte-level
definitions decide what the proof is about.

## Options considered

Request hash:
- Hash of the raw request bytes. Breaks on any re-serialization.
- Hash of JCS (RFC 8785) canonical JSON. Chosen.

Policy hash:
- Hash of the raw policy text. Whitespace and comments change it.
- Hash of the experimental protobuf form. Deterministic but ties the hash to an experimental feature and a prost version.
- Hash of the text after Cedar's formatter with default options. Chosen. Protobuf stays available as an input form for cycle savings without affecting the hash.

Entity commitment:
- Plain hash of the JCS entity array. Simple. Chosen as the version 0 form for E1.
- Merkle root over transitively closed entities, so the circuit backend and mode 1 provenance can prove inclusion. Chosen as the second form. Both are named by `entities.type`.

Reasons:
- Always include fired policy IDs. Good for audit, leaks attributes.
- Never include. Loses audit value.
- Optional, off in the private mode. Chosen.

## Decision

As written in `docs/protocol.md`.

## Consequences

- The envelope crate implements JCS and calls the Cedar formatter.
- The leaf and inner node encoding of the Merkle tree is fixed in the protocol so both backends agree.
- H7 tests the reasons leak.

## Sources

S02 for the request shape. RFC 8785 for JCS.
