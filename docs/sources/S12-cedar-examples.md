# S12: cedar-policy/cedar-examples

GitHub repository, Apache-2.0.
URL: https://github.com/cedar-policy/cedar-examples
Accessed 2026-11-04. Trust: primary.

## What it contains

- `tinytodo` (Rust and Python), `tinytodo-go`, hello-world apps in Java and Rust, a WASM example, a Java partial evaluation example.
- `cedar-policy-language-in-action`: workshop policies and schemas.
- `cedar-example-use-cases`: policies and schemas for two applications.
- `oopsla2024-benchmarks`: Google Drive, GitHub and tiny-todo policy sets, template variants, Python entity generators, a Rust harness, comparisons to OpenFGA and Rego. Parameters `--num-hierarchies`, `--num-requests`, `--num-entities`. Full run one to two hours.

## What we take

The corpus. See `corpus/README.md`. Generators are run with fixed seeds and
their output committed. Request generation in the benchmark appears randomized
and will be fixed by seed.
