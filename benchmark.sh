#!/bin/bash
# Benchmark dotsh across shells and input sizes

set -e

echo "Building release binary..."
cargo build --release

echo ""
echo "=== Benchmark: small input (3 vars) ==="
hyperfine --warmup 3 \
	"dotsh bash 'A=1' 'B=2' 'C=3'" \
	"dotsh pwsh 'A=1' 'B=2' 'C=3'" \
	"dotsh fish 'A=1' 'B=2' 'C=3'" \
	"dotsh cmd 'A=1' 'B=2' 'C=3'"

echo ""
echo "=== Benchmark: medium input (20 vars) ==="
MEDIUM_INPUT="$(seq 1 20 | xargs -I{} echo "KEY{}=value{}")"
hyperfine --warmup 3 \
	"dotsh bash $MEDIUM_INPUT" \
	"dotsh pwsh $MEDIUM_INPUT" \
	"dotsh fish $MEDIUM_INPUT" \
	"dotsh cmd $MEDIUM_INPUT"

echo ""
echo "=== Benchmark: large input (100 vars) ==="
LARGE_INPUT="$(seq 1 100 | xargs -I{} echo "KEY{}=value{}")"
hyperfine --warmup 3 \
	"dotsh bash $LARGE_INPUT" \
	"dotsh pwsh $LARGE_INPUT" \
	"dotsh fish $LARGE_INPUT" \
	"dotsh cmd $LARGE_INPUT"

echo ""
echo "=== Benchmark: Windows paths (10 vars) ==="
WIN_INPUT=""
for i in $(seq 1 10); do
	WIN_INPUT="$WIN_INPUT 'PATH$i=C:\\Users\\Name\\AppData\\Local\\mise\\installs\\tool\\$i.0.0'"
done
hyperfine --warmup 3 \
	"dotsh bash $WIN_INPUT" \
	"dotsh pwsh $WIN_INPUT" \
	"dotsh fish $WIN_INPUT" \
	"dotsh cmd $WIN_INPUT"

echo ""
echo "=== Benchmark: vs baseline (bash echo) ==="
hyperfine --warmup 10 \
	"bash -c 'echo export A=1'" \
	"dotsh bash 'A=1'"
