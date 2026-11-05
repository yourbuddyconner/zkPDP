# S26: GordieHowe

Sharp, C. and Grubbs, P. "Faster and Smaller Anonymous Credentials from Standardized Signatures." ePrint 2026/2615. 156 pages.
Code: https://github.com/cmlsharp/gordiehowe. Not run or inspected by this project.
PDF supplied by the human on 2026-11-04. Not in this repository. Trust: primary, authors' own measurements. Abstract, introduction, technical overview, sections 7 (setup), 8.3, 8.4, 9 (part) and 10 read. Most of the 156 pages not read. Peer-review status not checked.

## What it says

- A language, a certifying compiler and a proof system for relations over several fields, aimed at credentials over ECDSA, RSA and ML-DSA.
- The compiler exports the source and realized programs to Lean with certificates. Lean's kernel checks per program that the realized program is valid and that its predicates imply the source program's, up to small probability over challenges, and that every source witness extends.
- The proof system's knowledge soundness and zero knowledge are machine-checked in Lean, about 190,000 lines, no declared axioms. End to end only for ML-DSA-44 proofs of possession. For other programs the guarantee is conditional on the source program being complete and round-by-round knowledge sound.
- Remaining assumptions: three about the GKR circuits from Longfellow's compiler, that the export records the program actually used, that the C++ prover and verifier implement the Lean model, that the Lean specifications match the standards, and that extractor and simulator running times hold (argued on paper).
- ECDSA mDL presentation: 64.8 ms prove, 275.5 KiB proof, 10.1 times faster than Longfellow. Four cores of an AMD EPYC 9754. No phone measurement seen in the text searched. The verifier is not sublinear in program size.
- It reports a gap in the published soundness analyses of Ligero's masked tests, and that Longfellow's stated statistical zero knowledge does not follow for its salt length, with no attack by a bounded adversary. It cites a concurrent Lean analysis of Longfellow by Google.

## What we take

- The nearest thing to a verified compilation story for ZK, and the strongest published proof-system assurance we have seen. A reviewer will ask why zk-pdp does not build on it.
- The language is for writing relations by hand. It has no authorization language and no policy compiler in the text read, so it does not take zk-pdp's claim. It would be a candidate back end for Term IR.
- Per-program certificates checked in Lean are a direct alternative to the compilation proof (decision 0009). A certificate says the output is faithful. The compilation proof says only which compiler ran. The two answer different questions and could combine.
- It is a credentials comparison at a different scale: tens of milliseconds and hundreds of kilobytes, against our seconds and 14.7 kB. Different hardware and different statements, so not a direct comparison.

## Update 2026-11-04

Built and run by this project at commit 4837770, natively on Apple silicon with local patches. See E8. Upstream's own tests were not run here.
