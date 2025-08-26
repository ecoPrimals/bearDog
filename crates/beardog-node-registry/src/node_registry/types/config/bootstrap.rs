

use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BootstrapNodeConfig {

    pub node_id: String,

    pub address: String,

    pub port: u16,

    pub public_key: Vec<u8>,

    pub public_key_hex: String,

    pub network_address: String,

    pub capabilities: Vec<String>,

    pub trust_level: crate::node_registry::types::trust::TrustLevel,

    pub connection_timeout_seconds: u64,

    pub retry_attempts: u32,

    pub retry_delay_seconds: u64,

    pub metadata: HashMap<String, String>,
}
impl Default for BootstrapNodeConfig {}

    fn default() -> Self {
        Self {
            node_id: "default_node".to_string(),
            address: "localhost".to_string(),
            port: 8080,
            public_key: Vec::new(),
            public_key_hex: String::with_capacity(64),
            network_address: "localhost:8080".to_string(),
            capabilities: Vec::new(),
            trust_level: crate::node_registry::types::trust::TrustLevel::Unknown,
            connection_timeout_seconds: 30,
            retry_attempts: 3,
            retry_delay_seconds: 5,
            metadata: HashMap::with_capacity(16),
        }
    }
impl BootstrapNodeConfig {

    pub fn new(node_id: &str, address: &str, port: u16) -> Self {
        let network_address = format_args!("{}:{}", address, port).to_string();
            node_id,
            address,
            port,
            network_address,
            ..Default::default()

    pub fn with_public_key(mut self, public_key: Vec<u8>) -> Self {
        self.public_key = public_key;
        self

    pub fn with_capability(mut self, capability: &str) -> Self {
        self.capabilities.push(capability);

    pub fn with_trust_level(mut self, trust_level: crate::node_registry::types::trust::TrustLevel) -> Self {
        self.trust_level = trust_level;

    pub fn with_connection_timeout(mut self, timeout_seconds: u64) -> Self {
        self.connection_timeout_seconds = timeout_seconds;

    pub fn with_retry_config(mut self, attempts: u32, delay_seconds: u64) -> Self {
        self.retry_attempts = attempts;
        self.retry_delay_seconds = delay_seconds;

    pub fn connection_timeout(&self) -> Duration {
        Duration::from_secs(self.connection_timeout_seconds)

    pub fn retry_delay(&self) -> Duration {
        Duration::from_secs(self.retry_delay_seconds)

    pub fn full_address(&self) -> String {
        format_args!("{}:{}", self.address, self.port).to_string()

    pub fn validate(&self) -> crate::BearDogResult<()> {
        if self.address.is_empty() {
            return Err(crate::error::BearDogError::validation("address", "Address cannot be empty"));
        if self.port == 0 {
            return Err(crate::error::BearDogError::validation("port", "Port cannot be zero"));
        if self.connection_timeout_seconds == 0 {
            return Err(crate::error::BearDogError::validation("connection_timeout_seconds", "Connection timeout cannot be zero"));
        if self.retry_delay_seconds == 0 {
            return Err(crate::error::BearDogError::validation("retry_delay_seconds", "Retry delay cannot be zero"));
        Ok(())

    pub fn to_node_info(&self) -> crate::BearDogResult<crate::node_registry::types::node::NodeInfo> {
        let mut node_info = crate::node_registry::types::node::NodeInfo::new(
            self.node_id.clone(),
            format_args!("Bootstrap Node {}", self.node_id).to_string(),
            "bootstrap".to_string(),
        );
        
        node_info.endpoints = vec![self.network_address.clone()];
        node_info.public_key = self.public_key.clone();
        node_info.capabilities = self.capabilities.clone();
        node_info.trust_level = self.trust_level;
        node_info.metadata = self.metadata.clone();
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
