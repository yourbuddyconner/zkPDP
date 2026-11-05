# 0018: A record-shaped entity commitment for the circuit backend

Status: accepted, pending review. Date: 2026-11-04.
Proposed by: assistant, while designing the E2 circuit.
Accepted by: not yet reviewed by the human.
Extends: 0012.

## Context

`merkle-sha256-v0` commits to each entity's JSON. A circuit would have to
parse JSON to read an attribute. `sha256-jcs-v0` commits to the whole store,
which a circuit cannot open at all. The circuit also has to prove a deny,
which means proving a principal is not in a group.

## Options considered

- **Per-fact leaves**: one leaf per attribute and one per ancestor pair. Membership is one path. Non-membership needs a sorted tree and adjacent-leaf proofs, which is more circuit and more protocol.
- **Rebuild the entity's JCS bytes in the circuit** from typed fields. Keeps one commitment for both backends. Sets and nested records make the template open-ended.
- **A fixed-shape record per entity**: UID hash, attribute values in name order, and the transitive closure of ancestors, each a 32-byte slot. The circuit opens two leaves, the principal's and the resource's, and reads fixed positions. Membership and non-membership are both one scan of the ancestor slots.
- **Poseidon instead of SHA-256** for the tree. Much cheaper in a circuit. The zkVM backend would then need the same hash, with no precompile for it.

## Decision

The fixed-shape record, with SHA-256, named `merkle-sha256-record-v0`.
Defined in `bench/envelope/src/records.rs`. 8 attribute slots, 8 ancestor
slots. The tree shape is that of `merkle-sha256-v0`.

## Reasoning

Two openings per decision keeps the circuit small, and a deny is as easy to
prove as a permit. SHA-256 keeps the commitment usable by the zkVM backend
through its precompile, so one published root serves both. Hashing is where
nearly all the gates go, so the hash choice is the first thing to revisit if
gate count matters later.

## Consequences

- The attribute authority computes the transitive closure. The decider never does.
- An entity with more than 8 attributes or 8 ancestors cannot be committed. The limits are constants and can grow at a cost in gates.
- Attribute names are not in the leaf. Position is fixed by name order within an entity type, and the type is in the UID. A schema change that adds an attribute changes positions and so changes the root.
- Values the policies do not read, such as TinyTodo's `tasks` set, are committed as one opaque hash.
- A principal or resource that is not in the store cannot be opened. Cedar treats an unknown entity as one with no attributes and no parents. The circuit gives no proof instead. Limitation, noted.
- E3's mode 1 for the zkVM backend should use this same commitment.

## Sources

None beyond the Noir SHA-256 library.
