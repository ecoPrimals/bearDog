#!/usr/bin/env bash
# Automated Hardcoding Migration Script
#
# This script helps automate the migration from hardcoded values to
# the configuration system and universal adapter patterns.
#
# Usage: ./scripts/migrate-hardcoding.sh [--dry-run] [--file FILE]

set -euo pipefail

COLOR_RED='\033[0;31m'
COLOR_GREEN='\033[0;32m'
COLOR_YELLOW='\033[1;33m'
COLOR_BLUE='\033[0;34m'
COLOR_RESET='\033[0;33m'

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$WORKSPACE_ROOT"

DRY_RUN=false
TARGET_FILE=""

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --file)
            TARGET_FILE="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--dry-run] [--file FILE]"
            exit 1
            ;;
    esac
done

echo -e "${COLOR_BLUE}🔄 Automated Hardcoding Migration${COLOR_RESET}"
echo -e "${COLOR_BLUE}===================================${COLOR_RESET}\n"

if [ "$DRY_RUN" = true ]; then
    echo -e "${COLOR_YELLOW}📝 DRY RUN MODE - No changes will be made${COLOR_RESET}\n"
fi

CHANGES_MADE=0

# Function to apply migration pattern
apply_migration() {
    local file="$1"
    local pattern="$2"
    local replacement="$3"
    local description="$4"
    
    if ! [ -f "$file" ]; then
        return 0
    fi
    
    # Check if pattern exists
    if ! grep -q "$pattern" "$file" 2>/dev/null; then
        return 0
    fi
    
    echo -e "${COLOR_YELLOW}Found:${COLOR_RESET} $description in $file"
    
    if [ "$DRY_RUN" = false ]; then
        # Create backup
        cp "$file" "$file.backup"
        
        # Apply replacement (using perl for better regex support)
        if perl -i -pe "$replacement" "$file" 2>/dev/null; then
            echo -e "${COLOR_GREEN}✅ Migrated${COLOR_RESET}"
            CHANGES_MADE=$((CHANGES_MADE + 1))
            rm -f "$file.backup"
        else
            echo -e "${COLOR_RED}❌ Failed${COLOR_RESET}"
            mv "$file.backup" "$file"
        fi
    else
        echo -e "${COLOR_BLUE}Would migrate (dry-run)${COLOR_RESET}"
    fi
    echo ""
}

# Function to deprecate struct/function
deprecate_item() {
    local file="$1"
    local item_type="$2"  # "struct", "fn", "impl"
    local item_name="$3"
    local deprecation_note="$4"
    
    if ! [ -f "$file" ]; then
        return 0
    fi
    
    # Check if item exists and is not already deprecated
    if ! grep -q "^\\s*pub\\s\\+$item_type\\s\\+$item_name" "$file" 2>/dev/null; then
        return 0
    fi
    
    if grep -B5 "pub\\s\\+$item_type\\s\\+$item_name" "$file" | grep -q "#\\[deprecated" 2>/dev/null; then
        return 0  # Already deprecated
    fi
    
    echo -e "${COLOR_YELLOW}Found:${COLOR_RESET} $item_type $item_name (not deprecated) in $file"
    
    if [ "$DRY_RUN" = false ]; then
        # Add deprecation attribute before the item
        cp "$file" "$file.backup"
        
        perl -i -pe "s/(pub\\s+$item_type\\s+$item_name)/#[deprecated(since = \"3.3.0\", note = \"$deprecation_note\")]\\n\\1/g" "$file"
        
        echo -e "${COLOR_GREEN}✅ Added deprecation${COLOR_RESET}"
        CHANGES_MADE=$((CHANGES_MADE + 1))
        rm -f "$file.backup"
    else
        echo -e "${COLOR_BLUE}Would add deprecation (dry-run)${COLOR_RESET}"
    fi
    echo ""
}

echo -e "${COLOR_YELLOW}Phase 1: Constant Migrations${COLOR_RESET}"
echo -e "${COLOR_YELLOW}=============================${COLOR_RESET}\n"

# Pattern 1: Replace hardcoded port constants
if [ -z "$TARGET_FILE" ]; then
    FILES=$(find crates/ -name "*.rs" -type f 2>/dev/null || true)
else
    FILES="$TARGET_FILE"
fi

for file in $FILES; do
    # Migrate DEFAULT_HTTP_PORT usage
    apply_migration "$file" \
        "DEFAULT_HTTP_PORT" \
        's/\bDEFAULT_HTTP_PORT\b/BEARDOG_CONFIG.network.api.port/g' \
        "DEFAULT_HTTP_PORT constant usage"
    
    # Migrate DEFAULT_HTTPS_PORT usage
    apply_migration "$file" \
        "DEFAULT_HTTPS_PORT" \
        's/\bDEFAULT_HTTPS_PORT\b/8443  \/\/ Standard HTTPS alternate port/g' \
        "DEFAULT_HTTPS_PORT constant usage"
    
    # Migrate hardcoded 8080 (API port)
    apply_migration "$file" \
        ":\\s*8080\\b" \
        's/:(\s*)8080\b/:$1env::var("BEARDOG_API_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080)/g' \
        "Hardcoded 8080 port"
done

echo -e "\n${COLOR_YELLOW}Phase 2: Primal Name Migrations${COLOR_RESET}"
echo -e "${COLOR_YELLOW}================================${COLOR_RESET}\n"

# Pattern 2: Add comments about primal name hardcoding
for file in $FILES; do
    # Find hardcoded "songbird" (case-insensitive in strings)
    if grep -qi '"songbird"' "$file" 2>/dev/null; then
        echo -e "${COLOR_YELLOW}Found:${COLOR_RESET} Hardcoded 'songbird' in $file"
        echo -e "${COLOR_BLUE}  → Manual review needed (see templates)${COLOR_RESET}\n"
    fi
    
    # Find hardcoded "beardog-" prefix (not UUID-based)
    if grep -q '"beardog-' "$file" 2>/dev/null; then
        echo -e "${COLOR_YELLOW}Found:${COLOR_RESET} Hardcoded 'beardog-' prefix in $file"
        echo -e "${COLOR_BLUE}  → Consider using PRIMAL_TYPE environment variable${COLOR_RESET}\n"
    fi
done

echo -e "\n${COLOR_YELLOW}Phase 3: Add Deprecation Warnings${COLOR_RESET}"
echo -e "${COLOR_YELLOW}===================================${COLOR_RESET}\n"

# Deprecate specific known items (examples - can be extended)
# These are already done, but this shows the pattern

echo -e "\n${COLOR_GREEN}📊 Migration Summary${COLOR_RESET}"
echo -e "${COLOR_GREEN}====================${COLOR_RESET}\n"

if [ "$DRY_RUN" = true ]; then
    echo -e "${COLOR_YELLOW}DRY RUN: No changes were made${COLOR_RESET}"
    echo -e "Re-run without --dry-run to apply changes"
else
    echo -e "${COLOR_GREEN}Changes made: $CHANGES_MADE${COLOR_RESET}"
    
    if [ $CHANGES_MADE -gt 0 ]; then
        echo -e "\n${COLOR_YELLOW}⚠️  IMPORTANT:${COLOR_RESET}"
        echo "1. Review changes with: git diff"
        echo "2. Run tests: cargo test"
        echo "3. Check for linter errors: cargo clippy"
        echo "4. Validate: ./scripts/test-zero-knowledge-deployment.sh"
    fi
fi

echo -e "\n${COLOR_BLUE}📚 Next Steps:${COLOR_RESET}"
echo "1. For complex migrations, refer to ecosystem-templates/*.rs"
echo "2. For vendor-specific code, see vendor-agnostic-migration-template.rs"
echo "3. For primal names, see primal-hardcoding-elimination-template.rs"
echo ""

if [ $CHANGES_MADE -eq 0 ] && [ "$DRY_RUN" = false ]; then
    echo -e "${COLOR_GREEN}✅ No automatic migrations needed!${COLOR_RESET}"
    echo "Your code may already be clean, or require manual migration."
fi

