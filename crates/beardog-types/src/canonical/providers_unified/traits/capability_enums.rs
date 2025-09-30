//! # Provider Capability Enumerations
//!
//! This module defines the capability enums and types used by the unified provider system.
//! It provides both enum-based capabilities for type safety and struct-based capabilities
//! for dynamic configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **Provider Capability Types** - Enum-based capabilities for type safety
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProviderCapabilityType {
    // **Security Capabilities**
    /// Key generation capability
    KeyGeneration,
    /// Data encryption capability
    Encryption,
    /// Digital signing capability
    Signing,
    /// Key derivation capability
    KeyDerivation,
    /// Random number generation
    RandomGeneration,
    /// Authentication capability
    Authentication,
    /// Authorization capability
    Authorization,
    
    // **Storage Capabilities**
    /// Data storage capability
    DataStorage,
    /// Data retrieval capability
    DataRetrieval,
    /// Data backup capability
    DataBackup,
    /// Data archival capability
    DataArchival,
    
    // **Network Capabilities**
    /// Network communication
    NetworkCommunication,
    /// Service discovery
    ServiceDiscovery,
    /// Load balancing
    LoadBalancing,
    /// Protocol handling
    ProtocolHandling,
    
    // **Monitoring Capabilities**
    /// Metrics collection
    MetricsCollection,
    /// Log aggregation
    LogAggregation,
    /// Alerting
    Alerting,
    /// Health monitoring
    HealthMonitoring,
    
    // **AI Capabilities**
    /// Machine learning inference
    MlInference,
    /// Model training
    ModelTraining,
    /// Data preprocessing
    DataPreprocessing,
    /// Feature extraction
    FeatureExtraction,
    
    // **Generic Capabilities**
    /// Configuration management
    ConfigurationManagement,
    /// Event handling
    EventHandling,
    /// Data transformation
    DataTransformation,
    /// Custom capability
    Custom(String),
}

/// **Provider Capability Description** - Struct-based capability for dynamic configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapability {
    /// Capability type
    pub capability_type: ProviderCapabilityType,
    /// Capability name
    pub name: String,
    /// Capability version
    pub version: String,
    /// Capability description
    pub description: String,
    /// Required configuration parameters
    pub required_config: Vec<String>,
    /// Optional configuration parameters
    pub optional_config: Vec<String>,
    /// Whether this capability is enabled
    pub enabled: bool,
    /// Capability-specific metadata
    pub metadata: HashMap<String, String>,
}

impl ProviderCapability {
    /// Create a new capability with the given type and basic information
    pub fn new(
        capability_type: ProviderCapabilityType,
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            capability_type,
            name: name.into(),
            version: version.into(),
            description: description.into(),
            required_config: Vec::new(),
            optional_config: Vec::new(),
            enabled: true,
            metadata: HashMap::new(),
        }
    }

    /// Create a key generation capability
    pub fn key_generation() -> Self {
        Self::new(
            ProviderCapabilityType::KeyGeneration,
            "Key Generation",
            "1.0.0",
            "Generate cryptographic keys",
        )
    }

    /// Create an encryption capability
    pub fn encryption() -> Self {
        Self::new(
            ProviderCapabilityType::Encryption,
            "Encryption",
            "1.0.0",
            "Encrypt and decrypt data",
        )
    }

    /// Create a signing capability
    pub fn signing() -> Self {
        Self::new(
            ProviderCapabilityType::Signing,
            "Digital Signing",
            "1.0.0",
            "Create and verify digital signatures",
        )
    }

    /// Create a custom capability
    pub fn custom(name: impl Into<String>, description: impl Into<String>) -> Self {
        let name_str = name.into();
        Self::new(
            ProviderCapabilityType::Custom(name_str.clone()),
            name_str,
            "1.0.0",
            description,
        )
    }
}

/// **Capability Set** - Collection of capabilities for a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySet {
    /// Set of capabilities
    pub capabilities: Vec<ProviderCapability>,
    /// Capability set version
    pub version: String,
    /// Capability set metadata
    pub metadata: HashMap<String, String>,
}

impl CapabilitySet {
    /// Create a new empty capability set
    pub fn new() -> Self {
        Self {
            capabilities: Vec::new(),
            version: "1.0.0".to_string(),
            metadata: HashMap::new(),
        }
    }

    /// Add a capability to the set
    pub fn add_capability(&mut self, capability: ProviderCapability) {
        self.capabilities.push(capability);
    }

    /// Check if the set contains a specific capability type
    pub fn has_capability(&self, capability_type: &ProviderCapabilityType) -> bool {
        self.capabilities
            .iter()
            .any(|cap| &cap.capability_type == capability_type && cap.enabled)
    }

    /// Get all capabilities of a specific type
    pub fn get_capabilities_by_type(
        &self,
        capability_type: &ProviderCapabilityType,
    ) -> Vec<&ProviderCapability> {
        self.capabilities
            .iter()
            .filter(|cap| &cap.capability_type == capability_type)
            .collect()
    }

    /// Create a capability set for HSM providers
    pub fn hsm_capabilities() -> Self {
        let mut set = Self::new();
        set.add_capability(ProviderCapability::key_generation());
        set.add_capability(ProviderCapability::encryption());
        set.add_capability(ProviderCapability::signing());
        set.metadata.insert("provider_type".to_string(), "hsm".to_string());
        set
    }

    /// Create a capability set for security providers
    pub fn security_capabilities() -> Self {
        let mut set = Self::new();
        set.add_capability(ProviderCapability::authentication());
        set.add_capability(ProviderCapability::authorization());
        set.add_capability(ProviderCapability::encryption());
        set.metadata.insert("provider_type".to_string(), "security".to_string());
        set
    }
}

impl Default for CapabilitySet {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderCapability {
    /// Create an authentication capability
    pub fn authentication() -> Self {
        Self::new(
            ProviderCapabilityType::Authentication,
            "Authentication",
            "1.0.0",
            "User and service authentication",
        )
    }

    /// Create an authorization capability
    pub fn authorization() -> Self {
        Self::new(
            ProviderCapabilityType::Authorization,
            "Authorization",
            "1.0.0",
            "Access control and authorization",
        )
    }
} 