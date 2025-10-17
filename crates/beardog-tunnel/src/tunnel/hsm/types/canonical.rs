//! Canonical HSM Types
//!
//! Universal type definitions for HSM operations across all platforms.
//!
//! MODERNIZATION NOTE: This file contains vendor-specific references that should be migrated
//! to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
//! Target: Replace with capability-based discovery for vendor/primal agnosticism

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Memory protection levels for key storage
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryProtectionLevel {
    /// No memory protection
    None,
    /// Low security memory protection
    Low,
    /// Medium security memory protection
    Medium,
    /// High security memory protection
    High,
    /// Maximum security memory protection
    Maximum,
}

impl Default for MemoryProtectionLevel {
    fn default() -> Self {
        Self::Medium
    }
}

/// Performance metrics for HSM operations
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Operations per second
    pub operations_per_second: f64,
    /// Average latency in milliseconds
    pub average_latency_ms: f64,
    /// Success rate percentage
    pub success_rate: f64,
    /// Memory usage in MB
    pub memory_usage_mb: f64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Number of errors
    pub error_count: u64,
    /// Uptime in seconds
    pub uptime_seconds: u64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            operations_per_second: 0.0,
            average_latency_ms: 0.0,
            success_rate: 100.0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            error_count: 0,
            uptime_seconds: 0,
        }
    }
}

/// Attestation security levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttestationLevel {
    /// Software-only attestation
    Software,
    /// Hardware-backed attestation
    Hardware,
    /// Verified boot attestation
    VerifiedBoot,
    /// StrongBox-level attestation
    StrongBox,
}

/// HSM connection types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HsmType {
    /// Network-connected HSM
    Network,
    /// USB-connected HSM
    Usb,
    /// PCIe-connected HSM
    Pcie,
    /// Cloud-based HSM
    Cloud,
    /// Mobile device HSM
    Mobile,
    /// Trusted Platform Module
    Tpm,
}

/// Smartphone platform types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmartphoneType {
    /// Android device
    Android,
    /// iOS device
    Ios,
    /// Other smartphone platform
    Other(String),
}

/// Secure enclave implementations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecureEnclaveType {
    /// Apple Secure Enclave
    AppleSecureEnclave,
    /// Android StrongBox
    AndroidStrongBox,
    /// Samsung Knox
    SamsungKnox,
    /// Qualcomm SPU
    QualcommSpu,
    /// Trusted Execution Environment
    TrustedExecutionEnvironment,
}

/// Software HSM implementations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoftwareHsmType {
    /// SoftHSM implementation
    SoftHsm,
    /// OpenSSL-based implementation
    OpenSsl,
    /// BearDog native implementation
    BearDogNative,
    /// Custom implementation
    Custom(String),
}

/// Entropy quality ratings
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntropyQualityRating {
    /// Insufficient entropy
    Insufficient,
    /// Basic entropy quality
    Basic,
    /// Good entropy quality
    Good,
    /// Excellent entropy quality
    Excellent,
}

/// Entropy source types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntropySource {
    /// Hardware RNG
    HardwareRng,
    /// Operating system RNG
    OsRng,
    /// Combined sources
    Mixed,
    /// Custom entropy source
    Custom(String),
}

/// Key storage types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyStorageType {
    /// Hardware-backed storage
    Hardware,
    /// Software-encrypted storage
    Encrypted,
    /// In-memory storage
    Memory,
    /// File-based storage
    File,
    /// Database storage
    Database,
}

/// Android key algorithms
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AndroidKeyAlgorithm {
    /// RSA encryption
    Rsa,
    /// Elliptic Curve Cryptography
    Ec,
    /// AES encryption
    Aes,
    /// HMAC
    Hmac,
}

/// StrongBox implementation types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrongBoxImplementation {
    /// Qualcomm implementation
    Qualcomm,
    /// MediaTek implementation
    MediaTek,
    /// Samsung implementation
    Samsung,
    /// Generic implementation
    Generic,
}

/// HSM health status
#[derive(Debug, Clone)]
pub struct HsmHealthStatus {
    /// Whether the HSM is healthy
    pub is_healthy: bool,
    /// Last health check timestamp
    pub last_check: DateTime<Utc>,
    /// Optional error message
    pub error_message: Option<String>,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
}

/// HSM configuration
#[derive(Debug, Clone)]
pub struct HsmConfig {
    /// HSM type
    pub hsm_type: HsmType,
    /// Connection info
    pub connection_info: Option<String>,
    /// Security level
    pub security_level: u8,
    /// Enable attestation
    pub attestation_enabled: bool,
}

/// Key metadata
#[derive(Debug, Clone)]
pub struct KeyMetadata {
    /// Key ID
    pub key_id: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last access timestamp
    pub last_accessed: Option<DateTime<Utc>>,
    /// Access count
    pub access_count: u64,
    /// Custom metadata
    pub custom_data: HashMap<String, String>,
}

impl KeyMetadata {
    /// Create new key metadata
    pub fn new(key_id: String) -> Self {
        Self {
            key_id,
            created_at: Utc::now(),
            last_accessed: None,
            access_count: 0,
            custom_data: HashMap::new(),
        }
    }

    /// Record key access
    pub fn record_access(&mut self) {
        self.last_accessed = Some(Utc::now());
        self.access_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_protection_level_default() {
        assert_eq!(
            MemoryProtectionLevel::default(),
            MemoryProtectionLevel::Medium
        );
    }

    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();
        assert_eq!(metrics.operations_per_second, 0.0);
        assert_eq!(metrics.success_rate, 100.0);
        assert_eq!(metrics.error_count, 0);
    }

    #[test]
    fn test_key_metadata_creation() {
        let metadata = KeyMetadata::new("test-key".to_string());
        assert_eq!(metadata.key_id, "test-key");
        assert_eq!(metadata.access_count, 0);
        assert!(metadata.last_accessed.is_none());
    }

    #[test]
    fn test_key_metadata_record_access() {
        let mut metadata = KeyMetadata::new("test-key".to_string());
        metadata.record_access();

        assert_eq!(metadata.access_count, 1);
        assert!(metadata.last_accessed.is_some());

        metadata.record_access();
        assert_eq!(metadata.access_count, 2);
    }

    #[test]
    fn test_attestation_level_serialization() {
        let level = AttestationLevel::Hardware;
        let serialized = serde_json::to_string(&level).unwrap();
        let deserialized: AttestationLevel = serde_json::from_str(&serialized).unwrap();
        assert_eq!(level, deserialized);
    }

    #[test]
    fn test_hsm_type_variants() {
        let types = vec![
            HsmType::Network,
            HsmType::Usb,
            HsmType::Pcie,
            HsmType::Cloud,
            HsmType::Mobile,
            HsmType::Tpm,
        ];
        assert_eq!(types.len(), 6);
    }

    #[test]
    fn test_smartphone_type_custom() {
        let custom = SmartphoneType::Other("CustomOS".to_string());
        match custom {
            SmartphoneType::Other(name) => assert_eq!(name, "CustomOS"),
            _ => panic!("Expected Other variant"),
        }
    }

    #[test]
    fn test_software_hsm_type_custom() {
        let custom = SoftwareHsmType::Custom("MyHSM".to_string());
        match custom {
            SoftwareHsmType::Custom(name) => assert_eq!(name, "MyHSM"),
            _ => panic!("Expected Custom variant"),
        }
    }
}
