#!/usr/bin/env python3
"""Flatten a generated circuit's inputs for the GordieHowe policy interpreter.

Usage: make_policy_inputs.py <generated dir> <out dir>
"""
import glob, json, os, sys, tomllib
src, out = sys.argv[1], sys.argv[2]
os.makedirs(out, exist_ok=True)
idx = json.load(open(src + "/inputs/index.json"))
flat = lambda rows: b"".join(bytes(r) for r in rows).hex()
n = 0
for r in idx["requests"]:
    if "skipped" in r:
        continue
    t = tomllib.load(open(f"{src}/inputs/{r['n']}.toml", "rb"))
    lines = [f"name {r['name'].encode().hex()}", f"decision {int(t['decision']):02x}"]
    for k in ("request_hash", "policy_hash", "schema_hash", "entities_root", "context_hash"):
        lines.append(f"{k} {bytes(t[k]).hex()}")
    for k in ("nonce", "subject_type", "subject_id", "action", "resource_type", "resource_id"):
        lines.append(f"{k} {bytes(t[k]['storage'][:t[k]['len']]).hex()}")
    for j, e in enumerate(t["ents"]):
        lines.append(f"ents.{j}.uid_h {bytes(e['uid_h']).hex()}")
        lines.append(f"ents.{j}.present {e['present']:02x}")
        lines.append(f"ents.{j}.attrs {flat(e['attrs'])}")
        lines.append(f"ents.{j}.ancestors {flat(e['ancestors'])}")
        lines.append(f"ents.{j}.path {flat(e['path'])}")
        lines.append(f"ents.{j}.sides {bytes(int(b) for b in e['sibling_is_left']).hex()}")
        lines.append(f"ents.{j}.path_len {e['path_len']:02x}")
    for j, x in enumerate(t["sets"]):
        lines.append(f"sets.{j} {flat(x)}")
    for j, x in enumerate(t["recs"]):
        lines.append(f"recs.{j}.present {x['present']:02x}")
        lines.append(f"recs.{j}.slots {flat(x['slots'])}")
    lines.append(f"ctx.present {t['ctx']['present']:02x}")
    lines.append(f"ctx.slots {flat(t['ctx']['slots'])}")
    open(f"{out}/{r['n']:03d}.txt", "w").write("\n".join(lines) + "\n")
    n += 1
print(n, "inputs")
