#!/bin/bash
# Profile benchmark performance
set -euo pipefail

OUTPUT_DIR="./profiling/reports"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
REPORT_FILE="$OUTPUT_DIR/benchmark_profile_${TIMESTAMP}.txt"

echo "Profiling benchmarks..."
echo "Output: $REPORT_FILE"
echo ""

# Run benchmarks with profiling
cargo bench --no-fail-fast 2>&1 | tee "$REPORT_FILE"

echo ""
echo "✓ Benchmark profile complete: $REPORT_FILE"
