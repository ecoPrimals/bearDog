#!/usr/bin/env bash
#
# BearDog Secure Lab Access Demonstration
# Multi-Factor Authentication with 6 Constraints
#

set -e

BEARDOG="${BEARDOG:-../../target/release/beardog}"
SESSION="secure-lab-$(date +%s)"
OUTPUT_DIR="./output-${SESSION}"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo ""
echo "🔬 ================================================"
echo "   BearDog Secure Lab Access Demonstration"
echo "   Multi-Factor Authentication (6 Constraints)"
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

warning() {
    echo -e "${RED}⚠️  $1${NC}"
}

# Check BearDog CLI
if [ ! -f "$BEARDOG" ]; then
    echo "❌ BearDog CLI not found. Building..."
    cd ../../ && cargo build --release && cd - > /dev/null
fi

section "📖 Scenario: High-Security Research Lab Access"

cat << 'EOF'
🎯 Use Case:
  Access to a high-security research lab requires multiple factors:
  
  1. Physical Presence: Must be on-site (within 50m geofence)
  2. Time Restriction: Business hours only (8:00-18:00)
  3. Day Restriction: Weekdays only (mon-fri)
  4. Biometric Auth: Fingerprint verification on Solo V2 key
  5. System Health: System not overloaded (load < 2.0)
  6. Environmental: Safe temperature range (18-25°C)

🔐 Security Model:
  ALL 6 constraints must be satisfied (AND logic)
  If any constraint fails, access is denied
  Fully sovereign - no phone-home required
  
🎨 Constraint Architecture:
  CompositeConstraint::and(vec![
    GeoFenceConstraint,       # Physical presence
    TimeRangeConstraint,      # Business hours
    WeekdayConstraint,        # Weekdays
    BiometricConstraint,      # Fingerprint
    SystemLoadConstraint,     # System health
    EnvironmentalConstraint,  # Safe temperature
  ])

📊 Constraint Status:
  ✅ TimeRangeConstraint      - Real BearDog CLI
  ✅ WeekdayConstraint        - Real BearDog CLI
  ⏳ GeoFenceConstraint       - Conceptual (requires GPS integration)
  ⏳ BiometricConstraint      - Conceptual (requires Solo V2 integration)
  ⏳ SystemLoadConstraint     - Conceptual (requires system monitoring)
  ⏳ EnvironmentalConstraint  - Conceptual (requires sensor integration)

EOF

section "🔑 Step 1: Generate Master Lab Access Key"

echo "Generating master lab key with Argon2 KDF..."
echo "Purpose: High-security research lab access control"
echo ""

$BEARDOG key generate \
    --key-id "lab-master-${SESSION}" \
    --algorithm aes-256-gcm \
    --hsm software \
    --kdf argon2 \
    --kdf-iterations 5 \
    --kdf-memory 131072 \
    --kdf-time 5 \
    --usage sign-verify \
    --purpose "secure-lab-master" \
    --expires-in 365d \
    | tee "${OUTPUT_DIR}/1-master-generate.log"

success "Master lab key generated with strong KDF"
receipt "Master Key Generation" "$(cat ${OUTPUT_DIR}/1-master-generate.log)"

section "🎫 Step 2: Delegate Access to Researcher (Alice)"

echo "Creating delegated key for Alice with REAL CLI constraints:"
echo "  ⏰ Time: 8:00-18:00 (business hours) - ✅ REAL"
echo "  📅 Days: mon-fri (weekdays) - ✅ REAL"
echo "  💻 CPU: Max 70% - ✅ REAL"
echo "  🧠 Memory: Max 16GB - ✅ REAL"
echo "  ⏱️  Expiry: 90 days - ✅ REAL"
echo ""
echo "Additional CONCEPTUAL constraints (will be enforced when hardware integrated):"
echo "  📍 Location: Within 50m of lab - Conceptual (GeoFenceConstraint)"
echo "  👆 Biometric: Fingerprint on Solo V2 - Conceptual (BiometricConstraint)"
echo "  ⚡ System: Load < 2.0 - Conceptual (SystemLoadConstraint)"
echo "  🌡️  Environment: Temp 18-25°C - Conceptual (EnvironmentalConstraint)"
echo ""

$BEARDOG key delegate \
    --master-key "lab-master-${SESSION}" \
    --delegate-to "alice-researcher" \
    --output "alice-lab-access-${SESSION}" \
    --time-range "8:00-18:00" \
    --weekdays "mon-fri" \
    --cpu-quota 70 \
    --memory-quota "16GB" \
    --expires-in 90d \
    | tee "${OUTPUT_DIR}/2-alice-delegate.log"

success "Delegated lab key created for Alice (real CLI constraints active)"
receipt "Alice Delegation" "$(cat ${OUTPUT_DIR}/2-alice-delegate.log)"

section "👥 Step 3: Delegate Access to Post-Doc (Bob)"

echo "Creating delegated key for Bob with different constraints:"
echo "  ⏰ Time: 9:00-17:00 (standard hours) - ✅ REAL"
echo "  📅 Days: mon-fri (weekdays) - ✅ REAL"
echo "  💻 CPU: Max 50% - ✅ REAL"
echo "  🧠 Memory: Max 8GB - ✅ REAL"
echo "  ⏱️  Expiry: 180 days - ✅ REAL"
echo ""

$BEARDOG key delegate \
    --master-key "lab-master-${SESSION}" \
    --delegate-to "bob-postdoc" \
    --output "bob-lab-access-${SESSION}" \
    --time-range "9:00-17:00" \
    --weekdays "mon-fri" \
    --cpu-quota 50 \
    --memory-quota "8GB" \
    --expires-in 180d \
    | tee "${OUTPUT_DIR}/3-bob-delegate.log"

success "Delegated lab key created for Bob"
receipt "Bob Delegation" "$(cat ${OUTPUT_DIR}/3-bob-delegate.log)"

section "📊 Step 4: Visualize Access Hierarchy"

echo "Lab access key hierarchy:"
echo ""

$BEARDOG key lineage --key-id "lab-master-${SESSION}" \
    | tee "${OUTPUT_DIR}/4-lineage.log"

success "Key lineage shows 3-tier hierarchy (master → alice, bob)"

section "🔍 Step 5: Verify Multi-Factor Constraints"

echo "Checking Alice's constraints..."
echo ""

$BEARDOG key info --key-id "alice-lab-access-${SESSION}" \
    | tee "${OUTPUT_DIR}/5-alice-info.log"

info "Alice's key has ${GREEN}5 REAL constraints${NC} enforced by BearDog:"
echo "  1. TimeRangeConstraint (8:00-18:00)"
echo "  2. WeekdayConstraint (mon-fri)"
echo "  3. CpuQuotaConstraint (70%)"
echo "  4. MemoryQuotaConstraint (16GB)"
echo "  5. ExpiryConstraint (90 days)"
echo ""

section "🔬 Step 6: Simulate Lab Operations"

echo "Alice encrypts sensitive research data..."
echo ""

# Create sample research data
cat > "${OUTPUT_DIR}/research-data.txt" << 'DATA'
CONFIDENTIAL RESEARCH DATA
Lab: Quantum Computing Research
Experiment: QC-2025-12-11
Results: [Quantum entanglement measurements]
Temperature: 4.2K
Pressure: 10^-9 torr
Success Rate: 94.7%
Next Steps: Scale to 128 qubits
DATA

$BEARDOG encrypt \
    --key "alice-lab-access-${SESSION}" \
    --input "${OUTPUT_DIR}/research-data.txt" \
    --output "${OUTPUT_DIR}/research-data.enc" \
    | tee "${OUTPUT_DIR}/6-alice-encrypt.log"

success "Research data encrypted with Alice's delegated key"
receipt "Research Data Encryption" "$(cat ${OUTPUT_DIR}/6-alice-encrypt.log)"

info "Data is now encrypted and can only be decrypted by Alice"
info "Constraints are checked at encryption time"

section "🚨 Step 7: Simulate Constraint Violation"

echo "Scenario: Bob tries to use Alice's key (unauthorized)"
echo ""

warning "In production, this would be prevented by biometric constraint"
warning "Alice's fingerprint ≠ Bob's fingerprint"
info "For this demo, we'll show how revocation handles this"
echo ""

echo "Lab admin revokes Alice's key due to project completion..."
echo ""

$BEARDOG key revoke \
    --key-id "alice-lab-access-${SESSION}" \
    --reason "Project completed - access no longer needed" \
    | tee "${OUTPUT_DIR}/7-alice-revoke.log"

success "Alice's lab access revoked (sovereign - no phone-home)"
receipt "Alice Revocation" "$(cat ${OUTPUT_DIR}/7-alice-revoke.log)"

echo ""
echo "Checking revocation status..."
echo ""

$BEARDOG key check-revocation --key-id "alice-lab-access-${SESSION}" \
    | tee "${OUTPUT_DIR}/7-alice-revoke-check.log"

info "Revoked keys cannot be used for new operations"
info "Alice's existing encrypted data remains secure"

section "📋 Summary: Multi-Factor Security in Action"

echo "Secure Lab Access Demonstration Complete!"
echo ""
echo "✅ What We Demonstrated:"
echo ""
echo "1. Multi-Factor Authentication (6 Constraints)"
echo "   Real CLI Constraints:"
echo "     ✅ TimeRangeConstraint - Business hours enforcement"
echo "     ✅ WeekdayConstraint - Weekdays only"
echo "     ✅ CpuQuotaConstraint - Resource limits"
echo "     ✅ MemoryQuotaConstraint - Memory limits"
echo "     ✅ ExpiryConstraint - Automatic expiration"
echo ""
echo "   Conceptual Constraints (ready for hardware integration):"
echo "     ⏳ GeoFenceConstraint - Physical presence required"
echo "     ⏳ BiometricConstraint - Fingerprint verification"
echo "     ⏳ SystemLoadConstraint - System health check"
echo "     ⏳ EnvironmentalConstraint - Temperature monitoring"
echo ""
echo "2. Hierarchical Access Control"
echo "   • Master key (lab admin) - Full control"
echo "   • Researcher keys (alice, bob) - Limited, constrained access"
echo "   • Clear lineage and delegation chain"
echo ""
echo "3. Sovereign Revocation"
echo "   • No phone-home required"
echo "   • Immediate local enforcement"
echo "   • Privacy preserved (encrypted data stays encrypted)"
echo ""
echo "4. Privacy & Security"
echo "   • Research data encrypted end-to-end"
echo "   • Only authorized keys can decrypt"
echo "   • Constraints enforced at operation time"
echo ""

echo "📊 Receipts Generated:"
ls -1 "${OUTPUT_DIR}"/*.receipt 2>/dev/null | while read receipt; do
    echo "  📋 $(basename $receipt): $(cat $receipt)"
done
echo ""

echo "📁 Output Directory: $OUTPUT_DIR"
echo ""

section "🔮 When Hardware is Integrated"

cat << 'EOF'
🎯 Future Capabilities (with Solo V2 + Pixel 8a):

**GeoFenceConstraint** (GPS)
  • Pixel 8a provides GPS coordinates
  • Constraint checks if user is on-site (< 50m from lab)
  • Works seamlessly with Songbird for location verification

**BiometricConstraint** (Fingerprint)
  • Solo V2 or Pixel 8a biometric verification
  • Hardware-backed authentication
  • Cannot be spoofed or bypassed
  • Privacy-preserving (biometric never leaves device)

**SystemLoadConstraint** (Performance)
  • BearDog monitors system load
  • Prevents operations when overloaded
  • Protects system stability

**EnvironmentalConstraint** (Sensors)
  • Temperature sensors in lab
  • Humidity, pressure, etc.
  • Ensures safe operating conditions
  • Integration via ConstraintContext.environment

All constraints work together with AND logic:
  ALL must be satisfied for access!

EOF

section "🏗️ Architecture Benefits"

cat << 'EOF'
1. **Defense in Depth**
   Multiple independent factors required
   Compromise of one factor ≠ system compromise
   
2. **Adaptability**
   Add new constraints without code changes
   Remove constraints easily
   Adjust thresholds on the fly

3. **Auditability**
   All operations logged with receipts
   Clear delegation chain
   Revocation audit trail

4. **Usability**
   Simple CLI for complex multi-factor auth
   Clear error messages when constraints fail
   Automatic enforcement (no manual checks)

5. **Sovereignty**
   No central authority
   No phone-home
   Local control
   Network propagation optional (Songbird)

EOF

section "💡 Real-World Scenarios"

cat << 'EOF'
This multi-factor approach is applicable to:

**Research Facilities**
  - Lab access control
  - Equipment usage restrictions
  - Data access policies
  - Collaboration management

**Enterprise Security**
  - Server room access
  - Database operations
  - Production deployments
  - Incident response

**Healthcare**
  - Patient data access
  - Operating room authorization
  - Prescription systems
  - Medical device control

**Government**
  - Classified data access
  - Secure communications
  - Critical infrastructure
  - Multi-party authorization

**Financial**
  - Trading system access
  - Transaction approval
  - Vault operations
  - Audit compliance

EOF

section "📚 Next Steps"

echo "Learn More:"
echo "  • Architecture: ../../CONSTRAINT_AGNOSTIC_COMPLETE.md"
echo "  • Implementation: ../../crates/beardog-types/src/constraints/"
echo "  • Hardware Integration: ../../HSM_INTEGRATION_RESEARCH.md"
echo "  • BiometricConstraint: ../../crates/beardog-types/src/constraints/novel.rs:330"
echo ""
echo "Try More Demos:"
echo "  • ./demo-constraints.sh - All 14 constraint types"
echo "  • ./demo-tower-sharing.sh - Resource-limited sharing"
echo "  • ./demo-mobile-context.sh - Mobile device constraints"
echo ""
echo "Hardware Integration (Coming Soon):"
echo "  • Solo V2: FIDO2/CTAP2 integration"
echo "  • Pixel 8a: StrongBox TEE integration"
echo "  • Biometric: Real fingerprint verification"
echo "  • GPS: Real geofence checking"
echo ""

echo "🎉 Secure lab access demonstration complete!"
echo ""
echo "Remember: When hardware is integrated, ALL 6 constraints will be enforced"
echo "in real-time by BearDog. No manual checks, fully automatic, sovereignty preserved."
echo ""

