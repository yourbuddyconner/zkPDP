# 0016: The guest hashes the bytes it parses, canonical form is the authority's job, and the AuthZEN to Cedar mapping

Status: accepted, pending review. Date: 2026-11-04.
Proposed by: assistant, while writing the guest.
Accepted by: not yet reviewed by the human. Made under "proceed with 0 and 1".
Refines: 0012.

## Context

Decision 0012 defines hashes over canonical bytes: JCS for the request and
entities, Cedar's formatter for the policy set. Canonicalizing inside the
guest would cost cycles, and the formatter is a large dependency.

## Decision

1. The guest hashes exactly the bytes it was given and parsed. It does not canonicalize.
2. Whoever publishes a hash publishes the hash of canonical bytes. A decider that feeds non-canonical bytes gets a hash no verifier accepts. Soundness does not depend on the decider being honest about form.
3. The schema hash is over the schema file's bytes as the policy authority publishes them. Cedar's formatter covers policies, not schemas. This replaces the "after Cedar's formatter" wording for schemas in protocol version 0.
4. An absent schema has a schema hash of 32 zero bytes.
5. The mapping from an AuthZEN request to a Cedar request:
   - `subject.type` and `subject.id` become the principal UID. Same for `resource`.
   - `action.name` becomes `Action::"<name>"`. A name that contains `::` is read as a full Cedar UID, for namespaced actions.
   - `context` becomes the Cedar context after `nonce` and `epoch` are removed. Those two belong to the verifier, not to the policy.
   - `properties` on subject, resource and action are ignored in version 0. Attributes come from the entity store, which is what the entity commitment covers.
6. The verifier also checks that the top-level `decision` equals the journal's. Added as step 9 of the procedure.
7. Merkle details fixed: leaf 0 is SHA-256 of the entity count as 8 big-endian bytes. A node without a sibling moves up unchanged.

## Reasoning

Point 5's last item matters for soundness. If the guest took attributes from
the request's `properties`, the decider could invent them. Keeping attributes
in the committed store is the simplest way to keep input provenance in one
place.

## Consequences

- `docs/protocol.md` updated to match.
- The oracle's mapping in `corpus-tool` is written apart from the guest's, on purpose.
- Deployments that pass attributes in `properties` need a later version that commits to them.

## Sources

S02.
