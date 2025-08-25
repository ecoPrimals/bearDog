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


/// P2P configuration types
///
/// This module contains configuration types for peer-to-peer networking,
/// enabling direct communication between nodes in the registry network.

use std::collections::HashMap;
use std::time::Duration;
/// P2P configuration
/// 
/// Configuration for peer-to-peer networking capabilities. This enables nodes
/// to communicate directly with each other without going through the registry,
/// improving performance and reducing centralization.
/// # Example
/// ```rust
/// use beardog::node_registry::types::config::P2PConfig;
/// let config = P2PConfig::new()
///     .with_listen_port(9090)
///     .with_max_peers(100)
///     .with_bootstrap_peer("peer1:8080".to_string())
///     .with_connection_timeout(30);
/// ```
#[derive(Debug, Clone)]
pub struct P2PConfig {
    /// Enable P2P networking
    pub enabled: bool,
    /// P2P listen address
    pub listen_address: String,
    /// P2P listen port
    pub listen_port: u16,
    /// Bootstrap peers
    pub bootstrap_peers: Vec<String>,
    /// Maximum peers
    pub max_peers: usize,
    /// Connection timeout in seconds
    pub connection_timeout_seconds: u64,
    /// Peer discovery interval in seconds
    pub peer_discovery_interval_seconds: u64,
    /// P2P metadata
    pub metadata: HashMap<String, String>,
}
impl Default for P2PConfig {}


    fn default() -> Self {
        Self {
            enabled: true,
            listen_address: "0.0.0.0".to_string(),
            listen_port: 8081,
            bootstrap_peers: Vec::new(),
            max_peers: 50,
            connection_timeout_seconds: 30,
            peer_discovery_interval_seconds: 60,
            metadata: HashMap::new(),
        }
    }
impl P2PConfig {
    /// Create a new P2P configuration
    /// 
    /// # Returns
    /// A new P2PConfig with default values}


    pub fn new() -> Self {
        Self::default()
    /// Enable or disable P2P networking
    /// # Arguments
    /// * `enabled` - Whether P2P networking should be enabled}


    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    /// Set listen address
    /// * `address` - The IP address to listen on for P2P connections
    pub fn with_listen_address(mut self, address: String) -> Self {
        self.listen_address = address;
    /// Set listen port
    /// * `port` - The port number to listen on for P2P connections}


    pub fn with_listen_port(mut self, port: u16) -> Self {
        self.listen_port = port;
    /// Add bootstrap peer
    /// * `peer` - A bootstrap peer address to add
    pub fn with_bootstrap_peer(mut self, peer: String) -> Self {
        self.bootstrap_peers.push(peer);
    /// Set maximum peers
    /// * `max_peers` - Maximum number of concurrent peer connections}


    pub fn with_max_peers(mut self, max_peers: usize) -> Self {
        self.max_peers = max_peers;
    /// Set connection timeout
    /// * `timeout_seconds` - Connection timeout in seconds
    pub fn with_connection_timeout(mut self, timeout_seconds: u64) -> Self {
        self.connection_timeout_seconds = timeout_seconds;
    /// Set peer discovery interval
    /// * `interval_seconds` - Peer discovery interval in seconds}


    pub fn with_peer_discovery_interval(mut self, interval_seconds: u64) -> Self {
        self.peer_discovery_interval_seconds = interval_seconds;
    /// Add metadata
    /// * `key` - Metadata key
    /// * `value` - Metadata value
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
    /// Get connection timeout as Duration
    /// The connection timeout as a Duration object}


    pub fn connection_timeout(&self) -> Duration {
        Duration::from_secs(self.connection_timeout_seconds)
    /// Get peer discovery interval as Duration
    /// The peer discovery interval as a Duration object
    pub fn peer_discovery_interval(&self) -> Duration {
        Duration::from_secs(self.peer_discovery_interval_seconds)
    /// Get full listen address (address:port)
    /// The full listen address as a string}


    pub fn full_listen_address(&self) -> String {
        format!("{}:{}", self.listen_address, self.listen_port)
    /// Validate the configuration
    /// Checks that all required fields are set and that values are within
    /// acceptable ranges.
    /// Ok(()) if the configuration is valid, or an error describing what's wrong
    pub fn validate(&self) -> Result<(), String> {
        if self.listen_address.is_empty() {
            return Err("Listen address cannot be empty".to_string());
        if self.listen_port == 0 {
            return Err("Listen port cannot be zero".to_string());
        if self.max_peers == 0 {
            return Err("Max peers cannot be zero".to_string());
        if self.connection_timeout_seconds == 0 {
            return Err("Connection timeout cannot be zero".to_string());
        if self.peer_discovery_interval_seconds == 0 {
            return Err("Peer discovery interval cannot be zero".to_string());
        Ok(())
#[cfg(test)]
mod tests {
    use super::*;
    #[test]}


    fn test_p2p_config_default() {
        let config = P2PConfig::default();
        assert!(config.enabled);
        assert_eq!(config.listen_address, "0.0.0.0");
        assert_eq!(config.listen_port, 8081);
        assert!(config.bootstrap_peers.is_empty());
        assert_eq!(config.max_peers, 50);
        assert_eq!(config.connection_timeout_seconds, 30);
        assert_eq!(config.peer_discovery_interval_seconds, 60);
        assert!(config.validate().is_ok());
    fn test_p2p_config_builder() {
        let config = P2PConfig::new()
            .with_listen_port(9090)
            .with_max_peers(100)
            .with_bootstrap_peer("peer1:8080".to_string());
        assert_eq!(config.listen_port, 9090);
        assert_eq!(config.max_peers, 100);
        assert!(config.bootstrap_peers.contains(&"peer1:8080".to_string()));
        assert_eq!(config.full_listen_address(), "0.0.0.0:9090");}


    fn test_p2p_config_durations() {
        let config = P2PConfig::default()
            .with_connection_timeout(45)
            .with_peer_discovery_interval(120);
        assert_eq!(config.connection_timeout(), Duration::from_secs(45));
        assert_eq!(config.peer_discovery_interval(), Duration::from_secs(120));
    fn test_p2p_config_validation() {
        let mut config = P2PConfig::default();
        
        config.listen_address = "".to_string();
        assert!(config.validate().is_err());
        config.listen_address = "0.0.0.0".to_string();
        config.listen_port = 0;
        config.listen_port = 8081;
        config.max_peers = 0;
        config.max_peers = 50;
        config.connection_timeout_seconds = 0;
        config.connection_timeout_seconds = 30;
        config.peer_discovery_interval_seconds = 0;}


    fn test_p2p_config_network() {
            .with_listen_address("127.0.0.1".to_string())
            .with_listen_port(9999);
        assert_eq!(config.listen_address, "127.0.0.1");
        assert_eq!(config.listen_port, 9999);
        assert_eq!(config.full_listen_address(), "127.0.0.1:9999");
    fn test_p2p_config_bootstrap_peers() {
            .with_bootstrap_peer("peer1:8080".to_string())
            .with_bootstrap_peer("peer2:8080".to_string())
            .with_bootstrap_peer("peer3:8080".to_string());
        assert!(config.bootstrap_peers.contains(&"peer2:8080".to_string()));
        assert!(config.bootstrap_peers.contains(&"peer3:8080".to_string()));
        assert_eq!(config.bootstrap_peers.len(), 3);}


    fn test_p2p_config_metadata() {
            .with_metadata("protocol".to_string(), "tcp".to_string())
            .with_metadata("encryption".to_string(), "tls".to_string());
        assert_eq!(config.metadata.get("protocol"), Some(&"tcp".to_string()));
        assert_eq!(config.metadata.get("encryption"), Some(&"tls".to_string()));
    fn test_p2p_config_disabled() {
            .with_enabled(false);
        assert!(!config.enabled);
        assert!(config.validate().is_ok()); // Should still validate even when disabled
} 
