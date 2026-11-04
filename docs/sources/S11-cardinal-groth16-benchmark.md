# S11: Merkle wrapping with RISC Zero benchmark

Cardinal Cryptography, 2024. Repository.
URL: https://github.com/cardinal-cryptography/merkle-wrapping-with-risc0
Accessed 2026-11-04. Trust: secondary. One machine, an older RISC Zero version.

## What it reports

On an i7-12800H with 16 GB RAM: Groth16 verify 5.4 ms, succinct verify 12.4
ms, wrap from recursion proof to Groth16 25.9 s, Groth16 proof 256 bytes,
recursion proof 217 kB, composite receipt about 232 kB.

## What we take

Order-of-magnitude expectations for H8 and the proof size column. All to be
re-measured.
