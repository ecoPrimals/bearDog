//! HSM Cryptographic Provider Types
//!
//! Canonical type definitions for cryptographic providers.

use serde::{Deserialize, Serialize};

/// OpenSSL Cryptographic Provider
///
/// Provides cryptographic operations using the OpenSSL library.
/// Supports hardware acceleration when available through OpenSSL.
///
/// # Features
/// - AES-GCM encryption
/// - RSA operations
/// - ECDSA signing
/// - Hardware acceleration support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenSslCryptoProvider {
    /// Provider identifier
    pub id: String,
}

impl OpenSslCryptoProvider {
    /// Creates a new OpenSSL crypto provider with the given ID
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    /// Creates an OpenSSL crypto provider with default ID
    #[must_use]
    pub fn with_default_id() -> Self {
        Self::new("openssl")
    }
}

impl Default for OpenSslCryptoProvider {
    fn default() -> Self {
        Self::with_default_id()
    }
}

/// RustCrypto Cryptographic Provider
///
/// Pure Rust cryptographic provider using RustCrypto libraries.
/// No unsafe code, fully audited cryptographic primitives.
///
/// # Features
/// - ChaCha20-Poly1305 encryption
/// - Ed25519 signing
/// - Pure Rust (no C dependencies)
/// - Constant-time operations
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct RustCryptoProvider;

impl RustCryptoProvider {
    /// Creates a new RustCrypto provider
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

// ⚠️ RingCryptoProvider REMOVED (January 16, 2026)
//
// **Evolution**: Migrated to 100% Pure Rust using RustCryptoProvider
//
// Ring dependency removed due to:
// - C assembly code (blocks ARM cross-compilation)
// - Violates Pure Rust philosophy
// - RustCrypto provides equivalent functionality with:
//   ✅ 100% Pure Rust (no C dependencies)
//   ✅ ARM cross-compilation support
//   ✅ WebAssembly support
//   ✅ Easier auditing
//   ✅ NCC Group audited
//
// **Migration**: Use `RustCryptoProvider` instead
// **See**: docs/sessions/jan_16_2026/RUSTCRYPTO_MIGRATION_JAN_16_2026.md

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openssl_provider_creation() {
        let provider = OpenSslCryptoProvider::new("test-openssl");
        assert_eq!(provider.id, "test-openssl");
    }

    #[test]
    fn test_openssl_provider_default() {
        let provider = OpenSslCryptoProvider::default();
        assert_eq!(provider.id, "openssl");
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_rust_crypto_provider_creation() {
        let provider = RustCryptoProvider::new();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let _ = provider; // Just ensure it compiles
    }

    // RingCryptoProvider tests removed (January 16, 2026)
    // Evolved to RustCryptoProvider (100% Pure Rust)
}
