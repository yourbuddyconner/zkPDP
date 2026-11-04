# Hypotheses

Written 2026-11-04, before any experiment ran. Thresholds are guesses informed
by the sources in `sources/`. A changed threshold needs a decision record.
Status values: `open`, `supported`, `refuted`, `inconclusive`.

"TinyTodo-sized" means the TinyTodo policy set from the Cedar examples with an
entity store of about 50 entities and one request. "Laptop" means the machine
named in `benchmark-protocol.md`.

| ID | Hypothesis | Threshold | Tested by | Status |
|---|---|---|---|---|
| H1 | The unmodified Cedar evaluator runs in a RISC Zero guest and proves a TinyTodo-sized decision on a laptop CPU in practical time. | Under 50 million cycles. Composite receipt under 5 minutes. Succinct receipt under 15 minutes. | E1 | open |
| H1b | In the zkVM backend, deserialization and transitive closure cost more cycles than evaluation. | Evaluation under 25 percent of total cycles. | E1 | open |
| H2 | A hand-written circuit for TinyTodo is small enough to prove client-side. | Under 1 million gates. Proves in under 10 seconds in a laptop browser and under 5 seconds natively on a phone. Gate B uses the gate count. | E2 | open |
| H3 | Verifying one signature over the inputs costs more than evaluating the policy, in both backends. | Signature cost at least 2 times evaluation cost, measured in cycles for the zkVM backend and gates for the circuit backend. | E3 | open |
| H4 | Most real policies fall in the subset the circuit compiler supports. | At least 80 percent of policy sets in the corpus compile with no construct unsupported. | E4 | open |
| H5 | A compilation proof is cheap enough to run on every policy change. | Under 1 hour on a server CPU for the largest corpus policy set. | E5 | open |
| H6 | The proof-carrying response is backward compatible. | An unmodified open-source AuthZEN PEP accepts it with no change in behavior. | E7 | open |
| H7 | The `reasons` field leaks attribute values. | At least one corpus policy set where the fired policy IDs determine an attribute value. Constructive. | E6 | open |
| H8 | Verification is cheap enough for a constrained verifier. | Under 50 milliseconds on the laptop for every format except the raw composite receipt. | E1, E2 | open |
| H9 | The zkVM backend beats the circuit backend on policy-change cost, and the circuit backend beats the zkVM backend on client-side proving time, on the same policy set. | Both directions hold on TinyTodo. | E6 | open |

Hypotheses that were considered and not registered, with the reason:

- "RISC Zero is cheaper than SP1" or the reverse. Vendor benchmarks disagree and we have one data point planned. Not a claim this paper should make.
- "The zkVM backend is zero-knowledge." The open advisory says the STARK is not provably so. We report, not hypothesize.
