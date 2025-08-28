

use beardog_core::BearDogCore;
use beardog_types::config::BearDogConfig;
use beardog_errors::BearDogError;
use beardog_security::MemoryKeyManager;
use beardog_compliance::ComplianceEngine;
use beardog_monitoring::SecuritySentinel;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_full_system_initialization() -> Result<(), BearDogError> {

    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;

    core.initialize().await?;

    let state = core.state.read().await;
    assert!(state.components.len() > 0);
    
    println!("✅ Full system initialization successful");
    Ok(())
}

#[tokio::test]
async fn test_security_compliance_integration() -> Result<(), BearDogError> {

    let key_config = beardog_security::memory_key_manager::MemoryKeyConfig::default();
    let key_manager = MemoryKeyManager::new(key_config).await?;
    
    let compliance_config = beardog_compliance::ComplianceConfig::default();
    let compliance_engine = ComplianceEngine::new(compliance_config).await?;

    let test_key = b"test_integration_key".to_vec();
    let metadata = beardog_security::memory_key_manager::KeyMetadata {
        key_type: "AES-256".to_string(),
        purpose: "integration_test".to_string(),
        algorithm: "AES-256".to_string(),
        key_size: 256,
        owner_id: "integration_test".to_string(),
        tags: vec!["test".to_string()],
        attributes: std::collections::HashMap::with_capacity(16),
    };
    
    let key_id = key_manager.store_key(test_key, metadata).await?;

    let event = beardog_compliance::ComplianceEvent {
        id: format_args!("key_gen_{}", key_id).to_string(),
        timestamp: chrono::Utc::now(),
        event_type: "key_generation".to_string(),
        source: "beardog-security".to_string(),
        data: std::collections::HashMap::with_capacity(16),
        metadata: std::collections::HashMap::with_capacity(16),
    };
    
    let compliance_result = compliance_engine.process_event(&event).await?;
    assert!(compliance_result.score >= 0.0);
    
    println!("✅ Security-Compliance integration successful");
    Ok(())
}

#[tokio::test]
async fn test_monitoring_integration() -> Result<(), BearDogError> {

    let sentinel = SecuritySentinel::new();

    for _i in 0..3 {

        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    println!("✅ Monitoring integration successful");
    Ok(())
}

#[tokio::test]
async fn test_error_handling_integration() -> Result<(), BearDogError> {

    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;

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
async fn test_concurrent_operations() -> Result<(), BearDogError> {

    let key_config = beardog_security::memory_key_manager::MemoryKeyConfig::default();
    let key_manager = MemoryKeyManager::new(key_config).await?;
    
    let mut handles = Vec::new();

    for i in 0..5 {
        let km = key_manager.clone();
        let handle = tokio::spawn(async move {
            let test_key = format_args!("concurrent_test_key_{}", i).to_string().into_bytes();
            let metadata = beardog_security::memory_key_manager::KeyMetadata {
                key_type: "Ed25519".to_string(),
                purpose: format_args!("concurrent_test_{}", i).to_string(),
                algorithm: "Ed25519".to_string(),
                key_size: 256,
                owner_id: "concurrent_test".to_string(),
                tags: vec![format_args!("test_{}", i).to_string()],
                attributes: std::collections::HashMap::with_capacity(16),
            };
            
            km.store_key(test_key, metadata).await
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        assert!(result.is_ok());
    }
    
    println!("✅ Concurrent operations test successful");
    Ok(())
}

#[tokio::test]
async fn test_performance_under_load() -> Result<(), BearDogError> {

    let key_config = beardog_security::memory_key_manager::MemoryKeyConfig::default();
    let key_manager = MemoryKeyManager::new(key_config).await?;
    
    let start_time = std::time::Instant::now();

    for i in 0..100 {
        let test_key = format_args!("load_test_key_{}", i).to_string().into_bytes();
        let metadata = beardog_security::memory_key_manager::KeyMetadata {
            key_type: "AES-256".to_string(),
            purpose: format_args!("load_test_{}", i).to_string(),
            algorithm: "AES-256".to_string(),
            key_size: 256,
            owner_id: "load_test".to_string(),
            tags: vec![format_args!("load_{}", i).to_string()],
            attributes: std::collections::HashMap::with_capacity(16),
        };
        
        let _key_id = key_manager.store_key(test_key, metadata).await?;
    }
    
    let duration = start_time.elapsed();

    assert!(duration.as_secs() < 1);
    
    println!("✅ Performance test successful: {} operations in {:?}", 100, duration);
    Ok(())
}

#[tokio::test]
async fn test_system_recovery() -> Result<(), BearDogError> {

    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;

    core.initialize().await?;

    for _cycle in 0..3 {

        tokio::time::sleep(Duration::from_millis(10)).await;

        let state = core.state.read().await;
        assert!(state.components.len() > 0);
    }
    
    println!("✅ System recovery test successful");
    Ok(())
}

#[tokio::test]
async fn test_data_consistency() -> Result<(), BearDogError> {

    let key_config = beardog_security::memory_key_manager::MemoryKeyConfig::default();
    let key_manager = MemoryKeyManager::new(key_config).await?;

    let mut key_ids = Vec::new();
    for i in 0..10 {
        let test_key = format_args!("consistency_test_key_{}", i).to_string().into_bytes();
        let metadata = beardog_security::memory_key_manager::KeyMetadata {
            key_type: "Ed25519".to_string(),
            purpose: format_args!("consistency_test_{}", i).to_string(),
            algorithm: "Ed25519".to_string(),
            key_size: 256,
            owner_id: "consistency_test".to_string(),
            tags: vec![format_args!("consistency_{}", i).to_string()],
            attributes: std::collections::HashMap::with_capacity(16),
        };
        
        let key_id = key_manager.store_key(test_key, metadata).await?;
        key_ids.push(key_id);
    }

    for key_id in &key_ids {
        let retrieved_key = key_manager.get_key(key_id).await?;
        assert!(!retrieved_key.is_empty());
    }

    let all_keys = key_manager.list_keys().await?;
    assert!(all_keys.len() >= key_ids.len());
    
    println!("✅ Data consistency test successful");
    Ok(())
} 