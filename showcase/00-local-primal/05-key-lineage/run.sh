#!/bin/bash
# Run the Key Lineage demo

set -e

cd "$(dirname "$0")"

echo "🧬 Running Key Lineage Demo..."
echo ""

cargo run --release

echo ""
echo "✅ Demo complete!"
echo ""
echo "Key Takeaway: Every key has a traceable history!"
echo "Next: Try 06-btsp-tunnel for secure encrypted connections"

