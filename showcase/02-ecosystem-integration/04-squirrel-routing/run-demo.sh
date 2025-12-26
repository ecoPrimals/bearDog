#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Define paths
DEMO_DIR="$(dirname "$0")"
BEARDOG_ROOT="$(cd "$DEMO_DIR/../../.." && pwd)"

# Ensure we are in the demo directory
cd "$DEMO_DIR"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🐻🐿️ BearDog + Squirrel: Privacy-Preserving Routing Demo"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Step 1: Build the demo
echo "Step 1: Building the demo..."
(cd "$BEARDOG_ROOT" && cargo build --release -p beardog-squirrel-demo 2>/dev/null || (cd "$DEMO_DIR" && cargo build --release))
echo "✅ Demo built successfully!"
echo ""

# Step 2: Run the demo
echo "Step 2: Running the demo..."
./target/release/beardog-squirrel-demo \
  --request requests/ai_summarization.json \
  --config configs/demo.toml
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 Squirrel Privacy Routing Demo Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

