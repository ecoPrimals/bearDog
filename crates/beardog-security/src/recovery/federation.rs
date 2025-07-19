//! Federation Recovery
//!
//! This module handles federation recovery between trusted BearDog instances.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Federation recovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationRecoveryConfig {
    /// User ID this config belongs to
    pub user_id: String,
    /// List of trusted BearDog instances
    pub trusted_instances: Vec<TrustedInstance>,
    /// Minimum number of instances needed for recovery
    pub min_instances_required: u32,
    /// Whether federation recovery is enabled
    pub enabled: bool,
    /// Cross-instance verification settings
    pub verification_settings: FederationVerificationSettings,
}

/// Trusted BearDog instance for federation recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedInstance {
    /// Instance ID
    pub id: String,
    /// Instance endpoint
    pub endpoint: String,
    /// Instance public key
    pub public_key: String,
    /// Trust level (0-100)
    pub trust_level: u8,
    /// When this instance was added to trusted list
    pub added_at: DateTime<Utc>,
    /// Whether this instance is currently active
    pub active: bool,
}

/// Federation verification settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationVerificationSettings {
    /// Require cryptographic proof of identity
    pub require_crypto_proof: bool,
    /// Require reputation verification
    pub require_reputation: bool,
    /// Minimum reputation score required
    pub min_reputation_score: u32,
    /// Verification timeout in minutes
    pub verification_timeout_minutes: u32,
}

impl Default for FederationRecoveryConfig {
    fn default() -> Self {
        Self {
            user_id: String::new(),
            trusted_instances: Vec::new(),
            min_instances_required: 2,
            enabled: false,
            verification_settings: FederationVerificationSettings::default(),
        }
    }
}

impl Default for FederationVerificationSettings {
    fn default() -> Self {
        Self {
            require_crypto_proof: true,
            require_reputation: false,
            min_reputation_score: 70,
            verification_timeout_minutes: 30,
        }
    }
}
