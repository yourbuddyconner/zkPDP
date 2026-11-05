# S24: Longfellow ZK, IETF draft

Frigo, M. and shelat, a. Internet-Draft draft-google-cfrg-libzk-03, 23 September 2026, intended status Informational.
URL: https://www.ietf.org/archive/id/draft-google-cfrg-libzk-03.html
Supplied by the human on 2026-11-04. Trust: primary, read through a fetch tool's summary. Check any statement against the page before citing.

## What it says

- A non-interactive zero-knowledge argument for any circuit: Ligero commitment plus sumcheck. No trusted setup. Relies on a collision-resistant hash.
- Circuits are defined in a separate document. A circuit identifier is absorbed into the transcript, and the parties must agree on the circuit as part of the statement.
- No policy language, predicates or benchmarks appear in the text read.
- Security Considerations claims completeness, soundness and zero knowledge, and says a mechanically verifiable proof for the combined scheme is in preparation.
- Underlying paper: Frigo and shelat, "Anonymous credentials from ECDSA", ePrint 2024/2010. Not supplied, not read.

## What we take

A candidate third backend: transparent setup, general circuits. It does not address how a verifier authenticates a circuit, which is what the compilation proof targets. S26 reports gaps in the published analyses and a separate Lean analysis by Google.
