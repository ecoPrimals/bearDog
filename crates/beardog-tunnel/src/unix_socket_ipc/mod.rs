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
//! - `server` - Server core and connection management

pub mod handlers;
pub mod protocol;
pub mod server;
pub mod types;

// Re-export main types for backward compatibility
pub use server::UnixSocketIpcServer;
pub use types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, Protocol};
