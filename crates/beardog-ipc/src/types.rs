// SPDX-License-Identifier: AGPL-3.0-only

//! Type definitions for Primal IPC Protocol

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Service capability types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Capability {
    /// Cryptographic operations
    Crypto,
    /// BTSP tunnel protocol
    BTSP,
    /// Ed25519 signatures
    Ed25519,
    /// X25519 key exchange
    X25519,
    /// ChaCha20-Poly1305 encryption
    ChaCha20Poly1305,
    /// AES-GCM encryption
    AesGcm,
    /// Storage operations
    Storage,
    /// AI/ML operations
    AI,
    /// Discovery services
    Discovery,
    /// Custom capability
    Custom(String),
}

impl Capability {
    /// Convert capability to string representation
    pub fn as_str(&self) -> &str {
        match self {
            Self::Crypto => "crypto",
            Self::BTSP => "btsp",
            Self::Ed25519 => "ed25519",
            Self::X25519 => "x25519",
            Self::ChaCha20Poly1305 => "chacha20poly1305",
            Self::AesGcm => "aesgcm",
            Self::Storage => "storage",
            Self::AI => "ai",
            Self::Discovery => "discovery",
            Self::Custom(s) => s,
        }
    }
}

/// Information about a discovered service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service name (primal name)
    pub name: String,
    /// Service endpoint (socket path or URL)
    pub endpoint: String,
    /// Capabilities provided by this service
    pub capabilities: Vec<String>,
    /// Service version
    pub version: String,
    /// Whether service is currently available
    #[serde(default = "default_true")]
    pub available: bool,
    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

const fn default_true() -> bool {
    true
}

/// Query for discovering services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryQuery {
    /// Optional specific primal name
    pub primal: Option<String>,
    /// Required capabilities
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// Additional filters
    #[serde(default)]
    pub filters: HashMap<String, serde_json::Value>,
}

impl DiscoveryQuery {
    /// Create a new discovery query
    pub fn new() -> Self {
        Self {
            primal: None,
            capabilities: Vec::new(),
            filters: HashMap::new(),
        }
    }

    /// Query for a specific primal
    pub fn primal(name: &str) -> Self {
        Self {
            primal: Some(name.to_string()),
            capabilities: Vec::new(),
            filters: HashMap::new(),
        }
    }

    /// Query by capability
    pub fn capability(cap: Capability) -> Self {
        Self {
            primal: None,
            capabilities: vec![cap.as_str().to_string()],
            filters: HashMap::new(),
        }
    }

    /// Add a capability requirement
    pub fn with_capability(mut self, cap: Capability) -> Self {
        self.capabilities.push(cap.as_str().to_string());
        self
    }

    /// Add a filter
    pub fn with_filter(mut self, key: String, value: serde_json::Value) -> Self {
        self.filters.insert(key, value);
        self
    }
}

impl Default for DiscoveryQuery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_serialization() {
        let cap = Capability::Crypto;
        assert_eq!(cap.as_str(), "crypto");

        let json = serde_json::to_string(&cap).expect("serialize Capability");
        assert_eq!(json, "\"crypto\"");
    }

    #[test]
    fn test_capability_all_as_str() {
        assert_eq!(Capability::Crypto.as_str(), "crypto");
        assert_eq!(Capability::BTSP.as_str(), "btsp");
        assert_eq!(Capability::Ed25519.as_str(), "ed25519");
        assert_eq!(Capability::X25519.as_str(), "x25519");
        assert_eq!(Capability::ChaCha20Poly1305.as_str(), "chacha20poly1305");
        assert_eq!(Capability::AesGcm.as_str(), "aesgcm");
        assert_eq!(Capability::Storage.as_str(), "storage");
        assert_eq!(Capability::AI.as_str(), "ai");
        assert_eq!(Capability::Discovery.as_str(), "discovery");
        assert_eq!(Capability::Custom("mycap".to_string()).as_str(), "mycap");
    }

    #[test]
    fn test_capability_roundtrip() {
        for cap in [
            Capability::Crypto,
            Capability::BTSP,
            Capability::Custom("custom".to_string()),
        ] {
            let json = serde_json::to_string(&cap).expect("serialize Capability");
            let restored: Capability = serde_json::from_str(&json).expect("deserialize Capability");
            assert_eq!(cap, restored);
        }
    }

    #[test]
    fn test_discovery_query_builder() {
        let query =
            DiscoveryQuery::capability(Capability::Crypto).with_capability(Capability::Ed25519);

        assert_eq!(query.capabilities.len(), 2);
        assert!(query.capabilities.contains(&"crypto".to_string()));
        assert!(query.capabilities.contains(&"ed25519".to_string()));
    }

    #[test]
    fn test_discovery_query_new() {
        let q = DiscoveryQuery::new();
        assert!(q.primal.is_none());
        assert!(q.capabilities.is_empty());
        assert!(q.filters.is_empty());
    }

    #[test]
    fn test_discovery_query_primal() {
        let q = DiscoveryQuery::primal("beardog");
        assert_eq!(q.primal.as_deref(), Some("beardog"));
    }

    #[test]
    fn test_discovery_query_with_filter() {
        let q =
            DiscoveryQuery::new().with_filter("region".to_string(), serde_json::json!("us-east"));
        assert_eq!(q.filters.len(), 1);
    }

    #[test]
    fn test_discovery_query_default() {
        let q = DiscoveryQuery::default();
        assert!(q.primal.is_none());
    }

    #[test]
    fn test_service_info_serialization() {
        let info = ServiceInfo {
            name: "beardog".to_string(),
            endpoint: "/primal/beardog".to_string(),
            capabilities: vec!["crypto".to_string()],
            version: "1.0".to_string(),
            available: true,
            metadata: HashMap::new(),
        };
        let json = serde_json::to_string(&info).expect("serialize ServiceInfo");
        let restored: ServiceInfo = serde_json::from_str(&json).expect("deserialize ServiceInfo");
        assert_eq!(info.name, restored.name);
        assert!(restored.available);
    }

    #[test]
    fn test_service_info_default_available() {
        let json = r#"{"name":"x","endpoint":"/x","capabilities":[],"version":"1.0"}"#;
        let info: ServiceInfo = serde_json::from_str(json).expect("deserialize ServiceInfo");
        assert!(info.available);
    }

    #[test]
    fn test_service_info_with_metadata() {
        let mut meta = HashMap::new();
        meta.insert("desc".to_string(), serde_json::json!("test"));
        let info = ServiceInfo {
            name: "x".to_string(),
            endpoint: "/x".to_string(),
            capabilities: vec![],
            version: "1.0".to_string(),
            available: false,
            metadata: meta,
        };
        let json = serde_json::to_string(&info).expect("serialize ServiceInfo");
        assert!(json.contains("desc"));
    }

    #[test]
    fn test_default_true() {
        let json = r#"{"name":"x","endpoint":"/x","capabilities":[],"version":"1.0"}"#;
        let info: ServiceInfo = serde_json::from_str(json).expect("deserialize ServiceInfo");
        assert!(info.available);
    }
}
