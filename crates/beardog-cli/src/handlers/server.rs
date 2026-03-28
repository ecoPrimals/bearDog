// SPDX-License-Identifier: AGPL-3.0-only

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
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::multi_transport_server::MultiTransportServer;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use beardog_tunnel::tunnel::hsm::software_hsm::RustSoftwareHsm;
use beardog_tunnel::tunnel::hsm::{HsmTier, SoftwareHsmConfig};
use std::sync::Arc;
use tracing::{info, warn};

/// Resolve the effective socket path for server startup (abstract, multi-family, or explicit).
pub(crate) fn resolve_server_socket_path(args: &ServerArgs) -> String {
    if args.r#abstract {
        let family = args.family_id.as_deref().unwrap_or("default");
        format!("@biomeos_beardog_{family}")
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

/// Handle server command - start long-running service
pub async fn handle_server(args: ServerArgs) -> Result<(), BearDogError> {
    info!("BearDog server starting");

    // Determine socket path - use abstract socket if --abstract flag is set,
    // or derive family-scoped socket if --family-id is provided
    let socket_path = resolve_server_socket_path(&args);
    if args.r#abstract {
        info!(transport = "abstract", "transport selected");
        info!(socket_path = %socket_path, "abstract socket path");
    } else if args.family_id.is_some() {
        info!(socket_path = %socket_path, "multi-family socket");
    }

    // Resolve --port into --listen (UniBin v1.1: `server --port <PORT>`)
    let effective_listen = match (args.port, &args.listen) {
        (Some(port), None) => Some(format!("0.0.0.0:{port}")),
        (None, Some(addr)) => Some(addr.clone()),
        _ => None,
    };

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
    // PHASE 3: MULTI-TRANSPORT SERVER (Deep Debt Evolution)
    // ================================================================

    info!(platform = "universal", "creating multi-transport server");

    // Create multi-transport server (binds all available)
    let server = MultiTransportServer::bind_all_available(
        btsp_provider,
        identity,
        &socket_path,
        tcp_addr.as_deref(),
    )
    .await?;

    info!(
        transport_count = server.transport_count(),
        "multi-transport server created"
    );

    // Auto-register with Neural API if available (Tower Atomic TRUE PRIMAL)
    if let Some(neural_socket) = discover_neural_api_socket() {
        info!(neural_socket = %neural_socket, "Neural API detected");

        let registration_addr = if let Some(ref tcp) = tcp_addr {
            tcp.as_str()
        } else {
            &socket_path
        };

        match register_with_neural_api(&neural_socket, &primal_name, registration_addr).await {
            Ok(()) => info!("registered with Neural API"),
            Err(e) => warn!(error = %e, "Neural API registration failed (non-fatal)"),
        }
    } else {
        info!(mode = "standalone", "no Neural API detected");
    }

    // Best-effort orchestrator registry registration (non-fatal per PRIMAL IPC Protocol v3.1)
    attempt_songbird_registration(&socket_path, tcp_addr.as_deref()).await;

    // Start all transports (runs until Ctrl+C)
    server.start_all().await?;

    Ok(())
}

/// Best-effort registration with ecosystem IPC registry (non-fatal).
///
/// Attempts to connect to the orchestrator's IPC registry socket
/// (Songbird or equivalent) and register BearDog's capabilities.
/// Failure is logged and swallowed per PRIMAL IPC Protocol v3.1:
/// registration SHOULD be attempted but MUST NOT prevent standalone operation.
async fn attempt_songbird_registration(_socket_path: &str, _tcp_addr: Option<&str>) {
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
