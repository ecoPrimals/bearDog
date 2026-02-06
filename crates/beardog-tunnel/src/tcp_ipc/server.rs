//! TCP IPC Server for BearDog
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
use tracing::{debug, error, info, warn};

/// TCP IPC Server
///
/// Universal JSON-RPC server over TCP. Works on all platforms including
/// Android where Unix sockets may be restricted by SELinux.
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
    pub async fn start(&self) -> Result<(), BearDogError> {
        info!("🌐 Starting TCP IPC server: {}", self.bind_addr);

        // Bind TCP listener
        let listener = TcpListener::bind(self.bind_addr)
            .await
            .map_err(|e| BearDogError::system(format!("Failed to bind TCP: {}", e)))?;

        // Get actual bound address (important if port was 0)
        let bound_addr = listener
            .local_addr()
            .map_err(|e| BearDogError::system(format!("Failed to get local address: {}", e)))?;

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
            .map_err(|e| BearDogError::system(format!("Failed to get peer address: {}", e)))?;

        debug!("🔌 Handling connection from: {}", peer_addr);

        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            line.clear();

            // Read JSON-RPC request (newline-delimited)
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    // Connection closed
                    debug!("Connection closed by peer: {}", peer_addr);
                    break;
                }
                Ok(n) => {
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
                                writer.write_all(response.as_bytes()).await.ok();
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
                Err(e) => {
                    error!("Failed to read from {}: {}", peer_addr, e);
                    break;
                }
            }
        }

        debug!("✅ Connection handler finished: {}", peer_addr);
        Ok(())
    }
}
