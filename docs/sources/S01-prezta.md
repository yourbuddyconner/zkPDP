# S01: Prezta

Wei, Zhongjing and others. "Prezta: Provable Remote Execution of Zero-Trust
Authorization using SNARKs." USENIX Security 2026. arXiv 2607.11466.
URLs: https://arxiv.org/abs/2607.11466 and
https://www.usenix.org/system/files/usenixsecurity26-wei-zhongjing.pdf
Accessed 2026-11-04. Trust: primary, abstract and summary only. Full paper not
yet read. Phase 0 task.

## What it claims

Operational technology edge devices cannot run zero-trust policy themselves,
and gateways are a burden. Prezta has the client evaluate XACML 3.0 policy
inside the RISC Zero zkVM and hand the device a succinct proof. The device
verifies against a committed image ID stored at policy deployment. Policies
compile from XACML to Rust to RISC-V. Optimizations: compiling policies to
Rust, precompiling regular expressions, optimizing signature verification and
JWT parsing, giving more than an order of magnitude speedup. Proving in tens
of seconds on a desktop. Verification in tens of milliseconds. The compiler
passes 83 percent of the XACML conformance suite. The authors note their
proofs are succinct, not zero-knowledge in the strict sense.

## What we take

- The direct precedent and comparator. Same zkVM. Different language.
- The finding that input provenance, JWT parsing and signature verification, dominated cost. Our H3.
- The policy-as-image design, which we contrast with policy as data (decision 0007).
- The honest succinct-not-zero-knowledge note, which we adopt.

## Open items

- Read the full paper. Record proof sizes, exact hardware, and whether an artifact exists.
- Confirm the arXiv identifier resolves to this paper. It did on 2026-11-04.

## Update 2026-11-04

Read through the arXiv v2 body by the literature review, appendices not closely. Authors in USENIX order: Wei, Ameer, Zhang, Borisov. 83 percent is 323 of 389 testable conformance cases. Proving about 14 or 28 s, proofs 238 to 250 KB, verifying 14 to 15.5 ms on a 16-vCPU machine. On its heaviest policy RSA was 45 percent and regex 47 percent of user cycles, so "signatures dominate" was too simple. It makes no zero-knowledge claim and has no committed fact store. Artifact: doi:10.5281/zenodo.20303462.
