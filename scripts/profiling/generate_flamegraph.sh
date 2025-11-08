#!/bin/bash
# Generate flamegraph for a specific package or binary
set -euo pipefail

PACKAGE="${1:-beardog-core}"
OUTPUT_DIR="./profiling/flamegraphs"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

echo "Generating flamegraph for $PACKAGE..."
echo "This may take a few minutes..."

# Build with debug symbols
export CARGO_PROFILE_RELEASE_DEBUG=true

# Generate flamegraph
cargo flamegraph \
    --package "$PACKAGE" \
    --release \
    --output "$OUTPUT_DIR/${PACKAGE}_${TIMESTAMP}.svg" \
    -- --bench

echo "✓ Flamegraph generated: $OUTPUT_DIR/${PACKAGE}_${TIMESTAMP}.svg"
echo "  Open in browser to view"
