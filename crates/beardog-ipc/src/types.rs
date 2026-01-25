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
            Capability::Crypto => "crypto",
            Capability::BTSP => "btsp",
            Capability::Ed25519 => "ed25519",
            Capability::X25519 => "x25519",
            Capability::ChaCha20Poly1305 => "chacha20poly1305",
            Capability::AesGcm => "aesgcm",
            Capability::Storage => "storage",
            Capability::AI => "ai",
            Capability::Discovery => "discovery",
            Capability::Custom(s) => s,
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

fn default_true() -> bool {
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

        let json = serde_json::to_string(&cap).unwrap();
        assert_eq!(json, "\"crypto\"");
    }

    #[test]
    fn test_discovery_query_builder() {
        let query = DiscoveryQuery::capability(Capability::Crypto)
            .with_capability(Capability::Ed25519);

        assert_eq!(query.capabilities.len(), 2);
        assert!(query.capabilities.contains(&"crypto".to_string()));
        assert!(query.capabilities.contains(&"ed25519".to_string()));
    }
}

