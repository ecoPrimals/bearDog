//! Key Lifecycle Management Types
//!
//! Comprehensive types for managing cryptographic key lifecycle states,
//! rotation, expiration, and audit trails.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Key lifecycle state
///
/// Represents the current state of a cryptographic key throughout its lifecycle.
/// Keys transition through these states from generation to secure deletion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyLifecycleState {
    /// Key generated but not yet activated
    ///
    /// In this state, the key exists but is not yet in use. This allows for
    /// key validation and preparation before activation.
    Pending,

    /// Key currently in active use
    ///
    /// The key is actively being used for cryptographic operations.
    /// This is the primary operational state for keys.
    Active,

    /// Key being rotated (transitional state)
    ///
    /// Temporary state during the rotation process. A new key is being
    /// generated to replace this key.
    Rotating,

    /// Key no longer active but still valid for decryption
    ///
    /// After rotation, the old key is deprecated but retained to allow
    /// decryption of data encrypted with it during the deprecation period.
    Deprecated,

    /// Key past expiration date
    ///
    /// The key has exceeded its configured expiration time. While still
    /// available for decryption, it should not be used for new operations.
    Expired,

    /// Key manually revoked
    ///
    /// The key has been explicitly revoked and should not be used for
    /// any operations. This is a terminal state before destruction.
    Revoked,

    /// Key securely deleted (zeroized)
    ///
    /// The key has been securely deleted from memory and storage.
    /// This is the final terminal state.
    Destroyed,
}

impl KeyLifecycleState {
    /// Check if the key can be used for encryption
    #[must_use]
    pub const fn can_encrypt(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Check if the key can be used for decryption
    #[must_use]
    pub const fn can_decrypt(&self) -> bool {
        matches!(self, Self::Active | Self::Deprecated | Self::Expired)
    }

    /// Check if the key can be rotated
    #[must_use]
    pub const fn can_rotate(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Check if this is a terminal state (no further transitions)
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(self, Self::Revoked | Self::Destroyed)
    }
}

/// Key metadata with comprehensive lifecycle information
///
/// Tracks all lifecycle events, timestamps, and relationships for a cryptographic key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadataWithLifecycle {
    /// Unique key identifier
    pub key_id: String,

    /// Type of key (e.g., "aes", "rsa", "ecc", "ed25519")
    pub key_type: String,

    /// Cryptographic algorithm (e.g., "aes-256-gcm", "rsa-2048", "ed25519")
    pub algorithm: String,

    /// Current lifecycle state
    pub state: KeyLifecycleState,

    /// Timestamp when key was created/generated
    pub created_at: DateTime<Utc>,

    /// Timestamp when key was activated (if applicable)
    pub activated_at: Option<DateTime<Utc>>,

    /// Timestamp when key expires (if configured)
    pub expires_at: Option<DateTime<Utc>>,

    /// Timestamp when key was deprecated (after rotation)
    pub deprecated_at: Option<DateTime<Utc>>,

    /// Timestamp when key was revoked
    pub revoked_at: Option<DateTime<Utc>>,

    /// Timestamp when key was destroyed
    pub destroyed_at: Option<DateTime<Utc>>,

    /// Automatic rotation interval (if enabled)
    pub rotation_interval: Option<Duration>,

    /// Number of times this key has been used
    pub usage_count: u64,

    /// Timestamp of most recent use
    pub last_used_at: Option<DateTime<Utc>>,

    /// ID of the key this one replaced (if rotated from another)
    pub predecessor_key_id: Option<String>,

    /// ID of the key that replaced this one (if rotated)
    pub successor_key_id: Option<String>,

    /// Whether this key has an encrypted backup
    pub backup_encrypted: bool,

    /// Location/identifier of backup (if applicable)
    pub backup_location: Option<String>,

    /// Owner/creator of the key
    pub owner: Option<String>,

    /// Additional metadata tags
    pub tags: Vec<(String, String)>,
}

impl KeyMetadataWithLifecycle {
    /// Create new key metadata in Pending state
    #[must_use]
    pub fn new(key_id: String, key_type: String, algorithm: String) -> Self {
        Self {
            key_id,
            key_type,
            algorithm,
            state: KeyLifecycleState::Pending,
            created_at: Utc::now(),
            activated_at: None,
            expires_at: None,
            deprecated_at: None,
            revoked_at: None,
            destroyed_at: None,
            rotation_interval: None,
            usage_count: 0,
            last_used_at: None,
            predecessor_key_id: None,
            successor_key_id: None,
            backup_encrypted: false,
            backup_location: None,
            owner: None,
            tags: Vec::new(),
        }
    }

    /// Check if the key is expired
    #[must_use]
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() >= expires_at
        } else {
            false
        }
    }

    /// Check if the key needs rotation
    #[must_use]
    pub fn needs_rotation(&self) -> bool {
        // Already expired
        if self.is_expired() {
            return true;
        }

        // Check rotation interval
        if let (Some(rotation_interval), Some(activated_at)) =
            (self.rotation_interval, self.activated_at)
        {
            let age = Utc::now().signed_duration_since(activated_at);
            let rotation_threshold = chrono::Duration::from_std(rotation_interval)
                .unwrap_or_else(|_| chrono::Duration::zero());

            return age >= rotation_threshold;
        }

        false
    }

    /// Get key age since activation
    #[must_use]
    pub fn age(&self) -> Option<chrono::Duration> {
        self.activated_at
            .map(|activated| Utc::now().signed_duration_since(activated))
    }

    /// Record key usage
    pub fn record_usage(&mut self) {
        self.usage_count += 1;
        self.last_used_at = Some(Utc::now());
    }
}

/// Key rotation configuration
///
/// Controls automatic key rotation behavior, retention periods,
/// backup policies, and alerting thresholds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationConfig {
    /// Enable automatic rotation
    pub auto_rotation_enabled: bool,

    /// Rotation interval (default: 90 days)
    pub rotation_interval: Duration,

    /// Retain deprecated keys for decryption (default: 30 days)
    pub deprecation_period: Duration,

    /// Enable backup before rotation
    pub backup_before_rotation: bool,

    /// Maximum number of backups to retain per key
    pub max_backup_count: usize,

    /// Enable rotation alerts
    pub enable_expiration_alerts: bool,

    /// Alert threshold before expiration (default: 7 days)
    pub alert_threshold: Duration,

    /// Run auto-rotation check interval (default: 1 hour)
    pub check_interval: Duration,
}

impl Default for KeyRotationConfig {
    fn default() -> Self {
        Self {
            auto_rotation_enabled: true,
            rotation_interval: Duration::from_secs(90 * 24 * 3600), // 90 days
            deprecation_period: Duration::from_secs(30 * 24 * 3600), // 30 days
            backup_before_rotation: true,
            max_backup_count: 3,
            enable_expiration_alerts: true,
            alert_threshold: Duration::from_secs(7 * 24 * 3600), // 7 days
            check_interval: Duration::from_secs(3600),           // 1 hour
        }
    }
}

impl KeyRotationConfig {
    /// Create configuration for high-security environment
    ///
    /// More frequent rotation, shorter deprecation period, always backup.
    #[must_use]
    pub fn high_security() -> Self {
        Self {
            rotation_interval: Duration::from_secs(30 * 24 * 3600), // 30 days
            deprecation_period: Duration::from_secs(7 * 24 * 3600), // 7 days
            alert_threshold: Duration::from_secs(3 * 24 * 3600),    // 3 days
            ..Self::default()
        }
    }

    /// Create configuration for development environment
    ///
    /// Less frequent rotation, longer deprecation, optional backup.
    #[must_use]
    pub fn development() -> Self {
        Self {
            auto_rotation_enabled: false, // Manual rotation in dev
            rotation_interval: Duration::from_secs(365 * 24 * 3600), // 1 year
            deprecation_period: Duration::from_secs(90 * 24 * 3600), // 90 days
            backup_before_rotation: false,
            enable_expiration_alerts: false,
            ..Self::default()
        }
    }
}

/// Key rotation event for audit logging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationEvent {
    /// Event ID
    pub event_id: String,

    /// Old key ID
    pub old_key_id: String,

    /// New key ID
    pub new_key_id: String,

    /// Timestamp of rotation
    pub timestamp: DateTime<Utc>,

    /// Reason for rotation
    pub reason: KeyRotationReason,

    /// Whether rotation was successful
    pub success: bool,

    /// Error message (if failed)
    pub error: Option<String>,

    /// User/system that triggered rotation
    pub triggered_by: Option<String>,
}

/// Reason for key rotation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyRotationReason {
    /// Scheduled automatic rotation
    Scheduled,

    /// Key expired
    Expired,

    /// Manual rotation requested
    Manual,

    /// Key compromised or suspected compromise
    Compromised,

    /// Compliance requirement
    Compliance,

    /// Other reason
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_lifecycle_state_transitions() {
        let pending = KeyLifecycleState::Pending;
        assert!(!pending.can_encrypt());
        assert!(!pending.can_decrypt());
        assert!(!pending.can_rotate());

        let active = KeyLifecycleState::Active;
        assert!(active.can_encrypt());
        assert!(active.can_decrypt());
        assert!(active.can_rotate());

        let deprecated = KeyLifecycleState::Deprecated;
        assert!(!deprecated.can_encrypt());
        assert!(deprecated.can_decrypt());
        assert!(!deprecated.can_rotate());

        let destroyed = KeyLifecycleState::Destroyed;
        assert!(destroyed.is_terminal());
    }

    #[test]
    fn test_key_metadata_creation() {
        let metadata = KeyMetadataWithLifecycle::new(
            "test-key-1".to_string(),
            "aes".to_string(),
            "aes-256-gcm".to_string(),
        );

        assert_eq!(metadata.key_id, "test-key-1");
        assert_eq!(metadata.state, KeyLifecycleState::Pending);
        assert_eq!(metadata.usage_count, 0);
        assert!(metadata.activated_at.is_none());
    }

    #[test]
    fn test_key_expiration_check() {
        let mut metadata = KeyMetadataWithLifecycle::new(
            "test-key".to_string(),
            "aes".to_string(),
            "aes-256-gcm".to_string(),
        );

        // Not expired without expiration date
        assert!(!metadata.is_expired());

        // Set expiration in the past
        metadata.expires_at = Some(Utc::now() - chrono::Duration::days(1));
        assert!(metadata.is_expired());

        // Set expiration in the future
        metadata.expires_at = Some(Utc::now() + chrono::Duration::days(1));
        assert!(!metadata.is_expired());
    }

    #[test]
    fn test_key_rotation_needed() {
        let mut metadata = KeyMetadataWithLifecycle::new(
            "test-key".to_string(),
            "aes".to_string(),
            "aes-256-gcm".to_string(),
        );

        // No rotation needed without interval
        assert!(!metadata.needs_rotation());

        // Set rotation interval and activation time
        metadata.rotation_interval = Some(Duration::from_secs(30 * 24 * 3600)); // 30 days
        metadata.activated_at = Some(Utc::now() - chrono::Duration::days(31));

        // Should need rotation
        assert!(metadata.needs_rotation());
    }

    #[test]
    fn test_rotation_config_defaults() {
        let config = KeyRotationConfig::default();
        assert!(config.auto_rotation_enabled);
        assert_eq!(config.rotation_interval.as_secs(), 90 * 24 * 3600);
        assert_eq!(config.max_backup_count, 3);
    }

    #[test]
    fn test_high_security_config() {
        let config = KeyRotationConfig::high_security();
        assert_eq!(config.rotation_interval.as_secs(), 30 * 24 * 3600);
        assert_eq!(config.deprecation_period.as_secs(), 7 * 24 * 3600);
    }

    #[test]
    fn test_development_config() {
        let config = KeyRotationConfig::development();
        assert!(!config.auto_rotation_enabled);
        assert!(!config.backup_before_rotation);
    }

    #[test]
    fn test_record_usage() {
        let mut metadata = KeyMetadataWithLifecycle::new(
            "test-key".to_string(),
            "aes".to_string(),
            "aes-256-gcm".to_string(),
        );

        assert_eq!(metadata.usage_count, 0);
        assert!(metadata.last_used_at.is_none());

        metadata.record_usage();
        assert_eq!(metadata.usage_count, 1);
        assert!(metadata.last_used_at.is_some());

        metadata.record_usage();
        assert_eq!(metadata.usage_count, 2);
    }
}
