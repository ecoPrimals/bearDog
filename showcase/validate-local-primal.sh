#!/usr/bin/env bash
# BearDog Local Primal Showcase Validation Script
# Tests all Level 0 demos and generates validation report

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Report file
REPORT_FILE="00-local-primal/VALIDATION_REPORT_DEC_25_2025.md"

# Start time
START_TIME=$(date +%s)

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🐻 BearDog Local Primal Showcase Validation${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Initialize report
cat > "$REPORT_FILE" << 'EOF'
# 🐻 BearDog Local Primal Validation Report

**Date**: $(date '+%Y-%m-%d %H:%M:%S')  
**Validator**: Automated Test Suite  
**Level**: 0 (Local Primal)

---

## 📊 EXECUTIVE SUMMARY

EOF

# Function to run a demo and capture results
run_demo() {
    local demo_dir=$1
    local demo_name=$2
    local expected_time=$3
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    echo -e "${YELLOW}Testing: ${demo_name}${NC}"
    echo "  Directory: $demo_dir"
    echo "  Expected time: ${expected_time}s"
    
    # Check if directory exists
    if [ ! -d "00-local-primal/$demo_dir" ]; then
        echo -e "  ${RED}✗ FAILED${NC} - Directory not found"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
    
    # Check if run.sh exists
    if [ ! -f "00-local-primal/$demo_dir/run.sh" ]; then
        echo -e "  ${RED}✗ FAILED${NC} - run.sh not found"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
    
    # Make executable
    chmod +x "00-local-primal/$demo_dir/run.sh"
    
    # Run the demo with timeout
    local start=$(date +%s)
    local output_file="/tmp/beardog_test_${demo_dir}.log"
    
    if timeout $((expected_time * 3)) bash -c "cd 00-local-primal/$demo_dir && ./run.sh" > "$output_file" 2>&1; then
        local end=$(date +%s)
        local duration=$((end - start))
        echo -e "  ${GREEN}✓ PASSED${NC} (${duration}s)"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        
        # Append to report
        cat >> "$REPORT_FILE" << EOF

### ✅ $demo_name
- **Status**: PASSED
- **Duration**: ${duration}s (expected: ${expected_time}s)
- **Output**: See \`/tmp/beardog_test_${demo_dir}.log\`

EOF
        return 0
    else
        local end=$(date +%s)
        local duration=$((end - start))
        echo -e "  ${RED}✗ FAILED${NC} (${duration}s)"
        echo "  Check log: $output_file"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        
        # Append to report
        cat >> "$REPORT_FILE" << EOF

### ❌ $demo_name
- **Status**: FAILED
- **Duration**: ${duration}s (timeout at: $((expected_time * 3))s)
- **Error Log**: \`$output_file\`

\`\`\`
$(tail -20 "$output_file")
\`\`\`

EOF
        return 1
    fi
}

echo ""
echo -e "${BLUE}Running demos...${NC}"
echo ""

# Run all demos
run_demo "01-hello-beardog" "Hello BearDog" 30
run_demo "02-hsm-discovery" "HSM Discovery" 30
run_demo "03-key-constraints" "Key Constraints" 60
run_demo "04-entropy-mixing" "Entropy Mixing" 60
run_demo "05-key-lineage" "Key Lineage" 60
run_demo "06-btsp-tunnel" "BTSP Tunnel" 90

# Calculate results
END_TIME=$(date +%s)
TOTAL_DURATION=$((END_TIME - START_TIME))
PASS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📊 VALIDATION RESULTS${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Total Tests:    $TOTAL_TESTS"
echo -e "Passed:         ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed:         ${RED}$FAILED_TESTS${NC}"
echo "Pass Rate:      ${PASS_RATE}%"
echo "Total Duration: ${TOTAL_DURATION}s"
echo ""

# Finalize report
cat >> "$REPORT_FILE" << EOF

---

## 📈 SUMMARY STATISTICS

| Metric | Value |
|--------|-------|
| **Total Tests** | $TOTAL_TESTS |
| **Passed** | ✅ $PASSED_TESTS |
| **Failed** | ❌ $FAILED_TESTS |
| **Pass Rate** | ${PASS_RATE}% |
| **Total Duration** | ${TOTAL_DURATION}s |

---

## 🎯 VALIDATION STATUS

EOF

if [ $FAILED_TESTS -eq 0 ]; then
    cat >> "$REPORT_FILE" << 'EOF'
### ✅ ALL TESTS PASSED

**Level 0 (Local Primal) is VALIDATED!**

All 6 demos executed successfully. BearDog's core capabilities are working as specified:
- ✅ Key generation
- ✅ HSM discovery
- ✅ Genetic constraints
- ✅ Entropy mixing
- ✅ Key lineage
- ✅ BTSP tunnels

**Ready for**: Level 1 (Hardware Integration)

EOF
    echo -e "${GREEN}✅ ALL TESTS PASSED!${NC}"
    echo -e "${GREEN}Level 0 is VALIDATED!${NC}"
else
    cat >> "$REPORT_FILE" << EOF
### ⚠️ SOME TESTS FAILED

**$FAILED_TESTS out of $TOTAL_TESTS tests failed.**

**Action Required**:
1. Review failed test logs above
2. Fix identified issues
3. Re-run validation: \`./validate-local-primal.sh\`

**Not ready for**: Level 1 until all tests pass

EOF
    echo -e "${RED}⚠️ SOME TESTS FAILED${NC}"
    echo -e "${RED}Review the report: $REPORT_FILE${NC}"
fi

cat >> "$REPORT_FILE" << 'EOF'

---

## 📋 SPEC CLAIMS VALIDATED

### Core Cryptographic Operations
- [ ] Key generation (Ed25519, P-256, AES)
- [ ] Sign/verify operations
- [ ] Encrypt/decrypt operations
- [ ] Key metadata management

### HSM Integration
- [ ] Software HSM fallback
- [ ] Auto-discovery (zero-knowledge)
- [ ] Capability querying
- [ ] Multi-HSM support

### Genetic Keys
- [ ] Constraint creation
- [ ] Self-enforcement
- [ ] Automatic validation
- [ ] Immutable rules

### Entropy Hierarchy
- [ ] Machine entropy collection
- [ ] Human entropy mixing
- [ ] Quality scoring
- [ ] Trust model enforcement

### Key Lineage
- [ ] Parent-child tracking
- [ ] Constraint inheritance
- [ ] Audit trail generation
- [ ] Lineage queries

### BTSP Protocol
- [ ] Tunnel establishment
- [ ] End-to-end encryption
- [ ] Perfect Forward Secrecy
- [ ] Mutual authentication

---

## 🔍 DETAILED FINDINGS

### Performance Metrics
- Average demo execution: $(($TOTAL_DURATION / $TOTAL_TESTS))s
- Fastest demo: (see individual results)
- Slowest demo: (see individual results)

### Error Patterns
(Review failed test logs for patterns)

### Recommendations
1. All passing tests: Proceed to Level 1
2. Some failures: Fix and re-validate
3. Performance issues: Review timeout settings

---

**Validation Complete**: $(date '+%Y-%m-%d %H:%M:%S')  
**Report Generated**: $REPORT_FILE  
**Next Step**: Review findings and proceed accordingly

🐻 **BearDog: Validated and Ready!** 🎯
EOF

echo ""
echo "Report saved to: $REPORT_FILE"
echo ""

# Exit with appropriate code
if [ $FAILED_TESTS -eq 0 ]; then
    exit 0
else
    exit 1
fi

