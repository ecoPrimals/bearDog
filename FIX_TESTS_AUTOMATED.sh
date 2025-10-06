#!/bin/bash
# Automated Test Repair Script for BearDog
# Fixes the missing 'async' keyword in tokio::test functions
# Estimated time: 2-4 hours (mostly automated)

set -e  # Exit on error

echo "🔧 BearDog Test Suite Repair - Automated Script"
echo "=================================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Step 1: Backup
echo "📦 Step 1: Creating backup..."
BACKUP_DIR="tests_NEEDS_FIXING_BACKUP_$(date +%Y%m%d_%H%M%S)"
if [ ! -d "$BACKUP_DIR" ]; then
    cp -r tests_NEEDS_FIXING "$BACKUP_DIR"
    echo -e "${GREEN}✅ Backup created: $BACKUP_DIR${NC}"
else
    echo -e "${YELLOW}⚠️  Backup already exists, skipping${NC}"
fi
echo ""

# Step 2: Count files to fix
echo "📊 Step 2: Analyzing test files..."
TOTAL_FILES=$(find tests_NEEDS_FIXING -name "*.rs" -type f | wc -l)
echo "Total test files: $TOTAL_FILES"
echo ""

# Step 3: Fix async keywords in tokio::test functions
echo "🔧 Step 3: Fixing missing 'async' keywords..."
echo "This will add 'async' to functions marked with #[tokio::test]"
echo ""

FIXED_COUNT=0
find tests_NEEDS_FIXING -name "*.rs" -type f | while read file; do
    # Check if file has tokio::test without async
    if grep -Pzo '(?s)#\[tokio::test\]\s*\n\s*fn\s+' "$file" > /dev/null 2>&1; then
        # Use perl for multiline regex replacement
        perl -i -0pe 's/(#\[tokio::test\]\s*\n)(\s*)fn\s+/$1${2}async fn /g' "$file"
        echo "  Fixed: $file"
        FIXED_COUNT=$((FIXED_COUNT + 1))
    fi
done

echo ""
echo -e "${GREEN}✅ Fixed async keywords in files${NC}"
echo ""

# Step 4: Run initial compilation check
echo "🏗️  Step 4: Running initial compilation check..."
echo "This will identify remaining issues..."
echo ""

if cargo check --tests 2>&1 | tee test_fix_check.log; then
    echo -e "${GREEN}✅ All tests compile successfully!${NC}"
else
    ERROR_COUNT=$(grep -c "error:" test_fix_check.log || true)
    echo -e "${YELLOW}⚠️  Found $ERROR_COUNT compilation errors${NC}"
    echo "See test_fix_check.log for details"
    echo ""
    echo "Common remaining issues to fix manually:"
    echo "  1. Missing imports (use statements)"
    echo "  2. Type mismatches"
    echo "  3. API changes requiring updates"
    echo ""
    echo "Run: grep 'error:' test_fix_check.log | head -20"
fi
echo ""

# Step 5: Try to compile each test file individually
echo "🧪 Step 5: Testing individual files..."
echo "Checking which files compile successfully..."
echo ""

SUCCESS_COUNT=0
FAIL_COUNT=0
mkdir -p test_results

find tests_NEEDS_FIXING -name "*.rs" -type f | head -10 | while read file; do
    filename=$(basename "$file")
    if cargo test --test "${filename%.rs}" 2>&1 > "test_results/${filename}.log"; then
        echo -e "${GREEN}✅ $filename${NC}"
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
    else
        echo -e "${RED}❌ $filename${NC}"
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
done

echo ""
echo "📊 Initial Results (first 10 files):"
echo "  Success: $SUCCESS_COUNT"
echo "  Failed: $FAIL_COUNT"
echo ""

# Step 6: Summary and next steps
echo "📋 Step 6: Summary and Next Steps"
echo "=================================="
echo ""
echo "Automated fixes complete!"
echo ""
echo "What was fixed:"
echo "  ✅ Added 'async' keywords to tokio::test functions"
echo "  ✅ Created backup in: $BACKUP_DIR"
echo "  ✅ Generated compilation log: test_fix_check.log"
echo ""
echo "Next manual steps:"
echo "  1. Review test_fix_check.log for remaining errors"
echo "  2. Fix common issues:"
echo "     - Add missing imports"
echo "     - Update changed API calls"
echo "     - Fix type mismatches"
echo "  3. Move fixed tests from tests_NEEDS_FIXING/ to tests/"
echo "  4. Run: cargo test --workspace"
echo ""
echo "Expected outcome:"
echo "  - 80-90% of tests should compile after manual fixes"
echo "  - 2-4 hours total time (including manual work)"
echo "  - Test coverage: 4% → 40-50%"
echo ""
echo -e "${GREEN}🎉 Automated repair script complete!${NC}"
echo ""
echo "To continue, review errors with:"
echo "  grep 'error:' test_fix_check.log | less"
echo ""
echo "Or start manual fixes on individual files."

