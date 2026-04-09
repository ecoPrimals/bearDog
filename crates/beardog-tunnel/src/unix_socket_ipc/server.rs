// SPDX-License-Identifier: AGPL-3.0-or-later

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
use crate::btsp_handshake::{self, BtspSecurityMode, BtspSession};
use crate::btsp_provider::BeardogBtspProvider;
use crate::platform::{PlatformSocket, PlatformStream, Socket, SocketEndpoint};
use anyhow::{Context, Result};
use beardog_core::socket_config::{
    IpcCapabilitySymlinksConfig, install_ipc_symlinks_at, remove_ipc_symlinks_at,
};
use beardog_ipc::protocol::JSONRPC_VERSION;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::time::Duration;
use tracing::{debug, error, info, warn};

/// Per-read timeout for NDJSON connections.
///
/// Prevents indefinite blocking when a client connects but never sends a
/// newline (e.g. raw `nc` probes, `curl` health checks). On timeout the
/// connection is closed and the task freed.
const IPC_READ_TIMEOUT: Duration = Duration::from_secs(30);

/// Unix socket IPC server for inter-primal communication
pub struct UnixSocketIpcServer {
    /// Path to the Unix socket
    socket_path: PathBuf,

    /// wateringHole v3.1 capability-domain symlinks beside [`Self::socket_path`].
    ipc_symlinks: IpcCapabilitySymlinksConfig,

    /// BTSP provider (provides all capabilities)
    btsp_provider: Arc<BeardogBtspProvider>,

    /// Modular handler registry for JSON-RPC methods
    handler_registry: Arc<HandlerRegistry>,

    /// BTSP security mode (resolved at startup, checked per connection).
    security_mode: BtspSecurityMode,

    /// Server running state (using `RwLock` for compatibility)
    is_running: Arc<tokio::sync::RwLock<bool>>,

    /// Atomic readiness flag for lock-free checks
    /// This allows other components to wait for readiness without filesystem polling
    /// Avoids filesystem polling (readiness flag is a common IPC pattern)
    is_ready: Arc<std::sync::atomic::AtomicBool>,
}

impl UnixSocketIpcServer {
    /// Create a new Unix socket IPC server
    ///
    /// # Arguments
    /// * `socket_path` - Path to the Unix socket file
    /// * `btsp_provider` - BTSP provider for handling requests
    /// * `identity` - Primal identity (family and node)
    /// * `security_mode` - BTSP security posture (production vs development)
    /// * `ipc_symlinks` - Capability-domain symlink config (wateringHole v3.1)
    ///
    /// # Errors
    /// Returns error if unable to remove existing socket file
    pub async fn new(
        socket_path: impl AsRef<Path>,
        btsp_provider: Arc<BeardogBtspProvider>,
        identity: Arc<beardog_types::primal_identity::PrimalIdentity>,
        security_mode: BtspSecurityMode,
        ipc_symlinks: IpcCapabilitySymlinksConfig,
    ) -> Result<Self> {
        let socket_path = socket_path.as_ref().to_path_buf();

        // Remove existing socket file if present (ASYNC - non-blocking!)
        if socket_path.exists() {
            info!(path = %socket_path.display(), "Removing existing socket");
            tokio::fs::remove_file(&socket_path)
                .await
                .context("Failed to remove existing socket")?;
        }

        if security_mode.is_production() {
            info!("BTSP handshake enforcement ENABLED (production mode)");
        } else {
            info!("BTSP handshake enforcement disabled (development mode)");
        }

        Ok(Self {
            socket_path,
            ipc_symlinks,
            btsp_provider,
            handler_registry: HandlerRegistry::new(identity),
            security_mode,
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
            tokio::task::yield_now().await;
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
            tokio::task::yield_now().await;
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
        info!("Stopping Unix socket IPC server");

        // Mark as not ready (atomic, lock-free!)
        self.is_ready
            .store(false, std::sync::atomic::Ordering::Release);

        // Mark as not running (needs lock for compatibility)
        {
            let mut running = self.is_running.write().await;
            *running = false;
        }

        #[cfg(unix)]
        {
            remove_ipc_symlinks_at(
                &self.socket_path,
                &self.ipc_symlinks.symlink_suffix,
                &self.ipc_symlinks.domain_stems,
            );
        }

        if self.socket_path.exists() {
            tokio::fs::remove_file(&self.socket_path)
                .await
                .context("Failed to remove socket file")?;
            info!(path = %self.socket_path.display(), "Removed socket");
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
                warn!("Unix socket IPC server already running");
                return Ok(());
            }
            *is_running = true;
        }

        info!(
            path = %self.socket_path.display(),
            "Starting Unix socket IPC server"
        );

        // Platform-agnostic socket binding (ecoBin v2.0)
        // Use socket_path provided in constructor
        let platform_type = if cfg!(target_os = "android") {
            "Android (abstract socket)"
        } else {
            "Unix (filesystem)"
        };
        info!(platform = platform_type, "IPC server platform");

        // Create endpoint from stored socket_path
        let endpoint = SocketEndpoint::Filesystem(self.socket_path.clone());

        // Bind with platform-specific logic (universal listener!)
        let mut listener = Socket::bind(&endpoint).context(format!(
            "Failed to bind socket on {}: {}",
            platform_type,
            self.socket_path.display()
        ))?;

        #[cfg(unix)]
        {
            if let Some(name) = self.socket_path.file_name() {
                let _created = install_ipc_symlinks_at(
                    &self.socket_path,
                    name,
                    &self.ipc_symlinks.symlink_suffix,
                    &self.ipc_symlinks.domain_stems,
                );
            } else {
                warn!(
                    path = %self.socket_path.display(),
                    "socket path has no filename; skipping wateringHole capability symlinks"
                );
            }
        }

        // Mark server as ready atomically (no locks needed!)
        // This enables lock-free concurrent readiness checks!
        self.is_ready
            .store(true, std::sync::atomic::Ordering::Release);

        info!(
            endpoint = %endpoint.display(),
            "Unix socket IPC server listening"
        );
        info!(
            status = "ready",
            "Unix socket IPC server status (atomic flag set)"
        );

        // Accept connections loop (universal platform support!)
        loop {
            match listener.accept().await {
                Ok(stream) => {
                    // stream is Box<dyn PlatformStream> - works on all platforms!
                    let server = Arc::clone(&self);
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_connection(stream).await {
                            error!(error = %e, "Connection handler error");
                        }
                    });
                }
                Err(e) => {
                    error!(error = %e, "Failed to accept connection");
                }
            }
        }
    }

    /// Handle a single client connection with protocol detection
    ///
    /// In production mode (`FAMILY_ID` set), runs the BTSP 4-step handshake
    /// before accepting any JSON-RPC traffic. On successful handshake,
    /// communication switches to length-prefixed encrypted frames.
    ///
    /// In development mode, falls through to the existing NDJSON path.
    ///
    /// # Errors
    /// Returns error if unable to read from stream or handle request
    async fn handle_connection(&self, stream: Box<dyn PlatformStream>) -> Result<()> {
        debug!("New IPC connection (universal platform)");

        #[cfg(unix)]
        {
            // ── BTSP production mode: handshake before anything else ───
            if let BtspSecurityMode::Production { ref family_seed } = self.security_mode {
                debug!("BTSP production: initiating handshake");
                let mut stream = stream;
                match btsp_handshake::perform_server_handshake(&mut stream, family_seed).await {
                    Ok(session) => {
                        info!(
                            session_id = %session.session_id,
                            cipher = %session.cipher.wire_name(),
                            "BTSP handshake succeeded — switching to encrypted frames"
                        );
                        return self.handle_jsonrpc_btsp(stream, session).await;
                    }
                    Err(e) => {
                        warn!(error = %e, "BTSP handshake failed — refusing connection");
                        return Ok(());
                    }
                }
            }

            // ── Development mode: plain NDJSON (existing path) ─────────

            let mut buf_stream = BufReader::new(stream);
            let mut buffer = Vec::with_capacity(1024);

            let read_result =
                tokio::time::timeout(IPC_READ_TIMEOUT, buf_stream.read_until(b'\n', &mut buffer))
                    .await;

            match read_result {
                Err(_elapsed) => {
                    warn!(
                        timeout_secs = IPC_READ_TIMEOUT.as_secs(),
                        "IPC initial read timed out — closing idle connection"
                    );
                    return Ok(());
                }
                Ok(Ok(0)) => {
                    debug!("Client disconnected immediately");
                    return Ok(());
                }
                Ok(Ok(_)) => {}
                Ok(Err(e)) => {
                    error!(error = %e, "Failed to read from stream");
                    return Err(anyhow::anyhow!("Failed to read: {e}"));
                }
            }

            // `from_utf8_lossy` returns `Cow`; avoid `.to_string()` so valid UTF-8 borrows `buffer`
            // instead of allocating a second copy on the hot path.
            let first_line = String::from_utf8_lossy(&buffer);
            let stream = buf_stream.into_inner();

            if first_line.trim().is_empty() {
                debug!("Empty request, ignoring");
                return Ok(());
            }

            let protocol = Protocol::detect_from_bytes(first_line.as_bytes());

            match protocol {
                Protocol::JsonRpc => {
                    info!(
                        security_level = protocol.security_level(),
                        protocol = "json_rpc",
                        "JSON-RPC connection (primary protocol)"
                    );
                }
                Protocol::Http => {
                    warn!(
                        security_level = protocol.security_level(),
                        protocol = "http",
                        "HTTP connection (lower security than JSON-RPC for inter-primal communication)"
                    );
                    warn!("Consider migrating to JSON-RPC 2.0 over Unix sockets");
                }
            }

            match protocol {
                Protocol::JsonRpc => {
                    self.handle_jsonrpc_universal(first_line.as_ref(), stream)
                        .await?;
                }
                Protocol::Http => {
                    self.handle_http_universal(first_line.as_ref(), stream)
                        .await?;
                }
            }
        }

        #[cfg(not(unix))]
        {
            warn!("Non-Unix platform handler not yet fully implemented");
            return Err(anyhow::anyhow!(
                "Platform not yet supported in this handler"
            ));
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
        stream: Box<dyn PlatformStream>,
    ) -> Result<()> {
        let mut buf_stream = BufReader::new(stream);

        // Handle first request
        let response = self
            .handle_one_jsonrpc_request_universal(first_line)
            .await?;
        buf_stream.get_mut().write_all(response.as_bytes()).await?;
        buf_stream.get_mut().write_all(b"\n").await?;

        let mut line_buf = Vec::with_capacity(1024);

        loop {
            line_buf.clear();

            let read_result = tokio::time::timeout(
                IPC_READ_TIMEOUT,
                buf_stream.read_until(b'\n', &mut line_buf),
            )
            .await;

            match read_result {
                Err(_elapsed) => {
                    debug!(
                        timeout_secs = IPC_READ_TIMEOUT.as_secs(),
                        "IPC read timed out — closing idle connection"
                    );
                    return Ok(());
                }
                Ok(Ok(0)) => {
                    debug!("Client disconnected gracefully");
                    return Ok(());
                }
                Ok(Ok(_)) => {}
                Ok(Err(e)) => {
                    error!(error = %e, "Read error");
                    return Err(anyhow::anyhow!("Read failed: {e}"));
                }
            }

            let line = String::from_utf8_lossy(&line_buf);
            if line.trim().is_empty() {
                continue;
            }

            match self.handle_one_jsonrpc_request_universal(&line).await {
                Ok(response) => {
                    if let Err(e) = buf_stream.get_mut().write_all(response.as_bytes()).await {
                        warn!(error = %e, "Failed to write response");
                        break;
                    }
                    if let Err(e) = buf_stream.get_mut().write_all(b"\n").await {
                        warn!(error = %e, "Failed to write newline");
                        break;
                    }
                }
                Err(e) => {
                    warn!(error = %e, "Error handling request");
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

    /// Handle JSON-RPC over BTSP encrypted frames (production mode).
    ///
    /// Each frame is decrypted → parsed as JSON-RPC → processed → encrypted → sent.
    async fn handle_jsonrpc_btsp(
        &self,
        mut stream: Box<dyn PlatformStream>,
        mut session: BtspSession,
    ) -> Result<()> {
        loop {
            let frame = match btsp_handshake::read_frame(&mut stream).await {
                Ok(f) => f,
                Err(e) => {
                    debug!(error = %e, "BTSP frame read ended");
                    return Ok(());
                }
            };

            let plaintext = match session.decrypt_frame(&frame) {
                Ok(p) => p,
                Err(e) => {
                    warn!(error = %e, "BTSP frame decrypt failed — dropping connection");
                    return Ok(());
                }
            };

            let line = match String::from_utf8(plaintext) {
                Ok(s) => s,
                Err(e) => {
                    warn!(error = %e, "BTSP frame not valid UTF-8");
                    continue;
                }
            };

            if line.trim().is_empty() {
                continue;
            }

            let response_str = self.handle_one_jsonrpc_request_universal(&line).await?;
            let encrypted = session
                .encrypt_frame(response_str.as_bytes())
                .map_err(|e| anyhow::anyhow!("BTSP encrypt failed: {e}"))?;

            btsp_handshake::write_frame(&mut stream, &encrypted)
                .await
                .map_err(|e| anyhow::anyhow!("BTSP frame write failed: {e}"))?;
        }
    }

    /// Process one JSON-RPC request and return response string
    async fn handle_one_jsonrpc_request_universal(&self, line: &str) -> Result<String> {
        // Parse JSON-RPC request
        let mut request: JsonRpcRequest = match serde_json::from_str(line.trim()) {
            Ok(req) => req,
            Err(e) => {
                warn!(error = %e, "Invalid JSON-RPC request");
                let error_response = JsonRpcResponse {
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: format!("Parse error: {e}"),
                        data: None,
                    }),
                    id: serde_json::Value::Null,
                };
                return Ok(serde_json::to_string(&error_response)?);
            }
        };

        debug!(method = %request.method, "JSON-RPC request");

        // Take id to avoid clone (zero-copy: wateringHole standard)
        let id = request.id.take().unwrap_or(serde_json::Value::Null);

        // Validate JSON-RPC version
        if request.jsonrpc != "2.0" {
            let error_response = JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32600,
                    message: "Invalid JSON-RPC version (must be 2.0)".to_string(),
                    data: None,
                }),
                id,
            };
            return Ok(serde_json::to_string(&error_response)?);
        }

        // Process request through handler registry
        let response = match self
            .handler_registry
            .route(
                &request.method,
                request.params.as_ref(),
                &self.btsp_provider,
            )
            .await
        {
            Ok(result) => JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                result: Some(result),
                error: None,
                id,
            },
            Err(error_msg) => JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: error_msg,
                    data: None,
                }),
                id,
            },
        };

        Ok(serde_json::to_string(&response)?)
    }

    // Legacy handle_jsonrpc_persistent() removed - superseded by handle_jsonrpc_via_registry()
    // See TARPC_REMOVAL_RATIONALE_JAN_29_2026.md for context

    /// Handle JSON-RPC request via modular handler registry
    ///
    /// This method routes requests through the trait-based handler registry,
    /// bypassing the legacy router for cleaner, more efficient processing.
    async fn handle_jsonrpc_via_registry(&self, request: &JsonRpcRequest) -> JsonRpcResponse {
        debug!(method = %request.method, "JSON-RPC request (registry)");

        let id = request.id.clone().unwrap_or(serde_json::Value::Null);

        // Validate JSON-RPC version
        if request.jsonrpc != "2.0" {
            return JsonRpcResponse {
                jsonrpc: JSONRPC_VERSION.to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32600,
                    message: "Invalid JSON-RPC version (must be 2.0)".to_string(),
                    data: None,
                }),
                id,
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
                jsonrpc: JSONRPC_VERSION.to_string(),
                result: Some(value),
                error: None,
                id,
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
                    jsonrpc: JSONRPC_VERSION.to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code,
                        message,
                        data: None,
                    }),
                    id,
                }
            }
        }
    }

    // Legacy handle_one_jsonrpc_request() and route_request() removed
    // Superseded by handle_jsonrpc_via_registry() - cleaner, more efficient

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

    // Legacy handle_http_connection() removed - HTTP protocol deprecated
    // All clients should use JSON-RPC 2.0 over Unix socket
}

impl Drop for UnixSocketIpcServer {
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            remove_ipc_symlinks_at(
                &self.socket_path,
                &self.ipc_symlinks.symlink_suffix,
                &self.ipc_symlinks.domain_stems,
            );
            if self.socket_path.exists() {
                let _ = std::fs::remove_file(&self.socket_path);
            }
        }
        #[cfg(not(unix))]
        if self.socket_path.exists() {
            let _ = std::fs::remove_file(&self.socket_path);
        }
    }
}

#[cfg(test)]
#[path = "server_tests.rs"]
mod tests;
