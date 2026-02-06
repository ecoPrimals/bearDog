//! # Core Types for Universal HSM Discovery
//!
//! This module provides core types for the Universal HSM Discovery system,
//! enabling cross-platform HSM discovery and management.

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::universal_hsm_discovery::{
    DiscoveredHsm, EntropyCollectionMethod, HsmHealthStatus, HsmInterfaceType, HsmTier,
    PerformanceCapabilities,
};

// Re-export HealthStatus from canonical types
pub use beardog_types::canonical::HealthStatus;

// ============================================================
// HSM Connection Types
// ============================================================

/// HSM connection state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConnection {
    /// The interface type for this connection
    pub interface_type: HsmInterfaceType,

    /// Connection handle (implementation-specific)
    pub connection_handle: String,

    /// Current authentication status
    pub authentication_status: AuthenticationStatus,

    /// Performance capabilities of this HSM
    pub capabilities: PerformanceCapabilities,

    /// When the connection was established
    pub established_at: DateTime<Utc>,
}

/// Authentication status for HSM connection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthenticationStatus {
    /// Not authenticated
    Unauthenticated,

    /// Biometric authentication required
    BiometricRequired,

    /// PIN required
    PinRequired,

    /// Fully authenticated
    Authenticated,
}

impl Default for AuthenticationStatus {
    fn default() -> Self {
        Self::Unauthenticated
    }
}

// ============================================================
// Operation Types
// ============================================================

/// Operation type for HSM requests
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationType {
    /// Key generation operation
    KeyGeneration,

    /// Signing operation
    Signing,

    /// Verification operation
    Verification,

    /// Encryption operation
    Encryption,

    /// Decryption operation
    Decryption,

    /// Key exchange operation
    KeyExchange,

    /// Key derivation operation
    KeyDerivation,

    /// Key import operation
    KeyImport,

    /// Key export operation
    KeyExport,

    /// Health check operation
    HealthCheck,
}

/// Universal operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalOperation {
    /// Type of operation to perform
    pub operation_type: OperationType,

    /// Operation parameters
    pub parameters: HashMap<String, String>,

    /// Optional input data
    pub input_data: Option<Vec<u8>>,

    /// Timeout for the operation
    pub timeout: Option<Duration>,
}

/// Universal operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    /// Result data
    pub result_data: Vec<u8>,

    /// Operation metadata
    pub metadata: HashMap<String, String>,

    /// Performance metrics for this operation
    pub performance_metrics: PerformanceMetrics,
}

/// Performance metrics for HSM operations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Total operation duration in milliseconds
    pub duration_ms: f64,

    /// Throughput in bytes per second (if applicable)
    pub throughput_bps: Option<f64>,

    /// HSM-specific latency in milliseconds
    pub hsm_latency_ms: f64,

    /// Number of errors encountered
    pub error_count: u64,
}

// ============================================================
// Entropy Types
// ============================================================

/// Configuration for entropy collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyConfig {
    /// Collection method to use
    pub collection_method: EntropyCollectionMethod,

    /// Timeout for collection in seconds
    pub collection_timeout_seconds: u32,

    /// Minimum entropy estimate required
    pub min_entropy_estimate: f64,

    /// Whether to mix with system entropy
    pub mix_with_system_entropy: bool,
}

impl Default for EntropyConfig {
    fn default() -> Self {
        Self {
            collection_method: EntropyCollectionMethod::HardwareRng,
            collection_timeout_seconds: 5,
            min_entropy_estimate: 0.9,
            mix_with_system_entropy: true,
        }
    }
}

/// Ephemeral seed from entropy collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralSeed {
    /// The collected seed data
    pub seed_data: Vec<u8>,

    /// Estimated entropy (0.0 - 1.0)
    pub entropy_estimate: f64,

    /// When the seed was created
    pub creation_timestamp: DateTime<Utc>,

    /// Method used to collect entropy
    pub collection_method: EntropyCollectionMethod,
}

// ============================================================
// Provider Types
// ============================================================

/// HSM provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HsmProviderType {
    /// Software-based HSM
    Software,

    /// Android StrongBox
    AndroidStrongBox,

    /// iOS Secure Enclave
    IosSecureEnclave,

    /// FIDO2/Solo V2
    Fido2,

    /// TPM 2.0
    Tpm,

    /// PKCS#11 hardware token
    Pkcs11,

    /// Cloud KMS (AWS, GCP, Azure)
    CloudKms,
}

impl Default for HsmProviderType {
    fn default() -> Self {
        Self::Software
    }
}

/// Provider registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRegistration {
    /// Provider type
    pub provider_type: HsmProviderType,

    /// Provider name
    pub name: String,

    /// Priority (higher = preferred)
    pub priority: u32,

    /// Supported operations
    pub supported_operations: Vec<OperationType>,

    /// Health status
    pub health_status: HsmHealthStatus,

    /// When the provider was registered
    pub registered_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_status_default() {
        assert_eq!(
            AuthenticationStatus::default(),
            AuthenticationStatus::Unauthenticated
        );
    }

    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();
        assert_eq!(metrics.duration_ms, 0.0);
        assert_eq!(metrics.error_count, 0);
    }

    #[test]
    fn test_entropy_config_default() {
        let config = EntropyConfig::default();
        assert_eq!(config.collection_timeout_seconds, 5);
        assert!(config.mix_with_system_entropy);
    }

    #[test]
    fn test_hsm_provider_type_default() {
        assert_eq!(HsmProviderType::default(), HsmProviderType::Software);
    }
}
