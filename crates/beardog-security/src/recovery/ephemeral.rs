//! Ephemeral Recovery Keys
//!
//! This module handles ephemeral recovery keys and their permissions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Ephemeral recovery key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralRecoveryKey {
    /// Key ID
    pub id: String,
    /// User ID this key belongs to
    pub user_id: String,
    /// Key value (encrypted)
    pub key_value: String,
    /// When the key was generated
    pub generated_at: DateTime<Utc>,
    /// When the key expires
    pub expires_at: DateTime<Utc>,
    /// Whether the key has been used
    pub used: bool,
    /// Permissions for this key
    pub permissions: EphemeralPermissions,
    /// Time restrictions for key usage
    pub time_restrictions: TimeRestrictions,
}

impl EphemeralRecoveryKey {
    /// Create a new ephemeral recovery key
    pub fn new(id: String, user_id: String, permissions: EphemeralPermissions) -> Self {
        let now = Utc::now();
        Self {
            id,
            user_id,
            key_value: String::new(), // Would be generated securely
            generated_at: now,
            expires_at: now + chrono::Duration::hours(24), // 24 hour expiry
            used: false,
            permissions,
            time_restrictions: TimeRestrictions::default(),
        }
    }

    /// Check if the key is still valid
    pub fn is_valid(&self) -> bool {
        !self.used && Utc::now() < self.expires_at
    }

    /// Mark the key as used
    pub fn mark_used(&mut self) {
        self.used = true;
    }
}

/// Permissions for ephemeral recovery keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralPermissions {
    /// Can read user data
    pub can_read_data: bool,
    /// Can modify user settings
    pub can_modify_settings: bool,
    /// Can initiate recovery processes
    pub can_initiate_recovery: bool,
    /// Can access encrypted data
    pub can_access_encrypted: bool,
    /// Can perform administrative actions
    pub can_admin: bool,
    /// Maximum number of operations allowed
    pub max_operations: u32,
    /// Operations performed so far
    pub operations_used: u32,
}

/// Time-based restrictions for ephemeral keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    /// Valid from (optional)
    pub valid_from: Option<DateTime<Utc>>,
    /// Valid until (optional)
    pub valid_until: Option<DateTime<Utc>>,
    /// Maximum usage duration in hours
    pub max_usage_duration_hours: u32,
    /// Cooldown period between uses in minutes
    pub cooldown_minutes: u32,
}

impl Default for EphemeralPermissions {
    fn default() -> Self {
        Self {
            can_read_data: false,
            can_modify_settings: false,
            can_initiate_recovery: true,
            can_access_encrypted: false,
            can_admin: false,
            max_operations: 10,
            operations_used: 0,
        }
    }
}

impl Default for TimeRestrictions {
    fn default() -> Self {
        Self {
            valid_from: None,
            valid_until: None,
            max_usage_duration_hours: 24,
            cooldown_minutes: 5,
        }
    }
}
