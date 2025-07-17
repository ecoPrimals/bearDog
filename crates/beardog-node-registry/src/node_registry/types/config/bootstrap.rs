//! Bootstrap node configuration types
//!
//! This module contains configuration types for bootstrap nodes, which are
//! the initial nodes that help new nodes discover and join the network.

use std::collections::HashMap;
use std::time::Duration;

/// Bootstrap node configuration
/// 
/// Configuration for a bootstrap node that helps new nodes discover and join
/// the network. Bootstrap nodes are the initial entry points for the network
/// and must be highly available and trusted.
/// 
/// # Example
/// 
/// ```rust
/// use beardog::node_registry::types::config::BootstrapNodeConfig;
/// 
/// let config = BootstrapNodeConfig::new(
///     "bootstrap-1".to_string(),
///     "bootstrap.example.com".to_string(),
///     8080
/// )
/// .with_capability("security".to_string())
/// .with_trust_level(TrustLevel::High);
/// ```
#[derive(Debug, Clone)]
pub struct BootstrapNodeConfig {
    /// Bootstrap node ID
    pub node_id: String,
    /// Bootstrap node address
    pub address: String,
    /// Bootstrap node port
    pub port: u16,
    /// Bootstrap node public key
    pub public_key: Vec<u8>,
    /// Bootstrap node public key hex string
    pub public_key_hex: String,
    /// Bootstrap node network address
    pub network_address: String,
    /// Bootstrap node capabilities
    pub capabilities: Vec<String>,
    /// Bootstrap node trust level
    pub trust_level: crate::node_registry::types::trust::TrustLevel,
    /// Connection timeout in seconds
    pub connection_timeout_seconds: u64,
    /// Retry attempts
    pub retry_attempts: u32,
    /// Retry delay in seconds
    pub retry_delay_seconds: u64,
    /// Bootstrap node metadata
    pub metadata: HashMap<String, String>,
}

impl Default for BootstrapNodeConfig {
    fn default() -> Self {
        Self {
            node_id: "default_node".to_string(),
            address: "localhost".to_string(),
            port: 8080,
            public_key: Vec::new(),
            public_key_hex: String::new(),
            network_address: "localhost:8080".to_string(),
            capabilities: Vec::new(),
            trust_level: crate::node_registry::types::trust::TrustLevel::Unknown,
            connection_timeout_seconds: 30,
            retry_attempts: 3,
            retry_delay_seconds: 5,
            metadata: HashMap::new(),
        }
    }
}

impl BootstrapNodeConfig {
    /// Create a new bootstrap node configuration
    /// 
    /// # Arguments
    /// 
    /// * `node_id` - Unique identifier for the bootstrap node
    /// * `address` - Network address of the bootstrap node
    /// * `port` - Port number the bootstrap node listens on
    /// 
    /// # Returns
    /// 
    /// A new BootstrapNodeConfig with the specified parameters and default
    /// values for all other settings.
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

    /// Set public key
    /// 
    /// # Arguments
    /// 
    /// * `public_key` - The public key bytes for the bootstrap node
    pub fn with_public_key(mut self, public_key: Vec<u8>) -> Self {
        self.public_key = public_key;
        self
    }

    /// Add capability
    /// 
    /// # Arguments
    /// 
    /// * `capability` - A capability string to add to the bootstrap node
    pub fn with_capability(mut self, capability: String) -> Self {
        self.capabilities.push(capability);
        self
    }

    /// Set trust level
    /// 
    /// # Arguments
    /// 
    /// * `trust_level` - The trust level for the bootstrap node
    pub fn with_trust_level(mut self, trust_level: crate::node_registry::types::trust::TrustLevel) -> Self {
        self.trust_level = trust_level;
        self
    }

    /// Set connection timeout
    /// 
    /// # Arguments
    /// 
    /// * `timeout_seconds` - Connection timeout in seconds
    pub fn with_connection_timeout(mut self, timeout_seconds: u64) -> Self {
        self.connection_timeout_seconds = timeout_seconds;
        self
    }

    /// Set retry configuration
    /// 
    /// # Arguments
    /// 
    /// * `attempts` - Number of retry attempts
    /// * `delay_seconds` - Delay between retries in seconds
    pub fn with_retry_config(mut self, attempts: u32, delay_seconds: u64) -> Self {
        self.retry_attempts = attempts;
        self.retry_delay_seconds = delay_seconds;
        self
    }

    /// Get connection timeout as Duration
    /// 
    /// # Returns
    /// 
    /// The connection timeout as a Duration object
    pub fn connection_timeout(&self) -> Duration {
        Duration::from_secs(self.connection_timeout_seconds)
    }

    /// Get retry delay as Duration
    /// 
    /// # Returns
    /// 
    /// The retry delay as a Duration object
    pub fn retry_delay(&self) -> Duration {
        Duration::from_secs(self.retry_delay_seconds)
    }

    /// Get full address (address:port)
    /// 
    /// # Returns
    /// 
    /// The full network address as a string
    pub fn full_address(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }

    /// Validate the configuration
    /// 
    /// Checks that all required fields are set and that values are within
    /// acceptable ranges.
    /// 
    /// # Returns
    /// 
    /// Ok(()) if the configuration is valid, or an error describing what's wrong
    pub fn validate(&self) -> crate::BearDogResult<()> {
        if self.address.is_empty() {
            return Err(crate::error::BearDogError::validation("address", "Address cannot be empty"));
        }

        if self.port == 0 {
            return Err(crate::error::BearDogError::validation("port", "Port cannot be zero"));
        }

        if self.connection_timeout_seconds == 0 {
            return Err(crate::error::BearDogError::validation("connection_timeout_seconds", "Connection timeout cannot be zero"));
        }

        if self.retry_delay_seconds == 0 {
            return Err(crate::error::BearDogError::validation("retry_delay_seconds", "Retry delay cannot be zero"));
        }

        Ok(())
    }

    /// Convert to NodeInfo
    /// 
    /// Creates a NodeInfo structure from this bootstrap configuration,
    /// which can be used in the registry system.
    /// 
    /// # Returns
    /// 
    /// A NodeInfo structure representing this bootstrap node
    pub fn to_node_info(&self) -> crate::BearDogResult<crate::node_registry::types::node::NodeInfo> {
        let mut node_info = crate::node_registry::types::node::NodeInfo::new(
            self.node_id.clone(),
            format!("Bootstrap Node {}", self.node_id),
            "bootstrap".to_string(),
        );
        
        node_info.endpoints = vec![self.network_address.clone()];
        node_info.public_key = self.public_key.clone();
        node_info.capabilities = self.capabilities.clone();
        node_info.trust_level = self.trust_level;
        node_info.metadata = self.metadata.clone();
        
        Ok(node_info)
    }
}

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
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_bootstrap_node_config_default() {
        let config = BootstrapNodeConfig::default();
        assert_eq!(config.node_id, "default_node");
        assert_eq!(config.address, "localhost");
        assert_eq!(config.port, 8080);
        assert_eq!(config.network_address, "localhost:8080");
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_bootstrap_node_config_durations() {
        let config = BootstrapNodeConfig::default()
            .with_connection_timeout(60)
            .with_retry_config(5, 10);

        assert_eq!(config.connection_timeout(), Duration::from_secs(60));
        assert_eq!(config.retry_delay(), Duration::from_secs(10));
        assert_eq!(config.retry_attempts, 5);
    }

    #[test]
    fn test_bootstrap_node_config_validation() {
        let mut config = BootstrapNodeConfig::default();
        config.address = "".to_string();
        assert!(config.validate().is_err());

        config.address = "localhost".to_string();
        config.port = 0;
        assert!(config.validate().is_err());

        config.port = 8080;
        config.connection_timeout_seconds = 0;
        assert!(config.validate().is_err());
    }
} 