//! BearDog Server - Modern Idiomatic Rust
//!
//! Zero hardcoding, capability-based, primal-sovereign architecture.
//! 
//! This server provides:
//! - Unix socket IPC (primary inter-primal communication)
//! - HTTP API (optional, for external access)
//! - BTSP tunneling (VPN-free P2P mesh)
//! - Genetic lineage-based trust
//! - Hardware-backed security (HSM)

use anyhow::{Context, Result};
use beardog_genetics::ecosystem_evolution::EcosystemGeneticEngine;
use beardog_tunnel::{
    btsp_provider::BeardogBtspProvider,
    tunnel::hsm::manager::HsmManager,
    unix_socket_ipc::UnixSocketIpcServer,
};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::signal;
use tracing::{error, info, warn, Level};
use tracing_subscriber::FmtSubscriber;

/// Server configuration from environment
#[derive(Debug, Clone)]
struct ServerConfig {
    /// Family ID (genetic lineage)
    family_id: String,
    /// Node ID (unique identifier)
    node_id: String,
    /// Unix socket path
    socket_path: PathBuf,
    /// Optional HTTP port (0 = disabled for port-free operation)
    http_port: u16,
    /// Log level
    log_level: Level,
}

impl ServerConfig {
    /// Load configuration from environment (zero hardcoding)
    fn from_env() -> Result<Self> {
        // Support both FAMILY_ID and BEARDOG_FAMILY_ID for compatibility
        let family_id = std::env::var("FAMILY_ID")
            .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| {
                warn!("No FAMILY_ID set, using 'unknown'");
                "unknown".to_string()
            });

        let node_id = std::env::var("NODE_ID")
            .or_else(|_| std::env::var("BEARDOG_NODE_ID"))
            .or_else(|_| std::env::var("HOSTNAME"))
            .or_else(|_| {
                hostname::get()
                    .ok()
                    .and_then(|h| h.into_string().ok())
                    .ok_or_else(|| std::env::VarError::NotPresent)
            })
            .unwrap_or_else(|_| {
                warn!("No NODE_ID set, using 'node-unknown'");
                "node-unknown".to_string()
            });

        let socket_path = std::env::var("BEARDOG_SOCKET_PATH")
            .unwrap_or_else(|_| format!("/tmp/primals/beardog-{}.sock", node_id))
            .into();

        let http_port = std::env::var("BEARDOG_HTTP_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(0); // Default: 0 = disabled (port-free!)

        let log_level = std::env::var("BEARDOG_LOG_LEVEL")
            .ok()
            .and_then(|l| l.parse().ok())
            .unwrap_or(Level::INFO);

        Ok(Self {
            family_id,
            node_id,
            socket_path,
            http_port,
            log_level,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from environment
    let config = ServerConfig::from_env()
        .context("Failed to load server configuration")?;

    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(config.log_level)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set tracing subscriber")?;

    info!("🐻 BearDog Server Starting");
    info!("   Family: {}", config.family_id);
    info!("   Node: {}", config.node_id);
    info!("   Socket: {}", config.socket_path.display());
    info!("   HTTP Port: {} (0 = disabled for port-free operation)", config.http_port);

    // Initialize HSM manager
    info!("🔐 Initializing HSM...");
    let hsm = Arc::new(HsmManager::new());

    // Initialize genetic engine
    info!("🧬 Initializing genetic lineage engine...");
    let genetics = Arc::new(
        EcosystemGeneticEngine::new()
            .context("Failed to initialize genetic engine")?
    );

    // Initialize BTSP provider
    info!("🔒 Initializing BTSP provider...");
    let btsp_provider = Arc::new(
        BeardogBtspProvider::new(hsm.clone(), genetics.clone())
            .await
            .context("Failed to initialize BTSP provider")?
    );

    // Create Unix socket IPC server
    info!("🔌 Creating Unix socket IPC server...");
    let unix_server = Arc::new(
        UnixSocketIpcServer::new(&config.socket_path, btsp_provider.clone())
            .await
            .context("Failed to create Unix socket IPC server")?
    );

    info!("✅ BearDog Server initialized successfully");
    info!("");
    info!("📊 Capabilities:");
    info!("   • Unix Socket IPC: {}", config.socket_path.display());
    info!("   • BTSP Tunneling: Enabled (6 methods)");
    info!("   • Genetic Lineage: {}", config.family_id);
    info!("   • Security Provider: Hardware-backed (HSM)");
    info!("   • Port-Free P2P: {} (HTTP port {})", 
        if config.http_port == 0 { "Enabled ✅" } else { "Disabled ⚠️" },
        config.http_port
    );
    info!("");

    // Start Unix socket server
    let server_handle = {
        let unix_server = Arc::clone(&unix_server);
        tokio::spawn(async move {
            if let Err(e) = unix_server.start().await {
                error!("❌ Unix socket server error: {}", e);
            }
        })
    };

    info!("🚀 BearDog Server running - Press Ctrl+C to shutdown");

    // Wait for shutdown signal
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("🛑 Shutdown signal received");
        }
        Err(e) => {
            error!("Failed to listen for shutdown signal: {}", e);
        }
    }

    // Cleanup
    info!("🧹 Shutting down gracefully...");
    server_handle.abort();

    info!("👋 BearDog Server stopped");
    Ok(())
}

