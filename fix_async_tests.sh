#!/bin/bash
# Automated Test Suite Repair Script
# Fixes async keyword issues in tokio tests

set -e
echo "🔧 BearDog Test Suite Repair"
echo "=============================="
echo ""

# Step 1: Backup (if not already done)
echo "📦 Checking for backup..."
if [ ! -d "tests_NEEDS_FIXING_BACKUP" ]; then
    echo "Creating backup..."
    cp -r tests_NEEDS_FIXING tests_NEEDS_FIXING_BACKUP
    echo "✅ Backup created: tests_NEEDS_FIXING_BACKUP/"
else
    echo "⚠️  Backup already exists, skipping"
fi
echo ""

# Step 2: Fix async keywords in tokio tests
echo "🔧 Fixing async keywords in tokio tests..."
FIXED_COUNT=0

# Find all .rs files and fix the pattern
find tests_NEEDS_FIXING -name "*.rs" -type f | while read file; do
    # Use perl for multi-line regex to add async after #[tokio::test]
    if perl -i -0pe 's/(#\[tokio::test(?:\([^\)]*\))?\]\s*\n\s*)fn /${1}async fn /g' "$file"; then
        FIXED_COUNT=$((FIXED_COUNT + 1))
    fi
done

echo "✅ Processed files in tests_NEEDS_FIXING/"
echo ""

# Step 3: Count how many test functions we fixed
ASYNC_COUNT=$(find tests_NEEDS_FIXING -name "*.rs" -exec grep -l "#\[tokio::test" {} \; | wc -l)
echo "📊 Files with tokio tests: $ASYNC_COUNT"
echo ""

# Step 4: Summary
echo "✅ Test repair phase 1 complete!"
echo ""
echo "📋 Next steps:"
echo "   1. Run: cargo check --tests"
echo "   2. Review compilation errors"
echo "   3. Enable fixed tests gradually"
echo ""
echo "📁 Files:"
echo "   - Original backup: tests_NEEDS_FIXING_BACKUP/"
echo "   - Working files: tests_NEEDS_FIXING/"
