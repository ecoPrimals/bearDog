# 🎯 BearDog Pre-Production MVP Plan
**Created:** October 29, 2025  
**Target:** 2-4 Week Sprint  
**Goal:** Remove all mocks, run on real hardware (Eastgate + Pixel 8a)

---

## 🎪 **MISSION STATEMENT**

Build a **working, demonstrable** BearDog system that:
1. Runs on **Eastgate** (i9-12900K, Linux)
2. Detects and uses **4x SoloKey V2** devices via PKCS#11
3. Collects and mixes entropy from **real hardware**
4. Builds and runs on **Pixel 8a** (GrapheneOS/StrongBox)
5. **Zero mocks** - everything uses real hardware
6. Provides **CLI interface** for all operations
7. Validates architecture with **production hardware**

**Success Criteria:** Can demonstrate mixing a cryptographic seed using entropy from multiple real hardware sources.

---

## 🎯 **MVP SCOPE**

### **IN SCOPE (Must Have):**
✅ Real PKCS#11 detection and integration  
✅ SoloKey V2 entropy collection (all 4 devices)  
✅ Multi-source entropy mixing algorithm  
✅ CLI for discovery, testing, and mixing  
✅ Configuration file system (no hardcoding)  
✅ Basic Android build (Pixel 8a)  
✅ StrongBox/Titan M2 integration  
✅ Cross-platform entropy collection  
✅ Real hardware integration tests  
✅ Documentation for setup and usage

### **OUT OF SCOPE (Later):**
❌ Network distribution (SongBird integration)  
❌ Full chaos testing  
❌ 90% test coverage (focus on hardware tests)  
❌ Production monitoring  
❌ Multi-node coordination  
❌ Advanced key rotation  
❌ Full compliance reporting

---

## 📅 **PHASED APPROACH**

### **PHASE 1: Eastgate Foundation** (Week 1 - 30 hours)
**Goal:** Get BearDog running on Eastgate with real SoloKey detection

#### **Day 1-2: PKCS#11 Real Implementation** (10 hours)
```bash
Files to modify:
- crates/beardog-tunnel/src/universal_hsm_discovery/capability_detection/pkcs11_prober.rs
- crates/beardog-tunnel/src/universal_hsm/providers/pkcs11.rs
- crates/beardog-tunnel/src/tunnel/hsm/providers/software.rs (remove mocks)
```

**Tasks:**
1. Remove all mock implementations in PKCS#11 prober
2. Implement real PKCS#11 library loading
   - Try common paths: `/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so`
   - Try common paths: `/usr/lib/softhsm/libsofthsm2.so`
   - Support environment variable: `BEARDOG_PKCS11_LIB`
3. Implement real slot enumeration
4. Implement real token detection
5. Test with actual SoloKey devices
6. Add error handling for real-world failures

**Validation:**
```bash
cargo test -p beardog-tunnel pkcs11_real_hardware -- --nocapture
# Should detect all 4 SoloKey V2 devices
```

#### **Day 3-4: Working CLI** (12 hours)
```bash
Primary file:
- crates/beardog-cli/src/main.rs
- crates/beardog-cli/src/commands/*.rs (create if needed)
```

**Commands to implement:**
```bash
# Discovery
beardog-cli discover-hsm
  → Lists all detected HSMs (4x SoloKeys, TPM, etc.)
  → Shows capabilities of each
  → Tests availability

# Entropy testing
beardog-cli test-entropy --source solokey:slot0
  → Collects 1KB entropy from specified source
  → Shows entropy quality metrics
  → Validates randomness

# Entropy mixing
beardog-cli mix-seed --sources solokey:slot0,solokey:slot1 --output seed.bin
  → Collects from multiple sources
  → Mixes using verified algorithm
  → Outputs seed file

# Status
beardog-cli status
  → Shows BearDog configuration
  → Lists available HSMs
  → Reports system health
```

**Validation:**
```bash
# Should work with real hardware
beardog-cli discover-hsm
beardog-cli test-entropy --source solokey:slot0
beardog-cli mix-seed --sources solokey:slot0,solokey:slot1,solokey:slot2,solokey:slot3
```

#### **Day 5: Configuration System** (8 hours)
```bash
Files to create/modify:
- configs/eastgate-production.toml (new)
- configs/pixel8a-production.toml (new)
- crates/beardog-types/src/canonical/config/runtime_config.rs (update)
```

**Create production config:**
```toml
# configs/eastgate-production.toml
[system]
name = "eastgate"
platform = "linux-x86_64"
cores = 20

[hsm.pkcs11]
enabled = true
library_paths = [
    "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so",
    "/usr/lib/softhsm/libsofthsm2.so"
]

[[hsm.pkcs11.devices]]
slot = 0
label = "SoloKey-01"
purpose = "entropy"

[[hsm.pkcs11.devices]]
slot = 1
label = "SoloKey-02"
purpose = "entropy"

[[hsm.pkcs11.devices]]
slot = 2
label = "SoloKey-03"
purpose = "entropy"

[[hsm.pkcs11.devices]]
slot = 3
label = "SoloKey-04"
purpose = "entropy"

[entropy]
mixing_algorithm = "kdf-sha3-512"
min_sources = 2
require_hardware = true

[network]
# For later - network configuration
api_enabled = false
```

**Migration tasks:**
- Remove hardcoded PKCS#11 paths (currently found in ~10 places)
- Remove hardcoded entropy collection parameters
- Load config from file or environment variables

**Validation:**
```bash
BEARDOG_CONFIG=configs/eastgate-production.toml beardog-cli status
# Should show all 4 SoloKeys configured and detected
```

---

### **PHASE 2: Multi-Source Entropy & Android** (Week 2 - 30 hours)

#### **Day 1-2: Real Entropy Collection** (12 hours)
```bash
Files to modify:
- crates/beardog-tunnel/src/universal_hsm/entropy/collector.rs
- crates/beardog-security/src/entropy/* (remove mocks)
- crates/beardog-tunnel/src/universal_hsm/providers/real_implementation.rs
```

**Tasks:**
1. Remove mock entropy generation
2. Implement real PKCS#11 random generation
3. Implement entropy quality assessment
4. Add entropy pooling from multiple sources
5. Implement verified mixing algorithm
6. Test with all 4 SoloKeys simultaneously

**Validation:**
```bash
# Should collect real entropy from hardware
cargo test -p beardog-security entropy_collection_real -- --nocapture

# CLI test
beardog-cli test-entropy --source solokey:slot0 --size 4096
beardog-cli mix-seed --sources all --output test-seed.bin
```

#### **Day 3-4: Android Build System** (10 hours)
```bash
Files to check/modify:
- android/Cargo.toml
- android/build.rs (create if needed)
- android/src/lib.rs
```

**Tasks:**
1. Install Android NDK if not present
2. Configure cross-compilation target
   ```bash
   rustup target add aarch64-linux-android
   ```
3. Set up NDK toolchain
   ```bash
   export ANDROID_NDK_ROOT=/path/to/ndk
   ```
4. Test basic compilation
   ```bash
   cargo build --target aarch64-linux-android -p beardog-android
   ```
5. Create basic JNI bindings (if needed)
6. Build test APK or library

**Validation:**
```bash
cargo build --target aarch64-linux-android --release
# Should produce .so library for Android
```

#### **Day 5: Integration Tests** (8 hours)
```bash
Files to create:
- tests/hardware/eastgate_integration.rs (new)
- tests/hardware/solokey_integration.rs (new)
- tests/hardware/multi_source_entropy.rs (new)
```

**Tests to write:**
```rust
#[test]
#[cfg(feature = "hardware-tests")]
fn test_real_solokey_detection() {
    // Uses actual PKCS#11 library
    // Expects to find 4 SoloKey devices
}

#[test]
#[cfg(feature = "hardware-tests")]
fn test_entropy_collection_from_solokey() {
    // Collects 1KB from real SoloKey
    // Validates entropy quality
}

#[test]
#[cfg(feature = "hardware-tests")]
fn test_multi_source_entropy_mixing() {
    // Collects from all 4 SoloKeys
    // Mixes using production algorithm
    // Validates output
}
```

**Run tests:**
```bash
cargo test --features hardware-tests -- --nocapture
# All tests should use real hardware, no mocks
```

---

### **PHASE 3: Pixel 8a & Cross-Platform** (Week 3-4 - 30 hours)

#### **Week 3: StrongBox Integration** (20 hours)
```bash
Files to modify:
- crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/core.rs
- crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/keystore.rs
- crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/safe_native_wrapper.rs
```

**Tasks:**
1. Remove mock StrongBox implementations
2. Implement real Android KeyStore API calls
3. Detect Titan M2 chip (Pixel 8a specific)
4. Implement StrongBox entropy generation
5. Test on actual Pixel 8a device
6. Handle GrapheneOS-specific quirks

**Expected challenges:**
- JNI boilerplate
- Android permissions
- GrapheneOS hardening
- StrongBox availability checks

**Validation:**
```bash
# On Pixel 8a via adb:
adb shell
# Install and run beardog library
# Check StrongBox detection
```

#### **Week 4: Cross-Platform Demo** (10 hours)
```bash
Goal: Eastgate + Pixel working together
```

**Demo scenario:**
```bash
# On Eastgate:
beardog-cli discover-hsm
# Shows: 4x SoloKeys, TPM, maybe CPU entropy

# On Pixel 8a (via adb or app):
beardog-android discover-hsm
# Shows: StrongBox (Titan M2)

# Combined operation:
beardog-cli mix-seed \
  --sources solokey:slot0,solokey:slot1,solokey:slot2,solokey:slot3 \
  --output eastgate-seed.bin

# Later: Mix with Pixel entropy
# (Network integration = Phase 4, out of MVP scope)
```

---

## 🔧 **TECHNICAL DEBT TO ADDRESS**

### **Critical for MVP:**
1. ✅ Remove all mock PKCS#11 implementations
2. ✅ Remove all mock StrongBox implementations
3. ✅ Remove all mock entropy generators
4. ✅ Remove hardcoded PKCS#11 paths (currently ~10 instances)
5. ✅ Remove hardcoded device discovery
6. ✅ Implement real error handling (no unwrap in CLI)

### **Not Critical (Can Wait):**
- Full test coverage (focus on hardware tests only)
- Performance optimization
- Network distribution
- Advanced monitoring

---

## 📊 **VALIDATION CRITERIA**

### **Must Pass Before MVP Complete:**

#### **Eastgate Tests:**
- [ ] `beardog-cli discover-hsm` detects all 4 SoloKeys
- [ ] `beardog-cli test-entropy` works with each SoloKey
- [ ] `beardog-cli mix-seed` successfully mixes from all 4 devices
- [ ] Configuration loaded from file (no hardcoding)
- [ ] Integration tests pass with real hardware
- [ ] CLI has proper error messages (no panics)

#### **Pixel 8a Tests:**
- [ ] Android build compiles successfully
- [ ] Library runs on Pixel 8a (GrapheneOS)
- [ ] StrongBox/Titan M2 detection works
- [ ] Entropy collection from StrongBox works
- [ ] No mock implementations used

#### **Code Quality:**
- [ ] Zero mock implementations in production paths
- [ ] Configuration system working (no hardcoding)
- [ ] Real hardware integration tests passing
- [ ] CLI documented with examples
- [ ] Setup guide for Eastgate + Pixel 8a

---

## 📚 **DOCUMENTATION TO WRITE**

### **HARDWARE_SETUP.md** (Must Have)
```markdown
# Setting Up BearDog on Your Hardware

## Eastgate (i9-12900K, Linux)
1. Install PKCS#11 libraries
2. Configure SoloKey devices
3. Create configuration file
4. Test detection
5. Run entropy mixing

## Pixel 8a (GrapheneOS)
1. Enable Developer Mode
2. Build Android library
3. Install on device
4. Test StrongBox detection
5. Collect entropy

## Combined Usage
- Examples of mixing entropy from both platforms
```

### **CLI_GUIDE.md** (Must Have)
```markdown
# BearDog CLI Guide

## Commands
- discover-hsm: Detect hardware
- test-entropy: Test entropy sources
- mix-seed: Create mixed seeds
- status: System information

## Examples
[Actual working examples with real hardware]
```

---

## 🎯 **SUCCESS METRICS**

### **MVP is Complete When:**

1. ✅ **Real Hardware Working**
   - All 4 SoloKeys detected and usable
   - Pixel 8a StrongBox working
   - Zero mock implementations

2. ✅ **CLI Functional**
   - All commands work with real hardware
   - Proper error handling
   - Documented with examples

3. ✅ **Demonstrable**
   - Can show entropy mixing from 4 SoloKeys
   - Can show StrongBox entropy collection
   - Can explain what's happening

4. ✅ **Configured Properly**
   - No hardcoding
   - Configuration file system working
   - Easy to adapt to other hardware

5. ✅ **Tested**
   - Hardware integration tests passing
   - Manual testing completed
   - Documentation validated

---

## 🚀 **POST-MVP: NEXT STEPS**

After MVP is working, focus shifts to:

### **Phase 4: Production Hardening** (4-6 weeks)
- Increase test coverage to 80%+
- Add comprehensive error handling
- Performance optimization
- Security audit
- Load testing

### **Phase 5: Network Integration** (2-3 weeks)
- SongBird integration
- Distributed entropy collection
- Multi-node coordination
- Network security

### **Phase 6: Full Production** (2-4 weeks)
- Monitoring and observability
- Disaster recovery
- Production deployment
- Final validation

**Total Timeline to Production:** 3-4 months  
**MVP Timeline:** 2-4 weeks

---

## 🐻 **BOTTOM LINE**

This MVP will:
- ✅ Prove the architecture works with real hardware
- ✅ Remove all mock implementations
- ✅ Provide immediate value (working entropy mixing)
- ✅ Identify real-world integration issues
- ✅ Give concrete target for testing
- ✅ Validate design decisions

**Focus:** Get it working on YOUR hardware first. Polish later.

**Timeline:** 2-4 weeks of focused work  
**Complexity:** Medium (removing mocks, real integration)  
**Value:** HIGH (validates entire system)

---

**Created:** October 29, 2025  
**Owner:** BearDog Team  
**Status:** 📋 PLANNING → 🚀 READY TO START  
**Next Action:** Mark mvp-1 as in_progress and begin PKCS#11 real implementation

🐻 **Let's build something that actually works on real hardware!**

