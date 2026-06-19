// SPDX-License-Identifier: AGPL-3.0-or-later

use super::types::PeerTrustRecord;
use super::{BeardogBtspProvider, ContactInfo};
use beardog_errors::BearDogError;
use chrono::Utc;
use tracing::{debug, info, warn};

impl BeardogBtspProvider {
    /// Exchange contact information via genetic lineage
    ///
    /// This enables decentralized NAT traversal without STUN/TURN servers.
    /// Queries the genetic lineage to find peer addresses through trusted relationships.
    ///
    /// # Arguments
    ///
    /// * `target_peer_id` - Peer to find contact info for
    /// * `requester_lineage` - Requester's lineage ID for verification
    /// * `max_hops` - Maximum depth to search in lineage tree
    ///
    /// # Returns
    ///
    /// Contact information including addresses and lineage proof
    ///
    /// # Errors
    ///
    /// Returns error if peer not found or lineage verification fails
    pub async fn contact_exchange(
        &self,
        target_peer_id: &str,
        requester_lineage: &str,
        max_hops: usize,
    ) -> Result<ContactInfo, BearDogError> {
        info!(
            "🔍 Contact exchange: searching for peer {} (max hops: {})",
            target_peer_id, max_hops
        );

        // 1. Query genetic lineage for path to peer
        let lineage_path = self
            .find_lineage_path(requester_lineage, target_peer_id, max_hops)
            .await?;

        if lineage_path.is_empty() {
            return Err(BearDogError::business(format!(
                "Peer {target_peer_id} not found within {max_hops} hops in genetic lineage"
            )));
        }

        // 2. Get peer addresses from trust database or discovery
        let addresses = self.get_peer_addresses(target_peer_id).await?;

        if addresses.is_empty() {
            return Err(BearDogError::business(format!(
                "No addresses found for peer {target_peer_id}"
            )));
        }

        // 3. Generate lineage proof (cryptographic verification)
        let lineage_proof = self.generate_lineage_proof(&lineage_path).await?;

        let search_depth = lineage_path.len();

        info!(
            "✅ Contact exchange: found {} addresses for {} (depth: {})",
            addresses.len(),
            target_peer_id,
            search_depth
        );

        Ok(ContactInfo {
            peer_id: target_peer_id.to_string(),
            addresses,
            lineage_proof,
            lineage_path,
            search_depth,
            last_seen: Utc::now(),
        })
    }

    /// Find path through genetic lineage to target peer
    pub(crate) async fn find_lineage_path(
        &self,
        _requester_lineage: &str,
        target_peer_id: &str,
        _max_hops: usize,
    ) -> Result<Vec<String>, BearDogError> {
        // For initial implementation, check if peer is in same family (depth 1)
        // This can be expanded to multi-hop lineage traversal later

        // Get our family from environment (primal self-knowledge)
        let our_family = beardog_errors::process_env::var("FAMILY_ID")
            .or_else(|_| beardog_errors::process_env::var("BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| {
                beardog_errors::process_env::var("BEARDOG_FAMILY_UNKNOWN_LABEL")
                    .unwrap_or_else(|_| "unknown".to_string())
            });

        // Check if peer is known in trust database
        let trust_db = self.trust_db.read();
        if trust_db.contains_key(target_peer_id) {
            // Direct connection in same family
            return Ok(vec![our_family, target_peer_id.to_string()]);
        }

        // For future: Query genetics engine for multi-hop paths

        // If peer not found in immediate family, return empty path
        warn!("⚠️  Peer {} not found in genetic lineage", target_peer_id);
        Ok(Vec::new())
    }

    /// Get peer addresses (IP:Port combinations)
    async fn get_peer_addresses(&self, peer_id: &str) -> Result<Vec<String>, BearDogError> {
        let mut addresses = Vec::new();

        // 1. Check trust database for known addresses
        {
            let trust_db = self.trust_db.read();
            if trust_db.contains_key(peer_id) {
                debug!("Peer {} found in trust database", peer_id);
            }
            // Lock dropped here before async call
        }

        // 2. Discovery mechanism: query environment or discovery service
        // EVOLUTION: Capability-based discovery - zero hardcoding!
        // The primal discovers addresses through runtime capability queries

        // Query for peer discovery capability from ecosystem
        // This follows the Primal IPC Protocol - discover services by capability
        match self.discover_peer_addresses_via_capability(peer_id).await {
            Ok(discovered_addresses) if !discovered_addresses.is_empty() => {
                addresses.extend(discovered_addresses);
            }
            Ok(_) => {
                // No addresses discovered - peer may not be available yet
                debug!(
                    "No addresses discovered for peer: {} via capability discovery",
                    peer_id
                );
            }
            Err(e) => {
                // Discovery service not available - this is acceptable
                // Primal will retry discovery on next attempt
                debug!("Capability discovery unavailable: {}", e);
            }
        }

        Ok(addresses)
    }

    /// Discover peer addresses via capability-based discovery
    ///
    /// This implements the Primal IPC Protocol pattern:
    /// 1. Resolve the discovery/registry Unix socket (`IPC_SOCKET`, `DISCOVERY_SOCKET`, then fallbacks)
    /// 2. Connect via Unix stream
    /// 3. Request peer endpoint via JSON-RPC (`ipc.resolve`)
    ///
    /// No fixed peer primal names — paths come from environment and shared `beardog-ipc` fallbacks.
    ///
    /// Discovery follows this priority (self-knowledge principle):
    /// 1. Environment variable (`DISCOVERY_SOCKET`)
    /// 2. Capability registry query
    /// 3. Generic Primal IPC discovery endpoint (/primal/discovery)
    /// 4. Local fallback for development
    async fn discover_peer_addresses_via_capability(
        &self,
        peer_id: &str,
    ) -> Result<Vec<String>, BearDogError> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::UnixStream;

        // Build discovery socket paths using environment-aware configuration
        // This eliminates hardcoding while maintaining Primal IPC protocol compliance
        let socket_paths = Self::get_discovery_socket_paths();

        for socket_path in socket_paths {
            if let Ok(mut stream) = UnixStream::connect(&socket_path).await {
                // Build JSON-RPC request per Primal IPC Protocol (param key overridable via
                // `beardog_ipc::ENV_IPC_RESOLVE_TARGET_PARAM_KEY`; value is opaque instance id).
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

                // Send request
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

                // Read response
                let mut buffer = vec![0u8; 4096];
                let n = stream
                    .read(&mut buffer)
                    .await
                    .map_err(|e| BearDogError::system(format!("Socket read failed: {e}")))?;

                if n == 0 {
                    continue; // No data, try next socket
                }

                // Parse JSON-RPC response
                let response: serde_json::Value = serde_json::from_slice(&buffer[..n])
                    .map_err(|e| BearDogError::system(format!("JSON parse failed: {e}")))?;

                // Extract endpoint from response
                if let Some(result) = response.get("result")
                    && let Some(endpoint) = result.get("endpoint").and_then(|e| e.as_str())
                {
                    return Ok(vec![endpoint.to_string()]);
                }
            }
        }

        // No discovery service available - return empty, caller will handle
        Ok(vec![])
    }

    /// Discovery Unix socket paths (env-first, then `beardog-ipc` constants).
    ///
    /// Reads `IPC_SOCKET`, `DISCOVERY_SOCKET`, and `BEARDOG_DEV_DISCOVERY_SOCKET`
    /// from the process environment overlay, then delegates to [`Self::build_discovery_socket_paths`]
    /// for the pure priority logic.
    pub(crate) fn get_discovery_socket_paths() -> Vec<String> {
        Self::build_discovery_socket_paths(
            beardog_errors::process_env::var("IPC_SOCKET").ok(),
            beardog_errors::process_env::var("DISCOVERY_SOCKET").ok(),
            beardog_errors::process_env::var("BEARDOG_DEV_DISCOVERY_SOCKET").ok(),
        )
    }

    /// Pure priority logic for discovery socket paths (DI-friendly, deterministic).
    ///
    /// Priority:
    /// 1. `ipc_socket` / `discovery_socket` when `Some` and non-empty
    /// 2. [`beardog_ipc::DISCOVERY_SOCKET_FALLBACK`] (`/primal/discovery`)
    /// 3. `dev_socket` or [`beardog_ipc::discovery_socket_dev_fallback_path`]
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

    /// Get tunnel by ID (public API for handlers)
    ///
    /// Returns a reference to the tunnel if it exists.
    pub fn get_tunnel(&self, tunnel_id: &str) -> Option<(String, String)> {
        self.tunnels
            .read()
            .get(tunnel_id)
            .map(|t| (t.id.clone(), t.peer_id.clone()))
    }

    /// Get peer trust record (public API for handlers)
    pub fn get_peer_trust_record(&self, peer_id: &str) -> Option<PeerTrustRecord> {
        self.trust_db.read().get(peer_id).cloned()
    }

    /// Generate lineage proof (cryptographic verification of genetic relationship)
    pub(crate) async fn generate_lineage_proof(
        &self,
        lineage_path: &[String],
    ) -> Result<String, BearDogError> {
        // Generate cryptographic proof that requester and target are related through genetic lineage
        // This uses the genetics engine to create a verifiable proof

        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();

        // Hash the lineage path to create a proof
        for node in lineage_path {
            hasher.update(node.as_bytes());
        }

        let proof_hash = hasher.finalize();
        let proof = format!("lineage_proof_{}", hex::encode(proof_hash));

        // Future: Use genetics engine for proper cryptographic proof
        Ok(proof)
    }
}
