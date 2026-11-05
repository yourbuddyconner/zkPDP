#!/usr/bin/env python3
"""Copy compiled circuits and their first input into public/circuits for the browser benchmark.

Run after `nargo compile` in each circuit directory. Inputs go from TOML to JSON.
"""
import json, os, shutil, tomllib

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.join(HERE, "..", "circuit")
SETS = [
    ("tinytodo-hand", os.path.join(C, "tinytodo"), "tinytodo", "E2", "hand-written TinyTodo"),
    ("tinytodo", os.path.join(C, "generated", "tinytodo"), "policy", "E4", "generated TinyTodo"),
    ("github", os.path.join(C, "generated", "usecase-github_example"), "policy", "E4", "generated GitHub example"),
    ("hotel", os.path.join(C, "generated", "usecase-hotel_chains-static"), "policy", "E4", "generated hotel chains"),
]
out = os.path.join(HERE, "public", "circuits")
shutil.rmtree(out, ignore_errors=True)
index = []
for name, d, pkg, experiment, label in SETS:
    o = os.path.join(out, name)
    os.makedirs(o)
    shutil.copy(os.path.join(d, "target", f"{pkg}.json"), os.path.join(o, "circuit.json"))
    idx = json.load(open(os.path.join(d, "inputs", "index.json")))
    req = next(r for r in idx["requests"] if "skipped" not in r)
    inp = tomllib.load(open(os.path.join(d, "inputs", f"{req['n']}.toml"), "rb"))
    json.dump(inp, open(os.path.join(o, "input.json"), "w"))
    index.append({"name": name, "label": label, "experiment": experiment, "set": idx["set"], "request": req["name"],
                  "decision": req["decision"], "request_hash": req["request_hash"], "policy_hash": idx["policy_hash"],
                  "entities": idx["entities"]})
json.dump(index, open(os.path.join(out, "index.json"), "w"), indent=1)
print("circuits:", [i["name"] for i in index])
