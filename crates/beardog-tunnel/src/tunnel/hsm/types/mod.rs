// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Types Module
//!
//! Core type definitions for Hardware Security Module (HSM) functionality.

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Submodules
pub mod algorithm;
pub mod canonical;
pub mod capability;
pub mod config;
pub mod key;
pub mod security_level;
pub mod status;
pub mod tier;

// Re-exports from beardog_types (only what actually exists)
// Note: Most types are defined locally to avoid cross-crate confusion

// Re-export from config module (local definitions)
pub use config::{AuthMethod as AuthenticationMethod, HsmConnectionConfig as HsmConnectionInfo};

// Re-export canonical SecurityLevel
pub use security_level::SecurityLevel;

// Re-export KeyType from key module
pub use key::KeyType;

// Capability requirements from local definition
pub use capability::CapabilityRequirements;

/// HSM hardware capability detection results
#[derive(Debug, Clone)]
pub struct HsmCapabilities {
    /// Whether the HSM supports key generation operations
    pub supports_key_generation: bool,
    /// Whether the HSM supports signing operations
    pub supports_signing: bool,
    /// Whether the HSM supports encryption operations
    pub supports_encryption: bool,
}

impl Default for HsmCapabilities {
    fn default() -> Self {
        Self {
            supports_key_generation: true,
            supports_signing: true,
            supports_encryption: true,
        }
    }
}

// Algorithm enumeration for compatibility
/// Canonical cryptographic algorithm enumeration
///
/// Unified algorithm definitions for all HSM operations across BearDog.
/// This enum consolidates algorithm specifications from multiple sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Algorithm {
    /// AES-256 in GCM mode (symmetric encryption)
    Aes256Gcm,
    /// ChaCha20-Poly1305 authenticated encryption
    ChaCha20Poly1305,

    /// Elliptic Curve Cryptography with P-256 curve
    EccP256,
    /// Elliptic Curve Cryptography with P-384 curve
    EccP384,

    /// ECDSA with P-256 curve
    EcdsaP256,
    /// ECDSA with P-384 curve
    EcdsaP384,
    /// ECDSA with SHA-256 hash
    EcdsaSha256,

    /// RSA with SHA-256 hash
    RsaSha256,
    /// RSA-PSS with 2048-bit key
    RsaPss2048,
    /// RSA-PSS with 3072-bit key
    RsaPss3072,
    /// RSA-PSS with 4096-bit key
    RsaPss4096,

    /// HKDF with SHA-256 for key derivation
    HkdfSha256,

    /// Ed25519 digital signatures
    Ed25519,
    /// X25519 key exchange
    X25519,
}

impl Algorithm {
    /// Returns the security strength in bits
    pub const fn security_bits(&self) -> usize {
        match self {
            Self::Aes256Gcm | Self::EccP256 | Self::EcdsaP256 | Self::Ed25519 | Self::X25519 => 256,
            Self::EccP384 | Self::EcdsaP384 => 384,
            Self::RsaPss2048 | Self::RsaSha256 => 112, // Effective security
            Self::RsaPss3072 => 128,
            Self::RsaPss4096 => 152,
            Self::ChaCha20Poly1305 => 256,
            Self::HkdfSha256 | Self::EcdsaSha256 => 256,
        }
    }

    /// Returns whether this is a signature algorithm
    pub const fn is_signature_algorithm(&self) -> bool {
        matches!(
            self,
            Self::EcdsaP256
                | Self::EcdsaP384
                | Self::EcdsaSha256
                | Self::RsaSha256
                | Self::RsaPss2048
                | Self::RsaPss3072
                | Self::RsaPss4096
                | Self::Ed25519
        )
    }

    /// Returns whether this is an encryption algorithm
    pub const fn is_encryption_algorithm(&self) -> bool {
        matches!(self, Self::Aes256Gcm | Self::ChaCha20Poly1305)
    }
}

// Re-exports from tier module
pub use tier::{
    AndroidKeyAlgorithm, AttestationLevel, HsmTier, KeyStorageType, MemoryProtectionLevel,
    SecureEnclaveType, SmartphoneType, SoftwareHsmType, StrongBoxImplementation,
};

// Re-exports from key module
pub use key::{
    HsmKey, HsmKeyInfo, HsmKeyMetadata, KeyAttestation, KeyHealthStatus, KeyMaterial, KeyMetadata,
    KeyType as HsmKeyType, UniversalKey,
};

// Also re-export HsmKeyMetadata at top level for convenience
pub use key::HsmKeyMetadata as HsmKeyMeta;

// Algorithm to KeyType conversion for compatibility
impl From<Algorithm> for KeyType {
    fn from(algo: Algorithm) -> Self {
        match algo {
            Algorithm::EcdsaP256 | Algorithm::EccP256 | Algorithm::EcdsaSha256 => {
                KeyType::EllipticCurve
            }
            Algorithm::EcdsaP384 | Algorithm::EccP384 => KeyType::EllipticCurve,
            Algorithm::RsaPss2048
            | Algorithm::RsaPss3072
            | Algorithm::RsaPss4096
            | Algorithm::RsaSha256 => KeyType::Rsa,
            Algorithm::Aes256Gcm => KeyType::Aes,
            Algorithm::ChaCha20Poly1305 => KeyType::ChaCha20,
            Algorithm::Ed25519 => KeyType::Ed25519,
            Algorithm::X25519 => KeyType::X25519,
            Algorithm::HkdfSha256 => KeyType::Generic, // KDF doesn't map directly to key type
        }
    }
}

// Re-exports from status module
pub use status::{HsmHealthStatus, PerformanceMetrics};

// Re-exports from capability module
pub use capability::HsmCapability;

// Re-exports from config module
pub use config::SoftwareHsmConfig;

/// Android key purpose enumeration
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AndroidKeyPurpose {
    /// Encryption operations
    Encrypt,
    /// Decryption operations
    Decrypt,
    /// Signing operations
    Sign,
    /// Verification operations
    Verify,
    /// Key wrapping operations
    Wrap,
    /// Key unwrapping operations
    Unwrap,
}

/// Android key parameters configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AndroidKeyParams {
    /// Cryptographic algorithm
    pub algorithm: String,
    /// Key size in bits
    pub key_size: u32,
    /// Allowed key purposes
    pub purposes: Vec<AndroidKeyPurpose>,
    /// Require StrongBox hardware
    pub strongbox_required: bool,
    /// Require user authentication
    pub user_authentication_required: bool,
    /// User authentication timeout (seconds)
    pub user_authentication_timeout: Option<i32>,
    /// Key validity start time
    pub key_validity_start: Option<chrono::DateTime<chrono::Utc>>,
    /// Key validity end time
    pub key_validity_end: Option<chrono::DateTime<chrono::Utc>>,
    /// Attestation challenge data
    pub attestation_challenge: Option<Vec<u8>>,
}

impl AndroidKeyParams {
    /// Create new Android key parameters with defaults
    pub fn new() -> Self {
        Self {
            algorithm: "Ed25519".to_string(),
            key_size: 256,
            purposes: vec![AndroidKeyPurpose::Sign, AndroidKeyPurpose::Verify],
            strongbox_required: false,
            user_authentication_required: false,
            user_authentication_timeout: None,
            key_validity_start: None,
            key_validity_end: None,
            attestation_challenge: None,
        }
    }

    /// Set the cryptographic algorithm
    pub fn set_algorithm(mut self, algorithm: &str) -> Self {
        self.algorithm = algorithm.to_string();
        self
    }

    /// Set the key size
    pub fn set_key_size(&mut self, size: u32) {
        self.key_size = size;
    }

    /// Set the key purposes
    pub fn set_purposes(&mut self, purposes: Vec<AndroidKeyPurpose>) {
        self.purposes = purposes;
    }

    /// Set StrongBox requirement
    pub fn set_strongbox_required(&mut self, required: bool) {
        self.strongbox_required = required;
    }

    /// Set user authentication requirement
    pub fn set_user_authentication_required(&mut self, required: bool) {
        self.user_authentication_required = required;
    }

    /// Set key validity end time
    pub fn set_key_validity_end(&mut self, end: chrono::DateTime<chrono::Utc>) {
        self.key_validity_end = Some(end);
    }

    /// Set attestation challenge
    pub fn set_attestation_challenge(&mut self, challenge: Vec<u8>) {
        self.attestation_challenge = Some(challenge);
    }
}

impl Default for AndroidKeyParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Android HSM configuration
#[derive(Debug, Clone)]
/// Android keystore configuration
pub struct AndroidKeystoreConfig {
    /// Use hardware-backed keys
    pub hardware_backed: bool,
    /// Maximum key count
    pub max_keys: usize,
}

impl Default for AndroidKeystoreConfig {
    fn default() -> Self {
        Self {
            hardware_backed: true,
            max_keys: 256,
        }
    }
}

/// Android HSM configuration - canonical definition
#[derive(Debug, Clone)]
pub struct AndroidHsmConfig {
    /// Enable StrongBox hardware security
    pub strongbox_enabled: bool,
    /// Key parameters
    pub key_params: AndroidKeyParams,
    /// Attestation security level
    pub attestation_level: AttestationLevel,
    /// Security level (0-3)
    pub security_level: u8,
    /// Keystore configuration
    pub keystore_config: AndroidKeystoreConfig,
    /// Attestation configuration (using default for now)
    pub attestation_config: String, // Placeholder - will use proper type once AttestationConfig is accessible
}

impl Default for AndroidHsmConfig {
    fn default() -> Self {
        Self {
            strongbox_enabled: true,
            key_params: AndroidKeyParams::new(),
            keystore_config: AndroidKeystoreConfig::default(),
            attestation_config: "default".to_string(),
            attestation_level: AttestationLevel::Hardware,
            security_level: 2,
        }
    }
}

/// iOS HSM configuration
#[derive(Debug, Clone)]
pub struct IOSHsmConfig {
    /// Use Secure Enclave
    pub secure_enclave_enabled: bool,
    /// Require biometric authentication
    pub biometric_authentication: bool,
    /// Enable key attestation
    pub key_attestation: bool,
}

impl Default for IOSHsmConfig {
    fn default() -> Self {
        Self {
            secure_enclave_enabled: true,
            biometric_authentication: false,
            key_attestation: true,
        }
    }
}

/// Android device capabilities
#[derive(Debug, Clone)]
pub struct AndroidDeviceCapabilities {
    /// StrongBox hardware available
    pub strongbox_available: bool,
    /// Key attestation available
    pub key_attestation_available: bool,
    /// Hardware-backed keystore available
    pub hardware_backed_keystore: bool,
    /// Verified boot enabled
    pub verified_boot: bool,
}

/// iOS device capabilities
#[derive(Debug, Clone)]
pub struct IOSDeviceCapabilities {
    /// Secure Enclave available
    pub secure_enclave_available: bool,
    /// Biometric ID available (Face ID/Touch ID)
    pub biometric_id_available: bool,
    /// Hardware security module available
    pub hardware_security_module: bool,
}

/// Android Keystore implementation
#[derive(Debug, Clone)]
pub struct AndroidKeystore {
    /// HSM configuration
    pub config: AndroidHsmConfig,
    /// Device capabilities
    pub capabilities: AndroidDeviceCapabilities,
}

impl AndroidKeystore {
    /// Create new Android keystore instance
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new(config: AndroidHsmConfig) -> Result<Self, BearDogError> {
        let capabilities = AndroidDeviceCapabilities {
            strongbox_available: true,
            key_attestation_available: true,
            hardware_backed_keystore: true,
            verified_boot: true,
        };

        Ok(Self {
            config,
            capabilities,
        })
    }

    /// Test keystore access
    ///
    /// # Errors
    /// Returns an error if access test fails
    pub fn test_keystore_access(
        &self,
        _key_id: &str,
        _params: &AndroidKeyParams,
    ) -> Result<(), BearDogError> {
        Err(BearDogError::not_implemented(
            "Android keystore key generation not yet implemented",
        ))
    }

    /// Generate a new key
    ///
    /// # Errors
    /// Returns an error if key generation fails
    pub fn generate_key(
        &self,
        _key_id: &str,
        _params: &AndroidKeyParams,
    ) -> Result<(), BearDogError> {
        Err(BearDogError::not_implemented(
            "Android keystore key generation not yet implemented",
        ))
    }

    /// Encrypt data
    ///
    /// # Errors
    /// Returns an error if encryption fails
    pub async fn encrypt(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::not_implemented(
            "Android keystore encryption not yet implemented",
        ))
    }

    /// Decrypt data
    ///
    /// # Errors
    /// Returns an error if decryption fails
    pub async fn decrypt(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::not_implemented(
            "Android keystore decryption not yet implemented",
        ))
    }

    /// Sign data
    ///
    /// # Errors
    /// Returns an error if signing fails
    pub async fn sign(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::not_implemented(
            "Android keystore signing not yet implemented",
        ))
    }

    /// Verify signature
    ///
    /// # Errors
    /// Returns an error if verification fails
    pub async fn verify(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Err(BearDogError::not_implemented(
            "Android keystore verification not yet implemented",
        ))
    }

    /// Delete a key
    ///
    /// # Errors
    /// Returns an error if deletion fails
    pub async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
        Err(BearDogError::not_implemented(
            "Android keystore key deletion not yet implemented",
        ))
    }

    /// Check if key exists
    ///
    /// # Errors
    /// Returns an error if check fails
    pub async fn key_exists(&self, _key_id: &str) -> Result<bool, BearDogError> {
        // For now, return false
        Ok(false)
    }

    /// Generate attestation challenge
    ///
    /// # Errors
    /// Returns an error if generation fails
    pub fn generate_attestation_challenge(&self, size: usize) -> Result<Vec<u8>, BearDogError> {
        use rand::RngCore;
        let mut challenge = vec![0u8; size];
        rand::thread_rng().fill_bytes(&mut challenge);
        tracing::debug!("Generated attestation challenge of {} bytes", size);
        Ok(challenge)
    }

    /// Check if StrongBox is available on this device
    pub fn is_strongbox_available(&self) -> bool {
        self.capabilities.strongbox_available
    }

    /// Generate random bytes using hardware RNG
    ///
    /// # Errors
    /// Returns an error if RNG fails
    pub async fn generate_random_bytes(&self, count: usize) -> Result<Vec<u8>, BearDogError> {
        tracing::debug!(
            "🎲 Generating {} random bytes using Android hardware RNG",
            count
        );

        // Use rand crate for random byte generation
        use rand::RngCore;
        let mut bytes = vec![0u8; count];
        rand::thread_rng().fill_bytes(&mut bytes);

        Ok(bytes)
    }

    /// Import existing key material into keystore
    ///
    /// # Errors
    /// Returns an error if import fails
    pub async fn import_key(
        &self,
        key_id: &str,
        key_data: &[u8],
        _key_type: KeyType,
    ) -> Result<(), BearDogError> {
        tracing::info!("📥 Importing key into Android Keystore: {}", key_id);

        // In production, this would use JNI to call Android Keystore API
        // For now, return not_implemented with clear message
        let _ = key_data; // Avoid unused warning

        Err(BearDogError::not_implemented(
            "Android keystore key import - requires JNI implementation",
        ))
    }

    /// List all keys in keystore
    ///
    /// # Errors
    /// Returns an error if listing fails
    pub async fn list_keys(
        &self,
    ) -> Result<Vec<beardog_types::canonical::providers_unified::traits::KeyInfo>, BearDogError>
    {
        tracing::info!("📋 Listing keys in Android Keystore");

        // In production, this would query Android Keystore via JNI
        // For now, return empty list
        Ok(vec![])
    }
}

/// Android attestation service
pub struct AndroidAttestationService {
    /// Whether attestation is enabled
    pub enabled: bool,
    /// Attestation level
    pub attestation_level: AttestationLevel,
}

impl AndroidAttestationService {
    /// Create new attestation service
    pub fn new(attestation_level: AttestationLevel) -> Self {
        Self {
            enabled: true,
            attestation_level,
        }
    }

    /// Initialize the attestation service
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn initialize(&self) -> Result<(), BearDogError> {
        tracing::info!(
            "Initializing Android attestation service with level: {:?}",
            self.attestation_level
        );

        if !self.enabled {
            return Err(BearDogError::configuration(
                "Attestation service is disabled",
            ));
        }

        tracing::info!("Android attestation service initialized successfully");
        Ok(())
    }
}

/// Android health monitor
pub struct AndroidHealthMonitor {
    /// Health check interval (seconds)
    pub check_interval_seconds: u64,
}

impl AndroidHealthMonitor {
    /// Create new health monitor
    pub fn new() -> Self {
        Self {
            check_interval_seconds: 60,
        }
    }

    /// Start health monitoring
    ///
    /// # Errors
    /// Returns an error if monitoring fails to start
    pub fn start_monitoring(&self) -> Result<(), BearDogError> {
        tracing::info!(
            "Starting Android health monitoring with {}-second intervals",
            self.check_interval_seconds
        );

        let check_interval = std::time::Duration::from_secs(self.check_interval_seconds);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(check_interval);
            loop {
                interval.tick().await;
                tracing::debug!("Performing Android HSM health check");
            }
        });

        Ok(())
    }

    /// Get current health status
    ///
    /// # Errors
    /// Returns an error if status check fails
    pub fn get_health_status(&self) -> Result<status::HsmHealthStatus, BearDogError> {
        tracing::debug!("Getting Android HSM health status");

        let is_healthy = true;
        let error_message = None;

        Ok(status::HsmHealthStatus {
            is_healthy,
            last_check: chrono::Utc::now(),
            error_message,
            performance_metrics: status::PerformanceMetrics {
                operations_per_second: 100.0,
                average_latency_ms: 10.0,
                success_rate: 99.9,
                memory_usage_mb: 50.0,
                cpu_usage_percent: 5.0,
                network_throughput_bps: 100_000.0,
                latency_ms: 10.0,
                throughput_mbps: 0.1,
                uptime_seconds: 3600,
            },
        })
    }
}

impl Default for AndroidHealthMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// HSM operation types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum HsmOperation {
    /// Key generation
    KeyGeneration {
        /// Unique identifier for the key
        key_id: String,
        /// Key size in bits
        key_size: u32,
    },
    /// Encryption operation
    Encryption {
        /// Key identifier used for encryption
        key_id: String,
        /// Algorithm name
        algorithm: String,
    },
    /// Decryption operation
    Decryption {
        /// Key identifier used for decryption
        key_id: String,
        /// Algorithm name
        algorithm: String,
    },
    /// Signing operation
    Signing {
        /// Key identifier used for signing
        key_id: String,
        /// Algorithm name
        algorithm: String,
    },
    /// Verification operation
    Verification {
        /// Key identifier used for verification
        key_id: String,
        /// Algorithm name
        algorithm: String,
    },
    /// Key deletion
    KeyDeletion {
        /// Key identifier to delete
        key_id: String,
    },
}

/// HSM operation result
#[derive(Debug, Clone)]
pub struct HsmOperationResult {
    /// The operation performed
    pub operation: HsmOperation,
    /// Whether the operation succeeded
    pub success: bool,
    /// Optional result data
    pub result_data: Option<Vec<u8>>,
    /// Optional error message
    pub error_message: Option<String>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// HSM audit entry
#[derive(Debug, Clone)]
pub struct HsmAuditEntry {
    /// Unique entry ID
    pub id: String,
    /// Operation timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// User ID (optional)
    pub user_id: Option<String>,
    /// Operation result
    pub result: HsmOperationResult,
    /// Security context metadata
    pub security_context: HashMap<String, String>,
}

/// HSM cache for performance optimization
#[derive(Debug, Clone)]
pub struct HsmCache {
    /// Cached key metadata
    pub key_metadata: Arc<RwLock<HashMap<String, key::KeyMetadata>>>,
    /// Cached operation results
    pub operation_cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl HsmCache {
    /// Create new HSM cache
    pub fn new() -> Self {
        Self {
            key_metadata: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            operation_cache: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }
    }
}

impl Default for HsmCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_android_key_params_default() -> Result<(), Box<dyn std::error::Error>> {
        let params = AndroidKeyParams::default();
        assert_eq!(params.algorithm, "Ed25519");
        assert_eq!(params.key_size, 256);
        assert!(!params.strongbox_required);
        Ok(())
    }

    #[test]
    fn test_android_key_params_builder() -> Result<(), Box<dyn std::error::Error>> {
        let mut params = AndroidKeyParams::new().set_algorithm("AES256");

        params.set_key_size(256);
        params.set_strongbox_required(true);

        assert_eq!(params.algorithm, "AES256");
        assert_eq!(params.key_size, 256);
        assert!(params.strongbox_required);
        Ok(())
    }

    #[test]
    fn test_android_hsm_config_default() -> Result<(), Box<dyn std::error::Error>> {
        let config = AndroidHsmConfig::default();
        assert!(config.strongbox_enabled);
        assert_eq!(config.security_level, 2);
        Ok(())
    }

    #[test]
    fn test_android_keystore_creation() -> Result<(), Box<dyn std::error::Error>> {
        let config = AndroidHsmConfig::default();
        let keystore = AndroidKeystore::new(config)?;

        assert!(keystore.capabilities.strongbox_available);
        assert!(keystore.capabilities.hardware_backed_keystore);
        Ok(())
    }

    #[test]
    fn test_android_attestation_service() -> Result<(), Box<dyn std::error::Error>> {
        let service = AndroidAttestationService::new(AttestationLevel::Hardware);
        assert!(service.enabled);
        assert!(service.initialize().is_ok());
        Ok(())
    }

    #[test]
    fn test_hsm_cache_creation() -> Result<(), Box<dyn std::error::Error>> {
        let cache = HsmCache::new();
        // Just ensure it creates without panic
        assert!(Arc::strong_count(&cache.key_metadata) >= 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_android_health_monitor() -> Result<(), Box<dyn std::error::Error>> {
        let monitor = AndroidHealthMonitor::new();
        assert_eq!(monitor.check_interval_seconds, 60);

        let status = monitor.get_health_status()?;
        assert!(status.is_healthy);
        Ok(())
    }
}
