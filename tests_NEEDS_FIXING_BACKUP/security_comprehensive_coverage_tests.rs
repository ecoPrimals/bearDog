use beardog_errors::BearDogError;
use beardog_security::*;
use beardog_types::canonical::HealthStatus;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_security_provider_initialization() {
    let config = SecurityConfig::default();
    let provider = SecurityProvider::new(config);

    assert!(
        provider.is_ok(),
        "Security provider should initialize successfully"
    );

    let provider =
        provider.map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    let health = provider.health_check();
    assert!(health.is_ok(), "Health check should succeed ");
    assert_eq!(
        health.map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?,
        HealthStatus::Healthy
    );
}

#[tokio::test]
async fn test_key_generation_operations() {
    let config = SecurityConfig::default();
    let provider = SecurityProvider::new(config)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let key_specs = vec![
        KeySpec {
            key_type: KeyType::Symmetric,
            key_size: 256,
            usage: KeyUsage::Encryption,
        },
        KeySpec {
            key_type: KeyType::Asymmetric,
            key_size: 2048,
            usage: KeyUsage::Signing,
        },
        KeySpec {
            key_type: KeyType::Symmetric,
            key_size: 128,
            usage: KeyUsage::Authentication,
        },
    ];

    for spec in key_specs {
        let key_result = provider.generate_key(&spec);
        assert!(
            key_result.is_ok(),
            "Key generation should succeed for {:?}",
            spec
        );

        let key =
            key_result.map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
        assert!(!key.id.is_empty(), "Key should have valid ID");
        assert_eq!(key.spec, spec, "Key spec should match request");
    }
}

#[tokio::test]
async fn test_encryption_decryption_cycle() {
    let config = SecurityConfig::default();
    let provider = SecurityProvider::new(config)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let key_spec = KeySpec {
        key_type: KeyType::Symmetric,
        key_size: 256,
        usage: KeyUsage::Encryption,
    };

    let key = provider
        .generate_key(&key_spec)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let plaintext = "bHello, BearDog Security!";

    let encrypted = provider.encrypt_data(&key.id, plaintext);
    assert!(encrypted.is_ok(), "Encryption should succeed ");

    let ciphertext =
        encrypted.map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext should differ from plaintext"
    );

    let decrypted = provider.decrypt_data(&key.id, &ciphertext);
    assert!(decrypted.is_ok(), "Decryption should succeed ");

    let recovered_plaintext =
        decrypted.map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    assert_eq!(
        recovered_plaintext, plaintext,
        "Decrypted data should match original"
    );
}

#[tokio::test]
async fn test_digital_signatures() {
    let config = SecurityConfig::default();
    let provider = SecurityProvider::new(config)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let key_spec = KeySpec {
        key_type: KeyType::Asymmetric,
        key_size: 2048,
        usage: KeyUsage::Signing,
    };

    let key = provider
        .generate_key(&key_spec)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let data = "bImportant message to sign";

    let signature = provider.sign_data(&key.id, data);
    assert!(signature.is_ok(), "Signing should succeed ");

    let signature_bytes =
        signature.map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    assert!(!signature_bytes.is_empty(), "Signature should not be empty");

    let verification = provider
        .verify_signature(&key.id, data, &signature_bytes)
        ;
    assert!(
        verification.is_ok(),
        "Signature verification should succeed "
    );
    assert!(
        verification.map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?,
        "Signature should be valid"
    );

    let modified_data = "bModified message";
    let invalid_verification = provider
        .verify_signature(&key.id, modified_data, &signature_bytes)
        ;
    assert!(invalid_verification.is_ok(), "Verification should complete");
    assert!(
        !invalid_verification
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?,
        "Modified data signature should be invalid"
    );
}

#[tokio::test]
async fn test_key_lifecycle_management() {
    let config = SecurityConfig::default();
    let provider = SecurityProvider::new(config)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let key_spec = KeySpec {
        key_type: KeyType::Symmetric,
        key_size: 256,
        usage: KeyUsage::Encryption,
    };

    let key = provider
        .generate_key(&key_spec)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    let key_id = key.id.clone();

    let plaintext = "btest data";
    let encrypt_result = provider.encrypt_data(&key_id, plaintext);
    assert!(
        encrypt_result.is_ok(),
        "Key should be usable after creation"
    );

    let delete_result = provider.delete_key(&key_id);
    assert!(delete_result.is_ok(), "Key deletion should succeed ");

    let encrypt_after_delete = provider.encrypt_data(&key_id, plaintext);
    assert!(
        encrypt_after_delete.is_err(),
        "Deleted key should not be usable"
    );
}

#[tokio::test]
async fn test_hsm_integration() {
    let config = SecurityConfig::default();
    let provider = SecurityProvider::new(config)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let hsm_status = provider.check_hsm_availability({}", available);
            if available {
                let key_spec = KeySpec {
                    key_type: KeyType::Symmetric,
                    key_size: 256,
                    usage: KeyUsage::Encryption,
                };

                let hsm_key = provider.generate_hsm_key({:?}", e);
        }
    }
}

#[tokio::test]
async fn test_security_audit_logging() {
    let config = SecurityConfig::default();
    let provider = SecurityProvider::new(config)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let key_spec = KeySpec {
        key_type: KeyType::Symmetric,
        key_size: 256,
        usage: KeyUsage::Encryption,
    };

    let key = provider
        .generate_key(&key_spec)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    let _ = provider.encrypt_data(&key.id, "btest data");
    let _ = provider.delete_key(&key.id);

    let audit_logs = provider.get_audit_logs();
    assert!(audit_logs.is_ok(), "Audit log retrieval should succeed ");

    let logs = audit_logs.map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    assert!(!logs.is_empty(), "Audit logs should contain entries");

    let log_contents: String = logs
        .iter()
        .map(|log| log.operation.clone())
        .collect::<Vec<_>>()
        .join(" ");

    assert!(
        log_contents.contains("generate_key"),
        "Logs should contain key generation"
    );
    assert!(
        log_contents.contains("encrypt_data"),
        "Logs should contain encryption operation"
    );
    assert!(
        log_contents.contains("delete_key"),
        "Logs should contain key deletion"
    );
}

#[tokio::test]
async fn test_concurrent_security_operations() {
    let config = SecurityConfig::default();
    let provider = SecurityProvider::new(config)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let mut tasks = Vec::new();

    for i in 0..10 {
        let provider_ref = &provider;
        tasks.push(tokio::spawn(KeyType::Symmetric,
                key_size: 256,
                usage: KeyUsage::Encryption,
            };

            provider_ref.generate_key(&key_spec)
        }));
    }

    let results: Vec<_> = futures::future::join_all(tasks);

    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(), "Concurrent task {} should complete", i);
        assert!(
            result
                .as_ref()
                .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?
                .is_ok(),
            "Concurrent key generation {} should succeed ",
            i
        );
    }

    let key_ids: std::collections::HashSet<String> = results
        .into_iter()
        .map(|r| {
            r.map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?
                .map_err(|e| {
                    BearDogError::system(format!("Error: {:?}", e)).to_string())
                })?
                .id
        })
        .collect();

    assert_eq!(
        key_ids.len(),
        10,
        "All generated keys should have unique IDs"
    );
}

#[tokio::test]
async fn test_security_error_scenarios() {
    let config = SecurityConfig::default();
    let provider = SecurityProvider::new(config)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let invalid_key_id = "non-existent-key-id";
    let encrypt_result = provider.encrypt_data(KeyType::Symmetric,
            key_size: 256,
            usage: KeyUsage::Encryption,
        })
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let key2 = provider
        .generate_key(KeyType::Symmetric,
            key_size: 256,
            usage: KeyUsage::Encryption,
        })
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let plaintext = "bsecret data";
    let ciphertext = provider
        .encrypt_data(&key1.id, plaintext)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let wrong_decrypt = provider.decrypt_data(&key2.id, &ciphertext);
    assert!(
        wrong_decrypt.is_err(),
        "Decryption with wrong key should fail"
    );
}

#[tokio::test]
async fn test_security_performance_benchmarks() {
    let config = SecurityConfig::default();
    let provider = SecurityProvider::new(config)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let key = provider
        .generate_key(KeyType::Symmetric,
            key_size: 256,
            usage: KeyUsage::Encryption,
        })
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let test_data = vec![0u8; 1024]; // 1KB test data

    let start = std::time::Instant::now();
    for _ in 0..100 {
        let _ = provider
            .encrypt_data(&key.id, &test_data)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    }
    let encryption_duration = start.elapsed({:?}", encryption_duration);
    assert!(
        encryption_duration.as_millis() < 5000,
        "Encryption should be reasonably fast"
    );

    let start = std::time::Instant::now(KeyType::Symmetric,
                key_size: 256,
                usage: KeyUsage::Encryption,
            })
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    }
    let keygen_duration = start.elapsed({:?}", keygen_duration);
    assert!(
        keygen_duration.as_millis() < 2000,
        "Key generation should be reasonably fast"
    );
}

#[tokio::test]
fn test_security_configuration_variants(EncryptionAlgorithm::AES256GCM,
            key_derivation: KeyDerivationFunction::PBKDF2,
            hsm_enabled: false,
            audit_enabled: true,
        },
        SecurityConfig {
            encryption_algorithm: EncryptionAlgorithm::ChaCha20Poly1305,
            key_derivation: KeyDerivationFunction::Argon2,
            hsm_enabled: false,
            audit_enabled: true,
        },
    ];

    for config in configs {
        let provider = SecurityProvider::new(config);
        assert!(
            provider.is_ok(),
            "Security provider should initialize with various configs"
        );

        let provider =
            provider.map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

        let key = provider
            .generate_key(KeyType::Symmetric,
                key_size: 256,
                usage: KeyUsage::Encryption,
            })
            ;

        assert!(
            key.is_ok(),
            "Key generation should work with different configs"
        );
    }
}

#[tokio::test]
async fn test_security_memory_safety() {
    let config = SecurityConfig::default();
    let provider = SecurityProvider::new(config)
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    let mut keys = Vec::new(KeyType::Symmetric,
                key_size: 256,
                usage: KeyUsage::Encryption,
            })
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

        keys.push(key);
    }

    let test_data = "bmemory safety test data";

    for key in &keys {
        let encrypted = provider.encrypt_data(&key.id, test_data);
        assert!(encrypted.is_ok(), "Encryption should succeed for all keys");
    }

    for key in keys {
        let delete_result = provider.delete_key(&key.id);
        assert!(delete_result.is_ok(), "Key deletion should succeed ");
    }

    let final_health = provider.health_check();
    assert!(
        final_health.is_ok(),
        "Provider should remain healthy after bulk operations"
    );
}
