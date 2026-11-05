# Protocol: a proof inside an AuthZEN response

Version 0. Changes get a decision record.

This document says how a proof rides inside the OpenID AuthZEN Authorization
API 1.0, how the journal is formed, and how a verifier checks it. It uses the
spec's own terms and adds nothing to its endpoints.

## Request

An AuthZEN access evaluation request, unchanged. The verifier may put a nonce
in `context`:

```json
{
  "subject":  { "type": "user", "id": "alice", "properties": { } },
  "action":   { "name": "read" },
  "resource": { "type": "document", "id": "doc-17", "properties": { } },
  "context":  { "nonce": "b64url...", "epoch": "2026-11-04T00:00:00Z" }
}
```

`nonce` and `epoch` are optional. The verifier decides which it needs. Both are
plain context fields and mean nothing to a PDP that does not know them.

## Response

An AuthZEN decision. The proof sits in `context.proof`. AuthZEN says receivers
must ignore unknown fields, so a PEP that does not know `proof` behaves as
before.

```json
{
  "decision": true,
  "context": {
    "proof": {
      "format": "risc0-succinct-v0",
      "evaluator": "sha256:…",
      "journal": {
        "v": 0,
        "request_hash": "sha256:…",
        "policy_hash": "sha256:…",
        "schema_hash": "sha256:…",
        "entities": { "type": "merkle-sha256-v0", "root": "sha256:…" },
        "nonce": "b64url…",
        "decision": true,
        "reasons": ["policy0", "policy3"]
      },
      "seal": "b64url…",
      "compilation_proof": null
    }
  }
}
```

Fields:

- `format`: which proof system and receipt kind. Values this project uses: `risc0-composite-v0`, `risc0-succinct-v0`, `risc0-groth16-v0`, `noir-ultrahonk-v0`, `noir-ultrahonk-zk-v0`.
- `evaluator`: the identity the verifier pins. For the zkVM backend, the image ID. For the circuit backend, the SHA-256 of the verification key.
- `journal`: the public outputs. Defined below. For the zkVM backend this is the decoded RISC Zero journal. For the circuit backend these are the public inputs, in this order.
- `seal`: the proof bytes.
- `compilation_proof`: optional. A second proof object, from the zkVM backend, whose journal states that `policy_hash` compiled to this `evaluator`. See below.

## Journal

| Field | Definition |
|---|---|
| `v` | Journal version. 0. |
| `request_hash` | SHA-256 of the JCS (RFC 8785) bytes of the object `{subject, action, resource, context}` from the request, including the nonce. |
| `policy_hash` | SHA-256 of the UTF-8 bytes of the policy set after Cedar's formatter with default options. |
| `schema_hash` | SHA-256 of the bytes of the schema file in Cedar schema syntax, as the policy authority publishes it. 32 zero bytes when no schema is used. Required for the circuit backend. |
| `entities.type` | `sha256-jcs-v0`: SHA-256 of the JCS bytes of the entity array sorted by UID, before transitive closure. `merkle-sha256-v0`: root of a binary Merkle tree whose leaves are SHA-256 of the JCS bytes of each entity after transitive closure, sorted by UID, with SHA-256 of the entity count as 8 big-endian bytes as the first leaf, SHA-256 over `0x01 || left || right` for inner nodes, and a node without a sibling moved up unchanged. |
| `entities.type`, continued | `merkle-sha256-record-v0`: the same tree over fixed-shape entity records, for circuits. Each leaf is SHA-256 of the UID hash, the hash of the attribute slots and the hash of the ancestor slots. Defined in `bench/envelope/src/records.rs`. Decision 0018. |
| `entities.root` | The hash. |
| `nonce` | Echo of the request nonce, or empty. |
| `decision` | Boolean. True is permit. |
| `reasons` | Optional. Policy IDs of the policies that determined the decision, as Cedar reports them. Omitted in the private mode. |

For batch responses the journal is an array of these objects in request order.

## Verification

Strict mode. Each step is one check. Stop at the first failure.

1. Read `format`. Refuse formats you do not support.
2. Compare `evaluator` with the pinned identity for that format. Refuse a mismatch, unless a `compilation_proof` is present. Then verify the compilation proof first, as below.
3. Verify `seal` against `evaluator` and `journal` with the proof system's verifier.
4. Compute `request_hash` from the request you are about to enforce. Refuse a mismatch.
5. Check `policy_hash` is in your allowlist. Refuse a mismatch.
6. Check `schema_hash` if you require one.
7. Check `entities.root` is the commitment you currently accept. Refuse a mismatch.
8. Check `nonce` equals the nonce you issued, or that the epoch is current.
9. Check the top-level `decision` equals the journal's `decision`. Refuse a mismatch.
10. Enforce `decision`.

## Compilation proof

A proof object from the zkVM backend. Its `evaluator` is the image ID of the
compiler guest, which the verifier pins. Its journal is:

```json
{ "v": 0, "policy_hash": "sha256:…", "schema_hash": "sha256:…", "compiler_hash": "sha256:…", "vk_hash": "sha256:…" }
```

A verifier that accepts a compilation proof pins the compiler image ID instead
of individual verification keys. It still allowlists `policy_hash`.

## Discovery

A PDP that produces proofs lists a capability in its AuthZEN metadata at
`/.well-known/authzen-configuration`. AuthZEN defines `capabilities` as an
array of registered URNs. No URN is registered for this. Until one is, the
project uses a placeholder and says so in the paper.

## Mapping an AuthZEN request to a Cedar request

- `subject.type` and `subject.id` form the principal UID. `resource.type` and `resource.id` form the resource UID.
- `action.name` becomes `Action::"<name>"`. A name that contains `::` is a full Cedar UID.
- `context` becomes the Cedar context after `nonce` and `epoch` are removed.
- `properties` are ignored in version 0. Attributes come only from the committed entity store. See decision 0016.

## Who canonicalizes

The evaluator hashes the bytes it parses. It does not canonicalize. The party
that publishes a hash publishes the hash of canonical bytes, and the verifier
computes the request hash from its own canonical request. A decider that
supplies other bytes produces a hash no verifier accepts.

## Canonicalization notes

- JCS is used so two JSON serializations of the same request hash the same.
- Cedar's formatter is used so whitespace and comments in a policy set do not change its hash. The alternative, hashing the experimental protobuf form, is recorded in decision 0012.
- UIDs sort by their canonical Cedar text, `Type::"id"`, as UTF-8 bytes.

## Not covered

- The search endpoints.
- How the verifier learns identities and hashes.
- Transport security. AuthZEN's own requirements apply.
