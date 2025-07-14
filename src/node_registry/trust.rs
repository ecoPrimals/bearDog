//! Trust Management Module
//!
//! This module handles trust relationships between nodes in the BearDog registry.
//! It supports both direct trust relationships and transitive trust through
//! the federation network.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::types::*;
use crate::{BearDogError, BearDogResult};

/// Trust manager for handling node trust relationships
pub struct TrustManager {
    /// Direct trust relationships
    trust_store: Arc<RwLock<TrustStore>>,
    
    /// Configuration for trust propagation
    config: TrustPropagationConfig,
}

impl TrustManager {
    /// Create a new trust manager
    pub fn new(config: TrustPropagationConfig) -> Self {
        Self {
            trust_store: Arc::new(RwLock::new(TrustStore::with_config(config.clone()))),
            config,
        }
    }
    
    /// Set trust level between two nodes
    pub async fn set_trust_relationship(
        &self,
        from_node: &str,
        to_node: &str,
        trust_level: TrustLevel,
    ) -> BearDogResult<()> {
        debug!("🤝 Setting trust relationship: {} -> {} = {:?}", from_node, to_node, trust_level);
        
        let mut trust_store = self.trust_store.write().await;
        trust_store.direct_trust.insert(
            (from_node.to_string(), to_node.to_string()),
            trust_level,
        );
        
        info!("✅ Trust relationship established: {} trusts {} at level {:?}", 
              from_node, to_node, trust_level);
        Ok(())
    }
    
    /// Get trust level between two nodes
    pub async fn get_trust_relationship(
        &self,
        from_node: &str,
        to_node: &str,
    ) -> BearDogResult<TrustLevel> {
        let trust_store = self.trust_store.read().await;
        
        // Check direct trust first
        if let Some(trust_level) = trust_store.direct_trust.get(&(from_node.to_string(), to_node.to_string())) {
            return Ok(*trust_level);
        }
        
        // Check transitive trust if enabled
        if self.config.enable_transitive_trust {
            if let Some(trust_level) = self.calculate_transitive_trust(from_node, to_node, &trust_store).await {
                return Ok(trust_level);
            }
        }
        
        // No trust relationship found
        Ok(TrustLevel::Unknown)
    }
    
    /// Set trust level for a node (used by registry)
    pub async fn set_trust_level(&self, node_id: &str, trust_level: TrustLevel) -> BearDogResult<()> {
        // This sets the node's general trust level
        // For now, we'll use a special "system" -> node_id relationship
        self.set_trust_relationship("system", node_id, trust_level).await
    }
    
    /// Get trust level for a node (used by registry)
    pub async fn get_trust_level(&self, node_id: &str) -> BearDogResult<TrustLevel> {
        self.get_trust_relationship("system", node_id).await
    }
    
    /// Check if two nodes trust each other
    pub async fn nodes_trust_each_other(&self, node1: &str, node2: &str) -> BearDogResult<bool> {
        let trust1 = self.get_trust_relationship(node1, node2).await?;
        let trust2 = self.get_trust_relationship(node2, node1).await?;
        
        // Both nodes should have at least basic trust
        Ok(trust1 >= TrustLevel::Basic && trust2 >= TrustLevel::Basic)
    }
    
    /// Get all nodes that trust a given node
    pub async fn get_trusting_nodes(&self, node_id: &str) -> Vec<String> {
        let trust_store = self.trust_store.read().await;
        trust_store.get_trusting_nodes(node_id)
    }
    
    /// Get all nodes that are trusted by a given node
    pub async fn get_trusted_nodes(&self, node_id: &str) -> Vec<String> {
        let trust_store = self.trust_store.read().await;
        trust_store.get_trusted_nodes(node_id)
    }
    
    /// Remove all trust relationships for a node
    pub async fn remove_node(&self, node_id: &str) -> BearDogResult<()> {
        debug!("🗑️ Removing trust relationships for node: {}", node_id);
        
        let mut trust_store = self.trust_store.write().await;
        
        // Remove relationships where this node is the source
        trust_store.direct_trust.retain(|(from, _), _| from != node_id);
        
        // Remove relationships where this node is the target
        trust_store.direct_trust.retain(|(_, to), _| to != node_id);
        
        info!("✅ Trust relationships removed for node: {}", node_id);
        Ok(())
    }
    
    /// Get trust relationship count
    pub async fn get_relationship_count(&self) -> usize {
        let trust_store = self.trust_store.read().await;
        trust_store.relationship_count()
    }
    
    /// Get all trust relationships
    pub async fn get_all_relationships(&self) -> Vec<TrustRelationship> {
        let trust_store = self.trust_store.read().await;
        let mut relationships = Vec::new();
        
        for ((from_node, to_node), trust_level) in trust_store.direct_trust.iter() {
            relationships.push(TrustRelationship::new(
                from_node.clone(),
                to_node.clone(),
                *trust_level,
            ));
        }
        
        relationships
    }
    
    /// Calculate trust metrics for a node
    pub async fn calculate_trust_metrics(&self, node_id: &str) -> TrustMetrics {
        let trust_store = self.trust_store.read().await;
        
        let mut metrics = TrustMetrics {
            node_id: node_id.to_string(),
            direct_trust_relationships: 0,
            incoming_trust_count: 0,
            outgoing_trust_count: 0,
            average_trust_level: 0.0,
            trust_reputation_score: 0.0,
        };
        
        let mut trust_levels = Vec::new();
        
        // Count relationships
        for ((from, to), trust_level) in trust_store.direct_trust.iter() {
            if from == node_id {
                metrics.outgoing_trust_count += 1;
            }
            if to == node_id {
                metrics.incoming_trust_count += 1;
                trust_levels.push(trust_level.as_numeric() as f64);
            }
        }
        
        metrics.direct_trust_relationships = metrics.incoming_trust_count + metrics.outgoing_trust_count;
        
        // Calculate average trust level
        if !trust_levels.is_empty() {
            metrics.average_trust_level = trust_levels.iter().sum::<f64>() / trust_levels.len() as f64;
        }
        
        // Calculate reputation score (more incoming high trust = better reputation)
        metrics.trust_reputation_score = metrics.average_trust_level * (metrics.incoming_trust_count as f64).ln().max(1.0);
        
        metrics
    }
    
    /// Validate trust relationship
    pub async fn validate_trust_relationship(
        &self,
        from_node: &str,
        to_node: &str,
        required_trust_level: TrustLevel,
    ) -> BearDogResult<bool> {
        let trust_level = self.get_trust_relationship(from_node, to_node).await?;
        Ok(trust_level >= required_trust_level)
    }
    
    /// Clear all trust relationships
    pub async fn clear_all_relationships(&self) -> BearDogResult<()> {
        warn!("🧹 Clearing all trust relationships");
        
        let mut trust_store = self.trust_store.write().await;
        trust_store.clear_all();
        
        info!("✅ All trust relationships cleared");
        Ok(())
    }
    
    // Private methods
    
    async fn calculate_transitive_trust(
        &self,
        from_node: &str,
        to_node: &str,
        trust_store: &TrustStore,
    ) -> Option<TrustLevel> {
        // Implementation of transitive trust calculation
        // This is a simplified version - a full implementation would use graph algorithms
        
        if !self.config.enable_transitive_trust {
            return None;
        }
        
        // Look for paths from from_node to to_node through intermediate nodes
        let mut visited = std::collections::HashSet::new();
        self.find_trust_path(from_node, to_node, &mut visited, 0, trust_store)
    }
    
    fn find_trust_path(
        &self,
        current_node: &str,
        target_node: &str,
        visited: &mut std::collections::HashSet<String>,
        depth: usize,
        trust_store: &TrustStore,
    ) -> Option<TrustLevel> {
        // Prevent infinite loops
        if visited.contains(current_node) || depth >= self.config.max_trust_hops {
            return None;
        }
        
        visited.insert(current_node.to_string());
        
        // Find all nodes that current_node trusts
        let mut min_trust = TrustLevel::Explicit;
        let mut found_path = false;
        
        for ((from, to), trust_level) in trust_store.direct_trust.iter() {
            if from == current_node && trust_level >= &self.config.min_propagation_trust {
                if to == target_node {
                    // Direct path found
                    let effective_trust = if self.config.trust_degradation_enabled && depth > 0 {
                        // Degrade trust with distance
                        match trust_level {
                            TrustLevel::Explicit => TrustLevel::High,
                            TrustLevel::High => TrustLevel::Medium,
                            TrustLevel::Medium => TrustLevel::Basic,
                            TrustLevel::Basic => TrustLevel::Unknown,
                            TrustLevel::Unknown => TrustLevel::Unknown,
                        }
                    } else {
                        *trust_level
                    };
                    
                    if effective_trust < min_trust {
                        min_trust = effective_trust;
                    }
                    found_path = true;
                } else {
                    // Recursive path search
                    if let Some(path_trust) = self.find_trust_path(to, target_node, visited, depth + 1, trust_store) {
                        let effective_trust = if self.config.trust_degradation_enabled {
                            // Take minimum of current trust and path trust
                            if *trust_level < path_trust {
                                *trust_level
                            } else {
                                path_trust
                            }
                        } else {
                            path_trust
                        };
                        
                        if effective_trust < min_trust {
                            min_trust = effective_trust;
                        }
                        found_path = true;
                    }
                }
            }
        }
        
        visited.remove(current_node);
        
        if found_path {
            Some(min_trust)
        } else {
            None
        }
    }
}

/// Trust metrics for a node
#[derive(Debug, Clone)]
pub struct TrustMetrics {
    pub node_id: String,
    pub direct_trust_relationships: usize,
    pub incoming_trust_count: usize,
    pub outgoing_trust_count: usize,
    pub average_trust_level: f64,
    pub trust_reputation_score: f64,
}

impl Default for TrustMetrics {
    fn default() -> Self {
        Self {
            node_id: String::new(),
            direct_trust_relationships: 0,
            incoming_trust_count: 0,
            outgoing_trust_count: 0,
            average_trust_level: 0.0,
            trust_reputation_score: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_direct_trust_relationship() {
        let config = TrustPropagationConfig::default();
        let trust_manager = TrustManager::new(config);
        
        // Set trust relationship
        trust_manager.set_trust_relationship("alice", "bob", TrustLevel::High).await.unwrap();
        
        // Verify trust relationship
        let trust_level = trust_manager.get_trust_relationship("alice", "bob").await.unwrap();
        assert_eq!(trust_level, TrustLevel::High);
        
        // Check reverse relationship (should be Unknown)
        let reverse_trust = trust_manager.get_trust_relationship("bob", "alice").await.unwrap();
        assert_eq!(reverse_trust, TrustLevel::Unknown);
    }
    
    #[tokio::test]
    async fn test_trust_metrics() {
        let config = TrustPropagationConfig::default();
        let trust_manager = TrustManager::new(config);
        
        // Set up trust relationships
        trust_manager.set_trust_relationship("alice", "bob", TrustLevel::High).await.unwrap();
        trust_manager.set_trust_relationship("charlie", "bob", TrustLevel::Medium).await.unwrap();
        trust_manager.set_trust_relationship("bob", "david", TrustLevel::Basic).await.unwrap();
        
        // Calculate metrics for Bob
        let metrics = trust_manager.calculate_trust_metrics("bob").await;
        
        assert_eq!(metrics.incoming_trust_count, 2); // Alice and Charlie trust Bob
        assert_eq!(metrics.outgoing_trust_count, 1); // Bob trusts David
        assert_eq!(metrics.direct_trust_relationships, 3);
        assert!(metrics.average_trust_level > 0.0);
    }
} 