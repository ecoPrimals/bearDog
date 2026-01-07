# 🎯 Inter-Primal Communication Protocol Priority

**Date**: January 6, 2026  
**Status**: 🔴 **CLARIFICATION** - Protocol Priority for Inter-Primal Communication  
**Source**: Upstream Evolution Debt

---

## 🎯 TL;DR

**For inter-primal communication (BearDog ↔ Songbird ↔ ToadStool):**

1. **tarpc** (PRIMARY) ⭐⭐⭐⭐⭐
2. **JSON-RPC** (FALLBACK) ⭐⭐⭐⭐
3. **HTTP** (LEGACY - not for inter-primal) ⭐⭐

**Key Quote from Upstream**: *"interpriamls should be tarpc, and json rpc"*

---

## 📊 Protocol Priority Matrix

| Protocol | Priority | Use Case | Security | Type Safety | Performance |
|----------|----------|----------|----------|-------------|-------------|
| **tarpc** | **#1 PRIMARY** | **Known primals** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **JSON-RPC** | **#2 FALLBACK** | **Unknown primals** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| **HTTP** | #3 Legacy | External/legacy | ⭐⭐ | ⭐⭐ | ⭐⭐⭐ |

---

## 🏗️ Architecture Vision

### Inter-Primal Communication (Known Primals)

```
BearDog ←──tarpc (type-safe)──→ Songbird
   ↓                               ↓
   └───tarpc (type-safe)───→ ToadStool
```

**Why tarpc?**
- ✅ Type-safe (compile-time guarantees)
- ✅ Modern Rust idioms (async/await)
- ✅ Efficient (minimal overhead)
- ✅ Clear contracts (defined traits)

### Universal Adapter (Unknown Primals)

```
BearDog ←──JSON-RPC (agnostic)──→ Future Primal
   ↓                                  (unknown at build time)
   └───JSON-RPC (agnostic)──→ Custom Service
```

**Why JSON-RPC?**
- ✅ Universal compatibility
- ✅ No build-time dependencies
- ✅ Simple to implement
- ✅ Good security (better than HTTP)

### Legacy/External (Discouraged for Inter-Primal)

```
BearDog ←──HTTP (legacy)──→ External Tool
   ↓                         (monitoring, debugging)
   └───HTTP (legacy)──→ Legacy System
```

**Why HTTP?**
- ⚠️  Compatibility with legacy systems
- ⚠️  External monitoring tools
- ⚠️  NOT recommended for inter-primal

---

## 🎯 Implementation Roadmap

### Phase 1: HTTP + JSON-RPC (COMPLETE ✅)

**Status**: Implemented and tested
- ✅ JSON-RPC 2.0 (primary)
- ✅ HTTP (legacy, with warnings)
- ✅ Automatic protocol detection
- ✅ 61 tests passing

**Purpose**: Unblock Songbird immediately while we build proper inter-primal solution

### Phase 2: tarpc Integration (NEXT - HIGH PRIORITY)

**Goal**: Type-safe inter-primal communication
- [ ] Define `BearDogService` trait
- [ ] Implement tarpc server
- [ ] Create client library for Songbird
- [ ] Protocol negotiation
- [ ] Migration guide

**Timeline**: 2-3 days

### Phase 3: Deprecation Path

**Goal**: Phase out HTTP for inter-primal use
- [ ] All known primals use tarpc
- [ ] JSON-RPC for discovery/unknown
- [ ] HTTP only for external/legacy
- [ ] Document best practices

**Timeline**: 1-2 weeks

---

## 💻 Code Examples

### ✅ CORRECT: tarpc (Primary for Known Primals)

```rust
// BearDog Service Definition
#[tarpc::service]
pub trait BearDogService {
    async fn evaluate_trust(request: TrustEvaluationRequest) -> TrustEvaluationResponse;
    async fn birdsong_encrypt(plaintext: Vec<u8>, family_id: String) -> Result<Vec<u8>, String>;
    async fn birdsong_decrypt(ciphertext: Vec<u8>, family_id: String) -> Result<Vec<u8>, String>;
}

// Songbird calls BearDog
use tarpc::context;

let client = BearDogServiceClient::connect("unix:///tmp/beardog-nat0-tower1.sock").await?;
let response = client.evaluate_trust(
    context::current(),
    TrustEvaluationRequest { peer_id: "tower2", family: "nat0" }
).await?;

// ✅ Type-safe: compiler checks all fields
// ✅ Efficient: minimal overhead
// ✅ Modern: async/await throughout
```

### ✅ ACCEPTABLE: JSON-RPC (Fallback for Unknown Primals)

```rust
// BearDog receives unknown primal request
let request = json!({
    "jsonrpc": "2.0",
    "method": "evaluate_trust",
    "params": {
        "peer_id": "unknown-primal",
        "family": "nat0"
    },
    "id": 1
});

// ✅ Universal: works with any primal
// ✅ Secure: better than HTTP
// ⚠️  Not type-safe: runtime validation
```

### ⚠️ DISCOURAGED: HTTP (Legacy/External Only)

```http
GET /metrics/security HTTP/1.1
Host: unix

# ⚠️  NOT for inter-primal communication
# ⚠️  Only for: legacy systems, monitoring tools, debugging
# ⚠️  Less secure than tarpc/JSON-RPC
```

---

## 🔄 Migration Strategy

### Immediate (Phase 1 - Complete)
```
Songbird (HTTP) ──→ BearDog (JSON-RPC + HTTP)
✅ Works now with warnings
```

### Short Term (Phase 2 - Next)
```
Songbird (tarpc) ──→ BearDog (tarpc + JSON-RPC + HTTP)
✅ Type-safe inter-primal
✅ JSON-RPC fallback still works
⚠️  HTTP deprecated for inter-primal
```

### Long Term (Phase 3)
```
Songbird (tarpc) ──→ BearDog (tarpc + JSON-RPC)
✅ All known primals use tarpc
✅ JSON-RPC for unknown primals
❌ HTTP removed for inter-primal (external only)
```

---

## 📋 Decision Matrix

### Should I use tarpc or JSON-RPC?

**Use tarpc if:**
- ✅ Known primal (BearDog, Songbird, ToadStool)
- ✅ Build-time dependency acceptable
- ✅ Want type safety
- ✅ Want best performance

**Use JSON-RPC if:**
- ✅ Unknown primal (discovered at runtime)
- ✅ No build-time dependency wanted
- ✅ Universal compatibility needed
- ✅ Simple implementation desired

**Use HTTP if:**
- ⚠️  Legacy system (no choice)
- ⚠️  External monitoring tool
- ⚠️  Debugging/development only
- ❌ **NOT for inter-primal communication**

---

## 🎯 Success Criteria

### Phase 2 (tarpc) Complete When:
- [ ] BearDog has tarpc server running
- [ ] Songbird has tarpc client library
- [ ] All inter-primal calls use tarpc
- [ ] HTTP usage drops to <5% (external only)
- [ ] Type-safe contracts defined
- [ ] Performance validated (>2x faster than HTTP)

### Phase 3 (Deprecation) Complete When:
- [ ] All known primals use tarpc
- [ ] JSON-RPC for unknown primals only
- [ ] HTTP for external/legacy only
- [ ] Documentation updated
- [ ] Migration complete

---

## 📚 Why This Matters

### From Upstream Evolution Debt

**Key Insight**: "interpriamls should be tarpc, and json rpc"

**Rationale**:
1. **Type Safety**: Primals are trusted, type-safe communication prevents bugs
2. **Performance**: Inter-primal communication is frequent, efficiency matters
3. **Modern Rust**: tarpc uses modern async/await patterns
4. **Security**: Type-safe protocols are more secure than text-based HTTP

**HTTP Issues**:
- ❌ Not type-safe (runtime errors)
- ❌ More overhead (headers, parsing)
- ❌ Less secure (easier to attack)
- ❌ Not idiomatic Rust

---

## 🎊 Current Status

### ✅ Phase 1 Complete
- JSON-RPC implemented (primary)
- HTTP implemented (legacy, with warnings)
- 61 tests passing
- Ready for production

### ⏳ Phase 2 Next
- tarpc integration planned
- Will become primary for inter-primal
- JSON-RPC remains as fallback
- HTTP for external/legacy only

---

## 📞 For Developers

### I'm building a new primal, what should I use?

**Answer**: Plan for **tarpc**, support **JSON-RPC** as fallback

```rust
// Your primal should:
1. Implement tarpc client for known primals (BearDog, Songbird)
2. Implement JSON-RPC client for unknown primals
3. Avoid HTTP for inter-primal communication
```

### I'm integrating with BearDog, what should I use?

**Answer**: 
- **Now**: JSON-RPC (Phase 1, available today)
- **Soon**: tarpc (Phase 2, coming in days)
- **Avoid**: HTTP (legacy only)

### I need HTTP, is that wrong?

**Answer**: Depends on use case
- ✅ **OK**: External monitoring, debugging, legacy systems
- ❌ **NOT OK**: Inter-primal communication (use tarpc/JSON-RPC)

---

## 🎊 Summary

**Inter-Primal Communication Priority**:
1. **tarpc** (PRIMARY) - Type-safe, efficient, modern
2. **JSON-RPC** (FALLBACK) - Universal, simple, secure
3. **HTTP** (LEGACY) - External/legacy only

**Current State**: Phase 1 (JSON-RPC + HTTP) complete ✅
**Next Step**: Phase 2 (tarpc integration) - HIGH PRIORITY ⏳
**Goal**: Modern, idiomatic, type-safe inter-primal communication

---

**Key Takeaway**: Use **tarpc for known primals**, **JSON-RPC for unknown**, **avoid HTTP for inter-primal**.

**Date**: January 6, 2026  
**Status**: Clarified and documented  
**Action**: Proceed with Phase 2 (tarpc integration)

