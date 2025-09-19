use beardog_errors::BearDogError;

use super::HsmTestHarness;
use beardog::tunnel::hsm::types::{
    GenerateKeyRequest, HsmOperation, HsmTier, KeyMetadata, KeyType, KeyUsagePolicy,
    SecureEnclaveType, SmartphoneType, SoftwareHsmType, StrongBoxImplementation,
};
use beardog::BearDogError;

pub async fn test_hsm_providers(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("🔧 Testing HSM Provider Implementations");

    test_android_strongbox_provider(harness)?;

    test_software_hsm_provider(harness)?;

    test_provider_compatibility(harness)?;

    println!("✅ HSM Provider tests completed");
    Ok(())
}

async fn test_android_strongbox_provider(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("📱 Testing Android StrongBox HSM Provider");

    let start_time = std::time::Instant::now();

    let info = harness.android_strongbox.get_info()?;
    assert_eq!(
        info.hsm_type,
        HsmTier::SmartphoneHsm {
            smartphone_type: SmartphoneType::Android,
            secure_enclave: SecureEnclaveType::StrongBox(StrongBoxImplementation::TitanM),
        }
    );
    assert_eq!(info.vendor, "Google");
    assert_eq!(info.model, "Pixel 8a");

    let key_request = GenerateKeyRequest {
        key_id: "test_strongbox_key".to_string(KeyType::Symmetric,
        usage_policy: KeyUsagePolicy::default(),
        metadata: KeyMetadata::default(false,
        attestation_challenge: None,
    };

    let key = harness.android_strongbox.generate_key(key_request)?;
    assert_eq!(key.key_id, "test_strongbox_key");
    assert_eq!(key.key_type, KeyType::Symmetric);

    let test_data = b"test signature data for strongbox";
    let signature = harness
        .android_strongbox
        .sign(&key.key_id, test_data)
        ?;
    assert!(!signature.is_empty());

    let is_valid = harness
        .android_strongbox
        .verify(&key.key_id, test_data, &signature)
        ?;
    assert!(is_valid, "Signature should be valid");

    let health = harness.android_strongbox.health_check()?;
    assert!(health.healthy, "StrongBox should be healthy");

    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "strongbox", true);

    println!("✅ Android StrongBox provider tests passed");
    Ok(())
}

async fn test_software_hsm_provider(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("💾 Testing Software HSM Provider");

    let start_time = std::time::Instant::now();

    let info = harness.software_hsm.get_info()?;
    assert_eq!(
        info.hsm_type,
        HsmTier::SoftwareHsm {
            implementation: SoftwareHsmType::RustCrypto,
        }
    );

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
            metadata: KeyMetadata::default(false,
            attestation_challenge: None,
        };

        let key = harness.software_hsm.generate_key(key_request)?;
        assert_eq!(key.key_id, key_id);
        assert_eq!(key.key_type, key_type);

        if matches!(
            key_type,
            KeyType::Aes256 | KeyType::Aes128 | KeyType::Aes192 | KeyType::ChaCha20
        ) {
            let plaintext = b"test encryption data";
            let ciphertext = harness
                .software_hsm
                .encrypt(&key.metadata.key_id, plaintext)
                ?;
            assert_ne!(ciphertext, plaintext);

            let decrypted = harness
                .software_hsm
                .decrypt(&key.metadata.key_id, &ciphertext)
                ?;
            assert_eq!(decrypted, plaintext);
        }

        if matches!(
            key_type,
            KeyType::Rsa { .. }
                | KeyType::EccP256
                | KeyType::EccP384
                | KeyType::EccP521
                | KeyType::Ed25519
        ) {
            let test_data = b"test signature data";
            let signature = harness
                .software_hsm
                .sign(&key.metadata.key_id, test_data)
                ?;
            assert!(!signature.is_empty());

            let is_valid = harness
                .software_hsm
                .verify(&key.metadata.key_id, test_data, &signature)
                ?;
            assert!(is_valid, "Signature should be valid");
        }
    }

    let keys = harness.software_hsm.list_keys()?;
    assert!(keys.len() >= 3, "Should have at least 3 test keys");

    let health = harness.software_hsm.health_check()?;
    assert!(health.healthy, "Software HSM should be healthy");

    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "software", true);

    println!("✅ Software HSM provider tests passed");
    Ok(())
}

async fn test_provider_compatibility(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("🔗 Testing HSM Provider Compatibility");

    let start_time = std::time::Instant::now();

    let required_operations = vec![
        HsmOperation::KeyGeneration,
        HsmOperation::DigitalSignature,
        HsmOperation::DataEncryption,
        HsmOperation::DataDecryption,
        HsmOperation::KeyDerivation,
        HsmOperation::RandomGeneration,
    ];

    for operation in required_operations {
        let android_info = harness.android_strongbox.get_info(&mut HsmTestHarness,
) -> Result<(), BearDogError> {
    println!("🔒 Testing StrongBox-specific features");

    let info = harness.android_strongbox.get_info()?;
    if info.supports_attestation {
        println!("  ✓ Hardware attestation supported");

        let key_request = GenerateKeyRequest {
            key_id: "attested_key".to_string(KeyType::EccP256,
            usage_policy: KeyUsagePolicy::default(),
            metadata: KeyMetadata::default(false,
            attestation_challenge: Some(b"test_challenge".to_vec()),
        };

        let key = harness.android_strongbox.generate_key(key_request)?;
        assert!(
            key.attestation_certificate.is_some(),
            "Key should have attestation certificate"
        );
    }

    let security_props = harness.android_strongbox.get_security_properties()?;
    assert!(
        security_props.hardware_backed,
        "StrongBox keys should be hardware-backed"
    );
    assert!(
        security_props.strongbox_backed.unwrap_or(false),
        "Should be StrongBox-backed"
    );

    println!("✅ StrongBox-specific features tested");
    Ok(())
}

#[tokio::test]
async fn test_hsm_providers_standalone() -> Result<(), BearDogError> {
    let mut harness = super::HsmTestHarness::new()?;
    test_hsm_providers(&mut harness)
}
