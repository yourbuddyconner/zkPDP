# S09: RISC Zero recursion and shrink-wrapping

RISC Zero developer docs.
URLs: https://dev.risczero.com/api/recursion and
https://dev.risczero.com/api/blockchain-integration/shrink-wrapping
Accessed 2026-11-04. Trust: primary, read through a search summary.

## What they say

- Recursion compresses a composite receipt to a succinct receipt. The final `compress()` step produces a Groth16 receipt over BN254 verifiable on-chain.
- Groth16 is the only officially supported shrink-wrap type.

## What we take

- The three receipt kinds measured in E1.
- Groth16 brings a trusted setup assumption and a slow wrap step. Whether a server verifier needs it at all is an open question.
