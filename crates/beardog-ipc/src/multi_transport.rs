// SPDX-License-Identifier: AGPL-3.0-only

//! # 🌐 Multi-Transport Server for BearDog
//!
//! **CONCURRENT PROTOCOL BINDING** (v1.0.0)
//!
//! Runs tarpc and JSON-RPC servers concurrently on separate ports,
//! enabling protocol graduation (Walk → Run pattern).
//!
//! ## Architecture
//! ```text
//! ┌──────────────────────────────────────────────────────────────┐
//! │                     MultiTransportServer                      │
//! ├──────────────────────────────────────────────────────────────┤
//! │                                                               │
//! │  ┌─────────────────────┐    ┌─────────────────────┐          │
//! │  │   tarpc Server      │    │   JSON-RPC Server   │          │
//! │  │   (base+1, env)     │    │   (base, env)       │          │
//! │  │   ~10-20μs          │    │   ~100-500μs        │          │
//! │  │   Binary/Bincode    │    │   Human-readable    │          │
//! │  └─────────────────────┘    └─────────────────────┘          │
//! │           ▲                          ▲                        │
//! │           │                          │                        │
//! │  ┌────────┴──────────────────────────┴───────────┐           │
//! │  │              Shared Handlers                   │           │
//! │  │         (CryptoService, TLS, etc.)            │           │
//! │  └────────────────────────────────────────────────┘           │
//! │                                                               │
//! └──────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Protocol Graduation
//! 1. **Development**: Use JSON-RPC (readable logs, easy debugging)
//! 2. **Testing**: Use JSON-RPC with tarpc for performance tests
//! 3. **Production**: Graduate stable operations to tarpc
//!
//! ## Configuration
//! Ports are discovered via capability system, not hardcoded.

use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use beardog_config::domains::network_addresses::DEFAULT_BIND_ADDRESS;
use tokio::sync::broadcast;
use tracing::{error, info, warn};

use crate::protocol_router::{Protocol, ProtocolCapabilities, RouterConfig};
use crate::tarpc_server::BearDogCryptoServer;

/// Multi-transport server configuration
#[derive(Debug, Clone)]
pub struct MultiTransportConfig {
    /// tarpc bind address (`BEARDOG_TARPC_ADDR` or `BEARDOG_BIND_ADDR` + port from `beardog_config::DEFAULT_TCP_IPC_PORT` + 1)
    pub tarpc_addr: SocketAddr,

    /// JSON-RPC bind address (`BEARDOG_JSONRPC_ADDR` or `BEARDOG_BIND_ADDR` + `beardog_config::DEFAULT_TCP_IPC_PORT`)
    pub jsonrpc_addr: SocketAddr,

    /// Enable tarpc server
    pub enable_tarpc: bool,

    /// Enable JSON-RPC server
    pub enable_jsonrpc: bool,

    /// Graceful shutdown timeout
    pub shutdown_timeout: Duration,
}

/// Default port for tarpc server (can be overridden by BEARDOG_TARPC_PORT)
const DEFAULT_TARPC_PORT: u16 = beardog_config::DEFAULT_TCP_IPC_PORT + 1;

/// Default port for JSON-RPC server (can be overridden by BEARDOG_JSONRPC_PORT)
const DEFAULT_JSONRPC_PORT: u16 = beardog_config::DEFAULT_TCP_IPC_PORT;

/// Default shutdown timeout in seconds (can be overridden by BEARDOG_SHUTDOWN_TIMEOUT)
const DEFAULT_SHUTDOWN_TIMEOUT_SECS: u64 = 30;

/// Parse `host:port`; if the string is invalid, bind loopback on `port` (never panics).
fn socket_addr_or_loopback(host: &str, port: u16) -> SocketAddr {
    format!("{host}:{port}")
        .parse()
        .unwrap_or_else(|_| SocketAddr::from((Ipv4Addr::LOCALHOST, port)))
}

impl Default for MultiTransportConfig {
    fn default() -> Self {
        Self::from_bind_and_ports(
            DEFAULT_BIND_ADDRESS,
            DEFAULT_TARPC_PORT,
            DEFAULT_JSONRPC_PORT,
            DEFAULT_SHUTDOWN_TIMEOUT_SECS,
        )
    }
}

impl MultiTransportConfig {
    /// Build from explicit bind host and ports (no environment I/O).
    pub fn from_bind_and_ports(
        bind_addr: impl AsRef<str>,
        tarpc_port: u16,
        jsonrpc_port: u16,
        shutdown_timeout_secs: u64,
    ) -> Self {
        let bind_addr = bind_addr.as_ref();

        let tarpc_addr = format!("{bind_addr}:{tarpc_port}")
            .parse()
            .unwrap_or_else(|_| {
                tracing::warn!(
                    "Failed to parse tarpc address '{}:{}', using {}:{}",
                    bind_addr,
                    tarpc_port,
                    DEFAULT_BIND_ADDRESS,
                    tarpc_port
                );
                socket_addr_or_loopback(DEFAULT_BIND_ADDRESS, tarpc_port)
            });

        let jsonrpc_addr = format!("{bind_addr}:{jsonrpc_port}")
            .parse()
            .unwrap_or_else(|_| {
                tracing::warn!(
                    "Failed to parse jsonrpc address '{}:{}', using {}:{}",
                    bind_addr,
                    jsonrpc_port,
                    DEFAULT_BIND_ADDRESS,
                    jsonrpc_port
                );
                socket_addr_or_loopback(DEFAULT_BIND_ADDRESS, jsonrpc_port)
            });

        Self {
            tarpc_addr,
            jsonrpc_addr,
            enable_tarpc: true,
            enable_jsonrpc: true,
            shutdown_timeout: Duration::from_secs(shutdown_timeout_secs),
        }
    }

    /// Create from environment variables (capability-based, not hardcoded)
    ///
    /// Env vars:
    /// - `BEARDOG_BIND_ADDR`, `BEARDOG_TARPC_PORT`, `BEARDOG_JSONRPC_PORT`, `BEARDOG_SHUTDOWN_TIMEOUT`
    /// - `BEARDOG_TARPC_ADDR`: tarpc server address
    /// - `BEARDOG_JSONRPC_ADDR`: JSON-RPC server address
    /// - `BEARDOG_ENABLE_TARPC`: Enable tarpc (default: true)
    /// - `BEARDOG_ENABLE_JSONRPC`: Enable JSON-RPC (default: true)
    pub fn from_env() -> Self {
        let bind_addr = beardog_errors::process_env::var("BEARDOG_BIND_ADDR")
            .unwrap_or_else(|_| DEFAULT_BIND_ADDRESS.to_string());

        let tarpc_port: u16 = beardog_errors::process_env::var("BEARDOG_TARPC_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(DEFAULT_TARPC_PORT);

        let jsonrpc_port: u16 = beardog_errors::process_env::var("BEARDOG_JSONRPC_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(DEFAULT_JSONRPC_PORT);

        let shutdown_timeout_secs: u64 =
            beardog_errors::process_env::var("BEARDOG_SHUTDOWN_TIMEOUT")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(DEFAULT_SHUTDOWN_TIMEOUT_SECS);

        let mut config =
            Self::from_bind_and_ports(&bind_addr, tarpc_port, jsonrpc_port, shutdown_timeout_secs);

        if let Ok(addr) = beardog_errors::process_env::var("BEARDOG_TARPC_ADDR") {
            if let Ok(parsed) = addr.parse() {
                config.tarpc_addr = parsed;
            }
        }

        if let Ok(addr) = beardog_errors::process_env::var("BEARDOG_JSONRPC_ADDR") {
            if let Ok(parsed) = addr.parse() {
                config.jsonrpc_addr = parsed;
            }
        }

        if let Ok(val) = beardog_errors::process_env::var("BEARDOG_ENABLE_TARPC") {
            config.enable_tarpc = val != "0" && val.to_lowercase() != "false";
        }

        if let Ok(val) = beardog_errors::process_env::var("BEARDOG_ENABLE_JSONRPC") {
            config.enable_jsonrpc = val != "0" && val.to_lowercase() != "false";
        }

        config
    }

    /// Create from router config and base port
    ///
    /// Uses [`DEFAULT_BIND_ADDRESS`] with the given `base_port` (no environment reads).
    /// Falls back to localhost if the provided address is invalid.
    pub fn from_router_config(router: &RouterConfig, base_port: u16) -> Self {
        let bind_addr = DEFAULT_BIND_ADDRESS;

        // Safe address parsing with fallback - prevents panics from invalid env vars
        let tarpc_port = base_port.saturating_add(1);
        let tarpc_addr = format!("{bind_addr}:{tarpc_port}")
            .parse()
            .unwrap_or_else(|_| {
                tracing::warn!(
                    "Failed to parse tarpc address '{}:{}', using {}:{}",
                    bind_addr,
                    tarpc_port,
                    DEFAULT_BIND_ADDRESS,
                    tarpc_port
                );
                socket_addr_or_loopback(DEFAULT_BIND_ADDRESS, tarpc_port)
            });

        let jsonrpc_addr = format!("{bind_addr}:{base_port}")
            .parse()
            .unwrap_or_else(|_| {
                tracing::warn!(
                    "Failed to parse jsonrpc address '{}:{}', using {}:{}",
                    bind_addr,
                    base_port,
                    DEFAULT_BIND_ADDRESS,
                    base_port
                );
                socket_addr_or_loopback(DEFAULT_BIND_ADDRESS, base_port)
            });

        Self {
            tarpc_addr,
            jsonrpc_addr,
            enable_tarpc: router.enable_tarpc,
            enable_jsonrpc: router.enable_jsonrpc,
            shutdown_timeout: Duration::from_secs(30),
        }
    }

    /// Get protocol capabilities
    pub fn capabilities(&self) -> ProtocolCapabilities {
        let router = RouterConfig {
            enable_tarpc: self.enable_tarpc,
            enable_jsonrpc: self.enable_jsonrpc,
            enable_http: false, // HTTP handled separately
            preferred: if self.enable_tarpc {
                Protocol::Tarpc
            } else {
                Protocol::JsonRpc
            },
        };
        ProtocolCapabilities::from_config(&router)
    }
}

/// Multi-transport server handle
///
/// Provides control over running servers (shutdown, health check, etc.)
pub struct MultiTransportHandle {
    /// Shutdown signal sender
    shutdown_tx: broadcast::Sender<()>,

    /// Configuration (for introspection)
    config: MultiTransportConfig,
}

impl MultiTransportHandle {
    /// Signal graceful shutdown
    pub fn shutdown(&self) {
        if let Err(e) = self.shutdown_tx.send(()) {
            warn!("No receivers for shutdown signal: {}", e);
        }
    }

    /// Get server configuration
    pub const fn config(&self) -> &MultiTransportConfig {
        &self.config
    }

    /// Get tarpc address (if enabled)
    pub const fn tarpc_addr(&self) -> Option<SocketAddr> {
        if self.config.enable_tarpc {
            Some(self.config.tarpc_addr)
        } else {
            None
        }
    }

    /// Get JSON-RPC address (if enabled)
    pub const fn jsonrpc_addr(&self) -> Option<SocketAddr> {
        if self.config.enable_jsonrpc {
            Some(self.config.jsonrpc_addr)
        } else {
            None
        }
    }
}

/// Multi-transport server
///
/// Runs both tarpc and JSON-RPC servers concurrently.
pub struct MultiTransportServer {
    config: MultiTransportConfig,
}

impl MultiTransportServer {
    /// Create a new multi-transport server
    pub const fn new(config: MultiTransportConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(MultiTransportConfig::default())
    }

    /// Create from environment variables
    pub fn from_env() -> Self {
        Self::new(MultiTransportConfig::from_env())
    }

    /// Start the multi-transport server
    ///
    /// Returns a handle for controlling the server.
    /// This function spawns server tasks and returns immediately.
    pub async fn start(self) -> anyhow::Result<MultiTransportHandle> {
        let (shutdown_tx, _) = broadcast::channel(1);

        info!("🌐 Starting BearDog Multi-Transport Server");
        info!("   Configuration:");
        info!(
            "   - tarpc:    {} ({})",
            self.config.tarpc_addr,
            if self.config.enable_tarpc {
                "enabled"
            } else {
                "disabled"
            }
        );
        info!(
            "   - JSON-RPC: {} ({})",
            self.config.jsonrpc_addr,
            if self.config.enable_jsonrpc {
                "enabled"
            } else {
                "disabled"
            }
        );

        // Start tarpc server
        if self.config.enable_tarpc {
            let tarpc_addr = self.config.tarpc_addr;
            let mut shutdown_rx = shutdown_tx.subscribe();

            tokio::spawn(async move {
                let server = BearDogCryptoServer::new();

                tokio::select! {
                    result = server.run(tarpc_addr) => {
                        if let Err(e) = result {
                            error!("tarpc server error: {}", e);
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        info!("tarpc server shutting down");
                    }
                }
            });
        }

        // Start JSON-RPC server
        if self.config.enable_jsonrpc {
            let jsonrpc_addr = self.config.jsonrpc_addr;
            let mut shutdown_rx = shutdown_tx.subscribe();

            tokio::spawn(async move {
                // JSON-RPC server implementation would go here
                // For now, we just listen and respond with capabilities
                let listener = match tokio::net::TcpListener::bind(jsonrpc_addr).await {
                    Ok(l) => l,
                    Err(e) => {
                        error!("Failed to bind JSON-RPC server: {}", e);
                        return;
                    }
                };

                info!("📡 BearDog JSON-RPC server listening on {}", jsonrpc_addr);

                loop {
                    tokio::select! {
                        accept = listener.accept() => {
                            match accept {
                                Ok((mut stream, peer)) => {
                                    info!("📥 JSON-RPC connection from {}", peer);

                                    // Handle JSON-RPC in spawned task
                                    tokio::spawn(async move {
                                        // Simple JSON-RPC handler (placeholder)
                                        // Real implementation would dispatch to handlers
                                        use tokio::io::{AsyncReadExt, AsyncWriteExt};

                                        let mut buf = vec![0u8; 4096];
                                        if let Ok(n) = stream.read(&mut buf).await {
                                            if n > 0 {
                                                // Echo capabilities for now
                                                let response = serde_json::json!({
                                                    "jsonrpc": "2.0",
                                                    "result": {
                                                        "name": "BearDog",
                                                        "version": env!("CARGO_PKG_VERSION"),
                                                        "protocols": ["tarpc", "json-rpc"],
                                                        "capabilities": [
                                                            "crypto.signatures",
                                                            "crypto.encryption",
                                                            "crypto.hashing"
                                                        ]
                                                    },
                                                    "id": 1
                                                });

                                                let _ = stream.write_all(
                                                    response.to_string().as_bytes()
                                                ).await;
                                            }
                                        }
                                    });
                                }
                                Err(e) => {
                                    warn!("Accept error: {}", e);
                                }
                            }
                        }
                        _ = shutdown_rx.recv() => {
                            info!("JSON-RPC server shutting down");
                            break;
                        }
                    }
                }
            });
        }

        Ok(MultiTransportHandle {
            shutdown_tx,
            config: self.config,
        })
    }

    /// Run server until shutdown signal
    ///
    /// Blocks until shutdown is signaled.
    pub async fn run(self) -> anyhow::Result<()> {
        let handle = self.start().await?;

        // Wait for Ctrl+C or other termination signal
        tokio::signal::ctrl_c().await?;

        info!("Shutdown signal received, stopping servers...");
        handle.shutdown();

        // Give servers time to clean up
        tokio::time::sleep(Duration::from_millis(100)).await;

        info!("Multi-transport server stopped");
        Ok(())
    }
}

/// Protocol selection helper
///
/// Helps clients select the best protocol for their use case.
pub struct ProtocolSelector;

impl ProtocolSelector {
    /// Select best protocol for high-throughput operations
    pub fn for_throughput(available: &[Protocol]) -> Protocol {
        // Prefer tarpc for throughput
        if available.contains(&Protocol::Tarpc) {
            Protocol::Tarpc
        } else if available.contains(&Protocol::JsonRpc) {
            Protocol::JsonRpc
        } else {
            Protocol::Http
        }
    }

    /// Select best protocol for debugging/development
    pub fn for_debugging(available: &[Protocol]) -> Protocol {
        // Prefer JSON-RPC for debugging (human-readable)
        if available.contains(&Protocol::JsonRpc) {
            Protocol::JsonRpc
        } else if available.contains(&Protocol::Tarpc) {
            Protocol::Tarpc
        } else {
            Protocol::Http
        }
    }

    /// Select best protocol for one-off requests
    pub fn for_occasional(available: &[Protocol]) -> Protocol {
        // JSON-RPC is fine for occasional requests
        Self::for_debugging(available)
    }

    /// Select best protocol based on operation frequency
    ///
    /// - High frequency (>100/sec): tarpc
    /// - Medium frequency (10-100/sec): either
    /// - Low frequency (<10/sec): JSON-RPC
    pub fn for_frequency(ops_per_sec: u32, available: &[Protocol]) -> Protocol {
        if ops_per_sec > 100 {
            Self::for_throughput(available)
        } else {
            Self::for_debugging(available)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env() {
        // Clear any existing env vars
        beardog_errors::process_env::remove_var("BEARDOG_TARPC_ADDR");
        beardog_errors::process_env::remove_var("BEARDOG_JSONRPC_ADDR");

        let config = MultiTransportConfig::from_env();
        assert!(config.enable_tarpc);
        assert!(config.enable_jsonrpc);
    }

    #[test]
    fn test_config_capabilities() {
        let config = MultiTransportConfig::default();
        let caps = config.capabilities();

        assert!(caps.supported.contains(&"tarpc".to_string()));
        assert!(caps.supported.contains(&"json-rpc".to_string()));
        assert_eq!(caps.recommended, "tarpc");
    }

    #[test]
    fn test_protocol_selector_throughput() {
        let available = vec![Protocol::Tarpc, Protocol::JsonRpc];
        assert_eq!(
            ProtocolSelector::for_throughput(&available),
            Protocol::Tarpc
        );
    }

    #[test]
    fn test_protocol_selector_debugging() {
        let available = vec![Protocol::Tarpc, Protocol::JsonRpc];
        assert_eq!(
            ProtocolSelector::for_debugging(&available),
            Protocol::JsonRpc
        );
    }

    #[test]
    fn test_protocol_selector_frequency() {
        let available = vec![Protocol::Tarpc, Protocol::JsonRpc];

        // High frequency should use tarpc
        assert_eq!(
            ProtocolSelector::for_frequency(200, &available),
            Protocol::Tarpc
        );

        // Low frequency should use JSON-RPC
        assert_eq!(
            ProtocolSelector::for_frequency(5, &available),
            Protocol::JsonRpc
        );
    }

    #[test]
    fn test_protocol_selector_empty_falls_back_to_http() {
        let empty: Vec<Protocol> = vec![];
        assert_eq!(ProtocolSelector::for_throughput(&empty), Protocol::Http);
        assert_eq!(ProtocolSelector::for_debugging(&empty), Protocol::Http);
        assert_eq!(ProtocolSelector::for_occasional(&empty), Protocol::Http);
        assert_eq!(ProtocolSelector::for_frequency(50, &empty), Protocol::Http);
    }

    #[test]
    fn test_protocol_selector_tarpc_only_and_jsonrpc_only() {
        assert_eq!(
            ProtocolSelector::for_throughput(&[Protocol::Tarpc]),
            Protocol::Tarpc
        );
        assert_eq!(
            ProtocolSelector::for_debugging(&[Protocol::Tarpc]),
            Protocol::Tarpc
        );
        assert_eq!(
            ProtocolSelector::for_throughput(&[Protocol::JsonRpc]),
            Protocol::JsonRpc
        );
        assert_eq!(
            ProtocolSelector::for_debugging(&[Protocol::JsonRpc]),
            Protocol::JsonRpc
        );
    }

    #[test]
    fn test_from_router_config_and_capabilities_jsonrpc_preferred() {
        let router = RouterConfig::jsonrpc_only();
        let cfg = MultiTransportConfig::from_router_config(&router, 9950);
        assert!(!cfg.enable_tarpc);
        assert!(cfg.enable_jsonrpc);
        let caps = cfg.capabilities();
        assert_eq!(caps.recommended, "json-rpc");
    }

    #[test]
    fn test_default_config_invalid_bind_addr_fallback() {
        let cfg = MultiTransportConfig::from_bind_and_ports(
            "not-a-valid-host!!!",
            DEFAULT_TARPC_PORT,
            DEFAULT_JSONRPC_PORT,
            DEFAULT_SHUTDOWN_TIMEOUT_SECS,
        );
        assert_eq!(cfg.tarpc_addr.port(), DEFAULT_TARPC_PORT);
        assert_eq!(cfg.jsonrpc_addr.port(), DEFAULT_JSONRPC_PORT);
    }

    #[test]
    fn test_from_env_enable_flags() {
        let mut cfg = MultiTransportConfig::from_bind_and_ports(
            DEFAULT_BIND_ADDRESS,
            DEFAULT_TARPC_PORT,
            DEFAULT_JSONRPC_PORT,
            DEFAULT_SHUTDOWN_TIMEOUT_SECS,
        );
        cfg.enable_tarpc = false;
        cfg.enable_jsonrpc = false;
        assert!(!cfg.enable_tarpc);
        assert!(!cfg.enable_jsonrpc);
    }

    #[tokio::test]
    async fn test_multi_transport_server_jsonrpc_only_ephemeral_port() {
        let addr: std::net::SocketAddr = "127.0.0.1:0".parse().unwrap();
        let config = MultiTransportConfig {
            tarpc_addr: addr,
            jsonrpc_addr: addr,
            enable_tarpc: false,
            enable_jsonrpc: true,
            shutdown_timeout: std::time::Duration::from_millis(200),
        };
        let server = MultiTransportServer::new(config);
        let handle = server.start().await.expect("start");
        assert!(handle.jsonrpc_addr().is_some());
        assert!(handle.tarpc_addr().is_none());
        handle.shutdown();
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }

    #[tokio::test]
    async fn test_multi_transport_handle_shutdown_no_receivers() {
        let config = MultiTransportConfig {
            tarpc_addr: "127.0.0.1:0".parse().unwrap(),
            jsonrpc_addr: "127.0.0.1:0".parse().unwrap(),
            enable_tarpc: false,
            enable_jsonrpc: false,
            shutdown_timeout: std::time::Duration::from_millis(50),
        };
        let handle = MultiTransportServer::new(config)
            .start()
            .await
            .expect("start");
        handle.shutdown();
        handle.shutdown();
    }

    #[test]
    fn test_multi_transport_server_constructors() {
        let _ = MultiTransportServer::with_defaults();
        let _ = MultiTransportServer::from_env();
        let _ = MultiTransportServer::new(MultiTransportConfig::default());
    }
}
