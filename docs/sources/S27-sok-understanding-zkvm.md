# S27: SoK: Understanding zkVM

Yang, Y., Cheng, Y., Tang, H., Yang, G., Zhang, B., Ren, K. "SoK: Understanding zkVM: From Research to Practice." ASIA CCS 2026, pp. 852-868. ePrint 2026/525. DOI 10.1145/3779208.3805964.
PDF supplied by the human on 2026-11-04. Not in this repository. Trust: primary. Introduction, section 7 and table 5 read. Taxonomy sections searched, not read.

## What it says

- Benchmarks RISC Zero v3.0.3, SP1, Ziren, Pico, Airbender, Cairo-M, Jolt, Miden and Nexus on Fibonacci, Hash and Signature programs, on a MacBook Pro with M4 Pro, 14 cores, 48 GB. Ten runs averaged.
- RISC Zero pads each of these small programs to 32,768 cycles and takes about 1.55 s, about 21 kHz. Base proof about 209 KB. Compression about 6.8 s to about 222 KB. Groth16 about 9 s to 256 bytes, verifying in about 3 ms.
- The authors say larger real workloads are out of scope for single-machine testing.
- It says the zero-knowledge property is optional and depends on backend and configuration. No per-zkVM zero-knowledge verdict was found in the passages searched.

## What we take

- Same-workload gap: it has no circuit arm. Noir and Circom do not appear in its evaluation. This confirms the earlier null result.
- Conflict: this paper reports a Groth16 stage for RISC Zero on an Apple M4 Pro. RISC Zero's local proving docs say the Groth16 prover works only on x86 and that Apple silicon is unsupported. Unresolved. Check before claiming the laptop has no such configuration.
- Prediction, an inference and not a measurement: at 21 kHz a 23.3 million cycle proof would take about 18.5 minutes on an M4 Pro, and 7.9 million about 6 minutes. An M1 Max is older and slower by an unmeasured factor. The linear extrapolation from one small segment size is untested. On that basis the two stopped CPU proofs in E1 may simply have been stopped too early, and the five-minute cap on the `main` run was too short to show anything.
