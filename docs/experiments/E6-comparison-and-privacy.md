# E6: comparison table and privacy analysis

Status: planned. Written 2026-11-04.

## Question

Across every metric in `docs/benchmark-protocol.md`, where does each backend
win? For each backend and mode, is the proof zero-knowledge, and what does the
journal leak?

## Hypotheses

H7, H9.

## Procedure

Strict mode.

1. Fill every cell of the comparison table from result files produced by E1 to E5. Do not re-measure unless a toolchain changed. If one did, re-run both backends.
2. For each format in `docs/protocol.md`, write down the proof system's zero-knowledge status from its primary documentation and advisories. Cite the source note.
3. For the private mode, list every public value in the journal and state what it can reveal.
4. Construct the `reasons` leak: pick a corpus policy set where the set of fired policy IDs determines an attribute value. Show the inference. This tests H7.
5. Record the hours per backend from the notebook. Compute the difference. If over 25 percent, write the caveat for the paper.
6. Test H9 on TinyTodo from the filled table.

## Measurements

The table. The privacy statements. The leak demonstration.

## Results

None yet.
