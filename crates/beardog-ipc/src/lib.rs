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
pub mod neural_registration; // Neural API auto-registration (Tower Atomic)
pub mod protocol;
pub mod registry_client;
pub mod types;

pub use client::SongbirdClient;
pub use error::{IpcError, IpcResult};
pub use types::{Capability, DiscoveryQuery, ServiceInfo};
// Neural API auto-registration (Tower Atomic TRUE PRIMAL)
pub use neural_registration::{discover_neural_api_socket, register_with_neural_api};

// Registry client for JSON-RPC registration
pub use protocol::JsonRpcRequest as ProtocolJsonRpcRequest;
pub use registry_client::{JsonRpcRequest, PrimalRegistryClient};

/// Primal IPC Protocol version
pub const PROTOCOL_VERSION: &str = "1.0";

/// Default Songbird socket path
pub const SONGBIRD_SOCKET: &str = "/primal/songbird";

/// Default heartbeat interval (30 seconds)
pub const DEFAULT_HEARTBEAT_INTERVAL: std::time::Duration = std::time::Duration::from_secs(30);
