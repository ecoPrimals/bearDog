// SPDX-License-Identifier: AGPL-3.0-only

//! # BearDog Tower Atomic
//!
//! **Tower Atomic** = Unix socket-based JSON-RPC for inter-primal communication
//!
//! ## Purpose
//!
//! BearDog is a **Pure Rust crypto primal** with ZERO network dependencies.
//! When BearDog needs HTTP/TLS, it delegates to Songbird via Tower Atomic.
//!
//! ## Philosophy
//!
//! - **BearDog**: Crypto only (ed25519, x25519, chacha20, blake3)
//! - **Songbird**: TLS/HTTP gateway (Pure Rust, 95% complete)
//! - **Tower Atomic**: Inter-primal glue (Unix sockets, JSON-RPC)
//!
//! ## Example
//!
//! ```rust,no_run
//! use beardog_tower_atomic::Client;
//! use serde_json::json;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Peer is chosen by operator (env / capability discovery), not by hardcoded primal name.
//!     let peer_socket_name = std::env::var("TOWER_ATOMIC_PEER").unwrap_or_default();
//!     let mut peer = Client::connect(&peer_socket_name).await?;
//!
//!     let response = peer.call("http.get", json!({
//!         "url": "https://api.example.com/data",
//!         "headers": { "Authorization": "Bearer ..." }
//!     })).await?;
//!
//!     println!("Status: {}", response["status"]);
//!     println!("Body: {}", response["body"]);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────┐                          ┌─────────────┐
//! │  This primal │  Unix Socket JSON-RPC   │ HTTP peer   │
//! │   (Crypto)   │ ───────────────────────> │ (discovered)│
//! └─────────────┘                          └─────────────┘
//!       ↓                                         ↓
//!   Ed25519, X25519                        HTTPS to external
//!   ChaCha20, Blake3                       APIs, AI providers
//! ```

use serde::Deserialize;
use serde_json::{Value, json};
use std::path::Path;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tracing::{debug, info, warn};

mod error;
pub use error::{Error, Result};

mod discovery;
use discovery::discover_primal_socket;

/// Tower Atomic client for inter-primal communication
///
/// Uses Unix sockets and JSON-RPC 2.0 for fast, secure IPC.
pub struct Client {
    /// Unix socket stream
    stream: UnixStream,
    /// Target primal name
    primal_name: String,
    /// Request ID counter
    request_id: u64,
}

impl Client {
    /// Connect to a primal via Unix socket
    ///
    /// # Discovery
    ///
    /// Automatically discovers the primal's Unix socket path:
    /// 1. Check XDG_RUNTIME_DIR (/run/user/1000/ecoPrimals/)
    /// 2. Check HOME (~/.local/share/ecoPrimals/)
    /// 3. Check /var/run/ecoPrimals/
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog_tower_atomic::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let peer = Client::connect(&std::env::var("TOWER_ATOMIC_PEER").unwrap_or_default()).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect(primal_name: &str) -> Result<Self> {
        info!(
            "🔌 Connecting via Tower Atomic (socket key: {})",
            primal_name
        );

        // Discover primal's Unix socket
        let socket_path = discover_primal_socket(primal_name).await?;
        debug!("📍 Found socket at: {:?}", socket_path);

        Self::connect_unix_path(&socket_path, primal_name).await
    }

    /// Connect to an existing Unix socket path (capability / env resolved).
    ///
    /// `peer_label` is used only for logging (e.g. socket path or discovery id).
    pub async fn connect_unix_path(socket_path: &Path, peer_label: &str) -> Result<Self> {
        let stream = UnixStream::connect(socket_path).await.map_err(|e| {
            Error::ConnectionFailed(format!(
                "Failed to connect Tower Atomic peer `{peer_label}` at {}: {e}",
                socket_path.display()
            ))
        })?;

        info!("✅ Tower Atomic connected ({peer_label})");

        Ok(Self {
            stream,
            primal_name: peer_label.to_string(),
            request_id: 0,
        })
    }

    /// Call a method via JSON-RPC 2.0
    ///
    /// # Arguments
    ///
    /// * `method` - Method name (e.g., "http.get", "crypto.sign")
    /// * `params` - Method parameters as JSON value
    ///
    /// # Returns
    ///
    /// The JSON-RPC `result` field on success.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog_tower_atomic::Client;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut peer = Client::connect(&std::env::var("TOWER_ATOMIC_PEER").unwrap_or_default()).await?;
    ///
    ///     let response = peer.call("http.get", json!({
    ///         "url": "https://api.example.com"
    ///     })).await?;
    ///
    ///     println!("Response: {:?}", response);
    ///     Ok(())
    /// }
    /// ```
    pub async fn call(&mut self, method: &str, params: Value) -> Result<Value> {
        // Increment request ID
        self.request_id += 1;
        let id = self.request_id;

        debug!("📤 Tower Atomic JSON-RPC `{}` (id={})", method, id);

        // Build JSON-RPC 2.0 request
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": id
        });

        // Serialize request
        let request_str = serde_json::to_string(&request)
            .map_err(|e| Error::SerializationFailed(format!("Failed to serialize request: {e}")))?;

        // Send request (newline-delimited JSON)
        self.stream
            .write_all(request_str.as_bytes())
            .await
            .map_err(|e| Error::ConnectionFailed(format!("Failed to write request: {e}")))?;
        self.stream
            .write_all(b"\n")
            .await
            .map_err(|e| Error::ConnectionFailed(format!("Failed to write newline: {e}")))?;

        debug!("📤 Sent: {}", request_str);

        // Read response (newline-delimited JSON)
        let mut reader = BufReader::new(&mut self.stream);
        let mut response_str = String::new();
        reader
            .read_line(&mut response_str)
            .await
            .map_err(|e| Error::ConnectionFailed(format!("Failed to read response: {e}")))?;

        debug!("📥 Received: {}", response_str.trim());

        // Parse response
        let response: JsonRpcResponse = serde_json::from_str(&response_str)
            .map_err(|e| Error::SerializationFailed(format!("Failed to parse response: {e}")))?;

        // Check for JSON-RPC error
        if let Some(error) = response.error {
            warn!(
                "⚠️  JSON-RPC error from {}: {} (code {})",
                self.primal_name, error.message, error.code
            );
            return Err(Error::JsonRpcError {
                code: error.code,
                message: error.message,
                data: error.data,
            });
        }

        // Return result
        match response.result {
            Some(result) => {
                debug!("✅ Call successful (id={})", id);
                Ok(result)
            }
            None => Err(Error::JsonRpcError {
                code: -32603,
                message: "No result or error in JSON-RPC response".to_string(),
                data: None,
            }),
        }
    }

    /// Get the connected primal name
    pub fn primal_name(&self) -> &str {
        &self.primal_name
    }
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Deserialize)]
struct JsonRpcResponse {
    #[serde(rename = "jsonrpc")]
    _jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
    #[serde(rename = "id")]
    _id: Value,
}

/// JSON-RPC 2.0 Error
#[derive(Debug, Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::Arc;
    use tempfile::tempdir;
    use tokio::net::UnixListener;
    use tokio::sync::Notify;

    #[tokio::test]
    async fn test_connect_to_mock_primal() {
        // Create temporary Unix socket
        let dir = tempdir().unwrap();
        let socket_path = dir.path().join("mock_primal.sock");

        // Start mock server
        let listener = UnixListener::bind(&socket_path).unwrap();
        let server_socket_path = socket_path.clone();

        let ready = Arc::new(Notify::new());
        tokio::spawn({
            let ready_tx = Arc::clone(&ready);
            async move {
                ready_tx.notify_one();
                let (mut stream, _) = listener.accept().await.unwrap();
                use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

                let mut reader = BufReader::new(&mut stream);
                let mut request = String::new();
                reader.read_line(&mut request).await.unwrap();

                // Echo back success response
                let response = json!({
                    "jsonrpc": "2.0",
                    "result": { "status": "ok" },
                    "id": 1
                });
                stream
                    .write_all(serde_json::to_string(&response).unwrap().as_bytes())
                    .await
                    .unwrap();
                stream.write_all(b"\n").await.unwrap();
            }
        });

        ready.notified().await;

        // Connect directly to socket (bypass discovery for test)
        let stream = UnixStream::connect(&server_socket_path).await.unwrap();
        let mut client = Client {
            stream,
            primal_name: "mock_primal".to_string(),
            request_id: 0,
        };

        // Call method
        let response = client
            .call("test.method", json!({"param": "value"}))
            .await
            .unwrap();

        assert_eq!(response["status"], "ok");
    }
}
