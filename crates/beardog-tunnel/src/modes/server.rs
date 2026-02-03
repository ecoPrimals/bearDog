//! Server Mode - Primary BearDog operational mode
//!
//! Modern async/concurrent Rust architecture with clean error handling.

use crate::btsp_provider::BeardogBtspProvider;
use crate::tunnel::hsm::HsmManager;
use crate::unix_socket_ipc::UnixSocketIpcServer;
use beardog_core::self_knowledge::PrimalSelfKnowledge;
use beardog_core::socket_config::SocketConfig;
use beardog_errors::BearDogError;
use beardog_genetics::EcosystemGeneticEngine;
use std::sync::Arc;
use tokio::signal;
use tracing::{debug, error, info, warn};

/// Run BearDog in server mode
///
/// Modern async architecture with:
/// - Clean separation of concerns
/// - Structured error handling
/// - Graceful shutdown
/// - Lock-free concurrency
pub async fn run(
    socket: Option<String>,
    daemon: bool,
    family_id: Option<String>,
    orchestrator_id: Option<String>,
    http_enabled: bool,
    _bind_addr: Option<String>,
) -> anyhow::Result<()> {
    // Override environment with CLI args if provided
    if let Some(socket_path) = &socket {
        std::env::set_var("BEARDOG_SOCKET", socket_path);
    }
    if let Some(fam_id) = &family_id {
        std::env::set_var("BEARDOG_FAMILY_ID", fam_id);
    }
    if let Some(orch_id) = &orchestrator_id {
        std::env::set_var("BEARDOG_ORCHESTRATOR_ID", orch_id);
    }

    // Step 0: Discover Self-Knowledge (Zero Hardcoded Identity)
    info!("🔍 Discovering self-knowledge from environment...");
    let self_knowledge = PrimalSelfKnowledge::discover().map_err(|e| {
        error!("Failed to discover self-knowledge: {}", e);
        e
    })?;

    display_banner(&self_knowledge, daemon);

    // Step 1: Initialize HSM Manager
    info!("🔐 Initializing HSM Manager...");
    let hsm_mode = std::env::var("BEARDOG_HSM_MODE").unwrap_or_else(|_| "software".to_string());
    info!("   HSM Mode: {}", hsm_mode);

    let hsm = Arc::new(HsmManager::auto_initialize().await.map_err(|e| {
        error!("Failed to initialize HSM: {}", e);
        e
    })?);
    info!("✅ HSM Manager initialized successfully\n");

    // Step 2: Initialize Genetic Engine
    info!("🧬 Initializing Genetic Engine...");
    let genetics = Arc::new(EcosystemGeneticEngine::new().map_err(|e| {
        error!("Failed to initialize genetic engine: {}", e);
        e
    })?);
    info!("✅ Genetic Engine initialized\n");

    // Step 3: Load Family Seed (if provided)
    if let Ok(family_seed) = std::env::var("BEARDOG_FAMILY_SEED") {
        info!("👨‍👩‍👧‍👦 Family lineage seed detected");
        let family_id: String = family_seed
            .chars()
            .filter(|c| c.is_alphanumeric())
            .take(4)
            .collect();
        info!("   Family ID: {}", family_id.to_lowercase());
        info!("   Genetic siblings will auto-trust this family\n");
    }

    // Step 4: Create BTSP Provider
    info!("🛡️  Creating BTSP Provider...");
    let btsp_provider = Arc::new(BeardogBtspProvider::new(hsm, genetics).await.map_err(|e| {
        error!("Failed to create BTSP provider: {}", e);
        e
    })?);
    info!("✅ BTSP Provider created\n");

    // Step 5: Configure Unix Socket
    info!("🔌 Configuring Unix Socket IPC...");
    let socket_config = SocketConfig::from_env();

    info!("   Socket: {}", socket_config.socket_path().display());
    info!("   Source: {}", socket_config.description());
    info!("   Family: {}", socket_config.family_id());
    info!("   Node: {}", socket_config.node_id());
    info!("   PID: {}", std::process::id());

    // Prepare socket (create parent dir, remove old socket)
    socket_config.prepare().map_err(|e| {
        error!("Failed to prepare socket: {}", e);
        BearDogError::configuration(&e)
    })?;

    // Step 6: Create Unix Socket IPC Server
    info!("\n🔌 Creating Unix Socket IPC Server...");

    // Create primal identity from environment (fail-fast if not configured)
    let identity = Arc::new(
        beardog_types::primal_identity::PrimalIdentity::from_env().map_err(|e| {
            error!("Failed to read primal identity: {}", e);
            BearDogError::configuration(&e.to_string())
        })?,
    );
    info!(
        "🆔 Identity: family={}, node={}",
        identity.family_id(),
        identity.node_id()
    );

    let unix_server = Arc::new(
        UnixSocketIpcServer::new(
            socket_config.socket_path_string(),
            btsp_provider.clone(),
            identity,
        )
        .await
        .map_err(|e| {
            error!("Failed to create Unix socket server: {}", e);
            BearDogError::configuration(&format!("Failed to create Unix socket server: {}", e))
        })?,
    );
    info!("✅ Unix Socket IPC Server created\n");

    // Step 7: Start Unix Socket Server
    info!("🚀 Starting Unix Socket Server...");

    // Get readiness flag BEFORE moving server into spawn
    // Modern concurrent Rust pattern - lock-free atomics!
    let ready_flag = unix_server.readiness_flag();

    let unix_server_clone = unix_server.clone();
    let unix_task = tokio::spawn(async move {
        if let Err(e) = unix_server_clone.start().await {
            error!("Unix socket server error: {}", e);
        }
    });

    // Wait for readiness (atomic, lock-free!)
    info!("   Waiting for socket readiness (atomic check)...");
    if !UnixSocketIpcServer::wait_ready_flag(&ready_flag, tokio::time::Duration::from_secs(5)).await
    {
        error!("❌ Unix socket server failed to become ready within 5 seconds");
        return Err(BearDogError::configuration("Unix socket server startup timeout").into());
    }

    info!("✅ Unix Socket Server started and ready");
    info!("   ✨ Lock-free concurrent readiness verified!\n");

    // Step 7.5: Register with Discovery Service (Neural API or fallback to Songbird)
    // EVOLUTION: Use Neural API for TRUE PRIMAL pattern, fallback to legacy Songbird
    info!("🌐 Registering with Discovery Service...");
    match register_with_discovery_service(&socket_config).await {
        Ok(()) => {
            info!("✅ Successfully registered with discovery service");
            info!("   Other primals can now discover BearDog via capabilities\n");
        }
        Err(e) => {
            warn!("⚠️  Discovery registration skipped (non-fatal): {}", e);
            warn!("   BearDog will run without discovery service integration");
            warn!("   This is OK for standalone operation or development\n");
        }
    }

    // Step 8: HTTP API (deprecated, warn if enabled)
    if http_enabled {
        warn!("⚠️  HTTP API is deprecated!");
        warn!("   Use Unix socket JSON-RPC instead (ecosystem standard)");
        warn!("   See: docs/sessions/jan_16_2026/BTSP_EVOLUTION_COMPLETE_JAN_16_2026.md\n");
    }

    // Step 9: Display Ready Status
    info!("╔════════════════════════════════════════════════════════════════════╗");
    info!("║                                                                    ║");
    info!("║                  🎉 BearDog Server Ready! 🎉                      ║");
    info!("║                                                                    ║");
    info!("╚════════════════════════════════════════════════════════════════════╝\n");
    info!("📡 Endpoints:");
    info!("   • Unix Socket: {}", socket_config.socket_path_string());
    info!("   • Protocol: JSON-RPC 2.0");
    info!("   • Transport: Unix domain sockets");
    info!("\n🛡️  BTSP Capabilities:");
    info!("   • contact_exchange");
    info!("   • tunnel_establish");
    info!("   • tunnel_encrypt / tunnel_decrypt");
    info!("   • tunnel_status / tunnel_close");
    info!("\n⚡ Architecture:");
    info!("   • 100% Pure Rust");
    info!("   • Modern async/await (tokio)");
    info!("   • Lock-free atomics (parking_lot)");
    info!("   • Zero unsafe code");
    info!("\n🎯 Press Ctrl+C to shutdown gracefully...\n");

    // Step 10: Wait for shutdown signal
    wait_for_shutdown().await;

    info!("\n🛑 Shutdown signal received, cleaning up...");

    // Cleanup (drop handles, servers shutdown automatically)
    drop(unix_task);
    drop(unix_server);

    info!("✅ Cleanup complete");
    info!("👋 BearDog server stopped gracefully\n");

    Ok(())
}

/// Register BearDog with discovery service
///
/// Implements TRUE PRIMAL pattern via Neural API (capability.call).
/// Falls back to legacy Songbird registration if Neural API unavailable.
///
/// # Evolution Strategy
///
/// 1. **Primary**: Neural API (TRUE PRIMAL pattern)
///    - Auto-registration via `neural_registration` module
///    - Capability-based semantic routing
///    - Zero coupling between primals
///
/// 2. **Fallback**: Legacy Songbird client
///    - Direct Songbird registration (deprecated)
///    - Will be removed after full Neural API adoption
///
/// # Returns
///
/// Ok(()) if registered successfully with any service, Err if all methods fail.
/// Non-fatal - BearDog can operate standalone without discovery.
async fn register_with_discovery_service(socket_config: &SocketConfig) -> anyhow::Result<()> {
    use anyhow::Context;
    use beardog_ipc::{discover_neural_api_socket, register_with_neural_api};
    use beardog_types::primal_identity::PrimalIdentity;

    // PHASE 1: Try Neural API (TRUE PRIMAL pattern)
    if let Some(neural_socket) = discover_neural_api_socket() {
        info!("🌐 Neural API detected at: {}", neural_socket);

        // Get primal identity from environment
        let identity = PrimalIdentity::from_env()
            .context("Failed to load primal identity for registration")?;

        // Construct primal name and socket path
        let primal_name = format!("beardog-{}", identity.node_id());
        let socket_path = socket_config.socket_path_string();

        match register_with_neural_api(&neural_socket, &primal_name, &socket_path).await {
            Ok(_) => {
                info!("✅ Registered with Neural API (TRUE PRIMAL)");
                return Ok(());
            }
            Err(e) => {
                warn!("⚠️  Neural API registration failed: {}", e);
                // Fall through to legacy registration
            }
        }
    } else {
        debug!("ℹ️  Neural API not detected, trying legacy Songbird...");
    }

    // PHASE 2: Fallback to legacy Songbird (DEPRECATED)
    // This will be removed after full Neural API adoption
    // Intentional use of deprecated function for backward compatibility
    #[allow(deprecated)]
    match register_with_legacy_songbird().await {
        Ok(_) => {
            info!("✅ Registered with Songbird (legacy, will migrate to Neural API)");
            Ok(())
        }
        Err(e) => {
            warn!(
                "⚠️  No discovery service available (Neural API or Songbird): {}",
                e
            );
            Err(e)
        }
    }
}

/// Legacy Songbird registration (DEPRECATED)
///
/// This function will be removed after full Neural API adoption.
/// Use `register_with_neural_api` for TRUE PRIMAL pattern.
#[deprecated(
    since = "0.9.1",
    note = "Use Neural API registration for TRUE PRIMAL pattern"
)]
async fn register_with_legacy_songbird() -> anyhow::Result<()> {
    use beardog_ipc::{Capability, SongbirdClient};
    use std::time::Duration;

    // Attempt to connect to Songbird
    // This may fail if Songbird isn't running - that's OK for standalone operation
    let client = SongbirdClient::connect().await?;

    // Register BearDog with its capabilities
    // These should match what PrimalSelfKnowledge reports
    let capabilities = vec![
        Capability::Crypto,
        Capability::BTSP,
        Capability::Ed25519,
        Capability::X25519,
        Capability::AesGcm,
        Capability::ChaCha20Poly1305,
    ];

    client.register("beardog", capabilities).await?;

    // Start heartbeat to maintain registration
    // Songbird expects heartbeats every 30-60 seconds
    let heartbeat_interval = Duration::from_secs(30);
    tokio::spawn(async move {
        let _heartbeat = client.start_heartbeat(heartbeat_interval);
        // Heartbeat task runs until client is dropped
        // This keeps BearDog registered with Songbird
        std::future::pending::<()>().await;
    });

    Ok(())
}

/// Display startup banner
fn display_banner(self_knowledge: &PrimalSelfKnowledge, daemon: bool) {
    info!("╔════════════════════════════════════════════════════════════════════╗");
    info!("║                                                                    ║");
    info!(
        "║         🐻 {} v{}                                        ║",
        self_knowledge.my_name(),
        self_knowledge.my_version().version
    );
    info!("║                                                                    ║");
    info!("║              Security & Cryptography Primal                       ║");
    info!("║                                                                    ║");
    info!("╚════════════════════════════════════════════════════════════════════╝\n");

    info!("🎯 Self-Knowledge:");
    info!("   Name: {}", self_knowledge.my_name());
    info!("   Version: {}", self_knowledge.my_version().version);
    if let Some(git_hash) = &self_knowledge.my_version().git_hash {
        info!("   Git Hash: {}", git_hash);
    }
    info!("   Mode: server (daemon: {})", daemon);
    info!(
        "   Capabilities: {} discovered",
        self_knowledge.my_capabilities().len()
    );
    for cap in self_knowledge.my_capabilities() {
        info!("      • {:?}", cap);
    }
    info!("");
}

/// Wait for shutdown signal (Ctrl+C or SIGTERM)
///
/// Modern async pattern with graceful shutdown.
async fn wait_for_shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C");
        },
        _ = terminate => {
            info!("Received SIGTERM");
        },
    }
}
