# 0022: GordieHowe as a third backend

Status: accepted. Date: 2026-11-04.
Proposed by: assistant.
Accepted by: human. The assistant had recommended reading more first. The human chose to proceed.

## Context

The review and S26 found GordieHowe: a language, a certifying compiler and a proof system whose knowledge soundness and zero knowledge are machine-checked in Lean, and whose compiler emits a per-program Lean certificate. Noir with UltraHonk has no published zero-knowledge proof that we found, had a critical flaw before v5.0.0, and calls itself not recommended for production.

## Decision

Add GordieHowe as a third backend. It has the same inputs by hash, the same journal, and its own `format` string. Experiment E8 tests it, starting with TinyTodo written by hand, as E2 did for Noir. The compiler target decision waits for E8.

## Reasoning

It is the only option we found that answers "is the proof system zero-knowledge" with machine-checked theorems. Its per-program certificates also answer "is the output faithful", which the compilation proof (0009) does not.

## Consequences

- The comparison becomes three-way. The equal effort rule (0013) extends to it.
- Its guarantees are conditional: the source program must be complete and round-by-round knowledge sound, and the assumptions in S26 remain. "Machine-checked" must not be printed without them.
- Its verifier is not sublinear and its proofs are hundreds of kilobytes, so it may lose on proof size to UltraHonk.
- Most of its 156 pages are unread. If E8 finds it does not fit, this decision gets reversed by a new record.

## Sources

S26, S25, the literature review.

## Update 2026-11-04

The human allowed work on GordieHowe, on this machine only. Upstream documents x86-64 Linux with Nix, and a native build works with small local patches. See E8.
