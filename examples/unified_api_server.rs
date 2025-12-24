//! Unified BearDog API Server Example
//!
//! Demonstrates the unified API server with Genesis and BTSP capabilities.

use std::sync::Arc;

use beardog_genetics::ecosystem_evolution::engine::EcosystemGeneticEngine;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use beardog_tunnel::{BearDogApiServer, BearDogApiServerConfig, BeardogBtspProvider};

#[tokio::main]
async fn main() -> Result<(), beardog_errors::BearDogError> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    tracing::info!("🐻 Starting BearDog Unified API Server");

    // Initialize dependencies
    let hsm_manager = Arc::new(HsmManager::new());
    let genetic_engine = Arc::new(EcosystemGeneticEngine::new()?);

    // Create BTSP provider
    let btsp_provider = Arc::new(BeardogBtspProvider::new(hsm_manager, genetic_engine).await?);

    // Create server configuration
    let config = BearDogApiServerConfig {
        bind_addr: "127.0.0.1:9000".parse().unwrap(),
        enable_cors: true,
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    // Create and start server
    let server = BearDogApiServer::new(config, btsp_provider).await?;

    tracing::info!("🚀 Server ready at http://127.0.0.1:9000");
    tracing::info!("📖 Try:");
    tracing::info!("   curl http://127.0.0.1:9000/health");
    tracing::info!("   curl http://127.0.0.1:9000/");

    server.serve().await?;

    Ok(())
}
