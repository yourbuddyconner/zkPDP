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

There are two ways to produce such a proof. This project builds both and
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

The circuit reads the facts about users and resources from a Merkle tree, so
its proof depends on the two or three records a decision touches, not on the
size of the whole store. The virtual machine route still reads the whole store
today.

## What we have found so far

Measured on one laptop, an Apple M1 Max, in November 2026. One decision over a
small to-do list application's policy with 43 users, teams and lists.

| | Cedar in a virtual machine | Policy compiled to a circuit |
|---|---|---|
| Gives the same answers as Cedar | yes, 83 of 83 test requests | yes, 36 of 36 test requests |
| Time to prove | not finished after 28 minutes | 2.2 to 2.8 seconds |
| Time to prove in a web browser | not attempted | 4 to 7 seconds |
| Time to check the proof | not measured | 20 milliseconds |
| Proof size | not measured | 14.7 kilobytes |
| Cedar features covered | all | 3 of 7 sample policy sets |

What that says:

- **The circuit is fast enough to use.** A person can wait for it, in a browser, on their own machine.
- **The virtual machine is not, yet, on a laptop.** Almost none of its work is deciding. 99 percent is parsing text into data structures. It also had to read the whole store where the circuit reads two records, so this comparison is not yet like for like. And RISC Zero does not use the graphics processor on Apple hardware in its current release.
- **The circuit does not cover everything.** Policies that read extra context from the request, compare dates, or use tags do not compile yet. Most of the gap is one missing feature.
- **Browsers have a ceiling.** The largest sample circuit proves in 5 seconds natively and about 45 in a browser.

A third route is in progress. TinyTodo written by hand in GordieHowe, a proof system whose security theorems are machine-checked in Lean, proves in about 37 ms on one thread, with a 117 KB proof and 13 ms to check. Its strings have fixed lengths per request shape and its Lean certificate has not been checked for this program, so it is not yet comparable. Attack tests on the Noir circuits and on GordieHowe found no way to get a wrong decision accepted, with one change at a time.

None of this is reviewed or published. Phone measurements, a fair
like-for-like comparison of the two approaches, and proving times for the
virtual machine on suitable hardware are still to come. One building block,
the SHA-256 library used inside the circuits, raises a compiler warning that
has not been audited. Neither route has a published proof that it
is zero-knowledge, so private inputs are a goal and not yet a claim.

## How new this is

The idea is not new. Proof-carrying authorization dates from 1999, and Prezta
(USENIX Security 2026) already proves XACML decisions in RISC Zero. What we have
not found elsewhere is a compiler from Cedar to circuits, or one policy language
measured on both a virtual machine and a compiled circuit. That is a search
result, not proof of absence, and this niche moved monthly through 2026. The
review is in `docs/literature/reports/`, with 423 references in
`docs/literature/references.bib`.

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
