set -e

cargo build --release
./run_stress.sh
./scripts/test_bfpp.sh

