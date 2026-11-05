# 0024: The request context is a record behind its own public hash

Status: accepted, pending review. Date: 2026-11-05.
Proposed by: assistant, under step 4 of decision 0023.
Accepted by: not yet reviewed by the human. Made under "go ahead and do 1 through 6".

## Context

15 of the 25 terms the compiler could not translate read the request context. The request hash covers canonical JSON. A circuit rebuilds that JSON by concatenation, which only works for known field names. Rebuilding arbitrary context JSON inside a circuit would be large and fragile.

## Options considered

- Rebuild JCS of the context inside the circuit. Open-ended, as with entities.
- Pass the context as public inputs, one per attribute. Reveals the context in the proof's public data. The verifier knows the context, so this leaks nothing to it, but it grows with the schema and fixes a format per action.
- A record block in the schema's attribute order, the same encoding as entities, bound to one public hash.

## Decision

The last. For the circuit backends:

- `context_hash` is a new public input: SHA-256 of the record block of the context's attributes, in the layout the schema gives this principal type, action and resource type.
- `request_hash` covers the request with its context reduced to the nonce.
- The verifier computes both from the request it will enforce.
- The nonce and epoch stay outside the attributes.

The zkVM backend is unchanged: its request hash covers the whole context in JCS, because it can parse JSON.

## Reasoning

One encoding for entities and context means one set of tests. The verifier already holds the context, so it can compute the hash with no new trust.

## Consequences

- The two kinds of backend hash the request differently, so `format` must say which applies. `docs/protocol.md` records it.
- The verifier needs the schema's layout to compute `context_hash`.
- Each circuit grows by about 20,500 gates, one record digest.
- Extension values in a context remain unsupported.

## Sources

The E4 coverage results.
