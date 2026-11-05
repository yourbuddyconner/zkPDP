#!/usr/bin/env python3
"""E9, part B: explain every witness value whose single change was accepted.

Usage: classify_witness.py <results.jsonl> <generated dir> <summary out>
Reads the honest witness again and the constraints that mention the value.
  zero_multiplicand  every use is a product with a value that is 0, the is-zero inverse hint
  zero_after_offset  the same hint when the operand has a constant offset, (a + k) * inv, with a + k = 0
  sha_output         an output of a SHA-256 compression, which Barretenberg recomputes; see check_overwritten.py
  unused             no constraint or black box mentions it
  other              anything else; these need a person
"""
import base64, collections, gzip, json, os, shutil, subprocess, sys
import msgpack
res, gen, summary_path = sys.argv[1], sys.argv[2], sys.argv[3]
rows = [json.loads(l) for l in open(res)]
out = collections.Counter(); detail = []
print("attacks", len(rows), collections.Counter((r["set"], r["stratum"], r["accepted"]) for r in rows).most_common(30))
def mentions(x, w):
    if isinstance(x, dict):
        return any((k in ("Simple", "Witness") and v == w) or mentions(v, w) for k, v in x.items())
    if isinstance(x, list):
        return any(mentions(v, w) for v in x)
    return False
for s in sorted({r["set"] for r in rows}):
    d = os.path.join(gen, s)
    art = json.load(open(d + "/target/policy.json"))
    prog = msgpack.unpackb(gzip.decompress(base64.b64decode(art["bytecode"]))[1:], strict_map_key=False, raw=False)
    fn = prog[0][0]; ops = fn[1]; private, public = set(fn[2]), set(fn[3])
    idx = json.load(open(d + "/inputs/index.json"))
    uses = collections.defaultdict(list)
    for i, o in enumerate(ops):
        k = list(o.keys())[0]
        if k == "AssertZero":
            a = o[k]
            for term in a[0]:
                ws = [y for y in term if isinstance(y, int)]
                for w in ws:
                    uses[w].append(("mul", i, [z for z in ws if z != w] or [w]))
            for term in a[1]:
                ws = [y for y in term if isinstance(y, int)]
                for w in ws:
                    uses[w].append(("lin", i, []))
    bb = [(i, o) for i, o in enumerate(ops) if "AssertZero" not in o and "BrilligCall" not in o]
    sha_out = {w for o in ops if "BlackBoxFuncCall" in o and "Sha256Compression" in o["BlackBoxFuncCall"] for w in o["BlackBoxFuncCall"]["Sha256Compression"][2]}
    P = 21888242871839275222246405745257275088548364400416034343698204186575808495617
    for reqname in sorted({r["request"] for r in rows if r["set"] == s and r["accepted"]}):
        n = next(q["n"] for q in idx["requests"] if q["name"] == reqname)
        shutil.copy(f"{d}/inputs/{n}.toml", d + "/Prover.toml")
        subprocess.run(["nargo", "execute"], cwd=d, capture_output=True, check=True)
        m = msgpack.unpackb(gzip.open(d + "/target/policy.gz").read()[1:], strict_map_key=False, raw=False)[0][0][1]
        val = lambda i: int.from_bytes(m[i], "big") if i in m else 0
        for r in rows:
            if r["set"] != s or r["request"] != reqname or not r["accepted"]:
                continue
            w = r["witness"]; u = uses.get(w, [])
            inbb = any(mentions(o, w) for _, o in bb)
            def offset_zero():
                # c * a * w + k * w + ... with c * a + k = 0: the value multiplies zero.
                for k, i, _ in u:
                    t = ops[i]["AssertZero"]
                    total = 0
                    for term in t[0]:
                        ws = [y for y in term if isinstance(y, int)]
                        if w in ws:
                            co = int.from_bytes(term[0], "big")
                            other = [z for z in ws if z != w]
                            total += co * (val(other[0]) if other else val(w))
                    for term in t[1]:
                        ws = [y for y in term if isinstance(y, int)]
                        if w in ws:
                            total += int.from_bytes(term[0], "big")
                    if total % P != 0:
                        return False
                return True
            if w in sha_out:
                c = "sha_output"
            elif inbb:
                c = "other"
            elif not u:
                c = "unused"
            elif all(k == "mul" and all(val(z) == 0 for z in zs) for k, _, zs in u):
                c = "zero_multiplicand"
            elif offset_zero():
                c = "zero_after_offset"
            else:
                c = "other"
            region = "public" if w in public else "private_input" if w in private else "internal"
            out[(s, r["stratum"], c)] += 1
            detail.append(dict(set=s, request=reqname, witness=w, stratum=r["stratum"], region=region, classification=c, uses=[(k, i) for k, i, _ in u][:6]))
print(sorted(out.items()))
json.dump({"counts": {"/".join(k): v for k, v in sorted(out.items())}, "accepted": detail}, open(summary_path, "w"), indent=1)
others = [x for x in detail if x["classification"] == "other"]
print("other:", len(others)); [print(x) for x in others[:30]]
