#!/bin/bash

set -e

DEMO_DIR="$(dirname "$0")"
BEARDOG_ROOT="$(cd "$DEMO_DIR/../../.." && pwd)"

cd "$DEMO_DIR"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🛡️ BearDog: Runtime Policy Enforcement Demo"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Building demo..."
(cd "$BEARDOG_ROOT" && cargo build --release -p beardog-policy-demo 2>/dev/null || (cd "$DEMO_DIR" && cargo build --release))
echo "✅ Build complete!"
echo ""

echo "Running demo..."
./target/release/beardog-policy-demo \
  --policies policies/demo_policies.json \
  --config configs/demo.toml
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 Policy Enforcement Demo Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

