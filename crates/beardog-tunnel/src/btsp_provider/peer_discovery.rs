// SPDX-License-Identifier: AGPL-3.0-or-later

//! Peer address discovery via IPC capability resolution.

use super::BeardogBtspProvider;
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use tracing::debug;

impl BeardogBtspProvider {
    pub(super) async fn get_peer_addresses(
        &self,
        peer_id: &str,
    ) -> Result<Vec<String>, BearDogError> {
        let mut addresses = Vec::new();

        {
            let trust_db = self.trust_db.read();
            if trust_db.contains_key(peer_id) {
                debug!("Peer {} found in trust database", peer_id);
            }
        }

        match self.discover_peer_addresses_via_capability(peer_id).await {
            Ok(discovered_addresses) if !discovered_addresses.is_empty() => {
                addresses.extend(discovered_addresses);
            }
            Ok(_) => {
                debug!(
                    "No addresses discovered for peer: {} via capability discovery",
                    peer_id
                );
            }
            Err(e) => {
                debug!("Capability discovery unavailable: {}", e);
            }
        }

        Ok(addresses)
    }

    async fn discover_peer_addresses_via_capability(
        &self,
        peer_id: &str,
    ) -> Result<Vec<String>, BearDogError> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::UnixStream;

        let socket_paths = Self::get_discovery_socket_paths();

        for socket_path in socket_paths {
            if let Ok(mut stream) = UnixStream::connect(&socket_path).await {
                let mut params = serde_json::Map::new();
                params.insert(
                    beardog_ipc::ipc_resolve_target_param_key(),
                    serde_json::Value::String(peer_id.to_string()),
                );
                let request = serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "ipc.resolve",
                    "params": params,
                    "id": 1
                });

                let request_bytes = serde_json::to_vec(&request)
                    .map_err(|e| BearDogError::system(format!("JSON serialization failed: {e}")))?;

                stream
                    .write_all(&request_bytes)
                    .await
                    .map_err(|e| BearDogError::system(format!("Socket write failed: {e}")))?;
                stream
                    .write_all(b"\n")
                    .await
                    .map_err(|e| BearDogError::system(format!("Socket write failed: {e}")))?;

                let mut buffer = vec![0u8; 4096];
                let n = stream
                    .read(&mut buffer)
                    .await
                    .map_err(|e| BearDogError::system(format!("Socket read failed: {e}")))?;

                if n == 0 {
                    continue;
                }

                let response: serde_json::Value = serde_json::from_slice(&buffer[..n])
                    .map_err(|e| BearDogError::system(format!("JSON parse failed: {e}")))?;

                if let Some(result) = response.get("result")
                    && let Some(endpoint) = result.get("endpoint").and_then(|e| e.as_str())
                {
                    return Ok(vec![endpoint.to_string()]);
                }
            }
        }

        Ok(vec![])
    }

    pub(crate) fn get_discovery_socket_paths() -> Vec<String> {
        Self::build_discovery_socket_paths(
            beardog_errors::process_env::var(env_keys::ENV_IPC_SOCKET).ok(),
            beardog_errors::process_env::var(env_keys::ENV_DISCOVERY_SOCKET).ok(),
            beardog_errors::process_env::var(env_keys::ENV_DEV_DISCOVERY_SOCKET).ok(),
        )
    }

    pub(crate) fn build_discovery_socket_paths(
        ipc_socket: Option<String>,
        discovery_socket: Option<String>,
        dev_socket: Option<String>,
    ) -> Vec<String> {
        let mut paths = Vec::new();
        for val in [ipc_socket, discovery_socket] {
            if let Some(s) = val
                && !s.is_empty()
                && !paths.contains(&s)
            {
                paths.push(s);
            }
        }
        let generic = beardog_ipc::DISCOVERY_SOCKET_FALLBACK.to_string();
        if !paths.contains(&generic) {
            paths.push(generic);
        }
        let dev = dev_socket.unwrap_or_else(beardog_ipc::discovery_socket_dev_fallback_path);
        if !paths.contains(&dev) {
            paths.push(dev);
        }
        paths
    }
}
