# Benchmark protocol

How the two backends are measured so the comparison means something.

## Rule of comparability

Both backends receive the same inputs by hash, produce the same journal, and
return the same proof object with a different `format`. A result that does not
reference corpus hashes is not a result.

## Rule of equal effort

Hours spent on each backend are logged per day in the notebook. Optimization
work on one backend is time-boxed, and the same box is offered to the other.
The paper reports the hours. If they differ by more than 25 percent, the paper
says so beside every comparison.

## Corpus

Described in `corpus/README.md`. Fixed, committed, hashed. The native Cedar
evaluator's decision for every request is the oracle. A backend's run counts
only if its decision matches the oracle.

## Machines

To be filled in Phase 0 with exact models. Planned:

- Laptop: the researcher's development machine. CPU proving, browser proving.
- Phone: one iOS and, if available, one Android device. Native proving through a mobile wrapper, and browser proving.
- Server: a machine with a GPU if available. If none is available, the paper says the server-side column is CPU only.

Each result line records the machine description and toolchain versions.

## Metrics

| Metric | Unit | zkVM backend | Circuit backend |
|---|---|---|---|
| Program size | cycles or gates | guest cycles, by segment | ACIR gates and backend gates |
| Execution | seconds | guest execution without proving | witness generation |
| Proving | seconds | composite, succinct, Groth16 wrap, each separately | UltraHonk, plain and zero-knowledge mode |
| Peak memory | GB | prover process | prover process |
| Proof size | bytes | per receipt kind | per mode |
| Verification | milliseconds | per receipt kind | per mode |
| Policy change | seconds, steps | edit to accepted proof: none beyond hashing | edit to accepted proof: compile, key generation, and the compilation proof if used |
| Coverage | percent of corpus | always 100 | measured in E4 |
| Trust base | list | Cedar crate, RISC Zero | symcc front end, our translation, Noir, Barretenberg |

## Procedure

Strict mode.

1. Check out the commit named in the experiment file.
2. Record machine and toolchain versions into the result file header.
3. For each input in the experiment's corpus list, run the backend once untimed and compare the decision to the oracle. Stop if any mismatch.
4. Run each timed measurement 5 times. Record every run. Report median and interquartile range.
5. Run verification 20 times per proof. Report median.
6. Write result lines as the run proceeds, not at the end, so an aborted run leaves its partial record.
7. Commit the result file in the same commit as the notebook entry that describes the run.

## Scaling series

From `corpus/synthetic/`. One variable at a time, others held at the TinyTodo
size: policy count, entity count, hierarchy depth, string length. Each point
follows the procedure above with 3 repeats instead of 5.

## What is not measured

- Network time. Both backends return the same bytes over the same channel.
- Prover cost in money. Vendor prices change. Cycles and gates are the stable unit.
- Anything on hardware not listed above.

## Comparison table for the paper

Filled by E6. One row per metric, one column per backend and mode, TinyTodo
size, with scaling plots in an appendix.
