// SPDX-License-Identifier: AGPL-3.0-or-later

//! Server mode handler - long-running service for Tower Atomic
//!
//! This handler wires the existing Unix socket IPC server from beardog-tunnel
//! into the CLI for proper UniBin operation.
//!
//! Supports two transport modes:
//! - **Tier 1**: Unix sockets (Linux, macOS) - preferred
//! - **Tier 2**: TCP (Android, Windows, cross-device) - universal fallback

mod acme;
mod attestation;
mod gateway;
mod health;
mod registration;
mod transport;

#[cfg(test)]
mod tests;

pub use transport::{resolve_effective_tcp_listen, resolve_server_socket_path};

use crate::ServerArgs;
use beardog_config::env_keys::{self, resolve_primal_name};
use beardog_errors::BearDogError;
use beardog_genetics::EcosystemGeneticEngine;
use beardog_ipc::{
    discover_neural_api_socket, register_with_neural_api, send_primal_announce,
};
use beardog_tunnel::primal_announce::registered_announce_method_names_for_identity;
use beardog_tunnel::btsp_handshake;
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::multi_transport_server::MultiTransportServer;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use beardog_tunnel::tunnel::hsm::software_hsm::RustSoftwareHsm;
use beardog_tunnel::tunnel::hsm::{HsmProviderBackend, HsmTier, SoftwareHsmConfig};
use std::sync::Arc;
use tracing::{info, warn};

use self::acme::start_acme_gateway;
use self::attestation::build_neural_attestation;
use self::health::run_health_socket;
use self::registration::attempt_orchestrator_registration;
use self::transport::neural_registration_address;

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

    let use_abstract = match args.bind_mode {
        crate::BindMode::Abstract => true,
        crate::BindMode::Auto => {
            args.r#abstract
                || cfg!(target_os = "android")
                || std::env::var("ANDROID_ROOT").is_ok()
                || std::env::var("ANDROID_DATA").is_ok()
        }
        _ => args.r#abstract,
    };
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
        identity.clone(),
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
        let announce_methods =
            registered_announce_method_names_for_identity(identity.clone()).await;
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

    // Gatehouse mode: bearDog owns :443 (TLS) + :80 (ACME challenges + HTTPS redirect).
    // Activated by BEARDOG_GATEHOUSE_MODE=true OR BEARDOG_TLS_MODE=acme.
    let gatehouse_active = std::env::var(env_keys::ENV_GATEHOUSE_MODE)
        .ok()
        .is_some_and(|v| v.eq_ignore_ascii_case("true") || v == "1")
        || std::env::var(env_keys::ENV_TLS_MODE)
            .ok()
            .is_some_and(|v| v.eq_ignore_ascii_case("acme"));

    if gatehouse_active {
        let gateway = start_acme_gateway().await?;
        let https_port = std::env::var(env_keys::ENV_HTTPS_PORT)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(443u16);
        tokio::spawn(gateway::serve_https_gateway(gateway.acceptor, https_port));
        info!(
            https_port,
            "GATEHOUSE active: :443 TLS gateway + :80 ACME/redirect → upstream peer"
        );
    }

    // Health socket: lightweight plaintext listener for monitoring probes.
    // Always spawn unless --bind-mode=tcp (no UDS available).
    let health_path = args.health_socket.clone().unwrap_or_else(|| {
        let primal_name = resolve_primal_name();
        let default_health = format!("{primal_name}-default.sock");
        let main = std::path::Path::new(&socket_path);
        if let Some(dir) = main.parent() {
            dir.join(&default_health)
                .to_string_lossy()
                .to_string()
        } else {
            format!("/tmp/{default_health}")
        }
    });
    if !tcp_only {
        let hp = health_path.clone();
        info!(path = %hp, "spawning plaintext health socket");
        tokio::spawn(async move {
            if let Err(e) = run_health_socket(&hp).await {
                warn!(error = %e, "health socket exited");
            }
        });
    }

    // Start all transports (runs until Ctrl+C)
    server.start_all().await?;

    Ok(())
}
