#!/bin/bash
# Test Repairs Verification Script
# Date: October 6, 2025
# Purpose: Test if repaired test files compile

set -e

echo "🧪 Testing Repaired Test Files"
echo "=============================="
echo ""

PASSED=0
FAILED=0
FAILED_FILES=()

# Test each repaired file
for file in tests_NEEDS_FIXING/*.rs; do
    filename=$(basename "$file" .rs)
    
    # Skip if it's a module file
    if [[ "$filename" == "mod" ]] || [[ "$filename" == "common" ]]; then
        continue
    fi
    
    echo -n "Testing $filename... "
    
    # Try to compile the test
    if cargo test --test "$filename" --no-run 2>/dev/null >/dev/null; then
        echo "✅ PASS"
        PASSED=$((PASSED + 1))
    else
        echo "❌ FAIL"
        FAILED=$((FAILED + 1))
        FAILED_FILES+=("$filename")
    fi
done

echo ""
echo "=============================="
echo "📊 Test Compilation Summary"
echo "=============================="
echo "Passed: $PASSED"
echo "Failed: $FAILED"
echo ""

if [ $FAILED -gt 0 ]; then
    echo "❌ Failed files:"
    for file in "${FAILED_FILES[@]}"; do
        echo "  - $file"
    done
    echo ""
    echo "💡 These files need manual review"
    exit 1
else
    echo "✅ All test files compile successfully!"
    echo ""
    echo "📝 Next step: Run './scripts/move_fixed_tests.sh' to move tests back to tests/"
fi

