// SPDX-License-Identifier: AGPL-3.0-only

//! Capability metadata structures
//!
//! Defines metadata for capability advertisement and discovery.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::VarError;

/// Capability metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityMetadata {
    /// Unique capability identifier (e.g., "secure_tunnel", "lineage_signing")
    pub id: String,

    /// Capability version (semantic versioning)
    pub version: String,

    /// Human-readable description
    pub description: String,

    /// Trait interface name (for type checking)
    pub interface: String,

    /// Endpoint URL for this capability
    pub endpoint: String,

    /// Supported protocols (e.g., ["http", "grpc", "native"])
    pub protocols: Vec<String>,

    /// Optional rate limit (requests per second)
    pub rate_limit: Option<u64>,

    /// Whether authentication is required
    pub requires_auth: bool,

    /// Additional metadata (extensible)
    pub extra: HashMap<String, String>,
}

impl CapabilityMetadata {
    /// Create new capability metadata
    pub fn new(id: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            version: version.into(),
            description: String::new(),
            interface: String::new(),
            endpoint: String::new(),
            protocols: vec!["http".to_string()],
            rate_limit: None,
            requires_auth: false,
            extra: HashMap::new(),
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Set interface name
    pub fn with_interface(mut self, interface: impl Into<String>) -> Self {
        self.interface = interface.into();
        self
    }

    /// Set endpoint
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    /// Set protocols
    pub fn with_protocols(mut self, protocols: Vec<String>) -> Self {
        self.protocols = protocols;
        self
    }

    /// Set rate limit
    pub const fn with_rate_limit(mut self, limit: u64) -> Self {
        self.rate_limit = Some(limit);
        self
    }

    /// Set authentication requirement
    pub const fn with_auth(mut self, requires_auth: bool) -> Self {
        self.requires_auth = requires_auth;
        self
    }
}

/// Capability advertisement (full primal advertisement)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityAdvertisement {
    /// Primal information
    pub primal: PrimalInfo,

    /// List of capabilities offered
    pub capabilities: Vec<CapabilityMetadata>,

    /// Discovery configuration
    pub discovery: DiscoveryConfig,
}

/// Primal information (self-knowledge only)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalInfo {
    /// Unique primal instance identifier (UUID)
    pub id: String,

    /// Primal type (e.g., "cryptographic_services", "storage", "compute")
    pub primal_type: String,

    /// Self-description
    pub description: String,

    /// Version
    pub version: String,
}

/// HTTP path segment for the capability manifest, appended to the resolved HTTP base.
///
/// Override with `BEARDOG_CAPABILITY_HTTP_PATH` (must start with `/` unless a single segment).
pub const DEFAULT_CAPABILITY_HTTP_PATH: &str = "/capabilities";

/// Default DNS-SD service type for capability advertisement (`_<svc>._tcp`).
///
/// Instance names are combined at runtime; override the full service type with
/// `BEARDOG_CAPABILITY_MDNS_SERVICE` if needed.
pub const DEFAULT_CAPABILITY_MDNS_SERVICE_TYPE: &str = "_beardog-cap._tcp";

/// Environment variable: full HTTP base for capability URLs (no trailing path), e.g. `http://127.0.0.1:8080`.
pub const ENV_CAPABILITY_HTTP_BASE: &str = "BEARDOG_CAPABILITY_HTTP_BASE";

/// Environment variable: HTTP path for capability manifest (default [`DEFAULT_CAPABILITY_HTTP_PATH`]).
pub const ENV_CAPABILITY_HTTP_PATH: &str = "BEARDOG_CAPABILITY_HTTP_PATH";

/// Environment variable: mDNS instance label for this provider (defaults to sovereign instance id).
pub const ENV_CAPABILITY_MDNS_INSTANCE: &str = "BEARDOG_CAPABILITY_MDNS_INSTANCE";

/// Environment variable: DNS-SD service type including `_tcp` suffix (default [`DEFAULT_CAPABILITY_MDNS_SERVICE_TYPE`]).
pub const ENV_CAPABILITY_MDNS_SERVICE: &str = "BEARDOG_CAPABILITY_MDNS_SERVICE";

/// Resolve HTTP base URL for capability advertisement (scheme + host + port, no capability path).
///
/// Precedence:
/// 1. [`ENV_CAPABILITY_HTTP_BASE`]
/// 2. `BEARDOG_API_HOST` or `BEARDOG_BIND_ADDRESS` + `BEARDOG_API_PORT` (or [`beardog_config::DEFAULT_API_PORT`])
pub fn resolve_capability_http_base() -> String {
    resolve_capability_http_base_from_env(|k| beardog_errors::process_env::var(k))
}

/// Same as [`resolve_capability_http_base`] but reads configuration via `get` (tests use a local map; production uses [`beardog_errors::process_env::var`]).
pub fn resolve_capability_http_base_from_env(
    mut get: impl FnMut(&str) -> Result<String, VarError>,
) -> String {
    if let Ok(base) = get(ENV_CAPABILITY_HTTP_BASE) {
        return base.trim_end_matches('/').to_string();
    }

    let host = get("BEARDOG_API_HOST")
        .or_else(|_| get("BEARDOG_BIND_ADDRESS"))
        .unwrap_or_else(|_| {
            beardog_config::domains::network_addresses::DEFAULT_BIND_ADDRESS.to_string()
        });

    let port: u16 = get("BEARDOG_API_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(beardog_config::DEFAULT_API_PORT);

    format!("http://{host}:{port}")
}

/// Full HTTP URL for the capability discovery document.
pub fn resolve_capability_http_discovery_url() -> String {
    resolve_capability_http_discovery_url_from_env(|k| beardog_errors::process_env::var(k))
}

/// Same as [`resolve_capability_http_discovery_url`] with injectable env lookup (parallel-test safe).
pub fn resolve_capability_http_discovery_url_from_env(
    mut get: impl FnMut(&str) -> Result<String, VarError>,
) -> String {
    let path =
        get(ENV_CAPABILITY_HTTP_PATH).unwrap_or_else(|_| DEFAULT_CAPABILITY_HTTP_PATH.to_string());
    let base = resolve_capability_http_base_from_env(&mut get);
    if path.starts_with('/') {
        format!("{base}{path}")
    } else {
        format!("{base}/{path}")
    }
}

fn resolve_mdns_instance_label_from_env<F>(instance_id: &str, get: &mut F) -> String
where
    F: FnMut(&str) -> Result<String, VarError>,
{
    get(ENV_CAPABILITY_MDNS_INSTANCE).unwrap_or_else(|_| instance_id.to_string())
}

fn resolve_mdns_service_type_from_env<F>(get: &mut F) -> String
where
    F: FnMut(&str) -> Result<String, VarError>,
{
    get(ENV_CAPABILITY_MDNS_SERVICE)
        .unwrap_or_else(|_| DEFAULT_CAPABILITY_MDNS_SERVICE_TYPE.to_string())
}

/// DNS-SD-style name for capability advertisement: `<instance>.<svc>._tcp.local`.
///
/// Uses sovereign `instance_id` only as the default instance label; operators may override via
/// [`ENV_CAPABILITY_MDNS_INSTANCE`]. This is discovery metadata, not a coupled primal name.
pub fn resolve_capability_mdns_full_name(instance_id: &str) -> String {
    resolve_capability_mdns_full_name_from_env(instance_id, |k| beardog_errors::process_env::var(k))
}

/// Same as [`resolve_capability_mdns_full_name`] with injectable env lookup (parallel-test safe).
pub fn resolve_capability_mdns_full_name_from_env(
    instance_id: &str,
    mut get: impl FnMut(&str) -> Result<String, VarError>,
) -> String {
    let instance = resolve_mdns_instance_label_from_env(instance_id, &mut get);
    let svc = resolve_mdns_service_type_from_env(&mut get);
    format!("{instance}.{svc}.local")
}

/// Default discovery TTL (seconds). Override with runtime policy when integrating.
pub const DEFAULT_DISCOVERY_TTL_SECS: u64 = 300;

/// Discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// mDNS advertisement name (DNS-SD instance + service, e.g. `uuid._beardog-cap._tcp.local`)
    pub mdns: Option<String>,

    /// HTTP endpoint for capability discovery
    pub http: String,

    /// TTL for discovery caching (seconds)
    pub ttl: u64,
}

/// Capability endpoint (discovered capability)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityEndpoint {
    /// Capability metadata
    pub metadata: CapabilityMetadata,

    /// Provider primal information
    pub provider: PrimalInfo,

    /// Endpoint URL
    pub url: String,

    /// Discovery timestamp (ISO 8601)
    pub discovered_at: String,
}

impl CapabilityEndpoint {
    /// Create new capability endpoint
    pub fn new(metadata: CapabilityMetadata, provider: PrimalInfo, url: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            metadata,
            provider,
            url,
            discovered_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env::VarError;

    use super::*;

    /// Fixture URLs for unit tests only.
    const TEST_HTTP_BASE: &str = "http://127.0.0.1:54321";
    const TEST_HTTP_ALT: &str = "http://127.0.0.1:54322";

    #[test]
    fn test_capability_metadata_builder() {
        let metadata = CapabilityMetadata::new("secure_tunnel", "1.0")
            .with_description("Secure tunnel capability")
            .with_interface("SecureTunnelProvider")
            .with_endpoint(format!("{TEST_HTTP_BASE}/capabilities/secure_tunnel"))
            .with_rate_limit(1000);

        assert_eq!(metadata.id, "secure_tunnel");
        assert_eq!(metadata.version, "1.0");
        assert_eq!(metadata.rate_limit, Some(1000));
    }

    #[test]
    fn test_capability_metadata_all_builders() {
        let metadata = CapabilityMetadata::new("lineage_signing", "2.0")
            .with_description("Lineage signing capability")
            .with_interface("LineageSigningProvider")
            .with_endpoint(format!("{TEST_HTTP_ALT}/sign"))
            .with_protocols(vec!["http".to_string(), "grpc".to_string()])
            .with_rate_limit(500)
            .with_auth(true);

        assert_eq!(metadata.id, "lineage_signing");
        assert_eq!(metadata.version, "2.0");
        assert_eq!(metadata.description, "Lineage signing capability");
        assert_eq!(metadata.interface, "LineageSigningProvider");
        assert_eq!(metadata.endpoint, format!("{TEST_HTTP_ALT}/sign"));
        assert_eq!(metadata.protocols, vec!["http", "grpc"]);
        assert_eq!(metadata.rate_limit, Some(500));
        assert!(metadata.requires_auth);
    }

    #[test]
    fn test_capability_metadata_defaults() {
        let metadata = CapabilityMetadata::new("test_cap", "0.1");

        assert_eq!(metadata.id, "test_cap");
        assert_eq!(metadata.version, "0.1");
        assert!(metadata.description.is_empty());
        assert!(metadata.interface.is_empty());
        assert!(metadata.endpoint.is_empty());
        assert_eq!(metadata.protocols, vec!["http"]);
        assert_eq!(metadata.rate_limit, None);
        assert!(!metadata.requires_auth);
        assert!(metadata.extra.is_empty());
    }

    #[test]
    fn test_capability_advertisement() {
        let primal = PrimalInfo {
            id: "d0000000-0000-4000-8000-000000000010".to_string(),
            primal_type: "cryptographic_services".to_string(),
            description: "Test capability provider".to_string(),
            version: "0.1.0".to_string(),
        };

        let discovery = DiscoveryConfig {
            mdns: Some(format!(
                "d0000000-0000-4000-8000-000000000010.{DEFAULT_CAPABILITY_MDNS_SERVICE_TYPE}.local"
            )),
            http: format!("{TEST_HTTP_BASE}/capabilities"),
            ttl: DEFAULT_DISCOVERY_TTL_SECS,
        };

        let advertisement = CapabilityAdvertisement {
            primal,
            capabilities: vec![],
            discovery,
        };

        assert_eq!(
            advertisement.primal.id,
            "d0000000-0000-4000-8000-000000000010"
        );
        assert_eq!(advertisement.discovery.ttl, DEFAULT_DISCOVERY_TTL_SECS);
    }

    #[test]
    fn test_capability_endpoint_creation() {
        let tunnel_url = format!("{TEST_HTTP_BASE}/tunnel");
        let metadata = CapabilityMetadata::new("tunnel", "1.0").with_endpoint(&tunnel_url);

        let primal = PrimalInfo {
            id: "e0000000-0000-4000-8000-000000000020".to_string(),
            primal_type: "network".to_string(),
            description: "Network capability profile".to_string(),
            version: "1.0.0".to_string(),
        };

        let endpoint = CapabilityEndpoint::new(metadata, primal, tunnel_url.clone());

        assert_eq!(endpoint.metadata.id, "tunnel");
        assert_eq!(endpoint.provider.id, "e0000000-0000-4000-8000-000000000020");
        assert_eq!(endpoint.url, tunnel_url);
        assert!(!endpoint.discovered_at.is_empty());
    }

    #[test]
    fn test_primal_info_serialization() {
        let primal = PrimalInfo {
            id: "f0000000-0000-4000-8000-000000000030".to_string(),
            primal_type: "storage".to_string(),
            description: "Storage capability profile".to_string(),
            version: "2.0.0".to_string(),
        };

        let json = serde_json::to_string(&primal).expect("PrimalInfo serializes");
        let deserialized: PrimalInfo = serde_json::from_str(&json).expect("PrimalInfo roundtrips");

        assert_eq!(deserialized.id, primal.id);
        assert_eq!(deserialized.primal_type, primal.primal_type);
        assert_eq!(deserialized.description, primal.description);
        assert_eq!(deserialized.version, primal.version);
    }

    #[test]
    fn test_discovery_config_without_mdns() {
        let discovery = DiscoveryConfig {
            mdns: None,
            http: format!("{TEST_HTTP_BASE}/discover"),
            ttl: 600,
        };

        assert!(discovery.mdns.is_none());
        assert_eq!(discovery.http, format!("{TEST_HTTP_BASE}/discover"));
        assert_eq!(discovery.ttl, 600);
    }

    #[test]
    fn test_capability_metadata_with_extra() {
        let mut metadata = CapabilityMetadata::new("custom", "1.0");
        metadata
            .extra
            .insert("custom_key".to_string(), "custom_value".to_string());
        metadata
            .extra
            .insert("another_key".to_string(), "another_value".to_string());

        assert_eq!(metadata.extra.len(), 2);
        assert_eq!(
            metadata.extra.get("custom_key"),
            Some(&"custom_value".to_string())
        );
    }

    fn get_from_map(m: &HashMap<String, String>, key: &str) -> Result<String, VarError> {
        m.get(key).cloned().ok_or(VarError::NotPresent)
    }

    #[test]
    fn test_resolve_capability_http_base_from_env_override() {
        let m = HashMap::from([(
            ENV_CAPABILITY_HTTP_BASE.to_string(),
            "http://caps.example:9443/".to_string(),
        )]);
        let base = resolve_capability_http_base_from_env(|k| get_from_map(&m, k));
        assert_eq!(base, "http://caps.example:9443");
    }

    #[test]
    fn test_resolve_capability_http_base_from_host_and_port() {
        let m = HashMap::from([
            ("BEARDOG_API_HOST".to_string(), "192.0.2.10".to_string()),
            ("BEARDOG_API_PORT".to_string(), "65000".to_string()),
        ]);
        let base = resolve_capability_http_base_from_env(|k| get_from_map(&m, k));
        assert_eq!(base, "http://192.0.2.10:65000");
    }

    #[test]
    fn test_resolve_capability_http_discovery_url_paths() {
        let m_default_path = HashMap::from([
            ("BEARDOG_API_HOST".to_string(), "127.0.0.1".to_string()),
            ("BEARDOG_API_PORT".to_string(), "1".to_string()),
        ]);
        let default_path =
            resolve_capability_http_discovery_url_from_env(|k| get_from_map(&m_default_path, k));
        assert!(default_path.ends_with(DEFAULT_CAPABILITY_HTTP_PATH));

        let m_abs = HashMap::from([
            ("BEARDOG_API_HOST".to_string(), "127.0.0.1".to_string()),
            ("BEARDOG_API_PORT".to_string(), "1".to_string()),
            (
                ENV_CAPABILITY_HTTP_PATH.to_string(),
                "/v2/capabilities".to_string(),
            ),
        ]);
        let abs = resolve_capability_http_discovery_url_from_env(|k| get_from_map(&m_abs, k));
        assert!(abs.contains("/v2/capabilities"));
        assert!(!abs.ends_with("//v2/capabilities"));

        let m_rel = HashMap::from([
            ("BEARDOG_API_HOST".to_string(), "127.0.0.1".to_string()),
            ("BEARDOG_API_PORT".to_string(), "1".to_string()),
            (
                ENV_CAPABILITY_HTTP_PATH.to_string(),
                "manifest.json".to_string(),
            ),
        ]);
        let rel = resolve_capability_http_discovery_url_from_env(|k| get_from_map(&m_rel, k));
        assert!(rel.ends_with("/manifest.json"));
    }

    #[test]
    fn test_resolve_capability_mdns_full_name_defaults_and_overrides() {
        let m_empty = HashMap::<String, String>::new();
        let def = resolve_capability_mdns_full_name_from_env("sovereign-1", |k| {
            get_from_map(&m_empty, k)
        });
        assert!(def.contains("sovereign-1"));
        assert!(
            std::path::Path::new(&def)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("local"))
        );
        assert!(def.contains("_beardog-cap._tcp"));

        let m_custom = HashMap::from([
            (
                ENV_CAPABILITY_MDNS_INSTANCE.to_string(),
                "my-instance".to_string(),
            ),
            (
                ENV_CAPABILITY_MDNS_SERVICE.to_string(),
                "_custom._tcp".to_string(),
            ),
        ]);
        let custom =
            resolve_capability_mdns_full_name_from_env("ignored", |k| get_from_map(&m_custom, k));
        assert_eq!(custom, "my-instance._custom._tcp.local");
    }

    #[test]
    fn test_resolve_capability_http_base_uses_bind_address_when_api_host_unset() {
        let m = HashMap::from([
            (
                "BEARDOG_BIND_ADDRESS".to_string(),
                "198.51.100.7".to_string(),
            ),
            ("BEARDOG_API_PORT".to_string(), "9090".to_string()),
        ]);
        let base = resolve_capability_http_base_from_env(|k| get_from_map(&m, k));
        assert!(
            base.contains("198.51.100.7"),
            "expected bind address in base URL, got: {base}"
        );
    }

    #[test]
    fn test_resolve_capability_http_base_invalid_api_port_falls_back_to_default() {
        let m = HashMap::from([
            ("BEARDOG_API_HOST".to_string(), "127.0.0.1".to_string()),
            ("BEARDOG_API_PORT".to_string(), "not-a-port".to_string()),
        ]);
        let base = resolve_capability_http_base_from_env(|k| get_from_map(&m, k));
        assert_eq!(
            base,
            format!("http://127.0.0.1:{}", beardog_config::DEFAULT_API_PORT)
        );
    }

    #[test]
    fn test_capability_metadata_serde_roundtrip_with_extra() {
        let mut meta = CapabilityMetadata::new("cap_x", "1.2.3")
            .with_description("desc")
            .with_interface("Iface")
            .with_endpoint("http://example/cap")
            .with_protocols(vec!["native".to_string()])
            .with_rate_limit(42)
            .with_auth(true);
        meta.extra.insert("k".to_string(), "v".to_string());

        let json = serde_json::to_string(&meta).expect("CapabilityMetadata serializes");
        let back: CapabilityMetadata =
            serde_json::from_str(&json).expect("CapabilityMetadata roundtrips");
        assert_eq!(back.id, "cap_x");
        assert_eq!(back.version, "1.2.3");
        assert_eq!(back.extra.get("k"), Some(&"v".to_string()));
        assert!(back.requires_auth);
    }

    #[test]
    fn test_capability_advertisement_serde_roundtrip() {
        let primal = PrimalInfo {
            id: "id-1".to_string(),
            primal_type: "t".to_string(),
            description: "d".to_string(),
            version: "1.0.0".to_string(),
        };
        let discovery = DiscoveryConfig {
            mdns: None,
            http: "http://h/c".to_string(),
            ttl: 1,
        };
        let ad = CapabilityAdvertisement {
            primal,
            capabilities: vec![CapabilityMetadata::new("c", "1").with_interface("I")],
            discovery,
        };
        let json = serde_json::to_string(&ad).expect("CapabilityAdvertisement serializes");
        let back: CapabilityAdvertisement =
            serde_json::from_str(&json).expect("CapabilityAdvertisement roundtrips");
        assert_eq!(back.capabilities.len(), 1);
        assert_eq!(back.discovery.http, "http://h/c");
    }

    #[test]
    fn test_capability_metadata_new_defaults_and_rate_limit_auth() {
        let m = CapabilityMetadata::new("id", "v1");
        assert_eq!(m.protocols, vec!["http".to_string()]);
        assert!(!m.requires_auth);
        let m2 = m.with_rate_limit(9).with_auth(true);
        assert_eq!(m2.rate_limit, Some(9));
        assert!(m2.requires_auth);
    }
}
