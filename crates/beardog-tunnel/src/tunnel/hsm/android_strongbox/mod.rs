//! # Android StrongBox HSM Implementation
//!
//! This module provides Hardware Security Module (HSM) integration with Android StrongBox,
//! specifically optimized for GrapheneOS on Pixel devices with Titan M security chips.
//!
//! ## Features
//!
//! - **Hardware-backed key generation** using Android StrongBox
//! - **Key attestation** with certificate chain verification
//! - **Biometric authentication** integration
//! - **User presence validation** for sensitive operations
//! - **Secure key storage** in hardware-backed keystore
//! - **GrapheneOS optimizations** for enhanced security
//!
//! ## GrapheneOS/Pixel 8a Integration
//!
//! GrapheneOS provides excellent StrongBox support with:
//! - Titan M security chip integration
//! - Hardware-backed key attestation
//! - Enhanced user privacy controls
//! - Secure boot and verified boot
//! - Hardware-backed keystore
//!
//! ## Security Model
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    Android Application                          │
//! └─────────────────────────┬───────────────────────────────────────┘
//!                           │
//! ┌─────────────────────────▼───────────────────────────────────────┐
//! │                BearDog StrongBox HSM                           │
//! └─────────────────────────┬───────────────────────────────────────┘
//!                           │
//! ┌─────────────────────────▼───────────────────────────────────────┐
//! │              Android Keystore Service                          │
//! └─────────────────────────┬───────────────────────────────────────┘
//!                           │
//! ┌─────────────────────────▼───────────────────────────────────────┐
//! │                 Android StrongBox                              │
//! │              (Titan M Security Chip)                          │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Module Organization
//!
//! The Android StrongBox implementation is organized into focused modules:
//!
//! - [`types`] - Core types, structures, and enums
//! - [`core`] - Main AndroidStrongBoxHsm implementation and HsmProvider trait
//! - [`keystore`] - Android Keystore integration and operations
//! - [`attestation`] - Key attestation and certificate chain verification
//! - [`device_info`] - Android device information and capability detection
//! - [`health`] - Health monitoring for all StrongBox components
//! - [`entropy`] - Cryptographic entropy and challenge generation
//!
//! ## Usage Examples
//!
//! ### Basic HSM Initialization
//!
//! ```rust,no_run
//! use beardog::tunnel::hsm::android_strongbox::AndroidStrongBoxHsm;
//! use beardog::tunnel::hsm::types::*;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Configure Android HSM
//! let config = AndroidHsmConfig {
//!     keystore_config: KeystoreConfig::default(),
//!     attestation_config: AttestationConfig::default(),
//! };
//!
//! // Initialize StrongBox HSM
//! let hsm = AndroidStrongBoxHsm::new(config).await?;
//!
//! // Generate a hardware-backed key
//! let key_request = GenerateKeyRequest {
//!     key_id: "my_secure_key".to_string(),
//!     key_type: KeyType::EccP256,
//!     usage_policy: KeyUsagePolicy::default(),
//!     include_attestation: Some(true),
//!     expires_at: None,
//!     metadata: KeyMetadata::default(),
//! };
//!
//! let key = hsm.generate_key(key_request).await?;
//! println!("Generated StrongBox key: {}", key.id);
//! # Ok(())
//! # }
//! ```
//!
//! ### Key Attestation
//!
//! ```rust,no_run
//! # use beardog::tunnel::hsm::android_strongbox::AndroidStrongBoxHsm;
//! # use beardog::tunnel::hsm::types::*;
//! # async fn example(hsm: AndroidStrongBoxHsm) -> Result<(), Box<dyn std::error::Error>> {
//! // Generate key with attestation
//! let key_request = GenerateKeyRequest {
//!     key_id: "attested_key".to_string(),
//!     key_type: KeyType::EccP256,
//!     usage_policy: KeyUsagePolicy::default(),
//!     include_attestation: Some(true),
//!     expires_at: None,
//!     metadata: KeyMetadata::default(),
//! };
//!
//! let key = hsm.generate_key(key_request).await?;
//!
//! if let Some(attestation) = &key.attestation {
//!     println!("Key has hardware attestation");
//!     println!("Certificate chain length: {}", attestation.certificate_chain.len());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Cryptographic Operations
//!
//! ```rust,no_run
//! # use beardog::tunnel::hsm::android_strongbox::AndroidStrongBoxHsm;
//! # use beardog::tunnel::hsm::types::*;
//! # async fn example(hsm: AndroidStrongBoxHsm) -> Result<(), Box<dyn std::error::Error>> {
//! let data = b"Hello, StrongBox!";
//!
//! // Sign data with hardware-backed key
//! let signature = hsm.sign("my_secure_key", data).await?;
//! println!("Signature length: {} bytes", signature.len());
//!
//! // Verify signature
//! let valid = hsm.verify("my_secure_key", data, &signature).await?;
//! println!("Signature valid: {}", valid);
//!
//! // Encrypt data (for symmetric keys)
//! let ciphertext = hsm.encrypt("symmetric_key", data).await?;
//! let plaintext = hsm.decrypt("symmetric_key", &ciphertext).await?;
//! assert_eq!(data, &plaintext[..]);
//! # Ok(())
//! # }
//! ```
//!
//! ## Security Considerations
//!
//! ### Hardware Requirements
//!
//! - **StrongBox Support**: Requires Android device with StrongBox implementation
//! - **Titan M**: Optimal support on Google Pixel devices with Titan M chip
//! - **Verified Boot**: Best security with GREEN verified boot state
//! - **Android Version**: Requires Android 9+ for full StrongBox support
//!
//! ### Key Properties
//!
//! - **Hardware-bound**: Keys cannot be extracted from StrongBox
//! - **Attestation**: Keys can be attested to prove hardware backing
//! - **User Presence**: Optional user presence validation for operations
//! - **Tamper Resistance**: Hardware protection against physical attacks
//!
//! ### Operational Security
//!
//! - **No Key Export**: StrongBox keys cannot be backed up or exported
//! - **Device Binding**: Keys are permanently bound to the device
//! - **Secure Operations**: All cryptographic operations occur in hardware
//! - **Audit Logging**: Comprehensive logging of all HSM operations
//!
//! ## Performance Characteristics
//!
//! ### Operation Latencies (Typical)
//!
//! - **Key Generation**: 50-200ms (depending on key type)
//! - **Signing**: 5-20ms (ECDSA), 10-50ms (RSA)
//! - **Verification**: 3-15ms (ECDSA), 5-30ms (RSA)
//! - **Encryption/Decryption**: 1-10ms (AES), 10-50ms (RSA)
//! - **Attestation**: 100-500ms (includes certificate chain)
//!
//! ### Throughput Limits
//!
//! - **Concurrent Operations**: Limited by StrongBox hardware
//! - **Key Storage**: ~1000 keys maximum in Android Keystore
//! - **Operation Rate**: ~100-200 operations/second typical
//!
//! ## Error Handling
//!
//! The Android StrongBox implementation provides comprehensive error handling:
//!
//! - **Hardware Errors**: StrongBox unavailable, hardware failures
//! - **Security Errors**: Attestation failures, unauthorized access
//! - **Operational Errors**: Invalid parameters, key not found
//! - **System Errors**: Android service unavailable, permissions
//!
//! ## Compliance and Certification
//!
//! ### Standards Support
//!
//! - **FIDO2**: Hardware-backed FIDO2 authenticator support
//! - **WebAuthn**: Native WebAuthn integration
//! - **Common Criteria**: EAL4+ evaluation (Titan M)
//! - **FIPS 140-2**: Level 3 equivalent hardware protection
//!
//! ### Attestation Standards
//!
//! - **Android Key Attestation**: Full support for Android attestation format
//! - **Certificate Chains**: Proper validation back to Google root CA
//! - **Challenge-Response**: Secure challenge-based attestation
//! - **Device Identity**: Hardware-backed device identity

// Re-export all public types and functions
pub use device_info::*;
pub use entropy::*;
pub use health::*;
pub use pixel8_setup::*;
pub use types::*;

// Import from parent HSM module for configuration types
use beardog_errors::{BearDogError, BearDogResult};

// Module declarations
pub mod attestation;
pub mod core;
pub mod device_info;
pub mod entropy;
pub mod health;
pub mod keystore;
pub mod pixel8_setup;
pub mod types;

// Public API constants
/// Version of the Android StrongBox HSM integration
pub const VERSION: &str = "1.0.0";
/// Minimum Android version required for StrongBox support
pub const SUPPORTED_ANDROID_VERSION: u32 = 9; // Minimum Android version for StrongBox
/// Maximum number of keys that can be stored
pub const MAX_KEY_COUNT: usize = 1000;
/// Maximum size of attestation challenge in bytes
pub const MAX_CHALLENGE_SIZE: usize = 1024;

/// Android StrongBox HSM capabilities summary
pub const CAPABILITIES: &[&str] = &[
    "Hardware-backed key generation",
    "Key attestation with certificate chains",
    "ECDSA signing (P-256, P-384, P-521)",
    "RSA signing (2048, 4096 bits)",
    "AES encryption (128, 256 bits)",
    "User presence validation",
    "Biometric authentication",
    "Tamper-resistant hardware",
    "Secure key storage",
    "Device binding",
];

/// Supported key algorithms in Android StrongBox
pub const SUPPORTED_ALGORITHMS: &[&str] = &[
    "ECDSA-P256",
    "ECDSA-P384",
    "ECDSA-P521",
    "RSA-2048",
    "RSA-4096",
    "AES-128",
    "AES-256",
];

/// Performance benchmarks (typical values)
pub mod benchmarks {
    use std::time::Duration;

    /// Typical key generation times
    /// ECDSA key generation time estimate
    pub const KEY_GENERATION_TIME_ECDSA: Duration = Duration::from_millis(100);
    /// RSA key generation time estimate
    pub const KEY_GENERATION_TIME_RSA: Duration = Duration::from_millis(200);
    /// AES key generation time estimate
    pub const KEY_GENERATION_TIME_AES: Duration = Duration::from_millis(50);

    /// Typical signing times
    /// ECDSA signing time estimate
    pub const SIGNING_TIME_ECDSA: Duration = Duration::from_millis(10);
    /// RSA signing time estimate
    pub const SIGNING_TIME_RSA: Duration = Duration::from_millis(25);

    /// Typical verification times
    /// ECDSA verification time estimate
    pub const VERIFICATION_TIME_ECDSA: Duration = Duration::from_millis(5);
    /// RSA verification time estimate
    pub const VERIFICATION_TIME_RSA: Duration = Duration::from_millis(15);

    /// Typical encryption/decryption times
    /// AES encryption time estimate
    pub const ENCRYPTION_TIME_AES: Duration = Duration::from_millis(2);
    /// RSA encryption time estimate
    pub const ENCRYPTION_TIME_RSA: Duration = Duration::from_millis(20);

    /// Typical attestation time
    /// Attestation generation time estimate
    pub const ATTESTATION_TIME: Duration = Duration::from_millis(300);
}

/// Security summary for Android StrongBox
pub struct SecuritySummary {
    /// Whether StrongBox hardware is available
    pub strongbox_available: bool,
    /// Whether Titan M chip is available
    pub titan_m_available: bool,
    /// Whether verified boot is in GREEN state
    pub verified_boot_green: bool,
    /// Whether biometric authentication is supported
    pub biometric_support: bool,
    /// Whether the configuration is recommended for production use
    pub recommended_for_production: bool,
}

/// Configuration helpers and defaults
pub mod config {
    use crate::tunnel::hsm::types::*;

    /// Create default Android HSM configuration for GrapheneOS/Pixel
    pub fn default_grapheneos_config() -> AndroidHsmConfig {
        AndroidHsmConfig {
            manufacturer: "Google".to_string(),
            model: "Pixel 8a".to_string(),
            android_version: "14".to_string(),
            strongbox_version: Some("1.0".to_string()),
            strongbox_implementation: StrongBoxImplementation::TitanM {
                version: "1.0.0".to_string(),
                security_level: "Hardware".to_string(),
            },
            keystore_config: KeystoreConfig {
                alias_prefix: "beardog_".to_string(),
                require_user_authentication: false,
                user_authentication_validity_duration: Some(30000),
                require_strongbox: true,
            },
            attestation_config: AttestationConfig {
                enabled: true,
                require_hardware_backed: true,
                trusted_certificates: vec![],
                challenge_length: 32,
                attestation_challenge: None,
                enable_key_attestation: true,
                include_app_id: false,
            },
        }
    }

    /// Create development configuration (less strict requirements)
    pub fn development_config() -> AndroidHsmConfig {
        AndroidHsmConfig {
            manufacturer: "Google".to_string(),
            model: "Pixel 8a".to_string(),
            android_version: "14".to_string(),
            strongbox_version: Some("1.0".to_string()),
            strongbox_implementation: StrongBoxImplementation::TitanM {
                version: "1.0.0".to_string(),
                security_level: "Hardware".to_string(),
            },
            keystore_config: KeystoreConfig {
                alias_prefix: "beardog_dev_".to_string(),
                require_user_authentication: false,
                user_authentication_validity_duration: Some(60000),
                require_strongbox: false, // Allow software fallback in dev
            },
            attestation_config: AttestationConfig {
                enabled: false, // Optional in dev
                require_hardware_backed: false,
                trusted_certificates: vec![],
                challenge_length: 32,
                attestation_challenge: None,
                enable_key_attestation: true,
                include_app_id: false,
            },
        }
    }

    /// Create high-security configuration
    pub fn high_security_config() -> AndroidHsmConfig {
        AndroidHsmConfig {
            manufacturer: "Google".to_string(),
            model: "Pixel 8a".to_string(),
            android_version: "14".to_string(),
            strongbox_version: Some("1.0".to_string()),
            strongbox_implementation: StrongBoxImplementation::TitanM {
                version: "1.0.0".to_string(),
                security_level: "Hardware".to_string(),
            },
            keystore_config: KeystoreConfig {
                alias_prefix: "beardog_secure_".to_string(),
                require_user_authentication: true, // Always require user auth
                user_authentication_validity_duration: Some(10000), // Short timeout
                require_strongbox: true,
            },
            attestation_config: AttestationConfig {
                enabled: true,
                require_hardware_backed: true,
                trusted_certificates: vec![],
                challenge_length: 32,
                attestation_challenge: None,
                enable_key_attestation: true,
                include_app_id: false,
            },
        }
    }
}

/// Utility functions for Android StrongBox operations
pub mod utils {
    use super::*;
    use crate::tunnel::hsm::types::{GenerateKeyRequest, KeyType};
    use tracing::debug;

    /// Check if StrongBox is supported on the device
    pub async fn is_strongbox_supported() -> BearDogResult<bool> {
        debug!("Checking if StrongBox is supported");
        let device_info = AndroidDeviceInfo::detect().await?;
        Ok(device_info.is_strongbox_available())
    }

    /// Check if device is in optimal security configuration
    pub async fn is_optimal_security_config() -> BearDogResult<bool> {
        debug!("Checking for optimal security configuration");
        let device_info = AndroidDeviceInfo::detect().await?;
        Ok(device_info.is_optimal_security_config())
    }

    /// Get device security summary
    pub async fn get_security_summary() -> BearDogResult<SecuritySummary> {
        let device_info = AndroidDeviceInfo::detect().await?;
        let capabilities = device_info.get_capabilities();

        Ok(SecuritySummary {
            strongbox_available: capabilities.strongbox_available,
            titan_m_available: capabilities.titan_m_available,
            verified_boot_green: capabilities.verified_boot_green,
            biometric_support: capabilities.biometric_support,
            recommended_for_production: capabilities.is_production_ready(),
        })
    }

    /// Validate key generation request
    pub fn validate_key_request(request: &GenerateKeyRequest) -> BearDogResult<()> {
        debug!("Validating key generation request");
        match request.key_type {
            KeyType::EccP256 | KeyType::EccP384 | KeyType::EccP521 => {}
            KeyType::Rsa { key_size } => {
                if key_size < 2048 {
                    return Err(BearDogError::UnsupportedOperation {
                        operation: "RSA key size must be at least 2048 bits".to_string(),
                    });
                }
            }
            KeyType::Aes128 | KeyType::Aes256 => {}
            KeyType::Aes192 => {
                return Err(BearDogError::UnsupportedOperation {
                    operation: "AES-192 is not supported by StrongBox".to_string(),
                });
            }
            _ => {
                return Err(BearDogError::UnsupportedOperation {
                    operation: format!("Unsupported key type: {:?}", request.key_type),
                });
            }
        }
        Ok(())
    }
}
