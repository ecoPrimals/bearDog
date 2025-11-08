#!/bin/bash
# Analyze clone() usage in codebase
set -euo pipefail

OUTPUT_DIR="./profiling/reports"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
REPORT_FILE="$OUTPUT_DIR/clone_analysis_${TIMESTAMP}.txt"

echo "Analyzing clone() usage in codebase..."
echo "Output: $REPORT_FILE"
echo ""

{
    echo "═══════════════════════════════════════════════════════════════"
    echo "  BearDog Clone Usage Analysis"
    echo "  Generated: $(date)"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
    
    echo "1. Total .clone() calls:"
    grep -r "\.clone()" crates/ --include="*.rs" | wc -l
    echo ""
    
    echo "2. Clone calls by package:"
    for pkg in crates/*/; do
        pkg_name=$(basename "$pkg")
        count=$(grep -r "\.clone()" "$pkg" --include="*.rs" 2>/dev/null | wc -l)
        if [ -n "$count" ] && [ "$count" -gt 0 ]; then
            printf "   %-30s %5d\n" "$pkg_name" "$count"
        fi
    done | sort -k2 -rn
    echo ""
    
    echo "3. High-frequency clone patterns:"
    echo ""
    echo "   Arc clones:"
    grep -r "Arc::clone\|\.clone()" crates/ --include="*.rs" | grep -i "arc" | wc -l
    echo ""
    echo "   HashMap clones:"
    grep -r "\.clone()" crates/ --include="*.rs" | grep -i "hashmap" | wc -l
    echo ""
    echo "   String clones:"
    grep -r "String::clone\|\.to_string()\|\.to_owned()" crates/ --include="*.rs" | wc -l
    echo ""
    
    echo "4. Top 20 files by clone count:"
    for file in $(find crates/ -name "*.rs"); do
        count=$(grep -c "\.clone()" "$file" 2>/dev/null || echo 0)
        if [ "$count" -gt 0 ]; then
            echo "$count $file"
        fi
    done | sort -rn | head -20
    echo ""
    
    echo "5. Potential optimization targets (Arc<RwLock<HashMap>>):"
    grep -r "Arc<RwLock<HashMap" crates/ --include="*.rs" -n | head -20
    echo ""
    
} | tee "$REPORT_FILE"

echo "✓ Clone analysis complete: $REPORT_FILE"
