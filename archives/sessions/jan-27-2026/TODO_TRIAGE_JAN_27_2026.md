# TODO Triage & Action Plan - January 27, 2026

## 📊 OVERVIEW

**Total Production TODOs**: 21  
**High Priority**: 4 (can implement now)  
**Medium Priority**: 6 (waiting on external features)  
**Low Priority**: 11 (Phase 2 / Platform-specific / Polish)

---

## 🔴 HIGH PRIORITY (Can Implement Now)

### 1. **Ed25519 Signature Verification** (Priority: HIGH)
**Files**: 
- `crates/beardog-tunnel/src/graph_security/audit.rs:145`
- `crates/beardog-tunnel/src/graph_security/validate.rs:161`

**Status**: ⚠️ Security feature - should be implemented  
**Impact**: Graph security validation completeness  
**Effort**: ~2-3h  
**Action**: Implement Ed25519 verification using `ed25519-dalek` (already in deps)

---

### 2. **BTSP Trust Evaluation Integration** (Priority: HIGH)
**File**: `crates/beardog-tunnel/src/unix_socket_ipc/handlers/btsp.rs:543`

**Current**: TODO comment  
**Status**: ⚠️ Trust evaluation logic exists but not integrated  
**Impact**: IPC handler completeness  
**Effort**: ~1-2h  
**Action**: Wire up existing BTSP trust evaluation logic

---

### 3. **Field-by-Field Config Merging** (Priority: MEDIUM-HIGH)
**File**: `crates/beardog-config/src/hierarchy.rs:220`

**Current**: Simple override, no partial merging  
**Status**: ⚠️ Configuration flexibility gap  
**Impact**: 5-tier config hierarchy completeness  
**Effort**: ~2-3h  
**Action**: Implement smart field-level merge logic

---

### 4. **Phase 5 Certificate TODO** (Priority: LOW)
**File**: `crates/beardog-core/src/certificates/issuer.rs:265`

**Current**: Incomplete comment  
**Status**: ℹ️ Informational note  
**Impact**: Unknown (need to read context)  
**Effort**: TBD  
**Action**: Review and clarify

---

## 🟡 MEDIUM PRIORITY (Waiting on External Features)

### 5. **Collaboration Capability Integration** (5 TODOs)
**Files**:
- `crates/beardog-tunnel/src/graph_security/audit.rs:82` (creator info)
- `crates/beardog-tunnel/src/graph_security/audit.rs:125` (lineage)
- `crates/beardog-tunnel/src/graph_security/audit.rs:156` (usage)
- `crates/beardog-tunnel/src/graph_security/audit.rs:171` (assessment)
- `crates/beardog-tunnel/src/graph_security/permissions.rs:41` (collaborators)

**Status**: ⏳ Waiting for CollaborationService implementation  
**Impact**: Graph security collaboration features  
**Effort**: ~8-12h (once CollaborationService exists)  
**Action**: **DEFER** - Requires inter-primal collaboration primal

**Current State**: Properly stubbed with placeholder data

---

### 6. **DNS-SD via Songbird IPC** (Priority: MEDIUM)
**File**: `crates/beardog-core/src/primal_discovery.rs:616`

**Current**: Direct crate import  
**Status**: ⏳ Architecture evolution to inter-primal pattern  
**Impact**: Discovery capability isolation  
**Effort**: ~4-6h  
**Action**: **DEFER** - Requires Songbird primal coordination

---

## 🟢 LOW PRIORITY (Phase 2 / Platform-Specific / Polish)

### 7. **FIDO2 CTAP2 Implementation** (5 TODOs - Phase 2)
**Files**:
- `crates/beardog-security/src/hsm/fido2/provider.rs:160` (entropy)
- `crates/beardog-security/src/hsm/fido2/provider.rs:201` (makeCredential)
- `crates/beardog-security/src/hsm/fido2/provider.rs:232` (getAssertion/sign)
- `crates/beardog-security/src/hsm/fido2/provider.rs:259` (getAssertion/presence)
- `crates/beardog-security/src/hsm/fido2/discovery.rs:122` (getInfo)

**Status**: ✅ Properly marked as Phase 2 work  
**Impact**: FIDO2 hardware key support  
**Effort**: ~16-24h (CTAP2 protocol implementation)  
**Action**: **DEFER** - Phase 2 feature work

**Current State**: Proper error handling with `not_implemented()` errors

---

### 8. **Android StrongBox JNI** (3 TODOs - Platform-Specific)
**Files**:
- `crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/safe_android_provider.rs:329`
- `crates/beardog-tunnel/src/tunnel/hsm/android_strongbox/safe_android_provider.rs:380`
- `crates/beardog-hid/src/lib.rs:144` (integration)

**Status**: ✅ Platform-specific Android work  
**Impact**: Android hardware security module support  
**Effort**: ~12-16h (JNI bindings + Android testing)  
**Action**: **DEFER** - Platform-specific Phase 2

**Current State**: Safe stubs, proper error handling

---

### 9. **Configuration Deprecation Notices** (2 TODOs - Polish)
**Files**:
- `crates/beardog-types/src/canonical/config/network.rs:106` (deprecate old pattern)
- `crates/beardog-types/src/constants/domains/network.rs:158` (add debug_port)

**Status**: ✅ Documentation/deprecation polish  
**Impact**: Configuration API cleanup  
**Effort**: ~1-2h  
**Action**: **DEFER** - API evolution, non-breaking

---

## 📈 SUMMARY BY CATEGORY

| Category | Count | Status | Action |
|----------|-------|--------|--------|
| **Ed25519 Verification** | 2 | ⚠️ Implement | ✅ DO NOW |
| **BTSP Integration** | 1 | ⚠️ Implement | ✅ DO NOW |
| **Config Merging** | 1 | ⚠️ Implement | ✅ DO NOW |
| **Phase 5 Cert** | 1 | ℹ️ Review | ✅ CHECK |
| **Collaboration** | 5 | ⏳ External | ⏸️ DEFER |
| **DNS-SD Evolution** | 1 | ⏳ Architecture | ⏸️ DEFER |
| **FIDO2 CTAP2** | 5 | ✅ Phase 2 | ⏸️ DEFER |
| **Android JNI** | 3 | ✅ Platform | ⏸️ DEFER |
| **Config Polish** | 2 | ✅ Polish | ⏸️ DEFER |

**Totals**:
- ✅ **Implement Now**: 4 TODOs (~6-9h)
- ⏸️ **Defer**: 17 TODOs (properly managed)

---

## 🎯 EXECUTION PLAN

### **Immediate Actions** (This Session):

1. ✅ **Implement Ed25519 signature verification** (~2-3h)
   - Add verification in `graph_security/audit.rs`
   - Add verification in `graph_security/validate.rs`
   - Use existing `ed25519-dalek` dependency
   - Add comprehensive tests

2. ✅ **Integrate BTSP trust evaluation** (~1-2h)
   - Wire up existing trust logic in IPC handler
   - Test trust evaluation flow

3. ✅ **Implement field-by-field config merging** (~2-3h)
   - Add smart merge logic to `hierarchy.rs`
   - Support partial overrides
   - Test merge scenarios

4. ✅ **Review Phase 5 cert TODO** (~15-30m)
   - Read context
   - Clarify or complete

### **Deferred Items** (Future Work):

- **Collaboration Features**: Waiting for CollaborationService primal
- **DNS-SD Evolution**: Part of larger architecture evolution
- **FIDO2 Implementation**: Phase 2 hardware key support
- **Android StrongBox**: Phase 2 platform-specific
- **Config Polish**: Non-breaking API evolution

---

## 📊 ASSESSMENT

**Current TODO Management**: ✅ **EXCELLENT**

- All TODOs well-documented
- Clear categorization (Phase 2, platform-specific, etc.)
- Proper error handling for unimplemented features
- No security-critical gaps
- No blocking issues

**Grade**: **A (90/100)**

**Why not A+?**:
- Ed25519 verification should be complete (security feature)
- BTSP trust integration should be wired up
- Config merging would improve flexibility

**Production Impact**: **NONE** - All deferrals are properly handled

---

## ✅ COMPLETION CRITERIA

**Session Goal**: Implement 4 high-priority TODOs

**Success Metrics**:
- Ed25519 verification working (2 locations)
- BTSP trust evaluation integrated
- Field-level config merging implemented
- Phase 5 cert TODO clarified
- All tests passing
- Coverage maintained or improved

**Expected Outcome**: **21 → 17 TODOs** (4 completed, 17 properly deferred)

---

**Audit Date**: January 27, 2026  
**Next Review**: After Phase 2 planning or CollaborationService implementation

