// BTSP Server Example
// Starts a BTSP HTTP API server for demonstration purposes

use beardog_errors::BearDogError;
use beardog_genetics::ecosystem_evolution::EcosystemGeneticEngine;
use beardog_tunnel::btsp_api_server::BtspApiServer;
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use std::sync::Arc;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("🐻🎵 Starting BearDog BTSP Server...");

    // Initialize components
    info!("Initializing HSM Manager...");
    let hsm_manager = Arc::new(HsmManager::new());

    info!("Initializing Genetic Engine...");
    let genetics_engine = Arc::new(
        EcosystemGeneticEngine::new()
            .map_err(|e| BearDogError::system(format!("Failed to initialize genetics: {}", e)))?,
    );

    info!("Initializing BTSP Provider...");
    let btsp_provider = Arc::new(BeardogBtspProvider::new(hsm_manager, genetics_engine).await?);

    // Get port from environment or use default
    let port = std::env::var("BTSP_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(9000);

    info!("✅ All components initialized");
    info!("🚀 Starting BTSP API server on port {}...", port);
    info!("");
    info!("📡 Endpoints available:");
    info!("  • GET    http://localhost:{}/health", port);
    info!("  • POST   http://localhost:{}/btsp/tunnel/establish", port);
    info!("  • POST   http://localhost:{}/btsp/tunnel/encrypt", port);
    info!("  • POST   http://localhost:{}/btsp/tunnel/decrypt", port);
    info!(
        "  • GET    http://localhost:{}/btsp/tunnel/status/:id",
        port
    );
    info!("  • DELETE http://localhost:{}/btsp/tunnel/close/:id", port);
    info!("");
    info!("🎭 Try the showcase demos:");
    info!("  cd showcase/02-ecosystem-integration/01-songbird-btsp/phase0-foundation");
    info!("  ./04-full-api-demo.sh");
    info!("");
    info!("Press Ctrl+C to stop the server");
    info!("");

    // Create and start server (blocks until Ctrl+C)
    let server = BtspApiServer::new(btsp_provider, port);
    server.start().await?;

    Ok(())
}
