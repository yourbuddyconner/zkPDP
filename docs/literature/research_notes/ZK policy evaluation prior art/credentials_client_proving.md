# Anonymous credentials and client-side zero-knowledge identity, as prior art for proving a policy decision over private attributes

Notes for the report writer. Written 2026-11-04. Reading status for every paper is in the last section. Local paths point into the zk-pdp repository.

Short verdict, argued in the section "What would zk-pdp's private mode add":

- The cryptographic capability is not new. Any SNARK can prove an arbitrary predicate over signed attributes, and research systems already do it with hand-written circuits.
- Deployed systems stop at selective disclosure and fixed predicates.
- I found no system that takes Cedar, or any mainstream authorization language, and proves its decision with attributes hidden. The nearest are ZK-TPL (2022), Cinderella (2016), and a design note in the OpenAC repository.
- So zk-pdp adds a compiled, conformance-tested policy language in front of a known capability. For simple predicates it adds nothing and costs more. As specified, it also lacks properties anonymous credentials already have, above all unlinkability.

## Classic anonymous credentials: how far do they go?

### Takeaway
Classic schemes give selective disclosure plus a small menu of predicates that are efficient for the signature's algebra. They require issuers to adopt new signatures, and that is the barrier the newer work attacks. Attribute-based signatures reach circuit-shaped policies in theory, but as a new signature primitive with no bridge to deployed credentials.

### Cited Findings
- EU architecture text describes BBS+ as signing a list of messages with one constant-size signature, then proving knowledge of the signature while disclosing a sub-array of messages. It lists low overhead as the advantage and the need to change the issuer's signing process as the disadvantage. — [EUDI ARF Topic G](https://github.com/eu-digital-identity-wallet/eudi-doc-architecture-and-reference-framework/blob/main/docs/discussion-topics/g-zero-knowledge-proof.md)
- BBS# removes pairing-friendly curves and works with ECDSA device keys, but needs issuer interaction that scales linearly with presentations. — [EUDI ARF Topic G](https://github.com/eu-digital-identity-wallet/eudi-doc-architecture-and-reference-framework/blob/main/docs/discussion-topics/g-zero-knowledge-proof.md); the authors report implementations on several smartphones. — [Desmoulins et al., ePrint 2025/619](https://eprint.iacr.org/2025/619)
- Server-Aided Anonymous Credentials is a provably secure variant of the BBS# idea that keeps full unlinkability. — [EUDI ARF Topic G](https://github.com/eu-digital-identity-wallet/eudi-doc-architecture-and-reference-framework/blob/main/docs/discussion-topics/g-zero-knowledge-proof.md), [ePrint 2025/513](https://eprint.iacr.org/2025/513)
- Status of the standards: the BBS signature draft is at revision 14 (2026-10-29) and Blind BBS at revision 05 (2026-10-30), both IRTF informational-track drafts. — [datatracker BBS](https://datatracker.ietf.org/doc/draft-irtf-cfrg-bbs-signatures/), [datatracker Blind BBS](https://datatracker.ietf.org/doc/draft-irtf-cfrg-bbs-blind-signatures/). The W3C Data Integrity BBS Cryptosuites spec is a Candidate Recommendation, expected complete 2027-04-01. — [W3C VC WG charter](https://www.w3.org/2026/03/vc-wg-charter.html)
- Predicates in practice: Babel and Sedlmeir give set membership, non-membership (for revocation) and range proofs as the standard examples of predicates. They call AnonCreds, which underlies Hyperledger Aries, "arguably among the implementations with the most sophisticated privacy functionalities", then argue for general-purpose zk-SNARKs because they allow "customizable predicates". — [Babel and Sedlmeir, arXiv 2301.00823](https://arxiv.org/abs/2301.00823)
- A comparison of selective disclosure mechanisms treats predicate proofs as one criterion. For salted-hash mechanisms it says predicates depend on the issuer, and that unlinkability needs the issuer to supply single-use credentials in advance. CL and BBS can create predicate proofs. — [Flamini et al., arXiv 2401.08196](https://arxiv.org/abs/2401.08196)
- Coconut offers threshold issuance, public and private attributes, re-randomization and multi-show unlinkable selective reveal, with primitives taking a few milliseconds (verification 10 ms, the slowest). It does not describe a predicate language. — [Sonnino et al., arXiv 1802.07344](https://arxiv.org/abs/1802.07344)
- Privacy Pass carries issuance and redemption of tokens, standardized in RFC 9576 (architecture), RFC 9577 (HTTP authentication) and RFC 9578 (issuance). — [RFC 9576](https://datatracker.ietf.org/doc/rfc9576/), [RFC 9577](https://datatracker.ietf.org/doc/rfc9577/), [RFC 9578](https://datatracker.ietf.org/doc/rfc9578/)
- A post-quantum Privacy Pass built from a general-purpose proof yields 85 to 175 KB tokens, 0.3 to 5 s generation and 20 to 30 ms verification. Its abstract states that generic construction of credentials "supporting the showing of arbitrary predicates" has been known since Fischlin (CRYPTO 2006). — [Policharla et al., ePrint 2023/414](https://eprint.iacr.org/2023/414)
- C0C0 (2015) already evaluated composable SNARKs for applications including anonymous credentials, with proving keys of hundreds of MB and proving times of tens to hundreds of seconds. — [Kosba et al., ePrint 2015/1093](https://eprint.iacr.org/2015/1093), figures as reported by [Babel and Sedlmeir](https://arxiv.org/abs/2301.00823)
- Attribute-based signatures for circuits remain an active line, with work on optimal parameters from standard assumptions and code-based constructions. — [ePrint 2024/1129](https://eprint.iacr.org/2024/1129), [ePrint 2024/022](https://eprint.iacr.org/2024/022) (titles and author lists only)
- A 2026 SoK splits the field into two architectures: multi-message signatures with efficient proofs, and general-purpose proofs over legacy ECDSA. — [Bormann and Lehmann, ePrint 2026/330](https://eprint.iacr.org/2026/330)

### Inferences
- The classic line lands at "selective disclosure, plus range and set membership". A new kind of condition needs a new sigma protocol or a composition with a SNARK. A conjunction of a few comparisons fits. Cedar's entity hierarchy and forbid-overrides-permit semantics do not map onto it directly.
- From background knowledge, not checked this session: attribute-based signatures (Maji et al.) bind a signature to a policy over attributes, and policy-based signatures (Bellare and Fuchsbauer) let a signer sign only messages conforming to a policy. Both need new issuance, so neither helps with JWTs or mdocs. Treat this as framing, not as a cited finding.

### Gaps
- I did not read the original Camenisch-Lysyanskaya, idemix, U-Prove, BBS, Coconut or attribute-based-signature papers. Metadata is verified, content claims above come from later papers and specifications.
- The U-Prove specification itself was not retrieved. Only Brands's monograph has verified metadata.
- I could not map the columns of Flamini et al.'s predicate row from text extraction, so I do not report it cell by cell.

## ZK over existing credentials: how far do they go?

### Takeaway
This line solved the deployment problem: no issuer changes, and proofs fast enough for a phone. The statement is nearly always a fixed credential-validity check plus a small disclosure or predicate. Where predicates are programmable, the program is a hand-written circuit chosen ahead of time.

### Cited Findings
- **Longfellow, Google.** Sumcheck plus Ligero over P-256 and SHA-256, with parsing of ISO identity formats, no issuer or device changes, relying on SHA-256 as its only complexity assumption. — [Frigo and shelat, CIC 3(1) 2026](https://cic.iacr.org/p/3/1/7). The library targets ISO mdoc, JWT and W3C VCs, and its Lean development proves conditional bounds for mathematical models, not the implementation. — [longfellow-zk README](https://github.com/google/longfellow-zk). An independent review says its tooling makes it "very difficult to write new circuits or compose proofs". — [EPFL review](https://eid-privacy.github.io/wp0/2025/11/28/crescent-longfellow-showdown.html)
- **Longfellow deployment.** Google open sourced it on 2025-07-03 for EU age assurance. — [Google blog](https://blog.google/technology/safety-security/opening-up-zero-knowledge-proof-technology-to-promote-privacy-in-age-assurance/) The IETF document is an individual draft at revision 03 (2026-09-23). — [datatracker](https://datatracker.ietf.org/doc/draft-google-cfrg-libzk/)
- **Crescent, Microsoft.** Groth16 once per credential, then a fast show step, for JWT and mDL. — [Paquin et al., ePrint 2024/2013](https://eprint.iacr.org/2024/2013). The repository offers fixed parameter sets: `rs256` (which "hardcod[es] the disclosure of the user's email domain"), `rs256-sd`, `rs256-db` and `mdl1`. It is unaudited. — [crescent-credentials README](https://github.com/microsoft/crescent-credentials). Each attribute can be hidden, committed or revealed. — [EPFL review](https://eid-privacy.github.io/wp0/2025/11/28/crescent-longfellow-showdown.html)
- **Vega, Microsoft and UC Berkeley, IEEE S&P 2026.** Fold-and-reuse plus lookup-based field extraction instead of full in-circuit parsing, transparent setup. It evaluates one scenario: age threshold plus device binding. It observes that any change such as a "new predicate" forces a new trusted setup in systems that use one, and that client storage then grows with the number of predicates. — [Kaviani and Setty](https://www.microsoft.com/en-us/research/wp-content/uploads/2026/05/Vega_Low-Latency-Zero-Knowledge-Proofs-over-Existing-Credentials.pdf). Its book "treats circuits abstractly" and leaves credential arithmetization out of scope. — [Vega book](https://microsoft.github.io/vega-prover/)
- **OpenAC, Ethereum Foundation PSE.** Transparent, EUDI-aligned. A policy-independent Prepare proof over the credential and a Show proof that links to it through a re-randomized commitment. — [Eagen et al., ePrint 2026/251](https://eprint.iacr.org/2026/251), [paper source](https://github.com/privacy-ethereum/zkID/blob/main/paper/experiments.tex). A separate design note is covered in the policy language section below.
- **GordieHowe, University of Michigan.** "A language, a certifying compiler, and a zero-knowledge proof protocol", for credentials over ECDSA, RSA and ML-DSA. The compiler emits a Lean certificate that its optimizations preserve semantics, and the proof system's security theorems are machine-checked. — [Sharp and Grubbs, ePrint 2026/2615](https://eprint.iacr.org/2026/2615)
- **zkLogin, Sui.** Fixed statement about an OpenID JWT, Groth16 and Circom, around 1.1 million R1CS constraints, deployed with a delegated prover service because browser proving at that size "can lead to crashes or long delays". — [Baldimtsi et al., CCS 2024](https://arxiv.org/abs/2401.11735)
- **Attacks on zkLogin.** Security depends on JWT and JSON parsing, issuer trust policy, architectural binding and execution-environment integrity, none of which are protocol-level properties. Non-canonical claim extraction admits malformed JWTs. — [Celi et al., ePrint 2026/227](https://eprint.iacr.org/2026/227)
- **zk-creds.** General-purpose proofs let credentials be issued to a bulletin board, convert existing documents without issuer help, and allow "flexible, composable, and complex identity statements over multiple credentials". Passport age checks take under 150 ms. — [Rosenberg et al., ePrint 2022/878](https://eprint.iacr.org/2022/878)
- **zk-promises.** Stateful anonymous credentials whose state machines are Turing-complete with asynchronous callbacks. Groth16, Poseidon, Schnorr over Jubjub, written in Rust with Arkworks. — [Shih et al., USENIX Security 2025](https://www.usenix.org/conference/usenixsecurity25/presentation/shih)
- **Delegation and revocation.** The first delegatable credentials over legacy credentials use recursive SNARKs. — [ePrint 2026/1855](https://eprint.iacr.org/2026/1855). ZEBRA targets on-chain access control with auditability and revocation. — [ePrint 2022/1286](https://eprint.iacr.org/2022/1286)
- **zk-email.** A regex proof system based on path verification over epsilon-free NFAs, prover cost linear in the captured path. — [Londhe et al., PoPETs 2026](https://doi.org/10.56553/popets-2026-0143)
- **Anon Aadhaar.** About 1.7 million constraints, with SHA-256 the main cost. — [MoPerf design note](https://hackmd.io/5ItB2D50QcavF18cWIrmfQ) (informal, undated)
- **Self (formerly OpenPassport).** Register and disclose Circom circuits, proving validity while "only revealing specific attributes such as age, nationality or simply humanity". — [Self README](https://github.com/selfxyz/self). A January 2026 case study places its proving in Google Cloud Confidential Space, which means not purely on the client. — [Google case study](https://services.google.com/fh/files/misc/self_case_study.pdf) (search summary only). The circuits have an audit. — [zkSecurity](https://reports.zksecurity.xyz/reports/celo-self-audit.md) (search summary only)
- **zkPassport.** The repository explains the passport trust chain and notes that states pick among RSA, DSA and ECDSA, so the circuits must cover many signature combinations. — [zkpassport/circuits](https://github.com/zkpassport/circuits). Its work underlies Mopro's Noir support. — [Mopro Noir adapter](https://431a6bc2.mopro.pages.dev/docs/adapters/noir)
- **zkTLS.** GovBind combines Proxy-mode TLSNotary provenance with a document-profile-specific proof that constrains DEFLATE decompression, ending in a predicate over a government PDF. — [Gürlek and Ağırtaş, ePrint 2026/2206](https://eprint.iacr.org/2026/2206). Other 2026 entries: [Ticket to Hide, ePrint 2026/540](https://eprint.iacr.org/2026/540), [Reclaim, ePrint 2026/2187](https://eprint.iacr.org/2026/2187) (titles only). DECO has verified metadata only. — [Zhang et al., CCS 2020](https://doi.org/10.1145/3372297.3417239)
- **Zero-Knowledge Middleboxes.** Clients prove encrypted traffic follows a network policy, such as a DNS blocklist, on unmodified TLS 1.3. Middlebox overhead is 2 to 5 ms per proof. The client stores hundreds of MB and sees tens of seconds at setup and several seconds per packet. — [Grubbs et al., USENIX Security 2022](https://www.usenix.org/conference/usenixsecurity22/presentation/grubbs) (search summary)
- **Belgian eID.** R1CS parsing of a national ID card, date-of-birth extraction in 22 s on a consumer device, 230 ms verification. — [Godden et al., ePrint 2025/1266](https://eprint.iacr.org/2025/1266)
- **SD-JWT and W3C.** SD-JWT is RFC 9901. — [datatracker](https://datatracker.ietf.org/doc/rfc9901/). SD-JWT VC is a draft at revision 23. — [datatracker](https://datatracker.ietf.org/doc/draft-ietf-oauth-sd-jwt-vc/)
- **EUDI ZK work.** TS13 is exploratory, specifies arithmetic-circuit proofs "without trusted setup", is to be tested in Age Verification, and says "the current MDOC and SD-JWT standards only allow asserting equality on the disclosed attributes". It suggests specializing circuits by predicate, with the simplest profile supporting "only the equality predicate". It also says deployment needs few circuits. — [TS13](https://github.com/eu-digital-identity-wallet/eudi-doc-standards-and-technical-specifications/blob/main/docs/technical-specifications/ts13-zksnarks.md). ARF Topic G lists "Per use case circuits are required" as a disadvantage of programmable proofs, and "arbitrary properties" as an advantage. — [Topic G](https://github.com/eu-digital-identity-wallet/eudi-doc-architecture-and-reference-framework/blob/main/docs/discussion-topics/g-zero-knowledge-proof.md). A Münster group judged it unlikely in 2026 that ZKPs will be in the first wallet release. — [University of Münster](https://www.wi.uni-muenster.de/node/5104) (search summary); the Spanish data protection authority's response is also public. — [AEPD](https://www.aepd.es/documento/topicg-aepd-response.pdf) (search summary)

### Inferences
- Deployed systems sit at fixed predicates. Research systems allow arbitrary statements but ship one or a few circuits. The driver is operational: the EU text wants few circuits, and each circuit needs auditing and distribution.
- Vega's and OpenAC's split (once per credential, then a cheap show) is the architecture to compare with. It keeps parse and signature cost out of each decision. zk-pdp's monolithic Noir circuit pays that cost on every proof unless a split is found.

### Gaps
- I could not read the full Longfellow, Crescent, zk-creds or OpenAC paper PDFs. See the bibliography section for why.
- No paper exists for zkPassport or Self that I could find. Claims rest on repositories and case studies.
- I found no peer-reviewed description of Anon Aadhaar's circuit.

## Client-side proving measurements: gates, time, memory, device

### Takeaway
For credential statements, phones now reach about 100 ms for a show step and a few seconds for a full credential proof. Noir with UltraHonk has good public numbers on an M1 laptop, but its phone evidence is thin and includes an out-of-memory result in iPhone Safari. Most of this is self-reported, and peer-reviewed mobile data is rare.

### Cited Findings

Phones:

| System and workload | Size | Device | Time | Memory | Proof | Source |
|---|---|---|---|---|---|---|
| OpenAC Show (device-key ECDSA, nonce) | not stated | iPhone 17 | prove 99 ms, reblind 30 ms, verify 13 ms | not stated | 40.41 kB | [paper source](https://github.com/privacy-ethereum/zkID/blob/main/paper/experiments.tex) |
| same | | Pixel 10 Pro | prove 340 ms, reblind 125 ms, verify 61 ms | | 40.41 kB | same |
| OpenAC Prepare (SD-JWT parse and signature) | not stated | iPhone 17 | prove 2987 ms, reblind 856 ms, verify 151 ms, key setup 3499 ms | not stated | 109.29 kB | same |
| same | | Pixel 10 Pro | prove 7318 ms, reblind 1750 ms, verify 318 ms, key setup 9233 ms | | 109.29 kB | same |
| Longfellow, mdoc with age | not stated | Pixel 6 Pro | about 800 ms, verify about 0.6 s | design target under 0.5 GB per [RWC 2025 slides](https://iacr.org/submit/files/slides/2025/rwc/rwc2025/17/17_slides.pdf), not a measurement | about 400 KB | [TS13](https://github.com/eu-digital-identity-wallet/eudi-doc-standards-and-technical-specifications/blob/main/docs/technical-specifications/ts13-zksnarks.md), citing the paper |
| Longfellow, mdoc with issuer and holder signature, revocation, age | not stated | Pixel 6 Pro | 1.17 s, verify 0.68 s | not stated | about 300 KB | [EPFL review](https://eid-privacy.github.io/wp0/2025/11/28/crescent-longfellow-showdown.html) |
| ShadowPath credential status, Groth16 | not captured | iPhone 17 Pro; Galaxy S25; Galaxy A54 | 3.11 s; 2.93 s; 10.46 s | not captured | not captured | [Herbke et al., arXiv 2608.19937](https://arxiv.org/abs/2608.19937) |
| ShadowPath, PLONK | not captured | iPhone 17 Pro; Galaxy S25 | 39.12 s; 55.29 s | peak about 5.0 GB across devices, with thermal states up to severe | 1352 B | same |
| Anon Aadhaar, RSA path | about 160k constraints | iPhone 14 Pro Max | about 6.5 s, of which about 5 s is WASM witness generation | not stated | Groth16 | [Mopro slides](https://docs.google.com/presentation/d/1afIEgm8oYRvteWxUd04CcMOxChAiHaD55d5AKd0RkvY/htmlpresent) |
| Anon Aadhaar, full | 1.7M constraints | iPhone 14 Pro Max (6 GB) | about 30 s | proving key about 500 MB | Groth16 | [MoPerf note](https://hackmd.io/5ItB2D50QcavF18cWIrmfQ) |
| Noir UltraHonk P-256, multi-thread browser | not stated | Galaxy A23; iPhone 16 and Pro | about 6 s; out of memory | not stated | | [p256-proving-benchmark](https://github.com/vladfdp/p256-proving-benchmark) |
| Cairo M Fibonacci | n up to 100,000 | 1,420 device models | most 3 to 10 s, best at or under 3 s | below 3 GB RAM proving fails systematically, 3 to 4 GB crash-prone | | [FibRace, arXiv 2510.14693](https://arxiv.org/abs/2510.14693) |
| Binius SHA-256, 2048 B | | unnamed phones | about 5 s | 22 to 45 MB | | [CSP Q1'26 report](https://hackmd.io/@clientsideproving/report-q1) |

Laptop and desktop, same machine across rows (AWS mac2.metal, Apple M1, 8 cores, 16 GB). "Constraints" is the suite's `num_constraints` field. Results dated 2026-05-29, taken from [csp-benchmarks results](https://github.com/privacy-ethereum/csp-benchmarks/tree/main/results):

| System | Target | Constraints | Prove | Peak RAM | Verify | Proof | Preprocessing |
|---|---|---|---|---|---|---|---|
| Barretenberg (Noir) | ECDSA, 32-byte digest | 72,683 | 0.99 s | 166 MB | 18.8 ms | 14.7 KB | 8.4 MB |
| Barretenberg | SHA-256, 128 B | 36,000 | 0.61 s | 91 MB | 18.9 ms | 14.7 KB | 8.5 MB |
| Barretenberg | SHA-256, 2048 B | 153,803 | 2.01 s | 416 MB | 18.8 ms | 14.7 KB | 17.3 MB |
| ProveKit (Noir, Spartan and WHIR) | ECDSA | 143,282 | 1.91 s | 571 MB | 244.9 ms | 3.07 MB | 0.8 MB |
| ProveKit | SHA-256, 2048 B | 345,399 | 1.97 s | 1.20 GB | 389.2 ms | 3.23 MB | 3.3 MB |
| ProveKit-Groth16 | ECDSA | 143,282 | 1.97 s | 727 MB | 14.0 ms | 200 B | 77.3 MB |
| Spartan2 | SHA-256, 2048 B | 1,048,576 | 0.54 s | 2.05 GB | 86.9 ms | 78.7 KB | 216.4 MB |
| Circom Groth16 | SHA-256, 2048 B | 1,050,176 | 4.81 s | 1.35 GB | 422.8 ms | 1.0 KB | 621.2 MB |
| Ligetron | ECDSA | 7,773,895 | 18.2 s | 337 MB | 6.05 s | 12.8 MB | 1.4 MB |
| Ligetron | SHA-256, 2048 B | 2,415,600 | 5.72 s | 151 MB | 1.55 s | 7.59 MB | none |
| RISC Zero | ECDSA | n/a | 87.6 s | 4.97 GB | 12.4 ms | 223 KB | 0.4 MB |
| RISC Zero | SHA-256, 2048 B | n/a | 18.5 s | 1.59 GB | 12.5 ms | 223 KB | 0.3 MB |

Other machines:

| System and workload | Size | Device | Time | Memory | Proof | Source |
|---|---|---|---|---|---|---|
| zk-pdp TinyTodo decision, Noir UltraHonk | 353,518 gates | M1 Max | 2.2 to 2.8 s native; 4 to 7 s browser; verify 20 ms | not yet reported | 14.7 KB | [README](/Users/connerswann/code/zk-pdp/README.md), commit 7c0ac62 |
| zk-pdp largest sample circuit | not stated here | M1 Max | 5 s native, about 45 s browser | | | [README](/Users/connerswann/code/zk-pdp/README.md) |
| Vega, mDL 1920 B, age and device binding | not stated | Azure F16as v7, 16 vCPU, 64 GB | 92 ms at 16 threads, 105 ms at 4, 237 ms at 1; verify 23 to 57 ms; precompute 65 to 390 ms | 100 MB precompute, 562 MB online | 108 kB; proving key 464 kB | [Vega](https://www.microsoft.com/en-us/research/wp-content/uploads/2026/05/Vega_Low-Latency-Zero-Knowledge-Proofs-over-Existing-Credentials.pdf) |
| Longfellow, measured by Vega | | same, 1 thread | prove 501 ms, verify 244 ms, setup 5590 ms | | 325 kB; keys 202 kB | same |
| Crescent, measured by Vega | | same, 16 threads | setup 106 s, precompute 25.7 s, prove 182 ms, verify 89 ms | | 16 kB; proving key 710 MB | same |
| GordieHowe, mDL presentation | not in abstract | four cores, device not in abstract | 64.8 ms | | 276 KiB | [ePrint 2026/2615](https://eprint.iacr.org/2026/2615) |
| OpenAC, 1920 B | | desktop | Prepare 2727 ms, Show 77 ms, verify 74 and 9 ms | | proving keys 420 MB and 3.45 MB | [paper source](https://github.com/privacy-ethereum/zkID/blob/main/paper/experiments.tex) |
| zkLogin, RS256 JWT, Groth16 | 1.1M R1CS | 16 vCPU, 64 GB server | witness 550 ms, proof 2.1 s, total 2.78 s | peak 1.19 GB | Groth16 | [Baldimtsi et al.](https://arxiv.org/abs/2401.11735) |
| Semaphore in Noir, depth 10 | 16,217 gates | MacBook Air M2 | 484 ms Node, 1529 ms browser, verify about 15 ms | not reported | | [Hashcloak report](https://hashcloak.github.io/semaphore-noir-final-report/benchmarks.html) |
| StealthNote, Noir JWT | not stated | browser against Mac M1 Pro native | 37.292 s against 2.02 s | | | Mopro blog, via search summary only |
| zk-promises ShowAuth | 24,252 to 37,356 constraints | desktop, i9-11900KB | 316 to 558 ms; verify 2.81 ms | | 608 B and 568 B | [Shih et al.](https://www.usenix.org/conference/usenixsecurity25/presentation/shih) |
| KYC permissioning, ZoKrates equality | compiled artifact 4785 MB | i7-11800H, 16 GB | witness 19.44 s, prove 26.35 s, setup 27.84 s | proving key 631 MB | | [Piper et al.](https://arxiv.org/abs/2510.05807) |

Further findings:

- Baseline phones have 3.82 GB (Android) and 5.7 GB (iPhone). Both platforms kill apps that exceed per-app limits. Many systems need 2 to 10 GB. Barretenberg uses KZG and is not post-quantum. ProveKit and Ligetron are the only two systems tested that combine zero knowledge, post-quantum soundness and 128-bit security. — [CSP Q1'26 report](https://hackmd.io/@clientsideproving/report-q1)
- Mobile benchmarks in the suite so far cover only Binius and Plonky2. Barretenberg has no phone row. — [csp-benchmarks mobile folder](https://github.com/privacy-ethereum/csp-benchmarks/tree/main/mobile). The README states device-farm runs are future work. — [README](https://github.com/privacy-ethereum/csp-benchmarks)
- WHIR-based proof sizes at k=16 and 80-bit security: WHIR 30,724 B, Spartan-WHIR 13,995 B, Whirlaway 114,463 B, HyperPlonk 144,789 B. Timings exist only as charts. — [WHIR note](https://hackmd.io/@clientsideproving/whir-based)
- ProveKit's docs decline to give a single number and say witness count dominates proving time. They also note that SHA-256 and Keccak black boxes are heavier than native R1CS implementations, and that mobile can use file-backed memory. — [ProveKit docs](https://provekit-docs.pages.dev/reference/performance/). World plans to move from BN254 to Goldilocks and add a Groth16 backend. — [ID Tech](https://idtechwire.com/world-releases-mobile-first-zero-knowledge-toolkit-behind-world-id/)
- Peer-reviewed status: Longfellow (CIC 2026) makes a mobile claim. Vega (S&P 2026) and zk-promises (USENIX Security 2025) are desktop or server. OpenAC, FibRace and ShadowPath are preprints. zk-Bench (2024) is a peer-reviewed benchmarking tool, read only as metadata. — [Crossref record](https://doi.org/10.1007/978-3-031-71070-4_3)

Conflicts between sources:

- Longfellow on phones: "a few hundred milliseconds" in the 2026 abstract, about 800 ms in TS13, 1.17 s in the EPFL review, about 1.2 s in the [earlier ePrint summary](https://eid-privacy.github.io/zkp-vault/Resources/papers/FS24-Longfellow/), and under 700 ms in a [talk summary](https://www.a16zcrypto.com/posts/videos/privacy-preserving-digital-identity-with-abhi-shelat). ECDSA alone: about 20 ms in the 2026 abstract, 60 ms earlier. These likely reflect versions and different statements, but I could not confirm that from the paper body.
- OpenAC iPhone 17: the paper source reports Prepare prove 2987 ms and Show 99 ms. The [wallet README](https://github.com/privacy-ethereum/zkID/blob/main/wallet-unit-poc/README.md) reports 2102 ms and 85 ms. The paper source also pairs iPhone 17 with iOS 18, which looks wrong: ShadowPath reports iPhone 17 Pro on iOS 26.6.
- Crescent precompute: 25.7 s in Vega's table on 16 threads, against 20 s for SD-JWT and 140 s for mDL in the EPFL review on a Xeon W-2133. Also, 15 KB, 16 kB or 40 KB for proof size depending on source and what is counted.
- Barretenberg SHA-256 2048 B RAM: 584 MB in the Q1 report, 416 MB in the May JSON, 396.55 MB on [ethproofs](https://ethproofs.org/csp-benchmarks). Proof size 15.8 KB, 14.7 KB and 14.31 KB. ProveKit proof size: 388 to 614 KB in the Q1 report against about 3 MB in current data. These look like version changes.
- Google OAuth in Noir: zk-pdp's [S14 note](/Users/connerswann/code/zk-pdp/docs/sources/S14-mopro-noir-mobile.md) says about 20 s browser, and the Mopro blog via search says 37.292 s. The native figure, about 2 s, agrees. I could not refetch the Mopro post (404) to settle this.

### Inferences
- Vega's 562 MB online peak and Barretenberg's 416 MB for 2 KB of SHA-256 both sit well under the phone limits, but zk-pdp's 353,518 gate circuit plus credential checks would be several times the ECDSA row. The iPhone browser out-of-memory result on P-256 alone says iPhone browser is the case to test first, and a native wrapper is the fair comparison.
- FibRace's 3 GB floor is for Cairo M, not Noir. It is useful as a distribution shape for real devices, not as a Noir prediction.
- Numbers across systems are not comparable without care: thread counts and statements differ. Only the csp-benchmarks table above is like for like.

### Gaps
- I found no published phone measurement of Noir with UltraHonk on a credential or JWT circuit with memory reported, and no zkPassport timing. This is the main hole, and zk-pdp's phone experiment would fill it.
- Memory for zk-pdp's own proofs is not in the README.
- Gate counts for Longfellow, Crescent, Vega, OpenAC and GordieHowe were not in what I could read.
- ShadowPath's circuit size and proving library were not extracted. Its PLONK figures should not be taken as a statement about UltraHonk.

## Costs of signature and JWT verification inside circuits and zkVMs

### Takeaway
SHA-256 over the credential bytes, not the signature, usually dominates a legacy credential proof. In Noir, a P-256 check plus 2 KB of SHA-256 adds about 226k constraints, and JSON or base64 handling comes on top. A zkVM on an M1 is much slower for the same work.

### Cited Findings
- zkLogin's roughly 1.1M R1CS circuit breaks down as SHA-2 about 750k (66%), RSA big-integer work about 155k (14%), and everything else, chiefly JWT parsing, about 235k (20%). Base64 costs 73 constraints per character (about 110k at the maximum length). A naive slice (input 1600, output 100) costs 160k, done five times, and a chunking trick reduces each to about 33k. The IETF spec recommends RS256 and ES256. The authors found RS256 most widely used and support only that. — [Baldimtsi et al.](https://arxiv.org/abs/2401.11735)
- Anon Aadhaar: RSA about 160k, nullifier-related SHA-256 data about 800k, total about 1.7M. — [MoPerf note](https://hackmd.io/5ItB2D50QcavF18cWIrmfQ)
- Noir: P-256 ECDSA 72,683, SHA-256 of 128 B 36,000, SHA-256 of 2048 B 153,803. — [csp-benchmarks](https://github.com/privacy-ethereum/csp-benchmarks/tree/main/results). Same statements in ProveKit R1CS: 143,282 and 345,399. In Circom: SHA-256 of 2048 B 1,050,176. — same source
- `noir-jwt` supports only RS256 with 2048-bit keys, and offers partial SHA hashing to save constraints. It publishes no gate count. — [noir-jwt README](https://github.com/zkemail/noir-jwt)
- Longfellow's ECDSA circuit runs over the P-256 base field directly, with about 50 times fewer wires to commit than emulating that field. Circom ECDSA was 140,000 ms single-core with a 973 MB parameter file. — [RWC 2025 slides](https://iacr.org/submit/files/slides/2025/rwc/rwc2025/17/17_slides.pdf)
- Vega replaces in-circuit parsing with lookups into credential bytes, and says length-hiding hashing needs care because credential lengths differ. Its P-256 ECDSA gadget has about 10,000 constraints for a pre-hashed message, because it avoids non-native arithmetic, which also removes a class of limb-handling soundness bugs. Compare 72,683 in Noir and 143,282 in ProveKit below. — [Vega](https://www.microsoft.com/en-us/research/wp-content/uploads/2026/05/Vega_Low-Latency-Zero-Knowledge-Proofs-over-Existing-Credentials.pdf)
- In `BoundedVec` terms, a Noir circuit is compiled for its maximum size, so the prover pays for unused capacity. — [Gasser, EPFL](https://eid-privacy.github.io/wp2/2026/05/27/noir-benchmarking.html)
- ZK-TPL records the same trap in policy compilation: hidden strings must be fixed length, since length leaks and changes hash cost. — [More et al.](https://arxiv.org/abs/2212.02454)
- zkVMs on the same M1: RISC Zero ECDSA 87.6 s and 4.97 GB, SHA-256 18.5 s and 1.59 GB. — [csp-benchmarks](https://github.com/privacy-ethereum/csp-benchmarks/tree/main/results)
- On GPU, one benchmark post reports secp256k1 ECDSA at RISC Zero 1.0 s, SP1 12 s, OpenVM 14 s, Jolt 83 s. — [Fenbushi](https://fenbushi.vc/?p=795) (secondary, not comparable hardware)
- SP1 Turbo added secp256r1 and RSA precompiles. — [Succinct](https://blog.succinct.xyz/sp1-turbo/) (vendor). Succinct claims precompiles cut cycles about 5 to 10 times. — [Succinct](https://www.succinct.xyz/blog-articles/succinct-ships-optimized-bn254-bls12-381-precompiles-in-sp1) (vendor)
- Prezta found JWT parsing and signature verification dominate XACML-in-zkVM cost, and says its proofs are succinct, not strictly zero-knowledge. — [zk-pdp S01 note](/Users/connerswann/code/zk-pdp/docs/sources/S01-prezta.md) (abstract only)
- zk-pdp records that RISC Zero precompiles are not constant time and that it avoids them for private data. — [threat model](/Users/connerswann/code/zk-pdp/docs/threat-model.md)

### Inferences
- Rough estimate for one signed credential in zk-pdp's Noir circuit, to be measured and not cited as fact: 72,683 + 153,803 = about 226k constraints for an ES256 check over 2 KB, on top of TinyTodo's 353,518, before base64, JSON and claim extraction. That is about a 64 percent increase. If `num_constraints` matches `bb gates` and cost scales linearly, proving would go from 2.2 to 2.8 s to around 3.6 to 4.6 s on the M1 Max.
- From background knowledge, not checked this session: UltraHonk's cost steps at powers of two in circuit size. 353,518 is under 2^19 and 580k is over it, so the real jump could exceed the linear estimate.
- A Cedar decision can touch several entities. If each is a separately signed credential, the 226k repeats per credential. One signed credential holding many attributes is much cheaper than several small ones.
- The spec's order, "Ed25519 first", measures an algorithm that is neither of the two JWT algorithms the IETF recommends, per zkLogin, nor the P-256 signature TS13 specifies for mdoc. It is a fine first step, but it says little about real credentials. ES256 and RS256 are the numbers a reader will want.
- A Term-IR compiler that gets extraction from lookups, as Vega does, could drop much of the parsing share. The JSON soundness issues in [Celi et al.](https://eprint.iacr.org/2026/227) apply to any shortcut here.

### Gaps
- No public gate count for RS256 in Noir, nor for `noir-jwt` end to end. `nargo info` would settle this in minutes.
- No RISC Zero RSA cycle count found.
- I did not confirm that the suite's `num_constraints` for Barretenberg equals `bb gates`.

## Does any system accept a general policy language instead of hand-written predicates?

### Takeaway
Rarely, and never Cedar, Rego or XACML with hidden attributes. Four systems come close: ZK-TPL compiles a Prolog-like language to circuits, Cinderella compiles certificate templates, ABC4Trust and CARL offer policy languages limited to what the credential's sigma proofs can do, and an OpenAC design note evaluates a boolean expression over claims inside one circuit. Prezta compiles XACML but says it is not zero-knowledge.

### Cited Findings

| Level | What the verifier or author writes | Systems | Source |
|---|---|---|---|
| Token | Nothing about attributes | Privacy Pass | [RFC 9576](https://datatracker.ietf.org/doc/rfc9576/) |
| Selective disclosure | Claim paths and exact values | SD-JWT, DCQL over mdoc and SD-JWT | [RFC 9901](https://datatracker.ietf.org/doc/rfc9901/), [OpenID4VP](https://openid.net/specs/openid-4-verifiable-presentations-1_0.html) |
| Fixed predicates | Parameters of a built-in circuit or protocol | BBS with range add-ons, Longfellow and TS13 profiles, Crescent parameter sets, Self, Privado ID | see entries below |
| Hand-written circuit | Circuit code per statement | zk-creds, zk-promises, Vega, OpenAC Show, Dock Circom predicates, zk-email, GordieHowe | see entries below |
| Interpreter inside one circuit | Boolean expression as public input | OpenAC generalized predicates (design note) | [README](https://github.com/privacy-ethereum/zkID/blob/main/generalized-predicates/README.md) |
| Compile from a policy language | A real policy language | ZK-TPL, Cinderella, CARL and ABC4Trust (not circuit based), Prezta (not zero-knowledge), zk-pdp | see entries below |

- **OpenID4VP 1.0 and DCQL** (final, 9 July 2025): claim paths, exact `values` matching, `claim_sets` and `credential_sets`. In the portion I read there is no predicate or range construct, and `values` is a best-effort privacy hint that verifiers "MUST NOT rely on" for security. — [OpenID4VP](https://openid.net/specs/openid-4-verifiable-presentations-1_0.html). A working group mailing-list post raises the range gap. — [OpenID list](https://lists.openid.net/pipermail/openid-specs-digital-credentials-protocols/Week-of-Mon-20250915/000947.html)
- **Privado ID (formerly Polygon ID)** has a JSON-style "ZK Query Language" on generic Circom circuits, with six circuits and operators 1 to 6 in V2 plus 7 to 11 in V3. — [Privado docs](https://docs.privado.id/docs/verifier/verification-library/zk-query-language), [circuits](https://docs.privado.id/docs/verifier/circuits) (search summary). It is the closest deployed thing to "a language", though the circuits are named atomic queries.
- **Dock** lets developers specify predicates on BBS+ attributes in Circom with LegoGroth16. Those are designated-verifier proofs. — [Dock blog](https://blog.dock.io/circom-language-integration/) (vendor, search summary)
- **ZK-TPL.** TPL is a Horn-clause language with Prolog-like syntax. A new `zkaccept` entry point marks statements over hidden attributes, and a compiler turns policies into ZoKrates programs and then Groth16 circuits. The example policy proves age from a hidden birth year and consistency with `pub_hash`, a public commitment to the credential. Signature checks stay in the ordinary `accept` predicate outside the proof. Setup is per policy, and the authors call this a possible adoption blocker and want a universal CRS. Proof size and verification time do not depend on policy complexity. Setup and proving scale with attributes. — [More et al., arXiv 2212.02454](https://arxiv.org/abs/2212.02454)
- **Cinderella.** Applications verify proofs of "knowledge, validity, and policy compliance" over X.509 chains. It introduces a format for composing X.509 templates into policies and a compiler to C validation code, then proves with Pinocchio. Proofs are 288 B. Applications are TLS client authentication and Helios voting. — [IEEE page](https://ieeexplore.ieee.org/document/7546505) and [talk slides](https://www.ieee-security.org/TC/SP2016/slides/23-3/delignat.pdf); a later note gives up to 9 minutes for complex policies. — [IIW slides on Anastasia](https://www.docswell.com/s/yamdan/ZQXPR3-Anastasia-IIW41) (search summary)
- **CARL and ABC4Trust.** CARL covers credential issuance and presentation and supported only selective attribute disclosure, without predicates, inspection, key binding, pseudonyms or revocation. The ABC4Trust language extends it across the credential lifecycle and abstracts over backends. — [IBM report RZ3818](https://dominoweb.draco.res.ibm.com/reports/rz3818.pdf) (search summary). ZK-TPL's authors note ABC4Trust "supports a limited list of functions for use in predicates on private attributes", and that a policy cannot contain conditions hard to translate into a proof. — [More et al.](https://arxiv.org/abs/2212.02454). An IBM report translates CARL into Identity Mixer proof specifications. — [IBM RZ3749](https://dominoweb.draco.res.ibm.com/reports/rz3749.pdf) (search summary)
- **OpenAC generalized predicates.** Public inputs are a list of predicates `(claim_index, operator, operand)` with `<=`, `>=`, `==`, and a postfix expression with AND, OR, NOT. Claims stay private. Operands can be constants or another claim of the same credential. "The operator set is not fixed", but each operator adds constraints. Extensions for set membership and arithmetic are discussed. The repository describes the folder as a formal model with no circuits in it. — [README](https://github.com/privacy-ethereum/zkID/blob/main/generalized-predicates/README.md), [zkID docs mirror](https://zulu0echo.github.io/zkID/docs/intro) (mirror on a personal account)
- **Piper et al.** use XACML vocabulary for roles (PRP, PDP, PEP, PAP) but encode each condition as a tuple `(key, operator, value)` in ZoKrates, with range, equality, membership and time-dependent types. It is not a compiler from XACML. — [Piper et al.](https://arxiv.org/abs/2510.05807)
- **Prezta.** XACML 3.0 compiled to Rust and run in RISC Zero on the client, for JWT claims. It passes 83% of the conformance suite and takes tens of seconds on a desktop. — [zk-pdp S01 note](/Users/connerswann/code/zk-pdp/docs/sources/S01-prezta.md), [hyper.ai summary](https://hyper.ai/en/papers/2607.11466)
- **GordieHowe** has a language with static bounds checking and a certifying compiler, but the language is for writing circuits over signatures. — [ePrint 2026/2615](https://eprint.iacr.org/2026/2615)
- **zk-email's** regex compiler is a small language for extraction from messages. — [Londhe et al.](https://doi.org/10.56553/popets-2026-0143)
- **False friends, by name only.** "Policy-based Access Tokens" is a new primitive using bilinear pairings for delegated verification of attributes by a service, with token verification around 100 ms. — [Pun et al., ACNS 2026](https://eprint.iacr.org/2026/063). "Policy-based anonymous tokens" bound token sets at issuance. — [Kretzler and Li, ePrint 2025/2263](https://eprint.iacr.org/2025/2263), [IETF draft](https://datatracker.ietf.org/doc/draft-li-oauth-policy-based-anonymous-tokens/) (search summary). Neither is a policy language over attributes in what I could read. zkVC at DAC 2025 is about verifiable computing, not credentials, despite the name. — [Crossref record](https://doi.org/10.1109/dac63849.2025.11132681)
- **Not read but flagged:** a blockchain access control scheme with "hidden attributes and policies". — [ePrint 2026/1003](https://eprint.iacr.org/2026/1003) (title only)
- **Search outcome.** Searches combining Cedar with zero-knowledge, and OPA or Rego with verifiable credentials and zero-knowledge, found none of these combinations. A blog post on compiling agent policy into a circuit exists. — [OpenMatter](https://openmatter.substack.com/p/enforcing-policy-with-a-circuit) (informal, not read)

### Inferences
- ZK-TPL is the direct precedent and should be cited as such. Differences from zk-pdp: in its implementation and example the issuer signature is not verified inside the proof (the concept allows a proof of knowledge of a signature, or presenting the signature), and it reveals a public commitment hash, so showings are linkable by that hash. It needs per-policy Groth16 setup, reports no conformance test against the TPL interpreter, and its example is a single conjunction. Its Figure 3 numbers are chart-only.
- OpenAC's design is a different answer to the same problem. It keeps one circuit, so policy is data and the verifier key never changes, which fits the EU wish for few circuits. The cost is a bounded grammar over one credential. zk-pdp's compiled route handles richer semantics but issues a new verification key per policy set. Its spec's compilation proof exists to close that gap. A paper should present this as the main design trade-off in the field.
- The phrase "general policy language" needs care. TPL and Cedar are authorization languages. GordieHowe, Noir, Circom and ZoKrates are circuit languages. Query languages like Privado's and DCQL sit between.

### Gaps
- I read none of the CARL, ABC4Trust or Cinderella paper bodies, only slides, summaries and later descriptions.
- I could not determine how much of TPL ZK-TPL's compiler covers: recursion, multiple rules, built-ins. The paper's example is simple and it states no coverage figure.
- I did not read 2026/1003, S18 (POLARIS) or S19 (health data). They may belong in this table, and another researcher may cover them.
- Search can miss work phrased differently. "None found" is a search result, not a proof of absence.

## What would zk-pdp's private mode add, or does it add nothing?

### Takeaway
It adds a policy language with checked semantics in front of an old capability, and it adds decisions where other systems only have predicates. It adds no new cryptography, is slower than specialist systems for simple predicates, and as specified lacks unlinkability, device binding, revocation and issuer hiding. The honest claim is "private attributes under a real authorization language", not "a better anonymous credential".

### Cited Findings
- Spec for the mode: "Mode 2 with the attributes hidden. The journal reveals the decision and the issuer, not the attributes." It needs a zero-knowledge proof system and the `reasons` field off. — [spec.md](/Users/connerswann/code/zk-pdp/docs/spec.md)
- The spec's composition idea has two proofs exposing "the same commitment, for example the hash of the attributes". — [spec.md](/Users/connerswann/code/zk-pdp/docs/spec.md)
- The same class of value is a known privacy failure: in mDLs, per-attribute hashes stay constant across presentations, so "the resulting hash becomes a linkable identifier across services". — [Vega](https://www.microsoft.com/en-us/research/wp-content/uploads/2026/05/Vega_Low-Latency-Zero-Knowledge-Proofs-over-Existing-Credentials.pdf)
- OpenAC avoids this with a re-randomized commitment between Prepare and Show. — [paper source](https://github.com/privacy-ethereum/zkID/blob/main/paper/experiments.tex)
- Compiler coverage today: 3 of 7 upstream sample sets, 15 of 23 corpus sets, 36 of 36 requests agree with the oracle. — [README](/Users/connerswann/code/zk-pdp/README.md), commit 7fe7093. The related-work file says the private mode "extends them to arbitrary Cedar policies". — [related-work.md](/Users/connerswann/code/zk-pdp/paper/related-work.md)
- TS13 specifies "without trusted setup". — [TS13](https://github.com/eu-digital-identity-wallet/eudi-doc-standards-and-technical-specifications/blob/main/docs/technical-specifications/ts13-zksnarks.md). Barretenberg uses KZG. — [CSP Q1'26 report](https://hackmd.io/@clientsideproving/report-q1)
- Latency for comparison: TinyTodo 2.2 to 2.8 s without signature checks against 64.8 to 501 ms for full mDL proofs on desktop or server hardware, and proof sizes of 14.7 KB against 16 to 325 kB. — figures in the measurements section above.

### Inferences

What it adds:

1. **A real authorization language as the input.** The policy the enforcer already runs is the statement proved. I found no other system doing this with hidden attributes for Cedar, Rego or XACML. Authors do not touch a circuit, and the compiler handles schema and types.
2. **A decision, not a predicate.** Cedar evaluates a policy set with forbid overriding permit, so the proof covers "this set gives this answer", including deny. Credential systems prove that a holder satisfies a statement, and have no use for proving non-satisfaction. This matters for audit and for the agent setting.
3. **A relational statement.** Entity hierarchy, `in`, set containment and attributes across several entities and resources go beyond OpenAC's expression over one credential's claims. ZK-TPL's language could express relations, but its published compile path shows one credential.
4. **Conformance evidence.** A formally modelled language, a compiler from its symbolic Term IR, and differential tests. ZK-TPL states none. Prezta reports 83% of the XACML suite but is not zero-knowledge. GordieHowe's Lean certificate is the higher bar at the circuit level, and could inspire a stronger claim for zk-pdp.
5. **Existing protocol carriage.** AuthZEN gives a request and response shape that existing clients ignore safely. Credential systems define their own presentation flows.
6. **A two-backend comparison.** Measured zkVM against compiled circuit on the same policies. I found no equivalent for credentials, where zkVMs are rarely used on the client at all.

What it does not add:

1. **Cryptographic capability.** Arbitrary predicates over signed attributes are known in principle since at least 2006 and in practice since 2015 to 2023 (C0C0, Cinderella, zk-creds). A skilled author can hand-write any given Cedar policy as a circuit today.
2. **Anything for simple predicates.** Age over 18, domain equals X, or membership in a list are better served by Longfellow, Vega, OpenAC or GordieHowe, which prove a full mDL presentation in 65 to 500 ms on desktop or server hardware with mature parsing. zk-pdp has no signed-credential measurement yet to compare, but the parts estimate above points to seconds. Its smaller proof is its only clear gain there.
3. **Credential handling.** It inherits JWT and mdoc parsing, length hiding, canonicalization and issuer key management, and all the zkLogin failure classes in Celi et al.

What it lacks, as specified:

1. **Unlinkability.** If the request names a principal, the verifier knows who asked, and the mode hides attributes but not identity. If it does not, then repeated showings must not expose a fixed credential hash, signature or key. The spec's public attribute hash would be such a value unless salted with fresh randomness inside both proofs. Fresh randomness per showing means re-proving the credential each time. I did not find, and did not specifically search for, an UltraHonk equivalent of the re-randomization that Crescent and OpenAC use.
2. **Device binding.** Without it a stolen or shared credential proves fine. Vega, OpenAC and Longfellow prove possession of a device key, and Crescent can (`rs256-db`, `mdl1`). EU text treats device binding as a topic of its own. — [Topic G](https://github.com/eu-digital-identity-wallet/eudi-doc-architecture-and-reference-framework/blob/main/docs/discussion-topics/g-zero-knowledge-proof.md)
3. **Revocation and freshness.** Issuer expiry and status proofs are separate work in this literature (ShadowPath, EUDI comparison at [ePrint 2026/1824](https://eprint.iacr.org/2026/1824)). A verifier nonce must be bound into the journal.
4. **Issuer hiding.** The spec reveals the issuer, which shrinks the anonymity set. That can be a reasonable choice, but related work exists. — [Bosk et al.](https://doi.org/10.56553/popets-2022-0123), [ePrint 2026/555](https://eprint.iacr.org/2026/555)
5. **Decision as oracle.** A verifier who can pick policies learns one bit per query about hidden attributes. Pinning policy hash limits this, and it should be stated in the threat model next to the `reasons` leak (H7).
6. **Setup alignment.** KZG's universal reference string does not fit a "without trusted setup" requirement, if EUDI compatibility is ever a goal. ProveKit is a Noir backend without it, with larger proofs (about 3 MB) and higher memory. Since zk-pdp already emits Noir, trying it would be cheap, and is worth a row in the table.

Suggested wording for the paper: replace "extends them to arbitrary Cedar policies" with the current coverage, and say the mode provides attribute privacy under Cedar, composable with an existing credential scheme for anonymity. These are suggestions for the report writer, not decisions.

### Gaps
- I do not know the intended treatment of the principal in private mode. The spec says "the decision and the issuer" but not whether the request is public. This decides whether "anonymous" can be claimed at all, and is a question for Conner.
- No measured private-mode proof exists yet, so all cost statements above are estimates from parts.
- I found no study of how often real Cedar policies need more than one credential.

## Papers and standards: verified BibTeX and read status

### Takeaway
There are 77 paper entries, 13 standards or specifications, and 5 secondary sources below. Metadata comes from Crossref, publisher or arXiv pages, ePrint listings, or the paper's own title page. Among papers, only ZK-TPL is marked `read-full`. Papers I read in part are marked `read-abstract`, with the sections read noted below.

### Cited Findings
- Metadata source by group: entries with a DOI come directly from the [Crossref API](https://api.crossref.org/). `booktitle` is Crossref's container title, so many say "Lecture Notes in Computer Science" rather than a conference name. I did not assert conference names there. ePrint entries come from the ePrint record. Where the listing I saw cut off an author list I end with `and others` and say so in the note.
- Sections read beyond the abstract, by keyword extraction and targeted reading, not cover to cover:
  - zkLogin: implementation and evaluation.
  - Vega: introduction, evaluation, footnotes.
  - zk-promises: introduction, software, evaluation.
  - FibRace: most of the paper, excluding parts of section 3.4.
  - ShadowPath: mobile evaluation.
  - Piper et al.: design and evaluation.
  - Babel and Sedlmeir: background and related work.
  - OpenAC: its LaTeX `experiments.tex` and `comparison.tex` from the authors' repository.
  - Longfellow: abstract from the CIC page and the RWC 2025 slides.
  - Cinderella: talk slides only.
- Conflict: zk-creds has two DOIs. Crossref gives 10.1109/SP46215.2023.10179430, and the ePrint record gives 10.1109/SP46215.2023.00108. The entry uses Crossref's. Fuchsbauer et al. is dated 2018 by Crossref (online) and 2019 by citing papers (volume 32).

```bibtex
@inproceedings{camenisch2001credentials,
  author    = {Camenisch, Jan and Lysyanskaya, Anna},
  title     = {An Efficient System for Non-transferable Anonymous Credentials with Optional Anonymity Revocation},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2001},
  pages     = {93-118},
  doi       = {10.1007/3-540-44987-6_7},
  url       = {https://doi.org/10.1007/3-540-44987-6_7},
  note      = {status: search-summary-only}
}
@inproceedings{camenisch2004bilinear,
  author    = {Camenisch, Jan and Lysyanskaya, Anna},
  title     = {Signature Schemes and Anonymous Credentials from Bilinear Maps},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2004},
  pages     = {56-72},
  doi       = {10.1007/978-3-540-28628-8_4},
  url       = {https://doi.org/10.1007/978-3-540-28628-8_4},
  note      = {status: search-summary-only}
}
@book{brands2000rethinking,
  author    = {Brands, Stefan},
  title     = {Rethinking Public Key Infrastructures and Digital Certificates},
  publisher = {The MIT Press},
  year      = {2000},
  doi       = {10.7551/mitpress/5931.001.0001},
  url       = {https://doi.org/10.7551/mitpress/5931.001.0001},
  note      = {status: search-summary-only}
}
@inproceedings{boneh2004short,
  author    = {Boneh, Dan and Boyen, Xavier and Shacham, Hovav},
  title     = {Short Group Signatures},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2004},
  pages     = {41-55},
  doi       = {10.1007/978-3-540-28628-8_3},
  url       = {https://doi.org/10.1007/978-3-540-28628-8_3},
  note      = {status: search-summary-only}
}
@inproceedings{au2006ktaa,
  author    = {Au, Man Ho and Susilo, Willy and Mu, Yi},
  title     = {Constant-Size Dynamic k-TAA},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2006},
  pages     = {111-125},
  doi       = {10.1007/11832072_8},
  url       = {https://doi.org/10.1007/11832072_8},
  note      = {status: search-summary-only}
}
@inproceedings{camenisch2016daa,
  author    = {Camenisch, Jan and Drijvers, Manu and Lehmann, Anja},
  title     = {Anonymous Attestation Using the Strong Diffie Hellman Assumption Revisited},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2016},
  pages     = {1-20},
  doi       = {10.1007/978-3-319-45572-3_1},
  url       = {https://doi.org/10.1007/978-3-319-45572-3_1},
  note      = {status: search-summary-only}
}
@inproceedings{tessaro2023bbs,
  author    = {Tessaro, Stefano and Zhu, Chenzhi},
  title     = {Revisiting BBS Signatures},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2023},
  pages     = {691-721},
  doi       = {10.1007/978-3-031-30589-4_24},
  url       = {https://doi.org/10.1007/978-3-031-30589-4_24},
  note      = {status: search-summary-only}
}
@article{davidson2018privacypass,
  author    = {Davidson, Alex and Goldberg, Ian and Sullivan, Nick and Tankersley, George and Valsorda, Filippo},
  title     = {Privacy Pass: Bypassing Internet Challenges Anonymously},
  journal   = {Proceedings on Privacy Enhancing Technologies},
  year      = {2018},
  volume    = {2018},
  number    = {3},
  pages     = {164-180},
  doi       = {10.1515/popets-2018-0026},
  url       = {https://doi.org/10.1515/popets-2018-0026},
  note      = {status: search-summary-only}
}
@inproceedings{sonnino2019coconut,
  author    = {Sonnino, Alberto and Al-Bassam, Mustafa and Bano, Shehar and Meiklejohn, Sarah and Danezis, George},
  title     = {Coconut: Threshold Issuance Selective Disclosure Credentials with Applications to Distributed Ledgers},
  booktitle = {Proceedings 2019 Network and Distributed System Security Symposium},
  year      = {2019},
  doi       = {10.14722/ndss.2019.23272},
  url       = {https://doi.org/10.14722/ndss.2019.23272},
  note      = {status: read-abstract}
}
@inproceedings{pointcheval2016randomizable,
  author    = {Pointcheval, David and Sanders, Olivier},
  title     = {Short Randomizable Signatures},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2016},
  pages     = {111-126},
  doi       = {10.1007/978-3-319-29485-8_7},
  url       = {https://doi.org/10.1007/978-3-319-29485-8_7},
  note      = {status: search-summary-only}
}
@inproceedings{maji2011abs,
  author    = {Maji, Hemanta K. and Prabhakaran, Manoj and Rosulek, Mike},
  title     = {Attribute-Based Signatures},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2011},
  pages     = {376-392},
  doi       = {10.1007/978-3-642-19074-2_24},
  url       = {https://doi.org/10.1007/978-3-642-19074-2_24},
  note      = {status: search-summary-only}
}
@inproceedings{sakai2016abs,
  author    = {Sakai, Yusuke and Attrapadung, Nuttapong and Hanaoka, Goichiro},
  title     = {Attribute-Based Signatures for Circuits from Bilinear Map},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2016},
  pages     = {283-300},
  doi       = {10.1007/978-3-662-49384-7_11},
  url       = {https://doi.org/10.1007/978-3-662-49384-7_11},
  note      = {status: search-summary-only}
}
@inproceedings{bellare2014pbs,
  author    = {Bellare, Mihir and Fuchsbauer, Georg},
  title     = {Policy-Based Signatures},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2014},
  pages     = {520-537},
  doi       = {10.1007/978-3-642-54631-0_30},
  url       = {https://doi.org/10.1007/978-3-642-54631-0_30},
  note      = {status: search-summary-only}
}
@article{fuchsbauer2019spseq,
  author    = {Fuchsbauer, Georg and Hanser, Christian and Slamanig, Daniel},
  title     = {Structure-Preserving Signatures on Equivalence Classes and Constant-Size Anonymous Credentials},
  journal   = {Journal of Cryptology},
  year      = {2018},
  volume    = {32},
  number    = {2},
  pages     = {498-546},
  doi       = {10.1007/s00145-018-9281-4},
  url       = {https://doi.org/10.1007/s00145-018-9281-4},
  note      = {status: search-summary-only}
}
@inproceedings{hanser2014spseq,
  author    = {Hanser, Christian and Slamanig, Daniel},
  title     = {Structure-Preserving Signatures on Equivalence Classes and Their Application to Anonymous Credentials},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2014},
  pages     = {491-511},
  doi       = {10.1007/978-3-662-45611-8_26},
  url       = {https://doi.org/10.1007/978-3-662-45611-8_26},
  note      = {status: search-summary-only}
}
@inproceedings{chase2020signal,
  author    = {Chase, Melissa and Perrin, Trevor and Zaverucha, Greg},
  title     = {The Signal Private Group System and Anonymous Credentials Supporting Efficient Verifiable Encryption},
  booktitle = {Proceedings of the 2020 ACM SIGSAC Conference on Computer and Communications Security},
  year      = {2020},
  pages     = {1445-1459},
  doi       = {10.1145/3372297.3417887},
  url       = {https://doi.org/10.1145/3372297.3417887},
  note      = {status: search-summary-only}
}
@inproceedings{garman2014dac,
  author    = {Garman, Christina and Green, Matthew and Miers, Ian},
  title     = {Decentralized Anonymous Credentials},
  booktitle = {Proceedings 2014 Network and Distributed System Security Symposium},
  year      = {2014},
  doi       = {10.14722/ndss.2014.23253},
  url       = {https://doi.org/10.14722/ndss.2014.23253},
  note      = {status: search-summary-only}
}
@inproceedings{belenkiy2009delegatable,
  author    = {Belenkiy, Mira and Camenisch, Jan and Chase, Melissa and Kohlweiss, Markulf and Lysyanskaya, Anna and Shacham, Hovav},
  title     = {Randomizable Proofs and Delegatable Anonymous Credentials},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2009},
  pages     = {108-125},
  doi       = {10.1007/978-3-642-03356-8_7},
  url       = {https://doi.org/10.1007/978-3-642-03356-8_7},
  note      = {status: search-summary-only}
}
@inproceedings{holt2003hidden,
  author    = {Holt, Jason E. and Bradshaw, Robert W. and Seamons, Kent E. and Orman, Hilarie},
  title     = {Hidden Credentials},
  booktitle = {Proceedings of the 2003 ACM workshop on Privacy in the electronic society},
  year      = {2003},
  pages     = {1-8},
  doi       = {10.1145/1005140.1005142},
  url       = {https://doi.org/10.1145/1005140.1005142},
  note      = {status: search-summary-only}
}
@inproceedings{bradshaw2004hidden,
  author    = {Bradshaw, Robert W. and Holt, Jason E. and Seamons, Kent E.},
  title     = {Concealing complex policies with hidden credentials},
  booktitle = {Proceedings of the 11th ACM conference on Computer and communications security},
  year      = {2004},
  pages     = {146-157},
  doi       = {10.1145/1030083.1030104},
  url       = {https://doi.org/10.1145/1030083.1030104},
  note      = {status: search-summary-only}
}
@article{bosk2022hiddenissuer,
  author    = {Bosk, Daniel and Frey, Davide and Gestin, Mathieu and Piolle, Guillaume},
  title     = {Hidden Issuer Anonymous Credential},
  journal   = {Proceedings on Privacy Enhancing Technologies},
  year      = {2022},
  volume    = {2022},
  number    = {4},
  pages     = {571-607},
  doi       = {10.56553/popets-2022-0123},
  url       = {https://doi.org/10.56553/popets-2022-0123},
  note      = {status: search-summary-only}
}
@inproceedings{camenisch2010carl,
  author    = {Camenisch, Jan and Mödersheim, Sebastian and Neven, Gregory and Preiss, Franz-Stefan and Sommer, Dieter},
  title     = {A card requirements language enabling privacy-preserving access control},
  booktitle = {Proceedings of the 15th ACM symposium on Access control models and technologies},
  year      = {2010},
  pages     = {119-128},
  doi       = {10.1145/1809842.1809863},
  url       = {https://doi.org/10.1145/1809842.1809863},
  note      = {status: search-summary-only}
}
@article{camenisch2014concepts,
  author    = {Camenisch, Jan and Dubovitskaya, Maria and Enderlein, Robert R. and Lehmann, Anja and Neven, Gregory and Paquin, Christian and Preiss, Franz-Stefan},
  title     = {Concepts and languages for privacy-preserving attribute-based authentication},
  journal   = {Journal of Information Security and Applications},
  year      = {2014},
  volume    = {19},
  number    = {1},
  pages     = {25-44},
  doi       = {10.1016/j.jisa.2014.03.004},
  url       = {https://doi.org/10.1016/j.jisa.2014.03.004},
  note      = {status: search-summary-only}
}
@inproceedings{camenisch2016formal,
  author    = {Camenisch, Jan and Krenn, Stephan and Lehmann, Anja and Mikkelsen, Gert Læssøe and Neven, Gregory and Pedersen, Michael Østergaard},
  title     = {Formal Treatment of Privacy-Enhancing Credential Systems},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2016},
  pages     = {3-24},
  doi       = {10.1007/978-3-319-31301-6_1},
  url       = {https://doi.org/10.1007/978-3-319-31301-6_1},
  note      = {status: search-summary-only}
}
@article{flamini2024selective,
  author    = {Flamini, Andrea and Sciarretta, Giada and Scuro, Mario and Sharif, Amir and Tomasi, Alessandro and Ranise, Silvio},
  title     = {On cryptographic mechanisms for the selective disclosure of verifiable credentials},
  journal   = {Journal of Information Security and Applications},
  year      = {2024},
  volume    = {83},
  pages     = {103789},
  doi       = {10.1016/j.jisa.2024.103789},
  url       = {https://doi.org/10.1016/j.jisa.2024.103789},
  note      = {status: read-abstract}
}
@article{podda2025dataminimisation,
  author    = {Podda, Emanuela and Hölzmer, Pol and Amard, Alexandre and Sedlmeir, Johannes and Fridgen, Gilbert},
  title     = {The impact of zero-knowledge proofs on data minimisation compliance of digital identity wallets},
  journal   = {Internet Policy Review},
  year      = {2025},
  volume    = {14},
  number    = {3},
  doi       = {10.14763/2025.3.2019},
  url       = {https://doi.org/10.14763/2025.3.2019},
  note      = {status: search-summary-only}
}
@inproceedings{delignat2016cinderella,
  author    = {Delignat-Lavaud, Antoine and Fournet, Cedric and Kohlweiss, Markulf and Parno, Bryan},
  title     = {Cinderella: Turning Shabby X.509 Certificates into Elegant Anonymous Credentials with the Magic of Verifiable Computation},
  booktitle = {2016 IEEE Symposium on Security and Privacy (SP)},
  year      = {2016},
  pages     = {235-254},
  doi       = {10.1109/sp.2016.22},
  url       = {https://doi.org/10.1109/sp.2016.22},
  note      = {status: search-summary-only}
}
@inproceedings{rosenberg2023zkcreds,
  author    = {Rosenberg, Michael and White, Jacob and Garman, Christina and Miers, Ian},
  title     = {zk-creds: Flexible Anonymous Credentials from zkSNARKs and Existing Identity Infrastructure},
  booktitle = {2023 IEEE Symposium on Security and Privacy (SP)},
  year      = {2023},
  pages     = {790-808},
  doi       = {10.1109/sp46215.2023.10179430},
  url       = {https://doi.org/10.1109/sp46215.2023.10179430},
  note      = {status: read-abstract}
}
@inproceedings{baldimtsi2024zklogin,
  author    = {Baldimtsi, Foteini and Chalkias, Konstantinos Kryptos and Ji, Yan and Lindstrøm, Jonas and Maram, Deepak and Riva, Ben and Roy, Arnab and Sedaghat, Mahdi and Wang, Joy},
  title     = {zkLogin: Privacy-Preserving Blockchain Authentication with Existing Credentials},
  booktitle = {Proceedings of the 2024 on ACM SIGSAC Conference on Computer and Communications Security},
  year      = {2024},
  pages     = {3182-3196},
  doi       = {10.1145/3658644.3690356},
  url       = {https://doi.org/10.1145/3658644.3690356},
  note      = {status: read-abstract}
}
@inproceedings{zhang2020deco,
  author    = {Zhang, Fan and Maram, Deepak and Malvai, Harjasleen and Goldfeder, Steven and Juels, Ari},
  title     = {DECO: Liberating Web Data Using Decentralized Oracles for TLS},
  booktitle = {Proceedings of the 2020 ACM SIGSAC Conference on Computer and Communications Security},
  year      = {2020},
  pages     = {1919-1938},
  doi       = {10.1145/3372297.3417239},
  url       = {https://doi.org/10.1145/3372297.3417239},
  note      = {status: search-summary-only}
}
@inproceedings{maram2021candid,
  author    = {Maram, Deepak and Malvai, Harjasleen and Zhang, Fan and Jean-Louis, Nerla and Frolov, Alexander and Kell, Tyler and Lobban, Tyrone and Moy, Christine and Juels, Ari and Miller, Andrew},
  title     = {CanDID: Can-Do Decentralized Identity with Legacy Compatibility, Sybil-Resistance, and Accountability},
  booktitle = {2021 IEEE Symposium on Security and Privacy (SP)},
  year      = {2021},
  pages     = {1348-1366},
  doi       = {10.1109/sp40001.2021.00038},
  url       = {https://doi.org/10.1109/sp40001.2021.00038},
  note      = {status: search-summary-only}
}
@article{frigo2026ecdsa,
  author    = {Frigo, Matteo and shelat, abhi},
  title     = {Anonymous Credentials from ECDSA},
  journal   = {IACR Communications in Cryptology},
  year      = {2026},
  volume    = {3},
  number    = {1},
  doi       = {10.62056/a3qjmpgxq},
  url       = {https://doi.org/10.62056/a3qjmpgxq},
  note      = {status: read-abstract}
}
@inproceedings{kaviani2026vega,
  author    = {Kaviani, Darya and Setty, Srinath},
  title     = {Vega: Low-Latency Zero-Knowledge Proofs over Existing Credentials},
  booktitle = {2026 IEEE Symposium on Security and Privacy (SP)},
  year      = {2026},
  pages     = {1951-1969},
  doi       = {10.1109/sp63933.2026.00182},
  url       = {https://doi.org/10.1109/sp63933.2026.00182},
  note      = {status: read-abstract}
}
@article{londhe2026regex,
  author    = {Londhe, Shreyas and Gupta, Aayush and Suegami, Sora and Shahi, Yogesh and Fgueiredo, Rute and Hassanizadeh, Parisa and Ebrahimi, Shahriar},
  title     = {Zero-Knowledge Proofs of Generalized Regular Expression Matching for Anonymized Email Verification},
  journal   = {Proceedings on Privacy Enhancing Technologies},
  year      = {2026},
  volume    = {2026},
  number    = {4},
  pages     = {740-771},
  doi       = {10.56553/popets-2026-0143},
  url       = {https://doi.org/10.56553/popets-2026-0143},
  note      = {status: read-abstract}
}
@inproceedings{bormann2026sok,
  author    = {Bormann, Christian and Lehmann, Anja},
  title     = {SoK: Anonymous Credentials for Digital Identity Wallets},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2026},
  pages     = {3-25},
  doi       = {10.1007/978-3-032-19567-8_1},
  url       = {https://doi.org/10.1007/978-3-032-19567-8_1},
  note      = {status: read-abstract}
}
@inproceedings{pun2026pbat,
  author    = {Pun, Kiran and Gardham, Daniel and Frymann, Nick},
  title     = {Policy-Based Access Tokens: Privacy-Preserving Verification for Digital Identity},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2026},
  pages     = {221-246},
  doi       = {10.1007/978-3-032-32575-4_9},
  url       = {https://doi.org/10.1007/978-3-032-32575-4_9},
  note      = {status: read-abstract}
}
@inproceedings{more2022zktpl,
  author    = {More, Stefan and Ramacher, Sebastian and Alber, Lukas and Herzl, Marco},
  title     = {Extending Expressive Access Policies with Privacy Features},
  booktitle = {2022 IEEE International Conference on Trust, Security and Privacy in Computing and Communications (TrustCom)},
  year      = {2022},
  pages     = {574-581},
  doi       = {10.1109/trustcom56396.2022.00084},
  url       = {https://doi.org/10.1109/trustcom56396.2022.00084},
  note      = {status: read-full}
}
@inproceedings{piper2025kyc,
  author    = {Piper, Fabian and Wolf, Karl and Heiss, Jonathan},
  title     = {Privacy-Preserving On-chain Permissioning for KYC-Compliant Decentralized Applications},
  booktitle = {2025 7th Conference on Blockchain Research &amp;amp; Applications for Innovative Networks and Services (BRAINS)},
  year      = {2025},
  pages     = {1-10},
  doi       = {10.1109/brains67003.2025.11302906},
  url       = {https://doi.org/10.1109/brains67003.2025.11302906},
  note      = {status: read-abstract}
}
@inproceedings{ames2017ligero,
  author    = {Ames, Scott and Hazay, Carmit and Ishai, Yuval and Venkitasubramaniam, Muthuramakrishnan},
  title     = {Ligero},
  booktitle = {Proceedings of the 2017 ACM SIGSAC Conference on Computer and Communications Security},
  year      = {2017},
  pages     = {2087-2104},
  doi       = {10.1145/3133956.3134104},
  url       = {https://doi.org/10.1145/3133956.3134104},
  note      = {status: search-summary-only}
}
@inproceedings{wang2024ligetron,
  author    = {Wang, Ruihan and Hazay, Carmit and Venkitasubramaniam, Muthuramakrishnan},
  title     = {Ligetron: Lightweight Scalable End-to-End Zero-Knowledge Proofs Post-Quantum ZK-SNARKs on a Browser},
  booktitle = {2024 IEEE Symposium on Security and Privacy (SP)},
  year      = {2024},
  pages     = {1760-1776},
  doi       = {10.1109/sp54263.2024.00086},
  url       = {https://doi.org/10.1109/sp54263.2024.00086},
  note      = {status: search-summary-only}
}
@inproceedings{setty2020spartan,
  author    = {Setty, Srinath},
  title     = {Spartan: Efficient and General-Purpose zkSNARKs Without Trusted Setup},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2020},
  pages     = {704-737},
  doi       = {10.1007/978-3-030-56877-1_25},
  url       = {https://doi.org/10.1007/978-3-030-56877-1_25},
  note      = {status: search-summary-only}
}
@inproceedings{wahby2018hyrax,
  author    = {Wahby, Riad S. and Tzialla, Ioanna and Shelat, Abhi and Thaler, Justin and Walfish, Michael},
  title     = {Doubly-Efficient zkSNARKs Without Trusted Setup},
  booktitle = {2018 IEEE Symposium on Security and Privacy (SP)},
  year      = {2018},
  pages     = {926-943},
  doi       = {10.1109/sp.2018.00060},
  url       = {https://doi.org/10.1109/sp.2018.00060},
  note      = {status: search-summary-only}
}
@inproceedings{arnon2025whir,
  author    = {Arnon, Gal and Chiesa, Alessandro and Fenzi, Giacomo and Yogev, Eylon},
  title     = {WHIR: Reed–Solomon Proximity Testing with Super-Fast Verification},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2025},
  pages     = {214-243},
  doi       = {10.1007/978-3-031-91134-7_8},
  url       = {https://doi.org/10.1007/978-3-031-91134-7_8},
  note      = {status: search-summary-only}
}
@inproceedings{ernstberger2024zkbench,
  author    = {Ernstberger, Jens and Chaliasos, Stefanos and Kadianakis, George and Steinhorst, Sebastian and Jovanovic, Philipp and Gervais, Arthur and Livshits, Benjamin and Orrù, Michele},
  title     = {zk-Bench: A Toolset for Comparative Evaluation and Performance Benchmarking of SNARKs},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2024},
  pages     = {46-72},
  doi       = {10.1007/978-3-031-71070-4_3},
  url       = {https://doi.org/10.1007/978-3-031-71070-4_3},
  note      = {status: search-summary-only}
}
@inproceedings{camenisch2002idemix,
  author    = {Camenisch, Jan and Van Herreweghen, Els},
  title     = {Design and implementation of the idemix anonymous credential system},
  booktitle = {Proceedings of the 9th ACM conference on Computer and communications security},
  year      = {2002},
  pages     = {21-30},
  doi       = {10.1145/586110.586114},
  url       = {https://doi.org/10.1145/586110.586114},
  note      = {status: search-summary-only}
}
@inproceedings{chairattana2025bbs,
  author    = {Chairattana-Apirom, Rutchathon and Tessaro, Stefano},
  title     = {On the Concrete Security of BBS/BBS+ Signatures},
  booktitle = {Lecture Notes in Computer Science},
  year      = {2025},
  pages     = {402-434},
  doi       = {10.1007/978-981-95-5119-4_13},
  url       = {https://doi.org/10.1007/978-981-95-5119-4_13},
  note      = {status: search-summary-only}
}
@inproceedings{chase2014kvac,
  author    = {Chase, Melissa and Meiklejohn, Sarah and Zaverucha, Greg},
  title     = {Algebraic MACs and Keyed-Verification Anonymous Credentials},
  booktitle = {Proceedings of the 2014 ACM SIGSAC Conference on Computer and Communications Security},
  year      = {2014},
  pages     = {1205-1216},
  doi       = {10.1145/2660267.2660328},
  url       = {https://doi.org/10.1145/2660267.2660328},
  note      = {status: search-summary-only}
}
@misc{eagen2026openac,
  author       = {Eagen, Liam and Ngo, Hy and Rushi, Vikas and Tong, Ying and Tsai, Moven and Xia, Janabel},
  title        = {OpenAC: Open Design for Transparent and Lightweight Anonymous Credentials},
  howpublished = {Cryptology ePrint Archive, Paper 2026/251},
  year         = {2026},
  url          = {https://eprint.iacr.org/2026/251},
  note         = {status: read-abstract, plus experiments.tex and comparison.tex from the authors' repository}
}
@misc{paquin2024crescent,
  author       = {Paquin, Christian and Policharla, Guru-Vamsi and Zaverucha, Greg},
  title        = {Crescent: Stronger Privacy for Existing Credentials},
  howpublished = {Cryptology ePrint Archive, Paper 2024/2013},
  year         = {2024},
  url          = {https://eprint.iacr.org/2024/2013},
  note         = {status: read-abstract, plus repository README}
}
@misc{sharp2026gordiehowe,
  author       = {Sharp, Chad and Grubbs, Paul},
  title        = {Faster and Smaller Anonymous Credentials from Standardized Signatures},
  howpublished = {Cryptology ePrint Archive, Paper 2026/2615},
  year         = {2026},
  url          = {https://eprint.iacr.org/2026/2615},
  note         = {status: read-abstract}
}
@inproceedings{shih2025zkpromises,
  author    = {Shih, Maurice and Rosenberg, Michael and Kailad, Hari and Miers, Ian},
  title     = {zk-promises: Anonymous Moderation, Reputation, and Blocking from Anonymous Credentials with Callbacks},
  booktitle = {34th USENIX Security Symposium},
  year      = {2025},
  url       = {https://www.usenix.org/conference/usenixsecurity25/presentation/shih},
  note      = {ePrint 2024/1260. status: read-abstract, plus introduction, software and evaluation sections}
}
@inproceedings{grubbs2022zkmb,
  author    = {Grubbs, Paul and Arun, Arasu and Zhang, Ye and Bonneau, Joseph and Walfish, Michael},
  title     = {Zero-Knowledge Middleboxes},
  booktitle = {31st USENIX Security Symposium},
  year      = {2022},
  pages     = {4255-4272},
  url       = {https://www.usenix.org/conference/usenixsecurity22/presentation/grubbs},
  note      = {ePrint 2021/1022. status: search-summary-only}
}
@misc{wei2026prezta,
  author       = {Wei, Zhongjing and others},
  title        = {Prezta: Provable Remote Execution of Zero-Trust Authorization using SNARKs},
  howpublished = {arXiv:2607.11466},
  year         = {2026},
  url          = {https://arxiv.org/abs/2607.11466},
  note         = {Author list incomplete, as in zk-pdp's references.bib. USENIX Security 2026 per that file, not re-verified here. status: search-summary-only}
}
@misc{gurlek2026govbind,
  author       = {G{\"u}rlek, Yunus and A{\u{g}}{\i}rta{\c{s}}, Ahmet Ramazan},
  title        = {GovBind: From Authenticated Fetch to Zero-Knowledge Predicate over Government Documents},
  howpublished = {Cryptology ePrint Archive, Paper 2026/2206},
  year         = {2026},
  url          = {https://eprint.iacr.org/2026/2206},
  note         = {status: read-abstract}
}
@misc{godden2025eid,
  author       = {Godden, Tom and De Smet, Ruben and Steenhaut, Kris and Braeken, An},
  title        = {Efficiently parsing existing eID documents for zero-knowledge proofs},
  howpublished = {Cryptology ePrint Archive, Paper 2025/1266},
  year         = {2025},
  url          = {https://eprint.iacr.org/2025/1266},
  note         = {status: read-abstract}
}
@misc{celi2026zklogin,
  author       = {Celi, Sofia and Haddadi, Hamed and Den Hartog, Kyle},
  title        = {Analysis and Vulnerabilities in zkLogin},
  howpublished = {Cryptology ePrint Archive, Paper 2026/227},
  year         = {2026},
  url          = {https://eprint.iacr.org/2026/227},
  note         = {status: read-abstract}
}
@misc{kosba2015coco,
  author       = {Kosba, Ahmed and Zhao, Zhichao and Miller, Andrew and Qian, Yi and Chan, Hubert and Papamanthou, Charalampos and Pass, Rafael and shelat, abhi and Shi, Elaine},
  title        = {C0C0: A Framework for Building Composable Zero-Knowledge Proofs},
  howpublished = {Cryptology ePrint Archive, Paper 2015/1093},
  year         = {2015},
  url          = {https://eprint.iacr.org/2015/1093},
  note         = {The record writes the title with empty-set symbols. status: read-abstract}
}
@misc{desmoulins2025bbssharp,
  author       = {Desmoulins, Nicolas and Dumanois, Antoine and Kane, Seyni and Traor{\'e}, Jacques},
  title        = {Making BBS Anonymous Credentials eIDAS 2.0 Compliant},
  howpublished = {Cryptology ePrint Archive, Paper 2025/619},
  year         = {2025},
  url          = {https://eprint.iacr.org/2025/619},
  note         = {status: read-abstract}
}
@misc{rathee2022zebra,
  author       = {Rathee, Deevashwer and Policharla, Guru Vamsi and Xie, Tiancheng and Cottone, Ryan and Song, Dawn},
  title        = {ZEBRA: SNARK-based Anonymous Credentials for Practical, Private and Accountable On-chain Access Control},
  howpublished = {Cryptology ePrint Archive, Paper 2022/1286},
  year         = {2022},
  url          = {https://eprint.iacr.org/2022/1286},
  note         = {status: read-abstract}
}
@misc{policharla2023pqpp,
  author       = {Policharla, Guru-Vamsi and Westerbaan, Bas and Faz-Hern{\'a}ndez, Armando and Wood, Christopher A},
  title        = {Post-Quantum Privacy Pass via Post-Quantum Anonymous Credentials},
  howpublished = {Cryptology ePrint Archive, Paper 2023/414},
  year         = {2023},
  url          = {https://eprint.iacr.org/2023/414},
  note         = {status: read-abstract}
}
@misc{rometsch2026dac,
  author       = {Rometsch, Leandro and Lehwalder, Philipp-Florens and Faust, Sebastian and Schulte, Stefan},
  title        = {Delegatable Anonymous Credentials from Legacy Credentials using Recursive zk-SNARKs},
  howpublished = {Cryptology ePrint Archive, Paper 2026/1855},
  year         = {2026},
  url          = {https://eprint.iacr.org/2026/1855},
  note         = {status: read-abstract}
}
@misc{malatrait2025fibrace,
  author       = {Malatrait, Simon and Sirac, Alex},
  title        = {FibRace: a large-scale benchmark of client-side proving on mobile devices},
  howpublished = {arXiv:2510.14693},
  year         = {2025},
  url          = {https://arxiv.org/abs/2510.14693},
  note         = {status: read-abstract, plus most of the body}
}
@misc{babel2023dataminimization,
  author       = {Babel, Matthias and Sedlmeir, Johannes},
  title        = {Bringing data minimization to digital wallets at scale with general-purpose zero-knowledge proofs},
  howpublished = {arXiv:2301.00823},
  url          = {https://arxiv.org/abs/2301.00823},
  note         = {Year not asserted; arXiv identifier indicates January 2023. status: read-abstract, plus background and related work}
}
@misc{herbke2026shadowpath,
  author       = {Herbke, Patrick and Rieder, Wolf and Sechting, Christian Ren{\'e} and Yang, Huaning and Lamichhane, Sid and Raschke, Philip and K{\"u}pper, Axel},
  title        = {ShadowPath: Lookup-Private Credential Status Verification over Authenticated State},
  howpublished = {arXiv:2608.19937},
  url          = {https://arxiv.org/abs/2608.19937},
  note         = {Year not asserted; arXiv identifier indicates August 2026. status: read-abstract, plus mobile evaluation}
}
@misc{chairattana2025serveraided,
  author       = {Chairattana-Apirom, Rutchathon and Harding, Franklin and Lysyanskaya, Anna and Tessaro, Stefano},
  title        = {Server-Aided Anonymous Credentials},
  howpublished = {Cryptology ePrint Archive, Paper 2025/513},
  year         = {2025},
  url          = {https://eprint.iacr.org/2025/513},
  note         = {status: search-summary-only}
}
@misc{ling2024abs,
  author       = {Ling, San and Nguyen, Khoa and Phan, Duong Hieu and Tang, Khai Hanh},
  title        = {Fully Dynamic Attribute-Based Signatures for Circuits from Codes},
  howpublished = {Cryptology ePrint Archive, Paper 2024/022},
  year         = {2024},
  url          = {https://eprint.iacr.org/2024/022},
  note         = {status: search-summary-only}
}
@misc{hayashi2024abs,
  author       = {Hayashi, Ryuya and others},
  title        = {Attribute-Based Signatures for Circuits with Optimal Parameter Size from Standard Assumptions},
  howpublished = {Cryptology ePrint Archive, Paper 2024/1129},
  year         = {2024},
  url          = {https://eprint.iacr.org/2024/1129},
  note         = {Author list cut off in the listing I saw. status: search-summary-only}
}
@misc{little2026ticket,
  author       = {Little, Ryan and Roche, Daniel S. and Varia, Mayank},
  title        = {Ticket to Hide: Private, Practical Proofs of Provenance for TLS},
  howpublished = {Cryptology ePrint Archive, Paper 2026/540},
  year         = {2026},
  url          = {https://eprint.iacr.org/2026/540},
  note         = {status: search-summary-only}
}
@misc{bozhko2026reclaim,
  author       = {Bozhko, Andrey and Kutsenok, Kirill},
  title        = {Rethinking Zero-Knowledge TLS: Proof of Protocol Execution and the Reclaim Protocol},
  howpublished = {Cryptology ePrint Archive, Paper 2026/2187},
  year         = {2026},
  url          = {https://eprint.iacr.org/2026/2187},
  note         = {status: search-summary-only}
}
@misc{kaaniche2026issuerhiding,
  author       = {Kaaniche, Nesrine and Kane, Seyni and Laurent, Maryline and others},
  title        = {Improved Issuer Hiding for BBS-based Anonymous Credentials},
  howpublished = {Cryptology ePrint Archive, Paper 2026/555},
  year         = {2026},
  url          = {https://eprint.iacr.org/2026/555},
  note         = {Author list cut off in the listing I saw. status: search-summary-only}
}
@misc{flamini2026revocation,
  author       = {Flamini, Andrea and Lehmann, Anja and Sciarretta, Giada and others},
  title        = {Comparing Privacy-Preserving Revocation for the EUDI Wallet},
  howpublished = {Cryptology ePrint Archive, Paper 2026/1824},
  year         = {2026},
  url          = {https://eprint.iacr.org/2026/1824},
  note         = {Author list cut off in the listing I saw. status: search-summary-only}
}
@misc{celi2026devicebinding,
  author       = {Celi, Sof{\'i}a and Lehmann, Anja and Levin, Shai and others},
  title        = {Device Binding for Anonymous Credentials on Legacy Phones},
  howpublished = {Cryptology ePrint Archive, Paper 2026/965},
  year         = {2026},
  url          = {https://eprint.iacr.org/2026/965},
  note         = {Author list cut off in the listing I saw. status: search-summary-only}
}
@misc{setty2026binaryspartan,
  author       = {Setty, Srinath},
  title        = {BinarySpartan: Spartan over binary fields},
  howpublished = {Cryptology ePrint Archive, Paper 2026/1656},
  year         = {2026},
  url          = {https://eprint.iacr.org/2026/1656},
  note         = {status: search-summary-only}
}
@misc{merrill2026barcode,
  author       = {Merrill, Kelsey and Woo, Anna and others},
  title        = {Zero Knowledge Barcode Decoding with Application to Private Online Attribute Verification},
  howpublished = {Cryptology ePrint Archive, Paper 2026/1567},
  year         = {2026},
  url          = {https://eprint.iacr.org/2026/1567},
  note         = {Author list cut off in the listing I saw. status: search-summary-only}
}
@misc{mishra2026frontdoors,
  author       = {Mishra, Shailesh and Chevalley, Roxanne and others},
  title        = {Frontdoors, Not Backdoors: Accountable Anonymity for National Digital Identity},
  howpublished = {Cryptology ePrint Archive, Paper 2026/389},
  year         = {2026},
  url          = {https://eprint.iacr.org/2026/389},
  note         = {Author list cut off in the listing I saw. status: search-summary-only}
}
@misc{eprint2026hiddenpolicies,
  title        = {A Blockchain-Based Access Control Scheme with Hidden Attributes and Policies Using Commitments and Zero-Knowledge Proofs},
  howpublished = {Cryptology ePrint Archive, Paper 2026/1003},
  year         = {2026},
  url          = {https://eprint.iacr.org/2026/1003},
  note         = {Authors not captured. The listing cut the last word of the title at "Proo". status: search-summary-only}
}
@misc{kretzler2025policytokens,
  author       = {Kretzler and Li},
  howpublished = {Cryptology ePrint Archive, Paper 2025/2263},
  url          = {https://eprint.iacr.org/2025/2263},
  note         = {Surnames only and no title captured; AsiaCCS 2026 per a search summary. Look up before citing. status: search-summary-only}
}

% Standards and specifications
@misc{rfc9576, title = {The Privacy Pass Architecture}, howpublished = {RFC 9576}, url = {https://datatracker.ietf.org/doc/rfc9576/}, note = {Informational. Title and status from datatracker. status: search-summary-only}}
@misc{rfc9577, title = {The Privacy Pass HTTP Authentication Scheme}, howpublished = {RFC 9577}, url = {https://datatracker.ietf.org/doc/rfc9577/}, note = {Proposed Standard. status: search-summary-only}}
@misc{rfc9578, title = {Privacy Pass Issuance Protocols}, howpublished = {RFC 9578}, url = {https://datatracker.ietf.org/doc/rfc9578/}, note = {Proposed Standard. status: search-summary-only}}
@misc{rfc9901, title = {Selective Disclosure for JSON Web Tokens}, howpublished = {RFC 9901}, url = {https://datatracker.ietf.org/doc/rfc9901/}, note = {status: search-summary-only}}
@misc{bbsdraft, title = {The BBS Signature Scheme}, howpublished = {Internet-Draft draft-irtf-cfrg-bbs-signatures-14}, year = {2026}, url = {https://datatracker.ietf.org/doc/draft-irtf-cfrg-bbs-signatures/}, note = {status: search-summary-only}}
@misc{blindbbsdraft, title = {Blind BBS Signatures}, howpublished = {Internet-Draft draft-irtf-cfrg-bbs-blind-signatures-05}, year = {2026}, url = {https://datatracker.ietf.org/doc/draft-irtf-cfrg-bbs-blind-signatures/}, note = {status: search-summary-only}}
@misc{libzkdraft, title = {Longfellow ZK}, howpublished = {Internet-Draft draft-google-cfrg-libzk-03}, year = {2026}, url = {https://datatracker.ietf.org/doc/draft-google-cfrg-libzk/}, note = {status: search-summary-only}}
@misc{sdjwtvcdraft, title = {SD-JWT-based Verifiable Digital Credentials (SD-JWT VC)}, howpublished = {Internet-Draft draft-ietf-oauth-sd-jwt-vc-23}, year = {2026}, url = {https://datatracker.ietf.org/doc/draft-ietf-oauth-sd-jwt-vc/}, note = {status: search-summary-only}}
@misc{policytokensdraft, howpublished = {Internet-Draft draft-li-oauth-policy-based-anonymous-tokens}, url = {https://datatracker.ietf.org/doc/draft-li-oauth-policy-based-anonymous-tokens/}, note = {Title not captured. status: search-summary-only}}
@misc{openid4vp, title = {OpenID for Verifiable Presentations 1.0}, howpublished = {OpenID Foundation, Final}, year = {2025}, url = {https://openid.net/specs/openid-4-verifiable-presentations-1_0.html}, note = {Read through section 6 and part of the rest. status: read-abstract}}
@misc{w3cvcwg, title = {Verifiable Credentials Working Group Charter}, howpublished = {W3C}, year = {2026}, url = {https://www.w3.org/2026/03/vc-wg-charter.html}, note = {status: search-summary-only}}
@misc{arftopicg, title = {G - Zero Knowledge Proof}, howpublished = {EUDI Architecture and Reference Framework, discussion topic}, url = {https://github.com/eu-digital-identity-wallet/eudi-doc-architecture-and-reference-framework/blob/main/docs/discussion-topics/g-zero-knowledge-proof.md}, note = {Sections 3 and 4.3 read. status: read-abstract}}
@misc{ts13, title = {Specification for the implementation of Zero-Knowledge Proofs based on arithmetic circuits in the EUDI Wallet}, howpublished = {EUDI Standards and Technical Specifications, TS13}, url = {https://github.com/eu-digital-identity-wallet/eudi-doc-standards-and-technical-specifications/blob/main/docs/technical-specifications/ts13-zksnarks.md}, note = {Abstract and predicate and performance sections read. status: read-abstract}}

% Secondary sources, labelled as such
@misc{cspq1, title = {Benchmarks Report - Q1'26}, howpublished = {HackMD, client-side proving group}, url = {https://hackmd.io/@clientsideproving/report-q1}, note = {Secondary. No author or date on the page. status: read-abstract}}
@misc{cspbench, title = {Client-Side Proving Benchmarks}, howpublished = {GitHub, privacy-ethereum/csp-benchmarks}, url = {https://github.com/privacy-ethereum/csp-benchmarks}, note = {Secondary. Result JSON read directly. status: read-full}}
@misc{p256bench, title = {p256-proving-benchmark}, howpublished = {GitHub, vladfdp}, url = {https://github.com/vladfdp/p256-proving-benchmark}, note = {Secondary. Undated. status: read-full}}
@misc{semaphorenoir, title = {Benchmarks for Semaphore Noir}, howpublished = {Hashcloak report}, url = {https://hashcloak.github.io/semaphore-noir-final-report/benchmarks.html}, note = {Secondary. Read through a fetch summary, not raw. status: read-abstract}}
@misc{epflshowdown, author = {Humbert, Cl{\'e}ment and Gasser, Linus and Elghareeb, Ahmed}, title = {Crescent Longfellow showdown}, year = {2025}, howpublished = {Sotto E-ID Privacy blog}, url = {https://eid-privacy.github.io/wp0/2025/11/28/crescent-longfellow-showdown.html}, note = {Secondary. Title taken from the URL. Read through a fetch summary, not raw. status: read-abstract}}
```

### Inferences
- Coverage is deliberately broad, with depth concentrated on the systems closest to the question: ZK-TPL, Vega, OpenAC, zkLogin, Longfellow. A report writer citing one of the `search-summary-only` entries for a specific claim should read it first.

### Gaps
- **ePrint access.** After about 18 page fetches, eprint.iacr.org stopped serving my scripted requests, and its PDFs returned HTTP 403 or a bot-check page. One further attempt with different user-agent strings was refused by the permission system as bot-check circumvention, and I stopped there rather than work around it. No proxies or mirrors of ePrint were used. Consequently the bodies of Longfellow, Crescent, OpenAC (PDF), GordieHowe, zk-creds, SoK and Policy-based Access Tokens were not read. Someone with a browser should read at least the Longfellow, GordieHowe and SoK bodies. The SoK may have a predicate table that changes this note's classification.
- ePrint pages for items after the cutoff were seen only as lines in ePrint's search listing, so their abstracts are unread and some author lists are cut.
- The arXiv API returned nothing at the time, so arXiv metadata comes from page 1 of the PDFs, which is why two arXiv entries carry no year field. Coconut's abstract page was readable through WebFetch.
- DBLP presented a bot-check page, so it was not used.
- Not retrieved at all: a U-Prove specification, the Anon Aadhaar and Self documentation pages, TLSNotary's docs, and an arXiv item on TEEs with ZKPs for EUDI (2601.19893), whose title I did not capture.
