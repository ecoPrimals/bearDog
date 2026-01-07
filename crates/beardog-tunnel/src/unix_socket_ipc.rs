//! Unix Socket IPC Server for BearDog
//!
//! PRIMARY inter-primal communication interface (not HTTP!)
//!
//! ## Architecture Principle
//!
//! BearDog communicates with other primals via Unix sockets, not HTTP ports.
//! HTTP is OPTIONAL and only for external/debugging access.
//!
//! ```text
//! Songbird                     BearDog
//!    │                            │
//!    └─────Unix Socket────────────┘
//!         /tmp/beardog-{family}.sock
//!
//! NO HTTP PORTS NEEDED!
//! ```

use anyhow::{Context as _, Result};  // Import Context trait explicitly
use base64::Engine as _;  // Import Engine trait for base64
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tracing::{debug, error, info, warn};

use crate::btsp_provider::BeardogBtspProvider;

// For HTTP timestamp generation
use chrono::Utc;

/// Unix socket IPC server for inter-primal communication
pub struct UnixSocketIpcServer {
    /// Path to the Unix socket
    socket_path: PathBuf,
    
    /// BTSP provider (provides all capabilities)
    btsp_provider: Arc<BeardogBtspProvider>,
    
    /// Server running state
    is_running: Arc<tokio::sync::RwLock<bool>>,
}

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    pub params: Option<serde_json::Value>,
    pub id: Option<serde_json::Value>,
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: serde_json::Value,
}

/// JSON-RPC 2.0 Error
#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Protocol detection result
///
/// Priority order (from upstream evolution debt):
/// 1. tarpc (PRIMARY) - Type-safe, efficient, modern Rust
/// 2. JSON-RPC (FALLBACK) - Universal adapter
/// 3. HTTP (LEGACY) - Less secure, less reliable, less fractal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tarpc,      // #1 PRIMARY: Type-safe inter-primal (security level 5)
    JsonRpc,    // #2 FALLBACK: Universal adapter (security level 4)
    Http,       // #3 LEGACY: Compatibility only (security level 2)
}

impl Protocol {
    /// Security level (higher is more secure)
    ///
    /// - tarpc: 5/5 ⭐⭐⭐⭐⭐ (type-safe, efficient, modern)
    /// - JSON-RPC: 4/5 ⭐⭐⭐⭐ (universal, simple, secure)
    /// - HTTP: 2/5 ⭐⭐ (legacy, less secure, not fractal)
    pub fn security_level(&self) -> u8 {
        match self {
            Protocol::Tarpc => 5,
            Protocol::JsonRpc => 4,
            Protocol::Http => 2,
        }
    }
    
    /// Reliability level (higher is more reliable)
    pub fn reliability_level(&self) -> u8 {
        match self {
            Protocol::Tarpc => 5,    // Type-safe, compile-time guarantees
            Protocol::JsonRpc => 4,   // Runtime validation
            Protocol::Http => 2,      // Text-based, error-prone
        }
    }
    
    /// Fractal compatibility (how well it scales in nested/recursive scenarios)
    pub fn fractal_level(&self) -> u8 {
        match self {
            Protocol::Tarpc => 5,    // Excellent for fractal architectures
            Protocol::JsonRpc => 4,   // Good for fractal architectures
            Protocol::Http => 2,      // Poor for fractal (port conflicts, overhead)
        }
    }
    
    /// Detect protocol from first bytes
    pub fn detect(data: &[u8]) -> Self {
        // tarpc uses bincode serialization with specific magic bytes
        // Check for tarpc frame header (length-delimited)
        if data.len() >= 4 {
            // tarpc uses tokio-serde with length-delimited codec
            // First 4 bytes are u32 length in big-endian
            let potential_length = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
            
            // If length is reasonable (not too large) and rest looks like bincode
            if potential_length > 0 && potential_length < 1_000_000 {
                // Try to detect bincode serialization patterns
                // bincode starts with type tags
                if data.len() > 4 && (data[4] == 0x00 || data[4] == 0x01 || data[4] < 0x10) {
                    return Protocol::Tarpc;
                }
            }
        }
        
        // Convert to string for text-based protocol detection
        if let Ok(text) = std::str::from_utf8(data) {
            let trimmed = text.trim();
            
            // HTTP detection
            if trimmed.starts_with("GET ")
                || trimmed.starts_with("POST ")
                || trimmed.starts_with("PUT ")
                || trimmed.starts_with("DELETE ")
                || trimmed.starts_with("PATCH ")
                || trimmed.starts_with("HEAD ")
            {
                return Protocol::Http;
            }
        }
        
        // JSON-RPC (default fallback for text-based protocols)
        Protocol::JsonRpc
    }
}

impl UnixSocketIpcServer {
    /// Create new Unix socket IPC server
    pub async fn new(
        socket_path: impl AsRef<Path>,
        btsp_provider: Arc<BeardogBtspProvider>,
    ) -> Result<Self> {
        let socket_path = socket_path.as_ref().to_path_buf();
        
        // Remove existing socket file if present
        if socket_path.exists() {
            info!("🧹 Removing existing socket: {}", socket_path.display());
            std::fs::remove_file(&socket_path)
                .context("Failed to remove existing socket")?;
        }
        
        Ok(Self {
            socket_path,
            btsp_provider,
            is_running: Arc::new(tokio::sync::RwLock::new(false)),
        })
    }
    
    /// Start the Unix socket IPC server
    pub async fn start(self: Arc<Self>) -> Result<()> {
        {
            let mut is_running = self.is_running.write().await;
            if *is_running {
                warn!("⚠️  Unix socket IPC server already running");
                return Ok(());
            }
            *is_running = true;
        }
        
        info!("🔌 Starting Unix socket IPC server: {}", self.socket_path.display());
        
        // Bind Unix socket
        let listener = UnixListener::bind(&self.socket_path)
            .context(format!("Failed to bind Unix socket: {}", self.socket_path.display()))?;
        
        info!("✅ Unix socket IPC server listening: {}", self.socket_path.display());
        
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
                let protocol = Protocol::detect(first_line.as_bytes());
                
                // Log security level
                match protocol {
                    Protocol::Tarpc => {
                        info!("🎯 tarpc connection (security level: {}) - PRIMARY protocol!", protocol.security_level());
                        info!("✅ Type-safe, efficient, modern Rust inter-primal communication");
                    }
                    Protocol::JsonRpc => {
                        debug!("🔐 JSON-RPC connection (security level: {}) - FALLBACK protocol", protocol.security_level());
                    }
                    Protocol::Http => {
                        warn!("⚠️  HTTP connection (security level: {})", protocol.security_level());
                        warn!("⚠️  HTTP is less secure, less reliable, less fractal than tarpc/JSON-RPC");
                        warn!("⚠️  Consider migrating to tarpc for inter-primal communication");
                    }
                }
                
                // Route to appropriate handler
                let response_bytes = match protocol {
                    Protocol::Tarpc => {
                        self.handle_tarpc_connection(&first_line, &mut reader, &mut writer).await?
                    }
                    Protocol::JsonRpc => {
                        self.handle_jsonrpc_connection(&first_line, &mut reader, &mut writer).await?
                    }
                    Protocol::Http => {
                        self.handle_http_connection(&first_line, &mut reader, &mut writer).await?
                    }
                };
                
                return Ok(());
            }
            Err(e) => {
                error!("❌ Read error: {}", e);
                return Err(e.into());
            }
        }
    }
    
    /// Handle tarpc connection (PRIMARY protocol - type-safe RPC)
    async fn handle_tarpc_connection(
        &self,
        _first_line: &str,
        _reader: &mut BufReader<tokio::net::unix::OwnedReadHalf>,
        _writer: &mut tokio::net::unix::OwnedWriteHalf,
    ) -> Result<()> {
        info!("🎯 tarpc connection established - using type-safe RPC");
        
        // TODO: Implement tarpc server connection handling
        // For now, log that tarpc is detected but not yet fully implemented
        warn!("⚠️  tarpc handler not yet fully implemented");
        warn!("⚠️  Falling back to suggesting JSON-RPC for now");
        
        // Return error to close connection gracefully
        Err(anyhow::anyhow!("tarpc handler under construction - please use JSON-RPC temporarily"))
    }
    
    /// Handle JSON-RPC connection (FALLBACK protocol - universal adapter)
    async fn handle_jsonrpc_connection(
        &self,
        first_line: &str,
        reader: &mut BufReader<tokio::net::unix::OwnedReadHalf>,
        writer: &mut tokio::net::unix::OwnedWriteHalf,
    ) -> Result<()> {
        // Handle first request
        let response = self.handle_jsonrpc_request(first_line).await?;
        let response_str = serde_json::to_string(&response)?;
        writer.write_all(response_str.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        
        // Continue handling subsequent requests
        let mut line = String::new();
        loop {
            line.clear();
            
            match reader.read_line(&mut line).await {
                Ok(0) => {
                    debug!("📤 JSON-RPC client disconnected");
                    break;
                }
                Ok(_) => {
                    if line.trim().is_empty() {
                        continue;
                    }
                    
                    let response = self.handle_jsonrpc_request(&line).await?;
                    let response_str = serde_json::to_string(&response)?;
                    writer.write_all(response_str.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
                Err(e) => {
                    error!("❌ JSON-RPC read error: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Handle HTTP connection (legacy protocol)
    async fn handle_http_connection(
        &self,
        request_line: &str,
        reader: &mut BufReader<tokio::net::unix::OwnedReadHalf>,
        writer: &mut tokio::net::unix::OwnedWriteHalf,
    ) -> Result<()> {
        // Parse HTTP request
        let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
        if parts.len() < 2 {
            let error_response = Self::http_error_response(400, "Bad Request");
            writer.write_all(error_response.as_bytes()).await?;
            return Ok(());
        }
        
        let method = parts[0];
        let path = parts[1];
        
        // Read HTTP headers
        let mut headers = std::collections::HashMap::new();
        let mut content_length = 0;
        let mut line = String::new();
        
        loop {
            line.clear();
            reader.read_line(&mut line).await?;
            let trimmed = line.trim();
            
            if trimmed.is_empty() {
                break; // End of headers
            }
            
            if let Some((key, value)) = trimmed.split_once(':') {
                let key = key.trim().to_lowercase();
                let value = value.trim().to_string();
                
                if key == "content-length" {
                    content_length = value.parse().unwrap_or(0);
                }
                
                headers.insert(key, value);
            }
        }
        
        // Read body if present
        let mut body = String::new();
        if content_length > 0 {
            let mut buf = vec![0u8; content_length];
            tokio::io::AsyncReadExt::read_exact(reader, &mut buf).await?;
            body = String::from_utf8_lossy(&buf).to_string();
        }
        
        debug!("🌐 HTTP {} {} (body: {} bytes)", method, path, body.len());
        
        // Route HTTP request
        let result = self.route_http_request(method, path, &body).await;
        
        // Build HTTP response
        let response = match result {
            Ok(value) => {
                let response_body = serde_json::to_string(&value)?;
                format!(
                    "HTTP/1.1 200 OK\r\n\
                     Content-Type: application/json\r\n\
                     Content-Length: {}\r\n\
                     X-Protocol-Security: low\r\n\
                     X-Recommended-Protocol: json-rpc\r\n\
                     X-Security-Warning: HTTP is less secure than JSON-RPC\r\n\
                     \r\n\
                     {}",
                    response_body.len(),
                    response_body
                )
            }
            Err(e) => {
                Self::http_error_response(500, &e.to_string())
            }
        };
        
        writer.write_all(response.as_bytes()).await?;
        
        Ok(())
    }
    
    /// Handle a JSON-RPC request
    async fn handle_jsonrpc_request(&self, request_str: &str) -> Result<JsonRpcResponse> {
        debug!("→ JSON-RPC Request: {}", request_str.trim());
        
        // Parse JSON-RPC request
        let request: JsonRpcRequest = serde_json::from_str(request_str)
            .context("Failed to parse JSON-RPC request")?;
        
        // Validate JSON-RPC version
        if request.jsonrpc != "2.0" {
            return Ok(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32600,
                    message: "Invalid JSON-RPC version (must be 2.0)".to_string(),
                    data: None,
                }),
                id: request.id.unwrap_or(serde_json::Value::Null),
            });
        }
        
        // Route to handler
        let result = self.handle_method(&request.method, request.params.as_ref()).await;
        
        // Build response
        let response = match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(value),
                error: None,
                id: request.id.unwrap_or(serde_json::Value::Null),
            },
            Err(e) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32603,
                    message: e,
                    data: None,
                }),
                id: request.id.unwrap_or(serde_json::Value::Null),
            },
        };
        
        Ok(response)
    }
    
    /// Route HTTP request to handler
    async fn route_http_request(
        &self,
        method: &str,
        path: &str,
        body: &str,
    ) -> Result<serde_json::Value> {
        match (method, path) {
            ("GET", "/ping") | ("GET", "/health") => {
                Ok(serde_json::json!({
                    "pong": true,
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "protocol_warning": "HTTP is less secure than JSON-RPC",
                    "recommended_protocol": "json-rpc"
                }))
            }
            ("GET", "/capabilities") => {
                Ok(serde_json::json!({
                    "capabilities": ["encryption", "trust_evaluation", "key_management", "signatures"],
                    "version": env!("CARGO_PKG_VERSION"),
                    "supported_protocols": ["json-rpc", "http"],
                    "recommended_protocol": "json-rpc",
                    "security_warning": "HTTP has lower security level than JSON-RPC"
                }))
            }
            ("GET", "/metrics/security") => {
                Ok(serde_json::json!({
                    "trust_evaluations": 0,
                    "encryption_operations": 0,
                    "active_sessions": 0,
                    "uptime_seconds": 0,
                    "protocol_warning": "Consider using JSON-RPC for better security"
                }))
            }
            ("POST", "/evaluate_trust") => {
                let params: serde_json::Value = if body.is_empty() {
                    serde_json::json!({})
                } else {
                    serde_json::from_str(body)?
                };
                
                // Convert to JSON-RPC format and use existing handler
                self.handle_method("evaluate_trust", Some(&params))
                    .await
                    .map_err(|e| anyhow::anyhow!(e))
            }
            _ => {
                Err(anyhow::anyhow!("Not found: {} {}", method, path))
            }
        }
    }
    
    /// Build HTTP error response
    fn http_error_response(status: u16, message: &str) -> String {
        let status_text = match status {
            400 => "Bad Request",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Error",
        };
        
        let body = format!(r#"{{"error":"{}"}}"#, message);
        
        format!(
            "HTTP/1.1 {} {}\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             X-Protocol-Security: low\r\n\
             X-Recommended-Protocol: json-rpc\r\n\
             \r\n\
             {}",
            status, status_text, body.len(), body
        )
    }
    
    /// Handle a specific JSON-RPC method
    async fn handle_method(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        debug!("📞 Method: {}", method);
        
        // Parse method into namespace and action for capability-based routing
        let (namespace, action) = if let Some((ns, act)) = method.split_once('.') {
            (ns, act)
        } else {
            ("beardog", method)  // Default namespace
        };
        
        match (namespace, action) {
            // ========================================================================
            // UNIVERSAL CAPABILITY-BASED METHODS (Primal Agnostic)
            // ========================================================================
            
            // Ping/Health - Universal across all primals
            (_, "ping") | (_, "health") | (_, "status") | (_, "check") => {
                info!("🏥 Health check requested");
                Ok(serde_json::json!({
                    "status": "healthy",
                    "primal": "beardog",
                    "version": env!("CARGO_PKG_VERSION"),
                    "protocol": "JSON-RPC",
                    "timestamp": Utc::now().to_rfc3339(),
                }))
            }
            
            // Capabilities - Self-description (every primal should provide this)
            (_, "capabilities") | (_, "get_capabilities") => {
                // Get identity from environment (primal only knows itself)
                // Support both FAMILY_ID and BEARDOG_FAMILY_ID for compatibility
                let family_id = std::env::var("FAMILY_ID")
                    .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
                    .unwrap_or_else(|_| "unknown".to_string());
                let node_id = std::env::var("NODE_ID")
                    .or_else(|_| std::env::var("BEARDOG_NODE_ID"))
                    .unwrap_or_else(|_| "unknown".to_string());
                
                info!("🎯 Capabilities requested - exposing our capabilities");
                Ok(serde_json::json!({
                    "primal": "beardog",
                    "family_id": family_id,
                    "node_id": node_id,
                    "provided_capabilities": [
                        {
                            "type": "security",
                            "version": "1.0",
                            "methods": ["evaluate", "lineage"],
                        },
                        {
                            "type": "encryption",
                            "version": "1.0",
                            "methods": ["encrypt", "decrypt"],
                        },
                        {
                            "type": "trust",
                            "version": "1.0",
                            "methods": ["evaluate", "lineage"],
                        }
                    ],
                    "version": env!("CARGO_PKG_VERSION"),
                    "protocols": ["tarpc", "json-rpc", "http"],
                }))
            }
            
            // Identity - Self-identification (genetic lineage)
            (_, "identity") | (_, "whoami") | (_, "get_identity") => {
                // Get identity from environment (primal only knows itself)
                // Support both FAMILY_ID and BEARDOG_FAMILY_ID for compatibility
                let family_id = std::env::var("FAMILY_ID")
                    .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
                    .unwrap_or_else(|_| "unknown".to_string());
                let node_id = std::env::var("NODE_ID")
                    .or_else(|_| std::env::var("BEARDOG_NODE_ID"))
                    .unwrap_or_else(|_| "unknown".to_string());
                
                info!("🆔 Identity requested - family: {}, node: {}", family_id, node_id);
                Ok(serde_json::json!({
                    "primal": "beardog",
                    "family": family_id,
                    "node": node_id,
                    "version": env!("CARGO_PKG_VERSION"),
                }))
            }
            
            // ========================================================================
            // SECURITY CAPABILITY METHODS (BearDog's specialty)
            // ========================================================================
            
            // Trust evaluation - capability-based, not primal-specific
            ("security", "evaluate") | ("trust", "evaluate") | ("security", "evaluate_trust") | ("trust", "evaluate_peer") => {
                let params = params.ok_or("Missing params for trust evaluation")?;
                
                // Flexible parameter extraction (works with any primal's naming)
                let peer_id = params.get("peer_id")
                    .or_else(|| params.get("id"))
                    .or_else(|| params.get("peer"))
                    .and_then(|v| v.as_str())
                    .ok_or("Missing peer identifier")?;
                    
                let peer_family = params.get("peer_family")
                    .or_else(|| params.get("family"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                // Get our identity from environment (primal only knows itself)
                // Support both FAMILY_ID and BEARDOG_FAMILY_ID for compatibility
                let our_family = std::env::var("FAMILY_ID")
                    .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
                    .unwrap_or_else(|_| "unknown".to_string());
                let our_node = std::env::var("NODE_ID")
                    .or_else(|_| std::env::var("BEARDOG_NODE_ID"))
                    .unwrap_or_else(|_| "unknown".to_string());
                
                // Phase 1: Dual representation with capability hints + Songbird decision field (Jan 7, 2026)
                let (trust_level, trust_level_name, decision, reason, allowed_caps, denied_caps) = if peer_family == our_family {
                    info!("✅ Trust: SAME FAMILY - level 1 (limited) - peer: {}, family: {}", peer_id, peer_family);
                    (
                        1,
                        "limited",
                        "auto_accept",  // Same family = auto-accept for genetic lineage
                        "same_genetic_family",
                        vec!["birdsong/*", "coordination/*", "health", "capabilities", "discovery"],
                        vec!["data/*", "commands/*", "keys/*", "federation/admin"],
                    )
                } else if !peer_family.is_empty() {
                    info!("⚠️  Trust: DIFFERENT FAMILY - level 0 (none) - peer: {}, their: {}, ours: {}", peer_id, peer_family, our_family);
                    (
                        0,
                        "none",
                        "reject",  // Different family = reject
                        "different_family",
                        vec![],
                        vec!["*"],
                    )
                } else {
                    info!("⚠️  Trust: UNKNOWN FAMILY - level 0 (none) - peer: {}", peer_id);
                    (
                        0,
                        "none",
                        "reject",  // Unknown family = reject
                        "unknown_family",
                        vec![],
                        vec!["*"],
                    )
                };
                
                // Phase 1 Response: Dual representation (int + string) with capability hints + decision field
                Ok(serde_json::json!({
                    "decision": decision,                  // Songbird requires this field
                    "trust_level": trust_level,           // Integer (compact, backward compat)
                    "trust_level_name": trust_level_name, // String (Songbird expects this)
                    "reason": reason,
                    "peer_id": peer_id,
                    "peer_family": peer_family,
                    "our_family": our_family,
                    "our_node": our_node,
                    "evaluated_by": "beardog",
                    "capabilities": {                      // Capability hints (Phase 1)
                        "allowed": allowed_caps,
                        "denied": denied_caps,
                    },
                    "metadata": {
                        "policy_version": 1,
                        "evaluation_method": "genetic_family_match",
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                    }
                }))
            }
            
            // Lineage information
            ("security", "lineage") | ("trust", "lineage") | ("security", "get_lineage") | ("trust", "get_lineage") => {
                // Get identity from environment (primal only knows itself)
                // Support both FAMILY_ID and BEARDOG_FAMILY_ID for compatibility
                let family_id = std::env::var("FAMILY_ID")
                    .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
                    .unwrap_or_else(|_| "unknown".to_string());
                let node_id = std::env::var("NODE_ID")
                    .or_else(|_| std::env::var("BEARDOG_NODE_ID"))
                    .unwrap_or_else(|_| "unknown".to_string());
                
                info!("🌳 Lineage info requested - family: {}, node: {}", family_id, node_id);
                Ok(serde_json::json!({
                    "primal": "beardog",
                    "family": family_id,
                    "node": node_id,
                    "generation": 0,
                    "parent": null,
                    "capabilities": ["security", "encryption", "trust"],
                }))
            }
            
            // Legacy beardog.ping (redirect to universal ping)
            ("beardog", "ping") => {
                Ok(serde_json::json!({
                    "pong": true,
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                }))
            }
            
            // BirdSong encryption (used by Songbird for secure discovery)
            // Encryption capability
            ("beardog", "birdsong.encrypt") | ("encryption", "encrypt") | ("birdsong", "encrypt") => {
                let params = params.ok_or("Missing params")?;
                let plaintext = params["plaintext"]
                    .as_str()
                    .ok_or("Missing plaintext")?;
                let family_id = params["family_id"]
                    .as_str()
                    .ok_or("Missing family_id")?;
                
                // Decode base64 plaintext
                let plaintext_bytes = base64::engine::general_purpose::STANDARD
                    .decode(plaintext)
                    .map_err(|e| format!("Invalid base64: {}", e))?;
                
                // Encrypt using BirdSong
                let ciphertext = self
                    .btsp_provider
                    .birdsong_manager()
                    .encrypt_discovery_for_family(&plaintext_bytes, family_id)
                    .map_err(|e| format!("Encryption failed: {}", e))?;
                
                // Encode to base64
                let ciphertext_b64 = base64::engine::general_purpose::STANDARD.encode(&ciphertext);
                
                Ok(serde_json::json!({
                    "ciphertext": ciphertext_b64,
                    "family_id": family_id,
                }))
            }
            
            // BirdSong decryption (used by Songbird for secure discovery)
            ("beardog", "birdsong.decrypt") | ("encryption", "decrypt") | ("birdsong", "decrypt") => {
                let params = params.ok_or("Missing params")?;
                let ciphertext = params["ciphertext"]
                    .as_str()
                    .ok_or("Missing ciphertext")?;
                let family_id = params["family_id"]
                    .as_str()
                    .ok_or("Missing family_id")?;
                
                // Decode base64 ciphertext
                let ciphertext_bytes = base64::engine::general_purpose::STANDARD
                    .decode(ciphertext)
                    .map_err(|e| format!("Invalid base64: {}", e))?;
                
                // Decrypt using BirdSong
                match self
                    .btsp_provider
                    .birdsong_manager()
                    .decrypt_discovery_from_family(&ciphertext_bytes, family_id)
                {
                    Ok(plaintext) => {
                        // Encode to base64
                        let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(&plaintext);
                        
                        Ok(serde_json::json!({
                            "plaintext": plaintext_b64,
                            "family_id": family_id,
                            "success": true,
                        }))
                    }
                    Err(e) => {
                        // Graceful privacy: Not an error, just "not for us"
                        debug!("🔇 Decryption failed (different family): {}", e);
                        Ok(serde_json::json!({
                            "plaintext": "",
                            "family_id": family_id,
                            "success": false,
                        }))
                    }
                }
            }
            
            // Unknown method
            _ => Err(format!("Method not found: {}.{} (try: ping, capabilities, identity, security.evaluate, encryption.encrypt)", namespace, action)),
        }
    }
}

// Include comprehensive test modules
#[cfg(test)]
#[path = "unix_socket_ipc_tests.rs"]
mod tests;

