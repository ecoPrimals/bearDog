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
            key_value: Self::generate_secure_key(), // Securely generated key
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

    /// Generate a cryptographically secure ephemeral key
    fn generate_secure_key() -> String {
        use rand::Rng;

        // Generate 32 bytes of random data (256 bits)
        let random_bytes: Vec<u8> = (0..32).map(|_| rand::thread_rng().gen()).collect();

        // Convert to base64 for storage
        use base64::Engine as _;
        base64::engine::general_purpose::STANDARD_NO_PAD.encode(random_bytes)
    }

    /// Derive a time-bound recovery key with HKDF
    pub fn derive_time_bound_key(&self, salt: &[u8], info: &[u8]) -> Result<String, String> {
        use hkdf::Hkdf;

        use sha2::Sha256;

        // Decode the master key
        use base64::Engine as _;
        let master_key = base64::engine::general_purpose::STANDARD_NO_PAD
            .decode(&self.key_value)
            .map_err(|_| "Invalid key format")?;

        // Use HKDF to derive a time-bound key
        let hk = Hkdf::<Sha256>::new(Some(salt), &master_key);
        let mut derived_key = [0u8; 32];
        hk.expand(info, &mut derived_key)
            .map_err(|_| "Key derivation failed")?;

        // Return derived key as base64
        Ok(base64::engine::general_purpose::STANDARD_NO_PAD.encode(derived_key))
    }

    /// Generate proof that the key holder possesses the key without revealing it
    pub fn generate_possession_proof(&self, challenge: &[u8]) -> Result<String, String> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        // Decode the key
        use base64::Engine as _;
        let key_bytes = base64::engine::general_purpose::STANDARD_NO_PAD
            .decode(&self.key_value)
            .map_err(|_| "Invalid key format")?;

        // Create HMAC proof
        let mut mac =
            Hmac::<Sha256>::new_from_slice(&key_bytes).map_err(|_| "HMAC creation failed")?;
        mac.update(challenge);
        let proof = mac.finalize().into_bytes();

        // Return proof as hex
        Ok(hex::encode(proof))
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
