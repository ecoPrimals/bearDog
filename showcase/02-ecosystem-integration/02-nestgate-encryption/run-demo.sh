#!/usr/bin/env bash
# Run BearDog + NestGate Encryption Demo

set -e  # Exit on error

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🐻🏰 BearDog + NestGate Encryption Demo${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Build demo
echo -e "${YELLOW}Building demo...${NC}"
cargo build --release 2>&1 | grep -E "(Compiling|Finished)" || true
echo ""

# Check if sample data exists
if [ ! -f "data/sample_data.json" ]; then
    echo -e "${YELLOW}Error: Sample data not found at data/sample_data.json${NC}"
    exit 1
fi

# Run demo
echo -e "${GREEN}Running demo...${NC}"
echo ""
./target/release/beardog-nestgate-demo \
    --file data/sample_data.json \
    --config configs/demo.toml

echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Demo complete!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

