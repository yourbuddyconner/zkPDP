# E11: the Cedar compiler targeting GordieHowe

Status: first version done on 2026-11-05. Idle timing and variable-length strings not done. Written after the run, under step 3 of decision 0023.

## Question

Can every policy set the compiler handles run on GordieHowe, with policy as data?

## Design

GordieHowe's language is embedded in C++, and a C++ compile per policy would be slow. So `bench/compiler` writes each request environment's function as text, `policy.ir`, the same logic the Noir circuit has. `bench/gordiehowe/policy.cc` parses that text at run time and builds the relation as expressions. A policy change needs no C++ compile.

Differences from Noir:

- Strings have fixed lengths per request shape, so each shape compiles to its own system, in about 5 to 6 s. Environments a shape cannot match are left out, so a system holds few functions.
- A SHA-256 output is treated as never being the zero slot, so a request's own entity needs no multiplexer for the absent case.

## Results

`results/E11-policy-interpreter-smoke-2026-11-05.jsonl`. Commit to be read from the file's notes: this was a smoke run, with `smoke` in place of a commit, and a Lean build used the machine. So the times below are not measurements for the paper.

| Set | Requests correct | Controls verified | Attacks | Opposite decision accepted | Same decision accepted |
|---|---|---|---|---|---|
| tinytodo | 19 of 19 | 19 | 874 | 0 | 80 |
| github_example | 7 of 7 | 7 | 462 | 0 | 70 |
| hotel_chains | 6 of 6 | 6 | 312 | 0 | 0 |
| document_cloud | 5 of 5 | 5 | 335 | 0 | 60 |
| sales_orgs | 3 of 3 | 3 | 207 | 0 | 30 |

Accepted attacks with the same decision change entity slots the selected function does not read. Proving was about 40 to 100 ms, verifying about 13 to 33 ms, and proofs about 112 to 157 KB, while the machine was busy.

## Not done

- A timed run on an idle machine.
- Variable-length strings. `sha256_varlen` is the likely route, and would give one system for all shapes, as Noir has.
- The Lean link from this interpreter's output to Cedar's term semantics. E10 covers that.
- The interpreter's parser is new code outside any proof, and it reads text the Rust compiler wrote. Its checks are the differential and attack tests above.
