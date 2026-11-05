# E2: TinyTodo as a hand-written Noir circuit

Status: steps 1 to 4, native laptop proving and verification done on 2026-11-04. Browser and phone not done. Written 2026-11-04.

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

Run on 2026-11-04 at commit aca6faf. Apple M1 Max, 10 cores, 64 GB, macOS 27.2.
Noir 1.0.0, Barretenberg 6.0.0, `noir-lang/sha256` v0.3.0. Circuit in
`bench/circuit/tinytodo`. `results/E2-tinytodo-2026-11-04.jsonl`.

### Correctness

19 of 19 TinyTodo requests: the circuit accepts the oracle's decision and
rejects the opposite decision. 10 permits and 9 denies.

### Size and time, one decision

| Metric | Zero-knowledge mode | Plain mode |
|---|---|---|
| Gates | 353,518 | same circuit |
| Witness generation | 0.33 s | same |
| Proving, median of 5 | 2.22 s | 2.19 s |
| Peak memory | 0.78 GB | 0.75 GB |
| Proof | 14,656 bytes | 13,120 bytes |
| Public inputs | 5,184 bytes | 5,184 bytes |
| Verification, median of 20 | 20 ms | 19 ms |
| Verification key, once per circuit | 6.0 s | 1.1 s |

Times include process start. Proving used all 10 cores.

### What the circuit is

- Public inputs are the journal: request hash, policy hash, schema hash, entity root, nonce, decision. 162 field elements, one per byte, which is why the public inputs are 5 kB.
- The request is private. The circuit rebuilds its canonical JSON by concatenation and checks SHA-256 against the request hash.
- The principal and the resource are opened from the entity root by Merkle path, depth up to 12.
- The four policies are four boolean expressions over attribute slots and ancestor slots.
- The policy logic is a few hundred gates. Nearly all 353 thousand are SHA-256: about 80 compressions for the request hash, two UID hashes, two leaves and two paths.

### Design choices recorded

- A record-shaped entity commitment. Decision 0018.
- Strings in the request are limited to printable ASCII without quote or backslash, so canonical JSON is plain concatenation. Type and action up to 16 bytes, IDs and nonce up to 32.
- The circuit asserts the request fits the schema: a User acts, with a list action on a List or an application action on the application. A request outside the schema has no proof.
- A context with anything but a nonce is not supported by this circuit.

### Caveats

- Noir 1.0.0 reports "Brillig function call isn't properly covered by a manual constraint" inside `noir-lang/sha256` v0.3.0, at `build_msg_block`. The library does constrain the block by rebuilding each word, so this reads as a false positive of the checker. It has not been audited here. The library's README calls it experimental. A sound SHA-256 gadget is a precondition for every number above.
- `bbup` has no Barretenberg version mapped to Noir 1.0.0, which is six days old. Barretenberg 6.0.0, the latest stable, was chosen by hand. It proves and verifies this circuit.
- Not measured: browser proving, phone proving, the scaling series. The scaling series matters less here: the circuit's size depends on tree depth and slot counts, not on store size.

### Later change

During E4 the record commitment gained schema layouts, a presence bit per
attribute, and 16 ancestor slots in place of 8. The hand-written circuit was
updated to match and is now 388,254 gates. The result file above is from the
earlier definition.

### Gate B

Passes. 353,518 gates against a threshold of 1 million. Phase 4, the compiler,
is worth writing.
