# results

Machine-written only. One JSON Lines file per experiment run. Each line:

```json
{
  "experiment": "E1",
  "backend": "zkvm-risc0",
  "commit": "<git sha>",
  "timestamp": "<RFC 3339>",
  "machine": { "name": "", "cpu": "", "ram_gb": 0, "gpu": "", "os": "" },
  "toolchain": { "rustc": "", "risc0": "", "nargo": "", "bb": "" },
  "inputs": { "policy_hash": "", "schema_hash": "", "entity_commitment": "", "request_hash": "" },
  "metric": "prove_time_s",
  "value": 0.0,
  "repeat": 0,
  "status": "ok | failed | aborted",
  "notes": ""
}
```

Rules are in `docs/provenance.md`. Never edit a line. Never delete a file.
