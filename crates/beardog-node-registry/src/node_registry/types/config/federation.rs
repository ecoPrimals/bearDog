

use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct FederationConfig {

    pub enabled: bool,

    pub discovery_endpoints: Vec<String>,

    pub heartbeat_interval_seconds: u64,

    pub timeout_seconds: u64,

    pub max_partners: usize,

    pub max_federated_registries: usize,

    pub min_federation_trust: crate::node_registry::types::trust::TrustLevel,

    pub metadata: HashMap<String, String>,
}
impl Default for FederationConfig {}

    fn default() -> Self {
        Self {
            enabled: true,
            discovery_endpoints: vec![
                "https://federation.beardog.local:8443".to_string(),
            ],
            heartbeat_interval_seconds: 30,
            timeout_seconds: 300,
            max_partners: 100,
            max_federated_registries: 10,
            min_federation_trust: crate::node_registry::types::trust::TrustLevel::Basic,
            metadata: HashMap::with_capacity(16),
        }
    }
impl FederationConfig {

    pub fn new() -> Self {
        Self::default()

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self

    pub fn with_discovery_endpoint(mut self, endpoint: &str) -> Self {
        self.discovery_endpoints.push(endpoint);

    pub fn with_heartbeat_interval(mut self, interval_seconds: u64) -> Self {
        self.heartbeat_interval_seconds = interval_seconds;

    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;

    pub fn with_max_partners(mut self, max_partners: usize) -> Self {
        self.max_partners = max_partners;

    pub fn with_max_federated_registries(mut self, max_federated_registries: usize) -> Self {
        self.max_federated_registries = max_federated_registries;

    pub fn with_min_federation_trust(mut self, min_federation_trust: crate::node_registry::types::trust::TrustLevel) -> Self {
        self.min_federation_trust = min_federation_trust;

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key, value);

    pub fn heartbeat_interval(&self) -> Duration {
        Duration::from_secs(self.heartbeat_interval_seconds)

    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_seconds)

    pub fn validate(&self) -> Result<(), String> {
        if self.enabled && self.discovery_endpoints.is_empty() {
            return Err("Discovery endpoints cannot be empty when federation is enabled".to_string());
        if self.heartbeat_interval_seconds == 0 {
            return Err("Heartbeat interval cannot be zero".to_string());
        if self.timeout_seconds == 0 {
            return Err("Timeout cannot be zero".to_string());
        if self.max_partners == 0 {
            return Err("Max partners cannot be zero".to_string());
        if self.max_federated_registries == 0 {
            return Err("Max federated registries cannot be zero".to_string());
        if self.min_federation_trust == crate::node_registry::types::trust::TrustLevel::Unknown {
            return Err("Minimum federation trust level cannot be Unknown".to_string());
        Ok(())
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_federation_config_default() {
        let config = FederationConfig::default();
        assert!(config.enabled);
        assert!(!config.discovery_endpoints.is_empty());
        assert_eq!(config.heartbeat_interval_seconds, 30);
        assert_eq!(config.timeout_seconds, 300);
        assert_eq!(config.max_partners, 100);
        assert_eq!(config.max_federated_registries, 10);
        assert!(config.validate().is_ok());}

    fn test_federation_config_builder() {
        let config = FederationConfig::new()
            .with_enabled(true)
            .with_heartbeat_interval(60)
            .with_max_partners(50)
            .with_max_federated_registries(10)
            .with_min_federation_trust(crate::node_registry::types::trust::TrustLevel::Basic);
        assert_eq!(config.heartbeat_interval_seconds, 60);
        assert_eq!(config.max_partners, 50);
        assert_eq!(config.min_federation_trust, crate::node_registry::types::trust::TrustLevel::Basic);
    fn test_federation_config_durations() {
        let config = FederationConfig::default()
            .with_heartbeat_interval(45)
            .with_timeout(600);
        assert_eq!(config.heartbeat_interval(), Duration::from_secs(45));
        assert_eq!(config.timeout(), Duration::from_secs(600));}

    fn test_federation_config_validation() {
        let mut config = FederationConfig::default();
        config.enabled = true;
        config.discovery_endpoints.clear();
        assert!(config.validate().is_err());
        config.discovery_endpoints.push("https://example.com".to_string());
        config.heartbeat_interval_seconds = 0;
        config.heartbeat_interval_seconds = 30;
        config.timeout_seconds = 0;
        config.timeout_seconds = 300;
        config.max_partners = 0;
        config.max_partners = 100;
        config.max_federated_registries = 0;
        config.max_federated_registries = 10;
        config.min_federation_trust = crate::node_registry::types::trust::TrustLevel::Unknown;
    fn test_federation_config_metadata() {
            .with_metadata("region".to_string(), "us-east".to_string())
            .with_metadata("tier".to_string(), "production".to_string());
        assert_eq!(config.metadata.get("region"), Some(&"us-east".to_string()));
        assert_eq!(config.metadata.get("tier"), Some(&"production".to_string()));}

    fn test_federation_config_discovery_endpoints() {
            .with_discovery_endpoint("https://registry1.example.com:8443".to_string())
            .with_discovery_endpoint("https://registry2.example.com:8443".to_string());
        assert!(config.discovery_endpoints.contains(&"https://registry1.example.com:8443".to_string()));
        assert!(config.discovery_endpoints.contains(&"https://registry2.example.com:8443".to_string()));
        assert!(config.discovery_endpoints.len() >= 3); // Default + 2 added
} 
