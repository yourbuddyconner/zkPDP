#!/usr/bin/env python3
"""E9, part B: witness-level attacks on a generated circuit.

Usage: adversarial_witness.py <out.jsonl> <per-request sample> <workers> <circuit dir>...

Takes the honest witness for one permit and one deny, changes a single witness
value by adding 1, proves with bb and verifies with bb. A proof that verifies
means that value is free in the constraint system as Barretenberg lowered it.
Sampled strata: every public input, outputs of unconstrained Brillig calls,
private inputs, and other internal values.

Limit: one value at a time. It finds wires that nothing constrains. It does not
find a bug that needs several values changed together, such as a wrong but
consistent helper output.
"""
import base64, concurrent.futures as cf, gzip, json, os, random, re, shutil, subprocess, sys, tempfile, time
import msgpack

P = 21888242871839275222246405745257275088548364400416034343698204186575808495617
out_path, sample, workers, dirs = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), sys.argv[4:]
TARGET = "noir-recursive"
rng = random.Random(9)


def sh(*a, cwd=None, timeout=240):
    return subprocess.run(a, cwd=cwd, capture_output=True, text=True, timeout=timeout)


def brillig_outputs(ops):
    out = set()
    def walk(x):
        if isinstance(x, dict):
            for k, v in x.items():
                if k in ("Simple", "Witness") and isinstance(v, int):
                    out.add(v)
                else:
                    walk(v)
        elif isinstance(x, list):
            for v in x:
                walk(v)
    for o in ops:
        if isinstance(o, dict) and "BrilligCall" in o:
            call = o["BrilligCall"]
            # [id, inputs, outputs, predicate]
            walk(call["outputs"] if isinstance(call, dict) else call[2])
    return out


def read_witness(path):
    raw = gzip.open(path).read()
    return raw[0], msgpack.unpackb(raw[1:], strict_map_key=False, raw=False)


def write_witness(path, fmt, stack):
    with gzip.open(path, "wb") as f:
        f.write(bytes([fmt]) + msgpack.packb(stack, use_bin_type=True))


def prove_verify(circuit, vk, witness, work):
    r = sh("bb", "prove", "-b", circuit, "-w", witness, "-k", vk, "-o", work, "-t", TARGET)
    if r.returncode != 0:
        return False, "prove failed: " + (r.stderr + r.stdout)[-100:].strip()
    v = sh("bb", "verify", "-k", vk, "-p", work + "/proof", "-i", work + "/public_inputs", "-t", TARGET)
    ok = v.returncode == 0 and "verified successfully" in (v.stdout + v.stderr)
    return ok, "verified" if ok else "verify rejected"


def attack(job):
    circuit, vk, fmt, stack, idx, stratum, meta = job
    work = tempfile.mkdtemp(prefix="e9w")
    try:
        s = msgpack.unpackb(msgpack.packb(stack, use_bin_type=True), strict_map_key=False, raw=False)
        m = s[0][0][1]
        v = (int.from_bytes(m[idx], "big") + 1) % P
        m[idx] = v.to_bytes(32, "big")
        w = work + "/w.gz"
        write_witness(w, fmt, s)
        try:
            ok, why = prove_verify(circuit, vk, w, work)
        except subprocess.TimeoutExpired:
            ok, why = False, "timeout"
        return dict(meta, witness=idx, stratum=stratum, accepted=ok, why=why)
    finally:
        shutil.rmtree(work, ignore_errors=True)


out = open(out_path, "a")
for d in dirs:
    d = os.path.abspath(d)
    set_name = os.path.basename(d)
    subprocess.run(["nargo", "compile"], cwd=d, capture_output=True)
    circuit = d + "/target/policy.json"
    art = json.load(open(circuit))
    prog = msgpack.unpackb(gzip.decompress(base64.b64decode(art["bytecode"]))[1:], strict_map_key=False, raw=False)
    fn = prog[0][0]
    ops, private, public = fn[1], set(fn[2]), list(fn[3])
    brillig = brillig_outputs(ops)
    vkdir = tempfile.mkdtemp(prefix="e9vk")
    sh("bb", "write_vk", "-b", circuit, "-o", vkdir, "-t", TARGET)
    vk = vkdir + "/vk"
    idx = json.load(open(d + "/inputs/index.json"))
    runnable = [r for r in idx["requests"] if "skipped" not in r]
    picks = [next(r for r in runnable if r["decision"] == x) for x in (True, False) if any(r["decision"] == x for r in runnable)]
    for req in picks:
        shutil.copy(f"{d}/inputs/{req['n']}.toml", d + "/Prover.toml")
        subprocess.run(["nargo", "execute"], cwd=d, capture_output=True, check=True)
        fmt, stack = read_witness(d + "/target/policy.gz")
        m = stack[0][0][1]
        work = tempfile.mkdtemp(prefix="e9h")
        ok, why = prove_verify(circuit, vk, d + "/target/policy.gz", work)
        shutil.rmtree(work, ignore_errors=True)
        if not ok:
            raise SystemExit(f"honest witness rejected: {set_name} {req['name']}: {why}")
        ids = sorted(m)
        pub = [i for i in public if i in m]
        bri = [i for i in ids if i in brillig]
        pri = [i for i in ids if i in private]
        oth = [i for i in ids if i not in brillig and i not in private and i not in public]
        # Public inputs that decide the statement all go in; the rest are sampled.
        share = max(1, sample // 3)
        pick = lambda xs, k: rng.sample(xs, min(k, len(xs)))
        jobs = [(i, "public") for i in pub[-1:] + pick(pub[:-1], share // 2)]
        jobs += [(i, "brillig_output") for i in pick(bri, share)]
        jobs += [(i, "private_input") for i in pick(pri, share // 2)]
        jobs += [(i, "internal") for i in pick(oth, share)]
        meta = {"experiment": "E9", "part": "B", "set": set_name, "request": req["name"], "decision": req["decision"]}
        t0 = time.time()
        with cf.ThreadPoolExecutor(workers) as ex:
            res = list(ex.map(attack, [(circuit, vk, fmt, stack, i, s, meta) for i, s in jobs]))
        for r in res:
            out.write(json.dumps(r) + "\n")
        out.flush()
        acc = [r for r in res if r["accepted"]]
        print(set_name, req["name"], "witnesses", len(m), "brillig", len(bri), "private", len(pri), "jobs", len(res), "accepted", len(acc),
              [(r["witness"], r["stratum"]) for r in acc][:12], f"{time.time()-t0:.0f}s", flush=True)
    shutil.rmtree(vkdir, ignore_errors=True)
