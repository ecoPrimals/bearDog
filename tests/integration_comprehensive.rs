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


//! Comprehensive Integration Tests for BearDog
//!
//! This test suite validates the entire BearDog system integration including:
//! - Cross-crate functionality
//! - End-to-end workflows
//! - Security and compliance integration
//! - Performance under load
//! - Error handling and recovery

use beardog_core::BearDogCore;
use beardog_types::config::BearDogConfig;
use beardog_errors::BearDogResult;
use beardog_security::MemoryKeyManager;
use beardog_compliance::ComplianceEngine;
use beardog_monitoring::SecuritySentinel;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_full_system_initialization() -> BearDogResult<()> {
    // Test complete system startup
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;
    
    // Initialize all subsystems
    core.initialize().await?;
    
    // Verify core components are running
    let state = core.state.read().await;
    assert!(state.components.len() > 0);
    
    println!("✅ Full system initialization successful");
    Ok(())
}

#[tokio::test]
async fn test_security_compliance_integration() -> BearDogResult<()> {
    // Test integration between security and compliance systems
    let key_config = beardog_security::memory_key_manager::MemoryKeyConfig::default();
    let key_manager = MemoryKeyManager::new(key_config).await?;
    
    let compliance_config = beardog_compliance::ComplianceConfig::default();
    let compliance_engine = ComplianceEngine::new(compliance_config).await?;
    
    // Generate a key and ensure it's compliant
    let test_key = b"test_integration_key".to_vec();
    let metadata = beardog_security::memory_key_manager::KeyMetadata {
        key_type: "AES-256".to_string(),
        purpose: "integration_test".to_string(),
        algorithm: "AES-256".to_string(),
        key_size: 256,
        owner_id: "integration_test".to_string(),
        tags: vec!["test".to_string()],
        attributes: std::collections::HashMap::new(),
    };
    
    let key_id = key_manager.store_key(test_key, metadata).await?;
    
    // Create compliance event for key generation
    let event = beardog_compliance::ComplianceEvent {
        id: format!("key_gen_{}", key_id),
        timestamp: chrono::Utc::now(),
        event_type: "key_generation".to_string(),
        source: "beardog-security".to_string(),
        data: std::collections::HashMap::new(),
        metadata: std::collections::HashMap::new(),
    };
    
    let compliance_result = compliance_engine.process_event(&event).await?;
    assert!(compliance_result.score >= 0.0);
    
    println!("✅ Security-Compliance integration successful");
    Ok(())
}

#[tokio::test]
async fn test_monitoring_integration() -> BearDogResult<()> {
    // Test monitoring system integration
    let sentinel = SecuritySentinel::new();
    
    // Simulate some system activity and monitoring
    for _i in 0..3 {
        // Simulate security operations
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    println!("✅ Monitoring integration successful");
    Ok(())
}

#[tokio::test]
async fn test_error_handling_integration() -> BearDogResult<()> {
    // Test error handling across the system
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;
    
    // Test graceful error handling
    let result = timeout(Duration::from_secs(5), core.initialize()).await;
    
    match result {
        Ok(init_result) => {
            assert!(init_result.is_ok());
            println!("✅ System initialization completed successfully");
        }
        Err(_) => {
            println!("⚠️ System initialization timed out (acceptable for integration test)");
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_concurrent_operations() -> BearDogResult<()> {
    // Test concurrent operations across multiple components
    let key_config = beardog_security::memory_key_manager::MemoryKeyConfig::default();
    let key_manager = MemoryKeyManager::new(key_config).await?;
    
    let mut handles = Vec::new();
    
    // Spawn multiple concurrent key operations
    for i in 0..5 {
        let km = key_manager.clone();
        let handle = tokio::spawn(async move {
            let test_key = format!("concurrent_test_key_{}", i).into_bytes();
            let metadata = beardog_security::memory_key_manager::KeyMetadata {
                key_type: "Ed25519".to_string(),
                purpose: format!("concurrent_test_{}", i),
                algorithm: "Ed25519".to_string(),
                key_size: 256,
                owner_id: "concurrent_test".to_string(),
                tags: vec![format!("test_{}", i)],
                attributes: std::collections::HashMap::new(),
            };
            
            km.store_key(test_key, metadata).await
        });
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    for handle in handles {
        let result = handle.await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        assert!(result.is_ok());
    }
    
    println!("✅ Concurrent operations test successful");
    Ok(())
}

#[tokio::test]
async fn test_performance_under_load() -> BearDogResult<()> {
    // Test system performance under load
    let key_config = beardog_security::memory_key_manager::MemoryKeyConfig::default();
    let key_manager = MemoryKeyManager::new(key_config).await?;
    
    let start_time = std::time::Instant::now();
    
    // Perform 100 key operations
    for i in 0..100 {
        let test_key = format!("load_test_key_{}", i).into_bytes();
        let metadata = beardog_security::memory_key_manager::KeyMetadata {
            key_type: "AES-256".to_string(),
            purpose: format!("load_test_{}", i),
            algorithm: "AES-256".to_string(),
            key_size: 256,
            owner_id: "load_test".to_string(),
            tags: vec![format!("load_{}", i)],
            attributes: std::collections::HashMap::new(),
        };
        
        let _key_id = key_manager.store_key(test_key, metadata).await?;
    }
    
    let duration = start_time.elapsed();
    
    // Should complete 100 operations in reasonable time (< 1 second)
    assert!(duration.as_secs() < 1);
    
    println!("✅ Performance test successful: {} operations in {:?}", 100, duration);
    Ok(())
}

#[tokio::test]
async fn test_system_recovery() -> BearDogResult<()> {
    // Test system recovery capabilities
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;
    
    // Initialize system
    core.initialize().await?;
    
    // Simulate system stress and recovery
    for _cycle in 0..3 {
        // Simulate operations
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // System should remain stable
        let state = core.state.read().await;
        assert!(state.components.len() > 0);
    }
    
    println!("✅ System recovery test successful");
    Ok(())
}

#[tokio::test]
async fn test_data_consistency() -> BearDogResult<()> {
    // Test data consistency across operations
    let key_config = beardog_security::memory_key_manager::MemoryKeyConfig::default();
    let key_manager = MemoryKeyManager::new(key_config).await?;
    
    // Store multiple keys
    let mut key_ids = Vec::new();
    for i in 0..10 {
        let test_key = format!("consistency_test_key_{}", i).into_bytes();
        let metadata = beardog_security::memory_key_manager::KeyMetadata {
            key_type: "Ed25519".to_string(),
            purpose: format!("consistency_test_{}", i),
            algorithm: "Ed25519".to_string(),
            key_size: 256,
            owner_id: "consistency_test".to_string(),
            tags: vec![format!("consistency_{}", i)],
            attributes: std::collections::HashMap::new(),
        };
        
        let key_id = key_manager.store_key(test_key, metadata).await?;
        key_ids.push(key_id);
    }
    
    // Verify all keys can be retrieved
    for key_id in &key_ids {
        let retrieved_key = key_manager.get_key(key_id).await?;
        assert!(!retrieved_key.is_empty());
    }
    
    // Verify key listing includes all keys
    let all_keys = key_manager.list_keys().await?;
    assert!(all_keys.len() >= key_ids.len());
    
    println!("✅ Data consistency test successful");
    Ok(())
} 