#!/bin/bash
# Test Repair Script - Automated Import Path Updates
# Date: October 6, 2025
# Purpose: Fix outdated import paths in tests_NEEDS_FIXING/

set -e

echo "🔧 BearDog Test Repair Script"
echo "=============================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Create backup
BACKUP_DIR="tests_NEEDS_FIXING_BACKUP_$(date +%Y%m%d_%H%M%S)"
echo -e "${YELLOW}📦 Creating backup: $BACKUP_DIR${NC}"
cp -r tests_NEEDS_FIXING "$BACKUP_DIR"
echo -e "${GREEN}✅ Backup created${NC}"
echo ""

# Counter for files processed
TOTAL_FILES=0
FIXED_FILES=0
FAILED_FILES=0

# Function to fix imports in a file
fix_file() {
    local file="$1"
    local filename=$(basename "$file")
    
    echo -e "${YELLOW}🔧 Processing: $filename${NC}"
    TOTAL_FILES=$((TOTAL_FILES + 1))
    
    # Create temp file
    local temp_file="${file}.tmp"
    
    # Apply import path updates
    sed -e 's/use beardog::config::/use beardog_types::canonical::config::/g' \
        -e 's/use beardog::encryption::/use beardog_security::/g' \
        -e 's/use beardog::genetics::/use beardog_genetics::/g' \
        -e 's/use beardog::tunnel::/use beardog_tunnel::/g' \
        -e 's/use beardog::monitoring::/use beardog_monitoring::/g' \
        -e 's/use beardog::compliance::/use beardog_compliance::/g' \
        -e 's/use beardog::core::/use beardog_core::/g' \
        -e 's/use beardog::error::/use beardog_errors::/g' \
        -e 's/use beardog::types::/use beardog_types::/g' \
        -e 's/use beardog::utils::/use beardog_utils::/g' \
        -e 's/use beardog::auth::/use beardog_auth::/g' \
        -e 's/use beardog::threat::/use beardog_threat::/g' \
        -e 's/use beardog::adapters::/use beardog_adapters::/g' \
        -e 's/use beardog::workflows::/use beardog_workflows::/g' \
        "$file" > "$temp_file"
    
    # Check if changes were made
    if ! diff -q "$file" "$temp_file" > /dev/null 2>&1; then
        mv "$temp_file" "$file"
        echo -e "${GREEN}  ✅ Fixed imports${NC}"
        FIXED_FILES=$((FIXED_FILES + 1))
    else
        rm "$temp_file"
        echo -e "  ℹ️  No changes needed"
    fi
}

# Process all .rs files in tests_NEEDS_FIXING
echo "🔍 Scanning for test files..."
echo ""

while IFS= read -r -d '' file; do
    fix_file "$file"
done < <(find tests_NEEDS_FIXING -name "*.rs" -type f -print0)

echo ""
echo "=============================="
echo -e "${GREEN}📊 Summary${NC}"
echo "=============================="
echo "Total files processed: $TOTAL_FILES"
echo "Files fixed: $FIXED_FILES"
echo "Files unchanged: $((TOTAL_FILES - FIXED_FILES))"
echo ""
echo -e "${YELLOW}📝 Next Steps:${NC}"
echo "1. Review changes: git diff tests_NEEDS_FIXING/"
echo "2. Test compilation: ./scripts/test_repairs.sh"
echo "3. Move fixed files: ./scripts/move_fixed_tests.sh"
echo ""
echo -e "${GREEN}✅ Repair script complete!${NC}"

