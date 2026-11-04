# E2: TinyTodo as a hand-written Noir circuit

Status: planned. Written 2026-11-04.

## Question

What does one Cedar policy set look like as a circuit, how many gates does it
take, and how long does it prove on a laptop, in a browser and on a phone?

## Hypotheses

H2, H8.

## Setup

- Noir toolchain through `noirup`, Barretenberg through `bbup`. Neither is installed yet. Versions recorded in the result file.
- Corpus: TinyTodo policy set, schema, one entity store, the request list.
- Machines: the laptop, natively and in the browser. One phone, natively through a mobile wrapper if available, and in the browser.

## Procedure

Strict mode.

### Step 1: design the layout

1. List every Cedar construct TinyTodo uses: attribute reads, `in`, `has`, `==`, `&&`, `||`, `like`, sets, extension types.
2. For each construct, write the circuit encoding following `docs/spec.md` section 7.3. Record each choice in the notebook.
3. Fix the maximum string length and set size from the TinyTodo schema and entity store. Record them.
4. Define the private inputs: subject attributes, resource attributes, Merkle inclusion paths for every `in`.
5. Define the public inputs: the journal fields in the order given in `docs/protocol.md`.

### Step 2: write the circuit

1. One Noir function per policy, returning a boolean.
2. One authorizer function: permit if any permit is true and no forbid is true.
3. Request hash, policy hash and entity root as public inputs. The circuit checks Merkle paths against the root and hashes the attributes that the request hash covers.

### Step 3: correctness

1. For every TinyTodo request in the corpus, generate the witness and read the decision.
2. Compare to the oracle. Stop on any mismatch.

### Step 4: size

1. Record ACIR gate count and backend gate count.

### Step 5: proving

1. Prove natively on the laptop 5 times. Plain mode and zero-knowledge mode. Record time and peak memory.
2. Prove in the laptop browser 5 times. Record time and peak memory. Record witness generation separately.
3. Prove on the phone natively 5 times if the wrapper is available. Record time and peak memory. Record an out-of-memory failure as a result.
4. Prove in the phone browser 5 times. Record as above.
5. Record proof size.

### Step 6: verification

1. Verify 20 times natively. Record the median.

## Measurements

Gates. Witness time. Proving time by machine and mode. Peak memory. Proof size.
Verification time.

## Success and failure

- H2 supported if gates are under 1 million and the laptop browser proves in under 10 seconds.
- Gate B in `docs/plan.md` reads the gate count.
- A phone out-of-memory is reported, not hidden.

## Results

None yet.
