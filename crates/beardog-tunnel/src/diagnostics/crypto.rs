//! Crypto Operation Diagnostics
//!
//! Preserved diagnostic logging for cryptographic operations.
//! Originally from `crypto_handlers_aes_gcm.rs` during HTTPS TLS 1.3 debugging.
//!
//! **FOSSIL RECORD**: These diagnostics were CRITICAL for debugging the
//! 100% Pure Rust HTTPS implementation (January 2026). They helped identify:
//! - Empty AAD causing TLS decrypt_error
//! - Key length mismatches
//! - Nonce generation issues
//! - Tag verification failures
//!
//! **PRESERVED**: Not deleted, available via `--features diagnostics`

/// Diagnostic log for AES-128-GCM encryption
///
/// **Origin**: `crypto_handlers_aes_gcm.rs` lines 336-369
/// **Date**: January 23, 2026 (HTTPS debugging session)
/// **Purpose**: Debug AAD handling in TLS 1.3 record encryption
///
/// Enable with: `cargo build --features diagnostics`
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

/// Zero-cost no-op when diagnostics disabled (default)
///
/// This function is completely inlined and optimized away by the compiler.
/// **Zero runtime overhead** in production builds.
#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn log_aes128_gcm_encrypt(
    _key_len: usize,
    _nonce_len: usize,
    _plaintext_len: usize,
    _aad: &[u8],
    _ciphertext_len: usize,
) {
    // Zero-cost no-op - completely inlined away
}

/// Diagnostic log for AES-256-GCM encryption
#[cfg(feature = "diagnostics")]
pub fn log_aes256_gcm_encrypt(
    key_len: usize,
    nonce_len: usize,
    plaintext_len: usize,
    aad: &[u8],
    ciphertext_len: usize,
) {
    eprintln!("════════════════════════════════════════════════════════");
    eprintln!("🔐 AES-256-GCM ENCRYPT DIAGNOSTIC:");
    eprintln!("   Key: {} bytes", key_len);
    eprintln!("   Nonce: {} bytes", nonce_len);
    eprintln!("   Plaintext: {} bytes", plaintext_len);
    eprintln!("   AAD: {} bytes", aad.len());
    if !aad.is_empty() {
        eprintln!("   AAD (hex): {}", hex::encode(aad));
    }
    eprintln!("   Ciphertext: {} bytes (+ 16-byte tag)", ciphertext_len);
    eprintln!("════════════════════════════════════════════════════════");
}

#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn log_aes256_gcm_encrypt(
    _key_len: usize,
    _nonce_len: usize,
    _plaintext_len: usize,
    _aad: &[u8],
    _ciphertext_len: usize,
) {}

/// Diagnostic log for ChaCha20-Poly1305 encryption
#[cfg(feature = "diagnostics")]
pub fn log_chacha20_poly1305_encrypt(
    key_len: usize,
    nonce_len: usize,
    plaintext_len: usize,
    aad: &[u8],
    ciphertext_len: usize,
) {
    eprintln!("════════════════════════════════════════════════════════");
    eprintln!("🔐 ChaCha20-Poly1305 ENCRYPT DIAGNOSTIC:");
    eprintln!("   Key: {} bytes", key_len);
    eprintln!("   Nonce: {} bytes", nonce_len);
    eprintln!("   Plaintext: {} bytes", plaintext_len);
    eprintln!("   AAD: {} bytes", aad.len());
    if !aad.is_empty() {
        eprintln!("   AAD (hex): {}", hex::encode(aad));
    }
    eprintln!("   Ciphertext: {} bytes (+ 16-byte tag)", ciphertext_len);
    eprintln!("════════════════════════════════════════════════════════");
}

#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn log_chacha20_poly1305_encrypt(
    _key_len: usize,
    _nonce_len: usize,
    _plaintext_len: usize,
    _aad: &[u8],
    _ciphertext_len: usize,
) {}

/// Diagnostic log for HKDF key derivation
#[cfg(feature = "diagnostics")]
pub fn log_hkdf_derivation(
    input_len: usize,
    salt_len: usize,
    info: &str,
    output_len: usize,
) {
    eprintln!("════════════════════════════════════════════════════════");
    eprintln!("🔑 HKDF KEY DERIVATION DIAGNOSTIC:");
    eprintln!("   Input: {} bytes", input_len);
    eprintln!("   Salt: {} bytes", salt_len);
    eprintln!("   Info: {}", info);
    eprintln!("   Output: {} bytes", output_len);
    eprintln!("════════════════════════════════════════════════════════");
}

#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn log_hkdf_derivation(
    _input_len: usize,
    _salt_len: usize,
    _info: &str,
    _output_len: usize,
) {}

