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


/// Registry configuration types
///
/// This module contains the main registry configuration structure and its
/// associated builder methods and validation logic.

use std::collections::HashMap;
use std::time::Duration;
use crate::node_registry::types::trust::TrustPropagationConfig;
use super::{FederationConfig, PhonebookConfig, P2PConfig};
/// Registry configuration
/// 
/// The main configuration structure for the BearDog node registry.
/// This contains all the core settings needed to operate a registry instance,
/// including network settings, capacity limits, and trust requirements.
/// # Example
/// ```rust
/// use beardog::node_registry::types::config::RegistryConfig;
/// let config = RegistryConfig::new("my-registry".to_string(), "My Registry".to_string())
///     .with_port(9090)
///     .with_max_nodes(5000)
///     .with_metadata("region".to_string(), "us-east".to_string());
/// ```
#[derive(Debug, Clone)]
pub struct RegistryConfig {
    /// Registry identifier
    pub registry_id: String,
    /// Registry name
    pub registry_name: String,
    /// Registry version
    pub version: String,
    /// Registry bind address
    pub bind_address: String,
    /// Registry port
    pub port: u16,
    /// Maximum number of registered nodes
    pub max_nodes: usize,
    /// Node timeout in seconds
    pub node_timeout_seconds: u64,
    /// Health check interval in seconds
    pub health_check_interval_seconds: u64,
    /// Trust propagation configuration
    pub trust_propagation: TrustPropagationConfig,
    /// Registry metadata
    pub metadata: HashMap<String, String>,
    /// Minimum trust level required for node registration
    pub min_registration_trust: crate::node_registry::types::trust::TrustLevel,
    /// Enable federation
    pub enable_federation: bool,
    /// Federation settings
    pub federation: FederationConfig,
    /// Phonebook settings
    pub phonebook: PhonebookConfig,
    /// P2P settings
    pub p2p: P2PConfig,
}
impl Default for RegistryConfig {}


    fn default() -> Self {
        Self {
            registry_id: "beardog-registry".to_string(),
            registry_name: "BearDog Node Registry".to_string(),
            version: "1.0.0".to_string(),
            bind_address: std::env::var("BEARDOG_REGISTRY_BIND_ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("BEARDOG_REGISTRY_PORT")
                .unwrap_or_else(|_| "8443".to_string()) // Use 8443 for TLS by default
                .parse().unwrap_or(8443),
            max_nodes: 10000,
            node_timeout_seconds: 300, // 5 minutes
            health_check_interval_seconds: 60, // 1 minute
            trust_propagation: TrustPropagationConfig::default(),
            metadata: HashMap::new(),
            min_registration_trust: crate::node_registry::types::trust::TrustLevel::Basic,
            enable_federation: true,
            federation: FederationConfig::default(),
            phonebook: PhonebookConfig::default(),
            p2p: P2PConfig::default(),
        }
    }
impl RegistryConfig {
    /// Create a new registry configuration
    /// 
    /// # Arguments
    /// * `registry_id` - Unique identifier for the registry
    /// * `registry_name` - Human-readable name for the registry
    /// # Returns
    /// A new RegistryConfig with the specified ID and name, and default values
    /// for all other settings.}


    pub fn new(registry_id: String, registry_name: String) -> Self {
            registry_id,
            registry_name,
            ..Default::default()
    /// Set bind address
    /// * `address` - The IP address to bind the registry to}


    pub fn with_bind_address(mut self, address: String) -> Self {
        self.bind_address = address;
        self
    /// Set port
    /// * `port` - The port number to listen on
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
    /// Set maximum nodes
    /// * `max_nodes` - Maximum number of nodes that can register}


    pub fn with_max_nodes(mut self, max_nodes: usize) -> Self {
        self.max_nodes = max_nodes;
    /// Set node timeout
    /// * `timeout_seconds` - Timeout in seconds for node registration
    pub fn with_node_timeout(mut self, timeout_seconds: u64) -> Self {
        self.node_timeout_seconds = timeout_seconds;
    /// Set health check interval
    /// * `interval_seconds` - Health check interval in seconds}


    pub fn with_health_check_interval(mut self, interval_seconds: u64) -> Self {
        self.health_check_interval_seconds = interval_seconds;
    /// Set trust propagation configuration
    /// * `config` - Trust propagation configuration
    pub fn with_trust_propagation(mut self, config: TrustPropagationConfig) -> Self {
        self.trust_propagation = config;
    /// Add metadata
    /// * `key` - Metadata key
    /// * `value` - Metadata value}


    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
    /// Enable or disable federation
    /// * `enabled` - Whether federation should be enabled
    pub fn with_federation(mut self, enabled: bool) -> Self {
        self.enable_federation = enabled;
    /// Set federation configuration
    /// * `config` - Federation configuration}


    pub fn with_federation_config(mut self, config: FederationConfig) -> Self {
        self.federation = config;
    /// Set phonebook configuration
    /// * `config` - Phonebook configuration
    pub fn with_phonebook_config(mut self, config: PhonebookConfig) -> Self {
        self.phonebook = config;
    /// Set P2P configuration
    /// * `config` - P2P configuration}


    pub fn with_p2p_config(mut self, config: P2PConfig) -> Self {
        self.p2p = config;
    /// Get node timeout as Duration
    /// The node timeout as a Duration object
    pub fn node_timeout(&self) -> Duration {
        Duration::from_secs(self.node_timeout_seconds)
    /// Get health check interval as Duration
    /// The health check interval as a Duration object}


    pub fn health_check_interval(&self) -> Duration {
        Duration::from_secs(self.health_check_interval_seconds)
    /// Validate the configuration
    /// Checks that all required fields are set and that values are within
    /// acceptable ranges.
    /// Ok(()) if the configuration is valid, or an error describing what's wrong
    pub fn validate(&self) -> crate::BearDogResult<()> {
        if self.registry_id.is_empty() {
            return Err(crate::error::BearDogError::validation("registry_id", "Registry ID cannot be empty"));
        if self.registry_name.is_empty() {
            return Err(crate::error::BearDogError::validation("registry_name", "Registry name cannot be empty"));
        if self.port == 0 {
            return Err(crate::error::BearDogError::validation("port", "Port cannot be zero"));
        if self.max_nodes == 0 {
            return Err(crate::error::BearDogError::validation("max_nodes", "Max nodes cannot be zero"));
        if self.node_timeout_seconds == 0 {
            return Err(crate::error::BearDogError::validation("node_timeout_seconds", "Node timeout cannot be zero"));
        if self.health_check_interval_seconds == 0 {
            return Err(crate::error::BearDogError::validation("health_check_interval_seconds", "Health check interval cannot be zero"));
        Ok(())
#[cfg(test)]
mod tests {
    use super::*;
    #[test]}


    fn test_registry_config_default() {
        let config = RegistryConfig::default();
        assert_eq!(config.registry_id, "beardog-registry");
        assert_eq!(config.registry_name, "BearDog Node Registry");
        assert_eq!(config.port, 8080);
        assert_eq!(config.max_nodes, 10000);
        assert!(config.enable_federation);
        assert!(config.validate().is_ok());
    fn test_registry_config_builder() {
        let config = RegistryConfig::new("test-registry".to_string(), "Test Registry".to_string())
            .with_port(9090)
            .with_max_nodes(5000)
            .with_metadata("key".to_string(), "value".to_string());
        assert_eq!(config.registry_id, "test-registry");
        assert_eq!(config.registry_name, "Test Registry");
        assert_eq!(config.port, 9090);
        assert_eq!(config.max_nodes, 5000);
        assert_eq!(config.metadata.get("key"), Some(&"value".to_string()));}


    fn test_config_validation() {
        let mut config = RegistryConfig::default();
        config.registry_id = "".to_string();
        assert!(config.validate().is_err());
        config.registry_id = "valid-id".to_string();
        config.port = 0;
        config.port = 8080;
        config.max_nodes = 0;
    fn test_duration_helpers() {
        assert_eq!(config.node_timeout(), Duration::from_secs(300));
        assert_eq!(config.health_check_interval(), Duration::from_secs(60));
} 
