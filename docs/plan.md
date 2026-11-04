# Research plan

Date: 2026-11-04. Phase lengths are guesses that assume part-time work by one
person with an AI assistant. They are here so a reader can see later how wrong
they were. Hours are logged in the notebook.

## Shape of the work

Seven experiments, in an order chosen so each one settles the question the next
one depends on. Three decision gates where the plan can shrink. The paper is
written last, from the hypotheses file and the result files.

```
Phase 0  Foundations ─────────────────────────────┐
Phase 1  E1  Cedar in RISC Zero ──────────── gate A ┤
Phase 2  E2  One policy set by hand in Noir ─ gate B ┤
Phase 3  E3  Input provenance, both backends        │
Phase 4  E4  Term IR to Noir compiler, coverage ─ gate C
Phase 5  E5  Compilation proof in the zkVM
Phase 6  E6  Full comparison and privacy analysis
         E7  End-to-end AuthZEN demonstration
Phase 7  Paper and artifact
```

## Phase 0: foundations

Goal: everything later phases share, so they never argue about inputs.

- Assemble the corpus. TinyTodo, the two example use cases, the three OOPSLA 2024 applications with fixed seeds, the synthetic series. Commit outputs and a manifest with hashes.
- For every request in the corpus, record the native Cedar decision. This is the oracle every backend must match.
- Write the envelope crate: journal types, JCS, Cedar formatter hashing, Merkle commitment, verifier skeleton with one test per verification step.
- Literature pass. Read Prezta in full. Read the Cedar OOPSLA paper and the symbolic compiler docs. Update source notes from `unverified` to `primary` where read. Fill `paper/related-work.md`.
- Settle the open questions marked "for the human researcher" in `docs/open-questions.md`.

Exit: corpus committed with manifest. Envelope crate passes its tests. Every source note read in full has a trust level. Estimate: one to two weeks.

## Phase 1: E1, Cedar in RISC Zero

Goal: the baseline numbers and the first answer to "is this practical".

Steps are in `experiments/E1-cedar-in-risc0.md`. In short: build the Cedar crate for the guest, write the thin shell, measure cycles by segment, prove on the laptop in three receipt kinds, verify, run the scaling series.

**Gate A.** If the TinyTodo set proves in under the H1 threshold on the laptop, the zkVM backend is a candidate for client-side use and the plan continues as written. If it is over by less than ten times, the zkVM backend stays as the server-side and audit backend and the client-side claim rests on the circuit backend. If it is over by more than ten times, record why, and consider SP1 before continuing.

Estimate: two weeks.

## Phase 2: E2, one policy set by hand in Noir

Goal: learn what a Cedar policy set looks like as a circuit before writing a compiler for it.

TinyTodo, translated by hand following the rules in `spec.md` section 7.3. Public inputs are the journal. Measure gates, witness generation, proving time native and in the browser, memory, verification.

**Gate B.** If the circuit is under the H2 gate threshold, the compiler is worth writing and Phase 4 proceeds. If it is over by less than ten times, Phase 4 proceeds with a reduced scope: only the policy constructs TinyTodo used. If it is over by more than ten times, Phase 4 is cancelled. The paper then reports the circuit backend as a measured negative result with the reasons.

Estimate: two weeks. Noir toolchain install is part of this.

## Phase 3: E3, input provenance

Goal: the cost that Prezta found dominant, measured in both backends.

Mode 1, Merkle inclusion, in both backends. Mode 2, one Ed25519 signature over the entity data, in both backends. Mode 3 is mode 2 with `reasons` off and the private-input analysis from E6.

Estimate: two weeks.

## Phase 4: E4, the compiler and coverage

Goal: a Term IR to Noir translation and the fraction of the corpus it handles.

Only if gate B passed. Coverage is measured over the whole corpus by construct: which Cedar operators and types the translation handles, and which policy sets fall entirely inside that set. The number goes in the paper either way.

**Gate C.** Decides whether E5 happens. If the compiler handles under half the corpus, E5 is dropped and the paper argues composition on paper only.

Estimate: four weeks. This is the largest piece of new engineering.

## Phase 5: E5, compilation proof

Goal: run the compiler inside RISC Zero and measure the proof that policy hash H compiled to verification key K.

Estimate: one week if the compiler is plain Rust.

## Phase 6: E6 and E7

E6 fills the comparison table in `benchmark-protocol.md` and writes the privacy analysis: for each backend and mode, is the proof zero-knowledge, and what does the journal leak. Includes the constructive demonstration of the `reasons` leak on a corpus policy set.

E7 runs an off-the-shelf AuthZEN PEP against a proof-producing PDP and shows it accepts the response unchanged, then runs a modified PEP that verifies.

Estimate: two weeks.

## Phase 7: paper and artifact

Rewrite the abstract from results. Every claim links to an experiment, result file or source. Package the repository as an artifact: pinned toolchains, one script per experiment, the corpus manifest.

Estimate: three weeks.

## Milestones

| Milestone | What exists |
|---|---|
| M0 | Corpus with manifest, envelope crate, source notes read |
| M1 | First verifying proof of a Cedar decision from the zkVM backend |
| M2 | First verifying proof from a hand-written circuit |
| M3 | Both backends with input provenance |
| M4 | Compiler coverage number over the corpus |
| M5 | Compilation proof verifying |
| M6 | Comparison table filled, privacy section drafted, demo recorded |
| M7 | Paper draft with every claim linked |

## Rules that hold across phases

- Equal effort: hours per backend logged. Optimization time is time-boxed the same for both. Decision 0013.
- Pre-registration: hypotheses and thresholds are fixed before a phase starts. Changes are decision records.
- Negative results are kept and reported.
- The native Cedar evaluator is the oracle for correctness. Every proof's decision is compared to it before timing counts.
