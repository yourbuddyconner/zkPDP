# 0026: Fixed-layout request and UID hashes for the record backends

Status: accepted, pending review. Date: 2026-11-05.
Proposed by: assistant, under step 3 of decision 0023.
Accepted by: not yet reviewed by the human. Made under "go ahead and do 1 through 6".
Changes: the record commitment of 0018 becomes `merkle-sha256-record-v1`. Refines 0024.

## Context

GordieHowe needed one compiled system per combination of string lengths. Noir handled lengths with bounded vectors at a cost in gates, and needed a check that strings hold no quote or backslash. Both costs exist only because circuits rebuilt canonical JSON and `Type::"id"` text.

## Options considered

- `sha256_varlen` plus shifting each field to a private offset. Works, and adds a barrel shifter per field.
- Compile per shape and cache. Keeps the limit and its operational cost.
- Hash a fixed layout. The verifier computes the hash from the request, so the layout is free to choose.

## Decision

A string is one length byte then its bytes, zero-padded to a capacity: type 24, id 48, action 32, nonce 32. Longer strings are refused.

- `uid_h` is SHA-256 of the type's and the id's fixed forms.
- For circuit backends, `request_hash` is SHA-256 of the tag `zk-pdp-req-v1`, then the fixed forms of action, nonce, resource id, resource type, subject id and subject type.
- For those backends the nonce is bound only through `request_hash`. It is not a separate public input.

The zkVM backend's `request_hash` stays JCS, since it parses JSON. Its record guests use the new `uid_h`.

## Reasoning

One compiled system per policy on every backend, which makes the comparison like for like and suits deployment. The padding is inside the hash, so a non-canonical encoding gives a hash no verifier accepts, and the padding and plain-string checks go away.

## Consequences

- E2, E4, E8, E9 and E11 are rerun under v1. Earlier result files stay and describe v0.
- IDs over 48 bytes cannot be committed. This limit already existed.
- The TinyTodo Lean export of E10 must be redone.

## Sources

None. E8 and E9 motivate it.
