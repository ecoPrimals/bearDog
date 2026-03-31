// SPDX-License-Identifier: AGPL-3.0-only

use super::constants::{ROTATION_30_DAYS_SECS, ROTATION_90_DAYS_SECS, SECONDS_PER_HOUR_U32};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Key management configuration
/// `KeyManagementConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementConfig {
    /// Key storage backend
    /// The storage backend value
    pub storage_backend: String,
    /// Key backup enabled
    /// Whether backup is enabled
    pub backup_enabled: bool,
    /// Key escrow configuration
    pub escrow_config: Option<EscrowConfig>,
    /// Key rotation policy
    /// The rotation policy value
    pub rotation_policy: RotationPolicy,
    /// Hardware security module configuration
    pub hsm_config: Option<HsmConfig>,
}

impl Default for KeyManagementConfig {
    fn default() -> Self {
        Self {
            storage_backend: "secure_enclave".to_string(),
            backup_enabled: true,
            escrow_config: None,
            rotation_policy: RotationPolicy::default(),
            hsm_config: None,
        }
    }
}

/// Key rotation policy
/// `RotationPolicy`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationPolicy {
    /// Automatic rotation enabled
    /// Whether automatic is enabled
    pub automatic: bool,
    /// Rotation interval in seconds
    /// Number of `interval_seconds`
    pub interval_seconds: u32,
    /// Maximum key age in seconds
    /// Number of `max_age_seconds`
    pub max_age_seconds: u32,
    /// Rotation on compromise
    /// Whether `rotate_on_compromise` is enabled
    pub rotate_on_compromise: bool,
}

impl Default for RotationPolicy {
    fn default() -> Self {
        Self {
            automatic: true,
            interval_seconds: ROTATION_30_DAYS_SECS, // 30 days
            max_age_seconds: ROTATION_90_DAYS_SECS,  // 90 days
            rotate_on_compromise: true,
        }
    }
}

/// Key escrow configuration
/// `EscrowConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscrowConfig {
    /// Escrow enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Escrow agents required
    /// Number of `agents_required`
    pub agents_required: u32,
    /// Number of threshold
    pub threshold: u32,
    /// Escrow metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// Hardware Security Module configuration
/// `HsmConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    /// HSM provider
    pub provider: String,
    /// HSM connection configuration
    /// Mapping of connection
    pub connection: HashMap<String, String>,
    /// HSM authentication configuration
    /// Mapping of authentication
    pub authentication: HashMap<String, String>,
    /// HSM feature flags
    /// Mapping of features
    pub features: HashMap<String, bool>,
}

/// Random Number Generator configuration
/// `RngConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngConfig {
    /// RNG algorithm
    /// The algorithm value
    pub algorithm: String,
    /// Entropy source
    /// The entropy source value
    pub entropy_source: String,
    /// Seed size in bytes
    /// Number of `seed_size`
    pub seed_size: u32,
    /// Reseed interval in seconds
    /// Number of `reseed_interval_seconds`
    pub reseed_interval_seconds: u32,
}

impl Default for RngConfig {
    fn default() -> Self {
        Self {
            algorithm: "ChaCha20".to_string(),
            entropy_source: "hardware".to_string(),
            seed_size: 32,
            reseed_interval_seconds: SECONDS_PER_HOUR_U32, // 1 hour
        }
    }
}
