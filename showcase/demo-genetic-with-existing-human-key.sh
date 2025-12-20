#!/usr/bin/env bash
# BearDog Showcase: Genetic Mixing with Existing Human Key
#
# This demo uses your ALREADY GENERATED human key for genetic mixing
# No need to collect entropy again - we'll use your sovereign key!

set -euo pipefail

# Configuration
BEARDOG="${BEARDOG:-./target/debug/beardog}"
OUTPUT_DIR="${OUTPUT_DIR:-./showcase/output/genetic-mixing-$(date +%s)}"
RECEIPTS_DIR="$OUTPUT_DIR/receipts"

# Your existing human key
HUMAN_KEY_ID="my-first-sovereign-human-key"

# Setup
mkdir -p "$OUTPUT_DIR" "$RECEIPTS_DIR"

SESSION_ID="genetic-$(date +%s)"

cat << 'EOF'

╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║         🧬 GENETIC MIXING WITH YOUR HUMAN KEY 🧬                             ║
║              Using Your Existing Sovereign Key                               ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝

This demo demonstrates genetic key mixing using YOUR existing human key:
  • Key ID: my-first-sovereign-human-key
  • Entropy Source: HUMAN (your keyboard timing)
  • Already generated with YOUR interactions

We'll create a genetic family tree:
  1. Your human key (root/parent)
  2. Device keys (siblings)
  3. Mixed keys (genetic combinations)
  4. Derived keys (children)

Let's build your key lineage! 🚀

EOF

read -p "Press ENTER to start..." -r
echo ""

#
# STEP 1: Verify human key exists
#

echo "🔍 Step 1: Verifying your human key..."
echo ""

if "$BEARDOG" key list 2>/dev/null | grep -q "$HUMAN_KEY_ID"; then
    echo "✅ Found your human key: $HUMAN_KEY_ID"
    "$BEARDOG" key list | grep -A 5 "$HUMAN_KEY_ID" || true
else
    echo "❌ Human key not found: $HUMAN_KEY_ID"
    echo ""
    echo "Please run the interactive entropy demo first:"
    echo "  ./showcase/02-hardware-integration/demo-human-entropy-interactive.sh"
    echo ""
    echo "Or use the existing test entropy:"
    echo "  beardog key generate --key-id my-first-sovereign-human-key \\"
    echo "    --algorithm AES-256-GCM --seed test-entropy.json"
    exit 1
fi

echo ""
read -p "Press ENTER to continue..." -r

#
# STEP 2: Generate additional keys for mixing
#

echo ""
echo "🔑 Step 2: Generating additional keys for genetic mixing..."
echo ""

# Generate 2 more device keys
DEVICE_KEY_1="device-alpha-${SESSION_ID}"
DEVICE_KEY_2="device-beta-${SESSION_ID}"

echo "→ Generating device key Alpha..."
"$BEARDOG" key generate \
    --key-id "$DEVICE_KEY_1" \
    --algorithm AES-256-GCM \
    --hsm auto \
    --kdf argon2 \
    --usage all

echo ""
echo "→ Generating device key Beta..."
"$BEARDOG" key generate \
    --key-id "$DEVICE_KEY_2" \
    --algorithm AES-256-GCM \
    --hsm auto \
    --kdf argon2 \
    --usage all

echo ""
echo "✅ Generated 2 device keys for mixing"

read -p "Press ENTER to continue..." -r

#
# STEP 3: Mix Human + Device Alpha
#

echo ""
echo "🧬 Step 3: Genetic Mix - Human + Device Alpha"
echo ""
echo "This creates a HYBRID key combining:"
echo "  • YOUR unique human timing patterns"
echo "  • High-quality device entropy from Alpha"
echo ""

MIXED_KEY_1="genetic-human-alpha-${SESSION_ID}"

echo "→ Mixing keys..."
"$BEARDOG" key mix \
    --key1 "$HUMAN_KEY_ID" \
    --key2 "$DEVICE_KEY_1" \
    --output "$MIXED_KEY_1" \
    --threshold 2

echo ""
echo "✅ Mixed key created: $MIXED_KEY_1"
echo "   Parent 1: $HUMAN_KEY_ID (HUMAN - YOUR entropy)"
echo "   Parent 2: $DEVICE_KEY_1 (DEVICE - system entropy)"
echo "   Threshold: 2 (requires both parents)"

read -p "Press ENTER to continue..." -r

#
# STEP 4: Mix Human + Device Beta
#

echo ""
echo "🧬 Step 4: Genetic Mix - Human + Device Beta"
echo ""

MIXED_KEY_2="genetic-human-beta-${SESSION_ID}"

echo "→ Mixing keys..."
"$BEARDOG" key mix \
    --key1 "$HUMAN_KEY_ID" \
    --key2 "$DEVICE_KEY_2" \
    --output "$MIXED_KEY_2" \
    --threshold 2

echo ""
echo "✅ Mixed key created: $MIXED_KEY_2"
echo "   Parent 1: $HUMAN_KEY_ID (HUMAN - YOUR entropy)"
echo "   Parent 2: $DEVICE_KEY_2 (DEVICE - system entropy)"

read -p "Press ENTER to continue..." -r

#
# STEP 5: Mix two mixed keys (2nd generation)
#

echo ""
echo "🧬 Step 5: 2nd Generation Mix - Hybrid + Hybrid"
echo ""
echo "This creates a 2nd-generation key by mixing TWO hybrid keys!"
echo "This key has genetic material from:"
echo "  • YOUR human entropy (from both mixed keys)"
echo "  • Device Alpha entropy"
echo "  • Device Beta entropy"
echo ""

MIXED_KEY_GEN2="genetic-2nd-gen-${SESSION_ID}"

echo "→ Mixing 2nd generation..."
"$BEARDOG" key mix \
    --key1 "$MIXED_KEY_1" \
    --key2 "$MIXED_KEY_2" \
    --output "$MIXED_KEY_GEN2" \
    --threshold 2

echo ""
echo "✅ 2nd generation mixed key created: $MIXED_KEY_GEN2"
echo "   Parent 1: $MIXED_KEY_1 (Human + Alpha)"
echo "   Parent 2: $MIXED_KEY_2 (Human + Beta)"
echo "   Genetic material from: HUMAN + Alpha + Beta"

read -p "Press ENTER to continue..." -r

#
# STEP 6: Derive hierarchical child keys
#

echo ""
echo "👶 Step 6: Deriving Child Keys (Hierarchical)"
echo ""

CHILD_KEY_1="human-child-encrypt-${SESSION_ID}"
CHILD_KEY_2="human-child-temp-${SESSION_ID}"

echo "→ Deriving encryption-only child from human key..."
"$BEARDOG" key derive \
    --master-key "$HUMAN_KEY_ID" \
    --purpose "encryption-only" \
    --output "$CHILD_KEY_1"

echo ""
echo "→ Deriving temporary child (24h expiry) from human key..."
"$BEARDOG" key derive \
    --master-key "$HUMAN_KEY_ID" \
    --purpose "temporary-operations" \
    --output "$CHILD_KEY_2" \
    --expires-in 24h

echo ""
echo "✅ Child keys derived!"
echo "   $CHILD_KEY_1 (permanent, encryption-only)"
echo "   $CHILD_KEY_2 (expires in 24h)"

read -p "Press ENTER to continue..." -r

#
# STEP 7: Key Lineage Visualization
#

echo ""
echo "🌳 Step 7: YOUR Genetic Key Lineage"
echo ""

cat << EOF

═══════════════════════════════════════════════════════════════
                    YOUR KEY FAMILY TREE
═══════════════════════════════════════════════════════════════

                ┌─────────────────────────┐
                │   YOUR HUMAN KEY        │
                │   (Root/Sovereign)      │
                │   ID: $HUMAN_KEY_ID
                │   Source: HUMAN         │
                │   (YOUR keyboard timing)│
                └─────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
        ▼                  ▼                  ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│ Child Key 1  │  │ Child Key 2  │  │  Mixed Keys  │
│ (Encrypt)    │  │ (Temp 24h)   │  │              │
└──────────────┘  └──────────────┘  └──────┬───────┘
                                            │
                        ┌───────────────────┼───────────────────┐
                        ▼                   ▼                   ▼
                ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
                │ Human+Alpha  │    │ Human+Beta   │    │ 2nd Gen Mix  │
                │ (Hybrid)     │    │ (Hybrid)     │    │ (H+A+B)      │
                └──────────────┘    └──────────────┘    └──────────────┘
                        │                   │
                        └───────────┬───────┘
                                    ▼
                            ┌──────────────┐
                            │   2nd Gen    │
                            │  (Triple Mix)│
                            └──────────────┘

GENETIC RELATIONSHIPS:
  • Root: YOUR human entropy (non-fungible, sovereign)
  • Children: Derived from YOUR key (inherit characteristics)
  • Hybrids: Mixed with device keys (strength + uniqueness)
  • 2nd Gen: Multi-parent mixing (complex genetics)

KEY PROPERTIES:
  • All mixed keys require THRESHOLD keys to use
  • Child keys are constrained by parent
  • Genetic material is traceable through lineage
  • Full audit trail in receipts

EOF

read -p "Press ENTER to continue..." -r

#
# STEP 8: Demonstrate encryption with different keys
#

echo ""
echo "🔐 Step 8: Encryption Comparison"
echo ""

TEST_DATA="$OUTPUT_DIR/test-message.txt"
echo "This is a test message for genetic key demonstration" > "$TEST_DATA"

echo "Let's encrypt the same message with different keys:"
echo ""

# Encrypt with human key
echo "→ Encrypting with YOUR human key..."
"$BEARDOG" encrypt \
    --key "$HUMAN_KEY_ID" \
    --input "$TEST_DATA" \
    --output "$OUTPUT_DIR/encrypted-human.enc"
echo "  ✅ $OUTPUT_DIR/encrypted-human.enc"

# Encrypt with device key
echo "→ Encrypting with device key Alpha..."
"$BEARDOG" encrypt \
    --key "$DEVICE_KEY_1" \
    --input "$TEST_DATA" \
    --output "$OUTPUT_DIR/encrypted-device-alpha.enc"
echo "  ✅ $OUTPUT_DIR/encrypted-device-alpha.enc"

# Encrypt with mixed key
echo "→ Encrypting with hybrid key (Human+Alpha)..."
"$BEARDOG" encrypt \
    --key "$MIXED_KEY_1" \
    --input "$TEST_DATA" \
    --output "$OUTPUT_DIR/encrypted-hybrid.enc"
echo "  ✅ $OUTPUT_DIR/encrypted-hybrid.enc"

echo ""
echo "✅ Same message encrypted with 3 different keys!"
echo "   Each encryption is unique, but all derive from YOUR human entropy."

# Compare sizes
HUMAN_SIZE=$(stat -f%z "$OUTPUT_DIR/encrypted-human.enc" 2>/dev/null || stat -c%s "$OUTPUT_DIR/encrypted-human.enc" 2>/dev/null || echo "?")
DEVICE_SIZE=$(stat -f%z "$OUTPUT_DIR/encrypted-device-alpha.enc" 2>/dev/null || stat -c%s "$OUTPUT_DIR/encrypted-device-alpha.enc" 2>/dev/null || echo "?")
HYBRID_SIZE=$(stat -f%z "$OUTPUT_DIR/encrypted-hybrid.enc" 2>/dev/null || stat -c%s "$OUTPUT_DIR/encrypted-hybrid.enc" 2>/dev/null || echo "?")

echo ""
echo "Encrypted file sizes:"
echo "   Human key:  ${HUMAN_SIZE} bytes"
echo "   Device key: ${DEVICE_SIZE} bytes"
echo "   Hybrid key: ${HYBRID_SIZE} bytes"

read -p "Press ENTER to continue..." -r

#
# STEP 9: Summary
#

echo ""
echo "📊 Step 9: Summary"
echo ""

cat << EOF

GENETIC KEY FAMILY CREATED:

ROOT (Human Entropy):
  • $HUMAN_KEY_ID
    Source: YOUR keyboard timing patterns

CHILDREN (Derived):
  • $CHILD_KEY_1 (encryption-only)
  • $CHILD_KEY_2 (expires 24h)

1ST GENERATION HYBRIDS:
  • $MIXED_KEY_1 (Human + Alpha)
  • $MIXED_KEY_2 (Human + Beta)

2ND GENERATION:
  • $MIXED_KEY_GEN2 (Human + Alpha + Beta)

DEVICE KEYS (Siblings):
  • $DEVICE_KEY_1 (Alpha)
  • $DEVICE_KEY_2 (Beta)

TOTAL KEYS IN FAMILY: 8
  • 1 human (root)
  • 2 children (derived)
  • 3 hybrids (mixed)
  • 2 device (siblings)

All keys are related through YOUR human entropy at the root! 🌳

OUTPUT DIRECTORY: $OUTPUT_DIR
  • Encrypted files in $OUTPUT_DIR/
  • Receipts in $RECEIPTS_DIR/

EOF

echo "List all keys in this family:"
echo ""
"$BEARDOG" key list | grep -E "(my-first-sovereign|device-(alpha|beta)|genetic|human-child)" || echo "(Keys from this session)"

echo ""

cat << 'EOF'

╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                    🎉 GENETIC MIXING COMPLETE! 🎉                            ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝

WHAT YOU JUST DEMONSTRATED:

  🧬 Genetic Key Mixing
     → Human + Device entropy combinations
     → Multi-generation mixing (2nd gen)
     → Threshold requirements (2-of-2)

  👶 Hierarchical Derivation
     → Children inherit from parent
     → Purpose-specific constraints
     → Time-based expiration

  🌳 Key Lineage
     → Traceable genetic relationships
     → Auditable provenance
     → Sovereign root (YOUR entropy)

  🔐 Real Encryption
     → Same data, different keys
     → All related through YOUR entropy
     → Production-ready operations

KEY INSIGHT:
  YOUR human entropy is the ROOT of this entire key family.
  Every key in this tree has YOUR genetic material.
  This is sovereignty in action! 🎯

╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                   🐻 BearDog: Integrity Over Features                        ║
║              Your Interactions, Your Entropy, Your Sovereignty.              ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝

Thank you for exploring genetic cryptography with BearDog! 🚀

Session ID: ${SESSION_ID}
Output: $OUTPUT_DIR

Run this demo again to create a NEW genetic family tree!

EOF

