//! # Android StrongBox Types Module
//!
//! This module contains all types, structs, and enums specific to the Android StrongBox HSM.
//! It includes device information, key parameters, error types, and supporting structures.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use async_trait::async_trait;
use crate::error::{BearDogError, BearDogResult};
use crate::tunnel::hsm::types::*;
use tracing::{debug, info, warn};

/// StrongBox-specific error types
#[derive(Debug, Clone)]
pub enum StrongBoxError {
    /// Android Keystore operation failed
    KeystoreError {
        code: i32,
        message: String,
    },
    /// StrongBox hardware not available
    HardwareUnavailable {
        reason: String,
    },
    /// Attestation verification failed
    AttestationError {
        reason: String,
    },
    /// Invalid key parameters
    InvalidParameters {
        parameter: String,
        reason: String,
    },
    /// Device not supported
    UnsupportedDevice {
        manufacturer: String,
        model: String,
        reason: String,
    },
}

/// Main Android StrongBox HSM structure
///
/// This structure represents the Android StrongBox HSM implementation with
/// hardware-backed key operations on GrapheneOS/Pixel devices.
pub struct AndroidStrongBoxHsm {
    pub config: AndroidHsmConfig,
    pub keystore: Arc<AndroidKeystore>,
    pub attestation_service: Arc<AndroidAttestationService>,
    pub device_info: Arc<AndroidDeviceInfo>,
    pub key_cache: Arc<RwLock<HashMap<String, CachedKeyInfo>>>,
    pub health_monitor: Arc<AndroidHealthMonitor>,
}

/// Android Keystore integration structure
///
/// Handles communication with the Android Keystore service and StrongBox.
pub struct AndroidKeystore {
    pub config: KeystoreConfig,
    pub strongbox_available: bool,
    pub strongbox_implementation: StrongBoxImplementation,
}

/// Android Attestation Service structure
///
/// Manages key attestation and certificate chain verification.
pub struct AndroidAttestationService {
    pub config: AttestationConfig,
    pub trusted_certificates: Vec<Vec<u8>>,
    pub challenge_generator: Arc<ChallengeGenerator>,
}

/// Android Device Information structure
///
/// Contains information about the Android device and its security capabilities.
pub struct AndroidDeviceInfo {
    pub manufacturer: String,
    pub model: String,
    pub android_version: String,
    pub strongbox_version: Option<String>,
    pub titan_m_version: Option<String>,
    pub security_patch_level: String,
    pub verified_boot_state: VerifiedBootState,
}

/// Cached key information for Android StrongBox
#[derive(Debug, Clone)]
pub struct CachedKeyInfo {
    pub key_id: String,
    pub key_type: KeyType,
    pub hsm_type: HsmTier,
    pub strongbox_backed: bool,
    pub attestation_verified: bool,
    pub last_used: chrono::DateTime<Utc>,
    pub usage_count: u64,
    pub created_at: chrono::DateTime<Utc>,
    pub usage_policy: KeyUsagePolicy,
    pub health_status: KeyHealthStatus,
}

/// Android Health Monitor structure
///
/// Monitors the health of various Android StrongBox components.
pub struct AndroidHealthMonitor {
    pub keystore_health: Arc<RwLock<HsmHealthStatus>>,
    pub strongbox_health: Arc<RwLock<HsmHealthStatus>>,
    pub attestation_health: Arc<RwLock<HsmHealthStatus>>,
}

/// Challenge Generator structure
///
/// Generates cryptographic challenges for attestation and authentication.
pub struct ChallengeGenerator {
    pub entropy_source: Arc<dyn EntropySource>,
}

/// Trait for providing cryptographic entropy.
pub trait EntropySource: Send + Sync {
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
    Green,  // Verified boot with locked bootloader
    Yellow, // Verified boot with unlocked bootloader
    Orange, // Custom OS
    Red,    // Boot failure
    Unknown,
}

/// Android Key Parameters structure
///
/// Configuration parameters for Android Keystore key generation.
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

/// Android Key Algorithm enumeration
///
/// Supported cryptographic algorithms in Android StrongBox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidKeyAlgorithm {
    Ec,
    Rsa,
    Aes,
}

/// Android Key Purpose enumeration
///
/// Specifies the allowed purposes for an Android key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidKeyPurpose {
    Encrypt,
    Decrypt,
    Sign,
    Verify,
    Wrap,
    Unwrap,
}

/// Android Elliptic Curve enumeration
///
/// Supported elliptic curves in Android StrongBox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidEcCurve {
    P256,
    P384,
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
            message: format!("StrongBox error: {:?}", err),
        }
    }
}

impl StrongBoxError {
    pub fn from_strongbox_error(code: i32, message: &str) -> BearDogError {
        BearDogError::Hsm {
            message: format!("StrongBox error (code {}): {}", code, message),
        }
    }
} 