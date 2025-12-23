#!/bin/bash
# Profile memory usage with valgrind
set -euo pipefail

if ! command -v valgrind &> /dev/null; then
    echo "Error: valgrind not found"
    echo "Install with: sudo apt-get install valgrind"
    exit 1
fi

PACKAGE="${1:-beardog-core}"
OUTPUT_DIR="./profiling/reports"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
REPORT_FILE="$OUTPUT_DIR/memory_profile_${PACKAGE}_${TIMESTAMP}.txt"

echo "Profiling memory for $PACKAGE..."
echo "Output: $REPORT_FILE"
echo "This may take several minutes..."
echo ""

# Build test binary
cargo build --package "$PACKAGE" --tests

# Find test binary
TEST_BINARY=$(find target/debug/deps -name "${PACKAGE//-/_}*" -type f -executable | head -1)

if [ -z "$TEST_BINARY" ]; then
    echo "Error: Could not find test binary for $PACKAGE"
    exit 1
fi

# Run valgrind
valgrind \
    --tool=massif \
    --massif-out-file="$OUTPUT_DIR/massif.out.${TIMESTAMP}" \
    --stacks=yes \
    "$TEST_BINARY" \
    2>&1 | tee "$REPORT_FILE"

echo ""
echo "✓ Memory profile complete: $REPORT_FILE"
echo "  Massif output: $OUTPUT_DIR/massif.out.${TIMESTAMP}"
echo "  Analyze with: ms_print $OUTPUT_DIR/massif.out.${TIMESTAMP}"
