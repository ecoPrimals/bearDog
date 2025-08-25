#!/bin/bash
# BearDog File Size Monitor
# Ensures all source files remain under 2000-line limit

set -e

echo "🔍 BearDog File Size Monitor"
echo "=============================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
MAX_LINES=2000
WARNING_THRESHOLD=1500

# Find all Rust source files and check their sizes
echo -e "${BLUE}📊 Checking file sizes in crates/...${NC}"
echo ""

violations=0
warnings=0
total_files=0

while IFS= read -r -d '' file; do
    lines=$(wc -l < "$file")
    total_files=$((total_files + 1))
    
    if [ "$lines" -gt "$MAX_LINES" ]; then
        echo -e "${RED}❌ VIOLATION: $file${NC} (${lines} lines - exceeds ${MAX_LINES} limit)"
        violations=$((violations + 1))
    elif [ "$lines" -gt "$WARNING_THRESHOLD" ]; then
        echo -e "${YELLOW}⚠️  WARNING: $file${NC} (${lines} lines - approaching ${MAX_LINES} limit)"
        warnings=$((warnings + 1))
    fi
done < <(find crates/ -name "*.rs" -print0)

echo ""
echo -e "${BLUE}📈 Summary:${NC}"
echo "   Total files checked: ${total_files}"
echo "   Files exceeding limit (${MAX_LINES} lines): ${violations}"
echo "   Files approaching limit (>${WARNING_THRESHOLD} lines): ${warnings}"

# Show largest files for reference
echo ""
echo -e "${BLUE}🔝 Top 10 largest files:${NC}"
find crates/ -name "*.rs" -exec wc -l {} + | sort -n | tail -11 | head -10 | while read lines file; do
    if [ "$lines" -gt "$WARNING_THRESHOLD" ]; then
        echo -e "   ${YELLOW}${lines}${NC} lines: $file"
    else
        echo -e "   ${GREEN}${lines}${NC} lines: $file"
    fi
done

echo ""

# Exit with error if violations found
if [ "$violations" -gt 0 ]; then
    echo -e "${RED}💥 FAILURE: ${violations} files exceed the ${MAX_LINES}-line limit${NC}"
    echo "   Please refactor large files into smaller, focused modules"
    exit 1
elif [ "$warnings" -gt 0 ]; then
    echo -e "${YELLOW}⚠️  SUCCESS WITH WARNINGS: ${warnings} files approaching the limit${NC}"
    echo "   Consider refactoring files approaching the ${MAX_LINES}-line limit"
    exit 0
else
    echo -e "${GREEN}✅ SUCCESS: All files are within size limits${NC}"
    exit 0
fi 