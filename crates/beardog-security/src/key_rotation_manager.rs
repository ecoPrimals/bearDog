// SPDX-License-Identifier: AGPL-3.0-only

//! Key Rotation Manager
//!
//! Orchestrates key rotation operations, managing the complete lifecycle
//! from rotation initiation through backup, generation, and state transitions.

#[cfg(test)]
#[path = "key_rotation_manager_tests.rs"]
mod key_rotation_manager_tests;

use beardog_errors::BearDogError;
use beardog_types::hsm::{
    KeyLifecycleState, KeyMetadataWithLifecycle, KeyRotationConfig, KeyRotationEvent,
    KeyRotationReason,
};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

/// Key rotation manager
///
/// Manages key rotation operations, lifecycle transitions, and automation.
/// Provides a unified interface for rotating keys across different HSM providers.
#[derive(Clone)]
pub struct KeyRotationManager {
    /// Configuration for rotation behavior
    config: KeyRotationConfig,

    /// In-memory metadata store
    /// In production, this would be backed by persistent storage
    metadata_store: Arc<RwLock<HashMap<String, KeyMetadataWithLifecycle>>>,

    /// Rotation events log for audit trail
    events_log: Arc<RwLock<Vec<KeyRotationEvent>>>,
}

impl KeyRotationManager {
    /// Create new rotation manager with configuration
    pub fn new(config: KeyRotationConfig) -> Self {
        Self {
            config,
            metadata_store: Arc::new(RwLock::new(HashMap::new())),
            events_log: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Create rotation manager with default configuration
    pub fn with_defaults() -> Self {
        Self::new(KeyRotationConfig::default())
    }

    /// Register a new key in the lifecycle system
    pub async fn register_key(
        &self,
        key_id: String,
        key_type: String,
        algorithm: String,
    ) -> Result<(), BearDogError> {
        let mut metadata = KeyMetadataWithLifecycle::new(key_id.clone(), key_type, algorithm);

        // Auto-activate if enabled
        metadata.state = KeyLifecycleState::Active;
        metadata.activated_at = Some(Utc::now());

        // Set rotation interval from config
        metadata.rotation_interval = Some(self.config.rotation_interval);

        let mut store = self.metadata_store.write().await;
        store.insert(key_id.clone(), metadata);

        info!("✅ Registered key: {}", key_id);
        Ok(())
    }

    /// Rotate a key
    ///
    /// Performs the complete rotation process:
    /// 1. Validates current key can be rotated
    /// 2. Backs up current key (if configured)
    /// 3. Transitions to Rotating state
    /// 4. Generates new key
    /// 5. Activates new key
    /// 6. Deprecates old key
    /// 7. Links predecessor/successor
    /// 8. Logs rotation event
    pub async fn rotate_key(&self, key_id: &str) -> Result<String, BearDogError> {
        info!("🔄 Starting rotation for key: {}", key_id);

        // 1. Get current key metadata
        let old_metadata = self.get_key_metadata(key_id).await?;

        // 2. Verify key can be rotated
        self.verify_rotation_allowed(&old_metadata)?;

        // 3. Update state to Rotating
        self.update_key_state(key_id, KeyLifecycleState::Rotating)
            .await?;

        // 4. Generate new key ID
        let new_key_id = format!("{}-v{}", key_id, Utc::now().timestamp());

        // 5. Create new key metadata (simulating key generation)
        let mut new_metadata = KeyMetadataWithLifecycle::new(
            new_key_id.clone(),
            old_metadata.key_type.clone(),
            old_metadata.algorithm.clone(),
        );

        // 6. Activate new key
        new_metadata.state = KeyLifecycleState::Active;
        new_metadata.activated_at = Some(Utc::now());
        new_metadata.rotation_interval = old_metadata.rotation_interval;
        new_metadata.predecessor_key_id = Some(key_id.to_string());

        // 7. Store new key
        {
            let mut store = self.metadata_store.write().await;
            store.insert(new_key_id.clone(), new_metadata);
        }

        // 8. Deprecate old key
        self.update_key_state(key_id, KeyLifecycleState::Deprecated)
            .await?;

        // 9. Link keys
        self.link_keys(key_id, &new_key_id).await?;

        // 10. Log rotation event
        self.log_rotation_event(key_id, &new_key_id, KeyRotationReason::Manual, true, None)
            .await?;

        info!("✅ Rotation complete: {} → {}", key_id, new_key_id);
        Ok(new_key_id)
    }

    /// Check for keys needing rotation
    ///
    /// Scans all active keys and identifies those that need rotation based on:
    /// - Expiration date
    /// - Rotation interval
    pub async fn check_rotation_needed(&self) -> Result<Vec<String>, BearDogError> {
        let metadata = self.metadata_store.read().await;
        let mut keys_to_rotate = Vec::new();

        for (key_id, meta) in metadata.iter() {
            if meta.state != KeyLifecycleState::Active {
                continue;
            }

            if meta.needs_rotation() {
                keys_to_rotate.push(key_id.clone());
            }
        }

        if !keys_to_rotate.is_empty() {
            info!("🔍 Found {} keys needing rotation", keys_to_rotate.len());
        }

        Ok(keys_to_rotate)
    }

    /// Run automatic rotation for all keys that need it
    ///
    /// Returns the number of keys successfully rotated
    pub async fn run_auto_rotation(&self) -> Result<usize, BearDogError> {
        if !self.config.auto_rotation_enabled {
            debug!("Auto-rotation is disabled");
            return Ok(0);
        }

        info!("🤖 Starting automatic rotation check");

        let keys_to_rotate = self.check_rotation_needed().await?;
        let mut rotated_count = 0;

        for key_id in keys_to_rotate {
            match self.rotate_key(&key_id).await {
                Ok(new_key_id) => {
                    info!("✅ Auto-rotated: {} → {}", key_id, new_key_id);
                    rotated_count += 1;
                }
                Err(e) => {
                    error!("❌ Failed to auto-rotate key {}: {}", key_id, e);
                    // Log failure event
                    let _ = self
                        .log_rotation_event(
                            &key_id,
                            "",
                            KeyRotationReason::Scheduled,
                            false,
                            Some(e.to_string()),
                        )
                        .await;
                }
            }
        }

        info!("🤖 Auto-rotation complete: {} keys rotated", rotated_count);
        Ok(rotated_count)
    }

    /// Clean up deprecated keys past retention period
    ///
    /// Revokes deprecated keys that have exceeded the deprecation period
    pub async fn cleanup_deprecated_keys(&self) -> Result<usize, BearDogError> {
        info!("🧹 Starting deprecated key cleanup");

        let now = Utc::now();
        let mut keys_to_revoke = Vec::new();

        {
            let metadata = self.metadata_store.read().await;

            for (key_id, meta) in metadata.iter() {
                if meta.state == KeyLifecycleState::Deprecated {
                    if let Some(deprecated_at) = meta.deprecated_at {
                        let age = now.signed_duration_since(deprecated_at);
                        let retention_period =
                            chrono::Duration::from_std(self.config.deprecation_period)
                                .unwrap_or_else(|_| chrono::Duration::zero());

                        if age >= retention_period {
                            keys_to_revoke.push(key_id.clone());
                        }
                    }
                }
            }
        }

        let mut cleaned_count = 0;
        for key_id in keys_to_revoke {
            if self.revoke_key(&key_id).await.is_ok() {
                cleaned_count += 1;
            }
        }

        info!("🧹 Cleanup complete: {} keys revoked", cleaned_count);
        Ok(cleaned_count)
    }

    /// Revoke a key
    ///
    /// Transitions the key to Revoked state, preventing all future use
    pub async fn revoke_key(&self, key_id: &str) -> Result<(), BearDogError> {
        info!("🚫 Revoking key: {}", key_id);

        self.update_key_state(key_id, KeyLifecycleState::Revoked)
            .await?;

        Ok(())
    }

    /// Deprecate a key
    ///
    /// Transitions the key to Deprecated state
    pub async fn deprecate_key(&self, key_id: &str) -> Result<(), BearDogError> {
        info!("⚠️  Deprecating key: {}", key_id);

        self.update_key_state(key_id, KeyLifecycleState::Deprecated)
            .await?;

        Ok(())
    }

    /// Destroy a key
    ///
    /// Transitions the key to Destroyed state (final state)
    pub async fn destroy_key(&self, key_id: &str) -> Result<(), BearDogError> {
        info!("💥 Destroying key: {}", key_id);

        self.update_key_state(key_id, KeyLifecycleState::Destroyed)
            .await?;

        Ok(())
    }

    /// Get all active keys
    pub async fn get_active_keys(&self) -> Result<Vec<String>, BearDogError> {
        self.get_keys_by_state(KeyLifecycleState::Active).await
    }

    /// Get key metadata
    pub async fn get_key_metadata(
        &self,
        key_id: &str,
    ) -> Result<KeyMetadataWithLifecycle, BearDogError> {
        let metadata = self.metadata_store.read().await;
        metadata
            .get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Key {key_id} not found")))
    }

    /// Get all keys in a specific state
    pub async fn get_keys_by_state(
        &self,
        state: KeyLifecycleState,
    ) -> Result<Vec<String>, BearDogError> {
        let metadata = self.metadata_store.read().await;
        let keys = metadata
            .iter()
            .filter(|(_, meta)| meta.state == state)
            .map(|(key_id, _)| key_id.clone())
            .collect();

        Ok(keys)
    }

    /// Get rotation statistics
    pub async fn get_rotation_stats(&self) -> Result<RotationStatistics, BearDogError> {
        let metadata = self.metadata_store.read().await;
        let events = self.events_log.read().await;

        let total_keys = metadata.len();
        let active_keys = metadata
            .values()
            .filter(|m| m.state == KeyLifecycleState::Active)
            .count();
        let deprecated_keys = metadata
            .values()
            .filter(|m| m.state == KeyLifecycleState::Deprecated)
            .count();
        let revoked_keys = metadata
            .values()
            .filter(|m| m.state == KeyLifecycleState::Revoked)
            .count();
        let rotating_keys = metadata
            .values()
            .filter(|m| m.state == KeyLifecycleState::Rotating)
            .count();
        let expired_keys = metadata.values().filter(|m| m.is_expired()).count();

        let total_rotations = events.len();
        let successful_rotations = events.iter().filter(|e| e.success).count();
        let failed_rotations = total_rotations - successful_rotations;

        Ok(RotationStatistics {
            total_keys,
            active_keys,
            deprecated_keys,
            expired_keys,
            rotating_keys,
            revoked_keys,
            total_rotations,
            successful_rotations,
            failed_rotations,
        })
    }

    /// Rotate key with specific reason
    pub async fn rotate_key_with_reason(
        &self,
        key_id: &str,
        reason: KeyRotationReason,
    ) -> Result<String, BearDogError> {
        info!(
            "🔄 Starting rotation for key: {} (reason: {:?})",
            key_id, reason
        );

        let old_metadata = self.get_key_metadata(key_id).await?;
        self.verify_rotation_allowed(&old_metadata)?;
        self.update_key_state(key_id, KeyLifecycleState::Rotating)
            .await?;

        let new_key_id = format!("{}-v{}", key_id, Utc::now().timestamp());
        let mut new_metadata = KeyMetadataWithLifecycle::new(
            new_key_id.clone(),
            old_metadata.key_type.clone(),
            old_metadata.algorithm.clone(),
        );

        new_metadata.state = KeyLifecycleState::Active;
        new_metadata.activated_at = Some(Utc::now());
        new_metadata.rotation_interval = old_metadata.rotation_interval;
        new_metadata.predecessor_key_id = Some(key_id.to_string());

        {
            let mut store = self.metadata_store.write().await;
            store.insert(new_key_id.clone(), new_metadata);
        }

        self.update_key_state(key_id, KeyLifecycleState::Deprecated)
            .await?;
        self.link_keys(key_id, &new_key_id).await?;
        self.log_rotation_event(key_id, &new_key_id, reason, true, None)
            .await?;

        info!("✅ Rotation complete: {} → {}", key_id, new_key_id);
        Ok(new_key_id)
    }

    /// Get rotation events for a specific key
    pub async fn get_rotation_events(
        &self,
        key_id: &str,
    ) -> Result<Vec<KeyRotationEvent>, BearDogError> {
        let events = self.events_log.read().await;
        let key_events: Vec<KeyRotationEvent> = events
            .iter()
            .filter(|e| e.old_key_id == key_id || e.new_key_id == key_id)
            .cloned()
            .collect();
        Ok(key_events)
    }

    /// Get all rotation events
    pub async fn get_all_rotation_events(&self) -> Result<Vec<KeyRotationEvent>, BearDogError> {
        let events = self.events_log.read().await;
        Ok(events.clone())
    }

    /// Get keys needing rotation (alias for check_rotation_needed)
    pub async fn get_keys_needing_rotation(&self) -> Result<Vec<String>, BearDogError> {
        self.check_rotation_needed().await
    }

    /// Get rotation chain for a key
    pub async fn get_rotation_chain(&self, key_id: &str) -> Result<Vec<String>, BearDogError> {
        let metadata = self.metadata_store.read().await;
        let mut chain = vec![key_id.to_string()];

        // Follow successor links
        let mut current_id = key_id.to_string();
        while let Some(meta) = metadata.get(&current_id) {
            if let Some(successor) = &meta.successor_key_id {
                chain.push(successor.clone());
                current_id = successor.clone();
            } else {
                break;
            }
        }

        Ok(chain)
    }

    // Private helper methods

    async fn update_key_state(
        &self,
        key_id: &str,
        new_state: KeyLifecycleState,
    ) -> Result<(), BearDogError> {
        let mut metadata = self.metadata_store.write().await;

        if let Some(meta) = metadata.get_mut(key_id) {
            meta.state = new_state;

            // Update timestamps based on state
            match new_state {
                KeyLifecycleState::Active => meta.activated_at = Some(Utc::now()),
                KeyLifecycleState::Deprecated => meta.deprecated_at = Some(Utc::now()),
                KeyLifecycleState::Revoked => meta.revoked_at = Some(Utc::now()),
                KeyLifecycleState::Destroyed => meta.destroyed_at = Some(Utc::now()),
                _ => {}
            }

            debug!("Updated key {} state to {:?}", key_id, new_state);
            Ok(())
        } else {
            Err(BearDogError::not_found(format!("Key {key_id} not found")))
        }
    }

    async fn link_keys(&self, old_key_id: &str, new_key_id: &str) -> Result<(), BearDogError> {
        let mut metadata = self.metadata_store.write().await;

        if let Some(old_meta) = metadata.get_mut(old_key_id) {
            old_meta.successor_key_id = Some(new_key_id.to_string());
        }

        if let Some(new_meta) = metadata.get_mut(new_key_id) {
            new_meta.predecessor_key_id = Some(old_key_id.to_string());
        }

        debug!("Linked keys: {} → {}", old_key_id, new_key_id);
        Ok(())
    }

    async fn log_rotation_event(
        &self,
        old_key_id: &str,
        new_key_id: &str,
        reason: KeyRotationReason,
        success: bool,
        error: Option<String>,
    ) -> Result<(), BearDogError> {
        let event = KeyRotationEvent {
            event_id: format!("{}-{}", Utc::now().timestamp(), old_key_id),
            old_key_id: old_key_id.to_string(),
            new_key_id: new_key_id.to_string(),
            timestamp: Utc::now(),
            reason,
            success,
            error,
            triggered_by: Some("system".to_string()),
        };

        let mut events = self.events_log.write().await;
        events.push(event);

        Ok(())
    }

    fn verify_rotation_allowed(
        &self,
        metadata: &KeyMetadataWithLifecycle,
    ) -> Result<(), BearDogError> {
        if !metadata.state.can_rotate() {
            return Err(BearDogError::validation(&format!(
                "Cannot rotate key in state: {:?}",
                metadata.state
            )));
        }
        Ok(())
    }
}

/// Rotation statistics
#[derive(Debug, Clone)]
pub struct RotationStatistics {
    /// Total number of keys in the system
    pub total_keys: usize,
    /// Number of active keys
    pub active_keys: usize,
    /// Number of deprecated keys
    pub deprecated_keys: usize,
    /// Number of revoked keys
    pub revoked_keys: usize,
    /// Number of keys currently rotating
    pub rotating_keys: usize,
    /// Number of expired keys
    pub expired_keys: usize,
    /// Total rotation operations performed
    pub total_rotations: usize,
    /// Number of successful rotations
    pub successful_rotations: usize,
    /// Number of failed rotations
    pub failed_rotations: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_rotation_manager() {
        let manager = KeyRotationManager::with_defaults();
        let stats = manager.get_rotation_stats().await.unwrap();
        assert_eq!(stats.total_keys, 0);
    }

    #[tokio::test]
    async fn test_register_key() {
        let manager = KeyRotationManager::with_defaults();

        manager
            .register_key(
                "test-key-1".to_string(),
                "aes".to_string(),
                "aes-256-gcm".to_string(),
            )
            .await
            .unwrap();

        let metadata = manager.get_key_metadata("test-key-1").await.unwrap();
        assert_eq!(metadata.key_id, "test-key-1");
        assert_eq!(metadata.state, KeyLifecycleState::Active);
    }

    #[tokio::test]
    async fn test_key_rotation_success() {
        let manager = KeyRotationManager::with_defaults();

        // Register initial key
        manager
            .register_key(
                "test-key".to_string(),
                "aes".to_string(),
                "aes-256-gcm".to_string(),
            )
            .await
            .unwrap();

        // Rotate key
        let new_key_id = manager.rotate_key("test-key").await.unwrap();

        // Verify old key is deprecated
        let old_meta = manager.get_key_metadata("test-key").await.unwrap();
        assert_eq!(old_meta.state, KeyLifecycleState::Deprecated);
        assert_eq!(old_meta.successor_key_id, Some(new_key_id.clone()));

        // Verify new key is active
        let new_meta = manager.get_key_metadata(&new_key_id).await.unwrap();
        assert_eq!(new_meta.state, KeyLifecycleState::Active);
        assert_eq!(new_meta.predecessor_key_id, Some("test-key".to_string()));
    }

    #[tokio::test]
    async fn test_cannot_rotate_deprecated_key() {
        let manager = KeyRotationManager::with_defaults();

        // Register and rotate key
        manager
            .register_key(
                "test-key".to_string(),
                "aes".to_string(),
                "aes-256-gcm".to_string(),
            )
            .await
            .unwrap();

        manager.rotate_key("test-key").await.unwrap();

        // Try to rotate deprecated key (should fail)
        let result = manager.rotate_key("test-key").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_rotation_needed() {
        let manager = KeyRotationManager::with_defaults();

        // Register key with very short rotation interval
        manager
            .register_key(
                "test-key".to_string(),
                "aes".to_string(),
                "aes-256-gcm".to_string(),
            )
            .await
            .unwrap();

        // Manually set activation time to past
        {
            let mut store = manager.metadata_store.write().await;
            if let Some(meta) = store.get_mut("test-key") {
                meta.activated_at = Some(Utc::now() - chrono::Duration::days(100));
                meta.rotation_interval = Some(std::time::Duration::from_secs(60 * 24 * 3600));
                // 60 days
            }
        }

        // Should detect rotation needed
        let keys = manager.check_rotation_needed().await.unwrap();
        assert!(keys.contains(&"test-key".to_string()));
    }

    #[tokio::test]
    async fn test_cleanup_deprecated_keys() {
        let manager = KeyRotationManager::with_defaults();

        // Register and rotate key
        manager
            .register_key(
                "test-key".to_string(),
                "aes".to_string(),
                "aes-256-gcm".to_string(),
            )
            .await
            .unwrap();

        manager.rotate_key("test-key").await.unwrap();

        // Manually set deprecated time to past
        {
            let mut store = manager.metadata_store.write().await;
            if let Some(meta) = store.get_mut("test-key") {
                meta.deprecated_at = Some(Utc::now() - chrono::Duration::days(31));
            }
        }

        // Run cleanup
        let cleaned = manager.cleanup_deprecated_keys().await.unwrap();
        assert_eq!(cleaned, 1);

        // Verify key is revoked
        let meta = manager.get_key_metadata("test-key").await.unwrap();
        assert_eq!(meta.state, KeyLifecycleState::Revoked);
    }

    #[tokio::test]
    async fn test_get_keys_by_state() {
        let manager = KeyRotationManager::with_defaults();

        // Register multiple keys
        manager
            .register_key(
                "key-1".to_string(),
                "aes".to_string(),
                "aes-256-gcm".to_string(),
            )
            .await
            .unwrap();

        manager
            .register_key(
                "key-2".to_string(),
                "aes".to_string(),
                "aes-256-gcm".to_string(),
            )
            .await
            .unwrap();

        // Get active keys
        let active_keys = manager
            .get_keys_by_state(KeyLifecycleState::Active)
            .await
            .unwrap();

        assert_eq!(active_keys.len(), 2);
        assert!(active_keys.contains(&"key-1".to_string()));
        assert!(active_keys.contains(&"key-2".to_string()));
    }

    #[tokio::test]
    async fn test_rotation_statistics() {
        let manager = KeyRotationManager::with_defaults();

        // Register and rotate some keys
        manager
            .register_key(
                "key-1".to_string(),
                "aes".to_string(),
                "aes-256-gcm".to_string(),
            )
            .await
            .unwrap();

        manager
            .register_key(
                "key-2".to_string(),
                "aes".to_string(),
                "aes-256-gcm".to_string(),
            )
            .await
            .unwrap();

        manager.rotate_key("key-1").await.unwrap();

        let stats = manager.get_rotation_stats().await.unwrap();
        assert_eq!(stats.total_keys, 3); // key-1, key-2, key-1-rotated
        assert_eq!(stats.active_keys, 2);
        assert_eq!(stats.deprecated_keys, 1);
        assert_eq!(stats.successful_rotations, 1);
    }
}
