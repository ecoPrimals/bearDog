# 🎯 TODO Evolution Plan - January 7, 2026

**Philosophy**: Deep debt solutions, not quick fixes. Complete implementations following modern idiomatic Rust.

---

## 📊 TODO Categorization (27 Total)

### Category 1: Integration & Protocol (6 TODOs) 🔴 HIGH PRIORITY

1. **tarpc server connection handling** (`unix_socket_ipc.rs:309`)
   - Current: Logs warning, not implemented
   - Evolution: Complete tarpc service integration
   - Approach: Use existing tarpc infrastructure, wire to handlers
   - Estimated: 2-3 hours

2. **BTSP trust evaluation** (`tarpc_service.rs:156`)
   - Current: Returns basic response
   - Evolution: Wire to actual BTSP provider trust logic
   - Approach: Inject BtspProvider, call real evaluation
   - Estimated: 1-2 hours

3. **Security metrics collection** (`tarpc_service.rs:199`)
   - Current: Returns zeros
   - Evolution: Wire to actual metrics from BTSP provider
   - Approach: Query real state, return actual metrics
   - Estimated: 1 hour

4. **Genetics key derivation integration** (`btsp_provider.rs:724`)
   - Current: Uses ChaCha20-Poly1305 directly
   - Evolution: Integrate beardog-genetics key derivation
   - Approach: Use EcosystemGeneticEngine for lineage-based keys
   - Estimated: 2-3 hours

5. **Family ID from manager** (`api/birdsong.rs:338,383` - 2 instances)
   - Current: Uses "default" fallback
   - Evolution: Read from environment or config
   - Approach: Use existing env var pattern (FAMILY_ID)
   - Estimated: 30 minutes

**Total Category 1**: 7-10 hours

---

### Category 2: Discovery (5 TODOs) 🟡 MEDIUM PRIORITY

6. **mDNS discovery** (`discovery/discovery.rs:211`)
7. **DNS-SD discovery** (`discovery/discovery.rs:219`)
8. **Service registry discovery** (`discovery/discovery.rs:229`)
9. **mDNS announcement** (`discovery/announcement.rs:52`)
10. **Service registry announcement** (`discovery/announcement.rs:77`)

**Evolution Strategy**:
- Current: Returns empty, graceful fallback
- Evolution: Complete implementations using:
  - `mdns` crate for mDNS/DNS-SD
  - HTTP registry client for service registry
- Approach: Agnostic, capability-based discovery
- Primal sovereignty: Discover at runtime, no hardcoding

**Estimated**: 6-8 hours for all 5

---

### Category 3: Security & Crypto (4 TODOs) 🟡 MEDIUM PRIORITY

11. **Hardware attestation verification** (`genetics/birdsong/genesis_types.rs:191`)
    - Current: Just checks non-empty
    - Evolution: Real attestation verification
    - Approach: Use cryptographic validation, chain of trust
    - Estimated: 3-4 hours

12. **HSM-backed witness list** (`genetics/birdsong/genesis.rs:360`)
    - Current: Phase 2 planned
    - Evolution: HSM-backed trusted witness verification
    - Approach: Query HSM for trusted witnesses, verify signatures
    - Estimated: 3-4 hours

13. **Real Ed25519 verification** (`security/genesis/witness.rs:236`)
    - Current: Mock verification (always succeeds)
    - Evolution: Real Ed25519 signature verification
    - Approach: Use `ed25519-dalek` crate (already dependency)
    - Estimated: 1-2 hours

14. **RSA key management** (`core/crypto_service/implementation.rs:279`)
    - Current: Generates on-demand
    - Evolution: Proper RSA key management (HSM integration)
    - Approach: Load from HSM, persist properly
    - Estimated: 2-3 hours

**Total Category 3**: 9-13 hours

---

### Category 4: Monitoring & Metrics (4 TODOs) 🟢 LOW PRIORITY

15-17. **BTSP metrics** (`api/upa_client.rs:322-324` - 3 instances)
    - active_tunnels, cpu_percent, memory_mb
    - Current: Returns zeros
    - Evolution: Real system metrics
    - Approach: Query BTSP provider state, use `sysinfo` crate
    - Estimated: 2-3 hours

18. **Heartbeat interval update** (`api/upa_client.rs:347`)
    - Current: Ignores next_interval_secs
    - Evolution: Dynamic interval adjustment
    - Approach: Update heartbeat timer on response
    - Estimated: 1 hour

**Total Category 4**: 3-4 hours

---

### Category 5: Advanced Features (6 TODOs) 🔵 FUTURE

19. **Behavioral verification** (`types/genetics_constraints.rs:588`)
    - Biometric, MFA, usage patterns
    - Estimated: 8-10 hours (requires infrastructure)

20. **Multi-signature verification** (`genetics/constraints/enforcement.rs:240`)
    - Network coordination required
    - Estimated: 6-8 hours

21. **Behavioral checks** (`genetics/constraints/enforcement.rs:249`)
    - Integration with tunnel/HSM systems
    - Estimated: 6-8 hours

22. **Key persistence** (`core/core/security_tests.rs:288`)
    - Test infrastructure improvement
    - Estimated: 2-3 hours

23. **License checking** (`core/certificates/issuer.rs:194`)
    - Phase 5: Usage metering
    - Estimated: 10-15 hours (requires metering system)

24-25. **BirdSong proofs** (`cli/handlers/birdsong.rs:334-335` - 2 instances)
    - Cryptographic relationship proofs, Merkle root
    - Estimated: 4-6 hours

26. **mDNS advertisement** (`capabilities/registry.rs:185`)
    - Phase 3 planned
    - Estimated: 2-3 hours

**Total Category 5**: 38-53 hours (future work)

---

## 🎯 Execution Plan

### Phase 1: Critical Integration (NOW) 🔴

**TODOs 1-5**: Integration & Protocol (7-10 hours)

**Priority**: Unblock core functionality
**Approach**: Wire existing systems together
**Principle**: No mocks in production

**Order**:
1. ✅ Family ID from manager (30 min) - Quick win
2. ✅ Security metrics collection (1 hour) - Wire real data
3. ✅ BTSP trust evaluation (1-2 hours) - Real implementation
4. ✅ tarpc server connection (2-3 hours) - Complete protocol
5. ✅ Genetics key derivation (2-3 hours) - Deep integration

---

### Phase 2: Discovery Infrastructure (NEXT) 🟡

**TODOs 6-10**: Discovery (6-8 hours)

**Priority**: Enable runtime discovery
**Approach**: Agnostic, capability-based
**Principle**: Primal sovereignty (no hardcoding)

**Implementation**:
- Use `mdns` crate for mDNS/DNS-SD
- HTTP client for service registry
- Environment-driven configuration
- Graceful fallback if discovery fails

---

### Phase 3: Security Hardening (WEEK 2) 🟡

**TODOs 11-14**: Security & Crypto (9-13 hours)

**Priority**: Production-grade security
**Approach**: Fast AND safe implementations
**Principle**: Zero compromise on safety

**Implementation**:
- Real cryptographic verification
- HSM integration
- Proper key management
- No mock verification in production

---

### Phase 4: Observability (WEEK 2) 🟢

**TODOs 15-18**: Monitoring & Metrics (3-4 hours)

**Priority**: Production monitoring
**Approach**: Real-time metrics
**Principle**: Observable systems

---

### Phase 5: Advanced Features (FUTURE) 🔵

**TODOs 19-26**: Advanced (38-53 hours)

**Priority**: Future enhancements
**Approach**: When infrastructure ready
**Principle**: Don't rush complexity

---

## 🔧 Implementation Principles

### 1. Deep Debt Solutions

❌ **Quick Fix**:
```rust
// TODO: Implement later
return Ok(default_value());
```

✅ **Deep Debt Solution**:
```rust
// Complete implementation with proper error handling
self.provider.evaluate_trust(peer)
    .await
    .context("Failed to evaluate trust")?
```

---

### 2. Modern Idiomatic Rust

❌ **Old Pattern**:
```rust
let result = match something {
    Some(val) => val,
    None => return Err(...)
};
```

✅ **Modern Pattern**:
```rust
let result = something.ok_or_else(|| BearDogError::NotFound)?;
```

---

### 3. No Production Mocks

❌ **Mock in Production**:
```rust
// TODO: Implement real verification
fn verify(&self) -> bool {
    true // Mock: always succeeds
}
```

✅ **Real Implementation**:
```rust
fn verify(&self, signature: &Signature, pubkey: &PublicKey) -> Result<bool> {
    pubkey.verify(self.data, signature)
        .map(|_| true)
        .or_else(|_| Ok(false))
}
```

---

### 4. Agnostic & Capability-Based

❌ **Hardcoded**:
```rust
let family = "default".to_string();
```

✅ **Environment-Driven**:
```rust
let family = std::env::var("FAMILY_ID")
    .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
    .context("FAMILY_ID not set")?;
```

---

### 5. Fast AND Safe

❌ **Unsafe for Speed**:
```rust
unsafe {
    // Faster but unsafe
    ptr::copy_nonoverlapping(src, dst, len);
}
```

✅ **Safe AND Fast**:
```rust
// Modern Rust is fast without unsafe
dst[..len].copy_from_slice(&src[..len]);
// Or use zero-copy with Arc/Cow
```

---

## 📊 Timeline

### Week 1 (This Week)
- ✅ Phase 1: Critical Integration (7-10 hours)
- ✅ Phase 2: Discovery Infrastructure (6-8 hours)
- **Total**: 13-18 hours (2-3 days)

### Week 2
- ✅ Phase 3: Security Hardening (9-13 hours)
- ✅ Phase 4: Observability (3-4 hours)
- **Total**: 12-17 hours (2-3 days)

### Future (Week 3+)
- ⏳ Phase 5: Advanced Features (as needed)
- **Total**: 38-53 hours (5-7 days spread over time)

---

## ✅ Success Criteria

### Immediate (Phase 1 & 2)
- [ ] Zero "TODO" in critical paths
- [ ] All integration points wire to real implementations
- [ ] Discovery works at runtime (no hardcoding)
- [ ] All tests pass
- [ ] No production mocks

### Short-term (Phase 3 & 4)
- [ ] Real cryptographic verification
- [ ] HSM integration complete
- [ ] Real-time metrics collection
- [ ] Production-grade security

### Long-term (Phase 5)
- [ ] Advanced features as infrastructure permits
- [ ] No rushing complexity
- [ ] Maintain deep debt principles

---

## 🎯 First Implementation: Family ID Evolution

Let's start with the easiest TODO to demonstrate the approach:

**File**: `crates/beardog-tunnel/src/api/birdsong.rs`

**Current** (lines 338, 383):
```rust
let family_id = req.family_id.clone().unwrap_or_else(|| {
    // TODO: Get node's default family_id from manager
    "default".to_string()
});
```

**Evolution** (Deep Debt Solution):
```rust
let family_id = req.family_id.clone()
    .or_else(|| {
        // Get from environment (primal self-knowledge)
        std::env::var("FAMILY_ID")
            .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
            .ok()
    })
    .ok_or_else(|| {
        BearDogError::Configuration(
            "family_id required: provide in request or set FAMILY_ID environment variable"
                .to_string()
        )
    })?;
```

**Benefits**:
- ✅ No hardcoding ("default" removed)
- ✅ Primal self-knowledge (reads own identity)
- ✅ Proper error handling (no silent fallback)
- ✅ Clear error messages
- ✅ Environment-driven

---

## 📚 Related Documents

- `COMPREHENSIVE_AUDIT_JAN_7_2026.md` - Full audit findings
- `DEEP_DEBT_EVOLUTION_JAN_6_2026.md` - Deep debt principles
- `HARDCODING_AUDIT_JAN_6_2026.md` - Hardcoding analysis

---

**Status**: 📋 **PLAN READY** - Beginning execution  
**Next**: Implement Phase 1 (Critical Integration)  
**Timeline**: Week 1 complete, Week 2 security, Future advanced

🐻 **Evolution, not quick fixes** 🛡️

