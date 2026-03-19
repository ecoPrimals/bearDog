// SPDX-License-Identifier: AGPL-3.0-only

//! Songbird IPC Client
//!
//! Implements the client side of the Primal IPC Protocol for communicating with Songbird.

use crate::{
    error::{IpcError, IpcResult},
    protocol::{JsonRpcRequest, JsonRpcResponse},
    types::{Capability, ServiceInfo},
    DISCOVERY_SOCKET_FALLBACK,
};
use serde_json::json;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tracing::{debug, info, warn};

/// Client for communicating with Songbird service registry
#[derive(Debug)]
pub struct SongbirdClient {
    /// Songbird socket path
    socket_path: String,
    /// Request ID counter
    request_id: Arc<AtomicU64>,
    /// Registered primal name
    primal_name: Arc<RwLock<Option<String>>>,
}

impl SongbirdClient {
    /// Create a new Songbird client
    pub fn new() -> Self {
        Self {
            socket_path: DISCOVERY_SOCKET_FALLBACK.to_string(),
            request_id: Arc::new(AtomicU64::new(1)),
            primal_name: Arc::new(RwLock::new(None)),
        }
    }

    /// Connect to Songbird (validates socket exists)
    pub async fn connect() -> IpcResult<Self> {
        let client = Self::new();

        // Test connection
        let _stream = UnixStream::connect(&client.socket_path)
            .await
            .map_err(|e| {
                IpcError::Connection(format!(
                    "Cannot connect to Songbird at {}: {}",
                    client.socket_path, e
                ))
            })?;

        info!("✅ Connected to Songbird at {}", client.socket_path);
        Ok(client)
    }

    /// Register this primal with Songbird
    ///
    /// # Example
    /// ```no_run
    /// # use beardog_ipc::{SongbirdClient, Capability};
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = SongbirdClient::connect().await?;
    /// client.register(
    ///     "beardog",
    ///     vec![Capability::Crypto, Capability::BTSP],
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn register(
        &self,
        primal_name: &str,
        capabilities: Vec<Capability>,
    ) -> IpcResult<()> {
        info!(
            "📝 Registering {} with capabilities: {:?}",
            primal_name, capabilities
        );

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "ipc.register".to_string(),
            params: json!({
                "name": primal_name,
                "endpoint": format!("/primal/{}", primal_name),
                "capabilities": capabilities.iter().map(|c| c.as_str()).collect::<Vec<_>>(),
                "version": env!("CARGO_PKG_VERSION"),
                "metadata": {
                    "description": "BearDog - Cryptographic Security Primal"
                }
            }),
            id: self.next_request_id(),
        };

        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            if result
                .get("registered")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                *self.primal_name.write().await = Some(primal_name.to_string());
                info!("✅ Successfully registered {} with Songbird", primal_name);
                Ok(())
            } else {
                Err(IpcError::Protocol("Registration failed".to_string()))
            }
        } else if let Some(error) = response.error {
            Err(IpcError::Protocol(format!(
                "Registration error: {:?}",
                error
            )))
        } else {
            Err(IpcError::Protocol(
                "Invalid registration response".to_string(),
            ))
        }
    }

    /// Find services by capability
    ///
    /// # Example
    /// ```no_run
    /// # use beardog_ipc::SongbirdClient;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = SongbirdClient::connect().await?;
    /// let crypto_services = client.find_capability("crypto").await?;
    /// for service in crypto_services {
    ///     println!("Found: {} at {}", service.name, service.endpoint);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn find_capability(&self, capability: &str) -> IpcResult<Vec<ServiceInfo>> {
        debug!("🔍 Finding services with capability: {}", capability);

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "ipc.find_capability".to_string(),
            params: json!({
                "capability": capability
            }),
            id: self.next_request_id(),
        };

        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let services = result
                .get("services")
                .and_then(|s| s.as_array())
                .ok_or_else(|| IpcError::Protocol("Invalid services format".to_string()))?;

            let service_infos = services
                .iter()
                .filter_map(|s| serde_json::from_value(s.clone()).ok())
                .collect();

            debug!(
                "✅ Found {} services with capability '{}'",
                services.len(),
                capability
            );
            Ok(service_infos)
        } else if let Some(error) = response.error {
            Err(IpcError::Protocol(format!(
                "Find capability error: {:?}",
                error
            )))
        } else {
            Err(IpcError::Protocol(
                "Invalid find capability response".to_string(),
            ))
        }
    }

    /// Resolve a specific primal by name
    pub async fn resolve(&self, primal_name: &str) -> IpcResult<ServiceInfo> {
        debug!("🔍 Resolving primal: {}", primal_name);

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "ipc.resolve".to_string(),
            params: json!({
                "primal": primal_name
            }),
            id: self.next_request_id(),
        };

        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            serde_json::from_value(result)
                .map_err(|e| IpcError::Protocol(format!("Invalid service info: {}", e)))
        } else if let Some(error) = response.error {
            Err(IpcError::Protocol(format!("Resolve error: {:?}", error)))
        } else {
            Err(IpcError::Protocol("Invalid resolve response".to_string()))
        }
    }

    /// Send heartbeat to maintain registration
    pub async fn heartbeat(&self) -> IpcResult<()> {
        let primal_name = self.primal_name.read().await;
        let name = primal_name
            .as_ref()
            .ok_or_else(|| IpcError::Protocol("Not registered yet".to_string()))?;

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "ipc.heartbeat".to_string(),
            params: json!({
                "name": name
            }),
            id: self.next_request_id(),
        };

        let response = self.send_request(request).await?;

        if response.result.is_some() {
            debug!("💓 Heartbeat sent");
            Ok(())
        } else if let Some(error) = response.error {
            warn!("⚠️ Heartbeat error: {:?}", error);
            Err(IpcError::Protocol(format!("Heartbeat error: {:?}", error)))
        } else {
            Err(IpcError::Protocol("Invalid heartbeat response".to_string()))
        }
    }

    /// Start automatic heartbeat task
    ///
    /// Returns a handle that stops heartbeat when dropped
    pub fn start_heartbeat(&self, interval_duration: Duration) -> HeartbeatHandle {
        let client = Self {
            socket_path: self.socket_path.clone(),
            request_id: Arc::clone(&self.request_id),
            primal_name: Arc::clone(&self.primal_name),
        };

        let (tx, mut rx) = tokio::sync::oneshot::channel();

        let handle = tokio::spawn(async move {
            let mut ticker = interval(interval_duration);
            loop {
                tokio::select! {
                    _ = ticker.tick() => {
                        if let Err(e) = client.heartbeat().await {
                            warn!("Heartbeat failed: {}", e);
                        }
                    }
                    _ = &mut rx => {
                        info!("Stopping heartbeat");
                        break;
                    }
                }
            }
        });

        HeartbeatHandle {
            _stop_tx: tx,
            _task: handle,
        }
    }

    // Internal methods

    async fn send_request(&self, request: JsonRpcRequest) -> IpcResult<JsonRpcResponse> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| IpcError::Connection(format!("Failed to connect: {}", e)))?;

        // Serialize and send request
        let request_json =
            serde_json::to_vec(&request).map_err(|e| IpcError::Serialization(e.to_string()))?;

        stream
            .write_all(&request_json)
            .await
            .map_err(IpcError::Io)?;
        stream.write_all(b"\n").await.map_err(IpcError::Io)?;

        // Read response
        let mut buffer = vec![0u8; 8192];
        let n = stream.read(&mut buffer).await.map_err(IpcError::Io)?;

        // Deserialize response
        let response: JsonRpcResponse = serde_json::from_slice(&buffer[..n])
            .map_err(|e| IpcError::Serialization(e.to_string()))?;

        Ok(response)
    }

    fn next_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }
}

impl Default for SongbirdClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle for automatic heartbeat task
pub struct HeartbeatHandle {
    _stop_tx: tokio::sync::oneshot::Sender<()>,
    _task: tokio::task::JoinHandle<()>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = SongbirdClient::new();
        assert_eq!(client.socket_path, DISCOVERY_SOCKET_FALLBACK);
    }

    #[test]
    fn test_client_default() {
        let client = SongbirdClient::default();
        assert_eq!(client.socket_path, DISCOVERY_SOCKET_FALLBACK);
    }

    #[test]
    fn test_request_id_increment() {
        let client = SongbirdClient::new();
        assert_eq!(client.next_request_id(), 1);
        assert_eq!(client.next_request_id(), 2);
        assert_eq!(client.next_request_id(), 3);
    }

    #[tokio::test]
    async fn test_heartbeat_handle_creation_and_drop() {
        let client = SongbirdClient::new();
        let handle = client.start_heartbeat(Duration::from_secs(3600));
        drop(handle);
    }

    #[tokio::test]
    async fn test_connect_requires_socket() {
        let result = SongbirdClient::connect().await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, IpcError::Connection(_) | IpcError::Io(_)),
            "Expected Connection or Io error, got {:?}",
            err
        );
    }

    #[test]
    fn test_request_id_unique_per_client() {
        let c1 = SongbirdClient::new();
        let c2 = SongbirdClient::new();
        let id1 = c1.next_request_id();
        let id2 = c2.next_request_id();
        assert_eq!(id1, 1);
        assert_eq!(id2, 1);
    }
}
