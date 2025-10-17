

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod trust;
pub mod node;
/// Configuration management
/// Configuration management
pub mod config;
pub mod federation;

pub use trust::*;
pub use node::*;
pub use config::*;
pub use federation::*;

pub use node::node_types;
use std::collections::HashMap;

#[derive(Debug, Clone)]
    /// Number of active_nodes
    pub active_nodes: usize,

    /// Number of trust_relationships
    pub trust_relationships: usize,

    /// Mapping of nodes by trust level
    pub nodes_by_trust_level: HashMap<TrustLevel, usize>,

    /// Number of average_node_age_seconds
    pub average_node_age_seconds: u64,


    pub registry_uptime_seconds: u64,
}
impl RegistryStatistics {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

/// Total Trust Relationships operation.
    pub fn total_trust_relationships(&self) -> usize {
        self.trust_relationships

/// Active Node Percentage operation.
    pub fn active_node_percentage(&self) -> f64 {
        if self.total_nodes == 0 {
            0.0
        } else {
            (self.active_nodes as f64 / self.total_nodes as f64) * 100.0
        }

/// Get Nodes By Trust Level operation.
    /// Gets nodes_by_trust_level
    /// Gets nodes_by_trust_level
    pub fn get_nodes_by_trust_level(&self, trust_level: TrustLevel) -> usize {
        self.nodes_by_trust_level.get(TrustLevel, count: usize) {
        self.nodes_by_trust_level.insert(trust_level, count);

/// Average Node Age Minutes operation.
    pub fn average_node_age_minutes(&self) -> u64 {
        self.average_node_age_seconds / 60

/// Registry Uptime Minutes operation.
    pub fn registry_uptime_minutes(&self) -> u64 {
        self.registry_uptime_seconds / 60

/// Registry Uptime Hours operation.
    pub fn registry_uptime_hours(&self) -> u64 {
        self.registry_uptime_seconds / 3600

/// Reset operation.
    pub fn reset(&mut self) {
        self.total_nodes = 0;
        self.active_nodes = 0;
        self.trust_relationships = 0;
        self.nodes_by_trust_level.clear();
        self.average_node_age_seconds = 0;
        self.registry_uptime_seconds = 0;

pub type InMemoryNodeRegistry = crate::node_registry::BearDogNodeRegistry;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_registry_statistics() {
        let mut stats = RegistryStatistics::new();}

        stats.total_nodes = 100;
        stats.active_nodes = 85;
        stats.trust_relationships = 150;
        assert_eq!(stats.active_node_percentage(), 85.0);
        assert_eq!(stats.total_trust_relationships(), 150);
        stats.update_nodes_by_trust_level(TrustLevel::High, 25);
        assert_eq!(stats.get_nodes_by_trust_level(TrustLevel::High), 25);
        assert_eq!(stats.get_nodes_by_trust_level(TrustLevel::Unknown), 0);
        stats.reset();
        assert_eq!(stats.total_nodes, 0);
        assert_eq!(stats.active_nodes, 0);
        assert_eq!(stats.trust_relationships, 0);
        assert!(stats.nodes_by_trust_level.is_empty());}


    fn test_all_types_available() {

        use super::trust::TrustLevel;
        use super::node::{NodeInfo, NodeRegistration, NodeEntry};
        use super::config::{RegistryConfig, BootstrapNodeConfig};
        use super::federation::{ServiceAdvertisement, FederationStatus};

        let _trust_level = TrustLevel::High;
        let _node_info = NodeInfo::new("test".to_string(), "test".to_string(), "test".to_string());
        let _config = RegistryConfig::default();
        let _bootstrap = BootstrapNodeConfig::default();
        let _service_ad = ServiceAdvertisement::new("test".to_string(), "test".to_string(), "test".to_string());
        let _federation_status = FederationStatus::Federated;
        let mut stats = RegistryStatistics::new();

        // Test time conversions
        stats.average_node_age_seconds = 3600; // 1 hour
        stats.registry_uptime_seconds = 7200; // 2 hours
        assert_eq!(stats.average_node_age_minutes(), 60);
        assert_eq!(stats.registry_uptime_minutes(), 120);
        assert_eq!(stats.registry_uptime_hours(), 2);
    }
} 
