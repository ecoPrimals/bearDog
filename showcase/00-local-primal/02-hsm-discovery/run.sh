#!/bin/bash
# Run the HSM Discovery demo

set -e

cd "$(dirname "$0")"

echo "🔍 Running HSM Discovery Demo..."
echo ""

cargo run --release

echo ""
echo "✅ Demo complete!"
echo ""
echo "Next: Try 03-key-constraints to create self-enforcing keys"

