#!/usr/bin/env bash
# Quick Debt Fixes - Automated Pattern Corrections
# Date: November 28, 2025
#
# This script applies safe, automated fixes for common anti-patterns

set -euo pipefail

echo "🐻 BearDog Quick Debt Fixes"
echo "================================"
echo

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

fixes_applied=0
errors=0

# Function to apply a fix and track results
apply_fix() {
    local description="$1"
    local command="$2"
    
    echo -n "🔧 $description... "
    
    if eval "$command" > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
        ((fixes_applied++))
    else
        echo -e "${RED}✗${NC}"
        ((errors++))
    fi
}

echo "Phase 1: Formatting & Linting"
echo "------------------------------"
apply_fix "Running cargo fmt" "cargo fmt"
apply_fix "Running clippy auto-fix" "cargo clippy --fix --allow-dirty --allow-staged --lib"

echo
echo "Phase 2: Pattern Fixes"  
echo "---------------------"

# Remove trailing whitespace
apply_fix "Removing trailing whitespace" "find crates -name '*.rs' -type f -exec sed -i 's/[ \t]*$//' {} +"

# Fix common doc formatting (TEST_CATEGORY, TEST_DOMAIN, TEST_PRIORITY)
apply_fix "Fixing test metadata formatting" "find crates -name '*_tests.rs' -type f -exec sed -i 's/\/\/\/ TEST_CATEGORY:/\/\/\/ \`TEST_CATEGORY\`:/g; s/\/\/\/ TEST_DOMAIN:/\/\/\/ \`TEST_DOMAIN\`:/g; s/\/\/\/ TEST_PRIORITY:/\/\/\/ \`TEST_PRIORITY\`:/g' {} +"

echo
echo "Phase 3: Build Verification"
echo "---------------------------"
apply_fix "Checking build" "cargo check --workspace --lib"

echo
echo "================================"
echo "Summary:"
echo "  Fixes Applied: $fixes_applied"
echo "  Errors: $errors"
echo
if [ $errors -eq 0 ]; then
    echo -e "${GREEN}✅ All fixes applied successfully!${NC}"
    exit 0
else
    echo -e "${YELLOW}⚠️  Some fixes had errors (see above)${NC}"
    exit 1
fi

