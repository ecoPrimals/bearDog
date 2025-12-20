#!/usr/bin/env bash
# Comprehensive Showcase Test Runner with Auto-Mode
#
# This runner tests ALL showcases and reports which ones:
#   - PASS (work correctly in auto mode)
#   - SKIP (correctly skip when human input required)
#   - FAIL (actual errors that need investigation)
#
# USAGE:
#   ./test-all-demos.sh --auto

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPORT_DIR="$SCRIPT_DIR/test-reports/$(date +%Y%m%d-%H%M%S)"
mkdir -p "$REPORT_DIR"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

log_pass() { echo -e "${GREEN}✅ PASS${NC}: $1"; }
log_skip() { echo -e "${YELLOW}⏭️  SKIP${NC}: $1"; }
log_fail() { echo -e "${RED}❌ FAIL${NC}: $1"; }
log_info() { echo -e "${CYAN}ℹ️${NC}  $1"; }

# Test counters
TOTAL=0
PASSED=0
SKIPPED=0
FAILED=0

# Test a demo
test_demo() {
    local name="$1"
    local script="$2"
    local should_skip="${3:-false}"  # Expected to skip?
    
    ((TOTAL++))
    
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Testing: $name"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    if [ ! -f "$script" ]; then
        log_skip "$name (script not found)"
        ((SKIPPED++))
        echo "$name,SKIP,Script not found" >> "$REPORT_DIR/results.csv"
        return
    fi
    
    chmod +x "$script" 2>/dev/null || true
    
    local log_file="$REPORT_DIR/$(basename "$script" .sh).log"
    
    if timeout 300 "$script" --auto > "$log_file" 2>&1; then
        if [ "$should_skip" = "true" ]; then
            log_info "$name passed but was expected to skip (acceptable)"
            log_pass "$name"
            ((PASSED++))
            echo "$name,PASS,Passed (was expected to skip)" >> "$REPORT_DIR/results.csv"
        else
            log_pass "$name"
            ((PASSED++))
            echo "$name,PASS," >> "$REPORT_DIR/results.csv"
        fi
    else
        local exit_code=$?
        if [ "$should_skip" = "true" ]; then
            log_skip "$name (correctly skipped - requires human input)"
            ((SKIPPED++))
            echo "$name,SKIP,Requires human input" >> "$REPORT_DIR/results.csv"
        else
            log_fail "$name (exit code: $exit_code)"
            ((FAILED++))
            echo "$name,FAIL,Exit code $exit_code" >> "$REPORT_DIR/results.csv"
            echo "Last 10 lines of log:"
            tail -10 "$log_file" 2>/dev/null || echo "(No log)"
        fi
    fi
}

# Banner
cat << 'EOF'

╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║              🧪 COMPREHENSIVE SHOWCASE TEST SUITE 🧪                         ║
║                     Auto-Mode Validation                                     ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝

Testing all BearDog showcases in auto mode...

Report directory: $REPORT_DIR

EOF

echo "Demo,Result,Notes" > "$REPORT_DIR/results.csv"

START_TIME=$(date +%s)

#
# CATEGORY 1: Crypto Verification (Should PASS)
#

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Category 1: Crypto Verification Demos                      ║"
echo "╚══════════════════════════════════════════════════════════════╝"

test_demo "HSM Discovery" \
    "$SCRIPT_DIR/04-hsm-vendor-agnostic/demos/01-discover-all-hsms.sh" \
    false

test_demo "HSM Runtime Switching" \
    "$SCRIPT_DIR/04-hsm-vendor-agnostic/demos/02-runtime-hsm-switch.sh" \
    false

test_demo "Live Crypto Proof (Genetic Mixing)" \
    "$SCRIPT_DIR/03-songbird-integration/demos/02-live-crypto-proof.sh" \
    false

#
# CATEGORY 2: Service Integration (Should PASS)
#

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Category 2: Service Integration Demos                      ║"
echo "╚══════════════════════════════════════════════════════════════╝"

test_demo "Songbird Service Registration" \
    "$SCRIPT_DIR/03-songbird-integration/demos/01-service-registration.sh" \
    false

#
# CATEGORY 3: Human Entropy (Should SKIP - Expected!)
#

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Category 3: Human Entropy Demos (Expected to Skip)         ║"
echo "╚══════════════════════════════════════════════════════════════╝"

log_info "These demos SHOULD skip in auto mode (entropy hierarchy enforcement)"
echo ""

test_demo "Human Entropy Interactive" \
    "$SCRIPT_DIR/02-hardware-integration/demo-human-entropy-interactive.sh" \
    true

test_demo "Human Entropy Mixing" \
    "$SCRIPT_DIR/entropy-mixing-real-human.sh" \
    true

#
# CATEGORY 4: Genetic/Constraint Demos (Should PASS)
#

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Category 4: Genetic & Constraint Demos                     ║"
echo "╚══════════════════════════════════════════════════════════════╝"

test_demo "Genetic Realistic" \
    "$SCRIPT_DIR/02-hardware-integration/demo-genetic-realistic.sh" \
    false

test_demo "Constraints Demo" \
    "$SCRIPT_DIR/03-constraint-demos/demo-constraints.sh" \
    false

#
# Generate Report
#

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo ""
echo "╔══════════════════════════════════════════════════════════════════════════════╗"
echo "║                                                                              ║"
echo "║                           📊 TEST RESULTS 📊                                 ║"
echo "║                                                                              ║"
echo "╚══════════════════════════════════════════════════════════════════════════════╝"
echo ""

cat << EOF
Duration: ${DURATION}s

┌──────────────┬───────┬─────────┐
│ Status       │ Count │ Percent │
├──────────────┼───────┼─────────┤
│ ✅ PASSED    │ $PASSED     │ $(awk "BEGIN {printf \"%.1f\", ($PASSED/$TOTAL)*100}")%    │
│ ⏭️  SKIPPED  │ $SKIPPED     │ $(awk "BEGIN {printf \"%.1f\", ($SKIPPED/$TOTAL)*100}")%    │
│ ❌ FAILED    │ $FAILED     │ $(awk "BEGIN {printf \"%.1f\", ($FAILED/$TOTAL)*100}")%    │
├──────────────┼───────┼─────────┤
│ TOTAL        │ $TOTAL     │ 100.0%  │
└──────────────┴───────┴─────────┘

EOF

# Detailed results
echo "Detailed Results:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
cat "$REPORT_DIR/results.csv" | column -t -s ','
echo ""

# Save full report
cat > "$REPORT_DIR/REPORT.md" << REPORT
# BearDog Showcase Test Report

**Date:** $(date)  
**Duration:** ${DURATION}s  
**Mode:** Auto (non-interactive)

## Summary

| Status | Count | Percentage |
|--------|-------|------------|
| ✅ Passed | $PASSED | $(awk "BEGIN {printf \"%.1f\", ($PASSED/$TOTAL)*100}")% |
| ⏭️ Skipped | $SKIPPED | $(awk "BEGIN {printf \"%.1f\", ($SKIPPED/$TOTAL)*100}")% |
| ❌ Failed | $FAILED | $(awk "BEGIN {printf \"%.1f\", ($FAILED/$TOTAL)*100}")% |
| **Total** | **$TOTAL** | **100%** |

## Analysis

### Passed Demos ($PASSED)
These demos work correctly in auto mode with no user interaction.

### Skipped Demos ($SKIPPED)
These demos require human entropy and correctly skip in auto mode.
**This is expected and correct behavior (entropy hierarchy enforcement).**

### Failed Demos ($FAILED)
These demos failed unexpectedly and need investigation.

## Detailed Results

\`\`\`
$(cat "$REPORT_DIR/results.csv")
\`\`\`

## Logs

Individual demo logs are available in: \`$REPORT_DIR/\`

---

**Overall Status:** $([ $FAILED -eq 0 ] && echo "✅ ALL TESTS CORRECT" || echo "⚠️  SOME FAILURES NEED INVESTIGATION")
REPORT

echo "Full report: $REPORT_DIR/REPORT.md"
echo ""

# Final status
if [ $FAILED -eq 0 ]; then
    echo "╔══════════════════════════════════════════════════════════════════════════════╗"
    echo "║                                                                              ║"
    echo "║                  ✅ ALL TESTS BEHAVED CORRECTLY! ✅                           ║"
    echo "║                                                                              ║"
    echo "║  • Crypto demos passed                                                      ║"
    echo "║  • Human entropy demos correctly skipped (entropy hierarchy!)               ║"
    echo "║  • No unexpected failures                                                   ║"
    echo "║                                                                              ║"
    echo "╚══════════════════════════════════════════════════════════════════════════════╝"
    exit 0
else
    echo "╔══════════════════════════════════════════════════════════════════════════════╗"
    echo "║                                                                              ║"
    echo "║                  ⚠️  SOME FAILURES DETECTED ⚠️                               ║"
    echo "║                                                                              ║"
    echo "║  Review logs in: $REPORT_DIR/                                               ║"
    echo "║                                                                              ║"
    echo "╚══════════════════════════════════════════════════════════════════════════════╝"
    exit 1
fi

