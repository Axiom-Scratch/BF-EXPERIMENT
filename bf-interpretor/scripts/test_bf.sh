set -euo pipefail

cargo test
cargo build --release

./target/release/bf programs/stress/big_output.bf > /tmp/bf_big_output.out
./target/release/bf --max-steps 300000000 programs/stress/loop_heavy.bf > /dev/null
./target/release/bf programs/stress/memory_walk.bf > /tmp/bf_memory_walk.out

./target/release/bf programs/tests/pure_A.bf > /tmp/bf_pure_A.out
xxd -p /tmp/bf_pure_A.out | tr -d '\n' | grep -q '41'

printf 'AB' > /tmp/bf_expected_ab
cmp /tmp/bf_memory_walk.out /tmp/bf_expected_ab

bytes=$(wc -c < /tmp/bf_big_output.out)
if [ "$bytes" -le 0 ]; then
  exit 1
fi

echo "PASS"
