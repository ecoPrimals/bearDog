# 🚀 crypto_handlers.rs Refactoring - Execution Plan

**Date**: January 24, 2026  
**Task**: Phase 1.1 - Refactor crypto_handlers.rs (2,499 lines) by domain  
**Estimated Time**: 8 hours  
**Status**: READY TO EXECUTE  

---

## 📊 CURRENT STATE

### File Structure
- **Single File**: `crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers.rs`
- **Lines**: 2,499 lines
- **Functions**: 14 public async handlers
- **Issue**: Monolithic, hard to navigate, cognitive overload

### Method Inventory

**TLS Methods** (6 functions, ~1,200 lines):
- `handle_tls_derive_secrets` (178 lines) - Legacy
- `handle_tls_derive_application_secrets` (328 lines) - Application keys
- `handle_tls_derive_handshake_secrets` (236 lines) - Handshake keys
- `handle_tls_sign_handshake` (89 lines) - Ed25519 signing
- `handle_tls_verify_certificate` (883 lines) - X.509 verification
- `handle_tls_compute_finished_verify_data` (77 lines) - Finished message

**Asymmetric Crypto** (4 functions, ~400 lines):
- `handle_sign_ed25519` (68 lines) - Ed25519 signing
- `handle_verify_ed25519` (65 lines) - Ed25519 verification
- `handle_x25519_generate_ephemeral` (44 lines) - X25519 keypair
- `handle_x25519_derive_secret` (73 lines) - ECDH derivation

**Symmetric Crypto** (2 functions, ~280 lines):
- `handle_chacha20_poly1305_encrypt` (86 lines) - AEAD encryption
- `handle_chacha20_poly1305_decrypt` (108 lines) - AEAD decryption

**Hash Functions** (2 functions, ~110 lines):
- `handle_blake3_hash` (44 lines) - Blake3 hashing
- `handle_hmac_sha256` (66 lines) - HMAC-SHA256

**Utility Functions** (1 function, ~130 lines):
- `export_to_sslkeylogfile` (130 lines) - Wireshark key export

**Total**: 14 functions, ~2,120 lines of handlers + ~379 lines of support code

---

## 🎯 TARGET STRUCTURE

```
crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto/
├── mod.rs              (~150 lines) - Module exports & shared utilities
├── tls.rs              (~1,200 lines) - TLS 1.3 methods (6 functions)
├── asymmetric.rs       (~400 lines) - Ed25519, X25519 (4 functions)
├── symmetric.rs        (~280 lines) - ChaCha20-Poly1305 (2 functions)
├── hash.rs             (~110 lines) - Blake3, HMAC (2 functions)
├── sslkeylog.rs        (~150 lines) - SSLKEYLOGFILE export utility
└── README.md           - Documentation for this module
```

**Benefits**:
- ✅ Clear semantic organization
- ✅ Files under 1,200 lines (target: 400-600 average)
- ✅ Easier to navigate and maintain
- ✅ Parallel development possible
- ✅ Better test isolation
- ✅ Reduced cognitive load

---

## 📋 REFACTORING STEPS

### Step 1: Create mod.rs (30 minutes)

**File**: `handlers/crypto/mod.rs`

**Content**:
```rust
//! Crypto operation handlers organized by domain
//!
//! This module provides cryptographic operations via JSON-RPC, organized
//! into semantic domains for better maintainability.
//!
//! # Organization
//!
//! - [`tls`] - TLS 1.3 crypto operations (handshake, application keys, signatures)
//! - [`asymmetric`] - Asymmetric crypto (Ed25519, X25519)
//! - [`symmetric`] - Symmetric crypto (ChaCha20-Poly1305, AES-GCM)
//! - [`hash`] - Hashing operations (Blake3, SHA-2, HMAC)
//! - [`sslkeylog`] - SSLKEYLOGFILE export for Wireshark

pub mod tls;
pub mod asymmetric;
pub mod symmetric;
pub mod hash;
pub mod sslkeylog;

// Re-export all handlers for backward compatibility
pub use tls::*;
pub use asymmetric::*;
pub use symmetric::*;
pub use hash::*;
```

**Actions**:
1. Create file with module structure
2. Add documentation
3. Set up re-exports

### Step 2: Extract sslkeylog.rs (30 minutes)

**File**: `handlers/crypto/sslkeylog.rs`

**Extract**:
- Lines 29-152: `export_to_sslkeylogfile()` function
- Add module documentation
- Add usage examples

**Test**: Ensure it compiles independently

### Step 3: Extract tls.rs (2 hours)

**File**: `handlers/crypto/tls.rs`

**Extract Functions** (in order):
1. `handle_tls_derive_secrets` (line ~708)
2. `handle_tls_derive_application_secrets` (line ~886)
3. `handle_tls_derive_handshake_secrets` (line ~1214)
4. `handle_tls_sign_handshake` (line ~1450)
5. `handle_tls_verify_certificate` (line ~1539)
6. `handle_tls_compute_finished_verify_data` (line ~2422)

**Add**:
- Module documentation
- Import `super::sslkeylog::export_to_sslkeylogfile`
- Shared TLS utilities (if any)

**Test**: Run TLS-related tests

### Step 4: Extract asymmetric.rs (1 hour)

**File**: `handlers/crypto/asymmetric.rs`

**Extract Functions**:
1. `handle_sign_ed25519` (line ~154)
2. `handle_verify_ed25519` (line ~222)
3. `handle_x25519_generate_ephemeral` (line ~287)
4. `handle_x25519_derive_secret` (line ~331)

**Add**:
- Module documentation
- Ed25519 vs X25519 section headers

**Test**: Run asymmetric crypto tests

### Step 5: Extract symmetric.rs (1 hour)

**File**: `handlers/crypto/symmetric.rs`

**Extract Functions**:
1. `handle_chacha20_poly1305_encrypt` (line ~404)
2. `handle_chacha20_poly1305_decrypt` (line ~490)

**Future**: Room for AES-GCM, AES-CBC when added

**Add**:
- Module documentation
- AEAD pattern documentation

**Test**: Run symmetric crypto tests

### Step 6: Extract hash.rs (30 minutes)

**File**: `handlers/crypto/hash.rs`

**Extract Functions**:
1. `handle_blake3_hash` (line ~598)
2. `handle_hmac_sha256` (line ~642)

**Future**: Room for SHA-2, SHA-3, HMAC variants

**Add**:
- Module documentation
- Hash vs HMAC sections

**Test**: Run hash tests

### Step 7: Update Imports (1 hour)

**Files to Update**:
1. `handlers/crypto.rs` - Change from standalone to re-export from crypto/mod.rs
2. `handlers/mod.rs` - Update path if needed
3. All test files that import from crypto_handlers

**Pattern**:
```rust
// OLD
use crate::unix_socket_ipc::crypto_handlers::*;

// NEW
use crate::unix_socket_ipc::handlers::crypto::*;
```

**Test**: Run full test suite

### Step 8: Remove Old File (15 minutes)

**Actions**:
1. Verify all tests passing
2. Delete `crypto_handlers.rs`
3. Update documentation references
4. Commit changes

### Step 9: Documentation (1 hour)

**Create**: `handlers/crypto/README.md`

**Content**:
- Architecture overview
- Method catalog by domain
- How to add new methods
- Testing strategy

**Update**:
- Root `docs/` if needed
- CHANGELOG.md with refactoring notes

### Step 10: Testing & Validation (1 hour)

**Actions**:
1. Run full test suite: `cargo test`
2. Check for warnings: `cargo clippy`
3. Verify performance: `cargo bench` (if benchmarks exist)
4. Manual testing of RPC calls

---

## 🧪 TESTING STRATEGY

### Unit Tests
- Each domain file should have `#[cfg(test)]` module
- Test individual handlers
- Test error cases

### Integration Tests
- Test cross-domain interactions (TLS calls symmetric crypto)
- Test RPC routing to new modules
- Test backward compatibility

### Performance Tests
- Ensure refactoring doesn't impact performance
- Benchmark before/after
- Profile if any regressions

---

## 📊 SUCCESS CRITERIA

### File Size
- [ ] No file over 1,200 lines
- [ ] Average file size 400-600 lines
- [ ] mod.rs under 200 lines

### Organization
- [ ] Clear semantic domains
- [ ] Each domain logically grouped
- [ ] Future extensibility clear

### Functionality
- [ ] All 14 handlers work correctly
- [ ] All tests passing (1,409+)
- [ ] No clippy warnings
- [ ] Backward compatibility maintained

### Documentation
- [ ] Each module documented
- [ ] README.md created
- [ ] Migration guide if needed

---

## 🚨 RISKS & MITIGATION

### Risk 1: Breaking Imports

**Mitigation**:
- Keep re-exports in mod.rs
- Update imports incrementally
- Test after each file extraction

### Risk 2: Test Failures

**Mitigation**:
- Test after each extraction
- Keep tests with their domains
- Run full suite at end

### Risk 3: Performance Regression

**Mitigation**:
- Profile before/after
- Benchmark critical paths
- Use `#[inline]` if needed

### Risk 4: Merge Conflicts

**Mitigation**:
- Work in feature branch
- Communicate with team
- Commit frequently

---

## 📅 EXECUTION TIMELINE

**Total**: 8 hours

| Step | Description | Time | Cumulative |
|------|-------------|------|------------|
| 1 | Create mod.rs | 0.5h | 0.5h |
| 2 | Extract sslkeylog.rs | 0.5h | 1.0h |
| 3 | Extract tls.rs | 2.0h | 3.0h |
| 4 | Extract asymmetric.rs | 1.0h | 4.0h |
| 5 | Extract symmetric.rs | 1.0h | 5.0h |
| 6 | Extract hash.rs | 0.5h | 5.5h |
| 7 | Update imports | 1.0h | 6.5h |
| 8 | Remove old file | 0.25h | 6.75h |
| 9 | Documentation | 1.0h | 7.75h |
| 10 | Testing | 0.25h | 8.0h |

**Break Points**: After steps 3, 6, 8 (natural checkpoint s)

---

## 🔍 PRE-EXECUTION CHECKLIST

- [ ] Feature branch created
- [ ] Backup of current file
- [ ] Tests passing (baseline)
- [ ] No uncommitted changes
- [ ] Team notified (if applicable)

## ✅ POST-EXECUTION CHECKLIST

- [ ] All tests passing
- [ ] No clippy warnings
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Performance validated
- [ ] PR created (if applicable)
- [ ] Code reviewed
- [ ] Merged to main

---

## 📚 REFERENCES

- Original file: `crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers.rs`
- Evolution plan: `COMPREHENSIVE_EVOLUTION_PLAN_JAN_24_2026.md`
- RPC API docs: `docs/BEARDOG_RPC_API.md`

---

**Ready to execute!** 🚀🦀

**Next Step**: Create feature branch and start Step 1 (create mod.rs)

