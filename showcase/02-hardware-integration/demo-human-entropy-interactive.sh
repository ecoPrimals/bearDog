#!/usr/bin/env bash
# BearDog Showcase: Interactive Human Entropy Collection
# 
# This demo showcases LIVE human interaction entropy collection
# and demonstrates genetic key mixing with REAL human entropy.
#
# CRITICAL: This collects REAL human input (NO SIMULATION)

set -euo pipefail

# Source robust demo functions
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SHOWCASE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Fallback print functions (always define these)
print_header() { echo -e "\n=== $1 ===" ; }
print_step() { echo -e "\n→ $1" ; }
print_success() { echo "✅ $1" ; }
print_info() { echo "ℹ️  $1" ; }
print_warning() { echo "⚠️  $1" ; }
print_error() { echo "❌ $1" ; }
wait_for_user() { read -p "Press ENTER to continue..." -r ; }

# Load demo functions if available (will override above if present)
if [[ -f "$SHOWCASE_ROOT/lib/robust_demo_functions.sh" ]]; then
    source "$SHOWCASE_ROOT/lib/robust_demo_functions.sh"
fi

# Configuration
BEARDOG="${BEARDOG:-./target/debug/beardog}"
OUTPUT_DIR="${OUTPUT_DIR:-./showcase/output/human-entropy-$(date +%s)}"
RECEIPTS_DIR="$OUTPUT_DIR/receipts"
ENTROPY_DIR="$OUTPUT_DIR/entropy"
KEYS_DIR="$OUTPUT_DIR/keys"

# Setup
mkdir -p "$OUTPUT_DIR" "$RECEIPTS_DIR" "$ENTROPY_DIR" "$KEYS_DIR"

SESSION_ID="human-$(date +%s)"

# Banner
cat << 'EOF'

╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║           🎤 LIVE HUMAN ENTROPY & GENETIC MIXING SHOWCASE 🎤                 ║
║                      BearDog Interactive Demo                                ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝

This showcase demonstrates:
  1. LIVE human interaction entropy collection (keyboard + mouse)
  2. Sovereign key generation from YOUR entropy
  3. Genetic key mixing with human and device entropy
  4. Full audit trail with receipts

IMPORTANT: 
  • This collects REAL entropy from YOUR interactions
  • NO SIMULATION - only live human input accepted
  • Your timing patterns = cryptographic entropy
  • Each collection is unique (non-fungible)

EOF

wait_for_user

#
# PART 1: COLLECT LIVE HUMAN ENTROPY
#

print_header "Part 1: Collect LIVE Human Entropy"

print_info "This step collects entropy from YOUR keyboard and mouse interactions."
print_info "You will see an interactive UI - just type naturally and move your mouse!"
echo ""
print_step "Preparing to collect human entropy..."
wait_for_user

echo ""
print_info "Starting interactive entropy collection..."
print_info "Follow the on-screen instructions in the interactive UI."
echo ""

HUMAN_ENTROPY_FILE="$ENTROPY_DIR/human-entropy-${SESSION_ID}.json"

"$BEARDOG" entropy collect \
    --human-input \
    --device auto \
    --output "$HUMAN_ENTROPY_FILE" || {
    echo "❌ Entropy collection failed or cancelled"
    exit 1
}

if [[ -f "$HUMAN_ENTROPY_FILE" ]]; then
    print_success "Human entropy collected!"
    
    # Display seed info
    SEED_ID=$(jq -r '.seed_id' "$HUMAN_ENTROPY_FILE")
    QUALITY=$(jq -r '.quality_score' "$HUMAN_ENTROPY_FILE")
    QUALITY_PERCENT=$(echo "$QUALITY * 100" | bc -l | xargs printf "%.1f")
    HUMAN_INPUT=$(jq -r '.human_input' "$HUMAN_ENTROPY_FILE")
    
    echo ""
    echo "📊 Entropy Seed Details:"
    echo "   Seed ID: $SEED_ID"
    echo "   Quality: ${QUALITY_PERCENT}%"
    echo "   Human Input: $HUMAN_INPUT"
    echo "   File: $HUMAN_ENTROPY_FILE"
else
    echo "❌ Entropy file not created"
    exit 1
fi

wait_for_user

#
# PART 2: GENERATE SOVEREIGN HUMAN KEY
#

print_header "Part 2: Generate YOUR Sovereign Key"

print_info "Now we'll generate a cryptographic key using YOUR human entropy."
print_info "This key is derived from YOUR unique interaction timing patterns."
echo ""

HUMAN_KEY_ID="human-sovereign-key-${SESSION_ID}"

print_step "Generating key from human entropy..."

"$BEARDOG" key generate \
    --key-id "$HUMAN_KEY_ID" \
    --algorithm AES-256-GCM \
    --hsm auto \
    --kdf argon2 \
    --seed "$HUMAN_ENTROPY_FILE" \
    --usage all

print_success "Sovereign human key generated!"
echo "   Key ID: $HUMAN_KEY_ID"
echo "   Algorithm: AES-256-GCM"
echo "   Entropy Source: HUMAN (YOUR interactions)"

wait_for_user

#
# PART 3: GENERATE DEVICE KEY FOR COMPARISON
#

print_header "Part 3: Generate Device Key (for comparison)"

print_info "Let's generate a standard device-entropy key for comparison."

DEVICE_KEY_ID="device-key-${SESSION_ID}"

print_step "Generating device entropy key..."

"$BEARDOG" key generate \
    --key-id "$DEVICE_KEY_ID" \
    --algorithm AES-256-GCM \
    --hsm auto \
    --kdf argon2 \
    --usage all

print_success "Device key generated!"
echo "   Key ID: $DEVICE_KEY_ID"
echo "   Algorithm: AES-256-GCM"
echo "   Entropy Source: DEVICE (system)"

wait_for_user

#
# PART 4: GENETIC KEY MIXING
#

print_header "Part 4: Genetic Key Mixing (Human + Device)"

print_info "Now for the magic: Let's MIX your human key with the device key!"
print_info "This creates a HYBRID key combining:"
print_info "  • YOUR unique human timing patterns"
print_info "  • High-quality device entropy"
print_info "  • Cryptographic strength from both sources"
echo ""

MIXED_KEY_ID="genetic-mixed-human-device-${SESSION_ID}"

print_step "Mixing human and device keys..."

"$BEARDOG" key mix \
    --key1 "$HUMAN_KEY_ID" \
    --key2 "$DEVICE_KEY_ID" \
    --output "$MIXED_KEY_ID" \
    --threshold 2

print_success "Genetic mixed key created!"
echo "   Key ID: $MIXED_KEY_ID"
echo "   Parent 1: $HUMAN_KEY_ID (HUMAN)"
echo "   Parent 2: $DEVICE_KEY_ID (DEVICE)"
echo "   Threshold: 2 (requires both parents)"

wait_for_user

#
# PART 5: DERIVE CHILD KEY FROM HUMAN KEY
#

print_header "Part 5: Derive Child Key (Hierarchical)"

print_info "Let's derive a CHILD key from your human key."
print_info "This creates a key hierarchy with YOUR human entropy at the root."

CHILD_KEY_ID="human-child-${SESSION_ID}"

print_step "Deriving child key from human key..."

"$BEARDOG" key derive \
    --master-key "$HUMAN_KEY_ID" \
    --purpose "showcase-demo" \
    --output "$CHILD_KEY_ID" \
    --expires-in 24h

print_success "Child key derived!"
echo "   Key ID: $CHILD_KEY_ID"
echo "   Parent: $HUMAN_KEY_ID (YOUR human key)"
echo "   Purpose: showcase-demo"
echo "   Expires: 24 hours"

wait_for_user

#
# PART 6: KEY LINEAGE VISUALIZATION
#

print_header "Part 6: Key Lineage (Your Genetic Tree)"

print_info "Let's visualize the genetic tree of keys derived from YOUR entropy:"
echo ""

cat << EOF

YOUR KEY LINEAGE:
═════════════════

┌─────────────────────────────────────┐
│  Human Entropy Collection           │
│  (YOUR keyboard + mouse timing)     │
│  Quality: ${QUALITY_PERCENT}%                         │
└─────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────┐
│  Sovereign Human Key                │
│  ID: $HUMAN_KEY_ID
│  Algorithm: AES-256-GCM             │
│  Entropy: HUMAN (non-fungible)      │
└─────────────────────────────────────┘
         │                      │
         ▼                      ▼
┌──────────────────┐   ┌──────────────────┐
│  Child Key       │   │  Mixed Key       │
│  (Derived)       │   │  (Genetic)       │
│  Expires: 24h    │   │  Human + Device  │
└──────────────────┘   └──────────────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │  Device Key         │
                    │  (System Entropy)   │
                    └─────────────────────┘

KEY INSIGHTS:
  • Root: YOUR human entropy (sovereign, non-fungible)
  • Child: Inherits your entropy characteristics
  • Mixed: Combines YOUR uniqueness + device strength
  • All auditable with cryptographic receipts

EOF

wait_for_user

#
# PART 7: DEMONSTRATE ENCRYPTION WITH HUMAN KEY
#

print_header "Part 7: Encrypt Data with YOUR Human Key"

print_info "Let's encrypt some data using your sovereign human key."

TEST_DATA="$OUTPUT_DIR/secret-message.txt"
ENCRYPTED_DATA="$OUTPUT_DIR/secret-message.enc"

echo "This is a secret message encrypted with human-derived entropy!" > "$TEST_DATA"

print_step "Encrypting data with human key..."

"$BEARDOG" encrypt \
    --key "$HUMAN_KEY_ID" \
    --input "$TEST_DATA" \
    --output "$ENCRYPTED_DATA"

print_success "Data encrypted!"
echo "   Input: $TEST_DATA"
echo "   Output: $ENCRYPTED_DATA"
echo "   Key: $HUMAN_KEY_ID (YOUR human key)"

# Show file sizes
PLAIN_SIZE=$(stat -f%z "$TEST_DATA" 2>/dev/null || stat -c%s "$TEST_DATA" 2>/dev/null || echo "?")
ENC_SIZE=$(stat -f%z "$ENCRYPTED_DATA" 2>/dev/null || stat -c%s "$ENCRYPTED_DATA" 2>/dev/null || echo "?")

echo ""
echo "   Plain size: ${PLAIN_SIZE} bytes"
echo "   Encrypted size: ${ENC_SIZE} bytes"

wait_for_user

#
# PART 8: DECRYPT AND VERIFY
#

print_header "Part 8: Decrypt and Verify"

DECRYPTED_DATA="$OUTPUT_DIR/secret-message-decrypted.txt"

print_step "Decrypting data with human key..."

"$BEARDOG" decrypt \
    --key "$HUMAN_KEY_ID" \
    --input "$ENCRYPTED_DATA" \
    --output "$DECRYPTED_DATA"

print_success "Data decrypted!"

# Verify
if diff -q "$TEST_DATA" "$DECRYPTED_DATA" > /dev/null 2>&1; then
    print_success "✅ VERIFIED: Decrypted data matches original!"
else
    echo "❌ ERROR: Decrypted data does not match!"
    exit 1
fi

wait_for_user

#
# PART 9: SUMMARY & RECEIPTS
#

print_header "Part 9: Summary & Audit Trail"

print_info "Let's review what we accomplished:"
echo ""

# Count receipts
RECEIPT_COUNT=$(find "$RECEIPTS_DIR" -name "*.json" 2>/dev/null | wc -l | tr -d ' ')

cat << EOF

ACCOMPLISHMENTS:
  ✅ Collected LIVE human entropy (keyboard + mouse timing)
  ✅ Generated sovereign key from YOUR entropy
  ✅ Created device key for comparison
  ✅ Mixed human + device keys (genetic cryptography)
  ✅ Derived hierarchical child key
  ✅ Encrypted data with human key
  ✅ Decrypted and verified data
  ✅ Full audit trail with receipts

KEYS CREATED:
  1. $HUMAN_KEY_ID (HUMAN - YOUR entropy)
  2. $DEVICE_KEY_ID (DEVICE - system entropy)
  3. $MIXED_KEY_ID (MIXED - genetic)
  4. $CHILD_KEY_ID (CHILD - derived)

RECEIPTS GENERATED: ${RECEIPT_COUNT}
  Location: $RECEIPTS_DIR

OUTPUT DIRECTORY:
  $OUTPUT_DIR

EOF

print_info "All receipts are cryptographically signed and auditable."

# List keys
print_step "Current keys in system:"
"$BEARDOG" key list | grep -E "(human-|device-|genetic-mixed|human-child)" || echo "  (Keys from this session)"

wait_for_user

#
# FINALE
#

cat << 'EOF'

╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                    🎉 SHOWCASE COMPLETE! 🎉                                  ║
║                                                                              ║
║              Human Entropy → Sovereign Keys → Genetic Mixing                ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝

WHAT YOU JUST EXPERIENCED:

  🎤 LIVE Human Entropy Collection
     → Your keyboard timing patterns
     → Your mouse movement variations
     → Real-time quality metrics
     → LiveFeedValidator enforcement (NO SIMULATION)

  🔑 Sovereign Key Generation
     → Key derived from YOUR entropy
     → Non-fungible (unique to you)
     → Auditable provenance
     → Full cryptographic strength

  🧬 Genetic Cryptography
     → Mixed human + device entropy
     → Hierarchical key derivation
     → Parent-child relationships
     → Threshold requirements

  🔐 Real-World Usage
     → Encrypted data with your key
     → Decrypted and verified
     → End-to-end working flow
     → Production-ready system

KEY INSIGHTS:

  • Your typing rhythm IS cryptographic entropy
  • Each collection is unique (non-fungible)
  • Human entropy can be mixed with device entropy
  • Keys can form genetic hierarchies
  • Full audit trail for compliance
  • Privacy preserved (no keystrokes stored)

THIS IS NOT A DEMO. THIS IS PRODUCTION.

Your interactions created real cryptographic keys.
Your timing patterns are now security guarantees.
Your uniqueness is now sovereignty.

╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                   🐻 BearDog: Integrity Over Features                        ║
║              Your Interactions, Your Entropy, Your Sovereignty.              ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝

Thank you for participating in the future of human-centered cryptography! 🚀

Session ID: ${SESSION_ID}
Output: $OUTPUT_DIR

EOF

echo ""
print_info "Run this demo again to collect MORE unique entropy!"
print_info "Each run creates new, non-fungible keys from YOUR interactions."
echo ""

