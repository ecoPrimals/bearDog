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
use tracing::info;

use beardog_genetics::birdsong::GenesisLineageProvider;

use crate::btsp_provider::BeardogBtspProvider;

use super::{
    btsp::{routes as btsp_routes, BtspApiState},
    genesis::{routes as genesis_routes, GenesisApiState},
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
    fn default() -> Self {
        Self {
            // SAFETY: "127.0.0.1:9000" is a valid SocketAddr literal
            // This is a const-time operation that cannot fail with valid input
            bind_addr: "127.0.0.1:9000".parse().expect("Valid SocketAddr literal"),
            enable_cors: true,
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
    // TODO: Add BirdSong and Lineage when implementation is complete
}

impl BearDogApiServer {
    /// Create new API server
    pub async fn new(
        config: BearDogApiServerConfig,
        btsp_provider: Arc<BeardogBtspProvider>,
    ) -> Result<Self, beardog_errors::BearDogError> {
        // Initialize Genesis provider
        let genesis_provider = Arc::new(GenesisLineageProvider::new().await?);

        Ok(Self {
            config,
            btsp_provider,
            genesis_provider,
        })
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
                    // TODO: Add when implemented
                    // "birdsong".into(),
                    // "lineage".into(),
                ],
            })
        };

        // Build router with implemented capability routes
        let mut router = Router::new()
            .route("/health", get(health_handler))
            .route("/", get(root_handler))
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
            );
        // TODO: Add BirdSong and Lineage routes when implementation is complete

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
        assert_eq!(config.bind_addr.to_string(), "127.0.0.1:9000");
        assert!(config.enable_cors);
    }

    #[tokio::test]
    async fn test_server_creation() {
        use crate::btsp_provider::BeardogBtspProvider;
        use crate::tunnel::hsm::manager::HsmManager;
        use beardog_genetics::EcosystemGeneticEngine;

        let config = BearDogApiServerConfig::default();

        // Initialize required providers
        let hsm = Arc::new(HsmManager::new());
        let genetics = Arc::new(EcosystemGeneticEngine::new().unwrap());
        let provider = Arc::new(BeardogBtspProvider::new(hsm, genetics).await.unwrap());

        let server = BearDogApiServer::new(config, provider).await;

        assert!(server.is_ok());
    }
}
