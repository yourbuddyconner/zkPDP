# Related work

The full review is `docs/literature/reports/ZK policy evaluation prior art.md`. It supersedes this seed list where they differ.

A one-hour search on 2026-11-04 seeded this list. It is not a systematic
review. Phase 0 includes a proper pass. Each entry names its source note.

## Zero-knowledge proofs for access control

- Prezta (S01). XACML in RISC Zero for edge devices. The closest work. We differ in language, in policy as data, in the circuit backend, in the comparison, in composition, and in the AuthZEN envelope.
- Newton, Warrant and ERC-8354: Rego in SP1, an unbuilt proposal naming Cedar in a guest, and a Noir verdict envelope. See the literature review.
- ZK-EHR (S19). Do not cite as working enforcement or as a policy compiler: the measured prototype is a 5-constraint fixed check, and the published contract appears not to read the granted output.
- Policy compilers to circuits: Cinderella, ZK-TPL, zkGen, Circuitree. Proof-carrying authorization is the direct ancestor: Appel and Felten, Bauer, SD3, Evidence-Based Audit.
- POLARIS (S18) uses salted-hash disclosure and no ZK. It belongs with verifiable credentials, not here.
- To find: zero-knowledge attribute-based access control, anonymous credentials with policy predicates, SNARK-based policy compliance.

## Verified and analyzable authorization

- Cedar (S03, S04). The language, the Lean model, differential testing.
- Cedar symbolic analysis (S05, S06, S07). The Term IR we build on.
- To find: verified XACML or Rego semantics, if any.

## General-purpose proof systems

- RISC Zero (S08, S09, S10, S21). SP1 and others (S20, as vendor claims only).
- Noir and Barretenberg UltraHonk (S14, S15, S17).
- To find: the zero-knowledge advisory's underlying research by Habock and Kindi. Surveys of zkVM frameworks.

## Client-side proving

- Mopro (S14). P-256 benchmark (S15). Client-side proving report (S16).
- To find: zkPassport, zkEmail and the JWT-in-Noir libraries as examples of credential proofs we can compose with.

## Credentials with proofs

- To find: SD-JWT, BBS signatures, anonymous credentials, verifiable credentials with zero-knowledge presentations. Our private mode overlaps with these for simple predicates and aims at policies written in Cedar. Coverage today is 3 of 7 upstream sets, and the mode lacks unlinkability, device binding and revocation. S29 states the problem.

## Authorization standards and engines

- AuthZEN (S02, S22, S23). XACML. OPA and Rego. Zanzibar and OpenFGA.
- To find: prior proposals for signed or attested PDP responses.

## Agents under policy

- Already populated: ERC-8354, CVA, NiyamAI, a ZK Proof Gateway and Aegis. On what was read, none proves a mainstream policy language and none cites proof-carrying authorization.
