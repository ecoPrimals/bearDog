#!/bin/bash
# Quick Unwrap/Expect Analysis and Fix Script
# For immediate code quality improvement

CRATES_DIR="/home/eastgate/Development/ecoPrimals/beardog/crates"

echo "🔍 Analyzing unwrap/expect patterns..."
echo ""

# Count by crate
echo "📊 Unwrap/Expect by Crate:"
for crate in "$CRATES_DIR"/*/; do
    crate_name=$(basename "$crate")
    count=$(grep -r "\.unwrap()\|\.expect(" "$crate" --include="*.rs" 2>/dev/null | wc -l)
    if [ "$count" -gt 0 ]; then
        printf "  %-30s %3d instances\n" "$crate_name:" "$count"
    fi
done | sort -k2 -nr

echo ""
echo "📊 Clone by Crate:"
for crate in "$CRATES_DIR"/*/; do
    crate_name=$(basename "$crate")
    count=$(grep -r "\.clone()" "$crate" --include="*.rs" 2>/dev/null | wc -l)
    if [ "$count" -gt 0 ]; then
        printf "  %-30s %3d instances\n" "$crate_name:" "$count"
    fi
done | sort -k2 -nr

echo ""
echo "🎯 Top Files with unwrap/expect:"
grep -r "\.unwrap()\|\.expect(" "$CRATES_DIR" --include="*.rs" 2>/dev/null | \
    cut -d: -f1 | sort | uniq -c | sort -rn | head -20 | \
    awk '{printf "  %3d  %s\n", $1, $2}'

echo ""
echo "📋 Total Counts:"
unwrap_total=$(grep -r "\.unwrap()\|\.expect(" "$CRATES_DIR" --include="*.rs" 2>/dev/null | wc -l)
clone_total=$(grep -r "\.clone()" "$CRATES_DIR" --include="*.rs" 2>/dev/null | wc -l)
echo "  unwrap/expect: $unwrap_total"
echo "  clone():       $clone_total"

