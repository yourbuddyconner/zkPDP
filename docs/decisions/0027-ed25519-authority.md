# 0027: An Ed25519 authority, verified in Noir by our own code

Status: accepted. Date: 2026-11-05.
Proposed by: the human, at IIW, for peer-to-peer trust stacks whose keys are Ed25519: "Can you just go implement ed25519 verification".
Accepted by: the human's request. The choice of implementation is the assistant's, below.
Changes: adds to 0025, which chose P-256. P-256 stays.

## Context

The P2P Trust Stack (fpndtg.com, read 2026-11-05) gives every agent Ed25519 signing keys, and its relationship credentials are W3C credentials signed by them. Noir has a built-in for ECDSA P-256 and none for Ed25519, so our signed mode could not take a key from that stack.

## Options considered

- `willemolding/noir-ed25519` v0.1.0-alpha.1, about 228,000 gates for a 32-byte message, unaudited, 0 stars. Read, not used: the permission system refused fetching it as a build dependency, and we did not work around that.
- Our own verifier on noir-lang's `noir-bignum` v0.10.0, the organization whose SHA-256 we already use, with SHA-512 written here for a fixed length. Chosen.
- Prove over P-256 only and ask such stacks to add a P-256 key. Leaves the question open.

## Decision

`bench/circuit/ed25519` is a Noir library: RFC 8032 section 5.1.7, cofactored, `[8][S]B = [8]R + [8][k]A`. Decoding is strict: y and x below p, the prover's x must match the sign bit and lie on the curve, `S < L`, and a public key of small order is refused. Complete affine Edwards addition, one shared double-and-add chain over 253 bits. SHA-512 over a fixed length. The authority signs `zk-pdp-root-v0 || root || epoch` directly, the same 54 bytes P-256 signs after hashing.

## Reasoning

Written from the RFC, it is short enough to read in full. The noir-lang SHA-512 repository has no releases and a branch named for a fix to its variable-length mode, so a fixed-length SHA-512 avoids that code path. Simple double-and-add was chosen for being easy to check, not for speed.

## Consequences

- About twice the gates of the library we read, 436,722 for the check alone: room to optimize with windows.
- Unaudited, ours. Tested with RFC 8032 section 7.1 tests 1 to 3, against Python's `cryptography`, and with the attacks in E13.
- A credential signed by such a stack is still not usable directly: its W3C credential format is not parsed here. This verifies Ed25519 over our message.
