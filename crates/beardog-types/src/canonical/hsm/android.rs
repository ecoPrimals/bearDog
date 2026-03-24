// SPDX-License-Identifier: AGPL-3.0-only

//! Android Keystore, StrongBox, and attestation-facing configuration types.

use crate::constants::defaults;
use crate::constants::time;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Snapshot of the handset capabilities relevant to hardware-backed keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidDeviceInfo {
    /// Device model
    /// The model value
    pub model: String,
    /// Android API level
    /// Number of `api_level`
    pub api_level: u32,
    /// Security patch level
    /// The security patch level value
    pub security_patch_level: String,
    /// Device manufacturer
    /// The manufacturer value
    pub manufacturer: String,
    /// Hardware features available
    /// Collection of hardware features
    pub hardware_features: Vec<String>,
    /// Verified boot state
    /// Whether `verified_boot` is enabled
    pub verified_boot: bool,
    /// Device integrity attestation
    /// The device integrity value
    pub device_integrity: DeviceIntegrity,
}

impl Default for AndroidDeviceInfo {
    fn default() -> Self {
        Self {
            model: "Unknown".to_string(),
            api_level: 30,
            security_patch_level: "2024-01-01".to_string(),
            manufacturer: "Unknown".to_string(),
            hardware_features: vec![
                "android.hardware.keystore".to_string(),
                "android.hardware.security.model.compatible".to_string(),
            ],
            verified_boot: true,
            device_integrity: DeviceIntegrity::default(),
        }
    }
}

/// Play Integrity / SafetyNet style verdict flags used when gating key creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceIntegrity {
    /// Basic integrity verdict
    /// Whether `basic_integrity` is enabled
    pub basic_integrity: bool,
    /// CTS profile match
    /// Whether `cts_profile_match` is enabled
    pub cts_profile_match: bool,
    /// Evaluation type
    /// The evaluation type value
    pub evaluation_type: String,
    /// Integrity verdict details
    /// Collection of verdict details
    pub verdict_details: Vec<String>,
}

impl Default for DeviceIntegrity {
    fn default() -> Self {
        Self {
            basic_integrity: true,
            cts_profile_match: true,
            evaluation_type: "BASIC".to_string(),
            verdict_details: Vec::new(),
        }
    }
}

/// Android Keystore configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidKeystoreConfig {
    /// Keystore provider name
    pub provider: String,
    /// Hardware-backed keys required
    /// Whether `hardware_backed_required` is enabled
    pub hardware_backed_required: bool,
    /// Supported key algorithms
    /// Collection of supported algorithms
    pub supported_algorithms: Vec<String>,
    /// Key attestation required
    /// Whether `attestation_required` is enabled
    pub attestation_required: bool,
    /// Strongbox required
    /// Whether `strongbox_required` is enabled
    pub strongbox_required: bool,
    /// Whether `user_auth_required` is enabled
    pub user_auth_required: bool,
    /// Biometric authentication timeout (seconds)
    pub biometric_timeout_seconds: Option<u32>,
}

impl Default for AndroidKeystoreConfig {
    fn default() -> Self {
        Self {
            provider: "AndroidKeyStore".to_string(),
            hardware_backed_required: true,
            supported_algorithms: vec![
                "AES".to_string(),
                "RSA".to_string(),
                "EC".to_string(),
                "HMAC".to_string(),
            ],
            attestation_required: true,
            strongbox_required: false,
            user_auth_required: true,
            biometric_timeout_seconds: Some(300), // 5 minutes
        }
    }
}

/// `StrongBox` security module configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrongBoxConfig {
    /// `StrongBox` available
    /// Whether available is enabled
    pub available: bool,
    /// When true, high-value keys must be created in StrongBox if present. **Default:** `false`.
    pub required_for_sensitive: bool,
    /// Supported key types in `StrongBox`
    /// Collection of supported key types
    pub supported_key_types: Vec<String>,
    /// Maximum key operations per second
    /// Number of `max_ops_per_second`
    pub max_ops_per_second: u32,
    /// `StrongBox` version
    /// The version value
    pub version: String,
}

impl Default for StrongBoxConfig {
    fn default() -> Self {
        Self {
            available: false,
            required_for_sensitive: false,
            supported_key_types: vec!["AES".to_string(), "HMAC".to_string(), "EC".to_string()],
            max_ops_per_second: 100,
            version: "1.0".to_string(),
        }
    }
}

/// Biometric authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricConfig {
    /// Biometric authentication available
    /// Whether available is enabled
    pub available: bool,
    /// Supported biometric types
    /// Collection of supported types
    pub supported_types: Vec<BiometricType>,
    /// Authentication strength required
    /// The required strength value
    pub required_strength: BiometricStrength,
    /// Fallback to device credential allowed
    /// Whether `allow_device_credential` is enabled
    pub allow_device_credential: bool,
    /// Maximum authentication validity (seconds)
    pub max_validity_seconds: u32,
}

impl Default for BiometricConfig {
    fn default() -> Self {
        Self {
            available: true,
            supported_types: vec![BiometricType::Fingerprint, BiometricType::Face],
            required_strength: BiometricStrength::Strong,
            allow_device_credential: true,
            max_validity_seconds: 300, // 5 minutes
        }
    }
}

/// Biometric authentication types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of biometric
pub enum BiometricType {
    /// Fingerprint authentication
    Fingerprint,
    /// Face authentication
    Face,
    /// Iris authentication
    Iris,
    /// Voice authentication
    Voice,
}

/// Biometric authentication strength levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiometricStrength {
    /// Weak biometric authentication
    Weak,
    /// Strong biometric authentication
    Strong,
}

/// Android `SafetyNet` attestation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyNetConfig {
    /// `SafetyNet` attestation enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Vendor API key for server-side attestation verification (never log or persist in plaintext).
    pub api_key: Option<String>,
    /// Accepted attestation levels
    /// Collection of accepted attestation levels
    pub accepted_attestation_levels: Vec<String>,
    /// Nonce generation strategy
    /// The nonce strategy value
    pub nonce_strategy: NonceStrategy,
    /// Challenge timeout in seconds
    pub challenge_timeout_secs: u32,
    /// Verification endpoint
    /// Optional verification endpoint
    pub verification_endpoint: Option<String>,
}

impl Default for SafetyNetConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_key: None,
            accepted_attestation_levels: vec!["software".to_string(), "hardware".to_string()],
            nonce_strategy: NonceStrategy::Random,
            challenge_timeout_secs: 30,
            verification_endpoint: None,
        }
    }
}

/// How attestation nonces are derived for replay resistance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NonceStrategy {
    /// Generate a fresh cryptographically random nonce per attestation challenge.
    Random,
    /// Derive nonce from a time-bounded value (weaker replay resistance; prefer [`Self::Random`]).
    Timestamp,
    /// Custom nonce generation
    Custom {
        /// Custom nonce generation method identifier
        method: String,
    },
}

/// Android HSM session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidHsmSession {
    /// Session ID
    pub session_id: String,
    /// Associated device info
    /// The device info value
    pub device_info: AndroidDeviceInfo,
    /// Session creation time
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// Last access time
    /// The last access value
    pub last_access: DateTime<Utc>,
    /// Session timeout in seconds
    pub timeout_seconds: u32,
    /// Active key handles
    /// Collection of active keys
    pub active_keys: Vec<String>,
    /// Session metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

impl Default for AndroidHsmSession {
    fn default() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            device_info: AndroidDeviceInfo::default(),
            created_at: Utc::now(),
            last_access: Utc::now(),
            timeout_seconds: time::SECONDS_PER_HOUR as u32, // 1 hour
            active_keys: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

/// Comprehensive Android HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AndroidHsmConfig {
    /// The device info value
    pub device_info: AndroidDeviceInfo,
    /// Keystore configuration
    /// The keystore value
    pub keystore: AndroidKeystoreConfig,
    /// `StrongBox` configuration
    /// The strongbox value
    pub strongbox: StrongBoxConfig,
    /// Biometric configuration
    /// The biometric value
    pub biometric: BiometricConfig,
    /// `SafetyNet` configuration
    /// The safetynet value
    pub safetynet: SafetyNetConfig,
    /// Security requirements
    /// The security requirements value
    pub security_requirements: SecurityRequirements,
    /// Concurrency, caching, and batching for Android crypto sessions. **Default:** [`PerformanceSettings::default()`].
    pub performance: PerformanceSettings,
}

/// Policy floor before Android keys are considered trustworthy for Beardog workloads.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    /// Minimum Android API level required
    /// Number of `min_api_level`
    pub min_api_level: u32,
    /// Hardware attestation required
    /// Whether `hardware_attestation_required` is enabled
    pub hardware_attestation_required: bool,
    /// Verified boot required
    /// Whether `verified_boot_required` is enabled
    pub verified_boot_required: bool,
    /// Device integrity check required
    /// Whether `device_integrity_required` is enabled
    pub device_integrity_required: bool,
    /// Minimum security patch level
    /// Optional min security patch level
    pub min_security_patch_level: Option<String>,
    /// Allowed device manufacturers
    /// Optional allowed manufacturers
    pub allowed_manufacturers: Option<Vec<String>>,
}

impl Default for SecurityRequirements {
    fn default() -> Self {
        Self {
            min_api_level: 23, // Android 6.0 (Marshmallow)
            hardware_attestation_required: true,
            verified_boot_required: true,
            device_integrity_required: true,
            min_security_patch_level: Some("2023-01-01".to_string()),
            allowed_manufacturers: None, // Allow all by default
        }
    }
}

/// Tunable performance envelope for Android HSM-backed crypto calls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    /// Maximum concurrent operations
    /// Number of `max_concurrent_operations`
    pub max_concurrent_operations: u32,
    /// Operation timeout in milliseconds
    pub operation_timeout_ms: u32,
    /// Key cache size
    /// Number of `key_cache_size`
    pub key_cache_size: u32,
    /// Connection pool size
    /// Number of `connection_pool_size`
    pub connection_pool_size: u32,
    /// Enable operation batching
    /// Whether `enable_batching` is enabled
    pub enable_batching: bool,
    /// Number of `batch_size`
    pub batch_size: u32,
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        Self {
            max_concurrent_operations: 10,
            operation_timeout_ms: defaults::DEFAULT_MAX_RESPONSE_TIME_MS as u32, // 5 seconds
            key_cache_size: 100,
            connection_pool_size: 5,
            enable_batching: true,
            batch_size: 10,
        }
    }
}

impl AndroidHsmConfig {
    /// Create new Android HSM configuration with defaults
    #[must_use]
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the Android HSM configuration
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), String> {
        // Validate API level
        if self.device_info.api_level < self.security_requirements.min_api_level {
            return Err(format!(
                "Device API level {} is below minimum required {}",
                self.device_info.api_level, self.security_requirements.min_api_level
            ));
        }

        // Validate device integrity if required
        if self.security_requirements.device_integrity_required
            && !self.device_info.device_integrity.basic_integrity
        {
            return Err("Device integrity check failed".to_string());
        }

        // Validate verified boot if required
        if self.security_requirements.verified_boot_required && !self.device_info.verified_boot {
            return Err("Verified boot is required but not enabled".to_string());
        }

        // Validate manufacturer if restricted
        if let Some(ref allowed) = self.security_requirements.allowed_manufacturers
            && !allowed.contains(&self.device_info.manufacturer)
        {
            return Err(format!(
                "Device manufacturer {} is not in allowed list",
                self.device_info.manufacturer
            ));
        }

        // Validate StrongBox requirements
        if self.strongbox.required_for_sensitive && !self.strongbox.available {
            return Err("StrongBox is required but not available".to_string());
        }

        Ok(())
    }

    /// Check if the device meets security requirements
    #[must_use]
    pub fn meets_security_requirements(&self) -> bool {
        self.validate().is_ok()
    }

    /// Get the effective security level
    #[must_use]
    pub const fn security_level(&self) -> SecurityLevel {
        if self.strongbox.available && self.strongbox.required_for_sensitive {
            SecurityLevel::StrongBox
        } else if self.keystore.hardware_backed_required {
            SecurityLevel::HardwareBacked
        } else {
            SecurityLevel::Software
        }
    }

    /// Check if biometric authentication is available and configured
    #[must_use]
    pub fn biometric_available(&self) -> bool {
        self.biometric.available && !self.biometric.supported_types.is_empty()
    }

    /// Get supported key algorithms
    #[must_use]
    pub fn supported_algorithms(&self) -> &[String] {
        &self.keystore.supported_algorithms
    }
}

/// Android HSM security levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SecurityLevel {
    /// Software-only implementation
    Software,
    /// Hardware-backed security
    #[default]
    HardwareBacked,
    /// `StrongBox` security module
    StrongBox,
}
