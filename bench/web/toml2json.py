#!/usr/bin/env python3
"""Write inputs/N.json beside each inputs/N.toml in a generated circuit directory."""
import glob, json, sys, tomllib
for f in glob.glob(sys.argv[1] + "/inputs/*.toml"):
    json.dump(tomllib.load(open(f, "rb")), open(f[:-4] + "json", "w"))
