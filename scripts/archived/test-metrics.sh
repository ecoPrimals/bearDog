#!/bin/bash
# Test Organization Metrics
# Created: October 26, 2025

set -e

echo "📊 BearDog Test Organization Metrics"
echo "====================================="
echo ""

# Total test count
echo "=== Test Count ==="
TOTAL_TESTS=$(cargo test --workspace --lib --bins --tests --benches --all-features -- --list 2>&1 | grep -E ": test$" | wc -l)
echo "Total test functions: $TOTAL_TESTS"
echo ""

# Test files
echo "=== Test Files ==="
TEST_FILES=$(find tests crates -name "*.rs" -exec grep -l "#\[test\]" {} \; | wc -l)
echo "Files with tests: $TEST_FILES"
echo ""

# By location
echo "=== Tests by Location ==="
TESTS_DIR=$(find tests -name "*.rs" -exec grep -c "#\[test\]" {} + 2>/dev/null | awk -F: '{sum+=$2} END {print sum}')
CRATES_TESTS=$(find crates -path "*/tests/*.rs" -exec grep -c "#\[test\]" {} + 2>/dev/null | awk -F: '{sum+=$2} END {print sum}')
INLINE_TESTS=$(find crates -name "*.rs" ! -path "*/tests/*" -exec grep -c "#\[cfg(test)\]" {} + 2>/dev/null | awk -F: '{sum+=$2} END {print sum}')

echo "tests/ directory:     $TESTS_DIR"
echo "crates/*/tests/:      $CRATES_TESTS"
echo "Inline test modules:  $INLINE_TESTS"
echo ""

# E2E and Chaos
echo "=== Special Test Types ==="
E2E_TESTS=$(find tests/e2e -name "*.rs" -exec grep -c "#\[test\]\|#\[tokio::test\]" {} + 2>/dev/null | awk -F: '{sum+=$2} END {print sum}')
CHAOS_TESTS=$(find tests/chaos -name "*.rs" -exec grep -c "#\[test\]\|#\[tokio::test\]" {} + 2>/dev/null | awk -F: '{sum+=$2} END {print sum}')

echo "E2E tests:    $E2E_TESTS"
echo "Chaos tests:  $CHAOS_TESTS"
echo ""

# By crate
echo "=== Tests by Crate (top 10) ==="
for crate in beardog-core beardog-security beardog-tunnel beardog-types beardog-genetics \
             beardog-monitoring beardog-workflows beardog-adapters beardog-auth beardog-utils; do
    if [ -d "crates/$crate" ]; then
        count=$(find "crates/$crate" -name "*.rs" -exec grep -c "#\[test\]" {} + 2>/dev/null | awk -F: '{sum+=$2} END {print sum}')
        printf "%-25s %s\n" "$crate:" "$count"
    fi
done
echo ""

# Test categories (estimated from file names)
echo "=== Test Categories (estimated from names) ==="
SECURITY_TESTS=$(find . -name "*.rs" -exec grep -l "test.*security\|security.*test" {} \; | wc -l)
HSM_TESTS=$(find . -name "*.rs" -exec grep -l "test.*hsm\|hsm.*test" {} \; | wc -l)
CRYPTO_TESTS=$(find . -name "*.rs" -exec grep -l "test.*crypto\|crypto.*test" {} \; | wc -l)
AUTH_TESTS=$(find . -name "*.rs" -exec grep -l "test.*auth\|auth.*test" {} \; | wc -l)

echo "Security-related: $SECURITY_TESTS files"
echo "HSM-related:      $HSM_TESTS files"
echo "Crypto-related:   $CRYPTO_TESTS files"
echo "Auth-related:     $AUTH_TESTS files"
echo ""

# Ignored tests
echo "=== Test Status ==="
IGNORED_TESTS=$(grep -r "#\[ignore\]" tests crates --include="*.rs" | wc -l)
echo "Ignored tests: $IGNORED_TESTS"
echo ""

# Test documentation
echo "=== Test Documentation ==="
DOCUMENTED_TESTS=$(grep -r "/// TEST_CATEGORY\|/// TEST_DOMAIN" tests crates --include="*.rs" | wc -l)
echo "Tests with category/domain tags: $DOCUMENTED_TESTS"
echo ""

# Summary
echo "=== Summary ==="
echo "✅ Total Tests:        $TOTAL_TESTS"
echo "✅ Test Files:         $TEST_FILES"
echo "⚠️  E2E Tests:          $E2E_TESTS (target: 50+)"
echo "⚠️  Chaos Tests:        $CHAOS_TESTS (target: 100+)"
echo "⚠️  Documented Tests:   $DOCUMENTED_TESTS (target: $TOTAL_TESTS)"
echo ""

# Coverage (if tarpaulin is available)
if command -v cargo-tarpaulin &> /dev/null; then
    echo "=== Coverage Analysis ==="
    echo "Running coverage analysis (this may take a few minutes)..."
    cargo tarpaulin --out Json --output-dir /tmp/beardog-coverage --skip-clean 2>/dev/null || true
    if [ -f /tmp/beardog-coverage/tarpaulin-report.json ]; then
        COVERAGE=$(cat /tmp/beardog-coverage/tarpaulin-report.json | grep -o '"coverage":[0-9.]*' | head -1 | cut -d: -f2)
        echo "Overall coverage: ${COVERAGE}%"
    fi
    echo ""
fi

echo "✅ Test metrics analysis complete"

