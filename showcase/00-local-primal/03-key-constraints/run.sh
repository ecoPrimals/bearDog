#!/bin/bash
# Run the Key Constraints demo

set -e

cd "$(dirname "$0")"

echo "🧬 Running Key Constraints Demo..."
echo ""

cargo run --release

echo ""
echo "✅ Demo complete!"
echo ""
echo "Key Takeaway: Genetic keys self-enforce their own rules!"
echo "Next: Try 04-entropy-mixing to add your own entropy"

