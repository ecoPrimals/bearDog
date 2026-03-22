// SPDX-License-Identifier: AGPL-3.0-only

//! # UPA Registration Client (Tower Atomic Edition)
//!
//! **EVOLVED**: Now uses Tower Atomic (Unix sockets + JSON-RPC) instead of HTTP!
//!
//! Modern async client for Universal Port Authority (UPA) over Tower Atomic IPC.

use std::path::Path;
use std::sync::Arc;

use arc_swap::ArcSwap;
use beardog_discovery::{
    discovered_services_from_environment_from_env, primary_url_to_ipc_socket_path,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::sync::Mutex;
use tracing::{debug, info};

use beardog_errors::BearDogError;

use beardog_tower_atomic::Client as AtomicClient;

/// Environment capability key: `CAPABILITY_UPA_REGISTER_ENDPOINT` (see `beardog_discovery::capability_env`).
const UPA_REGISTER_CAPABILITY: &str = "upa_register";

/// Load metrics reported with heartbeats (CPU/memory from heartbeat; connections from API tracker).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadMetrics {
    /// CPU usage percentage (0–100).
    pub cpu_percent: f32,
    /// Memory usage percentage (0–100).
    pub memory_percent: f32,
    /// Concurrent in-flight HTTP requests on the integration API server.
    pub active_connections: u32,
}

/// Configuration for [`UpaClient`] (retained for API compatibility; discovery uses env/capabilities).
#[derive(Debug, Clone, Default)]
pub struct UpaClientConfig {
    /// Legacy URL field from HTTP client days; Tower Atomic uses Unix sockets from env.
    pub upa_url: String,
}

/// Minimal service record returned by discovery-style UPA responses (fields may grow without breaking callers).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service identifier.
    pub id: String,
}

/// Service status snapshot for UPA heartbeats and admin views.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    /// Human-readable status.
    pub status: String,
}

/// UPA registration request payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRequest {
    /// Service name (e.g., "beardog-security-provider")
    pub service_name: String,
    /// Service version
    pub version: String,
    /// List of capabilities this service provides
    pub capabilities: Vec<String>,
    /// Service endpoint (Unix socket path)
    pub endpoint: String,
    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// UPA registration response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationResponse {
    /// Unique service ID assigned by UPA
    pub service_id: String,
    /// Authentication token for subsequent requests
    pub token: String,
    /// Registration timestamp
    pub registered_at: String,
}

/// Connect to the UPA JSON-RPC peer using capability-first resolution.
///
/// Priority:
/// 1. `UPA_UNIX_SOCKET` — explicit filesystem path to the Unix socket
/// 2. `CAPABILITY_UPA_REGISTER_ENDPOINT` — capability env discovery (`upa_register`)
/// 3. `UPA_PROVIDER` — legacy socket filename key under the standard ecoPrimals search paths
async fn connect_upa_atomic() -> Result<AtomicClient, BearDogError> {
    if let Ok(path) = beardog_errors::process_env::var("UPA_UNIX_SOCKET") {
        return AtomicClient::connect_unix_path(Path::new(&path), "upa_unix_socket")
            .await
            .map_err(|e| {
                BearDogError::network(format!("UPA UPA_UNIX_SOCKET connect failed: {e}"))
            });
    }

    let services = discovered_services_from_environment_from_env(UPA_REGISTER_CAPABILITY, 600);
    if let Some(s) = services.first() {
        let path = primary_url_to_ipc_socket_path(&s.endpoint.primary_url);
        return AtomicClient::connect_unix_path(Path::new(&path), "upa_register_capability")
            .await
            .map_err(|e| {
                BearDogError::network(format!(
                    "UPA capability endpoint (upa_register / CAPABILITY_UPA_REGISTER_ENDPOINT): {e}"
                ))
            });
    }

    if let Ok(name) = beardog_errors::process_env::var("UPA_PROVIDER") {
        return AtomicClient::connect(&name)
            .await
            .map_err(|e| BearDogError::network(format!("UPA UPA_PROVIDER connect failed: {e}")));
    }

    Err(BearDogError::network(
        "UPA: set UPA_UNIX_SOCKET, CAPABILITY_UPA_REGISTER_ENDPOINT, or UPA_PROVIDER".to_string(),
    ))
}

/// UPA client for service registration and discovery
pub struct UpaClient {
    /// Tower Atomic client for UPA provider
    client: Arc<Mutex<AtomicClient>>,
    /// Cached registration token
    token: Arc<ArcSwap<Option<String>>>,
}

impl UpaClient {
    /// Create a new UPA client and connect via Tower Atomic.
    ///
    /// `config.upa_url` is accepted for compatibility; connection uses `UPA_UNIX_SOCKET` /
    /// capability discovery / `UPA_PROVIDER` as documented in `connect_upa_atomic`.
    ///
    /// # Errors
    ///
    /// Returns an error when no UPA endpoint is configured or connection fails.
    pub async fn new(config: UpaClientConfig) -> Result<Self, BearDogError> {
        let _ = config.upa_url;
        info!("🔌 Connecting to UPA via Tower Atomic (capability-first discovery)");

        let client = connect_upa_atomic().await?;

        info!("✅ Connected to UPA provider");

        Ok(Self {
            client: Arc::new(Mutex::new(client)),
            token: Arc::new(ArcSwap::new(Arc::new(None))),
        })
    }

    /// Register a service with UPA
    ///
    /// # Errors
    ///
    /// Returns an error when the JSON-RPC call fails or the response cannot be parsed.
    pub async fn register(
        &self,
        request: RegistrationRequest,
    ) -> Result<RegistrationResponse, BearDogError> {
        info!("📝 Registering service: {}", request.service_name);

        let params = json!({
            "service_name": request.service_name,
            "version": request.version,
            "capabilities": request.capabilities,
            "endpoint": request.endpoint,
            "metadata": request.metadata,
        });

        let response = self
            .client
            .lock()
            .await
            .call("upa.register", params)
            .await
            .map_err(|e| BearDogError::api(format!("UPA registration failed: {e}")))?;

        let registration: RegistrationResponse = serde_json::from_value(response).map_err(|e| {
            let msg = format!("Failed to parse registration response: {e}");
            BearDogError::serialization(&msg)
        })?;

        self.token.store(Arc::new(Some(registration.token.clone())));

        info!("✅ Registered with service ID: {}", registration.service_id);

        Ok(registration)
    }

    /// Discover services by capability
    ///
    /// # Errors
    ///
    /// Returns an error when the JSON-RPC call fails.
    pub async fn discover(&self, capability: &str) -> Result<Vec<Value>, BearDogError> {
        info!("🔍 Discovering services with capability: {capability}");

        let params = json!({
            "capability": capability,
        });

        let response = self
            .client
            .lock()
            .await
            .call("upa.discover", params)
            .await
            .map_err(|e| BearDogError::api(format!("UPA discovery failed: {e}")))?;

        let services: Vec<Value> =
            serde_json::from_value(response["services"].clone()).unwrap_or_default();

        info!("✅ Found {} services", services.len());

        Ok(services)
    }

    /// Heartbeat to keep registration alive (token-only; legacy UPA).
    ///
    /// # Errors
    ///
    /// Returns an error when not registered or the RPC fails.
    pub async fn heartbeat(&self) -> Result<(), BearDogError> {
        debug!("💓 Sending heartbeat to UPA");

        let token = self.token.load();
        let token = token
            .as_ref()
            .as_ref()
            .ok_or_else(|| BearDogError::unauthorized("Not registered".to_string()))?;

        let params = json!({
            "token": token,
        });

        self.client
            .lock()
            .await
            .call("upa.heartbeat", params)
            .await
            .map_err(|e| BearDogError::api(format!("UPA heartbeat failed: {e}")))?;

        debug!("✅ Heartbeat sent");

        Ok(())
    }

    /// Send a heartbeat with service identity, token, and load metrics.
    ///
    /// # Errors
    ///
    /// Returns an error when the JSON-RPC call fails.
    pub async fn send_heartbeat(
        &self,
        service_id: &str,
        token: &str,
        metrics: LoadMetrics,
    ) -> Result<(), BearDogError> {
        debug!(service_id = %service_id, "💓 Sending heartbeat with metrics");

        let params = json!({
            "service_id": service_id,
            "token": token,
            "metrics": metrics,
        });

        self.client
            .lock()
            .await
            .call("upa.heartbeat", params)
            .await
            .map_err(|e| BearDogError::api(format!("UPA heartbeat with metrics failed: {e}")))?;

        debug!("✅ Heartbeat with metrics sent");

        Ok(())
    }

    /// Deregister from UPA (best-effort).
    ///
    /// # Errors
    ///
    /// Returns an error when the JSON-RPC call fails.
    pub async fn deregister(&self, service_id: &str, token: &str) -> Result<(), BearDogError> {
        let params = json!({
            "service_id": service_id,
            "token": token,
        });

        self.client
            .lock()
            .await
            .call("upa.deregister", params)
            .await
            .map_err(|e| BearDogError::api(format!("UPA deregister failed: {e}")))?;

        self.token.store(Arc::new(None));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires a UPA endpoint (env / capability)
    async fn test_connect() {
        let client = UpaClient::new(UpaClientConfig::default()).await;
        match client {
            Ok(_) => println!("✅ Connected to UPA"),
            Err(e) => println!("⚠️  UPA not available: {e}"),
        }
    }
}
