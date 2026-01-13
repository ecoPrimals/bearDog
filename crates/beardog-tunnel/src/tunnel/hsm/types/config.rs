//! HSM Configuration Types
//!
//! Configuration type definitions for HSM operations and connections.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Memory protection configuration
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    /// Memory protection level
    pub protection_level: MemoryProtectionLevel,
    /// Enable memory encryption
    pub enable_encryption: bool,
    /// Memory pool size in bytes
    pub pool_size: usize,
}

/// Memory protection levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryProtectionLevel {
    /// No protection
    None,
    /// Low protection
    Low,
    /// Medium protection
    Medium,
    /// High protection
    High,
    /// Maximum protection
    Maximum,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            protection_level: MemoryProtectionLevel::Medium,
            enable_encryption: true,
            pool_size: 1024 * 1024, // 1MB
        }
    }
}

/// Hardware HSM connection configuration
#[derive(Debug, Clone)]
pub struct HsmConnectionConfig {
    /// Connection URL or identifier
    pub url: String,
    /// Connection timeout
    pub timeout: Duration,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Whether to use TLS
    pub use_tls: bool,
}

/// Unified authentication configuration
#[derive(Debug, Clone)]
pub struct UnifiedAuthConfig {
    /// Authentication method
    pub method: AuthMethod,
    /// Credentials
    pub credentials: HashMap<String, String>,
    /// Session timeout
    pub session_timeout: Option<Duration>,
}

/// Authentication methods
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthMethod {
    /// No authentication
    None,
    /// Password-based
    Password,
    /// Certificate-based
    Certificate,
    /// Token-based
    Token,
    /// Biometric
    Biometric,
    /// Multi-factor
    MultiFactory,
}

/// High Availability configuration
#[derive(Debug, Clone)]
pub struct HaConfig {
    /// Enable HA mode
    pub enabled: bool,
    /// Failover timeout
    pub failover_timeout: Duration,
    /// Replica nodes
    pub replicas: Vec<String>,
}

/// Hardware HSM configuration
#[derive(Debug, Clone)]
pub struct HardwareHsmConfig {
    /// Connection configuration
    pub connection: HsmConnectionConfig,
    /// Authentication configuration
    pub auth_config: UnifiedAuthConfig,
    /// High availability configuration
    pub ha_config: Option<HaConfig>,
}

/// Smartphone HSM configuration variants
#[derive(Debug, Clone)]
pub enum SmartphoneHsmConfig {
    /// iOS Secure Enclave configuration
    Ios(IosHsmConfig),
    /// Android StrongBox configuration
    Android(AndroidHsmConfig),
}

/// iOS HSM configuration
#[derive(Debug, Clone)]
pub struct IosHsmConfig {
    /// Use Secure Enclave
    pub use_secure_enclave: bool,
    /// Require biometric authentication
    pub require_biometric: bool,
    /// Access control flags
    pub access_control: u32,
}

/// Android HSM configuration
#[derive(Debug, Clone)]
pub struct AndroidHsmConfig {
    /// Use StrongBox
    pub use_strongbox: bool,
    /// Require user authentication
    pub require_user_auth: bool,
    /// Authentication timeout (seconds)
    pub auth_timeout: Option<u32>,
}

/// Universal HSM configuration
#[derive(Debug, Clone)]
pub struct UniversalHsmConfig {
    /// HSM provider type
    pub provider_type: HsmProviderType,
    /// Security level requirement
    pub security_level: SecurityLevel,
    /// Performance requirements
    pub performance: PerformanceRequirements,
    /// Feature requirements
    pub features: FeatureRequirements,
}

// Re-export canonical HsmProviderType
pub use beardog_types::canonical::hsm::HsmProviderType;

/// Security levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Low security
    Low,
    /// Medium security
    Medium,
    /// High security
    High,
    /// Maximum security
    Maximum,
}

/// Software HSM configuration  
#[derive(Debug, Clone)]
pub struct SoftwareHsmConfig {
    /// Memory configuration
    pub memory_config: MemoryConfig,
    /// Crypto backend to use
    pub crypto_backend: CryptoBackendType,
    /// Key storage configuration
    pub key_storage: super::tier::KeyStorageType,
    /// Enable encryption at rest
    pub encryption_at_rest: bool,
    /// Memory protection level
    pub memory_protection: super::tier::MemoryProtectionLevel,
}

impl Default for SoftwareHsmConfig {
    fn default() -> Self {
        Self {
            memory_config: MemoryConfig::default(),
            crypto_backend: CryptoBackendType::RustCrypto,
            key_storage: super::tier::KeyStorageType::Memory,
            encryption_at_rest: true,
            memory_protection: super::tier::MemoryProtectionLevel::High,
        }
    }
}

/// Crypto backend types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CryptoBackendType {
    /// GeneticCrypto - 100% Pure Rust with genetic enhancements (RECOMMENDED)
    GeneticCrypto,
    /// Ring cryptography library (has C dependencies)
    Ring,
    /// OpenSSL
    OpenSsl,
    /// RustCrypto
    RustCrypto,
}

/// Type alias for compatibility
pub type CryptoBackend = CryptoBackendType;

/// Performance requirements
#[derive(Debug, Clone)]
pub struct PerformanceRequirements {
    /// Minimum operations per second
    pub min_ops_per_second: f64,
    /// Maximum latency in milliseconds
    pub max_latency_ms: f64,
    /// Required throughput in MB/s
    pub min_throughput_mbps: f64,
}

impl Default for PerformanceRequirements {
    fn default() -> Self {
        Self {
            min_ops_per_second: 100.0,
            max_latency_ms: 100.0,
            min_throughput_mbps: 10.0,
        }
    }
}

/// Feature requirements
#[derive(Debug, Clone, Default)]
pub struct FeatureRequirements {
    /// Require hardware backing
    pub hardware_backed: bool,
    /// Require key attestation
    pub attestation_required: bool,
    /// Require secure boot
    pub secure_boot: bool,
    /// Require tamper detection
    pub tamper_detection: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_config_default() {
        let config = MemoryConfig::default();
        assert_eq!(config.protection_level, MemoryProtectionLevel::Medium);
        assert!(config.enable_encryption);
        assert_eq!(config.pool_size, 1024 * 1024);
    }

    #[test]
    fn test_hsm_connection_config() {
        const TEST_PORT: u16 = 8080;
        let url = format!("localhost:{}", TEST_PORT);
        let config = HsmConnectionConfig {
            url: url.clone(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            use_tls: true,
        };

        assert_eq!(config.url, url);
        assert!(config.use_tls);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_auth_method_variants() {
        let methods = [
            AuthMethod::None,
            AuthMethod::Password,
            AuthMethod::Certificate,
            AuthMethod::Token,
            AuthMethod::Biometric,
            AuthMethod::MultiFactory,
        ];
        assert_eq!(methods.len(), 6);
    }

    #[test]
    fn test_performance_requirements_default() {
        let perf = PerformanceRequirements::default();
        assert_eq!(perf.min_ops_per_second, 100.0);
        assert_eq!(perf.max_latency_ms, 100.0);
        assert_eq!(perf.min_throughput_mbps, 10.0);
    }

    #[test]
    fn test_feature_requirements_default() {
        let features = FeatureRequirements::default();
        assert!(!features.hardware_backed);
        assert!(!features.attestation_required);
        assert!(!features.secure_boot);
        assert!(!features.tamper_detection);
    }

    #[test]
    fn test_smartphone_hsm_config_variants() {
        let ios_config = SmartphoneHsmConfig::Ios(IosHsmConfig {
            use_secure_enclave: true,
            require_biometric: true,
            access_control: 0,
        });

        match ios_config {
            SmartphoneHsmConfig::Ios(config) => {
                assert!(config.use_secure_enclave);
                assert!(config.require_biometric);
            }
            _ => panic!("Expected iOS variant"),
        }
    }

    #[test]
    fn test_ha_config() {
        let ha = HaConfig {
            enabled: true,
            failover_timeout: Duration::from_secs(10),
            replicas: vec!["replica1".to_string(), "replica2".to_string()],
        };

        assert!(ha.enabled);
        assert_eq!(ha.replicas.len(), 2);
    }
}
