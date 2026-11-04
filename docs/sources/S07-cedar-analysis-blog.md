# S07: Introducing Cedar Analysis

AWS Open Source Blog.
URL: https://aws.amazon.com/blogs/opensource/introducing-cedar-analysis-open-source-tools-for-verifying-authorization-policies
Accessed 2026-11-04. Trust: secondary (vendor blog, read through a search summary).

## What it says

The symbolic compiler is implemented in Lean. Formulas are analyzed by SMT
solvers such as cvc5. Checks include always-allow, always-deny, implication,
equivalence, and never-error, with counterexamples.

## What we take

Context for S05.
