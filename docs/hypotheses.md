# Hypotheses

Written 2026-11-04, before any experiment ran. Thresholds are guesses informed
by the sources in `sources/`. A changed threshold needs a decision record.
Status values: `open`, `supported`, `refuted`, `inconclusive`.

"TinyTodo-sized" means the TinyTodo policy set from the Cedar examples with an
entity store of about 50 entities and one request. "Laptop" means the machine
named in `benchmark-protocol.md`.

| ID | Hypothesis | Threshold | Tested by | Status |
|---|---|---|---|---|
| H1 | The unmodified Cedar evaluator runs in a RISC Zero guest and proves a TinyTodo-sized decision on a laptop CPU in practical time. | Under 50 million cycles. Composite receipt under 5 minutes. Succinct receipt under 15 minutes. | E1 | cycles supported, time refuted. Cycles: supported, 23.3 million with schema, 11.7 million without. Time: refuted on this laptop's CPU prover. 28.5 minutes at 11.7 million cycles, and 13 minutes at 5.4 million with the guest reading two records. No GPU proving on Apple silicon in RISC Zero 3.0.6. |
| H1b | In the zkVM backend, deserialization and transitive closure cost more cycles than evaluation. | Evaluation under 25 percent of total cycles. | E1 | supported. Evaluation is 0.3 percent on TinyTodo and at most 5.5 percent on any corpus set. `results/E1-exec-2026-11-04.jsonl`. |
| H2 | A hand-written circuit for TinyTodo is small enough to prove client-side. | Under 1 million gates. Proves in under 10 seconds in a laptop browser and under 5 seconds natively on a phone. Gate B uses the gate count. | E2 | partly supported. 353,518 gates at first, 388,254 after the commitment change. Native laptop proof 2.2 seconds, 0.78 GB. Laptop browser 4.3 seconds. Phone not yet measured. Gate B passes. `results/E2-tinytodo-2026-11-04.jsonl`, `results/E2-E4-browser-2026-11-04.jsonl`. |
| H3 | Verifying one signature over the inputs costs more than evaluating the policy, in both backends. | Signature cost at least 2 times evaluation cost, measured in cycles for the zkVM backend and gates for the circuit backend. | E3 | supported, with P-256 and not Ed25519: 2.9 times in the zkVM with its precompile, far more in Noir. GordieHowe not yet measured. |
| H4 | Most real policies fall in the subset the circuit compiler supports. | At least 80 percent of policy sets in the corpus compile with no construct unsupported. | E4 | refuted at this version. With request context, 5 of the 7 distinct upstream policy sets compile, 71 percent. Earlier: 3 of 7. By term, 95.5 percent of the upstream sets' terms translate. The request context is the main blocker. `results/E4-coverage-2026-11-04.jsonl`. |
| H5 | A compilation proof is cheap enough to run on every policy change. | Under 1 hour on a server CPU for the largest corpus policy set. | E5 | open |
| H6 | The proof-carrying response is backward compatible. | An unmodified open-source AuthZEN PEP accepts it with no change in behavior. | E7 | open |
| H7 | The `reasons` field leaks attribute values. | At least one corpus policy set where the fired policy IDs determine an attribute value. Constructive. | E6 | open |
| H8 | Verification is cheap enough for a constrained verifier. | Under 50 milliseconds on the laptop for every format except the raw composite receipt. | E1, E2 | supported for the circuit backend: 20 milliseconds, process start included. Not measured for the zkVM backend. |
| H9 | The zkVM backend beats the circuit backend on policy-change cost, and the circuit backend beats the zkVM backend on client-side proving time, on the same policy set. | Both directions hold on TinyTodo. | E6 | open |
| H10 | GordieHowe proves the TinyTodo decision faster than UltraHonk, with a Lean-checked certificate for the program. | Under 1 second on the laptop, proof under 1 MB, Lean accepts the certificate. | E8 | partly supported. 36.7 ms and 116.6 KB. The certificate is not tested, and strings are fixed per request shape. |
| H11 | Generated circuits reject adversarial witnesses. | No mutation that changes the proven decision, the opened facts or the bound request is accepted, across every compiled upstream set. | E9 | supported within limits: single-value attacks, 3,216 at input level and 546 at witness level, and 236 on GordieHowe. One malleability gap was found and fixed. Not a proof of soundness. |

Hypotheses that were considered and not registered, with the reason:

- "RISC Zero is cheaper than SP1" or the reverse. Vendor benchmarks disagree and we have one data point planned. Not a claim this paper should make.
- "The zkVM backend is zero-knowledge." The open advisory says the STARK is not provably so. We report, not hypothesize.

Note on H6, 2026-11-04: the threshold is unchanged. AuthZEN section 5.5 allows a PEP to reject, so a supported result can only say "on these named PEPs".
