#!/bin/bash
# Run the BTSP Tunnel demo

set -e

cd "$(dirname "$0")"

echo "🔒 Running BTSP Tunnel Demo..."
echo ""

cargo run --release

echo ""
echo "✅ Demo complete!"
echo ""
echo "🎉 Congratulations! Level 0 Complete!"
echo ""
echo "You've mastered BearDog basics:"
echo "  ✓ Key generation"
echo "  ✓ HSM discovery"
echo "  ✓ Key constraints"
echo "  ✓ Entropy mixing"
echo "  ✓ Key lineage"
echo "  ✓ BTSP tunnels"
echo ""
echo "Next: Try Level 1 (Hardware Integration) for real HSM usage!"

