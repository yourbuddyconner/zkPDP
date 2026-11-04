# S21: RISC Zero guest code 101

RISC Zero developer docs, version 3.0.
URL: https://dev.risczero.com/api/zkvm/guest-code-101
Accessed 2026-11-04. Trust: primary.

## What it says

- The template guest is `#![no_std]` for performance. The page does not say std is unsupported.
- `env::commit` and `env::commit_slice` write the journal. `env::cycle_count` reads cycles. `env::log` prints.
- A separate page on Rust std support returned 404 on 2026-11-04.

## What we take

- The journal and cycle count APIs for E1.

## Open items

- Confirm std support in the guest. The Cedar crate needs std. The assistant believes std is supported with no threads, filesystem or network, from memory of earlier RISC Zero releases. Not confirmed this session. E1 step 0 settles it.
