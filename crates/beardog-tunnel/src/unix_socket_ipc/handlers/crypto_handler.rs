//! Crypto RPC Handler
//!
//! Handles all cryptographic operations exposed via JSON-RPC.
//!
//! # Semantic Method Support (TRUE PRIMAL Pattern)
//!
//! **Primary**: Actual method names (e.g., `crypto.x25519_generate_ephemeral`)
//! **Optional**: Semantic aliases (e.g., `generate_keypair`)
//!
//! Semantic names allow direct testing without Neural API, but in production,
//! Neural API handles all translation via the graph (`tower_atomic_bootstrap.toml`).
//!
//! Both forms route to the same handler:
//! - `crypto.x25519_generate_ephemeral` (actual) → handle_x25519_generate
//! - `generate_keypair` (semantic) → handle_x25519_generate
//!
//! # Methods (91 total - Feb 2026 Deep Debt Evolution)
//! ## See test_crypto_handler_methods() for full breakdown
//!
//! ## Core Crypto (20 methods)
//! - `crypto.sign_ed25519` - Sign data with Ed25519
//! - `crypto.verify_ed25519` - Verify Ed25519 signature
//! - `crypto.x25519_generate_ephemeral` - Generate ephemeral X25519 keypair
//! - `crypto.x25519_derive_secret` - Derive shared secret via X25519
//! - `crypto.ecdh_p256_generate` - Generate P-256 ECDH keypair (Phase 6 - TLS 1.3)
//! - `crypto.ecdh_p256_derive` - P-256 ECDH key exchange (Phase 6 - TLS 1.3)
//! - `crypto.ecdh_p384_generate` - Generate P-384 ECDH keypair (Phase 6 - TLS 1.3)
//! - `crypto.ecdh_p384_derive` - P-384 ECDH key exchange (Phase 6 - TLS 1.3)
//! - `crypto.chacha20_poly1305_encrypt` - AEAD encryption
//! - `crypto.chacha20_poly1305_decrypt` - AEAD decryption
//! - `crypto.aes256_gcm_encrypt` - AES-256-GCM encryption (Phase 6 - 90%+ of HTTPS!)
//! - `crypto.aes256_gcm_decrypt` - AES-256-GCM decryption (Phase 6 - 90%+ of HTTPS!)
//! - `crypto.aes128_gcm_encrypt` - AES-128-GCM encryption (Phase 6 - 80%+ of HTTPS!)
//! - `crypto.aes128_gcm_decrypt` - AES-128-GCM decryption (Phase 6 - 80%+ of HTTPS!)
//! - `crypto.blake3_hash` - BLAKE3 hashing
//! - `crypto.hmac_sha256` - HMAC-SHA256 message authentication
//! - `crypto.hash_for_cipher` - Cipher-aware hashing for TLS 1.3 (SHA-256 or SHA-384)
//! - `crypto.sha256` - SHA-256 hashing (Phase 6)
//! - `crypto.sha384` - SHA-384 hashing (Phase 6)
//! - `crypto.sha512` - SHA-512 hashing (Phase 6)
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
//! ## TLS Crypto (4 methods)
//! - `tls.derive_secrets` - HKDF handshake key derivation for TLS (handshake traffic keys)
//! - `tls.derive_application_secrets` - HKDF application key derivation for TLS (HTTP data keys)
//! - `tls.sign_handshake` - Ed25519 handshake signing
//! - `tls.verify_certificate` - X.509 certificate chain verification
//!
//! ## Genetic Crypto - Phase 5 (8 methods)
//! - `genetic.derive_lineage_key` - Derive keys from genetic family lineage
//! - `genetic.derive_lineage_beacon_key` - Derive BirdSong beacon key (TRUE Dark Forest)
//! - `genetic.mix_entropy` - Mix entropy across three tiers (Human/Supervised/Machine)
//! - `genetic.verify_lineage` - Verify genetic family relationships
//! - `genetic.generate_lineage_proof` - Generate lineage proof for verification
//! - `genetic.generate_challenge` - Generate challenge for lineage verification (Dark Forest)
//! - `genetic.respond_to_challenge` - Respond to lineage challenge (Dark Forest)
//! - `genetic.verify_challenge_response` - Verify challenge response (Dark Forest)
//!
//! ## Password Hashing - Phase 6 (3 methods)
//! - `crypto.argon2id_hash` - Modern password hashing (OWASP recommended, memory-hard)
//! - `crypto.argon2id_verify` - Verify Argon2id password hashes (constant-time)
//! - `crypto.pbkdf2_sha256` - Legacy PBKDF2 key derivation (iOS/macOS/WiFi)
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

use crate::btsp_provider::BeardogBtspProvider;
use crate::unix_socket_ipc::handlers::MethodHandler;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;

// Import refactored crypto handlers from the new crypto module
// The handlers are re-exported from crypto/mod.rs for easy access
use crate::unix_socket_ipc::handlers::crypto::{
    // TLS 1.2 handlers (Jan 27, 2026 - Tower Atomic Pattern for Songbird)
    handle_aes_128_gcm_decrypt,
    handle_aes_128_gcm_encrypt,
    handle_aes_256_gcm_decrypt,
    handle_aes_256_gcm_encrypt,
    // Hash handlers
    handle_blake3_hash,
    // Symmetric crypto handlers
    handle_chacha20_poly1305_decrypt,
    handle_chacha20_poly1305_encrypt,
    handle_ecdhe_p256_compute_shared,
    handle_ecdhe_p256_generate,
    handle_ecdhe_p384_compute_shared,
    handle_ecdhe_p384_generate,
    handle_ed25519_generate_keypair,
    handle_hash_for_cipher,
    handle_hmac_sha256,
    // Asymmetric crypto handlers
    handle_sign_ed25519,
    handle_tls12_prf,
    // TLS 1.3 handlers
    handle_tls_compute_finished_verify_data,
    handle_tls_derive_application_secrets,
    handle_tls_derive_handshake_secrets,
    handle_tls_derive_secrets,
    handle_tls_sign_handshake,
    handle_tls_verify_certificate,
    handle_verify_ed25519,
    handle_x25519_derive_secret,
    handle_x25519_generate_ephemeral,
};

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
            "crypto.ed25519_generate_keypair",
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
            // Key exchange (X25519 + ECDH)
            "crypto.x25519_generate_ephemeral",
            "crypto.x25519_derive_secret",
            "crypto.ecdh_p256_generate",
            "crypto.ecdh_p256_derive",
            "crypto.ecdh_p384_generate",
            "crypto.ecdh_p384_derive",
            // AEAD encryption
            "crypto.chacha20_poly1305_encrypt",
            "crypto.chacha20_poly1305_decrypt",
            "crypto.aes256_gcm_encrypt",
            "crypto.aes256_gcm_decrypt",
            "crypto.aes128_gcm_encrypt",
            "crypto.aes128_gcm_decrypt",
            // Note: AES Legacy Modes (CBC/CTR/XTS) deferred due to RustCrypto RC version conflicts
            // Hashing
            "crypto.blake3_hash",
            "crypto.hmac_sha256",
            "crypto.hash_for_cipher", // SHA-384 evolution: cipher-aware hashing
            "crypto.sha256",
            "crypto.sha384",
            "crypto.sha512",
            "crypto.sha1",     // Phase 7: Legacy (Git compatibility)
            "crypto.sha3_256", // Phase 7: Modern quantum-resistant
            // HMAC variants (Phase 7)
            "crypto.hmac_sha384",
            "crypto.hmac_sha512",
            "crypto.hmac_blake3",
            // Password hashing
            "crypto.argon2id_hash",
            "crypto.argon2id_verify",
            "crypto.pbkdf2_sha256",
            // Additional KDFs (Phase 7)
            "crypto.bcrypt_hash",
            "crypto.bcrypt_verify",
            "crypto.scrypt",
            // TLS crypto operations
            "tls.derive_secrets",
            "tls.derive_handshake_secrets",
            "tls.derive_application_secrets",
            "tls.compute_finished_verify_data",
            "tls.sign_handshake",
            "tls.verify_certificate",
            // TLS 1.2 crypto operations (Semantic Naming - Jan 27, 2026)
            "crypto.ecdhe.p256.generate",
            "crypto.ecdhe.p256.compute_shared",
            "crypto.ecdhe.p384.generate",
            "crypto.ecdhe.p384.compute_shared",
            "crypto.aead.aes_128_gcm.encrypt",
            "crypto.aead.aes_128_gcm.decrypt",
            "crypto.aead.aes_256_gcm.encrypt",
            "crypto.aead.aes_256_gcm.decrypt",
            "crypto.kdf.tls12_prf",
            // Genetic crypto operations (Phase 5)
            "genetic.derive_lineage_key",
            "genetic.mix_entropy",
            "genetic.verify_lineage",
            "genetic.generate_lineage_proof",
            "genetic.generate_challenge",
            "genetic.respond_to_challenge",
            "genetic.verify_challenge_response",
            // Device enrollment (Deep Debt: DERIVE, not COPY)
            "genetic.derive_device_seed",
            "genetic.sign_lineage_certificate",
            "genetic.verify_lineage_certificate",
            // ═══════════════════════════════════════════════════════════════
            // Semantic Aliases (Phase 2 - wateringHole Standard)
            // Added: January 27, 2026
            // ═══════════════════════════════════════════════════════════════
            // These provide semantic names for common operations, routing to
            // the most commonly-used algorithms as sensible defaults.
            // Production systems should use Neural API translation for
            // full semantic flexibility.
            "crypto.hash",             // → blake3_hash (default, fastest)
            "crypto.hmac",             // → hmac_sha256 (default, widely used)
            "crypto.sign",             // → sign_ed25519 (default, modern)
            "crypto.verify",           // → verify_ed25519 (default, modern)
            "crypto.encrypt",          // → chacha20_poly1305_encrypt (default, fast)
            "crypto.decrypt",          // → chacha20_poly1305_decrypt (default, fast)
            "crypto.generate_keypair", // → x25519_generate_ephemeral (default, ECDH)
            "crypto.derive_secret",    // → x25519_derive_secret (default, ECDH)
            // ═══════════════════════════════════════════════════════════════
            // Cross-Primal Namespace (beardog.crypto.*)
            // Added: February 6, 2026
            // Purpose: Support Songbird's Sovereign Onion Service crypto delegation
            // Pattern: Same as TLS 1.3 delegation (TRUE PRIMAL compliance)
            // ═══════════════════════════════════════════════════════════════
            "beardog.crypto.sha3_256",               // Onion address derivation (Tor v3)
            "beardog.crypto.ed25519_generate_keypair", // Onion identity generation
            "beardog.crypto.sign_ed25519",           // Onion identity keys
            "beardog.crypto.verify_ed25519",         // Identity verification
            "beardog.crypto.x25519_generate_ephemeral", // Session key generation
            "beardog.crypto.x25519_derive_secret",   // ECDH shared secret
            "beardog.crypto.chacha20_poly1305_encrypt", // Data encryption
            "beardog.crypto.chacha20_poly1305_decrypt", // Data decryption
            "beardog.crypto.hmac_sha256",            // HKDF for session keys
            "beardog.crypto.blake3_hash",            // General hashing
            // Tor v3 Onion Address Operations (Feb 7, 2026)
            "beardog.crypto.derive_onion_address",   // Tor v3 onion address from Ed25519 pubkey
            "beardog.crypto.generate_onion_identity", // Generate identity + derive address
            // ═══════════════════════════════════════════════════════════════
            // Tor Phase 2: Pure Rust Tor Protocol (Feb 2026)
            // ntor handshake, cell encryption, Tor-specific KDF
            // ═══════════════════════════════════════════════════════════════
            "beardog.crypto.tor_ntor_client_init",    // Initialize client ntor handshake
            "beardog.crypto.tor_ntor_client_finish",  // Complete client ntor handshake
            "beardog.crypto.tor_ntor_server_respond", // Server-side ntor response
            "beardog.crypto.tor_cell_encrypt",        // ChaCha20 cell encryption
            "beardog.crypto.tor_cell_decrypt",        // ChaCha20 cell decryption
            "beardog.crypto.tor_kdf",                 // Tor-specific HKDF expansion
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
            // Core Crypto Operations (9 methods) - Ed25519 keypair added Feb 6, 2026
            // ====================================================================
            "crypto.ed25519_generate_keypair" => {
                info!("🔑 Crypto: ed25519_generate_keypair (Songbird Onion Identity)");
                handle_ed25519_generate_keypair(params).await
            }

            "crypto.sign_ed25519" => {
                info!("✍️  Crypto: sign_ed25519");
                handle_sign_ed25519(params).await
            }

            "crypto.verify_ed25519" => {
                info!("✅ Crypto: verify_ed25519");
                handle_verify_ed25519(params).await
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
                handle_x25519_generate_ephemeral(params).await
            }

            "crypto.x25519_derive_secret" => {
                info!("🤝 Crypto: x25519_derive_secret (ECDH key exchange)");
                handle_x25519_derive_secret(params).await
            }

            "crypto.ecdh_p256_generate" => {
                info!("🔑 Crypto: ecdh_p256_generate (P-256 keypair for TLS 1.3)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_ecdh::handle_ecdh_p256_generate(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.ecdh_p256_derive" => {
                info!("🤝 Crypto: ecdh_p256_derive (P-256 ECDH key exchange for TLS 1.3)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_ecdh::handle_ecdh_p256_derive(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.ecdh_p384_generate" => {
                info!("🔑 Crypto: ecdh_p384_generate (P-384 keypair for TLS 1.3)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_ecdh::handle_ecdh_p384_generate(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.ecdh_p384_derive" => {
                info!("🤝 Crypto: ecdh_p384_derive (P-384 ECDH key exchange for TLS 1.3)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_ecdh::handle_ecdh_p384_derive(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.chacha20_poly1305_encrypt" => {
                info!("🔒 Crypto: chacha20_poly1305_encrypt (AEAD)");
                handle_chacha20_poly1305_encrypt(params).await
            }

            "crypto.chacha20_poly1305_decrypt" => {
                info!("🔓 Crypto: chacha20_poly1305_decrypt (AEAD)");
                handle_chacha20_poly1305_decrypt(params).await
            }

            "crypto.aes256_gcm_encrypt" => {
                info!("🔒 Crypto: aes256_gcm_encrypt (90%+ of HTTPS!)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_aes_gcm::handle_aes256_gcm_encrypt(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.aes256_gcm_decrypt" => {
                info!("🔓 Crypto: aes256_gcm_decrypt (90%+ of HTTPS!)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_aes_gcm::handle_aes256_gcm_decrypt(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.aes128_gcm_encrypt" => {
                info!("🔒 Crypto: aes128_gcm_encrypt (80%+ of HTTPS!)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_aes_gcm::handle_aes128_gcm_encrypt(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.aes128_gcm_decrypt" => {
                info!("🔓 Crypto: aes128_gcm_decrypt (80%+ of HTTPS!)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_aes_gcm::handle_aes128_gcm_decrypt(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.blake3_hash" => {
                info!("🔍 Crypto: blake3_hash");
                handle_blake3_hash(params).await
            }

            "crypto.hmac_sha256" => {
                info!("🔏 Crypto: hmac_sha256");
                handle_hmac_sha256(params).await
            }

            "crypto.hash_for_cipher" => {
                info!("🎯 Crypto: hash_for_cipher (cipher-aware hashing for TLS 1.3)");
                handle_hash_for_cipher(params).await
            }

            "crypto.sha256" => {
                info!("🔍 Crypto: sha256 (SHA-256 hashing)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_hashing::handle_sha256(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.sha384" => {
                info!("🔍 Crypto: sha384 (SHA-384 hashing)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_hashing::handle_sha384(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.sha512" => {
                info!("🔍 Crypto: sha512 (SHA-512 hashing)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_hashing::handle_sha512(params_ref)
                    .map_err(|e| e.to_string())
            }

            // Phase 7: SHA-1 (legacy Git compatibility)
            "crypto.sha1" => {
                info!("⚠️  Crypto: sha1 (LEGACY - Git compatibility only!)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_hashing::handle_sha1(params_ref)
                    .map_err(|e| e.to_string())
            }

            // Phase 7: SHA3-256 (modern quantum-resistant)
            "crypto.sha3_256" => {
                info!("🔬 Crypto: sha3_256 (modern quantum-resistant hashing)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_hashing::handle_sha3_256(params_ref)
                    .map_err(|e| e.to_string())
            }

            // Phase 7: HMAC variants
            "crypto.hmac_sha384" => {
                info!("🔐 Crypto: hmac_sha384 (high-security MAC)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_hmac::handle_hmac_sha384(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.hmac_sha512" => {
                info!("🔐 Crypto: hmac_sha512 (maximum-security MAC)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_hmac::handle_hmac_sha512(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.hmac_blake3" => {
                info!("⚡ Crypto: hmac_blake3 (modern high-performance MAC)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_hmac::handle_hmac_blake3(params_ref)
                    .map_err(|e| e.to_string())
            }

            // ====================================================================
            // Password Hashing (3 methods - Phase 6)
            // ====================================================================
            "crypto.argon2id_hash" => {
                info!("🔒 Crypto: argon2id_hash (OWASP password hashing)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_passwords::handle_argon2id_hash(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.argon2id_verify" => {
                info!("🔓 Crypto: argon2id_verify (password verification)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_passwords::handle_argon2id_verify(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.pbkdf2_sha256" => {
                info!("🔑 Crypto: pbkdf2_sha256 (legacy password derivation)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_passwords::handle_pbkdf2_sha256(params_ref)
                    .map_err(|e| e.to_string())
            }

            // Phase 7: Additional KDFs (bcrypt, scrypt)
            "crypto.bcrypt_hash" => {
                info!("🔐 Crypto: bcrypt_hash (legacy password hashing)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_kdf::handle_bcrypt_hash(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.bcrypt_verify" => {
                info!("🔐 Crypto: bcrypt_verify (legacy password verification)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_kdf::handle_bcrypt_verify(params_ref)
                    .map_err(|e| e.to_string())
            }

            "crypto.scrypt" => {
                info!("🔑 Crypto: scrypt (memory-hard KDF)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_kdf::handle_scrypt(params_ref)
                    .map_err(|e| e.to_string())
            }

            // ====================================================================
            // TLS Crypto Operations (4 methods)
            // ====================================================================
            "tls.derive_secrets" => {
                info!("🔑 TLS: derive_secrets (HKDF handshake key derivation - legacy)");
                handle_tls_derive_secrets(params).await
            }

            "tls.derive_handshake_secrets" => {
                info!("🔑 TLS: derive_handshake_secrets (RFC 8446 handshake key derivation)");
                handle_tls_derive_handshake_secrets(params).await
            }

            "tls.derive_application_secrets" => {
                info!("🔑 TLS: derive_application_secrets (RFC 8446 application key derivation for HTTP)");
                handle_tls_derive_application_secrets(params).await
            }

            "tls.compute_finished_verify_data" => {
                info!("🏁 TLS: compute_finished_verify_data (RFC 8446 Section 4.4.4 - Finished message)");
                handle_tls_compute_finished_verify_data(params).await
            }

            "tls.sign_handshake" => {
                info!("✍️  TLS: sign_handshake (Ed25519 handshake signing)");
                handle_tls_sign_handshake(params).await
            }

            "tls.verify_certificate" => {
                info!("🔍 TLS: verify_certificate (X.509 chain verification)");
                handle_tls_verify_certificate(params).await
            }

            // ====================================================================
            // TLS 1.2 Crypto Operations (9 methods - Jan 27, 2026)
            // Tower Atomic Pattern for Songbird integration
            // ====================================================================
            "crypto.ecdhe.p256.generate" => {
                info!("🔑 Crypto: ecdhe.p256.generate (TLS 1.2 P-256 keypair for Songbird)");
                handle_ecdhe_p256_generate(params).await
            }

            "crypto.ecdhe.p256.compute_shared" => {
                info!("🤝 Crypto: ecdhe.p256.compute_shared (TLS 1.2 P-256 ECDH for Songbird)");
                handle_ecdhe_p256_compute_shared(params).await
            }

            "crypto.ecdhe.p384.generate" => {
                info!("🔑 Crypto: ecdhe.p384.generate (TLS 1.2 P-384 keypair for Songbird)");
                handle_ecdhe_p384_generate(params).await
            }

            "crypto.ecdhe.p384.compute_shared" => {
                info!("🤝 Crypto: ecdhe.p384.compute_shared (TLS 1.2 P-384 ECDH for Songbird)");
                handle_ecdhe_p384_compute_shared(params).await
            }

            "crypto.aead.aes_128_gcm.encrypt" => {
                info!("🔒 Crypto: aead.aes_128_gcm.encrypt (TLS 1.2 AES-128-GCM for Songbird)");
                handle_aes_128_gcm_encrypt(params).await
            }

            "crypto.aead.aes_128_gcm.decrypt" => {
                info!("🔓 Crypto: aead.aes_128_gcm.decrypt (TLS 1.2 AES-128-GCM for Songbird)");
                handle_aes_128_gcm_decrypt(params).await
            }

            "crypto.aead.aes_256_gcm.encrypt" => {
                info!("🔒 Crypto: aead.aes_256_gcm.encrypt (TLS 1.2 AES-256-GCM for Songbird)");
                handle_aes_256_gcm_encrypt(params).await
            }

            "crypto.aead.aes_256_gcm.decrypt" => {
                info!("🔓 Crypto: aead.aes_256_gcm.decrypt (TLS 1.2 AES-256-GCM for Songbird)");
                handle_aes_256_gcm_decrypt(params).await
            }

            "crypto.kdf.tls12_prf" => {
                info!("🔑 Crypto: kdf.tls12_prf (TLS 1.2 PRF key expansion for Songbird)");
                handle_tls12_prf(params).await
            }

            // ====================================================================
            // Genetic Crypto Operations - Phase 5 (4 methods)
            // ====================================================================
            "genetic.derive_lineage_key" => {
                info!("🧬 Genetic: derive_lineage_key (lineage-based key derivation)");
                super::super::crypto_handlers_genetic::handle_derive_lineage_key(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.derive_lineage_key".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())
            }

            "genetic.derive_lineage_beacon_key" => {
                info!("🌑 Genetic: derive_lineage_beacon_key (TRUE Dark Forest beacon key)");
                super::super::crypto_handlers_genetic::handle_derive_lineage_beacon_key(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.derive_lineage_beacon_key".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())
            }

            "genetic.mix_entropy" => {
                info!("🌱 Genetic: mix_entropy (three-tier entropy hierarchy)");
                super::super::crypto_handlers_genetic::handle_mix_entropy(
                    params
                        .ok_or_else(|| "Parameters required for genetic.mix_entropy".to_string())?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())
            }

            "genetic.verify_lineage" => {
                info!("🔍 Genetic: verify_lineage (family relationship verification)");
                super::super::crypto_handlers_genetic::handle_verify_lineage(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.verify_lineage".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())
            }

            "genetic.generate_lineage_proof" => {
                info!("🔐 Genetic: generate_lineage_proof (proof generation)");
                super::super::crypto_handlers_genetic::handle_generate_lineage_proof(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.generate_lineage_proof".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())
            }

            "genetic.generate_challenge" => {
                info!("🎲 Genetic: generate_challenge (challenge generation)");
                super::super::crypto_handlers_genetic::handle_generate_challenge(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.generate_challenge".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())
            }

            "genetic.respond_to_challenge" => {
                info!("🔐 Genetic: respond_to_challenge (challenge response)");
                super::super::crypto_handlers_genetic::handle_respond_to_challenge(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.respond_to_challenge".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())
            }

            "genetic.verify_challenge_response" => {
                info!("🔍 Genetic: verify_challenge_response (response verification)");
                super::super::crypto_handlers_genetic::handle_verify_challenge_response(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.verify_challenge_response".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())
            }

            // ====================================================================
            // Device Enrollment (Deep Debt: DERIVE, not COPY)
            // Added: February 5, 2026
            // ====================================================================
            "genetic.derive_device_seed" => {
                info!("🧬 Genetic: derive_device_seed (unique device derivation)");
                super::super::crypto_handlers_genetic::handle_derive_device_seed(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.derive_device_seed".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())
            }

            "genetic.sign_lineage_certificate" => {
                info!("🧬 Genetic: sign_lineage_certificate (device enrollment)");
                super::super::crypto_handlers_genetic::handle_sign_lineage_certificate(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.sign_lineage_certificate".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())
            }

            "genetic.verify_lineage_certificate" => {
                info!("🔍 Genetic: verify_lineage_certificate (certificate verification)");
                super::super::crypto_handlers_genetic::handle_verify_lineage_certificate(
                    params
                        .ok_or_else(|| {
                            "Parameters required for genetic.verify_lineage_certificate".to_string()
                        })?
                        .clone(),
                )
                .await
                .map_err(|e| e.to_string())
            }

            // ====================================================================
            // Semantic Aliases (Phase 2 - wateringHole Standard)
            // Added: January 27, 2026
            // ====================================================================
            // These route to the most commonly-used algorithm as a sensible default.
            // Neural API provides full semantic flexibility in production.
            "crypto.hash" => {
                info!("🔐 Crypto: hash (semantic → blake3_hash)");
                handle_blake3_hash(params).await
            }

            "crypto.hmac" => {
                info!("🔐 Crypto: hmac (semantic → hmac_sha256)");
                handle_hmac_sha256(params).await
            }

            "crypto.sign" => {
                info!("✍️  Crypto: sign (semantic → sign_ed25519)");
                handle_sign_ed25519(params).await
            }

            "crypto.verify" => {
                info!("✅ Crypto: verify (semantic → verify_ed25519)");
                handle_verify_ed25519(params).await
            }

            "crypto.encrypt" => {
                info!("🔒 Crypto: encrypt (semantic → chacha20_poly1305_encrypt)");
                handle_chacha20_poly1305_encrypt(params).await
            }

            "crypto.decrypt" => {
                info!("🔓 Crypto: decrypt (semantic → chacha20_poly1305_decrypt)");
                handle_chacha20_poly1305_decrypt(params).await
            }

            "crypto.generate_keypair" => {
                info!("🔑 Crypto: generate_keypair (semantic → x25519_generate_ephemeral)");
                handle_x25519_generate_ephemeral(params).await
            }

            "crypto.derive_secret" => {
                info!("🔐 Crypto: derive_secret (semantic → x25519_derive_secret)");
                handle_x25519_derive_secret(params).await
            }

            // ====================================================================
            // Cross-Primal Namespace (beardog.crypto.*)
            // Added: February 6, 2026
            // Purpose: Support Songbird's Sovereign Onion Service crypto delegation
            // Pattern: Same as TLS 1.3 delegation (TRUE PRIMAL compliance)
            // ====================================================================
            "beardog.crypto.sha3_256" => {
                info!("🧅 Crypto: beardog.crypto.sha3_256 (Songbird Onion Service)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_hashing::handle_sha3_256(params_ref)
                    .map_err(|e| e.to_string())
            }

            "beardog.crypto.ed25519_generate_keypair" => {
                info!("🧅 Crypto: beardog.crypto.ed25519_generate_keypair (Songbird Onion Identity)");
                handle_ed25519_generate_keypair(params).await
            }

            "beardog.crypto.sign_ed25519" => {
                info!("🧅 Crypto: beardog.crypto.sign_ed25519 (Songbird Onion Service)");
                handle_sign_ed25519(params).await
            }

            "beardog.crypto.verify_ed25519" => {
                info!("🧅 Crypto: beardog.crypto.verify_ed25519 (Songbird Onion Service)");
                handle_verify_ed25519(params).await
            }

            "beardog.crypto.x25519_generate_ephemeral" => {
                info!("🧅 Crypto: beardog.crypto.x25519_generate_ephemeral (Songbird Onion Service)");
                handle_x25519_generate_ephemeral(params).await
            }

            "beardog.crypto.x25519_derive_secret" => {
                info!("🧅 Crypto: beardog.crypto.x25519_derive_secret (Songbird Onion Service)");
                handle_x25519_derive_secret(params).await
            }

            "beardog.crypto.chacha20_poly1305_encrypt" => {
                info!("🧅 Crypto: beardog.crypto.chacha20_poly1305_encrypt (Songbird Onion Service)");
                handle_chacha20_poly1305_encrypt(params).await
            }

            "beardog.crypto.chacha20_poly1305_decrypt" => {
                info!("🧅 Crypto: beardog.crypto.chacha20_poly1305_decrypt (Songbird Onion Service)");
                handle_chacha20_poly1305_decrypt(params).await
            }

            "beardog.crypto.hmac_sha256" => {
                info!("🧅 Crypto: beardog.crypto.hmac_sha256 (Songbird Onion Service)");
                handle_hmac_sha256(params).await
            }

            "beardog.crypto.blake3_hash" => {
                info!("🧅 Crypto: beardog.crypto.blake3_hash (Songbird Onion Service)");
                handle_blake3_hash(params).await
            }

            // Tor v3 Onion Address Operations (Feb 7, 2026)
            "beardog.crypto.derive_onion_address" => {
                info!("🧅 Crypto: beardog.crypto.derive_onion_address (Tor v3)");
                let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
                super::super::crypto_handlers_hashing::handle_derive_onion_address(params_ref)
                    .map_err(|e| e.to_string())
            }

            "beardog.crypto.generate_onion_identity" => {
                info!("🧅 Crypto: beardog.crypto.generate_onion_identity (Tor v3)");
                super::super::crypto_handlers_hashing::handle_generate_onion_identity(params).await
            }

            // ====================================================================
            // Tor Phase 2: Pure Rust Tor Protocol (6 methods)
            // ntor handshake, cell encryption, Tor-specific KDF
            // ====================================================================
            "beardog.crypto.tor_ntor_client_init" => {
                info!("🧅 Crypto: tor_ntor_client_init (Tor ntor handshake - client init)");
                super::super::crypto_handlers_tor::handle_tor_ntor_client_init(params).await
            }

            "beardog.crypto.tor_ntor_client_finish" => {
                info!("🧅 Crypto: tor_ntor_client_finish (Tor ntor handshake - client finish)");
                super::super::crypto_handlers_tor::handle_tor_ntor_client_finish(params).await
            }

            "beardog.crypto.tor_ntor_server_respond" => {
                info!("🧅 Crypto: tor_ntor_server_respond (Tor ntor handshake - server)");
                super::super::crypto_handlers_tor::handle_tor_ntor_server_respond(params).await
            }

            "beardog.crypto.tor_cell_encrypt" => {
                info!("🧅 Crypto: tor_cell_encrypt (Tor relay cell encryption)");
                super::super::crypto_handlers_tor::handle_tor_cell_encrypt(params).await
            }

            "beardog.crypto.tor_cell_decrypt" => {
                info!("🧅 Crypto: tor_cell_decrypt (Tor relay cell decryption)");
                super::super::crypto_handlers_tor::handle_tor_cell_decrypt(params).await
            }

            "beardog.crypto.tor_kdf" => {
                info!("🧅 Crypto: tor_kdf (Tor key derivation)");
                super::super::crypto_handlers_tor::handle_tor_kdf(params).await
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

        // Should have 91 methods (Phase 1-8 + TLS 1.2 + Dark Forest + Device Enrollment + Onion Service + Tor v3 + Tor Phase 2)
        // Breakdown (Feb 2026 - Deep Debt Evolution):
        //   - 3 Ed25519 (generate, sign, verify)
        //   - 4 ECDSA (P-256 + P-384 sign/verify)
        //   - 4 RSA (PKCS1 + PSS sign/verify)
        //   - 6 key exchange (X25519 x2, ECDH x4)
        //   - 6 AEAD (ChaCha20 x2, AES-GCM x4)
        //   - 11 hash (blake3, sha256, sha384, sha512, sha1, sha3_256, hash_for_cipher + hmac variants + hmac_blake3)
        //   - 6 password (argon2id x2, pbkdf2, bcrypt x2, scrypt)
        //   - 6 TLS 1.3 (derive_secrets, derive_handshake_secrets, derive_application_secrets, compute_finished_verify_data, sign_handshake, verify_certificate)
        //   - 9 TLS 1.2 semantic (ecdhe x4, aead x4, tls12_prf)
        //   - 10 genetic (derive_lineage_key, mix_entropy, verify_lineage, generate_lineage_proof, challenge x3, device x3)
        //   - 8 semantic aliases (hash, hmac, sign, verify, encrypt, decrypt, generate_keypair, derive_secret)
        //   - 10 beardog.crypto.* (Songbird Onion Service)
        //   - 2 Tor v3 Phase 1 (derive_onion_address, generate_onion_identity)
        //   - 6 Tor Phase 2 (ntor_client_init, ntor_client_finish, ntor_server_respond, cell_encrypt, cell_decrypt, tor_kdf)
        assert_eq!(methods.len(), 91);

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
        assert!(methods.contains(&"tls.derive_handshake_secrets"));
        assert!(methods.contains(&"tls.derive_application_secrets"));
        assert!(methods.contains(&"tls.sign_handshake"));
        assert!(methods.contains(&"tls.verify_certificate"));

        // Verify genetic methods (Phase 5 + Dark Forest)
        assert!(methods.contains(&"genetic.derive_lineage_key"));
        assert!(methods.contains(&"genetic.mix_entropy"));
        assert!(methods.contains(&"genetic.verify_lineage"));
        assert!(methods.contains(&"genetic.generate_lineage_proof"));
        assert!(methods.contains(&"genetic.generate_challenge")); // Dark Forest!
        assert!(methods.contains(&"genetic.respond_to_challenge")); // Dark Forest!
        assert!(methods.contains(&"genetic.verify_challenge_response")); // Dark Forest!
        
        // Verify device enrollment methods (Feb 5, 2026 - Deep Debt: DERIVE, not COPY)
        assert!(methods.contains(&"genetic.derive_device_seed"));
        assert!(methods.contains(&"genetic.sign_lineage_certificate"));
        assert!(methods.contains(&"genetic.verify_lineage_certificate"));

        // Verify semantic aliases (Phase 2 - Jan 27, 2026)
        assert!(methods.contains(&"crypto.hash"));
        assert!(methods.contains(&"crypto.hmac"));
        assert!(methods.contains(&"crypto.sign"));
        assert!(methods.contains(&"crypto.verify"));
        assert!(methods.contains(&"crypto.encrypt"));
        assert!(methods.contains(&"crypto.decrypt"));
        assert!(methods.contains(&"crypto.generate_keypair"));
        assert!(methods.contains(&"crypto.derive_secret"));

        // Verify beardog.crypto.* namespace (Feb 6, 2026 - Songbird Onion Service)
        assert!(methods.contains(&"beardog.crypto.sha3_256"));
        assert!(methods.contains(&"beardog.crypto.ed25519_generate_keypair"));
        assert!(methods.contains(&"beardog.crypto.sign_ed25519"));
        assert!(methods.contains(&"beardog.crypto.verify_ed25519"));
        assert!(methods.contains(&"beardog.crypto.x25519_generate_ephemeral"));
        assert!(methods.contains(&"beardog.crypto.x25519_derive_secret"));
        assert!(methods.contains(&"beardog.crypto.chacha20_poly1305_encrypt"));
        assert!(methods.contains(&"beardog.crypto.chacha20_poly1305_decrypt"));
        assert!(methods.contains(&"beardog.crypto.hmac_sha256"));
        assert!(methods.contains(&"beardog.crypto.blake3_hash"));

        // Verify Tor v3 methods (Feb 7, 2026 - Phase 1 Tor Integration)
        assert!(methods.contains(&"beardog.crypto.derive_onion_address"));
        assert!(methods.contains(&"beardog.crypto.generate_onion_identity"));

        // Verify Tor Phase 2 methods (Feb 2026 - Pure Rust Tor Protocol)
        assert!(methods.contains(&"beardog.crypto.tor_ntor_client_init"));
        assert!(methods.contains(&"beardog.crypto.tor_ntor_client_finish"));
        assert!(methods.contains(&"beardog.crypto.tor_ntor_server_respond"));
        assert!(methods.contains(&"beardog.crypto.tor_cell_encrypt"));
        assert!(methods.contains(&"beardog.crypto.tor_cell_decrypt"));
        assert!(methods.contains(&"beardog.crypto.tor_kdf"));
    }

    #[test]
    fn test_handler_method_count() {
        let handler = CryptoHandler;
        assert_eq!(
            handler.methods().len(),
            91,
            "Should have exactly 91 crypto methods (Phase 1-8 + TLS 1.2 + Dark Forest + Device Enrollment + Onion Service + Tor v3 + Tor Phase 2 - Feb 2026 Deep Debt Evolution)"
        );
    }
}
