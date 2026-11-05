#!/bin/zsh
# The final timing pass, on an idle machine. One commit, every backend, TinyTodo and the compiled sets.
# Usage: timing_run.sh <gordiehowe dir> <gordiehowe inputs root> <date tag>
set -e
GH=$1; IN=$2; TAG=$3
export PATH="$HOME/.nargo/bin:$HOME/.bb:$PATH"
R=$(git rev-parse --show-toplevel); cd $R
C=$(git rev-parse HEAD)
date
python3 -I bench/circuit/run.py results/T-noir-hand-$TAG.jsonl 5 bench/circuit/tinytodo E2 "hand-written TinyTodo, record v1"
python3 -I bench/circuit/run.py results/T-noir-signed-$TAG.jsonl 5 bench/circuit/tinytodo_signed E3 "hand-written TinyTodo, signed store, record v1"
for s in tinytodo usecase-github_example usecase-hotel_chains-static usecase-document_cloud usecase-sales_orgs-static; do
  python3 -I bench/circuit/run.py results/T-noir-generated-$TAG.jsonl 5 bench/circuit/generated/$s E4 "generated from $s, record v1"
  $GH/build/gordiehowe/zkpdp_policy bench/circuit/generated/$s/policy.ir $IN/gh_policy/$s results/T-gordiehowe-generated-$TAG.jsonl $s $C 5 | grep -v INFO | tail -1 | cut -c1-140
done
$GH/build/gordiehowe/zkpdp_tinytodo $IN/gh_inputs results/T-gordiehowe-hand-$TAG.jsonl $C 5 | grep -v INFO | tail -1 | cut -c1-140
$GH/build/gordiehowe/zkpdp_tinytodo_signed $IN/gh_inputs_signed results/T-gordiehowe-signed-$TAG.jsonl $C 5 | grep -v INFO | tail -1 | cut -c1-140
date
cd bench
./target/release/zkvm-host prove-records .. ../results/T-zkvm-records-$TAG.jsonl tinytodo 1 1
date
