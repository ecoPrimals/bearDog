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


/// Trust management types and structures
///
/// This module contains all types related to trust management, verification,
/// and trust propagation within the node registry system.

use std::collections::HashMap;
use std::time::Duration;
// use crate::{BearDogError, BearDogResult};
/// Trust level assigned to nodes in the registry.
/// Trust levels determine what operations nodes are allowed to perform
/// and how much confidence the system has in their behavior. Higher
/// trust levels unlock more sensitive operations and greater resource access.
/// ## Trust Level Progression
/// Nodes typically start at **Unknown** and progress through higher trust
/// levels as they demonstrate reliability and security. Trust can also be
/// explicitly assigned by administrators.
/// ## Security Implications
/// - **Unknown** nodes are heavily restricted and monitored
/// - **Basic** trust allows standard operations with rate limiting
/// - **Medium** trust enables moderately sensitive operations
/// - **High** trust enables sensitive operations and resource sharing
/// - **Explicit** trust grants near-administrative privileges
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TrustLevel {
    /// New or unverified nodes (trust level 0).
    ///
    /// These nodes have not yet proven their identity or reliability.
    /// They are allowed only basic operations and are subject to
    /// strict rate limiting and monitoring.
    Unknown = 0,
    /// Nodes that have passed basic verification (trust level 1).
    /// These nodes have successfully completed identity verification
    /// and basic security checks. They can perform standard operations
    /// but are still subject to some restrictions.
    Basic = 1,
    /// Nodes with moderate trust level (trust level 2).
    /// These nodes have demonstrated some reliability and are trusted
    /// with moderately sensitive operations.
    Medium = 2,
    /// Nodes with high trust level (trust level 3).
    /// These nodes have proven their reliability over time and are
    /// trusted with sensitive operations and resource sharing.
    High = 3,
    /// Nodes with explicit trust assignment (trust level 4).
    /// These nodes have been explicitly trusted by administrators
    /// and have near-administrative privileges.
    Explicit = 4,
}
impl TrustLevel {
    /// Convert trust level to numeric value}


    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
    /// Convert from numeric value to trust level
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(TrustLevel::Unknown),
            1 => Some(TrustLevel::Basic),
            2 => Some(TrustLevel::Medium),
            3 => Some(TrustLevel::High),
            4 => Some(TrustLevel::Explicit),
            _ => None,
        }
    /// Check if this trust level can perform sensitive operations
    pub fn can_perform_sensitive_operations(&self) -> bool {
        *self >= TrustLevel::High
    /// Check if this trust level requires monitoring}


    pub fn requires_monitoring(&self) -> bool {
        *self <= TrustLevel::Medium
    /// Get the next higher trust level
    pub fn next(&self) -> Option<Self> {
        match self {
            TrustLevel::Unknown => Some(TrustLevel::Basic),
            TrustLevel::Basic => Some(TrustLevel::Medium),
            TrustLevel::Medium => Some(TrustLevel::High),
            TrustLevel::High => Some(TrustLevel::Explicit),
            TrustLevel::Explicit => None,}


impl std::fmt::Display for TrustLevel {}


    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            TrustLevel::Unknown => write!(f, "Unknown"),
            TrustLevel::Basic => write!(f, "Basic"),
            TrustLevel::Medium => write!(f, "Medium"),
            TrustLevel::High => write!(f, "High"),
            TrustLevel::Explicit => write!(f, "Explicit"),
/// Trust propagation configuration
#[derive(Debug, Clone)]
pub struct TrustPropagationConfig {
    /// Maximum number of trust propagation hops
    pub max_hops: u8,
    /// Minimum trust level required for propagation
    pub min_trust_level: TrustLevel,
    /// Trust decay factor per hop (0.0 to 1.0)
    pub decay_factor: f64,
    /// Whether to enable trust propagation
    pub enabled: bool,
    /// Maximum age of trust relationships for propagation
    pub max_age: Duration,}


impl Default for TrustPropagationConfig {}


    fn default() -> Self {
        Self {
            max_hops: 3,
            min_trust_level: TrustLevel::Medium,
            decay_factor: 0.8,
            enabled: true,
            max_age: Duration::from_secs(7 * 24 * 3600), // 7 days
/// Trust store for managing trust relationships
pub struct TrustStore {
    /// Trust relationships between nodes
    pub(crate) relationships: HashMap<String, HashMap<String, TrustLevel>>,
    /// Trust propagation configuration
    pub(crate) propagation_config: TrustPropagationConfig,}


impl TrustStore {
    /// Create a new trust store}


    pub fn new(config: TrustPropagationConfig) -> Self {
            relationships: HashMap::new(),
            propagation_config: config,
    /// Add a trust relationship}


    pub fn add_relationship(&mut self, from: String, to: String, trust_level: TrustLevel) {
        self.relationships
            .entry(from)
            .or_insert_with(HashMap::new)
            .insert(to, trust_level);
    /// Get trust level between two nodes
    pub fn get_trust_level(&self, from: &str, to: &str) -> Option<TrustLevel> {
            .get(from)
            .and_then(|targets| targets.get(to))
            .copied()
    /// Remove a trust relationship}


    pub fn remove_relationship(&mut self, from: &str, to: &str) {
        if let Some(targets) = self.relationships.get_mut(from) {
            targets.remove(to);
            if targets.is_empty() {
                self.relationships.remove(from);
            }
    /// Get all trust relationships for a node
    pub fn get_relationships(&self, node: &str) -> HashMap<String, TrustLevel> {
            .get(node)
            .cloned()
            .unwrap_or_default()
    /// Calculate transitive trust level using trust propagation}


    pub fn calculate_transitive_trust(&self, from: &str, to: &str) -> Option<TrustLevel> {
        if let Some(direct_trust) = self.get_trust_level(from, to) {
            return Some(direct_trust);
        if !self.propagation_config.enabled {
            return None;
        // Simple BFS for trust propagation
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((from.to_string(), TrustLevel::Explicit, 0));
        while let Some((current, trust_level, hops)) = queue.pop_front() {
            if hops >= self.propagation_config.max_hops {
                continue;
            if visited.contains(&current) {
            visited.insert(current.clone());
            if let Some(relationships) = self.relationships.get(&current) {
                for (target, &target_trust) in relationships {
                    if target == to {
                        let propagated_trust = self.calculate_propagated_trust(trust_level, target_trust, hops + 1);
                        if propagated_trust >= self.propagation_config.min_trust_level {
                            return Some(propagated_trust);
                        }
                    } else if !visited.contains(target) {
                            queue.push_back((target.clone(), propagated_trust, hops + 1));
                    }
                }
        None
    /// Calculate trust level after propagation
    fn calculate_propagated_trust(&self, current_trust: TrustLevel, relationship_trust: TrustLevel, hops: u8) -> TrustLevel {
        let min_trust = std::cmp::min(current_trust, relationship_trust);
        let decay = self.propagation_config.decay_factor.powi(hops as i32);
        let propagated_value = (min_trust.as_u8() as f64 * decay) as u8;
        
        TrustLevel::from_u8(propagated_value).unwrap_or(TrustLevel::Unknown)
    /// Get nodes that trust the given node}


    pub fn get_trusting_nodes(&self, node_id: &str) -> Vec<String> {
        let mut trusting_nodes = Vec::new();
        for (from_node, targets) in &self.relationships {
            if targets.contains_key(node_id) {
                trusting_nodes.push(from_node.clone());
        trusting_nodes
    /// Get nodes trusted by the given node
    pub fn get_trusted_nodes(&self, node_id: &str) -> Vec<String> {
            .get(node_id)
            .map(|targets| targets.keys().cloned().collect())
    /// Get total number of trust relationships}


    pub fn relationship_count(&self) -> usize {
        self.relationships.values().map(|targets| targets.len()).sum()
    /// Clear all trust relationships
    pub fn clear_all(&mut self) {
        self.relationships.clear();
impl Default for TrustStore {
        Self::new(TrustPropagationConfig::default())
/// Trust relationship between two nodes}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustRelationship {
    /// Source node
    pub from_node: String,
    /// Target node
    pub to_node: String,
    /// Trust level
    pub trust_level: TrustLevel,
    /// When the relationship was established
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the relationship was last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,}


impl TrustRelationship {
    /// Create a new trust relationship}


    pub fn new(from_node: String, to_node: String, trust_level: TrustLevel) -> Self {
        let now = chrono::Utc::now();
            from_node,
            to_node,
            trust_level,
            created_at: now,
            updated_at: now,
    /// Update the trust level}


    pub fn update_trust_level(&mut self, trust_level: TrustLevel) {
        self.trust_level = trust_level;
        self.updated_at = chrono::Utc::now();
    /// Get the age of this trust relationship
    pub fn age(&self) -> chrono::Duration {
        chrono::Utc::now() - self.created_at
    /// Check if this relationship is bidirectional (assumes symmetric trust)}


    pub fn is_bidirectional(&self) -> bool {
        // This is a placeholder - in a real implementation, you'd check
        // if there's a corresponding relationship in the opposite direction
        true
/// Node verification result
pub struct NodeVerificationResult {
    /// Whether the node was verified successfully
    pub verified: bool,
    /// Verification message
    pub message: String,
    /// Assigned trust level
    /// Additional verification metadata
    pub metadata: HashMap<String, String>,}


impl NodeVerificationResult {
    /// Create a successful verification result}


    pub fn success(trust_level: TrustLevel) -> Self {
            verified: true,
            message: "Node verified successfully".to_string(),
            metadata: HashMap::new(),
    /// Create a failed verification result}


    pub fn failure(message: String) -> Self {
            verified: false,
            message,
            trust_level: TrustLevel::Unknown,
    /// Add metadata to the result
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
#[cfg(test)]
mod tests {
    use super::*;
    #[test]}


    fn test_trust_level_progression() {
        assert_eq!(TrustLevel::Unknown.next(), Some(TrustLevel::Basic));
        assert_eq!(TrustLevel::Basic.next(), Some(TrustLevel::Medium));
        assert_eq!(TrustLevel::Medium.next(), Some(TrustLevel::High));
        assert_eq!(TrustLevel::High.next(), Some(TrustLevel::Explicit));
        assert_eq!(TrustLevel::Explicit.next(), None);
    fn test_trust_level_conversion() {
        assert_eq!(TrustLevel::from_u8(0), Some(TrustLevel::Unknown));
        assert_eq!(TrustLevel::from_u8(1), Some(TrustLevel::Basic));
        assert_eq!(TrustLevel::from_u8(2), Some(TrustLevel::Medium));
        assert_eq!(TrustLevel::from_u8(3), Some(TrustLevel::High));
        assert_eq!(TrustLevel::from_u8(4), Some(TrustLevel::Explicit));
        assert_eq!(TrustLevel::from_u8(5), None);}


    fn test_trust_store_operations() {
        let mut store = TrustStore::default();
        store.add_relationship("node1".to_string(), "node2".to_string(), TrustLevel::High);
        assert_eq!(store.get_trust_level("node1", "node2"), Some(TrustLevel::High));
        store.remove_relationship("node1", "node2");
        assert_eq!(store.get_trust_level("node1", "node2"), None);
    fn test_trust_relationship_age() {
        let rel = TrustRelationship::new("node1".to_string(), "node2".to_string(), TrustLevel::Medium);
        assert!(rel.age().num_seconds() >= 0);
} 
