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

pub mod tls;
pub mod asymmetric;
pub mod symmetric;
pub mod hash;
pub mod sslkeylog;

// Re-export all handlers for backward compatibility
// This ensures existing code using `use crate::unix_socket_ipc::crypto_handlers::*` 
// continues to work without changes
pub use tls::*;
pub use asymmetric::*;
pub use symmetric::*;
pub use hash::*;

