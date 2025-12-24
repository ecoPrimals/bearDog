#!/usr/bin/env bash
# BirdSong Privacy Enforcement Integration Test
# Tests that strangers cannot decrypt lineage-encrypted messages

set -e

BEARDOG="./target/release/beardog"
TEST_DIR="/tmp/beardog-birdsong-test-$$"

echo "🎵 BirdSong Privacy Enforcement Test"
echo "====================================="
echo ""

# Setup
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

echo "📋 Test Setup:"
echo "   Test directory: $TEST_DIR"
echo ""

# Step 1: Create lineage (A -> B -> C)
echo "🔑 Step 1: Creating lineage A -> B -> C..."
echo ""

# Generate root key (Node A)
echo "  Generating root key (Node A)..."
$BEARDOG key generate \
  --key-id node-a-root \
  --algorithm ed25519 \
  --hsm auto

# Derive child key (Node B)
echo "  Deriving child key (Node B)..."
$BEARDOG key derive \
  --master-key node-a-root \
  --purpose child \
  --output node-b-child

# Derive grandchild key (Node C)
echo "  Deriving grandchild key (Node C)..."
$BEARDOG key derive \
  --master-key node-b-child \
  --purpose grandchild \
  --output node-c-grandchild

# Generate stranger key (Node X)
echo "  Generating stranger key (Node X)..."
$BEARDOG key generate \
  --key-id node-x-stranger \
  --algorithm ed25519 \
  --hsm auto

echo ""
echo "✅ Lineage created:"
echo "   Root: node-a-root"
echo "   Child: node-b-child"
echo "   Grandchild: node-c-grandchild"
echo "   Stranger: node-x-stranger"
echo ""

# Step 2: Encrypt message for lineage
echo "🔒 Step 2: Encrypting message for lineage..."
echo ""

MESSAGE="This is a secret relay request from Node C"
echo "  Message: $MESSAGE"
echo "  Lineage: node-a-root"
echo "  Hint: DirectAncestors (only ancestors can decrypt)"
echo ""

$BEARDOG birdsong encrypt \
  --message "$MESSAGE" \
  --hint "DirectAncestors" \
  --root-id "node-a-root" \
  --output encrypted.birdsong

echo ""
echo "✅ Message encrypted!"
echo ""

# Step 3: Test decryption by ancestor (Node A) - SHOULD WORK
echo "🔓 Step 3: Testing decryption by Node A (ancestor)..."
echo ""

if $BEARDOG birdsong decrypt \
  --input encrypted.birdsong \
  --key-id node-a-root 2>&1 | grep -q "Decrypted successfully"; then
  echo "✅ SUCCESS: Node A (ancestor) can decrypt!"
else
  echo "❌ FAIL: Node A (ancestor) should be able to decrypt!"
  exit 1
fi

echo ""

# Step 4: Test decryption by sender (Node C) - SHOULD WORK
echo "🔓 Step 4: Testing decryption by Node C (sender)..."
echo ""

if $BEARDOG birdsong decrypt \
  --input encrypted.birdsong \
  --key-id node-c-grandchild 2>&1 | grep -q "Decrypted successfully"; then
  echo "✅ SUCCESS: Node C (sender) can decrypt!"
else
  echo "❌ FAIL: Node C (sender) should be able to decrypt!"
  exit 1
fi

echo ""

# Step 5: Test decryption by stranger (Node X) - SHOULD FAIL
echo "🔓 Step 5: Testing decryption by Node X (stranger)..."
echo ""

if $BEARDOG birdsong decrypt \
  --input encrypted.birdsong \
  --key-id node-x-stranger 2>&1 | grep -q "Cannot decrypt"; then
  echo "✅ SUCCESS: Node X (stranger) CANNOT decrypt! Privacy enforced!"
else
  echo "❌ FAIL: Node X (stranger) should NOT be able to decrypt!"
  exit 1
fi

echo ""
echo "🎉 All Tests Passed!"
echo ""
echo "📊 Summary:"
echo "   ✅ Node A (ancestor): Can decrypt"
echo "   ✅ Node C (sender): Can decrypt"
echo "   ✅ Node X (stranger): CANNOT decrypt (privacy enforced!)"
echo ""
echo "🔐 Privacy Enforcement: WORKING!"
echo ""

# Cleanup
cd /
rm -rf "$TEST_DIR"

echo "✅ Test complete!"

