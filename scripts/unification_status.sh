#!/bin/bash
# BearDog Unification Status Tracker
# Provides real-time metrics on unification progress

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🐻 BearDog Unification Status Tracker${NC}"
echo "========================================"
echo ""

# 1. File Size Discipline
echo -e "${GREEN}📏 FILE SIZE DISCIPLINE${NC}"
FILES_OVER_2000=$(find crates -name "*.rs" -exec wc -l {} \; | awk '$1 > 2000 {print $1, $2}' | wc -l)
FILES_OVER_1000=$(find crates -name "*.rs" -exec wc -l {} \; | awk '$1 > 1000 {print $1, $2}' | wc -l)
TOTAL_FILES=$(find crates -name "*.rs" | wc -l)
TOTAL_LINES=$(find crates -name "*.rs" -exec wc -l {} \; | awk '{sum+=$1} END {print sum}')
AVG_LINES=$(echo "$TOTAL_LINES / $TOTAL_FILES" | bc)

echo "Total Rust files:      $TOTAL_FILES"
echo "Total lines of code:   $TOTAL_LINES"
echo "Average file size:     $AVG_LINES lines"
echo "Files > 2000 lines:    $FILES_OVER_2000 ✅"
echo "Files > 1000 lines:    $FILES_OVER_1000"
echo ""

# 2. Type Alias Usage
echo -e "${YELLOW}🔤 TYPE ALIAS STATUS${NC}"
BEARDOG_RESULT_ACTIVE=$(grep -r "BearDogResult" crates --include="*.rs" | grep -v "test" | grep -v "deprecated" | wc -l)
BEARDOG_RESULT_TOTAL=$(grep -r "BearDogResult" crates --include="*.rs" | wc -l)
echo "BearDogResult (active): $BEARDOG_RESULT_ACTIVE ⚠️"
echo "BearDogResult (total):  $BEARDOG_RESULT_TOTAL"
echo "Target:                 0"
echo ""

# 3. Unwrap/Expect Usage
echo -e "${YELLOW}⚠️  ERROR HANDLING${NC}"
UNWRAP_EXPECT_TOTAL=$(grep -r "\.unwrap()\|\.expect(" crates --include="*.rs" | wc -l)
UNWRAP_EXPECT_TESTS=$(grep -r "\.unwrap()\|\.expect(" crates --include="*.rs" | grep "test" | wc -l)
UNWRAP_EXPECT_PROD=$((UNWRAP_EXPECT_TOTAL - UNWRAP_EXPECT_TESTS))
echo "Unwrap/expect (total):       $UNWRAP_EXPECT_TOTAL"
echo "Unwrap/expect (tests):       $UNWRAP_EXPECT_TESTS ✅"
echo "Unwrap/expect (production):  $UNWRAP_EXPECT_PROD ⚠️"
echo "Target (production):         <50"
echo ""

# 4. Async Trait Usage
echo -e "${GREEN}⚡ ASYNC TRAIT STATUS${NC}"
ASYNC_TRAIT_TOTAL=$(grep -r "#\[async_trait\]" crates --include="*.rs" | wc -l)
ASYNC_TRAIT_TESTS=$(grep -r "#\[async_trait\]" crates --include="*.rs" | grep "test" | wc -l)
ASYNC_TRAIT_PROD=$((ASYNC_TRAIT_TOTAL - ASYNC_TRAIT_TESTS))
echo "async_trait (total):       $ASYNC_TRAIT_TOTAL"
echo "async_trait (tests):       $ASYNC_TRAIT_TESTS ✅"
echo "async_trait (production):  $ASYNC_TRAIT_PROD"
echo "Target:                    0"
echo ""

# 5. Config Structs
echo -e "${YELLOW}⚙️  CONFIG STRUCTS${NC}"
CONFIG_STRUCTS=$(grep -r "pub struct.*Config" crates --include="*.rs" | wc -l)
CONFIG_CANONICAL=$(grep -r "pub struct.*Config" crates/beardog-types/src/canonical --include="*.rs" | wc -l)
CONFIG_CANONICAL_PCT=$((CONFIG_CANONICAL * 100 / CONFIG_STRUCTS))
echo "Total Config structs:       $CONFIG_STRUCTS"
echo "Canonical Config structs:   $CONFIG_CANONICAL"
echo "Canonical coverage:         ${CONFIG_CANONICAL_PCT}%"
echo "Target:                     90%+"
echo ""

# 6. Constants
echo -e "${GREEN}📊 CONSTANTS${NC}"
CONST_DEFS=$(grep -r "^pub const.*TIMEOUT\|^pub const.*BUFFER\|^pub const.*PORT\|^pub const.*RETRY" crates --include="*.rs" | wc -l)
CONST_CANONICAL=$(grep -r "^pub const.*TIMEOUT\|^pub const.*BUFFER\|^pub const.*PORT\|^pub const.*RETRY" crates/beardog-types/src/constants --include="*.rs" | wc -l)
echo "Total constants:       $CONST_DEFS"
echo "Canonical constants:   $CONST_CANONICAL"
echo "Target:                95%+ canonical"
echo ""

# 7. Compilation Status
echo -e "${BLUE}🔨 COMPILATION STATUS${NC}"
if cargo check --workspace &> /dev/null; then
    echo -e "${GREEN}✅ Workspace compiles successfully${NC}"
else
    echo -e "${RED}❌ Compilation errors detected${NC}"
fi
echo ""

# 8. Test Status
echo -e "${BLUE}🧪 TEST STATUS${NC}"
if cargo test --workspace --no-fail-fast &> /dev/null; then
    echo -e "${GREEN}✅ All tests passing${NC}"
else
    echo -e "${YELLOW}⚠️  Some tests failing${NC}"
fi
echo ""

# 9. Overall Grade Estimation
echo -e "${BLUE}📈 ESTIMATED GRADE${NC}"
# Simple heuristic based on key metrics
SCORE=60

# File discipline bonus (max 10 points)
if [ "$FILES_OVER_2000" -eq 0 ]; then
    SCORE=$((SCORE + 10))
fi

# Type alias penalty
if [ "$BEARDOG_RESULT_ACTIVE" -lt 100 ]; then
    SCORE=$((SCORE + 5))
fi

# Error handling
if [ "$UNWRAP_EXPECT_PROD" -lt 300 ]; then
    SCORE=$((SCORE + 5))
elif [ "$UNWRAP_EXPECT_PROD" -lt 600 ]; then
    SCORE=$((SCORE + 3))
fi

# Async trait
if [ "$ASYNC_TRAIT_PROD" -lt 5 ]; then
    SCORE=$((SCORE + 5))
fi

# Config canonical coverage
if [ "$CONFIG_CANONICAL_PCT" -gt 80 ]; then
    SCORE=$((SCORE + 10))
elif [ "$CONFIG_CANONICAL_PCT" -gt 60 ]; then
    SCORE=$((SCORE + 7))
elif [ "$CONFIG_CANONICAL_PCT" -gt 40 ]; then
    SCORE=$((SCORE + 4))
fi

echo "Estimated Grade: $SCORE/100"
if [ "$SCORE" -ge 90 ]; then
    echo -e "${GREEN}Status: Excellent (A) ✅${NC}"
elif [ "$SCORE" -ge 80 ]; then
    echo -e "${GREEN}Status: Good (B) ✅${NC}"
elif [ "$SCORE" -ge 70 ]; then
    echo -e "${YELLOW}Status: Satisfactory (C) ⚠️${NC}"
else
    echo -e "${RED}Status: Needs Improvement (D) ⚠️${NC}"
fi
echo ""

# 10. Next Actions
echo -e "${BLUE}🎯 PRIORITY ACTIONS${NC}"
if [ "$BEARDOG_RESULT_ACTIVE" -gt 0 ]; then
    echo -e "${YELLOW}1. Migrate BearDogResult → Result<T, E> ($BEARDOG_RESULT_ACTIVE usages)${NC}"
fi
if [ "$UNWRAP_EXPECT_PROD" -gt 100 ]; then
    echo -e "${RED}2. Fix critical unwrap/expect ($UNWRAP_EXPECT_PROD in production)${NC}"
fi
if [ "$ASYNC_TRAIT_PROD" -gt 0 ]; then
    echo -e "${YELLOW}3. Complete async trait migration ($ASYNC_TRAIT_PROD remaining)${NC}"
fi
if [ "$CONFIG_CANONICAL_PCT" -lt 90 ]; then
    echo -e "${YELLOW}4. Continue config consolidation (${CONFIG_CANONICAL_PCT}% → 90%+)${NC}"
fi
echo ""

echo -e "${GREEN}✅ Status check complete!${NC}"
echo "Last updated: $(date)"

