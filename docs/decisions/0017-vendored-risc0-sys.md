# 0017: A vendored `risc0-sys` with a three-line Metal kernel fix

Status: withdrawn the same day. Date: 2026-11-04.
Proposed by: assistant. The human asked for any acceleration available.
Accepted by: not yet reviewed by the human.

## Context

The prebuilt `r0vm` 3.0.6 proves on CPU cores only on this laptop. The
in-process prover with the `metal` feature needs Xcode's Metal Toolchain,
which the human installed. With the toolchain of Xcode on macOS 27.2,
`risc0-sys` 1.5.0 fails to compile: three assignments of the literal `0` to an
`Fp` in `kernels/zkp/metal/poseidon2.metal`, lines 89, 93 and 186, are
ambiguous to the newer compiler.

## Decision

Vendor `risc0-sys` 1.5.0 at `bench/vendor/risc0-sys` with those three
assignments written as `Fp(0u)`, and patch it in through the workspace. Build
the GPU harness with `--features metal --target-dir target-metal`.

## Reasoning

The change makes an implicit conversion explicit and cannot change a value.
It touches only the prover. The guest, the image ID and receipt verification
are unaffected, so a wrong kernel could only produce a proof that fails to
verify.

## Consequences

- The Metal prover builds. It has not been timed. The human stopped proof generation for now on 2026-11-04.
- Any proving time measured with this build must say "risc0-sys 1.5.0 with a local kernel fix" in its result lines. The harness records `prover_build`.
- Drop the vendored copy when an upstream release compiles. Worth an issue upstream.

## Sources

Build log in the notebook for 2026-11-04.

## Withdrawn, 2026-11-04

The vendored fix made the build pass and bought nothing. RISC Zero 3.0.6 does
not use Metal for the main circuit on Apple silicon: in
`risc0-circuit-rv32im` 4.0.5, `segment_prover()` has the Metal branch
commented out and falls through to the CPU. The `metal` feature of
`risc0-zkvm` only implies `prove`. Upstream issue risc0/risc0#3753 reports
the same. The vendored copy and the `metal` feature of the harness were
removed. See the notebook for what else was tried.
