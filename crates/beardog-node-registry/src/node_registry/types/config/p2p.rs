

use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
    /// The listen address value
    pub listen_address: String,

    /// Number of listen_port
    pub listen_port: u16,

    /// Collection of bootstrap peers
    pub bootstrap_peers: Vec<String>,

    /// Number of max_peers
    pub max_peers: usize,


    pub connection_timeout_seconds: u64,

    /// Number of peer_discovery_interval_seconds
    pub peer_discovery_interval_seconds: u64,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}
impl Default for P2PConfig {
    fn default() -> Self {
        let network_config = beardog_types::canonical::config::network::NetworkConfig::default();
        Self {
            enabled: true,
            listen_address: std::env::var("BEARDOG_P2P_LISTEN_ADDRESS")
                .or_else(|_| std::env::var("BEARDOG_BIND_ADDRESS"))
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            listen_port: std::env::var("BEARDOG_P2P_LISTEN_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(network_config.service_ports.p2p_port),
            bootstrap_peers: Vec::new(),
            max_peers: std::env::var("BEARDOG_P2P_MAX_PEERS")
                .ok()
                .and_then(|m| m.parse().ok())
                .unwrap_or(50),
            connection_timeout_seconds: std::env::var("BEARDOG_P2P_CONNECTION_TIMEOUT_SECS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30),
            peer_discovery_interval_seconds: std::env::var("BEARDOG_P2P_PEER_DISCOVERY_INTERVAL_SECS")
                .ok()
                .and_then(|i| i.parse().ok())
                .unwrap_or(60),
            metadata: HashMap::with_capacity(16),
        }
    }
}
impl P2PConfig {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()

/// With Enabled operation.
    /// Creates instance with enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self

/// With Listen Address operation.
    /// Creates instance with listen address
    pub fn with_listen_address(mut self, address: &str) -> Self {
        self.listen_address = address;

/// With Listen Port operation.
    /// Creates instance with listen port
    pub fn with_listen_port(mut self, port: u16) -> Self {
        self.listen_port = port;

/// With Bootstrap Peer operation.
    /// Creates instance with bootstrap peer
    pub fn with_bootstrap_peer(mut self, peer: &str) -> Self {
        self.bootstrap_peers.push(peer);

/// With Max Peers operation.
    /// Creates instance with max peers
    pub fn with_max_peers(mut self, max_peers: usize) -> Self {
        self.max_peers = max_peers;

/// With Connection Timeout operation.
    /// Creates instance with connection timeout
    pub fn with_connection_timeout(mut self, timeout_seconds: u64) -> Self {
        self.connection_timeout_seconds = timeout_seconds;

/// With Peer Discovery Interval operation.
    /// Creates instance with peer discovery interval
    pub fn with_peer_discovery_interval(mut self, interval_seconds: u64) -> Self {
        self.peer_discovery_interval_seconds = interval_seconds;

/// With Metadata operation.
    /// Creates instance with metadata
    pub fn with_metadata(&str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.into());

/// Connection Timeout operation.
    pub fn connection_timeout(&self) -> Duration {
        Duration::from_secs(self.connection_timeout_seconds)

/// Peer Discovery Interval operation.
    pub fn peer_discovery_interval(&self) -> Duration {
        Duration::from_secs(self.peer_discovery_interval_seconds)

/// Full Listen Address operation.
    pub fn full_listen_address(&self) -> String {
        format!("{}:{}", self.listen_address, self.listen_port)

/// Validate operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates input
    /// Validates input
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
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
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
            .with_metadata("encryption ".to_string(), "tls".to_string());
        assert_eq!(config.metadata.get("protocol"), Some("tcp".to_string()));
        assert_eq!(config.metadata.get("encryption "), Some("tls".to_string()));
    fn test_p2p_config_disabled() {
            .with_enabled(false);
        assert!(!config.enabled);
        assert!(config.validate().is_ok()); // Should still validate even when disabled
} 
