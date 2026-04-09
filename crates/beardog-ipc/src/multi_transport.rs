// SPDX-License-Identifier: AGPL-3.0-or-later

//! # 🌐 Multi-Transport Server for `BearDog`
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

/// Default port for tarpc server (can be overridden by `BEARDOG_TARPC_PORT`)
const DEFAULT_TARPC_PORT: u16 = beardog_config::DEFAULT_TCP_IPC_PORT + 1;

/// Default port for JSON-RPC server (can be overridden by `BEARDOG_JSONRPC_PORT`)
const DEFAULT_JSONRPC_PORT: u16 = beardog_config::DEFAULT_TCP_IPC_PORT;

/// Default shutdown timeout in seconds (can be overridden by `BEARDOG_SHUTDOWN_TIMEOUT`)
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

        if let Ok(addr) = beardog_errors::process_env::var("BEARDOG_TARPC_ADDR")
            && let Ok(parsed) = addr.parse()
        {
            config.tarpc_addr = parsed;
        }

        if let Ok(addr) = beardog_errors::process_env::var("BEARDOG_JSONRPC_ADDR")
            && let Ok(parsed) = addr.parse()
        {
            config.jsonrpc_addr = parsed;
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

    /// Background server tasks (tarpc / JSON-RPC) — await after shutdown for clean teardown
    server_tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl MultiTransportHandle {
    /// Signal graceful shutdown
    pub fn shutdown(&self) {
        if let Err(e) = self.shutdown_tx.send(()) {
            warn!("No receivers for shutdown signal: {}", e);
        }
    }

    /// Wait for spawned server tasks to finish, bounded by `timeout`.
    ///
    /// Call after [`Self::shutdown`] so listeners exit their `select!` loops.
    pub async fn join_servers_with_timeout(self, timeout: Duration) {
        let Self { server_tasks, .. } = self;
        let _ = tokio::time::timeout(timeout, async move {
            for task in server_tasks {
                let _ = task.await;
            }
        })
        .await;
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
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying server setup fails before tasks are spawned.
    pub async fn start(self) -> anyhow::Result<MultiTransportHandle> {
        let (shutdown_tx, _) = broadcast::channel(1);
        let mut server_tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();

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

            let h = tokio::spawn(async move {
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
            server_tasks.push(h);
        }

        // Start JSON-RPC server
        if self.config.enable_jsonrpc {
            let jsonrpc_addr = self.config.jsonrpc_addr;
            let mut shutdown_rx = shutdown_tx.subscribe();

            let h = tokio::spawn(async move {
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

                                    // NDJSON: newline-delimited JSON-RPC per PRIMAL_IPC_PROTOCOL v3.1
                                    tokio::spawn(async move {
                                        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

                                        let (reader, mut writer) = stream.split();
                                        let mut lines = BufReader::new(reader).lines();
                                        let read_timeout = std::time::Duration::from_secs(30);

                                        loop {
                                            let line = match tokio::time::timeout(read_timeout, lines.next_line()).await {
                                                Ok(Ok(Some(line))) => line,
                                                Ok(Ok(None)) => break,
                                                Ok(Err(_)) => break,
                                                Err(_) => {
                                                    warn!("TCP JSON-RPC read timeout ({}s) — dropping idle connection", read_timeout.as_secs());
                                                    break;
                                                }
                                            };
                                            if line.is_empty() {
                                                continue;
                                            }
                                            if let Some(response) =
                                                handle_jsonrpc_request_line(&line)
                                            {
                                                let mut resp = response;
                                                resp.push('\n');
                                                let _ = writer.write_all(resp.as_bytes()).await;
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
            server_tasks.push(h);
        }

        Ok(MultiTransportHandle {
            shutdown_tx,
            config: self.config,
            server_tasks,
        })
    }

    /// Run server until shutdown signal
    ///
    /// Blocks until shutdown is signaled.
    ///
    /// # Errors
    ///
    /// Propagates errors from [`Self::start`], or from waiting for Ctrl+C / joining server tasks.
    pub async fn run(self) -> anyhow::Result<()> {
        let handle = self.start().await?;

        // Wait for Ctrl+C or other termination signal
        tokio::signal::ctrl_c().await?;

        info!("Shutdown signal received, stopping servers...");
        let shutdown_timeout = handle.config.shutdown_timeout;
        handle.shutdown();
        handle.join_servers_with_timeout(shutdown_timeout).await;

        info!("Multi-transport server stopped");
        Ok(())
    }
}

/// Minimal JSON-RPC 2.0 responder for the TCP listener: handles parse/validation errors and
/// capability discovery (`rpc.discover`, `system.capabilities`, `capabilities.list`). Broader method
/// dispatch belongs in a shared router that delegates to the same service types as
/// [`BearDogCryptoServer`] (tarpc path), keeping serde request/response types aligned.
fn handle_jsonrpc_request_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    let v: serde_json::Value = match serde_json::from_str(trimmed) {
        Ok(v) => v,
        Err(_) => {
            return Some(
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": { "code": -32700, "message": "Parse error" },
                    "id": null
                })
                .to_string(),
            );
        }
    };

    let Some(obj) = v.as_object() else {
        return Some(
            serde_json::json!({
                "jsonrpc": "2.0",
                "error": { "code": -32600, "message": "Invalid Request" },
                "id": null
            })
            .to_string(),
        );
    };

    if obj.get("jsonrpc") != Some(&serde_json::json!("2.0")) {
        return Some(
            serde_json::json!({
                "jsonrpc": "2.0",
                "error": { "code": -32600, "message": "Invalid Request" },
                "id": obj.get("id").cloned().unwrap_or(serde_json::Value::Null)
            })
            .to_string(),
        );
    }

    if !obj.contains_key("id") {
        return None;
    }

    let id = obj.get("id").cloned().unwrap_or(serde_json::Value::Null);
    let method = obj.get("method").and_then(|m| m.as_str()).unwrap_or("");

    let discovery_methods = [
        "rpc.discover",
        "system.capabilities",
        "capabilities.list",
        "capability.list",
        "primal.capabilities",
    ];
    if discovery_methods.contains(&method) {
        return Some(
            serde_json::json!({
                "jsonrpc": "2.0",
                "result": {
                    "primal": "beardog",
                    "version": env!("CARGO_PKG_VERSION"),
                    "methods": [
                        "crypto.sign_ed25519",
                        "crypto.verify_ed25519",
                        "crypto.blake3_hash",
                        "crypto.hmac_sha256",
                        "crypto.chacha20_poly1305_encrypt",
                        "crypto.chacha20_poly1305_decrypt",
                        "crypto.x25519_generate_ephemeral",
                        "crypto.x25519_derive_secret",
                        "health.liveness",
                        "health.readiness",
                        "health.check",
                        "capabilities.list",
                        "identity.get"
                    ],
                    "provided_capabilities": [
                        {
                            "type": "crypto",
                            "methods": ["sign_ed25519", "verify_ed25519", "blake3_hash", "hmac_sha256",
                                        "chacha20_poly1305_encrypt", "chacha20_poly1305_decrypt",
                                        "x25519_generate_ephemeral", "x25519_derive_secret"],
                            "version": "1.0",
                            "description": "Pure Rust cryptographic operations"
                        },
                        {
                            "type": "security",
                            "methods": ["evaluate", "lineage", "generate_jwt_secret"],
                            "version": "1.0",
                            "description": "Security provider - trust evaluation, genetic lineage"
                        }
                    ],
                    "consumed_capabilities": [],
                    "protocols": ["tarpc", "json-rpc"],
                    "transport": ["uds", "tcp"]
                },
                "id": id
            })
            .to_string(),
        );
    }

    if method.is_empty() {
        return Some(
            serde_json::json!({
                "jsonrpc": "2.0",
                "error": { "code": -32600, "message": "Invalid Request" },
                "id": id
            })
            .to_string(),
        );
    }

    Some(
        serde_json::json!({
            "jsonrpc": "2.0",
            "error": { "code": -32601, "message": "Method not found" },
            "id": id
        })
        .to_string(),
    )
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
#[path = "multi_transport_tests.rs"]
mod tests;
