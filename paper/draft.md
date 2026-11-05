# Proof-carrying authorization for Cedar: one policy language on a zkVM, on circuits, and on a machine-checked proof system

**Draft of 2026-11-05. Not for circulation.** Every number links to an experiment in `docs/experiments/` as `[E1b]`, and every outside claim to a key in `docs/literature/references.bib` as `[@key]`. A marked `TODO` is work the draft depends on. Reading status matters: most sources were read by an AI assistant in key sections only, and the literature review lists what a person must read before submission.

## Abstract

An authorization decision is usually trusted because of who made it. Proof-carrying authorization has long proposed trusting what was shown instead. We report what that takes today for Cedar, a widely used policy language with a formal model. We run one policy set on three proof backends behind one journal: the published evaluator in a zkVM, validated policies compiled to arithmetic circuits through Cedar's own symbolic compiler, and the same compiled form on a proof system whose security theorems are machine-checked. On one decision on a laptop, all three reading the same two records and compiled once per policy, proving took 13 minutes on the zkVM, 2.1 seconds on the circuit, and 43 milliseconds on the third, with proofs of 2.2 MB, 15 KB and 125 KB. In the zkVM, parsing and not policy evaluation took about 99 percent of cycles. The compiler handles 5 of 7 sample policy sets. Generated circuits agreed with the native evaluator on all 40 test requests, and rejected every attack we tried that would change a decision. An attribute authority's signature over the facts adds little to any backend. We say what is not shown: these are one-at-a-time attacks and not a soundness proof, our translation is unverified, and zero knowledge is unproven for two of the three backends.

`TODO:` rewrite after the E10 verdict.

## 1. Introduction

A resource server that calls a policy decision point trusts its operator, its key and the channel. That is reasonable within one organization. It fails where the decider is not trusted: a client device, an AI agent's runtime, a vendor's service, or a machine in someone else's building. A signature does not help, since it says who answered and not that the answer is right.

The idea of checking a proof in place of trusting a decider is old. Appel and Felten had clients submit proofs of authorization in 1999 [@appel1999pca], Bauer's web system put a small checker at the server [@bauer2001pca], SD3 had an evaluator emit a checked proof [@jim2001sd3], and Evidence-Based Audit logged proofs as receipts [@vaughan2008evidence]. What has changed is that a proof can now be succinct and can hide its premises. Prezta recently proved XACML decisions inside RISC Zero for edge devices [@wei2026prezta], and it is the closest prior work.

What we add, as far as we could find as of November 2026:

1. A compiler from Cedar to arithmetic circuits, through the term language of Cedar's symbolic compiler [E4].
2. One policy language measured on a zkVM evaluator and on compiled proofs, under one journal [E1b, E2, E8].
3. A measurement that parsing, not policy evaluation, dominates a policy engine in a zkVM [E1].
4. A fixed-shape record commitment with ancestors closed in the leaf, which makes a denial as cheap to prove as a permit [E2].
5. An adversarial test suite for generated circuits, with what it found [E9].

Each "first" here means "we found no earlier one". This niche filled monthly through 2026 [@erc8354], and our searches missed work in places. We did not search ACM DL, IEEE Xplore or DBLP through their own interfaces. `TODO:` repeat the searches before submission.

**Where this is not needed.** Inside one organization with a trusted decision point, a signed answer does the same job for far less. Where the verifier may hold the policy and facts, re-deciding wins: Cedar decides in microseconds [@cutler2024cedar] against our milliseconds to verify. In a well-run data center, a trusted execution environment is cheaper and covers all of Cedar [@jin2026guardrail]. Our case holds where the decider controls the hardware, since interposers now extract attestation keys [@chuang2026teefail], or where facts must stay hidden.

## 2. Background

**Cedar.** A policy set maps a request and an entity store to allow or deny: allow when some permit holds and no forbid does. Its Lean model has proofs about that model, and the Rust evaluator is tested differentially against it [@disselkoen2024vgd]. Its parsers are not modeled. SymCert verifies a symbolic compiler over a typed term language, and states correctness for evaluation of a term under an interpretation [@torlak2026symcert]. Reading the statement, the SMT solver is not part of it. That is what lets a circuit use these terms. `TODO:` a person confirms this reading.

**AuthZEN.** The OpenID AuthZEN Authorization API 1.0 [@authzen2026final] carries a subject, action, resource and context, and returns a decision with an optional context object. Its section 5.5 lets an enforcement point reject a permit over context it does not understand, so a proof there needs no specification change, but behavior at an unmodified enforcement point has to be tested.

**Proof systems.** We use RISC Zero's zkVM [@bruestle2023risczero], Noir with UltraHonk, and GordieHowe [@sharp2026gordiehowe].

## 3. Threat model

The decider is untrusted. It may lie about facts, replay a past permit, substitute a policy, or bind a proof to a different request. The verifier pins an evaluator identity, a policy hash, and either an entity root or an authority's key. A proof binds a hash of the request, including a verifier nonce. Details are in `docs/threat-model.md`.

Named and unaddressed: a publisher showing different roots to different verifiers, which key transparency addresses [@melara2015coniks]; a proof over a stale root, which is Zanzibar's "new enemy" problem [@pang2019zanzibar]; a decision acting as a one-bit oracle over private facts; and whether the request itself is private. We have not decided the last, so nothing here is called anonymous.

## 4. One journal, three backends

All three bind the same statement: request hash, policy hash, schema hash, entity commitment or authority key, nonce, and decision.

**The record commitment.** Each entity is a fixed-shape record: its UID hash, a presence byte, eight attribute slots in schema order, and sixteen ancestor slots holding the transitive closure. A Merkle tree over records is the store's commitment. Because closure is in the leaf, membership and non-membership are both one scan of known slots, so a proof of denial needs no sorted tree. The authority that publishes the root computes the closure. Sets and nested records sit behind a hash, opened only if a policy reads them. The limits are real: an entity with more than 16 ancestors or a record with more than 8 attributes cannot be committed.

**Cedar in a zkVM [E1, E1b].** The guest is the published `cedar-policy` crate and a thin shell, with policy as input, so the program's identity does not change when policy does. One dependency, `stacker`, does not build for the guest target and is replaced by a stand-in. Cedar's own source is unchanged. In the fair-comparison form, the guest recomputes the two record leaves from JSON, checks their Merkle paths, and hands Cedar only those two entities.

**Cedar compiled to circuits [E2, E4].** Cedar's symbolic compiler specializes each policy to a principal type, action and resource type, and reduces it to a typed term. For TinyTodo these terms are, one for one, what we had written by hand. We translate terms to Noir, one function per environment. Hashing to the root is shared across environments. Strings are compared by hash, and the request and entity UIDs are hashed in a fixed layout, one length byte and zero-padded bytes, so rebuilding text never arises and one circuit serves every length. Request context becomes a record behind its own public hash, computed by the verifier, since rebuilding arbitrary JSON in a circuit would be large and fragile. Not supported: extension values, integer arithmetic, entity tags, the `like` operator, and an entity absent from the store.

**The machine-checked target [E8, E11].** GordieHowe is a language, a certifying compiler and a proof system whose knowledge soundness and zero knowledge are machine-checked in Lean, conditional on the source program being complete and round-by-round knowledge sound [@sharp2026gordiehowe]. We wrote TinyTodo in it by hand, then ran the compiler's output through an interpreter at run time, so a policy is data and needs no C++ build. With the fixed layout, one system serves every request of a policy, and it compiles in seconds.

## 5. A path to a machine-checked chain

`TODO:` this section depends on E10.

Both Cedar's model and GordieHowe are in Lean, and we found no prior attempt to connect such a policy model to such a proof system. The chain has six links: Cedar's authorizer agrees with term evaluation (proved [@torlak2026symcert]); a satisfying witness of our relation makes the decision equal term evaluation over committed facts (ours, not yet stated); record openings bind facts under collision resistance (ours); the realized program implies the source (per-program kernel check); the argument is sound and zero-knowledge given a sound source (proved [@sharp2026gordiehowe]); and the source is round-by-round knowledge sound (ours, and large: the one worked example is about 47,000 lines of Lean, and we found no Lean specification of the SHA-256 gadget).

Status: TinyTodo exports to 171 Lean modules. Our first export's program was 115 MB of Lean, because a shared expression prints as a tree. Committing intermediate bits cut it to 14 MB and did not slow proving. `TODO:` the kernel check, then the second link for one policy. If these fail, the paper reports per-program certificates and names the gap.

## 6. Input provenance

An attribute authority signs SHA-256 of a tag, the root and an epoch with ECDSA P-256. The root becomes private, and the verifier pins the authority's key and epoch. One signature covers any number of opened entities [E3].

| Backend | Unsigned | Signed | Signature | Authorization |
|---|---|---|---|---|
| Noir, gates | 367,960 | 415,020 | 47,060 | a few hundred |
| RISC Zero, cycles | 5.37 M | 5.56 M | 198 thousand, with a precompile | 69 thousand |
| GordieHowe, proving | 37.4 ms | 72.3 ms | 34.9 ms | not separable |

GordieHowe's proof grows from 116 KB to 273 KB, and verifying from 13 ms to 33 ms, because the signature needs a second domain over the P-256 field. Noir's proving goes from 1.85 s to 2.07 s.

So the signature costs far more than the policy, and little against the rest of each proof. Prezta found RSA and regular expressions took about 92 percent of user cycles on its heaviest policy [@wei2026prezta]. This is integrity and not a complete private mode: as specified it lacks unlinkability, device binding and revocation beyond epochs. Bormann and Lehmann observe that practical credential systems reach efficiency by hard-coding policy, format and signature scheme [@bormann2026sok]. Keeping policy as data is what we offer in response, at a real cost against specialist systems [@sharp2026gordiehowe; @kaviani2026vega].

## 7. Evaluation

All on one Apple M1 Max, 10 cores, 64 GB, macOS 27.2. The corpus is TinyTodo and upstream Cedar use cases, with the native evaluator as the oracle [corpus/README.md].

### 7.1 One decision, TinyTodo

From one timing pass on an idle machine, `results/T-*-2026-11-05.jsonl`, median of 5 runs per request on circuits and GordieHowe, one zkVM proof here and three earlier at 775 to 783 s [E1b].

| | zkVM, two records | Noir, compiled | GordieHowe, compiled |
|---|---|---|---|
| Size | 5.37 M cycles | 431,573 gates | not comparable |
| Proving | 768 s | 2.08 s | 42.8 ms |
| Threads | CPU prover | 10 | 1 |
| Verification | 126 ms | 15 ms | 14.6 ms |
| Proof | 2.24 MB | 14.7 KB | 124.7 KB |
| Peak memory | 12.4 GB | 0.99 GB | not measured per proof |
| Per policy | nothing, policy is input | compile and 1.0 s for the key | 3.9 s to compile |
| Cedar coverage | all | 5 of 7 sets | 5 of 7 sets |

We print no ratios, since differences remain. Times include process start for Noir and exclude it for GordieHowe, whose system is already loaded in memory. The zkVM handles any string length and, in this form, denies a request outside the schema where the other two give no proof. The other two refuse strings over their capacities. The zkVM ran on CPU because RISC Zero's current release does not use the GPU on Apple silicon for its main circuit, and a GPU or a server would change this column. Its receipt is composite, which leaks execution length. Succinct and Groth16 receipts are not measured. GordieHowe's build used clang 20 where upstream pins 21, with local patches, and its test switches compiled in and off at run time.

Hand-written TinyTodo costs 1.85 s and 367,960 gates on Noir, and 37.4 ms and 115.8 KB on GordieHowe. So our compiler's overhead is about 17 percent in gates on Noir and about 14 percent in proving time on GordieHowe.

### 7.2 Parsing dominates the zkVM

Over the whole 43-entity store, a decision took 23.3 million cycles: 50 percent parsing the schema, 40 percent parsing entities, 9 percent parsing the policy set and 0.3 percent authorizing [E1]. Entities cost about 215 thousand cycles each. With two records it took 5.38 million, of which Cedar building its entities was 3.08 million and parsing the policy set 2.00 million [E1b]. The one outside figure we found is a vendor blog saying witness deserialization was half of proving cost at one point in a client guest's development (Category Labs, a secondary source). We found no zkVM-versus-circuit comparison on one policy language.

### 7.3 Compiler coverage

Of 7 upstream policy sets, 5 compile. All 40 of their requests match the oracle on Noir and on GordieHowe [E4, E11]. Blocked: `streaming_service` by datetime and arithmetic, `tags_n_roles` by entity tags. Generated Noir circuits range from 431,573 to 836,248 gates, prove in 2.1 to 3.9 s, and use 1.0 to 2.1 GB. On GordieHowe the same five prove in 43 to 115 ms, verify in 15 to 43 ms, make proofs of 125 to 163 KB, and compile in 3.9 to 6.8 s. One hundred extra policies add about 11 thousand gates: size follows what a decision opens, not how many policies exist. Store size does not matter below the tree's depth limit. In the browser, earlier versions of these circuits proved in 4 to 7 seconds up to 632 thousand gates and took about 45 seconds at 859 thousand [E2]. `TODO:` repeat under the final design.

### 7.4 Adversarial witnesses

Differential tests use honest witnesses and cannot see a missing constraint [@watzinger2026lie]. So we attacked [E9, E11]:

- **Noir, tampered inputs.** 3,880 attacks on the five final circuits, on top of 7,517 on earlier versions. None got the opposite decision accepted.
- **Noir, tampered witness values.** 546 single-value changes on three circuits, proved and verified with Barretenberg. No public input change was accepted. 71 were accepted without changing any public input: 70 fit the inverse hint of an is-zero check whose operand is zero, and one was a SHA output whose tampered value did not reach the proof, shown by byte-identical deterministic proofs in a follow-up.
- **GordieHowe.** 2,190 attacks on the five compiled sets and 274 on the signed hand-written relation, with an honest control through the same path. No attack changing a decision was accepted, and all rejections came from the verifier.

The tests found one real gap: string padding was unconstrained, including in the public nonce, so one request had many encodings. It could not change a decision. The fixed layout of section 4 removes it, since padding is inside the hash.

These are mostly one-at-a-time attacks and not a soundness proof. Noir's own checker is a connectivity heuristic, and its SHA-256 library raises a warning we have not audited. Noir 1.0.0's security file does not recommend production use.

## 8. Privacy

| Backend | Zero-knowledge status |
|---|---|
| RISC Zero, composite and succinct | Unproven. The vendor says no mathematical argument has been written. Haböck and Kindi report gaps they say were patched, with no attack [@habock2024note]. Composite receipts leak execution length |
| RISC Zero, Groth16 wrap | Plausibly restored against the verifier, by reasoning from Groth16. Not stated by the vendor. Its docs say this prover is x86 only, though one benchmark reports it on Apple silicon [@yang2026sok]. Unresolved |
| UltraHonk, zero-knowledge target | Designed to be. We found no published proof, and a critical flaw allowed forged ZK proofs before v5.0.0 |
| GordieHowe | Machine-checked for valid realized programs, under stated assumptions [@sharp2026gordiehowe]. It reports gaps in published Ligero analyses too. For our program the source condition is not shown |

`TODO:` demonstrate that fired policy IDs in the journal reveal attributes (H7). They are off in the private form.

## 9. Related work

**Proof-carrying authorization and tokens.** [@appel1999pca; @bauer2001pca; @jim2001sd3; @vaughan2008evidence]. These prove from premises the verifier sees, and reason from positive evidence. Our additions are succinctness, hidden premises and non-monotonic outcomes over a committed closed world. That last framing is our own argument and needs checking per system.

**ZK policy evaluation.** Prezta [@wei2026prezta] compiles each XACML policy to a zkVM program, and makes no zero-knowledge claim. It proves in tens of seconds with 238 to 250 KB proofs and has no committed fact store. Newton runs Rego in SP1 in a challenge path. A Warrant proposal names Cedar in a zkVM and says it is unbuilt. ERC-8354 [@erc8354] uses Noir and UltraHonk and argues against compiled policy.

**Policy compilers to circuits.** Cinderella [@delignat2016cinderella], ZK-TPL [@more2022zktpl], zkGen [@lauinger2024zkgen] and Circuitree [@circuitree2022] compile smaller policy languages. CirC [@ozdemir2022circ] lowers one IR to both SMT and R1CS, the same shape as our front end. We found none using a verified symbolic compiler's terms, or Cedar.

**Authenticating circuits.** vkproof [@vkproof2026] and Ron and Monperrus [@ron2026zkcompilation] bind a key or a binary to its source. GordieHowe's per-program certificates address faithfulness instead. Our compilation proof is unbuilt and is out of this paper.

**Comparisons.** Sheybani et al. compare frameworks on generic workloads [@sheybani2025frameworks]. Yang et al. benchmark zkVMs without a circuit arm [@yang2026sok].

## 10. Limitations

One machine. One policy language. TinyTodo is small. Our Term-to-circuit translation and the interpreter are unverified, and read terms from a Rust crate that a secondary source says is differentially tested, not proved. The compiler's coverage excludes real features. Phone proving is not measured. Most sources were read by an AI assistant in key sections.

## Disclosure

An AI assistant (Claude, Anthropic) took part in ideation, literature search, code and drafting. `docs/decisions/` records who proposed and accepted each choice, and `docs/notebook/` records the work by day. Authorship and how to describe this are open questions for the human researcher.
