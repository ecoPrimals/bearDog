//! Server mode handler - long-running service for Tower Atomic
//!
//! This handler wires the existing Unix socket IPC server from beardog-tunnel
//! into the CLI for proper UniBin operation.

use crate::ServerArgs;
use beardog_errors::BearDogError;
use beardog_genetics::EcosystemGeneticEngine;
use beardog_ipc::{discover_neural_api_socket, register_with_neural_api};
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use beardog_tunnel::tunnel::hsm::software_hsm::RustSoftwareHsm;
use beardog_tunnel::tunnel::hsm::{HsmTier, SoftwareHsmConfig};
use beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer;
use std::sync::Arc;
use tracing::{error, info, warn};

/// Handle server command - start long-running service
pub async fn handle_server(args: ServerArgs) -> Result<(), BearDogError> {
    info!("🐻🐕 BearDog Server Mode - Starting...");
    
    // Determine transport mode
    let transport_mode = if let Some(ref listen_addr) = args.listen {
        info!("   Transport: TCP (universal)");
        info!("   Listen: {}", listen_addr);
        "tcp"
    } else if let Some(ref socket_path) = args.socket {
        info!("   Transport: Unix socket");
        info!("   Socket: {}", socket_path);
        "unix"
    } else {
        // Default to platform-appropriate transport
        if cfg!(target_os = "android") {
            info!("   Transport: TCP (Android default, no SELinux issues)");
            info!("   Listen: 127.0.0.1:0 (OS-assigned port)");
            "tcp_auto"
        } else {
            return Err(BearDogError::Configuration {
                message: "Either --socket or --listen must be specified".to_string(),
                category: Default::default(),
            });
        }
    };

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

    // Construct primal name for registration
    let primal_name = format!("beardog-{}", identity.node_id());

    // Start appropriate server based on transport mode
    match transport_mode {
        "tcp" | "tcp_auto" => {
            // TCP Transport (Android, Windows, universal)
            use beardog_tunnel::tcp_ipc::TcpIpcServer;
            use std::net::SocketAddr;
            
            let listen_addr: SocketAddr = if transport_mode == "tcp_auto" {
                "127.0.0.1:0".parse().unwrap()
            } else {
                args.listen.as_ref().unwrap().parse().map_err(|e| {
                    BearDogError::Configuration {
                        message: format!("Invalid listen address: {}", e),
                        category: Default::default(),
                    }
                })?
            };

            let server = Arc::new(TcpIpcServer::new(listen_addr, btsp_provider, identity));
            
            let server_clone = server.clone();
            tokio::spawn(async move {
                if let Err(e) = server_clone.start().await {
                    error!("TCP server error: {}", e);
                }
            });

            // Wait for server to bind
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let bound_addr = server.get_bound_addr().await.unwrap_or(listen_addr);

            info!("");
            info!("╔════════════════════════════════════════════════════════════════╗");
            info!("║                                                                ║");
            info!("║        🐻🐕 BearDog Server READY - Tower Atomic Enabled       ║");
            info!("║                                                                ║");
            info!("╚════════════════════════════════════════════════════════════════╝");
            info!("");
            info!("📡 Listening on: {} (TCP)", bound_addr);
            info!("🔐 Crypto API: Ed25519, X25519, ChaCha20-Poly1305, Blake3");
            info!("🔌 Protocol: JSON-RPC 2.0 over TCP");
            info!("🏗️  Architecture: Tower Atomic (BearDog + Songbird)");
            info!("🌐 Mode: Universal (Android-compatible)");
            info!("");
            info!("Press Ctrl+C to stop");
            info!("");

            tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
            info!("");
            info!("🛑 Received shutdown signal");
            info!("Goodbye! 🐻");
            Ok(())
        }
        "unix" => {
            // Unix Socket Mode (Linux preferred)
            let socket_path = args.socket.as_ref().unwrap().clone();
            
            let server = Arc::new(
                UnixSocketIpcServer::new(&socket_path, btsp_provider, identity)
                    .await
                    .map_err(|e| BearDogError::Initialization {
                        message: format!("Failed to create server: {}", e),
                    })?,
            );
            info!("✅ Server created");

            // Auto-register with Neural API
            if let Some(neural_socket) = discover_neural_api_socket() {
                info!("🌐 Neural API detected at: {}", neural_socket);
                match register_with_neural_api(&neural_socket, &primal_name, &socket_path).await {
                    Ok(_) => info!("✅ BearDog registered with Neural API"),
                    Err(e) => warn!("⚠️  Neural API registration failed: {}", e),
                }
            } else {
                info!("ℹ️  No Neural API detected - standalone mode");
            }

            info!("🚀 Starting server...");
            info!("");
            info!("╔════════════════════════════════════════════════════════════════╗");
            info!("║                                                                ║");
            info!("║        🐻🐕 BearDog Server READY - Tower Atomic Enabled       ║");
            info!("║                                                                ║");
            info!("╚════════════════════════════════════════════════════════════════╝");
            info!("");
            info!("📡 Listening on: {}", socket_path);
            info!("🔐 Crypto API: Ed25519, X25519, ChaCha20-Poly1305, Blake3");
            info!("🔌 Protocol: JSON-RPC 2.0 over Unix sockets");
            info!("🏗️  Architecture: Tower Atomic (BearDog + Songbird)");
            info!("");
            info!("Press Ctrl+C to stop");
            info!("");

            let server_clone = Arc::clone(&server);
            tokio::spawn(async move {
                tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
                info!("");
                info!("🛑 Received Ctrl+C, shutting down...");
                if let Err(e) = server_clone.stop().await {
                    error!("❌ Error during shutdown: {}", e);
                }
                std::process::exit(0);
            });

            server.start().await.map_err(|e| BearDogError::System {
                message: format!("Server error: {}", e),
                category: Default::default(),
            })?;

            info!("✅ Server stopped");
            Ok(())
        }
        _ => Err(BearDogError::system(format!("Unknown transport mode: {}", transport_mode))),
    }
}
