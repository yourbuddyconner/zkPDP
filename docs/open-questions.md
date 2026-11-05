# Open questions

Questions nobody has answered yet. Each one names who can answer it and what
depends on it. When one is answered, move the answer into a decision record
and strike the line here with the record number.

## For the human researcher

- **License.** No license file yet. Apache-2.0 would match Cedar and RISC Zero. Blocks: publishing the repo.
- **Venue and timeline.** Prezta is at USENIX Security 2026, so that community has seen the zkVM half. Candidates: USENIX Security 2027, CCS, NDSS, PETS for the privacy angle, or a workshop first. Blocks: how much of the plan must finish before writing starts.
- **Authorship.** Who is on the paper. How the AI assistant's role is described. Blocks: nothing yet, needed before submission.
- **Hardware.** Which laptop, which phone, whether a GPU machine is available for server-side proving. Blocks: benchmark protocol's machine list.
- **Hours.** How many hours per week this gets. The plan's phase lengths are guesses that assume part-time work.

## Technical, to be settled by experiment

- Does the current `cedar-policy` crate build for the RISC Zero guest with default features, and if not, which features must be turned off? E1 step 0.
- Is the Cedar guest dominated by deserialization or by evaluation? Decides whether the protobuf input form is worth the experimental feature. E1.
- Is the `cedar-policy-symcc` Term IR stable enough at version 0.8 to build a backend on? Does it expose the IR before the SMT encoding step? E2 and E4.
- How are optional attributes (`has`) and Cedar extension types (decimal, ipaddr, datetime) laid out in a circuit? E2.
- Should the journal carry the IDs of the policies that fired? Useful for audit, leaks attributes. Protocol v0 makes it optional. E6 decides the default.
- Do RISC Zero's precompiles leak private inputs through timing? The docs say they are not constant time. Matters only for the private-attribute mode. E3 and E6.
- Is a Groth16 wrap needed at all when the verifier is a server and not a chain? Proof size versus wrap time. E1.
- Can one circuit serve a policy template with literals as inputs, so small policy edits do not need a new verification key? Not planned. Note for the paper's future work.
- ~~Is the Prezta artifact available for a direct comparison on the same hardware?~~ Yes, on Zenodo, doi:10.5281/zenodo.20303462. Not yet run.

## For the human researcher, added 2026-11-04

- **Is the request public in the private mode?** The human does not know yet. If the request names a principal, the mode hides attributes but not identity, and it cannot be called anonymous. Until settled, no document says anonymous. Blocks: the private mode's design in E3 and E6.

## Process

- How often the provenance records get a consistency check. Proposal: at each phase gate.
