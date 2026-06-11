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
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use beardog_genetics::EcosystemGeneticEngine;
use beardog_ipc::{
    beardog_announce_method_names, discover_neural_api_socket, register_with_neural_api,
    send_primal_announce,
};
use beardog_tunnel::btsp_handshake;
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::multi_transport_server::MultiTransportServer;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use beardog_tunnel::tunnel::hsm::software_hsm::RustSoftwareHsm;
use beardog_tunnel::tunnel::hsm::{HsmProviderBackend, HsmTier, SoftwareHsmConfig};
use beardog_types::constants::domains::network::addresses::WILDCARD_IPV4;
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
        (Some(p), None) => Some(format!("{WILDCARD_IPV4}:{p}")),
        (None, Some(addr)) => Some(addr.to_string()),
        _ => None,
    }
}

/// Resolve the effective socket path for server startup.
///
/// Bind-mode priority: `--bind-mode abstract` (or legacy `--abstract`) → abstract
/// namespace name. `--bind-mode tcp` → returns a placeholder (UDS is skipped
/// later). `--bind-mode filesystem` or `auto` → family-aware filesystem path.
pub fn resolve_server_socket_path(args: &ServerArgs) -> String {
    let use_abstract =
        args.bind_mode == crate::BindMode::Abstract || args.r#abstract;

    if use_abstract {
        let family = args.family_id.as_deref().unwrap_or("default");
        let ns = resolve_biomeos_ipc_subdir_from_optional(None);
        format!("@{ns}_beardog_{family}")
    } else if args.bind_mode == crate::BindMode::Tcp {
        String::new()
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

    info!(bind_mode = ?args.bind_mode, "guideStone startup contract");

    if let Some(ref dir) = args.audit_dir {
        info!(audit_dir = %dir.display(), "audit directory override");
    }

    let use_abstract =
        args.bind_mode == crate::BindMode::Abstract || args.r#abstract;
    let tcp_only = args.bind_mode == crate::BindMode::Tcp;

    if tcp_only && args.port.is_none() && args.listen.is_none() {
        return Err(BearDogError::configuration(
            "--bind-mode tcp requires --port or PORT env var",
        ));
    }

    // ================================================================
    // TRANSPORT RESOLUTION (Tier 0 → bind-mode → legacy tiers)
    // ================================================================

    // Tier 0: TRANSPORT_ENDPOINT env var (orchestrator-injected, highest priority)
    let transport_endpoint = beardog_types::btsp::TransportEndpoint::from_env().ok();
    let (socket_path, effective_listen) = if let Some(ref ep) = transport_endpoint {
        info!(endpoint = %ep, tier = 0, "TRANSPORT_ENDPOINT override");
        match ep {
            beardog_types::btsp::TransportEndpoint::Uds { path } => {
                (path.to_string_lossy().to_string(), None)
            }
            beardog_types::btsp::TransportEndpoint::Tcp { host, port } => {
                let addr = format!("{host}:{port}");
                let fallback_socket = resolve_server_socket_path(&args);
                (fallback_socket, Some(addr))
            }
            beardog_types::btsp::TransportEndpoint::MeshRelay { .. } => {
                return Err(BearDogError::configuration(
                    "TRANSPORT_ENDPOINT mesh_relay is not supported for server bind",
                ));
            }
        }
    } else {
        let socket_path = resolve_server_socket_path(&args);

        if use_abstract {
            info!(
                transport = "abstract-namespace",
                bind_mode = ?args.bind_mode,
                "transport selected (no filesystem path)"
            );
            info!(abstract_name = %socket_path, "bound to abstract namespace (kernel-only, not on disk)");
        } else if tcp_only {
            info!(bind_mode = "tcp", "UDS disabled — TCP-only mode");
        } else if args.family_id.is_some() {
            info!(socket_path = %socket_path, "multi-family socket");
        }

        let effective_listen = resolve_effective_tcp_listen(args.port, args.listen.as_deref());
        (socket_path, effective_listen)
    };

    // Prepare socket directory: ensure parent exists and stale socket is removed.
    // Skip for abstract sockets (no filesystem path) and TCP-only mode.
    if !use_abstract
        && !tcp_only
        && transport_endpoint
            .as_ref()
            .is_none_or(|ep| matches!(ep, beardog_types::btsp::TransportEndpoint::Uds { .. }))
    {
        let sock = std::path::Path::new(&socket_path);
        if let Some(parent) = sock.parent()
            && !parent.exists()
        {
            info!(dir = %parent.display(), "creating socket directory");
            std::fs::create_dir_all(parent).map_err(|e| {
                BearDogError::configuration(&format!(
                    "Cannot create socket directory {}: {e}",
                    parent.display()
                ))
            })?;
        }
        if sock.exists() {
            info!(path = %sock.display(), "removing stale socket");
            std::fs::remove_file(sock).map_err(|e| {
                BearDogError::configuration(&format!(
                    "Cannot remove stale socket {}: {e}",
                    sock.display()
                ))
            })?;
        }
    }

    // Log transport selection
    if let Some(ref addr) = effective_listen {
        let tier = transport_endpoint.as_ref().map_or(5_u8, |_| 0);
        info!(transport = "tcp", tier, "transport selected");
        info!(listen = %addr, "listen address");
    } else if !use_abstract && !tcp_only {
        let tier = transport_endpoint.as_ref().map_or(1_u8, |_| 0);
        info!(transport = "unix", tier, "transport selected");
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

    hsm.register_hsm_provider(
        HsmTier::Software,
        Arc::new(HsmProviderBackend::RustSoftware(software_hsm)),
    )
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

    let uds_path = if tcp_only { None } else { Some(socket_path.as_str()) };
    let server = MultiTransportServer::bind_all_available(
        btsp_provider,
        identity,
        uds_path,
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

        // Legacy capability.register calls (backwards compat)
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

        // biomeOS v3.69+ primal.announce (Wave 43 — push-style ecosystem registration)
        let announce_methods: Vec<String> = beardog_announce_method_names()
            .iter()
            .map(|s| String::from(*s))
            .collect();
        match send_primal_announce(
            &neural_socket,
            &primal_name,
            &socket_path,
            &announce_methods,
            Some(&attestation),
        )
        .await
        {
            Ok(()) => info!("primal.announce sent to biomeOS"),
            Err(e) => warn!(error = %e, "primal.announce failed (non-fatal)"),
        }
    } else {
        info!(mode = "standalone", "no Neural API detected");
    }

    // Best-effort orchestrator registry registration (non-fatal per PRIMAL IPC Protocol v3.1)
    attempt_orchestrator_registration(&socket_path, tcp_addr.as_deref()).await;

    // ACME renewal daemon (gated by BEARDOG_TLS_MODE=acme)
    if std::env::var(env_keys::ENV_TLS_MODE)
        .ok()
        .is_some_and(|v| v.eq_ignore_ascii_case("acme"))
    {
        match spawn_acme_renewal_daemon() {
            Ok(()) => info!("ACME renewal daemon spawned"),
            Err(e) => warn!(error = %e, "ACME daemon init failed (non-fatal)"),
        }
    }

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

/// Spawn the ACME renewal daemon as a background tokio task.
///
/// Reads config from `BEARDOG_ACME_DOMAINS`, `BEARDOG_ACME_EMAIL`, etc.
/// The daemon runs `AcmeClient::run_renewal_loop()` which checks cert
/// expiry every 12 hours and renews when within 30 days of expiration.
///
/// # Errors
///
/// Returns an error if `AcmeConfig::from_env()` or `AcmeClient::new()`
/// fails (e.g., missing `BEARDOG_ACME_DOMAINS`).
fn spawn_acme_renewal_daemon() -> Result<(), BearDogError> {
    let config =
        beardog_acme::AcmeConfig::from_env().map_err(|e| BearDogError::Initialization {
            message: format!("ACME config: {e}"),
        })?;

    info!(
        domains = ?config.domains,
        renewal_days = config.renewal_days_before_expiry,
        "initializing ACME renewal daemon"
    );

    let mut client =
        beardog_acme::AcmeClient::new(config).map_err(|e| BearDogError::Initialization {
            message: format!("ACME client: {e}"),
        })?;

    tokio::spawn(async move {
        client.run_renewal_loop().await;
    });

    Ok(())
}

#[cfg(test)]
mod server_handler_tests {
    use crate::{BindMode, ServerArgs};
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
            bind_mode: BindMode::Auto,
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
            bind_mode: BindMode::Auto,
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
            bind_mode: BindMode::Auto,
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
            bind_mode: BindMode::Auto,
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
            bind_mode: BindMode::Auto,
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

    #[test]
    fn resolve_server_socket_path_bind_mode_abstract_without_legacy_flag() {
        let args = ServerArgs {
            bind_mode: BindMode::Abstract,
            socket: "/tmp/ignored.sock".to_string(),
            r#abstract: false,
            port: None,
            listen: None,
            audit_dir: None,
            family_id: Some("gamma".to_string()),
            orchestrator_id: None,
        };
        let ns = resolve_biomeos_ipc_subdir_from_optional(None);
        assert_eq!(
            resolve_server_socket_path(&args),
            format!("@{ns}_beardog_gamma")
        );
    }

    #[test]
    fn resolve_server_socket_path_bind_mode_tcp_returns_empty() {
        let args = ServerArgs {
            bind_mode: BindMode::Tcp,
            socket: "/tmp/ignored.sock".to_string(),
            r#abstract: false,
            port: Some(9100),
            listen: None,
            audit_dir: None,
            family_id: None,
            orchestrator_id: None,
        };
        assert_eq!(resolve_server_socket_path(&args), "");
    }

    #[test]
    fn resolve_server_socket_path_bind_mode_filesystem_uses_explicit() {
        let args = ServerArgs {
            bind_mode: BindMode::Filesystem,
            socket: "/run/beardog.sock".to_string(),
            r#abstract: false,
            port: None,
            listen: None,
            audit_dir: None,
            family_id: None,
            orchestrator_id: None,
        };
        assert_eq!(resolve_server_socket_path(&args), "/run/beardog.sock");
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
