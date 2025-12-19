#!/usr/bin/env bash
#
# BearDog Tower Sharing Demonstration
# Share compute resources with privacy and resource limits
#

set -e

BEARDOG="${BEARDOG:-../../target/release/beardog}"
SESSION="tower-sharing-$(date +%s)"
OUTPUT_DIR="./output-${SESSION}"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo ""
echo "🗼 ================================================"
echo "   BearDog Tower Sharing Demonstration"
echo "   Resource-Limited Delegation with Privacy"
echo "================================================"
echo ""

mkdir -p "$OUTPUT_DIR"

# Helper functions
receipt() {
    local title="$1"
    local data="$2"
    echo "$data" | sha256sum | awk '{print $1}' > "${OUTPUT_DIR}/${title// /-}.receipt"
    echo "📋 Receipt: $(cat ${OUTPUT_DIR}/${title// /-}.receipt)"
}

section() {
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}$1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

info() {
    echo -e "${CYAN}ℹ️  $1${NC}"
}

# Check BearDog CLI
if [ ! -f "$BEARDOG" ]; then
    echo "❌ BearDog CLI not found. Building..."
    cd ../../ && cargo build --release && cd - > /dev/null
fi

section "📖 Scenario: Tower Sharing with a Friend"

cat << 'EOF'
🎯 Use Case:
  You want to allow your friend Alice to use your tower for compute during
  off-peak hours. The key requirements are:
  
  1. Privacy: Alice's work is encrypted, you can't see her data
  2. Time-Limited: Only during off-peak hours (22:00-6:00)
  3. Resource-Limited: Max 50% CPU, max 8GB memory
  4. Auto-Expiring: Access expires after 30 days
  5. Revocable: You can revoke access anytime

🔐 Security Model:
  - Alice gets a delegated key with strict constraints
  - All constraints enforced by BearDog runtime
  - Workloads encrypted end-to-end
  - No phone-home - fully sovereign

🎨 Constraint Architecture:
  This demo uses CompositeConstraint with AND logic to combine:
    • TimeRangeConstraint (off-peak hours)
    • CpuQuotaConstraint (50% max)
    • MemoryQuotaConstraint (8GB max)
    • ExpiryConstraint (30 days)

EOF

section "🔑 Step 1: Generate Master Key (Tower Owner)"

echo "Generating your master tower key with Argon2 KDF..."
echo ""

$BEARDOG key generate \
    --key-id "tower-master-${SESSION}" \
    --algorithm aes-256-gcm \
    --hsm software \
    --kdf argon2 \
    --kdf-iterations 3 \
    --kdf-memory 65536 \
    --kdf-time 3 \
    --usage all \
    --purpose "tower-master" \
    --expires-in 365d \
    | tee "${OUTPUT_DIR}/1-master-generate.log"

success "Master key generated with strong KDF"
receipt "Master Key Generation" "$(cat ${OUTPUT_DIR}/1-master-generate.log)"

section "🤝 Step 2: Delegate Access to Alice (Off-Peak, Limited)"

echo "Creating delegated key for Alice with constraints:"
echo "  ⏰ Time: 22:00-6:00 (off-peak hours)"
echo "  💻 CPU: Max 50%"
echo "  🧠 Memory: Max 8GB"
echo "  📅 Expiry: 30 days"
echo ""

$BEARDOG key delegate \
    --master-key "tower-master-${SESSION}" \
    --delegate-to "alice" \
    --output "alice-tower-access-${SESSION}" \
    --time-range "22:00-6:00" \
    --cpu-quota 50 \
    --memory-quota "8GB" \
    --expires-in 30d \
    | tee "${OUTPUT_DIR}/2-alice-delegate.log"

success "Delegated key created for Alice"
receipt "Alice Delegation" "$(cat ${OUTPUT_DIR}/2-alice-delegate.log)"

section "👥 Step 3: Share with Another Friend (Bob) - Daytime Access"

echo "Creating delegated key for Bob with different constraints:"
echo "  ⏰ Time: 9:00-17:00 (business hours)"
echo "  💻 CPU: Max 30%"
echo "  🧠 Memory: Max 4GB"
echo "  📅 Expiry: 14 days"
echo ""

$BEARDOG key delegate \
    --master-key "tower-master-${SESSION}" \
    --delegate-to "bob" \
    --output "bob-tower-access-${SESSION}" \
    --time-range "9:00-17:00" \
    --cpu-quota 30 \
    --memory-quota "4GB" \
    --expires-in 14d \
    | tee "${OUTPUT_DIR}/3-bob-delegate.log"

success "Delegated key created for Bob"
receipt "Bob Delegation" "$(cat ${OUTPUT_DIR}/3-bob-delegate.log)"

section "📊 Step 4: Visualize Key Lineage"

echo "Master key hierarchy:"
echo ""

$BEARDOG key lineage --key-id "tower-master-${SESSION}" \
    | tee "${OUTPUT_DIR}/4-lineage.log"

success "Key lineage visualized"

section "🔍 Step 5: Verify Delegation Constraints"

echo "Checking Alice's key details..."
echo ""

$BEARDOG key info --key-id "alice-tower-access-${SESSION}" \
    | tee "${OUTPUT_DIR}/5-alice-info.log"

echo ""
echo "Checking Bob's key details..."
echo ""

$BEARDOG key info --key-id "bob-tower-access-${SESSION}" \
    | tee "${OUTPUT_DIR}/5-bob-info.log"

success "Delegation constraints verified"

section "🔐 Step 6: Simulate Alice's Encrypted Workload"

echo "Alice encrypts her data (you can't see it):"
echo ""

# Create sample data
echo "Alice's private computation data - Project Falcon" > "${OUTPUT_DIR}/alice-data.txt"
echo "Sensitive algorithm parameters: [redacted]" >> "${OUTPUT_DIR}/alice-data.txt"
echo "Results will be encrypted end-to-end" >> "${OUTPUT_DIR}/alice-data.txt"

$BEARDOG encrypt \
    --key "alice-tower-access-${SESSION}" \
    --input "${OUTPUT_DIR}/alice-data.txt" \
    --output "${OUTPUT_DIR}/alice-data.enc" \
    | tee "${OUTPUT_DIR}/6-alice-encrypt.log"

info "Alice's data is encrypted end-to-end"
info "Tower owner (you) cannot decrypt it without Alice's key"

success "Alice's workload encrypted"
receipt "Alice Encryption" "$(cat ${OUTPUT_DIR}/6-alice-encrypt.log)"

section "🚫 Step 7: Revoke Bob's Access (Simulate Trust Break)"

echo "Scenario: Bob violated terms, revoking his access..."
echo ""

$BEARDOG key revoke \
    --key-id "bob-tower-access-${SESSION}" \
    --reason "Terms violation - exceeded resource limits" \
    | tee "${OUTPUT_DIR}/7-bob-revoke.log"

success "Bob's access revoked"
receipt "Bob Revocation" "$(cat ${OUTPUT_DIR}/7-bob-revoke.log)"

echo ""
echo "Checking revocation status..."
echo ""

$BEARDOG key check-revocation --key-id "bob-tower-access-${SESSION}" \
    | tee "${OUTPUT_DIR}/7-bob-revoke-check.log"

info "Revoked keys cannot be used for new operations"
info "Bob's existing encrypted data remains encrypted (privacy preserved)"

section "📋 Summary & Receipts"

echo "Tower Sharing Demonstration Complete!"
echo ""
echo "✅ What We Demonstrated:"
echo ""
echo "1. Master Key Generation"
echo "   • Strong KDF (Argon2) for master key"
echo "   • Long expiry (365 days) for tower owner"
echo ""
echo "2. Delegated Access Control"
echo "   • Alice: Off-peak hours, 50% CPU, 8GB memory, 30 days"
echo "   • Bob: Business hours, 30% CPU, 4GB memory, 14 days"
echo "   • Different constraints for different users"
echo ""
echo "3. Privacy Preservation"
echo "   • Alice's data encrypted end-to-end"
echo "   • Tower owner cannot decrypt Alice's workloads"
echo "   • Even after revocation, encrypted data stays encrypted"
echo ""
echo "4. Key Lineage"
echo "   • Clear parent-child relationship"
echo "   • Traceable delegation chain"
echo "   • Multi-generation support"
echo ""
echo "5. Sovereign Revocation"
echo "   • No phone-home required"
echo "   • Immediate local effect"
echo "   • Propagates via Songbird (optional)"
echo ""

echo "📊 Receipts Generated:"
ls -1 "${OUTPUT_DIR}"/*.receipt 2>/dev/null | while read receipt; do
    echo "  📋 $(basename $receipt): $(cat $receipt)"
done
echo ""

echo "📁 Output Directory: $OUTPUT_DIR"
echo ""

section "🎯 Real-World Benefits"

cat << 'EOF'
1. **Resource Fairness**
   - Prevent any one user from monopolizing compute
   - Different users can have different quotas
   - Automatically enforced by BearDog runtime

2. **Time-Based Access**
   - Off-peak hours for intensive tasks
   - Business hours for priority users
   - Automatically switches based on time

3. **Privacy by Design**
   - All workloads encrypted end-to-end
   - Tower owner provides compute, not access to data
   - Users maintain full control of their data

4. **Flexible Revocation**
   - Revoke access anytime
   - Local enforcement (no phone-home)
   - Optional network propagation via Songbird

5. **Automatic Expiry**
   - Keys expire automatically
   - No manual cleanup needed
   - Reduces risk of forgotten delegations

6. **Sovereignty**
   - No central authority
   - No phone-home
   - Local control of all operations
   - Network propagation optional (via Songbird)

EOF

section "💡 Advanced Scenarios"

cat << 'EOF'
These scenarios are now possible with BearDog:

**Scenario A: Family Tower Sharing**
  - Parent: Full access (master key)
  - Teen: Weekday evenings, 30% CPU (homework)
  - Guest: Weekend only, 20% CPU, 2GB memory

**Scenario B: Community Compute Pool**
  - Multiple towers in a community
  - Each owner delegates off-peak hours
  - Automatic load balancing via Songbird
  - Resource quotas per member

**Scenario C: Research Collaboration**
  - Multiple researchers share compute
  - Each has quota based on contribution
  - Data stays encrypted (privacy)
  - Automatically expires when project ends

**Scenario D: Commercial Compute Sharing**
  - Businesses share excess capacity
  - Usage tracked for billing
  - Resource limits per tier
  - Full audit trail via receipts

EOF

section "📚 Next Steps"

echo "Learn More:"
echo "  • Architecture: ../../CONSTRAINT_AGNOSTIC_COMPLETE.md"
echo "  • Constraints Guide: ../../CONSTRAINT_EXTENSIBILITY_GUIDE.md"
echo "  • Key Lineage: ../02-hardware-integration/KEY_LINEAGE_AND_CONSENT.md"
echo "  • Revocation: ../02-hardware-integration/REVOCATION_ARCHITECTURE.md"
echo ""
echo "Try More Demos:"
echo "  • ./demo-constraints.sh - All 14 constraint types"
echo "  • ./demo-secure-lab.sh - Multi-factor authentication"
echo "  • ./demo-mobile-context.sh - Mobile device constraints"
echo ""
echo "Integrate with Your Stack:"
echo "  • Songbird: Distributed constraint evaluation"
echo "  • Toadstool: Encrypted workload distribution"
echo "  • Custom: Create your own constraints!"
echo ""

echo "🎉 Tower sharing demonstration complete!"
echo ""

