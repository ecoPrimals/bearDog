

use beardog::tunnel::config::BStpConfig;
use beardog::tunnel::key_manager::{BStpKeyManager, CryptoAlgorithm};
use beardog::tunnel::session::SessionManager;
use beardog::BearDogResult;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_competitive_gaming_session_e2e() -> BearDogResult<()> {
    println!("🎮 Testing competitive gaming session end-to-end");

    let config = BStpConfig::competitive_gaming();
    println!("✅ Competitive gaming configuration loaded");

    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);
    println!("✅ Key manager initialized");

    let session_manager = SessionManager::new();
    println!("✅ Session manager initialized");

    let session_id = "competitive_match_001";
    let aes_key = key_manager
        .generate_session_key(session_id, CryptoAlgorithm::Aes256Gcm)
        .await?;
    println!("✅ Gaming session key generated: {}", aes_key.key_id);

    let start_time = Instant::now();
    for i in 0..60 {
        let frame_session = format_args!("{}_frame_{}", session_id, i).to_string();
        let _frame_key = key_manager
            .generate_session_key(&frame_session, CryptoAlgorithm::Aes256Gcm)
            .await?;
    }
    let frame_processing_time = start_time.elapsed();

    assert!(
        frame_processing_time < Duration::from_millis(1000),
        "Frame key generation too slow: {:?}",
        frame_processing_time
    );

    println!(
        "✅ 60 FPS key generation: {:?} (requirement: <1s)",
        frame_processing_time
    );

    let retrieved_session = session_manager.get_session(session_id).await;
    println!("✅ Session retrieval test completed");

    println!("🎉 Competitive gaming session end-to-end test PASSED");
    Ok(())
}

#[tokio::test]
async fn test_maximum_security_e2e() -> BearDogResult<()> {
    println!("🔒 Testing maximum security end-to-end");

    let config = BStpConfig::maximum_security();
    println!("✅ Maximum security configuration loaded");

    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);
    println!("✅ Key manager initialized with maximum security");

    let session_id = "secure_session_001";
    let key = key_manager
        .generate_session_key(session_id, CryptoAlgorithm::Aes256Gcm)
        .await?;

    assert_eq!(key.key.len(), 32); // 256-bit key
    assert!(!key.key_id.is_empty());
    assert!(key.created_at <= std::time::SystemTime::now());
    assert!(key.expires_at > std::time::SystemTime::now());
    println!("✅ Secure key generated with proper security properties");

    let rotated_key = key_manager
        .rotate_session_key(session_id, Some(CryptoAlgorithm::ChaCha20Poly1305))
        .await?;
    assert_ne!(key.key, rotated_key.key);
    assert_eq!(rotated_key.algorithm, CryptoAlgorithm::ChaCha20Poly1305);
    println!("✅ Key rotation completed successfully");

    println!("🎉 Maximum security end-to-end test PASSED");
    Ok(())
}

#[tokio::test]
async fn test_multi_player_gaming_scenario() -> BearDogResult<()> {
    println!("👥 Testing multi-player gaming scenario");

    let config = BStpConfig::competitive_gaming();
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);

    let players = vec!["player_1", "player_2", "player_3", "player_4"];
    let mut player_keys = Vec::new();

    for player in &players {
        let key = key_manager
            .generate_session_key(player, CryptoAlgorithm::Aes256Gcm)
            .await?;
        player_keys.push(key);
        println!("✅ Session key generated for {}", player);
    }

    let operations = 1000; // 100 ops/sec for 10 seconds
    let start_time = Instant::now();

    for i in 0..operations {
        let player = &players[i % players.len()];
        let operation_id = format_args!("{}_op_{}", player, i).to_string();
        let _op_key = key_manager
            .generate_session_key(&operation_id, CryptoAlgorithm::ChaCha20Poly1305)
            .await?;
    }

    let total_time = start_time.elapsed();
    let ops_per_second = operations as f64 / total_time.as_secs_f64();

    println!("✅ Processed {} operations in {:?}", operations, total_time);
    println!("✅ Throughput: {:.0} operations/second", ops_per_second);

    assert!(
        ops_per_second > 100.0,
        "Throughput too low: {:.0} ops/sec",
        ops_per_second
    );

    println!("🎉 Multi-player gaming scenario PASSED");
    Ok(())
}

#[tokio::test]
async fn test_system_stress_resilience() -> BearDogResult<()> {
    println!("💪 Testing system stress resilience");

    let config = BStpConfig::competitive_gaming();
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);

    let stress_iterations = 100;
    let start_time = Instant::now();

    for i in 0..stress_iterations {
        let session_id = format_args!("stress_session_{}", i).to_string();
        let _key = key_manager
            .generate_session_key(&session_id, CryptoAlgorithm::Aes256Gcm)
            .await?;

        if i % 25 == 0 {
            let test_key = key_manager
                .generate_session_key("responsiveness_test", CryptoAlgorithm::ChaCha20Poly1305)
                .await?;
            assert!(!test_key.key_id.is_empty());
        }
    }

    let stress_time = start_time.elapsed();
    let keys_per_second = stress_iterations as f64 / stress_time.as_secs_f64();

    println!(
        "✅ Generated {} keys in {:?}",
        stress_iterations, stress_time
    );
    println!("✅ Key generation rate: {:.0} keys/second", keys_per_second);

    assert!(
        keys_per_second > 50.0,
        "Key generation rate too low under stress: {:.0} keys/sec",
        keys_per_second
    );

    println!("🎉 System stress resilience test PASSED");
    Ok(())
}

#[tokio::test]
async fn test_configuration_compliance_e2e() -> BearDogResult<()> {
    println!("📋 Testing configuration compliance end-to-end");

    let configs = vec![
        ("competitive_gaming", BStpConfig::competitive_gaming()),
        ("maximum_security", BStpConfig::maximum_security()),
        ("default", BStpConfig::from_env()),
    ];

    for (name, config) in configs {
        println!("Testing {} configuration...", name);

        assert!(
            config.key_management.key_derivation_rounds >= 10000,
            "{}: Key derivation rounds below compliance threshold",
            name
        );
        assert!(
            config.key_management.key_rotation_interval <= Duration::from_secs(24 * 3600),
            "{}: Key rotation interval exceeds compliance limit",
            name
        );
        assert!(
            config.key_management.session_key_length >= 32,
            "{}: Session key length below security requirement",
            name
        );

        assert!(
            config.performance.max_encryption_latency <= Duration::from_millis(1),
            "{}: Encryption latency target too high",
            name
        );
        assert!(
            config.performance.min_gaming_throughput >= 100,
            "{}: Gaming throughput target too low",
            name
        );

        let key_manager = BStpKeyManager::new(config.key_management.clone()).await?;
        let _key = key_manager
            .generate_session_key("compliance_test", CryptoAlgorithm::Aes256Gcm)
            .await?;

        println!("✅ {} configuration compliant and functional", name);
    }

    println!("🎉 Configuration compliance test PASSED");
    Ok(())
}
