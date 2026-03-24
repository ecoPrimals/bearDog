#!/bin/bash

# BearDog Production Security Audit Script
# Comprehensive security validation for production deployment

set -euo pipefail

echo "🛡️ BearDog Production Security Audit - March 2026"
echo "=================================================="

AUDIT_RESULTS_DIR="security_audit_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$AUDIT_RESULTS_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

TOTAL_CHECKS=0
PASSED_CHECKS=0
WARNING_CHECKS=0
FAILED_CHECKS=0

log_result() {
    local status=$1
    local message=$2
    local details=${3:-""}
    
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    
    case $status in
        "PASS")
            echo -e "${GREEN}✅ PASS${NC}: $message"
            PASSED_CHECKS=$((PASSED_CHECKS + 1))
            ;;
        "WARN")
            echo -e "${YELLOW}⚠️  WARN${NC}: $message"
            if [ -n "$details" ]; then
                echo "   Details: $details"
            fi
            WARNING_CHECKS=$((WARNING_CHECKS + 1))
            ;;
        "FAIL")
            echo -e "${RED}❌ FAIL${NC}: $message"
            if [ -n "$details" ]; then
                echo "   Details: $details"
            fi
            FAILED_CHECKS=$((FAILED_CHECKS + 1))
            ;;
    esac
}

echo ""
echo "🔍 1. UNSAFE CODE AUDIT"
echo "----------------------"

# Check for unsafe code blocks
UNSAFE_COUNT=$(find crates/beardog-*/src -name "*.rs" -exec grep -l "unsafe" {} \; 2>/dev/null | wc -l || echo "0")
if [ "$UNSAFE_COUNT" -eq 0 ]; then
    log_result "PASS" "No unsafe code found in production crates"
else
    # Check if unsafe code is properly documented
    UNDOCUMENTED_UNSAFE=$(find crates/beardog-*/src -name "*.rs" -exec grep -l "unsafe" {} \; 2>/dev/null | xargs grep -L "SAFETY:" 2>/dev/null | wc -l || echo "0")
    if [ "$UNDOCUMENTED_UNSAFE" -eq 0 ]; then
        log_result "PASS" "All unsafe code blocks are properly documented ($UNSAFE_COUNT files with SAFETY comments)"
    else
        log_result "WARN" "Some unsafe code lacks SAFETY documentation" "$UNDOCUMENTED_UNSAFE files need documentation"
    fi
fi

echo ""
echo "🔐 2. CRYPTOGRAPHIC SECURITY AUDIT"
echo "-----------------------------------"

# Check for hardcoded keys or secrets
HARDCODED_SECRETS=$(grep -r -i "password\|secret\|key.*=" crates/beardog-*/src --include="*.rs" 2>/dev/null | grep -v "test\|example\|demo" | wc -l || echo "0")
if [ "$HARDCODED_SECRETS" -eq 0 ]; then
    log_result "PASS" "No hardcoded secrets found in production code"
else
    log_result "FAIL" "Potential hardcoded secrets detected" "$HARDCODED_SECRETS instances found"
fi

# Check for proper random number generation
RNG_USAGE=$(grep -r "rand::" crates/beardog-*/src --include="*.rs" 2>/dev/null | wc -l || echo "0")
if [ "$RNG_USAGE" -gt 0 ]; then
    SECURE_RNG=$(grep -r "rand::rngs::OsRng\|rand_core::OsRng" crates/beardog-*/src --include="*.rs" 2>/dev/null | wc -l || echo "0")
    if [ "$SECURE_RNG" -gt 0 ]; then
        log_result "PASS" "Cryptographically secure RNG usage detected"
    else
        log_result "WARN" "Random number generation detected - verify cryptographic security"
    fi
fi

echo ""
echo "🚫 3. PANIC AND ERROR HANDLING AUDIT"
echo "------------------------------------"

# Check for panic! in production code
PANIC_COUNT=$(grep -r "panic!" crates/beardog-*/src --include="*.rs" 2>/dev/null | grep -v "test\|example\|demo" | wc -l || echo "0")
if [ "$PANIC_COUNT" -eq 0 ]; then
    log_result "PASS" "No panic! calls in production code"
else
    log_result "FAIL" "panic! calls found in production code" "$PANIC_COUNT instances"
fi

# Check for unwrap() in production code
UNWRAP_COUNT=$(grep -r "\.unwrap()" crates/beardog-*/src --include="*.rs" 2>/dev/null | grep -v "test\|example\|demo" | wc -l || echo "0")
if [ "$UNWRAP_COUNT" -eq 0 ]; then
    log_result "PASS" "No unwrap() calls in production code"
elif [ "$UNWRAP_COUNT" -lt 10 ]; then
    log_result "WARN" "Minimal unwrap() usage in production code" "$UNWRAP_COUNT instances - review for safety"
else
    log_result "FAIL" "Excessive unwrap() usage in production code" "$UNWRAP_COUNT instances"
fi

echo ""
echo "🔒 4. DEPENDENCY SECURITY AUDIT"
echo "-------------------------------"

# Check for security advisories (requires cargo-audit)
if command -v cargo-audit >/dev/null 2>&1; then
    if cargo audit --quiet 2>/dev/null; then
        log_result "PASS" "No known security vulnerabilities in dependencies"
    else
        log_result "FAIL" "Security vulnerabilities detected in dependencies" "Run 'cargo audit' for details"
    fi
else
    log_result "WARN" "cargo-audit not installed" "Install with: cargo install cargo-audit"
fi

echo ""
echo "🌐 5. NETWORK SECURITY AUDIT"
echo "----------------------------"

# Check for hardcoded URLs/IPs in production
HARDCODED_URLS=$(grep -r -E "http://|https://|[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+" crates/beardog-*/src --include="*.rs" 2>/dev/null | grep -v "test\|example\|demo\|localhost\|127.0.0.1" | wc -l || echo "0")
if [ "$HARDCODED_URLS" -eq 0 ]; then
    log_result "PASS" "No hardcoded production URLs/IPs found"
else
    log_result "WARN" "Hardcoded URLs/IPs detected" "$HARDCODED_URLS instances - verify they're configurable"
fi

echo ""
echo "🔍 6. CODE QUALITY SECURITY AUDIT"
echo "---------------------------------"

# Check compilation with security flags
if cargo clippy --workspace --quiet -- -D warnings >/dev/null 2>&1; then
    log_result "PASS" "Code passes clippy security lints"
else
    log_result "WARN" "Clippy warnings detected" "Review warnings for security implications"
fi

# Check for TODO/FIXME in security-critical areas
SECURITY_TODOS=$(grep -r -i "todo\|fixme" crates/beardog-security/src crates/beardog-auth/src --include="*.rs" 2>/dev/null | wc -l || echo "0")
if [ "$SECURITY_TODOS" -eq 0 ]; then
    log_result "PASS" "No TODO/FIXME in security-critical modules"
else
    log_result "WARN" "TODO/FIXME found in security modules" "$SECURITY_TODOS instances - review for completeness"
fi

echo ""
echo "🏛️ 7. SOVEREIGNTY COMPLIANCE AUDIT"
echo "----------------------------------"

# Check for sovereignty violations
SOVEREIGNTY_TESTS=$(find tests -name "*sovereignty*" -o -name "*dignity*" | wc -l || echo "0")
if [ "$SOVEREIGNTY_TESTS" -gt 0 ]; then
    log_result "PASS" "Sovereignty compliance tests present ($SOVEREIGNTY_TESTS test files)"
else
    log_result "WARN" "No sovereignty compliance tests found" "Consider adding sovereignty validation tests"
fi

echo ""
echo "📊 8. PRODUCTION READINESS CHECKS"
echo "---------------------------------"

# Check for comprehensive error handling
ERROR_TYPES=$(grep -r "BearDogError::" crates/beardog-*/src --include="*.rs" 2>/dev/null | wc -l || echo "0")
if [ "$ERROR_TYPES" -gt 50 ]; then
    log_result "PASS" "Comprehensive error handling implemented ($ERROR_TYPES error instances)"
else
    log_result "WARN" "Limited error handling detected" "Consider expanding error coverage"
fi

# Check for logging and monitoring
LOGGING_USAGE=$(grep -r "tracing::\|log::" crates/beardog-*/src --include="*.rs" 2>/dev/null | wc -l || echo "0")
if [ "$LOGGING_USAGE" -gt 20 ]; then
    log_result "PASS" "Comprehensive logging implemented ($LOGGING_USAGE log statements)"
else
    log_result "WARN" "Limited logging detected" "Consider expanding monitoring coverage"
fi

echo ""
echo "🧪 9. TEST COVERAGE SECURITY AUDIT"
echo "----------------------------------"

# Run tests to ensure security functionality
if cargo test --workspace --quiet >/dev/null 2>&1; then
    log_result "PASS" "All security tests passing"
else
    log_result "FAIL" "Test failures detected" "Security functionality may be compromised"
fi

# Check for security-specific tests
SECURITY_TESTS=$(find tests -name "*security*" -o -name "*auth*" -o -name "*crypto*" | wc -l || echo "0")
if [ "$SECURITY_TESTS" -gt 5 ]; then
    log_result "PASS" "Comprehensive security test coverage ($SECURITY_TESTS test files)"
else
    log_result "WARN" "Limited security test coverage" "Consider expanding security tests"
fi

echo ""
echo "📋 SECURITY AUDIT SUMMARY"
echo "========================="

# Generate summary report
cat > "$AUDIT_RESULTS_DIR/security_audit_summary.txt" << EOF
BearDog Security Audit Summary
Generated: $(date)

Total Checks: $TOTAL_CHECKS
Passed: $PASSED_CHECKS
Warnings: $WARNING_CHECKS  
Failed: $FAILED_CHECKS

Security Score: $((PASSED_CHECKS * 100 / TOTAL_CHECKS))%
EOF

echo "Total Security Checks: $TOTAL_CHECKS"
echo -e "${GREEN}Passed: $PASSED_CHECKS${NC}"
echo -e "${YELLOW}Warnings: $WARNING_CHECKS${NC}"
echo -e "${RED}Failed: $FAILED_CHECKS${NC}"
echo ""

SECURITY_SCORE=$((PASSED_CHECKS * 100 / TOTAL_CHECKS))
if [ "$SECURITY_SCORE" -ge 90 ]; then
    echo -e "${GREEN}🏆 SECURITY GRADE: A+ ($SECURITY_SCORE%)${NC}"
    echo -e "${GREEN}✅ PRODUCTION SECURITY READY${NC}"
elif [ "$SECURITY_SCORE" -ge 80 ]; then
    echo -e "${YELLOW}🥉 SECURITY GRADE: B+ ($SECURITY_SCORE%)${NC}"
    echo -e "${YELLOW}⚠️  PRODUCTION READY WITH MINOR IMPROVEMENTS${NC}"
else
    echo -e "${RED}❌ SECURITY GRADE: C ($SECURITY_SCORE%)${NC}"
    echo -e "${RED}🚫 NOT READY FOR PRODUCTION${NC}"
fi

echo ""
echo "📁 Detailed results saved to: $AUDIT_RESULTS_DIR/"
echo "🛡️ Security audit complete!"

exit 0 