

use beardog::config::EncryptionConfig;
use beardog::encryption::EncryptionEngine;
use beardog::genetics::{DefaultBearDogGeneticsEngine, GeneticsConfig, InMemoryGeneticsStore};
use beardog::tunnel::{
    genetic_healing::{HealingResult, NetworkEvent, SecurityIssue, SecurityIssueType, Severity},
    BStpConfig, BStpKeyManager, GamingCryptoEngine, GeneticSecurityHealing, SecurityGenetics,
};
use beardog::{BearDogError, BearDogResult};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};

#[tokio::test]
async fn test_full_bstp_integration() -> BearDogResult<()> {

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

    let mut healing = GeneticSecurityHealing::new(genetics.clone()).await?;

    println!("🎮 Testing full gaming session workflow...");

    let security_genetics = SecurityGenetics::default();
    let gaming_packets = [b"player_move_command_1".to_vec(),
        b"player_attack_unit_2".to_vec(),
        b"player_build_structure_3".to_vec()];

    let mut total_encryption_time = Duration::ZERO;
    let mut total_decryption_time = Duration::ZERO;

    for (i, packet) in gaming_packets.iter().enumerate() {
        let session_id_packet = format!("test_session_{i}");

        let start = Instant::now();
        let encrypted = crypto_engine
            .ultra_fast_encrypt(&session_id_packet, packet, &security_genetics)
            .await?;
        total_encryption_time += start.elapsed();

        let start = Instant::now();
        let decrypted = crypto_engine
            .ultra_fast_decrypt(&session_id_packet, &encrypted, &security_genetics)
            .await?;
        total_decryption_time += start.elapsed();

        assert_eq!(packet, &decrypted);
    }

    let security_issue = SecurityIssue {
        issue_type: SecurityIssueType::PerformanceDegradation,
        severity: Severity::Medium,
        description: "Gaming latency spike detected".to_string(),
        timestamp: SystemTime::now(),
    };

    let healing_result = healing.heal_security_issue(security_issue).await?;
    assert_eq!(healing_result, HealingResult::Success);

    let network_event = NetworkEvent::NetworkCongestion { latency_ms: 150 };
    healing.heal_from_network_event(network_event).await?;

    let avg_encryption = total_encryption_time / gaming_packets.len() as u32;
    let avg_decryption = total_decryption_time / gaming_packets.len() as u32;

    assert!(avg_encryption <= config.performance.max_encryption_latency);
    assert!(avg_decryption <= config.performance.max_decryption_latency);

    println!("✅ Full BSTP integration test passed");
    println!("   Average encryption: {}μs", avg_encryption.as_micros());
    println!("   Average decryption: {}μs", avg_decryption.as_micros());

    Ok(())
}

#[tokio::test]
async fn test_krogan_resilience_scenario() -> BearDogResult<()> {

    println!("🦎 Testing krogan-grade resilience to multiple threats...");

    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let mut healing = GeneticSecurityHealing::new(genetics.clone()).await?;

    let threats = vec![
        SecurityIssue {
            issue_type: SecurityIssueType::EncryptionCompromised,
            severity: Severity::Critical,
            description: "Crypto algorithm compromised".to_string(),
            timestamp: SystemTime::now(),
        },
        SecurityIssue {
            issue_type: SecurityIssueType::AuthenticationBreach,
            severity: Severity::High,
            description: "Authentication system breach".to_string(),
            timestamp: SystemTime::now(),
        },
        SecurityIssue {
            issue_type: SecurityIssueType::NetworkAnomaly,
            severity: Severity::Medium,
            description: "Suspicious network activity".to_string(),
            timestamp: SystemTime::now(),
        },
    ];

    for threat in threats {
        let result = healing.heal_security_issue(threat).await?;
        assert_eq!(result, HealingResult::Success);
    }

    let network_events = vec![
        NetworkEvent::PeerDisconnected {
            reason: "Connection lost".to_string(),
        },
        NetworkEvent::NetworkCongestion { latency_ms: 500 },
        NetworkEvent::SuspiciousTraffic {
            source: "unknown_peer".to_string(),
        },
    ];

    for event in network_events {
        healing.heal_from_network_event(event).await?;
    }

    println!("✅ Krogan resilience test passed - adapted to all threats!");
    Ok(())
}

#[tokio::test]
async fn test_toadstool_extension_interfaces() -> BearDogResult<()> {

    println!("🍄 Testing basic toadstool-compute interface...");

    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let mut healing = GeneticSecurityHealing::new(genetics.clone()).await?;

    let security_issue = SecurityIssue {
        issue_type: SecurityIssueType::EncryptionCompromised,
        severity: Severity::High,
        description: "Test crypto issue for toadstool interface".to_string(),
        timestamp: SystemTime::now(),
    };

    println!("🍄 Simulating network healing coordination...");
    let healing_result = healing.heal_security_issue(security_issue).await?;
    assert_eq!(healing_result, HealingResult::Success);

    println!("✅ Basic toadstool interface test passed - ready for network evolution!");
    Ok(())
}

#[tokio::test]
async fn test_concurrent_gaming_sessions() -> BearDogResult<()> {

    println!("🕹️ Testing concurrent gaming sessions...");

    let config = BStpConfig::competitive_gaming();
    let encryption = Arc::new(EncryptionEngine::new(EncryptionConfig::default()).await?);
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);

    let crypto_engine = Arc::new(
        GamingCryptoEngine::new(encryption, genetics.clone(), key_manager, config.clone()).await?,
    );

    let num_sessions = 10;
    let mut session_handles = Vec::new();

    for i in 0..num_sessions {
        let engine = crypto_engine.clone();
        let config_clone = config.clone();

        let handle = tokio::spawn(async move {

            let session_id = format!("concurrent_session_{i}");
            let encryption_start = Instant::now();

            let security_genetics = SecurityGenetics::default();
            let packet = format!("gaming_packet_from_peer_{i}").into_bytes();

            let encrypted = engine
                .ultra_fast_encrypt(&session_id, &packet, &security_genetics)
                .await?;

            let decrypted = engine
                .ultra_fast_decrypt(&session_id, &encrypted, &security_genetics)
                .await?;

            assert_eq!(packet, decrypted);

            let encryption_time = encryption_start.elapsed();
            assert!(encryption_time <= Duration::from_micros(200)); // Gaming target

            let is_valid = !encrypted.data.is_empty();
            assert!(is_valid);

            Ok::<(), BearDogError>(())
        });

        session_handles.push(handle);
    }

    for handle in session_handles {
        handle.await.map_err(|e| BearDogError::OperationTimeout {
            operation: e.to_string(),
        })??;
    }

    println!(
        "✅ Concurrent gaming sessions test passed - {num_sessions} sessions handled"
    );
    Ok(())
}

#[tokio::test]
async fn test_gaming_crypto_optimization() -> BearDogResult<()> {

    println!("🎯 Testing gaming crypto optimization...");

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

    let security_genetics = SecurityGenetics::default();
    let test_data = b"StarCraft 2 competitive match data";

    let start = Instant::now();
    let encrypted = crypto_engine
        .ultra_fast_encrypt("optimization_test_session", test_data, &security_genetics)
        .await?;
    let encryption_time = start.elapsed();

    let start = Instant::now();
    let decrypted = crypto_engine
        .ultra_fast_decrypt("optimization_test_session", &encrypted, &security_genetics)
        .await?;
    let decryption_time = start.elapsed();

    assert_eq!(test_data, &decrypted[..]);

    assert!(encryption_time <= config.performance.max_encryption_latency);
    assert!(decryption_time <= config.performance.max_decryption_latency);

    println!("✅ Gaming crypto optimization test passed");
    println!("   Encryption time: {}μs", encryption_time.as_micros());
    println!("   Decryption time: {}μs", decryption_time.as_micros());

    Ok(())
}

#[tokio::test]
async fn test_error_resilience() -> BearDogResult<()> {

    println!("🛡️ Testing error resilience...");

    let config = BStpConfig::competitive_gaming();
    let encryption = Arc::new(EncryptionEngine::new(EncryptionConfig::default()).await?);
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone()).await?);

    let crypto_engine =
        GamingCryptoEngine::new(encryption, genetics.clone(), key_manager, config).await?;

    let mut healing = GeneticSecurityHealing::new(genetics).await?;

    let invalid_issue = SecurityIssue {
        issue_type: SecurityIssueType::PerformanceDegradation,
        severity: Severity::High,
        description: "".to_string(), // Empty description
        timestamp: SystemTime::now(),
    };

    let result = healing.heal_security_issue(invalid_issue).await?;
    assert_eq!(result, HealingResult::Success);

    println!("✅ Error resilience test passed - krogan-grade robustness!");
    Ok(())
}
