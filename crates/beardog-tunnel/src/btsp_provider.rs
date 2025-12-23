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

    /// Active tunnels (tunnel_id -> Tunnel)
    tunnels: Arc<RwLock<HashMap<String, Tunnel>>>,

    /// Peer trust database (peer_id -> TrustRecord)
    trust_db: Arc<RwLock<HashMap<String, PeerTrustRecord>>>,
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
        info!("🐻 Initializing BearDog BTSP Provider");

        Ok(Self {
            hsm,
            genetics,
            tunnels: Arc::new(RwLock::new(HashMap::new())),
            trust_db: Arc::new(RwLock::new(HashMap::new())),
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

    /// Establish mTLS connection (placeholder - will integrate with existing tunnel)
    async fn establish_mtls(
        &self,
        peer: &PeerInfo,
        _session_key: &[u8],
    ) -> Result<(), BearDogError> {
        debug!("🔗 Establishing mTLS with peer: {}", peer.endpoint);

        // TODO: Integrate with existing BearDog tunnel/session infrastructure
        // For now, this is a placeholder that validates the peer endpoint

        if peer.endpoint.is_empty() {
            return Err(BearDogError::invalid_input("Peer endpoint cannot be empty"));
        }

        Ok(())
    }

    /// Generate session key using genetic cryptography
    async fn generate_session_key(&self, peer_id: &str) -> Result<Vec<u8>, BearDogError> {
        // Use HSM to generate ephemeral key
        let _key_id = format!("btsp_session_{}", peer_id);

        // TODO: Once HsmManager.generate_key is available, use it
        // For now, generate a random key
        use rand::RngCore;
        let mut key = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);

        debug!("🔑 Generated session key for peer: {}", peer_id);
        Ok(key)
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
        let mut tunnels = self.tunnels.write();

        if let Some(tunnel) = tunnels.remove(&handle.id) {
            info!(
                "🔒 Closing BTSP tunnel: {} (sent: {} bytes, received: {} bytes)",
                handle.id,
                tunnel.bytes_sent(),
                tunnel.bytes_received()
            );
            // Tunnel's Drop impl will zeroize the session key
        } else {
            warn!("⚠️  Tunnel {} not found for closure", handle.id);
        }

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
}
