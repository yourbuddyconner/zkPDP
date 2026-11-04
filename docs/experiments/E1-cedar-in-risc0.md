# E1: Cedar in RISC Zero

Status: planned. Written 2026-11-04.

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

None yet.
