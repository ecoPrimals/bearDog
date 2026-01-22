//! Crypto RPC Handler
//!
//! Handles all cryptographic operations exposed via JSON-RPC.
//!
//! # Methods
//!
//! ## Core Crypto (8 methods)
//! - `crypto.sign_ed25519` - Sign data with Ed25519
//! - `crypto.verify_ed25519` - Verify Ed25519 signature
//! - `crypto.x25519_generate_ephemeral` - Generate ephemeral X25519 keypair
//! - `crypto.x25519_derive_secret` - Derive shared secret via X25519
//! - `crypto.chacha20_poly1305_encrypt` - AEAD encryption
//! - `crypto.chacha20_poly1305_decrypt` - AEAD decryption
//! - `crypto.blake3_hash` - BLAKE3 hashing
//! - `crypto.hmac_sha256` - HMAC-SHA256 message authentication
//!
//! ## ECDSA Signature Algorithms (2 methods)
//! - `crypto.sign_ecdsa_secp256r1` - ECDSA P-256 signing (TLS 1.3)
//! - `crypto.verify_ecdsa_secp256r1` - ECDSA P-256 verification (TLS 1.3)
//!
//! ## TLS Crypto (3 methods)
//! - `tls.derive_secrets` - HKDF key derivation for TLS
//! - `tls.sign_handshake` - Ed25519 handshake signing
//! - `tls.verify_certificate` - X.509 certificate chain verification
//!
//! # Architecture
//!
//! This handler wraps the existing `crypto_handlers` module, providing a
//! clean interface for the handler registry pattern. All crypto operations
//! are delegated to the specialized handlers in `crypto_handlers.rs`.
//!
//! # Performance
//!
//! All operations target < 1ms latency:
//! - Ed25519 operations: ~50-100μs
//! - X25519 key exchange: ~100-200μs
//! - ChaCha20-Poly1305: ~500-800μs per 1KB
//! - BLAKE3: ~300-500μs per 1KB
//! - HKDF: ~50-100μs
//! - X.509 parsing: ~500-800μs per cert

use async_trait::async_trait;
use crate::btsp_provider::BeardogBtspProvider;
use crate::unix_socket_ipc::handlers::MethodHandler;
use std::sync::Arc;
use tracing::info;

/// Crypto RPC handler
///
/// Handles all cryptographic operations for BearDog, including:
/// - Digital signatures (Ed25519)
/// - Key exchange (X25519)
/// - Authenticated encryption (ChaCha20-Poly1305)
/// - Hashing (BLAKE3, HMAC-SHA256)
/// - TLS-specific operations (HKDF, handshake signing, cert verification)
pub struct CryptoHandler;

#[async_trait]
impl MethodHandler for CryptoHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            // Core crypto operations (EdDSA)
            "crypto.sign_ed25519",
            "crypto.verify_ed25519",
            // ECDSA signature algorithms (TLS 1.3)
            "crypto.sign_ecdsa_secp256r1",
            "crypto.verify_ecdsa_secp256r1",
            "crypto.sign_ecdsa_secp384r1",
            "crypto.verify_ecdsa_secp384r1",
            // Key exchange
            "crypto.x25519_generate_ephemeral",
            "crypto.x25519_derive_secret",
            // AEAD encryption
            "crypto.chacha20_poly1305_encrypt",
            "crypto.chacha20_poly1305_decrypt",
            // Hashing
            "crypto.blake3_hash",
            "crypto.hmac_sha256",
            // TLS crypto operations
            "tls.derive_secrets",
            "tls.sign_handshake",
            "tls.verify_certificate",
        ]
    }

    async fn handle(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        match method {
            // ====================================================================
            // Core Crypto Operations (8 methods)
            // ====================================================================

            "crypto.sign_ed25519" => {
                info!("✍️  Crypto: sign_ed25519");
                super::super::crypto_handlers::handle_sign_ed25519(params).await
            }

            "crypto.verify_ed25519" => {
                info!("✅ Crypto: verify_ed25519");
                super::super::crypto_handlers::handle_verify_ed25519(params).await
            }

            // ====================================================================
            // ECDSA Signature Algorithms (TLS 1.3 Support)
            // ====================================================================

            "crypto.sign_ecdsa_secp256r1" => {
                info!("✍️  Crypto: sign_ecdsa_secp256r1 (ECDSA P-256 for TLS 1.3)");
                super::super::crypto_handlers_ecdsa::handle_sign_ecdsa_secp256r1(params).await
            }

            "crypto.verify_ecdsa_secp256r1" => {
                info!("✅ Crypto: verify_ecdsa_secp256r1 (ECDSA P-256 for TLS 1.3)");
                super::super::crypto_handlers_ecdsa::handle_verify_ecdsa_secp256r1(params).await
            }

            "crypto.sign_ecdsa_secp384r1" => {
                info!("✍️  Crypto: sign_ecdsa_secp384r1 (ECDSA P-384 for TLS 1.3)");
                super::super::crypto_handlers_ecdsa::handle_sign_ecdsa_secp384r1(params).await
            }

            "crypto.verify_ecdsa_secp384r1" => {
                info!("✅ Crypto: verify_ecdsa_secp384r1 (ECDSA P-384 for TLS 1.3)");
                super::super::crypto_handlers_ecdsa::handle_verify_ecdsa_secp384r1(params).await
            }

            // ====================================================================
            // Key Exchange
            // ====================================================================

            "crypto.x25519_generate_ephemeral" => {
                info!("🔑 Crypto: x25519_generate_ephemeral");
                super::super::crypto_handlers::handle_x25519_generate_ephemeral(params).await
            }

            "crypto.x25519_derive_secret" => {
                info!("🤝 Crypto: x25519_derive_secret (ECDH key exchange)");
                super::super::crypto_handlers::handle_x25519_derive_secret(params).await
            }

            "crypto.chacha20_poly1305_encrypt" => {
                info!("🔒 Crypto: chacha20_poly1305_encrypt (AEAD)");
                super::super::crypto_handlers::handle_chacha20_poly1305_encrypt(params).await
            }

            "crypto.chacha20_poly1305_decrypt" => {
                info!("🔓 Crypto: chacha20_poly1305_decrypt (AEAD)");
                super::super::crypto_handlers::handle_chacha20_poly1305_decrypt(params).await
            }

            "crypto.blake3_hash" => {
                info!("🔍 Crypto: blake3_hash");
                super::super::crypto_handlers::handle_blake3_hash(params).await
            }

            "crypto.hmac_sha256" => {
                info!("🔏 Crypto: hmac_sha256");
                super::super::crypto_handlers::handle_hmac_sha256(params).await
            }

            // ====================================================================
            // TLS Crypto Operations (3 methods)
            // ====================================================================

            "tls.derive_secrets" => {
                info!("🔑 TLS: derive_secrets (HKDF key derivation)");
                super::super::crypto_handlers::handle_tls_derive_secrets(params).await
            }

            "tls.sign_handshake" => {
                info!("✍️  TLS: sign_handshake (Ed25519 handshake signing)");
                super::super::crypto_handlers::handle_tls_sign_handshake(params).await
            }

            "tls.verify_certificate" => {
                info!("🔍 TLS: verify_certificate (X.509 chain verification)");
                super::super::crypto_handlers::handle_tls_verify_certificate(params).await
            }

            _ => Err(format!("Unknown crypto method: {}", method)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_handler_methods() {
        let handler = CryptoHandler;
        let methods = handler.methods();
        
        // Should have 11 methods (8 crypto + 3 TLS)
        assert_eq!(methods.len(), 11);
        
        // Verify all core crypto methods are present
        assert!(methods.contains(&"crypto.sign_ed25519"));
        assert!(methods.contains(&"crypto.verify_ed25519"));
        assert!(methods.contains(&"crypto.x25519_generate_ephemeral"));
        assert!(methods.contains(&"crypto.x25519_derive_secret"));
        assert!(methods.contains(&"crypto.chacha20_poly1305_encrypt"));
        assert!(methods.contains(&"crypto.chacha20_poly1305_decrypt"));
        assert!(methods.contains(&"crypto.blake3_hash"));
        assert!(methods.contains(&"crypto.hmac_sha256"));
        
        // Verify all TLS methods are present
        assert!(methods.contains(&"tls.derive_secrets"));
        assert!(methods.contains(&"tls.sign_handshake"));
        assert!(methods.contains(&"tls.verify_certificate"));
    }

    #[test]
    fn test_handler_method_count() {
        let handler = CryptoHandler;
        assert_eq!(handler.methods().len(), 11, "Should have exactly 11 crypto methods");
    }
}

