# S13: cedar-policy crate 4.14.1 features

docs.rs feature list for `cedar-policy` 4.14.1.
URL: https://docs.rs/crate/cedar-policy/latest/features
Accessed 2026-11-04. Trust: primary.

## What it says

Default features: `datetime`, `decimal`, `ipaddr`. Experimental features
include `protobufs` (prost 0.14), `tpe` (type-aware partial evaluation),
`partial-eval`, `entity-manifest`, `extended-schema`. `wasm` enables
`wasm-bindgen` and `tsify`.

## What we take

- E1 step 0 starts with default features.
- `protobufs` is the candidate input form if parsing dominates (E1 step 7).
- `entity-manifest` may compute which entities a request needs, useful for mode 1 inclusion paths. Note for E3.
