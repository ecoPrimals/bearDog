// SPDX-License-Identifier: AGPL-3.0-or-later

//! IPC registry client (Primal IPC Protocol)
//!
//! JSON-RPC 2.0 over transport-agnostic IPC to an orchestrator-agnostic service registry (`ipc.*` methods).

use crate::{
    DISCOVERY_SOCKET_FALLBACK, IpcSocketDiscoveryOptions,
    error::{IpcError, IpcResult},
    isomorphic::connect_raw,
    protocol::{JsonRpcRequest, JsonRpcResponse},
    resolve_ipc_socket_from_options,
    types::{Capability, ServiceInfo},
};
use beardog_types::btsp::TransportEndpoint;
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tokio::time::{Duration, interval};
use tracing::{debug, info, warn};

/// Client for the Primal IPC registry transport (capability discovery, `ipc.register`, etc.).
#[derive(Debug)]
pub struct OrchestratorRegistryClient {
    /// Registry Unix socket path (from env / capability discovery)
    socket_path: String,
    /// Request ID counter
    request_id: Arc<AtomicU64>,
    /// Registered primal name
    primal_name: Arc<RwLock<Option<String>>>,
}

impl OrchestratorRegistryClient {
    /// Create a new client (socket path defaults to [`DISCOVERY_SOCKET_FALLBACK`] until [`connect`](Self::connect)).
    pub fn new() -> Self {
        Self {
            socket_path: DISCOVERY_SOCKET_FALLBACK.to_string(),
            request_id: Arc::new(AtomicU64::new(1)),
            primal_name: Arc::new(RwLock::new(None)),
        }
    }

    /// Connect to the IPC registry (validates transport is reachable).
    ///
    /// # Errors
    ///
    /// Returns [`IpcError::Connection`] when the transport endpoint cannot be opened.
    pub async fn connect() -> IpcResult<Self> {
        let socket_path = resolve_ipc_socket_from_options(&IpcSocketDiscoveryOptions::from_env());
        let client = Self {
            socket_path,
            request_id: Arc::new(AtomicU64::new(1)),
            primal_name: Arc::new(RwLock::new(None)),
        };

        let endpoint = client.endpoint();
        let _stream = connect_raw(&endpoint).await.map_err(|e| {
            IpcError::Connection(format!("Cannot connect to IPC registry at {endpoint}: {e}",))
        })?;

        info!(endpoint = %endpoint, "Connected to IPC registry");
        Ok(client)
    }

    /// Register this primal with the IPC registry
    ///
    /// # Example
    /// ```no_run
    /// # use beardog_ipc::{OrchestratorRegistryClient, Capability};
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = OrchestratorRegistryClient::connect().await?;
    /// client.register(
    ///     "beardog",
    ///     vec![Capability::Crypto, Capability::BTSP],
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] when the JSON-RPC call fails or registration is rejected.
    pub async fn register(
        &self,
        primal_name: &str,
        capabilities: Vec<Capability>,
    ) -> IpcResult<()> {
        let cap_strings: Vec<String> = capabilities
            .iter()
            .map(|c| super::types::Capability::as_str(c).to_string())
            .collect();
        self.register_ipc(
            primal_name,
            &format!("/primal/{primal_name}"),
            &cap_strings,
            env!("CARGO_PKG_VERSION"),
        )
        .await
    }

    /// JSON-RPC `ipc.register` with explicit endpoint, capability strings, and version (wateringHole v3.1).
    ///
    /// Use this when the endpoint is the primal's Unix socket path (from `SocketConfig`) rather than
    /// the `/primal/<name>` logical path only.
    ///
    /// # Errors
    ///
    /// Returns [`IpcError`] when the JSON-RPC call fails or registration is rejected.
    pub async fn register_ipc(
        &self,
        primal_name: &str,
        endpoint: &str,
        capabilities: &[String],
        version: &str,
    ) -> IpcResult<()> {
        info!(
            primal_name,
            endpoint,
            cap_count = capabilities.len(),
            "Registering primal with IPC registry (ipc.register)"
        );

        let request = JsonRpcRequest::new(
            "ipc.register",
            json!({
                "name": primal_name,
                "endpoint": endpoint,
                "capabilities": capabilities,
                "version": version,
                "metadata": {
                    "description": format!("IPC service ({primal_name})")
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
    /// # use beardog_ipc::OrchestratorRegistryClient;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = OrchestratorRegistryClient::connect().await?;
    /// let crypto_services = client.find_capability("crypto").await?;
    /// for service in crypto_services {
    ///     println!("Found: {} at {}", service.name, service.endpoint);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`IpcError::Protocol`] when the response is invalid or reports an RPC error.
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
    ///
    /// # Errors
    ///
    /// Returns [`IpcError::Protocol`] when resolution fails or the payload cannot be parsed.
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
    ///
    /// # Errors
    ///
    /// Returns [`IpcError::Protocol`] when not registered or the heartbeat RPC fails.
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

    fn endpoint(&self) -> TransportEndpoint {
        TransportEndpoint::Uds {
            path: PathBuf::from(&self.socket_path),
        }
    }

    async fn send_request(&self, request: JsonRpcRequest) -> IpcResult<JsonRpcResponse> {
        use beardog_types::constants::domains::network::ribocipher;

        let endpoint = self.endpoint();
        let mut stream = connect_raw(&endpoint)
            .await
            .map_err(|e| IpcError::Connection(format!("Failed to connect to {endpoint}: {e}")))?;

        stream
            .write_all(&ribocipher::clear_signal(ribocipher::PROTO_NDJSON_JSONRPC))
            .await
            .map_err(IpcError::Io)?;

        let request_json =
            serde_json::to_vec(&request).map_err(|e| IpcError::Serialization(e.to_string()))?;

        stream
            .write_all(&request_json)
            .await
            .map_err(IpcError::Io)?;
        stream.write_all(b"\n").await.map_err(IpcError::Io)?;

        let mut buffer = vec![0u8; 8192];
        let n = stream.read(&mut buffer).await.map_err(IpcError::Io)?;

        let response: JsonRpcResponse = serde_json::from_slice(&buffer[..n])
            .map_err(|e| IpcError::Serialization(e.to_string()))?;

        Ok(response)
    }

    fn next_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }
}

impl Default for OrchestratorRegistryClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
impl OrchestratorRegistryClient {
    pub(crate) fn with_socket_path_for_test(path: impl Into<String>) -> Self {
        Self {
            socket_path: path.into(),
            request_id: Arc::new(AtomicU64::new(1)),
            primal_name: Arc::new(RwLock::new(None)),
        }
    }

    /// Same validation as [`OrchestratorRegistryClient::connect`], but uses a caller-provided socket path.
    pub(crate) async fn connect_test(self) -> IpcResult<Self> {
        let endpoint = self.endpoint();
        let _stream = connect_raw(&endpoint).await.map_err(|e| {
            IpcError::Connection(format!("Cannot connect to IPC registry at {endpoint}: {e}",))
        })?;

        info!(endpoint = %endpoint, "Connected to IPC registry");
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
                // Consume riboCipher signal prefix
                let mut sig = [0u8; 2];
                if tokio::io::AsyncReadExt::read_exact(&mut stream, &mut sig)
                    .await
                    .is_err()
                {
                    continue;
                }
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

        let client =
            OrchestratorRegistryClient::with_socket_path_for_test(path.to_string_lossy().as_ref());
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

        let client =
            OrchestratorRegistryClient::with_socket_path_for_test(path.to_string_lossy().as_ref());
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

        let client =
            OrchestratorRegistryClient::with_socket_path_for_test(path.to_string_lossy().as_ref());
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
            // Consume riboCipher signal prefix
            let mut sig = [0u8; 2];
            let _ = tokio::io::AsyncReadExt::read_exact(&mut stream, &mut sig).await;
            let _ = stream.write_all(b"not-json").await;
        });
        ready.notified().await;

        let client =
            OrchestratorRegistryClient::with_socket_path_for_test(path.to_string_lossy().as_ref());
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

        let client =
            OrchestratorRegistryClient::with_socket_path_for_test(path.to_string_lossy().as_ref());
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
        let client = OrchestratorRegistryClient::new();
        assert_eq!(client.socket_path, DISCOVERY_SOCKET_FALLBACK);
    }

    #[test]
    fn test_client_default() {
        let client = OrchestratorRegistryClient::default();
        assert_eq!(client.socket_path, DISCOVERY_SOCKET_FALLBACK);
    }

    #[test]
    fn test_request_id_increment() {
        let client = OrchestratorRegistryClient::new();
        assert_eq!(client.next_request_id(), 1);
        assert_eq!(client.next_request_id(), 2);
        assert_eq!(client.next_request_id(), 3);
    }

    #[tokio::test]
    async fn test_heartbeat_handle_creation_and_drop() {
        let client = OrchestratorRegistryClient::new();
        let handle = client.start_heartbeat(Duration::from_secs(3600));
        drop(handle);
    }

    #[tokio::test]
    async fn test_connect_requires_socket() {
        let result = OrchestratorRegistryClient::connect().await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, IpcError::Connection(_) | IpcError::Io(_)),
            "Expected Connection or Io error, got {err:?}"
        );
    }

    #[test]
    fn test_request_id_unique_per_client() {
        let c1 = OrchestratorRegistryClient::new();
        let c2 = OrchestratorRegistryClient::new();
        let id1 = c1.next_request_id();
        let id2 = c2.next_request_id();
        assert_eq!(id1, 1);
        assert_eq!(id2, 1);
    }

    #[test]
    fn songbird_client_debug_includes_socket() {
        let c = OrchestratorRegistryClient::with_socket_path_for_test("/tmp/unit-test.sock");
        let s = format!("{c:?}");
        assert!(s.contains("OrchestratorRegistryClient") || s.contains("socket"));
    }
}
