# Corpus

Every experiment reads its policy sets, schemas, entity stores and requests
from here. Nothing is generated at run time without a committed seed.

Planned contents, assembled in Phase 0:

- `tinytodo/`: the TinyTodo policy set, schema and sample entities from the Cedar examples repository (Apache-2.0).
- `use-cases/`: the two applications in `cedar-example-use-cases` from the same repository.
- `oopsla2024/`: the Google Drive, GitHub and tiny-todo policy sets and the entity generators from the OOPSLA 2024 Cedar benchmark, with fixed seeds and the generated stores committed.
- `synthetic/`: scaling series. Policy count 1 to 100. Entity count 10 to 10,000. Hierarchy depth 1 to 8. String attribute length 8 to 256 bytes. Each series has a generator script and a committed output with its SHA-256.
- `requests/`: one request file per policy set with a mix of permit and deny cases and the expected decision from the native Cedar evaluator, so every backend is checked for correctness before it is timed.

`MANIFEST.json` will list every file with its SHA-256. Result files reference inputs by these hashes.
