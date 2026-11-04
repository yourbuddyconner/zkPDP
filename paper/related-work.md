# Related work

A one-hour search on 2026-11-04 seeded this list. It is not a systematic
review. Phase 0 includes a proper pass. Each entry names its source note.

## Zero-knowledge proofs for access control

- Prezta (S01). XACML in RISC Zero for edge devices. The closest work. We differ in language, in policy as data, in the circuit backend, in the comparison, in composition, and in the AuthZEN envelope.
- POLARIS (S18). Cross-domain access control with verifiable identity. Not yet read.
- Health data policy enforcement with zero-knowledge proofs on a blockchain (S19). Private satisfaction of an access policy. Not yet read.
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

- To find: SD-JWT, BBS signatures, anonymous credentials, verifiable credentials with zero-knowledge presentations. Our private mode overlaps with these for simple predicates and extends them to arbitrary Cedar policies.

## Authorization standards and engines

- AuthZEN (S02, S22, S23). XACML. OPA and Rego. Zanzibar and OpenFGA.
- To find: prior proposals for signed or attested PDP responses.

## Agents under policy

- To find: work on AI agents proving compliance with operator policy to tool servers. Likely thin. If so, that is a framing opportunity, not a related-work gap.
