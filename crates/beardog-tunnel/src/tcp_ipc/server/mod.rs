// SPDX-License-Identifier: AGPL-3.0-or-later

//! TCP IPC Server for `BearDog`
//!
//! Provides JSON-RPC over TCP for universal platform support.

mod connection;

#[cfg(test)]
mod tests;

use crate::btsp_handshake::BtspSecurityMode;
use crate::btsp_provider::BeardogBtspProvider;
use crate::method_gate::{CallerContext, MethodGate};
use crate::tcp_ipc::rate_limiter::{ConnectionRateLimiter, RateLimitConfig};
use crate::unix_socket_ipc::handlers::HandlerRegistry;
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use beardog_types::primal_identity::PrimalIdentity;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio::time::Duration;
use tracing::{debug, error, info, warn};

pub(crate) static TCP_READ_TIMEOUT: std::sync::LazyLock<Duration> =
    std::sync::LazyLock::new(|| {
        Duration::from_secs(
            std::env::var(beardog_config::env_keys::ENV_READ_TIMEOUT_SECS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
        )
    });

pub(crate) static TCP_HANDSHAKE_DETECT_TIMEOUT: std::sync::LazyLock<Duration> =
    std::sync::LazyLock::new(|| {
        Duration::from_secs(
            std::env::var(beardog_config::env_keys::ENV_HANDSHAKE_TIMEOUT_SECS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
        )
    });

/// TCP IPC Server
///
/// Universal JSON-RPC server over TCP. Works on all platforms including
/// Android where Unix sockets may be restricted by `SELinux`.
pub struct TcpIpcServer {
    pub(crate) bind_addr: SocketAddr,
    pub(crate) btsp_provider: Arc<BeardogBtspProvider>,
    pub(crate) handler_registry: Arc<HandlerRegistry>,
    pub(crate) security_mode: BtspSecurityMode,
    pub(crate) method_gate: Arc<MethodGate>,
    pub(crate) bound_addr: Arc<RwLock<Option<SocketAddr>>>,
    pub(crate) rate_limiter: Arc<ConnectionRateLimiter>,
    #[cfg(feature = "tls-server")]
    pub(crate) tls_acceptor: Option<tokio_rustls::TlsAcceptor>,
}

impl TcpIpcServer {
    /// Create new TCP IPC server
    pub fn new(
        bind_addr: SocketAddr,
        btsp_provider: Arc<BeardogBtspProvider>,
        identity: Arc<PrimalIdentity>,
        security_mode: BtspSecurityMode,
    ) -> Self {
        let primal_name =
            std::env::var(env_keys::ENV_PRIMAL_NAME).unwrap_or_else(|_| "beardog".to_owned());
        let method_gate = Arc::new(MethodGate::from_env(&primal_name, identity.node_id()));
        info!(
            mode = method_gate.mode().as_str(),
            "TCP method gate initialized (JH-0/JH-1)"
        );

        let rate_limiter = Arc::new(ConnectionRateLimiter::new(RateLimitConfig::from_env()));
        info!("TCP rate limiter initialized (H2-11 sovereignty)");

        #[cfg(feature = "tls-server")]
        let tls_acceptor = {
            use crate::tcp_ipc::tls::{TlsTerminationConfig, build_tls_acceptor};
            if let Some(tls_config) = TlsTerminationConfig::from_env() {
                match build_tls_acceptor(&tls_config) {
                    Ok(acceptor) => {
                        info!("TLS termination enabled (H2-10 sovereignty)");
                        Some(acceptor)
                    }
                    Err(e) => {
                        warn!(error = %e, "TLS config present but failed to initialize — running without TLS");
                        None
                    }
                }
            } else {
                info!(
                    "TLS termination not configured (set BEARDOG_TLS_CERT_PATH + BEARDOG_TLS_KEY_PATH to enable)"
                );
                None
            }
        };

        Self {
            bind_addr,
            btsp_provider,
            handler_registry: HandlerRegistry::new(identity),
            security_mode,
            method_gate,
            bound_addr: Arc::new(RwLock::new(None)),
            rate_limiter,
            #[cfg(feature = "tls-server")]
            tls_acceptor,
        }
    }

    /// Get the actual bound address (after server starts)
    pub async fn get_bound_addr(&self) -> Option<SocketAddr> {
        *self.bound_addr.read().await
    }

    /// Start TCP server
    ///
    /// # Errors
    ///
    /// Returns an error if the TCP listener cannot be bound or the local address cannot be read.
    pub async fn start(&self) -> Result<(), BearDogError> {
        info!("🌐 Starting TCP IPC server: {}", self.bind_addr);

        let listener = TcpListener::bind(self.bind_addr)
            .await
            .map_err(|e| BearDogError::system(format!("Failed to bind TCP: {e}")))?;

        let bound_addr = listener
            .local_addr()
            .map_err(|e| BearDogError::system(format!("Failed to get local address: {e}")))?;

        *self.bound_addr.write().await = Some(bound_addr);

        info!("✅ TCP IPC server listening: {}", bound_addr);
        info!("   Protocol: JSON-RPC 2.0 over TCP");
        info!("   Platform: Universal (Android, Linux, Windows, iOS)");
        #[cfg(feature = "tls-server")]
        if self.tls_acceptor.is_some() {
            info!("   TLS: Enabled (X.509 termination, H2-10)");
        }
        info!("   Rate limiting: Enabled (H2-11)");

        let prune_limiter = self.rate_limiter.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300));
            loop {
                interval.tick().await;
                prune_limiter.prune_stale();
            }
        });

        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    if let Err(reason) = self.rate_limiter.check_connection(&peer_addr.ip()) {
                        warn!(peer = %peer_addr, reason = %reason, "connection rejected by rate limiter");
                        drop(stream);
                        continue;
                    }

                    self.rate_limiter.on_connect();
                    debug!("📥 New connection from: {}", peer_addr);

                    let registry = self.handler_registry.clone();
                    let btsp = self.btsp_provider.clone();
                    let sec_mode = self.security_mode.clone();
                    let gate = self.method_gate.clone();
                    let limiter = self.rate_limiter.clone();

                    #[cfg(feature = "tls-server")]
                    if let Some(ref acceptor) = self.tls_acceptor {
                        let acceptor = acceptor.clone();
                        tokio::spawn(async move {
                            match acceptor.accept(stream).await {
                                Ok(tls_stream) => {
                                    let (reader, writer) = tokio::io::split(tls_stream);
                                    if let Err(e) = Self::handle_plaintext_connection(
                                        reader,
                                        writer,
                                        peer_addr,
                                        registry,
                                        btsp,
                                        &gate,
                                        &mut CallerContext::remote(),
                                    )
                                    .await
                                    {
                                        error!("TLS connection handler error: {}", e);
                                    }
                                    limiter.on_disconnect();
                                }
                                Err(e) => {
                                    debug!(peer = %peer_addr, error = %e, "TLS handshake failed — falling through to cleartext");
                                    limiter.on_disconnect();
                                }
                            }
                        });
                        continue;
                    }

                    tokio::spawn(async move {
                        if let Err(e) =
                            Self::handle_connection(stream, registry, btsp, sec_mode, gate).await
                        {
                            error!("Connection handler error: {}", e);
                        }
                        limiter.on_disconnect();
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}
