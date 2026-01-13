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
//! ## HSM Configuration
//! - `BEARDOG_HSM_MODE` - HSM mode (default: "software")
//!
//! ## Unix Socket Configuration (Primary IPC)
//! - `BEARDOG_SOCKET` - **Explicit socket path** (highest priority)
//! - `BEARDOG_FAMILY_ID` - Family identifier (for auto socket path)
//! - `BEARDOG_NODE_ID` - Node identifier (for auto socket path)
//! - `NODE_ID` - Fallback node identifier
//!
//! ### Socket Path Resolution (3-Tier Fallback)
//! 1. **Explicit**: Use `BEARDOG_SOCKET` if set (e.g., `/run/user/1000/beardog.sock`)
//! 2. **XDG Runtime**: Use `/run/user/<uid>/beardog-<family>.sock` if XDG dir exists
//! 3. **Temp Fallback**: Use `/tmp/beardog-<family>-<node>.sock` as last resort
//!
//! ## HTTP Configuration (Optional, Default: Disabled)
//! - `BEARDOG_HTTP_ENABLED` - Enable HTTP API (default: false)
//! - `BEARDOG_BIND_ADDR` - HTTP bind address (only if HTTP enabled)
//! - `HTTP_PORT` - HTTP port (only if HTTP enabled)
//! - `BEARDOG_ENABLE_CORS` - Enable CORS (default: true)
//!
//! ## Genetic Lineage (Optional)
//! - `BEARDOG_FAMILY_SEED` - Family lineage seed
//!
//! # Usage
//!
//! ```bash
//! # Start with default settings (Unix socket only, XDG-compliant)
//! beardog-server
//! # Creates socket at: /run/user/<uid>/beardog-default.sock (or /tmp fallback)
//!
//! # Specify explicit socket path (highest priority)
//! BEARDOG_SOCKET=/run/user/1000/beardog-nat0.sock beardog-server
//!
//! # Configure socket via family/node IDs
//! BEARDOG_FAMILY_ID=nat0 BEARDOG_NODE_ID=tower1 beardog-server
//! # Creates socket at: /run/user/<uid>/beardog-nat0.sock (or /tmp fallback)
//!
//! # Start with HTTP enabled (optional, dev/debug only)
//! BEARDOG_HTTP_ENABLED=true HTTP_PORT=9000 beardog-server
//!
//! # With family seed (Unix socket only)
//! BEARDOG_FAMILY_SEED="$(cat /media/usb/seed.txt)" beardog-server
//! ```

use beardog_core::socket_config::SocketConfig;
use beardog_errors::BearDogError;
use beardog_genetics::EcosystemGeneticEngine;
#[cfg(feature = "btsp-api")]
use beardog_tunnel::api::{BearDogApiServer, BearDogApiServerConfig};
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::tunnel::hsm::HsmManager;
use beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer;
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
    info!(
        "║         🐻 BearDog Standalone Service v{}                     ║",
        env!("CARGO_PKG_VERSION")
    );
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

    // Step 5: Configure Unix Socket with 3-Tier Fallback
    info!("🔌 Step 4: Configuring Unix Socket IPC...");
    let socket_config = SocketConfig::from_env();

    info!("   Socket Path: {}", socket_config.description());
    info!("   Family ID: {}", socket_config.family_id());
    info!("   Node ID: {}", socket_config.node_id());

    // Prepare socket (create parent dir, remove old socket)
    socket_config.prepare().map_err(|e| {
        error!("Failed to prepare socket: {}", e);
        BearDogError::configuration(&e)
    })?;

    info!("");

    // Step 6: Create Unix Socket IPC Server
    info!("🔌 Step 5: Creating Unix Socket IPC Server...");
    let unix_server = Arc::new(
        UnixSocketIpcServer::new(socket_config.socket_path_string(), btsp_provider.clone())
            .await
            .map_err(|e| {
                error!("Failed to create Unix socket server: {}", e);
                BearDogError::configuration(&format!("Failed to create Unix socket server: {}", e))
            })?,
    );
    info!("✅ Unix Socket IPC Server created");
    info!("");

    // Step 7: Start Unix Socket Server in Background
    info!("🚀 Step 6: Starting Unix Socket Server...");

    // Get readiness flag BEFORE moving server into spawn
    // This is the modern concurrent Rust pattern!
    let ready_flag = unix_server.readiness_flag();

    let unix_server_clone = unix_server.clone();
    let unix_task = tokio::spawn(async move {
        if let Err(e) = unix_server_clone.start().await {
            error!("Unix socket server error: {}", e);
        }
    });

    // Wait for readiness (atomic, lock-free!)
    // No filesystem polling, just pure concurrent Rust!
    info!("   Waiting for socket to be ready (atomic check)...");
    if !UnixSocketIpcServer::wait_ready_flag(&ready_flag, tokio::time::Duration::from_secs(5)).await
    {
        error!("❌ Unix socket server failed to become ready within 5 seconds");
        return Err(BearDogError::configuration("Unix socket server startup timeout").into());
    }

    info!("✅ Unix Socket Server started and ready");
    info!("   ✨ Lock-free concurrent readiness verified!");
    info!("");

    // Step 8: Check if HTTP is enabled (optional)
    let http_enabled = std::env::var("BEARDOG_HTTP_ENABLED")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(false);

    #[cfg(feature = "btsp-api")]
    let http_task = if http_enabled {
        info!("🌐 Step 7: HTTP API Enabled");

        // Determine bind address from environment
        let bind_addr = std::env::var("BEARDOG_BIND_ADDR")
            .or_else(|_| std::env::var("HTTP_PORT").map(|port| format!("0.0.0.0:{}", port)))
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
        info!("");

        info!("📡 Step 8: Creating HTTP API Server...");
        let server = BearDogApiServer::new(config.clone(), btsp_provider.clone())
            .await
            .map_err(|e| {
                error!("Failed to create API server: {}", e);
                e
            })?;
        info!("✅ HTTP API Server created");
        info!("");

        info!("🚦 Step 9: Starting HTTP Server...");
        Some((
            tokio::spawn(async move {
                if let Err(e) = server.serve().await {
                    error!("HTTP Server error: {}", e);
                }
            }),
            config,
        ))
    } else {
        info!("🔌 HTTP API: Disabled (Port-Free Mode)");
        info!("   Set BEARDOG_HTTP_ENABLED=true to enable HTTP");
        info!("   ✅ Zero HTTP ports - Maximum security");
        info!("");
        None
    };

    #[cfg(not(feature = "btsp-api"))]
    let _http_task: Option<(tokio::task::JoinHandle<()>, ())> = {
        if http_enabled {
            warn!("⚠️  HTTP requested but btsp-api feature not enabled");
            warn!("   Rebuild with --features btsp-api to enable HTTP");
            info!("");
        }
        None
    };

    // Display status
    info!("╔════════════════════════════════════════════════════════════════════╗");
    info!("║                                                                    ║");
    info!("║         ✅ BearDog Service Ready!                                  ║");
    info!("║                                                                    ║");
    info!(
        "║  🔌 Unix Socket: {}               ║",
        socket_config.socket_path_string()
    );
    info!("║                                                                    ║");
    #[cfg(feature = "btsp-api")]
    if let Some((_, config)) = &http_task {
        info!(
            "║  🌐 HTTP API: http://{}                              ║",
            config.bind_addr
        );
        info!("║                                                                    ║");
        info!("║  Health Check:                                                     ║");
        info!(
            "║    curl http://{}/health                         ║",
            config.bind_addr
        );
        info!("║                                                                    ║");
    } else {
        info!("║  🔒 Port-Free Mode (HTTP Disabled)                                 ║");
        info!("║                                                                    ║");
    }
    #[cfg(not(feature = "btsp-api"))]
    {
        info!("║  🔒 Port-Free Mode (HTTP Disabled)                                 ║");
        info!("║                                                                    ║");
    }
    info!("║  Press Ctrl+C or send SIGTERM to shutdown gracefully              ║");
    info!("║                                                                    ║");
    info!("╚════════════════════════════════════════════════════════════════════╝");
    info!("");

    // Step 10: Wait for Shutdown Signal
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
        _ = unix_task => {
            error!("Unix socket server task ended unexpectedly");
        }
    }

    // Step 11: Graceful Shutdown
    info!("");
    info!("🛑 Shutting down BearDog service...");

    // Stop Unix socket server gracefully
    if let Err(e) = unix_server.stop().await {
        error!("Error stopping Unix socket server: {}", e);
    } else {
        info!("   Unix socket server stopped gracefully");
    }

    // Stop HTTP server if running
    #[cfg(feature = "btsp-api")]
    if let Some((task, _)) = http_task {
        task.abort();
        info!("   HTTP server stopped");
    }

    // Wait a moment for graceful shutdown
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    info!("✅ BearDog service stopped");
    info!("");

    Ok(())
}
