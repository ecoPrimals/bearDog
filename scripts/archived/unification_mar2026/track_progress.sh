#!/bin/bash
# BearDog Unification Progress Dashboard
# Track progress toward 100% unification

echo "📊 BearDog Unification Progress Dashboard"
echo "=========================================="
echo ""

# Type System Metrics
echo "🔧 TYPE SYSTEM:"
RESULT_USAGE=$(grep -r "BearDogResult" crates --include="*.rs" | grep -v "deprecated" | grep -v "test" | grep -v "//.*BearDogResult" | wc -l)
ASYNC_TRAIT=$(grep -r "#\[async_trait\]" crates --include="*.rs" | wc -l)
echo "  • BearDogResult usages: $RESULT_USAGE (target: 0)"
echo "  • async_trait count: $ASYNC_TRAIT (target: 0)"

if [ "$RESULT_USAGE" -eq 0 ] && [ "$ASYNC_TRAIT" -eq 0 ]; then
    echo "  ✅ Type system: 100% unified"
    TYPE_SCORE=100
else
    TYPE_SCORE=$(( 100 - (RESULT_USAGE + ASYNC_TRAIT * 2) ))
    [ "$TYPE_SCORE" -lt 0 ] && TYPE_SCORE=0
    echo "  ⚠️  Type system: ${TYPE_SCORE}% unified"
fi

echo ""

# Config System Metrics
echo "⚙️  CONFIG SYSTEM:"
TOTAL_CONFIGS=$(grep -r "pub struct.*Config" crates --include="*.rs" | wc -l)
CANONICAL_CONFIGS=$(grep -r "pub struct.*Config" crates/beardog-types/src/canonical --include="*.rs" | wc -l)
CONFIG_PERCENT=$(( CANONICAL_CONFIGS * 100 / TOTAL_CONFIGS ))
echo "  • Total configs: $TOTAL_CONFIGS"
echo "  • Canonical configs: $CANONICAL_CONFIGS"
echo "  • Canonicalization: ${CONFIG_PERCENT}%"

if [ "$CONFIG_PERCENT" -ge 95 ]; then
    echo "  ✅ Config system: ${CONFIG_PERCENT}% canonical"
else
    echo "  ⚠️  Config system: ${CONFIG_PERCENT}% canonical (target: 95%)"
fi

echo ""

# Legacy Code Metrics
echo "🧹 LEGACY CODE:"
LEGACY_FILES=$(find crates -name "*.rs" -exec grep -l "legacy\|compat\|shim" {} \; | wc -l)
echo "  • Files with legacy patterns: $LEGACY_FILES (target: <50)"

if [ "$LEGACY_FILES" -lt 50 ]; then
    echo "  ✅ Legacy code: Minimal ($LEGACY_FILES files)"
    LEGACY_SCORE=100
else
    LEGACY_SCORE=$(( 100 - (LEGACY_FILES - 50) ))
    [ "$LEGACY_SCORE" -lt 0 ] && LEGACY_SCORE=0
    echo "  ⚠️  Legacy code: $LEGACY_FILES files to clean"
fi

echo ""

# File Size Compliance
echo "📏 FILE SIZE COMPLIANCE:"
LARGE_FILES=$(find crates -name "*.rs" -exec wc -l {} \; | awk '$1 > 2000 {print $2}' | wc -l)
echo "  • Files > 2000 lines: $LARGE_FILES (target: 0)"

if [ "$LARGE_FILES" -eq 0 ]; then
    echo "  ✅ File size: 100% compliant"
    SIZE_SCORE=100
else
    echo "  ⚠️  File size: $LARGE_FILES violations"
    SIZE_SCORE=80
fi

echo ""

# Overall Score
OVERALL=$(( (TYPE_SCORE + CONFIG_PERCENT + LEGACY_SCORE + SIZE_SCORE) / 4 ))
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎯 OVERALL UNIFICATION: ${OVERALL}%"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ "$OVERALL" -ge 98 ]; then
    echo "🏆 Status: EXCELLENT - Nearly perfect unification!"
elif [ "$OVERALL" -ge 90 ]; then
    echo "✅ Status: GOOD - On track to 100%"
elif [ "$OVERALL" -ge 80 ]; then
    echo "⚠️  Status: FAIR - More work needed"
else
    echo "🔴 Status: NEEDS WORK - Execute action plan"
fi

echo ""
echo "📅 Last Updated: $(date)"
echo "📊 Track: Run this script weekly to monitor progress"

