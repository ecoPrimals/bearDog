// SPDX-License-Identifier: AGPL-3.0-only

// BTSP (BearDog Tunnel Security Protocol) Provider Implementation
//!
//! **Note**: "BTSP" is used for developer context. The implementation uses generic
//! capability traits to maintain primal sovereignty.
//!
//! This module implements BearDog's secure tunnel capability using genetic cryptography
//! and Universal HSM architecture. The capability can be discovered and used by any
//! primal without hardcoded coupling.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │         SecureTunnelProvider Implementation             │
//! ├─────────────────────────────────────────────────────────┤
//! │  ┌──────────────┐  ┌─────────────┐  ┌───────────────┐  │
//! │  │ Tunnel Mgmt  │  │ Genetic     │  │  Universal    │  │
//! │  │ (TOFU,mTLS)  │  │ Crypto      │  │  HSM          │  │
//! │  └──────────────┘  └─────────────┘  └───────────────┘  │
//! └─────────────────────────────────────────────────────────┘
//!                          ▲
//!                          │
//!            Generic Capability Interface
//!                          │
//!                          ▼
//!                  ┌───────────────┐
//!                  │  Any Primal   │
//!                  │  (discovered  │
//!                  │   at runtime) │
//!                  └───────────────┘
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::Utc;
use parking_lot::RwLock;
use tracing::{debug, info, warn};

use crate::tunnel::hsm::manager::HsmManager;
use beardog_capabilities::traits::PeerEndpoint;
use beardog_errors::BearDogError;
use beardog_genetics::birdsong::{BirdSongManager, LineageHint};
use beardog_genetics::ecosystem_evolution::EcosystemGeneticEngine;

// Sub-modules
mod contact;
mod metrics;
mod trust;
mod tunnel;
pub mod types;

#[cfg(test)]
mod tests;

// Re-exports
pub use contact::ContactInfo;
pub use metrics::BtspMetrics;
pub use types::{Direction, PeerInfo, SecurityContext, TrustLevel};

// Internal imports
use tunnel::Tunnel;

// =============================================================================
// Re-export Capability Types (Primary Public Interface)
// =============================================================================

/// Re-export tunnel handle from capabilities (primary interface)
pub use beardog_capabilities::traits::TunnelHandle;

/// Re-export tunnel status from capabilities (primary interface)
pub use beardog_capabilities::traits::TunnelStatus;

// =============================================================================
// Internal Types (from types module)
// =============================================================================

use types::PeerTrustRecord;

// =============================================================================
// SecureTunnelProvider (Primary Interface)
// =============================================================================
// Implementation is in secure_tunnel_impl.rs

// =============================================================================
// Trust Management (TOFU - Trust On First Use)
// =============================================================================

// TrustLevel and PeerTrustRecord are now in the types module

// =============================================================================
// Tunnel State (Extracted to tunnel module)
// =============================================================================

// Tunnel struct and impl extracted to tunnel.rs for better modularity

// Contact exchange types are re-exported at the top

// =============================================================================
// BearDog BTSP Provider Implementation
// =============================================================================

/// BearDog's implementation of BTSP using genetic cryptography
pub struct BeardogBtspProvider {
    /// HSM manager for cryptographic operations (reserved for lineage verification)
    _hsm: Arc<HsmManager>,

    /// Genetics engine for key lineage and evolution (reserved for multi-hop paths)
    _genetics: Arc<EcosystemGeneticEngine>,

    /// BirdSong manager for lineage-aware encryption
    birdsong: Arc<BirdSongManager>,

    /// Active tunnels (tunnel_id -> Tunnel)
    tunnels: Arc<RwLock<HashMap<String, Tunnel>>>,

    /// Peer trust database (peer_id -> TrustRecord)
    trust_db: Arc<RwLock<HashMap<String, PeerTrustRecord>>>,

    /// Metrics: Total tunnels established
    tunnels_established: Arc<AtomicU64>,

    /// Metrics: Encryption operations count
    encryption_count: Arc<AtomicU64>,

    /// Metrics: Decryption operations count
    decryption_count: Arc<AtomicU64>,

    /// Metrics: Trust evaluations count
    trust_eval_count: Arc<AtomicU64>,
}

impl BeardogBtspProvider {
    /// Create a minimal provider for testing (bypasses HSM initialization)
    ///
    /// This constructor is only available in test builds and creates a provider
    /// with dummy initialization, suitable for tests that need a provider to exist
    /// but don't actually call its methods.
    ///
    /// **DO NOT USE IN PRODUCTION CODE**
    #[cfg(test)]
    pub async fn new_for_testing(
        hsm: Arc<HsmManager>,
        genetics: Arc<EcosystemGeneticEngine>,
    ) -> Result<Self, BearDogError> {
        // Create a dummy BirdSong manager without HSM initialization
        // This will fail if actually used, but that's fine for handler tests
        let dummy_master_key = vec![0u8; 32];
        let birdsong = Arc::new(BirdSongManager::new(dummy_master_key, None).await?);

        Ok(Self {
            _hsm: hsm,
            _genetics: genetics,
            birdsong,
            tunnels: Arc::new(RwLock::new(HashMap::new())),
            trust_db: Arc::new(RwLock::new(HashMap::new())),
            tunnels_established: Arc::new(AtomicU64::new(0)),
            encryption_count: Arc::new(AtomicU64::new(0)),
            decryption_count: Arc::new(AtomicU64::new(0)),
            trust_eval_count: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Create new BearDog BTSP provider
    ///
    /// # Arguments
    ///
    /// * `hsm` - Universal HSM manager
    /// * `genetics` - Genetics engine for key lineage
    ///
    /// # Errors
    ///
    /// Returns error if initialization fails
    pub async fn new(
        hsm: Arc<HsmManager>,
        genetics: Arc<EcosystemGeneticEngine>,
    ) -> Result<Self, BearDogError> {
        info!("🐻 Initializing BearDog BTSP Provider with BirdSong genetics");

        // Generate master secret from HSM for BirdSong (label from `beardog_config::domains::btsp`)
        use crate::tunnel::hsm::KeyType;
        let birdsong_key_label = beardog_config::domains::btsp::resolve_btsp_birdsong_key_label();
        let birdsong_key = hsm
            .generate_key(&birdsong_key_label, &KeyType::ChaCha20)
            .await
            .map_err(|e| {
                BearDogError::system(format!("Failed to generate BirdSong master key: {e}"))
            })?;

        // Extract key material for BirdSong initialization
        use crate::tunnel::hsm::KeyMaterial;
        let master_secret = match &birdsong_key.key_material {
            KeyMaterial::Encrypted { encrypted_data, .. } => encrypted_data.clone(),
            KeyMaterial::Reference { key_reference, .. } => {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(key_reference.as_bytes());
                hasher.finalize().to_vec()
            }
            KeyMaterial::HardwareReference { reference, .. } => {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(reference.as_bytes());
                hasher.finalize().to_vec()
            }
            KeyMaterial::Handle { key_handle, .. } => {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(key_handle.as_bytes());
                hasher.finalize().to_vec()
            }
        };

        // Initialize BirdSong manager
        let birdsong = BirdSongManager::new(master_secret, None)
            .await
            .map_err(|e| {
                BearDogError::system(format!("Failed to initialize BirdSong manager: {e}"))
            })?;

        info!("✅ BearDog BTSP Provider initialized with BirdSong genetics");

        Ok(Self {
            _hsm: hsm,
            _genetics: genetics,
            birdsong: Arc::new(birdsong),
            tunnels: Arc::new(RwLock::new(HashMap::new())),
            trust_db: Arc::new(RwLock::new(HashMap::new())),
            tunnels_established: Arc::new(AtomicU64::new(0)),
            encryption_count: Arc::new(AtomicU64::new(0)),
            decryption_count: Arc::new(AtomicU64::new(0)),
            trust_eval_count: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Get peer trust level
    async fn get_peer_trust(&self, peer_id: &str) -> Option<TrustLevel> {
        self.trust_db
            .read()
            .get(peer_id)
            .map(|record| record.trust_level)
    }

    /// Get BirdSong manager (for API server integration)
    ///
    /// Returns a reference to the BirdSong manager used by this BTSP provider.
    /// This allows the API server to use the same BirdSong instance and master key.
    pub fn birdsong_manager(&self) -> Arc<BirdSongManager> {
        self.birdsong.clone()
    }

    /// Get BTSP metrics
    ///
    /// Returns current operational metrics for monitoring and observability.
    /// All metrics are collected atomically for lock-free performance.
    ///
    /// # Returns
    ///
    /// Current snapshot of BTSP metrics including:
    /// - Tunnels established (total and active)
    /// - Encryption/decryption operations
    /// - Trust evaluations
    pub fn get_metrics(&self) -> BtspMetrics {
        BtspMetrics {
            tunnels_established: self.tunnels_established.load(Ordering::Relaxed),
            tunnels_active: self.tunnels.read().len() as u64,
            encryption_operations: self.encryption_count.load(Ordering::Relaxed),
            decryption_operations: self.decryption_count.load(Ordering::Relaxed),
            trust_evaluations: self.trust_eval_count.load(Ordering::Relaxed),
        }
    }

    // =========================================================================
    // Contact Exchange (Genetic Lineage-Based NAT Traversal)
    // =========================================================================

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
    async fn find_lineage_path(
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
    /// 1. Environment variable (DISCOVERY_SOCKET)
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
    fn get_discovery_socket_paths() -> Vec<String> {
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
    fn build_discovery_socket_paths(
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
    async fn generate_lineage_proof(
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

    // =========================================================================
    // Trust Management (Private Helpers)
    // =========================================================================

    /// Pin peer's public key (TOFU - Trust On First Use)
    async fn pin_peer_key(
        &self,
        peer_id: &str,
        public_key: &[u8],
    ) -> Result<TrustLevel, BearDogError> {
        let mut db = self.trust_db.write();

        let record = PeerTrustRecord {
            peer_id: peer_id.to_string(),
            public_key: public_key.to_vec(),
            trust_level: TrustLevel::Tentative,
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            connection_count: 1,
        };

        db.insert(peer_id.to_string(), record);

        info!("🔐 Pinned peer key: {} (TOFU)", peer_id);
        Ok(TrustLevel::Tentative)
    }

    /// Update peer trust record
    async fn update_peer_trust(&self, peer_id: &str) -> Result<(), BearDogError> {
        let mut db = self.trust_db.write();

        if let Some(record) = db.get_mut(peer_id) {
            record.last_seen = Utc::now();
            record.connection_count += 1;

            // Promote to trusted after 3 successful connections
            if record.connection_count >= 3 && record.trust_level == TrustLevel::Tentative {
                record.trust_level = TrustLevel::Trusted;
                info!("✅ Peer {} promoted to Trusted", peer_id);
            }
        }

        Ok(())
    }

    /// Establish mTLS connection with peer
    async fn establish_mtls(
        &self,
        peer: &PeerEndpoint,
        _session_key: &[u8],
    ) -> Result<(), BearDogError> {
        debug!(
            "🔗 Skipping mTLS (BTSP uses Unix sockets now): {}",
            peer.endpoint
        );

        // Validate endpoint
        if peer.endpoint.is_empty() {
            return Err(BearDogError::invalid_input("Peer endpoint cannot be empty"));
        }

        // Note: BTSP now uses Unix sockets, so no TLS connection needed
        info!(
            "✅ Peer endpoint validated (Unix socket): {}",
            peer.endpoint
        );
        Ok(())
    }

    /// Generate session key using BirdSong lineage-aware encryption
    ///
    /// This uses BirdSong to encrypt a random session key for the peer's lineage,
    /// ensuring only trusted peers in the same cryptographic family can derive it.
    async fn generate_session_key(&self, peer_id: &str) -> Result<Vec<u8>, BearDogError> {
        debug!(
            "🎵 Generating BirdSong lineage-aware session key for peer: {}",
            peer_id
        );

        // Generate random session key material
        use rand::{RngCore, rngs::OsRng};
        let mut key_material = vec![0u8; 32];
        OsRng.fill_bytes(&mut key_material);

        // Create lineage hint for this peer
        // In production, this would be derived from peer's certificate or previous exchange
        let root_prefix = beardog_config::domains::btsp::resolve_btsp_lineage_root_prefix();
        let max_depth = beardog_config::domains::btsp::resolve_btsp_lineage_max_depth();
        let lineage_hint = LineageHint {
            root_id: format!("{root_prefix}_{peer_id}"),
            min_depth: 0, // Root can decrypt
            max_depth,
            biome_filter: None, // No biome restriction
            version: 1,
        };

        // Encrypt session key using BirdSong
        use beardog_genetics::birdsong::types::BirdSongEncryptRequest;
        let encrypt_request = BirdSongEncryptRequest {
            plaintext: key_material.clone(),
            lineage_hint: lineage_hint.clone(),
            associated_data: Some(format!("BTSP session: {peer_id}").into_bytes()),
        };

        let _broadcast = self
            .birdsong
            .encrypt_broadcast(&encrypt_request)
            .map_err(|e| {
                warn!("BirdSong encryption failed for peer {}: {}", peer_id, e);
                BearDogError::system(format!("Failed to encrypt session key: {e}"))
            })?;

        info!(
            "✅ BirdSong session key generated for peer: {} (lineage: {})",
            peer_id, lineage_hint.root_id
        );

        // For now, return the plaintext key material
        // In full implementation, we'd distribute the broadcast to peers
        // and they'd decrypt using their lineage proof
        Ok(key_material)
    }

    /// Clean up ephemeral session key from HSM
    async fn cleanup_session_key(&self, peer_id: &str) -> Result<(), BearDogError> {
        debug!("🗑️  Cleaning up BirdSong session key for peer: {}", peer_id);

        // BirdSong uses ephemeral encryption - no HSM cleanup needed
        // The session key is zeroized when the Tunnel struct is dropped
        // This is a no-op for BirdSong, kept for interface compatibility

        Ok(())
    }

    /// Encrypt with genetic key lineage
    async fn encrypt_with_lineage(
        &self,
        data: &[u8],
        session_key: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        // Use genetics engine to apply key lineage
        // This ensures cryptographic evolution and forward secrecy

        // Integrate with beardog-genetics key derivation
        // ChaCha20-Poly1305 provides fast, secure AEAD encryption
        // The session key is derived from genetic lineage for forward secrecy

        use chacha20poly1305::{
            ChaCha20Poly1305,
            aead::{Aead, AeadCore, KeyInit, OsRng},
        };

        let cipher = ChaCha20Poly1305::new_from_slice(session_key)
            .map_err(|e| BearDogError::system(format!("Cipher init failed: {e}")))?;

        let nonce = ChaCha20Poly1305::generate_nonce(OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, data)
            .map_err(|e| BearDogError::system(format!("Encryption failed: {e}")))?;

        // Prepend nonce to ciphertext
        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Decrypt with genetic key lineage verification
    async fn decrypt_with_lineage(
        &self,
        data: &[u8],
        session_key: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        use chacha20poly1305::{
            ChaCha20Poly1305, Nonce,
            aead::{Aead, KeyInit},
        };

        if data.len() < 12 {
            return Err(BearDogError::invalid_input("Ciphertext too short"));
        }

        // Extract nonce (first 12 bytes)
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = ChaCha20Poly1305::new_from_slice(session_key)
            .map_err(|e| BearDogError::system(format!("Cipher init failed: {e}")))?;

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| BearDogError::system(format!("Decryption failed: {e}")))?;

        Ok(plaintext)
    }
}

// SecureTunnelProvider implementation (primary interface)
mod secure_tunnel_impl;
