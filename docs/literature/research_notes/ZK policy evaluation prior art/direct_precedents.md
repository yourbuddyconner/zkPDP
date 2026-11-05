# Direct precedents for zk-pdp: has anyone already done this?

Researched 2026-11-04 (GitHub results show dates up to 2026-11-05 UTC). Approach: adversarial. Each section tries to prove the claim is not novel.

Reliability notes for the report writer:

- Papers marked `read-full` were read as extracted text. GitHub projects were read at README and selected-file level. None was built or run.
- Several 2026 GitHub projects look heavily machine-written, with few stars and very broad claims (Atlas PCA, dregg, SBO3L, Warrant). Their dates and public text count as disclosure. Their performance and correctness claims are self-reported.
- The repository `yourbuddyconner/zkPDP` is this project. It was filtered out of every null result below. It is public, and its first commit is dated 2026-11-04.

## For each of the five claims, what is the closest work and is it already done?

### Takeaway

No one found has built any of the five claims exactly, but none is clean, and the idea behind claim 1 was published a month before this project started. The strongest remaining novelty is claim 2 (Cedar compiled to circuits through SymCC's Term IR) and the applied form of claim 4 (a proof that a policy compiled to a verification key). Claims 3 and 5 look open but are technically simple, so a reviewer may call them obvious.

| Claim | Verdict | Closest work | Exact difference |
|---|---|---|---|
| 1. Unmodified Cedar evaluator in RISC Zero | **Partly done** (the architecture with Rego; the Cedar idea published, not built) | Newton Protocol (built, Rego). Warrant proposal (Cedar, unbuilt). Prezta (XACML). | Newton: a Regorus fork in SP1, used in the challenge path. Warrant: names this exact step, states nothing is implemented. Prezta: generated per-policy Rust, not an engine. |
| 2. Cedar to Noir through SymCC Term IR, with differential testing | **Apparently open** as stated. Parts done: policy-to-circuit for smaller languages, SMT-style IR to R1CS. | CirC. ZK-TPL. zk-odrl. Warrant and decern use SymCC for analysis only. | No work found uses SymCC's IR as a circuit front end, or compiles Cedar to any circuit. |
| 3. Proof in the `context` of an AuthZEN 1.0 response | **Apparently open**, low technical barrier | Atlas `pca-authzen`. Bernstein issue 5032. AuthZEN issues 656 and 795. | Atlas: a signed object in the request `context`, a boolean response. The others are signatures or receipts with no proof of evaluation. |
| 4. zkVM proof that policy hash H compiled to verification key K | **Partly done**: compiler-in-zkVM exists, the application to circuit keys does not | Ron and Monperrus, COSMICTURTLE. Trident roadmap (aspiration). | COSMICTURTLE: C to RISC-V, for software provenance. No circuit, no verification key, no policy. |
| 5. Merkle tree of fixed-shape records with ancestor closure, so denials are provable | **Apparently open** as a construction. "Denials are provable" alone is **already done**. | Warrant proposal. ERC-8354. Prezta. | Warrant: Merkle slice completeness, unbuilt, no closure-in-leaf. ERC-8354: a DENY envelope, but its reference circuit proves only ALLOW. Prezta: commits a boolean, has no entity store. |

### Cited Findings

**Claim 1: Cedar in a zkVM**

- Warrant's proposal, merged 2026-10-07, proposes to "Run the Rust `cedar-policy` evaluator (DRT-tested) in the guest beside the Verus evaluator, and compare decisions" and says "Nothing in this document is implemented" — [Warrant proposal](https://github.com/mnemonik-xyz/warrant/blob/main/proposals/verified-intent-chain.md); [PR 6](https://github.com/mnemonik-xyz/warrant/pull/6).
- Warrant's current release runs a Verus-proven evaluator for JSON decision trees in a RISC Zero guest, returning Allow, Ask or Deny — [Warrant proposal](https://github.com/mnemonik-xyz/warrant/blob/main/proposals/verified-intent-chain.md). A GitHub code search of its `policy-execution` module for "cedar" returned 0 hits on 2026-11-04 — [repo](https://github.com/mnemonik-xyz/policy-execution).
- Newton Protocol describes "intent-based transaction validation using Rego policies and zero-knowledge proofs", supporting SP1 and RISC0, where operators evaluate and "Challengers can dispute incorrect evaluations with ZK proofs". The repo was created 2025-07-08 — [newton-contracts README](https://github.com/newt-foundation/newton-contracts).
- The crate `newton-rego-kernel` 0.1.0 (published 2026-05-27) is shared "between the operator's evaluation path ... and the SP1 rego challenge circuit (`circuits/sp1-rego`)". Its `evaluate()` takes `policy: String` and calls `engine.add_policy`, which is policy as data — [crate on docs.rs](https://docs.rs/newton-rego-kernel/latest/newton_rego_kernel/), crate source read from crates.io.
- Newton's engine is a fork of Regorus 0.9.0 targeting an OPA v1.2.0-compatible subset with added crypto builtins — [Newton whitepaper](https://docs.newton.xyz/whitepaper/policy-engine). The circuit repo `newton-prover-avs` is not publicly resolvable, so the guest and any proving times were not verified, and the docs read give no proving numbers.
- Atlas PCA's `pca-policy-bridge` translates a Cedar subset to its own predicate form in TypeScript; arithmetic, set operations and extension functions are not modeled and fail closed — [bridge README](https://github.com/Atlas-Authorization/pca/tree/main/packages/pca-policy-bridge). Separately, `zkvm-pca` is "a faithful port of the PCA Policy-VM release gate ... proven inside a RISC Zero zkVM guest" — [ECOSYSTEM.md](https://github.com/Atlas-Authorization/pca/blob/main/ECOSYSTEM.md). This is the only found route from Cedar text toward a RISC Zero proof, but it goes through a translator and a different evaluator. I did not confirm an end-to-end run.
- ERC-8354 (created 2026-07-16) requires the proven program to be "a policy interpreter, not a compiled policy", and allows a fixed zkVM guest with the ruleset as data. It names no policy language — [ERC-8354](https://github.com/ethereum/ERCs/blob/master/ERCS/erc-8354.md).
- SBO3L lists "RISC Zero alebo SP1 zkVM execution policy evaluation" with Regorus as a backlog item only — [backlog](https://github.com/B2JK-Industry/SBO3L-ethglobal-openagents-2026/blob/main/docs/spec/12_backlog.md).
- `dkgoutham/agent-governance-toolkit` proves properties of an audit log in RISC Zero, not a Cedar evaluation. Its PR notes that the predicate fails "every Rego/Cedar/authority/no-policies-loaded entry today" — [PR 7](https://github.com/dkgoutham/agent-governance-toolkit/pull/7). EctoLedger offers SP1 proofs "over the full event hash chain and policy commitment" — [README](https://github.com/EctoSpace/EctoLedger). Neither runs a policy engine as the proven statement in the sense claimed here.
- Prezta compiles XACML to Rust per policy, so each policy gets its own image ID; its authors chose a zkVM because hand-written circuits for regex, JWT and RSA are "cumbersome, error-prone, and expensive to maintain" — [Prezta](https://arxiv.org/abs/2607.11466).
- zk-pdp's own state limits a priority claim: `cedar-policy` does not build for the guest unchanged, so `stacker` is replaced in the guest through `[patch.crates-io]` while Cedar's source stays untouched — `docs/decisions/0015-stacker-stand-in.md`. No proof has completed: `results/E1-prove-2026-11-04.jsonl` is empty and the notebook says "No proving time, proof size or verification time exists yet" — `docs/notebook/2026-11-04.md`.

**Claim 2: Cedar to circuits through the Term IR**

- CirC compiles {C, ZoKrates} to {SMT, ILP, R1CS, MPC} through one shared IR over booleans, bit-vectors, prime fields and bounded arrays — [CirC repo](https://github.com/circify/circ); [paper](https://cs.stanford.edu/~aozdemir/docs/circ.pdf). This is the closest prior art for "an SMT-style IR that lowers to arithmetic constraints". It starts from programming languages, not policy, and does not use a third party's verified IR.
- Ozdemir et al. apply bounded verification to finite-field blasting, the lowering step to field constraints in a ZK compiler — [ePrint 2023/778](https://eprint.iacr.org/2023/778), CAV 2023 (title and search summary only). This bears directly on the Term-to-Noir step, because that step is the unverified link in zk-pdp's chain.
- ZK-TPL compiles a policy language to circuits: rules in the Prolog-like TPL language become ZoKrates programs, then Groth16. It covers the rules marked as zero-knowledge statements over credentials, not whole-policy evaluation, and it uses a non-mainstream language — [arXiv 2212.02454](https://arxiv.org/abs/2212.02454). This is the oldest policy-language-to-circuit compiler found (TrustCom 2022).
- zk-odrl is a bachelor's thesis prototype with a working ODRL compiler to "slots" and a `policyHash`. Its generic circuit is "specified ... not built yet", and the proof today "comes from a hello-world circuit with one constraint" — [zk-odrl README](https://github.com/dedonnodev/zk-odrl). Created 2026-09-15.
- Piper, Wolf and Heiss use XACML component roles and ZoKrates circuits for ABAC conditions, with equality, range, membership and time-dependent proof types — [arXiv 2510.05807](https://arxiv.org/abs/2510.05807).
- zk-Guard-R specifies a bounded reverse-Polish policy bytecode (PUSH_ATTR, EQ, RANGE, AND, OR, NOT, HALT), but benchmarks a simpler private policy vector circuit: 425,574 constraints at 128 attributes, about 2.8 to 3.1 s to prove in gnark/Groth16 — [zk-Guard-R](https://doi.org/10.3390/s26165045). It cites no XACML, Rego or Cedar work.
- Llambí-Morillas and Fernández-Fernández hand-write one Circom policy predicate, "implemented as arithmetic constraints ... at circuit generation time"; proof 351.9 ± 26.6 ms, verify 309.4 ± 22.6 ms using a WASM prover — [paper](https://doi.org/10.3389/fcomp.2026.1966725).
- zkSAS compiles spectrum allocation constraints to R1CS using Circom. This is a hand-written domain circuit, not a language compiler — [arXiv 2609.28699](https://arxiv.org/abs/2609.28699).
- SymCC is used for analysis elsewhere, always ending at SMT. `decern-proof` calls it "Cedar's Lean-verified symbolic compiler (`cedar-policy-symcc`) → SMT (cvc5)" — [decern-proof](https://github.com/anivar/decern/blob/main/crates/decern-proof/src/lib.rs). No project found lowers its terms to a circuit.
- Warrant's proposal states: "The Rust `cedar-policy-symcc` crate is only DRT-tested", and that SymCC's theorem needs well-typed, schema-valid policies and trusts cvc5 and the emitter — [Warrant proposal](https://github.com/mnemonik-xyz/warrant/blob/main/proposals/verified-intent-chain.md) (secondary, citing cedar-spec). zk-pdp's wording "Lean-verified symbolic compiler" should be checked against this before publication.
- ZKSMT proves SMT unsatisfiability in zero knowledge, and zkPi proves Lean theorems in zero knowledge — [ZKSMT](https://eprint.iacr.org/2023/1762); [zkPi](https://dl.acm.org/doi/10.1145/3658644.3670322). These prove the solver's result, which is a different statement from evaluating a policy, but a reviewer may raise them: "a zero-knowledge proof of a Cedar policy analysis" is a neighboring idea.

**Claim 3: proof in the AuthZEN response**

- AuthZEN 1.0 lets a response carry a `context` object whose "semantics and format ... are an implementation concern", says receivers "MUST ignore unknown fields", and says "The PDP MAY choose to sign its authorization response" — [AuthZEN 1.0, sections 5.5, 10.1.1, 11.6](https://openid.net/specs/authorization-api-1_0.html). So the carrier works by specification. That makes claim 3 easy to do and also easy for a reviewer to call trivial.
- Atlas `pca-authzen` puts PCA behind an AuthZEN PDP: "If the request `context` carries a signed PCActn (key `pca_action`), the PDP verifies it", and it returns "a boolean decision" — [README](https://github.com/Atlas-Authorization/pca/tree/main/packages/pca-authzen). Closest AuthZEN integration found. It carries a signed object in the request, not a proof in the response, and the verifier is still the PDP.
- The AuthZEN group discussed signing in 2023 and 2025 ([issue 55](https://github.com/openid/authzen/issues/55)). [Issue 656](https://github.com/openid/authzen/issues/656), opened 2026-09-17 and still open, asks for a signed-response profile "where a PDP decision needs to be verified beyond the immediate PEP". [Issue 795](https://github.com/openid/authzen/issues/795), opened 2026-11-03, proposes a `receipt` obligation for verifiable execution after a permit. Neither mentions proving the evaluation. The live issue 656 is a good opening for this work.
- Bernstein issue 5032 (2026-09-01) proposes AuthZEN 1.0 with a signed receipt embedding the canonical request and policy digest. Its verification model is re-evaluation offline, with no zero-knowledge proof. It flags that Cedar parsing drops `when{}` in its bridge — [issue](https://github.com/sipyourdrink-ltd/bernstein/issues/5032).
- Cedar RFC 0111 (PR 114, open) proposes `decision_id` and `policy_set_hash` on Cedar's response. A maintainer replied that it "seems peripheral" and that a client can wrap the decision — [PR 114](https://github.com/cedar-policy/rfcs/pull/114).
- siros-verifier-core combines ZK credential verification, AuthZEN trust queries and "decision receipts", but the ZK covers the credential and the AuthZEN decision is taken after it. It is the reverse order from zk-pdp — [README](https://github.com/sirosfoundation/siros-verifier-core).
- EMILIA supports AuthZEN mappings and a separate Bulletproofs range receipt, and states that its operator-signed record "is not a zero-knowledge proof" — [Gate README](https://github.com/emiliaprotocol/emilia-protocol/blob/main/packages/gate/README.md).
- A far older ancestor: the OASIS SAML profile of XACML (2005, revised 2014) carries XACML authorization decisions as SAML statements (`XACMLAuthzDecisionStatement`) — [XACML 2.0 SAML profile](https://docs.oasis-open.org/xacml/2.0/SAML-PROFILE/access_control-xacml-2.0-saml-profile-spec-os.html); [SAML profile v2.0](https://docs.oasis-open.org/xacml/3.0/xacml-profile-saml2.0-v2-spec-en.html). Search-summary only. I did not verify its signing rules.
- Prezta removes the PDP and PEP interface instead of carrying a proof over a PDP API: the device verifies directly — [Prezta](https://arxiv.org/abs/2607.11466).

**Claim 4: compilation proof**

- Ron and Monperrus run a compiler inside a zkVM, "producing both the compiled output and a cryptographic proof", using ChibiCC on RISC Zero: 200 random programs, 31 OpenSSL files, 21 libsodium files. Random programs: zk-compile 57 to 979 s (median 78.9 s), verify about 48 to 69 s, receipt 0.26 to 114 MB — [arXiv 2602.11887](https://arxiv.org/abs/2602.11887). The verifier pins the compiler's image ID, which is the same trust shape as zk-pdp's `docs/protocol.md`. They even use the phrase "a compilation proof" and suggest a certifying compiler "could run inside a zkVM". Differences: C to RISC-V, software provenance goal, no circuit, no verification key, no policy.
- Trident's roadmap says "You don't trust the compiler (verify the proof of compilation)" for a self-hosted compiler on Triton VM. This is a stated goal, not a measured result — [roadmap](https://github.com/cyberia-to/trident/blob/master/reference/roadmap.md).
- Coglio et al. describe Leo as a "verifying compiler" that checks each generated circuit family and emits formal proofs in ACL2. This is per-compile assurance for circuits, closest in intent. The evidence is an ACL2 theorem, not a succinct proof, and the verifier needs to check or trust it — [ePrint 2023/1278](https://eprint.iacr.org/2023/1278). Ozdemir et al. say Leo produces partial ACL2 proofs — [ePrint 2023/778](https://eprint.iacr.org/2023/778).
- Almeida et al. (ESORICS 2010) built a certifying compiler for Σ-protocol ZK proofs, with certificates checked in Isabelle/HOL. This is an old precedent for "the compile step carries its own evidence" — [paper](https://doi.org/10.1007/978-3-642-15497-3_10). Search-summary only.
- Ix compiles Lean 4 programs to SNARKs ("zero-knowledge proof-carrying code"), so type checking of a Lean artifact can be verified against a small certificate. My inference: it could in principle carry a Lean-based compiler's correctness theorem, which is a stronger and different route — [Ix](https://github.com/argumentcomputer/ix).
- Tan, Chaliasos and Donaldson (3 Nov 2026) show 48 of 88 known compiler bugs make an SP1 guest commit a wrong output, and conclude that zkVM projects should "treat their compiler as part of their trusted base" — [arXiv 2611.04701](https://arxiv.org/abs/2611.04701). This supports the motivation for claim 4, and it applies against claim 1 as well.
- Three alternatives remove the need for the compile proof, so reviewers will compare against them: a fixed interpreter (ERC-8354: "a new verifier deployment, on every chain, for every rule change ... is a disqualification"; zk-odrl: "One circuit for every policy"), a signed key (Prezta: the policy authority signs `IMAGE_ID`), and a recursion key allowlist (Ziren docs, search summary only: a fixed recursion verifying-key root) — [ERC-8354](https://github.com/ethereum/ERCs/blob/master/ERCS/erc-8354.md); [zk-odrl](https://github.com/dedonnodev/zk-odrl); [Prezta](https://arxiv.org/abs/2607.11466); [Ziren](https://docs.zkm.io/dev/verifier.html).
- Prezta names the trust gap directly: "we assume semantic equivalence between the regular expression in the original XACML and its compiled DFA. The zkVM only needs to prove correct evaluation over a known DFA, not correctness of the compilation itself." Section 6.1 also suggests that update-checking logic "could itself be encoded into a separate (long-lived) zkSNARK or zkVM program". That is close in spirit, but it covers signatures, expiry and revocation, not compilation — [Prezta](https://arxiv.org/abs/2607.11466).

**Claim 5: entity commitment and provable denials**

- Proving a denial is not new in general. Prezta's guest commits `decision` as a boolean whether true or false — [Prezta, Appendix D](https://arxiv.org/abs/2607.11466). Warrant's guest returns Allow, Ask or Deny — [Warrant proposal](https://github.com/mnemonik-xyz/warrant/blob/main/proposals/verified-intent-chain.md). ERC-8354's envelope carries DENY with three reasons: "a rule fired", "nothing authorized", "could not be evaluated", and calls a DENY verdict "a useful off-chain artifact" — [ERC-8354](https://github.com/ethereum/ERCs/blob/master/ERCS/erc-8354.md).
- Many systems do not prove denials: ERC-8354's reference Noir circuit "only proves ALLOW verdicts" — [main.nr](https://github.com/ethereum/ERCs/blob/master/assets/erc-8354/circuits/src/main.nr). zk-odrl: "If the policy is not satisfied no witness exists, so there is no proof to send" — [how-it-works](https://github.com/dedonnodev/zk-odrl/blob/main/docs/how-it-works.md). agent-governance-toolkit: "A violation panics, so no receipt exists" — [PR 7](https://github.com/dkgoutham/agent-governance-toolkit/pull/7).
- A denial only means something if the prover cannot omit the fact that would change it. The closest found treatment is Warrant's roadmap phase 3: "Anchor the policy-set root and the entity root ... The guest checks Merkle inclusion and slice completeness before it evaluates", with tests where "the host drops, swaps or replays entities". It is unbuilt, and asks whether a root can prove completeness "given Cedar's level-based slicing preconditions" — [Warrant proposal](https://github.com/mnemonik-xyz/warrant/blob/main/proposals/verified-intent-chain.md). This is the nearest idea to claim 5. It does not specify a fixed-shape leaf or closure in the leaf.
- Prezta has no entity store. Attributes come from one signed JWT and from inputs the device must check against other sources — [Prezta, section 4.2](https://arxiv.org/abs/2607.11466). zk-Guard-R uses MiMC-Merkle roots for policy and attributes in Groth16, with a policy-hidden goal — [zk-Guard-R](https://doi.org/10.3390/s26165045). Search summary suggests denial there is failure to prove; not confirmed.
- Atlas evaluates `principal in Group` as `member_of` with "bounded closure" from a supplied entity parent map, which is not a committed tree — [cedar.ts](https://github.com/Atlas-Authorization/pca/blob/main/packages/pca-policy-bridge/src/cedar.ts).
- Precomputed transitive closure is standard in access control. Patents cover cached group closures with incremental updates — [US 10230733](https://patents.google.com/patent/US10230733) (search summary only). The Cedar paper discusses the closure as an input invariant in the Rego comparison — [Cedar extended version](https://arxiv.org/abs/2403.04651). Non-membership over authenticated structures is also old: zero-knowledge sets and accumulators — [Catalano, Fiore, Messina (EUROCRYPT 2008)](https://www.iacr.org/archive/eurocrypt2008/49650430/49650430.pdf); [oZKS](https://github.com/microsoft/oZKS). None of these results connects the two in a circuit for policy evaluation.
- zk-pdp's own record shows the construction's limits: 8 attribute and 8 ancestor slots, an authority that must compute the closure honestly, and "A principal or resource that is not in the store cannot be opened" — `docs/decisions/0018-record-entity-commitment.md`.

### Inferences

- Claim 1 is best framed as "the first measured Cedar evaluator run in a zkVM with policy as data". "First policy engine in a zkVM" would be false given Newton, and "first to think of Cedar in a zkVM" would be false given Warrant. Until a proof completes, zk-pdp has run Cedar in the guest in execute mode, not proved it.
- "Unmodified" needs its footnote about `stacker`. A reviewer who finds the patch and no footnote would discount the claim.
- Claim 2 is the most defensible contribution, because it combines three things no found work combines: a production policy language, its symbolic compiler's IR as the front end, and differential testing against the native engine. The claim's assurance is limited: the Term-to-Noir step is not verified, and the Rust SymCC crate's verification status needs care.
- ERC-8354 argues directly against claim 2's architecture, and independently chose Noir/UltraHonk. It will be a natural comparison in a paper. Claim 4 is the answer to its objection, so 2 and 4 are stronger together than alone.
- Claim 4's real risk is cost, not novelty. A verification key for an UltraHonk circuit requires curve commitments to selector polynomials, and COSMICTURTLE needs over a minute for trivial C programs. Running Noir compilation plus key generation inside RISC Zero may be far heavier. This is my inference, not a measurement. Until built, this is a proposal, and Warrant's and Trident's unbuilt statements have the same standing.
- Claims 3 and 5 are good engineering and worth stating, but as design choices rather than as primary novelty. An unmodified PEP ignores the proof by specification, so backward compatibility delivers no new assurance until something checks the proof.
- Claim 5 should be narrowed to what is actually distinctive: a denial in a per-policy circuit, against a committed store, with positional non-membership, so the prover cannot pick a favorable subset. "Denials are provable" alone is not distinctive.

### Gaps

- Private code, unindexed branches and hackathon entries are invisible to GitHub search. Null results are strong for public default branches only.
- Newton's SP1 circuit source and numbers are not public, so I could not confirm whether the challenge proof has been run in production, or how large it is.
- ZK-EHR (Systems 14(4):385): metadata confirmed through Crossref, but the text was not readable. See the last section.
- I could not check whether Warrant has unpublished Cedar work beyond its public `policy-execution` module.

## Has anyone run Cedar in any zkVM or circuit system?

### Takeaway

No public implementation found on RISC Zero, SP1, Jolt, OpenVM or any circuit system. The only public mention of doing it is Warrant's unbuilt proposal, and the only public code path toward it is Atlas's subset translator.

### Cited Findings

- GitHub code search for `cedar_policy risc0_zkvm` returned 3 hits, all in this project. `cedar_policy sp1_zkvm` and `cedar_policy::Authorizer guest risc0` returned 0 — GitHub code search API, 2026-11-04 (full list in the last section).
- Cedar's organization shows no ZK discussion: `org:cedar-policy zk` and `org:cedar-policy snark` returned 0 issues, `repo:cedar-policy/cedar zkvm` returned 0, and GraphQL discussion search for "zero knowledge" and "zk" returned 0. The single "zero knowledge" issue hit is unrelated ([issue 2324](https://github.com/cedar-policy/cedar/issues/2324)).
- The crates.io reverse dependency list of `cedar-policy` (89 crates) contains no crate named for a zkVM or ZK system — [crates.io API](https://crates.io/api/v1/crates/cedar-policy/reverse_dependencies?per_page=100&page=1). This is name-level only, and it does not show dependencies on the ZK side.
- The closest related items are listed under claim 1 above: [Warrant proposal](https://github.com/mnemonik-xyz/warrant/blob/main/proposals/verified-intent-chain.md) and [Atlas ECOSYSTEM.md](https://github.com/Atlas-Authorization/pca/blob/main/ECOSYSTEM.md).

### Inferences

- This is a strong null for public code. Combined with Warrant's "Nothing in this document is implemented", it suggests zk-pdp is ahead in implementation by weeks, not years. Publishing soon matters.

### Gaps

- Web search cannot prove absence in talks, private blogs or non-English sources. Jolt and OpenVM were only covered by generic terms and `cedar-policy jolt` and `cedar-policy openvm` repository searches.

## Has anyone run OPA/Rego, XACML, Casbin, OpenFGA/Zanzibar, Biscuit Datalog or any policy engine in a ZK system? What exactly does Prezta do?

### Takeaway

Yes for Rego (Newton, on SP1, in a challenge path) and XACML (Prezta, via compile to Rust, on RISC Zero). No real implementation found for Casbin, OpenFGA or Zanzibar, and none for Biscuit Datalog inside a proof. Prezta is the peer-reviewed comparator: it proves integrity, not privacy, and it has no entity store.

### Cited Findings

**Prezta, read in full** ([arXiv v2](https://arxiv.org/html/2607.11466v2); [USENIX page](https://www.usenix.org/conference/usenixsecurity26/presentation/wei-zhongjing), pages 5811 to 5830):

- Setting: operational technology. It removes the application gateway by having the client run the policy in a zkVM and send a proof to the edge device.
- Architecture: Policy Authority compiles XACML to IR to Rust (Jinja templates plus generated expressions) to RISC-V. `IMAGE_ID` serves as the verification key, is signed by the authority, and is stored on the device. The subject proves with JWT, environment attributes and the request. The device verifies the proof and separately checks public inputs against its network stack, clock and request.
- Relation: `P(att, Req, info) = 1` and the JWT signature verifies. Security proof sketch reduces to knowledge soundness and signature unforgeability. Network threats are out of scope.
- Numbers: AWS c7i.4xlarge (16 vCPU, 32 GB), RISC Zero v3.0.3. Proving is about 14 s or 28 s depending on padding (2^17 or 2^18 cycles). 7 s for 88.6% of policies with RSA removed (a projection, as RSA was not integrated outside the zkVM). Proof 238 to 250 KB, verify 14 to 15.5 ms, about 256 bytes if wrapped in Groth16 (not measured as an end-to-end cost). An empty guest costs 3.8 s at 2^15 cycles. Compile 0.24 s to Rust plus 1.73 s to image.
- Ablation on the heaviest policy: unoptimized 10,521,257 user cycles and 1,301.7 s; all optimizations 153,677 cycles and 27.3 s (47.7x). RSA is 45% and regex 47% of user cycles.
- Optimizations: regex precompiled to a DFA embedded in the guest; `serde_json` for JWT with few fields; precompiled big-integer for RSA with a hardcoded key.
- Conformance: 323 of 389 testable XACML 3.0 cases (from 397 mandatory), all 323 pass. Their test maps Permit to true and NotApplicable, Deny and Indeterminate to false, so the check is binary.
- Limits: no XPath, AttributeSelector, policy references, obligations, advice, duration arithmetic, RFC 822 names and others (Appendix C). A new image per policy change. No type validation at IR level. Unlinkability is future work.
- What it hides: the paper says "our architecture only utilizes the soundness and succinctness of SNARK, but not zero-knowledge". Table 1 marks JWT contents private, but the policy is shared with subjects, and the guest commits `inp` and `decision` to the journal. So it has no privacy claim for attributes, and the policy is not hidden.
- Reproducibility: artifact at [Zenodo 10.5281/zenodo.20303462](https://doi.org/10.5281/zenodo.20303462), code at [ZK_Zero_Trust](https://github.com/walotta/ZK_Zero_Trust) and [xacml-to-rust](https://github.com/osaidameer/xacml-to-rust). This answers the open question in `docs/open-questions.md`. The USENIX page shows artifact badge images with filenames for Available, Functional and Reproduced, inferred from filenames only.
- Directly relevant remarks: "to support a new policy language, such as Rego [38] or NGAC [23], it would be necessary to build a new compiler". Cedar and AuthZEN are not mentioned anywhere in the text. "Our compiler" trust is assumed from the authority. Outsourced proving is suggested with zkLogin-style JWT binding. Audit through an append-only log inclusion proof is suggested (section 6.3).
- Citations that bear on this project: Reef regex proofs, zkLogin, Aptos Keyless, zk-creds, Park et al. address abstraction, Moser et al. eIDAS in zkVM, Bonsai Pay, Woo et al. legacy signatures, xJsnark, Ozdemir and Boneh collaborative SNARKs, zkSaaS, Larch, Certificate Transparency, ALFA draft, Caplets, Active Certificates, Cao et al. stateful least privilege.

**Other engines:**

- Rego: Newton, as above — [newton-contracts](https://github.com/newt-foundation/newton-contracts); [crate](https://docs.rs/newton-rego-kernel/latest/newton_rego_kernel/). `microsoft/regorus` shows 0 zkvm issues, and `open-policy-agent/opa` 0 zkvm issues — GitHub issue search.
- Atlas also translates a small Rego subset and OpenFGA models to its predicates, ahead of its RISC Zero guest; userset-valued tuples, Rego iteration and more are dropped — [bridge README](https://github.com/Atlas-Authorization/pca/tree/main/packages/pca-policy-bridge). Treat as self-reported.
- Biscuit: `biscuit-auth` is used natively in dregg's token crate, whose SP1 guest leg is described as using the "macaroon vocabulary", so this is not Biscuit's Datalog proved in a guest, based on a partial read — [dregg token/Cargo.toml](https://github.com/emberian/dregg/blob/main/token/Cargo.toml). The Biscuit roadmap's ZK note is about signature aggregation, and PR 197 is a third-party bridge with no proof of Datalog evaluation seen — [roadmap issue 12](https://github.com/eclipse-biscuit/biscuit/issues/12); [PR 197](https://github.com/eclipse-biscuit/biscuit/pull/197).
- TPL (Prolog-like): ZK-TPL — [arXiv 2212.02454](https://arxiv.org/abs/2212.02454).
- ODRL: zk-odrl — [repo](https://github.com/dedonnodev/zk-odrl).
- ABAC with smart contracts: zk-ABAC with ZoKrates (repo created 2022, "supports a scientific paper currently in peer review") — [zkABAC](https://github.com/0Alic/zkABAC). Di Francesco Maesa et al. — [JNCA 2023](https://doi.org/10.1016/j.jnca.2022.103577), cited by Piper et al.
- POLARIS is not a proof-of-evaluation system. It extends XACML into a "verifiable policy language" with verifiable credentials and describes itself against "ZKP-heavy approaches" — [arXiv 2511.22017](https://arxiv.org/abs/2511.22017).
- Agent predicates: Gopalakrishna proves a 32-bit threshold predicate with Bulletproofs (6.2 ms prove, 1.0 ms verify, 608 bytes), bound to MCP and A2A — [arXiv 2608.30083](https://arxiv.org/abs/2608.30083). It credits the Aegis Protocol with first proposing ZK policy proofs for agents, though evaluated by simulation — same paper.
- Proof of Execution: states that it "does not replace consensus, TEEs, or zkVMs"; no proof system of evaluation — [arXiv 2607.05397](https://arxiv.org/abs/2607.05397).

### Inferences

- Prezta and zk-pdp differ on three axes that matter for a paper: an engine with policy as data versus generated code per policy; a Merkle-committed entity store versus one token; and a standard PDP API versus no API. Prezta's numbers also suggest a useful comparison: its typical policies have under 20K user cycles, while zk-pdp's Cedar guest was 11.7M to 23.3M cycles, dominated by parsing.
- Compiling policy ahead of time (Prezta) and interpreting it (Newton, ERC-8354, zk-odrl) is already a live split. zk-pdp's two-backend comparison is a measurement of that split, which seems not to have been published for one language.

### Gaps

- Casbin: all searches null, or matches on awesome-lists. OpenFGA and Zanzibar: null apart from Atlas's translator.
- I did not read Aegis Protocol, Moser et al., or Di Francesco Maesa et al. in full.

## Has anyone compiled a policy language, or an SMT-style verified IR, to arithmetic circuits?

### Takeaway

Policy language to circuit: yes, for narrow languages (TPL in 2022, ABAC condition shapes, ODRL partly). SMT-style IR to constraints: yes (CirC). Cedar, or any verified policy compiler's IR, to circuits: nothing found.

### Cited Findings

- See claim 2 above: [CirC](https://github.com/circify/circ), [field-blasting verification](https://eprint.iacr.org/2023/778), [ZK-TPL](https://arxiv.org/abs/2212.02454), [Piper et al.](https://arxiv.org/abs/2510.05807), [zk-odrl](https://github.com/dedonnodev/zk-odrl), [zk-Guard-R](https://doi.org/10.3390/s26165045).
- A search summary found "no mature, general-purpose compiler that takes a standard access control language (such as XACML or Rego) directly to a zk-SNARK circuit" — WebSearch, query in the last section. My direct reads agree: Prezta targets RISC-V, not constraints.
- GitHub code search for `cedar-policy-symcc noir` and `cedar-policy-symcc circuit` hit only this project and unrelated `circ` bibliography files.

### Inferences

- CirC is the citation a program committee is most likely to raise. zk-pdp could note that the Term IR plays the same role as CirC's IR, and consider field-blasting verification for the lowering step.

### Gaps

- I did not read CirC or the Ozdemir papers in full, and did not check whether a Cedar-to-CirC route has been discussed.

## Has anyone combined AuthZEN, or any standard PDP API, with a ZK proof or signed verifiable decision?

### Takeaway

Signed or receipted decisions: yes, in proposals and old standards. A ZK proof in a standard PDP API's response: nothing found. The working group has no ZK discussion, but it has a live thread on verification beyond the immediate PEP.

### Cited Findings

- `repo:openid/authzen zero knowledge` and `repo:openid/authzen zk` returned 0 issues — GitHub issue search, 2026-11-04.
- See claim 3 above for AuthZEN sections 5.5, 10.1.1 and 11.6, issues 55, 656 and 795, Atlas `pca-authzen`, Bernstein 5032, Cedar RFC 0111, siros, EMILIA and the XACML SAML profile.
- Logius's draft Authorization Decision Log builds on the AuthZEN model and lists integrity and non-repudiation as out of scope, as described by the author of issue 656 — [issue 656](https://github.com/openid/authzen/issues/656). I did not read the draft itself.

### Inferences

- Issue 656 frames exactly zk-pdp's third-party verification case. A proof-carrying response is a concrete answer to it, which may have more practical value than the claim of novelty.

### Gaps

- I searched GitHub issues, not the OpenID mailing list archive or WG meeting notes in full. A ZK remark could exist there.

## Has anyone proved compiler correctness per-instance inside a zkVM to authenticate a circuit's verification key?

### Takeaway

Not found. The two halves exist separately: compile-inside-a-zkVM (COSMICTURTLE) and per-compile assurance for circuits (Leo with ACL2). Note that this statement proves "the compiler ran on this input and gave this key". It does not prove the compiler is correct, so the term "compiler correctness" overstates it.

### Cited Findings

- See claim 4 above: [arXiv 2602.11887](https://arxiv.org/abs/2602.11887), [ePrint 2023/1278](https://eprint.iacr.org/2023/1278), [Trident roadmap](https://github.com/cyberia-to/trident/blob/master/reference/roadmap.md), [Almeida et al.](https://doi.org/10.1007/978-3-642-15497-3_10), [Ix](https://github.com/argumentcomputer/ix).
- Ron and Monperrus separate the two explicitly: CompCert "does not guarantee provenance", and verifiable compilation "establishes provenance guarantees" — [arXiv 2602.11887](https://arxiv.org/abs/2602.11887).
- zk-pdp's design: the journal states `policy_hash`, `schema_hash`, `compiler_hash`, `vk_hash`, and the verifier pins the compiler image and still allowlists `policy_hash` — `docs/protocol.md`.

### Inferences

- What claim 4 gives is removal of a trusted build machine, not removal of trust in the compiler. A wrong compiler produces a validly proven wrong key. The paper should say "provenance of the verification key", and pair it with differential testing or a verified compiler for correctness.
- In zk-pdp's protocol the verifier still allowlists `policy_hash`, so the saving is key distribution, not policy trust. The README's sentence "policy is data again" should be read that way.

### Gaps

- Other zkVM-based verifiable build projects were seen only as titles: [Kettle](https://arxiv.org/abs/2605.08363) uses TEEs, and VeriSBOM covers SBOMs. No results show Noir or Circom compilers running as guests. The code search query on this was cut off by a GitHub rate limit, and one similar query (`noir compile zkvm "verification key" prove nargo guest`) returned nothing relevant.

## Are there patents on ZK policy decision or ZK authorization?

### Takeaway

Yes, there are adjacent patents, but none found claims a ZK proof of a general policy engine's evaluation or a compile-to-circuit step for policies. The three RISC Zero patents cover the zkVM itself. This is a limited search, not a freedom-to-operate opinion.

### Cited Findings

- [US 12028457 B2](https://patents.google.com/patent/US12028457B2/en), RISC Zero (Bruestle, Retford, Laub), priority 2022-02-18, granted 2024-07-02. Sorted execution trace, adjacent-cycle constraint and permutation check. No policy or authorization use; the compiler described handles instructions, not policies.
- [US 12395340 B2](https://patents.google.com/patent/US12395340B2/en), RISC Zero, granted 2025-08-19. Polynomial constraints per instruction. Claims text not in what I read.
- [US 12519646 B2](https://patents.google.com/patent/US12519646B2/en), RISC Zero, granted 2026-01-06. Same trace and sort family as 12028457; claims text not in what I read.
- [US 12689516 B1](https://patents.google.com/patent/US12689516B1/en), OneSource Solutions International (Arkoff, Jukic), granted 2026-07-21. Closest in shape: a gateway verifier state machine with ZKP-verification and policy-evaluation states, for tokens on AI-generated artifacts. Claim 1 also requires an artifact registry, consent scope, jurisdictional lineage and monetization parameters, and a ledger commit. It names no policy engine, zkVM or compile step in the portion read (about 100,000 of 115,231 characters).
- Seen in search summaries only, not read: [US 12393720](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/12393720) (policy access criteria checked after proof validation), [US 12705654](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/12705654) (delegated access with verifiable credentials), [US 12401630](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/12401630) (zero-trust data sharing), [US 11323243](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/11323243) (hidden endorsement policy in ZK), [WO2019170617A1](https://patents.google.com/patent/WO2019170617A1/en) (ZK proof of entitlement), [US 12101301](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/12101301) (ZK proofs for login), [US 12726355](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/12726355) and [US 12676749](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/12676749) (AI compliance and governance).

### Inferences

- zk-pdp's use of RISC Zero's zkVM falls under RISC Zero's own patents and license, which is a question for its license terms, not a novelty issue.
- The public repo and its date may matter for any patent filing of zk-pdp's own: disclosure can bar protection in many countries. This needs a patent attorney.

### Gaps

- I had no structured patent database. Searches were web queries limited to patent domains, so a full-text search for "policy decision point" with "zero-knowledge", pending applications and non-US families were not done. USPTO PDFs are image-only and could not be read through the tool. Google Patents text cut off before claims in two cases.

## Are there startups or products in this space?

### Takeaway

Crypto-native and AI-agent projects exist. No enterprise authorization vendor was found offering ZK-proved decisions. The commercial neighbors are Newton (Rego with on-chain enforcement) and the agent-authority projects.

### Cited Findings

- Newton Protocol: token (NEWT), SDK, policy packs and docs, with verifiable Rego for on-chain authorization and agent use cases. Its core assurance is BLS-signed operator quorum, with ZK at challenge time — [GitHub org](https://github.com/newt-foundation); [docs](https://docs.newton.xyz/developers/use-cases/agent-security) (page title and summary only).
- Atlas Authorization: "Proof-Carrying Authority", 83 published packages by its own account, "Preview", states its cryptography "has not been independently audited", 0 stars — [repo](https://github.com/Atlas-Authorization/pca).
- Mnemonik's Warrant: agent payments with RISC Zero proofs — [repo](https://github.com/mnemonik-xyz/warrant).
- EMILIA Gate: authority control plane, AuthZEN mapping, separate range proofs — [README](https://github.com/emiliaprotocol/emilia-protocol/blob/main/packages/gate/README.md).
- The author of AuthZEN issue 656 discloses building "an authorization gateway that retains signed decision records", unnamed — [issue 656](https://github.com/openid/authzen/issues/656).
- A search naming Cerbos, Oso, Permit.io and AWS Verified Permissions returned no ZK features for any of them, and no vendor pages at all. A funding-oriented search returned identity and infrastructure companies (zCloak, RISC Zero, Self, Rarimo), none in access-control policy, and some unreliable promotional token pages — WebSearch, queries in the last section.

### Inferences

- The commercial space is empty at the enterprise end, or vendors do not advertise it. Newton's token design and on-chain focus make it a technical precedent more than a competitor for AuthZEN deployments.

### Gaps

- I did not check each vendor's docs directly, so "no ZK feature" is an absence in search results, not a confirmed absence. Predicate, Zama and zkPass were named in a query and did not appear; they were not examined.

## What is the evidence record: searches run, null results and citations?

### Takeaway

Null results are recorded below because they are the evidence for novelty. They cover public GitHub default branches and web search as of 2026-11-04.

### Cited Findings

**GitHub repository search, name and description scope, null results (0 hits, excluding this project):** `cedar zkvm`, `cedar-policy risc0`, `cedar-policy risc zero`, `cedar sp1 zkvm`, `cedar policy zero knowledge`, `cedar noir circuit`, `cedar-policy jolt`, `cedar-policy openvm`, `cedar zk authorization`, `zk policy decision point`, `zero knowledge policy decision`, `opa zkvm`, `rego zkvm`, `rego zero knowledge`, `xacml zero knowledge`, `casbin zkvm`, `casbin zero knowledge`, `openfga zero knowledge`, `zanzibar zero knowledge`, `biscuit datalog zk`, `datalog zkvm`, `authzen zero knowledge`, `authzen zk`, `policy zkvm authorization`, `proof of compilation zk`, `verifiable compilation zkvm`, `compiler zkvm verification key`, `zk authorization policy compiler`, `zk rbac abac circuit`, `zkrbac`, `zk abac noir`, `zanzibar zk proof`, `verifiable authorization zkvm`, `provable authorization`. Also `zk-fga` (one unrelated hit) and `zkabac` (zkABAC only). The last batch ran close to a GitHub rate limit, but it reported no errors, and the first README-scope reruns below agree.

**GitHub repository search, README scope:** `cedar zkvm`, `cedar risc zero authorization`, `cedar policy zero-knowledge`, `cedar policy zk proof`, `rego zkvm`, `opa zero-knowledge proof policy`, `xacml zk`, `casbin zk proof`, `biscuit datalog zk`, `zanzibar zero-knowledge`, `openfga zk`, `authzen zero-knowledge`, `authzen zk proof`, `policy decision point zero-knowledge`, `policy decision point zkvm`, `policy compiler noir`, `verifiable compilation zkvm`, `proof of compilation circuit`. These returned mostly awesome-lists and unrelated projects. Relevant hits were Atlas, Newton, zk-odrl, zkABAC, siros, nucleus and Ix. Nucleus has a RISC Zero guest described as "research, no proof is verified anywhere". VAK says "Not a proof system" for its ZK feature. `abac zk circuit noir` returned nothing.

**GitHub code search:**

- 0 hits: `cedar-policy risc0 language:Rust`, `cedar_policy sp1_zkvm`, `cedar_policy::Authorizer guest risc0`, `biscuit_auth risc0`, `"zero knowledge" cedar policy "cedar_policy"`, `cedar-policy guest zkvm Cargo.toml`.
- Only this project or unrelated data files (dependency lists, datasets): `cedar_policy risc0_zkvm`, `cedar-policy sp1-zkvm`, `risc0_zkvm cedar`, `"cedar_policy" "risc0"`, `cedar-policy-symcc noir`, `cedar-policy-symcc circuit`, `symcc Term circuit noir`, `zkAuthZ`, `casbin risc0`.
- Yielded leads, all followed above: `regorus risc0`, `regorus zkvm`, `regorus sp1` (Newton, SBO3L), `xacml risc0` (Prezta repos), `zkvm "policy engine" authorization guest` (ERC-8354, ProofOfClaw, EctoLedger, A1, Atlas, WalletWall, SBO3L), `zkvm cedar authorizer proof receipt` (Atlas, arc), `AuthZEN zk proof`, `zkvm authzen`, `risc0 authzen`, `groth16 authzen`. The `zkvm`, `risc0` and `groth16` AuthZEN hits are PostOakLabs metadata files, where AuthZEN is a test fixture name and zkVM fields are generic — [art-651](https://github.com/PostOakLabs/ainumbers/blob/main/chaingraph/okf/computations/art-651-authzen-conformance-fixture.md).
- Rate limited, not completed: `nargo risc0 guest compile circuit`, `circom compiler zkvm guest prove compilation`.

**GitHub issue and discussion search:** Zero or irrelevant: `repo:cedar-policy/cedar zero knowledge`, `zkvm`, `risc` (dependency bumps only); `org:cedar-policy zk`, `snark`, `circuit` (unrelated); `repo:openid/authzen zero knowledge`, `zk`; `repo:open-policy-agent/opa zkvm`, `zero knowledge`; `repo:microsoft/regorus zkvm`, `risc`; `repo:openfga/openfga zero knowledge`. `repo:casbin/casbin` search failed with a validation error, so it is not evidence. Cedar discussions by GraphQL: 0 for "zero knowledge" and "zk". Useful: `repo:cedar-policy/rfcs zero` (RFC 0111), `repo:openid/authzen signed` and `proof` (issues 55, 656, 795), `zkvm "cedar policy"` (Warrant, agent-governance-toolkit).

**crates.io:** reverse dependencies of `cedar-policy` by name (no ZK crate among 89); text queries `zkvm authorization policy`, `risc0 policy`, `cedar zk` (0), `zk policy` (surfaced Newton crates and `stark-pca`).

**Web searches run:** `Cedar policy zkVM RISC Zero zero-knowledge authorization`; `OPA Rego zero knowledge proof policy evaluation zkVM`; `AuthZEN zero-knowledge proof PDP verifiable decision`; `Newton Protocol Rego policy zkVM SP1 ...`; `compile access control policy language to zk-SNARK arithmetic circuit compiler`; `proving compiler correctness zkVM verification key certified circuit compilation proof`; `zero-knowledge access control startup authorization policy proof product`; `patent zero-knowledge proof policy decision point ...`; `patent policy compiled into zero-knowledge circuit ...`; `zk-SNARK proof policy evaluation denial non-membership Merkle ...`; `zero-knowledge relationship-based access control Zanzibar ReBAC transitive closure proof SNARK` (nothing on ZK ReBAC); `verifiable compilation zkVM prove circuit compiled ...`; `Rego OR Datalog policy zkVM proof ...` (nothing on Datalog); `CirC compiler infrastructure ...`; `zero-knowledge proof SMT theorem ZKSMT Datalog ...` (nothing on Datalog); `enterprise authorization vendor zero-knowledge ...`; `zero-knowledge proof authorization company ...`; `SAML 2.0 profile of XACML ...`; `authenticated data structure transitive closure ...`; `translation validation zero-knowledge circuit compiler ...`.

**Sources that could not be read:** the MDPI page for ZK-EHR returned HTTP 403 to the fetch tool, and a second attempt with a browser-like client was blocked by this environment's permission system, so I stopped. The USPTO PDFs have no text layer.

**BibTeX.** Status marker is in `note`. Only verified fields are given. Fields come from the source pages, Crossref, ePrint metadata or the paper's first page.

```bibtex
@inproceedings{wei2026prezta,
  title     = {Prezta: Provable Remote Execution of Zero-Trust Authorization using {SNARKs}},
  author    = {Wei, Zhongjing and Ameer, Osaid Muhammad and Zhang, Yupeng and Borisov, Nikita},
  booktitle = {35th USENIX Security Symposium (USENIX Security 26)},
  pages     = {5811--5830},
  year      = {2026},
  month     = aug,
  publisher = {USENIX Association},
  isbn      = {978-1-939133-58-8},
  eprint    = {2607.11466},
  archivePrefix = {arXiv},
  url       = {https://www.usenix.org/conference/usenixsecurity26/presentation/wei-zhongjing},
  note      = {read-full (arXiv v2). Author order differs between arXiv and USENIX; USENIX order used here. Artifact: doi:10.5281/zenodo.20303462}
}
@article{llambi2026cva,
  title   = {Cryptographically verifiable authorization for autonomous {AI} agents: a falsifiable hypothesis and proof of concept},
  author  = {Llamb{\'\i}-Morillas, Mar and Fern{\'a}ndez-Fern{\'a}ndez, Daniel},
  journal = {Frontiers in Computer Science},
  volume  = {8},
  pages   = {1966725},
  year    = {2026},
  doi     = {10.3389/fcomp.2026.1966725},
  eprint  = {2607.21325},
  archivePrefix = {arXiv},
  note    = {read-abstract plus sections on implementation and measurements}
}
@misc{ron2026zkcompilation,
  title  = {Verifiable Provenance of Software Artifacts with Zero-Knowledge Compilation},
  author = {Ron, Javier and Monperrus, Martin},
  year   = {2026},
  eprint = {2602.11887},
  archivePrefix = {arXiv},
  url    = {https://arxiv.org/abs/2602.11887},
  note   = {read-abstract plus results and related-work sections. Code: github.com/ASSERT-KTH/verifiable-compilation}
}
@misc{tan2026miscompilation,
  title  = {Proving the Wrong Program: Miscompilations as a Threat to {zkVM} Guarantees},
  author = {Tan, Hui Xuan and Chaliasos, Stefanos and Donaldson, Alastair F.},
  year   = {2026},
  eprint = {2611.04701},
  archivePrefix = {arXiv},
  url    = {https://arxiv.org/abs/2611.04701},
  note   = {read-abstract}
}
@inproceedings{more2022zktpl,
  title     = {Extending Expressive Access Policies with Privacy Features},
  author    = {More, Stefan and Ramacher, Sebastian and Alber, Lukas and Herzl, Marco},
  booktitle = {21st IEEE International Conference on Trust, Security and Privacy in Computing and Communications (TrustCom 2022)},
  year      = {2022},
  eprint    = {2212.02454},
  archivePrefix = {arXiv},
  note      = {read-abstract plus design sections. Year of the venue taken from the arXiv comment and the conference name; proceedings DOI not verified}
}
@inproceedings{ozdemir2022circ,
  title     = {{CirC}: Compiler infrastructure for proof systems, software verification, and more},
  author    = {Ozdemir, Alex and Brown, Fraser and Wahby, Riad S.},
  booktitle = {2022 IEEE Symposium on Security and Privacy (SP)},
  year      = {2022},
  doi       = {10.1109/SP46214.2022.9833782},
  note      = {search-summary-only}
}
@misc{ozdemir2023fieldblasting,
  title  = {Bounded Verification for Finite-Field-Blasting (In a Compiler for Zero Knowledge Proofs)},
  author = {Ozdemir, Alex and Wahby, Riad S. and Brown, Fraser and Barrett, Clark},
  howpublished = {Cryptology ePrint Archive, Paper 2023/778. Published elsewhere: CAV 2023},
  year   = {2023},
  url    = {https://eprint.iacr.org/2023/778},
  note   = {search-summary-only. Journal version: Formal Methods in System Design, 2025, doi:10.1007/s10703-025-00476-3}
}
@misc{coglio2023leo,
  title  = {Compositional Formal Verification of Zero-Knowledge Circuits},
  author = {Coglio, Alessandro and McCarthy, Eric and Smith, Eric and Chin, Collin and Gaddamadugu, Pranav and Dellepere, Michel},
  howpublished = {Cryptology ePrint Archive, Paper 2023/1278},
  year   = {2023},
  url    = {https://eprint.iacr.org/2023/1278},
  note   = {search-summary-only}
}
@inproceedings{almeida2010certifying,
  title     = {A Certifying Compiler for Zero-Knowledge Proofs of Knowledge Based on {$\Sigma$}-Protocols},
  author    = {Almeida, Jos{\'e} Bacelar and Bangerter, Endre and Barbosa, Manuel and Krenn, Stephan and Sadeghi, Ahmad-Reza and Schneider, Thomas},
  booktitle = {ESORICS 2010},
  series    = {Lecture Notes in Computer Science},
  year      = {2010},
  doi       = {10.1007/978-3-642-15497-3_10},
  note      = {search-summary-only}
}
@inproceedings{luick2024zksmt,
  title     = {{ZKSMT}: A {VM} for Proving {SMT} Theorems in Zero Knowledge},
  author    = {Luick, Daniel and others},
  booktitle = {USENIX Security Symposium},
  year      = {2024},
  url       = {https://eprint.iacr.org/2023/1762},
  note      = {search-summary-only. Full author list not verified}
}
@inproceedings{laufer2024zkpi,
  title     = {{zkPi}: Proving Lean Theorems in Zero-Knowledge},
  author    = {Laufer, Evan and Ozdemir, Alex and Boneh, Dan},
  booktitle = {Proceedings of the 2024 ACM SIGSAC Conference on Computer and Communications Security},
  year      = {2024},
  doi       = {10.1145/3658644.3670322},
  note      = {search-summary-only}
}
@article{hou2026zkguard,
  title   = {{zk-Guard-R}: Policy-Hidden and Replay-Safe {zk-SNARK} Access Control for {IoT} Sensor Data Stored on {IPFS}},
  author  = {Hou, Huiying and Ma, Yucong and Zhao, Zisu and Gan, Xuerui},
  journal = {Sensors},
  volume  = {26},
  number  = {16},
  pages   = {5045},
  year    = {2026},
  doi     = {10.3390/s26165045},
  note    = {read through a fetch-tool summary of the PMC full text, not read line by line}
}
@article{albalwy2026zkehr,
  title   = {Zero-Knowledge-Based Policy Enforcement for Privacy-Preserving Cross-Institutional Health Data Sharing on Blockchain},
  author  = {Albalwy, Faisal},
  journal = {Systems},
  volume  = {14},
  number  = {4},
  pages   = {385},
  year    = {2026},
  doi     = {10.3390/systems14040385},
  note    = {search-summary-only. Text blocked (HTTP 403); metadata from Crossref}
}
@misc{piper2025onchain,
  title  = {Privacy-Preserving On-chain Permissioning for {KYC}-Compliant Decentralized Applications},
  author = {Piper, Fabian and Wolf, Karl and Heiss, Jonathan},
  year   = {2025},
  eprint = {2510.05807},
  archivePrefix = {arXiv},
  note   = {read-abstract plus scan of design sections}
}
@misc{zhang2025polaris,
  title  = {{POLARIS}: Cross-Domain Access Control via Verifiable Identity and Policy-Based Authorization},
  author = {Zhang, Aiyao and Lee, Xiaodong and Zhuang, Zhixian and Wei, Jiuqi and Fu, Yufan and Peng, Botao},
  year   = {2025},
  eprint = {2511.22017},
  archivePrefix = {arXiv},
  note   = {read-abstract plus scan of comparison sections}
}
@misc{gopalakrishna2026zkgateway,
  title  = {Zero-Knowledge Predicate Proofs Between {AI} Agents: A Measured, Cross-Protocol Gateway and the Source-Integrity Gap},
  author = {Gopalakrishna, Ashok Subbabhatta},
  year   = {2026},
  eprint = {2608.30083},
  archivePrefix = {arXiv},
  note   = {read-abstract plus related-work scan}
}
@inproceedings{purbasha2026zksas,
  title     = {{zkSAS}: Practical Zero-Knowledge Proofs for Verifiable Spectrum Access Management},
  author    = {Purbasha, Nishat F. and Alom, Ifteher and Burger, Eric W. and Hou, Y. Thomas and Lou, Wenjing and Xiao, Yang},
  booktitle = {2026 IEEE DySPAN},
  year      = {2026},
  eprint    = {2609.28699},
  archivePrefix = {arXiv},
  note      = {read-abstract}
}
@misc{rhodes2026poe,
  title  = {Proof of Execution: Runtime Verification for Governed {AI} Agent Actions},
  author = {Rhodes, James and Kang, George},
  year   = {2026},
  eprint = {2607.05397},
  archivePrefix = {arXiv},
  note   = {read-abstract}
}
@article{cutler2024cedar,
  title   = {Cedar: A New Language for Expressive, Fast, Safe, and Analyzable Authorization},
  author  = {Cutler, Joseph W. and Disselkoen, Craig and Eline, Aaron and He, Shaobo and Headley, Kyle and Hicks, Michael and Hietala, Kesha and Ioannidis, Eleftherios and Kastner, John and Mamat, Anwar and McAdams, Darin and McCutchen, Matt and Rungta, Neha and Torlak, Emina and Wells, Andrew M.},
  journal = {Proceedings of the ACM on Programming Languages},
  volume  = {8},
  number  = {OOPSLA1},
  pages   = {670--697},
  year    = {2024},
  doi     = {10.1145/3649835},
  note    = {search-summary-only. Metadata from Crossref}
}
@article{maesa2023ssi,
  title   = {Self sovereign and blockchain based access control: Supporting attributes privacy with zero knowledge},
  author  = {Di Francesco Maesa, Damiano and Lisi, Andrea and Mori, Paolo and Ricci, Laura and Boschi, Gianluca},
  journal = {Journal of Network and Computer Applications},
  year    = {2023},
  doi     = {10.1016/j.jnca.2022.103577},
  note    = {search-summary-only}
}
@misc{erc8354,
  title  = {{ERC-8354}: Confidential Agent Policy Verdicts},
  author = {Fatonie, Muhammad Zidan and Firdani, Faisal and Muhammad, Maulana Asykari},
  howpublished = {Ethereum ERC, Draft, created 2026-07-16},
  year   = {2026},
  url    = {https://github.com/ethereum/ERCs/blob/master/ERCS/erc-8354.md},
  note   = {read in the relevant sections, including the reference Noir circuit header}
}
```

Non-paper sources to cite as software or web references: Newton (`newt-foundation/newton-contracts`, crate `newton-rego-kernel` 0.1.0), Warrant (`mnemonik-xyz/warrant`, `proposals/verified-intent-chain.md`, merged 2026-10-07), Atlas PCA (`Atlas-Authorization/pca`, package version 0.4.3 for the bridge), zk-odrl (`dedonnodev/zk-odrl`), Trident (`cyberia-to/trident`), Ix (`argumentcomputer/ix`), AuthZEN 1.0 (11 January 2026, Final), Bernstein issue 5032, Cedar RFC PR 114.

### Inferences

- The dense 2026 activity (Newton's kernel in May, Prezta and ERC-8354 in July, zk-odrl in September, Warrant and Atlas in October) suggests this niche is filling quickly, and that these null results may have a short shelf life.

### Gaps

- Google Scholar, ACM DL, IEEE Xplore and DBLP were not searched directly with their own query interfaces. A DBLP API call failed to parse, and coverage of those venues came through general web search. A dedicated search there could find workshop papers missed here.
