// SPDX-License-Identifier: AGPL-3.0-or-later

//! Cryptographic Configuration
//!
//! This module provides encryption, key management, and cryptographic provider
//! configuration structures for the `BearDog` security system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Encryption configuration - consolidates `EncryptionConfig` and `SafeCryptoConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfiguration {
    /// Default encryption algorithm
    pub default_algorithm: String,
    /// Key size in bits
    pub key_size_bits: u32,
    /// Enable hardware acceleration
    pub enable_hardware_acceleration: bool,
    /// Safe crypto configuration
    pub safe_crypto: SafeCryptoConfiguration,
    /// Context-aware key configuration
    pub context_aware_keys: ContextAwareKeyConfiguration,
}

impl EncryptionConfiguration {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::{get_bool, get_parsed};

        Self {
            default_algorithm: source.get_or("BEARDOG_ENCRYPTION_ALGORITHM", "AES-256-GCM"),
            key_size_bits: get_parsed(source, "BEARDOG_ENCRYPTION_KEY_SIZE_BITS", 256),
            enable_hardware_acceleration: get_bool(
                source,
                "BEARDOG_ENCRYPTION_HW_ACCEL_ENABLED",
                true,
            ),
            safe_crypto: SafeCryptoConfiguration::default(),
            context_aware_keys: ContextAwareKeyConfiguration {
                enable_context_aware: get_bool(source, "BEARDOG_CONTEXT_KEYS_ENABLED", true),
                context_factors: vec![
                    "time".to_string(),
                    "location".to_string(),
                    "user_behavior".to_string(),
                ],
                context_rotation_threshold: get_parsed(
                    source,
                    "BEARDOG_CONTEXT_ROTATION_THRESHOLD",
                    0.8,
                ),
                context_validation_timeout: get_parsed(
                    source,
                    "BEARDOG_CONTEXT_VALIDATION_TIMEOUT_SECS",
                    30,
                ),
            },
        }
    }
}

/// Safe cryptographic operations configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeCryptoConfiguration {
    /// Enable safe crypto mode
    pub enable_safe_mode: bool,
    /// Validation level (strict, moderate, lenient)
    pub validation_level: String,
    /// Enable constant-time operations
    pub enable_constant_time: bool,
    /// Memory protection level
    pub memory_protection_level: String,
}

impl Default for SafeCryptoConfiguration {
    fn default() -> Self {
        Self {
            enable_safe_mode: true,
            validation_level: "strict".to_string(),
            enable_constant_time: true,
            memory_protection_level: "high".to_string(),
        }
    }
}

/// Context-aware key configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAwareKeyConfiguration {
    /// Enable context-aware key management
    pub enable_context_aware: bool,
    /// Context factors to consider
    pub context_factors: Vec<String>,
    /// Key rotation based on context
    pub context_rotation_threshold: f64,
    /// Context validation timeout
    pub context_validation_timeout: u64,
}

/// Key management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementConfiguration {
    /// Key rotation interval in seconds
    pub rotation_interval_seconds: u64,
    /// Enable automatic key rotation
    pub enable_auto_rotation: bool,
    /// Key backup configuration
    pub backup_enabled: bool,
    /// Key escrow configuration
    pub escrow_enabled: bool,
    /// Genetic key renewal settings
    pub genetic_renewal: GeneticRenewalConfiguration,
}

/// Genetic key renewal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticRenewalConfiguration {
    /// Enable genetic key renewal
    pub enable_genetic_renewal: bool,
    /// Genetic algorithm parameters
    pub genetic_parameters: HashMap<String, f64>,
    /// Renewal trigger conditions
    pub renewal_triggers: Vec<String>,
    /// Renewal frequency
    pub renewal_frequency_hours: u64,
}

impl Default for GeneticRenewalConfiguration {
    fn default() -> Self {
        let mut genetic_parameters = HashMap::new();
        genetic_parameters.insert(
            "mutation_rate".to_string(),
            std::env::var("BEARDOG_GENETIC_RENEWAL_MUTATION_RATE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.1),
        );
        genetic_parameters.insert(
            "crossover_rate".to_string(),
            std::env::var("BEARDOG_GENETIC_RENEWAL_CROSSOVER_RATE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.8),
        );
        genetic_parameters.insert(
            "selection_pressure".to_string(),
            std::env::var("BEARDOG_GENETIC_RENEWAL_SELECTION_PRESSURE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.7),
        );

        Self {
            enable_genetic_renewal: true,
            genetic_parameters,
            renewal_triggers: vec![
                "time_based".to_string(),
                "usage_based".to_string(),
                "threat_based".to_string(),
            ],
            renewal_frequency_hours: std::env::var("BEARDOG_GENETIC_RENEWAL_FREQUENCY_HOURS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(168), // Weekly
        }
    }
}

/// Crypto provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoProviderConfiguration {
    /// Primary crypto provider
    pub primary_provider: String,
    /// Fallback providers
    pub fallback_providers: Vec<String>,
    /// Provider-specific settings
    pub provider_settings: HashMap<String, serde_json::Value>,
    /// Enable provider failover
    pub enable_failover: bool,
}
