#!/bin/bash
# Quick Metrics Check - Daily Health Status
# Usage: ./scripts/quick_metrics.sh

set -e

echo "========================================"
echo "🐻 BearDog Quick Metrics - $(date +%Y-%m-%d)"
echo "========================================"
echo ""

# Test Status
echo "📊 Test Status:"
cargo test --workspace --quiet 2>&1 | grep "test result" || echo "  ❌ Tests failed"
echo ""

# Build Status
echo "🔨 Build Status:"
if cargo build --workspace --quiet 2>&1 > /dev/null; then
    echo "  ✅ Clean build"
else
    echo "  ❌ Build failed"
fi
echo ""

# Formatting
echo "📝 Formatting:"
if cargo fmt --all -- --check 2>&1 > /dev/null; then
    echo "  ✅ All files formatted"
else
    echo "  ⚠️  Some files need formatting"
fi
echo ""

# Clippy
echo "🔍 Clippy:"
CLIPPY_WARNINGS=$(cargo clippy --workspace --quiet -- -D warnings 2>&1 | grep -c "warning\|error" || true)
if [ "$CLIPPY_WARNINGS" -eq 0 ]; then
    echo "  ✅ No warnings"
else
    echo "  ⚠️  $CLIPPY_WARNINGS warnings/errors"
fi
echo ""

# TODOs
echo "📋 TODOs:"
TODO_COUNT=$(grep -r "TODO\|FIXME\|HACK" crates/ --include="*.rs" 2>/dev/null | wc -l || echo "0")
echo "  📝 $TODO_COUNT items remaining"
echo ""

# Clones
echo "🔄 Clone Usage:"
CLONE_COUNT=$(grep -r "\.clone()" crates/ --include="*.rs" 2>/dev/null | wc -l || echo "0")
echo "  🔗 $CLONE_COUNT clone() calls"
echo ""

# File Sizes
echo "📏 File Sizes:"
OVERSIZED=$(find crates -name "*.rs" -exec wc -l {} \; 2>/dev/null | awk '$1 > 1000 {count++} END {print count+0}')
echo "  📦 $OVERSIZED files over 1000 lines"
echo ""

echo "========================================"
echo "✅ Quick metrics complete!"
echo "========================================"

