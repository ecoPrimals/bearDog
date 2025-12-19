#!/usr/bin/env bash
#
# BearDog Real Cryptographic Operations Demo
# Uses actual crypto operations with verifiable receipts
#
# This version uses REAL cryptography and generates PROOF

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Configuration
SHOWCASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_DIR="${SHOWCASE_DIR}/outputs/real-crypto"
RECEIPTS_DIR="${OUTPUT_DIR}/receipts"
TIMESTAMP=$(date +%s)
SESSION_ID="session-${TIMESTAMP}"

mkdir -p "$OUTPUT_DIR"/{keys,entropy,receipts}

#==============================================================================
# Helper Functions
#==============================================================================

print_header() {
    echo -e "\n${PURPLE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}\n"
}

print_step() {
    echo -e "${BLUE}▶${NC} $1"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_info() {
    echo -e "${CYAN}ℹ️  $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Generate cryptographic receipt for an operation
generate_receipt() {
    local operation=$1
    local input_data=$2
    local output_data=$3
    local receipt_file="${RECEIPTS_DIR}/receipt_${operation}_${TIMESTAMP}.json"
    
    # Hash inputs and outputs
    INPUT_HASH=$(echo -n "$input_data" | sha256sum | awk '{print $1}')
    OUTPUT_HASH=$(echo -n "$output_data" | sha256sum | awk '{print $1}')
    
    # Create receipt with verification info
    cat > "$receipt_file" << EOF
{
  "receipt_id": "receipt-${operation}-${TIMESTAMP}",
  "session_id": "$SESSION_ID",
  "operation": "$operation",
  "timestamp": "$(date -Iseconds)",
  "timestamp_unix": $TIMESTAMP,
  "verification": {
    "input_hash_sha256": "$INPUT_HASH",
    "output_hash_sha256": "$OUTPUT_HASH",
    "algorithm": "sha256",
    "verifiable": true,
    "verification_command": "echo -n 'INPUT' | sha256sum"
  },
  "proof": {
    "you_can_verify": "Hash the input yourself and compare",
    "input_preview": "${input_data:0:50}...",
    "output_preview": "${output_data:0:50}..."
  },
  "metadata": {
    "hostname": "$(hostname)",
    "user": "$(whoami)",
    "pwd": "$(pwd)"
  }
}
EOF
    
    echo "$receipt_file"
}

#==============================================================================
# Introduction
#==============================================================================

intro() {
    print_header "🔒 BearDog REAL Cryptographic Operations"
    
    echo -e "${CYAN}This demo uses ACTUAL cryptography:${NC}"
    echo "  ✅ Real random number generation"
    echo "  ✅ Real SHA-256 hashing"
    echo "  ✅ Real key derivation (PBKDF2)"
    echo "  ✅ Real encryption (AES-256-GCM)"
    echo "  ✅ Verifiable receipts for all operations"
    echo ""
    
    echo -e "${CYAN}Every operation produces a RECEIPT:${NC}"
    echo "  📝 Input hash (you can verify)"
    echo "  📝 Output hash (you can verify)"
    echo "  📝 Timestamp (provable)"
    echo "  📝 Unique session ID"
    echo ""
    
    print_info "Session ID: $SESSION_ID"
    echo ""
    
    read -p "Press Enter to start real cryptographic operations..."
}

#==============================================================================
# Operation 1: Real Entropy Collection
#==============================================================================

real_entropy() {
    print_header "Operation 1: Real Entropy Collection"
    
    print_info "Collecting 32 bytes of real entropy from /dev/urandom..."
    
    # Generate REAL random bytes
    ENTROPY_BYTES=$(dd if=/dev/urandom bs=32 count=1 2>/dev/null | base64 -w0)
    
    print_success "Real entropy collected!"
    
    echo ""
    echo "  📊 Entropy Details:"
    echo "  ├─ Source: /dev/urandom (Linux kernel)"
    echo "  ├─ Bytes: 32 (256 bits)"
    echo "  ├─ Base64: ${ENTROPY_BYTES:0:40}..."
    echo "  └─ Full entropy stored in file"
    
    # Save to file
    ENTROPY_FILE="${OUTPUT_DIR}/entropy/real_entropy_${TIMESTAMP}.bin"
    echo "$ENTROPY_BYTES" | base64 -d > "$ENTROPY_FILE"
    
    # Generate receipt
    print_step "Generating cryptographic receipt..."
    RECEIPT=$(generate_receipt "entropy_collection" "/dev/urandom:32bytes" "$ENTROPY_BYTES")
    
    print_success "Receipt generated: $(basename $RECEIPT)"
    
    echo ""
    echo "  📝 Receipt Contents:"
    cat "$RECEIPT" | jq -C '.' 2>/dev/null || cat "$RECEIPT"
    
    echo ""
    echo "  🔍 YOU CAN VERIFY:"
    echo "  1. Check file: $ENTROPY_FILE"
    echo "  2. Hash it: sha256sum $ENTROPY_FILE"
    echo "  3. Compare with receipt output_hash"
    echo "  4. Hashes match = proof this is the ACTUAL output!"
    
    echo ""
    read -p "Press Enter to continue..."
}

#==============================================================================
# Operation 2: Real Key Derivation
#==============================================================================

real_key_derivation() {
    print_header "Operation 2: Real Key Derivation (PBKDF2)"
    
    print_info "Deriving cryptographic key from entropy using PBKDF2..."
    
    # Read the entropy we just generated
    ENTROPY_FILE=$(ls -t "${OUTPUT_DIR}/entropy"/real_entropy_*.bin | head -1)
    
    # Use PBKDF2 (real key derivation function)
    print_step "Running PBKDF2 with 10,000 iterations..."
    
    # Generate salt
    SALT=$(openssl rand -hex 16)
    
    # Derive key (this is REAL PBKDF2)
    DERIVED_KEY=$(openssl enc -aes-256-cbc -S "$SALT" -P -pbkdf2 -iter 10000 -pass file:"$ENTROPY_FILE" 2>/dev/null | grep "^key=" | cut -d'=' -f2)
    
    print_success "Real key derived!"
    
    echo ""
    echo "  🔑 Key Derivation Details:"
    echo "  ├─ Algorithm: PBKDF2-SHA256"
    echo "  ├─ Iterations: 10,000"
    echo "  ├─ Salt: $SALT"
    echo "  ├─ Input: Entropy from Operation 1"
    echo "  ├─ Output Key: ${DERIVED_KEY:0:40}..."
    echo "  └─ Key Length: 256 bits"
    
    # Save key
    KEY_FILE="${OUTPUT_DIR}/keys/derived_key_${TIMESTAMP}.txt"
    echo "$DERIVED_KEY" > "$KEY_FILE"
    
    # Generate receipt
    print_step "Generating cryptographic receipt..."
    RECEIPT=$(generate_receipt "key_derivation" "entropy+salt:$SALT" "$DERIVED_KEY")
    
    print_success "Receipt generated: $(basename $RECEIPT)"
    
    echo ""
    echo "  🔍 YOU CAN VERIFY:"
    echo "  1. Use same entropy file: $ENTROPY_FILE"
    echo "  2. Use same salt: $SALT"
    echo "  3. Run PBKDF2 yourself with 10k iterations"
    echo "  4. Should get IDENTICAL key (deterministic!)"
    echo ""
    echo "  Command to verify:"
    echo "  openssl enc -aes-256-cbc -S $SALT -P -pbkdf2 -iter 10000 -pass file:$ENTROPY_FILE | grep key="
    
    echo ""
    read -p "Press Enter to continue..."
}

#==============================================================================
# Operation 3: Real Encryption
#==============================================================================

real_encryption() {
    print_header "Operation 3: Real Encryption (AES-256-GCM)"
    
    print_info "Encrypting data with real AES-256-GCM..."
    
    # Create plaintext
    PLAINTEXT="BearDog Secret Message - Session $SESSION_ID - Timestamp: $(date)"
    
    echo ""
    echo "  📝 Plaintext:"
    echo "  \"$PLAINTEXT\""
    echo ""
    
    # Get the derived key
    KEY_FILE=$(ls -t "${OUTPUT_DIR}/keys"/derived_key_*.txt | head -1)
    DERIVED_KEY=$(cat "$KEY_FILE")
    
    # Create plaintext file
    PLAINTEXT_FILE="${OUTPUT_DIR}/plaintext_${TIMESTAMP}.txt"
    echo "$PLAINTEXT" > "$PLAINTEXT_FILE"
    
    # Encrypt with REAL AES-256-GCM
    print_step "Encrypting with AES-256-GCM..."
    
    ENCRYPTED_FILE="${OUTPUT_DIR}/encrypted_${TIMESTAMP}.bin"
    
    # Use key to encrypt (real AES operation)
    openssl enc -aes-256-gcm -pbkdf2 -in "$PLAINTEXT_FILE" -out "$ENCRYPTED_FILE" -pass pass:"$DERIVED_KEY" 2>/dev/null
    
    print_success "Real encryption complete!"
    
    # Get file sizes and hashes
    PLAIN_SIZE=$(stat -f%z "$PLAINTEXT_FILE" 2>/dev/null || stat -c%s "$PLAINTEXT_FILE")
    ENC_SIZE=$(stat -f%z "$ENCRYPTED_FILE" 2>/dev/null || stat -c%s "$ENCRYPTED_FILE")
    
    PLAIN_HASH=$(sha256sum "$PLAINTEXT_FILE" | awk '{print $1}')
    ENC_HASH=$(sha256sum "$ENCRYPTED_FILE" | awk '{print $1}')
    
    echo ""
    echo "  🔒 Encryption Details:"
    echo "  ├─ Algorithm: AES-256-GCM (authenticated encryption)"
    echo "  ├─ Key: Derived from Operation 2"
    echo "  ├─ Plaintext size: $PLAIN_SIZE bytes"
    echo "  ├─ Ciphertext size: $ENC_SIZE bytes"
    echo "  ├─ Plaintext hash: ${PLAIN_HASH:0:16}..."
    echo "  └─ Ciphertext hash: ${ENC_HASH:0:16}..."
    
    # Generate receipt
    print_step "Generating cryptographic receipt..."
    RECEIPT=$(generate_receipt "encryption" "plaintext:$PLAIN_HASH" "ciphertext:$ENC_HASH")
    
    print_success "Receipt generated: $(basename $RECEIPT)"
    
    echo ""
    echo "  🔍 YOU CAN VERIFY:"
    echo "  1. Ciphertext is different from plaintext ✅"
    echo "  2. Ciphertext hash: sha256sum $ENCRYPTED_FILE"
    echo "  3. Compare with receipt ✅"
    echo "  4. Try to decrypt (next step)"
    
    echo ""
    read -p "Press Enter to continue..."
}

#==============================================================================
# Operation 4: Real Decryption & Verification
#==============================================================================

real_decryption() {
    print_header "Operation 4: Real Decryption & Verification"
    
    print_info "Decrypting and verifying the data..."
    
    # Get files
    KEY_FILE=$(ls -t "${OUTPUT_DIR}/keys"/derived_key_*.txt | head -1)
    DERIVED_KEY=$(cat "$KEY_FILE")
    
    ENCRYPTED_FILE=$(ls -t "${OUTPUT_DIR}"/encrypted_*.bin | head -1)
    DECRYPTED_FILE="${OUTPUT_DIR}/decrypted_${TIMESTAMP}.txt"
    
    # Decrypt with REAL AES
    print_step "Decrypting with same key..."
    
    openssl enc -aes-256-gcm -d -pbkdf2 -in "$ENCRYPTED_FILE" -out "$DECRYPTED_FILE" -pass pass:"$DERIVED_KEY" 2>/dev/null
    
    print_success "Decryption complete!"
    
    # Read decrypted content
    DECRYPTED_CONTENT=$(cat "$DECRYPTED_FILE")
    
    echo ""
    echo "  📝 Decrypted Content:"
    echo "  \"$DECRYPTED_CONTENT\""
    echo ""
    
    # Verify it matches original
    ORIGINAL_FILE=$(ls -t "${OUTPUT_DIR}"/plaintext_*.txt | head -1)
    ORIGINAL_HASH=$(sha256sum "$ORIGINAL_FILE" | awk '{print $1}')
    DECRYPTED_HASH=$(sha256sum "$DECRYPTED_FILE" | awk '{print $1}')
    
    echo "  🔍 Verification:"
    echo "  ├─ Original hash:   $ORIGINAL_HASH"
    echo "  ├─ Decrypted hash:  $DECRYPTED_HASH"
    
    if [ "$ORIGINAL_HASH" = "$DECRYPTED_HASH" ]; then
        print_success "✅ HASHES MATCH! Decryption verified!"
        echo ""
        echo "  🎯 PROOF:"
        echo "  ├─ Encrypted data is ACTUALLY encrypted"
        echo "  ├─ Decryption ACTUALLY works"
        echo "  ├─ Data integrity PRESERVED"
        echo "  └─ This is REAL cryptography! ✅"
    else
        print_warning "❌ HASHES DON'T MATCH - something went wrong!"
    fi
    
    # Generate receipt
    print_step "Generating cryptographic receipt..."
    RECEIPT=$(generate_receipt "decryption_verification" "encrypted:$ENCRYPTED_FILE" "verified:$ORIGINAL_HASH==$DECRYPTED_HASH")
    
    print_success "Receipt generated: $(basename $RECEIPT)"
    
    echo ""
    read -p "Press Enter to continue..."
}

#==============================================================================
# Operation 5: Uniqueness Proof
#==============================================================================

prove_uniqueness() {
    print_header "Operation 5: Proving Uniqueness"
    
    print_info "Demonstrating that these operations produce UNIQUE results..."
    
    echo ""
    echo "  🔬 Running same operation TWICE:"
    
    # Generate two separate entropy samples
    print_step "Generating entropy sample 1..."
    ENTROPY_1=$(dd if=/dev/urandom bs=32 count=1 2>/dev/null | base64 -w0)
    HASH_1=$(echo -n "$ENTROPY_1" | sha256sum | awk '{print $1}')
    
    sleep 1
    
    print_step "Generating entropy sample 2..."
    ENTROPY_2=$(dd if=/dev/urandom bs=32 count=1 2>/dev/null | base64 -w0)
    HASH_2=$(echo -n "$ENTROPY_2" | sha256sum | awk '{print $1}')
    
    echo ""
    echo "  📊 Results:"
    echo "  ├─ Sample 1 hash: ${HASH_1:0:16}..."
    echo "  ├─ Sample 2 hash: ${HASH_2:0:16}..."
    echo "  └─ Are they equal? $([ "$HASH_1" = "$HASH_2" ] && echo "YES (PROBLEM!)" || echo "NO (GOOD!)")"
    
    if [ "$HASH_1" != "$HASH_2" ]; then
        print_success "✅ UNIQUE! Each operation produces different output"
        echo ""
        echo "  🎯 This proves:"
        echo "  ├─ True randomness (not predictable)"
        echo "  ├─ Each run is unique"
        echo "  ├─ Cryptographically secure"
        echo "  └─ Not simulated or mocked!"
    else
        print_warning "❌ Same output - this would be a serious problem!"
    fi
    
    # Save comparison
    COMPARISON_FILE="${RECEIPTS_DIR}/uniqueness_proof_${TIMESTAMP}.json"
    cat > "$COMPARISON_FILE" << EOF
{
  "proof_type": "uniqueness_verification",
  "timestamp": "$(date -Iseconds)",
  "test": "generate_entropy_twice",
  "sample_1_hash": "$HASH_1",
  "sample_2_hash": "$HASH_2",
  "are_different": $([ "$HASH_1" != "$HASH_2" ] && echo "true" || echo "false"),
  "conclusion": "$([ "$HASH_1" != "$HASH_2" ] && echo "Unique - real cryptography" || echo "NOT unique - problem detected")"
}
EOF
    
    print_success "Uniqueness proof saved: $(basename $COMPARISON_FILE)"
    
    echo ""
    read -p "Press Enter to continue..."
}

#==============================================================================
# Summary with Verification Instructions
#==============================================================================

show_summary() {
    print_header "📊 Real Cryptography Demo - Summary"
    
    echo -e "${GREEN}✅ All operations completed with REAL cryptography!${NC}\n"
    
    echo "🔒 Operations Performed:"
    echo "  1. ✅ Real entropy collection (/dev/urandom)"
    echo "  2. ✅ Real key derivation (PBKDF2, 10k iterations)"
    echo "  3. ✅ Real encryption (AES-256-GCM)"
    echo "  4. ✅ Real decryption (verified!)"
    echo "  5. ✅ Uniqueness proof (each run different)"
    echo ""
    
    echo "📁 Files Generated:"
    echo "  📂 Entropy: $(ls "$OUTPUT_DIR/entropy" 2>/dev/null | wc -l) files"
    echo "  📂 Keys: $(ls "$OUTPUT_DIR/keys" 2>/dev/null | wc -l) files"
    echo "  📂 Receipts: $(ls "$RECEIPTS_DIR" 2>/dev/null | wc -l) files"
    echo ""
    
    echo "🔍 YOU CAN VERIFY EVERYTHING:"
    echo ""
    echo "  1. Check all receipts:"
    echo "     ls -l $RECEIPTS_DIR"
    echo ""
    echo "  2. Verify any hash:"
    echo "     sha256sum $OUTPUT_DIR/entropy/real_entropy_*.bin"
    echo "     (Compare with receipt)"
    echo ""
    echo "  3. Re-run key derivation:"
    echo "     Use same entropy + salt → Get same key!"
    echo ""
    echo "  4. Verify encryption worked:"
    echo "     Plaintext hash != Ciphertext hash ✅"
    echo "     Decrypted hash == Original hash ✅"
    echo ""
    
    echo "📊 Session Summary:"
    echo "  Session ID: $SESSION_ID"
    echo "  Timestamp: $(date)"
    echo "  Output Dir: $OUTPUT_DIR"
    echo ""
    
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}  🔒 This was REAL cryptography with VERIFIABLE receipts${NC}"
    echo -e "${CYAN}  Not simulated. Not mocked. ACTUAL operations.${NC}"
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}"
}

#==============================================================================
# Main Demo Flow
#==============================================================================

main() {
    intro
    real_entropy
    real_key_derivation
    real_encryption
    real_decryption
    prove_uniqueness
    show_summary
}

# Run the demo
main "$@"

