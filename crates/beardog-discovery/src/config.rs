//! Configuration loading for discovery

use crate::{
    error::{DiscoveryError, Result},
    types::{Capability, PrimalInfo, RequiredCapability, ServiceEndpoint},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    pub primal_self: PrimalSelfConfig,
    pub required_capabilities: HashMap<String, RequiredCapabilityConfig>,
    pub discovery: DiscoveryMethodsConfig,
    pub service_selection: ServiceSelectionConfig,
}

impl DiscoveryConfig {
    /// Load configuration from TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: DiscoveryConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Get primal info (self-knowledge)
    pub fn primal_info(&self) -> PrimalInfo {
        PrimalInfo {
            primal_id: self.primal_self.primal_id.clone(),
            primal_type: self.primal_self.primal_type.clone(),
            version: self.primal_self.version.clone(),
            display_name: self.primal_self.display_name.clone(),
            capabilities: self
                .primal_self
                .self_capabilities
                .iter()
                .map(|cap_type| Capability {
                    capability_type: cap_type.clone(),
                    version: self.primal_self.version.clone(),
                    features: vec![],
                    parameters: HashMap::new(),
                })
                .collect(),
            endpoint: ServiceEndpoint {
                primary_url: format!(
                    "{}://{}:{}",
                    self.primal_self.endpoint.scheme,
                    self.primal_self.endpoint.host,
                    self.primal_self.endpoint.port
                ),
                fallback_urls: vec![],
                use_tls: self.primal_self.endpoint.scheme == "https",
                path_prefix: Some(self.primal_self.endpoint.path_prefix.clone()),
            },
        }
    }

    /// Get required capabilities
    pub fn required_capabilities(&self) -> Vec<RequiredCapability> {
        self.required_capabilities
            .iter()
            .map(|(cap_type, config)| RequiredCapability {
                capability_type: cap_type.clone(),
                required: config.required,
                min_version: config.min_version.clone(),
                features: config.features.clone(),
                fallback: config.fallback.clone(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalSelfConfig {
    pub primal_id: String,
    pub primal_type: String,
    pub version: String,
    pub display_name: String,
    pub self_capabilities: Vec<String>,
    pub endpoint: EndpointConfig,
    pub announcement: AnnouncementConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    pub host: String,
    pub port: u16,
    pub scheme: String,
    pub path_prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementConfig {
    pub enabled: bool,
    pub methods: Vec<String>,
    pub announcement_interval_secs: u64,
    pub ttl_secs: u64,
    #[serde(default)]
    pub mdns: MdnsAnnouncementConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MdnsAnnouncementConfig {
    pub enabled: bool,
    pub service_name: String,
    pub service_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredCapabilityConfig {
    pub required: bool,
    pub preferred: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,
    pub features: Vec<String>,
    pub fallback: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryMethodsConfig {
    pub methods: Vec<String>,
    pub discovery_timeout_secs: u64,
    pub discovery_interval_secs: u64,
    pub cache_ttl_secs: u64,
    #[serde(default)]
    pub environment: EnvironmentDiscoveryConfig,
    #[serde(default)]
    pub mdns: MdnsDiscoveryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentDiscoveryConfig {
    pub enabled: bool,
    pub capability_pattern: String,
    pub primal_pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MdnsDiscoveryConfig {
    pub enabled: bool,
    pub timeout_secs: u64,
    pub service_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSelectionConfig {
    pub strategy: String,
    pub qos_weights: QoSWeightsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QoSWeightsConfig {
    pub latency: f64,
    pub throughput: f64,
    pub availability: f64,
    pub reliability: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        let config_toml = r#"
[primal_self]
primal_id = "beardog"
primal_type = "security"
version = "0.9.5"
display_name = "BearDog Security"
self_capabilities = ["security", "encryption"]

[primal_self.endpoint]
host = "0.0.0.0"
port = 8443
scheme = "https"
path_prefix = "/api/v1"

[primal_self.announcement]
enabled = true
methods = ["mdns"]
announcement_interval_secs = 60
ttl_secs = 300

[required_capabilities.orchestration]
required = false
preferred = true
features = ["service_discovery"]
fallback = "standalone"

[discovery]
methods = ["environment", "mdns"]
discovery_timeout_secs = 10
discovery_interval_secs = 300
cache_ttl_secs = 600

[service_selection]
strategy = "qos_based"

[service_selection.qos_weights]
latency = 0.4
throughput = 0.2
availability = 0.3
reliability = 0.1
        "#;

        let config: DiscoveryConfig = toml::from_str(config_toml).unwrap();
        assert_eq!(config.primal_self.primal_id, "beardog");
        assert_eq!(config.primal_self.self_capabilities.len(), 2);
    }
}

