# E4: compiling Cedar to Noir through the Term IR

Status: first version done on 2026-11-04. Written 2026-11-04.

## Question

Can validated Cedar policy sets be translated from the symbolic compiler's Term
IR to Noir automatically, and what fraction of the corpus does the translation
handle?

## Hypotheses

H4.

## Setup

- `cedar-policy-symcc` pinned at the version current when the phase starts. Version 0.8.1 was current on 2026-11-04.
- The circuit shape learned in E2.
- The whole corpus.

## Procedure

Strict mode.

1. Confirm the Term IR is reachable from a downstream crate before the SMT encoding step. If not, record it, and fall back to walking the validated AST. Record the fallback as a decision.
2. Write the translation, one Term operator at a time, in the order of frequency in the corpus.
3. After each operator, run the differential test: for every corpus policy set the translation handles, compare the circuit decision to the oracle on every request.
4. Record, per policy set, whether it compiles and which construct blocked it if not.
5. Record gates per compiled policy set.
6. Compute coverage: compiled policy sets over all policy sets in the corpus. Also compute coverage by construct.
7. Stop adding operators when the time box in the notebook is spent. Report coverage at that point.

## Measurements

Coverage by policy set and by construct. Gates per policy set. Compile time.
Differential test pass rate.

## Success and failure

- H4 supported if coverage by policy set is at least 80 percent.
- Any differential mismatch is a defect and is fixed or reported.
- Gate C in `docs/plan.md` reads the coverage number.

## Results

Run on 2026-11-04 at commit 1761855. Same machine and toolchain as E2, with
`cedar-policy-symcc` 0.8.1. Compiler in `bench/compiler`. Design in decision
0019.

### Step 1: is the Term IR reachable

Yes. The types are public. The compiled term is private, and the public
`never_matches_asserts` yields the boolean "this policy matches" as its last
assertion. For TinyTodo those terms are the expressions the E2 circuit wrote
by hand, for example:

```
or(set.member(getattr["readers"](attrs[List](resource)), ancs[User, Team](principal)),
   set.member(getattr["editors"](attrs[List](resource)), ancs[User, Team](principal)))
```

Every policy in every corpus set compiles to terms with no error from the
symbolic compiler.

### Coverage

`results/E4-coverage-2026-11-04.jsonl`.

| Measure | Compiles | Of | Share |
|---|---|---|---|
| Corpus sets | 15 | 23 | 65% |
| Distinct upstream policy sets | 3 | 7 | 43% |
| Terms, all sets | 2,384 | 2,437 | 97.8% |
| Terms, upstream sets | 531 | 556 | 95.5% |

The corpus count is flattered by 16 synthetic sets built on TinyTodo. The
honest number for "real policies" is 3 of 7.

Compiles: TinyTodo, the GitHub example, hotel chains, and the synthetic
policy, entity and depth series.

What blocks the rest, by untranslated term in the upstream sets:

| Blocker | Terms | Sets |
|---|---|---|
| The request context | 15 | document_cloud, sales_orgs |
| Entity tags | 4 | tags_n_roles |
| Extension values | 3 | streaming_service |
| Integer arithmetic | 3 | streaming_service |

The four synthetic string-length sets are blocked by the `like` operator.

### Differential test

`results/E4-compiled-2026-11-04.jsonl`. For each request the generated circuit
must accept the native decision and reject the opposite one.

| Set | Requests | Match |
|---|---|---|
| tinytodo | 19 | 19 |
| usecase-github_example | 7 | 7 |
| usecase-hotel_chains-static | 6 | 6 |
| syn-policies-010 | 2 | 2 |
| syn-policies-100 | 2 | 2 |
| syn-entities-01000 | 2 | 2 |

36 of 36. No mismatch was found at any point, including the first run. The
upstream sets have few requests each, so this is a thin test. More generated
requests per set are needed before the paper leans on it.

### Size and time of generated circuits

Zero-knowledge mode, median of 5, native on the laptop.

| Set | Entities opened | Sets | Records | Gates | Witness | Prove | Memory | Verify |
|---|---|---|---|---|---|---|---|---|
| tinytodo | 2 | 0 | 0 | 453,510 | 0.39 s | 2.75 s | 0.90 GB | 19 ms |
| github_example | 3 | 0 | 0 | 631,404 | 0.47 s | 3.83 s | 1.26 GB | 20 ms |
| hotel_chains | 2 | 6 | 2 | 858,182 | 1.94 s | 4.92 s | 2.14 GB | 20 ms |

Proof size is 14,656 bytes for all three.

Observations:

- The generated TinyTodo circuit is 453,510 gates against 388,254 by hand at the same limits. The 17 percent comes from longer string limits and the unused set and record slot the generated circuit always carries.
- 100 extra policies add 11 thousand gates. Policy logic is nearly free. Size follows what is opened: about 170 thousand gates per entity and tens of thousands per set.
- 1,000 entities in the store gives the same circuit as 43. Store size does not matter below the tree depth limit.
- Hotel chains needs 2.1 GB to prove. That is over what a phone browser allows.

### Gate C

The plan drops E5 if the compiler handles under half the corpus. By corpus
sets it handles 65 percent. By distinct upstream sets, 43 percent. The plan's
wording was "the corpus", so the gate passes on its letter. The spirit is
borderline. Context support would settle it.
