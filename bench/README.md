# bench

Empty until Phase 1. Planned layout:

```
bench/
  envelope/      the shared AuthZEN envelope, journal types, canonicalization, verifier
  zkvm/          Cedar in a zkVM: RISC Zero host and guest
  circuit/       Cedar compiled to a circuit: hand-written Noir in Phase 2, the compiler in Phase 4
  harness/       runs the corpus through a backend, writes results/
```

The envelope crate has no dependency on either backend. Both backends produce
the same journal and the same proof object, with a different `format` string.
