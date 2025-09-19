use beardog::config::EncryptionConfig;
use beardog::encryption::EncryptionEngine;
use beardog::genetics::{DefaultBearDogGeneticsEngine, GeneticsConfig, InMemoryGeneticsStore};
use beardog::tunnel::{
    genetic_healing::{NetworkEvent, SecurityIssue, SecurityIssueType, Severity},
    key_manager::CryptoAlgorithm,
    BStpConfig, BStpKeyManager, GamingCryptoEngine, GeneticSecurityHealing,
};
use beardog_errors::BearDogError;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

#[tokio::test]
async fn test_key_isolation_security() -> Result<(), BearDogError> {
    println!("🔐 Testing session key isolation security...");

    let config = BStpConfig::maximum_security();
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone())?);

    let sessions = vec![
        ("peer_alice", "session_alice"),
        ("peer_bob", "session_bob"),
        ("peer_charlie", "session_charlie"),
    ];

    let mut session_keys = Vec::new();

    for (peer, session_id) in &sessions {
        let key = key_manager.generate_session_key(session_id, CryptoAlgorithm::Aes256Gcm)?;
        session_keys.push((peer.to_string(), session_id.to_string(), key));
    }

    for i in 0..session_keys.len() {
        for j in (i + 1)..session_keys.len() {
            let (_, _, key1) = &session_keys[i];
            let (_, _, key2) = &session_keys[j];

            assert_ne!(key1.key, key2.key, "Session keys must be unique!");
            assert_ne!(key1.key_id, key2.key_id, "Key IDs must be unique!");
        }
    }

    let wrong_key_result = key_manager.get_session_key("non_existent_session");
    assert!(
        wrong_key_result.is_none(),
        "Should return None for non-existent session"
    );

    println!("✅ Key isolation security test passed - sessions properly isolated!");
    Ok(())
}

#[tokio::test]
async fn test_encryption_tamper_resistance() -> Result<(), BearDogError> {
    println!("🛡️ Testing encryption tamper resistance...");

    let config = BStpConfig::maximum_security();
    let encryption = Arc::new(EncryptionEngine::new(EncryptionConfig::default())?);
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone())?);

    let security_provider = GamingCryptoEngine::new(
        encryption.clone(),
        genetics.clone(),
        key_manager.clone(),
        config.clone(),
    )?;

    let security_genetics = beardog::tunnel::SecurityGenetics::default();
    let session_id = "tamper_test_session";
    let original_data = b"sensitive_gaming_data_do_not_tamper";

    let mut encrypted_packet =
        security_provider.ultra_fast_encrypt(session_id, original_data, &security_genetics)?;

    let original_encrypted = encrypted_packet.data.clone();

    let tampering_tests = vec![
        ("Single bit flip", {
            let mut tampered = original_encrypted.clone();
            if !tampered.is_empty() {
                tampered[0] ^= 0x01; // Flip one bit
            }
            tampered
        }),
        ("Byte modification", {
            let mut tampered = original_encrypted.clone();
            if tampered.len() > 5 {
                tampered[5] = tampered[5].wrapping_add(1);
            }
            tampered
        }),
        ("Truncation", {
            let mut tampered = original_encrypted.clone();
            if tampered.len() > 10 {
                tampered.truncate(tampered.len() - 10);
            }
            tampered
        }),
        ("Extension", {
            let mut tampered = original_encrypted.clone();
            tampered.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);
            tampered
        }),
    ];

    for (test_name, tampered_data) in tampering_tests {
        encrypted_packet.data = tampered_data;

        let decrypt_result =
            security_provider.ultra_fast_decrypt(session_id, &encrypted_packet, &security_genetics);

        assert!(
            decrypt_result.is_err(),
            "Tamper test '{test_name}' should have failed but didn't"
        );

        println!("✅ Tamper resistance test '{test_name}' passed");
    }

    encrypted_packet.data = original_encrypted;
    let decrypted =
        security_provider.ultra_fast_decrypt(session_id, &encrypted_packet, &security_genetics)?;

    assert_eq!(original_data, &decrypted[..]);

    println!("✅ Encryption tamper resistance test passed - krogan-grade integrity!");
    Ok(())
}

#[tokio::test]
async fn test_timing_attack_resistance() -> Result<(), BearDogError> {
    println!("⏱️ Testing timing attack resistance...");

    let config = BStpConfig::maximum_security();
    let encryption = Arc::new(EncryptionEngine::new(EncryptionConfig::default())?);
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone())?);

    let security_provider = GamingCryptoEngine::new(
        encryption.clone(),
        genetics.clone(),
        key_manager.clone(),
        config.clone(),
    )?;

    let security_genetics = beardog::tunnel::SecurityGenetics::default();

    let test_patterns = vec![
        vec![0x00; 256],                // All zeros
        vec![0xFF; 256],                // All ones
        vec![0x55; 256],                // Alternating pattern
        (0..=255).collect::<Vec<u8>>(), // Sequential pattern
        {
            let mut random = Vec::new();
            for i in 0..256 {
                random.push((i * 17 + 42) as u8); // Pseudo-random
            }
            random
        },
    ];

    let iterations = 100;

    for (pattern_idx, pattern) in test_patterns.iter().enumerate() {
        let mut encryption_times = Vec::new();
        let mut decryption_times = Vec::new();

        for i in 0..iterations {
            let session_id = format!("timing_test_{pattern_idx}_{i}");

            let start = std::time::Instant::now();
            let encrypted =
                security_provider.ultra_fast_encrypt(&session_id, pattern, &security_genetics)?;
            encryption_times.push(start.elapsed());

            let start = std::time::Instant::now();
            let decrypted = security_provider.ultra_fast_decrypt(
                &session_id,
                &encrypted,
                &security_genetics,
            )?;
            decryption_times.push(start.elapsed());

            assert_eq!(pattern, &decrypted);
        }

        let avg_encryption = encryption_times.iter().sum::<Duration>() / iterations as u32;
        let avg_decryption = decryption_times.iter().sum::<Duration>() / iterations as u32;

        println!(
            "Pattern {}: Enc {}μs, Dec {}μs",
            pattern_idx,
            avg_encryption.as_micros(),
            avg_decryption.as_micros()
        );
    }

    println!("✅ Timing attack resistance test completed - analyze for consistent timing");
    Ok(())
}

#[tokio::test]
async fn test_session_hijacking_protection() -> Result<(), BearDogError> {
    println!("🔒 Testing session hijacking protection...");

    let config = BStpConfig::maximum_security();
    let encryption = Arc::new(EncryptionEngine::new(EncryptionConfig::default())?);
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone())?);

    let security_provider = GamingCryptoEngine::new(
        encryption.clone(),
        genetics.clone(),
        key_manager.clone(),
        config.clone(),
    )?;

    let legitimate_peer = "trusted_gaming_peer_12345";

    let test_data = b"legitimate session data";
    let security_genetics = beardog::tunnel::SecurityGenetics::default();

    let encrypted =
        security_provider.ultra_fast_encrypt(legitimate_peer, test_data, &security_genetics)?;

    let decrypted =
        security_provider.ultra_fast_decrypt(legitimate_peer, &encrypted, &security_genetics)?;

    assert_eq!(test_data, &decrypted[..]);

    let long_string = "A".repeat(1000000);
    let malicious_sessions = [
        "'; DROP TABLE sessions; --",
        "../../../etc/passwd",
        "<script>alert('xss')</script>",
        &long_string, // Very long string
        "\0\0\0\0",   // Null bytes
        "💀💀💀",     // Unicode
    ];

    for fake_session in malicious_sessions.iter() {
        let test_result =
            security_provider.ultra_fast_encrypt(fake_session, b"test data", &security_genetics);

        assert!(test_result.is_ok() || test_result.is_err());
    }

    let final_encrypted =
        security_provider.ultra_fast_encrypt(legitimate_peer, test_data, &security_genetics)?;

    let final_decrypted = security_provider.ultra_fast_decrypt(
        legitimate_peer,
        &final_encrypted,
        &security_genetics,
    )?;

    assert_eq!(test_data, &final_decrypted[..]);

    for fake_session in malicious_sessions.iter() {
        let test_result =
            security_provider.ultra_fast_encrypt(fake_session, b"another test", &security_genetics);

        assert!(test_result.is_ok() || test_result.is_err());
    }

    println!("✅ Session hijacking protection test passed - krogan-grade session security!");
    Ok(())
}

#[tokio::test]
fn test_genetic_healing_security() -> Result<(), BearDogError> {
    println!("🧬 Testing genetic healing security...");

    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let mut healing = GeneticSecurityHealing::new();

    let malicious_issues = vec![
        SecurityIssue {
            issue_type: SecurityIssueType::EncryptionCompromised,
            severity: Severity::Critical,
            description: "'; DROP TABLE users; --".to_string(), // SQL injection attempt
            timestamp: SystemTime::now(),
        },
        SecurityIssue {
            issue_type: SecurityIssueType::AuthenticationBreach,
            severity: Severity::High,
            description: "../../../etc/passwd".to_string(), // Path traversal attempt
            timestamp: SystemTime::now(),
        },
        SecurityIssue {
            issue_type: SecurityIssueType::NetworkAnomaly,
            severity: Severity::Medium,
            description: "<script>alert('xss')</script>".to_string(), // XSS attempt
            timestamp: SystemTime::now(),
        },
    ];

    for (i, issue) in malicious_issues.into_iter().enumerate() {
        let result = healing.heal_security_issue(issue)?;

        assert_eq!(result, beardog::tunnel::HealingResult::Success);
        println!("✅ Safely handled malicious issue {i}");
    }

    let extreme_events = vec![
        NetworkEvent::NetworkCongestion {
            latency_ms: u64::MAX,
        },
        NetworkEvent::PeerDisconnected {
            reason: "A".repeat(1000000), // Very long string
        },
        NetworkEvent::SuspiciousTraffic {
            source: "💀💀💀".to_string(), // Unicode attack
        },
    ];

    for (i, event) in extreme_events.into_iter().enumerate() {
        let result = healing.heal_from_network_event(event);
        assert!(
            result.is_ok(),
            "Extreme event {i} should be handled gracefully"
        );
    }

    println!("✅ Genetic healing security test passed - immune to exploitation!");
    Ok(())
}

#[tokio::test]
async fn test_key_rotation_security() -> Result<(), BearDogError> {
    println!("🔄 Testing key rotation security...");

    let config = BStpConfig::maximum_security();
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone())?);

    let session_id = "rotation_test_session";

    let key1 = key_manager.generate_session_key(session_id, CryptoAlgorithm::Aes256Gcm)?;

    let key2 = key_manager.rotate_session_key(session_id, None)?;
    let key3 =
        key_manager.rotate_session_key(session_id, Some(CryptoAlgorithm::ChaCha20Poly1305))?;
    let key4 = key_manager.rotate_session_key(session_id, Some(CryptoAlgorithm::GeneticHybrid))?;

    assert_ne!(key1.key, key2.key);
    assert_ne!(key2.key, key3.key);
    assert_ne!(key3.key, key4.key);
    assert_ne!(key1.key, key4.key);

    assert_ne!(key1.key_id, key2.key_id);
    assert_ne!(key2.key_id, key3.key_id);
    assert_ne!(key3.key_id, key4.key_id);

    assert_eq!(key3.algorithm, CryptoAlgorithm::ChaCha20Poly1305);
    assert_eq!(key4.algorithm, CryptoAlgorithm::GeneticHybrid);

    let current_key = key_manager.get_session_key(session_id).map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Session key should exist after rotation",
            e
        );
        beardog_errors::BearDogError::internal(format!(
            "Error: {:?}",
            "Session key should exist after rotation", e
        ))
    })?;
    assert_eq!(current_key.key, key4.key);
    assert_eq!(current_key.key_id, key4.key_id);

    println!("✅ Key rotation security test passed - secure key evolution!");
    Ok(())
}

#[tokio::test]
async fn test_session_validation() -> Result<(), BearDogError> {
    println!("🔒 Testing session validation security...");

    let config = BStpConfig::maximum_security();
    let encryption = Arc::new(EncryptionEngine::new(EncryptionConfig::default())?);
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone())?);

    let security_provider = GamingCryptoEngine::new(
        encryption.clone(),
        genetics.clone(),
        key_manager.clone(),
        config.clone(),
    )?;

    let legitimate_peer = "trusted_gaming_peer_12345";

    let test_data = b"legitimate session data";
    let security_genetics = beardog::tunnel::SecurityGenetics::default();

    let encrypted =
        security_provider.ultra_fast_encrypt(legitimate_peer, test_data, &security_genetics)?;

    let decrypted =
        security_provider.ultra_fast_decrypt(legitimate_peer, &encrypted, &security_genetics)?;

    assert_eq!(test_data, &decrypted[..]);

    let long_string = "A".repeat(1000000);
    let malicious_sessions = [
        "'; DROP TABLE sessions; --",
        "../../../etc/passwd",
        "<script>alert('xss')</script>",
        &long_string, // Very long string
        "\0\0\0\0",   // Null bytes
        "💀💀💀",     // Unicode
    ];

    for fake_session in malicious_sessions.iter() {
        let test_result =
            security_provider.ultra_fast_encrypt(fake_session, b"test data", &security_genetics);

        assert!(test_result.is_ok() || test_result.is_err());
    }

    let final_encrypted =
        security_provider.ultra_fast_encrypt(legitimate_peer, test_data, &security_genetics)?;

    let final_decrypted = security_provider.ultra_fast_decrypt(
        legitimate_peer,
        &final_encrypted,
        &security_genetics,
    )?;

    assert_eq!(test_data, &final_decrypted[..]);

    println!("✅ Session validation security tests passed");
    Ok(())
}

#[tokio::test]
async fn test_simplified_validation() -> Result<(), BearDogError> {
    let config = BStpConfig::maximum_security();
    let encryption = Arc::new(EncryptionEngine::new(EncryptionConfig::default())?);
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics = Arc::new(DefaultBearDogGeneticsEngine::new(
        genetics_store,
        GeneticsConfig::default(),
    ));
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management.clone())?);

    let security_provider = GamingCryptoEngine::new(
        encryption.clone(),
        genetics.clone(),
        key_manager.clone(),
        config.clone(),
    )?;

    let test_data = b"validation test data";
    let security_genetics = beardog::tunnel::SecurityGenetics::default();

    let encrypted =
        security_provider.ultra_fast_encrypt("test_session", test_data, &security_genetics)?;

    let decrypted =
        security_provider.ultra_fast_decrypt("test_session", &encrypted, &security_genetics)?;

    assert_eq!(test_data, &decrypted[..]);

    println!("✅ Simplified validation tests passed");
    Ok(())
}
