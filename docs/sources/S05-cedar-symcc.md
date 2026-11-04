# S05: cedar-policy-symcc

docs.rs page for `cedar-policy-symcc` version 0.8.1, Apache-2.0.
URL: https://docs.rs/cedar-policy-symcc
Accessed 2026-11-04. Trust: primary.

## What it says

- Public modules include `term`, `term_type`, `term_factory`, `bitvec`, `ext`, `extension_types`, `op`, `solver`, `solver_pool`.
- `term` is "a simply typed IR to which we reduce Cedar expressions during symbolic compilation".
- Compile API: `CedarSymCompiler::new(solver)`, `SymEnv::new(&schema, &req_env)`, `WellTypedPolicies::from_policies(...)` (marked deprecated in this version), checks such as `check_always_allows`, `check_never_errors`, `check_implies`, `check_equivalent`, with counterexample variants. `CompiledPolicy`, `CompiledPolicySet`, `CompiledSchema` are reusable intermediates. `*_asserts` functions produce assertions without a solver.
- Solving needs cvc5 1.3.1 through the `CVC5` environment variable. Compilation to terms does not.
- The compiler and verifiers are modeled and verified in Lean.

## What we take

- The front end for the circuit backend. Decision 0006.
- The `*_asserts` path and `CompiledPolicySet` as likely entry points to the Term IR without a solver.

## Open items

- Confirm the Term IR is reachable from a downstream crate and stable across minor versions. E4 step 1.
- An earlier crate listing showed version 0.1.3 dated 2025-11-10 and 2025-12-12. Version 0.8.1 is current. Rapid movement. Pin.
