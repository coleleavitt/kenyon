#!/usr/bin/env bash
# Regenerate every number in RESULTS.md and research/wiring/PAPER.md.
# All randomness is seeded inside the binaries; re-running gives the same
# results up to floating-point summation order in the LeNet5 encoder (rayon /
# gemm threading). Needs: datasets (scripts/download_datasets.sh) and the
# FlyWire v783 CSVs at $KENYON_FLYWIRE (default ~/flywire-data/fafb/783).
# Runtime on a 24-core box: ~1.5 h, dominated by the encoder (25 epochs).
set -euo pipefail
cd "$(dirname "$0")/.."
export KENYON_OUT="${KENYON_OUT:-results}"
mkdir -p "$KENYON_OUT"

scripts/download_datasets.sh
timeout 3600 cargo build --release -p kenyon-repro --bins

B=target/release
echo "== R1 FlyHash vs LSH (MNIST, 50 trials)"
timeout 3600 $B/r1_flyhash 50 > "$KENYON_OUT/r1_flyhash.md" 2> "$KENYON_OUT/r1_flyhash.log"
echo "== R2 fly Bloom filter (SIFT, odours)"
timeout 5400 $B/r2_bloom sift > "$KENYON_OUT/r2_bloom_sift.md" 2> "$KENYON_OUT/r2_bloom_sift.log"
timeout 1800 $B/r2_bloom odors > "$KENYON_OUT/r2_bloom_odors.md" 2> "$KENYON_OUT/r2_bloom_odors.log"
echo "== R3 encoder (LeNet5 on KMNIST, seed 0, 25 epochs) + FlyModel"
if [ ! -s "${KENYON_DATA:-$HOME/datasets}/kenyon-features/lenet5_kmnist_s0/mnist_test.f32" ] || [ "${KENYON_RETRAIN:-0}" = 1 ]; then
  timeout 7200 $B/encoder 0 25 > "$KENYON_OUT/encoder_s0.log" 2>&1
fi
timeout 7200 $B/r3_flymodel 0 > "$KENYON_OUT/r3_flymodel.md" 2> "$KENYON_OUT/r3_flymodel.log"
echo "== Wiring study (20 seeds)"
timeout 3600 $B/wiring 20 > "$KENYON_OUT/wiring.md" 2> "$KENYON_OUT/wiring.log"
timeout 1800 $B/flyrag_eval 20 > "$KENYON_OUT/flyrag.md" 2> "$KENYON_OUT/flyrag.log"
timeout 1800 $B/flyrag_followup dev > /dev/null
timeout 1800 $B/flyrag_followup eval 0.10 4 > /dev/null
timeout 3000 $B/flyrag_scifact 20 > /dev/null 2> "$KENYON_OUT/flyrag_scifact.log"   # needs $KENYON_DATA/scifact
timeout 5400 $B/hnsw_vs_fly > "$KENYON_OUT/hnsw_vs_fly.md" 2> "$KENYON_OUT/hnsw_vs_fly.log"
KENYON_CONNECTOME=mcns timeout 7000 $B/wiring 20 > "$KENYON_OUT/wiring_mcns.md" 2> "$KENYON_OUT/wiring_mcns.log"
echo "== Memory"
$B/memory > "$KENYON_OUT/memory.md"
echo "== Throughput (criterion; single thread)"
timeout 3600 cargo bench -p kenyon --bench throughput -- --noplot 2>&1 | tee "$KENYON_OUT/bench.txt" | grep -E "^[a-z_]+/|time:|thrpt:" || true
echo "done: see $KENYON_OUT/"
