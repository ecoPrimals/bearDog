// SPDX-License-Identifier: AGPL-3.0-only

// Encryption Configuration
//
// Canonical encryption configuration for algorithms, key management, and HSM integration.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **CANONICAL ENCRYPTION CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalEncryptionConfig {
    /// Default encryption algorithm
    /// The default algorithm value
    pub default_algorithm: String,

    /// Supported encryption algorithms
    /// Collection of supported algorithms
    pub supported_algorithms: Vec<String>,

    /// Key derivation configuration
    /// The key derivation value
    pub key_derivation: KeyDerivationConfig,

    /// HSM configuration
    /// The hsm value
    pub hsm: HsmEncryptionConfig,

    /// Key rotation settings
    /// Number of `key_rotation_days`
    pub key_rotation_days: u32,

    /// Enable at-rest encryption
    /// Whether `enable_at_rest_encryption` is enabled
    pub enable_at_rest_encryption: bool,

    /// Enable in-transit encryption
    /// Whether `enable_in_transit_encryption` is enabled
    pub enable_in_transit_encryption: bool,
}

impl Default for CanonicalEncryptionConfig {
    fn default() -> Self {
        Self {
            default_algorithm: "AES-256-GCM".to_string(),
            supported_algorithms: vec!["AES-256-GCM".to_string(), "ChaCha20-Poly1305".to_string()],
            key_derivation: KeyDerivationConfig::default(),
            hsm: HsmEncryptionConfig::default(),
            key_rotation_days: 90,
            enable_at_rest_encryption: true,
            enable_in_transit_encryption: true,
        }
    }
}

impl CanonicalEncryptionConfig {
    /// Production
    #[must_use]
    pub fn production() -> Self {
        Self {
            key_rotation_days: 30, // More frequent rotation for production
            ..Self::default()
        }
    }

    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.default_algorithm.is_empty() {
            return Err(BearDogError::security(
                "Default encryption algorithm cannot be empty".to_string(),
            ));
        }
        self.key_derivation.validate()?;
        Ok(())
    }
}

/// Key derivation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationConfig {
    /// The algorithm value
    pub algorithm: String,

    /// Number of iterations
    /// Number of iterations
    pub iterations: u32,

    /// Salt length in bytes
    /// Number of `salt_length`
    pub salt_length: usize,

    /// Key length in bytes
    /// Number of `key_length`
    pub key_length: usize,
}

impl Default for KeyDerivationConfig {
    fn default() -> Self {
        Self {
            algorithm: "Argon2id".to_string(),
            iterations: 100_000,
            salt_length: 32,
            key_length: 32,
        }
    }
}

impl KeyDerivationConfig {
    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.iterations < 10000 {
            return Err(BearDogError::security(
                "Key derivation iterations must be at least 10000".to_string(),
            ));
        }
        if self.salt_length < 16 {
            return Err(BearDogError::security(
                "Salt length must be at least 16 bytes".to_string(),
            ));
        }
        Ok(())
    }
}

/// HSM encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmEncryptionConfig {
    /// Whether feature is enabled
    pub enabled: bool,

    /// HSM provider type
    pub provider: String,

    /// HSM configuration parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, String>,
}

impl Default for HsmEncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "software".to_string(),
            parameters: HashMap::new(),
        }
    }
}
