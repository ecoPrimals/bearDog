#!/usr/bin/env bash
#
# BearDog Mobile Context-Aware Operations
# Battery, Network, and System State Constraints
#

set -e

BEARDOG="${BEARDOG:-../../target/release/beardog}"
SESSION="mobile-context-$(date +%s)"
OUTPUT_DIR="./output-${SESSION}"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
RED='\033[0;31m'
NC='\033[0m'

echo ""
echo "📱 ================================================"
echo "   BearDog Mobile Context-Aware Operations"
echo "   Battery, Network, and System State Constraints"
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

section "📖 Scenario: Mobile Device Context-Aware Cryptography"

cat << 'EOF'
🎯 Use Case:
  Mobile devices have unique constraints that desktop systems don't:
  
  1. Battery Life: Don't drain battery with crypto on low power
  2. Network Trust: Only operate on trusted WiFi or VPN
  3. System Load: Avoid operations when device is overloaded
  4. Storage: Limit operations based on available storage
  
🔐 Security Model:
  Context-aware constraints adapt to device state
  Operations automatically adjust to conditions
  Battery-conscious cryptography
  Network-aware security decisions
  
🎨 Constraint Architecture:
  This demo uses OR logic for alternatives:
    (BatteryConstraint > 30%) AND
    (NetworkSsidConstraint OR VpnConstraint) AND
    (SystemLoadConstraint < 2.0)

📊 Constraint Status:
  ⏳ BatteryConstraint     - Conceptual (requires battery API integration)
  ⏳ NetworkSsidConstraint - Conceptual (requires WiFi API integration)
  ⏳ VpnConstraint         - Conceptual (requires VPN detection)
  ⏳ SystemLoadConstraint  - Conceptual (requires system monitoring)
  
  Note: This demo shows the DESIGN and ARCHITECTURE.
        Real implementation requires platform-specific APIs:
        - Android: BatteryManager, WifiManager, ConnectivityManager
        - iOS: UIDevice.batteryLevel, NEVPNManager, NetworkReachability
        - Linux: /sys/class/power_supply/, NetworkManager

EOF

section "🔑 Step 1: Generate Mobile Master Key"

echo "Generating master key for mobile device operations..."
echo "Using lighter KDF for mobile battery efficiency"
echo ""

$BEARDOG key generate \
    --key-id "mobile-master-${SESSION}" \
    --algorithm aes-256-gcm \
    --hsm software \
    --kdf argon2 \
    --kdf-iterations 2 \
    --kdf-memory 32768 \
    --kdf-time 2 \
    --usage all \
    --purpose "mobile-operations" \
    --expires-in 365d \
    | tee "${OUTPUT_DIR}/1-master-generate.log"

success "Mobile master key generated (battery-efficient KDF)"
receipt "Master Key Generation" "$(cat ${OUTPUT_DIR}/1-master-generate.log)"

info "KDF tuned for mobile: Lower iterations, less memory, faster"

section "📱 Step 2: Create Context-Aware Operation Key"

echo "Creating a key that adapts to mobile device state..."
echo ""
echo "CONCEPTUAL Constraints (would be enforced with real APIs):"
echo "  🔋 Battery > 30% - BatteryConstraint"
echo "  📶 Trusted Network - NetworkSsidConstraint OR VpnConstraint"
echo "  ⚡ System Load < 2.0 - SystemLoadConstraint"
echo ""
echo "REAL CLI Constraints (enforced now):"
echo "  ⏰ Time: 6:00-23:00 (awake hours)"
echo "  📅 Days: All days"
echo "  💻 CPU: Max 40% (mobile efficiency)"
echo "  🧠 Memory: Max 2GB (mobile limits)"
echo "  ⏱️  Expiry: 30 days"
echo ""

$BEARDOG key delegate \
    --master-key "mobile-master-${SESSION}" \
    --delegate-to "mobile-user" \
    --output "mobile-context-key-${SESSION}" \
    --time-range "6:00-23:00" \
    --cpu-quota 40 \
    --memory-quota "2GB" \
    --expires-in 30d \
    | tee "${OUTPUT_DIR}/2-mobile-delegate.log"

success "Context-aware mobile key created"
receipt "Mobile Delegation" "$(cat ${OUTPUT_DIR}/2-mobile-delegate.log)"

section "🎬 Step 3: Simulate Mobile Operation Scenarios"

echo "Scenario A: High Battery, Trusted WiFi ✅"
echo "  🔋 Battery: 80% (> 30% threshold)"
echo "  📶 Network: Home WiFi (trusted)"
echo "  ⚡ System Load: 0.5 (< 2.0 threshold)"
echo "  ✅ All constraints satisfied - operation allowed"
echo ""

# Create sample mobile data
cat > "${OUTPUT_DIR}/mobile-photo.txt" << 'DATA'
MOBILE PHOTO METADATA
Timestamp: 2025-12-11 13:45:00
Location: [Latitude, Longitude]
Device: Pixel 8a with GrapheneOS
Camera: 64MP Main
Settings: HDR+ Enhanced, Night Sight
File Size: 12.4 MB
DATA

$BEARDOG encrypt \
    --key "mobile-context-key-${SESSION}" \
    --input "${OUTPUT_DIR}/mobile-photo.txt" \
    --output "${OUTPUT_DIR}/mobile-photo.enc" \
    | tee "${OUTPUT_DIR}/3a-encrypt-high-battery.log"

success "Scenario A: Photo encrypted (optimal conditions)"
receipt "High Battery Encryption" "$(cat ${OUTPUT_DIR}/3a-encrypt-high-battery.log)"

echo ""
echo "Scenario B: Low Battery, Trusted Network ⚠️"
echo "  🔋 Battery: 20% (< 30% threshold)"
echo "  📶 Network: Office WiFi (trusted)"
echo "  ⚡ System Load: 0.8"
echo "  ⚠️  BatteryConstraint NOT satisfied - operation would be blocked"
echo ""

warning "In production: BatteryConstraint would prevent this operation"
info "Mobile device preserves battery by deferring non-critical crypto"
info "Operation queued for later when battery > 30%"

echo ""
echo "Scenario C: High Battery, Untrusted Network ⚠️"
echo "  🔋 Battery: 85%"
echo "  📶 Network: Public WiFi (untrusted)"
echo "  📶 VPN: Not connected"
echo "  ⚠️  NetworkConstraint NOT satisfied - operation blocked"
echo ""

warning "In production: NetworkConstraint requires trusted network or VPN"
info "User is prompted to connect to VPN before sensitive operations"

echo ""
echo "Scenario D: System Overloaded 🔥"
echo "  🔋 Battery: 60%"
echo "  📶 Network: Trusted"
echo "  ⚡ System Load: 3.5 (> 2.0 threshold)"
echo "  ⚠️  SystemLoadConstraint NOT satisfied - operation deferred"
echo ""

warning "In production: SystemLoadConstraint prevents operations during high load"
info "Protects device performance and user experience"

section "📊 Step 4: Visualize Mobile Key Hierarchy"

echo "Mobile device key structure:"
echo ""

$BEARDOG key lineage --key-id "mobile-master-${SESSION}" \
    | tee "${OUTPUT_DIR}/4-lineage.log"

success "Key lineage shows context-aware delegation"

section "🔍 Step 5: Context State Detection (Conceptual)"

echo "How BearDog Would Detect Context on Real Mobile Platforms:"
echo ""

cat << 'EOF'
📱 **Android (via JNI)**
  
  Battery State:
    BatteryManager batteryManager = getSystemService(BATTERY_SERVICE);
    int level = batteryManager.getIntProperty(BATTERY_PROPERTY_CAPACITY);
    return level > 30;
  
  Network State:
    WifiManager wifi = getSystemService(WIFI_SERVICE);
    String ssid = wifi.getConnectionInfo().getSSID();
    return TRUSTED_SSIDS.contains(ssid);
  
  VPN State:
    ConnectivityManager cm = getSystemService(CONNECTIVITY_SERVICE);
    Network[] networks = cm.getAllNetworks();
    for (Network network : networks) {
        if (cm.getNetworkCapabilities(network).hasTransport(TRANSPORT_VPN)) {
            return true;
        }
    }
  
  System Load:
    ActivityManager.MemoryInfo memInfo = new ActivityManager.MemoryInfo();
    activityManager.getMemoryInfo(memInfo);
    long availMem = memInfo.availMem;
    long totalMem = memInfo.totalMem;
    return (totalMem - availMem) / totalMem < 0.7;

📱 **iOS (via Swift/Objective-C)**
  
  Battery State:
    UIDevice.current.isBatteryMonitoringEnabled = true
    let level = UIDevice.current.batteryLevel
    return level > 0.30
  
  Network State:
    import Network
    let monitor = NWPathMonitor()
    monitor.pathUpdateHandler = { path in
        if path.usesInterfaceType(.wifi) {
            // Check SSID
        }
    }
  
  VPN State:
    import NetworkExtension
    NEVPNManager.shared().loadFromPreferences { error in
        return NEVPNManager.shared().connection.status == .connected
    }

🐧 **Linux/Desktop**
  
  Battery (if available):
    cat /sys/class/power_supply/BAT0/capacity
    # Returns percentage
  
  Network:
    nmcli -t -f TYPE,STATE device
    # Parse WiFi connection state
  
  VPN:
    ip link show | grep tun
    # Check for VPN tunnel interfaces
  
  System Load:
    cat /proc/loadavg
    # First three numbers are 1/5/15 min load averages

EOF

success "Context detection APIs identified for all platforms"

section "🎨 Architecture: Constraint Context Provider"

cat << 'EOF'
How Context Gets to Constraints:

```rust
// 1. Platform-specific detection
#[cfg(target_os = "android")]
fn get_battery_level() -> Result<u8, BearDogError> {
    // JNI call to Android BatteryManager
    let jni_env = get_jni_env()?;
    let battery_manager = jni_env.call_method(
        "android/os/BatteryManager",
        "getIntProperty",
        ...
    )?;
    Ok(battery_level)
}

#[cfg(target_os = "ios")]
fn get_battery_level() -> Result<u8, BearDogError> {
    // Objective-C bridge to UIDevice
    unsafe {
        let device = UIDevice::currentDevice();
        let level = device.batteryLevel();
        Ok((level * 100.0) as u8)
    }
}

// 2. Build ConstraintContext
let mut context = ConstraintContext::new()
    .with_current_time(Utc::now())
    .with_system_state(SystemState::current()?);

// 3. Add platform-specific state
#[cfg(any(target_os = "android", target_os = "ios"))]
{
    let battery = get_battery_level()?;
    context.environment.insert(
        "battery_percent".to_string(),
        json!(battery)
    );
    
    let network = get_network_state()?;
    context = context.with_network_state(network);
}

// 4. Evaluate constraints
for constraint in &constraints {
    if !constraint.is_satisfied(&context)? {
        return Err(BearDogError::constraint(
            &constraint.description()
        ));
    }
}
```

Key Points:
  • Platform-specific detection via conditional compilation
  • Extensible context (HashMap for new data types)
  • Constraints access context, not platform APIs directly
  • Clean separation of concerns
  • Zero overhead when constraints not used

EOF

section "💡 Real-World Mobile Scenarios"

cat << 'EOF'
**Scenario 1: Photo App with Privacy**
  User takes photos with sensitive content
  
  Constraints:
    • BatteryConstraint > 30% (don't drain battery)
    • NetworkSsidConstraint: Home/Office (only encrypt on trusted networks)
    • StorageConstraint: > 1GB available
  
  Behavior:
    • High battery + Home WiFi → Encrypt immediately
    • Low battery → Queue for later
    • Untrusted network → Warn user, defer encryption

**Scenario 2: Secure Messaging**
  End-to-end encrypted messages
  
  Constraints:
    • BatteryConstraint > 20% (lower threshold for messages)
    • NetworkConstraint: (Trusted WiFi OR VPN)
    • SystemLoadConstraint: < 1.5
  
  Behavior:
    • Trusted network → Send immediately
    • Untrusted network → Auto-connect VPN or queue
    • System overloaded → Defer non-urgent messages

**Scenario 3: Cloud Backup**
  Automatic encrypted backup
  
  Constraints:
    • BatteryConstraint > 50% AND Charging
    • NetworkSsidConstraint: Home WiFi
    • TimeRangeConstraint: Night hours (22:00-6:00)
    • StorageConstraint: > 5GB available
  
  Behavior:
    • All constraints met → Backup proceeds
    • Any constraint fails → Backup deferred
    • User can override for manual backup

**Scenario 4: Corporate Device**
  BYOD with company data
  
  Constraints:
    • GeoFenceConstraint: Within office building
    • BatteryConstraint: > 40%
    • VpnConstraint: Company VPN connected
    • BiometricConstraint: Fingerprint required
  
  Behavior:
    • All constraints enforced strictly
    • No override possible
    • Access revoked when constraints fail

EOF

section "📋 Summary & Implementation Path"

echo "Mobile Context-Aware Operations Complete!"
echo ""
echo "✅ What This Demo Showed:"
echo ""
echo "1. Context-Aware Architecture"
echo "   • Constraints adapt to device state"
echo "   • Battery-conscious operations"
echo "   • Network-aware security"
echo "   • System load management"
echo ""
echo "2. Platform Integration Design"
echo "   • Android API integration plan"
echo "   • iOS API integration plan"
echo "   • Linux/Desktop support"
echo "   • Conditional compilation strategy"
echo ""
echo "3. Real-World Scenarios"
echo "   • Photo encryption with constraints"
echo "   • Secure messaging behavior"
echo "   • Automatic backup scheduling"
echo "   • Corporate device policies"
echo ""

echo "📊 Receipts Generated:"
ls -1 "${OUTPUT_DIR}"/*.receipt 2>/dev/null | while read receipt; do
    echo "  📋 $(basename $receipt): $(cat $receipt)"
done
echo ""

section "🚀 Implementation Roadmap"

cat << 'EOF'
**Phase 1: Platform Detection (1-2 weeks)**
  • Add JNI bindings for Android
  • Add Swift/ObjC bridge for iOS
  • Implement battery detection
  • Implement network detection
  • Test on real devices

**Phase 2: Constraint Wiring (1 week)**
  • Wire BatteryConstraint to APIs
  • Wire NetworkSsidConstraint to WiFi APIs
  • Wire VpnConstraint to VPN detection
  • Wire SystemLoadConstraint to system monitoring
  • Integration testing

**Phase 3: Mobile Demo App (1-2 weeks)**
  • Create Android demo app
  • Create iOS demo app
  • Showcase context-aware crypto
  • Real device testing
  • User experience refinement

**Phase 4: Production Hardening (1 week)**
  • Edge case handling
  • Battery optimization
  • Performance tuning
  • Security audit
  • Documentation

**Total Estimate: 4-6 weeks for full mobile integration**

EOF

section "📚 Next Steps"

echo "Learn More:"
echo "  • Constraints: ../../crates/beardog-types/src/constraints/novel.rs"
echo "  • BatteryConstraint: Line 545-610"
echo "  • NetworkSsidConstraint: Line 205-260"
echo "  • VpnConstraint: Line 260-315"
echo "  • SystemLoadConstraint: Line 410-475"
echo ""
echo "Try Other Demos:"
echo "  • ./demo-constraints.sh - All 14 constraint types"
echo "  • ./demo-tower-sharing.sh - Resource limits"
echo "  • ./demo-secure-lab.sh - Multi-factor auth"
echo ""
echo "Mobile Integration:"
echo "  • See: ../../HSM_INTEGRATION_RESEARCH.md"
echo "  • Platform: Android + iOS specific code paths"
echo "  • Testing: Requires physical mobile devices"
echo ""

echo "🎉 Mobile context-aware operations demonstration complete!"
echo ""
echo "Remember: This demo shows the ARCHITECTURE and DESIGN."
echo "Real mobile integration requires platform-specific APIs (JNI for Android, Swift bridge for iOS)."
echo "The constraint system is ready - just needs platform wiring!"
echo ""

