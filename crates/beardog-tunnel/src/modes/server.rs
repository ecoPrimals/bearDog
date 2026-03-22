// SPDX-License-Identifier: AGPL-3.0-only

//! Server Mode - Primary BearDog operational mode
//!
//! Modern async/concurrent Rust architecture with clean error handling.

use crate::btsp_provider::BeardogBtspProvider;
use crate::tunnel::hsm::HsmManager;
use crate::tunnel::hsm::manager::HsmAutoInitConfig;
use crate::unix_socket_ipc::UnixSocketIpcServer;
use beardog_core::self_knowledge::PrimalSelfKnowledge;
use beardog_core::socket_config::SocketConfig;
use beardog_errors::BearDogError;
use beardog_genetics::EcosystemGeneticEngine;
use std::sync::Arc;
use tokio::signal;
use tracing::{debug, error, info, warn};

/// Neural API registration fields (inject in tests; use [`Self::from_env`] at process startup).
#[derive(Debug, Clone, Default)]
pub struct NeuralRegistrationParams {
    /// `BEARDOG_NEURAL_REGISTRATION_INSTANCE`
    pub instance_override: Option<String>,
    /// `PRIMAL_TYPE`
    pub primal_type: Option<String>,
    /// `BEARDOG_PRIMAL_TYPE`
    pub beardog_primal_type: Option<String>,
}

impl NeuralRegistrationParams {
    /// Read `BEARDOG_NEURAL_REGISTRATION_INSTANCE`, `PRIMAL_TYPE`, `BEARDOG_PRIMAL_TYPE`.
    pub fn from_env() -> Self {
        Self {
            instance_override: beardog_errors::process_env::var(
                "BEARDOG_NEURAL_REGISTRATION_INSTANCE",
            )
            .ok(),
            primal_type: beardog_errors::process_env::var("PRIMAL_TYPE").ok(),
            beardog_primal_type: beardog_errors::process_env::var("BEARDOG_PRIMAL_TYPE").ok(),
        }
    }

    /// Registry instance id (capability-oriented; not a fixed product name).
    pub fn registration_instance_id(
        &self,
        identity: &beardog_types::primal_identity::PrimalIdentity,
    ) -> String {
        self.instance_override.clone().unwrap_or_else(|| {
            let role = self
                .primal_type
                .clone()
                .or_else(|| self.beardog_primal_type.clone())
                .unwrap_or_else(|| "security".to_string());
            format!("{role}-{}", identity.node_id())
        })
    }
}

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
        beardog_errors::process_env::set_var("BEARDOG_SOCKET", socket_path);
    }
    if let Some(fam_id) = &family_id {
        beardog_errors::process_env::set_var("BEARDOG_FAMILY_ID", fam_id);
    }
    if let Some(orch_id) = &orchestrator_id {
        beardog_errors::process_env::set_var("BEARDOG_ORCHESTRATOR_ID", orch_id);
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
    if let Ok(family_seed) = beardog_errors::process_env::var("BEARDOG_FAMILY_SEED") {
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
            BearDogError::configuration(&format!("Failed to create Unix socket server: {e}"))
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

    // Step 7.5: Register with discovery (Neural API first, then legacy capability registry client)
    info!("🌐 Registering with discovery service...");
    let neural_registration = NeuralRegistrationParams::from_env();
    match register_with_discovery_service(&socket_config, &neural_registration).await {
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
    info!("   • Fully memory-safe");
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

/// Register BearDog with a runtime-discovered discovery/registry endpoint
///
/// 1. **Primary**: Neural API (`capability.call` semantics via `neural_registration`).
/// 2. **Fallback**: Legacy JSON-RPC registry client (`SongbirdClient`) for deployments
///    that have not migrated — still capability-oriented at the protocol level.
///
/// # Returns
///
/// Ok(()) if registered successfully with any service, Err if all methods fail.
/// Non-fatal - BearDog can operate standalone without discovery.
async fn register_with_discovery_service(
    socket_config: &SocketConfig,
    neural_registration: &NeuralRegistrationParams,
) -> anyhow::Result<()> {
    use anyhow::Context;
    use beardog_ipc::{discover_neural_api_socket, register_with_neural_api};
    use beardog_types::primal_identity::PrimalIdentity;

    // PHASE 1: Try Neural API (TRUE PRIMAL pattern)
    if let Some(neural_socket) = discover_neural_api_socket() {
        info!("🌐 Neural API detected at: {}", neural_socket);

        // Get primal identity from environment
        let identity = PrimalIdentity::from_env()
            .context("Failed to load primal identity for registration")?;

        let registration_instance = neural_registration.registration_instance_id(&identity);
        let socket_path = socket_config.socket_path_string();

        match register_with_neural_api(&neural_socket, &registration_instance, &socket_path).await {
            Ok(()) => {
                info!("✅ Registered with Neural API (TRUE PRIMAL)");
                return Ok(());
            }
            Err(e) => {
                warn!("⚠️  Neural API registration failed: {}", e);
                // Fall through to legacy registration
            }
        }
    } else {
        debug!("ℹ️  Neural API socket not detected; trying legacy registry client...");
    }

    // PHASE 2: Fallback to legacy registry transport (deprecated path)
    #[allow(deprecated)]
    match register_with_legacy_songbird().await {
        Ok(()) => {
            info!(
                "✅ Registered with legacy discovery registry (migrate to Neural API when available)"
            );
            Ok(())
        }
        Err(e) => {
            warn!(
                "⚠️  No discovery endpoint available (Neural API or legacy registry): {}",
                e
            );
            Err(e)
        }
    }
}

/// Legacy registry registration (deprecated transport; capability list is unchanged)
///
/// Prefer `register_with_neural_api` for semantic routing without a fixed registry implementation.
#[deprecated(
    since = "0.9.1",
    note = "Use Neural API registration for TRUE PRIMAL pattern"
)]
async fn register_with_legacy_songbird() -> anyhow::Result<()> {
    use beardog_ipc::{Capability, SongbirdClient};

    // Connects via `beardog-ipc` discovery (env + fallbacks — no hardcoded peer host)
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

    let primal_name = crate::unix_socket_ipc::handlers::utils::get_primal_name();
    client.register(&primal_name, capabilities).await?;

    let heartbeat_interval = beardog_ipc::DEFAULT_HEARTBEAT_INTERVAL;
    tokio::spawn(async move {
        let _heartbeat = client.start_heartbeat(heartbeat_interval);
        // Heartbeat task runs until client is dropped
        // This keeps BearDog registered with Songbird
        std::future::pending::<()>().await;
    });

    Ok(())
}

/// First alphanumeric characters of a family seed (for logging only).
pub(crate) fn family_id_preview_from_seed(seed: &str) -> String {
    seed.chars()
        .filter(|c| c.is_alphanumeric())
        .take(4)
        .collect::<String>()
        .to_lowercase()
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
        if let Err(e) = signal::ctrl_c().await {
            error!("Failed to install Ctrl+C handler: {e}");
            // Fall back to pending — SIGTERM or other shutdown path will be used
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

// -----------------------------------------------------------------------------
// Unit tests (same module as `display_banner` — exercises startup banner logic
// without binding sockets or running the full `run` lifecycle).
// -----------------------------------------------------------------------------

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

#[cfg(test)]
mod neural_registration_tests {
    use super::NeuralRegistrationParams;
    use beardog_types::primal_identity::PrimalIdentity;

    #[test]
    fn registration_instance_id_uses_override_when_set() {
        let p = NeuralRegistrationParams {
            instance_override: Some("my-reg".to_string()),
            primal_type: Some("ignored".to_string()),
            beardog_primal_type: Some("ignored2".to_string()),
        };
        let id = PrimalIdentity::for_test("fam", "node-z");
        assert_eq!(p.registration_instance_id(&id), "my-reg");
    }

    #[test]
    fn registration_instance_id_uses_primal_type_when_no_override() {
        let p = NeuralRegistrationParams {
            instance_override: None,
            primal_type: Some("compute".to_string()),
            beardog_primal_type: None,
        };
        let id = PrimalIdentity::for_test("fam", "node-a");
        assert_eq!(p.registration_instance_id(&id), "compute-node-a");
    }

    #[test]
    fn registration_instance_id_falls_back_to_beardog_primal_type() {
        let p = NeuralRegistrationParams {
            instance_override: None,
            primal_type: None,
            beardog_primal_type: Some("edge".to_string()),
        };
        let id = PrimalIdentity::for_test("fam", "node-b");
        assert_eq!(p.registration_instance_id(&id), "edge-node-b");
    }

    #[test]
    fn registration_instance_id_default_role_is_security() {
        let p = NeuralRegistrationParams::default();
        let id = PrimalIdentity::for_test("fam", "node-c");
        assert_eq!(p.registration_instance_id(&id), "security-node-c");
    }

    #[test]
    fn primal_type_precedence_over_beardog_primal_type() {
        let p = NeuralRegistrationParams {
            instance_override: None,
            primal_type: Some("primary".to_string()),
            beardog_primal_type: Some("secondary".to_string()),
        };
        let id = PrimalIdentity::for_test("fam", "n");
        assert_eq!(p.registration_instance_id(&id), "primary-n");
    }

    #[test]
    fn family_id_preview_from_seed_filters_and_lowercases() {
        assert_eq!(super::family_id_preview_from_seed("Ab12!@#xy"), "ab12");
        assert_eq!(super::family_id_preview_from_seed("!!!"), "");
    }

    #[test]
    fn neural_registration_params_from_env_reads_overrides() {
        beardog_errors::process_env::set_var("BEARDOG_NEURAL_REGISTRATION_INSTANCE", "inst-x");
        beardog_errors::process_env::set_var("PRIMAL_TYPE", "compute");
        beardog_errors::process_env::set_var("BEARDOG_PRIMAL_TYPE", "ignored");
        let p = NeuralRegistrationParams::from_env();
        beardog_errors::process_env::remove_var("BEARDOG_NEURAL_REGISTRATION_INSTANCE");
        beardog_errors::process_env::remove_var("PRIMAL_TYPE");
        beardog_errors::process_env::remove_var("BEARDOG_PRIMAL_TYPE");
        assert_eq!(p.instance_override, Some("inst-x".to_string()));
        assert_eq!(p.primal_type, Some("compute".to_string()));
        assert_eq!(p.beardog_primal_type, Some("ignored".to_string()));
    }
}
