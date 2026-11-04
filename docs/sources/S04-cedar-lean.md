# S04: Cedar's Lean verification

Lean FRO use-case page and AWS Open Source Blog "Lean into verified software
development".
URLs: https://lean-lang.org/use-cases/cedar/ and
https://aws.amazon.com/blogs/opensource/lean-into-verified-software-development
Accessed 2026-11-04. Trust: primary for claims about AWS's own process.

## What they say

- Cedar's authorization engine and validator are modeled in Lean. Safety and security properties are proved about the models.
- Millions of differential tests compare the Rust implementation to the Lean model.
- Lean evaluates a differential test input in about 5 microseconds, Rust in about 7.
- The validator soundness proof: if the validator accepts a policy, evaluation cannot raise a type error. About 18 person-days.
- No release ships unless model, proofs and differential tests are current.

## What we take

- The trust story for the zkVM backend: a verified evaluator, unmodified.
- The method for the circuit backend: differential testing against the reference, which is how Cedar tests itself.
- The validator guarantee that lets the circuit encoding drop error semantics for validated policies on schema-conforming entities.
