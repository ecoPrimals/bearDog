#!/usr/bin/env bash
# BearDog - SAFETY Comment Checker
# Checks that all unsafe blocks have proper SAFETY comments

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🔒 BearDog SAFETY Comment Checker"
echo "================================="
echo

# Find all unsafe blocks in production code (exclude tests)
unsafe_blocks=$(find crates -name '*.rs' \
    ! -path '*/tests/*' \
    ! -path '*/test/*' \
    ! -name '*test*.rs' \
    -exec grep -n 'unsafe {' {} + | wc -l || true)

# Find unsafe blocks with SAFETY comments
# Look for SAFETY: or Safety: within 10 lines before unsafe {
safety_commented=0
missing_safety=0

echo "Analyzing unsafe blocks..."
echo

# Create temporary file for results
temp_file=$(mktemp)

find crates -name '*.rs' \
    ! -path '*/tests/*' \
    ! -path '*/test/*' \
    ! -name '*test*.rs' \
    -print0 | while IFS= read -r -d '' file; do
    
    # Find line numbers of unsafe blocks
    grep -n 'unsafe {' "$file" 2>/dev/null | while IFS=: read -r line_num _; do
        # Check 10 lines before for SAFETY comment
        start_line=$((line_num - 10))
        if [ $start_line -lt 1 ]; then
            start_line=1
        fi
        
        # Extract context and check for SAFETY comment
        context=$(sed -n "${start_line},${line_num}p" "$file")
        
        if echo "$context" | grep -qi "SAFETY:"; then
            echo "${file}:${line_num}:✅" >> "$temp_file"
        else
            echo "${file}:${line_num}:❌" >> "$temp_file"
        fi
    done
done

# Count results
if [ -f "$temp_file" ]; then
    safety_commented=$(grep -c "✅" "$temp_file" || echo "0")
    missing_safety=$(grep -c "❌" "$temp_file" || echo "0")
    
    echo "📊 Results:"
    echo "==========="
    echo -e "Total unsafe blocks:     ${YELLOW}$unsafe_blocks${NC}"
    echo -e "With SAFETY comments:    ${GREEN}$safety_commented${NC}"
    echo -e "Missing SAFETY comments: ${RED}$missing_safety${NC}"
    echo
    
    if [ $missing_safety -gt 0 ]; then
        echo -e "${YELLOW}⚠️  Files with missing SAFETY comments:${NC}"
        echo
        grep "❌" "$temp_file" | while IFS=: read -r file line status; do
            echo -e "  ${RED}❌${NC} $file:$line"
        done
        echo
    fi
    
    # Calculate percentage
    if [ $unsafe_blocks -gt 0 ]; then
        percentage=$((safety_commented * 100 / unsafe_blocks))
        echo -e "Coverage: ${GREEN}${percentage}%${NC}"
        echo
        
        if [ $percentage -lt 100 ]; then
            echo -e "${YELLOW}💡 Recommendation:${NC}"
            echo "   Add SAFETY comments to all unsafe blocks using this template:"
            echo
            echo "   // SAFETY: This is safe because:"
            echo "   // 1. [Memory safety guarantee]"
            echo "   // 2. [Null pointer check]"
            echo "   // 3. [Lifetime guarantee]"
            echo "   // 4. [Platform API guarantee]"
            echo "   // 5. [Error handling]"
            echo
            exit 1
        else
            echo -e "${GREEN}✅ All unsafe blocks have SAFETY comments!${NC}"
            exit 0
        fi
    fi
    
    rm -f "$temp_file"
else
    echo -e "${GREEN}✅ No unsafe blocks found in production code!${NC}"
    exit 0
fi

