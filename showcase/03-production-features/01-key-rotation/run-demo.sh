#!/bin/bash

set -e

DEMO_DIR="$(dirname "$0")"
BEARDOG_ROOT="$(cd "$DEMO_DIR/../../.." && pwd)"

cd "$DEMO_DIR"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔄 BearDog: Automated Key Rotation Demo"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Building demo..."
(cd "$BEARDOG_ROOT" && cargo build --release -p beardog-key-rotation-demo 2>/dev/null || (cd "$DEMO_DIR" && cargo build --release))
echo "✅ Build complete!"
echo ""

echo "Running demo..."
./target/release/beardog-key-rotation-demo \
  --scenario scenarios/rotation_workflow.json \
  --config configs/demo.toml
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 Key Rotation Demo Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

