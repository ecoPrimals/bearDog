#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

DEMO_NAME="Benchmarking & Performance"
DEMO_BIN="benchmarking-performance"
CONFIG_PATH="configs/demo.toml"
SUITE_PATH="benchmarks/suite.json"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 ${DEMO_NAME} Demo - FINAL DEMO!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo ""
echo "🔨 Building demo..."
# Build the demo in release mode
cargo build --release --bin "${DEMO_BIN}"

echo "✅ Build complete"
echo ""

echo "▶️  Running benchmarks..."
# Run the demo with the specified config and suite
./target/release/"${DEMO_BIN}" --config "${CONFIG_PATH}" --suite "${SUITE_PATH}"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Demo Complete - Phase 4 at 100%!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

