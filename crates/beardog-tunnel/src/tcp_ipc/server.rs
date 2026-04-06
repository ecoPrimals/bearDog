// SPDX-License-Identifier: AGPL-3.0-or-later

//! TCP IPC Server for `BearDog`
//!
//! Provides JSON-RPC over TCP for universal platform support.

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

    /// Actual bound address (after OS assigns port if using :0)
    bound_addr: Arc<RwLock<Option<SocketAddr>>>,
}

impl TcpIpcServer {
    /// Create new TCP IPC server
    pub fn new(
        bind_addr: SocketAddr,
        btsp_provider: Arc<BeardogBtspProvider>,
        identity: Arc<PrimalIdentity>,
    ) -> Self {
        Self {
            bind_addr,
            btsp_provider,
            handler_registry: HandlerRegistry::new(identity),
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

                    // Spawn task to handle connection
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(stream, registry, btsp).await {
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
        stream: TcpStream,
        registry: Arc<HandlerRegistry>,
        btsp_provider: Arc<BeardogBtspProvider>,
    ) -> Result<(), BearDogError> {
        let peer_addr = stream
            .peer_addr()
            .map_err(|e| BearDogError::system(format!("Failed to get peer address: {e}")))?;

        debug!("🔌 Handling connection from: {}", peer_addr);

        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            line.clear();

            // Read JSON-RPC request (NDJSON). Wrapped with timeout to prevent
            // indefinite blocking when probes connect without sending a newline.
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
            TcpIpcServer::handle_connection(stream, registry, provider).await
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
            TcpIpcServer::handle_connection(stream, registry, provider).await
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
            TcpIpcServer::handle_connection(stream, registry, provider).await
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
        let server = TcpIpcServer::new(addr, provider, identity);
        assert!(server.get_bound_addr().await.is_none());
    }
}
