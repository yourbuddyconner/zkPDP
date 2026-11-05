# Compiling high-level languages and logics to zero-knowledge circuits, and verifying those compilers and circuits

Notes for the zk-pdp prior-art report. Checked on 2026-11-04.

**How to read the status marks.** No paper was read cover to cover.
- `read-full`: I retrieved the full text and read directly the sections behind the claims here (abstract, introduction, and the design, results or limits sections).
- `read-abstract`: I read the abstract, and sometimes a few located passages, which the entry notes.
- `search-summary-only`: I saw a search summary or only a bibliographic record. Claims resting on these are weaker.

**How metadata was checked.** Crossref records, ePrint and arXiv pages, USENIX pages and the papers' title pages. DBLP blocked automated access, so nothing here comes from DBLP. Papers I could not confirm are listed as unverified leads, with no BibTeX entry.

**Corrections to the brief's premises.**
- ZKAP is a static analyzer for Circom circuits, not a ZK SQL system. See [Wen et al.](https://www.usenix.org/conference/usenixsecurity24/presentation/wen). The USENIX page does not use the name; [Kolozyan et al.](https://arxiv.org/abs/2607.23752) and [NAVe](https://arxiv.org/abs/2601.09372) do.
- "Clap" names two projects: the [Matter Labs Rust eDSL with an Agda model](https://arxiv.org/abs/2405.12115), and a [Nethermind Lean project for Aptos Keyless circuits](https://www.nethermind.io/blog/clap-correctly-compiling-the-leanest-possible-circuits).
- Ecne is a 2022 tool. [Ozdemir et al.](https://eprint.iacr.org/2023/778) and the [SoK](https://arxiv.org/abs/2402.15293) cite it by author and repository. I found no peer-reviewed paper for it.
- I found no paper called "zkBuild". The closest work is [Ron and Monperrus](https://arxiv.org/abs/2602.11887).

## Objective: what is established and what is open

### Takeaway
Compiling to circuits is mature engineering, but no compiler from a mainstream language to R1CS or Plonkish is verified end to end. Proving compilation has two near precedents from 2026, and neither proves that the output means what the source means. zk-pdp would be new in two respects, as far as I found: an authorization language compiled to a circuit through a verified front end, and a compilation proof that binds a policy hash to a verification key.

### Cited Findings

**(a) DSL and SMT-to-circuit compilation: established**
- SMT and circuit targets share an abstraction, so one compiler can serve both. CirC calls them stateless, non-deterministic, non-uniform targets and lowers one IR to R1CS and to SMT — [CirC](https://par.nsf.gov/servlets/purl/10432512).
- A shared IR makes front ends cheap: CirC's ZoKrates front end took about a week and 900 lines, against about 24,000 lines in the reference compiler, with slightly better R1CS for ZoKrates programs — [CirC](https://par.nsf.gov/servlets/purl/10432512).
- A circuit need not be per-program: Lurk gives programs as data to one universal interpreter circuit — [Lurk](https://eprint.iacr.org/2023/369). Cairo uses one set of polynomial equations for a whole CPU — [Cairo](https://eprint.iacr.org/2021/1063).

**(a) Open**
- Most security tooling still targets Circom, and newer DSLs and zkVMs have limited support — [Kolozyan et al.](https://arxiv.org/abs/2607.23752).
- I found no published compiler that takes an SMT-like policy IR to Noir. See Gaps under the policy question.

**(b) Verified or certified compilation: established**
- Pieces are verified, not whole pipelines. CirC's field-blaster is verified only for bounded sizes (bit-width and arity of at most 4), and that work found 4 bugs, one a severe soundness bug — [Ozdemir et al.](https://eprint.iacr.org/2023/778).
- Clap proves soundness and completeness of its translation step in an Agda model — [Clap](https://arxiv.org/abs/2405.12115). The paper notes the Agda model only mimics the implementation (full text, Agda section).
- Cairo's and S-two's algebraic encodings have machine-checked soundness proofs — [NAVe's account of Avigad et al. 2022](https://arxiv.org/abs/2601.09372), [Avigad et al. 2026](https://arxiv.org/abs/2606.04311).
- Certifying compilers for sigma protocols date from 2010 and 2012, judging by their titles; I saw only the records — [Almeida et al. 2010](https://doi.org/10.1007/978-3-642-15497-3_10), [Almeida et al. 2012](https://doi.org/10.1145/2382196.2382249).

**(b) Open**
- zkVM verification results are isolated: they mostly prove constraint soundness, leave key artifacts unverified, and rely on trusted extraction and modeling — [Kolozyan et al.](https://arxiv.org/abs/2607.23752).
- Leo's end-to-end proof is unfinished. The 2023 report calls correct compilation of Aleo instructions to R1CS future work, and Leo to Aleo instructions "mainly future work" — [Coglio et al.](https://eprint.iacr.org/2023/1278). The 2021 Leo paper warns that some described features may not be fully implemented — [Leo](https://eprint.iacr.org/2021/651).

**(c) Authenticating a circuit by proof of compilation: established**
- The threat is recognized: a maliciously generated verifying key makes verifiers accept proofs of a different functionality — [vkproof](https://eprint.iacr.org/2026/581).
- Running a compiler in a zkVM works for a small compiler: a median of about 79 s for ChibiCC (about 10,000 lines of C) in RISC Zero 3.0 — [Ron and Monperrus](https://arxiv.org/abs/2602.11887).

**(c) Open**
- vkproof is not implemented; implementing it is the first item in its future work — [vkproof](https://eprint.iacr.org/2026/581).
- Ron and Monperrus prove which compiler ran, not that its output is faithful. They propose combining with certifying compilers as future work — [Ron and Monperrus](https://arxiv.org/abs/2602.11887).

### Inferences
- The zk-pdp proposal differs from both precedents. vkproof is a special-purpose argument for one proof system whose verifier reads the program. Ron and Monperrus target a C compiler and do not produce a key. Proving a whole Term-IR-to-Noir, Noir-to-ACIR and ACIR-to-key pipeline in a zkVM appears unmeasured.
- The strongest honest claim for the compilation proof is identity: this pinned compiler image, run on the input with hash H, output key K. Semantic correctness still rests on Cedar's Lean work for the front end and on testing for the back end.

### Gaps
- I searched by keyword and did not walk citation graphs exhaustively, so "none found" claims are not proofs of absence.

## Compilers to R1CS and Plonkish: which are relevant comparators?

### Takeaway
CirC is the closest technical comparator, because its input is SMT-like and it lowers bit-vector logic to a field, which is the same job as Term IR to Noir. Lurk is the comparator for policy as data, and ZoKrates matters because the nearest policy compiler targets it. Vamp-IR is archived and can be dropped.

### Cited Findings

| System | Relevance to zk-pdp | Evidence | Status of evidence |
|---|---|---|---|
| CirC | High. Shared IR for SMT and R1CS. Field-blasting is the analogue of Term IR to Noir. | [CirC](https://par.nsf.gov/servlets/purl/10432512), [Ozdemir et al.](https://eprint.iacr.org/2023/778) | passages read |
| Noir / ACIR | The target. Compiles to bytecode, and the backend then lowers it. Few verification tools exist. | [NAVe](https://arxiv.org/abs/2601.09372), [LIEZZ](https://arxiv.org/abs/2608.30648) | passages read |
| ZoKrates | Medium. Target of the nearest policy compiler, ZK-TPL. | [More et al.](https://arxiv.org/abs/2212.02454) | passages read |
| Circom | Medium. Most analyzers and verified re-implementations target it. Reported target of a 2026 health-policy system. | [Kolozyan et al.](https://arxiv.org/abs/2607.23752), [Coda](https://eprint.iacr.org/2023/547), [Albalwy](https://doi.org/10.3390/systems14040385) | passages read; Albalwy search summary only |
| Leo | High for method. A verifying compiler design with ACL2, and the setting of vkproof. | [Leo](https://eprint.iacr.org/2021/651), [vkproof](https://eprint.iacr.org/2026/581) | passages read |
| Lurk | High for design. Universal interpreter circuit, content-addressed programs. | [Lurk](https://eprint.iacr.org/2023/369) | abstract |
| Cairo | Medium. One CPU AIR with a verified encoding. Closer to the zkVM backend than to the compiler. | [Cairo](https://eprint.iacr.org/2021/1063), [Avigad et al. 2026](https://arxiv.org/abs/2606.04311) | abstract |
| zkLLVM | Low. Compiles LLVM IR to arithmetizations. Repository last pushed 2025-01. | [SoK](https://arxiv.org/abs/2402.15293), [repository](https://github.com/NilFoundation/zkLLVM) | passage; repository metadata |
| Pequin | Low, historical. Repository last pushed 2022-04. | [repository](https://github.com/pepper-project/pequin) | repository metadata |
| Vamp-IR | Drop. Repository archived, last pushed 2023-08. | [repository](https://github.com/anoma/vamp-ir) | repository metadata |
| LLZK | Medium, as a trust warning. MLIR-based shared IR, in which the compiler pipeline joins the trusted base. | [Kolozyan et al.](https://arxiv.org/abs/2607.23752), [LLZK](https://github.com/project-llzk/llzk-lib) | passage |
| Pinocchio, Geppetto, Buffet, Pantry, xJsnark, SNARKs for C | Foundational only. | Bibliography below | records only |

- Circom's compiler emits R1CS plus WebAssembly and C++ witness generators — [Circom preprint](https://quarxiv.authorea.com/doi/full/10.36227/techrxiv.19374986.v1) (search summary).
- ZK-TPL warns that its compiler and ZoKrates together define the encoding, so a change in either may make old proofs unverifiable — [More et al.](https://arxiv.org/abs/2212.02454).
- Coda's compiler produces R1CS of comparable size to Circom at `-O1` — [Coda](https://www.cs.utexas.edu/~isil/coda.pdf).

### Inferences
- zk-pdp could reuse CirC's method rather than its code: per-operator encoding rules, each with a verification condition discharged by an SMT solver. Cedar's stack already depends on cvc5 — [project note S05](https://docs.rs/cedar-policy-symcc).
- ZK-TPL's warning implies a Noir or Barretenberg upgrade can change every verification key. Pinning toolchain versions is therefore part of the identity the proof binds.

### Gaps
- Noir has no peer-reviewed design paper that I found. Its description rests on docs and third-party papers.
- I did not measure any compiler against Noir on Cedar-shaped workloads. No such comparison turned up.

## Verified compilers for ZK: what exists?

### Takeaway
No verified Noir or ACIR compiler exists that I could find. The field has bounded verification of single passes, certifying designs in progress, DSLs with proofs of individual circuits, and proof-assistant models. Translation validation for ZK compilers is sparse and mostly per-operator.

### Cited Findings

**Verified or certified passes and compilers**
- CirC proofs: Ozdemir et al. define correctness for ZKP compilers, split a field-blaster into encoding rules, and show that if the rule conditions hold then the pass is correct. They checked bounded versions with cvc5. Limits stated: bounded size, bounded arity, and an assumption about each rule's form — [Ozdemir et al.](https://eprint.iacr.org/2023/778). A journal version exists — [FMSD 2025](https://doi.org/10.1007/s10703-025-00476-3).
- Field theory for solvers: finite-field SMT underlies this work — [Ozdemir et al., CAV 2023](https://eprint.iacr.org/2023/091).
- Leo: the compiler is designed as a verifying compiler, emitting per-run ACL2 proofs of phases — [Leo](https://eprint.iacr.org/2021/651). Ozdemir et al. call these "(partial)" proofs — [Ozdemir et al.](https://eprint.iacr.org/2023/778). A 2023 report verifies snarkVM R1CS gadgets compositionally and leaves compilation as future work — [Coglio et al.](https://eprint.iacr.org/2023/1278).
- ACL2 and Axe verify individual R1CS gadgets, and gadget extraction is trusted — [Coglio, McCarthy and Smith](https://arxiv.org/abs/2311.08858).
- Clap: a Rust eDSL treating constraint generation as semantic-preserving compilation, with an Agda model — [Clap](https://arxiv.org/abs/2405.12115).
- Coda: refinement types with Coq lemmas. The ePrint abstract says 79 circuits ([ePrint](https://eprint.iacr.org/2023/547)); the PDF on the author's page says 77 re-implemented, 66 verified, 6 new vulnerabilities ([PDF](https://www.cs.utexas.edu/~isil/coda.pdf)). These are different versions and they conflict. Coda certifies circuits by types; it is not a verified compiler. Its arXiv listing says it was submitted there in error — [arXiv 2304.07648](https://arxiv.org/abs/2304.07648).
- Certifying compilers for sigma protocols: [Almeida et al. 2010](https://doi.org/10.1007/978-3-642-15497-3_10) and ["Full proof cryptography"](https://doi.org/10.1145/2382196.2382249). I saw only their records in this session.

**Noir and ACIR specifically**
- NAVe encodes a subset of ACIR in SMT-LIB and checks developer assertions with cvc5. It covers AssertZero, Range, MemoryInit and MemoryOp. It leaves out Call and most black-box functions. It translates BrilligCall as `true`, so unconstrained results stay free — [NAVe](https://arxiv.org/abs/2601.09372).
- Lampe extracts Noir semantics to Lean with interactive proofs. The repository was pushed 2026-10-21 — [Lampe](https://github.com/reilabs/lampe), [NAVe](https://arxiv.org/abs/2601.09372).
- Barretenberg's `acir_formal_proofs` checks ACIR arithmetic, bitwise, shift and comparison gadgets against SMT terms, per operator. Its table shows timeouts, for example `Binary::Div` on `Unsigned_128`, with entries last checked 20.03.2026 — [README](https://github.com/AztecProtocol/aztec-packages/tree/master/barretenberg/cpp/src/barretenberg/acir_formal_proofs).

**Lean and Coq formalization of circuits**
- Survey view: zkVM verification is converging on Lean and the Sail RISC-V specification. Compiler translation "joins the trusted base", and processing below the verified level is trusted not to alter semantics — [Kolozyan et al.](https://arxiv.org/abs/2607.23752).
- Garden is Formal Land's Rocq framework, with a Plonky3 Keccak verification as its main example — [Garden](https://github.com/formal-land/garden).
- Clean is a Lean circuit DSL using correctness by construction, proving both constraint correctness and witness consistency — [Clean](https://github.com/Verified-zkEVM/clean), [Kolozyan et al.](https://arxiv.org/abs/2607.23752).
- CertiPlonk extracts Plonky3 constraints into Lean — [CertiPlonk](https://github.com/NethermindEth/CertiPlonk). zkLean is a Lean DSL for ZK statements — [zkLean](https://github.com/GaloisInc/zk-lean).
- ZkFlow is a small verified compiler built on zkLean, described on a blog marked as under construction, so treat it as preliminary — [ZkFlow](https://tannerduve.github.io/blog/zkflow).

**Under-constraint analysis (circuit level, not compiler level)**
- Picus/QED2 solved 70% of 163 Circomlib circuits and found 8 vulnerable templates. Because it works on the output constraints, it can also catch compiler-caused bugs — [Pailoor et al.](https://eprint.iacr.org/2023/512).
- Also: CIVER — [Isabel et al.](https://doi.org/10.1109/sp54263.2024.00133); consistency checking — [Stephens et al.](https://doi.org/10.1007/978-3-031-98668-0_16); Halo2 analysis — [Soureshjani et al.](https://eprint.iacr.org/2023/1051); AC4 — [Yang et al.](https://arxiv.org/abs/2403.15676).

**Translation validation**
- The closest work I found is R1CS normalization, which maps semantically equal R1CS from different compilers to a standard form — [Shi et al.](https://arxiv.org/abs/2309.04274). The classic reference is [Pnueli et al.](https://doi.org/10.1007/bfb0054170).

**Cedar front end**
- SymCert is a Lean framework with a verified symbolic compiler and authorizer. It splits correctness into reducibility and interpretability, and verifies five analyses used in an AWS service — [SymCert](https://www.amazon.science/publications/symcert-verifying-smt-based-policy-analyses). The page does not state theorems about the term language.

### Inferences
- The verified boundary in zk-pdp stops at the Term IR. Below it lie the Term-to-Noir translation, Noir SSA and ACIR generation, Barretenberg lowering and key generation. LIEZZ shows Barretenberg lowering has had a soundness bug, in a layer the Noir SSA checks do not reach.
- A realistic strengthening is per-operator verification conditions in CirC's style for each Term constructor. A second is equivalence checking of a term against its own ACIR, using an encoding like NAVe's. The second would be translation validation per policy, and I found no one doing it. Both are my suggestions, not results.
- SymCert's stated result is about SMT-based analyses. Whether its theorems transfer to a non-SMT consumer of the Term IR needs reading the Lean statements. A circuit backend needs an interpretation of terms in the field. I would not expect the theorems to provide this, but I have not read them.

### Gaps
- I could not read SymCert's full text. The project's source note S06 also marks it unread. This is the most important unread item for the claim of a verified front end.
- I found no primary source confirming Leo's current proof coverage after 2023. A third-party blog claims per-phase proofs; I treat this as unverified.
- Nethermind's Lean Clap and zkFlow status: blog-level sources only.

## ZK for logic and proofs: what exists?

### Takeaway
There is steady work proving that a solver's or prover's certificate checks in zero knowledge, plus query-language systems for SQL and regex. These prove properties of secret formulas or data, which differs from zk-pdp's job of evaluating a known policy on hidden inputs, though Datalog systems come close.

### Cited Findings
- UNSAT: resolution proofs are checked in ZK. Only the proof's dimensions are revealed — [Luo et al.](https://eprint.iacr.org/2022/206).
- SMT: ZKSMT is a VM for checking SMT validity proofs in ZK, with theories of equality and linear integer arithmetic. It reports about three orders of magnitude over running the checker on a general ZK system, and practicality on Boogie-generated formulas — [Luick et al.](https://www.usenix.org/conference/usenixsecurity24/presentation/luick).
- Lean: zkPi is a zkSNARK for Lean proofs that handles 57.9% of stdlib theorems and 14.1% of mathlib theorems — [Laufer et al.](https://eprint.iacr.org/2024/267).
- Model checking: ranking-function certificates for omega-regular properties, including LTL, with explicit-state and symbolic schemes; it claims to be first — [Berrang et al.](https://arxiv.org/abs/2605.00487).
- QBF and PSPACE: [Karthikeyan et al.](https://doi.org/10.1109/sp63933.2026.00041) (record only).
- Datalog: Circuitree is a Datalog reasoner producing R1CS for Bulletproofs, evaluated for peer-to-peer access control. Its example verifies a COVID certificate against an application policy — [Godden et al.](https://orbi.uliege.be/bitstream/2268/288551/1/published-Circuitree_A_Datalog_Reasoner_in_Zero-Knowledge.pdf).
- SQL: PoneglyphDB gives non-interactive proofs for SQL using Plonkish operators and recursion, and identifies ZKSQL as interactive — [Gu et al.](https://arxiv.org/abs/2411.15031) (search summary). See also [ZKSQL](https://doi.org/10.14778/3594512.3594513), [vSQL](https://doi.org/10.1109/sp.2017.43), [IntegriDB](https://doi.org/10.1145/2810103.2813711).
- Regex and grammars: [Reef](https://eprint.iacr.org/2023/1886), [Coral](https://doi.org/10.1109/sp63933.2026.00059); and middleboxes that check encrypted traffic against policy: [ZKMB](https://www.usenix.org/conference/usenixsecurity22/presentation/grubbs), [Zombie](https://www.usenix.org/conference/nsdi24/presentation/zhang-collin).
- Program analysis: [Fang et al.](https://doi.org/10.1145/3460120.3484795); vulnerability proofs: [Cheesecloth](https://www.usenix.org/conference/usenixsecurity23/presentation/cuellar).

### Inferences
- These papers suggest a route for the compilation proof that neither precedent takes. If the compiler emitted a checkable certificate per run, a zkVM or a system like zkPi could check the certificate rather than re-execute the compiler. The coverage figures for zkPi suggest this is not close for Lean. This is my speculation.
- A Cedar decision is a known-policy evaluation, so a certificate of unsatisfiability is not needed for a single decision. These systems would matter for a claim such as "this secret policy never allows X" (for private policies), a claim the current zk-pdp plan does not make.

### Gaps
- I found one ZK Datalog system. A dedicated search for zkSNARK Datalog gave nothing further, though other names may exist.

## Proving compilation: precedents and the state of the art

### Takeaway
vkproof and Ron and Monperrus are the two direct precedents. Deployed systems authenticate keys by allowlist or registry, not by proof of derivation. Nobody I found has proved real circuit-compiler output in a zkVM, so the cost of H5 is an unmeasured quantity.

### Cited Findings

**vkproof (ePrint 2026/581)**
- It is a preprocessing SNARG showing that a Varuna verifying key is correct for the R1CS compiled from a program. Proof size is constant, about 1.7 KB with the Marlin PCS. Verifier cost is linear in program instructions, not R1CS density — [vkproof](https://eprint.iacr.org/2026/581).
- Direct key regeneration costs O(d log d) field work and O(d) group work in the matrix density d. Varuna keys are deterministic given program code, so regeneration is the strawman — [vkproof](https://eprint.iacr.org/2026/581).
- The verifier reads the program, and the one-off indexer is publicly verifiable. Its verifier efficiency comes from modular compilation, where programs are assembled from per-instruction R1CS. It asks whether modular compilation fits a real language like Leo as an open question — [vkproof](https://eprint.iacr.org/2026/581).

**Zero-knowledge compilation (Ron and Monperrus, arXiv 2602.11887)**
- The receipt binds a compiler image ID, a source digest and the output assembly. RISC Zero 3.0, ChibiCC, about 10,000 lines of C — [paper](https://arxiv.org/abs/2602.11887).
- Median zk-compile was 78.91 s (range 57.01 to 979.12) on Csmith programs, about four orders of magnitude over native, with verification of about 49 s. The largest input was 82.94 KB — [paper](https://arxiv.org/abs/2602.11887). Those figures came through a summarizing fetch of the HTML, and I checked only that the PDF exists and contains the claims in outline. The summary also flagged an inconsistency inside the paper: receipt maximum of 114.41 MB in a table against "120 MB" in text.
- It relates itself to reproducible builds, TEEs, CompCert (correctness but not provenance) and proof-carrying code (complementary), and claims to be first to study ZK execution for compilation — [paper](https://arxiv.org/abs/2602.11887).
- ZK is not needed for this use; succinctness is what matters — [paper](https://arxiv.org/abs/2602.11887).

**Alternatives and baselines**
- TEE builds: 42 s start-up latency and about 14% overhead by default, with trust moved to hardware — [Attestable Builds](https://arxiv.org/abs/2505.02521).
- Reproducible builds: [Lamb and Zacchiroli](https://doi.org/10.1109/ms.2021.3073045). Diverse double-compiling: [Wheeler](https://doi.org/10.1109/csac.2005.17). Root concern: [Thompson](https://doi.org/10.1145/1283920.1283940).
- Prezta: the policy admin compiles XACML to Rust and to a RISC-V image, and deploys a signed IMAGE_ID to devices. That is an allowlist, with no compiler proof — [Prezta](https://arxiv.org/abs/2607.11466).
- Aztec: the circuit checks that a function's VK hash is in the registered private-functions root, while the artifact hash is unenforced and used by clients. The page does not describe a proof that a VK derives from source — [Aztec docs](https://docs.aztec.network/developers/docs/foundational-topics/contract_creation).
- Noir recursion: the compiler does not itself constrain `key_hash` to be a valid hash of the verification key; this is left to the developer or proving system — [aztec-nr docs](https://docs.aztec.network/aztec-nr-api/mainnet/std/fn.verify_proof_with_type).
- Proof-carrying code and certifying compilers are the conceptual ancestors: [Necula 1997](https://doi.org/10.1145/263699.263712), [Necula and Lee 1998](https://doi.org/10.1145/277650.277752). Translation validation: [Pnueli et al.](https://doi.org/10.1007/bfb0054170). CompCert: [Leroy](https://doi.org/10.1145/1538788.1538814).

**Universal circuits and policy as data**
- Lurk's universal interpreter circuit makes programs data, with content addressing — [Lurk](https://eprint.iacr.org/2023/369).
- Prior context for one key serving many functions: [Valiant](https://doi.org/10.1145/800113.803649), [Zexe](https://doi.org/10.1109/sp40000.2020.00050), PCD recursion [Bitansky et al.](https://doi.org/10.1145/2488608.2488623) (records only).
- Universal SRS systems preprocess any computation deterministically up to a size bound — [ark-marlin docs](https://docs.rs/crate/ark-marlin/latest), [Marlin](https://doi.org/10.1007/978-3-030-45721-1_26).

**Trust base of a zkVM compilation proof**
- Arguzz found 11 bugs in 3 of 6 zkVMs, with one RISC Zero bug paying a $50,000 bounty despite prior audits — [Arguzz](https://arxiv.org/abs/2509.10819).
- L2BEAT lists a critical RISC Zero vulnerability dated October 2025. I saw only the snippet, without details — [L2BEAT](https://l2beat.com/zk-catalog/risc0).

### Inferences
- If the policy is public, a verifier can recompute K itself, since Noir compilation and key generation are expected to be deterministic for a fixed toolchain. The proof then only saves verifier work, which matters mostly for constrained verifiers. If the policy is private, the proof also hides it, but only if the hash is salted and the receipt is zero-knowledge. Both points need stating in the paper. I did not find evidence of Noir's determinism as a guarantee.
- The proof binds three things for the verifier: compiler image, policy hash and K. The relevant checks should be part of the guest. Noir issue #12843 shows an artifact cache can reuse output compiled with skipped checks, so running the checks must be inside the proved computation, not assumed from the artifact.
- The H5 feasibility question has a specific unknown. Barretenberg key generation includes elliptic-curve commitments, and vkproof names group work as a major key-generation cost. This suggests key generation may dominate cycles in a zkVM, unless a precompile helps. This needs measuring in E5. A staged alternative is to prove compilation to ACIR or a circuit hash only, and let any party derive K.
- The proof needs a Barretenberg build that runs as a zkVM guest. I found no evidence anyone has done so. This could be the largest practical risk.
- Ron and Monperrus's verify time of about 49 s is far above H8's 50 ms limit. H8 already excludes raw composite receipts, and the summary I saw does not say which receipt type they measured. A constrained verifier of a compilation proof likely needs a succinct or Groth16-wrapped receipt.

### Gaps
- No measured cost for proving Noir compilation or Barretenberg key generation in any zkVM.
- No paper studies composing a compilation proof with a certifying compiler. Both precedents leave it as future work or out of scope.
- vkproof gives asymptotic and component costs but no implementation numbers.
- Unverified leads, not in the bibliography because I could not confirm the metadata: Chiesa and Tromer on proof-carrying data (ICS 2010); Boneh, Nguyen and Ozdemir on functional commitments; Liu, Yu and Wang on linear-size universal circuits. Mina's side-loaded keys and SP1's vk root were not checked.

## Bugs in ZK compilers and under-constrained circuits, and Noir's Brillig checker

### Takeaway
Compiler bugs are a confirmed, recurring class, and they include Noir. The key point for zk-pdp: soundness bugs are invisible to differential testing on valid executions. Noir's checks are coarse connectivity heuristics reported as warnings, with many documented false positives and no guarantee in the other direction.

### Cited Findings

**Studies**
- SoK: 141 vulnerabilities from 107 audit reports and 16 disclosures. Under-constrained circuits are the most frequent class in the circuit layer. It defines V8 "Incorrect Constraint Compilation" and V9 "Witness Generation Error", and says the front-end and back-end layer "has largely been overlooked" — [Chaliasos et al.](https://arxiv.org/abs/2402.15293).
- Tool effectiveness: at least one of six tools detects 45.7% of 70 real bugs on isolated circuits, and 19.6% on whole projects. Practitioners see gaps around semantic errors, Fiat-Shamir issues and integrations — [Kolozyan et al.](https://arxiv.org/abs/2607.23752).
- Also [Tang et al.](https://eprint.iacr.org/2024/514) (abstract not read) and [zkFuzz](https://arxiv.org/abs/2504.11961): 85 bugs including 59 zero-days across 452 Circom circuits, with only preliminary Noir support (search summary).

**Compiler testing**
- MTZK: metamorphic testing on four industrial ZK compilers found 21 bugs, 15 patched — [MTZK](https://www.ndss-symposium.org/ndss-paper/mtzk-testing-and-exploring-bugs-in-zero-knowledge-zk-compilers/).
- Circuzz: 16 unique bugs in Circom, Corset, Gnark and Noir, 15 confirmed and fixed. At the time of the paper it did not exercise Noir's non-field input types — [Circuzz](https://arxiv.org/abs/2411.02077). A vendor post says it has since passed 30 soundness or completeness bugs (secondary) — [Diligence](https://diligence.security/blog/2026/04/zk-fuzzing-valentin-w%C3%BCstholz-has-surfaced-30-critical-bugs-in-zk-compilers-and-zkvms/).
- LIEZZ: 13 bugs across the four toolchains, seven with soundness impact. Its abstract says such bugs may remain invisible to valid-execution testing, and a same-budget baseline of valid-execution testing exposed none of the accepted-witness soundness failures — [LIEZZ](https://arxiv.org/abs/2608.30648).
- In Noir, LIEZZ's bug 9 weakens constraints when Barretenberg lowers ACIR into backend constraints. Bug 10 leaves `is_infinite` unconstrained relative to point coordinates — [LIEZZ](https://arxiv.org/abs/2608.30648).
- CVE-2026-41197: foreign calls returning composite arrays corrupt the Brillig VM heap, fixed in 1.0.0-beta.19 (search summary) — [NVD](https://nvd.nist.gov/vuln/detail/CVE-2026-41197).

**What the Noir checks flag** (source read at `master`, head `e0ca395`, 2026-11-04)

There are two SSA passes, both reporting `SsaReport::Bug`.

1. `check_for_underconstrained_values` finds parts of a function's constraint graph connected to neither inputs nor outputs, using union-find. Message: "Input to Brillig function is in a separate subgraph to output", with "This is quite likely a soundness vulnerability" — [source](https://github.com/noir-lang/noir/blob/master/compiler/noirc_evaluator/src/ssa/checks/check_for_underconstrained_values.rs), [#11382](https://github.com/noir-lang/noir/issues/11382).
2. `check_for_missing_brillig_constraints` looks for a constraint whose ancestors intersect both the call's output descendants and its input ancestors. Message: "Brillig function call isn't properly covered by a manual constraint". The source says: "The goal isn't to verify that the constraint is correct, just that some (indirect) connection between inputs and outputs is made." — [source](https://github.com/noir-lang/noir/blob/master/compiler/noirc_evaluator/src/ssa/checks/check_for_missing_brillig_constraints.rs).

Built-in limits that create misses:
- Arrays longer than `DEFAULT_MAX_ARRAY_OUTPUT_LENGTH = 64` count as constrained once any one item is constrained — [source](https://github.com/noir-lang/noir/blob/master/compiler/noirc_evaluator/src/ssa/checks/check_for_missing_brillig_constraints.rs).
- Ancestor search stops at `DEFAULT_MAX_ANCESTOR_DISTANCE = 10`, added for runtime on large protocol circuits — [source](https://github.com/noir-lang/noir/blob/master/compiler/noirc_evaluator/src/ssa/checks/check_for_missing_brillig_constraints.rs).
- Both checks can be skipped: `--skip-underconstrained-check`, `--skip-brillig-constraints-check`. The driver warns they should always run on production code — [driver](https://github.com/noir-lang/noir/blob/master/compiler/noirc_driver/src/lib.rs).

Conflict to note: a search summary of the Noir security docs mentions `--enable-brillig-constraints-check-lookback`. That flag name is not in the driver at `master`, and the docs URL returned 404 when I fetched it. PR [#7190](https://github.com/noir-lang/noir/pull/7190) made lookback opt-in earlier, and the check was refactored in [#12026](https://github.com/noir-lang/noir/pull/12026). Treat the docs description as stale.

**Documented false positives** (issue state as of 2026-11-04)

| Issue | State | Trigger |
|---|---|---|
| [#10728](https://github.com/noir-lang/noir/issues/10728) | open | Array index is a variable |
| [#13600](https://github.com/noir-lang/noir/issues/13600) | open | Fully constrained `bigcurve` transcript replay, beta.26 |
| [#6793](https://github.com/noir-lang/noir/issues/6793) | open | `println` in stdlib |
| [#13878](https://github.com/noir-lang/noir/pull/13878) | open PR | Output related to a value only through an equivalence, when input descends from an earlier Brillig call |
| [#12506](https://github.com/noir-lang/noir/issues/12506) | closed | Source order of two equivalent assertions |
| [#11382](https://github.com/noir-lang/noir/issues/11382) | closed | Function with assertions only, no return value |
| [#10547](https://github.com/noir-lang/noir/issues/10547) | closed | Output cast to `Field` before comparing |
| [#10660](https://github.com/noir-lang/noir/issues/10660) | closed | `BoundedVec` with `sha256` |
| [#7577](https://github.com/noir-lang/noir/issues/7577) | closed | `println` |
| [#8554](https://github.com/noir-lang/noir/issues/8554), [#7292](https://github.com/noir-lang/noir/issues/7292) | closed | Higher-order and indirect Brillig calls |
| [#13879](https://github.com/noir-lang/noir/pull/13879) | closed PR | Outputs that are never used |

- Artifact cache: `nargo compile` can reuse an artifact built with the checks skipped during a later default compile, because the cache key does not record the validation policy — [#12843](https://github.com/noir-lang/noir/issues/12843) (open).

### Inferences
- zk-pdp's differential tests compare outputs on honest witnesses. They test completeness and functional agreement. They cannot find a missing constraint, because an honest prover still produces the correct answer. LIEZZ makes this point directly. The project's threat model lists differential tests of the back end as the correctness basis, so this is a real gap to close or state. Adversarial witness tests in LIEZZ's style, or an NAVe-style encoding, would address it.
- A clean Noir check does not show soundness; a warning does not show a bug. For generated code, the simplest policy is to emit no `unconstrained` calls of its own and to fail the build on any report, then triage stdlib-caused reports one at a time.
- Bugs below ACIR, such as LIEZZ bug 9, are not visible to the SSA checks at all, so testing must reach the Barretenberg level.
- Circuzz's blind spot on non-field types, as of its paper, matters here, since Cedar terms include bit-vectors. That zone may be less fuzzed.

### Gaps
- I found no measured false-positive or false-negative rate for the Noir checks. Only issue-level evidence exists. The table is a sample from search, not a full census.
- `nargo` report handling (warning versus error, and exit status) was not tested in this session.
- No published study covers bugs specific to Noir's stdlib integer gadgets that Cedar terms would use.

## Is there any compiler from an authorization or logic policy language to circuits?

### Takeaway
Yes, a few, but none for Cedar or Rego, none with a verified front end, and none with a proof of compilation that I could find. ZK-TPL and Cinderella are the strongest prior art, and both predate Prezta's zkVM route. The zk-pdp claim should be "first through a verified front end", not "first policy compiler".

### Cited Findings
- ZK-TPL compiles TPL, a Prolog-style trust policy language, to ZoKrates programs and from there to circuits, with reveal and hide annotations. Evaluation found negligible communication and verification overhead for regular credentials. Passages I read do not discuss verifying the compiler — [More et al.](https://arxiv.org/abs/2212.02454), [TrustCom record](https://doi.org/10.1109/trustcom56396.2022.00084).
- Cinderella: X.509 validation policies written as compositions of templates compile to C, then Geppetto produces a verifiable computation scheme for that policy. Applied to TLS and to Helios access control — [Delignat-Lavaud et al.](https://www.andrew.cmu.edu/user/bparno/papers/cinderella.pdf).
- Circuitree: Datalog reasoner to R1CS, evaluated for access control — [Godden et al.](https://orbi.uliege.be/bitstream/2268/288551/1/published-Circuitree_A_Datalog_Reasoner_in_Zero-Knowledge.pdf).
- Albalwy 2026: a policy engine turns rules into Boolean conditions, and a generator encodes them in Circom for Groth16 (search summary; the publisher blocked my fetch) — [Albalwy](https://doi.org/10.3390/systems14040385).
- Prezta: XACML 3.0 to Rust to RISC-V in RISC Zero, 83% of the conformance suite. Prezta chose a zkVM explicitly for policy flexibility over circuit-based SNARKs — [Prezta](https://arxiv.org/abs/2607.11466).
- Vendor claim, unverified: OpenMatter describes compiling agent policy to an "evaluation circuit" run in a zkVM with compile at policy change — [OpenMatter](https://openmatter.substack.com/p/enforcing-policy-with-a-circuit).
- Language-level ancestors: ZKPDL ([Meiklejohn et al.](https://www.usenix.org/legacy/events/sec10/tech/full_papers/Meiklejohn.pdf)), ZQL ([Fournet et al.](https://www.usenix.org/conference/usenixsecurity13/technical-sessions/presentation/fournet)), ZØ ([Fredrikson and Livshits](https://www.usenix.org/conference/usenixsecurity14/technical-sessions/presentation/fredrikson)). On authorization: Privacy-aware proof-carrying authorization ([Maffei and Pecina](https://doi.org/10.1145/2166956.2166963)), proof-carrying authentication ([Appel and Felten](https://doi.org/10.1145/319709.319718)), and type-checking ZK ([Backes et al.](https://doi.org/10.1145/1455770.1455816)). Records or title pages only, except where noted.

### Inferences
- Cinderella's chain (policy to C to Geppetto) and zk-pdp's chain (policy to Term IR to Noir) have the same shape. A reviewer will likely raise it, so it should be cited early.
- What ZK-TPL and Circuitree lack, and Cedar supplies, is a verified semantics and validator. That is the distinctive claim.

### Gaps
- No Cedar or Rego compiler to circuits was found in several searches.
- Project sources S18 (POLARIS) and S19 (health data) are marked unread. I could only partly fill S19.
- I read Prezta's compiler and deployment sections only. Another researcher may cover it in depth.

## Bibliography: what are the verified records?

### Takeaway
There are 95 entries below (10 `read-full`, 23 `read-abstract`, 62 `search-summary-only`). All metadata comes from the checks named at the top, and each entry carries a status comment. Fields I could not confirm are omitted.

### Cited Findings

Direct precedents and policy compilers:

```bibtex
% read-full (intro, related work, costs, future work)
@misc{vkproof2026, title={vkproof: Succinct verification of indexed verifying keys using modular compilation and polynomial fingerprinting}, author={Mejias Gil, Antonio and Zhao, Xueqin}, year={2026}, howpublished={Cryptology ePrint Archive, Paper 2026/581}, url={https://eprint.iacr.org/2026/581}}
% read-full (via summarizing fetch of the HTML; location of claims confirmed in the PDF)
@misc{ron2026zkcompile, title={Verifiable Provenance of Software Artifacts with Zero-Knowledge Compilation}, author={Ron, Javier and Monperrus, Martin}, year={2026}, eprint={2602.11887}, archivePrefix={arXiv}, doi={10.48550/arXiv.2602.11887}, url={https://arxiv.org/abs/2602.11887}}
% read-full (compiler and evaluation passages)
@inproceedings{more2022zktpl, title={Extending Expressive Access Policies with Privacy Features}, author={More, Stefan and Ramacher, Sebastian and Alber, Lukas and Herzl, Marco}, booktitle={2022 IEEE International Conference on Trust, Security and Privacy in Computing and Communications (TrustCom)}, year={2022}, doi={10.1109/trustcom56396.2022.00084}, eprint={2212.02454}, archivePrefix={arXiv}}
% read-abstract (plus located passages in the PDF)
@inproceedings{cinderella2016, title={Cinderella: Turning Shabby X.509 Certificates into Elegant Anonymous Credentials with the Magic of Verifiable Computation}, author={Delignat-Lavaud, Antoine and Fournet, Cedric and Kohlweiss, Markulf and Parno, Bryan}, booktitle={2016 IEEE Symposium on Security and Privacy (SP)}, year={2016}, doi={10.1109/sp.2016.22}}
% read-abstract (plus located passages in the PDF)
@article{circuitree2022, title={Circuitree: A Datalog Reasoner in Zero-Knowledge}, author={Godden, Tom and De Smet, Ruben and Debruyne, Christophe and Vandervelden, Thibaut and Steenhaut, Kris and Braeken, An}, journal={IEEE Access}, year={2022}, doi={10.1109/access.2022.3153366}}
% read-full (design and compiler passages). Project spec says USENIX Security 2026; not checked here.
@misc{prezta2026, title={Prezta: Provable Remote Execution of Zero-Trust Authorization using SNARKs}, author={Wei, Zhongjing and Ameer, Osaid Muhammad and Borisov, Nikita and Zhang, Yupeng}, year={2026}, eprint={2607.11466}, archivePrefix={arXiv}}
% search-summary-only
@article{albalwy2026, title={Zero-Knowledge-Based Policy Enforcement for Privacy-Preserving Cross-Institutional Health Data Sharing on Blockchain}, author={Albalwy, Faisal}, journal={Systems}, year={2026}, doi={10.3390/systems14040385}}
% read-abstract (page summary). The page lists one author, which may be incomplete.
@inproceedings{symcert2026, title={SymCert: Verifying SMT-based policy analyses}, author={Torlak, Emina}, booktitle={FMCAD}, year={2026}, url={https://www.amazon.science/publications/symcert-verifying-smt-based-policy-analyses}}
% search-summary-only
@article{cedar2024, title={Cedar: A New Language for Expressive, Fast, Safe, and Analyzable Authorization}, author={Cutler, Joseph W. and Disselkoen, Craig and Eline, Aaron and He, Shaobo and Headley, Kyle and Hicks, Michael and Hietala, Kesha and Ioannidis, Eleftherios and Kastner, John and Mamat, Anwar and McAdams, Darin and McCutchen, Matt and Rungta, Neha and Torlak, Emina and Wells, Andrew}, journal={Proceedings of the ACM on Programming Languages}, year={2024}, doi={10.1145/3649835}, eprint={2403.04651}, archivePrefix={arXiv}}
% search-summary-only
@inproceedings{cedarbuilt2024, title={How We Built Cedar: A Verification-Guided Approach}, author={Disselkoen, Craig and Eline, Aaron and He, Shaobo and Headley, Kyle and Hicks, Michael and Hietala, Kesha and Kastner, John and Mamat, Anwar and McCutchen, Matt and Rungta, Neha and Shah, Bhakti and Torlak, Emina and Wells, Andrew}, year={2024}, doi={10.1145/3663529.3663854}}
```

Compilers and DSLs:

```bibtex
% read-abstract (plus introduction passages)
@inproceedings{circ2022, title={CirC: Compiler infrastructure for proof systems, software verification, and more}, author={Ozdemir, Alex and Brown, Fraser and Wahby, Riad S.}, booktitle={2022 IEEE Symposium on Security and Privacy (SP)}, year={2022}, doi={10.1109/sp46214.2022.9833782}}
% search-summary-only
@inproceedings{pinocchio2013, title={Pinocchio: Nearly Practical Verifiable Computation}, author={Parno, B. and Howell, J. and Gentry, C. and Raykova, M.}, booktitle={2013 IEEE Symposium on Security and Privacy}, year={2013}, doi={10.1109/sp.2013.47}}
% search-summary-only
@incollection{snarksforc2013, title={SNARKs for C: Verifying Program Executions Succinctly and in Zero Knowledge}, author={Ben-Sasson, Eli and Chiesa, Alessandro and Genkin, Daniel and Tromer, Eran and Virza, Madars}, booktitle={Lecture Notes in Computer Science}, year={2013}, doi={10.1007/978-3-642-40084-1_6}}
% search-summary-only
@inproceedings{buffet2015, title={Efficient RAM and Control Flow in Verifiable Outsourced Computation}, author={Wahby, Riad S. and Setty, Srinath and Ren, Zuocheng and Blumberg, Andrew J. and Walfish, Michael}, booktitle={Network and Distributed System Security Symposium}, year={2015}, doi={10.14722/ndss.2015.23097}}
% search-summary-only
@inproceedings{pantry2013, title={Resolving the conflict between generality and plausibility in verified computation}, author={Setty, Srinath and Braun, Benjamin and Vu, Victor and Blumberg, Andrew J. and Parno, Bryan and Walfish, Michael}, booktitle={Proceedings of the 8th ACM European Conference on Computer Systems}, year={2013}, doi={10.1145/2465351.2465359}}
% search-summary-only
@inproceedings{geppetto2015, title={Geppetto: Versatile Verifiable Computation}, author={Costello, Craig and Fournet, Cedric and Howell, Jon and Kohlweiss, Markulf and Kreuter, Benjamin and Naehrig, Michael and Parno, Bryan and Zahur, Samee}, booktitle={2015 IEEE Symposium on Security and Privacy}, year={2015}, doi={10.1109/sp.2015.23}}
% search-summary-only
@inproceedings{xjsnark2018, title={xJsnark: A Framework for Efficient Verifiable Computation}, author={Kosba, Ahmed and Papamanthou, Charalampos and Shi, Elaine}, booktitle={2018 IEEE Symposium on Security and Privacy (SP)}, year={2018}, doi={10.1109/sp.2018.00018}}
% search-summary-only
@inproceedings{zokrates2018, title={ZoKrates - Scalable Privacy-Preserving Off-Chain Computations}, author={Eberhardt, Jacob and Tai, Stefan}, year={2018}, doi={10.1109/cybermatics_2018.2018.00199}}
% search-summary-only
@article{circom2023, title={Circom: A Circuit Description Language for Building Zero-Knowledge Applications}, author={Bell{\'e}s-Mu{\~n}oz, Marta and Isabel, Miguel and Mu{\~n}oz-Tapia, Jose Luis and Rubio, Albert and Baylina, Jordi}, journal={IEEE Transactions on Dependable and Secure Computing}, year={2023}, doi={10.1109/tdsc.2022.3232813}}
% read-abstract (plus located passages)
@misc{leo2021, title={Leo: A Programming Language for Formally Verified, Zero-Knowledge Applications}, author={Chin, Collin and Wu, Howard and Chu, Raymond and Coglio, Alessandro and McCarthy, Eric and Smith, Eric}, year={2021}, howpublished={Cryptology ePrint Archive, Paper 2021/651}, url={https://eprint.iacr.org/2021/651}}
% read-abstract. ePrint lists ICFP 2023.
@misc{lurk2023, title={LURK: Lambda, the Ultimate Recursive Knowledge}, author={Amin, Nada and Burnham, John and Garillot, Fran{\c{c}}ois and Gennaro, Rosario and K{\"u}nzang, Chhi'm{\`e}d and Rogozin, Daniel and Wong, Cameron}, year={2023}, howpublished={Cryptology ePrint Archive, Paper 2023/369}, url={https://eprint.iacr.org/2023/369}}
% search-summary-only. Later version with a longer author list.
@article{lurkjfp2026, title={LURK: Lambda, the ultimate recursive knowledge: (Experience Report)}, author={Amin, Nada and Burnham, John C. and Caparica, Gustavo and Garillot, Fran{\c{c}}ois and Gennaro, Rosario and Hamelink, Adrian and K{\"u}nzang, Chhi'M{\`e}d and Rogozin, Daniel and Wong, Cameron}, journal={Journal of Functional Programming}, year={2026}, doi={10.46298/jfp.17841}}
% read-abstract
@misc{cairo2021, title={Cairo -- a Turing-complete STARK-friendly CPU architecture}, author={Goldberg, Lior and Papini, Shahar and Riabzev, Michael}, year={2021}, howpublished={Cryptology ePrint Archive, Paper 2021/1063}, url={https://eprint.iacr.org/2021/1063}}
% read-abstract. Title changed between arXiv versions.
@misc{clap2024, title={Clap: a Semantic-Preserving Optimizing eDSL for Plonkish Proof Systems}, author={Stronati, Marco and Firsov, Denis and Locascio, Antonio and Livshits, Benjamin}, year={2024}, eprint={2405.12115}, archivePrefix={arXiv}}
% search-summary-only
@incollection{marlin2020, title={Marlin: Preprocessing zkSNARKs with Universal and Updatable SRS}, author={Chiesa, Alessandro and Hu, Yuncong and Maller, Mary and Mishra, Pratyush and Vesely, Noah and Ward, Nicholas}, booktitle={Lecture Notes in Computer Science}, year={2020}, doi={10.1007/978-3-030-45721-1_26}}
% search-summary-only
@incollection{spartan2020, title={Spartan: Efficient and General-Purpose zkSNARKs Without Trusted Setup}, author={Setty, Srinath}, booktitle={Lecture Notes in Computer Science}, year={2020}, doi={10.1007/978-3-030-56877-1_25}}
% search-summary-only
@misc{plonk2019, title={PLONK: Permutations over Lagrange-bases for Oecumenical Noninteractive arguments of Knowledge}, author={Gabizon, Ariel and Williamson, Zachary J. and Ciobotaru, Oana}, year={2019}, howpublished={Cryptology ePrint Archive, Paper 2019/953}, url={https://eprint.iacr.org/2019/953}}
% search-summary-only
@incollection{zkbench2024, title={zk-Bench: A Toolset for Comparative Evaluation and Performance Benchmarking of SNARKs}, author={Ernstberger, Jens and Chaliasos, Stefanos and Kadianakis, George and Steinhorst, Sebastian and Jovanovic, Philipp and Gervais, Arthur and Livshits, Benjamin and Orr{\`u}, Michele}, booktitle={Lecture Notes in Computer Science}, year={2024}, doi={10.1007/978-3-031-71070-4_3}}
% search-summary-only
@misc{sheybani2025, title={Zero-Knowledge Proof Frameworks: A Systematic Survey}, author={Sheybani, Nojan and Ahmed, Anees and Kinsy, Michel and Koushanfar, Farinaz}, year={2025}, eprint={2502.07063}, archivePrefix={arXiv}}
% search-summary-only
@inproceedings{hawk2016, title={Hawk: The Blockchain Model of Cryptography and Privacy-Preserving Smart Contracts}, author={Kosba, Ahmed and Miller, Andrew and Shi, Elaine and Wen, Zikai and Papamanthou, Charalampos}, booktitle={2016 IEEE Symposium on Security and Privacy (SP)}, year={2016}, doi={10.1109/sp.2016.55}}
% search-summary-only
@inproceedings{zo2014, title={Z{\O}: An Optimizing Distributing Zero-Knowledge Compiler}, author={Fredrikson, Matthew and Livshits, Benjamin}, booktitle={USENIX Security Symposium}, year={2014}, url={https://www.usenix.org/conference/usenixsecurity14/technical-sessions/presentation/fredrikson}}
% search-summary-only
@inproceedings{zql2013, title={ZQL: A Compiler for Privacy-Preserving Data Processing}, author={Fournet, C{\'e}dric and Kohlweiss, Markulf and Danezis, George and Luo, Zhengqin}, booktitle={USENIX Security Symposium}, year={2013}, url={https://www.usenix.org/conference/usenixsecurity13/technical-sessions/presentation/fournet}}
% search-summary-only. Title page text shows three authors and may be incomplete.
@inproceedings{zkpdl2010, title={ZKPDL: A Language-Based System for Efficient Zero-Knowledge Proofs and Electronic Cash}, author={Meiklejohn, Sarah and Erway, C. Chris and Hinkle, Theodora and others}, url={https://www.usenix.org/legacy/events/sec10/tech/full_papers/Meiklejohn.pdf}}
```

Verified and certified compilation, and circuit verification:

```bibtex
% read-full (abstract, limits, bugs, related work)
@incollection{bounded2023, title={Bounded Verification for Finite-Field-Blasting}, author={Ozdemir, Alex and Wahby, Riad S. and Brown, Fraser and Barrett, Clark}, booktitle={Lecture Notes in Computer Science}, year={2023}, doi={10.1007/978-3-031-37709-9_8}, note={CAV 2023; ePrint 2023/778}, url={https://eprint.iacr.org/2023/778}}
% search-summary-only (journal page redirected to login)
@article{boundedfmsd2025, title={Bounded verification for finite-field-blasting in a compiler for zero knowledge proofs}, author={Ozdemir, Alex and Wahby, Riad S. and Brown, Fraser and Barrett, Clark}, journal={Formal Methods in System Design}, year={2025}, doi={10.1007/s10703-025-00476-3}}
% search-summary-only (metadata only)
@incollection{smtff2023, title={Satisfiability Modulo Finite Fields}, author={Ozdemir, Alex and Kremer, Gereon and Tinelli, Cesare and Barrett, Clark}, booktitle={Lecture Notes in Computer Science}, year={2023}, doi={10.1007/978-3-031-37703-7_8}, note={CAV 2023; ePrint 2023/091}}
% read-abstract (plus contribution and evaluation passages). 77 versus 79 circuits differs by version.
@inproceedings{coda2024, title={Certifying Zero-Knowledge Circuits with Refinement Types}, author={Liu, Junrui and Kretz, Ian and Liu, Hanzhi and Tan, Bryan and Wang, Jonathan and Sun, Yi and Pearson, Luke and Miltner, Anders and Dillig, I{\c{s}}{\i}l and Feng, Yu}, booktitle={2024 IEEE Symposium on Security and Privacy (SP)}, year={2024}, doi={10.1109/sp54263.2024.00078}, note={ePrint 2023/547}}
% read-full (introduction and scope). Described as a preliminary report.
@misc{compositional2023, title={Compositional Formal Verification of Zero-Knowledge Circuits}, author={Coglio, Alessandro and McCarthy, Eric and Smith, Eric and Chin, Collin and Gaddamadugu, Pranav and Dellepere, Michel}, year={2023}, howpublished={Cryptology ePrint Archive, Paper 2023/1278}, url={https://eprint.iacr.org/2023/1278}}
% read-abstract (located passages on extraction, abstract only in part)
@article{coglio2023, title={Formal Verification of Zero-Knowledge Circuits}, author={Coglio, Alessandro and McCarthy, Eric and Smith, Eric W.}, journal={Electronic Proceedings in Theoretical Computer Science}, volume={393}, pages={94--112}, year={2023}, doi={10.4204/eptcs.393.9}, eprint={2311.08858}, archivePrefix={arXiv}}
% search-summary-only
@inproceedings{cairoverified2022, title={A verified algebraic representation of Cairo program execution}, author={Avigad, Jeremy and Goldberg, Lior and Levit, David and Seginer, Yoav and Titelman, Alon}, booktitle={Proceedings of the 11th ACM SIGPLAN International Conference on Certified Programs and Proofs}, year={2022}, doi={10.1145/3497775.3503675}}
% read-abstract (one-sentence page summary)
@misc{stwo2026, title={Formal verification of the S-two AIR}, author={Avigad, Jeremy and Ganor, Anat and Goldberg, Lior and Levit, David and Nir, Ohad and Seginer, Yoav and Titelman, Alon}, year={2026}, eprint={2606.04311}, archivePrefix={arXiv}}
% read-full (via summarizing fetch; Brillig and Lampe passages confirmed in the PDF)
@misc{nave2026, title={Formally Verifying Noir Zero Knowledge Programs with NAVe}, author={Antonino, Pedro and Jain, Namrata}, year={2026}, eprint={2601.09372}, archivePrefix={arXiv}}
% read-abstract (plus located passages)
@article{picus2023, title={Automated Detection of Under-Constrained Circuits in Zero-Knowledge Proofs}, author={Pailoor, Shankara and Chen, Yanju and Wang, Franklyn and Rodr{\'i}guez, Clara and Van Geffen, Jacob and Morton, Jason and Chu, Michael and Gu, Brian and Feng, Yu and Dillig, I{\c{s}}{\i}l}, journal={Proceedings of the ACM on Programming Languages}, year={2023}, doi={10.1145/3591282}, note={ePrint 2023/512}}
% search-summary-only
@inproceedings{civer2024, title={Scalable Verification of Zero-Knowledge Protocols}, author={Isabel, Miguel and Rodr{\'i}guez-N{\'u}{\~n}ez, Clara and Rubio, Albert}, booktitle={2024 IEEE Symposium on Security and Privacy (SP)}, year={2024}, doi={10.1109/sp54263.2024.00133}}
% read-abstract (page summary)
@inproceedings{zkap2024, title={Practical Security Analysis of Zero-Knowledge Proof Circuits}, author={Wen, Hongbo and Stephens, Jon and Chen, Yanju and Ferles, Kostas and Pailoor, Shankara and Charbonnet, Kyle and Dillig, Isil and Feng, Yu}, booktitle={33rd USENIX Security Symposium}, pages={1471--1487}, year={2024}, url={https://www.usenix.org/conference/usenixsecurity24/presentation/wen}}
% read-abstract
@misc{halo2analysis2023, title={Automated Analysis of Halo2 Circuits}, author={Soureshjani, Fatemeh Heidari and Hall-Andersen, Mathias and Jahanara, MohammadMahdi and Kam, Jeffrey and Gorzny, Jan and Ahmadvand, Mohsen}, year={2023}, howpublished={Cryptology ePrint Archive, Paper 2023/1051; SMT 2023}, url={https://eprint.iacr.org/2023/1051}}
% search-summary-only
@incollection{consistency2025, title={Automated Verification of Consistency in Zero-Knowledge Proof Circuits}, author={Stephens, Jon and Pailoor, Shankara and Dillig, Isil}, booktitle={Lecture Notes in Computer Science}, pages={315--338}, year={2025}, doi={10.1007/978-3-031-98668-0_16}}
% search-summary-only
@incollection{joltlookup2026, title={Verifying Jolt zkVM Lookup Semantics}, author={Kwan, Carl and Dao, Quang and Thaler, Justin}, booktitle={Lecture Notes in Computer Science}, pages={163--179}, year={2026}, doi={10.1007/978-3-032-07024-1_10}}
% search-summary-only. Kolozyan et al. cite a Formal Aspects of Computing version; not confirmed.
@misc{ac4, title={AC4: Algebraic Computation Checker for Circuit Constraints in ZKPs}, author={Yang, Qizhe and Liang, Boxuan and Chen, Hao and Li, Guoqiang}, year={2024}, eprint={2403.15676}, archivePrefix={arXiv}}
% search-summary-only
@inproceedings{scalecirc2025, title={ScaleCirc: Scaling the Analysis over Circom Circuits}, author={Jiang, Jinan and Qin, Haoran and Luo, Xiapu}, booktitle={2025 40th IEEE/ACM International Conference on Automated Software Engineering (ASE)}, year={2025}, doi={10.1109/ase63991.2025.00124}}
% read-abstract (page summary). Comment says shorter version accepted at PRDC 2023.
@misc{r1csnorm2023, title={Data-Flow-Based Normalization Generation Algorithm of R1CS for Zero-Knowledge Proof}, author={Shi, Chenhao and Chen, Hao and Liu, Ruibang and Li, Guoqiang}, year={2023}, eprint={2309.04274}, archivePrefix={arXiv}}
% search-summary-only
@incollection{almeida2010, title={A Certifying Compiler for Zero-Knowledge Proofs of Knowledge Based on {$\Sigma$}-Protocols}, author={Almeida, Jos{\'e} Bacelar and Bangerter, Endre and Barbosa, Manuel and Krenn, Stephan and Sadeghi, Ahmad-Reza and Schneider, Thomas}, booktitle={Lecture Notes in Computer Science}, year={2010}, doi={10.1007/978-3-642-15497-3_10}}
% search-summary-only
@inproceedings{almeida2012, title={Full proof cryptography}, author={Almeida, Jos{\'e} Bacelar and Barbosa, Manuel and Bangerter, Endre and Barthe, Gilles and Krenn, Stephan and Zanella B{\'e}guelin, Santiago}, booktitle={Proceedings of the 2012 ACM conference on Computer and communications security}, year={2012}, doi={10.1145/2382196.2382249}}
% search-summary-only
@inproceedings{backes2008, title={Type-checking zero-knowledge}, author={Backes, Michael and Hritcu, C{\u{a}}t{\u{a}}lin and Maffei, Matteo}, booktitle={Proceedings of the 15th ACM conference on Computer and communications security}, year={2008}, doi={10.1145/1455770.1455816}}
```

Foundations of compiler trust and of proving builds:

```bibtex
% search-summary-only
@inproceedings{necula1997, title={Proof-carrying code}, author={Necula, George C.}, booktitle={Proceedings of the 24th ACM SIGPLAN-SIGACT symposium on Principles of programming languages}, year={1997}, doi={10.1145/263699.263712}}
% search-summary-only
@inproceedings{necula1998, title={The design and implementation of a certifying compiler}, author={Necula, George C. and Lee, Peter}, booktitle={Proceedings of the ACM SIGPLAN 1998 conference on Programming language design and implementation}, year={1998}, doi={10.1145/277650.277752}}
% search-summary-only
@incollection{pnueli1998, title={Translation validation}, author={Pnueli, A. and Siegel, M. and Singerman, E.}, booktitle={Lecture Notes in Computer Science}, year={1998}, doi={10.1007/bfb0054170}}
% search-summary-only
@article{leroy2009, title={Formal verification of a realistic compiler}, author={Leroy, Xavier}, journal={Communications of the ACM}, year={2009}, doi={10.1145/1538788.1538814}}
% search-summary-only. Record is the 2007 reprint in ACM Turing award lectures.
@incollection{thompson, title={Reflections on trusting trust}, author={Thompson, Ken}, booktitle={ACM Turing award lectures}, year={2007}, doi={10.1145/1283920.1283940}}
% search-summary-only
@inproceedings{wheeler2005, title={Countering Trusting Trust through Diverse Double-Compiling}, author={Wheeler, D.}, booktitle={21st Annual Computer Security Applications Conference (ACSAC'05)}, doi={10.1109/csac.2005.17}}
% search-summary-only
@article{lamb2022, title={Reproducible Builds: Increasing the Integrity of Software Supply Chains}, author={Lamb, Chris and Zacchiroli, Stefano}, journal={IEEE Software}, year={2022}, doi={10.1109/ms.2021.3073045}}
% read-abstract (plus overhead passages)
@inproceedings{attestable2025, title={Attestable Builds: Compiling Verifiable Binaries on Untrusted Systems using Trusted Execution Environments}, author={Hugenroth, Daniel and Lins, Mario and Mayrhofer, Ren{\'e} and Beresford, Alastair R.}, booktitle={Proceedings of the 2025 ACM SIGSAC Conference on Computer and Communications Security}, year={2025}, doi={10.1145/3719027.3765128}, eprint={2505.02521}, archivePrefix={arXiv}}
% search-summary-only
@inproceedings{valiant1976, title={Universal circuits (Preliminary Report)}, author={Valiant, Leslie G.}, booktitle={Proceedings of the eighth annual ACM symposium on Theory of computing}, year={1976}, doi={10.1145/800113.803649}}
% search-summary-only
@inproceedings{zexe2020, title={ZEXE: Enabling Decentralized Private Computation}, author={Bowe, Sean and Chiesa, Alessandro and Green, Matthew and Miers, Ian and Mishra, Pratyush and Wu, Howard}, booktitle={2020 IEEE Symposium on Security and Privacy (SP)}, year={2020}, doi={10.1109/sp40000.2020.00050}}
% search-summary-only
@inproceedings{bitansky2013, title={Recursive composition and bootstrapping for SNARKS and proof-carrying data}, author={Bitansky, Nir and Canetti, Ran and Chiesa, Alessandro and Tromer, Eran}, booktitle={Proceedings of the forty-fifth annual ACM symposium on Theory of Computing}, year={2013}, doi={10.1145/2488608.2488623}}
% search-summary-only
@inproceedings{hashfirst2016, title={Hash First, Argue Later}, author={Fiore, Dario and Fournet, C{\'e}dric and Ghosh, Esha and Kohlweiss, Markulf and Ohrimenko, Olga and Parno, Bryan}, booktitle={Proceedings of the 2016 ACM SIGSAC Conference on Computer and Communications Security}, year={2016}, doi={10.1145/2976749.2978368}}
% search-summary-only
@incollection{nova2022, title={Nova: Recursive Zero-Knowledge Arguments from Folding Schemes}, author={Kothapalli, Abhiram and Setty, Srinath and Tzialla, Ioanna}, booktitle={Lecture Notes in Computer Science}, year={2022}, doi={10.1007/978-3-031-15985-5_13}}
% search-summary-only
@misc{halo2019, title={Recursive Proof Composition without a Trusted Setup}, author={Bowe, Sean and Grigg, Jack and Hopwood, Daira}, year={2019}, howpublished={Cryptology ePrint Archive, Paper 2019/1021}, url={https://eprint.iacr.org/2019/1021}}
% search-summary-only
@inproceedings{maffei2011, title={Privacy-aware proof-carrying authorization}, author={Maffei, Matteo and Pecina, Kim}, booktitle={Proceedings of the ACM SIGPLAN 6th Workshop on Programming Languages and Analysis for Security}, year={2011}, doi={10.1145/2166956.2166963}}
% search-summary-only
@inproceedings{appel1999, title={Proof-carrying authentication}, author={Appel, Andrew W. and Felten, Edward W.}, booktitle={Proceedings of the 6th ACM conference on Computer and communications security}, year={1999}, doi={10.1145/319709.319718}}
```

ZK for logic, queries and analysis:

```bibtex
% read-abstract (plus located passages)
@inproceedings{zkunsat2022, title={Proving UNSAT in Zero Knowledge}, author={Luo, Ning and Antonopoulos, Timos and Harris, William R. and Piskac, Ruzica and Tromer, Eran and Wang, Xiao}, booktitle={Proceedings of the 2022 ACM SIGSAC Conference on Computer and Communications Security}, year={2022}, doi={10.1145/3548606.3559373}, note={ePrint 2022/206}}
% read-abstract (page summary)
@inproceedings{zksmt2024, title={ZKSMT: A VM for Proving SMT Theorems in Zero Knowledge}, author={Luick, Daniel and Kolesar, John C. and Antonopoulos, Timos and Harris, William R. and Parker, James and Piskac, Ruzica and Tromer, Eran and Wang, Xiao and Luo, Ning}, booktitle={USENIX Security Symposium}, year={2024}, url={https://www.usenix.org/conference/usenixsecurity24/presentation/luick}}
% read-abstract (plus located passages)
@inproceedings{zkpi2024, title={zkPi: Proving Lean Theorems in Zero-Knowledge}, author={Laufer, Evan and Ozdemir, Alex and Boneh, Dan}, booktitle={Proceedings of the 2024 ACM SIGSAC Conference on Computer and Communications Security}, year={2024}, doi={10.1145/3658644.3670322}, note={ePrint 2024/267}}
% read-abstract
@misc{zkmc2026, title={Zero-Knowledge Model Checking}, author={Berrang, Pascal and Giacobbe, Mirco and Swales, Jacob and Yang, Xiao}, year={2026}, eprint={2605.00487}, archivePrefix={arXiv}}
% search-summary-only
@inproceedings{pspace2026, title={Towards Practical Zero-Knowledge Proof for PSPACE}, author={Karthikeyan, Ashwin and Liu, Hengyu and Meel, Kuldeep S. and Luo, Ning}, booktitle={2026 IEEE Symposium on Security and Privacy (SP)}, year={2026}, doi={10.1109/sp63933.2026.00041}}
% search-summary-only (metadata only)
@inproceedings{reef2024, title={Reef: Fast Succinct Non-Interactive Zero-Knowledge Regex Proofs}, author={Angel, Sebastian and Ioannidis, Eleftherios and Margolin, Elizabeth and Setty, Srinath and Woods, Jess}, booktitle={USENIX Security Symposium}, year={2024}, url={https://www.usenix.org/conference/usenixsecurity24/presentation/angel}, note={ePrint 2023/1886}}
% search-summary-only
@inproceedings{coral2026, title={Coral: Fast Succinct Non-Interactive Zero-Knowledge CFG Proofs}, author={Angel, Sebastian and Celi, Sof{\'i}a and Margolin, Elizabeth and Mishra, Pratyush and Sander, Martin and Woods, Jess}, booktitle={2026 IEEE Symposium on Security and Privacy (SP)}, year={2026}, doi={10.1109/sp63933.2026.00059}}
% search-summary-only
@inproceedings{zkmb2022, title={Zero-Knowledge Middleboxes}, author={Grubbs, Paul and Arun, Arasu and Zhang, Ye and Bonneau, Joseph and Walfish, Michael}, booktitle={USENIX Security Symposium}, year={2022}, url={https://www.usenix.org/conference/usenixsecurity22/presentation/grubbs}}
% search-summary-only
@inproceedings{zombie2024, title={Zombie: Middleboxes that Don't Snoop}, author={Zhang, Collin and DeStefano, Zachary and Arun, Arasu and Bonneau, Joseph and Grubbs, Paul and Walfish, Michael}, booktitle={USENIX Symposium on Networked Systems Design and Implementation (NSDI)}, year={2024}, url={https://www.usenix.org/conference/nsdi24/presentation/zhang-collin}}
% search-summary-only
@inproceedings{vsql2017, title={vSQL: Verifying Arbitrary SQL Queries over Dynamic Outsourced Databases}, author={Zhang, Yupeng and Genkin, Daniel and Katz, Jonathan and Papadopoulos, Dimitrios and Papamanthou, Charalampos}, booktitle={2017 IEEE Symposium on Security and Privacy (SP)}, year={2017}, doi={10.1109/sp.2017.43}}
% search-summary-only
@article{zksql2023, title={ZKSQL: Verifiable and Efficient Query Evaluation with Zero-Knowledge Proofs}, author={Li, Xiling and Weng, Chenkai and Xu, Yongxin and Wang, Xiao and Rogers, Jennie}, journal={Proceedings of the VLDB Endowment}, year={2023}, doi={10.14778/3594512.3594513}}
% search-summary-only
@article{poneglyph2025, title={PoneglyphDB: Efficient Non-interactive Zero-Knowledge Proofs for Arbitrary SQL-Query Verification}, author={Gu, Binbin and Fang, Juncheng and Nawab, Faisal}, journal={Proceedings of the ACM on Management of Data}, year={2025}, doi={10.1145/3709713}, eprint={2411.15031}, archivePrefix={arXiv}}
% search-summary-only
@inproceedings{integridb2015, title={IntegriDB}, author={Zhang, Yupeng and Katz, Jonathan and Papamanthou, Charalampos}, booktitle={Proceedings of the 22nd ACM SIGSAC Conference on Computer and Communications Security}, year={2015}, doi={10.1145/2810103.2813711}}
% search-summary-only
@inproceedings{zkstatic2021, title={Zero Knowledge Static Program Analysis}, author={Fang, Zhiyong and Darais, David and Near, Joseph P. and Zhang, Yupeng}, booktitle={Proceedings of the 2021 ACM SIGSAC Conference on Computer and Communications Security}, year={2021}, doi={10.1145/3460120.3484795}}
% search-summary-only
@inproceedings{cheesecloth2023, title={Cheesecloth: Zero-Knowledge Proofs of Real World Vulnerabilities}, author={Cu{\'e}llar, Santiago and Harris, Bill and Parker, James and Pernsteiner, Stuart and Tromer, Eran}, booktitle={USENIX Security Symposium}, year={2023}, url={https://www.usenix.org/conference/usenixsecurity23/presentation/cuellar}}
% search-summary-only. Journal version, with an added author.
@article{cheesecloth2025, title={Cheesecloth: Zero-Knowledge Proofs of Real-World Vulnerabilities}, author={Cu{\'e}llar Gempeler, Santiago and Harris, Bill and Parker, James and Pernsteiner, Stuart and Sweet, Ian and Tromer, Eran}, journal={ACM Transactions on Privacy and Security}, year={2025}, doi={10.1145/3747589}}
```

Bugs and testing:

```bibtex
% read-full (abstract, taxonomy, defenses)
@inproceedings{sok2024, title={SoK: What Don't We Know? Understanding Security Vulnerabilities in SNARKs}, author={Chaliasos, Stefanos and Ernstberger, Jens and Theodore, David and Wong, David and Jahanara, Mohammad and Livshits, Benjamin}, booktitle={USENIX Security Symposium}, year={2024}, url={https://www.usenix.org/conference/usenixsecurity24/presentation/chaliasos}, eprint={2402.15293}, archivePrefix={arXiv}}
% read-full (abstract, research-question findings, formal verification and trust sections)
@misc{kolozyan2026, title={ZKP Security Tools and Verification: Coverage, Effectiveness, Adoption, and Challenges}, author={Kolozyan, Arman and Sorger, Tom and Hicks, Alexander and Chaliasos, Stefanos}, year={2026}, eprint={2607.23752}, archivePrefix={arXiv}}
% read-full (abstract, results, Noir bugs)
@misc{liezz2026, title={Lie to Me: Finding Bugs in ZK DSL Toolchains with Adversarial Witness Injection}, author={Watzinger, Sebastian and Hochrainer, Christoph and W{\"u}stholz, Valentin and Christakis, Maria}, year={2026}, eprint={2608.30648}, archivePrefix={arXiv}}
% read-abstract (plus Noir passages)
@inproceedings{circuzz2025, title={Fuzzing Processing Pipelines for Zero-Knowledge Circuits}, author={Hochrainer, Christoph and Isychev, Anastasia and W{\"u}stholz, Valentin and Christakis, Maria}, booktitle={Proceedings of the 2025 ACM SIGSAC Conference on Computer and Communications Security}, year={2025}, doi={10.1145/3719027.3744791}, eprint={2411.02077}, archivePrefix={arXiv}}
% read-abstract (page summary)
@inproceedings{mtzk2025, title={MTZK: Testing and Exploring Bugs in Zero-Knowledge (ZK) Compilers}, author={Xiao, Dongwei and Liu, Zhibo and Peng, Yiteng and Wang, Shuai}, booktitle={Network and Distributed System Security Symposium}, year={2025}, doi={10.14722/ndss.2025.230530}}
% read-abstract
@inproceedings{arguzz2026, title={Arguzz: Testing zkVMs for Soundness and Completeness Bugs}, author={Hochrainer, Christoph and W{\"u}stholz, Valentin and Christakis, Maria}, booktitle={USENIX Security Symposium}, year={2026}, url={https://www.usenix.org/conference/usenixsecurity26/presentation/hochrainer}, eprint={2509.10819}, archivePrefix={arXiv}}
% search-summary-only
@inproceedings{zkfuzz2026, title={zkFuzz: Foundation and Framework for Effective Fuzzing of Zero-Knowledge Circuits}, author={Takahashi, Hideaki and Kim, Jihwan and Jana, Suman and Yang, Junfeng}, booktitle={2026 IEEE Symposium on Security and Privacy (SP)}, year={2026}, doi={10.1109/sp63933.2026.00049}, eprint={2504.11961}, archivePrefix={arXiv}}
% search-summary-only
@misc{tang2024, title={Zero-Knowledge Proof Vulnerability Analysis and Security Auditing}, author={Tang, Xueyan and Shi, Lingzhi and Wang, Xun and Charbonnet, Kyle and Tang, Shixiang and Sun, Shixiao}, year={2024}, howpublished={Cryptology ePrint Archive, Paper 2024/514}, url={https://eprint.iacr.org/2024/514}}
% search-summary-only
@misc{fiatshamir2025, title={How to Prove False Statements: Practical Attacks on Fiat-Shamir}, author={Khovratovich, Dmitry and Rothblum, Ron D. and Soukhanov, Lev}, year={2025}, howpublished={Cryptology ePrint Archive, Paper 2025/118}, url={https://eprint.iacr.org/2025/118}}
```

Software and documentation, not papers: [Noir](https://noir-lang.org), [Lampe](https://github.com/reilabs/lampe), [Garden](https://github.com/formal-land/garden), [Clean](https://github.com/Verified-zkEVM/clean), [CertiPlonk](https://github.com/NethermindEth/CertiPlonk), [zkLean](https://github.com/GaloisInc/zk-lean), [LLZK](https://github.com/project-llzk/llzk-lib), [zkLLVM](https://github.com/NilFoundation/zkLLVM), [Vamp-IR](https://github.com/anoma/vamp-ir), [Pequin](https://github.com/pepper-project/pequin).

### Inferences
- About a dozen papers carry most of the argument: vkproof, Ron and Monperrus, ZK-TPL, Cinderella, CirC, the bounded verification paper, NAVe, LIEZZ, Circuzz, the SoK, Kolozyan et al. and Lurk. A report writer should check those first if time is short.

### Gaps
- `search-summary-only` entries are verified for metadata, but their contents were not read in this session, and this applies to many foundational papers. Do not attribute specific results to them from these notes.
- SymCert's author list and Prezta's venue need confirming from the papers.
