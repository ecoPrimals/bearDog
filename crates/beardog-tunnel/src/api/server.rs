//! BearDog Unified API Server
//!
//! Modern Axum-based HTTP server exposing all BearDog capabilities:
//! - BTSP (Secure Tunnels)
//! - Genesis (Physical Bootstrap)
//! - BirdSong (Privacy-Preserving Broadcasts)
//! - Lineage (Cryptographic Proofs)

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    http::{header, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info, warn};

use beardog_config::ZeroHardcodingConfig;
use beardog_genetics::birdsong::{
    BirdSongManager, GenesisLineageProvider, LineageChainManager, LineageProofManager,
};

use crate::btsp_provider::BeardogBtspProvider;

use super::{
    birdsong::{routes as birdsong_routes, routes_v2 as birdsong_routes_v2, BirdSongApiState},
    btsp::{routes as btsp_routes, BtspApiState},
    genesis::{routes as genesis_routes, GenesisApiState},
    lineage::{routes as lineage_routes, LineageApiState},
    trust::{routes as trust_routes, TrustApiState},
    types::HealthResponse,
};

/// BearDog API server configuration
#[derive(Debug, Clone)]
pub struct BearDogApiServerConfig {
    /// Bind address
    pub bind_addr: SocketAddr,
    /// Enable CORS
    pub enable_cors: bool,
    /// Service version
    pub version: String,
}

impl Default for BearDogApiServerConfig {
    /// Create default configuration from environment
    ///
    /// Uses zero-hardcoding infrastructure:
    /// - `BEARDOG_HTTP_PORT` or 0 (OS auto-select)
    /// - `BEARDOG_BIND_ADDR` or 0.0.0.0
    /// - `BEARDOG_ENABLE_CORS` or true
    fn default() -> Self {
        // Use zero-hardcoding config for bind address
        let zero_config = ZeroHardcodingConfig::from_env();
        let bind_addr = zero_config.endpoints.http_socket_addr();

        Self {
            bind_addr,
            enable_cors: std::env::var("BEARDOG_ENABLE_CORS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// BearDog API server
pub struct BearDogApiServer {
    /// Server configuration
    config: BearDogApiServerConfig,
    /// BTSP provider
    btsp_provider: Arc<BeardogBtspProvider>,
    /// Genesis provider
    genesis_provider: Arc<GenesisLineageProvider>,
    /// BirdSong manager
    birdsong_manager: Arc<BirdSongManager>,
    /// Lineage chain manager
    lineage_chain_manager: Arc<LineageChainManager>,
    /// Lineage proof manager
    lineage_proof_manager: Arc<LineageProofManager>,
    /// Family ID (from USB seed or genesis)
    family_id: Option<String>,
    /// Node ID (for encryption tags)
    node_id: String,
}

impl BearDogApiServer {
    /// Create new API server
    pub async fn new(
        config: BearDogApiServerConfig,
        btsp_provider: Arc<BeardogBtspProvider>,
    ) -> Result<Self, beardog_errors::BearDogError> {
        // Initialize Genesis provider
        let genesis_provider = Arc::new(GenesisLineageProvider::new().await?);

        // Use BirdSong manager from BTSP provider (shares same HSM-derived master key)
        let birdsong_manager = btsp_provider.birdsong_manager();

        // Initialize Lineage managers
        let lineage_chain_manager = Arc::new(LineageChainManager::new());
        let lineage_proof_manager =
            Arc::new(LineageProofManager::new(lineage_chain_manager.clone()));

        // Check for USB family seed and create child lineage if present
        let (family_id, node_id) = if let Ok(family_seed) = std::env::var("BEARDOG_FAMILY_SEED") {
            info!("🔐 USB family seed detected, creating child lineage");

            // Extract family ID from seed (first 4 alphanumeric chars of base64)
            let family_id: String = family_seed
                .chars()
                .filter(|c| c.is_alphanumeric())
                .take(4)
                .collect();
            let family_id = family_id.to_lowercase();

            // Generate node ID (mix seed + machine entropy)
            let hostname = hostname::get()
                .ok()
                .and_then(|h| h.into_string().ok())
                .unwrap_or_else(|| "tower".to_string());

            let node_id = format!(
                "{}_{}",
                hostname,
                uuid::Uuid::new_v4().to_string().split('-').next().unwrap()
            );

            info!(
                "✅ Child lineage created: family={}, node={}",
                family_id, node_id
            );

            // Attempt to create genesis lineage (non-blocking, graceful fallback)
            let root_node_id = format!("{}-genesis", family_id);
            match lineage_chain_manager
                .generate_root_chain(root_node_id.clone(), Default::default())
                .await
            {
                Ok(genesis) => {
                    info!("✅ Family genesis created: {}", genesis.chain_id);
                    // Genesis created successfully - lineage proofs will work
                }
                Err(e) => {
                    warn!("⚠️ Failed to create family genesis: {}", e);
                    warn!("   Server will continue without genetic lineage proofs");
                    warn!("   BirdSong encryption will still work for discovery");
                    // Continue without genesis - not a fatal error
                }
            }

            (Some(family_id), node_id)
        } else {
            info!("ℹ️  No USB family seed, starting without family lineage");
            // Generate a unique node ID
            let node_id = format!(
                "node_{}",
                uuid::Uuid::new_v4().to_string().split('-').next().unwrap()
            );
            (None, node_id)
        };

        Ok(Self {
            config,
            btsp_provider,
            genesis_provider,
            birdsong_manager,
            lineage_chain_manager,
            lineage_proof_manager,
            family_id,
            node_id,
        })
    }

    /// Get the configured bind address
    pub fn local_addr(&self) -> SocketAddr {
        self.config.bind_addr
    }

    /// Build the application router
    fn app(&self) -> Router {
        // Health check handler
        let version = self.config.version.clone();
        let health_handler = move || async move {
            Json(HealthResponse {
                status: "healthy".into(),
                version: version.clone(),
                capabilities: vec![
                    "btsp".into(),
                    "genesis".into(),
                    "birdsong".into(),
                    "lineage".into(),
                    "trust".into(),
                ],
            })
        };

        // Build router with all capability routes
        // API v1 routes
        let api_v1 = Router::new()
            .nest(
                "/btsp",
                btsp_routes(BtspApiState {
                    provider: self.btsp_provider.clone(),
                }),
            )
            .nest(
                "/genesis",
                genesis_routes(GenesisApiState {
                    provider: self.genesis_provider.clone(),
                }),
            )
            .nest(
                "/birdsong",
                birdsong_routes(BirdSongApiState {
                    manager: self.birdsong_manager.clone(),
                }),
            )
            .nest(
                "/lineage",
                lineage_routes(LineageApiState::new(
                    self.lineage_chain_manager.clone(),
                    self.lineage_proof_manager.clone(),
                )),
            )
            .nest(
                "/trust",
                trust_routes(TrustApiState::new(self.family_id.clone(), &self.node_id)),
            );

        // API v2 routes (Songbird-compatible, clean generic interface)
        let api_v2 = Router::new().nest(
            "/birdsong",
            birdsong_routes_v2(BirdSongApiState {
                manager: self.birdsong_manager.clone(),
            }),
        );

        // Mount API v1 and v2 routes
        let mut router = Router::new()
            .route("/health", get(health_handler))
            .route("/", get(root_handler))
            .nest("/api/v1", api_v1)
            .nest("/api/v2", api_v2);

        // Add middleware layers
        let middleware = ServiceBuilder::new().layer(TraceLayer::new_for_http());

        router = router.layer(middleware);

        // Add CORS if enabled
        if self.config.enable_cors {
            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::DELETE])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);
            router = router.layer(cors);
        }

        router
    }

    /// Start the API server
    pub async fn serve(self) -> Result<(), beardog_errors::BearDogError> {
        let app = self.app();
        let listener = TcpListener::bind(&self.config.bind_addr)
            .await
            .map_err(|e| {
                beardog_errors::BearDogError::network(format!("Failed to bind to address: {}", e))
            })?;

        info!(
            "🚀 BearDog API Server starting on {}",
            self.config.bind_addr
        );
        info!("📡 Capabilities: BTSP, Genesis");
        info!("🔗 Health check: http://{}/health", self.config.bind_addr);
        info!("📖 API endpoints:");
        info!("   BTSP:    http://{}/btsp/*", self.config.bind_addr);
        info!("   Genesis: http://{}/genesis/*", self.config.bind_addr);

        axum::serve(listener, app)
            .await
            .map_err(|e| beardog_errors::BearDogError::network(format!("Server error: {}", e)))?;

        Ok(())
    }
}

/// Root handler
async fn root_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "service": "BearDog API",
            "version": env!("CARGO_PKG_VERSION"),
            "capabilities": ["btsp", "genesis"],
            "endpoints": {
                "health": "/health",
                "btsp": "/btsp/*",
                "genesis": "/genesis/*"
            }
        })),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = BearDogApiServerConfig::default();
        // Default uses zero-hardcoding (0.0.0.0:0 = OS auto-select)
        assert_eq!(config.bind_addr.to_string(), "0.0.0.0:0");
        assert!(config.enable_cors);
    }

    #[tokio::test]
    async fn test_server_creation() {
        use crate::btsp_provider::BeardogBtspProvider;
        use crate::tunnel::hsm::manager::HsmManager;
        use beardog_genetics::EcosystemGeneticEngine;
        use std::env;

        let config = BearDogApiServerConfig::default();

        // Use auto_initialize for modern idiomatic initialization
        env::set_var("BEARDOG_HSM_MODE", "software");
        let hsm = Arc::new(HsmManager::auto_initialize().await.unwrap());
        env::remove_var("BEARDOG_HSM_MODE");

        let genetics = Arc::new(EcosystemGeneticEngine::new().unwrap());
        let provider = Arc::new(BeardogBtspProvider::new(hsm, genetics).await.unwrap());

        let server = BearDogApiServer::new(config, provider).await;

        assert!(server.is_ok());
    }
}
