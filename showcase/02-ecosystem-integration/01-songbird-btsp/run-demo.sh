#!/usr/bin/env bash
# BearDog + Songbird BTSP Integration Demo Runner

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🐻🐦 BearDog + Songbird BTSP Integration Demo${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Check if Songbird is available
SONGBIRD_PATH="/home/eastgate/Development/ecoPrimals/songbird"
if [ ! -d "$SONGBIRD_PATH" ]; then
    echo -e "${RED}❌ Error: Songbird not found at $SONGBIRD_PATH${NC}"
    echo "Please ensure Songbird is cloned and built."
    exit 1
fi

# Check if Songbird tower is running
echo -e "${CYAN}Checking Songbird tower status...${NC}"
if pgrep -f "songbird.*tower" > /dev/null; then
    echo -e "${GREEN}✅ Songbird tower is running${NC}"
    TOWER_RUNNING=true
else
    echo -e "${YELLOW}⚠️  Songbird tower not running${NC}"
    echo -e "${YELLOW}Attempting to start Songbird tower...${NC}"
    TOWER_RUNNING=false
    
    # Try to start Songbird tower
    if [ -f "$SONGBIRD_PATH/showcase/02-federation/start-tower.sh" ]; then
        cd "$SONGBIRD_PATH/showcase/02-federation"
        ./start-tower.sh &
        TOWER_PID=$!
        echo -e "${GREEN}✅ Started Songbird tower (PID: $TOWER_PID)${NC}"
        echo "Waiting for tower to initialize..."
        sleep 5
    else
        echo -e "${RED}❌ Could not start Songbird tower${NC}"
        echo "Please start it manually:"
        echo "  cd $SONGBIRD_PATH/showcase/02-federation"
        echo "  ./start-tower.sh"
        exit 1
    fi
fi

cd "$(dirname "$0")"

# Build the demo
echo ""
echo -e "${CYAN}Building demo...${NC}"
if cargo build --release 2>&1 | grep -q "error"; then
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Build complete${NC}"

# Create log directory
mkdir -p logs

echo ""
echo -e "${CYAN}Starting BearDog nodes...${NC}"
echo ""

# Start Node B (Bob) first - it needs to be listening
echo -e "${YELLOW}Starting Node B (Bob - Responder)...${NC}"
cargo run --release -- --node bob --config configs/node-b.toml > logs/node-b.log 2>&1 &
BOB_PID=$!
echo -e "${GREEN}✅ Node B started (PID: $BOB_PID)${NC}"
echo "   Log: logs/node-b.log"

# Give Bob time to start
sleep 2

# Start Node A (Alice) - it will initiate the tunnel
echo ""
echo -e "${YELLOW}Starting Node A (Alice - Initiator)...${NC}"
cargo run --release -- --node alice --config configs/node-a.toml > logs/node-a.log 2>&1 &
ALICE_PID=$!
echo -e "${GREEN}✅ Node A started (PID: $ALICE_PID)${NC}"
echo "   Log: logs/node-a.log"

echo ""
echo -e "${CYAN}Demo running...${NC}"
echo ""
echo "Watch logs in real-time:"
echo "  ${YELLOW}tail -f logs/node-a.log${NC}  # Alice"
echo "  ${YELLOW}tail -f logs/node-b.log${NC}  # Bob"
echo ""
echo "Press Ctrl+C to stop..."
echo ""

# Function to cleanup on exit
cleanup() {
    echo ""
    echo -e "${YELLOW}Stopping nodes...${NC}"
    
    if [ -n "${ALICE_PID:-}" ]; then
        kill $ALICE_PID 2>/dev/null || true
        echo -e "${GREEN}✅ Stopped Alice${NC}"
    fi
    
    if [ -n "${BOB_PID:-}" ]; then
        kill $BOB_PID 2>/dev/null || true
        echo -e "${GREEN}✅ Stopped Bob${NC}"
    fi
    
    # If we started the tower, stop it
    if [ "$TOWER_RUNNING" = false ] && [ -n "${TOWER_PID:-}" ]; then
        kill $TOWER_PID 2>/dev/null || true
        echo -e "${GREEN}✅ Stopped Songbird tower${NC}"
    fi
    
    echo ""
    echo -e "${BLUE}Demo stopped. Check logs for results.${NC}"
}

trap cleanup EXIT INT TERM

# Wait for Alice to complete (it will exit after sending message)
wait $ALICE_PID 2>/dev/null || true

# Small delay for Bob to finish receiving
sleep 2

# Show results
echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📊 Demo Results${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo -e "${CYAN}Alice's Log (last 20 lines):${NC}"
echo -e "${YELLOW}───────────────────────────${NC}"
tail -20 logs/node-a.log
echo ""

echo -e "${CYAN}Bob's Log (last 20 lines):${NC}"
echo -e "${YELLOW}───────────────────────────${NC}"
tail -20 logs/node-b.log
echo ""

# Check for success indicators
if grep -q "SUCCESS" logs/node-a.log && grep -q "SUCCESS" logs/node-b.log; then
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}✅ DEMO SUCCESSFUL!${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo "Validated:"
    echo "  ✅ Songbird tower discovery"
    echo "  ✅ Node registration"
    echo "  ✅ BTSP tunnel establishment"
    echo "  ✅ End-to-end encryption"
    echo "  ✅ Perfect Forward Secrecy"
    echo "  ✅ Message transmission"
    echo ""
    echo -e "${GREEN}🐻🐦 BearDog + Songbird integration working!${NC}"
    EXIT_CODE=0
else
    echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${RED}❌ DEMO FAILED${NC}"
    echo -e "${RED}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo "Check logs for errors:"
    echo "  logs/node-a.log"
    echo "  logs/node-b.log"
    EXIT_CODE=1
fi

echo ""

# Cleanup will be called automatically
exit $EXIT_CODE

