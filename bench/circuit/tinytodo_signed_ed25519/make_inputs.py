#!/usr/bin/env python3
"""Re-sign bench/circuit/tinytodo_signed's inputs with an Ed25519 test authority.

The key is derived from a public string, like the P-256 test authority, so anyone can reproduce it
and it protects nothing. Usage: make_inputs.py
"""
import hashlib, json, os, shutil, sys, tomllib
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "ed25519_root"))
import gen
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from cryptography.hazmat.primitives import serialization

SRC = os.path.join(HERE, "..", "tinytodo_signed", "inputs")
OUT = os.path.join(HERE, "inputs")
key = Ed25519PrivateKey.from_private_bytes(hashlib.sha256(b"zk-pdp test authority ed25519 v0").digest())
pk = key.public_key().public_bytes(serialization.Encoding.Raw, serialization.PublicFormat.Raw)
pk_x, _ = gen.decode(pk)
os.makedirs(OUT, exist_ok=True)
shutil.copy(os.path.join(SRC, "index.json"), os.path.join(OUT, "index.json"))
n = 0
for f in sorted(os.listdir(SRC)):
    if not f.endswith(".toml"):
        continue
    text = open(os.path.join(SRC, f)).read()
    t = tomllib.loads(text)
    msg = gen.TAG + bytes(t["entities_root"]) + int(t["epoch"]).to_bytes(8, "big")
    sig = key.sign(msg)
    key.public_key().verify(sig, msg)
    r_x, _ = gen.decode(sig[:32])
    keep = [l for l in text.splitlines() if not l.startswith(("authority_x", "authority_y", "signature"))]
    q = lambda xs: "[" + ", ".join(f'"{x}"' for x in xs) + "]"
    head = [f"authority_pk = {list(pk)}", f"signature = {list(sig)}", f"pk_x = {q(gen.limbs(pk_x))}", f"r_x = {q(gen.limbs(r_x))}"]
    open(os.path.join(OUT, f), "w").write("\n".join(head + keep) + "\n")
    n += 1
print("re-signed", n, "inputs; authority key", pk.hex())
