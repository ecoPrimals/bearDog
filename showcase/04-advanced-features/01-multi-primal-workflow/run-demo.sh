#!/bin/bash
# 🌐 Multi-Primal Workflow Demo Runner

set -e

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🌐 Multi-Primal Workflow Demo"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if scenario argument provided
SCENARIO="${1:-scenarios/medical_analysis.json}"
CONFIG="${2:-configs/demo.toml}"

if [ ! -f "$SCENARIO" ]; then
    echo "❌ Scenario file not found: $SCENARIO"
    echo ""
    echo "Usage: $0 [scenario.json] [config.toml]"
    echo "Example: $0 scenarios/medical_analysis.json configs/demo.toml"
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

# Set up discovery environment (fallback if no real services)
echo "🔍 Setting up service discovery environment..."
export RUST_LOG="${RUST_LOG:-info}"

# Optional: Set environment variables for real service discovery
# Uncomment and modify these if you have actual services running:
# export PRIMAL_ORCHESTRATION_ENDPOINT="http://localhost:9090"
# export PRIMAL_ORCHESTRATION_CAPABILITIES="orchestration,federation"
# export PRIMAL_AI_ENDPOINT="http://localhost:6060"
# export PRIMAL_AI_CAPABILITIES="ai,privacy"
# export PRIMAL_STORAGE_ENDPOINT="http://localhost:8080"
# export PRIMAL_STORAGE_CAPABILITIES="storage,encryption"
# export PRIMAL_COMPUTE_ENDPOINT="http://localhost:7070"
# export PRIMAL_COMPUTE_CAPABILITIES="compute,gpu"

echo "✅ Discovery configured (using fallback endpoints for demo)"
echo ""

# Run the demo
echo "🚀 Running Multi-Primal Workflow Demo..."
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

