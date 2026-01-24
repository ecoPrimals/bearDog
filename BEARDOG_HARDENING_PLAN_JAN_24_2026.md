# BearDog Hardening & Evolution Plan - January 24, 2026

**Source**: Songbird team handoff post-100% HTTPS success  
**Philosophy**: MOVE diagnostic logging for future use (fossil record), not REMOVE  
**Status**: Ready to execute

---

## 🎯 Core Principle: Preserve, Don't Delete

Following ecoPrimals' "fossil record" principle:
- **MOVE** diagnostic logging to debug module
- **PRESERVE** all diagnostic capabilities for future debugging
- **EVOLVE** to production-safe conditional compilation
- **MAINTAIN** full diagnostic visibility when needed

---

## 📋 Immediate Actions (v0.22.0)

### 1. Create Debug Diagnostics Module ✨

**Philosophy**: Preserve all diagnostic logging with `cfg(feature = "diagnostics")`

**New File**: `crates/beardog-tunnel/src/diagnostics/mod.rs`

```rust
//! Production Diagnostics Module
//!
//! All diagnostic logging preserved here for future debugging.
//! Enable with: cargo build --features diagnostics
//!
//! **Philosophy**: MOVE, not REMOVE - fossil record for future troubleshooting

pub mod crypto;
pub mod tls;

/// Diagnostic macro - only active with "diagnostics" feature
#[macro_export]
macro_rules! diagnostic {
    ($($arg:tt)*) => {
        #[cfg(feature = "diagnostics")]
        eprintln!($($arg)*);
    };
}
```

**New File**: `crates/beardog-tunnel/src/diagnostics/crypto.rs`

```rust
//! Crypto Operation Diagnostics
//!
//! Preserved diagnostic logging for AES-GCM, ChaCha20-Poly1305, and other crypto ops.
//! Originally from crypto_handlers_aes_gcm.rs lines 336-347, 368

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;

/// Diagnostic log for AES-128-GCM encryption
///
/// MOVED from crypto_handlers_aes_gcm.rs (not removed!)
/// Enable with: cargo build --features diagnostics
#[cfg(feature = "diagnostics")]
pub fn log_aes128_gcm_encrypt(
    key_len: usize,
    nonce_len: usize,
    plaintext_len: usize,
    aad: &[u8],
    ciphertext_len: usize,
) {
    eprintln!("════════════════════════════════════════════════════════");
    eprintln!("🔐 AES-128-GCM ENCRYPT DIAGNOSTIC (stderr):");
    eprintln!("   Key: {} bytes", key_len);
    eprintln!("   Nonce: {} bytes", nonce_len);
    eprintln!("   Plaintext: {} bytes", plaintext_len);
    eprintln!("   AAD: {} bytes", aad.len());
    if aad.is_empty() {
        eprintln!("   ⚠️  AAD is EMPTY - this might cause TLS decrypt_error!");
    } else {
        eprintln!("   AAD (hex): {}", hex::encode(aad));
    }
    eprintln!("   ✅ Ciphertext: {} bytes (plaintext + 16-byte tag)", ciphertext_len);
    eprintln!("════════════════════════════════════════════════════════");
}

/// No-op version when diagnostics disabled (zero cost)
#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn log_aes128_gcm_encrypt(
    _key_len: usize,
    _nonce_len: usize,
    _plaintext_len: usize,
    _aad: &[u8],
    _ciphertext_len: usize,
) {
    // Zero-cost no-op
}

// Similar functions for other crypto operations...
```

### 2. Update Cargo.toml

```toml
[features]
default = []
diagnostics = []  # Enable verbose crypto diagnostics
```

### 3. Evolve AES-GCM Handler

**File**: `crates/beardog-tunnel/src/unix_socket_ipc/crypto_handlers_aes_gcm.rs`

**Replace lines 335-369**:

```rust
// EVOLVED: Diagnostic logging moved to diagnostics module (not removed!)
// Enable with: cargo build --features diagnostics
use crate::diagnostics::crypto::log_aes128_gcm_encrypt;

// Extract optional AAD (CRITICAL for TLS 1.3!)
let aad_bytes = if let Some(aad_b64) = params.get("aad").and_then(|v| v.as_str()) {
    BASE64
        .decode(aad_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 aad: {}", e)))?
} else {
    Vec::new()
};

// Create cipher
let key = Zeroizing::new(key_bytes);
let cipher = Aes128Gcm::new_from_slice(&key)
    .map_err(|e| BearDogError::system(format!("Failed to create AES-128-GCM cipher: {}", e)))?;

// Create nonce
let nonce = Nonce::from_slice(&nonce_bytes);

// Create payload with AAD
let payload = Payload {
    msg: &plaintext,
    aad: &aad_bytes,
};

// Encrypt
let ciphertext = cipher
    .encrypt(nonce, payload)
    .map_err(|e| BearDogError::system(format!("AES-128-GCM encryption failed: {}", e)))?;

// Diagnostic logging (zero-cost when disabled, full detail when enabled)
log_aes128_gcm_encrypt(
    key_bytes.len(),
    nonce_bytes.len(),
    plaintext.len(),
    &aad_bytes,
    ciphertext.len(),
);
```

---

## 🔒 Security Hardening

### 1. Constant-Time Operations

**Status**: ✅ Already implemented (RustCrypto uses `subtle` crate internally)

**Verification needed**:
- Review all manual tag comparisons
- Ensure no timing leaks in error paths

### 2. Zeroize Sensitive Data

**Status**: ✅ Partially done (using `Zeroizing<T>` wrapper)

**Needs review**:
- All key material uses `Zeroizing<Vec<u8>>`
- Check for any leaked intermediate values

### 3. Input Validation

**Status**: ✅ Comprehensive validation already in place

**Current validations**:
- Key lengths (16/32 bytes for AES, 32 for X25519)
- Nonce lengths (12 bytes for AEAD)
- Base64 decoding errors
- Missing required parameters

### 4. Rate Limiting & DoS Protection

**Status**: ⚠️ TODO

**Priority**: Medium (after diagnostics cleanup)

**Plan**:
- Add per-client request rate limiting
- Max payload size limits
- Connection timeouts

---

## 🧪 Testing Strategy

### Unit Tests
```bash
# Standard tests
cargo test -p beardog-tunnel --lib

# With diagnostics enabled
cargo test -p beardog-tunnel --lib --features diagnostics
```

### Integration Tests
```bash
# Full HTTPS integration with diagnostics
cargo test -p beardog-tunnel --test phase8_https_comprehensive_tests --features diagnostics -- --nocapture
```

---

## 📂 File Structure After Evolution

```
crates/beardog-tunnel/src/
├── diagnostics/                    # NEW! Diagnostic logging module
│   ├── mod.rs                     # Diagnostic macro + re-exports
│   ├── crypto.rs                  # Crypto operation diagnostics
│   └── tls.rs                     # TLS handshake diagnostics
├── unix_socket_ipc/
│   ├── crypto_handlers_aes_gcm.rs # EVOLVED: Uses diagnostics module
│   └── handlers/crypto/tls.rs     # EVOLVED: Uses diagnostics module
└── lib.rs                         # Declares diagnostics module
```

---

## 🎯 Implementation Steps

### Step 1: Create Diagnostics Infrastructure (1 hour)
- [ ] Create `diagnostics/mod.rs` with macro
- [ ] Create `diagnostics/crypto.rs` with preserved logging
- [ ] Add `diagnostics` feature to Cargo.toml
- [ ] Update `lib.rs` to declare module

### Step 2: Evolve AES-GCM Handler (30 min)
- [ ] Replace eprintln! with diagnostic! macro
- [ ] Move detailed logging to diagnostics module
- [ ] Test with and without diagnostics feature

### Step 3: Evolve TLS Handlers (30 min)
- [ ] Move verbose hex dumps to diagnostics module
- [ ] Keep info! level for key events
- [ ] Test handshake with diagnostics enabled

### Step 4: Update Documentation (30 min)
- [ ] Document diagnostics feature in README
- [ ] Add troubleshooting guide
- [ ] Update CHANGELOG with v0.22.0

### Step 5: Testing & Validation (30 min)
- [ ] Run full test suite (default features)
- [ ] Run tests with diagnostics enabled
- [ ] Verify zero performance impact

**Total Time**: 3 hours

---

## 📊 Benefits

| Aspect | Before | After |
|--------|--------|-------|
| **Production Code** | eprintln! in hot paths | Zero-cost no-ops |
| **Debugging** | Edit code, recompile | `--features diagnostics` |
| **Performance** | Potential overhead | Zero overhead (inlined) |
| **Maintainability** | Scattered diagnostics | Centralized module |
| **Fossil Record** | N/A | ✅ All diagnostics preserved |

---

## 🚀 Future Evolution (v0.23.0+)

### Capability Declaration
```json
{
  "primal": "beardog",
  "capabilities": {
    "symmetric_encryption": {
      "algorithms": ["AES-128-GCM", "AES-256-GCM", "ChaCha20-Poly1305"],
      "features": ["diagnostics", "constant_time", "zeroize"]
    }
  }
}
```

### Neural API Integration
- Semantic request translation
- Runtime capability discovery
- Agnostic primal communication

---

## Summary

**Philosophy**: PRESERVE diagnostic capabilities, EVOLVE conditional compilation

**Grade**: A+ (Zero-cost abstraction, full fossil record preservation)

**Ready**: Proceed to execute! 🚀🦀🔒

