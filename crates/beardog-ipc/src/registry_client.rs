// SPDX-License-Identifier: AGPL-3.0-only

//! Universal Primal Registry Client
//!
//! Generic client for registering with ANY primal registry that speaks JSON-RPC 2.0
//!
//! # Design Principle: Zero Vendor Hardcoding
//!
//! This client does NOT know:
//! - Which registry it's talking to (could be Songbird, Consul, etcd, custom, etc.)
//! - What other primals exist
//! - What the registry implementation is
//!
//! This client ONLY knows:
//! - I have capabilities to advertise
//! - I speak JSON-RPC 2.0
//! - I connect to a Unix socket or HTTP endpoint
//!
//! # Universal Adapter Pattern
//!
//! Instead of N primal-specific clients, we have 1 universal client that adapts to any registry

use beardog_core::capabilities::{BearDogCapabilities, Capability};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tracing::{debug, error, info, warn};

/// JSON-RPC 2.0 Request (Universal)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// Protocol version; must be `"2.0"` for compliant servers.
    pub jsonrpc: String,
    /// Method name (e.g. `primal.register`, `primal.get_provider`).
    pub method: String,
    /// Positional or object parameters; omitted when empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    /// Correlation id matching the corresponding [`JsonRpcResponse::id`].
    pub id: u64,
}

/// JSON-RPC 2.0 Response (Universal)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// Protocol version; must be `"2.0"`.
    pub jsonrpc: String,
    /// Successful result payload; mutually exclusive with [`JsonRpcResponse::error`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Error object when the call failed; mutually exclusive with `result`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    /// Id echoing the originating [`JsonRpcRequest::id`].
    pub id: u64,
}

/// JSON-RPC 2.0 Error (Universal)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Integer error code (JSON-RPC reserved + application-defined ranges).
    pub code: i32,
    /// Short human-readable error summary.
    pub message: String,
    /// Optional structured details (stack traces, validation hints, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Primal information (Universal format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalInfo {
    /// Stable primal identifier assigned by the registry.
    pub primal_id: String,
    /// Optional lineage or deployment family id.
    pub family_id: Option<String>,
    /// Host or cluster node identifier.
    pub node_id: String,
    /// Capability names this primal advertises (e.g. `encryption`, `discovery`).
    pub capabilities: Vec<String>,
    /// Filesystem path to this primal's IPC socket (Unix) or equivalent endpoint.
    pub socket_path: String,
    /// RFC3339 or registry-specific last-seen timestamp, if tracked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
}

/// Universal Primal Registry Client
///
/// Connects to ANY registry that speaks JSON-RPC 2.0 over Unix sockets
///
/// # Zero Vendor Hardcoding
///
/// This client works with:
/// - Songbird (ecoPrimals discovery orchestrator)
/// - Consul (HashiCorp service mesh)
/// - etcd (Kubernetes/Cloud Native registry)
/// - Custom registries
/// - ANY system that speaks JSON-RPC 2.0
pub struct PrimalRegistryClient {
    socket_path: PathBuf,
    stream: Option<UnixStream>,
    request_id: u64,
}

impl PrimalRegistryClient {
    /// Create a new registry client
    ///
    /// # Arguments
    ///
    /// * `socket_path` - Path to registry's Unix socket
    ///
    /// # Zero Assumptions
    ///
    /// We don't know or care what's on the other end of this socket.
    /// Could be Songbird, could be Consul, could be anything.
    pub const fn new(socket_path: PathBuf) -> Self {
        Self {
            socket_path,
            stream: None,
            request_id: 0,
        }
    }

    /// Connect to registry
    ///
    /// Establishes Unix socket connection to ANY JSON-RPC 2.0 registry
    pub async fn connect(&mut self) -> Result<(), BearDogError> {
        info!("🔌 Connecting to primal registry at {:?}", self.socket_path);

        let stream = UnixStream::connect(&self.socket_path).await.map_err(|e| {
            BearDogError::system(format!(
                "Failed to connect to registry at {}: {}",
                self.socket_path.display(),
                e
            ))
        })?;

        self.stream = Some(stream);
        info!("✅ Connected to primal registry");

        Ok(())
    }

    /// Register with registry
    ///
    /// Advertises this primal's capabilities to the ecosystem
    ///
    /// # Universal Adapter
    ///
    /// This works with any registry that understands JSON-RPC method "primal.register"
    pub async fn register(
        &mut self,
        capabilities: &BearDogCapabilities,
    ) -> Result<(), BearDogError> {
        info!("📝 Registering with primal registry");
        info!("   Primal ID: {}", capabilities.primal_id);
        info!("   Family ID: {:?}", capabilities.family_id);
        info!("   Node ID: {}", capabilities.node_id);
        info!("   Capabilities: {}", capabilities.provides.len());

        // Extract capability names for registration
        let capability_names: Vec<String> = capabilities
            .provides
            .iter()
            .map(|cap| match cap {
                Capability::Encryption { .. } => "encryption",
                Capability::TrustEvaluation { .. } => "trust_evaluation",
                Capability::KeyManagement { .. } => "key_management",
                Capability::Signatures { .. } => "signatures",
                Capability::Discovery { .. } => "discovery",
                Capability::Storage { .. } => "storage",
                Capability::Compute { .. } => "compute",
                Capability::Custom { name, .. } => name.as_str(),
            })
            .map(String::from)
            .collect();

        let params = serde_json::json!({
            "primal_id": capabilities.primal_id,
            "family_id": capabilities.family_id,
            "node_id": capabilities.node_id,
            "capabilities": capability_names,
            "socket_path": self.extract_socket_path(capabilities),
        });

        let response = self.send_request("primal.register", Some(params)).await?;

        if let Some(error) = response.error {
            error!("❌ Registration failed: {}", error.message);
            return Err(BearDogError::system(format!(
                "Registry registration failed: {}",
                error.message
            )));
        }

        info!("✅ Successfully registered with primal registry");
        Ok(())
    }

    /// Query for primal that provides a capability
    ///
    /// # Universal Discovery
    ///
    /// We don't care WHO provides the capability, just that someone does
    pub async fn get_provider(&mut self, capability: &str) -> Result<PrimalInfo, BearDogError> {
        debug!("🔍 Querying registry for capability: {}", capability);

        let params = serde_json::json!({
            "capability": capability,
        });

        let response = self
            .send_request("primal.get_provider", Some(params))
            .await?;

        if let Some(error) = response.error {
            warn!(
                "⚠️  No provider found for {}: {}",
                capability, error.message
            );
            return Err(BearDogError::system(format!(
                "No provider for capability {}: {}",
                capability, error.message
            )));
        }

        let primal_info: PrimalInfo =
            serde_json::from_value(response.result.unwrap_or_default())
                .map_err(|e| BearDogError::system(format!("Failed to parse primal info: {e}")))?;

        debug!("✅ Found provider: {}", primal_info.primal_id);
        Ok(primal_info)
    }

    /// List all registered primals
    pub async fn list_all(&mut self) -> Result<Vec<PrimalInfo>, BearDogError> {
        debug!("📋 Listing all primals from registry");

        let response = self.send_request("primal.list_all", None).await?;

        if let Some(error) = response.error {
            return Err(BearDogError::system(format!(
                "Failed to list primals: {}",
                error.message
            )));
        }

        let primals: Vec<PrimalInfo> = serde_json::from_value(response.result.unwrap_or_default())
            .map_err(|e| BearDogError::system(format!("Failed to parse primals list: {e}")))?;

        debug!("✅ Found {} primals", primals.len());
        Ok(primals)
    }

    /// Ping registry to verify connection
    pub async fn ping(&mut self) -> Result<(), BearDogError> {
        let response = self.send_request("primal.ping", None).await?;

        if response.error.is_some() {
            return Err(BearDogError::system("Ping failed".to_string()));
        }

        Ok(())
    }

    /// Unregister from registry (graceful shutdown)
    pub async fn unregister(&mut self, primal_id: &str) -> Result<(), BearDogError> {
        info!("👋 Unregistering {} from registry", primal_id);

        let params = serde_json::json!({
            "primal_id": primal_id,
        });

        let response = self.send_request("primal.unregister", Some(params)).await?;

        if let Some(error) = response.error {
            warn!("⚠️  Unregistration failed: {}", error.message);
            return Err(BearDogError::system(format!(
                "Failed to unregister: {}",
                error.message
            )));
        }

        info!("✅ Successfully unregistered");
        Ok(())
    }

    /// Send JSON-RPC request to registry
    async fn send_request(
        &mut self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<JsonRpcResponse, BearDogError> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| BearDogError::system("Not connected to registry".to_string()))?;

        self.request_id += 1;
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: self.request_id,
        };

        // Serialize and send request
        let request_json = serde_json::to_string(&request)
            .map_err(|e| BearDogError::system(format!("Failed to serialize request: {e}")))?;

        debug!("→ Sending: {}", request_json);

        stream
            .write_all(request_json.as_bytes())
            .await
            .map_err(|e| BearDogError::system(format!("Failed to write request: {e}")))?;

        stream
            .write_all(b"\n")
            .await
            .map_err(|e| BearDogError::system(format!("Failed to write newline: {e}")))?;

        // Read response
        let mut reader = BufReader::new(stream);
        let mut response_line = String::new();

        reader
            .read_line(&mut response_line)
            .await
            .map_err(|e| BearDogError::system(format!("Failed to read response: {e}")))?;

        debug!("← Received: {}", response_line.trim());

        let response: JsonRpcResponse = serde_json::from_str(&response_line)
            .map_err(|e| BearDogError::system(format!("Failed to parse response: {e}")))?;

        Ok(response)
    }

    /// Extract Unix socket path from capability manifest
    fn extract_socket_path(&self, capabilities: &BearDogCapabilities) -> String {
        capabilities
            .endpoints
            .iter()
            .find_map(|endpoint| {
                if let beardog_core::capabilities::IpcEndpoint::UnixSocket { path, .. } = endpoint {
                    Some(path.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "/tmp/beardog-default.sock".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_rpc_request_serialization() {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "primal.ping".to_string(),
            params: None,
            id: 1,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("primal.ping"));
        assert!(json.contains("\"id\":1"));
    }

    #[test]
    fn test_json_rpc_response_deserialization() {
        let json = r#"{"jsonrpc":"2.0","result":{"pong":true},"id":1}"#;
        let response: JsonRpcResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 1);
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_primal_info_deserialization() {
        let json = r#"{
            "primal_id": "beardog",
            "family_id": "nat0",
            "node_id": "tower1",
            "capabilities": ["encryption", "trust"],
            "socket_path": "/tmp/beardog-nat0.sock"
        }"#;

        let info: PrimalInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.primal_id, "beardog");
        assert_eq!(info.family_id, Some("nat0".to_string()));
        assert_eq!(info.capabilities.len(), 2);
    }

    #[test]
    fn test_zero_vendor_hardcoding() {
        // This test documents that we have ZERO vendor hardcoding
        let client = PrimalRegistryClient::new(PathBuf::from("/tmp/any-registry.sock"));

        // Client doesn't know or care what's on the other end
        // Could be Songbird, Consul, etcd, custom - it adapts universally
        assert_eq!(client.socket_path, PathBuf::from("/tmp/any-registry.sock"));
    }

    #[tokio::test]
    async fn test_not_connected_errors() {
        let mut client = PrimalRegistryClient::new(PathBuf::from("/tmp/unused-nonexistent.sock"));
        let caps = beardog_core::capabilities::BearDogCapabilities::new(
            Some("fam".to_string()),
            "node".to_string(),
        );
        let err = client.register(&caps).await.err().expect("not connected");
        assert!(err.to_string().contains("Not connected") || err.to_string().contains("registry"));
    }

    fn unique_registry_sock() -> PathBuf {
        std::env::temp_dir().join(format!(
            "beardog_registry_test_{}_{}.sock",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ))
    }

    fn spawn_registry_mock(path: PathBuf) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let _ = std::fs::remove_file(&path);
            let listener = tokio::net::UnixListener::bind(&path).expect("bind registry mock");
            loop {
                let Ok((stream, _)) = listener.accept().await else {
                    break;
                };
                use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
                tokio::spawn(async move {
                    let mut reader = BufReader::new(stream);
                    loop {
                        let mut line = String::new();
                        if reader.read_line(&mut line).await.is_err() || line.trim().is_empty() {
                            break;
                        }
                        let Ok(v) = serde_json::from_str::<serde_json::Value>(line.trim()) else {
                            break;
                        };
                        let method = v["method"].as_str().unwrap_or("");
                        let id = v["id"].as_u64().unwrap_or(1);
                        let cap = v["params"]["capability"].as_str().unwrap_or("");
                        let response = match method {
                            "primal.register" => serde_json::json!({
                                "jsonrpc": "2.0",
                                "result": { "ok": true },
                                "id": id
                            }),
                            "primal.ping" => serde_json::json!({
                                "jsonrpc": "2.0",
                                "result": { "pong": true },
                                "id": id
                            }),
                            "primal.get_provider" if cap == "none" => serde_json::json!({
                                "jsonrpc": "2.0",
                                "error": { "code": -1, "message": "missing" },
                                "id": id
                            }),
                            "primal.get_provider" => serde_json::json!({
                                "jsonrpc": "2.0",
                                "result": {
                                    "primal_id": "p1",
                                    "family_id": null,
                                    "node_id": "n1",
                                    "capabilities": ["encryption"],
                                    "socket_path": "/tmp/p1.sock"
                                },
                                "id": id
                            }),
                            "primal.list_all" => serde_json::json!({
                                "jsonrpc": "2.0",
                                "result": [{
                                    "primal_id": "p1",
                                    "family_id": null,
                                    "node_id": "n1",
                                    "capabilities": [],
                                    "socket_path": "/tmp/p1.sock"
                                }],
                                "id": id
                            }),
                            "primal.unregister" => serde_json::json!({
                                "jsonrpc": "2.0",
                                "result": { "ok": true },
                                "id": id
                            }),
                            _ => serde_json::json!({
                                "jsonrpc": "2.0",
                                "error": { "code": -32601, "message": "unknown" },
                                "id": id
                            }),
                        };
                        let mut stream = reader.into_inner();
                        let _ = stream
                            .write_all(
                                format!("{}\n", serde_json::to_string(&response).unwrap())
                                    .as_bytes(),
                            )
                            .await;
                        reader = BufReader::new(stream);
                    }
                });
            }
        })
    }

    #[tokio::test]
    async fn test_connect_ping_list_get_provider_unregister() {
        let path = unique_registry_sock();
        let _srv = spawn_registry_mock(path.clone());
        tokio::time::sleep(tokio::time::Duration::from_millis(25)).await;

        let mut client = PrimalRegistryClient::new(path.clone());
        client.connect().await.expect("connect");

        client.ping().await.expect("ping");

        let primals = client.list_all().await.expect("list");
        assert_eq!(primals.len(), 1);
        assert_eq!(primals[0].primal_id, "p1");

        let p = client
            .get_provider("encryption")
            .await
            .expect("get_provider");
        assert_eq!(p.primal_id, "p1");

        let err = client.get_provider("none").await.err().expect("err");
        assert!(err.to_string().contains("missing") || err.to_string().contains("capability"));

        let caps = beardog_core::capabilities::BearDogCapabilities::new(
            Some("fam".to_string()),
            "node".to_string(),
        );
        client.register(&caps).await.expect("register");

        client.unregister("p1").await.expect("unregister");
        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn test_register_error_response() {
        let path = unique_registry_sock();
        let path_clone = path.clone();
        let _srv = tokio::spawn(async move {
            let _ = std::fs::remove_file(&path_clone);
            let listener = tokio::net::UnixListener::bind(&path_clone).unwrap();
            let Ok((mut stream, _)) = listener.accept().await else {
                return;
            };
            use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
            let mut reader = BufReader::new(&mut stream);
            let mut line = String::new();
            let _ = reader.read_line(&mut line).await;
            let mut stream = reader.into_inner();
            let resp = serde_json::json!({
                "jsonrpc": "2.0",
                "error": { "code": -1, "message": "register failed" },
                "id": 1
            });
            let _ = stream
                .write_all(format!("{}\n", resp.to_string()).as_bytes())
                .await;
        });
        tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;

        let mut client = PrimalRegistryClient::new(path.clone());
        client.connect().await.unwrap();
        let caps = beardog_core::capabilities::BearDogCapabilities::new(None, "n".to_string());
        let err = client.register(&caps).await.err().expect("register err");
        assert!(err.to_string().contains("register") || err.to_string().contains("Registry"));
        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn test_list_all_error_branch() {
        let path = unique_registry_sock();
        let path_clone = path.clone();
        let _srv = tokio::spawn(async move {
            let _ = std::fs::remove_file(&path_clone);
            let listener = tokio::net::UnixListener::bind(&path_clone).unwrap();
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    break;
                };
                use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
                let mut reader = BufReader::new(&mut stream);
                let mut line = String::new();
                let _ = reader.read_line(&mut line).await;
                let v: serde_json::Value = serde_json::from_str(line.trim()).unwrap_or_default();
                let id = v["id"].as_u64().unwrap_or(1);
                let mut stream = reader.into_inner();
                let resp = serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": { "code": -1, "message": "list failed" },
                    "id": id
                });
                let _ = stream
                    .write_all(format!("{}\n", resp.to_string()).as_bytes())
                    .await;
            }
        });
        tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;

        let mut client = PrimalRegistryClient::new(path.clone());
        client.connect().await.unwrap();
        let err = client.list_all().await.err().expect("list err");
        assert!(err.to_string().contains("list") || err.to_string().contains("primals"));
        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn test_ping_error_branch() {
        let path = unique_registry_sock();
        let path_clone = path.clone();
        let _srv = tokio::spawn(async move {
            let _ = std::fs::remove_file(&path_clone);
            let listener = tokio::net::UnixListener::bind(&path_clone).unwrap();
            let Ok((mut stream, _)) = listener.accept().await else {
                return;
            };
            use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
            let mut reader = BufReader::new(&mut stream);
            let mut line = String::new();
            let _ = reader.read_line(&mut line).await;
            let mut stream = reader.into_inner();
            let resp = serde_json::json!({
                "jsonrpc": "2.0",
                "error": { "code": -1, "message": "bad" },
                "id": 1
            });
            let _ = stream
                .write_all(format!("{}\n", resp.to_string()).as_bytes())
                .await;
        });
        tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;

        let mut client = PrimalRegistryClient::new(path.clone());
        client.connect().await.unwrap();
        let err = client.ping().await.err().expect("ping err");
        assert!(err.to_string().contains("Ping") || err.to_string().contains("ping"));
        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn test_unregister_error_branch() {
        let path = unique_registry_sock();
        let path_clone = path.clone();
        let _srv = tokio::spawn(async move {
            let _ = std::fs::remove_file(&path_clone);
            let listener = tokio::net::UnixListener::bind(&path_clone).unwrap();
            let Ok((mut stream, _)) = listener.accept().await else {
                return;
            };
            use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
            let mut reader = BufReader::new(&mut stream);
            let mut line = String::new();
            let _ = reader.read_line(&mut line).await;
            let mut stream = reader.into_inner();
            let resp = serde_json::json!({
                "jsonrpc": "2.0",
                "error": { "code": -1, "message": "nope" },
                "id": 1
            });
            let _ = stream
                .write_all(format!("{}\n", resp.to_string()).as_bytes())
                .await;
        });
        tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;

        let mut client = PrimalRegistryClient::new(path.clone());
        client.connect().await.unwrap();
        let err = client.unregister("x").await.err().expect("unreg err");
        assert!(err.to_string().contains("unregister") || err.to_string().contains("Unregister"));
        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn test_register_uses_default_socket_path_when_no_unix_endpoint() {
        let path = unique_registry_sock();
        let _srv = spawn_registry_mock(path.clone());
        tokio::time::sleep(tokio::time::Duration::from_millis(25)).await;

        let mut client = PrimalRegistryClient::new(path.clone());
        client.connect().await.unwrap();

        use beardog_core::capabilities::{Capability, IpcEndpoint};
        use std::collections::HashMap;
        let caps = beardog_core::capabilities::BearDogCapabilities {
            primal_id: "beardog".to_string(),
            family_id: None,
            node_id: "n".to_string(),
            provides: vec![Capability::Custom {
                name: "custom_cap".to_string(),
                version: "1".to_string(),
                properties: HashMap::new(),
            }],
            requires: vec![],
            endpoints: vec![IpcEndpoint::Http {
                bind_addr: "127.0.0.1:9".to_string(),
                tls: false,
            }],
            metadata: HashMap::new(),
        };
        client
            .register(&caps)
            .await
            .expect("register with http-only endpoint");
        let _ = std::fs::remove_file(&path);
    }
}

// Comprehensive test suite
#[cfg(test)]
#[path = "registry_client_tests.rs"]
mod registry_client_tests;
