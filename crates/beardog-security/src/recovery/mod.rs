

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::canonical::security::{SecurityContext, SecurityLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub mod challenges;
pub mod ephemeral;
pub mod federation;
pub mod policies;
pub mod sessions;
pub mod shards;
pub mod social;
pub mod types;

pub use types::*;

#[derive(Debug, Clone)]
    /// Mapping of active keys
    pub active_keys: HashMap<String, EphemeralRecoveryKey>,
}

        }
    }
pub struct EphemeralRecoveryKey {
    pub key_id: String,
    pub user_id: String,
    /// Collection of key data
    pub key_data: Vec<u8>,
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The expires at value
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Whether is_active is enabled
    pub is_active: bool,}

impl RecoveryManager {}

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
            config: UnifiedRecoveryConfig::default(),
            active_keys: HashMap::with_capacity(&str,
        recovery_type: &str,
    ) -> Result<EphemeralRecoveryKey, BearDogError> {
        let key_id = Uuid::new_v4().to_string();
        let key_data = self.generate_secure_key()?;
        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::hours(self.config.key_expiry_hours as i64);
        let key = EphemeralRecoveryKey {
            key_id: key_id.clone(),
            user_id: user_id.to_string(), key);
        Ok(key)
/// Validate Recovery Key operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates recovery_key
    /// Validates recovery_key
    pub fn validate_recovery_key(&self, key_id: &str) -> Result<bool, BearDogError> {
        if let Some(key) = self.active_keys.get(key_id) {
            let now = chrono::Utc::now();
            Ok(key.is_active && now < key.expires_at)
        } else {
            Ok(false)
    fn generate_secure_key(&self) -> Result<Vec<u8>, BearDogError>> {

        use rand::RngCore;
        let mut key = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);

