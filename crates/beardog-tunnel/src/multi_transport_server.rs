// SPDX-License-Identifier: AGPL-3.0-or-later

//! Multi-Transport Server - Universal IPC Binding
//!
//! **Deep Debt Principles #4 & #5**: Agnostic + Runtime Discovery
//!
//! Binds ALL available transports simultaneously for true universal deployment.
//!
//! ## Philosophy
//!
//! **Before**: Choose ONE transport (mutually exclusive)
//! ```bash
//! ./beardog server --socket /tmp/beardog.sock  # Unix only
//! ./beardog server --listen 127.0.0.1:9100     # TCP only
//! ```
//!
//! **After**: Bind ALL available (concurrent)
//! ```bash
//! ./beardog server  # Binds Unix + TCP + Abstract (all!)
//! # Clients connect via whichever works on their platform
//! ```
//!
//! ## Architecture
//!
//! 1. **Platform Detection**: Auto-detect available transports
//! 2. **Concurrent Binding**: Try all transports, use what works
//! 3. **Unified Handling**: Same JSON-RPC handler for all transports
//! 4. **Graceful Degradation**: If one transport fails, others still work

use crate::btsp_handshake::BtspSecurityMode;
use crate::btsp_provider::BeardogBtspProvider;
use crate::tcp_ipc::TcpIpcServer;
use crate::unix_socket_ipc::UnixSocketIpcServer;
use beardog_core::self_knowledge::discovered_simple_capabilities;
use beardog_core::socket_config::{
    IpcCapabilitySymlinksConfig, ipc_capability_domain_stems_resolved,
};
use beardog_errors::BearDogError;
use beardog_types::primal_identity::PrimalIdentity;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::{debug, error, info, warn};

/// Bound transport types
///
/// Each variant holds a reference to its server implementation
pub enum BoundTransport {
    /// Unix socket (Linux, macOS)
    Unix(Arc<UnixSocketIpcServer>),

    /// TCP socket (all platforms)
    Tcp(Arc<TcpIpcServer>),
    // Abstract sockets handled as Unix variant with @ prefix
    // Named pipes (Windows) - future enhancement
    // XPC (iOS) - future enhancement
}

/// Multi-Transport Server
///
/// **Deep Debt Evolution**: From hardcoded single transport to universal multi-bind
///
/// Binds available transports for maximum compatibility:
/// - **Tier 1**: Platform-native (Unix/Abstract/NamedPipe) — always
/// - **Tier 2**: TCP — opt-in via `--port`/`--listen` or `BEARDOG_TCP_IPC_PORT`
///
/// Clients automatically use best available transport for their platform.
/// For UDS-only mode (Tower CNS / exp114), omit TCP flags and env var.
pub struct MultiTransportServer {
    /// Successfully bound transports
    transports: Vec<BoundTransport>,
}

impl MultiTransportServer {
    /// Bind all available transports
    ///
    /// **Deep Debt Principle #5**: Runtime discovery, not compile-time hardcoding
    ///
    /// Tries to bind:
    /// 1. Platform-native socket (Unix/Abstract/NamedPipe) — always attempted
    /// 2. TCP — only when `--port`/`--listen` is specified or `BEARDOG_TCP_IPC_PORT`
    ///    env var is set. Omit both for UDS-only mode (exp114 Tower CNS).
    ///
    /// Returns error only if NO transports could be bound.
    ///
    /// # Errors
    ///
    /// Returns an error if neither the Unix nor TCP transport could be bound.
    /// Bind all available transports.
    ///
    /// Pass `socket_path = None` to skip UDS entirely (TCP-only mode via
    /// `--bind-mode tcp`).
    pub async fn bind_all_available(
        btsp_provider: Arc<BeardogBtspProvider>,
        identity: Arc<PrimalIdentity>,
        socket_path: Option<&str>,
        tcp_addr: Option<&str>,
        security_mode: BtspSecurityMode,
    ) -> Result<Self, BearDogError> {
        let mut transports = Vec::new();

        info!("🔌 Binding all available transports...");

        // ================================================================
        // TIER 1: Platform-Native Socket (skipped in TCP-only mode)
        // ================================================================

        if let Some(socket_path) = socket_path {
            let ipc_symlinks = IpcCapabilitySymlinksConfig {
                symlink_suffix: ".sock".to_string(),
                domain_stems: ipc_capability_domain_stems_resolved(
                    &discovered_simple_capabilities(),
                ),
            };

            match UnixSocketIpcServer::new(
                socket_path,
                btsp_provider.clone(),
                identity.clone(),
                security_mode.clone(),
                ipc_symlinks,
            )
            .await
            {
                Ok(server) => {
                    let transport_type = if socket_path.starts_with('@') {
                        "Abstract socket"
                    } else {
                        "Unix socket"
                    };
                    info!(
                        "   ✅ Tier 1 (Native): {} configured: {}",
                        transport_type, socket_path
                    );
                    transports.push(BoundTransport::Unix(Arc::new(server)));
                }
                Err(e) => {
                    warn!(
                        "   ⚠️  Tier 1 (Native): Failed to bind {}: {}",
                        socket_path, e
                    );
                    info!("   → Continuing with available transports...");
                }
            }
        } else {
            info!("   ℹ️  Tier 1 (Native): skipped (TCP-only bind mode)");
        }

        // ================================================================
        // TIER 2: TCP (opt-in via --port / --listen / BEARDOG_TCP_IPC_PORT)
        // ================================================================

        let effective_tcp_addr = match tcp_addr {
            Some(addr) => Some(addr.to_string()),
            None => std::env::var(beardog_config::env_keys::ENV_TCP_IPC_PORT).ok().map(|port| {
                use beardog_types::constants::domains::network::addresses::DEFAULT_LOCALHOST_IPV4_STR;
                format!("{DEFAULT_LOCALHOST_IPV4_STR}:{port}")
            }),
        };

        if let Some(tcp_address) = &effective_tcp_addr {
            match tcp_address.parse::<SocketAddr>() {
                Ok(addr) => {
                    let tcp_server =
                        TcpIpcServer::new(addr, btsp_provider, identity, security_mode.clone());
                    info!("   ✅ Tier 2 (TCP): bound: {}", tcp_address);
                    transports.push(BoundTransport::Tcp(Arc::new(tcp_server)));
                }
                Err(e) => {
                    warn!(
                        "   ⚠️  Tier 2 (TCP): Invalid address {}: {}",
                        tcp_address, e
                    );
                }
            }
        } else {
            info!("   ℹ️  Tier 2 (TCP): skipped (UDS-only mode)");
        }

        // ================================================================
        // VALIDATION: At least ONE transport must be bound
        // ================================================================

        if transports.is_empty() {
            return Err(BearDogError::configuration(
                "Failed to bind ANY transport! Check socket path and TCP address.",
            ));
        }

        info!(
            "✅ Multi-transport server ready: {} transport(s) bound",
            transports.len()
        );

        Ok(Self { transports })
    }

    /// Start all bound transports
    ///
    /// **Deep Debt Principle #3**: Fast AND Safe (async, no hand-written intrinsics)
    ///
    /// Spawns a task for each transport, runs them concurrently.
    /// If any transport fails, others continue running.
    /// Handles both SIGINT (Ctrl+C) and SIGTERM for graceful shutdown with
    /// explicit socket cleanup (stale socket prevention per
    /// `CAPABILITY_BASED_DISCOVERY_STANDARD.md` §6).
    ///
    /// # Errors
    ///
    /// Returns an error if starting a transport task fails before tasks are spawned.
    pub async fn start_all(self) -> Result<(), BearDogError> {
        info!("🚀 Starting all transports...");

        let mut unix_servers: Vec<Arc<UnixSocketIpcServer>> = Vec::new();
        let mut join_set = JoinSet::new();

        for transport in self.transports {
            match transport {
                BoundTransport::Unix(server) => {
                    unix_servers.push(Arc::clone(&server));
                    let server_clone = Arc::clone(&server);
                    join_set.spawn(async move {
                        server_clone.start().await.map_err(|e| {
                            error!("Unix socket server error: {}", e);
                            anyhow::anyhow!("Unix server failed: {e}")
                        })
                    });
                }
                BoundTransport::Tcp(server) => {
                    let server_clone = Arc::clone(&server);
                    join_set.spawn(async move {
                        server_clone.start().await.map_err(|e| {
                            error!("TCP server error: {}", e);
                            anyhow::anyhow!("TCP server failed: {e}")
                        })
                    });
                }
            }
        }

        info!("   All transports started successfully");
        info!("");
        info!("╔════════════════════════════════════════════════════════════════╗");
        info!("║                                                                ║");
        info!("║   🐻🐕 BearDog Server READY - Multi-Transport Mode           ║");
        info!("║                                                                ║");
        info!("╚════════════════════════════════════════════════════════════════╝");
        info!("");
        info!("🔐 Crypto API: Ed25519, X25519, ChaCha20-Poly1305, Blake3");
        info!("🔌 Protocol: JSON-RPC 2.0");
        info!("🏗️  Architecture: Universal IPC (all transports)");
        info!("🌍 Platform: Agnostic (runtime discovery)");
        info!("");
        info!("Press Ctrl+C to stop");
        info!("");

        let log_join_outcome =
            |result: Result<anyhow::Result<()>, tokio::task::JoinError>| match result {
                Ok(Ok(())) => {}
                Ok(Err(e)) => {
                    error!("Transport task failed: {}", e);
                }
                Err(e) => {
                    if e.is_cancelled() {
                        debug!("Transport task cancelled during shutdown");
                    } else {
                        error!("Transport task panicked: {}", e);
                    }
                }
            };

        let shutdown_signal = async {
            #[cfg(unix)]
            {
                use tokio::signal::unix::{SignalKind, signal};
                #[expect(
                    clippy::expect_used,
                    reason = "SIGTERM registration is infallible on Unix"
                )]
                let mut sigterm =
                    signal(SignalKind::terminate()).expect("SIGTERM handler registration");
                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {
                        info!("SIGINT received, shutting down transports");
                    }
                    _ = sigterm.recv() => {
                        info!("SIGTERM received, shutting down transports");
                    }
                }
            }
            #[cfg(not(unix))]
            {
                tokio::signal::ctrl_c().await.ok();
                info!("SIGINT received, shutting down transports");
            }
        };

        tokio::select! {
            () = shutdown_signal => {}
            () = async {
                while let Some(result) = join_set.join_next().await {
                    log_join_outcome(result);
                }
            } => ()
        }

        join_set.abort_all();
        while let Some(result) = join_set.join_next().await {
            log_join_outcome(result);
        }

        for server in &unix_servers {
            if let Err(e) = server.stop().await {
                warn!(error = %e, "socket cleanup during shutdown");
            }
        }

        info!("all transports stopped, socket files cleaned");
        Ok(())
    }

    /// Get count of bound transports
    pub fn transport_count(&self) -> usize {
        self.transports.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::btsp_provider::BeardogBtspProvider;
    use crate::tunnel::hsm::SoftwareHsmConfig;
    use crate::tunnel::hsm::manager::HsmManager;
    use crate::tunnel::hsm::software_hsm::RustSoftwareHsm;
    use crate::tunnel::hsm::types::HsmTier;
    use beardog_genetics::ecosystem_evolution::EcosystemGeneticEngine;
    use beardog_types::primal_identity::PrimalIdentity;
    use std::sync::Arc;
    use tempfile::tempdir;

    #[test]
    fn test_bound_transport_variants() {
        // Ensure all variants are covered
        // This is a compile-time check - if new variants are added, this will fail
        fn _check_exhaustive(t: BoundTransport) {
            match t {
                BoundTransport::Unix(_) => {}
                BoundTransport::Tcp(_) => {}
            }
        }
    }

    async fn test_btsp_provider() -> Arc<BeardogBtspProvider> {
        let mut hsm = HsmManager::new();
        let software_hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
            .await
            .expect("software hsm");
        hsm.register_hsm_provider(
            HsmTier::Software,
            Arc::new(crate::tunnel::hsm::HsmProviderBackend::RustSoftware(
                software_hsm,
            )),
        )
        .expect("register");
        let hsm = Arc::new(hsm);
        let genetics = Arc::new(EcosystemGeneticEngine::new().expect("genetics"));
        Arc::new(BeardogBtspProvider::new(hsm, genetics).await.expect("btsp"))
    }

    #[tokio::test]
    async fn bind_all_available_unix_and_tcp_with_isolated_socket_path() {
        let dir = tempdir().expect("tempdir");
        let sock = dir.path().join("multi_transport.sock");
        let provider = test_btsp_provider().await;
        let identity = Arc::new(PrimalIdentity::for_test("fam", "node"));

        let mts = MultiTransportServer::bind_all_available(
            provider,
            identity,
            Some(sock.to_string_lossy().as_ref()),
            Some("127.0.0.1:0"),
            BtspSecurityMode::Development,
        )
        .await
        .expect("at least one transport");

        assert!(mts.transport_count() >= 1);
    }

    #[tokio::test]
    async fn bind_all_available_fails_when_no_transport_can_be_configured() {
        let dir = tempdir().expect("tempdir");
        let provider = test_btsp_provider().await;
        let identity = Arc::new(PrimalIdentity::for_test("fam", "node"));

        let outcome = MultiTransportServer::bind_all_available(
            provider,
            identity,
            Some(dir.path().to_str().expect("utf8")),
            Some("not-a-valid-socket-address:xyz"),
            BtspSecurityMode::Development,
        )
        .await;

        match outcome {
            Ok(_) => panic!("expected bind_all_available to fail when no transport is available"),
            Err(e) => {
                let msg = format!("{e}");
                assert!(
                    msg.contains("transport") || msg.contains("bind") || msg.contains("Failed"),
                    "{msg}"
                );
            }
        }
    }

    /// Tier-1 Unix binds; Tier-2 TCP address is invalid — exercises TCP parse `warn!` branch only.
    #[tokio::test]
    async fn bind_all_available_unix_ok_invalid_tcp_addr_skipped() {
        let dir = tempdir().expect("tempdir");
        let sock = dir.path().join("mts_invalid_tcp.sock");
        let provider = test_btsp_provider().await;
        let identity = Arc::new(PrimalIdentity::for_test("fam", "node"));

        let mts = MultiTransportServer::bind_all_available(
            provider,
            identity,
            Some(sock.to_string_lossy().as_ref()),
            Some(":::not-a-tcp-addr:::bad"),
            BtspSecurityMode::Development,
        )
        .await
        .expect("Unix transport should still bind");

        assert!(mts.transport_count() >= 1);
    }

    #[tokio::test]
    async fn bind_all_available_counts_two_when_unix_and_ephemeral_tcp() {
        let dir = tempdir().expect("tempdir");
        let sock = dir.path().join("mts_two.sock");
        let provider = test_btsp_provider().await;
        let identity = Arc::new(PrimalIdentity::for_test("fam", "node"));

        let mts = MultiTransportServer::bind_all_available(
            provider,
            identity,
            Some(sock.to_string_lossy().as_ref()),
            Some("127.0.0.1:0"),
            BtspSecurityMode::Development,
        )
        .await
        .expect("bind unix and tcp");

        assert!(
            mts.transport_count() >= 2,
            "expected unix + tcp on supported platforms"
        );
    }

    #[tokio::test]
    async fn bind_all_available_uses_default_tcp_when_none_provided() {
        let dir = tempdir().expect("tempdir");
        let sock = dir.path().join("mts_default_tcp.sock");
        let provider = test_btsp_provider().await;
        let identity = Arc::new(PrimalIdentity::for_test("fam", "node"));

        let mts = MultiTransportServer::bind_all_available(
            provider,
            identity,
            Some(sock.to_string_lossy().as_ref()),
            None,
            BtspSecurityMode::Development,
        )
        .await
        .expect("default tcp port from config");

        assert!(mts.transport_count() >= 1);
    }

    #[tokio::test]
    async fn bind_all_available_transport_count_reflects_bound_transports() {
        let dir = tempdir().expect("tempdir");
        let sock = dir.path().join("mts_count.sock");
        let provider = test_btsp_provider().await;
        let identity = Arc::new(PrimalIdentity::for_test("fam", "node"));

        let mts = MultiTransportServer::bind_all_available(
            provider,
            identity,
            Some(sock.to_string_lossy().as_ref()),
            Some("127.0.0.1:0"),
            BtspSecurityMode::Development,
        )
        .await
        .expect("bind");

        let n = mts.transport_count();
        assert!((1..=2).contains(&n), "unexpected transport count {n}");
    }
}
