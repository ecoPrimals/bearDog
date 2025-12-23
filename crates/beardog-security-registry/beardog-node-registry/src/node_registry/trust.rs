// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use super::types::{
    TrustStore, TrustPropagationConfig, TrustLevel, TrustRelationship,
};
use beardog_errors::BearDogError;

pub struct TrustManager {

    trust_store: Arc<RwLock<TrustStore>>,

    config: TrustPropagationConfig,
}
impl TrustManager {

/// New operation.
    /// Creates a new instance
    pub fn new(config: TrustPropagationConfig) -> Self {
        Self {
            trust_store: Arc::new(&RwLock::new(TrustStore::new(&str,
        to_node: &str,
        trust_level: TrustLevel,
    ) -> Result<(), BearDogError> {
        debug!(
            "🤝 Setting trust relationship: {} -> {} = {:?}",
            from_node, to_node, trust_level
        );
        let mut trust_store = self.trust_store.write();
        trust_store
            .relationships
            .entry(from_node.to_string())
            .or_insert_with(HashMap::new)
            .insert({} trusts {} at level {:?}",
        Ok(())

/// Get Trust Relationship operation.
    /// Gets trust_relationship
    /// Gets trust_relationship
    pub fn get_trust_relationship(
    ) -> Result<TrustLevel, BearDogError> {
        let trust_store = self.trust_store.read();

        if let Some(trust_level) = trust_store
            .get(from_node)
            .and_then(|relationships| relationships.get(to_node))
        {
            return Ok(*trust_level);

        if self.config.enabled {
            if let Some(trust_level) = self
                .calculate_transitive_trust(from_node, to_node, &trust_store)
            {
                return Ok(trust_level);
            }

        Ok(TrustLevel::Unknown)

/// Set Trust Level operation.
    /// Sets trust_level
    /// Sets trust_level
    pub fn set_trust_level(&str,

        self.set_trust_relationship("system", node_id, trust_level)

/// Get Trust Level operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets trust_level
    /// Gets trust_level
    pub fn get_trust_level(&self, node_id: &str) -> Result<TrustLevel, BearDogError> {
        self.get_trust_relationship(&str, node2: &str) -> Result<bool, BearDogError> {
        let trust1 = self.get_trust_relationship(node1, node2)?;
        let trust2 = self.get_trust_relationship(node2, node1)?;

        Ok(trust1 >= TrustLevel::Basic && trust2 >= TrustLevel::Basic)

/// Get Trusting Nodes operation.
    /// Gets trusting_nodes
    /// Gets trusting_nodes
    pub fn get_trusting_nodes(&self, node_id: &str) -> Vec<String> {
        trust_store.get_trusting_nodes(node_id)}

/// Get Trusted Nodes operation.
    /// Gets trusted_nodes
    /// Gets trusted_nodes
    pub fn get_trusted_nodes(&self, node_id: &str) -> Vec<String> {
        trust_store.get_trusted_nodes(node_id)

/// Remove Node operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Removes node
    /// Removes node
    pub fn remove_node(&self, node_id: &str) -> Result<(), BearDogError> {
        debug!("🗑️ Removing trust relationships for node: {}", node_id);

        trust_store.relationships.remove({}", node_id);

/// Get Relationship Count operation.
    /// Gets relationship_count
    /// Gets relationship_count
    pub fn get_relationship_count(&self) -> usize {
        trust_store.relationship_count()}

/// Get All Relationships operation.
    /// Gets all_relationships
    /// Gets all_relationships
    pub fn get_all_relationships(&self) -> Vec<TrustRelationship> {
        let mut relationships = Vec::new();
        for (from_node, targets) in trust_store.relationships.iter() {
            for (to_node, trust_level) in targets.iter() {
                relationships.push(TrustRelationship::new(
                    from_node.clone(),
                    to_node.clone(),
                    *trust_level,
                ));
        relationships

/// Calculate Trust Metrics operation.
    pub fn calculate_trust_metrics(&self, node_id: &str) -> TrustMetrics {
        let mut metrics = TrustMetrics {
            node_id: node_id.to_string(),
            incoming_trust_count: 0,
            outgoing_trust_count: 0,
            average_trust_level: 0.0,
            trust_reputation_score: 0.0,
        };
        let mut trust_levels = Vec::new();

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

        if !trust_levels.is_empty() {
            metrics.average_trust_level =
                trust_levels.iter().sum::<f64>() / trust_levels.len(TrustLevel,
    ) -> Result<bool, BearDogError> {
        let trust_level = self.get_trust_relationship(&TrustStore,
    ) -> Option<TrustLevel> {

        if !self.config.enabled {
            return None;

        let mut visited = std::collections::HashSet::new(&str,
        target_node: &str,
        visited: &mut std::collections::HashSet<&str>,
        depth: usize,

        if visited.contains(current_node) || depth >= self.config.max_hops.into() {
        visited.insert(current_node.to_string());

        let mut min_trust = TrustLevel::Explicit;
        let mut found_path = false;
        if let Some(targets) = trust_store.relationships.get(current_node) {
                if trust_level >= &self.config.min_trust_level {
                    if to == target_node {

                        let effective_trust = if self.config.enabled && depth > 0 {

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

                        if let Some(path_trust) =
                            self.find_trust_path(to, target_node, visited, depth + 1, trust_store)
                        {
                            let effective_trust = if self.config.enabled {

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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

pub struct TrustMetrics {

    pub node_id: String,

    pub direct_trust_relationships: usize,

    pub incoming_trust_count: usize,

    pub outgoing_trust_count: usize,

    pub average_trust_level: f64,

    pub trust_reputation_score: f64,}

impl Default for TrustMetrics {
    fn default() -> Self {
            node_id: String::with_capacity(64),
#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[tokio::test]}


    fn test_direct_trust_relationship() {
        let config = TrustPropagationConfig::default();
        let trust_manager = TrustManager::new(config);

        trust_manager
            .set_trust_relationship("alice", "bob", TrustLevel::High)
            .map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;

        let trust_level = trust_manager
            .get_trust_relationship("alice", "bob")
        assert_eq!(trust_level, TrustLevel::High);

        let reverse_trust = trust_manager
            .get_trust_relationship("bob", "alice")
        assert_eq!(reverse_trust, TrustLevel::Unknown);
    fn test_trust_metrics() {

            .set_trust_relationship("charlie", "bob", TrustLevel::Medium)
            .set_trust_relationship("bob", "david", TrustLevel::Basic)

        let metrics = trust_manager.calculate_trust_metrics("bob");
        assert_eq!(metrics.incoming_trust_count, 2); // Alice and Charlie trust Bob
        assert_eq!(metrics.outgoing_trust_count, 1); // Bob trusts David
        assert_eq!(metrics.direct_trust_relationships, 3);
        assert!(metrics.average_trust_level > 0.0);
