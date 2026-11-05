# Threat model

## Assets

- The decision: a permit or deny for one request.
- The policy set and its hash.
- The entity data: attributes and parent relations.
- In the private mode, the attributes themselves.
- The evaluator identity: image ID or verification key.

## Parties and trust

| Party | Trusted for | Not trusted for |
|---|---|---|
| Policy authority | Writing the policy set, publishing its hash and schema | Nothing else |
| Attribute authority | The truth of entity data it commits to or signs | Nothing else |
| Decider | Nothing | Everything |
| Verifier | Enforcing what it verifies, holding the pinned identities | Seeing private attributes in the private mode |
| Auditor | Re-checking stored responses | Nothing else |

The decider is the adversary in the integrity model. The verifier is the
adversary in the privacy model.

## Adversary goals, integrity

1. Obtain a permit for a request the policy set denies.
2. Replay a past permit for a new request or after revocation.
3. Fabricate or alter attributes or parent relations.
4. Substitute a different policy set, schema or evaluator.
5. Make the verifier accept a proof about a different request than the one it will enforce.

## Adversary goals, privacy

6. Learn attribute values from the proof, the journal or timing.
7. Learn which policies fired, and from that infer attributes.

## What stops each goal

| Goal | Control |
|---|---|
| 1 | Soundness of the proof system. Correctness of the evaluator (Cedar's Lean model and differential tests) or of the compiler (a front end with a verified Lean model, differential tests of the back end). Differential tests use honest witnesses and do not find missing constraints. |
| 2 | Verifier nonce or epoch in the request context, echoed in the journal. Entity commitment or epoch rotated on revocation. |
| 3 | Input provenance. Mode 1: Merkle inclusion against a root the verifier owns. Mode 2 and 3: signature of the attribute authority verified inside the proof. The transitive closure is computed by the authority, never by the decider. |
| 4 | Verifier pins the evaluator identity and allowlists policy hashes, or accepts a compilation proof from a pinned compiler image. Schema hash in the journal. |
| 5 | Request hash in the journal over the canonical request the verifier will enforce. |
| 6 | Proof system must be zero-knowledge, not only succinct. Private inputs never appear in the journal. Prover-side timing is out of scope, except that RISC Zero's precompiles are documented as not constant time, which we record and avoid for private data. |
| 7 | The `reasons` field is off in the private mode. The paper demonstrates the leak on the corpus so nobody turns it on by accident. |

## Assumptions

- The proof system is sound and, where claimed, zero-knowledge, under its published assumptions. RISC Zero's STARK has an open advisory on the zero-knowledge property. Groth16 needs a trusted setup. We state which applies to each measured configuration.
- The verifier obtains the evaluator identity and policy hashes over a channel the adversary does not control. How is out of scope.
- The attribute authority's keys are not compromised.
- Cedar's evaluator matches its Lean model for the inputs we use. We rely on Cedar's differential testing and run our own against the native evaluator.

## Known gaps raised by the literature review

Named here so they are not lost. None is addressed yet.

- Adversarial-witness tests of generated circuits.
- Equivocation: a publisher showing different roots to different verifiers, and a proof over a stale root. CONIKS, SEEMless and SCITT address the first. Zanzibar's "new enemy" problem is the second.
- A trusted execution environment as the main alternative, and where it fails.
- Source integrity and execution binding: a sound proof can still authorize the wrong thing.
- A decision as an oracle: a verifier that can choose policies learns one bit per query.

## Out of scope

- Availability. A decider can refuse to prove.
- The policy being wrong. A proof shows the policy was applied, not that the policy is right.
- Side channels on the decider's device beyond the precompile note above.
- Compromise of the verifier.
- Key distribution and rotation for authorities.

## Known leaks to document

- Proof size and proving time can vary with input. Noted, not mitigated.
- The `reasons` list.
- In mode 1, the Merkle path reveals the position of entities in the store. The store layout should not encode anything sensitive.
