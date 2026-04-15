// SPDX-License-Identifier: AGPL-3.0-or-later

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
use beardog_tunnel::btsp_handshake;
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::multi_transport_server::MultiTransportServer;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use beardog_tunnel::tunnel::hsm::software_hsm::RustSoftwareHsm;
use beardog_tunnel::tunnel::hsm::{HsmTier, SoftwareHsmConfig};
use beardog_types::constants::domains::network::ipc_discovery::resolve_biomeos_ipc_subdir_from_optional;
use std::sync::Arc;
use tracing::{info, warn};

/// Resolve TCP listen address from [`ServerArgs::port`] / [`ServerArgs::listen`] (`UniBin` v1.1).
///
/// Returns [`None`] when neither is set, or when both are set (normally prevented by `clap`
/// `conflicts_with`, but callers may construct [`ServerArgs`] programmatically).
#[must_use]
pub fn resolve_effective_tcp_listen(port: Option<u16>, listen: Option<&str>) -> Option<String> {
    match (port, listen) {
        (Some(p), None) => Some(format!("0.0.0.0:{p}")),
        (None, Some(addr)) => Some(addr.to_string()),
        _ => None,
    }
}

/// Resolve the effective socket path for server startup (abstract, multi-family, or explicit).
pub fn resolve_server_socket_path(args: &ServerArgs) -> String {
    if args.r#abstract {
        let family = args.family_id.as_deref().unwrap_or("default");
        let ns = resolve_biomeos_ipc_subdir_from_optional(None);
        format!("@{ns}_beardog_{family}")
    } else if let Some(ref family_id) = args.family_id {
        let family_sock = std::path::PathBuf::from(&args.socket);
        let parent = family_sock
            .parent()
            .unwrap_or_else(|| std::path::Path::new("/tmp"));
        parent
            .join(format!("beardog-{family_id}.sock"))
            .to_string_lossy()
            .to_string()
    } else {
        args.socket.clone()
    }
}

/// Address string advertised to the Neural API for registration (`TCP` vs Unix path).
#[must_use]
pub(super) fn neural_registration_address<'a>(
    tcp_listen: Option<&'a str>,
    unix_socket_path: &'a str,
) -> &'a str {
    tcp_listen.unwrap_or(unix_socket_path)
}

/// Build a signed attestation for Neural API registration using the primal's
/// unified Ed25519 identity key.
fn build_neural_attestation(primal_name: &str) -> serde_json::Value {
    use beardog_tunnel::unix_socket_ipc::handlers::primal_signing::{
        canonical_announcement_message, sign_with_primal_identity,
    };

    let node_id = beardog_types::primal_identity::resolve_node_id_from_env_or_ephemeral(None);
    let version = env!("CARGO_PKG_VERSION");
    let methods: Vec<String> = Vec::new();
    let message = canonical_announcement_message(primal_name, version, &methods);
    let (signature, public_key) = sign_with_primal_identity(primal_name, &node_id, &message);

    serde_json::json!({
        "schema_version": 2,
        "algorithm": "ed25519",
        "public_key": public_key,
        "signature": signature,
        "signed_fields": ["primal", "version"],
    })
}

/// Handle server command - start long-running service
///
/// # Errors
///
/// Returns an error if neural API registration, HSM setup, or transport binding fails, or the
/// server exits with an error.
pub async fn handle_server(args: ServerArgs) -> Result<(), BearDogError> {
    info!("BearDog server starting");

    if let Some(ref dir) = args.audit_dir {
        info!(audit_dir = %dir.display(), "audit directory override");
    }

    // Determine socket path - use abstract socket if --abstract flag is set,
    // or derive family-scoped socket if --family-id is provided
    let socket_path = resolve_server_socket_path(&args);
    if args.r#abstract {
        info!(
            transport = "abstract-namespace",
            "transport selected (no filesystem path)"
        );
        info!(abstract_name = %socket_path, "bound to abstract namespace (kernel-only, not on disk)");
    } else if args.family_id.is_some() {
        info!(socket_path = %socket_path, "multi-family socket");
    }

    // Resolve --port into --listen (UniBin v1.1: `server --port <PORT>`)
    let effective_listen = resolve_effective_tcp_listen(args.port, args.listen.as_deref());

    // Determine transport mode
    if let Some(ref addr) = effective_listen {
        info!(transport = "tcp", tier = 2, "transport selected");
        info!(listen = %addr, "listen address");
    } else if !args.r#abstract {
        info!(transport = "unix", tier = 1, "transport selected");
        info!(socket_path = %socket_path, "unix socket path");
    }

    if let Some(ref family_id) = args.family_id {
        info!(family_id = %family_id, "family id");
    }

    if let Some(ref orchestrator_id) = args.orchestrator_id {
        info!(orchestrator_id = %orchestrator_id, "orchestrator id");
    }

    // Create HSM manager with software provider
    info!("initializing HSM manager");
    let mut hsm = HsmManager::new();
    let config = SoftwareHsmConfig::default();
    let software_hsm =
        RustSoftwareHsm::new(config)
            .await
            .map_err(|e| BearDogError::Initialization {
                message: format!("Failed to create software HSM: {e}"),
            })?;

    hsm.register_hsm_provider(HsmTier::Software, Arc::new(software_hsm))
        .map_err(|e| BearDogError::Initialization {
            message: format!("Failed to register HSM provider: {e}"),
        })?;
    let hsm = Arc::new(hsm);
    info!("HSM manager initialized");

    // Create genetics engine
    info!("initializing genetics engine");
    let genetics =
        Arc::new(
            EcosystemGeneticEngine::new().map_err(|e| BearDogError::Initialization {
                message: format!("Failed to create genetics engine: {e}"),
            })?,
        );
    info!("Genetics engine initialized");

    // Create BTSP provider (provides all capabilities)
    info!("initializing BTSP provider");
    let btsp_provider = Arc::new(BeardogBtspProvider::new(hsm, genetics).await.map_err(|e| {
        BearDogError::Initialization {
            message: format!("Failed to create BTSP provider: {e}"),
        }
    })?);
    info!("BTSP provider initialized");

    // Create primal identity from environment (standalone fallback per UniBin v1.1)
    let identity = Arc::new(beardog_types::primal_identity::PrimalIdentity::from_env());
    if identity.is_standalone() {
        info!(
            mode = "standalone",
            "no identity env vars set — running in standalone mode"
        );
    }
    info!(
        family_id = %identity.family_id(),
        node_id = %identity.node_id(),
        standalone = identity.is_standalone(),
        "identity"
    );

    // Construct primal name for registration (before identity is moved)
    let primal_name = format!("beardog-{}", identity.node_id());
    // socket_path is already determined above based on --abstract flag
    let tcp_addr = effective_listen;

    // ================================================================
    // BTSP SECURITY MODE (per BTSP_PROTOCOL_STANDARD.md)
    // ================================================================

    let security_mode = btsp_handshake::resolve_security_mode()?;
    match &security_mode {
        btsp_handshake::BtspSecurityMode::Production { .. } => {
            info!(mode = "production", "BTSP handshake enforcement ACTIVE");
        }
        btsp_handshake::BtspSecurityMode::Development => {
            info!(mode = "development", "BTSP handshake enforcement disabled");
        }
    }

    // ================================================================
    // PHASE 3: MULTI-TRANSPORT SERVER (Deep Debt Evolution)
    // ================================================================

    info!(platform = "universal", "creating multi-transport server");

    let server = MultiTransportServer::bind_all_available(
        btsp_provider,
        identity,
        &socket_path,
        tcp_addr.as_deref(),
        security_mode,
    )
    .await?;

    info!(
        transport_count = server.transport_count(),
        "multi-transport server created"
    );

    // Auto-register with Neural API if available (Tower Atomic TRUE PRIMAL)
    if let Some(neural_socket) = discover_neural_api_socket() {
        info!(neural_socket = %neural_socket, "Neural API detected");

        let registration_addr =
            neural_registration_address(tcp_addr.as_deref(), socket_path.as_str());

        let attestation = build_neural_attestation(&primal_name);
        match register_with_neural_api(
            &neural_socket,
            &primal_name,
            registration_addr,
            Some(&attestation),
        )
        .await
        {
            Ok(()) => info!("registered with Neural API"),
            Err(e) => warn!(error = %e, "Neural API registration failed (non-fatal)"),
        }
    } else {
        info!(mode = "standalone", "no Neural API detected");
    }

    // Best-effort orchestrator registry registration (non-fatal per PRIMAL IPC Protocol v3.1)
    attempt_orchestrator_registration(&socket_path, tcp_addr.as_deref()).await;

    // Start all transports (runs until Ctrl+C)
    server.start_all().await?;

    Ok(())
}

/// Best-effort registration with the ecosystem's IPC registry (non-fatal).
///
/// Attempts to connect to whatever orchestrator provides the registry capability
/// and register `BearDog`'s capabilities. The caller never knows which primal
/// provides the registry — capability-based discovery handles routing.
///
/// Failure is logged and swallowed per PRIMAL IPC Protocol v3.1:
/// registration SHOULD be attempted but MUST NOT prevent standalone operation.
async fn attempt_orchestrator_registration(_socket_path: &str, _tcp_addr: Option<&str>) {
    use beardog_ipc::{Capability, OrchestratorRegistryClient};

    let Ok(client) = OrchestratorRegistryClient::connect().await else {
        info!("no IPC registry socket found (standalone operation)");
        return;
    };

    let capabilities = vec![Capability::Crypto, Capability::BTSP, Capability::Ed25519];

    if let Err(e) = client.register("beardog", capabilities).await {
        warn!(error = %e, "IPC registry registration failed (non-fatal)");
    } else {
        info!("registered with ecosystem IPC registry");
    }
}

#[cfg(test)]
mod server_handler_tests {
    use crate::ServerArgs;
    use beardog_types::constants::domains::network::ipc_discovery::resolve_biomeos_ipc_subdir_from_optional;

    use super::{
        attempt_orchestrator_registration, neural_registration_address,
        resolve_effective_tcp_listen, resolve_server_socket_path,
    };

    #[test]
    fn neural_registration_address_prefers_tcp_when_present() {
        assert_eq!(
            neural_registration_address(Some("0.0.0.0:9000"), "/tmp/x.sock"),
            "0.0.0.0:9000"
        );
    }

    #[test]
    fn neural_registration_address_falls_back_to_unix_path() {
        assert_eq!(
            neural_registration_address(None, "@abstract_sock"),
            "@abstract_sock"
        );
    }

    #[test]
    fn resolve_effective_tcp_listen_from_port_only() {
        assert_eq!(
            resolve_effective_tcp_listen(Some(9900), None).as_deref(),
            Some("0.0.0.0:9900")
        );
    }

    #[test]
    fn resolve_effective_tcp_listen_from_listen_only() {
        assert_eq!(
            resolve_effective_tcp_listen(None, Some("127.0.0.1:7777")).as_deref(),
            Some("127.0.0.1:7777")
        );
    }

    #[test]
    fn resolve_effective_tcp_listen_none_when_neither_set() {
        assert_eq!(resolve_effective_tcp_listen(None, None), None);
    }

    #[test]
    fn resolve_effective_tcp_listen_none_when_both_set_like_invalid_cli_state() {
        assert_eq!(
            resolve_effective_tcp_listen(Some(8080), Some("127.0.0.1:1")),
            None
        );
    }

    #[test]
    fn resolve_server_socket_path_abstract_default_family() {
        let args = ServerArgs {
            socket: "/tmp/ignored.sock".to_string(),
            r#abstract: true,
            port: None,
            listen: None,
            audit_dir: None,
            family_id: None,
            orchestrator_id: None,
        };
        let ns = resolve_biomeos_ipc_subdir_from_optional(None);
        assert_eq!(
            resolve_server_socket_path(&args),
            format!("@{ns}_beardog_default")
        );
    }

    #[test]
    fn resolve_server_socket_path_abstract_named_family() {
        let args = ServerArgs {
            socket: "/tmp/ignored.sock".to_string(),
            r#abstract: true,
            port: None,
            listen: None,
            audit_dir: None,
            family_id: Some("alpha".to_string()),
            orchestrator_id: None,
        };
        let ns = resolve_biomeos_ipc_subdir_from_optional(None);
        assert_eq!(
            resolve_server_socket_path(&args),
            format!("@{ns}_beardog_alpha")
        );
    }

    #[test]
    fn resolve_server_socket_path_family_scoped_file() {
        let args = ServerArgs {
            socket: "/var/run/beardog.sock".to_string(),
            r#abstract: false,
            port: None,
            listen: None,
            audit_dir: None,
            family_id: Some("fam99".to_string()),
            orchestrator_id: None,
        };
        assert_eq!(
            resolve_server_socket_path(&args),
            "/var/run/beardog-fam99.sock"
        );
    }

    #[test]
    fn resolve_server_socket_path_family_with_socket_filename_only_uses_parent_join() {
        let args = ServerArgs {
            socket: "beardog.sock".to_string(),
            r#abstract: false,
            port: None,
            listen: None,
            audit_dir: None,
            family_id: Some("rel".to_string()),
            orchestrator_id: None,
        };
        let resolved = resolve_server_socket_path(&args);
        assert!(
            resolved.ends_with("beardog-rel.sock"),
            "unexpected path: {resolved}"
        );
    }

    #[test]
    fn resolve_server_socket_path_explicit_when_no_family() {
        let args = ServerArgs {
            socket: "/tmp/custom.sock".to_string(),
            r#abstract: false,
            port: None,
            listen: None,
            audit_dir: None,
            family_id: None,
            orchestrator_id: None,
        };
        assert_eq!(resolve_server_socket_path(&args), "/tmp/custom.sock");
    }

    #[tokio::test]
    async fn attempt_orchestrator_registration_completes_without_panic() {
        attempt_orchestrator_registration("/tmp/beardog_unit_test_orchestrator.sock", None).await;
    }

    #[tokio::test]
    async fn attempt_orchestrator_registration_with_tcp_addr_completes_without_panic() {
        attempt_orchestrator_registration("/tmp/beardog.sock", Some("127.0.0.1:9900")).await;
    }
}
