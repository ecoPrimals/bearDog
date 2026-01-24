//! Unix Socket IPC Server for BearDog
//!
//! PRIMARY inter-primal communication interface (not HTTP!)
//!
//! ## Architecture Principle
//!
//! BearDog communicates with other primals via Unix sockets, not HTTP ports.
//! HTTP is OPTIONAL and only for external/debugging access.
//!
//! ## Module Structure (Semantic Boundaries)
//!
//! - `types` - JSON-RPC types, Protocol enum
//! - `protocol` - Protocol detection logic
//! - `handlers` - Request handlers for JSON-RPC (includes refactored handlers/crypto/)
//! - `crypto_handlers_*` - Domain-specific crypto handlers (ECDSA, RSA, AES-GCM, etc.)
//! - `server` - Server core and connection management

// REFACTORED: crypto_handlers.rs has been split into semantic domain modules
// See: handlers/crypto/ for the new modular structure
// pub mod crypto_handlers;
pub mod crypto_handlers_aes_gcm; // AES-GCM encryption (Phase 6 - 90%+ of HTTPS!)
// pub mod crypto_handlers_aes_legacy; // DEFERRED: AES-CBC/CTR/XTS (RC version conflicts)
pub mod crypto_handlers_ecdh; // ECDH P-256/P-384 key exchange (Phase 6 - TLS 1.3)
pub mod crypto_handlers_ecdsa; // ECDSA signature algorithms (P-256, P-384)
pub mod crypto_handlers_genetic; // Genetic crypto (Phase 5)
pub mod crypto_handlers_hashing; // SHA-256/384/512 standalone hashing (Phase 6)
pub mod crypto_handlers_hmac; // HMAC variants: SHA384/512, Blake3 (Phase 7)
pub mod crypto_handlers_kdf; // Additional KDFs: bcrypt, scrypt (Phase 7)
pub mod crypto_handlers_passwords; // Password hashing: Argon2id, PBKDF2 (Phase 6)
pub mod crypto_handlers_rsa; // RSA signature algorithms (PKCS#1 v1.5, PSS)
pub mod handlers;
pub mod protocol;
pub mod server;
pub mod types;

// Re-export main types for backward compatibility
pub use server::UnixSocketIpcServer;
pub use types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, Protocol};
