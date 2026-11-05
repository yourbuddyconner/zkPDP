# 0019: How the Cedar to Noir compiler is built

Status: accepted, pending review. Date: 2026-11-04.
Proposed by: assistant, during E4.
Accepted by: not yet reviewed by the human.
Implements: 0006. Extends: 0018.

## Context

E4 step 1 asked whether the Term IR of `cedar-policy-symcc` 0.8.1 can be
reached from a downstream crate. The types `Term`, `TermType` and `Op` are
public. The compiled term of a policy is not: `CompiledPolicy` keeps it
private to its crate.

## Decisions

1. **Getting the term.** `never_matches_asserts(&compiled_policy)` is public. Its last assertion is the boolean "this policy matches", already simplified. The compiler reads that. No solver is involved and no private field is touched. If upstream changes how that assertion is built, this breaks. Pinned to 0.8.1.
2. **One function per request environment.** The symbolic compiler specializes a policy to a principal type, an action and a resource type. The circuit has one function per such environment, selects by the request's types and action, and asserts that exactly the schema's environments are reachable. Environments with identical functions share one.
3. **Openings are shared.** The expensive part, hashing an opened entity up to the root, is done once per slot in `main`. Each environment function only asserts that the opened entity's UID is the one its term names. A circuit's size follows the largest number of entities any one environment reads, not the number of environments.
4. **Entities reached through attributes are opened too.** `resource.repo.readers` opens the resource and then the repository it names.
5. **Sets and nested records are opened by hash.** A set-valued or record-valued attribute is one slot holding the hash of a fixed-size block. A policy that reads it takes the block as a private input and the circuit checks the hash. The record commitment was extended for this, and attribute positions now follow the schema, with a presence bit per attribute, so an absent optional attribute does not shift the others.
6. **An absent value is the zero slot.** When a term's entity or block is absent, the circuit substitutes zeros and skips the hash check. It relies on the symbolic compiler's guarantee that a term never uses the content of an absent option.
7. **Strings are compared by hash.** Equality only.
8. **Limits are constants**: 8 attributes per record, 16 ancestors, 8 set elements, tree depth 12, and string lengths for the request fields.

## Not supported in this version

- The request context. The circuit rebuilds the canonical request from typed fields, and only an empty context or a nonce is handled.
- The `like` operator. Strings are committed as hashes.
- Extension values: datetime, duration, decimal, ipaddr.
- Integer arithmetic. Integer comparison is supported.
- Entity tags.
- Actions in a namespace, policy templates.
- A principal or resource that is not in the store.

## Reasoning

Every choice keeps the back end a direct, local translation of terms, so it
can be checked by differential test against the native evaluator, which is
how Cedar itself is tested. The verified front end does the hard part:
typing, error semantics, and specialization per environment.

## Consequences

- The coverage number in E4 is the measure of the "not supported" list.
- Context support is the largest single gain: 15 of 25 untranslated terms in the upstream use cases.
- The record commitment changed after E2 ran. The hand-written circuit was updated and its gate count rose from 353,518 to 388,254 with 16 ancestor slots. E2's result file is from the earlier definition and says so in the experiment file.

## Sources

S05.
