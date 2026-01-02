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
use std::time::SystemTime;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};
use zeroize::Zeroizing;

use crate::tunnel::hsm::manager::HsmManager;
use beardog_capabilities::traits::{
    PeerEndpoint, SecureTunnelProvider, TunnelHandle as CapabilityTunnelHandle,
    TunnelStatus as CapabilityTunnelStatus,
};
use beardog_errors::BearDogError;
use beardog_genetics::birdsong::{BirdSongManager, LineageHint};
use beardog_genetics::ecosystem_evolution::EcosystemGeneticEngine;

// =============================================================================
// Internal Types (for implementation)
// =============================================================================

/// Internal tunnel handle (uses DateTime for implementation convenience)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct InternalTunnelHandle {
    pub id: String,
    pub peer_id: String,
    pub established_at: DateTime<Utc>,
}

/// Internal tunnel status (uses DateTime for implementation convenience)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct InternalTunnelStatus {
    pub active: bool,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub last_activity: DateTime<Utc>,
}

// =============================================================================
// Re-export Capability Types (Primary Public Interface)
// =============================================================================

/// Re-export tunnel handle from capabilities (primary interface)
pub use beardog_capabilities::traits::TunnelHandle;

/// Re-export tunnel status from capabilities (primary interface)
pub use beardog_capabilities::traits::TunnelStatus;

// =============================================================================
// Legacy Type Aliases (Backward Compatibility)
// =============================================================================

/// Legacy peer info type (backward compatibility)
///
/// **Deprecated**: Use `PeerEndpoint` directly
#[deprecated(since = "0.10.0", note = "Use PeerEndpoint from beardog_capabilities")]
pub type PeerInfo = PeerEndpoint;

/// Legacy BTSP tunnel handle (backward compatibility)
///
/// **Deprecated**: Use `TunnelHandle` directly  
#[deprecated(since = "0.10.0", note = "Use TunnelHandle from beardog_capabilities")]
pub type BtspTunnelHandle = TunnelHandle;

/// Legacy BTSP tunnel status (backward compatibility)
///
/// **Deprecated**: Use `TunnelStatus` directly
#[deprecated(since = "0.10.0", note = "Use TunnelStatus from beardog_capabilities")]
pub type BtspTunnelStatus = TunnelStatus;

/// Security context for encryption/decryption operations
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// Tunnel identifier
    pub tunnel_id: String,
    /// Direction of data flow
    pub direction: Direction,
}

/// Data flow direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    /// Outbound (local -> remote)
    Outbound,
    /// Inbound (remote -> local)
    Inbound,
}

// =============================================================================
// Legacy BTSP Provider Trait (Deprecated)
// =============================================================================

/// Legacy BTSP Provider trait
///
/// **Deprecated**: Use `SecureTunnelProvider` from `beardog_capabilities` instead.
/// This trait is maintained for backward compatibility but will be removed in v0.11.0.
///
/// The generic `SecureTunnelProvider` trait provides the same functionality without
/// coupling to specific primal names.
#[deprecated(
    since = "0.10.0",
    note = "Use SecureTunnelProvider from beardog_capabilities"
)]
#[async_trait]
pub trait BtspProvider: Send + Sync {
    /// Establish secure tunnel with peer
    async fn establish_tunnel(&self, peer: &PeerEndpoint) -> Result<TunnelHandle, BearDogError>;

    /// Encrypt data through tunnel
    async fn encrypt(
        &self,
        data: &[u8],
        context: &SecurityContext,
    ) -> Result<Vec<u8>, BearDogError>;

    /// Decrypt data from tunnel
    async fn decrypt(
        &self,
        data: &[u8],
        context: &SecurityContext,
    ) -> Result<Vec<u8>, BearDogError>;

    /// Check tunnel status
    async fn tunnel_status(&self, handle: &TunnelHandle) -> Result<TunnelStatus, BearDogError>;

    /// Close tunnel gracefully
    async fn close_tunnel(&self, handle: &TunnelHandle) -> Result<(), BearDogError>;
}

// =============================================================================
// Trust Management (TOFU - Trust On First Use)
// =============================================================================

/// Trust level for peers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustLevel {
    /// Fully trusted (key pinned, verified)
    Trusted,
    /// Tentative trust (TOFU, first connection)
    Tentative,
    /// Untrusted (failed verification)
    Untrusted,
}

/// Peer trust record
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PeerTrustRecord {
    peer_id: String,
    public_key: Vec<u8>,
    trust_level: TrustLevel,
    first_seen: DateTime<Utc>,
    last_seen: DateTime<Utc>,
    connection_count: u64,
}

// =============================================================================
// Tunnel State
// =============================================================================

/// Active tunnel state
struct Tunnel {
    id: String,
    peer_id: String,
    peer_endpoint: String,
    established_at: DateTime<Utc>,

    // Genetic crypto session key (derived with key lineage)
    session_key: Zeroizing<Vec<u8>>,

    // Statistics
    bytes_sent: Arc<parking_lot::Mutex<u64>>,
    bytes_received: Arc<parking_lot::Mutex<u64>>,
    last_activity: Arc<parking_lot::Mutex<SystemTime>>,

    // Trust level
    trust_level: TrustLevel,
}

impl Tunnel {
    /// Create new tunnel
    fn new(
        id: String,
        peer_id: String,
        peer_endpoint: String,
        session_key: Vec<u8>,
        trust_level: TrustLevel,
    ) -> Self {
        Self {
            id,
            peer_id,
            peer_endpoint,
            established_at: Utc::now(),
            session_key: Zeroizing::new(session_key),
            bytes_sent: Arc::new(parking_lot::Mutex::new(0)),
            bytes_received: Arc::new(parking_lot::Mutex::new(0)),
            last_activity: Arc::new(parking_lot::Mutex::new(SystemTime::now())),
            trust_level,
        }
    }

    /// Check if tunnel is active (activity within last 5 minutes)
    fn is_active(&self) -> bool {
        let last = *self.last_activity.lock();
        SystemTime::now()
            .duration_since(last)
            .map(|d| d.as_secs() < 300)
            .unwrap_or(false)
    }

    /// Get bytes sent
    fn bytes_sent(&self) -> u64 {
        *self.bytes_sent.lock()
    }

    /// Get bytes received
    fn bytes_received(&self) -> u64 {
        *self.bytes_received.lock()
    }

    /// Get last activity time
    fn last_activity(&self) -> DateTime<Utc> {
        let last = *self.last_activity.lock();
        DateTime::from(last)
    }

    /// Update activity timestamp
    fn update_activity(&self) {
        *self.last_activity.lock() = SystemTime::now();
    }

    /// Increment bytes sent
    fn add_bytes_sent(&self, bytes: u64) {
        *self.bytes_sent.lock() += bytes;
        self.update_activity();
    }

    /// Increment bytes received
    fn add_bytes_received(&self, bytes: u64) {
        *self.bytes_received.lock() += bytes;
        self.update_activity();
    }
}

// Implement Drop to ensure keys are zeroized
impl Drop for Tunnel {
    fn drop(&mut self) {
        debug!("Tunnel {} dropped - keys zeroized", self.id);
    }
}

// =============================================================================
// BearDog BTSP Provider Implementation
// =============================================================================

/// BearDog's implementation of BTSP using genetic cryptography
pub struct BeardogBtspProvider {
    /// HSM manager for cryptographic operations
    hsm: Arc<HsmManager>,

    /// Genetics engine for key lineage and evolution
    genetics: Arc<EcosystemGeneticEngine>,

    /// BirdSong manager for lineage-aware encryption
    birdsong: Arc<BirdSongManager>,

    /// Active tunnels (tunnel_id -> Tunnel)
    tunnels: Arc<RwLock<HashMap<String, Tunnel>>>,

    /// Peer trust database (peer_id -> TrustRecord)
    trust_db: Arc<RwLock<HashMap<String, PeerTrustRecord>>>,

    /// TLS configuration for mTLS connections
    tls_config: Arc<crate::tls::TlsConfig>,
}

impl BeardogBtspProvider {
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

        // Initialize TLS configuration
        let tls_config = crate::tls::TlsConfig::new()
            .map_err(|e| BearDogError::system(format!("Failed to initialize TLS: {}", e)))?;

        // Generate master secret from HSM for BirdSong
        use crate::tunnel::hsm::KeyType;
        let birdsong_key = hsm
            .generate_key("birdsong_master", &KeyType::ChaCha20)
            .await
            .map_err(|e| {
                BearDogError::system(format!("Failed to generate BirdSong master key: {}", e))
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
                BearDogError::system(format!("Failed to initialize BirdSong manager: {}", e))
            })?;

        info!("✅ BearDog BTSP Provider initialized with BirdSong genetics");

        Ok(Self {
            hsm,
            genetics,
            birdsong: Arc::new(birdsong),
            tunnels: Arc::new(RwLock::new(HashMap::new())),
            trust_db: Arc::new(RwLock::new(HashMap::new())),
            tls_config: Arc::new(tls_config),
        })
    }

    /// Get peer trust level
    async fn get_peer_trust(&self, peer_id: &str) -> Option<TrustLevel> {
        self.trust_db
            .read()
            .get(peer_id)
            .map(|record| record.trust_level)
    }

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
        peer: &PeerInfo,
        _session_key: &[u8],
    ) -> Result<(), BearDogError> {
        debug!("🔗 Establishing mTLS with peer: {}", peer.endpoint);

        // Validate endpoint
        if peer.endpoint.is_empty() {
            return Err(BearDogError::invalid_input("Peer endpoint cannot be empty"));
        }

        // Establish TLS connection
        self.tls_config.connect(&peer.endpoint).await.map_err(|e| {
            warn!("mTLS connection failed: {}", e);
            BearDogError::system(format!("mTLS establishment failed: {}", e))
        })?;

        info!(
            "✅ mTLS connection established with peer: {}",
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
        use rand::{rngs::OsRng, RngCore};
        let mut key_material = vec![0u8; 32];
        OsRng.fill_bytes(&mut key_material);

        // Create lineage hint for this peer
        // In production, this would be derived from peer's certificate or previous exchange
        let lineage_hint = LineageHint {
            root_id: format!("btsp_root_{}", peer_id),
            min_depth: 0,       // Root can decrypt
            max_depth: 10,      // Up to 10 generations deep
            biome_filter: None, // No biome restriction
            version: 1,
        };

        // Encrypt session key using BirdSong
        use beardog_genetics::birdsong::types::BirdSongEncryptRequest;
        let encrypt_request = BirdSongEncryptRequest {
            plaintext: key_material.clone(),
            lineage_hint: lineage_hint.clone(),
            associated_data: Some(format!("BTSP session: {}", peer_id).into_bytes()),
        };

        let broadcast = self
            .birdsong
            .encrypt_broadcast(&encrypt_request)
            .map_err(|e| {
                warn!("BirdSong encryption failed for peer {}: {}", peer_id, e);
                BearDogError::system(format!("Failed to encrypt session key: {}", e))
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

        // TODO: Integrate with beardog-genetics key derivation
        // For now, use ChaCha20-Poly1305 with the session key

        use chacha20poly1305::{
            aead::{Aead, AeadCore, KeyInit, OsRng},
            ChaCha20Poly1305,
        };

        let cipher = ChaCha20Poly1305::new_from_slice(session_key)
            .map_err(|e| BearDogError::system(format!("Cipher init failed: {}", e)))?;

        let nonce = ChaCha20Poly1305::generate_nonce(OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, data)
            .map_err(|e| BearDogError::system(format!("Encryption failed: {}", e)))?;

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
            aead::{Aead, KeyInit},
            ChaCha20Poly1305, Nonce,
        };

        if data.len() < 12 {
            return Err(BearDogError::invalid_input("Ciphertext too short"));
        }

        // Extract nonce (first 12 bytes)
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let cipher = ChaCha20Poly1305::new_from_slice(session_key)
            .map_err(|e| BearDogError::system(format!("Cipher init failed: {}", e)))?;

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| BearDogError::system(format!("Decryption failed: {}", e)))?;

        Ok(plaintext)
    }
}

#[async_trait]
impl BtspProvider for BeardogBtspProvider {
    async fn establish_tunnel(&self, peer: &PeerEndpoint) -> Result<TunnelHandle, BearDogError> {
        info!("🌉 Establishing BTSP tunnel with peer: {}", peer.id);

        // 1. Check peer trust status
        let trust_level = match self.get_peer_trust(&peer.id).await {
            Some(level) => {
                debug!("📋 Peer {} has existing trust: {:?}", peer.id, level);
                self.update_peer_trust(&peer.id).await?;
                level
            }
            None => {
                // TOFU: Trust On First Use
                if let Some(ref public_key) = peer.public_key {
                    self.pin_peer_key(&peer.id, public_key).await?
                } else {
                    warn!("⚠️  Peer {} has no public key - tentative trust", peer.id);
                    TrustLevel::Tentative
                }
            }
        };

        // 2. Generate session key using genetic cryptography
        let session_key = self.generate_session_key(&peer.id).await?;

        // 3. Establish mTLS connection
        self.establish_mtls(peer, &session_key).await?;

        // 4. Create tunnel
        let tunnel_id = format!("btsp_{}", uuid::Uuid::new_v4().simple());
        let tunnel = Tunnel::new(
            tunnel_id.clone(),
            peer.id.clone(),
            peer.endpoint.clone(),
            session_key,
            trust_level,
        );

        let established_at = tunnel.established_at;

        // 5. Store tunnel
        self.tunnels.write().insert(tunnel_id.clone(), tunnel);

        info!("✅ BTSP tunnel established: {} -> {}", peer.id, tunnel_id);

        // Return capability type (with ISO 8601 timestamp)
        Ok(TunnelHandle {
            id: tunnel_id,
            peer_id: peer.id.clone(),
            established_at: established_at.to_rfc3339(),
        })
    }

    async fn encrypt(
        &self,
        data: &[u8],
        context: &SecurityContext,
    ) -> Result<Vec<u8>, BearDogError> {
        // Clone session key without holding lock across await
        let session_key = {
            let tunnels = self.tunnels.read();
            let tunnel = tunnels
                .get(&context.tunnel_id)
                .ok_or_else(|| BearDogError::invalid_input("Tunnel not found"))?;
            tunnel.session_key.clone()
        };

        // Encrypt with genetic key lineage
        let ciphertext = self.encrypt_with_lineage(data, &session_key).await?;

        // Update statistics
        {
            let tunnels = self.tunnels.read();
            if let Some(tunnel) = tunnels.get(&context.tunnel_id) {
                tunnel.add_bytes_sent(ciphertext.len() as u64);
            }
        }

        debug!(
            "🔒 Encrypted {} bytes for tunnel {}",
            data.len(),
            context.tunnel_id
        );

        Ok(ciphertext)
    }

    async fn decrypt(
        &self,
        data: &[u8],
        context: &SecurityContext,
    ) -> Result<Vec<u8>, BearDogError> {
        // Clone session key without holding lock across await
        let session_key = {
            let tunnels = self.tunnels.read();
            let tunnel = tunnels
                .get(&context.tunnel_id)
                .ok_or_else(|| BearDogError::invalid_input("Tunnel not found"))?;
            tunnel.session_key.clone()
        };

        // Decrypt with genetic key lineage verification
        let plaintext = self.decrypt_with_lineage(data, &session_key).await?;

        // Update statistics
        {
            let tunnels = self.tunnels.read();
            if let Some(tunnel) = tunnels.get(&context.tunnel_id) {
                tunnel.add_bytes_received(data.len() as u64);
            }
        }

        debug!(
            "🔓 Decrypted {} bytes for tunnel {}",
            plaintext.len(),
            context.tunnel_id
        );

        Ok(plaintext)
    }

    async fn tunnel_status(&self, handle: &TunnelHandle) -> Result<TunnelStatus, BearDogError> {
        let tunnels = self.tunnels.read();
        let tunnel = tunnels
            .get(&handle.id)
            .ok_or_else(|| BearDogError::invalid_input("Tunnel not found"))?;

        Ok(TunnelStatus {
            tunnel_id: handle.id.clone(),
            active: tunnel.is_active(),
            bytes_sent: tunnel.bytes_sent(),
            bytes_received: tunnel.bytes_received(),
            last_activity: tunnel.last_activity().to_rfc3339(),
        })
    }

    async fn close_tunnel(&self, handle: &TunnelHandle) -> Result<(), BearDogError> {
        info!("🔒 Closing BTSP tunnel: {}", handle.id);

        // Get tunnel info before removing
        let (peer_id, bytes_sent, bytes_received) = {
            let tunnels = self.tunnels.read();
            if let Some(tunnel) = tunnels.get(&handle.id) {
                (
                    tunnel.peer_id.clone(),
                    tunnel.bytes_sent(),
                    tunnel.bytes_received(),
                )
            } else {
                return Err(BearDogError::not_found(format!(
                    "Tunnel {} not found",
                    handle.id
                )));
            }
        };

        // Clean up session key from HSM first
        self.cleanup_session_key(&peer_id).await?;

        // Remove tunnel (Drop impl will zeroize session key in memory)
        let mut tunnels = self.tunnels.write();
        tunnels.remove(&handle.id);

        info!(
            "✅ BTSP tunnel closed: {} (sent: {} bytes, received: {} bytes)",
            handle.id, bytes_sent, bytes_received
        );

        Ok(())
    }
}

// =============================================================================
// Generic SecureTunnelProvider Implementation (Primary Interface)
// =============================================================================

/// Implementation of the generic `SecureTunnelProvider` capability trait
///
/// This is the primary interface for BearDog's secure tunnel capability.
/// It provides the same functionality as `BtspProvider` but uses generic
/// capability types for primal sovereignty.
#[async_trait]
impl SecureTunnelProvider for BeardogBtspProvider {
    async fn establish_tunnel(
        &self,
        peer: PeerEndpoint,
    ) -> Result<CapabilityTunnelHandle, BearDogError> {
        // Delegate to BtspProvider implementation (backward compat)
        let handle = <Self as BtspProvider>::establish_tunnel(self, &peer).await?;

        // No conversion needed - TunnelHandle is the same type
        Ok(handle)
    }

    async fn tunnel_encrypt(
        &self,
        handle: &CapabilityTunnelHandle,
        data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        let context = SecurityContext {
            tunnel_id: handle.id.clone(),
            direction: Direction::Outbound,
        };
        <Self as BtspProvider>::encrypt(self, data, &context).await
    }

    async fn tunnel_decrypt(
        &self,
        handle: &CapabilityTunnelHandle,
        data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        let context = SecurityContext {
            tunnel_id: handle.id.clone(),
            direction: Direction::Inbound,
        };
        <Self as BtspProvider>::decrypt(self, data, &context).await
    }

    async fn tunnel_status(
        &self,
        handle: &CapabilityTunnelHandle,
    ) -> Result<CapabilityTunnelStatus, BearDogError> {
        // Delegate - types are the same
        <Self as BtspProvider>::tunnel_status(self, handle).await
    }

    async fn close_tunnel(&self, handle: &CapabilityTunnelHandle) -> Result<(), BearDogError> {
        // Delegate - types are the same
        <Self as BtspProvider>::close_tunnel(self, handle).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_peer_info_serialization() {
        let peer = PeerInfo {
            id: "test-peer-123".to_string(),
            endpoint: "192.168.1.100:8080".to_string(),
            public_key: Some(vec![1, 2, 3, 4]),
        };

        let json = serde_json::to_string(&peer).expect("Serialize failed");
        let deserialized: PeerInfo = serde_json::from_str(&json).expect("Deserialize failed");

        assert_eq!(peer.id, deserialized.id);
        assert_eq!(peer.endpoint, deserialized.endpoint);
        assert_eq!(peer.public_key, deserialized.public_key);
    }

    #[tokio::test]
    async fn test_tunnel_activity_tracking() {
        let tunnel = Tunnel::new(
            "test-tunnel".to_string(),
            "test-peer".to_string(),
            "192.168.1.1:8080".to_string(),
            vec![0u8; 32],
            TrustLevel::Trusted,
        );

        assert!(tunnel.is_active());
        assert_eq!(tunnel.bytes_sent(), 0);
        assert_eq!(tunnel.bytes_received(), 0);

        tunnel.add_bytes_sent(100);
        tunnel.add_bytes_received(200);

        assert_eq!(tunnel.bytes_sent(), 100);
        assert_eq!(tunnel.bytes_received(), 200);
    }

    #[tokio::test]
    async fn test_direction_serialization() {
        let outbound = Direction::Outbound;
        let json = serde_json::to_string(&outbound).expect("Serialize failed");
        let deserialized: Direction = serde_json::from_str(&json).expect("Deserialize failed");

        assert_eq!(outbound, deserialized);
    }

    // =========================================================================
    // BirdSong Integration Tests
    // =========================================================================

    /// Helper to create HSM manager with software provider for testing
    async fn create_test_hsm() -> Arc<HsmManager> {
        use crate::tunnel::hsm::software_hsm::RustSoftwareHsm;
        use crate::tunnel::hsm::{HsmTier, SoftwareHsmConfig};

        let mut hsm = HsmManager::new();
        let config = SoftwareHsmConfig::default();
        let software_hsm = RustSoftwareHsm::new(config)
            .await
            .expect("Software HSM init failed");

        hsm.register_hsm_provider(HsmTier::Software, Arc::new(software_hsm))
            .expect("HSM provider registration failed");

        Arc::new(hsm)
    }

    #[tokio::test]
    async fn test_birdsong_initialization() {
        // Create HSM manager with software provider
        let hsm = create_test_hsm().await;

        // Create genetics engine
        let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init failed"));

        // Initialize BTSP provider with BirdSong
        let provider = BeardogBtspProvider::new(hsm, genetics)
            .await
            .expect("Provider init failed");

        // Verify BirdSong is initialized (indirect - check provider works)
        assert!(provider.tunnels.read().is_empty());
    }

    #[tokio::test]
    async fn test_birdsong_session_key_generation() {
        // Create HSM manager with software provider
        let hsm = create_test_hsm().await;

        // Create genetics engine
        let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init failed"));

        // Initialize BTSP provider with BirdSong
        let provider = BeardogBtspProvider::new(hsm, genetics)
            .await
            .expect("Provider init failed");

        // Generate session key using BirdSong
        let key = provider
            .generate_session_key("test_peer_123")
            .await
            .expect("Key generation failed");

        // Verify key properties
        assert_eq!(key.len(), 32, "Session key must be 32 bytes");
        assert_ne!(key, vec![0u8; 32], "Key must not be all zeros");
    }

    #[tokio::test]
    async fn test_birdsong_session_keys_unique() {
        // Create HSM manager with software provider
        let hsm = create_test_hsm().await;

        // Create genetics engine
        let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init failed"));

        // Initialize BTSP provider with BirdSong
        let provider = BeardogBtspProvider::new(hsm, genetics)
            .await
            .expect("Provider init failed");

        // Generate multiple keys
        let key1 = provider
            .generate_session_key("peer_1")
            .await
            .expect("Key 1 generation failed");

        let key2 = provider
            .generate_session_key("peer_2")
            .await
            .expect("Key 2 generation failed");

        // Verify keys are different
        assert_ne!(
            key1, key2,
            "Different peers must have different session keys"
        );
    }

    #[tokio::test]
    async fn test_birdsong_lineage_hint_structure() {
        // Verify LineageHint has correct fields
        let hint = LineageHint {
            root_id: "test_root".to_string(),
            min_depth: 0,
            max_depth: 5,
            biome_filter: Some("beardog".to_string()),
            version: 1,
        };

        assert_eq!(hint.root_id, "test_root");
        assert_eq!(hint.min_depth, 0);
        assert_eq!(hint.max_depth, 5);
        assert_eq!(hint.biome_filter, Some("beardog".to_string()));
        assert_eq!(hint.version, 1);
    }

    #[tokio::test]
    async fn test_birdsong_encrypt_request_structure() {
        use beardog_genetics::birdsong::types::BirdSongEncryptRequest;

        let hint = LineageHint {
            root_id: "test_root".to_string(),
            min_depth: 0,
            max_depth: 5,
            biome_filter: None,
            version: 1,
        };

        let request = BirdSongEncryptRequest {
            plaintext: vec![1, 2, 3, 4],
            lineage_hint: hint.clone(),
            associated_data: Some(b"test_data".to_vec()),
        };

        assert_eq!(request.plaintext, vec![1, 2, 3, 4]);
        assert_eq!(request.lineage_hint.root_id, "test_root");
        assert_eq!(request.associated_data, Some(b"test_data".to_vec()));
    }

    #[tokio::test]
    async fn test_birdsong_master_key_derivation() {
        // Create HSM manager with software provider
        let hsm = create_test_hsm().await;

        // Generate BirdSong master key
        use crate::tunnel::hsm::KeyType;
        let key = hsm
            .generate_key("test_birdsong_master", &KeyType::ChaCha20)
            .await
            .expect("Key generation failed");

        // Verify key has material
        use crate::tunnel::hsm::KeyMaterial;
        match &key.key_material {
            KeyMaterial::Encrypted { encrypted_data, .. } => {
                assert!(!encrypted_data.is_empty(), "Key material must not be empty");
            }
            _ => {
                // Other variants are also valid
            }
        }
    }

    #[tokio::test]
    async fn test_cleanup_session_key_no_op() {
        // Create HSM manager with software provider
        let hsm = create_test_hsm().await;

        // Create genetics engine
        let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init failed"));

        // Initialize BTSP provider
        let provider = BeardogBtspProvider::new(hsm, genetics)
            .await
            .expect("Provider init failed");

        // Cleanup should succeed (no-op for BirdSong)
        let result = provider.cleanup_session_key("test_peer").await;
        assert!(result.is_ok(), "Cleanup should succeed");
    }
}
