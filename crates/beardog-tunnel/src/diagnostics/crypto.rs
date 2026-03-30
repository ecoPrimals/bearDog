// SPDX-License-Identifier: AGPL-3.0-only

//! Crypto Operation Diagnostics
//!
//! Preserved diagnostic logging for cryptographic operations.
//! Originally from `crypto_handlers_aes_gcm.rs` during HTTPS TLS 1.3 debugging.
//!
//! **FOSSIL RECORD**: These diagnostics were CRITICAL for debugging the
//! 100% Pure Rust HTTPS implementation (January 2026). They helped identify:
//! - Empty AAD causing TLS `decrypt_error`
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
    eprintln!("   Key: {key_len} bytes");
    eprintln!("   Nonce: {nonce_len} bytes");
    eprintln!("   Plaintext: {plaintext_len} bytes");
    eprintln!("   AAD: {} bytes", aad.len());
    if aad.is_empty() {
        eprintln!("   ⚠️  AAD is EMPTY - this might cause TLS decrypt_error!");
    } else {
        eprintln!("   AAD (hex): {}", hex::encode(aad));
    }
    eprintln!("   ✅ Ciphertext: {ciphertext_len} bytes (plaintext + 16-byte tag)");
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
    eprintln!("   Key: {key_len} bytes");
    eprintln!("   Nonce: {nonce_len} bytes");
    eprintln!("   Plaintext: {plaintext_len} bytes");
    eprintln!("   AAD: {} bytes", aad.len());
    if !aad.is_empty() {
        eprintln!("   AAD (hex): {}", hex::encode(aad));
    }
    eprintln!("   Ciphertext: {ciphertext_len} bytes (+ 16-byte tag)");
    eprintln!("════════════════════════════════════════════════════════");
}

/// No-op diagnostic log for AES-256-GCM encryption (diagnostics feature disabled)
#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn log_aes256_gcm_encrypt(
    _key_len: usize,
    _nonce_len: usize,
    _plaintext_len: usize,
    _aad: &[u8],
    _ciphertext_len: usize,
) {
}

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
    eprintln!("   Key: {key_len} bytes");
    eprintln!("   Nonce: {nonce_len} bytes");
    eprintln!("   Plaintext: {plaintext_len} bytes");
    eprintln!("   AAD: {} bytes", aad.len());
    if !aad.is_empty() {
        eprintln!("   AAD (hex): {}", hex::encode(aad));
    }
    eprintln!("   Ciphertext: {ciphertext_len} bytes (+ 16-byte tag)");
    eprintln!("════════════════════════════════════════════════════════");
}

/// No-op diagnostic log for ChaCha20-Poly1305 encryption (diagnostics feature disabled)
#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn log_chacha20_poly1305_encrypt(
    _key_len: usize,
    _nonce_len: usize,
    _plaintext_len: usize,
    _aad: &[u8],
    _ciphertext_len: usize,
) {
}

/// Diagnostic log for ChaCha20-Poly1305 decryption
///
/// **Origin**: `handlers/crypto/symmetric.rs` lines 213-222
/// **Purpose**: Cross-verify decryption params with Songbird
#[cfg(feature = "diagnostics")]
pub fn log_chacha20_poly1305_decrypt(
    key: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
    tag: &[u8],
    aad: Option<&[u8]>,
) {
    eprintln!("════════════════════════════════════════════════════════");
    eprintln!("🔍 BEARDOG RECEIVED - ChaCha20-Poly1305 DECRYPT:");
    eprintln!("   Key (32 bytes): {}", hex::encode(key));
    eprintln!("   Nonce ({} bytes): {}", nonce.len(), hex::encode(nonce));
    eprintln!(
        "   Ciphertext ({} bytes): {}",
        ciphertext.len(),
        hex::encode(ciphertext)
    );
    eprintln!("   Tag ({} bytes): {}", tag.len(), hex::encode(tag));
    if let Some(aad_data) = aad {
        eprintln!(
            "   AAD ({} bytes): {}",
            aad_data.len(),
            hex::encode(aad_data)
        );
    } else {
        eprintln!("   AAD: None");
    }
    eprintln!("════════════════════════════════════════════════════════");
}

/// No-op diagnostic log for ChaCha20-Poly1305 decryption (diagnostics feature disabled)
#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn log_chacha20_poly1305_decrypt(
    _key: &[u8],
    _nonce: &[u8],
    _ciphertext: &[u8],
    _tag: &[u8],
    _aad: Option<&[u8]>,
) {
}

/// Diagnostic log for HKDF key derivation
#[cfg(feature = "diagnostics")]
pub fn log_hkdf_derivation(input_len: usize, salt_len: usize, info: &str, output_len: usize) {
    eprintln!("════════════════════════════════════════════════════════");
    eprintln!("🔑 HKDF KEY DERIVATION DIAGNOSTIC:");
    eprintln!("   Input: {input_len} bytes");
    eprintln!("   Salt: {salt_len} bytes");
    eprintln!("   Info: {info}");
    eprintln!("   Output: {output_len} bytes");
    eprintln!("════════════════════════════════════════════════════════");
}

/// No-op diagnostic log for HKDF key derivation (diagnostics feature disabled)
#[cfg(not(feature = "diagnostics"))]
#[inline(always)]
pub fn log_hkdf_derivation(_input_len: usize, _salt_len: usize, _info: &str, _output_len: usize) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_no_ops_are_callable_without_panicking() {
        log_aes128_gcm_encrypt(32, 12, 64, b"aad", 80);
        log_aes256_gcm_encrypt(32, 12, 64, &[], 80);
        log_chacha20_poly1305_encrypt(32, 12, 100, b"ctx", 116);
        log_chacha20_poly1305_decrypt(&[1u8; 32], &[2; 12], &[3; 8], &[4; 16], Some(b"aad"));
        log_chacha20_poly1305_decrypt(&[1u8; 32], &[2; 12], &[3; 8], &[4; 16], None);
        log_hkdf_derivation(48, 32, "info", 64);
    }
}

#[cfg(all(test, feature = "diagnostics"))]
mod diagnostics_feature_tests {
    use super::*;

    #[test]
    fn diagnostic_verbose_paths_execute() {
        log_aes128_gcm_encrypt(16, 12, 0, b"", 16);
        log_aes128_gcm_encrypt(16, 12, 10, b"aad-bytes", 26);
        log_aes256_gcm_encrypt(32, 12, 5, b"x", 21);
        log_chacha20_poly1305_encrypt(32, 12, 7, b"y", 23);
        log_chacha20_poly1305_decrypt(&[0xff; 32], &[1; 12], &[], &[0; 16], None);
        log_hkdf_derivation(10, 8, "hkdf", 32);
    }
}
