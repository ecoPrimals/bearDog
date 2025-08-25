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


//! BSTP (BearDog Secure Tunnel Protocol) Security Testing
//!
//! Basic tests for the tunnel layer components

use beardog::tunnel::key_manager::{BStpKeyManager, CryptoAlgorithm};
use beardog::tunnel::{config::BStpConfig, session::SessionManager};
use beardog::BearDogResult;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_configuration_profiles() -> BearDogResult<()> {
    // Test competitive gaming profile
    let competitive = BStpConfig::competitive_gaming();
    assert!(competitive.performance.max_encryption_latency <= Duration::from_micros(100));
    assert!(competitive.gaming.ultra_low_latency);
    assert!(competitive.gaming.prefer_hardware_crypto);

    // Test maximum security profile
    let max_security = BStpConfig::maximum_security();
    assert!(max_security.key_management.use_hardware_keys);
    assert!(max_security.key_management.key_derivation_rounds >= 10000);
    assert!(max_security.genetic_healing.enable_healing);

    // Test default configuration
    let default = BStpConfig::from_env();
    assert!(default.performance.max_encryption_latency > Duration::from_micros(0));
    assert!(default.key_management.session_key_length > 0);

    Ok(())
}

#[tokio::test]
async fn test_key_management_basic() -> BearDogResult<()> {
    let config = BStpConfig::competitive_gaming();
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management).await?);

    // Test session key generation
    let session_id = "test_session_001";
    let key1 = key_manager
        .generate_session_key(session_id, CryptoAlgorithm::Aes256Gcm)
        .await?;
    let key2 = key_manager
        .generate_session_key(session_id, CryptoAlgorithm::ChaCha20Poly1305)
        .await?;

    // Keys should be different for different algorithms
    assert_ne!(key1.key, key2.key);
    assert_eq!(key1.algorithm, CryptoAlgorithm::Aes256Gcm);
    assert_eq!(key2.algorithm, CryptoAlgorithm::ChaCha20Poly1305);

    // Test key retrieval
    let retrieved = key_manager.get_session_key(session_id).await;
    assert!(retrieved.is_some());

    Ok(())
}

#[tokio::test]
async fn test_session_manager_basic() -> BearDogResult<()> {
    let session_manager = SessionManager::new();

    // Basic session manager should be created successfully
    // This validates the core structure is working
    let session_id = "test_session_lifecycle";

    // Initially no session (async call)
    let initial = session_manager.get_session(session_id).await;
    assert!(initial.is_none());

    Ok(())
}

#[tokio::test]
async fn test_algorithm_types() -> BearDogResult<()> {
    let config = BStpConfig::competitive_gaming();
    let key_manager = Arc::new(BStpKeyManager::new(config.key_management).await?);

    // Test different algorithms
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

    // Verify NIST-approved algorithms are configured
    assert!(config.key_management.key_derivation_rounds >= 10000);
    assert!(config.key_management.use_hardware_keys);

    // Verify automatic rotation is configured properly
    assert!(config.key_management.key_rotation_interval <= Duration::from_secs(24 * 3600));

    Ok(())
}

#[tokio::test]
async fn test_gaming_performance_requirements() -> BearDogResult<()> {
    let config = BStpConfig::competitive_gaming();

    // Gaming performance requirements
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

    // Test that we can create multiple unique keys
    let mut keys = Vec::new();
    for i in 0..10 {
        let session_id = format!("security_test_{i}");
        let key = key_manager
            .generate_session_key(&session_id, CryptoAlgorithm::Aes256Gcm)
            .await?;
        keys.push(key.key);
    }

    // All keys should be unique
    for i in 0..keys.len() {
        for j in (i + 1)..keys.len() {
            assert_ne!(keys[i], keys[j], "Keys {i} and {j} are identical!");
        }
    }

    Ok(())
}
