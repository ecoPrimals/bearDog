#!/bin/bash
# Run the Hello BearDog demo

set -e

echo "🐻 Running Hello BearDog Demo..."
echo ""

cd "$(dirname "$0")"
cargo run --release

echo ""
echo "✅ Demo complete!"

