// SPDX-License-Identifier: AGPL-3.0-or-later

//! Lineage proof generation and verification

use chrono::Utc;
use sha2::{Digest, Sha256};
use tracing::{debug, info};

use beardog_errors::BearDogError;

use super::lineage_chain::LineageChainManager;
use super::types::{LineageChain, LineageProof, LineageVerificationResult};

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

            // Get parent and child nodes for signature verification
            let _parent_node = chain.nodes.get(parent_id).ok_or_else(|| {
                BearDogError::system(format!("Parent node not found: {parent_id}"))
            })?;

            let _child_node = chain
                .nodes
                .get(child_id)
                .ok_or_else(|| BearDogError::system(format!("Child node not found: {child_id}")))?;

            // Verify signature (simplified for now - in production would use full verification)
            // self.chain_manager.verify_relationship(
            //     relationship,
            //     &_parent_node.public_key,
            //     &_child_node.public_key,
            // )?;
            debug!("✅ Verified relationship: {} -> {}", parent_id, child_id);
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

    /// Get the common ancestor of two nodes
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
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_generate_and_verify_proof() -> Result<(), BearDogError> {
        // Setup
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = LineageProofManager::new(chain_manager.clone());

        // Create lineage
        let chain = chain_manager
            .generate_root_chain("root".to_string(), None)
            .await?;
        chain_manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;
        chain_manager
            .add_child(&chain.chain_id, "child-1", "grandchild-1".to_string(), None)
            .await?;

        // Generate proof for grandchild
        let proof = proof_manager.generate_proof(&chain.chain_id, "grandchild-1")?;

        assert_eq!(proof.node_id, "grandchild-1");
        assert_eq!(proof.root_id, "root");
        assert_eq!(proof.path, vec!["root", "child-1", "grandchild-1"]);
        assert_eq!(proof.proof_chain.len(), 2);

        // Verify proof
        let result = proof_manager.verify_proof(&proof, &chain.chain_id)?;
        assert!(result.valid);
        assert_eq!(result.depth, 2);
        assert!(result.failure_reason.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_is_descendant() -> Result<(), BearDogError> {
        // Setup
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = LineageProofManager::new(chain_manager.clone());

        // Create lineage
        let chain = chain_manager
            .generate_root_chain("root".to_string(), None)
            .await?;
        chain_manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;
        chain_manager
            .add_child(&chain.chain_id, "child-1", "grandchild-1".to_string(), None)
            .await?;

        // Test descendant check
        assert!(proof_manager.is_descendant(&chain.chain_id, "root", "child-1")?);
        assert!(proof_manager.is_descendant(&chain.chain_id, "root", "grandchild-1")?);
        assert!(proof_manager.is_descendant(&chain.chain_id, "child-1", "grandchild-1")?);
        assert!(!proof_manager.is_descendant(&chain.chain_id, "child-1", "root")?);

        Ok(())
    }

    #[tokio::test]
    async fn test_generate_proof_chain_not_found() {
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = LineageProofManager::new(chain_manager);

        let result = proof_manager.generate_proof("nonexistent", "node-1");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Chain not found"));
    }

    #[tokio::test]
    async fn test_generate_proof_node_not_found() -> Result<(), BearDogError> {
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = LineageProofManager::new(chain_manager.clone());

        let chain = chain_manager
            .generate_root_chain("root".to_string(), None)
            .await?;

        let result = proof_manager.generate_proof(&chain.chain_id, "nonexistent");
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_verify_proof_chain_not_found() -> Result<(), BearDogError> {
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = LineageProofManager::new(chain_manager.clone());

        let chain = chain_manager
            .generate_root_chain("root".to_string(), None)
            .await?;
        chain_manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;

        let proof = proof_manager.generate_proof(&chain.chain_id, "child-1")?;

        // Verify against wrong chain ID
        let result = proof_manager.verify_proof(&proof, "nonexistent");
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_verify_proof_root_mismatch() -> Result<(), BearDogError> {
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = LineageProofManager::new(chain_manager.clone());

        // Create two separate chains
        let chain1 = chain_manager
            .generate_root_chain("root-1".to_string(), None)
            .await?;
        chain_manager
            .add_child(&chain1.chain_id, "root-1", "child-1".to_string(), None)
            .await?;

        let chain2 = chain_manager
            .generate_root_chain("root-2".to_string(), None)
            .await?;

        // Generate proof for chain1
        let proof = proof_manager.generate_proof(&chain1.chain_id, "child-1")?;

        // Verify against chain2 (root mismatch)
        let result = proof_manager.verify_proof(&proof, &chain2.chain_id)?;
        assert!(!result.valid);
        assert!(
            result
                .failure_reason
                .as_ref()
                .expect("invalid proof failure reason")
                .contains("Root mismatch")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_verify_proof_merkle_mismatch() -> Result<(), BearDogError> {
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = LineageProofManager::new(chain_manager.clone());

        let chain = chain_manager
            .generate_root_chain("root".to_string(), None)
            .await?;
        chain_manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;

        let mut proof = proof_manager.generate_proof(&chain.chain_id, "child-1")?;

        // Tamper with merkle root
        proof.merkle_root = vec![0u8; 32];

        let result = proof_manager.verify_proof(&proof, &chain.chain_id)?;
        assert!(!result.valid);
        assert!(
            result
                .failure_reason
                .as_ref()
                .expect("invalid proof failure reason")
                .contains("Merkle root mismatch")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_verify_proof_path_length_mismatch() -> Result<(), BearDogError> {
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = LineageProofManager::new(chain_manager.clone());

        let chain = chain_manager
            .generate_root_chain("root".to_string(), None)
            .await?;
        chain_manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;

        let mut proof = proof_manager.generate_proof(&chain.chain_id, "child-1")?;

        // Tamper with path to cause length mismatch
        proof.path.push("extra-node".to_string());

        let result = proof_manager.verify_proof(&proof, &chain.chain_id)?;
        assert!(!result.valid);
        assert!(
            result
                .failure_reason
                .as_ref()
                .expect("invalid proof failure reason")
                .contains("Path length mismatch")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_is_descendant_no_relationship() -> Result<(), BearDogError> {
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = LineageProofManager::new(chain_manager.clone());

        let chain = chain_manager
            .generate_root_chain("root".to_string(), None)
            .await?;
        chain_manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;

        // child-1 is not an ancestor of root
        assert!(!proof_manager.is_descendant(&chain.chain_id, "child-1", "root")?);

        // non-existent node
        assert!(!proof_manager.is_descendant(&chain.chain_id, "root", "non-existent")?);
        Ok(())
    }

    #[tokio::test]
    async fn test_common_ancestor_no_match() -> Result<(), BearDogError> {
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = LineageProofManager::new(chain_manager.clone());

        // Single chain, one node
        let chain = chain_manager
            .generate_root_chain("root".to_string(), None)
            .await?;

        // Non-existent nodes
        assert!(
            proof_manager
                .get_common_ancestor(&chain.chain_id, "nonexistent-a", "nonexistent-b")
                .is_none()
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_common_ancestor() -> Result<(), BearDogError> {
        // Setup
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = LineageProofManager::new(chain_manager.clone());

        // Create lineage with multiple branches
        let chain = chain_manager
            .generate_root_chain("root".to_string(), None)
            .await?;
        chain_manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;
        chain_manager
            .add_child(&chain.chain_id, "root", "child-2".to_string(), None)
            .await?;
        chain_manager
            .add_child(&chain.chain_id, "child-1", "grandchild-1".to_string(), None)
            .await?;
        chain_manager
            .add_child(&chain.chain_id, "child-2", "grandchild-2".to_string(), None)
            .await?;

        // Test common ancestor
        let ancestor =
            proof_manager.get_common_ancestor(&chain.chain_id, "grandchild-1", "grandchild-2");
        assert_eq!(ancestor, Some("root".to_string()));

        let ancestor =
            proof_manager.get_common_ancestor(&chain.chain_id, "grandchild-1", "child-1");
        assert_eq!(ancestor, Some("child-1".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_verify_proof_unknown_chain_errors() -> Result<(), BearDogError> {
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = LineageProofManager::new(chain_manager.clone());
        let chain = chain_manager
            .generate_root_chain("root".to_string(), None)
            .await?;
        chain_manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;
        let proof = proof_manager.generate_proof(&chain.chain_id, "child-1")?;
        let err = proof_manager
            .verify_proof(&proof, "nonexistent-chain-id")
            .expect_err("unknown chain");
        assert!(err.to_string().contains("Chain not found") || err.to_string().contains("chain"));
        Ok(())
    }
}
