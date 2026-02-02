//! BTSP Provider Core Implementation
//!
//! This module contains the core provider struct and initialization logic.
//! Manages HSM integration, trust tracking, and active tunnel state.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;
use tracing::{debug, info, warn};

use crate::tunnel::hsm::manager::HsmManager;
use beardog_errors::BearDogError;
use beardog_genetics::birdsong::{BirdSongManager, LineageHint};
use beardog_genetics::ecosystem_evolution::EcosystemGeneticEngine;

use super::tunnel_lifecycle::{Tunnel, TunnelLifecycleManager};
use super::types::{PeerTrustRecord, TrustLevel};
use super::metrics::BtspMetrics;

// =============================================================================
// Provider Core
// =============================================================================

/// BearDog's implementation of secure tunnel provider using genetic cryptography
///
/// This is the main provider struct that implements the SecureTunnelProvider trait.
/// It uses:
/// - Universal HSM architecture for cryptographic operations
/// - Genetic lineage for trust establishment
/// - TOFU (Trust On First Use) for peer verification
///
/// # Thread Safety
///
/// This struct is thread-safe via interior mutability (Arc<RwLock<T>>).
/// Send + Sync are auto-derived since all fields are Send + Sync.
#[derive(Clone)]
pub struct BeardogBtspProvider {
    inner: Arc<BtspProviderInner>,
}

/// Inner provider state (all fields are Send + Sync)
struct BtspProviderInner {
    /// HSM manager for cryptographic operations
    hsm_manager: Arc<HsmManager>,

    /// BirdSong manager for encrypted discovery
    birdsong_manager: Option<Arc<BirdSongManager>>,

    /// Ecosystem genetic engine for lineage verification
    genetic_engine: Option<Arc<EcosystemGeneticEngine>>,

    /// Active tunnels (tunnel_id -> Tunnel)
    tunnels: Arc<RwLock<HashMap<String, Tunnel>>>,

    /// Peer trust records (peer_id -> trust record)
    peer_trust: Arc<RwLock<HashMap<String, PeerTrustRecord>>>,

    /// Tunnel lifecycle manager
    lifecycle: TunnelLifecycleManager,

    /// Metrics
    metrics: Arc<BtspMetrics>,
}

impl BeardogBtspProvider {
    /// Create new BTSP provider with HSM manager
    ///
    /// # Arguments
    /// * `hsm_manager` - HSM manager for cryptographic operations
    ///
    /// # Returns
    /// New provider instance
    pub fn new(hsm_manager: Arc<HsmManager>) -> Self {
        info!("Initializing BearDog BTSP Provider");

        Self {
            inner: Arc::new(BtspProviderInner {
                hsm_manager,
                birdsong_manager: None,
                genetic_engine: None,
                tunnels: Arc::new(RwLock::new(HashMap::new())),
                peer_trust: Arc::new(RwLock::new(HashMap::new())),
                lifecycle: TunnelLifecycleManager::default(),
                metrics: Arc::new(BtspMetrics::new()),
            }),
        }
    }

    /// Set BirdSong manager for encrypted discovery
    ///
    /// This enables the provider to use BirdSong for peer discovery
    /// with genetic lineage encryption.
    pub fn with_birdsong(self, manager: Arc<BirdSongManager>) -> Self {
        debug!("Configuring BirdSong manager");
        // Clone the Arc to get mutable access
        let mut inner = (*self.inner).clone();
        inner.birdsong_manager = Some(manager);
        Self {
            inner: Arc::new(inner),
        }
    }

    /// Set genetic engine for lineage verification
    ///
    /// This enables genetic lineage-based trust decisions.
    pub fn with_genetic_engine(self, engine: Arc<EcosystemGeneticEngine>) -> Self {
        debug!("Configuring genetic engine");
        let mut inner = (*self.inner).clone();
        inner.genetic_engine = Some(engine);
        Self {
            inner: Arc::new(inner),
        }
    }

    /// Get reference to HSM manager
    pub(super) fn hsm(&self) -> &Arc<HsmManager> {
        &self.inner.hsm_manager
    }

    /// Get reference to BirdSong manager if configured
    pub(super) fn birdsong(&self) -> Option<&Arc<BirdSongManager>> {
        self.inner.birdsong_manager.as_ref()
    }

    /// Get reference to genetic engine if configured
    pub(super) fn genetic_engine(&self) -> Option<&Arc<EcosystemGeneticEngine>> {
        self.inner.genetic_engine.as_ref()
    }

    /// Get reference to lifecycle manager
    pub(super) fn lifecycle(&self) -> &TunnelLifecycleManager {
        &self.inner.lifecycle
    }

    /// Get reference to metrics
    pub(super) fn metrics(&self) -> &Arc<BtspMetrics> {
        &self.inner.metrics
    }

    /// Add tunnel to active tunnels
    pub(super) fn add_tunnel(&self, tunnel: Tunnel) -> String {
        let tunnel_id = tunnel.id.clone();
        let mut tunnels = self.inner.tunnels.write();
        tunnels.insert(tunnel_id.clone(), tunnel);
        debug!(tunnel_id = %tunnel_id, "Tunnel added");
        tunnel_id
    }

    /// Get tunnel by ID
    /// Get tunnel by ID
    ///
    /// Returns a clone of the tunnel if it exists.
    pub fn get_tunnel(&self, tunnel_id: &str) -> Option<Tunnel> {
        let tunnels = self.inner.tunnels.read();
        tunnels.get(tunnel_id).cloned()
    }

    /// Remove tunnel by ID
    pub(super) fn remove_tunnel(&self, tunnel_id: &str) -> Option<Tunnel> {
        let mut tunnels = self.inner.tunnels.write();
        let tunnel = tunnels.remove(tunnel_id);
        if tunnel.is_some() {
            debug!(tunnel_id = %tunnel_id, "Tunnel removed");
        }
        tunnel
    }

    /// Get all active tunnel IDs
    pub(super) fn active_tunnel_ids(&self) -> Vec<String> {
        let tunnels = self.inner.tunnels.read();
        tunnels.keys().cloned().collect()
    }

    /// Get trust record for peer
    /// Get peer trust record
    ///
    /// Returns a clone of the trust record if it exists.
    pub fn get_peer_trust(&self, peer_id: &str) -> Option<PeerTrustRecord> {
        let trust_records = self.inner.peer_trust.read();
        trust_records.get(peer_id).cloned()
    }

    /// Update or create trust record for peer
    pub(super) fn update_peer_trust(&self, peer_id: String, record: PeerTrustRecord) {
        let mut trust_records = self.inner.peer_trust.write();
        trust_records.insert(peer_id, record);
    }

    /// Check if peer is trusted
    pub(super) fn is_peer_trusted(&self, peer_id: &str) -> bool {
        if let Some(record) = self.get_peer_trust(peer_id) {
            record.trust_level == TrustLevel::Verified
                || record.trust_level == TrustLevel::Trusted
        } else {
            false
        }
    }

    /// Cleanup idle tunnels
    ///
    /// Removes tunnels that have been idle beyond the configured threshold.
    pub fn cleanup_idle_tunnels(&self) -> usize {
        let mut tunnels = self.inner.tunnels.write();
        let idle_tunnels: Vec<String> = tunnels
            .iter()
            .filter(|(_, tunnel)| self.inner.lifecycle.should_cleanup(tunnel))
            .map(|(id, _)| id.clone())
            .collect();

        let count = idle_tunnels.len();
        for tunnel_id in idle_tunnels {
            tunnels.remove(&tunnel_id);
            debug!(tunnel_id = %tunnel_id, "Idle tunnel cleaned up");
        }

        if count > 0 {
            info!(cleaned = count, "Idle tunnels cleaned up");
        }
        count
    }

    /// Get total number of active tunnels
    pub fn active_tunnel_count(&self) -> usize {
        let tunnels = self.inner.tunnels.read();
        tunnels.len()
    }

    /// Check if at maximum tunnel capacity
    pub(super) fn at_capacity(&self) -> bool {
        self.active_tunnel_count() >= self.inner.lifecycle.max_tunnels()
    }
}

// Thread-safe: All interior mutability is protected by RwLock

// Add Clone impl for BtspProviderInner (needed for builder pattern)
impl Clone for BtspProviderInner {
    fn clone(&self) -> Self {
        Self {
            hsm_manager: Arc::clone(&self.hsm_manager),
            birdsong_manager: self.birdsong_manager.clone(),
            genetic_engine: self.genetic_engine.clone(),
            tunnels: Arc::clone(&self.tunnels),
            peer_trust: Arc::clone(&self.peer_trust),
            lifecycle: self.lifecycle.clone(),
            metrics: Arc::clone(&self.metrics),
        }
    }
}

// ✅ ZERO UNSAFE CODE! Send + Sync auto-derived from Arc<T>
// All fields in BtspProviderInner are Send + Sync:
// - Arc<HsmManager>: Send + Sync
// - Option<Arc<BirdSongManager>>: Send + Sync
// - Option<Arc<EcosystemGeneticEngine>>: Send + Sync
// - Arc<RwLock<HashMap<...>>>: Send + Sync
// - TunnelLifecycleManager: Send + Sync
// - Arc<BtspMetrics>: Send + Sync

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_hsm() -> Arc<HsmManager> {
        // For tests, we create a basic HSM manager
        // In production, this would be properly initialized
        Arc::new(HsmManager::new_for_testing())
    }

    #[test]
    fn test_provider_creation() {
        let hsm = create_test_hsm();
        let provider = BeardogBtspProvider::new(hsm);
        
        assert_eq!(provider.active_tunnel_count(), 0);
        assert!(!provider.at_capacity());
    }

    #[test]
    fn test_tunnel_management() {
        let hsm = create_test_hsm();
        let provider = BeardogBtspProvider::new(hsm);
        
        let tunnel = provider.lifecycle().create_tunnel(
            "peer-123".to_string(),
            "endpoint".to_string(),
            vec![1, 2, 3, 4],
            TrustLevel::Trusted,
        ).unwrap();
        
        let tunnel_id = provider.add_tunnel(tunnel);
        assert_eq!(provider.active_tunnel_count(), 1);
        
        let retrieved = provider.get_tunnel(&tunnel_id);
        assert!(retrieved.is_some());
        
        let removed = provider.remove_tunnel(&tunnel_id);
        assert!(removed.is_some());
        assert_eq!(provider.active_tunnel_count(), 0);
    }

    #[test]
    fn test_peer_trust_tracking() {
        let hsm = create_test_hsm();
        let provider = BeardogBtspProvider::new(hsm);
        
        let record = PeerTrustRecord {
            peer_id: "peer-456".to_string(),
            trust_level: TrustLevel::Verified,
            first_seen: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            interactions: 1,
        };
        
        provider.update_peer_trust("peer-456".to_string(), record);
        
        assert!(provider.is_peer_trusted("peer-456"));
        assert!(!provider.is_peer_trusted("unknown-peer"));
    }
}

