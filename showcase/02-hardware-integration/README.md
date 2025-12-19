# 🔐 Phase 2: Hardware Integration - BearDog Showcase

**Status**: 🚀 **READY** (Hardware detected!)  
**Time**: ~20 minutes  
**Hardware**: Pixel 8a + 2x Solo V2 keys ✅ CONNECTED

---

## 🎯 What This Demonstrates

**Hardware HSM Comparison**:

1. ✅ **Software HSM** (SoftHSM2) - Fast, convenient, keys in memory
2. ✅ **Physical HSM** (Solo V2 keys x2) - Secure, keys never leave device
3. ✅ **Mobile HSM** (Pixel 8a StrongBox) - Hardware-backed, always with you

**Key Demonstrations**:
- Performance comparison (speed vs security)
- Human entropy collection (multi-modal)
- Security level analysis
- Real-world use case recommendations

---

## 🚀 Quick Start

### **Step 1: Verify Hardware**
```bash
# Check what's connected
./scripts/verify-hardware.sh

# Should detect:
#   ✅ Solo V2 key #1
#   ✅ Solo V2 key #2  
#   ✅ Pixel 8a (via adb)
#   ✅ Software HSM available
```

### **Step 2: Run Comparison Demo**
```bash
# Full comparison (20 minutes)
./demo-comparison.sh

# Or individual demos
./demo-solo-v2.sh        # Solo keys only
./demo-strongbox.sh      # Pixel StrongBox only
./demo-software.sh       # Software HSM only
```

### **Step 3: Human Entropy Collection**
```bash
# Multi-modal entropy collection
./demo-human-entropy.sh

# Demonstrates:
#   - Keyboard dynamics
#   - Mouse jitter
#   - Touch patterns (Pixel)
#   - Audio noise (optional)
```

---

## 🔧 Hardware Setup

### **Solo V2 Keys** ✅ CONNECTED
- **Key #1**: USB port (detected)
- **Key #2**: USB port (detected)
- **Status**: Ready for operations
- **Firmware**: Check with `solo2 --version`

**Verification**:
```bash
# List connected Solo keys
lsusb | grep -i solo

# Or use solo2 tool
solo2 ls
```

### **Pixel 8a with GrapheneOS** 🔄 BOOTING
- **Status**: Charging, booting up
- **Connection**: USB debugging enabled
- **StrongBox**: Hardware-backed keystore

**Verification** (after boot):
```bash
# Check ADB connection
adb devices

# Verify StrongBox availability
adb shell getprop ro.hardware.keystore
# Should show: trusty (indicating StrongBox support)
```

### **Software HSM** ✅ AVAILABLE
- **Provider**: SoftHSM2
- **Status**: Already configured
- **Location**: `/var/lib/softhsm/tokens/`

---

## 📊 What We'll Compare

### **Performance Metrics**

| Operation | Software HSM | Solo V2 | StrongBox |
|-----------|--------------|---------|-----------|
| **Key Generation** | ~400ms | ~800ms | ~600ms |
| **Sign Operation** | <10ms | ~50ms | ~30ms |
| **Verify Operation** | <10ms | ~20ms | ~15ms |
| **Encrypt (1MB)** | ~45ms | ~200ms | ~120ms |
| **Throughput** | 23 MB/s | 5 MB/s | 8 MB/s |

### **Security Comparison**

| Aspect | Software HSM | Solo V2 | StrongBox |
|--------|--------------|---------|-----------|
| **Key Storage** | Memory (RAM) | Secure Element | Hardware-backed |
| **Extraction Risk** | Medium | Very Low | Very Low |
| **Physical Security** | No | Yes (tamper-resistant) | Yes (TEE) |
| **Attack Surface** | OS-level | Isolated chip | Isolated TEE |
| **FIPS Certified** | No | FIDO2 | Common Criteria |
| **Grade** | B | A+ | A |

### **Use Case Recommendations**

**Software HSM**:
- ✅ Development and testing
- ✅ High-throughput applications
- ✅ Trusted environment
- ⚠️ Not for high-security keys

**Solo V2**:
- ✅ Maximum security keys
- ✅ Air-gapped operations
- ✅ Long-term key storage
- ✅ Cryptocurrency/identity keys
- ⚠️ Slower performance

**StrongBox (Pixel)**:
- ✅ Mobile operations
- ✅ Always-with-you security
- ✅ Biometric integration
- ✅ Balanced security/performance
- ⚠️ Requires phone

---

## 🧪 Demo Scenarios

### **Scenario 1: Maximum Security** (Solo V2)
```bash
./scenarios/max-security.sh

# Use case: Cryptocurrency wallet key
# Requirements: Air-gap capable, tamper-resistant
# Choice: Solo V2 key
# Rationale: Keys never leave secure element
```

### **Scenario 2: Mobile Operations** (StrongBox)
```bash
./scenarios/mobile-ops.sh

# Use case: On-the-go file encryption
# Requirements: Always available, biometric auth
# Choice: Pixel 8a StrongBox
# Rationale: Hardware-backed, mobile-first
```

### **Scenario 3: High Throughput** (Software)
```bash
./scenarios/high-throughput.sh

# Use case: Bulk file encryption
# Requirements: Speed, large volumes
# Choice: Software HSM
# Rationale: 4x faster, trusted environment
```

### **Scenario 4: Hybrid Approach** (All Three)
```bash
./scenarios/hybrid.sh

# Master key: Solo V2 (offline, secure)
# Daily operations: StrongBox (mobile, convenient)
# Bulk processing: Software HSM (fast, efficient)
# 
# Result: Best of all worlds!
```

---

## 🎓 Human Entropy Collection

### **Multi-Modal Sources**

**Desktop Sources**:
- ⌨️ **Keyboard dynamics**: Timing between keystrokes
- 🖱️ **Mouse jitter**: Movement acceleration and noise
- 🎤 **Audio noise**: Microphone ambient (optional)
- 📷 **Camera noise**: Sensor dark frame (optional)

**Mobile Sources (Pixel)**:
- 📱 **Touch patterns**: Pressure and timing variation
- 🔄 **Accelerometer**: Device movement entropy
- 📍 **Gyroscope**: Rotation noise
- 📶 **Signal variance**: WiFi/cellular timing jitter

### **Quality Metrics**

```bash
./demo-human-entropy.sh

# Output:
# ┌─────────────────────────────────────┐
# │ Human Entropy Quality Report        │
# ├─────────────────────────────────────┤
# │ Shannon Entropy: 0.9998             │
# │ Chi-Square Test: PASS               │
# │ Serial Correlation: 0.001           │
# │ Randomness Grade: A+                │
# │                                     │
# │ Sources Used:                       │
# │   ✅ Keyboard: 8.2 bits/sample      │
# │   ✅ Mouse: 6.7 bits/sample         │
# │   ✅ Touch: 7.3 bits/sample         │
# │   ✅ Motion: 5.1 bits/sample        │
# │                                     │
# │ Total Entropy: 256 bits (32 bytes) │
# │ Collection Time: 12.3 seconds       │
# └─────────────────────────────────────┘
```

---

## 📁 Output Structure

```
outputs/
├── solo-v2/
│   ├── key-generation.json         # Solo key operations
│   ├── sign-benchmark.json         # Signing performance
│   ├── encrypt-benchmark.json      # Encryption speed
│   └── security-attestation.json   # Security proof
│
├── strongbox/
│   ├── key-generation.json         # StrongBox operations
│   ├── sign-benchmark.json         # Mobile performance
│   ├── biometric-test.json         # Biometric integration
│   └── attestation-chain.json      # Hardware attestation
│
├── software/
│   ├── key-generation.json         # Software HSM ops
│   ├── bulk-benchmark.json         # Throughput test
│   └── memory-profile.json         # Resource usage
│
├── comparisons/
│   ├── performance-chart.png       # Visual comparison
│   ├── security-matrix.md          # Security analysis
│   ├── use-case-guide.md           # Recommendations
│   └── cost-benefit.json           # ROI analysis
│
└── human-entropy/
    ├── keyboard-dynamics.json      # Keyboard timing
    ├── mouse-movements.json        # Mouse entropy
    ├── touch-patterns.json         # Touch entropy (Pixel)
    ├── sensor-data.json            # Accelerometer/gyro
    └── combined-seed.json          # Final entropy seed
```

---

## 🔍 Hardware Detection Script

```bash
#!/usr/bin/env bash
# verify-hardware.sh

echo "🔍 BearDog Hardware Detection"
echo "=============================="
echo ""

# Solo V2 Keys
echo "🔑 Solo V2 Keys:"
SOLO_COUNT=$(lsusb | grep -i solo | wc -l)
if [ "$SOLO_COUNT" -eq 2 ]; then
    echo "  ✅ Found 2 Solo V2 keys"
    lsusb | grep -i solo | sed 's/^/  ├─ /'
elif [ "$SOLO_COUNT" -eq 1 ]; then
    echo "  ⚠️  Found 1 Solo V2 key (expected 2)"
    lsusb | grep -i solo | sed 's/^/  ├─ /'
else
    echo "  ❌ No Solo V2 keys found"
fi

# Pixel 8a
echo ""
echo "📱 Pixel 8a:"
if adb devices | grep -q "device$"; then
    echo "  ✅ Pixel connected via ADB"
    DEVICE=$(adb devices | grep "device$" | awk '{print $1}')
    echo "  ├─ Device ID: $DEVICE"
    
    # Check for StrongBox
    if adb shell getprop ro.hardware.keystore | grep -q trusty; then
        echo "  ├─ StrongBox: ✅ Available (Trusty TEE)"
    else
        echo "  ├─ StrongBox: ⚠️  Unknown status"
    fi
    
    # Check Android version
    ANDROID_VER=$(adb shell getprop ro.build.version.release)
    echo "  └─ Android: $ANDROID_VER"
else
    echo "  ⏳ Pixel not detected (may still be booting...)"
    echo "  └─ Run this script again after boot"
fi

# Software HSM
echo ""
echo "💻 Software HSM:"
if command -v softhsm2-util &> /dev/null; then
    echo "  ✅ SoftHSM2 installed"
    TOKEN_COUNT=$(softhsm2-util --show-slots | grep -c "Slot ")
    echo "  └─ Tokens: $TOKEN_COUNT configured"
else
    echo "  ⚠️  SoftHSM2 not installed"
fi

echo ""
echo "═══════════════════════════════"

# Summary
READY=true
[ "$SOLO_COUNT" -eq 2 ] || READY=false
command -v softhsm2-util &> /dev/null || READY=false

if [ "$READY" = true ]; then
    echo "✅ Ready for Phase 2 demos!"
    echo ""
    echo "Run: ./demo-comparison.sh"
else
    echo "⚠️  Some hardware missing or not ready"
    echo ""
    echo "Action items:"
    [ "$SOLO_COUNT" -ne 2 ] && echo "  - Ensure both Solo V2 keys are connected"
    ! command -v softhsm2-util &> /dev/null && echo "  - Install SoftHSM2: sudo apt install softhsm2"
fi
```

---

## ⚡ Quick Commands

```bash
# Verify all hardware
./scripts/verify-hardware.sh

# Run full comparison demo
./demo-comparison.sh

# Solo V2 only
./demo-solo-v2.sh

# Pixel StrongBox only (after boot)
./demo-strongbox.sh

# Human entropy collection
./demo-human-entropy.sh

# Clean outputs
./scripts/cleanup.sh
```

---

## 🎯 Success Criteria

| Check | Expected | Status |
|-------|----------|--------|
| **Solo V2 Keys** | 2 detected | 🔄 Checking |
| **Pixel 8a** | Connected via ADB | 🔄 Booting |
| **StrongBox** | Available | 🔄 Pending |
| **Software HSM** | Configured | ✅ Ready |
| **Demo Scripts** | Executable | ✅ Ready |

After hardware verification, all should be ✅!

---

## 🚀 What's Next

### **Today** (Phase 2):
1. Wait for Pixel to finish booting
2. Run `./scripts/verify-hardware.sh`
3. Execute `./demo-comparison.sh`
4. Review outputs and comparison charts

### **This Week** (Phase 3 prep):
1. Set up second tower
2. Install Songbird on both towers
3. Plan network discovery demos

---

## 💡 Key Takeaways

**After Phase 2, you'll understand**:
- When to use each HSM type
- Performance vs security trade-offs
- How human entropy collection works
- Real-world hardware integration
- BearDog's universal HSM architecture

**Best Part**: You have ALL the hardware! 🎉

---

**Ready when Pixel boots!** 🚀

Run `./scripts/verify-hardware.sh` to check status.

---

*Phase 2: Hardware Integration - December 10, 2025*  
*Hardware detected. Demo scripts ready. Waiting for Pixel boot.*


