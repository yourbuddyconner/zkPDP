# E8: TinyTodo in GordieHowe

Status: planned. Written 2026-11-04.

## Question

Can the TinyTodo decision be stated in GordieHowe's language over the record commitment, and what does it cost against the Noir circuit from E2?

## Hypotheses

H10.

## Procedure

Strict mode.

1. Build GordieHowe at a pinned commit. Record the commit and any change needed.
2. Write the TinyTodo relation by hand: SHA-256 openings of two records, ancestor scans, the four policies, the journal as public inputs.
3. For every TinyTodo request, check it accepts the oracle's decision and rejects the opposite one.
4. Run the adversarial mutations from `bench/circuit/adversarial.py`, adapted.
5. Measure compile time, proving, proof size, verification, memory, per the benchmark protocol, and whether Lean accepts the program's certificate.

## Success and failure

- H10 supported if it proves in under 1 second on the laptop, with a proof under 1 MB, and Lean accepts the certificate.
- A failure to build on this machine, or a language that cannot state the relation, is a result.

## Results

None yet.
