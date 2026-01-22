# TLS Signature Algorithms Implementation Plan

**Date**: January 22, 2026  
**From**: BearDog Team  
**To**: Songbird Team, biomeOS Team  
**Status**: 🟢 **APPROVED** - Starting Phase 1 implementation  
**Priority**: 🔴 **CRITICAL** - Unblocks 65%+ of HTTPS servers

---

## 🎯 Executive Response

**We accept this handoff!** Songbird's analysis is excellent and aligns perfectly with BearDog's Pure Rust philosophy.

### Current State Verified ✅

**BearDog Already Has**:
- ✅ p256 v0.13 with ECDSA (Cargo.toml line 100)
- ✅ rsa v0.9 (Cargo.toml line 99)
- ✅ sha2 v0.10 for hashing (Cargo.toml line 95)
- ✅ ed25519-dalek v2.1 (Cargo.toml line 87)

**What We Need to Add**:
- ❌ p384 crate (Pure Rust ECDSA P-384)
- ❌ ed448-goldilocks crate (Pure Rust Ed448)
- ❌ Handler implementations for all algorithms

---

## 📊 Pure Rust Verification

All proposed libraries are **100% Pure Rust** ✅:

### ECDSA Libraries (RustCrypto)
- **p256** v0.13 - ✅ Already in Cargo.toml
  - Pure Rust ECDSA P-256 (secp256r1)
  - Zero C dependencies
  - Production-ready, widely used
  
- **p384** v0.13 - ⏳ Need to add
  - Pure Rust ECDSA P-384 (secp384r1)
  - Same quality as p256
  - Zero C dependencies

- **p521** v0.1 - ⏳ Need to add (Phase 2)
  - Pure Rust ECDSA P-521 (secp521r1)
  - Less mature, but stable
  - Zero C dependencies

### EdDSA Libraries
- **ed25519-dalek** v2.1 - ✅ Already in Cargo.toml
  - Pure Rust Ed25519
  - Zero C dependencies
  - Currently used in BearDog

- **ed448-goldilocks** v0.9 - ⏳ Need to add
  - Pure Rust Ed448
  - Zero C dependencies
  - Production-ready

### RSA Libraries (RustCrypto)
- **rsa** v0.9 - ✅ Already in Cargo.toml
  - Pure Rust RSA (PKCS#1 v1.5, PSS)
  - Zero C dependencies
  - Production-ready

**Conclusion**: All libraries meet BearDog's Pure Rust requirement! 🎉

---

## 🚀 4-Phase Implementation Plan

### Phase 1: ECDSA P-256 (CRITICAL) 🔴

**Timeline**: 1-2 days  
**Priority**: CRITICAL (blocks 65% of HTTPS servers)  
**Impact**: Unblocks GitHub, CloudFlare, Google

**Deliverables**:
1. ✅ Dependencies: p256 already in Cargo.toml
2. ⏳ Create `crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers_ecdsa.rs`
3. ⏳ Implement `crypto.sign_ecdsa_secp256r1`
4. ⏳ Implement `crypto.verify_ecdsa_secp256r1`
5. ⏳ Add routes to `handlers/crypto.rs`
6. ⏳ Unit tests (5 tests minimum)
7. ⏳ Update `docs/BEARDOG_RPC_API.md`
8. ⏳ Integration test with Songbird

**Success Criteria**:
- ✅ 5+ unit tests passing
- ✅ Songbird can negotiate ECDSA P-256
- ✅ Real HTTPS handshake with GitHub works

---

### Phase 2: ECDSA P-384/P-521 (HIGH) 🟡

**Timeline**: 1-2 days  
**Priority**: HIGH (adds high-security server support)  
**Impact**: Government, defense, financial servers

**Deliverables**:
1. ⏳ Add p384 v0.13 to Cargo.toml
2. ⏳ Implement `crypto.sign_ecdsa_secp384r1` / `crypto.verify_ecdsa_secp384r1`
3. ⏳ Add p521 (experimental) to Cargo.toml
4. ⏳ Implement `crypto.sign_ecdsa_secp521r1` / `crypto.verify_ecdsa_secp521r1`
5. ⏳ Unit tests (10 tests total)
6. ⏳ Update documentation

**Success Criteria**:
- ✅ 10+ unit tests passing (P-256 + P-384 + P-521)
- ✅ Songbird can negotiate all ECDSA curves
- ✅ 90%+ server compatibility

---

### Phase 3: Ed448 (MEDIUM) 🟢

**Timeline**: 1 day  
**Priority**: MEDIUM (ultra-secure modern servers)  
**Impact**: Let's Encrypt (some), modern CDNs

**Deliverables**:
1. ⏳ Add ed448-goldilocks v0.9 to Cargo.toml
2. ⏳ Create `crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers_eddsa.rs`
3. ⏳ Implement `crypto.sign_ed448`
4. ⏳ Implement `crypto.verify_ed448`
5. ⏳ Unit tests (5 tests)
6. ⏳ Update documentation

**Success Criteria**:
- ✅ 5+ unit tests passing
- ✅ Songbird can negotiate Ed448
- ✅ Compatible with Ed448-enabled servers

---

### Phase 4: RSA (LOW) ⚪

**Timeline**: 2-3 days  
**Priority**: LOW (legacy, but 30% of servers need it)  
**Impact**: Legacy enterprise, AWS, internal servers

**Deliverables**:
1. ✅ Dependencies: rsa already in Cargo.toml
2. ⏳ Create `crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers_rsa.rs`
3. ⏳ Implement RSA PKCS#1 v1.5 (SHA-256, SHA-384, SHA-512)
4. ⏳ Implement RSA-PSS (SHA-256, SHA-384, SHA-512)
5. ⏳ Unit tests (12 tests)
6. ⏳ Update documentation

**Success Criteria**:
- ✅ 12+ unit tests passing
- ✅ Songbird can negotiate RSA
- ✅ 99%+ server compatibility

---

## 🔧 Technical Architecture

### File Structure

```
crates/beardog-tunnel/src/unix_socket_ipc/
├── handlers/
│   └── crypto.rs                    (Add routes to new handlers)
├── crypto_handlers.rs               (Existing Ed25519, X25519, etc.)
├── crypto_handlers_ecdsa.rs         (NEW - Phase 1 & 2)
├── crypto_handlers_eddsa.rs         (NEW - Phase 3)
└── crypto_handlers_rsa.rs           (NEW - Phase 4)
```

### Handler Module Pattern

Each new handler module will follow BearDog's established pattern:

```rust
//! ECDSA Crypto Handlers
//! 
//! Pure Rust ECDSA operations using RustCrypto's p256/p384/p521 crates.

use anyhow::Result;
use base64::Engine;

/// Sign data with ECDSA P-256
pub async fn handle_sign_ecdsa_secp256r1(
    params: Option<&serde_json::Value>,
) -> Result<serde_json::Value, String> {
    // Implementation
}

/// Verify ECDSA P-256 signature
pub async fn handle_verify_ecdsa_secp256r1(
    params: Option<&serde_json::Value>,
) -> Result<serde_json::Value, String> {
    // Implementation
}

#[cfg(test)]
mod tests {
    // Comprehensive tests
}
```

### Integration with Handler Registry

Update `handlers/crypto.rs` to route new methods:

```rust
"crypto.sign_ecdsa_secp256r1" => {
    info!("🔐 Crypto: sign_ecdsa_secp256r1");
    super::super::crypto_handlers_ecdsa::handle_sign_ecdsa_secp256r1(params).await
}

"crypto.verify_ecdsa_secp256r1" => {
    info!("✅ Crypto: verify_ecdsa_secp256r1");
    super::super::crypto_handlers_ecdsa::handle_verify_ecdsa_secp256r1(params).await
}
```

---

## 🧪 Testing Strategy

### Unit Tests (Per Algorithm)

**For Each Algorithm**:
1. ✅ Sign with generated key, verify signature (roundtrip)
2. ✅ Verify known-good signature (test vectors)
3. ✅ Reject invalid signature
4. ✅ Reject malformed signature
5. ✅ Handle missing parameters gracefully

**Total Tests**:
- Phase 1: 5 tests (ECDSA P-256)
- Phase 2: +10 tests (P-384, P-521)
- Phase 3: +5 tests (Ed448)
- Phase 4: +12 tests (RSA variants)
- **Grand Total**: 32 new tests

### Integration Tests (With Songbird)

**Test Scenarios**:
1. Songbird sends ClientHello with new algorithm
2. BearDog verifies server certificate signature
3. Full TLS 1.3 handshake completes
4. HTTP request succeeds

### Real-World Validation

**Target Servers**:
- ✅ GitHub API (`api.github.com`) - ECDSA P-256
- ✅ CloudFlare (`cloudflare.com`) - ECDSA P-256
- ✅ Google APIs (`googleapis.com`) - ECDSA P-256
- ✅ Let's Encrypt (`letsencrypt.org`) - Ed25519 / ECDSA P-256
- ✅ AWS Services (`amazonaws.com`) - RSA

---

## 📦 Dependencies Update

### Cargo.toml Changes

```toml
# Cryptography (100% Pure Rust!)
ed25519-dalek = "2.1"                              # Already have
x25519-dalek = "2.0"                               # Already have
p256 = { version = "0.13", features = ["ecdsa"] }  # Already have
p384 = { version = "0.13", features = ["ecdsa"] }  # NEW - Phase 2
# p521 = { version = "0.1", features = ["ecdsa"] }  # Optional - Phase 2 (less mature)
ed448-goldilocks = "0.9"                           # NEW - Phase 3
rsa = "0.9"                                        # Already have
sha2 = "0.10"                                      # Already have
```

**Total New Dependencies**: 2 (p384, ed448-goldilocks)

---

## 📈 Implementation Timeline

### Week 1: Critical Path (Phase 1)

**Day 1-2**: ECDSA P-256 Implementation
- Create `crypto_handlers_ecdsa.rs`
- Implement sign/verify methods
- Write unit tests
- Integration with handler registry

**Day 3**: Testing & Validation
- Run unit tests
- Test with Songbird
- Validate with GitHub API
- Document in BEARDOG_RPC_API.md

**Milestone**: Songbird can connect to GitHub! 🎉

### Week 2: High Priority (Phase 2 & 3)

**Day 4-5**: ECDSA P-384/P-521
- Add p384 dependency
- Implement P-384 methods
- (Optional) Add P-521 experimental support
- Unit tests

**Day 6**: Ed448
- Add ed448-goldilocks dependency
- Create `crypto_handlers_eddsa.rs`
- Implement Ed448 methods
- Unit tests

**Day 7**: Integration & Validation
- Test all new algorithms with Songbird
- Validate with diverse servers
- Performance benchmarks

**Milestone**: 90%+ server compatibility! 🎉

### Week 3+: Legacy Support (Phase 4)

**Day 8-10**: RSA Implementation
- Create `crypto_handlers_rsa.rs`
- Implement PKCS#1 v1.5 variants
- Implement RSA-PSS variants
- Comprehensive unit tests (12 tests)

**Day 11**: Final Validation
- Test with legacy servers
- Test with AWS services
- Performance benchmarks
- Final documentation updates

**Milestone**: 99%+ server compatibility! 🎉

---

## 🎯 Success Metrics

### Phase 1 (ECDSA P-256) - CRITICAL

**Blocking Metrics**:
- ✅ 5+ unit tests passing
- ✅ < 1ms signing latency
- ✅ < 1ms verification latency
- ✅ Songbird ClientHello includes ECDSA P-256
- ✅ GitHub handshake succeeds

**Compatibility**:
- ✅ GitHub API working
- ✅ CloudFlare working
- ✅ Google APIs working
- ✅ 65%+ of HTTPS servers accessible

### Phase 2-3 (ECDSA P-384/P-521, Ed448)

**Quality Metrics**:
- ✅ 20+ total unit tests passing
- ✅ All algorithms < 1ms latency
- ✅ Zero unsafe code
- ✅ Comprehensive documentation

**Compatibility**:
- ✅ 90%+ of HTTPS servers accessible
- ✅ High-security servers working
- ✅ Modern Ed448 servers working

### Phase 4 (RSA)

**Complete Metrics**:
- ✅ 32+ total unit tests passing
- ✅ RSA operations < 5ms (acceptable for legacy)
- ✅ Zero C dependencies maintained
- ✅ Full RPC API documentation

**Compatibility**:
- ✅ 99%+ of HTTPS servers accessible
- ✅ Legacy enterprise servers working
- ✅ AWS services working

---

## 🔐 Security Considerations

### Key Generation

**Default Behavior**:
- Use `OsRng` for all key generation (cryptographically secure)
- Support HSM-backed key generation (future)
- Never log or expose private keys

### Constant-Time Operations

**All Algorithms**:
- Signature verification: constant-time (timing attack resistant)
- Private key operations: constant-time
- No branching on secret data

### Zeroization

**After Each Operation**:
- Private keys: zeroize immediately after use
- Shared secrets: zeroize after derivation
- Temporary buffers: zeroize before deallocation

### Input Validation

**For All Methods**:
- Validate key sizes (reject too small)
- Validate signature formats (ASN.1 DER)
- Validate public key formats
- Reject malformed inputs gracefully

---

## 📚 Documentation Updates

### Files to Update

1. **`docs/BEARDOG_RPC_API.md`**
   - Add 18 new crypto methods
   - Document parameters/responses
   - Add examples for each algorithm

2. **`docs/TLS_CRYPTO_API.md`**
   - Update supported signature algorithms
   - Add algorithm selection guide
   - Add performance benchmarks

3. **`README.md`**
   - Update crypto capabilities section
   - Highlight ECDSA P-256 support

4. **`CHANGELOG.md`**
   - Document all new methods
   - Note performance characteristics
   - Credit Songbird team for excellent handoff

---

## 🤝 Coordination with Songbird

### What BearDog Commits To

**Phase 1 (Days 1-3)**:
- ✅ ECDSA P-256 complete
- ✅ Unit tests passing
- ✅ RPC methods documented
- ✅ Ready for Songbird integration

**Phase 2-3 (Days 4-7)**:
- ✅ ECDSA P-384/P-521 complete
- ✅ Ed448 complete
- ✅ Comprehensive tests
- ✅ 90%+ server compatibility

**Phase 4 (Days 8-11)**:
- ✅ RSA complete
- ✅ 99%+ server compatibility
- ✅ Final documentation

### What Songbird Can Expect

**Immediate (Phase 1)**:
- RPC methods: `crypto.sign_ecdsa_secp256r1`, `crypto.verify_ecdsa_secp256r1`
- API format: Same as existing Ed25519 methods
- Performance: < 1ms per operation
- Zero breaking changes to existing API

**Near-Term (Phase 2-3)**:
- 6 more ECDSA/EdDSA methods
- Same API patterns
- Same performance targets
- Adaptive negotiation ready

**Future (Phase 4)**:
- 12 RSA methods
- Legacy compatibility
- Complete TLS 1.3 signature algorithm support

---

## 💡 Design Decisions

### Why Separate Handler Modules?

**Rationale**:
1. **Modularity**: Each crypto family is self-contained
2. **Testability**: Independent unit tests per module
3. **Maintainability**: Clear separation of concerns
4. **Performance**: Can optimize per algorithm
5. **Future**: Easy to add new algorithms

### Why Software-First, HSM-Later?

**Rationale**:
1. **Velocity**: RustCrypto is production-ready now
2. **Compatibility**: Software works on all platforms
3. **Testing**: Easier to test software implementations
4. **HSM Support**: Can add later without API changes
5. **Pure Rust**: Maintains zero C dependency goal

### Why Not Ring?

**Rationale**:
1. **C Dependencies**: Ring has C/assembly code
2. **Universal Portability**: RustCrypto is Pure Rust
3. **ARM Support**: RustCrypto works on all architectures
4. **Ecosystem Alignment**: BearDog is 100% Pure Rust
5. **Principle**: Deep debt solution (Pure Rust everywhere)

---

## 🎊 Summary

### BearDog Team's Response

**We enthusiastically accept this handoff!**

✅ **Analysis**: Excellent, actionable, comprehensive  
✅ **Priority**: Agreed - ECDSA P-256 is CRITICAL  
✅ **Libraries**: All Pure Rust, already in Cargo.toml  
✅ **Timeline**: Achievable - Phase 1 in 1-3 days  
✅ **Architecture**: Aligns with BearDog patterns  

### Commitment

**Phase 1 (CRITICAL)**:
- Start: Immediately
- Complete: 1-3 days
- Deliverable: ECDSA P-256 working with GitHub

**Phase 2-3 (HIGH/MEDIUM)**:
- Start: After Phase 1
- Complete: 1-2 weeks total
- Deliverable: 90%+ server compatibility

**Phase 4 (LOW)**:
- Start: After Phase 2-3
- Complete: 2-3 weeks total
- Deliverable: 99%+ server compatibility

### Impact

**Without ECDSA P-256**: ❌ 65% of servers inaccessible  
**With ECDSA P-256**: ✅ GitHub, CloudFlare, Google work!  
**With All Algorithms**: ✅ 99%+ of servers work!

**Let's make Pure Rust TLS 1.3 a reality!** 🐕🦀🚀

---

*Plan Created: January 22, 2026*  
*Status: APPROVED - Starting Phase 1 immediately*  
*Timeline: Phase 1 complete in 1-3 days*  
*Principle: 100% Pure Rust, zero C dependencies*

🐕 **BearDog is on it!** 🐕

