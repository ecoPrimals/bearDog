// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM crypto provider configurations and capabilities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Crypto provider configuration
///
/// Defines which cryptographic provider to use and its settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoProviderConfig {
    /// Provider identifier (e.g., "rust-crypto")
    pub provider_id: String,

    /// Provider type
    pub provider_type: CryptoProviderType,

    /// Enable FIPS mode (if supported by provider)
    #[serde(default)]
    pub fips_mode: bool,

    /// Provider-specific parameters
    #[serde(default)]
    pub parameters: HashMap<String, String>,

    /// Capabilities this provider supports
    pub capabilities: ProviderCapabilities,
}

/// Types of cryptographic providers
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CryptoProviderType {
    /// Software-based cryptography (pure Rust)
    Software,
    /// OpenSSL-based provider
    OpenSsl,
    /// Ring crypto library (DEPRECATED: enum variant kept for wire compatibility)
    Ring,
    /// Hardware security module
    Hardware,
    /// Cloud KMS
    CloudKms,
}

/// Provider capabilities
///
/// Describes what cryptographic operations a provider supports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    /// Supported key types
    pub key_types: Vec<String>,

    /// Supported encryption algorithms
    pub encryption_algorithms: Vec<String>,

    /// Supported signing algorithms
    pub signing_algorithms: Vec<String>,

    /// Supported hash algorithms
    pub hash_algorithms: Vec<String>,

    /// Maximum key size in bits
    pub max_key_size: u32,

    /// Supports hardware acceleration
    pub hardware_acceleration: bool,

    /// Supports secure enclaves (TEE/SGX)
    pub secure_enclave: bool,

    /// FIPS 140-2/3 certified
    pub fips_certified: bool,
}

impl Default for CryptoProviderConfig {
    fn default() -> Self {
        Self {
            provider_id: "software".to_string(),
            provider_type: CryptoProviderType::Software,
            fips_mode: false,
            parameters: HashMap::new(),
            capabilities: ProviderCapabilities::default(),
        }
    }
}

impl Default for ProviderCapabilities {
    fn default() -> Self {
        Self {
            key_types: vec![
                "RSA-2048".to_string(),
                "RSA-4096".to_string(),
                "ECDSA-P256".to_string(),
                "ECDSA-P384".to_string(),
                "Ed25519".to_string(),
                "AES-256".to_string(),
            ],
            encryption_algorithms: vec!["AES-256-GCM".to_string(), "ChaCha20-Poly1305".to_string()],
            signing_algorithms: vec![
                "ECDSA-SHA256".to_string(),
                "Ed25519".to_string(),
                "RSA-PSS-SHA256".to_string(),
            ],
            hash_algorithms: vec![
                "SHA-256".to_string(),
                "SHA-384".to_string(),
                "SHA-512".to_string(),
                "BLAKE3".to_string(),
            ],
            max_key_size: 4096,
            hardware_acceleration: false,
            secure_enclave: false,
            fips_certified: false,
        }
    }
}

impl CryptoProviderConfig {
    /// Creates an OpenSSL provider configuration
    #[must_use]
    pub fn openssl() -> Self {
        Self {
            provider_id: "openssl".to_string(),
            provider_type: CryptoProviderType::OpenSsl,
            fips_mode: false,
            parameters: HashMap::new(),
            capabilities: ProviderCapabilities {
                hardware_acceleration: true,
                ..Default::default()
            },
        }
    }

    /// Creates a Ring provider configuration
    #[must_use]
    pub fn ring() -> Self {
        Self {
            provider_id: "ring".to_string(),
            provider_type: CryptoProviderType::Ring,
            fips_mode: false,
            parameters: HashMap::new(),
            capabilities: ProviderCapabilities::default(),
        }
    }

    /// Creates a software provider configuration (pure Rust)
    #[must_use]
    pub fn software() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_provider_config_default() {
        let config = CryptoProviderConfig::default();
        assert_eq!(config.provider_id, "software");
        assert_eq!(config.provider_type, CryptoProviderType::Software);
        assert!(!config.fips_mode);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_crypto_provider_config_openssl() {
        let config = CryptoProviderConfig::openssl();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(config.provider_id, "openssl");
        assert_eq!(config.provider_type, CryptoProviderType::OpenSsl);
        assert!(config.capabilities.hardware_acceleration);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_provider_capabilities_default() {
        let caps = ProviderCapabilities::default();
        assert!(!caps.key_types.is_empty());
        assert!(!caps.encryption_algorithms.is_empty());
        assert_eq!(caps.max_key_size, 4096);
    }
}
