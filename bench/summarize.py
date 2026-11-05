#!/usr/bin/env python3
"""Summarize result files. Usage: summarize.py cycles|prove <file.jsonl>..."""
import json, sys, statistics as st
from collections import defaultdict
mode, files = sys.argv[1], sys.argv[2:]
rows = [json.loads(l) for f in files for l in open(f)]
rows = [r for r in rows if r["status"] == "ok"]
if mode == "cycles":
    segs = ["read", "parse_policies", "parse_schema", "parse_entities", "map_request", "authorize", "hash_commit"]
    by = defaultdict(lambda: defaultdict(list))
    for r in rows:
        by[r["set"]][r["metric"]].append(r["value"])
    print("| Set | Requests | Total cycles (median) | " + " | ".join(segs) + " | authorize share |")
    print("|---|---|---|" + "---|" * (len(segs) + 1))
    for s in sorted(by):
        m = {k: st.median(v) for k, v in by[s].items()}
        u = m["user_cycles"]
        cells = [f'{m["cycles_" + x] / u * 100:.1f}%' for x in segs]
        print(f'| {s} | {len(by[s]["total_cycles"])} | {int(m["total_cycles"]):,} | ' + " | ".join(cells) + f' | {m["cycles_authorize"] / u * 100:.1f}% |')
else:
    by = defaultdict(list)
    for r in rows:
        by[(r["set"], r["metric"])].append(r["value"])
    print("| Set | Metric | n | median | min | max |")
    print("|---|---|---|---|---|---|")
    for (s, m), v in sorted(by.items()):
        print(f"| {s} | {m} | {len(v)} | {st.median(v):.4g} | {min(v):.4g} | {max(v):.4g} |")
