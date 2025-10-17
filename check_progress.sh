#!/bin/bash
# BearDog Progress Tracking Script
# Run this weekly to track progress toward production

set -e

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🐻 BEARDOG PROGRESS TRACKER"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Date: $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# Test Coverage
echo "━━━ TEST COVERAGE ━━━"
if [ -f "coverage/tarpaulin-report.json" ]; then
    COVERAGE=$(cat coverage/tarpaulin-report.json | jq -r '.coverage' 2>/dev/null || echo "ERROR")
    echo "Current: ${COVERAGE}%"
    echo "Target:  90.0%"
    echo "Gap:     $(echo "90.0 - ${COVERAGE}" | bc)%"
else
    echo "⚠️  No coverage report found"
    echo "Run: cargo tarpaulin --output-dir coverage --out Json"
fi
echo ""

# Error Handling
echo "━━━ ERROR HANDLING ━━━"
UNWRAPS=$(grep -r '\.unwrap()' crates/ --include='*.rs' 2>/dev/null | wc -l || echo "0")
EXPECTS=$(grep -r '\.expect(' crates/ --include='*.rs' 2>/dev/null | wc -l || echo "0")
TOTAL=$((UNWRAPS + EXPECTS))
echo "Unwraps:  ${UNWRAPS}"
echo "Expects:  ${EXPECTS}"
echo "Total:    ${TOTAL}"
echo "Target:   <10"
echo ""

# Code Quality
echo "━━━ CODE QUALITY ━━━"
echo "Clippy warnings: (running...)"
CLIPPY_WARNINGS=$(cargo clippy --workspace --all-targets 2>&1 | grep -c "warning:" || echo "0")
echo "Current: ${CLIPPY_WARNINGS}"
echo "Target:  <50"
echo ""

# Hardcoded Values
echo "━━━ HARDCODED VALUES ━━━"
HARDCODED=$(grep -r '127\.0\.0\.1\|localhost\|:8080\|:3000\|:5432' crates/ --include='*.rs' 2>/dev/null | wc -l || echo "0")
echo "Current: ${HARDCODED}"
echo "Target:  <20"
echo ""

# TODOs
echo "━━━ TECHNICAL DEBT ━━━"
TODOS=$(grep -ri 'TODO\|FIXME\|XXX\|HACK' crates/ --include='*.rs' 2>/dev/null | wc -l || echo "0")
echo "TODOs:   ${TODOS}"
echo "Target:  0"
echo ""

# File Discipline
echo "━━━ FILE DISCIPLINE ━━━"
LARGE_FILES=$(find crates -name '*.rs' -exec wc -l {} + 2>/dev/null | awk '$1 > 1000' | wc -l || echo "0")
TOTAL_FILES=$(find crates -name '*.rs' 2>/dev/null | wc -l || echo "0")
echo "Files:        ${TOTAL_FILES}"
echo "Over 1000:    ${LARGE_FILES}"
echo "Target:       0"
echo ""

# Build Status
echo "━━━ BUILD STATUS ━━━"
if cargo check --workspace --quiet 2>/dev/null; then
    echo "✅ Build: PASSING"
else
    echo "❌ Build: FAILING"
fi
echo ""

# Test Status
echo "━━━ TEST STATUS ━━━"
echo "(Running tests...)"
TEST_RESULT=$(cargo test --workspace --quiet 2>&1 | grep "test result:" | tail -1 || echo "No tests found")
echo "${TEST_RESULT}"
echo ""

# Formatting
echo "━━━ FORMATTING ━━━"
if cargo fmt --all -- --check >/dev/null 2>&1; then
    echo "✅ Formatting: PASSING"
else
    echo "⚠️  Formatting: NEEDS FIX (run: cargo fmt --all)"
fi
echo ""

# Memory Safety
echo "━━━ MEMORY SAFETY ━━━"
UNSAFE=$(grep -r "unsafe" crates/ --include='*.rs' 2>/dev/null | grep -v "// unsafe" | grep -v "/// unsafe" | wc -l || echo "0")
echo "Unsafe blocks: ${UNSAFE}"
echo "Target:        0"
echo ""

# Grade Calculation
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 OVERALL GRADE"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Simple grade calculation based on key metrics
GRADE="B+"
if [ -f "coverage/tarpaulin-report.json" ]; then
    COVERAGE_NUM=$(echo ${COVERAGE} | bc 2>/dev/null || echo "5")
    if (( $(echo "$COVERAGE_NUM >= 90" | bc -l) )); then
        if [ "$TOTAL" -lt 10 ] && [ "$CLIPPY_WARNINGS" -lt 50 ]; then
            GRADE="A (95/100)"
        fi
    elif (( $(echo "$COVERAGE_NUM >= 60" | bc -l) )); then
        GRADE="A- (92/100)"
    elif (( $(echo "$COVERAGE_NUM >= 40" | bc -l) )); then
        GRADE="A- (90/100)"
    elif (( $(echo "$COVERAGE_NUM >= 30" | bc -l) )); then
        GRADE="B+ (87/100)"
    else
        GRADE="B+ (84/100)"
    fi
fi

echo "Current: ${GRADE}"
echo ""

# Timeline
echo "━━━ PRODUCTION TIMELINE ━━━"
if [ -f "coverage/tarpaulin-report.json" ]; then
    COVERAGE_NUM=$(echo ${COVERAGE} | bc 2>/dev/null || echo "5")
    if (( $(echo "$COVERAGE_NUM >= 90" | bc -l) )); then
        echo "Status: ✅ PRODUCTION READY"
    elif (( $(echo "$COVERAGE_NUM >= 60" | bc -l) )); then
        echo "Status: 🟡 6-8 weeks to production"
    elif (( $(echo "$COVERAGE_NUM >= 40" | bc -l) )); then
        echo "Status: 🟠 8-12 weeks to production"
    elif (( $(echo "$COVERAGE_NUM >= 30" | bc -l) )); then
        echo "Status: 🟠 10-14 weeks to production"
    else
        echo "Status: 🔴 15-18 weeks to production"
    fi
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔐 Track progress weekly!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

