//! HSM Manager Tests
//!
//! Comprehensive test suite for HSM Manager functionality including:
//! - Provider registration and selection
//! - Key lifecycle (generate, delete)
//! - Auto-initialization
//! - Concurrent safety
//! - Failover and error handling

use super::*;
use crate::tunnel::hsm::{GenerateKeyRequest, KeyType};
use async_trait::async_trait;

// Mock HSM Provider for testing
struct MockHsmProvider {
    available: bool,
    fail_generate: bool,
    fail_delete: bool,
}

impl MockHsmProvider {
    fn new() -> Self {
        Self {
            available: true,
            fail_generate: false,
            fail_delete: false,
        }
    }

    fn unavailable() -> Self {
        Self {
            available: false,
            fail_generate: false,
            fail_delete: false,
        }
    }

    fn failing_generate() -> Self {
        Self {
            available: true,
            fail_generate: true,
            fail_delete: false,
        }
    }

    fn failing_delete() -> Self {
        Self {
            available: true,
            fail_generate: false,
            fail_delete: true,
        }
    }
}

#[async_trait]
impl HsmProvider for MockHsmProvider {
    async fn get_info(&self) -> Result<ProviderInfo, BearDogError> {
        Ok(ProviderInfo {
            id: "mock".to_string(),
            name: "Mock HSM".to_string(),
            security_level: 3,
        })
    }

    async fn generate_key(&self, request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
        if self.fail_generate {
            return Err(BearDogError::system(
                "Mock generate_key failure".to_string(),
            ));
        }

        use crate::tunnel::hsm::{KeyHealthStatus, KeyMaterial, KeyMetadata};
        use chrono::Utc;

        Ok(HsmKey {
            id: request.key_id.clone(),
            hsm_type: "MockHSM".to_string(),
            key_type: request.key_type.clone(),
            metadata: KeyMetadata::new(request.key_id, request.key_type),
            key_material: KeyMaterial::Encrypted {
                encrypted_data: vec![1, 2, 3, 4],
                encryption_algorithm: "AES-256-GCM".to_string(),
                kdf_params: None,
            },
            hsm_tier: "Software".to_string(),
            health_status: KeyHealthStatus::Healthy,
            attestation: None,
            created_at: Utc::now(),
        })
    }

    async fn sign(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(vec![])
    }

    async fn verify(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Ok(true)
    }

    async fn encrypt(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(vec![])
    }

    async fn decrypt(
        &self,
        _key_id: &str,
        _ciphertext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        Ok(vec![])
    }

    async fn import_key(
        &self,
        _key_data: &[u8],
        _key_id: &str,
    ) -> Result<HsmKey, BearDogError> {
        Err(BearDogError::not_implemented("Mock import_key"))
    }

    async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
        if self.fail_delete {
            return Err(BearDogError::system("Mock delete_key failure".to_string()));
        }
        Ok(())
    }

    async fn get_key_info(&self, _key_id: &str) -> Result<KeyInfo, BearDogError> {
        Ok(KeyInfo {
            key_id: "test".to_string(),
            key_type: "AES".to_string(),
            is_hardware_backed: false,
        })
    }

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        Ok(HealthStatus {
            is_healthy: true,
            error_message: None,
        })
    }

    fn is_available(&self) -> bool {
        self.available
    }
}

#[tokio::test]
async fn test_generate_key_success() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(HsmTier::Software, Arc::new(MockHsmProvider::new()))
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
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No HSM providers available"));
}

#[tokio::test]
async fn test_generate_key_provider_unavailable() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(HsmTier::Software, Arc::new(MockHsmProvider::unavailable()))
        .unwrap();

    let result = manager.generate_key("test_key", &KeyType::Aes).await;

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No HSM providers available"));
}

#[tokio::test]
async fn test_generate_key_provider_failure() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(
            HsmTier::Software,
            Arc::new(MockHsmProvider::failing_generate()),
        )
        .unwrap();

    let result = manager.generate_key("test_key", &KeyType::Aes).await;

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Mock generate_key failure"));
}

#[tokio::test]
async fn test_delete_key_success() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(HsmTier::Software, Arc::new(MockHsmProvider::new()))
        .unwrap();

    let result = manager.delete_key("test_key_123").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_key_no_providers() {
    let manager = HsmManager::new();

    let result = manager.delete_key("test_key").await;

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No HSM providers available"));
}

#[tokio::test]
async fn test_delete_key_provider_unavailable() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(HsmTier::Software, Arc::new(MockHsmProvider::unavailable()))
        .unwrap();

    let result = manager.delete_key("test_key").await;

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No HSM providers available"));
}

#[tokio::test]
async fn test_delete_key_provider_failure() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(
            HsmTier::Software,
            Arc::new(MockHsmProvider::failing_delete()),
        )
        .unwrap();

    let result = manager.delete_key("test_key").await;

    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Mock delete_key failure"));
}

#[tokio::test]
async fn test_generate_and_delete_key_lifecycle() {
    let mut manager = HsmManager::new();
    manager
        .register_hsm_provider(HsmTier::Software, Arc::new(MockHsmProvider::new()))
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
        .register_hsm_provider(HsmTier::Software, Arc::new(MockHsmProvider::new()))
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
        use std::env;
        for key in &self.keys {
            env::remove_var(key);
        }
    }
}

#[tokio::test]
async fn test_auto_initialize_default_software_mode() {
    use std::env;
    let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);

    // Clear any existing env vars
    env::remove_var("BEARDOG_HSM_MODE");
    env::remove_var("BEARDOG_HSM_AUTO_INIT");

    // Should default to software mode
    let manager = HsmManager::auto_initialize().await;
    assert!(
        manager.is_ok(),
        "Auto-initialize should succeed with default software mode"
    );

    let manager = manager.unwrap();
    // Verify we can generate a key (proves HSM is registered)
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
        auto_init: true,
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
        auto_init: true,
    };
    let manager = HsmManager::auto_initialize_with_config(config).await;
    assert!(manager.is_ok(), "Should handle uppercase mode");

    // Test mixed case
    let config = HsmAutoInitConfig {
        mode: "SoftWare".to_string(),
        auto_init: true,
    };
    let manager = HsmManager::auto_initialize_with_config(config).await;
    assert!(manager.is_ok(), "Should handle mixed case mode");
}

#[tokio::test]
async fn test_auto_initialize_hardware_mode_fallback() {
    use std::env;
    let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);

    env::set_var("BEARDOG_HSM_MODE", "hardware");

    // Should fallback to software (hardware not yet implemented)
    let manager = HsmManager::auto_initialize().await;
    assert!(
        manager.is_ok(),
        "Should fallback to software when hardware not available"
    );

    let manager = manager.unwrap();
    // Verify it works
    let key = manager.generate_key("test_key", &KeyType::Aes).await;
    assert!(key.is_ok(), "Fallback software HSM should work");
}

#[tokio::test]
async fn test_auto_initialize_android_mode_fallback() {
    use std::env;
    let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);

    env::set_var("BEARDOG_HSM_MODE", "android_strongbox");

    // Should fallback to software (android not yet implemented)
    let manager = HsmManager::auto_initialize().await;
    assert!(
        manager.is_ok(),
        "Should fallback to software when Android not available"
    );
}

#[tokio::test]
async fn test_auto_initialize_ios_mode_fallback() {
    use std::env;
    let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);

    env::set_var("BEARDOG_HSM_MODE", "ios_secure_enclave");

    // Should fallback to software (iOS not yet implemented)
    let manager = HsmManager::auto_initialize().await;
    assert!(
        manager.is_ok(),
        "Should fallback to software when iOS not available"
    );
}

#[tokio::test]
async fn test_auto_initialize_invalid_mode() {
    let config = HsmAutoInitConfig {
        mode: "invalid_mode".to_string(),
        auto_init: true,
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
    use std::env;
    let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);

    // Clear env vars first
    env::remove_var("BEARDOG_HSM_MODE");
    env::set_var("BEARDOG_HSM_AUTO_INIT", "false");

    let manager = HsmManager::auto_initialize().await;
    assert!(manager.is_ok(), "Should succeed even when disabled");

    let manager = manager.unwrap();
    // Should have NO providers registered
    let key_result = manager.generate_key("test_key", &KeyType::Ed25519).await;
    assert!(
        key_result.is_err(),
        "Should fail - no providers registered when auto-init disabled"
    );
    assert!(key_result
        .unwrap_err()
        .to_string()
        .contains("No HSM providers available"));
}

#[tokio::test]
async fn test_auto_initialize_multiple_key_operations() {
    use std::env;
    let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);

    env::set_var("BEARDOG_HSM_MODE", "software");

    let manager = HsmManager::auto_initialize().await.unwrap();

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
        auto_init: true,
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
    use std::env;
    let _cleanup = EnvCleanup::new(&["BEARDOG_HSM_MODE", "BEARDOG_HSM_AUTO_INIT"]);

    // Test that environment variable takes precedence over default
    env::set_var("BEARDOG_HSM_MODE", "software");
    env::set_var("BEARDOG_HSM_AUTO_INIT", "true");

    let manager = HsmManager::auto_initialize().await.unwrap();
    let key = manager.generate_key("test", &KeyType::Ed25519).await;
    assert!(key.is_ok());
}

#[tokio::test]
async fn test_auto_initialize_bool_states() {
    // Test enabled state
    let config = HsmAutoInitConfig {
        mode: "software".to_string(),
        auto_init: true,
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
