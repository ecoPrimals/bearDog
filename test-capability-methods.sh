#!/bin/bash

# Test BearDog Capability-Based IPC Methods
# Agnostic architecture - works with any primal

SOCKET="/tmp/beardog-test.sock"

echo "═══════════════════════════════════════════════════════════════"
echo "  Testing BearDog Capability-Based Methods"
echo "  (Primal Agnostic - No Hardcoding)"
echo "═══════════════════════════════════════════════════════════════"
echo ""

# Test 1: Health (Universal)
echo "1. Testing health check..."
echo '{"jsonrpc":"2.0","method":"health.check","id":1}' | nc -U "$SOCKET" -w 1 2>/dev/null
echo ""

# Test 2: Identity (Self-knowledge only)
echo "2. Testing identity..."
echo '{"jsonrpc":"2.0","method":"identity","id":2}' | nc -U "$SOCKET" -w 1 2>/dev/null
echo ""

# Test 3: Capabilities (Self-description)
echo "3. Testing capabilities..."
echo '{"jsonrpc":"2.0","method":"capabilities","id":3}' | nc -U "$SOCKET" -w 1 2>/dev/null
echo ""

# Test 4: Trust Evaluation (Capability-based)
echo "4. Testing trust.evaluate_peer..."
echo '{"jsonrpc":"2.0","method":"trust.evaluate_peer","params":{"peer_id":"test-peer","peer_family":"nat0"},"id":4}' | nc -U "$SOCKET" -w 1 2>/dev/null
echo ""

# Test 5: Security namespace
echo "5. Testing security.evaluate..."
echo '{"jsonrpc":"2.0","method":"security.evaluate","params":{"peer_id":"test-peer","family":"nat0"},"id":5}' | nc -U "$SOCKET" -w 1 2>/dev/null
echo ""

# Test 6: Lineage
echo "6. Testing lineage..."
echo '{"jsonrpc":"2.0","method":"trust.get_lineage","id":6}' | nc -U "$SOCKET" -w 1 2>/dev/null
echo ""

echo "═══════════════════════════════════════════════════════════════"
echo "  All tests complete!"
echo "═══════════════════════════════════════════════════════════════"

