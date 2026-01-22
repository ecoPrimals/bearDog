//! Crypto RPC Handler
//!
//! Handles all cryptographic operations exposed via JSON-RPC.
//!
//! # Methods (23 total: 8 core + 4 ECDSA + 4 RSA + 3 TLS + 4 genetic)
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
//! ## ECDSA Signature Algorithms (4 methods)
//! - `crypto.sign_ecdsa_secp256r1` - ECDSA P-256 signing (TLS 1.3)
//! - `crypto.verify_ecdsa_secp256r1` - ECDSA P-256 verification (TLS 1.3)
//! - `crypto.sign_ecdsa_secp384r1` - ECDSA P-384 signing (TLS 1.3)
//! - `crypto.verify_ecdsa_secp384r1` - ECDSA P-384 verification (TLS 1.3)
//!
//! Note: P-521 postponed (< 1% usage, rand_core version conflict with p521 crate)
//!
//! ## RSA Signature Algorithms (4 methods)
//! - `crypto.sign_rsa_pkcs1_sha256` - RSA PKCS#1 v1.5 signing (legacy)
//! - `crypto.verify_rsa_pkcs1_sha256` - RSA PKCS#1 v1.5 verification (legacy)
//! - `crypto.sign_rsa_pss_sha256` - RSA-PSS signing (modern, recommended)
//! - `crypto.verify_rsa_pss_sha256` - RSA-PSS verification (modern, recommended)
//!
//! ## TLS Crypto (3 methods)
//! - `tls.derive_secrets` - HKDF key derivation for TLS
//! - `tls.sign_handshake` - Ed25519 handshake signing
//! - `tls.verify_certificate` - X.509 certificate chain verification
//!
//! ## Genetic Crypto - Phase 5 (4 methods)
//! - `genetic.derive_lineage_key` - Derive keys from genetic family lineage
//! - `genetic.mix_entropy` - Mix entropy across three tiers (Human/Supervised/Machine)
//! - `genetic.verify_lineage` - Verify genetic family relationships
//! - `genetic.generate_lineage_proof` - Generate lineage proof for verification
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
            // Note: P-521 methods commented out (< 1% usage, blocked by rand_core conflict)
            // "crypto.sign_ecdsa_secp521r1",
            // "crypto.verify_ecdsa_secp521r1",
            // RSA signature algorithms (legacy + modern)
            "crypto.sign_rsa_pkcs1_sha256",
            "crypto.verify_rsa_pkcs1_sha256",
            "crypto.sign_rsa_pss_sha256",
            "crypto.verify_rsa_pss_sha256",
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
            // Genetic crypto operations (Phase 5)
            "genetic.derive_lineage_key",
            "genetic.mix_entropy",
            "genetic.verify_lineage",
            "genetic.generate_lineage_proof",
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

            // Note: P-521 match arms commented out (< 1% usage, implementation delayed)
            // "crypto.sign_ecdsa_secp521r1" => {
            //     info!("✍️  Crypto: sign_ecdsa_secp521r1 (ECDSA P-521)");
            //     Err("P-521 not yet implemented (< 1% server usage)".to_string())
            // }
            //
            // "crypto.verify_ecdsa_secp521r1" => {
            //     info!("✅ Crypto: verify_ecdsa_secp521r1 (ECDSA P-521)");
            //     Err("P-521 not yet implemented (< 1% server usage)".to_string())
            // }

            // ====================================================================
            // RSA Signature Algorithms (Legacy + Modern)
            // ====================================================================

            "crypto.sign_rsa_pkcs1_sha256" => {
                info!("✍️  Crypto: sign_rsa_pkcs1_sha256 (RSA PKCS#1 v1.5 - legacy)");
                super::super::crypto_handlers_rsa::handle_sign_rsa_pkcs1_sha256(params).await
            }

            "crypto.verify_rsa_pkcs1_sha256" => {
                info!("✅ Crypto: verify_rsa_pkcs1_sha256 (RSA PKCS#1 v1.5 - legacy)");
                super::super::crypto_handlers_rsa::handle_verify_rsa_pkcs1_sha256(params).await
            }

            "crypto.sign_rsa_pss_sha256" => {
                info!("✍️  Crypto: sign_rsa_pss_sha256 (RSA-PSS - modern, recommended)");
                super::super::crypto_handlers_rsa::handle_sign_rsa_pss_sha256(params).await
            }

            "crypto.verify_rsa_pss_sha256" => {
                info!("✅ Crypto: verify_rsa_pss_sha256 (RSA-PSS - modern, recommended)");
                super::super::crypto_handlers_rsa::handle_verify_rsa_pss_sha256(params).await
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

            // ====================================================================
            // Genetic Crypto Operations - Phase 5 (4 methods)
            // ====================================================================

            "genetic.derive_lineage_key" => {
                info!("🧬 Genetic: derive_lineage_key (lineage-based key derivation)");
                super::super::crypto_handlers_genetic::handle_derive_lineage_key(
                    params.ok_or_else(|| "Parameters required for genetic.derive_lineage_key".to_string())?.clone()
                ).await.map_err(|e| e.to_string())
            }

            "genetic.mix_entropy" => {
                info!("🌱 Genetic: mix_entropy (three-tier entropy hierarchy)");
                super::super::crypto_handlers_genetic::handle_mix_entropy(
                    params.ok_or_else(|| "Parameters required for genetic.mix_entropy".to_string())?.clone()
                ).await.map_err(|e| e.to_string())
            }

            "genetic.verify_lineage" => {
                info!("🔍 Genetic: verify_lineage (family relationship verification)");
                super::super::crypto_handlers_genetic::handle_verify_lineage(
                    params.ok_or_else(|| "Parameters required for genetic.verify_lineage".to_string())?.clone()
                ).await.map_err(|e| e.to_string())
            }

            "genetic.generate_lineage_proof" => {
                info!("🔐 Genetic: generate_lineage_proof (proof generation)");
                super::super::crypto_handlers_genetic::handle_generate_lineage_proof(
                    params.ok_or_else(|| "Parameters required for genetic.generate_lineage_proof".to_string())?.clone()
                ).await.map_err(|e| e.to_string())
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
        
        // Should have 23 methods (8 core + 4 ECDSA + 4 RSA + 3 TLS + 4 genetic)
        assert_eq!(methods.len(), 23);
        
        // Verify all core crypto methods are present
        assert!(methods.contains(&"crypto.sign_ed25519"));
        assert!(methods.contains(&"crypto.verify_ed25519"));
        assert!(methods.contains(&"crypto.x25519_generate_ephemeral"));
        assert!(methods.contains(&"crypto.x25519_derive_secret"));
        assert!(methods.contains(&"crypto.chacha20_poly1305_encrypt"));
        assert!(methods.contains(&"crypto.chacha20_poly1305_decrypt"));
        assert!(methods.contains(&"crypto.blake3_hash"));
        assert!(methods.contains(&"crypto.hmac_sha256"));
        
        // Verify ECDSA methods
        assert!(methods.contains(&"crypto.sign_ecdsa_secp256r1"));
        assert!(methods.contains(&"crypto.verify_ecdsa_secp256r1"));
        assert!(methods.contains(&"crypto.sign_ecdsa_secp384r1"));
        assert!(methods.contains(&"crypto.verify_ecdsa_secp384r1"));
        
        // Verify RSA methods
        assert!(methods.contains(&"crypto.sign_rsa_pkcs1_sha256"));
        assert!(methods.contains(&"crypto.verify_rsa_pkcs1_sha256"));
        assert!(methods.contains(&"crypto.sign_rsa_pss_sha256"));
        assert!(methods.contains(&"crypto.verify_rsa_pss_sha256"));
        
        // Verify TLS methods
        assert!(methods.contains(&"tls.derive_secrets"));
        assert!(methods.contains(&"tls.sign_handshake"));
        assert!(methods.contains(&"tls.verify_certificate"));
        
        // Verify genetic methods (Phase 5)
        assert!(methods.contains(&"genetic.derive_lineage_key"));
        assert!(methods.contains(&"genetic.mix_entropy"));
        assert!(methods.contains(&"genetic.verify_lineage"));
        assert!(methods.contains(&"genetic.generate_lineage_proof"));
    }

    #[test]
    fn test_handler_method_count() {
        let handler = CryptoHandler;
        assert_eq!(handler.methods().len(), 23, "Should have exactly 23 crypto methods (8 core + 4 ECDSA + 4 RSA + 3 TLS + 4 genetic)");
    }
}

