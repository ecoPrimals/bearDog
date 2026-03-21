// SPDX-License-Identifier: AGPL-3.0-only
#![forbid(unsafe_code)]

//! # BearDog Client Library (Tower Atomic Edition)
//!
//! **EVOLVED**: Now uses Tower Atomic (Unix sockets + JSON-RPC) instead of HTTP!
//!
//! Easy integration with BearDog's lineage API via **Pure Rust IPC**.
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
//! │  Your App    │   Unix Socket JSON-RPC   │   BearDog   │
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

/// BearDog client for lineage API (Tower Atomic edition)
///
/// Uses Unix sockets + JSON-RPC instead of HTTP for 100% Pure Rust communication.
pub struct BearDogClient {
    /// Tower Atomic client
    client: AtomicClient,
}

impl BearDogClient {
    /// Connect to BearDog via Tower Atomic (Unix socket discovery)
    ///
    /// Automatically discovers BearDog's Unix socket path.
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
    /// * `service_type` - Type of service (e.g., "tower", "songbird")
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
    use std::sync::Mutex;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixListener;

    static CLIENT_TEST_ENV: Mutex<()> = Mutex::new(());

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

    #[tokio::test]
    async fn connect_maps_missing_socket_to_connection_error() {
        let _lock = CLIENT_TEST_ENV.lock().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        let empty_xdg = tmp.path().join("empty_xdg");
        std::fs::create_dir_all(&empty_xdg).unwrap();
        process_env::set_var("HOME", tmp.path().to_string_lossy().as_ref());
        process_env::set_var("XDG_RUNTIME_DIR", empty_xdg.to_string_lossy().as_ref());

        let result = BearDogClient::connect().await;
        let err = match result {
            Err(e) => e,
            Ok(_) => panic!("expected connection failure"),
        };
        let msg = err.to_string();
        process_env::remove_var("HOME");
        process_env::remove_var("XDG_RUNTIME_DIR");
        assert!(
            msg.contains("Primal not found") || msg.contains("Connection failed"),
            "unexpected error: {msg}"
        );
    }

    #[tokio::test]
    async fn lineage_methods_round_trip_over_json_rpc() {
        let _lock = CLIENT_TEST_ENV.lock().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        let xdg = tmp.path().join("xdg_run");
        let eco = xdg.join("ecoPrimals");
        std::fs::create_dir_all(&eco).unwrap();
        let socket_path = eco.join("beardog.sock");

        let listener = UnixListener::bind(&socket_path).unwrap();

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let (mut read_half, mut write_half) = stream.split();
            let mut reader = BufReader::new(&mut read_half);
            loop {
                let mut line = String::new();
                let n = reader.read_line(&mut line).await.unwrap();
                if n == 0 {
                    break;
                }
                let req: Value = serde_json::from_str(line.trim()).unwrap();
                let id = req["id"].clone();
                let method = req["method"].as_str().unwrap();
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
                let payload = serde_json::to_string(&response).unwrap();
                write_half.write_all(payload.as_bytes()).await.unwrap();
                write_half.write_all(b"\n").await.unwrap();
            }
        });

        tokio::time::sleep(std::time::Duration::from_millis(40)).await;

        process_env::set_var("XDG_RUNTIME_DIR", xdg.to_string_lossy().as_ref());

        let mut client = BearDogClient::connect().await.unwrap();
        let created = client.create_lineage("tower", None).await.unwrap();
        assert_eq!(created["lineage_id"], "L-genesis");

        let verified = client
            .verify_lineage(&lineage_proof_sample())
            .await
            .unwrap();
        assert_eq!(verified["valid"], true);

        let extended = client.extend_lineage("L1", "parent", None).await.unwrap();
        assert_eq!(extended["node_id"], "child-1");

        let got = client.get_lineage("L1").await.unwrap();
        assert_eq!(got["lineage_id"], "L1");

        process_env::remove_var("XDG_RUNTIME_DIR");
    }

    #[tokio::test]
    async fn lineage_create_and_extend_with_metadata_serializes_in_params() {
        let _lock = CLIENT_TEST_ENV.lock().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        let xdg = tmp.path().join("xdg_meta");
        let eco = xdg.join("ecoPrimals");
        std::fs::create_dir_all(&eco).unwrap();
        let socket_path = eco.join("beardog.sock");

        let listener = UnixListener::bind(&socket_path).unwrap();

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let (mut read_half, mut write_half) = stream.split();
            let mut reader = BufReader::new(&mut read_half);
            for _ in 0..2 {
                let mut line = String::new();
                reader.read_line(&mut line).await.unwrap();
                let req: Value = serde_json::from_str(line.trim()).unwrap();
                let id = req["id"].clone();
                let method = req["method"].as_str().unwrap();
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
                let payload = serde_json::to_string(&response).unwrap();
                write_half.write_all(payload.as_bytes()).await.unwrap();
                write_half.write_all(b"\n").await.unwrap();
            }
        });

        tokio::time::sleep(std::time::Duration::from_millis(40)).await;

        process_env::set_var("XDG_RUNTIME_DIR", xdg.to_string_lossy().as_ref());

        let mut meta = LineageMetadata::default();
        meta.biome_type = Some("test-biome".to_string());
        meta.trust_level = 0.5;

        let mut client = BearDogClient::connect().await.unwrap();
        let created = client
            .create_lineage("tower", Some(meta.clone()))
            .await
            .unwrap();
        assert_eq!(created["lineage_id"], "L-with-meta");

        let extended = client
            .extend_lineage("L-with-meta", "root", Some(meta))
            .await
            .unwrap();
        assert_eq!(extended["node_id"], "child-meta");

        process_env::remove_var("XDG_RUNTIME_DIR");
    }

    #[tokio::test]
    async fn api_error_maps_json_rpc_fault() {
        let _lock = CLIENT_TEST_ENV.lock().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        let xdg = tmp.path().join("xdg_err");
        let eco = xdg.join("ecoPrimals");
        std::fs::create_dir_all(&eco).unwrap();
        let socket_path = eco.join("beardog.sock");

        let listener = UnixListener::bind(&socket_path).unwrap();

        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let (mut read_half, mut write_half) = stream.split();
            let mut reader = BufReader::new(&mut read_half);
            let mut line = String::new();
            reader.read_line(&mut line).await.unwrap();
            let response = json!({
                "jsonrpc": "2.0",
                "error": { "code": -32601, "message": "method not found" },
                "id": 1
            });
            write_half
                .write_all(serde_json::to_string(&response).unwrap().as_bytes())
                .await
                .unwrap();
            write_half.write_all(b"\n").await.unwrap();
        });

        tokio::time::sleep(std::time::Duration::from_millis(40)).await;

        process_env::set_var("XDG_RUNTIME_DIR", xdg.to_string_lossy().as_ref());

        let mut client = BearDogClient::connect().await.unwrap();
        let result = client.create_lineage("x", None).await;
        let err = match result {
            Err(e) => e,
            Ok(_) => panic!("expected API error"),
        };
        let msg = err.to_string();
        process_env::remove_var("XDG_RUNTIME_DIR");
        assert!(
            msg.contains("method not found") || msg.contains("Api error"),
            "unexpected: {msg}"
        );
    }
}
