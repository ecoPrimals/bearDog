#!/bin/bash

# BearDog Deployment Readiness Check
# Comprehensive validation script for production deployment

set -e

echo "🚀 BearDog Deployment Readiness Check"
echo "===================================="
echo ""

# Color codes for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0
WARNINGS=0

# Helper functions
pass() {
    echo -e "${GREEN}✅ PASS${NC}: $1"
    ((PASSED++))
}

fail() {
    echo -e "${RED}❌ FAIL${NC}: $1"
    ((FAILED++))
}

warn() {
    echo -e "${YELLOW}⚠️  WARN${NC}: $1"
    ((WARNINGS++))
}

info() {
    echo -e "${BLUE}ℹ️  INFO${NC}: $1"
}

# Check 1: Compilation
echo "🔨 1. COMPILATION CHECKS"
echo "----------------------"

if cargo check --all --all-features > /dev/null 2>&1; then
    pass "All crates compile successfully"
else
    fail "Compilation errors detected"
fi

if cargo build --release > /dev/null 2>&1; then
    pass "Release build successful"
else
    fail "Release build failed"
fi

# Check 2: Code Quality
echo ""
echo "🧹 2. CODE QUALITY CHECKS"
echo "-------------------------"

# Clippy check
CLIPPY_OUTPUT=$(cargo clippy --all-targets --all-features -- -D warnings 2>&1 || true)
if echo "$CLIPPY_OUTPUT" | grep -q "error:"; then
    fail "Clippy errors detected"
    echo "$CLIPPY_OUTPUT" | grep "error:" | head -5
else
    pass "Clippy validation passed"
fi

# Format check
if cargo fmt --check > /dev/null 2>&1; then
    pass "Code formatting is consistent"
else
    fail "Code formatting issues detected"
fi

# Check 3: Test Coverage
echo ""
echo "🧪 3. TEST COVERAGE CHECKS"
echo "--------------------------"

# Core package tests
if cargo test --package beardog-types --lib > /dev/null 2>&1; then
    TEST_COUNT=$(cargo test --package beardog-types --lib 2>&1 | grep "test result:" | grep -o '[0-9]\+ passed' | grep -o '[0-9]\+')
    pass "beardog-types: $TEST_COUNT tests passing"
else
    fail "beardog-types tests failing"
fi

if cargo test --package beardog-security --lib > /dev/null 2>&1; then
    TEST_COUNT=$(cargo test --package beardog-security --lib 2>&1 | grep "test result:" | grep -o '[0-9]\+ passed' | grep -o '[0-9]\+')
    pass "beardog-security: $TEST_COUNT tests passing"
else
    fail "beardog-security tests failing"
fi

# Check 4: Documentation
echo ""
echo "📚 4. DOCUMENTATION CHECKS"
echo "--------------------------"

if cargo doc --all --no-deps > /dev/null 2>&1; then
    DOC_WARNINGS=$(cargo doc --all --no-deps 2>&1 | grep -c "warning:" || echo "0")
    if [ "$DOC_WARNINGS" -lt 5 ]; then
        pass "Documentation generates with minimal warnings ($DOC_WARNINGS)"
    else
        warn "Documentation has $DOC_WARNINGS warnings"
    fi
else
    fail "Documentation generation failed"
fi

# Check 5: Security Validation
echo ""
echo "🛡️  5. SECURITY VALIDATION"
echo "-------------------------"

# Check for unsafe code
UNSAFE_COUNT=$(find . -name "*.rs" -not -path "./target/*" -exec grep -l "unsafe" {} \; | wc -l)
if [ "$UNSAFE_COUNT" -eq 0 ]; then
    pass "Zero unsafe code blocks in production code"
else
    warn "$UNSAFE_COUNT files contain unsafe code"
fi

# Check for unwrap patterns in production code
UNWRAP_COUNT=$(find ./crates -name "*.rs" -not -path "./target/*" -exec grep -l "\.unwrap()" {} \; | wc -l)
if [ "$UNWRAP_COUNT" -lt 5 ]; then
    pass "Minimal unwrap() usage in production code ($UNWRAP_COUNT files)"
else
    warn "$UNWRAP_COUNT files contain unwrap() patterns"
fi

# Check 6: Performance Validation
echo ""
echo "⚡ 6. PERFORMANCE VALIDATION"
echo "---------------------------"

# Check for zero-copy implementations
ZERO_COPY_COUNT=$(find ./crates -name "*.rs" -exec grep -l "zero_copy\|ZeroCopy" {} \; | wc -l)
if [ "$ZERO_COPY_COUNT" -gt 0 ]; then
    pass "Zero-copy optimizations implemented ($ZERO_COPY_COUNT files)"
else
    warn "No zero-copy optimizations detected"
fi

# Check 7: Configuration Management
echo ""
echo "⚙️  7. CONFIGURATION VALIDATION"
echo "-------------------------------"

# Check for hardcoded values
HARDCODE_COUNT=$(find ./crates -name "*.rs" -exec grep -l "127\.0\.0\.1\|localhost\|8080" {} \; | wc -l)
if [ "$HARDCODE_COUNT" -lt 3 ]; then
    pass "Minimal hardcoded constants ($HARDCODE_COUNT files)"
else
    warn "$HARDCODE_COUNT files contain hardcoded values"
fi

# Check for configuration helpers
if grep -r "default_api_host\|default_api_port" ./crates > /dev/null 2>&1; then
    pass "Configuration helpers implemented"
else
    fail "Configuration helpers missing"
fi

# Check 8: Dependency Audit
echo ""
echo "📦 8. DEPENDENCY AUDIT"
echo "---------------------"

# Check for known vulnerabilities (if cargo-audit is available)
if command -v cargo-audit > /dev/null 2>&1; then
    if cargo audit > /dev/null 2>&1; then
        pass "No known security vulnerabilities in dependencies"
    else
        warn "Potential security vulnerabilities detected"
    fi
else
    warn "cargo-audit not available - install with: cargo install cargo-audit"
fi

# Check 9: File Size Compliance
echo ""
echo "📏 9. FILE SIZE COMPLIANCE"
echo "--------------------------"

LARGE_FILES=$(find ./crates -name "*.rs" -exec wc -l {} + | awk '$1 > 1000 {print $2}' | wc -l)
if [ "$LARGE_FILES" -eq 0 ]; then
    pass "All source files under 1000 lines"
else
    warn "$LARGE_FILES files exceed 1000 line limit"
fi

# Check 10: Sovereignty Compliance
echo ""
echo "👑 10. SOVEREIGNTY COMPLIANCE"
echo "-----------------------------"

# Check for sovereignty monitoring
if grep -r "sovereignty\|human_dignity" ./crates > /dev/null 2>&1; then
    pass "Sovereignty and human dignity monitoring implemented"
else
    warn "Sovereignty monitoring not detected"
fi

# Final Summary
echo ""
echo "📊 DEPLOYMENT READINESS SUMMARY"
echo "==============================="
echo -e "✅ Passed: ${GREEN}$PASSED${NC}"
echo -e "❌ Failed: ${RED}$FAILED${NC}"
echo -e "⚠️  Warnings: ${YELLOW}$WARNINGS${NC}"
echo ""

# Calculate readiness score
TOTAL=$((PASSED + FAILED + WARNINGS))
SCORE=$((PASSED * 100 / TOTAL))

if [ "$FAILED" -eq 0 ] && [ "$SCORE" -gt 85 ]; then
    echo -e "${GREEN}🎉 DEPLOYMENT READY${NC} - Score: ${SCORE}%"
    echo "System is ready for production deployment!"
    exit 0
elif [ "$FAILED" -eq 0 ]; then
    echo -e "${YELLOW}⚠️  DEPLOYMENT CAUTION${NC} - Score: ${SCORE}%"
    echo "System is deployable but has warnings to address."
    exit 1
else
    echo -e "${RED}❌ DEPLOYMENT BLOCKED${NC} - Score: ${SCORE}%"
    echo "Critical issues must be resolved before deployment."
    exit 2
fi 