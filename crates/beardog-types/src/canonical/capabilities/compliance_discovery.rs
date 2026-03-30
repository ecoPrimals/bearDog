// SPDX-License-Identifier: AGPL-3.0-only

//! Compliance, encryption policy, and discovery request/response payloads.

use serde::{Deserialize, Serialize};

use super::capability_type::CapabilityType;
use super::discovery::{SecurityLevel, UniversalCapability};

/// Compliance requirements and certifications
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComplianceCapabilities {
    /// Regulatory frameworks supported
    /// Collection of frameworks
    pub frameworks: Vec<String>,
    /// Compliance level achieved
    /// The compliance level value
    pub compliance_level: ComplianceLevel,
    /// Audit requirements
    /// Collection of audit requirements
    pub audit_requirements: Vec<String>,
    /// Data residency requirements
    pub data_residency: Vec<String>,
    /// Encryption requirements
    /// The encryption requirements value
    pub encryption_requirements: EncryptionRequirements,
}

/// Compliance level classification
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum ComplianceLevel {
    /// Basic compliance
    #[default]
    Basic,
    /// Standard compliance (SOC 2 Type I)
    Standard,
    /// High compliance (SOC 2 Type II, ISO 27001)
    High,
    /// Critical compliance (`FedRAMP`, FIPS 140-2)
    Critical,
}

/// Cryptographic policy constraints demanded by compliance or workload classification.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionRequirements {
    /// Minimum encryption strength in bits
    /// Number of `min_key_size`
    pub min_key_size: u32,
    /// Required encryption algorithms
    /// Collection of required algorithms
    pub required_algorithms: Vec<String>,
    /// Key management requirements
    /// Collection of key management
    pub key_management: Vec<String>,
    /// Data-at-rest encryption required
    /// Whether `data_at_rest` is enabled
    pub data_at_rest: bool,
    /// Data-in-transit encryption required
    /// Whether `data_in_transit` is enabled
    pub data_in_transit: bool,
}

/// Capability discovery request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDiscoveryRequest {
    /// Capabilities to discover
    /// Collection of capability types
    pub capability_types: Vec<CapabilityType>,
    /// Minimum security level required
    /// Optional min security level
    pub min_security_level: Option<SecurityLevel>,
    /// Maximum response time requirement (ms)
    pub max_response_time_ms: Option<u64>,
    /// Minimum success rate required (0.0 to 1.0)
    /// Optional min success rate
    pub min_success_rate: Option<f64>,
    /// Preferred regions
    /// Collection of preferred regions
    pub preferred_regions: Vec<String>,
    /// Required compliance levels
    /// Collection of required compliance
    pub required_compliance: Vec<ComplianceLevel>,
}

/// Capability discovery response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDiscoveryResponse {
    /// Discovered capabilities
    /// Collection of capabilities
    pub capabilities: Vec<UniversalCapability>,
    /// Discovery metadata
    /// The metadata value
    pub metadata: DiscoveryMetadata,
}

/// Discovery metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryMetadata {
    /// Discovery timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Discovery duration in milliseconds
    /// Number of `discovery_duration_ms`
    pub discovery_duration_ms: u64,
    /// Number of providers queried
    pub providers_queried: u32,
    /// Number of capabilities found
    /// Number of `capabilities_found`
    pub capabilities_found: u32,
}
