// SPDX-License-Identifier: AGPL-3.0-only

//! TLS 1.2 cryptographic operations
//!
//! This module provides TLS 1.2-specific cryptographic operations for calling
//! primals that need backward compatibility with older systems.
//!
//! # Overview
//!
//! TLS 1.2 uses different algorithms than TLS 1.3:
//! - **Key Exchange**: ECDHE with NIST P-256/P-384 curves (vs X25519)
//! - **Bulk Encryption**: AES-GCM AEAD (vs ChaCha20-Poly1305)
//! - **Key Derivation**: TLS 1.2 PRF (vs HKDF)
//! - **Signatures**: ECDSA with NIST curves (vs Ed25519)
//!
//! # Architecture: Tower Atomic Pattern
//!
//! `BearDog` provides the cryptographic "atoms" via JSON-RPC:
//! - The calling primal orchestrates the TLS 1.2 handshake
//! - `BearDog` executes the crypto operations
//! - Zero crypto code duplication across primals
//! - Pure Rust for both (ecoBin compliant)
//!
//! # Methods (Semantic Naming)
//!
//! ## ECDHE Key Exchange
//! - `crypto.ecdhe.p256.generate` - Generate P-256 ephemeral keypair
//! - `crypto.ecdhe.p256.compute_shared` - Compute ECDH shared secret (P-256)
//! - `crypto.ecdhe.p384.generate` - Generate P-384 ephemeral keypair
//! - `crypto.ecdhe.p384.compute_shared` - Compute ECDH shared secret (P-384)
//!
//! ## AES-GCM AEAD
//! - `crypto.aead.aes_128_gcm.encrypt` - AES-128-GCM encryption
//! - `crypto.aead.aes_128_gcm.decrypt` - AES-128-GCM decryption
//! - `crypto.aead.aes_256_gcm.encrypt` - AES-256-GCM encryption
//! - `crypto.aead.aes_256_gcm.decrypt` - AES-256-GCM decryption
//!
//! ## TLS 1.2 PRF
//! - `crypto.kdf.tls12_prf` - TLS 1.2 Pseudorandom Function (key expansion)
//!
//! # Security
//!
//! - **NIST Curves**: P-256 (128-bit security), P-384 (192-bit security)
//! - **AES-GCM**: AEAD with 128-bit or 256-bit keys, 12-byte nonces, 16-byte tags
//! - **TLS 1.2 PRF**: HMAC-SHA256 or HMAC-SHA384 based key expansion
//! - **Pure Rust**: 100% `RustCrypto`, zero C dependencies
//!
//! # Usage
//!
//! ```rust,ignore
//! // Generate P-256 ephemeral keypair for ECDHE
//! let keypair = handle_ecdhe_p256_generate(None).await?;
//!
//! // Compute shared secret
//! let shared = handle_ecdhe_p256_compute_shared(params).await?;
//!
//! // Derive TLS 1.2 keys
//! let keys = handle_tls12_prf(params).await?;
//!
//! // Encrypt with AES-128-GCM
//! let ciphertext = handle_aes_128_gcm_encrypt(params).await?;
//! ```
//!
//! # References
//!
//! - [RFC 5246](https://www.rfc-editor.org/rfc/rfc5246.html) - TLS 1.2 Protocol
//! - [RFC 5288](https://www.rfc-editor.org/rfc/rfc5288.html) - AES-GCM for TLS 1.2
//! - [RFC 4492](https://www.rfc-editor.org/rfc/rfc4492.html) - ECC for TLS
//! - [RFC 5869](https://www.rfc-editor.org/rfc/rfc5869.html) - HKDF (TLS 1.3)
//!
//! # Created
//!
//! January 27, 2026 - Deep Debt Evolution Session\
//! Added for TLS 1.2 backward compatibility (interoperability with legacy stacks)

mod aead;
mod ecdhe;
mod prf;

pub use aead::{
    handle_aes_128_gcm_decrypt, handle_aes_128_gcm_encrypt, handle_aes_256_gcm_decrypt,
    handle_aes_256_gcm_encrypt,
};
pub use ecdhe::{
    handle_ecdhe_p256_compute_shared, handle_ecdhe_p256_generate, handle_ecdhe_p384_compute_shared,
    handle_ecdhe_p384_generate,
};
pub use prf::handle_tls12_prf;

#[cfg(test)]
#[path = "tls12_tests.rs"]
mod tests;
