# 🎯 Deep Debt Evolution Plan - Self-Knowledge & Discovery

**Date**: January 17, 2026  
**Focus**: Architectural Debt Elimination - 10 Near-Term TODOs  
**Status**: READY TO EXECUTE

---

## 🔍 ROOT CAUSE ANALYSIS

### **The Problem: Primal Self-Knowledge Violations**

**Current State**: 10 TODOs that violate core architectural principles

**Violation Categories**:
1. **NestGate Direct Integration (5 TODOs)** - Breaks self-knowledge principle
2. **Discovery Stubs (3 TODOs)** - Protocol implementation gap
3. **Tarpc Integration (2 TODOs)** - Missing protocol handler

---

## 📊 THE 10 NEAR-TERM TODOs

### **Category 1: NestGate Integration (Self-Knowledge Violation)**

**Location**: `crates/beardog-tunnel/src/graph_security/`

```rust
// audit.rs:82
// TODO: Get actual creator info from NestGate

// audit.rs:124
// TODO: Get actual lineage from NestGate

// audit.rs:154
// TODO: Get actual usage from NestGate

// audit.rs:168
// TODO: Get actual assessment from recent validation

// permissions.rs:41
// TODO: Check collaborator list (requires NestGate integration)
```

**Problem**: Direct NestGate dependency violates "primals only have self-knowledge"

**Solution**: Use capability-based discovery pattern (already implemented!)

---

### **Category 2: Discovery Stubs (Protocol Gap)**

**Location**: `crates/beardog-core/src/primal_discovery.rs`

```rust
// Line 374
// TODO: Implement UPA client discovery

// Line 389
// TODO: Implement mDNS discovery

// Line 401
// TODO: Implement DNS-SD discovery
```

**Problem**: Discovery methods exist but return empty results

**Solution**: Wire to existing mDNS infrastructure and implement UPA client

---

### **Category 3: Tarpc Integration (Missing Handler)**

**Location**: `crates/beardog-tunnel/src/unix_socket_ipc/`

```rust
// server.rs:274
// TODO: Implement tarpc handler when tarpc support is ready

// types.rs:137
// TODO: Add tarpc magic byte detection when tarpc format is finalized
```

**Problem**: We claim "tarpc AND json-rpc first" but tarpc falls back to JSON-RPC

**Solution**: Implement real tarpc protocol handler

---

## 🎯 EXECUTION STRATEGY

### **Phase 1: NestGate → Capability Infrastructure (Priority 1)**

**Insight**: BearDog already has a complete capability-based discovery system!

**Files**:
- `crates/beardog-adapters/src/universal/primal_capability_adapter.rs`
- `crates/beardog-adapters/src/universal/primal_runtime_discovery.rs`
- `crates/beardog-core/src/primal_self_knowledge.rs`

**Pattern Already Exists**:
```rust
// Request data storage (replaces direct nestgate calls)
pub fn request_data_storage(
    &self,
    storage_config: serde_json::Value,
) -> Result<PrimalResponse> {
    let storage_capability = UniversalCapabilityType::Storage {
        characteristics: vec![
            StorageCharacteristic::Persistent,
            StorageCharacteristic::Encrypted,
        ],
    };
    
    self.send_capability_request(storage_capability, storage_config)
}
```

**Evolution**:
1. Create `UniversalCapabilityType::Collaboration` capability
2. Define `CollaborationFunction` enum (template_storage, user_auth, lineage_tracking, etc.)
3. Replace 5 NestGate TODOs with capability-based discovery
4. No hardcoding - discovers NestGate (or any primal) by capability at runtime

---

### **Phase 2: Discovery Implementation (Priority 2)**

**Existing Infrastructure**:
- `crates/beardog-discovery/` - mDNS implementation exists!
- `crates/beardog-adapters/src/universal/primal_runtime_discovery.rs` - Discovery pattern

**Evolution**:
1. **mDNS Discovery**: Wire `primal_discovery.rs` to `beardog-discovery` crate
2. **UPA Discovery**: Implement UPA registry client (JSON-RPC over Unix socket)
3. **DNS-SD Discovery**: Leverage mDNS infrastructure for DNS-SD queries

**Result**: All 3 discovery methods operational

---

### **Phase 3: Tarpc Protocol Handler (Priority 3)**

**Current State**: Protocol detection exists, handler is stub

**Files**:
- `crates/beardog-tunnel/src/unix_socket_ipc/types.rs` - Protocol enum
- `crates/beardog-tunnel/src/unix_socket_ipc/server.rs` - Handler stub

**Evolution**:
1. Define tarpc magic bytes (e.g., `0x74 0x72 0x70 0x63` = "trpc")
2. Implement tarpc frame decoder
3. Create `handle_tarpc_persistent()` method
4. Wire to existing capability handlers

**Benefit**: True "tarpc AND json-rpc first" - both protocols fully operational

---

## 🏗️ ARCHITECTURE VALIDATION

### **Capability Types We Need**

```rust
// NEW: Collaboration capability for NestGate integration
UniversalCapabilityType::Collaboration {
    functions: vec![
        CollaborationFunction::TemplateStorage,
        CollaborationFunction::UserAuthentication,
        CollaborationFunction::LineageTracking,
        CollaborationFunction::CommunityMetrics,
        CollaborationFunction::SecurityAssessment,
    ],
}
```

### **Discovery Flow (Post-Evolution)**

```
1. BearDog needs template info
   ↓
2. Query: "Who provides Collaboration::TemplateStorage?"
   ↓
3. Discovery tries:
   - mDNS: "_collaboration._tcp.local"
   - UPA: Query registry for capability
   - DNS-SD: Query domain for service
   ↓
4. Finds NestGate (or compatible primal)
   ↓
5. Connects via tarpc or JSON-RPC (primal's choice)
   ↓
6. No hardcoding, pure runtime discovery! ✅
```

---

## 📋 IMPLEMENTATION CHECKLIST

### **Phase 1: NestGate Evolution (5 TODOs)**

- [ ] Define `CollaborationFunction` enum
- [ ] Add `Collaboration` to `UniversalCapabilityType`
- [ ] Implement `request_template_info()` in capability adapter
- [ ] Implement `request_user_permissions()` in capability adapter
- [ ] Implement `request_lineage_data()` in capability adapter
- [ ] Implement `request_community_metrics()` in capability adapter
- [ ] Implement `request_security_assessment()` in capability adapter
- [ ] Replace 5 TODOs in `graph_security/` with capability calls
- [ ] Add integration tests

**Estimated Complexity**: MEDIUM (pattern already exists, just extend)

---

### **Phase 2: Discovery Implementation (3 TODOs)**

- [ ] Wire mDNS discovery to `beardog-discovery` crate
- [ ] Implement UPA registry client (JSON-RPC)
- [ ] Implement DNS-SD query wrapper
- [ ] Update `discover_from_mdns()` to call real implementation
- [ ] Update `discover_from_upa()` to call UPA client
- [ ] Update `discover_from_dns_sd()` to call DNS-SD implementation
- [ ] Add discovery integration tests
- [ ] Test multi-method discovery fallback

**Estimated Complexity**: MEDIUM (infrastructure exists, just wire)

---

### **Phase 3: Tarpc Handler (2 TODOs)**

- [ ] Define tarpc magic bytes constant
- [ ] Implement tarpc frame decoder
- [ ] Create `handle_tarpc_persistent()` method
- [ ] Wire tarpc handler to capability router
- [ ] Update protocol detection in `types.rs`
- [ ] Add tarpc protocol tests
- [ ] Test tarpc/JSON-RPC fallback

**Estimated Complexity**: MEDIUM-HIGH (new protocol handler)

---

## 🎯 SUCCESS METRICS

### **Before Evolution**
```
TODOs: 10 architectural debt items
NestGate Integration: Hardcoded (violates self-knowledge)
Discovery: 3 stub methods (return empty)
Tarpc: Falls back to JSON-RPC (not truly "tarpc first")
Protocol Support: JSON-RPC only
Self-Knowledge Violations: 5 instances
```

### **After Evolution**
```
TODOs: 3 (Ed25519 + Phase 5 cert - legitimate future work)
NestGate Integration: Capability-based (zero hardcoding) ✅
Discovery: 3 operational methods (mDNS, UPA, DNS-SD) ✅
Tarpc: Full protocol support (true "tarpc AND json-rpc") ✅
Protocol Support: tarpc + JSON-RPC (both first-class) ✅
Self-Knowledge Violations: 0 ✅
```

---

## 💡 DEEP DEBT PHILOSOPHY

### **Why This is "Deep Debt"**

These aren't just TODOs - they're **architectural violations**:

1. **Self-Knowledge Violation**: Direct NestGate calls break primal autonomy
2. **Discovery Gap**: Stub methods undermine runtime discovery principle
3. **Protocol Gap**: tarpc fallback contradicts "tarpc first" claim

### **Why This is "Near-Term"**

1. **Infrastructure Exists**: All patterns already implemented
2. **High Impact**: Fixes 5 self-knowledge violations
3. **Unlocks Features**: Graph security, collaboration, true discovery
4. **Philosophy Alignment**: Delivers on "no hardcoding" promise

---

## 🚀 IMPLEMENTATION ORDER

### **Recommended Sequence**

1. **Phase 1 First** (5 TODOs) - Highest impact, easiest (pattern exists)
2. **Phase 2 Second** (3 TODOs) - Enables real discovery
3. **Phase 3 Third** (2 TODOs) - Completes "tarpc AND json-rpc first"

**Rationale**: Phase 1 has immediate impact and proves the pattern. Phase 2 enables true runtime discovery. Phase 3 completes protocol support.

---

## 📊 REMAINING LEGITIMATE TODOs

After this evolution, only **3 TODOs** remain:

1. **Ed25519 Signature Verification** (2 instances) - Phase 5 cryptography
2. **Certificate Validation Enhancement** (1 instance) - Phase 5 PKI

These are truly "future work" (Phase 5), not architectural debt.

---

## 🎯 ESTIMATED EFFORT

**Phase 1**: 4-6 hours (extend existing pattern)
**Phase 2**: 3-5 hours (wire existing infrastructure)
**Phase 3**: 5-7 hours (new protocol handler)

**Total**: 12-18 hours of focused evolution

**ROI**: Eliminate 70% of production TODOs, fix 5 self-knowledge violations, enable true runtime discovery

---

## ✅ PHILOSOPHY DELIVERED

- ✅ **"Primals only have self-knowledge"** - NestGate via capabilities
- ✅ **"Discover at runtime, never hardcode"** - 3 discovery methods operational
- ✅ **"tarpc AND json-rpc first"** - Both protocols fully supported
- ✅ **"Deep debt solutions"** - Fix root cause, not symptoms

---

## 🔥 READY TO EXECUTE

**All Prerequisites Met**:
- ✅ Capability infrastructure exists
- ✅ Discovery patterns proven
- ✅ Protocol detection implemented
- ✅ Test infrastructure ready

**Blocker Status**: ZERO BLOCKERS

**Recommendation**: **PROCEED TO EXECUTE** 🚀

---

**Document**: `DEEP_DEBT_EXECUTION_PLAN_JAN_17_2026.md`  
**Status**: READY FOR IMPLEMENTATION  
**Priority**: HIGH (architectural violations)  
**Philosophy**: "vendor locks are vendor problems, primals have self-knowledge"

*Let's evolve beyond architectural debt to true primal autonomy!* 🐻🦀✨

