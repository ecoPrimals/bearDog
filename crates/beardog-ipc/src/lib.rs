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

// tarpc high-performance RPC (Protocol Graduation: JSON-RPC → tarpc)
// Walk → Run pattern: JSON-RPC for flexibility, tarpc for speed
#[cfg(feature = "tarpc")]
pub mod tarpc_types;
#[cfg(feature = "tarpc")]
pub mod tarpc_server;
#[cfg(feature = "tarpc")]
pub mod tarpc_client;

// Protocol routing and multi-transport (always available for discovery)
pub mod protocol_router;
#[cfg(feature = "tarpc")]
pub mod multi_transport;

pub use client::SongbirdClient;
pub use error::{IpcError, IpcResult};
pub use types::{Capability, DiscoveryQuery, ServiceInfo};
// Neural API auto-registration (Tower Atomic TRUE PRIMAL)
pub use neural_registration::{discover_neural_api_socket, register_with_neural_api};

// Isomorphic IPC discovery (automatic Unix or TCP)
pub use isomorphic::{connect_beardog, discover_beardog_endpoint, AsyncStream, IpcEndpoint};

// Registry client for JSON-RPC registration
pub use protocol::JsonRpcRequest as ProtocolJsonRpcRequest;
pub use registry_client::{JsonRpcRequest, PrimalRegistryClient};

// tarpc types, server, client (when feature enabled)
#[cfg(feature = "tarpc")]
pub use tarpc_types::*;
#[cfg(feature = "tarpc")]
pub use tarpc_server::BearDogCryptoServer;
#[cfg(feature = "tarpc")]
pub use tarpc_client::TarpcCryptoClient;
#[cfg(feature = "tarpc")]
pub use multi_transport::{MultiTransportConfig, MultiTransportServer, ProtocolSelector};

// Protocol routing (always available)
pub use protocol_router::{Protocol, ProtocolCapabilities, ProtocolDetector, RouterConfig};

/// Primal IPC Protocol version
pub const PROTOCOL_VERSION: &str = "1.0";

/// Default discovery socket path (fallback only - prefer runtime discovery)
///
/// EVOLUTION NOTE: This constant exists as a fallback for compatibility.
/// Modern code should use `discover_ipc_socket()` for runtime discovery.
/// See: primal_discovery.rs for capability-based discovery pattern.
///
/// SELF-KNOWLEDGE PRINCIPLE (Feb 4, 2026): Primals should only know themselves.
/// The fallback uses a generic "/primal/discovery" endpoint that any discovery
/// service can bind to, rather than hardcoding a specific primal name.
pub const DISCOVERY_SOCKET_FALLBACK: &str = "/primal/discovery";

/// Discover IPC socket path via capability-based discovery
///
/// Priority order (zero hardcoding + self-knowledge principles):
/// 1. Environment variable: `IPC_SOCKET` or `DISCOVERY_SOCKET`
/// 2. Discovery via beardog-discovery (when available)
/// 3. Fallback: `/primal/discovery` (generic endpoint any discovery service can bind)
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

    if let Ok(socket) = std::env::var("DISCOVERY_SOCKET") {
        tracing::info!("📡 IPC socket from DISCOVERY_SOCKET env: {}", socket);
        return socket;
    }

    // 2. NOTE: beardog-discovery ready (45 tests pass), pending integration wiring
    // This will use capability-based discovery to find IPC services dynamically
    // Example: let ipc_services = beardog_discovery::discover_capability("ipc").await?;

    // 3. Fallback (generic discovery endpoint - any primal can bind here)
    // This follows the self-knowledge principle: we don't hardcode "songbird",
    // we use a generic endpoint any discovery service can claim.
    tracing::debug!("📡 IPC socket using fallback: {}", DISCOVERY_SOCKET_FALLBACK);
    DISCOVERY_SOCKET_FALLBACK.to_string()
}

/// Default heartbeat interval (30 seconds)
pub const DEFAULT_HEARTBEAT_INTERVAL: std::time::Duration = std::time::Duration::from_secs(30);
