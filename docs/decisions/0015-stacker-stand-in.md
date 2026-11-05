# 0015: A stand-in for the `stacker` crate in the guest

Status: accepted, pending review. Date: 2026-11-04.
Proposed by: assistant, during E1 step 0.
Accepted by: not yet reviewed by the human. Made under "proceed with 0 and 1".

## Context

E1 step 0 asked whether `cedar-policy` 4.14.1 builds for the RISC Zero guest
target with default features. It does not. `cedar-policy-core` depends on
`stacker` 0.1.25, which needs `mmap`, `mprotect`, `munmap` and `sysconf`. The
target `riscv32im-risc0-zkvm-elf` has none. Every other dependency built.

Cedar calls `stacker` in two places, `evaluator.rs` and
`validator/typecheck.rs`, and only `stacker::remaining_stack()`. It is a guard
that turns very deep recursion into a Cedar error before the stack overflows.

## Options considered

- **Turn off Cedar features.** `stacker` is not optional. No feature removes it.
- **Fork `cedar-policy-core`** and delete the guard. Changes the evaluator's source, which weakens the "unmodified evaluator" claim.
- **Switch to SP1.** Its target would hit the same missing calls.
- **Replace `stacker` through `[patch.crates-io]`** with a crate of the same name whose `remaining_stack` returns `None`. Cedar's source is untouched. Cedar reads `None` as "enough stack".

## Decision

The last. `bench/zkvm/methods/stacker-stub`, patched in the guest only. The
host and the oracle use the real crate.

## Reasoning

Cedar's own source and version stay exactly as published. The behavior change
is confined to one case: a policy or entity nested so deep that the guard
would have fired. In the guest that case now exhausts the stack and aborts,
and an aborted guest yields no proof. It cannot yield a wrong decision.

## Consequences

- The paper's claim becomes "the unmodified Cedar crate, with one dependency replaced for the target", and says which.
- The guest's behavior differs from native Cedar on pathological depth: no proof instead of a deny with an error. The differential test does not cover this. Noted as a limitation.
- An upstream change that made `stacker` optional, or a `stacker` release with a fallback for unknown targets as it has for `wasm32`, would remove this record's reason. Worth an issue upstream.
- No Cedar feature had to be turned off. `datetime`, `decimal`, `ipaddr` all build.

## Sources

S13, S21. Build log in the notebook for 2026-11-04.
