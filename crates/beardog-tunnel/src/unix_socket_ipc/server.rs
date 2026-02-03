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
    handlers::HandlerRegistry,
    types::{JsonRpcError, JsonRpcRequest, JsonRpcResponse, Protocol},
};
use crate::btsp_provider::BeardogBtspProvider;
use crate::platform::{PlatformSocket, PlatformStream, Socket, SocketEndpoint};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, error, info, warn};

/// Unix socket IPC server for inter-primal communication
pub struct UnixSocketIpcServer {
    /// Path to the Unix socket
    socket_path: PathBuf,

    /// BTSP provider (provides all capabilities)
    btsp_provider: Arc<BeardogBtspProvider>,

    /// Modular handler registry for JSON-RPC methods
    handler_registry: Arc<HandlerRegistry>,

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
    /// * `identity` - Primal identity (family and node)
    ///
    /// # Errors
    /// Returns error if unable to remove existing socket file
    pub async fn new(
        socket_path: impl AsRef<Path>,
        btsp_provider: Arc<BeardogBtspProvider>,
        identity: Arc<beardog_types::primal_identity::PrimalIdentity>,
    ) -> Result<Self> {
        let socket_path = socket_path.as_ref().to_path_buf();

        // Remove existing socket file if present (ASYNC - non-blocking!)
        if socket_path.exists() {
            info!("🧹 Removing existing socket: {}", socket_path.display());
            tokio::fs::remove_file(&socket_path)
                .await
                .context("Failed to remove existing socket")?;
        }

        Ok(Self {
            socket_path,
            btsp_provider,
            handler_registry: HandlerRegistry::new(identity),
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

        // Platform-agnostic socket binding (ecoBin v2.0)
        // Use socket_path provided in constructor
        let platform_type = if cfg!(target_os = "android") {
            "Android (abstract socket)"
        } else {
            "Unix (filesystem)"
        };
        info!("   Platform: {}", platform_type);

        // Create endpoint from stored socket_path
        let endpoint = SocketEndpoint::Filesystem(self.socket_path.clone());

        // Bind with platform-specific logic (universal listener!)
        let mut listener = Socket::bind(&endpoint).context(format!(
            "Failed to bind socket on {}: {}",
            platform_type,
            self.socket_path.display()
        ))?;

        // Mark server as ready atomically (no locks needed!)
        // This enables lock-free concurrent readiness checks!
        self.is_ready
            .store(true, std::sync::atomic::Ordering::Release);

        info!(
            "✅ Unix socket IPC server listening: {}",
            endpoint.display()
        );
        info!("   Status: READY ✅ (atomic flag set)");

        // Accept connections loop (universal platform support!)
        loop {
            match listener.accept().await {
                Ok(stream) => {
                    // stream is Box<dyn PlatformStream> - works on all platforms!
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
    async fn handle_connection(&self, stream: Box<dyn PlatformStream>) -> Result<()> {
        debug!("📥 New IPC connection (universal platform)");

        // TODO: Full universal stream refactoring in Phase 3
        // For now, we need to downcast to UnixStream on Unix platforms
        // This is temporary until we refactor handlers to use AsyncRead/AsyncWrite traits
        
        #[cfg(unix)]
        {
            // On Unix platforms, downcast the stream
            // This is safe because we know the platform at compile time
            
            
            
            // SAFETY: We can't directly downcast Box<dyn PlatformStream>,
            // so we need a different approach. Let's use AsyncRead/AsyncWrite directly!
            
            // Read first line using AsyncRead trait
            let mut first_line = String::new();
            let mut buffer = Vec::new();
            let mut stream = stream; // Make mutable
            
            // Read until newline
            loop {
                let mut byte = [0u8; 1];
                match stream.read_exact(&mut byte).await {
                    Ok(_) => {
                        buffer.push(byte[0]);
                        if byte[0] == b'\n' {
                            break;
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                        if buffer.is_empty() {
                            debug!("📤 Client disconnected immediately");
                            return Ok(());
                        }
                        break;
                    }
                    Err(e) => {
                        error!("❌ Failed to read from stream: {}", e);
                        return Err(anyhow::anyhow!("Failed to read: {}", e));
                    }
                }
            }
            
            first_line = String::from_utf8_lossy(&buffer).to_string();
            
            if first_line.trim().is_empty() {
                debug!("📤 Empty request, ignoring");
                return Ok(());
            }

            // Detect protocol
            let protocol = Protocol::detect_from_bytes(first_line.as_bytes());

            // Log security level
            match protocol {
                Protocol::JsonRpc => {
                    info!(
                        "📡 JSON-RPC connection (security level: {}) - PRIMARY protocol",
                        protocol.security_level()
                    );
                }
                Protocol::Http => {
                    warn!(
                        "⚠️  HTTP connection (security level: {})",
                        protocol.security_level()
                    );
                    warn!(
                        "⚠️  HTTP is less secure than JSON-RPC for inter-primal communication"
                    );
                    warn!("⚠️  Consider migrating to JSON-RPC 2.0 over Unix sockets");
                }
            }

            // Route to universal handler (using AsyncRead/AsyncWrite traits!)
            match protocol {
                Protocol::JsonRpc => {
                    self.handle_jsonrpc_universal(&first_line, stream).await?;
                }
                Protocol::Http => {
                    self.handle_http_universal(&first_line, stream).await?;
                }
            }
        }
        
        #[cfg(not(unix))]
        {
            // For non-Unix platforms, implement similar logic
            warn!("⚠️  Non-Unix platform handler not yet fully implemented");
            return Err(anyhow::anyhow!("Platform not yet supported in this handler"));
        }

        Ok(())
    }
    
    /// Handle JSON-RPC requests using universal platform stream
    ///
    /// **Phase 2 Universal Handler** (Jan 31, 2026):
    /// Uses AsyncRead/AsyncWrite traits - works on all platforms!
    async fn handle_jsonrpc_universal(
        &self,
        first_line: &str,
        mut stream: Box<dyn PlatformStream>,
    ) -> Result<()> {
        // Handle first request
        let response = self.handle_one_jsonrpc_request_universal(first_line).await?;
        stream.write_all(response.as_bytes()).await?;
        stream.write_all(b"\n").await?;

        // Continue handling requests until connection closes
        loop {
            let mut line_buf = Vec::new();
            let mut byte = [0u8; 1];
            
            loop {
                match stream.read_exact(&mut byte).await {
                    Ok(_) => {
                        line_buf.push(byte[0]);
                        if byte[0] == b'\n' {
                            break;
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                        if line_buf.is_empty() {
                            debug!("📤 Client disconnected gracefully");
                            return Ok(());
                        }
                        break;
                    }
                    Err(e) => {
                        error!("❌ Read error: {}", e);
                        return Err(anyhow::anyhow!("Read failed: {}", e));
                    }
                }
            }
            
            let line = String::from_utf8_lossy(&line_buf).to_string();
            if line.trim().is_empty() {
                continue;
            }
            
            match self.handle_one_jsonrpc_request_universal(&line).await {
                Ok(response) => {
                    if let Err(e) = stream.write_all(response.as_bytes()).await {
                        warn!("⚠️  Failed to write response: {}", e);
                        break;
                    }
                    if let Err(e) = stream.write_all(b"\n").await {
                        warn!("⚠️  Failed to write newline: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    warn!("⚠️  Error handling request: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }
    
    /// Handle HTTP requests using universal platform stream
    async fn handle_http_universal(
        &self,
        _first_line: &str,
        mut stream: Box<dyn PlatformStream>,
    ) -> Result<()> {
        // Simple HTTP not supported message
        let response = b"HTTP/1.1 501 Not Implemented\r\nContent-Length: 50\r\n\r\nHTTP deprecated - use JSON-RPC over Unix sockets\n";
        stream.write_all(response).await?;
        Ok(())
    }
    
    /// Process one JSON-RPC request and return response string
    async fn handle_one_jsonrpc_request_universal(&self, line: &str) -> Result<String> {
        // Parse JSON-RPC request
        let request: JsonRpcRequest = match serde_json::from_str(line.trim()) {
            Ok(req) => req,
            Err(e) => {
                warn!("⚠️  Invalid JSON-RPC request: {}", e);
                let error_response = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: format!("Parse error: {}", e),
                        data: None,
                    }),
                    id: serde_json::Value::Null,
                };
                return Ok(serde_json::to_string(&error_response)?);
            }
        };

        debug!("📨 JSON-RPC request: {}", request.method);

        // Process request through handler registry
        let response = match self
            .handler_registry
            .route(&request.method, request.params.as_ref(), &self.btsp_provider)
            .await
        {
            Ok(result) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(result),
                error: None,
                id: request.id.clone().unwrap_or(serde_json::Value::Null),
            },
            Err(error_msg) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: error_msg,
                    data: None,
                }),
                id: request.id.clone().unwrap_or(serde_json::Value::Null),
            },
        };
        
        Ok(serde_json::to_string(&response)?)
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

    // handle_tarpc_persistent() removed - see TARPC_REMOVAL_RATIONALE_JAN_29_2026.md
    // JSON-RPC provides comprehensive functionality (8+ handler modules, 30+ methods)

    /// Handle JSON-RPC request via modular handler registry
    ///
    /// This method routes requests through the trait-based handler registry,
    /// bypassing the legacy router for cleaner, more efficient processing.
    async fn handle_jsonrpc_via_registry(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        debug!("→ JSON-RPC Request: {}", request.method);

        // Validate JSON-RPC version
        if request.jsonrpc != "2.0" {
            return JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32600,
                    message: "Invalid JSON-RPC version (must be 2.0)".to_string(),
                    data: None,
                }),
                id: request.id.clone().unwrap_or(serde_json::Value::Null),
            };
        }

        // Route to handler via registry
        let result = self
            .handler_registry
            .route(
                &request.method,
                request.params.as_ref(),
                &self.btsp_provider,
            )
            .await;

        // Build response with proper error codes
        match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(value),
                error: None,
                id: request.id.clone().unwrap_or(serde_json::Value::Null),
            },
            Err(e) => {
                // Detect error type and use appropriate error code
                let (code, message) =
                    if e.contains("Method not found") || e.contains("Unknown method") {
                        (JsonRpcError::METHOD_NOT_FOUND, e)
                    } else if e.contains("Invalid params") || e.contains("Missing required") {
                        (JsonRpcError::INVALID_PARAMS, e)
                    } else {
                        (JsonRpcError::INTERNAL_ERROR, e)
                    };

                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code,
                        message,
                        data: None,
                    }),
                    id: request.id.clone().unwrap_or(serde_json::Value::Null),
                }
            }
        }
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

        // Handle request via modular handler registry
        let response = self.handle_jsonrpc_via_registry(&request).await;

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
    async fn route_request(
        &self,
        method: &str,
        request_data: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        // Convert to JsonRpcRequest format for handler registry
        let json_rpc_request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params: request_data.get("params").cloned(),
            id: request_data.get("id").cloned(),
        };

        // Use modular handler registry
        let response = self.handle_jsonrpc_via_registry(&json_rpc_request).await;

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
        Ok(self.handle_jsonrpc_via_registry(&request).await)
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

        debug!("📨 HTTP request: {} {} (DEPRECATED)", method, path);

        // HTTP is deprecated - return JSON-RPC migration notice
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: application/json\r\n\
             \r\n\
             {{\
               \"status\": \"deprecated\",\
               \"message\": \"HTTP protocol is deprecated. Use JSON-RPC 2.0 over Unix socket.\",\
               \"migration\": {{\
                 \"protocol\": \"JSON-RPC 2.0\",\
                 \"transport\": \"Unix socket\",\
                 \"socket_path\": \"{}\",\
                 \"example\": {{\"jsonrpc\":\"2.0\",\"method\":\"health\",\"id\":1}}\
               }}\
             }}",
            self.socket_path.display()
        );

        // Send deprecation notice
        writer.write_all(response.as_bytes()).await?;

        Ok(())
    }
}
