# Multi-Protocol HSM Implementation Tracker

**Started**: November 9, 2025  
**Target Completion**: January 2026 (10 weeks)  
**Current Phase**: Phase 1 - FIDO2/CTAP2 Support  
**Status**: 🚧 **IN PROGRESS**

---

## 🎯 **Overview**

BearDog is evolving from PKCS#11-only HSM support to **universal multi-protocol** support. This tracker monitors progress toward true hardware-agnostic cryptographic operations.

**Specification**: `specs/current/security/MULTI_PROTOCOL_HSM_SPECIFICATION.md`  
**Analysis**: `docs/investigations/HSM_PROTOCOL_GAP_ANALYSIS_NOV_9_2025.md`

---

## 📊 **Progress Summary**

| Phase | Protocol | Status | Progress | Target Date |
|-------|----------|--------|----------|-------------|
| **Phase 1** | FIDO2/CTAP2 | 🚧 In Progress | 5% | Nov 30, 2025 |
| **Phase 2** | TPM 2.0 | ⏳ Pending | 0% | Dec 14, 2025 |
| **Phase 3** | Abstraction | ⏳ Pending | 0% | Dec 21, 2025 |
| **Phase 4** | OpenPGP/U2F/WebAuthn | ⏳ Pending | 0% | Jan 11, 2026 |
| **Phase 5** | Production Hardening | ⏳ Pending | 0% | Jan 18, 2026 |

**Overall Progress**: 🔵🔵⚪⚪⚪⚪⚪⚪⚪⚪ **5% Complete**

---

## 📋 **Phase 1: FIDO2/CTAP2 Support (Current)**

### Week 1-3 (Nov 9 - Nov 30, 2025)

#### ✅ Completed Tasks:

- [x] **Nov 9**: Identified gap during hardware testing (SoloKeys not detected)
- [x] **Nov 9**: Created comprehensive analysis document
- [x] **Nov 9**: Created specification document
- [x] **Nov 9**: Created this tracker

#### ⏳ In Progress Tasks:

- [ ] **Nov 9**: Add Rust dependencies (`fido-common`, `ctap-2`, `hidapi`)
- [ ] **Nov 9**: Create `crates/beardog-security/src/hsm/fido2_provider.rs` module skeleton
- [ ] **Nov 10**: Implement HID device scanning for FIDO2 devices
- [ ] **Nov 10**: Implement basic CTAP2 communication

#### 🎯 Upcoming Tasks:

- [ ] **Nov 11**: Implement `hmac-secret` extension for entropy generation
- [ ] **Nov 12**: Test entropy generation with SoloKeys hardware
- [ ] **Nov 13**: Implement credential management (resident keys)
- [ ] **Nov 14**: Implement signature operations
- [ ] **Nov 15**: Add FIDO2 to unified discovery engine
- [ ] **Nov 16-17**: Weekend: Integration testing with SoloKeys
- [ ] **Nov 18-20**: Comprehensive test suite
- [ ] **Nov 21-22**: Documentation and examples
- [ ] **Nov 23-24**: Code review and refinement
- [ ] **Nov 25-30**: Buffer for unexpected issues

### Acceptance Criteria (Phase 1):

- [ ] `./target/release/beardog discover-hsm` detects both SoloKeys
- [ ] Can generate 256+ bytes of entropy from each SoloKey
- [ ] Entropy passes NIST randomness tests
- [ ] Can create resident keys on SoloKeys
- [ ] Can sign data using resident keys
- [ ] Can verify signatures from SoloKeys
- [ ] Full test coverage (unit + integration)
- [ ] Documentation complete
- [ ] Zero unsafe code
- [ ] Works on Linux, macOS, Windows

---

## 📋 **Phase 2: TPM 2.0 Direct Access**

### Week 4-5 (Dec 1 - Dec 14, 2025)

#### Tasks:

- [ ] Add `tss-esapi` dependency
- [ ] Create `crates/beardog-security/src/hsm/tpm2_provider.rs`
- [ ] Implement TPM device detection (`/dev/tpm0`, `/dev/tpmrm0`)
- [ ] Implement TPM context initialization
- [ ] Implement key generation in TPM NV storage
- [ ] Implement signing/verification with TPM keys
- [ ] Test on systems with TPM 2.0 chips (eastgate's tower)
- [ ] Add to unified discovery engine
- [ ] Documentation

### Acceptance Criteria (Phase 2):

- [ ] Detects platform TPM 2.0 chips
- [ ] Can generate keys in TPM
- [ ] Can perform crypto operations with TPM
- [ ] No dependency on PKCS#11 wrapper
- [ ] Test coverage 100%

---

## 📋 **Phase 3: Protocol Abstraction & Unification**

### Week 6 (Dec 15 - Dec 21, 2025)

#### Tasks:

- [ ] Refactor all providers to implement `UniversalHsmProvider` trait
- [ ] Implement protocol capability negotiation
- [ ] Implement automatic fallback between protocols
- [ ] Implement device deduplication (same physical device, multiple protocols)
- [ ] Add protocol preference ordering
- [ ] Update all tests to use unified interface
- [ ] Protocol-agnostic configuration

### Acceptance Criteria (Phase 3):

- [ ] All providers implement same trait
- [ ] Can access same device via multiple protocols
- [ ] Automatic selection of best protocol
- [ ] Zero protocol-specific code in application layer
- [ ] Seamless fallback on protocol failure

---

## 📋 **Phase 4: Additional Protocols**

### Week 7-9 (Dec 22, 2025 - Jan 11, 2026)

#### OpenPGP Card Support:

- [ ] Add `openpgp-card` dependency
- [ ] Create `crates/beardog-security/src/hsm/openpgp_provider.rs`
- [ ] Implement card detection via PC/SC
- [ ] Implement PGP operations (sign, decrypt, authenticate)
- [ ] Test with Nitrokey/YubiKey in OpenPGP mode

#### U2F Support:

- [ ] Implement U2F protocol (legacy FIDO)
- [ ] Create `crates/beardog-security/src/hsm/u2f_provider.rs`
- [ ] Test with older FIDO devices

#### WebAuthn Support:

- [ ] Implement WebAuthn client API
- [ ] Create `crates/beardog-security/src/hsm/webauthn_provider.rs`
- [ ] Test browser authentication flows

### Acceptance Criteria (Phase 4):

- [ ] Full OpenPGP card support
- [ ] U2F device support
- [ ] WebAuthn authentication flows
- [ ] All protocols work through unified interface

---

## 📋 **Phase 5: Production Hardening**

### Week 10 (Jan 12 - Jan 18, 2026)

#### Tasks:

- [ ] Security audit of all protocol implementations
- [ ] Fuzz testing of protocol parsers
- [ ] Error handling and recovery testing
- [ ] Protocol-specific security validations
- [ ] Performance optimization and benchmarking
- [ ] Documentation completion
- [ ] Example code and tutorials
- [ ] Migration guide for existing users
- [ ] Release notes and announcement

### Acceptance Criteria (Phase 5):

- [ ] Security audit complete (no critical findings)
- [ ] Fuzz testing 24+ hours without crashes
- [ ] All error paths tested
- [ ] Performance benchmarks documented
- [ ] Documentation review complete
- [ ] Examples tested on all platforms
- [ ] Ready for production use

---

## 🔧 **Current Sprint (Nov 9-15, 2025)**

### This Week's Goals:

1. ✅ Create specification and tracking documents
2. 🚧 Set up FIDO2 infrastructure (dependencies, module structure)
3. 🚧 Implement basic FIDO2 device discovery
4. 🚧 Implement entropy generation via `hmac-secret`
5. 🎯 Test with SoloKeys hardware

### Daily Progress:

#### **Saturday, Nov 9, 2025**:

**Morning Session**:
- ✅ Identified gap during hardware testing with SoloKeys
- ✅ Created `HSM_PROTOCOL_GAP_ANALYSIS_NOV_9_2025.md`
- ✅ Created `MULTI_PROTOCOL_HSM_SPECIFICATION.md`
- ✅ Created this tracker document
- 🚧 Starting implementation work...

**Afternoon Session** (Current):
- ⏳ Adding FIDO2 dependencies to `Cargo.toml`
- ⏳ Creating module structure
- ⏳ Implementing basic device scanning

**Evening Goal**:
- 🎯 Have basic FIDO2 device detection working
- 🎯 Detect SoloKeys via HID API

---

## 🧪 **Hardware Testing Status**

### Available Hardware:

| Device | Quantity | Protocol Support | Status |
|--------|----------|------------------|--------|
| **SoloKeys Solo 2** | 2 | FIDO2, PIV (uninit) | ✅ Connected |
| **Pixel 8a + GrapheneOS** | 1 | StrongBox | ✅ Connected |
| **TPM 2.0** | 1 | TPM 2.0 API | ✅ In tower |
| **YubiKey 5** | 0 | PKCS#11, FIDO2, OpenPGP | ❌ Need hardware |
| **Nitrokey** | 0 | PKCS#11, OpenPGP | ❌ Need hardware |

### Test Results:

#### SoloKeys Solo 2 (2 units):
- ✅ USB detection working (`lsusb`)
- ✅ PC/SC detection working (`pcsc_scan`)
- ✅ FIDO2 detection working (`fido2-token -L`)
- ❌ BearDog detection **FAILED** (PKCS#11 only)
- 📊 Both devices show `hmac-secret` extension support
- 📊 FIDO 2.1 PRE compliant
- 🎯 **Primary test target for Phase 1**

#### Pixel 8a + GrapheneOS:
- ✅ ADB detection working
- ⏳ Waiting for USB debugging authorization
- ✅ StrongBox feature confirmed via `pm list features`
- 🎯 **Test target for Android StrongBox validation**

---

## 📦 **Dependencies Status**

### Added:

- ⏳ `fido-common` - Pending
- ⏳ `ctap-2` - Pending
- ⏳ `hidapi` - Pending

### Planned (Future Phases):

- `tss-esapi` (Phase 2 - TPM 2.0)
- `openpgp-card` (Phase 4 - OpenPGP)
- `webauthn-rs` (Phase 4 - WebAuthn)

---

## 🎯 **Success Metrics**

### Technical Targets:

- [ ] 8+ protocols supported
- [ ] < 2 seconds discovery time
- [ ] < 100ms entropy generation
- [ ] < 500ms signature operations
- [ ] 100% test coverage
- [ ] Zero unsafe code

### Hardware Compatibility:

- [ ] SoloKeys Solo 2 ✅ (test hardware available)
- [ ] YubiKey 5 (all modes)
- [ ] Nitrokey (PKCS#11 + OpenPGP)
- [ ] OnlyKey
- [ ] Pixel StrongBox ✅ (test hardware available)
- [ ] TPM 2.0 chips ✅ (test hardware available)
- [ ] Software HSMs ✅ (already working)

---

## 🚨 **Blockers & Risks**

### Current Blockers:

- None (starting fresh implementation)

### Identified Risks:

1. **Protocol Complexity**: FIDO2/CTAP2 is complex, lots of edge cases
   - **Mitigation**: Use well-tested `ctap-2` crate, comprehensive testing

2. **Hardware Availability**: Need more test hardware (YubiKey, Nitrokey)
   - **Mitigation**: Phase 1 uses available SoloKeys, other phases when hardware acquired

3. **Platform Differences**: HID access differs on Linux/Windows/macOS
   - **Mitigation**: Use `hidapi` crate (cross-platform abstraction)

4. **TPM Permissions**: TPM access may require root on some systems
   - **Mitigation**: Document permission requirements, provide udev rules

---

## 📝 **Notes & Decisions**

### Architecture Decisions:

1. **Unified Trait**: All protocols implement `UniversalHsmProvider` trait
   - **Rationale**: Protocol-agnostic application code, easy testing

2. **Parallel Discovery**: Scan all protocols in parallel
   - **Rationale**: Fast discovery, better UX

3. **Device Deduplication**: Handle same device via multiple protocols
   - **Rationale**: Avoid confusing users with duplicate devices

4. **Protocol Preference**: FIDO2 > TPM > PKCS#11 for discovery
   - **Rationale**: Modern protocols first, fallback to legacy

### Implementation Decisions:

1. **Pure Rust**: Use Rust crates, avoid C FFI where possible
   - **Rationale**: Memory safety, better error handling

2. **Async/Await**: All HSM operations are async
   - **Rationale**: Non-blocking, better for network/cloud HSMs

3. **Zero Unsafe**: Maintain BearDog's zero-unsafe-code policy
   - **Rationale**: Security-critical code must be memory-safe

---

## 🎉 **Milestones**

- [ ] **Milestone 1**: SoloKeys detected by BearDog (Nov 15, 2025)
- [ ] **Milestone 2**: Entropy generation from SoloKeys (Nov 20, 2025)
- [ ] **Milestone 3**: TPM 2.0 direct access (Dec 14, 2025)
- [ ] **Milestone 4**: All protocols unified (Dec 21, 2025)
- [ ] **Milestone 5**: Production-ready (Jan 18, 2026)

---

## 📞 **Contacts & Resources**

### Documentation:
- Specification: `specs/current/security/MULTI_PROTOCOL_HSM_SPECIFICATION.md`
- Analysis: `docs/investigations/HSM_PROTOCOL_GAP_ANALYSIS_NOV_9_2025.md`
- Hardware Setup: `docs/setup/HARDWARE_SETUP.md`
- Testing Plan: `HARDWARE_TESTING_PLAN_NOV_9_2025.md`

### External Resources:
- FIDO Alliance: https://fidoalliance.org/specifications/
- TPM 2.0 Specs: https://trustedcomputinggroup.org/
- SoloKeys Docs: https://github.com/solokeys/solo2
- tss-esapi Docs: https://docs.rs/tss-esapi/

---

**Last Updated**: November 9, 2025 - 4:30 PM  
**Next Update**: Daily during active development  
**Status**: 🚧 Phase 1 in progress, implementing FIDO2 support

