//! # beardog-ipc - Primal IPC Protocol Implementation
//!
//! This crate implements the ecoPrimals Primal IPC Protocol for BearDog,
//! enabling runtime discovery and communication with other primals via Songbird.
//!
//! ## Standards Compliance
//!
//! Implements: `/wateringHole/PRIMAL_IPC_PROTOCOL.md`
//!
//! - ✅ JSON-RPC 2.0 over Unix sockets
//! - ✅ `/primal/*` namespace convention
//! - ✅ Songbird service registry integration
//! - ✅ Capability-based discovery
//! - ✅ Heartbeat mechanism
//!
//! ## Quick Start
//!
//! ```no_run
//! use beardog_ipc::{SongbirdClient, Capability};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Connect to Songbird
//!     let client = SongbirdClient::connect().await?;
//!     
//!     // Register BearDog capabilities
//!     client.register(
//!         "beardog",
//!         vec![Capability::Crypto, Capability::BTSP],
//!     ).await?;
//!     
//!     // Discover crypto service
//!     let services = client.find_capability("crypto").await?;
//!     if let Some(crypto) = services.first() {
//!         println!("Found crypto service at: {}", crypto.endpoint);
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod isomorphic; // Isomorphic IPC client discovery (Jan 31, 2026)
pub mod neural_registration; // Neural API auto-registration (Tower Atomic)
pub mod protocol;
pub mod registry_client;
pub mod types;

pub use client::SongbirdClient;
pub use error::{IpcError, IpcResult};
pub use types::{Capability, DiscoveryQuery, ServiceInfo};
// Neural API auto-registration (Tower Atomic TRUE PRIMAL)
pub use neural_registration::{discover_neural_api_socket, register_with_neural_api};

// Isomorphic IPC discovery (automatic Unix or TCP)
pub use isomorphic::{
    discover_beardog_endpoint, connect_beardog, IpcEndpoint, AsyncStream,
};

// Registry client for JSON-RPC registration
pub use protocol::JsonRpcRequest as ProtocolJsonRpcRequest;
pub use registry_client::{JsonRpcRequest, PrimalRegistryClient};

/// Primal IPC Protocol version
pub const PROTOCOL_VERSION: &str = "1.0";

/// Default Songbird socket path (fallback only - prefer runtime discovery)
///
/// EVOLUTION NOTE: This constant exists as a fallback for compatibility.
/// Modern code should use `discover_ipc_socket()` for runtime discovery.
/// See: primal_discovery.rs for capability-based discovery pattern.
pub const SONGBIRD_SOCKET: &str = "/primal/songbird";

/// Discover IPC socket path via capability-based discovery
///
/// Priority order (zero hardcoding principle):
/// 1. Environment variable: `IPC_SOCKET` or `SONGBIRD_SOCKET`
/// 2. Discovery via beardog-discovery (when available)
/// 3. Fallback: `/primal/songbird` (compatibility)
///
/// # Example
/// ```no_run
/// use beardog_ipc::discover_ipc_socket;
///
/// #[tokio::main]
/// async fn main() {
///     let socket = discover_ipc_socket().await;
///     println!("IPC socket: {}", socket);
/// }
/// ```
pub async fn discover_ipc_socket() -> String {
    // 1. Check environment (highest priority - operator control)
    if let Ok(socket) = std::env::var("IPC_SOCKET") {
        tracing::info!("📡 IPC socket from IPC_SOCKET env: {}", socket);
        return socket;
    }

    if let Ok(socket) = std::env::var("SONGBIRD_SOCKET") {
        tracing::info!("📡 IPC socket from SONGBIRD_SOCKET env: {}", socket);
        return socket;
    }

    // 2. TODO: Discovery via beardog-discovery crate (when available)
    // This will use capability-based discovery to find IPC services dynamically
    // Example: let ipc_services = beardog_discovery::discover_capability("ipc").await?;

    // 3. Fallback (compatibility - will be removed when discovery is complete)
    tracing::debug!("📡 IPC socket using fallback: {}", SONGBIRD_SOCKET);
    SONGBIRD_SOCKET.to_string()
}

/// Default heartbeat interval (30 seconds)
pub const DEFAULT_HEARTBEAT_INTERVAL: std::time::Duration = std::time::Duration::from_secs(30);
