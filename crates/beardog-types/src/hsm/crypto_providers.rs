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

/// Ring Cryptographic Provider
///
/// Cryptographic provider using the Ring library.
/// Provides hardware-accelerated operations when available.
///
/// # Features
/// - AES-GCM encryption
/// - Ed25519 signing
/// - Hardware acceleration (AES-NI, etc.)
/// - Minimal API surface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingCryptoProvider {
    /// Provider identifier
    pub id: String,
}

impl RingCryptoProvider {
    /// Creates a new Ring crypto provider with the given ID
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    /// Creates a Ring crypto provider with default ID
    #[must_use]
    pub fn with_default_id() -> Self {
        Self::new("ring")
    }
}

impl Default for RingCryptoProvider {
    fn default() -> Self {
        Self::with_default_id()
    }
}

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

    #[test]
    fn test_rust_crypto_provider_creation() {
        let provider = RustCryptoProvider::new();
        let _ = provider; // Just ensure it compiles
    }

    #[test]
    fn test_ring_provider_creation() {
        let provider = RingCryptoProvider::new("test-ring");
        assert_eq!(provider.id, "test-ring");
    }

    #[test]
    fn test_ring_provider_default() {
        let provider = RingCryptoProvider::default();
        assert_eq!(provider.id, "ring");
    }
}
