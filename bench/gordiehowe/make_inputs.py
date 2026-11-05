#!/usr/bin/env python3
"""Flatten the hand-written circuit's TinyTodo inputs for the GordieHowe driver.

Usage: make_inputs.py <circuit dir> <out dir>
Writes one text file per request: `key hex` per line. Strings hold only their used bytes.
"""
import glob, json, os, sys, tomllib
src, out = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)
idx = json.load(open(src + "/inputs/index.json"))
for r in idx["requests"]:
    t = tomllib.load(open(f"{src}/inputs/{r['n']}.toml", "rb"))
    lines = [f"name {r['name'].encode().hex()}", f"decision {int(t['decision']):02x}"]
    for k in ("request_hash", "policy_hash", "schema_hash", "entities_root"):
        lines.append(f"{k} {bytes(t[k]).hex()}")
    for k in ("nonce", "subject_type", "subject_id", "action", "resource_type", "resource_id"):
        lines.append(f"{k} {bytes(t[k]['storage'][:t[k]['len']]).hex()}")
    for e in ("principal", "resource"):
        x = t[e]
        lines.append(f"{e}.present {x['present']:02x}")
        for k in ("attrs", "ancestors", "path"):
            lines.append(f"{e}.{k} {b''.join(bytes(s) for s in x[k]).hex()}")
        lines.append(f"{e}.sides {bytes(int(b) for b in x['sibling_is_left']).hex()}")
        lines.append(f"{e}.path_len {x['path_len']:02x}")
    open(f"{out}/{r['n']}.txt", "w").write("\n".join(lines) + "\n")
print(len(idx["requests"]), "inputs")
