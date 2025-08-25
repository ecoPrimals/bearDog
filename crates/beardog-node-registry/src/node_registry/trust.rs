// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Trust Management Module
///
/// This module handles trust relationships between nodes in the BearDog registry.
/// It supports both direct trust relationships and transitive trust through
/// the federation network.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use super::types::{
    TrustStore, TrustPropagationConfig, TrustLevel, TrustRelationship,
};
use crate::BearDogResult;
/// Trust manager for handling node trust relationships
pub struct TrustManager {
    /// Direct trust relationships
    trust_store: Arc<RwLock<TrustStore>>,
    /// Configuration for trust propagation
    config: TrustPropagationConfig,
}
impl TrustManager {
    /// Create a new trust manager}


    pub fn new(config: TrustPropagationConfig) -> Self {
        Self {
            trust_store: Arc::new(RwLock::new(TrustStore::new(config.clone()))),
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
        debug!(
            "🤝 Setting trust relationship: {} -> {} = {:?}",
            from_node, to_node, trust_level
        );
        let mut trust_store = self.trust_store.write().await;
        trust_store
            .relationships
            .entry(from_node.to_string())
            .or_insert_with(HashMap::new)
            .insert(to_node.to_string(), trust_level);
        info!(
            "✅ Trust relationship established: {} trusts {} at level {:?}",
        Ok(())
    /// Get trust level between two nodes
    pub async fn get_trust_relationship(
    ) -> BearDogResult<TrustLevel> {
        let trust_store = self.trust_store.read().await;
        // Check direct trust first
        if let Some(trust_level) = trust_store
            .get(from_node)
            .and_then(|relationships| relationships.get(to_node))
        {
            return Ok(*trust_level);
        // Check transitive trust if enabled
        if self.config.enabled {
            if let Some(trust_level) = self
                .calculate_transitive_trust(from_node, to_node, &trust_store)
                .await
            {
                return Ok(trust_level);
            }
        // No trust relationship found
        Ok(TrustLevel::Unknown)
    /// Set trust level for a node (used by registry)
    pub async fn set_trust_level(
        node_id: &str,
        // This sets the node's general trust level
        // For now, we'll use a special "system" -> node_id relationship
        self.set_trust_relationship("system", node_id, trust_level)
            .await
    /// Get trust level for a node (used by registry)
    pub async fn get_trust_level(&self, node_id: &str) -> BearDogResult<TrustLevel> {
        self.get_trust_relationship("system", node_id).await
    /// Check if two nodes trust each other}


    pub async fn nodes_trust_each_other(&self, node1: &str, node2: &str) -> BearDogResult<bool> {
        let trust1 = self.get_trust_relationship(node1, node2).await?;
        let trust2 = self.get_trust_relationship(node2, node1).await?;
        // Both nodes should have at least basic trust
        Ok(trust1 >= TrustLevel::Basic && trust2 >= TrustLevel::Basic)
    /// Get all nodes that trust a given node
    pub async fn get_trusting_nodes(&self, node_id: &str) -> Vec<String> {
        trust_store.get_trusting_nodes(node_id)
    /// Get all nodes that are trusted by a given node}


    pub async fn get_trusted_nodes(&self, node_id: &str) -> Vec<String> {
        trust_store.get_trusted_nodes(node_id)
    /// Remove all trust relationships for a node
    pub async fn remove_node(&self, node_id: &str) -> BearDogResult<()> {
        debug!("🗑️ Removing trust relationships for node: {}", node_id);
        // Remove relationships where this node is the source
        trust_store.relationships.remove(node_id);
        // Remove relationships where this node is the target
        // Remove all relationships where this node is the target
        for relationships in trust_store.relationships.values_mut() {
            relationships.retain(|to, _| to != node_id);
        // Remove any empty relationship maps
        trust_store.relationships.retain(|_, relationships| !relationships.is_empty());
        info!("✅ Trust relationships removed for node: {}", node_id);
    /// Get trust relationship count
    pub async fn get_relationship_count(&self) -> usize {
        trust_store.relationship_count()
    /// Get all trust relationships}


    pub async fn get_all_relationships(&self) -> Vec<TrustRelationship> {
        let mut relationships = Vec::new();
        for (from_node, targets) in trust_store.relationships.iter() {
            for (to_node, trust_level) in targets.iter() {
                relationships.push(TrustRelationship::new(
                    from_node.clone(),
                    to_node.clone(),
                    *trust_level,
                ));
        relationships
    /// Calculate trust metrics for a node
    pub async fn calculate_trust_metrics(&self, node_id: &str) -> TrustMetrics {
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
        for (from, targets) in trust_store.relationships.iter() {
            for (to, trust_level) in targets.iter() {
                if from == node_id {
                    metrics.outgoing_trust_count += 1;
                }
                if to == node_id {
                    metrics.incoming_trust_count += 1;
                    trust_levels.push(trust_level.as_u8() as f64);
        metrics.direct_trust_relationships =
            metrics.incoming_trust_count + metrics.outgoing_trust_count;
        // Calculate average trust level
        if !trust_levels.is_empty() {
            metrics.average_trust_level =
                trust_levels.iter().sum::<f64>() / trust_levels.len() as f64;
        // Calculate reputation score (more incoming high trust = better reputation)
        metrics.trust_reputation_score =
            metrics.average_trust_level * (metrics.incoming_trust_count as f64).ln().max(1.0);
        metrics
    /// Validate trust relationship
    pub async fn validate_trust_relationship(
        required_trust_level: TrustLevel,
    ) -> BearDogResult<bool> {
        let trust_level = self.get_trust_relationship(from_node, to_node).await?;
        Ok(trust_level >= required_trust_level)
    /// Clear all trust relationships}


    pub async fn clear_all_relationships(&self) -> BearDogResult<()> {
        warn!("🧹 Clearing all trust relationships");
        trust_store.clear_all();
        info!("✅ All trust relationships cleared");
    // Private methods
    async fn calculate_transitive_trust(
        trust_store: &TrustStore,
    ) -> Option<TrustLevel> {
        // Implementation of transitive trust calculation
        // This is a simplified version - a full implementation would use graph algorithms
        if !self.config.enabled {
            return None;
        // Look for paths from from_node to to_node through intermediate nodes
        let mut visited = std::collections::HashSet::new();
        self.find_trust_path(from_node, to_node, &mut visited, 0, trust_store)}


    fn find_trust_path(
        current_node: &str,
        target_node: &str,
        visited: &mut std::collections::HashSet<String>,
        depth: usize,
        // Prevent infinite loops
        if visited.contains(current_node) || depth >= self.config.max_hops.into() {
        visited.insert(current_node.to_string());
        // Find all nodes that current_node trusts
        let mut min_trust = TrustLevel::Explicit;
        let mut found_path = false;
        if let Some(targets) = trust_store.relationships.get(current_node) {
                if trust_level >= &self.config.min_trust_level {
                    if to == target_node {
                        // Direct path found
                        let effective_trust = if self.config.enabled && depth > 0 {
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
                        if let Some(path_trust) =
                            self.find_trust_path(to, target_node, visited, depth + 1, trust_store)
                        {
                            let effective_trust = if self.config.enabled {
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
                            found_path = true;
                    }
        visited.remove(current_node);
        if found_path {
            Some(min_trust)
        } else {
            None
/// Trust metrics for a node in the network
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrustMetrics {
    /// Unique identifier for the node
    pub node_id: String,
    /// Number of direct trust relationships this node has
    pub direct_trust_relationships: usize,
    /// Number of nodes that trust this node
    pub incoming_trust_count: usize,
    /// Number of nodes this node trusts
    pub outgoing_trust_count: usize,
    /// Average trust level for this node (0.0 to 1.0)
    pub average_trust_level: f64,
    /// Overall trust reputation score in the network
    pub trust_reputation_score: f64,}


impl Default for TrustMetrics {}


    fn default() -> Self {
            node_id: String::new(),
#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    #[tokio::test]}


    async fn test_direct_trust_relationship() {
        let config = TrustPropagationConfig::default();
        let trust_manager = TrustManager::new(config);
        // Set trust relationship
        trust_manager
            .set_trust_relationship("alice", "bob", TrustLevel::High)
            .map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        // Verify trust relationship
        let trust_level = trust_manager
            .get_trust_relationship("alice", "bob")
        assert_eq!(trust_level, TrustLevel::High);
        // Check reverse relationship (should be Unknown)
        let reverse_trust = trust_manager
            .get_trust_relationship("bob", "alice")
        assert_eq!(reverse_trust, TrustLevel::Unknown);
    async fn test_trust_metrics() {
        // Set up trust relationships
            .set_trust_relationship("charlie", "bob", TrustLevel::Medium)
            .set_trust_relationship("bob", "david", TrustLevel::Basic)
        // Calculate metrics for Bob
        let metrics = trust_manager.calculate_trust_metrics("bob").await;
        assert_eq!(metrics.incoming_trust_count, 2); // Alice and Charlie trust Bob
        assert_eq!(metrics.outgoing_trust_count, 1); // Bob trusts David
        assert_eq!(metrics.direct_trust_relationships, 3);
        assert!(metrics.average_trust_level > 0.0);
