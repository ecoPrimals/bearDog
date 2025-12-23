// Android StrongBox Types
//
// Canonical type definitions for Android StrongBox HSM integration

use beardog_errors::BearDogError;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// StrongBox implementation variants on Android devices
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrongBoxImplementation {
    /// Google Titan M security chip
    TitanM {
        version: String,
        security_level: String,
    },
    /// Qualcomm SPU (Secure Processing Unit)
    QualcommSpu { attestation_support: bool },
    /// Samsung Knox security platform
    SamsungKnox { version: String },
    /// MediaTek HSM implementation
    MediaTekHsm { features: Vec<String> },
    /// Generic StrongBox implementation
    Generic {
        vendor: String,
        capabilities: Vec<String>,
    },
}

impl Default for StrongBoxImplementation {
    fn default() -> Self {
        StrongBoxImplementation::TitanM {
            version: "1.0".to_string(),
            security_level: "EAL4+".to_string(),
        }
    }
}

/// Android device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidDeviceInfo {
    pub manufacturer: String,
    pub model: String,
    pub android_version: String,
    pub strongbox_version: Option<String>,
    pub titan_m_version: Option<String>,
    pub security_patch_level: String,
    pub verified_boot_state: VerifiedBootState,
}

impl AndroidDeviceInfo {
    /// Creates a new instance with default Pixel 8 values
    ///
    /// # Errors
    /// Returns error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            manufacturer: "Google".to_string(),
            model: "Pixel 8".to_string(),
            android_version: "14".to_string(),
            strongbox_version: Some("1.0".to_string()),
            titan_m_version: Some("1.0".to_string()),
            security_patch_level: "2024-01-01".to_string(),
            verified_boot_state: VerifiedBootState::Verified,
        })
    }

    /// Detect device information from system
    ///
    /// # Errors
    /// Returns error if detection fails
    #[cfg(target_os = "android")]
    pub fn detect() -> Result<Self, BearDogError> {
        // In real implementation, query Android system properties
        Self::new()
    }

    #[cfg(not(target_os = "android"))]
    pub fn detect() -> Result<Self, BearDogError> {
        // Mock implementation for non-Android platforms
        Self::new()
    }
}

impl Default for AndroidDeviceInfo {
    fn default() -> Self {
        // Use a fallback if platform detection fails
        Self::new().unwrap_or_else(|_| AndroidDeviceInfo {
            manufacturer: "unknown".to_string(),
            model: "unknown".to_string(),
            device: "unknown".to_string(),
            hardware: None,
            board: None,
            brand: None,
            android_version: 0,
            api_level: 0,
            security_patch: None,
            verified_boot_state: VerifiedBootState::Unverified,
        })
    }
}

/// Verified boot state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerifiedBootState {
    Verified,
    SelfSigned,
    Unverified,
    Failed,
}

/// Android key parameters for StrongBox
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidKeyParams {
    pub algorithm: AndroidKeyAlgorithm,
    pub key_size: u32,
    pub purposes: Vec<AndroidKeyPurpose>,
    pub strongbox_required: bool,
    pub user_authentication_required: bool,
    pub user_authentication_validity_duration: Option<u32>,
    pub attestation_challenge: Option<Vec<u8>>,
    pub key_validity_end: Option<chrono::DateTime<Utc>>,
    pub curve: Option<AndroidEcCurve>,
}

impl AndroidKeyParams {
    /// Creates new key parameters with defaults
    pub fn new() -> Self {
        Self {
            algorithm: AndroidKeyAlgorithm::Ec,
            key_size: 256,
            purposes: vec![AndroidKeyPurpose::Sign, AndroidKeyPurpose::Verify],
            strongbox_required: true,
            user_authentication_required: false,
            user_authentication_validity_duration: None,
            attestation_challenge: None,
            key_validity_end: None,
            curve: Some(AndroidEcCurve::P256),
        }
    }

    /// Sets algorithm
    pub fn set_algorithm(&mut self, algorithm: AndroidKeyAlgorithm) -> &mut Self {
        self.algorithm = algorithm;
        self
    }

    /// Sets key size
    pub fn set_key_size(&mut self, size: u32) -> &mut Self {
        self.key_size = size;
        self
    }

    /// Sets purposes
    pub fn set_purposes(&mut self, purposes: Vec<AndroidKeyPurpose>) -> &mut Self {
        self.purposes = purposes;
        self
    }

    /// Sets StrongBox requirement
    pub fn set_strongbox_required(&mut self, required: bool) -> &mut Self {
        self.strongbox_required = required;
        self
    }

    /// Sets user authentication requirement
    pub fn set_user_authentication_required(&mut self, required: bool) -> &mut Self {
        self.user_authentication_required = required;
        self
    }

    /// Sets user authentication validity duration
    pub fn set_user_authentication_validity_duration(&mut self, duration: u32) -> &mut Self {
        self.user_authentication_validity_duration = Some(duration);
        self
    }

    /// Sets attestation challenge
    pub fn set_attestation_challenge(&mut self, challenge: Vec<u8>) -> &mut Self {
        self.attestation_challenge = Some(challenge);
        self
    }

    /// Sets key validity end time
    pub fn set_key_validity_end(&mut self, end: chrono::DateTime<Utc>) -> &mut Self {
        self.key_validity_end = Some(end);
        self
    }

    /// Sets elliptic curve
    pub fn set_curve(&mut self, curve: AndroidEcCurve) -> &mut Self {
        self.curve = Some(curve);
        self
    }
}

impl Default for AndroidKeyParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Android key algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AndroidKeyAlgorithm {
    Rsa,
    Ec,
    Aes,
    Hmac,
}

/// Android key purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AndroidKeyPurpose {
    Encrypt,
    Decrypt,
    Sign,
    Verify,
    WrapKey,
}

/// Android elliptic curves
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AndroidEcCurve {
    P256,
    P384,
    P521,
}

/// StrongBox error types
#[derive(Debug, Clone)]
pub struct StrongBoxError {
    pub code: i32,
    pub message: String,
}

impl StrongBoxError {
    /// Creates StrongBox error from code and message
    pub fn from_code(code: i32, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
}

impl From<StrongBoxError> for BearDogError {
    fn from(err: StrongBoxError) -> Self {
        BearDogError::Hsm {
            message: format!("StrongBox error (code {}): {}", err.code, err.message),
        }
    }
}

impl std::fmt::Display for StrongBoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StrongBox error (code {}): {}", self.code, self.message)
    }
}

impl std::error::Error for StrongBoxError {}

/// Android attestation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationConfig {
    pub verify_chain: bool,
    pub require_strongbox: bool,
    pub max_chain_depth: usize,
}

impl Default for AttestationConfig {
    fn default() -> Self {
        Self {
            verify_chain: true,
            require_strongbox: true,
            max_chain_depth: 5,
        }
    }
}

/// Android attestation service
#[derive(Debug, Clone)]
pub struct AndroidAttestationService {
    pub config: AttestationConfig,
    pub trusted_certificates: Vec<Vec<u8>>,
    pub challenge_generator: Arc<ChallengeGenerator>,
}

/// Challenge generator for attestation
#[derive(Debug)]
pub struct ChallengeGenerator {
    // Placeholder for entropy source
}

impl ChallengeGenerator {
    /// Creates new challenge generator
    pub fn new() -> Self {
        Self {}
    }

    /// Generates attestation challenge
    ///
    /// # Errors
    /// Returns error if challenge generation fails
    pub fn generate_challenge(&self, length: usize) -> Result<Vec<u8>, BearDogError> {
        // Use secure random generation
        let mut challenge = vec![0u8; length];
        use rand::RngCore;
        rand::thread_rng().fill_bytes(&mut challenge);
        Ok(challenge)
    }
}

impl Default for ChallengeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Android health monitor
#[derive(Debug, Clone)]
pub struct AndroidHealthMonitor {
    pub keystore_health: Arc<RwLock<HsmHealthStatus>>,
    pub strongbox_health: Arc<RwLock<HsmHealthStatus>>,
    pub attestation_health: Arc<RwLock<HsmHealthStatus>>,
}

/// HSM health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

impl Default for HsmHealthStatus {
    fn default() -> Self {
        HsmHealthStatus::Unknown
    }
}

/// Key health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyHealthStatus {
    Active,
    Expired,
    Revoked,
    Compromised,
}

/// Key usage policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyUsagePolicy {
    pub max_usage_count: Option<u64>,
    pub expiration: Option<chrono::DateTime<Utc>>,
    pub allowed_purposes: Vec<AndroidKeyPurpose>,
}

impl Default for KeyUsagePolicy {
    fn default() -> Self {
        Self {
            max_usage_count: None,
            expiration: None,
            allowed_purposes: vec![AndroidKeyPurpose::Sign, AndroidKeyPurpose::Verify],
        }
    }
}

/// Key type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    Symmetric,
    Asymmetric,
}

/// HSM tier classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmTier {
    Software,
    Hardware,
    StrongBox,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strongbox_implementation_default() {
        let impl_default = StrongBoxImplementation::default();
        match impl_default {
            StrongBoxImplementation::TitanM {
                version,
                security_level,
            } => {
                assert_eq!(version, "1.0");
                assert_eq!(security_level, "EAL4+");
            }
            _ => panic!("Expected TitanM variant"),
        }
    }

    #[test]
    fn test_android_device_info_new() {
        let device_info = AndroidDeviceInfo::new()?;
        assert_eq!(device_info.manufacturer, "Google");
        assert_eq!(device_info.model, "Pixel 8");
    }

    #[test]
    fn test_android_key_params_builder() {
        let mut params = AndroidKeyParams::new();
        params
            .set_key_size(384)
            .set_strongbox_required(true)
            .set_curve(AndroidEcCurve::P384);

        assert_eq!(params.key_size, 384);
        assert!(params.strongbox_required);
        assert_eq!(params.curve, Some(AndroidEcCurve::P384));
    }

    #[test]
    fn test_challenge_generator() {
        let generator = ChallengeGenerator::new();
        let challenge = generator.generate_challenge(32)?;
        assert_eq!(challenge.len(), 32);
    }

    #[test]
    fn test_strongbox_error_conversion() {
        let sb_error = StrongBoxError::from_code(42, "Test error");
        let beardog_error: BearDogError = sb_error.into();

        match beardog_error {
            BearDogError::Hsm { message } => {
                assert!(message.contains("42"));
                assert!(message.contains("Test error"));
            }
            _ => panic!("Expected Hsm error variant"),
        }
    }
}
