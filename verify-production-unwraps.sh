#!/bin/bash
# Production Unwrap Verification Script
# Distinguishes between test code and production code unwraps

echo "=== PRODUCTION UNWRAP VERIFICATION ==="
echo "Date: $(date)"
echo ""
echo "Analyzing unwraps in production code (excluding tests)..."
echo ""

total_prod=0
total_test=0

for file in $(find crates -name "*.rs" -type f | grep -v "/tests/" | sort); do
    # Skip test files
    if [[ "$file" == *"_tests.rs" ]] || [[ "$file" == *"_test.rs" ]]; then
        continue
    fi
    
    # Find test section start
    test_line=$(grep -n "#\[cfg(test)\]" "$file" | head -1 | cut -d: -f1)
    
    if [ -n "$test_line" ]; then
        # Count unwraps before test section (production)
        prod_unwraps=$(head -n "$test_line" "$file" | grep -c "\.unwrap()\|\.expect(" 2>/dev/null || echo 0)
        
        # Count unwraps in test section
        total_unwraps=$(grep -c "\.unwrap()\|\.expect(" "$file" 2>/dev/null || echo 0)
        test_unwraps=$((total_unwraps - prod_unwraps))
        
        if [ "$prod_unwraps" -gt 0 ]; then
            echo "⚠️  $file: $prod_unwraps production unwraps ($test_unwraps in tests)"
            total_prod=$((total_prod + prod_unwraps))
            total_test=$((total_test + test_unwraps))
        elif [ "$test_unwraps" -gt 0 ]; then
            echo "✅ $file: 0 production unwraps ($test_unwraps in tests - OK)"
            total_test=$((total_test + test_unwraps))
        fi
    else
        # No test section, count all (but verify manually)
        total=$(grep -c "\.unwrap()\|\.expect(" "$file" 2>/dev/null || echo 0)
        
        if [ "$total" -gt 0 ]; then
            echo "❓ $file: $total unwraps (no test section - VERIFY MANUALLY)"
            total_prod=$((total_prod + total))
        fi
    fi
done

echo ""
echo "=== SUMMARY ==="
echo "Production unwraps: $total_prod"
echo "Test unwraps: $total_test"
echo "Total unwraps: $((total_prod + total_test))"
echo ""
echo "✅ Test unwraps are acceptable"
echo "⚠️  Production unwraps need fixing"
echo ""
echo "Impact: $(echo "scale=1; $total_test * 100 / ($total_prod + $total_test)" | bc)% of unwraps are in acceptable test code!"

