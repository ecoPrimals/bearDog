# TODO/FIXME Markers Inventory - January 31, 2026

**Document Version**: 1.0  
**Date**: January 31, 2026  
**Total Markers**: 23  
**Status**: Analyzed & Prioritized  

---

## 📊 Summary by Priority

| Priority | Count | Status |
|----------|-------|--------|
| P0 (CRITICAL) | 7 | Needs implementation |
| P1 (HIGH) | 8 | Integration work |
| P2 (MEDIUM) | 5 | Improvements |
| P3 (LOW) | 3 | Documentation/cleanup |

---

## 🔴 P0 (CRITICAL) - Implementation Gaps

### 1. **Android StrongBox JNI Calls** (2 instances)

**File**: `crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/safe_android_provider.rs`

**Lines**: 329, 380

**Issue**:
```rust
// TODO: Implement actual Android StrongBox JNI call
```

**Context**: Placeholder stubs for JNI integration with Android KeyStore

**Action Required**: Full JNI implementation for Android platform
- Requires Android NDK integration
- JNI bindings to `android.security.keystore` APIs
- Platform-specific `#[cfg(target_os = "android")]` code

**Estimated Effort**: 8-12 hours (complex JNI work)

**Blocker**: Android-specific, requires Android NDK setup

---

### 2. **FIDO2 CTAP2 Protocol Implementation** (5 instances)

**File**: `crates/beardog-security/src/hsm/fido2/provider.rs`

**Lines**: 160, 201, 232, 259

**Issues**:
```rust
// TODO: Implement CTAP2 hmac-secret entropy generation
// TODO: Implement CTAP2 makeCredential command
// TODO: Implement CTAP2 getAssertion command (2x)
```

**File**: `crates/beardog-security/src/hsm/fido2/discovery.rs`

**Line**: 122
```rust
// TODO: Query actual capabilities via CTAP2 getInfo command
```

**Context**: FIDO2/WebAuthn protocol implementation (CTAP 2.0 spec)

**Action Required**:
1. Implement CTAP 2.0 protocol commands:
   - `authenticatorGetInfo` (0x04) - Device capabilities
   - `authenticatorMakeCredential` (0x01) - Create new credential
   - `authenticatorGetAssertion` (0x02) - Sign assertion
   - `authenticatorClientPIN` (0x06) - hmac-secret extension
2. CBOR encoding/decoding for CTAP messages
3. USB HID transport layer integration

**Estimated Effort**: 40-60 hours (full CTAP2 protocol)

**Dependencies**:
- CTAP 2.0 specification (W3C standard)
- CBOR serialization (`serde_cbor` or `ciborium`)
- USB HID communication (via `beardog-hid`)

**Recommendation**: This is a large, well-defined spec. Should be tackled as a dedicated multi-week feature.

---

## 🟠 P1 (HIGH) - Integration Work

### 3. **UniversalPrimalAdapter Integration** (5 instances)

**File**: `crates/beardog-tunnel/src/graph_security/collaboration_service.rs`

**Lines**: 47, 59, 72, 84, 97

**Issue**:
```rust
// TODO: Integrate UniversalPrimalAdapter when beardog-adapters is stable
```

**Context**: Placeholder for runtime primal discovery via adapters

**Action Required**:
✅ **READY NOW** - `beardog-adapters` is stable (we just fixed it!)

**Implementation**:
```rust
use beardog_adapters::UniversalPrimalAdapter;

pub struct CollaborationService {
    primal_adapter: Arc<UniversalPrimalAdapter>,
}

impl CollaborationService {
    pub async fn discover_songbird(&self) -> Result<SongbirdCapability, BearDogError> {
        self.primal_adapter.discover_capability(CapabilityType::Network).await
    }
}
```

**Estimated Effort**: 2-3 hours

**Status**: ✅ **READY TO IMPLEMENT** (beardog-adapters is stable)

---

### 4. **beardog-discovery Crate Integration** (3 instances)

**File**: `crates/beardog-ipc/src/lib.rs`

**Line**: 98
```rust
// 2. TODO: Discovery via beardog-discovery crate (when available)
```

**File**: `crates/beardog-core/src/primal_discovery.rs`

**Lines**: 548, 623
```rust
// TODO: Integrate beardog-discovery crate when available
// TODO: Complete beardog-discovery crate integration (in progress)
```

**Context**: Cross-primal discovery (mDNS, capability queries)

**Action Required**:
1. Check if `beardog-discovery` crate exists
2. If yes: integrate it
3. If no: create it (mDNS-based discovery)

**Estimated Effort**: 
- If exists: 3-4 hours (integration)
- If not: 20-30 hours (create + integrate)

**Status**: Needs investigation

---

## 🟡 P2 (MEDIUM) - Improvements

### 5. **Ed25519 Signature Verification**

**File**: `crates/beardog-tunnel/src/graph_security/audit.rs`

**Line**: 159
```rust
// TODO: Verify Ed25519 signature against modifier's public key
```

**Context**: Graph audit trail verification

**Action Required**:
```rust
use ed25519_dalek::{Verifier, Signature, VerifyingKey};

fn verify_audit_signature(
    public_key: &[u8; 32],
    message: &[u8],
    signature: &[u8; 64],
) -> Result<(), BearDogError> {
    let verifying_key = VerifyingKey::from_bytes(public_key)?;
    let sig = Signature::from_bytes(signature);
    verifying_key.verify(message, &sig)?;
    Ok(())
}
```

**Estimated Effort**: 1 hour

**Status**: ✅ **EASY WIN** - Can implement immediately

---

### 6. **Field-by-Field Config Merging**

**File**: `crates/beardog-config/src/hierarchy.rs`

**Line**: 220
```rust
// TODO: Implement field-by-field merging for partial overrides
```

**Context**: Configuration hierarchy (allow partial overrides)

**Action Required**: Implement `serde` deserialize_with for smart merging

**Estimated Effort**: 3-4 hours

**Status**: Improvement, not blocking

---

### 7. **Get Creator's Public Key**

**File**: `crates/beardog-tunnel/src/graph_security/validate.rs`

**Line**: 199
```rust
// TODO: Get creator's public key via collaboration capability
```

**Context**: Graph validation needs public key lookup

**Action Required**: Use UniversalPrimalAdapter to query creator's key

**Estimated Effort**: 2 hours (blocked by TODO #3)

**Status**: Blocked by UniversalPrimalAdapter integration

---

### 8. **Audit Assessment**

**File**: `crates/beardog-tunnel/src/graph_security/audit.rs`

**Line**: 211
```rust
// TODO: Get actual assessment from recent validation
```

**Context**: Security assessment in audit trail

**Action Required**: Query validation service for assessment

**Estimated Effort**: 2 hours

---

### 9. **Debug Port Configuration**

**File**: `crates/beardog-types/src/constants/domains/network.rs`

**Line**: 158
```rust
/// **TODO**: Add `debug_port` to NetworkConfig for full hierarchy support
```

**Context**: Documentation note

**Action Required**: Add `debug_port` field to NetworkConfig struct

**Estimated Effort**: 30 minutes

**Status**: ✅ **TRIVIAL** - Can do immediately

---

## ⚪ P3 (LOW) - Documentation/Cleanup

### 10. **Deprecation Markers**

**File**: `crates/beardog-types/src/canonical/config/network.rs`

**Line**: 106
```rust
/// **TODO**: Deprecate in favor of direct `BEARDOG_CONFIG` usage
```

**Context**: Old config API

**Action Required**: Add `#[deprecated]` attribute

**Estimated Effort**: 15 minutes

**Status**: ✅ **TRIVIAL**

---

### 11. **Certificate Phase 5 TODO**

**File**: `crates/beardog-core/src/certificates/issuer.rs`

**Line**: 265
```rust
// Phase 5 TODO:
```

**Context**: Incomplete comment (no details)

**Action Required**: Investigate what Phase 5 TODO was supposed to be, or remove

**Estimated Effort**: 15 minutes (investigation)

---

### 12. **Android StrongBox Integration Note**

**File**: `crates/beardog-hid/src/lib.rs`

**Line**: 144
```rust
// TODO: Integrate with existing Android StrongBox code
```

**Context**: Documentation note for future Android HID integration

**Action Required**: Cross-reference with TODO #1 (StrongBox JNI)

**Estimated Effort**: N/A (documentation)

---

## 🎯 Immediate Action Items

### **Can Do Now** (5-6 hours total)

1. ✅ **Ed25519 signature verification** (1 hour) - P2
2. ✅ **Add debug_port to NetworkConfig** (30 min) - P2
3. ✅ **Add deprecation attribute** (15 min) - P3
4. ✅ **Investigate Phase 5 comment** (15 min) - P3
5. ✅ **Integrate UniversalPrimalAdapter** (3 hours) - P1 (HIGH IMPACT)

### **Requires Investigation** (3-4 hours)

1. ❓ **beardog-discovery crate** - Does it exist? If yes, integrate.

### **Major Projects** (weeks of work)

1. 🔴 **FIDO2 CTAP2 Implementation** (40-60 hours) - Dedicated feature
2. 🔴 **Android StrongBox JNI** (8-12 hours) - Platform-specific

---

## 📊 Execution Priority

### **Phase 1: Quick Wins** (Now - 6 hours)
- Ed25519 verification
- NetworkConfig debug_port
- Deprecation attributes
- Clean up incomplete comments
- UniversalPrimalAdapter integration

### **Phase 2: Integration** (Next - 4 hours)
- Investigate beardog-discovery
- Public key lookup via collaboration
- Audit assessment

### **Phase 3: Major Features** (Future sprints)
- FIDO2 CTAP2 protocol (dedic feature)
- Android StrongBox JNI (Android sprint)
- Field-by-field config merging

---

## ✅ Success Criteria

- [ ] All P3 (LOW) items resolved
- [ ] All P2 (MEDIUM) items resolved or tracked as issues
- [ ] P1 (HIGH) items integrated or blocked with clear owner
- [ ] P0 (CRITICAL) items tracked as major features with milestones

---

**Date**: January 31, 2026  
**Status**: Analyzed & Ready for Execution  
**Next**: Execute Phase 1 (Quick Wins)

---

**🦀 CLEAR ACTION ITEMS, NO AMBIGUITY! 🚀**
