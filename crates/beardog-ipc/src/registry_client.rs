// SPDX-License-Identifier: AGPL-3.0-or-later

//! Universal Primal Registry Client
//!
//! Generic client for registering with ANY primal registry that speaks JSON-RPC 2.0
//!
//! # Registry endpoint discovery (tier hierarchy)
//!
//! The transport address is **never** hardcoded in production logic:
//!
//! 1. **Constructor** — [`PrimalRegistryClient::new`] receives the registry Unix socket path from
//!    the caller (orchestrator, manifest, or discovery layer).
//! 2. **Capability manifest** — [`PrimalRegistryClient::register`] advertises the primal’s IPC
//!    path from the first [`beardog_core::capabilities::IpcEndpoint::UnixSocket`] in
//!    [`beardog_core::capabilities::BearDogCapabilities::endpoints`], when present.
//! 3. **Environment override** — `BEARDOG_REGISTRY_SOCKET_FALLBACK` supplies a Unix path when the
//!    manifest has no Unix endpoint (misconfiguration / HTTP-only endpoints).
//! 4. **Last-resort file path** — if tier 3 is unset, `{std::env::temp_dir()}/beardog-registry-default.sock`
//!    (see `fallback_registry_unix_socket_path`).
//!
//! There are no default HTTP(S) URLs or TCP ports in this module; use Unix sockets and explicit
//! configuration at the call site.
//!
//! # Design Principle: Zero Vendor Hardcoding
//!
//! This client does NOT know:
//! - Which registry it's talking to (any JSON-RPC 2.0 registry implementation)
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

use crate::isomorphic::{IpcStream, connect_raw};
use crate::protocol::JSONRPC_VERSION;
use beardog_config::env_keys;
use beardog_core::capabilities::{BearDogCapabilities, Capability};
use beardog_errors::BearDogError;
use beardog_types::btsp::TransportEndpoint;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, error, info, warn};

/// Tier-3/4 fallback: registry UDS when the capability manifest has no Unix endpoint.
///
/// **Tier 3:** `BEARDOG_REGISTRY_SOCKET_FALLBACK` when set. **Tier 4:** otherwise
/// `{std::env::temp_dir()}/beardog-registry-default.sock` (platform temp dir, not a hardcoded `/tmp`).
#[must_use]
fn fallback_registry_unix_socket_path() -> String {
    std::env::var(env_keys::ENV_REGISTRY_SOCKET_FALLBACK).unwrap_or_else(|_| {
        std::env::temp_dir()
            .join("beardog-registry-default.sock")
            .to_string_lossy()
            .into_owned()
    })
}

/// JSON-RPC 2.0 Request (Universal)
///
/// Per spec, `id` may be String, Number, or Null. Requests without `id` are
/// notifications and MUST NOT receive a response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// Protocol version; must be `"2.0"` for compliant servers.
    pub jsonrpc: Cow<'static, str>,
    /// Method name (e.g. `primal.register`, `primal.get_provider`).
    pub method: Cow<'static, str>,
    /// Positional or object parameters; omitted when empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    /// Correlation id matching the corresponding [`JsonRpcResponse::id`].
    pub id: serde_json::Value,
}

/// JSON-RPC 2.0 Response (Universal)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// Protocol version; must be `"2.0"`.
    pub jsonrpc: Cow<'static, str>,
    /// Successful result payload; mutually exclusive with [`JsonRpcResponse::error`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Error object when the call failed; mutually exclusive with `result`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    /// Id echoing the originating [`JsonRpcRequest::id`].
    pub id: serde_json::Value,
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
/// Connects to ANY registry that speaks JSON-RPC 2.0 over transport-agnostic IPC
///
/// # Zero Vendor Hardcoding
///
/// This client works with:
/// - Any JSON-RPC primal registry (orchestrator-agnostic)
/// - Consul (`HashiCorp` service mesh)
/// - etcd (Kubernetes/Cloud Native registry)
/// - Custom registries
/// - ANY system that speaks JSON-RPC 2.0
pub struct PrimalRegistryClient {
    endpoint: TransportEndpoint,
    stream: Option<IpcStream>,
    request_id: u64,
}

impl PrimalRegistryClient {
    /// Create a new registry client from a socket path.
    ///
    /// The path is wrapped in a [`TransportEndpoint::Uds`] for transport dispatch.
    ///
    /// # Zero Assumptions
    ///
    /// We don't know or care what's on the other end of this endpoint.
    /// Could be any registry implementation that speaks the wire protocol.
    #[must_use]
    pub const fn new(socket_path: PathBuf) -> Self {
        Self {
            endpoint: TransportEndpoint::Uds { path: socket_path },
            stream: None,
            request_id: 0,
        }
    }

    /// Create a registry client from an explicit [`TransportEndpoint`].
    #[must_use]
    pub const fn from_endpoint(endpoint: TransportEndpoint) -> Self {
        Self {
            endpoint,
            stream: None,
            request_id: 0,
        }
    }

    /// Connect to registry via transport-agnostic IPC.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when the transport connection fails.
    pub async fn connect(&mut self) -> Result<(), BearDogError> {
        info!(endpoint = %self.endpoint, "Connecting to primal registry");

        let stream = connect_raw(&self.endpoint).await.map_err(|e| {
            BearDogError::system(format!(
                "Failed to connect to registry at {}: {e}",
                self.endpoint,
            ))
        })?;

        self.stream = Some(stream);
        info!("Connected to primal registry");

        Ok(())
    }

    /// The underlying socket path (if the endpoint is UDS).
    ///
    /// Returns the path for UDS endpoints; panics on non-UDS (test-only usage).
    #[cfg(test)]
    #[must_use]
    pub fn socket_path(&self) -> &std::path::Path {
        match &self.endpoint {
            TransportEndpoint::Uds { path } => path,
            _ => panic!("socket_path() called on non-UDS endpoint"),
        }
    }

    /// Register with registry
    ///
    /// Advertises this primal's capabilities to the ecosystem
    ///
    /// # Universal Adapter
    ///
    /// This works with any registry that understands JSON-RPC method "primal.register"
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when the RPC fails or the registry reports an error.
    pub async fn register(
        &mut self,
        capabilities: &BearDogCapabilities,
    ) -> Result<(), BearDogError> {
        info!("📝 Registering with primal registry");
        info!("   Primal ID: {}", capabilities.primal_id);
        info!("   Family ID: {:?}", capabilities.family_id);
        info!("   Node ID: {}", capabilities.node_id);
        info!("   Capabilities: {}", capabilities.provides.len());

        // Extract capability names for registration (borrow static / manifest strings — no per-cap clone)
        let capability_names: Vec<&str> = capabilities
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
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when no provider exists or the response cannot be parsed.
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
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when the RPC fails or the registry list cannot be parsed.
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
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when the ping RPC fails.
    pub async fn ping(&mut self) -> Result<(), BearDogError> {
        let response = self.send_request("primal.ping", None).await?;

        if response.error.is_some() {
            return Err(BearDogError::system("Ping failed".to_string()));
        }

        Ok(())
    }

    /// Unregister from registry (graceful shutdown)
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when the registry reports an unregistration error.
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
        method: impl Into<Cow<'static, str>>,
        params: Option<serde_json::Value>,
    ) -> Result<JsonRpcResponse, BearDogError> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| BearDogError::system("Not connected to registry".to_string()))?;

        self.request_id += 1;
        let request = JsonRpcRequest {
            jsonrpc: Cow::Borrowed(JSONRPC_VERSION),
            method: method.into(),
            params,
            id: serde_json::Value::from(self.request_id),
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

        let mut reader = BufReader::new(stream);
        let mut response_line = String::new();

        tokio::time::timeout(
            std::time::Duration::from_secs(30),
            reader.read_line(&mut response_line),
        )
        .await
        .map_err(|_| BearDogError::system("Registry response read timed out (30s)".to_string()))?
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
            .unwrap_or_else(fallback_registry_unix_socket_path)
    }
}

#[cfg(test)]
#[path = "registry_client_inline_tests.rs"]
mod tests;

// Comprehensive test suite
#[cfg(test)]
#[path = "registry_client_tests.rs"]
mod registry_client_tests;
