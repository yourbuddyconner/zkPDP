# zkVM performance and proof-system security, for a zkVM backend versus circuit backend comparison

Current as of 2026-11-04. Access limits that affect every section:

- ePrint PDFs and abstract pages, and DBLP, sit behind a bot wall, so ePrint papers were read through ePrint's OAI metadata interface, which returns the abstract only. No ePrint PDF was read.
- ACM Digital Library pages returned 403. The two ACM papers most likely to hold same-workload numbers were not read in full.
- Findings marked "search summary" come from a search tool's summary of a page, not from the page itself. Treat them as leads.
- The local measurements below are scratch runs. They are not project result files and follow none of the rules in `docs/provenance.md`.

## Are there peer-reviewed zkVM-versus-circuit comparisons on the same workload, and what do zkVM designs and evaluations show?

### Takeaway
I found no peer-reviewed paper that runs a general program through both a zkVM and a hand-built circuit under one harness and reports the trade-off. The closest are an unreviewed survey with one SHA-256 and one matrix row each, and vendor pages from 2023 and 2024. Two relevant ACM papers could not be read, so this is "none found", not "none exists".

### Cited Findings

Same-workload evidence:

- Sheybani et al. run 32x32 matrix multiplication and one SHA-256 2-to-1 hash across frameworks. Noir takes 6,972 ms for matrix multiplication and 10,508 ms for SHA-256, setup and proving together. RISC Zero takes 57,269 ms and 4,197 ms, with proofs of 279.6 KB and 215.3 KB against Noir's 2.368 KB. Tested versions were Noir of Nov. 2024 and RISC Zero of Dec. 2024. It is an arXiv preprint with no venue listed — [Sheybani et al., Table IV](https://arxiv.org/pdf/2502.07063).
- The same paper says "it is not possible or fair to compare all of these frameworks" and attributes RISC Zero's slowness to "its nature as a zkVM" — [Sheybani et al.](https://arxiv.org/pdf/2502.07063).
- zkbench.dev (Polybase Labs, last updated 2023-10-03) compares Polylang, Miden, RISC Zero, Noir and Leo on Fibonacci, SHA-256, Pedersen, RPO and Merkle membership. It says "We directly compare different levels of abstraction because this is a valid choice that developers face." Polybase builds Polylang, one of the compared systems. The scraped table could not be mapped to frameworks reliably, so its per-framework numbers are not usable here — [zkbench.dev](https://zkbench.dev/).
- A GitHub benchmark from Nov. 2024 reports Circom at about 46 s and RISC Zero at about 2.5 s for SHA-256 over 1 KB. Search summary only — [babybear-labs/benchmark](https://github.com/babybear-labs/benchmark).
- Base's passkey ECDSA benchmark covers SnarkJS, Rapidsnark, Gnark and Noir. It did not run RISC Zero or SP1 — [Base](https://base.org/blog/benchmarking-zkp-systems).
- provebench excludes zkVMs and lists them as future work — [provebench](https://github.com/agnij-dutta/provebench).
- Prezta (XACML in RISC Zero v3.0.3) compares with circuits only in prose. It says optimized RSA circuits take "around 2 seconds" under Groth16, and that it could not integrate them into RISC Zero — [Wei et al.](https://arxiv.org/pdf/2607.11466).

Evaluations of zkVMs, without a circuit arm:

- SoK: Understanding zkVM (ASIA CCS 2026) decomposes zkVMs into ISA, VM and proving layers and runs an experimental evaluation of "representative zkVMs" — [ePrint 2026/525 abstract](https://eprint.iacr.org/2026/525); [ACM DOI](https://doi.org/10.1145/3779208.3805964). Its table lists SP1 v5.2.2 and Ziren v1.2.1 as FRI inner proofs with a final Groth16 proof over BN254. Search summary only — [ePrint 2026/525 PDF](https://eprint.iacr.org/2026/525.pdf).
- Zero-Knowledge VMs: Security and Usability in Practice appeared at ASE 2026. Metadata only; content unread — [ACM DOI](https://doi.org/10.1145/3832783.3837500).
- The compiler study evaluates 64 LLVM passes, six optimization levels and an unoptimized baseline on 58 benchmarks, on RISC Zero and SP1. Optimization levels improve average execution by 60.5% on RISC Zero and 47.3% on SP1, and proving by 55.5% and 51.1%. It is a zkVM-versus-CPU study, not zkVM-versus-circuit — [Gassmann et al.](https://arxiv.org/html/2508.17518). Published at ASPLOS 2026, pp. 697-714 — [DOI](https://doi.org/10.1145/3779212.3790159).
- In that study, executed instructions track proving time with Pearson r of 0.95 on RISC Zero and 0.9 on SP1. Autotuned pass sequences reach up to 2.2x proving speedup over -O3, and `licm` is the most harmful pass — [Gassmann et al.](https://arxiv.org/html/2508.17518).
- zk-Bench (SCN 2024) targets public-key arithmetic and general ZKP systems, not zkVM-versus-circuit — [ePrint 2023/1503](https://eprint.iacr.org/2023/1503); [DOI](https://doi.org/10.1007/978-3-031-71070-4_3).
- Prezta reports proof generation in "tens of seconds on a desktop" after compiling policies to Rust and pre-compiling regexes. An empty RISC Zero guest pads to 2^15 cycles and takes 3.8 s on an AWS c7i.4xlarge with 16 vCPUs. With RSA removed, 88.6% of policies prove in about 7 s — [Wei et al.](https://arxiv.org/pdf/2607.11466).
- A constraint-level survey of zkEVMs and zkVMs says deployments often exercise "only succinctness rather than the zero-knowledge property" — [Hassanzadeh-Nazarabadi and Taheri-Boshrooyeh](https://arxiv.org/pdf/2510.05376).

Designs:

- RISC Zero: the draft paper dates from Aug. 11, 2023 and adds "random padding to the trace columns" for zero-knowledge — [Bruestle, Gafni et al.](https://dev.risczero.com/proof-system-in-detail.pdf).
- Jolt: lookups as the whole front end — [ePrint 2023/1217](https://eprint.iacr.org/2023/1217); EUROCRYPT 2024 — [DOI](https://doi.org/10.1007/978-3-031-58751-1_1). Lasso is its lookup argument — [ePrint 2023/1216](https://eprint.iacr.org/2023/1216). Twist and Shout covers memory checking — [ePrint 2025/105](https://eprint.iacr.org/2025/105); CRYPTO 2026 — [DOI](https://doi.org/10.1007/978-3-032-35424-2_10).
- SP1 Hypercube: multilinear design with Jagged PCS and LogUp GKR. The vendor reports 99.7% of L1 blocks under 12 s on 16 RTX 5090 GPUs — [Succinct, vendor claim](https://blog.succinct.xyz/real-time-proving-16-gpus/).
- OpenVM: v2.0.0 and later are "recommended for production use" as of July 2026, after a zkSecurity audit — [OpenVM README](https://github.com/openvm-org/openvm).
- Nexus: "experimental stage and is not currently recommended for production use" — [Nexus README](https://github.com/nexus-xyz/nexus-zkvm).
- Ceno: "under construction and not suitable for use in production" — [Ceno README](https://github.com/scroll-tech/ceno). Paper in Journal of Cryptology 2025 — [DOI](https://doi.org/10.1007/s00145-024-09533-2).
- Ziren, formerly zkMIPS, runs the MIPS32r2 ISA — [Ziren README](https://github.com/ProjectZKM/Ziren).
- Cairo — [ePrint 2021/1063](https://eprint.iacr.org/2021/1063). Circle STARKs — [ePrint 2024/278](https://eprint.iacr.org/2024/278). Stwo describes itself as in production — [Stwo README](https://github.com/starkware-libs/stwo).
- Ethproofs tracks real-time proving for OpenVM, ZisK, zkDTVM and others on multi-GPU rigs. It has a soundcalc column and no zero-knowledge metric — [Ethproofs](https://ethproofs.org/).

Project numbers, for reference:

- TinyTodo: 23.3 million cycles, with 50.3% schema parsing, 40.3% entities, 8.6% policies and 0.3% authorize. One composite proof was stopped unfinished after 28 minutes at about 8 busy cores and 10 GB — [E1, `docs/experiments/E1-cedar-in-risc0.md`](file:///Users/connerswann/code/zk-pdp/docs/experiments/E1-cedar-in-risc0.md).
- The hand-written circuit is 388,254 gates and proves natively in about 2.4 s. The result file is from the earlier 353,518-gate version, at 2.22 s in ZK mode and 2.19 s plain — [E2, `docs/experiments/E2-tinytodo-in-noir.md`](file:///Users/connerswann/code/zk-pdp/docs/experiments/E2-tinytodo-in-noir.md).

### Inferences
- The 28-minute result is not an anomaly. Prezta's 3.8 s for 2^15 cycles scales linearly to about 45 minutes for 23.3 million cycles on 16 vCPUs. This is a rough extrapolation. It ignores segment-size effects and receipt-type differences, and Prezta's padded empty guest is a poor rate estimator.
- Sheybani's SHA-256 row, where RISC Zero beats Noir, probably reflects RISC Zero's SHA-256 precompile. The paper does not say this. It shows the direction of a zkVM-versus-circuit result depends on whether the workload has a precompile, so one workload cannot settle the question.
- The honest statement for the paper is a lower bound, not a ratio: over 28 minutes against about 2.4 s, on one machine, CPU only, with the zkVM reading the whole store and the circuit reading two records. No "700x" figure should be printed until the zkVM run finishes and both backends read the same records.
- The compiler lever is too small to matter here. The guest already builds with `opt-level = 3` and fat LTO, and the best reported tuning is about 2.2x. Reducing input is the larger lever: E1 measures 215 thousand cycles per entity.
- A rough estimate for a like-for-like zkVM input: the no-schema run at 11.7 million cycles, less 39 of 43 entities at about 215 thousand each, gives about 3 million cycles. The per-entity figure may differ without a schema. This should be measured, not cited.
- The two backends differ in coverage as well as speed. The zkVM handles all Cedar features and the circuit handles 3 of 7 sample policy sets. Any speed claim must carry that next to it.

### Gaps
- SoK: Understanding zkVM and the ASE 2026 paper were not read. Either may contain a circuit arm or per-zkVM zero-knowledge verdicts. A person with library access should check both.
- No primary technical source was read for Airbender or Valida. Their repositories state nothing useful in the top-level README that I checked. The Valida compiler repository had no matching statement.
- RISC Zero's benchmark site did not resolve (`benchmarks.risczero.com`), so no current vendor kHz figure is included.

## Does parsing or serialization dominate zkVM guests, and what reduces it?

### Takeaway
Parsing dominance is reported in practice but barely in the literature. The only figure I found is a vendor blog saying 50 percent of proving cost at one point in early development. The academic work attacks parsing by moving it out of the proved computation or proving it with witness help. I found no paper or benchmark that reports measured rkyv savings in a zkVM guest.

### Cited Findings
- "At one point in early development, 50% of proving costs came just from deserializing the witness" for a Monad client guest. They report "large performance gains" from changing the witness format and do not quantify them. Vendor blog, Oct. 20, 2026 — [Category Labs](https://www.category.xyz/blogs/optimizing-the-monad-client-for-zkvm-execution).
- Prezta pre-compiles regexes to a serialized DFA so that "Pattern parsing and NFA/DFA construction" leave the trace, and compiles policies to Rust. The combined measures reduce prover time "by more than an order of magnitude" — [Wei et al.](https://arxiv.org/pdf/2607.11466).
- Prezta deliberately keeps `serde_json` for JWT parsing. It reports a special protocol in the zkVM has similar overhead for tokens of no more than 10 payload fields, and it contrasts this with zkLogin's auxiliary-input approach — [Wei et al.](https://arxiv.org/pdf/2607.11466).
- RISC Zero's guide: "Proving times for the zkVM are directly related to the number of cycles." `env::read` deserializes into structs. `env::read_slice` and `read_to_end` "do no (de)serialization." It advises looking "for places where you are copying or (de)serializing data when not necessary." It does not mention rkyv or bytemuck — [RISC Zero Guest Optimization Guide v3.0](https://dev.risczero.com/api/zkvm/optimization).
- A page-in or page-out costs 1,094 to 5,130 cycles, 1,130 on average, with 1 kB pages — [RISC Zero guide](https://dev.risczero.com/api/zkvm/optimization). The compiler study confirms about 1,130 and finds `licm` raises paging cycles 32% across benchmarks — [Gassmann et al.](https://arxiv.org/html/2508.17518).
- rkyv offers zero-copy access, with optional validation that still has a cost — [rkyv FAQ](https://rkyv.org/faq.html).
- RISC Zero added a raw byte interface "to enable usage of frameworks like rkyv" — [risc0 PR 256](https://github.com/risc0/risc0/pull/256). OpenVM carries an rkyv guest benchmark in its tree — [OpenVM `benchmarks/guest/rkyv`](https://github.com/openvm-org/openvm/tree/main/benchmarks/guest/rkyv).
- Frigo and shelat add "efficient ZK arguments for statements about SHA-256 and document parsing for ISO-standardized identity formats" — [ePrint 2024/2010](https://eprint.iacr.org/2024/2010); IACR CiC — [DOI](https://doi.org/10.62056/a3qjmpgxq).
- Vega uses lookups to extract fields "without full in-circuit parsing" and pushes repeated work into a rerandomizable precomputation: 92 ms proving for a 1,920-byte credential — [ePrint 2025/2094](https://eprint.iacr.org/2025/2094).
- Coral reduces correct parsing against a context-free grammar to "a few simple checks over a left-child right-sibling tree," and proves JSON, TOML and C parsing in seconds on a laptop — [ePrint 2025/1420](https://eprint.iacr.org/2025/1420); IEEE S&P 2026 — [DOI](https://doi.org/10.1109/sp63933.2026.00059).
- Reef does the same for regular expressions — [ePrint 2023/1886](https://eprint.iacr.org/2023/1886).
- Earlier precedents, metadata only: Cinderella for X.509 — [DOI](https://doi.org/10.1109/sp.2016.22), and zkLogin for JWTs — [DOI](https://doi.org/10.1145/3658644.3690356).

### Inferences
- E1's result, 99 percent of cycles in parsing for a policy engine, looks like a new and citable measurement. Prior reports are about witness formats for blockchain clients or JWT and regex gadgets.
- Parsing-heavy Rust allocates and touches many pages, so its paging cost may add to its instruction cost. E1's per-segment counts include paging only if read through total cycles. Reporting user and paging cycles apart would show this.
- Coral and Vega suggest a middle path for the circuit backend's limits and for the zkVM's parsing cost: prove that a pre-parsed form matches a committed byte string, instead of parsing. Neither has been applied to policy languages in anything I found.

### Gaps
- No measured bincode-to-rkyv cycle saving was found in any paper, or in the RSP, Zeth or OpenVM reth benchmark repositories I could search. OpenVM's benchmark could be run directly.
- No source measures validated rkyv access against schema-based deserialization inside a guest, which is what Cedar would need.

## Which configurations are actually zero-knowledge?

### Takeaway
Among the configurations this project uses or could use, the only one with a published theorem behind its zero-knowledge is Groth16. RISC Zero's composite and succinct receipts target zero-knowledge but are not proven, and the advisory remains open with no patch. Groth16 cannot be produced on the project's laptop. UltraHonk with a ZK target is designed to hide the witness, but I found no published proof of its zero-knowledge, and that mode carried a critical soundness bug until v5.0.0.

### Cited Findings

Status table (each row is sourced in the bullets below):

| Configuration | Hides witness from verifier? | Basis | Runs on M1 Max? |
|---|---|---|---|
| RISC Zero composite receipt | Targeted, not proven. Leaks execution length. | Vendor statement; advisory open | Yes, CPU |
| RISC Zero succinct receipt | Targeted, not proven. No length leak claimed. | Vendor statement; advisory open | Yes, CPU |
| RISC Zero Groth16 receipt | Yes by Groth16's theorem, given an honest or checked CRS. RISC Zero makes no explicit statement that I found. | Groth 2016, inference | No, x86 only |
| SP1 compressed STARK | No, by the vendor's own statement | SP1 docs | n/a |
| SP1 Groth16 or PLONK | Yes, by the vendor's statement | SP1 docs | n/a |
| Jolt default | No | a16z | n/a |
| Jolt with `zk` feature (BlindFold) | Designed to be. No formal theorem on the docs page. | Jolt docs | n/a |
| UltraHonk `noir-recursive`, `evm`, `noir-rollup` | Designed to be. No published proof found. | bb docs, code | Yes |
| UltraHonk `*-no-zk` | No. "the witness ... may be recoverable." | bb docs | Yes |

RISC Zero and the advisory:

- GHSA-5xgj-pmjj-gw49, "RISC Zero zkVM notes on zero-knowledge," published July 15, 2024, severity Low, all versions affected, no patched version, no CVE. It says research by Haböck and Kindi indicates several STARK implementations including RISC Zero "do not provably meet the requirements for the zero-knowledge property," and that "we are not aware of any attacks." It was to be updated periodically; the page shows no update — [advisory](https://github.com/risc0/risc0/security/advisories/GHSA-5xgj-pmjj-gw49).
- The v3.0 security model says the zkVM "hides all program inputs and all witness data, and targets perfect zero-knowledge" but "we have not written a mathematical argument to prove that our system is zero-knowledge," and urges "those with critical privacy requirements to take caution." It also says non-recursed RISC-V proofs "leak information about the length of execution," "recursion proofs leak no information about execution length," and "Whoever is generating the proofs can see the secret data" — [RISC Zero security model](https://dev.risczero.com/api/security-model).
- Haböck and Kindi abstract: the note treats small-field FRI STARKs and two techniques, randomization by basefield polynomials and quotient decomposition, the latter "a source for mistakes, both in literature as well as in software implementations." The updated version adds perfect zero-knowledge in permutation arguments. First posted 2024-06-26, revised 2025-02-22 — [ePrint 2024/1037 via OAI](https://eprint.iacr.org/2024/1037).
- Search summaries of the PDF add: gaps found "in the codebases of Plonky2, Risc-Zero, Triton"; a theorem that the protocol with quotient decomposition 8 rather than 5 is perfect honest-verifier zero-knowledge; and that a grand product argument leaks witness information. **I did not read the PDF, so these three points need checking against it** — [ePrint 2024/1037 PDF](https://eprint.iacr.org/2024/1037.pdf).
- A practitioner explainer built on the note gives the masking budget h >= 2·S·(e·n_D + n_F) + n_F and adds a fresh random polynomial before FRI — [Hexens](https://hexens.io/blog/zk-in-starks).
- RISC Zero's source sets `ZK_CYCLES = 1024` with the comment "TODO: Ideally we'd compute ZK_CYCLES programmatically" — [`risc0/zkp/src/lib.rs`](https://github.com/risc0/risc0/blob/main/risc0/zkp/src/lib.rs).
- Masking errors are live elsewhere. Plonky3 PR 2100, merged 2026-09-09, says "Hiding FRI accepted traces with insufficient masking" — [Plonky3 PR 2100](https://github.com/Plonky3/Plonky3/pull/2100). An issue on a Plonky3-based prover reports recovering a 4-row trace from a single ZK proof, and says trace heights leak through proof shape — [multi-stark issue 89](https://github.com/argumentcomputer/multi-stark/issues/89).
- A fork of gnark's PLONK-FRI implementing the note's fixes reports honest-verifier zero-knowledge, with proof size growing from 0.84 to 0.90 MB at 65k constraints and from 0.19 to 0.43 MB at 2 constraints. Not externally reviewed — [gnark-fork PR 6](https://github.com/dewindtk/gnark-fork/pull/6).
- Fault injection against STARK provers to violate zero-knowledge is now studied. These are candidate attacks and need physical access to the prover — [Dalton, Schofnegger, Page, ePrint 2026/835](https://eprint.iacr.org/2026/835).

Groth16 wrap:

- Groth's SNARK "is zero-knowledge and does not reveal anything about the witness" — [ePrint 2016/260](https://eprint.iacr.org/2016/260); EUROCRYPT 2016 — [DOI](https://doi.org/10.1007/978-3-662-49896-5_11).
- Fuchsbauer proves subversion zero-knowledge for "the almost optimal construction by Groth," meaning zero-knowledge can survive a maliciously chosen CRS under a plausible assumption — [ePrint 2017/587](https://eprint.iacr.org/2017/587).
- Rapidsnark draws fresh random r and s for each proof — [`groth16.cpp`, lines 226-227](https://github.com/iden3/rapidsnark/blob/main/src/groth16.cpp).
- SP1 states plainly: "our implementations of Groth16 and PLONK are zero-knowledge" and "individual STARK proofs in SP1 do not currently satisfy the zero-knowledge property" — [SP1 security model](https://docs.succinct.xyz/docs/sp1/security/security-model).
- Google's quantum resource paper relies on exactly this. It proves its secret circuit in SP1 with a Groth16 wrap, because individual STARK proofs are not zero-knowledge — [Babbush et al., Appendix A](https://arxiv.org/pdf/2603.28846).
- RISC Zero's Groth16 step assumes BN254 pairing security, Knowledge of Exponent and ceremony integrity, and is quantum-vulnerable — [RISC Zero security model](https://dev.risczero.com/api/security-model).
- "The Groth16 prover currently only works on x86 architecture" and "Apple Silicon is currently unsupported," even with Docker — [RISC Zero local proving](https://dev.risczero.com/api/generating-proofs/local-proving).
- RISC Zero's security repository lists a zkSecurity Groth16 report and a Hexens stark2snark report. I saw the file names and did not read the reports — [rz-security `audits`](https://github.com/risc0/rz-security/tree/main/audits).

UltraHonk:

- bb's target table marks `evm`, `noir-recursive` and `noir-rollup` as ZK and their `-no-zk` variants as not. "Proofs are still sound ... but the witness (private inputs) may be recoverable from the proof" — [bb CLI options](https://github.com/AztecProtocol/aztec-packages/blob/next/barretenberg/docs/docs/cli_options.md).
- The project's harness measures both `noir-recursive` and `noir-recursive-no-zk` — [`bench/circuit/run.py`, line 108](file:///Users/connerswann/code/zk-pdp/bench/circuit/run.py). E2 measures ZK against plain at 2.22 s and 2.19 s, 14,656 and 13,120 bytes — [E2](file:///Users/connerswann/code/zk-pdp/docs/experiments/E2-tinytodo-in-noir.md).
- The construction uses a Libra-style zero-knowledge sumcheck plus a small-subgroup inner-product argument. The code labels it "ZK Sumcheck (Libra)" — [`zk_sumcheck_data.hpp`](https://github.com/AztecProtocol/aztec-packages/blob/next/barretenberg/cpp/src/barretenberg/sumcheck/zk_sumcheck_data.hpp). Building-block papers: [Libra, ePrint 2019/317](https://eprint.iacr.org/2019/317); [Chiesa, Forbes, Spooner, ePrint 2017/305](https://eprint.iacr.org/2017/305); [Zeromorph, ePrint 2023/917](https://eprint.iacr.org/2023/917).
- **The ZK flavor carried a critical soundness bug.** "Every Barretenberg release before v5.0.0, and every 5.0.0 nightly" has the small-subgroup IPA flaw, which "only affects ZK proofs: Plain proofs do not run the Libra/SmallSubgroupIPA sub-protocol," and permits forging a ZK proof for any witness. The fix changes the verification equation — [zkVerify PR 441](https://github.com/zkVerify/zkVerify/pull/441).
- On September 24, 2026, Payy lost about $1.92M through forged proofs on a pinned October 2025 build. The fix was committed May 14 and released in v5.0.0 on July 13. Certora's Lean model shows the old check accepts any claim and the fixed check is sound for that sum, and says it does not prove Barretenberg sound — [Certora](https://www.certora.com/blog/payy-exploit-forged-zk-proof-attack); [fix commit](https://github.com/AztecProtocol/aztec-packages/commit/58bf73a289).
- External audit scope files for `ultra_zk_flavor` and the Honk system exist in the repository, pointing at a private repository. No reports sit beside them — [`honk_proving_system_audit_scope.md`](https://github.com/AztecProtocol/aztec-packages/blob/next/barretenberg/cpp/scripts/audit/audit_scopes/honk_proving_system_audit_scope.md).

"Succinct but not ZK":

- a16z: "Most zkVMs are not actually zero knowledge — unless an expensive 'wrapping' procedure is applied." It calls the terminology misuse "a real problem" — [The Block, March 3, 2026](https://www.theblock.co/post/392098/a16z-crypto-updates-jolt-zkvm-challenges-loose-use-of-zk-label).
- BlindFold adds Pedersen-committed sumcheck messages folded with Nova and proved with Spartan. It is off by default, behind a `zk` Cargo feature with the Dory backend, and hides computationally under discrete log. The page gives no formal zero-knowledge theorem. Reported overhead is about 3 KB — [Jolt docs](https://jolt.a16zcrypto.com/how/blindfold.html); [The Block](https://www.theblock.co/post/392098/a16z-crypto-updates-jolt-zkvm-challenges-loose-use-of-zk-label).
- Prezta: "our architecture only utilizes the soundness and succinctness of SNARK, but not zero-knowledge. In fact, the term 'zkVM' refers to a SNARK for a virtual machine not necessarily with zero-knowledge as a convention in the community" — [Wei et al.](https://arxiv.org/pdf/2607.11466).
- A 2024 comparison concludes SP1, Nexus and Jolt do not prioritize or achieve full zero-knowledge. It predates BlindFold and may be stale — [vac.dev](https://vac.dev/rlog/zkVM-explorations).
- Plonky3's README notes one of its examples "is not zero knowledge" — [Plonky3 README](https://github.com/Plonky3/Plonky3).

### Inferences
- A Groth16 wrap restores zero-knowledge against the verifier, whatever the STARK's status, because the STARK seal becomes the witness to a zero-knowledge outer proof. This rests on Groth's theorem and is what SP1 and Babbush et al. rely on. RISC Zero does not say it about its own receipts in anything I found, so the paper should state it as reasoning.
- The wrap does not help against the wrapping prover. If wrapping runs on another machine, that machine sees the succinct receipt, whose zero-knowledge is unproven. Since RISC Zero cannot wrap on Apple silicon, a laptop user who needs Groth16 must either use a separate x86 machine or trust a service.
- **So the project's laptop has no RISC Zero configuration with proven zero-knowledge.** The comparison should say this directly. The circuit's ZK mode runs on that laptop at about 1 percent extra proving time and 1,536 extra bytes, on the 353,518-gate version.
- Calling UltraHonk ZK mode "zero-knowledge" needs the same care as RISC Zero. The honest wording is "designed to be zero-knowledge, no published proof that we could find." The difference from RISC Zero is that no advisory says it falls short, not that a theorem exists.
- Plain mode and ZK mode have different soundness histories: ZK mode ran extra sub-protocols that had a forgery bug. A paper saying "ZK mode costs only 1 percent" should also say it adds code that failed. The project's bb 6.0.0 is after the fix.
- Public outputs are outside all of these guarantees. The journal, the public inputs and any hash of low-entropy data are visible. A hash of a small request space can be brute-forced whatever the proof system's status.

### Gaps
- **The Haböck and Kindi PDF was not read.** The bot wall blocked it, and the Wayback Machine, ResearchGate and Semantic Scholar were not available to the tools. The exact claim about RISC Zero, and whether `ZK_CYCLES = 1024` meets the note's bound, need a person with the PDF.
- I found no statement from RISC Zero on whether the advisory's issue applies to succinct receipts differently from composite ones, and no update to the advisory since 2024.
- I found no published proof of UltraHonk's zero-knowledge, and no audit report covering the ZK flavor. A proof could exist in unpublished Aztec material.
- I found no explicit current statement on zero-knowledge from OpenVM, Airbender, Ziren, Valida or Nexus.
- No concrete attack extracting a witness from a RISC Zero receipt was found. The advisory says none is known. The multi-stark recovery concerns a different prover, and a very short trace.

## What do recent attacks on Fiat-Shamir and STARK proximity gaps imply?

### Takeaway
Up-to-capacity proximity gap conjectures for Reed-Solomon codes have been disproved, so bit counts resting on them are not reliable. RISC Zero's stated 96 to 99 bits rest on a conjecture of this family. The Fiat-Shamir attack is real but targets a GKR construction, and I found nothing applying it to FRI or UltraHonk as deployed.

### Cited Findings

Proximity gaps:

- BCIKS show a proximity gap for affine spaces for any distance below the Johnson/Guruswami-Sudan bound. The later papers below refer to its up-to-capacity conjecture — FOCS 2020 [DOI](https://doi.org/10.1109/focs46700.2020.00088); J. ACM 2023 [DOI](https://doi.org/10.1145/3614423); [ePrint 2020/654](https://eprint.iacr.org/2020/654).
- Diamond and Gruen prove "the capacity conjecture ... is false" for families with rates converging to 0, and say "many deployed SNARKs may be less secure than they were formerly—optimistically—assumed to be" — [ePrint 2025/2010](https://eprint.iacr.org/2025/2010).
- Crites and Stewart disprove correlated agreement up to capacity, mutual correlated agreement up to capacity (WHIR), and list-decodability up to capacity (DEEP-FRI) — [ePrint 2025/2046](https://eprint.iacr.org/2025/2046).
- Ben-Sasson, Carmon, Haböck, Kopparty and Saraf give improved positive results up to the unique decoding and Johnson radii, plus negative results — [ePrint 2025/2055](https://eprint.iacr.org/2025/2055).
- Krachun, Kazanin and Haböck give a simple counterexample over multiplicative subgroups of prime fields near capacity, at distance 1-rho-eta with eta = Theta(1/log n) — [ePrint 2026/782](https://eprint.iacr.org/2026/782). Kambiré writes out the same result — [arXiv 2604.09724](https://arxiv.org/html/2604.09724).
- Goyal and Guruswami, STOC 2026: optimal proximity gaps for subspace-design codes and random Reed-Solomon codes. Metadata only — [DOI](https://doi.org/10.1145/3798129.3800864).
- Contrary claim, unvetted: Okamoto claims a "complete resolution" of correlated agreement up to capacity. I did not assess it against the counterexamples and would not cite it as settling anything — [ePrint 2025/1712](https://eprint.iacr.org/2025/1712).
- RISC Zero: 96 bits for the RISC-V prover and 99 for recursion, assuming the random oracle model and the "Toy Problem Conjecture," which "says that the best known attack on STARK proof systems is the best possible attack" — [RISC Zero security model](https://dev.risczero.com/api/security-model). The source comment says "50 FRI queries are sufficient to achieve our security target of 97 bits (conjectured security)" — [`risc0/zkp/src/lib.rs`](https://github.com/risc0/risc0/blob/main/risc0/zkp/src/lib.rs).
- The Ethereum Foundation requires 100-bit provable security by May 2026 and 128-bit by the end of 2026, saying "what was advertised as 100 bits might actually be 80" — [EF blog, Dec. 18, 2025](https://blog.ethereum.org/2025/12/18/zkevm-security-foundations). Ethproofs still shows the 100-bit milestone in progress — [Ethproofs](https://ethproofs.org/).
- SP1 Hypercube says it relies on proven proximity gap theorems in the unique decoding regime, not on conjectures — [SP1 security model](https://docs.succinct.xyz/docs/sp1/security/security-model).
- A simpler round-by-round proof of FRI soundness exists — [Garreta, Mohnblatt, Wagner, ePrint 2025/1993](https://eprint.iacr.org/2025/1993).

Fiat-Shamir:

- Khovratovich, Rothblum and Soukhanov: for every choice of hash, a GKR-based argument is not adaptively sound after Fiat-Shamir, with an explicit circuit proving a false statement — [ePrint 2025/118](https://eprint.iacr.org/2025/118); CRYPTO 2025 — [DOI](https://doi.org/10.1007/978-3-032-01887-8_1).
- Brzuska, Hubáček and Kalsta simplify earlier attacks, PKC 2026. Metadata and search summary only — [DOI](https://doi.org/10.1007/978-3-032-26731-3_17).
- Arnon and Yogev propose a white-box-secure transform — [ePrint 2025/329](https://eprint.iacr.org/2025/329). Ghosal et al. propose a VRF-based instantiation — [ePrint 2026/2114](https://eprint.iacr.org/2026/2114).
- FRI and batched FRI have Fiat-Shamir security proofs in the random oracle model — [Block et al., ePrint 2023/1071](https://eprint.iacr.org/2023/1071); ASIACRYPT 2023 — [DOI](https://doi.org/10.1007/978-981-99-8724-5_1).
- Weak Fiat-Shamir, using an insufficiently bound transcript, is an implementation failure class — [Dao et al., ePrint 2023/691](https://eprint.iacr.org/2023/691); IEEE S&P 2023 — [DOI](https://doi.org/10.1109/sp46215.2023.10179408). SP1 shipped exactly this class in 2025: "Missing verifier checks and fiat-shamir observations" — [GHSA-c873-wfhp-wx5m](https://github.com/succinctlabs/sp1/security/advisories/GHSA-c873-wfhp-wx5m).

### Inferences
- The paper should report RISC Zero's bits as "conjectured, per vendor," and not treat 96 bits as a fact. The conjecture named is the Toy Problem one, not exactly the disproved ones. Whether RISC Zero's parameters lose bits under the new results is an open question, not a known loss.
- For a policy decision with a short life, 80-plus bits may well suffice. That is a judgement for the threat model, and the note should not make it for the paper.
- UltraHonk with KZG does not rest on Reed-Solomon proximity gaps. This is a real difference, but it brings a universal trusted setup and pairing assumptions. It is also quantum-vulnerable, as the STARK is not.
- `noir-recursive` hashes its transcript with Poseidon2. A reader may ask about KRS-style attacks on circuits that embed their own Fiat-Shamir hash. A search of the project's circuit sources finds no recursive verification, so this seems not to apply today. It would become relevant with recursion or composition.

### Gaps
- I found no analysis stating the current RISC Zero bit count under the 2025-2026 results. Its security calculator was not run.
- I found no published application of the KRS attack to FRI-based or sumcheck-based systems as deployed.
- The status of the December 2026 EF milestone is unknown.

## What is known about soundness bugs, formal verification, audits, and the status of Noir 1.0.0 and noir-lang/sha256?

### Takeaway
No zkVM has end-to-end formal verification, and every major system has had critical soundness bugs since 2024. Noir 1.0.0 shipped on 2026-10-29, while its own `SECURITY.md` at that tag says "not fully audited" and "not recommended for use in production." `noir-lang/sha256` calls itself experimental and not fully audited. I found no audit of it.

### Cited Findings

RISC Zero:

- CVE-2025-61588, critical: the host can write to arbitrary guest memory through `sys_read`, leading to code execution in the guest, for all guests built with affected versions. Fixed in `risc0-zkvm` 2.3.2 and 3.0.3, `risc0-zkvm-platform` 2.1.0. No circuit change. Guests must be rebuilt and image IDs change — [GHSA-jqq4-c7wq-36h7](https://github.com/risc0/risc0/security/advisories/GHSA-jqq4-c7wq-36h7).
- The project's guest lock has `risc0-zkvm` 3.0.6 and `risc0-zkvm-platform` 2.2.3, both past the fix — [`bench/zkvm/methods/guest/Cargo.lock`](file:///Users/connerswann/code/zk-pdp/bench/zkvm/methods/guest/Cargo.lock).
- CVE-2025-52484, critical: missing constraint in the rv32im circuit — [GHSA-g3qg-6746-3mg9](https://github.com/risc0/risc0/security/advisories/GHSA-g3qg-6746-3mg9).
- CVE-2025-54873, medium: signed division allowed two outputs and division by zero was underconstrained. Found with Veridise's Picus — [GHSA-f6rc-24x4-ppxp](https://github.com/risc0/risc0/security/advisories/GHSA-f6rc-24x4-ppxp).
- Reports from Hexens, Veridise and zkSecurity are listed. File names only; not read — [rz-security](https://github.com/risc0/rz-security/tree/main/audits).

SP1:

- January 2025: two vulnerabilities in V3, one found by Aligned, LambdaClass and 3MI Labs and one by Succinct, plus a Plonky3 issue. Patched in SP1 Turbo — [Succinct](https://blog.succinct.xyz/sp1-security-update-1-27-25/). LambdaClass says the two combine to allow false proofs — [LambdaClass](https://blog.lambdaclass.com/responsible-disclosure-of-an-exploit-in-succincts-sp1-zkvm-found-in-partnership-with-3mi-labs-and-aligned-which-arises-from-the-interaction-of-two-distinct-security-vulnerabilities/).
- CVE-2026-40323, high: recursion circuit row-count binding gap in V6. The authors have not shown a full exploit — [GHSA-63x8-x938-vx33](https://github.com/succinctlabs/sp1/security/advisories/GHSA-63x8-x938-vx33).
- GHSA-p7v2-jjv3-8p8f, high, Oct. 2026: native Rust verifier, mismatched tensor lengths — [SP1 advisories](https://github.com/succinctlabs/sp1/security/advisories).

Jolt:

- The repository says Jolt "is in alpha and is not suitable for production use" and "has not been audited" — [Jolt README](https://github.com/a16z/jolt).
- zkSecurity's manual review, "not a formal audit," found truncated traces that verify, a zero mask polynomial letting arbitrary outputs pass, and a prover-controlled memory layout, all fixed — [zkSecurity, Nov. 19, 2024](https://blog.zksecurity.xyz/posts/jolt-findings/).
- **Conflict:** a Jolt PR says Thaler's April 2026 notes found "No soundness vulnerabilities" — [Jolt PR 1442 mirror](https://github.zuike.workers.dev/a16z/jolt/pull/1442) — while a third-party write-up says three findings around the same codebase were real security issues — [n0zom1z0](https://n0zom1z0.github.io/posts/jolt-audit/). Both are search summaries; I did not read the primary report.

Testing and verification:

- Arguzz found eleven bugs, three soundness and eight completeness, in three of six zkVMs (RISC Zero, Nexus, Jolt, SP1, OpenVM, Pico). One RISC Zero bug earned $50,000 "despite prior audits" — [Hochrainer et al.](https://arxiv.org/abs/2509.10819).
- ZEBRA flagged 13 behaviors, 11 previously unknown, in Pico, SP1, Sphinx, Ziren and Valida, including four critical (Valida 3, Sphinx 1). Its authors report Arguzz found none on these targets in six hours per target. Preprint, not replicated — [Takahashi, Jana, Yang](https://arxiv.org/html/2609.15020).
- "No zkVM has end-to-end formal verification, and verified components are isolated." Six tools tested against 70 real vulnerabilities detect 45.7% in isolated targets and 19.6% in full codebases. Most tools target Circom. The verification work it surveys targets soundness and completeness, and it describes none for zero-knowledge — [Kolozyan et al.](https://arxiv.org/html/2607.23752). I read the first 73 percent.
- SP1 Hypercube's Lean proofs: EF's review finds 51 of 62 opcodes with complete, correct proofs. LH, LHU, LW and LWU are "effectively unverified," all load theorems depend on `sorry`, and the bus and memory consistency are assumed. It also found a JALR completeness bug, fixed in v6.1.0 — [EF, May 20, 2026](https://zkevm.ethereum.foundation/blog/sp1-fv).
- RISC Zero: Picus has verified determinism for the Keccak accelerator and much of the RISC-V circuit, with a full proof in progress. Vendor statement, date not checked — [Veridise](https://veridise.com/blog/zero-knowledge/risc-zeros-zk-vm-security-how-veridise-enabled-risc-zero-to-achieve-provable-continuous-zk-security/). Nethermind has a Lean proof of concept for selected components — [Nethermind](https://nethermind2023.webflow.io/blog/towards-formal-verification-of-the-first-risc-v-zkvm).
- Precompiles remain largely unverified. Individual cases include Plonky3's Keccak in Rocq and RISC Zero's Keccak determinism — [Kolozyan et al.](https://arxiv.org/html/2607.23752).
- The SNARK vulnerability SoK analyzes 141 real vulnerabilities — [Chaliasos et al.](https://arxiv.org/abs/2402.15293).
- Liezz splices witnesses to test ZK DSL toolchains including Noir: 13 bugs, seven with soundness impact across Circom, Corset, Gnark and Noir. The abstract does not break these down by DSL — [Watzinger et al.](https://arxiv.org/abs/2608.30648).
- Circuzz, CCS 2025. Metadata only — [DOI](https://doi.org/10.1145/3719027.3744791). Picus's method, PACMPL 2023. Metadata only — [DOI](https://doi.org/10.1145/3591282).

Noir and Barretenberg:

- Noir v1.0.0 was released 2026-10-29, marked Latest and not a prerelease. The notes list only a release chore, with nothing on audits — [v1.0.0 release](https://github.com/noir-lang/noir/releases/tag/v1.0.0).
- `SECURITY.md` at the v1.0.0 tag: "Noir is not fully audited and is not recommended for use in production," with "All versions" marked "Not production ready" — [SECURITY.md at v1.0.0](https://github.com/noir-lang/noir/blob/v1.0.0/SECURITY.md).
- The default-branch README says "It has not been reviewed or audited. It is not suitable to be used in production. Expect bugs!" — [Noir README](https://github.com/noir-lang/noir).
- **Conflict:** Noir's X account said "A fully audited Noir is the final step" and targeted Q2 2026 — [Noir on X](https://x.com/NoirLang/status/1991586815749550383). Aztec's pages mention internal and external audits without dates or reports (search summary) — [Aztec roadmap update](https://aztec.network/blog/aztec-network-roadmap-update). I found no published audit report tied to 1.0.0.
- Noir's repository has 55 published security advisories, with 7 critical, 9 high, 35 medium and 4 low. 52 are dated 2026 and many concern Brillig, its unconstrained runtime — [GitHub advisories API](https://github.com/noir-lang/noir/security/advisories).
- `noir-lang/sha256` README, warning added 2026-10-20: "This library is experimental software. It has not been fully audited and is not guaranteed to be secure. Expect bugs!" and "Bugs in this library can lead to under-constrained circuits, allowing invalid proofs to be accepted." It also says "tested to work as of Noir v0.36.0" — [sha256 README](https://github.com/noir-lang/sha256).
- The latest tag is v0.3.0, from 2025-12-09 — [sha256 changelog](https://github.com/noir-lang/sha256/blob/main/CHANGELOG.md). On 2026-07-29 PR 62 fixed `partial_sha256_var_end` dropping trailing bytes in the constrained path, so "a prover could set the dropped bytes to anything," a "preimage-binding break." That fix came after v0.3.0 — [sha256 PR 62](https://github.com/noir-lang/sha256/pull/62).
- The project pins v0.3.0 and calls `digest` and `sha256_var`, not the partial API — [`bench/circuit/generated/tinytodo`](file:///Users/connerswann/code/zk-pdp/bench/circuit/generated/tinytodo/src/main.nr). E2 records a Brillig constraint warning from Noir 1.0.0 inside `build_msg_block`, judged a likely false positive and not audited — [E2](file:///Users/connerswann/code/zk-pdp/docs/experiments/E2-tinytodo-in-noir.md).
- The `noir-lang/poseidon` library carries the same experimental warning — [poseidon README](https://github.com/noir-lang/poseidon).
- Barretenberg critical bugs in 2026: the Payy flaw above; a bug rated "medium for the network and critical for Noir," reported by Consensys Diligence and TU Wien, fixed in v5.0.0-nightly.20260324 — [Aztec, March 27, 2026](https://aztec.network/blog/critical-vulnerability-in-alpha-v4); a V5 proving system flaw found by "internal AI-assisted auditing," details undisclosed — [Aztec, Aug. 7, 2026](https://aztec.network/blog/alpha-v5-proving-system-vulnerability).
- GHSA-2hc8-mq24-76v2, high: the native UltraHonk verifier accepts proofs whose recursive pairing accumulator has one point at infinity. Fixed in 5.2.1 and in the 6.0.0 nightlies from 20260721 — [advisory](https://github.com/AztecProtocol/aztec-packages/security/advisories/GHSA-2hc8-mq24-76v2).
- The only third-party Barretenberg audit report I retrieved a link for is zkSecurity's Bigfield review of July 2024, out of scope for sumcheck. Search summary — [zkSecurity via CertiK](https://skynet.certik.com/third-party-audit-reports/jqfm0se0f03r/4yTkoeodbu4E06Kfig9muv/982245d97d3f1a872f0b30486d15594c/ZKSecurity_Audit_Report_-_Barretenberg_Bigfield.pdf).
- An unrelated project's adversarial audit of the bb 5.2.0 ZK Honk verifier reports no soundness bug. Secondary, search summary — [libID-contracts PR 61](https://github.com/libid-org/libID-contracts/pull/61).

### Inferences
- Neither backend can be called production-ready in the paper. Both should be described with their own caveats: the circuit stack by its own security policy, the zkVM by its advisories.
- Bug history is a real argument on both sides. A zkVM's circuit is shared and audited by many, but a bug breaks every guest. A policy compiler adds a new trusted translator, plus the SHA-256 gadget, on top of Noir and Barretenberg. The zkVM's trusted base is larger in lines but more reviewed. I found no paper that measures this trade-off.
- The project's pins sit after each fix named here. The project should recheck on each upgrade. Payy shows that a pinned verifier does not receive fixes on its own, and the small-subgroup fix changes the verification equation, so a stale verifier and a new prover would disagree visibly.
- E2's sentence that a sound SHA-256 gadget is a precondition for every number remains the correct framing. PR 62 shows the library had a real binding bug in an adjacent API as recently as July 2026, after the tag the project pins. Whether PR 62 touched paths used by `sha256_var` should be read from the diff.

### Gaps
- I found no audit of `noir-lang/sha256` or of Noir 1.0.0 as such. Private audits may exist.
- I did not determine whether audits of Barretenberg's Honk scope have completed. Only scope files are public.
- I did not read the Payy postmortem, the V5 details, or Consensys Diligence's April 2026 post (403). The attribution of the March bug rests on Aztec's own sentence.
- Nothing was found on formal verification of Noir or of UltraHonk as a whole. Lampe and similar work were not searched for.

## What does a Merkle commitment cost in a circuit, and what do standard benchmarks offer?

### Takeaway
On the project's own toolchain, the marginal cost per hash is about 3,946 gates for one SHA-256 compression, 2,368 for a 64-byte Blake3 and 73 for a 2-input Poseidon2. The standard benchmarks do not cover this. Hash choice cuts in opposite directions for the two backends, so it needs to be stated in any comparison.

### Cited Findings
- Local scratch measurement, Noir 1.0.0 and bb 6.0.0, `bb gates` `circuit_size`, loops of N hashes with the slope taken at N of 20 to 80. These are not project result files:

| Hash, per call | Gates per call | Floor for very small circuits |
|---|---|---|
| `std::hash::sha256_compression` (one 64-byte block) | 3,946 | 33,944 |
| `std::hash::blake2s`, 64-byte message | 3,184 | 17,408 |
| `std::hash::blake3`, 64-byte message | 2,368 | 17,408 |
| `poseidon::poseidon2::Poseidon2::hash`, 2 Fields, noir-lang/poseidon v0.4.0 | 73 | none seen |

- provebench gives 72,215 UltraHonk gates for SHA-256 over 1 KiB, and for a depth-20 Merkle proof 37,262 gates with Poseidon and 3,044 with Poseidon2, proving in 460 ms and 93 ms on an M4 MacBook Air. It uses Noir 1.0.0-beta.19 and bb 4.0.0-nightly. Single unreviewed repository. It does not compare SHA-256 and Poseidon2 on one workload, and it has no Blake3 — [provebench](https://github.com/agnij-dutta/provebench).
- E2 attributes nearly all gates to SHA-256, at about 80 compressions — [E2](file:///Users/connerswann/code/zk-pdp/docs/experiments/E2-tinytodo-in-noir.md).
- In the zkVM, an accelerated SHA-256 compression "typically takes 68 cycles per 64-byte block and 6 cycles to initialize" — [RISC Zero guide](https://dev.risczero.com/api/zkvm/optimization).
- Noir's Poseidon2 is a black-box function, and Barretenberg implements only t=4. One post gives about 2,313 gates per permutation without custom gates — [Aztec forum](https://forum.aztec.network/t/on-the-usability-differences-between-poseidon-and-poseidon2/8233).
- Poseidon — [ePrint 2019/458](https://eprint.iacr.org/2019/458). Poseidon2, LNCS 2023 — [ePrint 2023/323](https://eprint.iacr.org/2023/323), [DOI](https://doi.org/10.1007/978-3-031-37679-5_8). Alternatives: [Monolith, ePrint 2023/1025](https://eprint.iacr.org/2023/1025); [Rescue-Prime Optimized, ePrint 2022/1577](https://eprint.iacr.org/2022/1577).
- Sanso and Vitto attack reduced-round Poseidon and Poseidon2 bounty instances over NTT-friendly fields via Graeffe root-finding. This does not cover full rounds. BN254, which UltraHonk uses, should be checked against the paper's scope — [ePrint 2025/937](https://eprint.iacr.org/2025/937).
- zk-Harness covers Bellman, Circom, Gnark, Halo2-PSE and Starky, with exponentiation and SHA-256 circuits. No Noir. Developed at a Berkeley RDI hackathon, last date not shown — [zk-Harness](https://github.com/zkCollective/zk-Harness).
- zkalc, by Orrù and Kadianakis, estimates BLS12-381 primitive operations on an AWS m5.2xlarge. The visible page shows primitives, not hash circuits — [zka.lc](https://zka.lc/).
- Ethproofs tracks whole-block proving, not hash circuits — [Ethproofs](https://ethproofs.org/).

### Inferences
- A SHA-256 Merkle node with two 32-byte children needs two compressions when hashed as a 64-byte message, because of the padding block. That is about 7,892 gates against 2,368 for Blake3 and 73 for Poseidon2, ratios of roughly 108 to 32 to 1. provebench's gate counts are consistent with these slopes.
- A zkVM favours SHA-256 and a circuit punishes it. Choosing SHA-256 everywhere handicaps the circuit, and Poseidon2 everywhere would handicap the zkVM. A fair paper shows both, or fixes the external commitment format by need and reports the other as a variant.
- Moving the Merkle tree to Poseidon2 is the cheap change. The request hash has to stay byte-oriented to bind canonical JSON that outside parties can recompute. E2 suggests the remaining SHA-256 work would remain large for that reason.
- Poseidon2 is much younger than SHA-256, with active cryptanalysis of reduced-round versions. A paper should weigh that and not cite gate counts alone.

### Gaps
- I found no peer-reviewed benchmark of SHA-256, Blake3 and Poseidon2 on one UltraHonk toolchain. The table above should be rerun under project rules before it is cited.
- Proving time per hash was not measured. Lookup-heavy and custom-gate workloads may not scale identically per gate.
- PSE's client-side benchmarks were not found in search.

## What is the state of Metal and GPU proving on Apple silicon?

### Takeaway
RISC Zero's released versions do not prove on Metal, whatever the v3.0 docs say. `main` has Metal kernels, but the project's trial showed little GPU use and no finished proof. The best independent number is a self-reported 1.7x from an out-of-tree patch, which would not change the comparison.

### Cited Findings
- Issue 3753, "release-3.0 docs incorrectly claim Metal proving is enabled by default," opened May 16, 2026 by sputn1ck and still open. The Metal branches are commented out in the rv32im and recursion provers. The `metal` feature does not propagate. Confirmed on M5 Max and M4 Max — [risc0 issue 3753](https://github.com/risc0/risc0/issues/3753).
- The removal came in PRs 2952 and 3114, per the same issue. zkbench.dev listed RISC Zero with Metal in 2023, so this is a regression, not a gap that always existed — [issue 3753](https://github.com/risc0/risc0/issues/3753); [zkbench.dev](https://zkbench.dev/).
- The v3.0 docs still say RISC Zero uses the "integrated Metal compute cores" with no extra configuration — [RISC Zero local proving](https://dev.risczero.com/api/generating-proofs/local-proving). The docs PR is still open — [risc0 PR 3764](https://github.com/risc0/risc0/pull/3764).
- PR 3688, "Enable metal support," merged to `main` on January 30, 2026, repairs and hooks up recursion and m3 kernels and the Metal HAL. Its description says "The metal recursion witgen and poseidon_254 uses CPU." It contains no performance numbers — [risc0 PR 3688](https://github.com/risc0/risc0/pull/3688).
- v3.1.0-rc.1, published 2026-10-30, still has `// self::hal::metal::segment_prover(hashfn)` and the recursion equivalent commented out. `main` calls the Metal recursion prover and holds Metal sources under `risc0/circuit/rv32im-sys/cxx/hal/metal` — [source at v3.1.0-rc.1](https://github.com/risc0/risc0/blob/v3.1.0-rc.1/risc0/circuit/rv32im/src/prove/mod.rs); [`main` tree](https://github.com/risc0/risc0/tree/main/risc0/circuit).
- Project trial: `main` at 164c64b built after fetching Git LFS archives by hand and one API change. A 7.9 million cycle proof did not finish in 300 seconds, and the GPU monitor showed only brief small spikes — [E1](file:///Users/connerswann/code/zk-pdp/docs/experiments/E1-cedar-in-risc0.md).
- An outside contributor built a hybrid lane, GPU for STARK operations and CPU for circuit kernels, about 677 lines over 5 files. Self-reported 1.6x to 1.73x end to end, with about 87% of proof time in the CPU-bound `eval_check`. No maintainer response on the page — [issue 3753 comments](https://github.com/risc0/risc0/issues/3753).
- ICICLE-Metal v3.8.0 on an M3 MacBook Air: MSM at 2^20 takes 421 ms against 942 ms for Arkworks on CPU, and at 2^24 takes 4,945 ms against 14,061 ms. It is slower than CPU up to 2^16. Mopro calls it the state of the art for Metal MSM. July 28, 2025 — [Mopro](https://zkmopro.org/blog/metal-msm-v2/).

### Inferences
- The paper must say the zkVM run was CPU-only because released RISC Zero has no working Metal path, and that this is not a choice of the authors. A reader of the v3.0 docs would otherwise assume Metal was used.
- A 1.7x gain would move 28-plus minutes to about 16-plus. It would not close the gap. If 87 percent of time is CPU-bound `eval_check`, GPU help on the rest has a low ceiling, unless `main`'s full Metal kernels change this.
- ICICLE's MSM numbers matter for pairing-based proving, including Groth16 and possibly UltraHonk, not for RISC Zero's hash-and-FRI core. They should not be cited as evidence about STARK proving on Metal.
- The circuit prover also ran CPU only, so both arms are CPU. That is fair and should be stated.

### Gaps
- I found no independent, published benchmark of RISC Zero on Metal from any version, and no paper covering STARK proving on Apple GPUs.
- Whether `main` proves segments on Metal end to end is not established. PR 3688's text and the project's trial suggest partial use at best.
- No ICICLE NTT figures on Metal were found, and no benchmark of Barretenberg on Metal.

## What is the verified bibliography?

### Takeaway
73 entries follow. Metadata came from ePrint OAI, Crossref, OpenAlex or the paper's own first page. Fields I could not verify are left out, so some entries lack a venue I would otherwise expect. Papers I believe appeared at a venue but could not verify are listed as ePrint only.

Status definitions:

- `read-full`: the full text was processed through a fetch tool's summary. Nothing here was read line by line.
- `read-abstract`: the abstract was read. "(opening)" means only its first 420 characters. "+ passages" means I also read targeted parts of the full text.
- `search-summary-only`: metadata is verified, but the content was seen only through search summaries, or not at all.

### Cited Findings

```bibtex
% ===== zkVM designs =====
@misc{bruestle2023risczero, author={Jeremy Bruestle and Paul Gafni and {RISC Zero Team}}, title={{RISC Zero zkVM}: Scalable, Transparent Arguments of {RISC-V} Integrity}, year={2023}, howpublished={Draft, August 11, 2023}, url={https://dev.risczero.com/proof-system-in-detail.pdf}, note={read-abstract + passages}}
@inproceedings{arun2024jolt, author={Arasu Arun and Srinath Setty and Justin Thaler}, title={Jolt: {SNARKs} for Virtual Machines via Lookups}, booktitle={Advances in Cryptology -- EUROCRYPT 2024}, series={LNCS}, pages={3--33}, year={2024}, doi={10.1007/978-3-031-58751-1_1}, eprint={2023/1217}, note={read-abstract (opening)}}
@inproceedings{setty2024lasso, author={Srinath Setty and Justin Thaler and Riad Wahby}, title={Unlocking the Lookup Singularity with {Lasso}}, booktitle={Advances in Cryptology -- EUROCRYPT 2024}, series={LNCS}, pages={180--209}, year={2024}, doi={10.1007/978-3-031-58751-1_7}, eprint={2023/1216}, note={read-abstract (opening)}}
@inproceedings{setty2026twist, author={Srinath Setty and Justin Thaler and Michael Zhu}, title={Twist and Shout: Faster Memory Checking Arguments via One-Hot Addressing and Increments}, booktitle={Advances in Cryptology -- CRYPTO 2026}, series={LNCS}, pages={267--296}, year={2026}, doi={10.1007/978-3-032-35424-2_10}, eprint={2025/105}, note={read-abstract (opening)}}
@article{liu2025ceno, author={Tianyi Liu and Zhenfei Zhang and Yuncong Zhang and Wenqing Hu and Ye Zhang}, title={Ceno: Non-uniform, Segment and Parallel Zero-Knowledge Virtual Machine}, journal={Journal of Cryptology}, year={2025}, doi={10.1007/s00145-024-09533-2}, eprint={2024/387}, note={read-abstract (opening)}}
@misc{goldberg2021cairo, author={Lior Goldberg and Shahar Papini and Michael Riabzev}, title={Cairo -- a {Turing}-complete {STARK}-friendly {CPU} architecture}, year={2021}, howpublished={Cryptology ePrint Archive, Paper 2021/1063}, url={https://eprint.iacr.org/2021/1063}, note={read-abstract (opening)}}
@misc{habock2024circle, author={Ulrich Hab{\"o}ck and David Levit and Shahar Papini}, title={Circle {STARKs}}, year={2024}, howpublished={Cryptology ePrint Archive, Paper 2024/278}, url={https://eprint.iacr.org/2024/278}, note={read-abstract (opening)}}
@inproceedings{diamond2025towers, author={Benjamin E. Diamond and Jim Posen}, title={Succinct Arguments over Towers of Binary Fields}, booktitle={Advances in Cryptology -- EUROCRYPT 2025}, series={LNCS}, pages={93--122}, year={2025}, doi={10.1007/978-3-031-91134-7_4}, eprint={2023/1784}, note={read-abstract (opening)}}
@inproceedings{diamond2026polylog, author={Benjamin E. Diamond and Jim Posen}, title={Polylogarithmic Proofs for Multilinears over Binary Towers}, series={LNCS}, pages={3--32}, year={2026}, doi={10.1007/978-3-032-25336-1_1}, eprint={2024/504}, note={read-abstract (opening)}}
@inproceedings{bunz2024deepthought, author={Benedikt B{\"u}nz and Jessica Chen}, title={Proofs for Deep Thought: Accumulation for Large Memories and Deterministic Computations}, series={LNCS}, pages={269--301}, year={2024}, doi={10.1007/978-981-96-0935-2_9}, eprint={2024/325}, note={read-abstract (opening)}}
@inproceedings{arun2026nebula, author={Arasu Arun and Srinath Setty}, title={Nebula: Proving Machine Executions via Folding Schemes}, booktitle={2026 IEEE Symposium on Security and Privacy (SP)}, pages={3223--3242}, year={2026}, doi={10.1109/sp63933.2026.00099}, note={search-summary-only}}
@misc{bensasson2014neumann, author={Eli Ben-Sasson and Alessandro Chiesa and Eran Tromer and Madars Virza}, title={Succinct Non-Interactive Zero Knowledge for a von {Neumann} Architecture}, year={2014}, url={https://www.usenix.org/conference/usenixsecurity14/technical-sessions/presentation/ben-sasson}, note={search-summary-only}}
@inproceedings{parno2013pinocchio, author={B. Parno and J. Howell and C. Gentry and M. Raykova}, title={Pinocchio: Nearly Practical Verifiable Computation}, booktitle={2013 IEEE Symposium on Security and Privacy}, pages={238--252}, year={2013}, doi={10.1109/sp.2013.47}, note={search-summary-only; metadata only; Crossref gives initials}}

% ===== Surveys, evaluations, benchmarks =====
@inproceedings{yang2026sok, author={Yunbo Yang and Yuejia Cheng and Haibo Tang and Guomin Yang and Bingsheng Zhang and Kui Ren}, title={{SoK}: Understanding {zkVM}: From Research to Practice}, booktitle={Proceedings of the ACM Asia Conference on Computer and Communications Security}, pages={852--868}, year={2026}, doi={10.1145/3779208.3805964}, eprint={2026/525}, note={read-abstract; PDF and ACM page blocked; table fragments seen by search summary}}
@inproceedings{jiang2026zkvms, author={Jinan Jiang and Jinzhao Chu and Pengyu Xue and Xinghao Peng and Haoran Qin and Xiapu Luo}, title={Zero-Knowledge {VMs}: Security and Usability in Practice}, booktitle={Proceedings of the 41st IEEE/ACM International Conference on Automated Software Engineering}, pages={2587--2599}, year={2026}, doi={10.1145/3832783.3837500}, note={search-summary-only; metadata only}}
@inproceedings{gassmann2026compiler, author={Thomas Gassmann and Stefanos Chaliasos and Thodoris Sotiropoulos and Zhendong Su}, title={Evaluating Compiler Optimization Impacts on {zkVM} Performance}, booktitle={Proceedings of the 31st ACM International Conference on Architectural Support for Programming Languages and Operating Systems, Volume 2}, pages={697--714}, year={2026}, doi={10.1145/3779212.3790159}, archiveprefix={arXiv}, eprint={2508.17518}, note={read-full; first 94 percent}}
@misc{sheybani2025frameworks, author={Nojan Sheybani and Anees Ahmed and Michel Kinsy and Farinaz Koushanfar}, title={Zero-Knowledge Proof Frameworks: A Systematic Survey}, year={2025}, archiveprefix={arXiv}, eprint={2502.07063}, doi={10.48550/arXiv.2502.07063}, note={read-abstract + passages (evaluation section)}}
@inproceedings{liang2025sok, author={Junkai Liang and Daqi Hu and Pengfei Wu and Yunbo Yang and Qingni Shen and Zhonghai Wu}, title={{SoK}: Understanding {zk-SNARKs}: The Gap Between Research and Practice}, booktitle={34th USENIX Security Symposium}, year={2025}, url={https://www.usenix.org/conference/usenixsecurity25/presentation/liang-sok}, archiveprefix={arXiv}, eprint={2502.02387}, note={read-abstract}}
@misc{hassanzadeh2026constraint, author={Yahya Hassanzadeh-Nazarabadi and Sanaz Taheri-Boshrooyeh}, title={Constraint-Level Design of {zkEVMs}: Architectures, Trade-offs, and Evolution}, year={2026}, archiveprefix={arXiv}, eprint={2510.05376}, note={read-abstract + passages}}
@inproceedings{ernstberger2024zkbench, author={Jens Ernstberger and Stefanos Chaliasos and George Kadianakis and Sebastian Steinhorst and Philipp Jovanovic and Arthur Gervais and Benjamin Livshits and Michele Orr{\`u}}, title={{zk-Bench}: A Toolset for Comparative Evaluation and Performance Benchmarking of {SNARKs}}, booktitle={Security and Cryptography for Networks}, series={LNCS}, pages={46--72}, year={2024}, doi={10.1007/978-3-031-71070-4_3}, eprint={2023/1503}, note={read-abstract}}
@misc{chaliasos2024rollups, author={Stefanos Chaliasos and Itamar Reif and Adri{\`a} Torralba-Agell and Jens Ernstberger and Assimakis Kattis and Benjamin Livshits}, title={Analyzing and Benchmarking {ZK-Rollups}}, year={2024}, howpublished={Cryptology ePrint Archive, Paper 2024/889}, url={https://eprint.iacr.org/2024/889}, note={read-abstract}}
@misc{babbush2026quantum, author={Ryan Babbush and Adam Zalcman and Craig Gidney and Michael Broughton and Tanuj Khattar and Hartmut Neven and Thiago Bergamaschi and Justin Drake and Dan Boneh}, title={Securing Elliptic Curve Cryptocurrencies against Quantum Vulnerabilities: Resource Estimates and Mitigations}, year={2026}, archiveprefix={arXiv}, eprint={2603.28846}, note={read-abstract + passages (Appendix A on SP1 and Groth16)}}

% ===== Authorization in zkVM; parsing =====
@misc{wei2026prezta, author={Zhongjing Wei and Osaid Muhammad Ameer and Nikita Borisov and Yupeng Zhang}, title={Prezta: Provable Remote Execution of Zero-Trust Authorization using {SNARKs}}, year={2026}, archiveprefix={arXiv}, eprint={2607.11466}, note={read-abstract + passages (Sections 4.2 and 5). Author list from first page; verify against the published version, which references.bib lists at USENIX Security}}
@article{frigo2026ecdsa, author={Matteo Frigo and abhi shelat}, title={Anonymous Credentials from {ECDSA}}, journal={IACR Communications in Cryptology}, volume={3}, year={2026}, doi={10.62056/a3qjmpgxq}, eprint={2024/2010}, note={read-abstract}}
@misc{kaviani2025vega, author={Darya Kaviani and Srinath Setty}, title={Vega: Low-Latency Zero-Knowledge Proofs over Existing Credentials}, year={2025}, howpublished={Cryptology ePrint Archive, Paper 2025/2094}, url={https://eprint.iacr.org/2025/2094}, note={read-abstract}}
@inproceedings{angel2026coral, author={Sebastian Angel and Sof{\'i}a Celi and Elizabeth Margolin and Pratyush Mishra and Martin Sander and Jess Woods}, title={Coral: Fast Succinct Non-Interactive Zero-Knowledge {CFG} Proofs}, booktitle={2026 IEEE Symposium on Security and Privacy (SP)}, pages={1882--1901}, year={2026}, doi={10.1109/sp63933.2026.00059}, eprint={2025/1420}, note={read-abstract}}
@misc{angel2023reef, author={Sebastian Angel and Eleftherios Ioannidis and Elizabeth Margolin and Srinath Setty and Jess Woods}, title={Reef: Fast Succinct Non-Interactive Zero-Knowledge Regex Proofs}, year={2023}, howpublished={Cryptology ePrint Archive, Paper 2023/1886}, url={https://eprint.iacr.org/2023/1886}, note={read-abstract (opening)}}
@inproceedings{delignat2016cinderella, author={Antoine Delignat-Lavaud and Cedric Fournet and Markulf Kohlweiss and Bryan Parno}, title={Cinderella: Turning Shabby {X.509} Certificates into Elegant Anonymous Credentials with the Magic of Verifiable Computation}, booktitle={2016 IEEE Symposium on Security and Privacy (SP)}, pages={235--254}, year={2016}, doi={10.1109/sp.2016.22}, note={search-summary-only; metadata only}}
@inproceedings{baldimtsi2024zklogin, author={Foteini Baldimtsi and Konstantinos Kryptos Chalkias and Yan Ji and Jonas Lindstr{\o}m and Deepak Maram and Ben Riva and Arnab Roy and Mahdi Sedaghat and Joy Wang}, title={{zkLogin}: Privacy-Preserving Blockchain Authentication with Existing Credentials}, booktitle={ACM SIGSAC Conference on Computer and Communications Security (CCS)}, year={2024}, doi={10.1145/3658644.3690356}, note={search-summary-only; metadata only}}

% ===== Zero-knowledge =====
@misc{habock2024note, author={Ulrich Hab{\"o}ck and Al Kindi}, title={A note on adding zero-knowledge to {STARKs}}, year={2024}, howpublished={Cryptology ePrint Archive, Paper 2024/1037, revised 2025-02-22}, url={https://eprint.iacr.org/2024/1037}, note={read-abstract; PDF blocked; body claims only via search summary. Must be read by a person before citing specifics}}
@misc{dalton2026fault, author={Alexander Dalton and Markus Schofnegger and Daniel Page}, title={Fault Injection Attacks Against {zkSTARKs}}, year={2026}, howpublished={Cryptology ePrint Archive, Paper 2026/835}, url={https://eprint.iacr.org/2026/835}, note={read-abstract}}
@inproceedings{groth2016size, author={Jens Groth}, title={On the Size of Pairing-Based Non-interactive Arguments}, booktitle={Advances in Cryptology -- EUROCRYPT 2016}, series={LNCS}, pages={305--326}, year={2016}, doi={10.1007/978-3-662-49896-5_11}, eprint={2016/260}, note={read-abstract (opening)}}
@inproceedings{fuchsbauer2018subversion, author={Georg Fuchsbauer}, title={Subversion-Zero-Knowledge {SNARKs}}, series={LNCS}, pages={315--347}, year={2018}, doi={10.1007/978-3-319-76578-5_11}, eprint={2017/587}, note={read-abstract}}
@inproceedings{abdolmaleki2017subversion, author={Behzad Abdolmaleki and Karim Baghery and Helger Lipmaa and Micha{\l} Zaj{\k{a}}c}, title={A Subversion-Resistant {SNARK}}, series={LNCS}, pages={3--33}, year={2017}, doi={10.1007/978-3-319-70700-6_1}, eprint={2017/599}, note={search-summary-only; metadata only}}
@inproceedings{baghery2021another, author={Karim Baghery and Markulf Kohlweiss and Janno Siim and Mikhail Volkhov}, title={Another Look at Extraction and Randomization of {Groth}'s {zk-SNARK}}, series={LNCS}, pages={457--475}, year={2021}, doi={10.1007/978-3-662-64322-8_22}, note={search-summary-only; metadata only}}
@inproceedings{xie2019libra, author={Tiacheng Xie and Jiaheng Zhang and Yupeng Zhang and Charalampos Papamanthou and Dawn Song}, title={Libra: Succinct Zero-Knowledge Proofs with Optimal Prover Computation}, series={LNCS}, pages={733--764}, year={2019}, doi={10.1007/978-3-030-26954-8_24}, eprint={2019/317}, note={read-abstract (opening). Crossref spells the first author Tiacheng; ePrint spells Tiancheng}}
@misc{chiesa2017sumcheck, author={Alessandro Chiesa and Michael A. Forbes and Nicholas Spooner}, title={A Zero Knowledge Sumcheck and its Applications}, year={2017}, archiveprefix={arXiv}, eprint={1704.02086}, doi={10.48550/arxiv.1704.02086}, howpublished={Also Cryptology ePrint Archive, Paper 2017/305}, note={read-abstract (opening)}}
@misc{bootle2022gemini, author={Jonathan Bootle and Alessandro Chiesa and Yuncong Hu and Michele Orr{\`u}}, title={Gemini: Elastic {SNARKs} for Diverse Environments}, year={2022}, howpublished={Cryptology ePrint Archive, Paper 2022/420}, url={https://eprint.iacr.org/2022/420}, note={read-abstract (opening)}}
@misc{boneh2020shplonk, author={Dan Boneh and Justin Drake and Ben Fisch and Ariel Gabizon}, title={Efficient polynomial commitment schemes for multiple points and polynomials}, year={2020}, howpublished={Cryptology ePrint Archive, Paper 2020/081}, url={https://eprint.iacr.org/2020/081}, note={search-summary-only; metadata only}}
@misc{kohrita2023zeromorph, author={Tohru Kohrita and Patrick Towa}, title={Zeromorph: Zero-Knowledge Multilinear-Evaluation Proofs from Homomorphic Univariate Commitments}, year={2023}, howpublished={Cryptology ePrint Archive, Paper 2023/917}, url={https://eprint.iacr.org/2023/917}, note={read-abstract (opening)}}
@inproceedings{chen2023hyperplonk, author={Binyi Chen and Benedikt B{\"u}nz and Dan Boneh and Zhenfei Zhang}, title={{HyperPlonk}: Plonk with Linear-Time Prover and High-Degree Custom Gates}, booktitle={Advances in Cryptology -- EUROCRYPT 2023}, series={LNCS}, pages={499--530}, year={2023}, doi={10.1007/978-3-031-30617-4_17}, eprint={2022/1355}, note={read-abstract (opening)}}
@misc{gabizon2019plonk, author={Ariel Gabizon and Zachary J. Williamson and Oana Ciobotaru}, title={{PLONK}: Permutations over {Lagrange}-bases for Oecumenical Noninteractive arguments of Knowledge}, year={2019}, howpublished={Cryptology ePrint Archive, Paper 2019/953}, url={https://eprint.iacr.org/2019/953}, note={search-summary-only; metadata only}}
@misc{eagen2023protogalaxy, author={Liam Eagen and Ariel Gabizon}, title={{ProtoGalaxy}: Efficient {ProtoStar}-style folding of multiple instances}, year={2023}, howpublished={Cryptology ePrint Archive, Paper 2023/1106}, url={https://eprint.iacr.org/2023/1106}, note={read-abstract (opening)}}

% ===== Fiat-Shamir =====
@inproceedings{khovratovich2025false, author={Dmitry Khovratovich and Ron D. Rothblum and Lev Soukhanov}, title={How to Prove False Statements: Practical Attacks on {Fiat-Shamir}}, booktitle={Advances in Cryptology -- CRYPTO 2025}, series={LNCS}, pages={3--26}, year={2025}, doi={10.1007/978-3-032-01887-8_1}, eprint={2025/118}, note={read-abstract}}
@inproceedings{dao2023weak, author={Quang Dao and Jim Miller and Opal Wright and Paul Grubbs}, title={Weak {Fiat-Shamir} Attacks on Modern Proof Systems}, booktitle={2023 IEEE Symposium on Security and Privacy (SP)}, pages={199--216}, year={2023}, doi={10.1109/sp46215.2023.10179408}, eprint={2023/691}, note={read-abstract (opening)}}
@inproceedings{block2023fri, author={Alexander R. Block and Albert Garreta and Jonathan Katz and Justin Thaler and Pratyush Ranjan Tiwari and Micha{\l} Zaj{\k{a}}c}, title={{Fiat-Shamir} Security of {FRI} and Related {SNARKs}}, booktitle={Advances in Cryptology -- ASIACRYPT 2023}, series={LNCS}, pages={3--40}, year={2023}, doi={10.1007/978-981-99-8724-5_1}, eprint={2023/1071}, note={read-abstract (opening)}}
@inproceedings{brzuska2026simple, author={Chris Brzuska and Pavel Hub{\'a}{\v{c}}ek and Aleksi Kalsta}, title={Simple Attacks Against (Extended) {Fiat-Shamir}}, booktitle={Public-Key Cryptography -- PKC 2026}, series={LNCS}, pages={510--543}, year={2026}, doi={10.1007/978-3-032-26731-3_17}, note={search-summary-only}}
@misc{arnon2025whitebox, author={Gal Arnon and Eylon Yogev}, title={Towards a White-Box Secure {Fiat-Shamir} Transformation}, year={2025}, howpublished={Cryptology ePrint Archive, Paper 2025/329}, url={https://eprint.iacr.org/2025/329}, note={read-abstract (opening)}}
@misc{ghosal2026instantiate, author={Riddhi Ghosal and Aniket Kate and Pratyay Mukherjee and Pratik Sarkar and Rohit Sinha}, title={How to instantiate {Fiat-Shamir} Provably and Practically?}, year={2026}, howpublished={Cryptology ePrint Archive, Paper 2026/2114}, url={https://eprint.iacr.org/2026/2114}, note={read-abstract}}

% ===== Proximity gaps and FRI =====
@article{bensasson2023proximity, author={Eli Ben-Sasson and Dan Carmon and Yuval Ishai and Swastik Kopparty and Shubhangi Saraf}, title={Proximity Gaps for {Reed--Solomon} Codes}, journal={Journal of the ACM}, volume={70}, pages={1--57}, year={2023}, doi={10.1145/3614423}, eprint={2020/654}, note={read-abstract (opening). Also FOCS 2020, pp. 900--909, doi 10.1109/focs46700.2020.00088}}
@misc{diamond2025distribution, author={Benjamin E. Diamond and Angus Gruen}, title={On the Distribution of the Distances of Random Words}, year={2025}, howpublished={Cryptology ePrint Archive, Paper 2025/2010}, url={https://eprint.iacr.org/2025/2010}, note={read-abstract}}
@misc{crites2025conjectures, author={Elizabeth Crites and Alistair Stewart}, title={On {Reed--Solomon} Proximity Gaps Conjectures}, year={2025}, howpublished={Cryptology ePrint Archive, Paper 2025/2046}, url={https://eprint.iacr.org/2025/2046}, note={read-abstract}}
@misc{bensasson2025gaps, author={Eli Ben-Sasson and Dan Carmon and Ulrich Hab{\"o}ck and Swastik Kopparty and Shubhangi Saraf}, title={On Proximity Gaps for {Reed--Solomon} Codes}, year={2025}, howpublished={Cryptology ePrint Archive, Paper 2025/2055}, url={https://eprint.iacr.org/2025/2055}, note={read-abstract}}
@misc{krachun2026failure, author={Dmitry Krachun and Stepan Kazanin and Ulrich Hab{\"o}ck}, title={Failure of proximity gaps close to capacity}, year={2026}, howpublished={Cryptology ePrint Archive, Paper 2026/782}, url={https://eprint.iacr.org/2026/782}, note={read-abstract}}
@misc{kambire2026fails, author={Antonio Kambir{\'e}}, title={Proximity Gaps Conjecture Fails Near Capacity over Prime Fields}, year={2026}, archiveprefix={arXiv}, eprint={2604.09724}, note={read-full}}
@misc{okamoto2025syndrome, author={Russell Okamoto}, title={The Syndrome-Space Lens: A Complete Resolution of Proximity Gaps for {Reed-Solomon} Codes}, year={2025}, howpublished={Cryptology ePrint Archive, Paper 2025/1712}, url={https://eprint.iacr.org/2025/1712}, note={read-abstract (opening). Unvetted, conflicts on its face with the counterexamples}}
@inproceedings{goyal2026subspace, author={Rohan Goyal and Venkatesan Guruswami}, title={Optimal Proximity Gaps for Subspace-Design Codes and (Random) {Reed-Solomon} Codes}, booktitle={Proceedings of the 58th Annual ACM Symposium on Theory of Computing}, pages={1558--1568}, year={2026}, doi={10.1145/3798129.3800864}, note={search-summary-only; metadata only}}
@misc{garreta2025fri, author={Albert Garreta and Nicolas Mohnblatt and Benedikt Wagner}, title={A Simplified Round-by-round Soundness Proof of {FRI}}, year={2025}, howpublished={Cryptology ePrint Archive, Paper 2025/1993}, url={https://eprint.iacr.org/2025/1993}, note={read-abstract (opening)}}
@inproceedings{arnon2025whir, author={Gal Arnon and Alessandro Chiesa and Giacomo Fenzi and Eylon Yogev}, title={{WHIR}: {Reed--Solomon} Proximity Testing with Super-Fast Verification}, booktitle={Advances in Cryptology -- EUROCRYPT 2025}, series={LNCS}, pages={214--243}, year={2025}, doi={10.1007/978-3-031-91134-7_8}, eprint={2024/1586}, note={read-abstract (opening)}}

% ===== Soundness, testing, verification =====
@misc{chaliasos2024sok, author={Stefanos Chaliasos and Jens Ernstberger and David Theodore and David Wong and Mohammad Jahanara and Benjamin Livshits}, title={{SoK}: What don't we know? Understanding Security Vulnerabilities in {SNARKs}}, year={2024}, archiveprefix={arXiv}, eprint={2402.15293}, doi={10.48550/arXiv.2402.15293}, note={read-abstract. arXiv lists no journal reference}}
@misc{tang2024vulnerability, author={Xueyan Tang and Lingzhi Shi and Xun Wang and Kyle Charbonnet and Shixiang Tang and Shixiao Sun}, title={Zero-Knowledge Proof Vulnerability Analysis and Security Auditing}, year={2024}, howpublished={Cryptology ePrint Archive, Paper 2024/514}, url={https://eprint.iacr.org/2024/514}, note={read-abstract (opening)}}
@misc{hochrainer2025arguzz, author={Christoph Hochrainer and Valentin W{\"u}stholz and Maria Christakis}, title={Arguzz: Testing {zkVMs} for Soundness and Completeness Bugs}, year={2025}, archiveprefix={arXiv}, eprint={2509.10819}, doi={10.48550/arxiv.2509.10819}, note={read-abstract + passages}}
@inproceedings{hochrainer2025circuzz, author={Christoph Hochrainer and Anastasia Isychev and Valentin W{\"u}stholz and Maria Christakis}, title={Fuzzing Processing Pipelines for Zero-Knowledge Circuits}, booktitle={Proceedings of the 2025 ACM SIGSAC Conference on Computer and Communications Security}, pages={783--797}, year={2025}, doi={10.1145/3719027.3744791}, note={search-summary-only; metadata only}}
@misc{takahashi2026zebra, author={Hideaki Takahashi and Suman Jana and Junfeng Yang}, title={Efficient Branch-and-Bound Testing and Verification of {zkVMs}}, year={2026}, archiveprefix={arXiv}, eprint={2609.15020}, doi={10.48550/arXiv.2609.15020}, note={read-abstract + passages; first 59 percent via fetch summary}}
@misc{kolozyan2026tools, author={Arman Kolozyan and Tom Sorger and Alexander Hicks and Stefanos Chaliasos}, title={{ZKP} Security Tools and Verification: Coverage, Effectiveness, Adoption, and Challenges}, year={2026}, archiveprefix={arXiv}, eprint={2607.23752}, note={read-abstract + passages; first 73 percent via fetch summary}}
@misc{watzinger2026lie, author={Sebastian Watzinger and Christoph Hochrainer and Valentin W{\"u}stholz and Maria Christakis}, title={Lie to Me: Finding Bugs in {ZK DSL} Toolchains with Adversarial Witness Injection}, year={2026}, archiveprefix={arXiv}, eprint={2608.30648}, note={read-abstract}}
@article{pailoor2023picus, author={Shankara Pailoor and Yanju Chen and Franklyn Wang and Clara Rodr{\'i}guez and Jacob Van Geffen and Jason Morton and Michael Chu and Brian Gu}, title={Automated Detection of Under-Constrained Circuits in Zero-Knowledge Proofs}, journal={Proceedings of the ACM on Programming Languages}, volume={7}, pages={1510--1532}, year={2023}, doi={10.1145/3591282}, note={search-summary-only; metadata only}}
@inproceedings{chaliasos2025fuzz, author={Stefanos Chaliasos and Imam Al-Fath and Alastair F. Donaldson}, title={Towards Fuzzing Zero-Knowledge Proof Circuits (Short Paper)}, booktitle={ACM SIGSOFT International Symposium on Software Testing and Analysis (ISSTA)}, year={2025}, doi={10.1145/3713081.3731718}, note={search-summary-only; metadata only}}

% ===== Hashes =====
@misc{grassi2019poseidon, author={Lorenzo Grassi and Dmitry Khovratovich and Christian Rechberger and Arnab Roy and Markus Schofnegger}, title={Poseidon: A New Hash Function for Zero-Knowledge Proof Systems}, year={2019}, howpublished={Cryptology ePrint Archive, Paper 2019/458. OpenAlex also lists a USENIX Security location}, url={https://eprint.iacr.org/2019/458}, note={read-abstract (opening)}}
@inproceedings{grassi2023poseidon2, author={Lorenzo Grassi and Dmitry Khovratovich and Markus Schofnegger}, title={{Poseidon2}: A Faster Version of the {Poseidon} Hash Function}, series={LNCS}, pages={177--203}, year={2023}, doi={10.1007/978-3-031-37679-5_8}, eprint={2023/323}, note={read-abstract (opening)}}
@misc{grassi2023monolith, author={Lorenzo Grassi and Dmitry Khovratovich and Reinhard L{\"u}ftenegger and Christian Rechberger and Markus Schofnegger and Roman Walch}, title={Monolith: Circuit-Friendly Hash Functions with New Nonlinear Layers for Fast and Constant-Time Implementations}, year={2023}, howpublished={Cryptology ePrint Archive, Paper 2023/1025}, url={https://eprint.iacr.org/2023/1025}, note={search-summary-only; metadata only}}
@misc{ashur2022rpo, author={Tomer Ashur and Al Kindi and Willi Meier and Alan Szepieniec and Bobbin Threadbare}, title={Rescue-Prime Optimized}, year={2022}, howpublished={Cryptology ePrint Archive, Paper 2022/1577}, url={https://eprint.iacr.org/2022/1577}, note={search-summary-only; metadata only}}
@misc{sanso2025graeffe, author={Antonio Sanso and Giuseppe Vitto}, title={Attacking {Poseidon} via {Graeffe}-Based Root-Finding over {NTT}-Friendly Fields}, year={2025}, howpublished={Cryptology ePrint Archive, Paper 2025/937}, url={https://eprint.iacr.org/2025/937}, note={read-abstract}}
```

Non-paper primary sources, for `@misc` entries the report writer can build: the GitHub advisories, issues and PRs linked above; [RISC Zero v3.0 docs](https://dev.risczero.com/api/security-model); [SP1 security model](https://docs.succinct.xyz/docs/sp1/security/security-model); [bb CLI options](https://github.com/AztecProtocol/aztec-packages/blob/next/barretenberg/docs/docs/cli_options.md); [EF zkEVM blog](https://blog.ethereum.org/2025/12/18/zkevm-security-foundations).

Vendor and secondary sources, to be labelled as such in the paper: Succinct, a16z and The Block, Category Labs, Base, Mopro, Polybase's zkbench, provebench, Veridise, Certora.

### Inferences
- 35 of the 73 entries are typed as conference or journal publications, all confirmed by Crossref, OpenAlex or the publisher's page. The most recent and most relevant zero-knowledge and proximity results are preprints. The paper should say so where it leans on them.

### Gaps
- `eprint` fields on entries with a DOI come from ePrint OAI title matches. I did not check that each ePrint version is identical to the published one.
- Fuchsbauer, Abdolmaleki, Baghery, Bünz and Chen, Diamond and Posen (2026), and Poseidon2 are in LNCS, but Crossref's output in this run did not name the conference in those records. I left `booktitle` out rather than guess.
- Haböck and Kindi's venue, if any, is unknown. ePrint shows only the archive.
