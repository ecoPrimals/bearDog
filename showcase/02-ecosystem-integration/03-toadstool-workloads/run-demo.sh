#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Define paths
DEMO_DIR="$(dirname "$0")"
BEARDOG_ROOT="$(cd "$DEMO_DIR/../../.." && pwd)"

# Ensure we are in the demo directory
cd "$DEMO_DIR"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🐻🍄 BearDog + Toadstool: Encrypted Compute Workloads Demo"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Step 1: Build the demo
echo "Step 1: Building the demo..."
(cd "$BEARDOG_ROOT" && cargo build --release -p beardog-toadstool-demo)
echo "✅ Demo built successfully!"
echo ""

# Step 2: Run the demo
echo "Step 2: Running the demo..."
"$BEARDOG_ROOT/target/release/beardog-toadstool-demo" \
  --workload workloads/ai_inference.json \
  --config configs/demo.toml
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 Toadstool Encrypted Workloads Demo Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

