# E9: adversarial witnesses

Status: done for single-value attacks on 2026-11-04. Written after the first run, at the human's request, so the procedure below describes what ran.

## Question

Differential tests use honest witnesses, so they cannot see a missing constraint. Do the circuits reject a dishonest prover?

## Hypotheses

H11.

## Procedure

Part A, `bench/web/adversarial.mjs`. Each attack changes the inputs of an honest request: flipped or forged attribute and ancestor slots, a valid opening of a different entity, zeroed entities, corrupted paths, changed request strings, hashes and padding. The solver then runs twice, with the oracle's decision and with the opposite. The opposite must never solve. Up to 12 requests per set, permits and denies balanced.

Part B, `bench/circuit/adversarial_witness.py`. One witness value is increased by 1, then `bb prove` and `bb verify` run. Sampled: public inputs, Brillig outputs, private inputs, internal values. `bb check` is not implemented in this CLI. `classify_witness.py` explains every accepted value, and `check_overwritten.py` tests whether a change reached the proof at all.

GordieHowe, in `bench/gordiehowe/tinytodo.cc`. Proves regardless with forged facts or the opposite decision, then verifies. A control sends an honest witness down the same path.

## Results

`results/E9-input-attacks-2026-11-04.jsonl`, `results/E9-witness-attacks-2026-11-04.jsonl`, `results/E9-witness-classified-2026-11-04.json`, `results/E9-witness-overwritten-2026-11-04.json`, and the E9 lines in `results/E8-tinytodo-gordiehowe-2026-11-04.jsonl`.

### Part A, generated Noir circuits

| Set | Requests | Attacks | Opposite decision accepted | Same decision accepted |
|---|---|---|---|---|
| tinytodo | 12 | 1,382 | 0 | 234 |
| github_example | 7 | 1,066 | 0 | 323 |
| hotel_chains | 6 | 768 | 0 | 54 |

"Same decision accepted" means the changed value did not alter the public inputs or the decision. These are slots the selected function does not read, or changes that had no effect, such as emptying an already empty set.

**A real gap found and fixed.** The first run accepted nonzero padding after a string's length, including in the public nonce, so one request had many encodings of its public inputs. The decision could not be changed by it. `assert_plain` now requires zero padding in generated and hand circuits. Generated circuits grew by about 620 gates: tinytodo to 454,134, github_example to 632,028, hotel_chains to 858,806. The hand circuit is now 388,686. The table above is from after the fix.

### Part B, witness level, generated Noir circuits

546 single-value changes across 6 requests. No public input change was accepted.

| Accepted | Count | Explanation |
|---|---|---|
| Inverse hint of an is-zero check whose operand is 0 | 70 | The constraints force the result for any inverse, so the value is free and harmless. One of these needed reading by hand, because its operand has a constant offset |
| SHA compression output | 1 | See below |

Harder-looking result, then settled. Changing a SHA compression output was accepted in most tries of a targeted test. Plain-mode proofs are deterministic, so each tampered proof was compared with the honest one. Of 16, 9 were byte-identical and 7 were rejected. None differed and verified. So Barretenberg recomputes these values and the file's value does not reach the proof. This explanation comes from the bytes. The Barretenberg source was not read to confirm it. In the same test, of 16 Brillig outputs, 15 were rejected and 1 gave a different proof that verified, and that one's honest value was 0, consistent with the is-zero pattern.

### GordieHowe, hand-written TinyTodo

236 attacks in each of 3 runs. 19 honest controls verified each run. 0 attacks accepted. Every rejection came from the verifier and none from a prover crash.

## Limits

- One change at a time, plus the forged-fact attacks. A bug that needs several values changed together would not show. This is evidence, not a proof of soundness.
- Part A runs the solver honestly, so a malicious helper output is only exercised in part B.
- Values Barretenberg recomputes cannot be attacked from a witness file. A bug in that recomputation is out of reach here, and LIEZZ reports a Noir bug in that lowering layer.
- Three upstream sets, and sampled witnesses: 546 of hundreds of thousands.
- The SHA-256 library's soundness warning is not resolved by this. PR 62 of that library only changes the partial API we do not call, apart from removing a `mut`.

## H11

Supported within these limits.

## Update 2026-11-05: under the final design

Fixed-layout hashes, decision 0026. Three generated circuits, tinytodo, github_example and sales_orgs. `results/E9-witness-attacks-v1-2026-11-05.jsonl`, `results/E9-witness-classified-v1-2026-11-05.json`, `results/E9-witness-overwritten-v1-2026-11-05.json`, `results/E9-input-attacks-v1-2026-11-05.jsonl`.

- 366 single-value witness changes. No public input change was accepted. 68 were accepted without changing any public input.
- `classify_witness.py` now checks each accepted value against explicit patterns, and leaves none unexplained: 60 are the is-zero inverse hint with a zero operand, 2 the same hint after a constant offset, and 6 are SHA compression outputs.
- For SHA outputs, of 16 targeted changes, 11 gave a byte-identical deterministic proof and 5 were rejected. None gave a different proof that verified. Of 16 Brillig outputs beside an assertion, 15 were rejected and 1 gave a different proof that verified, with an honest value of 0, which fits the is-zero pattern.
- 3,880 input attacks on all five circuits, none accepting the opposite decision.

The limits above still apply: one change at a time, and a sample.
