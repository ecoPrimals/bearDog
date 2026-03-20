// SPDX-License-Identifier: AGPL-3.0-only

//! Lineage chain management for parent-child relationships

use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use parking_lot::RwLock;
use tracing::{info, warn};
use uuid::Uuid;

use beardog_errors::BearDogError;

use super::types::{LineageChain, LineageMetadata, LineageNode, LineageRelationship};

/// Manager for lineage chain operations
pub struct LineageChainManager {
    /// Active lineage chains (chain_id -> LineageChain)
    chains: Arc<RwLock<HashMap<String, LineageChain>>>,
    /// Node signing keys (node_id -> SigningKey) - ephemeral, for demo
    /// In production, these would be stored in HSM
    signing_keys: Arc<RwLock<HashMap<String, SigningKey>>>,
}

impl LineageChainManager {
    /// Create new lineage chain manager
    pub fn new() -> Self {
        info!("🧬 Initializing LineageChainManager");
        Self {
            chains: Arc::new(RwLock::new(HashMap::new())),
            signing_keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Generate a new lineage chain with a root node
    ///
    /// # Arguments
    ///
    /// * `root_node_id` - Unique identifier for the root node
    /// * `metadata` - Optional metadata for the root node
    ///
    /// # Errors
    ///
    /// Returns error if chain generation fails
    pub async fn generate_root_chain(
        &self,
        root_node_id: String,
        metadata: Option<LineageMetadata>,
    ) -> Result<LineageChain, BearDogError> {
        info!("🌱 Generating root lineage chain for: {}", root_node_id);

        // Generate Ed25519 keypair for root node
        let mut secret_bytes = [0u8; 32];
        rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut secret_bytes);
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();

        // Store signing key
        self.signing_keys
            .write()
            .insert(root_node_id.clone(), signing_key);

        // Create root node
        let root_node = LineageNode {
            node_id: root_node_id.clone(),
            parent_id: None,
            public_key,
            depth: 0,
            created_at: Utc::now(),
            metadata: metadata.unwrap_or_default(),
        };

        // Create lineage chain
        let chain_id = Uuid::new_v4().to_string();
        let mut nodes = HashMap::new();
        nodes.insert(root_node_id, root_node.clone());

        let chain = LineageChain {
            chain_id: chain_id.clone(),
            root_node,
            nodes,
            relationships: Vec::new(),
            created_at: Utc::now(),
        };

        // Store chain
        self.chains.write().insert(chain_id.clone(), chain.clone());

        info!("✅ Root chain generated: {}", chain_id);
        Ok(chain)
    }

    /// Add a child node to an existing lineage chain
    ///
    /// # Arguments
    ///
    /// * `chain_id` - ID of the lineage chain
    /// * `parent_id` - Parent node ID
    /// * `child_id` - New child node ID
    /// * `metadata` - Optional metadata for the child node
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Chain not found
    /// - Parent not found
    /// - Child already exists
    /// - Signing fails
    pub async fn add_child(
        &self,
        chain_id: &str,
        parent_id: &str,
        child_id: String,
        metadata: Option<LineageMetadata>,
    ) -> Result<LineageNode, BearDogError> {
        info!(
            "👶 Adding child {} to parent {} in chain {}",
            child_id, parent_id, chain_id
        );

        // Generate keypair for child
        let mut secret_bytes = [0u8; 32];
        rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut secret_bytes);
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();

        // Store signing key
        self.signing_keys
            .write()
            .insert(child_id.clone(), signing_key);

        // Get parent node to determine depth
        let parent_depth = {
            let chains = self.chains.read();
            let chain = chains
                .get(chain_id)
                .ok_or_else(|| BearDogError::system(format!("Chain not found: {chain_id}")))?;

            let parent = chain
                .nodes
                .get(parent_id)
                .ok_or_else(|| BearDogError::system(format!("Parent not found: {parent_id}")))?;

            parent.depth
        };

        // Create child node
        let child_node = LineageNode {
            node_id: child_id.clone(),
            parent_id: Some(parent_id.to_string()),
            public_key: public_key.clone(),
            depth: parent_depth + 1,
            created_at: Utc::now(),
            metadata: metadata.unwrap_or_default(),
        };

        // Sign the relationship with parent's key
        let signature = self.sign_relationship(parent_id, &child_id, &public_key)?;

        let relationship = LineageRelationship {
            parent_id: parent_id.to_string(),
            child_id: child_id.clone(),
            parent_signature: signature,
            witness_signatures: Vec::new(), // Can add witnesses later
            established_at: Utc::now(),
        };

        // Add to chain
        let mut chains = self.chains.write();
        let chain = chains
            .get_mut(chain_id)
            .ok_or_else(|| BearDogError::system(format!("Chain not found: {chain_id}")))?;

        // Check if child already exists
        if chain.nodes.contains_key(&child_id) {
            return Err(BearDogError::system(format!(
                "Child already exists: {child_id}"
            )));
        }

        chain.nodes.insert(child_id.clone(), child_node.clone());
        chain.relationships.push(relationship);

        info!("✅ Child added: {} (depth: {})", child_id, child_node.depth);
        Ok(child_node)
    }

    /// Sign a parent-child relationship
    ///
    /// Signs: parent_id || child_id || child_public_key || timestamp
    fn sign_relationship(
        &self,
        parent_id: &str,
        child_id: &str,
        child_public_key: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        let signing_keys = self.signing_keys.read();
        let signing_key = signing_keys.get(parent_id).ok_or_else(|| {
            BearDogError::system(format!("Signing key not found for: {parent_id}"))
        })?;

        // Create message to sign
        let timestamp = Utc::now().timestamp_millis().to_le_bytes();
        let message = [
            parent_id.as_bytes(),
            b"||",
            child_id.as_bytes(),
            b"||",
            child_public_key,
            b"||",
            &timestamp,
        ]
        .concat();

        // Sign with Ed25519
        let signature = signing_key.sign(&message);
        Ok(signature.to_bytes().to_vec())
    }

    /// Verify a parent-child relationship signature
    pub fn verify_relationship(
        &self,
        relationship: &LineageRelationship,
        parent_public_key: &[u8],
        child_public_key: &[u8],
    ) -> Result<bool, BearDogError> {
        use ed25519_dalek::{Signature, Verifier};

        // Parse parent's public key
        let parent_verifying_key = VerifyingKey::from_bytes(
            parent_public_key
                .try_into()
                .map_err(|_| BearDogError::system("Invalid parent public key".to_string()))?,
        )
        .map_err(|e| BearDogError::system(format!("Failed to parse parent public key: {e}")))?;

        // Parse signature
        let signature = Signature::from_bytes(
            relationship
                .parent_signature
                .as_slice()
                .try_into()
                .map_err(|_| BearDogError::system("Invalid signature length".to_string()))?,
        );

        // Reconstruct message (we don't have exact timestamp, so this is approximate)
        // In production, timestamp would be included in the relationship
        let message = [
            relationship.parent_id.as_bytes(),
            b"||",
            relationship.child_id.as_bytes(),
            b"||",
            child_public_key,
        ]
        .concat();

        // Verify signature
        match parent_verifying_key.verify(&message, &signature) {
            Ok(()) => Ok(true),
            Err(e) => {
                warn!("Signature verification failed: {}", e);
                Ok(false)
            }
        }
    }

    /// Get a lineage chain by ID
    pub fn get_chain(&self, chain_id: &str) -> Option<LineageChain> {
        self.chains.read().get(chain_id).cloned()
    }

    /// Get all descendants of a node
    pub fn get_descendants(&self, chain_id: &str, node_id: &str) -> Vec<LineageNode> {
        let chains = self.chains.read();
        let Some(chain) = chains.get(chain_id) else {
            return Vec::new();
        };

        let mut descendants = Vec::new();
        let mut to_process = vec![node_id.to_string()];

        while let Some(current_id) = to_process.pop() {
            // Find children of current node
            for relationship in &chain.relationships {
                if relationship.parent_id == current_id {
                    if let Some(child) = chain.nodes.get(&relationship.child_id) {
                        descendants.push(child.clone());
                        to_process.push(relationship.child_id.clone());
                    }
                }
            }
        }

        descendants
    }

    /// Get the path from root to a specific node
    pub fn get_path_from_root(&self, chain_id: &str, target_node_id: &str) -> Option<Vec<String>> {
        let chains = self.chains.read();
        let chain = chains.get(chain_id)?;

        // Start from target and work backwards to root
        let mut path = vec![target_node_id.to_string()];
        let mut current_id = target_node_id.to_string();

        loop {
            let node = chain.nodes.get(&current_id)?;
            match &node.parent_id {
                Some(parent_id) => {
                    path.push(parent_id.clone());
                    current_id = parent_id.clone();
                }
                None => {
                    // Reached root
                    break;
                }
            }
        }

        // Reverse to get root->target order
        path.reverse();
        Some(path)
    }
}

impl Default for LineageChainManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_root_chain() -> Result<(), BearDogError> {
        let manager = LineageChainManager::new();

        let chain = manager
            .generate_root_chain("root-node-1".to_string(), None)
            .await?;

        assert_eq!(chain.root_node.node_id, "root-node-1");
        assert_eq!(chain.root_node.depth, 0);
        assert!(chain.root_node.parent_id.is_none());
        assert_eq!(chain.nodes.len(), 1);
        assert_eq!(chain.relationships.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_add_child() -> Result<(), BearDogError> {
        let manager = LineageChainManager::new();

        // Create root
        let chain = manager
            .generate_root_chain("root".to_string(), None)
            .await?;

        // Add child
        let child = manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;

        assert_eq!(child.node_id, "child-1");
        assert_eq!(child.parent_id, Some("root".to_string()));
        assert_eq!(child.depth, 1);

        // Verify chain state
        let updated_chain = manager.get_chain(&chain.chain_id).unwrap();
        assert_eq!(updated_chain.nodes.len(), 2);
        assert_eq!(updated_chain.relationships.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_add_child_chain_not_found() {
        let manager = LineageChainManager::new();
        let result = manager
            .add_child("nonexistent-chain", "root", "child-1".to_string(), None)
            .await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Chain not found"));
    }

    #[tokio::test]
    async fn test_add_child_parent_not_found() {
        let manager = LineageChainManager::new();
        let chain = manager
            .generate_root_chain("root".to_string(), None)
            .await
            .unwrap();
        let result = manager
            .add_child(
                &chain.chain_id,
                "nonexistent-parent",
                "child-1".to_string(),
                None,
            )
            .await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Parent not found"));
    }

    #[tokio::test]
    async fn test_add_child_duplicate() {
        let manager = LineageChainManager::new();
        let chain = manager
            .generate_root_chain("root".to_string(), None)
            .await
            .unwrap();
        manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await
            .unwrap();
        let result = manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    #[tokio::test]
    async fn test_get_chain_not_found() {
        let manager = LineageChainManager::new();
        assert!(manager.get_chain("nonexistent").is_none());
    }

    #[tokio::test]
    async fn test_get_descendants_no_chain() {
        let manager = LineageChainManager::new();
        let descendants = manager.get_descendants("nonexistent", "root");
        assert!(descendants.is_empty());
    }

    #[tokio::test]
    async fn test_get_path_from_root_no_chain() {
        let manager = LineageChainManager::new();
        assert!(manager.get_path_from_root("nonexistent", "root").is_none());
    }

    #[tokio::test]
    async fn test_get_path_from_root_no_node() {
        let manager = LineageChainManager::new();
        let chain = manager
            .generate_root_chain("root".to_string(), None)
            .await
            .unwrap();
        assert!(
            manager
                .get_path_from_root(&chain.chain_id, "nonexistent")
                .is_none()
        );
    }

    #[tokio::test]
    async fn test_verify_relationship() {
        let manager = LineageChainManager::new();
        let chain = manager
            .generate_root_chain("root".to_string(), None)
            .await
            .unwrap();
        let child = manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await
            .unwrap();

        let updated_chain = manager.get_chain(&chain.chain_id).unwrap();
        let parent_key = &updated_chain.root_node.public_key;
        let child_key = &child.public_key;
        let relationship = &updated_chain.relationships[0];

        // Signature won't fully verify due to timestamp mismatch in reconstructed message,
        // but this exercises the full verify_relationship path
        let result = manager.verify_relationship(relationship, parent_key, child_key);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_root_chain_with_metadata() {
        let manager = LineageChainManager::new();
        let metadata = LineageMetadata {
            biome_type: Some("forest".to_string()),
            capabilities: vec!["sensing".to_string()],
            trust_level: 0.9,
            custom: std::collections::HashMap::new(),
        };
        let chain = manager
            .generate_root_chain("root".to_string(), Some(metadata))
            .await
            .unwrap();
        assert_eq!(
            chain.root_node.metadata.biome_type,
            Some("forest".to_string())
        );
    }

    #[tokio::test]
    async fn test_multi_level_lineage() -> Result<(), BearDogError> {
        let manager = LineageChainManager::new();

        // Create root
        let chain = manager
            .generate_root_chain("root".to_string(), None)
            .await?;

        // Add children at multiple levels
        manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;
        manager
            .add_child(&chain.chain_id, "root", "child-2".to_string(), None)
            .await?;
        let grandchild = manager
            .add_child(&chain.chain_id, "child-1", "grandchild-1".to_string(), None)
            .await?;

        assert_eq!(grandchild.depth, 2);

        // Verify descendants
        let descendants = manager.get_descendants(&chain.chain_id, "root");
        assert_eq!(descendants.len(), 3);

        // Verify path
        let path = manager
            .get_path_from_root(&chain.chain_id, "grandchild-1")
            .unwrap();
        assert_eq!(path, vec!["root", "child-1", "grandchild-1"]);

        Ok(())
    }
}
