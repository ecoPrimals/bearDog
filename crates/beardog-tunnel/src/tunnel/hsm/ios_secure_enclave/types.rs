//! # iOS Secure Enclave Types
//!
//! This module provides types for iOS Secure Enclave integration,
//! enabling hardware-backed key storage and cryptographic operations.

use std::marker::PhantomData;

// ============================================================
// Device Types
// ============================================================

/// iOS device type with Secure Enclave
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecureEnclaveDevice {
    /// iPhone with Secure Enclave
    IPhone,
    /// iPad with Secure Enclave
    IPad,
    /// Mac with Apple Silicon
    MacAppleSilicon,
    /// Simulator (no real Secure Enclave)
    Simulator,
}

impl Default for SecureEnclaveDevice {
    fn default() -> Self {
        Self::Simulator
    }
}

/// Biometric feature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiometricFeature {
    /// Touch ID
    TouchID,
    /// Face ID
    FaceID,
    /// None available
    None,
}

impl Default for BiometricFeature {
    fn default() -> Self {
        Self::None
    }
}

// ============================================================
// iOS Version
// ============================================================

/// iOS version
#[derive(Debug, Clone)]
pub struct IOSVersion {
    /// Major version
    pub major: u32,
    /// Minor version
    pub minor: u32,
    /// Patch version
    pub patch: u32,
}

impl Default for IOSVersion {
    fn default() -> Self {
        Self {
            major: 17,
            minor: 0,
            patch: 0,
        }
    }
}

impl IOSVersion {
    /// Create new version
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    /// Check if version meets minimum requirement
    pub fn meets_minimum(&self, major: u32, minor: u32) -> bool {
        self.major > major || (self.major == major && self.minor >= minor)
    }
}

// ============================================================
// Algorithm Types
// ============================================================

/// Secure Enclave algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecureEnclaveAlgorithm {
    /// ECDSA P-256
    EcdsaP256,
    /// ECDH P-256
    EcdhP256,
}

impl Default for SecureEnclaveAlgorithm {
    fn default() -> Self {
        Self::EcdsaP256
    }
}

/// Biometric policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiometricPolicy {
    /// Touch ID required
    TouchIDRequired,
    /// Face ID required
    FaceIDRequired,
    /// Touch ID or Face ID
    TouchIDOrFaceID,
    /// Face ID only
    FaceIDOnly,
    /// Touch ID only
    TouchIDOnly,
    /// Any biometric
    AnyBiometric,
    /// No biometric required
    NoBiometric,
}

impl Default for BiometricPolicy {
    fn default() -> Self {
        Self::NoBiometric
    }
}

// ============================================================
// Key Types
// ============================================================

/// Secure key reference
#[derive(Debug, Clone)]
pub struct SecureKeyReference {
    /// Key identifier
    pub key_id: String,
    /// Whether key is in Secure Enclave
    pub in_secure_enclave: bool,
}

impl Default for SecureKeyReference {
    fn default() -> Self {
        Self {
            key_id: String::new(),
            in_secure_enclave: false,
        }
    }
}

/// Secure Enclave key pair
#[derive(Debug, Clone)]
pub struct SecureEnclaveKeyPair {
    /// Public key bytes
    pub public_key: Vec<u8>,
    /// Private key bytes (only for software fallback)
    pub private_key: Option<Vec<u8>>,
}

impl Default for SecureEnclaveKeyPair {
    fn default() -> Self {
        Self {
            public_key: Vec::new(),
            private_key: None,
        }
    }
}

/// Secure Enclave key
#[derive(Debug, Clone)]
pub struct SecureEnclaveKey {
    /// Key reference
    pub key_ref: SecureKeyReference,
    /// Algorithm
    pub algorithm: SecureEnclaveAlgorithm,
    /// Biometric policy
    pub biometric_policy: BiometricPolicy,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Usage count
    pub usage_count: u64,
}

impl Default for SecureEnclaveKey {
    fn default() -> Self {
        Self {
            key_ref: SecureKeyReference::default(),
            algorithm: SecureEnclaveAlgorithm::default(),
            biometric_policy: BiometricPolicy::default(),
            created_at: chrono::Utc::now(),
            usage_count: 0,
        }
    }
}

// ============================================================
// Metrics
// ============================================================

/// Secure Enclave metrics
#[derive(Debug, Clone)]
pub struct SecureEnclaveMetrics {
    /// Total operations
    pub total_operations: u64,
    /// Success rate (0.0 - 1.0)
    pub success_rate: f64,
    /// Average operation time in ms
    pub avg_operation_time_ms: f64,
    /// Last operation timestamp
    pub last_operation: Option<chrono::DateTime<chrono::Utc>>,
    /// Biometric failures
    pub biometric_failures: u64,
}

impl Default for SecureEnclaveMetrics {
    fn default() -> Self {
        Self {
            total_operations: 0,
            success_rate: 1.0,
            avg_operation_time_ms: 0.0,
            last_operation: None,
            biometric_failures: 0,
        }
    }
}

impl SecureEnclaveMetrics {
    /// Create new metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an operation
    pub fn record_operation(&mut self, success: bool, duration_ms: f64) {
        self.total_operations += 1;
        self.last_operation = Some(chrono::Utc::now());

        if self.total_operations == 1 {
            self.avg_operation_time_ms = duration_ms;
            self.success_rate = if success { 1.0 } else { 0.0 };
        } else {
            // Update average
            let total_time =
                self.avg_operation_time_ms * (self.total_operations - 1) as f64 + duration_ms;
            self.avg_operation_time_ms = total_time / self.total_operations as f64;

            // Update success rate
            let successes = (self.success_rate * (self.total_operations - 1) as f64
                + if success { 1.0 } else { 0.0 }) as f64;
            self.success_rate = successes / self.total_operations as f64;
        }
    }

    /// Record biometric failure
    pub fn record_biometric_failure(&mut self) {
        self.biometric_failures += 1;
    }
}

// ============================================================
// Capability Types
// ============================================================

/// Secure Enclave capability
#[derive(Debug, Clone)]
pub struct SecureEnclaveCapability {
    /// iOS version
    pub ios_version: IOSVersion,
    /// Device type
    pub device_type: SecureEnclaveDevice,
    /// Biometric features
    pub biometric_features: Vec<BiometricFeature>,
    /// Marker
    pub(crate) _marker: PhantomData<()>,
}

impl SecureEnclaveCapability {
    /// Create new capability
    pub fn new(
        ios_version: IOSVersion,
        device_type: SecureEnclaveDevice,
        biometric_features: Vec<BiometricFeature>,
    ) -> Self {
        Self {
            ios_version,
            device_type,
            biometric_features,
            _marker: PhantomData,
        }
    }

    /// Get iOS version
    pub fn ios_version(&self) -> &IOSVersion {
        &self.ios_version
    }

    /// Get device type
    pub fn device_type(&self) -> &SecureEnclaveDevice {
        &self.device_type
    }

    /// Get biometric features
    pub fn biometric_features(&self) -> &[BiometricFeature] {
        &self.biometric_features
    }

    /// Check if Secure Enclave is available
    pub fn is_secure_enclave_available(&self) -> bool {
        !matches!(self.device_type, SecureEnclaveDevice::Simulator)
    }

    /// Check if biometric authentication is available
    pub fn has_biometric(&self) -> bool {
        self.biometric_features
            .iter()
            .any(|f| !matches!(f, BiometricFeature::None))
    }
}

impl Default for SecureEnclaveCapability {
    fn default() -> Self {
        Self {
            ios_version: IOSVersion::default(),
            device_type: SecureEnclaveDevice::default(),
            biometric_features: vec![BiometricFeature::None],
            _marker: PhantomData,
        }
    }
}

// ============================================================
// Traits
// ============================================================

/// Secure Enclave constraint trait
pub trait SecureEnclaveConstraint {
    /// Get algorithm
    fn algorithm(&self) -> SecureEnclaveAlgorithm;

    /// Check if Secure Enclave is supported
    fn is_secure_enclave_supported(&self) -> bool;
}

/// ECDSA P-256 marker type
pub struct SecureEnclaveEcdsaP256;

/// ECDH P-256 marker type
pub struct SecureEnclaveEcdhP256;

impl SecureEnclaveConstraint for SecureEnclaveEcdsaP256 {
    fn algorithm(&self) -> SecureEnclaveAlgorithm {
        SecureEnclaveAlgorithm::EcdsaP256
    }

    fn is_secure_enclave_supported(&self) -> bool {
        true // ECDSA P-256 is supported by Secure Enclave
    }
}

impl SecureEnclaveConstraint for SecureEnclaveEcdhP256 {
    fn algorithm(&self) -> SecureEnclaveAlgorithm {
        SecureEnclaveAlgorithm::EcdhP256
    }

    fn is_secure_enclave_supported(&self) -> bool {
        true // ECDH P-256 is supported by Secure Enclave
    }
}

impl SecureEnclaveConstraint for SecureEnclaveAlgorithm {
    fn algorithm(&self) -> SecureEnclaveAlgorithm {
        *self
    }

    fn is_secure_enclave_supported(&self) -> bool {
        matches!(
            self,
            SecureEnclaveAlgorithm::EcdsaP256 | SecureEnclaveAlgorithm::EcdhP256
        )
    }
}

/// Key agreement capable marker trait
pub trait KeyAgreementCapable {}

impl KeyAgreementCapable for SecureEnclaveEcdhP256 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ios_version() {
        let version = IOSVersion::new(17, 0, 0);
        assert!(version.meets_minimum(16, 0));
        assert!(version.meets_minimum(17, 0));
        assert!(!version.meets_minimum(18, 0));
    }

    #[test]
    fn test_metrics_default() {
        let metrics = SecureEnclaveMetrics::default();
        assert_eq!(metrics.total_operations, 0);
        assert_eq!(metrics.success_rate, 1.0);
    }

    #[test]
    fn test_metrics_recording() {
        let mut metrics = SecureEnclaveMetrics::new();
        metrics.record_operation(true, 10.0);
        assert_eq!(metrics.total_operations, 1);
        assert_eq!(metrics.avg_operation_time_ms, 10.0);
    }

    #[test]
    fn test_capability() {
        let cap = SecureEnclaveCapability::new(
            IOSVersion::new(17, 0, 0),
            SecureEnclaveDevice::IPhone,
            vec![BiometricFeature::FaceID],
        );
        assert!(cap.is_secure_enclave_available());
        assert!(cap.has_biometric());
    }

    #[test]
    fn test_algorithm_constraint() {
        let ecdsa = SecureEnclaveEcdsaP256;
        assert!(ecdsa.is_secure_enclave_supported());
        assert_eq!(ecdsa.algorithm(), SecureEnclaveAlgorithm::EcdsaP256);
    }
}
