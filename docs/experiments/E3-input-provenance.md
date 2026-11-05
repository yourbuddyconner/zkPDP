# E3: input provenance in both backends

Status: Noir and zkVM done on 2026-11-05, GordieHowe in progress. Written 2026-11-04. Mode 2 changed to P-256 by decision 0025.

## Question

What does it cost to prove that the attributes fed to the evaluator are the
real ones, and does that cost dominate evaluation as Prezta found?

## Hypotheses

H3.

## Setup

- Both backends as left by E1 and E2.
- Corpus: TinyTodo.
- Mode 1: the entity store committed as a Merkle root per `docs/protocol.md`. The decider supplies inclusion paths.
- Mode 2: the entity data signed by an attribute authority key. Ed25519. The guest or circuit verifies the signature.

## Procedure

Strict mode.

1. In the zkVM backend, add Merkle inclusion checks for every entity the request touches. Record cycles for the checks alone.
2. In the circuit backend, the Merkle checks exist from E2. Record gates for the checks alone by building a variant without them.
3. In the zkVM backend, add one Ed25519 verification over the JCS bytes of the entity array, with the patched `curve25519-dalek` precompile. Record cycles for the check alone.
4. In the circuit backend, add one Ed25519 verification with the Noir standard library or a maintained library. Record gates for the check alone.
5. Prove and verify each variant per the benchmark protocol.
6. Compute the ratio of provenance cost to evaluation cost for each backend and mode.
7. Note in the result file that the RISC Zero precompiles are not constant time, so mode 3 with private attributes cannot use them without a timing analysis.

## Measurements

Cycles and gates for the provenance checks. Proving time delta. Proof size
delta.

## Success and failure

- H3 supported if the Ed25519 check costs at least 2 times the evaluation in both backends.
- A backend where Ed25519 does not fit in memory on the laptop is a result.

## Results

Mode 1, opening records against a root, is what E1b, E2 and E8 already measure.

Mode 2, an authority's ECDSA P-256 signature over the root and epoch. TinyTodo.

| Backend | Unsigned | Signed | Signature's cost | Authorization's cost |
|---|---|---|---|---|
| Noir, gates | 388,686 | 435,232 | 46,546 | a few hundred, estimated from the circuit |
| RISC Zero, cycles | 5.38 M | 5.58 M | 198 thousand, with the precompile | 69 thousand |

`results/E3-tinytodo-signed-2026-11-05.jsonl`, `results/E3-exec-signed-2026-11-05.jsonl`. All 19 requests match the oracle in both. A forged signature and a wrong epoch are rejected by the circuit. A forged signature aborts the guest.

Noir's proving time here, 2.92 s, was measured while a Lean build used the machine, so it is not comparable and is to be retimed.

H3 is supported on its threshold: the signature costs at least 2 times evaluation in both. In the zkVM it is small against parsing, 3.5 percent of the total.
