# 🔄 Key Rotation and Lifecycle Management Specification

**Version**: 1.0.0  
**Date**: November 6, 2025 (spec) | **Reviewed**: July 25, 2026  
**Status**: 📋 **PARTIAL** — types/tests exist; automated rotation daemon not shipped

---

## 📋 Overview

### Purpose

Implement automated key rotation and comprehensive lifecycle management for cryptographic keys in BearDog's HSM and security infrastructure.

### Scope

- **Automated Key Rotation**: Periodic rotation based on time or usage thresholds
- **Key Expiration**: Detection and handling of expired keys
- **Lifecycle States**: Track keys through their entire lifecycle
- **Backup & Recovery**: Safe key backup before rotation
- **Audit Trail**: Complete audit logging of all lifecycle events

---

## 🎯 Requirements

### Functional Requirements

1. **Key Rotation**
   - Automatic rotation based on configurable intervals
   - Manual rotation on demand
   - Seamless rotation (no service interruption)
   - Rollback capability

2. **Key Lifecycle States**
   - `Pending`: Key generated but not yet active
   - `Active`: Key currently in use
   - `Rotating`: Key being rotated (transitional)
   - `Deprecated`: Key no longer active but still valid for decryption
   - `Expired`: Key past expiration date
   - `Revoked`: Key manually revoked
   - `Destroyed`: Key securely deleted

3. **Expiration Handling**
   - Configurable expiration periods
   - Automatic detection of expired keys
   - Graceful handling (allow decryption with expired keys)
   - Notification/alerts before expiration

4. **Backup & Recovery**
   - Automatic backup before rotation
   - Encrypted backup storage
   - Recovery from backup
   - Backup rotation (delete old backups)

5. **Audit & Monitoring**
   - Log all lifecycle events
   - Track key usage statistics
   - Alert on approaching expiration
   - Report on key inventory

### Non-Functional Requirements

1. **Security**
   - Keys never exposed in plaintext
   - Secure key deletion (zeroization)
   - Backup encryption
   - Access control for lifecycle operations

2. **Performance**
   - Rotation completes in <5 seconds
   - No service disruption during rotation
   - Minimal overhead for key lookups

3. **Reliability**
   - Atomic rotation operations
   - Rollback on failure
   - Idempotent operations

---

## 🏗️ Architecture

### Key Lifecycle State Machine

```
        [Generated]
             ↓
        [Pending] ←──────── Import
             ↓
        [Active] ←──────────┐
             ↓              │
      [Rotating] ───────────┘
             ↓
      [Deprecated]
             ↓
      [Expired/Revoked]
             ↓
      [Destroyed]
```

### Core Types

```rust
/// Key lifecycle state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyLifecycleState {
    /// Key generated but not yet active
    Pending,
    /// Key currently in use
    Active,
    /// Key being rotated (transitional state)
    Rotating,
    /// Key no longer active but still valid for decryption
    Deprecated,
    /// Key past expiration date
    Expired,
    /// Key manually revoked
    Revoked,
    /// Key securely deleted
    Destroyed,
}

/// Key metadata with lifecycle information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadataWithLifecycle {
    pub key_id: String,
    pub key_type: KeyType,
    pub algorithm: Algorithm,
    pub state: KeyLifecycleState,
    pub created_at: DateTime<Utc>,
    pub activated_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub deprecated_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub destroyed_at: Option<DateTime<Utc>>,
    pub rotation_interval: Option<Duration>,
    pub usage_count: u64,
    pub last_used_at: Option<DateTime<Utc>>,
    pub predecessor_key_id: Option<String>,
    pub successor_key_id: Option<String>,
    pub backup_encrypted: bool,
}

/// Key rotation configuration
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
    /// Maximum number of backups to retain
    pub max_backup_count: usize,
    /// Enable rotation alerts
    pub enable_expiration_alerts: bool,
    /// Alert threshold before expiration (default: 7 days)
    pub alert_threshold: Duration,
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
        }
    }
}
```

---

## 🔧 Implementation

### Key Rotation Manager

```rust
/// Manages key rotation and lifecycle
pub struct KeyRotationManager {
    hsm: Arc<dyn HsmProvider>,
    config: KeyRotationConfig,
    backup_storage: Arc<dyn BackupStorage>,
    audit_logger: Arc<dyn AuditLogger>,
    metadata_store: Arc<RwLock<HashMap<String, KeyMetadataWithLifecycle>>>,
}

impl KeyRotationManager {
    /// Create new rotation manager
    pub fn new(
        hsm: Arc<dyn HsmProvider>,
        config: KeyRotationConfig,
        backup_storage: Arc<dyn BackupStorage>,
        audit_logger: Arc<dyn AuditLogger>,
    ) -> Self {
        Self {
            hsm,
            config,
            backup_storage,
            audit_logger,
            metadata_store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Rotate a key
    pub async fn rotate_key(&self, key_id: &str) -> BearDogResult<String> {
        // 1. Get current key metadata
        let old_metadata = self.get_key_metadata(key_id).await?;
        
        // 2. Verify key can be rotated
        self.verify_rotation_allowed(&old_metadata)?;
        
        // 3. Backup current key (if enabled)
        if self.config.backup_before_rotation {
            self.backup_key(key_id).await?;
        }
        
        // 4. Update state to Rotating
        self.update_key_state(key_id, KeyLifecycleState::Rotating).await?;
        
        // 5. Generate new key with same parameters
        let new_key_id = format!("{}-rotated-{}", key_id, Utc::now().timestamp());
        let generate_request = KeyGenerationRequest {
            key_id: new_key_id.clone(),
            key_type: old_metadata.key_type,
            algorithm: old_metadata.algorithm,
            ..Default::default()
        };
        
        // 6. Generate new key
        match self.hsm.generate_key(generate_request).await {
            Ok(_) => {
                // 7. Activate new key
                self.update_key_state(&new_key_id, KeyLifecycleState::Active).await?;
                
                // 8. Deprecate old key
                self.update_key_state(key_id, KeyLifecycleState::Deprecated).await?;
                
                // 9. Link keys (predecessor/successor)
                self.link_keys(key_id, &new_key_id).await?;
                
                // 10. Audit log
                self.audit_logger.log_key_rotation(key_id, &new_key_id)?;
                
                Ok(new_key_id)
            }
            Err(e) => {
                // Rollback on failure
                self.update_key_state(key_id, KeyLifecycleState::Active).await?;
                Err(e)
            }
        }
    }
    
    /// Check for keys needing rotation
    pub async fn check_rotation_needed(&self) -> BearDogResult<Vec<String>> {
        let metadata = self.metadata_store.read().await;
        let now = Utc::now();
        
        let mut keys_to_rotate = Vec::new();
        
        for (key_id, meta) in metadata.iter() {
            if meta.state != KeyLifecycleState::Active {
                continue;
            }
            
            // Check expiration
            if let Some(expires_at) = meta.expires_at {
                if now >= expires_at {
                    keys_to_rotate.push(key_id.clone());
                    continue;
                }
            }
            
            // Check rotation interval
            if let Some(rotation_interval) = meta.rotation_interval {
                if let Some(activated_at) = meta.activated_at {
                    if now.signed_duration_since(activated_at) >= rotation_interval.into() {
                        keys_to_rotate.push(key_id.clone());
                    }
                }
            }
        }
        
        Ok(keys_to_rotate)
    }
    
    /// Automated rotation task (run periodically)
    pub async fn run_auto_rotation(&self) -> BearDogResult<usize> {
        if !self.config.auto_rotation_enabled {
            return Ok(0);
        }
        
        let keys_to_rotate = self.check_rotation_needed().await?;
        let mut rotated_count = 0;
        
        for key_id in keys_to_rotate {
            match self.rotate_key(&key_id).await {
                Ok(new_key_id) => {
                    info!("✅ Rotated key {} → {}", key_id, new_key_id);
                    rotated_count += 1;
                }
                Err(e) => {
                    error!("❌ Failed to rotate key {}: {}", key_id, e);
                }
            }
        }
        
        Ok(rotated_count)
    }
    
    /// Clean up deprecated keys past retention period
    pub async fn cleanup_deprecated_keys(&self) -> BearDogResult<usize> {
        let metadata = self.metadata_store.read().await;
        let now = Utc::now();
        let mut cleaned_up = 0;
        
        for (key_id, meta) in metadata.iter() {
            if meta.state == KeyLifecycleState::Deprecated {
                if let Some(deprecated_at) = meta.deprecated_at {
                    let age = now.signed_duration_since(deprecated_at);
                    if age >= self.config.deprecation_period.into() {
                        drop(metadata); // Release read lock
                        self.revoke_key(key_id).await?;
                        cleaned_up += 1;
                        // Re-acquire lock for next iteration
                        let metadata = self.metadata_store.read().await;
                    }
                }
            }
        }
        
        Ok(cleaned_up)
    }
    
    /// Revoke a key
    pub async fn revoke_key(&self, key_id: &str) -> BearDogResult<()> {
        self.update_key_state(key_id, KeyLifecycleState::Revoked).await?;
        self.audit_logger.log_key_revocation(key_id)?;
        Ok(())
    }
    
    /// Get key metadata
    pub async fn get_key_metadata(&self, key_id: &str) -> BearDogResult<KeyMetadataWithLifecycle> {
        let metadata = self.metadata_store.read().await;
        metadata
            .get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Key {} not found", key_id)))
    }
    
    // Helper methods...
    
    async fn update_key_state(&self, key_id: &str, new_state: KeyLifecycleState) -> BearDogResult<()> {
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
        }
        Ok(())
    }
    
    async fn link_keys(&self, old_key_id: &str, new_key_id: &str) -> BearDogResult<()> {
        let mut metadata = self.metadata_store.write().await;
        
        if let Some(old_meta) = metadata.get_mut(old_key_id) {
            old_meta.successor_key_id = Some(new_key_id.to_string());
        }
        
        if let Some(new_meta) = metadata.get_mut(new_key_id) {
            new_meta.predecessor_key_id = Some(old_key_id.to_string());
        }
        
        Ok(())
    }
    
    async fn backup_key(&self, key_id: &str) -> BearDogResult<()> {
        // Implementation depends on backup storage interface
        self.backup_storage.backup_key(key_id).await
    }
    
    fn verify_rotation_allowed(&self, metadata: &KeyMetadataWithLifecycle) -> BearDogResult<()> {
        match metadata.state {
            KeyLifecycleState::Active => Ok(()),
            KeyLifecycleState::Rotating => {
                Err(BearDogError::invalid_state("Key already rotating"))
            }
            _ => Err(BearDogError::invalid_state(format!(
                "Cannot rotate key in state: {:?}",
                metadata.state
            ))),
        }
    }
}
```

---

## 🧪 Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_key_rotation_success() {
        let manager = create_test_manager();
        let key_id = "test-key-1";
        
        // Generate initial key
        let gen_request = KeyGenerationRequest {
            key_id: key_id.to_string(),
            key_type: KeyType::Aes,
            algorithm: Algorithm::Aes256Gcm,
            ..Default::default()
        };
        manager.hsm.generate_key(gen_request).await.unwrap();
        
        // Rotate key
        let new_key_id = manager.rotate_key(key_id).await.unwrap();
        
        // Verify old key is deprecated
        let old_meta = manager.get_key_metadata(key_id).await.unwrap();
        assert_eq!(old_meta.state, KeyLifecycleState::Deprecated);
        
        // Verify new key is active
        let new_meta = manager.get_key_metadata(&new_key_id).await.unwrap();
        assert_eq!(new_meta.state, KeyLifecycleState::Active);
        
        // Verify linkage
        assert_eq!(old_meta.successor_key_id, Some(new_key_id.clone()));
        assert_eq!(new_meta.predecessor_key_id, Some(key_id.to_string()));
    }
    
    #[tokio::test]
    async fn test_auto_rotation_detects_expired_keys() {
        let manager = create_test_manager();
        
        // Create key that's already expired
        let key_id = "expired-key";
        create_test_key_with_expiration(
            &manager,
            key_id,
            Utc::now() - chrono::Duration::days(1),
        ).await;
        
        // Check for rotation needed
        let keys_to_rotate = manager.check_rotation_needed().await.unwrap();
        assert!(keys_to_rotate.contains(&key_id.to_string()));
    }
    
    #[tokio::test]
    async fn test_cleanup_deprecated_keys() {
        let manager = create_test_manager();
        
        // Create deprecated key past retention period
        let key_id = "old-deprecated-key";
        create_deprecated_key(
            &manager,
            key_id,
            Utc::now() - chrono::Duration::days(31),
        ).await;
        
        // Run cleanup
        let cleaned = manager.cleanup_deprecated_keys().await.unwrap();
        assert_eq!(cleaned, 1);
        
        // Verify key is revoked
        let meta = manager.get_key_metadata(key_id).await.unwrap();
        assert_eq!(meta.state, KeyLifecycleState::Revoked);
    }
    
    #[tokio::test]
    async fn test_rotation_rollback_on_failure() {
        let manager = create_test_manager_with_failing_hsm();
        let key_id = "test-key";
        
        // Create initial key
        create_test_key(&manager, key_id).await;
        
        // Attempt rotation (will fail)
        let result = manager.rotate_key(key_id).await;
        assert!(result.is_err());
        
        // Verify key is still active (rolled back)
        let meta = manager.get_key_metadata(key_id).await.unwrap();
        assert_eq!(meta.state, KeyLifecycleState::Active);
    }
}
```

### Integration Tests

- Test with real HSM providers
- Test backup and recovery
- Test concurrent rotation attempts
- Test rotation under load

---

## 📊 Monitoring & Metrics

### Metrics to Track

1. **Rotation Metrics**
   - Keys rotated (per day/week/month)
   - Rotation success rate
   - Average rotation duration
   - Failed rotations

2. **Lifecycle Metrics**
   - Active keys count
   - Deprecated keys count
   - Expired keys count
   - Keys by state

3. **Alert Conditions**
   - Keys approaching expiration (<7 days)
   - Keys past expiration
   - Rotation failures
   - Backup failures

---

## ✅ Acceptance Criteria

1. ✅ Key rotation completes successfully
2. ✅ Old key is deprecated, new key is active
3. ✅ Keys are properly linked (predecessor/successor)
4. ✅ Backup created before rotation
5. ✅ Audit log entries created
6. ✅ Automatic rotation runs on schedule
7. ✅ Expired keys detected and rotated
8. ✅ Deprecated keys cleaned up after retention period
9. ✅ Rollback works on rotation failure
10. ✅ All tests passing

---

## 🚀 Implementation Plan

### Phase 1: Core Types & Infrastructure (2 hours)
- Define `KeyLifecycleState` enum
- Create `KeyMetadataWithLifecycle` struct
- Implement `KeyRotationConfig`

### Phase 2: Rotation Manager (3 hours)
- Implement `KeyRotationManager`
- Implement `rotate_key()` method
- Implement state transitions

### Phase 3: Automation (2 hours)
- Implement `check_rotation_needed()`
- Implement `run_auto_rotation()`
- Implement `cleanup_deprecated_keys()`

### Phase 4: Testing (1 hour)
- Write unit tests
- Write integration tests
- Verify all acceptance criteria

**Total Estimate**: ~8 hours

---

**Specification Version**: 1.0.0  
**Last Updated**: November 6, 2025  
**Status**: Ready for Implementation

