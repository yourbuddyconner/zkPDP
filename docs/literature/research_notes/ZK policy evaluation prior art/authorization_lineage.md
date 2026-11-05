# Authorization languages, verified policy analysis and proof-carrying authorization: the non-cryptographic lineage of a verifiable decision

Researched 2026-11-04.

Status marks on every BibTeX entry:

- `read-full`: I held the full text and read the sections cited here, through excerpts. None was read line by line.
- `read-abstract`: I read the abstract or landing page, plus any excerpt named in the notes. This includes pages I saw only through a fetch tool's answers about the page, which the entry's comment says.
- `search-summary-only`: Metadata is verified against the source named, but I did not read the work. Claims about it rest on a search summary or on another paper's description.

BibTeX fields come from Crossref, OpenAlex, arXiv, USENIX, RFC Editor, or the authors' own pages. Fields I could not verify are left out. Three DOIs I recalled from memory resolved to unrelated papers when checked (Abadi LICS 2003, Code-Carrying Authorization, Kolovski WWW 2007). Crossref gave the correct DOIs used below, so do not add DOIs to these entries from memory.

## Proof-carrying authorization and logic-based access control: what they proved, who checked, why adoption stalled

### Takeaway
Moving the decision to an untrusted party and having the verifier check a proof is a 1999 idea. zk-pdp cannot claim it as new. What is new is the form of the proof: a succinct cryptographic argument about a program's execution over committed inputs, not a readable derivation in a logic that exposes its premises. I found no retrospective on why these systems stalled. The nearest statement is one sentence in the Macaroons paper.

### Cited Findings

**Appel and Felten, 1999 (the original).**
- The framework is built on higher-order logic. Taos, SPKI, SDSI and X.509 are explained in the logic and implemented in one system. — [Appel and Felten, CCS 1999, abstract](https://doi.org/10.1145/319709.319718)
- The reason proofs travel at all is undecidability: "Because our logic has no decision procedure — although proof checking is simple — users of the framework must submit proofs with their requests." — [Appel and Felten, CCS 1999, abstract](https://doi.org/10.1145/319709.319718)

**Bauer's web system (2001 to 2003).**
- The server does not decide. "PCA solves this problem by making it the client's responsibility to prove that access should be granted. All the server needs to do is verify that the client's proof is valid." — [Bauer thesis](http://users.ece.cmu.edu/~lbauer/papers/thesis.pdf)
- Clients work in a decidable, application-specific subset of the undecidable logic. — [Bauer thesis](http://users.ece.cmu.edu/~lbauer/papers/thesis.pdf)
- The browser generates proofs by iteratively fetching proof components. Twelf is the prover. — [Bauer, Schneider, Felten, TR-638-01](http://users.ece.cmu.edu/~lbauer/papers/pcaprototr.pdf)
- The server checks a proof by type-checking an LF term. "The proof checker is part of the trusted computing base of the system", so it should be small. — [TR-638-01](http://users.ece.cmu.edu/~lbauer/papers/pcaprototr.pdf)
- Replay was already addressed. Clients prove a goal bound to a session identifier, which "prevents clients from using stolen proofs, but it doesn't preclude them from being allowed to reuse an old proof". — [TR-638-01](http://users.ece.cmu.edu/~lbauer/papers/pcaprototr.pdf)
- The report also covers caching and modularizing proofs, and hiding parts of the policy from unauthorized clients. — [TR-638-01](http://users.ece.cmu.edu/~lbauer/papers/pcaprototr.pdf)

**Grey (the deployment).**
- Grey turns a smartphone into the device that exercises and delegates authority, for offices and computer logins. — [Bauer et al., 2005, abstract](https://doi.org/10.1007/11556992_31)
- Credentials are scattered, so proving had to become distributed. The algorithm succeeds whenever a centralized prover with remote certificate retrieval would. — [Bauer, Garriss, Reiter, S&P 2005, abstract](https://doi.org/10.1109/sp.2005.9)
- Field data: 19 participants after nine months, in one Carnegie Mellon office building. Across 335 logged door accesses, an average of 6.6 seconds (SD 1.7) from first request to unlock. — [Bauer et al., SOUPS 2007](https://doi.org/10.1145/1280680.1280689)
- Users complained about speed, and Grey users spent an average 2.9 seconds idly waiting. Yet 18 of 19 were still using it when given the option to stop. — [Bauer et al., SOUPS 2007](https://doi.org/10.1145/1280680.1280689)

**Later PCA systems.**
- PCFS adapts PCA to a file system. It uses logic BL, whose policy consequences change with time and state, and enforcement "combines proof verification with conditional capabilities". The authors prove the capability enforcement correct. — [Garg and Pfenning, S&P 2010, abstract](https://doi.org/10.1109/sp.2010.28)
- NAL lets authorization depend on a requester's pedigree, on the outcome of a mechanized analysis, or on trusted software that encapsulates it. It classifies authorization bases as axiomatic, analytic and synthetic. — [Schneider, Walsh, Sirer, TISSEC 2011](https://www.cs.cornell.edu/fbs/publications/NexusNalRationale.pdf)
- A guard using proof-carrying code authorizes a program if and only if "a proof checker trusted by the guard establishes that the proof is correct". — [NAL](https://www.cs.cornell.edu/fbs/publications/NexusNalRationale.pdf)
- Logical attestation treats attributable, unforgeable statements about program properties, expressed in a logic, as credentials. This is the earliest attestation-plus-logic hybrid I found. — [Sirer et al., SOSP 2011, abstract](https://doi.org/10.1145/2043556.2043580)

**A certified evaluator, in 2001.**
- SD3's evaluator "computes a proof that the answer follows from the security policy. Before the answer is returned, the proof is passed through a simple checker and incorrect proofs are reported as errors. The certified evaluator reduces the trusted computing base." — [Jim, S&P 2001, abstract](https://doi.org/10.1109/secpri.2001.924291)

**Proofs as audit records.**
- "A proof in the logic is evidence that an access-control decision has been made in accordance with policy." The paper's two named benefits of "proofs as log entries" are a reduced TCB and the ability to detect flaws in complex policies. — [Vaughan et al., CSF 2008](https://www.cis.upenn.edu/~stevez/papers/VJMZ08.pdf)
- Logs record "the reasoning used to reach access control decisions", so audit "can reveal which rules contributed" when a decision is allowed by the rules but violates intent. Operations return a proof acting as "a receipt". — [Vaughan et al., CSF 2008](https://www.cis.upenn.edu/~stevez/papers/VJMZ08.pdf)

**Languages where the verifier evaluates, with no proof carried.**
- SecPAL: "Access is granted if the query succeeds against the current database of clauses." Queries may include negation. — [Becker, Fournet, Gordon, JCS 2010, abstract](https://doi.org/10.3233/jcs-2009-0364)
- DKAL: based on existential fixed-point logic, with a query algorithm in the same complexity bounds as SecPAL, and targeted communication. — [Gurevich and Neeman, CSF 2008, abstract](https://doi.org/10.1109/csf.2008.8)
- Taos: an operating system component "checks credentials according to the formal rules of a logic of authentication". The checker is local and trusted. — [Wobber et al., TOCS 1994, abstract](https://doi.org/10.1145/174613.174614)
- Abadi's survey covers logical foundations and their use in languages for programming security policies. — [Abadi, LICS 2003](https://users.soe.ucsc.edu/~abadi/Papers/lics2003.pdf)

**Evidence on adoption.**
- The most direct statement I found: "whether due to the cost of their primitives, their need for federation, coordination, or public identities, or the complexity of their implementation—such flexible public-key-certificate mechanisms have seen little use in the Cloud." The authors name SPKI/SDSI, Trust Management and SecPAL, and say authorization there is "near exclusively" pure bearer tokens. — [Birgisson et al., NDSS 2014](https://theory.stanford.edu/~ataly/Papers/macaroons.pdf)
- Bauer himself notes that "security logics ... are often not an accurate model of an implemented system". — [Bauer thesis](http://users.ece.cmu.edu/~lbauer/papers/thesis.pdf)
- Cedar leaves the decentralized logic path open: "It would be interesting to explore extensions to Cedar to support the decentralized approach." — [Cutler et al., OOPSLA 2024](https://arxiv.org/abs/2403.04651)

### Inferences

**What a reviewer will press on, and how zk-pdp actually differs from PCA.** These are my analysis, not statements in the sources.

| Property | Proof-carrying authorization | zk-pdp |
|---|---|---|
| Statement | Premises (signed `says` facts) entail a goal, in a logic | A named program, on a committed policy and committed facts, output this decision |
| Who proves | The client, by proof search over credentials | The decider, by running a deterministic evaluator. There is no search. |
| Who checks | The resource server's small proof checker | The verifier's SNARK or STARK verifier |
| Verifier trusts | Logic axioms, checker, signature keys | Proof system soundness, evaluator image or circuit key, and whoever published the commitments |
| Verifier work | Grows with the derivation | Roughly constant |
| What the verifier sees | The whole derivation and the credentials | Only the journal, and hiding holds only in the mode that is actually zero-knowledge |
| Can it prove Deny? | No. Absence of a proof is not a proof of denial. | Yes |
| Can it prove "no forbid applies"? | Only by adding a closed-world premise, since these logics reason from positive evidence | Yes, given a commitment to the complete policy set and store |
| Explanation for audit | The derivation is the explanation | None in the proof. `reasons` in the journal is the substitute. |
| Delegation | Native, it is what the logic is for | None |

- **The new claim is narrower than the project's framing.** "Trusted because of what was computed, not who computed it" is what Appel and Felten built. The defensible new claim has three parts: succinctness, hiding of the premises, and proof of non-monotonic decisions (forbid-overrides, default deny, Deny itself) over a committed closed world. The third is the sharpest. It follows from Cedar's semantics, and I found no source that states it, so a reviewer should treat it as our argument.
- **Name collision.** `docs/spec.md` line 1 and `paper/outline.md` line 3 use "proof-carrying authorization" as the title. `paper/related-work.md` does not mention PCA at all. A reviewer who knows Appel and Felten will read this as either ignorance or a claim of priority. Either cite PCA as the direct ancestor and explain the difference, or retitle.
- **What zk-pdp reinvents from this lineage:**
  - The nonce in the request is the session-bound goal of TR-638-01.
  - A proof log is evidence-based audit.
  - The `reasons` field stands in for what the derivation gave for free.
  - Any verify-then-cache step would be PCFS's conditional capabilities.
  - The "evaluator produces a checkable answer" idea is SD3, though SD3's check runs on the same side of the trust boundary.
- **NAL's taxonomy gives clean wording.** A signed PDP answer is an axiomatic basis: trust the principal. A proof of evaluation is an analytic basis: trust the analysis result. This fits the paper's framing and has a citation behind it.
- **The stall, as far as evidence allows.** The Macaroons sentence names cost, coordination, public identities and complexity. Grey adds client-side proving, scattered credentials, and latency users noticed. Determinism removes proof search and its failure modes from zk-pdp. Attribute gathering does not go away: it moves into producing and distributing commitments. zk-pdp still makes a person wait: 2.2 to 7 seconds per its own README. Grey's average of 6.6 seconds drew complaints, although that figure is end to end and I have no breakdown of how much of it was proving.

### Gaps
- I found no retrospective or adoption study of PCA, Binder, SecPAL or DKAL. Reasons for stalling are inferred from the sources above.
- Not read: Binder, Aura, RT, Delegation Logic, Consumable Credentials, Code-Carrying Authorization. Entries are metadata only or abstract only. The claim that these logics reason from positive evidence alone should be checked per system before printing. SecPAL's queries include negation, though these are evaluated by the verifier and are not carried in a proof.
- The PCA 1999 full text was not retrieved (the guessed URL returned 404). Only its abstract was read.

### BibTeX
```bibtex
% read-abstract
@inproceedings{appel1999pca, title={Proof-carrying authentication}, author={Appel, Andrew W. and Felten, Edward W.}, booktitle={Proceedings of the 6th ACM Conference on Computer and Communications Security}, pages={52--62}, year={1999}, doi={10.1145/319709.319718}}
% read-full (TR)
@techreport{bauer2001pca, title={A Proof-Carrying Authorization System}, author={Bauer, Lujo and Schneider, Michael A. and Felten, Edward W.}, institution={Princeton University}, number={TR-638-01}, year={2001}, url={http://users.ece.cmu.edu/~lbauer/papers/pcaprototr.pdf}}
% search-summary-only
@inproceedings{bauer2002general, title={A General and Flexible Access-Control System for the Web}, author={Bauer, Lujo and Schneider, M. and Felten, Edward W.}, pages={93--108}, year={2002}}
% search-summary-only. Year taken from the DOI; Crossref has no issued date.
@inproceedings{bauer2003discex, title={Access control on the Web using proof-carrying authorization}, author={Bauer, L. and Schneider, M.A. and Felten, E.W. and Appel, A.W.}, booktitle={Proceedings DARPA Information Survivability Conference and Exposition}, pages={117--119}, year={2003}, doi={10.1109/discex.2003.1194942}}
% read-full (cited sections)
@phdthesis{bauer2003thesis, title={Access Control for the Web via Proof-Carrying Authorization}, author={Bauer, Ljudevit}, school={Princeton University}, year={2003}, url={http://users.ece.cmu.edu/~lbauer/papers/thesis.pdf}}
% read-abstract
@inproceedings{bauer2005grey, title={Device-Enabled Authorization in the Grey System}, author={Bauer, Lujo and Garriss, Scott and McCune, Jonathan M. and Reiter, Michael K. and Rouse, Jason and Rutenbar, Peter}, booktitle={Lecture Notes in Computer Science}, pages={431--445}, year={2005}, doi={10.1007/11556992_31}}
% read-abstract
@inproceedings{bauer2005distributed, title={Distributed Proving in Access-Control Systems}, author={Bauer, Lujo and Garriss, Scott and Reiter, Michael K.}, booktitle={IEEE Symposium on Security and Privacy}, pages={81--95}, year={2005}, doi={10.1109/sp.2005.9}}
% search-summary-only
@inproceedings{bauer2007efficient, title={Efficient Proving for Practical Distributed Access-Control Systems}, author={Bauer, Lujo and Garriss, Scott and Reiter, Michael K.}, booktitle={Lecture Notes in Computer Science}, pages={19--37}, year={2007}, doi={10.1007/978-3-540-74835-9_3}}
% read-full
@inproceedings{bauer2007lessons, title={Lessons learned from the deployment of a smartphone-based access-control system}, author={Bauer, Lujo and Cranor, Lorrie Faith and Reiter, Michael K. and Vaniea, Kami}, booktitle={Symposium On Usable Privacy and Security}, pages={64--75}, year={2007}, doi={10.1145/1280680.1280689}}
% search-summary-only
@inproceedings{bowers2007consumable, title={Consumable Credentials in Linear-Logic-Based Access-Control Systems}, author={Bowers, Kevin D. and Bauer, Lujo and Garg, Deepak and Pfenning, Frank and Reiter, Michael K.}, booktitle={Network and Distributed System Security Symposium}, year={2007}}
% search-summary-only
@incollection{garg2006linear, title={A Linear Logic of Authorization and Knowledge}, author={Garg, Deepak and Bauer, Lujo and Bowers, Kevin D. and Pfenning, Frank and Reiter, Michael K.}, booktitle={Lecture Notes in Computer Science}, pages={297--312}, year={2006}, doi={10.1007/11863908_19}}
% read-abstract
@inproceedings{garg2010pcfs, title={A Proof-Carrying File System}, author={Garg, Deepak and Pfenning, Frank}, booktitle={IEEE Symposium on Security and Privacy}, pages={349--364}, year={2010}, doi={10.1109/sp.2010.28}}
% read-full (opening sections)
@article{schneider2011nal, title={Nexus Authorization Logic (NAL): Design Rationale and Applications}, author={Schneider, Fred B. and Walsh, Kevin and Sirer, Emin G{\"u}n}, journal={ACM Transactions on Information and System Security}, volume={14}, number={1}, pages={1--28}, year={2011}, doi={10.1145/1952982.1952990}}
% read-abstract
@inproceedings{sirer2011logical, title={Logical attestation}, author={Sirer, Emin G{\"u}n and de Bruijn, Willem and Reynolds, Patrick and Shieh, Alan and Walsh, Kevin S. and Williams, Dan and Schneider, Fred B.}, booktitle={ACM SIGOPS Symposium on Operating Systems Principles}, pages={249--264}, year={2011}, doi={10.1145/2043556.2043580}}
% search-summary-only
@inproceedings{detreville2002binder, title={Binder, a logic-based security language}, author={DeTreville, J.}, booktitle={Proceedings 2002 IEEE Symposium on Security and Privacy}, pages={105--113}, year={2002}, doi={10.1109/SECPRI.2002.1004365}}
% read-abstract
@article{becker2010secpal, title={SecPAL: Design and semantics of a decentralized authorization language}, author={Becker, Moritz Y. and Fournet, C{\'e}dric and Gordon, Andrew D.}, journal={Journal of Computer Security}, pages={619--665}, year={2010}, doi={10.3233/jcs-2009-0364}}
% read-abstract
@inproceedings{gurevich2008dkal, title={DKAL: Distributed-Knowledge Authorization Language}, author={Gurevich, Yuri G. and Neeman, Itay}, booktitle={IEEE Computer Security Foundations Symposium}, pages={149--162}, year={2008}, doi={10.1109/csf.2008.8}}
% search-summary-only
@article{abadi1993calculus, title={A calculus for access control in distributed systems}, author={Abadi, Mart{\'\i}n and Burrows, Michael T. and Lampson, Butler and Plotkin, Gordon D.}, journal={ACM Transactions on Programming Languages and Systems}, pages={706--734}, year={1993}, doi={10.1145/155183.155225}}
% read-abstract
@article{wobber1994taos, title={Authentication in the Taos operating system}, author={Wobber, Edward P. and Abadi, Mart{\'\i}n and Burrows, Michael T. and Lampson, Butler}, journal={ACM Transactions on Computer Systems}, pages={3--32}, year={1994}, doi={10.1145/174613.174614}}
% search-summary-only
@article{lampson1992authentication, title={Authentication in distributed systems}, author={Lampson, Butler and Abadi, Mart{\'\i}n and Burrows, Michael T. and Wobber, Edward P.}, journal={ACM Transactions on Computer Systems}, pages={265--310}, year={1992}, doi={10.1145/138873.138874}}
% search-summary-only
@inproceedings{abadi2006core, title={Access control in a core calculus of dependency}, author={Abadi, Mart{\'\i}n}, booktitle={ACM SIGPLAN International Conference on Functional Programming}, pages={263--273}, year={2006}, doi={10.1145/1159803.1159839}}
% read-abstract (first page)
@inproceedings{abadi2003logic, title={Logic in access control}, author={Abadi, Mart{\'\i}n}, booktitle={18th Annual IEEE Symposium of Logic in Computer Science}, pages={228--233}, year={2003}, doi={10.1109/lics.2003.1210062}}
% search-summary-only. Crossref lists the main title only.
@inproceedings{jia2008aura, title={AURA}, author={Jia, Limin and Vaughan, Jeffrey A. and Mazurak, Karl and Zhao, Jianzhou and Zarko, Luke and Schorr, Joseph and Zdancewic, Steve}, booktitle={Proceedings of the 13th ACM SIGPLAN International Conference on Functional Programming}, pages={27--38}, year={2008}, doi={10.1145/1411204.1411212}}
% read-full (cited sections)
@inproceedings{vaughan2008evidence, title={Evidence-Based Audit}, author={Vaughan, Jeffrey A. and Jia, Limin and Mazurak, Karl and Zdancewic, Steve}, booktitle={2008 21st IEEE Computer Security Foundations Symposium}, pages={177--191}, year={2008}, doi={10.1109/CSF.2008.24}}
% read-abstract
@inproceedings{jim2001sd3, title={SD3: a trust management system with certified evaluation}, author={Jim, T.}, booktitle={Proceedings 2001 IEEE Symposium on Security and Privacy}, pages={106--115}, year={2001}, doi={10.1109/SECPRI.2001.924291}}
% search-summary-only
@inproceedings{necula1997pcc, title={Proof-carrying code}, author={Necula, George C.}, booktitle={Proceedings of the 24th ACM SIGPLAN-SIGACT Symposium on Principles of Programming Languages}, pages={106--119}, year={1997}, doi={10.1145/263699.263712}}
% search-summary-only
@incollection{maffeis2008code, title={Code-Carrying Authorization}, author={Maffeis and Abadi and Fournet and Gordon}, booktitle={Lecture Notes in Computer Science}, pages={563--579}, year={2008}, doi={10.1007/978-3-540-88313-5_36}}
```

## Trust management and capability tokens: PolicyMaker, KeyNote, SPKI/SDSI, Macaroons, Biscuit, UCAN, Zcaps

### Takeaway
These systems answer a different question. They prove that authority was granted and restricted along a chain, and then the verifier runs the check itself and sees the facts. zk-pdp proves that an evaluation came out a given way, so the verifier runs nothing. It has no delegation at all, which is the one thing every token system has.

### Cited Findings
- Trust management began with PolicyMaker. — [Blaze, Feigenbaum, Lacy, S&P 1996](https://doi.org/10.1109/SECPRI.1996.502679); [compliance checking](https://doi.org/10.1007/bfb0055488)
- KeyNote's architecture has a "compliance checker" as a service to applications, which "can simply ask the compliance checker" and gets back an application-configured "policy compliance value". This is a PDP/PEP split in a 1999 RFC. — [RFC 2704](https://www.rfc-editor.org/rfc/rfc2704.txt)
- SPKI certificate theory is an Experimental RFC. — [RFC 2693](https://www.rfc-editor.org/rfc/rfc2693.txt)
- Chain discovery is a separate research problem. — [Clarke et al., JCS 2001](https://doi.org/10.3233/jcs-2001-9402)
- SD3 and others are Datalog-based trust management, per Cedar's related-work section, which also lists RT and QCM. — [Cutler et al.](https://arxiv.org/abs/2403.04651)
- Macaroons are built from nested HMACs. They are bearer credentials with caveats that restrict when, where, by whom and for what purpose they are accepted, and the paper formalizes them in authorization logic. — [Birgisson et al., NDSS 2014](https://doi.org/10.14722/ndss.2014.23212)
- Third-party caveats, their "main innovation", "require any number of holder-of-key proofs" through discharge macaroons, for example from a revocation-checking service. — [Birgisson et al.](https://theory.stanford.edu/~ataly/Papers/macaroons.pdf)
- Biscuit "is a bearer token that supports offline attenuation, can be verified by any system that knows the root public key, and provides a flexible authorization language based on logic programming". Allow/deny policies "can only be defined in the authorizer". — [Biscuit specification](https://raw.githubusercontent.com/eclipse-biscuit/biscuit/main/SPECIFICATIONS.md)
- Biscuit's history includes a forged third-party block request that could trick the authority into signing wrong-key trust, and a vulnerable v1 signature algorithm. This comes from a search summary of a CVE list. — [releasealert CVE listing](https://releasealert.dev/github/eclipse-biscuit/biscuit/cves)
- UCAN "improves the familiarity and adoptability of schemes like SPKI/SDSI". "The delegation chain is by definition a provenance log", and there is no authorization server. — [UCAN specification](https://raw.githubusercontent.com/ucan-wg/spec/main/README.md)
- UCAN's own comparison: ZCAP-LD differs in formatting, addressing, and separating invocation from authorization. Biscuit "uses Datalog to describe capabilities". Macaroons avoid asymmetric signatures. — [UCAN specification](https://raw.githubusercontent.com/ucan-wg/spec/main/README.md)
- UCAN's version: the main-branch README reads 1.0.0 — [README](https://raw.githubusercontent.com/ucan-wg/spec/main/README.md); a search summary of the repo showed 1.0.0-rc.1 — [repo](https://github.com/ucan-wg/spec). I take the README as newer but did not find a release record.
- ZCAP's published version is v0.4.0 under the W3C CCG. — [ZCAP spec](https://w3c-ccg.github.io/zcap-spec/) An April 2026 list post calls it "somewhat dormant" and asks for editors. — [public-credentials list, via search summary](https://lists.w3.org/Archives/Public/public-credentials/2026Apr/0055.html)
- Anonymous delegation exists in the ZK literature. A 2026 ePrint uses recursive proofs for constant-size delegation, reporting 1.2 s to delegate and 3.3 ms to verify at 64 attributes. — [ePrint 2026/1855, via search summary](https://eprint.iacr.org/2026/1855)
- Agent token work builds on this lineage. AIP's Invocation-Bound Capability Tokens use a Biscuit chained mode with Datalog for multi-hop delegation across MCP and A2A. — [Prakash, arXiv 2603.24775](https://arxiv.org/abs/2603.24775)

### Inferences

| System | What the verifier checks | Verifier sees facts? | Public verify? | Offline attenuation? | What zk-pdp adds | What it lacks or duplicates |
|---|---|---|---|---|---|---|
| PolicyMaker, KeyNote | Runs compliance check locally on assertions | Yes | Signature-based | Delegation assertions | Verifier need not run the checker or hold the facts | Delegation. Its PDP/PEP split is not new. |
| SPKI/SDSI | Reduces a certificate chain | Yes | Yes | Yes | Hiding, and rich conditions | Names and delegation. No chain-discovery problem, but a commitment-gathering one. |
| Macaroons | Recomputes the HMAC chain with the root key, checks caveats | Yes | No, needs the root key (the paper outlines a public-key variant) | Yes, cheaply | Public verifiability, plus hiding | Third-party discharge is a composition mechanism zk-pdp would need to reinvent |
| Biscuit | Verifies signatures, then runs Datalog | Yes | Yes | Yes | Verifier runs nothing, and sees nothing | Offline attenuation |
| UCAN, Zcaps | Verifies a delegation chain and invocation | Yes | Yes | Yes | Evaluated policy with private inputs | Delegation provenance |

- **Biscuit is the sharpest comparator for "a client decides offline".** If the verifier can run the policy and may see the data, a signed token with local evaluation has none of zk-pdp's proving cost. The verifier sidesteps trust in the decider by simply re-deciding.
- **Succinctness does not save CPU here, and the paper should say so.** Cedar decides in microseconds (median 4.0 to 5.0 µs on gdrive, about 11.0 µs on github) — [Cutler et al.](https://arxiv.org/abs/2403.04651). zk-pdp's README reports 20 ms to verify, roughly 1,800 to 5,000 times longer. This compares different machines and is order-of-magnitude only. So zk-pdp's value is never that checking is faster than deciding. It holds only where the verifier cannot or may not hold the policy or the facts, or where those are too large to ship. The spec's non-goal about trusted PDPs is right, and should be extended to "verifiers that hold the inputs".
- **Composition.** The natural pairing is a token that establishes who and what authority, and a proof that the policy evaluated correctly over that authority. A third-party caveat discharged by a zk-pdp proof is a concrete design that uses both. I found no source proposing it.
- zk-pdp proofs are not attenuable. Doing so requires recursion, which is a further proof, and there is no holder-side narrowing like a macaroon caveat.

### Gaps
- PolicyMaker and SPKI/SDSI papers were not read beyond their metadata and the KeyNote and SPKI RFC headers and excerpts.
- No peer-reviewed Biscuit, UCAN or ZCAP paper was found. All three are specifications.
- I found no formal comparison of capability tokens against SNARK-based authorization.

### BibTeX
```bibtex
% search-summary-only
@inproceedings{blaze1996decentralized, title={Decentralized trust management}, author={Blaze, M. and Feigenbaum, J. and Lacy, J.}, booktitle={Proceedings 1996 IEEE Symposium on Security and Privacy}, pages={164--173}, year={1996}, doi={10.1109/SECPRI.1996.502679}}
% search-summary-only
@incollection{blaze1998compliance, title={Compliance checking in the PolicyMaker trust management system}, author={Blaze, Matt and Feigenbaum, Joan and Strauss, Martin}, booktitle={Lecture Notes in Computer Science}, pages={254--274}, year={1998}, doi={10.1007/BFb0055488}}
% read-abstract (excerpts)
@misc{rfc2704, title={The KeyNote Trust-Management System Version 2}, author={Blaze, M. and Feigenbaum, J. and Ioannidis, J. and Keromytis, A.}, howpublished={RFC 2704}, year={1999}, url={https://www.rfc-editor.org/rfc/rfc2704.txt}}
% search-summary-only
@misc{rfc2693, title={SPKI Certificate Theory}, author={Ellison, C. and Frantz, B. and Lampson, B. and Rivest, R. and Thomas, B. and Ylonen, T.}, howpublished={RFC 2693}, year={1999}, url={https://www.rfc-editor.org/rfc/rfc2693.txt}}
% search-summary-only
@article{clarke2001chain, title={Certificate chain discovery in SPKI/SDSI}, author={Clarke and Elien and Ellison and Fredette and Morcos and Rivest}, journal={Journal of Computer Security}, pages={285--322}, year={2001}, doi={10.3233/jcs-2001-9402}}
% read-full (cited sections)
@inproceedings{birgisson2014macaroons, title={Macaroons: Cookies with Contextual Caveats for Decentralized Authorization in the Cloud}, author={Birgisson, Arnar and Politz, Joe Gibbs and Erlingsson, {\'U}lfar and Taly, Ankur and Vrable, Michael and Lentczner, Mark}, booktitle={Network and Distributed System Security Symposium}, year={2014}, doi={10.14722/ndss.2014.23212}}
% read-full (cited sections). A specification.
@misc{biscuitspec, title={Biscuit, a bearer token with offline attenuation and decentralized verification}, howpublished={Eclipse Biscuit specification}, url={https://github.com/eclipse-biscuit/biscuit}}
% read-full (cited sections). A specification. Version in README: 1.0.0.
@misc{ucanspec, title={User Controlled Authorization Network (UCAN) Specification}, editor={Zelenka, Brooklyn}, url={https://github.com/ucan-wg/spec}}
% search-summary-only. A draft.
@misc{zcapspec, title={Authorization Capabilities (ZCAP)}, howpublished={W3C Credentials Community Group, v0.4.0}, url={https://w3c-ccg.github.io/zcap-spec/}}
% read-abstract
@misc{prakash2026aip, title={AIP: Agent Identity Protocol for Verifiable Delegation Across MCP and A2A}, author={Prakash, Sunil}, year={2026}, eprint={2603.24775}, archivePrefix={arXiv}}
% search-summary-only. ePrint id only; title and authors not captured.
@misc{eprint20261855, howpublished={IACR Cryptology ePrint Archive, Paper 2026/1855}, year={2026}, url={https://eprint.iacr.org/2026/1855}}
```

## Policy languages and verified analysis: Cedar, its verification, Zelkova, and formal semantics for the others

### Takeaway
Cedar is unusually well placed. It has a machine-checked model, proved properties, and a verified symbolic compiler, and I found no comparable mechanized semantics for XACML, Rego, Zanzibar, OpenFGA, Polar or Casbin. Two things need care. The verified object is a Lean model and not the Rust evaluator. And policy analysis proves properties for all requests, which neither replaces nor is replaced by a proof of one decision.

### Cited Findings

**Cedar (read).**
- Authorization: a request is allowed only if no forbid policy and at least one permit policy is satisfied. — [Cutler et al.](https://arxiv.org/abs/2403.04651)
- Proved in Lean: forbid trumps permit, default deny, explicit allow, sound slicing, and validation soundness. — [Cutler et al.](https://arxiv.org/abs/2403.04651)
- The decision comes with diagnostics: the IDs of the determining policies (permits that were true for Allow, or forbids that were true for Deny) and errors. — [Cutler et al.](https://arxiv.org/abs/2403.04651)
- Policies have no side effects, and evaluation order does not matter. — [Cutler et al.](https://arxiv.org/abs/2403.04651)
- Encoding: reduction to a decidable SMT fragment, sound and complete for standard analyses such as equivalence and subsumption. In the 2024 paper this was proved on paper, with Lean proofs "underway". — [Cutler et al.](https://arxiv.org/abs/2403.04651)
- Performance: 28.7× to 35.2× faster than OpenFGA and 42.8× to 80.8× faster than Rego on generated inputs. Analysis takes 75.1 ms on average to encode and solve. — [Cutler et al.](https://arxiv.org/abs/2403.04651)
- Cedar decides in about 4.0 to 5.0 µs (gdrive) and 11.0 µs (github) median. — [Cutler et al.](https://arxiv.org/abs/2403.04651)

**How Cedar is verified.**
- Verification-guided development has three parts: an executable model with mechanized proofs, differential random testing (DRT) of production code against the model, and property-based testing of unmodeled parts. — [Disselkoen et al., FSE Companion 2024](https://doi.org/10.1145/3663529.3663854)
- Proofs found 4 validator bugs. DRT and PBT found 21 more. — [Disselkoen et al.](https://arxiv.org/abs/2407.01688)
- Size: Lean model 1,673 LOC, Lean proofs 5,714, Rust production 15,693. The evaluator and authorizer are 897 Lean LOC with 347 of proof, against 4,877 of Rust. The validator has 4,686 lines of proof. — [Disselkoen et al.](https://arxiv.org/abs/2407.01688)
- "Our parsers are not modeled in Lean." — [Disselkoen et al.](https://arxiv.org/abs/2407.01688)

**SymCert (read).**
- Provides a verified symbolic compiler and authorizer, a hierarchy enforcer, and a counterexample extractor, handling full Cedar. The title page shows Emina Torlak as sole author, in FMCAD 2026. — [Torlak, FMCAD 2026](https://cdn.amazon.science/9f/c9/5d18658a41c48628b686493aa327/scipub-approval152134-46496997-symcert-verifying-smtbased-policy-analyses.pdf)
- Compiler correctness decomposes into reducibility and interpretability. Theorem 4 states authorizer correctness: `isAuthorized(C, ε) ∼ evals(authorize(C, ε̂), I)`. — [Torlak](https://cdn.amazon.science/9f/c9/5d18658a41c48628b686493aa327/scipub-approval152134-46496997-symcert-verifying-smtbased-policy-analyses.pdf)
- The solver is trusted (566 LOC model, 0 proof). The compiler is 318 LOC, and the Term language has 42 operators. — [Torlak](https://cdn.amazon.science/9f/c9/5d18658a41c48628b686493aa327/scipub-approval152134-46496997-symcert-verifying-smtbased-policy-analyses.pdf)
- Extended for enums, datetime and duration (8 days, 2,537 lines), and tags, on Cedar 4.4. Five industrial analyses took one work day to prove. Queries take under 20 ms to generate and under 40 ms to solve. — [Torlak](https://cdn.amazon.science/9f/c9/5d18658a41c48628b686493aa327/scipub-approval152134-46496997-symcert-verifying-smtbased-policy-analyses.pdf)

**Zelkova and AWS policy analysis.**
- Zelkova encodes AWS policy semantics into SMT, compares behaviors and verifies properties, solving a PSPACE-complete problem, and is "invoked many millions of times daily". It analyzes policies, not request-time decisions. — [Backes et al., FMCAD 2018](https://www.amazon.science/publications/semantic-based-automated-reasoning-for-aws-access-policies-using-smt)
- Related AWS work: [Cook, 2018](https://doi.org/10.1007/978-3-319-96145-3_3) and [Bouchet et al., FSE 2020](https://doi.org/10.1145/3368089.3409728).
- Eiers et al. use model counting to compare permissiveness, per Cedar's related work. — [Cutler et al.](https://arxiv.org/abs/2403.04651)

**Other languages (formal status).**
- XACML: per Cedar's authors, it has typed operators but "no schema-style validator (that we could find)", and prior analyses rest on partial formalizations. Margrave's multi-terminal decision diagrams cover a limited subset, and Hughes and Bultan's SAT encoding is neither sound nor complete. — [Cutler et al.](https://arxiv.org/abs/2403.04651); [Fisler et al., ICSE 2005](https://doi.org/10.1145/1062455.1062502)
- XACML also has a logical treatment and a description-logic analysis. — [Ramli et al., SCP 2014](https://doi.org/10.1016/j.scico.2013.05.003); [Kolovski et al., WWW 2007](https://doi.org/10.1145/1242572.1242664)
- Rego: OPA says it "was inspired by Datalog and extends it" to JSON. — [OPA docs](https://www.openpolicyagent.org/docs/policy-language) Cedar's authors say its JSON-schema validator "is neither specified carefully nor proved sound", and note that Datalog program equivalence is undecidable. — [Cutler et al.](https://arxiv.org/abs/2403.04651)
- OPA does sign policy bundles and does produce decision logs. — [OPA bundles](https://www.openpolicyagent.org/docs/management-bundles); [OPA policy language](https://www.openpolicyagent.org/docs/policy-language)
- Zanzibar: "trillions of access control lists and millions of authorization requests per second", p95 under 10 ms, over 99.999% availability across 3 years. — [Pang et al., ATC 2019](https://www.usenix.org/conference/atc19/presentation/pang)
- Zanzibar's "new enemy" problem, solved with zookies, ensures ACL checks respect causal ordering with content changes. — [Pang et al.](https://www.usenix.org/system/files/atc19-pang.pdf)
- Cedar's authors note Zanzibar "provides no deep policy analysis". — [Cutler et al.](https://arxiv.org/abs/2403.04651)
- OpenFGA implements Zanzibar's core operations and adds a schema DSL and CEL conditions. — [OpenFGA docs, via search summary](https://openfga.dev/docs/learn/zanzibar.md)
- Datalog foundations for trust languages: [Li and Mitchell, 2002](https://doi.org/10.1007/3-540-36388-2_6).
- Casbin: the paper usually associated with it, PML, describes an interpreter-based policy language. The arXiv page does not mention Casbin or a formal semantics. — [Luo, Shen, Wu, arXiv 1903.09756](https://arxiv.org/abs/1903.09756)

### Inferences
- **Correct the loose wording in the spec.** `docs/spec.md` section 2 says Cedar "has a Lean-verified evaluator". More precisely, a Lean model of the evaluator has proved properties, and the Rust evaluator is differentially tested against it. The README's phrasing ("checked against a formal model") is the accurate one. This matters directly for the zkVM backend: its proof attests the Rust binary ran correctly, so its link to the Lean theorems is DRT, not proof. The README attributes 99 percent of zk-pdp's zkVM work to parsing text into data structures, and Cedar's parsers are not modeled in Lean, so most of what that proof attests has no formal model behind it.
- **The circuit backend's chain has a hole that is easy to overclaim.** SymCert covers Cedar to Term. Term to Noir is zk-pdp's own lowering and sits outside the proof. The solver leaves the trusted base, but a lowering pass, the Noir compiler and the proof system enter it. Saying "verified compiler" for the whole path would be wrong.
- **zk-pdp's missing features are lowering work, not semantic gaps.** SymCert already handles datetime, duration and tags, which the README lists as not compiling yet.
- **Analysis and proof are complementary, and that composition is the novel angle.** Zelkova and Cedar Analysis prove "for all requests, property P holds of this policy". A proof of evaluation shows "for this request, this policy gave this answer", and says nothing about whether the policy is any good. Bound to a shared policy hash, the two give: this decision came from a policy that has been shown never to permit X. No analysis verdict is cryptographically bound to a decision today in any source I read.
- **Why Cedar and not Rego or XACML.** Beyond its model, Cedar's termination, determinism and order independence are what make a fixed circuit possible. Rego's general search and Datalog rules make that harder. Prezta tests XACML against a conformance suite (83 percent, per the local note `docs/sources/S01-prezta.md`), and I found no mechanized XACML reference it could test against instead.
- **Zanzibar's "new enemy" is zk-pdp's stale-root problem under another name.** A proof over an old entity commitment is a proof of an old truth. This should be cited rather than rediscovered.

### Gaps
- I found no formal semantics for Rego, Zanzibar, SpiceDB, OpenFGA or Polar, in two searches. This is absence in search, not proof of absence. The Oso docs URL I tried returned 404, so Polar's description rests on third-party comparison pages.
- Ramli et al., Kolovski et al., Margrave, Cook and Bouchet were not read beyond metadata.
- The current Cedar Lean status beyond SymCert was not checked. `docs/sources` has other notes on it.

### BibTeX
```bibtex
% read-full
@article{cutler2024cedar, title={Cedar: A New Language for Expressive, Fast, Safe, and Analyzable Authorization}, author={Cutler, Joseph W. and Disselkoen, Craig and Eline, Aaron and He, Shaobo and Headley, Kyle and Hicks, Michael and Hietala, Kesha and Ioannidis, Eleftherios and Kastner, John and Mamat, Anwar and McAdams, Darin and McCutchen, Matt and Rungta, Neha and Torlak, Emina and Wells, Andrew}, journal={Proceedings of the ACM on Programming Languages}, volume={8}, number={OOPSLA1}, pages={670--697}, year={2024}, doi={10.1145/3649835}, eprint={2403.04651}, archivePrefix={arXiv}}
% read-full
@inproceedings{disselkoen2024vgd, title={How We Built Cedar: A Verification-Guided Approach}, author={Disselkoen, Craig and Eline, Aaron and He, Shaobo and Headley, Kyle and Hicks, Michael and Hietala, Kesha and Kastner, John and Mamat, Anwar and McCutchen, Matt and Rungta, Neha and Shah, Bhakti and Torlak, Emina and Wells, Andrew}, booktitle={Companion Proceedings of the 32nd ACM International Conference on the Foundations of Software Engineering}, pages={351--357}, year={2024}, doi={10.1145/3663529.3663854}, eprint={2407.01688}, archivePrefix={arXiv}}
% read-full
@inproceedings{torlak2026symcert, title={SymCert: Verifying SMT-Based Policy Analyses}, author={Torlak, Emina}, booktitle={Formal Methods in Computer-Aided Design}, year={2026}, url={https://www.amazon.science/publications/symcert-verifying-smt-based-policy-analyses}}
% read-abstract
@inproceedings{backes2018zelkova, title={Semantic-based Automated Reasoning for AWS Access Policies using SMT}, author={Backes, John and Bolignano, Pauline and Cook, Byron and Dodge, Catherine and Gacek, Andrew and Luckow, Kasper and Rungta, Neha and Tkachuk, Oksana and Varming, Carsten}, booktitle={2018 Formal Methods in Computer Aided Design}, pages={1--9}, year={2018}, doi={10.23919/FMCAD.2018.8602994}}
% search-summary-only
@incollection{cook2018formal, title={Formal Reasoning About the Security of Amazon Web Services}, author={Cook, Byron}, booktitle={Lecture Notes in Computer Science}, pages={38--47}, year={2018}, doi={10.1007/978-3-319-96145-3_3}}
% search-summary-only
@inproceedings{bouchet2020block, title={Block public access: trust safety verification of access control policies}, author={Bouchet, Malik and Cook, Byron and Cutler, Bryant and Druzkina, Anna and Gacek, Andrew and Hadarean, Liana and Jhala, Ranjit and Marshall, Brad and Peebles, Dan and Rungta, Neha and Schlesinger, Cole and Stephens, Chriss and Varming, Carsten and Warfield, Andy}, booktitle={Proceedings of the 28th ACM Joint Meeting on European Software Engineering Conference and Symposium on the Foundations of Software Engineering}, pages={281--291}, year={2020}, doi={10.1145/3368089.3409728}}
% search-summary-only
@inproceedings{fisler2005margrave, title={Verification and change-impact analysis of access-control policies}, author={Fisler, Kathi and Krishnamurthi, Shriram and Meyerovich, Leo A. and Tschantz, Michael Carl}, booktitle={Proceedings of the 27th International Conference on Software Engineering}, year={2005}, doi={10.1145/1062455.1062502}}
% search-summary-only
@article{ramli2014logic, title={The logic of XACML}, author={Ramli, Carroline Dewi Puspa Kencana and Nielson, Hanne Riis and Nielson, Flemming}, journal={Science of Computer Programming}, volume={83}, pages={80--105}, year={2014}, doi={10.1016/j.scico.2013.05.003}}
% search-summary-only
@inproceedings{kolovski2007analyzing, title={Analyzing web access control policies}, author={Kolovski and Hendler and Parsia}, booktitle={Proceedings of the 16th International Conference on World Wide Web}, pages={677--686}, year={2007}, doi={10.1145/1242572.1242664}}
% search-summary-only
@inproceedings{li2002datalog, title={Datalog with Constraints: A Foundation for Trust Management Languages}, author={Li, Ninghui and Mitchell, John C.}, booktitle={Lecture Notes in Computer Science}, pages={58--73}, year={2002}, doi={10.1007/3-540-36388-2_6}}
% search-summary-only
@inproceedings{li2002rt, title={Design of a role-based trust-management framework}, author={Li, Ninghui and Mitchell, J.C. and Winsborough, W.H.}, booktitle={Proceedings 2002 IEEE Symposium on Security and Privacy}, pages={114--130}, year={2002}, doi={10.1109/SECPRI.2002.1004366}}
% read-abstract
@article{li2003delegation, title={Delegation logic}, author={Li, Ninghui and Grosof, Benjamin N. and Feigenbaum, Joan}, journal={ACM Transactions on Information and System Security}, volume={6}, number={1}, pages={128--171}, year={2003}, doi={10.1145/605434.605438}}
% read-abstract, plus excerpts on new enemy and zookies
@inproceedings{pang2019zanzibar, title={Zanzibar: Google's Consistent, Global Authorization System}, author={Pang, Ruoming and Caceres, Ramon and Burrows, Mike and Chen, Zhifeng and Dave, Pratik and Germer, Nathan and Golynski, Alexander and Graney, Kevin and Kang, Nina and Kissner, Lea and Korn, Jeffrey L. and Parmar, Abhishek and Richards, Christina D. and Wang, Mengzhi}, booktitle={2019 USENIX Annual Technical Conference}, pages={33--46}, year={2019}, url={https://www.usenix.org/conference/atc19/presentation/pang}}
% read-abstract
@misc{luo2019pml, title={PML: An Interpreter-Based Access Control Policy Language for Web Services}, author={Luo, Yang and Shen, Qingni and Wu, Zhonghai}, year={2019}, eprint={1903.09756}, archivePrefix={arXiv}}
```

## Standards: AuthZEN, XACML, NIST, OAuth, WIMSE, and drafts about signed or verifiable decisions

### Takeaway
Every standard I read treats the PDP as trusted and stops at a signature. Signed portable decisions are at least twelve years old in XACML's SAML profile. The new receipt drafts sign decisions and explicitly do not claim they are correct. None of the drafts I read defines a proof of correct evaluation. One thing in AuthZEN contradicts a zk-pdp claim: section 5.5 lets a PEP reject a permit whose context it does not understand.

### Cited Findings

**AuthZEN Authorization API 1.0.**
- Final, published 11 January 2026, editors Gazitt, Brossard, Tulshibagwale. — [Spec](https://openid.net/specs/authorization-api-1_0.html)
- Approved by vote: 81 approve, 1 object, 25 abstain. — [OpenID Foundation](https://openid.net/authorization-api-1-0-final-specification-approved/)
- **Section 5.5:** on `true`, "If the PEP does not understand information in the context response object, the PEP MAY choose to reject the decision." — [Spec](https://openid.net/specs/authorization-api-1_0.html)
- Section 10.1.1: "receivers MUST ignore unknown fields present in request or response bodies." — [Spec](https://openid.net/specs/authorization-api-1_0.html)
- Section 5.5.2: the format and semantics of `context` "are an implementation concern and outside the scope of this specification". — [Spec](https://openid.net/specs/authorization-api-1_0.html)
- **Section 11.6:** "The PDP MAY choose to sign its authorization response, ensuring the PEP can verify the integrity of the data it receives." Also: "It ensures response accuracy and completeness." It cites non-repudiation and tamper detection, and defines no mechanism. — [Spec](https://openid.net/specs/authorization-api-1_0.html)
- Section 11.4: "The architecture of this model assumes the PDP must trust the PEP, as the PEP is ultimately responsible for enforcing the decision." — [Spec](https://openid.net/specs/authorization-api-1_0.html)
- The WG's drafts are COAZ, COAZ-MCP, Access Request and Approval, and Obligations. The listing page mentions no signed or verifiable decisions. — [AuthZEN specifications](https://openid.net/wg/authzen/specifications/)
- COAZ-MCP (Tulshibagwale, Olivier, Draft 1, 13 February 2026) maps `tools/call` to subject, action `tools/call` and resource type `tool`. The PEP is an MCP gateway or the server. It mentions no signed responses or proofs. — [COAZ-MCP](https://openid.github.io/authzen/authzen-coaz-mcp-binding-1_0)

**XACML.**
- XACML 3.0: OASIS Standard, 22 January 2013, editor Erik Rissanen. Section 9.2.1: "it is important for a PEP to authenticate the identity of the PDP". Otherwise "an adversary could provide false or invalid authorization decisions". Section 9.1.5 asks for a message integrity safeguard. — [XACML 3.0](https://docs.oasis-open.org/xacml/3.0/xacml-3.0-core-spec-os-en.html)
- **XACML SAML Profile 2.0 (Committee Specification 02, 19 August 2014):** "The XACMLAuthzDecision Assertion SHOULD be signed by a Policy Decision Point trusted by the Web Service. A Web Service MAY use this token to determine that a trusted 3rd party has evaluated an XACML Request Context." — [XACML SAML Profile](https://docs.oasis-open.org/xacml/3.0/xacml-profile-saml2.0-v2-spec-en.html)

**NIST.**
- SP 800-162 ABAC: January 2014, updated 2 August 2019. — [NIST](https://csrc.nist.gov/pubs/sp/800/162/upd2/final)
- SP 800-207 Zero Trust: August 2020. "Access is granted through a policy decision point (PDP) and corresponding policy enforcement point (PEP)", with an "implicit trust zone" shrunk around them. — [SP 800-207](https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-207.pdf)
- NCCoE concept paper (Booth, Fisher, Galluzzo, Roberts, February 2026) asks "What are the mechanisms for an agent to prove its authority to perform a specific action?" and how agents can log "in a tamper-proof and verifiable manner". — [NCCoE concept paper](https://www.nccoe.nist.gov/sites/default/files/2026-02/accelerating-the-adoption-of-software-and-ai-agent-identity-and-authorization-concept-paper.pdf)

**OAuth and workload identity.**
- RFC 9396 (May 2023): `authorization_details` carries fine-grained requirements, and the authorization server decides. — [RFC 9396](https://www.rfc-editor.org/rfc/rfc9396.html)
- RFC 9449 DPoP (September 2023): shows possession of a key, and does not by itself justify an access decision. — [RFC 9449](https://www.rfc-editor.org/rfc/rfc9449.html)
- WIMSE architecture: draft-ietf-wimse-arch-08, 6 July 2026, Informational, active. — [Datatracker](https://datatracker.ietf.org/doc/draft-ietf-wimse-arch/)
- Transaction Tokens: short-lived signed JWTs carrying identity and authorization context through a call chain. I saw revision -11. — [draft-ietf-oauth-transaction-tokens-11](https://ftp.sparcs.org/ietf/draft-ietf-oauth-transaction-tokens-11.xml)

**Drafts on signed or receipted decisions.**
- draft-farley-acta-signed-receipts-01 (Farley, ScopeBlind, 25 April 2026; individual, Informational) signs decision metadata with Ed25519 and offers an optional `policy_digest`. It makes no claim that a decision was correct and defines no proof of evaluation. Section 5.9 requires only that a ZK proof from a separate protocol bind to `committed_fields_root`. Appendix A mentions Cedar WASM evaluation and "AWS Cedar for Agents". AuthZEN is not mentioned. — [Draft -01](https://www.ietf.org/archive/id/draft-farley-acta-signed-receipts-01.html) A -02 exists. — [-02](https://ftp.sjtu.edu.cn/pub/internet-drafts/draft-farley-acta-signed-receipts-02.txt)
- draft-chueayen-attestation-receipts-02 specifies a receipt that, per a search summary, explicitly does not attest to platform state or decision correctness. — [Draft](https://www.ietf.org/ietf-ftp/internet-drafts/draft-chueayen-attestation-receipts-02.html)
- Others seen only in search summaries: [draft-watts-agent-authority-transition-receipts-00](https://zenodo.org/records/23488392), [draft-hawkins-scitt-attested-agent-payment-00](https://datatracker.ietf.org/doc/html/draft-hawkins-scitt-attested-agent-payment-00), [draft-mih-agent-bilateral-attestation-01](https://datatracker.ietf.org/doc/html/draft-mih-agent-bilateral-attestation-01), [draft-aravind-oauth-decision-subject-00](https://datatracker.ietf.org/doc/html/draft-aravind-oauth-decision-subject-00), [draft-pinto-agent-authz-contestability-00](https://datatracker.ietf.org/doc/html/draft-pinto-agent-authz-contestability-00), [draft-schrock-ep-authorization-receipts-08](https://www.emiliaprotocol.ai/spec), [draft-nennemann-wimse-ect-00](https://datatracker.ietf.org/doc/draft-nennemann-wimse-ect/).

### Inferences
- **Hypothesis H6 and the README claim are stronger than AuthZEN allows.** The README says software that does not understand a proof "ignores it and works as before", and decision 0002 relies on ignore-unknown. But section 5.5 specifically permits a conforming PEP to reject a `true` decision over a context it does not understand. Section 10.1.1 governs unknown fields in general. Section 5.5 governs context on a permit and is the more specific rule. So "no spec change" holds, and "no behavior change" is implementation-dependent. H6 should be tested and worded as "observed on these PEPs". Also, a PEP that ignores the proof gains nothing, so the unmodified PEP case is compatibility and not security.
- **Section 11.6 is the best citation for the project's motivation.** It claims a signature "ensures response accuracy". A signature gives origin and integrity, not accuracy of the evaluation. The README's "a signature says who answered, not that the answer is right" is exactly that gap, and the spec text can be quoted.
- **Section 11.4 limits the claim.** AuthZEN's PDP trusts the PEP for attributes. In zk-pdp, facts in the request remain PEP-supplied unless bound to a commitment or a credential. A proof of evaluation does not fix garbage-in. This is the same point as source integrity in the agent literature below.
- **What zk-pdp reinvents.** Portable decisions that a client carries to a service are the XACML SAML Profile's decision assertion. The only difference is what the receiver relies on: "a trusted 3rd party has evaluated" against "this was evaluated". Cite this in the paper.
- **Potential fit.** The Farley draft's `committed_fields_root` and `policy_digest` are a near-ready place to bind a proof, and it already references Cedar. This is an individual draft and not adopted. Treat it as a neighbor and not a standard.

### Gaps
- I did not find a draft defining a proof of correct evaluation inside an authorization response, in two searches. Most drafts were seen only as summaries, so this is a search result and not a certainty.
- The NIST 800-162 text and the WIMSE draft's content were not read beyond metadata.
- The final 4 percent of the AuthZEN page was not fetched in the first pass. The whole HTML was later downloaded, and the sections quoted were read from it.
- The publication month of RFC 9943 was not visible on the page read.

### BibTeX
```bibtex
% read-full (cited sections)
@misc{authzen2026final, title={Authorization API 1.0}, author={Gazitt, O. and Brossard, D. and Tulshibagwale, A.}, howpublished={OpenID Foundation, Final Specification}, year={2026}, url={https://openid.net/specs/authorization-api-1_0.html}}
% read-abstract (fetched summary)
@misc{coazmcp2026, title={COAZ-MCP: COAZ Binding for the Model Context Protocol - Draft 1}, author={Tulshibagwale, A. and Olivier, A.}, howpublished={OpenID AuthZEN Working Group draft}, year={2026}, url={https://openid.github.io/authzen/authzen-coaz-mcp-binding-1_0}}
% read-abstract (section 9 read)
@misc{xacml3, title={eXtensible Access Control Markup Language (XACML) Version 3.0}, editor={Rissanen, Erik}, howpublished={OASIS Standard}, year={2013}, url={https://docs.oasis-open.org/xacml/3.0/xacml-3.0-core-spec-os-en.html}}
% read-abstract (cited clauses)
@misc{xacmlsaml2, title={XACML SAML Profile Version 2.0}, howpublished={OASIS Committee Specification 02}, year={2014}, url={https://docs.oasis-open.org/xacml/3.0/xacml-profile-saml2.0-v2-spec-en.html}}
% search-summary-only
@techreport{nist800162, title={Guide to Attribute Based Access Control (ABAC) Definition and Considerations}, author={Hu, Vincent and Ferraiolo, David and Kuhn, Richard and Schnitzer, Adam and Sandlin, Kenneth and Miller, Robert and Scarfone, Karen}, institution={NIST}, number={SP 800-162}, year={2014}, doi={10.6028/NIST.SP.800-162}}
% read-abstract (excerpts)
@techreport{nist800207, title={Zero Trust Architecture}, author={Rose, Scott and Borchert, Oliver and Mitchell, Stu and Connelly, Sean}, institution={NIST}, number={SP 800-207}, year={2020}, doi={10.6028/NIST.SP.800-207}}
% read-abstract (excerpts)
@techreport{nccoe2026agent, title={Accelerating the Adoption of Software and AI Agent Identity and Authorization}, author={Booth, Harold and Fisher, Bill and Galluzzo, Ryan and Roberts, Joshua}, institution={NIST NCCoE}, type={Concept paper}, year={2026}, url={https://www.nccoe.nist.gov/sites/default/files/2026-02/accelerating-the-adoption-of-software-and-ai-agent-identity-and-authorization-concept-paper.pdf}}
% read-abstract
@misc{rfc9396, title={OAuth 2.0 Rich Authorization Requests}, author={Lodderstedt, T. and Richer, J. and Campbell, B.}, howpublished={RFC 9396}, year={2023}, url={https://www.rfc-editor.org/rfc/rfc9396.html}}
% read-abstract
@misc{rfc9449, title={OAuth 2.0 Demonstrating Proof of Possession (DPoP)}, author={Fett, D. and Campbell, B. and Bradley, J. and Lodderstedt, T. and Jones, M. and Waite, D.}, howpublished={RFC 9449}, year={2023}, url={https://www.rfc-editor.org/rfc/rfc9449.html}}
% search-summary-only. Datatracker's full title reads as below.
@misc{wimsearch08, title={Workload Identity in a Multi System Environment (WIMSE) Architecture}, author={Salowey, Joseph A. and Rosomakho, Yaroslav and Tschofenig, Hannes}, howpublished={Internet-Draft draft-ietf-wimse-arch-08}, year={2026}, url={https://datatracker.ietf.org/doc/draft-ietf-wimse-arch/}}
% read-abstract (fetch-tool answers over the whole draft; I did not read the text directly)
@misc{farley2026receipts, title={Signed Decision Receipts for Machine-to-Machine Access Control}, author={Farley, Tom}, howpublished={Internet-Draft draft-farley-acta-signed-receipts-01}, year={2026}, url={https://www.ietf.org/archive/id/draft-farley-acta-signed-receipts-01.html}}
```

## Verifiable decision logs and audit: transparency, tamper evidence, policy transparency

### Takeaway
Transparency logs and proofs of evaluation answer different questions and do not compete. A log shows that everyone sees the same append-only history. A proof shows one computation was right, given its inputs. zk-pdp asks the verifier to check against published fingerprints, and it has no account of how a verifier knows those fingerprints are current and unforked. Key transparency solved that problem, and zk-pdp should reuse it.

### Cited Findings
- Digital timestamping as a research problem goes back to 1991. — [Haber and Stornetta](https://doi.org/10.1007/BF00196791)
- Secure audit logs for forensics go back to 1999. — [Schneier and Kelsey](https://doi.org/10.1145/317087.317089)
- Crosby and Wallach study tamper-evident logging, including the case of an untrusted logger. — [USENIX Security 2009](https://www.usenix.org/legacy/event/sec09/tech/full_papers/crosby.pdf)
- Certificate Transparency: [Laurie, CACM 2014](https://doi.org/10.1145/2659897), and CT v2, RFC 9162, which obsoletes RFC 6962. — [RFC 9162](https://www.rfc-editor.org/rfc/rfc9162.txt)
- Key transparency line: [CONIKS](https://www.usenix.org/conference/usenixsecurity15/technical-sessions/presentation/melara), [SEEMless](https://doi.org/10.1145/3319535.3363202), [Merkle²](https://doi.org/10.1109/SP40001.2021.00088), [Parakeet](https://doi.org/10.14722/ndss.2023.24545).
- Append-only authenticated dictionaries: [Tomescu et al., CCS 2019](https://doi.org/10.1145/3319535.3345652). Software signing: [Sigstore](https://doi.org/10.1145/3548606.3560596).
- SCITT's architecture is now RFC 9943, Proposed Standard: registering a signed statement returns a receipt. — [RFC 9943](https://www.rfc-editor.org/info/rfc9943) The main Datatracker page still showed it as RFC Editor in progress when searched, a stale view. — [Datatracker](https://datatracker.ietf.org/doc/draft-ietf-scitt-architecture/)
- A search for a standard combining policy transparency with authorization decision logs found none. The nearest is signed receipts plus a Merkle log, and products such as Traefik Hub. — [Traefik docs](https://doc.traefik.io/traefik-hub/api-gateway/observability/transparency-logs)
- Notarized Agents has the receiving service sign a receipt, encrypted to the owner, published to a witness-cosigned Merkle log. — [Figuera, arXiv 2606.04193](https://arxiv.org/abs/2606.04193)
- In the ZK agent literature, decentralizing verification "is sound only if audit completeness is preserved", for example through signed verification receipts in a shared append-only log. "A compromised relying party could verify, act, and never log." — [Gopalakrishna, arXiv 2608.30083](https://arxiv.org/abs/2608.30083)

### Inferences
- **The gap in zk-pdp.** `docs/spec.md`, `docs/protocol.md` and `docs/threat-model.md` contain no mention of "transparency" or "equivocation". The threat model rotates entity commitments on revocation and uses a nonce or epoch. It does not cover a decider or publisher showing different roots to different verifiers. This is exactly CONIKS's and SEEMless's problem, and policy hashes have it too.
- **Do not reinvent this.** Publish policy hashes and entity roots as SCITT statements or in a CT-style log, and have the verifier require an inclusion proof. Doing so adds a monitored party back, which weakens "needs no trust in the decider" into "needs no trust in the decider, given a witnessed log". Say so.
- **Completeness of a decision log is not something a proof can give.** A proof shows an included decision was correct. It cannot show nothing was omitted. Only a log with an enforcement path can, and the Gopalakrishna quote above makes the same point.
- **Lineage.** Evidence-based audit (Vaughan 2008) put proofs in logs for their content. Transparency logs make logs trustworthy and do not care about content. A zk-pdp audit trail combines the two: a proof log whose contents can be checked without the facts. This combination is what the "someone audits later" use case needs. I found no paper that describes it.

### Gaps
- "Policy transparency" as a term of art did not surface in any peer-reviewed source. It may exist under other names.
- No read of the transparency papers beyond metadata. They are cited for their well-known problem statements, and that is the extent of the claim.
- OPTIKS, Contour and Trillian were searched for but not verified, because DBLP was rate limited, so they are omitted.

### BibTeX
```bibtex
% search-summary-only
@article{haber1991timestamp, title={How to time-stamp a digital document}, author={Haber, Stuart and Stornetta, W. Scott}, journal={Journal of Cryptology}, volume={3}, number={2}, pages={99--111}, year={1991}, doi={10.1007/BF00196791}}
% search-summary-only
@article{schneier1999audit, title={Secure audit logs to support computer forensics}, author={Schneier, Bruce and Kelsey, John}, journal={ACM Transactions on Information and System Security}, volume={2}, number={2}, pages={159--176}, year={1999}, doi={10.1145/317087.317089}}
% read-abstract. Year from the USENIX path (sec09).
@inproceedings{crosby2009tamper, title={Efficient Data Structures for Tamper-Evident Logging}, author={Crosby, Scott A. and Wallach, Dan S.}, booktitle={USENIX Security Symposium}, year={2009}, url={https://www.usenix.org/legacy/event/sec09/tech/full_papers/crosby.pdf}}
% search-summary-only
@article{laurie2014ct, title={Certificate transparency}, author={Laurie, Ben}, journal={Communications of the ACM}, volume={57}, number={10}, pages={40--46}, year={2014}, doi={10.1145/2659897}}
% read-abstract (header)
@misc{rfc9162, title={Certificate Transparency Version 2.0}, author={Laurie, B. and Messeri, E. and Stradling, R.}, howpublished={RFC 9162}, year={2021}, url={https://www.rfc-editor.org/rfc/rfc9162.txt}}
% search-summary-only
@inproceedings{melara2015coniks, title={CONIKS: Bringing Key Transparency to End Users}, author={Melara, Marcela S. and Blankstein, Aaron and Bonneau, Joseph and Felten, Edward W. and Freedman, Michael J.}, booktitle={24th USENIX Security Symposium}, pages={383--398}, year={2015}, url={https://www.usenix.org/conference/usenixsecurity15/technical-sessions/presentation/melara}}
% search-summary-only. Crossref lists the main title only.
@inproceedings{chase2019seemless, title={SEEMless}, author={Chase, Melissa and Deshpande, Apoorvaa and Ghosh, Esha and Malvai, Harjasleen}, booktitle={Proceedings of the 2019 ACM SIGSAC Conference on Computer and Communications Security}, pages={1639--1656}, year={2019}, doi={10.1145/3319535.3363202}}
% search-summary-only
@inproceedings{hu2021merkle2, title={Merkle2: A Low-Latency Transparency Log System}, author={Hu, Yuncong and Hooshmand, Kian and Kalidhindi, Harika and Yang, Seung Jin and Popa, Raluca Ada}, booktitle={2021 IEEE Symposium on Security and Privacy}, pages={285--303}, year={2021}, doi={10.1109/SP40001.2021.00088}}
% search-summary-only
@inproceedings{tomescu2019aad, title={Transparency Logs via Append-Only Authenticated Dictionaries}, author={Tomescu, Alin and Bhupatiraju, Vivek and Papadopoulos, Dimitrios and Papamanthou, Charalampos and Triandopoulos, Nikos and Devadas, Srinivas}, booktitle={Proceedings of the 2019 ACM SIGSAC Conference on Computer and Communications Security}, pages={1299--1316}, year={2019}, doi={10.1145/3319535.3345652}}
% search-summary-only
@inproceedings{malvai2023parakeet, title={Parakeet: Practical Key Transparency for End-to-End Encrypted Messaging}, author={Malvai, Harjasleen and Kokoris-Kogias, Lefteris and Sonnino, Alberto and Ghosh, Esha and Oztürk, Ercan and Lewi, Kevin and Lawlor, Sean}, booktitle={Network and Distributed System Security Symposium}, year={2023}, doi={10.14722/ndss.2023.24545}}
% search-summary-only. Crossref lists the main title only.
@inproceedings{newman2022sigstore, title={Sigstore}, author={Newman, Zachary and Meyers, John Speed and Torres-Arias, Santiago}, booktitle={Proceedings of the 2022 ACM SIGSAC Conference on Computer and Communications Security}, pages={2353--2367}, year={2022}, doi={10.1145/3548606.3560596}}
% read-abstract. Publication month not visible.
@misc{rfc9943, title={An Architecture for Trustworthy and Transparent Digital Supply Chains}, author={Birkholz, H. and Delignat-Lavaud, A. and Fournet, C. and Deshpande, Y. and Lasker, S.}, howpublished={RFC 9943}, year={2026}, url={https://www.rfc-editor.org/info/rfc9943}}
% read-abstract
@misc{figuera2026notarized, title={Notarized Agents: Receiver-Attested Confidential Receipts for AI Agent Actions}, author={Figuera, Juan}, year={2026}, eprint={2606.04193}, archivePrefix={arXiv}}
```

## AI agent authorization: is a ZK proof of policy compliance a named idea yet?

### Takeaway
Yes, and it has several competing names as of November 2026, which makes `paper/related-work.md`'s "likely thin" out of date. None of the agent works I read evaluates a mainstream policy language in the proof. The closest is an Ethereum draft whose reference implementation uses zk-pdp's own stack of Noir and UltraHonk. Production uses Cedar for agent tool calls at a trusted gateway, with no proof of the decision.

### Cited Findings

**Named ZK ideas.**
- **"Cryptographically Verifiable Agent Authorization (CVA)" and "authorization evidence"** (Llambí-Morillas and Fernández-Fernández, July 2026). A relation with BindPrincipal, BindRequest, BindContext and SatisfyPolicy, and a Groth16 proof of concept in Circom. Policies are arithmetic constraints, so the language is bespoke. It reports no constraint counts or timings, leaves reductions as future work, and does not cite PCA or AuthZEN. — [arXiv 2607.21325](https://arxiv.org/html/2607.21325v1)
- CVA states the limit precisely: "a proof may establish that a committed request satisfies an encoded policy but cannot by itself establish that the same request was subsequently executed." — [arXiv 2607.21325](https://arxiv.org/pdf/2607.21325)
- **ERC-8354 "Confidential Agent Policy Verdicts"** (Fatonie, Firdani, Muhammad; Draft, created 2026-07-16). The proving program checks that a private ruleset hashes to a registered commitment, evaluates it to yield the decision, and derives a nullifier. Its reference implementation is a Noir circuit on Barretenberg/UltraHonk. It defines no policy language. It says it does not prove that the interpreter implements the intended policy, and calls a verdict "an integrity property only". It cites neither PCA nor AuthZEN. — [ERC-8354](https://eips.ethereum.org/EIPS/eip-8354)
- **NiyamAI** (Katkar et al., August 2026) proves a small neural "Judge" (11→8→2, 431 constraint rows) with Halo2-KZG through EZKL: 1.7 s to prove, 51 ms to verify, 18.6 KB, on an "Intent Contract" under a SHA-256 commitment. — [arXiv 2608.07167](https://arxiv.org/abs/2608.07167); [PDF](https://arxiv.org/pdf/2608.07167)
- **Zero-Knowledge Proof Gateway** (Gopalakrishna, 30 August 2026) proves a threshold predicate with Bulletproofs in 6.2 ms and verifies in 1.0 ms, with 608 bytes, on MCP and A2A through a proposed `zk-attach/v0` slot. It credits priority to Aegis. — [arXiv 2608.30083](https://arxiv.org/abs/2608.30083)
- **Aegis Protocol** (Adapala and Alugubelly, August 2025) uses a Policy → Arithmetic Circuit → Halo2 pipeline in a discrete-event simulation only, with a 2.79 s median proof latency drawn from it. — [arXiv 2508.19267](https://arxiv.org/abs/2508.19267); [PDF](https://arxiv.org/pdf/2508.19267)
- **zk-MCP** (Jing and Qi, December 2025) proves that messages follow expected formats and rules, with Circom. — [arXiv 2512.14737](https://arxiv.org/abs/2512.14737)
- **Conflict.** Gopalakrishna's table lists Distributed Sentinel (Wu and Gong) as a Groth16 graph-policy ZK system. — [arXiv 2608.30083](https://arxiv.org/abs/2608.30083) But that paper's abstract page mentions no ZK or proof system. — [arXiv 2604.22879](https://arxiv.org/abs/2604.22879) I did not read its body.
- Policy Cards names ZK adherence proofs as a possibility. — [Mavračić, arXiv 2510.24383](https://arxiv.org/pdf/2510.24383) Huang et al. only "explore" ZKPs. — [arXiv 2505.19301](https://arxiv.org/abs/2505.19301)

**Proofs of execution without ZK.**
- Proof of Execution has guarantees of authorization, path compliance and no-effect-on-deny from signatures, hashes and deployment assumptions, adding about 2.7 ms. It states it does not replace consensus, TEEs or zkVMs. — [Rhodes and Kang, arXiv 2607.05397](https://arxiv.org/abs/2607.05397)

**Source integrity.**
- "A range proof binds the statement to the committed value; it cannot by itself show the committed value equals the system-of-record value." Also: "no proof system constrains an endpoint that remains reachable without one." And judgment criteria do not reduce to circuits. — [Gopalakrishna](https://arxiv.org/abs/2608.30083)

**Production and specifications.**
- AWS AgentCore Policy evaluates each MCP tool call against Cedar at a Gateway, which denies by default, and uses Cedar Analysis during authoring and partial evaluation to filter tool lists. The post mentions no signing or proof of decisions. — [Hadarean and Tristan, AWS Security Blog, 20 May 2026](https://aws.amazon.com/blogs/security/why-policy-in-amazon-bedrock-agentcore-chose-cedar-for-securing-agentic-workflows/)
- MCP authorization (revision 2025-11-25) is optional. HTTP servers act as OAuth 2.1 resource servers with RFC 9728 and RFC 8707. STDIO "SHOULD NOT" follow it. It has no signed decisions or proofs. — [MCP spec](https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization)

**Identity and delegation papers.**
- Authenticated delegation extends OAuth 2.0 and OpenID Connect with agent-specific credentials. — [South et al., arXiv 2501.09674](https://arxiv.org/abs/2501.09674)
- Others: [Visibility into AI Agents](https://doi.org/10.1145/3630106.3658948), [IDs for AI Systems](https://arxiv.org/abs/2406.12137), [Infrastructure for AI Agents](https://arxiv.org/abs/2501.10114).
- Otsuka et al. identify five structural gaps, including intent verification and recursive delegation accountability. — [arXiv 2604.23280](https://arxiv.org/abs/2604.23280)

**Enforcement inside the agent's own stack.**
- Progent checks tool calls deterministically against a symbolic policy, with an SMT solver classifying updates. — [Shi et al., arXiv 2504.11703](https://arxiv.org/abs/2504.11703)
- CaMeL uses capabilities with control flow derived from the trusted query. — [Debenedetti et al., arXiv 2503.18813](https://arxiv.org/abs/2503.18813)

### Inferences

| Work | Policy language | What is proved | Measured? | Relation to zk-pdp |
|---|---|---|---|---|
| ERC-8354 | None defined | Ruleset evaluation against committed hash | Reference implementation, no figures read | Same proof stack and same statement shape. Lacks semantics, and admits this. On-chain. |
| CVA | Bespoke circuit | Predicate plus bindings | No figures | Cleanest formal statement. Reuse its binding terms. |
| NiyamAI | None (small neural net) | A classifier's inference | 1.7 s, 51 ms, 18.6 KB | A similar cost envelope to zk-pdp's 2.2 to 2.8 s, 20 ms, 14.7 KB, for a different statement. |
| ZK Proof Gateway | Single predicates | Range or threshold | 6.2 ms, 1.0 ms, 608 B | Far cheaper for the single-predicate case, and honest about source integrity. |
| Aegis | Generic circuits | Unspecified | Simulated | Named first, by that paper's account. |
| Prezta | XACML | Evaluation in RISC Zero | Tens of seconds | Not agent-focused. |

- **The defensible claim for agents.** zk-pdp would be, as far as I found, the first to prove evaluation of a standard policy language with a formal reference semantics for agent tool calls, in the same language as a deployed production system (AgentCore) and through a standard wire binding (AuthZEN COAZ-MCP). Before printing, repeat the search, since this field moved monthly during 2026.
- **ERC-8354's own limitation is zk-pdp's best argument.** It says interpreter fidelity is unproved. Cedar's Lean model and differential testing are what address that gap.
- **The realistic competitor for tool servers is not ZK, it is AgentCore.** A gateway the tool owner already trusts needs no proof. So the zk-pdp agent case holds only when the tool server does not trust the agent's host or gateway, and that case needs to be named concretely.
- **Limits zk-pdp inherits.** A proof says nothing about what the runtime did next (CVA), whether a value came from the system of record (Gopalakrishna), or whether the agent's reasoning was sound. Judgment policies stay out. The tool server must also refuse calls that come without a proof.
- **Vocabulary.** "Authorization evidence" (CVA), "policy verdict" (ERC-8354), "proof of guardrail" and "proof of execution" are all live. "Proof-carrying authorization" is not used in any of these, and none cites it, so zk-pdp can be the work that connects the lineages. That is a real contribution for a related-work section.

### Gaps
- OpenID Foundation's agentic AI whitepaper was not found in the one search run. The claim that it flags recursive delegation is secondhand from [arXiv 2605.22333](https://arxiv.org/pdf/2605.22333), and I give no bibliographic entry.
- "Cedar for Agents" appears only in the Farley draft's appendix. I did not locate the repository.
- ERC-8354's reference circuit was not read, so its performance is unknown.
- The Conseca paper (arXiv 2501.17070) was not fetched.
- I read MCP authorization revision 2025-11-25 and did not check whether a newer revision exists.

### BibTeX
```bibtex
% read-full (fetched HTML and PDF excerpts)
@misc{llambi2026cva, title={Toward cryptographically verifiable authorization for autonomous AI agents: A security hypothesis, preliminary formal model, and proof-of-concept implementation}, author={Llamb{\'\i}-Morillas, M. and Fern{\'a}ndez-Fern{\'a}ndez, D.}, year={2026}, eprint={2607.21325}, archivePrefix={arXiv}}
% read-abstract (fetch-tool answers over the whole page; reference circuit not read)
@misc{erc8354, title={ERC-8354: Confidential Agent Policy Verdicts}, author={Fatonie, Muhammad Zidan and Firdani, Faisal and Muhammad, Maulana Asykari}, howpublished={Ethereum Improvement Proposals, Draft}, year={2026}, url={https://eips.ethereum.org/EIPS/eip-8354}}
% read-abstract, plus excerpts
@misc{katkar2026niyamai, title={NiyamAI - An Intent-Bound AI Agent with Cryptographically Verifiable Guardrails using Zero-Knowledge Proofs}, author={Katkar, Aditya and Karkele, Om and Mandhane, Kartik and More, Manisha and Kashid, Yash}, year={2026}, eprint={2608.07167}, archivePrefix={arXiv}}
% read-full (cited sections)
@misc{gopalakrishna2026zkpredicate, title={Zero-Knowledge Predicate Proofs Between AI Agents: A Measured, Cross-Protocol Gateway and the Source-Integrity Gap}, author={Gopalakrishna, Ashok Subbabhatta}, year={2026}, eprint={2608.30083}, archivePrefix={arXiv}, doi={10.48550/arXiv.2608.30083}}
% read-abstract, plus excerpts
@misc{adapala2025aegis, title={The Aegis Protocol: A Foundational Security Framework for Autonomous AI Agents}, author={Adapala, Sai Teja Reddy and Alugubelly, Yashwanth Reddy}, year={2025}, eprint={2508.19267}, archivePrefix={arXiv}}
% read-abstract
@misc{jing2025zkmcp, title={Zero-Knowledge Audit for Internet of Agents: Privacy-Preserving Communication Verification with Model Context Protocol}, author={Jing, Guanlin and Qi, Huayi}, year={2025}, eprint={2512.14737}, archivePrefix={arXiv}}
% read-abstract
@misc{wu2026sentinel, title={Beyond Single-Agent Alignment: Preventing Context-Fragmented Violations in Multi-Agent Systems}, author={Wu, Jie and Gong, Ming}, year={2026}, eprint={2604.22879}, archivePrefix={arXiv}}
% read-abstract
@misc{rhodes2026poe, title={Proof of Execution: Runtime Verification for Governed AI Agent Actions}, author={Rhodes, James and Kang, George}, year={2026}, eprint={2607.05397}, archivePrefix={arXiv}}
% read-abstract (Prezta; authors corrected from the local note's "and others")
@inproceedings{wei2026prezta, title={Prezta: Provable Remote Execution of Zero-Trust Authorization using SNARKs}, author={Wei, Zhongjing and Ameer, Osaid Muhammad and Borisov, Nikita and Zhang, Yupeng}, booktitle={USENIX Security Symposium}, year={2026}, eprint={2607.11466}, archivePrefix={arXiv}}
% search-summary-only
@misc{mavracic2025policycards, title={Policy Cards: Machine-Readable Runtime Governance for Autonomous AI Agents}, author={Mavra{\v{c}}i{\'c}, Juraj}, year={2025}, eprint={2510.24383}, archivePrefix={arXiv}}
% read-abstract
@misc{huang2025zerotrust, title={A Novel Zero-Trust Identity Framework for Agentic AI: Decentralized Authentication and Fine-Grained Access Control}, author={Huang, Ken and Narajala, Vineeth Sai and Yeoh, John and Ross, Jason and Raskar, Ramesh and Harkati, Youssef and Huang, Jerry and Habler, Idan and Hughes, Chris}, year={2025}, eprint={2505.19301}, archivePrefix={arXiv}}
% read-abstract
@misc{south2025delegation, title={Authenticated Delegation and Authorized AI Agents}, author={South, Tobin and Marro, Samuele and Hardjono, Thomas and Mahari, Robert and Whitney, Cedric Deslandes and Greenwood, Dazza and Chan, Alan and Pentland, Alex}, year={2025}, eprint={2501.09674}, archivePrefix={arXiv}}
% search-summary-only
@inproceedings{chan2024visibility, title={Visibility into AI Agents}, author={Chan, Alan and Ezell, Carson and Kaufmann, Max and Wei, Kevin and Hammond, Lewis and Bradley, Herbie and Bluemke, Emma and Rajkumar, Nitarshan and Krueger, David and Kolt, Noam and Heim, Lennart and Anderljung, Markus}, booktitle={The 2024 ACM Conference on Fairness, Accountability, and Transparency}, pages={958--973}, year={2024}, doi={10.1145/3630106.3658948}}
% search-summary-only
@misc{chan2024ids, title={IDs for AI Systems}, author={Chan, Alan and Kolt, Noam and Wills, Peter and Anwar, Usman and Schroeder de Witt, Christian and Rajkumar, Nitarshan and Hammond, Lewis and Krueger, David and Heim, Lennart and Anderljung, Markus}, year={2024}, eprint={2406.12137}, archivePrefix={arXiv}}
% search-summary-only
@misc{chan2025infrastructure, title={Infrastructure for AI Agents}, author={Chan, Alan and Wei, Kevin and Huang, Sihao and Rajkumar, Nitarshan and Perrier, Elija and Lazar, Seth and Hadfield, Gillian K. and Anderljung, Markus}, year={2025}, eprint={2501.10114}, archivePrefix={arXiv}}
% read-abstract
@misc{otsuka2026aiidentity, title={AI Identity: Standards, Gaps, and Research Directions for AI Agents}, author={Otsuka, Takumi and Toyoda, Kentaroh and Leung, Alex}, year={2026}, eprint={2604.23280}, archivePrefix={arXiv}}
% read-abstract
@misc{shi2025progent, title={Progent: Securing AI Agents with Privilege Control}, author={Shi, Tianneng and He, Jingxuan and Wang, Zhun and Li, Hongwei and Wu, Linyu and Guo, Wenbo and Song, Dawn}, year={2025}, eprint={2504.11703}, archivePrefix={arXiv}}
% read-abstract
@misc{debenedetti2025camel, title={Defeating Prompt Injections by Design}, author={Debenedetti, Edoardo and Shumailov, Ilia and Fan, Tianqi and Hayes, Jamie and Carlini, Nicholas and Fabian, Daniel and Kern, Christoph and Shi, Chongyang and Terzis, Andreas and Tram{\`e}r, Florian}, year={2025}, eprint={2503.18813}, archivePrefix={arXiv}}
% read-abstract
@inproceedings{zhang2025polaris, title={POLARIS: Cross-Domain Access Control via Verifiable Identity and Policy-Based Authorization}, author={Zhang, Aiyao and Lee, Xiaodong and Zhuang, Zhixian and Wei, Jiuqi and Fu, Yufan and Peng, Botao}, booktitle={Proc. 2025 IEEE TrustCom}, pages={1489--1498}, year={2025}, eprint={2511.22017}, archivePrefix={arXiv}}
```

## Attested alternatives: TEE-based PDPs and remote attestation as the main competitor

### Takeaway
A TEE-hosted PDP is cheaper, runs unmodified Cedar, keeps inputs from the host, and can bind to a data source. It loses in one place, which is the setting zk-pdp names: a machine in someone else's building. Since 2025, attestation keys have been extracted with cheap physical interposers, and vendors place this out of scope. I found no paper describing a TEE-hosted PDP by that name, so the comparison is mine.

### Cited Findings
- RATS is the IETF attestation architecture. — [RFC 9334](https://www.rfc-editor.org/rfc/rfc9334.txt)
- SGX attestation depends on a vendor quoting mechanism, and on an Intel service or DCAP for verification. — [TEE key-value survey, arXiv 2501.03118, via search summary](https://arxiv.org/pdf/2501.03118); [SoK, arXiv 2205.12742, via search summary](https://arxiv.org/pdf/2205.12742)
- **Proof-of-Guardrail** (Jin et al., March 2026) runs a guardrail and agent in AWS Nitro Enclaves. Attestation generation takes about 98 ms and verification about 5 ms. Latency rises 25 to 38 percent, and cost is about 18.5× a t3.micro. "A valid proof establishes guardrail execution, but not the actual agent safety." It relies on AWS's Nitro hypervisor as trust anchor, and calls TEEs "a computationally efficient alternative to prohibitive zero-knowledge proofs", without benchmarking ZK. — [arXiv 2603.05786](https://arxiv.org/html/2603.05786v1)
- Confidential Attestation (CAT) verifies policy compliance inside an SGX enclave using an untrusted generator and a small trusted verifier. This is adjacent to a PDP but is about program compliance. — [Liu et al., arXiv 2007.10513](https://arxiv.org/abs/2007.10513)
- Barbican applies policies based on enclave measurement or signer after attestation, which is a PDP-style gate in the other direction. — [arXiv 1712.07694, via search summary](https://arxiv.org/pdf/1712.07694)
- Attestable Audits run AI safety benchmarks in TEEs. — [Schnabl et al., arXiv 2506.23706](https://arxiv.org/abs/2506.23706)
- A survey argues TEE attestation shows what code ran but cannot prove the code meets a trust property without revealing it, while ZK proofs of a safety property do not establish that the agent you talk to runs that code. — [Allen et al., arXiv 2606.13892, via search summary](https://arxiv.org/pdf/2606.13892)

**Attacks.**
- Foreshadow extracted keys from SGX by transient execution. — [Van Bulck et al., USENIX Security 2018](https://www.usenix.org/conference/usenixsecurity18/presentation/bulck)
- WireTap built a DRAM bus interposer from easily bought equipment and "extract[ed] an SGX secret attestation key from a machine in fully trusted status". — [Seto et al., CCS 2025](https://wiretap.fail/)
- Battering RAM is a roughly $50 interposer that bypasses memory-encryption defenses on Intel and AMD cloud processors. — [De Meulemeester et al., S&P 2026](https://batteringram.eu/)
- TEE.fail extracts ECDSA attestation keys from Intel's Provisioning Certification Enclave, "allowing us to break SGX and TDX attestation", "from a single signing operation", and forges a TDX quote. — [Chuang et al., S&P 2026](https://tee.fail/)
- Press: AMD and Intel treat physical attacks as out of scope. Secondary source, not read from the vendors. — [The Hacker News](https://thehackernews.com/2025/10/new-teefail-side-channel-attack.html)
- The Battering RAM site also lists DDRop (CCS '26), dropping DDR5 writes against TDX and SEV-SNP. I did not read it. — [Battering RAM site](https://batteringram.eu/)

**Hybrids.**
- The ZK Proof Gateway fuses enclave attestation and proof so that verifying one artifact certifies both the predicate and that the value was read by a measured binary. It is tested only against a mock authority. — [Gopalakrishna](https://arxiv.org/abs/2608.30083)
- CVA also says execution binding needs trusted execution evidence such as attestation. — [arXiv 2607.21325](https://arxiv.org/html/2607.21325v1)
- Logical attestation combined attestation statements and logic in 2011. — [Sirer et al.](https://doi.org/10.1145/2043556.2043580)

### Inferences

Figures below are from different systems and machines, so read them as rough orders only.

| Criterion | TEE PDP | zk-pdp circuit backend |
|---|---|---|
| Decide and produce evidence | About 100 ms (attestation), plus Cedar's microseconds | 2.2 to 2.8 s native, 4 to 7 s in a browser |
| Verify | About 5 ms | About 20 ms |
| Language coverage | All of Cedar, unchanged | 3 of 7 sample policy sets |
| Inputs hidden from the host | Yes, if the TEE holds | No. The prover holds the witness. |
| Inputs hidden from the verifier | Yes | Only in true ZK mode |
| Can bind to a source of record | Yes, with attested TLS or similar | Only through a signature or commitment |
| Runs on a client's own device | Rarely | Yes, measured in the browser |
| Host has physical access | Broken, per WireTap and TEE.fail | Unaffected. Soundness has no hardware assumption. |
| Key compromise | One extracted attestation key forges quotes, with vendor-level recovery | No such key. A trusted setup, where used, and bugs are the analogues. |
| Root of trust | Hardware vendor | Cryptographic assumptions, plus evaluator and circuit integrity |

- **Where each wins.** For a vendor-hosted PDP in a professionally run data center, a TEE is the honest default, and the paper should say so. zk-pdp wins when the decider controls the hardware (client device, agent host, someone else's building), when no suitable TEE exists (browser, phone), or when vendor trust is unacceptable. Proof-of-Guardrail's wording of "prohibitive" ZK is also out of date for this narrow statement, given zk-pdp's own timings. Its claim concerned inference, not policy evaluation.
- **What ZK does not give that TEEs do.** Provers see their inputs, so a vendor-run prover does not hide them from the vendor. Neither mechanism solves "was this the real data": TEEs can read the source directly, and ZK needs a signature or commitment. A hybrid is defensible only with a clear account of which party each mechanism constrains.
- **zk-pdp's threat model should list the TEE as a named alternative.** It is what a reviewer will raise first.

### Gaps
- No paper on an SGX, TDX, SEV or Nitro hosted XACML, Cedar or OPA PDP was found in two searches. This is a real finding, but it is not proof that none exists.
- CCF was searched but not verified or read, so it is omitted.
- Vendor positions rest on press. The attack claims are from the authors' sites, not the full papers.
- ZK soundness failures in practice are out of this note's scope and belong in the proof-systems notes. The table's "unaffected" refers only to physical attacks.

### BibTeX
```bibtex
% read-abstract (header)
@misc{rfc9334, title={Remote ATtestation procedureS (RATS) Architecture}, author={Birkholz, H. and Thaler, D. and Richardson, M. and Smith, N. and Pan, W.}, howpublished={RFC 9334}, year={2023}, url={https://www.rfc-editor.org/rfc/rfc9334.txt}}
% read-abstract (fetch-tool answers over the whole HTML, plus PDF excerpts)
@misc{jin2026guardrail, title={Proof-of-Guardrail in AI Agents and What (Not) to Trust from It}, author={Jin, Xisen and Duan, Michael and Lin, Qin and Chan, Aaron and Chen, Zhenglun and Du, Junyi and Ren, Xiang}, year={2026}, eprint={2603.05786}, archivePrefix={arXiv}}
% read-abstract. Author list from the page; the page's own count differs by one.
@misc{liu2020cat, title={Confidential Attestation: Efficient in-Enclave Verification of Privacy Policy Compliance}, author={Liu, Weijie and Wang, Wenhao and Wang, Xiaofeng and Meng, Xiaozhu and Lu, Yaosong and Chen, Hongbo and Wang, Xinyu and Shen, Qingtao and Chen, Kai and Tang, Haixu and Chen, Yi and Xing, Luyi}, year={2020}, eprint={2007.10513}, archivePrefix={arXiv}}
% read-abstract
@misc{schnabl2025attestable, title={Attestable Audits: Verifiable AI Safety Benchmarks Using Trusted Execution Environments}, author={Schnabl, Christoph and Hugenroth, Daniel and Marino, Bill and Beresford, Alastair R.}, year={2025}, eprint={2506.23706}, archivePrefix={arXiv}}
% search-summary-only
@misc{allen2026cryptoai, title={Crypto x AI, AI x Crypto: A Survey}, author={Allen, Sarah and Anchuri, Pranay and Austgen, James and Bahrani, Maryam and Breckenridge, Samuel and Buchwald, Aaron and Cachin, Christian and F{\'a}brega, Andr{\'e}s and Fernandez, Jared and Chiang, James Hsin-yu and Mouallem, Marwa and Bar-Zur, Roi and DeSilva, Neil and Eyal, Ittay and Fanti, Giulia and Juels, Ari and Miller, Andrew and Sillaber, Christian and Vilardell, Dani and Viswanath, Pramod and Wang, Wenhao and Weinberg, Matt and Yang, Sen and Yao, Jianzhu and Zhang, Fan}, year={2026}, eprint={2606.13892}, archivePrefix={arXiv}}
% search-summary-only
@inproceedings{vanbulck2018foreshadow, title={Foreshadow: Extracting the Keys to the Intel SGX Kingdom with Transient Out-of-Order Execution}, author={Van Bulck, Jo and Minkin, Marina and Weisse, Ofir and Genkin, Daniel and Kasikci, Baris and Piessens, Frank and Silberstein, Mark and Wenisch, Thomas F. and Yarom, Yuval and Strackx, Raoul}, booktitle={27th USENIX Security Symposium}, pages={991--1008}, year={2018}, url={https://www.usenix.org/conference/usenixsecurity18/presentation/bulck}}
% read-abstract (authors' site). BibTeX from the site.
@inproceedings{seto2025wiretap, title={WireTap: Breaking Server SGX via DRAM Bus Interposition}, author={Seto, Alex and Duran, Oytun Kuday and Amer, Samy and Chuang, Jalen and van Schaik, Stephan and Genkin, Daniel and Garman, Christina}, booktitle={2025 SIGSAC Conference on Computer and Communications Security}, year={2025}, url={https://wiretap.fail}}
% read-abstract (authors' site). BibTeX from the site.
@inproceedings{chuang2026teefail, title={TEE.fail: Breaking Trusted Execution Environments via DDR5 Memory Bus Interposition}, author={Chuang, Jalen and Seto, Alex and Berrios, Nicolas and van Schaik, Stephan and Garman, Christina and Genkin, Daniel}, booktitle={47th IEEE Symposium on Security and Privacy}, year={2026}, url={https://tee.fail}}
% read-abstract (authors' site). BibTeX from the site.
@inproceedings{demeulemeester2026battering, title={Battering RAM: Low-Cost Interposer Attacks on Confidential Computing via Dynamic Memory Aliasing}, author={De Meulemeester, Jesse and Oswald, David and Verbauwhede, Ingrid and Van Bulck, Jo}, booktitle={47th IEEE Symposium on Security and Privacy}, year={2026}, url={https://batteringram.eu/}}
```
