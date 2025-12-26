#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

DEMO_NAME="Constraint Composition"
DEMO_BIN="constraint-composition"
CONFIG_PATH="configs/demo.toml"
SCENARIO_PATH="scenarios/policy_composition.json"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔗 ${DEMO_NAME} Demo"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

echo ""
echo "🔨 Building demo..."
# Build the demo in release mode
cargo build --release --bin "${DEMO_BIN}"

echo "✅ Build complete"
echo ""

echo "▶️  Running demo..."
# Run the demo with the specified config and scenario
./target/release/"${DEMO_BIN}" --config "${CONFIG_PATH}" --scenario "${SCENARIO_PATH}"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Demo Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

