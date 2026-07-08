// SPDX-License-Identifier: AGPL-3.0-or-later

//! Server Mode - Primary `BearDog` operational mode.
//!
//! Modern async/concurrent Rust architecture with clean error handling.

mod registration;

pub use registration::NeuralRegistrationParams;
use registration::{
    family_id_preview_from_seed, register_with_discovery_service,
    spawn_ipc_registry_registration_task,
};

use crate::btsp_provider::BeardogBtspProvider;
use crate::tunnel::hsm::HsmManager;
use crate::tunnel::hsm::manager::HsmAutoInitConfig;
use crate::unix_socket_ipc::UnixSocketIpcServer;
use beardog_config::env_keys;
use beardog_core::self_knowledge::PrimalSelfKnowledge;
use beardog_core::socket_config::{IpcCapabilitySymlinksConfig, SocketConfig};
use beardog_errors::BearDogError;
use beardog_genetics::EcosystemGeneticEngine;
use std::sync::Arc;
use tokio::signal;
use tracing::{error, info, warn};

/// Run `BearDog` in server mode.
///
/// Modern async architecture with:
/// - Clean separation of concerns
/// - Structured error handling
/// - Graceful shutdown
/// - Lock-free concurrency
///
/// # Errors
///
/// Returns an error if self-knowledge discovery, HSM initialization, BTSP setup, socket binding,
/// or server startup fails.
pub async fn run(
    socket: Option<String>,
    daemon: bool,
    family_id: Option<String>,
    orchestrator_id: Option<String>,
    http_enabled: bool,
    _bind_addr: Option<String>,
) -> anyhow::Result<()> {
    if let Some(socket_path) = &socket {
        beardog_errors::process_env::set_var(env_keys::ENV_SOCKET, socket_path);
    }
    if let Some(fam_id) = &family_id {
        beardog_errors::process_env::set_var(env_keys::ENV_FAMILY_ID_PREFIXED, fam_id);
    }
    if let Some(orch_id) = &orchestrator_id {
        beardog_errors::process_env::set_var(env_keys::ENV_ORCHESTRATOR_ID, orch_id);
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
    let hsm_init = HsmAutoInitConfig::from_env();
    info!("   HSM Mode: {}", hsm_init.mode);

    let hsm = Arc::new(
        HsmManager::auto_initialize_with_config(hsm_init)
            .await
            .map_err(|e| {
                error!("Failed to initialize HSM: {}", e);
                e
            })?,
    );
    info!("✅ HSM Manager initialized successfully\n");

    // Step 2: Initialize Genetic Engine
    info!("🧬 Initializing Genetic Engine...");
    let genetics = Arc::new(EcosystemGeneticEngine::new().map_err(|e| {
        error!("Failed to initialize genetic engine: {}", e);
        e
    })?);
    info!("✅ Genetic Engine initialized\n");

    // Step 3: Load Family Seed (if provided)
    if let Ok(family_seed) = beardog_errors::process_env::var(env_keys::ENV_FAMILY_SEED_PREFIXED) {
        info!("👨‍👩‍👧‍👦 Family lineage seed detected");
        let family_id = family_id_preview_from_seed(&family_seed);
        info!("   Family ID: {}", family_id);
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
    let socket_config = SocketConfig::from_env().map_err(|e| {
        error!("{e}");
        BearDogError::configuration(&e.to_string())
    })?;

    info!("   Socket: {}", socket_config.socket_path().display());
    info!("   Source: {}", socket_config.description());
    info!("   Family: {}", socket_config.family_id());
    info!("   Node: {}", socket_config.node_id());
    info!("   PID: {}", std::process::id());

    socket_config.prepare().inspect_err(|e| {
        error!("Failed to prepare socket: {}", e);
    })?;

    // wateringHole v3.1: capability-domain symlinks are installed after bind in [`UnixSocketIpcServer::start`].
    let ipc_symlinks = IpcCapabilitySymlinksConfig::from_socket_config_and_capabilities(
        &socket_config,
        self_knowledge.my_capabilities(),
    );
    let ipc_symlink_stems = ipc_symlinks.domain_stems.clone();

    info!(
        "   Production mode: {}",
        if socket_config.production_mode() {
            "yes (BTSP handshake required)"
        } else {
            "no (development, cleartext JSON-RPC)"
        }
    );

    // Step 6: Create Unix Socket IPC Server
    info!("\n🔌 Creating Unix Socket IPC Server...");

    let identity = Arc::new(beardog_types::primal_identity::PrimalIdentity::from_env());
    if identity.is_standalone() {
        info!("🆔 Running in standalone mode (no identity env vars set)");
    }
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
            crate::btsp_handshake::resolve_security_mode()
                .unwrap_or(crate::btsp_handshake::BtspSecurityMode::Development),
            ipc_symlinks,
        )
        .await
        .map_err(|e| {
            error!("Failed to create Unix socket server: {}", e);
            BearDogError::configuration(&format!("Failed to create Unix socket server: {e}"))
        })?,
    );
    info!("✅ Unix Socket IPC Server created\n");

    // Step 7: Start Unix Socket Server
    info!("🚀 Starting Unix Socket Server...");

    let ready_flag = unix_server.readiness_flag();

    let unix_server_clone = unix_server.clone();
    let unix_task = tokio::spawn(async move {
        if let Err(e) = unix_server_clone.start().await {
            error!("Unix socket server error: {}", e);
        }
    });

    info!("   Waiting for socket readiness (atomic check)...");
    if !UnixSocketIpcServer::wait_ready_flag(&ready_flag, tokio::time::Duration::from_secs(5)).await
    {
        error!("❌ Unix socket server failed to become ready within 5 seconds");
        return Err(BearDogError::configuration("Unix socket server startup timeout").into());
    }

    info!("✅ Unix Socket Server started and ready");
    info!("   ✨ Lock-free concurrent readiness verified!\n");

    // Step 7.4: IPC registry (`ipc.register` / `ipc.heartbeat`) — non-blocking, retries with backoff
    spawn_ipc_registry_registration_task(
        self_knowledge.clone(),
        socket_config.socket_path_string(),
    );

    // Step 7.5: Register with discovery (Neural API — capability-oriented routing)
    info!("🌐 Registering with discovery service...");
    let neural_registration = NeuralRegistrationParams::from_env();
    match register_with_discovery_service(
        &socket_config,
        &neural_registration,
        unix_server.handler_registry(),
    )
    .await {
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
        warn!("   See: ARCHITECTURE.md for current IPC transport guidance\n");
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
    info!("   • Fully memory-safe");
    info!("\n🎯 Press Ctrl+C to shutdown gracefully...\n");

    // Step 10: Wait for shutdown signal
    wait_for_shutdown().await;

    info!("\n🛑 Shutdown signal received, cleaning up...");

    socket_config.remove_ipc_capability_symlinks(&ipc_symlink_stems);

    drop(unix_task);
    drop(unix_server);

    info!("✅ Cleanup complete");
    info!("👋 BearDog server stopped gracefully\n");

    Ok(())
}

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

async fn wait_for_shutdown() {
    let ctrl_c = async {
        if let Err(e) = signal::ctrl_c().await {
            error!("Failed to install Ctrl+C handler: {e}");
            std::future::pending::<()>().await;
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match signal::unix::signal(signal::unix::SignalKind::terminate()) {
            Ok(mut sig) => {
                sig.recv().await;
            }
            Err(e) => {
                error!("Failed to install SIGTERM handler: {e}");
                std::future::pending::<()>().await;
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {
            info!("Received Ctrl+C");
        },
        () = terminate => {
            info!("Received SIGTERM");
        },
    }
}

#[cfg(test)]
mod banner_tests {
    use super::display_banner;
    use beardog_core::self_knowledge::PrimalSelfKnowledge;

    #[test]
    fn display_banner_smoke_with_discovered_self_knowledge() {
        let sk = PrimalSelfKnowledge::discover().expect("discover self-knowledge");
        display_banner(&sk, false);
        display_banner(&sk, true);
    }
}
