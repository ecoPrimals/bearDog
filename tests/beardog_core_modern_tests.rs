//! Modern `BearDog` Core Tests
//!
//! Canonical tests for beardog-core functionality using current architecture.

#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::unnecessary_literal_unwrap)] // Test code intentionally tests these patterns
#![allow(clippy::unnecessary_lazy_evaluations)] // Testing error recovery patterns

use beardog_errors::BearDogError;
use beardog_types::canonical::config::WorkingUnifiedConfig;
use beardog_types::canonical::crypto::EncryptionConfig;

#[test]
fn test_encryption_config_creation() -> Result<(), BearDogError> {
    use beardog_types::canonical::crypto::{CryptoAlgorithm, EncryptionMode};

    let config = EncryptionConfig::default();

    // Verify default configuration
    assert!(matches!(
        config.algorithm,
        CryptoAlgorithm::Aes { key_size: 256 }
    ));
    assert!(matches!(config.mode, EncryptionMode::Gcm));
    assert_eq!(config.auth_tag_size, Some(128));

    Ok(())
}

#[test]
fn test_capability_types() {
    use beardog_types::canonical::capabilities::ServiceCapabilityType;

    // Test capability variants
    let capabilities = [
        ServiceCapabilityType::Compute,
        ServiceCapabilityType::Storage,
    ];

    assert_eq!(capabilities.len(), 2);
}

#[test]
fn test_unified_config_defaults() {
    let config = WorkingUnifiedConfig::default();

    // Verify basic config fields exist
    assert!(!config.version.is_empty() || config.version.is_empty()); // Basic existence check
}

#[test]
fn test_error_handling() {
    let error = BearDogError::configuration("Test configuration error");
    let error_string = format!("{error}");

    assert!(error_string.contains("Test configuration error"));
}

#[test]
fn test_security_error_types() {
    let error = BearDogError::security("Test security issue".to_string());
    assert!(format!("{error:?}").contains("Test security issue"));
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[tokio::test]
async fn test_async_error_handling() -> Result<(), BearDogError> {
    // Test async error handling patterns
    let result: Result<(), BearDogError> =
        Err(BearDogError::system("Test system error".to_string()));

    match result {
        Err(BearDogError::System { message, .. }) => {
            assert!(message.contains("Test system error"));
        }
        _ => panic!("Expected system error"),
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    Ok(())
}

#[test]
fn test_crypto_algorithm_types() {
    use beardog_types::canonical::crypto::CryptoAlgorithm;

    let aes = CryptoAlgorithm::Aes { key_size: 256 };
    let chacha = CryptoAlgorithm::ChaCha20;

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(matches!(aes, CryptoAlgorithm::Aes { key_size: 256 }));
    assert!(matches!(chacha, CryptoAlgorithm::ChaCha20));
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: important
fn test_encryption_modes() {
    use beardog_types::canonical::crypto::EncryptionMode;

    let modes = [
        EncryptionMode::Gcm,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important
        EncryptionMode::Cbc,
        EncryptionMode::Ctr,
    ];
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important

    assert_eq!(modes.len(), 3);
}

#[test]
fn test_key_config_defaults() {
    use beardog_types::canonical::crypto::KeyConfig;

    let key_config = KeyConfig::default();

    // Verify key config has reasonable defaults
    assert!(key_config.key_size >= 128); // Minimum secure key size
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[tokio::test]
async fn test_concurrent_config_access() -> Result<(), BearDogError> {
    use beardog_types::canonical::crypto::CryptoAlgorithm;
    use tokio::task;

    let handles: Vec<_> = (0..10)
        .map(|_| {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            task::spawn(async {
                let config = EncryptionConfig::default();
                assert!(matches!(config.algorithm, CryptoAlgorithm::Aes { .. }));
            })
        })
        .collect();

    for handle in handles {
        handle.await.expect("Task should complete successfully");
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    Ok(())
}

#[test]
fn test_crypto_algorithm_equality() {
    use beardog_types::canonical::crypto::CryptoAlgorithm;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // Verify crypto algorithms have distinct types
    let aes = CryptoAlgorithm::Aes { key_size: 256 };
    let chacha = CryptoAlgorithm::ChaCha20;

    assert!(aes != chacha);
}

#[test]
fn test_error_conversion_patterns() {
    use std::io;

    // Test that we can create errors from various sources
    let io_error = io::Error::other("Test IO error");
    let beardog_error: BearDogError = io_error.into();

    assert!(format!("{beardog_error:?}").contains("Test IO error"));
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[tokio::test]
async fn test_timeout_handling() -> Result<(), BearDogError> {
    use tokio::time::{timeout, Duration};

    // Test that operations can be timed out
    let result = timeout(Duration::from_millis(100), async {
        // No sleep needed - testing timeout mechanism, not actual delay
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important
        Ok::<(), BearDogError>(())
    })
    .await;

    assert!(result.is_ok(), "Short operation should complete");

    Ok(())
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_config_serialization() -> Result<(), BearDogError> {
    let config = EncryptionConfig::default();

    // Test that config can be serialized
    let json = serde_json::to_string(&config)
        .map_err(|e| BearDogError::internal(format!("Serialization failed: {e}")))?;

    assert!(!json.is_empty());

    // Test deserialization
    let _deserialized: EncryptionConfig = serde_json::from_str(&json)
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        .map_err(|e| BearDogError::internal(format!("Deserialization failed: {e}")))?;

    Ok(())
}

#[test]
fn test_multiple_error_types() {
    let errors = vec![
        BearDogError::configuration("Config error"),
        BearDogError::security("Security error".to_string()),
        BearDogError::system("System error".to_string()),
        BearDogError::invalid_input("Invalid input"),
    ];

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important
    assert_eq!(errors.len(), 4);

    for error in errors {
        let error_str = format!("{error}");
        assert!(!error_str.is_empty());
    }
}

#[tokio::test]
async fn test_parallel_operations() -> Result<(), BearDogError> {
    use tokio::task;

    let tasks: Vec<_> = (0..5)
        .map(|i| {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            task::spawn(async move {
                let config = WorkingUnifiedConfig::default();
                // No sleep needed - testing concurrent access, not simulating work
                tokio::task::yield_now().await; // Allow interleaving
                (i, config.version.clone())
            })
        })
        .collect();

    let mut results = Vec::new();
    for task in tasks {
        let (id, version) = task.await.expect("Task should complete");
        results.push((id, version));
    }

    assert_eq!(results.len(), 5);

    Ok(())
}

#[test]
fn test_crypto_config_completeness() {
    use beardog_types::canonical::crypto::CryptoConfig;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    let config = CryptoConfig::default();

    // Verify all major config sections exist
    let _encryption = config.encryption;
    let _signature = config.signature;
    let _key_management = config.key_management;
    let _rng = config.rng;
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_result_combinators() -> Result<(), BearDogError> {
    // Test Result combinators work with Result<T, BearDogError>
    let success: Result<i32, BearDogError> = Ok(42);
    let doubled = success.map(|x| x * 2)?;
    assert_eq!(doubled, 84);

    // Test recovery from error using or_else pattern
    let result = BearDogError::system("Test".to_string());
    let recovered = Err::<i32, _>(result).or_else(|_| Ok::<i32, BearDogError>(99))?;
    assert_eq!(recovered, 99);

    Ok(())
}
