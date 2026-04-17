// SPDX-License-Identifier: AGPL-3.0-or-later
//! Concurrent operations and load stress.

#![cfg(test)]

use super::common::*;

/// Test concurrent operations
#[tokio::test]
async fn test_concurrent_operations() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = Arc::new(RustSoftwareHsm::new(config).await?);

    // Generate keys concurrently
    let mut handles = vec![];
    for i in 0..5 {
        let hsm_clone = hsm.clone();
        let handle = tokio::spawn(async move {
            let request = GenerateKeyRequest {
                key_type: KeyType::Aes,
                key_id: format!("concurrent-key-{i}"),
            };
            hsm_clone.generate_key(request).await
        });
        handles.push(handle);
    }

    // Wait for all operations
    for handle in handles {
        handle
            .await
            .map_err(|e| BearDogError::system(format!("Task join failed: {}", e)))??;
    }

    // Verify all keys exist
    for i in 0..5 {
        let key_info = hsm.get_key_info(&format!("concurrent-key-{i}")).await?;
        assert_eq!(key_info.key_id, format!("concurrent-key-{i}"));
    }

    Ok(())
}

/// Test concurrent crypto operations on same key
#[tokio::test]
async fn test_concurrent_crypto_operations_same_key() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = Arc::new(RustSoftwareHsm::new(config).await?);

    // Generate key
    let request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "concurrent-ops-key".to_string(),
    };
    hsm.generate_key(request).await?;

    // Perform 20 concurrent encryptions
    let mut handles = vec![];
    for i in 0..20 {
        let hsm_clone = Arc::clone(&hsm);
        let handle = tokio::spawn(async move {
            let plaintext = format!("message {i}");
            let ciphertext = hsm_clone
                .encrypt("concurrent-ops-key", plaintext.as_bytes())
                .await?;
            let decrypted = hsm_clone.decrypt("concurrent-ops-key", &ciphertext).await?;
            assert_eq!(decrypted, plaintext.as_bytes());
            Ok::<(), BearDogError>(())
        });
        handles.push(handle);
    }

    // Wait for all operations
    for handle in handles {
        handle
            .await
            .map_err(|e| BearDogError::system(format!("Task join failed: {}", e)))??;
    }

    Ok(())
}

/// Test memory protection under high load
#[tokio::test]
async fn test_memory_protection_under_load() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = Arc::new(RustSoftwareHsm::new(config).await?);

    // Generate multiple keys rapidly
    let mut handles = vec![];
    for i in 0..50 {
        let hsm_clone = Arc::clone(&hsm);
        let handle = tokio::spawn(async move {
            let request = GenerateKeyRequest {
                key_type: KeyType::Aes,
                key_id: format!("load-test-key-{i}"),
            };
            hsm_clone.generate_key(request).await?;

            // Immediately use the key
            let plaintext = format!("data for key {i}");
            let _ciphertext = hsm_clone
                .encrypt(&format!("load-test-key-{i}"), plaintext.as_bytes())
                .await?;

            // Delete to test memory cleanup
            hsm_clone.delete_key(&format!("load-test-key-{i}")).await?;

            Ok::<(), BearDogError>(())
        });
        handles.push(handle);
    }

    // Wait for all operations
    for handle in handles {
        handle
            .await
            .map_err(|e| BearDogError::system(format!("Task join failed: {}", e)))??;
    }

    // Verify health after stress
    let health = hsm.health_check().await?;
    assert!(health.is_healthy);

    Ok(())
}
