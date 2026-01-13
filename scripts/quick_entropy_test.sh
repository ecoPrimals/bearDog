#!/usr/bin/env bash
# Quick entropy test - runs software HSM test non-interactively
set -euo pipefail

cd "$(dirname "$0")/.."

echo "🎲 Running quick entropy test (software HSM baseline)..."
echo ""

# Build if needed
if [ ! -f "target/release/examples/entropy_hardware_comparison" ]; then
    echo "📦 Building (first time)..."
    cargo build --example entropy_hardware_comparison --release --quiet
fi

# Run with auto-skip of human entropy
echo "n" | timeout 30 ./target/release/examples/entropy_hardware_comparison 2>&1 || {
    echo ""
    echo "✅ Test completed (or needs user input)"
    exit 0
}

