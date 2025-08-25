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


/// Node Registry Types and Data Structures
///
/// This module contains all the data structures, enums, and type definitions
/// used throughout the node registry and trust management system.
/// The module is organized into focused sub-modules:
/// - `trust`: Trust management and verification types
/// - `node`: Node information and registration types
/// - `config`: Configuration types for registry settings
/// - `federation`: Federation and service discovery types
/// - `statistics`: Registry statistics and monitoring types

pub mod trust;
pub mod node;
pub mod config;
pub mod federation;
// Core types - use canonical imports
pub use trust::*;
pub use node::*;
pub use config::*;
pub use federation::*;
// Re-export the node_types module specifically
pub use node::node_types;
use std::collections::HashMap;
/// Registry statistics
#[derive(Debug, Clone, Default)]
pub struct RegistryStatistics {
    /// Total number of registered nodes
    pub total_nodes: usize,
    /// Number of active nodes
    pub active_nodes: usize,
    /// Number of trust relationships
    pub trust_relationships: usize,
    /// Nodes by trust level
    pub nodes_by_trust_level: HashMap<TrustLevel, usize>,
    /// Average node age in seconds
    pub average_node_age_seconds: u64,
    /// Registry uptime in seconds
    pub registry_uptime_seconds: u64,
}
impl RegistryStatistics {
    /// Create new empty statistics}


    pub fn new() -> Self {
        Self::default()
    }
    /// Get total trust relationships
    pub fn total_trust_relationships(&self) -> usize {
        self.trust_relationships
    /// Get percentage of active nodes}


    pub fn active_node_percentage(&self) -> f64 {
        if self.total_nodes == 0 {
            0.0
        } else {
            (self.active_nodes as f64 / self.total_nodes as f64) * 100.0
        }
    /// Get nodes by trust level
    pub fn get_nodes_by_trust_level(&self, trust_level: TrustLevel) -> usize {
        self.nodes_by_trust_level.get(&trust_level).copied().unwrap_or(0)
    /// Update nodes by trust level}


    pub fn update_nodes_by_trust_level(&mut self, trust_level: TrustLevel, count: usize) {
        self.nodes_by_trust_level.insert(trust_level, count);
    /// Get average node age in minutes
    pub fn average_node_age_minutes(&self) -> u64 {
        self.average_node_age_seconds / 60
    /// Get registry uptime in minutes}


    pub fn registry_uptime_minutes(&self) -> u64 {
        self.registry_uptime_seconds / 60
    /// Get registry uptime in hours
    pub fn registry_uptime_hours(&self) -> u64 {
        self.registry_uptime_seconds / 3600
    /// Reset statistics}


    pub fn reset(&mut self) {
        self.total_nodes = 0;
        self.active_nodes = 0;
        self.trust_relationships = 0;
        self.nodes_by_trust_level.clear();
        self.average_node_age_seconds = 0;
        self.registry_uptime_seconds = 0;
/// Node identifier type
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
        // Test that all main types are available after refactoring
        use super::trust::TrustLevel;
        use super::node::{NodeInfo, NodeRegistration, NodeEntry};
        use super::config::{RegistryConfig, BootstrapNodeConfig};
        use super::federation::{ServiceAdvertisement, FederationStatus};
        // Create instances to verify all types are properly exported
        let _trust_level = TrustLevel::High;
        let _node_info = NodeInfo::new("test".to_string(), "test".to_string(), "test".to_string());
        let _config = RegistryConfig::default();
        let _bootstrap = BootstrapNodeConfig::default();
        let _service_ad = ServiceAdvertisement::new("test".to_string(), "test".to_string(), "test".to_string());
        let _federation_status = FederationStatus::Federated;
        let _stats = RegistryStatistics::new();
        // If we got here without compilation errors, all types are available
        assert!(true);
    fn test_time_conversions() {
        stats.average_node_age_seconds = 3600; // 1 hour
        stats.registry_uptime_seconds = 7200; // 2 hours
        assert_eq!(stats.average_node_age_minutes(), 60);
        assert_eq!(stats.registry_uptime_minutes(), 120);
        assert_eq!(stats.registry_uptime_hours(), 2);
} 
