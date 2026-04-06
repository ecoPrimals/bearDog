// SPDX-License-Identifier: AGPL-3.0-or-later

//! Split from `coverage_gap_tests_12`: HSM keys method coverage.

#[cfg(test)]
mod hsm_keys_methods_tests {
    use crate::canonical::hsm::keys::*;
    use std::collections::HashMap;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_hsm_key_new() {
        let key = HsmKey::new("key-1".into(), "AES".into(), 256);
        assert_eq!(key.key_id, "key-1");
        assert_eq!(key.algorithm, "AES");
        assert_eq!(key.key_size, 256);
    }

    #[test]
    fn test_hsm_key_is_active() {
        let key = HsmKey::new("key-1".into(), "AES".into(), 256);
        assert!(key.is_active());
    }

    #[test]
    fn test_hsm_key_supports_usage() {
        let mut key = HsmKey::new("key-1".into(), "AES".into(), 256);
        key.usage = vec![KeyUsage::Encrypt, KeyUsage::Decrypt];
        assert!(key.supports_usage(&KeyUsage::Encrypt));
        assert!(!key.supports_usage(&KeyUsage::Sign));
    }

    #[test]
    fn test_hsm_key_mark_accessed() {
        let mut key = HsmKey::new("key-1".into(), "AES".into(), 256);
        key.mark_accessed();
        assert!(key.last_accessed.is_some());
        assert_eq!(key.health.operation_count, 1);
    }

    #[test]
    fn test_hsm_key_add_remove_tag() {
        let mut key = HsmKey::new("key-1".into(), "AES".into(), 256);
        key.add_tag("production".to_string());
        assert!(key.metadata.tags.contains(&"production".to_string()));
        // Adding same tag again should not duplicate
        key.add_tag("production".to_string());
        assert_eq!(
            key.metadata
                .tags
                .iter()
                .filter(|t| *t == "production")
                .count(),
            1
        );
        key.remove_tag("production");
        assert!(!key.metadata.tags.contains(&"production".to_string()));
    }

    #[test]
    fn test_hsm_key_set_get_attribute() {
        let mut key = HsmKey::new("key-1".into(), "AES".into(), 256);
        key.set_attribute("owner".into(), "test-team".into());
        assert_eq!(key.get_attribute("owner"), Some(&"test-team".to_string()));
        assert_eq!(key.get_attribute("nonexistent"), None);
    }

    #[test]
    fn test_hsm_key_is_expired() {
        let mut key = HsmKey::new("key-1".into(), "AES".into(), 256);
        assert!(!key.is_expired()); // No expiry set
        // Set to past
        key.expires_at = Some(SystemTime::now() - Duration::from_secs(100));
        assert!(key.is_expired());
    }

    #[test]
    fn test_hsm_key_age_seconds() {
        let key = HsmKey::new("key-1".into(), "AES".into(), 256);
        assert!(key.age_seconds() < 5); // Just created
    }

    #[test]
    fn test_hsm_key_update_health() {
        let mut key = HsmKey::new("key-1".into(), "AES".into(), 256);
        key.update_health("degraded".to_string());
        assert_eq!(key.health.status, "degraded");
    }

    #[test]
    fn test_hsm_key_record_error() {
        let mut key = HsmKey::new("key-1".into(), "AES".into(), 256);
        for _ in 0..11 {
            key.record_error();
        }
        assert_eq!(key.health.error_count, 11);
        assert_eq!(key.health.status, "degraded");
    }

    #[test]
    fn test_hsm_key_needs_health_check() {
        let mut key = HsmKey::new("key-1".into(), "AES".into(), 256);
        // Just created, health was just checked
        assert!(!key.needs_health_check());
        // Force old check time
        key.health.last_check = SystemTime::now() - Duration::from_secs(7200);
        assert!(key.needs_health_check());
    }

    #[test]
    fn test_key_usage_variants() {
        let _ = KeyUsage::Encrypt;
        let _ = KeyUsage::Decrypt;
        let _ = KeyUsage::Sign;
        let _ = KeyUsage::Verify;
        let _ = KeyUsage::Derive;
        let _ = KeyUsage::Wrap;
        let _ = KeyUsage::Unwrap;
        let _ = KeyUsage::KeyAgreement;
    }

    #[test]
    fn test_key_material_default() {
        let m = KeyMaterial::default();
        assert_eq!(m.storage_type, StorageType::Software);
        assert!(m.key_data.is_empty());
        assert!(m.hsm_handle.is_none());
    }

    #[test]
    fn test_storage_type_variants() {
        let _ = StorageType::Software;
        let _ = StorageType::Hardware;
        let _ = StorageType::Reference;
        let _ = StorageType::Distributed;
    }

    #[test]
    fn test_key_metadata_default() {
        let m = KeyMetadata::default();
        assert_eq!(m.name, "Unnamed Key");
        assert_eq!(m.owner, "system");
        assert_eq!(m.version, 1);
    }

    #[test]
    fn test_key_health_default() {
        let h = KeyHealth::default();
        assert_eq!(h.status, "healthy");
        assert_eq!(h.check_interval, 3600);
        assert_eq!(h.operation_count, 0);
        assert_eq!(h.error_count, 0);
    }

    #[test]
    fn test_encryption_info_default() {
        let e = EncryptionInfo::default();
        assert_eq!(e.algorithm, "AES-256");
        assert_eq!(e.mode, "GCM");
    }

    #[test]
    fn test_backup_info_default() {
        let b = BackupInfo::default();
        assert!(b.enabled);
        assert_eq!(b.backup_location, "local");
    }

    #[test]
    fn test_key_lifecycle_state_variants() {
        let _ = KeyLifecycleState::Generating;
        let _ = KeyLifecycleState::Active;
        let _ = KeyLifecycleState::Suspended;
        let _ = KeyLifecycleState::Compromised;
        let _ = KeyLifecycleState::Expired;
        let _ = KeyLifecycleState::Destroyed;
        assert_eq!(KeyLifecycleState::default(), KeyLifecycleState::Active);
    }

    #[test]
    fn test_key_operation_variants() {
        let _ = KeyOperation::Encrypt;
        let _ = KeyOperation::Decrypt;
        let _ = KeyOperation::Sign;
        let _ = KeyOperation::Verify;
        let _ = KeyOperation::Derive;
        let _ = KeyOperation::Wrap;
        let _ = KeyOperation::Unwrap;
    }

    #[test]
    fn test_key_operation_request() {
        let r = KeyOperationRequest {
            key_id: "key-1".into(),
            operation: KeyOperation::Encrypt,
            input_data: vec![1, 2, 3],
            parameters: HashMap::new(),
            request_id: "req-1".into(),
        };
        assert_eq!(r.key_id, "key-1");
        assert_eq!(r.request_id, "req-1");
    }

    #[test]
    fn test_key_operation_response() {
        let r = KeyOperationResponse {
            request_id: "req-1".into(),
            result: OperationResult::Success,
            output_data: vec![4, 5, 6],
            metadata: HashMap::new(),
            processing_time_ms: 42,
        };
        assert_eq!(r.processing_time_ms, 42);
    }

    #[test]
    fn test_operation_result_variants() {
        let _ = OperationResult::Success;
        let _ = OperationResult::Failed {
            error: "test error".into(),
        };
        let _ = OperationResult::Pending;
        let _ = OperationResult::Cancelled;
    }

    #[test]
    fn test_key_manager_methods() {
        let id = KeyManager::generate_key_id();
        assert!(!id.is_empty());
        assert!(KeyManager::validate_key_id(&id));
        assert!(!KeyManager::validate_key_id(""));

        assert!(KeyManager::is_algorithm_supported("AES"));
        assert!(KeyManager::is_algorithm_supported("RSA"));
        assert!(KeyManager::is_algorithm_supported("ECC"));
        assert!(!KeyManager::is_algorithm_supported("XOR"));

        assert!(KeyManager::is_key_size_valid("AES", 256));
        assert!(KeyManager::is_key_size_valid("RSA", 2048));
        assert!(!KeyManager::is_key_size_valid("AES", 512));
        assert!(!KeyManager::is_key_size_valid("unknown", 256));

        assert_eq!(KeyManager::recommended_key_size("AES"), Some(256));
        assert_eq!(KeyManager::recommended_key_size("RSA"), Some(2048));
        assert_eq!(KeyManager::recommended_key_size("unknown"), None);
    }
}
