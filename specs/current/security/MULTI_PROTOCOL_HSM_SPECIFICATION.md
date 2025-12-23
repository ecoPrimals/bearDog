# Multi-Protocol HSM Support Specification

**Version**: 1.0  
**Date**: November 9, 2025  
**Status**: 🚧 **IN DEVELOPMENT**  
**Priority**: HIGH - Future-Proofing Critical  
**Implementation**: Pure Rust, Protocol-Agnostic, Hardware-Agnostic  

---

## 🎯 **Executive Summary**

This specification extends BearDog's HSM capabilities beyond PKCS#11 to support **all major hardware security protocols** in a truly vendor-agnostic fashion. The goal is to make BearDog work with ANY security hardware "out of the box" without requiring device-specific initialization or configuration.

**Core Principle**: *"Any hardware that can do cryptography should work with BearDog, regardless of protocol"*

### Current State vs. Target State:

| Protocol | Current | Target | Impact |
|----------|---------|--------|--------|
| **PKCS#11** | ✅ Working | ✅ Keep | Traditional smart cards, YubiKey PIV |
| **FIDO2/CTAP2** | ❌ Missing | ✅ **NEW** | SoloKeys, modern security keys |
| **TPM 2.0** | ⚠️ Via wrapper | ✅ **Direct** | Platform TPM chips |
| **Android Keystore** | ✅ Working | ✅ Keep | Pixel StrongBox, mobile devices |
| **iOS Security Framework** | ✅ Working | ✅ Keep | iPhone Secure Enclave |
| **OpenPGP Card** | ❌ Missing | ✅ **NEW** | Nitrokey, YubiKey PGP |
| **U2F** | ❌ Missing | ✅ **NEW** | Legacy FIDO devices |
| **WebAuthn** | ❌ Missing | ✅ **NEW** | Browser authentication |

---

## 🏗️ **Architecture**

### Unified HSM Provider Trait

All HSM protocols implement a single unified interface:

```rust
/// Universal HSM provider trait - protocol agnostic
#[async_trait]
pub trait UniversalHsmProvider: Send + Sync + Debug {
    // ============================================================================
    // DISCOVERY & CAPABILITIES
    // ============================================================================
    
    /// Get provider information and capabilities
    async fn get_provider_info(&self) -> Result<ProviderInfo, BearDogError>;
    
    /// Get detailed capabilities of this HSM
    async fn get_capabilities(&self) -> Result<HsmCapabilities, BearDogError>;
    
    /// Get the underlying protocol type
    fn protocol_type(&self) -> HsmProtocolType;
    
    /// Get device information
    fn device_info(&self) -> DeviceInfo;
    
    // ============================================================================
    // ENTROPY & RANDOMNESS
    // ============================================================================
    
    /// Generate cryptographically secure random bytes
    /// This is the most universal operation - all HSMs can do this
    async fn generate_entropy(&self, size: usize) -> Result<Vec<u8>, BearDogError>;
    
    /// Test entropy quality (returns statistical metrics)
    async fn test_entropy_quality(&self, samples: usize) -> Result<EntropyQualityMetrics, BearDogError>;
    
    // ============================================================================
    // KEY MANAGEMENT
    // ============================================================================
    
    /// Generate a new key pair
    async fn generate_key(
        &self,
        algorithm: KeyAlgorithm,
        params: KeyGenerationParams,
    ) -> Result<KeyHandle, BearDogError>;
    
    /// List available keys (if supported)
    async fn list_keys(&self) -> Result<Vec<KeyInfo>, BearDogError>;
    
    /// Get key metadata
    async fn get_key_info(&self, key: &KeyHandle) -> Result<KeyInfo, BearDogError>;
    
    /// Delete a key (if supported and allowed)
    async fn delete_key(&self, key: &KeyHandle) -> Result<(), BearDogError>;
    
    // ============================================================================
    // CRYPTOGRAPHIC OPERATIONS
    // ============================================================================
    
    /// Sign data with a key
    async fn sign(
        &self,
        data: &[u8],
        key: &KeyHandle,
        algorithm: SignatureAlgorithm,
    ) -> Result<Vec<u8>, BearDogError>;
    
    /// Verify a signature (if supported)
    async fn verify(
        &self,
        data: &[u8],
        signature: &[u8],
        key: &KeyHandle,
        algorithm: SignatureAlgorithm,
    ) -> Result<bool, BearDogError>;
    
    /// Encrypt data (if supported)
    async fn encrypt(
        &self,
        plaintext: &[u8],
        key: &KeyHandle,
        algorithm: EncryptionAlgorithm,
    ) -> Result<Vec<u8>, BearDogError>;
    
    /// Decrypt data (if supported)
    async fn decrypt(
        &self,
        ciphertext: &[u8],
        key: &KeyHandle,
        algorithm: EncryptionAlgorithm,
    ) -> Result<Vec<u8>, BearDogError>;
    
    // ============================================================================
    // HUMAN INTERACTION (for FIDO2 and other interactive devices)
    // ============================================================================
    
    /// Request human presence proof (button press, touch, etc.)
    async fn require_human_presence(&self) -> Result<HumanPresenceProof, BearDogError>;
    
    /// Check if device supports human interaction
    fn supports_human_interaction(&self) -> bool;
    
    // ============================================================================
    // ATTESTATION & VERIFICATION
    // ============================================================================
    
    /// Get hardware attestation (if supported)
    async fn get_attestation(&self) -> Result<AttestationData, BearDogError>;
    
    /// Verify device is genuine hardware (not software emulation)
    async fn verify_hardware_backed(&self) -> Result<bool, BearDogError>;
    
    // ============================================================================
    // HEALTH & STATUS
    // ============================================================================
    
    /// Check if provider is healthy and operational
    async fn health_check(&self) -> Result<HealthStatus, BearDogError>;
    
    /// Get usage statistics
    async fn get_statistics(&self) -> Result<HsmStatistics, BearDogError>;
}
```

### Protocol Type Enumeration

```rust
/// HSM protocol types - extensible for future protocols
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HsmProtocolType {
    /// PKCS#11 - Traditional smart card interface
    Pkcs11 {
        library_path: String,
        slot_id: u64,
    },
    
    /// FIDO2/CTAP2 - Modern security keys (SoloKeys, YubiKey FIDO2 mode)
    Fido2Ctap2 {
        device_path: PathBuf,  // e.g., /dev/hidraw5
        transport: Fido2Transport,
    },
    
    /// TPM 2.0 - Trusted Platform Module direct access
    Tpm20 {
        device_path: Option<PathBuf>,  // e.g., /dev/tpm0
        tcti_type: TpmTctiType,
    },
    
    /// Android Keystore - Android StrongBox/TEE
    AndroidKeystore {
        security_level: AndroidSecurityLevel,
    },
    
    /// iOS Security Framework - Secure Enclave
    IosSecurityFramework {
        enclave_type: SecureEnclaveType,
    },
    
    /// OpenPGP Card - PGP smart cards
    OpenPgpCard {
        reader: String,
        card_version: String,
    },
    
    /// U2F - Legacy FIDO Universal 2nd Factor
    U2f {
        device_path: PathBuf,
    },
    
    /// WebAuthn - Web authentication API
    WebAuthn {
        rp_id: String,
        origin: String,
    },
    
    /// Cloud KMS - Cloud-based key management
    CloudKms {
        provider: CloudProvider,
        region: String,
    },
    
    /// Software HSM - Pure software implementation
    Software {
        implementation: SoftwareHsmType,
    },
    
    /// Custom protocol - For future extensions
    Custom {
        protocol_name: String,
        protocol_version: String,
    },
}
```

---

## 🔍 **Multi-Protocol Discovery**

### Discovery Engine Architecture

```rust
/// Universal HSM discovery engine
pub struct MultiProtocolDiscoveryEngine {
    /// Enabled protocols to scan
    enabled_protocols: Vec<HsmProtocolType>,
    
    /// Discovery timeout per protocol
    timeout: Duration,
    
    /// Cache of discovered devices
    device_cache: Arc<RwLock<HashMap<String, DiscoveredDevice>>>,
}

impl MultiProtocolDiscoveryEngine {
    /// Discover all HSM devices using all enabled protocols
    pub async fn discover_all(&self) -> Result<Vec<DiscoveredDevice>, BearDogError> {
        let mut devices = Vec::new();
        
        // Try each protocol in parallel
        let discovery_tasks = vec![
            tokio::spawn(self.discover_pkcs11()),
            tokio::spawn(self.discover_fido2()),
            tokio::spawn(self.discover_tpm2()),
            tokio::spawn(self.discover_android_keystore()),
            tokio::spawn(self.discover_ios_secure_enclave()),
            tokio::spawn(self.discover_openpgp_cards()),
            tokio::spawn(self.discover_cloud_kms()),
        ];
        
        // Collect results from all protocols
        for task in discovery_tasks {
            if let Ok(Ok(mut protocol_devices)) = task.await {
                devices.append(&mut protocol_devices);
            }
        }
        
        // Deduplicate (same physical device via multiple protocols)
        let deduplicated = self.deduplicate_devices(devices);
        
        // Rank by capability and preference
        let ranked = self.rank_devices(deduplicated);
        
        Ok(ranked)
    }
    
    /// Discover FIDO2/CTAP2 devices (NEW!)
    async fn discover_fido2(&self) -> Result<Vec<DiscoveredDevice>, BearDogError> {
        let mut devices = Vec::new();
        
        // Scan /dev/hidraw* for FIDO2 devices
        let hidraw_devices = self.scan_hidraw_devices()?;
        
        for device_path in hidraw_devices {
            // Try to communicate via CTAP2
            if let Ok(info) = self.probe_fido2_device(&device_path).await {
                devices.push(DiscoveredDevice {
                    protocol: HsmProtocolType::Fido2Ctap2 {
                        device_path: device_path.clone(),
                        transport: Fido2Transport::Usb,
                    },
                    device_info: info,
                    capabilities: self.query_fido2_capabilities(&device_path).await?,
                });
            }
        }
        
        Ok(devices)
    }
    
    /// Discover TPM 2.0 devices (NEW!)
    async fn discover_tpm2(&self) -> Result<Vec<DiscoveredDevice>, BearDogError> {
        // Check for /dev/tpm0, /dev/tpmrm0
        // Query TPM capabilities via tss-esapi
        todo!()
    }
    
    /// Discover OpenPGP cards (NEW!)
    async fn discover_openpgp_cards(&self) -> Result<Vec<DiscoveredDevice>, BearDogError> {
        // Scan PC/SC readers for OpenPGP card applet
        todo!()
    }
}
```

---

## 🚀 **Implementation Phases**

### Phase 1: FIDO2/CTAP2 Support (CURRENT - Week 1-3)

**Goal**: Make SoloKeys and other FIDO2 devices work with BearDog

**Tasks**:
1. ✅ Create specification (this document)
2. ⏳ Add dependencies: `fido-common`, `ctap-2`, `hidapi`
3. ⏳ Implement `Fido2HsmProvider` struct
4. ⏳ Implement device scanning (`/dev/hidraw*` on Linux, HID API on Windows/Mac)
5. ⏳ Implement CTAP2 communication protocol
6. ⏳ Implement `hmac-secret` extension for entropy generation
7. ⏳ Implement credential management for key storage
8. ⏳ Implement signature operations with resident keys
9. ⏳ Add to unified discovery engine
10. ⏳ Write comprehensive tests with SoloKeys
11. ⏳ Update CLI to show FIDO2 devices
12. ⏳ Update documentation

**Acceptance Criteria**:
- [ ] `./target/release/beardog discover-hsm` detects SoloKeys
- [ ] Can generate entropy from SoloKeys via `hmac-secret`
- [ ] Can create resident keys on SoloKeys
- [ ] Can sign data using resident keys
- [ ] Full test coverage with actual SoloKeys hardware

### Phase 2: TPM 2.0 Direct Access (Week 4-5)

**Goal**: Direct TPM chip access without PKCS#11 wrapper

**Tasks**:
1. Add dependency: `tss-esapi` (pure Rust TPM 2.0 bindings)
2. Implement `Tpm2HsmProvider` struct
3. Implement TPM device detection (`/dev/tpm0`, `/dev/tpmrm0`)
4. Implement key generation in TPM NV storage
5. Implement signing/verification with TPM keys
6. Add to unified discovery engine
7. Test on systems with TPM 2.0 chips
8. Document TPM usage

**Acceptance Criteria**:
- [ ] Detects platform TPM 2.0 chips
- [ ] Can generate keys in TPM
- [ ] Can perform crypto operations with TPM
- [ ] No dependency on PKCS#11 wrapper

### Phase 3: Protocol Abstraction & Unification (Week 6)

**Goal**: Clean adapter pattern, protocol-agnostic operations

**Tasks**:
1. Refactor existing providers to implement `UniversalHsmProvider` trait
2. Implement protocol capability negotiation
3. Implement automatic fallback (try multiple protocols)
4. Add protocol-agnostic configuration
5. Implement device deduplication (same device via multiple protocols)
6. Add protocol preference ordering
7. Update all tests to use unified interface

**Acceptance Criteria**:
- [ ] All providers implement same trait
- [ ] Can access same device via multiple protocols
- [ ] Automatic selection of best protocol
- [ ] Zero protocol-specific code in application layer

### Phase 4: Additional Protocols (Week 7-9)

**Goal**: OpenPGP Card, U2F, WebAuthn

**Tasks**:
1. OpenPGP Card support (Nitrokey, YubiKey PGP mode)
2. U2F legacy support (older FIDO devices)
3. WebAuthn integration (browser authentication flows)
4. Each protocol gets same treatment as Phase 1

**Acceptance Criteria**:
- [ ] Full OpenPGP card support
- [ ] U2F device support
- [ ] WebAuthn authentication flows
- [ ] All protocols work through unified interface

### Phase 5: Production Hardening (Week 10)

**Goal**: Production-ready, secure, audited

**Tasks**:
1. Security audit of all protocol implementations
2. Fuzz testing of protocol parsers
3. Error handling and recovery
4. Protocol-specific security validations
5. Performance optimization
6. Documentation completion
7. Example code and tutorials

---

## 🔐 **Security Considerations**

### Protocol-Specific Threats:

#### FIDO2/CTAP2:
- **Threat**: Malicious HID devices pretending to be security keys
- **Mitigation**: Validate CTAP2 responses, check attestation certificates
- **Threat**: Relay attacks (forward CTAP2 commands to remote device)
- **Mitigation**: Enforce human presence checks, timeout enforcement

#### TPM 2.0:
- **Threat**: Software TPM emulation (not hardware-backed)
- **Mitigation**: Verify TPM attestation, check device properties
- **Threat**: TPM access from unprivileged processes
- **Mitigation**: Proper permission checking, session isolation

#### OpenPGP Card:
- **Threat**: PIN bruteforce attacks
- **Mitigation**: Respect card PIN retry counters, enforce delays
- **Threat**: Card cloning attacks
- **Mitigation**: Verify card serial numbers, use attestation

### General Security Principles:

1. **Defense in Depth**: Multiple validation layers for each protocol
2. **Fail Secure**: Unknown protocols rejected, not bypassed
3. **Minimal Trust**: Each protocol implementation is sandboxed
4. **Audit Trail**: All HSM operations logged with context
5. **Rate Limiting**: Prevent DoS via excessive HSM operations

---

## 📋 **Testing Strategy**

### Hardware Testing Matrix:

| Device | Protocol | Test Status | Hardware Owner |
|--------|----------|-------------|----------------|
| **SoloKeys Solo 2** | FIDO2 | ⏳ In Progress | eastgate (2 units) |
| **Pixel 8a + GrapheneOS** | Android StrongBox | ⏳ Ready | eastgate |
| **YubiKey 5** | PKCS#11 (PIV) | ✅ Tested | (need hardware) |
| **YubiKey 5** | FIDO2 | ⏳ Pending | (need hardware) |
| **YubiKey 5** | OpenPGP | ⏳ Pending | (need hardware) |
| **TPM 2.0 (various)** | TPM 2.0 API | ⏳ Pending | eastgate (in tower) |
| **Nitrokey** | PKCS#11 | ⏳ Pending | (need hardware) |
| **Nitrokey** | OpenPGP | ⏳ Pending | (need hardware) |
| **OnlyKey** | FIDO2 | ⏳ Pending | (need hardware) |
| **Software HSM** | PKCS#11 | ✅ Tested | All systems |

### Test Categories:

1. **Discovery Tests**: Each protocol can discover compatible devices
2. **Entropy Tests**: Can generate cryptographically secure random bytes
3. **Key Generation Tests**: Can create keys with each algorithm
4. **Signature Tests**: Can sign and verify with generated keys
5. **Encryption Tests**: Can encrypt/decrypt (if supported)
6. **Attestation Tests**: Can verify hardware backing
7. **Stress Tests**: Concurrent operations, high load
8. **Failure Tests**: Handle disconnection, errors gracefully
9. **Security Tests**: Resist known attack patterns
10. **Integration Tests**: Multi-protocol fallback, device switching

---

## 📊 **Success Metrics**

### Technical Metrics:

- **Protocol Coverage**: 8+ protocols supported (PKCS#11, FIDO2, TPM, Android, iOS, OpenPGP, U2F, WebAuthn)
- **Device Compatibility**: 95%+ of commercial security hardware works out-of-box
- **Discovery Time**: < 2 seconds to discover all devices
- **Operation Latency**: < 100ms for entropy, < 500ms for signatures
- **Test Coverage**: 100% of protocol implementations
- **Zero Unsafe Code**: All implementations in safe Rust

### User Experience Metrics:

- **Plug-and-Play**: No device initialization required for FIDO2/TPM
- **Automatic Fallback**: Try multiple protocols transparently
- **Clear Errors**: Protocol-specific troubleshooting guidance
- **Consistent API**: Same code works for all protocols

### Ecosystem Impact:

- **Human Sovereignty**: Max hardware compatibility = max user freedom
- **Vendor Independence**: No lock-in to specific hardware
- **Future-Proof**: Easy to add new protocols as they emerge
- **Competitive Advantage**: Most comprehensive HSM support in Rust

---

## 🎯 **Deliverables**

### Code Artifacts:

1. `crates/beardog-security/src/hsm/fido2_provider.rs` - FIDO2/CTAP2 implementation
2. `crates/beardog-security/src/hsm/tpm2_provider.rs` - TPM 2.0 implementation
3. `crates/beardog-security/src/hsm/openpgp_provider.rs` - OpenPGP Card implementation
4. `crates/beardog-security/src/hsm/u2f_provider.rs` - U2F implementation
5. `crates/beardog-security/src/hsm/webauthn_provider.rs` - WebAuthn implementation
6. `crates/beardog-traits/src/unified/hsm.rs` - Unified HSM trait (updated)
7. `crates/beardog-cli/src/commands/discover_hsm.rs` - Multi-protocol discovery (updated)

### Documentation:

1. This specification (tracking progress)
2. Protocol-specific guides (FIDO2, TPM, OpenPGP)
3. Hardware compatibility matrix
4. Migration guide (for PKCS#11-only users)
5. Security audit report
6. Performance benchmarks

### Tests:

1. Unit tests for each protocol implementation
2. Integration tests with real hardware
3. Fuzzing tests for protocol parsers
4. Security tests for known attack patterns
5. Performance benchmarks

---

## 🔄 **Migration Path**

### For Existing Users:

**No Breaking Changes!**

- Current PKCS#11 code continues to work
- New protocols are opt-in via discovery
- Configuration remains backward compatible
- Gradual migration recommended

### For New Users:

- Use unified discovery API (automatic protocol selection)
- Protocol-agnostic code (works with any HSM)
- Modern defaults (FIDO2/TPM preferred over PKCS#11)

---

## 📖 **References**

### Standards & Specifications:

- [FIDO2 CTAP2 Specification](https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html)
- [TPM 2.0 Library Specification](https://trustedcomputinggroup.org/resource/tpm-library-specification/)
- [OpenPGP Card Specification](https://gnupg.org/ftp/specs/OpenPGP-smart-card-application-3.4.1.pdf)
- [PKCS#11 v2.40](http://docs.oasis-open.org/pkcs11/pkcs11-base/v2.40/pkcs11-base-v2.40.html)
- [WebAuthn Level 2](https://www.w3.org/TR/webauthn-2/)

### Rust Crates:

- `fido-common` - FIDO2 common types
- `ctap-2` - CTAP2 protocol implementation
- `tss-esapi` - TPM 2.0 Software Stack (pure Rust)
- `hidapi` - Cross-platform HID device access
- `openpgp-card` - OpenPGP card operations
- `pcsc` - PC/SC smart card access

---

**Status**: 🚧 IN DEVELOPMENT - Phase 1 (FIDO2) Starting  
**Target**: Production-ready multi-protocol HSM support  
**Timeline**: 10 weeks (aggressive but achievable)  
**Risk**: LOW (well-defined protocols, existing libraries)  
**Impact**: HIGH (true hardware universality)  

---

**Last Updated**: November 9, 2025  
**Next Review**: Weekly during development  
**Owner**: BearDog Security Team

