#!/bin/bash
# Tower Atomic Registration Validation Script
# Tests BearDog auto-registration with Neural API

set -e

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                                                               ║"
echo "║   🧪 TOWER ATOMIC REGISTRATION VALIDATION                    ║"
echo "║                                                               ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Cleanup function
cleanup() {
    echo ""
    echo "${YELLOW}🧹 Cleaning up...${NC}"
    
    if [ -n "$BEARDOG_PID" ]; then
        echo "  Stopping BearDog (PID: $BEARDOG_PID)..."
        kill $BEARDOG_PID 2>/dev/null || true
    fi
    
    if [ -n "$NEURAL_PID" ]; then
        echo "  Stopping Neural API (PID: $NEURAL_PID)..."
        kill $NEURAL_PID 2>/dev/null || true
    fi
    
    # Clean up socket files
    rm -f /tmp/beardog-test.sock 2>/dev/null || true
    rm -f /tmp/neural-api-test.sock 2>/dev/null || true
    
    echo "${GREEN}✅ Cleanup complete${NC}"
}

# Set trap for cleanup
trap cleanup EXIT INT TERM

# Test 1: Verify discover_neural_api_socket() finds default paths
echo "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo "${BLUE}Test 1: Socket Discovery (Default Paths)${NC}"
echo "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo ""

# Create a mock Neural API socket
echo "${YELLOW}📝 Creating mock Neural API socket at /tmp/neural-api.sock...${NC}"
touch /tmp/neural-api.sock

# Build BearDog to ensure latest code
echo "${YELLOW}🔨 Building BearDog...${NC}"
cd "$(dirname "$0")"
cargo build --release --bin beardog 2>&1 | grep -E "(Compiling|Finished)" || true
echo ""

# Test socket discovery
echo "${YELLOW}🔍 Testing socket discovery...${NC}"
export FAMILY_ID="test"
export NODE_ID="test-node"

# Start BearDog and capture logs
echo "${YELLOW}🚀 Starting BearDog...${NC}"
./target/release/beardog server --socket /tmp/beardog-test.sock > /tmp/beardog-test.log 2>&1 &
BEARDOG_PID=$!

# Wait for startup
sleep 3

# Check logs for socket discovery
if grep -q "Found Neural API socket at default path: /tmp/neural-api.sock" /tmp/beardog-test.log; then
    echo "${GREEN}✅ Test 1 PASSED: Socket discovery working${NC}"
else
    echo "${RED}❌ Test 1 FAILED: Socket not discovered${NC}"
    echo ""
    echo "BearDog logs:"
    tail -20 /tmp/beardog-test.log
    exit 1
fi

echo ""

# Clean up for next test
kill $BEARDOG_PID 2>/dev/null || true
rm -f /tmp/beardog-test.sock /tmp/neural-api.sock
sleep 1

# Test 2: Verify NEURAL_API_SOCKET env var takes precedence
echo "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo "${BLUE}Test 2: Environment Variable Override${NC}"
echo "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo ""

echo "${YELLOW}📝 Creating mock Neural API socket at /tmp/neural-api-custom.sock...${NC}"
touch /tmp/neural-api-custom.sock

echo "${YELLOW}🔍 Testing env var override...${NC}"
export NEURAL_API_SOCKET="/tmp/neural-api-custom.sock"

./target/release/beardog server --socket /tmp/beardog-test.sock > /tmp/beardog-test.log 2>&1 &
BEARDOG_PID=$!

sleep 3

if grep -q "Using NEURAL_API_SOCKET: /tmp/neural-api-custom.sock" /tmp/beardog-test.log; then
    echo "${GREEN}✅ Test 2 PASSED: Environment variable override working${NC}"
else
    echo "${RED}❌ Test 2 FAILED: Environment variable not respected${NC}"
    echo ""
    echo "BearDog logs:"
    tail -20 /tmp/beardog-test.log
    exit 1
fi

echo ""

# Clean up for next test
kill $BEARDOG_PID 2>/dev/null || true
unset NEURAL_API_SOCKET
rm -f /tmp/beardog-test.sock /tmp/neural-api-custom.sock
sleep 1

# Test 3: Verify graceful fallback when no Neural API found
echo "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo "${BLUE}Test 3: Graceful Fallback (No Neural API)${NC}"
echo "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo ""

echo "${YELLOW}🔍 Testing fallback behavior (no Neural API)...${NC}"

./target/release/beardog server --socket /tmp/beardog-test.sock > /tmp/beardog-test.log 2>&1 &
BEARDOG_PID=$!

sleep 3

if grep -q "No Neural API socket found" /tmp/beardog-test.log && \
   grep -q "running in standalone mode" /tmp/beardog-test.log; then
    echo "${GREEN}✅ Test 3 PASSED: Graceful fallback working${NC}"
else
    echo "${RED}❌ Test 3 FAILED: Fallback behavior incorrect${NC}"
    echo ""
    echo "BearDog logs:"
    tail -20 /tmp/beardog-test.log
    exit 1
fi

echo ""

# Clean up
kill $BEARDOG_PID 2>/dev/null || true
rm -f /tmp/beardog-test.sock
sleep 1

# Summary
echo "${GREEN}╔═══════════════════════════════════════════════════════════════╗${NC}"
echo "${GREEN}║                                                               ║${NC}"
echo "${GREEN}║   ✅ ALL TESTS PASSED - TOWER ATOMIC REGISTRATION READY      ║${NC}"
echo "${GREEN}║                                                               ║${NC}"
echo "${GREEN}╚═══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "${BLUE}Test Results:${NC}"
echo "  ${GREEN}✅${NC} Socket discovery (default paths)"
echo "  ${GREEN}✅${NC} Environment variable override"
echo "  ${GREEN}✅${NC} Graceful fallback (no Neural API)"
echo ""
echo "${BLUE}Next Steps:${NC}"
echo "  1. Start Neural API: ${YELLOW}biomeos neural-api${NC}"
echo "  2. Start BearDog: ${YELLOW}beardog server${NC}"
echo "  3. Verify registration in BearDog logs"
echo "  4. Test capability.call from Songbird"
echo ""
echo "${GREEN}Ready for Tower Atomic end-to-end validation!${NC}"

