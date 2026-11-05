# E8: TinyTodo in GordieHowe

Status: steps 1 to 5 done on 2026-11-04, except the Lean certificate. Written 2026-11-04.

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

Run on 2026-11-04 at commit 8c07ea9, GordieHowe 4837770 with `bench/gordiehowe/local.patch`. Apple M1 Max, ROM parameters, clang 20.1.6 where upstream pins 21. `results/E8-tinytodo-gordiehowe-2026-11-04.jsonl`. Three runs of 5 repeats per request on an idle machine.

### Step 1: build

Upstream documents x86-64 Linux with Nix only. It builds natively on this Mac with three small local changes. See `bench/gordiehowe/README.md`. Upstream's test suite was not built or run here.

### Steps 2 and 3: the relation

`bench/gordiehowe/tinytodo.cc` states the same logic and the same record commitment as the Noir circuit: canonical request hash, two record openings, ancestor scans, the four policies. 19 of 19 requests accept the oracle's decision.

### Step 5: cost, against the Noir circuit on the same machine

| Metric | GordieHowe | Noir with UltraHonk, zero-knowledge target |
|---|---|---|
| Proving, median | 36.7 ms | 1.93 s |
| Threads | 1, measured from real and user time | 10 |
| Verification, median | 13.4 ms | 16 ms |
| Proof | 116.6 KB | 14.7 KB |
| Program compile | 3.4 s per request shape | about 1 s, one circuit for all shapes |

These are not yet like for like, and no ratio should be printed:

- GordieHowe compiles one system per request shape, the six string lengths, so its strings are fixed-length. The Noir circuit handles every length up to its limits, at a cost in gates. TinyTodo's 19 requests need 4 shapes.
- GordieHowe's time is in-process with a loaded system. Noir's includes process start and loading the proving data.
- GordieHowe's peak memory was 2.4 GB for the whole run with four compiled systems held. That is not a per-proof figure and is not comparable to Noir's 0.78 GB.
- GordieHowe was built with its test switches compiled in, off at run time, and with clang 20.
- The browser and phones are not measured for GordieHowe.

### What is not done

- **The Lean certificate.** Lean and its Mathlib dependency are not installed here. So the program's certificate has not been exported or checked. Without it, the machine-checked guarantee does not yet apply to this program. It is also conditional on the source program being complete and round-by-round knowledge sound, which nobody has shown for TinyTodo.
- **A compiler.** This relation is written by hand.

### H10

Partly supported. Proving and proof-size thresholds are met. The Lean condition is not tested.
