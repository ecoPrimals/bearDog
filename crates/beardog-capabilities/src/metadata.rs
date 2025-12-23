//! Capability metadata structures
//!
//! Defines metadata for capability advertisement and discovery.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub fn with_rate_limit(mut self, limit: u64) -> Self {
        self.rate_limit = Some(limit);
        self
    }

    /// Set authentication requirement
    pub fn with_auth(mut self, requires_auth: bool) -> Self {
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

/// Discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// mDNS service name (e.g., "_beardog_capabilities._tcp.local")
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
    use super::*;

    #[test]
    fn test_capability_metadata_builder() {
        let metadata = CapabilityMetadata::new("secure_tunnel", "1.0")
            .with_description("Secure tunnel capability")
            .with_interface("SecureTunnelProvider")
            .with_endpoint("http://localhost:8080/capabilities/secure_tunnel")
            .with_rate_limit(1000);

        assert_eq!(metadata.id, "secure_tunnel");
        assert_eq!(metadata.version, "1.0");
        assert_eq!(metadata.rate_limit, Some(1000));
    }

    #[test]
    fn test_capability_advertisement() {
        let primal = PrimalInfo {
            id: "beardog-test-1".to_string(),
            primal_type: "cryptographic_services".to_string(),
            description: "Test BearDog instance".to_string(),
            version: "0.1.0".to_string(),
        };

        let discovery = DiscoveryConfig {
            mdns: Some("_beardog._tcp.local".to_string()),
            http: "http://localhost:8080/capabilities".to_string(),
            ttl: 300,
        };

        let advertisement = CapabilityAdvertisement {
            primal,
            capabilities: vec![],
            discovery,
        };

        assert_eq!(advertisement.primal.id, "beardog-test-1");
        assert_eq!(advertisement.discovery.ttl, 300);
    }
}
