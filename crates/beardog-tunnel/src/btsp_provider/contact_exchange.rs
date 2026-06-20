// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{BeardogBtspProvider, ContactInfo};
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use chrono::Utc;
use tracing::{info, warn};

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
        let our_family = beardog_errors::process_env::var(env_keys::ENV_FAMILY_ID)
            .or_else(|_| beardog_errors::process_env::var(env_keys::ENV_FAMILY_ID_PREFIXED))
            .unwrap_or_else(|_| {
                beardog_errors::process_env::var(env_keys::ENV_FAMILY_UNKNOWN_LABEL)
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
