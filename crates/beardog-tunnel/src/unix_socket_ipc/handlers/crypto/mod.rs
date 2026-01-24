//! Crypto operation handlers organized by domain
//!
//! This module provides cryptographic operations via JSON-RPC, organized
//! into semantic domains for better maintainability and clarity.
//!
//! # Organization
//!
//! - [`tls`] - TLS 1.3 crypto operations (handshake keys, application keys, signatures, certificates)
//! - [`asymmetric`] - Asymmetric cryptography (Ed25519 signatures, X25519 key exchange)
//! - [`symmetric`] - Symmetric cryptography (ChaCha20-Poly1305, AES-GCM AEAD)
//! - [`hash`] - Hashing operations (Blake3, HMAC-SHA256)
//! - [`sslkeylog`] - SSLKEYLOGFILE export utility for Wireshark debugging
//!
//! # Architecture
//!
//! - **Complete Implementation**: No mocks, all operations are production-ready
//! - **Pure Rust**: 100% RustCrypto ecosystem, zero C dependencies
//! - **Capability-Based**: Methods exposed as discoverable capabilities
//! - **Self-Knowledge Only**: No hardcoded primal names or external service assumptions
//!
//! # Refactoring History
//!
//! **v0.21.0 (January 24, 2026)**: Smart refactoring by semantic domain
//! - Previously: Single 2,499-line `crypto_handlers.rs` file
//! - Now: 7 focused domain files averaging 400-600 lines each
//! - Benefit: Better organization, easier maintenance, reduced cognitive load
//!
//! # Usage
//!
//! All handlers are re-exported at the module level for backward compatibility:
//!
//! ```rust,no_run
//! use crate::unix_socket_ipc::handlers::crypto::*;
//!
//! // TLS operations
//! let result = handle_tls_derive_application_secrets(params).await?;
//!
//! // Asymmetric crypto
//! let keypair = handle_x25519_generate_ephemeral(None).await?;
//!
//! // Symmetric crypto
//! let ciphertext = handle_chacha20_poly1305_encrypt(params).await?;
//!
//! // Hash operations
//! let hash = handle_blake3_hash(params).await?;
//! ```

// Module declarations (modules extracted from crypto_handlers.rs)
pub mod asymmetric; // ✅ Step 4 - Ed25519, X25519 (325 lines)
pub mod hash; // ✅ Step 6 - Blake3, HMAC-SHA256 (186 lines)
pub mod sslkeylog; // ✅ Step 2 - SSLKEYLOGFILE export utility
pub mod symmetric; // ✅ Step 5 - ChaCha20-Poly1305 (256 lines)
pub mod tls; // ✅ Step 3 - TLS 1.3 operations (1,884 lines)
pub mod utils; // ✅ Step 7 - Shared utility functions (derive_key_from_id)

// Re-export sslkeylog utility for backward compatibility
pub use sslkeylog::export_to_sslkeylogfile;

// Re-export TLS handlers for backward compatibility
pub use tls::{
    handle_tls_compute_finished_verify_data, handle_tls_derive_application_secrets,
    handle_tls_derive_handshake_secrets, handle_tls_derive_secrets, handle_tls_sign_handshake,
    handle_tls_verify_certificate,
};

// Re-export asymmetric crypto handlers for backward compatibility
pub use asymmetric::{
    handle_sign_ed25519, handle_verify_ed25519, handle_x25519_derive_secret,
    handle_x25519_generate_ephemeral,
};

// Re-export symmetric crypto handlers for backward compatibility
pub use symmetric::{handle_chacha20_poly1305_decrypt, handle_chacha20_poly1305_encrypt};

// Re-export hash handlers for backward compatibility
pub use hash::{handle_blake3_hash, handle_hmac_sha256};
