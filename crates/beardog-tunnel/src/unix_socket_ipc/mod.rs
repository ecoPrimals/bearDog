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
//! - `handlers` - Request handlers for JSON-RPC and HTTP
//! - `crypto_handlers` - Crypto operation handlers (for Songbird TLS)
//! - `server` - Server core and connection management

pub mod crypto_handlers;
pub mod crypto_handlers_ecdsa;
pub mod crypto_handlers_genetic;
pub mod crypto_handlers_rsa; // ECDSA signature algorithms (P-256, P-384, P-521)
pub mod handlers;
pub mod handlers_legacy; // Keep old monolith as reference during migration
pub mod protocol;
pub mod server;
pub mod types;

// Re-export main types for backward compatibility
pub use server::UnixSocketIpcServer;
pub use types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, Protocol};
