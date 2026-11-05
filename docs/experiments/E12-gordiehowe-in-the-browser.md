# E12: GordieHowe in WebAssembly

Status: first try, 2026-11-05, about 25 minutes, one policy. Not a full benchmark. No threshold was set before the run.

Question: does the GordieHowe prover run in a browser at all, and how slow is it?

Build: GordieHowe 4837770 with `local.patch`, Emscripten 6.0.13, wasm32, `-msimd128`, one thread. OpenSSL 3.4.1 and zstd 1.5.6 built from source. CMake got empty `benchmark` and `GTest` packages. Upstream's portable field arithmetic replaces carry-less multiplication instructions. No GordieHowe source change was needed for this path. Build files are in `~/code/zk-pdp-build`, not in this repository.

Results, TinyTodo, compiled policy, on the M1 Max laptop, see `results/E12-*`:

| | Native | WebAssembly, Node 22 | WebAssembly, Chrome 156 worker |
|---|---|---|---|
| Compile per policy | 3.9 s | 14.6 s | 71 s |
| Prove | 42.8 ms | about 1.02 s | 4.9 to 6.0 s at first, then about 1.2 s |
| Verify | 14.6 ms | about 0.34 s | 1.6 to 2.2 s at first, then about 0.42 s |
| Proof | 125 KB | 125 KB | 125 KB |

Notes:

- In Chrome, the first 6 requests, 30 proofs at 5 runs per printed line, took about 5 s each. The next 3 requests, 15 proofs, took about 1.2 s, close to Node. The cause of the change is not known. A later WebAssembly tier in V8 is one candidate, and the extension's debugger, attached throughout, or other load are others. Chrome's 71 s compile may include the same effect. So a fresh page is slow at first, and only 15 proofs show the faster figure.
- Attacks did not run in the browser. Under Node, the run with attacks stopped at a 280 s limit, and it did not report a total.
- Chrome's run was stopped early for time: only the lines in the results file exist.
- Decisions match the corpus for the requests printed. Not all 19 were run in Chrome.
- Not measured: threads, phone, memory.

The browser pass for Noir on the same day is in `results/E2-E4-browser-v1-2026-11-05.jsonl`. Its second pass went bad from 07:46 PST on, when the 6 s proofs became 36 to 147 s with long gaps. The machine or tab was likely interrupted, which is not confirmed. Use the first pass's steady runs and the second pass up to that point.
