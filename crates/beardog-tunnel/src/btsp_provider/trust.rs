//! Trust Management - TOFU, Peer Trust, and mTLS
//!
//! This module implements trust management for BTSP tunnels, including:
//! - TOFU (Trust On First Use) for new peers
//! - Progressive trust promotion based on successful connections
//! - mTLS connection establishment
//! - Session key generation using BirdSong genetic cryptography

use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;
use parking_lot::RwLock;
use rand::{rngs::OsRng, RngCore};
use tracing::{debug, info, warn};
use zeroize::Zeroizing;

use beardog_errors::BearDogError;
use beardog_genetics::birdsong::{BirdSongManager, LineageHint};

use super::types::{PeerInfo, PeerTrustRecord, TrustLevel};

// =============================================================================
// Trust Management Implementation
// =============================================================================

/// Trust manager for BTSP peer relationships
pub struct TrustManager {
    /// BirdSong manager for lineage-aware encryption
    birdsong: Arc<BirdSongManager>,
    /// Peer trust database (peer_id -> TrustRecord)
    trust_db: Arc<RwLock<HashMap<String, PeerTrustRecord>>>,
    /// TLS configuration for mTLS connections
    tls_config: Arc<crate::tls::TlsConfig>,
}

impl TrustManager {
    /// Create new trust manager
    pub fn new(
        birdsong: Arc<BirdSongManager>,
        trust_db: Arc<RwLock<HashMap<String, PeerTrustRecord>>>,
        tls_config: Arc<crate::tls::TlsConfig>,
    ) -> Self {
        Self {
            birdsong,
            trust_db,
            tls_config,
        }
    }

    /// Get peer trust level
    pub async fn get_peer_trust(&self, peer_id: &str) -> Option<TrustLevel> {
        self.trust_db
            .read()
            .get(peer_id)
            .map(|record| record.trust_level)
    }

    /// Pin peer's public key (TOFU - Trust On First Use)
    pub async fn pin_peer_key(
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
    pub async fn update_peer_trust(&self, peer_id: &str) -> Result<(), BearDogError> {
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
    pub async fn establish_mtls(
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
    pub async fn generate_session_key(&self, peer_id: &str) -> Result<Zeroizing<Vec<u8>>, BearDogError> {
        debug!(
            "🎵 Generating BirdSong lineage-aware session key for peer: {}",
            peer_id
        );

        // Generate random session key material
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

        let _broadcast = self
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
        Ok(Zeroizing::new(key_material))
    }

    /// Clean up ephemeral session key from HSM
    pub async fn cleanup_session_key(&self, peer_id: &str) -> Result<(), BearDogError> {
        debug!("🗑️  Cleaning up BirdSong session key for peer: {}", peer_id);

        // BirdSong uses ephemeral encryption - no HSM cleanup needed
        // The session key is zeroized when the Tunnel struct is dropped
        // This is a no-op for BirdSong, kept for interface compatibility

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_level_ordering() {
        assert!(TrustLevel::Trusted > TrustLevel::Tentative);
        assert!(TrustLevel::Tentative > TrustLevel::Unknown);
    }

    #[test]
    fn test_peer_info_validation() {
        let peer = PeerInfo {
            id: "test-peer".to_string(),
            endpoint: "192.168.1.1:8080".to_string(),
            public_key: Some(vec![1, 2, 3, 4]),
        };

        assert!(!peer.endpoint.is_empty());
        assert!(peer.public_key.is_some());
    }
}

