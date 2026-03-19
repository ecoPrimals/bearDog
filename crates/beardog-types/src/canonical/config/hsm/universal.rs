// SPDX-License-Identifier: AGPL-3.0-only

// Universal HSM Configuration
//
// This module provides universal HSM configuration patterns that replace
// cloud-specific configurations with capability-based discovery.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal HSM configuration (replaces cloud-specific configurations)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalHsmConfig {
    /// Enable universal HSM capability discovery
    /// Whether enable_discovery is enabled
    pub enable_discovery: bool,

    /// Capability-based HSM configuration
    pub capability_config: HsmCapabilityConfig,

    /// Legacy cloud configurations (deprecated)
    #[deprecated(note = "Use capability_config instead")]
    /// Optional legacy cloud
    pub legacy_cloud: Option<LegacyCloudConfig>,
}

/// HSM capability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmCapabilityConfig {
    /// Required HSM capabilities
    /// Collection of required capabilities
    pub required_capabilities: Vec<String>,

    /// Preferred HSM types
    /// Collection of preferred types
    pub preferred_types: Vec<String>,

    /// Security requirements
    /// The security requirements value
    pub security_requirements: SecurityRequirements,

    pub performance_requirements: PerformanceRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    /// Minimum security level required
    /// The min security level value
    pub min_security_level: String,

    /// Required compliance certifications
    /// Collection of required certifications
    pub required_certifications: Vec<String>,

    /// Hardware-backed requirement
    /// Whether require_hardware_backed is enabled
    pub require_hardware_backed: bool,

    /// Attestation requirements
    /// Whether require_attestation is enabled
    pub require_attestation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {
    /// Maximum acceptable latency (ms)
    /// Number of max_latency_ms
    pub max_latency_ms: u64,

    /// Minimum throughput (operations per second)
    /// Number of min_throughput_ops
    pub min_throughput_ops: u64,

    /// Availability requirement (percentage)
    /// The min availability percent value
    pub min_availability_percent: f64,
}

/// Legacy cloud configuration (deprecated)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(note = "Use capability-based configuration instead")]
pub struct LegacyCloudConfig {
    /// Legacy cloud provider flags
    pub provider_flags: HashMap<String, bool>,

    /// Migration assistance
    /// Whether migration is enabled
    pub migration_enabled: bool,
}

impl Default for UniversalHsmConfig {
    fn default() -> Self {
        Self {
            enable_discovery: true,
            capability_config: HsmCapabilityConfig::default(),
            legacy_cloud: None,
        }
    }
}

impl Default for HsmCapabilityConfig {
    fn default() -> Self {
        Self {
            required_capabilities: vec![
                "KeyManagement".to_string(),
                "HardwareSecurityModule".to_string(),
            ],
            preferred_types: vec!["hardware_backed".to_string(), "cloud_native".to_string()],
            security_requirements: SecurityRequirements::default(),
            performance_requirements: PerformanceRequirements::default(),
        }
    }
}

impl Default for SecurityRequirements {
    fn default() -> Self {
        Self {
            min_security_level: "high".to_string(),
            required_certifications: vec!["FIPS_140_2".to_string(), "Common_Criteria".to_string()],
            require_hardware_backed: true,
            require_attestation: true,
        }
    }
}

impl Default for PerformanceRequirements {
    fn default() -> Self {
        Self {
            max_latency_ms: 100,
            min_throughput_ops: 1000,
            min_availability_percent: 99.9,
        }
    }
}

impl UniversalHsmConfig {
    /// Create a new universal HSM configuration
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    pub fn high_security() -> Self {
        Self {
            enable_discovery: true,
            capability_config: HsmCapabilityConfig {
                required_capabilities: vec![
                    "KeyManagement".to_string(),
                    "HardwareSecurityModule".to_string(),
                    "QuantumCrypto".to_string(),
                ],
                preferred_types: vec![
                    "hardware_backed".to_string(),
                    "quantum_resistant".to_string(),
                ],
                security_requirements: SecurityRequirements {
                    min_security_level: "critical".to_string(),
                    required_certifications: vec![
                        "FIPS_140_2_Level_3".to_string(),
                        "Common_Criteria_EAL_4".to_string(),
                        "FedRAMP_High".to_string(),
                    ],
                    require_hardware_backed: true,
                    require_attestation: true,
                },
                performance_requirements: PerformanceRequirements {
                    max_latency_ms: 50,
                    min_throughput_ops: 5000,
                    min_availability_percent: 99.99,
                },
            },
            legacy_cloud: None,
        }
    }

    pub fn development() -> Self {
        Self {
            enable_discovery: true,
            capability_config: HsmCapabilityConfig {
                required_capabilities: vec!["KeyManagement".to_string()],
                preferred_types: vec!["software_backed".to_string(), "development".to_string()],
                security_requirements: SecurityRequirements {
                    min_security_level: "basic".to_string(),
                    required_certifications: vec![],
                    require_hardware_backed: false,
                    require_attestation: false,
                },
                performance_requirements: PerformanceRequirements {
                    max_latency_ms: 1000,
                    min_throughput_ops: 100,
                    min_availability_percent: 95.0,
                },
            },
            legacy_cloud: None,
        }
    }

    /// Check if this configuration is compatible with a capability
    /// Checks if compatible with
    /// Checks if compatible with
    pub fn is_compatible_with(&self, capability_type: &str) -> bool {
        self.capability_config
            .required_capabilities
            .contains(&capability_type.to_string())
    }

    /// Get security level requirement
    /// Gets security_level
    /// Gets security_level
    pub fn get_security_level(&self) -> &str {
        &self
            .capability_config
            .security_requirements
            .min_security_level
    }

    pub fn get_performance_requirements(&self) -> &PerformanceRequirements {
        &self.capability_config.performance_requirements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_hsm_config_creation() {
        let config = UniversalHsmConfig::new();
        assert!(config.enable_discovery);
        assert!(!config.capability_config.required_capabilities.is_empty());
    }

    #[test]
    fn test_high_security_config() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let config = UniversalHsmConfig::high_security();
        assert_eq!(
            config
                .capability_config
                // TEST_CATEGORY: unit
                // TEST_DOMAIN: types
                // TEST_PRIORITY: normal
                .security_requirements
                .min_security_level,
            "critical"
        );
        assert!(
            config
                .capability_config
                .security_requirements
                .require_hardware_backed
        );
        assert!(
            config
                .capability_config
                .performance_requirements
                .max_latency_ms
                <= 50
        );
    }

    #[test]
    fn test_development_config() {
        let config = UniversalHsmConfig::development();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(
            config
                .capability_config
                .security_requirements
                .min_security_level,
            "basic"
        );
        assert!(
            !config
                .capability_config
                .security_requirements
                .require_hardware_backed
        );
        assert!(
            config
                .capability_config
                .performance_requirements
                .max_latency_ms
                >= 1000
        );
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_capability_compatibility() {
        let config = UniversalHsmConfig::default();
        assert!(config.is_compatible_with("KeyManagement"));
        assert!(config.is_compatible_with("HardwareSecurityModule"));
        assert!(!config.is_compatible_with("UnknownCapability"));
    }
}
