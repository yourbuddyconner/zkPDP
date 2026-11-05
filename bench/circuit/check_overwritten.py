#!/usr/bin/env python3
"""E9, part B: does a tampered witness value reach the proof at all?

Usage (in a generated circuit directory, after `nargo execute`): check_overwritten.py <out.json>

Plain-mode proofs are deterministic, so a tampered witness whose proof is
byte-identical to the honest proof had no effect. Barretenberg recomputed the
value. A proof that differs and still verifies means a value really was free.
"""
import base64, collections, gzip, hashlib, json, random, shutil, subprocess, sys, tempfile
import msgpack
P = 21888242871839275222246405745257275088548364400416034343698204186575808495617
T = "noir-recursive-no-zk"
raw = gzip.open("target/policy.gz").read()
fmt = raw[0]
ops = msgpack.unpackb(gzip.decompress(base64.b64decode(json.load(open("target/policy.json"))["bytecode"]))[1:], strict_map_key=False, raw=False)[0][0][1]
sha = [o["BlackBoxFuncCall"]["Sha256Compression"] for o in ops if "BlackBoxFuncCall" in o and "Sha256Compression" in o["BlackBoxFuncCall"]]
outs = [w for c in sha for w in c[2]]
inv = []
for i, o in enumerate(ops[:-1]):
    if "BrilligCall" in o and "AssertZero" in ops[i + 1]:
        inv += [v for x in o["BrilligCall"][2] for k, v in x.items() if k == "Simple"]
vk = tempfile.mkdtemp()
subprocess.run(["bb", "write_vk", "-b", "target/policy.json", "-o", vk, "-t", T], capture_output=True)
def prove(mut=None):
    work = tempfile.mkdtemp()
    s = msgpack.unpackb(raw[1:], strict_map_key=False, raw=False)
    m = s[0][0][1]
    if mut is not None:
        m[mut] = ((int.from_bytes(m[mut], "big") + 1) % P).to_bytes(32, "big")
    with gzip.open(work + "/w.gz", "wb") as f:
        f.write(bytes([fmt]) + msgpack.packb(s, use_bin_type=True))
    r = subprocess.run(["bb", "prove", "-b", "target/policy.json", "-w", work + "/w.gz", "-k", vk + "/vk", "-o", work, "-t", T], capture_output=True, text=True)
    if r.returncode:
        return None, False
    v = subprocess.run(["bb", "verify", "-k", vk + "/vk", "-p", work + "/proof", "-i", work + "/public_inputs", "-t", T], capture_output=True)
    d = hashlib.sha256(open(work + "/proof", "rb").read() + open(work + "/public_inputs", "rb").read()).hexdigest()
    shutil.rmtree(work, ignore_errors=True)
    return d, v.returncode == 0
h1, ok1 = prove(); h2, ok2 = prove()
print("honest verifies", ok1, ok2, "deterministic", h1 == h2)
rng = random.Random(3)
res = collections.Counter(); rows = []
m0 = msgpack.unpackb(raw[1:], strict_map_key=False, raw=False)[0][0][1]
for kind, pool, k in (("sha_output", outs, 16), ("brillig_next_to_assert", [w for w in inv if w in m0], 16)):
    for w in rng.sample(pool, min(k, len(pool))):
        d, ok = prove(w)
        c = "rejected" if not ok else "identical_proof" if d == h1 else "different_proof_verifies"
        res[(kind, c)] += 1
        rows.append({"kind": kind, "witness": w, "outcome": c, "honest_value_zero": int.from_bytes(m0[w], "big") == 0})
print(sorted(res.items()))
json.dump({"deterministic": h1 == h2, "rows": rows}, open(sys.argv[1], "w"), indent=1)
