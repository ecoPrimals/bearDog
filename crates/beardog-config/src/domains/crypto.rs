// SPDX-License-Identifier: AGPL-3.0-only

//! Concurrent-Safe Cryptographic Configuration Module
//!
//! Cryptographic parameters configuration for `BearDog`.
//!
//! ## Design Pattern: Explicit Environment Loading
//!
//! To ensure **concurrent safety** and **testability**, this module separates:
//! - **Static defaults** (`Default` trait) - Pure, no environment reads
//! - **Environment loading** (`from_env()`) - Explicit environment variable reads
//! - **Flexible construction** (`builder()`) - Testing without env var pollution

use crate::error::{ConfigError, ConfigResult};
use serde::{Deserialize, Serialize};

/// Cryptographic parameters configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CryptoConfig {
    /// RSA key size in bits
    pub rsa_key_size: u32,

    /// Elliptic curve to use
    pub ec_curve: String,

    /// AES key size in bits
    pub aes_key_size: u32,

    /// Hash algorithm for signatures
    pub hash_algorithm: String,

    /// PBKDF2 iterations
    pub pbkdf2_iterations: u32,

    /// Use FIPS-compliant algorithms only
    pub fips_mode: bool,
}

impl CryptoConfig {
    /// Pure static defaults (no environment variable reads)
    #[must_use]
    pub fn const_defaults() -> Self {
        Self {
            rsa_key_size: 2048,
            ec_curve: "secp256r1".to_string(),
            aes_key_size: 256,
            hash_algorithm: "sha256".to_string(),
            pbkdf2_iterations: 100_000,
            fips_mode: false,
        }
    }

    /// Load configuration from environment variables with fallback to defaults
    #[must_use]
    pub fn from_env() -> Self {
        let defaults = Self::const_defaults();

        Self {
            rsa_key_size: std::env::var("BEARDOG_RSA_KEY_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.rsa_key_size),

            ec_curve: std::env::var("BEARDOG_EC_CURVE")
                .ok()
                .unwrap_or(defaults.ec_curve),

            aes_key_size: std::env::var("BEARDOG_AES_KEY_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.aes_key_size),

            hash_algorithm: std::env::var("BEARDOG_HASH_ALGORITHM")
                .ok()
                .unwrap_or(defaults.hash_algorithm),

            pbkdf2_iterations: std::env::var("BEARDOG_PBKDF2_ITERATIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.pbkdf2_iterations),

            fips_mode: defaults.fips_mode,
        }
    }

    /// Create a builder for flexible configuration construction
    #[must_use]
    pub fn builder() -> CryptoConfigBuilder {
        CryptoConfigBuilder::new()
    }

    /// Validate crypto configuration
    ///
    /// # Errors
    ///
    /// Returns error if any cryptographic parameters are invalid or insecure
    pub fn validate(&self) -> ConfigResult<()> {
        // Validate RSA key size
        if self.rsa_key_size < 2048 {
            return Err(ConfigError::invalid_value(
                "crypto.rsa_key_size",
                "RSA key size must be at least 2048 bits for security",
            ));
        }

        // Validate AES key size
        if ![128, 192, 256].contains(&self.aes_key_size) {
            return Err(ConfigError::invalid_value(
                "crypto.aes_key_size",
                "AES key size must be 128, 192, or 256 bits",
            ));
        }

        // Validate EC curve
        let valid_curves = ["secp256r1", "secp384r1", "secp521r1", "ed25519"];
        if !valid_curves.contains(&self.ec_curve.as_str()) {
            return Err(ConfigError::invalid_value(
                "crypto.ec_curve",
                "Must be one of: secp256r1, secp384r1, secp521r1, ed25519",
            ));
        }

        // Validate hash algorithm
        let valid_hashes = ["sha256", "sha384", "sha512"];
        if !valid_hashes.contains(&self.hash_algorithm.as_str()) {
            return Err(ConfigError::invalid_value(
                "crypto.hash_algorithm",
                "Must be one of: sha256, sha384, sha512",
            ));
        }

        // Validate PBKDF2 iterations
        if self.pbkdf2_iterations < 100_000 {
            return Err(ConfigError::invalid_value(
                "crypto.pbkdf2_iterations",
                "PBKDF2 iterations must be at least 100,000 for security",
            ));
        }

        Ok(())
    }
}

impl Default for CryptoConfig {
    fn default() -> Self {
        Self::const_defaults()
    }
}

/// Builder for `CryptoConfig`
#[derive(Debug, Default)]
pub struct CryptoConfigBuilder {
    rsa_key_size: Option<u32>,
    ec_curve: Option<String>,
    aes_key_size: Option<u32>,
    hash_algorithm: Option<String>,
    pbkdf2_iterations: Option<u32>,
    fips_mode: Option<bool>,
}

impl CryptoConfigBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn rsa_key_size(mut self, size: u32) -> Self {
        self.rsa_key_size = Some(size);
        self
    }

    #[must_use]
    pub fn ec_curve(mut self, curve: String) -> Self {
        self.ec_curve = Some(curve);
        self
    }

    #[must_use]
    pub fn aes_key_size(mut self, size: u32) -> Self {
        self.aes_key_size = Some(size);
        self
    }

    #[must_use]
    pub fn hash_algorithm(mut self, algorithm: String) -> Self {
        self.hash_algorithm = Some(algorithm);
        self
    }

    #[must_use]
    pub fn pbkdf2_iterations(mut self, iterations: u32) -> Self {
        self.pbkdf2_iterations = Some(iterations);
        self
    }

    #[must_use]
    pub fn fips_mode(mut self, enabled: bool) -> Self {
        self.fips_mode = Some(enabled);
        self
    }

    #[must_use]
    pub fn build(self) -> CryptoConfig {
        let defaults = CryptoConfig::const_defaults();

        CryptoConfig {
            rsa_key_size: self.rsa_key_size.unwrap_or(defaults.rsa_key_size),
            ec_curve: self.ec_curve.unwrap_or(defaults.ec_curve),
            aes_key_size: self.aes_key_size.unwrap_or(defaults.aes_key_size),
            hash_algorithm: self.hash_algorithm.unwrap_or(defaults.hash_algorithm),
            pbkdf2_iterations: self.pbkdf2_iterations.unwrap_or(defaults.pbkdf2_iterations),
            fips_mode: self.fips_mode.unwrap_or(defaults.fips_mode),
        }
    }
}

#[cfg(test)]
#[path = "crypto_comprehensive_tests.rs"]
mod crypto_comprehensive_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_crypto() {
        let config = CryptoConfig::default();
        assert!(config.validate().is_ok());
        assert_eq!(config.rsa_key_size, 2048);
        assert_eq!(config.aes_key_size, 256);
    }

    #[test]
    fn test_invalid_rsa_key_size() {
        let config = CryptoConfig::builder()
            .rsa_key_size(1024) // Too small
            .build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_aes_key_size() {
        let config = CryptoConfig::builder()
            .aes_key_size(512) // Invalid
            .build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_pbkdf2_iterations() {
        let config = CryptoConfig::builder()
            .pbkdf2_iterations(1000) // Too low
            .build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_builder() {
        let config = CryptoConfig::builder()
            .rsa_key_size(4096)
            .aes_key_size(192)
            .ec_curve("secp384r1".to_string())
            .build();

        assert_eq!(config.rsa_key_size, 4096);
        assert_eq!(config.aes_key_size, 192);
        assert_eq!(config.ec_curve, "secp384r1");
    }

    #[test]
    fn test_fips_mode() {
        let config = CryptoConfig::builder().fips_mode(true).build();

        assert!(config.fips_mode);
    }

    #[test]
    fn test_high_security_config() {
        let config = CryptoConfig::builder()
            .rsa_key_size(4096)
            .aes_key_size(256)
            .hash_algorithm("sha512".to_string())
            .ec_curve("secp521r1".to_string())
            .pbkdf2_iterations(250_000)
            .build();

        assert!(config.validate().is_ok());
        assert_eq!(config.rsa_key_size, 4096);
        assert_eq!(config.hash_algorithm, "sha512");
    }
}
