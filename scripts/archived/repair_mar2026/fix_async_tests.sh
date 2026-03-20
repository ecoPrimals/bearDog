#!/bin/bash
# BearDog Test Suite Repair - Async Keyword Fixer
# Created: October 4, 2025

set -e  # Exit on error

echo "🔧 BearDog Test Suite Repair"
echo "=============================="
echo ""

# Check if backup exists
if [ ! -d "tests_NEEDS_FIXING_BACKUP" ]; then
    echo "❌ ERROR: Backup not found!"
    echo "   Run: cp -r tests_NEEDS_FIXING tests_NEEDS_FIXING_BACKUP"
    exit 1
fi

echo "✅ Backup verified: tests_NEEDS_FIXING_BACKUP/"
echo ""

# Fix async keywords
echo "🔧 Fixing async keywords in tokio tests..."
echo "   Pattern: #[tokio::test] followed by fn (without async)"
echo ""

FIXED_COUNT=0

find tests_NEEDS_FIXING -name "*.rs" -type f | while read file; do
    # Count before
    BEFORE=$(grep -c "^fn " "$file" 2>/dev/null || echo "0")
    
    # Fix pattern: #[tokio::test] on one line, then fn on next line (without async)
    # Use perl for better multi-line handling
    perl -i -0pe 's/(#\[tokio::test\])\s*\n(\s*)fn /${1}\n${2}async fn /g' "$file"
    
    # Count after (rough estimate)
    AFTER=$(grep -c "^async fn " "$file" 2>/dev/null || echo "0")
    
    if [ "$AFTER" -gt "0" ]; then
        echo "  ✓ $file"
    fi
done

echo ""
echo "✅ Async keyword fixes applied to all .rs files"
echo ""

# Verify with cargo
echo "🏗️  Verifying compilation..."
echo "   (This may take a moment...)"
echo ""

if cargo check --tests --quiet 2>&1 | tee test_check.log | grep -q "Finished"; then
    echo "✅ Compilation check complete!"
    echo ""
    echo "📊 Results:"
    grep -c "error" test_check.log || echo "   No compilation errors found!"
    echo ""
else
    echo "⚠️  Some compilation issues remain"
    echo "   See test_check.log for details"
    echo ""
    echo "🔍 Error summary:"
    grep "^error" test_check.log | head -10 || echo "   (Run 'cat test_check.log' for full output)"
    echo ""
fi

# Count fixed files
ASYNC_TEST_COUNT=$(find tests_NEEDS_FIXING -name "*.rs" -exec grep -l "#\[tokio::test\]" {} \; | wc -l)
echo "📊 Summary:"
echo "   Files with tokio tests: $ASYNC_TEST_COUNT"
echo "   Backup location: tests_NEEDS_FIXING_BACKUP/"
echo "   Fixed files: tests_NEEDS_FIXING/"
echo "   Verification log: test_check.log"
echo ""
echo "✅ Test repair phase 1 complete!"
echo ""
echo "📋 Next steps:"
echo "   1. Review test_check.log for remaining issues"
echo "   2. Fix any remaining import/type errors manually"
echo "   3. Run specific test to verify: cargo test --test <test_name>"
echo ""

