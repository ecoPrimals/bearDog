#!/bin/bash

set -e

DEMO_DIR="$(dirname "$0")"
BEARDOG_ROOT="$(cd "$DEMO_DIR/../../.." && pwd)"

cd "$DEMO_DIR"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 BearDog: Monitoring Integration Demo"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Building demo..."
(cd "$BEARDOG_ROOT" && cargo build --release -p beardog-monitoring-demo 2>/dev/null || (cd "$DEMO_DIR" && cargo build --release))
echo "✅ Build complete!"
echo ""

echo "Running demo..."
./target/release/beardog-monitoring-demo \
  --config configs/demo.toml
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 Monitoring Integration Demo Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

