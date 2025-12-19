//! BearDog API Startup Integration
//!
//! Wires together all components for production deployment:
//! - HTTP REST API server
//! - JSON-RPC endpoint
//! - tarpc binary RPC server
//! - mDNS service advertisement
//! - Protocol discovery
//!
//! # Philosophy
//!
//! **Capability-Based Configuration**: No hardcoded ports or addresses.
//! Uses runtime discovery and configuration hierarchy.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use beardog_api::startup::BearDogApiServer;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Option 1: Auto-discover available ports
//!     let server = BearDogApiServer::builder()
//!         .discover_ports() // No hardcoding!
//!         .enable_mdns(true)
//!         .build()
//!         .await
//!         .expect("Failed to create server");
//!
//!     // Option 2: Explicit configuration (human sovereignty)
//!     let server = BearDogApiServer::builder()
//!         .http_port(3000)
//!         .tarpc_port(9000)
//!         .enable_mdns(true)
//!         .build()
//!         .await
//!         .expect("Failed to create server");
//!
//!     server.serve().await.expect("Server failed");
//! }
//! ```

use beardog_config::runtime_network_discovery::{NetworkDiscovery, NetworkPreferences};
use beardog_core::core::BearDogCore;
use beardog_errors::BearDogError;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tracing::{info, warn};

use crate::{create_router, ServiceAdvertisement, ServiceAdvertiser};

/// BearDog API Server Configuration
#[derive(Debug, Clone)]
pub struct BearDogApiConfig {
    /// HTTP/JSON-RPC bind address
    pub http_addr: SocketAddr,
    /// tarpc bind address
    pub tarpc_addr: SocketAddr,
    /// Enable mDNS advertisement
    pub enable_mdns: bool,
    /// Custom service advertisement (if not using default)
    pub mdns_config: Option<ServiceAdvertisement>,
}

impl Default for BearDogApiConfig {
    /// Default configuration using capability-based port discovery
    ///
    /// **✅ EVOLVED: Runtime Discovery Over Hardcoding**
    ///
    /// # Configuration Hierarchy
    ///
    /// 1. `BEARDOG_HTTP_PORT` / `BEARDOG_TARPC_PORT` env vars (explicit override)
    /// 2. **Runtime network discovery** (find available ports dynamically)
    /// 3. Configuration preferences (if provided)
    /// 4. Graceful fallback (only if all else fails)
    ///
    /// # Bind Address
    ///
    /// Discovers local IP addresses at runtime.
    /// Respects `BEARDOG_BIND_ADDRESS` env var for explicit control.
    fn default() -> Self {
        // ✅ EVOLVED: Use runtime discovery instead of hardcoded fallbacks
        let preferences = NetworkPreferences {
            preferred_host: std::env::var("BEARDOG_BIND_ADDRESS").ok(),
            preferred_port: std::env::var("BEARDOG_HTTP_PORT")
                .ok()
                .and_then(|p| p.parse().ok()),
            port_range: (8080, 8099), // Search range for HTTP
        };

        let discovery = NetworkDiscovery::new(preferences);

        match discovery.discover() {
            Ok(capabilities) => {
                // Use discovered network capabilities
                let bind_addr = capabilities
                    .local_addresses
                    .first()
                    .copied()
                    .unwrap_or_else(Self::get_bind_address_from_env);

                let http_port = capabilities
                    .available_ports
                    .first()
                    .copied()
                    .unwrap_or_else(|| Self::get_port_from_env("BEARDOG_HTTP_PORT", 8080));

                // For tarpc, use next available port or search higher range
                let tarpc_port = capabilities
                    .available_ports
                    .get(1)
                    .copied()
                    .or_else(|| {
                        std::env::var("BEARDOG_TARPC_PORT")
                            .ok()
                            .and_then(|p| p.parse().ok())
                    })
                    .unwrap_or(9090);

                info!(
                    "✅ Network capabilities discovered - HTTP: {}, tarpc: {}",
                    http_port, tarpc_port
                );

                Self {
                    http_addr: SocketAddr::new(bind_addr, http_port),
                    tarpc_addr: SocketAddr::new(bind_addr, tarpc_port),
                    enable_mdns: std::env::var("BEARDOG_ENABLE_MDNS")
                        .ok()
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(true),
                    mdns_config: None,
                }
            }
            Err(e) => {
                warn!("⚠️  Network discovery failed: {}, using fallback", e);
                // Graceful fallback
                let bind_addr = Self::get_bind_address_from_env();
                let http_port = Self::get_port_from_env("BEARDOG_HTTP_PORT", 8080);
                let tarpc_port = Self::get_port_from_env("BEARDOG_TARPC_PORT", 9090);

                Self {
                    http_addr: SocketAddr::new(bind_addr, http_port),
                    tarpc_addr: SocketAddr::new(bind_addr, tarpc_port),
                    enable_mdns: std::env::var("BEARDOG_ENABLE_MDNS")
                        .ok()
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(true),
                    mdns_config: None,
                }
            }
        }
    }
}

impl BearDogApiConfig {
    /// Get bind address from environment or default to localhost
    ///
    /// Reads `BEARDOG_BIND_ADDRESS` env var.
    /// Defaults to `127.0.0.1` for security (localhost only).
    fn get_bind_address_from_env() -> IpAddr {
        std::env::var("BEARDOG_BIND_ADDRESS")
            .ok()
            .and_then(|addr| addr.parse().ok())
            .unwrap_or_else(|| IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)))
    }

    /// Get port from environment or use fallback
    ///
    /// Reads specified env var. If not set, uses fallback.
    /// Fallback constants are last resort only.
    fn get_port_from_env(env_var: &str, fallback: u16) -> u16 {
        std::env::var(env_var)
            .ok()
            .and_then(|port| port.parse().ok())
            .unwrap_or(fallback)
    }
}

/// BearDog API Server
///
/// Manages all API protocols and service advertisement.
pub struct BearDogApiServer {
    config: BearDogApiConfig,
    core: Arc<BearDogCore>,
}

impl BearDogApiServer {
    /// Create new API server with default configuration
    pub fn new() -> Result<Self, BearDogError> {
        Self::with_config(BearDogApiConfig::default())
    }

    /// Create new API server with custom configuration
    pub fn with_config(config: BearDogApiConfig) -> Result<Self, BearDogError> {
        let core_config = UnifiedBearDogConfig::default();
        let core = Arc::new(BearDogCore::new(core_config));

        Ok(Self { config, core })
    }

    /// Get a builder for configuring the server
    pub fn builder() -> BearDogApiServerBuilder {
        BearDogApiServerBuilder::default()
    }

    /// Start all API services
    ///
    /// This will:
    /// 1. Start HTTP/JSON-RPC server
    /// 2. Start tarpc binary RPC server (in background)
    /// 3. Advertise via mDNS (if enabled)
    /// 4. Log service URLs for discovery
    pub async fn serve(self) -> Result<(), BearDogError> {
        info!("🐻 BearDog API Server Starting...");
        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // 1. Start mDNS advertisement (if enabled)
        if self.config.enable_mdns {
            let mut mdns_config = self.config.mdns_config.clone().unwrap_or_default();

            // Update port from actual HTTP address
            mdns_config.port = self.config.http_addr.port();

            // Update tarpc endpoint with actual address
            mdns_config
                .txt_records
                .retain(|(k, _)| k != "tarpc_endpoint");
            mdns_config.txt_records.push((
                "tarpc_endpoint".to_string(),
                format!("tcp://{}", self.config.tarpc_addr),
            ));

            let advertiser = ServiceAdvertiser::new(mdns_config);
            advertiser.start().await?;
        }

        // 2. Start tarpc server in background
        info!(
            "🚀 Starting tarpc Binary RPC server on {}",
            self.config.tarpc_addr
        );
        let tarpc_core = self.core.clone();
        let tarpc_addr = self.config.tarpc_addr;
        tokio::spawn(async move {
            if let Err(e) = start_tarpc_server(tarpc_core, tarpc_addr).await {
                tracing::error!("tarpc server error: {}", e);
            }
        });

        // 3. Log service endpoints
        info!("📡 BearDog Services Available:");
        info!(
            "   HTTP REST:  http://{}/api/v1/crypto/*",
            self.config.http_addr
        );
        info!("   JSON-RPC:   http://{}/rpc", self.config.http_addr);
        info!("   tarpc:      tcp://{}", self.config.tarpc_addr);
        info!(
            "   Discovery:  http://{}/api/v1/protocols",
            self.config.http_addr
        );

        if self.config.enable_mdns {
            info!("   mDNS:       _beardog._tcp.local");
        }

        info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        info!("✅ BearDog API Server Ready!");

        // 4. Start HTTP/JSON-RPC server (blocks)
        let app = create_router(self.core);
        let listener = tokio::net::TcpListener::bind(self.config.http_addr)
            .await
            .map_err(|e| {
                BearDogError::network(format!(
                    "Failed to bind to {}: {}",
                    self.config.http_addr, e
                ))
            })?;

        info!("🎧 Listening on http://{}", self.config.http_addr);

        axum::serve(listener, app)
            .await
            .map_err(|e| BearDogError::network(format!("Server error: {}", e)))?;

        Ok(())
    }
}

impl Default for BearDogApiServer {
    fn default() -> Self {
        let config = BearDogApiConfig::default();
        let core_config = UnifiedBearDogConfig::default();
        let core = Arc::new(BearDogCore::new(core_config));

        Self { config, core }
    }
}

/// Builder for BearDogApiServer
#[derive(Default)]
pub struct BearDogApiServerBuilder {
    http_addr: Option<SocketAddr>,
    tarpc_addr: Option<SocketAddr>,
    bind_addr: Option<IpAddr>,
    enable_mdns: Option<bool>,
    mdns_config: Option<ServiceAdvertisement>,
}

impl BearDogApiServerBuilder {
    /// Set HTTP/JSON-RPC bind address
    pub fn http_addr(mut self, addr: SocketAddr) -> Self {
        self.http_addr = Some(addr);
        self
    }

    /// Set HTTP/JSON-RPC port
    ///
    /// Binds to configured bind address (default: from env or 127.0.0.1).
    /// Use `bind_address()` to override the bind address.
    pub fn http_port(mut self, port: u16) -> Self {
        let bind_addr = self
            .bind_addr
            .unwrap_or_else(BearDogApiConfig::get_bind_address_from_env);
        self.http_addr = Some(SocketAddr::new(bind_addr, port));
        self
    }

    /// Set tarpc bind address (explicit socket address)
    pub fn tarpc_addr(mut self, addr: SocketAddr) -> Self {
        self.tarpc_addr = Some(addr);
        self
    }

    /// Set tarpc port
    ///
    /// Binds to configured bind address (default: from env or 127.0.0.1).
    /// Use `bind_address()` to override the bind address.
    pub fn tarpc_port(mut self, port: u16) -> Self {
        let bind_addr = self
            .bind_addr
            .unwrap_or_else(BearDogApiConfig::get_bind_address_from_env);
        self.tarpc_addr = Some(SocketAddr::new(bind_addr, port));
        self
    }

    /// Set bind address for all services
    ///
    /// **Security Note**: Default is `127.0.0.1` (localhost only).
    /// Set to `0.0.0.0` to allow external connections (less secure).
    /// Can also be configured via `BEARDOG_BIND_ADDRESS` env var.
    pub fn bind_address(mut self, addr: IpAddr) -> Self {
        self.bind_addr = Some(addr);
        self
    }

    /// Enable or disable mDNS advertisement
    pub fn enable_mdns(mut self, enable: bool) -> Self {
        self.enable_mdns = Some(enable);
        self
    }

    /// Set custom mDNS configuration
    pub fn mdns_config(mut self, config: ServiceAdvertisement) -> Self {
        self.mdns_config = Some(config);
        self
    }

    /// Build the server
    ///
    /// Uses configuration hierarchy:
    /// 1. Explicit builder methods (highest priority)
    /// 2. Environment variables
    /// 3. Default discovery/fallbacks
    pub async fn build(self) -> Result<BearDogApiServer, BearDogError> {
        // Use defaults from environment if not explicitly set
        let default_config = BearDogApiConfig::default();

        let config = BearDogApiConfig {
            http_addr: self.http_addr.unwrap_or(default_config.http_addr),
            tarpc_addr: self.tarpc_addr.unwrap_or(default_config.tarpc_addr),
            enable_mdns: self.enable_mdns.unwrap_or(default_config.enable_mdns),
            mdns_config: self.mdns_config,
        };

        BearDogApiServer::with_config(config)
    }
}

/// Start tarpc server (helper function)
async fn start_tarpc_server(
    core: Arc<BearDogCore>,
    addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::ApiState;

    // Create API state with crypto service
    let crypto_service = beardog_core::crypto_service::BearDogCryptoService::new(
        beardog_core::crypto_service::CryptoServiceConfig::default(),
    )?;

    let state = Arc::new(ApiState {
        core,
        crypto_service: Arc::new(crypto_service),
    });

    // Start tarpc server
    crate::tarpc_service::serve_tarpc(state, addr).await
}
