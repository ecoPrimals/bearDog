# TLS Module Refactoring Plan - January 24, 2026

## Current State

**File**: `crates/beardog-tunnel/src/unix_socket_ipc/handlers/crypto/tls.rs`
**Size**: 2174 lines
**Status**: Single monolithic file with excellent documentation

## Analysis

The file is already well-organized with clear sections:
- Module-level documentation (excellent)
- TLS 1.3 key schedule diagram
- Imports and utilities
- Key derivation functions
- Signature functions
- Certificate verification
- Finished MAC computation

**Quality**: High - comprehensive docs, RFC-compliant, well-tested

## Refactoring Strategy: Smart Separation

### Goal
Split by **logical concern** while preserving:
- ✅ Excellent documentation
- ✅ RFC compliance
- ✅ Test coverage
- ✅ API compatibility

### New Structure

```
crypto/
├── tls/
│   ├── mod.rs              # Module exports + overview docs
│   ├── key_derivation.rs   # HKDF, key schedule (Stage 1 & 2)
│   ├── signatures.rs       # Ed25519, handshake signing
│   ├── certificates.rs     # X.509 parsing, chain validation
│   └── finished_mac.rs     # TLS Finished computation
└── tls.rs → DEPRECATED (re-exports for compatibility)
```

### Module Breakdown

#### 1. `tls/mod.rs` (100-150 lines)
- Module-level documentation
- TLS 1.3 key schedule diagram
- Re-exports of all public functions
- Common imports

#### 2. `tls/key_derivation.rs` (~600-700 lines)
**Functions**:
- `handle_tls_derive_secrets` (legacy)
- `handle_tls_derive_handshake_secrets` (Stage 1)
- `handle_tls_derive_application_secrets` (Stage 2)
- Helper: `hkdf_expand_label`
- Helper: `derive_secret`

**Rationale**: These are tightly coupled (HKDF operations)

#### 3. `tls/signatures.rs` (~400-500 lines)
**Functions**:
- `handle_tls_sign_handshake`
- `handle_tls_compute_finished_verify_data`
- Helper: Key derivation for signatures

**Rationale**: Both involve signing/MAC operations

#### 4. `tls/certificates.rs` (~800-900 lines)
**Functions**:
- `handle_tls_verify_certificate`
- Helper: X.509 parsing
- Helper: Chain validation
- Helper: Name verification

**Rationale**: Certificate operations are self-contained

#### 5. `tls/finished_mac.rs` (~200-300 lines)
**Functions**:
- `handle_tls_compute_finished_verify_data` (if not in signatures.rs)
- Helper: HMAC computation

**Rationale**: TLS Finished is specific enough to warrant separation

### Migration Steps

1. ✅ **Create module structure**
2. **Extract key_derivation.rs**
   - Copy HKDF functions
   - Update imports
   - Add module docs
3. **Extract signatures.rs**
   - Copy signing functions
   - Update imports
   - Add module docs
4. **Extract certificates.rs**
   - Copy X.509 functions
   - Update imports
   - Add module docs
5. **Create mod.rs**
   - Module overview
   - Re-export all functions
   - Maintain API compatibility
6. **Deprecate tls.rs**
   - Add deprecation notice
   - Re-export from tls/ module
   - Keep for backward compatibility
7. **Update imports**
   - Search for `use super::tls::`
   - Update to `use super::tls::key_derivation::`
8. **Run tests**
   - Ensure all tests pass
   - No API breakage

### Benefits

**Code Organization**:
- ✅ Each file < 1000 lines
- ✅ Clear separation of concerns
- ✅ Easier navigation

**Maintainability**:
- ✅ Focused modules
- ✅ Easier to review
- ✅ Simpler to test

**Documentation**:
- ✅ Module-level context
- ✅ Focused docs per concern
- ✅ Preserve RFC references

### Backward Compatibility

Old code continues to work:
```rust
// Still works (via re-export)
use crate::unix_socket_ipc::handlers::crypto::tls::handle_tls_derive_handshake_secrets;
```

New code can use specific modules:
```rust
// More explicit
use crate::unix_socket_ipc::handlers::crypto::tls::key_derivation::handle_tls_derive_handshake_secrets;
```

### Testing Strategy

1. **No test changes required**
   - Tests import from mod.rs
   - Public API unchanged
2. **Run full test suite**
   - `cargo test -p beardog-tunnel`
3. **Verify no regressions**
   - All 1,399+ tests must pass

---

## Implementation Notes

### Key Principles

1. **Smart, not mechanical**
   - Group by logical concern
   - Preserve relationships
   - Maintain documentation quality

2. **Preserve excellence**
   - Keep excellent docs
   - Maintain RFC references
   - Preserve test coverage

3. **API stability**
   - No breaking changes
   - Re-exports for compatibility
   - Deprecation path if needed

### File Size Targets

- `mod.rs`: 100-150 lines
- `key_derivation.rs`: 600-700 lines
- `signatures.rs`: 400-500 lines
- `certificates.rs`: 800-900 lines

**Total**: ~2000 lines (similar to current, but organized)

---

## Timeline

**Session 1** (Current, 1-2 hours):
- ✅ Create structure
- 🔄 Extract key_derivation.rs
- 🔄 Extract signatures.rs

**Session 2** (1-2 hours):
- Extract certificates.rs
- Create mod.rs
- Update imports
- Test

**Total**: 2-4 hours

---

**Plan Created**: January 24, 2026
**Status**: Ready for execution
**Risk**: Low (API-compatible refactoring)

