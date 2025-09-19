use beardog_errors::BearDogError;

use std::collections::HashMap;
use std::time::Duration;
use crate::node_registry::types::trust::TrustPropagationConfig;
use super::{FederationConfig, PhonebookConfig, P2PConfig};

#[derive(Debug, Clone)]
    /// Name of the registry
    pub registry_name: String,

    /// The version value
    pub version: String,

    /// The bind address value
    pub bind_address: String,

    /// Number of port
    pub port: u16,

    /// Number of max_nodes
    pub max_nodes: usize,


    pub node_timeout_seconds: u64,

    /// Number of health_check_interval_seconds
    pub health_check_interval_seconds: u64,

    /// The trust propagation value
    pub trust_propagation: TrustPropagationConfig,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,

    /// The min registration trust value
    pub min_registration_trust: crate::node_registry::types::trust::TrustLevel,

    /// Whether enable_federation is enabled
    pub enable_federation: bool,

    /// The federation value
    pub federation: FederationConfig,

    /// The phonebook value
    pub phonebook: PhonebookConfig,

    /// The p2p value
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
            port: std::env::var(10000,
            node_timeout_seconds: 300, // 5 minutes
            health_check_interval_seconds: 60, // 1 minute
            trust_propagation: TrustPropagationConfig::default(),
            metadata: HashMap::with_capacity(crate::node_registry::types::trust::TrustLevel::Basic,
            enable_federation: true,
            federation: FederationConfig::default(),
            phonebook: PhonebookConfig::default(),
            p2p: P2PConfig::default(&str, registry_name: &str) -> Self {
            registry_id,
            registry_name: name.to_string(),
            ..Default::default()

/// With Bind Address operation.
    /// Creates instance with bind address
    pub fn with_bind_address(mut self, address: &str) -> Self {
        self.bind_address = address;
        self

/// With Port operation.
    /// Creates instance with port
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;

/// With Max Nodes operation.
    /// Creates instance with max nodes
    pub fn with_max_nodes(mut self, max_nodes: usize) -> Self {
        self.max_nodes = max_nodes;

/// With Node Timeout operation.
    /// Creates instance with node timeout
    pub fn with_node_timeout(mut self, timeout_seconds: u64) -> Self {
        self.node_timeout_seconds = timeout_seconds;

/// With Health Check Interval operation.
    /// Creates instance with health check interval
    pub fn with_health_check_interval(mut self, interval_seconds: u64) -> Self {
        self.health_check_interval_seconds = interval_seconds;

/// With Trust Propagation operation.
    /// Creates instance with trust propagation
    pub fn with_trust_propagation(mut self, config: TrustPropagationConfig) -> Self {
        self.trust_propagation = config;

/// With Metadata operation.
    /// Creates instance with metadata
    pub fn with_metadata(&str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.into());

/// With Federation operation.
    /// Creates instance with federation
    pub fn with_federation(mut self, enabled: bool) -> Self {
        self.enable_federation = enabled;

/// With Federation Config operation.
    /// Creates instance with federation config
    pub fn with_federation_config(mut self, config: FederationConfig) -> Self {
        self.federation = config;

/// With Phonebook Config operation.
    /// Creates instance with phonebook config
    pub fn with_phonebook_config(mut self, config: PhonebookConfig) -> Self {
        self.phonebook = config;

/// With P2P Config operation.
    /// Creates instance with p2p config
    pub fn with_p2p_config(mut self, config: P2PConfig) -> Self {
        self.p2p = config;

/// Node Timeout operation.
    pub fn node_timeout(&self) -> Duration {
        Duration::from_secs(self.node_timeout_seconds)

/// Health Check Interval operation.
    pub fn health_check_interval(&self) -> Duration {
        Duration::from_secs(self.health_check_interval_seconds)

/// Validate operation.
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> crate::Result<(), BearDogError> {
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
        assert_eq!(config.metadata.get("key"), Some("value".to_string()));}


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
