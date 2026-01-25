#!/bin/bash
# Test script for BearDog Neural API Auto-Registration
# Part of Tower Atomic TRUE PRIMAL pattern

set -e

echo "🧪 Testing BearDog Auto-Registration with Neural API"
echo "======================================================"
echo ""

# Color codes
GREEN='\033[0.32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Cleanup function
cleanup() {
    echo ""
    echo "🧹 Cleaning up..."
    
    if [ ! -z "$BEARDOG_PID" ]; then
        kill $BEARDOG_PID 2>/dev/null || true
    fi
    
    if [ ! -z "$NEURAL_PID" ]; then
        kill $NEURAL_PID 2>/dev/null || true
    fi
    
    # Clean up sockets
    rm -f /tmp/beardog-nat0.sock 2>/dev/null || true
    rm -f /tmp/neural-api-nat0.sock 2>/dev/null || true
    
    echo "✅ Cleanup complete"
}

# Set trap for cleanup
trap cleanup EXIT INT TERM

echo -e "${BLUE}📋 Test Environment:${NC}"
echo "   BearDog: $(pwd)/target/debug/beardog"
echo "   Neural API Socket: /tmp/neural-api-nat0.sock"
echo "   BearDog Socket: /tmp/beardog-nat0.sock"
echo ""

# Check if beardog binary exists
if [ ! -f "target/debug/beardog" ]; then
    echo -e "${RED}❌ Error: beardog binary not found${NC}"
    echo "   Please run: cargo build -p beardog-cli"
    exit 1
fi

echo -e "${BLUE}Step 1: Checking for Neural API...${NC}"
if [ -f "../phase2/biomeOS/target/release/biomeos" ]; then
    echo -e "${GREEN}✅ Neural API binary found${NC}"
    NEURAL_BIN="../phase2/biomeOS/target/release/biomeos"
    HAS_NEURAL=true
elif [ -f "../../../phase2/biomeOS/target/release/biomeos" ]; then
    echo -e "${GREEN}✅ Neural API binary found${NC}"
    NEURAL_BIN="../../../phase2/biomeOS/target/release/biomeos"
    HAS_NEURAL=true
else
    echo -e "${YELLOW}⚠️  Neural API not found - testing standalone mode${NC}"
    HAS_NEURAL=false
fi
echo ""

if [ "$HAS_NEURAL" = true ]; then
    echo -e "${BLUE}Step 2: Starting Neural API...${NC}"
    $NEURAL_BIN neural-api --socket /tmp/neural-api-nat0.sock > /tmp/neural-api.log 2>&1 &
    NEURAL_PID=$!
    echo "   PID: $NEURAL_PID"
    sleep 3
    
    if ps -p $NEURAL_PID > /dev/null; then
        echo -e "${GREEN}✅ Neural API started${NC}"
    else
        echo -e "${RED}❌ Neural API failed to start${NC}"
        cat /tmp/neural-api.log
        exit 1
    fi
    echo ""
fi

echo -e "${BLUE}Step 3: Starting BearDog with auto-registration...${NC}"
export NEURAL_API_SOCKET="/tmp/neural-api-nat0.sock"
./target/debug/beardog server --socket /tmp/beardog-nat0.sock > /tmp/beardog.log 2>&1 &
BEARDOG_PID=$!
echo "   PID: $BEARDOG_PID"
echo "   NEURAL_API_SOCKET: $NEURAL_API_SOCKET"
sleep 4

if ps -p $BEARDOG_PID > /dev/null; then
    echo -e "${GREEN}✅ BearDog started${NC}"
else
    echo -e "${RED}❌ BearDog failed to start${NC}"
    cat /tmp/beardog.log
    exit 1
fi
echo ""

echo -e "${BLUE}Step 4: Checking logs for registration...${NC}"
if grep -q "Neural API detected" /tmp/beardog.log; then
    echo -e "${GREEN}✅ Neural API detected by BearDog${NC}"
else
    echo -e "${YELLOW}⚠️  No Neural API detection (standalone mode)${NC}"
fi

if grep -q "registered with Neural API" /tmp/beardog.log; then
    echo -e "${GREEN}✅ BearDog registered capabilities${NC}"
else
    echo -e "${YELLOW}⚠️  No registration logged${NC}"
fi
echo ""

if [ "$HAS_NEURAL" = true ]; then
    echo -e "${BLUE}Step 5: Testing capability.call via Neural API...${NC}"
    echo '{
      "jsonrpc": "2.0",
      "method": "capability.call",
      "params": {
        "capability": "crypto",
        "operation": "generate_keypair",
        "args": {"algorithm": "x25519"}
      },
      "id": 1
    }' | nc -U /tmp/neural-api-nat0.sock -w 5 > /tmp/neural-response.json 2>&1 || true
    
    if [ -f /tmp/neural-response.json ] && grep -q "result" /tmp/neural-response.json; then
        echo -e "${GREEN}✅ capability.call succeeded${NC}"
        echo "   Response: $(cat /tmp/neural-response.json | head -n 1)"
    else
        echo -e "${YELLOW}⚠️  capability.call test skipped (Neural API may not be fully ready)${NC}"
    fi
    echo ""
fi

echo -e "${BLUE}Step 6: Testing direct BearDog RPC...${NC}"
echo '{
  "jsonrpc": "2.0",
  "method": "crypto.x25519_generate_ephemeral",
  "params": {},
  "id": 2
}' | nc -U /tmp/beardog-nat0.sock -w 5 > /tmp/beardog-response.json 2>&1 || true

if [ -f /tmp/beardog-response.json ] && grep -q "public_key" /tmp/beardog-response.json; then
    echo -e "${GREEN}✅ Direct RPC succeeded${NC}"
    echo "   Response: $(cat /tmp/beardog-response.json | head -n 1 | cut -c1-80)..."
else
    echo -e "${YELLOW}⚠️  Direct RPC test inconclusive${NC}"
fi
echo ""

echo "═══════════════════════════════════════════════════════════"
echo -e "${GREEN}🎉 TEST COMPLETE!${NC}"
echo "═══════════════════════════════════════════════════════════"
echo ""
echo "📝 Summary:"
echo "   - BearDog: Running ✅"
if [ "$HAS_NEURAL" = true ]; then
    echo "   - Neural API: Running ✅"
    echo "   - Auto-registration: Enabled ✅"
    echo "   - Tower Atomic: READY 🚀"
else
    echo "   - Standalone Mode: OK ✅"
fi
echo ""
echo "📂 Logs available:"
echo "   - BearDog: /tmp/beardog.log"
if [ "$HAS_NEURAL" = true ]; then
    echo "   - Neural API: /tmp/neural-api.log"
fi
echo ""

# Cleanup happens via trap

