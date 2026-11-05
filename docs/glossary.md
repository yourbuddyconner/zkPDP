# Glossary

One name for one thing. These are the names this project uses.

- **PDP**: policy decision point. The component that answers "may this subject do this action on this resource".
- **PEP**: policy enforcement point. The component that asks the PDP and acts on the answer. Here, usually the resource server.
- **AuthZEN**: the OpenID Foundation Authorization API 1.0. A JSON over HTTP format for asking a PDP and getting a decision back.
- **Cedar**: an open-source authorization policy language from AWS, with a Rust evaluator, a validator and a Lean formal model.
- **Policy set**: the Cedar policies in force for one verifier. Identified by its policy hash.
- **Schema**: the Cedar schema that types entities and actions. Validation of a policy set is against a schema.
- **Entity store**: the Cedar entities (principals, resources, groups) and their attributes and parent relations.
- **Transitive closure**: the full set of ancestor relations in the entity store. Cedar computes it when it loads entities. The `in` operator reads it.
- **Decider**: the party that evaluates the policy and produces the proof. Also called the prover. Untrusted.
- **Verifier**: the party that checks the proof and acts on the decision. Usually the PEP.
- **Policy authority**: the party that writes the policy set and publishes its hash.
- **Attribute authority**: the party that vouches for the entity data, by signing credentials or publishing an entity commitment.
- **zkVM**: a proof system that proves the correct execution of an ordinary program, here RISC-V machine code compiled from Rust.
- **Image ID**: the hash that identifies one guest program in RISC Zero. The verifier pins it.
- **Guest**: the program that runs inside the zkVM. **Host**: the program that runs the zkVM and feeds the guest.
- **Journal**: the public outputs a guest commits to. The verifier reads the decision from it.
- **Seal**: the cryptographic part of a RISC Zero receipt. **Receipt**: journal plus seal.
- **Circuit**: a fixed arithmetic constraint system. Here, written in Noir and proved with Barretenberg's UltraHonk.
- **Verification key**: the identifier of one circuit. The verifier pins it, as it pins an image ID.
- **Gate**: the unit of circuit size. Proving time grows with gate count.
- **Witness**: the private inputs plus every intermediate value of one circuit execution.
- **Succinct proof**: a proof that is small and fast to verify no matter how long the computation was.
- **Zero-knowledge proof**: a succinct proof that also reveals nothing about private inputs beyond the public outputs. Not every succinct proof is zero-knowledge.
- **STARK**: the hash-based proof RISC Zero produces first. Large, no trusted setup.
- **Groth16**: a small pairing-based proof. RISC Zero can wrap a STARK into one. Needs a trusted setup.
- **UltraHonk**: Barretenberg's proof system for Noir circuits. Its zero-knowledge mode is designed to hide the witness. We found no published proof of that.
- **Term IR**: the typed intermediate representation that Cedar's symbolic compiler produces from a validated policy set.
- **Symbolic compiler**: the `cedar-policy-symcc` crate. Compiles Cedar to Term IR and then to SMT formulas, for analysis.
- **JCS**: JSON Canonicalization Scheme, RFC 8785. One byte string for one JSON value.
- **Entity commitment**: a hash or Merkle root that pins one entity store so the proof is about that store and no other.
- **Nonce**: a verifier-chosen value that makes a proof fresh and not replayable.
- **Epoch**: a verifier-published period. Proofs made in one epoch expire with it.
- **Input provenance**: evidence that the attributes the decider fed the evaluator are the real ones.
- **Compilation proof**: a zkVM proof that a given policy hash compiles to a given verification key.
