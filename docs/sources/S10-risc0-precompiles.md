# S10: RISC Zero cryptography precompiles

RISC Zero developer docs, version 3.0.
URL: https://dev.risczero.com/api/zkvm/precompiles
Accessed 2026-11-04. Trust: primary.

## What it says

Patched crates give accelerated guest execution for: `sha2` (SHA-256),
`tiny-keccak`, `k256`, `p256`, `curve25519-dalek` (used by `ed25519-dalek`),
`substrate-bn`, `bls12_381`, `blst`, `c-kzg`, `crypto-bigint`, `rsa`. Some need
the `unstable` feature. The precompiles do not guarantee constant-time
execution, so they should not be used with private data.

## What we take

- SHA-256 for the journal hashes and Ed25519 for mode 2 provenance in E1 and E3.
- The constant-time warning goes into the threat model and E3 step 7.
