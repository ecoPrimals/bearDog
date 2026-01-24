//! Server mode handler - long-running service for Tower Atomic
//!
//! This handler wires the existing Unix socket IPC server from beardog-tunnel
//! into the CLI for proper UniBin operation.

use crate::ServerArgs;
use beardog_errors::BearDogError;
use beardog_genetics::EcosystemGeneticEngine;
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use beardog_tunnel::tunnel::hsm::software_hsm::RustSoftwareHsm;
use beardog_tunnel::tunnel::hsm::{HsmTier, SoftwareHsmConfig};
use beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer;
use std::sync::Arc;
use tracing::{error, info};

/// Handle server command - start long-running service
pub async fn handle_server(args: ServerArgs) -> Result<(), BearDogError> {
    info!("🐻🐕 BearDog Server Mode - Starting...");
    info!("   Socket: {}", args.socket);

    if let Some(ref family_id) = args.family_id {
        info!("   Family ID: {}", family_id);
    }

    if let Some(ref orchestrator_id) = args.orchestrator_id {
        info!("   Orchestrator ID: {}", orchestrator_id);
    }

    // Create HSM manager with software provider
    info!("🔧 Initializing HSM manager...");
    let mut hsm = HsmManager::new();
    let config = SoftwareHsmConfig::default();
    let software_hsm =
        RustSoftwareHsm::new(config)
            .await
            .map_err(|e| BearDogError::Initialization {
                message: format!("Failed to create software HSM: {}", e),
            })?;

    hsm.register_hsm_provider(HsmTier::Software, Arc::new(software_hsm))
        .map_err(|e| BearDogError::Initialization {
            message: format!("Failed to register HSM provider: {}", e),
        })?;
    let hsm = Arc::new(hsm);
    info!("✅ HSM manager initialized");

    // Create genetics engine
    info!("🔧 Initializing genetics engine...");
    let genetics =
        Arc::new(
            EcosystemGeneticEngine::new().map_err(|e| BearDogError::Initialization {
                message: format!("Failed to create genetics engine: {}", e),
            })?,
        );
    info!("✅ Genetics engine initialized");

    // Create BTSP provider (provides all capabilities)
    info!("🔧 Initializing BTSP provider...");
    let btsp_provider = Arc::new(BeardogBtspProvider::new(hsm, genetics).await.map_err(|e| {
        BearDogError::Initialization {
            message: format!("Failed to create BTSP provider: {}", e),
        }
    })?);
    info!("✅ BTSP provider initialized");

    // Create Unix socket IPC server
    info!("🔌 Creating Unix socket IPC server...");
    let server = Arc::new(
        UnixSocketIpcServer::new(&args.socket, btsp_provider)
            .await
            .map_err(|e| BearDogError::Initialization {
                message: format!("Failed to create server: {}", e),
            })?,
    );
    info!("✅ Server created");

    // Start server (this blocks until shutdown)
    info!("🚀 Starting server...");
    info!("");
    info!("╔════════════════════════════════════════════════════════════════╗");
    info!("║                                                                ║");
    info!("║        🐻🐕 BearDog Server READY - Tower Atomic Enabled       ║");
    info!("║                                                                ║");
    info!("╚════════════════════════════════════════════════════════════════╝");
    info!("");
    info!("📡 Listening on: {}", args.socket);
    info!("🔐 Crypto API: Ed25519, X25519, ChaCha20-Poly1305, Blake3");
    info!("🔌 Protocol: JSON-RPC 2.0 over Unix sockets");
    info!("🏗️  Architecture: Tower Atomic (BearDog + Songbird)");
    info!("");
    info!("Press Ctrl+C to stop");
    info!("");

    // Handle Ctrl+C gracefully
    let server_clone: Arc<UnixSocketIpcServer> = Arc::clone(&server);
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");
        info!("");
        info!("🛑 Received Ctrl+C, shutting down gracefully...");
        if let Err(e) = server_clone.stop().await {
            error!("❌ Error during shutdown: {}", e);
        }
        std::process::exit(0);
    });

    // Start server (blocks until stopped)
    server.start().await.map_err(|e| BearDogError::System {
        message: format!("Server error: {}", e),
        category: Default::default(),
    })?;

    info!("✅ Server stopped");
    Ok(())
}
