# Paper outline

Working title: Proof-carrying authorization: running Cedar inside a zkVM and
compiling it to circuits, under the AuthZEN API.

The title is descriptive on purpose (decision 0014). It will shorten once the
results say which half carried the paper.

## Thesis

A policy decision can be moved to a party the verifier does not trust, at a
cost of seconds of proving and milliseconds of verification, inside the
standard AuthZEN wire format, with no change to the specification.

## Draft abstract

This is a statement of hope, written before any result. It is rewritten in
Phase 7 from the results. Do not quote it.

> Authorization decisions are trusted because of who makes them. We show how
> to make them trusted because of what was computed. A decider evaluates a
> Cedar policy set inside a proof system and returns an ordinary AuthZEN
> response carrying a succinct proof that a known evaluator applied a known
> policy to authenticated inputs. We build two backends under one envelope:
> the published Cedar evaluator, with one dependency replaced, running in a general zkVM, and
> a compiler from validated Cedar policies, through Cedar's own symbolic
> intermediate representation, to arithmetic circuits proved
> client-side. We measure both on a shared corpus, including the cost of
> input provenance, and show that they compose: proofs bind at the verifier
> through shared commitments, and a zkVM proof of compilation lets a verifier
> obtain circuit keys by proof for an allowlisted policy. We report what each backend
> leaks, which is zero-knowledge and which only succinct, and demonstrate
> that an unmodified AuthZEN enforcement point accepts the response.

## Contributions, each mapped to evidence

1. A proof-carrying AuthZEN response and verification procedure. `docs/protocol.md`, E7.
2. Measurements of the published Cedar evaluator in a zkVM, by segment. E1.
3. A compiler from validated Cedar to circuits through the Term IR, with a coverage number over a public corpus. E2, E4.
4. The cost of input provenance in both backends. E3.
5. Composition by verifier-side binding and by compilation proof. E5.
6. A comparison on one corpus under an equal effort rule. E6.
7. A privacy analysis that separates succinct from zero-knowledge and demonstrates the `reasons` leak. E6.

## Sections

1. **Introduction.** The trust shift, credited to proof-carrying authorization (Appel and Felten 1999). Three settings where it matters: untrusted deciders including AI agent runtimes, private attributes, audit. One paragraph on where it is overhead. Contributions.
2. **Background.** Cedar and its verification. AuthZEN. zkVMs and circuits in two paragraphs each. Prezta.
3. **Threat model.** From `docs/threat-model.md`.
4. **The envelope.** From `docs/protocol.md`. Freshness, revocation, batch.
5. **Cedar in a zkVM.** Design. Policy as data. Segment costs. What dominated.
6. **Cedar compiled to circuits.** Why validated Cedar is static. The Term IR. Encoding choices: strings, hierarchy, sets, optionals, extension types. The compiler. Coverage.
7. **Input provenance.** Three modes. Costs. The precompile timing note.
8. **Composition.** Verifier-side binding. Compilation proof.
9. **Evaluation.** Corpus, machines, protocol, equal effort hours. The comparison table. Scaling plots. Gate outcomes, including any cancelled path.
10. **Privacy.** Per format and mode. The `reasons` demonstration.
11. **Discussion and limitations.** Prove once present many. Where a signature wins. Trusted setup. Policy agility. Phone memory. What we did not measure.
12. **Related work.** Opens with the proof-carrying authorization lineage, then `paper/related-work.md`.
13. **Conclusion.**

Appendix: result file index. Artifact description. Toolchain versions.

## Known limitations before starting

- One policy language.
- One zkVM measured, with a port offered but not promised.
- Proving times from one laptop, one phone, and a server if available.
- The circuit backend covers a subset. The number is reported, not hidden.
- RISC Zero's zero-knowledge status is an open advisory.

## Ethics and disclosure

No human subjects. No private data. All corpus data is public or synthetic.
AI assistance in ideation, literature search and drafting is disclosed in the
paper and recorded in `docs/notebook/` and `docs/decisions/`.

## Artifact

The repository at a tagged commit. Pinned toolchains. One script per
experiment. The corpus manifest. Result files. Target: an artifact evaluation
badge for reproducibility.
