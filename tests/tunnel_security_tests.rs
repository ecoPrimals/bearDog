

use beardog::tunnel::key_manager::{BStpKeyManager, CryptoAlgorithm};
use beardog::tunnel::{config::BStpConfig, session::SessionManager};
use beardog::BearDogResult;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_configuration_profiles() -> BearDogResult<()> {

    let competitive = BStpConfig::competitive_gaming();
    assert!(competitive.performance.max_encryption_latency <= Duration::from_micros(100));
    assert!(competitive.gaming.ultra_low_latency);
    assert!(competitive.gaming.prefer_hardware_crypto);

    let max_security = BStpConfig::maximum_security();
    assert!(max_security.key_management.use_hardware_keys);
    assert!(max_security.key_management.key_derivation_rounds >= 10000);
    assert!(max_security.genetic_healing.enable_healing);

    let default = BStpConfig::from_env();
    assert!(default.performance.max_encryption_latency > Duration::from_micros(0));
    assert!(default.key_management.session_key_length > 0);

    Ok(())
}

#[tokio::test]
async fn test_key_management_basic() -> BearDogResult<()> {
    let config = BStpConfig::competitive_gaming();
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management).await?);

    let session_id = "test_session_001";
    let key1 = key_manager
        .generate_session_key(session_id, CryptoAlgorithm::Aes256Gcm)
        .await?;
    let key2 = key_manager
        .generate_session_key(session_id, CryptoAlgorithm::ChaCha20Poly1305)
        .await?;

    assert_ne!(key1.key, key2.key);
    assert_eq!(key1.algorithm, CryptoAlgorithm::Aes256Gcm);
    assert_eq!(key2.algorithm, CryptoAlgorithm::ChaCha20Poly1305);

    let retrieved = key_manager.get_session_key(session_id).await;
    assert!(retrieved.is_some());

    Ok(())
}

#[tokio::test]
async fn test_session_manager_basic() -> BearDogResult<()> {
    let session_manager = SessionManager::new();

    let session_id = "test_session_lifecycle";

    let initial = session_manager.get_session(session_id).await;
    assert!(initial.is_none());

    Ok(())
}

#[tokio::test]
async fn test_algorithm_types() -> BearDogResult<()> {
    let config = BStpConfig::competitive_gaming();
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management).await?);

    let session_id = "algo_test_session";

    let aes_key = key_manager
        .generate_session_key(session_id, CryptoAlgorithm::Aes256Gcm)
        .await?;
    assert_eq!(aes_key.algorithm, CryptoAlgorithm::Aes256Gcm);
    assert_eq!(aes_key.key.len(), 32); // 256 bits

    let chacha_key = key_manager
        .generate_session_key(session_id, CryptoAlgorithm::ChaCha20Poly1305)
        .await?;
    assert_eq!(chacha_key.algorithm, CryptoAlgorithm::ChaCha20Poly1305);
    assert_eq!(chacha_key.key.len(), 32); // 256 bits

    Ok(())
}

#[tokio::test]
async fn test_security_compliance_requirements() -> BearDogResult<()> {
    let config = BStpConfig::maximum_security();

    assert!(config.key_management.key_derivation_rounds >= 10000);
    assert!(config.key_management.use_hardware_keys);

    assert!(config.key_management.key_rotation_interval <= Duration::from_secs(24 * 3600));

    Ok(())
}

#[tokio::test]
async fn test_gaming_performance_requirements() -> BearDogResult<()> {
    let config = BStpConfig::competitive_gaming();

    assert!(config.performance.max_encryption_latency <= Duration::from_micros(100));
    assert!(config.performance.max_decryption_latency <= Duration::from_micros(100));
    assert!(config.performance.max_session_setup_time <= Duration::from_millis(10));
    assert!(config.performance.min_gaming_throughput >= 1000); // packets/sec

    Ok(())
}

#[tokio::test]
async fn test_key_generation_security() -> BearDogResult<()> {
    let config = BStpConfig::competitive_gaming();
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management).await?);

    let mut keys = Vec::new();
    for i in 0..10 {
        let session_id = format!("security_test_{i}");
        let key = key_manager
            .generate_session_key(&session_id, CryptoAlgorithm::Aes256Gcm)
            .await?;
        keys.push(key.key);
    }

    for i in 0..keys.len() {
        for j in (i + 1)..keys.len() {
            assert_ne!(keys[i], keys[j], "Keys {i} and {j} are identical!");
        }
    }

    Ok(())
}
