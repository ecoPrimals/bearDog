#!/bin/bash
# BearDog Technical Debt Monitor
# Monitors for unwrap calls, TODO items, and other technical debt indicators

set -e

echo "🔍 BearDog Technical Debt Monitor"
echo "=================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check for production unwrap calls (excluding tests)
echo -e "${BLUE}🚨 Checking for production unwrap calls...${NC}"
production_unwraps=$(find crates/ -name "*.rs" -not -path "*/tests/*" -not -path "*/test_*" -exec grep -l "\.unwrap()" {} \; | wc -l)
if [ "$production_unwraps" -gt 0 ]; then
    echo -e "${YELLOW}⚠️  Found ${production_unwraps} files with .unwrap() calls in production code:${NC}"
    find crates/ -name "*.rs" -not -path "*/tests/*" -not -path "*/test_*" -exec grep -l "\.unwrap()" {} \; | head -5
    if [ "$production_unwraps" -gt 5 ]; then
        echo "   ... and $((production_unwraps - 5)) more files"
    fi
else
    echo -e "${GREEN}✅ No production unwrap calls found${NC}"
fi
echo ""

# Check for expect calls in production
echo -e "${BLUE}🔍 Checking for production expect calls...${NC}"
production_expects=$(find crates/ -name "*.rs" -not -path "*/tests/*" -not -path "*/test_*" -exec grep -l "\.expect(" {} \; | wc -l)
if [ "$production_expects" -gt 0 ]; then
    echo -e "${YELLOW}⚠️  Found ${production_expects} files with .expect() calls in production code${NC}"
else
    echo -e "${GREEN}✅ No production expect calls found${NC}"
fi
echo ""

# Check for TODO comments
echo -e "${BLUE}📝 Checking for TODO comments...${NC}"
todo_count=$(find crates/ -name "*.rs" -exec grep -c "TODO\|FIXME\|XXX\|HACK" {} \; 2>/dev/null | awk '{sum += $1} END {print sum+0}')
if [ "$todo_count" -gt 0 ]; then
    echo -e "${YELLOW}📋 Found ${todo_count} TODO/FIXME comments${NC}"
    # Show a few examples
    echo "   Examples:"
    find crates/ -name "*.rs" -exec grep -n "TODO\|FIXME\|XXX\|HACK" {} \; 2>/dev/null | head -3 | while read line; do
        echo "   - $line"
    done
else
    echo -e "${GREEN}✅ No TODO comments found${NC}"
fi
echo ""

# Check for async_trait usage (should be migrated to native async)
echo -e "${BLUE}⚡ Checking for async_trait usage...${NC}"
async_trait_count=$(find crates/ -name "*.rs" -exec grep -c "#\[async_trait\]" {} \; 2>/dev/null | awk '{sum += $1} END {print sum+0}')
if [ "$async_trait_count" -gt 0 ]; then
    echo -e "${YELLOW}📦 Found ${async_trait_count} async_trait usages (consider migrating to native async)${NC}"
else
    echo -e "${GREEN}✅ No async_trait usage found (fully migrated to native async)${NC}"
fi
echo ""

# Check for Box<dyn Trait> patterns (potential performance impact)
echo -e "${BLUE}📦 Checking for Box<dyn Trait> patterns...${NC}"
box_dyn_count=$(find crates/ -name "*.rs" -exec grep -c "Box<dyn" {} \; 2>/dev/null | awk '{sum += $1} END {print sum+0}')
if [ "$box_dyn_count" -gt 0 ]; then
    echo -e "${YELLOW}📦 Found ${box_dyn_count} Box<dyn Trait> patterns${NC}"
    echo "   (These may be acceptable for flexibility, but consider static dispatch where possible)"
else
    echo -e "${GREEN}✅ No Box<dyn Trait> patterns found${NC}"
fi
echo ""

# Check for panic! calls in production
echo -e "${BLUE}💥 Checking for panic! calls in production...${NC}"
panic_count=$(find crates/ -name "*.rs" -not -path "*/tests/*" -not -path "*/test_*" -exec grep -c "panic!" {} \; 2>/dev/null | awk '{sum += $1} END {print sum+0}')
if [ "$panic_count" -gt 0 ]; then
    echo -e "${RED}🚨 Found ${panic_count} panic! calls in production code${NC}"
    echo "   These should be replaced with proper error handling"
else
    echo -e "${GREEN}✅ No panic! calls found in production code${NC}"
fi
echo ""

# Summary
echo -e "${BLUE}📊 Technical Debt Summary:${NC}"
echo "   Production unwrap calls: ${production_unwraps}"
echo "   Production expect calls: ${production_expects}"
echo "   TODO/FIXME comments: ${todo_count}"
echo "   async_trait usage: ${async_trait_count}"
echo "   Box<dyn Trait> patterns: ${box_dyn_count}"
echo "   Production panic! calls: ${panic_count}"

# Calculate overall score
total_debt=$((production_unwraps + production_expects + todo_count + async_trait_count + panic_count * 5))

echo ""
if [ "$total_debt" -eq 0 ]; then
    echo -e "${GREEN}🎉 EXCELLENT: Zero technical debt detected!${NC}"
elif [ "$total_debt" -lt 10 ]; then
    echo -e "${GREEN}✅ VERY GOOD: Minimal technical debt (score: ${total_debt})${NC}"
elif [ "$total_debt" -lt 25 ]; then
    echo -e "${YELLOW}⚠️  GOOD: Low technical debt (score: ${total_debt})${NC}"
elif [ "$total_debt" -lt 50 ]; then
    echo -e "${YELLOW}⚠️  MODERATE: Some technical debt needs attention (score: ${total_debt})${NC}"
else
    echo -e "${RED}🚨 HIGH: Significant technical debt requires immediate attention (score: ${total_debt})${NC}"
fi

# Exit with appropriate code
if [ "$panic_count" -gt 0 ] || [ "$production_unwraps" -gt 10 ]; then
    exit 1
else
    exit 0
fi 