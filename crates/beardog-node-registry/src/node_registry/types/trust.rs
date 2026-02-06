//! # Trust Types for Node Registry
//!
//! This module provides trust-related types for managing trust relationships
//! between nodes in the registry.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::Duration;

// ============================================================
// Trust Level
// ============================================================

/// Trust level for nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TrustLevel {
    /// Unknown trust level
    Unknown = 0,

    /// Basic trust level
    Basic = 1,

    /// Medium trust level
    Medium = 2,

    /// High trust level
    High = 3,

    /// Explicit trust level (highest)
    Explicit = 4,
}

impl TrustLevel {
    /// Convert to u8
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    /// Create from u8
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(TrustLevel::Unknown),
            1 => Some(TrustLevel::Basic),
            2 => Some(TrustLevel::Medium),
            3 => Some(TrustLevel::High),
            4 => Some(TrustLevel::Explicit),
            _ => None,
        }
    }

    /// Check if can perform sensitive operations
    pub fn can_perform_sensitive_operations(&self) -> bool {
        *self >= TrustLevel::High
    }

    /// Check if requires monitoring
    pub fn requires_monitoring(&self) -> bool {
        *self <= TrustLevel::Medium
    }

    /// Get next trust level
    pub fn next(&self) -> Option<Self> {
        match self {
            TrustLevel::Unknown => Some(TrustLevel::Basic),
            TrustLevel::Basic => Some(TrustLevel::Medium),
            TrustLevel::Medium => Some(TrustLevel::High),
            TrustLevel::High => Some(TrustLevel::Explicit),
            TrustLevel::Explicit => None,
        }
    }

    /// Get previous trust level
    pub fn previous(&self) -> Option<Self> {
        match self {
            TrustLevel::Unknown => None,
            TrustLevel::Basic => Some(TrustLevel::Unknown),
            TrustLevel::Medium => Some(TrustLevel::Basic),
            TrustLevel::High => Some(TrustLevel::Medium),
            TrustLevel::Explicit => Some(TrustLevel::High),
        }
    }
}

impl Default for TrustLevel {
    fn default() -> Self {
        TrustLevel::Unknown
    }
}

impl std::fmt::Display for TrustLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrustLevel::Unknown => write!(f, "Unknown"),
            TrustLevel::Basic => write!(f, "Basic"),
            TrustLevel::Medium => write!(f, "Medium"),
            TrustLevel::High => write!(f, "High"),
            TrustLevel::Explicit => write!(f, "Explicit"),
        }
    }
}

// ============================================================
// Trust Propagation Configuration
// ============================================================

/// Configuration for trust propagation
#[derive(Debug, Clone)]
pub struct TrustPropagationConfig {
    /// Maximum hops for trust propagation
    pub max_hops: u8,

    /// Minimum trust level required
    pub min_trust_level: TrustLevel,

    /// Decay factor per hop (0.0 - 1.0)
    pub decay_factor: f64,

    /// Whether propagation is enabled
    pub enabled: bool,

    /// Maximum age for trust relationships
    pub max_age: Duration,
}

impl Default for TrustPropagationConfig {
    fn default() -> Self {
        Self {
            max_hops: 3,
            min_trust_level: TrustLevel::Medium,
            decay_factor: 0.8,
            enabled: true,
            max_age: Duration::from_secs(86400), // 24 hours
        }
    }
}

// ============================================================
// Trust Store
// ============================================================

/// Store for trust relationships
#[derive(Debug)]
pub struct TrustStore {
    /// Trust relationships: from_node -> (to_node -> trust_level)
    relationships: HashMap<String, HashMap<String, TrustLevel>>,

    /// Propagation configuration
    propagation_config: TrustPropagationConfig,
}

impl TrustStore {
    /// Create a new trust store
    pub fn new(config: TrustPropagationConfig) -> Self {
        Self {
            relationships: HashMap::with_capacity(64),
            propagation_config: config,
        }
    }

    /// Add a trust relationship
    pub fn add_relationship(&mut self, from: &str, to: &str, trust_level: TrustLevel) {
        self.relationships
            .entry(from.to_string())
            .or_insert_with(HashMap::new)
            .insert(to.to_string(), trust_level);
    }

    /// Get direct trust level
    pub fn get_trust_level(&self, from: &str, to: &str) -> Option<TrustLevel> {
        self.relationships
            .get(from)
            .and_then(|targets| targets.get(to))
            .copied()
    }

    /// Remove a trust relationship
    pub fn remove_relationship(&mut self, from: &str, to: &str) {
        if let Some(targets) = self.relationships.get_mut(from) {
            targets.remove(to);
            if targets.is_empty() {
                self.relationships.remove(from);
            }
        }
    }

    /// Get all relationships for a node
    pub fn get_relationships(&self, node: &str) -> HashMap<String, TrustLevel> {
        self.relationships
            .get(node)
            .cloned()
            .unwrap_or_default()
    }

    /// Get propagated trust level
    pub fn get_propagated_trust(&self, from: &str, to: &str) -> Option<TrustLevel> {
        // Check direct trust first
        if let Some(direct_trust) = self.get_trust_level(from, to) {
            return Some(direct_trust);
        }

        if !self.propagation_config.enabled {
            return None;
        }

        // BFS for propagated trust
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((from.to_string(), TrustLevel::Explicit, 0u8));

        while let Some((current, current_trust, hops)) = queue.pop_front() {
            if hops >= self.propagation_config.max_hops {
                continue;
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            if let Some(targets) = self.relationships.get(&current) {
                for (target, &target_trust) in targets {
                    if target == to {
                        let propagated = self.calculate_propagated_trust(
                            current_trust,
                            target_trust,
                            hops + 1,
                        );
                        if propagated >= self.propagation_config.min_trust_level {
                            return Some(propagated);
                        }
                    }

                    queue.push_back((target.clone(), target_trust, hops + 1));
                }
            }
        }

        None
    }

    /// Calculate propagated trust level
    fn calculate_propagated_trust(
        &self,
        current_trust: TrustLevel,
        relationship_trust: TrustLevel,
        hops: u8,
    ) -> TrustLevel {
        let min_trust = std::cmp::min(current_trust, relationship_trust);
        let decay = self.propagation_config.decay_factor.powi(hops as i32);
        let propagated_value = (min_trust.as_u8() as f64 * decay) as u8;

        TrustLevel::from_u8(propagated_value).unwrap_or(TrustLevel::Unknown)
    }

    /// Get nodes that trust the given node
    pub fn get_trusting_nodes(&self, node_id: &str) -> Vec<String> {
        let mut trusting_nodes = Vec::new();
        for (from_node, targets) in &self.relationships {
            if targets.contains_key(node_id) {
                trusting_nodes.push(from_node.clone());
            }
        }
        trusting_nodes
    }

    /// Get nodes trusted by the given node
    pub fn get_trusted_nodes(&self, node_id: &str) -> Vec<String> {
        self.relationships
            .get(node_id)
            .map(|targets| targets.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Get total relationship count
    pub fn relationship_count(&self) -> usize {
        self.relationships.values().map(|targets| targets.len()).sum()
    }

    /// Clear all relationships
    pub fn clear_all(&mut self) {
        self.relationships.clear();
    }
}

impl Default for TrustStore {
    fn default() -> Self {
        Self::new(TrustPropagationConfig::default())
    }
}

// ============================================================
// Trust Relationship
// ============================================================

/// A trust relationship between two nodes
#[derive(Debug, Clone)]
pub struct TrustRelationship {
    /// Source node
    pub from_node: String,

    /// Target node
    pub to_node: String,

    /// Trust level
    pub trust_level: TrustLevel,

    /// When the relationship was created
    pub created_at: DateTime<Utc>,

    /// When the relationship was last updated
    pub updated_at: DateTime<Utc>,
}

impl TrustRelationship {
    /// Create a new trust relationship
    pub fn new(from_node: &str, to_node: &str, trust_level: TrustLevel) -> Self {
        let now = Utc::now();
        Self {
            from_node: from_node.to_string(),
            to_node: to_node.to_string(),
            trust_level,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update trust level
    pub fn update_trust_level(&mut self, trust_level: TrustLevel) {
        self.trust_level = trust_level;
        self.updated_at = Utc::now();
    }

    /// Get relationship age
    pub fn age(&self) -> chrono::Duration {
        Utc::now() - self.created_at
    }
}

// ============================================================
// Node Verification Result
// ============================================================

/// Result of node verification
#[derive(Debug, Clone)]
pub struct NodeVerificationResult {
    /// Trust level assigned
    pub trust_level: TrustLevel,

    /// Whether verification succeeded
    pub verified: bool,

    /// Result message
    pub message: String,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl NodeVerificationResult {
    /// Create a success result
    pub fn success(trust_level: TrustLevel) -> Self {
        Self {
            trust_level,
            verified: true,
            message: "Node verified successfully".to_string(),
            metadata: HashMap::with_capacity(16),
        }
    }

    /// Create a failure result
    pub fn failure(message: &str) -> Self {
        Self {
            trust_level: TrustLevel::Unknown,
            verified: false,
            message: message.to_string(),
            metadata: HashMap::with_capacity(16),
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_level_progression() {
        assert_eq!(TrustLevel::Unknown.next(), Some(TrustLevel::Basic));
        assert_eq!(TrustLevel::Basic.next(), Some(TrustLevel::Medium));
        assert_eq!(TrustLevel::Medium.next(), Some(TrustLevel::High));
        assert_eq!(TrustLevel::High.next(), Some(TrustLevel::Explicit));
        assert_eq!(TrustLevel::Explicit.next(), None);
    }

    #[test]
    fn test_trust_level_conversion() {
        assert_eq!(TrustLevel::from_u8(0), Some(TrustLevel::Unknown));
        assert_eq!(TrustLevel::from_u8(1), Some(TrustLevel::Basic));
        assert_eq!(TrustLevel::from_u8(2), Some(TrustLevel::Medium));
        assert_eq!(TrustLevel::from_u8(3), Some(TrustLevel::High));
        assert_eq!(TrustLevel::from_u8(4), Some(TrustLevel::Explicit));
        assert_eq!(TrustLevel::from_u8(5), None);
    }

    #[test]
    fn test_trust_store_operations() {
        let mut store = TrustStore::default();
        store.add_relationship("node1", "node2", TrustLevel::High);
        assert_eq!(store.get_trust_level("node1", "node2"), Some(TrustLevel::High));
        store.remove_relationship("node1", "node2");
        assert_eq!(store.get_trust_level("node1", "node2"), None);
    }

    #[test]
    fn test_trust_relationship_age() {
        let rel = TrustRelationship::new("node1", "node2", TrustLevel::Medium);
        assert!(rel.age().num_seconds() >= 0);
    }

    #[test]
    fn test_trust_level_display() {
        assert_eq!(format!("{}", TrustLevel::High), "High");
        assert_eq!(format!("{}", TrustLevel::Unknown), "Unknown");
    }

    #[test]
    fn test_sensitive_operations() {
        assert!(!TrustLevel::Unknown.can_perform_sensitive_operations());
        assert!(!TrustLevel::Basic.can_perform_sensitive_operations());
        assert!(!TrustLevel::Medium.can_perform_sensitive_operations());
        assert!(TrustLevel::High.can_perform_sensitive_operations());
        assert!(TrustLevel::Explicit.can_perform_sensitive_operations());
    }

    #[test]
    fn test_verification_result() {
        let success = NodeVerificationResult::success(TrustLevel::High);
        assert!(success.verified);
        assert_eq!(success.trust_level, TrustLevel::High);

        let failure = NodeVerificationResult::failure("Verification failed");
        assert!(!failure.verified);
        assert_eq!(failure.trust_level, TrustLevel::Unknown);
    }
}
