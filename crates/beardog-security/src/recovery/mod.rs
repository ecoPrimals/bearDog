

use beardog_errors::{BearDogError, BearDogResult};
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
pub struct RecoveryManager {
    pub config: UnifiedRecoveryConfig,
    pub active_keys: HashMap<String, EphemeralRecoveryKey>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(since = "3.1.0", note = "Use UnifiedRecoveryConfig instead")]
#[deprecated(since = "3.1.0", note = "Use UnifiedRecoveryConfig instead")]
pub struct RecoveryConfig {
    pub enable_social_recovery: bool,
    pub enable_shard_recovery: bool,
    pub enable_federation_recovery: bool,
    pub key_expiry_hours: u64,}

impl Default for RecoveryConfig {}

    fn default() -> Self {
        Self {
            enable_social_recovery: true,
            enable_shard_recovery: true,
            enable_federation_recovery: false,
            key_expiry_hours: 24,
        }
    }
pub struct EphemeralRecoveryKey {
    pub key_id: String,
    pub user_id: String,
    pub key_data: Vec<u8>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub is_active: bool,}

impl RecoveryManager {}

    pub fn new() -> Self {
            config: UnifiedRecoveryConfig::default(),
            active_keys: HashMap::with_capacity(16),}

    pub async fn generate_ephemeral_key(
        &mut self,
        user_id: &str,
        recovery_type: &str,
    ) -> BearDogResult<EphemeralRecoveryKey> {
        let key_id = Uuid::new_v4().to_string();
        let key_data = self.generate_secure_key().await?;
        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::hours(self.config.key_expiry_hours as i64);
        let key = EphemeralRecoveryKey {
            key_id: key_id.clone(),
            user_id: user_id.to_string(),
            key_data,
            created_at: now,
            expires_at,
            is_active: true,
        };
        self.active_keys.insert(key_id, key.clone());
        Ok(key)
    pub async fn validate_recovery_key(&self, key_id: &str) -> BearDogResult<bool> {
        if let Some(key) = self.active_keys.get(key_id) {
            let now = chrono::Utc::now();
            Ok(key.is_active && now < key.expires_at)
        } else {
            Ok(false)
    async fn generate_secure_key(&self) -> BearDogResult<Vec<u8>> {

        use rand::RngCore;
        let mut key = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);

