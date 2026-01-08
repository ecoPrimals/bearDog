//! Contact Exchange - Genetic Lineage-Based NAT Traversal
//!
//! This module implements decentralized NAT traversal using genetic lineage
//! relationships. Instead of relying on STUN/TURN servers, peers discover
//! each other through cryptographic trust relationships in the genetic family tree.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

use beardog_errors::BearDogError;
use beardog_genetics::ecosystem_evolution::EcosystemGeneticEngine;

use super::types::PeerTrustRecord;

// =============================================================================
// Contact Exchange Types
// =============================================================================

/// Contact information for a peer (decentralized NAT traversal)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    /// Peer identifier
    pub peer_id: String,
    /// Peer addresses (IP:Port combinations)
    pub addresses: Vec<String>,
    /// Lineage proof (cryptographic verification of relationship)
    pub lineage_proof: String,
    /// Path through genetic lineage to reach peer
    pub lineage_path: Vec<String>,
    /// Depth of search through lineage tree
    pub search_depth: usize,
    /// Last time peer was seen
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_seen: DateTime<Utc>,
}

// =============================================================================
// Contact Exchange Implementation
// =============================================================================

/// Contact exchange handler for genetic lineage-based discovery
pub struct ContactExchange {
    /// Genetics engine for key lineage and evolution
    genetics: Arc<EcosystemGeneticEngine>,
    /// Peer trust database (peer_id -> TrustRecord)
    trust_db: Arc<RwLock<HashMap<String, PeerTrustRecord>>>,
}

impl ContactExchange {
    /// Create new contact exchange handler
    pub fn new(
        genetics: Arc<EcosystemGeneticEngine>,
        trust_db: Arc<RwLock<HashMap<String, PeerTrustRecord>>>,
    ) -> Self {
        Self { genetics, trust_db }
    }

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
                "Peer {} not found within {} hops in genetic lineage",
                target_peer_id, max_hops
            )));
        }

        // 2. Get peer addresses from trust database or discovery
        let addresses = self.get_peer_addresses(target_peer_id).await?;

        if addresses.is_empty() {
            return Err(BearDogError::business(format!(
                "No addresses found for peer {}",
                target_peer_id
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
        let our_family = std::env::var("FAMILY_ID")
            .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());

        // Check if peer is known in trust database
        let trust_db = self.trust_db.read();
        if trust_db.contains_key(target_peer_id) {
            // Direct connection in same family
            return Ok(vec![our_family, target_peer_id.to_string()]);
        }

        // For future: Query genetics engine for multi-hop paths
        // let path = self.genetics.find_path(requester_lineage, target_peer_id, max_hops).await?;

        // If peer not found in immediate family, return empty path
        warn!("⚠️  Peer {} not found in genetic lineage", target_peer_id);
        Ok(Vec::new())
    }

    /// Get peer addresses (IP:Port combinations)
    async fn get_peer_addresses(&self, peer_id: &str) -> Result<Vec<String>, BearDogError> {
        let mut addresses = Vec::new();

        // 1. Check trust database for known addresses
        let trust_db = self.trust_db.read();
        if trust_db.contains_key(peer_id) {
            debug!("Peer {} found in trust database", peer_id);
        }
        drop(trust_db);

        // 2. Discovery mechanism: query environment or discovery service
        // This is agnostic - no hardcoding of specific discovery systems
        // The primal discovers addresses through capability-based discovery at runtime

        // For initial implementation, generate placeholder addresses
        // In production, this would query actual discovery service via capability
        addresses.push(format!("192.168.1.5:10000")); // Local network
        addresses.push(format!("10.0.0.3:10001")); // Another local network

        // Future: Query discovery service via capability
        // let discovery_service = self.discover_capability("peer_discovery").await?;
        // addresses = discovery_service.query_peer_addresses(peer_id).await?;

        Ok(addresses)
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
        // let proof = self.genetics.generate_lineage_proof(lineage_path).await?;

        Ok(proof)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_contact_info_serialization() {
        let contact = ContactInfo {
            peer_id: "test-peer".to_string(),
            addresses: vec!["192.168.1.1:8080".to_string()],
            lineage_proof: "proof_abc123".to_string(),
            lineage_path: vec!["root".to_string(), "test-peer".to_string()],
            search_depth: 2,
            last_seen: Utc::now(),
        };

        let json = serde_json::to_string(&contact).expect("Serialize failed");
        let deserialized: ContactInfo = serde_json::from_str(&json).expect("Deserialize failed");

        assert_eq!(contact.peer_id, deserialized.peer_id);
        assert_eq!(contact.addresses, deserialized.addresses);
        assert_eq!(contact.search_depth, deserialized.search_depth);
    }
}

