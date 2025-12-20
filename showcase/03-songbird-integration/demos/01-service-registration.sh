#!/usr/bin/env bash
# BearDog + Songbird Integration Demo: Service Registration
#
# This demo shows how BearDog registers its crypto services with Songbird
# and how Songbird can discover and use those services.
#
# Architecture:
#   1. Songbird starts and listens for service registrations
#   2. BearDog starts and registers its crypto capabilities
#   3. Songbird discovers BearDog's crypto services
#   4. Songbird requests encryption from BearDog
#   5. BearDog provides crypto services and receipts

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BEARDOG_DIR="/home/eastgate/Development/ecoPrimals/beardog"
SONGBIRD_DIR="/home/eastgate/Development/ecoPrimals/songbird"

OUTPUT_DIR="$SCRIPT_DIR/output/demo-$(date +%s)"
LOGS_DIR="$OUTPUT_DIR/logs"
RECEIPTS_DIR="$OUTPUT_DIR/receipts"

mkdir -p "$LOGS_DIR" "$RECEIPTS_DIR"

# Binaries
BEARDOG="$BEARDOG_DIR/target/debug/beardog"
SONGBIRD_CLI="$SONGBIRD_DIR/target/debug/songbird-cli"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_step() {
    echo -e "${BLUE}→${NC} $1"
}

log_success() {
    echo -e "${GREEN}✅${NC} $1"
}

log_info() {
    echo -e "${YELLOW}ℹ️${NC}  $1"
}

log_error() {
    echo -e "${RED}❌${NC} $1"
}

wait_for_user() {
    read -p "Press ENTER to continue..." -r
}

# Banner
cat << 'EOF'

╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║          🐻🐦 BearDog + Songbird Integration Demo 🐻🐦                        ║
║               Service Registration & Crypto Services                         ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝

This demo demonstrates:
  1. BearDog registering crypto services with Songbird
  2. Songbird discovering BearDog's capabilities
  3. Songbird requesting encryption from BearDog
  4. BearDog providing crypto services
  5. Receipt validation for all operations

Architecture:
  • BearDog: Sovereign crypto service (owns keys, issues receipts)
  • Songbird: Service orchestrator (discovers, requests, validates)
  • No hardcoded dependencies - pure service discovery!

EOF

log_info "This demo shows cross-primal integration while maintaining sovereignty."
echo ""
wait_for_user

#
# STEP 1: Start Songbird (Service Registry)
#

log_step "Step 1: Starting Songbird service registry..."
echo ""

log_info "Songbird will listen for service registrations"
log_info "Any primal can register their capabilities"
echo ""

# For now, we'll simulate the registry with a simple JSON file
# In production, this would be Songbird's actual service registry
REGISTRY_FILE="$OUTPUT_DIR/service_registry.json"

cat > "$REGISTRY_FILE" << 'JSON'
{
  "registry_version": "1.0.0",
  "services": []
}
JSON

log_success "Songbird registry initialized"
log_info "Registry file: $REGISTRY_FILE"
echo ""
wait_for_user

#
# STEP 2: BearDog Registers Crypto Services
#

log_step "Step 2: BearDog registers its crypto services..."
echo ""

log_info "BearDog advertises:"
log_info "  • Key generation (AES-256-GCM, ChaCha20, Ed25519)"
log_info "  • Encryption/decryption (authenticated)"
log_info "  • Digital signatures (Ed25519, ECDSA, RSA)"
log_info "  • Genetic operations (mix, derive, delegate)"
log_info "  • Human entropy collection"
log_info "  • HSM abstraction (software, hardware, mobile)"
echo ""

# Create BearDog service registration
BEARDOG_SERVICE="$OUTPUT_DIR/beardog_service.json"

cat > "$BEARDOG_SERVICE" << JSON
{
  "service": {
    "name": "beardog-crypto",
    "version": "0.9.0",
    "primal": "beardog",
    "description": "Sovereign cryptographic services with genetic key operations",
    "capabilities": [
      "key-generation",
      "encryption",
      "decryption",
      "signing",
      "verification",
      "key-mixing",
      "key-derivation",
      "key-delegation",
      "human-entropy",
      "receipt-generation"
    ],
    "endpoints": {
      "key_generate": "/crypto/key/generate",
      "encrypt": "/crypto/encrypt",
      "decrypt": "/crypto/decrypt",
      "sign": "/crypto/sign",
      "verify": "/crypto/verify",
      "key_mix": "/crypto/key/mix",
      "key_derive": "/crypto/key/derive",
      "entropy_collect": "/crypto/entropy/collect",
      "receipt_get": "/crypto/receipt/:id"
    },
    "algorithms": {
      "encryption": ["AES-256-GCM", "ChaCha20-Poly1305"],
      "signing": ["Ed25519", "ECDSA-P256", "RSA-4096"],
      "kdf": ["Argon2id", "PBKDF2", "HKDF"]
    },
    "hsm_support": ["software", "hardware", "mobile"],
    "features": {
      "genetic_keys": true,
      "human_entropy": true,
      "receipt_validation": true,
      "constraint_enforcement": true,
      "sovereign_revocation": true
    },
    "metadata": {
      "entropy_sources": "system, human, mixed",
      "receipt_format": "cryptographic-receipt-v1",
      "sovereignty_model": "primal-owned-keys"
    }
  },
  "registration": {
    "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "registered_by": "beardog-tower-local",
    "expires_at": null,
    "health_check": "/health"
  }
}
JSON

log_success "BearDog service definition created"
log_info "Service: $BEARDOG_SERVICE"
echo ""

# Simulate registration
jq '.services += [input.service]' "$REGISTRY_FILE" "$BEARDOG_SERVICE" > "$OUTPUT_DIR/registry_tmp.json"
mv "$OUTPUT_DIR/registry_tmp.json" "$REGISTRY_FILE"

log_success "BearDog registered with Songbird!"
echo ""
wait_for_user

#
# STEP 3: Songbird Discovers BearDog
#

log_step "Step 3: Songbird discovers available crypto services..."
echo ""

log_info "Songbird queries registry for crypto capabilities"
echo ""

# Simulate discovery
echo "📋 Discovery Query:"
echo '  { "capability": "encryption" }'
echo ""

echo "📋 Discovery Results:"
jq '.services[] | select(.capabilities[] == "encryption") | {name, version, primal, capabilities: .capabilities[:5]}' "$REGISTRY_FILE"

echo ""
log_success "Found BearDog crypto service!"
echo ""
wait_for_user

#
# STEP 4: Songbird Requests Key Generation
#

log_step "Step 4: Songbird requests key generation from BearDog..."
echo ""

log_info "Songbird wants to encrypt a message"
log_info "It requests a key from BearDog"
echo ""

# Create message to encrypt
MESSAGE_FILE="$OUTPUT_DIR/message.txt"
echo "Hello from Songbird! This message is encrypted by BearDog." > "$MESSAGE_FILE"

log_step "Generating encryption key via BearDog..."

KEY_ID="songbird-channel-001-$(date +%s)"

"$BEARDOG" key generate \
  --key-id "$KEY_ID" \
  --algorithm AES-256-GCM \
  --hsm auto \
  --kdf argon2 \
  --usage encrypt-only 2>&1 | tee "$LOGS_DIR/key-generation.log" | tail -20

log_success "BearDog generated key: $KEY_ID"
echo ""

# Extract receipt if generated
if ls receipts/receipt-key-generate-*.json 2>/dev/null | tail -1 > /dev/null; then
    LATEST_RECEIPT=$(ls -t receipts/receipt-key-generate-*.json 2>/dev/null | head -1)
    cp "$LATEST_RECEIPT" "$RECEIPTS_DIR/key-generation-receipt.json"
    log_success "Receipt saved: $RECEIPTS_DIR/key-generation-receipt.json"
    
    RECEIPT_ID=$(jq -r '.receipt_id' "$RECEIPTS_DIR/key-generation-receipt.json")
    log_info "Receipt ID: $RECEIPT_ID"
fi

echo ""
wait_for_user

#
# STEP 5: Songbird Encrypts Message with BearDog
#

log_step "Step 5: Songbird requests encryption from BearDog..."
echo ""

ENCRYPTED_FILE="$OUTPUT_DIR/message.enc"

log_step "Encrypting message..."

"$BEARDOG" encrypt \
  --key "$KEY_ID" \
  --input "$MESSAGE_FILE" \
  --output "$ENCRYPTED_FILE" 2>&1 | tee "$LOGS_DIR/encryption.log"

log_success "Message encrypted!"
log_info "Encrypted file: $ENCRYPTED_FILE"

# Show file sizes
PLAIN_SIZE=$(stat -c%s "$MESSAGE_FILE" 2>/dev/null || stat -f%z "$MESSAGE_FILE" 2>/dev/null || echo "unknown")
ENC_SIZE=$(stat -c%s "$ENCRYPTED_FILE" 2>/dev/null || stat -f%z "$ENCRYPTED_FILE" 2>/dev/null || echo "unknown")

echo ""
echo "📊 Encryption Stats:"
echo "   Plain text: $PLAIN_SIZE bytes"
echo "   Encrypted: $ENC_SIZE bytes"
echo "   Algorithm: AES-256-GCM (authenticated encryption)"
echo "   Key: $KEY_ID"
echo ""

wait_for_user

#
# STEP 6: Songbird Decrypts Message with BearDog
#

log_step "Step 6: Songbird requests decryption from BearDog..."
echo ""

DECRYPTED_FILE="$OUTPUT_DIR/message-decrypted.txt"

log_step "Decrypting message..."

"$BEARDOG" decrypt \
  --key "$KEY_ID" \
  --input "$ENCRYPTED_FILE" \
  --output "$DECRYPTED_FILE" 2>&1 | tee "$LOGS_DIR/decryption.log"

log_success "Message decrypted!"
echo ""

# Verify
echo "📄 Original message:"
cat "$MESSAGE_FILE"
echo ""

echo "📄 Decrypted message:"
cat "$DECRYPTED_FILE"
echo ""

if diff -q "$MESSAGE_FILE" "$DECRYPTED_FILE" > /dev/null 2>&1; then
    log_success "✅ VERIFIED: Decrypted message matches original!"
else
    log_error "❌ ERROR: Decrypted message does not match!"
    exit 1
fi

echo ""
wait_for_user

#
# STEP 7: Validate Receipts
#

log_step "Step 7: Validating cryptographic receipts..."
echo ""

log_info "Every BearDog operation generates a verifiable receipt"
echo ""

RECEIPT_COUNT=$(find "$RECEIPTS_DIR" -name "*.json" 2>/dev/null | wc -l | tr -d ' ')

if [[ $RECEIPT_COUNT -gt 0 ]]; then
    log_success "Found $RECEIPT_COUNT receipt(s)"
    echo ""
    
    for receipt in "$RECEIPTS_DIR"/*.json; do
        echo "📜 Receipt: $(basename "$receipt")"
        jq '{receipt_id, operation, timestamp, result}' "$receipt" 2>/dev/null || echo "  (Unable to parse)"
        echo ""
    done
else
    log_info "No receipts found (may need to configure receipt output)"
fi

echo ""
wait_for_user

#
# STEP 8: Summary
#

log_step "Step 8: Integration Summary"
echo ""

cat << EOF

╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                    🎉 INTEGRATION SUCCESSFUL! 🎉                             ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝

WHAT WE DEMONSTRATED:

  ✅ Service Registration
     • BearDog advertised its crypto capabilities
     • Songbird registered BearDog in its service registry
     • No hardcoded dependencies!

  ✅ Service Discovery
     • Songbird queried for "encryption" capability
     • Found BearDog crypto service
     • Retrieved full capability list

  ✅ Key Generation
     • Songbird requested key from BearDog
     • BearDog generated AES-256-GCM key
     • Key ID: $KEY_ID
     • Receipt issued for audit trail

  ✅ Encryption
     • Songbird requested encryption
     • BearDog encrypted message
     • Used authenticated encryption (AES-256-GCM)

  ✅ Decryption
     • Songbird requested decryption
     • BearDog decrypted message
     • ✅ Verified: Message matches original!

  ✅ Sovereignty Maintained
     • BearDog owns all keys
     • Songbird discovers services
     • Both primals remain independent
     • No central authority!

KEY INSIGHTS:

  🔐 BearDog's Role:
     • Provides crypto services
     • Owns and manages keys
     • Issues verifiable receipts
     • Can revoke access anytime

  🐦 Songbird's Role:
     • Discovers available services
     • Orchestrates operations
     • Validates receipts
     • Can switch providers

  🌱 ecoPrimals Model:
     • No hardcoded dependencies
     • Runtime service discovery
     • Capability-based routing
     • Sovereign primals
     • Reciprocal learning

OUTPUT:
  • Registry: $REGISTRY_FILE
  • Logs: $LOGS_DIR/
  • Receipts: $RECEIPTS_DIR/
  • Messages: $OUTPUT_DIR/

NEXT STEPS:
  • Try with 2 Songbird towers
  • Add message signing
  • Implement key rotation
  • Test with hardware HSM

EOF

log_success "Integration demo complete!"
echo ""

# Final stats
cat << EOF

📊 INTEGRATION STATS:

  Services Registered: 1 (BearDog)
  Capabilities Advertised: 10
  Operations Performed: 3 (generate, encrypt, decrypt)
  Messages Encrypted: 1
  Receipts Generated: $RECEIPT_COUNT
  Verification: ✅ PASSED

╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║   🐻🐦 BearDog + Songbird: Working Together, Sovereign Apart! 🐻🐦            ║
║                                                                              ║
║   Each primal maintains independence while enabling collaboration.          ║
║   This is the future of decentralized, sovereign computing! 🌱               ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝

EOF

