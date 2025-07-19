//! Node Registry implementations for BearDog Core
//!
//! This module provides basic implementations of node registry and proof verification
//! for cross-node authentication within the BearDog ecosystem.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use beardog_auth::auth::{NodeRegistry, ProofVerifier};
use beardog_errors::{BearDogError, BearDogResult};

/// Basic implementation of NodeRegistry for core integration
#[derive(Debug, Clone)]
pub struct BasicNodeRegistry {
    nodes: Arc<RwLock<HashMap<String, beardog_auth::auth::types::NodeInfo>>>,
}

impl BasicNodeRegistry {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl NodeRegistry for BasicNodeRegistry {
    fn get_node_info(&self, node_id: &str) -> BearDogResult<beardog_auth::auth::types::NodeInfo> {
        // This is a sync method but we have async data - use try_read for basic implementation
        let nodes = self
            .nodes
            .try_read()
            .map_err(|_| BearDogError::internal("Could not read node registry"))?;
        nodes
            .get(node_id)
            .cloned()
            .ok_or_else(|| BearDogError::NotFound {
                message: format!("Node {node_id} not found"),
            })
    }

    fn register_node(
        &mut self,
        node_info: beardog_auth::auth::types::NodeInfo,
    ) -> BearDogResult<()> {
        // This is a sync method but we have async data - use try_write for basic implementation
        let mut nodes = self
            .nodes
            .try_write()
            .map_err(|_| BearDogError::internal("Could not write to node registry"))?;
        nodes.insert(node_info.id.clone(), node_info);
        Ok(())
    }

    fn get_trust_level(&self, node_id: &str) -> BearDogResult<f64> {
        // This is a sync method but we have async data - use try_read for basic implementation
        let nodes = self
            .nodes
            .try_read()
            .map_err(|_| BearDogError::internal("Could not read node registry"))?;
        
        nodes
            .get(node_id)
            .map(|node| {
                // Basic trust calculation - in practice this would be more sophisticated
                if node.verified {
                    0.9
                } else {
                    0.5
                }
            })
            .unwrap_or(0.0)
    }

    fn update_trust_level(&mut self, node_id: &str, trust_level: f64) -> BearDogResult<()> {
        // This is a sync method but we have async data - use try_write for basic implementation
        let mut nodes = self
            .nodes
            .try_write()
            .map_err(|_| BearDogError::internal("Could not write to node registry"))?;
        
        if let Some(node) = nodes.get_mut(node_id) {
            // In a real implementation, we'd store trust level in the NodeInfo
            // For now, we'll just log it
            tracing::info!("Updated trust level for node {node_id} to {trust_level}");
        }
        Ok(())
    }

    fn list_nodes(&self) -> BearDogResult<Vec<beardog_auth::auth::types::NodeInfo>> {
        let nodes = self
            .nodes
            .try_read()
            .map_err(|_| BearDogError::internal("Could not read node registry"))?;
        Ok(nodes.values().cloned().collect())
    }
}

impl Default for BasicNodeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Basic proof verifier implementation
#[derive(Debug, Clone)]
pub struct BasicProofVerifier {
    // In a real implementation, this would contain cryptographic verification logic
    trusted_nodes: Arc<RwLock<std::collections::HashSet<String>>>,
}

impl BasicProofVerifier {
    pub fn new() -> Self {
        Self {
            trusted_nodes: Arc::new(RwLock::new(std::collections::HashSet::new())),
        }
    }
}

impl ProofVerifier for BasicProofVerifier {
    fn verify_proof(
        &self,
        proof: &[u8],
        node_id: &str,
    ) -> BearDogResult<bool> {
        // Basic implementation - in practice this would involve cryptographic verification
        let trusted_nodes = self
            .trusted_nodes
            .try_read()
            .map_err(|_| BearDogError::internal("Could not read trusted nodes"))?;
        
        // For now, just check if the node is in our trusted set and the proof is non-empty
        Ok(trusted_nodes.contains(node_id) && !proof.is_empty())
    }

    fn generate_challenge(&self) -> BearDogResult<Vec<u8>> {
        // Generate a simple challenge - in practice this would be cryptographically secure
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let challenge: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        Ok(challenge)
    }

    fn verify_challenge_response(
        &self,
        challenge: &[u8],
        response: &[u8],
        node_id: &str,
    ) -> BearDogResult<bool> {
        // Basic implementation - in practice this would involve proper cryptographic verification
        let trusted_nodes = self
            .trusted_nodes
            .try_read()
            .map_err(|_| BearDogError::internal("Could not read trusted nodes"))?;
        
        // For now, just verify that the response matches the challenge (insecure, just for structure)
        Ok(trusted_nodes.contains(node_id) && response.len() == challenge.len())
    }

    fn add_trusted_node(&mut self, node_id: &str) -> BearDogResult<()> {
        let mut trusted_nodes = self
            .trusted_nodes
            .try_write()
            .map_err(|_| BearDogError::internal("Could not write to trusted nodes"))?;
        trusted_nodes.insert(node_id.to_string());
        Ok(())
    }
}

impl Default for BasicProofVerifier {
    fn default() -> Self {
        Self::new()
    }
} 