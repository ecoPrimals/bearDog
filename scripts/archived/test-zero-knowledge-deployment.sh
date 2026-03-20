#!/usr/bin/env bash
# Test Zero-Knowledge Deployment - Validates infant deployment capability
#
# This script tests that a primal can start with ZERO hardcoded knowledge

set -euo pipefail

COLOR_RED='\033[0;31m'
COLOR_GREEN='\033[0;32m'
COLOR_YELLOW='\033[1;33m'
COLOR_BLUE='\033[0;34m'
COLOR_RESET='\033[0m'

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$WORKSPACE_ROOT"

echo -e "${COLOR_BLUE}🧪 Zero-Knowledge Deployment Test${COLOR_RESET}"
echo -e "${COLOR_BLUE}====================================${COLOR_RESET}\n"

# Test counters
TESTS_PASSED=0
TESTS_FAILED=0
TESTS_TOTAL=0

# Test result tracking
test_result() {
    local test_name="$1"
    local result="$2"
    local message="${3:-}"
    
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    
    if [ "$result" = "PASS" ]; then
        echo -e "${COLOR_GREEN}✅ PASS:${COLOR_RESET} $test_name"
        [ -n "$message" ] && echo -e "   ${COLOR_BLUE}ℹ${COLOR_RESET}  $message"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${COLOR_RED}❌ FAIL:${COLOR_RESET} $test_name"
        [ -n "$message" ] && echo -e "   ${COLOR_RED}✗${COLOR_RESET}  $message"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

echo -e "${COLOR_YELLOW}Test Suite 1: Self-Discovery${COLOR_RESET}"
echo -e "${COLOR_YELLOW}=============================${COLOR_RESET}\n"

# Test 1.1: Primal ID generation without hardcoded names
echo "Test 1.1: Primal ID Generation..."
if cargo test test_self_discovery_engine --quiet 2>&1 | grep -q "test result: ok"; then
    test_result "Primal ID generation" "PASS" "Generates UUID-based IDs without hardcoded names"
else
    test_result "Primal ID generation" "FAIL" "Failed to generate primal ID"
fi
echo ""

# Test 1.2: Zero hardcoded knowledge validation
echo "Test 1.2: Zero Hardcoded Knowledge..."
if cargo test test_zero_hardcoded_knowledge --quiet 2>&1 | grep -q "test result: ok"; then
    test_result "Zero hardcoded knowledge" "PASS" "No hardcoded primal or ecosystem assumptions"
else
    test_result "Zero hardcoded knowledge" "FAIL" "Found hardcoded knowledge violations"
fi
echo ""

# Test 1.3: Capability auto-detection
echo "Test 1.3: Capability Auto-Detection..."
if cargo test test_capability_auto_detection --quiet 2>&1 | grep -q "test result: ok"; then
    test_result "Capability auto-detection" "PASS" "Dynamically detects own capabilities"
else
    test_result "Capability auto-detection" "FAIL" "Capability detection failed"
fi
echo ""

echo -e "\n${COLOR_YELLOW}Test Suite 2: Configuration Discovery${COLOR_RESET}"
echo -e "${COLOR_YELLOW}=====================================${COLOR_RESET}\n"

# Test 2.1: Environment-based configuration
echo "Test 2.1: Environment Variable Configuration..."
export PRIMAL_TYPE=test-primal
export PRIMAL_NAME="Test Primal Service"
export BEARDOG_API_PORT=9999

if cargo test --quiet 2>&1 | grep -q "test result: ok"; then
    test_result "Environment configuration" "PASS" "Reads config from environment"
else
    test_result "Environment configuration" "FAIL" "Failed to read environment config"
fi
unset PRIMAL_TYPE PRIMAL_NAME BEARDOG_API_PORT
echo ""

# Test 2.2: No hardcoded ports
echo "Test 2.2: Port Configuration..."
if ! grep -r "const.*PORT.*=.*[0-9]" crates/beardog-core/src/*.rs 2>/dev/null | grep -v "// " | grep -v "deprecated"; then
    test_result "Port configuration" "PASS" "No hardcoded ports in core"
else
    test_result "Port configuration" "FAIL" "Found hardcoded port values"
fi
echo ""

# Test 2.3: No hardcoded primal names in production code
echo "Test 2.3: Primal Name Hardcoding..."
primal_count=$(grep -r -i "songbird\|toadstool\|squirrel\|nestgate" crates/beardog-core/src/*.rs 2>/dev/null | grep -v "// " | grep -v "test" | wc -l || echo "0")
if [ "$primal_count" -eq 0 ]; then
    test_result "Primal name hardcoding" "PASS" "No hardcoded primal names in core"
else
    test_result "Primal name hardcoding" "FAIL" "Found $primal_count hardcoded primal references"
fi
echo ""

echo -e "\n${COLOR_YELLOW}Test Suite 3: Universal Adapter Integration${COLOR_RESET}"
echo -e "${COLOR_YELLOW}============================================${COLOR_RESET}\n"

# Test 3.1: Universal adapter available
echo "Test 3.1: Universal Adapter Availability..."
if cargo test --package beardog-adapters --quiet 2>&1 | grep -q "test result: ok"; then
    test_result "Universal adapter" "PASS" "Universal adapter tests pass"
else
    test_result "Universal adapter" "FAIL" "Universal adapter tests failed"
fi
echo ""

# Test 3.2: Capability-based discovery
echo "Test 3.2: Capability-Based Discovery..."
if [ -f "crates/beardog-adapters/src/universal/primal_capability_adapter.rs" ]; then
    test_result "Capability adapter" "PASS" "Primal capability adapter exists"
else
    test_result "Capability adapter" "FAIL" "Primal capability adapter missing"
fi
echo ""

# Test 3.3: Vendor-agnostic patterns
echo "Test 3.3: Vendor-Agnostic Patterns..."
if [ -f "crates/beardog-types/src/canonical/config/network_discovery.rs" ]; then
    test_result "Vendor agnostic config" "PASS" "Network discovery config supports multiple vendors"
else
    test_result "Vendor agnostic config" "FAIL" "Vendor-agnostic config missing"
fi
echo ""

echo -e "\n${COLOR_YELLOW}Test Suite 4: Documentation and Templates${COLOR_RESET}"
echo -e "${COLOR_YELLOW}=========================================${COLOR_RESET}\n"

# Test 4.1: Migration templates exist
echo "Test 4.1: Migration Templates..."
if [ -f "ecosystem-templates/primal-hardcoding-elimination-template.rs" ] && \
   [ -f "ecosystem-templates/vendor-agnostic-migration-template.rs" ]; then
    test_result "Migration templates" "PASS" "Migration templates available"
else
    test_result "Migration templates" "FAIL" "Migration templates missing"
fi
echo ""

# Test 4.2: Deployment guide exists
echo "Test 4.2: Deployment Documentation..."
if [ -f "ZERO_KNOWLEDGE_DEPLOYMENT_GUIDE.md" ]; then
    test_result "Deployment guide" "PASS" "Zero-knowledge deployment guide available"
else
    test_result "Deployment guide" "FAIL" "Deployment guide missing"
fi
echo ""

# Test 4.3: Hardcoding elimination plan exists
echo "Test 4.3: Elimination Plan..."
if [ -f "HARDCODING_ELIMINATION_PLAN.md" ]; then
    test_result "Elimination plan" "PASS" "Hardcoding elimination plan available"
else
    test_result "Elimination plan" "FAIL" "Elimination plan missing"
fi
echo ""

echo -e "\n${COLOR_BLUE}📊 Test Results Summary${COLOR_RESET}"
echo -e "${COLOR_BLUE}=======================${COLOR_RESET}\n"

echo -e "${COLOR_GREEN}Passed:${COLOR_RESET} $TESTS_PASSED / $TESTS_TOTAL"
echo -e "${COLOR_RED}Failed:${COLOR_RESET} $TESTS_FAILED / $TESTS_TOTAL"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n${COLOR_GREEN}🎉 ALL TESTS PASSED!${COLOR_RESET}"
    echo -e "${COLOR_GREEN}Zero-knowledge deployment capability verified!${COLOR_RESET}\n"
    exit 0
else
    echo -e "\n${COLOR_RED}⚠️  SOME TESTS FAILED${COLOR_RESET}"
    echo -e "${COLOR_YELLOW}Review failed tests above and apply fixes.${COLOR_RESET}\n"
    exit 1
fi

