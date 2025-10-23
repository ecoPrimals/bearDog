#!/bin/bash
# Weekly status check script

echo "=========================================="
echo "BEARDOG WEEKLY STATUS"
echo "Date: $(date +%Y-%m-%d)"
echo "=========================================="
echo ""

# Test Coverage
echo "=== TEST COVERAGE ==="
echo "Running tarpaulin (this may take a minute)..."
coverage=$(cargo tarpaulin --workspace --out Json --skip-clean 2>/dev/null | \
  grep -o '"coverage":[0-9.]*' | \
  cut -d: -f2 | head -1)

if [ -n "$coverage" ]; then
    echo "Current: ${coverage}%"
else
    echo "Current: (run 'cargo tarpaulin --workspace' manually)"
fi
echo ""

# Unwraps
echo "=== PRODUCTION UNWRAPS ==="
prod_unwraps=$(grep -r "\.unwrap()\|\.expect(" crates/ \
  --include="*.rs" \
  --exclude-dir="tests" \
  | grep -v "test\|#\[cfg(test)\]" \
  | wc -l)
echo "Production unwraps: $prod_unwraps"
echo "Target: 0"
echo ""

# Hardcoding
echo "=== HARDCODED CONFIG ==="
hardcoded=$(grep -rE "(localhost|127\.0\.0\.1|:8080|:8081)" crates/ \
  --include="*.rs" \
  --exclude-dir="tests" \
  | wc -l)
echo "Hardcoded values: $hardcoded"
echo "Target: 0"
echo ""

# Tests
echo "=== TESTS ==="
test_files=$(find crates -path "*/tests/*.rs" | wc -l)
echo "Test files: $test_files"

# Run a quick test
echo "Running quick test suite..."
test_results=$(cargo test --workspace --lib 2>&1 | grep "test result")
if [ -n "$test_results" ]; then
    echo "$test_results"
else
    echo "(Run 'cargo test --workspace' for full results)"
fi
echo ""

# Clippy
echo "=== CLIPPY WARNINGS ==="
echo "Running clippy (this may take a minute)..."
clippy_count=$(cargo clippy --workspace --all-targets 2>&1 | \
  grep "warning:" | wc -l)
echo "Warnings: $clippy_count"
echo "Target: 0"
echo ""

# File sizes
echo "=== FILE SIZE COMPLIANCE ==="
over_1000=$(find crates -name "*.rs" -exec wc -l {} + | \
  awk '$1 > 1000 {print}' | \
  wc -l)
echo "Files over 1000 lines: $over_1000"
echo "Target: 0"
echo ""

# Summary
echo "=========================================="
echo "SUMMARY"
echo "=========================================="
echo "✅ Next actions:"
echo "  - Write tests to increase coverage"
echo "  - Fix production unwraps (priority: security/crypto)"
echo "  - Remove hardcoded config (use env vars)"
echo "  - Address clippy warnings"
echo ""
echo "Run individual analysis scripts:"
echo "  ./scripts/find_production_unwraps.sh"
echo "  ./scripts/find_hardcoding.sh"
echo ""

