# S06: SymCert

Torlak, E. and others. "SymCert: Verifying SMT-based policy analyses." Amazon
Science, 2026.
URL: https://www.amazon.science/publications/symcert-verifying-smt-based-policy-analyses
Accessed 2026-11-04. Trust: unverified. Phase 0 task to read.

## Expected content

A Lean framework for verified SMT-based analyses of Cedar policies: a verified
symbolic compiler and authorizer, and a counterexample extractor.

## What we take, pending reading

The formal statement of what the Term IR preserves, which is what makes it a
trustworthy front end for our translation.

## Update 2026-11-04

Read in part by the literature review. The title page shows Emina Torlak as sole author, FMCAD 2026. It verifies a Lean compiler over a 42-operator term language on Cedar 4.4, covering datetime, duration and tags, and trusts the solver. Whether its theorems hold for a consumer that is not an SMT solver is unread and is the most urgent item for a person.

## Update 2026-11-04, later: theorem statements read by the assistant

PDF text read, proofs and Lean sources not checked in full. Theorem 1, compiler correctness, and Theorem 4, authorizer correctness, are about `evals`, a bottom-up evaluation of a term under an interpretation. Reading the statements, the SMT solver is not in them: for well-formed inputs where the concrete environment matches the interpreted symbolic one, the concrete authorizer and the evaluated symbolic authorizer agree. The solver is trusted only for the analyses. So the earlier worry, that the theorems may not hold for a consumer that is not an SMT solver, looks answered for the Lean model: a circuit that computes `evals` faithfully would inherit it. What remains: the theorem covers the Lean compiler, the paper's evaluation is on Cedar 4.4, and the Rust crate this project reads terms from is not what is proved. The Lean statements in `cedar-spec` (`Cedar/Thm/SymbolicCompilation.lean`) match in shape. A person should confirm this reading.
