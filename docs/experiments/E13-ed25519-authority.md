# E13: An Ed25519 authority in Noir

Status: done, 2026-11-05. Decision 0027. No threshold was set before the run.

Question: can the signed mode take an Ed25519 key, as used by peer-to-peer trust stacks, and at what cost?

## What was built

- `bench/circuit/ed25519`: an Ed25519 verifier in Noir, ours, on noir-lang's `noir-bignum` v0.10.0. SHA-512 written here for fixed lengths.
- `bench/circuit/ed25519_root`: the check alone, over `zk-pdp-root-v0 || root || epoch`. `gen.py` computes the constants and makes test signatures with Python's `cryptography` 42.0.2. `rfc8032.py` writes RFC 8032's tests 1 to 3 as Noir tests, after checking each vector against `cryptography`.
- `bench/circuit/tinytodo_signed_ed25519`: `tinytodo_signed` with the P-256 check replaced by this one. `make_inputs.py` re-signs its 19 inputs with a test key derived from a public string.

## Results

Apple M1 Max, Noir 1.0.0, Barretenberg 6.0.0, UltraHonk, zero-knowledge mode.

| | Gates | Prove | Verify | Proof | Peak memory |
|---|---|---|---|---|---|
| Ed25519 check alone | 436,722 | 2.3 s | about 10 ms | 14,656 bytes | 1.2 GB |
| Signed TinyTodo, P-256 (E3) | 415,020 | 2.07 s | 15 ms | 14,656 bytes | 0.73 GB |
| Signed TinyTodo, Ed25519 | 801,516 | 4.12 s | 18 ms | 14,656 bytes | 2.0 GB |

Times for the check alone are from `/usr/bin/time` on 3 runs. The signed TinyTodo rows are `bench/circuit/run.py`, `results/E13-ed25519-signed-tinytodo-2026-11-05.jsonl` and E3's file.

Correctness:

- 19 of 19 TinyTodo requests match Cedar with the Ed25519 authority.
- RFC 8032 tests 1 to 3 verify, and each fails with a changed message. These have messages of 0 to 2 bytes, so they exercise one SHA-512 block. Our 54-byte message takes two blocks, which the signatures from `cryptography` exercise.
- Rejected, each for the stated reason: wrong root, wrong epoch, wrong key, a changed S, S + L, R from another signature (all "signature does not verify" or a range check), a public key encoded as y = p ("not canonical"), the identity and an order-2 point as keys ("small order"), and S = L ("not below L").

## Notes

- Ed25519 costs about 386,000 gates where P-256 costs about 47,000, because Barretenberg has a built-in for P-256 and Ed25519 runs on emulated field arithmetic.
- Our verifier uses simple double-and-add. A third-party library we read reports 228,000 gates for its check, with windowed multiplication, so about half of ours is room to optimize.
- Not done: browser timing, GordieHowe, adversarial witness tests on this circuit, and parsing a W3C credential signed by such a key.
