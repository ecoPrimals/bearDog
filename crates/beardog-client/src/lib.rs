// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

//! # `BearDog` Client Library (Tower Atomic Edition)
//!
//! **EVOLVED**: Now uses Tower Atomic (Unix sockets + JSON-RPC) instead of HTTP!
//!
//! Easy integration with `BearDog`'s lineage API via **Pure Rust IPC**.
//!
//! ## Evolution
//!
//! - **Before**: HTTP client using reqwest (had ring dependency via rustls)
//! - **After**: Tower Atomic client using Unix sockets (100% Pure Rust!)
//!
//! ## Example
//!
//! ```rust,no_run
//! use beardog_client::BearDogClient;
//! use serde_json::json;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to BearDog via Unix socket (not HTTP!)
//!     let mut client = BearDogClient::connect().await?;
//!     
//!     // Create genesis lineage
//!     let genesis = client
//!         .create_lineage("tower", None)
//!         .await?;
//!     
//!     println!("Created lineage: {}", genesis["lineage_id"]);
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────┐                          ┌─────────────┐
//! │  Your App    │   Unix Socket JSON-RPC   │   `BearDog`   │
//! │  (Client)    │ ──────────────────────>  │   (Server)  │
//! └──────────────┘                          └─────────────┘
//!                                                  ↓
//!                                            Lineage API
//!                                            (create, verify, extend)
//! ```

use beardog_tower_atomic::Client as AtomicClient;
use serde_json::{Value, json};
use tracing::{debug, info};

mod error;
pub use error::{BearDogClientError, ClientResult};

use beardog_genetics::birdsong::{LineageProof, types::LineageMetadata};

/// `BearDog` client for lineage API (Tower Atomic edition)
///
/// Uses Unix sockets + JSON-RPC instead of HTTP for 100% Pure Rust communication.
pub struct BearDogClient {
    /// Tower Atomic client
    client: AtomicClient,
}

impl BearDogClient {
    /// Connect to `BearDog` via Tower Atomic (Unix socket discovery)
    ///
    /// Automatically discovers `BearDog`'s Unix socket path.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog_client::BearDogClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BearDogClient::connect().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`BearDogClientError::ConnectionFailed`] when the Unix socket cannot be opened or the handshake fails.
    pub async fn connect() -> ClientResult<Self> {
        info!("🔌 Connecting to BearDog via Tower Atomic");

        let client = AtomicClient::connect("beardog")
            .await
            .map_err(|e| BearDogClientError::ConnectionFailed(e.to_string()))?;

        info!("✅ Connected to BearDog via Unix socket");

        Ok(Self { client })
    }

    /// Create a new genesis lineage
    ///
    /// # Arguments
    ///
    /// * `service_type` - Type of service (opaque label, e.g. deployment role from your registry)
    /// * `metadata` - Optional metadata for the root node
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_client::BearDogClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = BearDogClient::connect().await?;
    /// let genesis = client.create_lineage("tower", None).await?;
    /// println!("Created: {}", genesis["lineage_id"]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`BearDogClientError::ApiError`] when the JSON-RPC call fails.
    pub async fn create_lineage(
        &mut self,
        service_type: &str,
        metadata: Option<LineageMetadata>,
    ) -> ClientResult<Value> {
        info!("🌱 Creating genesis lineage for: {}", service_type);

        let params = json!({
            "service_type": service_type,
            "metadata": metadata,
        });

        let response = self
            .client
            .call("lineage.create", params)
            .await
            .map_err(|e| BearDogClientError::ApiError(e.to_string()))?;

        debug!("✅ Created lineage: {:?}", response);
        Ok(response)
    }

    /// Verify a lineage proof
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_client::BearDogClient;
    /// # use beardog_genetics::birdsong::LineageProof;
    /// # async fn example(proof: LineageProof) -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = BearDogClient::connect().await?;
    /// let verification = client.verify_lineage(&proof).await?;
    ///
    /// if verification["valid"].as_bool().unwrap_or(false) {
    ///     println!("✅ Valid lineage!");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`BearDogClientError::ApiError`] when the JSON-RPC call fails.
    pub async fn verify_lineage(&mut self, proof: &LineageProof) -> ClientResult<Value> {
        info!("🔍 Verifying lineage proof");

        let params = json!({
            "proof": proof,
        });

        let response = self
            .client
            .call("lineage.verify", params)
            .await
            .map_err(|e| BearDogClientError::ApiError(e.to_string()))?;

        debug!("✅ Verification result: {:?}", response);
        Ok(response)
    }

    /// Extend an existing lineage
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_client::BearDogClient;
    /// # async fn example(lineage_id: String, parent_id: String) -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = BearDogClient::connect().await?;
    /// let extended = client.extend_lineage(&lineage_id, &parent_id, None).await?;
    /// println!("Extended: {}", extended["node_id"]);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`BearDogClientError::ApiError`] when the JSON-RPC call fails.
    pub async fn extend_lineage(
        &mut self,
        lineage_id: &str,
        parent_id: &str,
        metadata: Option<LineageMetadata>,
    ) -> ClientResult<Value> {
        info!("🌿 Extending lineage: {}", lineage_id);

        let params = json!({
            "lineage_id": lineage_id,
            "parent_id": parent_id,
            "metadata": metadata,
        });

        let response = self
            .client
            .call("lineage.extend", params)
            .await
            .map_err(|e| BearDogClientError::ApiError(e.to_string()))?;

        debug!("✅ Extended lineage: {:?}", response);
        Ok(response)
    }

    /// Get lineage history
    ///
    /// # Errors
    ///
    /// Returns [`BearDogClientError::ApiError`] when the JSON-RPC call fails.
    pub async fn get_lineage(&mut self, lineage_id: &str) -> ClientResult<Value> {
        info!("📜 Getting lineage: {}", lineage_id);

        let params = json!({
            "lineage_id": lineage_id,
        });

        let response = self
            .client
            .call("lineage.get", params)
            .await
            .map_err(|e| BearDogClientError::ApiError(e.to_string()))?;

        debug!("✅ Retrieved lineage: {:?}", response);
        Ok(response)
    }
}

// Legacy type aliases for backward compatibility
/// Create lineage response (now returns generic Value)
pub type CreateLineageResponse = Value;

/// Verification response (now returns generic Value)
pub type VerificationResponse = Value;

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_errors::process_env;
    use beardog_genetics::birdsong::LineageProof;
    use chrono::Utc;
    use serde_json::{Value, json};
    use std::sync::Arc;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixListener;
    use tokio::sync::{Mutex, Notify};

    /// Tests in this module share the global `process_env` overlay (`BEARDOG_SOCKET`).
    /// Async mutex so we can `.await` while holding the lock (no `await_holding_lock` on std mutex).
    static ENV_LOCK: Mutex<()> = Mutex::const_new(());

    fn lineage_proof_sample() -> LineageProof {
        LineageProof {
            node_id: "n1".to_string(),
            root_id: "r0".to_string(),
            path: vec![],
            proof_chain: vec![],
            merkle_root: vec![0xab; 8],
            generated_at: Utc::now(),
        }
    }

    /// RAII guard that removes env vars on drop (even on panic).
    struct EnvGuard(Vec<&'static str>);
    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for key in &self.0 {
                process_env::remove_var(key);
            }
        }
    }

    #[tokio::test]
    async fn connect_maps_missing_socket_to_connection_error() {
        let _env = ENV_LOCK.lock().await;
        let tmp = tempfile::tempdir().expect("tempdir for BearDog client test");
        let sock = tmp.path().join("beardog-missing.sock");
        process_env::set_var("BEARDOG_SOCKET", sock.to_string_lossy().as_ref());
        let _guard = EnvGuard(vec!["BEARDOG_SOCKET"]);

        let result = BearDogClient::connect().await;
        let Err(err) = result else {
            panic!("expected connection failure");
        };
        let msg = err.to_string();
        assert!(
            msg.contains("Primal not found") || msg.contains("Connection failed"),
            "unexpected error: {msg}"
        );
    }

    #[tokio::test]
    async fn lineage_methods_round_trip_over_json_rpc() {
        let _env = ENV_LOCK.lock().await;
        let tmp = tempfile::tempdir().expect("tempdir for BearDog client test");
        let socket_path = tmp.path().join("beardog-round-trip.sock");

        let listener =
            UnixListener::bind(&socket_path).expect("bind test Unix socket for mock server");
        let ready = Arc::new(Notify::new());
        let ready_tx = Arc::clone(&ready);

        tokio::spawn(async move {
            ready_tx.notify_one();
            let (mut stream, _) = listener
                .accept()
                .await
                .expect("mock server accept lineage round-trip client");
            let (mut read_half, mut write_half) = stream.split();
            let mut reader = BufReader::new(&mut read_half);
            loop {
                let mut line = String::new();
                let n = reader
                    .read_line(&mut line)
                    .await
                    .expect("mock server read JSON-RPC line");
                if n == 0 {
                    break;
                }
                let req: Value =
                    serde_json::from_str(line.trim()).expect("mock server parse JSON-RPC request");
                let id = req["id"].clone();
                let method = req["method"]
                    .as_str()
                    .expect("JSON-RPC request must have string method");
                let result = match method {
                    "lineage.create" => json!({ "lineage_id": "L-genesis" }),
                    "lineage.verify" => json!({ "valid": true }),
                    "lineage.extend" => json!({ "node_id": "child-1" }),
                    "lineage.get" => json!({ "lineage_id": "L1", "nodes": [] }),
                    _ => panic!("unexpected method {method}"),
                };
                let response = json!({
                    "jsonrpc": "2.0",
                    "result": result,
                    "id": id,
                });
                let payload =
                    serde_json::to_string(&response).expect("serialize JSON-RPC response");
                write_half
                    .write_all(payload.as_bytes())
                    .await
                    .expect("write JSON-RPC response body");
                write_half
                    .write_all(b"\n")
                    .await
                    .expect("write JSON-RPC newline");
            }
        });

        ready.notified().await;
        process_env::set_var("BEARDOG_SOCKET", socket_path.to_string_lossy().as_ref());
        let _guard = EnvGuard(vec!["BEARDOG_SOCKET"]);

        let mut client = BearDogClient::connect()
            .await
            .expect("connect to mock round-trip socket");
        let created = client
            .create_lineage("tower", None)
            .await
            .expect("create_lineage round-trip");
        assert_eq!(created["lineage_id"], "L-genesis");

        let verified = client
            .verify_lineage(&lineage_proof_sample())
            .await
            .expect("verify_lineage round-trip");
        assert_eq!(verified["valid"], true);

        let extended = client
            .extend_lineage("L1", "parent", None)
            .await
            .expect("extend_lineage round-trip");
        assert_eq!(extended["node_id"], "child-1");

        let got = client
            .get_lineage("L1")
            .await
            .expect("get_lineage round-trip");
        assert_eq!(got["lineage_id"], "L1");
    }

    #[tokio::test]
    async fn lineage_create_and_extend_with_metadata_serializes_in_params() {
        let _env = ENV_LOCK.lock().await;
        let tmp = tempfile::tempdir().expect("tempdir for BearDog client test");
        let socket_path = tmp.path().join("beardog-meta.sock");

        let listener =
            UnixListener::bind(&socket_path).expect("bind test Unix socket for mock server");
        let ready = Arc::new(Notify::new());
        let ready_tx = Arc::clone(&ready);

        tokio::spawn(async move {
            ready_tx.notify_one();
            let (mut stream, _) = listener
                .accept()
                .await
                .expect("mock server accept metadata test client");
            let (mut read_half, mut write_half) = stream.split();
            let mut reader = BufReader::new(&mut read_half);
            for _ in 0..2 {
                let mut line = String::new();
                reader
                    .read_line(&mut line)
                    .await
                    .expect("mock server read metadata JSON-RPC line");
                let req: Value = serde_json::from_str(line.trim())
                    .expect("mock server parse metadata JSON-RPC request");
                let id = req["id"].clone();
                let method = req["method"]
                    .as_str()
                    .expect("JSON-RPC request must have string method");
                let params = &req["params"];
                let result = match method {
                    "lineage.create" => {
                        assert_eq!(params["service_type"], "tower");
                        assert!(
                            !params["metadata"].is_null(),
                            "metadata should be present in JSON-RPC params"
                        );
                        assert_eq!(params["metadata"]["biome_type"], "test-biome");
                        json!({ "lineage_id": "L-with-meta" })
                    }
                    "lineage.extend" => {
                        assert_eq!(params["lineage_id"], "L-with-meta");
                        assert_eq!(params["parent_id"], "root");
                        assert_eq!(params["metadata"]["trust_level"], 0.5);
                        json!({ "node_id": "child-meta" })
                    }
                    _ => panic!("unexpected method {method}"),
                };
                let response = json!({
                    "jsonrpc": "2.0",
                    "result": result,
                    "id": id,
                });
                let payload =
                    serde_json::to_string(&response).expect("serialize metadata JSON-RPC response");
                write_half
                    .write_all(payload.as_bytes())
                    .await
                    .expect("write metadata JSON-RPC response");
                write_half
                    .write_all(b"\n")
                    .await
                    .expect("write metadata JSON-RPC newline");
            }
        });

        ready.notified().await;
        process_env::set_var("BEARDOG_SOCKET", socket_path.to_string_lossy().as_ref());
        let _guard = EnvGuard(vec!["BEARDOG_SOCKET"]);

        let mut meta = LineageMetadata::default();
        meta.biome_type = Some("test-biome".to_string());
        meta.trust_level = 0.5;

        let mut client = BearDogClient::connect()
            .await
            .expect("connect to mock metadata socket");
        let created = client
            .create_lineage("tower", Some(meta.clone()))
            .await
            .expect("create_lineage with metadata");
        assert_eq!(created["lineage_id"], "L-with-meta");

        let extended = client
            .extend_lineage("L-with-meta", "root", Some(meta))
            .await
            .expect("extend_lineage with metadata");
        assert_eq!(extended["node_id"], "child-meta");
    }

    #[tokio::test]
    async fn api_error_maps_json_rpc_fault() {
        let _env = ENV_LOCK.lock().await;
        let tmp = tempfile::tempdir().expect("tempdir for BearDog client test");
        let socket_path = tmp.path().join("beardog-err.sock");

        let listener =
            UnixListener::bind(&socket_path).expect("bind test Unix socket for mock server");
        let ready = Arc::new(Notify::new());
        let ready_tx = Arc::clone(&ready);

        tokio::spawn(async move {
            ready_tx.notify_one();
            let (mut stream, _) = listener
                .accept()
                .await
                .expect("mock server accept fault test client");
            let (mut read_half, mut write_half) = stream.split();
            let mut reader = BufReader::new(&mut read_half);
            let mut line = String::new();
            reader
                .read_line(&mut line)
                .await
                .expect("mock server read fault-test request");
            let response = json!({
                "jsonrpc": "2.0",
                "error": { "code": -32601, "message": "method not found" },
                "id": 1
            });
            let err_body = serde_json::to_string(&response).expect("serialize error response");
            write_half
                .write_all(err_body.as_bytes())
                .await
                .expect("write JSON-RPC error response");
            write_half
                .write_all(b"\n")
                .await
                .expect("write JSON-RPC error newline");
        });

        ready.notified().await;
        process_env::set_var("BEARDOG_SOCKET", socket_path.to_string_lossy().as_ref());
        let _guard = EnvGuard(vec!["BEARDOG_SOCKET"]);

        let mut client = BearDogClient::connect()
            .await
            .expect("connect to mock fault socket");
        let result = client.create_lineage("x", None).await;
        let Err(err) = result else {
            panic!("expected API error");
        };
        let msg = err.to_string();
        assert!(
            msg.contains("method not found") || msg.contains("Api error"),
            "unexpected: {msg}"
        );
    }
}
