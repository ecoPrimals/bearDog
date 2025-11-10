# HSM Protocol Gap Analysis - November 9, 2025

**Issue**: SoloKeys not detected via BearDog's `discover-hsm` command  
**Root Cause**: BearDog only implements PKCS#11 protocol  
**Impact**: Missing modern security key support (FIDO2/CTAP2)  
**Priority**: HIGH - Future-proofing required  

---

## 🔍 **Current State**

### ✅ What BearDog HAS:

1. **Architecture Designed** (specs/current/security/UNIVERSAL_HARDWARE_SECURITY_TOKEN_INTEGRATION.md):
   - `UniversalSecurityToken` trait defined
   - Vendor-agnostic design philosophy
   - Capability-based discovery planned

2. **Interface Types Enumerated** (`crates/beardog-tunnel/src/tunnel/hsm/universal_discovery/mod.rs`):
   ```rust
   pub enum HsmInterfaceType {
       CloudKms { provider, region },
       NetworkHsm { endpoint, port },
       UsbHsm { device_id },          // ⚠️ Defined but limited
       SoftwareHsm { implementation },
       MobileHsm { platform, chip },   // ✅ Android StrongBox implemented
       CustomApi { api_type, endpoint },
       Tpm { version },                // ⚠️ Defined but not implemented
       SmartCard { reader },           // ⚠️ PKCS#11 only
   }
   ```

3. **Working Implementations**:
   - ✅ **PKCS#11** (YubiKey PIV, traditional smart cards)
   - ✅ **Android StrongBox** (Pixel Titan M2)
   - ✅ **iOS Secure Enclave** (iPhone)
   - ✅ **Software HSM** (development/testing)
   - ✅ **Cloud KMS** (AWS, Azure, GCP)

### ❌ What BearDog LACKS:

1. **FIDO2/CTAP2 Protocol** - Modern security keys (SoloKeys, YubiKey FIDO2 mode)
2. **TPM 2.0 Direct Access** - Direct TPM communication (currently relies on PKCS#11 wrapper)
3. **U2F/WebAuthn** - Web authentication protocols
4. **OpenPGP Card** - PGP smart cards
5. **HOTP/TOTP** - Time-based one-time passwords (hardware tokens)

---

## 🎯 **The SoloKey Problem**

### What Happened:

```bash
# USB detection: ✅ Working
$ lsusb | grep Solo
Bus 001 Device 007: ID 1209:beee Generic Solo 2 Security Key
Bus 001 Device 005: ID 1209:beee Generic Solo 2 Security Key

# PC/SC detection: ✅ Working
$ pcsc_scan
Reader 0: SoloKeys Solo 2 [CCID/ICCD Interface]
Reader 1: SoloKeys Solo 2 [CCID/ICCD Interface]

# FIDO2 detection: ✅ Working
$ fido2-token -L
/dev/hidraw5: SoloKeys Solo 2 Security Key
/dev/hidraw6: SoloKeys Solo 2 Security Key

# PKCS#11 detection: ❌ FAILED
$ pkcs11-tool --list-slots
Slot 0: SoloKeys Solo 2
  (token not recognized)       <-- ❌ Not initialized for PIV

# BearDog detection: ❌ FAILED
$ ./target/release/beardog discover-hsm
❌ No HSM devices found.       <-- ❌ Only checks PKCS#11!
```

### Why It Fails:

**SoloKeys are FIDO2-primary devices**:
- Primary interface: **FIDO2/CTAP2** (for authentication)
- Secondary interface: **PIV** (requires initialization)
- PKCS#11 only works if PIV applet is initialized
- BearDog's CLI only tries PKCS#11

---

## 📊 **Protocol Support Matrix**

| Hardware Type | Primary Protocol | BearDog Support | Status |
|--------------|------------------|-----------------|--------|
| **SoloKeys Solo 2** | FIDO2/CTAP2 | ❌ | **MISSING** |
| **YubiKey 5** (PIV mode) | PKCS#11 | ✅ | Working |
| **YubiKey 5** (FIDO2 mode) | FIDO2/CTAP2 | ❌ | **MISSING** |
| **TPM 2.0 Chip** | TPM 2.0 API | ⚠️ | Via PKCS#11 wrapper only |
| **Android StrongBox** | Android Keystore | ✅ | Working |
| **iOS Secure Enclave** | iOS Security Framework | ✅ | Working |
| **Traditional Smart Cards** | PKCS#11 | ✅ | Working |
| **Nitrokey** | PKCS#11/OpenPGP | ⚠️ | PKCS#11 only |
| **OnlyKey** | FIDO2/U2F | ❌ | **MISSING** |

---

## 🚀 **Future-Proofing Requirements**

### Critical Protocols to Add:

#### 1. **FIDO2/CTAP2 Support** (HIGHEST PRIORITY)

**Why**: Modern security keys (SoloKeys, YubiKey, Titan, OnlyKey) primarily use FIDO2.

**What to implement**:
```rust
// crates/beardog-security/src/hsm/fido2_provider.rs

use fido_common::credential::public_key::PublicKeyCredential;
use ctap_2::authenticator::Authenticator;

pub struct Fido2HsmProvider {
    device_path: PathBuf,          // e.g., /dev/hidraw5
    authenticator: Authenticator,
    capabilities: Fido2Capabilities,
}

impl Fido2HsmProvider {
    /// Discover all FIDO2 devices
    pub async fn discover_devices() -> Result<Vec<Fido2HsmProvider>> {
        // Scan /dev/hidraw* for FIDO2 devices
        // Query capabilities via CTAP2
        // Return list of discovered devices
    }
    
    /// Generate entropy using hmac-secret extension
    pub async fn generate_entropy(&self, size: usize) -> Result<Vec<u8>> {
        // Use hmac-secret extension for cryptographic material
        // SoloKeys support this!
    }
    
    /// Sign data using resident key
    pub async fn sign_with_resident_key(&self, data: &[u8], key_id: &str) -> Result<Vec<u8>> {
        // Leverage FIDO2 resident keys for signing
    }
}
```

**Dependencies**:
```toml
[dependencies]
fido-common = "0.5"
ctap-2 = "0.3"
```

#### 2. **TPM 2.0 Direct Access** (HIGH PRIORITY)

**Why**: Nearly all modern PCs have TPM 2.0 chips. Currently only accessible via PKCS#11 wrapper.

**What to implement**:
```rust
// crates/beardog-security/src/hsm/tpm2_provider.rs

use tpm2_tss::*;

pub struct Tpm2HsmProvider {
    context: TpmContext,
    handles: HashMap<String, KeyHandle>,
}

impl Tpm2HsmProvider {
    /// Direct TPM 2.0 access
    pub async fn initialize() -> Result<Self> {
        let context = TpmContext::new()?;
        // Initialize TPM context
    }
    
    /// Generate hardware-backed key in TPM
    pub async fn generate_tpm_key(&self, algorithm: TpmAlgorithm) -> Result<KeyHandle> {
        // Create key in TPM NV storage
        // Return handle for future operations
    }
}
```

**Dependencies**:
```toml
[dependencies]
tss-esapi = "7.4"  # Pure Rust TPM 2.0 bindings
```

#### 3. **OpenPGP Card Protocol** (MEDIUM PRIORITY)

**Why**: Nitrokey, YubiKey, and other devices support OpenPGP cards.

**What to implement**:
```rust
// crates/beardog-security/src/hsm/openpgp_provider.rs

use openpgp_card::*;

pub struct OpenPgpCardProvider {
    card: Card,
}
```

#### 4. **WebAuthn/U2F** (MEDIUM PRIORITY)

**Why**: Web authentication integration, FIDO U2F legacy support.

---

## 🏗️ **Proposed Architecture**

### Multi-Protocol Discovery Engine:

```rust
// crates/beardog-cli/src/commands/discover_hsm.rs

pub async fn discover_all_hsm_devices() -> Result<Vec<DiscoveredHsm>> {
    let mut devices = Vec::new();
    
    // 1. Try PKCS#11 (traditional smart cards, YubiKey PIV)
    devices.extend(discover_pkcs11_devices().await?);
    
    // 2. Try FIDO2/CTAP2 (modern security keys) - NEW!
    devices.extend(discover_fido2_devices().await?);
    
    // 3. Try TPM 2.0 (platform TPM chips) - NEW!
    devices.extend(discover_tpm2_devices().await?);
    
    // 4. Try Android StrongBox (mobile devices)
    devices.extend(discover_android_strongbox().await?);
    
    // 5. Try iOS Secure Enclave (iOS devices)
    devices.extend(discover_ios_secure_enclave().await?);
    
    // 6. Try Cloud KMS (AWS, Azure, GCP)
    devices.extend(discover_cloud_kms().await?);
    
    Ok(devices)
}
```

### Unified HSM Provider Trait:

```rust
// crates/beardog-traits/src/unified/hsm.rs

#[async_trait]
pub trait UniversalHsmProvider: Send + Sync {
    /// Protocol-agnostic methods
    async fn get_capabilities(&self) -> Result<HsmCapabilities>;
    async fn generate_entropy(&self, size: usize) -> Result<Vec<u8>>;
    async fn generate_key(&self, algorithm: KeyAlgorithm) -> Result<KeyHandle>;
    async fn sign(&self, data: &[u8], key: &KeyHandle) -> Result<Vec<u8>>;
    async fn verify(&self, data: &[u8], signature: &[u8], key: &KeyHandle) -> Result<bool>;
    
    /// Protocol-specific info
    fn protocol_type(&self) -> HsmProtocolType;
    fn device_info(&self) -> DeviceInfo;
}

pub enum HsmProtocolType {
    Pkcs11,
    Fido2Ctap2,      // NEW!
    Tpm20,           // NEW!
    AndroidKeystore,
    IosSecurityFramework,
    OpenPgpCard,     // NEW!
    CloudKms,
}
```

---

## 🎯 **Implementation Plan**

### Phase 1: FIDO2/CTAP2 Support (2-3 weeks)

**Goal**: Make SoloKeys work with BearDog

**Tasks**:
1. Add `fido-common` and `ctap-2` dependencies to `beardog-security`
2. Implement `Fido2HsmProvider` struct
3. Implement device discovery via `/dev/hidraw*` scanning
4. Implement `hmac-secret` extension for entropy generation
5. Add FIDO2 to unified discovery in `beardog-cli`
6. Write integration tests with SoloKeys
7. Update documentation

**Deliverable**: `./target/release/beardog discover-hsm` detects SoloKeys!

### Phase 2: TPM 2.0 Direct Access (1-2 weeks)

**Goal**: Direct TPM chip access without PKCS#11 wrapper

**Tasks**:
1. Add `tss-esapi` dependency
2. Implement `Tpm2HsmProvider` struct
3. Add TPM detection to discovery engine
4. Test on systems with TPM 2.0 chips
5. Document TPM usage

### Phase 3: Protocol Abstraction (1 week)

**Goal**: Clean adapter pattern for all protocols

**Tasks**:
1. Refactor existing providers to implement unified trait
2. Add protocol-agnostic configuration
3. Implement automatic fallback (try multiple protocols)
4. Add protocol capability negotiation

### Phase 4: Additional Protocols (2-3 weeks)

**Goal**: OpenPGP Card, U2F, WebAuthn

**Tasks**:
1. OpenPGP Card support for Nitrokey
2. U2F legacy support
3. WebAuthn integration for browser flows

---

## 🔐 **Security Considerations**

### Current PKCS#11-only approach:

✅ **Pros**:
- Industry standard
- Well-tested
- Wide compatibility with initialized devices

❌ **Cons**:
- Misses modern FIDO2 devices
- Requires device initialization (PIV setup)
- Single point of failure (one protocol)

### Multi-protocol approach:

✅ **Pros**:
- Discovers ALL available hardware
- Future-proof (new protocols can be added)
- No initialization required for FIDO2
- Graceful fallback between protocols

⚠️ **Considerations**:
- More complex codebase
- Each protocol needs security audit
- Protocol-specific vulnerabilities
- Dependency bloat (more crates)

---

## 📋 **Immediate Actions**

### For Your Hardware Testing:

**Option A: Quick Test (Use FIDO2 tools)**

```bash
# Test SoloKeys via native FIDO2 interface
fido2-token -G -t hmac-secret /dev/hidraw5  # Generate entropy
fido2-cred -M /dev/hidraw5                  # Create credential
```

**Option B: Initialize PIV (Make PKCS#11 work)**

```bash
# Initialize PIV applet on SoloKeys
# Note: This will make BearDog's current code work!
solo2 piv init
solo2 piv set-pin
```

**Option C: Focus on Pixel 8a (Already Supported!)**

```bash
# Your Pixel has full BearDog support via StrongBox
adb devices  # (authorize the device first)
# Then deploy Android app for testing
```

### For BearDog Development:

1. **File Issue**: "Add FIDO2/CTAP2 Protocol Support for Modern Security Keys"
2. **Create Feature Branch**: `feature/fido2-hsm-support`
3. **Add Dependencies**: `fido-common`, `ctap-2`, `tss-esapi`
4. **Implement `Fido2HsmProvider`**: Based on architecture above
5. **Update Discovery**: Multi-protocol scanning in CLI
6. **Test with SoloKeys**: Your hardware is perfect for validation!

---

## ✅ **Recommendations**

### SHORT TERM (This Week):

1. **Test Pixel 8a StrongBox** - Already supported, excellent testing opportunity
2. **Document this gap** - This file serves that purpose
3. **Create GitHub issue** - Track FIDO2 implementation

### MEDIUM TERM (Next Month):

1. **Implement FIDO2/CTAP2 support** - Priority #1
2. **Add TPM 2.0 direct access** - Priority #2
3. **Refactor to multi-protocol discovery** - Cleaner architecture

### LONG TERM (Next Quarter):

1. **Full protocol matrix** - All major HSM protocols supported
2. **Automatic protocol negotiation** - Try multiple protocols, use best available
3. **Protocol capability mapping** - Abstract operations across protocols

---

## 🎯 **Conclusion**

### Is this a BearDog shortcoming?

**YES** - This is a **real architectural gap** that needs addressing.

### Should we build it for future-proofing?

**YES** - For the following reasons:

1. **Market Reality**: Modern security keys use FIDO2, not just PKCS#11
2. **User Experience**: Devices should "just work" without initialization
3. **Hardware Diversity**: TPM 2.0 chips are everywhere and underutilized
4. **Sovereignty Goal**: Max hardware compatibility = max human sovereignty
5. **Competitive**: Other projects support multiple protocols

### Is it urgent?

**MEDIUM Priority**:
- Current PKCS#11 support works for many devices (YubiKey PIV, traditional cards)
- Android/iOS mobile HSM support already excellent
- But: Modern security keys (your SoloKeys!) are increasingly common
- Trend: FIDO2 is becoming dominant protocol for consumer hardware

### Can we ship without it?

**YES, but with limitations**:
- BearDog is production-ready for PKCS#11 and mobile HSM
- The gap only affects specific hardware (FIDO2-primary devices)
- Workaround exists (initialize PIV on SoloKeys)
- But: It's not "Universal HSM" without FIDO2/TPM direct access

---

## 📝 **Next Steps**

1. ✅ Document this analysis (this file)
2. ⏳ Test Pixel 8a StrongBox (already supported!)
3. ⏳ Create GitHub issue for FIDO2 support
4. ⏳ Prototype `Fido2HsmProvider` implementation
5. ⏳ Validate with your SoloKeys hardware

---

**Status**: Gap Identified ✅  
**Priority**: HIGH (for future-proofing) ⚠️  
**Workarounds Available**: YES (PIV init, use Pixel instead) ✅  
**Recommendation**: Implement FIDO2 support in next development cycle 🚀

