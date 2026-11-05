# S25: A note on adding zero-knowledge to STARK

Haböck, U. and Kindi, A. ePrint 2024/1037, version dated 20 February 2025. 21 pages.
PDF supplied by the human on 2026-11-04. Not in this repository. Trust: primary. Introduction, theorem statements and the overhead section read. Proofs not checked.

## What it says

- The authors found gaps in the treatment of zero-knowledge in the Plonky2, RISC Zero and Triton codebases, in the protocol design. They say the gaps have since been patched and that none caused an exploitable vulnerability.
- It gives perfect honest-verifier zero-knowledge theorems for specific IOPs (Theorems 4, 6, 7). These concern protocols described in the note, not a named implementation.
- FFT-type quotient decomposition is the main pitfall. Permutation arguments leak through incompleteness of the grand product unless handled separately.
- Overhead with minimal randomization is small: a ratio of about 1 + 1/log|H| in one configuration, expected to be an upper bound in wide-trace AIRs.

## What we take

- This corrects the way the project first read advisory GHSA-5xgj-pmjj-gw49 (S08). The paper does not show RISC Zero is not zero-knowledge. It reports gaps, says they were patched, and reports no attack.
- It does not prove RISC Zero is zero-knowledge either. The note never mentions `ZK_CYCLES` and gives no RISC Zero parameters. So the open question stays open: S08's advisory and RISC Zero's own security page remain the statements about the implementation, and the bound would have to be checked against RISC Zero's query counts.
- Any privacy claim for the zkVM backend must say "honest-verifier" where that is what a theorem gives.
