// SPDX-License-Identifier: AGPL-3.0-or-later

//! TCP IPC Server for `BearDog`
//!
//! Provides JSON-RPC over TCP for universal platform support.

use crate::btsp_handshake::BtspSecurityMode;
use crate::btsp_provider::BeardogBtspProvider;
use crate::method_gate::{CallerContext, MethodGate};
use crate::tcp_ipc::rate_limiter::{ConnectionRateLimiter, RateLimitConfig};
use crate::unix_socket_ipc::handlers::HandlerRegistry;
use beardog_errors::BearDogError;
use beardog_types::primal_identity::PrimalIdentity;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio::time::Duration;
use tracing::{debug, error, info, warn};

/// Per-read timeout for TCP NDJSON connections.
///
/// Prevents indefinite blocking when a client connects but never sends a
/// newline (common with raw `nc` or `curl` probes). On timeout the connection
/// is closed and the task freed. Value chosen to be generous for legitimate
/// clients while still bounding resource usage.
pub(super) static TCP_READ_TIMEOUT: std::sync::LazyLock<Duration> = std::sync::LazyLock::new(|| {
    Duration::from_secs(
        std::env::var("BEARDOG_TCP_READ_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30),
    )
});

mod btsp;
mod connection;

/// TCP IPC Server
///
/// Universal JSON-RPC server over TCP. Works on all platforms including
/// Android where Unix sockets may be restricted by `SELinux`.
pub struct TcpIpcServer {
    /// Bind address
    bind_addr: SocketAddr,

    /// BTSP provider (provides crypto/genetics capabilities)
    btsp_provider: Arc<BeardogBtspProvider>,

    /// Handler registry (shared with Unix socket implementation)
    handler_registry: Arc<HandlerRegistry>,

    /// BTSP security mode (resolved at startup, checked per connection).
    security_mode: BtspSecurityMode,

    /// Pre-dispatch authorization gate (JH-0 ecosystem standard).
    method_gate: Arc<MethodGate>,

    /// Actual bound address (after OS assigns port if using :0)
    bound_addr: Arc<RwLock<Option<SocketAddr>>>,

    /// Per-IP connection rate limiter (H2-11 sovereignty).
    rate_limiter: Arc<ConnectionRateLimiter>,

    /// TLS acceptor for X.509 termination (H2-10 sovereignty).
    #[cfg(feature = "tls-server")]
    tls_acceptor: Option<tokio_rustls::TlsAcceptor>,
}

impl TcpIpcServer {
    /// Create new TCP IPC server
    pub fn new(
        bind_addr: SocketAddr,
        btsp_provider: Arc<BeardogBtspProvider>,
        identity: Arc<PrimalIdentity>,
        security_mode: BtspSecurityMode,
    ) -> Self {
        let primal_name = std::env::var("PRIMAL_NAME").unwrap_or_else(|_| "beardog".to_owned());
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

        // Bind TCP listener
        let listener = TcpListener::bind(self.bind_addr)
            .await
            .map_err(|e| BearDogError::system(format!("Failed to bind TCP: {e}")))?;

        // Get actual bound address (important if port was 0)
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

        // Periodic rate-limiter pruning (every 5 minutes)
        let prune_limiter = self.rate_limiter.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300));
            loop {
                interval.tick().await;
                prune_limiter.prune_stale();
            }
        });

        // Accept connections loop
        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    // H2-11: rate limit check before any processing
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

                    // H2-10: TLS upgrade if acceptor is configured
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::method_gate::EnforcementMode;
    use crate::test_helpers::mocks::create_minimal_beardog_provider;
    use crate::unix_socket_ipc::handlers::HandlerRegistry;
    use beardog_types::primal_identity::PrimalIdentity;
    use std::sync::Arc;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::TcpListener;

    fn test_gate() -> Arc<MethodGate> {
        Arc::new(MethodGate::new(
            EnforcementMode::Permissive,
            "beardog",
            "test-node",
        ))
    }

    #[tokio::test]
    async fn tcp_ipc_health_ping_roundtrip() {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind TCP listener for health ping test");
        let addr = listener
            .local_addr()
            .expect("local_addr after bind for health ping test");
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let registry = HandlerRegistry::new(identity);
        let provider = create_minimal_beardog_provider().await;

        let gate = test_gate();
        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept health ping client");
            TcpIpcServer::handle_connection(
                stream,
                registry,
                provider,
                BtspSecurityMode::Development,
                gate,
            )
            .await
        });

        let mut client = tokio::net::TcpStream::connect(addr)
            .await
            .expect("connect health ping client");
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "ping",
            "id": 1
        });
        let line = serde_json::to_string(&request).expect("serialize ping request") + "\n";
        client
            .write_all(line.as_bytes())
            .await
            .expect("write ping request");

        let mut reader = BufReader::new(client);
        let mut buf = String::new();
        reader
            .read_line(&mut buf)
            .await
            .expect("read ping response line");
        let resp: serde_json::Value =
            serde_json::from_str(buf.trim()).expect("parse ping JSON-RPC response");
        assert_eq!(resp["jsonrpc"], "2.0");
        assert_eq!(resp["id"], 1);
        assert!(resp.get("result").is_some());

        drop(reader);
        server.abort();
    }

    #[tokio::test]
    async fn tcp_ipc_invalid_json_returns_parse_error() {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind TCP listener for invalid JSON test");
        let addr = listener
            .local_addr()
            .expect("local_addr after bind for invalid JSON test");
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let registry = HandlerRegistry::new(identity);
        let provider = create_minimal_beardog_provider().await;

        let gate = test_gate();
        let server = tokio::spawn(async move {
            let (stream, _) = listener
                .accept()
                .await
                .expect("accept invalid JSON test client");
            TcpIpcServer::handle_connection(
                stream,
                registry,
                provider,
                BtspSecurityMode::Development,
                gate,
            )
            .await
        });

        let mut client = tokio::net::TcpStream::connect(addr)
            .await
            .expect("connect invalid JSON test client");
        client
            .write_all(b"not json at all\n")
            .await
            .expect("write invalid payload");

        let mut reader = BufReader::new(client);
        let mut buf = String::new();
        reader
            .read_line(&mut buf)
            .await
            .expect("read parse-error response line");
        let resp: serde_json::Value =
            serde_json::from_str(buf.trim()).expect("parse parse-error JSON-RPC response");
        assert_eq!(resp["jsonrpc"], "2.0");
        assert_eq!(resp["error"]["code"], -32700);

        drop(reader);
        server.abort();
    }

    #[tokio::test]
    async fn tcp_ipc_unknown_method_returns_error() {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind TCP listener for unknown method test");
        let addr = listener
            .local_addr()
            .expect("local_addr after bind for unknown method test");
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let registry = HandlerRegistry::new(identity);
        let provider = create_minimal_beardog_provider().await;

        let gate = test_gate();
        let server = tokio::spawn(async move {
            let (stream, _) = listener
                .accept()
                .await
                .expect("accept unknown method test client");
            TcpIpcServer::handle_connection(
                stream,
                registry,
                provider,
                BtspSecurityMode::Development,
                gate,
            )
            .await
        });

        let mut client = tokio::net::TcpStream::connect(addr)
            .await
            .expect("connect unknown method test client");
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "definitely.not.a.registered.method",
            "id": 42
        });
        let req_line =
            serde_json::to_string(&request).expect("serialize unknown-method request") + "\n";
        client
            .write_all(req_line.as_bytes())
            .await
            .expect("write unknown-method request");

        let mut reader = BufReader::new(client);
        let mut buf = String::new();
        reader
            .read_line(&mut buf)
            .await
            .expect("read unknown-method error line");
        let resp: serde_json::Value =
            serde_json::from_str(buf.trim()).expect("parse unknown-method JSON-RPC response");
        assert_eq!(resp["jsonrpc"], "2.0");
        assert_eq!(resp["id"], 42);
        assert_eq!(resp["error"]["code"], -32601);

        drop(reader);
        server.abort();
    }

    #[tokio::test]
    async fn tcp_ipc_server_new_bound_addr_starts_none() {
        let identity = Arc::new(PrimalIdentity::for_test("fam", "node"));
        let provider = create_minimal_beardog_provider().await;
        let addr: std::net::SocketAddr =
            "127.0.0.1:0".parse().expect("parse loopback :0 SocketAddr");
        let server = TcpIpcServer::new(addr, provider, identity, BtspSecurityMode::Development);
        assert!(server.get_bound_addr().await.is_none());
    }
}
