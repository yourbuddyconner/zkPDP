# Corpus

Every experiment reads its policy sets, schemas, entity stores and requests
from here. Nothing is generated at run time.

## Layout

- `build.py` assembles `raw/` from a checkout of `cedar-policy/cedar-examples` and from seeded generators. Deterministic.
- `raw/<set>/` holds each set as assembled: `policies.cedar`, `schema.cedarschema`, `entities.json`, `requests.json`, and a `NOTE` saying where it came from.
- `sets/<set>/` holds the canonical form every backend reads. Policies after Cedar's formatter. Entities as JCS bytes sorted by UID. `requests.jsonl` with one canonical AuthZEN request per line, its hash, and the native Cedar decision, the oracle.
- `MANIFEST.json` lists, per set, the policy hash, schema hash, both entity commitments, counts, and whether the store conforms to its schema.

`bench/corpus-tool` writes `sets/` and the manifest from `raw/`.

## Rebuild

```sh
git clone https://github.com/cedar-policy/cedar-examples /tmp/cedar-examples
git -C /tmp/cedar-examples checkout f4d2dbbab361a38372e6d6fcfc7b777eb1b30c37
python3 corpus/build.py /tmp/cedar-examples
(cd bench && cargo run --release -p corpus-tool -- ../corpus)
git status --short corpus   # expect no change
```

## Sets, as of 2026-11-04

23 sets, 83 requests, 47 permit and 36 deny.

- `tinytodo`: upstream policies and schema. A generated store of 43 entities, seed 1. 19 requests. This is the "TinyTodo-sized" set the hypotheses name.
- `usecase-*`: six upstream use cases with their own entities and ALLOW and DENY queries, converted to AuthZEN requests. The oracle agrees with every upstream expectation.
- `syn-policies-*`, `syn-entities-*`, `syn-depth-*`, `syn-strlen-*`: scaling series on the TinyTodo schema, seed 2, one variable at a time.

## Findings from assembly

- Two upstream stores, `document_cloud` and `github_example`, do not conform to their own schemas. Upstream authorizes them without a schema and so do we. The manifest marks them `uses_schema: false`. This matters for the circuit backend, which needs conforming entities.
- `tax_preparer`, `hotel_chains/templated` and `sales_orgs/templated` use linked templates and are not yet included.
- The three OOPSLA 2024 benchmark applications are not yet included. The synthetic series covers scaling for E1. They are still wanted for the coverage number in E4.
