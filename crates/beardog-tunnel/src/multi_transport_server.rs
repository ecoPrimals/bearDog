// SPDX-License-Identifier: AGPL-3.0-only

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
//! ./beardog server --listen 127.0.0.1:9900     # TCP only
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

use crate::btsp_provider::BeardogBtspProvider;
use crate::tcp_ipc::TcpIpcServer;
use crate::unix_socket_ipc::UnixSocketIpcServer;
use beardog_errors::BearDogError;
use beardog_types::primal_identity::PrimalIdentity;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing::{error, info, warn};

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
/// Binds all available transports for maximum compatibility:
/// - **Tier 1**: Platform-native (Unix/Abstract/NamedPipe)
/// - **Tier 2**: TCP fallback (universal)
///
/// Clients automatically use best available transport for their platform.
pub struct MultiTransportServer {
    /// Successfully bound transports
    transports: Vec<BoundTransport>,

    /// Running transport tasks (use anyhow for compatibility)
    tasks: Vec<JoinHandle<anyhow::Result<()>>>,
}

impl MultiTransportServer {
    /// Bind all available transports
    ///
    /// **Deep Debt Principle #5**: Runtime discovery, not compile-time hardcoding
    ///
    /// Tries to bind:
    /// 1. Platform-native socket (Unix/Abstract/NamedPipe)
    /// 2. TCP fallback (always attempted)
    ///
    /// Returns error only if NO transports could be bound.
    pub async fn bind_all_available(
        btsp_provider: Arc<BeardogBtspProvider>,
        identity: Arc<PrimalIdentity>,
        socket_path: &str,
        tcp_addr: Option<&str>,
    ) -> Result<Self, BearDogError> {
        let mut transports = Vec::new();

        info!("🔌 Binding all available transports...");

        // ================================================================
        // TIER 1: Platform-Native Socket
        // ================================================================

        // Try native socket (Unix or Abstract)
        match UnixSocketIpcServer::new(socket_path, btsp_provider.clone(), identity.clone()).await {
            Ok(server) => {
                let transport_type = if socket_path.starts_with('@') {
                    "Abstract socket"
                } else {
                    "Unix socket"
                };
                info!(
                    "   ✅ Tier 1 (Native): {} bound: {}",
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

        // ================================================================
        // TIER 2: TCP Universal Fallback
        // ================================================================

        // Use config-based port discovery instead of hardcoded value
        let default_tcp_addr = {
            use beardog_config::global::BEARDOG_CONFIG;
            format!("127.0.0.1:{}", BEARDOG_CONFIG.network.ports.tcp_ipc_port)
        };
        let tcp_address = tcp_addr.unwrap_or(&default_tcp_addr);

        match tcp_address.parse::<SocketAddr>() {
            Ok(addr) => {
                let tcp_server = TcpIpcServer::new(addr, btsp_provider, identity);
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

        Ok(Self {
            transports,
            tasks: Vec::new(),
        })
    }

    /// Start all bound transports
    ///
    /// **Deep Debt Principle #3**: Fast AND Safe (async, no hand-written intrinsics)
    ///
    /// Spawns a task for each transport, runs them concurrently.
    /// If any transport fails, others continue running.
    pub async fn start_all(mut self) -> Result<(), BearDogError> {
        info!("🚀 Starting all transports...");

        for transport in self.transports {
            match transport {
                BoundTransport::Unix(server) => {
                    let server_clone = Arc::clone(&server);
                    let task = tokio::spawn(async move {
                        server_clone.start().await.map_err(|e| {
                            error!("Unix socket server error: {}", e);
                            anyhow::anyhow!("Unix server failed: {e}")
                        })
                    });
                    self.tasks.push(task);
                }
                BoundTransport::Tcp(server) => {
                    let server_clone = Arc::clone(&server);
                    let task = tokio::spawn(async move {
                        server_clone.start().await.map_err(|e| {
                            error!("TCP server error: {}", e);
                            anyhow::anyhow!("TCP server failed: {e}")
                        })
                    });
                    self.tasks.push(task);
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

        // Wait for ALL tasks (runs until Ctrl+C or error)
        // Use tokio's join_all which is always available
        use tokio::task::JoinSet;

        let mut join_set = JoinSet::new();
        for task in self.tasks {
            join_set.spawn(task);
        }

        // Wait for all tasks to complete
        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(Ok(()))) => {} // Task completed successfully
                Ok(Ok(Err(e))) => {
                    error!("Transport task failed: {}", e);
                }
                Ok(Err(e)) => {
                    error!("Transport task panicked: {}", e);
                }
                Err(e) => {
                    error!("Join error: {}", e);
                }
            }
        }

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
        hsm.register_hsm_provider(HsmTier::Software, Arc::new(software_hsm))
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
            sock.to_string_lossy().as_ref(),
            Some("127.0.0.1:0"),
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
            dir.path().to_str().expect("utf8"),
            Some("not-a-valid-socket-address:xyz"),
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
}
