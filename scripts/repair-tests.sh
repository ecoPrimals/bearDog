#!/bin/bash
set -e

echo "🔧 BearDog Test Suite Repair - Automated Phase"
echo "=============================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Counter for fixes
fixes=0
files_processed=0

# Function to add .await to common patterns
fix_async_patterns() {
    local file="$1"
    local file_fixes=0
    
    # Common async function patterns
    patterns=(
        "discover_services()"
        "discover_hsm()"
        "connect()"
        "initialize()"
        "discover()"
        "health_check()"
        "validate()"
        "execute()"
        "process()"
        "run_comprehensive_workflow()"
        "run_chaos_engineering_tests()"
        "run_scalability_tests()"
        "run_security_validation()"
        "test_core_operations()"
        "test_security_integration()"
        "test_performance_benchmarks()"
        "test_concurrent_operations()"
        "execute_chaos_scenario()"
        "generate_key()"
        "sign_data()"
        "verify_signature()"
        "encrypt_data()"
        "decrypt_data()"
        "store_value()"
        "retrieve_value()"
        "new()"
    )
    
    for pattern in "${patterns[@]}"; do
        # Check if pattern exists without .await
        if grep -q "\.${pattern}[^.]" "$file" 2>/dev/null; then
            # Only add .await if not already present
            if ! grep -q "\.${pattern}\.await" "$file" 2>/dev/null; then
                sed -i "s/\.${pattern}\([^.]\)/\.${pattern}.await\1/g" "$file"
                ((file_fixes++))
                ((fixes++))
            fi
        fi
    done
    
    return $file_fixes
}

# Check if tests_NEEDS_FIXING directory exists
if [ ! -d "tests_NEEDS_FIXING" ]; then
    echo "${RED}❌ Error: tests_NEEDS_FIXING directory not found${NC}"
    echo "   Please run this script from the beardog root directory"
    exit 1
fi

# Create backup
echo "${BLUE}📦 Creating backup...${NC}"
if [ ! -d "tests_NEEDS_FIXING_BACKUP" ]; then
    cp -r tests_NEEDS_FIXING tests_NEEDS_FIXING_BACKUP
    echo "   ✓ Backup created at tests_NEEDS_FIXING_BACKUP/"
else
    echo "   ✓ Backup already exists"
fi
echo ""

# Process all Rust test files
echo "${BLUE}📝 Processing test files...${NC}"
while IFS= read -r file; do
    basename=$(basename "$file")
    fix_async_patterns "$file"
    file_fixes=$?
    ((files_processed++))
    
    if [ $file_fixes -gt 0 ]; then
        echo "   ${GREEN}✓${NC} $basename ($file_fixes fixes)"
    else
        echo "   ○ $basename (no changes)"
    fi
done < <(find tests_NEEDS_FIXING -name "*.rs" -type f)

echo ""
echo "${GREEN}✅ Automated fixes complete!${NC}"
echo ""
echo "   📊 Statistics:"
echo "      Files processed: $files_processed"
echo "      Async patterns fixed: $fixes"
echo ""
echo "${YELLOW}⚠️  Next steps:${NC}"
echo "   1. Verify changes:"
echo "      ${BLUE}cargo check --tests 2>&1 | tee test-errors.log${NC}"
echo ""
echo "   2. Count remaining errors:"
echo "      ${BLUE}grep -c 'error' test-errors.log${NC}"
echo ""
echo "   3. Review specific errors:"
echo "      ${BLUE}grep 'error\[' test-errors.log | head -20${NC}"
echo ""
echo "   4. Start manual fixes on Priority 1 files:"
echo "      ${BLUE}code tests_NEEDS_FIXING/adapter_integration_tests.rs${NC}"
echo ""
echo "${GREEN}🚀 Ready for Phase 2: Manual fixes${NC}"
echo ""

