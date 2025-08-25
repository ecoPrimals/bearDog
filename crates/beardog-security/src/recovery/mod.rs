// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Security Recovery Module
///
/// This module provides security recovery mechanisms for key recovery,
/// account recovery, and security incident response.

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::security::{SecurityContext, SecurityLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
// Import recovery sub-modules
pub mod challenges;
pub mod ephemeral;
pub mod federation;
pub mod policies;
pub mod sessions;
pub mod shards;
pub mod social;
pub mod types;
// Re-export recovery types
pub use types::*;
/// Recovery Manager - Handles all recovery operations
#[derive(Debug, Clone)]
pub struct RecoveryManager {
    pub config: RecoveryConfig,
    pub active_keys: HashMap<String, EphemeralRecoveryKey>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
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
            config: RecoveryConfig::default(),
            active_keys: HashMap::new(),}


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
        // Generate a secure random key
        use rand::RngCore;
        let mut key = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);
// All recovery operations now use BearDogError directly for unified error handling
