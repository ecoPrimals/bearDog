use beardog_errors::BearDogError;

use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
    /// The address value
    pub address: String,

    /// Number of port
    pub port: u16,

    /// Collection of public key
    pub public_key: Vec<u8>,

    /// The public key hex value
    pub public_key_hex: String,

    /// The network address value
    pub network_address: String,

    /// Collection of capabilities
    pub capabilities: Vec<String>,

    /// The trust level value
    pub trust_level: crate::node_registry::types::trust::TrustLevel,


    pub connection_timeout_seconds: u64,

    /// Number of retry_attempts
    pub retry_attempts: u32,

    /// Number of retry_delay_seconds
    pub retry_delay_seconds: u64,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

impl Default for BootstrapNodeConfig {
    fn default() -> Self {
        let network_config = beardog_types::canonical::config::network::NetworkConfig::default();
        let default_address = format!("{}:{}", network_config.default_host, network_config.service_ports.api_port);
        let default_network_address = std::env::var("BEARDOG_BOOTSTRAP_NETWORK_ADDRESS")
            .unwrap_or_else(|_| format!("{}:{}", network_config.default_host, network_config.service_ports.api_port));
        
        Self {
            node_id: std::env::var("BEARDOG_BOOTSTRAP_NODE_ID")
                .unwrap_or_else(|_| "default_node".to_string()),
            address: std::env::var("BEARDOG_BOOTSTRAP_ADDRESS")
                .unwrap_or_else(|_| network_config.default_host.clone()),
            port: std::env::var("BEARDOG_BOOTSTRAP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(network_config.service_ports.api_port),
            public_key: Vec::new(),
            public_key_hex: String::with_capacity(64),
            network_address: default_network_address,
            capabilities: Vec::new(),
            trust_level: crate::node_registry::types::trust::TrustLevel::Unknown,
            connection_timeout_seconds: 30,
            retry_attempts: 3,
            retry_delay_seconds: 5,
            metadata: HashMap::with_capacity(16),
        }
    }
}

impl BootstrapNodeConfig {
    /// New operation.
    /// Creates a new instance
    pub fn new(node_id: String, address: String, port: u16) -> Self {
        let network_address = format!("{}:{}", address, port);
        Self {
            node_id,
            address,
            port,
            network_address,
            ..Default::default()
        }
    }

/// With Public Key operation.
    /// Creates instance with public key
    pub fn with_public_key(mut self, public_key: Vec<u8>) -> Self {
        self.public_key = public_key;
        self

/// With Capability operation.
    /// Creates instance with capability
    pub fn with_capability(mut self, capability: &str) -> Self {
        self.capabilities.push(capability);

/// With Trust Level operation.
    /// Creates instance with trust level
    pub fn with_trust_level(mut self, trust_level: crate::node_registry::types::trust::TrustLevel) -> Self {
        self.trust_level = trust_level;

/// With Connection Timeout operation.
    /// Creates instance with connection timeout
    pub fn with_connection_timeout(mut self, timeout_seconds: u64) -> Self {
        self.connection_timeout_seconds = timeout_seconds;

/// With Retry Config operation.
    /// Creates instance with retry config
    pub fn with_retry_config(u32, delay_seconds: u64) -> Self {
        self.retry_attempts = attempts;
        self.retry_delay_seconds = delay_seconds;

/// Connection Timeout operation.
    pub fn connection_timeout(&self) -> Duration {
        Duration::from_secs(self.connection_timeout_seconds)

/// Retry Delay operation.
    pub fn retry_delay(&self) -> Duration {
        Duration::from_secs(self.retry_delay_seconds)

/// Full Address operation.
    pub fn full_address(&self) -> String {
        format!("{}:{}", self.address, self.port)

/// Validate operation.
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> crate::Result<(), BearDogError> {
        if self.address.is_empty() {
            return Err(crate::error::BearDogError::validation("address", "Address cannot be empty"));
        if self.port == 0 {
            return Err(crate::error::BearDogError::validation("port", "Port cannot be zero"));
        if self.connection_timeout_seconds == 0 {
            return Err(crate::error::BearDogError::validation("connection_timeout_seconds", "Connection timeout cannot be zero"));
        if self.retry_delay_seconds == 0 {
            return Err(crate::error::BearDogError::validation("retry_delay_seconds", "Retry delay cannot be zero"));
        Ok(())

/// To Node Info operation.
    /// Converts to node info
    pub fn to_node_info(&self) -> crate::Result<crate::node_registry::types::node::NodeInfo, BearDogError> {
        let mut node_info = crate::node_registry::types::node::NodeInfo::new(
            &self.node_id,
            format!("Bootstrap Node {}", self.node_id),
            "bootstrap");
        
        node_info.endpoints = vec![&self.network_address];
        node_info.public_key = &self.public_key;
        node_info.capabilities = &self.capabilities;
        node_info.trust_level = self.trust_level;
        node_info.metadata = &self.metadata;
        Ok(node_info)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bootstrap_node_config() {
        let config = BootstrapNodeConfig::new("test_node".to_string(), "localhost".to_string(), 8080)
            .with_capability("security".to_string())
            .with_trust_level(crate::node_registry::types::trust::TrustLevel::High);
        assert_eq!(config.address, "localhost");
        assert_eq!(config.port, 8080);
        assert!(config.capabilities.contains(&"security".to_string()));
        assert_eq!(config.trust_level, crate::node_registry::types::trust::TrustLevel::High);
        assert_eq!(config.full_address(), "localhost:8080");
        assert!(config.validate().is_ok());}


    fn test_bootstrap_node_config_default() {
        let config = BootstrapNodeConfig::default();
        assert_eq!(config.node_id, "default_node");
        assert_eq!(config.network_address, "localhost:8080");
    fn test_bootstrap_node_config_durations() {
        let config = BootstrapNodeConfig::default()
            .with_connection_timeout(60)
            .with_retry_config(5, 10);
        assert_eq!(config.connection_timeout(), Duration::from_secs(60));
        assert_eq!(config.retry_delay(), Duration::from_secs(10));
        assert_eq!(config.retry_attempts, 5);}


    fn test_bootstrap_node_config_validation() {
        let mut config = BootstrapNodeConfig::default();
        config.address = "".to_string();
        assert!(config.validate().is_err());
        config.address = "localhost".to_string();
        config.port = 0;
        config.port = 8080;
        config.connection_timeout_seconds = 0;
} 
