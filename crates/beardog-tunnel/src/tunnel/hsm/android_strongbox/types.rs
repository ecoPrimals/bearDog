//! # Android StrongBox Types Module
//!
//! This module contains all types, structs, and enums specific to the Android StrongBox HSM.
//! It includes device information, key parameters, error types, and supporting structures.

use beardog_errors::{BearDogError, BearDogResult};
use crate::tunnel::hsm::types::*;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// StrongBox-specific error types
#[derive(Debug, Clone)]
pub enum StrongBoxError {
    /// Android Keystore operation failed
    KeystoreError { 
        /// Android Keystore error code
        code: i32, 
        /// Error message description
        message: String 
    },
    /// StrongBox hardware not available
    HardwareUnavailable { 
        /// Reason why hardware is unavailable
        reason: String 
    },
    /// Attestation verification failed
    AttestationError { 
        /// Reason for attestation failure
        reason: String 
    },
    /// Invalid key parameters
    InvalidParameters { 
        /// Name of the invalid parameter
        parameter: String, 
        /// Reason why the parameter is invalid
        reason: String 
    },
    /// Device not supported
    UnsupportedDevice {
        /// Device manufacturer name
        manufacturer: String,
        /// Device model name
        model: String,
        /// Reason why device is not supported
        reason: String,
    },
}

/// Main Android StrongBox HSM structure
///
/// This structure represents the Android StrongBox HSM implementation with
/// hardware-backed key operations on GrapheneOS/Pixel devices.
pub struct AndroidStrongBoxHsm {
    /// Android HSM configuration
    pub config: AndroidHsmConfig,
    /// Android Keystore interface
    pub keystore: Arc<AndroidKeystore>,
    /// Attestation service for key verification
    pub attestation_service: Arc<AndroidAttestationService>,
    /// Device information and capabilities
    pub device_info: Arc<AndroidDeviceInfo>,
    /// Cache for frequently accessed key information
    pub key_cache: Arc<RwLock<HashMap<String, CachedKeyInfo>>>,
    /// Health monitoring service
    pub health_monitor: Arc<AndroidHealthMonitor>,
}

/// Android Keystore integration structure
///
/// Handles communication with the Android Keystore service and StrongBox.
pub struct AndroidKeystore {
    /// Keystore configuration
    pub config: KeystoreConfig,
    /// Whether StrongBox is available on this device
    pub strongbox_available: bool,
    /// Type of StrongBox implementation
    pub strongbox_implementation: StrongBoxImplementation,
}

/// Android Attestation Service structure
///
/// Manages key attestation and certificate chain verification.
pub struct AndroidAttestationService {
    /// Attestation configuration
    pub config: AttestationConfig,
    /// Trusted root certificates for validation
    pub trusted_certificates: Vec<Vec<u8>>,
    /// Challenge generator for attestation
    pub challenge_generator: Arc<ChallengeGenerator>,
}

/// Android Device Information structure
///
/// Contains information about the Android device and its security capabilities.
pub struct AndroidDeviceInfo {
    /// Device manufacturer (e.g., "Google", "Samsung")
    pub manufacturer: String,
    /// Device model (e.g., "Pixel 8a", "Galaxy S23")
    pub model: String,
    /// Android version (e.g., "13", "14")
    pub android_version: String,
    /// StrongBox implementation version
    pub strongbox_version: Option<String>,
    /// Titan M chip version (for Pixel devices)
    pub titan_m_version: Option<String>,
    /// Security patch level date
    pub security_patch_level: String,
    /// Verified boot state of the device
    pub verified_boot_state: VerifiedBootState,
}

/// Cached key information for Android StrongBox
#[derive(Debug, Clone)]
pub struct CachedKeyInfo {
    /// Unique key identifier
    pub key_id: String,
    /// Type of cryptographic key
    pub key_type: KeyType,
    /// HSM tier information
    pub hsm_type: HsmTier,
    /// Whether key is backed by StrongBox
    pub strongbox_backed: bool,
    /// Whether attestation has been verified
    pub attestation_verified: bool,
    /// Timestamp of last key usage
    pub last_used: chrono::DateTime<Utc>,
    /// Number of times key has been used
    pub usage_count: u64,
    /// Timestamp when key was created
    pub created_at: chrono::DateTime<Utc>,
    /// Key usage policy
    pub usage_policy: KeyUsagePolicy,
    /// Current health status of the key
    pub health_status: KeyHealthStatus,
}

/// Android Health Monitor structure
///
/// Monitors the health of various Android StrongBox components.
pub struct AndroidHealthMonitor {
    /// Health status of Android Keystore
    pub keystore_health: Arc<RwLock<HsmHealthStatus>>,
    /// Health status of StrongBox hardware
    pub strongbox_health: Arc<RwLock<HsmHealthStatus>>,
    /// Health status of attestation service
    pub attestation_health: Arc<RwLock<HsmHealthStatus>>,
}

/// Challenge Generator structure
///
/// Generates cryptographic challenges for attestation and authentication.
pub struct ChallengeGenerator {
    /// Source of cryptographic entropy
    pub entropy_source: Arc<dyn EntropySource>,
}

/// Trait for providing cryptographic entropy.
pub trait EntropySource: Send + Sync {
    /// Generate cryptographic entropy of the specified length
    fn generate_entropy(&self, length: usize) -> BearDogResult<Vec<u8>>;
}

/// Android Entropy Source implementation
///
/// Uses Android's hardware random number generator.
pub struct AndroidEntropySource;

/// Verified Boot State enumeration
///
/// Represents the verified boot state of the Android device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifiedBootState {
    /// Verified boot with locked bootloader
    Green,  // Verified boot with locked bootloader
    /// Verified boot with unlocked bootloader
    Yellow, // Verified boot with unlocked bootloader
    /// Custom OS
    Orange, // Custom OS
    /// Boot failure
    Red,    // Boot failure
    /// Unknown boot state
    Unknown,
}

/// Android Key Parameters structure
///
/// Configuration parameters for Android Keystore key generation.
pub struct AndroidKeyParams {
    /// Cryptographic algorithm to use
    pub algorithm: AndroidKeyAlgorithm,
    /// Key size in bits
    pub key_size: u32,
    /// Allowed purposes for the key
    pub purposes: Vec<AndroidKeyPurpose>,
    /// Whether StrongBox backing is required
    pub strongbox_required: bool,
    /// Whether user authentication is required
    pub user_authentication_required: bool,
    /// User authentication validity duration in seconds
    pub user_authentication_validity_duration: Option<u32>,
    /// Challenge for key attestation
    pub attestation_challenge: Option<Vec<u8>>,
    /// Key validity end time
    pub key_validity_end: Option<chrono::DateTime<Utc>>,
    /// Elliptic curve for EC keys
    pub curve: Option<AndroidEcCurve>,
}

/// Android Key Algorithm enumeration
///
/// Supported cryptographic algorithms in Android StrongBox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidKeyAlgorithm {
    /// Elliptic Curve cryptography
    Ec,
    /// RSA cryptography
    Rsa,
    /// AES symmetric encryption
    Aes,
}

/// Android Key Purpose enumeration
///
/// Specifies the allowed purposes for an Android key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidKeyPurpose {
    /// Key can be used for encryption operations
    Encrypt,
    /// Key can be used for decryption operations
    Decrypt,
    /// Key can be used for signing operations
    Sign,
    /// Key can be used for verification operations
    Verify,
    /// Key can be used for key wrapping operations
    Wrap,
    /// Key can be used for key unwrapping operations
    Unwrap,
}

/// Supported elliptic curves in Android StrongBox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidEcCurve {
    /// NIST P-256 elliptic curve
    P256,
    /// NIST P-384 elliptic curve
    P384,
    /// NIST P-521 elliptic curve
    P521,
}

impl AndroidKeyParams {
    /// Create new Android key parameters with default values
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

    /// Set the key algorithm
    pub fn set_algorithm(&mut self, algorithm: AndroidKeyAlgorithm) {
        self.algorithm = algorithm;
    }

    /// Set the key size
    pub fn set_key_size(&mut self, size: u32) {
        self.key_size = size;
    }

    /// Set the key purposes
    pub fn set_purposes(&mut self, purposes: Vec<AndroidKeyPurpose>) {
        self.purposes = purposes;
    }

    /// Set whether StrongBox is required
    pub fn set_strongbox_required(&mut self, required: bool) {
        self.strongbox_required = required;
    }

    /// Set whether user authentication is required
    pub fn set_user_authentication_required(&mut self, required: bool) {
        self.user_authentication_required = required;
    }

    /// Set user authentication validity duration
    pub fn set_user_authentication_validity_duration(&mut self, duration: u32) {
        self.user_authentication_validity_duration = Some(duration);
    }

    /// Set attestation challenge
    pub fn set_attestation_challenge(&mut self, challenge: Vec<u8>) {
        self.attestation_challenge = Some(challenge);
    }

    /// Set key validity end time
    pub fn set_key_validity_end(&mut self, end: chrono::DateTime<Utc>) {
        self.key_validity_end = Some(end);
    }

    /// Set elliptic curve
    pub fn set_curve(&mut self, curve: AndroidEcCurve) {
        self.curve = Some(curve);
    }
}

impl Default for AndroidKeyParams {
    fn default() -> Self {
        Self::new()
    }
}

impl From<StrongBoxError> for BearDogError {
    fn from(err: StrongBoxError) -> Self {
        BearDogError::Hsm {
            message: format!("StrongBox error: {err:?}"),
        }
    }
}

impl StrongBoxError {
    /// Create a BearDogError from a StrongBox error code and message
    pub fn from_strongbox_error(code: i32, message: &str) -> BearDogError {
        BearDogError::Hsm {
            message: format!("StrongBox error (code {code}): {message}"),
        }
    }
}
