// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration loading for discovery

use crate::{
    error::Result,
    types::{Capability, PrimalInfo, RequiredCapability, ServiceEndpoint},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Root TOML configuration for how this primal describes itself and finds others.
///
/// Loaded from files such as `configs/beardog-primal-capabilities.toml` and drives both
/// outbound discovery (finding dependencies) and optional announcement of this instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Identity, advertised capabilities, and listen endpoint for this primal.
    pub primal_self: PrimalSelfConfig,
    /// Capability keys this primal expects from the ecosystem (required vs optional, versions, fallbacks).
    pub required_capabilities: HashMap<String, RequiredCapabilityConfig>,
    /// Which discovery transports to use, timeouts, and transport-specific options.
    pub discovery: DiscoveryMethodsConfig,
    /// How to pick among multiple matching services (e.g. QoS-weighted ranking).
    pub service_selection: ServiceSelectionConfig,
}

impl DiscoveryConfig {
    /// Load configuration from TOML file
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be read or TOML parsing fails.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    /// Get primal info (self-knowledge)
    #[must_use]
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
    #[must_use]
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

/// Static identity and networking surface for this primal as embedded in config files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalSelfConfig {
    /// Stable logical name (e.g. `beardog`) used in logs and discovery keys.
    pub primal_id: String,
    /// High-level role label (e.g. `security`, `orchestration`) for policy and UX.
    pub primal_type: String,
    /// Semver or build string advertised to peers for compatibility checks.
    pub version: String,
    /// Human-facing title for dashboards and CLI output.
    pub display_name: String,
    /// Capability type strings this instance provides (maps to [`crate::types::Capability`]).
    pub self_capabilities: Vec<String>,
    /// Host, port, and URL layout clients should use to reach this primal.
    pub endpoint: EndpointConfig,
    /// Whether and how this primal publishes its presence on the network or via env/registry hooks.
    pub announcement: AnnouncementConfig,
}

/// Listen address and URL shape for this primal’s primary API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    /// Bind hostname or IP as configured (may be `0.0.0.0` for all interfaces).
    pub host: String,
    /// TCP (or QUIC) port exposed to clients.
    pub port: u16,
    /// URL scheme, typically `http` or `https`, used when building [`crate::types::ServiceEndpoint`].
    pub scheme: String,
    /// Path prefix prepended before API routes (e.g. `/api/v1`).
    pub path_prefix: String,
}

/// Controls periodic publication of this primal’s endpoint and capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementConfig {
    /// When false, [`crate::announcement::Announcer`] returns immediately without publishing.
    pub enabled: bool,
    /// Ordered list of announcement backends to try (`mdns`, `environment`, `service_registry`, …).
    pub methods: Vec<String>,
    /// Minimum seconds between re-announcements for long-lived registrations.
    pub announcement_interval_secs: u64,
    /// Time-to-live hint for registry or DNS-like records, in seconds.
    pub ttl_secs: u64,
    /// Optional mDNS-specific overrides (service name, `_tcp` type, enable flag).
    #[serde(default)]
    pub mdns: MdnsAnnouncementConfig,
}

/// Fine-grained mDNS TXT/SRV parameters when `"mdns"` is listed in [`AnnouncementConfig::methods`].
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MdnsAnnouncementConfig {
    /// When false, mDNS announcement is skipped even if the parent `methods` list includes it.
    pub enabled: bool,
    /// Instance label embedded in the mDNS name (often the primal id or hostname).
    pub service_name: String,
    /// DNS-SD service type including domain suffix (e.g. `_beardog._tcp.local.`).
    pub service_type: String,
}

/// Declares how strongly a dependency on another capability is required and how to degrade if missing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequiredCapabilityConfig {
    /// If true, startup or workflow may fail when no matching provider is found.
    pub required: bool,
    /// If true, discovery should prefer this capability when multiple candidates exist.
    pub preferred: bool,
    /// Optional minimum semantic version of the remote capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,
    /// Feature flags the consumer needs from the provider.
    pub features: Vec<String>,
    /// Named fallback strategy (`standalone`, `degraded`, etc.) interpreted by higher layers.
    pub fallback: String,
}

/// Discovery client behavior: which mechanisms to run, how often, and how long to cache results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryMethodsConfig {
    /// Discovery backends to use in order (`environment`, `mdns`, …).
    pub methods: Vec<String>,
    /// Upper bound in seconds for a single discovery round across methods.
    pub discovery_timeout_secs: u64,
    /// How often to re-run discovery while the process stays up.
    pub discovery_interval_secs: u64,
    /// How long to trust cached [`crate::types::DiscoveredService`] entries before refresh.
    pub cache_ttl_secs: u64,
    /// Environment-variable scanning patterns for capability endpoints.
    #[serde(default)]
    pub environment: EnvironmentDiscoveryConfig,
    /// mDNS browse/query settings for local network discovery.
    #[serde(default)]
    pub mdns: MdnsDiscoveryConfig,
}

/// Reads peer hints from process environment (e.g. `PRIMAL_*_ENDPOINT` variables).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentDiscoveryConfig {
    /// When false, environment scanning is disabled regardless of global `methods`.
    pub enabled: bool,
    /// `sprintf`-style or glob pattern for capability env keys.
    pub capability_pattern: String,
    /// Pattern for resolving primal-specific endpoint variables.
    pub primal_pattern: String,
}

/// Parameters for browsing mDNS/DNS-SD on the LAN.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MdnsDiscoveryConfig {
    /// When false, mDNS is not used even if listed in [`DiscoveryMethodsConfig::methods`].
    pub enabled: bool,
    /// Per-query timeout in seconds.
    pub timeout_secs: u64,
    /// Service types to browse (e.g. `_beardog._tcp.local.`).
    pub service_types: Vec<String>,
}

/// Names the selection algorithm and supplies weights for QoS-based ranking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSelectionConfig {
    /// Strategy identifier (`qos_based`, `first_match`, …) consumed by the selector implementation.
    pub strategy: String,
    /// Relative importance of latency, throughput, availability, and reliability when scoring peers.
    pub qos_weights: QoSWeightsConfig,
}

/// Serialized form of [`crate::types::QoSWeights`] for TOML configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QoSWeightsConfig {
    /// Weight for latency in `QoS` scoring.
    pub latency: f64,
    /// Weight for throughput in `QoS` scoring.
    pub throughput: f64,
    /// Weight for availability in `QoS` scoring.
    pub availability: f64,
    /// Weight for reliability in `QoS` scoring.
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

        let config: DiscoveryConfig =
            toml::from_str(config_toml).expect("embedded discovery TOML in test must parse");
        assert_eq!(config.primal_self.primal_id, "beardog");
        assert_eq!(config.primal_self.self_capabilities.len(), 2);
    }

    #[test]
    fn primal_info_http_scheme_disables_tls_on_endpoint() {
        let config_toml = r#"
[primal_self]
primal_id = "p"
primal_type = "t"
version = "1"
display_name = "P"
self_capabilities = ["a"]

[primal_self.endpoint]
host = "127.0.0.1"
port = 8080
scheme = "http"
path_prefix = "/api"

[primal_self.announcement]
enabled = false
methods = []
announcement_interval_secs = 60
ttl_secs = 300

[required_capabilities.k]
required = true
preferred = false
features = []
fallback = "none"

[discovery]
methods = ["environment"]
discovery_timeout_secs = 1
discovery_interval_secs = 300
cache_ttl_secs = 600

[service_selection]
strategy = "qos_based"

[service_selection.qos_weights]
latency = 0.25
throughput = 0.25
availability = 0.25
reliability = 0.25
"#;
        let config: DiscoveryConfig =
            toml::from_str(config_toml).expect("embedded primal_info TOML in test must parse");
        let info = config.primal_info();
        assert!(!info.endpoint.use_tls);
        assert!(info.endpoint.primary_url.starts_with("http://"));
        let req = config.required_capabilities();
        assert_eq!(req.len(), 1);
        assert_eq!(req[0].capability_type, "k");
        assert!(req[0].required);
    }

    #[test]
    fn test_load_repo_primal_capabilities_toml() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../configs/beardog-primal-capabilities.toml");
        let config = DiscoveryConfig::from_file(&path).unwrap_or_else(|e| {
            panic!("configs/beardog-primal-capabilities.toml must parse as DiscoveryConfig: {e}");
        });
        assert_eq!(config.primal_self.primal_id, "beardog");
        assert!(
            config
                .primal_self
                .announcement
                .mdns
                .service_type
                .ends_with("._tcp.local."),
            "mDNS type should follow ecosystem `._tcp.local.` suffix"
        );
    }
}
