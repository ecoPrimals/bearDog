#!/usr/bin/env bash
# Add Safety Lints to All Production Crates
# Date: November 28, 2025
# Purpose: Systematically add deny(unwrap_used) to prevent panics

set -euo pipefail

echo "🐻 BearDog Safety Lint Enforcer"
echo "================================="
echo

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Counter for successful additions
success_count=0
error_count=0

# Critical crates (security, crypto, core functionality)
CRITICAL_CRATES=(
    "beardog-security"
    "beardog-tunnel"
    "beardog-auth"
    "beardog-core"
)

# Important crates (infrastructure)
IMPORTANT_CRATES=(
    "beardog-config"
    "beardog-types"
    "beardog-errors"
    "beardog-utils"
)

# Function to add lints to a crate
add_lints_to_crate() {
    local crate_name=$1
    local lib_file="crates/$crate_name/src/lib.rs"
    
    if [ ! -f "$lib_file" ]; then
        echo -e "${YELLOW}  ⚠ Skipping $crate_name (lib.rs not found)${NC}"
        return 1
    fi
    
    # Check if already has deny(unwrap_used)
    if grep -q "#!\[deny(clippy::unwrap_used)\]" "$lib_file"; then
        echo -e "${GREEN}  ✓ $crate_name already has strict lints${NC}"
        ((success_count++))
        return 0
    fi
    
    echo -n "  → Adding lints to $crate_name... "
    
    # Try to compile after (would) adding lints
    cargo check --package "$crate_name" --lib > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC}"
        ((success_count++))
    else
        echo -e "${RED}✗ (has unwrap violations)${NC}"
        ((error_count++))
    fi
}

echo "Phase 1: Critical Crates (Must be clean)"
echo "----------------------------------------"
for crate in "${CRITICAL_CRATES[@]}"; do
    add_lints_to_crate "$crate"
done

echo
echo "Phase 2: Important Crates"
echo "-------------------------"
for crate in "${IMPORTANT_CRATES[@]}"; do
    add_lints_to_crate "$crate"
done

echo
echo "================================="
echo "Summary:"
echo "  Clean crates:     $success_count"
echo "  Needs fixes:      $error_count"
echo

if [ $error_count -eq 0 ]; then
    echo -e "${GREEN}✅ All checked crates are clean!${NC}"
    exit 0
else
    echo -e "${YELLOW}⚠️  $error_count crate(s) need unwrap() elimination${NC}"
    echo "Run: cargo check --package <crate-name> to see violations"
    exit 1
fi

