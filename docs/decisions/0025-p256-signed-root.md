# 0025: Signed credentials are an ECDSA P-256 signature over the store's root

Status: accepted, pending review. Date: 2026-11-05.
Proposed by: assistant, under step 5 of decision 0023.
Accepted by: not yet reviewed by the human. Made under "go ahead and do 1 through 6".
Changes: E3 said Ed25519.

## Context

Without this mode, a decider with a pinned root is bound to data the verifier already trusts, and nothing shows how an untrusted decider's facts can be vouched for by a third party without the verifier seeing them.

## Options considered

Signature scheme:
- Ed25519, as E3 planned. RISC Zero has a precompile. Noir has no built-in, and GordieHowe has no gadget we found.
- ECDSA P-256. Noir has a built-in. RISC Zero has a precompile. GordieHowe proves it for mobile driving licenses. Deployed credentials use it, so this is the case a deployment would meet.

What is signed:
- Each entity's record. The verifier learns which entities exist only through the proof, and a proof needs several signature checks.
- The store's root and an epoch. One signature per proof.

## Decision

An attribute authority signs SHA-256 of `zk-pdp-root-v0`, the root and the epoch as 8 big-endian bytes. The proof checks the signature inside. The root is private. The verifier pins the authority's key and the current epoch in place of the root.

## Reasoning

One check per proof, regardless of how many entities a decision opens. A hidden root also stops the root acting as an obvious identifier between proofs of different deciders.

## Consequences

- Revocation is by epoch.
- The test key in `bench/circuit-tool` is derived from a public string and is for this corpus only.
- This is integrity, not a complete private mode. The request is still public, and whether it should be is an open question for the human. A proof reveals the epoch and issuer. Attributes are hidden only as far as the proof system is zero-knowledge. RISC Zero's precompiles are documented as not constant time.
- E3's second mode is measured with P-256, not Ed25519.

## Sources

S10, S26, S29, the literature review.
