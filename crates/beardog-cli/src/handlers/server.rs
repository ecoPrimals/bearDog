//! Server mode handler - long-running service for Tower Atomic
//!
//! This handler wires the existing Unix socket IPC server from beardog-tunnel
//! into the CLI for proper UniBin operation.
//!
//! Supports two transport modes:
//! - **Tier 1**: Unix sockets (Linux, macOS) - preferred
//! - **Tier 2**: TCP (Android, Windows, cross-device) - universal fallback

use crate::ServerArgs;
use beardog_errors::BearDogError;
use beardog_genetics::EcosystemGeneticEngine;
use beardog_ipc::{discover_neural_api_socket, register_with_neural_api};
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::multi_transport_server::MultiTransportServer;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use beardog_tunnel::tunnel::hsm::software_hsm::RustSoftwareHsm;
use beardog_tunnel::tunnel::hsm::{HsmTier, SoftwareHsmConfig};
use std::sync::Arc;
use tracing::{info, warn};

/// Handle server command - start long-running service
pub async fn handle_server(args: ServerArgs) -> Result<(), BearDogError> {
    info!("🐻🐕 BearDog Server Mode - Starting...");
    
    // Determine transport mode
    let use_tcp = args.listen.is_some();
    if use_tcp {
        info!("   Transport: TCP (Tier 2 - Universal)");
        info!("   Listen: {}", args.listen.as_ref().unwrap());
    } else {
        info!("   Transport: Unix Socket (Tier 1 - Native)");
        info!("   Socket: {}", args.socket);
    }

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

    // Create primal identity from environment (fail-fast if not configured)
    let identity = Arc::new(
        beardog_types::primal_identity::PrimalIdentity::from_env().map_err(|e| {
            BearDogError::Initialization {
                message: format!("Failed to read primal identity: {}", e),
            }
        })?,
    );
    info!(
        "🆔 Identity: family={}, node={}",
        identity.family_id(),
        identity.node_id()
    );

    // Construct primal name for registration (before identity is moved)
    let primal_name = format!("beardog-{}", identity.node_id());
    let socket_path = args.socket.clone();
    let tcp_addr = args.listen.clone();

    // ================================================================
    // PHASE 3: MULTI-TRANSPORT SERVER (Deep Debt Evolution)
    // ================================================================
    
    info!("🌐 Creating multi-transport server...");
    info!("   Platform: Universal (all available transports)");
    info!("   Deep Debt: Agnostic + Runtime Discovery");
    
    // Create multi-transport server (binds all available)
    let server = MultiTransportServer::bind_all_available(
        btsp_provider,
        identity,
        &socket_path,
        tcp_addr.as_deref(),
    )
    .await?;

    info!("✅ Multi-transport server created: {} transport(s)", server.transport_count());

    // Auto-register with Neural API if available (Tower Atomic TRUE PRIMAL)
    if let Some(neural_socket) = discover_neural_api_socket() {
        info!("🌐 Neural API detected at: {}", neural_socket);
        
        // Register primary socket path
        let registration_addr = if let Some(ref tcp) = tcp_addr {
            tcp.as_str()
        } else {
            &socket_path
        };
        
        match register_with_neural_api(&neural_socket, &primal_name, registration_addr).await {
            Ok(_) => info!("✅ BearDog registered with Neural API (Tower Atomic enabled)"),
            Err(e) => warn!("⚠️  Neural API registration failed (non-fatal): {}", e),
        }
    } else {
        info!("ℹ️  No Neural API detected - running in standalone mode");
    }

    // Start all transports (runs until Ctrl+C)
    server.start_all().await?;

    Ok(())
}

/*
/// OLD IMPLEMENTATION: Single transport mode (DEPRECATED by Phase 3)
///
/// This code remains for reference but is no longer used.
/// Phase 3 evolution: Single → Multi-transport
fn _old_single_transport_server_code() {
    // This function exists purely for documentation - shows evolution path
    //
    // Before Phase 3:
    // - User chose EITHER Unix socket OR TCP
    // - Manual selection required
    // - Not universal
    //
    // After Phase 3:
    // - Server binds ALL available transports
    // - Automatic platform detection
    // - Universal deployment
    
    /*
    if let Some(ref listen_addr) = args.listen {
        // TCP mode (Tier 2 - Android, Windows, universal)
        info!("🌐 Creating TCP IPC server...");
        
        let bind_addr: std::net::SocketAddr = listen_addr.parse().map_err(|e| {
            BearDogError::Initialization {
                message: format!("Invalid listen address '{}': {}", listen_addr, e),
            }
        })?;

        let tcp_server = TcpIpcServer::new(bind_addr, btsp_provider, identity);
        info!("✅ TCP server created");

        // Auto-register with Neural API if available (Tower Atomic TRUE PRIMAL)
        if let Some(neural_socket) = discover_neural_api_socket() {
            info!("🌐 Neural API detected at: {}", neural_socket);
            match register_with_neural_api(&neural_socket, &primal_name, listen_addr).await {
                Ok(_) => info!("✅ BearDog registered with Neural API (Tower Atomic enabled)"),
                Err(e) => warn!("⚠️  Neural API registration failed (non-fatal): {}", e),
            }
        } else {
            info!("ℹ️  No Neural API detected - running in standalone mode");
        }

        // Start TCP server
        info!("🚀 Starting TCP server...");
        info!("");
        info!("╔════════════════════════════════════════════════════════════════╗");
        info!("║                                                                ║");
        info!("║        🐻🐕 BearDog Server READY - TCP Mode (Tier 2)          ║");
        info!("║                                                                ║");
        info!("╚════════════════════════════════════════════════════════════════╝");
        info!("");
        info!("📡 Listening on: {}", listen_addr);
        info!("🔐 Crypto API: Ed25519, X25519, ChaCha20-Poly1305, Blake3");
        info!("🔌 Protocol: JSON-RPC 2.0 over TCP");
        info!("🏗️  Architecture: Tower Atomic (BearDog + Songbird)");
        info!("📱 Platform: Universal (Android, Windows, Linux, macOS)");
        info!("");
        info!("Press Ctrl+C to stop");
        info!("");

        tcp_server.start().await.map_err(|e| BearDogError::System {
            message: format!("TCP server error: {}", e),
            category: Default::default(),
        })?;

        info!("✅ TCP server stopped");
        return Ok(());
    }

    // Unix socket mode (Tier 1 - Native)
    info!("🔌 Creating Unix socket IPC server...");

    let server = Arc::new(
        UnixSocketIpcServer::new(&args.socket, btsp_provider, identity)
            .await
            .map_err(|e| BearDogError::Initialization {
                message: format!("Failed to create server: {}", e),
            })?,
    );
    info!("✅ Server created");

    // Auto-register with Neural API if available (Tower Atomic TRUE PRIMAL)
    if let Some(neural_socket) = discover_neural_api_socket() {
        info!("🌐 Neural API detected at: {}", neural_socket);

        match register_with_neural_api(&neural_socket, &primal_name, &socket_path).await {
            Ok(_) => info!("✅ BearDog registered with Neural API (Tower Atomic enabled)"),
            Err(e) => warn!("⚠️  Neural API registration failed (non-fatal): {}", e),
        }
    } else {
        info!("ℹ️  No Neural API detected - running in standalone mode");
    }

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
    */
}
*/
