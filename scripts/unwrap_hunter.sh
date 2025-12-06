#!/usr/bin/env bash
# Unwrap Hunter - Find and categorize unwrap() calls
# Date: November 28, 2025
# Purpose: Systematically identify unwrap() calls for elimination

set -euo pipefail

echo "🐻 BearDog Unwrap Hunter"
echo "=========================="
echo

# Colors
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

OUTPUT_DIR="target/debt-analysis"
mkdir -p "$OUTPUT_DIR"

REPORT_FILE="$OUTPUT_DIR/unwrap_analysis_$(date +%Y%m%d_%H%M%S).md"

echo "# Unwrap Analysis Report" > "$REPORT_FILE"
echo "**Generated**: $(date)" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Function to count unwraps in a directory
count_unwraps() {
    local dir=$1
    local label=$2
    local pattern=$3
    
    local count=$(find "$dir" -name "*.rs" -type f ! -path "*/target/*" -exec grep -c "$pattern" {} + 2>/dev/null | awk '{s+=$1} END {print s}')
    echo "$count|$label"
}

# Analyze by crate and code type
echo "## Summary by Crate" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "| Crate | Production Code | Test Code | Total |" >> "$REPORT_FILE"
echo "|-------|----------------|-----------|-------|" >> "$REPORT_FILE"

echo -e "${BLUE}Analyzing crates...${NC}"

total_prod=0
total_test=0

for crate_dir in crates/*/; do
    crate_name=$(basename "$crate_dir")
    
    # Count in src/ (production)
    if [ -d "$crate_dir/src" ]; then
        prod_unwrap=$(find "$crate_dir/src" -name "*.rs" -type f ! -name "*test*.rs" ! -path "*/tests/*" -exec grep -c "\.unwrap()\|\.expect(" {} + 2>/dev/null | awk '{s+=$1} END {print s+0}')
    else
        prod_unwrap=0
    fi
    
    # Count in tests/ and test files
    test_unwrap=$(find "$crate_dir" -name "*test*.rs" -o -path "*/tests/*" -name "*.rs" 2>/dev/null | xargs grep -c "\.unwrap()\|\.expect(" 2>/dev/null | awk '{s+=$1} END {print s+0}')
    
    total=$((prod_unwrap + test_unwrap))
    
    if [ $total -gt 0 ]; then
        echo "| $crate_name | $prod_unwrap | $test_unwrap | $total |" >> "$REPORT_FILE"
        total_prod=$((total_prod + prod_unwrap))
        total_test=$((total_test + test_unwrap))
    fi
done

echo "|-------|----------------|-----------|-------|" >> "$REPORT_FILE"
echo "| **TOTAL** | **$total_prod** | **$total_test** | **$((total_prod + total_test))** |" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Priority Analysis
echo "## Priority Analysis" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

echo "### 🔴 CRITICAL (Security & Crypto)" >> "$REPORT_FILE"
for crate in beardog-security beardog-crypto beardog-tunnel; do
    if [ -d "crates/$crate/src" ]; then
        count=$(find "crates/$crate/src" -name "*.rs" -type f ! -name "*test*.rs" -exec grep -c "\.unwrap()\|\.expect(" {} + 2>/dev/null | awk '{s+=$1} END {print s+0}')
        if [ $count -gt 0 ]; then
            echo "- **$crate**: $count instances" >> "$REPORT_FILE"
        fi
    fi
done
echo "" >> "$REPORT_FILE"

echo "### 🟡 HIGH (Core & Networking)" >> "$REPORT_FILE"
for crate in beardog-core beardog-networking beardog-auth; do
    if [ -d "crates/$crate/src" ]; then
        count=$(find "crates/$crate/src" -name "*.rs" -type f ! -name "*test*.rs" -exec grep -c "\.unwrap()\|\.expect(" {} + 2>/dev/null | awk '{s+=$1} END {print s+0}')
        if [ $count -gt 0 ]; then
            echo "- **$crate**: $count instances" >> "$REPORT_FILE"
        fi
    fi
done
echo "" >> "$REPORT_FILE"

echo "### 🟢 MEDIUM (Utilities & Types)" >> "$REPORT_FILE"
for crate in beardog-utils beardog-types beardog-config; do
    if [ -d "crates/$crate/src" ]; then
        count=$(find "crates/$crate/src" -name "*.rs" -type f ! -name "*test*.rs" -exec grep -c "\.unwrap()\|\.expect(" {} + 2>/dev/null | awk '{s+=$1} END {print s+0}')
        if [ $count -gt 0 ]; then
            echo "- **$crate**: $count instances" >> "$REPORT_FILE"
        fi
    fi
done
echo "" >> "$REPORT_FILE"

# Detailed listings for top offenders
echo "## Detailed Analysis - Top Files" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "### Files with Most Unwraps (Production Code)" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

find crates -name "*.rs" -type f ! -name "*test*.rs" ! -path "*/tests/*" -path "*/src/*" -exec grep -l "\.unwrap()\|\.expect(" {} \; 2>/dev/null | while read file; do
    count=$(grep -c "\.unwrap()\|\.expect(" "$file" 2>/dev/null || echo 0)
    if [ $count -gt 5 ]; then
        echo "$count|$file"
    fi
done | sort -rn | head -20 | while IFS='|' read count file; do
    echo "- **$file**: $count instances" >> "$REPORT_FILE"
done

echo "" >> "$REPORT_FILE"

# Recommendations
echo "## Recommendations" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "### Immediate Actions (Week 1)" >> "$REPORT_FILE"
echo "1. Add \`#![deny(clippy::unwrap_used)]\` to critical crates (security, crypto, tunnel)" >> "$REPORT_FILE"
echo "2. Fix compilation errors systematically" >> "$REPORT_FILE"
echo "3. Replace unwrap() with proper error handling using \`?\` operator" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "### Pattern to Follow" >> "$REPORT_FILE"
echo '```rust' >> "$REPORT_FILE"
echo '// Before (panic risk):' >> "$REPORT_FILE"
echo 'let value = option.unwrap();' >> "$REPORT_FILE"
echo '' >> "$REPORT_FILE"
echo '// After (proper error handling):' >> "$REPORT_FILE"
echo 'let value = option.ok_or_else(|| BearDogError::internal(' >> "$REPORT_FILE"
echo '    "Expected value to be present".to_string()' >> "$REPORT_FILE"
echo '))?;' >> "$REPORT_FILE"
echo '```' >> "$REPORT_FILE"

# Display summary
echo ""
echo -e "${GREEN}Analysis Complete!${NC}"
echo ""
echo "Summary:"
echo "  Production Code: $total_prod unwrap() calls"
echo "  Test Code:       $total_test unwrap() calls"
echo "  Total:           $((total_prod + total_test)) unwrap() calls"
echo ""
echo "Report saved to: $REPORT_FILE"
echo ""
echo -e "${YELLOW}Priority: Focus on the $total_prod production unwrap() calls first${NC}"

