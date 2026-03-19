// SPDX-License-Identifier: AGPL-3.0-only

//! Songbird IPC Client
//!
//! Client for communicating with Songbird discovery orchestrator via Unix socket
//!
//! # Architecture
//!
//! BearDog uses this client to:
//! - Register capabilities with Songbird
//! - Query for other primals
//! - Receive primal updates
//!
//! # Protocol
//!
//! JSON-RPC 2.0 over Unix domain sockets

use beardog_core::capabilities::BearDogCapabilities;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tracing::{debug, error, info, warn};

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    pub id: u64,
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: u64,
}

/// JSON-RPC 2.0 Error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Primal information returned by Songbird
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalInfo {
    pub primal_id: String,
    pub family_id: Option<String>,
    pub node_id: String,
    pub capabilities: Vec<String>,
    pub socket_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
}

/// Songbird IPC Client
///
/// Connects to Songbird's Unix socket and provides capability-based discovery
pub struct SongbirdClient {
    socket_path: PathBuf,
    stream: Option<UnixStream>,
    request_id: u64,
}

impl SongbirdClient {
    /// Create a new Songbird client
    ///
    /// # Arguments
    ///
    /// * `socket_path` - Path to Songbird's Unix socket (e.g., `/tmp/songbird-nat0.sock`)
    pub fn new(socket_path: PathBuf) -> Self {
        Self {
            socket_path,
            stream: None,
            request_id: 0,
        }
    }

    /// Connect to Songbird
    ///
    /// Establishes Unix socket connection to Songbird orchestrator
    pub async fn connect(&mut self) -> Result<(), BearDogError> {
        info!("🔌 Connecting to Songbird at {:?}", self.socket_path);

        let stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| {
                BearDogError::system(format!(
                    "Failed to connect to Songbird at {:?}: {}",
                    self.socket_path, e
                ))
            })?;

        self.stream = Some(stream);
        info!("✅ Connected to Songbird");

        Ok(())
    }

    /// Register BearDog with Songbird
    ///
    /// Advertises BearDog's capabilities to the ecosystem
    ///
    /// # Arguments
    ///
    /// * `capabilities` - BearDog's capability manifest
    pub async fn register(
        &mut self,
        capabilities: &BearDogCapabilities,
    ) -> Result<(), BearDogError> {
        info!("📝 Registering BearDog with Songbird");
        info!("   Primal ID: {}", capabilities.primal_id);
        info!("   Family ID: {:?}", capabilities.family_id);
        info!("   Node ID: {}", capabilities.node_id);
        info!("   Capabilities: {}", capabilities.provides.len());

        // Extract capability names for registration
        let capability_names: Vec<String> = capabilities
            .provides
            .iter()
            .map(|cap| match cap {
                beardog_core::capabilities::Capability::Encryption { .. } => "encryption",
                beardog_core::capabilities::Capability::TrustEvaluation { .. } => "trust_evaluation",
                beardog_core::capabilities::Capability::KeyManagement { .. } => "key_management",
                beardog_core::capabilities::Capability::Signatures { .. } => "signatures",
                beardog_core::capabilities::Capability::Discovery { .. } => "discovery",
                beardog_core::capabilities::Capability::Storage { .. } => "storage",
                beardog_core::capabilities::Capability::Compute { .. } => "compute",
                beardog_core::capabilities::Capability::Custom { name, .. } => name.as_str(),
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
                "Songbird registration failed: {}",
                error.message
            )));
        }

        info!("✅ Successfully registered with Songbird");
        Ok(())
    }

    /// Get provider for a capability
    ///
    /// Query Songbird to find which primal provides a given capability
    pub async fn get_provider(&mut self, capability: &str) -> Result<PrimalInfo, BearDogError> {
        debug!("🔍 Querying Songbird for capability: {}", capability);

        let params = serde_json::json!({
            "capability": capability,
        });

        let response = self.send_request("primal.get_provider", Some(params)).await?;

        if let Some(error) = response.error {
            warn!("⚠️  No provider found for {}: {}", capability, error.message);
            return Err(BearDogError::system(format!(
                "No provider for capability {}: {}",
                capability, error.message
            )));
        }

        let primal_info: PrimalInfo = serde_json::from_value(response.result.unwrap_or_default())
            .map_err(|e| BearDogError::system(format!("Failed to parse primal info: {}", e)))?;

        debug!("✅ Found provider: {}", primal_info.primal_id);
        Ok(primal_info)
    }

    /// List all registered primals
    pub async fn list_all(&mut self) -> Result<Vec<PrimalInfo>, BearDogError> {
        debug!("📋 Listing all primals from Songbird");

        let response = self.send_request("primal.list_all", None).await?;

        if let Some(error) = response.error {
            return Err(BearDogError::system(format!(
                "Failed to list primals: {}",
                error.message
            )));
        }

        let primals: Vec<PrimalInfo> = serde_json::from_value(response.result.unwrap_or_default())
            .map_err(|e| BearDogError::system(format!("Failed to parse primals list: {}", e)))?;

        debug!("✅ Found {} primals", primals.len());
        Ok(primals)
    }

    /// Ping Songbird to verify connection
    pub async fn ping(&mut self) -> Result<(), BearDogError> {
        let response = self.send_request("primal.ping", None).await?;

        if response.error.is_some() {
            return Err(BearDogError::system("Ping failed".to_string()));
        }

        Ok(())
    }

    /// Unregister from Songbird (graceful shutdown)
    pub async fn unregister(&mut self, primal_id: &str) -> Result<(), BearDogError> {
        info!("👋 Unregistering {} from Songbird", primal_id);

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

    /// Send JSON-RPC request to Songbird
    async fn send_request(
        &mut self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<JsonRpcResponse, BearDogError> {
        let stream = self.stream.as_mut().ok_or_else(|| {
            BearDogError::system("Not connected to Songbird".to_string())
        })?;

        self.request_id += 1;
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: self.request_id,
        };

        // Serialize and send request
        let request_json = serde_json::to_string(&request)
            .map_err(|e| BearDogError::system(format!("Failed to serialize request: {}", e)))?;

        debug!("→ Sending: {}", request_json);

        stream
            .write_all(request_json.as_bytes())
            .await
            .map_err(|e| BearDogError::system(format!("Failed to write request: {}", e)))?;

        stream
            .write_all(b"\n")
            .await
            .map_err(|e| BearDogError::system(format!("Failed to write newline: {}", e)))?;

        // Read response
        let mut reader = BufReader::new(stream);
        let mut response_line = String::new();

        reader
            .read_line(&mut response_line)
            .await
            .map_err(|e| BearDogError::system(format!("Failed to read response: {}", e)))?;

        debug!("← Received: {}", response_line.trim());

        let response: JsonRpcResponse = serde_json::from_str(&response_line)
            .map_err(|e| BearDogError::system(format!("Failed to parse response: {}", e)))?;

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
}

