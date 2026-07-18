// SPDX-License-Identifier: AGPL-3.0-or-later

//! BTSP (`BearDog` Tunnel Security Protocol) Provider Implementation
//!
//! **Note**: "BTSP" is used for developer context. The implementation uses generic
//! capability traits to maintain primal sovereignty.

mod contact;
mod contact_exchange;
mod metrics;
mod peer_discovery;
mod secure_tunnel_impl;
mod session_crypto;
mod trust;
mod tunnel;
pub mod types;

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use parking_lot::RwLock;
use tracing::{info, warn};

use crate::tunnel::hsm::manager::HsmManager;
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use beardog_genetics::birdsong::BirdSongManager;
use beardog_genetics::ecosystem_evolution::EcosystemGeneticEngine;

use crate::tunnel::hsm::{KeyMaterial, KeyType};
use tunnel::Tunnel;
use types::PeerTrustRecord;

pub use contact::ContactInfo;
pub use metrics::BtspMetrics;
pub use types::{Direction, PeerInfo, SecurityContext, TrustLevel};

pub use beardog_capabilities::traits::TunnelHandle;
pub use beardog_capabilities::traits::TunnelStatus;

/// `BearDog`'s implementation of BTSP using genetic cryptography
pub struct BeardogBtspProvider {
    pub(crate) _hsm: Arc<HsmManager>,
    pub(crate) _genetics: Arc<EcosystemGeneticEngine>,
    pub(crate) birdsong: Arc<BirdSongManager>,
    tunnels: Arc<RwLock<HashMap<String, Tunnel>>>,
    pub(crate) trust_db: Arc<RwLock<HashMap<String, PeerTrustRecord>>>,
    pub(crate) tunnels_established: Arc<AtomicU64>,
    pub(crate) encryption_count: Arc<AtomicU64>,
    pub(crate) decryption_count: Arc<AtomicU64>,
    pub(crate) trust_eval_count: Arc<AtomicU64>,
}

impl BeardogBtspProvider {
    /// Create a minimal provider for testing (bypasses HSM initialization)
    #[cfg(test)]
    /// # Errors
    ///
    /// Returns an error if key generation fails in the underlying HSM provider.
    pub async fn new_for_testing(
        hsm: Arc<HsmManager>,
        genetics: Arc<EcosystemGeneticEngine>,
    ) -> Result<Self, BearDogError> {
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

    /// Create new `BearDog` BTSP provider
    ///
    /// # Errors
    ///
    /// Returns error if initialization fails
    pub async fn new(
        hsm: Arc<HsmManager>,
        genetics: Arc<EcosystemGeneticEngine>,
    ) -> Result<Self, BearDogError> {
        info!("🐻 Initializing BearDog BTSP Provider with BirdSong genetics");

        let birdsong_key_label = beardog_config::domains::btsp::resolve_btsp_birdsong_key_label();
        let birdsong_key = hsm
            .generate_key(&birdsong_key_label, &KeyType::ChaCha20)
            .await
            .map_err(|e| {
                BearDogError::system(format!("Failed to generate BirdSong master key: {e}"))
            })?;

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

        let birdsong = BirdSongManager::new(master_secret, None)
            .await
            .map_err(|e| {
                BearDogError::system(format!("Failed to initialize BirdSong manager: {e}"))
            })?;

        info!("✅ BearDog BTSP Provider initialized with BirdSong genetics");

        let provider = Self {
            _hsm: hsm,
            _genetics: genetics,
            birdsong: Arc::new(birdsong),
            tunnels: Arc::new(RwLock::new(HashMap::new())),
            trust_db: Arc::new(RwLock::new(HashMap::new())),
            tunnels_established: Arc::new(AtomicU64::new(0)),
            encryption_count: Arc::new(AtomicU64::new(0)),
            decryption_count: Arc::new(AtomicU64::new(0)),
            trust_eval_count: Arc::new(AtomicU64::new(0)),
        };

        if let Err(e) = provider.seed_trusted_peers_from_env().await {
            warn!(
                "Failed to seed trusted peers from {}: {}",
                env_keys::ENV_TRUSTED_PEERS,
                e
            );
        }

        Ok(provider)
    }

    /// Get `BirdSong` manager (for API server integration)
    pub fn birdsong_manager(&self) -> Arc<BirdSongManager> {
        self.birdsong.clone()
    }

    /// Get BTSP metrics
    pub fn get_metrics(&self) -> BtspMetrics {
        BtspMetrics {
            tunnels_established: self.tunnels_established.load(Ordering::Relaxed),
            tunnels_active: self.tunnels.read().len() as u64,
            encryption_operations: self.encryption_count.load(Ordering::Relaxed),
            decryption_operations: self.decryption_count.load(Ordering::Relaxed),
            trust_evaluations: self.trust_eval_count.load(Ordering::Relaxed),
        }
    }

    /// Get tunnel by ID (public API for handlers)
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

    /// Get peer trust level (used by `SecureTunnelProvider`).
    pub(crate) async fn get_peer_trust(&self, peer_id: &str) -> Option<types::TrustLevel> {
        self.trust_db.read().get(peer_id).map(|r| r.trust_level)
    }
}

/// Parse `peer_id:family_id` bootstrap trust entries.
pub(crate) fn parse_trusted_peer_pair(entry: &str) -> Result<(&str, &str), BearDogError> {
    let (peer_id, family_id) = entry.split_once(':').ok_or_else(|| {
        BearDogError::invalid_input(&format!(
            "Invalid trusted peer entry '{entry}': expected peer_id:family_id"
        ))
    })?;
    let peer_id = peer_id.trim();
    let family_id = family_id.trim();
    if peer_id.is_empty() || family_id.is_empty() {
        return Err(BearDogError::invalid_input(&format!(
            "Invalid trusted peer entry '{entry}': peer_id and family_id must be non-empty"
        )));
    }
    Ok((peer_id, family_id))
}
