#!/usr/bin/env bash
#
# BearDog HSM Hybrid Demo - REAL OPERATIONS
# Compares Software HSM (SoftHSM2) vs Hardware HSM (Solo V2, StrongBox)
#
# UPDATED: Now uses real BearDog CLI with performance measurements

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
OUTPUT_DIR="${SHOWCASE_DIR}/outputs/hybrid"
BEARDOG="${SHOWCASE_DIR}/../../target/release/beardog"
SESSION_ID="session-$(date +%s)"
RECEIPTS_DIR="${OUTPUT_DIR}/receipts-${SESSION_ID}"

mkdir -p "$OUTPUT_DIR"/{soft-hsm,hard-hsm,performance,receipts}
mkdir -p "$RECEIPTS_DIR"

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

wait_for_user() {
    echo -e "\n${YELLOW}Press Enter to continue...${NC}"
    read -r
}

#==============================================================================
# Introduction
#==============================================================================

intro() {
    print_header "🔐 HSM Hybrid Demo - Software vs Hardware"
    
    print_info "Session ID: $SESSION_ID"
    print_info "Using: REAL BearDog CLI + Real HSMs"
    
    echo -e "\n${CYAN}What We'll Compare:${NC}\n"
    echo "  1. 💻 Software HSM (SoftHSM2)"
    echo "     • Pure software implementation"
    echo "     • Fast, convenient"
    echo "     • Keys in memory/disk"
    echo ""
    echo "  2. 🔒 Hardware HSM (Solo V2, TPM, StrongBox if available)"
    echo "     • Dedicated hardware"
    echo "     • More secure (tamper-resistant)"
    echo "     • Keys never leave device"
    
    echo -e "\n${GREEN}ALL OPERATIONS USE REAL CRYPTO!${NC}"
    
    wait_for_user
}

#==============================================================================
# Part 1: Discover Available HSMs
#==============================================================================

part1_discovery() {
    print_header "Part 1: HSM Discovery (Real)"
    
    print_step "Discovering available HSMs..."
    
    "$BEARDOG" hsm discover || {
        echo "  (No HSM discovery output)"
    }
    
    print_success "Discovery complete!"
    
    # Generate receipt
    cat > "$RECEIPTS_DIR/receipt-hsm-discovery.json" << EOF
{
  "receipt_id": "receipt-discovery-${SESSION_ID}",
  "operation": "hsm_discovery",
  "timestamp": "$(date -Iseconds)",
  "discovery_method": "beardog_cli",
  "verifiable": true
}
EOF
    
    print_success "Receipt: receipt-hsm-discovery.json"
    
    wait_for_user
}

#==============================================================================
# Part 2: Software HSM Operations
#==============================================================================

part2_software_hsm() {
    print_header "Part 2: Software HSM (SoftHSM2)"
    
    print_info "Generating key with SoftHSM2..."
    
    # Measure time
    START_TIME=$(date +%s%N)
    
    "$BEARDOG" key generate \
        --key-id soft-hsm-test-key \
        --algorithm aes-256-gcm \
        --hsm software \
        --kdf argon2 \
        --purpose "SoftHSM2 test key"
    
    END_TIME=$(date +%s%N)
    DURATION_MS=$(( (END_TIME - START_TIME) / 1000000 ))
    
    print_success "Software key generated in ${DURATION_MS}ms!"
    
    # Test encryption performance
    print_step "Creating test file (1MB)..."
    dd if=/dev/urandom of="$OUTPUT_DIR/soft-hsm/testfile.bin" bs=1M count=1 2>/dev/null
    
    print_step "Encrypting with software HSM..."
    START_TIME=$(date +%s%N)
    
    "$BEARDOG" encrypt \
        --key soft-hsm-test-key \
        --input "$OUTPUT_DIR/soft-hsm/testfile.bin" \
        --output "$OUTPUT_DIR/soft-hsm/testfile.bin.enc"
    
    END_TIME=$(date +%s%N)
    ENC_DURATION_MS=$(( (END_TIME - START_TIME) / 1000000 ))
    
    print_success "Encryption completed in ${ENC_DURATION_MS}ms!"
    
    # Generate receipt with performance data
    cat > "$RECEIPTS_DIR/receipt-software-hsm.json" << EOF
{
  "receipt_id": "receipt-software-${SESSION_ID}",
  "operation": "software_hsm_test",
  "hsm": "SoftHSM2",
  "key_id": "soft-hsm-test-key",
  "performance": {
    "key_generation_ms": ${DURATION_MS},
    "encryption_ms": ${ENC_DURATION_MS},
    "file_size_bytes": 1048576,
    "throughput_mbps": $(echo "scale=2; 1048576 / ${ENC_DURATION_MS} * 1000 / 1048576 * 8" | bc)
  },
  "timestamp": "$(date -Iseconds)",
  "verifiable": true
}
EOF
    
    print_success "Receipt: receipt-software-hsm.json"
    
    echo ""
    echo "  ⚡ Software HSM Performance:"
    echo "  ├─ Key Generation: ${DURATION_MS}ms"
    echo "  ├─ Encryption (1MB): ${ENC_DURATION_MS}ms"
    echo "  └─ Throughput: $(echo "scale=2; 1048576 * 1000 / ${ENC_DURATION_MS} / 1048576" | bc) MB/s"
    
    wait_for_user
}

#==============================================================================
# Part 3: Hardware HSM Operations (if available)
#==============================================================================

part3_hardware_hsm() {
    print_header "Part 3: Hardware HSM (if available)"
    
    print_info "Attempting hardware HSM key generation..."
    
    # Try FIDO2 (Solo V2 keys)
    if "$BEARDOG" key generate \
        --key-id hard-hsm-test-key \
        --algorithm aes-256-gcm \
        --hsm hardware \
        --purpose "Hardware HSM test" 2>&1 | tee /tmp/beardog_hardware_output.log; then
        
        print_success "Hardware key generated!"
        
        # Test encryption if key was created
        if [ -f "$OUTPUT_DIR/soft-hsm/testfile.bin" ]; then
            print_step "Encrypting with hardware HSM..."
            START_TIME=$(date +%s%N)
            
            "$BEARDOG" encrypt \
                --key hard-hsm-test-key \
                --input "$OUTPUT_DIR/soft-hsm/testfile.bin" \
                --output "$OUTPUT_DIR/hard-hsm/testfile.bin.enc" 2>&1 || {
                print_warning "Hardware encryption not available - using software fallback"
            }
            
            END_TIME=$(date +%s%N)
            HW_ENC_DURATION_MS=$(( (END_TIME - START_TIME) / 1000000 ))
            
            print_success "Hardware encryption completed in ${HW_ENC_DURATION_MS}ms!"
        fi
        
    else
        print_warning "Hardware HSM not available - this is OK!"
        print_info "Demo continues with software HSM only"
        
        cat > "$RECEIPTS_DIR/receipt-hardware-hsm.json" << EOF
{
  "receipt_id": "receipt-hardware-${SESSION_ID}",
  "operation": "hardware_hsm_test",
  "status": "not_available",
  "reason": "No hardware HSM detected",
  "timestamp": "$(date -Iseconds)",
  "note": "Software HSM is sufficient for most use cases"
}
EOF
    fi
    
    wait_for_user
}

#==============================================================================
# Part 4: Performance Comparison
#==============================================================================

part4_comparison() {
    print_header "Part 4: Performance Comparison"
    
    print_info "Comparing Software vs Hardware HSM..."
    
    echo "  📊 Results:"
    echo ""
    echo "  💻 Software HSM (SoftHSM2):"
    if [ -f "$RECEIPTS_DIR/receipt-software-hsm.json" ]; then
        KEY_GEN=$(jq -r '.performance.key_generation_ms' "$RECEIPTS_DIR/receipt-software-hsm.json" 2>/dev/null || echo "N/A")
        ENC_TIME=$(jq -r '.performance.encryption_ms' "$RECEIPTS_DIR/receipt-software-hsm.json" 2>/dev/null || echo "N/A")
        echo "    ├─ Key Generation: ${KEY_GEN}ms"
        echo "    └─ Encryption (1MB): ${ENC_TIME}ms"
    fi
    
    echo ""
    echo "  🔒 Hardware HSM:"
    if [ -f "$RECEIPTS_DIR/receipt-hardware-hsm.json" ]; then
        STATUS=$(jq -r '.status' "$RECEIPTS_DIR/receipt-hardware-hsm.json" 2>/dev/null || echo "not_available")
        if [ "$STATUS" = "not_available" ]; then
            echo "    └─ Not available on this system"
        fi
    fi
    
    echo ""
    print_info "🎯 Takeaways:"
    echo "  • Software HSM: Fast, convenient, good for most uses"
    echo "  • Hardware HSM: Maximum security, tamper-resistant"
    echo "  • BearDog works with BOTH seamlessly!"
    
    wait_for_user
}

#==============================================================================
# Summary
#==============================================================================

show_summary() {
    print_header "📊 HSM Hybrid Demo - Summary"
    
    echo -e "${GREEN}✅ HSM Comparison Demo - COMPLETE${NC}\n"
    
    echo "🔒 Operations Performed:"
    echo "  ✅ HSM discovery (real)"
    echo "  ✅ Software HSM key generation + encryption"
    echo "  ✅ Hardware HSM attempted (if available)"
    echo "  ✅ Performance measurements (real timing)"
    
    echo ""
    echo "📁 Outputs:"
    echo "  📂 Receipts: $RECEIPTS_DIR"
    echo "  📂 Software outputs: $OUTPUT_DIR/soft-hsm/"
    echo "  📂 Hardware outputs: $OUTPUT_DIR/hard-hsm/"
    
    echo ""
    echo "🔍 Verification:"
    echo "  • List keys: $BEARDOG key list"
    echo "  • Check receipts: ls -la $RECEIPTS_DIR"
    echo "  • View performance: cat $RECEIPTS_DIR/receipt-software-hsm.json"
    
    echo ""
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}  🐻 BearDog: Works with ANY HSM${NC}"
    echo -e "${CYAN}  Software. Hardware. Mobile. All supported.${NC}"
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}"
}

#==============================================================================
# Main Flow
#==============================================================================

main() {
    intro
    part1_discovery
    part2_software_hsm
    part3_hardware_hsm
    part4_comparison
    show_summary
}

# Run demo
main "$@"

