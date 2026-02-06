//! # Crypto Types
//!
//! This module provides cryptographic types for key management,
//! expiry policies, and entropy-based configurations.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

// Re-export RateLimitConfig
pub use beardog_types::canonical::configuration::security::RateLimitConfig;

// ============================================================
// Entropy Configuration
// ============================================================

/// Configuration for entropy adjustment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyAdjustmentConfig {
    /// Whether entropy adjustment is enabled
    pub enabled: bool,

    /// Adjustment factor for entropy calculations
    pub adjustment_factor: f64,

    /// Minimum entropy threshold
    pub minimum_entropy: u32,

    /// Maximum entropy threshold
    pub maximum_entropy: u32,
}

impl Default for EntropyAdjustmentConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            adjustment_factor: 1.0,
            minimum_entropy: 128,
            maximum_entropy: 256,
        }
    }
}

/// Entropy-based key expiry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyBasedExpiry {
    /// Whether entropy-based expiry is enabled
    pub enabled: bool,

    /// Base expiry time in seconds
    pub base_expiry_seconds: u64,

    /// Quality threshold for entropy (0.0 - 1.0)
    pub quality_threshold: f64,
}

impl Default for EntropyBasedExpiry {
    fn default() -> Self {
        Self {
            enabled: false,
            base_expiry_seconds: 86400, // 24 hours
            quality_threshold: 0.9,
        }
    }
}

// ============================================================
// Genetic Renewal
// ============================================================

/// Configuration for genetic-based key renewal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticRenewalConfig {
    /// Maximum generations for genetic optimization
    pub max_generations: u32,

    /// Mutation rate (0.0 - 1.0)
    pub mutation_rate: f64,

    /// Population size
    pub population_size: usize,
}

impl Default for GeneticRenewalConfig {
    fn default() -> Self {
        Self {
            max_generations: 10,
            mutation_rate: 0.05,
            population_size: 100,
        }
    }
}

// ============================================================
// Key Expiry
// ============================================================

/// Key expiry policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyExpiryPolicy {
    /// Fixed duration expiry
    Fixed {
        /// Duration in seconds
        duration_seconds: u64,
    },

    /// Entropy-based expiry
    EntropyBased {
        /// Entropy configuration
        config: EntropyBasedExpiry,
    },

    /// Genetic algorithm-based renewal
    Genetic {
        /// Genetic renewal configuration
        config: GeneticRenewalConfig,
    },

    /// Key never expires
    Never,

    /// Usage-based expiry
    UsageBased {
        /// Maximum number of uses
        max_uses: u32,

        /// Optional time window in seconds
        time_window_seconds: Option<u64>,
    },
}

impl Default for KeyExpiryPolicy {
    fn default() -> Self {
        Self::Fixed {
            duration_seconds: 86400, // 24 hours
        }
    }
}

/// Key expiry status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyExpiryStatus {
    /// Key is valid
    Valid {
        /// Time until expiry in seconds
        expires_in_seconds: u64,
    },

    /// Key has expired
    Expired {
        /// Time since expiry in seconds
        expired_seconds_ago: u64,
    },

    /// Key is expiring soon
    ExpiringSoon {
        /// Time until expiry in seconds
        expires_in_seconds: u64,

        /// Warning threshold in seconds
        warning_threshold_seconds: u64,
    },

    /// Key has unlimited validity
    Unlimited,

    /// Usage-based status
    UsageBased {
        /// Remaining uses (if tracked)
        remaining_uses: Option<u32>,

        /// Time until window expiry (if applicable)
        time_window_expires_in: Option<u64>,
    },
}

impl Default for KeyExpiryStatus {
    fn default() -> Self {
        Self::UsageBased {
            remaining_uses: None,
            time_window_expires_in: None,
        }
    }
}

// ============================================================
// Context-Aware Key Configuration
// ============================================================

/// Context-aware key configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAwareKeyConfig {
    /// Context identifier
    pub context: String,

    /// Purpose of the key
    pub purpose: String,

    /// Security tier (1-5)
    pub security_tier: u8,

    /// Expiry policy
    pub expiry_policy: KeyExpiryPolicy,

    /// Rate limit configuration
    pub rate_limit: Option<RateLimitConfig>,
}

impl Default for ContextAwareKeyConfig {
    fn default() -> Self {
        Self {
            context: "default".to_string(),
            purpose: "general".to_string(),
            security_tier: 3,
            expiry_policy: KeyExpiryPolicy::default(),
            rate_limit: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_adjustment_default() {
        let config = EntropyAdjustmentConfig::default();
        assert!(config.enabled);
        assert_eq!(config.adjustment_factor, 1.0);
        assert_eq!(config.minimum_entropy, 128);
    }

    #[test]
    fn test_key_expiry_policy_default() {
        let policy = KeyExpiryPolicy::default();
        match policy {
            KeyExpiryPolicy::Fixed { duration_seconds } => {
                assert_eq!(duration_seconds, 86400);
            }
            _ => panic!("Expected Fixed policy"),
        }
    }

    #[test]
    fn test_genetic_renewal_default() {
        let config = GeneticRenewalConfig::default();
        assert_eq!(config.max_generations, 10);
        assert_eq!(config.mutation_rate, 0.05);
    }

    #[test]
    fn test_context_aware_key_default() {
        let config = ContextAwareKeyConfig::default();
        assert_eq!(config.context, "default");
        assert_eq!(config.security_tier, 3);
    }
}
