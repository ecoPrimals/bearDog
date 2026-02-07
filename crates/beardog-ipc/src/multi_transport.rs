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
//! │  │   (Port 9901)       │    │   (Port 9900)       │          │
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

use std::net::SocketAddr;
use std::time::Duration;

use tokio::sync::broadcast;
use tracing::{error, info, warn};

use crate::protocol_router::{Protocol, ProtocolCapabilities, RouterConfig};
use crate::tarpc_server::BearDogCryptoServer;

/// Multi-transport server configuration
#[derive(Debug, Clone)]
pub struct MultiTransportConfig {
    /// tarpc server address (default: 127.0.0.1:9901)
    pub tarpc_addr: SocketAddr,
    
    /// JSON-RPC server address (default: 127.0.0.1:9900)
    pub jsonrpc_addr: SocketAddr,
    
    /// Enable tarpc server
    pub enable_tarpc: bool,
    
    /// Enable JSON-RPC server
    pub enable_jsonrpc: bool,
    
    /// Graceful shutdown timeout
    pub shutdown_timeout: Duration,
}

impl Default for MultiTransportConfig {
    fn default() -> Self {
        // Self-knowledge: discover bind address from environment
        let bind_addr = std::env::var("BEARDOG_BIND_ADDR")
            .unwrap_or_else(|_| "127.0.0.1".to_string());
        
        // Safe address parsing with fallback to known-good defaults
        // This prevents panics from malformed BEARDOG_BIND_ADDR values
        let tarpc_addr = format!("{}:9901", bind_addr)
            .parse()
            .unwrap_or_else(|_| {
                tracing::warn!(
                    "Failed to parse tarpc address '{}:9901', using 127.0.0.1:9901",
                    bind_addr
                );
                "127.0.0.1:9901".parse().expect("hardcoded address is valid")
            });
        
        let jsonrpc_addr = format!("{}:9900", bind_addr)
            .parse()
            .unwrap_or_else(|_| {
                tracing::warn!(
                    "Failed to parse jsonrpc address '{}:9900', using 127.0.0.1:9900",
                    bind_addr
                );
                "127.0.0.1:9900".parse().expect("hardcoded address is valid")
            });
        
        Self {
            tarpc_addr,
            jsonrpc_addr,
            enable_tarpc: true,
            enable_jsonrpc: true,
            shutdown_timeout: Duration::from_secs(30),
        }
    }
}

impl MultiTransportConfig {
    /// Create from environment variables (capability-based, not hardcoded)
    ///
    /// Env vars:
    /// - `BEARDOG_TARPC_ADDR`: tarpc server address
    /// - `BEARDOG_JSONRPC_ADDR`: JSON-RPC server address
    /// - `BEARDOG_ENABLE_TARPC`: Enable tarpc (default: true)
    /// - `BEARDOG_ENABLE_JSONRPC`: Enable JSON-RPC (default: true)
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        if let Ok(addr) = std::env::var("BEARDOG_TARPC_ADDR") {
            if let Ok(parsed) = addr.parse() {
                config.tarpc_addr = parsed;
            }
        }
        
        if let Ok(addr) = std::env::var("BEARDOG_JSONRPC_ADDR") {
            if let Ok(parsed) = addr.parse() {
                config.jsonrpc_addr = parsed;
            }
        }
        
        if let Ok(val) = std::env::var("BEARDOG_ENABLE_TARPC") {
            config.enable_tarpc = val != "0" && val.to_lowercase() != "false";
        }
        
        if let Ok(val) = std::env::var("BEARDOG_ENABLE_JSONRPC") {
            config.enable_jsonrpc = val != "0" && val.to_lowercase() != "false";
        }
        
        config
    }

    /// Create from router config and base port
    ///
    /// Uses config-based bind address with self-knowledge pattern.
    /// Falls back to localhost if the provided address is invalid.
    pub fn from_router_config(router: &RouterConfig, base_port: u16) -> Self {
        // Self-knowledge: discover bind address from config or environment
        let bind_addr = std::env::var("BEARDOG_BIND_ADDR")
            .unwrap_or_else(|_| "127.0.0.1".to_string());
        
        // Safe address parsing with fallback - prevents panics from invalid env vars
        let tarpc_port = base_port.saturating_add(1);
        let tarpc_addr = format!("{}:{}", bind_addr, tarpc_port)
            .parse()
            .unwrap_or_else(|_| {
                tracing::warn!(
                    "Failed to parse tarpc address '{}:{}', using 127.0.0.1:{}",
                    bind_addr, tarpc_port, tarpc_port
                );
                format!("127.0.0.1:{}", tarpc_port)
                    .parse()
                    .expect("hardcoded localhost address is valid")
            });
        
        let jsonrpc_addr = format!("{}:{}", bind_addr, base_port)
            .parse()
            .unwrap_or_else(|_| {
                tracing::warn!(
                    "Failed to parse jsonrpc address '{}:{}', using 127.0.0.1:{}",
                    bind_addr, base_port, base_port
                );
                format!("127.0.0.1:{}", base_port)
                    .parse()
                    .expect("hardcoded localhost address is valid")
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
    pub fn config(&self) -> &MultiTransportConfig {
        &self.config
    }

    /// Get tarpc address (if enabled)
    pub fn tarpc_addr(&self) -> Option<SocketAddr> {
        if self.config.enable_tarpc {
            Some(self.config.tarpc_addr)
        } else {
            None
        }
    }

    /// Get JSON-RPC address (if enabled)
    pub fn jsonrpc_addr(&self) -> Option<SocketAddr> {
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
    pub fn new(config: MultiTransportConfig) -> Self {
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
        info!("   - tarpc:    {} ({})", 
            self.config.tarpc_addr, 
            if self.config.enable_tarpc { "enabled" } else { "disabled" }
        );
        info!("   - JSON-RPC: {} ({})", 
            self.config.jsonrpc_addr,
            if self.config.enable_jsonrpc { "enabled" } else { "disabled" }
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
        std::env::remove_var("BEARDOG_TARPC_ADDR");
        std::env::remove_var("BEARDOG_JSONRPC_ADDR");
        
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
}
