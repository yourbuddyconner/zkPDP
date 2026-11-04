# Design: proof-carrying authorization decisions

Date: 2026-11-04. Status: approved for planning by the human researcher on
2026-11-04 ("ship it"). Changes after this date get a decision record.

## 1. Intent

Today an authorization decision is trusted because of who made it. A resource
server calls a policy decision point it trusts, over a channel it trusts, and
acts on the answer. The trust is in the operator, the key and the network.

We want a decision that is trusted because of what was computed. The decider
produces a proof that a known evaluator ran a known policy set over
authenticated inputs and reached this decision. The verifier checks the proof
in milliseconds and needs to trust nothing about the decider.

That changes who may decide. The decider can be a client, an AI agent's
runtime, a decision service run by a vendor the resource owner does not trust,
or a device that cannot reach any server. It also changes what the verifier
learns. In the private mode, the verifier learns the decision and nothing about
the attributes that produced it.

The thesis, in one sentence: a policy decision can be moved to a party the
verifier does not trust, at a cost of seconds of proving and milliseconds of
verification, inside the standard AuthZEN wire format, with no change to
enforcement points that do not understand proofs.

## 2. Why this and why now

- Cedar has a Lean-verified evaluator, a validator that removes runtime errors from validated policies, and a verified symbolic compiler with a typed intermediate representation. No other mainstream policy language offers all three. They make both of our backends possible.
- AuthZEN 1.0 became a Final specification in January 2026. Its response has an open `context` object and a rule that receivers ignore unknown fields. A proof fits without a spec change.
- General zkVMs now run unmodified Rust. Prezta, at USENIX Security 2026, proved XACML decisions in RISC Zero. That is the direct precedent and the direct comparator.
- Client-side proving in Noir is now measured in seconds on laptops and phones.
- AI agents acting under operator policy are a live problem. An agent that can prove its action is within policy, to a tool server that does not trust the agent's host, is a concrete use of this work.

## 3. Goals

1. Define one proof-carrying AuthZEN response and verification procedure that both backends produce and one verifier checks.
2. Run the unmodified Cedar evaluator inside a zkVM and measure it.
3. Compile validated Cedar policy sets to circuits through Cedar's own Term IR and measure them.
4. Measure the cost of input provenance in both backends: a committed entity store and signed credentials.
5. Compare the two backends on one corpus, one machine set and one rule of equal effort.
6. Show composition: proofs bound at the verifier through shared commitments, and a zkVM proof that a policy hash compiles to a circuit key.
7. Say plainly what is zero-knowledge and what is only succinct, and what the journal leaks.
8. Demonstrate backward compatibility with an unmodified AuthZEN enforcement point.

## 4. Non-goals

- A new policy language or a new proof system.
- Replacing the decision point in deployments where it is already trusted. A signature is cheaper there and we say so.
- The AuthZEN search endpoints. They enumerate subjects and resources and do not fit a proof.
- Production hardening, key management products, or a hosted service.
- Formal verification of our own code. We reuse verified components and test differentially against them.

## 5. Anti-goals

Outcomes that would make this a failure even if everything runs:

- Numbers that cannot be reproduced from the repository.
- A "zero-knowledge" claim where only succinctness holds.
- A comparison that favors one backend because it got more engineering hours.
- A policy subset for the circuit backend chosen to make the coverage number look good.
- Product names coined for the backends or the project.

## 6. Parties

- **Policy authority** writes the policy set and schema, validates them, and publishes the policy hash.
- **Attribute authority** vouches for entity data. Either it publishes an entity commitment or it signs credentials.
- **Decider** holds the inputs, runs the evaluator inside the proof system, and returns an AuthZEN response with a proof.
- **Verifier** holds the evaluator identity and the accepted policy hashes, checks the proof, and enforces the decision.
- **Auditor** (optional) re-checks stored responses later without running anything.

The decider is untrusted. The threat model is in `threat-model.md`.

## 7. Architecture

### 7.1 Shared layer

Both backends sit under one envelope, specified in `protocol.md`.

- The request is an AuthZEN access evaluation request. The verifier may add a nonce to its `context`.
- The response is an AuthZEN decision. The proof sits in `context.proof`.
- The journal, the proof's public output, commits to: the request hash, the policy hash, the schema hash, the entity commitment, the nonce, the decision, and optionally the IDs of the policies that fired.
- Hashes use SHA-256 over canonical bytes. JSON is canonicalized with JCS (RFC 8785). The policy set is canonicalized by Cedar's formatter.
- The verifier pins an evaluator identity: a RISC Zero image ID or a circuit verification key hash.

### 7.2 Backend A: Cedar in a zkVM

The guest program is the Cedar crate plus a thin shell. The policy set, schema, entity store and request are inputs, so the image ID never changes when policy changes. The guest parses the inputs, runs the authorizer, hashes what the journal needs, and commits the journal.

First zkVM: RISC Zero. The guest is plain Rust and should port to SP1 if a second data point is wanted. Receipts come in three sizes: composite, succinct, and a Groth16 wrap. We measure all three and let the use case pick.

What we expect to find: deserialization and transitive closure of entities cost more cycles than evaluation. If so, the protobuf input form and a pre-closed store are the first optimizations. Prezta found signature verification over inputs dominated everything else. We expect the same.

### 7.3 Backend B: Cedar compiled to a circuit

A validated Cedar policy set is static: a known number of policies, a type for every expression, and a guarantee of no runtime error on schema-conforming entities. That makes a policy set a fixed boolean formula, which is what a circuit wants.

Front end: the `cedar-policy-symcc` crate, whose Term IR is modeled in Lean. Back end: a translation from Term IR to Noir, written by us and tested differentially against the Rust evaluator on the corpus, which is how Cedar itself is tested. Phase 2 translates one policy set by hand to learn the shape and the gate count before any compiler is written.

Design choices the circuit forces:

- Strings are fixed-length byte arrays with a per-schema maximum. Equality with a literal compares hashes. The `like` operator is a bounded glob matcher.
- The entity hierarchy is closed outside the circuit by the attribute authority. The store is committed as a Merkle root. `in` becomes one membership proof against that root.
- Sets have a per-schema maximum size.
- Optional attributes carry a presence flag. Extension types map to fixed-width integers and bit operations.
- A policy change produces a new circuit and a new verification key. The verifier allowlists keys, or accepts a compilation proof (7.5).

Proof system: Noir with Barretenberg's UltraHonk, which has a zero-knowledge mode and proves in the browser and natively on phones.

### 7.4 Input provenance

A proof that the evaluator said permit over some entities is worthless if the decider wrote the entities. Three modes, in rising cost, each measured in both backends:

1. **Verifier-owned store.** The verifier publishes a Merkle root of its own entity store. The decider supplies inclusion paths. Cheap. Only works when the verifier owns the data.
2. **Signed credentials.** Attributes arrive as credentials signed by the attribute authority. The proof verifies the signature inside. Ed25519 first. JWT and SD-JWT later. RSA last, because it is the expensive case.
3. **Private attributes.** Mode 2 with the attributes hidden. The journal reveals the decision and the issuer, not the attributes. This mode needs a proof system that is zero-knowledge, not only succinct, and it needs the `reasons` field off.

### 7.5 Composition

Not proof-in-proof. Verifying one proof system inside another costs millions of cycles or gates and buys little here. Two cheaper compositions:

- **At the verifier.** Two proofs expose the same commitment, for example the hash of the attributes. The verifier checks each proof and checks the commitments match. A credential proof in Noir can bind to a policy proof in either backend.
- **Compilation proof.** The weak point of the circuit backend is that a policy change yields a new verification key the verifier must trust. Run the Cedar-to-circuit compiler inside RISC Zero once per policy change. The journal states that policy hash H compiled to verification key K. Compilation is rare and offline, so a slow proof is fine. The verifier then accepts any K that arrives with a compilation proof, and policy is data again.

### 7.6 Freshness and revocation

A permit proof must not be replayable. The verifier puts a nonce in the request context, or publishes an epoch. The journal echoes it. Revocation is an epoch root the verifier rotates, or a new entity commitment.

### 7.7 Batch

The AuthZEN batch endpoint returns one decision per item. The zkVM backend proves N decisions in one receipt at linear cost. The circuit backend needs a fixed-width variant or recursion. We note the asymmetry and measure it if time allows.

## 8. Where this earns its cost

- The verifier does not trust the decider: a client, an agent runtime, a third-party decision service, a device in another organization.
- Attributes or policy must stay private.
- Audit without re-execution: a decision log any auditor can check against a policy hash.
- Offline or edge verifiers that hold one evaluator identity and a few policy hashes and never call home.

Where it is pure overhead: a trusted decision point inside one organization. A signed response does the same job at a fraction of the cost. The paper says this.

## 9. Decisions made so far

See `decisions/`. In brief: Cedar (0001). AuthZEN envelope with the proof in the response context (0002). Two backends as peers (0003). RISC Zero first (0004). Noir and UltraHonk (0005). Cedar's Term IR as the compiler front end (0006). Policy as data in the zkVM (0007). Order of experiments (0008). Composition at the verifier and by compilation proof (0009). Results as committed JSON Lines (0010). The provenance system itself (0011). Journal contents and canonicalization (0012). Equal effort rule (0013). Plain names (0014).

## 10. Left to the builder

- The exact Rust types of the envelope crate.
- The Merkle tree shape for the entity commitment, within the constraints in `protocol.md`.
- How the corpus generators are seeded and committed.
- The Noir layout for TinyTodo in Phase 2.
- The Term IR to Noir translation rules in Phase 4, within the choices in 7.3.
- Choice of PEP for the compatibility demonstration in E7.

## 11. Success criteria for the project

1. Both backends produce verifying proofs for every policy set in the corpus that they support, with decisions equal to the native Cedar evaluator.
2. Every hypothesis in `hypotheses.md` has a status other than `open`, with result files.
3. The comparison table in `benchmark-protocol.md` is filled for every cell both backends can reach.
4. The coverage number for the circuit backend is measured over the whole corpus, not a chosen subset.
5. An unmodified AuthZEN PEP accepts the proof-carrying response.
6. The privacy section of the paper states, for each backend and mode, whether the proof is zero-knowledge and what the journal leaks.
7. A reader can rerun every experiment from the repository on the listed hardware.

## 12. Risks

- The Cedar crate may not build in the guest, or may need features turned off. Mitigation: E1 step 0 is only this. Fallback: SP1, or a trimmed fork with the change recorded.
- Prover memory may exceed what a phone allows. Mitigation: measure native, not only browser. Report the failure if it happens.
- The Term IR may not be a stable public surface at version 0.8. Mitigation: pin the version. Fallback: our own walker over the validated AST, with the loss of the verified front end recorded.
- The two backends may get unequal effort because one is more fun. Mitigation: hours logged in the notebook, decision 0013.
- Prezta may publish a Cedar variant first. Mitigation: our contribution is the comparison, the compiler, the composition and the protocol, not only the zkVM run.
