

use beardog::config::EncryptionConfig;
use beardog::encryption::EncryptionEngine;
use beardog::genetics::{DefaultBearDogGeneticsEngine, GeneticsConfig, InMemoryGeneticsStore};
use beardog::tunnel::{
    genetic_healing::{NetworkEvent, SecurityIssue, SecurityIssueType, Severity},
    key_manager::CryptoAlgorithm,
    BStpConfig, BStpKeyManager, BStpSecurityProvider, GamingCryptoEngine, GeneticSecurityHealing,
    SecurityGenetics,
};
use beardog::BearDogResult;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio;

#[tokio::test]
async fn test_key_manager_security() -> BearDogResult<()> {

    let config = BStpConfig::default();
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);

    let session_id = "test_session_123";

    let key1 = key_manager
        .generate_session_key(session_id, CryptoAlgorithm::Aes256Gcm)
        .await?;
    let key2 = key_manager
        .generate_session_key("different_session", CryptoAlgorithm::ChaCha20Poly1305)
        .await?;

    assert_ne!(key1.key, key2.key);
    assert_ne!(key1.key_id, key2.key_id);

    let retrieved_key = key_manager
        .get_session_key(session_id)
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Session key should exist", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Session key should exist", e).to_string())
})?;
    assert_eq!(retrieved_key.key, key1.key);
    assert_eq!(retrieved_key.algorithm, CryptoAlgorithm::Aes256Gcm);

    let rotated_key = key_manager
        .rotate_session_key(session_id, Some(CryptoAlgorithm::ChaCha20Poly1305))
        .await?;
    assert_ne!(rotated_key.key, key1.key);
    assert_eq!(rotated_key.algorithm, CryptoAlgorithm::ChaCha20Poly1305);

    println!("✅ Key manager security tests passed");
    Ok(())
}

#[tokio::test]
async fn test_gaming_crypto_performance() -> BearDogResult<()> {

    let config = BStpConfig::competitive_gaming();
    let encryption = Arc::new(EncryptionEngine::new(EncryptionConfig::default()).await?);
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);

    let crypto_engine =
        GamingCryptoEngine::new(encryption, genetics, key_manager, config.clone()).await?;

    let session_id = "performance_test_session";
    let test_data = b"StarCraft2 gaming packet data for ultra-low latency testing";
    let security_genetics = beardog::tunnel::SecurityGenetics::default();

    let start = Instant::now();
    let encrypted = crypto_engine
        .ultra_fast_encrypt(session_id, test_data, &security_genetics)
        .await?;
    let encryption_time = start.elapsed();

    let start = Instant::now();
    let decrypted = crypto_engine
        .ultra_fast_decrypt(session_id, &encrypted, &security_genetics)
        .await?;
    let decryption_time = start.elapsed();

    assert_eq!(test_data, &decrypted[..]);

    assert!(
        encryption_time <= config.performance.max_encryption_latency,
        "Encryption took {}μs, target was {}μs",
        encryption_time.as_micros(),
        config.performance.max_encryption_latency.as_micros()
    );

    assert!(
        decryption_time <= config.performance.max_decryption_latency,
        "Decryption took {}μs, target was {}μs",
        decryption_time.as_micros(),
        config.performance.max_decryption_latency.as_micros()
    );

    println!("✅ Gaming crypto performance tests passed");
    println!(
        "   Encryption: {}μs (target: {}μs)",
        encryption_time.as_micros(),
        config.performance.max_encryption_latency.as_micros()
    );
    println!(
        "   Decryption: {}μs (target: {}μs)",
        decryption_time.as_micros(),
        config.performance.max_decryption_latency.as_micros()
    );

    Ok(())
}

#[tokio::test]
async fn test_crypto_algorithm_selection() -> BearDogResult<()> {

    let config = BStpConfig::default();
    let encryption = Arc::new(EncryptionEngine::new(EncryptionConfig::default()).await?);
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);

    let crypto_engine =
        GamingCryptoEngine::new(encryption, genetics, key_manager, config.clone()).await?;

    let security_genetics = beardog::tunnel::SecurityGenetics::default();

    let small_data = vec![0u8; 512];
    let encrypted_small = crypto_engine
        .ultra_fast_encrypt("small_packet_session", &small_data, &security_genetics)
        .await?;
    assert_eq!(encrypted_small.crypto_method, CryptoAlgorithm::Aes256Gcm);

    let medium_data = vec![0u8; 1500];
    let encrypted_medium = crypto_engine
        .ultra_fast_encrypt("medium_packet_session", &medium_data, &security_genetics)
        .await?;
    assert_eq!(
        encrypted_medium.crypto_method,
        CryptoAlgorithm::ChaCha20Poly1305
    );

    let large_data = vec![0u8; 4096];
    let encrypted_large = crypto_engine
        .ultra_fast_encrypt("large_packet_session", &large_data, &security_genetics)
        .await?;
    assert_eq!(
        encrypted_large.crypto_method,
        CryptoAlgorithm::GeneticHybrid
    );

    println!("✅ Crypto algorithm selection tests passed");
    Ok(())
}

#[tokio::test]
async fn test_session_security() -> BearDogResult<()> {

    let config = BStpConfig::maximum_security();
    let encryption = Arc::new(EncryptionEngine::new(EncryptionConfig::default()).await?);
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);

    let crypto_engine =
        GamingCryptoEngine::new(encryption.clone(), genetics.clone(), key_manager, config).await?;

    let peer_id = "secure_gaming_peer";

    let test_data = b"session security test data";
    let security_genetics = beardog::tunnel::SecurityGenetics::default();

    let encrypted = crypto_engine
        .ultra_fast_encrypt(peer_id, test_data, &security_genetics)
        .await?;

    let decrypted = crypto_engine
        .ultra_fast_decrypt(peer_id, &encrypted, &security_genetics)
        .await?;

    assert_eq!(test_data, &decrypted[..]);

    println!("✅ Session security tests passed");
    Ok(())
}

#[tokio::test]
async fn test_genetic_healing_adaptation() -> BearDogResult<()> {

    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let mut healing = GeneticSecurityHealing::new(genetics).await?;

    let security_issue = SecurityIssue {
        issue_type: SecurityIssueType::EncryptionCompromised,
        severity: Severity::High,
        description: "Test encryption compromise".to_string(),
        timestamp: SystemTime::now(),
    };

    let healing_result = healing.heal_security_issue(security_issue).await?;
    assert_eq!(
        healing_result,
        beardog::tunnel::genetic_healing::HealingResult::Success
    );

    let network_event = NetworkEvent::PeerDisconnected {
        reason: "Suspicious behavior detected".to_string(),
    };

    healing.heal_from_network_event(network_event).await?;

    println!("✅ Genetic healing adaptation tests passed");
    Ok(())
}

#[tokio::test]
async fn test_configuration_profiles() -> BearDogResult<()> {

    let competitive = BStpConfig::competitive_gaming();
    assert_eq!(
        competitive.performance.max_encryption_latency,
        Duration::from_micros(50)
    );
    assert!(competitive.gaming.ultra_low_latency);
    assert_eq!(competitive.genetic_healing.mutation_rate, 0.05);

    let max_security = BStpConfig::maximum_security();
    assert_eq!(
        max_security.key_management.key_rotation_interval,
        Duration::from_secs(900)
    );
    assert_eq!(max_security.genetic_healing.crossover_rate, 1.0);
    assert_eq!(max_security.genetic_healing.population_size, 100);

    std::env::set_var("BEARDOG_GAMING_MODE", "1");
    let env_config = BStpConfig::from_env();
    assert!(env_config.gaming.ultra_low_latency);
    std::env::remove_var("BEARDOG_GAMING_MODE");

    println!("✅ Configuration profile tests passed");
    Ok(())
}

#[tokio::test]
async fn test_key_expiration_and_rotation() -> BearDogResult<()> {

    let mut config = BStpConfig::default();
    config.key_management.key_rotation_interval = Duration::from_millis(100); // Short interval for testing

    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);

    let session_id = "expiration_test_session";

    let key1 = key_manager
        .generate_session_key(session_id, CryptoAlgorithm::Aes256Gcm)
        .await?;

    tokio::time::sleep(Duration::from_millis(150)).await;

    let result = key_manager.get_session_key(session_id).await;
    assert!(result.is_none());

    println!("✅ Key expiration and rotation tests passed");
    Ok(())
}

#[tokio::test]
async fn test_contextual_key_derivation() -> BearDogResult<()> {

    let config = BStpConfig::default();
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);

    let session_id = "context_test_session";
    let base_key = key_manager
        .generate_session_key(session_id, CryptoAlgorithm::Aes256Gcm)
        .await?;

    let context1 = b"gaming_context_1";
    let context2 = b"gaming_context_2";

    let derived_key1 = key_manager
        .derive_contextual_key(&base_key, context1)
        .await?;
    let derived_key2 = key_manager
        .derive_contextual_key(&base_key, context2)
        .await?;

    assert_ne!(derived_key1, derived_key2);

    let derived_key1_again = key_manager
        .derive_contextual_key(&base_key, context1)
        .await?;
    assert_eq!(derived_key1, derived_key1_again);

    println!("✅ Contextual key derivation tests passed");
    Ok(())
}

#[tokio::test]
async fn test_stress_performance() -> BearDogResult<()> {

    let config = BStpConfig::competitive_gaming();
    let encryption = Arc::new(EncryptionEngine::new(EncryptionConfig::default()).await?);
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);

    let crypto_engine =
        Arc::new(GamingCryptoEngine::new(encryption, genetics, key_manager, config.clone()).await?);

    let security_genetics = beardog::tunnel::SecurityGenetics::default();
    let test_data = b"Real-time gaming packet";

    let start = Instant::now();
    let mut handles = Vec::new();

    for i in 0..100 {
        let crypto_engine = crypto_engine.clone();
        let session_id = format_args!("stress_session_{}", i).to_string();
        let data = test_data.clone();
        let genetics = security_genetics.clone();

        let handle = tokio::spawn(async move {
            crypto_engine
                .ultra_fast_encrypt(&session_id, &data, &genetics)
                .await
        });
        handles.push(handle);
    }

    let mut successes = 0;
    for handle in handles {
        if handle.await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?.is_ok() {
            successes += 1;
        }
    }

    let total_time = start.elapsed();
    let avg_time_per_op = total_time / 100;

    assert_eq!(successes, 100, "All stress test encryptions should succeed");
    assert!(
        avg_time_per_op <= config.performance.max_encryption_latency,
        "Average encryption time {}μs exceeds target {}μs under load",
        avg_time_per_op.as_micros(),
        config.performance.max_encryption_latency.as_micros()
    );

    println!("✅ Stress performance tests passed");
    println!(
        "   100 concurrent encryptions: {}ms total, {}μs avg",
        total_time.as_millis(),
        avg_time_per_op.as_micros()
    );

    Ok(())
}

#[tokio::test]
async fn test_security_context_isolation() -> BearDogResult<()> {

    println!("🔒 Testing security context isolation...");

    let config = BStpConfig::competitive_gaming();
    let encryption = Arc::new(EncryptionEngine::new(EncryptionConfig::default()).await?);
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);

    let crypto_engine = GamingCryptoEngine::new(
        encryption.clone(),
        genetics.clone(),
        key_manager.clone(),
        config.clone(),
    )
    .await?;

    let session1_data = b"session1 isolated data";
    let session2_data = b"session2 isolated data";
    let security_genetics = beardog::tunnel::SecurityGenetics::default();

    let encrypted1 = crypto_engine
        .ultra_fast_encrypt("session1", session1_data, &security_genetics)
        .await?;

    let encrypted2 = crypto_engine
        .ultra_fast_encrypt("session2", session2_data, &security_genetics)
        .await?;

    let decrypted1 = crypto_engine
        .ultra_fast_decrypt("session1", &encrypted1, &security_genetics)
        .await?;

    let decrypted2 = crypto_engine
        .ultra_fast_decrypt("session2", &encrypted2, &security_genetics)
        .await?;

    assert_eq!(session1_data, &decrypted1[..]);
    assert_eq!(session2_data, &decrypted2[..]);

    println!("✅ Security context isolation tests passed");
    Ok(())
}
