#!/bin/bash
# ================================================================================================
# BEARDOG UNWRAP MIGRATION SCRIPT
# ================================================================================================
#
# This script runs the BearDog specialized unwrap migrator to systematically eliminate
# unwrap/expect calls and replace them with proper BearDogError handling.
#
# Usage:
#   ./scripts/run_unwrap_migration.sh --dry-run    # Preview changes
#   ./scripts/run_unwrap_migration.sh --apply      # Apply changes
#   ./scripts/run_unwrap_migration.sh --stats-only # Show statistics

set -euo pipefail

echo "🔧 BearDog Unwrap Migration Script"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]] || [[ ! -d "crates" ]]; then
    echo -e "${RED}❌ Error: This script must be run from the BearDog root directory${NC}"
    exit 1
fi

# Build the migrator tool first
echo -e "${YELLOW}📦 Building BearDog unwrap migrator...${NC}"
if ! cargo build --bin beardog-unwrap-migrator --release; then
    echo -e "${RED}❌ Failed to build the migrator tool${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Migrator built successfully${NC}"
echo ""

# Run the migrator with provided arguments
echo -e "${BLUE}🚀 Running BearDog unwrap migration...${NC}"
echo ""

# Pass all arguments to the migrator
exec cargo run --bin beardog-unwrap-migrator --release -- "$@" 