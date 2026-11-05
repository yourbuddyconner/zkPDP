# S08: RISC Zero zero-knowledge advisory

GitHub security advisory GHSA-5xgj-pmjj-gw49 on risc0/risc0.
URL: https://github.com/risc0/risc0/security/advisories/GHSA-5xgj-pmjj-gw49
Accessed 2026-11-04. Trust: primary.

## What it says

Research by Habock and Kindi indicates several STARK implementations,
including the RISC Zero zkVM, do not meet the requirements to provably assert
zero knowledge. RISC Zero is not aware of attacks. Applications that depend on
privacy should review the research.

## What we take

- Every RISC Zero format in `docs/protocol.md` is labeled succinct, with zero-knowledge unproven, in the privacy analysis (E6).
- The private mode (mode 3) on the zkVM backend carries this caveat. The Groth16 wrap's zero-knowledge property is a separate question to be checked in Phase 0.

## Update 2026-11-04

S25 reports gaps in RISC Zero's design that it says were patched, with no attack. The advisory shows no patched version. These address different things and a person should reconcile them. RISC Zero's security model says it has written no mathematical argument for zero knowledge. Say "unproven", not "known to fail".
