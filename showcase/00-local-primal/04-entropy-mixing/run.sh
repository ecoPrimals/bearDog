#!/bin/bash
# Run the Entropy Mixing demo

set -e

cd "$(dirname "$0")"

echo "🌊 Running Entropy Mixing Demo..."
echo ""
echo "⚠️  Note: This demo simulates human entropy for educational purposes."
echo "⚠️  Production code must use real human input collection!"
echo ""

cargo run --release

echo ""
echo "✅ Demo complete!"
echo ""
echo "Key Takeaway: Mix machine quality + human uniqueness!"
echo "Next: Try 05-key-lineage to track key ancestry"
echo ""
echo "📚 Read: ../../ENTROPY_HIERARCHY_PRINCIPLE.md (essential!)"

