// SPDX-License-Identifier: AGPL-3.0-or-later

//! Primal identity and configuration types.

use super::attestation::AuthRequirements;
use beardog_types::canonical::capabilities::ServiceCapabilityType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core capabilities that primals can provide in the ecosystem
///
/// Defines the functional categories of services that primals offer,
/// enabling capability-based service discovery and routing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PrimalCapability {
    /// Security services (encryption, authentication, HSM integration)
    Security,
    /// Storage services (distributed storage, caching, persistence)
    Storage,
    /// Compute services (processing, analysis, transformation)
    Compute,
    /// Networking services (routing, discovery, tunneling)
    Networking,
    /// AI/ML services (inference, training, optimization)
    AI,
    /// Represents custom variant
    Custom(String),
}

/// Configuration data for a primal service
///
/// Flexible key-value configuration storage using JSON values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalConfig {
    /// Configuration key-value pairs
    pub data: HashMap<String, serde_json::Value>,
}

/// Universal Integration Configuration
/// Replaces hardcoded primal integration flags with capability-based discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalIntegrationConfig {
    /// Enable capability-based service discovery
    /// Whether `enable_capability_discovery` is enabled
    pub enable_capability_discovery: bool,
    /// Collection of required capabilities
    pub required_capabilities: Vec<ServiceCapabilityType>,
    /// Optional capabilities that enhance functionality
    /// Collection of optional capabilities
    pub optional_capabilities: Vec<ServiceCapabilityType>,
    /// Collection of discovery endpoints
    pub discovery_endpoints: Vec<String>,
    /// Custom capability configurations
    pub custom_config: HashMap<String, serde_json::Value>,
    /// Fallback to environment-based discovery if explicit discovery fails
    /// Whether `enable_environment_discovery` is enabled
    pub enable_environment_discovery: bool,
}

impl Default for UniversalIntegrationConfig {
    fn default() -> Self {
        Self {
            enable_capability_discovery: true,
            required_capabilities: vec![
                ServiceCapabilityType::Security,
                ServiceCapabilityType::Storage,
            ],
            optional_capabilities: vec![
                ServiceCapabilityType::Compute,
                ServiceCapabilityType::ArtificialIntelligence,
                ServiceCapabilityType::Networking,
                ServiceCapabilityType::Orchestration,
            ],
            discovery_endpoints: vec![],
            custom_config: HashMap::new(),
            enable_environment_discovery: true,
        }
    }
}

/// Security configuration for service endpoints
///
/// Defines TLS requirements, certificate validation, cipher suites, and authentication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EndpointSecurityConfig {
    /// Whether TLS/HTTPS is required
    pub tls_required: bool,
    /// Whether to validate server certificates
    pub cert_validation: bool,
    /// Allowed TLS cipher suites
    pub allowed_ciphers: Vec<String>,
    /// Authentication requirements
    pub auth_requirements: AuthRequirements,
}
