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


/// Phonebook configuration types
///
/// This module contains configuration types for the phonebook service, which
/// provides node discovery and lookup capabilities within the registry.

use std::collections::HashMap;
use std::time::Duration;
/// Phonebook configuration
/// 
/// Configuration for the phonebook service, which provides node discovery and
/// lookup capabilities. The phonebook maintains a cache of known nodes and
/// their capabilities, enabling efficient node discovery.
/// # Example
/// ```rust
/// use beardog::node_registry::types::config::PhonebookConfig;
/// let config = PhonebookConfig::new()
///     .with_cache_size(5000)
///     .with_cache_ttl(600)
///     .with_default_region("us-east".to_string())
///     .with_max_tracked_nodes(10000);
/// ```
#[derive(Debug, Clone)]
pub struct PhonebookConfig {
    /// Enable phonebook service
    pub enabled: bool,
    /// Phonebook bind address
    pub bind_address: String,
    /// Phonebook port
    pub port: u16,
    /// Phonebook cache size
    pub cache_size: usize,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
    /// Refresh interval in seconds
    pub refresh_interval_seconds: u64,
    /// Maximum number of tracked nodes
    pub max_tracked_nodes: usize,
    /// Node entry TTL in seconds
    pub node_entry_ttl: Duration,
    /// Default regions to serve
    pub default_regions: Vec<String>,
    /// Phonebook metadata
    pub metadata: HashMap<String, String>,
}
impl Default for PhonebookConfig {}


    fn default() -> Self {
        Self {
            enabled: true,
            bind_address: "0.0.0.0".to_string(),
            port: 8844,
            cache_size: 10000,
            cache_ttl_seconds: 300, // 5 minutes
            refresh_interval_seconds: 60, // 1 minute
            max_tracked_nodes: 10000,
            node_entry_ttl: Duration::from_secs(300), // 5 minutes
            default_regions: vec!["default".to_string()],
            metadata: HashMap::new(),
        }
    }
impl PhonebookConfig {
    /// Create a new phonebook configuration
    /// 
    /// # Returns
    /// A new PhonebookConfig with default values}


    pub fn new() -> Self {
        Self::default()
    /// Enable or disable phonebook service
    /// # Arguments
    /// * `enabled` - Whether the phonebook service should be enabled}


    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    /// Set cache size
    /// * `size` - Maximum number of entries in the cache
    pub fn with_cache_size(mut self, size: usize) -> Self {
        self.cache_size = size;
    /// Set cache TTL
    /// * `ttl_seconds` - Cache entry time-to-live in seconds}


    pub fn with_cache_ttl(mut self, ttl_seconds: u64) -> Self {
        self.cache_ttl_seconds = ttl_seconds;
    /// Set refresh interval
    /// * `interval_seconds` - Cache refresh interval in seconds
    pub fn with_refresh_interval(mut self, interval_seconds: u64) -> Self {
        self.refresh_interval_seconds = interval_seconds;
    /// Set bind address
    /// * `bind_address` - The IP address to bind the phonebook service to}


    pub fn with_bind_address(mut self, bind_address: String) -> Self {
        self.bind_address = bind_address;
    /// Set port
    /// * `port` - The port number for the phonebook service
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
    /// Set max tracked nodes
    /// * `max_tracked_nodes` - Maximum number of nodes to track}


    pub fn with_max_tracked_nodes(mut self, max_tracked_nodes: usize) -> Self {
        self.max_tracked_nodes = max_tracked_nodes;
    /// Set node entry TTL
    /// * `ttl_seconds` - Node entry time-to-live in seconds
    pub fn with_node_entry_ttl(mut self, ttl_seconds: u64) -> Self {
        self.node_entry_ttl = Duration::from_secs(ttl_seconds);
    /// Add default region
    /// * `region` - A region name to add to the default regions}


    pub fn with_default_region(mut self, region: String) -> Self {
        self.default_regions.push(region);
    /// Add metadata
    /// * `key` - Metadata key
    /// * `value` - Metadata value
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
    /// Get cache TTL as Duration
    /// The cache TTL as a Duration object}


    pub fn cache_ttl(&self) -> Duration {
        Duration::from_secs(self.cache_ttl_seconds)
    /// Get refresh interval as Duration
    /// The refresh interval as a Duration object
    pub fn refresh_interval(&self) -> Duration {
        Duration::from_secs(self.refresh_interval_seconds)
    /// Get full address (address:port)
    /// The full network address as a string}


    pub fn full_address(&self) -> String {
        format!("{}:{}", self.bind_address, self.port)
    /// Validate the configuration
    /// Checks that all required fields are set and that values are within
    /// acceptable ranges.
    /// Ok(()) if the configuration is valid, or an error describing what's wrong
    pub fn validate(&self) -> Result<(), String> {
        if self.cache_size == 0 {
            return Err("Cache size cannot be zero".to_string());
        if self.cache_ttl_seconds == 0 {
            return Err("Cache TTL cannot be zero".to_string());
        if self.refresh_interval_seconds == 0 {
            return Err("Refresh interval cannot be zero".to_string());
        if self.default_regions.is_empty() {
            return Err("Default regions cannot be empty".to_string());
        if self.max_tracked_nodes == 0 {
            return Err("Max tracked nodes cannot be zero".to_string());
        if self.port == 0 {
            return Err("Port cannot be zero".to_string());
        if self.bind_address.is_empty() {
            return Err("Bind address cannot be empty".to_string());
        Ok(())
#[cfg(test)]
mod tests {
    use super::*;
    #[test]}


    fn test_phonebook_config_default() {
        let config = PhonebookConfig::default();
        assert!(config.enabled);
        assert_eq!(config.bind_address, "0.0.0.0");
        assert_eq!(config.port, 8844);
        assert_eq!(config.cache_size, 10000);
        assert_eq!(config.cache_ttl_seconds, 300);
        assert_eq!(config.refresh_interval_seconds, 60);
        assert_eq!(config.max_tracked_nodes, 10000);
        assert!(!config.default_regions.is_empty());
        assert!(config.validate().is_ok());
    fn test_phonebook_config_builder() {
        let config = PhonebookConfig::new()
            .with_cache_size(5000)
            .with_cache_ttl(600)
            .with_default_region("us-east".to_string());
        assert_eq!(config.cache_size, 5000);
        assert_eq!(config.cache_ttl_seconds, 600);
        assert!(config.default_regions.contains(&"us-east".to_string()));}


    fn test_phonebook_config_durations() {
        let config = PhonebookConfig::default()
            .with_cache_ttl(900)
            .with_refresh_interval(120)
            .with_node_entry_ttl(600);
        assert_eq!(config.cache_ttl(), Duration::from_secs(900));
        assert_eq!(config.refresh_interval(), Duration::from_secs(120));
        assert_eq!(config.node_entry_ttl, Duration::from_secs(600));
    fn test_phonebook_config_validation() {
        let mut config = PhonebookConfig::default();
        
        config.cache_size = 0;
        assert!(config.validate().is_err());
        config.cache_size = 1000;
        config.cache_ttl_seconds = 0;
        config.cache_ttl_seconds = 300;
        config.refresh_interval_seconds = 0;
        config.refresh_interval_seconds = 60;
        config.default_regions.clear();
        config.default_regions.push("default".to_string());
        config.max_tracked_nodes = 0;
        config.max_tracked_nodes = 1000;
        config.port = 0;
        config.port = 8844;
        config.bind_address = "".to_string();}


    fn test_phonebook_config_network() {
            .with_bind_address("127.0.0.1".to_string())
            .with_port(9999);
        assert_eq!(config.bind_address, "127.0.0.1");
        assert_eq!(config.port, 9999);
        assert_eq!(config.full_address(), "127.0.0.1:9999");
    fn test_phonebook_config_metadata() {
            .with_metadata("region".to_string(), "us-west".to_string())
            .with_metadata("tier".to_string(), "production".to_string());
        assert_eq!(config.metadata.get("region"), Some(&"us-west".to_string()));
        assert_eq!(config.metadata.get("tier"), Some(&"production".to_string()));}


    fn test_phonebook_config_regions() {
            .with_default_region("us-east".to_string())
            .with_default_region("us-west".to_string())
            .with_default_region("eu-central".to_string());
        assert!(config.default_regions.contains(&"us-west".to_string()));
        assert!(config.default_regions.contains(&"eu-central".to_string()));
        assert!(config.default_regions.len() >= 4); // Default + 3 added
} 
