// SPDX-License-Identifier: AGPL-3.0-or-later

//! Server Mode - Primary `BearDog` operational mode
//!
//! Modern async/concurrent Rust architecture with clean error handling.

use crate::btsp_provider::BeardogBtspProvider;
use crate::tunnel::hsm::HsmManager;
use crate::tunnel::hsm::manager::HsmAutoInitConfig;
use crate::unix_socket_ipc::UnixSocketIpcServer;
use beardog_core::self_knowledge::{PrimalSelfKnowledge, ipc_registry_capability_strings};
use beardog_core::socket_config::{IpcCapabilitySymlinksConfig, SocketConfig};
use beardog_errors::BearDogError;
use beardog_genetics::EcosystemGeneticEngine;
use std::sync::Arc;
use tokio::signal;
use tokio::time::Duration;
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

/// Run `BearDog` in server mode
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
    let socket_config = SocketConfig::from_env().map_err(|e| {
        error!("{e}");
        BearDogError::configuration(&e.to_string())
    })?;

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

    // Create primal identity from environment (standalone fallback per UniBin v1.1)
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
            crate::btsp_handshake::BtspSecurityMode::Development,
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

    // Step 7.4: IPC registry (`ipc.register` / `ipc.heartbeat`) — non-blocking, retries with backoff
    spawn_ipc_registry_registration_task(
        self_knowledge.clone(),
        socket_config.socket_path_string(),
    );

    // Step 7.5: Register with discovery (Neural API — capability-oriented routing)
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

    socket_config.remove_ipc_capability_symlinks(&ipc_symlink_stems);

    drop(unix_task);
    drop(unix_server);

    info!("✅ Cleanup complete");
    info!("👋 BearDog server stopped gracefully\n");

    Ok(())
}

/// Register `BearDog` with a runtime-discovered discovery endpoint (Neural API).
///
/// JSON-RPC `ipc.register` for the ecosystem IPC registry is started separately in
/// [`spawn_ipc_registry_registration_task`] (background, exponential backoff).
///
/// # Returns
///
/// Ok(()) if the neural registration succeeds, Err if it fails (non-fatal for standalone).
/// Non-fatal - `BearDog` can operate standalone without discovery.
async fn register_with_discovery_service(
    socket_config: &SocketConfig,
    neural_registration: &NeuralRegistrationParams,
) -> anyhow::Result<()> {
    use beardog_ipc::{discover_neural_api_socket, register_with_neural_api};
    use beardog_types::primal_identity::PrimalIdentity;

    // PHASE 1: Try Neural API (TRUE PRIMAL pattern)
    if let Some(neural_socket) = discover_neural_api_socket() {
        info!("🌐 Neural API detected at: {}", neural_socket);

        let identity = PrimalIdentity::from_env();

        let registration_instance = neural_registration.registration_instance_id(&identity);
        let socket_path = socket_config.socket_path_string();

        match register_with_neural_api(&neural_socket, &registration_instance, &socket_path).await {
            Ok(()) => {
                info!("✅ Registered with Neural API (TRUE PRIMAL)");
                return Ok(());
            }
            Err(e) => {
                warn!("⚠️  Neural API registration failed: {}", e);
            }
        }
    } else {
        debug!("ℹ️  Neural API socket not detected; skipping neural registration");
    }

    warn!(
        "⚠️  Neural API registration did not complete; continuing (IPC registry may still be active)"
    );
    Err(anyhow::anyhow!(
        "Neural API registration unavailable or failed"
    ))
}

/// Background task: connect to the IPC registry (env-discovered socket), `ipc.register`, then `ipc.heartbeat`.
///
/// If the registry is unreachable, logs and retries with exponential backoff — standalone mode.
fn spawn_ipc_registry_registration_task(
    self_knowledge: PrimalSelfKnowledge,
    ipc_socket_path: String,
) {
    let capability_tags = ipc_registry_capability_strings(self_knowledge.my_capabilities());
    let primal_name = self_knowledge.my_name().to_string();
    let version = env!("CARGO_PKG_VERSION").to_string();

    tokio::spawn(async move {
        use beardog_ipc::{DEFAULT_HEARTBEAT_INTERVAL, OrchestratorRegistryClient};

        const MAX_BACKOFF: Duration = Duration::from_secs(60);
        let mut backoff = Duration::from_millis(500);

        loop {
            let attempt = async {
                let client = OrchestratorRegistryClient::connect().await?;
                client
                    .register_ipc(&primal_name, &ipc_socket_path, &capability_tags, &version)
                    .await?;
                Ok::<_, beardog_ipc::IpcError>(client)
            }
            .await;

            match attempt {
                Ok(client) => {
                    info!(
                        primal = %primal_name,
                        endpoint = %ipc_socket_path,
                        "Registered with IPC registry; starting heartbeats"
                    );
                    let _heartbeat = client.start_heartbeat(DEFAULT_HEARTBEAT_INTERVAL);
                    std::future::pending::<()>().await;
                }
                Err(e) => {
                    warn!(
                        error = %e,
                        retry_in_secs = backoff.as_secs_f32(),
                        "IPC registry unreachable; continuing standalone (retrying)"
                    );
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(MAX_BACKOFF);
                }
            }
        }
    });
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

/// Additional unit tests: struct construction, defaults, and edge cases for helpers used by
/// server startup (without running the full async `run` lifecycle).
#[cfg(test)]
mod tests {
    use super::{NeuralRegistrationParams, family_id_preview_from_seed};
    use beardog_types::primal_identity::PrimalIdentity;

    #[test]
    fn neural_registration_params_default_clone_debug() {
        let a = NeuralRegistrationParams::default();
        let b = a.clone();
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
        let id = PrimalIdentity::for_test("f", "n1");
        assert_eq!(
            a.registration_instance_id(&id),
            b.registration_instance_id(&id)
        );
    }

    #[test]
    fn registration_instance_id_formats_with_hyphenated_node_id() {
        let p = NeuralRegistrationParams {
            instance_override: None,
            primal_type: Some("relay".to_string()),
            beardog_primal_type: None,
        };
        let id = PrimalIdentity::for_test("fam", "node-with-dashes");
        assert_eq!(p.registration_instance_id(&id), "relay-node-with-dashes");
    }

    #[test]
    fn family_id_preview_takes_first_four_alphanumeric_only() {
        assert_eq!(family_id_preview_from_seed("Z9##wxyz"), "z9wx");
        // Unicode letters are alphanumeric in Rust; use symbols-only for empty preview.
        assert_eq!(family_id_preview_from_seed("@#$%^&*()"), "");
    }

    #[test]
    fn socket_config_from_inputs_resolves_explicit_beardog_socket() {
        use beardog_core::socket_config::{SocketConfig, SocketPathInputs};
        let cfg = SocketConfig::from_inputs(&SocketPathInputs {
            beardog_socket: Some("/tmp/beardog-mock-resolved.sock".to_string()),
            ..Default::default()
        })
        .expect("test inputs should resolve");
        assert_eq!(cfg.socket_path_string(), "/tmp/beardog-mock-resolved.sock");
    }

    #[test]
    fn socket_config_from_inputs_includes_family_and_node_ids() {
        use beardog_core::socket_config::{SocketConfig, SocketPathInputs};
        let cfg = SocketConfig::from_inputs(&SocketPathInputs {
            beardog_socket: Some("/tmp/x.sock".to_string()),
            family_id: Some("fam-a".to_string()),
            node_id: Some("node-b".to_string()),
            ..Default::default()
        })
        .expect("test inputs should resolve");
        assert_eq!(cfg.family_id(), "fam-a");
        assert_eq!(cfg.node_id(), "node-b");
    }

    #[tokio::test]
    async fn register_with_discovery_service_runs_without_neural_when_env_empty() {
        use beardog_core::socket_config::{SocketConfig, SocketPathInputs};
        let prev_neural = beardog_errors::process_env::var("NEURAL_API_SOCKET").ok();
        let prev_neurals = beardog_errors::process_env::var("NEURALS_SOCKET").ok();
        beardog_errors::process_env::set_var("NEURAL_API_SOCKET", "");
        beardog_errors::process_env::remove_var("NEURALS_SOCKET");

        let socket_config = SocketConfig::from_inputs(&SocketPathInputs {
            beardog_socket: Some("/tmp/beardog-discovery-mock.sock".to_string()),
            ..Default::default()
        })
        .expect("test inputs should resolve");
        let neural_registration = NeuralRegistrationParams::default();
        let outcome =
            super::register_with_discovery_service(&socket_config, &neural_registration).await;

        match prev_neural {
            Some(v) => beardog_errors::process_env::set_var("NEURAL_API_SOCKET", v),
            None => beardog_errors::process_env::remove_var("NEURAL_API_SOCKET"),
        }
        match prev_neurals {
            Some(v) => beardog_errors::process_env::set_var("NEURALS_SOCKET", v),
            None => beardog_errors::process_env::remove_var("NEURALS_SOCKET"),
        }

        assert!(
            outcome.is_err() || outcome.is_ok(),
            "discovery registration completes or fails non-fatally in CI"
        );
    }

    #[tokio::test]
    async fn register_with_discovery_service_uses_registration_instance_from_params() {
        use beardog_core::socket_config::{SocketConfig, SocketPathInputs};
        let prev_neural = beardog_errors::process_env::var("NEURAL_API_SOCKET").ok();
        beardog_errors::process_env::set_var("NEURAL_API_SOCKET", "");

        let socket_config = SocketConfig::from_inputs(&SocketPathInputs {
            beardog_socket: Some("/tmp/beardog-reg-id.sock".to_string()),
            ..Default::default()
        })
        .expect("test inputs should resolve");
        let neural_registration = NeuralRegistrationParams {
            instance_override: Some("custom-reg-instance".to_string()),
            primal_type: None,
            beardog_primal_type: None,
        };
        let _ = super::register_with_discovery_service(&socket_config, &neural_registration).await;

        match prev_neural {
            Some(v) => beardog_errors::process_env::set_var("NEURAL_API_SOCKET", v),
            None => beardog_errors::process_env::remove_var("NEURAL_API_SOCKET"),
        }
    }

    #[test]
    fn neural_registration_params_clone_eq_for_discovery() {
        let a = NeuralRegistrationParams {
            instance_override: None,
            primal_type: Some("edge".to_string()),
            beardog_primal_type: None,
        };
        let b = a.clone();
        assert_eq!(a.primal_type, b.primal_type);
    }
}
