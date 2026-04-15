// SPDX-License-Identifier: AGPL-3.0-or-later

//! TCP IPC Server for `BearDog`
//!
//! Provides JSON-RPC over TCP for universal platform support.

use crate::btsp_handshake::{self, BtspSecurityMode, BtspSession};
use crate::btsp_provider::BeardogBtspProvider;
use crate::unix_socket_ipc::handlers::HandlerRegistry;
use beardog_errors::BearDogError;
use beardog_types::primal_identity::PrimalIdentity;
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio::time::Duration;
use tracing::{debug, error, info, warn};

/// Per-read timeout for TCP NDJSON connections.
///
/// Prevents indefinite blocking when a client connects but never sends a
/// newline (common with raw `nc` or `curl` probes). On timeout the connection
/// is closed and the task freed. Value chosen to be generous for legitimate
/// clients while still bounding resource usage.
const TCP_READ_TIMEOUT: Duration = Duration::from_secs(30);

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

    /// Actual bound address (after OS assigns port if using :0)
    bound_addr: Arc<RwLock<Option<SocketAddr>>>,
}

impl TcpIpcServer {
    /// Create new TCP IPC server
    pub fn new(
        bind_addr: SocketAddr,
        btsp_provider: Arc<BeardogBtspProvider>,
        identity: Arc<PrimalIdentity>,
        security_mode: BtspSecurityMode,
    ) -> Self {
        Self {
            bind_addr,
            btsp_provider,
            handler_registry: HandlerRegistry::new(identity),
            security_mode,
            bound_addr: Arc::new(RwLock::new(None)),
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

        // Accept connections loop
        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    debug!("📥 New connection from: {}", peer_addr);

                    let registry = self.handler_registry.clone();
                    let btsp = self.btsp_provider.clone();
                    let sec_mode = self.security_mode.clone();

                    tokio::spawn(async move {
                        if let Err(e) =
                            Self::handle_connection(stream, registry, btsp, sec_mode).await
                        {
                            error!("Connection handler error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    /// Handle a single TCP connection
    async fn handle_connection(
        mut stream: TcpStream,
        registry: Arc<HandlerRegistry>,
        btsp_provider: Arc<BeardogBtspProvider>,
        security_mode: BtspSecurityMode,
    ) -> Result<(), BearDogError> {
        let peer_addr = stream
            .peer_addr()
            .map_err(|e| BearDogError::system(format!("Failed to get peer address: {e}")))?;

        debug!("🔌 Handling connection from: {}", peer_addr);

        // ── BTSP production mode with protocol auto-detection ──────────
        //
        // Peek the first byte to distinguish BTSP binary framing from plain
        // JSON-RPC text. BTSP frames start with a 4-byte big-endian length;
        // JSON-RPC starts with '{' (0x7B). This allows biomeOS (the local
        // composition substrate) to forward capability.call via plain JSON-RPC
        // over TCP without requiring BTSP client implementation, while external
        // connections still get full BTSP enforcement.
        if let BtspSecurityMode::Production { ref family_seed } = security_mode {
            let mut peek_buf = [0u8; 1];
            match tokio::time::timeout(
                std::time::Duration::from_secs(5),
                stream.peek(&mut peek_buf),
            )
            .await
            {
                Ok(Ok(1)) if peek_buf[0] == b'{' => {
                    debug!(
                        peer = %peer_addr,
                        "TCP peek: JSON-RPC detected (0x7B) — bypassing BTSP for local composition"
                    );
                    // Fall through to plain NDJSON handler below
                }
                _ => {
                    debug!(peer = %peer_addr, "BTSP production: initiating TCP handshake");
                    match btsp_handshake::perform_server_handshake(&mut stream, family_seed).await {
                        Ok(mut session) => {
                            info!(
                                peer = %peer_addr,
                                session_id = %session.session_id,
                                cipher = %session.cipher.wire_name(),
                                "BTSP TCP handshake succeeded"
                            );
                            return Self::handle_jsonrpc_btsp_tcp(
                                &mut stream,
                                &mut session,
                                &registry,
                                &btsp_provider,
                            )
                            .await;
                        }
                        Err(e) => {
                            warn!(peer = %peer_addr, error = %e, "BTSP TCP handshake failed");
                            return Ok(());
                        }
                    }
                }
            }
        }

        // ── Plain NDJSON (development mode or JSON-RPC auto-detected) ──
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            line.clear();

            let read_result =
                tokio::time::timeout(TCP_READ_TIMEOUT, reader.read_line(&mut line)).await;

            let bytes_read = match read_result {
                Err(_elapsed) => {
                    warn!(
                        peer = %peer_addr,
                        timeout_secs = TCP_READ_TIMEOUT.as_secs(),
                        "TCP read timed out — closing idle connection"
                    );
                    break;
                }
                Ok(Err(e)) => {
                    warn!("Read error from {}: {}", peer_addr, e);
                    break;
                }
                Ok(Ok(n)) => n,
            };

            match bytes_read {
                0 => {
                    debug!("Connection closed by peer: {}", peer_addr);
                    break;
                }
                n => {
                    debug!("📨 Received {} bytes from {}", n, peer_addr);

                    let request_str = line.trim();
                    if request_str.is_empty() {
                        continue;
                    }

                    // Parse JSON-RPC request
                    let request: Value = match serde_json::from_str(request_str) {
                        Ok(req) => req,
                        Err(e) => {
                            warn!("Invalid JSON from {}: {}", peer_addr, e);
                            let error_response = serde_json::json!({
                                "jsonrpc": "2.0",
                                "error": {
                                    "code": -32700,
                                    "message": format!("Parse error: {}", e)
                                },
                                "id": null
                            });
                            // Serialization of json! macro is infallible, but handle gracefully
                            if let Ok(response) = serde_json::to_string(&error_response) {
                                let response = response + "\n";
                                if let Err(e) = writer.write_all(response.as_bytes()).await {
                                    warn!("Failed to write error response to client: {}", e);
                                }
                            }
                            continue;
                        }
                    };

                    // Extract method and params
                    let method = request["method"].as_str().unwrap_or("");
                    let params = request.get("params").cloned();
                    let id = request.get("id").cloned();

                    debug!("📥 Request: {} (id: {:?})", method, id);

                    // Route through handler registry
                    let response = registry
                        .route(method, params.as_ref(), &btsp_provider)
                        .await;

                    // Build JSON-RPC response
                    let json_response = match response {
                        Ok(result) => serde_json::json!({
                            "jsonrpc": "2.0",
                            "result": result,
                            "id": id
                        }),
                        Err(e) => serde_json::json!({
                            "jsonrpc": "2.0",
                            "error": {
                                "code": -32601,
                                "message": format!("Business error: {}", e)
                            },
                            "id": id
                        }),
                    };

                    // Send response (serialization of json! macro is infallible, but handle gracefully)
                    let response_str = match serde_json::to_string(&json_response) {
                        Ok(s) => s + "\n",
                        Err(e) => {
                            error!("Failed to serialize response: {}", e);
                            continue;
                        }
                    };
                    if let Err(e) = writer.write_all(response_str.as_bytes()).await {
                        error!("Failed to write response to {}: {}", peer_addr, e);
                        break;
                    }

                    debug!("📤 Response sent to {}", peer_addr);
                }
            }
        }

        debug!("✅ Connection handler finished: {}", peer_addr);
        Ok(())
    }

    /// Handle JSON-RPC over BTSP encrypted frames on a TCP stream.
    async fn handle_jsonrpc_btsp_tcp(
        stream: &mut TcpStream,
        session: &mut BtspSession,
        registry: &Arc<HandlerRegistry>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<(), BearDogError> {
        loop {
            let frame = match btsp_handshake::read_frame(stream).await {
                Ok(f) => f,
                Err(e) => {
                    debug!(error = %e, "BTSP TCP frame read ended");
                    return Ok(());
                }
            };

            let plaintext = match session.decrypt_frame(&frame) {
                Ok(p) => p,
                Err(e) => {
                    warn!(error = %e, "BTSP TCP frame decrypt failed");
                    return Ok(());
                }
            };

            let request_str = match String::from_utf8(plaintext) {
                Ok(s) => s,
                Err(e) => {
                    warn!(error = %e, "BTSP TCP frame not valid UTF-8");
                    continue;
                }
            };

            let trimmed = request_str.trim();
            if trimmed.is_empty() {
                continue;
            }

            let request: Value = match serde_json::from_str(trimmed) {
                Ok(req) => req,
                Err(e) => {
                    let err_resp = serde_json::json!({
                        "jsonrpc": "2.0",
                        "error": {"code": -32700, "message": format!("Parse error: {e}")},
                        "id": null
                    });
                    if let Ok(s) = serde_json::to_string(&err_resp) {
                        let encrypted = session
                            .encrypt_frame(s.as_bytes())
                            .map_err(|e| BearDogError::system(format!("BTSP encrypt: {e}")))?;
                        let _ = btsp_handshake::write_frame(stream, &encrypted).await;
                    }
                    continue;
                }
            };

            let method = request["method"].as_str().unwrap_or("");
            let params = request.get("params").cloned();
            let id = request.get("id").cloned();

            let response = registry.route(method, params.as_ref(), btsp_provider).await;

            let json_response = match response {
                Ok(result) => serde_json::json!({"jsonrpc":"2.0","result":result,"id":id}),
                Err(e) => serde_json::json!({
                    "jsonrpc":"2.0",
                    "error":{"code":-32601,"message":format!("Business error: {e}")},
                    "id":id
                }),
            };

            let resp_str = serde_json::to_string(&json_response)
                .map_err(|e| BearDogError::system(format!("Serialize response: {e}")))?;
            let encrypted = session
                .encrypt_frame(resp_str.as_bytes())
                .map_err(|e| BearDogError::system(format!("BTSP encrypt: {e}")))?;
            btsp_handshake::write_frame(stream, &encrypted)
                .await
                .map_err(|e| BearDogError::system(format!("BTSP frame write: {e}")))?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::mocks::create_minimal_beardog_provider;
    use crate::unix_socket_ipc::handlers::HandlerRegistry;
    use beardog_types::primal_identity::PrimalIdentity;
    use std::sync::Arc;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::TcpListener;

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

        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.expect("accept health ping client");
            TcpIpcServer::handle_connection(
                stream,
                registry,
                provider,
                BtspSecurityMode::Development,
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
