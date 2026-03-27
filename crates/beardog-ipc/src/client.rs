// SPDX-License-Identifier: AGPL-3.0-only

//! IPC registry client (Primal IPC Protocol)
//!
//! JSON-RPC 2.0 over Unix socket to an orchestrator-agnostic service registry (`ipc.*` methods).

use crate::{
    DISCOVERY_SOCKET_FALLBACK, IpcSocketDiscoveryOptions,
    error::{IpcError, IpcResult},
    protocol::{JsonRpcRequest, JsonRpcResponse},
    resolve_ipc_socket_from_options,
    types::{Capability, ServiceInfo},
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tokio::sync::RwLock;
use tokio::time::{Duration, interval};
use tracing::{debug, info, warn};

/// Client for the Primal IPC registry transport (capability discovery, `ipc.register`, etc.).
#[derive(Debug)]
pub struct SongbirdClient {
    /// Registry Unix socket path (from env / capability discovery)
    socket_path: String,
    /// Request ID counter
    request_id: Arc<AtomicU64>,
    /// Registered primal name
    primal_name: Arc<RwLock<Option<String>>>,
}

impl SongbirdClient {
    /// Create a new client (socket path defaults to [`DISCOVERY_SOCKET_FALLBACK`] until [`connect`](Self::connect)).
    pub fn new() -> Self {
        Self {
            socket_path: DISCOVERY_SOCKET_FALLBACK.to_string(),
            request_id: Arc::new(AtomicU64::new(1)),
            primal_name: Arc::new(RwLock::new(None)),
        }
    }

    /// Connect to the IPC registry (validates socket exists)
    pub async fn connect() -> IpcResult<Self> {
        let socket_path = resolve_ipc_socket_from_options(&IpcSocketDiscoveryOptions::from_env());
        let client = Self {
            socket_path,
            request_id: Arc::new(AtomicU64::new(1)),
            primal_name: Arc::new(RwLock::new(None)),
        };

        // Test connection
        let _stream = UnixStream::connect(&client.socket_path)
            .await
            .map_err(|e| {
                IpcError::Connection(format!(
                    "Cannot connect to IPC registry at {}: {}",
                    client.socket_path, e
                ))
            })?;

        info!(socket_path = %client.socket_path, "Connected to IPC registry");
        Ok(client)
    }

    /// Register this primal with the IPC registry
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
            primal_name,
            ?capabilities,
            "Registering primal with IPC registry"
        );

        let request = JsonRpcRequest::new(
            "ipc.register",
            json!({
                "name": primal_name,
                "endpoint": format!("/primal/{}", primal_name),
                "capabilities": capabilities.iter().map(super::types::Capability::as_str).collect::<Vec<_>>(),
                "version": env!("CARGO_PKG_VERSION"),
                "metadata": {
                    "description": "BearDog - Cryptographic Security Primal"
                }
            }),
            self.next_request_id(),
        );

        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            if result
                .get("registered")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
            {
                *self.primal_name.write().await = Some(primal_name.to_string());
                info!(
                    primal_name,
                    "Successfully registered primal with IPC registry"
                );
                Ok(())
            } else {
                Err(IpcError::Protocol("Registration failed".to_string()))
            }
        } else if let Some(error) = response.error {
            Err(IpcError::Protocol(format!("Registration error: {error:?}")))
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
        debug!(capability, "Finding services by capability");

        let request = JsonRpcRequest::new(
            "ipc.find_capability",
            json!({
                "capability": capability
            }),
            self.next_request_id(),
        );

        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            let services = result
                .get("services")
                .and_then(|s| s.as_array())
                .ok_or_else(|| IpcError::Protocol("Invalid services format".to_string()))?;

            let service_infos = services
                .iter()
                .filter_map(|s| ServiceInfo::deserialize(s).ok())
                .collect();

            debug!(
                count = services.len(),
                capability, "Found services with capability"
            );
            Ok(service_infos)
        } else if let Some(error) = response.error {
            Err(IpcError::Protocol(format!(
                "Find capability error: {error:?}"
            )))
        } else {
            Err(IpcError::Protocol(
                "Invalid find capability response".to_string(),
            ))
        }
    }

    /// Resolve a specific primal by name
    pub async fn resolve(&self, primal_name: &str) -> IpcResult<ServiceInfo> {
        debug!(primal_name, "Resolving primal");

        let key = crate::ipc_resolve_target_param_key();
        let mut params = serde_json::Map::new();
        params.insert(key, serde_json::Value::String(primal_name.to_string()));
        let request = JsonRpcRequest::new(
            "ipc.resolve",
            serde_json::Value::Object(params),
            self.next_request_id(),
        );

        let response = self.send_request(request).await?;

        if let Some(result) = response.result {
            serde_json::from_value(result)
                .map_err(|e| IpcError::Protocol(format!("Invalid service info: {e}")))
        } else if let Some(error) = response.error {
            Err(IpcError::Protocol(format!("Resolve error: {error:?}")))
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

        let request = JsonRpcRequest::new(
            "ipc.heartbeat",
            json!({
                "name": name
            }),
            self.next_request_id(),
        );

        let response = self.send_request(request).await?;

        if response.result.is_some() {
            debug!("Heartbeat sent");
            Ok(())
        } else if let Some(error) = response.error {
            warn!(?error, "Heartbeat error");
            Err(IpcError::Protocol(format!("Heartbeat error: {error:?}")))
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
            .map_err(|e| IpcError::Connection(format!("Failed to connect: {e}")))?;

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

#[cfg(test)]
impl SongbirdClient {
    pub(crate) fn with_socket_path_for_test(path: impl Into<String>) -> Self {
        Self {
            socket_path: path.into(),
            request_id: Arc::new(AtomicU64::new(1)),
            primal_name: Arc::new(RwLock::new(None)),
        }
    }

    /// Same validation as [`SongbirdClient::connect`], but uses a caller-provided socket path.
    pub(crate) async fn connect_test(self) -> IpcResult<Self> {
        let _stream = UnixStream::connect(&self.socket_path).await.map_err(|e| {
            IpcError::Connection(format!(
                "Cannot connect to IPC registry at {}: {}",
                self.socket_path, e
            ))
        })?;

        info!("✅ Connected to IPC registry at {}", self.socket_path);
        Ok(self)
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
    use std::path::Path;
    use std::sync::Arc;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixListener;
    use tokio::sync::Notify;
    use tokio::time::Duration;

    fn unique_sock_path() -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "beardog_songbird_test_{}_{}.sock",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ))
    }

    fn spawn_line_json_mock(
        path: &Path,
        register_ok: bool,
        ready: Arc<Notify>,
    ) -> tokio::task::JoinHandle<()> {
        let path = path.to_path_buf();
        tokio::spawn(async move {
            let _ = std::fs::remove_file(&path);
            let listener = UnixListener::bind(&path).expect("bind mock unix socket");
            ready.notify_one();
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    break;
                };
                let mut reader = BufReader::new(&mut stream);
                let mut line = String::new();
                if reader.read_line(&mut line).await.is_err() || line.trim().is_empty() {
                    continue;
                }
                let Ok(v) = serde_json::from_str::<serde_json::Value>(line.trim()) else {
                    let stream = reader.into_inner();
                    let _ = stream
                        .write_all(br#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"parse"},"id":null}"#)
                        .await;
                    continue;
                };
                let method = v["method"].as_str().unwrap_or("");
                let id = v["id"].as_u64().unwrap_or(1);
                let cap = v["params"]["capability"].as_str().unwrap_or("");
                let response = match method {
                    "ipc.register" => serde_json::json!({
                        "jsonrpc": "2.0",
                        "result": { "registered": register_ok },
                        "id": id
                    }),
                    "ipc.find_capability" if cap == "fail" => serde_json::json!({
                        "jsonrpc": "2.0",
                        "error": { "code": -1, "message": "find failed" },
                        "id": id
                    }),
                    "ipc.find_capability" => serde_json::json!({
                        "jsonrpc": "2.0",
                        "result": {
                            "services": [{
                                "name": "svc",
                                "endpoint": "/primal/svc",
                                "capabilities": ["crypto"],
                                "version": "1.0.0",
                                "available": true
                            }]
                        },
                        "id": id
                    }),
                    "ipc.resolve" => serde_json::json!({
                        "jsonrpc": "2.0",
                        "result": {
                            "name": "resolved",
                            "endpoint": "/primal/resolved",
                            "capabilities": [],
                            "version": "1.0.0",
                            "available": true
                        },
                        "id": id
                    }),
                    "ipc.heartbeat" => serde_json::json!({
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
                let stream = reader.into_inner();
                let body = serde_json::to_string(&response)
                    .expect("serialize ipc line-json mock response in test");
                let _ = stream.write_all(body.as_bytes()).await;
            }
        })
    }

    #[tokio::test]
    async fn test_connect_and_register_find_resolve_heartbeat() {
        let path = unique_sock_path();
        let ready = Arc::new(Notify::new());
        let _guard = spawn_line_json_mock(&path, true, Arc::clone(&ready));
        ready.notified().await;

        let client = SongbirdClient::with_socket_path_for_test(path.to_string_lossy().as_ref());
        let client = client.connect_test().await.expect("connect mock");

        client
            .register("beardog", vec![Capability::Crypto])
            .await
            .expect("register");

        let services = client.find_capability("crypto").await.expect("find");
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].name, "svc");

        let resolved = client.resolve("any").await.expect("resolve");
        assert_eq!(resolved.name, "resolved");

        client.heartbeat().await.expect("heartbeat");
        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn test_register_failure_branch() {
        let path = unique_sock_path();
        let ready = Arc::new(Notify::new());
        let _guard = spawn_line_json_mock(&path, false, Arc::clone(&ready));
        ready.notified().await;

        let client = SongbirdClient::with_socket_path_for_test(path.to_string_lossy().as_ref());
        let client = client
            .connect_test()
            .await
            .expect("connect mock for register failure test");

        let err = client
            .register("x", vec![Capability::Crypto])
            .await
            .expect_err("expected err");
        assert!(matches!(err, IpcError::Protocol(_)), "{err:?}");
        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn test_find_capability_error_branch() {
        let path = unique_sock_path();
        let ready = Arc::new(Notify::new());
        let _guard = spawn_line_json_mock(&path, true, Arc::clone(&ready));
        ready.notified().await;

        let client = SongbirdClient::with_socket_path_for_test(path.to_string_lossy().as_ref());
        let client = client
            .connect_test()
            .await
            .expect("connect mock for find_capability error test");

        let err = client
            .find_capability("fail")
            .await
            .expect_err("expected err");
        assert!(matches!(err, IpcError::Protocol(_)), "{err:?}");
        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn test_send_request_invalid_json_response() {
        let path = unique_sock_path();
        let ready = Arc::new(Notify::new());
        let ready_clone = Arc::clone(&ready);
        let path_clone = path.clone();
        let _guard = tokio::spawn(async move {
            let _ = std::fs::remove_file(&path_clone);
            let listener = UnixListener::bind(&path_clone).expect("bind");
            ready_clone.notify_one();
            let Ok((mut stream, _)) = listener.accept().await else {
                return;
            };
            let _ = stream.write_all(b"not-json").await;
        });
        ready.notified().await;

        let client = SongbirdClient::with_socket_path_for_test(path.to_string_lossy().as_ref());
        let err = client
            .find_capability("crypto")
            .await
            .expect_err("expected serialization err");
        assert!(matches!(err, IpcError::Serialization(_)), "{err:?}");
        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn test_heartbeat_not_registered() {
        let path = unique_sock_path();
        let ready = Arc::new(Notify::new());
        let _guard = spawn_line_json_mock(&path, true, Arc::clone(&ready));
        ready.notified().await;

        let client = SongbirdClient::with_socket_path_for_test(path.to_string_lossy().as_ref());
        let client = client
            .connect_test()
            .await
            .expect("connect mock for heartbeat-not-registered test");

        let err = client.heartbeat().await.expect_err("expected err");
        assert!(matches!(err, IpcError::Protocol(_)), "{err:?}");
        let _ = std::fs::remove_file(&path);
    }

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
            "Expected Connection or Io error, got {err:?}"
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

    #[test]
    fn songbird_client_debug_includes_socket() {
        let c = SongbirdClient::with_socket_path_for_test("/tmp/unit-test.sock");
        let s = format!("{c:?}");
        assert!(s.contains("SongbirdClient") || s.contains("socket"));
    }
}
