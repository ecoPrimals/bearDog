# 🔐 Hardware Testing Setup Guide - Pixel 8a + SoloKey

**Date**: January 25, 2026  
**Hardware**: Pixel 8a (Android StrongBox) + SoloKey (PKCS#11)  
**Status**: ✅ **BearDog has EXCELLENT hardware support!**

---

## 🎯 WHAT YOU HAVE

### 1. Pixel 8a 📱
**Capabilities**:
- ✅ **Android StrongBox** - Hardware-backed keystore (Titan M2 chip)
- ✅ **TEE (Trusted Execution Environment)**
- ✅ Hardware-backed key generation (ECDSA, RSA, AES)
- ✅ Secure key storage (keys never leave chip)
- ✅ Attestation support

**BearDog Support**: ✅ **Extensive!**
- `crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/` (15+ modules)
- `pixel8_setup.rs` - **Pixel 8-specific optimizations!**
- `safe_android_provider.rs` - Safe Android crypto operations
- `attestation.rs` - Hardware attestation
- `device_info.rs` - Device capability detection

### 2. SoloKey 🔑
**Capabilities**:
- ✅ **FIDO2 / U2F** authentication
- ✅ **PKCS#11** interface (with OpenSC)
- ✅ Hardware entropy generation
- ✅ Secure key storage

**BearDog Support**: ✅ **Ready!**
- `hardware_pkcs11_tests.rs` - Comprehensive test suite (10 tests)
- PKCS#11 client implementation
- Entropy collection from hardware

---

## 🚀 SETUP GUIDE

### Step 1: Install Prerequisites

#### For SoloKey (PKCS#11):
```bash
# Install OpenSC for PKCS#11 support
sudo apt update
sudo apt install opensc pcscd pcsc-tools

# Start PCSC daemon
sudo systemctl start pcscd
sudo systemctl enable pcscd

# Verify SoloKey is detected
pcsc_scan
# Should show your SoloKey

# Test with OpenSC
pkcs11-tool --module /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so --list-slots
```

#### For Pixel 8a (Android):
```bash
# Install Android platform tools
sudo apt install android-tools-adb android-tools-fastboot

# Enable USB debugging on Pixel 8a:
# Settings → About Phone → tap Build Number 7 times
# Settings → System → Developer Options → USB Debugging ON

# Connect Pixel 8a via USB and authorize
adb devices
# Should show: XXXXXX device

# Verify StrongBox availability
adb shell getprop ro.hardware.keystore
# Should show: trusty (indicates StrongBox support)
```

---

## 🧪 RUNNING HARDWARE TESTS

### Option 1: SoloKey PKCS#11 Tests

```bash
cd /home/eastgate/Development/ecoPrimals/phase1/beardog

# Enable the hardware test file
mv crates/beardog-tunnel/tests/hardware_pkcs11_tests.rs.disabled \
   crates/beardog-tunnel/tests/hardware_pkcs11_tests.rs

# Set environment variables
export BEARDOG_HARDWARE_TESTS=1
export BEARDOG_PKCS11_LIB="/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so"

# Run hardware tests
cargo test --test hardware_pkcs11_tests -- --ignored --nocapture

# Or run specific test
cargo test --test hardware_pkcs11_tests test_list_devices -- --ignored --nocapture
```

**Expected Output**:
```
📚 Testing with library: /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so
✅ PKCS#11 initialized successfully
✅ Found 1 device(s)
  Device #1
    Slot ID: 0
    Label: SoloKey
    Manufacturer: SoloKeys
    Model: ...
    Serial: ...
```

---

### Option 2: Pixel 8a Android StrongBox Tests

BearDog has extensive Android support but needs to be compiled for Android target:

#### Quick Test (via ADB):
```bash
# Check if StrongBox is available
adb shell getprop ro.hardware.keystore

# Check security patch level
adb shell getprop ro.build.version.security_patch

# List available keystores
adb shell pm list packages | grep keystore
```

#### Compile for Android (Advanced):
```bash
# Add Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi

# Install Android NDK
# (instructions depend on your setup)

# Build for Android
cargo build --target aarch64-linux-android --release

# Push to device
adb push target/aarch64-linux-android/release/beardog /data/local/tmp/
adb shell chmod +x /data/local/tmp/beardog

# Run tests on device
adb shell /data/local/tmp/beardog --test
```

---

## 📋 AVAILABLE HARDWARE TESTS

### SoloKey Tests (10 tests):

1. **test_pkcs11_client_initialization** - Basic PKCS#11 setup
2. **test_list_devices** - Enumerate connected devices
3. **test_entropy_collection_basic** - Get random bytes from hardware
4. **test_entropy_collection_various_sizes** - Test different buffer sizes
5. **test_multiple_device_collection** - Handle multiple devices
6. **test_entropy_quality_distribution** - Verify entropy quality
7. **test_concurrent_access** - Parallel device access
8. **test_error_handling_invalid_slot** - Error scenarios
9. **test_reinitialize_after_finalize** - Cleanup and re-init
10. **test_full_workflow_discovery_to_entropy** - End-to-end workflow

---

## 🎯 RECOMMENDED TESTING WORKFLOW

### Phase 1: Basic Verification (15 minutes)

1. **Verify SoloKey Detection**:
```bash
pcsc_scan
pkcs11-tool --module /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so --list-slots
```

2. **Run Basic Tests**:
```bash
export BEARDOG_HARDWARE_TESTS=1
cargo test --test hardware_pkcs11_tests test_list_devices -- --ignored --nocapture
```

3. **Test Entropy Collection**:
```bash
cargo test --test hardware_pkcs11_tests test_entropy_collection_basic -- --ignored --nocapture
```

---

### Phase 2: Comprehensive Testing (30 minutes)

```bash
# Run all hardware tests
cargo test --test hardware_pkcs11_tests -- --ignored --nocapture

# Check results:
# - All 10 tests should pass
# - SoloKey should be detected
# - Entropy should be collected successfully
```

---

### Phase 3: Android StrongBox (Future)

```bash
# Verify Pixel 8a capabilities
adb shell getprop ro.hardware.keystore
adb shell getprop ro.build.version.security_patch

# Check for Titan M2 chip
adb shell getprop ro.hardware.titan

# Compile and deploy BearDog for Android
# (requires Android NDK setup)
```

---

## 🔍 TROUBLESHOOTING

### SoloKey Not Detected:
```bash
# Check USB connection
lsusb | grep -i solo

# Restart PCSC daemon
sudo systemctl restart pcscd

# Check permissions
sudo usermod -a -G pcscd $USER
# Logout and login again
```

### PKCS#11 Library Not Found:
```bash
# Find your PKCS#11 library
find /usr -name "*pkcs11*.so" 2>/dev/null

# Common locations:
# /usr/lib/x86_64-linux-gnu/opensc-pkcs11.so
# /usr/lib/softhsm/libsofthsm2.so
# /usr/local/lib/libpkcs11.so

# Set environment variable:
export BEARDOG_PKCS11_LIB="/path/to/your/pkcs11.so"
```

### Android ADB Issues:
```bash
# Kill and restart ADB
adb kill-server
adb start-server

# Re-authorize device
adb devices
# If "unauthorized", check phone for authorization prompt
```

---

## 📊 EXPECTED BENEFITS

### With Hardware Testing:
- ✅ **Real HSM validation** - Test actual hardware behavior
- ✅ **Entropy quality** - Verify true hardware randomness
- ✅ **Performance metrics** - Measure hardware crypto speed
- ✅ **Compatibility** - Ensure BearDog works with real devices
- ✅ **Production confidence** - Tests match deployment

### Coverage Impact:
- Current: 72%
- With hardware tests enabled: ~73-74% (+1-2%)
- Tests: 540 → 550 (+10 hardware tests)

---

## 🎓 PIXEL 8A FEATURES

Your Pixel 8a has **exceptional** security hardware:

### Titan M2 Security Chip:
- ✅ **StrongBox Keymaster** - Hardware-backed crypto
- ✅ **Secure element** - Keys never in memory
- ✅ **Tamper detection** - Physical attack protection
- ✅ **Verified boot** - Attestation chain

### BearDog Integration:
BearDog's `pixel8_setup.rs` has specific optimizations:
- Pixel 8 device detection
- StrongBox capability queries
- Titan M2-specific features
- Performance tuning

---

## 🚀 QUICK START COMMANDS

```bash
# 1. Setup (one-time)
sudo apt install opensc pcscd pcsc-tools
sudo systemctl start pcscd

# 2. Verify SoloKey
pcsc_scan

# 3. Enable tests
mv crates/beardog-tunnel/tests/hardware_pkcs11_tests.rs.disabled \
   crates/beardog-tunnel/tests/hardware_pkcs11_tests.rs

# 4. Run tests
export BEARDOG_HARDWARE_TESTS=1
cargo test --test hardware_pkcs11_tests -- --ignored --nocapture

# 5. Success! 🎉
```

---

## ✨ SUMMARY

**Your Hardware**: ⭐ **EXCELLENT** for BearDog testing!

| Device | Support | Status | Tests Available |
|--------|---------|--------|-----------------|
| **Pixel 8a** | ✅ Extensive | Ready | Android module (15+ files) |
| **SoloKey** | ✅ Full | Ready | 10 comprehensive tests |

**Next Steps**:
1. Install OpenSC/PCSC
2. Enable hardware tests (rename .disabled file)
3. Run tests with `BEARDOG_HARDWARE_TESTS=1`
4. Report results!

**Impact**: Real hardware testing will validate BearDog's production-ready HSM support and increase confidence for deployment!

🐻🐕 **BearDog: Hardware-ready! Your Pixel 8a + SoloKey = Perfect test setup!** ✨

