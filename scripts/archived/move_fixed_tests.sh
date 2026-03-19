#!/bin/bash
# Move Fixed Tests Script
# Date: October 6, 2025  
# Purpose: Move successfully compiled tests from tests_NEEDS_FIXING/ to tests/

set -e

echo "📦 Moving Fixed Test Files"
echo "=============================="
echo ""

MOVED=0
SKIPPED=0

# Test and move each file
for file in tests_NEEDS_FIXING/*.rs; do
    filename=$(basename "$file")
    test_name="${filename%.rs}"
    
    # Skip module files
    if [[ "$test_name" == "mod" ]] || [[ "$test_name" == "common" ]]; then
        continue
    fi
    
    echo -n "Checking $filename... "
    
    # Try to compile
    if cargo test --test "$test_name" --no-run 2>/dev/null >/dev/null; then
        # Check if file already exists in tests/
        if [ -f "tests/$filename" ]; then
            echo "⚠️  Already exists in tests/, skipping"
            SKIPPED=$((SKIPPED + 1))
        else
            mv "$file" "tests/$filename"
            echo "✅ Moved to tests/"
            MOVED=$((MOVED + 1))
        fi
    else
        echo "❌ Still failing, leaving in tests_NEEDS_FIXING/"
        SKIPPED=$((SKIPPED + 1))
    fi
done

echo ""
echo "=============================="
echo "📊 Summary"
echo "=============================="
echo "Moved to tests/: $MOVED"
echo "Left in tests_NEEDS_FIXING/: $SKIPPED"
echo ""

if [ $MOVED -gt 0 ]; then
    echo "✅ Successfully moved $MOVED test files!"
    echo ""
    echo "📝 Next steps:"
    echo "1. Run: cargo test --workspace"
    echo "2. Check coverage: cargo tarpaulin --out Html"
    echo "3. Commit changes: git add tests/ tests_NEEDS_FIXING/"
else
    echo "ℹ️  No files were moved"
fi

