// SPDX-License-Identifier: AGPL-3.0-or-later

//! Contact exchange via genetic lineage.

use super::{BeardogBtspProvider, ContactInfo};
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use chrono::Utc;
use tracing::{info, warn};

impl BeardogBtspProvider {
    /// Exchange contact information via genetic lineage.
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if the peer cannot be found within `max_hops` or contact exchange fails.
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

        let lineage_path = self
            .find_lineage_path(requester_lineage, target_peer_id, max_hops)
            .await?;

        if lineage_path.is_empty() {
            return Err(BearDogError::business(format!(
                "Peer {target_peer_id} not found within {max_hops} hops in genetic lineage"
            )));
        }

        let addresses = self.get_peer_addresses(target_peer_id).await?;

        if addresses.is_empty() {
            return Err(BearDogError::business(format!(
                "No addresses found for peer {target_peer_id}"
            )));
        }

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

    pub(crate) async fn find_lineage_path(
        &self,
        _requester_lineage: &str,
        target_peer_id: &str,
        _max_hops: usize,
    ) -> Result<Vec<String>, BearDogError> {
        let our_family = beardog_errors::process_env::var(env_keys::ENV_FAMILY_ID)
            .or_else(|_| beardog_errors::process_env::var(env_keys::ENV_FAMILY_ID_PREFIXED))
            .unwrap_or_else(|_| {
                beardog_errors::process_env::var(env_keys::ENV_FAMILY_UNKNOWN_LABEL)
                    .unwrap_or_else(|_| "unknown".to_string())
            });

        let trust_db = self.trust_db.read();
        if trust_db.contains_key(target_peer_id) {
            return Ok(vec![our_family, target_peer_id.to_string()]);
        }

        warn!("⚠️  Peer {} not found in genetic lineage", target_peer_id);
        Ok(Vec::new())
    }

    pub(crate) async fn generate_lineage_proof(
        &self,
        lineage_path: &[String],
    ) -> Result<String, BearDogError> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();

        for node in lineage_path {
            hasher.update(node.as_bytes());
        }

        let proof_hash = hasher.finalize();
        Ok(format!("lineage_proof_{}", hex::encode(proof_hash)))
    }
}
