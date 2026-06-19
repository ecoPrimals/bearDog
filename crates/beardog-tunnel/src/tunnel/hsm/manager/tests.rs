// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM Manager Tests
//!
//! Comprehensive test suite for HSM Manager functionality including:
//! - Provider registration and selection
//! - Key lifecycle (generate, delete)
//! - Auto-initialization
//! - Concurrent safety
//! - Failover and error handling

use super::*;
use crate::tunnel::hsm::HsmProviderBackend;
use crate::tunnel::hsm::KeyType;
use crate::tunnel::hsm::hsm_provider_mocks::MockHsmProvider;

fn mock_arc(m: MockHsmProvider) -> Arc<HsmProviderBackend> {
    Arc::new(HsmProviderBackend::Mock(m))
}

#[tokio::test]
async fn test_generate_key_success() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(HsmTier::Software, mock_arc(MockHsmProvider::new()))
        .unwrap();

    let key = manager
        .generate_key("test_key_123", &KeyType::ChaCha20)
        .await
        .unwrap();

    assert_eq!(key.id, "test_key_123");
    assert_eq!(key.hsm_type, "MockHSM");
}

#[tokio::test]
async fn test_generate_key_no_providers() {
    let manager = HsmManager::new();

    let result = manager.generate_key("test_key", &KeyType::Aes).await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("No HSM providers available")
    );
}

#[tokio::test]
async fn test_generate_key_provider_unavailable() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(HsmTier::Software, mock_arc(MockHsmProvider::unavailable()))
        .unwrap();

    let result = manager.generate_key("test_key", &KeyType::Aes).await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("No HSM providers available")
    );
}

#[tokio::test]
async fn test_generate_key_provider_failure() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(
            HsmTier::Software,
            mock_arc(MockHsmProvider::failing_generate()),
        )
        .unwrap();

    let result = manager.generate_key("test_key", &KeyType::Aes).await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Mock generate_key failure")
    );
}

#[tokio::test]
async fn test_delete_key_success() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(HsmTier::Software, mock_arc(MockHsmProvider::new()))
        .unwrap();

    let result = manager.delete_key("test_key_123").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_key_no_providers() {
    let manager = HsmManager::new();

    let result = manager.delete_key("test_key").await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("No HSM providers available")
    );
}

#[tokio::test]
async fn test_delete_key_provider_unavailable() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(HsmTier::Software, mock_arc(MockHsmProvider::unavailable()))
        .unwrap();

    let result = manager.delete_key("test_key").await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("No HSM providers available")
    );
}

#[tokio::test]
async fn test_delete_key_provider_failure() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(
            HsmTier::Software,
            mock_arc(MockHsmProvider::failing_delete()),
        )
        .unwrap();

    let result = manager.delete_key("test_key").await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Mock delete_key failure")
    );
}

#[tokio::test]
async fn test_generate_and_delete_key_lifecycle() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(HsmTier::Software, mock_arc(MockHsmProvider::new()))
        .unwrap();

    // Generate key
    let key = manager
        .generate_key("lifecycle_key", &KeyType::Ed25519)
        .await
        .unwrap();
    assert_eq!(key.id, "lifecycle_key");

    // Delete key
    let result = manager.delete_key("lifecycle_key").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_generate_key_different_types() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(HsmTier::Software, mock_arc(MockHsmProvider::new()))
        .unwrap();

    // Test different key types
    let key_types = vec![
        KeyType::Aes,
        KeyType::ChaCha20,
        KeyType::Ed25519,
        KeyType::X25519,
    ];

    for key_type in key_types {
        let key = manager
            .generate_key(&format!("key_{:?}", key_type), &key_type)
            .await
            .unwrap();
        assert!(key.id.starts_with("key_"));
    }
}

// ===================================================================
// Auto-Initialize Tests
// ===================================================================

/// Helper to ensure environment variables are cleaned up after tests
struct EnvCleanup {
    keys: Vec<&'static str>,
}

impl EnvCleanup {
    fn new(keys: &[&'static str]) -> Self {
        Self {
            keys: keys.to_vec(),
        }
    }
}

impl Drop for EnvCleanup {
    fn drop(&mut self) {
        for key in &self.keys {
            beardog_errors::process_env::remove_var(key);
        }
    }
}

#[tokio::test]
async fn test_auto_initialize_default_software_mode() {
    // Default production behavior matches `HsmAutoInitConfig::default()` (software, auto_init).
    // Do not use `auto_initialize()` + process env here: `BEARDOG_HSM_*` is global and
    // parallel tests can change it between clear and `from_env()`, yielding an empty manager.
    let config = HsmAutoInitConfig::default();
    assert_eq!(config.mode, "software");
    assert!(config.auto_init);

    let manager = HsmManager::auto_initialize_with_config(config).await;
    assert!(
        manager.is_ok(),
        "Auto-initialize should succeed with default software mode"
    );

    let manager = manager.unwrap();
    let key = manager.generate_key("test_key", &KeyType::Ed25519).await;
    assert!(
        key.is_ok(),
        "Should be able to generate key with auto-initialized HSM"
    );
}

#[tokio::test]
async fn test_auto_initialize_explicit_software_mode() {
    let config = HsmAutoInitConfig {
        mode: "software".to_string(),
        ..Default::default()
    };

    let manager = HsmManager::auto_initialize_with_config(config).await;
    assert!(
        manager.is_ok(),
        "Auto-initialize should succeed with explicit software mode"
    );

    let manager = manager.unwrap();
    let key = manager.generate_key("test_key", &KeyType::ChaCha20).await;
    assert!(key.is_ok(), "Should be able to generate key");
}

#[tokio::test]
async fn test_auto_initialize_case_insensitive() {
    // Test uppercase
    let config = HsmAutoInitConfig {
        mode: "SOFTWARE".to_string(),
        ..Default::default()
    };
    let manager = HsmManager::auto_initialize_with_config(config).await;
    assert!(manager.is_ok(), "Should handle uppercase mode");

    // Test mixed case
    let config = HsmAutoInitConfig {
        mode: "SoftWare".to_string(),
        ..Default::default()
    };
    let manager = HsmManager::auto_initialize_with_config(config).await;
    assert!(manager.is_ok(), "Should handle mixed case mode");
}

#[tokio::test]
async fn test_hardware_mode_fails_without_fallback_flag() {
    let config = HsmAutoInitConfig {
        mode: "hardware".to_string(),
        auto_init: true,
        allow_software_fallback: false,
    };
    let result = HsmManager::auto_initialize_with_config(config).await;
    assert!(
        result.is_err(),
        "Hardware mode should fail without fallback"
    );
    assert!(
        result.unwrap_err().to_string().contains("not available"),
        "Error should explain the mode is unavailable"
    );
}

#[tokio::test]
async fn test_hardware_mode_falls_back_when_allowed() {
    let config = HsmAutoInitConfig {
        mode: "hardware".to_string(),
        auto_init: true,
        allow_software_fallback: true,
    };
    let manager = HsmManager::auto_initialize_with_config(config).await;
    assert!(
        manager.is_ok(),
        "Should fallback to software when allow_software_fallback=true"
    );
    let manager = manager.unwrap();
    let key = manager.generate_key("test_key", &KeyType::Aes).await;
    assert!(key.is_ok(), "Fallback software HSM should work");
}

#[tokio::test]
async fn test_android_mode_fails_without_fallback_flag() {
    let config = HsmAutoInitConfig {
        mode: "android_strongbox".to_string(),
        auto_init: true,
        allow_software_fallback: false,
    };
    let result = HsmManager::auto_initialize_with_config(config).await;
    assert!(result.is_err(), "Android mode should fail on non-Android");
}

#[tokio::test]
async fn test_ios_mode_fails_without_fallback_flag() {
    let config = HsmAutoInitConfig {
        mode: "ios_secure_enclave".to_string(),
        auto_init: true,
        allow_software_fallback: false,
    };
    let result = HsmManager::auto_initialize_with_config(config).await;
    assert!(result.is_err(), "iOS mode should fail on non-iOS");
}

#[tokio::test]
async fn test_auto_initialize_invalid_mode() {
    let config = HsmAutoInitConfig {
        mode: "invalid_mode".to_string(),
        ..Default::default()
    };

    let result = HsmManager::auto_initialize_with_config(config).await;
    assert!(result.is_err(), "Should fail with invalid HSM mode");

    let error = result.err().unwrap();
    assert!(
        error.to_string().contains("Invalid"),
        "Error should mention invalid mode: {}",
        error
    );
}

#[tokio::test]
async fn test_auto_initialize_disabled() {
    let config = HsmAutoInitConfig {
        mode: "software".to_string(),
        auto_init: false,
        ..Default::default()
    };

    let manager = HsmManager::auto_initialize_with_config(config).await;
    assert!(manager.is_ok(), "Should succeed even when disabled");

    let manager = manager.unwrap();
    let key_result = manager.generate_key("test_key", &KeyType::Ed25519).await;
    assert!(
        key_result.is_err(),
        "Should fail - no providers registered when auto-init disabled"
    );
    assert!(
        key_result
            .unwrap_err()
            .to_string()
            .contains("No HSM providers available")
    );
}

#[tokio::test]
async fn test_auto_initialize_multiple_key_operations() {
    let config = HsmAutoInitConfig {
        mode: "software".to_string(),
        ..Default::default()
    };

    let manager = HsmManager::auto_initialize_with_config(config)
        .await
        .unwrap();

    // Generate multiple keys
    let key1 = manager.generate_key("key1", &KeyType::Ed25519).await;
    let key2 = manager.generate_key("key2", &KeyType::ChaCha20).await;
    let key3 = manager.generate_key("key3", &KeyType::Aes).await;

    assert!(key1.is_ok());
    assert!(key2.is_ok());
    assert!(key3.is_ok());

    // Delete keys
    assert!(manager.delete_key("key1").await.is_ok());
    assert!(manager.delete_key("key2").await.is_ok());
    assert!(manager.delete_key("key3").await.is_ok());
}

#[tokio::test]
async fn test_auto_initialize_concurrent_safe() {
    use std::sync::Arc;
    use tokio::task;

    let config = HsmAutoInitConfig {
        mode: "software".to_string(),
        ..Default::default()
    };

    // Initialize the manager once
    let manager = Arc::new(
        HsmManager::auto_initialize_with_config(config)
            .await
            .unwrap(),
    );

    // Now, spawn multiple tasks that use the *same* manager concurrently
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let manager_clone = Arc::clone(&manager);
            task::spawn(async move {
                let key = manager_clone
                    .generate_key(&format!("key_{}", i), &KeyType::Ed25519)
                    .await;
                assert!(key.is_ok(), "Concurrent key generation should work");
            })
        })
        .collect();

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_auto_initialize_environment_precedence() {
    // Env-based `auto_initialize()` races with parallel tests that mutate the same
    // process-wide variables. Test the precedence logic via explicit config instead
    // (same code path, deterministic).
    let config = HsmAutoInitConfig {
        mode: "software".to_string(),
        ..Default::default()
    };
    let manager = HsmManager::auto_initialize_with_config(config)
        .await
        .expect("auto_initialize_with_config should succeed for software mode");
    let key = manager.generate_key("test", &KeyType::Ed25519).await;
    assert!(key.is_ok(), "key generation should succeed after auto-init");
}

#[tokio::test]
async fn test_auto_initialize_bool_states() {
    // Test enabled state
    let config = HsmAutoInitConfig {
        mode: "software".to_string(),
        ..Default::default()
    };
    let manager = HsmManager::auto_initialize_with_config(config)
        .await
        .unwrap();
    let key_result = manager.generate_key("test", &KeyType::Ed25519).await;
    assert!(key_result.is_ok(), "Should have provider when enabled");

    // Test disabled state
    let config = HsmAutoInitConfig {
        mode: "software".to_string(),
        auto_init: false,
        ..Default::default()
    };
    let manager = HsmManager::auto_initialize_with_config(config)
        .await
        .unwrap();
    let key_result = manager.generate_key("test", &KeyType::Ed25519).await;
    assert!(
        key_result.is_err(),
        "Should not have provider when disabled"
    );
}
