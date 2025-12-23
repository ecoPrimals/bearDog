#!/bin/bash
# Unification Progress Tracker for BearDog
# Created: November 10, 2025

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  🐻 BearDog Unification Progress Dashboard"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 1. File Size Compliance
echo "📏 File Size Compliance:"
MAX_LINES=2000
OVER_LIMIT=$(find crates -name "*.rs" -type f -exec wc -l {} \; | awk -v max=$MAX_LINES '$1 > max' | wc -l)
if [ "$OVER_LIMIT" -eq 0 ]; then
    echo -e "   ${GREEN}✅ 100% compliant${NC} - 0 files over ${MAX_LINES} lines"
else
    echo -e "   ${RED}⚠️  $OVER_LIMIT files${NC} over ${MAX_LINES} lines"
fi

# 2. Configuration Consolidation
echo ""
echo "🗂️  Configuration Consolidation:"
TOTAL_CONFIGS=$(grep -r "pub struct.*Config" crates --include="*.rs" 2>/dev/null | wc -l)
CANONICAL_CONFIGS=$(grep -r "pub struct.*Config" crates/beardog-types/src/canonical --include="*.rs" 2>/dev/null | wc -l)
if [ "$TOTAL_CONFIGS" -gt 0 ]; then
    CANONICAL_PCT=$((CANONICAL_CONFIGS * 100 / TOTAL_CONFIGS))
    if [ "$CANONICAL_PCT" -ge 95 ]; then
        echo -e "   ${GREEN}✅ ${CANONICAL_PCT}%${NC} canonical ($CANONICAL_CONFIGS/$TOTAL_CONFIGS)"
    elif [ "$CANONICAL_PCT" -ge 80 ]; then
        echo -e "   ${YELLOW}📈 ${CANONICAL_PCT}%${NC} canonical ($CANONICAL_CONFIGS/$TOTAL_CONFIGS)"
    else
        echo -e "   ${YELLOW}⚠️  ${CANONICAL_PCT}%${NC} canonical ($CANONICAL_CONFIGS/$TOTAL_CONFIGS)"
    fi
else
    echo "   ⚠️  Unable to measure"
fi

# 3. async_trait Migration
echo ""
echo "⚡ async_trait Migration:"
ASYNC_TRAIT_COUNT=$(grep -r "#\[async_trait\]" crates --include="*.rs" 2>/dev/null | wc -l)
if [ "$ASYNC_TRAIT_COUNT" -eq 0 ]; then
    echo -e "   ${GREEN}✅ Complete${NC} - 0 instances remaining"
elif [ "$ASYNC_TRAIT_COUNT" -le 5 ]; then
    echo -e "   ${YELLOW}📈 Nearly done${NC} - $ASYNC_TRAIT_COUNT instances remaining"
else
    echo -e "   ${YELLOW}⚠️  In progress${NC} - $ASYNC_TRAIT_COUNT instances remaining"
fi

# 4. Result Type Unification
echo ""
echo "🔧 Result Type Unification:"
RESULT_STRAGGLERS=$(grep -r "pub type.*Result.*=" crates --include="*.rs" 2>/dev/null | \
  grep -v "unified_types.rs" | grep -v "test" | grep -v "/lib.rs" | wc -l)
if [ "$RESULT_STRAGGLERS" -eq 0 ]; then
    echo -e "   ${GREEN}✅ Complete${NC} - All in unified_types.rs"
elif [ "$RESULT_STRAGGLERS" -le 5 ]; then
    echo -e "   ${YELLOW}📈 Nearly done${NC} - $RESULT_STRAGGLERS stragglers"
else
    echo -e "   ${YELLOW}⚠️  In progress${NC} - $RESULT_STRAGGLERS stragglers"
fi

# 5. Legacy Code Markers
echo ""
echo "🧹 Legacy Code Cleanup:"
LEGACY_COUNT=$(grep -r "legacy\|compat\|shim\|TODO\|FIXME\|HACK" crates --include="*.rs" 2>/dev/null | wc -l)
if [ "$LEGACY_COUNT" -lt 200 ]; then
    echo -e "   ${GREEN}✅ Clean${NC} - $LEGACY_COUNT instances (target: <200)"
elif [ "$LEGACY_COUNT" -lt 500 ]; then
    echo -e "   ${YELLOW}📈 Good${NC} - $LEGACY_COUNT instances (target: <200)"
else
    echo -e "   ${YELLOW}⚠️  High${NC} - $LEGACY_COUNT instances (target: <200)"
fi

# 6. Import Hygiene
echo ""
echo "📦 Import Hygiene:"
RELATIVE_IMPORTS=$(find crates -name "*.rs" -exec grep -l "^use.*\.\.\." {} \; 2>/dev/null | wc -l)
if [ "$RELATIVE_IMPORTS" -eq 0 ]; then
    echo -e "   ${GREEN}✅ Perfect${NC} - No relative imports"
else
    echo -e "   ${YELLOW}⚠️  $RELATIVE_IMPORTS files${NC} with relative imports"
fi

# Calculate overall grade
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Scoring
SCORE=0
MAX_SCORE=600

# File size: 100 points if perfect
[ "$OVER_LIMIT" -eq 0 ] && SCORE=$((SCORE + 100))

# Config: up to 100 points
if [ "$TOTAL_CONFIGS" -gt 0 ]; then
    SCORE=$((SCORE + CANONICAL_CONFIGS * 100 / TOTAL_CONFIGS))
fi

# async_trait: 100 points if zero
[ "$ASYNC_TRAIT_COUNT" -eq 0 ] && SCORE=$((SCORE + 100))
[ "$ASYNC_TRAIT_COUNT" -gt 0 ] && [ "$ASYNC_TRAIT_COUNT" -le 14 ] && SCORE=$((SCORE + 50))

# Result types: 100 points if zero stragglers
[ "$RESULT_STRAGGLERS" -eq 0 ] && SCORE=$((SCORE + 100))
[ "$RESULT_STRAGGLERS" -gt 0 ] && [ "$RESULT_STRAGGLERS" -le 5 ] && SCORE=$((SCORE + 80))

# Legacy: up to 100 points
if [ "$LEGACY_COUNT" -lt 200 ]; then
    SCORE=$((SCORE + 100))
elif [ "$LEGACY_COUNT" -lt 500 ]; then
    SCORE=$((SCORE + 60))
elif [ "$LEGACY_COUNT" -lt 1000 ]; then
    SCORE=$((SCORE + 20))
fi

# Imports: 100 points if zero relative
[ "$RELATIVE_IMPORTS" -eq 0 ] && SCORE=$((SCORE + 100))

# Calculate percentage
GRADE=$(awk "BEGIN {printf \"%.1f\", $SCORE / $MAX_SCORE * 100}")

echo "📊 Overall Unification Grade: $GRADE / 100"
echo ""

# Status message
if awk "BEGIN {exit !($GRADE >= 99.5)}"; then
    echo -e "${GREEN}🏆 Status: EXCELLENT${NC} - Near perfect unification!"
elif awk "BEGIN {exit !($GRADE >= 95)}"; then
    echo -e "${YELLOW}📈 Status: VERY GOOD${NC} - Minor gaps remaining"
elif awk "BEGIN {exit !($GRADE >= 85)}"; then
    echo -e "${YELLOW}📊 Status: GOOD${NC} - On track for completion"
else
    echo "📋 Status: IN PROGRESS - Continue unification work"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "💡 Run this script regularly to track progress!"
echo "   Location: scripts/unification_progress_tracker.sh"
echo ""
