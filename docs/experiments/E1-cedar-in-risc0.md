# E1: Cedar in RISC Zero

Status: steps 0 to 3 done on 2026-11-04. Steps 4 to 7, proving, deferred by the human the same day. Written 2026-11-04.

## Question

Does the unmodified Cedar evaluator run inside a RISC Zero guest, and what does
one decision cost in cycles, proving time, proof size and verification time?

## Hypotheses

H1, H1b, H8.

## Setup

- RISC Zero toolchain through `rzup`, version recorded in the result file. `rzup` is already installed on the laptop.
- `cedar-policy` crate at the latest 4.x at the time of the run, version pinned in `Cargo.lock`.
- Corpus: TinyTodo for the main measurements. Synthetic series for scaling.
- Machine: the laptop.

## Procedure

Strict mode.

### Step 0: does it build

1. Create a guest crate that depends on `cedar-policy` with default features.
2. Build for the guest target.
3. If the build fails, record the error in the notebook. Turn off features one at a time until it builds. Record the final feature set in the result file and in a decision record if any feature is lost.
4. If it does not build with any feature set, stop. Record the cause. Open the SP1 question in `open-questions.md`.

### Step 1: the guest

1. Read from the host: policy set text, schema text, entities JSON, request JSON, nonce.
2. Record the cycle count.
3. Parse the policy set. Record the cycle count.
4. Parse the schema and entities with transitive closure. Record the cycle count.
5. Parse the request. Run the authorizer. Record the cycle count.
6. Compute the journal hashes with the SHA-256 precompile. Record the cycle count.
7. Commit the journal as defined in `docs/protocol.md`.

### Step 2: correctness

1. For every TinyTodo request in the corpus, run the guest in the executor without proving.
2. Compare the journal decision to the oracle. Stop on any mismatch.

### Step 3: cycles

1. Run the executor for each request. Record total cycles and the per-segment counts from step 1.
2. Compute the share of evaluation in the total. This tests H1b.

### Step 4: proving

1. Prove one TinyTodo request as a composite receipt. Record time and peak memory. Repeat 5 times.
2. Produce a succinct receipt. Record time and peak memory. Repeat 5 times.
3. Produce a Groth16 receipt. Record time and peak memory. Repeat 5 times. If the wrap needs hardware the laptop lacks, record that and skip.
4. Record proof size for each kind.

### Step 5: verification

1. Verify each receipt kind 20 times. Record the median. This tests H8.

### Step 6: scaling

1. Run steps 3 and 4 with 3 repeats for each point of the synthetic series.

### Step 7: input form

1. If H1b holds, repeat steps 3 and 4 with the entities passed in the experimental protobuf form, if the feature builds. Record the difference.

## Measurements

Cycles by segment. Execution time. Proving time by receipt kind. Peak memory.
Proof size. Verification time. All as result lines per `results/README.md`.

## Success and failure

- H1 supported if total cycles are under 50 million and the composite receipt proves in under 5 minutes.
- H1 refuted if either bound is exceeded by more than 10 times. See gate A in `docs/plan.md`.
- Step 0 failing is a result. It is recorded and reported.

## Results

Run on 2026-11-04 at commit c9528ee. Apple M1 Max, 10 cores, 64 GB, macOS 27.2.
RISC Zero 3.0.6, Rust toolchain r0.1.97.0, `cedar-policy` 4.14.1.

### Step 0: build

`cedar-policy` 4.14.1 builds for the guest with every default feature, after
one dependency is replaced. `stacker` needs `mmap` and `mprotect`. Decision
0015 swaps it for a stand-in. Cedar's own source is unchanged.

### Step 2: correctness

83 of 83 corpus requests give the oracle's decision and the oracle's set of
determining policies. The journal's policy, entity and request hashes equal
the manifest's. `results/E1-exec-2026-11-04.jsonl`.

### Step 3: cycles

TinyTodo, 4 policies, 43 entities, median of 19 requests: 23.3 million cycles.

| Segment | Share |
|---|---|
| Parse the schema | 50.3% |
| Parse the entities, with transitive closure | 40.3% |
| Parse the policy set | 8.6% |
| Map the request | 0.2% |
| Authorize | 0.3% |
| Hash and commit the journal | 0.2% |
| Read the input | 0.1% |

Scaling, from the synthetic series, cycles by executor:

| Variable | Points | Total cycles |
|---|---|---|
| Entities | 11, 100, 1,000, 10,000 | 15.7 M, 34.1 M, 228 M, 2,172 M |
| Policies | 5, 14, 34, 104 | 23.4 M, 24.8 M, 27.8 M, 38.3 M |
| Hierarchy depth | 1, 2, 4, 8 | 23.2 M, 23.9 M, 25.7 M, 29.7 M |
| String length | 8, 32, 128, 256 | 23.4 M, 23.4 M, 23.6 M, 23.7 M |

Entity parsing costs about 215 thousand cycles per entity and grows linearly.
A policy costs about 150 thousand cycles to parse. String length and a `like`
policy cost almost nothing. Authorization never exceeds 5.5 percent.

### Variant: no schema in the guest

`results/E1-exec-noschema-2026-11-04.jsonl`. TinyTodo falls to 11.7 million
cycles. The authority would validate entities against the schema before it
commits to them. Four `streaming_service` requests fail in this variant: their
context holds datetime values, which Cedar parses only with a schema.

### Steps 4 to 7: proving

Not measured. What is known:

- One composite proof of a TinyTodo decision on the prebuilt `r0vm`, which uses CPU cores only here, was stopped unfinished after 28 minutes at about 8 busy cores and 10 GB. No result line was written, so this is recorded here and in the notebook, not in `results/`.
- A proof of the smallest set, 7.9 million cycles, was stopped unfinished after about 12 minutes on the same prover while builds competed for the cores.
- No GPU proving is available on Apple silicon with RISC Zero 3.0.6. The Metal branch of the segment prover is commented out in the release (upstream issue 3753). Decision 0017 is withdrawn.
- RISC Zero `main` at 164c64b re-enables Metal (upstream PR 3688). The harness built against it after fetching Git LFS archives by hand and one API change. A proof of the 7.9 million cycle set did not finish in 300 seconds, and the human's GPU monitor showed only brief small spikes. Not pursued further.

### What this says

The cost of this backend is parsing, not policy evaluation. A decision over
TinyTodo spends 99 percent of its cycles turning text into Cedar's data
structures. The levers, in order of expected gain:

1. Give the guest only the entities the request can reach, proved by inclusion in the entity commitment. This is E3's mode 1, and it would cut the largest segment from 43 entities to about 4.
2. Leave the schema out of the guest where the context has no extension values.
3. Pass entities and policies in a pre-parsed binary form, such as Cedar's experimental protobuf encoding.

It also strengthens the case for the circuit backend, which parses nothing.

## Later results

### First finished proof, 2026-11-05

Commit dabc4ca, tree clean. TinyTodo without schema in the guest, 11.7 million cycles, one request. `results/E1-prove-noschema-2026-11-05.jsonl`.

| Metric | Value |
|---|---|
| Composite proof, one run | 1,711 s, 28.5 minutes |
| Peak memory | 9.5 GB |
| Receipt | 4.2 MB |
| Verification, median of 20 | 298 ms |

About 6.8 kHz on this CPU prover. For the last few minutes a low-priority `brew install`, and some reading commands, ran alongside. One run, so no spread. At this rate the stopped 23.3 million cycle proof needed about an hour, so it was not close to finishing at 28 minutes, and the assistant's suggestion that it might be was wrong. S27's 21 kHz on an M4 Pro did not carry over to this machine.

### E1b: the guest reads the same two records as the circuits

`bench/zkvm/methods/guest/src/bin/records.rs`. The guest recomputes both leaves from the entities' JSON and checks their paths to the root, then gives Cedar only those two entities, with closed ancestors as parents. 19 of 19 TinyTodo requests match the oracle's decision and determining policies. `results/E1b-exec-records-2026-11-05.jsonl`.

Median 5.38 million cycles: Cedar building its entities 3.08 million, parsing the policy set 2.00 million, opening the records 0.13 million, mapping the request 0.07 million, authorizing 0.07 million.

Differences that remain from the circuits, by design: the guest hashes the schema without parsing it, so it does not refuse a request outside the schema, and Cedar denies it. It handles any string length. A policy that dereferences a third entity would need that entity opened too, and this guest opens two.

### E1b: proving, the first like-for-like number

Commit cdca793, tree clean. One TinyTodo deny, `l0-stranger-gets`, three runs. `results/E1b-prove-records-2026-11-05.jsonl`. A low-priority Lean toolchain download ran alongside.

| Metric | Value |
|---|---|
| Composite proof | 779 s median, 775 to 783 |
| Peak memory | 11.5 GB |
| Receipt | 2.24 MB |
| Verification, median of 20 | 125 ms |

Succinct and Groth16 receipts are not measured. Gate A: the 5 minute composite bound in H1 fails by less than ten times, so the zkVM backend stays as the server-side and audit backend, and the client-side claim rests on the circuit backends.
