//! BearDog Standalone Service
//!
//! Standalone primal service for sovereign tower orchestration.
//! Designed to be spawned by biomeOS tower as a separate process.
//!
//! # Architecture
//!
//! ```text
//! Tower (Orchestrator)
//!   ├─ Spawns → BearDog (this binary)
//!   ├─ Spawns → Songbird
//!   └─ Spawns → Other primals...
//!
//! Communication:
//!   • Unix Sockets (primary) - /tmp/primals/beardog-{node}.sock
//!   • JSON-RPC / tarpc
//!   • HTTP (lowest security)
//! ```
//!
//! # Environment Variables
//!
//! - `BEARDOG_HSM_MODE` - HSM mode (default: "software")
//! - `BEARDOG_BIND_ADDR` - HTTP bind address (e.g., "0.0.0.0:9000")
//! - `HTTP_PORT` - HTTP port (alternative to BEARDOG_BIND_ADDR)
//! - `BEARDOG_FAMILY_SEED` - Family lineage seed (optional)
//! - `NODE_ID` - Node identifier for socket path
//! - `BEARDOG_ENABLE_CORS` - Enable CORS (default: true)
//!
//! # Usage
//!
//! ```bash
//! # Start with default settings
//! beardog-server
//!
//! # Start with custom port
//! BEARDOG_BIND_ADDR=0.0.0.0:19000 beardog-server
//!
//! # Or using HTTP_PORT
//! HTTP_PORT=19000 beardog-server
//!
//! # With family seed
//! BEARDOG_FAMILY_SEED="$(cat /media/usb/seed.txt)" beardog-server
//! ```

use beardog_errors::BearDogError;
use beardog_genetics::EcosystemGeneticEngine;
use beardog_tunnel::api::{BearDogApiServer, BearDogApiServerConfig};
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::tunnel::hsm::HsmManager;
use std::sync::Arc;
use tokio::signal;
use tracing::{error, info, warn, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    let log_level = std::env::var("RUST_LOG")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(Level::INFO);

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .init();

    info!("╔════════════════════════════════════════════════════════════════════╗");
    info!("║                                                                    ║");
    info!("║         🐻 BearDog Standalone Service v{}                     ║", env!("CARGO_PKG_VERSION"));
    info!("║                                                                    ║");
    info!("║              Sovereign Primal for Tower Orchestration             ║");
    info!("║                                                                    ║");
    info!("╚════════════════════════════════════════════════════════════════════╝");
    info!("");

    // Step 1: Initialize HSM Manager
    info!("🔐 Step 1: Initializing HSM Manager...");
    let hsm_mode = std::env::var("BEARDOG_HSM_MODE").unwrap_or_else(|_| "software".to_string());
    info!("   HSM Mode: {}", hsm_mode);

    let hsm = Arc::new(HsmManager::auto_initialize().await.map_err(|e| {
        error!("Failed to initialize HSM: {}", e);
        e
    })?);
    info!("✅ HSM Manager initialized successfully");
    info!("");

    // Step 2: Initialize Genetic Engine
    info!("🧬 Step 2: Initializing Genetic Engine...");
    let genetics = Arc::new(EcosystemGeneticEngine::new().map_err(|e| {
        error!("Failed to initialize genetic engine: {}", e);
        e
    })?);
    info!("✅ Genetic Engine initialized");
    info!("");

    // Step 3: Load Family Seed (if provided)
    if let Ok(family_seed) = std::env::var("BEARDOG_FAMILY_SEED") {
        info!("👨‍👩‍👧‍👦 Family lineage seed detected");
        let family_id: String = family_seed
            .chars()
            .filter(|c| c.is_alphanumeric())
            .take(4)
            .collect();
        info!("   Family ID: {}", family_id.to_lowercase());
        info!("   Genetic siblings will auto-trust this family");
        info!("");
    } else {
        info!("ℹ️  No family seed provided (running in standalone mode)");
        info!("");
    }

    // Step 4: Create BTSP Provider
    info!("🛡️  Step 3: Creating BTSP Provider...");
    let btsp_provider = Arc::new(BeardogBtspProvider::new(hsm, genetics).await.map_err(|e| {
        error!("Failed to create BTSP provider: {}", e);
        e
    })?);
    info!("✅ BTSP Provider created");
    info!("");

    // Step 5: Configure API Server
    info!("🚀 Step 4: Configuring API Server...");

    // Determine bind address from environment
    let bind_addr = std::env::var("BEARDOG_BIND_ADDR")
        .or_else(|_| {
            std::env::var("HTTP_PORT").map(|port| format!("0.0.0.0:{}", port))
        })
        .unwrap_or_else(|_| {
            warn!("   No BEARDOG_BIND_ADDR or HTTP_PORT set, using default: 0.0.0.0:9000");
            "0.0.0.0:9000".to_string()
        })
        .parse()
        .map_err(|e| {
            error!("Invalid bind address: {}", e);
            BearDogError::configuration(&format!("Invalid bind address: {}", e))
        })?;

    let enable_cors = std::env::var("BEARDOG_ENABLE_CORS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(true);

    let config = BearDogApiServerConfig {
        bind_addr,
        enable_cors,
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    info!("   Bind Address: {}", config.bind_addr);
    info!("   CORS Enabled: {}", config.enable_cors);
    info!("   Version: {}", config.version);
    info!("");

    // Step 6: Create API Server
    info!("📡 Step 5: Creating API Server...");
    let server = BearDogApiServer::new(config.clone(), btsp_provider)
        .await
        .map_err(|e| {
            error!("Failed to create API server: {}", e);
            e
        })?;
    info!("✅ API Server created");
    info!("");

    // Step 7: Start Server in Background
    info!("🚦 Step 6: Starting Server...");
    let server_task = tokio::spawn(async move {
        if let Err(e) = server.serve().await {
            error!("Server error: {}", e);
        }
    });

    info!("╔════════════════════════════════════════════════════════════════════╗");
    info!("║                                                                    ║");
    info!("║         ✅ BearDog Service Ready!                                  ║");
    info!("║                                                                    ║");
    info!("║  HTTP API: http://{}                              ", config.bind_addr);
    info!("║                                                                    ║");
    info!("║  Unix Socket: /tmp/primals/beardog-{{node}}.sock                    ║");
    info!("║                                                                    ║");
    info!("║  Health Check:                                                     ║");
    info!("║    curl http://{}/health                         ", config.bind_addr);
    info!("║                                                                    ║");
    info!("║  Press Ctrl+C or send SIGTERM to shutdown gracefully              ║");
    info!("║                                                                    ║");
    info!("╚════════════════════════════════════════════════════════════════════╝");
    info!("");

    // Step 8: Wait for Shutdown Signal
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("📡 Received SIGINT (Ctrl+C)");
        }
        result = async {
            let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())?;
            sigterm.recv().await;
            Ok::<_, std::io::Error>(())
        } => {
            match result {
                Ok(_) => info!("📡 Received SIGTERM"),
                Err(e) => error!("Error setting up SIGTERM handler: {}", e),
            }
        }
    }

    // Step 9: Graceful Shutdown
    info!("");
    info!("🛑 Shutting down BearDog service...");
    server_task.abort();

    // Wait a moment for graceful shutdown
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    info!("✅ BearDog service stopped");
    info!("");

    Ok(())
}

