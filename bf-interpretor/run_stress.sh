#!/usr/bin/env bash
set -e
cargo build --release
./target/release/bf programs/stress/big_output.bf > /tmp/out1.txt
./target/release/bf --tape 200000 programs/stress/memory_walk.bf > /tmp/out2.txt
./target/release/bf programs/stress/loop_heavy.bf > /tmp/out3.txt
wc -c /tmp/out1.txt /tmp/out2.txt /tmp/out3.txt
