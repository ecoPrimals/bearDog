#!/bin/bash
# 🔐 Threshold Key Shares Demo Runner

set -e

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔐 Threshold Key Shares Demo"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if scenario argument provided
SCENARIO="${1:-scenarios/corporate_treasury.json}"
CONFIG="${2:-configs/demo.toml}"

if [ ! -f "$SCENARIO" ]; then
    echo "❌ Scenario file not found: $SCENARIO"
    echo ""
    echo "Usage: $0 [scenario.json] [config.toml]"
    echo "Example: $0 scenarios/corporate_treasury.json configs/demo.toml"
    exit 1
fi

if [ ! -f "$CONFIG" ]; then
    echo "❌ Config file not found: $CONFIG"
    exit 1
fi

echo "📋 Scenario: $SCENARIO"
echo "⚙️  Config: $CONFIG"
echo ""

# Build the demo
echo "🔨 Building demo..."
cargo build --release 2>&1 | tail -5
echo ""

# Set up environment
echo "🔧 Setting up environment..."
export RUST_LOG="${RUST_LOG:-info}"
echo "✅ Environment configured"
echo ""

# Run the demo
echo "🚀 Running Threshold Key Shares Demo..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cargo run --release -- \
    --scenario "$SCENARIO" \
    --config "$CONFIG" \
    "$@"

EXIT_CODE=$?

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ Demo completed successfully!"
else
    echo "❌ Demo failed with exit code $EXIT_CODE"
fi
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

exit $EXIT_CODE

