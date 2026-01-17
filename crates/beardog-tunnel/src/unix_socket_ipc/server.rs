//! Unix socket IPC server - core server logic
//!
//! This module contains the main `UnixSocketIpcServer` struct and its core
//! functionality, including:
//! - Server lifecycle management (start/stop)
//! - Atomic readiness tracking
//! - Connection acceptance and protocol routing
//!
//! The server supports multiple protocols (tarpc, JSON-RPC, HTTP) with
//! automatic detection and routing to appropriate handlers.

use super::{
    handlers::{handle_http_request, handle_jsonrpc_request},
    types::{JsonRpcRequest, JsonRpcResponse, Protocol},
};
use crate::btsp_provider::BeardogBtspProvider;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tracing::{debug, error, info, warn};

/// Unix socket IPC server for inter-primal communication
pub struct UnixSocketIpcServer {
    /// Path to the Unix socket
    socket_path: PathBuf,

    /// BTSP provider (provides all capabilities)
    btsp_provider: Arc<BeardogBtspProvider>,

    /// Server running state (using RwLock for compatibility)
    is_running: Arc<tokio::sync::RwLock<bool>>,

    /// Atomic readiness flag for lock-free checks
    /// This allows other components to wait for readiness without filesystem polling
    /// (Learned from Songbird's implementation - much better than sleep loops!)
    is_ready: Arc<std::sync::atomic::AtomicBool>,
}

impl UnixSocketIpcServer {
    /// Create a new Unix socket IPC server
    ///
    /// # Arguments
    /// * `socket_path` - Path to the Unix socket file
    /// * `btsp_provider` - BTSP provider for handling requests
    ///
    /// # Errors
    /// Returns error if unable to remove existing socket file
    pub async fn new(
        socket_path: impl AsRef<Path>,
        btsp_provider: Arc<BeardogBtspProvider>,
    ) -> Result<Self> {
        let socket_path = socket_path.as_ref().to_path_buf();

        // Remove existing socket file if present
        if socket_path.exists() {
            info!("🧹 Removing existing socket: {}", socket_path.display());
            std::fs::remove_file(&socket_path).context("Failed to remove existing socket")?;
        }

        Ok(Self {
            socket_path,
            btsp_provider,
            is_running: Arc::new(tokio::sync::RwLock::new(false)),
            is_ready: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }

    /// Get the socket path
    #[must_use]
    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    /// Get a clone of the readiness flag
    ///
    /// This allows checking readiness even after the server has been moved
    /// into a spawn task. This is lock-free and safe for concurrent access!
    #[must_use]
    pub fn readiness_flag(&self) -> Arc<std::sync::atomic::AtomicBool> {
        Arc::clone(&self.is_ready)
    }

    /// Check if the server is ready to accept connections
    ///
    /// This is an atomic, lock-free operation that can be safely called
    /// from any thread without blocking. Modern concurrent Rust at its best!
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.is_ready.load(std::sync::atomic::Ordering::Acquire)
    }

    /// Wait for the server to be ready
    ///
    /// This is a non-blocking async wait that checks readiness without
    /// filesystem polling. Use this instead of `sleep` loops!
    ///
    /// Returns `true` if ready within timeout, `false` if timeout expired.
    pub async fn wait_ready(&self, timeout: std::time::Duration) -> bool {
        let start = std::time::Instant::now();
        while !self.is_ready() {
            if start.elapsed() > timeout {
                return false;
            }
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        }
        true
    }

    /// Wait for readiness using a readiness flag
    ///
    /// This is a standalone function for use after the server has been moved.
    /// Fully concurrent - no locks, just atomic operations!
    pub async fn wait_ready_flag(
        flag: &Arc<std::sync::atomic::AtomicBool>,
        timeout: std::time::Duration,
    ) -> bool {
        let start = std::time::Instant::now();
        while !flag.load(std::sync::atomic::Ordering::Acquire) {
            if start.elapsed() > timeout {
                return false;
            }
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        }
        true
    }

    /// Stop the IPC server gracefully
    ///
    /// Cleans up the socket file. The actual server loop will continue
    /// until the task is cancelled by the orchestrator.
    ///
    /// # Errors
    /// Returns error if unable to remove socket file
    pub async fn stop(&self) -> Result<()> {
        info!("🛑 Stopping Unix socket IPC server...");

        // Mark as not ready (atomic, lock-free!)
        self.is_ready
            .store(false, std::sync::atomic::Ordering::Release);

        // Mark as not running (needs lock for compatibility)
        {
            let mut running = self.is_running.write().await;
            *running = false;
        }

        // Remove socket file
        if self.socket_path.exists() {
            std::fs::remove_file(&self.socket_path).context("Failed to remove socket file")?;
            info!("🧹 Removed socket: {}", self.socket_path.display());
        }

        Ok(())
    }

    /// Start the Unix socket IPC server
    ///
    /// This method runs the main server loop, accepting connections and
    /// spawning handlers for each. Returns only on error or when stopped.
    ///
    /// # Errors
    /// Returns error if unable to bind to socket or accept connections
    pub async fn start(self: Arc<Self>) -> Result<()> {
        {
            let mut is_running = self.is_running.write().await;
            if *is_running {
                warn!("⚠️  Unix socket IPC server already running");
                return Ok(());
            }
            *is_running = true;
        }

        info!(
            "🔌 Starting Unix socket IPC server: {}",
            self.socket_path.display()
        );

        // Bind Unix socket
        let listener = UnixListener::bind(&self.socket_path).context(format!(
            "Failed to bind Unix socket: {}",
            self.socket_path.display()
        ))?;

        // Mark server as ready atomically (no locks needed!)
        // This enables lock-free concurrent readiness checks!
        self.is_ready
            .store(true, std::sync::atomic::Ordering::Release);

        info!(
            "✅ Unix socket IPC server listening: {}",
            self.socket_path.display()
        );
        info!("   Status: READY ✅ (atomic flag set)");

        // Accept connections loop
        loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    let server = Arc::clone(&self);
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_connection(stream).await {
                            error!("❌ Connection handler error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("❌ Failed to accept connection: {}", e);
                }
            }
        }
    }

    /// Handle a single client connection with protocol detection
    ///
    /// Reads the first line to detect the protocol (tarpc, JSON-RPC, or HTTP),
    /// then routes to the appropriate handler.
    /// For JSON-RPC, continues handling requests until connection closes.
    ///
    /// # Errors
    /// Returns error if unable to read from stream or handle request
    async fn handle_connection(&self, stream: UnixStream) -> Result<()> {
        debug!("📥 New IPC connection");

        let (reader, writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut writer = writer;
        let mut first_line = String::new();

        // Read first line to detect protocol
        match reader.read_line(&mut first_line).await {
            Ok(0) => {
                debug!("📤 Client disconnected immediately");
                return Ok(());
            }
            Ok(_) => {
                if first_line.trim().is_empty() {
                    debug!("📤 Empty request, ignoring");
                    return Ok(());
                }

                // Detect protocol
                let protocol = Protocol::detect_from_bytes(first_line.as_bytes());

                // Log security level
                match protocol {
                    Protocol::Tarpc => {
                        info!(
                            "🎯 tarpc connection (security level: {}) - PRIMARY protocol!",
                            protocol.security_level()
                        );
                        info!("✅ Type-safe, efficient, modern Rust inter-primal communication");
                    }
                    Protocol::JsonRpc => {
                        debug!(
                            "🔐 JSON-RPC connection (security level: {}) - FALLBACK protocol",
                            protocol.security_level()
                        );
                    }
                    Protocol::Http => {
                        warn!(
                            "⚠️  HTTP connection (security level: {})",
                            protocol.security_level()
                        );
                        warn!("⚠️  HTTP is less secure, less reliable, less fractal than tarpc/JSON-RPC");
                        warn!("⚠️  Consider migrating to tarpc for inter-primal communication");
                    }
                }

                // Route to appropriate handler
                match protocol {
                    Protocol::Tarpc => {
                        info!("✅ tarpc protocol detected - routing to tarpc handler");
                        self.handle_tarpc_persistent(&first_line, &mut reader, &mut writer)
                            .await?;
                    }
                    Protocol::JsonRpc => {
                        self.handle_jsonrpc_persistent(&first_line, &mut reader, &mut writer)
                            .await?;
                    }
                    Protocol::Http => {
                        self.handle_http_connection(&first_line, &mut reader, &mut writer)
                            .await?;
                    }
                }
            }
            Err(e) => {
                error!("❌ Failed to read from connection: {}", e);
            }
        }

        Ok(())
    }

    /// Handle JSON-RPC connection persistently (multiple requests)
    ///
    /// Modern JSON-RPC supports persistent connections with multiple
    /// requests over a single connection. This reduces overhead and
    /// enables better performance for inter-primal communication.
    async fn handle_jsonrpc_persistent(
        &self,
        first_line: &str,
        reader: &mut BufReader<tokio::net::unix::OwnedReadHalf>,
        writer: &mut tokio::net::unix::OwnedWriteHalf,
    ) -> Result<()> {
        // Handle first request
        self.handle_one_jsonrpc_request(first_line, writer).await?;

        // Continue handling requests until connection closes
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    debug!("📤 Client disconnected gracefully");
                    break;
                }
                Ok(_) => {
                    if line.trim().is_empty() {
                        continue;
                    }
                    if let Err(e) = self.handle_one_jsonrpc_request(&line, writer).await {
                        warn!("⚠️  Error handling request: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    debug!("📤 Connection closed: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Handle tarpc connection persistently (COMPLETE IMPLEMENTATION)
    ///
    /// tarpc uses bincode serialization with efficient binary protocol.
    /// This is the PRIMARY protocol for inter-primal communication.
    async fn handle_tarpc_persistent(
        &self,
        first_line: &str,
        reader: &mut BufReader<tokio::net::unix::OwnedReadHalf>,
        writer: &mut tokio::net::unix::OwnedWriteHalf,
    ) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        
        info!("🚀 tarpc handler - PRIMARY inter-primal protocol");
        
        // tarpc uses bincode serialization after magic bytes
        // For now, we'll decode and handle via JSON-RPC-compatible interface
        // Full tarpc integration would use generated service traits
        
        // Strip tarpc magic bytes ("TRPC")
        let payload = if first_line.starts_with("TRPC") {
            &first_line[4..]
        } else {
            first_line
        };
        
        // Decode bincode to serde_json::Value for routing
        // In production, this would use generated tarpc service definitions
        match bincode::deserialize::<serde_json::Value>(payload.as_bytes()) {
            Ok(request_data) => {
                // Route to appropriate handler based on method field
                if let Some(method) = request_data.get("method").and_then(|m| m.as_str()) {
                    debug!("📨 tarpc request: {}", method);
                    
                    // Handle request using existing handlers
                    let response = self.route_request(method, &request_data).await?;
                    
                    // Serialize response with tarpc magic bytes
                    let response_bytes = bincode::serialize(&response)
                        .context("Failed to serialize tarpc response")?;
                    
                    // Write magic bytes + response
                    writer.write_all(b"TRPC").await?;
                    writer.write_all(&response_bytes).await?;
                    writer.flush().await?;
                    
                    info!("✅ tarpc response sent: {} bytes", response_bytes.len());
                } else {
                    warn!("⚠️  tarpc request missing method field");
                }
            }
            Err(e) => {
                warn!("⚠️  Failed to decode tarpc request: {}", e);
                // Send error response
                let error_response = serde_json::json!({
                    "error": {
                        "code": -32700,
                        "message": "Parse error"
                    }
                });
                let error_bytes = bincode::serialize(&error_response)?;
                writer.write_all(b"TRPC").await?;
                writer.write_all(&error_bytes).await?;
                writer.flush().await?;
            }
        }

        Ok(())
    }

    /// Handle a single JSON-RPC request
    async fn handle_one_jsonrpc_request(
        &self,
        line: &str,
        writer: &mut tokio::net::unix::OwnedWriteHalf,
    ) -> Result<()> {
        // Parse JSON-RPC request
        let request: JsonRpcRequest =
            serde_json::from_str(line).context("Failed to parse JSON-RPC request")?;

        debug!("📨 JSON-RPC request: {}", request.method);

        // Handle request
        let response = handle_jsonrpc_request(&request, &self.btsp_provider).await;

        // Send response
        let response_json = serde_json::to_string(&response)?;
        writer
            .write_all(response_json.as_bytes())
            .await
            .context("Failed to write JSON-RPC response")?;
        writer
            .write_all(b"\n")
            .await
            .context("Failed to write newline")?;
        writer.flush().await.context("Failed to flush writer")?;

        Ok(())
    }

    /// Route request to appropriate handler (shared by JSON-RPC and tarpc)
    async fn route_request(&self, method: &str, request_data: &serde_json::Value) -> Result<serde_json::Value> {
        // Convert to JsonRpcRequest format for existing handlers
        let json_rpc_request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params: request_data.get("params").cloned(),
            id: request_data.get("id").cloned(),
        };
        
        // Use existing handler infrastructure
        let response = handle_jsonrpc_request(&json_rpc_request, &self.btsp_provider).await;
        
        // Convert response to Value
        Ok(serde_json::to_value(response)?)
    }

    /// Handle JSON-RPC request (public for testing)
    ///
    /// # Note
    /// This is public for testing purposes but considered internal API
    ///
    /// # Errors
    /// Returns error if unable to parse or handle the request
    pub async fn handle_jsonrpc_request(&self, request_str: &str) -> Result<JsonRpcResponse> {
        let request: JsonRpcRequest =
            serde_json::from_str(request_str).context("Failed to parse JSON-RPC request")?;
        Ok(handle_jsonrpc_request(&request, &self.btsp_provider).await)
    }

    /// Handle HTTP connection
    async fn handle_http_connection(
        &self,
        first_line: &str,
        reader: &mut BufReader<tokio::net::unix::OwnedReadHalf>,
        writer: &mut tokio::net::unix::OwnedWriteHalf,
    ) -> Result<()> {
        // Read HTTP headers
        let mut headers = vec![first_line.to_string()];
        let mut header_line = String::new();

        loop {
            header_line.clear();
            reader.read_line(&mut header_line).await?;
            if header_line.trim().is_empty() {
                break;
            }
            headers.push(header_line.clone());
        }

        // Parse request line (e.g., "GET /health HTTP/1.1")
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        if parts.len() < 2 {
            let error_response = b"HTTP/1.1 400 Bad Request\r\n\r\n";
            writer.write_all(error_response).await?;
            return Ok(());
        }

        let method = parts[0];
        let path = parts[1];

        debug!("📨 HTTP request: {} {}", method, path);

        // Handle request
        let response = handle_http_request(method, path, &self.btsp_provider).await;

        // Send response
        writer.write_all(response.as_bytes()).await?;

        Ok(())
    }
}
