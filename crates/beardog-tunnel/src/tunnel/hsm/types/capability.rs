//! HSM Capability Types
//!
//! Type definitions for HSM capability requirements and feature support.

use serde::{Deserialize, Serialize};

// Re-export from parent module for convenience
pub use super::HsmCapabilities;

/// HSM capability enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmCapability {
    /// Key generation capability
    KeyGeneration,
    /// Signing capability
    Signing,
    /// Encryption capability
    Encryption,
    /// Decryption capability
    Decryption,
    /// Key derivation capability
    KeyDerivation,
    /// Attestation capability
    Attestation,
    /// Hardware-backed storage
    HardwareStorage,
    /// Biometric authentication
    BiometricAuth,
}

/// HSM capability requirements
#[derive(Debug, Clone)]
pub struct CapabilityRequirements {
    /// Minimum security level required
    pub min_security_level: String,
    /// Key management capabilities
    pub key_management: KeyManagementCapabilities,
    /// Advanced feature capabilities
    pub advanced_features: AdvancedFeatureCapabilities,
    /// Performance capabilities
    pub performance: PerformanceCapabilities,
    /// Security capabilities
    pub security: SecurityCapabilities,
    /// Human entropy capabilities
    pub human_entropy: HumanEntropyCapabilities,
    /// API support capabilities
    pub api_support: ApiSupportCapabilities,
    /// Compliance capabilities
    pub compliance: ComplianceCapabilities,
    /// Whether hardware is required
    pub hardware_required: bool,
    /// Whether attestation is required
    pub attestation_required: bool,
}

impl Default for CapabilityRequirements {
    fn default() -> Self {
        Self {
            min_security_level: "software".to_string(),
            key_management: KeyManagementCapabilities::default(),
            advanced_features: AdvancedFeatureCapabilities::default(),
            performance: PerformanceCapabilities::default(),
            security: SecurityCapabilities::default(),
            human_entropy: HumanEntropyCapabilities::default(),
            api_support: ApiSupportCapabilities::default(),
            compliance: ComplianceCapabilities::default(),
            hardware_required: false,
            attestation_required: false,
        }
    }
}

/// Key management capabilities
#[derive(Debug, Clone)]
pub struct KeyManagementCapabilities {
    /// Maximum number of keys supported
    pub max_keys: Option<u64>,
    /// Supported key types
    pub key_types: Vec<String>,
    /// Key rotation support
    pub rotation_supported: bool,
    /// Key derivation support
    pub derivation_supported: bool,
}

impl Default for KeyManagementCapabilities {
    fn default() -> Self {
        Self {
            max_keys: None,
            key_types: vec!["AES256".to_string(), "Ed25519".to_string()],
            rotation_supported: true,
            derivation_supported: true,
        }
    }
}

/// Advanced feature capabilities
#[derive(Debug, Clone, Default)]
pub struct AdvancedFeatureCapabilities {
    /// Secure enclave support
    pub secure_enclave: bool,
    /// Hardware-backed keystore
    pub hardware_keystore: bool,
    /// Biometric authentication
    pub biometric_auth: bool,
    /// Key attestation
    pub attestation: bool,
}

/// Performance capabilities
#[derive(Debug, Clone)]
pub struct PerformanceCapabilities {
    /// Operations per second
    pub ops_per_second: f64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// Maximum throughput in MB/s
    pub max_throughput_mbps: f64,
}

impl Default for PerformanceCapabilities {
    fn default() -> Self {
        Self {
            ops_per_second: 1000.0,
            avg_latency_ms: 10.0,
            max_throughput_mbps: 100.0,
        }
    }
}

/// Security capabilities
#[derive(Debug, Clone)]
pub struct SecurityCapabilities {
    /// Tamper detection
    pub tamper_detection: bool,
    /// Secure boot
    pub secure_boot: bool,
    /// Memory protection
    pub memory_protection: bool,
    /// Side-channel resistance
    pub side_channel_resistant: bool,
}

impl Default for SecurityCapabilities {
    fn default() -> Self {
        Self {
            tamper_detection: false,
            secure_boot: false,
            memory_protection: true,
            side_channel_resistant: false,
        }
    }
}

/// Human entropy capabilities
#[derive(Debug, Clone)]
pub struct HumanEntropyCapabilities {
    /// Support for human entropy input
    pub supported: bool,
    /// Minimum entropy bits required
    pub min_entropy_bits: u32,
    /// Quality verification
    pub quality_verification: bool,
}

impl Default for HumanEntropyCapabilities {
    fn default() -> Self {
        Self {
            supported: true,
            min_entropy_bits: 128,
            quality_verification: true,
        }
    }
}

/// API support capabilities
#[derive(Debug, Clone)]
pub struct ApiSupportCapabilities {
    /// REST API support
    pub rest_api: bool,
    /// gRPC support
    pub grpc: bool,
    /// WebSocket support
    pub websocket: bool,
    /// GraphQL support
    pub graphql: bool,
}

impl Default for ApiSupportCapabilities {
    fn default() -> Self {
        Self {
            rest_api: true,
            grpc: false,
            websocket: false,
            graphql: false,
        }
    }
}

/// Compliance capabilities
#[derive(Debug, Clone)]
pub struct ComplianceCapabilities {
    /// FIPS 140-2/3 certified
    pub fips_certified: bool,
    /// Common Criteria certified
    pub common_criteria: bool,
    /// PCI DSS compliant
    pub pci_dss: bool,
    /// GDPR compliant
    pub gdpr: bool,
}

impl Default for ComplianceCapabilities {
    fn default() -> Self {
        Self {
            fips_certified: false,
            common_criteria: false,
            pci_dss: false,
            gdpr: true,
        }
    }
}

/// Crypto operation capabilities
#[derive(Debug, Clone, Default)]
pub struct CryptoOperationCapabilities {
    /// Supports encryption operations
    pub supports_encryption: bool,
    /// Supports decryption operations
    pub supports_decryption: bool,
    /// Supports signing operations
    pub supports_signing: bool,
    /// Supports verification operations
    pub supports_verification: bool,
}

/// Key generation capabilities
#[derive(Debug, Clone, Default)]
pub struct KeyGenerationCapabilities {
    /// Supports key generation
    pub supports_generation: bool,
    /// Supports key import
    pub supports_import: bool,
    /// Supports key derivation
    pub supports_derivation: bool,
    /// Maximum key size
    pub max_key_size: usize,
}

/// Tamper resistance level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TamperResistanceLevel {
    /// No tamper resistance
    None,
    /// Software-based tamper detection
    Software,
    /// Hardware-based tamper detection
    Hardware,
    /// Military-grade tamper resistance
    MilitaryGrade,
}

impl Default for TamperResistanceLevel {
    fn default() -> Self {
        Self::Software
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_requirements_default() {
        let caps = CapabilityRequirements::default();
        assert_eq!(caps.min_security_level, "software");
        assert!(!caps.hardware_required);
        assert!(!caps.attestation_required);
    }

    #[test]
    fn test_key_management_capabilities() {
        let caps = KeyManagementCapabilities::default();
        assert!(caps.rotation_supported);
        assert!(caps.derivation_supported);
        assert_eq!(caps.key_types.len(), 2);
    }

    #[test]
    fn test_advanced_features() {
        let caps = AdvancedFeatureCapabilities::default();
        assert!(!caps.secure_enclave);
        assert!(!caps.hardware_keystore);
    }

    #[test]
    fn test_performance_capabilities() {
        let caps = PerformanceCapabilities::default();
        assert_eq!(caps.ops_per_second, 1000.0);
        assert_eq!(caps.avg_latency_ms, 10.0);
    }

    #[test]
    fn test_security_capabilities() {
        let caps = SecurityCapabilities::default();
        assert!(caps.memory_protection);
        assert!(!caps.tamper_detection);
    }

    #[test]
    fn test_human_entropy_capabilities() {
        let caps = HumanEntropyCapabilities::default();
        assert!(caps.supported);
        assert_eq!(caps.min_entropy_bits, 128);
        assert!(caps.quality_verification);
    }

    #[test]
    fn test_api_support_capabilities() {
        let caps = ApiSupportCapabilities::default();
        assert!(caps.rest_api);
        assert!(!caps.grpc);
        assert!(!caps.websocket);
    }

    #[test]
    fn test_compliance_capabilities() {
        let caps = ComplianceCapabilities::default();
        assert!(caps.gdpr);
        assert!(!caps.fips_certified);
        assert!(!caps.pci_dss);
    }
}
