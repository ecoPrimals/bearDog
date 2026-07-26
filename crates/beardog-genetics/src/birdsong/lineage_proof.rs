// SPDX-License-Identifier: AGPL-3.0-or-later

//! Lineage proof generation and verification

use chrono::Utc;
use sha2::{Digest, Sha256};
use tracing::{debug, info};

use beardog_errors::BearDogError;

use super::lineage_chain::LineageChainManager;
use super::types::{GeneticEnrollmentTier, LineageChain, LineageProof, LineageVerificationResult};

/// Manager for lineage proof operations
pub struct LineageProofManager {
    chain_manager: std::sync::Arc<LineageChainManager>,
}

impl LineageProofManager {
    /// Create new lineage proof manager
    ///
    /// # Arguments
    ///
    /// * `chain_manager` - Shared lineage chain manager
    pub fn new(chain_manager: std::sync::Arc<LineageChainManager>) -> Self {
        info!("🔐 Initializing LineageProofManager");
        Self { chain_manager }
    }

    /// Generate a lineage proof for a node
    ///
    /// # Arguments
    ///
    /// * `chain_id` - ID of the lineage chain
    /// * `node_id` - Node requesting the proof
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Chain not found
    /// - Node not found in chain
    /// - Path to root cannot be determined
    pub fn generate_proof(
        &self,
        chain_id: &str,
        node_id: &str,
    ) -> Result<LineageProof, BearDogError> {
        info!(
            "📜 Generating lineage proof for {} in chain {}",
            node_id, chain_id
        );

        // Get the chain
        let chain = self
            .chain_manager
            .get_chain(chain_id)
            .ok_or_else(|| BearDogError::system(format!("Chain not found: {chain_id}")))?;

        // Get path from root to this node
        let path = self
            .chain_manager
            .get_path_from_root(chain_id, node_id)
            .ok_or_else(|| {
                BearDogError::system(format!("Cannot determine path for node: {node_id}"))
            })?;

        // Collect the relationships along the path
        let mut proof_chain = Vec::new();
        for i in 0..(path.len() - 1) {
            let parent_id = &path[i];
            let child_id = &path[i + 1];

            // Find the relationship
            let relationship = chain
                .relationships
                .iter()
                .find(|r| r.parent_id == *parent_id && r.child_id == *child_id)
                .ok_or_else(|| {
                    BearDogError::system(format!(
                        "Relationship not found: {parent_id} -> {child_id}"
                    ))
                })?;

            proof_chain.push(relationship.clone());
        }

        // Calculate Merkle root for the entire chain
        let merkle_root = self.calculate_merkle_root(&chain);

        let proof = LineageProof {
            node_id: node_id.to_string(),
            root_id: chain.root_node.node_id,
            path: path.clone(),
            proof_chain,
            merkle_root,
            generation: chain.generation,
            head_commitment: chain.head_commitment,
            generated_at: Utc::now(),
        };

        debug!("✅ Generated proof with path: {:?}", path);
        Ok(proof)
    }

    /// Verify a lineage proof
    ///
    /// # Arguments
    ///
    /// * `proof` - The lineage proof to verify
    /// * `chain_id` - ID of the chain to verify against
    ///
    /// # Errors
    ///
    /// Returns error if chain not found
    pub fn verify_proof(
        &self,
        proof: &LineageProof,
        chain_id: &str,
    ) -> Result<LineageVerificationResult, BearDogError> {
        info!(
            "🔍 Verifying lineage proof for {} in chain {}",
            proof.node_id, chain_id
        );

        // Get the chain
        let chain = self
            .chain_manager
            .get_chain(chain_id)
            .ok_or_else(|| BearDogError::system(format!("Chain not found: {chain_id}")))?;

        // Verify root matches
        if proof.root_id != chain.root_node.node_id {
            return Ok(LineageVerificationResult {
                valid: false,
                depth: 0,
                failure_reason: Some(format!(
                    "Root mismatch: expected {}, got {}",
                    chain.root_node.node_id, proof.root_id
                )),
            });
        }

        // Verify path length matches proof chain length + 1
        if proof.path.len() != proof.proof_chain.len() + 1 {
            return Ok(LineageVerificationResult {
                valid: false,
                depth: 0,
                failure_reason: Some("Path length mismatch".to_string()),
            });
        }

        // Verify each relationship in the proof chain
        for (i, relationship) in proof.proof_chain.iter().enumerate() {
            let parent_id = &proof.path[i];
            let child_id = &proof.path[i + 1];

            // Check IDs match
            if relationship.parent_id != *parent_id || relationship.child_id != *child_id {
                return Ok(LineageVerificationResult {
                    valid: false,
                    depth: 0,
                    failure_reason: Some(format!(
                        "Relationship mismatch at step {}: expected {} -> {}, got {} -> {}",
                        i, parent_id, child_id, relationship.parent_id, relationship.child_id
                    )),
                });
            }

            let parent_node = chain.nodes.get(parent_id).ok_or_else(|| {
                BearDogError::system(format!("Parent node not found: {parent_id}"))
            })?;

            let child_node = chain
                .nodes
                .get(child_id)
                .ok_or_else(|| BearDogError::system(format!("Child node not found: {child_id}")))?;

            let sig_valid = self.chain_manager.verify_relationship(
                relationship,
                &parent_node.public_key,
                &child_node.public_key,
            )?;
            if !sig_valid {
                return Ok(LineageVerificationResult {
                    valid: false,
                    depth: 0,
                    failure_reason: Some(format!(
                        "Signature verification failed: {parent_id} -> {child_id}"
                    )),
                });
            }
            debug!("Verified relationship: {} -> {}", parent_id, child_id);
        }

        // Verify Merkle root
        let expected_merkle_root = self.calculate_merkle_root(&chain);
        if proof.merkle_root != expected_merkle_root {
            return Ok(LineageVerificationResult {
                valid: false,
                depth: 0,
                failure_reason: Some("Merkle root mismatch".to_string()),
            });
        }

        // Verify generational provenance (Phase 3)
        if !proof.head_commitment.is_empty() {
            if proof.generation > chain.generation {
                return Ok(LineageVerificationResult {
                    valid: false,
                    depth: 0,
                    failure_reason: Some(format!(
                        "Proof generation {} exceeds chain generation {}",
                        proof.generation, chain.generation,
                    )),
                });
            }
            if proof.generation == chain.generation
                && proof.head_commitment != chain.head_commitment
            {
                return Ok(LineageVerificationResult {
                    valid: false,
                    depth: 0,
                    failure_reason: Some(
                        "Head commitment mismatch at current generation".to_string(),
                    ),
                });
            }
        }

        // Proof is valid!
        #[expect(
            clippy::cast_possible_truncation,
            reason = "Merkle path depth fits u32 for verification result"
        )]
        let depth = (proof.path.len() - 1) as u32;
        info!(
            "✅ Lineage proof verified: {} (depth: {})",
            proof.node_id, depth
        );

        Ok(LineageVerificationResult {
            valid: true,
            depth,
            failure_reason: None,
        })
    }

    /// Calculate Merkle root for a lineage chain
    ///
    /// This provides a compact cryptographic commitment to the entire chain state.
    fn calculate_merkle_root(&self, chain: &LineageChain) -> Vec<u8> {
        let mut hasher = Sha256::new();

        // Hash chain ID
        hasher.update(chain.chain_id.as_bytes());

        // Hash root node
        hasher.update(chain.root_node.node_id.as_bytes());
        hasher.update(&chain.root_node.public_key);

        // Hash all relationships (deterministic order)
        let mut sorted_relationships = chain.relationships.clone();
        sorted_relationships
            .sort_by(|a, b| (&a.parent_id, &a.child_id).cmp(&(&b.parent_id, &b.child_id)));

        for rel in sorted_relationships {
            hasher.update(rel.parent_id.as_bytes());
            hasher.update(rel.child_id.as_bytes());
            hasher.update(&rel.parent_signature);
        }

        hasher.finalize().to_vec()
    }

    /// Check if a node is a descendant of another node
    ///
    /// # Errors
    ///
    /// Currently always returns `Ok`; the `Result` type is reserved for future lineage lookup failures.
    pub fn is_descendant(
        &self,
        chain_id: &str,
        ancestor_id: &str,
        descendant_id: &str,
    ) -> Result<bool, BearDogError> {
        let descendants = self.chain_manager.get_descendants(chain_id, ancestor_id);
        Ok(descendants.iter().any(|n| n.node_id == descendant_id))
    }

    /// Compute the genetic distance between two nodes in a lineage chain.
    ///
    /// Distance = `depth(A) + depth(B) − 2 × depth(common_ancestor(A, B))`.
    ///
    /// Returns `None` if either node is not in the chain or they share no
    /// common ancestor (different roots — impossible within a single chain,
    /// but defensive).
    ///
    /// # Examples
    ///
    /// - Parent → child: distance = 1
    /// - Siblings: distance = 2 (each is depth 1 from common parent)
    /// - Cousins: distance = 4 (each is depth 2 from common grandparent)
    /// - Self: distance = 0
    #[must_use]
    pub fn genetic_distance(
        &self,
        chain_id: &str,
        node_a_id: &str,
        node_b_id: &str,
    ) -> Option<u32> {
        if node_a_id == node_b_id {
            return Some(0);
        }

        let path_a = self.chain_manager.get_path_from_root(chain_id, node_a_id)?;
        let path_b = self.chain_manager.get_path_from_root(chain_id, node_b_id)?;

        // Find depth of lowest common ancestor (shared prefix length − 1)
        let common_depth = path_a
            .iter()
            .zip(path_b.iter())
            .take_while(|(a, b)| a == b)
            .count();

        if common_depth == 0 {
            return None;
        }

        #[expect(
            clippy::cast_possible_truncation,
            reason = "Lineage path lengths fit u32"
        )]
        let depth_a = (path_a.len() - 1) as u32;
        #[expect(
            clippy::cast_possible_truncation,
            reason = "Lineage path lengths fit u32"
        )]
        let depth_b = (path_b.len() - 1) as u32;
        #[expect(
            clippy::cast_possible_truncation,
            reason = "Common prefix length fits u32"
        )]
        let ancestor_depth = (common_depth - 1) as u32;

        Some(depth_a + depth_b - 2 * ancestor_depth)
    }

    /// Classify genetic distance into an enrollment trust tier.
    ///
    /// The two-layer model:
    /// - **Mitochondrial gate** (prerequisite): same `FAMILY_SEED` → can even
    ///   attempt enrollment. This is checked externally via HMAC.
    /// - **Nuclear distance** (this function): lineage tree proximity determines
    ///   the trust level granted upon enrollment.
    ///
    /// | Distance | Tier | Meaning |
    /// |----------|------|---------|
    /// | 0 | `Self` | Same node re-enrolling |
    /// | 1 | `Kin` | Direct parent or child |
    /// | 2 | `Sibling` | Siblings (same parent) |
    /// | 3–4 | `Extended` | Cousins, aunts/uncles |
    /// | 5+ | `Distant` | Far relatives — may require ceremony |
    #[must_use]
    pub const fn classify_enrollment_tier(distance: u32) -> GeneticEnrollmentTier {
        match distance {
            0 => GeneticEnrollmentTier::Identity,
            1 => GeneticEnrollmentTier::Kin,
            2 => GeneticEnrollmentTier::Sibling,
            3 | 4 => GeneticEnrollmentTier::Extended,
            _ => GeneticEnrollmentTier::Distant,
        }
    }

    /// Get the common ancestor of two nodes
    #[must_use]
    pub fn get_common_ancestor(
        &self,
        chain_id: &str,
        node_a_id: &str,
        node_b_id: &str,
    ) -> Option<String> {
        let path_a = self.chain_manager.get_path_from_root(chain_id, node_a_id)?;
        let path_b = self.chain_manager.get_path_from_root(chain_id, node_b_id)?;

        // Find the last common node in both paths
        let mut common_ancestor = None;
        for (a, b) in path_a.iter().zip(path_b.iter()) {
            if a == b {
                common_ancestor = Some(a.clone());
            } else {
                break;
            }
        }

        common_ancestor
    }
}

#[cfg(test)]
#[path = "lineage_proof_tests.rs"]
mod tests;
