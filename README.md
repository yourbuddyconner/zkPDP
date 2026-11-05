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

Anyone can check the proof in about 20 milliseconds. Checking needs three
published fingerprints: one for the policy, one for the facts, one for the
program that evaluated them. It needs no trust in the decider.

Two existing standards carry the idea, so nothing new has to be adopted:

- **Cedar**, an open-source policy language from AWS, with an evaluator whose behavior is checked against a formal model.
- **AuthZEN**, the OpenID Foundation's standard format for asking a decision point a question. Its response has room for extra data, and software that does not understand a proof ignores it and works as before.

## How

There are two ways to produce such a proof. This project builds both and
compares them on the same policies, the same data and the same requests.

**Run Cedar inside a proving virtual machine.** The published Cedar evaluator
runs, unchanged, inside RISC Zero, a system that proves a program ran
correctly. The policy is ordinary input, so changing it changes nothing else.
This is the general approach: every Cedar feature works on day one.

**Compile the policy into a circuit.** A Cedar policy set is translated into a
fixed arithmetic circuit, written in the Noir language, that computes the
same answer. The translation starts from Cedar's own verified compiler, so
the hard parts of the language's meaning are not reimplemented. This is the
fast approach: a circuit does only the work the policy needs.

Both read the facts about users and resources from a Merkle tree, so the
proof depends on the two or three records a decision touches, not on the size
of the whole store.

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

None of this is reviewed or published. Phone measurements, a fair
like-for-like comparison of the two approaches, and proving times for the
virtual machine on suitable hardware are still to come. One building block,
the SHA-256 library used inside the circuits, raises a compiler warning that
has not been audited.

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
