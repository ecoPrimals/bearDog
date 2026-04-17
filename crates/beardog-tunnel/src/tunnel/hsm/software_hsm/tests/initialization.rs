// SPDX-License-Identifier: AGPL-3.0-or-later
//! HSM initialization, crypto backends, and health monitoring.

#![cfg(test)]

use super::common::*;

/// Test HSM initialization with default configuration
#[tokio::test]
async fn test_hsm_initialization_default_config() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Verify HSM is created successfully
    let health = hsm.health_check().await?;
    assert!(health.is_healthy);
    Ok(())
}

/// Test HSM initialization with all crypto backends
#[tokio::test]
async fn test_hsm_initialization_all_backends() -> Result<(), BearDogError> {
    // RustCrypto backend
    let config_rust_crypto = SoftwareHsmConfig {
        crypto_backend: CryptoBackendType::RustCrypto,
        ..Default::default()
    };
    let hsm_rust = RustSoftwareHsm::new(config_rust_crypto).await?;
    let health = hsm_rust.health_check().await?;
    assert!(health.is_healthy);

    // Ring backend
    let config_ring = SoftwareHsmConfig {
        crypto_backend: CryptoBackendType::Ring,
        ..Default::default()
    };
    let hsm_ring = RustSoftwareHsm::new(config_ring).await?;
    let health = hsm_ring.health_check().await?;
    assert!(health.is_healthy);

    // OpenSSL backend
    let config_openssl = SoftwareHsmConfig {
        crypto_backend: CryptoBackendType::OpenSsl,
        ..Default::default()
    };
    let hsm_openssl = RustSoftwareHsm::new(config_openssl).await?;
    let health = hsm_openssl.health_check().await?;
    assert!(health.is_healthy);

    Ok(())
}

/// Test health check functionality
#[tokio::test]
async fn test_health_check() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    let health_status = hsm.health_check().await?;
    assert!(health_status.is_healthy);

    Ok(())
}

/// Test HSM health monitoring after various operations
#[tokio::test]
async fn test_health_monitoring_comprehensive() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Initial health check
    let health1 = hsm.health_check().await?;
    assert!(health1.is_healthy);

    // Generate keys
    for i in 0..5 {
        let request = GenerateKeyRequest {
            key_type: KeyType::Aes,
            key_id: format!("health-test-key-{i}"),
        };
        hsm.generate_key(request).await?;
    }

    // Health check after key generation
    let health2 = hsm.health_check().await?;
    assert!(health2.is_healthy);

    // Perform crypto operations
    for i in 0..5 {
        let _ = hsm
            .encrypt(&format!("health-test-key-{i}"), b"test data")
            .await?;
    }

    // Health check after crypto operations
    let health3 = hsm.health_check().await?;
    assert!(health3.is_healthy);

    // Delete keys
    for i in 0..5 {
        hsm.delete_key(&format!("health-test-key-{i}")).await?;
    }

    // Final health check
    let health4 = hsm.health_check().await?;
    assert!(health4.is_healthy);

    Ok(())
}
