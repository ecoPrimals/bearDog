// SPDX-License-Identifier: AGPL-3.0-or-later

//! Crypto Provider Failure Tests
//!
//! Tests error handling and fallback mechanisms for crypto providers

use crate::tunnel::hsm::manager::HsmProvider;
use crate::tunnel::hsm::software_hsm::core::RustSoftwareHsm;
use crate::tunnel::hsm::software_hsm::crypto_providers::{CryptoProvider, SoftwareHsmCryptoProvider};
// RingCryptoProvider removed - evolved to RustCrypto (100% Pure Rust!)
// OpenSslCryptoProvider removed - evolved to pure Rust only
use crate::tunnel::hsm::GenerateKeyRequest;
use crate::tunnel::hsm::types::KeyType;
use crate::tunnel::hsm::types::config::{CryptoBackendType, SoftwareHsmConfig};
use beardog_errors::BearDogError;

// TEST_CATEGORY: error_path
// TEST_DOMAIN: crypto_providers
// TEST_PRIORITY: high

#[tokio::test]
async fn test_rustcrypto_provider_initialization() -> Result<(), BearDogError> {
    let provider = SoftwareHsmCryptoProvider::new().await?;
    let result = provider.initialize().await;
    assert!(result.is_ok());
    Ok(())
}

// Ring test removed - evolved to RustCrypto (100% Pure Rust!)

// OpenSSL tests removed - pure Rust only

#[tokio::test]
async fn test_hsm_with_rustcrypto_backend() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig {
        crypto_backend: CryptoBackendType::RustCrypto,
        ..Default::default()
    };

    let hsm = RustSoftwareHsm::new(config).await?;
    let health = hsm.health_check().await?;
    assert!(health.is_healthy);
    Ok(())
}

#[tokio::test]
async fn test_hsm_with_ring_backend() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig {
        crypto_backend: CryptoBackendType::Ring,
        ..Default::default()
    };

    let hsm = RustSoftwareHsm::new(config).await?;
    let health = hsm.health_check().await?;
    assert!(health.is_healthy);
    Ok(())
}

#[tokio::test]
async fn test_hsm_with_openssl_backend() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig {
        crypto_backend: CryptoBackendType::OpenSsl,
        ..Default::default()
    };

    let hsm = RustSoftwareHsm::new(config).await?;
    let health = hsm.health_check().await?;
    assert!(health.is_healthy);
    Ok(())
}

#[tokio::test]
async fn test_key_generation_across_backends() -> Result<(), BearDogError> {
    let backends = [
        CryptoBackendType::RustCrypto,
        CryptoBackendType::Ring,
        CryptoBackendType::OpenSsl,
    ];

    for (idx, backend) in backends.iter().enumerate() {
        let config = SoftwareHsmConfig {
            crypto_backend: backend.clone(),
            ..Default::default()
        };

        let hsm = RustSoftwareHsm::new(config).await?;

        let request = GenerateKeyRequest {
            key_id: format!("test-key-{}", idx),
            key_type: KeyType::Ed25519,
        };

        let result = hsm.generate_key(request).await;
        assert!(result.is_ok(), "Backend {:?} should generate keys", backend);
    }

    Ok(())
}

#[tokio::test]
async fn test_sign_verify_across_backends() -> Result<(), BearDogError> {
    let backends = [
        CryptoBackendType::RustCrypto,
        CryptoBackendType::Ring,
        CryptoBackendType::OpenSsl,
    ];

    for (idx, backend) in backends.iter().enumerate() {
        let config = SoftwareHsmConfig {
            crypto_backend: backend.clone(),
            ..Default::default()
        };

        let hsm = RustSoftwareHsm::new(config).await?;

        // Generate key
        let request = GenerateKeyRequest {
            key_id: format!("sign-key-{}", idx),
            key_type: KeyType::Ed25519,
        };
        hsm.generate_key(request).await?;

        // Sign and verify
        let message = b"test message";
        let signature = hsm.sign(&format!("sign-key-{}", idx), message).await?;
        let valid = hsm
            .verify(&format!("sign-key-{}", idx), message, &signature)
            .await?;

        assert!(valid, "Backend {:?} sign/verify should work", backend);
    }

    Ok(())
}

#[tokio::test]
async fn test_encrypt_decrypt_across_backends() -> Result<(), BearDogError> {
    let backends = [
        CryptoBackendType::RustCrypto,
        CryptoBackendType::Ring,
        CryptoBackendType::OpenSsl,
    ];

    for (idx, backend) in backends.iter().enumerate() {
        let config = SoftwareHsmConfig {
            crypto_backend: backend.clone(),
            ..Default::default()
        };

        let hsm = RustSoftwareHsm::new(config).await?;

        // Generate key
        let request = GenerateKeyRequest {
            key_id: format!("encrypt-key-{}", idx),
            key_type: KeyType::Aes,
        };
        hsm.generate_key(request).await?;

        // Encrypt and decrypt
        let plaintext = b"sensitive data";
        let ciphertext = hsm
            .encrypt(&format!("encrypt-key-{}", idx), plaintext)
            .await?;
        let decrypted = hsm
            .decrypt(&format!("encrypt-key-{}", idx), &ciphertext)
            .await?;

        assert_eq!(
            plaintext.as_ref(),
            decrypted.as_slice(),
            "Backend {:?} encrypt/decrypt should work",
            backend
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_concurrent_operations_different_backends() {
    use tokio::task::JoinSet;

    let backends = [
        CryptoBackendType::RustCrypto,
        CryptoBackendType::Ring,
        CryptoBackendType::OpenSsl,
    ];

    let mut set = JoinSet::new();

    for (idx, backend) in backends.into_iter().enumerate() {
        set.spawn(async move {
            let config = SoftwareHsmConfig {
                crypto_backend: backend,
                ..Default::default()
            };

            let hsm = RustSoftwareHsm::new(config).await?;

            let request = GenerateKeyRequest {
                key_id: format!("concurrent-key-{}", idx),
                key_type: KeyType::Ed25519,
            };

            hsm.generate_key(request).await
        });
    }

    let mut success_count = 0;
    while let Some(result) = set.join_next().await {
        if let Ok(Ok(_)) = result {
            success_count += 1;
        }
    }

    assert_eq!(success_count, 3, "All backends should work concurrently");
}

#[tokio::test]
async fn test_backend_switching_new_hsm() -> Result<(), BearDogError> {
    // Create HSM with RustCrypto
    let config1 = SoftwareHsmConfig {
        crypto_backend: CryptoBackendType::RustCrypto,
        ..Default::default()
    };
    let hsm1 = RustSoftwareHsm::new(config1).await?;

    // Create HSM with Ring
    let config2 = SoftwareHsmConfig {
        crypto_backend: CryptoBackendType::Ring,
        ..Default::default()
    };
    let hsm2 = RustSoftwareHsm::new(config2).await?;

    // Both should work independently
    let health1 = hsm1.health_check().await?;
    let health2 = hsm2.health_check().await?;

    assert!(health1.is_healthy);
    assert!(health2.is_healthy);
    Ok(())
}

#[tokio::test]
async fn test_provider_initialize_multiple_times() -> Result<(), BearDogError> {
    let provider = SoftwareHsmCryptoProvider::new().await?;

    // Initialize multiple times
    provider.initialize().await?;
    provider.initialize().await?;
    provider.initialize().await?;

    // Should handle gracefully
    Ok(())
}

#[tokio::test]
async fn test_all_providers_support_ed25519() -> Result<(), BearDogError> {
    let backends = [
        CryptoBackendType::RustCrypto,
        CryptoBackendType::Ring,
        CryptoBackendType::OpenSsl,
    ];

    for backend in backends {
        let config = SoftwareHsmConfig {
            crypto_backend: backend,
            ..Default::default()
        };

        let hsm = RustSoftwareHsm::new(config).await?;

        let request = GenerateKeyRequest {
            key_id: "ed25519-test".to_string(),
            key_type: KeyType::Ed25519,
        };

        let result = hsm.generate_key(request).await;
        assert!(result.is_ok(), "All backends should support Ed25519");
    }

    Ok(())
}

#[tokio::test]
async fn test_all_providers_support_aes() -> Result<(), BearDogError> {
    let backends = [
        CryptoBackendType::RustCrypto,
        CryptoBackendType::Ring,
        CryptoBackendType::OpenSsl,
    ];

    for backend in backends {
        let config = SoftwareHsmConfig {
            crypto_backend: backend,
            ..Default::default()
        };

        let hsm = RustSoftwareHsm::new(config).await?;

        let request = GenerateKeyRequest {
            key_id: "aes-test".to_string(),
            key_type: KeyType::Aes,
        };

        let result = hsm.generate_key(request).await;
        assert!(result.is_ok(), "All backends should support AES");
    }

    Ok(())
}
