# zk-pdp

Research in progress. Can a system trust an access decision because of what
was computed, and not because of who computed it?

## Why

When an application asks "may this user do this", something answers yes or
no. That something is called a policy decision point. Today the application
trusts the answer because it trusts whoever runs the decision point, the key
that signed the answer, and the network in between.

That is fine inside one organization. It breaks down in three places:

- **The decider is not trusted.** A client device, an AI agent's runtime, a vendor's hosted service, or a machine in someone else's building makes the decision, and the party enforcing it has no reason to believe it.
- **The inputs are private.** The user can show they qualify, but should not have to reveal the attributes that make them qualify.
- **Someone audits later.** A reviewer wants to check that past decisions followed the policy, without re-running the system or trusting its operator.

In all three, a signature does not help. A signature says who answered. It
does not say the answer is right.

## What

A decision that carries a proof. The decider returns the usual yes or no,
plus a small cryptographic proof of this statement:

> This policy, applied to this request and this committed set of facts about
> users and resources, gives this answer.

On the circuit route, anyone can check the proof in about 20 milliseconds. Checking needs three
published fingerprints: one for the policy, one for the facts, one for the
program that evaluated them. It needs no trust in the decider.

Two existing standards carry the idea, so no new standard is needed.
A verifier still has to check the proof to gain anything from it:

- **Cedar**, an open-source policy language from AWS, with an evaluator whose behavior is checked against a formal model.
- **AuthZEN**, the OpenID Foundation's standard format for asking a decision point a question. Its response has room for extra data. The spec lets a client that does not understand that data ignore it or reject the decision, so compatibility has to be tested on real clients.

## How

There are several ways to produce such a proof. This project builds three and
compares them on the same policies, the same data and the same requests.

**Run Cedar inside a proving virtual machine.** The published Cedar evaluator
runs inside RISC Zero, a system that proves a program ran correctly. Cedar's own
source is untouched, but one dependency that cannot build for that target is
replaced with a stand-in. The policy is ordinary input, so changing it changes nothing else.
This is the general approach: every Cedar feature works on day one.

**Compile the policy into a circuit.** A Cedar policy set is translated into a
fixed arithmetic circuit, written in the Noir language, that computes the
same answer. The translation starts from Cedar's own symbolic compiler,
which has a formal model and proofs in Lean, so the hard parts of the language's
meaning are not reimplemented. The step from there to Noir is ours, and it is
tested, not proved. This is the
fast approach: a circuit does only the work the policy needs.

**Run that circuit on GordieHowe.** The same compiled policy, read at run time, on a
proof system whose security theorems are machine-checked in Lean. This is the
fastest route so far, and the one that may let the whole chain be checked.

Every route reads the facts about users and resources from a Merkle tree, so a
proof depends on the two or three records a decision touches, not on the size
of the whole store.

## What we have found so far

Measured on one laptop, an Apple M1 Max, in November 2026. One decision over a small to-do list application's policy with 43 users, teams and lists. All three routes read the same two records and are compiled once per policy.

| | Cedar in a virtual machine | Policy compiled to Noir | Policy compiled to GordieHowe |
|---|---|---|---|
| Gives the same answers as Cedar | 83 of 83 test requests | 40 of 40 | 40 of 40 |
| Time to prove | 13 minutes | 2.1 seconds | 43 milliseconds |
| Time to check the proof | 126 milliseconds | 15 milliseconds | 15 milliseconds |
| Proof size | 2.2 megabytes | 15 kilobytes | 125 kilobytes |
| Cedar coverage | all | 5 of 7 sample policy sets | 5 of 7 sample policy sets |

What that says:

- **The compiled routes are fast enough to use.** The virtual machine route is not, yet, on a laptop, and it runs on the CPU here because RISC Zero's release does not use the graphics processor on Apple hardware. Its 99 percent of work is parsing text, not deciding.
- **The compiler does not cover everything.** Policies that compare dates, do arithmetic, or use tags do not compile yet.
- **Facts can be vouched for.** An authority's signature over the facts adds little: under 40 milliseconds on GordieHowe, though its proof more than doubles.
- **We tried to cheat and could not.** Thousands of attacks on the circuits found no way to get a wrong decision accepted. They also found one real weakness, now fixed. These attacks mostly change one value at a time, so this is evidence and not a proof of soundness.

None of this is reviewed or published. Three caveats matter. The translation from Cedar's compiler to these circuits is ours and is tested, not proved. GordieHowe has machine-checked security theorems, but they do not yet apply to our programs, and work to connect them is in progress. Neither Noir nor RISC Zero has a published proof that it is zero-knowledge, and all three warn they are research or not production-ready in places, so private inputs are a goal and not yet a claim.

## How new this is

The idea is not new. Proof-carrying authorization dates from 1999, and Prezta (USENIX Security 2026) already proves XACML decisions in RISC Zero. What we have not found elsewhere is a compiler from Cedar to circuits, or one policy language measured on both a virtual machine and compiled proofs. That is a search result, not proof of absence, and this niche moved monthly through 2026. The review is in `docs/literature/reports/`, with 423 references in `docs/literature/references.bib`.

## What this is not

It does not replace an ordinary decision point where one is already trusted.
There a signed answer does the same job for far less work. It does not show
that a policy is the right policy, only that the policy was applied.

## Going further

Everything here was planned as a research paper, and the repository keeps a
record of every decision, source and day of work.

- `docs/spec.md`: the design in full.
- `docs/plan.md` and `docs/hypotheses.md`: what is being tested, with thresholds set before each run.
- `docs/experiments/`: each experiment, its procedure and its results.
- `docs/decisions/`: why each choice was made and what was rejected.
- `docs/notebook/`: what happened each day, dead ends included.
- `bench/`: the code. `corpus/`: the test policies and data. `results/`: raw measurements.

## Disclosure

An AI assistant (Claude, Anthropic) took part in the ideation, literature
search, code and drafting recorded here. The notebook and decision records
say which ideas and choices came from the human researcher and which the
assistant proposed.
