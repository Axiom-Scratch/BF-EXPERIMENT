#!/usr/bin/env bash
set -euo pipefail

cargo build >/dev/null

mkdir -p programs/bfpp /tmp

printf 'hello BFPP\n+++.>--[[]]\n' > programs/bfpp/pass_through.bfpp
./target/debug/bfpp programs/bfpp/pass_through.bfpp -o /tmp/out.bf
cmp programs/bfpp/pass_through.bfpp /tmp/out.bf

printf '+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.\n' > programs/bfpp/prints_A.bfpp
./target/debug/bfpp programs/bfpp/prints_A.bfpp -o /tmp/prints_A.bf
./target/debug/bf /tmp/prints_A.bf | xxd | rg '41' >/dev/null

echo "bfpp tests OK"
