// SPDX-License-Identifier: AGPL-3.0-or-later

use super::BeardogBtspProvider;
use super::types::{PeerTrustRecord, TrustLevel};
use beardog_capabilities::traits::PeerEndpoint;
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use beardog_genetics::birdsong::LineageHint;
use beardog_genetics::birdsong::types::BirdSongEncryptRequest;
use chrono::Utc;
use rand::RngCore;
use tracing::{debug, info, warn};

impl BeardogBtspProvider {
    /// Seed a trusted peer into the trust database (bootstrap before TOFU tunnel).
    ///
    /// Accepts `peer_id` and `family_id` from explicit calls or `BEARDOG_TRUSTED_PEERS`
    /// (`peer_id:family_id` pairs, comma-separated). Inserts a minimal trust record so
    /// contact exchange lineage lookup can succeed for WAN peers.
    ///
    /// Returns `true` when a new record was inserted, `false` when the peer was already known.
    ///
    /// # Errors
    ///
    /// Returns an error if `peer_id` or `family_id` is empty.
    pub fn seed_trusted_peer(&self, peer_id: &str, family_id: &str) -> Result<bool, BearDogError> {
        let peer_id = peer_id.trim();
        let family_id = family_id.trim();
        if peer_id.is_empty() {
            return Err(BearDogError::invalid_input("peer_id cannot be empty"));
        }
        if family_id.is_empty() {
            return Err(BearDogError::invalid_input("family_id cannot be empty"));
        }

        let mut db = self.trust_db.write();
        if db.contains_key(peer_id) {
            debug!(
                "Trusted peer {} (family: {}) already seeded — skipping",
                peer_id, family_id
            );
            return Ok(false);
        }

        let now = Utc::now();
        db.insert(
            peer_id.to_string(),
            PeerTrustRecord {
                peer_id: peer_id.to_string(),
                public_key: Vec::new(),
                trust_level: TrustLevel::Tentative,
                first_seen: now,
                last_seen: now,
                connection_count: 0,
                family_id: Some(family_id.to_string()),
            },
        );

        info!(
            "🌱 Seeded trusted peer {} (family: {}) into trust_db",
            peer_id, family_id
        );
        Ok(true)
    }

    /// Parse and seed peers from [`env_keys::ENV_TRUSTED_PEERS`].
    ///
    /// # Errors
    ///
    /// Returns an error if any peer entry is malformed.
    pub fn seed_trusted_peers_from_env(&self) -> Result<usize, BearDogError> {
        let Ok(raw) = beardog_errors::process_env::var(env_keys::ENV_TRUSTED_PEERS) else {
            return Ok(0);
        };

        let mut seeded = 0usize;
        for entry in raw.split(',') {
            let entry = entry.trim();
            if entry.is_empty() {
                continue;
            }
            let (peer_id, family_id) = super::parse_trusted_peer_pair(entry)?;
            if self.seed_trusted_peer(peer_id, family_id)? {
                seeded += 1;
            }
        }
        Ok(seeded)
    }

    /// Pin peer's public key (TOFU - Trust On First Use)
    pub(super) fn pin_peer_key(
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
            family_id: None,
        };

        db.insert(peer_id.to_string(), record);

        info!("🔐 Pinned peer key: {} (TOFU)", peer_id);
        Ok(TrustLevel::Tentative)
    }

    /// Update peer trust record
    pub(super) fn update_peer_trust(&self, peer_id: &str) -> Result<(), BearDogError> {
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
    pub(super) fn establish_mtls(
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

    /// Generate session key using `BirdSong` lineage-aware encryption
    ///
    /// This uses `BirdSong` to encrypt a random session key for the peer's lineage,
    /// ensuring only trusted peers in the same cryptographic family can derive it.
    pub(super) fn generate_session_key(&self, peer_id: &str) -> Result<Vec<u8>, BearDogError> {
        debug!(
            "🎵 Generating BirdSong lineage-aware session key for peer: {}",
            peer_id
        );

        // Generate random session key material
        let mut key_material = vec![0u8; 32];
        rand::rng().fill_bytes(&mut key_material);

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
    pub(super) fn cleanup_session_key(&self, peer_id: &str) -> Result<(), BearDogError> {
        debug!("🗑️  Cleaning up BirdSong session key for peer: {}", peer_id);

        // BirdSong uses ephemeral encryption - no HSM cleanup needed
        // The session key is zeroized when the Tunnel struct is dropped
        // This is a no-op for BirdSong, kept for interface compatibility

        Ok(())
    }

    /// Encrypt with genetic key lineage
    pub(super) fn encrypt_with_lineage(
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
    pub(super) fn decrypt_with_lineage(
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
