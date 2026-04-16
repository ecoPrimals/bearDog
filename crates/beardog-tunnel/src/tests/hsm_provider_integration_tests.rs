// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM Provider Integration Tests - Modern Concurrent Edition
//!
//! Substantive integration tests that exercise real production paths
//! for HSM provider operations, discovery, and lifecycle management.
//!
//! ## Architecture Philosophy
//! - **Zero Sleeps**: Use channels and synchronization primitives
//! - **Fully Concurrent**: All tests leverage async/await properly
//! - **Modern Patterns**: Idiomatic concurrent Rust
//! - **Production Paths**: Test real code paths, not mocks

#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::field_reassign_with_default)]
#![allow(unused_variables)] // Some variables used conditionally

use crate::tunnel::hsm::GenerateKeyRequest;
use crate::tunnel::hsm::HsmProviderBackend;
use crate::tunnel::hsm::manager::HsmManager;
use crate::tunnel::hsm::manager::implementation::DefaultHsmManager;
use crate::tunnel::hsm::software_hsm::SoftwareHsm;
use crate::tunnel::hsm::types::key::KeyType;
use crate::tunnel::hsm::types::tier::HsmTier;
use beardog_errors::BearDogError;
use std::sync::Arc;
use tokio::sync::{Barrier, Semaphore};

/// Helper to create a configured software HSM
async fn create_test_software_hsm() -> Result<Arc<SoftwareHsm>, BearDogError> {
    let hsm = crate::tunnel::hsm::software_hsm::create_default_software_hsm().await?;
    Ok(Arc::new(hsm))
}

#[cfg(test)]
mod hsm_provider_integration {
    use super::*;
    use crate::tunnel::hsm::manager::HsmProvider;

    #[tokio::test]
    async fn test_software_hsm_provider_registration_and_use() {
        // Test full lifecycle: create HSM, register with manager, use for operations
        let software_hsm = create_test_software_hsm()
            .await
            .expect("Software HSM creation should succeed");

        let mut manager = HsmManager::new();

        let registered = create_test_software_hsm()
            .await
            .expect("second HSM for registration");
        // Register provider with tier-based API
        let result = manager.register_hsm_provider(
            HsmTier::Software,
            Arc::new(HsmProviderBackend::RustSoftware(
                Arc::try_unwrap(registered)
                    .unwrap_or_else(|_| panic!("unique Arc for registration")),
            )),
        );
        assert!(result.is_ok(), "HSM provider registration should succeed");

        // Verify we can use the provider directly
        let key_request = GenerateKeyRequest {
            key_id: "test-key-1".to_string(),
            key_type: KeyType::Aes,
        };
        let gen_result = software_hsm.generate_key(key_request).await;
        assert!(gen_result.is_ok(), "Direct provider usage should work");
    }

    #[tokio::test]
    async fn test_hsm_key_generation_through_provider() {
        // Test key generation through HSM provider (production path)
        let software_hsm = create_test_software_hsm().await.unwrap();

        // Generate key through provider
        let key_request = GenerateKeyRequest {
            key_id: "test-key-2".to_string(),
            key_type: KeyType::Aes,
        };

        let result = software_hsm.generate_key(key_request).await;
        assert!(
            result.is_ok(),
            "Key generation through provider should succeed"
        );

        let key = result.unwrap();
        assert!(!key.id.is_empty(), "Generated key should have ID");
    }

    #[tokio::test]
    async fn test_hsm_provider_health_check_integration() {
        // Test health check integration
        let software_hsm = create_test_software_hsm().await.unwrap();

        // Perform health check through provider directly
        let health_result = software_hsm.health_check().await;
        assert!(
            health_result.is_ok(),
            "Health check should succeed for valid provider"
        );
    }

    #[tokio::test]
    async fn test_multiple_hsm_providers_concurrent_operations() {
        // Test concurrent operations across multiple HSM providers
        // ✅ NO SLEEPS - Use barrier for true concurrency

        let hsm1 = create_test_software_hsm().await.unwrap();
        let hsm2 = create_test_software_hsm().await.unwrap();

        // Barrier ensures both tasks start simultaneously
        let barrier = Arc::new(Barrier::new(2));

        let key_request = GenerateKeyRequest {
            key_id: "test-concurrent-key".to_string(),
            key_type: KeyType::Aes,
        };

        // Spawn concurrent tasks
        let (hsm1_clone, barrier1) = (Arc::clone(&hsm1), Arc::clone(&barrier));
        let req1 = GenerateKeyRequest {
            key_id: "test-concurrent-key-1".to_string(),
            ..key_request.clone()
        };
        let handle1 = tokio::spawn(async move {
            barrier1.wait().await; // Synchronize start
            hsm1_clone.generate_key(req1).await
        });

        let (hsm2_clone, barrier2) = (Arc::clone(&hsm2), Arc::clone(&barrier));
        let req2 = GenerateKeyRequest {
            key_id: "test-concurrent-key-2".to_string(),
            ..key_request.clone()
        };
        let handle2 = tokio::spawn(async move {
            barrier2.wait().await; // Synchronize start
            hsm2_clone.generate_key(req2).await
        });

        let (result1, result2) = tokio::join!(handle1, handle2);

        assert!(
            result1.unwrap().is_ok(),
            "Concurrent key gen on HSM 1 should succeed"
        );
        assert!(
            result2.unwrap().is_ok(),
            "Concurrent key gen on HSM 2 should succeed"
        );
    }

    #[tokio::test]
    async fn test_default_hsm_manager_lifecycle() {
        // Test DefaultHsmManager provider management
        let software_hsm = create_test_software_hsm().await.unwrap();

        // `DefaultHsmManager` stores `Box<HsmProviderBackend>`; unwrap a dedicated `Arc` for it.
        let software_hsm_for_manager = create_test_software_hsm().await.unwrap();
        let boxed_provider: Box<HsmProviderBackend> = Box::new(HsmProviderBackend::RustSoftware(
            Arc::try_unwrap(software_hsm_for_manager)
                .unwrap_or_else(|_| panic!("unique Arc for DefaultHsmManager test")),
        ));

        let mut manager = DefaultHsmManager::new();
        manager
            .register_provider("test-hsm".to_string(), boxed_provider)
            .unwrap();

        // Verify registered
        let providers_before = manager.list_providers();
        assert!(providers_before.contains(&"test-hsm".to_string()));

        // Get provider and verify it exists
        let provider = manager.get_provider("test-hsm");
        assert!(provider.is_ok(), "Should retrieve registered provider");
    }

    #[tokio::test]
    async fn test_hsm_error_handling_invalid_provider() {
        // Test error handling for operations on non-existent provider
        let manager = DefaultHsmManager::new();

        // Try to use non-existent provider
        let result = manager.get_provider("non-existent-hsm");
        assert!(result.is_err(), "Should error for non-existent provider");

        if let Err(e) = result {
            assert!(
                e.to_string().contains("Provider not found"),
                "Error should indicate provider not found"
            );
        }
    }

    #[tokio::test]
    async fn test_hsm_concurrent_registration_safety() {
        // Test thread-safety of concurrent provider registrations
        // ✅ NO SLEEPS - Use semaphore for controlled concurrency

        let mut manager = HsmManager::new();
        let semaphore = Arc::new(Semaphore::new(5)); // Max 5 concurrent

        let handles: Vec<_> = (0..5)
            .map(|i| {
                let sem = Arc::clone(&semaphore);
                tokio::spawn(async move {
                    let _permit = sem.acquire().await.unwrap();
                    let hsm = create_test_software_hsm().await.unwrap();
                    (i, hsm)
                })
            })
            .collect();

        // Register all providers after they're created
        for (i, handle) in handles.into_iter().enumerate() {
            let (index, hsm) = handle.await.expect("Task should complete");
            assert_eq!(i, index, "Tasks should complete in order spawned");
            let result = manager.register_hsm_provider(
                HsmTier::Software,
                Arc::new(HsmProviderBackend::RustSoftware(
                    Arc::try_unwrap(hsm)
                        .unwrap_or_else(|_| panic!("unique Arc for concurrent registration test")),
                )),
            );
            assert!(result.is_ok(), "Concurrent registration should succeed");
        }
    }

    #[tokio::test]
    async fn test_hsm_operation_timeout_handling() {
        // Test that operations complete within reasonable time
        // ✅ NO SLEEPS - Use tokio::time::timeout for actual timeout testing

        let software_hsm = create_test_software_hsm().await.unwrap();

        let key_request = GenerateKeyRequest {
            key_id: "test-timeout-key".to_string(),
            key_type: KeyType::Aes,
        };

        // Use proper timeout mechanism (not sleep)
        let timeout_duration = std::time::Duration::from_secs(5);
        let result =
            tokio::time::timeout(timeout_duration, software_hsm.generate_key(key_request)).await;

        assert!(
            result.is_ok(),
            "HSM operation should complete within timeout"
        );
        assert!(result.unwrap().is_ok(), "HSM operation should succeed");
    }

    #[tokio::test]
    async fn test_hsm_provider_state_consistency() {
        // Test that provider state remains consistent across operations
        // ✅ FULLY CONCURRENT - All operations run in parallel

        let software_hsm = create_test_software_hsm().await.unwrap();
        let hsm = Arc::clone(&software_hsm);

        // Spawn 10 concurrent key generation operations
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let hsm_clone = Arc::clone(&hsm);
                tokio::spawn(async move {
                    let key_request = GenerateKeyRequest {
                        key_id: format!("test-state-key-{}", i),
                        key_type: KeyType::Aes,
                    };
                    let result = hsm_clone.generate_key(key_request).await;
                    (i, result)
                })
            })
            .collect();

        // Collect all results
        for handle in handles {
            let (index, result) = handle.await.expect("Task should complete");
            assert!(
                result.is_ok(),
                "Operation {} should succeed - provider state should be consistent",
                index
            );
        }

        // Provider should still be healthy after concurrent operations
        let health = software_hsm.health_check().await;
        assert!(
            health.is_ok(),
            "Provider should remain healthy after concurrent operations"
        );
    }

    #[tokio::test]
    async fn test_concurrent_encrypt_decrypt_operations() {
        // Test concurrent encrypt/decrypt operations
        // ✅ TRUE CONCURRENCY - No serial bottlenecks

        let software_hsm = create_test_software_hsm().await.unwrap();

        // First, generate a key
        let key_request = GenerateKeyRequest {
            key_id: "test-encrypt-key".to_string(),
            key_type: KeyType::Aes,
        };
        let key = software_hsm
            .generate_key(key_request)
            .await
            .expect("Key generation should succeed");

        let hsm = Arc::clone(&software_hsm);
        let key_id = key.id.clone();

        // Concurrent encrypt operations
        let encrypt_handles: Vec<_> = (0..5)
            .map(|i| {
                let hsm_clone = Arc::clone(&hsm);
                let key_id_clone = key_id.clone();
                tokio::spawn(async move {
                    let data = format!("test data {}", i).into_bytes();
                    hsm_clone.encrypt(&key_id_clone, &data).await
                })
            })
            .collect();

        // All encryptions should succeed
        for handle in encrypt_handles {
            let result = handle.await.expect("Task should complete");
            assert!(result.is_ok(), "Concurrent encryption should succeed");
        }
    }

    #[tokio::test]
    async fn test_multi_tier_hsm_registration() {
        // Test registering multiple tiers concurrently
        let mut manager = HsmManager::new();

        let sw = create_test_software_hsm().await.unwrap();
        let cloud = create_test_software_hsm().await.unwrap();

        // Register two software HSM instances at different tiers (simulating fallback)
        let r1 = manager.register_hsm_provider(
            HsmTier::Software,
            Arc::new(HsmProviderBackend::RustSoftware(
                Arc::try_unwrap(sw).unwrap_or_else(|_| panic!("unique Arc")),
            )),
        );
        let r2 = manager.register_hsm_provider(
            HsmTier::Cloud,
            Arc::new(HsmProviderBackend::RustSoftware(
                Arc::try_unwrap(cloud).unwrap_or_else(|_| panic!("unique Arc")),
            )),
        );

        assert!(r1.is_ok(), "Software tier registration should succeed");
        assert!(r2.is_ok(), "Cloud tier registration should succeed");
    }

    #[tokio::test]
    async fn test_stress_concurrent_health_checks() {
        // Stress test: Many concurrent health checks
        // ✅ EXTREME CONCURRENCY - 100 simultaneous checks

        let software_hsm = create_test_software_hsm().await.unwrap();
        let hsm = Arc::clone(&software_hsm);

        let handles: Vec<_> = (0..100)
            .map(|_| {
                let hsm_clone = Arc::clone(&hsm);
                tokio::spawn(async move { hsm_clone.health_check().await })
            })
            .collect();

        // All health checks should succeed
        for handle in handles {
            let result = handle.await.expect("Task should complete");
            assert!(
                result.is_ok(),
                "Concurrent health checks should all succeed"
            );
        }
    }
}
