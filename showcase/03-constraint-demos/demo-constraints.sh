#!/usr/bin/env bash
#
# BearDog Constraint System Demonstration
# All 14 constraint types with real CLI operations
#

set -e

BEARDOG="${BEARDOG:-../../target/release/beardog}"
SESSION="constraint-demo-$(date +%s)"
OUTPUT_DIR="./output-${SESSION}"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo ""
echo "🎨 ================================================"
echo "   BearDog Constraint System Demonstration"
echo "   All 14 Constraint Types + Real CLI"
echo "================================================"
echo ""

mkdir -p "$OUTPUT_DIR"

# Helper functions
receipt() {
    local title="$1"
    local data="$2"
    echo "$data" | sha256sum | awk '{print $1}' > "${OUTPUT_DIR}/${title// /-}.receipt"
    echo "📋 Receipt: ${title// /-}.receipt"
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

# Check BearDog CLI
if [ ! -f "$BEARDOG" ]; then
    echo "❌ BearDog CLI not found. Building..."
    cd ../../ && cargo build --release && cd - > /dev/null
fi

section "📦 PART 1: Built-in Constraints (6 types)"

# 1. TimeRangeConstraint
section "⏰ 1. TimeRangeConstraint - Active Hours"
echo "Use Case: 'Only allow access during business hours (9:00-17:00)'"
echo ""

$BEARDOG key generate \
    --key-id "master-time-${SESSION}" \
    --algorithm aes-256-gcm \
    --hsm software \
    --kdf argon2 \
    > "${OUTPUT_DIR}/1-time-generate.log" 2>&1

$BEARDOG key delegate \
    --master-key "master-time-${SESSION}" \
    --delegate-to "business-hours-user" \
    --output "delegated-time-${SESSION}" \
    --time-range "9:00-17:00" \
    --expires-in 30d \
    > "${OUTPUT_DIR}/1-time-delegate.log" 2>&1

success "TimeRangeConstraint: Business hours (9:00-17:00)"
receipt "TimeRangeConstraint" "$(cat ${OUTPUT_DIR}/1-time-delegate.log)"

# 2. WeekdayConstraint
section "📅 2. WeekdayConstraint - Active Days"
echo "Use Case: 'Only allow access on weekdays (mon-fri)'"
echo ""

$BEARDOG key generate \
    --key-id "master-weekday-${SESSION}" \
    --algorithm aes-256-gcm \
    --hsm software \
    > "${OUTPUT_DIR}/2-weekday-generate.log" 2>&1

$BEARDOG key delegate \
    --master-key "master-weekday-${SESSION}" \
    --delegate-to "weekday-user" \
    --output "delegated-weekday-${SESSION}" \
    --weekdays "mon-fri" \
    --expires-in 30d \
    > "${OUTPUT_DIR}/2-weekday-delegate.log" 2>&1

success "WeekdayConstraint: Weekdays only (mon-fri)"
receipt "WeekdayConstraint" "$(cat ${OUTPUT_DIR}/2-weekday-delegate.log)"

# 3. CpuQuotaConstraint
section "💻 3. CpuQuotaConstraint - CPU Limit"
echo "Use Case: 'Limit CPU usage to 50% for delegated operations'"
echo ""

$BEARDOG key generate \
    --key-id "master-cpu-${SESSION}" \
    --algorithm aes-256-gcm \
    --hsm software \
    > "${OUTPUT_DIR}/3-cpu-generate.log" 2>&1

$BEARDOG key delegate \
    --master-key "master-cpu-${SESSION}" \
    --delegate-to "limited-cpu-user" \
    --output "delegated-cpu-${SESSION}" \
    --cpu-quota 50 \
    --expires-in 30d \
    > "${OUTPUT_DIR}/3-cpu-delegate.log" 2>&1

success "CpuQuotaConstraint: Max 50% CPU"
receipt "CpuQuotaConstraint" "$(cat ${OUTPUT_DIR}/3-cpu-delegate.log)"

# 4. MemoryQuotaConstraint
section "🧠 4. MemoryQuotaConstraint - Memory Limit"
echo "Use Case: 'Limit memory usage to 8GB for delegated operations'"
echo ""

$BEARDOG key generate \
    --key-id "master-memory-${SESSION}" \
    --algorithm aes-256-gcm \
    --hsm software \
    > "${OUTPUT_DIR}/4-memory-generate.log" 2>&1

$BEARDOG key delegate \
    --master-key "master-memory-${SESSION}" \
    --delegate-to "limited-memory-user" \
    --output "delegated-memory-${SESSION}" \
    --memory-quota "8GB" \
    --expires-in 30d \
    > "${OUTPUT_DIR}/4-memory-delegate.log" 2>&1

success "MemoryQuotaConstraint: Max 8GB memory"
receipt "MemoryQuotaConstraint" "$(cat ${OUTPUT_DIR}/4-memory-delegate.log)"

# 5. ExpiryConstraint
section "⏱️  5. ExpiryConstraint - Time-Limited"
echo "Use Case: 'Key expires automatically in 7 days'"
echo ""

$BEARDOG key generate \
    --key-id "expiring-key-${SESSION}" \
    --algorithm aes-256-gcm \
    --hsm software \
    --expires-in 7d \
    > "${OUTPUT_DIR}/5-expiry-generate.log" 2>&1

success "ExpiryConstraint: Expires in 7 days"
receipt "ExpiryConstraint" "$(cat ${OUTPUT_DIR}/5-expiry-generate.log)"

# 6. CompositeConstraint (AND logic)
section "🔗 6. CompositeConstraint - Combined (AND logic)"
echo "Use Case: 'Business hours AND weekdays AND CPU limit AND memory limit'"
echo ""

$BEARDOG key generate \
    --key-id "master-composite-${SESSION}" \
    --algorithm aes-256-gcm \
    --hsm software \
    > "${OUTPUT_DIR}/6-composite-generate.log" 2>&1

$BEARDOG key delegate \
    --master-key "master-composite-${SESSION}" \
    --delegate-to "fully-constrained-user" \
    --output "delegated-composite-${SESSION}" \
    --time-range "9:00-17:00" \
    --weekdays "mon-fri" \
    --cpu-quota 50 \
    --memory-quota "4GB" \
    --expires-in 30d \
    > "${OUTPUT_DIR}/6-composite-delegate.log" 2>&1

success "CompositeConstraint: Time AND Weekday AND CPU AND Memory (4 constraints)"
receipt "CompositeConstraint" "$(cat ${OUTPUT_DIR}/6-composite-delegate.log)"

section "📦 PART 2: Novel Constraint Examples (8 types)"

echo "Note: Novel constraints demonstrate the extensibility of the system."
echo "These are implemented in crates/beardog-types/src/constraints/novel.rs"
echo "Users can create their own constraints following these patterns!"
echo ""

# 7. ProximityConstraint (conceptual - requires location data)
section "📍 7. ProximityConstraint - GPS Proximity"
echo "Use Case: 'Allow access only when within 100m of another party'"
echo ""
echo "Constraint Details:"
echo "  - Type: ProximityConstraint"
echo "  - Max Distance: 100 meters"
echo "  - Other Party: Alice"
echo "  - Implementation: crates/beardog-types/src/constraints/novel.rs:45"
echo ""
success "ProximityConstraint: Within 100m of Alice (conceptual)"
echo "📋 To use: Provide location data via ConstraintContext"

# 8. GeoFenceConstraint (conceptual)
section "🗺️  8. GeoFenceConstraint - Geographic Boundary"
echo "Use Case: 'Allow key use only inside secure facility (50m radius)'"
echo ""
echo "Constraint Details:"
echo "  - Type: GeoFenceConstraint"
echo "  - Center: (37.7749, -122.4194)"
echo "  - Radius: 50 meters"
echo "  - Implementation: crates/beardog-types/src/constraints/novel.rs:475"
echo ""
success "GeoFenceConstraint: 50m radius geofence (conceptual)"
echo "📋 To use: Provide location data via ConstraintContext"

# 9. EnvironmentalConstraint (conceptual)
section "🌡️  9. EnvironmentalConstraint - Sensor Conditions"
echo "Use Case: 'Allow operations only when temperature is 18-25°C'"
echo ""
echo "Constraint Details:"
echo "  - Type: EnvironmentalConstraint"
echo "  - Sensor: room-temp"
echo "  - Min: 18.0°C"
echo "  - Max: 25.0°C"
echo "  - Implementation: crates/beardog-types/src/constraints/novel.rs:125"
echo ""
success "EnvironmentalConstraint: Temperature 18-25°C (conceptual)"
echo "📋 To use: Provide sensor data via ConstraintContext.environment"

# 10. NetworkSsidConstraint (conceptual)
section "📶 10. NetworkSsidConstraint - WiFi Network"
echo "Use Case: 'Allow access only on trusted WiFi networks'"
echo ""
echo "Constraint Details:"
echo "  - Type: NetworkSsidConstraint"
echo "  - Allowed SSIDs: ['Home', 'Office', 'SecureNet']"
echo "  - Implementation: crates/beardog-types/src/constraints/novel.rs:205"
echo ""
success "NetworkSsidConstraint: Trusted WiFi only (conceptual)"
echo "📋 To use: Provide network state via ConstraintContext"

# 11. VpnConstraint (conceptual)
section "🔒 11. VpnConstraint - VPN Requirement"
echo "Use Case: 'Require Songbird VPN connection'"
echo ""
echo "Constraint Details:"
echo "  - Type: VpnConstraint"
echo "  - Required Tunnel: songbird"
echo "  - Implementation: crates/beardog-types/src/constraints/novel.rs:260"
echo ""
success "VpnConstraint: Songbird VPN required (conceptual)"
echo "📋 To use: Provide VPN state via ConstraintContext"

# 12. BiometricConstraint (conceptual - requires hardware)
section "👆 12. BiometricConstraint - Hardware Auth"
echo "Use Case: 'Require fingerprint verification on Solo V2 key'"
echo ""
echo "Constraint Details:"
echo "  - Type: BiometricConstraint"
echo "  - Biometric Type: Fingerprint"
echo "  - Device: solo-v2"
echo "  - Implementation: crates/beardog-types/src/constraints/novel.rs:330"
echo ""
success "BiometricConstraint: Fingerprint on Solo V2 (requires hardware)"
echo "📋 To use: Integrate with hardware HSM (see Phase 2 hardware demos)"

# 13. SystemLoadConstraint (conceptual)
section "⚡ 13. SystemLoadConstraint - Performance Limit"
echo "Use Case: 'Prevent operations when system is overloaded'"
echo ""
echo "Constraint Details:"
echo "  - Type: SystemLoadConstraint"
echo "  - Max Load (1m): 2.0"
echo "  - Implementation: crates/beardog-types/src/constraints/novel.rs:410"
echo ""
success "SystemLoadConstraint: Load < 2.0 (conceptual)"
echo "📋 To use: Provide system state via ConstraintContext"

# 14. BatteryConstraint (conceptual)
section "🔋 14. BatteryConstraint - Power Level"
echo "Use Case: 'Prevent high-power operations when battery is low'"
echo ""
echo "Constraint Details:"
echo "  - Type: BatteryConstraint"
echo "  - Min Percent: 20%"
echo "  - Implementation: crates/beardog-types/src/constraints/novel.rs:545"
echo ""
success "BatteryConstraint: Battery > 20% (conceptual)"
echo "📋 To use: Provide battery state via ConstraintContext.environment"

section "📊 Summary & Receipts"

echo "Constraint System Demonstration Complete!"
echo ""
echo "Built-in Constraints Demonstrated (with real CLI):"
echo "  ✅ 1. TimeRangeConstraint - Business hours"
echo "  ✅ 2. WeekdayConstraint - Weekdays only"
echo "  ✅ 3. CpuQuotaConstraint - CPU limit"
echo "  ✅ 4. MemoryQuotaConstraint - Memory limit"
echo "  ✅ 5. ExpiryConstraint - Time-limited"
echo "  ✅ 6. CompositeConstraint - Multiple constraints (AND logic)"
echo ""
echo "Novel Constraint Examples (extensibility demo):"
echo "  📍 7. ProximityConstraint - GPS proximity"
echo "  🗺️  8. GeoFenceConstraint - Geographic boundary"
echo "  🌡️  9. EnvironmentalConstraint - Sensor conditions"
echo "  📶 10. NetworkSsidConstraint - WiFi network"
echo "  🔒 11. VpnConstraint - VPN requirement"
echo "  👆 12. BiometricConstraint - Hardware auth"
echo "  ⚡ 13. SystemLoadConstraint - Performance limit"
echo "  🔋 14. BatteryConstraint - Power level"
echo ""
echo "Total: 14 constraint types (6 built-in + 8 novel examples)"
echo ""

echo "Receipts Generated:"
ls -1 "${OUTPUT_DIR}"/*.receipt 2>/dev/null | while read receipt; do
    echo "  📋 $(basename $receipt)"
done
echo ""

echo "Output Directory: $OUTPUT_DIR"
echo ""

section "🎨 Key Takeaways"

echo "1. Built-in constraints work with real BearDog CLI"
echo "2. Constraints can be combined with AND logic (CompositeConstraint)"
echo "3. Novel constraints demonstrate infinite extensibility"
echo "4. Users can create their own constraints without modifying BearDog core"
echo "5. All constraints use the same Constraint trait"
echo "6. Context provides runtime data (time, location, sensors, etc.)"
echo ""

echo "Philosophy: 'Users define their own rules. We provide the framework, not the limits.'"
echo ""

section "📚 Next Steps"

echo "Learn More:"
echo "  • Quick Reference: ../../QUICK_REFERENCE_CONSTRAINTS.md"
echo "  • Complete Guide: ../../CONSTRAINT_EXTENSIBILITY_GUIDE.md"
echo "  • Implementation: ../../crates/beardog-types/src/constraints/"
echo ""
echo "Try More Demos:"
echo "  • ./demo-secure-lab.sh - Multi-factor authentication"
echo "  • ./demo-tower-sharing.sh - Resource-limited delegation"
echo "  • ./demo-mobile-context.sh - Context-aware operations"
echo ""

echo "🎉 Constraint system demonstration complete!"
echo ""

