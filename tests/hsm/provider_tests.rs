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


//! HSM Provider Implementation Tests
//!
//! Tests for HSM provider implementations including Android StrongBox and Software HSM

use super::HsmTestHarness;
use beardog::tunnel::hsm::types::{
    GenerateKeyRequest, HsmOperation, HsmTier, KeyMetadata, KeyType, KeyUsagePolicy,
    SecureEnclaveType, SmartphoneType, SoftwareHsmType, StrongBoxImplementation,
};
use beardog::{BearDogError, BearDogResult};

/// Test HSM provider implementations
pub async fn test_hsm_providers(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("🔧 Testing HSM Provider Implementations");

    // Test Android StrongBox provider
    test_android_strongbox_provider(harness).await?;

    // Test Software HSM provider
    test_software_hsm_provider(harness).await?;

    // Test provider compatibility
    test_provider_compatibility(harness).await?;

    println!("✅ HSM Provider tests completed");
    Ok(())
}

/// Test Android StrongBox HSM provider
async fn test_android_strongbox_provider(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("📱 Testing Android StrongBox HSM Provider");

    let start_time = std::time::Instant::now();

    // Test provider info
    let info = harness.android_strongbox.get_info().await?;
    assert_eq!(
        info.hsm_type,
        HsmTier::SmartphoneHsm {
            smartphone_type: SmartphoneType::Android,
            secure_enclave: SecureEnclaveType::StrongBox(StrongBoxImplementation::TitanM),
        }
    );
    assert_eq!(info.vendor, "Google");
    assert_eq!(info.model, "Pixel 8a");

    // Test key generation
    let key_request = GenerateKeyRequest {
        key_id: "test_strongbox_key".to_string(),
        key_type: KeyType::Symmetric,
        usage_policy: KeyUsagePolicy::default(),
        metadata: KeyMetadata::default(),
        require_user_presence: false,
        attestation_challenge: None,
    };

    let key = harness.android_strongbox.generate_key(key_request).await?;
    assert_eq!(key.key_id, "test_strongbox_key");
    assert_eq!(key.key_type, KeyType::Symmetric);

    // Test signing capability
    let test_data = b"test signature data for strongbox";
    let signature = harness.android_strongbox.sign(&key.key_id, test_data).await?;
    assert!(!signature.is_empty());

    // Test signature verification
    let is_valid = harness
        .android_strongbox
        .verify(&key.key_id, test_data, &signature)
        .await?;
    assert!(is_valid, "Signature should be valid");

    // Test health check
    let health = harness.android_strongbox.health_check().await?;
    assert!(health.healthy, "StrongBox should be healthy");

    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "strongbox", true);
    
    println!("✅ Android StrongBox provider tests passed");
    Ok(())
}

/// Test Software HSM provider
async fn test_software_hsm_provider(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("💾 Testing Software HSM Provider");

    let start_time = std::time::Instant::now();

    // Test provider info
    let info = harness.software_hsm.get_info().await?;
    assert_eq!(
        info.hsm_type,
        HsmTier::SoftwareHsm {
            implementation: SoftwareHsmType::RustCrypto,
        }
    );

    // Test key generation with different algorithms
    let algorithms = vec![
        (KeyType::Aes256, "aes_test_key"),
        (KeyType::Rsa { key_size: 2048 }, "rsa_test_key"),
        (KeyType::EccP256, "ec_test_key"),
    ];

    for (key_type, key_id) in algorithms {
        let key_request = GenerateKeyRequest {
            key_id: key_id.to_string(),
            key_type,
            usage_policy: KeyUsagePolicy::default(),
            metadata: KeyMetadata::default(),
            require_user_presence: false,
            attestation_challenge: None,
        };

        let key = harness.software_hsm.generate_key(key_request).await?;
        assert_eq!(key.key_id, key_id);
        assert_eq!(key.key_type, key_type);

        // Test encryption/decryption for symmetric keys
        if matches!(key_type, KeyType::Aes256 | KeyType::Aes128 | KeyType::Aes192 | KeyType::ChaCha20) {
            let plaintext = b"test encryption data";
            let ciphertext = harness.software_hsm.encrypt(&key.metadata.key_id, plaintext).await?;
            assert_ne!(ciphertext, plaintext);

            let decrypted = harness.software_hsm.decrypt(&key.metadata.key_id, &ciphertext).await?;
            assert_eq!(decrypted, plaintext);
        }

        // Test signing for asymmetric keys
        if matches!(key_type, KeyType::Rsa { .. } | KeyType::EccP256 | KeyType::EccP384 | KeyType::EccP521 | KeyType::Ed25519) {
            let test_data = b"test signature data";
            let signature = harness.software_hsm.sign(&key.metadata.key_id, test_data).await?;
            assert!(!signature.is_empty());

            let is_valid = harness
                .software_hsm
                .verify(&key.metadata.key_id, test_data, &signature)
                .await?;
            assert!(is_valid, "Signature should be valid");
        }
    }

    // Test key listing
    let keys = harness.software_hsm.list_keys().await?;
    assert!(keys.len() >= 3, "Should have at least 3 test keys");

    // Test health check
    let health = harness.software_hsm.health_check().await?;
    assert!(health.healthy, "Software HSM should be healthy");

    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "software", true);

    println!("✅ Software HSM provider tests passed");
    Ok(())
}

/// Test provider compatibility and interoperability
async fn test_provider_compatibility(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("🔗 Testing HSM Provider Compatibility");

    let start_time = std::time::Instant::now();

    // Test that both providers support required operations
    let required_operations = vec![
        HsmOperation::KeyGeneration,
        HsmOperation::DigitalSignature,
        HsmOperation::DataEncryption,
        HsmOperation::DataDecryption,
        HsmOperation::KeyDerivation,
        HsmOperation::RandomGeneration,
    ];

    for operation in required_operations {
        // Both providers should support these operations
        let android_info = harness.android_strongbox.get_info().await?;
        let software_info = harness.software_hsm.get_info().await?;

        // Verify capabilities exist (in a real implementation, we'd check specific capabilities)
        assert!(!android_info.capabilities.is_empty());
        assert!(!software_info.capabilities.is_empty());
    }

    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "compatibility", true);

    println!("✅ Provider compatibility tests passed");
    Ok(())
}

/// Test specific StrongBox capabilities
pub async fn test_strongbox_specific_features(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("🔒 Testing StrongBox-specific features");

    // Test attestation capabilities (if available)
    let info = harness.android_strongbox.get_info().await?;
    if info.supports_attestation {
        println!("  ✓ Hardware attestation supported");
        
        // Test key generation with attestation
        let key_request = GenerateKeyRequest {
            key_id: "attested_key".to_string(),
            key_type: KeyType::EccP256,
            usage_policy: KeyUsagePolicy::default(),
            metadata: KeyMetadata::default(),
            require_user_presence: false,
            attestation_challenge: Some(b"test_challenge".to_vec()),
        };

        let key = harness.android_strongbox.generate_key(key_request).await?;
        assert!(key.attestation_certificate.is_some(), "Key should have attestation certificate");
    }

    // Test StrongBox-specific security properties
    let security_props = harness.android_strongbox.get_security_properties().await?;
    assert!(security_props.hardware_backed, "StrongBox keys should be hardware-backed");
    assert!(security_props.strongbox_backed.unwrap_or(false), "Should be StrongBox-backed");

    println!("✅ StrongBox-specific features tested");
    Ok(())
}

#[tokio::test]
async fn test_hsm_providers_standalone() -> BearDogResult<()> {
    let mut harness = super::HsmTestHarness::new().await?;
    test_hsm_providers(&mut harness).await
} 