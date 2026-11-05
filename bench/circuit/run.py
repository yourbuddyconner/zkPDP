#!/usr/bin/env python3
"""E2 harness: run the hand-written TinyTodo circuit over the corpus and write results as JSON Lines.

Usage: python3 bench/circuit/run.py <out.jsonl> [repeats]
Needs nargo and bb on PATH. Run `circuit-tool` first to write inputs/.
"""
import json, os, re, shutil, subprocess, sys, time, statistics

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", ".."))
C = os.path.join(HERE, "tinytodo")
out_path, repeats = sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 5


def sh(*a, cwd=None):
    return subprocess.run(a, cwd=cwd, capture_output=True, text=True)


def one(*a):
    return sh(*a).stdout.strip()


idx = json.load(open(os.path.join(C, "inputs", "index.json")))
dirty = bool(one("git", "-C", REPO, "status", "--porcelain", "--", "bench", "corpus"))
header = {
    "experiment": "E2",
    "backend": "circuit-noir",
    "commit": one("git", "-C", REPO, "rev-parse", "HEAD"),
    "tree_dirty": dirty,
    "machine": {
        "cpu": one("sysctl", "-n", "machdep.cpu.brand_string"),
        "cores": one("sysctl", "-n", "hw.ncpu"),
        "ram_gb": int(one("sysctl", "-n", "hw.memsize")) >> 30,
        "os": "macOS " + one("sw_vers", "-productVersion"),
    },
    "toolchain": {"nargo": one("nargo", "--version").splitlines()[0], "bb": one("bb", "--version"),
                  "sha256_lib": "noir-lang/sha256 v0.3.0"},
    "set": idx["set"],
    "circuit": "bench/circuit/tinytodo, hand-written",
}
out = open(out_path, "a")


def line(req, metric, value, repeat=0, status="ok", notes=""):
    l = dict(header)
    l.update({
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "request": req["name"],
        "inputs": {"policy_hash": idx["policy_hash"], "schema_hash": idx["schema_hash"],
                   "entity_commitment": idx["entities"], "request_hash": req["request_hash"]},
        "metric": metric, "value": value, "repeat": repeat, "status": status, "notes": notes,
    })
    out.write(json.dumps(l) + "\n")
    out.flush()


def execute(n, flip=False):
    t = open(os.path.join(C, "inputs", f"{n}.toml")).read()
    if flip:
        t = re.sub(r"^decision = (true|false)", lambda m: "decision = " + ("false" if m.group(1) == "true" else "true"), t, flags=re.M)
    open(os.path.join(C, "Prover.toml"), "w").write(t)
    t0 = time.time()
    r = sh("nargo", "execute", cwd=C)
    return "successfully solved" in r.stdout + r.stderr, time.time() - t0


sh("nargo", "compile", cwd=C)

# Correctness: the circuit accepts the oracle's decision and rejects the opposite one.
ok = bad = 0
for req in idx["requests"]:
    accepts, _ = execute(req["n"])
    rejects_flip = not execute(req["n"], flip=True)[0]
    good = accepts and rejects_flip
    ok += good
    bad += not good
    line(req, "matches_oracle", good, status="ok" if good else "failed",
         notes=f"oracle decision {req['decision']}; accepted {accepts}; opposite decision rejected {rejects_flip}")
print(f"matched oracle: {ok}  failed: {bad}")
if bad:
    sys.exit(1)

req = idx["requests"][0]
g = json.loads(one("bb", "gates", "-b", os.path.join(C, "target/tinytodo.json")))["functions"][0]
line(req, "acir_opcodes", g["acir_opcodes"])
line(req, "gates", g["circuit_size"], notes="bb gates circuit_size")
print("gates", g["circuit_size"])

for i in range(repeats):
    _, secs = execute(req["n"])
    line(req, "witness_time_s", secs, i, notes="nargo execute, includes process start and reading inputs")

RSS = re.compile(r"(\d+)\s+maximum resident set size")
for mode, target in [("zk", "noir-recursive"), ("plain", "noir-recursive-no-zk")]:
    d = os.path.join(C, "target", mode)
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d)
    t0 = time.time()
    r = sh("bb", "write_vk", "-b", "target/tinytodo.json", "-o", f"target/{mode}", "-t", target, cwd=C)
    line(req, f"vk_time_s_{mode}", time.time() - t0, status="ok" if r.returncode == 0 else "failed", notes=r.stderr[-200:] if r.returncode else "once per circuit")
    times = []
    for i in range(repeats):
        t0 = time.time()
        r = sh("/usr/bin/time", "-l", "bb", "prove", "-b", "target/tinytodo.json", "-w", "target/tinytodo.gz",
               "-k", f"target/{mode}/vk", "-o", f"target/{mode}", "-t", target, cwd=C)
        secs = time.time() - t0
        if r.returncode != 0:
            line(req, f"prove_time_s_{mode}", None, i, "failed", r.stderr[-300:])
            continue
        times.append(secs)
        line(req, f"prove_time_s_{mode}", secs, i, notes="bb prove with a precomputed verification key, includes process start")
        m = RSS.search(r.stderr)
        if m:
            line(req, f"peak_memory_gb_{mode}", int(m.group(1)) / 2**30, i)
        line(req, f"proof_bytes_{mode}", os.path.getsize(os.path.join(d, "proof")), i)
        line(req, f"public_inputs_bytes_{mode}", os.path.getsize(os.path.join(d, "public_inputs")), i)
    ms = []
    for k in range(20):
        t0 = time.time()
        r = sh("bb", "verify", "-k", f"target/{mode}/vk", "-p", f"target/{mode}/proof", "-i", f"target/{mode}/public_inputs", "-t", target, cwd=C)
        v = (time.time() - t0) * 1e3
        good = r.returncode == 0 and "verified successfully" in r.stdout + r.stderr
        ms.append(v)
        line(req, f"verify_time_ms_{mode}", v, k, "ok" if good else "failed", "bb verify as a process, includes process start")
    print(f"{mode}: prove median {statistics.median(times):.2f} s, verify median {statistics.median(ms):.0f} ms, proof {os.path.getsize(os.path.join(d, 'proof'))} bytes")
