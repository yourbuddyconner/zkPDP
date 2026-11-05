# 0006: Cedar's symbolic compiler Term IR as the compiler front end

Status: accepted. Date: 2026-11-04.
Proposed by: assistant.
Accepted by: human, batch.

## Context

Compiling Cedar to a circuit needs a static, typed form of a policy set.

## Options considered

- **Our own walker over the validated AST.** Full control, no dependency on an unstable crate, but we reimplement the typing and the semantics Cedar already proved.
- **Translate the SMT-LIB output** of the symbolic compiler. Stable text format, but SMT theories for strings and sets are not circuit shaped.
- **The Term IR** inside `cedar-policy-symcc`. A simply typed IR the crate reduces Cedar to before SMT encoding. Modeled in Lean. Public modules `term`, `term_type`, `term_factory` exist at version 0.8.1.

## Decision

The Term IR, pinned by version. Fallback: our own walker, recorded as a new
decision if taken.

## Reasoning

A verified front end plus a differentially tested back end is the same trust
shape Cedar itself uses. It is the strongest trust story available for the
circuit backend.

## Consequences

- E4 step 1 confirms the IR is reachable before any translation is written.
- The crate is young. Version pinning and a fallback are required.

## Sources

S05, S06, S07.

## Update 2026-11-04

SymCert covers Cedar to Term in a Lean model. The Rust crate is reported, by a secondary source, to be differentially tested. Our Term to Noir step is outside any proof. Whether SymCert's theorems hold for a consumer that is not an SMT solver is unread. Say "a front end with a verified Lean model", not "a verified front end". Decision 0020.
