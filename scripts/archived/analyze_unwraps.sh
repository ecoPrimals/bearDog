#!/bin/bash
# Analyze unwrap/expect usage in codebase
# Distinguishes between test code and production code

echo "=== BEARDOG UNWRAP/EXPECT ANALYSIS ==="
echo "Date: $(date)"
echo

# Total counts
echo "📊 TOTAL COUNTS:"
total_unwrap=$(grep -r "\.unwrap()" crates --include="*.rs" 2>/dev/null | wc -l)
total_expect=$(grep -r "\.expect(" crates --include="*.rs" 2>/dev/null | wc -l)
echo "  Total unwrap():        $total_unwrap"
echo "  Total expect():        $total_expect"
echo "  TOTAL:                 $((total_unwrap + total_expect))"
echo

# Production code (exclude tests, examples, benches)
echo "🏭 PRODUCTION CODE:"
prod_unwrap=$(find crates -name "*.rs" -path "*/src/*" \
  ! -path "*/tests/*" ! -path "*/test/*" ! -path "*_tests.rs" \
  ! -path "*/examples/*" ! -path "*/benches/*" \
  -exec grep -c "\.unwrap()" {} \; 2>/dev/null | awk '{s+=$1} END {print s}')
prod_expect=$(find crates -name "*.rs" -path "*/src/*" \
  ! -path "*/tests/*" ! -path "*/test/*" ! -path "*_tests.rs" \
  ! -path "*/examples/*" ! -path "*/benches/*" \
  -exec grep -c "\.expect(" {} \; 2>/dev/null | awk '{s+=$1} END {print s}')
echo "  Production unwrap():   ${prod_unwrap:-0}"
echo "  Production expect():   ${prod_expect:-0}"
echo "  PRODUCTION TOTAL:      $((${prod_unwrap:-0} + ${prod_expect:-0}))"
echo

# Test code
echo "🧪 TEST CODE:"
test_unwrap=$((total_unwrap - ${prod_unwrap:-0}))
test_expect=$((total_expect - ${prod_expect:-0}))
echo "  Test unwrap():         $test_unwrap"
echo "  Test expect():         $test_expect"
echo "  TEST TOTAL:            $((test_unwrap + test_expect))"
echo

# Breakdown by crate (top 10)
echo "📦 TOP 10 CRATES (Production Code):"
echo "Crate                           unwrap() expect() TOTAL"
echo "-------------------------------------------------------------"
for crate_dir in crates/beardog-*/src; do
    crate_name=$(basename $(dirname "$crate_dir"))
    unwrap_count=$(find "$crate_dir" -name "*.rs" \
      ! -path "*/tests/*" ! -path "*/test/*" ! -path "*_tests.rs" \
      -exec grep -c "\.unwrap()" {} \; 2>/dev/null | awk '{s+=$1} END {print s+0}')
    expect_count=$(find "$crate_dir" -name "*.rs" \
      ! -path "*/tests/*" ! -path "*/test/*" ! -path "*_tests.rs" \
      -exec grep -c "\.expect(" {} \; 2>/dev/null | awk '{s+=$1} END {print s+0}')
    total=$((unwrap_count + expect_count))
    if [ "$total" -gt 0 ]; then
        printf "%-32s %6d    %6d   %6d\n" "$crate_name" "$unwrap_count" "$expect_count" "$total"
    fi
done | sort -k4 -rn | head -10
echo

# Summary
echo "=== SUMMARY ==="
prod_total=$((${prod_unwrap:-0} + ${prod_expect:-0}))
test_total=$((test_unwrap + test_expect))
prod_percent=$((prod_total * 100 / (prod_total + test_total)))

echo "Production calls:      $prod_total ($prod_percent%)"
echo "Test calls:            $test_total ($((100 - prod_percent))%)"
echo
echo "✅ Test code can keep unwrap/expect (acceptable)"
echo "⚠️  Production code should be reviewed and fixed"
echo

# Priority assessment
if [ "$prod_total" -lt 100 ]; then
    echo "🟢 Status: EXCELLENT - Very few production unwraps"
elif [ "$prod_total" -lt 300 ]; then
    echo "🟡 Status: GOOD - Manageable amount of production unwraps"
elif [ "$prod_total" -lt 600 ]; then
    echo "🟡 Status: FAIR - Significant production unwraps to address"
else
    echo "🔴 Status: NEEDS WORK - Many production unwraps to address"
fi
echo

echo "=== ANALYSIS COMPLETE ==="

