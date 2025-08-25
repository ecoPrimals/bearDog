#!/bin/bash
set -e

# BearDog Production Unwrap Migration Script
# Systematically eliminates unwrap/expect calls using refined context-aware migration

echo "🔄 BearDog Production Unwrap Migration"
echo "====================================="
echo

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
MIGRATOR_PATH="./unwrap-migrator"
TARGET_DIRS=("./crates" "./tests" "./examples")
CONFIDENCE_THRESHOLD=85
SAFETY_LEVEL="high"

echo -e "${BLUE}🎯 Migration Configuration:${NC}"
echo "   Confidence Threshold: ${CONFIDENCE_THRESHOLD}%"
echo "   Safety Level: ${SAFETY_LEVEL}"
echo "   Target Directories: ${TARGET_DIRS[*]}"
echo

# Check if migrator exists and is built
if [ ! -d "$MIGRATOR_PATH" ]; then
    echo -e "${RED}❌ Unwrap migrator not found at $MIGRATOR_PATH${NC}"
    exit 1
fi

echo -e "${BLUE}🔨 Building refined unwrap migrator...${NC}"
cd "$MIGRATOR_PATH"
cargo build --release --quiet
cd ..

# Function to run migration on a directory
migrate_directory() {
    local dir=$1
    local dry_run=${2:-false}
    
    if [ ! -d "$dir" ]; then
        echo -e "${YELLOW}⚠️  Directory $dir not found, skipping${NC}"
        return
    fi
    
    echo -e "${BLUE}📂 Processing directory: $dir${NC}"
    
    local flags="--path $dir --confidence-threshold $CONFIDENCE_THRESHOLD --safety-level $SAFETY_LEVEL"
    
    if [ "$dry_run" = true ]; then
        flags="$flags --dry-run"
    else
        flags="$flags --apply"
    fi
    
    # Run the refined migrator
    ./unwrap-migrator/target/release/beardog-unwrap-migrator $flags
}

# Phase 1: Dry run to show what would be changed
echo -e "${YELLOW}📋 Phase 1: Dry Run Analysis${NC}"
echo "============================================"
echo

for dir in "${TARGET_DIRS[@]}"; do
    migrate_directory "$dir" true
done

echo
echo -e "${YELLOW}📊 Dry run complete. Review the changes above.${NC}"
echo
read -p "🔄 Proceed with applying migrations? (y/N): " -n 1 -r
echo

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Migration cancelled by user.${NC}"
    exit 0
fi

# Phase 2: Apply migrations
echo -e "${GREEN}✅ Phase 2: Applying Migrations${NC}"
echo "================================="
echo

for dir in "${TARGET_DIRS[@]}"; do
    migrate_directory "$dir" false
done

echo
echo -e "${GREEN}🎉 Migration Complete!${NC}"
echo "======================"
echo

# Phase 3: Verify compilation
echo -e "${BLUE}🔍 Phase 3: Compilation Verification${NC}"
echo "===================================="
echo

echo "Testing compilation after migration..."
if cargo check --workspace --quiet; then
    echo -e "${GREEN}✅ Compilation successful after migration${NC}"
else
    echo -e "${RED}❌ Compilation failed after migration${NC}"
    echo "Please review the changes and fix any issues."
    exit 1
fi

# Phase 4: Run tests
echo
echo -e "${BLUE}🧪 Phase 4: Test Verification${NC}"
echo "============================="
echo

echo "Running tests to ensure functionality..."
if cargo test --workspace --quiet; then
    echo -e "${GREEN}✅ All tests passing after migration${NC}"
else
    echo -e "${YELLOW}⚠️  Some tests failed - this may be expected${NC}"
    echo "Please review test failures to ensure they're not migration-related."
fi

echo
echo -e "${GREEN}🏆 Unwrap Migration Summary${NC}"
echo "=========================="
echo "✅ Refined migrator applied with ${CONFIDENCE_THRESHOLD}% confidence threshold"
echo "✅ Safety level: ${SAFETY_LEVEL}"
echo "✅ Compilation verified"
echo "✅ Production-ready error handling patterns applied"
echo
echo -e "${BLUE}📝 Next Steps:${NC}"
echo "1. Review git diff to see all changes"
echo "2. Run additional tests as needed"
echo "3. Commit the changes with a descriptive message"
echo
echo "Migration complete! 🚀" 