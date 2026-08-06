// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

//! # beardog-ipc - Primal IPC Protocol Implementation
//!
//! This crate implements the ecoPrimals Primal IPC Protocol for `BearDog`,
//! enabling runtime discovery and communication via capability-based IPC registries.
//!
//! ## Standards Compliance
//!
//! Implements: `/wateringHole/PRIMAL_IPC_PROTOCOL.md`
//!
//! - ✅ JSON-RPC 2.0 over Unix sockets
//! - ✅ `/primal/*` namespace convention
//! - ✅ IPC registry integration (`ipc.register`, capability discovery)
//! - ✅ Capability-based discovery
//! - ✅ Heartbeat mechanism
//!
//! ## Quick Start
//!
//! ```no_run
//! use beardog_ipc::{OrchestratorRegistryClient, Capability};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Connect to IPC registry (socket from env / capability discovery)
//!     let client = OrchestratorRegistryClient::connect().await?;
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
pub mod dispatch;
pub mod error;
pub mod isomorphic; // Isomorphic IPC client discovery (Jan 31, 2026)
pub mod method_names;
pub mod neural_registration; // Neural API auto-registration (Tower Atomic)
pub mod protocol;
pub mod registry_client;
pub mod types;

pub mod protocol_router;

use beardog_config::env_keys;

pub use client::OrchestratorRegistryClient;
pub use dispatch::{DispatchOutcome, IpcErrorPhase};
pub use error::{IpcError, IpcResult};
pub use types::{Capability, DiscoveryQuery, ServiceInfo};
// Neural API auto-registration (Tower Atomic TRUE PRIMAL)
#[allow(
    deprecated,
    reason = "intentional use of deprecated API pending migration"
)]
pub use neural_registration::{
    beardog_announce_method_names, discover_neural_api_socket, discover_neural_api_socket_with,
    register_with_neural_api, send_primal_announce,
};

// Isomorphic IPC discovery (automatic Unix or TCP)
pub use isomorphic::{
    AsyncStream, IpcEndpoint, IpcStream, TcpDiscoveryPathHints, UnixSocketPathHints,
    connect_beardog, connect_raw, connect_transport, discover_beardog_endpoint,
    get_tcp_discovery_file_candidates_with, get_unix_socket_paths_with, ipc_endpoint_to_transport,
};
#[cfg(unix)]
pub use isomorphic::connect_unix;

// Registry client for JSON-RPC registration
pub use protocol::JsonRpcRequest as ProtocolJsonRpcRequest;
pub use registry_client::{JsonRpcRequest, PrimalRegistryClient};

pub use protocol_router::{Protocol, ProtocolCapabilities, ProtocolDetector, RouterConfig};

/// Primal IPC Protocol version
pub const PROTOCOL_VERSION: &str = "1.0";

/// Default discovery socket path (fallback only - prefer runtime discovery)
///
/// EVOLUTION NOTE: This constant exists as a fallback for compatibility.
/// Modern code should use `discover_ipc_socket()` for runtime discovery.
/// See: `primal_discovery.rs` for capability-based discovery pattern.
///
/// SELF-KNOWLEDGE PRINCIPLE (Feb 4, 2026): Primals should only know themselves.
/// The fallback uses a generic "/primal/discovery" endpoint that any discovery
/// service can bind to, rather than hardcoding a specific primal name.
pub const DISCOVERY_SOCKET_FALLBACK: &str = "/primal/discovery";

/// Platform-aware dev discovery socket path when `BEARDOG_DEV_DISCOVERY_SOCKET` is unset.
///
/// Equivalent to joining [`std::env::temp_dir`] with `"beardog-discovery"`.
/// Override the directory only with `BEARDOG_DEV_DISCOVERY_SOCKET_DIR` (optional).
#[must_use]
pub fn discovery_socket_dev_fallback_path() -> String {
    let name = "beardog-discovery";
    if let Ok(dir) = std::env::var(env_keys::ENV_DEV_DISCOVERY_SOCKET_DIR) {
        return std::path::PathBuf::from(dir)
            .join(name)
            .to_string_lossy()
            .into_owned();
    }
    std::env::temp_dir()
        .join(name)
        .to_string_lossy()
        .into_owned()
}

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
/// fn main() {
///     let socket = discover_ipc_socket();
///     println!("IPC socket: {}", socket);
/// }
/// ```
/// Inputs for [`discover_ipc_socket_with`] (injectable; [`Default`] is I/O-free).
#[derive(Debug, Clone, Default)]
pub struct IpcSocketDiscoveryOptions {
    /// `IPC_SOCKET` when set.
    pub ipc_socket: Option<String>,
    /// `DISCOVERY_SOCKET` when set.
    pub discovery_socket: Option<String>,
    /// `BEARDOG_DEV_DISCOVERY_SOCKET` when set.
    pub beardog_dev_discovery_socket: Option<String>,
    /// Pre-resolved `ipc` capability services (e.g. from [`beardog_discovery::discovered_services_from_environment_with`]).
    pub ipc_capability_services: Vec<beardog_discovery::DiscoveredService>,
}

impl IpcSocketDiscoveryOptions {
    /// Load from [`beardog_errors::process_env`] and capability env discovery.
    #[must_use]
    pub fn from_env() -> Self {
        let ttl = beardog_discovery::DEFAULT_ENV_DISCOVERY_TTL_SECS;
        Self {
            ipc_socket: beardog_errors::process_env::var(env_keys::ENV_IPC_SOCKET).ok(),
            discovery_socket: beardog_errors::process_env::var(env_keys::ENV_DISCOVERY_SOCKET).ok(),
            beardog_dev_discovery_socket: beardog_errors::process_env::var(
                env_keys::ENV_DEV_DISCOVERY_SOCKET,
            )
            .ok(),
            ipc_capability_services:
                beardog_discovery::discovered_services_from_environment_from_env("ipc", ttl),
        }
    }
}

/// Resolve IPC socket path from injected options (tests pass explicit values; synchronous).
#[must_use]
pub fn resolve_ipc_socket_from_options(opts: &IpcSocketDiscoveryOptions) -> String {
    if let Some(socket) = &opts.ipc_socket {
        tracing::info!(source = "IPC_SOCKET", socket = %socket, "IPC socket resolved");
        return socket.clone();
    }

    if let Some(socket) = &opts.discovery_socket {
        tracing::info!(source = "DISCOVERY_SOCKET", socket = %socket, "IPC socket resolved");
        return socket.clone();
    }

    if let Some(svc) = opts.ipc_capability_services.first() {
        let path = beardog_discovery::primary_url_to_ipc_socket_path(&svc.endpoint.primary_url);
        if !path.is_empty() {
            tracing::info!(source = "capability_ipc", socket = %path, "IPC socket resolved");
            return path;
        }
    }

    if let Some(dev) = &opts.beardog_dev_discovery_socket {
        tracing::info!(
            source = "BEARDOG_DEV_DISCOVERY_SOCKET",
            socket = %dev,
            "IPC socket resolved"
        );
        return dev.clone();
    }

    tracing::debug!(
        source = "fallback",
        socket = DISCOVERY_SOCKET_FALLBACK,
        "IPC socket resolved"
    );
    DISCOVERY_SOCKET_FALLBACK.to_string()
}

/// Resolve IPC socket path from injected options (async wrapper; identical to [`resolve_ipc_socket_from_options`]).
#[must_use]
pub fn discover_ipc_socket_with(opts: IpcSocketDiscoveryOptions) -> String {
    resolve_ipc_socket_from_options(&opts)
}

/// Discover IPC socket path via [`IpcSocketDiscoveryOptions::from_env`].
#[must_use]
pub fn discover_ipc_socket() -> String {
    discover_ipc_socket_with(IpcSocketDiscoveryOptions::from_env())
}

/// JSON-RPC `ipc.resolve` params key for the target service instance id (legacy wire name).
/// The value is an opaque id from capability discovery, not a fixed peer product name.
pub const IPC_RESOLVE_TARGET_PARAM_KEY: &str = "primal";

/// Returns the JSON-RPC parameter name used for `ipc.resolve` targets from an explicit override.
#[must_use]
pub fn ipc_resolve_target_param_key_with(override_name: Option<String>) -> String {
    override_name.unwrap_or_else(|| IPC_RESOLVE_TARGET_PARAM_KEY.to_string())
}

/// Reads [`env_keys::ENV_IPC_RESOLVE_TARGET_PARAM_KEY`] from the environment, or defaults to
/// [`IPC_RESOLVE_TARGET_PARAM_KEY`] so registries can rename the field without recompiling clients.
#[must_use]
pub fn ipc_resolve_target_param_key() -> String {
    ipc_resolve_target_param_key_with(
        beardog_errors::process_env::var(env_keys::ENV_IPC_RESOLVE_TARGET_PARAM_KEY).ok(),
    )
}

/// Default heartbeat interval (aligned with [`beardog_types::constants::domains::system::intervals::HEARTBEAT_INTERVAL`]).
pub const DEFAULT_HEARTBEAT_INTERVAL: std::time::Duration =
    beardog_types::constants::domains::system::intervals::HEARTBEAT_INTERVAL;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn discover_ipc_socket_prefers_ipc_socket_env() {
        let opts = IpcSocketDiscoveryOptions {
            ipc_socket: Some("/tmp/from-ipc-socket".to_string()),
            ..Default::default()
        };
        assert_eq!(
            resolve_ipc_socket_from_options(&opts),
            "/tmp/from-ipc-socket"
        );
    }

    #[tokio::test]
    async fn discover_ipc_socket_uses_discovery_socket_when_ipc_socket_unset() {
        let opts = IpcSocketDiscoveryOptions {
            ipc_socket: None,
            discovery_socket: Some("/tmp/from-discovery-socket".to_string()),
            ..Default::default()
        };
        assert_eq!(
            resolve_ipc_socket_from_options(&opts),
            "/tmp/from-discovery-socket"
        );
    }

    #[tokio::test]
    async fn discover_ipc_socket_fallback_when_no_env() {
        assert_eq!(
            resolve_ipc_socket_from_options(&IpcSocketDiscoveryOptions::default()),
            DISCOVERY_SOCKET_FALLBACK
        );
    }

    #[tokio::test]
    async fn discover_ipc_socket_uses_capability_ipc_endpoint() {
        use beardog_discovery::types::{
            Capability, DiscoveredService, HealthStatus, QoSMetrics, ServiceEndpoint,
        };
        use std::collections::HashMap;
        use std::time::SystemTime;

        let svc = DiscoveredService {
            id: "env-ipc".to_string(),
            service_type: "t".to_string(),
            display_name: "d".to_string(),
            endpoint: ServiceEndpoint {
                primary_url: "unix:///tmp/from-capability-ipc".to_string(),
                fallback_urls: vec![],
                use_tls: false,
                path_prefix: None,
            },
            capabilities: vec![Capability {
                capability_type: "ipc".to_string(),
                version: "1".to_string(),
                features: vec![],
                parameters: HashMap::new(),
            }],
            qos: QoSMetrics::default(),
            health: HealthStatus::Unknown,
            discovered_at: SystemTime::now(),
            ttl_secs: 60,
            discovery_method: "test".to_string(),
            metadata: HashMap::new(),
        };
        let opts = IpcSocketDiscoveryOptions {
            ipc_capability_services: vec![svc],
            ..Default::default()
        };
        assert_eq!(
            resolve_ipc_socket_from_options(&opts),
            "/tmp/from-capability-ipc"
        );
    }

    #[test]
    fn ipc_resolve_target_param_key_respects_env() {
        assert_eq!(
            ipc_resolve_target_param_key_with(Some("custom_target".to_string())),
            "custom_target"
        );
    }

    #[test]
    fn ipc_resolve_target_param_key_default() {
        assert_eq!(
            ipc_resolve_target_param_key_with(None),
            IPC_RESOLVE_TARGET_PARAM_KEY
        );
    }
}

#[cfg(test)]
mod coverage_tests;
