//! Cryptographic Types
//!
//! This module contains all types related to cryptographic operations,
//! key management, entropy handling, and security-related configurations.

use serde::{Deserialize, Serialize};

/// Key lifecycle status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyStatus {
    /// Key is active and can be used
    Active,
    /// Key is pending activation
    Pending,
    /// Key has been revoked
    Revoked,
    /// Key has expired
    Expired,
}

/// Configuration for entropy-based adjustments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyAdjustmentConfig {
    /// Enable entropy adjustments
    pub enabled: bool,
    /// Minimum entropy quality threshold
    pub min_quality: f64,
}

/// Entropy-based expiry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyBasedExpiry {
    /// Enable entropy-based expiry
    pub enabled: bool,
    /// Base expiry time in seconds
    pub base_expiry_seconds: u64,
    /// Entropy quality threshold for expiry adjustment
    pub quality_threshold: f64,
}

/// Configuration for genetic renewal of keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticRenewalConfig {
    /// Enable genetic renewal
    pub enabled: bool,
    /// Generation limit before forced renewal
    pub max_generations: u32,
}

/// Key expiry policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyExpiryPolicy {
    /// Fixed time expiry
    Fixed {
        /// Duration in seconds
        duration_seconds: u64,
    },
    /// Entropy-based dynamic expiry
    EntropyBased {
        /// Entropy configuration
        config: EntropyBasedExpiry,
    },
    /// Genetic algorithm-based renewal
    Genetic {
        /// Genetic renewal configuration
        config: GeneticRenewalConfig,
    },
    /// Never expires (use with caution)
    Never,
    /// Usage-based expiry
    UsageBased {
        /// Maximum number of uses
        max_uses: u32,
        /// Time window in seconds
        time_window_seconds: Option<u64>,
    },
}

/// Current status of key expiry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyExpiryStatus {
    /// Key is valid and not expired
    Valid {
        /// Time until expiry
        expires_in_seconds: u64,
    },
    /// Key is expired
    Expired {
        /// Time since expiry
        expired_seconds_ago: u64,
    },
    /// Key is expiring soon
    ExpiringSoon {
        /// Time until expiry
        expires_in_seconds: u64,
        /// Warning threshold that was exceeded
        warning_threshold_seconds: u64,
    },
    /// Key has unlimited usage
    Unlimited,
    /// Usage-based expiry status
    UsageBased {
        /// Remaining uses
        remaining_uses: Option<u32>,
    },
}

/// Context-aware key configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAwareKeyConfig {
    /// Key context identifier
    pub context: String,
    /// Purpose of the key
    pub purpose: String,
    /// Security tier requirement
    pub security_tier: u8,
    /// Expiry policy
    pub expiry_policy: KeyExpiryPolicy,
}

/// Rate limiting configuration for cryptographic operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum operations per window
    pub max_operations: u32,
    /// Time window in seconds
    pub window_seconds: u64,
    /// Whether to enable rate limiting
    pub enabled: bool,
}

// Default implementations

impl Default for EntropyAdjustmentConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_quality: 0.8,
        }
    }
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

impl Default for GeneticRenewalConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_generations: 10,
        }
    }
}

impl Default for KeyExpiryPolicy {
    fn default() -> Self {
        Self::Fixed {
            duration_seconds: 86400, // 24 hours
        }
    }
}

impl Default for KeyExpiryStatus {
    fn default() -> Self {
        Self::UsageBased {
            remaining_uses: None,
        }
    }
}

impl Default for ContextAwareKeyConfig {
    fn default() -> Self {
        Self {
            context: "default".to_string(),
            purpose: "general".to_string(),
            security_tier: 1,
            expiry_policy: KeyExpiryPolicy::default(),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_operations: 100,
            window_seconds: 60,
            enabled: true,
        }
    }
} 